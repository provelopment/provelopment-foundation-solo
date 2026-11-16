//! Owner-approved Platform library **source** acquisition (`R15`).
//!
//! This module is the inward half of one narrow, nonpublishing gate: given an independently
//! selected library version, an explicitly adopted **Platform** root capability and one or more
//! typed Platform asset references, acquire the exact approved source bytes of the pinned
//! `1.0.0` library or refuse the whole request.
//!
//! **Trusted selection, never autodiscovery.** Nothing here looks at an Entity root, searches a
//! directory, reads a route, a locale, a role, a basename, a manifest fallback, a URL or a
//! repository-relative default. The version is supplied by the caller, the root capability is
//! adopted explicitly and the requested asset identifiers are typed values. A missing version,
//! an absent root, an `Entity`-namespaced reference, an unapproved catalogue or an unknown
//! identifier refuses **before** any source read and therefore before any worker launch.
//!
//! **The catalogue is pinned by bytes, not by its own claim.** The reader supplies the raw
//! product catalogue bytes; this module hashes exactly those bytes with the accepted
//! cryptographic utility and refuses unless they are
//! [`APPROVED_PLATFORM_CATALOGUE_SHA256_V1`]. The typed catalogue travels with those same bytes
//! and is cross-checked against them for every requested identifier and digest, so an
//! inconsistent model cannot smuggle an unapproved digest: the approved byte set is fixed, and
//! a digest it does not declare cannot be matched. The strict R12 parser in the adapter layer
//! stays unchanged; this module never re-validates its syntax and never repairs it.
//!
//! **Sources are hashed here.** For every requested asset the actual read bytes are hashed in
//! this module and compared with the catalogue's declared digest for that exact identifier
//! *before* any media inspection. A port-supplied digest, an operating-system filename, a worker
//! output or a non-cryptographic fingerprint is never accepted as the source identity.
//!
//! **Whole-set discipline.** The result is one complete set or one refusal. A late failure —
//! including a corrupted last source after earlier matches — discards every earlier record, so no
//! caller can mistake a partial read for a verified library, and a refusal never degrades into an
//! empty set or an Entity fallback.
//!
//! **What a success does not prove.** It does not prove that the bytes are valid or sanitized SVG
//! media, that a derivative or canonical output exists, that a route or visibility was chosen,
//! that a global multi-file snapshot exists, or that publication, release or deployment is
//! authorized. [`PLATFORM_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY_V1`],
//! [`PLATFORM_ASSET_READ_PROVES_MEDIA_V1`] and [`PLATFORM_ASSET_READ_PROVES_GLOBAL_SNAPSHOT_V1`]
//! are the machine-checkable statement of that boundary.

use std::collections::BTreeMap;
use std::fmt;

use solo_domain::{AssetNamespace, AssetReferenceV1};

use crate::media_worker::sha256_media_worker_bytes_v1;
use crate::platform_asset_catalogue::{
    PLATFORM_ASSET_LIBRARY_VERSION_V1, PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1,
};
use crate::source_read::PhysicalIdentityV1;

/// The one library version this gate can ever read.
pub const APPROVED_PLATFORM_LIBRARY_VERSION_V1: &str = PLATFORM_ASSET_LIBRARY_VERSION_V1;

/// The approved catalogue, pinned by SHA-256 of its exact raw bytes.
///
/// This value is an independent authority: it is not read from the catalogue, not supplied by the
/// reader and not derived from the parsed model.
pub const APPROVED_PLATFORM_CATALOGUE_SHA256_V1: &str =
    "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

/// The approved number of declared assets in library `1.0.0`.
pub const APPROVED_PLATFORM_ASSET_COUNT_V1: usize = 81;

/// The maximum number of source bytes one Platform asset may contribute.
pub const MAX_PLATFORM_SOURCE_BYTES_V1: u64 = 20 * 1024 * 1024;

