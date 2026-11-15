//! Pure positive and negative tests for the closed `1.0.0` Platform catalogue contract and its inert
//! V4 fact binding (`SOLO-WO-118 R12`).
//!
//! **Every record used here is a deliberate fake.** The V4 records are produced by the real V4 use
//! case over a *fake* legacy port that fabricates a still observation; they are labelled fakes and
//! prove the catalogue matcher's data handling only. No confined worker, no Platform directory and no
//! product artwork is involved, and nothing here asserts or implies legal approval of any asset.

use std::cell::RefCell;

use solo_application::{
    EntityAssetSourceV1, GraphicAssetObligationInventoryV1, GraphicAssetReadPort,
    GraphicAssetReadRefusalKind, GraphicAssetReadRefusalV1, GraphicAssetReadSetOutcomeV1,
    GraphicImageFormatV1, GraphicImageInspectPortOutcomeV3, GraphicImageInspectPortV3,
    GraphicImageInspectRequestV3, GraphicImageInspectionOutcomeV4, GraphicImageInspectionV4,
    GraphicImageObservationV3, GraphicImageWorkerObservationV1, MediaWorkerPortV1,
    MediaWorkerRefusalKindV1, MediaWorkerRequestV1, MediaWorkerRunV1,
    PLATFORM_ASSET_LIBRARY_VERSION_V1, PhysicalIdentityV1, PlatformAssetCatalogueEntryV1,
    PlatformAssetCatalogueV1, PlatformAssetDeclarationV1, PlatformAssetLicenceV1,
    PlatformAssetProvenanceV1, PlatformCatalogueBindingScopeV1, PlatformCatalogueBindingV1,
    PlatformCatalogueMatchRefusalV1, PlatformCatalogueMatchV1, PlatformCatalogueRefusalV1,
    PropertyResolutionRequestV1, bind_catalogue_to_v4_evidence_v1,
    collect_graphic_asset_obligations_v1, fingerprint_of, inspect_graphic_image_source_set_v4,
    match_catalogue_entry_to_v4_evidence_v1, read_graphic_asset_bytes_v1,
    resolve_declared_properties_v1, sha256_media_worker_bytes_v1,
};
use solo_domain::{
    AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CurrentContextV1,
    DeclaredOwnerPropertiesV1, EfsOwnershipDraft, EntityIdV1, Exposure, FacetDraft, Inheritance,
    LocaleKeyV1, PageDraft, PropertyDeclarationV1, PropertyKey, PropertyValueV1, QualifiedOwnerV1,
    SiteDraft,
};

fn entity_owner() -> QualifiedOwnerV1 {
    QualifiedOwnerV1::entity(EntityIdV1::parse("example").expect("entity"))
}

fn inventory(entries: &[(&str, Exposure)]) -> GraphicAssetObligationInventoryV1 {
    let values = entries
        .iter()
        .map(|(reference, exposure)| {
            AuthoredPropertyValueV1::new(
                PropertyValueV1::graphic(
                    AssetReferenceV1::parse(AssetNamespace::Entity, reference).expect("reference"),
                ),
                *exposure,
            )
            .expect("authored value")
        })
        .collect::<Vec<_>>();
    let declaration =
        PropertyDeclarationV1::new(Inheritance::Inherit, CurrentContextV1::Values(values));
    let declared = DeclaredOwnerPropertiesV1::new(
        entity_owner(),
        vec![(PropertyKey::GraphicBackground, declaration)],
    )
    .expect("declaration set");
    let mut page = PageDraft::new(solo_domain::PageIdV1::parse("home").expect("page"));
    page.add_representation(LocaleKeyV1::from_already_canonical("en-001").expect("locale"));
    let mut site = SiteDraft::new(solo_domain::SiteIdV1::parse("bali").expect("site"));
    site.add_page(page);
    let mut facet = FacetDraft::new(solo_domain::FacetIdV1::parse("main").expect("facet"));
    facet.add_site(site);
    let mut draft = EfsOwnershipDraft::new(EntityIdV1::parse("example").expect("entity"));
    draft.add_facet(facet);
    let graph = draft.build().expect("one-Entity graph");
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        vec![declared],
        entity_owner(),
    ))
    .expect("candidate");
    collect_graphic_asset_obligations_v1(&candidate).expect("inventory")
}

