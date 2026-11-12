//! `R03.C3` application tests: the trusted-parent AVIF worker boundary.
//!
//! Each case drives the **real** `R03.C1` inventory and the real `R03.C2` confined read use case
//! over the accepted one-Entity graph, so the source set a worker sees is produced by the accepted
//! pipeline rather than hand-built. The worker itself is a controlled in-memory port that records
//! every call and replays exactly the observation or refusal a test asks for.
//!
//! Assertions compare materialized owner/role/index, lengths, digests and geometry — never a bare
//! `is_err()` — and every hostile case names its expected stable refusal category.
//!
//! Nothing here proves media: every positive case is a fabricated protocol fixture, so pixel
//! correctness, colour semantics, orientation, MIME/magic agreement, AVIF conformance, host
//! isolation and publication authority all remain unverified.

use std::cell::RefCell;

use solo_application::{
    EntityAssetSourceV1, GraphicAssetObligationInventoryV1, GraphicAssetReadPort,
    GraphicAssetReadRefusalKind, GraphicAssetReadRefusalV1, GraphicAssetReadSetOutcomeV1,
    GraphicAssetSourceSetV1, MAX_GRAPHIC_SOURCE_BYTES, MEDIA_WORKER_MAX_SOURCE_BYTES_V1,
    MEDIA_WORKER_PROTOCOL_VERSION_V1, MediaWorkerBoundaryOutcomeV1, MediaWorkerColorModelV1,
    MediaWorkerDigestV1, MediaWorkerEnvelopeV1, MediaWorkerExitV1, MediaWorkerFormatV1,
    MediaWorkerLimitsV1, MediaWorkerMimeV1, MediaWorkerObservationV1, MediaWorkerOrientationV1,
    MediaWorkerPlaneClaimV1, MediaWorkerPortV1, MediaWorkerRefusalKindV1, MediaWorkerRequestV1,
    MediaWorkerRunV1, MediaWorkerStatusV1, PhysicalIdentityV1, PropertyResolutionRequestV1,
    collect_graphic_asset_obligations_v1, fingerprint_of, inspect_media_worker_source_set_v1,
    media_worker_output_sha256_v1, read_graphic_asset_bytes_v1, resolve_declared_properties_v1,
    sha256_media_worker_bytes_v1,
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

fn entity_asset(reference: &str, exposure: Exposure) -> AuthoredPropertyValueV1 {
    AuthoredPropertyValueV1::new(
        PropertyValueV1::graphic(
            AssetReferenceV1::parse(AssetNamespace::Entity, reference)
                .expect("a valid asset reference"),
        ),
        exposure,
    )
    .expect("a valid authored value")
}

/// One Entity-level graphic array carrying exactly the given references and exposures.
fn inventory(entries: &[(&str, Exposure)]) -> GraphicAssetObligationInventoryV1 {
    let values = entries
        .iter()
        .map(|(reference, exposure)| entity_asset(reference, *exposure))
        .collect::<Vec<_>>();
    let declaration =
        PropertyDeclarationV1::new(Inheritance::Inherit, CurrentContextV1::Values(values));
    let declared = DeclaredOwnerPropertiesV1::new(
        entity_owner(),
        vec![(PropertyKey::GraphicLogo, declaration)],
    )
    .expect("a valid declaration set");
    let graph = graph();
    let candidate = resolve_declared_properties_v1(PropertyResolutionRequestV1::new(
        &graph,
        vec![declared],
        entity_owner(),
    ))
    .expect("a valid whole-graph property candidate");
    collect_graphic_asset_obligations_v1(&candidate).expect("the inventory collects")
}

/// A controlled read port: it replays exact acquired bodies and records every call.
struct FakeReadPort {
    bodies: Vec<Vec<u8>>,
    calls: RefCell<usize>,
}

impl FakeReadPort {
    fn new(bodies: Vec<Vec<u8>>) -> Self {
        Self {
            bodies,
            calls: RefCell::new(0),
        }
    }
}

impl GraphicAssetReadPort for FakeReadPort {
    fn read_entity_asset(
        &self,
        _reference: &AssetReferenceV1,
        max_bytes: u64,
    ) -> EntityAssetSourceV1 {
        assert_eq!(
            max_bytes, MAX_GRAPHIC_SOURCE_BYTES,
            "the read use case always passes the documented bound"
        );
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

/// Drives the accepted confined read use case to produce a real source set.
fn source_set(entries: &[(&str, Exposure, Vec<u8>)]) -> GraphicAssetSourceSetV1 {
    let declarations = entries
        .iter()
        .map(|(reference, exposure, _)| (*reference, *exposure))
        .collect::<Vec<_>>();
    let bodies = entries.iter().map(|(_, _, body)| body.clone()).collect();
    let inventory = inventory(&declarations);
    assert_eq!(
        inventory.len(),
        entries.len(),
        "every authored source is one obligation"
    );
    match read_graphic_asset_bytes_v1(&inventory, &FakeReadPort::new(bodies)) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            panic!("expected acquired source bytes, got refusal {refusal}")
        }
    }
}

/// A controlled worker port: it replays exact runs and records every call it receives.
struct FakeWorkerPort {
    responses: Vec<MediaWorkerRunV1>,
    calls: RefCell<Vec<(u64, String)>>,
}

impl FakeWorkerPort {
    fn new(responses: Vec<MediaWorkerRunV1>) -> Self {
        Self {
            responses,
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<(u64, String)> {
        self.calls.borrow().clone()
    }
}

impl MediaWorkerPortV1 for FakeWorkerPort {
    fn inspect_avif(&self, request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1 {
        assert!(
            request.source_length() <= MEDIA_WORKER_MAX_SOURCE_BYTES_V1,
            "the parent gates the source bound before calling the port"
        );
        assert_eq!(
            request.limits(),
            MediaWorkerLimitsV1::reference(),
            "the request always carries the fixed reference profile"
        );
        let index = self.calls.borrow().len();
        self.calls
            .borrow_mut()
            .push((request.source_length(), request.source_sha256().to_hex()));
        match self.responses.get(index) {
            Some(response) => response.clone(),
            None => MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerUnavailable),
        }
    }
}

/// The mutable fields of one fabricated envelope claim.
///
/// Tests start from [`valid_claim`] and change exactly the field under test, so each hostile case
/// differs from the accepted baseline in one documented way.
struct Claim {
    protocol_version: u32,
    status: MediaWorkerStatusV1,
    format: MediaWorkerFormatV1,
    mime: MediaWorkerMimeV1,
    source_length: u64,
    source_sha256: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_model: MediaWorkerColorModelV1,
    has_alpha: bool,
    is_grid: bool,
    is_sequence: bool,
    orientation: MediaWorkerOrientationV1,
    planes: Vec<MediaWorkerPlaneClaimV1>,
    output_sha256: MediaWorkerDigestV1,
}

impl Claim {
    fn build(&self) -> MediaWorkerEnvelopeV1 {
        MediaWorkerEnvelopeV1::new(
            self.protocol_version,
            self.status,
            self.format,
            self.mime,
            self.source_length,
            self.source_sha256,
            self.width,
            self.height,
            self.bit_depth,
            self.color_model,
            self.has_alpha,
            self.is_grid,
            self.is_sequence,
            self.orientation,
            self.planes.clone(),
            self.output_sha256,
        )
    }
}

/// Three deterministic planes for a 32×32 YUV claim: 1024 + 256 + 256 bytes.
fn yuv_planes() -> Vec<Vec<u8>> {
    vec![
        vec![0x11u8; 32 * 32],
        vec![0x22u8; 16 * 16],
        vec![0x33u8; 16 * 16],
    ]
}

/// The plane claims that exactly describe [`yuv_planes`].
fn yuv_claims(planes: &[Vec<u8>]) -> Vec<MediaWorkerPlaneClaimV1> {
    vec![
        MediaWorkerPlaneClaimV1::new(0, 32, 32, 1024, sha256_media_worker_bytes_v1(&planes[0])),
        MediaWorkerPlaneClaimV1::new(1, 16, 16, 256, sha256_media_worker_bytes_v1(&planes[1])),
        MediaWorkerPlaneClaimV1::new(2, 16, 16, 256, sha256_media_worker_bytes_v1(&planes[2])),
    ]
}

/// A claim that is internally consistent with the given source bytes and planes.
fn valid_claim(source: &[u8], planes: &[Vec<u8>]) -> Claim {
    let claims = yuv_claims(planes);
    Claim {
        protocol_version: MEDIA_WORKER_PROTOCOL_VERSION_V1,
        status: MediaWorkerStatusV1::Decoded,
        format: MediaWorkerFormatV1::Avif,
        mime: MediaWorkerMimeV1::ImageAvif,
        source_length: source.len() as u64,
        source_sha256: sha256_media_worker_bytes_v1(source),
        width: 32,
        height: 32,
        bit_depth: 8,
        color_model: MediaWorkerColorModelV1::Yuv,
        has_alpha: false,
        is_grid: false,
        is_sequence: false,
        orientation: MediaWorkerOrientationV1::Identity,
        output_sha256: media_worker_output_sha256_v1(&claims, planes),
        planes: claims,
    }
}

/// A host observation that reports every positive fact.
fn observation(claim: &Claim, planes: Vec<Vec<u8>>) -> MediaWorkerObservationV1 {
    MediaWorkerObservationV1::new(
        true,
        MediaWorkerExitV1::Normal,
        true,
        true,
        false,
        MediaWorkerLimitsV1::reference(),
        claim.build(),
        planes,
    )
}

/// The checked envelopes rendered as data a test can compare exactly.
fn checked_summary(
    outcome: &MediaWorkerBoundaryOutcomeV1,
) -> Vec<(String, usize, u32, u32, usize)> {
    outcome
        .checked()
        .expect("expected checked envelopes")
        .iter()
        .map(|record| {
            (
                record.obligation().role().name().to_owned(),
                record.obligation().local_index(),
                record.width(),
                record.height(),
                record.plane_count(),
            )
        })
        .collect()
}

/// The stable refusal token and the authored index it names.
fn refusal_summary(outcome: &MediaWorkerBoundaryOutcomeV1) -> (String, Option<usize>) {
    let refusal = outcome.refusal().expect("expected a refusal");
    (refusal.code().to_owned(), refusal.local_index())
}

/// A small, deterministic "source" body. The bytes are never parsed: they only need to be bytes.
fn source_body(seed: u8) -> Vec<u8> {
    let mut body = vec![seed; 299];
    body[0] = 0x00;
    body[4] = b'f';
    body[8] = b'a';
    body
}

#[test]
fn positive_structure_checks_one_envelope_and_grants_nothing() {
    let source = source_body(0x21);
    let set = source_set(&[("media/hero.avif", Exposure::Public, source.clone())]);
    let planes = yuv_planes();
    let claim = valid_claim(&source, &planes);
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &claim,
        planes.clone(),
    ))]);

    let outcome = inspect_media_worker_source_set_v1(&set, &port);

    assert_eq!(
        checked_summary(&outcome),
        vec![(PropertyKey::GraphicLogo.name().to_owned(), 0, 32, 32, 3)],
        "one AVIF obligation yields exactly one checked envelope"
    );
    let record = &outcome.checked().expect("checked envelopes")[0];
    assert!(!record.grants_publication_authority());
    assert!(!record.proves_media_correctness());
    assert_eq!(record.source_length(), source.len() as u64);
    assert_eq!(
        record.source_sha256(),
        sha256_media_worker_bytes_v1(&source),
        "the retained digest is the parent's own rehash of its owned bytes"
    );
    assert_eq!(record.bit_depth(), 8);
    assert_eq!(record.color_model(), MediaWorkerColorModelV1::Yuv);
    assert_eq!(record.plane_count(), 3);
    assert_eq!(record.output_sha256(), claim.output_sha256);
    assert_eq!(
        port.calls(),
        vec![(
            source.len() as u64,
            sha256_media_worker_bytes_v1(&source).to_hex()
        )],
        "the port is called exactly once, with the parent's own length and digest"
    );
}