/// The maximum number of bytes the catalogue document may occupy.
pub const MAX_PLATFORM_CATALOGUE_BYTES_V1: u64 = 2 * 1024 * 1024;

/// Whether acquiring Platform source bytes authorizes publication of any kind. Always `false`.
pub const PLATFORM_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Whether acquiring Platform source bytes proves the media itself. Always `false`.
pub const PLATFORM_ASSET_READ_PROVES_MEDIA_V1: bool = false;

/// Whether acquiring Platform source bytes proves a filesystem-wide snapshot. Always `false`.
pub const PLATFORM_ASSET_READ_PROVES_GLOBAL_SNAPSHOT_V1: bool = false;

/// The namespace segment a Platform reference is resolved beneath.
pub const PLATFORM_ASSET_NAMESPACE_SEGMENT_V1: &str = "assets";

/// The one sub-namespace the approved library declares its images in.
pub const PLATFORM_ICON_DIRECTORY_V1: &str = "icons";

/// The one admitted Platform image extension.
pub const PLATFORM_IMAGE_EXTENSION_V1: &str = "svg";

/// The ceiling on one portable identifier component, in bytes.
const MAX_IDENTIFIER_COMPONENT_BYTES_V1: usize = 80;

/// Why one Platform library read produced no trustworthy source evidence.
///
/// Every variant refuses the whole request. None is absence-as-success and none may be
/// downgraded to an empty or partial result by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformAssetReadRefusalKindV1 {
    /// The selected library version is absent or is not the approved version.
    UnselectedVersion,
    /// The caller supplied no explicitly adopted Platform root capability.
    MissingRootCapability,
    /// The requested set is empty.
    EmptyRequestedSet,
    /// The request names one identifier more than once.
    DuplicateRequestedAsset,
    /// A reference is `Entity`-namespaced; this gate can never read the Entity namespace.
    EntityNamespaceRefused,
    /// The reference is not a portable lowercase `icons/<name>.svg` Platform identifier.
    UnsafeAssetIdentifier,
    /// The requested identifier is not declared by the approved catalogue.
    UnapprovedAssetIdentifier,
    /// The catalogue could not be produced by the reader at all.
    CatalogueUnavailable,
    /// The catalogue exceeded the admitted bound.
    CatalogueOversize,
    /// The catalogue bytes are not the approved catalogue.
    CatalogueUnapproved,
    /// The typed catalogue is inconsistent with the approved bytes for a requested asset.
    CatalogueModelMismatch,
    /// The catalogue does not declare the approved number of assets.
    CatalogueCountMismatch,
    /// The declared source positively does not exist.
    MissingSource,
    /// The location or object is unsafe: a symlink, reparse point, non-regular object, escape,
    /// unsafe root or wrong root.
    UnsafeSourceLocation,
    /// Access to some component or to the leaf was refused.
    PermissionDenied,
    /// The object changed identity, length, link count or timestamps while it was read.
    ChangedDuringRead,
    /// Link count other than exactly one, or two distinct identifiers resolving to one physical
    /// object.
    IdentityCollision,
    /// The source exceeds [`MAX_PLATFORM_SOURCE_BYTES_V1`] and is refused, never truncated.
    OversizeSource,
    /// The transferred bytes are not the digest the approved catalogue declares for that
    /// identifier.
    SourceDigestMismatch,
    /// The host cannot provide the required confinement, identity or immutable-transfer
    /// guarantee, so nothing is reported.
    Unsupported,
    /// Some other operating-system fault prevented a trustworthy result.
    OtherIo,
}

