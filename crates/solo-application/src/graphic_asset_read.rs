//! Confined Entity graphic source-byte acquisition (`R03.C2`).
//!
//! This module is the pure application half of one narrow, nonpublishing gate: for every
//! graphic source the author actually declared (`R03.C1`), acquire that Entity's own source
//! bytes beneath the confined `assets/` namespace of the explicitly adopted content root, or
//! refuse the whole candidate.
//!
//! **What a success proves.** One [`GraphicAssetSourceSetV1`] holds, for every independently
//! authored obligation, the exact logical path, the owned bytes, the physical identity observed
//! on the handle those bytes came from and their exact length. It proves nothing else. In
//! particular a success never asserts that the bytes are an image, that they match their
//! extension, MIME or magic, that they decode, that their dimensions or pixel count are bounded,
//! that they are licensed, that a platform catalogue entry exists, that a derivative was built
//! or that publication is authorized. [`GRAPHIC_ASSET_READ_VALIDATES_MEDIA`] and
//! [`GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY`] are the machine-checkable statement of
//! that boundary.
//!
//! **What this module never does.** It opens no file, imports no filesystem, network, decoder,
//! catalogue or adapter type, scans no directory, infers no root, infers no media type and
//! offers no fallback. Every observation comes from the inward port the caller supplies, and
//! every policy decision below is driven by the complete authored inventory rather than by the
//! requested target, a public projection or a set of distinct filenames.
//!
//! **Whole-candidate discipline.** The result is either complete or refused. A single failing
//! obligation — public, internal, unlisted or descendant-suppressed — discards every partial
//! success, so a caller can never mistake a partly-read inventory for a verified one, and no
//! refusal is ever reported as absence or as an empty set.

use std::collections::BTreeMap;
use std::fmt;

use solo_domain::{
    ACCEPTED_IMAGE_EXTENSIONS, AssetNamespace, AssetReferenceV1, PropertyKey, QualifiedOwnerV1,
};

use crate::graphic_asset_inventory::{GraphicAssetObligationInventoryV1, GraphicAssetObligationV1};
use crate::source_read::PhysicalIdentityV1;

/// The maximum number of source bytes one Entity graphic may contribute.
///
/// Applies to every admitted extension and to every exposure. Exactly this many bytes is
/// allowed; one byte more is refused rather than truncated. A zero-byte file is faithfully
/// acquirable and is emphatically *not* thereby valid image media.
pub const MAX_GRAPHIC_SOURCE_BYTES: u64 = 20 * 1024 * 1024;

/// Whether acquiring source bytes authorizes publication of any kind.
///
/// Always `false`. A successful acquisition is evidence about bytes and physical identity, not
/// a publication decision, a route, a visibility choice or a rendered value.
pub const GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether acquiring source bytes proves anything about the media itself.
///
/// Always `false`. Extension, MIME, magic, decoding, dimensions, pixel bounds, licence and
/// catalogue membership are later, separately authorized gates that must refuse malformed
/// media. This gate deliberately performs none of them.
pub const GRAPHIC_ASSET_READ_VALIDATES_MEDIA: bool = false;

/// The one namespace segment an Entity graphic reference is resolved beneath.
pub const ENTITY_ASSET_NAMESPACE_SEGMENT: &str = "assets";

/// The ceiling on the complete logical asset path, including the `assets/` segment.
const MAX_ASSET_PATH_BYTES: usize = 240;

/// The ceiling on logical segments, counted inclusively of the `assets/` segment.
const MAX_ASSET_PATH_SEGMENTS: usize = 8;

/// The ceiling on one portable logical component, in bytes.
const MAX_ASSET_COMPONENT_BYTES: usize = 80;

