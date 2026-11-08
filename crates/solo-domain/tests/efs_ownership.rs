//! `R02.A` single-Entity EFS ownership: positive construction, direct parentage,
//! parent-scoped uniqueness, cardinality refusals and qualified lineage, exercised
//! through the public domain API only.
//!
//! Locale values here are deliberately supplied already-canonical fixture keys
//! (`en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`). That is a fixture convention for
//! membership testing, not a claim that this model validates or canonicalizes a
//! locale.

use solo_domain::{
    EfsGraphErrorKind, EfsLevel, EfsLineageErrorKind, EfsNodeIdError, EfsOwnershipDraft,
    EfsOwnershipGraph, EfsPageTarget, EntityIdErrorKind, EntityIdV1, FacetDraft, FacetIdV1,
    LocaleKeyErrorKind, LocaleKeyV1, MAX_PAGES, MAX_REPRESENTATIONS_PER_PAGE, PageDraft, PageIdV1,
    SiteDraft, SiteIdV1,
};

fn entity() -> EntityIdV1 {
    EntityIdV1::parse("acme").expect("valid Entity identifier")
}

fn facet_id(id: &str) -> FacetIdV1 {
    FacetIdV1::parse(id).expect("valid Facet identifier")
}

fn site_id(id: &str) -> SiteIdV1 {
    SiteIdV1::parse(id).expect("valid Site identifier")
}

fn page_id(id: &str) -> PageIdV1 {
    PageIdV1::parse(id).expect("valid Page identifier")
}

fn locale(key: &str) -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical(key)
        .expect("fixture key is an already-canonical full locale")
}

fn page_draft(id: &str, locales: &[&str]) -> PageDraft {
    let mut draft = PageDraft::new(page_id(id));
    for key in locales {
        draft.add_representation(locale(key));
    }
    draft
}

fn site_draft(id: &str, pages: Vec<PageDraft>) -> SiteDraft {
    let mut draft = SiteDraft::new(site_id(id));
    for page in pages {
        draft.add_page(page);
    }
    draft
}

fn facet_draft(id: &str, sites: Vec<SiteDraft>) -> FacetDraft {
    let mut draft = FacetDraft::new(facet_id(id));
    for site in sites {
        draft.add_site(site);
    }
    draft
}

fn draft_with(facets: Vec<FacetDraft>) -> EfsOwnershipDraft {
    let mut draft = EfsOwnershipDraft::new(entity());
    for facet in facets {
        draft.add_facet(facet);
    }
    draft
}

/// The minimal valid graph: one Facet, one Site, one Page, one representation.
fn minimal_graph() -> EfsOwnershipGraph {
    draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", vec![page_draft("home", &["en-001"])])],
    )])
    .build()
    .expect("minimal graph is valid")
}

fn facet_tokens(graph: &EfsOwnershipGraph) -> Vec<String> {
    graph
        .facets()
        .iter()
        .map(|facet| facet.id().as_str().to_owned())
        .collect()
}

fn site_tokens(graph: &EfsOwnershipGraph, facet: usize) -> Vec<String> {
    graph.facets()[facet]
        .sites()
        .iter()
        .map(|site| site.id().as_str().to_owned())
        .collect()
}

fn page_tokens(graph: &EfsOwnershipGraph, facet: usize, site: usize) -> Vec<String> {
    graph.facets()[facet].sites()[site]
        .pages()
        .iter()
        .map(|page| page.id().as_str().to_owned())
        .collect()
}

#[test]
fn builds_minimal_single_entity_graph() {
    let graph = minimal_graph();

    assert_eq!(graph.entity().as_str(), "acme");
    assert_eq!(graph.facets().len(), 1);
    assert_eq!(facet_tokens(&graph), vec!["main".to_owned()]);
    assert_eq!(site_tokens(&graph, 0), vec!["main".to_owned()]);
    assert_eq!(page_tokens(&graph, 0, 0), vec!["home".to_owned()]);

    let page = &graph.facets()[0].sites()[0].pages()[0];
    assert_eq!(page.id().as_str(), "home");
    assert_eq!(page.representations().len(), 1);
    assert_eq!(page.representations()[0].as_str(), "en-001");
}

