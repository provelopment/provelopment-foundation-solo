//! Integration tests for inert body preflight and the in-memory source pipeline.
//!
//! Every case uses bytes already in hand and exercises the public API. No test reads,
//! lists or opens a file, and no fixture directory exists: the pipeline under test has no
//! file-opening capability at all, so "no fallback" is observable rather than asserted.

use solo_adapters::{
    BODY_CONTENT_VERSION, BodyPreflightRefusal, BodyPreflightRefusalKind, CONTENT_VALIDITY_PROVEN,
    InMemorySourceRefusalV1, InMemorySourceRequestV1, InertBodyCandidateV1, MAX_JSON_BODY_BYTES,
    MAX_MARKDOWN_BODY_BYTES, preflight_selected_body, run_in_memory_source_pipeline,
};
use solo_domain::{
    ProvisionalSourceChoiceV1, SourceCandidateDraftV1, SourceCandidateKind, SourceCandidatePathV1,
    SourceCandidateSetV1, SourceObservationV1, SuppliedObservationsV1, select_provisional_source,
};

const JSON_PATH: &str = "pages/home.en-001.json";
const MARKDOWN_PATH: &str = "pages/home.en-001.md";

/// The smallest accepted JSON body: one node, no module registry semantics claimed.
const MINIMAL_JSON: &[u8] = br#"{"contentVersion":"1.0.0","title":"Home","blocks":[{}]}"#;

/// A valid Markdown body.
const MINIMAL_MARKDOWN: &[u8] = b"# Home\n\nBody copy.\n";

fn path(value: &str, kind: SourceCandidateKind) -> SourceCandidatePathV1 {
    match SourceCandidatePathV1::parse(value, kind) {
        Ok(locator) => locator,
        Err(error) => panic!("{value:?} must be a valid locator, refused as {error}"),
    }
}

fn json_candidates() -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .json(path(JSON_PATH, SourceCandidateKind::Json))
        .expect("declare the json member");
    draft.build().expect("a declared member is a non-empty set")
}

fn markdown_candidates() -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .markdown(path(MARKDOWN_PATH, SourceCandidateKind::Markdown))
        .expect("declare the markdown member");
    draft.build().expect("a declared member is a non-empty set")
}

fn observations_json(observation: SourceObservationV1<'_>) -> SuppliedObservationsV1<'_> {
    SuppliedObservationsV1::new().with_json(observation)
}

fn observations_markdown(observation: SourceObservationV1<'_>) -> SuppliedObservationsV1<'_> {
    SuppliedObservationsV1::new().with_markdown(observation)
}

fn observations_both<'a>(
    json: SourceObservationV1<'a>,
    markdown: SourceObservationV1<'a>,
) -> SuppliedObservationsV1<'a> {
    let observations = SuppliedObservationsV1::new().with_json(json);
    observations.with_markdown(markdown)
}

fn select<'a>(
    candidates: &'a SourceCandidateSetV1,
    kind: SourceCandidateKind,
    bytes: &'a [u8],
) -> ProvisionalSourceChoiceV1<'a> {
    let present = SourceObservationV1::Present(bytes);
    let observations = match kind {
        SourceCandidateKind::Json => observations_json(present),
        SourceCandidateKind::Markdown => observations_markdown(present),
    };
    match select_provisional_source(candidates, observations) {
        Ok(choice) => choice,
        Err(refusal) => panic!("selection must succeed, refused as {refusal}"),
    }
}

fn preflight_json(bytes: &[u8]) -> Result<InertBodyCandidateV1, BodyPreflightRefusal> {
    let candidates = json_candidates();
    let choice = select(&candidates, SourceCandidateKind::Json, bytes);
    preflight_selected_body(&choice)
}

fn preflight_markdown(bytes: &[u8]) -> Result<InertBodyCandidateV1, BodyPreflightRefusal> {
    let candidates = markdown_candidates();
    let choice = select(&candidates, SourceCandidateKind::Markdown, bytes);
    preflight_selected_body(&choice)
}

fn json_refusal(bytes: &[u8]) -> BodyPreflightRefusalKind {
    match preflight_json(bytes) {
        Ok(candidate) => panic!(
            "body must be refused, accepted {} bytes as {}",
            candidate.len(),
            candidate.kind()
        ),
        Err(error) => {
            assert_eq!(error.code(), "PREFLIGHT_SOURCE_BODY_REFUSED");
            error.kind()
        }
    }
}

