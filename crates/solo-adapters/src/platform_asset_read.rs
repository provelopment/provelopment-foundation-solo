//! The separate, native, read-only **Platform** root capability (`R15`).
//!
//! This module is the only place in the project that opens the installation's `platform/` tree.
//! It owns its **own** adopted `source_read_native::RootHandle`, distinct from
//! [`crate::source_read::ContentRootCapability`] and from the Entity `assets/` root, and it
//! resolves exactly four leaf shapes beneath the `assets/` namespace of that handle:
//!
//! ```text
//! assets/catalogue-1.0.0.json            the approved catalogue document
//! assets/icons/<catalogue basename>.svg  one approved image source
//! assets/licensing/TABLER-ICONS-MIT.txt  the retained Tabler MIT notice
//! assets/licensing/icon-provenance.json  the retained provenance record
//! ```
//!
//! **No path assembly, no second reader.** Every component handed to the native `open_leaf` is a
//! single validated segment produced by the inward identifier type or by a fixed constant. There
//! is no `Path::join` on authored text, no `canonicalize`, no `starts_with` on a path string, no
//! pre-open `metadata` used as proof, no second filesystem reader and no `std::fs::read` in the
//! source port. Confinement, immutable transfer and the before/after re-observation are the
//! accepted `source_read_native` mechanism unchanged: on Linux an anonymous copy-on-write clone
//! on a verified reflink filesystem, on Windows a handle that denies write and delete sharing. A
//! host that cannot provide that mechanism refuses; it never silently falls back to a plain copy.
//!
//! **The capability is read-only by API.** It has no create, rename, write, truncate, delete or
//! public-serving method, and it holds no publication, derivative, release or deployment
//! authority. It cannot attest that the operator supplied the right root merely from a path, and
//! it proves no global multi-file snapshot: each transferred file is individually immutable for
//! its read, and whole-set correctness rests on the exact catalogue and source pins.

use std::path::Path;

use solo_application::{
    MAX_PLATFORM_CATALOGUE_BYTES_V1, MAX_PLATFORM_SOURCE_BYTES_V1, PhysicalIdentityV1,
    PlatformAssetLibraryReadPortV1, PlatformAssetReadRefusalKindV1, PlatformAssetReadRefusalV1,
    PlatformAssetRefV1, PlatformAssetSourceSetV1, PlatformAssetSourceV1, PlatformCatalogueSourceV1,
    PlatformLibraryReadOutcomeV1, PlatformRootProvenanceV1, PlatformRootSelectionV1,
    fingerprint_of, read_pinned_platform_sources_v1, sha256_media_worker_bytes_v1,
};

use crate::platform_asset_catalogue::parse_platform_asset_catalogue_v1;
use crate::source_read::TrustedDirectoryHandle;
use crate::source_read_native::{self as native, NativeError};

/// The fixed namespace segment every Platform document is resolved beneath.
const ASSETS_SEGMENT: &str = "assets";

/// The one approved catalogue document name.
const CATALOGUE_LEAF: &str = "catalogue-1.0.0.json";

/// The fixed licensing directory and its two retained documents.
const LICENSING_SEGMENT: &str = "licensing";
const MIT_NOTICE_LEAF: &str = "TABLER-ICONS-MIT.txt";
const PROVENANCE_LEAF: &str = "icon-provenance.json";

/// The images sub-namespace.
const ICONS_SEGMENT: &str = "icons";

/// The one admitted Platform image extension.
const ICON_EXTENSION: &str = "svg";

/// One bounded licensing document the operator may read beside the catalogue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformLicensingDocumentV1 {
    /// The retained Tabler MIT notice.
    MitNotice,
    /// The retained machine provenance record.
    Provenance,
}

impl PlatformLicensingDocumentV1 {
    /// The exact leaf name of this document.
    #[must_use]
    pub const fn leaf(self) -> &'static str {
        match self {
            Self::MitNotice => MIT_NOTICE_LEAF,
            Self::Provenance => PROVENANCE_LEAF,
        }
    }
}

/// What one confined read of one Platform document observed.
struct PlatformDocumentV1 {
    bytes: Vec<u8>,
    identity: PhysicalIdentityV1,
    length: u64,
    links: u64,
    fingerprint: u64,
}

