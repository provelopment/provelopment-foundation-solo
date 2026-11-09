//! Integration tests for the pure provisional source-selection policy.
//!
//! Every case exercises the public API with observations already in hand. No test reads,
//! lists or opens a file, because the policy under test performs no I/O: an `Absent`
//! observation here is a caller-supplied claim, which is exactly what the module accepts.

use solo_domain::{
    OBSERVATION_IS_FILESYSTEM_PROOF, PROVISIONAL_CHOICE_IS_FINAL_SELECTION,
    ProvisionalSourceChoiceV1, SourceCandidateDraftV1, SourceCandidateKind, SourceCandidatePathV1,
    SourceCandidateSetV1, SourceFailureKind, SourceObservationV1, SourceSelectionRefusal,
    SourceSelectionRefusalKind, SuppliedObservationsV1, select_provisional_source,
};

const JSON_BODY: &[u8] = b"{\"contentVersion\":\"1.0.0\"}";
const MARKDOWN_BODY: &[u8] = b"# Title\n";

fn locator(path: &str, kind: SourceCandidateKind) -> SourceCandidatePathV1 {
    match SourceCandidatePathV1::parse(path, kind) {
        Ok(value) => value,
        Err(error) => panic!("{path:?} must be a valid locator, refused as {error}"),
    }
}

/// A declaration carrying only the `json` member.
fn json_only(path: &str) -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .json(locator(path, SourceCandidateKind::Json))
        .expect("declare the json member");
    draft.build().expect("a declared member is a non-empty set")
}

/// A declaration carrying only the `markdown` member.
fn markdown_only(path: &str) -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .markdown(locator(path, SourceCandidateKind::Markdown))
        .expect("declare the markdown member");
    draft.build().expect("a declared member is a non-empty set")
}

/// A declaration carrying both members, `json` authored first.
fn both(json_path: &str, markdown_path: &str) -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .json(locator(json_path, SourceCandidateKind::Json))
        .expect("declare the json member");
    draft
        .markdown(locator(markdown_path, SourceCandidateKind::Markdown))
        .expect("declare the markdown member");
    draft.build().expect("a declared member is a non-empty set")
}

fn chosen<'a>(
    result: Result<ProvisionalSourceChoiceV1<'a>, SourceSelectionRefusal>,
) -> ProvisionalSourceChoiceV1<'a> {
    match result {
        Ok(choice) => choice,
        Err(refusal) => panic!("selection must succeed, refused as {refusal}"),
    }
}

fn refused<'a>(
    result: Result<ProvisionalSourceChoiceV1<'a>, SourceSelectionRefusal>,
) -> SourceSelectionRefusal {
    match result {
        Ok(choice) => panic!(
            "selection must be refused, chose {} at {}",
            choice.kind(),
            choice.logical_path()
        ),
        Err(refusal) => {
            assert_eq!(refusal.code(), "PREFLIGHT_SOURCE_SELECTION_REFUSED");
            refusal
        }
    }
}

#[test]
fn a_provisional_choice_claims_no_authority_and_no_snapshot() {
    // Bind first so these stay runtime observations rather than constant assertions.
    let filesystem_proof: bool = OBSERVATION_IS_FILESYSTEM_PROOF;
    let final_selection: bool = PROVISIONAL_CHOICE_IS_FINAL_SELECTION;
    assert!(!filesystem_proof, "an observation proves no filesystem state");
    assert!(!final_selection, "selection stays provisional");

    let candidates = json_only("pages/home.en-001.json");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(JSON_BODY)),
    ));
    assert!(!choice.is_publication_authorized());
    assert!(!choice.is_verified_filesystem_snapshot());
}

#[test]
fn a_json_only_declaration_chooses_present_json() {
    let candidates = json_only("pages/home.en-001.json");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(JSON_BODY)),
    ));

    assert_eq!(choice.kind(), SourceCandidateKind::Json);
    assert_eq!(choice.logical_path(), "pages/home.en-001.json");
    assert_eq!(choice.supplied_bytes(), JSON_BODY);
}

#[test]
fn a_markdown_only_declaration_chooses_present_markdown() {
    let candidates = markdown_only("pages/about.en-001.md");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));

    assert_eq!(choice.kind(), SourceCandidateKind::Markdown);
    assert_eq!(choice.logical_path(), "pages/about.en-001.md");
    assert_eq!(choice.supplied_bytes(), MARKDOWN_BODY);
}

#[test]
fn present_json_wins_when_both_members_are_declared() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(JSON_BODY)),
    ));

    assert_eq!(choice.kind(), SourceCandidateKind::Json);
    assert_eq!(choice.logical_path(), "pages/home.en-001.json");
}

#[test]
fn reported_json_absence_permits_present_markdown() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Absent)
            .with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));

    assert_eq!(choice.kind(), SourceCandidateKind::Markdown);
    assert_eq!(choice.logical_path(), "pages/home.en-001.md");
}