impl PlatformAssetReadRefusalKindV1 {
    /// Stable, safe diagnostic token. It carries no path, identifier or handle.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnselectedVersion => "PLATFORM_VERSION_UNSELECTED",
            Self::MissingRootCapability => "PLATFORM_ROOT_ABSENT",
            Self::EmptyRequestedSet => "PLATFORM_REQUEST_EMPTY",
            Self::DuplicateRequestedAsset => "PLATFORM_REQUEST_DUPLICATE",
            Self::EntityNamespaceRefused => "PLATFORM_ENTITY_NAMESPACE",
            Self::UnsafeAssetIdentifier => "PLATFORM_IDENTIFIER_UNSAFE",
            Self::UnapprovedAssetIdentifier => "PLATFORM_IDENTIFIER_UNAPPROVED",
            Self::CatalogueUnavailable => "PLATFORM_CATALOGUE_UNAVAILABLE",
            Self::CatalogueOversize => "PLATFORM_CATALOGUE_OVERSIZE",
            Self::CatalogueUnapproved => "PLATFORM_CATALOGUE_UNAPPROVED",
            Self::CatalogueModelMismatch => "PLATFORM_CATALOGUE_MODEL_MISMATCH",
            Self::CatalogueCountMismatch => "PLATFORM_CATALOGUE_COUNT_MISMATCH",
            Self::MissingSource => "PLATFORM_SOURCE_MISSING",
            Self::UnsafeSourceLocation => "PLATFORM_SOURCE_UNSAFE_LOCATION",
            Self::PermissionDenied => "PLATFORM_SOURCE_PERMISSION_DENIED",
            Self::ChangedDuringRead => "PLATFORM_SOURCE_CHANGED",
            Self::IdentityCollision => "PLATFORM_SOURCE_IDENTITY_COLLISION",
            Self::OversizeSource => "PLATFORM_SOURCE_OVERSIZE",
            Self::SourceDigestMismatch => "PLATFORM_SOURCE_DIGEST_MISMATCH",
            Self::Unsupported => "PLATFORM_SOURCE_UNSUPPORTED",
            Self::OtherIo => "PLATFORM_SOURCE_IO_ERROR",
        }
    }

    /// Fixed, bounded explanation carrying no authored value.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::UnselectedVersion => {
                "the selected Platform library version is not the approved one"
            }
            Self::MissingRootCapability => {
                "no explicitly adopted Platform root capability was supplied"
            }
            Self::EmptyRequestedSet => "no Platform asset was requested",
            Self::DuplicateRequestedAsset => "the request repeats one Platform identifier",
            Self::EntityNamespaceRefused => {
                "an Entity-namespaced reference is not a Platform source"
            }
            Self::UnsafeAssetIdentifier => "the Platform identifier is not a portable image path",
            Self::UnapprovedAssetIdentifier => {
                "the Platform identifier is not declared by the approved catalogue"
            }
            Self::CatalogueUnavailable => "the approved catalogue could not be read",
            Self::CatalogueOversize => "the catalogue exceeds its admitted bound",
            Self::CatalogueUnapproved => "the catalogue bytes are not the approved catalogue",
            Self::CatalogueModelMismatch => {
                "the typed catalogue disagrees with the approved catalogue bytes"
            }
            Self::CatalogueCountMismatch => "the catalogue does not declare 81 assets",
            Self::MissingSource => "the declared Platform source does not exist",
            Self::UnsafeSourceLocation => "the Platform source location or object is unsafe",
            Self::PermissionDenied => "access to the Platform source was denied",
            Self::ChangedDuringRead => "the Platform source changed while it was read",
            Self::IdentityCollision => "the Platform source identity is ambiguous",
            Self::OversizeSource => "the Platform source exceeds its admitted bound",
            Self::SourceDigestMismatch => "the Platform source is not its declared approved digest",
            Self::Unsupported => "the host cannot provide the required read guarantee",
            Self::OtherIo => "a Platform read failed for another operating-system reason",
        }
    }
}

impl fmt::Display for PlatformAssetReadRefusalKindV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

/// One typed refusal: a closed category and, at most, the ordinal of the request it concerns.
///
/// Neither a filesystem path nor an asset identifier is carried, so a diagnostic can be rendered
/// without leaking a source location, a private declaration or catalogue text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformAssetReadRefusalV1 {
    kind: PlatformAssetReadRefusalKindV1,
    position: Option<usize>,
}

