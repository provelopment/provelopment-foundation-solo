//! Driving adapters for Foundation Solo.
//!
//! The CLI half translates raw operating-system command-line arguments into the
//! application's typed input, invokes the inbound use case through its interface, and
//! renders a deterministic result. It reads no environment variable, no file, no current
//! directory and no network location, so an authored value can never be silently
//! supplied for the operator.
//!
//! The manifest half strictly parses **already-supplied** publication-manifest bytes into
//! an inert syntactic candidate. It walks no content tree, opens no source and holds no
//! publication authority.
//!
//! Dependency direction is strictly inward: this crate may depend on
//! `solo-application` and `solo-domain`, and neither of those may depend on it. The two
//! reviewed external roots `serde` and `serde_json` are normal dependencies of this outer
//! adapter alone, and are the only exceptions.

#![forbid(unsafe_code)]

mod bootstrap_cli;
mod publication_manifest;

pub use bootstrap_cli::{
    CliOutcome, CliRefusal, CliRefusalCode, EXIT_INTERNAL, EXIT_OK, EXIT_REFUSED, FLAG_BASE_URL,
    FLAG_ENTITY_ID, SUBCOMMAND, parse_request, run,
};
pub use publication_manifest::{
    BaseDeclarationV1, DeclaredCaseMatching, DeclaredMissingTranslation, DeclaredNode,
    DeclaredVersionV1, DeclaredVisibility, EntityDeclarationV1, FacetDeclarationV1,
    InertPropertiesV1, MANIFEST_LOGICAL_IDENTITY, MAX_BASE_ALIASES, MAX_MANIFEST_BYTES,
    MAX_PROPERTY_DEPTH, MAX_PROPERTY_ENTRIES, ManifestError, ManifestErrorKind, PageDeclarationV1,
    ParsedManifestCandidateV1, QualifiedTargetV1, ROOT_KEYS, RepresentationDeclarationV1,
    RouteAliasV1, RouteDeclarationV1, SUPPORTED_VERSION, SiteDeclarationV1, parse_manifest_bytes,
};