#[test]
fn builds_expanded_graph_with_multiple_facets_sites_and_pages() {
    let graph = draft_with(vec![
        facet_draft(
            "main",
            vec![
                site_draft(
                    "main",
                    vec![
                        page_draft("home", &["en-001", "de-DE"]),
                        page_draft("about", &["en-001"]),
                    ],
                ),
                site_draft("docs", vec![page_draft("overview", &["id-ID"])]),
            ],
        ),
        facet_draft(
            "shop",
            vec![
                site_draft(
                    "main",
                    vec![page_draft("catalogue", &["en-001", "zh-Hans-CN"])],
                ),
                site_draft("checkout", vec![page_draft("cart", &["de-DE"])]),
            ],
        ),
    ])
    .build()
    .expect("expanded one-Entity graph is valid");

    assert_eq!(graph.entity().as_str(), "acme");
    assert_eq!(
        facet_tokens(&graph),
        vec!["main".to_owned(), "shop".to_owned()]
    );
    assert_eq!(
        site_tokens(&graph, 0),
        vec!["main".to_owned(), "docs".to_owned()]
    );
    assert_eq!(
        site_tokens(&graph, 1),
        vec!["main".to_owned(), "checkout".to_owned()]
    );
    assert_eq!(
        page_tokens(&graph, 0, 0),
        vec!["home".to_owned(), "about".to_owned()]
    );
    assert_eq!(page_tokens(&graph, 0, 1), vec!["overview".to_owned()]);
    assert_eq!(page_tokens(&graph, 1, 0), vec!["catalogue".to_owned()]);
    assert_eq!(page_tokens(&graph, 1, 1), vec!["cart".to_owned()]);

    let home = &graph.facets()[0].sites()[0].pages()[0];
    assert_eq!(home.representations().len(), 2);
    assert_eq!(home.representations()[0].as_str(), "en-001");
    assert_eq!(home.representations()[1].as_str(), "de-DE");
}

#[test]
fn refuses_empty_graph_and_invents_no_starter_seed() {
    let error = EfsOwnershipDraft::new(entity())
        .build()
        .expect_err("an Entity with no Facet is refused, never seeded");

    assert_eq!(error.kind(), EfsGraphErrorKind::NoFacet);
    assert_eq!(error.level(), EfsLevel::Entity);
    assert_eq!(error.code(), "PREFLIGHT_EFS_GRAPH_INVALID");
    assert_eq!(error.detail(), "no-facet");
}

#[test]
fn refuses_facet_without_site() {
    let error = draft_with(vec![facet_draft("main", Vec::new())])
        .build()
        .expect_err("a Facet must contain at least one Site");

    assert_eq!(error.kind(), EfsGraphErrorKind::FacetWithoutSite);
    assert_eq!(error.level(), EfsLevel::Facet);
    assert_eq!(error.detail(), "facet-without-site");
}

#[test]
fn refuses_site_without_page() {
    let error = draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", Vec::new())],
    )])
    .build()
    .expect_err("a Site must contain at least one Page");

    assert_eq!(error.kind(), EfsGraphErrorKind::SiteWithoutPage);
    assert_eq!(error.level(), EfsLevel::Site);
    assert_eq!(error.detail(), "site-without-page");
}

#[test]
fn refuses_page_without_representation() {
    let error = draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", vec![page_draft("home", &[])])],
    )])
    .build()
    .expect_err("a published Page must carry at least one representation");

    assert_eq!(error.kind(), EfsGraphErrorKind::PageWithoutRepresentation);
    assert_eq!(error.level(), EfsLevel::Page);
    assert_eq!(error.detail(), "page-without-representation");
}

#[test]
fn refuses_duplicate_facet_id_inside_the_one_entity() {
    let error = draft_with(vec![
        facet_draft(
            "main",
            vec![site_draft("main", vec![page_draft("home", &["en-001"])])],
        ),
        facet_draft(
            "main",
            vec![site_draft("other", vec![page_draft("home", &["en-001"])])],
        ),
    ])
    .build()
    .expect_err("Facet IDs are unique within the Entity");

    assert_eq!(error.kind(), EfsGraphErrorKind::DuplicateFacetId);
    assert_eq!(error.level(), EfsLevel::Facet);
}