/// Windows device basenames that stay reserved at any extension (Architecture §6.2C).
///
/// A name is reserved on its stem, so `con.png` is refused exactly like `con`. They are listed
/// in the one casing the portable grammar admits, because any other casing is already refused.
const RESERVED_WINDOWS_STEMS: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// Why one authored graphic obligation produced no trustworthy source-byte evidence.
///
/// Every variant refuses the whole candidate. None of them is absence-as-success, and none may
/// be downgraded to an empty or partial result by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphicAssetReadRefusalKind {
    /// The inventory declares at least one `platform` reference, which this gate cannot
    /// verify: no platform root, catalogue or artwork is opened, inferred or generated.
    PlatformUnverified,
    /// The reference is not a portable lowercase Entity asset path, so it is refused exactly
    /// as authored rather than normalized, trimmed, guessed or repaired.
    UnsafeAssetReference,
    /// The declared source positively does not exist. Never a benign result for a declared
    /// asset: a missing declared graphic fails the whole candidate.
    MissingSource,
    /// The location or object is unsafe: a symlink, junction, reparse point, non-regular
    /// object, cross-device escape, unsafe root or wrong root.
    UnsafeSourceLocation,
    /// Access to some component or to the leaf was refused.
    PermissionDenied,
    /// The object changed identity, length, link count or timestamps while it was read.
    ChangedDuringRead,
    /// Link count other than exactly one, two distinct logical asset paths that resolve to one
    /// physical object, or an inconsistent repeat observation of one logical reference.
    IdentityCollision,
    /// The source exceeds [`MAX_GRAPHIC_SOURCE_BYTES`] and is refused, never truncated.
    OversizeSource,
    /// The host cannot provide the required confinement, identity or immutable-transfer
    /// guarantee, so nothing is reported.
    Unsupported,
    /// Some other operating-system fault prevented a trustworthy result.
    OtherIo,
}

impl GraphicAssetReadRefusalKind {
    /// Stable, safe diagnostic token. It carries no path, byte or handle.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::PlatformUnverified => "ASSET_PLATFORM_UNVERIFIED",
            Self::UnsafeAssetReference => "ASSET_REFERENCE_UNSAFE",
            Self::MissingSource => "ASSET_SOURCE_MISSING",
            Self::UnsafeSourceLocation => "ASSET_SOURCE_UNSAFE_LOCATION",
            Self::PermissionDenied => "ASSET_SOURCE_PERMISSION_DENIED",
            Self::ChangedDuringRead => "ASSET_SOURCE_CHANGED",
            Self::IdentityCollision => "ASSET_SOURCE_IDENTITY_COLLISION",
            Self::OversizeSource => "ASSET_SOURCE_OVERSIZE",
            Self::Unsupported => "ASSET_SOURCE_UNSUPPORTED",
            Self::OtherIo => "ASSET_SOURCE_IO_ERROR",
        }
    }

    /// Fixed, bounded explanation carrying no authored value.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::PlatformUnverified => {
                "a platform-namespaced graphic cannot be verified by this gate"
            }
            Self::UnsafeAssetReference => {
                "the reference is not a portable lowercase Entity asset path"
            }
            Self::MissingSource => "the declared Entity source does not exist",
            Self::UnsafeSourceLocation => "the location or object is not a confined regular file",
            Self::PermissionDenied => "access to the source was refused",
            Self::ChangedDuringRead => "the source changed while it was being acquired",
            Self::IdentityCollision => {
                "the physical identity of the source is ambiguous for this inventory"
            }
            Self::OversizeSource => "the source exceeds the documented byte bound",
            Self::Unsupported => "the host cannot provide the required guarantee",
            Self::OtherIo => "an operating-system fault prevented a trustworthy result",
        }
    }
}

impl fmt::Display for GraphicAssetReadRefusalKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

/// One typed refusal, naming the declaring obligation it concerns and nothing more.
///
/// The coordinates are the ones the author actually wrote: the declaring owner, the registered
/// role and the zero-based local index. No path, byte, handle or authored value is carried, so a
/// diagnostic can be rendered without leaking a source location or a private declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicAssetReadRefusalV1 {
    kind: GraphicAssetReadRefusalKind,
    declaring_owner: Option<Box<QualifiedOwnerV1>>,
    role: Option<PropertyKey>,
    local_index: Option<usize>,
}

