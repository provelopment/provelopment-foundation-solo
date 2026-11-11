//! `R03A-09` adapter-boundary tests over real strict JSON bytes.
//!
//! Each test starts from authored manifest bytes, runs the unchanged strict
//! [`parse_manifest_bytes`] parser, builds the accepted one-Entity graph, then drives the new
//! translation and the application use case to a validated public projection. The parser's
//! inert return type is asserted explicitly so no test can imply that parsing became
//! publication authority.

use std::fs;
use std::path::{Path, PathBuf};

use solo_adapters::{
    ManifestPropertyRefusalKind, ParsedManifestCandidateV1, parse_manifest_bytes,
    validate_manifest_property_candidate_v1,
};
use solo_domain::{
    EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1, FacetDraft, FacetIdV1, LocaleKeyV1,
    PageDraft, PageIdV1, PropertyKey, PropertyRefusalKind, PropertyResolutionRefusalKind,
    PropertyValueV1, QualifiedOwnerV1, SiteDraft, SiteIdV1,
};

/// A manifest with no authored properties at any level.
const NO_PROPERTIES: &str = "{}";

/// Builds the accepted five-level manifest skeleton with one authored `properties` object per
/// level, exactly as the committed starter seed authorises.
fn manifest(
    entity_properties: &str,
    facet_properties: &str,
    site_properties: &str,
    page_properties: &str,
    representation_properties: &str,
) -> String {
    format!(
        r#"{{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {{
    "canonical": "https://example.org/site",
    "landing": {{ "entity": "example", "facet": "main", "site": "main", "page": "home" }}
  }},
  "entity": {{
    "id": "example",
    "label": "Example",
    "properties": {entity_properties},
    "facets": [
      {{
        "id": "main",
        "label": "Main",
        "properties": {facet_properties},
        "sites": [
          {{
            "id": "main",
            "label": "Main site",
            "defaultLocale": "en-001",
            "languagePreferences": {{ "en": "en-001" }},
            "properties": {site_properties},
            "pages": [
              {{
                "id": "home",
                "label": "Home",
                "properties": {page_properties},
                "representations": [
                  {{
                    "locale": "en-001",
                    "route": {{ "path": "/en-001/home" }},
                    "sourceCandidates": {{ "markdown": "pages/main-home.en-001.md" }},
                    "properties": {representation_properties}
                  }}
                ]
              }}
            ]
          }}
        ]
      }}
    ]
  }}
}}"#
    )
}

fn entity() -> EntityIdV1 {
    EntityIdV1::parse("example").expect("a canonical Entity identifier")
}

fn facet() -> FacetIdV1 {
    FacetIdV1::parse("main").expect("a canonical Facet identifier")
}

fn site() -> SiteIdV1 {
    SiteIdV1::parse("main").expect("a canonical Site identifier")
}

fn page() -> PageIdV1 {
    PageIdV1::parse("home").expect("a canonical Page identifier")
}

fn locale(value: &str) -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical(value).expect("a canonical full locale key")
}

fn entity_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::entity(entity())
}

fn facet_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::facet(entity(), facet())
}

fn site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), site())
}

fn page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), site(), page())
}

fn representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), site(), page(), locale("en-001"))
}

/// The pure domain draft of the already parsed declarations.
fn draft_from(parsed: &ParsedManifestCandidateV1) -> EfsOwnershipDraft {
    let mut draft = EfsOwnershipDraft::new(parsed.entity().id().clone());
    for facet_declaration in parsed.entity().facets() {
        let mut facet_draft = FacetDraft::new(facet_declaration.id().clone());
        for site_declaration in facet_declaration.sites() {
            let mut site_draft = SiteDraft::new(site_declaration.id().clone());
            for page_declaration in site_declaration.pages() {
                let mut page_draft = PageDraft::new(page_declaration.id().clone());
                for representation in page_declaration.representations() {
                    page_draft.add_representation(
                        LocaleKeyV1::from_already_canonical(representation.locale())
                            .expect("the test manifests author canonical full locale keys"),
                    );
                }
                site_draft.add_page(page_draft);
            }
            facet_draft.add_site(site_draft);
        }
        draft.add_facet(facet_draft);
    }
    draft
}

