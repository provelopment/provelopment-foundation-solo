//! Bounded parsing and negative tests for the strict `1.0.0` Platform catalogue parser
//! (`SOLO-WO-118 R12`).
//!
//! Every document here is **synthetic test text** built in the test process. It is not product
//! artwork, not an installed `platform/assets/` library and not a rights assertion: the licence and
//! provenance strings are placeholders whose only purpose is to satisfy the closed shape.

use solo_adapters::{MAX_PLATFORM_CATALOGUE_BYTES_V1, parse_platform_asset_catalogue_v1};
use solo_application::{PLATFORM_ASSET_LIBRARY_VERSION_V1, PlatformCatalogueRefusalV1};

const SYNTHETIC_SHA_A: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const SYNTHETIC_SHA_B: &str = "2222222222222222222222222222222222222222222222222222222222222222";

fn licence() -> &'static str {
    r#"{"expression":"Synthetic-Test-Only-1.0","rightsHolder":"Synthetic placeholder","evidence":"synthetic/licence-notice.txt"}"#
}

fn provenance() -> &'static str {
    r#"{"source":"Synthetic bytes generated in a test","evidence":"synthetic/provenance-notice.txt"}"#
}

fn asset_body(id: &str, sha: &str, class: &str, mime: &str, width: &str, height: &str) -> String {
    format!(
        r#"{{"id":"{id}","sha256":"{sha}","mediaClass":"{class}","mime":"{mime}","width":{width},"height":{height},"licence":{},"provenance":{}}}"#,
        licence(),
        provenance()
    )
}

fn asset(id: &str, sha: &str, mime: &str) -> String {
    asset_body(id, sha, "image", mime, "2", "2")
}

