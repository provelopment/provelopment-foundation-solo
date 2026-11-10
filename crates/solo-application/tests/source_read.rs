//! Public-API tests for the source-read port and its use case, driven by a fake adapter.
//!
//! The fake records every call, so these tests can prove what the use case *asked for* —
//! in particular that a Markdown body is never read while a JSON body is available — as
//! well as what it decided. Nothing here touches a filesystem: these are the platform-
//! independent decisions, and the real filesystem cases live in the adapter crate.

use std::cell::RefCell;

use solo_application::{
    CandidateBodyV1, CandidateProbeV1, DeclaredSourceSetSinkV1, PhysicalIdentityV1,
    ReadSourceCandidateV1, SetSourceEntryV1, SourceReadOutcomeV1, SourceReadPort,
    SourceReadRefusalKind, SourceReadRefusalV1, SourceReadSetOutcomeV1,
    check_physical_reservations_v1, drive_declared_source_set_v1, fingerprint_of,
    read_declared_source_v1,
};
use solo_domain::SourceCandidateKind::{Json, Markdown};
use solo_domain::{
    SourceCandidateDraftV1, SourceCandidateKind, SourceCandidatePathV1, SourceCandidateSetV1,
};

/// What the fake returns for one member: its probe, then its body.
type Reply = (CandidateProbeV1, CandidateBodyV1);

#[derive(Default)]
struct FakePort {
    replies: Vec<(SourceCandidateKind, Reply)>,
    calls: RefCell<Vec<String>>,
}

impl FakePort {
    fn new() -> Self {
        Self::default()
    }

    fn with(mut self, member: SourceCandidateKind, reply: Reply) -> Self {
        self.replies.push((member, reply));
        self
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn reply(&self, member: SourceCandidateKind) -> Option<&Reply> {
        self.replies
            .iter()
            .find(|(candidate, _)| *candidate == member)
            .map(|(_, reply)| reply)
    }
}

impl SourceReadPort for FakePort {
    fn probe_candidate(
        &self,
        member: SourceCandidateKind,
        _locator: &SourceCandidatePathV1,
    ) -> CandidateProbeV1 {
        self.calls.borrow_mut().push(format!("probe:{member}"));
        self.reply(member)
            .map_or(CandidateProbeV1::ConfirmedAbsent, |reply| reply.0)
    }

    fn read_candidate(
        &self,
        member: SourceCandidateKind,
        _locator: &SourceCandidatePathV1,
    ) -> CandidateBodyV1 {
        self.calls.borrow_mut().push(format!("read:{member}"));
        self.reply(member)
            .map_or(CandidateBodyV1::ConfirmedAbsent, |reply| reply.1.clone())
    }
}

fn locator(path: &str, kind: SourceCandidateKind) -> SourceCandidatePathV1 {
    match SourceCandidatePathV1::parse(path, kind) {
        Ok(value) => value,
        Err(error) => panic!("{path} must be an accepted locator: {error}"),
    }
}

fn declaration(json: Option<&str>, markdown: Option<&str>) -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    if let Some(path) = json {
        draft
            .json(locator(path, SourceCandidateKind::Json))
            .expect("json member declared once");
    }
    if let Some(path) = markdown {
        draft
            .markdown(locator(path, SourceCandidateKind::Markdown))
            .expect("markdown member declared once");
    }
    draft.build().expect("a non-empty declaration")
}

fn identity(value: u64) -> PhysicalIdentityV1 {
    PhysicalIdentityV1::new(7, value, 0)
}

/// The timestamps every ordinary fixture reports, so probe and read agree by default.
const STAMP: (i64, i64) = (1_700_000_000_000_000_000, 1_700_000_000_000_000_001);

/// The timestamps after a deterministic same-size rewrite: only the stamps move.
const LATER: (i64, i64) = (1_700_000_000_000_000_500, 1_700_000_000_000_000_501);

/// A located, singly-linked probe for one identity and length.
fn located(value: u64, length: u64) -> CandidateProbeV1 {
    located_stamped(value, length, STAMP)
}

/// A located probe reporting the given timestamps.
fn located_stamped(value: u64, length: u64, stamp: (i64, i64)) -> CandidateProbeV1 {
    CandidateProbeV1::Located {
        identity: identity(value),
        byte_length: length,
        links: 1,
        changed_nanos: stamp,
    }
}

/// A present body for one identity.
fn present(bytes: &[u8], value: u64) -> CandidateBodyV1 {
    present_stamped(bytes, value, STAMP)
}