#[test]
fn two_reported_absences_are_neither_present_and_never_an_empty_page() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Absent)
            .with_markdown(SourceObservationV1::Absent),
    ));

    assert_eq!(refusal.kind(), SourceSelectionRefusalKind::NeitherPresent);
    assert_eq!(refusal.member(), None);
    assert_eq!(refusal.failure(), None);
}

#[test]
fn a_json_failure_is_an_immediate_refusal_even_when_markdown_is_declared() {
    // Markdown is logically available and could have supplied a body, but a JSON failure
    // is never a reason to fall back.
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Failure(
                SourceFailureKind::PermissionDenied,
            ))
            .with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));

    assert_eq!(refusal.kind(), SourceSelectionRefusalKind::ObservationFailure);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Json));
    assert_eq!(refusal.failure(), Some(SourceFailureKind::PermissionDenied));
}

#[test]
fn every_observation_failure_kind_stays_distinct_and_never_becomes_absence() {
    let cases = [
        SourceFailureKind::PermissionDenied,
        SourceFailureKind::OtherIo,
        SourceFailureKind::UnsafeLocation,
        SourceFailureKind::ChangedDuringRead,
    ];

    let mut codes = Vec::new();
    for failure in cases {
        let candidates = json_only("pages/home.en-001.json");
        let refusal = refused(select_provisional_source(
            &candidates,
            SuppliedObservationsV1::new().with_json(SourceObservationV1::Failure(failure)),
        ));
        assert_eq!(refusal.kind(), SourceSelectionRefusalKind::ObservationFailure);
        assert_eq!(refusal.failure(), Some(failure));
        codes.push(failure.code());
    }

    assert_eq!(codes.len(), 4);
    for (index, code) in codes.iter().enumerate() {
        assert!(
            !codes[..index].contains(code),
            "failure code {code} must be distinct"
        );
    }
}

#[test]
fn markdown_body_bytes_are_refused_when_json_is_present() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Present(JSON_BODY))
            .with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));

    assert_eq!(
        refusal.kind(),
        SourceSelectionRefusalKind::RedundantMarkdownObservation
    );
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Markdown));
}

#[test]
fn an_observation_for_an_undeclared_member_is_refused() {
    let candidates = json_only("pages/home.en-001.json");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Present(JSON_BODY))
            .with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));
    assert_eq!(
        refusal.kind(),
        SourceSelectionRefusalKind::UndeclaredCandidateObservation
    );
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Markdown));

    let candidates = markdown_only("pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Present(JSON_BODY))
            .with_markdown(SourceObservationV1::Present(MARKDOWN_BODY)),
    ));
    assert_eq!(
        refusal.kind(),
        SourceSelectionRefusalKind::UndeclaredCandidateObservation
    );
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Json));
}

#[test]
fn a_declared_member_without_an_observation_is_refused() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new(),
    ));
    assert_eq!(refusal.kind(), SourceSelectionRefusalKind::MissingObservation);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Json));

    // A reported-absent JSON still requires the declared Markdown observation.
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Absent),
    ));
    assert_eq!(refusal.kind(), SourceSelectionRefusalKind::MissingObservation);
    assert_eq!(refusal.member(), Some(SourceCandidateKind::Markdown));
}

#[test]
fn byte_empty_and_explicit_absence_are_different_outcomes() {
    // Empty bytes are a present observation the policy accepts; the separate body
    // preflight is what refuses them. Absence is never inferred from emptiness.
    let candidates = json_only("pages/home.en-001.json");
    let choice = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(b"")),
    ));
    assert_eq!(choice.kind(), SourceCandidateKind::Json);
    assert!(choice.supplied_bytes().is_empty());

    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Absent),
    ));
    assert_eq!(refusal.kind(), SourceSelectionRefusalKind::NeitherPresent);
}

#[test]
fn the_decision_never_varies_with_enumeration_or_repetition() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let first = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(JSON_BODY)),
    ));
    let second = chosen(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new().with_json(SourceObservationV1::Present(JSON_BODY)),
    ));

    assert_eq!(first.kind(), second.kind());
    assert_eq!(first.logical_path(), second.logical_path());
    assert_eq!(first.supplied_bytes(), second.supplied_bytes());
    assert_eq!(
        candidates.iter().count(),
        2,
        "the declaration itself is unchanged by selection"
    );
}

#[test]
fn diagnostics_never_echo_a_body() {
    let candidates = both("pages/home.en-001.json", "pages/home.en-001.md");
    let refusal = refused(select_provisional_source(
        &candidates,
        SuppliedObservationsV1::new()
            .with_json(SourceObservationV1::Failure(SourceFailureKind::UnsafeLocation))
            .with_markdown(SourceObservationV1::Present(b"secret markdown body")),
    ));

    let rendered = refusal.to_string();
    assert!(!rendered.contains("secret"));
    assert!(!rendered.contains("markdown body"));
    assert!(rendered.contains("PREFLIGHT_SOURCE_SELECTION_REFUSED"));
    assert!(rendered.contains("SOURCE_OBSERVATION_UNSAFE_LOCATION"));
}