impl GraphicAssetReadRefusalV1 {
    /// A refusal that concerns no single obligation, for a boundary decision taken before any
    /// obligation was examined.
    #[must_use]
    pub const fn of(kind: GraphicAssetReadRefusalKind) -> Self {
        Self {
            kind,
            declaring_owner: None,
            role: None,
            local_index: None,
        }
    }

    /// The same refusal, attributed to the obligation it concerns.
    #[must_use]
    pub fn concerning(
        kind: GraphicAssetReadRefusalKind,
        obligation: &GraphicAssetObligationV1,
    ) -> Self {
        Self {
            kind,
            declaring_owner: Some(Box::new(obligation.declaring_owner().clone())),
            role: Some(obligation.role()),
            local_index: Some(obligation.local_index()),
        }
    }

    /// The stable category.
    #[must_use]
    pub const fn kind(&self) -> GraphicAssetReadRefusalKind {
        self.kind
    }

    /// The stable diagnostic token.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// The fixed, bounded explanation.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }

    /// The owner that authored the affected obligation, when one was identified.
    #[must_use]
    pub fn declaring_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.declaring_owner.as_deref()
    }

    /// The registered role of the affected obligation, when one was identified.
    #[must_use]
    pub const fn role(&self) -> Option<PropertyKey> {
        self.role
    }

    /// The authored local index of the affected obligation, when one was identified.
    #[must_use]
    pub const fn local_index(&self) -> Option<usize> {
        self.local_index
    }
}

impl fmt::Display for GraphicAssetReadRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for GraphicAssetReadRefusalV1 {}

/// One validated, confined logical Entity asset path.
///
/// The value is `assets/` followed by the authored reference's own segments, unchanged. It is
/// relative to the caller's already adopted content root, so `brand/logo.png` can only ever name
/// `<content>/assets/brand/logo.png`. Nothing here resolves a path, opens a file or proves that
/// the leaf exists.
///
/// Construction is the defense-in-depth boundary the adapter re-runs immediately before a native
/// open. The domain reference constructor already proves a typed namespace, an admitted image
/// extension and much of the portable grammar, but it deliberately admits spellings the portable
/// filename rule forbids — uppercase stems, reserved Windows device stems, over-long components,
/// over-deep paths and adjacent dots. Those are refused here, exactly as authored: no value is
/// lowercased, trimmed, shortened, de-duplicated or repaired, and the domain parser and the
/// accepted Page locator grammar are untouched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAssetPathV1 {
    logical_path: String,
    components: Vec<String>,
}

impl EntityAssetPathV1 {
    /// Validates one Entity asset reference into its confined logical path.
    ///
    /// # Errors
    ///
    /// Returns [`GraphicAssetReadRefusalKind::PlatformUnverified`] for a `platform` reference,
    /// which this gate can never verify, and
    /// [`GraphicAssetReadRefusalKind::UnsafeAssetReference`] for any spelling outside the
    /// portable lowercase Entity asset grammar.
    pub fn from_reference(reference: &AssetReferenceV1) -> Result<Self, GraphicAssetReadRefusalV1> {
        if reference.namespace() != AssetNamespace::Entity {
            return Err(GraphicAssetReadRefusalV1::of(
                GraphicAssetReadRefusalKind::PlatformUnverified,
            ));
        }
        let authored = reference.reference();
        let refuse =
            || GraphicAssetReadRefusalV1::of(GraphicAssetReadRefusalKind::UnsafeAssetReference);
        if authored.is_empty() || !authored.is_ascii() {
            return Err(refuse());
        }
        let mut components: Vec<String> = Vec::new();
        components.push(ENTITY_ASSET_NAMESPACE_SEGMENT.to_owned());
        let mut last = "";
        for segment in authored.split('/') {
            if components.len() >= MAX_ASSET_PATH_SEGMENTS || !portable_component(segment) {
                return Err(refuse());
            }
            components.push(segment.to_owned());
            last = segment;
        }
        let logical_path = format!("{ENTITY_ASSET_NAMESPACE_SEGMENT}/{authored}");
        if logical_path.len() > MAX_ASSET_PATH_BYTES || !accepted_image_extension(last) {
            return Err(refuse());
        }
        Ok(Self {
            logical_path,
            components,
        })
    }