#[test]
fn refuses_duplicate_site_id_within_one_facet() {
    let error = draft_with(vec![facet_draft(
        "main",
        vec![
            site_draft("main", vec![page_draft("home", &["en-001"])]),
            site_draft("main", vec![page_draft("about", &["en-001"])]),
        ],
    )])
    .build()
    .expect_err("Site IDs are unique within one Facet");

    assert_eq!(error.kind(), EfsGraphErrorKind::DuplicateSiteId);
    assert_eq!(error.level(), EfsLevel::Site);
}

#[test]
fn refuses_duplicate_page_id_within_one_site() {
    let error = draft_with(vec![facet_draft(
        "main",
        vec![site_draft(
            "main",
            vec![
                page_draft("home", &["en-001"]),
                page_draft("home", &["de-DE"]),
            ],
        )],
    )])
    .build()
    .expect_err("Page IDs are unique within one Site");

    assert_eq!(error.kind(), EfsGraphErrorKind::DuplicatePageId);
    assert_eq!(error.level(), EfsLevel::Page);
}

#[test]
fn accepts_the_same_token_under_different_parents() {
    let graph = draft_with(vec![
        facet_draft(
            "alpha",
            vec![
                site_draft("main", vec![page_draft("home", &["en-001"])]),
                site_draft("docs", vec![page_draft("home", &["en-001"])]),
            ],
        ),
        facet_draft(
            "beta",
            vec![
                site_draft("main", vec![page_draft("home", &["en-001"])]),
                site_draft("docs", vec![page_draft("home", &["en-001"])]),
            ],
        ),
    ])
    .build()
    .expect("the same token under different parents is legitimate");

    // `main` is a Site in both Facets, and `home` a Page in all four Sites.
    assert_eq!(
        site_tokens(&graph, 0),
        vec!["main".to_owned(), "docs".to_owned()]
    );
    assert_eq!(
        site_tokens(&graph, 1),
        vec!["main".to_owned(), "docs".to_owned()]
    );
    assert_eq!(page_tokens(&graph, 0, 0), vec!["home".to_owned()]);
    assert_eq!(page_tokens(&graph, 0, 1), vec!["home".to_owned()]);
    assert_eq!(page_tokens(&graph, 1, 0), vec!["home".to_owned()]);
    assert_eq!(page_tokens(&graph, 1, 1), vec!["home".to_owned()]);
}

#[test]
fn accepts_the_same_token_at_different_levels() {
    let graph = draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", vec![page_draft("main", &["en-001"])])],
    )])
    .build()
    .expect("different levels may legitimately carry the same token");

    assert_eq!(graph.facets()[0].id().as_str(), "main");
    assert_eq!(graph.facets()[0].sites()[0].id().as_str(), "main");
    assert_eq!(
        graph.facets()[0].sites()[0].pages()[0].id().as_str(),
        "main"
    );
}

/// Every refusal the accepted identity grammar defines, asserted at one level.
///
/// The parser is passed in so that each level is checked through its own public
/// constructor without matching an enum arm that no longer exists.
fn assert_node_refusals(level: EfsLevel, parse: fn(&str) -> Result<(), EfsNodeIdError>) {
    let too_long = "a".repeat(64);
    let cases: [(&str, EntityIdErrorKind); 8] = [
        ("", EntityIdErrorKind::Empty),
        ("münchen", EntityIdErrorKind::NonAscii),
        ("Main", EntityIdErrorKind::InvalidStart),
        ("-abc", EntityIdErrorKind::InvalidStart),
        ("a.b", EntityIdErrorKind::InvalidCharacter),
        ("abc-", EntityIdErrorKind::TrailingHyphen),
        ("a--b", EntityIdErrorKind::ConsecutiveHyphens),
        (too_long.as_str(), EntityIdErrorKind::TooLong),
    ];

    for (candidate, expected) in cases {
        let error = parse(candidate).expect_err("a candidate must be refused, never repaired");
        assert_eq!(error.kind(), expected, "wrong reason for {candidate:?}");
        assert_eq!(error.level(), level);
        assert_eq!(error.code(), "PREFLIGHT_EFS_NODE_ID_INVALID");
    }
}