fn graph_from(parsed: &ParsedManifestCandidateV1) -> EfsOwnershipGraph {
    draft_from(parsed)
        .build()
        .expect("the test manifests describe a valid one-Entity structure")
}

fn parse_document(bytes: &str) -> ParsedManifestCandidateV1 {
    parse_manifest_bytes(bytes.as_bytes()).expect("the test manifest is strict valid JSON")
}

fn starter_manifest_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/starter/content/publication.json")
}

/// `R03A-09` — real strict JSON through the unchanged parser, the new translation and the
/// application use case, proving all five owner levels end to end.
#[test]
fn r03a_09_real_manifest_bytes_resolve_all_five_levels() {
    let document = manifest(
        r#"{ "contact.email": { "inheritance": "inherit", "currentContext": [ { "address": "mail@example.org", "exposure": "public" } ] } }"#,
        r#"{ "graphic.background": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/facet.png" }, "exposure": "public" } ] } }"#,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Bali site", "exposure": "public" } ] } }"#,
        r#"{ "seo.description": { "inheritance": "inherit", "currentContext": [ { "text": "Everything about our work in Bali", "exposure": "public" } ] } }"#,
        r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "association", "fields": { "countryCode": "ID", "locality": "Denpasar" }, "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&document);
    assert!(!parsed.is_publication_authorized());
    let graph = graph_from(&parsed);
    let candidate =
        validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
            .expect("a valid whole-graph property candidate");

    assert_eq!(candidate.declared_owner_count(), 5);
    assert!(!candidate.is_publication_authorized());
    assert!(!candidate.has_recognized_location_extension());
    assert_eq!(candidate.target(), &representation_owner());

    // Each level contributes its own authored value with its own original owner.
    let email = candidate.resolved().values_of(PropertyKey::ContactEmail);
    assert_eq!(email.len(), 1);
    assert_eq!(email[0].origin_owner(), &entity_owner());
    assert_eq!(email[0].origin_local_index(), 0);
    match email[0].data() {
        PropertyValueV1::Mailbox(mailbox) => assert_eq!(mailbox.as_str(), "mail@example.org"),
        other => panic!("expected a mailbox value, found {other:?}"),
    }

    let background = candidate
        .resolved()
        .values_of(PropertyKey::GraphicBackground);
    assert_eq!(background.len(), 1);
    assert_eq!(background[0].origin_owner(), &facet_owner());
    match background[0].data() {
        PropertyValueV1::Graphic(reference) => {
            assert_eq!(reference.reference(), "images/facet.png");
            assert_eq!(reference.namespace(), solo_domain::AssetNamespace::Entity);
        }
        other => panic!("expected a graphic value, found {other:?}"),
    }

    let title = candidate.resolved().values_of(PropertyKey::SeoTitle);
    assert_eq!(title.len(), 1);
    assert_eq!(title[0].origin_owner(), &site_owner());
    match title[0].data() {
        PropertyValueV1::Text(text) => assert_eq!(text.as_str(), "Bali site"),
        other => panic!("expected a text value, found {other:?}"),
    }

    let description = candidate.resolved().values_of(PropertyKey::SeoDescription);
    assert_eq!(description.len(), 1);
    assert_eq!(description[0].origin_owner(), &page_owner());

    let facts = candidate.resolved().values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].origin_owner(), &representation_owner());
    let fact = facts[0]
        .data()
        .location_fact()
        .expect("a resolved location fact");
    assert_eq!(fact.fact_id().as_str(), "bali-base");
    assert_eq!(
        fact.fields().set_field_names(),
        vec!["countryCode", "locality"]
    );
    let origins = facts[0].field_origins().expect("field origins");
    assert_eq!(origins.len(), 2);
    assert_eq!(origins[0].1.owner(), &representation_owner());
    assert_eq!(origins[0].1.local_index(), 0);

    // The public projection carries the same values in order and no provenance.
    assert_eq!(
        candidate
            .public()
            .values_of(PropertyKey::ContactEmail)
            .len(),
        1
    );
    assert_eq!(candidate.public().values_of(PropertyKey::SeoTitle).len(), 1);
    assert!(!format!("{:?}", candidate.public()).contains("originLocalIndex"));
}