impl PlatformAssetReadRefusalV1 {
    /// A refusal taken before any single asset was examined.
    #[must_use]
    pub const fn of(kind: PlatformAssetReadRefusalKindV1) -> Self {
        Self {
            kind,
            position: None,
        }
    }

    /// The same refusal, attributed to the requested ordinal it concerns.
    #[must_use]
    pub const fn at(kind: PlatformAssetReadRefusalKindV1, position: usize) -> Self {
        Self {
            kind,
            position: Some(position),
        }
    }

    /// The stable category.
    #[must_use]
    pub const fn kind(self) -> PlatformAssetReadRefusalKindV1 {
        self.kind
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn code(self) -> &'static str {
        self.kind.code()
    }

    /// The fixed, bounded explanation.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        self.kind.detail()
    }

    /// The zero-based ordinal of the requested asset this refusal concerns, when one is named.
    #[must_use]
    pub const fn position(self) -> Option<usize> {
        self.position
    }
}

impl fmt::Display for PlatformAssetReadRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for PlatformAssetReadRefusalV1 {}

/// Why the caller asserts that a directory is the installation Platform root.
///
/// The reader cannot attest provenance; it can only refuse to *assume* it. Requiring this value
/// makes the trust input explicit at every call site, and a Platform capability can therefore
/// never be confused with the Entity content-root capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformRootProvenanceV1 {
    /// The operator designated this directory as the installation's Platform root.
    OperatorDesignatedPlatformRoot,
}

impl PlatformRootProvenanceV1 {
    /// A stable code for the execution report.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::OperatorDesignatedPlatformRoot => "operator-designated-platform-root",
        }
    }
}

/// The explicit, separately adopted Platform root capability the caller supplies.
///
/// Construction is a deliberate act with a visible name; no default value exists, and the use
/// case refuses without one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformRootSelectionV1 {
    provenance: PlatformRootProvenanceV1,
}

impl PlatformRootSelectionV1 {
    /// Adopts one explicitly designated Platform root.
    #[must_use]
    pub const fn adopt(provenance: PlatformRootProvenanceV1) -> Self {
        Self { provenance }
    }

    /// The trust assertion the caller made.
    #[must_use]
    pub const fn provenance(self) -> PlatformRootProvenanceV1 {
        self.provenance
    }
}

/// One validated, confined Platform asset identifier.
///
/// The value is the catalogue's own `icons/<name>.svg` spelling, unchanged. It names no
/// operating-system path, opens nothing and proves no membership: membership is decided only
/// against the approved catalogue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlatformAssetRefV1 {
    id: String,
}

impl PlatformAssetRefV1 {
    /// Validates one typed Platform reference into its exact catalogue identifier.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformAssetReadRefusalKindV1::EntityNamespaceRefused`] for an `Entity`
    /// reference, which this gate can never read, and
    /// [`PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier`] for any spelling outside the
    /// portable lowercase `icons/<name>.svg` grammar.
    pub fn from_reference(
        reference: &AssetReferenceV1,
    ) -> Result<Self, PlatformAssetReadRefusalV1> {
        if reference.namespace() != AssetNamespace::Platform {
            return Err(PlatformAssetReadRefusalV1::of(
                PlatformAssetReadRefusalKindV1::EntityNamespaceRefused,
            ));
        }
        Self::from_identifier(reference.reference())
    }

