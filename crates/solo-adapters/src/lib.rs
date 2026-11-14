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
//! declared `pages/…` locator, and — for the separate `R03.C2` graphic gate — an authored Entity
//! `assets/…` reference, both beneath an explicitly adopted content-root handle. It refuses
//! every symlink, reparse point, escape and identity collision, and returns owned bytes with
//! the physical identity of the object they came from. Neither namespace can reach the other,
//! and it publishes nothing.
//!
//! Dependency direction is strictly inward: this crate may depend on
//! `solo-application` and `solo-domain`, and neither of those may depend on it. The reviewed
//! external roots are normal dependencies of this outer adapter alone: `serde` and
//! `serde_json` for parsing, and the three target-gated native roots `rustix` and `libc` (Linux
//! only) and `windows-sys` (Windows only).
//!
//! **Unsafe isolation.** This crate denies `unsafe_code` rather than forbidding it, because
//! exactly two platform-gated modules must call the operating system directly: the Windows NT
//! source reader, and — from `SOLO-WO-076 R01`, and widened by `SOLO-WO-079 R01` — the Linux
//! limit boundary, which reads the calling process's existing `RLIMIT_AS` and `RLIMIT_CPU`
//! ceilings with `getrlimit`, and which in one fixed internal mode installs those ceilings and a
//! new session on a single trusted child inside `pre_exec`, before that child is executed. There
//! are therefore exactly two module-scoped allowances below, one Windows and one Linux; the
//! architecture guard fails the build if any other file, module or attribute acquires an unsafe
//! capability. The `R03.C3` protocol module added by `SOLO-WO-072 R01` holds no allowance of its
//! own: it parses supplied bytes only and starts no process, opens no file and holds no handle.
//! The safe `media_worker_host` façade added by `SOLO-WO-079 R01` holds no allowance either: it
//! is ordinary safe Rust that names no OS constant, no raw handle and no native root.
//!
//! The media-worker half strictly parses **already-supplied** worker metadata into an inert,
//! untrusted claim. It performs no I/O, cannot execute anything, and authorizes nothing.

#![deny(unsafe_code)]

