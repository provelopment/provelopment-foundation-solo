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
use solo_application::collect_graphic_asset_obligations_v1;
use solo_domain::{
    AssetNamespace, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1, Exposure, FacetDraft,
    FacetIdV1, FactIdV1, LocaleKeyV1, LocationExtensionCause, LocationKind, PageDraft, PageIdV1,
    PropertyKey, PropertyRefusalKind, PropertyResolutionRefusalKind, PropertyValueV1,
    ProvenanceValueV1, QualifiedOwnerV1, ResolutionOperation, SiteDraft, SiteIdV1,
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

/// `R03B-04` — converted from `R03A-10`. The accepted `extend` syntax is still recognized, and
/// a target with no inherited fact now refuses with the R03.B code and typed cause.
#[test]
fn r03b_04_extend_without_an_inherited_target_refuses() {
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
        .expect_err("an extension with no inherited target refuses");
    assert_eq!(refusal.code(), "LOCATION_INVALID_EXTEND");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::InvalidLocationExtension
        )
    );
    assert_eq!(
        refusal.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
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

/// `R03B-04`/`R03B-11` — converted from `R03A-10`. A sibling-targeted extension in another Site
/// now refuses with the R03.B code, names that declaration's owner, and fails the whole
/// candidate even though the requested branch is valid.
#[test]
fn r03b_04_extension_in_another_site_fails_the_whole_candidate() {
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
    assert_eq!(refusal.code(), "LOCATION_INVALID_EXTEND");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::InvalidLocationExtension
        )
    );
    assert_eq!(
        refusal.cause(),
        Some(LocationExtensionCause::TargetNotAncestor)
    );
    assert_eq!(
        refusal.owner().map(QualifiedOwnerV1::describe),
        Some(second_representation_owner().describe()),
        "the refusal names the offending declaration, not the requested target"
    );
    assert_eq!(refusal.property(), Some(PropertyKey::Location));
}

/// `R03B-08` — the accepted `extend` envelope stays fully closed at the JSON boundary: empty or
/// unknown `set`, `null`, wrong types, invalid geometry, locale, lineage and extra members all
/// refuse as declarations, never as resolution outcomes.
#[test]
fn r03b_08_extend_syntax_refusals_are_unchanged() {
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

    // A closed extension may not author an `exposure`, and a `null` member is not a typed value.
    assert_eq!(
        page_level_refusal_kind(
            r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main" }, "factId": "bali-base" }, "set": { "locality": "Ubud" }, "exposure": "public" } ] } }"#
        ),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::ExtraEntryField)
    );
    assert_eq!(
        page_level_refusal_kind(&extend(
            r#"{ "entity": "example", "facet": "main" }"#,
            r#"{ "locality": null }"#,
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::WrongEntryShape)
    );

    // A syntactically complete extension is recognized as a declaration and refused only when
    // resolution finds no inherited fact to extend.
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
        .expect_err("a valid extension with no inherited target is refused at resolution");
    assert_eq!(refusal.code(), "LOCATION_INVALID_EXTEND");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::ResolutionRefused(
            PropertyResolutionRefusalKind::InvalidLocationExtension
        )
    );
    assert_eq!(
        refusal.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
    );
}

/// The **unlisted** `id-ID` representation owner of the two-representation test manifest.
fn unlisted_representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), site(), page(), locale("id-ID"))
}

/// The Bali Site owner of the two-Site geography manifest.
fn bali_site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), bali_site_id())
}

/// The Jakarta Site owner of the two-Site geography manifest.
fn jakarta_site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), jakarta_site_id())
}

fn bali_site_id() -> SiteIdV1 {
    SiteIdV1::parse("bali").expect("a canonical Site identifier")
}

fn jakarta_site_id() -> SiteIdV1 {
    SiteIdV1::parse("jakarta").expect("a canonical Site identifier")
}

/// The Bali Page owner of the two-Site geography manifest.
fn bali_page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), bali_site_id(), page())
}

/// The Jakarta Page owner of the two-Site geography manifest.
fn jakarta_page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), jakarta_site_id(), page())
}

/// The Bali representation owner for one authored full-locale key.
fn bali_representation_owner(language: &str) -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), bali_site_id(), page(), locale(language))
}