    /// Validates one catalogue identifier directly, for a caller that already holds one.
    ///
    /// # Errors
    ///
    /// Returns [`PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier`] unless the value is
    /// exactly `icons/` followed by one portable lowercase `.svg` component.
    pub fn from_identifier(identifier: &str) -> Result<Self, PlatformAssetReadRefusalV1> {
        let refuse = || {
            PlatformAssetReadRefusalV1::of(PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier)
        };
        if identifier.is_empty() || !identifier.is_ascii() {
            return Err(refuse());
        }
        let mut segments = identifier.split('/');
        let (Some(directory), Some(name), None) =
            (segments.next(), segments.next(), segments.next())
        else {
            return Err(refuse());
        };
        if directory != PLATFORM_ICON_DIRECTORY_V1
            || !portable_component(name)
            || !name.ends_with(&format!(".{PLATFORM_IMAGE_EXTENSION_V1}"))
            || name.len() <= PLATFORM_IMAGE_EXTENSION_V1.len() + 1
        {
            return Err(refuse());
        }
        Ok(Self {
            id: identifier.to_owned(),
        })
    }

    /// The exact catalogue identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.id
    }

    /// The single file component, without the `icons/` directory.
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.id
            .split_once('/')
            .map_or(self.id.as_str(), |(_, name)| name)
    }
}

/// Whether one identifier component is a portable lowercase filename component.
fn portable_component(segment: &str) -> bool {
    if segment.is_empty() || segment.len() > MAX_IDENTIFIER_COMPONENT_BYTES_V1 {
        return false;
    }
    if segment.starts_with('.') || segment.ends_with('.') || segment.contains("..") {
        return false;
    }
    segment.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '.' | '_' | '-')
    })
}

/// What one outbound read of the catalogue observed.
///
/// `Present` carries the raw bytes the reader actually transferred and the observations it made
/// on that handle. The application hashes those bytes itself and never trusts a supplied digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformCatalogueSourceV1 {
    /// The catalogue was read under its bound and its bytes are owned here.
    Present {
        /// The owned, immutable bytes exactly as transferred.
        bytes: Vec<u8>,
        /// Physical identity observed on the handle before and after the transfer.
        identity: PhysicalIdentityV1,
        /// Byte length that same handle reported and that the transfer had to match.
        length: u64,
        /// How many directory entries reference the object.
        links: u64,
        /// Non-cryptographic fingerprint of exactly these bytes.
        fingerprint: u64,
    },
    /// The catalogue was not produced, for the stated reason.
    Refused(PlatformAssetReadRefusalKindV1),
}

/// What one outbound read of one Platform asset source observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformAssetSourceV1 {
    /// The confined source was read under its bound and its bytes are owned here.
    Present {
        /// The owned, immutable bytes exactly as transferred.
        bytes: Vec<u8>,
        /// Physical identity observed on the handle before and after the transfer.
        identity: PhysicalIdentityV1,
        /// Byte length that same handle reported and that the transfer had to match.
        length: u64,
        /// How many directory entries reference the object.
        links: u64,
        /// Non-cryptographic fingerprint of exactly these bytes.
        fingerprint: u64,
    },
    /// The source was not produced, for the stated reason.
    Refused(PlatformAssetReadRefusalKindV1),
}

/// The two readings one Platform library acquisition needs, and nothing more.
///
/// It is deliberately narrower than a filesystem API: it cannot list, scan, glob, guess an
/// extension, resolve an Entity reference or return a usable handle. Every refusal is a closed
/// category, so a port cannot leak an operating-system path into a diagnostic.
pub trait PlatformAssetLibraryReadPortV1 {
    /// Reads the product catalogue document, within `max_bytes`.
    fn read_catalogue(&self, max_bytes: u64) -> PlatformCatalogueSourceV1;

    /// Reads one exact catalogue identifier's own source bytes, within `max_bytes`.
    fn read_platform_source(&self, asset_id: &str, max_bytes: u64) -> PlatformAssetSourceV1;
}

/// Source-byte evidence for exactly one requested Platform asset.
///
/// Holding one proves that these bytes are the ones one confined read transferred from the
/// object whose identity is recorded, and that their SHA-256 is the digest the approved
/// catalogue declares for this exact identifier. It proves nothing about media validity,
/// sanitizer acceptance, derivative output or publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetSourceEvidenceV1 {
    entry: PlatformAssetCatalogueEntryV1,
    bytes: Vec<u8>,
    identity: PhysicalIdentityV1,
    length: u64,
    fingerprint: u64,
}