struct FakeReadPort {
    bodies: Vec<Vec<u8>>,
    calls: RefCell<usize>,
}

impl GraphicAssetReadPort for FakeReadPort {
    fn read_entity_asset(
        &self,
        _reference: &AssetReferenceV1,
        _max_bytes: u64,
    ) -> EntityAssetSourceV1 {
        let index = *self.calls.borrow();
        *self.calls.borrow_mut() = index + 1;
        match self.bodies.get(index) {
            Some(body) => EntityAssetSourceV1::Present {
                bytes: body.clone(),
                identity: PhysicalIdentityV1::new(1, u64::try_from(index).unwrap_or(0) + 7, 0),
                length: body.len() as u64,
                links: 1,
                changed_nanos: (1_000, 2_000),
                fingerprint: fingerprint_of(body),
            },
            None => EntityAssetSourceV1::Refused(GraphicAssetReadRefusalV1::of(
                GraphicAssetReadRefusalKind::MissingSource,
            )),
        }
    }
}

/// A fake V3 port that fabricates one still observation. Never a real worker.
struct FakeLegacyPort;

impl GraphicImageInspectPortV3 for FakeLegacyPort {
    fn inspect_v3(
        &self,
        request: &GraphicImageInspectRequestV3,
    ) -> GraphicImageInspectPortOutcomeV3 {
        GraphicImageInspectPortOutcomeV3::Inspected(Box::new(GraphicImageObservationV3::Still(
            GraphicImageWorkerObservationV1 {
                confirmed_format: GraphicImageFormatV1::Png,
                observed_mime: "image/png",
                source_digest: *request.source_digest(),
                width: 2,
                height: 2,
                decoded_bytes: 16,
                decoded_digest: sha256_media_worker_bytes_v1(b"a fabricated plane"),
                worker_pid: 4242,
                limits_verified: true,
                cleanup_verified: true,
                wall_ms: 3,
            },
        )))
    }
}

/// A fake AVIF port: it never runs and always reports no trustworthy result.
struct FakeAvifPort;

impl MediaWorkerPortV1 for FakeAvifPort {
    fn inspect_avif(&self, _request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1 {
        MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerUnavailable)
    }
}

/// One fake V4 PNG record over fabricated bytes: a test double, not confined-worker evidence.
fn fake_v4_record(body: u8) -> GraphicImageInspectionV4 {
    let bytes = vec![body; 64];
    let set = match read_graphic_asset_bytes_v1(
        &inventory(&[("media/hero.png", Exposure::Public)]),
        &FakeReadPort {
            bodies: vec![bytes],
            calls: RefCell::new(0),
        },
    ) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => panic!("read refused: {refusal:?}"),
    };
    match inspect_graphic_image_source_set_v4(&set, &FakeLegacyPort, &FakeAvifPort) {
        GraphicImageInspectionOutcomeV4::Complete(complete) => complete.inspections()[0].clone(),
        GraphicImageInspectionOutcomeV4::Refused(refusal) => {
            panic!("fake record refused: {refusal:?}")
        }
    }
}

fn licence() -> PlatformAssetLicenceV1 {
    PlatformAssetLicenceV1::new(
        "Synthetic-Test-Only-1.0",
        "Synthetic test owner (not a rights assertion)",
        "synthetic/test-notice.txt",
    )
    .expect("synthetic licence shape")
}

fn provenance() -> PlatformAssetProvenanceV1 {
    PlatformAssetProvenanceV1::new(
        "Synthetic test bytes, generated in the test process",
        "synthetic/provenance-notice.txt",
    )
    .expect("synthetic provenance shape")
}

fn entry(
    id: &str,
    sha: &str,
    mime: &str,
    width: u32,
    height: u32,
) -> PlatformAssetCatalogueEntryV1 {
    PlatformAssetCatalogueEntryV1::new(
        &PlatformAssetDeclarationV1::of(id, sha, "image", mime, width, height),
        licence(),
        provenance(),
    )
    .expect("a well-formed synthetic entry")
}

fn entry_of(record: &GraphicImageInspectionV4, id: &str) -> PlatformAssetCatalogueEntryV1 {
    entry(id, &record.source_digest().to_hex(), record.mime(), 2, 2)
}

