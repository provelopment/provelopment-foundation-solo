//! Pure domain layer for the Foundation Solo `R01.A` slice.
//!
//! This crate holds only deterministic decisions over already-supplied authored
//! values. It performs no I/O, owns no environment access, imports no adapter or
//! transport concern, and depends on the standard library alone so that the
//! inward-only dependency direction of the accepted Hexagonal contract is
//! mechanically checkable.
//!
//! Three value objects and one validated graph are provided:
//!
//! * [`EntityIdV1`] — the validated, language-neutral Entity identifier of the
//!   accepted one-Entity model. It is never derived from a public address,
//!   locale, route or hostname.
//! * [`BaseCandidateV1`] — the deliberately narrow engineering subset of the
//!   separately authored public base URL contract. This is a preflight check,
//!   not the future full §6.2J URL profile, and no address it refuses may be
//!   reported as prohibited by the product Charter.
//! * [`EfsOwnershipGraph`] — the validated `R02.A` single-Entity ownership
//!   structure. Facets, Sites, Pages and representation membership keys are
//!   validated and refused rather than repaired, direct containment is the only
//!   parentage, and locale keys are membership keys rather than validated tags.
//! * [`SourceCandidatePathV1`] — one validated logical `sourceCandidates` path, and
//!   [`SourceCandidateReservationsV1`] — the cross-representation claim set. This is
//!   the pure lexical half of the accepted portable-source contract: it opens no
//!   file, tests no existence, selects no body and proves no filesystem identity.
//! * [`ProvisionalSourceChoiceV1`] — the outcome of the pure JSON-first selection
//!   policy over caller-supplied observations. It is *provisional*: it borrows the
//!   caller's bytes, proves no confinement or snapshot and authorizes no publication.

#![forbid(unsafe_code)]

mod base_candidate;
mod efs_ownership;
mod entity_id;
mod property_registry;
mod property_resolution;
mod source_locator;
mod source_selection;

pub use base_candidate::{
    BaseCandidateDetail, BaseCandidateError, BaseCandidateErrorKind, BaseCandidateV1, BaseScheme,
    MAX_HOST_OCTETS, MAX_LABEL_OCTETS, MAX_PREFIX_SEGMENTS,
};
pub use efs_ownership::{
    EfsGraphError, EfsGraphErrorKind, EfsLevel, EfsLineageError, EfsLineageErrorKind,
    EfsNodeIdError, EfsOwnershipDraft, EfsOwnershipGraph, EfsPageTarget, FacetDraft, FacetIdV1,
    FacetNode, LocaleKeyError, LocaleKeyErrorKind, LocaleKeyV1, MAX_EFS_NODES, MAX_PAGES,
    MAX_REPRESENTATIONS_PER_PAGE, PageDraft, PageIdV1, PageNode, SiteDraft, SiteIdV1, SiteNode,
};
pub use entity_id::{ENTITY_ID_MAX_BYTES, EntityIdError, EntityIdErrorKind, EntityIdV1};
pub use property_registry::{
    ACCEPTED_IMAGE_EXTENSIONS, AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1,
    CoordinatesV1, CurrentContextV1, DeclaredOwnerPropertiesV1, Exposure, FactIdV1, Inheritance,
    LOCATION_FIELD_NAMES, LocationExtensionRequestV1, LocationFactV1, LocationFieldsV1,
    LocationKind, MAX_ADDRESS_LINE_SCALARS, MAX_AREA_SCALARS, MAX_ASSET_REFERENCE_BYTES,
    MAX_BACKGROUND_LAYERS, MAX_LOCAL_ENTRIES, MAX_LOCATION_FACTS, MAX_MAILBOX_BYTES,
    MAX_NAMED_PLACE_SCALARS, MAX_POSTAL_CODE_SCALARS, MAX_SEO_DESCRIPTION_SCALARS,
    MAX_SEO_TITLE_SCALARS, MAX_WORLD_REGION_SCALARS, MailboxV1, PROPERTY_REGISTRY_VERSION,
    PlainTextV1, PropertyDeclarationRefusalV1, PropertyDeclarationV1, PropertyKey,
    PropertyRefusalKind, PropertyValueV1, QualifiedOwnerV1,
};
pub use property_resolution::{
    FieldOriginV1, LocationExtensionCause, PropertyResolutionRefusalKind,
    PropertyResolutionRefusalV1, ProvenanceValueV1, PublicPropertyProjectionV1, PublicPropertyV1,
    ResolutionEventV1, ResolutionOperation, ResolvedPropertiesV1, ResolvedPropertyGraphV1,
    ResolvedPropertyV1, ValidatedPropertyInputV1, project_public_properties_v1,
    require_matching_owner_membership_v1, resolve_all_owners_v1, validate_declared_properties_v1,
};
pub use source_locator::{
    FILESYSTEM_IDENTITY_PROVEN, MAX_LOCATOR_BYTES, MAX_LOCATOR_SEGMENTS, MAX_SEGMENT_BYTES,
    RESERVED_WINDOWS_STEMS, SOURCE_NAMESPACE, SourceCandidateDraftV1, SourceCandidateKind,
    SourceCandidatePathV1, SourceCandidateReservationsV1, SourceCandidateSetV1, SourceClaimError,
    SourceClaimErrorKind, SourceLocatorError, SourceLocatorErrorKind,
};
pub use source_selection::{
    OBSERVATION_IS_FILESYSTEM_PROOF, PROVISIONAL_CHOICE_IS_FINAL_SELECTION,
    ProvisionalSourceChoiceV1, SourceFailureKind, SourceObservationV1, SourceSelectionRefusal,
    SourceSelectionRefusalKind, SuppliedObservationsV1, select_provisional_source,
};