#[test]
fn source_gate_refuses_empty_and_oversize_before_any_worker_call() {
    let set = source_set(&[("media/empty.avif", Exposure::Public, Vec::new())]);
    let port = FakeWorkerPort::new(Vec::new());

    let outcome = inspect_media_worker_source_set_v1(&set, &port);

    assert_eq!(
        refusal_summary(&outcome),
        ("SOURCE_EMPTY".to_owned(), Some(0))
    );
    assert!(
        port.calls().is_empty(),
        "a zero-byte source is refused before the port is consulted"
    );

    assert_eq!(
        MediaWorkerRequestV1::for_source(&[]).unwrap_err(),
        MediaWorkerRefusalKindV1::SourceEmpty
    );
    let oversize = vec![0u8; (MEDIA_WORKER_MAX_SOURCE_BYTES_V1 + 1) as usize];
    assert_eq!(
        MediaWorkerRequestV1::for_source(&oversize).unwrap_err(),
        MediaWorkerRefusalKindV1::SourceOversize
    );
    assert!(
        MediaWorkerRequestV1::for_source(&oversize).is_err(),
        "one byte over the maximum never produces a request"
    );

    let maximum = vec![0x5au8; MEDIA_WORKER_MAX_SOURCE_BYTES_V1 as usize];
    let set = source_set(&[("media/max.avif", Exposure::Public, maximum)]);
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Refused(
        MediaWorkerRefusalKindV1::WorkerUnavailable,
    )]);

    let outcome = inspect_media_worker_source_set_v1(&set, &port);

    assert_eq!(
        refusal_summary(&outcome),
        ("WORKER_UNAVAILABLE".to_owned(), Some(0)),
        "exactly the maximum passes the byte gate and is refused later, for another reason"
    );
    assert_eq!(
        port.calls().len(),
        1,
        "exactly the maximum does reach the port"
    );
    assert_eq!(port.calls()[0].0, MEDIA_WORKER_MAX_SOURCE_BYTES_V1);
}

