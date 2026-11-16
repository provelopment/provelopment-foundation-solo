//! `R03.A` pure-domain property registry, cascade, provenance and projection matrix.
//!
//! Every test names the acceptance case it proves. The graph helpers build the accepted
//! one-Entity shape — one Entity, two Facets, Sites with the **same** Page identifier under
//! different Sites, and canonical full-locale representations — so isolation is proven by
//! identifiers rather than by a comment.

use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CoordinatesV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsLevel, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1,
    Exposure, FacetDraft, FacetIdV1, FactIdV1, Inheritance, LocaleKeyV1, LocationExtensionCause,
    LocationExtensionRequestV1, LocationFactV1, LocationFieldsV1, LocationKind,
    MAX_BACKGROUND_LAYERS, MAX_LOCAL_ENTRIES, MAX_LOCATION_FACTS, MAX_SEO_TITLE_SCALARS, MailboxV1,
    PROPERTY_REGISTRY_VERSION, PageDraft, PageIdV1, PlainTextV1, PropertyDeclarationV1,
    PropertyKey, PropertyRefusalKind, PropertyResolutionRefusalKind, PropertyResolutionRefusalV1,
    PropertyValueV1, ProvenanceValueV1, QualifiedOwnerV1, ResolutionOperation, SiteDraft, SiteIdV1,
    project_public_properties_v1, resolve_all_owners_v1, validate_declared_properties_v1,
};

fn entity_id() -> EntityIdV1 {
    EntityIdV1::parse("example").expect("a canonical Entity identifier")
}

fn facet(value: &str) -> FacetIdV1 {
    FacetIdV1::parse(value).expect("a canonical Facet identifier")
}

fn site(value: &str) -> SiteIdV1 {
    SiteIdV1::parse(value).expect("a canonical Site identifier")
}

fn page(value: &str) -> PageIdV1 {
    PageIdV1::parse(value).expect("a canonical Page identifier")
}

fn locale(value: &str) -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical(value).expect("a canonical full locale key")
}

/// Entity `example` containing Facet `main` (Sites `bali` and `jakarta`, both with a Page
/// `home`) and Facet `company` (Site `corporate` with a Page `home`).
fn graph() -> EfsOwnershipGraph {
    let mut bali_home = PageDraft::new(page("home"));
    bali_home.add_representation(locale("en-001"));
    bali_home.add_representation(locale("id-ID"));
    let mut bali_about = PageDraft::new(page("about"));
    bali_about.add_representation(locale("en-001"));
    let mut bali = SiteDraft::new(site("bali"));
    bali.add_page(bali_home);
    bali.add_page(bali_about);

    let mut jakarta_home = PageDraft::new(page("home"));
    jakarta_home.add_representation(locale("en-001"));
    let mut jakarta = SiteDraft::new(site("jakarta"));
    jakarta.add_page(jakarta_home);

    let mut main = FacetDraft::new(facet("main"));
    main.add_site(bali);
    main.add_site(jakarta);

    let mut corporate_home = PageDraft::new(page("home"));
    corporate_home.add_representation(locale("de-DE"));
    let mut corporate = SiteDraft::new(site("corporate"));
    corporate.add_page(corporate_home);
    let mut company = FacetDraft::new(facet("company"));
    company.add_site(corporate);

    let mut draft = EfsOwnershipDraft::new(entity_id());
    draft.add_facet(main);
    draft.add_facet(company);
    draft.build().expect("a valid one-Entity structure")
}

fn entity_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::entity(entity_id())
}

fn facet_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::facet(entity_id(), facet("main"))
}

fn site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity_id(), facet("main"), site("bali"))
}

fn jakarta_site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity_id(), facet("main"), site("jakarta"))
}

fn asset_references(values: &[ProvenanceValueV1]) -> Vec<String> {
    values
        .iter()
        .filter_map(|value| value.data().asset_reference())
        .map(|reference| reference.reference().to_owned())
        .collect()
}

fn page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity_id(), facet("main"), site("bali"), page("home"))
}

fn representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("bali"),
        page("home"),
        locale("en-001"),
    )
}

fn jakarta_representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("jakarta"),
        page("home"),
        locale("en-001"),
    )
}

fn auth(value: PropertyValueV1, exposure: Exposure) -> AuthoredPropertyValueV1 {
    AuthoredPropertyValueV1::new(value, exposure).expect("a valid authored value")
}

fn inherit(values: Vec<AuthoredPropertyValueV1>) -> PropertyDeclarationV1 {
    let context = if values.is_empty() {
        CurrentContextV1::None
    } else {
        CurrentContextV1::Values(values)
    };
    PropertyDeclarationV1::new(Inheritance::Inherit, context)
}

fn suppress(values: Vec<AuthoredPropertyValueV1>) -> PropertyDeclarationV1 {
    let context = if values.is_empty() {
        CurrentContextV1::None
    } else {
        CurrentContextV1::Values(values)
    };
    PropertyDeclarationV1::new(Inheritance::Suppress, context)
}

fn declared(
    owner: QualifiedOwnerV1,
    entries: Vec<(PropertyKey, PropertyDeclarationV1)>,
) -> DeclaredOwnerPropertiesV1 {
    DeclaredOwnerPropertiesV1::new(owner, entries).expect("a valid declaration set")
}

fn title(text: &str) -> PropertyValueV1 {
    PropertyValueV1::text(
        PlainTextV1::parse(text, MAX_SEO_TITLE_SCALARS).expect("valid plain text"),
    )
}

fn email(address: &str) -> PropertyValueV1 {
    PropertyValueV1::mailbox(MailboxV1::parse(address).expect("a valid mailbox"))
}

fn background(reference: &str) -> PropertyValueV1 {
    PropertyValueV1::graphic(
        AssetReferenceV1::parse(AssetNamespace::Entity, reference)
            .expect("a valid asset reference"),
    )
}

fn location(fact_id: &str, locality: &str) -> PropertyValueV1 {
    let mut fields = LocationFieldsV1::new();
    fields
        .set_country_code("ID")
        .expect("an explicit uppercase country code");
    fields.set_locality(locality).expect("valid locality text");
    PropertyValueV1::location(
        LocationFactV1::add(
            FactIdV1::parse(fact_id).expect("a valid fact identifier"),
            LocationKind::Association,
            fields,
        )
        .expect("a non-empty location fact"),
    )
}

fn extension_request(fact_id: &str, locality: &str) -> PropertyValueV1 {
    let mut set = LocationFieldsV1::new();
    set.set_locality(locality).expect("valid locality text");
    extension_to(site_owner(), fact_id, set)
}

/// One typed `extend` operation naming `target` and the members to set.
fn extension_to(target: QualifiedOwnerV1, fact_id: &str, set: LocationFieldsV1) -> PropertyValueV1 {
    PropertyValueV1::location_extension(
        LocationExtensionRequestV1::new(
            FactIdV1::parse(fact_id).expect("a valid fact identifier"),
            target,
            set,
        )
        .expect("a nonempty typed extension set"),
    )
}

/// `R03A-01` — the registry is exactly the nine accepted keys with the accepted spellings,
/// effective maxima and allowed EFS levels, at the accepted registry version.
#[test]
fn r03a_01_registry_declares_exactly_the_nine_accepted_keys() {
    assert_eq!(PROPERTY_REGISTRY_VERSION, "1.0.0");
    assert_eq!(PropertyKey::ALL.len(), 9);
    let names: Vec<&str> = PropertyKey::ALL.iter().map(|key| key.name()).collect();
    assert_eq!(
        names,
        vec![
            "location",
            "graphic.logo",
            "graphic.favicon",
            "graphic.banner",
            "graphic.background",
            "graphic.social-preview",
            "seo.title",
            "seo.description",
            "contact.email",
        ]
    );
    for key in PropertyKey::ALL {
        assert_eq!(PropertyKey::from_name(key.name()), Some(key));
    }
    assert_eq!(PropertyKey::from_name("SEO.Title"), None);
    assert_eq!(PropertyKey::from_name("seo.keywords"), None);
    assert_eq!(PropertyKey::from_name("custom"), None);

    assert_eq!(PropertyKey::Location.max_effective(), MAX_LOCATION_FACTS);
    assert_eq!(MAX_LOCATION_FACTS, 16);
    assert_eq!(
        PropertyKey::GraphicBackground.max_effective(),
        MAX_BACKGROUND_LAYERS
    );
    assert_eq!(MAX_BACKGROUND_LAYERS, 4);
    for key in PropertyKey::ALL {
        if key != PropertyKey::Location && key != PropertyKey::GraphicBackground {
            assert_eq!(key.max_effective(), 1, "{} is singular", key.name());
        }
    }

    for key in [PropertyKey::SeoTitle, PropertyKey::SeoDescription] {
        assert!(!key.allows_level(EfsLevel::Entity));
        assert!(!key.allows_level(EfsLevel::Facet));
        assert!(key.allows_level(EfsLevel::Site));
        assert!(key.allows_level(EfsLevel::Page));
        assert!(key.allows_level(EfsLevel::Representation));
    }
    for level in [
        EfsLevel::Entity,
        EfsLevel::Facet,
        EfsLevel::Site,
        EfsLevel::Page,
        EfsLevel::Representation,
    ] {
        assert!(PropertyKey::Location.allows_level(level));
        assert!(PropertyKey::ContactEmail.allows_level(level));
        assert!(PropertyKey::GraphicLogo.allows_level(level));
    }
}

/// `R03A-01` — closed encoding: a wrong-level key, a duplicate declaration, a wrong entry
/// shape and unsafe typed values are refused with their exact categories.
#[test]
fn r03a_01_closed_encoding_refuses_wrong_level_and_unsafe_values() {
    let entity_title = DeclaredOwnerPropertiesV1::new(
        entity_owner(),
        vec![(
            PropertyKey::SeoTitle,
            inherit(vec![auth(title("Entity title"), Exposure::Public)]),
        )],
    )
    .expect_err("an Entity-level seo.title must refuse");
    assert_eq!(entity_title.kind(), PropertyRefusalKind::WrongLevel);

    let duplicate = DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![
            (PropertyKey::SeoTitle, inherit(Vec::new())),
            (PropertyKey::SeoTitle, suppress(Vec::new())),
        ],
    )
    .expect_err("a duplicate declaration must refuse");
    assert_eq!(
        duplicate.kind(),
        PropertyRefusalKind::DuplicatePropertyEntry
    );

    let wrong_shape = DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![(
            PropertyKey::ContactEmail,
            inherit(vec![auth(title("Not a mailbox"), Exposure::Public)]),
        )],
    )
    .expect_err("a text value under contact.email must refuse");
    assert_eq!(wrong_shape.kind(), PropertyRefusalKind::WrongEntryShape);

    let jpeg_favicon = PropertyValueV1::graphic(
        AssetReferenceV1::parse(AssetNamespace::Platform, "icons/favicon.jpg")
            .expect("a valid JPEG reference"),
    );
    let favicon = DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![(
            PropertyKey::GraphicFavicon,
            inherit(vec![auth(jpeg_favicon, Exposure::Public)]),
        )],
    )
    .expect_err("a JPEG favicon must refuse");
    assert_eq!(favicon.kind(), PropertyRefusalKind::FaviconNotPng);

    let png_favicon = PropertyValueV1::graphic(
        AssetReferenceV1::parse(AssetNamespace::Platform, "icons/favicon.png")
            .expect("a valid PNG reference"),
    );
    DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![(
            PropertyKey::GraphicFavicon,
            inherit(vec![auth(png_favicon, Exposure::Public)]),
        )],
    )
    .expect("a PNG favicon is accepted");

    assert_eq!(
        PlainTextV1::parse("", MAX_SEO_TITLE_SCALARS)
            .expect_err("empty text refuses")
            .kind(),
        PropertyRefusalKind::EmptyPlainText
    );
    assert_eq!(
        PlainTextV1::parse("<b>bold</b>", MAX_SEO_TITLE_SCALARS)
            .expect_err("markup refuses")
            .kind(),
        PropertyRefusalKind::ActiveMarkup
    );
    assert_eq!(
        PlainTextV1::parse("line\nbreak", MAX_SEO_TITLE_SCALARS)
            .expect_err("a control character refuses")
            .kind(),
        PropertyRefusalKind::ControlCharacter
    );
    assert_eq!(
        PlainTextV1::parse(
            &"x".repeat(MAX_SEO_TITLE_SCALARS + 1),
            MAX_SEO_TITLE_SCALARS
        )
        .expect_err("over-long text refuses")
        .kind(),
        PropertyRefusalKind::PlainTextTooLong
    );

    for candidate in [
        "",
        "plain",
        "a@b",
        "a@@example.com",
        "a b@example.com",
        "Name <a@example.com>",
        "a@-example.com",
        "a@example.c",
        "a..b@example.com",
    ] {
        assert_eq!(
            MailboxV1::parse(candidate)
                .expect_err("an unsafe mailbox refuses")
                .kind(),
            PropertyRefusalKind::InvalidMailbox,
            "candidate {candidate} must be refused"
        );
    }
    MailboxV1::parse("contact@example.com").expect("a plain addr-spec is accepted");
}

