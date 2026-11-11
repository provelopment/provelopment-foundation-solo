//! `R03.C2` application tests: confined Entity graphic source-byte acquisition.
//!
//! Each case drives the **real** property use case and the real `R03.C1` inventory over the
//! accepted one-Entity graph, then acquires bytes through a controlled in-memory port that
//! records every call. Assertions are made on materialized owner/role/index/exposure/reference,
//! path, byte, length and identity data — never on a bool, an `is_err()` or a call count alone —
//! and the refusals assert the exact typed category as well as the count of observable effects.
//!
//! Real operating-system behaviour is not simulated here: the filesystem, link, reparse-point and
//! snapshot cases live in `solo-adapters/tests/graphic_asset_read.rs` and run on both CI hosts.

use std::cell::RefCell;

use solo_application::{
    EntityAssetPathV1, EntityAssetSourceV1, GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_ASSET_READ_VALIDATES_MEDIA, GraphicAssetObligationInventoryV1, GraphicAssetReadPort,
    GraphicAssetReadRefusalKind, GraphicAssetReadRefusalV1, GraphicAssetReadSetOutcomeV1,
    GraphicAssetSourceSetV1, MAX_GRAPHIC_SOURCE_BYTES, PhysicalIdentityV1,
    PropertyResolutionRequestV1, collect_graphic_asset_obligations_v1, fingerprint_of,
    read_graphic_asset_bytes_v1, resolve_declared_properties_v1,
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

fn home() -> PageIdV1 {
    PageIdV1::parse("home").expect("a canonical Page identifier")
}

fn locale(value: &str) -> LocaleKeyV1 {
    LocaleKeyV1::from_already_canonical(value).expect("a canonical full locale key")
}

/// One Entity, one Facet, one Site, one Page and two full-locale representations.
fn graph() -> EfsOwnershipGraph {
    let mut page = PageDraft::new(home());
    page.add_representation(locale("en-001"));
    page.add_representation(locale("de-DE"));
    let mut site = SiteDraft::new(bali());
    site.add_page(page);
    let mut main = FacetDraft::new(facet());
    main.add_site(site);
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

fn site_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::site(entity(), facet(), bali())
}

fn page_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::page(entity(), facet(), bali(), home())
}

fn en_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), bali(), home(), locale("en-001"))
}

fn de_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::representation(entity(), facet(), bali(), home(), locale("de-DE"))
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

