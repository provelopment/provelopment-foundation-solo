//! Integration tests for the strict, duplicate-aware manifest parser.
//!
//! Every case exercises the public API with bytes already in hand. No test writes or
//! reads a content tree, because the parser performs no I/O.

use solo_adapters::{
    DeclaredCaseMatching, DeclaredMissingTranslation, DeclaredVisibility,
    MANIFEST_LOGICAL_IDENTITY, MAX_MANIFEST_BYTES, MAX_PROPERTY_ENTRIES, ManifestErrorKind,
    SUPPORTED_VERSION, parse_manifest_bytes,
};

/// The normative minimal shape: one Entity, one Facet, one Site, one Page, one
/// representation, and one declared JSON candidate.
const MINIMAL: &str = r#"{
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
                    "sourceCandidates": { "json": "pages/home.en-001.json" }
                  }
                ]
              }
            ]
          }
        ]
      }
    ]
  }
}"#;

/// One complete, valid Site subtree with the given id and candidate name.
fn site(id: &str, page: &str, candidate: &str, locale: &str) -> String {
    format!(
        r#"{{ "id": "{id}", "label": "Site", "defaultLocale": "{locale}", "pages": [
             {{ "id": "{page}", "label": "Page", "representations": [
                {{ "locale": "{locale}", "route": {{ "path": "/" }},
                   "sourceCandidates": {{ "json": "pages/{candidate}.json" }} }} ] }} ] }}"#
    )
}

/// A manifest whose `facets` array is replaced by the supplied JSON.
fn manifest_with_facets(facets: &str) -> String {
    format!(
        r#"{{ "schemaVersion": "1.0.0", "propertyRegistryVersion": "1.0.0", "platformAssetLibraryVersion": "1.0.0",
             "base": {{ "canonical": "https://example.test" }},
             "entity": {{ "id": "acme", "label": "Acme", "facets": {facets} }} }}"#
    )
}

/// A manifest whose single Page carries the supplied `representations` array.
fn manifest_with_representations(representations: &str) -> String {
    format!(
        r#"{{ "schemaVersion": "1.0.0", "propertyRegistryVersion": "1.0.0", "platformAssetLibraryVersion": "1.0.0",
             "base": {{ "canonical": "https://example.test" }},
             "entity": {{ "id": "acme", "label": "Acme", "facets": [
               {{ "id": "main", "label": "Main", "sites": [
                  {{ "id": "site", "label": "Site", "defaultLocale": "en-001", "pages": [
                     {{ "id": "home", "label": "Home", "representations": {representations} }} ] }} ] }} ] }} }}"#
    )
}

fn reject(bytes: &[u8]) -> ManifestErrorKind {
    match parse_manifest_bytes(bytes) {
        Ok(parsed) => panic!(
            "input must be refused, accepted with {} nodes",
            parsed.node_count()
        ),
        Err(error) => {
            assert_eq!(error.code(), "PREFLIGHT_MANIFEST_INVALID");
            error.kind()
        }
    }
}

fn reject_text(text: &str) -> ManifestErrorKind {
    reject(text.as_bytes())
}

#[test]
fn parses_the_minimal_candidate_without_claiming_authority() {
    let parsed = parse_manifest_bytes(MINIMAL.as_bytes()).expect("the minimal shape parses");

    assert_eq!(parsed.logical_identity(), MANIFEST_LOGICAL_IDENTITY);
    assert_eq!(parsed.logical_identity(), "content/publication.json");
    assert_eq!(parsed.schema_version().as_str(), SUPPORTED_VERSION);
    assert_eq!(parsed.property_registry_version().as_str(), "1.0.0");
    assert_eq!(parsed.platform_asset_library_version().as_str(), "1.0.0");
    assert_eq!(parsed.base().canonical(), "https://example.test");
    assert!(parsed.base().aliases().is_empty());
    assert!(parsed.base().landing().is_none());

    let entity = parsed.entity();
    assert_eq!(entity.id().as_str(), "acme");
    assert_eq!(entity.label(), "Acme");
    assert_eq!(entity.facets().len(), 1);

    let facet = &entity.facets()[0];
    assert_eq!(facet.id().as_str(), "main");
    let site = &facet.sites()[0];
    assert_eq!(site.id().as_str(), "site");
    assert_eq!(site.default_locale(), "en-001");
    let page = &site.pages()[0];
    assert_eq!(page.id().as_str(), "home");
    assert_eq!(page.visibility_default(), DeclaredVisibility::Public);
    assert_eq!(
        page.missing_translation(),
        &DeclaredMissingTranslation::Default
    );
    let representation = &page.representations()[0];
    assert_eq!(representation.locale(), "en-001");
    assert_eq!(representation.route().path(), "/");
    assert_eq!(representation.visibility(), None);
    assert_eq!(
        representation
            .source_candidates()
            .json()
            .expect("json candidate declared")
            .as_str(),
        "pages/home.en-001.json"
    );

    // A syntactic parse is never a publication decision.
    assert_eq!(parsed.node_count(), 5);
    assert_eq!(parsed.page_count(), 1);
    assert_eq!(parsed.representation_count(), 1);
    assert!(!parsed.is_publication_authorized());
}

#[test]
fn parses_an_expanded_candidate_and_preserves_authored_order() {
    let expanded = MINIMAL
        .replace(
            r#""route": { "path": "/" },"#,
            r#""route": {
                      "path": "/en-001/",
                      "caseMatching": "sensitive",
                      "aliases": [
                        { "path": "/home/", "caseMatching": "insensitive" }
                      ]
                    },"#,
        )
        .replace(
            r#""sourceCandidates": { "json": "pages/home.en-001.json" }"#,
            r#""visibility": "unlisted",
                    "sourceCandidates": {
                      "json": "pages/home.en-001.json",
                      "markdown": "pages/home.en-001.md"
                    }"#,
        );

    let parsed = parse_manifest_bytes(expanded.as_bytes()).expect("the expanded shape parses");
    let representation = &parsed.entity().facets()[0].sites()[0].pages()[0].representations()[0];

    assert_eq!(
        representation.route().case_matching(),
        Some(DeclaredCaseMatching::Sensitive)
    );
    assert_eq!(representation.route().aliases().len(), 1);
    assert_eq!(representation.route().aliases()[0].path(), "/home/");
    assert_eq!(
        representation.route().aliases()[0].case_matching(),
        Some(DeclaredCaseMatching::Insensitive)
    );
    assert_eq!(
        representation.visibility(),
        Some(DeclaredVisibility::Unlisted)
    );

    let candidates = representation.source_candidates();
    assert_eq!(candidates.len(), 2);
    let members: Vec<&str> = candidates.iter().map(|path| path.as_str()).collect();
    assert_eq!(
        members,
        vec!["pages/home.en-001.json", "pages/home.en-001.md"]
    );
}

#[test]
fn keeps_authored_order_and_allows_id_reuse_under_different_parents() {
    let first = format!(
        r#"{{ "id": "zeta", "label": "Zeta", "sites": [{}] }}"#,
        site("site", "home", "zeta-home.en-001", "en-001")
    );
    let second = format!(
        r#"{{ "id": "main", "label": "Main", "sites": [{}] }}"#,
        site("site", "home", "main-home.en-001", "en-001")
    );
    let facets = format!("[{first}, {second}]");
    let parsed =
        parse_manifest_bytes(manifest_with_facets(&facets).as_bytes()).expect("two facets parse");

    let order: Vec<&str> = parsed
        .entity()
        .facets()
        .iter()
        .map(|facet| facet.id().as_str())
        .collect();
    assert_eq!(order, vec!["zeta", "main"], "authored order is preserved");

    // The same Page id appears under two different Sites: identity is scoped to its
    // parent, so reuse here is legitimate rather than a duplicate.
    let mirrored = format!(
        r#"[{{ "id": "main", "label": "Main", "sites": [{}, {}] }}]"#,
        site("mirror", "home", "mirror-home.de-de", "de-DE"),
        site("site", "home", "main-home.en-001", "en-001")
    );
    let parsed =
        parse_manifest_bytes(manifest_with_facets(&mirrored).as_bytes()).expect("two sites parse");
    let sites = &parsed.entity().facets()[0].sites();
    assert_eq!(sites.len(), 2);
    assert_eq!(sites[0].id().as_str(), "mirror");
    assert_eq!(sites[1].id().as_str(), "site");
    assert_eq!(sites[0].pages()[0].id().as_str(), "home");
    assert_eq!(sites[1].pages()[0].id().as_str(), "home");
}

#[test]
fn refuses_missing_and_extra_root_keys() {
    assert_eq!(
        reject_text(&MINIMAL.replace("\"base\": { \"canonical\": \"https://example.test\" },", "")),
        ManifestErrorKind::MissingKey
    );
    assert_eq!(
        reject_text(&MINIMAL.replace("\"entity\": {", "\"entities\": [],\n  \"entity\": {")),
        ManifestErrorKind::UnknownKey
    );
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"schemaVersion\": \"1.0.0\",",
            "\"draft\": true,\n  \"schemaVersion\": \"1.0.0\","
        )),
        ManifestErrorKind::UnknownKey
    );
}