/// The Jakarta representation owner for one authored full-locale key.
fn jakarta_representation_owner(language: &str) -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(
        entity(),
        facet(),
        jakarta_site_id(),
        page(),
        locale(language),
    )
}

/// Every field origin of one resolved location value as `(field, owner, local index)`.
fn origins_of(value: &ProvenanceValueV1) -> Vec<(String, String, usize)> {
    value
        .field_origins()
        .expect("field origins")
        .iter()
        .map(|(name, origin)| {
            (
                name.clone(),
                origin.owner().describe(),
                origin.local_index(),
            )
        })
        .collect()
}

/// The authored five-level geography manifest: one Entity with an Australian association fact,
/// one Facet with its own Indonesian premises fact, and two sibling Sites — Bali and Jakarta —
/// each authoring its own fact and extending an inherited one. The Bali Page and both of its
/// full-locale representations extend as well, and the Jakarta representation extends its own
/// Site fact. Nothing is inferred from a locale, route, name, identifier or coordinate.
const GEOGRAPHY_MANIFEST: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {
    "canonical": "https://example.org/site",
    "landing": { "entity": "example", "facet": "main", "site": "bali", "page": "home", "locale": "en-001" }
  },
  "entity": {
    "id": "example",
    "label": "Example",
    "properties": {
      "location": { "inheritance": "inherit", "currentContext": [
        { "operation": "add", "factId": "aus-hq", "kind": "association",
          "fields": { "worldRegion": "Oceania", "countryCode": "AU" }, "exposure": "public" }
      ] }
    },
    "facets": [
      {
        "id": "main",
        "label": "Main",
        "properties": {
          "location": { "inheritance": "inherit", "currentContext": [
            { "operation": "add", "factId": "id-office", "kind": "premises",
              "fields": { "countryCode": "ID", "locality": "Jakarta" }, "exposure": "internal" },
            { "operation": "extend", "target": { "owner": { "entity": "example" }, "factId": "aus-hq" },
              "set": { "administrativeArea": "Bali" } }
          ] }
        },
        "sites": [
          {
            "id": "bali",
            "label": "Bali site",
            "defaultLocale": "en-001",
            "languagePreferences": { "en": "en-001", "id": "id-ID" },
            "properties": {
              "location": { "inheritance": "inherit", "currentContext": [
                { "operation": "add", "factId": "bali-base", "kind": "association",
                  "fields": { "countryCode": "ID", "locality": "Denpasar" }, "exposure": "public" },
                { "operation": "extend", "target": { "owner": { "entity": "example" }, "factId": "aus-hq" },
                  "set": { "district": "Renon" } }
              ] }
            },
            "pages": [
              {
                "id": "home",
                "label": "Bali home",
                "properties": {
                  "location": { "inheritance": "inherit", "currentContext": [
                    { "operation": "extend", "target": { "owner": { "entity": "example" }, "factId": "aus-hq" },
                      "set": { "addressLine1": "Jalan Raya 1" } },
                    { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "bali" }, "factId": "bali-base" },
                      "set": { "addressLine2": "Suite 2" } }
                  ] }
                },
                "representations": [
                  {
                    "locale": "en-001",
                    "route": { "path": "/en-001/home" },
                    "sourceCandidates": { "markdown": "pages/bali-home.en-001.md" },
                    "properties": {
                      "location": { "inheritance": "inherit", "currentContext": [
                        { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "bali" }, "factId": "bali-base" },
                          "set": { "coordinates": { "latitude": -8.65, "longitude": 115.22 } } }
                      ] }
                    }
                  },
                  {
                    "locale": "id-ID",
                    "route": { "path": "/id-ID/home" },
                    "sourceCandidates": { "markdown": "pages/bali-home.id-id.md" },
                    "properties": {
                      "location": { "inheritance": "inherit", "currentContext": [
                        { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "bali" }, "factId": "bali-base" },
                          "set": { "namedPlace": "Renon" } }
                      ] }
                    }
                  }
                ]
              }
            ]
          },
          {
            "id": "jakarta",
            "label": "Jakarta site",
            "defaultLocale": "en-001",
            "languagePreferences": { "en": "en-001" },
            "properties": {
              "location": { "inheritance": "inherit", "currentContext": [
                { "operation": "add", "factId": "jkt-hq", "kind": "premises",
                  "fields": { "countryCode": "ID", "locality": "Jakarta", "postalCode": "10110" }, "exposure": "public" },
                { "operation": "extend", "target": { "owner": { "entity": "example" }, "factId": "aus-hq" },
                  "set": { "namedPlace": "Monas" } }
              ] }
            },
            "pages": [
              {
                "id": "home",
                "label": "Jakarta home",
                "representations": [
                  {
                    "locale": "en-001",
                    "route": { "path": "/jakarta/en-001/home" },
                    "sourceCandidates": { "markdown": "pages/jakarta-home.en-001.md" },
                    "properties": {
                      "location": { "inheritance": "inherit", "currentContext": [
                        { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "jakarta" }, "factId": "jkt-hq" },
                          "set": { "addressLine1": "Jalan Monas 2" } }
                      ] }
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
}"#;

/// `R03B-10` — real strict JSON bytes with an independently authored Indonesia Facet fact and
/// sibling Bali/Jakarta Sites. Every extension keeps its original identity, each branch sees only
/// its own changes, and Page identity, locale, base URL and authored route claims are unchanged.
#[test]
fn r03b_10_real_json_geography_extends_without_identity_or_locale_effects() {
    let parsed = parse_document(GEOGRAPHY_MANIFEST);
    let graph = graph_from(&parsed);

    // Authored identity, base and route claims, read from containment and the manifest.
    assert_eq!(parsed.entity().id().as_str(), "example");
    assert_eq!(parsed.base().canonical(), "https://example.org/site");
    let facet_declaration = &parsed.entity().facets()[0];
    assert_eq!(facet_declaration.id().as_str(), "main");
    assert_eq!(
        facet_declaration
            .sites()
            .iter()
            .map(|declaration| declaration.id().as_str())
            .collect::<Vec<_>>(),
        vec!["bali", "jakarta"]
    );
    let bali_pages = facet_declaration.sites()[0].pages();
    assert_eq!(bali_pages.len(), 1);
    assert_eq!(
        bali_pages[0]
            .representations()
            .iter()
            .map(|representation| representation.locale())
            .collect::<Vec<_>>(),
        vec!["en-001", "id-ID"]
    );
    assert_eq!(
        bali_pages[0]
            .representations()
            .iter()
            .map(|representation| representation.route().path())
            .collect::<Vec<_>>(),
        vec!["/en-001/home", "/id-ID/home"]
    );
    assert_eq!(
        facet_declaration.sites()[1].pages()[0]
            .representations()
            .iter()
            .map(|representation| representation.route().path())
            .collect::<Vec<_>>(),
        vec!["/jakarta/en-001/home"]
    );

    // ---- Bali, `en-001` ----
    let english = validate_manifest_property_candidate_v1(
        &parsed,
        &graph,
        &bali_representation_owner("en-001"),
    )
    .expect("the Bali en-001 branch resolves");
    assert!(!english.is_publication_authorized());
    assert_eq!(
        english.public().values_of(PropertyKey::Location).len(),
        2,
        "the internal Facet fact is never projected"
    );
    let resolved = english.resolved();
    let facts = resolved.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 3, "one fact per author level, inherited first");
    assert_eq!(
        facts
            .iter()
            .map(|value| value.origin_owner().describe())
            .collect::<Vec<_>>(),
        vec![
            entity_owner().describe(),
            facet_owner().describe(),
            bali_site_owner().describe()
        ]
    );
    let australia = facts[0].data().location_fact().expect("a fact");
    assert_eq!(australia.fact_id().as_str(), "aus-hq");
    assert_eq!(australia.kind(), LocationKind::Association);
    assert_eq!(
        australia.fields().set_field_names(),
        vec![
            "worldRegion",
            "countryCode",
            "administrativeArea",
            "district",
            "addressLine1"
        ]
    );
    assert_eq!(
        origins_of(&facts[0]),
        vec![
            ("worldRegion".to_owned(), entity_owner().describe(), 0),
            ("countryCode".to_owned(), entity_owner().describe(), 0),
            ("administrativeArea".to_owned(), facet_owner().describe(), 1),
            ("district".to_owned(), bali_site_owner().describe(), 1),
            ("addressLine1".to_owned(), bali_page_owner().describe(), 0),
        ]
    );
    assert!(facts[0].exposure().is_public());
    let indonesia = facts[1].data().location_fact().expect("a fact");
    assert_eq!(indonesia.fact_id().as_str(), "id-office");
    assert_eq!(indonesia.kind(), LocationKind::Premises);
    assert_eq!(facts[1].exposure(), Exposure::Internal);
    assert_eq!(
        origins_of(&facts[1]),
        vec![
            ("countryCode".to_owned(), facet_owner().describe(), 0),
            ("locality".to_owned(), facet_owner().describe(), 0),
        ]
    );
    let bali = facts[2].data().location_fact().expect("a fact");
    assert_eq!(bali.fact_id().as_str(), "bali-base");
    assert!(facts[2].exposure().is_public());
    assert_eq!(
        bali.fields().set_field_names(),
        vec!["countryCode", "locality", "addressLine2", "coordinates"]
    );
    assert_eq!(
        origins_of(&facts[2]),
        vec![
            ("countryCode".to_owned(), bali_site_owner().describe(), 0),
            ("locality".to_owned(), bali_site_owner().describe(), 0),
            ("addressLine2".to_owned(), bali_page_owner().describe(), 1),
            (
                "coordinates".to_owned(),
                bali_representation_owner("en-001").describe(),
                0
            ),
        ]
    );
    assert_eq!(
        english
            .public()
            .values_of(PropertyKey::Location)
            .iter()
            .map(|value| value.location_fact().expect("a fact").fact_id().to_string())
            .collect::<Vec<_>>(),
        vec!["aus-hq".to_owned(), "bali-base".to_owned()],
        "the public projection keeps effective order and drops the internal fact"
    );
    let events = resolved
        .property(PropertyKey::Location)
        .expect("a location record")
        .resolution_events();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.operation() == ResolutionOperation::Added)
            .count(),
        3,
        "one `added` event per independently authored fact"
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.operation() == ResolutionOperation::Extended)
            .count(),
        5,
        "one `extended` event per successful extension on this direct chain"
    );
    assert!(
        events.iter().any(|event| {
            event.operation() == ResolutionOperation::Extended
                && event.target_fact_id().map(FactIdV1::as_str) == Some("bali-base")
        }),
        "the representation's extension names its inherited Site fact"
    );

    // ---- Bali, `id-ID`: its own extension, never the sibling's coordinate ----
    let indonesian = validate_manifest_property_candidate_v1(
        &parsed,
        &graph,
        &bali_representation_owner("id-ID"),
    )
    .expect("the Bali id-ID branch resolves");
    let indonesian_facts = indonesian.resolved().values_of(PropertyKey::Location);
    assert_eq!(indonesian_facts.len(), 3);
    let indonesian_bali = indonesian_facts[2].data().location_fact().expect("a fact");
    assert_eq!(indonesian_bali.fact_id().as_str(), "bali-base");
    assert_eq!(
        indonesian_bali.fields().set_field_names(),
        vec!["countryCode", "locality", "namedPlace", "addressLine2"]
    );
    assert!(
        !indonesian_bali
            .fields()
            .set_field_names()
            .contains(&"coordinates"),
        "the en-001 coordinate never reaches the id-ID representation"
    );
    assert_eq!(
        origins_of(&indonesian_facts[2])[2],
        (
            "namedPlace".to_owned(),
            bali_representation_owner("id-ID").describe(),
            0
        )
    );

    // ---- Jakarta: its own Site fact, its own extension, no Bali value ----
    let jakarta = validate_manifest_property_candidate_v1(
        &parsed,
        &graph,
        &jakarta_representation_owner("en-001"),
    )
    .expect("the Jakarta branch resolves");
    let jakarta_facts = jakarta.resolved().values_of(PropertyKey::Location);
    assert_eq!(jakarta_facts.len(), 3);
    assert_eq!(
        jakarta_facts
            .iter()
            .map(|value| value.origin_owner().describe())
            .collect::<Vec<_>>(),
        vec![
            entity_owner().describe(),
            facet_owner().describe(),
            jakarta_site_owner().describe()
        ]
    );
    let jakarta_australia = jakarta_facts[0].data().location_fact().expect("a fact");
    assert_eq!(
        jakarta_australia.fields().set_field_names(),
        vec![
            "worldRegion",
            "countryCode",
            "administrativeArea",
            "namedPlace"
        ]
    );
    assert!(
        !jakarta_australia
            .fields()
            .set_field_names()
            .contains(&"district"),
        "the Bali Site extension never reaches Jakarta"
    );
    let jakarta_hq = jakarta_facts[2].data().location_fact().expect("a fact");
    assert_eq!(jakarta_hq.fact_id().as_str(), "jkt-hq");
    assert_eq!(
        jakarta_hq.fields().set_field_names(),
        vec!["countryCode", "locality", "addressLine1", "postalCode"]
    );
    assert_eq!(
        origins_of(&jakarta_facts[2])[2],
        (
            "addressLine1".to_owned(),
            jakarta_representation_owner("en-001").describe(),
            0
        )
    );
    assert_eq!(
        jakarta.public().values_of(PropertyKey::Location).len(),
        2,
        "the internal Facet fact stays unprojected in this branch too"
    );

    // ---- identity is language-neutral and independent of every property value ----
    assert_eq!(bali_page_owner().page_id(), jakarta_page_owner().page_id());
    assert_eq!(
        bali_page_owner().entity_id(),
        jakarta_page_owner().entity_id()
    );
    assert_eq!(
        bali_representation_owner("en-001").page_id(),
        bali_representation_owner("id-ID").page_id()
    );
    assert_ne!(
        bali_representation_owner("en-001").site_id(),
        jakarta_representation_owner("en-001").site_id()
    );
    assert_eq!(
        bali_representation_owner("en-001").locale(),
        Some(&locale("en-001"))
    );
}