/// The refusal category of one Page-level declaration, for the envelope and entry matrix.
fn page_level_refusal_kind(properties: &str) -> ManifestPropertyRefusalKind {
    let document = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        properties,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("the authored declaration must refuse")
        .kind()
}

/// `R03A-09` — an unregistered property name is refused by category and qualified location
/// without echoing authored content.
#[test]
fn r03a_09_unknown_property_refuses_by_category() {
    let document = manifest(
        r#"{ "seo.keywords": { "inheritance": "inherit", "currentContext": "none" } }"#,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &entity_owner())
        .expect_err("an unregistered property refuses");
    assert_eq!(refusal.kind(), ManifestPropertyRefusalKind::UnknownProperty);
    assert_eq!(refusal.code(), "PROPERTY_UNKNOWN_KEY");
    assert_eq!(refusal.detail(), "unknown-property");
    assert_eq!(refusal.property(), None);
    assert!(refusal.to_string().contains("entity:example"));
    assert!(!refusal.to_string().contains("seo.keywords"));
}

/// `R03A-09` — a wrong-level declaration (Entity-level `seo.title`) refuses.
#[test]
fn r03a_09_wrong_level_declaration_refuses() {
    let document = manifest(
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Entity title", "exposure": "public" } ] } }"#,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &entity_owner())
        .expect_err("an Entity-level seo.title refuses");
    assert_eq!(refusal.code(), "PROPERTY_DECLARATION_REFUSED");
    assert_eq!(refusal.property(), Some(PropertyKey::SeoTitle));
    assert_eq!(refusal.owner(), Some(&entity_owner()));
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::WrongLevel)
    );
}