    /// The complete logical path, `assets/` namespace segment included.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The logical segments, the first of which is always the `assets/` namespace segment.
    #[must_use]
    pub fn components(&self) -> &[String] {
        &self.components
    }

    /// The same segments as borrowed slices, for a native boundary that takes single names.
    #[must_use]
    pub fn component_slices(&self) -> Vec<&str> {
        self.components
            .iter()
            .map(std::string::String::as_str)
            .collect()
    }

    /// The length of the complete logical path in bytes.
    #[must_use]
    pub fn byte_length(&self) -> usize {
        self.logical_path.len()
    }
}

/// Whether one authored segment is a portable lowercase filename component.
fn portable_component(segment: &str) -> bool {
    if segment.is_empty()
        || segment.len() > MAX_ASSET_COMPONENT_BYTES
        || segment == "."
        || segment == ".."
    {
        return false;
    }
    let bytes = segment.as_bytes();
    if !bytes[0].is_ascii_lowercase() {
        return false;
    }
    for byte in bytes {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || matches!(byte, b'.' | b'_' | b'-'))
        {
            return false;
        }
    }
    if segment.ends_with('.') || segment.ends_with(' ') || segment.contains("..") {
        return false;
    }
    let stem = segment.split('.').next().unwrap_or(segment);
    !RESERVED_WINDOWS_STEMS.contains(&stem)
}

/// Whether the segment carries exactly one admitted, lowercase image extension.
fn accepted_image_extension(segment: &str) -> bool {
    match segment.rsplit_once('.') {
        Some((_, extension)) => ACCEPTED_IMAGE_EXTENSIONS.contains(&extension),
        None => false,
    }
}

/// What one outbound read of one Entity asset reference observed.
///
/// `Present` carries the adapter's own observations and the owned bytes. The application
/// re-checks every invariant it can from these values — the bound, the exact transferred length,
/// the link count and the identity consistency across obligations — so a port that under-reports
/// cannot turn an unsafe object into accepted evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityAssetSourceV1 {
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
        /// Modification and change timestamps the handle reported.
        changed_nanos: (i64, i64),
        /// Non-cryptographic fingerprint of exactly these bytes.
        fingerprint: u64,
    },
    /// The source was not produced, for the stated reason.
    Refused(GraphicAssetReadRefusalV1),
}

/// The minimal inward port one Entity asset read needs.
///
/// It takes an already typed reference and the caller's bound, and it answers with observations
/// or a typed refusal. It is deliberately narrower than a filesystem API: it cannot be asked to
/// list, scan, glob, guess an extension, resolve a platform asset or read a Page source, and it
/// returns no handle a caller could keep using.
pub trait GraphicAssetReadPort {
    /// Reads one already authored Entity asset reference, within `max_bytes`.
    fn read_entity_asset(
        &self,
        reference: &AssetReferenceV1,
        max_bytes: u64,
    ) -> EntityAssetSourceV1;
}

/// Source-byte evidence for exactly one authored obligation.
///
/// Holding one proves that these bytes are the ones one confined read transferred from the
/// object whose identity is recorded, for the obligation named here. It proves nothing about
/// media validity, licensing, catalogue membership or publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicAssetSourceEvidenceV1 {
    obligation: GraphicAssetObligationV1,
    logical_path: String,
    bytes: Vec<u8>,
    identity: PhysicalIdentityV1,
    length: u64,
    fingerprint: u64,
}

