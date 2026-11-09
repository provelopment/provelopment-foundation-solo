//! Integration tests for the pure lexical `sourceCandidates` locator.
//!
//! These exercise the public API only. Nothing here reads, lists or opens a file: the
//! module under test is pure, so "absent candidate" cases are genuine because no file
//! is ever consulted.

use solo_domain::{
    FILESYSTEM_IDENTITY_PROVEN, MAX_LOCATOR_BYTES, SOURCE_NAMESPACE, SourceCandidateDraftV1,
    SourceCandidateKind, SourceCandidatePathV1, SourceCandidateReservationsV1,
    SourceClaimErrorKind, SourceLocatorErrorKind,
};

fn reject(input: &str, kind: SourceCandidateKind) -> SourceLocatorErrorKind {
    match SourceCandidatePathV1::parse(input, kind) {
        Ok(value) => panic!("{input:?} must be refused, accepted as {value}"),
        Err(error) => {
            assert_eq!(error.code(), "PREFLIGHT_SOURCE_LOCATOR_INVALID");
            error.kind()
        }
    }
}

fn accept(input: &str, kind: SourceCandidateKind) -> SourceCandidatePathV1 {
    match SourceCandidatePathV1::parse(input, kind) {
        Ok(value) => value,
        Err(error) => panic!("{input:?} must be accepted, refused as {error}"),
    }
}

#[test]
fn logical_candidate_comparison_is_not_filesystem_identity() {
    // The module must never claim alias/hardlink/case identity; that is the future
    // confined snapshot adapter's job (§6.2C.3). Bind first so this stays a runtime
    // observation rather than a constant assertion.
    let proven: bool = FILESYSTEM_IDENTITY_PROVEN;
    assert!(
        !proven,
        "logical comparison must not claim filesystem identity"
    );
    assert_eq!(SOURCE_NAMESPACE, "pages");
}

#[test]
fn accepts_the_normative_minimal_and_expanded_locators() {
    let json = accept("pages/main-home.en-001.json", SourceCandidateKind::Json);
    assert_eq!(json.as_str(), "pages/main-home.en-001.json");
    assert_eq!(json.kind(), SourceCandidateKind::Json);
    assert_eq!(json.to_string(), "pages/main-home.en-001.json");

    let markdown = accept("pages/main-home.en-001.md", SourceCandidateKind::Markdown);
    assert_eq!(markdown.kind(), SourceCandidateKind::Markdown);

    // Multiple segments, underscores, hyphens, digits and dots are all portable.
    accept(
        "pages/section/entry_01.part-a.json",
        SourceCandidateKind::Json,
    );
    accept("pages/a/b/c/d/e/f.json", SourceCandidateKind::Json);
    // A dot inside the stem is portable, because only a trailing dot or a doubled dot
    // is refused.
    let multi_dot = accept("pages/a.b.json", SourceCandidateKind::Json);
    assert_eq!(multi_dot.as_str(), "pages/a.b.json");
    assert_eq!(MAX_LOCATOR_BYTES, 240);
}

#[test]
fn refuses_every_unsafe_or_alias_spelling() {
    let json = SourceCandidateKind::Json;
    assert_eq!(reject("", json), SourceLocatorErrorKind::Empty);
    assert_eq!(
        reject("/pages/a.json", json),
        SourceLocatorErrorKind::LeadingSlash
    );
    assert_eq!(
        reject("pages/a.json/", json),
        SourceLocatorErrorKind::TrailingSlash
    );
    assert_eq!(
        reject("pages//a.json", json),
        SourceLocatorErrorKind::EmptySegment
    );
    assert_eq!(
        reject("pages\\a.json", json),
        SourceLocatorErrorKind::Backslash
    );
    assert_eq!(
        reject("pages/./a.json", json),
        SourceLocatorErrorKind::DotSegment
    );
    assert_eq!(
        reject("pages/../a.json", json),
        SourceLocatorErrorKind::DotDotSegment
    );
    assert_eq!(
        reject("c:/pages/a.json", json),
        SourceLocatorErrorKind::Colon
    );
    assert_eq!(
        reject("pages/a.json:x", json),
        SourceLocatorErrorKind::Colon
    );
    assert_eq!(
        reject("~/pages/a.json", json),
        SourceLocatorErrorKind::Tilde
    );
    assert_eq!(
        reject("pages/a.json?x=1", json),
        SourceLocatorErrorKind::QueryOrFragment
    );
    assert_eq!(
        reject("pages/a.json#top", json),
        SourceLocatorErrorKind::QueryOrFragment
    );
    assert_eq!(
        reject("pages/a%2ejson", json),
        SourceLocatorErrorKind::PercentEscape
    );
    assert_eq!(
        reject("pages/a\u{1}.json", json),
        SourceLocatorErrorKind::ControlCharacter
    );
    assert_eq!(
        reject("pages/\u{0}.json", json),
        SourceLocatorErrorKind::ControlCharacter
    );
    assert_eq!(
        reject("pages/é.json", json),
        SourceLocatorErrorKind::NonAscii
    );
    assert_eq!(
        reject("content/pages/a.json", json),
        SourceLocatorErrorKind::NotUnderSourceNamespace
    );
    assert_eq!(
        reject("pages.json", json),
        SourceLocatorErrorKind::NotUnderSourceNamespace
    );
}