/// `R03A-02` — a real five-level chain inherits only its direct ancestors, and changing one
/// branch changes only the descendants on that branch.
#[test]
fn r03a_02_five_levels_inherit_only_direct_ancestors() {
    fn five_levels(site_reference: &str) -> Vec<DeclaredOwnerPropertiesV1> {
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/entity.png"), Exposure::Public)]),
                )],
            ),
            declared(
                facet_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/facet.png"), Exposure::Public)]),
                )],
            ),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background(site_reference), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/page.png"), Exposure::Public)]),
                )],
            ),
            declared(
                representation_owner(),
                vec![(PropertyKey::GraphicBackground, inherit(Vec::new()))],
            ),
        ]
    }

    fn references(values: &[solo_domain::ProvenanceValueV1]) -> Vec<String> {
        values
            .iter()
            .filter_map(|value| value.data().asset_reference())
            .map(|reference| reference.reference().to_owned())
            .collect()
    }

    let input = validate_declared_properties_v1(&graph(), five_levels("bg/site.png"))
        .expect("a valid authored input");
    let resolved = input
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let values = resolved.values_of(PropertyKey::GraphicBackground);
    assert_eq!(values.len(), 4);
    let origins: Vec<QualifiedOwnerV1> = values
        .iter()
        .map(|value| value.origin_owner().clone())
        .collect();
    assert_eq!(
        origins,
        vec![entity_owner(), facet_owner(), site_owner(), page_owner()]
    );
    let indices: Vec<usize> = values
        .iter()
        .map(|value| value.origin_local_index())
        .collect();
    assert_eq!(indices, vec![0, 0, 0, 0]);
    assert_eq!(
        references(values),
        vec![
            "bg/entity.png",
            "bg/facet.png",
            "bg/site.png",
            "bg/page.png"
        ]
    );

    // The sibling Site never sees bali's Site or Page contribution.
    let jakarta = input
        .resolve(&jakarta_representation_owner())
        .expect("a valid resolution");
    assert_eq!(
        references(jakarta.values_of(PropertyKey::GraphicBackground)),
        vec!["bg/entity.png", "bg/facet.png"]
    );

    // Changing only the bali Site value changes only the bali branch.
    let changed = validate_declared_properties_v1(&graph(), five_levels("bg/site-changed.png"))
        .expect("a valid authored input");
    let bali_changed = changed
        .resolve(&representation_owner())
        .expect("a valid resolution");
    assert_eq!(
        references(bali_changed.values_of(PropertyKey::GraphicBackground)),
        vec![
            "bg/entity.png",
            "bg/facet.png",
            "bg/site-changed.png",
            "bg/page.png"
        ]
    );
    let jakarta_changed = changed
        .resolve(&jakarta_representation_owner())
        .expect("a valid resolution");
    assert_eq!(
        references(jakarta_changed.values_of(PropertyKey::GraphicBackground)),
        vec!["bg/entity.png", "bg/facet.png"]
    );
}

/// `R03A-03` — an absent declaration, an explicit `inherit` with the literal `"none"` and an
/// explicit `suppress` are three distinct authored states with distinct recorded operations.
#[test]
fn r03a_03_absent_and_explicit_none_are_distinct_and_suppression_is_final() {
    let entity_email = declared(
        entity_owner(),
        vec![(
            PropertyKey::ContactEmail,
            inherit(vec![auth(email("mail@example.com"), Exposure::Public)]),
        )],
    );

    // A: the Page declares nothing at all.
    let absent = validate_declared_properties_v1(&graph(), vec![entity_email.clone()])
        .expect("a valid input");
    assert!(absent.declaration_for(&page_owner()).is_none());
    let absent_resolved = absent
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let absent_record = absent_resolved
        .property(PropertyKey::ContactEmail)
        .expect("the contact record");
    assert_eq!(absent_record.values().len(), 1);
    let absent_events: Vec<ResolutionOperation> = absent_record
        .resolution_events()
        .iter()
        .map(|event| event.operation())
        .collect();
    assert_eq!(absent_events, vec![ResolutionOperation::Added]);

    // B: the Page declares `inherit` with the literal "none".
    let explicit = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_email.clone(),
            declared(
                page_owner(),
                vec![(PropertyKey::ContactEmail, inherit(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let explicit_envelope = explicit
        .declaration_for(&page_owner())
        .expect("the Page declares the key")
        .declaration(PropertyKey::ContactEmail)
        .expect("the declared key");
    assert!(
        explicit_envelope.current_context().is_none_literal(),
        "an explicit \"none\" stays distinguishable from absence"
    );
    let explicit_resolved = explicit
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let explicit_record = explicit_resolved
        .property(PropertyKey::ContactEmail)
        .expect("the contact record");
    assert_eq!(explicit_record.values().len(), 1);
    let explicit_events: Vec<(ResolutionOperation, Option<usize>)> = explicit_record
        .resolution_events()
        .iter()
        .map(|event| (event.operation(), event.local_index()))
        .collect();
    assert_eq!(
        explicit_events,
        vec![
            (ResolutionOperation::Added, Some(0)),
            (ResolutionOperation::Inherited, None),
        ]
    );

    // C: `suppress` with "none" removes the inherited value and never revives it.
    let suppressed = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_email.clone(),
            declared(
                page_owner(),
                vec![(PropertyKey::ContactEmail, suppress(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let suppressed_resolved = suppressed
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let suppressed_record = suppressed_resolved
        .property(PropertyKey::ContactEmail)
        .expect("the contact record");
    assert!(suppressed_record.values().is_empty());
    let suppressed_events: Vec<ResolutionOperation> = suppressed_record
        .resolution_events()
        .iter()
        .map(|event| event.operation())
        .collect();
    assert_eq!(
        suppressed_events,
        vec![ResolutionOperation::Added, ResolutionOperation::Suppressed,]
    );
    assert!(
        project_public_properties_v1(&suppressed_resolved)
            .values_of(PropertyKey::ContactEmail)
            .is_empty(),
        "a suppressed inherited value is never resurrected for public output"
    );

    // D: `suppress` plus one local value legitimately replaces the inherited value.
    let replaced = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_email.clone(),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::ContactEmail,
                    suppress(vec![auth(email("page@example.com"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    let replaced_resolved = replaced
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let replaced_values = replaced_resolved.values_of(PropertyKey::ContactEmail);
    assert_eq!(replaced_values.len(), 1);
    assert_eq!(replaced_values[0].origin_owner(), &page_owner());
    assert_eq!(replaced_values[0].origin_local_index(), 0);

    // E: a downstream `inherit` never restores what `suppress` removed.
    let downstream = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_email,
            declared(
                page_owner(),
                vec![(PropertyKey::ContactEmail, suppress(Vec::new()))],
            ),
            declared(
                representation_owner(),
                vec![(PropertyKey::ContactEmail, inherit(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let downstream_resolved = downstream
        .resolve(&representation_owner())
        .expect("a valid resolution");
    assert!(
        downstream_resolved
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
}

/// `R03A-04` — inherited-first then authored local order, with the exact qualified origins,
/// zero-based local indices, per-field location origins and deterministic repetition.
#[test]
fn r03a_04_ordering_provenance_and_field_origins_are_exact() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![
                    auth(location("bali-base", "Denpasar"), Exposure::Public),
                    auth(location("bali-hq", "Denpasar"), Exposure::Internal),
                ]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(location("bali-annex", "Ubud"), Exposure::Public)]),
            )],
        ),
    ];
    let input = validate_declared_properties_v1(&graph(), declarations).expect("a valid input");
    let resolved = input
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let record = resolved
        .property(PropertyKey::Location)
        .expect("the location record");
    assert_eq!(record.values().len(), 3);
    assert_eq!(record.values()[0].origin_owner(), &entity_owner());
    assert_eq!(record.values()[0].origin_local_index(), 0);
    assert_eq!(record.values()[1].origin_owner(), &entity_owner());
    assert_eq!(record.values()[1].origin_local_index(), 1);
    assert_eq!(record.values()[1].exposure(), Exposure::Internal);
    assert_eq!(record.values()[2].origin_owner(), &site_owner());
    assert_eq!(record.values()[2].origin_local_index(), 0);

    // Field origins name the adding node and that node's own array index, per set field.
    let origins = record.values()[2]
        .field_origins()
        .expect("a location fact carries field origins");
    let names: Vec<&str> = origins.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(names, vec!["countryCode", "locality"]);
    assert_eq!(origins[0].1.owner(), &site_owner());
    assert_eq!(origins[0].1.local_index(), 0);
    assert_eq!(origins[1].1.owner(), &site_owner());
    assert_eq!(origins[1].1.local_index(), 0);

    // Events run in direct-ancestry then local declaration order.
    let events: Vec<(ResolutionOperation, Option<usize>)> = record
        .resolution_events()
        .iter()
        .map(|event| (event.operation(), event.local_index()))
        .collect();
    assert_eq!(
        events,
        vec![
            (ResolutionOperation::Added, Some(0)),
            (ResolutionOperation::Added, Some(1)),
            (ResolutionOperation::Inherited, None),
            (ResolutionOperation::Added, Some(0)),
        ]
    );
    assert_eq!(record.resolution_events()[0].owner(), &entity_owner());
    assert_eq!(record.resolution_events()[2].owner(), &site_owner());
    assert_eq!(record.resolution_events()[3].owner(), &site_owner());

    // A non-location property carries no field origins at all.
    let graphic = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![auth(background("bg/logo.png"), Exposure::Public)]),
            )],
        )],
    )
    .expect("a valid input");
    let graphic_resolved = graphic
        .resolve(&representation_owner())
        .expect("a valid resolution");
    assert!(
        graphic_resolved.values_of(PropertyKey::GraphicLogo)[0]
            .field_origins()
            .is_none()
    );

    // Repeated resolution is value-deterministic.
    let repeated = input
        .resolve(&representation_owner())
        .expect("a valid resolution");
    assert_eq!(resolved, repeated);
}

/// `R03A-05` — effective cardinality is enforced on the full collection before any public
/// filtering: singular conflicts, the fifth background layer and the seventeenth fact refuse.
#[test]
fn r03a_05_effective_cardinality_is_enforced_before_public_filtering() {
    let two_titles = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Bali"), Exposure::Public)]),
                )],
            ),
            declared(
                representation_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Bali EN"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        two_titles
            .resolve(&representation_owner())
            .expect_err("two effective titles refuse")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // A hidden internal ancestor still counts for the singular conflict.
    let hidden_conflict = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Hidden"), Exposure::Internal)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Shown"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        hidden_conflict
            .resolve(&representation_owner())
            .expect_err("a hidden ancestor still conflicts")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // The fifth background layer refuses.
    let five_layers = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![
                        auth(background("bg/1.png"), Exposure::Public),
                        auth(background("bg/2.png"), Exposure::Public),
                        auth(background("bg/3.png"), Exposure::Public),
                        auth(background("bg/4.png"), Exposure::Public),
                    ]),
                )],
            ),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/5.png"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        five_layers
            .resolve(&representation_owner())
            .expect_err("a fifth layer refuses")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // The seventeenth location fact refuses.
    let mut sixteen = Vec::new();
    for index in 0..MAX_LOCATION_FACTS {
        sixteen.push(auth(
            location(&format!("fact-{index}"), "Denpasar"),
            Exposure::Public,
        ));
    }
    let seventeen = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(PropertyKey::Location, inherit(sixteen))],
            ),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(location("fact-extra", "Ubud"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        seventeen
            .resolve(&representation_owner())
            .expect_err("a seventeenth fact refuses")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // The separate local array bound also refuses.
    let mut over_long = Vec::new();
    for index in 0..=MAX_LOCAL_ENTRIES {
        over_long.push(auth(
            location(&format!("fact-{index}"), "Denpasar"),
            Exposure::Public,
        ));
    }
    let local_bound = DeclaredOwnerPropertiesV1::new(
        site_owner(),
        vec![(PropertyKey::Location, inherit(over_long))],
    )
    .expect_err("seventeen local entries refuse");
    assert_eq!(local_bound.kind(), PropertyRefusalKind::TooManyLocalEntries);
}

/// `R03A-06` — duplicate and malformed authored entries refuse without repair.
#[test]
fn r03a_06_duplicate_and_malformed_entries_refuse_without_repair() {
    let duplicate_source = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/shared.png"), Exposure::Public)]),
                )],
            ),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![auth(background("bg/shared.png"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        duplicate_source
            .resolve(&representation_owner())
            .expect_err("a repeated background source refuses")
            .kind(),
        PropertyResolutionRefusalKind::DuplicateGraphicSource
    );

    let duplicate_fact = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![
                    auth(location("bali-base", "Denpasar"), Exposure::Public),
                    auth(location("bali-base", "Ubud"), Exposure::Public),
                ]),
            )],
        )],
    )
    .expect("a valid input");
    assert_eq!(
        duplicate_fact
            .resolve(&representation_owner())
            .expect_err("a repeated original location identity refuses")
            .kind(),
        PropertyResolutionRefusalKind::DuplicateLocationFact
    );

    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "../escape.png")
            .expect_err("a path escape refuses")
            .kind(),
        PropertyRefusalKind::InvalidAssetReference
    );
    // `SOLO-WO-116 R02` authorized the single lowercase `svg` extension token, so this direct
    // parser assertion is corrected from "an active SVG refuses" to "a lowercase SVG reference is
    // admitted as *syntax only*". Syntax admission is not a media, decode or publication claim:
    // the V3 inspector still refuses any SVG that is not a safe static vector, and `.svg` on the
    // PNG-only favicon role still refuses. The uppercase spelling and every other refusal below
    // are unchanged.
    let admitted_svg = AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.svg")
        .expect("a lowercase svg reference is admitted");
    assert_eq!(admitted_svg.reference(), "assets/logo.svg");
    assert_eq!(admitted_svg.namespace(), AssetNamespace::Entity);
    // `SOLO-WO-113 R02` authorized the single `gif` extension token, so this direct parser
    // assertion is corrected from "a GIF refuses" to "a lowercase GIF reference is admitted";
    // every other refusal in this test is unchanged, and the uppercase spelling still refuses.
    let admitted_gif = AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.gif")
        .expect("a lowercase GIF reference is admitted by the R02 allowlist");
    assert_eq!(admitted_gif.reference(), "assets/logo.gif");
    assert_eq!(admitted_gif.namespace(), AssetNamespace::Entity);
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.GIF")
            .expect_err("an upper-case GIF extension refuses rather than being lowered")
            .kind(),
        PropertyRefusalKind::UnsupportedAssetExtension
    );
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "https://example.com/logo.png")
            .expect_err("a remote URL refuses")
            .kind(),
        PropertyRefusalKind::InvalidAssetReference
    );
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "C:/logo.png")
            .expect_err("a drive path refuses")
            .kind(),
        PropertyRefusalKind::InvalidAssetReference
    );
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo")
            .expect_err("a missing extension refuses")
            .kind(),
        PropertyRefusalKind::InvalidAssetReference
    );
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.PNG")
            .expect_err("an upper-case extension refuses rather than being lowered")
            .kind(),
        PropertyRefusalKind::UnsupportedAssetExtension
    );

    assert_eq!(
        FactIdV1::parse("Bali-1")
            .expect_err("an invalid fact identifier refuses")
            .kind(),
        PropertyRefusalKind::InvalidFactId
    );
    assert_eq!(
        FactIdV1::parse("-bali")
            .expect_err("a leading hyphen refuses")
            .kind(),
        PropertyRefusalKind::InvalidFactId
    );

    let mut fields = LocationFieldsV1::new();
    assert_eq!(
        fields
            .set_country_code("id")
            .expect_err("a lower-case country code refuses")
            .kind(),
        PropertyRefusalKind::InvalidCountryCode
    );
    assert_eq!(
        fields
            .set_country_code("IDN")
            .expect_err("a three-letter country code refuses")
            .kind(),
        PropertyRefusalKind::InvalidCountryCode
    );
    assert_eq!(
        fields
            .set_locality("")
            .expect_err("an empty locality refuses")
            .kind(),
        PropertyRefusalKind::InvalidLocationField
    );
    assert_eq!(
        fields
            .set_locality(&"x".repeat(200))
            .expect_err("an over-long locality refuses")
            .kind(),
        PropertyRefusalKind::InvalidLocationField
    );

    assert_eq!(
        CoordinatesV1::new(91.0, 0.0)
            .expect_err("an out-of-range latitude refuses")
            .kind(),
        PropertyRefusalKind::InvalidCoordinates
    );
    assert_eq!(
        CoordinatesV1::new(0.0, -181.0)
            .expect_err("an out-of-range longitude refuses")
            .kind(),
        PropertyRefusalKind::InvalidCoordinates
    );
    assert_eq!(
        CoordinatesV1::new(f64::NAN, 0.0)
            .expect_err("a non-finite coordinate refuses")
            .kind(),
        PropertyRefusalKind::InvalidCoordinates
    );

    assert_eq!(
        LocationFactV1::add(
            FactIdV1::parse("bali-base").expect("a valid fact identifier"),
            LocationKind::Premises,
            LocationFieldsV1::new(),
        )
        .expect_err("an empty field set refuses")
        .kind(),
        PropertyRefusalKind::EmptyLocationFields
    );
}