impl PlatformAssetSourceEvidenceV1 {
    /// The approved catalogue entry this evidence answers, unchanged.
    #[must_use]
    pub const fn entry(&self) -> &PlatformAssetCatalogueEntryV1 {
        &self.entry
    }

    /// The exact catalogue identifier.
    #[must_use]
    pub fn asset_id(&self) -> &str {
        self.entry.id()
    }

    /// The owned source bytes, exactly as transferred.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The SHA-256 of these exact bytes, which equals the catalogue's declared digest.
    #[must_use]
    pub fn source_sha256(&self) -> String {
        sha256_media_worker_bytes_v1(&self.bytes).to_hex()
    }

    /// The physical identity observed on the handle those bytes came from.
    #[must_use]
    pub const fn identity(&self) -> PhysicalIdentityV1 {
        self.identity
    }

    /// The exact length the same handle reported.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// Non-cryptographic fingerprint of exactly these bytes.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }
}

/// The complete, ordered source evidence of one Platform library acquisition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformAssetSourceSetV1 {
    version: &'static str,
    evidence: Vec<PlatformAssetSourceEvidenceV1>,
}

impl PlatformAssetSourceSetV1 {
    /// The selected library version, which is the approved one.
    #[must_use]
    pub const fn version(&self) -> &'static str {
        self.version
    }

    /// Every requested asset's evidence, in the request's own order.
    #[must_use]
    pub fn evidence(&self) -> &[PlatformAssetSourceEvidenceV1] {
        &self.evidence
    }

    /// How many assets were acquired.
    #[must_use]
    pub fn len(&self) -> usize {
        self.evidence.len()
    }

    /// Whether no asset was acquired. Never true for a complete set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }

    /// The evidence for one exact identifier, never a near match.
    #[must_use]
    pub fn exact(&self, asset_id: &str) -> Option<&PlatformAssetSourceEvidenceV1> {
        self.evidence
            .iter()
            .find(|evidence| evidence.asset_id() == asset_id)
    }

    /// The namespace these sources were read from. Always the Platform namespace.
    #[must_use]
    pub const fn source_namespace(&self) -> AssetNamespace {
        AssetNamespace::Platform
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        PLATFORM_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY_V1
    }

    /// Whether this set proves the media itself. Always `false`.
    #[must_use]
    pub const fn proves_media_correctness(&self) -> bool {
        PLATFORM_ASSET_READ_PROVES_MEDIA_V1
    }

    /// Whether this set proves a global multi-file snapshot. Always `false`.
    #[must_use]
    pub const fn proves_global_snapshot(&self) -> bool {
        PLATFORM_ASSET_READ_PROVES_GLOBAL_SNAPSHOT_V1
    }
}

/// The outcome of one whole Platform library acquisition.
///
/// There is no third case: the request either produced every requested asset's evidence or one
/// refusal. Partial evidence is discarded before a refusal is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformLibraryReadOutcomeV1 {
    /// Every requested asset was acquired.
    Complete(PlatformAssetSourceSetV1),
    /// One asset, or the library boundary, refused the whole request.
    Refused(PlatformAssetReadRefusalV1),
}

impl PlatformLibraryReadOutcomeV1 {
    /// The complete set, if the request succeeded.
    #[must_use]
    pub const fn complete(&self) -> Option<&PlatformAssetSourceSetV1> {
        match self {
            Self::Complete(set) => Some(set),
            Self::Refused(_) => None,
        }
    }

    /// The one refusal, if the request did not succeed.
    #[must_use]
    pub const fn refusal(&self) -> Option<&PlatformAssetReadRefusalV1> {
        match self {
            Self::Complete(_) => None,
            Self::Refused(refusal) => Some(refusal),
        }
    }
}