fn platform_asset(reference: &str) -> PropertyValueV1 {
    asset(AssetNamespace::Platform, reference)
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

/// A controlled inbound-side port: it replays exact observations and records every call.
struct FakePort {
    responses: Vec<EntityAssetSourceV1>,
    calls: RefCell<Vec<String>>,
}

impl FakePort {
    fn new(responses: Vec<EntityAssetSourceV1>) -> Self {
        Self {
            responses,
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl GraphicAssetReadPort for FakePort {
    fn read_entity_asset(
        &self,
        reference: &AssetReferenceV1,
        max_bytes: u64,
    ) -> EntityAssetSourceV1 {
        assert_eq!(
            max_bytes, MAX_GRAPHIC_SOURCE_BYTES,
            "the use case always passes the documented bound"
        );
        let index = self.calls.borrow().len();
        self.calls
            .borrow_mut()
            .push(reference.reference().to_owned());
        match self.responses.get(index) {
            Some(response) => response.clone(),
            None => EntityAssetSourceV1::Refused(GraphicAssetReadRefusalV1::of(
                GraphicAssetReadRefusalKind::MissingSource,
            )),
        }
    }
}

fn identity(container: u64, object: u64) -> PhysicalIdentityV1 {
    PhysicalIdentityV1::new(container, object, 0)
}

fn present(bytes: &[u8], id: PhysicalIdentityV1) -> EntityAssetSourceV1 {
    present_with(bytes, id, bytes.len() as u64, 1, fingerprint_of(bytes))
}

fn present_with(
    bytes: &[u8],
    id: PhysicalIdentityV1,
    length: u64,
    links: u64,
    fingerprint: u64,
) -> EntityAssetSourceV1 {
    EntityAssetSourceV1::Present {
        bytes: bytes.to_vec(),
        identity: id,
        length,
        links,
        changed_nanos: (1_000, 2_000),
        fingerprint,
    }
}

fn completed(outcome: GraphicAssetReadSetOutcomeV1) -> GraphicAssetSourceSetV1 {
    match outcome {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            panic!("expected complete evidence, got refusal {refusal}")
        }
    }
}

fn refused(outcome: GraphicAssetReadSetOutcomeV1) -> (GraphicAssetReadRefusalKind, String) {
    match outcome {
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            (refusal.kind(), refusal.code().to_owned())
        }
        GraphicAssetReadSetOutcomeV1::Complete(set) => {
            panic!("expected a refusal, got {} evidence records", set.len())
        }
    }
}

/// One evidence record rendered as data a test can compare exactly.
fn rendered(
    set: &GraphicAssetSourceSetV1,
) -> Vec<(String, String, usize, String, String, String, String)> {
    set.evidence()
        .iter()
        .map(|evidence| {
            (
                evidence.obligation().declaring_owner().describe(),
                evidence.obligation().role().name().to_owned(),
                evidence.obligation().local_index(),
                evidence.obligation().exposure().as_str().to_owned(),
                evidence.obligation().namespace().as_str().to_owned(),
                evidence.obligation().reference_text().to_owned(),
                evidence.logical_path().to_owned(),
            )
        })
        .collect()
}

/// The six independently authored sources used by most cases: five EFS levels, one internal
/// value, one descendant local replacement, and two values in one site-level array.
fn standard_declarations() -> Vec<DeclaredOwnerPropertiesV1> {
    vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/entity-logo.png"),
                    Exposure::Internal,
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
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![
                    authored(entity_asset("bg/bali-1.png"), Exposure::Public),
                    authored(entity_asset("bg/bali-2.png"), Exposure::Public),
                ]),
            )],
        ),
        declared(
            page_owner(),
            vec![(
                PropertyKey::GraphicSocialPreview,
                inherit(vec![authored(
                    entity_asset("social/bali-page.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            en_owner(),
            vec![(
                PropertyKey::GraphicFavicon,
                suppress(vec![authored(
                    entity_asset("icons/favicon.png"),
                    Exposure::Public,
                )]),
            )],
        ),
    ]
}

/// The six source bodies, in inventory order.
const BODIES: [&[u8]; 6] = [
    b"entity-logo-source",
    b"facet-banner-source",
    b"bali-background-one-source",
    b"bali-background-two-source",
    b"bali-page-social-source",
    b"bali-favicon-source",
];

/// Six responses, one per obligation, with distinct physical identities.
fn standard_responses() -> Vec<EntityAssetSourceV1> {
    BODIES
        .iter()
        .enumerate()
        .map(|(index, body)| present(body, identity(7, 100 + index as u64)))
        .collect()
}

/// `R03C2-01` — the real accepted one-Entity graph yields complete ordered evidence for every
/// authored Entity value across all five levels, including the `internal` logo and the
/// descendant-suppressed local favicon, and the requested target changes no decision.
#[test]
fn r03c2_01_complete_inventory_yields_ordered_evidence_for_every_authored_value() {
    let port = FakePort::new(standard_responses());
    let collected = inventory(standard_declarations(), entity_owner());
    assert_eq!(
        collected.len(),
        6,
        "six independently authored sources exist"
    );

    let outcome = read_graphic_asset_bytes_v1(&collected, &port);
    assert_eq!(
        port.calls(),
        vec![
            "brand/entity-logo.png".to_owned(),
            "brand/facet-banner.png".to_owned(),
            "bg/bali-1.png".to_owned(),
            "bg/bali-2.png".to_owned(),
            "social/bali-page.png".to_owned(),
            "icons/favicon.png".to_owned(),
        ],
        "every obligation is read exactly once, in inventory order"
    );
    let set = completed(outcome);
    assert_eq!(set.len(), 6);
    assert!(!set.grants_publication_authority());
    assert!(!set.validates_media());

    let records = rendered(&set);
    assert_eq!(
        records,
        vec![
            (
                entity_owner().describe(),
                "graphic.logo".to_owned(),
                0usize,
                "internal".to_owned(),
                "entity".to_owned(),
                "brand/entity-logo.png".to_owned(),
                "assets/brand/entity-logo.png".to_owned(),
            ),
            (
                facet_owner().describe(),
                "graphic.banner".to_owned(),
                0,
                "public".to_owned(),
                "entity".to_owned(),
                "brand/facet-banner.png".to_owned(),
                "assets/brand/facet-banner.png".to_owned(),
            ),
            (
                site_owner().describe(),
                "graphic.background".to_owned(),
                0,
                "public".to_owned(),
                "entity".to_owned(),
                "bg/bali-1.png".to_owned(),
                "assets/bg/bali-1.png".to_owned(),
            ),
            (
                site_owner().describe(),
                "graphic.background".to_owned(),
                1,
                "public".to_owned(),
                "entity".to_owned(),
                "bg/bali-2.png".to_owned(),
                "assets/bg/bali-2.png".to_owned(),
            ),
            (
                page_owner().describe(),
                "graphic.social-preview".to_owned(),
                0,
                "public".to_owned(),
                "entity".to_owned(),
                "social/bali-page.png".to_owned(),
                "assets/social/bali-page.png".to_owned(),
            ),
            (
                en_owner().describe(),
                "graphic.favicon".to_owned(),
                0,
                "public".to_owned(),
                "entity".to_owned(),
                "icons/favicon.png".to_owned(),
                "assets/icons/favicon.png".to_owned(),
            ),
        ]
    );

    // Bytes, length and physical identity are materialized per record, not summarised once.
    for (index, evidence) in set.evidence().iter().enumerate() {
        assert_eq!(evidence.bytes(), BODIES[index]);
        assert_eq!(evidence.identity(), identity(7, 100 + index as u64));
        assert_eq!(evidence.length(), BODIES[index].len() as u64);
        assert!(!evidence.grants_publication_authority());
        assert!(!evidence.media_verified());
    }

    // A different requested target produces the same obligations and the same evidence.
    let other = inventory(standard_declarations(), de_owner());
    let other_outcome = read_graphic_asset_bytes_v1(&other, &FakePort::new(standard_responses()));
    assert_eq!(rendered(&completed(other_outcome)), records);
    // The two public constants restate the boundary the whole module depends on.
    let grants_publication = GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY;
    let validates_media = GRAPHIC_ASSET_READ_VALIDATES_MEDIA;
    assert!(!grants_publication);
    assert!(!validates_media);
}

/// `R03C2-02` — an inventory that authored nothing yields an empty complete result with zero
/// outbound calls, and an all-private inventory still reads every declared source while granting
/// no publication right.
#[test]
fn r03c2_02_empty_inventory_makes_no_calls_and_private_inventory_still_reads() {
    let port = FakePort::new(Vec::new());
    let empty = inventory(Vec::new(), entity_owner());
    assert!(empty.is_empty());
    let set = completed(read_graphic_asset_bytes_v1(&empty, &port));
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
    assert_eq!(port.calls().len(), 0, "an empty inventory opens nothing");
    assert!(!set.grants_publication_authority());

    let private = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/private-logo.png"),
                    Exposure::Internal,
                )]),
            )],
        ),
        declared(
            facet_owner(),
            vec![(
                PropertyKey::GraphicBanner,
                inherit(vec![authored(
                    entity_asset("brand/private-banner.png"),
                    Exposure::Internal,
                )]),
            )],
        ),
    ];
    let port = FakePort::new(vec![
        present(b"private-logo", identity(9, 1)),
        present(b"private-banner", identity(9, 2)),
    ]);
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(private, entity_owner()),
        &port,
    ));
    assert_eq!(
        port.calls(),
        vec![
            "brand/private-logo.png".to_owned(),
            "brand/private-banner.png".to_owned(),
        ]
    );
    assert_eq!(set.len(), 2);
    assert_eq!(
        set.evidence()[0].obligation().exposure(),
        Exposure::Internal
    );
    assert_eq!(
        set.evidence()[1].obligation().exposure(),
        Exposure::Internal
    );
    assert!(!set.grants_publication_authority());
    assert!(!set.validates_media());
}

