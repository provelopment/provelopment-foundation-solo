//! `R03.C1` application tests: the authored graphic asset-obligation inventory.
//!
//! Each test drives the real property use case over the accepted one-Entity graph, then collects
//! the inventory from the resulting validated candidate. Assertions are made on materialized
//! owner/role/index/exposure/reference data, never on a bool or an empty vector alone.

use solo_application::{
    GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY, GraphicAssetObligationInventoryV1,
    PropertyResolutionRequestV1, collect_graphic_asset_obligations_v1,
    resolve_declared_properties_v1,
};
use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsOwnershipDraft, EfsOwnershipGraph, EntityIdV1, Exposure,
    FacetDraft, FacetIdV1, Inheritance, LocaleKeyV1, PageDraft, PageIdV1, PropertyDeclarationV1,
    PropertyKey, PropertyValueV1, QualifiedOwnerV1, SiteDraft, SiteIdV1,
};

fn entity() -> EntityIdV1 {
    EntityIdV1::parse("example").expect("a canonical Entity identifier")
}

fn facet() -> FacetIdV1 {
    FacetIdV1::parse("main").expect("a canonical Facet identifier")
}

fn bali() -> SiteIdV1 {
    SiteIdV1::parse("bali").expect("a canonical Site identifier")
}

fn jakarta() -> SiteIdV1 {
    SiteIdV1::parse("jakarta").expect("a canonical Site identifier")
}

fn home() -> PageIdV1 {
    PageIdV1::parse("home").expect("a canonical Page identifier")
}

fn locale(value: &str) -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical(value).expect("a canonical full locale key")
}

/// One Entity, one Facet, two sibling Sites, a Page in each and — on the Bali Page — two
/// full-locale representations. Nine owners in total.
fn graph() -> EfsOwnershipGraph {
    let mut bali_page = PageDraft::new(home());
    bali_page.add_representation(locale("en-001"));
    bali_page.add_representation(locale("de-DE"));
    let mut bali_site = SiteDraft::new(bali());
    bali_site.add_page(bali_page);

    let mut jakarta_page = PageDraft::new(home());
    jakarta_page.add_representation(locale("en-001"));
    let mut jakarta_site = SiteDraft::new(jakarta());
    jakarta_site.add_page(jakarta_page);

    let mut main = FacetDraft::new(facet());
    main.add_site(bali_site);
    main.add_site(jakarta_site);

    let mut draft = EfsOwnershipDraft::new(entity());
    draft.add_facet(main);
    draft.build().expect("a valid one-Entity structure")
}

fn entity_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::entity(entity())
}

fn facet_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::facet(entity(), facet())
}

fn bali_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), bali())
}

fn jakarta_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), jakarta())
}

fn bali_page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), bali(), home())
}

fn jakarta_page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), jakarta(), home())
}

fn bali_en_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), bali(), home(), locale("en-001"))
}

fn bali_de_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), bali(), home(), locale("de-DE"))
}

fn jakarta_en_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), jakarta(), home(), locale("en-001"))
}

fn authored(value: PropertyValueV1, exposure: Exposure) -> AuthoredPropertyValueV1 {
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

fn asset(namespace: AssetNamespace, reference: &str) -> PropertyValueV1 {
    PropertyValueV1::graphic(
        AssetReferenceV1::parse(namespace, reference).expect("a valid asset reference"),
    )
}

fn entity_asset(reference: &str) -> PropertyValueV1 {
    asset(AssetNamespace::Entity, reference)
}

fn declared(
    owner: QualifiedOwnerV1,
    entries: Vec<(PropertyKey, PropertyDeclarationV1)>,
) -> DeclaredOwnerPropertiesV1 {
    DeclaredOwnerPropertiesV1::new(owner, entries).expect("a valid declaration set")
}

fn inventory(
    declarations: Vec<DeclaredOwnerPropertiesV1>,
    target: QualifiedOwnerV1,
) -> GraphicAssetObligationInventoryV1 {
    let graph = graph();
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        target,
    ))
    .expect("a valid whole-graph property candidate");
    collect_graphic_asset_obligations_v1(&candidate).expect("the inventory collects")
}

