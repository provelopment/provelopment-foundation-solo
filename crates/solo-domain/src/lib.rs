//! Pure domain layer for the Foundation Solo `R01.A` slice.
//!
//! This crate holds only deterministic decisions over already-supplied authored
//! values. It performs no I/O, owns no environment access, imports no adapter or
//! transport concern, and depends on the standard library alone so that the
//! inward-only dependency direction of the accepted Hexagonal contract is
//! mechanically checkable.
//!
//! Two value objects are provided:
//!
//! * [`EntityIdV1`] — the validated, language-neutral Entity identifier of the
//!   accepted one-Entity model. It is never derived from a public address,
//!   locale, route or hostname.
//! * [`BaseCandidateV1`] — the deliberately narrow engineering subset of the
//!   separately authored public base URL contract. This is a preflight check,
//!   not the future full §6.2J URL profile, and no address it refuses may be
//!   reported as prohibited by the product Charter.

#![forbid(unsafe_code)]

mod base_candidate;
mod entity_id;

pub use base_candidate::{
    BaseCandidateDetail, BaseCandidateError, BaseCandidateErrorKind, BaseCandidateV1, BaseScheme,
    MAX_HOST_OCTETS, MAX_LABEL_OCTETS, MAX_PREFIX_SEGMENTS,
};
pub use entity_id::{ENTITY_ID_MAX_BYTES, EntityIdError, EntityIdErrorKind, EntityIdV1};