fn document(version: &str, assets: &str) -> String {
    format!(r#"{{"platformAssetLibraryVersion":"{version}","assets":[{assets}]}}"#)
}

fn parse(
    text: &str,
) -> Result<solo_application::PlatformAssetCatalogueV1, PlatformCatalogueRefusalV1> {
    parse_platform_asset_catalogue_v1(text.as_bytes(), PLATFORM_ASSET_LIBRARY_VERSION_V1)
}

#[test]
fn a_synthetic_two_entry_catalogue_parses_in_order_without_authority() {
    let text = document(
        PLATFORM_ASSET_LIBRARY_VERSION_V1,
        &format!(
            "{},{}",
            asset("icons/one.png", SYNTHETIC_SHA_A, "image/png"),
            asset("media/two.jpg", SYNTHETIC_SHA_B, "image/jpeg")
        ),
    );
    let catalogue = parse(&text).expect("a well-formed synthetic catalogue");
    assert_eq!(catalogue.version(), PLATFORM_ASSET_LIBRARY_VERSION_V1);
    assert_eq!(catalogue.len(), 2);
    let ids: Vec<&str> = catalogue.entries().iter().map(|item| item.id()).collect();
    assert_eq!(ids, vec!["icons/one.png", "media/two.jpg"]);
    assert_eq!(catalogue.entries()[1].mime(), "image/jpeg");
    assert!(!catalogue.grants_publication_authority());
    assert!(!catalogue.proves_media_correctness());
    assert!(!catalogue.rights_reviewed());
    assert!(!catalogue.platform_source_readable());
    assert!(!catalogue.derived_output_verified());
    assert!(!catalogue.release_ready());
    for entry in catalogue.entries() {
        assert!(!entry.licence().rights_reviewed());
    }
}

#[test]
fn every_hostile_document_is_refused_with_a_closed_path_free_category() {
    for (label, text, want) in hostile_cases() {
        let outcome = parse(&text);
        assert_eq!(outcome.as_ref().err(), Some(&want), "case {label}");
        let rendered = format!("{:?}", outcome.as_ref().err());
        for forbidden in ["synthetic/", "Synthetic", "icons/", "placeholder"] {
            assert!(
                !rendered.contains(forbidden),
                "case {label} leaked {forbidden}"
            );
        }
    }
}

/// The hostile matrix: each document is synthetic and each refusal is a closed category.
fn hostile_cases() -> Vec<(&'static str, String, PlatformCatalogueRefusalV1)> {
    use PlatformCatalogueRefusalV1 as Refusal;
    let good = asset("icons/one.png", SYNTHETIC_SHA_A, "image/png");
    let version = PLATFORM_ASSET_LIBRARY_VERSION_V1;
    let mut cases: Vec<(&'static str, String, Refusal)> = vec![
        ("empty", String::new(), Refusal::InvalidSyntax),
        (
            "truncated",
            "{\"assets\":[".to_string(),
            Refusal::InvalidSyntax,
        ),
        (
            "missing-version",
            format!(r#"{{"assets":[{good}]}}"#),
            Refusal::InvalidSyntax,
        ),
        (
            "wrong-version",
            document("1.0.1", &good),
            Refusal::VersionMismatch,
        ),
        (
            "duplicate-root-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","platformAssetLibraryVersion":"1.0.0","assets":[{good}]}}"#
            ),
            Refusal::DuplicateField,
        ),
        (
            "unknown-root-key",
            format!(r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{good}],"extra":1}}"#),
            Refusal::UnknownField,
        ),
        (
            "duplicate-nested-licence-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"id":"icons/one.png","sha256":"{SYNTHETIC_SHA_A}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{{"expression":"A","expression":"B","rightsHolder":"C","evidence":"D"}},"provenance":{}}}]}}"#,
                provenance()
            ),
            Refusal::DuplicateField,
        ),
        (
            "unknown-nested-provenance-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"id":"icons/one.png","sha256":"{SYNTHETIC_SHA_A}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{},"provenance":{{"source":"S","evidence":"E","note":"x"}}}}]}}"#,
                licence()
            ),
            Refusal::UnknownField,
        ),
        (
            "duplicate-asset-key",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"id":"icons/one.png","id":"icons/two.png","sha256":"{SYNTHETIC_SHA_A}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{},"provenance":{}}}]}}"#,
                licence(),
                provenance()
            ),
            Refusal::DuplicateField,
        ),
        (
            "missing-id",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"sha256":"{SYNTHETIC_SHA_A}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{},"provenance":{}}}]}}"#,
                licence(),
                provenance()
            ),
            Refusal::InvalidSyntax,
        ),
        (
            "empty-assets",
            document(version, ""),
            Refusal::EmptyCatalogue,
        ),
        (
            "traversal-id",
            document(
                version,
                &asset("../escape.png", SYNTHETIC_SHA_A, "image/png"),
            ),
            Refusal::InvalidId,
        ),
        (
            "absolute-id",
            document(
                version,
                &asset("/abs/one.png", SYNTHETIC_SHA_A, "image/png"),
            ),
            Refusal::InvalidId,
        ),
        (
            "reserved-id",
            document(
                version,
                &asset("icons/con.png", SYNTHETIC_SHA_A, "image/png"),
            ),
            Refusal::InvalidId,
        ),
        (
            "uppercase-extension",
            document(
                version,
                &asset("icons/one.PNG", SYNTHETIC_SHA_A, "image/png"),
            ),
            Refusal::InvalidId,
        ),
        (
            "platform-prefix-id",
            document(
                version,
                &asset("platform/one.png", SYNTHETIC_SHA_A, "image/png"),
            ),
            Refusal::InvalidId,
        ),
    ];
    cases.extend([
        (
            "short-digest",
            document(version, &asset("icons/one.png", "abc", "image/png")),
            Refusal::InvalidDigest,
        ),
        (
            "uppercase-digest",
            document(
                version,
                &asset("icons/one.png", &"A".repeat(64), "image/png"),
            ),
            Refusal::InvalidDigest,
        ),
        (
            "zero-digest",
            document(
                version,
                &asset("icons/one.png", &"0".repeat(64), "image/png"),
            ),
            Refusal::InvalidDigest,
        ),
        (
            "unsupported-mime",
            document(
                version,
                &asset("icons/one.png", SYNTHETIC_SHA_A, "text/vtt"),
            ),
            Refusal::UnsupportedMediaType,
        ),
        (
            "mime-extension-pair",
            document(
                version,
                &asset("icons/one.png", SYNTHETIC_SHA_A, "image/jpeg"),
            ),
            Refusal::MimeExtensionMismatch,
        ),
        (
            "bad-media-class",
            document(
                version,
                &asset_body(
                    "icons/one.png",
                    SYNTHETIC_SHA_A,
                    "video",
                    "image/png",
                    "2",
                    "2",
                ),
            ),
            Refusal::InvalidMediaClass,
        ),
        (
            "zero-geometry",
            document(
                version,
                &asset_body(
                    "icons/one.png",
                    SYNTHETIC_SHA_A,
                    "image",
                    "image/png",
                    "0",
                    "2",
                ),
            ),
            Refusal::InvalidDimensions,
        ),
        (
            "fraction-geometry",
            document(
                version,
                &asset_body(
                    "icons/one.png",
                    SYNTHETIC_SHA_A,
                    "image",
                    "image/png",
                    "2.5",
                    "2",
                ),
            ),
            Refusal::InvalidSyntax,
        ),
    ]);
    cases.extend([
        (
            "nul-rights-text",
            format!(
                r#"{{"platformAssetLibraryVersion":"1.0.0","assets":[{{"id":"icons/one.png","sha256":"{SYNTHETIC_SHA_A}","mediaClass":"image","mime":"image/png","width":2,"height":2,"licence":{{"expression":"E","rightsHolder":"H\u0000","evidence":"V"}},"provenance":{}}}]}}"#,
                provenance()
            ),
            Refusal::InvalidRightsText,
        ),
        (
            "bom",
            format!("\u{FEFF}{}", document(version, &good)),
            Refusal::InvalidSyntax,
        ),
        (
            "trailing-data",
            format!("{}{}", document(version, &good), "{}"),
            Refusal::InvalidSyntax,
        ),
        (
            "duplicate-id",
            document(
                version,
                &format!(
                    "{},{}",
                    asset("icons/one.png", SYNTHETIC_SHA_A, "image/png"),
                    asset("icons/one.png", SYNTHETIC_SHA_B, "image/png")
                ),
            ),
            Refusal::DuplicateId,
        ),
        (
            "case-colliding-id",
            document(
                version,
                &format!(
                    "{},{}",
                    asset("icons/One.png", SYNTHETIC_SHA_A, "image/png"),
                    asset("icons/one.png", SYNTHETIC_SHA_B, "image/png")
                ),
            ),
            Refusal::CaseCollidingIdentifier,
        ),
        (
            "duplicate-digest",
            document(
                version,
                &format!(
                    "{},{}",
                    asset("icons/one.png", SYNTHETIC_SHA_A, "image/png"),
                    asset("icons/two.png", SYNTHETIC_SHA_A, "image/png")
                ),
            ),
            Refusal::DuplicateDigest,
        ),
    ]);
    cases
}