/// `R03C2-03` — a Platform-only inventory and a mixed inventory both refuse as
/// `PlatformUnverified` before any outbound call, including a Platform value declared after an
/// Entity one and one declared `internal`.
#[test]
fn r03c2_03_platform_only_and_mixed_inventories_refuse_before_any_outbound_call() {
    let platform_only = vec![declared(
        entity_owner(),
        vec![(
            PropertyKey::GraphicLogo,
            inherit(vec![authored(
                platform_asset("brand/platform-logo.png"),
                Exposure::Public,
            )]),
        )],
    )];
    let port = FakePort::new(Vec::new());
    let (kind, code) = refused(read_graphic_asset_bytes_v1(
        &inventory(platform_only, entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::PlatformUnverified);
    assert_eq!(code, "ASSET_PLATFORM_UNVERIFIED");
    assert_eq!(port.calls().len(), 0);

    let mixed = vec![
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
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    platform_asset("bg/platform-1.png"),
                    Exposure::Internal,
                )]),
            )],
        ),
    ];
    let port = FakePort::new(standard_responses());
    let outcome = read_graphic_asset_bytes_v1(&inventory(mixed, entity_owner()), &port);
    assert_eq!(
        port.calls().len(),
        0,
        "no Entity read starts before the refusal"
    );
    match outcome {
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            assert_eq!(
                refusal.kind(),
                GraphicAssetReadRefusalKind::PlatformUnverified
            );
            assert_eq!(
                refusal.declaring_owner().map(QualifiedOwnerV1::describe),
                Some(site_owner().describe())
            );
            assert_eq!(refusal.role(), Some(PropertyKey::GraphicBackground));
            assert_eq!(refusal.local_index(), Some(0));
        }
        GraphicAssetReadSetOutcomeV1::Complete(_) => {
            panic!("a mixed inventory must never be reported as a complete Entity-only success")
        }
    }
}