/// A present body reporting the given timestamps.
fn present_stamped(bytes: &[u8], value: u64, stamp: (i64, i64)) -> CandidateBodyV1 {
    CandidateBodyV1::Present {
        fingerprint: fingerprint_of(bytes),
        bytes: bytes.to_vec(),
        identity: identity(value),
        links: 1,
        changed_nanos: stamp,
    }
}

/// A present body that claims an extra directory entry, i.e. a hardlinked object.
fn present_hardlinked(bytes: &[u8], value: u64, links: u64) -> CandidateBodyV1 {
    CandidateBodyV1::Present {
        fingerprint: fingerprint_of(bytes),
        bytes: bytes.to_vec(),
        identity: identity(value),
        links,
        changed_nanos: STAMP,
    }
}

const JSON_BODY: &[u8] = br#"{"contentVersion":"1.0.0"}"#;
const MARKDOWN_BODY: &[u8] = b"# Title\n";

#[test]
fn json_present_is_chosen_and_markdown_is_never_read() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(1, JSON_BODY.len() as u64), present(JSON_BODY, 1)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(
        Some("pages/main-home.en-001.json"),
        Some("pages/main-home.en-001.md"),
    );

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Chosen(choice) = outcome else {
        panic!("expected a chosen JSON body, got {outcome:?}");
    };
    assert_eq!(choice.member(), SourceCandidateKind::Json);
    assert_eq!(choice.logical_path(), "pages/main-home.en-001.json");
    assert_eq!(choice.bytes(), JSON_BODY);
    assert_eq!(choice.identity(), identity(1));
    assert_eq!(choice.fingerprint(), fingerprint_of(JSON_BODY));
    // Both members are probed so identities can be compared, and only JSON is read.
    assert_eq!(
        fake.calls(),
        vec!["probe:json", "probe:markdown", "read:json"]
    );
    assert!(!choice.is_publication_authorized());
    assert!(!choice.is_verified_filesystem_snapshot());
}

#[test]
fn markdown_only_is_chosen_when_it_is_the_only_declared_member() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Markdown,
        (
            located(2, MARKDOWN_BODY.len() as u64),
            present(MARKDOWN_BODY, 2),
        ),
    );
    let candidates = declaration(None, Some("pages/only.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Chosen(choice) = outcome else {
        panic!("expected a chosen Markdown body, got {outcome:?}");
    };
    assert_eq!(choice.member(), SourceCandidateKind::Markdown);
    assert_eq!(choice.bytes(), MARKDOWN_BODY);
    assert_eq!(fake.calls(), vec!["probe:markdown", "read:markdown"]);
}

#[test]
fn confirmed_json_absence_permits_markdown() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (
                CandidateProbeV1::ConfirmedAbsent,
                CandidateBodyV1::ConfirmedAbsent,
            ),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Chosen(choice) = outcome else {
        panic!("expected Markdown after a confirmed JSON absence, got {outcome:?}");
    };
    assert_eq!(choice.member(), SourceCandidateKind::Markdown);
    assert_eq!(
        fake.calls(),
        vec!["probe:json", "probe:markdown", "read:markdown"]
    );
}