/// `R03A-01`/`R03A-09` — the closed declaration envelope refuses every malformed authored form
/// at the adapter boundary, and an explicit `"none"` is still valid.
#[test]
fn r03a_09_closed_envelope_forms_refuse() {
    fn envelope(body: &str) -> String {
        format!(r#"{{ "seo.title": {body} }}"#)
    }

    assert_eq!(
        page_level_refusal_kind(&envelope(
            r#"{ "inheritance": "inherit", "currentContext": null }"#
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidCurrentContext)
    );
    assert_eq!(
        page_level_refusal_kind(&envelope(
            r#"{ "inheritance": "inherit", "currentContext": [] }"#
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::EmptyContextArray)
    );
    assert_eq!(
        page_level_refusal_kind(&envelope(
            r#"{ "inheritance": "maybe", "currentContext": "none" }"#
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidInheritance)
    );
    assert_eq!(
        page_level_refusal_kind(&envelope(r#"{ "inheritance": "inherit" }"#)),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::MissingEnvelopeField)
    );
    assert_eq!(
        page_level_refusal_kind(&envelope(
            r#"{ "inheritance": "inherit", "currentContext": "none", "note": "x" }"#
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::ExtraEnvelopeField)
    );
    assert_eq!(
        page_level_refusal_kind(r#"{ "seo.title": null }"#),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidCurrentContext)
    );

    // A valid explicit `suppress` with the literal "none" is accepted.
    let document = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "seo.title": { "inheritance": "suppress", "currentContext": "none" } }"#,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let candidate =
        validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
            .expect("an explicit suppress with \"none\" is a valid declaration");
    assert!(
        candidate
            .resolved()
            .values_of(PropertyKey::SeoTitle)
            .is_empty()
    );
}

/// `R03A-06`/`R03A-09` — closed typed entries refuse malformed graphics, text, mailboxes and
/// location facts at the adapter boundary, and no value is repaired.
#[test]
fn r03a_09_closed_typed_entries_refuse() {
    let cases: [(&str, PropertyRefusalKind); 13] = [
        (
            r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Title", "exposure": "hidden" } ] } }"#,
            PropertyRefusalKind::InvalidExposure,
        ),
        (
            r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "", "exposure": "public" } ] } }"#,
            PropertyRefusalKind::EmptyPlainText,
        ),
        (
            r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "<b>bold</b>", "exposure": "public" } ] } }"#,
            PropertyRefusalKind::ActiveMarkup,
        ),
        (
            r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/logo.png" }, "exposure": "public", "url": "https://example.org/logo.png" } ] } }"#,
            PropertyRefusalKind::ExtraEntryField,
        ),
        (
            r#"{ "graphic.favicon": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/favicon.jpg" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::FaviconNotPng,
        ),
        (
            r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "../escape.png" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidAssetReference,
        ),
        (
            r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/logo.svg" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::UnsupportedAssetExtension,
        ),
        (
            r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "cdn", "reference": "images/logo.png" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidAssetNamespace,
        ),
        (
            r#"{ "contact.email": { "inheritance": "inherit", "currentContext": [ { "address": "not a mailbox", "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidMailbox,
        ),
        (
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "association", "fields": { "country": "ID" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::UnknownLocationField,
        ),
        (
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "association", "fields": { "coordinates": { "latitude": 91, "longitude": 0 } }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidCoordinates,
        ),
        (
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "Bali-Base", "kind": "association", "fields": { "locality": "Denpasar" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidFactId,
        ),
        (
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "residence", "fields": { "locality": "Denpasar" }, "exposure": "public" } ] } }"#,
            PropertyRefusalKind::InvalidLocationKind,
        ),
    ];
    for (properties, expected) in cases {
        assert_eq!(
            page_level_refusal_kind(properties),
            ManifestPropertyRefusalKind::DeclarationRefused(expected),
            "properties {properties} must be refused"
        );
    }

    // An empty field set and an unknown operation also refuse.
    assert_eq!(
        page_level_refusal_kind(
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "association", "fields": {}, "exposure": "public" } ] } }"#
        ),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::EmptyLocationFields)
    );
    assert_eq!(
        page_level_refusal_kind(
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "replace", "factId": "bali-base", "kind": "association", "fields": { "locality": "Denpasar" }, "exposure": "public" } ] } }"#
        ),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidOperation)
    );
}

/// `R03A-05`/`R03A-06`/`R03A-09` — effective cardinality and duplicate refusals reach the
/// adapter boundary as typed resolution refusals.
#[test]
fn r03a_09_effective_cardinality_and_duplicates_refuse() {
    let two_titles = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Site title", "exposure": "public" } ] } }"#,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Page title", "exposure": "public" } ] } }"#,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&two_titles);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("two effective titles refuse");
    assert_eq!(refusal.code(), "PROPERTY_RESOLUTION_REFUSED");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::CardinalityExceeded
        )
    );

    let duplicate_source = manifest(
        NO_PROPERTIES,
        r#"{ "graphic.background": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/shared.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.background": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "images/shared.png" }, "exposure": "public" } ] } }"#,
        NO_PROPERTIES,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&duplicate_source);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("a repeated background source refuses");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::DuplicateGraphicSource
        )
    );

    let duplicate_fact = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "bali-base", "kind": "association", "fields": { "locality": "Denpasar" }, "exposure": "public" }, { "operation": "add", "factId": "bali-base", "kind": "association", "fields": { "locality": "Ubud" }, "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&duplicate_fact);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("a repeated original location identity refuses");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::DuplicateLocationFact
        )
    );
}

/// `R03A-10`/`R03A-09` — the accepted `extend` syntax is recognized as a declaration and then
/// refused by its dedicated code, never merged and never treated as `add`.
#[test]
fn r03a_09_recognized_extend_is_refused_by_name() {
    let document = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main" }, "factId": "bali-base" }, "set": { "locality": "Ubud" } } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("R03.A refuses an extension at resolution");
    assert_eq!(refusal.code(), "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A
        )
    );
    assert!(
        !matches!(
            refusal.kind(),
            ManifestPropertyRefusalKind::DeclarationRefused(_)
        ),
        "the accepted extend syntax is recognized, not reported as invalid"
    );
}