/// `R03C2-04` — the same literal filename authored by two different owners stays two separately
/// accounted obligations, each read and evidenced, and an inconsistent repeat observation of one
/// reference fails the whole attempt.
#[test]
fn r03c2_04_same_filename_at_two_owners_is_two_obligations_and_reread_must_agree() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
    ];

    let port = FakePort::new(vec![
        present(b"shared-bytes", identity(11, 5)),
        present(b"shared-bytes", identity(11, 5)),
    ]);
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(set.len(), 2, "neither occurrence is deduplicated away");
    assert_eq!(
        port.calls(),
        vec!["brand/shared.png".to_owned(), "brand/shared.png".to_owned(),],
        "each authored occurrence is read"
    );
    assert_eq!(
        set.evidence()[0].obligation().declaring_owner().describe(),
        entity_owner().describe()
    );
    assert_eq!(
        set.evidence()[1].obligation().declaring_owner().describe(),
        site_owner().describe()
    );
    assert_eq!(set.evidence()[0].identity(), set.evidence()[1].identity());
    assert_eq!(set.evidence()[0].bytes(), set.evidence()[1].bytes());
    assert_eq!(set.evidence()[0].logical_path(), "assets/brand/shared.png");
    assert_eq!(set.evidence()[1].logical_path(), "assets/brand/shared.png");

    // A changed identity on the second observation of the same reference is a collision.
    let port = FakePort::new(vec![
        present(b"shared-bytes", identity(11, 5)),
        present(b"shared-bytes", identity(11, 6)),
    ]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::IdentityCollision);

    // The same identity with different content on the second observation is a change.
    let port = FakePort::new(vec![
        present(b"shared-bytes", identity(11, 5)),
        present(b"tampered-bytes", identity(11, 5)),
    ]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations, entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::ChangedDuringRead);
}

