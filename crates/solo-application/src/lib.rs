//! Application layer for the Foundation Solo `R01.A` slice.
//!
//! This crate owns the inbound use-case boundary and nothing else. It depends on
//! the pure domain crate plus the standard library, and it deliberately declares
//! **no outbound port**: the bootstrap-preflight use case reads nothing, writes
//! nothing, calls nothing and publishes nothing, so an ornamental outbound
//! interface would be an abstraction without a use case.

#![forbid(unsafe_code)]

mod bootstrap_preflight;

pub use bootstrap_preflight::{
    BootstrapAcceptedV1, BootstrapInputV1, BootstrapOutcomeV1, BootstrapPreflightService,
    BootstrapPreflightV1, BootstrapRefusalCode, BootstrapRefusedV1,
};