#[test]
fn whole_inventory_refuses_every_obligation_when_one_fails() {
    let first = source_body(0x31);
    let second = source_body(0x32);
    let set = source_set(&[
        ("media/one.avif", Exposure::Public, first.clone()),
        ("media/two.avif", Exposure::Internal, second.clone()),
    ]);
    let planes = yuv_planes();
    let good = valid_claim(&first, &planes);
    let mut bad = valid_claim(&second, &planes);
    bad.output_sha256 = MediaWorkerDigestV1::from_bytes([0u8; 32]);
    let port = FakeWorkerPort::new(vec![
        MediaWorkerRunV1::Observed(observation(&good, planes.clone())),
        MediaWorkerRunV1::Observed(observation(&bad, planes)),
    ]);

    let outcome = inspect_media_worker_source_set_v1(&set, &port);

    assert_eq!(
        refusal_summary(&outcome),
        ("WORKER_DIGEST_MISMATCH".to_owned(), Some(1)),
        "the failing internal obligation refuses the whole set"
    );
    assert!(
        outcome.checked().is_none(),
        "no partial record survives a failure: zero checked successes are exposed"
    );
    assert_eq!(
        port.calls().len(),
        2,
        "the internal obligation is dispatched like any other"
    );
}

#[test]
fn whole_inventory_refuses_a_mixed_format_set() {
    let first = source_body(0x41);
    let png = source_body(0x42);
    let planes = yuv_planes();
    let claim = valid_claim(&first, &planes);

    let set = source_set(&[
        ("media/one.avif", Exposure::Public, first.clone()),
        ("media/two.png", Exposure::Public, png.clone()),
    ]);
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &claim,
        planes.clone(),
    ))]);
    let outcome = inspect_media_worker_source_set_v1(&set, &port);
    assert_eq!(
        refusal_summary(&outcome),
        ("WORKER_FORMAT_UNSUPPORTED".to_owned(), Some(1)),
        "a non-AVIF obligation is never claimed as a supported format"
    );
    assert!(outcome.checked().is_none());
    assert_eq!(port.calls().len(), 1);

    let set = source_set(&[
        ("media/two.png", Exposure::Public, png),
        ("media/one.avif", Exposure::Public, first),
    ]);
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &claim, planes,
    ))]);
    let outcome = inspect_media_worker_source_set_v1(&set, &port);
    assert_eq!(
        refusal_summary(&outcome),
        ("WORKER_FORMAT_UNSUPPORTED".to_owned(), Some(0)),
        "a non-AVIF first obligation refuses before any worker call"
    );
    assert!(port.calls().is_empty());
}

