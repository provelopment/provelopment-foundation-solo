//! `R03.A` pure-domain property registry, cascade, provenance and projection matrix.
//!
//! Every test names the acceptance case it proves. The graph helpers build the accepted
//! one-Entity shape — one Entity, two Facets, Sites with the **same** Page identifier under
//! different Sites, and canonical full-locale representations — so isolation is proven by
//! identifiers rather than by a comment.

use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CoordinatesV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsLevel, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1,
    Exposure, FacetDraft, FacetIdV1, FactIdV1, Inheritance, LocaleKeyV1,
    LocationExtensionRequestV1, LocationFactV1, LocationFieldsV1, LocationKind,
    MAX_BACKGROUND_LAYERS, MAX_LOCAL_ENTRIES, MAX_LOCATION_FACTS, MAX_SEO_TITLE_SCALARS, MailboxV1,
    PROPERTY_REGISTRY_VERSION, PageDraft, PageIdV1, PlainTextV1, PropertyDeclarationV1,
    PropertyKey, PropertyRefusalKind, PropertyResolutionRefusalKind, PropertyValueV1,
    QualifiedOwnerV1, ResolutionOperation, SiteDraft, SiteIdV1, project_public_properties_v1,
    validate_declared_properties_v1,
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
    AuthoredPropertyValueV1::new(value, exposure)
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
            (ResolutionOperation::Added, Some(0)),
        ]
    );
    assert_eq!(record.resolution_events()[0].owner(), &entity_owner());
    assert_eq!(record.resolution_events()[2].owner(), &site_owner());

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
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.svg")
            .expect_err("an active SVG refuses")
            .kind(),
        PropertyRefusalKind::UnsupportedAssetExtension
    );
    assert_eq!(
        AssetReferenceV1::parse(AssetNamespace::Entity, "assets/logo.gif")
            .expect_err("a GIF refuses")
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
    // Another locale of the same Page is a different representation.
    let indonesian = QualifiedOwnerV1::representation(
        entity_id(),
        facet("main"),
        site("bali"),
        page("home"),
        locale("id-ID"),
    );
    assert!(
        input
            .resolve(&indonesian)
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
    // The Page is not its representation, and the Site is not its Page.
    assert!(
        input
            .resolve(&page_owner())
            .expect("a valid resolution")
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
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

/// `R03A-10` — the accepted `location` `extend` syntax is recognized and refused by name,
/// while the ordinary `add` cascade still resolves.
#[test]
fn r03a_10_recognized_extend_is_refused_by_name_while_add_works() {
    let extension = PropertyValueV1::location_extension(LocationExtensionRequestV1::new(
        FactIdV1::parse("bali-base").expect("a valid fact identifier"),
    ));
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
                    inherit(vec![auth(extension, Exposure::Internal)]),
                )],
            ),
        ],
    )
    .expect("the accepted extend syntax is recognized, not reported as invalid");
    assert!(input.has_recognized_location_extension());
    let refusal = input
        .resolve(&representation_owner())
        .expect_err("an attempted resolution involving extend refuses");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A
    );
    assert_eq!(refusal.code(), "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A");

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
    let facts = add_resolved.values_of(PropertyKey::Location);
    assert_eq!(facts.len(), 1);
    let fact = facts[0]
        .data()
        .location_fact()
        .expect("a resolved location fact");
    assert_eq!(fact.fact_id().as_str(), "bali-base");
    assert_eq!(fact.kind(), LocationKind::Association);
    assert_eq!(
        fact.fields().set_field_names(),
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