/// A manifest whose Page also publishes an **unlisted** Indonesian representation.
fn manifest_with_unlisted_sibling(page_properties: &str, sibling_properties: &str) -> String {
    format!(
        r#"{{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {{
    "canonical": "https://example.org/site",
    "landing": {{ "entity": "example", "facet": "main", "site": "main", "page": "home", "locale": "en-001" }}
  }},
  "entity": {{
    "id": "example",
    "label": "Example",
    "facets": [
      {{
        "id": "main",
        "label": "Main",
        "sites": [
          {{
            "id": "main",
            "label": "Main site",
            "defaultLocale": "en-001",
            "languagePreferences": {{ "en": "en-001" }},
            "pages": [
              {{
                "id": "home",
                "label": "Home",
                "properties": {page_properties},
                "representations": [
                  {{
                    "locale": "en-001",
                    "route": {{ "path": "/en-001/home" }},
                    "sourceCandidates": {{ "markdown": "pages/main-home.en-001.md" }}
                  }},
                  {{
                    "locale": "id-ID",
                    "route": {{ "path": "/id-ID/home" }},
                    "sourceCandidates": {{ "markdown": "pages/main-home.id-id.md" }},
                    "visibility": "unlisted",
                    "properties": {sibling_properties}
                  }}
                ]
              }}
            ]
          }}
        ]
      }}
    ]
  }}
}}"#
    )
}

/// `R03A-09`/`R03A-11` — one invalid declaration on an **unlisted** representation fails the
/// whole candidate, while a valid unlisted declaration stays isolated to that representation.
#[test]
fn r03a_09_unlisted_sibling_declaration_fails_closed() {
    let invalid = manifest_with_unlisted_sibling(
        NO_PROPERTIES,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "<b>not plain</b>", "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&invalid);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("an invalid unlisted declaration fails the whole candidate");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::ActiveMarkup)
    );
    let message = refusal.to_string();
    assert!(message.contains("representation:id-ID"));
    assert!(message.contains("seo.title"));
    assert!(!message.contains("not plain"));

    let valid = manifest_with_unlisted_sibling(
        NO_PROPERTIES,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Beranda", "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&valid);
    let graph = graph_from(&parsed);
    let english = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect("a valid unlisted declaration does not block the public target");
    assert!(
        english
            .resolved()
            .values_of(PropertyKey::SeoTitle)
            .is_empty()
    );

    let indonesian =
        QualifiedOwnerV1::representation(entity(), facet(), site(), page(), locale("id-ID"));
    let unlisted_candidate = validate_manifest_property_candidate_v1(&parsed, &graph, &indonesian)
        .expect("the unlisted representation resolves its own properties");
    assert_eq!(
        unlisted_candidate
            .resolved()
            .values_of(PropertyKey::SeoTitle)
            .len(),
        1
    );
    assert_eq!(
        unlisted_candidate
            .public()
            .values_of(PropertyKey::SeoTitle)
            .len(),
        1
    );
}

/// `R03A-09`/`R03A-12` — the committed starter seed still parses through the unchanged inert
/// parser and yields a valid, empty property candidate.
#[test]
fn r03a_09_committed_starter_seed_stays_inert() {
    let bytes =
        fs::read(starter_manifest_path()).expect("the committed starter manifest is readable");
    let parsed = parse_manifest_bytes(&bytes).expect("the committed starter manifest parses");
    assert_eq!(parsed.logical_identity(), "content/publication.json");
    assert!(!parsed.is_publication_authorized());
    assert!(parsed.node_count() >= 4);

    let graph = graph_from(&parsed);
    let candidate =
        validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
            .expect("a manifest without properties still yields a valid empty candidate");
    assert_eq!(candidate.declared_owner_count(), 0);
    assert_eq!(candidate.resolved().properties().len(), 9);
    assert!(
        candidate
            .resolved()
            .values_of(PropertyKey::Location)
            .is_empty()
    );
    assert!(
        candidate
            .public()
            .values_of(PropertyKey::GraphicLogo)
            .is_empty()
    );
}

