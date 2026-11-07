//! Driving adapter for the Foundation Solo `R01.A` slice.
//!
//! The adapter translates raw operating-system command-line arguments into the
//! application's typed input, invokes the inbound use case through its interface,
//! and renders a deterministic result. It reads no environment variable, no file,
//! no current directory and no network location, so an authored value can never be
//! silently supplied for the operator.
//!
//! Dependency direction is strictly inward: this crate may depend on
//! `solo-application` and `solo-domain`, and neither of those may depend on it.

#![forbid(unsafe_code)]

mod bootstrap_cli;

pub use bootstrap_cli::{
    CliOutcome, CliRefusal, CliRefusalCode, EXIT_INTERNAL, EXIT_OK, EXIT_REFUSED, FLAG_BASE_URL,
    FLAG_ENTITY_ID, SUBCOMMAND, parse_request, run,
};