#[test]
fn refuses_unsafe_segments_reserved_names_and_bad_extensions() {
    let json = SourceCandidateKind::Json;
    assert_eq!(
        reject("pages/A.json", json),
        SourceLocatorErrorKind::SegmentInvalidStart
    );
    assert_eq!(
        reject("pages/1a.json", json),
        SourceLocatorErrorKind::SegmentInvalidStart
    );
    assert_eq!(
        reject("pages/a b.json", json),
        SourceLocatorErrorKind::SegmentInvalidCharacter
    );
    assert_eq!(
        reject("pages/a.txt", json),
        SourceLocatorErrorKind::WrongExtension
    );
    assert_eq!(
        reject("pages/a.md", json),
        SourceLocatorErrorKind::ExtensionMismatch
    );
    assert_eq!(
        reject("pages/a.json", SourceCandidateKind::Markdown),
        SourceLocatorErrorKind::ExtensionMismatch
    );
    assert_eq!(
        reject("pages/a./b.json", json),
        SourceLocatorErrorKind::SegmentTrailingDot
    );
    assert_eq!(
        reject("pages/a..b.json", json),
        SourceLocatorErrorKind::SegmentConsecutiveDots
    );
    // Windows reserves the stem regardless of the extension that follows it.
    assert_eq!(
        reject("pages/con.json", json),
        SourceLocatorErrorKind::ReservedWindowsStem
    );
    assert_eq!(
        reject("pages/con.report.json", json),
        SourceLocatorErrorKind::ReservedWindowsStem
    );
    assert_eq!(
        reject("pages/lpt1.md", SourceCandidateKind::Markdown),
        SourceLocatorErrorKind::ReservedWindowsStem
    );
    assert_eq!(
        reject("pages/aux/site.json", json),
        SourceLocatorErrorKind::ReservedWindowsStem
    );
}

#[test]
fn refuses_oversized_locators_and_excess_depth() {
    let json = SourceCandidateKind::Json;
    let oversized = format!("pages/{}.json", "a".repeat(230));
    assert!(oversized.len() > MAX_LOCATOR_BYTES);
    assert_eq!(reject(&oversized, json), SourceLocatorErrorKind::TooLong);

    let long_segment = format!("pages/{}.json", "a".repeat(81));
    assert!(long_segment.len() <= MAX_LOCATOR_BYTES);
    assert_eq!(
        reject(&long_segment, json),
        SourceLocatorErrorKind::SegmentTooLong
    );

    let deep = "pages/a/a/a/a/a/a/a/a.json";
    assert_eq!(reject(deep, json), SourceLocatorErrorKind::TooManySegments);
}

#[test]
fn a_candidate_set_is_one_or_two_named_members_and_never_empty() {
    let empty = SourceCandidateDraftV1::new();
    assert_eq!(
        empty.build().expect_err("an empty set is refused").kind(),
        SourceLocatorErrorKind::EmptyCandidateSet
    );

    let json = accept("pages/home.en-001.json", SourceCandidateKind::Json);
    let markdown = accept("pages/home.en-001.md", SourceCandidateKind::Markdown);

    let mut only_json = SourceCandidateDraftV1::new();
    only_json.json(json.clone()).expect("first json member");
    let set = only_json.build().expect("non-empty set");
    assert_eq!(set.len(), 1);
    assert!(!set.is_empty());
    assert!(set.json().is_some());
    assert!(set.markdown().is_none());
    assert_eq!(set.iter().count(), 1);

    let mut both = SourceCandidateDraftV1::new();
    both.json(json).expect("json member");
    both.markdown(markdown).expect("markdown member");
    let set = both.build().expect("both members");
    assert_eq!(set.len(), 2);
    let members: Vec<&str> = set.iter().map(SourceCandidatePathV1::as_str).collect();
    assert_eq!(
        members,
        vec!["pages/home.en-001.json", "pages/home.en-001.md"]
    );
}

#[test]
fn declaring_one_member_twice_is_refused_not_overwritten() {
    let json = accept("pages/home.en-001.json", SourceCandidateKind::Json);
    let mut draft = SourceCandidateDraftV1::new();
    draft.json(json.clone()).expect("first declaration");
    let error = draft
        .json(json)
        .expect_err("a second json member is refused");
    assert_eq!(error.kind(), SourceLocatorErrorKind::DuplicateMember);
    assert_eq!(error.detail(), "duplicate-member");

    let markdown = accept("pages/home.en-001.md", SourceCandidateKind::Markdown);
    let mut draft = SourceCandidateDraftV1::new();
    draft.markdown(markdown.clone()).expect("first declaration");
    let error = draft
        .markdown(markdown)
        .expect_err("a second markdown member is refused");
    assert_eq!(error.kind(), SourceLocatorErrorKind::DuplicateMember);
}