#[test]
fn confirmed_absence_of_every_declared_member_is_none_effective() {
    let fake = FakePort::new();
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("expected a refusal, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::NoneEffective);
    assert_eq!(refusal.code(), "SOURCE_NONE_EFFECTIVE");
    // Absence is never inferred from a refusal, and nothing is read.
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

#[test]
fn a_probe_refusal_is_final_and_never_falls_back() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (
                CandidateProbeV1::Refused(SourceReadRefusalV1::new(
                    SourceReadRefusalKind::PermissionDenied,
                    Some(SourceCandidateKind::Json),
                )),
                present(JSON_BODY, 1),
            ),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("expected a refusal, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::PermissionDenied);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Json));
    // The declared Markdown exists and is still not read: refusal is not absence.
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

#[test]
fn two_members_resolving_to_one_object_are_a_collision() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(9, JSON_BODY.len() as u64), present(JSON_BODY, 9)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(9, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 9),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("expected a collision refusal, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.code(), "SOURCE_COLLISION");
    // Detected from the probes alone, so neither body was read.
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

#[test]
fn a_probe_to_read_replacement_that_keeps_identity_and_length_is_refused() {
    // The deterministic hostile replacement: the object keeps its identity and its length and
    // only its recorded change timestamps move. A read compared only against its own
    // before/after samples cannot see this; the comparison against the probe can.
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (
            located_stamped(1, JSON_BODY.len() as u64, STAMP),
            present_stamped(JSON_BODY, 1, LATER),
        ),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("a body disagreeing with its probe must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::ChangedDuringRead);
    assert_eq!(refusal.code(), "SNAPSHOT_CHANGED");
}

#[test]
fn a_read_of_a_different_object_than_the_probe_located_is_refused() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (located(1, JSON_BODY.len() as u64), present(JSON_BODY, 2)),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("a substituted object must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::ChangedDuringRead);
}

#[test]
fn a_read_whose_length_disagrees_with_the_probe_is_refused() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (
            located(1, u64::try_from(JSON_BODY.len()).expect("small") + 1),
            present(JSON_BODY, 1),
        ),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("a length that disagrees with the probe must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::ChangedDuringRead);
}

#[test]
fn two_representations_resolving_to_one_physical_object_are_refused() {
    // Both declarations probe to the same object. Checking one representation's two members
    // against each other would never see this, and one authored body would then answer for
    // two distinct representations.
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(9, JSON_BODY.len() as u64), present(JSON_BODY, 9)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(9, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 9),
            ),
        );
    let first = declaration(Some("pages/a.en-001.json"), None);
    let second = declaration(Some("pages/b.en-001.json"), None);

    let error = check_physical_reservations_v1(&fake, &[first, second])
        .expect_err("one physical object cannot back two representations");

    assert_eq!(error.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(error.code(), "SOURCE_COLLISION");
}

#[test]
fn distinct_representations_reserve_their_own_identities() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(1, 11), present(JSON_BODY, 1)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (located(2, 12), present(MARKDOWN_BODY, 2)),
        );
    let first = declaration(Some("pages/a.en-001.json"), None);
    let second = declaration(None, Some("pages/b.en-001.md"));

    let reservations =
        check_physical_reservations_v1(&fake, &[first, second]).expect("distinct identities");

    assert_eq!(reservations.len(), 2);
    assert_eq!(reservations[0].declaration, 0);
    assert_eq!(reservations[0].member, SourceCandidateKind::Json);
    assert_eq!(reservations[0].locator.as_str(), "pages/a.en-001.json");
    assert_eq!(reservations[1].declaration, 1);
    assert_eq!(reservations[1].member, SourceCandidateKind::Markdown);
    assert_eq!(reservations[1].locator.as_str(), "pages/b.en-001.md");
    assert_ne!(reservations[0].identity, reservations[1].identity);
}

#[test]
fn a_hardlinked_body_is_refused_rather_than_shared() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (
            located(1, JSON_BODY.len() as u64),
            present_hardlinked(JSON_BODY, 1, 2),
        ),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("expected a collision refusal, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Json));
}

#[test]
fn a_body_that_vanishes_after_its_probe_is_a_change_not_an_absence() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (
                located(1, JSON_BODY.len() as u64),
                CandidateBodyV1::ConfirmedAbsent,
            ),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("expected a change refusal, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::ChangedDuringRead);
    assert_eq!(refusal.code(), "SNAPSHOT_CHANGED");
    // Markdown was never promoted, even though it exists.
    assert!(!fake.calls().contains(&"read:markdown".to_owned()));
}

#[test]
fn the_fingerprint_separates_content_and_ignores_identity() {
    assert_eq!(fingerprint_of(JSON_BODY), fingerprint_of(JSON_BODY));
    assert_ne!(fingerprint_of(JSON_BODY), fingerprint_of(MARKDOWN_BODY));
    assert_eq!(fingerprint_of(b""), 0xcbf2_9ce4_8422_2325);
}

#[test]
fn diagnostics_stay_bounded_and_carry_no_location() {
    let refusal = SourceReadRefusalV1::new(
        SourceReadRefusalKind::UnsafeLocation,
        Some(SourceCandidateKind::Markdown),
    );
    let rendered = refusal.to_string();
    assert!(rendered.starts_with("SOURCE_UNSAFE_PATH: "));
    assert!(rendered.contains("markdown"));
    // No separator, no drive, no extension: the logical member is named, not a path.
    assert!(!rendered.contains('/'));
    assert!(!rendered.contains('\\'));
    assert!(rendered.len() < 120, "diagnostic must stay bounded");
}

#[test]
fn no_read_outcome_can_claim_publication_or_snapshot_authority() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (located(1, JSON_BODY.len() as u64), present(JSON_BODY, 1)),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);
    let outcome = read_declared_source_v1(&fake, &candidates);
    let SourceReadOutcomeV1::Chosen(choice) = outcome else {
        panic!("expected a chosen body");
    };
    // The public constants and the per-value answers must agree, and both must be `false`.
    let publication_authority = solo_application::SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY;
    let snapshot_authority = solo_application::SOURCE_READ_PROVES_GLOBAL_SNAPSHOT;
    assert_eq!(publication_authority, choice.is_publication_authorized());
    assert_eq!(snapshot_authority, choice.is_verified_filesystem_snapshot());
    assert!(!publication_authority);
    assert!(!snapshot_authority);
}