#[test]
fn host_facts_must_all_be_positive_before_a_claim_is_read() {
    let source = source_body(0x51);
    let set = source_set(&[("media/hero.avif", Exposure::Public, source.clone())]);
    let planes = yuv_planes();
    let claim = valid_claim(&source, &planes);

    let cases: Vec<(&str, MediaWorkerObservationV1, &str)> = vec![
        (
            "limits not verified",
            MediaWorkerObservationV1::new(
                false,
                MediaWorkerExitV1::Normal,
                true,
                true,
                false,
                MediaWorkerLimitsV1::reference(),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_LIMIT_UNVERIFIED",
        ),
        (
            "wrong effective profile",
            MediaWorkerObservationV1::new(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                false,
                MediaWorkerLimitsV1::new(512 * 1024 * 1024, 10, 15_000),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_LIMIT_UNVERIFIED",
        ),
        (
            "abnormal exit",
            MediaWorkerObservationV1::new(
                true,
                MediaWorkerExitV1::Abnormal,
                true,
                true,
                false,
                MediaWorkerLimitsV1::reference(),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_EXIT_ABNORMAL",
        ),
        (
            "wall deadline missed",
            MediaWorkerObservationV1::new(
                true,
                MediaWorkerExitV1::Normal,
                false,
                true,
                false,
                MediaWorkerLimitsV1::reference(),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_WALL_TIMEOUT",
        ),
        (
            "descendant survived",
            MediaWorkerObservationV1::new(
                true,
                MediaWorkerExitV1::Normal,
                true,
                false,
                false,
                MediaWorkerLimitsV1::reference(),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_DESCENDANTS_REMAIN",
        ),
        (
            "cleanup leak observed",
            MediaWorkerObservationV1::new(
                true,
                MediaWorkerExitV1::Normal,
                true,
                true,
                true,
                MediaWorkerLimitsV1::reference(),
                claim.build(),
                planes.clone(),
            ),
            "WORKER_UNAVAILABLE",
        ),
    ];

    for (label, observation, expected) in cases {
        let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation)]);
        let outcome = inspect_media_worker_source_set_v1(&set, &port);
        assert_eq!(
            refusal_summary(&outcome).0,
            expected.to_owned(),
            "{label} must refuse with {expected}"
        );
        assert!(outcome.checked().is_none(), "{label} retains no output");
    }
}

/// One AVIF source and a mutable claim, ready for a single-defect adversarial case.
fn adversarial_setup() -> (GraphicAssetSourceSetV1, Vec<u8>, Vec<Vec<u8>>, Claim) {
    let source = source_body(0x61);
    let set = source_set(&[("media/hero.avif", Exposure::Public, source.clone())]);
    let planes = yuv_planes();
    let claim = valid_claim(&source, &planes);
    (set, source, planes, claim)
}

#[test]
fn parent_side_checks_refuse_each_hostile_envelope_by_category() {
    let (set, source, planes, base) = adversarial_setup();
    let other = source_body(0x62);

    // Source identity: digest and byte count are both re-derived by the parent.
    let mut wrong_digest = valid_claim(&source, &planes);
    wrong_digest.source_sha256 = sha256_media_worker_bytes_v1(&other);
    let mut wrong_length = valid_claim(&source, &planes);
    wrong_length.source_length = source.len() as u64 + 1;

    // Geometry: zero, above the per-side ceiling, and above the pixel ceiling.
    let mut zero_width = valid_claim(&source, &planes);
    zero_width.width = 0;
    let mut too_wide = valid_claim(&source, &planes);
    too_wide.width = 8_193;
    let mut too_many_pixels = valid_claim(&source, &planes);
    too_many_pixels.width = 8_192;
    too_many_pixels.height = 8_192;

    // Protocol and media facts.
    let mut wrong_version = valid_claim(&source, &planes);
    wrong_version.protocol_version = 2;
    let mut wrong_bit_depth = valid_claim(&source, &planes);
    wrong_bit_depth.bit_depth = 9;

    // Plane structure.
    let mut missing_plane = valid_claim(&source, &planes);
    missing_plane.planes.truncate(2);
    let mut duplicate_ordinal = valid_claim(&source, &planes);
    duplicate_ordinal.planes[1] =
        MediaWorkerPlaneClaimV1::new(0, 16, 16, 256, sha256_media_worker_bytes_v1(&planes[1]));
    let mut zero_stride = valid_claim(&source, &planes);
    zero_stride.planes[0] =
        MediaWorkerPlaneClaimV1::new(0, 0, 0, 0, sha256_media_worker_bytes_v1(&planes[0]));
    let mut stride_overflow = valid_claim(&source, &planes);
    stride_overflow.planes[0] =
        MediaWorkerPlaneClaimV1::new(0, u64::MAX, 2, 0, sha256_media_worker_bytes_v1(&planes[0]));
    let mut alpha_mismatch = valid_claim(&source, &planes);
    alpha_mismatch.color_model = MediaWorkerColorModelV1::Yuva;
    alpha_mismatch.has_alpha = false;

    // Digests.
    let mut plane_digest = valid_claim(&source, &planes);
    plane_digest.planes[2] = MediaWorkerPlaneClaimV1::new(
        2,
        16,
        16,
        256,
        sha256_media_worker_bytes_v1(b"not the plane"),
    );
    let mut output_digest = valid_claim(&source, &planes);
    output_digest.output_sha256 = MediaWorkerDigestV1::from_bytes([7u8; 32]);

    let cases: Vec<(&str, Claim, Vec<Vec<u8>>, &str)> = vec![
        (
            "source digest",
            wrong_digest,
            planes.clone(),
            "WORKER_SOURCE_MISMATCH",
        ),
        (
            "source length",
            wrong_length,
            planes.clone(),
            "WORKER_SOURCE_MISMATCH",
        ),
        (
            "zero width",
            zero_width,
            planes.clone(),
            "WORKER_DIMENSIONS_INVALID",
        ),
        (
            "width above ceiling",
            too_wide,
            planes.clone(),
            "WORKER_DIMENSIONS_INVALID",
        ),
        (
            "pixel ceiling",
            too_many_pixels,
            planes.clone(),
            "WORKER_DIMENSIONS_INVALID",
        ),
        (
            "protocol version",
            wrong_version,
            planes.clone(),
            "WORKER_PROTOCOL_INVALID",
        ),
        (
            "bit depth",
            wrong_bit_depth,
            planes.clone(),
            "WORKER_MEDIA_UNVERIFIED",
        ),
        (
            "missing plane",
            missing_plane,
            planes[..2].to_vec(),
            "WORKER_PLANE_INVALID",
        ),
        (
            "duplicate ordinal",
            duplicate_ordinal,
            planes.clone(),
            "WORKER_PLANE_INVALID",
        ),
        (
            "zero stride",
            zero_stride,
            vec![Vec::new(), planes[1].clone(), planes[2].clone()],
            "WORKER_PLANE_INVALID",
        ),
        (
            "stride overflow",
            stride_overflow,
            planes.clone(),
            "WORKER_PLANE_INVALID",
        ),
        (
            "alpha mismatch",
            alpha_mismatch,
            planes.clone(),
            "WORKER_PLANE_INVALID",
        ),
        (
            "plane digest",
            plane_digest,
            planes.clone(),
            "WORKER_DIGEST_MISMATCH",
        ),
        (
            "output digest",
            output_digest,
            planes.clone(),
            "WORKER_DIGEST_MISMATCH",
        ),
    ];

    for (label, claim, received, expected) in cases {
        let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
            &claim, received,
        ))]);
        let outcome = inspect_media_worker_source_set_v1(&set, &port);
        assert_eq!(
            refusal_summary(&outcome).0,
            expected.to_owned(),
            "{label} must refuse with {expected}"
        );
        assert!(outcome.checked().is_none(), "{label} retains no output");
        assert_eq!(port.calls().len(), 1, "{label} still made one call");
    }

    // A received buffer shorter than the claim's own length is a plane defect, not a digest one.
    let mut short_buffer = planes.clone();
    short_buffer[0] = vec![0x11u8; 1_023];
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &base,
        short_buffer,
    ))]);
    assert_eq!(
        refusal_summary(&inspect_media_worker_source_set_v1(&set, &port)).0,
        "WORKER_PLANE_INVALID".to_owned(),
        "a reported length that disagrees with the received buffer is a plane defect"
    );

    // A single mutated plane byte, with every claim unchanged, is a digest defect.
    let mut mutated = planes.clone();
    mutated[1][7] ^= 0xff;
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &base, mutated,
    ))]);
    assert_eq!(
        refusal_summary(&inspect_media_worker_source_set_v1(&set, &port)).0,
        "WORKER_DIGEST_MISMATCH".to_owned(),
        "a mutated plane byte is caught by the parent's own rehash"
    );
}