impl GraphicAssetSourceEvidenceV1 {
    /// The authored obligation this evidence answers, unchanged.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The confined logical path the bytes were read from.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The owned source bytes, exactly as transferred.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The physical identity of the object the bytes came from.
    #[must_use]
    pub const fn identity(&self) -> PhysicalIdentityV1 {
        self.identity
    }

    /// The exact transferred byte count.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// A non-cryptographic fingerprint of exactly these bytes.
    ///
    /// It is a linkage aid, never a verified cryptographic hash, never content provenance, never
    /// media MIME, never a safe decode, never a rights statement and never publication authority.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    /// Whether this evidence authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this evidence proves the media itself. Always `false`.
    #[must_use]
    pub const fn media_verified(&self) -> bool {
        GRAPHIC_ASSET_READ_VALIDATES_MEDIA
    }
}

/// Every obligation's source-byte evidence, in inventory order.
///
/// A set is only ever built by a complete traversal, so holding one means every eligible
/// obligation of the inventory it came from succeeded. It publishes nothing and validates no
/// media.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicAssetSourceSetV1 {
    evidence: Vec<GraphicAssetSourceEvidenceV1>,
}

impl GraphicAssetSourceSetV1 {
    /// Every piece of evidence, in inventory order.
    #[must_use]
    pub fn evidence(&self) -> &[GraphicAssetSourceEvidenceV1] {
        &self.evidence
    }

    /// How many obligations were acquired.
    #[must_use]
    pub fn len(&self) -> usize {
        self.evidence.len()
    }

    /// Whether the inventory authored no graphic source at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this set proves the media itself. Always `false`.
    #[must_use]
    pub const fn validates_media(&self) -> bool {
        GRAPHIC_ASSET_READ_VALIDATES_MEDIA
    }
}

/// The outcome of one whole-inventory acquisition attempt.
///
/// There is no third case: an inventory either produced every eligible obligation's evidence or
/// produced one refusal. Partial evidence is discarded before a refusal is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicAssetReadSetOutcomeV1 {
    /// Every eligible obligation was acquired.
    Complete(GraphicAssetSourceSetV1),
    /// One obligation, or the platform boundary, refused the whole attempt.
    Refused(GraphicAssetReadRefusalV1),
}

impl GraphicAssetReadSetOutcomeV1 {
    /// The complete set, if the attempt succeeded.
    #[must_use]
    pub const fn complete(&self) -> Option<&GraphicAssetSourceSetV1> {
        match self {
            Self::Complete(set) => Some(set),
            Self::Refused(_) => None,
        }
    }

    /// The one refusal, if the attempt did not succeed.
    #[must_use]
    pub const fn refusal(&self) -> Option<&GraphicAssetReadRefusalV1> {
        match self {
            Self::Complete(_) => None,
            Self::Refused(refusal) => Some(refusal),
        }
    }
}