/// A located probe that reports a second directory entry for the same object.
fn located_hardlinked(value: u64, length: u64) -> CandidateProbeV1 {
    CandidateProbeV1::Located {
        identity: identity(value),
        byte_length: length,
        links: 2,
        changed_nanos: STAMP,
    }
}

/// A present body that claims a different number of directory entries than its probe did.
fn present_with_links(bytes: &[u8], value: u64, links: u64) -> CandidateBodyV1 {
    CandidateBodyV1::Present {
        fingerprint: fingerprint_of(bytes),
        bytes: bytes.to_vec(),
        identity: identity(value),
        links,
        changed_nanos: STAMP,
    }
}

#[test]
fn an_unselected_hardlinked_candidate_is_refused_before_any_body_is_read() {
    // A valid JSON member would win the selection, but the *unselected* Markdown is ambiguous.
    // The ambiguity belongs to the candidate, so neither body is read.
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(1, JSON_BODY.len() as u64), present(JSON_BODY, 1)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located_hardlinked(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("an ambiguous declared candidate must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Markdown));
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

#[test]
fn a_link_count_that_changes_between_probe_and_read_is_refused() {
    // One directory entry at probe time, two when the same name is read: ambiguity arriving
    // after the probe is a collision, not a benign change.
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (
            located(1, JSON_BODY.len() as u64),
            present_with_links(JSON_BODY, 1, 2),
        ),
    );
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("a changed link count must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.code(), "SOURCE_COLLISION");
}

#[test]
fn an_injected_unsupported_answer_is_refused_and_never_becomes_absence() {
    // This is an **injected** mapping, not a host observation: it proves the fail-closed
    // classification of an unsupported/native-less answer, which no mechanism available in
    // this environment can force from a real kernel.
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (
                CandidateProbeV1::Refused(SourceReadRefusalV1::new(
                    SourceReadRefusalKind::Unsupported,
                    Some(SourceCandidateKind::Json),
                )),
                present(JSON_BODY, 1),
            ),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let outcome = read_declared_source_v1(&fake, &candidates);

    let SourceReadOutcomeV1::Refused(refusal) = outcome else {
        panic!("an unsupported answer must be refused, got {outcome:?}");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::Unsupported);
    assert_eq!(refusal.code(), "SOURCE_UNSUPPORTED");
    // Neither a fallback nor an absence is inferred from it.
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

// ---------------------------------------------------------------------------
// R03 — the global physical reservation must refuse ambiguity on its own.
//
// Comparing identities against each other cannot see it: with a single declaration there is
// nothing to collide with, so a candidate that a second directory entry also names used to be
// accepted and reserved. The link count is refused per candidate, before any reservation is
// recorded and before any body is read.
// ---------------------------------------------------------------------------

#[test]
fn a_sole_declaration_with_two_directory_entries_is_refused_before_any_reservation() {
    let fake = FakePort::new().with(
        SourceCandidateKind::Json,
        (
            located_hardlinked(1, JSON_BODY.len() as u64),
            present(JSON_BODY, 1),
        ),
    );
    let declarations = vec![declaration(Some("pages/p.en-001.json"), None)];

    let outcome = check_physical_reservations_v1(&fake, &declarations);

    let Err(refusal) = outcome else {
        panic!("a sole candidate with a second directory entry must be refused");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.code(), "SOURCE_COLLISION");
    // The decision needs no body at all.
    assert_eq!(fake.calls(), vec!["probe:json"]);
}

#[test]
fn an_unselected_multi_link_member_is_refused_by_the_global_reservation() {
    let fake = FakePort::new()
        .with(
            SourceCandidateKind::Json,
            (located(1, JSON_BODY.len() as u64), present(JSON_BODY, 1)),
        )
        .with(
            SourceCandidateKind::Markdown,
            (
                located_hardlinked(2, MARKDOWN_BODY.len() as u64),
                present(MARKDOWN_BODY, 2),
            ),
        );
    let declarations = vec![declaration(
        Some("pages/p.en-001.json"),
        Some("pages/p.en-001.md"),
    )];

    let outcome = check_physical_reservations_v1(&fake, &declarations);

    let Err(refusal) = outcome else {
        panic!("an ambiguous unselected member must be refused globally");
    };
    assert_eq!(refusal.kind(), SourceReadRefusalKind::IdentityCollision);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Markdown));
    assert_eq!(fake.calls(), vec!["probe:json", "probe:markdown"]);
}

/// A fake adapter keyed by member kind **and** logical locator.
///
/// The single-representation fake above answers by member only, which is enough when one
/// declaration is in play. A whole-set attempt addresses several declarations that can declare
/// the same member kind, so this fake keys on both and records both in every call it logs.
#[derive(Default)]
struct SetPort {
    replies: Vec<(SourceCandidateKind, String, Reply)>,
    calls: RefCell<Vec<String>>,
}

impl SetPort {
    fn new() -> Self {
        Self::default()
    }

    fn with(mut self, member: SourceCandidateKind, path: &str, reply: Reply) -> Self {
        self.replies.push((member, path.to_owned(), reply));
        self
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn reply(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> Option<&Reply> {
        self.replies
            .iter()
            .find(|(candidate, path, _)| *candidate == member && path == locator.as_str())
            .map(|(_, _, reply)| reply)
    }
}

impl SourceReadPort for SetPort {
    fn probe_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateProbeV1 {
        self.calls
            .borrow_mut()
            .push(format!("probe:{member}:{}", locator.as_str()));
        self.reply(member, locator)
            .map_or(CandidateProbeV1::ConfirmedAbsent, |reply| reply.0)
    }

    fn read_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateBodyV1 {
        self.calls
            .borrow_mut()
            .push(format!("read:{member}:{}", locator.as_str()));
        self.reply(member, locator)
            .map_or(CandidateBodyV1::ConfirmedAbsent, |reply| reply.1.clone())
    }
}

/// What one recording sink accepted, and what it observed about live body bytes.
///
/// The sink is the only holder the driver ever hands a body to, so these counters are observed
/// state rather than an inference from the number of calls: `peak_in_flight_bytes` is the largest
/// single body the driver ever had live, and `total_body_bytes` is everything it passed through.
struct RecordedSet {
    entries: Vec<(usize, ReadSourceCandidateV1)>,
    peak_in_flight_bytes: usize,
    total_body_bytes: usize,
}

impl RecordedSet {
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &(usize, ReadSourceCandidateV1)> {
        self.entries.iter()
    }

    fn get(&self, declaration: usize) -> Option<&(usize, ReadSourceCandidateV1)> {
        self.entries.get(declaration)
    }

    fn declaration(&self, index: usize) -> Option<usize> {
        self.get(index).map(|(declaration, _)| *declaration)
    }

    fn source(&self, index: usize) -> Option<&ReadSourceCandidateV1> {
        self.get(index).map(|(_, source)| source)
    }
}

/// The sink's own typed refusal, so the driver's sink-failure route can be observed.
#[derive(Debug, PartialEq, Eq)]
enum SinkRefusal {
    /// The sink refused the body for this declaration.
    At(usize),
    /// The sink refused to commit.
    Finish,
}

/// A sink that keeps what it is handed and records the peak it ever held.
struct RecordingSink {
    accept_until: usize,
    refuse_finish: bool,
    recorded: RecordedSet,
}

impl RecordingSink {
    fn new() -> Self {
        Self {
            accept_until: usize::MAX,
            refuse_finish: false,
            recorded: RecordedSet {
                entries: Vec::new(),
                peak_in_flight_bytes: 0,
                total_body_bytes: 0,
            },
        }
    }

    fn refusing_at(accept_until: usize) -> Self {
        Self {
            accept_until,
            ..Self::new()
        }
    }

    fn refusing_finish() -> Self {
        Self {
            refuse_finish: true,
            ..Self::new()
        }
    }
}

impl DeclaredSourceSetSinkV1 for RecordingSink {
    type Refusal = SinkRefusal;
    type Complete = RecordedSet;

    fn stage(&mut self, entry: SetSourceEntryV1) -> Result<(), SinkRefusal> {
        let declaration = entry.declaration();
        if declaration >= self.accept_until {
            return Err(SinkRefusal::At(declaration));
        }
        let source = entry.into_source();
        let length = source.bytes().len();
        self.recorded.peak_in_flight_bytes = self.recorded.peak_in_flight_bytes.max(length);
        self.recorded.total_body_bytes += length;
        self.recorded.entries.push((declaration, source));
        Ok(())
    }

    fn finish(self) -> Result<RecordedSet, SinkRefusal> {
        if self.refuse_finish {
            return Err(SinkRefusal::Finish);
        }
        Ok(self.recorded)
    }
}

fn drove(
    fake: &SetPort,
    declarations: &[SourceCandidateSetV1],
) -> SourceReadSetOutcomeV1<RecordedSet, SinkRefusal> {
    drive_declared_source_set_v1(fake, declarations, RecordingSink::new())
}

fn complete(outcome: SourceReadSetOutcomeV1<RecordedSet, SinkRefusal>) -> RecordedSet {
    match outcome {
        SourceReadSetOutcomeV1::Complete(set) => set,
        SourceReadSetOutcomeV1::ReadRefused(refusal) => {
            panic!("expected a complete set, got refusal {refusal}")
        }
        SourceReadSetOutcomeV1::SinkRefused(refusal) => {
            panic!("expected a complete set, got sink refusal {refusal:?}")
        }
    }
}

fn refused_set(
    outcome: SourceReadSetOutcomeV1<RecordedSet, SinkRefusal>,
) -> (SourceReadRefusalKind, &'static str, Option<usize>) {
    match outcome {
        SourceReadSetOutcomeV1::ReadRefused(refusal) => {
            (refusal.kind(), refusal.code(), refusal.declaration())
        }
        SourceReadSetOutcomeV1::Complete(set) => {
            panic!("expected a refusal, got {} accepted bodies", set.len())
        }
        SourceReadSetOutcomeV1::SinkRefused(refusal) => {
            panic!("expected a read refusal, got sink refusal {refusal:?}")
        }
    }
}

fn sink_refused(outcome: SourceReadSetOutcomeV1<RecordedSet, SinkRefusal>) -> SinkRefusal {
    match outcome {
        SourceReadSetOutcomeV1::SinkRefused(refusal) => refusal,
        SourceReadSetOutcomeV1::ReadRefused(refusal) => {
            panic!("expected a sink refusal, got read refusal {refusal}")
        }
        SourceReadSetOutcomeV1::Complete(set) => {
            panic!("expected a sink refusal, got {} accepted bodies", set.len())
        }
    }
}

#[test]
fn every_declared_representation_is_read_in_declaration_order() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(11))
        .with(Json, "pages/b.de-de.json", json_reply(12))
        .with(Json, "pages/c.id-id.json", json_reply(13));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
        declaration(Some("pages/c.id-id.json"), None),
    ];

    let set = complete(drove(&fake, &declarations));

    assert_eq!(set.len(), 3);
    assert!(!set.is_empty());
    for index in 0..3 {
        assert_eq!(set.declaration(index), Some(index));
        let source = set.source(index).expect("a recorded source");
        assert_eq!(source.bytes(), JSON_BODY);
        assert_eq!(source.fingerprint(), fingerprint_of(JSON_BODY));
        assert_eq!(
            source.identity(),
            identity(u64::try_from(index).expect("a small index") + 11)
        );
    }
    assert!(set.get(3).is_none());

    // The driver kept no collection of bodies: the largest single body it ever had live is one
    // body, while six times that many bytes passed through the sink.
    let single = JSON_BODY.len();
    assert_eq!(set.peak_in_flight_bytes, single);
    assert_eq!(set.total_body_bytes, 3 * single);
    assert!(
        set.total_body_bytes > 2 * set.peak_in_flight_bytes,
        "cumulative bytes must exceed the live bound"
    );

    // One reservation probe for every declared candidate, then exactly one probe/read pair per
    // declaration, in declared order.
    assert_eq!(
        fake.calls(),
        vec![
            "probe:json:pages/a.en-001.json",
            "probe:json:pages/b.de-de.json",
            "probe:json:pages/c.id-id.json",
            "probe:json:pages/a.en-001.json",
            "read:json:pages/a.en-001.json",
            "probe:json:pages/b.de-de.json",
            "read:json:pages/b.de-de.json",
            "probe:json:pages/c.id-id.json",
            "read:json:pages/c.id-id.json",
        ]
    );
}

#[test]
fn an_empty_declaration_list_is_refused_rather_than_returned_empty() {
    let fake = SetPort::new();

    let (kind, code, declaration) = refused_set(drove(&fake, &[]));

    assert_eq!(kind, SourceReadRefusalKind::NoneEffective);
    assert_eq!(code, "SOURCE_NONE_EFFECTIVE");
    assert_eq!(declaration, None);
    assert_eq!(fake.calls(), Vec::<String>::new());
}

#[test]
fn one_refused_declaration_refuses_the_whole_set_and_the_next_one_is_never_read() {
    let length = u64::try_from(JSON_BODY.len()).expect("a short fixture body");
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(11))
        .with(
            Json,
            "pages/b.de-de.json",
            (
                located(12, length),
                CandidateBodyV1::Refused(SourceReadRefusalV1::new(
                    SourceReadRefusalKind::PermissionDenied,
                    Some(SourceCandidateKind::Json),
                )),
            ),
        )
        .with(Json, "pages/c.id-id.json", json_reply(13));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
        declaration(Some("pages/c.id-id.json"), None),
    ];

    let (kind, code, declaration) = refused_set(drove(&fake, &declarations));

    assert_eq!(kind, SourceReadRefusalKind::PermissionDenied);
    assert_eq!(code, "SOURCE_OBSERVATION_PERMISSION_DENIED");
    assert_eq!(declaration, Some(1));
    // The third declaration is never reached: a refused member ends the attempt, so no valid
    // subset can be observed by a caller.
    assert!(
        !fake
            .calls()
            .iter()
            .any(|call| call == "read:json:pages/c.id-id.json"),
        "the declaration after the refused one must not be read"
    );
}