#[test]
fn a_synthetic_entry_carries_no_authority() {
    let record = fake_v4_record(0x5A);
    let entry = entry_of(&record, "icons/example.png");
    assert_eq!(entry.namespace(), AssetNamespace::Platform);
    assert_ne!(entry.namespace(), AssetNamespace::Entity);
    assert_eq!(entry.id(), "icons/example.png");
    assert_eq!(entry.mime(), "image/png");
    assert_eq!(entry.media_class().as_str(), "image");
    assert!(!entry.licence().rights_reviewed());
    let catalogue = PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, vec![entry])
        .expect("one well-formed entry");
    assert_eq!(catalogue.version(), PLATFORM_ASSET_LIBRARY_VERSION_V1);
    assert_eq!(catalogue.len(), 1);
    assert!(!catalogue.grants_publication_authority());
    assert!(!catalogue.proves_media_correctness());
    assert!(!catalogue.rights_reviewed());
    assert!(!catalogue.platform_source_readable());
    assert!(!catalogue.derived_output_verified());
    assert!(!catalogue.release_ready());
}

#[test]
fn the_entry_constructor_refuses_every_out_of_contract_field() {
    use PlatformCatalogueRefusalV1 as Refusal;
    let sha = "a".repeat(64);
    let check = |id: &str, digest: &str, class: &str, mime: &str, w: u32, h: u32, want: Refusal| {
        let outcome = PlatformAssetCatalogueEntryV1::new(
            &PlatformAssetDeclarationV1::of(id, digest, class, mime, w, h),
            licence(),
            provenance(),
        );
        assert_eq!(outcome.err(), Some(want), "case {id:?} {mime:?}");
    };
    check(
        "../escape.png",
        &sha,
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidId,
    );
    check(
        "/absolute.png",
        &sha,
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidId,
    );
    check(
        "platform/hero.png",
        &sha,
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidId,
    );
    check(
        "icons/nul.png",
        &sha,
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidId,
    );
    check(
        "icons/ICON.PNG",
        &sha,
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidId,
    );
    check(
        "icons/example.png",
        &"A".repeat(64),
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidDigest,
    );
    check(
        "icons/example.png",
        &"0".repeat(64),
        "image",
        "image/png",
        2,
        2,
        Refusal::InvalidDigest,
    );
    check(
        "icons/example.png",
        &sha,
        "video",
        "image/png",
        2,
        2,
        Refusal::InvalidMediaClass,
    );
    check(
        "icons/example.png",
        &sha,
        "image",
        "text/vtt",
        2,
        2,
        Refusal::UnsupportedMediaType,
    );
    check(
        "icons/example.png",
        &sha,
        "image",
        "image/jpeg",
        2,
        2,
        Refusal::MimeExtensionMismatch,
    );
    check(
        "icons/example.png",
        &sha,
        "image",
        "image/png",
        0,
        2,
        Refusal::InvalidDimensions,
    );
    check(
        "icons/example.png",
        &sha,
        "image",
        "image/png",
        8193,
        1,
        Refusal::InvalidDimensions,
    );
    check(
        "icons/example.png",
        &sha,
        "image",
        "image/png",
        8192,
        8192,
        Refusal::InvalidDimensions,
    );
}