/// Acquires the exact approved source bytes of **every** requested Platform asset.
///
/// The inputs are the caller's independently selected version, its explicitly adopted Platform
/// root token, the catalogue model the reader parsed and the typed requests. Nothing is
/// discovered: no directory is scanned, no Entity root is consulted, no route, locale, role or
/// filename selects an asset, and no fallback exists.
///
/// The catalogue is trusted only through [`APPROVED_PLATFORM_CATALOGUE_SHA256_V1`], computed here
/// from the bytes the reader transferred, not from anything the reader or the model claims. Each
/// source is then hashed here and compared with the catalogue's declared digest for its exact
/// identifier **before** any media inspection can be attempted by the caller.
///
/// A duplicate request, an unknown identifier, an `Entity` reference, an inconsistent catalogue
/// model, a wrong digest, an ambiguous identity or a late failure refuses the whole request, and
/// every earlier record is discarded. The returned set is therefore all-or-nothing.
pub fn read_pinned_platform_sources_v1<P: PlatformAssetLibraryReadPortV1 + ?Sized>(
    selection: PlatformRootSelectionV1,
    selected_version: &str,
    catalogue: &PlatformAssetCatalogueV1,
    requests: &[PlatformAssetRefV1],
    port: &P,
) -> PlatformLibraryReadOutcomeV1 {
    use PlatformAssetReadRefusalKindV1 as Kind;

    // The capability token is consumed here: a caller that never adopted one cannot reach a read.
    let _adopted = selection.provenance();

    if selected_version != APPROVED_PLATFORM_LIBRARY_VERSION_V1 {
        return refused(Kind::UnselectedVersion);
    }
    if requests.is_empty() {
        return refused(Kind::EmptyRequestedSet);
    }
    let mut requested: BTreeMap<&str, usize> = BTreeMap::new();
    for (position, request) in requests.iter().enumerate() {
        if requested.insert(request.as_str(), position).is_some() {
            return refused_at(Kind::DuplicateRequestedAsset, position);
        }
    }

    let (catalogue_bytes, _catalogue_identity) =
        match port.read_catalogue(MAX_PLATFORM_CATALOGUE_BYTES_V1) {
            PlatformCatalogueSourceV1::Present {
                bytes,
                identity,
                length,
                links,
                fingerprint: _,
            } => {
                let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                if transferred != length {
                    return refused(Kind::ChangedDuringRead);
                }
                if length > MAX_PLATFORM_CATALOGUE_BYTES_V1 {
                    return refused(Kind::CatalogueOversize);
                }
                if links != 1 {
                    return refused(Kind::IdentityCollision);
                }
                (bytes, identity)
            }
            PlatformCatalogueSourceV1::Refused(kind) => return refused(kind),
        };

    // The pin is computed from the transferred bytes, here, independently of the model.
    if sha256_media_worker_bytes_v1(&catalogue_bytes).to_hex()
        != APPROVED_PLATFORM_CATALOGUE_SHA256_V1
    {
        return refused(Kind::CatalogueUnapproved);
    }
    if catalogue.len() != APPROVED_PLATFORM_ASSET_COUNT_V1 {
        return refused(Kind::CatalogueCountMismatch);
    }
    if catalogue.version() != APPROVED_PLATFORM_LIBRARY_VERSION_V1 {
        return refused(Kind::CatalogueModelMismatch);
    }
    read_requested_sources(catalogue, &catalogue_bytes, requests, port)
}