/// An explicitly adopted, read-only Platform root capability.
#[derive(Debug)]
pub struct PlatformRootCapabilityV1 {
    root: native::RootHandle,
    provenance: PlatformRootProvenanceV1,
}

impl PlatformRootCapabilityV1 {
    /// Opens the operator-designated `platform/` directory **once** and records what was opened.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the root cannot be opened, is not a directory, or is itself a
    /// symlink or reparse point.
    pub fn adopt_bootstrap_path(
        path: &Path,
        provenance: PlatformRootProvenanceV1,
    ) -> Result<Self, PlatformAssetReadRefusalV1> {
        native::RootHandle::adopt_bootstrap_path(path)
            .map(|root| Self { root, provenance })
            .map_err(map_native_refusal)
    }

    /// Takes ownership of a directory handle a trusted bootstrap already opened.
    ///
    /// # Errors
    ///
    /// Returns a refusal when the supplied handle is not a directory, names a symlink or reparse
    /// point, or cannot be interrogated.
    pub fn from_trusted_directory(
        handle: TrustedDirectoryHandle,
        provenance: PlatformRootProvenanceV1,
    ) -> Result<Self, PlatformAssetReadRefusalV1> {
        native::RootHandle::from_trusted_directory(handle)
            .map(|root| Self { root, provenance })
            .map_err(map_native_refusal)
    }

    /// The trust assertion the caller made when adopting this capability.
    #[must_use]
    pub const fn provenance(&self) -> PlatformRootProvenanceV1 {
        self.provenance
    }

    /// The explicit root-selection token the inward contract requires.
    #[must_use]
    pub const fn selection(&self) -> PlatformRootSelectionV1 {
        PlatformRootSelectionV1::adopt(self.provenance)
    }

    /// The physical identity recorded when the capability was adopted.
    #[must_use]
    pub fn identity(&self) -> PhysicalIdentityV1 {
        self.root.identity()
    }

    /// The host mechanism this reader uses, for the execution report.
    #[must_use]
    pub const fn mechanism(&self) -> &'static str {
        self.root.mechanism()
    }

    /// Reads one leaf by already-validated single segments, within `max_bytes`.
    fn read_document(
        &self,
        components: &[&str],
        max_bytes: u64,
    ) -> Result<PlatformDocumentV1, PlatformAssetReadRefusalKindV1> {
        let mut leaf = native::open_leaf(&self.root, components).map_err(map_native_kind)?;
        let before = leaf.facts();
        if before.length > max_bytes {
            return Err(PlatformAssetReadRefusalKindV1::OversizeSource);
        }
        let bytes = match native::read_bounded(&self.root, &mut leaf, max_bytes) {
            Ok(bytes) => bytes,
            Err(error) => {
                // A file that grew past the bound between the open and the transfer is an oversize
                // source, not an unsafe one, and the distinction is observed on the handle.
                let grew = native::recheck(&leaf)
                    .is_ok_and(|after: native::LeafFacts| after.length > max_bytes);
                return Err(if grew {
                    PlatformAssetReadRefusalKindV1::OversizeSource
                } else {
                    map_native_kind(error)
                });
            }
        };
        let after = native::recheck(&leaf).map_err(map_native_kind)?;
        let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if after != before || transferred != before.length {
            return Err(PlatformAssetReadRefusalKindV1::ChangedDuringRead);
        }
        if before.links != 1 || after.links != 1 {
            return Err(PlatformAssetReadRefusalKindV1::IdentityCollision);
        }
        let fingerprint = fingerprint_of(&bytes);
        Ok(PlatformDocumentV1 {
            bytes,
            identity: before.identity,
            length: before.length,
            links: before.links,
            fingerprint,
        })
    }

    /// Reads the approved catalogue document under its own bound.
    ///
    /// A missing catalogue leaf is reported as [`PlatformAssetReadRefusalKindV1::CatalogueUnavailable`]
    /// rather than as a missing *asset*: the library's own catalogue is what could not be produced.
    #[must_use]
    pub fn read_catalogue_document(&self, max_bytes: u64) -> PlatformCatalogueSourceV1 {
        let max_bytes = max_bytes.min(MAX_PLATFORM_CATALOGUE_BYTES_V1);
        match self.read_document(&[ASSETS_SEGMENT, CATALOGUE_LEAF], max_bytes) {
            Ok(document) => PlatformCatalogueSourceV1::Present {
                bytes: document.bytes,
                identity: document.identity,
                length: document.length,
                links: document.links,
                fingerprint: document.fingerprint,
            },
            Err(kind) => PlatformCatalogueSourceV1::Refused(match kind {
                PlatformAssetReadRefusalKindV1::MissingSource => {
                    PlatformAssetReadRefusalKindV1::CatalogueUnavailable
                }
                other => other,
            }),
        }
    }

    /// Reads one retained licensing document, or refuses for the stated reason.
    ///
    /// # Errors
    ///
    /// Returns a closed refusal category; never a path.
    pub fn read_licensing_document(
        &self,
        document: PlatformLicensingDocumentV1,
        max_bytes: u64,
    ) -> Result<Vec<u8>, PlatformAssetReadRefusalV1> {
        self.read_document(
            &[ASSETS_SEGMENT, LICENSING_SEGMENT, document.leaf()],
            max_bytes,
        )
        .map(|read| read.bytes)
        .map_err(PlatformAssetReadRefusalV1::of)
    }

    /// Whether one catalogue identifier names a single portable `.svg` component.
    ///
    /// The inward type has already proven this; the check is repeated here so the native boundary
    /// stays safe even for a caller that reached it another way.
    fn icon_leaf(asset_id: &str) -> Result<&str, PlatformAssetReadRefusalKindV1> {
        let refused = PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier;
        if !asset_id.is_ascii() || asset_id.contains('\\') || asset_id.contains('\0') {
            return Err(refused);
        }
        let mut segments = asset_id.split('/');
        let (Some(directory), Some(name), None) =
            (segments.next(), segments.next(), segments.next())
        else {
            return Err(refused);
        };
        if directory != ICONS_SEGMENT
            || name.is_empty()
            || name.len() > 80
            || name.starts_with('.')
            || name.contains("..")
            || !name.ends_with(&format!(".{ICON_EXTENSION}"))
            || name.len() <= ICON_EXTENSION.len() + 1
            || !name.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || matches!(character, '.' | '_' | '-')
            })
        {
            return Err(refused);
        }
        Ok(name)
    }
}

