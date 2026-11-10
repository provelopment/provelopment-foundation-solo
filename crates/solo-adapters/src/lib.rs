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
//! The preflight half applies an inert byte check to one already-selected body and owns an
//! immutable copy of it. It still opens no file, proves no filesystem state and authorizes
//! nothing: the one documented in-memory pipeline is a trust boundary, not a publisher.
//!
//! The source-read half is the only code in the project that opens a file. It resolves a
//! declared `pages/…` locator beneath an explicitly adopted content-root handle, refuses
//! every symlink, reparse point, escape and identity collision, and returns owned bytes with
//! the physical identity of the object they came from. It publishes nothing.
//!
//! Dependency direction is strictly inward: this crate may depend on
//! `solo-application` and `solo-domain`, and neither of those may depend on it. The reviewed
//! external roots are normal dependencies of this outer adapter alone: `serde` and
//! `serde_json` for parsing, and the two target-gated native roots `rustix` (Linux only) and
//! `windows-sys` (Windows only).
//!
//! **Unsafe isolation.** This crate denies `unsafe_code` rather than forbidding it, because
//! one Windows-gated module must call the NT entry point directly. The single allowance below
//! is scoped to that module and only on Windows; the architecture guard fails the build if any
//! other file, module or attribute acquires an unsafe capability.

#![deny(unsafe_code)]

mod bootstrap_cli;
mod publication_manifest;
mod source_body_preflight;
mod source_read;
#[cfg_attr(windows, allow(unsafe_code))]
mod source_read_native;
mod source_set_candidate;
mod source_set_staging;

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
pub use source_body_preflight::{
    BODY_CONTENT_VERSION, BODY_ROOT_KEYS, BodyPreflightRefusal, BodyPreflightRefusalKind,
    CONTENT_VALIDITY_PROVEN, InMemorySourceRefusalV1, InMemorySourceRequestV1,
    InertBodyCandidateV1, MAX_BLOCKS, MAX_JSON_BODY_BYTES, MAX_MARKDOWN_BODY_BYTES,
    MAX_SUMMARY_SCALARS, MAX_TITLE_SCALARS, preflight_selected_body, run_in_memory_source_pipeline,
};
pub use source_read::{ContentRootCapability, RootProvenanceV1, TrustedDirectoryHandle};
pub use source_set_candidate::{
    QualifiedSourceLineageV1, SourceSetCandidateV1, SourceSetMemberV1, SourceSetRefusalV1,
    StagedSourceEvidenceV1, assemble_declared_source_set_v1,
};
pub use source_set_staging::{
    STAGE_READ_CHUNK_BYTES, STAGE_SHARD_TARGET_BYTES, SourceSetStagingV1, StageReaderV1,
    StagedBodyV1, StagingRootCapability,
};