#[test]
fn a_cross_representation_identity_collision_is_refused_before_any_body_is_read() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(21))
        .with(Json, "pages/b.de-de.json", json_reply(21));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let (kind, code, declaration) = refused_set(drove(&fake, &declarations));

    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
    assert_eq!(code, "SOURCE_COLLISION");
    assert_eq!(declaration, None);
    assert!(
        !fake.calls().iter().any(|call| call.starts_with("read:")),
        "a reserved collision must be refused before any body is read"
    );
}

/// The probe and body a healthy JSON leaf answers with.
fn json_reply(value: u64) -> Reply {
    let length = u64::try_from(JSON_BODY.len()).expect("a short fixture body");
    (located(value, length), present(JSON_BODY, value))
}

/// The probe and body a healthy Markdown leaf answers with.
fn markdown_reply(value: u64) -> Reply {
    let length = u64::try_from(MARKDOWN_BODY.len()).expect("a short fixture body");
    (located(value, length), present(MARKDOWN_BODY, value))
}

#[test]
fn a_present_json_body_is_chosen_and_its_declared_markdown_is_never_read() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(31))
        .with(Markdown, "pages/a.en-001.md", markdown_reply(32))
        .with(Json, "pages/b.de-de.json", json_reply(33));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), Some("pages/a.en-001.md")),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let set = complete(drove(&fake, &declarations));

    assert_eq!(set.len(), 2);
    assert_eq!(
        set.source(0).map(|source| source.member()),
        Some(SourceCandidateKind::Json)
    );
    assert_eq!(
        set.source(0).map(|source| source.logical_path().to_owned()),
        Some("pages/a.en-001.json".to_owned())
    );
    assert!(
        !fake
            .calls()
            .iter()
            .any(|call| call == "read:markdown:pages/a.en-001.md"),
        "the unselected Markdown body must never be read while JSON is available"
    );
}