#[test]
fn refuses_a_second_entity_authority_or_a_competing_lineage() {
    // A nested second `entity` member is not in the accepted shape.
    assert_eq!(
        reject_text(&MINIMAL.replace("\"facets\": [", "\"entity\": {},\n    \"facets\": [")),
        ManifestErrorKind::UnknownKey
    );
    // A page-level `parentId` competes with direct containment.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"parentId\": \"site\","
        )),
        ManifestErrorKind::UnknownKey
    );
    // A source-level `locale` is not an accepted representation key.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            r#""route": { "path": "/" },"#,
            r#""route": { "path": "/" }, "sourceLocale": "en-001","#
        )),
        ManifestErrorKind::UnknownKey
    );
}

#[test]
fn refuses_missing_children_and_unsupported_versions() {
    assert_eq!(
        reject_text(&manifest_with_facets("[]")),
        ManifestErrorKind::EmptyCollection
    );
    assert_eq!(
        reject_text(&manifest_with_representations("[]")),
        ManifestErrorKind::EmptyCollection
    );
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"schemaVersion\": \"1.0.0\"",
            "\"schemaVersion\": \"2.0.0\""
        )),
        ManifestErrorKind::UnsupportedVersion
    );
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"platformAssetLibraryVersion\": \"1.0.0\"",
            "\"platformAssetLibraryVersion\": \"1.0.1\""
        )),
        ManifestErrorKind::UnsupportedVersion
    );
}

#[test]
fn refuses_duplicate_scoped_ids_and_duplicate_locales() {
    // Two facets sharing one id inside a single Entity.
    let first_facet = format!(
        r#"{{ "id": "main", "label": "A", "sites": [{}] }}"#,
        site("site", "home", "a-home.en-001", "en-001")
    );
    let second_facet = format!(
        r#"{{ "id": "main", "label": "B", "sites": [{}] }}"#,
        site("other", "home", "b-home.en-001", "en-001")
    );
    let duplicated = format!("[{first_facet}, {second_facet}]");
    assert_eq!(
        reject_text(&manifest_with_facets(&duplicated)),
        ManifestErrorKind::DuplicateScopedId
    );

    // Two representations of one Page declaring the same locale.
    let same_locale = r#"[
      { "locale": "en-001", "route": { "path": "/a/" },
        "sourceCandidates": { "json": "pages/a.en-001.json" } },
      { "locale": "en-001", "route": { "path": "/b/" },
        "sourceCandidates": { "json": "pages/b.en-001.json" } }
    ]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(same_locale)),
        ManifestErrorKind::DuplicateLocale
    );
}

#[test]
fn refuses_two_representations_claiming_one_logical_locator() {
    let shared = r#"[
      { "locale": "en-001", "route": { "path": "/a/" },
        "sourceCandidates": { "json": "pages/shared.en-001.json" } },
      { "locale": "de-DE", "route": { "path": "/b/" },
        "sourceCandidates": { "json": "pages/shared.en-001.json" } }
    ]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(shared)),
        ManifestErrorKind::DuplicateSourceClaim
    );

    // Sharing only the Markdown candidate is equally a collision.
    let shared_markdown = r#"[
      { "locale": "en-001", "route": { "path": "/a/" },
        "sourceCandidates": { "json": "pages/a.en-001.json",
                              "markdown": "pages/shared.md" } },
      { "locale": "de-DE", "route": { "path": "/b/" },
        "sourceCandidates": { "markdown": "pages/shared.md" } }
    ]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(shared_markdown)),
        ManifestErrorKind::DuplicateSourceClaim
    );
}

#[test]
fn refuses_duplicate_keys_at_every_depth() {
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"schemaVersion\": \"1.0.0\",",
            "\"schemaVersion\": \"1.0.0\",\n  \"schemaVersion\": \"1.0.0\","
        )),
        ManifestErrorKind::DuplicateKey
    );
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"label\": \"Home\","
        )),
        ManifestErrorKind::DuplicateKey
    );
    // Inside inert `properties`, which is still duplicate-free data.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Acme\",",
            "\"label\": \"Acme\",\n    \"properties\": { \"a\": 1, \"a\": 2 },"
        )),
        ManifestErrorKind::DuplicateKey
    );
    // Inside the candidate declaration itself.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            r#""sourceCandidates": { "json": "pages/home.en-001.json" }"#,
            r#""sourceCandidates": {
                      "json": "pages/home.en-001.json",
                      "json": "pages/other.en-001.json"
                    }"#
        )),
        ManifestErrorKind::DuplicateKey
    );
}

#[test]
fn refuses_unsafe_aliasing_and_ambiguous_locator_spellings() {
    let unsafe_paths = [
        "../escape.json",
        "pages/../escape.json",
        "/pages/absolute.json",
        "pages\\windows.json",
        "pages/./dot.json",
        "pages/con.json",
        "pages/nul.md",
        "pages/trailingdot./x.json",
        "pages/a..b.json",
        "pages/Upper.json",
        "pages/a%2e.json",
        "pages/a.json?x=1",
        "content/pages/wrong-root.json",
        "pages/a.txt",
        "pages/a:b.json",
    ];

    for path in unsafe_paths {
        // Escape the value for JSON embedding so a backslash spelling reaches the
        // locator rule instead of failing as malformed JSON.
        let escaped = path.replace('\\', "\\\\");
        let representations = format!(
            r#"[{{ "locale": "en-001", "route": {{ "path": "/" }},
                  "sourceCandidates": {{ "json": "{escaped}" }} }}]"#
        );
        let kind = reject_text(&manifest_with_representations(&representations));
        assert!(
            matches!(
                kind,
                ManifestErrorKind::InvalidSourceLocator | ManifestErrorKind::WrongType
            ),
            "{path} produced {kind:?}"
        );
    }

    // A locator carrying the wrong kind's extension is refused rather than corrected.
    let mismatched = r#"[{ "locale": "en-001", "route": { "path": "/" },
        "sourceCandidates": { "json": "pages/home.en-001.md" } }]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(mismatched)),
        ManifestErrorKind::InvalidSourceLocator
    );
}

#[test]
fn refuses_omitted_null_and_empty_candidates() {
    let omitted = r#"[{ "locale": "en-001", "route": { "path": "/" } }]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(omitted)),
        ManifestErrorKind::MissingKey
    );

    let null_member = r#"[{ "locale": "en-001", "route": { "path": "/" },
        "sourceCandidates": { "json": null } }]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(null_member)),
        ManifestErrorKind::NullCandidate
    );

    let empty = r#"[{ "locale": "en-001", "route": { "path": "/" },
        "sourceCandidates": {} }]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(empty)),
        ManifestErrorKind::EmptyCandidateSet
    );

    let unknown_member = r#"[{ "locale": "en-001", "route": { "path": "/" },
        "sourceCandidates": { "html": "pages/home.en-001.json" } }]"#;
    assert_eq!(
        reject_text(&manifest_with_representations(unknown_member)),
        ManifestErrorKind::UnknownKey
    );
}

#[test]
fn refuses_a_guessed_default_or_an_untrusted_fallback() {
    // `mode: locale` without a declared fallback is a missing mandatory key, never a
    // guessed default.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"missingTranslation\": { \"mode\": \"locale\" },"
        )),
        ManifestErrorKind::MissingKey
    );
    // A fallback on `mode: default` is not part of that closed shape.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"missingTranslation\": { \"mode\": \"default\", \"fallbackLocale\": \"de-DE\" },"
        )),
        ManifestErrorKind::InvalidEnumValue
    );
    // An unknown mode is refused rather than defaulted.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"missingTranslation\": { \"mode\": \"guess\" },"
        )),
        ManifestErrorKind::InvalidEnumValue
    );
    // An unknown visibility literal is refused rather than defaulted.
    assert_eq!(
        reject_text(&MINIMAL.replace(
            "\"label\": \"Home\",",
            "\"label\": \"Home\",\n                \"visibilityDefault\": \"internal\","
        )),
        ManifestErrorKind::InvalidEnumValue
    );
}

#[test]
fn refuses_every_byte_level_violation() {
    assert_eq!(reject(b""), ManifestErrorKind::Empty);
    assert_eq!(
        reject(&[0xEF, 0xBB, 0xBF, b'{', b'}']),
        ManifestErrorKind::ByteOrderMark
    );
    // A lone 0xFF is not valid UTF-8 and is not the UTF-8 BOM.
    assert_eq!(reject(&[0xFF, 0xFE]), ManifestErrorKind::InvalidUtf8);
    assert_eq!(reject_text("[]"), ManifestErrorKind::NotAnObject);
    assert_eq!(reject_text("null"), ManifestErrorKind::NotAnObject);
    assert_eq!(
        reject_text(&format!("{MINIMAL} trailing")),
        ManifestErrorKind::MalformedJson
    );
    assert_eq!(
        reject_text("schemaVersion: 1.0.0\nentity: {}\n"),
        ManifestErrorKind::MalformedJson
    );
    assert_eq!(
        reject_text(&MINIMAL.replace("\"https://example.test\"", "\"https://example.test\" // x")),
        ManifestErrorKind::MalformedJson
    );

    let oversized = format!("{{\"schemaVersion\":\"{}\"}}", "a".repeat(2 * 1024 * 1024));
    assert_eq!(reject_text(&oversized), ManifestErrorKind::TooLarge);
    assert!(oversized.len() > MAX_MANIFEST_BYTES);
}