#[test]
fn the_catalogue_container_refuses_version_empty_and_duplicate_shapes() {
    use PlatformCatalogueRefusalV1 as Refusal;
    let record = fake_v4_record(0x5A);
    let one = entry_of(&record, "icons/example.png");
    assert_eq!(
        PlatformAssetCatalogueV1::new("1.0.1", vec![one.clone()]).err(),
        Some(Refusal::VersionMismatch)
    );
    assert_eq!(
        PlatformAssetCatalogueV1::new("", vec![one.clone()]).err(),
        Some(Refusal::VersionMismatch)
    );
    assert_eq!(
        PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, Vec::new()).err(),
        Some(Refusal::EmptyCatalogue)
    );
    assert_eq!(
        PlatformAssetCatalogueV1::new(
            PLATFORM_ASSET_LIBRARY_VERSION_V1,
            vec![one.clone(), one.clone()]
        )
        .err(),
        Some(Refusal::DuplicateId)
    );
    // A case-colliding identifier is refused even though the exact ids differ.
    let upper = entry("icons/Example.png", &"b".repeat(64), "image/png", 2, 2);
    let lower = entry("icons/example.png", &"c".repeat(64), "image/png", 2, 2);
    assert_eq!(
        PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, vec![upper, lower]).err(),
        Some(Refusal::CaseCollidingIdentifier)
    );
    // A duplicate digest alias is refused so provenance cannot be shared ambiguously.
    let first = entry("icons/one.png", &"d".repeat(64), "image/png", 2, 2);
    let second = entry("icons/two.png", &"d".repeat(64), "image/png", 2, 2);
    assert_eq!(
        PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, vec![first, second]).err(),
        Some(Refusal::DuplicateDigest)
    );
    // The accepted shape preserves declared order and refuses case-folded lookup.
    let first = entry("icons/one.png", &"d".repeat(64), "image/png", 2, 2);
    let second = entry("icons/two.png", &"e".repeat(64), "image/png", 2, 2);
    let catalogue =
        PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, vec![first, second])
            .expect("two distinct entries");
    let ids: Vec<&str> = catalogue.entries().iter().map(|item| item.id()).collect();
    assert_eq!(ids, vec!["icons/one.png", "icons/two.png"]);
    assert!(catalogue.exact_entry("icons/one.png").is_some());
    assert!(catalogue.exact_entry("icons/ONE.png").is_none());
    assert!(catalogue.exact_entry("one.png").is_none());
}

#[test]
fn the_inert_matcher_accepts_matching_facts_and_refuses_every_mismatch() {
    use PlatformCatalogueMatchRefusalV1 as Refusal;
    use PlatformCatalogueMatchV1 as Match;
    let record = fake_v4_record(0x5A);
    let scope = PlatformCatalogueBindingScopeV1::ContentFactComparisonOnly;
    let exact = entry_of(&record, "icons/example.png");
    assert_eq!(
        match_catalogue_entry_to_v4_evidence_v1(&exact, &record, scope),
        Match::MatchedForReview
    );

    // A different source digest never matches, whatever else agrees.
    let forged = entry("icons/example.png", &"f".repeat(64), "image/png", 2, 2);
    assert_eq!(
        match_catalogue_entry_to_v4_evidence_v1(&forged, &record, scope),
        Match::Refused(Refusal::SourceDigestMismatch)
    );
    // A declared geometry that disagrees with the observed record is refused.
    let wrong_shape = entry(
        "icons/example.png",
        &record.source_digest().to_hex(),
        "image/png",
        3,
        2,
    );
    assert_eq!(
        match_catalogue_entry_to_v4_evidence_v1(&wrong_shape, &record, scope),
        Match::Refused(Refusal::DimensionsMismatch)
    );
    // The real Platform call site cannot be served in R12.
    assert_eq!(
        match_catalogue_entry_to_v4_evidence_v1(
            &exact,
            &record,
            PlatformCatalogueBindingScopeV1::PlatformSource
        ),
        Match::Refused(Refusal::PlatformSourceUnavailable)
    );
}

#[test]
fn a_late_mismatch_discards_the_whole_binding() {
    use PlatformCatalogueMatchRefusalV1 as Refusal;
    let first = fake_v4_record(0x5A);
    let second = fake_v4_record(0x6B);
    let scope = PlatformCatalogueBindingScopeV1::ContentFactComparisonOnly;
    let records = vec![first.clone(), second.clone()];
    let good = vec![
        entry_of(&first, "icons/one.png"),
        entry_of(&second, "icons/two.png"),
    ];
    let catalogue =
        PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, good).expect("catalogue");
    let binding = bind_catalogue_to_v4_evidence_v1(&catalogue, &records, scope);
    assert_eq!(
        binding,
        PlatformCatalogueBindingV1::CompleteForReview { matched: 2 }
    );
    assert!(!binding.grants_publication_authority());
    assert!(!binding.rights_reviewed());
    assert!(!binding.platform_source_readable());

    // The second declared entry is forged: the whole binding refuses and retains nothing.
    let forged = vec![
        entry_of(&first, "icons/one.png"),
        entry("icons/two.png", &"9".repeat(64), "image/png", 2, 2),
    ];
    let catalogue = PlatformAssetCatalogueV1::new(PLATFORM_ASSET_LIBRARY_VERSION_V1, forged)
        .expect("catalogue");
    assert_eq!(
        bind_catalogue_to_v4_evidence_v1(&catalogue, &records, scope),
        PlatformCatalogueBindingV1::Refused {
            position: 1,
            kind: Refusal::SourceDigestMismatch,
        }
    );

    // One declared identifier without a corresponding record is a count mismatch, never a fallback.
    let catalogue = PlatformAssetCatalogueV1::new(
        PLATFORM_ASSET_LIBRARY_VERSION_V1,
        vec![
            entry_of(&first, "icons/one.png"),
            entry_of(&second, "icons/two.png"),
        ],
    )
    .expect("catalogue");
    assert_eq!(
        bind_catalogue_to_v4_evidence_v1(&catalogue, &records[..1], scope),
        PlatformCatalogueBindingV1::Refused {
            position: 0,
            kind: Refusal::CatalogueCountMismatch,
        }
    );
    // A Platform-scoped binding is refused outright.
    assert_eq!(
        bind_catalogue_to_v4_evidence_v1(
            &catalogue,
            &records,
            PlatformCatalogueBindingScopeV1::PlatformSource
        ),
        PlatformCatalogueBindingV1::Refused {
            position: 0,
            kind: Refusal::PlatformSourceUnavailable,
        }
    );
}