#[test]
fn a_confirmed_json_absence_selects_markdown_in_that_declaration_only() {
    let fake = SetPort::new()
        .with(Markdown, "pages/a.en-001.md", markdown_reply(41))
        .with(Json, "pages/b.de-de.json", json_reply(42));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), Some("pages/a.en-001.md")),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let set = complete(drove(&fake, &declarations));

    assert_eq!(
        set.source(0).map(|source| source.member()),
        Some(SourceCandidateKind::Markdown)
    );
    assert_eq!(
        set.source(0).map(|source| source.bytes().to_vec()),
        Some(MARKDOWN_BODY.to_vec())
    );
    assert_eq!(
        set.source(1).map(|source| source.member()),
        Some(SourceCandidateKind::Json)
    );
}

#[test]
fn an_unselected_hardlinked_markdown_refuses_the_whole_set_before_any_body_is_read() {
    let length = u64::try_from(MARKDOWN_BODY.len()).expect("a short fixture body");
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(51))
        .with(
            Markdown,
            "pages/a.en-001.md",
            (
                located_hardlinked(52, length),
                present_hardlinked(MARKDOWN_BODY, 52, 2),
            ),
        );
    let declarations = vec![declaration(
        Some("pages/a.en-001.json"),
        Some("pages/a.en-001.md"),
    )];

    let (kind, code, declaration) = refused_set(drove(&fake, &declarations));

    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
    assert_eq!(code, "SOURCE_COLLISION");
    assert_eq!(declaration, None);
    assert!(
        !fake.calls().iter().any(|call| call.starts_with("read:")),
        "an ambiguous declaration must be refused before either body is read"
    );
}

