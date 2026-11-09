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