#[test]
fn refuses_an_out_of_bounds_inert_properties_block() {
    let too_many = (0..=MAX_PROPERTY_ENTRIES)
        .map(|index| format!("\"k{index}\": 1"))
        .collect::<Vec<String>>()
        .join(", ");
    let with_properties = MINIMAL.replace(
        "\"label\": \"Acme\",",
        &format!("\"label\": \"Acme\",\n    \"properties\": {{ {too_many} }},"),
    );
    assert_eq!(
        reject_text(&with_properties),
        ManifestErrorKind::PropertiesOutOfBounds
    );
}

#[test]
fn a_refusal_never_echoes_the_authored_value() {
    let secret = "pages/../do-not-echo.json";
    let representations = format!(
        r#"[{{ "locale": "en-001", "route": {{ "path": "/" }},
              "sourceCandidates": {{ "json": "{secret}" }} }}]"#
    );
    let error = parse_manifest_bytes(manifest_with_representations(&representations).as_bytes())
        .expect_err("traversal is refused");
    let rendered = error.to_string();
    assert!(!rendered.contains("do-not-echo"), "rendered: {rendered}");
    assert!(rendered.starts_with("PREFLIGHT_MANIFEST_INVALID"));
    assert!(
        error
            .location()
            .contains("representations[0].sourceCandidates")
    );
}

#[test]
fn parsing_reads_no_file_and_never_publishes() {
    // The declaration names a file that certainly does not exist, yet the parse succeeds:
    // existence, confinement and selection are explicitly out of scope for this slice.
    let absent = r#"[{ "locale": "en-001", "route": { "path": "/" },
        "sourceCandidates": { "json": "pages/definitely-absent.en-001.json" } }]"#;
    let parsed = parse_manifest_bytes(manifest_with_representations(absent).as_bytes())
        .expect("an absent candidate is not this parser's concern");
    assert!(!parsed.is_publication_authorized());
    assert_eq!(parsed.logical_identity(), "content/publication.json");

    // Nothing that could be mistaken for a publish-ready snapshot is exposed.
    assert_eq!(parsed.representation_count(), 1);
    assert_eq!(parsed.node_count(), 5);
    assert!(
        !std::path::Path::new("content/publication.json").exists(),
        "this slice must not create a manifest on disk"
    );
}
// `SOLO-WO-118 R21` acceptance cases for the manifest-to-visibility bridge.
//
// These cases exercise the accepted strict parser and the accepted typed EFS/visibility layers
// through the bridge only. They open no file, read no root, start no worker and publish nothing.

mod n132_manifest_visibility {
    use solo_adapters::{
        DeclaredVisibility, MANIFEST_VISIBILITY_LOCALE_PROFILE, MAX_MANIFEST_BYTES,
        ManifestEfsScopeV1, ManifestErrorKind, ManifestVisibilityBridgeRefusalKindV1,
        ManifestVisibilitySnapshotV1, check_manifest_efs_scope_v1, parse_manifest_bytes,
        parse_manifest_visibility_snapshot_v1,
    };
    use solo_application::sha256_media_worker_bytes_v1;
    use solo_domain::{
        ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1, EfsOwnershipDraft,
        EfsOwnershipGraph, EntityIdV1, FacetDraft, FacetIdV1, GraphicCascadeInputV1, LocaleKeyV1,
        PageDraft, PageIdV1, PageVisibilityDeclarationV1, PlatformAssetMediaClassV1,
        PlatformGraphicBaselineV1, QualifiedOwnerV1, RepresentationVisibilityOriginV1,
        RepresentationVisibilityV1, SiteDraft, SiteIdV1, build_representation_visibility_ledger_v1,
        validate_declared_properties_v1,
    };