/// `R03A-07` — the public projection omits every internal value and all internal provenance,
/// and a suppressed or filtered-away value is never replaced by an ancestor.
#[test]
fn r03a_07_public_projection_omits_internal_values_and_cannot_resurrect() {
    // A hidden site title reaches the representation but never the public projection.
    let hidden = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Hidden site title"), Exposure::Internal)]),
                )],
            ),
            declared(
                representation_owner(),
                vec![(PropertyKey::SeoTitle, inherit(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let hidden_resolved = hidden
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let hidden_record = hidden_resolved
        .property(PropertyKey::SeoTitle)
        .expect("the title record");
    assert_eq!(hidden_record.values().len(), 1);
    assert_eq!(hidden_record.public_value_count(), 0);
    let hidden_projection = project_public_properties_v1(&hidden_resolved);
    assert!(
        hidden_projection
            .values_of(PropertyKey::SeoTitle)
            .is_empty()
    );
    assert!(
        !format!("{hidden_projection:?}").contains("Hidden site title"),
        "no internal value may appear in the public projection"
    );
    assert!(
        !format!("{hidden_projection:?}").contains("originLocalIndex"),
        "no internal provenance may appear in the public projection"
    );

    // A suppressed public ancestor is not resurrected.
    let suppressed = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Site title"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(PropertyKey::SeoTitle, suppress(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let suppressed_resolved = suppressed
        .resolve(&representation_owner())
        .expect("a valid resolution");
    assert!(
        suppressed_resolved
            .values_of(PropertyKey::SeoTitle)
            .is_empty()
    );
    assert!(
        project_public_properties_v1(&suppressed_resolved)
            .values_of(PropertyKey::SeoTitle)
            .is_empty()
    );

    // An explicit replacement after suppression is published in its own place.
    let replaced = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    inherit(vec![auth(title("Site title"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::SeoTitle,
                    suppress(vec![auth(title("Page title"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    let replaced_resolved = replaced
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let published = project_public_properties_v1(&replaced_resolved);
    let values = published.values_of(PropertyKey::SeoTitle);
    assert_eq!(values.len(), 1);
    assert!(
        values[0].location_fact().is_none(),
        "an SEO value is a text value, not a location fact"
    );
    assert_eq!(
        match &values[0] {
            PropertyValueV1::Text(text) => text.as_str(),
            _ => "<not text>",
        },
        "Page title"
    );
    // The internal record still knows the single surviving origin.
    assert_eq!(
        replaced_resolved.values_of(PropertyKey::SeoTitle)[0].origin_owner(),
        &page_owner()
    );
}

/// `R03A-08` — identically named Pages under different Sites, Facets and locales never borrow
/// each other's properties.
#[test]
fn r03a_08_identical_page_ids_under_different_sites_and_locales_never_mix() {
    let input = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            page_owner(),
            vec![(
                PropertyKey::ContactEmail,
                inherit(vec![auth(email("bali@example.com"), Exposure::Public)]),
            )],
        )],
    )
    .expect("a valid input");
    assert_eq!(
        input
            .resolve(&representation_owner())
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .len(),
        1
    );

    // The sibling Site's identically named Page.
    assert!(
        input
            .resolve(&jakarta_representation_owner())
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
    // Another locale of the same Page is a different representation key, but it is a member
    // of the same Page, so it inherits the Page-level value without adding one of its own.
    let indonesian = QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("bali"),
        page("home"),
        locale("id-ID"),
    );
    let indonesian_resolved = input.resolve(&indonesian).expect("a valid resolution");
    assert_eq!(
        indonesian_resolved
            .values_of(PropertyKey::ContactEmail)
            .len(),
        1
    );
    assert_eq!(
        indonesian_resolved.values_of(PropertyKey::ContactEmail)[0].origin_owner(),
        &page_owner()
    );
    // The Page itself owns the authored value; the Site above it does not inherit it.
    assert_eq!(
        input
            .resolve(&page_owner())
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .len(),
        1
    );
    assert!(
        input
            .resolve(&site_owner())
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
    // The identically named Page of the other Facet.
    let corporate = QualifiedOwnerV1::representation(
        entity_id(),
        facet("company"),
        site("corporate"),
        page("home"),
        locale("de-DE"),
    );
    assert!(
        input
            .resolve(&corporate)
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
}

/// `R03A-08` — a wrong lineage, an undeclared locale, a second Entity and a duplicated owner
/// all refuse rather than finding a sibling.
#[test]
fn r03a_08_wrong_lineage_and_second_entity_refuse() {
    let input = validate_declared_properties_v1(&graph(), Vec::new()).expect("a valid input");

    let undeclared_locale = QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("bali"),
        page("home"),
        locale("ja-JP"),
    );
    assert_eq!(
        input
            .resolve(&undeclared_locale)
            .expect_err("an undeclared locale refuses")
            .kind(),
        PropertyResolutionRefusalKind::UnknownOwner
    );

    let wrong_site =
        QualifiedOwnerV1::page(entity_id(), facet("main"), site("jakarta"), page("about"));
    assert_eq!(
        input
            .resolve(&wrong_site)
            .expect_err("a wrong lineage refuses")
            .kind(),
        PropertyResolutionRefusalKind::UnknownOwner
    );

    let second_entity =
        QualifiedOwnerV1::entity(EntityIdV1::parse("other").expect("a canonical identifier"));
    let refusal = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            second_entity,
            vec![(
                PropertyKey::ContactEmail,
                inherit(vec![auth(email("x@example.com"), Exposure::Public)]),
            )],
        )],
    )
    .expect_err("a second-Entity owner refuses");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::DeclarationOwnerUnknown
    );

    let wrong_facet = QualifiedOwnerV1::site(entity_id(), facet("company"), site("bali"));
    let refusal = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            wrong_facet,
            vec![(
                PropertyKey::ContactEmail,
                inherit(vec![auth(email("y@example.com"), Exposure::Public)]),
            )],
        )],
    )
    .expect_err("a cross-Facet owner refuses");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::DeclarationOwnerUnknown
    );

    let twice = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                page_owner(),
                vec![(PropertyKey::SeoTitle, inherit(Vec::new()))],
            ),
            declared(
                page_owner(),
                vec![(PropertyKey::SeoDescription, inherit(Vec::new()))],
            ),
        ],
    )
    .expect_err("a duplicated declaration owner refuses");
    assert_eq!(
        twice.kind(),
        PropertyResolutionRefusalKind::DuplicateDeclarationOwner
    );
}

/// `R03B-01` — converted from `R03A-10`. A Page-level `extend` of an inherited Site fact now
/// updates that one fact in place, and an `add`-only input still resolves unchanged.
#[test]
fn r03b_01_page_extension_updates_the_inherited_fact_in_place() {
    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("valid locality text");
    set.set_district("Gianyar").expect("valid district text");
    let input = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        location("bali-base", "Denpasar"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(site_owner(), "bali-base", set),
                        Exposure::Internal,
                    )]),
                )],
            ),
        ],
    )
    .expect("the accepted extend syntax is recognized, not reported as invalid");
    assert!(input.has_recognized_location_extension());

    let resolved = input
        .resolve(&representation_owner())
        .expect("a valid extension resolves");
    let facts = resolved.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1, "an extension never appends a fact");
    assert_eq!(facts[0].origin_owner(), &site_owner());
    assert_eq!(facts[0].origin_local_index(), 0);
    assert!(
        facts[0].exposure().is_public(),
        "an extension never changes exposure"
    );
    let fact = facts[0]
        .data()
        .location_fact()
        .expect("the resolved location fact");
    assert_eq!(fact.fact_id().as_str(), "bali-base");
    assert_eq!(fact.kind(), LocationKind::Association);
    let mut expected = LocationFieldsV1::new();
    expected.set_country_code("ID").expect("ID");
    expected.set_locality("Ubud").expect("Ubud");
    expected.set_district("Gianyar").expect("Gianyar");
    assert_eq!(fact.fields(), &expected);
    let origins = facts[0].field_origins().expect("field origins");
    assert_eq!(
        origins
            .iter()
            .map(|(name, origin)| (
                name.as_str(),
                origin.owner().describe(),
                origin.local_index()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("countryCode", site_owner().describe(), 0),
            ("locality", page_owner().describe(), 0),
            ("district", page_owner().describe(), 0),
        ]
    );

    let add_only = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    location("bali-base", "Denpasar"),
                    Exposure::Public,
                )]),
            )],
        )],
    )
    .expect("a valid input");
    assert!(!add_only.has_recognized_location_extension());
    let add_resolved = add_only
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let add_facts = add_resolved.values_of(PropertyKey::Location);
    assert_eq!(add_facts.len(), 1);
    let add_fact = add_facts[0]
        .data()
        .location_fact()
        .expect("a resolved location fact");
    assert_eq!(add_fact.fact_id().as_str(), "bali-base");
    assert_eq!(
        add_fact.fields().set_field_names(),
        vec!["countryCode", "locality"]
    );
}