/// `R03C2-05` — every injected outbound failure produces its exact typed whole-candidate refusal,
/// a later failing suppressed/private value discards the earlier successes, and no diagnostic
/// carries a path or an authored value.
#[test]
fn r03c2_05_injected_failures_refuse_the_whole_candidate_without_partial_evidence() {
    let cases = [
        (
            GraphicAssetReadRefusalKind::MissingSource,
            "ASSET_SOURCE_MISSING",
        ),
        (
            GraphicAssetReadRefusalKind::PermissionDenied,
            "ASSET_SOURCE_PERMISSION_DENIED",
        ),
        (
            GraphicAssetReadRefusalKind::ChangedDuringRead,
            "ASSET_SOURCE_CHANGED",
        ),
        (
            GraphicAssetReadRefusalKind::UnsafeSourceLocation,
            "ASSET_SOURCE_UNSAFE_LOCATION",
        ),
        (
            GraphicAssetReadRefusalKind::Unsupported,
            "ASSET_SOURCE_UNSUPPORTED",
        ),
        (
            GraphicAssetReadRefusalKind::OtherIo,
            "ASSET_SOURCE_IO_ERROR",
        ),
    ];
    for (kind, code) in cases {
        let mut responses = standard_responses();
        if let Some(last) = responses.last_mut() {
            *last = EntityAssetSourceV1::Refused(GraphicAssetReadRefusalV1::of(kind));
        }
        let port = FakePort::new(responses);
        let (actual, actual_code) = refused(read_graphic_asset_bytes_v1(
            &inventory(standard_declarations(), entity_owner()),
            &port,
        ));
        assert_eq!(actual, kind);
        assert_eq!(actual_code, code);
        assert_eq!(
            port.calls().len(),
            6,
            "the failing last obligation is reached after the earlier reads, and its successes are discarded"
        );
    }

    let mut responses = standard_responses();
    if let Some(last) = responses.last_mut() {
        *last = EntityAssetSourceV1::Refused(GraphicAssetReadRefusalV1::of(
            GraphicAssetReadRefusalKind::MissingSource,
        ));
    }
    let outcome = read_graphic_asset_bytes_v1(
        &inventory(standard_declarations(), entity_owner()),
        &FakePort::new(responses),
    );
    match outcome {
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            assert_eq!(
                refusal.declaring_owner().map(QualifiedOwnerV1::describe),
                Some(en_owner().describe())
            );
            assert_eq!(refusal.role(), Some(PropertyKey::GraphicFavicon));
            assert_eq!(refusal.local_index(), Some(0));
            let text = format!("{refusal}");
            assert!(!text.contains("assets/"), "a diagnostic carries no path");
            assert!(
                !text.contains("favicon"),
                "a diagnostic carries no authored value"
            );
        }
        GraphicAssetReadSetOutcomeV1::Complete(_) => {
            panic!("a failing obligation must never yield complete evidence")
        }
    }
}