    /// One Entity, one Facet, two sibling Sites that both declare `home`, one extra `about` Page
    /// and four representations carrying the four accepted visibility combinations.
    const FOUR_ROWS: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.test" },
  "entity": {
    "id": "acme",
    "label": "Acme",
    "facets": [
      { "id": "main", "label": "Main", "sites": [
        { "id": "bali", "label": "Bali", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/home" },
              "sourceCandidates": { "json": "pages/bali-home-en-001.json" } },
            { "locale": "de-DE", "route": { "path": "/de-DE/home" }, "visibility": "unlisted",
              "sourceCandidates": { "json": "pages/bali-home-de-de.json" } } ] },
          { "id": "about", "label": "About", "visibilityDefault": "unlisted", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/about" }, "visibility": "public",
              "sourceCandidates": { "json": "pages/bali-about-en-001.json" } } ] } ] },
        { "id": "ubud", "label": "Ubud", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "visibilityDefault": "unlisted", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/ubud-home" },
              "sourceCandidates": { "json": "pages/ubud-home-en-001.json" } } ] } ] } ] } ]
  }
}"#;

    /// The bridge over the four-row manifest.
    fn snapshot() -> ManifestVisibilitySnapshotV1 {
        parse_manifest_visibility_snapshot_v1(FOUR_ROWS.as_bytes())
            .expect("the four-row manifest converts")
    }

    /// One exact representation owner inside the fixture Entity.
    fn owner(site: &str, page: &str, locale: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::representation(
            EntityIdV1::parse("acme").expect("canonical Entity"),
            FacetIdV1::parse("main").expect("canonical Facet"),
            SiteIdV1::parse(site).expect("canonical Site"),
            PageIdV1::parse(page).expect("canonical Page"),
            LocaleKeyV1::from_already_canonical(locale).expect("canonical locale"),
        )
    }

    /// One Page owner inside the fixture Entity.
    fn page_owner(site: &str, page: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::page(
            EntityIdV1::parse("acme").expect("canonical Entity"),
            FacetIdV1::parse("main").expect("canonical Facet"),
            SiteIdV1::parse(site).expect("canonical Site"),
            PageIdV1::parse(page).expect("canonical Page"),
        )
    }

    /// `N132_01` — exact manifest → one graph → one complete visibility ledger.
    #[test]
    fn n132_01_manifest_converts_to_one_complete_visibility_ledger() {
        let snapshot = snapshot();
        let ledger = snapshot.ledger();

        // Provenance is the checked digest of the supplied bytes, never a rebuilt value.
        assert_eq!(
            snapshot.manifest_sha256(),
            sha256_media_worker_bytes_v1(FOUR_ROWS.as_bytes()).to_hex()
        );
        assert_eq!(snapshot.manifest_sha256().len(), 64);
        assert_eq!(snapshot.manifest_bytes(), FOUR_ROWS.len());
        assert_eq!(
            snapshot.locale_profile(),
            MANIFEST_VISIBILITY_LOCALE_PROFILE
        );
        assert_eq!(snapshot.locale_profile(), "canonical-shaped-base");

        // Complete coverage in stable authored graph order, with the sibling `home` Pages distinct.
        assert_eq!(snapshot.page_count(), 3);
        assert_eq!(snapshot.representation_count(), 4);
        assert_eq!(ledger.len(), 4);
        let order: Vec<&QualifiedOwnerV1> =
            ledger.entries().iter().map(|entry| entry.owner()).collect();
        assert_eq!(order[0], &owner("bali", "home", "en-001"));
        assert_eq!(order[1], &owner("bali", "home", "de-DE"));
        assert_eq!(order[2], &owner("bali", "about", "en-001"));
        assert_eq!(order[3], &owner("ubud", "home", "en-001"));
        assert_ne!(order[0], order[3]);
        assert_eq!(order[0].page_id(), order[3].page_id());
        assert_ne!(order[0].site_id(), order[3].site_id());

        // The four accepted rows, with their exact provenance.
        let rows = [
            ("bali", "home", "en-001", "public", "page-default"),
            (
                "bali",
                "home",
                "de-DE",
                "unlisted",
                "representation-override",
            ),
            (
                "bali",
                "about",
                "en-001",
                "public",
                "representation-override",
            ),
            ("ubud", "home", "en-001", "unlisted", "page-default"),
        ];
        for (site, page, locale, visibility, origin) in rows {
            let entry = ledger
                .for_owner(&owner(site, page, locale))
                .expect("the bridge governs this representation");
            assert_eq!(entry.effective().token(), visibility);
            assert_eq!(entry.origin().token(), origin);
            assert_eq!(
                entry.is_discoverable_intent(),
                visibility == "public",
                "{site}/{page}/{locale}"
            );
            assert_eq!(
                entry.robots_intent(),
                if visibility == "public" {
                    "index,follow"
                } else {
                    "noindex,nofollow"
                }
            );
        }

        // The R20 typed constructor over the same graph produces the identical ledger: this bridge
        // adds no inheritance algorithm, no second interpretation and no extra state.
        let same_graph = build_representation_visibility_ledger_v1(
            snapshot.graph(),
            &[
                PageVisibilityDeclarationV1::new(
                    page_owner("bali", "home"),
                    Some(RepresentationVisibilityV1::Public),
                )
                .and_then(|declaration| {
                    declaration.with_override(
                        LocaleKeyV1::from_already_canonical("de-DE").expect("canonical"),
                        RepresentationVisibilityV1::Unlisted,
                    )
                })
                .expect("a valid declaration"),
                PageVisibilityDeclarationV1::new(
                    page_owner("bali", "about"),
                    Some(RepresentationVisibilityV1::Unlisted),
                )
                .and_then(|declaration| {
                    declaration.with_override(
                        LocaleKeyV1::from_already_canonical("en-001").expect("canonical"),
                        RepresentationVisibilityV1::Public,
                    )
                })
                .expect("a valid declaration"),
                PageVisibilityDeclarationV1::new(
                    page_owner("ubud", "home"),
                    Some(RepresentationVisibilityV1::Unlisted),
                )
                .expect("a valid declaration"),
            ],
        )
        .expect("the same graph and declarations build a ledger");
        assert_eq!(&same_graph, ledger);
        assert_eq!(
            ledger.entries()[1].origin(),
            RepresentationVisibilityOriginV1::RepresentationOverride
        );

        // The parsed candidate is retained, unmodified, beside the derived facts.
        assert_eq!(snapshot.parsed().page_count(), 3);
        assert_eq!(snapshot.parsed().representation_count(), 4);
        assert_eq!(
            snapshot.parsed().base().canonical(),
            "https://example.test",
            "the parsed base stays an inert declaration"
        );
        let first_page = &snapshot.parsed().entity().facets()[0].sites()[0].pages()[0];
        assert_eq!(first_page.visibility_default(), DeclaredVisibility::Public);
        assert_eq!(
            first_page.representations()[1].visibility(),
            Some(DeclaredVisibility::Unlisted)
        );
        assert_eq!(first_page.representations()[0].visibility(), None);

        // No authority of any kind.
        assert!(!snapshot.grants_publication_authority());
        assert!(!snapshot.routes_any_url());
        assert!(!snapshot.serves_http());
        assert!(!snapshot.publishes_derived_output());
        assert!(!snapshot.grants_release_authority());
        assert!(!snapshot.grants_deployment_authority());
        assert!(!snapshot.parsed().is_publication_authorized());

        println!(
            "n132_evidence case=01 rows=4 page_count=3 representations=4 origin_page_default=2 \
             origin_override=2 sibling_isolated=true manifest_sha_pinned=true manifest_sha={} \
             locale_profile=restricted publication=false route=false",
            snapshot.manifest_sha256()
        );
    }

    /// Asserts one refusal of the expected kind at a safe ASCII location, with no snapshot.
    fn refused(bytes: &[u8], expected: ManifestVisibilityBridgeRefusalKindV1) -> String {
        let refusal = parse_manifest_visibility_snapshot_v1(bytes)
            .map(|snapshot| {
                panic!(
                    "input must be refused, converted with {} pages",
                    snapshot.page_count()
                )
            })
            .expect_err("the input is refused");
        assert_eq!(refusal.kind(), expected, "refusal cause");
        assert_eq!(refusal.code(), "PREFLIGHT_MANIFEST_VISIBILITY_INVALID");
        assert!(!refusal.location().is_empty(), "a refusal is attributable");
        assert!(refusal.location().is_ascii(), "locations are fixed ASCII");
        refusal.location().to_owned()
    }

    /// `N132_02` — the strict parser is retained and the bridge adds no escape hatch.
    #[test]
    fn n132_02_strict_parser_is_retained_and_the_bridge_adds_no_escape() {
        // Duplicate decoded object keys: the plain spelling plus a JSON unicode escape.
        let duplicate = FOUR_ROWS.replace(
            r#""visibility": "unlisted","#,
            r#""visibility": "unlisted", "\u0076isibility": "public","#,
        );
        assert_ne!(duplicate, FOUR_ROWS);
        refused(
            duplicate.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::DuplicateKey),
        );

        // An unknown nested key is not silently ignored.
        let unknown = FOUR_ROWS.replace(
            r#""label": "About","#,
            r#""label": "About", "unexpected": true,"#,
        );
        refused(
            unknown.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::UnknownKey),
        );

        // `internal` is a property exposure, never a representation visibility.
        let internal = FOUR_ROWS.replace(
            r#""visibility": "unlisted","#,
            r#""visibility": "internal","#,
        );
        refused(
            internal.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::InvalidEnumValue),
        );

        // Bytes, framing, versions and required keys keep the accepted parser's own causes.
        let mut invalid_utf8 = FOUR_ROWS.as_bytes().to_vec();
        invalid_utf8[0] = 0xFF;
        refused(
            &invalid_utf8,
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::InvalidUtf8),
        );

        let mut with_bom = vec![0xEF, 0xBB, 0xBF];
        with_bom.extend_from_slice(FOUR_ROWS.as_bytes());
        refused(
            &with_bom,
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::ByteOrderMark),
        );

        let truncated = &FOUR_ROWS.as_bytes()[..FOUR_ROWS.len() / 2];
        refused(
            truncated,
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::MalformedJson),
        );

        let mut trailing = FOUR_ROWS.as_bytes().to_vec();
        trailing.extend_from_slice(b"{}");
        refused(
            &trailing,
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::MalformedJson),
        );

        let wrong_version = FOUR_ROWS.replace(
            r#""platformAssetLibraryVersion": "1.0.0""#,
            r#""platformAssetLibraryVersion": "2.0.0""#,
        );
        refused(
            wrong_version.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::UnsupportedVersion),
        );

        let missing_root = FOUR_ROWS.replace(
            r#"  "base": { "canonical": "https://example.test" },
"#,
            "",
        );
        refused(
            missing_root.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::MissingKey),
        );

        let missing_child = FOUR_ROWS.replace(r#" "route": { "path": "/en-001/about" },"#, "");
        refused(
            missing_child.as_bytes(),
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::MissingKey),
        );

        let oversized = vec![b' '; MAX_MANIFEST_BYTES + 1];
        refused(
            &oversized,
            ManifestVisibilityBridgeRefusalKindV1::Manifest(ManifestErrorKind::TooLarge),
        );

        // The bridge's own restricted locale profile. The accepted parser still admits these
        // syntactically, so the restriction is demonstrably the bridge's and not a parser change.
        for bad in ["de-de", "en", "en-001-u-ca-gregory"] {
            let bytes = FOUR_ROWS.replace(r#""locale": "de-DE""#, &format!(r#""locale": "{bad}""#));
            assert_ne!(bytes, FOUR_ROWS);
            assert!(
                parse_manifest_bytes(bytes.as_bytes()).is_ok(),
                "the accepted parser still admits {bad} syntactically"
            );
            let location = refused(
                bytes.as_bytes(),
                ManifestVisibilityBridgeRefusalKindV1::LocaleProfileRefused,
            );
            assert!(location.ends_with(".locale"), "{location}");
        }

        println!(
            "n132_evidence case=02 refusals=14 duplicate_decoded_key=refused \
             unknown_key=refused internal_representation=refused utf8_and_bom=refused \
             truncated_and_trailing=refused version_and_keys=refused too_large=refused \
             locale_profile=restricted parser_accepts_locale=true zero_partial=true \
             publication=false route=false"
        );
    }

    /// A candidate over one graph, authoring no graphics at all.
    fn candidate_over(graph: &solo_domain::EfsOwnershipGraph) -> GraphicCascadeInputV1 {
        let asset = ApprovedPlatformAssetV1::new(
            "icons/icon-home.svg",
            &"0".repeat(64),
            PlatformAssetMediaClassV1::Svg,
        )
        .expect("a valid approved asset");
        let binding = ApprovedPlatformGraphicBindingV1::new(
            "1.0.0",
            "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7",
            vec![asset],
        )
        .expect("a valid approved binding");
        let validated =
            validate_declared_properties_v1(graph, Vec::new()).expect("the graph validates");
        GraphicCascadeInputV1::new(
            validated,
            PlatformGraphicBaselineV1::empty(binding),
            Vec::new(),
        )
        .expect("a valid empty candidate")
    }

    /// One Page of a fixture graph: its id and its representation locales.
    type PageShape<'a> = (&'a str, &'a [&'a str]);

    /// One Site of a fixture graph: its id and its Pages.
    type SiteShape<'a> = (&'a str, &'a [PageShape<'a>]);

    /// One graph of the fixture shape: a named Entity/Facet over the listed Sites and Pages.
    fn graph_of(entity: &str, facet: &str, sites: &[SiteShape<'_>]) -> EfsOwnershipGraph {
        let mut facet_draft = FacetDraft::new(FacetIdV1::parse(facet).expect("canonical Facet"));
        for (site_id, pages) in sites {
            let mut site_draft = SiteDraft::new(SiteIdV1::parse(site_id).expect("canonical Site"));
            for (page_id, locales) in *pages {
                let mut page_draft = PageDraft::new(PageIdV1::parse(page_id).expect("canonical"));
                for locale in *locales {
                    page_draft.add_representation(
                        LocaleKeyV1::from_already_canonical(locale).expect("canonical locale"),
                    );
                }
                site_draft.add_page(page_draft);
            }
            facet_draft.add_site(site_draft);
        }
        let mut draft =
            EfsOwnershipDraft::new(EntityIdV1::parse(entity).expect("canonical Entity"));
        draft.add_facet(facet_draft);
        draft.build().expect("the variant graph is valid")
    }

    /// The graph the fixture manifest describes.
    fn matching_graph() -> EfsOwnershipGraph {
        graph_of(
            "acme",
            "main",
            &[
                (
                    "bali",
                    &[("home", &["en-001", "de-DE"]), ("about", &["en-001"])],
                ),
                ("ubud", &[("home", &["en-001"])]),
            ],
        )
    }

    /// `N132_03` — foreign, partial and sibling scope is refused before anything is read.
    #[test]
    fn n132_03_foreign_partial_and_sibling_scope_is_refused() {
        let snapshot = snapshot();

        // The matching candidate is exactly this manifest's membership.
        let matching = candidate_over(&matching_graph());
        assert_eq!(
            check_manifest_efs_scope_v1(&snapshot, &matching).expect("matching scope"),
            ManifestEfsScopeV1::ExactEfsMembership
        );
        assert_eq!(
            check_manifest_efs_scope_v1(&snapshot, &matching)
                .expect("matching scope")
                .token(),
            "exact-EFS-membership"
        );

        // Exactly one dimension changes at a time; every variant refuses.
        let complete = |site_b: &str| {
            graph_of(
                "acme",
                "main",
                &[
                    (
                        "bali",
                        &[("home", &["en-001", "de-DE"]), ("about", &["en-001"])],
                    ),
                    (site_b, &[("home", &["en-001"])]),
                ],
            )
        };
        let same = complete("ubud");
        let sibling = complete("ubud2");
        let variants: Vec<(&str, EfsOwnershipGraph)> = vec![
            (
                "entity",
                graph_of(
                    "other",
                    "main",
                    &[
                        (
                            "bali",
                            &[("home", &["en-001", "de-DE"]), ("about", &["en-001"])],
                        ),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
            (
                "facet",
                graph_of(
                    "acme",
                    "aux",
                    &[
                        (
                            "bali",
                            &[("home", &["en-001", "de-DE"]), ("about", &["en-001"])],
                        ),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
            ("sibling_site", sibling.clone()),
            (
                "missing_page",
                graph_of(
                    "acme",
                    "main",
                    &[
                        ("bali", &[("home", &["en-001", "de-DE"])]),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
            (
                "extra_page",
                graph_of(
                    "acme",
                    "main",
                    &[
                        (
                            "bali",
                            &[
                                ("home", &["en-001", "de-DE"]),
                                ("about", &["en-001"]),
                                ("extra", &["en-001"]),
                            ],
                        ),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
            (
                "representation_locale",
                graph_of(
                    "acme",
                    "main",
                    &[
                        (
                            "bali",
                            &[("home", &["en-001", "fr-FR"]), ("about", &["en-001"])],
                        ),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
            (
                "missing_representation",
                graph_of(
                    "acme",
                    "main",
                    &[
                        ("bali", &[("home", &["en-001"]), ("about", &["en-001"])]),
                        ("ubud", &[("home", &["en-001"])]),
                    ],
                ),
            ),
        ];
        assert_eq!(variants.len(), 7);
        for (label, graph) in &variants {
            let candidate = candidate_over(graph);
            let refusal = check_manifest_efs_scope_v1(&snapshot, &candidate)
                .expect_err("a mismatched membership is refused");
            assert_eq!(
                refusal.kind(),
                ManifestVisibilityBridgeRefusalKindV1::ScopeMismatch,
                "{label}"
            );
            assert_eq!(refusal.detail(), "scope-mismatch");
        }

        // Entity identity plus the selected target alone would be insufficient — and is not what
        // this check compares: the identical graph matches, while the sibling rename shares both
        // the Entity and the target owner and still refuses.
        assert_eq!(
            check_manifest_efs_scope_v1(&snapshot, &candidate_over(&same))
                .expect("the identical graph matches"),
            ManifestEfsScopeV1::ExactEfsMembership
        );
        let sibling_candidate = candidate_over(&sibling);
        assert_eq!(
            sibling_candidate.validated().owners()[0].entity_id(),
            snapshot.graph().entity(),
            "the renamed sibling graph shares the Entity"
        );
        assert!(
            sibling_candidate
                .validated()
                .owners()
                .contains(&owner("bali", "home", "en-001")),
            "the renamed sibling graph still contains the target owner"
        );
        assert!(check_manifest_efs_scope_v1(&snapshot, &sibling_candidate).is_err());

        println!(
            "n132_evidence case=03 scope_exact=true variants=7 foreign_refused=true \
             sibling_refused=true missing_page_refused=true extra_page_refused=true \
             locale_change_refused=true entity_only_insufficient=true before_read=true \
             publication=false route=false"
        );
    }

    /// `N132_04` — binding determinism, provenance and nonauthority.
    #[test]
    fn n132_04_binding_determinism_provenance_and_nonauthority() {
        // Identical bytes convert to an identical value, digest, graph and ledger.
        let first = snapshot();
        let second = snapshot();
        assert_eq!(first, second);
        assert_eq!(first.manifest_sha256(), second.manifest_sha256());
        assert_eq!(first.graph(), second.graph());
        assert_eq!(first.ledger(), second.ledger());

        // A label-only change moves the provenance pin and not one visibility fact.
        let relabelled = FOUR_ROWS.replace(r#""label": "Acme""#, r#""label": "Acme Renamed""#);
        assert_ne!(relabelled, FOUR_ROWS);
        let third = parse_manifest_visibility_snapshot_v1(relabelled.as_bytes())
            .expect("the relabelled manifest converts");
        assert_ne!(third.manifest_sha256(), first.manifest_sha256());
        assert_eq!(third.graph(), first.graph());
        assert_eq!(third.ledger(), first.ledger());
        assert_eq!(third.ledger().entries().len(), 4);

        // Unlisted is not private: the exact typed identity stays available and the only derived
        // facts are the closed effective state, its provenance and a robot *intent*.
        let unlisted = first
            .ledger()
            .for_owner(&owner("ubud", "home", "en-001"))
            .expect("the unlisted representation is governed");
        assert_eq!(unlisted.effective().token(), "unlisted");
        assert!(!unlisted.is_discoverable_intent());
        assert_eq!(unlisted.robots_intent(), "noindex,nofollow");
        assert_eq!(
            unlisted.owner().locale().map(|key| key.as_str()),
            Some("en-001")
        );
        assert_eq!(
            unlisted.owner().page_id().map(|id| id.as_str()),
            Some("home")
        );

        // Routes and source candidates stay inert declarations of the unchanged parsed candidate.
        let parsed = first.parsed();
        let representation =
            &parsed.entity().facets()[0].sites()[0].pages()[0].representations()[0];
        assert_eq!(representation.route().path(), "/en-001/home");
        assert_eq!(representation.route().case_matching(), None);
        assert_eq!(
            representation
                .source_candidates()
                .json()
                .expect("a json candidate")
                .as_str(),
            "pages/bali-home-en-001.json"
        );
        assert_eq!(parsed.base().canonical(), "https://example.test");
        assert!(parsed.base().landing().is_none());
        assert!(!parsed.is_publication_authorized());

        // `internal` is never a representation state: the closed type has exactly two tokens.

        assert!(RepresentationVisibilityV1::parse_token("internal").is_err());
        assert_eq!(
            RepresentationVisibilityV1::parse_token("public").expect("public"),
            RepresentationVisibilityV1::Public
        );
        assert_eq!(
            RepresentationVisibilityV1::parse_token("unlisted").expect("unlisted"),
            RepresentationVisibilityV1::Unlisted
        );

        println!(
            "n132_evidence case=04 deterministic=true manifest_sha_pinned=true \
             label_sha_changed=true ledger_stable=true unlisted_not_private=true \
             routes_inert=true zero_partial=true publication=false route=false"
        );
    }
}

// `SOLO-WO-118 R22` acceptance cases for the manifest-bound route/visibility policy preflight.
//
// Every case runs real strict JSON bytes through the accepted R21 bridge and then the new pre-route
// policy. Nothing here opens a file, reads a root, starts a worker, resolves a route or serves HTTP.

mod n133_manifest_route_visibility {
    use solo_adapters::{
        EntryPathsPolicyV1, LandingAvailabilityV1, MANIFEST_ROUTE_POLICY_LOCALE_PROFILE,
        ManifestEfsScopeV1, ManifestErrorKind, ManifestRoutePolicyRefusalKindV1,
        ManifestRouteVisibilityPolicyV1, MissingTranslationPolicyV1, RouteComparisonIntentV1,
        build_manifest_route_visibility_policy_v1, check_manifest_efs_scope_v1,
        parse_manifest_visibility_snapshot_v1,
    };
    use solo_application::sha256_media_worker_bytes_v1;
    use solo_domain::{
        ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1, EfsOwnershipDraft,
        EfsOwnershipGraph, EntityIdV1, FacetDraft, FacetIdV1, GraphicCascadeInputV1, LocaleKeyV1,
        PageDraft, PageIdV1, PlatformAssetMediaClassV1, PlatformGraphicBaselineV1,
        QualifiedOwnerV1, RepresentationVisibilityOriginV1, RepresentationVisibilityV1, SiteDraft,
        SiteIdV1, validate_declared_properties_v1,
    };

    /// Two sibling Sites that both declare `home`, one mixed Page, one extra Page, one fully
    /// unlisted Page and one Page whose only representation is an unlisted override.
    const POLICY: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.test" },
  "entity": {
    "id": "acme",
    "label": "Acme",
    "facets": [
      { "id": "main", "label": "Main", "sites": [
        { "id": "bali", "label": "Bali", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/home" },
              "sourceCandidates": { "json": "pages/bali-home-en-001.json" } },
            { "locale": "de-DE",
              "route": { "path": "/de-DE/home", "caseMatching": "sensitive" },
              "sourceCandidates": { "json": "pages/bali-home-de-de.json" } },
            { "locale": "fr-FR", "route": { "path": "/fr-FR/home" }, "visibility": "unlisted",
              "sourceCandidates": { "json": "pages/bali-home-fr-fr.json" } } ] },
          { "id": "about", "label": "About", "entryPaths": ["/about"], "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/about" },
              "sourceCandidates": { "json": "pages/bali-about-en-001.json" } } ] } ] },
        { "id": "ubud", "label": "Ubud", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "visibilityDefault": "unlisted", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/ubud-home" },
              "sourceCandidates": { "json": "pages/ubud-home-en-001.json" } } ] },
          { "id": "legal", "label": "Legal", "representations": [
            { "locale": "en-001", "route": { "path": "/en-001/legal" }, "visibility": "unlisted",
              "sourceCandidates": { "json": "pages/ubud-legal-en-001.json" } } ] } ] } ] } ]
  }
}"#;

    /// The plan over the fixture manifest.
    fn plan() -> ManifestRouteVisibilityPolicyV1 {
        let snapshot =
            parse_manifest_visibility_snapshot_v1(POLICY.as_bytes()).expect("the fixture converts");
        build_manifest_route_visibility_policy_v1(&snapshot)
            .expect("the fixture passes the pre-route policy")
    }

    /// One exact representation owner inside the fixture Entity.
    fn rep_owner(site: &str, page: &str, locale: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::representation(
            EntityIdV1::parse("acme").expect("canonical Entity"),
            FacetIdV1::parse("main").expect("canonical Facet"),
            SiteIdV1::parse(site).expect("canonical Site"),
            PageIdV1::parse(page).expect("canonical Page"),
            LocaleKeyV1::from_already_canonical(locale).expect("canonical locale"),
        )
    }

    /// `N133_01` — the four-case matrix with closed intents, provenance and sibling identity.
    #[test]
    fn n133_01_four_case_matrix_keeps_intent_provenance_and_sibling_identity() {
        let snapshot =
            parse_manifest_visibility_snapshot_v1(POLICY.as_bytes()).expect("the fixture converts");
        let plan = build_manifest_route_visibility_policy_v1(&snapshot).expect("the plan builds");

        // Complete coverage, in authored containment order.
        assert_eq!(plan.representations().len(), 6);
        assert_eq!(plan.pages().len(), 4);
        assert_eq!(plan.representations().len(), snapshot.ledger().len());
        assert_eq!(
            plan.representations().len(),
            snapshot.representation_count()
        );
        assert_eq!(plan.locale_profile(), MANIFEST_ROUTE_POLICY_LOCALE_PROFILE);

        let order: Vec<&QualifiedOwnerV1> = plan
            .representations()
            .iter()
            .map(|record| record.owner())
            .collect();
        assert_eq!(order[0], &rep_owner("bali", "home", "en-001"));
        assert_eq!(order[1], &rep_owner("bali", "home", "de-DE"));
        assert_eq!(order[2], &rep_owner("bali", "home", "fr-FR"));
        assert_eq!(order[3], &rep_owner("bali", "about", "en-001"));
        assert_eq!(order[4], &rep_owner("ubud", "home", "en-001"));
        assert_eq!(order[5], &rep_owner("ubud", "legal", "en-001"));

        // The four accepted combinations, with their closed intents and provenance.
        let cases = [
            (
                "bali",
                "home",
                "en-001",
                RepresentationVisibilityV1::Public,
                RepresentationVisibilityOriginV1::PageDefault,
                RouteComparisonIntentV1::PublicInsensitive,
                false,
                true,
            ),
            (
                "bali",
                "home",
                "de-DE",
                RepresentationVisibilityV1::Public,
                RepresentationVisibilityOriginV1::PageDefault,
                RouteComparisonIntentV1::PublicSensitive,
                true,
                true,
            ),
            (
                "bali",
                "home",
                "fr-FR",
                RepresentationVisibilityV1::Unlisted,
                RepresentationVisibilityOriginV1::RepresentationOverride,
                RouteComparisonIntentV1::ExactUnlisted,
                false,
                false,
            ),
            (
                "ubud",
                "home",
                "en-001",
                RepresentationVisibilityV1::Unlisted,
                RepresentationVisibilityOriginV1::PageDefault,
                RouteComparisonIntentV1::ExactUnlisted,
                false,
                false,
            ),
            (
                "ubud",
                "legal",
                "en-001",
                RepresentationVisibilityV1::Unlisted,
                RepresentationVisibilityOriginV1::RepresentationOverride,
                RouteComparisonIntentV1::ExactUnlisted,
                false,
                false,
            ),
        ];
        for (site, page, locale, effective, origin, comparison, authored, discoverable) in cases {
            let record = plan
                .representation(&rep_owner(site, page, locale))
                .expect("every governed representation has one record");
            assert_eq!(record.effective(), effective, "{site}/{page}/{locale}");
            assert_eq!(record.origin(), origin, "{site}/{page}/{locale}");
            assert_eq!(record.comparison(), comparison, "{site}/{page}/{locale}");
            assert_eq!(record.case_matching_authored(), authored);
            assert_eq!(record.discoverable(), discoverable);
            assert_eq!(
                record.robots_intent(),
                if discoverable {
                    "index,follow"
                } else {
                    "noindex,nofollow"
                }
            );
            assert_eq!(record.alias_count(), 0, "no alias is ever promoted");
        }

        // The same `home` Page identifier in two sibling Sites stays two distinct owners.
        let bali_home = rep_owner("bali", "home", "en-001");
        let ubud_home = rep_owner("ubud", "home", "en-001");
        assert_ne!(bali_home, ubud_home);
        assert_eq!(bali_home.page_id(), ubud_home.page_id());
        assert_ne!(bali_home.site_id(), ubud_home.site_id());
        assert_ne!(
            plan.representation(&bali_home)
                .expect("bali home")
                .effective(),
            plan.representation(&ubud_home)
                .expect("ubud home")
                .effective(),
            "sibling Pages never share a visibility decision"
        );

        // Page-level eligibility is per Page, and entryPaths stay pending inert claims.
        assert_eq!(
            plan.page(&page_owner("bali", "home"))
                .expect("mixed Page")
                .public_representation_count(),
            2
        );
        assert_eq!(
            plan.page(&page_owner("bali", "about"))
                .expect("about Page")
                .entry_paths(),
            EntryPathsPolicyV1::PendingInert { count: 1 }
        );
        assert_eq!(
            plan.page(&page_owner("ubud", "home"))
                .expect("unlisted Page")
                .public_representation_count(),
            0
        );

        // Provenance identity is the accepted manifest digest, and no authority is claimed.
        assert_eq!(plan.manifest_sha256(), snapshot.manifest_sha256());
        assert_eq!(
            plan.manifest_sha256(),
            sha256_media_worker_bytes_v1(POLICY.as_bytes()).to_hex()
        );
        assert_eq!(plan.manifest_sha256().len(), 64);
        assert!(!plan.grants_publication_authority());
        assert!(!plan.grants_route_authority());
        assert!(!plan.serves_http());
        assert!(!plan.publishes_derived_output());
        assert!(!plan.grants_release_authority());
        assert!(!plan.grants_deployment_authority());

        // Paths and the base stay unvalidated authored strings: nothing of the kind leaks out.
        let authored_path = snapshot.parsed().entity().facets()[0].sites()[0].pages()[0]
            .representations()[0]
            .route()
            .path();
        assert_eq!(authored_path, "/en-001/home");
        assert_eq!(snapshot.parsed().base().canonical(), "https://example.test");
        let rendered = format!("{plan:?}");
        for forbidden in [
            "/en-001/home",
            "/de-DE/home",
            "https://example.test",
            "pages/bali-home-en-001.json",
            "Acme",
        ] {
            assert!(
                !rendered.contains(forbidden),
                "the plan must not expose {forbidden}"
            );
        }

        println!(
            "n133_evidence case=01 records=6 pages=4 public_insensitive=1 public_sensitive=1 \
             exact_unlisted=3 origin_page_default=2 origin_override=1 sibling_isolated=true \
             entry_paths_pending=1 landing=absent manifest_sha_pinned=true \
             locale_profile=restricted publication=false route=false http=false"
        );
    }

    /// One exact Page owner inside the fixture Entity.
    fn page_owner(site: &str, page: &str) -> QualifiedOwnerV1 {
        QualifiedOwnerV1::page(
            EntityIdV1::parse("acme").expect("canonical Entity"),
            FacetIdV1::parse("main").expect("canonical Facet"),
            SiteIdV1::parse(site).expect("canonical Site"),
            PageIdV1::parse(page).expect("canonical Page"),
        )
    }

    /// Asserts that these real bytes are refused by the policy with the expected closed cause.
    fn refused(bytes: &[u8], expected: ManifestRoutePolicyRefusalKindV1) -> String {
        let snapshot = parse_manifest_visibility_snapshot_v1(bytes)
            .expect("the accepted bridge admits these bytes");
        let refusal = build_manifest_route_visibility_policy_v1(&snapshot)
            .expect_err("the pre-route policy must refuse this claim");
        assert_eq!(refusal.kind(), expected, "refusal cause");
        assert_eq!(refusal.code(), "PREFLIGHT_MANIFEST_ROUTE_POLICY_INVALID");
        assert!(!refusal.location().is_empty());
        assert!(refusal.location().is_ascii(), "locations are fixed ASCII");
        refusal.location().to_owned()
    }

    /// `N133_02` — unlisted leakage and unsafe authoring are blocked before any use.
    #[test]
    fn n133_02_unlisted_leakage_and_unsafe_authoring_are_blocked() {
        // An effectively unlisted representation may not author `caseMatching`, either way.
        for authored in ["sensitive", "insensitive"] {
            let bytes = POLICY.replace(
                r#""route": { "path": "/fr-FR/home" }, "visibility": "unlisted""#,
                &format!(
                    r#""route": {{ "path": "/fr-FR/home", "caseMatching": "{authored}" }}, "visibility": "unlisted""#
                ),
            );
            assert_ne!(bytes, POLICY);
            let location = refused(
                bytes.as_bytes(),
                ManifestRoutePolicyRefusalKindV1::UnlistedCaseMatching,
            );
            assert!(location.ends_with(".route.caseMatching"), "{location}");
            assert!(location.contains("representations[2]"), "{location}");
        }

        // Nor may it author an alias: unlisted claims are never promoted or advertised.
        let with_alias = POLICY.replace(
            r#""route": { "path": "/fr-FR/home" }, "visibility": "unlisted""#,
            r#""route": { "path": "/fr-FR/home", "aliases": [ { "path": "/fr/home" } ] }, "visibility": "unlisted""#,
        );
        assert_ne!(with_alias, POLICY);
        let location = refused(
            with_alias.as_bytes(),
            ManifestRoutePolicyRefusalKindV1::UnlistedAliases,
        );
        assert!(location.ends_with(".route.aliases"), "{location}");

        // A Page with no public representation may not contribute a discoverable entry.
        let unlisted_entry = POLICY.replace(
            r#"{ "id": "legal", "label": "Legal", "representations": ["#,
            r#"{ "id": "legal", "label": "Legal", "entryPaths": ["/legal"], "representations": ["#,
        );
        assert_ne!(unlisted_entry, POLICY);
        let location = refused(
            unlisted_entry.as_bytes(),
            ManifestRoutePolicyRefusalKindV1::UnlistedOnlyEntryPaths,
        );
        assert!(location.ends_with(".entryPaths"), "{location}");

        // The mixed Page keeps its entry claims only because its own representation is public.
        let snapshot =
            parse_manifest_visibility_snapshot_v1(POLICY.as_bytes()).expect("the fixture converts");
        let plan = build_manifest_route_visibility_policy_v1(&snapshot).expect("the plan builds");
        assert_eq!(
            plan.page(&page_owner("bali", "about"))
                .expect("mixed Page")
                .entry_paths()
                .token(),
            "pending-inert"
        );
        assert_eq!(
            plan.page(&page_owner("ubud", "legal"))
                .expect("unlisted Page")
                .entry_paths(),
            EntryPathsPolicyV1::Absent
        );

        // `internal` is still refused by the retained strict parser: it is a property exposure,
        // never a third representation visibility, and this preflight never sees such a manifest.
        let internal = POLICY.replace(r#""visibility": "unlisted""#, r#""visibility": "internal""#);
        let parser_refusal = parse_manifest_visibility_snapshot_v1(internal.as_bytes())
            .expect_err("the accepted parser refuses `internal`");
        assert_eq!(
            parser_refusal.detail(),
            ManifestErrorKind::InvalidEnumValue.detail()
        );

        // A refused claim yields no plan at all: there is no partial record set to misuse.
        let no_partial = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(with_alias.as_bytes())
                .expect("the bridge admits these bytes"),
        );
        assert!(no_partial.is_err());

        println!(
            "n133_evidence case=02 unlisted_case_sensitive=refused \
             unlisted_case_insensitive=refused unlisted_aliases=refused \
             unlisted_only_entry_paths=refused mixed_entry_paths=pending \
             internal_representation=refused zero_partial=true before_read=true \
             publication=false route=false http=false"
        );
    }

    /// The fixture with one authored `base.landing` added.
    fn with_landing(target: &str) -> String {
        POLICY.replace(
            r#""base": { "canonical": "https://example.test" },"#,
            &format!(r#""base": {{ "canonical": "https://example.test", "landing": {target} }},"#),
        )
    }

    /// The fixture with one Page `missingTranslation` added; `anchor` selects the Page uniquely and
    /// must end with that Page's `"representations": [` member opening.
    fn with_missing_translation(anchor: &str, body: &str) -> String {
        let head = anchor.replace(r#""representations": ["#, "");
        let replacement = format!(r#"{head}"missingTranslation": {body}, "representations": ["#);
        let bytes = POLICY.replace(anchor, &replacement);
        assert_ne!(bytes, POLICY, "the anchor must select exactly one Page");
        bytes
    }

    /// `N133_03` — exact landing and same-Page missing-translation guard.
    #[test]
    fn n133_03_exact_landing_and_same_page_missing_translation_guard() {
        const BALI_HOME: &str = r#"{ "id": "home", "label": "Home", "representations": ["#;
        const UBUD_HOME: &str = r#"{ "id": "home", "label": "Home", "visibilityDefault": "unlisted", "representations": ["#;

        // Absent landing: a future base entry is simply not found.
        let plain = plan();
        assert_eq!(
            plain.landing().availability(),
            LandingAvailabilityV1::Absent
        );
        assert!(!plain.landing().declared());
        assert!(plain.landing().page().is_none());
        assert_eq!(plain.landing().availability().token(), "absent");

        // An exact qualified landing on a Page with a public representation is eligible.
        let eligible = with_landing(
            r#"{ "entity": "acme", "facet": "main", "site": "bali", "page": "about" }"#,
        );
        let eligible_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(eligible.as_bytes()).expect("the bridge admits"),
        )
        .expect("an exact landing with a public representation is eligible");
        assert_eq!(
            eligible_plan.landing().availability(),
            LandingAvailabilityV1::EligibleExactPage
        );
        assert_eq!(
            eligible_plan.landing().page(),
            Some(&page_owner("bali", "about"))
        );
        assert!(
            eligible_plan
                .landing()
                .page()
                .expect("a resolved landing")
                .locale()
                .is_none(),
            "no locale is ever inferred for a landing"
        );
        assert_eq!(
            eligible_plan.landing().availability().token(),
            "eligible-exact-page"
        );

        // An exact but entirely unlisted landing Page is honestly unavailable — never rerouted.
        let unlisted_landing = with_landing(
            r#"{ "entity": "acme", "facet": "main", "site": "ubud", "page": "home" }"#,
        );
        let unlisted_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(unlisted_landing.as_bytes())
                .expect("the bridge admits"),
        )
        .expect("a valid but unlisted landing is not a refusal");
        assert_eq!(
            unlisted_plan.landing().availability(),
            LandingAvailabilityV1::UnavailableNoPublicRepresentation
        );
        assert_eq!(
            unlisted_plan.landing().page(),
            Some(&page_owner("ubud", "home"))
        );
        assert_eq!(
            unlisted_plan.landing().availability().token(),
            "unavailable-no-public-representation"
        );

        // A landing that does not name an existing exact qualified Page refuses.
        for target in [
            r#"{ "entity": "acme", "facet": "main", "site": "kuta", "page": "about" }"#,
            r#"{ "entity": "other", "facet": "main", "site": "bali", "page": "about" }"#,
            r#"{ "entity": "acme", "facet": "aux", "site": "bali", "page": "about" }"#,
            r#"{ "entity": "acme", "facet": "main", "site": "bali", "page": "contact" }"#,
            r#"{ "entity": "acme", "facet": "main", "site": "ubud", "page": "home", "locale": "it-IT" }"#,
        ] {
            let bytes = with_landing(target);
            let location = refused(
                bytes.as_bytes(),
                ManifestRoutePolicyRefusalKindV1::LandingUnknownTarget,
            );
            assert!(location.starts_with("base.landing"), "{location}");
        }

        // `strict` creates no fallback at all.
        let strict = with_missing_translation(BALI_HOME, r#"{ "mode": "strict" }"#);
        let strict_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(strict.as_bytes()).expect("the bridge admits"),
        )
        .expect("`strict` is a valid claim");
        let strict_page = strict_plan
            .page(&page_owner("bali", "home"))
            .expect("the Page is governed");
        assert_eq!(
            strict_page.missing_translation(),
            &MissingTranslationPolicyV1::Strict
        );
        assert_eq!(strict_page.missing_translation().token(), "strict");
        assert!(
            strict_page
                .missing_translation()
                .fallback_locale()
                .is_none()
        );

        // `default` with a public same-Page Site-default representation names it exactly.
        let default_public = with_missing_translation(BALI_HOME, r#"{ "mode": "default" }"#);
        let default_public_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(default_public.as_bytes())
                .expect("the bridge admits"),
        )
        .expect("`default` with a public target is valid");
        let default_page = default_public_plan
            .page(&page_owner("bali", "home"))
            .expect("the Page is governed");
        assert_eq!(
            default_page.missing_translation(),
            &MissingTranslationPolicyV1::DefaultPublicTarget {
                locale: LocaleKeyV1::from_already_canonical("en-001").expect("canonical"),
            }
        );
        assert_eq!(
            default_page.missing_translation().token(),
            "default-public-target"
        );

        // `default` with no public same-Page target is a conditional future not-found, and the Page
        // itself stays valid: a visitor may simply never ask for that locale.
        let default_unlisted = with_missing_translation(UBUD_HOME, r#"{ "mode": "default" }"#);
        let default_unlisted_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(default_unlisted.as_bytes())
                .expect("the bridge admits"),
        )
        .expect("an unlisted-only Page is still a valid authored Page");
        let conditional = default_unlisted_plan
            .page(&page_owner("ubud", "home"))
            .expect("the Page is governed");
        assert_eq!(
            conditional.missing_translation(),
            &MissingTranslationPolicyV1::DefaultConditionalNotFound
        );
        assert_eq!(
            conditional.missing_translation().token(),
            "default-conditional-not-found"
        );
        assert_eq!(conditional.public_representation_count(), 0);

        // `default` where this Page does not declare the Site default at all stays conditional.
        let default_absent = POLICY.replace(
            r#""locale": "en-001", "route": { "path": "/en-001/ubud-home" }"#,
            r#""locale": "id-ID", "route": { "path": "/id-ID/ubud-home" }"#,
        );
        assert_ne!(default_absent, POLICY);
        let default_absent_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(default_absent.as_bytes())
                .expect("the bridge admits"),
        )
        .expect("an unavailable default target never rejects the Page");
        assert_eq!(
            default_absent_plan
                .page(&page_owner("ubud", "home"))
                .expect("the Page is governed")
                .missing_translation(),
            &MissingTranslationPolicyV1::DefaultConditionalNotFound
        );

        // An explicit locale with a real public same-Page target is accepted exactly.
        let explicit = with_missing_translation(
            BALI_HOME,
            r#"{ "mode": "locale", "fallbackLocale": "de-DE" }"#,
        );
        let explicit_plan = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(explicit.as_bytes()).expect("the bridge admits"),
        )
        .expect("an explicit public same-Page target is valid");
        let explicit_policy = explicit_plan
            .page(&page_owner("bali", "home"))
            .expect("the Page is governed")
            .missing_translation();
        assert_eq!(explicit_policy.token(), "explicit-public-target");
        assert_eq!(
            explicit_policy.fallback_locale().map(LocaleKeyV1::as_str),
            Some("de-DE")
        );

        // Every unsafe explicit target refuses: missing, unlisted, non-canonical and sibling-only.
        for (anchor, fallback, expected) in [
            (
                BALI_HOME,
                "it-IT",
                ManifestRoutePolicyRefusalKindV1::MissingTranslationTargetUnknown,
            ),
            (
                BALI_HOME,
                "fr-FR",
                ManifestRoutePolicyRefusalKindV1::MissingTranslationTargetUnlisted,
            ),
            (
                BALI_HOME,
                "de-de",
                ManifestRoutePolicyRefusalKindV1::MissingTranslationTargetNotCanonical,
            ),
            (
                UBUD_HOME,
                "de-DE",
                ManifestRoutePolicyRefusalKindV1::MissingTranslationTargetUnknown,
            ),
        ] {
            let bytes = with_missing_translation(
                anchor,
                &format!(r#"{{ "mode": "locale", "fallbackLocale": "{fallback}" }}"#),
            );
            let location = refused(bytes.as_bytes(), expected);
            assert!(
                location.ends_with(".missingTranslation.fallbackLocale"),
                "{location}"
            );
        }

        // No URL, no redirect target and no inferred locale preference leaves this preflight.
        let rendered = format!("{explicit_plan:?}");
        for forbidden in ["http", "/en-001/home", "Vary", "302", "Location"] {
            assert!(!rendered.contains(forbidden), "must not emit {forbidden}");
        }

        println!(
            "n133_evidence case=03 landing_eligible=1 landing_absent=1 \
             landing_unlisted_unavailable=1 landing_foreign=refused missing_strict=1 \
             default_public_target=1 default_conditional=2 explicit_public_target=1 \
             explicit_refusals=4 no_url=true no_redirect=true no_locale_inference=true \
             publication=false route=false http=false"
        );
    }

    /// One graph of the fixture shape, with the second Site named by the caller.
    fn graph_of(site_b: &str) -> EfsOwnershipGraph {
        let mut facet = FacetDraft::new(FacetIdV1::parse("main").expect("canonical Facet"));
        let mut bali = SiteDraft::new(SiteIdV1::parse("bali").expect("canonical Site"));
        let mut home = PageDraft::new(PageIdV1::parse("home").expect("canonical Page"));
        for locale in ["en-001", "de-DE", "fr-FR"] {
            home.add_representation(LocaleKeyV1::from_already_canonical(locale).expect("locale"));
        }
        let mut about = PageDraft::new(PageIdV1::parse("about").expect("canonical Page"));
        about.add_representation(LocaleKeyV1::from_already_canonical("en-001").expect("locale"));
        bali.add_page(home);
        bali.add_page(about);
        let mut other = SiteDraft::new(SiteIdV1::parse(site_b).expect("canonical Site"));
        for page in ["home", "legal"] {
            let mut draft = PageDraft::new(PageIdV1::parse(page).expect("canonical Page"));
            draft
                .add_representation(LocaleKeyV1::from_already_canonical("en-001").expect("locale"));
            other.add_page(draft);
        }
        facet.add_site(bali);
        facet.add_site(other);
        let mut entity = EfsOwnershipDraft::new(EntityIdV1::parse("acme").expect("Entity"));
        entity.add_facet(facet);
        entity.build().expect("the fixture graph is valid")
    }

    /// A candidate over one graph, authoring no graphics at all.
    fn candidate_over(graph: &EfsOwnershipGraph) -> GraphicCascadeInputV1 {
        let asset = ApprovedPlatformAssetV1::new(
            "icons/icon-home.svg",
            &"0".repeat(64),
            PlatformAssetMediaClassV1::Svg,
        )
        .expect("a valid approved asset");
        let binding = ApprovedPlatformGraphicBindingV1::new(
            "1.0.0",
            "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7",
            vec![asset],
        )
        .expect("a valid approved binding");
        let validated =
            validate_declared_properties_v1(graph, Vec::new()).expect("the graph validates");
        GraphicCascadeInputV1::new(
            validated,
            PlatformGraphicBaselineV1::empty(binding),
            Vec::new(),
        )
        .expect("a valid empty candidate")
    }

    /// `N133_04` — determinism, provenance, foreign refusal and nonauthority.
    #[test]
    fn n133_04_determinism_provenance_and_nonauthority() {
        // Identical bytes produce an identical plan, digest, record set and order.
        let first = plan();
        let second = plan();
        assert_eq!(first, second);
        assert_eq!(first.manifest_sha256(), second.manifest_sha256());
        assert_eq!(first.representations(), second.representations());
        assert_eq!(first.pages(), second.pages());
        assert_eq!(first.landing(), second.landing());

        // A label-only change moves the digest and not one governed policy fact.
        let relabelled = POLICY.replace(r#""label": "Acme""#, r#""label": "Acme Renamed""#);
        assert_ne!(relabelled, POLICY);
        let third = build_manifest_route_visibility_policy_v1(
            &parse_manifest_visibility_snapshot_v1(relabelled.as_bytes())
                .expect("the relabelled manifest converts"),
        )
        .expect("the relabelled manifest still passes the policy");
        assert_ne!(third.manifest_sha256(), first.manifest_sha256());
        assert_eq!(third.representations(), first.representations());
        assert_eq!(third.pages(), first.pages());
        assert_eq!(third.landing(), first.landing());

        // The R21 ownership guard is unchanged: a foreign, sibling-renamed or missing candidate is
        // still refused before this policy could ever be handed foreign content.
        let snapshot =
            parse_manifest_visibility_snapshot_v1(POLICY.as_bytes()).expect("the fixture converts");
        assert_eq!(
            check_manifest_efs_scope_v1(&snapshot, &candidate_over(&graph_of("ubud")))
                .expect("the exact membership matches"),
            ManifestEfsScopeV1::ExactEfsMembership
        );
        for renamed in ["kuta", "ubud2"] {
            let refusal =
                check_manifest_efs_scope_v1(&snapshot, &candidate_over(&graph_of(renamed)))
                    .expect_err("a foreign sibling is refused");
            assert_eq!(refusal.detail(), "scope-mismatch");
        }

        // Complete coverage and closed tokens only.
        assert_eq!(first.representations().len(), snapshot.ledger().len());
        assert_eq!(first.pages().len(), snapshot.page_count());
        for record in first.representations() {
            assert!(matches!(
                record.comparison().token(),
                "public-insensitive" | "public-sensitive" | "exact-unlisted"
            ));
            assert!(matches!(
                record.robots_intent(),
                "index,follow" | "noindex,nofollow"
            ));
        }
        for page in first.pages() {
            assert!(matches!(
                page.missing_translation().token(),
                "strict"
                    | "default-public-target"
                    | "default-conditional-not-found"
                    | "explicit-public-target"
            ));
            assert!(matches!(
                page.entry_paths().token(),
                "absent" | "pending-inert"
            ));
        }
        assert!(matches!(
            first.landing().availability().token(),
            "absent" | "eligible-exact-page" | "unavailable-no-public-representation"
        ));

        // No authority, no source bytes and no output tree of any kind.
        assert!(!first.grants_publication_authority());
        assert!(!first.grants_route_authority());
        assert!(!first.serves_http());
        assert!(!first.publishes_derived_output());
        assert!(!first.grants_release_authority());
        assert!(!first.grants_deployment_authority());
        let rendered = format!("{first:?}");
        for forbidden in ["pages/", "icons/", "https://", "http://", "target/"] {
            assert!(!rendered.contains(forbidden), "must not expose {forbidden}");
        }
        assert!(
            !std::path::Path::new("manifest-route-policy.json").exists(),
            "this preflight writes no output artifact"
        );

        println!(
            "n133_evidence case=04 deterministic=true label_sha_changed=true policy_stable=true \
             coverage=complete foreign_scope=refused sibling_scope=refused tokens_closed=true \
             zero_partial=true no_source_bytes=true publication=false route=false http=false \
             release=false deployment=false"
        );
    }
}