/// `R03A-11` — resolving properties changes no identity fact, and nothing internal reaches the
/// public projection.
#[test]
fn r03a_11_property_resolution_leaves_identity_untouched() {
    let graph_before = graph();
    let input = validate_declared_properties_v1(
        &graph_before,
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::ContactEmail,
                    inherit(vec![auth(email("entity@example.com"), Exposure::Internal)]),
                )],
            ),
            declared(
                representation_owner(),
                vec![(
                    PropertyKey::ContactEmail,
                    suppress(vec![auth(email("page@example.com"), Exposure::Internal)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    let resolved = input
        .resolve(&representation_owner())
        .expect("a valid resolution");
    let projection = project_public_properties_v1(&resolved);

    // Identity facts are exactly what they were before resolution.
    assert_eq!(graph_before.entity().as_str(), "example");
    assert_eq!(graph_before.facets().len(), 2);
    assert_eq!(
        representation_owner().page_id().map(PageIdV1::as_str),
        Some("home")
    );
    assert_eq!(
        representation_owner().locale().map(LocaleKeyV1::as_str),
        Some("en-001")
    );

    // The internal replacement is the only effective value and it stays internal.
    let effective = resolved.values_of(PropertyKey::ContactEmail);
    assert_eq!(effective.len(), 1);
    assert_eq!(effective[0].exposure(), Exposure::Internal);
    assert_eq!(effective[0].origin_owner(), &representation_owner());
    assert!(projection.values_of(PropertyKey::ContactEmail).is_empty());
    assert!(
        !format!("{projection:?}").contains("page@example.com"),
        "an internal mailbox never appears in the public projection"
    );
    assert!(
        !format!("{projection:?}").contains("entity@example.com"),
        "a suppressed ancestor mailbox never reappears"
    );
}

/// `R03A-05` - whole-graph validation refuses a conflict in a Site the target never visits.
#[test]
fn r03a_05_whole_graph_refuses_a_conflict_in_an_unrelated_site() {
    let declarations = vec![
        declared(
            site_owner(),
            vec![(
                PropertyKey::SeoTitle,
                inherit(vec![auth(title("Bali"), Exposure::Public)]),
            )],
        ),
        declared(
            jakarta_site_owner(),
            vec![(
                PropertyKey::SeoTitle,
                inherit(vec![auth(title("Jakarta"), Exposure::Public)]),
            )],
        ),
        declared(
            jakarta_representation_owner(),
            vec![(
                PropertyKey::SeoTitle,
                inherit(vec![auth(title("Jakarta EN"), Exposure::Public)]),
            )],
        ),
    ];
    let input = validate_declared_properties_v1(&graph(), declarations).expect("a valid input");

    // The bali target is valid on its own: one effective title, its own direct chain only.
    assert_eq!(
        input
            .resolve(&representation_owner())
            .expect("a valid single-owner resolution")
            .values_of(PropertyKey::SeoTitle)
            .len(),
        1
    );

    // The whole graph is not: the jakarta branch carries two effective titles.
    assert_eq!(
        resolve_all_owners_v1(&input)
            .expect_err("the jakarta conflict refuses the whole graph")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );
}

/// `R03A-03` - the typed API itself refuses an explicitly empty `currentContext` array.
#[test]
fn r03a_03_explicit_empty_array_refuses_in_the_typed_api() {
    let declaration =
        PropertyDeclarationV1::new(Inheritance::Inherit, CurrentContextV1::Values(Vec::new()));
    let error = DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![(PropertyKey::ContactEmail, declaration)],
    )
    .expect_err("an explicitly empty array refuses");
    assert_eq!(error.kind(), PropertyRefusalKind::EmptyContextArray);
    assert_eq!(error.key(), Some(PropertyKey::ContactEmail));

    // The literal "none" stays valid, and stays distinct from an absent declaration.
    DeclaredOwnerPropertiesV1::new(
        page_owner(),
        vec![(PropertyKey::ContactEmail, inherit(Vec::new()))],
    )
    .expect("an explicit \"none\" is a valid declaration");
    let absent = validate_declared_properties_v1(&graph(), Vec::new()).expect("a valid input");
    assert!(absent.declaration_for(&page_owner()).is_none());
}

/// `R03A-04` - whole-graph resolution preserves each branch's own values, order and provenance.
#[test]
fn r03a_04_whole_graph_preserves_each_branch_original_values() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![auth(background("bg/entity.png"), Exposure::Public)]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![auth(background("bg/bali.png"), Exposure::Public)]),
            )],
        ),
        declared(
            jakarta_site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![auth(background("bg/jakarta.png"), Exposure::Public)]),
            )],
        ),
    ];
    let input = validate_declared_properties_v1(&graph(), declarations).expect("a valid input");
    let whole = resolve_all_owners_v1(&input).expect("the whole graph resolves");

    assert_eq!(whole.len(), input.owners().len());
    assert!(!whole.is_empty());

    let bali = whole
        .for_owner(&representation_owner())
        .expect("the bali representation resolved");
    let jakarta = whole
        .for_owner(&jakarta_representation_owner())
        .expect("the jakarta representation resolved");
    assert_eq!(
        asset_references(bali.values_of(PropertyKey::GraphicBackground)),
        vec!["bg/entity.png", "bg/bali.png"]
    );
    assert_eq!(
        asset_references(jakarta.values_of(PropertyKey::GraphicBackground)),
        vec!["bg/entity.png", "bg/jakarta.png"]
    );
    assert_eq!(
        bali.values_of(PropertyKey::GraphicBackground)[1].origin_owner(),
        &site_owner()
    );
    assert_eq!(
        jakarta.values_of(PropertyKey::GraphicBackground)[1].origin_owner(),
        &jakarta_site_owner()
    );
    assert_eq!(
        bali.values_of(PropertyKey::GraphicBackground)[1].origin_local_index(),
        0
    );

    // The traversal is deterministic.
    assert_eq!(
        whole,
        resolve_all_owners_v1(&input).expect("a repeated whole-graph resolution")
    );
}

/// `R03B-04` — converted from `R03A-10`. A sibling-targeted extension still refuses the whole
/// candidate, now with the R03.B code and typed cause, while the valid Bali branch is untouched.
#[test]
fn r03b_04_sibling_extension_refuses_the_whole_graph() {
    let declarations = vec![
        declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    location("bali-base", "Denpasar"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            jakarta_site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    extension_request("bali-base", "Ubud"),
                    Exposure::Internal,
                )]),
            )],
        ),
    ];
    let input = validate_declared_properties_v1(&graph(), declarations)
        .expect("the accepted extend syntax is recognized, not invalid");
    assert!(input.has_recognized_location_extension());

    // The bali target alone is unaffected by the jakarta request.
    assert_eq!(
        input
            .resolve(&representation_owner())
            .expect("a valid single-owner resolution")
            .values_of(PropertyKey::Location)
            .len(),
        1
    );

    let refusal = resolve_all_owners_v1(&input).expect_err("the extension refuses the whole graph");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::InvalidLocationExtension
    );
    assert_eq!(refusal.code(), "LOCATION_INVALID_EXTEND");
    assert_eq!(
        refusal.cause(),
        Some(LocationExtensionCause::TargetNotAncestor)
    );
    assert_eq!(
        refusal.owner().map(solo_domain::QualifiedOwnerV1::describe),
        Some(jakarta_site_owner().describe()),
        "the diagnostic names the declaring node, not the requested target"
    );
    assert_eq!(refusal.property(), Some(PropertyKey::Location));
}

/// Every resolved field origin of one location value as `(field, owner, local index)`.
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

/// `R03B-02` — consecutive extensions of one original fact preserve untouched fields and their
/// earlier origins while updating only newly-set fields, across all five EFS levels.
#[test]
fn r03b_02_consecutive_extensions_overlay_all_ten_fields_across_five_levels() {
    let mut entity_fields = LocationFieldsV1::new();
    entity_fields.set_world_region("Oceania").expect("region");
    entity_fields.set_country_code("AU").expect("AU");
    let hq = PropertyValueV1::location(
        LocationFactV1::add(
            FactIdV1::parse("hq").expect("hq"),
            LocationKind::Premises,
            entity_fields,
        )
        .expect("a nonempty fact"),
    );

    let mut facet_set = LocationFieldsV1::new();
    facet_set.set_administrative_area("Bali").expect("area");
    let mut site_set = LocationFieldsV1::new();
    site_set.set_locality("Denpasar").expect("locality");
    let mut page_set = LocationFieldsV1::new();
    page_set.set_district("Denpasar Barat").expect("district");
    page_set.set_named_place("Renon").expect("named place");
    let mut representation_set = LocationFieldsV1::new();
    representation_set
        .set_address_line1("Jalan Raya 1")
        .expect("line 1");
    representation_set
        .set_address_line2("Suite 2")
        .expect("line 2");
    representation_set
        .set_postal_code("80226")
        .expect("postal code");
    representation_set.set_coordinates(CoordinatesV1::new(-8.65, 115.22).expect("WGS84"));

    let extensions = vec![
        (facet_owner(), facet_set),
        (site_owner(), site_set),
        (page_owner(), page_set),
        (representation_owner(), representation_set),
    ];
    let mut declarations = vec![declared(
        entity_owner(),
        vec![(
            PropertyKey::Location,
            inherit(vec![auth(hq, Exposure::Public)]),
        )],
    )];
    for (owner, set) in extensions {
        declarations.push(declared(
            owner,
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    extension_to(entity_owner(), "hq", set),
                    Exposure::Internal,
                )]),
            )],
        ));
    }
    let input = validate_declared_properties_v1(&graph(), declarations)
        .expect("every extension targets a strict ancestor's inherited fact");

    let resolved = input
        .resolve(&representation_owner())
        .expect("the five-level chain resolves");
    let facts = resolved.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1, "one original fact, never a second");
    assert_eq!(facts[0].origin_owner(), &entity_owner());
    assert_eq!(facts[0].origin_local_index(), 0);
    assert!(facts[0].exposure().is_public());
    let fact = facts[0].data().location_fact().expect("the fact");
    assert_eq!(fact.fact_id().as_str(), "hq");
    assert_eq!(fact.kind(), LocationKind::Premises);
    assert_eq!(
        fact.fields().set_field_names(),
        solo_domain::LOCATION_FIELD_NAMES.to_vec(),
        "all ten members are present in canonical order"
    );
    assert_eq!(
        origins_of(&facts[0]),
        vec![
            ("worldRegion".to_owned(), entity_owner().describe(), 0),
            ("countryCode".to_owned(), entity_owner().describe(), 0),
            ("administrativeArea".to_owned(), facet_owner().describe(), 0),
            ("locality".to_owned(), site_owner().describe(), 0),
            ("district".to_owned(), page_owner().describe(), 0),
            ("namedPlace".to_owned(), page_owner().describe(), 0),
            (
                "addressLine1".to_owned(),
                representation_owner().describe(),
                0
            ),
            (
                "addressLine2".to_owned(),
                representation_owner().describe(),
                0
            ),
            (
                "postalCode".to_owned(),
                representation_owner().describe(),
                0
            ),
            (
                "coordinates".to_owned(),
                representation_owner().describe(),
                0
            ),
        ],
        "only explicitly set members move their origin; coordinates is one field"
    );
    let events = resolved
        .property(PropertyKey::Location)
        .expect("a location record")
        .resolution_events();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.operation() == ResolutionOperation::Extended)
            .count(),
        4,
        "one `extended` event per successful extension"
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.operation() == ResolutionOperation::Added)
            .count(),
        1,
        "an extension never records a phantom `added` event"
    );
}

/// `R03B-03` — mixed `add`, `extend`, `add` keeps inherited-first then authored order, records
/// the real sequence, repeats identically and keeps `none` distinct from an absent key.
#[test]
fn r03b_03_mixed_add_extend_add_keeps_order_and_records_the_real_sequence() {
    fn added(fact_id: &str, locality: &str) -> PropertyValueV1 {
        let mut fields = LocationFieldsV1::new();
        fields.set_locality(locality).expect("locality");
        PropertyValueV1::location(
            LocationFactV1::add(
                FactIdV1::parse(fact_id).expect("a fact identifier"),
                LocationKind::Association,
                fields,
            )
            .expect("a nonempty fact"),
        )
    }
    let mut ubud = LocationFieldsV1::new();
    ubud.set_locality("Ubud").expect("locality");

    let input = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        location("bali-base", "Denpasar"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![
                        auth(added("page-annex", "Sanur"), Exposure::Public),
                        auth(
                            extension_to(site_owner(), "bali-base", ubud),
                            Exposure::Internal,
                        ),
                        auth(added("page-depot", "Jimbaran"), Exposure::Internal),
                    ]),
                )],
            ),
        ],
    )
    .expect("the mixed declarations are valid");

    let resolved = input
        .resolve(&representation_owner())
        .expect("the mixed declaration resolves");
    let facts = resolved.values_of(PropertyKey::Location);
    assert_eq!(
        facts.len(),
        3,
        "two adds and no extra fact from the extension"
    );
    assert_eq!(
        facts
            .iter()
            .map(|value| (value.origin_owner().describe(), value.origin_local_index()))
            .collect::<Vec<_>>(),
        vec![
            (site_owner().describe(), 0),
            (page_owner().describe(), 0),
            (page_owner().describe(), 2),
        ],
        "the inherited fact stays first and keeps its slot; adds keep authored order"
    );
    assert_eq!(
        origins_of(&facts[0])[1],
        ("locality".to_owned(), page_owner().describe(), 1),
        "a field origin records the extension's own zero-based local index"
    );
    let mut extended_fields = LocationFieldsV1::new();
    extended_fields.set_country_code("ID").expect("ID");
    extended_fields.set_locality("Ubud").expect("Ubud");
    assert_eq!(
        facts[0].data().location_fact().expect("a fact").fields(),
        &extended_fields,
        "the extension updated the inherited fact in place"
    );

    let sequence = resolved
        .property(PropertyKey::Location)
        .expect("a location record")
        .resolution_events()
        .iter()
        .map(|event| {
            (
                event.owner().describe(),
                event.operation().detail(),
                event.local_index(),
                event.target_fact_id().map(ToString::to_string),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        sequence,
        vec![
            (site_owner().describe(), "added", Some(0), None),
            (page_owner().describe(), "inherited", None, None),
            (page_owner().describe(), "added", Some(0), None),
            (
                page_owner().describe(),
                "extended",
                Some(1),
                Some("bali-base".to_owned())
            ),
            (page_owner().describe(), "added", Some(2), None),
        ]
    );

    // Repeated resolution is value-identical: no memoised mutation survives between calls.
    assert_eq!(
        input
            .resolve(&representation_owner())
            .expect("a repeated resolution"),
        resolved
    );

    // An explicit `"none"` declares the key and contributes nothing; an absent key declares
    // nothing at all. Both resolve identically, so the distinction is authored, not effective.
    let literal_none = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            page_owner(),
            vec![(PropertyKey::Location, inherit(Vec::new()))],
        )],
    )
    .expect("an explicit `none` is valid");
    let none_resolved = literal_none
        .resolve(&page_owner())
        .expect("`none` resolves");
    assert!(none_resolved.values_of(PropertyKey::Location).is_empty());
    assert!(
        none_resolved
            .property(PropertyKey::Location)
            .expect("a location record")
            .resolution_events()
            .is_empty(),
        "`none` carries nothing and records no inheritance event"
    );
    let absent = validate_declared_properties_v1(&graph(), Vec::new()).expect("a valid input");
    let absent_resolved = absent
        .resolve(&page_owner())
        .expect("an absent key resolves");
    assert_eq!(absent_resolved, none_resolved);
    assert!(
        literal_none
            .declaration_for(&page_owner())
            .expect("a declaration set")
            .declaration(PropertyKey::Location)
            .is_some(),
        "the explicit `none` declaration still exists"
    );
    assert!(absent.declaration_for(&page_owner()).is_none());
}