#[test]
fn a_change_observed_in_the_second_declaration_refuses_the_whole_set() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(61))
        .with(
            Json,
            "pages/b.de-de.json",
            (
                located(
                    62,
                    u64::try_from(JSON_BODY.len()).expect("a short fixture body"),
                ),
                present_stamped(JSON_BODY, 62, LATER),
            ),
        );
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let (kind, code, declaration) = refused_set(drove(&fake, &declarations));

    assert_eq!(kind, SourceReadRefusalKind::ChangedDuringRead);
    assert_eq!(code, "SNAPSHOT_CHANGED");
    assert_eq!(declaration, Some(1));
}

#[test]
fn no_complete_set_can_claim_publication_or_snapshot_authority() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(71))
        .with(Json, "pages/b.de-de.json", json_reply(72));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let set = complete(drove(&fake, &declarations));

    assert!(!set.is_empty());
    for (_, source) in set.iter() {
        assert!(!source.is_publication_authorized());
        assert!(!source.is_verified_filesystem_snapshot());
    }
}

#[test]
fn a_set_refusal_diagnostic_names_only_a_declaration() {
    let fake = SetPort::new().with(Json, "pages/a.en-001.json", json_reply(81));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let outcome = drove(&fake, &declarations);
    let SourceReadSetOutcomeV1::ReadRefused(refusal) = outcome else {
        panic!("an absent second declaration must be refused");
    };

    // The second declaration was never located, so it is `SOURCE_NONE_EFFECTIVE`, and the
    // diagnostic names the declaration without naming any path.
    assert_eq!(refusal.kind(), SourceReadRefusalKind::NoneEffective);
    assert_eq!(refusal.declaration(), Some(1));
    let rendered = format!("{refusal}");
    assert!(rendered.contains("declaration 1"), "{rendered}");
    assert!(!rendered.contains("pages/"), "{rendered}");
}

