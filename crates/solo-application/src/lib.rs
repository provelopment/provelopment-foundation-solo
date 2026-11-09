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
mod source_read;

pub use bootstrap_preflight::{
    BootstrapAcceptedV1, BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightService,
    BootstrapPreflightV1, BootstrapRefusalCode, BootstrapRefusedV1,
};
pub use source_read::{
    CandidateBodyV1, CandidateProbeV1, MAX_JSON_SOURCE_BYTES, MAX_MARKDOWN_SOURCE_BYTES,
    PhysicalIdentityV1, PhysicalReservationV1, ProbeFactsV1, ReadSourceCandidateV1,
    SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY, SOURCE_READ_PROVES_GLOBAL_SNAPSHOT,
    SourceReadOutcomeV1, SourceReadPort, SourceReadRefusalKind, SourceReadRefusalV1, bound_for,
    check_physical_reservations_v1, fingerprint_of, read_declared_source_v1,
};