#[test]
fn two_representations_cannot_reserve_one_logical_path() {
    let mut first = SourceCandidateDraftV1::new();
    first
        .json(accept("pages/home.en-001.json", SourceCandidateKind::Json))
        .expect("json");
    first
        .markdown(accept(
            "pages/home.en-001.md",
            SourceCandidateKind::Markdown,
        ))
        .expect("markdown");
    let first = first.build().expect("set");

    let mut second = SourceCandidateDraftV1::new();
    second
        .json(accept("pages/home.en-001.json", SourceCandidateKind::Json))
        .expect("json");
    second
        .markdown(accept("pages/home.de-de.md", SourceCandidateKind::Markdown))
        .expect("markdown");
    let second = second.build().expect("set");

    let mut reservations = SourceCandidateReservationsV1::new();
    reservations.reserve(7, &first).expect("first claim wins");
    assert_eq!(reservations.len(), 2);
    assert_eq!(reservations.owner_of("pages/home.en-001.json"), Some(7));

    let error = reservations
        .reserve(9, &second)
        .expect_err("the same logical json path cannot be claimed twice");
    assert_eq!(error.code(), "PREFLIGHT_SOURCE_CLAIM_DUPLICATE");
    assert_eq!(error.detail(), "duplicate-claim");
    assert_eq!(error.first(), 7);
    assert_eq!(error.second(), 9);
    // The refused owner must not half-claim its markdown path either.
    assert_eq!(reservations.len(), 2);
    assert_eq!(reservations.owner_of("pages/home.de-de.md"), None);
}

#[test]
fn the_winner_never_depends_on_declaration_order() {
    let mut a = SourceCandidateDraftV1::new();
    a.json(accept("pages/home.en-001.json", SourceCandidateKind::Json))
        .expect("json");
    let a = a.build().expect("set");

    let mut b = SourceCandidateDraftV1::new();
    b.json(accept("pages/home.en-001.json", SourceCandidateKind::Json))
        .expect("json");
    let b = b.build().expect("set");

    let mut forward = SourceCandidateReservationsV1::new();
    forward.reserve(1, &a).expect("first");
    assert!(forward.reserve(2, &b).is_err(), "second must be refused");

    let mut backward = SourceCandidateReservationsV1::new();
    backward.reserve(2, &b).expect("first");
    assert!(backward.reserve(1, &a).is_err(), "second must be refused");
}

#[test]
fn an_absent_declared_candidate_still_reserves_its_path() {
    // The file genuinely does not exist, yet the declaration alone reserves the path,
    // because this module never consults a filesystem (§6.2C.4).
    let absent = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("content")
        .join("pages")
        .join("never-written.json");
    assert!(!absent.exists(), "this test requires the file to be absent");

    let mut owner_one = SourceCandidateDraftV1::new();
    owner_one
        .json(accept(
            "pages/never-written.json",
            SourceCandidateKind::Json,
        ))
        .expect("json");
    let owner_one = owner_one.build().expect("set");

    let mut owner_two = SourceCandidateDraftV1::new();
    owner_two
        .json(accept(
            "pages/never-written.json",
            SourceCandidateKind::Json,
        ))
        .expect("json");
    let owner_two = owner_two.build().expect("set");

    let mut reservations = SourceCandidateReservationsV1::new();
    reservations.reserve(1, &owner_one).expect("first claim");
    let error = reservations
        .reserve(2, &owner_two)
        .expect_err("an absent candidate still reserves its logical path");
    assert_eq!(error.kind(), SourceClaimErrorKind::DuplicateClaim);
}

#[test]
fn one_owner_may_reclaim_its_own_path() {
    let mut draft = SourceCandidateDraftV1::new();
    draft
        .json(accept("pages/home.en-001.json", SourceCandidateKind::Json))
        .expect("json");
    let set = draft.build().expect("set");

    let mut reservations = SourceCandidateReservationsV1::new();
    reservations.reserve(4, &set).expect("first claim");
    reservations
        .reserve(4, &set)
        .expect("the same owner repeating its claim is not a collision");
    assert_eq!(reservations.len(), 1);
}

#[test]
fn a_refusal_never_leaks_the_authored_value() {
    let refused = "pages/../secrets.json";
    let error = SourceCandidatePathV1::parse(refused, SourceCandidateKind::Json)
        .expect_err("traversal is refused");
    let rendered = error.to_string();
    assert!(!rendered.contains("secrets"), "rendered: {rendered}");
    assert!(rendered.starts_with("PREFLIGHT_SOURCE_LOCATOR_INVALID"));
    assert_eq!(error.segment(), Some(2));
}

#[test]
fn the_locator_error_implementations_are_safe_to_render() {
    let error =
        SourceCandidatePathV1::parse("", SourceCandidateKind::Json).expect_err("empty is refused");
    assert_eq!(error.segment(), None);
    let as_error: &dyn std::error::Error = &error;
    assert!(as_error.to_string().contains("empty"));
}