/// `R03B-04` — every forbidden extension target refuses with `LOCATION_INVALID_EXTEND` and its
/// typed cause, and never falls back to another fact or the requested target.
#[test]
fn r03b_04_invalid_extension_targets_refuse_with_typed_causes() {
    fn refusal(
        declarations: Vec<DeclaredOwnerPropertiesV1>,
        target: &QualifiedOwnerV1,
    ) -> PropertyResolutionRefusalV1 {
        validate_declared_properties_v1(&graph(), declarations)
            .expect("a structurally valid declaration set")
            .resolve(target)
            .expect_err("the extension must refuse")
    }

    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("locality");
    let other_entity = QualifiedOwnerV1::entity(EntityIdV1::parse("other").expect("other"));
    let company_facet = QualifiedOwnerV1::facet(entity_id(), facet("company"));
    let id_representation = QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("bali"),
        page("home"),
        locale("id-ID"),
    );
    let extension_declaration =
        |declaring: QualifiedOwnerV1, target: QualifiedOwnerV1, fact_id: &str| {
            declared(
                declaring,
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(target, fact_id, set.clone()),
                        Exposure::Internal,
                    )]),
                )],
            )
        };

    // Forbidden lineage: the Entity root, the current node, a sibling, another Facet, another
    // Entity, a descendant and a different full locale fail before any fact lookup.
    let lineage_cases = vec![
        (entity_owner(), entity_owner(), "hq"),
        (page_owner(), page_owner(), "bali-base"),
        (page_owner(), jakarta_site_owner(), "jakarta-base"),
        (page_owner(), company_facet, "corporate-base"),
        (page_owner(), other_entity, "hq"),
        (site_owner(), page_owner(), "bali-base"),
        (page_owner(), id_representation, "bali-base"),
    ];
    for (declaring, target, fact_id) in lineage_cases {
        let failure = refusal(
            vec![extension_declaration(declaring, target.clone(), fact_id)],
            &page_owner(),
        );
        assert_eq!(
            failure.kind(),
            PropertyResolutionRefusalKind::InvalidLocationExtension
        );
        assert_eq!(failure.code(), "LOCATION_INVALID_EXTEND");
        assert_eq!(
            failure.cause(),
            Some(LocationExtensionCause::TargetNotAncestor),
            "target {} must be a forbidden lineage",
            target.describe()
        );
    }

    // A missing fact, and an ancestor named with the right identifier but not the original
    // owner, both refuse as not-inherited rather than matching another identity.
    let site_fact = declared(
        site_owner(),
        vec![(
            PropertyKey::Location,
            inherit(vec![auth(
                location("bali-base", "Denpasar"),
                Exposure::Public,
            )]),
        )],
    );
    let missing = refusal(
        vec![
            site_fact.clone(),
            extension_declaration(page_owner(), site_owner(), "nope"),
        ],
        &representation_owner(),
    );
    assert_eq!(
        missing.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
    );
    let wrong_original_owner = refusal(
        vec![
            site_fact,
            extension_declaration(page_owner(), entity_owner(), "bali-base"),
        ],
        &representation_owner(),
    );
    assert_eq!(
        wrong_original_owner.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
    );
    assert_eq!(wrong_original_owner.code(), "LOCATION_INVALID_EXTEND");

    // A fact removed by an intervening ancestor is no longer inherited and cannot be extended.
    let removed = refusal(
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(location("shared", "Denpasar"), Exposure::Public)]),
                )],
            ),
            declared(
                site_owner(),
                vec![(PropertyKey::Location, suppress(Vec::new()))],
            ),
            extension_declaration(page_owner(), entity_owner(), "shared"),
        ],
        &representation_owner(),
    );
    assert_eq!(
        removed.cause(),
        Some(LocationExtensionCause::TargetNotInherited)
    );
}

/// `R03B-05` — suppression is final: a suppressed target refuses, `suppress` with a new `add`
/// is valid, and a fact added at the same node is not an eligible target.
#[test]
fn r03b_05_suppression_is_final_for_extensions() {
    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("locality");
    let entity_fact = declared(
        entity_owner(),
        vec![(
            PropertyKey::Location,
            inherit(vec![auth(location("shared", "Denpasar"), Exposure::Public)]),
        )],
    );

    // The same node both suppresses and names the removed fact: the accurate cause is
    // suppression, not a generic missing fact.
    let suppressed = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_fact.clone(),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    suppress(vec![auth(
                        extension_to(entity_owner(), "shared", set.clone()),
                        Exposure::Internal,
                    )]),
                )],
            ),
        ],
    )
    .expect("a structurally valid declaration set")
    .resolve(&site_owner())
    .expect_err("a suppressed target cannot be extended");
    assert_eq!(
        suppressed.cause(),
        Some(LocationExtensionCause::TargetSuppressed)
    );
    assert_eq!(suppressed.code(), "LOCATION_INVALID_EXTEND");

    // `suppress` plus an independent `add` stays valid and clears the suppressed value.
    let replaced = validate_declared_properties_v1(
        &graph(),
        vec![
            entity_fact,
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    suppress(vec![auth(
                        location("bali-new", "Denpasar"),
                        Exposure::Public,
                    )]),
                )],
            ),
        ],
    )
    .expect("suppress plus add is valid")
    .resolve(&representation_owner())
    .expect("the replacement resolves");
    let facts = replaced.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].origin_owner(), &site_owner());
    assert_eq!(
        facts[0]
            .data()
            .location_fact()
            .expect("a fact")
            .fact_id()
            .as_str(),
        "bali-new"
    );
    assert!(
        replaced
            .property(PropertyKey::Location)
            .expect("a record")
            .resolution_events()
            .iter()
            .any(|event| event.operation() == ResolutionOperation::Suppressed)
    );

    // A fact the same node has just added is not an eligible extension target. Naming it means
    // naming the current node, which the direct-ancestor rule refuses before any identity match;
    // the resolver's inherited-prefix guard enforces the same §6.2H.5 rule structurally.
    let same_node = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            page_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![
                    auth(location("page-annex", "Sanur"), Exposure::Public),
                    auth(
                        extension_to(page_owner(), "page-annex", set),
                        Exposure::Internal,
                    ),
                ]),
            )],
        )],
    )
    .expect("a structurally valid declaration set")
    .resolve(&page_owner())
    .expect_err("a same-node add is not extendable");
    assert_eq!(
        same_node.cause(),
        Some(LocationExtensionCause::TargetNotAncestor)
    );
}

/// `R03B-06` — two extensions of one original fact at one node refuse even when identical, and
/// two inherited facts that share a `factId` under different original owners stay distinct.
#[test]
fn r03b_06_duplicate_extensions_refuse_and_same_ids_stay_distinguishable() {
    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("locality");
    let site_fact = declared(
        site_owner(),
        vec![(
            PropertyKey::Location,
            inherit(vec![auth(
                location("bali-base", "Denpasar"),
                Exposure::Public,
            )]),
        )],
    );
    let repeated = validate_declared_properties_v1(
        &graph(),
        vec![
            site_fact,
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![
                        auth(
                            extension_to(site_owner(), "bali-base", set.clone()),
                            Exposure::Internal,
                        ),
                        auth(
                            extension_to(site_owner(), "bali-base", set.clone()),
                            Exposure::Internal,
                        ),
                    ]),
                )],
            ),
        ],
    )
    .expect("a structurally valid declaration set")
    .resolve(&page_owner())
    .expect_err("two extensions of one target refuse");
    assert_eq!(
        repeated.cause(),
        Some(LocationExtensionCause::DuplicateExtensionTarget)
    );
    assert_eq!(repeated.code(), "LOCATION_INVALID_EXTEND");

    let distinct = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(location("shared", "Denpasar"), Exposure::Public)]),
                )],
            ),
            declared(
                facet_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(location("shared", "Denpasar"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(facet_owner(), "shared", set),
                        Exposure::Internal,
                    )]),
                )],
            ),
        ],
    )
    .expect("distinct original owners may reuse one fact identifier")
    .resolve(&representation_owner())
    .expect("the qualified target resolves");
    let facts = distinct.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 2, "no silent de-duplication");
    assert_eq!(facts[0].origin_owner(), &entity_owner());
    assert_eq!(facts[1].origin_owner(), &facet_owner());
    let mut untouched = LocationFieldsV1::new();
    untouched.set_country_code("ID").expect("ID");
    untouched.set_locality("Denpasar").expect("Denpasar");
    assert_eq!(
        facts[0].data().location_fact().expect("a fact").fields(),
        &untouched,
        "the entity-owned fact is untouched"
    );
    let mut extended = LocationFieldsV1::new();
    extended.set_country_code("ID").expect("ID");
    extended.set_locality("Ubud").expect("Ubud");
    assert_eq!(
        facts[1].data().location_fact().expect("a fact").fields(),
        &extended,
        "only the named original owner changes"
    );
}

/// `R03B-07` — the effective maximum stays 16 distinct facts: a valid extension is never a
/// seventeenth fact, a seventeenth independent `add` refuses before public filtering, and a
/// duplicate independent `add` identity still refuses.
#[test]
fn r03b_07_effective_maximum_and_duplicate_add_refusals_are_unchanged() {
    let site_facts: Vec<AuthoredPropertyValueV1> = (0..MAX_LOCATION_FACTS)
        .map(|index| {
            auth(
                location(&format!("f-{index}"), "Denpasar"),
                Exposure::Public,
            )
        })
        .collect();
    let site_declaration = declared(
        site_owner(),
        vec![(PropertyKey::Location, inherit(site_facts))],
    );
    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("locality");

    let extended = validate_declared_properties_v1(
        &graph(),
        vec![
            site_declaration.clone(),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(site_owner(), "f-0", set.clone()),
                        Exposure::Internal,
                    )]),
                )],
            ),
        ],
    )
    .expect("the extension's target is inherited")
    .resolve(&representation_owner())
    .expect("a valid extension at the effective maximum resolves");
    assert_eq!(
        extended.values_of(PropertyKey::Location).len(),
        MAX_LOCATION_FACTS,
        "an extension is not a seventeenth fact"
    );

    // A seventeenth fact refuses even though the extra value is `internal` and would never be
    // publicly projected.
    let seventeenth = validate_declared_properties_v1(
        &graph(),
        vec![
            site_declaration,
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(location("extra", "Sanur"), Exposure::Internal)]),
                )],
            ),
        ],
    )
    .expect("a structurally valid declaration set")
    .resolve(&page_owner())
    .expect_err("seventeen effective facts refuse");
    assert_eq!(
        seventeenth.kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // A duplicate independent `add` at one owner still refuses by identity.
    let duplicate = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![
                    auth(location("dup", "Denpasar"), Exposure::Public),
                    auth(location("dup", "Sanur"), Exposure::Public),
                ]),
            )],
        )],
    )
    .expect("a structurally valid declaration set")
    .resolve(&site_owner())
    .expect_err("a repeated identity refuses");
    assert_eq!(
        duplicate.kind(),
        PropertyResolutionRefusalKind::DuplicateLocationFact
    );
}

/// `R03B-08` — the validated typed boundary refuses an empty extension `set` and a public
/// extension wrapper, so a typed caller cannot express either forbidden state.
#[test]
fn r03b_08_typed_boundary_refuses_empty_set_and_public_wrapper() {
    let fact_id = FactIdV1::parse("bali-base").expect("a fact identifier");
    let empty =
        LocationExtensionRequestV1::new(fact_id.clone(), site_owner(), LocationFieldsV1::new())
            .expect_err("an empty set can never be an extension");
    assert_eq!(empty.kind(), PropertyRefusalKind::EmptyLocationFields);

    let mut set = LocationFieldsV1::new();
    set.set_locality("Ubud").expect("locality");
    let request =
        LocationExtensionRequestV1::new(fact_id, site_owner(), set).expect("a typed request");
    let public = AuthoredPropertyValueV1::new(
        PropertyValueV1::location_extension(request.clone()),
        Exposure::Public,
    )
    .expect_err("an extension may not claim a public exposure");
    assert_eq!(
        public.kind(),
        PropertyRefusalKind::ExtensionExposureForbidden
    );
    let internal = AuthoredPropertyValueV1::new(
        PropertyValueV1::location_extension(request),
        Exposure::Internal,
    )
    .expect("the inert internal placeholder is allowed");
    assert_eq!(internal.exposure(), Exposure::Internal);
}

/// `R03B-09` — an internal inherited fact extended at Page and representation stays internal,
/// the public projection omits it and all provenance, public sibling facts stay in their own
/// branch, and a refusal never echoes authored content.
#[test]
fn r03b_09_internal_facts_stay_unprojected_after_extension() {
    let mut page_set = LocationFieldsV1::new();
    page_set.set_locality("Ubud").expect("locality");
    let mut representation_set = LocationFieldsV1::new();
    representation_set.set_district("Renon").expect("district");

    let input = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        location("secret", "Denpasar"),
                        Exposure::Internal,
                    )]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(site_owner(), "secret", page_set),
                        Exposure::Internal,
                    )]),
                )],
            ),
            declared(
                representation_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        extension_to(site_owner(), "secret", representation_set),
                        Exposure::Internal,
                    )]),
                )],
            ),
            declared(
                jakarta_site_owner(),
                vec![(
                    PropertyKey::Location,
                    inherit(vec![auth(
                        location("jakarta-base", "Jakarta"),
                        Exposure::Public,
                    )]),
                )],
            ),
        ],
    )
    .expect("the declarations are valid");

    let resolved = input
        .resolve(&representation_owner())
        .expect("the internal chain resolves");
    let facts = resolved.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1);
    assert_eq!(
        facts[0].exposure(),
        Exposure::Internal,
        "an extension never elevates a whole-record exposure"
    );
    let fact = facts[0].data().location_fact().expect("a fact");
    assert!(
        fact.fields().set_field_names().contains(&"district"),
        "both extensions were applied"
    );
    let projection = project_public_properties_v1(&resolved);
    assert!(
        projection.values_of(PropertyKey::Location).is_empty(),
        "an internal fact never reaches the public projection"
    );
    assert!(
        projection
            .property(PropertyKey::Location)
            .expect("a projected record")
            .is_empty()
    );

    // The public sibling branch keeps only its own fact and its own projection.
    let jakarta = input
        .resolve(&jakarta_representation_owner())
        .expect("the sibling branch resolves");
    assert_eq!(jakarta.values_of(PropertyKey::Location).len(), 1);
    assert_eq!(
        jakarta.values_of(PropertyKey::Location)[0].origin_owner(),
        &jakarta_site_owner()
    );
    assert_eq!(
        project_public_properties_v1(&jakarta)
            .values_of(PropertyKey::Location)
            .len(),
        1
    );

    // A refusal is safe: it names the code and structural identifiers, never authored content.
    let mut leaked = LocationFieldsV1::new();
    leaked.set_locality("Ubud").expect("locality");
    let refusal = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            page_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    extension_to(site_owner(), "missing", leaked),
                    Exposure::Internal,
                )]),
            )],
        )],
    )
    .expect("a structurally valid declaration set")
    .resolve(&page_owner())
    .expect_err("a missing target refuses");
    let rendered = format!("{refusal}");
    assert!(rendered.contains("LOCATION_INVALID_EXTEND"));
    assert!(!rendered.contains("Ubud"), "no authored value is echoed");
    assert!(
        !rendered.contains("Denpasar"),
        "no inherited value is echoed"
    );
}