/// `R03B-11` — an invalid `extend` on an **unlisted** representation fails the whole candidate
/// even though the requested `en-001` branch is valid, and the refusal names that declaration.
#[test]
fn r03b_11_unlisted_invalid_extension_fails_the_whole_candidate() {
    let page_properties = r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "add", "factId": "page-annex", "kind": "association", "fields": { "countryCode": "ID", "locality": "Sanur" }, "exposure": "public" } ] } }"#;
    let invalid_sibling = r#"{ "location": { "inheritance": "inherit", "currentContext": [ { "operation": "extend", "target": { "owner": { "entity": "example", "facet": "main", "site": "main" }, "factId": "main-base" }, "set": { "locality": "Ubud" } } ] } }"#;
    let parsed = parse_document(&manifest_with_unlisted_sibling(
        page_properties,
        invalid_sibling,
    ));
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("the unlisted sibling's invalid extension fails the whole candidate");
    assert_eq!(refusal.code(), "LOCATION_INVALID_EXTEND");
    assert_eq!(
        refusal.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
    );
    assert_eq!(
        refusal.owner().map(QualifiedOwnerV1::describe),
        Some(unlisted_representation_owner().describe()),
        "the refusal names the invalid unlisted declaration"
    );
    assert_eq!(refusal.property(), Some(PropertyKey::Location));
    let rendered = format!("{refusal}");
    assert!(!rendered.contains("Ubud"), "no authored value is echoed");
    assert!(!rendered.contains("Sanur"), "no unrelated value is echoed");

    // The same manifest without the invalid declaration resolves, and the unlisted branch keeps
    // its own inherited copy of the same Page fact.
    let control = parse_document(&manifest_with_unlisted_sibling(
        page_properties,
        NO_PROPERTIES,
    ));
    let control_graph = graph_from(&control);
    let requested =
        validate_manifest_property_candidate_v1(&control, &control_graph, &representation_owner())
            .expect("the valid branch resolves");
    assert_eq!(requested.public().values_of(PropertyKey::Location).len(), 1);
    let unlisted = validate_manifest_property_candidate_v1(
        &control,
        &control_graph,
        &unlisted_representation_owner(),
    )
    .expect("the unlisted branch resolves");
    let unlisted_values = unlisted.public().values_of(PropertyKey::Location);
    assert_eq!(unlisted_values.len(), 1);
    assert_eq!(
        unlisted_values[0]
            .location_fact()
            .expect("a fact")
            .fact_id()
            .as_str(),
        "page-annex",
        "unlisted visibility changes no property inheritance"
    );
}