impl PlatformAssetLibraryReadPortV1 for PlatformRootCapabilityV1 {
    fn read_catalogue(&self, max_bytes: u64) -> PlatformCatalogueSourceV1 {
        self.read_catalogue_document(max_bytes)
    }

    fn read_platform_source(&self, asset_id: &str, max_bytes: u64) -> PlatformAssetSourceV1 {
        let max_bytes = max_bytes.min(MAX_PLATFORM_SOURCE_BYTES_V1);
        let leaf = match Self::icon_leaf(asset_id) {
            Ok(leaf) => leaf,
            Err(kind) => return PlatformAssetSourceV1::Refused(kind),
        };
        match self.read_document(&[ASSETS_SEGMENT, ICONS_SEGMENT, leaf], max_bytes) {
            Ok(document) => PlatformAssetSourceV1::Present {
                bytes: document.bytes,
                identity: document.identity,
                length: document.length,
                links: document.links,
                fingerprint: document.fingerprint,
            },
            Err(kind) => PlatformAssetSourceV1::Refused(kind),
        }
    }
}

/// Maps a native refusal onto the closed Platform category.
const fn map_native_kind(error: NativeError) -> PlatformAssetReadRefusalKindV1 {
    match error {
        // `Absent` is a refusal here, never a benign result: a declared Platform source that does
        // not exist fails the whole request.
        NativeError::Absent => PlatformAssetReadRefusalKindV1::MissingSource,
        NativeError::Unsafe => PlatformAssetReadRefusalKindV1::UnsafeSourceLocation,
        NativeError::PermissionDenied => PlatformAssetReadRefusalKindV1::PermissionDenied,
        NativeError::Changed => PlatformAssetReadRefusalKindV1::ChangedDuringRead,
        NativeError::OtherIo => PlatformAssetReadRefusalKindV1::OtherIo,
        NativeError::Unsupported => PlatformAssetReadRefusalKindV1::Unsupported,
    }
}

/// Maps a native refusal onto the public Platform refusal.
fn map_native_refusal(error: NativeError) -> PlatformAssetReadRefusalV1 {
    PlatformAssetReadRefusalV1::of(map_native_kind(error))
}