mod bootstrap_cli;
mod graphic_gif_inspection;
mod graphic_image_inspection;
mod media_worker_host;
// One reserved process-native file, available on both hosts. The `any(..)` gates *module
// availability* only; it is never a proof of the scope of an `unsafe` expression, and every
// process-native `unsafe` inside the file still sits in its own positive `#[cfg(target_os =
// "linux")]` or `#[cfg(windows)]` item, which is what the AST checker proves from real text.
#[cfg(any(target_os = "linux", windows))]
#[cfg_attr(target_os = "linux", allow(unsafe_code))]
#[cfg_attr(windows, allow(unsafe_code))]
mod media_worker_host_native;
mod media_worker_protocol;
mod property_resolution;
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
pub use graphic_gif_inspection::{
    GIF_MIME_V1, GIF87A_SIGNATURE_V1, GIF89A_SIGNATURE_V1, GifAnimationEvidenceV1, GifDisposalV1,
    GifFrameEvidenceV1, GifInspectionRefusalV1, GifLoopV1, MAX_GIF_FRAMES_V1,
    MAX_GIF_ONE_CYCLE_DELAY_MS_V1, MAX_GIF_SOURCE_BYTES_V1, MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1,
    MAX_GIF_TRACE_BYTES_V1, inspect_gif_animation_v1,
};
pub use graphic_image_inspection::{
    ConfinedImageInspectAdapterV1, EntityImageOutcomeV1, EntityImageRefusalV1,
    IMAGE_WORKER_EXIT_DECODE_FAILED_V1, IMAGE_WORKER_EXIT_INVALID_MEDIA_V1,
    IMAGE_WORKER_EXIT_REFUSED_V1, IMAGE_WORKER_EXIT_UNCONFINED_V1, IMAGE_WORKER_FRAME_PREFIX_V1,
    IMAGE_WORKER_MODE_DECODE_V1, IMAGE_WORKER_MODE_HANG_V1, IMAGE_WORKER_MODE_REFUSE_V1,
    IMAGE_WORKER_MODE_SHORT_OUTPUT_V1, IMAGE_WORKER_MODE_TRAILING_V1,
    IMAGE_WORKER_MODE_WRONG_OUTPUT_V1, IMAGE_WORKER_MODE_WRONG_SOURCE_V1,
    INTERNAL_IMAGE_WORKER_FLAG_V1, MAX_IMAGE_WORKER_FRAME_BYTES_V1,
    MAX_IMAGE_WORKER_STDOUT_BYTES_V1, TestOwnerV1, inspect_entity_graphic_images_v1,
    inspect_image_bytes_with_mode_v1, run_internal_image_worker_v1, test_inventory_v1,
};
pub use media_worker_host::{
    CHILD_OBSERVATION_MARKER_V1, EXIT_INTERNAL_PROBE_REFUSED, INTERNAL_PROBE_CHILD_TOKEN_V1,
    INTERNAL_PROBE_FLAG_V1, INTERNAL_PROBE_PARENT_TOKEN_V1, INTERNAL_REFUSAL_PREFIX_V1,
    ImageWorkerConfinementV1, ImageWorkerHostV1, ImageWorkerRunRefusalV1, ImageWorkerRunV1,
    InternalProbeModeV1, InternalProbeReportV1, MAX_CHILD_STDERR_BYTES, MAX_CHILD_STDOUT_BYTES,
    PARENT_EVIDENCE_MARKER_V1, PROFILE_V1_AS_BYTES, PROFILE_V1_CPU_SECONDS, SMOKE_WALL_DEADLINE_MS,
    SmokeObservationV1, SmokeRefusalV1, child_image_worker_confinement_v1, classify_child_exit_v1,
    classify_internal_arguments_v1, parse_child_observation_v1, run_confined_image_worker_v1,
    run_internal_linux_limit_smoke_v1, run_internal_media_host_probe_v1, validate_observation_v1,
};
pub use media_worker_protocol::{
    MediaWorkerProtocolRefusalKindV1, MediaWorkerProtocolRefusalV1, parse_media_worker_envelope_v1,
};
pub use property_resolution::{
    ManifestPropertyCandidateV1, ManifestPropertyRefusalKind, ManifestPropertyRefusalV1,
    validate_manifest_property_candidate_v1,
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
    StagedBodyV1, StagingFaultV1, StagingFaultsV1, StagingRootCapability,
};

/// `SOLO-WO-104 R01` closed N102 test seam.
///
/// Exactly four bounded Windows facts cross this boundary, for the integration suite only: the
/// injection selector, the boundary that refused, the extra inheritable handle one canary holds
/// across a real spawn, and the two child-side isolation probes. None of them exposes a process,
/// command, limit or handle operation, and none adds an `unsafe` allowance.
#[cfg(windows)]
pub use media_worker_host::{
    N102ExtraInheritableV1, n102_probe_environment_v1, n102_probe_handle_v1,
};

/// `SOLO-WO-104 R01` — the closed N102 probe surface the integration suite drives.
///
/// Four bounded facts and two typed refusals only: run one handle probe, run one environment
/// probe, run the isolated helper parent, and parse either bounded answer. No process, command,
/// limit or handle operation is exported, and no `unsafe` allowance is added.
pub use media_worker_host::{
    INTERNAL_N102_ENV_PARENT_FLAG_V1, N102_ENV_PARENT_PREFIX_V1, N102_LAST_FAULT_STEP_V1,
    N102EnvironmentProbeV1, N102FaultRunV1, N102HandleProbeV1, N102ProbeRefusalV1,
    parse_n102_environment_answer_v1, parse_n102_handle_answer_v1,
    run_internal_n102_environment_parent_v1, run_n102_environment_probe_v1,
    run_n102_fault_attempt_v1, run_n102_handle_probe_v1,
};
