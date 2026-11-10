//! `R03.A` application use-case tests: the inbound property-resolution boundary.
//!
//! These tests drive the real use case over the accepted one-Entity graph and assert the
//! candidate's typed contents, its refusals and that it never claims publication authority.

use solo_application::{
    PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY, PropertyResolutionRequestV1,
    resolve_declared_properties_v1,
};
use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1, Exposure,
    FacetDraft, FacetIdV1, FactIdV1, Inheritance, LocaleKeyV1, LocationExtensionRequestV1,
    LocationFactV1, LocationFieldsV1, LocationKind, MailboxV1, PageDraft, PageIdV1,
    PropertyDeclarationV1, PropertyKey, PropertyResolutionRefusalKind, PropertyValueV1,
    QualifiedOwnerV1, SiteDraft, SiteIdV1,
};

fn entity() -> EntityIdV1 {
    EntityIdV1::parse("example").expect("a canonical Entity identifier")
}

fn facet() -> FacetIdV1 {
    FacetIdV1::parse("main").expect("a canonical Facet identifier")
}

fn site() -> SiteIdV1 {
    SiteIdV1::parse("bali").expect("a canonical Site identifier")
}

fn page() -> PageIdV1 {
    PageIdV1::parse("home").expect("a canonical Page identifier")
}

fn locale() -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical("en-001").expect("a canonical full locale key")
}

fn graph() -> EfsOwnershipGraph {
    let mut home = PageDraft::new(page());
    home.add_representation(locale());
    let mut bali = SiteDraft::new(site());
    bali.add_page(home);
    let mut main = FacetDraft::new(facet());
    main.add_site(bali);
    let mut draft = EfsOwnershipDraft::new(entity());
    draft.add_facet(main);
    draft.build().expect("a valid one-Entity structure")
}

fn entity_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::entity(entity())
}

fn site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), site())
}

fn representation_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), site(), page(), locale())
}

fn inherit(values: Vec<AuthoredPropertyValueV1>) -> PropertyDeclarationV1 {
    let context = if values.is_empty() {
        CurrentContextV1::None
    } else {
        CurrentContextV1::Values(values)
    };
    PropertyDeclarationV1::new(Inheritance::Inherit, context)
}

fn auth(value: PropertyValueV1, exposure: Exposure) -> AuthoredPropertyValueV1 {
    AuthoredPropertyValueV1::new(value, exposure)
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

fn declared(
    owner: QualifiedOwnerV1,
    entries: Vec<(PropertyKey, PropertyDeclarationV1)>,
) -> DeclaredOwnerPropertiesV1 {
    DeclaredOwnerPropertiesV1::new(owner, entries).expect("a valid declaration set")
}

/// `R03A-02` — the use case resolves all five levels and returns a public projection while
/// publishing nothing.
#[test]
fn application_resolves_all_five_levels_without_publication_authority() {
    let publishes = PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY;
    assert!(
        !publishes,
        "a property candidate is never publication authority"
    );
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
                inherit(vec![auth(background("bg/site.png"), Exposure::Public)]),
            )],
        ),
        declared(
            representation_owner(),
            vec![(PropertyKey::GraphicBackground, inherit(Vec::new()))],
        ),
    ];
    let graph = graph();
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        representation_owner(),
    ))
    .expect("a valid resolution");

    assert_eq!(candidate.resolved().owner(), &representation_owner());
    assert_eq!(
        candidate
            .resolved()
            .values_of(PropertyKey::GraphicBackground)
            .len(),
        2
    );
    assert_eq!(
        candidate
            .public()
            .values_of(PropertyKey::GraphicBackground)
            .len(),
        2
    );
    assert!(!candidate.is_publication_authorized());
    assert!(!candidate.has_recognized_location_extension());
    assert_eq!(candidate.resolved().properties().len(), 9);
}

/// `R03A-07` — an internal value is resolved but never projected, and the candidate stays a
/// candidate.
#[test]
fn application_public_projection_omits_internal_values() {
    let declarations = vec![declared(
        site_owner(),
        vec![(
            PropertyKey::ContactEmail,
            inherit(vec![auth(email("hidden@example.com"), Exposure::Internal)]),
        )],
    )];
    let graph = graph();
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        representation_owner(),
    ))
    .expect("a valid resolution");

    assert_eq!(
        candidate
            .resolved()
            .values_of(PropertyKey::ContactEmail)
            .len(),
        1
    );
    assert!(
        candidate
            .public()
            .values_of(PropertyKey::ContactEmail)
            .is_empty()
    );
    assert!(!format!("{:?}", candidate.public()).contains("hidden@example.com"));
}

/// `R03A-08` — an owner outside the graph is refused by the use case.
#[test]
fn application_refuses_an_owner_outside_the_graph() {
    let graph = graph();
    let elsewhere = QualifiedOwnerV1::representation(
        entity(),
        facet(),
        site(),
        page(),
        LocaleKeyV1::from_already_canonical("id-ID").expect("a canonical full locale key"),
    );
    let refusal = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        Vec::new(),
        elsewhere,
    ))
    .expect_err("an undeclared locale refuses");
    assert_eq!(refusal.kind(), PropertyResolutionRefusalKind::UnknownOwner);
}

/// `R03A-08` — a declaration attached to a second Entity is refused by the use case.
#[test]
fn application_refuses_a_declaration_owner_outside_the_graph() {
    let graph = graph();
    let second_entity =
        QualifiedOwnerV1::entity(EntityIdV1::parse("other").expect("a canonical identifier"));
    let refusal = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        vec![declared(
            second_entity,
            vec![(
                PropertyKey::ContactEmail,
                inherit(vec![auth(email("x@example.com"), Exposure::Public)]),
            )],
        )],
        representation_owner(),
    ))
    .expect_err("a second-Entity declaration refuses");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::DeclarationOwnerUnknown
    );
}

/// `R03A-10` — the recognized `extend` request surfaces as the named unsupported outcome, and
/// the `add` path stays a plain validated fact.
#[test]
fn application_surfaces_the_named_extension_refusal() {
    let mut fields = LocationFieldsV1::new();
    fields
        .set_locality("Denpasar")
        .expect("valid locality text");
    let fact = LocationFactV1::add(
        FactIdV1::parse("bali-base").expect("a valid fact identifier"),
        LocationKind::Association,
        fields,
    )
    .expect("a non-empty location fact");
    let extension = PropertyValueV1::location_extension(LocationExtensionRequestV1::new(
        FactIdV1::parse("bali-base").expect("a valid fact identifier"),
    ));
    let declarations = vec![
        declared(
            site_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(
                    PropertyValueV1::location(fact),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            representation_owner(),
            vec![(
                PropertyKey::Location,
                inherit(vec![auth(extension, Exposure::Internal)]),
            )],
        ),
    ];
    let graph = graph();
    let refusal = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        representation_owner(),
    ))
    .expect_err("R03.A cannot execute an extension");
    assert_eq!(
        refusal.kind(),
        PropertyResolutionRefusalKind::LocationExtensionNotImplementedInR03A
    );
    assert_eq!(refusal.code(), "LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A");
}