/// One validated graphic reference value under an explicit namespace.
fn graphic_ref(namespace: AssetNamespace, reference: &str) -> PropertyValueV1 {
    PropertyValueV1::graphic(
        AssetReferenceV1::parse(namespace, reference).expect("a valid asset reference"),
    )
}

/// One Entity-namespace logo reference.
fn logo(reference: &str) -> PropertyValueV1 {
    graphic_ref(AssetNamespace::Entity, reference)
}

/// `R03C1-07` — the accepted domain graphic gates are unchanged: singular cardinality, the
/// four-layer background bound, duplicate exact background sources, `suppress` plus replacement,
/// explicit suppression and internal filtering all still behave exactly as accepted, and the
/// cardinality check runs **before** the public filtering so an internal value still conflicts.
#[test]
fn r03c1_07_graphic_cardinality_duplicates_and_suppression_are_unchanged() {
    // A singular role refuses a second effective value inherited from a direct ancestor.
    let singular = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/site.png"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/page.png"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        singular
            .resolve(&representation_owner())
            .expect_err("a second effective logo refuses")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // The same conflict is still refused when the ancestor value is internal: the maximum is
    // enforced before any public filtering.
    let hidden = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/hidden.png"), Exposure::Internal)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/shown.png"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    assert_eq!(
        hidden
            .resolve(&representation_owner())
            .expect_err("a hidden ancestor still conflicts")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // The background bound is exactly four layers.
    let mut five_layers = Vec::new();
    for index in 0..=MAX_BACKGROUND_LAYERS {
        five_layers.push(auth(
            background(&format!("bg/layer-{index}.png")),
            Exposure::Public,
        ));
    }
    assert_eq!(five_layers.len(), MAX_BACKGROUND_LAYERS + 1);
    let excessive = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            entity_owner(),
            vec![(PropertyKey::GraphicBackground, inherit(five_layers))],
        )],
    )
    .expect("a valid local bound");
    assert_eq!(
        excessive
            .resolve(&page_owner())
            .expect_err("five background layers refuse")
            .kind(),
        PropertyResolutionRefusalKind::CardinalityExceeded
    );

    // Two identical background sources are refused by their exact category.
    let duplicate = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![
                    auth(background("bg/same.png"), Exposure::Public),
                    auth(background("bg/same.png"), Exposure::Internal),
                ]),
            )],
        )],
    )
    .expect("a valid local bound");
    assert_eq!(
        duplicate
            .resolve(&page_owner())
            .expect_err("a repeated exact background source refuses")
            .kind(),
        PropertyResolutionRefusalKind::DuplicateGraphicSource
    );
}

/// `R03C1-07` — `suppress` plus a local graphic replaces a singular inherited selection, `suppress`
/// alone leaves nothing effective, and an internal value is kept internally while staying out of
/// the public projection.
#[test]
fn r03c1_07_graphic_suppression_and_internal_filtering_are_unchanged() {
    let replaced = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/entity.png"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    suppress(vec![auth(logo("brand/page.png"), Exposure::Public)]),
                )],
            ),
        ],
    )
    .expect("a valid input");
    let resolved = replaced
        .resolve(&representation_owner())
        .expect("suppress plus replacement is legal");
    let values = resolved.values_of(PropertyKey::GraphicLogo);
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].origin_owner(), &page_owner());
    assert_eq!(values[0].origin_local_index(), 0);
    assert_eq!(
        asset_references(values),
        vec!["brand/page.png".to_owned()],
        "the inherited entity logo is replaced, never merged"
    );
    assert_eq!(
        project_public_properties_v1(&resolved)
            .values_of(PropertyKey::GraphicLogo)
            .len(),
        1
    );

    // `suppress` with `"none"` clears the collection at that descendant only.
    let cleared = validate_declared_properties_v1(
        &graph(),
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/entity.png"), Exposure::Public)]),
                )],
            ),
            declared(
                page_owner(),
                vec![(PropertyKey::GraphicLogo, suppress(Vec::new()))],
            ),
        ],
    )
    .expect("a valid input");
    let cleared_resolved = cleared
        .resolve(&representation_owner())
        .expect("explicit suppression resolves");
    assert!(
        cleared_resolved
            .values_of(PropertyKey::GraphicLogo)
            .is_empty()
    );
    assert!(
        project_public_properties_v1(&cleared_resolved)
            .values_of(PropertyKey::GraphicLogo)
            .is_empty()
    );
    // An unrelated sibling branch still sees the inherited value: no global side effect.
    let sibling = cleared
        .resolve(&jakarta_representation_owner())
        .expect("the sibling branch resolves");
    assert_eq!(sibling.values_of(PropertyKey::GraphicLogo).len(), 1);

    // Internal values stay in the internal record and never reach the public projection.
    let internal = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![auth(background("bg/hidden.png"), Exposure::Internal)]),
            )],
        )],
    )
    .expect("a valid input");
    let internal_resolved = internal
        .resolve(&representation_owner())
        .expect("an internal value resolves");
    assert_eq!(
        internal_resolved
            .values_of(PropertyKey::GraphicBackground)
            .len(),
        1
    );
    assert!(
        project_public_properties_v1(&internal_resolved)
            .values_of(PropertyKey::GraphicBackground)
            .is_empty()
    );
}

/// `R03C1-02`/`R03C1-07` — each of the five registered graphic roles keeps its exact original
/// owner, local index, namespace and reference through resolution, with no renamed role and no
/// guessed filename.
#[test]
fn r03c1_07_five_registered_graphic_roles_keep_exact_origin() {
    let validated = validate_declared_properties_v1(
        &graph(),
        vec![declared(
            page_owner(),
            vec![
                (
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(logo("brand/logo.png"), Exposure::Public)]),
                ),
                (
                    PropertyKey::GraphicFavicon,
                    inherit(vec![auth(logo("icons/favicon.png"), Exposure::Public)]),
                ),
                (
                    PropertyKey::GraphicBanner,
                    inherit(vec![auth(logo("brand/banner.png"), Exposure::Internal)]),
                ),
                (
                    PropertyKey::GraphicBackground,
                    inherit(vec![
                        auth(
                            graphic_ref(AssetNamespace::Platform, "shared/bg-1.webp"),
                            Exposure::Public,
                        ),
                        auth(logo("bg/bg-2.jpg"), Exposure::Public),
                    ]),
                ),
                (
                    PropertyKey::GraphicSocialPreview,
                    inherit(vec![auth(logo("social/preview.jpeg"), Exposure::Public)]),
                ),
            ],
        )],
    )
    .expect("a valid input");
    let resolved = validated
        .resolve(&representation_owner())
        .expect("six authored values resolve");
    let public = project_public_properties_v1(&resolved);

    let expected: Vec<(PropertyKey, usize, &str, AssetNamespace)> = vec![
        (
            PropertyKey::GraphicLogo,
            0,
            "brand/logo.png",
            AssetNamespace::Entity,
        ),
        (
            PropertyKey::GraphicFavicon,
            0,
            "icons/favicon.png",
            AssetNamespace::Entity,
        ),
        (
            PropertyKey::GraphicBanner,
            0,
            "brand/banner.png",
            AssetNamespace::Entity,
        ),
        (
            PropertyKey::GraphicBackground,
            0,
            "shared/bg-1.webp",
            AssetNamespace::Platform,
        ),
        (
            PropertyKey::GraphicBackground,
            1,
            "bg/bg-2.jpg",
            AssetNamespace::Entity,
        ),
        (
            PropertyKey::GraphicSocialPreview,
            0,
            "social/preview.jpeg",
            AssetNamespace::Entity,
        ),
    ];
    for (role, index, reference, namespace) in expected {
        let values = resolved.values_of(role);
        let value = values
            .iter()
            .find(|value| value.origin_local_index() == index)
            .unwrap_or_else(|| panic!("{role} local index {index} resolved"));
        assert_eq!(value.origin_owner(), &page_owner());
        let reference_value = value.data().asset_reference().expect("a graphic reference");
        assert_eq!(reference_value.reference(), reference);
        assert_eq!(reference_value.namespace(), namespace);
    }

    // The internal banner is the only value withheld from the public projection.
    assert!(public.values_of(PropertyKey::GraphicBanner).is_empty());
    assert_eq!(public.values_of(PropertyKey::GraphicLogo).len(), 1);
    assert_eq!(public.values_of(PropertyKey::GraphicFavicon).len(), 1);
    assert_eq!(public.values_of(PropertyKey::GraphicBackground).len(), 2);
    assert_eq!(public.values_of(PropertyKey::GraphicSocialPreview).len(), 1);

    // Exactly the five registered graphic keys are classified as graphic roles.
    let graphic_keys: Vec<&str> = PropertyKey::ALL
        .iter()
        .filter(|key| key.is_graphic())
        .map(|key| key.name())
        .collect();
    assert_eq!(
        graphic_keys,
        vec![
            "graphic.logo",
            "graphic.favicon",
            "graphic.banner",
            "graphic.background",
            "graphic.social-preview",
        ]
    );
}

/// `SOLO-WO-118 R17` — focused N128 matrix for the governed Platform-first graphic cascade.
///
/// The risk under test is **how a root baseline affects inherited selection, privacy and source
/// identity**, not whether an already validated SVG can be decoded again. The fixtures therefore use
/// a small verified approved set with the real approved catalogue digest, synthetic *authored*
/// references for state transitions, and the real graph helpers above for EFS lineage.
mod n128_domain {
    use super::*;
    use solo_domain::{
        ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1, GRAPHIC_PRIMARY_SLOT_V1,
        GraphicCascadeInputV1, GraphicCascadeRefusalKind, GraphicLineageOperationV1,
        GraphicSlotIdV1, GraphicSlotSelectionV1, GraphicSourceKindV1, OwnerGraphicSelectionsV1,
        PlatformAssetMediaClassV1, PlatformGraphicBaselineRecordV1, PlatformGraphicBaselineV1,
        ResolvedGraphicOwnerV1, project_public_graphic_slots_v1,
    };

    /// The real approved catalogue digest of library `1.0.0`.
    const CATALOGUE_SHA256: &str =
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";
    /// Real approved digests of four icons of the immutable `1.0.0` catalogue.
    const HOME_SHA256: &str = "cb742a3b1aa0007ec0c3f26f898e5833c6e67476e136b8eae3c52e2f2c751b5c";
    const ABOUT_SHA256: &str = "bf4377dff0aaaddf1676a4bb7292f90f1102f19daaca7021331ba6a191317c1c";
    const CLIENTS_SHA256: &str = "944697c41cc10ebada52ca069d8a6f7da5ddf96a9b6c1c6df653f33933f22053";
    const SEARCH_SHA256: &str = "57e054b6c1508be40d5eba1ea434b702f5ff55dca04e28e5e3f22472c2f6176d";

    /// The verified approved binding these tests may register defaults against.
    fn binding() -> ApprovedPlatformGraphicBindingV1 {
        let assets = vec![
            ApprovedPlatformAssetV1::new(
                "icons/icon-home.svg",
                HOME_SHA256,
                PlatformAssetMediaClassV1::Svg,
            )
            .expect("an approved SVG asset"),
            ApprovedPlatformAssetV1::new(
                "icons/icon-about.svg",
                ABOUT_SHA256,
                PlatformAssetMediaClassV1::Svg,
            )
            .expect("an approved SVG asset"),
            ApprovedPlatformAssetV1::new(
                "icons/icon-clients.svg",
                CLIENTS_SHA256,
                PlatformAssetMediaClassV1::Svg,
            )
            .expect("an approved SVG asset"),
            ApprovedPlatformAssetV1::new(
                "icons/icon-search.svg",
                SEARCH_SHA256,
                PlatformAssetMediaClassV1::Svg,
            )
            .expect("an approved SVG asset"),
        ];
        ApprovedPlatformGraphicBindingV1::new("1.0.0", CATALOGUE_SHA256, assets)
            .expect("a verified binding")
    }

    /// One registered Platform default for `key` at `slot`.
    fn record(
        key: PropertyKey,
        slot: &str,
        reference: &str,
        digest: &str,
    ) -> PlatformGraphicBaselineRecordV1 {
        PlatformGraphicBaselineRecordV1::new(
            key,
            slot,
            AssetReferenceV1::parse(AssetNamespace::Platform, reference)
                .expect("a valid Platform reference"),
            digest,
        )
        .expect("a bound registration record")
    }

    /// A baseline with one registered `graphic.logo` default at `primary`.
    fn logo_baseline() -> PlatformGraphicBaselineV1 {
        PlatformGraphicBaselineV1::new(
            binding(),
            vec![record(
                PropertyKey::GraphicLogo,
                GRAPHIC_PRIMARY_SLOT_V1,
                "icons/icon-home.svg",
                HOME_SHA256,
            )],
        )
        .expect("a valid registration table")
    }

    /// One authored slot selection, either operation, in the given namespace.
    fn selection(
        key: PropertyKey,
        slot: &str,
        reference: &str,
        supersede: bool,
        exposure: Exposure,
    ) -> GraphicSlotSelectionV1 {
        let namespace = if reference.starts_with("icons/") {
            AssetNamespace::Platform
        } else {
            AssetNamespace::Entity
        };
        let reference = AssetReferenceV1::parse(namespace, reference).expect("a valid reference");
        if supersede {
            GraphicSlotSelectionV1::supersede(key, slot, reference, exposure)
        } else {
            GraphicSlotSelectionV1::append(key, slot, reference, exposure)
        }
        .expect("a valid slot selection")
    }