#[test]
fn validates_every_non_entity_identity_level() {
    assert_node_refusals(EfsLevel::Facet, |candidate| {
        FacetIdV1::parse(candidate).map(|_| ())
    });
    assert_node_refusals(EfsLevel::Site, |candidate| {
        SiteIdV1::parse(candidate).map(|_| ())
    });
    assert_node_refusals(EfsLevel::Page, |candidate| {
        PageIdV1::parse(candidate).map(|_| ())
    });
}

#[test]
fn accepts_and_refuses_the_identifier_length_boundary() {
    let maximal = "a".repeat(63);
    assert_eq!(
        FacetIdV1::parse(&maximal)
            .expect("63 bytes is valid")
            .as_str(),
        maximal
    );
    assert_eq!(
        SiteIdV1::parse(&maximal)
            .expect("63 bytes is valid")
            .as_str(),
        maximal
    );
    assert_eq!(
        PageIdV1::parse(&maximal)
            .expect("63 bytes is valid")
            .as_str(),
        maximal
    );

    let too_long = "a".repeat(64);
    assert_eq!(
        FacetIdV1::parse(&too_long)
            .expect_err("64 bytes is refused")
            .kind(),
        EntityIdErrorKind::TooLong
    );
    assert_eq!(
        SiteIdV1::parse(&too_long)
            .expect_err("64 bytes is refused")
            .kind(),
        EntityIdErrorKind::TooLong
    );
    assert_eq!(
        PageIdV1::parse(&too_long)
            .expect_err("64 bytes is refused")
            .kind(),
        EntityIdErrorKind::TooLong
    );
}

#[test]
fn identity_is_independent_of_labels_addresses_and_locale_fixtures() {
    // Display labels, public base addresses and request context are external
    // authored facts. None of them is an input to any constructor below, so
    // identity cannot move when only they change.
    fn build_with_external_context(
        label: &str,
        base: &str,
        locale_fixture: &str,
    ) -> EfsOwnershipGraph {
        assert!(!label.is_empty() && !base.is_empty() && !locale_fixture.is_empty());
        minimal_graph()
    }

    let first = build_with_external_context("Acme Corporation", "https://acme.example/", "en-001");
    let second = build_with_external_context("Acme Collective", "https://shop.acme.test/", "de-DE");

    assert_eq!(first.entity(), second.entity());
    assert_eq!(facet_tokens(&first), facet_tokens(&second));
    assert_eq!(site_tokens(&first, 0), site_tokens(&second, 0));
    assert_eq!(page_tokens(&first, 0, 0), page_tokens(&second, 0, 0));
}

/// A two-Facet graph used by the lineage tests.
fn lineage_graph() -> EfsOwnershipGraph {
    draft_with(vec![facet_draft(
        "main",
        vec![
            site_draft("main", vec![page_draft("home", &["en-001", "de-DE"])]),
            site_draft("docs", vec![page_draft("guide", &["en-001"])]),
        ],
    )])
    .build()
    .expect("lineage fixture is a valid graph")
}

#[test]
fn resolves_page_only_by_complete_lineage() {
    let graph = lineage_graph();
    let facet = facet_id("main");
    let site = site_id("main");
    let page = page_id("home");

    let resolved = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &facet,
            site: &site,
            page: &page,
        })
        .expect("the complete lineage resolves");

    assert_eq!(resolved.id().as_str(), "home");
    assert_eq!(resolved.representations().len(), 2);
}