/// `R03C2-06` — the 20 MiB bound is exact: the bound itself succeeds, bound + 1 is refused
/// rather than truncated, a reported length beyond it is refused, an empty source is faithfully
/// acquired, and a short transfer is a change rather than a shorter body.
#[test]
fn r03c2_06_bound_is_exactly_twenty_mib_and_oversize_is_never_truncated() {
    assert_eq!(MAX_GRAPHIC_SOURCE_BYTES, 20 * 1024 * 1024);
    let declarations = vec![declared(
        entity_owner(),
        vec![(
            PropertyKey::GraphicLogo,
            inherit(vec![authored(
                entity_asset("brand/bound.png"),
                Exposure::Public,
            )]),
        )],
    )];

    let at_bound = vec![0_u8; MAX_GRAPHIC_SOURCE_BYTES as usize];
    let port = FakePort::new(vec![present(&at_bound, identity(3, 1))]);
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(set.evidence()[0].length(), MAX_GRAPHIC_SOURCE_BYTES);
    assert_eq!(
        set.evidence()[0].bytes().len(),
        MAX_GRAPHIC_SOURCE_BYTES as usize
    );
    assert!(!set.evidence()[0].media_verified());

    let over_bound = vec![0_u8; MAX_GRAPHIC_SOURCE_BYTES as usize + 1];
    let port = FakePort::new(vec![present(&over_bound, identity(3, 2))]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::OversizeSource);

    let port = FakePort::new(vec![present_with(
        b"short",
        identity(3, 3),
        MAX_GRAPHIC_SOURCE_BYTES + 1,
        1,
        7,
    )]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::OversizeSource);

    let port = FakePort::new(vec![present(b"", identity(3, 4))]);
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(declarations.clone(), entity_owner()),
        &port,
    ));
    assert_eq!(set.evidence()[0].length(), 0);
    assert!(set.evidence()[0].bytes().is_empty());
    assert!(!set.validates_media());
    assert!(!set.evidence()[0].media_verified());

    let port = FakePort::new(vec![present_with(b"short", identity(3, 5), 9, 1, 7)]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations, entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::ChangedDuringRead);
}

/// `R03C2-07` — a link count other than exactly one is refused, two distinct logical Entity
/// references observed at one physical identity are refused even when the port reports a benign
/// link count, and the permitted same-reference case is not falsely treated as a conflict.
#[test]
fn r03c2_07_link_count_and_physical_identity_collisions_are_refused() {
    let single = vec![declared(
        entity_owner(),
        vec![(
            PropertyKey::GraphicLogo,
            inherit(vec![authored(
                entity_asset("brand/logo.png"),
                Exposure::Public,
            )]),
        )],
    )];
    let port = FakePort::new(vec![present_with(b"linked", identity(4, 1), 6, 2, 9)]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(single, entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::IdentityCollision);

    let distinct_paths = vec![declared(
        entity_owner(),
        vec![
            (
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/one.png"),
                    Exposure::Public,
                )]),
            ),
            (
                PropertyKey::GraphicBanner,
                inherit(vec![authored(
                    entity_asset("brand/two.png"),
                    Exposure::Public,
                )]),
            ),
        ],
    )];
    let port = FakePort::new(vec![
        present(b"same-object", identity(4, 2)),
        present(b"same-object", identity(4, 2)),
    ]);
    let (kind, _) = refused(read_graphic_asset_bytes_v1(
        &inventory(distinct_paths, entity_owner()),
        &port,
    ));
    assert_eq!(
        kind,
        GraphicAssetReadRefusalKind::IdentityCollision,
        "one physical object cannot satisfy two distinct logical asset paths"
    );
}