#[test]
fn the_document_and_bounds_are_enforced() {
    use PlatformCatalogueRefusalV1 as Refusal;
    let version = PLATFORM_ASSET_LIBRARY_VERSION_V1;
    // The trusted selected version is independent of the document it is checking.
    let text = document(
        version,
        &asset("icons/one.png", SYNTHETIC_SHA_A, "image/png"),
    );
    assert_eq!(
        parse_platform_asset_catalogue_v1(text.as_bytes(), "9.9.9").err(),
        Some(Refusal::VersionMismatch)
    );
    // An oversize body is refused before any parsing work.
    let oversize = vec![b' '; MAX_PLATFORM_CATALOGUE_BYTES_V1 + 1];
    assert_eq!(
        parse_platform_asset_catalogue_v1(&oversize, version).err(),
        Some(Refusal::InvalidSyntax)
    );
    // More than the accepted entry count is refused.
    let entry = asset("icons/one.png", SYNTHETIC_SHA_A, "image/png");
    let repeated = std::iter::repeat_n(entry, 1025)
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(
        parse(&document(version, &repeated)).err(),
        Some(Refusal::TooManyEntries)
    );
    // Non-UTF-8 input is refused.
    assert_eq!(
        parse_platform_asset_catalogue_v1(&[0xFF, 0xFE, 0x00], version).err(),
        Some(Refusal::InvalidSyntax)
    );
    // The documented matrix really is at least the required size.
    assert!(hostile_cases().len() >= 18);
}