/// Acquires confined source bytes for **every** authored Entity graphic obligation.
///
/// The inventory is the sole input, so the decision set is independent of the requested target,
/// the public projection and the number of distinct filenames. Ordering is the inventory's own
/// deterministic order, and every obligation — `public`, `internal`, unlisted and
/// descendant-suppressed alike — is acquired.
///
/// Platform-namespaced obligations are refused first, before any outbound call, because this gate
/// cannot verify them; an inventory that mixes namespaces is therefore never reported as a
/// complete Entity-only success.
///
/// A logical reference authored more than once is read once per authored obligation, and every
/// repeat observation must agree with the first in physical identity, reported length and the
/// **exact acquired byte sequence**. A non-cryptographic fingerprint is never accepted as a
/// substitute for that comparison, so two different bodies of equal length can never be reported
/// as one consistent source, however their fingerprints were produced.
pub fn read_graphic_asset_bytes_v1<P: GraphicAssetReadPort + ?Sized>(
    inventory: &GraphicAssetObligationInventoryV1,
    port: &P,
) -> GraphicAssetReadSetOutcomeV1 {
    for obligation in inventory.obligations() {
        if obligation.namespace() == AssetNamespace::Platform {
            return GraphicAssetReadSetOutcomeV1::Refused(GraphicAssetReadRefusalV1::concerning(
                GraphicAssetReadRefusalKind::PlatformUnverified,
                obligation,
            ));
        }
    }

    let mut evidence: Vec<GraphicAssetSourceEvidenceV1> = Vec::new();
    // Index of the first evidence record for each logical path. The retained record owns its
    // exact bytes, so a repeated reference is compared against those bytes directly instead of
    // against a second copy of them or against a digest of them.
    let mut seen_paths: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_identities: BTreeMap<(u64, u64, u64), String> = BTreeMap::new();

    for obligation in inventory.obligations() {
        let path = match EntityAssetPathV1::from_reference(obligation.reference()) {
            Ok(path) => path,
            Err(refusal) => {
                return GraphicAssetReadSetOutcomeV1::Refused(
                    GraphicAssetReadRefusalV1::concerning(refusal.kind(), obligation),
                );
            }
        };
        let (bytes, identity, length, links, fingerprint) =
            match port.read_entity_asset(obligation.reference(), MAX_GRAPHIC_SOURCE_BYTES) {
                EntityAssetSourceV1::Present {
                    bytes,
                    identity,
                    length,
                    links,
                    fingerprint,
                    changed_nanos: _,
                } => (bytes, identity, length, links, fingerprint),
                EntityAssetSourceV1::Refused(refusal) => {
                    return GraphicAssetReadSetOutcomeV1::Refused(
                        GraphicAssetReadRefusalV1::concerning(refusal.kind(), obligation),
                    );
                }
            };

        let transferred = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        let failure = if length > MAX_GRAPHIC_SOURCE_BYTES || transferred > MAX_GRAPHIC_SOURCE_BYTES
        {
            Some(GraphicAssetReadRefusalKind::OversizeSource)
        } else if transferred != length {
            Some(GraphicAssetReadRefusalKind::ChangedDuringRead)
        } else if links != 1 {
            Some(GraphicAssetReadRefusalKind::IdentityCollision)
        } else {
            let key = identity_key(identity);
            match seen_paths.get(path.logical_path()) {
                Some(first) => {
                    let first = &evidence[*first];
                    if first.identity() != identity {
                        Some(GraphicAssetReadRefusalKind::IdentityCollision)
                    } else if first.length() != length
                        || first.fingerprint() != fingerprint
                        || first.bytes() != bytes.as_slice()
                    {
                        // A non-cryptographic fingerprint proves nothing about content: the
                        // exact bytes must agree, so equal-length different bodies are a change
                        // even when the supplied fingerprints happen to match.
                        Some(GraphicAssetReadRefusalKind::ChangedDuringRead)
                    } else {
                        None
                    }
                }
                None => match seen_identities.get(&key) {
                    Some(existing) if existing != path.logical_path() => {
                        Some(GraphicAssetReadRefusalKind::IdentityCollision)
                    }
                    Some(_) | None => None,
                },
            }
        };
        if let Some(kind) = failure {
            return GraphicAssetReadSetOutcomeV1::Refused(GraphicAssetReadRefusalV1::concerning(
                kind, obligation,
            ));
        }

        seen_paths.insert(path.logical_path().to_owned(), evidence.len());
        seen_identities.insert(identity_key(identity), path.logical_path().to_owned());
        evidence.push(GraphicAssetSourceEvidenceV1 {
            obligation: obligation.clone(),
            logical_path: path.logical_path().to_owned(),
            bytes,
            identity,
            length,
            fingerprint,
        });
    }

    GraphicAssetReadSetOutcomeV1::Complete(GraphicAssetSourceSetV1 { evidence })
}

/// The comparable form of a physical identity, for within-attempt consistency checks.
fn identity_key(identity: PhysicalIdentityV1) -> (u64, u64, u64) {
    (
        identity.container(),
        identity.object_low(),
        identity.object_high(),
    )
}