/// A claim whose claimed total output exceeds the defensive decoded bound.
#[test]
fn parent_side_checks_refuse_an_oversize_claimed_output_before_reading_bytes() {
    let source = source_body(0x71);
    let set = source_set(&[("media/hero.avif", Exposure::Public, source.clone())]);
    let per_plane = 200 * 1024 * 1024u64;
    let claims = vec![
        MediaWorkerPlaneClaimV1::new(
            0,
            per_plane,
            1,
            per_plane,
            MediaWorkerDigestV1::from_bytes([1; 32]),
        ),
        MediaWorkerPlaneClaimV1::new(
            1,
            per_plane,
            1,
            per_plane,
            MediaWorkerDigestV1::from_bytes([2; 32]),
        ),
        MediaWorkerPlaneClaimV1::new(2, 16, 16, 256, MediaWorkerDigestV1::from_bytes([3; 32])),
    ];
    let mut claim = valid_claim(&source, &yuv_planes());
    claim.planes = claims;
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &claim,
        vec![Vec::new(), Vec::new(), Vec::new()],
    ))]);

    let outcome = inspect_media_worker_source_set_v1(&set, &port);

    assert_eq!(
        refusal_summary(&outcome).0,
        "WORKER_OUTPUT_OVERSIZE".to_owned(),
        "400 MiB of claimed output exceeds the 256 MiB defensive bound"
    );
    assert!(outcome.checked().is_none());
}