/// One obligation rendered as `(declaring owner, role, local index, exposure, namespace, text)`.
fn rendered(
    inventory: &GraphicAssetObligationInventoryV1,
) -> Vec<(String, String, usize, String, String, String)> {
    inventory
        .obligations()
        .iter()
        .map(|obligation| {
            (
                obligation.declaring_owner().describe(),
                obligation.role().name().to_owned(),
                obligation.local_index(),
                obligation.exposure().as_str().to_owned(),
                obligation.namespace().as_str().to_owned(),
                obligation.reference_text().to_owned(),
            )
        })
        .collect()
}

/// `R03C1-01` — graphic declarations at all five EFS levels produce exactly one obligation per
/// independently authored value, ordered by declaring owner in graph order, then registry key,
/// then zero-based local index. Inherited copies never become obligations.
#[test]
fn r03c1_01_inventory_covers_all_five_levels_in_graph_and_registry_order() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/entity-logo.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            facet_owner(),
            vec![(
                PropertyKey::GraphicBanner,
                inherit(vec![authored(
                    entity_asset("brand/facet-banner.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            bali_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![
                    authored(entity_asset("bg/bali-1.png"), Exposure::Public),
                    authored(entity_asset("bg/bali-2.png"), Exposure::Public),
                ]),
            )],
        ),
        declared(
            jakarta_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("bg/jakarta-1.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            bali_page_owner(),
            vec![(
                PropertyKey::GraphicSocialPreview,
                inherit(vec![authored(
                    entity_asset("social/bali-page.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            bali_en_owner(),
            vec![(
                PropertyKey::GraphicFavicon,
                inherit(vec![authored(
                    entity_asset("icons/favicon.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            bali_de_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/de-logo.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            jakarta_en_owner(),
            vec![(
                PropertyKey::GraphicBanner,
                inherit(vec![authored(
                    entity_asset("brand/jakarta-banner.png"),
                    Exposure::Internal,
                )]),
            )],
        ),
    ];
    let graph = graph();
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        bali_en_owner(),
    ))
    .expect("a valid whole-graph candidate");
    let inventory =
        collect_graphic_asset_obligations_v1(&candidate).expect("the inventory collects");

    // Nine independently authored sources. The Bali `en-001` representation inherits several
    // Entity/Facet/Site/Page values but adds exactly its own favicon.
    assert_eq!(inventory.len(), 9);
    assert!(!inventory.is_empty());

    // The declaring-owner sequence is the graph's own owner order filtered to owners that really
    // authored a graphic value, so ordering is parent-first regardless of authored key order.
    let authored_owners: Vec<String> = candidate
        .input()
        .owners()
        .iter()
        .filter(|owner| inventory.obligations_of(owner).next().is_some())
        .map(QualifiedOwnerV1::describe)
        .collect();
    let mut declared_owners: Vec<String> = Vec::new();
    for obligation in inventory.obligations() {
        let described = obligation.declaring_owner().describe();
        if !declared_owners.contains(&described) {
            declared_owners.push(described);
        }
    }
    assert_eq!(declared_owners, authored_owners);
    assert_eq!(
        inventory.obligations()[0].declaring_owner(),
        &entity_owner()
    );
    assert_eq!(
        inventory
            .obligations()
            .last()
            .expect("obligations")
            .declaring_owner(),
        &jakarta_en_owner()
    );

    // Every declaring owner carries only its own authored values: no inherited duplication.
    assert_eq!(inventory.obligations_of(&bali_en_owner()).count(), 1);
    assert_eq!(inventory.obligations_of(&jakarta_page_owner()).count(), 0);

    // Exact materialized `(declaring owner, role, index, exposure, namespace, text)` data.
    let row = |owner: &QualifiedOwnerV1, role: &str, index: usize, exposure: &str, text: &str| {
        (
            owner.describe(),
            role.to_owned(),
            index,
            exposure.to_owned(),
            "entity".to_owned(),
            text.to_owned(),
        )
    };
    let mut actual = rendered(&inventory);
    actual.sort();
    let mut expected = vec![
        row(
            &entity_owner(),
            "graphic.logo",
            0,
            "public",
            "brand/entity-logo.png",
        ),
        row(
            &facet_owner(),
            "graphic.banner",
            0,
            "public",
            "brand/facet-banner.png",
        ),
        row(
            &bali_owner(),
            "graphic.background",
            0,
            "public",
            "bg/bali-1.png",
        ),
        row(
            &bali_owner(),
            "graphic.background",
            1,
            "public",
            "bg/bali-2.png",
        ),
        row(
            &jakarta_owner(),
            "graphic.background",
            0,
            "public",
            "bg/jakarta-1.png",
        ),
        row(
            &bali_page_owner(),
            "graphic.social-preview",
            0,
            "public",
            "social/bali-page.png",
        ),
        row(
            &bali_en_owner(),
            "graphic.favicon",
            0,
            "public",
            "icons/favicon.png",
        ),
        row(
            &bali_de_owner(),
            "graphic.logo",
            0,
            "public",
            "brand/de-logo.png",
        ),
        row(
            &jakarta_en_owner(),
            "graphic.banner",
            0,
            "internal",
            "brand/jakarta-banner.png",
        ),
    ];
    expected.sort();
    assert_eq!(actual, expected);

    assert_eq!(inventory.role_count(PropertyKey::GraphicBackground), 3);
    assert_eq!(inventory.role_count(PropertyKey::GraphicLogo), 2);
    assert_eq!(inventory.role_count(PropertyKey::GraphicBanner), 2);
    assert_eq!(inventory.role_count(PropertyKey::GraphicFavicon), 1);
    assert_eq!(inventory.role_count(PropertyKey::GraphicSocialPreview), 1);
}

/// `R03C1-02` — every one of the five registered graphic roles is inventoried with its exact
/// typed namespace, unchanged reference text, original exposure and original local index.
#[test]
fn r03c1_02_all_five_roles_keep_namespace_reference_exposure_and_index() {
    let inventory = inventory(
        vec![declared(
            jakarta_page_owner(),
            vec![
                (
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        entity_asset("brand/logo.png"),
                        Exposure::Public,
                    )]),
                ),
                (
                    PropertyKey::GraphicFavicon,
                    inherit(vec![authored(
                        entity_asset("icons/favicon.png"),
                        Exposure::Public,
                    )]),
                ),
                (
                    PropertyKey::GraphicBanner,
                    inherit(vec![authored(
                        asset(AssetNamespace::Platform, "shared/banner.webp"),
                        Exposure::Internal,
                    )]),
                ),
                (
                    PropertyKey::GraphicBackground,
                    inherit(vec![
                        authored(
                            asset(AssetNamespace::Platform, "shared/bg-a.avif"),
                            Exposure::Public,
                        ),
                        authored(entity_asset("bg/b.jpeg"), Exposure::Public),
                    ]),
                ),
                (
                    PropertyKey::GraphicSocialPreview,
                    inherit(vec![authored(
                        entity_asset("social/preview.jpg"),
                        Exposure::Public,
                    )]),
                ),
            ],
        )],
        jakarta_en_owner(),
    );

    assert_eq!(inventory.len(), 6);
    let rows = inventory.obligations();
    // Registry order inside one owner: logo, favicon, banner, background, background, social.
    let observed: Vec<(&str, usize, AssetNamespace, &str, Exposure)> = rows
        .iter()
        .map(|row| {
            (
                row.role().name(),
                row.local_index(),
                row.namespace(),
                row.reference_text(),
                row.exposure(),
            )
        })
        .collect();
    assert_eq!(
        observed,
        vec![
            (
                "graphic.logo",
                0,
                AssetNamespace::Entity,
                "brand/logo.png",
                Exposure::Public
            ),
            (
                "graphic.favicon",
                0,
                AssetNamespace::Entity,
                "icons/favicon.png",
                Exposure::Public
            ),
            (
                "graphic.banner",
                0,
                AssetNamespace::Platform,
                "shared/banner.webp",
                Exposure::Internal
            ),
            (
                "graphic.background",
                0,
                AssetNamespace::Platform,
                "shared/bg-a.avif",
                Exposure::Public
            ),
            (
                "graphic.background",
                1,
                AssetNamespace::Entity,
                "bg/b.jpeg",
                Exposure::Public
            ),
            (
                "graphic.social-preview",
                0,
                AssetNamespace::Entity,
                "social/preview.jpg",
                Exposure::Public
            ),
        ]
    );

    // No renamed and no omitted role: exactly the five registered graphic keys appear.
    let mut unique: Vec<PropertyKey> = Vec::new();
    for row in rows {
        if !unique.contains(&row.role()) {
            unique.push(row.role());
        }
    }
    assert_eq!(
        unique,
        vec![
            PropertyKey::GraphicLogo,
            PropertyKey::GraphicFavicon,
            PropertyKey::GraphicBanner,
            PropertyKey::GraphicBackground,
            PropertyKey::GraphicSocialPreview,
        ]
    );
    for row in rows {
        assert!(row.role().is_graphic());
        assert_eq!(row.declaring_owner(), &jakarta_page_owner());
    }
}

/// `R03C1-03` — two sibling Sites and two full-locale representations carry different graphic
/// declarations, and the inventory is identical whichever target is requested.
#[test]
fn r03c1_03_inventory_is_identical_for_any_requested_target_and_never_infers_geography() {
    let declarations = || {
        vec![
            declared(
                bali_owner(),
                vec![(
                    PropertyKey::GraphicBanner,
                    inherit(vec![authored(
                        entity_asset("bali/banner.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                jakarta_owner(),
                vec![(
                    PropertyKey::GraphicBanner,
                    inherit(vec![authored(
                        entity_asset("jakarta/banner.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                bali_en_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        entity_asset("bali/logo-en.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                bali_de_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        entity_asset("bali/logo-de.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                jakarta_en_owner(),
                vec![(
                    PropertyKey::GraphicFavicon,
                    inherit(vec![authored(
                        entity_asset("jakarta/favicon.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
        ]
    };

    let from_bali = inventory(declarations(), bali_en_owner());
    let from_jakarta = inventory(declarations(), jakarta_en_owner());
    let from_bali_de = inventory(declarations(), bali_de_owner());

    assert_eq!(from_bali, from_jakarta);
    assert_eq!(from_bali, from_bali_de);
    assert_eq!(from_bali.len(), 5);

    // Each sibling keeps its own authored source; nothing is shared, substituted or inferred.
    let mut texts: Vec<String> = from_bali
        .obligations()
        .iter()
        .map(|row| row.reference_text().to_owned())
        .collect();
    texts.sort();
    assert_eq!(
        texts,
        vec![
            "bali/banner.png".to_owned(),
            "bali/logo-de.png".to_owned(),
            "bali/logo-en.png".to_owned(),
            "jakarta/banner.png".to_owned(),
            "jakarta/favicon.png".to_owned(),
        ]
    );
    // A locale is a representation membership key, never geography: the `de-DE` owner keeps
    // exactly its own obligation and never receives the `en-001` one.
    assert_eq!(from_bali.obligations_of(&bali_de_owner()).count(), 1);
    assert_eq!(from_bali.obligations_of(&bali_en_owner()).count(), 1);
}
/// `R03C1-04` — an internal graphic and an ancestor graphic suppressed by a descendant both stay
/// inventoried as their original authored obligations, while the public projection still exposes
/// neither at that descendant.
#[test]
fn r03c1_04_internal_and_descendant_suppressed_sources_stay_inventoried_and_unpublished() {
    let graph = graph();
    let declarations = vec![declared(
        entity_owner(),
        vec![
            (
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/entity-logo.png"),
                    Exposure::Internal,
                )]),
            ),
            (
                PropertyKey::GraphicBanner,
                inherit(vec![authored(
                    entity_asset("brand/entity-banner.png"),
                    Exposure::Public,
                )]),
            ),
        ],
    )];
    let mut with_suppression = declarations.clone();
    with_suppression.push(declared(
        bali_en_owner(),
        vec![(PropertyKey::GraphicBanner, suppress(Vec::new()))],
    ));

    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        with_suppression,
        bali_en_owner(),
    ))
    .expect("a valid whole-graph candidate");
    let inventory =
        collect_graphic_asset_obligations_v1(&candidate).expect("the inventory collects");

    // Both Entity declarations are still authored obligations, with their original exposures.
    assert_eq!(inventory.len(), 2);
    assert_eq!(inventory.role_count(PropertyKey::GraphicLogo), 1);
    assert_eq!(inventory.role_count(PropertyKey::GraphicBanner), 1);
    let logo = &inventory.obligations()[0];
    let banner = &inventory.obligations()[1];
    assert_eq!(logo.declaring_owner(), &entity_owner());
    assert_eq!(logo.role(), PropertyKey::GraphicLogo);
    assert_eq!(logo.exposure(), Exposure::Internal);
    assert_eq!(logo.reference_text(), "brand/entity-logo.png");
    assert_eq!(banner.declaring_owner(), &entity_owner());
    assert_eq!(banner.role(), PropertyKey::GraphicBanner);
    assert_eq!(banner.exposure(), Exposure::Public);
    assert_eq!(banner.reference_text(), "brand/entity-banner.png");

    // The accepted public projection at that descendant still shows neither value.
    assert!(
        candidate
            .public()
            .values_of(PropertyKey::GraphicLogo)
            .is_empty(),
        "an internal graphic is never projected"
    );
    assert!(
        candidate
            .public()
            .values_of(PropertyKey::GraphicBanner)
            .is_empty(),
        "a suppressed inherited graphic is never projected"
    );
    // The internal record still exists internally, so the suppression did not delete authoring.
    assert_eq!(
        candidate
            .resolved()
            .values_of(PropertyKey::GraphicLogo)
            .len(),
        1
    );
    assert!(
        candidate
            .resolved()
            .values_of(PropertyKey::GraphicBanner)
            .is_empty(),
        "suppression clears inherited banner values at this descendant only"
    );

    // A sibling branch that never suppressed still sees the public banner, and still never
    // publishes the internal logo.
    let sibling = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        declarations,
        jakarta_en_owner(),
    ))
    .expect("the jakarta branch resolves");
    assert_eq!(
        sibling.public().values_of(PropertyKey::GraphicBanner).len(),
        1
    );
    assert!(
        sibling
            .public()
            .values_of(PropertyKey::GraphicLogo)
            .is_empty()
    );
}
/// `R03C1-05` — `suppress` with no local value adds no obligation, `suppress` plus a local public
/// graphic adds exactly one, and an absent key stays distinct from an explicit `"none"`.
#[test]
fn r03c1_05_suppress_without_a_local_value_adds_no_obligation() {
    let inventory = inventory(
        vec![
            declared(
                facet_owner(),
                vec![(PropertyKey::GraphicLogo, suppress(Vec::new()))],
            ),
            declared(
                bali_owner(),
                vec![(
                    PropertyKey::GraphicBanner,
                    suppress(vec![authored(
                        entity_asset("bali/banner.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                jakarta_owner(),
                vec![(PropertyKey::GraphicFavicon, inherit(Vec::new()))],
            ),
        ],
        bali_en_owner(),
    );

    // Only the replacement declaration produced an obligation, with the local owner and index.
    assert_eq!(inventory.len(), 1);
    let only = &inventory.obligations()[0];
    assert_eq!(only.declaring_owner(), &bali_owner());
    assert_eq!(only.role(), PropertyKey::GraphicBanner);
    assert_eq!(only.local_index(), 0);
    assert_eq!(only.exposure(), Exposure::Public);
    assert_eq!(only.reference_text(), "bali/banner.png");
    assert_eq!(inventory.obligations_of(&facet_owner()).count(), 0);
    assert_eq!(inventory.obligations_of(&jakarta_owner()).count(), 0);
    // The Jakarta Page and both representations declare nothing at all.
    assert_eq!(inventory.obligations_of(&jakarta_page_owner()).count(), 0);
    assert_eq!(inventory.obligations_of(&jakarta_en_owner()).count(), 0);
}

/// `R03C1-06` — two independently authored uses of one filename stay distinct obligations, and the
/// same literal filename under `entity` and `platform` is never the same asset identity.
#[test]
fn r03c1_06_same_filename_at_two_owners_and_two_namespaces_stays_distinct() {
    let inventory = inventory(
        vec![
            declared(
                entity_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        entity_asset("brand/logo.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                bali_page_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        entity_asset("brand/logo.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
            declared(
                jakarta_page_owner(),
                vec![(
                    PropertyKey::GraphicLogo,
                    inherit(vec![authored(
                        asset(AssetNamespace::Platform, "brand/logo.png"),
                        Exposure::Public,
                    )]),
                )],
            ),
        ],
        bali_en_owner(),
    );

    assert_eq!(inventory.len(), 3);
    let rows = inventory.obligations();
    // The reference text is identical in all three, yet the owners and namespaces differ.
    for row in rows {
        assert_eq!(row.reference_text(), "brand/logo.png");
        assert_eq!(row.role(), PropertyKey::GraphicLogo);
        assert_eq!(row.local_index(), 0);
    }
    assert_eq!(rows[0].declaring_owner(), &entity_owner());
    assert_eq!(rows[0].namespace(), AssetNamespace::Entity);
    assert_eq!(rows[1].declaring_owner(), &bali_page_owner());
    assert_eq!(rows[1].namespace(), AssetNamespace::Entity);
    assert_eq!(rows[2].declaring_owner(), &jakarta_page_owner());
    assert_eq!(rows[2].namespace(), AssetNamespace::Platform);

    // No two obligations are equal, and no cross-Entity owner or substitution appears.
    assert_ne!(rows[0], rows[1]);
    assert_ne!(rows[1], rows[2]);
    assert_eq!(
        rows.iter()
            .filter(|row| row.namespace() == AssetNamespace::Platform)
            .count(),
        1
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.namespace() == AssetNamespace::Entity)
            .count(),
        2
    );
}

/// `R03C1-10` — the inventory and the candidate stay nonpublishing, repeated collection is
/// value-identical, and an empty or wholly internal authoring set is handled without inventing a
/// default or claiming that any asset exists or is safe.
#[test]
fn r03c1_10_inventory_is_nonpublishing_repeatable_and_empty_safe() {
    assert!(
        !GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY,
        "an authored inventory is never publication authority"
    );

    let graph = graph();
    let empty_candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        Vec::new(),
        bali_en_owner(),
    ))
    .expect("an empty authoring set is valid");
    let empty = collect_graphic_asset_obligations_v1(&empty_candidate).expect("collects");
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert!(!empty.grants_publication_authority());
    assert!(!empty_candidate.is_publication_authorized());

    let internal_only = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        vec![declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("bg/internal.png"),
                    Exposure::Internal,
                )]),
            )],
        )],
        bali_en_owner(),
    ))
    .expect("an internal-only authoring set is valid");
    let first = collect_graphic_asset_obligations_v1(&internal_only).expect("collects");
    let second = collect_graphic_asset_obligations_v1(&internal_only).expect("collects again");

    // An all-private declaration set still has exactly one obligation, and nothing is published.
    assert_eq!(first.len(), 1);
    assert_eq!(first, second, "repeated collection is value-identical");
    let obligation = &first.obligations()[0];
    assert_eq!(obligation.exposure(), Exposure::Internal);
    assert!(!obligation.grants_publication_authority());
    assert!(!first.grants_publication_authority());
    assert!(
        internal_only
            .public()
            .values_of(PropertyKey::GraphicBackground)
            .is_empty(),
        "an internal graphic is never projected"
    );

    // A missing file is irrelevant: the inventory attests only authored references, never bytes.
    // No filesystem, network, decoder or catalogue access exists in this boundary, so a reference
    // that certainly does not exist on disk still inventories exactly once.
    assert_eq!(obligation.reference_text(), "bg/internal.png");
}