fn markdown_refusal(bytes: &[u8]) -> BodyPreflightRefusalKind {
    match preflight_markdown(bytes) {
        Ok(candidate) => panic!(
            "body must be refused, accepted {} bytes as {}",
            candidate.len(),
            candidate.kind()
        ),
        Err(error) => {
            assert_eq!(error.code(), "PREFLIGHT_SOURCE_BODY_REFUSED");
            error.kind()
        }
    }
}

/// A JSON body with the supplied `title` and exactly one empty block.
fn json_with_title(title: &str) -> Vec<u8> {
    let text = format!(
        r#"{{"contentVersion":"{BODY_CONTENT_VERSION}","title":"{title}","blocks":[{{}}]}}"#
    );
    text.into_bytes()
}

/// A JSON body with the supplied number of empty blocks.
fn json_with_blocks(count: usize) -> Vec<u8> {
    let items = vec!["{}"; count].join(",");
    let text = format!(
        r#"{{"contentVersion":"{BODY_CONTENT_VERSION}","title":"Home","blocks":[{items}]}}"#
    );
    text.into_bytes()
}

/// A JSON body of exactly `target` bytes, valid in every other respect.
fn json_of_exact_size(target: usize) -> Vec<u8> {
    let head = "{\"contentVersion\":\"1.0.0\",\"title\":\"Home\",\"blocks\":[\"";
    let tail = "\"]}";
    let filler = target - head.len() - tail.len();
    let mut text = String::with_capacity(target);
    text.push_str(head);
    text.push_str(&"a".repeat(filler));
    text.push_str(tail);
    let bytes = text.into_bytes();
    assert_eq!(bytes.len(), target);
    bytes
}

#[test]
fn preflight_proves_no_content_validity_and_no_authority() {
    let proven: bool = CONTENT_VALIDITY_PROVEN;
    assert!(!proven, "preflight is not content validation");

    let candidate = match preflight_json(MINIMAL_JSON) {
        Ok(candidate) => candidate,
        Err(error) => panic!("a minimal body must pass preflight, refused as {error}"),
    };
    assert_eq!(candidate.kind(), SourceCandidateKind::Json);
    assert_eq!(candidate.logical_path(), JSON_PATH);
    assert_eq!(candidate.bytes(), MINIMAL_JSON);
    assert!(!candidate.is_publication_authorized());
    assert!(!candidate.is_verified_filesystem_snapshot());
}

#[test]
fn preflight_preserves_the_selected_logical_path_and_kind_for_both_bodies() {
    let json = match preflight_json(MINIMAL_JSON) {
        Ok(candidate) => candidate,
        Err(error) => panic!("a minimal body must pass preflight, refused as {error}"),
    };
    assert_eq!(json.kind(), SourceCandidateKind::Json);
    assert_eq!(json.logical_path(), JSON_PATH);

    let markdown = match preflight_markdown(MINIMAL_MARKDOWN) {
        Ok(candidate) => candidate,
        Err(error) => panic!("a minimal body must pass preflight, refused as {error}"),
    };
    assert_eq!(markdown.kind(), SourceCandidateKind::Markdown);
    assert_eq!(markdown.logical_path(), MARKDOWN_PATH);
    assert_eq!(markdown.bytes(), MINIMAL_MARKDOWN);
}

#[test]
fn preflight_owns_its_bytes_so_a_caller_cannot_change_them() {
    let mut supplied = MINIMAL_JSON.to_vec();
    let candidate = match preflight_json(&supplied) {
        Ok(candidate) => candidate,
        Err(error) => panic!("a minimal body must pass preflight, refused as {error}"),
    };

    // Reuse and overwrite the caller's buffer, then confirm the owned copy is unaffected.
    supplied.clear();
    supplied.extend_from_slice(b"tampered");
    assert_eq!(candidate.bytes(), MINIMAL_JSON);
    assert_ne!(candidate.bytes(), supplied.as_slice());
}