/// A manifest with two Sites under one Facet, each publishing a Page `home` in `en-001`.
fn manifest_two_sites(
    first_site_properties: &str,
    first_representation_properties: &str,
    second_site_properties: &str,
    second_representation_properties: &str,
) -> String {
    format!(
        r#"{{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {{
    "canonical": "https://example.org/site",
    "landing": {{ "entity": "example", "facet": "main", "site": "main", "page": "home", "locale": "en-001" }}
  }},
  "entity": {{
    "id": "example",
    "label": "Example",
    "facets": [
      {{
        "id": "main",
        "label": "Main",
        "sites": [
          {{
            "id": "main",
            "label": "Main site",
            "defaultLocale": "en-001",
            "languagePreferences": {{ "en": "en-001" }},
            "properties": {first_site_properties},
            "pages": [
              {{
                "id": "home",
                "label": "Home",
                "representations": [
                  {{
                    "locale": "en-001",
                    "route": {{ "path": "/en-001/home" }},
                    "sourceCandidates": {{ "markdown": "pages/main-home.en-001.md" }},
                    "properties": {first_representation_properties}
                  }}
                ]
              }}
            ]
          }},
          {{
            "id": "second",
            "label": "Second site",
            "defaultLocale": "en-001",
            "languagePreferences": {{ "en": "en-001" }},
            "properties": {second_site_properties},
            "pages": [
              {{
                "id": "home",
                "label": "Home",
                "representations": [
                  {{
                    "locale": "en-001",
                    "route": {{ "path": "/second/en-001/home" }},
                    "sourceCandidates": {{ "markdown": "pages/second-home.en-001.md" }},
                    "properties": {second_representation_properties}
                  }}
                ]
              }}
            ]
          }}
        ]
      }}
    ]
  }}
}}"#
    )
}

fn second_site_id() -> SiteIdV1 {
    SiteIdV1::parse("second").expect("a canonical Site identifier")
}

fn second_representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(
        entity(),
        facet(),
        second_site_id(),
        page(),
        locale("en-001"),
    )
}

/// `R03A-05`/`R03A-09` - a singular conflict in a Site the target never visits refuses the whole
/// candidate, even though the requested branch itself is valid.
#[test]
fn r03a_09_unrelated_site_conflict_refuses_the_whole_candidate() {
    let document = manifest_two_sites(
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Second site", "exposure": "public" } ] } }"#,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Second site EN", "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("the second Site conflict refuses the whole candidate");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::CardinalityExceeded
        )
    );
    assert_eq!(refusal.code(), "PROPERTY_RESOLUTION_REFUSED");

    // Without the conflicting second-Site declarations the same shape succeeds, proving the
    // refusal came from the whole-graph check and not from the target's own chain.
    let clean = manifest_two_sites(NO_PROPERTIES, NO_PROPERTIES, NO_PROPERTIES, NO_PROPERTIES);
    let clean_parsed = parse_document(&clean);
    let clean_graph = graph_from(&clean_parsed);
    let candidate = validate_manifest_property_candidate_v1(
        &clean_parsed,
        &clean_graph,
        &representation_owner(),
    )
    .expect("a conflict-free graph resolves");
    assert_eq!(candidate.resolved_owner_count(), 8);
    assert!(
        candidate
            .resolved_graph()
            .for_owner(&second_representation_owner())
            .is_some(),
        "the unrelated Site is resolved and validated too"
    );
}

/// `R03A-09` - the same conflict on an **unlisted** representation refuses the whole candidate.
#[test]
fn r03a_09_unlisted_representation_conflict_refuses() {
    let document = manifest_with_unlisted_sibling(
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Home", "exposure": "public" } ] } }"#,
        r#"{ "seo.title": { "inheritance": "inherit", "currentContext": [ { "text": "Beranda", "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("the unlisted representation conflict refuses the whole candidate");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::CardinalityExceeded
        )
    );
}