/// Which stage refused a pinned Platform icon inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformInspectionStageV1 {
    /// The catalogue pin, membership or confined source read refused.
    SourceRead,
    /// The accepted SVG worker refused, or its observation did not match the approved entry.
    MediaInspection,
}

/// A closed, path-free refusal for one whole pinned Platform icon inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformInspectionRefusalV1 {
    stage: PlatformInspectionStageV1,
    source: Option<PlatformAssetReadRefusalV1>,
    position: Option<usize>,
}

impl PlatformInspectionRefusalV1 {
    /// A refusal taken during source acquisition, carrying the inward refusal unchanged.
    #[must_use]
    pub const fn source(source: PlatformAssetReadRefusalV1) -> Self {
        Self {
            stage: PlatformInspectionStageV1::SourceRead,
            source: Some(source),
            position: None,
        }
    }

    /// A refusal taken while one worker's observation was validated.
    #[must_use]
    pub const fn media(position: usize) -> Self {
        Self {
            stage: PlatformInspectionStageV1::MediaInspection,
            source: None,
            position: Some(position),
        }
    }

    /// The stage that refused.
    #[must_use]
    pub const fn stage(self) -> PlatformInspectionStageV1 {
        self.stage
    }

    /// The inward source refusal, when this inspection refused before any media work.
    #[must_use]
    pub const fn source_refusal(self) -> Option<PlatformAssetReadRefusalV1> {
        self.source
    }

    /// The requested ordinal the media refusal concerns, when one is named.
    #[must_use]
    pub const fn position(self) -> Option<usize> {
        self.position
    }
}

/// The accepted media evidence for one approved Platform icon.
///
/// The two digests are deliberately distinct: `source_sha256` is the unmodified approved source
/// the catalogue declares, and `canonical_sha256` describes the worker's separate canonical
/// output. Neither is a display, plane or public-asset digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPinnedIconEvidenceV1 {
    id: String,
    source_sha256: String,
    canonical_sha256: String,
    width: u32,
    height: u32,
    elements: usize,
    depth: usize,
    path_commands: usize,
    worker_pid: u32,
}

impl PlatformPinnedIconEvidenceV1 {
    /// The exact catalogue identifier.
    #[must_use]
    pub fn asset_id(&self) -> &str {
        &self.id
    }

    /// The SHA-256 of the unmodified approved source bytes.
    #[must_use]
    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }

    /// The SHA-256 of the worker's canonical output.
    #[must_use]
    pub fn canonical_sha256(&self) -> &str {
        &self.canonical_sha256
    }

    /// The observed vector width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The observed vector height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The observed element count.
    #[must_use]
    pub const fn elements(&self) -> usize {
        self.elements
    }

    /// The observed nesting depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// The observed path-command count.
    #[must_use]
    pub const fn path_commands(&self) -> usize {
        self.path_commands
    }

    /// The process identifier the real child reported. Never `0`.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }
}

/// The complete ordered evidence of one pinned Platform icon inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPinnedIconSetV1 {
    records: Vec<PlatformPinnedIconEvidenceV1>,
    source_set: PlatformAssetSourceSetV1,
}

impl PlatformPinnedIconSetV1 {
    /// Every inspected asset, in the request's own order.
    #[must_use]
    pub fn records(&self) -> &[PlatformPinnedIconEvidenceV1] {
        &self.records
    }

    /// How many assets were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether nothing was inspected. Never true for a complete set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// The accepted source set this inspection was built from.
    #[must_use]
    pub const fn source_set(&self) -> &PlatformAssetSourceSetV1 {
        &self.source_set
    }

    /// The namespace the sources came from, for the execution report. Always `platform`.
    #[must_use]
    pub const fn source_root(&self) -> &'static str {
        "platform"
    }

    /// Whether this inspection authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }

    /// Whether this inspection produced a public derivative. Always `false`.
    #[must_use]
    pub const fn produces_public_derivative(&self) -> bool {
        false
    }

    /// Whether this inspection proves a global multi-file snapshot. Always `false`.
    #[must_use]
    pub const fn proves_global_snapshot(&self) -> bool {
        false
    }
}

/// The outcome of one whole pinned Platform icon inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformPinnedIconInspectionV1 {
    /// Every requested asset was read and inspected.
    Complete(PlatformPinnedIconSetV1),
    /// One stage refused the whole request.
    Refused(PlatformInspectionRefusalV1),
}