#[test]
fn the_closed_body_root_is_enforced() {
    assert_eq!(
        json_refusal(br#"["not-an-object"]"#),
        BodyPreflightRefusalKind::NonObjectRoot
    );
    assert_eq!(
        json_refusal(br#"{"title":"Home","blocks":[{}]}"#),
        BodyPreflightRefusalKind::MissingMember
    );
    assert_eq!(
        json_refusal(br#"{"contentVersion":"1.0.0","blocks":[{}]}"#),
        BodyPreflightRefusalKind::MissingMember
    );
    assert_eq!(
        json_refusal(br#"{"contentVersion":"1.0.0","title":"Home"}"#),
        BodyPreflightRefusalKind::MissingMember
    );
    assert_eq!(
        json_refusal(br#"{"title":"a","blocks":[{}],"draft":true}"#),
        BodyPreflightRefusalKind::UnknownMember
    );
    assert_eq!(
        json_refusal(br#"{"contentVersion":"2.0.0","title":"a","blocks":[1]}"#),
        BodyPreflightRefusalKind::UnsupportedContentVersion
    );
    assert_eq!(
        json_refusal(br#"{"contentVersion":"1.0.0","title":"Home","blocks":{}}"#),
        BodyPreflightRefusalKind::InvalidBlocks
    );
}

#[test]
fn title_and_summary_scalar_bounds_are_enforced() {
    let longest_title = "a".repeat(140);
    assert!(
        preflight_json(&json_with_title(&longest_title)).is_ok(),
        "140 title scalars must be accepted"
    );
    assert_eq!(
        json_refusal(&json_with_title(&"a".repeat(141))),
        BodyPreflightRefusalKind::InvalidTitle
    );
    assert_eq!(
        json_refusal(&json_with_title("")),
        BodyPreflightRefusalKind::InvalidTitle
    );

    let with_summary = |summary: &str| {
        let text = format!(
            r#"{{"contentVersion":"1.0.0","title":"Home","summary":"{summary}","blocks":[{{}}]}}"#
        );
        text.into_bytes()
    };
    assert!(
        preflight_json(&with_summary(&"a".repeat(500))).is_ok(),
        "500 summary scalars must be accepted"
    );
    assert_eq!(
        json_refusal(&with_summary(&"a".repeat(501))),
        BodyPreflightRefusalKind::InvalidSummary
    );

    // A non-string title is refused rather than coerced.
    assert_eq!(
        json_refusal(br#"{"contentVersion":"1.0.0","title":7,"blocks":[{}]}"#),
        BodyPreflightRefusalKind::InvalidTitle
    );
}

#[test]
fn block_count_bounds_are_enforced() {
    assert_eq!(
        json_refusal(&json_with_blocks(0)),
        BodyPreflightRefusalKind::InvalidBlocks
    );
    assert_eq!(
        json_refusal(&json_with_blocks(251)),
        BodyPreflightRefusalKind::InvalidBlocks
    );
    assert!(
        preflight_json(&json_with_blocks(250)).is_ok(),
        "250 blocks must be accepted"
    );
    assert!(
        preflight_json(&json_with_blocks(1)).is_ok(),
        "one block must be accepted"
    );
}

#[test]
fn byte_level_faults_are_refused_for_both_body_kinds() {
    assert_eq!(json_refusal(b""), BodyPreflightRefusalKind::Empty);
    assert_eq!(markdown_refusal(b""), BodyPreflightRefusalKind::Empty);

    let mut bom_json = String::from("\u{feff}").into_bytes();
    bom_json.extend_from_slice(MINIMAL_JSON);
    assert_eq!(
        json_refusal(&bom_json),
        BodyPreflightRefusalKind::ByteOrderMark
    );

    let mut bom_markdown = String::from("\u{feff}").into_bytes();
    bom_markdown.extend_from_slice(MINIMAL_MARKDOWN);
    assert_eq!(
        markdown_refusal(&bom_markdown),
        BodyPreflightRefusalKind::ByteOrderMark
    );

    // A lone continuation byte is never valid UTF-8.
    assert_eq!(
        json_refusal(&[0x7B, 0x80, 0x7D]),
        BodyPreflightRefusalKind::InvalidUtf8
    );
    assert_eq!(
        markdown_refusal(&[b'#', b' ', 0x80]),
        BodyPreflightRefusalKind::InvalidUtf8
    );
}

#[test]
fn json_syntax_faults_are_refused_without_fallback() {
    assert_eq!(
        json_refusal(br#"{"title":"a"} trailing"#),
        BodyPreflightRefusalKind::MalformedJson
    );
    assert_eq!(
        json_refusal(b"{\"blocks\":[{},]}"),
        BodyPreflightRefusalKind::MalformedJson
    );
    assert_eq!(
        json_refusal(b"{/* comment */}"),
        BodyPreflightRefusalKind::MalformedJson
    );
    assert_eq!(
        json_refusal(br#"{"blocks":[{"a":NaN}]}"#),
        BodyPreflightRefusalKind::MalformedJson
    );
}

#[test]
fn duplicate_members_are_refused_at_the_root_and_at_depth() {
    assert_eq!(
        json_refusal(br#"{"title":"a","title":"b"}"#),
        BodyPreflightRefusalKind::DuplicateMember
    );
    assert_eq!(
        json_refusal(br#"{"a":{"b":{"c":1,"c":2}}}"#),
        BodyPreflightRefusalKind::DuplicateMember
    );
}

#[test]
fn size_boundaries_are_exact_for_both_body_kinds() {
    let at_limit = json_of_exact_size(MAX_JSON_BODY_BYTES);
    assert!(
        preflight_json(&at_limit).is_ok(),
        "exactly {MAX_JSON_BODY_BYTES} JSON bytes must be accepted"
    );
    let over_limit = json_of_exact_size(MAX_JSON_BODY_BYTES + 1);
    assert_eq!(
        json_refusal(&over_limit),
        BodyPreflightRefusalKind::TooLarge
    );

    let markdown_at_limit = vec![b'a'; MAX_MARKDOWN_BODY_BYTES];
    assert!(
        preflight_markdown(&markdown_at_limit).is_ok(),
        "exactly {MAX_MARKDOWN_BODY_BYTES} Markdown bytes must be accepted"
    );
    let markdown_over_limit = vec![b'a'; MAX_MARKDOWN_BODY_BYTES + 1];
    assert_eq!(
        markdown_refusal(&markdown_over_limit),
        BodyPreflightRefusalKind::TooLarge
    );
}

#[test]
fn a_refusal_never_echoes_the_authored_body() {
    let body = br#"{"contentVersion":"1.0.0","title":"SECRET-TITLE","blocks":[]}"#;
    let candidates = json_candidates();
    let choice = select(&candidates, SourceCandidateKind::Json, body);
    let error = match preflight_selected_body(&choice) {
        Ok(candidate) => panic!("body must be refused, got {} bytes", candidate.len()),
        Err(error) => error,
    };

    let rendered = error.to_string();
    assert!(!rendered.contains("SECRET"));
    assert!(rendered.contains("PREFLIGHT_SOURCE_BODY_REFUSED"));
    assert_eq!(error.location(), "blocks");
}

/// One Entity, one Facet, one Site, one Page, one representation declaring both candidates.
fn manifest_with_both_candidates() -> Vec<u8> {
    br#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.test" },
  "entity": {
    "id": "acme",
    "label": "Acme",
    "facets": [
      {
        "id": "main",
        "label": "Main",
        "sites": [
          {
            "id": "site",
            "label": "Site",
            "defaultLocale": "en-001",
            "pages": [
              {
                "id": "home",
                "label": "Home",
                "representations": [
                  {
                    "locale": "en-001",
                    "route": { "path": "/" },
                    "sourceCandidates": {
                      "json": "pages/home.en-001.json",
                      "markdown": "pages/home.en-001.md"
                    }
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  }
}"#
    .to_vec()
}

fn pipeline_error(request: &InMemorySourceRequestV1<'_>) -> InMemorySourceRefusalV1 {
    match run_in_memory_source_pipeline(request) {
        Ok(candidate) => panic!("the pipeline must refuse, got {} bytes", candidate.len()),
        Err(error) => error,
    }
}

fn pipeline_request<'a>(
    manifest: &'a [u8],
    locale: &'a str,
    observations: SuppliedObservationsV1<'a>,
) -> InMemorySourceRequestV1<'a> {
    InMemorySourceRequestV1::new(manifest, "main", "site", "home", locale, observations)
}

#[test]
fn the_pipeline_selects_and_preflights_only_the_chosen_body() {
    let manifest = manifest_with_both_candidates();
    let present = SourceObservationV1::Present(MINIMAL_JSON);
    let observations = observations_json(present);
    let request = pipeline_request(&manifest, "en-001", observations);

    let candidate = match run_in_memory_source_pipeline(&request) {
        Ok(candidate) => candidate,
        Err(error) => panic!("the pipeline must succeed, refused as {error}"),
    };

    assert_eq!(candidate.kind(), SourceCandidateKind::Json);
    assert_eq!(candidate.logical_path(), JSON_PATH);
    assert_eq!(candidate.bytes(), MINIMAL_JSON);
    assert!(!candidate.is_publication_authorized());
}

#[test]
fn the_pipeline_refuses_invalid_present_json_without_ever_touching_markdown() {
    // A perfectly valid Markdown body exists in this scope, but it is never supplied.
    let _unused_valid_markdown_elsewhere: &[u8] = MINIMAL_MARKDOWN;
    let manifest = manifest_with_both_candidates();
    let invalid_json = br#"{"contentVersion":"1.0.0","title":"Home","blocks":["#;
    let present = SourceObservationV1::Present(invalid_json);
    let observations = observations_json(present);
    let request = pipeline_request(&manifest, "en-001", observations);

    let error = pipeline_error(&request);
    assert!(
        matches!(error, InMemorySourceRefusalV1::Body(_)),
        "present invalid JSON must be a body refusal, got {error}"
    );
    assert_eq!(error.code(), "PREFLIGHT_SOURCE_BODY_REFUSED");
}

#[test]
fn the_pipeline_chooses_markdown_only_after_a_reported_json_absence() {
    let manifest = manifest_with_both_candidates();
    let absent = SourceObservationV1::Absent;
    let present = SourceObservationV1::Present(MINIMAL_MARKDOWN);
    let observations = observations_both(absent, present);
    let request = pipeline_request(&manifest, "en-001", observations);

    let candidate = match run_in_memory_source_pipeline(&request) {
        Ok(candidate) => candidate,
        Err(error) => panic!("the pipeline must succeed, refused as {error}"),
    };
    assert_eq!(candidate.kind(), SourceCandidateKind::Markdown);
    assert_eq!(candidate.logical_path(), MARKDOWN_PATH);
    assert_eq!(candidate.bytes(), MINIMAL_MARKDOWN);
}

#[test]
fn the_pipeline_refuses_a_redundant_markdown_observation() {
    let manifest = manifest_with_both_candidates();
    let present = SourceObservationV1::Present(MINIMAL_JSON);
    let markdown = SourceObservationV1::Present(MINIMAL_MARKDOWN);
    let observations = observations_both(present, markdown);
    let request = pipeline_request(&manifest, "en-001", observations);

    let error = pipeline_error(&request);
    assert!(
        matches!(error, InMemorySourceRefusalV1::Selection(_)),
        "a redundant Markdown observation must be a selection refusal, got {error}"
    );
    assert_eq!(error.code(), "PREFLIGHT_SOURCE_SELECTION_REFUSED");
}

#[test]
fn the_pipeline_refuses_a_missing_observation() {
    let manifest = manifest_with_both_candidates();
    let observations = SuppliedObservationsV1::new();
    let request = pipeline_request(&manifest, "en-001", observations);

    let error = pipeline_error(&request);
    assert!(matches!(error, InMemorySourceRefusalV1::Selection(_)));
    assert_eq!(error.code(), "PREFLIGHT_SOURCE_SELECTION_REFUSED");
}

#[test]
fn the_pipeline_refuses_an_unmatched_representation() {
    let manifest = manifest_with_both_candidates();
    let observations = SuppliedObservationsV1::new();
    let request = pipeline_request(&manifest, "de-DE", observations);

    let error = pipeline_error(&request);
    let unmatched = matches!(error, InMemorySourceRefusalV1::UnknownRepresentation);
    assert!(unmatched);
    let code = error.code();
    assert_eq!(code, "PREFLIGHT_SOURCE_PIPELINE_UNKNOWN_REPRESENTATION");
}

#[test]
fn the_pipeline_refuses_a_manifest_the_parser_refuses() {
    let manifest = b"{}";
    let observations = SuppliedObservationsV1::new();
    let request = pipeline_request(manifest, "en-001", observations);

    let error = pipeline_error(&request);
    assert!(matches!(error, InMemorySourceRefusalV1::Manifest(_)));
    assert_eq!(error.code(), "PREFLIGHT_MANIFEST_INVALID");
}