/// `R03C2-09` — the portable asset boundary refuses spellings the domain reference admits but
/// the portable filename rule forbids, refuses them without repairing the authored text, admits
/// every allowed lowercase image label, and confirms that an extension alone proves no MIME.
#[test]
fn r03c2_09_portable_boundary_refuses_prohibited_spellings_without_repair() {
    for reference in [
        "a.png",
        "brand/logo.png",
        "bg/deep-nested_1.jpeg",
        "i/f.webp",
        "x/y.avif",
        "j/photo.jpg",
    ] {
        let value = AssetReferenceV1::parse(AssetNamespace::Entity, reference)
            .expect("the domain admits this spelling");
        let path = EntityAssetPathV1::from_reference(&value)
            .expect("the portable boundary admits this spelling");
        assert_eq!(path.logical_path(), format!("assets/{reference}"));
        assert_eq!(
            path.components().first().map(String::as_str),
            Some("assets")
        );
        assert_eq!(path.component_slices().len(), path.components().len());
        assert_eq!(path.byte_length(), path.logical_path().len());
    }

    let long_component = format!("a{}.png", "b".repeat(80));
    let too_deep = "a/a/a/a/a/a/a/a.png";
    let prohibited = [
        "Brand/Logo.png",
        "brand/con.png",
        "brand/aux.jpeg",
        "brand/1logo.png",
        "brand/_logo.png",
        "brand/logo..png",
        long_component.as_str(),
        too_deep,
    ];
    for reference in prohibited {
        let value = AssetReferenceV1::parse(AssetNamespace::Entity, reference)
            .expect("the domain reference admits this spelling");
        let refusal = EntityAssetPathV1::from_reference(&value)
            .expect_err("the portable boundary must refuse this spelling");
        assert_eq!(
            refusal.kind(),
            GraphicAssetReadRefusalKind::UnsafeAssetReference
        );
        assert_eq!(refusal.code(), "ASSET_REFERENCE_UNSAFE");
        assert_eq!(
            value.reference(),
            reference,
            "the authored text is refused, never rewritten"
        );
    }

    // Spellings the domain reference itself already refuses can never reach the boundary.
    for reference in [
        "brand/logo.txt",
        "brand//logo.png",
        "/brand/logo.png",
        "brand/logo.PNG",
        "brand/logo.png/",
        ".hidden/logo.png",
    ] {
        assert!(
            AssetReferenceV1::parse(AssetNamespace::Entity, reference).is_err(),
            "{reference} is not an admissible authored reference"
        );
    }

    // A Platform reference is not an Entity asset path at all.
    let platform = AssetReferenceV1::parse(AssetNamespace::Platform, "brand/logo.png")
        .expect("a typed platform reference");
    let refusal = EntityAssetPathV1::from_reference(&platform)
        .expect_err("a platform reference is unverifiable in this gate");
    assert_eq!(
        refusal.kind(),
        GraphicAssetReadRefusalKind::PlatformUnverified
    );

    // An admitted extension is evidence about a name, never about the media.
    let value = AssetReferenceV1::parse(AssetNamespace::Entity, "brand/photo.png")
        .expect("a valid entity reference");
    let port = FakePort::new(vec![present(b"not-a-png-at-all", identity(12, 1))]);
    let declarations = vec![declared(
        entity_owner(),
        vec![(
            PropertyKey::GraphicLogo,
            inherit(vec![authored(
                PropertyValueV1::graphic(value),
                Exposure::Public,
            )]),
        )],
    )];
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(declarations, entity_owner()),
        &port,
    ));
    assert_eq!(set.evidence()[0].bytes(), b"not-a-png-at-all");
    assert!(!set.evidence()[0].media_verified());
    assert!(!set.validates_media());
    assert!(!set.grants_publication_authority());
}