/// `R03A-09`/`R03A-08` - a graph whose membership differs from the manifest refuses even when the
/// differing owners declare no properties at all.
#[test]
fn r03a_09_graph_membership_mismatch_refuses() {
    let two_sites = parse_document(&manifest_two_sites(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
    ));
    let single_site = parse_document(&manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
    ));

    // The narrower graph is missing the manifest's second Site.
    let narrow_graph = graph_from(&single_site);
    let refusal =
        validate_manifest_property_candidate_v1(&two_sites, &narrow_graph, &representation_owner())
            .expect_err("a graph missing a manifest owner refuses");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::OwnerMembershipMismatch
        )
    );

    // The matching graph resolves.
    let matching_graph = graph_from(&two_sites);
    validate_manifest_property_candidate_v1(&two_sites, &matching_graph, &representation_owner())
        .expect("the matching graph resolves");
}

/// `R03A-10`/`R03A-09` - a valid recognized extension in another Site refuses the whole candidate.
#[test]
fn r03a_09_extension_anywhere_refuses_the_whole_candidate() {
    let document = manifest_two_sites(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "main" }, "factId": "main-base" }, "set": { "locality": "Ubud" } } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("the extension in the second Site refuses the whole candidate");
    assert_eq!(refusal.code(), "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A
        )
    );
}

/// `R03A-06`/`R03A-10`/`R03A-09` - the accepted extend envelope is fully validated before it can
/// ever be reported as the named unsupported operation.
#[test]
fn r03a_09_extend_syntax_refusals() {
    fn extend(target_owner: &str, set: &str) -> String {
        format!(
            r#"{{ "location": {{ "inheritance": "inherit", "currentContext": [ {{ "operation": "extend", "target": {{ "owner": {target_owner}, "factId": "bali-base" }}, "set": {set} }} ] }} }}"#
        )
    }

    let cases: Vec<(String, PropertyRefusalKind)> = vec![
        (
            extend(r#"{ "entity": "example", "facet": "main" }"#, "{}"),
            PropertyRefusalKind::EmptyLocationFields,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main" }"#,
                r#"{ "country": "ID" }"#,
            ),
            PropertyRefusalKind::UnknownLocationField,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main" }"#,
                r#"{ "locality": 42 }"#,
            ),
            PropertyRefusalKind::WrongEntryShape,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main" }"#,
                r#"{ "coordinates": { "latitude": 91, "longitude": 0 } }"#,
            ),
            PropertyRefusalKind::InvalidCoordinates,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main" }"#,
                r#"{ "countryCode": "id" }"#,
            ),
            PropertyRefusalKind::InvalidCountryCode,
        ),
        (
            extend(
                r#"{ "entity": "example", "page": "home" }"#,
                r#"{ "locality": "Ubud" }"#,
            ),
            PropertyRefusalKind::InconsistentOwner,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main", "site": "main", "page": "home", "locale": "en" }"#,
                r#"{ "locality": "Ubud" }"#,
            ),
            PropertyRefusalKind::InconsistentOwner,
        ),
        (
            extend(
                r#"{ "entity": "example", "facet": "main", "country": "ID" }"#,
                r#"{ "locality": "Ubud" }"#,
            ),
            PropertyRefusalKind::ExtraEntryField,
        ),
        (
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main" }, "factId": "bali-base" } } ] } }"#
                .to_owned(),
            PropertyRefusalKind::MissingEntryField,
        ),
    ];
    for (properties, expected) in cases {
        assert_eq!(
            page_level_refusal_kind(&properties),
            ManifestPropertyRefusalKind::DeclarationRefused(expected),
            "properties {properties} must be refused"
        );
    }

    // A complete, typed extension is still recognized, and refused only at resolution.
    let valid = extend(
        r#"{ "entity": "example", "facet": "main" }"#,
        r#"{ "locality": "Ubud", "countryCode": "ID" }"#,
    );
    let document = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        &valid,
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("a valid extension is still refused at resolution");
    assert_eq!(refusal.code(), "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A
        )
    );
}