/// A transposed-orientation claim is preserved as claimed, never inferred correct from width alone.
#[test]
fn transposed_orientation_is_accepted_as_a_claim_and_still_length_checked() {
    let source = source_body(0x72);
    let set = source_set(&[("media/rotate90.avif", Exposure::Public, source.clone())]);
    let planes = yuv_planes();
    let mut claim = valid_claim(&source, &planes);
    claim.orientation = MediaWorkerOrientationV1::Transposed;

    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &claim,
        planes.clone(),
    ))]);
    assert_eq!(
        checked_summary(&inspect_media_worker_source_set_v1(&set, &port)),
        vec![(PropertyKey::GraphicLogo.name().to_owned(), 0, 32, 32, 3)],
        "a structurally consistent transposed claim is retained as a claim"
    );

    // The same shape with a length that no longer matches its own stride arithmetic is refused.
    let mut broken = valid_claim(&source, &planes);
    broken.orientation = MediaWorkerOrientationV1::Transposed;
    broken.planes[0] =
        MediaWorkerPlaneClaimV1::new(0, 32, 32, 1_025, sha256_media_worker_bytes_v1(&planes[0]));
    let port = FakeWorkerPort::new(vec![MediaWorkerRunV1::Observed(observation(
        &broken,
        planes.clone(),
    ))]);
    assert_eq!(
        refusal_summary(&inspect_media_worker_source_set_v1(&set, &port)).0,
        "WORKER_PLANE_INVALID".to_owned(),
        "orientation never excuses a plane whose length disagrees with stride × rows"
    );
}

