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
mod graphic_image_inspection;
mod media_worker;
mod property_resolution;
mod source_read;

pub use bootstrap_preflight::{
    BootstrapAcceptedV1, BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightService,
    BootstrapPreflightV1, BootstrapRefusalCode, BootstrapRefusedV1,
};
pub use graphic_asset_inventory::{
    GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY, GraphicAssetObligationInventoryV1,
    GraphicAssetObligationV1, collect_graphic_asset_obligations_v1,
};
pub use graphic_asset_read::{
    ENTITY_ASSET_NAMESPACE_SEGMENT, EntityAssetPathV1, EntityAssetSourceV1,
    GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY, GRAPHIC_ASSET_READ_VALIDATES_MEDIA,
    GraphicAssetReadPort, GraphicAssetReadRefusalKind, GraphicAssetReadRefusalV1,
    GraphicAssetReadSetOutcomeV1, GraphicAssetSourceEvidenceV1, GraphicAssetSourceSetV1,
    MAX_GRAPHIC_SOURCE_BYTES, read_graphic_asset_bytes_v1,
};
pub use graphic_image_inspection::{
    GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY,
    GRAPHIC_IMAGE_INSPECTION_PROVES_MEDIA_CORRECTNESS, GraphicImageFormatV1,
    GraphicImageInspectPortOutcomeV1, GraphicImageInspectPortV1, GraphicImageInspectRefusalKindV1,
    GraphicImageInspectRefusalV1, GraphicImageInspectRequestV1, GraphicImageInspectionOutcomeV1,
    GraphicImageInspectionSetV1, GraphicImageInspectionV1, GraphicImageWorkerObservationV1,
    IMAGE_MIME_JPEG_V1, IMAGE_MIME_PNG_V1, IMAGE_MIME_WEBP_V1, IMAGE_WORKER_WALL_MS_V1,
    MAX_IMAGE_DECODED_BYTES_V1, MAX_IMAGE_PIXELS_V1, MAX_IMAGE_SIDE_V1, MAX_IMAGE_SOURCE_BYTES_V1,
    checked_rgba8_bytes_v1, inspect_graphic_image_source_set_v1, side_within_bounds_v1,
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
