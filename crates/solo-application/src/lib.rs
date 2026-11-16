//! Application layer for the Foundation Solo slices.
//!
//! This crate owns the inbound use-case boundary, and — since `SOLO-WO-047 R02` — exactly
//! one outbound port. It depends on the pure domain crate plus the standard library only.
//!
//! The bootstrap-preflight use case still reads nothing, writes nothing, calls nothing and
//! publishes nothing. The source-read port exists because a read genuinely needs an
//! outward implementation: [`source_read::SourceReadPort`] describes what a filesystem read
//! must be able to report, and [`source_read::read_declared_source_v1`] drives the accepted
//! JSON-first selection policy over those reports without ever holding an operating-system
//! handle or turning a read into publication authority.

#![forbid(unsafe_code)]

mod bootstrap_preflight;
mod graphic_asset_inventory;
mod graphic_asset_read;
mod graphic_delivery_preparation;
mod graphic_image_inspection;
mod media_worker;
mod platform_asset_catalogue;
mod platform_asset_read;
mod platform_graphic_baseline;
mod property_resolution;
mod source_read;

pub use bootstrap_preflight::{
    BootstrapAcceptedV1, BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightService,
    BootstrapPreflightV1, BootstrapRefusalCode, BootstrapRefusedV1,
};
pub use graphic_asset_inventory::{
    GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY, GraphicAssetObligationInventoryV1,
    GraphicAssetObligationV1, MAX_MIXED_GRAPHIC_OBLIGATIONS_V1, MixedGraphicObligationV1,
    MixedGraphicOperationV1, MixedGraphicOriginV1, MixedGraphicPlanRefusalKindV1,
    MixedGraphicPlanRefusalV1, MixedGraphicPlanV1, collect_graphic_asset_obligations_v1,
    collect_mixed_graphic_obligations_v1,
};
pub use graphic_asset_read::{
    ENTITY_ASSET_NAMESPACE_SEGMENT, EntityAssetPathV1, EntityAssetSourceV1,
    GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY, GRAPHIC_ASSET_READ_VALIDATES_MEDIA,
    GraphicAssetReadPort, GraphicAssetReadRefusalKind, GraphicAssetReadRefusalV1,
    GraphicAssetReadSetOutcomeV1, GraphicAssetSourceEvidenceV1, GraphicAssetSourceSetV1,
    MAX_GRAPHIC_SOURCE_BYTES, read_graphic_asset_bytes_v1,
};
pub use graphic_delivery_preparation::{
    GRAPHIC_PACKAGE_GRANTS_PUBLICATION_AUTHORITY_V1, GRAPHIC_PACKAGE_ROUTES_ANY_URL_V1,
    GraphicDeliveryPackageOutcomeV1, GraphicDeliveryPackageV1, GraphicPackageEntryV1,
    GraphicPackageLimitsV1, GraphicPackagePayloadV1, GraphicPackageRefusalKindV1,
    GraphicPackageRefusalV1, GraphicPackageRequestV1, MAX_PACKAGE_CONTENT_KEY_BYTES_V1,
    MAX_PACKAGE_ENTRIES_V1, MAX_PACKAGE_TOTAL_BYTES_V1, PACKAGE_KEY_PREFIX_V1,
    PrepublicationDisplayEligibilityV1, PublicGraphicPackageEntryV1, PublicGraphicPackageV1,
    prepare_mixed_graphic_package_v1,
};
pub use graphic_image_inspection::{
    AvifEvidenceV4, AvifPlaneEvidenceV4, GRAPHIC_IMAGE_INSPECTION_V4_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_IMAGE_INSPECTION_V4_PROVES_WHOLE_PUBLICATION, GraphicImageInspectRefusalKindV4,
    GraphicImageInspectRefusalV4, GraphicImageInspectionOutcomeV4, GraphicImageInspectionSetV4,
    GraphicImageInspectionV4, GraphicImagePayloadV4, GraphicMixedInspectionSetV1,
    GraphicMixedInspectionV1, MIXED_GRAPHIC_INSPECTION_GRANTS_PUBLICATION_AUTHORITY_V1,
    MIXED_GRAPHIC_INSPECTION_PROVES_WHOLE_PUBLICATION_V1, MixedGraphicCandidateOutcomeV1,
    MixedGraphicInspectRefusalKindV1, MixedGraphicInspectRefusalV1, PlatformSourceRequestV1,
    inspect_graphic_image_source_set_v4, inspect_mixed_graphic_candidate_v1,
};
pub use graphic_image_inspection::{
    GIF87A_SIGNATURE_V2, GIF89A_SIGNATURE_V2,
    GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_IMAGE_INSPECTION_PROVES_MEDIA_CORRECTNESS,
    GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_IMAGE_INSPECTION_V2_PROVES_MEDIA_CORRECTNESS, GifAnimationEvidenceV2, GifDisposalV2,
    GifFrameEvidenceV2, GifLoopV2, GraphicImageFormatV1, GraphicImageFormatV2,
    GraphicImageInspectPortOutcomeV1, GraphicImageInspectPortOutcomeV2, GraphicImageInspectPortV1,
    GraphicImageInspectPortV2, GraphicImageInspectRefusalKindV1, GraphicImageInspectRefusalKindV2,
    GraphicImageInspectRefusalV1, GraphicImageInspectRefusalV2, GraphicImageInspectRequestV1,
    GraphicImageInspectRequestV2, GraphicImageInspectionOutcomeV1, GraphicImageInspectionOutcomeV2,
    GraphicImageInspectionSetV1, GraphicImageInspectionSetV2, GraphicImageInspectionV1,
    GraphicImageInspectionV2, GraphicImageObservationV2, GraphicImagePayloadV2,
    GraphicImageWorkerObservationV1, IMAGE_MIME_AVIF_V3, IMAGE_MIME_GIF_V2, IMAGE_MIME_JPEG_V1,
    IMAGE_MIME_PNG_V1, IMAGE_MIME_SVG_V3, IMAGE_MIME_WEBP_V1, IMAGE_WORKER_WALL_MS_V1,
    MAX_GIF_FRAMES_V2, MAX_GIF_ONE_CYCLE_DELAY_MS_V2, MAX_GIF_TOTAL_COMPOSITED_PIXELS_V2,
    MAX_GIF_TRACE_BYTES_V2, MAX_IMAGE_DECODED_BYTES_V1, MAX_IMAGE_PIXELS_V1, MAX_IMAGE_SIDE_V1,
    MAX_IMAGE_SOURCE_BYTES_V1, MAX_SVG_ATTRS_PER_ELEMENT_V3, MAX_SVG_CANONICAL_BYTES_V3,
    MAX_SVG_ELEMENTS_V3, MAX_SVG_FILTER_ELEMENTS_V3, MAX_SVG_FILTER_OPERATIONS_V3,
    MAX_SVG_NESTING_V3, MAX_SVG_PATH_COMMANDS_V3, MAX_SVG_REFERENCES_V3, MAX_SVG_SOURCE_BYTES_V3,
    StillImageEvidenceV2, checked_rgba8_bytes_v1, inspect_graphic_image_source_set_v1,
    inspect_graphic_image_source_set_v2, role_admits_format_v2, side_within_bounds_v1,
};
pub use graphic_image_inspection::{
    GRAPHIC_IMAGE_INSPECTION_V3_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_IMAGE_INSPECTION_V3_PROVES_MEDIA_CORRECTNESS, GraphicImageFormatV3,
    GraphicImageInspectPortOutcomeV3, GraphicImageInspectPortV3, GraphicImageInspectRefusalKindV3,
    GraphicImageInspectRefusalV3, GraphicImageInspectRequestV3, GraphicImageInspectionOutcomeV3,
    GraphicImageInspectionSetV3, GraphicImageInspectionV3, GraphicImageObservationV3,
    GraphicImagePayloadV3, GraphicVectorEvidenceV3, IMAGE_MIME_GIF_V3, StillImageEvidenceV3,
    inspect_graphic_image_source_set_v3, role_admits_format_v3,
};
pub use media_worker::{
    MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1, MEDIA_WORKER_MAX_DECODED_BYTES_V1,
    MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1, MEDIA_WORKER_MAX_SOURCE_BYTES_V1,
    MEDIA_WORKER_PROTOCOL_VERSION_V1, MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1,
    MEDIA_WORKER_REFERENCE_CPU_SECONDS_V1, MEDIA_WORKER_REFERENCE_MEMORY_BYTES_V1,
    MEDIA_WORKER_REFERENCE_WALL_MS_V1, MediaWorkerBoundaryOutcomeV1, MediaWorkerCheckedEnvelopeV1,
    MediaWorkerColorModelV1, MediaWorkerDigestV1, MediaWorkerEnvelopeV1, MediaWorkerExitV1,
    MediaWorkerFormatV1, MediaWorkerLimitsV1, MediaWorkerMimeV1, MediaWorkerObservationV1,
    MediaWorkerOrientationV1, MediaWorkerPlaneClaimV1, MediaWorkerPortV1, MediaWorkerRefusalKindV1,
    MediaWorkerRefusalV1, MediaWorkerRequestV1, MediaWorkerRunV1, MediaWorkerStatusV1,
    inspect_media_worker_source_set_v1, media_worker_output_sha256_v1,
    sha256_media_worker_bytes_v1,
};
pub use platform_asset_catalogue::{
    MAX_PLATFORM_ASSET_ID_BYTES_V1, MAX_PLATFORM_ASSET_ID_SEGMENTS_V1,
    MAX_PLATFORM_ASSET_PIXELS_V1, MAX_PLATFORM_ASSET_SIDE_V1, MAX_PLATFORM_CATALOGUE_ENTRIES_V1,
    MAX_PLATFORM_RIGHTS_SCALARS_V1, PLATFORM_ASSET_CATALOGUE_GRANTS_PUBLICATION_AUTHORITY_V1,
    PLATFORM_ASSET_CATALOGUE_PLATFORM_SOURCE_READABLE_V1,
    PLATFORM_ASSET_CATALOGUE_PROVES_MEDIA_CORRECTNESS_V1,
    PLATFORM_ASSET_CATALOGUE_RIGHTS_REVIEWED_V1, PLATFORM_ASSET_LIBRARY_VERSION_V1,
    PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1, PlatformAssetDeclarationV1,
    PlatformAssetLicenceV1, PlatformAssetMediaClassV1, PlatformAssetProvenanceV1,
    PlatformCatalogueBindingScopeV1, PlatformCatalogueBindingV1, PlatformCatalogueMatchRefusalV1,
    PlatformCatalogueMatchV1, PlatformCatalogueRefusalV1, bind_catalogue_to_v4_evidence_v1,
    match_catalogue_entry_to_v4_evidence_v1,
};
pub use platform_asset_read::{
    APPROVED_PLATFORM_ASSET_COUNT_V1, APPROVED_PLATFORM_CATALOGUE_SHA256_V1,
    APPROVED_PLATFORM_LIBRARY_VERSION_V1, MAX_PLATFORM_CATALOGUE_BYTES_V1,
    MAX_PLATFORM_SOURCE_BYTES_V1, PLATFORM_ASSET_NAMESPACE_SEGMENT_V1,
    PLATFORM_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY_V1,
    PLATFORM_ASSET_READ_PROVES_GLOBAL_SNAPSHOT_V1, PLATFORM_ASSET_READ_PROVES_MEDIA_V1,
    PLATFORM_ICON_DIRECTORY_V1, PLATFORM_IMAGE_EXTENSION_V1, PlatformAssetLibraryReadPortV1,
    PlatformAssetReadRefusalKindV1, PlatformAssetReadRefusalV1, PlatformAssetRefV1,
    PlatformAssetSourceEvidenceV1, PlatformAssetSourceSetV1, PlatformAssetSourceV1,
    PlatformCatalogueSourceV1, PlatformLibraryReadOutcomeV1, PlatformRootProvenanceV1,
    PlatformRootSelectionV1, read_pinned_platform_sources_v1,
};

pub use platform_graphic_baseline::{
    PLATFORM_GRAPHIC_BASELINE_GRANTS_PUBLICATION_AUTHORITY_V1,
    PlatformGraphicBaselineRefusalKindV1, PlatformGraphicBaselineRefusalV1,
    approved_platform_graphic_binding_v1,
};
pub use property_resolution::{
    PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY, PropertyResolutionCandidateV1,
    PropertyResolutionRequestV1, resolve_declared_properties_v1,
};
pub use source_read::{
    CandidateBodyV1, CandidateProbeV1, DeclaredSourceSetSinkV1, MAX_JSON_SOURCE_BYTES,
    MAX_MARKDOWN_SOURCE_BYTES, PhysicalIdentityV1, PhysicalReservationV1, ProbeFactsV1,
    ReadSourceCandidateV1, SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY,
    SOURCE_READ_PROVES_GLOBAL_SNAPSHOT, SetSourceEntryV1, SourceReadOutcomeV1, SourceReadPort,
    SourceReadRefusalKind, SourceReadRefusalV1, SourceReadSetOutcomeV1, SourceReadSetRefusalV1,
    bound_for, check_physical_reservations_v1, drive_declared_source_set_v1, fingerprint_of,
    read_declared_source_v1,
};