/// Reads every requested source against the already-approved catalogue.
fn read_requested_sources<P: PlatformAssetLibraryReadPortV1 + ?Sized>(
    catalogue: &PlatformAssetCatalogueV1,
    catalogue_bytes: &[u8],
    requests: &[PlatformAssetRefV1],
    port: &P,
) -> PlatformLibraryReadOutcomeV1 {
    use PlatformAssetReadRefusalKindV1 as Kind;

    let mut evidence: Vec<PlatformAssetSourceEvidenceV1> = Vec::new();
    let mut seen_identities: BTreeMap<(u64, u64, u64), &str> = BTreeMap::new();

    for (position, request) in requests.iter().enumerate() {
        let entry = match catalogue.exact_entry(request.as_str()) {
            Some(entry) => entry,
            None => return refused_at(Kind::UnapprovedAssetIdentifier, position),
        };
        if entry.namespace() != AssetNamespace::Platform {
            return refused_at(Kind::CatalogueModelMismatch, position);
        }
        // The typed model must describe exactly the asset the approved bytes declare.
        if !approved_catalogue_declares(catalogue_bytes, entry) {
            return refused_at(Kind::CatalogueModelMismatch, position);
        }

        let (bytes, identity, length, links, fingerprint) =
            match port.read_platform_source(request.as_str(), MAX_PLATFORM_SOURCE_BYTES_V1) {
                PlatformAssetSourceV1::Present {
                    bytes,
                    identity,
                    length,
                    links,
                    fingerprint,
                } => (bytes, identity, length, links, fingerprint),
                PlatformAssetSourceV1::Refused(kind) => return refused_at(kind, position),
            };

        let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if length > MAX_PLATFORM_SOURCE_BYTES_V1 || transferred > MAX_PLATFORM_SOURCE_BYTES_V1 {
            return refused_at(Kind::OversizeSource, position);
        }
        if transferred != length {
            return refused_at(Kind::ChangedDuringRead, position);
        }
        if links != 1 {
            return refused_at(Kind::IdentityCollision, position);
        }
        // The source digest is computed here, from the actual bytes, before anything inspects them.
        if sha256_media_worker_bytes_v1(&bytes).to_hex() != entry.sha256().to_hex() {
            return refused_at(Kind::SourceDigestMismatch, position);
        }
        let key = (
            identity.container(),
            identity.object_low(),
            identity.object_high(),
        );
        match seen_identities.get(&key) {
            Some(existing) if *existing != request.as_str() => {
                return refused_at(Kind::IdentityCollision, position);
            }
            Some(_) => return refused_at(Kind::DuplicateRequestedAsset, position),
            None => {
                seen_identities.insert(key, request.as_str());
            }
        }

        evidence.push(PlatformAssetSourceEvidenceV1 {
            entry: entry.clone(),
            bytes,
            identity,
            length,
            fingerprint,
        });
    }

    PlatformLibraryReadOutcomeV1::Complete(PlatformAssetSourceSetV1 {
        version: APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        evidence,
    })
}

/// A refusal taken before any single asset was examined.
const fn refused(kind: PlatformAssetReadRefusalKindV1) -> PlatformLibraryReadOutcomeV1 {
    PlatformLibraryReadOutcomeV1::Refused(PlatformAssetReadRefusalV1::of(kind))
}

/// A refusal attributed to one requested ordinal.
const fn refused_at(
    kind: PlatformAssetReadRefusalKindV1,
    position: usize,
) -> PlatformLibraryReadOutcomeV1 {
    PlatformLibraryReadOutcomeV1::Refused(PlatformAssetReadRefusalV1::at(kind, position))
}

/// Whether the approved catalogue bytes declare exactly this entry's identifier and digest.
///
/// The approved byte set is fixed, so this is a deterministic consistency check, not a heuristic:
/// a model that names an identifier or digest the approved document does not declare cannot pass
/// it, and the document's own formatting is pinned by its hash.
fn approved_catalogue_declares(bytes: &[u8], entry: &PlatformAssetCatalogueEntryV1) -> bool {
    let identifier = format!("\"id\": \"{}\"", entry.id());
    let digest = entry.sha256().to_hex();
    contains(bytes, identifier.as_bytes()) && contains(bytes, digest.as_bytes())
}

/// Whether `haystack` contains `needle` as a contiguous byte sequence.
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}