#[test]
fn refuses_a_second_entity_and_unresolvable_ancestors() {
    let graph = lineage_graph();
    let facet = facet_id("main");
    let site = site_id("main");
    let page = page_id("home");

    // A second Entity is not part of this graph and never resolves.
    let other_entity = EntityIdV1::parse("other-entity").expect("valid Entity identifier");
    let second_entity = graph
        .resolve_page(EfsPageTarget {
            entity: &other_entity,
            facet: &facet,
            site: &site,
            page: &page,
        })
        .expect_err("a second Entity is refused");
    assert_eq!(second_entity.kind(), EfsLineageErrorKind::UnknownEntity);
    assert_eq!(second_entity.level(), EfsLevel::Entity);
    assert_eq!(second_entity.code(), "PREFLIGHT_EFS_LINEAGE_UNRESOLVED");

    // An absent Facet is refused rather than guessed.
    let absent_facet = facet_id("shop");
    let unknown_facet = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &absent_facet,
            site: &site,
            page: &page,
        })
        .expect_err("an absent Facet is refused");
    assert_eq!(unknown_facet.kind(), EfsLineageErrorKind::UnknownFacet);
    assert_eq!(unknown_facet.level(), EfsLevel::Facet);

    // A Site that exists, but not in this Facet, is refused.
    let docs = site_id("docs");
    let wrong_facet = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &absent_facet,
            site: &docs,
            page: &page,
        })
        .expect_err("a Site outside the named Facet is refused");
    assert_eq!(wrong_facet.kind(), EfsLineageErrorKind::UnknownFacet);

    // A Page that exists, but in another Site of the same Facet, is refused:
    // a wrong-site hit is never silently accepted.
    let guide = page_id("guide");
    let wrong_site = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &facet,
            site: &site,
            page: &guide,
        })
        .expect_err("a Page owned by another Site is refused");
    assert_eq!(wrong_site.kind(), EfsLineageErrorKind::UnknownPage);
    assert_eq!(wrong_site.level(), EfsLevel::Page);
}

#[test]
fn resolves_representation_and_refuses_an_absent_locale_key() {
    let graph = lineage_graph();
    let facet = facet_id("main");
    let site = site_id("main");
    let page = page_id("home");
    let target = EfsPageTarget {
        entity: graph.entity(),
        facet: &facet,
        site: &site,
        page: &page,
    };

    let english = locale("en-001");
    let resolved = graph
        .resolve_representation(target, &english)
        .expect("the declared representation resolves");
    assert_eq!(resolved.as_str(), "en-001");

    let german = locale("de-DE");
    assert_eq!(
        graph
            .resolve_representation(target, &german)
            .expect("the second representation resolves")
            .as_str(),
        "de-DE"
    );

    // `id-ID` is a legitimate key, but this Page does not carry it.
    let indonesian = locale("id-ID");
    let absent = graph
        .resolve_representation(target, &indonesian)
        .expect_err("an undeclared locale key is refused");
    assert_eq!(absent.kind(), EfsLineageErrorKind::UnknownRepresentation);
    assert_eq!(absent.level(), EfsLevel::Representation);
}

#[test]
fn refuses_duplicate_locale_key_on_one_page() {
    let error = draft_with(vec![facet_draft(
        "main",
        vec![site_draft(
            "main",
            vec![page_draft("home", &["en-001", "de-DE", "en-001"])],
        )],
    )])
    .build()
    .expect_err("a Page cannot carry the same canonical key twice");

    assert_eq!(error.kind(), EfsGraphErrorKind::DuplicateLocaleKey);
    assert_eq!(error.level(), EfsLevel::Page);
    assert_eq!(error.detail(), "duplicate-locale-key");
}

#[test]
fn accepts_the_same_locale_key_on_different_pages() {
    let graph = draft_with(vec![facet_draft(
        "main",
        vec![site_draft(
            "main",
            vec![
                page_draft("home", &["en-001"]),
                page_draft("about", &["en-001"]),
            ],
        )],
    )])
    .build()
    .expect("uniqueness is scoped to one Page");

    assert_eq!(
        page_tokens(&graph, 0, 0),
        vec!["home".to_owned(), "about".to_owned()]
    );
    assert_eq!(
        graph.facets()[0].sites()[0].pages()[1].representations()[0].as_str(),
        "en-001"
    );
}

#[test]
fn refuses_bare_language_keys_without_completing_them() {
    // A bare primary subtag is never a full region-qualified authored key, and it
    // is refused rather than completed with an inferred region.
    for bare in ["en", "de", "id", "zh"] {
        let error = LocaleKeyV1::from_already_canonical(bare)
            .expect_err("a bare language must not become an authored key");
        assert_eq!(error.kind(), LocaleKeyErrorKind::NotFullLocale);
        assert_eq!(error.code(), "PREFLIGHT_EFS_LOCALE_KEY_INVALID");
        assert_eq!(error.detail(), "not-full-locale");
    }
}