/// `R03C1-08` — real strict JSON through the unchanged parser, the unchanged manifest
/// translation, the accepted use case and then the new inventory: all five owning levels are
/// collected with their exact declaring owner, role, local index, namespace, exposure and
/// reference, and the result is still nonpublishing.
#[test]
fn r03c1_08_real_json_graphic_inventory_covers_all_five_levels() {
    let document = manifest(
        r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "brand/entity-logo.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.banner": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "brand/facet-banner.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.background": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "bg/site-1.png" }, "exposure": "public" }, { "source": { "namespace": "platform", "reference": "shared/site-2.webp" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.social-preview": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "social/page.jpg" }, "exposure": "internal" } ] } }"#,
        r#"{ "graphic.favicon": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "icons/favicon.png" }, "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let candidate =
        validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
            .expect("a valid whole-graph property candidate");
    assert!(!candidate.is_publication_authorized());

    let inventory = collect_graphic_asset_obligations_v1(candidate.candidate())
        .expect("the authored inventory collects");
    assert_eq!(inventory.len(), 6);
    let rows = inventory.obligations();
    let observed: Vec<(String, &str, usize, AssetNamespace, &str, Exposure)> = rows
        .iter()
        .map(|row| {
            (
                row.declaring_owner().describe(),
                row.role().name(),
                row.local_index(),
                row.namespace(),
                row.reference_text(),
                row.exposure(),
            )
        })
        .collect();
    let expected: Vec<(String, &str, usize, AssetNamespace, &str, Exposure)> = vec![
        (
            entity_owner().describe(),
            "graphic.logo",
            0,
            AssetNamespace::Entity,
            "brand/entity-logo.png",
            Exposure::Public,
        ),
        (
            facet_owner().describe(),
            "graphic.banner",
            0,
            AssetNamespace::Entity,
            "brand/facet-banner.png",
            Exposure::Public,
        ),
        (
            site_owner().describe(),
            "graphic.background",
            0,
            AssetNamespace::Entity,
            "bg/site-1.png",
            Exposure::Public,
        ),
        (
            site_owner().describe(),
            "graphic.background",
            1,
            AssetNamespace::Platform,
            "shared/site-2.webp",
            Exposure::Public,
        ),
        (
            page_owner().describe(),
            "graphic.social-preview",
            0,
            AssetNamespace::Entity,
            "social/page.jpg",
            Exposure::Internal,
        ),
        (
            representation_owner().describe(),
            "graphic.favicon",
            0,
            AssetNamespace::Entity,
            "icons/favicon.png",
            Exposure::Public,
        ),
    ];
    assert_eq!(observed, expected);

    // The internal Page social preview stays inventoried even though nothing public shows it.
    assert!(
        candidate
            .public()
            .values_of(PropertyKey::GraphicSocialPreview)
            .is_empty()
    );
    assert!(!inventory.grants_publication_authority());
    assert!(!candidate.candidate().is_publication_authorized());
}

/// `R03C1-08` — an invalid graphic declaration on an **unlisted, unrequested** representation
/// refuses the whole candidate before any inventory exists, and the refusal names the true
/// declaring owner and key without echoing authored content.
#[test]
fn r03c1_08_invalid_graphic_on_an_unlisted_representation_refuses_before_inventory() {
    let document = manifest_with_unlisted_sibling(
        NO_PROPERTIES,
        r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "../escape.png" }, "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let refusal = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect_err("an unsafe graphic reference on the unlisted branch fails closed");
    assert_eq!(
        refusal.kind(),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidAssetReference)
    );
    assert_eq!(
        refusal.owner().map(QualifiedOwnerV1::describe),
        Some(unlisted_representation_owner().describe()),
        "the refusal names the declaring unlisted representation"
    );
    assert_eq!(refusal.property(), Some(PropertyKey::GraphicLogo));
    let message = refusal.to_string();
    assert!(message.contains("graphic.logo"));
    assert!(
        !message.contains("escape"),
        "no authored reference is echoed"
    );

    // The same manifest without the unsafe declaration yields one inventory per authored source,
    // and the unlisted declaration is included because unlisted is not invisible to authoring.
    let control = manifest_with_unlisted_sibling(
        NO_PROPERTIES,
        r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "brand/unlisted-logo.png" }, "exposure": "public" } ] } }"#,
    );
    let control = parse_document(&control);
    let control_graph = graph_from(&control);
    let requested =
        validate_manifest_property_candidate_v1(&control, &control_graph, &representation_owner())
            .expect("the requested branch resolves");
    let from_public = collect_graphic_asset_obligations_v1(requested.candidate())
        .expect("the inventory collects");
    let unlisted = validate_manifest_property_candidate_v1(
        &control,
        &control_graph,
        &unlisted_representation_owner(),
    )
    .expect("the unlisted branch resolves");
    let from_unlisted =
        collect_graphic_asset_obligations_v1(unlisted.candidate()).expect("the inventory collects");

    assert_eq!(from_public.len(), 1);
    assert_eq!(from_public, from_unlisted);
    let only = &from_public.obligations()[0];
    assert_eq!(
        only.declaring_owner(),
        &unlisted_representation_owner(),
        "the obligation keeps its own unlisted declaring owner"
    );
    assert_eq!(only.reference_text(), "brand/unlisted-logo.png");
    assert_eq!(only.role(), PropertyKey::GraphicLogo);
}