#[test]
fn the_public_diagnostics_stay_path_and_value_free() {
    use PlatformCatalogueRefusalV1 as Refusal;
    let rendered = format!(
        "{:?} {} {} {} {}",
        Refusal::InvalidId,
        Refusal::VersionMismatch.token(),
        Refusal::MissingRightsEvidence,
        PlatformCatalogueMatchRefusalV1::SourceDigestMismatch,
        PlatformCatalogueMatchRefusalV1::PlatformSourceUnavailable.token()
    );
    for forbidden in ["icons/", "Synthetic", "synthetic/", "holder", ".png"] {
        assert!(
            !rendered.contains(forbidden),
            "the diagnostic leaked {forbidden:?}: {rendered}"
        );
    }
}

/// One inventory whose single obligation names a **Platform** reference.
fn platform_inventory(reference: &str) -> GraphicAssetObligationInventoryV1 {
    let owner = entity_owner();
    let value = AuthoredPropertyValueV1::new(
        PropertyValueV1::graphic(
            AssetReferenceV1::parse(AssetNamespace::Platform, reference)
                .expect("a valid Platform reference"),
        ),
        Exposure::Public,
    )
    .expect("authored value");
    let declaration =
        PropertyDeclarationV1::new(Inheritance::Inherit, CurrentContextV1::Values(vec![value]));
    let declared = DeclaredOwnerPropertiesV1::new(
        owner.clone(),
        vec![(PropertyKey::GraphicFavicon, declaration)],
    )
    .expect("declaration set");
    let mut page = PageDraft::new(solo_domain::PageIdV1::parse("home").expect("page"));
    page.add_representation(LocaleKeyV1::from_already_canonical("en-001").expect("locale"));
    let mut site = SiteDraft::new(solo_domain::SiteIdV1::parse("bali").expect("site"));
    site.add_page(page);
    let mut facet = FacetDraft::new(solo_domain::FacetIdV1::parse("main").expect("facet"));
    facet.add_site(site);
    let mut draft = EfsOwnershipDraft::new(EntityIdV1::parse("example").expect("entity"));
    draft.add_facet(facet);
    let graph = draft.build().expect("one-Entity graph");
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        vec![declared],
        owner,
    ))
    .expect("candidate");
    collect_graphic_asset_obligations_v1(&candidate).expect("inventory")
}

#[test]
fn the_real_read_use_case_refuses_a_platform_obligation_before_any_source_read() {
    let inventory = platform_inventory("icon.png");
    assert_eq!(inventory.len(), 1, "one Platform obligation");
    let port = FakeReadPort {
        bodies: vec![vec![0x5Au8; 32]],
        calls: RefCell::new(0),
    };
    match read_graphic_asset_bytes_v1(&inventory, &port) {
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => assert_eq!(
            format!("{:?}", refusal.kind()),
            "PlatformUnverified",
            "the accepted read refuses a Platform obligation"
        ),
        GraphicAssetReadSetOutcomeV1::Complete(set) => panic!(
            "the Platform namespace was read through the Entity route: {} sources",
            set.len()
        ),
    }
    assert_eq!(
        *port.calls.borrow(),
        0,
        "the refusal precedes every source read"
    );
}