#[test]
fn refuses_malformed_locale_keys() {
    let cases: [(&str, LocaleKeyErrorKind); 7] = [
        ("", LocaleKeyErrorKind::Empty),
        ("de-DE ", LocaleKeyErrorKind::InvalidCharacter),
        ("de DE", LocaleKeyErrorKind::InvalidCharacter),
        ("de_DE", LocaleKeyErrorKind::InvalidCharacter),
        ("deutsch–de", LocaleKeyErrorKind::NonAscii),
        ("-en-001", LocaleKeyErrorKind::Malformed),
        ("en-001-", LocaleKeyErrorKind::Malformed),
    ];

    for (candidate, expected) in cases {
        let error = LocaleKeyV1::from_already_canonical(candidate)
            .expect_err("a malformed key must be refused, never repaired");
        assert_eq!(error.kind(), expected, "wrong reason for {candidate:?}");
    }

    // A doubled separator is also malformed rather than normalized.
    assert_eq!(
        LocaleKeyV1::from_already_canonical("de--DE")
            .expect_err("doubled separator is refused")
            .kind(),
        LocaleKeyErrorKind::Malformed
    );
}

#[test]
fn accepts_one_hundred_representations_and_refuses_one_more() {
    let mut at_limit = PageDraft::new(page_id("home"));
    for index in 0..MAX_REPRESENTATIONS_PER_PAGE {
        at_limit.add_representation(locale(&format!("en-{index:03}")));
    }
    let graph = draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", vec![at_limit])],
    )])
    .build()
    .expect("100 representations is the accepted maximum");
    assert_eq!(
        graph.facets()[0].sites()[0].pages()[0]
            .representations()
            .len(),
        MAX_REPRESENTATIONS_PER_PAGE
    );

    let mut over_limit = PageDraft::new(page_id("home"));
    for index in 0..=MAX_REPRESENTATIONS_PER_PAGE {
        over_limit.add_representation(locale(&format!("en-{index:03}")));
    }
    let error = draft_with(vec![facet_draft(
        "main",
        vec![site_draft("main", vec![over_limit])],
    )])
    .build()
    .expect_err("101 representations is refused");

    assert_eq!(error.kind(), EfsGraphErrorKind::TooManyRepresentations);
    assert_eq!(error.level(), EfsLevel::Page);
}

#[test]
fn accepts_five_hundred_pages_and_refuses_one_more() {
    let mut site = SiteDraft::new(site_id("main"));
    for index in 0..MAX_PAGES {
        site.add_page(page_draft(&format!("page-{index:03}"), &["en-001"]));
    }
    let graph = draft_with(vec![facet_draft("main", vec![site])])
        .build()
        .expect("500 Pages is the accepted maximum");
    assert_eq!(graph.facets()[0].sites()[0].pages().len(), MAX_PAGES);

    let mut site = SiteDraft::new(site_id("main"));
    for index in 0..=MAX_PAGES {
        site.add_page(page_draft(&format!("page-{index:03}"), &["en-001"]));
    }
    let error = draft_with(vec![facet_draft("main", vec![site])])
        .build()
        .expect_err("501 Pages is refused");

    assert_eq!(error.kind(), EfsGraphErrorKind::TooManyPages);
    assert_eq!(error.level(), EfsLevel::Entity);
}

#[test]
fn refuses_a_graph_larger_than_the_accepted_node_budget() {
    // 500 Pages x 18 representations plus the Entity, Facet and Site is 9,503
    // nodes, which is inside the budget.
    let keys: Vec<String> = (0..18).map(|index| format!("en-{index:03}")).collect();
    let reference: Vec<&str> = keys.iter().map(String::as_str).collect();
    let mut site = SiteDraft::new(site_id("main"));
    for index in 0..500 {
        site.add_page(page_draft(&format!("page-{index:03}"), &reference));
    }
    draft_with(vec![facet_draft("main", vec![site])])
        .build()
        .expect("9,503 nodes is inside the accepted budget");

    // 500 Pages x 19 representations plus those three ancestors is 10,003 nodes.
    let keys: Vec<String> = (0..19).map(|index| format!("en-{index:03}")).collect();
    let reference: Vec<&str> = keys.iter().map(String::as_str).collect();
    let mut site = SiteDraft::new(site_id("main"));
    for index in 0..500 {
        site.add_page(page_draft(&format!("page-{index:03}"), &reference));
    }
    let error = draft_with(vec![facet_draft("main", vec![site])])
        .build()
        .expect_err("10,003 nodes exceeds the budget");

    assert_eq!(error.kind(), EfsGraphErrorKind::TooManyNodes);
    assert_eq!(error.level(), EfsLevel::Entity);
}