/// `R03C1-03` — two sibling Sites and two full-locale representations author different graphics;
/// the inventory is identical for every valid requested owner and never infers geography.
#[test]
fn r03c1_03_sibling_sites_and_two_locales_inventory_is_target_independent() {
    // Two sibling Sites under one Facet, each with its own Page `home` in `en-001`.
    let two_sites = manifest_two_sites(
        r#"{ "graphic.banner": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "bali/banner.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.logo": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "bali/logo-en.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.banner": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "jakarta/banner.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.favicon": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "jakarta/favicon.png" }, "exposure": "internal" } ] } }"#,
    );
    let parsed = parse_document(&two_sites);
    let graph = graph_from(&parsed);
    let first = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect("the first Site resolves");
    let second =
        validate_manifest_property_candidate_v1(&parsed, &graph, &second_representation_owner())
            .expect("the second Site resolves");
    let from_first =
        collect_graphic_asset_obligations_v1(first.candidate()).expect("the inventory collects");
    let from_second =
        collect_graphic_asset_obligations_v1(second.candidate()).expect("the inventory collects");

    assert_eq!(from_first, from_second);
    assert_eq!(from_first.len(), 4);
    let owners: Vec<String> = from_first
        .obligations()
        .iter()
        .map(|row| row.declaring_owner().describe())
        .collect();
    assert_eq!(
        owners,
        vec![
            site_owner().describe(),
            representation_owner().describe(),
            QualifiedOwnerV1::site(entity(), facet(), second_site_id()).describe(),
            second_representation_owner().describe(),
        ],
        "each sibling keeps its own declaring owner, in graph order"
    );
    // The internal second-Site favicon stays inventoried but is never published.
    assert!(
        second
            .public()
            .values_of(PropertyKey::GraphicFavicon)
            .is_empty()
    );

    // One Site with two full-locale representations, the second being unlisted.
    let two_locales = manifest_with_unlisted_sibling(
        r#"{ "graphic.background": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "bg/page.png" }, "exposure": "public" } ] } }"#,
        r#"{ "graphic.banner": { "inheritance": "inherit", "currentContext": [ { "source": { "namespace": "entity", "reference": "id/banner.png" }, "exposure": "public" } ] } }"#,
    );
    let parsed = parse_document(&two_locales);
    let graph = graph_from(&parsed);
    let english = validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
        .expect("the en-001 branch resolves");
    let indonesian =
        validate_manifest_property_candidate_v1(&parsed, &graph, &unlisted_representation_owner())
            .expect("the unlisted id-ID branch resolves");
    let from_english = collect_graphic_asset_obligations_v1(english.candidate()).expect("collects");
    let from_indonesian =
        collect_graphic_asset_obligations_v1(indonesian.candidate()).expect("collects");

    assert_eq!(from_english, from_indonesian);
    assert_eq!(from_english.len(), 2);
    assert_eq!(
        from_english.obligations()[0].declaring_owner(),
        &page_owner()
    );
    assert_eq!(
        from_english.obligations()[1].declaring_owner(),
        &unlisted_representation_owner(),
        "a locale is membership, and the unlisted representation keeps its own obligation"
    );
    assert_eq!(
        from_english.obligations()[1].reference_text(),
        "id/banner.png"
    );
}