#[test]
fn the_driver_never_holds_more_than_one_body_while_many_pass_through() {
    let mut fake = SetPort::new();
    let mut declarations = Vec::new();
    for index in 0..6_u64 {
        let locator = format!("pages/p{index}.en-001.json");
        fake = fake.with(Json, locator.as_str(), json_reply(100 + index));
        declarations.push(declaration(Some(locator.as_str()), None));
    }

    let set = complete(drove(&fake, &declarations));

    assert_eq!(set.len(), 6);
    // Observed, not inferred: six bodies were accepted, but the largest single body live at any
    // moment stayed at one body's length while the cumulative total was six times that.
    let single = JSON_BODY.len();
    assert_eq!(set.peak_in_flight_bytes, single);
    assert_eq!(set.total_body_bytes, 6 * single);
    assert!(
        set.total_body_bytes > 5 * set.peak_in_flight_bytes,
        "cumulative body bytes must far exceed the live bound"
    );
    for index in 0..6 {
        assert_eq!(
            set.source(index).map(|source| source.bytes()),
            Some(JSON_BODY)
        );
    }
}

#[test]
fn a_refusing_sink_ends_the_attempt_and_no_later_declaration_is_read() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(91))
        .with(Json, "pages/b.de-de.json", json_reply(92))
        .with(Json, "pages/c.id-id.json", json_reply(93));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
        declaration(Some("pages/c.id-id.json"), None),
    ];

    let outcome = drive_declared_source_set_v1(&fake, &declarations, RecordingSink::refusing_at(1));

    assert_eq!(sink_refused(outcome), SinkRefusal::At(1));
    assert!(
        !fake
            .calls()
            .iter()
            .any(|call| call == "read:json:pages/c.id-id.json"),
        "a sink refusal must stop the driver before the next declaration is read"
    );
}

#[test]
fn a_sink_that_refuses_to_commit_returns_no_completed_set() {
    let fake = SetPort::new()
        .with(Json, "pages/a.en-001.json", json_reply(101))
        .with(Json, "pages/b.de-de.json", json_reply(102));
    let declarations = vec![
        declaration(Some("pages/a.en-001.json"), None),
        declaration(Some("pages/b.de-de.json"), None),
    ];

    let outcome =
        drive_declared_source_set_v1(&fake, &declarations, RecordingSink::refusing_finish());

    assert_eq!(sink_refused(outcome), SinkRefusal::Finish);
}