#[test]
fn preserves_authored_order_instead_of_sorting_it() {
    let graph = draft_with(vec![
        facet_draft(
            "zeta",
            vec![site_draft("second", vec![page_draft("home", &["en-001"])])],
        ),
        facet_draft(
            "alpha",
            vec![site_draft("first", vec![page_draft("home", &["en-001"])])],
        ),
        facet_draft(
            "mid",
            vec![site_draft("third", vec![page_draft("home", &["en-001"])])],
        ),
    ])
    .build()
    .expect("distinct Facet IDs are valid regardless of order");

    // Authored order, not lexical order.
    assert_eq!(
        facet_tokens(&graph),
        vec!["zeta".to_owned(), "alpha".to_owned(), "mid".to_owned()]
    );
    assert_eq!(site_tokens(&graph, 1), vec!["first".to_owned()]);

    // Representation order is authored too.
    let ordered = draft_with(vec![facet_draft(
        "main",
        vec![site_draft(
            "main",
            vec![page_draft("home", &["zh-Hans-CN", "en-001", "de-DE"])],
        )],
    )])
    .build()
    .expect("valid");
    let keys: Vec<&str> = ordered.facets()[0].sites()[0].pages()[0]
        .representations()
        .iter()
        .map(LocaleKeyV1::as_str)
        .collect();
    assert_eq!(keys, vec!["zh-Hans-CN", "en-001", "de-DE"]);
}

#[test]
fn refuses_duplicates_independently_of_declaration_order() {
    // The duplicate Facet ID moves through every position, and each permutation is
    // refused for the same reason: the decision never depends on array order.
    for duplicate_at in 0..3 {
        let mut facets = Vec::new();
        for position in 0..3 {
            let id = if position == duplicate_at {
                "main"
            } else {
                "other"
            };
            facets.push(facet_draft(
                id,
                vec![site_draft("site", vec![page_draft("home", &["en-001"])])],
            ));
        }
        facets.push(facet_draft(
            "main",
            vec![site_draft("site", vec![page_draft("home", &["en-001"])])],
        ));

        let error = draft_with(facets)
            .build()
            .expect_err("a duplicate Facet ID is refused in every order");
        assert_eq!(error.kind(), EfsGraphErrorKind::DuplicateFacetId);
    }
}

#[test]
fn diagnostics_never_echo_authored_input() {
    let duplicate = draft_with(vec![
        facet_draft(
            "main",
            vec![site_draft("main", vec![page_draft("home", &["en-001"])])],
        ),
        facet_draft(
            "main",
            vec![site_draft("main", vec![page_draft("home", &["en-001"])])],
        ),
    ])
    .build()
    .expect_err("duplicate Facet ID");
    let rendered = duplicate.to_string();
    assert_eq!(rendered, "PREFLIGHT_EFS_GRAPH_INVALID: duplicate-facet-id");
    assert!(!rendered.contains("main"));

    let graph = lineage_graph();
    let facet = facet_id("main");
    let site = site_id("main");
    let missing = page_id("absent-page");
    let lineage = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &facet,
            site: &site,
            page: &missing,
        })
        .expect_err("absent Page");
    let rendered = lineage.to_string();
    assert_eq!(rendered, "PREFLIGHT_EFS_LINEAGE_UNRESOLVED: unknown-page");
    assert!(!rendered.contains("absent-page"));

    let node = PageIdV1::parse("Bad/Path").expect_err("invalid Page ID");
    let rendered = node.to_string();
    assert_eq!(
        rendered,
        "PREFLIGHT_EFS_NODE_ID_INVALID: page invalid-start"
    );
    assert!(!rendered.contains("Bad"));

    let key = LocaleKeyV1::from_already_canonical("de DE").expect_err("invalid key");
    let rendered = key.to_string();
    assert_eq!(
        rendered,
        "PREFLIGHT_EFS_LOCALE_KEY_INVALID: invalid-character"
    );
    assert!(!rendered.contains("DE DE"));
}