#[test]
fn sha256_matches_nist_vectors_over_raw_bytes() {
    assert_eq!(
        sha256_media_worker_bytes_v1(b"").to_hex(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256_media_worker_bytes_v1(b"abc").to_hex(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let million = vec![b'a'; 1_000_000];
    assert_eq!(
        sha256_media_worker_bytes_v1(&million).to_hex(),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
    assert_ne!(
        sha256_media_worker_bytes_v1(b"abc"),
        sha256_media_worker_bytes_v1(b"abd"),
        "the digest covers the raw bytes, not a rendering of them"
    );
}

#[test]
fn digest_hex_parsing_is_strict() {
    let digest = sha256_media_worker_bytes_v1(b"abc");
    let hex = digest.to_hex();

    assert_eq!(hex.len(), 64);
    assert_eq!(MediaWorkerDigestV1::parse_lower_hex(&hex), Some(digest));
    assert_eq!(
        MediaWorkerDigestV1::parse_lower_hex(&hex.to_uppercase()),
        None,
        "uppercase is rejected rather than normalised"
    );
    assert_eq!(MediaWorkerDigestV1::parse_lower_hex(&hex[1..]), None);
    assert_eq!(
        MediaWorkerDigestV1::parse_lower_hex(&format!("{hex}0")),
        None
    );
    assert_eq!(
        MediaWorkerDigestV1::parse_lower_hex(&format!("  {}", &hex[2..])),
        None
    );
    assert_eq!(MediaWorkerDigestV1::parse_lower_hex(""), None);
}

#[test]
fn aggregate_digest_encoding_is_exactly_the_documented_v1_layout() {
    let planes = vec![vec![1u8, 2, 3], vec![4u8, 5]];
    let claims = vec![
        MediaWorkerPlaneClaimV1::new(0, 3, 1, 3, sha256_media_worker_bytes_v1(&planes[0])),
        MediaWorkerPlaneClaimV1::new(1, 2, 1, 2, sha256_media_worker_bytes_v1(&planes[1])),
    ];

    let mut expected: Vec<u8> = Vec::new();
    for (ordinal, stride, rows, plane) in [(0u32, 3u64, 1u64, &planes[0]), (1, 2, 1, &planes[1])] {
        expected.extend_from_slice(&ordinal.to_le_bytes());
        expected.extend_from_slice(&stride.to_le_bytes());
        expected.extend_from_slice(&rows.to_le_bytes());
        expected.extend_from_slice(&(plane.len() as u64).to_le_bytes());
        expected.extend_from_slice(plane);
    }

    assert_eq!(
        media_worker_output_sha256_v1(&claims, &planes),
        sha256_media_worker_bytes_v1(&expected),
        "ordinal:u32 LE || stride:u64 LE || rows:u64 LE || length:u64 LE || bytes"
    );
}