/// `S065R02-04` / `R03C2-04` — the same logical reference authored twice must agree in its
/// **exact bytes**. Two equal-length bodies with a deliberately identical supplied fingerprint
/// are a change, not a consistent source, and no partial evidence escapes.
#[test]
fn r03c2_04b_same_reference_with_equal_fingerprint_and_different_bytes_is_refused() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
    ];

    // Same object, same length, same deliberately supplied fingerprint, different bytes. A
    // fingerprint-only comparison would accept this; the exact-byte rule must not.
    let deliberately_equal_fingerprint = 42;
    let port = FakePort::new(vec![
        present_with(
            b"shared-one",
            identity(21, 7),
            10,
            1,
            deliberately_equal_fingerprint,
        ),
        present_with(
            b"shared-two",
            identity(21, 7),
            10,
            1,
            deliberately_equal_fingerprint,
        ),
    ]);
    assert_eq!(
        b"shared-one".len(),
        b"shared-two".len(),
        "the two bodies differ only in content"
    );

    let outcome = read_graphic_asset_bytes_v1(&inventory(declarations, entity_owner()), &port);
    assert_eq!(port.calls().len(), 2, "both authored occurrences are read");
    match outcome {
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            assert_eq!(
                refusal.kind(),
                GraphicAssetReadRefusalKind::ChangedDuringRead
            );
            assert_eq!(refusal.code(), "ASSET_SOURCE_CHANGED");
            // The second obligation is the one that detected the inconsistency.
            assert_eq!(
                refusal.declaring_owner().map(QualifiedOwnerV1::describe),
                Some(site_owner().describe())
            );
            assert_eq!(refusal.role(), Some(PropertyKey::GraphicBackground));
            assert_eq!(refusal.local_index(), Some(0));
            let text = format!("{refusal}");
            assert!(!text.contains("assets/"), "a diagnostic carries no path");
            assert!(
                !text.contains("shared"),
                "a diagnostic carries no authored value"
            );
            assert!(
                !text.contains("bytes"),
                "a diagnostic carries no source bytes"
            );
        }
        GraphicAssetReadSetOutcomeV1::Complete(set) => panic!(
            "different bytes under one logical reference must never be accepted, got {} records",
            set.len()
        ),
    }
}

/// `S065R02-04` / `R03C2-04` — identical bytes, identity, length and fingerprint remain two
/// separately ordered observations of the same authored filename, never deduplicated.
#[test]
fn r03c2_04b_same_reference_with_equal_bytes_stays_two_ordered_observations() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
    ];

    let port = FakePort::new(vec![
        present(b"shared-same", identity(22, 3)),
        present(b"shared-same", identity(22, 3)),
    ]);
    let set = completed(read_graphic_asset_bytes_v1(
        &inventory(declarations, entity_owner()),
        &port,
    ));
    assert_eq!(
        set.len(),
        2,
        "two authored obligations, not one deduplicated value"
    );
    assert_eq!(port.calls().len(), 2);
    assert_eq!(
        set.evidence()[0].obligation().declaring_owner().describe(),
        entity_owner().describe()
    );
    assert_eq!(
        set.evidence()[1].obligation().declaring_owner().describe(),
        site_owner().describe()
    );
    assert_eq!(set.evidence()[0].bytes(), b"shared-same");
    assert_eq!(set.evidence()[1].bytes(), b"shared-same");
    assert_eq!(set.evidence()[0].identity(), set.evidence()[1].identity());
    assert_eq!(set.evidence()[0].logical_path(), "assets/brand/shared.png");
    assert_eq!(set.evidence()[1].logical_path(), "assets/brand/shared.png");
}

/// `S065R02-04` / `R03C2-04` — the retained fingerprint check is still real: equal bytes with an
/// inconsistent fingerprint is a refused repeat observation rather than a silent acceptance.
#[test]
fn r03c2_04b_same_reference_with_equal_bytes_and_different_fingerprint_is_inconsistent() {
    let declarations = vec![
        declared(
            entity_owner(),
            vec![(
                PropertyKey::GraphicLogo,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
        declared(
            site_owner(),
            vec![(
                PropertyKey::GraphicBackground,
                inherit(vec![authored(
                    entity_asset("brand/shared.png"),
                    Exposure::Public,
                )]),
            )],
        ),
    ];

    let port = FakePort::new(vec![
        present_with(b"shared-same", identity(23, 8), 11, 1, 111),
        present_with(b"shared-same", identity(23, 8), 11, 1, 222),
    ]);
    let (kind, code) = refused(read_graphic_asset_bytes_v1(
        &inventory(declarations, entity_owner()),
        &port,
    ));
    assert_eq!(kind, GraphicAssetReadRefusalKind::ChangedDuringRead);
    assert_eq!(code, "ASSET_SOURCE_CHANGED");
}