    /// Wraps one owner's selections.
    fn block(
        owner: QualifiedOwnerV1,
        items: Vec<GraphicSlotSelectionV1>,
    ) -> OwnerGraphicSelectionsV1 {
        OwnerGraphicSelectionsV1::new(owner, items).expect("a valid selection block")
    }

    /// The candidate under test.
    fn input(
        declarations: Vec<DeclaredOwnerPropertiesV1>,
        baseline: PlatformGraphicBaselineV1,
        blocks: Vec<OwnerGraphicSelectionsV1>,
    ) -> GraphicCascadeInputV1 {
        let validated = validate_declared_properties_v1(&graph(), declarations)
            .expect("a valid declaration set");
        GraphicCascadeInputV1::new(validated, baseline, blocks).expect("a valid candidate")
    }

    /// The effective reference of one role/slot.
    fn slot_reference(resolved: &ResolvedGraphicOwnerV1, key: PropertyKey, slot: &str) -> String {
        resolved
            .role(key)
            .and_then(|role| role.slot(slot))
            .map_or_else(
                || panic!("{} must resolve slot {slot}", key.name()),
                |held| held.source().reference().to_owned(),
            )
    }

    /// `N128-01` — a registered root default is visible at the Entity and descends unchanged to the
    /// representation, an unregistered role stays absent, and no non-graphic value moves.
    #[test]
    fn n128_01_root_baseline_descends_and_unregistered_roles_stay_absent() {
        let declarations = vec![declared(
            site_owner(),
            vec![(
                PropertyKey::SeoTitle,
                inherit(vec![auth(title("Site"), Exposure::Public)]),
            )],
        )];
        let candidate = input(declarations, logo_baseline(), Vec::new());
        let graph_record = candidate
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");

        for owner in [
            entity_owner(),
            facet_owner(),
            site_owner(),
            page_owner(),
            representation_owner(),
        ] {
            let record = graph_record
                .for_owner(&owner)
                .expect("every owner resolves");
            assert_eq!(
                slot_reference(record, PropertyKey::GraphicLogo, "primary"),
                "icons/icon-home.svg"
            );
            for key in [
                PropertyKey::GraphicFavicon,
                PropertyKey::GraphicBanner,
                PropertyKey::GraphicBackground,
                PropertyKey::GraphicSocialPreview,
            ] {
                assert!(
                    record.role(key).expect("a role record").is_empty(),
                    "{} must stay absent when nothing is registered for it",
                    key.name()
                );
            }
        }

        let entity = graph_record.for_owner(&entity_owner()).expect("the Entity");
        let logo = entity
            .role(PropertyKey::GraphicLogo)
            .and_then(|role| role.slot("primary"))
            .expect("the root default");
        assert_eq!(
            logo.source().kind(),
            GraphicSourceKindV1::PlatformRootDefault
        );
        assert_eq!(logo.source().namespace(), AssetNamespace::Platform);
        assert!(logo.lineage().len() == 1 && logo.lineage()[0].is_platform_root());
        assert_eq!(
            logo.lineage()[0].operation(),
            GraphicLineageOperationV1::Selected
        );
        let binding = logo.source().binding().expect("a catalogue binding");
        assert_eq!(binding.library_version(), "1.0.0");
        assert_eq!(binding.catalogue_sha256(), CATALOGUE_SHA256);
        assert_eq!(binding.asset_id(), "icons/icon-home.svg");
        assert_eq!(binding.asset_sha256(), HOME_SHA256);

        let deepest = graph_record
            .for_owner(&representation_owner())
            .expect("the representation");
        assert_eq!(
            deepest.non_graphic().values_of(PropertyKey::SeoTitle).len(),
            1,
            "the inherited SEO title is untouched by the graphic baseline"
        );
        assert_eq!(
            representation_owner().level_index(),
            4,
            "the default reached all five levels"
        );
        println!(
            "n128_evidence case=01 root=visible_at_entity=true descends=5 unregistered_absent=true \
             non_graphic_unchanged=true binding=1.0.0"
        );
    }

    /// `N128-02` — an explicit nearer supersede wins, a still-nearer supersede wins in turn, the
    /// lineage records the chain, and the legacy additive conflict still refuses without supersede.
    #[test]
    fn n128_02_nearer_supersede_wins_then_still_nearer_and_append_conflicts() {
        let facet_override = block(
            facet_owner(),
            vec![selection(
                PropertyKey::GraphicLogo,
                "primary",
                "icons/icon-about.svg",
                true,
                Exposure::Public,
            )],
        );
        let site_override = block(
            site_owner(),
            vec![selection(
                PropertyKey::GraphicLogo,
                "primary",
                "icons/icon-clients.svg",
                true,
                Exposure::Public,
            )],
        );
        let candidate = input(
            Vec::new(),
            logo_baseline(),
            vec![facet_override, site_override],
        );
        let graph_record = candidate
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");

        // The Entity still carries the root default.
        let entity = graph_record.for_owner(&entity_owner()).expect("the Entity");
        assert_eq!(
            slot_reference(entity, PropertyKey::GraphicLogo, "primary"),
            "icons/icon-home.svg"
        );
        // The nearer Facet supersedes it, and the still-nearer Site supersedes the Facet.
        let facet = graph_record.for_owner(&facet_owner()).expect("the Facet");
        assert_eq!(
            slot_reference(facet, PropertyKey::GraphicLogo, "primary"),
            "icons/icon-about.svg"
        );
        assert_eq!(
            facet
                .role(PropertyKey::GraphicLogo)
                .and_then(|role| role.slot("primary"))
                .expect("the slot")
                .source()
                .kind(),
            GraphicSourceKindV1::AuthoredSelection
        );
        let site = graph_record.for_owner(&site_owner()).expect("the Site");
        assert_eq!(
            slot_reference(site, PropertyKey::GraphicLogo, "primary"),
            "icons/icon-clients.svg"
        );
        let site_slot = site
            .role(PropertyKey::GraphicLogo)
            .and_then(|role| role.slot("primary"))
            .expect("the site slot");
        let lineage = site_slot.lineage();
        assert_eq!(lineage.len(), 3, "root, Facet supersede, Site supersede");
        assert!(lineage[0].is_platform_root());
        assert_eq!(
            lineage[1].operation(),
            GraphicLineageOperationV1::Superseded
        );
        assert_eq!(lineage[1].owner(), Some(&facet_owner()));
        assert_eq!(
            lineage[2].operation(),
            GraphicLineageOperationV1::Superseded
        );
        assert_eq!(lineage[2].owner(), Some(&site_owner()));
        assert_eq!(
            site_slot.winning_owner(),
            Some(&site_owner()),
            "the winning owner is the nearest superseder"
        );

        // A sibling Site with no supersede keeps the nearest ancestor value: the Bali Site's
        // supersede never leaks into the Jakarta branch.
        let jakarta = graph_record
            .for_owner(&jakarta_site_owner())
            .expect("the sibling");
        assert_eq!(
            slot_reference(jakarta, PropertyKey::GraphicLogo, "primary"),
            "icons/icon-about.svg",
            "the sibling keeps the Facet value, not the Bali Site supersede"
        );

        // The legacy additive conflict still refuses when supersede is not authored.
        let additive_conflict = input(
            Vec::new(),
            logo_baseline(),
            vec![block(
                entity_owner(),
                vec![selection(
                    PropertyKey::GraphicLogo,
                    "primary",
                    "icons/icon-search.svg",
                    false,
                    Exposure::Public,
                )],
            )],
        );
        let failure = additive_conflict
            .resolve_graphic_cascade_all_v1()
            .expect_err("an append into an occupied slot must refuse");
        assert_eq!(
            failure.graphic_kind(),
            Some(GraphicCascadeRefusalKind::SlotAlreadyFilled)
        );
        println!(
            "n128_evidence case=02 nearer=win still_nearer=win lineage=3 namespace=platform \
             slot=primary append_conflict=refused sibling_isolated=true"
        );
    }

    /// `N128-03` — suppression is final within its scope, a valid local value after suppression is
    /// eligible, and the literal `none` context never acts as a supersede.
    #[test]
    fn n128_03_suppress_is_final_and_none_is_not_a_supersede() {
        // Root suppression clears the root-layer default for the whole Entity branch.
        let root_suppress = input(
            vec![declared(
                entity_owner(),
                vec![(PropertyKey::GraphicLogo, suppress(Vec::new()))],
            )],
            logo_baseline(),
            Vec::new(),
        );
        let record = root_suppress
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        for owner in [
            entity_owner(),
            facet_owner(),
            site_owner(),
            representation_owner(),
        ] {
            let resolved = record.for_owner(&owner).expect("every owner resolves");
            assert!(
                resolved
                    .role(PropertyKey::GraphicLogo)
                    .expect("a role")
                    .is_empty(),
                "a root suppress must clear the baseline for {}",
                owner.level_index()
            );
        }

        // A descendant suppress is final for its own subtree but does not touch its ancestors.
        let descendant_suppress = input(
            vec![declared(
                facet_owner(),
                vec![(PropertyKey::GraphicLogo, suppress(Vec::new()))],
            )],
            logo_baseline(),
            Vec::new(),
        );
        let record = descendant_suppress
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        assert_eq!(
            slot_reference(
                record.for_owner(&entity_owner()).expect("the Entity"),
                PropertyKey::GraphicLogo,
                "primary"
            ),
            "icons/icon-home.svg",
            "the ancestor keeps its own value"
        );
        assert!(
            record
                .for_owner(&site_owner())
                .expect("the Site")
                .role(PropertyKey::GraphicLogo)
                .expect("a role")
                .is_empty(),
            "the suppressed subtree keeps nothing"
        );

        // An explicitly authored local value after suppression is eligible, not a fallthrough.
        let suppressed_then_local = input(
            vec![declared(
                facet_owner(),
                vec![(PropertyKey::GraphicLogo, suppress(Vec::new()))],
            )],
            logo_baseline(),
            vec![block(
                facet_owner(),
                vec![selection(
                    PropertyKey::GraphicLogo,
                    "primary",
                    "logo-local.svg",
                    true,
                    Exposure::Public,
                )],
            )],
        );
        let record = suppressed_then_local
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        let facet = record.for_owner(&facet_owner()).expect("the Facet");
        assert_eq!(
            slot_reference(facet, PropertyKey::GraphicLogo, "primary"),
            "logo-local.svg"
        );
        assert_eq!(
            facet
                .role(PropertyKey::GraphicLogo)
                .expect("a role")
                .suppressed_at(),
            Some(&facet_owner()),
            "the suppression is recorded, not silently forgotten"
        );

        // `currentContext: none` is not a supersede: it neither clears nor replaces.
        let none_literal = input(
            vec![declared(
                site_owner(),
                vec![(PropertyKey::GraphicLogo, inherit(Vec::new()))],
            )],
            logo_baseline(),
            Vec::new(),
        );
        let record = none_literal
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        assert_eq!(
            slot_reference(
                record.for_owner(&site_owner()).expect("the Site"),
                PropertyKey::GraphicLogo,
                "primary"
            ),
            "icons/icon-home.svg",
            "the literal none inherits the baseline unchanged"
        );
        println!(
            "n128_evidence case=03 root_suppress=cleared descendant_suppress=final \
             local_after_suppress=eligible none_is_not_supersede=true suppressed_at=recorded"
        );
    }

    /// A baseline with two named `graphic.background` slots, in registration order.
    fn background_baseline() -> PlatformGraphicBaselineV1 {
        PlatformGraphicBaselineV1::new(
            binding(),
            vec![
                record(
                    PropertyKey::GraphicBackground,
                    "surface-main",
                    "icons/icon-home.svg",
                    HOME_SHA256,
                ),
                record(
                    PropertyKey::GraphicBackground,
                    "layer-top",
                    "icons/icon-search.svg",
                    SEARCH_SHA256,
                ),
            ],
        )
        .expect("a valid registration table")
    }