/// Reads the approved catalogue, acquires every requested Platform source and inspects each one
/// with the already-accepted confined SVG worker.
///
/// The order is deliberate and load-bearing:
///
/// 1. the catalogue is read once through the Platform capability and parsed by the unchanged
///    strict R12 parser with the caller's independently selected version;
/// 2. [`read_pinned_platform_sources_v1`] pins those exact bytes to the approved SHA-256, checks
///    membership and hashes every source **before** any worker exists;
/// 3. only then does one real `solo-cli` SVG child run per distinct asset, and its observation is
///    validated against the same approved entry.
///
/// A refusal at any step discards the whole request: no partial set, no Entity fallback, no
/// publication, no derivative and no release authority.
#[must_use]
pub fn inspect_pinned_platform_icon_sources_v1(
    capability: &PlatformRootCapabilityV1,
    selected_version: &str,
    requests: &[PlatformAssetRefV1],
    executable: &Path,
) -> PlatformPinnedIconInspectionV1 {
    // The selected version is checked before anything is read, so an unselected version can never
    // be reported as a catalogue problem.
    if selected_version != solo_application::APPROVED_PLATFORM_LIBRARY_VERSION_V1 {
        return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::source(
            PlatformAssetReadRefusalV1::of(PlatformAssetReadRefusalKindV1::UnselectedVersion),
        ));
    }
    let catalogue_bytes = match capability.read_catalogue(MAX_PLATFORM_CATALOGUE_BYTES_V1) {
        PlatformCatalogueSourceV1::Present { bytes, .. } => bytes,
        PlatformCatalogueSourceV1::Refused(kind) => {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::source(
                PlatformAssetReadRefusalV1::of(kind),
            ));
        }
    };
    // The unchanged strict parser, with the version supplied independently of the document.
    let catalogue = match parse_platform_asset_catalogue_v1(&catalogue_bytes, selected_version) {
        Ok(catalogue) => catalogue,
        Err(_) => {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::source(
                PlatformAssetReadRefusalV1::of(PlatformAssetReadRefusalKindV1::CatalogueUnapproved),
            ));
        }
    };
    let source_set = match read_pinned_platform_sources_v1(
        capability.selection(),
        selected_version,
        &catalogue,
        requests,
        capability,
    ) {
        PlatformLibraryReadOutcomeV1::Complete(set) => set,
        PlatformLibraryReadOutcomeV1::Refused(refusal) => {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::source(
                refusal,
            ));
        }
    };

    let mut records: Vec<PlatformPinnedIconEvidenceV1> = Vec::new();
    for (position, evidence) in source_set.evidence().iter().enumerate() {
        let entry = evidence.entry();
        let source_digest = evidence.source_sha256();
        let observation =
            match crate::graphic_image_inspection::inspect_svg_bytes_with_executable_v1(
                executable,
                evidence.bytes(),
            ) {
                Ok(observation) => observation,
                Err(_) => {
                    return PlatformPinnedIconInspectionV1::Refused(
                        PlatformInspectionRefusalV1::media(position),
                    );
                }
            };
        let canonical_sha = sha256_media_worker_bytes_v1(observation.canonical()).to_hex();
        if observation.width() != entry.width() || observation.height() != entry.height() {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::media(
                position,
            ));
        }
        if observation.source_digest().to_hex() != source_digest
            || observation.output_digest().to_hex() != canonical_sha
        {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::media(
                position,
            ));
        }
        if observation.worker_pid() == 0
            || !observation.limits_verified()
            || !observation.cleanup_verified()
        {
            return PlatformPinnedIconInspectionV1::Refused(PlatformInspectionRefusalV1::media(
                position,
            ));
        }
        records.push(PlatformPinnedIconEvidenceV1 {
            id: evidence.asset_id().to_owned(),
            source_sha256: source_digest,
            canonical_sha256: canonical_sha,
            width: observation.width(),
            height: observation.height(),
            elements: observation.elements(),
            depth: observation.depth(),
            path_commands: observation.path_commands(),
            worker_pid: observation.worker_pid(),
        });
    }

    PlatformPinnedIconInspectionV1::Complete(PlatformPinnedIconSetV1 {
        records,
        source_set,
    })
}
