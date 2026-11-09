//! Public-API tests for the source-read port and its use case, driven by a fake adapter.
//!
//! The fake records every call, so these tests can prove what the use case *asked for* —
//! in particular that a Markdown body is never read while a JSON body is available — as
//! well as what it decided. Nothing here touches a filesystem: these are the platform-
//! independent decisions, and the real filesystem cases live in the adapter crate.

use std::cell::RefCell;

use solo_application::{
    CandidateBodyV1, CandidateProbeV1, PhysicalIdentityV1, SourceReadOutcomeV1, SourceReadPort,
    SourceReadRefusalKind, SourceReadRefusalV1, check_physical_reservations_v1, fingerprint_of,
    read_declared_source_v1,
};
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