    /// `N128-04` — a collection supersedes only the named slot it names, appends preserve the other
    /// slots and their order, the registered maximum holds, and every slot conflict refuses.
    #[test]
    fn n128_04_named_slot_collection_supersede_append_and_conflicts() {
        let candidate = input(
            Vec::new(),
            background_baseline(),
            vec![
                block(
                    facet_owner(),
                    vec![selection(
                        PropertyKey::GraphicBackground,
                        "surface-main",
                        "icons/icon-about.svg",
                        true,
                        Exposure::Public,
                    )],
                ),
                block(
                    site_owner(),
                    vec![selection(
                        PropertyKey::GraphicBackground,
                        "sidebar",
                        "brand/sidebar.png",
                        false,
                        Exposure::Public,
                    )],
                ),
            ],
        );
        let record = candidate
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        let site = record.for_owner(&site_owner()).expect("the Site");
        let slots = site
            .role(PropertyKey::GraphicBackground)
            .expect("the role")
            .slots();
        assert_eq!(
            slots
                .iter()
                .map(|slot| slot.slot().as_str())
                .collect::<Vec<_>>(),
            vec!["surface-main", "layer-top", "sidebar"],
            "superseding one slot must not reorder or drop the others"
        );
        assert_eq!(slots[0].source().reference(), "icons/icon-about.svg");
        assert_eq!(
            slots[1].source().reference(),
            "icons/icon-search.svg",
            "the untouched slot keeps its root default"
        );
        assert_eq!(slots[2].source().namespace(), AssetNamespace::Entity);
        assert_eq!(
            slots[2].lineage()[0].operation(),
            GraphicLineageOperationV1::Appended
        );

        // An append into an occupied named slot refuses rather than overwriting it.
        let filled = input(
            Vec::new(),
            background_baseline(),
            vec![block(
                facet_owner(),
                vec![selection(
                    PropertyKey::GraphicBackground,
                    "layer-top",
                    "icons/icon-clients.svg",
                    false,
                    Exposure::Public,
                )],
            )],
        );
        assert_eq!(
            filled
                .resolve_graphic_cascade_all_v1()
                .expect_err("an occupied slot must refuse an append")
                .graphic_kind(),
            Some(GraphicCascadeRefusalKind::SlotAlreadyFilled)
        );
        // A fourth slot is the registered maximum; a fifth refuses deterministically.
        let over_capacity = input(
            Vec::new(),
            background_baseline(),
            vec![
                block(
                    facet_owner(),
                    vec![
                        selection(
                            PropertyKey::GraphicBackground,
                            "band-a",
                            "band/a.png",
                            false,
                            Exposure::Public,
                        ),
                        selection(
                            PropertyKey::GraphicBackground,
                            "band-b",
                            "band/b.png",
                            false,
                            Exposure::Public,
                        ),
                    ],
                ),
                block(
                    site_owner(),
                    vec![selection(
                        PropertyKey::GraphicBackground,
                        "band-c",
                        "band/c.png",
                        false,
                        Exposure::Public,
                    )],
                ),
            ],
        );
        assert_eq!(
            over_capacity
                .resolve_graphic_cascade_all_v1()
                .expect_err("a fifth background slot must refuse")
                .graphic_kind(),
            Some(GraphicCascadeRefusalKind::SlotCapacityExceeded)
        );

        // The same owner naming one slot twice refuses, and so does a duplicated block.
        let duplicate_slot = OwnerGraphicSelectionsV1::new(
            facet_owner(),
            vec![
                selection(
                    PropertyKey::GraphicBackground,
                    "surface-main",
                    "icons/icon-about.svg",
                    true,
                    Exposure::Public,
                ),
                selection(
                    PropertyKey::GraphicBackground,
                    "surface-main",
                    "icons/icon-clients.svg",
                    true,
                    Exposure::Public,
                ),
            ],
        )
        .expect_err("a duplicate role/slot pair must refuse");
        assert_eq!(
            duplicate_slot.kind(),
            GraphicCascadeRefusalKind::DuplicateGraphicSelection
        );
        let duplicate_block = GraphicCascadeInputV1::new(
            validate_declared_properties_v1(&graph(), Vec::new()).expect("a valid candidate"),
            logo_baseline(),
            vec![
                block(facet_owner(), Vec::new()),
                block(facet_owner(), Vec::new()),
            ],
        )
        .expect_err("two blocks for one owner must refuse");
        assert_eq!(
            duplicate_block.kind(),
            GraphicCascadeRefusalKind::DuplicateGraphicSelection
        );

        // A non-primary slot on a singular role is not a legal slot at all.
        let wrong_slot = GraphicSlotSelectionV1::supersede(
            PropertyKey::GraphicLogo,
            "surface-main",
            AssetReferenceV1::parse(AssetNamespace::Platform, "icons/icon-home.svg")
                .expect("a valid reference"),
            Exposure::Public,
        )
        .expect_err("a singular role admits only the primary slot");
        assert_eq!(
            wrong_slot.kind(),
            GraphicCascadeRefusalKind::SlotNotPermittedForRole
        );

        println!(
            "n128_evidence case=04 slot_replaced=1 others_preserved=2 order=stable max=4 \
             append_occupied=refused capacity=refused duplicate_slot=refused wrong_slot=refused"
        );
    }

    /// `N128-05` — registration, slot, role, format, digest and namespace identity all refuse, an
    /// available approved asset never becomes a default by itself, and no filename infers a role.
    #[test]
    fn n128_05_registration_and_identity_refusals() {
        let platform = |reference: &str| {
            AssetReferenceV1::parse(AssetNamespace::Platform, reference).expect("a valid reference")
        };

        // A registration for a non-graphic key is meaningless and refused.
        assert_eq!(
            PlatformGraphicBaselineRecordV1::new(
                PropertyKey::SeoTitle,
                GRAPHIC_PRIMARY_SLOT_V1,
                platform("icons/icon-home.svg"),
                HOME_SHA256,
            )
            .expect_err("only graphic roles may be registered")
            .kind(),
            GraphicCascadeRefusalKind::BaselineRoleNotGraphic
        );
        // A baseline record must live in the Platform namespace, never the Entity root.
        assert_eq!(
            PlatformGraphicBaselineRecordV1::new(
                PropertyKey::GraphicLogo,
                GRAPHIC_PRIMARY_SLOT_V1,
                AssetReferenceV1::parse(AssetNamespace::Entity, "brand/logo.png")
                    .expect("a valid reference"),
                HOME_SHA256,
            )
            .expect_err("the Entity root is not a Platform registration")
            .kind(),
            GraphicCascadeRefusalKind::BaselineNamespaceAlias
        );
        // Availability is not selection: an approved-library filename that is not in the verified
        // approved set cannot be registered, so `icons/icon-services.svg` can never become a default.
        assert_eq!(
            PlatformGraphicBaselineV1::new(
                binding(),
                vec![record(
                    PropertyKey::GraphicLogo,
                    GRAPHIC_PRIMARY_SLOT_V1,
                    "icons/icon-services.svg",
                    HOME_SHA256,
                )],
            )
            .expect_err("an unapproved identifier must refuse")
            .kind(),
            GraphicCascadeRefusalKind::BaselineAssetUnapproved
        );
        // A digest that disagrees with the verified catalogue refuses.
        let wrong_digest = record(
            PropertyKey::GraphicLogo,
            GRAPHIC_PRIMARY_SLOT_V1,
            "icons/icon-home.svg",
            ABOUT_SHA256,
        );
        assert_eq!(
            PlatformGraphicBaselineV1::new(binding(), vec![wrong_digest])
                .expect_err("a digest that is not the catalogue's own must refuse")
                .kind(),
            GraphicCascadeRefusalKind::BaselineDigestMismatch
        );
        // `graphic.favicon` is PNG only, so an approved SVG can never fill the favicon role.
        assert_eq!(
            PlatformGraphicBaselineV1::new(
                binding(),
                vec![record(
                    PropertyKey::GraphicFavicon,
                    GRAPHIC_PRIMARY_SLOT_V1,
                    "icons/icon-home.svg",
                    HOME_SHA256,
                )],
            )
            .expect_err("an SVG favicon default must refuse")
            .kind(),
            GraphicCascadeRefusalKind::BaselineRoleFormatMismatch
        );
        // The same role/slot registered twice refuses.
        assert_eq!(
            PlatformGraphicBaselineV1::new(
                binding(),
                vec![
                    record(
                        PropertyKey::GraphicLogo,
                        GRAPHIC_PRIMARY_SLOT_V1,
                        "icons/icon-home.svg",
                        HOME_SHA256,
                    ),
                    record(
                        PropertyKey::GraphicLogo,
                        GRAPHIC_PRIMARY_SLOT_V1,
                        "icons/icon-about.svg",
                        ABOUT_SHA256,
                    ),
                ],
            )
            .expect_err("a duplicate role/slot registration must refuse")
            .kind(),
            GraphicCascadeRefusalKind::DuplicateBaselineRecord
        );

        // Only lowercase slot identifiers parse; an uppercase or edge-hyphen spelling is refused.
        for invalid in ["Primary", "surface-", "-surface", "sur--face", ""] {
            assert!(
                GraphicSlotIdV1::parse(invalid).is_err(),
                "{invalid:?} must not parse as a slot identifier"
            );
        }
        assert!(GraphicSlotIdV1::parse("surface-main").is_ok());

        // A selection block for an owner outside the graph refuses.
        let stray = OwnerGraphicSelectionsV1::new(
            QualifiedOwnerV1::site(entity_id(), facet("company"), site("elsewhere")),
            Vec::new(),
        )
        .expect("a syntactically valid block");
        assert_eq!(
            GraphicCascadeInputV1::new(
                validate_declared_properties_v1(&graph(), Vec::new()).expect("a valid candidate"),
                logo_baseline(),
                vec![stray],
            )
            .expect_err("a block for a non-member owner must refuse")
            .kind(),
            GraphicCascadeRefusalKind::SelectionOwnerUnknown
        );

        // Even with four approved assets bound, only the registered role is filled: every other
        // graphic role stays absent, so neither availability nor a filename infers a role.
        let candidate = input(Vec::new(), logo_baseline(), Vec::new());
        let record = candidate
            .resolve_graphic_cascade_v1(&representation_owner())
            .expect("the candidate resolves");
        assert!(
            record
                .role(PropertyKey::GraphicFavicon)
                .expect("a role")
                .is_empty()
        );
        assert!(
            record
                .role(PropertyKey::GraphicBanner)
                .expect("a role")
                .is_empty()
        );
        assert_eq!(
            record
                .role(PropertyKey::GraphicLogo)
                .expect("a role")
                .slots()
                .len(),
            1,
            "exactly one slot is registered and nothing is inferred beyond it"
        );
        println!(
            "n128_evidence case=05 unregistered=absent wrong_role=refused favicon_svg=refused \
             digest=refused namespace_alias=refused unapproved_asset=refused duplicate=refused \
             slot_grammar=closed filename_inference=false"
        );
    }

    /// `N128-06` — an internal winner never reappears publicly, a late invalid declaration refuses
    /// the whole candidate with zero partial result, and a legacy caller stays byte-identical.
    #[test]
    fn n128_06_whole_owner_refusal_internal_projection_and_legacy_parity() {
        // An internal supersede at the Site resolves internally and is never projected publicly.
        let internal = input(
            Vec::new(),
            logo_baseline(),
            vec![block(
                site_owner(),
                vec![selection(
                    PropertyKey::GraphicLogo,
                    "primary",
                    "brand/private-logo.png",
                    true,
                    Exposure::Internal,
                )],
            )],
        );
        let graph_record = internal
            .resolve_graphic_cascade_all_v1()
            .expect("the candidate resolves");
        let site = graph_record.for_owner(&site_owner()).expect("the Site");
        let slot = site
            .role(PropertyKey::GraphicLogo)
            .and_then(|role| role.slot("primary"))
            .expect("the site slot");
        assert_eq!(slot.source().exposure(), Exposure::Internal);
        assert!(
            project_public_graphic_slots_v1(site)
                .slots_of(PropertyKey::GraphicLogo)
                .is_empty(),
            "an internal winner is never projected"
        );
        let entity_projection = project_public_graphic_slots_v1(
            graph_record.for_owner(&entity_owner()).expect("the Entity"),
        );
        assert_eq!(
            entity_projection.slots_of(PropertyKey::GraphicLogo).len(),
            1,
            "an ancestor's public default is unaffected"
        );
        let deepest = graph_record
            .for_owner(&representation_owner())
            .expect("the representation");
        assert!(
            project_public_graphic_slots_v1(deepest)
                .slots_of(PropertyKey::GraphicLogo)
                .is_empty(),
            "the internal winner is inherited and never replaced by the root default"
        );

        // A malicious late declaration refuses the entire candidate: no owner is returned.
        let late = input(
            Vec::new(),
            logo_baseline(),
            vec![block(
                representation_owner(),
                vec![selection(
                    PropertyKey::GraphicLogo,
                    "primary",
                    "brand/late.png",
                    false,
                    Exposure::Public,
                )],
            )],
        );
        assert!(
            late.resolve_graphic_cascade_all_v1().is_err(),
            "a filled slot at the deepest owner must refuse"
        );
        assert!(
            late.resolve_graphic_cascade_v1(&entity_owner()).is_err(),
            "an early owner cannot be returned from an invalid candidate"
        );

        // Parity: with no registration and no selection the new path reproduces the legacy graph.
        let declarations = vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![auth(background("brand/root.png"), Exposure::Public)]),
                )],
            ),
            declared(
                site_owner(),
                vec![(
                    PropertyKey::GraphicBackground,
                    inherit(vec![
                        auth(background("brand/one.png"), Exposure::Public),
                        auth(background("brand/two.png"), Exposure::Public),
                    ]),
                )],
            ),
        ];
        let validated = validate_declared_properties_v1(&graph(), declarations)
            .expect("a valid declaration set");
        let legacy = validated.resolve_all().expect("the legacy graph resolves");
        let parity = GraphicCascadeInputV1::new(
            validated.clone(),
            PlatformGraphicBaselineV1::empty(binding()),
            Vec::new(),
        )
        .expect("a valid candidate");
        assert!(
            !parity.is_slot_aware(),
            "no registration and no selection is parity mode"
        );
        let parity_graph = parity
            .resolve_graphic_cascade_all_v1()
            .expect("the parity graph resolves");
        for owner in validated.owners() {
            let legacy_record = legacy.for_owner(owner).expect("a legacy owner");
            let parity_record = parity_graph.for_owner(owner).expect("a parity owner");
            for key in PropertyKey::ALL.into_iter().filter(|key| key.is_graphic()) {
                let legacy_references = asset_references(legacy_record.values_of(key));
                let parity_references: Vec<String> = parity_record
                    .role(key)
                    .expect("a role record")
                    .legacy()
                    .iter()
                    .filter_map(|value| value.data().asset_reference())
                    .map(|reference| reference.reference().to_owned())
                    .collect();
                assert_eq!(
                    legacy_references,
                    parity_references,
                    "{} at level {} must be byte-identical",
                    key.name(),
                    owner.level_index()
                );
                assert!(
                    parity_record
                        .role(key)
                        .expect("a role record")
                        .slots()
                        .is_empty(),
                    "parity mode never invents a slot identity"
                );
            }
        }

        // Registering a default cannot change the legacy path's own answer for the same input.
        let baseline_present =
            GraphicCascadeInputV1::new(validated.clone(), logo_baseline(), Vec::new())
                .expect("a valid candidate");
        assert!(baseline_present.is_slot_aware());
        let legacy_again = validated
            .resolve_all()
            .expect("the legacy graph still resolves");
        for owner in validated.owners() {
            let before = legacy.for_owner(owner).expect("a legacy owner");
            let after = legacy_again.for_owner(owner).expect("a legacy owner");
            assert_eq!(
                asset_references(before.values_of(PropertyKey::GraphicLogo)),
                asset_references(after.values_of(PropertyKey::GraphicLogo)),
                "an installed registration must not reach the legacy resolver"
            );
        }
        println!(
            "n128_evidence case=06 internal_hidden=true no_public_reappearance=true \
             late_refusal=whole zero_partial=true legacy_parity=identical \
             defaults_without_optin=false"
        );
    }
}