/// One Page-level graphic declaration as strict JSON.
fn graphic_page_properties(role: &str, reference: &str, exposure: &str) -> String {
    format!(
        r#"{{ "{role}": {{ "inheritance": "inherit", "currentContext": [ {{ "source": {{ "namespace": "entity", "reference": "{reference}" }}, "exposure": "{exposure}" }} ] }} }}"#
    )
}

/// `R03C1-09` — the closed reference grammar is unchanged: the accepted image extensions still
/// parse, external/absolute/traversal/backslash/encoded forms still refuse, SVG and GIF still
/// refuse, a favicon stays PNG-only, and an accepted reference is never attested as existing,
/// safe or published media.
#[test]
fn r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested() {
    for extension in ["png", "jpg", "jpeg", "webp", "avif"] {
        let reference = format!("media/logo.{extension}");
        let document = manifest(
            NO_PROPERTIES,
            NO_PROPERTIES,
            NO_PROPERTIES,
            &graphic_page_properties("graphic.logo", &reference, "public"),
            NO_PROPERTIES,
        );
        let parsed = parse_document(&document);
        let graph = graph_from(&parsed);
        let candidate =
            validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
                .expect("an accepted image reference resolves");
        let inventory =
            collect_graphic_asset_obligations_v1(candidate.candidate()).expect("collects");
        assert_eq!(inventory.len(), 1);
        assert_eq!(
            inventory.obligations()[0].reference_text(),
            reference.as_str()
        );
        assert_eq!(
            inventory.obligations()[0].namespace(),
            AssetNamespace::Entity
        );
        // Existence, confinement, magic bytes, dimensions, licence and catalogue membership are
        // all still unproven: the inventory never claims them.
        assert!(!inventory.grants_publication_authority());
        assert!(!inventory.obligations()[0].grants_publication_authority());
    }

    let invalid_reference =
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::InvalidAssetReference);
    for reference in [
        "https://example.org/logo.png",
        "/media/logo.png",
        "../escape.png",
        r"media\\logo.png",
        "media/logo.png?x=1",
        "media/%2e%2e/logo.png",
        "media/logo.png#fragment",
    ] {
        assert_eq!(
            page_level_refusal_kind(&graphic_page_properties(
                "graphic.logo",
                reference,
                "public"
            )),
            invalid_reference,
            "reference `{reference}` must refuse as an invalid asset reference"
        );
    }

    let unsupported = ManifestPropertyRefusalKind::DeclarationRefused(
        PropertyRefusalKind::UnsupportedAssetExtension,
    );
    for reference in ["media/logo.svg", "media/logo.gif"] {
        assert_eq!(
            page_level_refusal_kind(&graphic_page_properties(
                "graphic.logo",
                reference,
                "public"
            )),
            unsupported,
            "reference `{reference}` must refuse as an unsupported extension"
        );
    }

    // A favicon remains PNG-only.
    assert_eq!(
        page_level_refusal_kind(&graphic_page_properties(
            "graphic.favicon",
            "media/favicon.jpg",
            "public"
        )),
        ManifestPropertyRefusalKind::DeclarationRefused(PropertyRefusalKind::FaviconNotPng)
    );

    // A PNG favicon is accepted, and its obligation keeps the exact reference.
    let document = manifest(
        NO_PROPERTIES,
        NO_PROPERTIES,
        NO_PROPERTIES,
        &graphic_page_properties("graphic.favicon", "media/favicon.png", "public"),
        NO_PROPERTIES,
    );
    let parsed = parse_document(&document);
    let graph = graph_from(&parsed);
    let candidate =
        validate_manifest_property_candidate_v1(&parsed, &graph, &representation_owner())
            .expect("a PNG favicon resolves");
    let inventory = collect_graphic_asset_obligations_v1(candidate.candidate()).expect("collects");
    assert_eq!(inventory.role_count(PropertyKey::GraphicFavicon), 1);
    assert_eq!(
        inventory.obligations()[0].reference_text(),
        "media/favicon.png"
    );
}
