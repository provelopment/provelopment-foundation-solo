//! Whole-inventory Entity image inspection (`R03.C3`, PNG/JPEG/WebP only).
//!
//! This module is the pure application half of one narrow, still-nonpublishing gate. It consumes
//! the **complete** accepted `R03.C2` source set — every authored obligation, public, internal,
//! unlisted and descendant-suppressed alike — and asks one replaceable outbound port to decode
//! each already-confined source in a separately confined worker. It either believes every
//! obligation or refuses the whole candidate.
//!
//! **What a success proves.** One [`GraphicImageInspectionSetV1`] holds, per authored obligation,
//! the unchanged owner and reference, the exact SHA-256 of the owned source bytes the parent
//! hashed itself, the canonical MIME and confirmed still-image format, the decoded dimensions and
//! pixel count, the decoded byte count and the digest of exactly those decoded bytes. It proves
//! that those bytes decoded completely under the fixed bounds. It does **not** prove colour,
//! orientation, ancillary-chunk or trailing-data conformance, licence, catalogue membership,
//! derivative output, a route, a browser surface or publication authority.
//!
//! **What this module never does.** It opens no file, starts no process, imports no codec, no
//! operating-system type, no filesystem, no network and no adapter type. It infers no media type
//! from untrusted metadata: the declared format comes only from the authored Entity reference's
//! strictly lowercase extension. `.avif`, a platform reference and any other extension are a typed
//! refusal for this slice, never a silent skip, because AVIF decoding is not implemented here and
//! an unimplemented format must never look like an absent obligation.
//!
//! **Whole-candidate discipline.** A single failing obligation — empty, oversized, unsupported,
//! mismatched, out-of-range, unverified, truncated or wrongly digested — discards every partial
//! success and returns exactly one typed refusal. A refusal is never reported as absence and never
//! yields an empty set.

use solo_domain::{AssetNamespace, AssetReferenceV1, PropertyKey, QualifiedOwnerV1};

use crate::graphic_asset_inventory::{
    GraphicAssetObligationV1, MixedGraphicObligationV1, MixedGraphicPlanRefusalV1,
    MixedGraphicPlanV1,
};
use crate::graphic_asset_read::{
    GraphicAssetReadPort, GraphicAssetReadRefusalV1, GraphicAssetReadSetOutcomeV1,
    GraphicAssetSourceSetV1, MAX_GRAPHIC_SOURCE_BYTES, read_graphic_asset_bytes_v1,
};
use crate::media_worker::{
    MediaWorkerCheckedEnvelopeV1, MediaWorkerDigestV1, MediaWorkerOrientationV1, MediaWorkerPortV1,
    MediaWorkerRefusalKindV1, MediaWorkerRequestV1, MediaWorkerRunV1, check_observation_v1,
    sha256_media_worker_bytes_v1,
};
use crate::platform_asset_catalogue::PlatformAssetCatalogueV1;
use crate::platform_asset_read::{
    PlatformAssetLibraryReadPortV1, PlatformAssetReadRefusalV1, PlatformAssetRefV1,
    PlatformAssetSourceSetV1, PlatformLibraryReadOutcomeV1, PlatformRootSelectionV1,
    read_pinned_platform_sources_v1,
};
use crate::platform_graphic_baseline::{
    PlatformGraphicBaselineRefusalV1, approved_platform_graphic_binding_v1,
};

/// Whether a complete inspection authorizes publication of any kind.
///
/// Always `false`. A contained, correctly decoded still image is an inert validation result, not a
/// publishable Page or asset, not a derived output and not a route.
pub const GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether a complete inspection proves the media is *correct*.
///
/// Always `false`. Decode success and bounded geometry say nothing about colour management,
/// orientation, ancillary-chunk conformance, trailing data, licensing or platform-catalogue
/// membership; each remains a separate, separately authorized gate.
pub const GRAPHIC_IMAGE_INSPECTION_PROVES_MEDIA_CORRECTNESS: bool = false;

/// The largest accepted side length, in pixels, of any admitted still image.
pub const MAX_IMAGE_SIDE_V1: u32 = 8192;

/// The largest accepted `width × height` product, in pixels.
pub const MAX_IMAGE_PIXELS_V1: u64 = 32_000_000;

/// The largest decoded payload the worker protocol may transfer, in bytes.
///
/// A 32 000 000-pixel RGBA8 plane is 128 000 000 bytes, so this ceiling bounds the protocol well
/// above every admissible image while still refusing an unbounded worker.
pub const MAX_IMAGE_DECODED_BYTES_V1: u64 = 256 * 1024 * 1024;

/// The whole wall-time budget, in milliseconds, of one worker attempt.
pub const IMAGE_WORKER_WALL_MS_V1: u64 = 15_000;

/// The largest accepted source, in bytes, for one declared image.
///
/// The same number the accepted `R03.C2` acquisition already enforces, so the two gates cannot
/// drift apart.
pub const MAX_IMAGE_SOURCE_BYTES_V1: u64 = MAX_GRAPHIC_SOURCE_BYTES;

/// The canonical MIME of a confirmed PNG still image.
pub const IMAGE_MIME_PNG_V1: &str = "image/png";

/// The canonical MIME of a confirmed JPEG still image.
pub const IMAGE_MIME_JPEG_V1: &str = "image/jpeg";

/// The canonical MIME of a confirmed WebP still image.
pub const IMAGE_MIME_WEBP_V1: &str = "image/webp";

/// The three still-image formats this slice may inspect.
///
/// Closed and exhaustive: there is no fourth variant, no `Other` and no `Avif`, so an AVIF source
/// cannot be represented as an approved request at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphicImageFormatV1 {
    /// Portable Network Graphics.
    Png,
    /// JPEG (baseline, extended or progressive).
    Jpeg,
    /// WebP (lossy or lossless still).
    WebP,
}

impl GraphicImageFormatV1 {
    /// Every admitted format, in a fixed order.
    pub const ALL: [Self; 3] = [Self::Png, Self::Jpeg, Self::WebP];

    /// The canonical MIME type of this format, fixed here rather than taken from the worker.
    #[must_use]
    pub const fn canonical_mime(self) -> &'static str {
        match self {
            Self::Png => IMAGE_MIME_PNG_V1,
            Self::Jpeg => IMAGE_MIME_JPEG_V1,
            Self::WebP => IMAGE_MIME_WEBP_V1,
        }
    }

    /// The exact lowercase extension tokens this format accepts, as authored.
    ///
    /// `jpg` and `jpeg` are both the JPEG format; `png` and `webp` have one spelling each.
    #[must_use]
    pub const fn accepted_extensions(self) -> &'static [&'static str] {
        match self {
            Self::Png => &["png"],
            Self::Jpeg => &["jpg", "jpeg"],
            Self::WebP => &["webp"],
        }
    }

    /// The fixed worker argument token naming this format.
    ///
    /// A closed token, never free text, so no caller can ask the worker for a format that is not
    /// one of the three approved ones.
    #[must_use]
    pub const fn worker_token(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::WebP => "webp",
        }
    }

    /// The one format an authored Entity reference declares, or `None` when this slice cannot
    /// inspect it.
    ///
    /// The decision is made **only** from the authored reference text: the namespace must be
    /// `entity`, and the final path segment's extension must be exactly `png`, `jpg`, `jpeg` or
    /// `webp` in lowercase. `AVIF`, `PNG`, a trailing dot, an extension-less segment, a `platform`
    /// reference and anything else return `None`, which the caller turns into a typed refusal
    /// rather than a skip.
    #[must_use]
    pub fn declared_by(reference: &solo_domain::AssetReferenceV1) -> Option<Self> {
        if reference.namespace() != AssetNamespace::Entity {
            return None;
        }
        let text = reference.reference();
        let segment = text.rsplit('/').next().unwrap_or(text);
        let (_, extension) = segment.rsplit_once('.')?;
        if extension.is_empty() {
            return None;
        }
        Self::ALL
            .into_iter()
            .find(|format| format.accepted_extensions().contains(&extension))
    }
}

/// One bounded, path-free inspection request handed to the outbound port.
///
/// It carries only owned source bytes, the exact SHA-256 the parent computed over them and the one
/// approved format. It never carries a file path, a command, an environment value, a publication
/// key, a URL, an EFS owner or an output destination, so a compromised worker cannot learn or
/// choose any of them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRequestV1 {
    bytes: Vec<u8>,
    source_digest: MediaWorkerDigestV1,
    declared_format: GraphicImageFormatV1,
}

impl GraphicImageInspectRequestV1 {
    /// Builds the one request shape this gate is allowed to hand out.
    ///
    /// The digest is computed here, from the bytes actually carried, rather than accepted as an
    /// argument: a caller therefore cannot ask a worker to confirm a digest of different bytes.
    #[must_use]
    pub fn of(bytes: Vec<u8>, declared_format: GraphicImageFormatV1) -> Self {
        let source_digest = sha256_media_worker_bytes_v1(&bytes);
        Self {
            bytes,
            source_digest,
            declared_format,
        }
    }

    /// The owned source bytes, exactly as acquired.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The SHA-256 the trusted parent computed over exactly these bytes.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The one approved format this request asks the worker to confirm and decode.
    #[must_use]
    pub const fn declared_format(&self) -> GraphicImageFormatV1 {
        self.declared_format
    }

    /// The largest accepted source size, in bytes, as a plain number.
    #[must_use]
    pub const fn max_source_bytes(&self) -> u64 {
        MAX_IMAGE_SOURCE_BYTES_V1
    }
}

/// What the trusted parent accepted about one worker attempt.
///
/// Every field is a parent-side fact: the format and MIME were re-derived from the bytes the parent
/// received, the digests were recomputed over owned bytes, and `limits_verified` and
/// `cleanup_verified` are host facts the parent established (kernel readback on Linux, Job Object
/// query on Windows; a confirmed reap in both cases). A worker's own assertion is never trusted in
/// their place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageWorkerObservationV1 {
    /// The still-image format the parent re-derived from the received bytes.
    pub confirmed_format: GraphicImageFormatV1,
    /// The canonical MIME of that confirmed format.
    pub observed_mime: &'static str,
    /// The SHA-256 the parent recomputed over the source it sent.
    pub source_digest: MediaWorkerDigestV1,
    /// The decoded width the parent confirmed.
    pub width: u32,
    /// The decoded height the parent confirmed.
    pub height: u32,
    /// The exact number of decoded bytes the parent received and hashed.
    pub decoded_bytes: u64,
    /// The SHA-256 the parent recomputed over exactly the decoded bytes it received.
    pub decoded_digest: MediaWorkerDigestV1,
    /// The worker process identifier the parent observed.
    pub worker_pid: u32,
    /// Whether the parent proved the confinement bounds were really installed before decode.
    pub limits_verified: bool,
    /// Whether the parent confirmed the child was reaped and its resources released.
    pub cleanup_verified: bool,
    /// The measured wall time of the attempt, in milliseconds.
    pub wall_ms: u64,
}

impl GraphicImageWorkerObservationV1 {
    /// The still-image format the parent re-derived from the received bytes.
    #[must_use]
    pub const fn confirmed_format(&self) -> GraphicImageFormatV1 {
        self.confirmed_format
    }

    /// The canonical MIME of that confirmed format.
    #[must_use]
    pub const fn observed_mime(&self) -> &'static str {
        self.observed_mime
    }

    /// The SHA-256 the parent recomputed over the source it sent.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The decoded width the parent confirmed.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The decoded height the parent confirmed.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The exact number of decoded bytes the parent received and hashed.
    #[must_use]
    pub const fn decoded_bytes(&self) -> u64 {
        self.decoded_bytes
    }

    /// The SHA-256 the parent recomputed over exactly the decoded bytes it received.
    #[must_use]
    pub const fn decoded_digest(&self) -> &MediaWorkerDigestV1 {
        &self.decoded_digest
    }

    /// The worker process identifier the parent observed.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// Whether the parent proved the confinement bounds were really installed before decode.
    #[must_use]
    pub const fn limits_verified(&self) -> bool {
        self.limits_verified
    }

    /// Whether the parent confirmed the child was reaped and its resources released.
    #[must_use]
    pub const fn cleanup_verified(&self) -> bool {
        self.cleanup_verified
    }

    /// The measured wall time of the attempt, in milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }
}

/// The closed outcome of one outbound inspection attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectPortOutcomeV1 {
    /// The worker ran, decoded completely and every parent-side check passed.
    Inspected(Box<GraphicImageWorkerObservationV1>),
    /// The attempt produced no acceptable observation.
    Refused(GraphicImageInspectRefusalKindV1),
}

/// The one replaceable outbound port of this gate.
///
/// Exactly one method, no discovery, no configuration and no second capability: an implementation
/// can confine and decode one already-owned bounded byte stream or refuse it, and can do nothing
/// else through this interface.
pub trait GraphicImageInspectPortV1 {
    /// Confines, decodes and checks one bounded request.
    fn inspect(&self, request: &GraphicImageInspectRequestV1) -> GraphicImageInspectPortOutcomeV1;
}

/// Why one whole candidate produced no complete inspection.
///
/// Closed and typed, ordered from the authored-declaration failures to the worker and output
/// failures. It carries no OS error text, no path and no source byte, so a caller can never leak
/// host detail or media content through it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphicImageInspectRefusalKindV1 {
    /// The declared reference is an AVIF, a platform reference or another unsupported extension.
    UnsupportedFormat,
    /// The acquired source is zero bytes long.
    EmptySource,
    /// The acquired source exceeds [`MAX_IMAGE_SOURCE_BYTES_V1`].
    OversizeSource,
    /// The port refused the attempt.
    PortRefused,
    /// The worker's confirmed format is not the declared one.
    FormatMismatch,
    /// The observed MIME is not the declared format's canonical MIME.
    MimeMismatch,
    /// The source digest the parent recomputed does not match the request.
    SourceDigestMismatch,
    /// A side length is zero or above [`MAX_IMAGE_SIDE_V1`].
    DimensionsOutOfRange,
    /// `width × height` overflowed or exceeds [`MAX_IMAGE_PIXELS_V1`].
    PixelCountOutOfRange,
    /// The decoded byte count is zero, above [`MAX_IMAGE_DECODED_BYTES_V1`] or not `4 × pixels`.
    DecodedBytesOutOfRange,
    /// The decoded digest is not the digest of exactly the decoded bytes received.
    OutputDigestMismatch,
    /// The parent could not prove the bounds were installed, or that cleanup completed.
    UnverifiedWorker,
}

impl GraphicImageInspectRefusalKindV1 {
    /// The stable machine-readable token of this refusal kind.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::UnsupportedFormat => "unsupported-format",
            Self::EmptySource => "empty-source",
            Self::OversizeSource => "oversize-source",
            Self::PortRefused => "port-refused",
            Self::FormatMismatch => "format-mismatch",
            Self::MimeMismatch => "mime-mismatch",
            Self::SourceDigestMismatch => "source-digest-mismatch",
            Self::DimensionsOutOfRange => "dimensions-out-of-range",
            Self::PixelCountOutOfRange => "pixel-count-out-of-range",
            Self::DecodedBytesOutOfRange => "decoded-bytes-out-of-range",
            Self::OutputDigestMismatch => "output-digest-mismatch",
            Self::UnverifiedWorker => "unverified-worker",
        }
    }
}

/// One whole-candidate refusal, naming the obligation that failed when one was identified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRefusalV1 {
    kind: GraphicImageInspectRefusalKindV1,
    declaring_owner: Option<QualifiedOwnerV1>,
    role: Option<PropertyKey>,
}

impl GraphicImageInspectRefusalV1 {
    /// A refusal that concerns no particular obligation.
    #[must_use]
    pub const fn of(kind: GraphicImageInspectRefusalKindV1) -> Self {
        Self {
            kind,
            declaring_owner: None,
            role: None,
        }
    }

    /// The same refusal, attributed to one authored obligation.
    ///
    /// It records only the qualified owner and the registered role — never a reference text, a
    /// path or a source byte.
    #[must_use]
    pub fn concerning(
        kind: GraphicImageInspectRefusalKindV1,
        obligation: &GraphicAssetObligationV1,
    ) -> Self {
        Self {
            kind,
            declaring_owner: Some(obligation.declaring_owner().clone()),
            role: Some(obligation.role()),
        }
    }

    /// Which rule refused.
    #[must_use]
    pub const fn kind(&self) -> GraphicImageInspectRefusalKindV1 {
        self.kind
    }

    /// The owner that authored the affected obligation, when one was identified.
    #[must_use]
    pub fn declaring_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.declaring_owner.as_ref()
    }

    /// The registered role of the affected obligation, when one was identified.
    #[must_use]
    pub const fn role(&self) -> Option<PropertyKey> {
        self.role
    }
}

/// One inspected image, still inert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectionV1 {
    obligation: GraphicAssetObligationV1,
    logical_path: String,
    format: GraphicImageFormatV1,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    decoded_bytes: u64,
    decoded_digest: MediaWorkerDigestV1,
    worker_pid: u32,
    wall_ms: u64,
}

impl GraphicImageInspectionV1 {
    /// The authored obligation this result answers, unchanged and still independently owned.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The confined logical path the source bytes came from.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The confirmed still-image format.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV1 {
        self.format
    }

    /// The canonical MIME of the confirmed format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The SHA-256 the parent computed over exactly the source bytes it sent.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The confirmed decoded width in pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The confirmed decoded height in pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The confirmed `width × height` product.
    #[must_use]
    pub const fn pixels(&self) -> u64 {
        self.pixels
    }

    /// The exact decoded byte count the parent received and hashed.
    #[must_use]
    pub const fn decoded_bytes(&self) -> u64 {
        self.decoded_bytes
    }

    /// The parent's digest of exactly those decoded bytes.
    #[must_use]
    pub const fn decoded_digest(&self) -> &MediaWorkerDigestV1 {
        &self.decoded_digest
    }

    /// The worker process identifier the parent observed for this attempt.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The measured wall time of this attempt, in milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this result authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// The complete inspected set of one whole candidate.
///
/// Complete or absent: there is no partial constructor, so a caller can never mistake a
/// partly-inspected inventory for a verified one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicImageInspectionSetV1 {
    inspections: Vec<GraphicImageInspectionV1>,
}

impl GraphicImageInspectionSetV1 {
    /// Every inspection, in inventory order.
    #[must_use]
    pub fn inspections(&self) -> &[GraphicImageInspectionV1] {
        &self.inspections
    }

    /// How many independently declared obligations were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inspections.len()
    }

    /// Whether the candidate declared no graphic obligation at all.
    ///
    /// A zero-obligation success is valid and still authorizes nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inspections.is_empty()
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this set is a global publishable snapshot. Always `false`.
    #[must_use]
    pub const fn proves_whole_publication(&self) -> bool {
        false
    }
}

/// The closed result of one whole-candidate inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectionOutcomeV1 {
    /// Every declared obligation was inspected.
    Complete(GraphicImageInspectionSetV1),
    /// Exactly one typed refusal; no partial set exists.
    Refused(GraphicImageInspectRefusalV1),
}

/// The decoded payload size one RGBA8 plane of these dimensions must have.
///
/// Pure checked arithmetic: a `u64` product that would overflow returns `None` rather than a
/// wrapped value, so an adversarial pair of side lengths can never look admissible.
#[must_use]
pub fn checked_rgba8_bytes_v1(width: u32, height: u32) -> Option<u64> {
    u64::from(width)
        .checked_mul(u64::from(height))?
        .checked_mul(4)
}

/// Whether one side length is inside the accepted range.
#[must_use]
pub const fn side_within_bounds_v1(side: u32) -> bool {
    side >= 1 && side <= MAX_IMAGE_SIDE_V1
}

/// Inspects every declared obligation of one complete source set, or refuses the whole candidate.
///
/// The policy is deliberately total and ordered: the authored format, the acquired size, the port
/// attempt, the confirmed format and MIME, the parent's source digest, the confinement and cleanup
/// facts, the geometry, the pixel product and the decoded payload size are each checked for every
/// obligation, and the first violation discards every result collected so far. There is no
/// "best effort", no per-image partial success and no fallback to an uninspected source.
///
/// Internal, unlisted and descendant-suppressed obligations are inspected exactly like public ones,
/// because the input is the complete authored inventory. Nothing here filters by exposure, and
/// nothing here decides publication.
#[must_use]
pub fn inspect_graphic_image_source_set_v1(
    source_set: &GraphicAssetSourceSetV1,
    port: &impl GraphicImageInspectPortV1,
) -> GraphicImageInspectionOutcomeV1 {
    let evidence = source_set.evidence();
    let mut inspections: Vec<GraphicImageInspectionV1> = Vec::with_capacity(evidence.len());

    for item in evidence {
        let obligation = item.obligation();
        let Some(format) = GraphicImageFormatV1::declared_by(obligation.reference()) else {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::UnsupportedFormat,
                obligation,
            );
        };

        let bytes = item.bytes();
        let length = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if bytes.is_empty() {
            return refuse_v1(GraphicImageInspectRefusalKindV1::EmptySource, obligation);
        }
        if length > MAX_IMAGE_SOURCE_BYTES_V1 {
            return refuse_v1(GraphicImageInspectRefusalKindV1::OversizeSource, obligation);
        }

        let request = GraphicImageInspectRequestV1::of(bytes.to_vec(), format);
        let observation = match port.inspect(&request) {
            GraphicImageInspectPortOutcomeV1::Inspected(observation) => *observation,
            GraphicImageInspectPortOutcomeV1::Refused(kind) => {
                return refuse_v1(kind, obligation);
            }
        };

        if observation.confirmed_format() != format {
            return refuse_v1(GraphicImageInspectRefusalKindV1::FormatMismatch, obligation);
        }
        if observation.observed_mime() != format.canonical_mime() {
            return refuse_v1(GraphicImageInspectRefusalKindV1::MimeMismatch, obligation);
        }
        if observation.source_digest() != request.source_digest() {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::SourceDigestMismatch,
                obligation,
            );
        }
        if !observation.limits_verified() || !observation.cleanup_verified() {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::UnverifiedWorker,
                obligation,
            );
        }
        if !side_within_bounds_v1(observation.width())
            || !side_within_bounds_v1(observation.height())
        {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::DimensionsOutOfRange,
                obligation,
            );
        }

        let Some(pixels) =
            u64::from(observation.width()).checked_mul(u64::from(observation.height()))
        else {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::PixelCountOutOfRange,
                obligation,
            );
        };
        if pixels > MAX_IMAGE_PIXELS_V1 {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::PixelCountOutOfRange,
                obligation,
            );
        }

        let Some(expected_bytes) =
            checked_rgba8_bytes_v1(observation.width(), observation.height())
        else {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
                obligation,
            );
        };
        if observation.decoded_bytes() == 0
            || observation.decoded_bytes() > MAX_IMAGE_DECODED_BYTES_V1
            || observation.decoded_bytes() != expected_bytes
        {
            return refuse_v1(
                GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
                obligation,
            );
        }

        inspections.push(GraphicImageInspectionV1 {
            obligation: obligation.clone(),
            logical_path: item.logical_path().to_owned(),
            format,
            mime: format.canonical_mime(),
            source_digest: *request.source_digest(),
            width: observation.width(),
            height: observation.height(),
            pixels,
            decoded_bytes: observation.decoded_bytes(),
            decoded_digest: *observation.decoded_digest(),
            worker_pid: observation.worker_pid(),
            wall_ms: observation.wall_ms(),
        });
    }

    GraphicImageInspectionOutcomeV1::Complete(GraphicImageInspectionSetV1 { inspections })
}

/// One attributed whole-candidate refusal.
fn refuse_v1(
    kind: GraphicImageInspectRefusalKindV1,
    obligation: &GraphicAssetObligationV1,
) -> GraphicImageInspectionOutcomeV1 {
    GraphicImageInspectionOutcomeV1::Refused(GraphicImageInspectRefusalV1::concerning(
        kind, obligation,
    ))
}

// ============================================================================
// V2 — one atomic four-format (PNG/JPEG/WebP/GIF) whole-owner inspection gate
// ============================================================================
//
// `SOLO-WO-113 R01`. The V1 contract above stays exactly as accepted: three still formats, one
// still outbound port, one still observation shape. Everything below is additive and inward-owned.
//
// The V2 gate exists because a GIF is not a still plane. A GIF source is inspected by a real
// contained worker that returns a *complete* animation observation — signature, canvas, every
// authored frame rectangle, delay, disposal, transparency, interlace, loop semantics, cycle
// duration, aggregate composed-pixel budget, source digest, bounded transcript digest and one
// composite pixel digest per frame. There is no first-frame downgrade, no implicit static
// transcode, no rewritten zero delay and no synthetic animation.
//
// What this module still never does: it opens no file, starts no process, imports no codec, no
// operating-system type, no filesystem and no adapter type. The only input is owned bytes, and
// every claim an outbound adapter makes is re-checked here against the request that was sent.

/// Whether a complete V2 four-format inspection authorizes publication of any kind.
///
/// Always `false`. A correctly inspected candidate is an inert validation result: it is not a
/// route, not a rendered Page, not a publishable asset, not a derived output and not a global
/// filesystem snapshot.
pub const GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether a complete V2 inspection proves the media is *correct*.
///
/// Always `false`. Decode success, bounded geometry and an intact worker transcript say nothing
/// about colour management, orientation, ancillary chunks, trailing data, licensing or platform
/// catalogue membership; each remains a separate, separately authorized gate.
pub const GRAPHIC_IMAGE_INSPECTION_V2_PROVES_MEDIA_CORRECTNESS: bool = false;

/// The canonical MIME of a confirmed GIF image.
pub const IMAGE_MIME_GIF_V2: &str = "image/gif";

/// The `GIF87a` signature token the parent accepts.
pub const GIF87A_SIGNATURE_V2: &str = "GIF87a";

/// The `GIF89a` signature token the parent accepts.
pub const GIF89A_SIGNATURE_V2: &str = "GIF89a";

/// The largest accepted frame count of one admitted GIF animation.
///
/// Mirrors the accepted codec contract exactly, so the inward policy cannot admit an animation the
/// confined worker would refuse.
pub const MAX_GIF_FRAMES_V2: usize = 1024;

/// The largest accepted aggregate composed-pixel budget of one admitted GIF animation.
pub const MAX_GIF_TOTAL_COMPOSITED_PIXELS_V2: u64 = 128_000_000;

/// The largest accepted authored duration of one GIF playback cycle, in milliseconds.
pub const MAX_GIF_ONE_CYCLE_DELAY_MS_V2: u64 = 3_600_000;

/// The largest accepted canonical GIF transcript, in bytes.
pub const MAX_GIF_TRACE_BYTES_V2: usize = 1024 * 1024;

/// The four image formats this V2 gate admits.
///
/// Closed and exhaustive, and deliberately **separate** from [`GraphicImageFormatV1`]: GIF is not
/// retro-fitted into the still enum, so no V1 caller can observe a fourth still variant. There is
/// still no `Other`, no `Avif` and no `Svg`, so an unimplemented format cannot be represented as an
/// approved request at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphicImageFormatV2 {
    /// Portable Network Graphics.
    Png,
    /// JPEG.
    Jpeg,
    /// WebP.
    WebP,
    /// GIF87a or GIF89a, still or animated.
    Gif,
}

impl GraphicImageFormatV2 {
    /// Every admitted format, in a fixed order.
    pub const ALL: [Self; 4] = [Self::Png, Self::Jpeg, Self::WebP, Self::Gif];

    /// The canonical MIME type of this format, fixed here rather than taken from a worker.
    #[must_use]
    pub const fn canonical_mime(self) -> &'static str {
        match self {
            Self::Png => IMAGE_MIME_PNG_V1,
            Self::Jpeg => IMAGE_MIME_JPEG_V1,
            Self::WebP => IMAGE_MIME_WEBP_V1,
            Self::Gif => IMAGE_MIME_GIF_V2,
        }
    }

    /// The exact lowercase extension tokens this format accepts, as authored.
    #[must_use]
    pub const fn accepted_extensions(self) -> &'static [&'static str] {
        match self {
            Self::Png => &["png"],
            Self::Jpeg => &["jpg", "jpeg"],
            Self::WebP => &["webp"],
            Self::Gif => &["gif"],
        }
    }

    /// Whether this format is the animation-capable one.
    #[must_use]
    pub const fn is_gif(self) -> bool {
        matches!(self, Self::Gif)
    }

    /// The V1 still format this format narrows to, or `None` for GIF.
    ///
    /// This is the only bridge between the two versions: a still format is inspected by the
    /// accepted V1 port unchanged, and GIF never becomes a still.
    #[must_use]
    pub const fn as_still_v1(self) -> Option<GraphicImageFormatV1> {
        match self {
            Self::Png => Some(GraphicImageFormatV1::Png),
            Self::Jpeg => Some(GraphicImageFormatV1::Jpeg),
            Self::WebP => Some(GraphicImageFormatV1::WebP),
            Self::Gif => None,
        }
    }

    /// The one format an authored Entity reference declares for this V2 gate, or `None`.
    ///
    /// The decision is made **only** from authored reference text: the namespace must be `entity`
    /// and the final segment's extension must be exactly `png`, `jpg`, `jpeg`, `webp` or `gif` in
    /// lowercase. `GIF`, a trailing dot, an extensionless segment, a `platform` reference, `.avif`,
    /// `.svg` and anything else return `None`, which the policy turns into a typed refusal rather
    /// than a skip.
    #[must_use]
    pub fn declared_by(reference: &solo_domain::AssetReferenceV1) -> Option<Self> {
        if reference.namespace() != AssetNamespace::Entity {
            return None;
        }
        let text = reference.reference();
        let segment = text.rsplit('/').next().unwrap_or(text);
        let (_, extension) = segment.rsplit_once('.')?;
        if extension.is_empty() {
            return None;
        }
        Self::ALL
            .into_iter()
            .find(|format| format.accepted_extensions().contains(&extension))
    }
}

/// The authored loop semantics of one GIF animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GifLoopV2 {
    /// No repeat extension was authored: the animation plays once.
    Absent,
    /// A finite authored repeat count.
    Finite(u16),
    /// An authored repeat count of zero, which means "loop forever".
    Infinite,
}

impl GifLoopV2 {
    /// The stable machine-readable token of these loop semantics.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Finite(_) => "finite",
            Self::Infinite => "infinite",
        }
    }
}

/// The authored disposal method of one GIF frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GifDisposalV2 {
    /// No disposal method was authored.
    Unspecified,
    /// Leave the frame in place.
    Keep,
    /// Restore the frame rectangle to the background.
    Background,
    /// Restore the frame rectangle to the previous canvas.
    Previous,
}

impl GifDisposalV2 {
    /// The stable machine-readable token of this disposal method.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::Keep => "keep",
            Self::Background => "background",
            Self::Previous => "previous",
        }
    }
}

/// One ordered frame of a parent-validated GIF animation, in inward-owned terms.
///
/// Fields are plain data — index, authored rectangle, authored delay, disposal, transparency,
/// interlace, local-palette presence and the composite pixel digest the trusted parent recomputed
/// for this frame. No adapter type, codec handle or decoder state crosses this boundary, and the
/// policy re-checks every field against the canvas and the frame sequence before accepting it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GifFrameEvidenceV2 {
    /// Zero-based authored position in the animation.
    pub index: usize,
    /// Authored frame delay, in centiseconds, exactly as written.
    pub delay_centiseconds: u16,
    /// Authored left offset of the frame rectangle.
    pub left: u16,
    /// Authored top offset of the frame rectangle.
    pub top: u16,
    /// Authored frame width.
    pub width: u16,
    /// Authored frame height.
    pub height: u16,
    /// Authored disposal method.
    pub disposal: GifDisposalV2,
    /// Authored transparent palette index, when one was authored.
    pub transparent_index: Option<u8>,
    /// Whether the frame was authored interlaced.
    pub interlaced: bool,
    /// Whether the frame carried its own local colour table.
    pub local_palette: bool,
    /// The digest the trusted parent recomputed over this frame's composited RGBA8 canvas.
    pub composite_digest: MediaWorkerDigestV1,
}

/// A complete, parent-validated observation of one GIF animation or still GIF.
///
/// Produced only after a real contained worker returned normally and the parent re-verified the
/// source digest, the child identity, confinement and reap, the transcript length within
/// [`MAX_GIF_TRACE_BYTES_V2`], the transcript digest and the whole canonical transcript grammar.
///
/// A well-formed transcript digest proves *transport* integrity: that the bytes the worker intended
/// to send arrived unchanged. It is not independent proof that a malicious child reported truthful
/// pixels; pixel truth is only ever established by the recorded composite digests and, for known
/// fixtures, by an independent expected-pixel oracle in the test suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GifAnimationEvidenceV2 {
    /// The confirmed signature, `GIF87a` or `GIF89a`.
    pub signature: &'static str,
    /// The canonical MIME, always [`IMAGE_MIME_GIF_V2`].
    pub mime: &'static str,
    /// The confirmed logical-screen width.
    pub width: u16,
    /// The confirmed logical-screen height.
    pub height: u16,
    /// Every frame, in authored order.
    pub frames: Vec<GifFrameEvidenceV2>,
    /// The authored loop semantics.
    pub loop_semantics: GifLoopV2,
    /// The authored duration of exactly one playback cycle, in milliseconds.
    pub one_cycle_delay_ms: u64,
    /// The aggregate composed-pixel budget the worker proved.
    pub total_composited_pixels: u64,
    /// The SHA-256 the parent computed over exactly the source bytes it sent.
    pub source_digest: MediaWorkerDigestV1,
    /// The SHA-256 the parent recomputed over exactly the transcript it received.
    pub transcript_digest: MediaWorkerDigestV1,
    /// The exact transcript length in bytes.
    pub transcript_len: usize,
    /// The child process identifier the parent observed.
    pub worker_pid: u32,
    /// Whether the parent proved the confinement bounds were installed.
    pub limits_verified: bool,
    /// Whether the parent confirmed the child was reaped and released.
    pub cleanup_verified: bool,
    /// The parent-measured wall time, in milliseconds.
    pub wall_ms: u64,
}

impl GifAnimationEvidenceV2 {
    /// How many frames this animation declares.
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Whether this is a genuine multi-frame timeline.
    #[must_use]
    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1
    }

    /// The intrinsic `width × height` product, or `None` on overflow.
    #[must_use]
    pub fn intrinsic_pixels(&self) -> Option<u64> {
        u64::from(self.width).checked_mul(u64::from(self.height))
    }
}

/// One bounded, path-free V2 inspection request handed to the outbound port.
///
/// It carries only owned source bytes, the exact SHA-256 computed here over them and the one
/// approved V2 format. It never carries a path, a command, an environment value, a publication key,
/// a URL, an EFS owner or an output destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRequestV2 {
    bytes: Vec<u8>,
    source_digest: MediaWorkerDigestV1,
    declared_format: GraphicImageFormatV2,
}

impl GraphicImageInspectRequestV2 {
    /// Builds the one request shape this gate may hand out.
    ///
    /// The digest is computed from the carried bytes rather than accepted as an argument, so no
    /// caller can ask a worker to confirm a digest of different bytes.
    #[must_use]
    pub fn of(bytes: Vec<u8>, declared_format: GraphicImageFormatV2) -> Self {
        let source_digest = sha256_media_worker_bytes_v1(&bytes);
        Self {
            bytes,
            source_digest,
            declared_format,
        }
    }

    /// The owned source bytes, exactly as acquired.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The SHA-256 computed here over exactly these bytes.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The one approved V2 format this request asks the port to confirm.
    #[must_use]
    pub const fn declared_format(&self) -> GraphicImageFormatV2 {
        self.declared_format
    }

    /// The largest accepted source size, in bytes.
    #[must_use]
    pub const fn max_source_bytes(&self) -> u64 {
        MAX_IMAGE_SOURCE_BYTES_V1
    }
}

/// What one outbound attempt returned, in the two result classes this gate distinguishes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageObservationV2 {
    /// A confirmed still image, in the accepted V1 observation shape.
    Still(Box<GraphicImageWorkerObservationV1>),
    /// A confirmed GIF animation or still GIF.
    Animation(Box<GifAnimationEvidenceV2>),
}

/// The closed outcome of one V2 outbound attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectPortOutcomeV2 {
    /// The port returned a typed observation; the policy still validates every claim.
    Inspected(Box<GraphicImageObservationV2>),
    /// The attempt produced no acceptable observation.
    Refused(GraphicImageInspectRefusalKindV2),
}

/// The one replaceable V2 outbound port.
///
/// Exactly one method: an implementation can confine and inspect one already-owned bounded byte
/// stream, or refuse it, and can do nothing else through this interface. It cannot learn a path, an
/// owner, a role, a publication key or an output destination from the request.
pub trait GraphicImageInspectPortV2 {
    /// Inspects one bounded request in the format it declares.
    fn inspect_v2(
        &self,
        request: &GraphicImageInspectRequestV2,
    ) -> GraphicImageInspectPortOutcomeV2;
}

/// Why one whole V2 candidate produced no complete inspection.
///
/// Closed, typed and ordered from authored-declaration failures through worker and GIF-evidence
/// failures. It carries no OS error text, no path and no source byte, so a caller can never leak
/// host detail or media content through it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphicImageInspectRefusalKindV2 {
    /// The declared reference is AVIF, SVG, a platform reference or another unsupported extension.
    UnsupportedFormat,
    /// A `graphic.favicon` obligation was not a PNG, which the registered role forbids.
    FaviconNotPng,
    /// The acquired source is zero bytes long.
    EmptySource,
    /// The acquired source exceeds [`MAX_IMAGE_SOURCE_BYTES_V1`].
    OversizeSource,
    /// The port refused the attempt.
    PortRefused,
    /// The observation's format or result class is not the declared one.
    FormatMismatch,
    /// The observed MIME is not the declared format's canonical MIME.
    MimeMismatch,
    /// The source digest the parent recomputed does not match the request.
    SourceDigestMismatch,
    /// The parent could not prove confinement, cleanup or a real child identity.
    UnverifiedWorker,
    /// A side length, frame width or frame height is zero or above [`MAX_IMAGE_SIDE_V1`].
    DimensionsOutOfRange,
    /// `width × height` overflowed or exceeds [`MAX_IMAGE_PIXELS_V1`].
    PixelCountOutOfRange,
    /// A decoded byte count is zero, oversized or not `4 × pixels`.
    DecodedBytesOutOfRange,
    /// A decoded or composite digest is not the digest of exactly the bytes it claims.
    OutputDigestMismatch,
    /// A GIF declared zero frames, too many frames, or a non-sequential frame index.
    GifFrameCountOutOfRange,
    /// A GIF frame rectangle leaves the canvas or has a zero side.
    GifFrameGeometryOutOfRange,
    /// A GIF claim is internally impossible: loop, cycle, canvas budget or transcript.
    GifTimelineInconsistent,
}

impl GraphicImageInspectRefusalKindV2 {
    /// The stable machine-readable token of this refusal kind.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::UnsupportedFormat => "unsupported-format",
            Self::FaviconNotPng => "favicon-not-png",
            Self::EmptySource => "empty-source",
            Self::OversizeSource => "oversize-source",
            Self::PortRefused => "port-refused",
            Self::FormatMismatch => "format-mismatch",
            Self::MimeMismatch => "mime-mismatch",
            Self::SourceDigestMismatch => "source-digest-mismatch",
            Self::UnverifiedWorker => "unverified-worker",
            Self::DimensionsOutOfRange => "dimensions-out-of-range",
            Self::PixelCountOutOfRange => "pixel-count-out-of-range",
            Self::DecodedBytesOutOfRange => "decoded-bytes-out-of-range",
            Self::OutputDigestMismatch => "output-digest-mismatch",
            Self::GifFrameCountOutOfRange => "gif-frame-count-out-of-range",
            Self::GifFrameGeometryOutOfRange => "gif-frame-geometry-out-of-range",
            Self::GifTimelineInconsistent => "gif-timeline-inconsistent",
        }
    }
}

/// One whole-candidate V2 refusal, naming the obligation that failed when one was identified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRefusalV2 {
    kind: GraphicImageInspectRefusalKindV2,
    declaring_owner: Option<QualifiedOwnerV1>,
    role: Option<PropertyKey>,
    local_index: Option<usize>,
}

impl GraphicImageInspectRefusalV2 {
    /// A refusal that concerns no particular obligation.
    #[must_use]
    pub const fn of(kind: GraphicImageInspectRefusalKindV2) -> Self {
        Self {
            kind,
            declaring_owner: None,
            role: None,
            local_index: None,
        }
    }

    /// The same refusal, attributed to one authored obligation.
    ///
    /// It records only the qualified owner, the registered role and the authored local index —
    /// never a reference text, a path or a source byte.
    #[must_use]
    pub fn concerning(
        kind: GraphicImageInspectRefusalKindV2,
        obligation: &GraphicAssetObligationV1,
    ) -> Self {
        Self {
            kind,
            declaring_owner: Some(obligation.declaring_owner().clone()),
            role: Some(obligation.role()),
            local_index: Some(obligation.local_index()),
        }
    }

    /// Which rule refused.
    #[must_use]
    pub const fn kind(&self) -> GraphicImageInspectRefusalKindV2 {
        self.kind
    }

    /// The stable token of the rule that refused.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.kind.token()
    }

    /// The owner that authored the affected obligation, when one was identified.
    #[must_use]
    pub fn declaring_owner(&self) -> Option<&QualifiedOwnerV1> {
        self.declaring_owner.as_ref()
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

/// The still-image half of one V2 result: the decoded payload facts only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StillImageEvidenceV2 {
    /// The exact decoded byte count the parent received and hashed.
    pub decoded_bytes: u64,
    /// The parent's digest of exactly those decoded bytes.
    pub decoded_digest: MediaWorkerDigestV1,
}

/// The typed payload of one inspected image: still plane or GIF timeline, never both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImagePayloadV2 {
    /// A decoded still plane.
    Still(StillImageEvidenceV2),
    /// A complete GIF animation or still GIF.
    Animation(GifAnimationEvidenceV2),
}

/// One inspected image of a V2 candidate, still inert.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectionV2 {
    obligation: GraphicAssetObligationV1,
    logical_path: String,
    format: GraphicImageFormatV2,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV2,
    worker_pid: u32,
    wall_ms: u64,
}

impl GraphicImageInspectionV2 {
    /// The authored obligation this result answers, unchanged and independently owned.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The confined logical path the source bytes came from.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The confirmed format.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV2 {
        self.format
    }

    /// The canonical MIME of the confirmed format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The SHA-256 computed over exactly the source bytes sent.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The confirmed intrinsic width in pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The confirmed intrinsic height in pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The confirmed `width × height` product.
    #[must_use]
    pub const fn pixels(&self) -> u64 {
        self.pixels
    }

    /// The typed payload: exactly one of the two result classes.
    #[must_use]
    pub const fn payload(&self) -> &GraphicImagePayloadV2 {
        &self.payload
    }

    /// Whether this result is a GIF timeline.
    #[must_use]
    pub const fn is_animation(&self) -> bool {
        matches!(self.payload, GraphicImagePayloadV2::Animation(_))
    }

    /// The GIF evidence, when this result is an animation.
    #[must_use]
    pub const fn animation(&self) -> Option<&GifAnimationEvidenceV2> {
        match &self.payload {
            GraphicImagePayloadV2::Animation(evidence) => Some(evidence),
            GraphicImagePayloadV2::Still(_) => None,
        }
    }

    /// The still payload, when this result is a still image.
    #[must_use]
    pub const fn still(&self) -> Option<&StillImageEvidenceV2> {
        match &self.payload {
            GraphicImagePayloadV2::Still(evidence) => Some(evidence),
            GraphicImagePayloadV2::Animation(_) => None,
        }
    }

    /// The worker process identifier the parent observed for this attempt.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The measured wall time of this attempt, in milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this result authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// The complete inspected set of one whole V2 candidate.
///
/// Complete or absent: there is no partial constructor, so a caller can never mistake a
/// partly-inspected inventory for a verified one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicImageInspectionSetV2 {
    inspections: Vec<GraphicImageInspectionV2>,
}

impl GraphicImageInspectionSetV2 {
    /// Every inspection, in inventory order.
    #[must_use]
    pub fn inspections(&self) -> &[GraphicImageInspectionV2] {
        &self.inspections
    }

    /// How many independently declared obligations were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inspections.len()
    }

    /// Whether the candidate declared no graphic obligation at all.
    ///
    /// A zero-obligation success is valid, distinct from a refusal, and still authorizes nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inspections.is_empty()
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this set is a global publishable snapshot. Always `false`.
    #[must_use]
    pub const fn proves_whole_publication(&self) -> bool {
        false
    }
}

/// The closed result of one whole-candidate V2 inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectionOutcomeV2 {
    /// Every declared obligation was inspected.
    Complete(GraphicImageInspectionSetV2),
    /// Exactly one typed refusal; no partial set exists.
    Refused(GraphicImageInspectRefusalV2),
}

use GraphicImageInspectRefusalKindV2 as RefusalV2;

/// Whether one registered property role admits this V2 format.
///
/// `graphic.favicon` stays **PNG-only**: the registered role, not a filename and not an inference,
/// is what the inward policy reads. Every other registered graphic role admits all four formats.
#[must_use]
pub const fn role_admits_format_v2(role: PropertyKey, format: GraphicImageFormatV2) -> bool {
    match role {
        PropertyKey::GraphicFavicon => matches!(format, GraphicImageFormatV2::Png),
        _ => true,
    }
}

/// Whether a digest could be the SHA-256 of any real payload.
///
/// A digest of thirty-two zero bytes is never a real SHA-256 of media bytes, so an observation that
/// claims one is refused as impossible rather than accepted as opaque evidence.
fn digest_is_plausible_v2(digest: &MediaWorkerDigestV1) -> bool {
    digest.as_bytes() != &[0u8; 32]
}

/// Validates one still observation against the request that produced it.
///
/// Returns the confirmed geometry and pixel product only when every parent-side claim holds: the
/// narrow V1 format, the canonical MIME, the source digest, the child identity, confinement, reap,
/// both side bounds, the checked pixel product and the decoded payload length.
fn validate_still_observation_v2(
    observation: &GraphicImageWorkerObservationV1,
    request: &GraphicImageInspectRequestV2,
) -> Result<(u32, u32, u64, StillImageEvidenceV2), RefusalV2> {
    let format = request.declared_format();
    if format.as_still_v1() != Some(observation.confirmed_format()) {
        return Err(RefusalV2::FormatMismatch);
    }
    if observation.observed_mime() != format.canonical_mime() {
        return Err(RefusalV2::MimeMismatch);
    }
    if observation.source_digest() != request.source_digest() {
        return Err(RefusalV2::SourceDigestMismatch);
    }
    if !observation.limits_verified()
        || !observation.cleanup_verified()
        || observation.worker_pid() == 0
    {
        return Err(RefusalV2::UnverifiedWorker);
    }
    let width = observation.width();
    let height = observation.height();
    if !side_within_bounds_v1(width) || !side_within_bounds_v1(height) {
        return Err(RefusalV2::DimensionsOutOfRange);
    }
    let Some(pixels) = u64::from(width).checked_mul(u64::from(height)) else {
        return Err(RefusalV2::PixelCountOutOfRange);
    };
    if pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(RefusalV2::PixelCountOutOfRange);
    }
    let Some(expected_bytes) = checked_rgba8_bytes_v1(width, height) else {
        return Err(RefusalV2::DecodedBytesOutOfRange);
    };
    if observation.decoded_bytes() == 0
        || observation.decoded_bytes() > MAX_IMAGE_DECODED_BYTES_V1
        || observation.decoded_bytes() != expected_bytes
    {
        return Err(RefusalV2::DecodedBytesOutOfRange);
    }
    if !digest_is_plausible_v2(observation.decoded_digest()) {
        return Err(RefusalV2::OutputDigestMismatch);
    }
    Ok((
        width,
        height,
        pixels,
        StillImageEvidenceV2 {
            decoded_bytes: observation.decoded_bytes(),
            decoded_digest: *observation.decoded_digest(),
        },
    ))
}

/// Validates one GIF animation observation against the request that produced it.
///
/// This is the inward half of the GIF contract: signature and MIME, source digest, child identity,
/// confinement and reap, both intrinsic side bounds, the intrinsic 32-million-pixel ceiling (checked
/// separately from the codec's own composed budget), the frame count and exact index order, every
/// authored rectangle against the canvas, the per-frame delay sum against the claimed cycle, the
/// aggregate composed-pixel budget, the transcript bound and the impossibility of a zero-length or
/// all-zero digest. Returns the confirmed intrinsic geometry only when every rule holds.
fn validate_gif_evidence_v2(
    evidence: &GifAnimationEvidenceV2,
    request: &GraphicImageInspectRequestV2,
) -> Result<(u32, u32, u64), RefusalV2> {
    if evidence.signature != GIF87A_SIGNATURE_V2 && evidence.signature != GIF89A_SIGNATURE_V2 {
        return Err(RefusalV2::FormatMismatch);
    }
    if evidence.mime != IMAGE_MIME_GIF_V2 {
        return Err(RefusalV2::MimeMismatch);
    }
    if &evidence.source_digest != request.source_digest() {
        return Err(RefusalV2::SourceDigestMismatch);
    }
    if !evidence.limits_verified || !evidence.cleanup_verified || evidence.worker_pid == 0 {
        return Err(RefusalV2::UnverifiedWorker);
    }

    let width = u32::from(evidence.width);
    let height = u32::from(evidence.height);
    if !side_within_bounds_v1(width) || !side_within_bounds_v1(height) {
        return Err(RefusalV2::DimensionsOutOfRange);
    }
    let Some(intrinsic_pixels) = evidence.intrinsic_pixels() else {
        return Err(RefusalV2::PixelCountOutOfRange);
    };
    if intrinsic_pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(RefusalV2::PixelCountOutOfRange);
    }

    if evidence.frames.is_empty()
        || evidence.frames.len() > MAX_GIF_FRAMES_V2
        || evidence.transcript_len == 0
        || evidence.transcript_len > MAX_GIF_TRACE_BYTES_V2
    {
        return Err(RefusalV2::GifFrameCountOutOfRange);
    }
    if matches!(evidence.loop_semantics, GifLoopV2::Finite(0)) {
        return Err(RefusalV2::GifTimelineInconsistent);
    }
    if !digest_is_plausible_v2(&evidence.transcript_digest) {
        return Err(RefusalV2::OutputDigestMismatch);
    }

    let mut delay_sum_ms: u64 = 0;
    for (position, frame) in evidence.frames.iter().enumerate() {
        if frame.index != position {
            return Err(RefusalV2::GifFrameCountOutOfRange);
        }
        if frame.width == 0
            || frame.height == 0
            || !side_within_bounds_v1(u32::from(frame.width))
            || !side_within_bounds_v1(u32::from(frame.height))
        {
            return Err(RefusalV2::GifFrameGeometryOutOfRange);
        }
        let right = u64::from(frame.left).checked_add(u64::from(frame.width));
        let bottom = u64::from(frame.top).checked_add(u64::from(frame.height));
        let (Some(right), Some(bottom)) = (right, bottom) else {
            return Err(RefusalV2::GifFrameGeometryOutOfRange);
        };
        if right > u64::from(evidence.width) || bottom > u64::from(evidence.height) {
            return Err(RefusalV2::GifFrameGeometryOutOfRange);
        }
        if !digest_is_plausible_v2(&frame.composite_digest) {
            return Err(RefusalV2::OutputDigestMismatch);
        }
        let Some(frame_ms) = u64::from(frame.delay_centiseconds).checked_mul(10) else {
            return Err(RefusalV2::GifTimelineInconsistent);
        };
        let Some(total_ms) = delay_sum_ms.checked_add(frame_ms) else {
            return Err(RefusalV2::GifTimelineInconsistent);
        };
        delay_sum_ms = total_ms;
        if delay_sum_ms > MAX_GIF_ONE_CYCLE_DELAY_MS_V2 {
            return Err(RefusalV2::GifTimelineInconsistent);
        }
    }
    if delay_sum_ms != evidence.one_cycle_delay_ms {
        return Err(RefusalV2::GifTimelineInconsistent);
    }

    let Some(frames) = u64::try_from(evidence.frames.len()).ok() else {
        return Err(RefusalV2::GifFrameCountOutOfRange);
    };
    let Some(composed) = frames.checked_mul(intrinsic_pixels) else {
        return Err(RefusalV2::GifTimelineInconsistent);
    };
    if composed != evidence.total_composited_pixels
        || evidence.total_composited_pixels > MAX_GIF_TOTAL_COMPOSITED_PIXELS_V2
    {
        return Err(RefusalV2::GifTimelineInconsistent);
    }
    Ok((width, height, intrinsic_pixels))
}

/// One attributed whole-candidate V2 refusal.
fn refuse_v2(
    kind: GraphicImageInspectRefusalKindV2,
    obligation: &GraphicAssetObligationV1,
) -> GraphicImageInspectionOutcomeV2 {
    GraphicImageInspectionOutcomeV2::Refused(GraphicImageInspectRefusalV2::concerning(
        kind, obligation,
    ))
}

/// Inspects every declared obligation of one complete source set through the V2 four-format gate.
///
/// The policy is total and ordered: the authored format, the registered-role admissibility, the
/// acquired size, the port attempt, then every claim the observation makes are each checked for
/// every obligation, and the first violation discards every result collected so far. There is no
/// "best effort", no per-image partial success, no skipped unsupported asset and no fallback to an
/// uninspected source.
///
/// Internal, unlisted and descendant-suppressed obligations are inspected exactly like public ones,
/// because the input is the complete authored inventory. Nothing here filters by exposure, and
/// nothing here decides publication.
#[must_use]
pub fn inspect_graphic_image_source_set_v2(
    source_set: &GraphicAssetSourceSetV1,
    port: &impl GraphicImageInspectPortV2,
) -> GraphicImageInspectionOutcomeV2 {
    let evidence = source_set.evidence();
    let mut inspections: Vec<GraphicImageInspectionV2> = Vec::with_capacity(evidence.len());

    for item in evidence {
        let obligation = item.obligation();
        let Some(format) = GraphicImageFormatV2::declared_by(obligation.reference()) else {
            return refuse_v2(RefusalV2::UnsupportedFormat, obligation);
        };
        if !role_admits_format_v2(obligation.role(), format) {
            return refuse_v2(RefusalV2::FaviconNotPng, obligation);
        }

        let bytes = item.bytes();
        let length = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if bytes.is_empty() {
            return refuse_v2(RefusalV2::EmptySource, obligation);
        }
        if length > MAX_IMAGE_SOURCE_BYTES_V1 {
            return refuse_v2(RefusalV2::OversizeSource, obligation);
        }

        let request = GraphicImageInspectRequestV2::of(bytes.to_vec(), format);
        let observation = match port.inspect_v2(&request) {
            GraphicImageInspectPortOutcomeV2::Inspected(observation) => *observation,
            GraphicImageInspectPortOutcomeV2::Refused(kind) => {
                return refuse_v2(kind, obligation);
            }
        };

        let (width, height, pixels, payload, worker_pid, wall_ms) = match observation {
            GraphicImageObservationV2::Still(observation) => {
                if format.is_gif() {
                    return refuse_v2(RefusalV2::FormatMismatch, obligation);
                }
                let (width, height, pixels, still) =
                    match validate_still_observation_v2(&observation, &request) {
                        Ok(checked) => checked,
                        Err(kind) => return refuse_v2(kind, obligation),
                    };
                (
                    width,
                    height,
                    pixels,
                    GraphicImagePayloadV2::Still(still),
                    observation.worker_pid(),
                    observation.wall_ms(),
                )
            }
            GraphicImageObservationV2::Animation(evidence) => {
                if !format.is_gif() {
                    return refuse_v2(RefusalV2::FormatMismatch, obligation);
                }
                let (width, height, pixels) = match validate_gif_evidence_v2(&evidence, &request) {
                    Ok(checked) => checked,
                    Err(kind) => return refuse_v2(kind, obligation),
                };
                let worker_pid = evidence.worker_pid;
                let wall_ms = evidence.wall_ms;
                (
                    width,
                    height,
                    pixels,
                    GraphicImagePayloadV2::Animation(*evidence),
                    worker_pid,
                    wall_ms,
                )
            }
        };

        inspections.push(GraphicImageInspectionV2 {
            obligation: obligation.clone(),
            logical_path: item.logical_path().to_owned(),
            format,
            mime: format.canonical_mime(),
            source_digest: *request.source_digest(),
            width,
            height,
            pixels,
            payload,
            worker_pid,
            wall_ms,
        });
    }

    GraphicImageInspectionOutcomeV2::Complete(GraphicImageInspectionSetV2 { inspections })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(namespace: AssetNamespace, text: &str) -> solo_domain::AssetReferenceV1 {
        solo_domain::AssetReferenceV1::parse(namespace, text).expect("test reference is valid")
    }

    #[test]
    fn n097_01_only_lowercase_entity_extensions_declare_an_admitted_format() {
        let png = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Entity,
            "assets/hero.png",
        ));
        let jpg = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Entity,
            "assets/hero.jpg",
        ));
        let jpeg = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Entity,
            "assets/deep/hero.jpeg",
        ));
        let webp = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Entity,
            "assets/hero.webp",
        ));

        assert_eq!(png, Some(GraphicImageFormatV1::Png));
        assert_eq!(jpg, Some(GraphicImageFormatV1::Jpeg));
        assert_eq!(jpeg, Some(GraphicImageFormatV1::Jpeg));
        assert_eq!(webp, Some(GraphicImageFormatV1::WebP));
    }

    #[test]
    fn n097_01_avif_and_platform_references_are_never_admitted() {
        let avif = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Entity,
            "assets/hero.avif",
        ));
        let platform = GraphicImageFormatV1::declared_by(&reference(
            AssetNamespace::Platform,
            "assets/hero.png",
        ));

        assert_eq!(avif, None, "AVIF decoding is not implemented in this slice");
        assert_eq!(
            platform, None,
            "a platform reference is not an Entity source and is refused, never skipped"
        );
    }

    #[test]
    fn n097_01_the_declared_format_comes_from_the_reference_not_from_content() {
        // Two references with equal bytes but different extensions are different formats: the
        // decision is textual and authored, never an inference from content.
        let as_png = reference(AssetNamespace::Entity, "assets/a.png");
        let as_jpeg = reference(AssetNamespace::Entity, "assets/a.jpeg");
        assert_eq!(
            GraphicImageFormatV1::declared_by(&as_png),
            Some(GraphicImageFormatV1::Png)
        );
        assert_eq!(
            GraphicImageFormatV1::declared_by(&as_jpeg),
            Some(GraphicImageFormatV1::Jpeg)
        );
    }

    #[test]
    fn n097_01_the_canonical_mime_and_worker_tokens_are_fixed() {
        assert_eq!(GraphicImageFormatV1::Png.canonical_mime(), "image/png");
        assert_eq!(GraphicImageFormatV1::Jpeg.canonical_mime(), "image/jpeg");
        assert_eq!(GraphicImageFormatV1::WebP.canonical_mime(), "image/webp");
        assert_eq!(GraphicImageFormatV1::Png.worker_token(), "png");
        assert_eq!(GraphicImageFormatV1::Jpeg.worker_token(), "jpeg");
        assert_eq!(GraphicImageFormatV1::WebP.worker_token(), "webp");
        assert_eq!(GraphicImageFormatV1::ALL.len(), 3);
    }

    #[test]
    fn n097_01_the_geometry_bounds_are_the_accepted_contract() {
        assert!(side_within_bounds_v1(1));
        assert!(side_within_bounds_v1(MAX_IMAGE_SIDE_V1));
        assert!(!side_within_bounds_v1(0));
        assert!(!side_within_bounds_v1(MAX_IMAGE_SIDE_V1 + 1));
        assert_eq!(MAX_IMAGE_PIXELS_V1, 32_000_000);
        assert_eq!(MAX_IMAGE_SOURCE_BYTES_V1, 20 * 1024 * 1024);
        assert_eq!(MAX_IMAGE_DECODED_BYTES_V1, 256 * 1024 * 1024);
        assert_eq!(IMAGE_WORKER_WALL_MS_V1, 15_000);
    }

    #[test]
    fn n097_01_the_pixel_product_is_checked_and_never_wraps() {
        assert_eq!(checked_rgba8_bytes_v1(1, 1), Some(4));
        assert_eq!(checked_rgba8_bytes_v1(8192, 8192), Some(268_435_456));
        assert_eq!(checked_rgba8_bytes_v1(u32::MAX, u32::MAX), None);
        assert_eq!(checked_rgba8_bytes_v1(0, 10), Some(0));
    }
}

#[cfg(test)]
mod contract_tests {
    use super::*;

    #[test]
    fn n097_01_every_refusal_kind_has_one_stable_token() {
        let kinds = [
            GraphicImageInspectRefusalKindV1::UnsupportedFormat,
            GraphicImageInspectRefusalKindV1::EmptySource,
            GraphicImageInspectRefusalKindV1::OversizeSource,
            GraphicImageInspectRefusalKindV1::PortRefused,
            GraphicImageInspectRefusalKindV1::FormatMismatch,
            GraphicImageInspectRefusalKindV1::MimeMismatch,
            GraphicImageInspectRefusalKindV1::SourceDigestMismatch,
            GraphicImageInspectRefusalKindV1::DimensionsOutOfRange,
            GraphicImageInspectRefusalKindV1::PixelCountOutOfRange,
            GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
            GraphicImageInspectRefusalKindV1::OutputDigestMismatch,
            GraphicImageInspectRefusalKindV1::UnverifiedWorker,
        ];
        let mut tokens: Vec<&str> = kinds.iter().map(|kind| kind.token()).collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "refusal tokens must be unique");
        assert!(tokens.iter().all(|token| token.is_ascii()));
    }

    #[test]
    fn n097_01_publication_authority_is_never_granted() {
        const {
            assert!(!GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY);
            assert!(!GRAPHIC_IMAGE_INSPECTION_PROVES_MEDIA_CORRECTNESS);
        }

        let empty = GraphicImageInspectionSetV1::default();
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(!empty.grants_publication_authority());
        assert!(!empty.proves_whole_publication());
    }

    #[test]
    fn n097_01_the_request_digest_is_computed_from_the_carried_bytes() {
        let request = GraphicImageInspectRequestV1::of(vec![1, 2, 3, 4], GraphicImageFormatV1::Png);
        assert_eq!(request.bytes(), &[1, 2, 3, 4]);
        assert_eq!(
            request.source_digest().to_hex(),
            sha256_media_worker_bytes_v1(&[1, 2, 3, 4]).to_hex()
        );
        assert_eq!(request.declared_format(), GraphicImageFormatV1::Png);
        assert_eq!(request.max_source_bytes(), MAX_IMAGE_SOURCE_BYTES_V1);

        let other = GraphicImageInspectRequestV1::of(vec![4, 3, 2, 1], GraphicImageFormatV1::Png);
        assert_ne!(request.source_digest(), other.source_digest());
    }

    #[test]
    fn n097_01_an_unattributed_refusal_names_no_obligation() {
        let refusal =
            GraphicImageInspectRefusalV1::of(GraphicImageInspectRefusalKindV1::PortRefused);
        assert_eq!(
            refusal.kind(),
            GraphicImageInspectRefusalKindV1::PortRefused
        );
        assert!(refusal.declaring_owner().is_none());
        assert!(refusal.role().is_none());
    }
}

/// `SOLO-WO-113 R01` — the real inward V2 validation, driven by fabricated observations.
///
/// These are not a shadow validator and not a replay of adapter logic: every case calls the same
/// `validate_*_v2` and policy functions production calls, with an observation whose fields a hostile
/// adapter could really return. Each falsification must be refused by the inward side alone.
#[cfg(test)]
mod v2_tests {
    use super::*;

    fn request(bytes: &[u8], format: GraphicImageFormatV2) -> GraphicImageInspectRequestV2 {
        GraphicImageInspectRequestV2::of(bytes.to_vec(), format)
    }

    fn still_observation(
        request: &GraphicImageInspectRequestV2,
        width: u32,
        height: u32,
    ) -> GraphicImageWorkerObservationV1 {
        let format = request.declared_format();
        GraphicImageWorkerObservationV1 {
            confirmed_format: format.as_still_v1().expect("still format"),
            observed_mime: format.canonical_mime(),
            source_digest: *request.source_digest(),
            width,
            height,
            decoded_bytes: u64::from(width) * u64::from(height) * 4,
            decoded_digest: sha256_media_worker_bytes_v1(&[9u8; 4]),
            worker_pid: 4242,
            limits_verified: true,
            cleanup_verified: true,
            wall_ms: 3,
        }
    }

    fn frame(index: usize, delay: u16) -> GifFrameEvidenceV2 {
        GifFrameEvidenceV2 {
            index,
            delay_centiseconds: delay,
            left: 0,
            top: 0,
            width: 1,
            height: 1,
            disposal: GifDisposalV2::Keep,
            transparent_index: None,
            interlaced: false,
            local_palette: false,
            composite_digest: sha256_media_worker_bytes_v1(&[255, 0, 0, 255]),
        }
    }

    fn animation(
        request: &GraphicImageInspectRequestV2,
        frames: Vec<GifFrameEvidenceV2>,
        cycle_ms: u64,
    ) -> GifAnimationEvidenceV2 {
        let composed = u64::try_from(frames.len()).unwrap_or(u64::MAX);
        GifAnimationEvidenceV2 {
            signature: GIF89A_SIGNATURE_V2,
            mime: IMAGE_MIME_GIF_V2,
            width: 1,
            height: 1,
            frames,
            loop_semantics: GifLoopV2::Infinite,
            one_cycle_delay_ms: cycle_ms,
            total_composited_pixels: composed,
            source_digest: *request.source_digest(),
            transcript_digest: sha256_media_worker_bytes_v1(b"transcript"),
            transcript_len: 96,
            worker_pid: 4343,
            limits_verified: true,
            cleanup_verified: true,
            wall_ms: 5,
        }
    }

    fn expect_still_error(
        observation: &GraphicImageWorkerObservationV1,
        request: &GraphicImageInspectRequestV2,
        expected: GraphicImageInspectRefusalKindV2,
    ) {
        match validate_still_observation_v2(observation, request) {
            Ok(_) => panic!("still falsification was accepted: {expected:?}"),
            Err(kind) => assert_eq!(kind, expected),
        }
    }

    fn expect_gif_error(
        evidence: &GifAnimationEvidenceV2,
        request: &GraphicImageInspectRequestV2,
        expected: GraphicImageInspectRefusalKindV2,
    ) {
        match validate_gif_evidence_v2(evidence, request) {
            Ok(_) => panic!("gif falsification was accepted: {expected:?}"),
            Err(kind) => assert_eq!(kind, expected),
        }
    }

    #[test]
    fn n113_01_the_v2_format_enum_stays_closed_and_extension_driven() {
        assert_eq!(GraphicImageFormatV2::ALL.len(), 4);
        // A declaration may be refused by the domain itself (unknown extension, uppercase, platform
        // namespace) before this gate ever sees it; either way it never declares a V2 format.
        let declared = |namespace: AssetNamespace, text: &str| {
            solo_domain::AssetReferenceV1::parse(namespace, text)
                .ok()
                .as_ref()
                .and_then(GraphicImageFormatV2::declared_by)
        };
        assert_eq!(
            declared(AssetNamespace::Entity, "assets/deep/a.jpeg"),
            Some(GraphicImageFormatV2::Jpeg)
        );
        // `SOLO-WO-113 R02` admitted the lowercase `gif` token, so a real authored GIF reference
        // now declares a V2 format through the same extension-only rule as the still formats.
        assert_eq!(
            declared(AssetNamespace::Entity, "assets/a.gif"),
            Some(GraphicImageFormatV2::Gif)
        );
        assert_eq!(
            declared(AssetNamespace::Entity, "assets/a.png"),
            Some(GraphicImageFormatV2::Png)
        );
        assert_eq!(declared(AssetNamespace::Entity, "assets/a.PNG"), None);
        assert_eq!(declared(AssetNamespace::Entity, "assets/a.avif"), None);
        assert_eq!(declared(AssetNamespace::Entity, "assets/a.svg"), None);
        assert_eq!(declared(AssetNamespace::Entity, "assets/a.bin"), None);
        assert_eq!(declared(AssetNamespace::Entity, "assets/a"), None);
        assert_eq!(declared(AssetNamespace::Platform, "assets/a.gif"), None);
        assert_eq!(GraphicImageFormatV2::Gif.canonical_mime(), "image/gif");
        assert_eq!(GraphicImageFormatV2::Gif.as_still_v1(), None);
        assert_eq!(
            GraphicImageFormatV2::Png.as_still_v1(),
            Some(GraphicImageFormatV1::Png)
        );
        // The V1 still enum is untouched: three variants, no GIF.
        assert_eq!(GraphicImageFormatV1::ALL.len(), 3);
    }

    #[test]
    fn n113_02_a_favicon_admits_only_png_in_the_inward_policy() {
        assert!(role_admits_format_v2(
            PropertyKey::GraphicFavicon,
            GraphicImageFormatV2::Png
        ));
        for format in [
            GraphicImageFormatV2::Jpeg,
            GraphicImageFormatV2::WebP,
            GraphicImageFormatV2::Gif,
        ] {
            assert!(
                !role_admits_format_v2(PropertyKey::GraphicFavicon, format),
                "favicon must refuse {format:?}"
            );
        }
        for role in [PropertyKey::GraphicLogo, PropertyKey::GraphicBanner] {
            for format in GraphicImageFormatV2::ALL {
                assert!(role_admits_format_v2(role, format));
            }
        }
    }

    #[test]
    fn n113_03_every_v2_refusal_token_is_unique_and_ascii() {
        let kinds = [
            GraphicImageInspectRefusalKindV2::UnsupportedFormat,
            GraphicImageInspectRefusalKindV2::FaviconNotPng,
            GraphicImageInspectRefusalKindV2::EmptySource,
            GraphicImageInspectRefusalKindV2::OversizeSource,
            GraphicImageInspectRefusalKindV2::PortRefused,
            GraphicImageInspectRefusalKindV2::FormatMismatch,
            GraphicImageInspectRefusalKindV2::MimeMismatch,
            GraphicImageInspectRefusalKindV2::SourceDigestMismatch,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
            GraphicImageInspectRefusalKindV2::DimensionsOutOfRange,
            GraphicImageInspectRefusalKindV2::PixelCountOutOfRange,
            GraphicImageInspectRefusalKindV2::DecodedBytesOutOfRange,
            GraphicImageInspectRefusalKindV2::OutputDigestMismatch,
            GraphicImageInspectRefusalKindV2::GifFrameCountOutOfRange,
            GraphicImageInspectRefusalKindV2::GifFrameGeometryOutOfRange,
            GraphicImageInspectRefusalKindV2::GifTimelineInconsistent,
        ];
        let mut tokens: Vec<&str> = kinds.iter().map(|kind| kind.token()).collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "v2 refusal tokens must be unique");
        assert!(tokens.iter().all(|token| token.is_ascii()));
        const {
            assert!(!GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY);
            assert!(!GRAPHIC_IMAGE_INSPECTION_V2_PROVES_MEDIA_CORRECTNESS);
        }
        assert!(!GraphicImageInspectionSetV2::default().grants_publication_authority());
        assert!(!GraphicImageInspectionSetV2::default().proves_whole_publication());
    }

    #[test]
    fn n113_04_a_forged_still_observation_is_refused_by_the_real_validator() {
        let bytes = b"still-source-bytes";
        let png = request(bytes, GraphicImageFormatV2::Png);
        let good = still_observation(&png, 4, 3);
        let checked = validate_still_observation_v2(&good, &png).expect("valid still observation");
        assert_eq!((checked.0, checked.1, checked.2), (4, 3, 12));
        assert_eq!(checked.3.decoded_bytes, 48);

        // Wrong still format for the declared one.
        let mut wrong_format = good.clone();
        wrong_format.confirmed_format = GraphicImageFormatV1::Jpeg;
        expect_still_error(
            &wrong_format,
            &png,
            GraphicImageInspectRefusalKindV2::FormatMismatch,
        );

        // Wrong canonical MIME.
        let mut wrong_mime = good.clone();
        wrong_mime.observed_mime = IMAGE_MIME_WEBP_V1;
        expect_still_error(
            &wrong_mime,
            &png,
            GraphicImageInspectRefusalKindV2::MimeMismatch,
        );

        // Forged source digest.
        let mut wrong_source = good.clone();
        wrong_source.source_digest = sha256_media_worker_bytes_v1(b"other");
        expect_still_error(
            &wrong_source,
            &png,
            GraphicImageInspectRefusalKindV2::SourceDigestMismatch,
        );

        // Unproved confinement, unproved reap and an impossible zero child identity.
        let mut unproved = good.clone();
        unproved.limits_verified = false;
        expect_still_error(
            &unproved,
            &png,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
        );
        let mut unproved = good.clone();
        unproved.cleanup_verified = false;
        expect_still_error(
            &unproved,
            &png,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
        );
        let mut no_pid = good.clone();
        no_pid.worker_pid = 0;
        expect_still_error(
            &no_pid,
            &png,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
        );

        // Zero side and an over-wide side.
        let zero = still_observation(&png, 0, 3);
        expect_still_error(
            &zero,
            &png,
            GraphicImageInspectRefusalKindV2::DimensionsOutOfRange,
        );
        let wide = still_observation(&png, 8193, 3);
        expect_still_error(
            &wide,
            &png,
            GraphicImageInspectRefusalKindV2::DimensionsOutOfRange,
        );

        // Intrinsic pixel product above the 32-million ceiling.
        let oversized = still_observation(&png, 8192, 8192);
        expect_still_error(
            &oversized,
            &png,
            GraphicImageInspectRefusalKindV2::PixelCountOutOfRange,
        );

        // Decoded length inconsistent with the confirmed geometry.
        let mut wrong_decoded = good.clone();
        wrong_decoded.decoded_bytes = 47;
        expect_still_error(
            &wrong_decoded,
            &png,
            GraphicImageInspectRefusalKindV2::DecodedBytesOutOfRange,
        );

        // An all-zero digest can never be a real payload digest.
        let mut null_digest = good.clone();
        null_digest.decoded_digest = MediaWorkerDigestV1::from_bytes([0u8; 32]);
        expect_still_error(
            &null_digest,
            &png,
            GraphicImageInspectRefusalKindV2::OutputDigestMismatch,
        );
    }

    #[test]
    fn n113_05_a_forged_gif_animation_is_refused_by_the_real_validator() {
        let source = b"gif89a-three-frame-source";
        let request = request(source, GraphicImageFormatV2::Gif);
        let frames = vec![frame(0, 10), frame(1, 20), frame(2, 5)];
        let good = animation(&request, frames, 350);
        let checked = validate_gif_evidence_v2(&good, &request).expect("valid animation");
        assert_eq!((checked.0, checked.1, checked.2), (1, 1, 1));

        // Signature and MIME claims.
        let mut wrong_signature = good.clone();
        wrong_signature.signature = "GIF00a";
        expect_gif_error(
            &wrong_signature,
            &request,
            GraphicImageInspectRefusalKindV2::FormatMismatch,
        );
        let mut wrong_mime = good.clone();
        wrong_mime.mime = IMAGE_MIME_PNG_V1;
        expect_gif_error(
            &wrong_mime,
            &request,
            GraphicImageInspectRefusalKindV2::MimeMismatch,
        );

        // Forged source digest and unproved host facts.
        let mut wrong_source = good.clone();
        wrong_source.source_digest = sha256_media_worker_bytes_v1(b"other");
        expect_gif_error(
            &wrong_source,
            &request,
            GraphicImageInspectRefusalKindV2::SourceDigestMismatch,
        );
        let mut unproved = good.clone();
        unproved.limits_verified = false;
        expect_gif_error(
            &unproved,
            &request,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
        );
        let mut no_pid = good.clone();
        no_pid.worker_pid = 0;
        expect_gif_error(
            &no_pid,
            &request,
            GraphicImageInspectRefusalKindV2::UnverifiedWorker,
        );

        // Canvas side and the intrinsic 32-million-pixel ceiling.
        let mut zero_side = good.clone();
        zero_side.width = 0;
        expect_gif_error(
            &zero_side,
            &request,
            GraphicImageInspectRefusalKindV2::DimensionsOutOfRange,
        );
        let mut huge_canvas = good.clone();
        huge_canvas.width = 8192;
        huge_canvas.height = 8192;
        expect_gif_error(
            &huge_canvas,
            &request,
            GraphicImageInspectRefusalKindV2::PixelCountOutOfRange,
        );

        // Frame count, order, rectangle and delay arithmetic.
        let mut none = good.clone();
        none.frames.clear();
        expect_gif_error(
            &none,
            &request,
            GraphicImageInspectRefusalKindV2::GifFrameCountOutOfRange,
        );
        let mut out_of_order = good.clone();
        out_of_order.frames = vec![frame(0, 10), frame(2, 20), frame(1, 5)];
        expect_gif_error(
            &out_of_order,
            &request,
            GraphicImageInspectRefusalKindV2::GifFrameCountOutOfRange,
        );
        let mut outside = good.clone();
        outside.frames = vec![frame(0, 10)];
        outside.frames[0].left = 1;
        outside.frames[0].width = 1;
        expect_gif_error(
            &outside,
            &request,
            GraphicImageInspectRefusalKindV2::GifFrameGeometryOutOfRange,
        );
        let mut zero_frame = good.clone();
        zero_frame.frames = vec![frame(0, 10)];
        zero_frame.frames[0].width = 0;
        expect_gif_error(
            &zero_frame,
            &request,
            GraphicImageInspectRefusalKindV2::GifFrameGeometryOutOfRange,
        );
        let mut wrong_cycle = good.clone();
        wrong_cycle.one_cycle_delay_ms = 351;
        expect_gif_error(
            &wrong_cycle,
            &request,
            GraphicImageInspectRefusalKindV2::GifTimelineInconsistent,
        );
        let mut over_cycle = good.clone();
        over_cycle.frames = (0..6).map(|index| frame(index, u16::MAX)).collect();
        over_cycle.one_cycle_delay_ms = 6 * 655_350;
        over_cycle.total_composited_pixels = 6;
        expect_gif_error(
            &over_cycle,
            &request,
            GraphicImageInspectRefusalKindV2::GifTimelineInconsistent,
        );
        let mut wrong_budget = good.clone();
        wrong_budget.total_composited_pixels = 99;
        expect_gif_error(
            &wrong_budget,
            &request,
            GraphicImageInspectRefusalKindV2::GifTimelineInconsistent,
        );
        let mut zero_loop = good.clone();
        zero_loop.loop_semantics = GifLoopV2::Finite(0);
        expect_gif_error(
            &zero_loop,
            &request,
            GraphicImageInspectRefusalKindV2::GifTimelineInconsistent,
        );
        let mut no_transcript = good.clone();
        no_transcript.transcript_len = 0;
        expect_gif_error(
            &no_transcript,
            &request,
            GraphicImageInspectRefusalKindV2::GifFrameCountOutOfRange,
        );
        let mut null_pixel = good.clone();
        null_pixel.frames = vec![frame(0, 10)];
        null_pixel.frames[0].composite_digest = MediaWorkerDigestV1::from_bytes([0u8; 32]);
        expect_gif_error(
            &null_pixel,
            &request,
            GraphicImageInspectRefusalKindV2::OutputDigestMismatch,
        );
    }
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-116 R01` — the additive V3 SVG policy surface.
//
// These constants belong to the inward-owned V3 contract. They are declared here, next to the V1
// and V2 policy, so the confined adapter's codec bounds can never drift from the parent policy that
// enforces them. They grant no publication authority and admit no new format on their own.
// ---------------------------------------------------------------------------------------------

/// The canonical MIME of a confirmed SVG vector document.
pub const IMAGE_MIME_SVG_V3: &str = "image/svg+xml";

/// The canonical MIME of an AVIF image. AVIF is **refused** by the V3 policy in this order.
pub const IMAGE_MIME_AVIF_V3: &str = "image/avif";

/// Maximum accepted original SVG source, matching the approved 20 MiB media ceiling.
pub const MAX_SVG_SOURCE_BYTES_V3: u64 = MAX_IMAGE_SOURCE_BYTES_V1;

/// Maximum accepted canonical SVG output: 4 MiB of vector text.
pub const MAX_SVG_CANONICAL_BYTES_V3: usize = 4 * 1024 * 1024;

/// Maximum accepted number of XML elements in one SVG document.
pub const MAX_SVG_ELEMENTS_V3: usize = 4096;

/// Maximum accepted SVG element nesting depth.
pub const MAX_SVG_NESTING_V3: usize = 32;

/// Maximum accepted attributes on one SVG element.
pub const MAX_SVG_ATTRS_PER_ELEMENT_V3: usize = 32;

/// Maximum accepted SVG `id`/`url(#id)`/`href="#id"` reference count.
pub const MAX_SVG_REFERENCES_V3: usize = 1024;

/// Maximum accepted aggregate SVG path-command count.
pub const MAX_SVG_PATH_COMMANDS_V3: usize = 40000;

/// Maximum accepted number of SVG `<filter>` elements.
pub const MAX_SVG_FILTER_ELEMENTS_V3: usize = 64;

/// Maximum accepted primitive operations inside one SVG `<filter>`.
pub const MAX_SVG_FILTER_OPERATIONS_V3: usize = 8;

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-116 R02` — the additive V3 five-format inspection contract.
//
// V3 admits exactly PNG, JPEG, WebP, GIF and SVG as inspectable formats. AVIF is syntactically
// admitted by the reference grammar so it can be *named*, but every AVIF obligation is refused with
// one typed, attributable refusal before any worker is started. V1 (three formats) and V2 (four
// formats, GIF animation) remain byte-compatible and unchanged.
// ---------------------------------------------------------------------------------------------

/// Whether a V3 inspection result authorizes publication. Always `false`.
pub const GRAPHIC_IMAGE_INSPECTION_V3_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether a V3 inspection result proves whole-site media correctness. Always `false`.
pub const GRAPHIC_IMAGE_INSPECTION_V3_PROVES_MEDIA_CORRECTNESS: bool = false;

/// The canonical MIME of a confirmed GIF under the V3 contract.
pub const IMAGE_MIME_GIF_V3: &str = "image/gif";

/// One format the V3 contract can classify.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicImageFormatV3 {
    /// Portable Network Graphics.
    Png,
    /// JPEG.
    Jpeg,
    /// WebP.
    WebP,
    /// GIF, still or animated.
    Gif,
    /// Safe static true-vector SVG.
    Svg,
    /// AVIF. Never inspected under this order: always refused.
    Avif,
}

impl GraphicImageFormatV3 {
    /// Every format the reference grammar can name, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Png,
        Self::Jpeg,
        Self::WebP,
        Self::Gif,
        Self::Svg,
        Self::Avif,
    ];

    /// The format an authored `.`-extension declares, or `None` for any other reference.
    ///
    /// The decision is textual and comes from the reference, never from file content, and a
    /// non-Entity namespace is never admitted.
    #[must_use]
    pub fn declared_by(reference: &solo_domain::AssetReferenceV1) -> Option<Self> {
        if reference.namespace() != AssetNamespace::Entity {
            return None;
        }
        let text = reference.reference();
        let segment = text.rsplit('/').next().unwrap_or(text);
        let (_, extension) = segment.rsplit_once('.')?;
        if extension.is_empty() {
            return None;
        }
        Self::ALL
            .into_iter()
            .find(|format| format.accepted_extensions().contains(&extension))
    }

    /// The accepted authored extensions of this format. All are lowercase.
    #[must_use]
    pub const fn accepted_extensions(self) -> &'static [&'static str] {
        match self {
            Self::Png => &["png"],
            Self::Jpeg => &["jpg", "jpeg"],
            Self::WebP => &["webp"],
            Self::Gif => &["gif"],
            Self::Svg => &["svg"],
            Self::Avif => &["avif"],
        }
    }

    /// The canonical MIME type of this format.
    #[must_use]
    pub const fn canonical_mime(self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::WebP => "image/webp",
            Self::Gif => IMAGE_MIME_GIF_V3,
            Self::Svg => IMAGE_MIME_SVG_V3,
            Self::Avif => IMAGE_MIME_AVIF_V3,
        }
    }

    /// Whether this format is GIF.
    #[must_use]
    pub const fn is_gif(self) -> bool {
        matches!(self, Self::Gif)
    }

    /// Whether this format is SVG.
    #[must_use]
    pub const fn is_svg(self) -> bool {
        matches!(self, Self::Svg)
    }

    /// Whether this format is AVIF, which this order always refuses.
    #[must_use]
    pub const fn is_avif(self) -> bool {
        matches!(self, Self::Avif)
    }
}

/// One V3 inspection request: the exact bytes plus the authored format declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRequestV3 {
    bytes: Vec<u8>,
    format: GraphicImageFormatV3,
    source_digest: MediaWorkerDigestV1,
}

impl GraphicImageInspectRequestV3 {
    /// Builds one request and computes the authoritative source digest at construction.
    #[must_use]
    pub fn of(bytes: Vec<u8>, format: GraphicImageFormatV3) -> Self {
        let source_digest = sha256_media_worker_bytes_v1(&bytes);
        Self {
            bytes,
            format,
            source_digest,
        }
    }

    /// Exactly the bytes to inspect.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The authored format declaration.
    #[must_use]
    pub const fn declared_format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The digest of exactly those bytes.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }
}

/// The still-image half of one V3 result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StillImageEvidenceV3 {
    /// The exact decoded byte count the parent received and hashed.
    pub decoded_bytes: u64,
    /// The parent's digest of exactly those decoded bytes.
    pub decoded_digest: MediaWorkerDigestV1,
}

/// The bounded vector half of one V3 result: real SVG text, never a raster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicVectorEvidenceV3 {
    /// The confirmed intrinsic width in user units.
    pub width: u32,
    /// The confirmed intrinsic height in user units.
    pub height: u32,
    /// The number of retained elements.
    pub elements: usize,
    /// The maximum element depth.
    pub depth: usize,
    /// The number of validated internal references.
    pub references: usize,
    /// The aggregate path-command count.
    pub path_commands: usize,
    /// The number of filters.
    pub filters: usize,
    /// Exactly the canonical SVG text bytes, never rendered.
    pub canonical: Vec<u8>,
    /// The parent's digest of exactly the source bytes it sent.
    pub source_digest: MediaWorkerDigestV1,
    /// The parent's digest of exactly the canonical bytes it received.
    pub output_digest: MediaWorkerDigestV1,
    /// The observed child process identifier.
    pub worker_pid: u32,
    /// The measured wall time of the attempt.
    pub wall_ms: u64,
    /// Whether the supervisor proved the confinement bounds.
    pub limits_verified: bool,
    /// Whether the supervisor proved the reap and cleanup.
    pub cleanup_verified: bool,
}

impl GraphicVectorEvidenceV3 {
    /// The canonical SVG text bytes.
    #[must_use]
    pub fn canonical(&self) -> &[u8] {
        &self.canonical
    }

    /// The canonical byte length.
    #[must_use]
    pub fn canonical_len(&self) -> u64 {
        self.canonical.len() as u64
    }
}

/// One observed V3 result: still plane, GIF timeline, or canonical vector text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageObservationV3 {
    /// A fully decoded still plane.
    Still(GraphicImageWorkerObservationV1),
    /// A complete GIF timeline.
    Animation(GifAnimationEvidenceV2),
    /// A complete canonical SVG vector document.
    Vector(GraphicVectorEvidenceV3),
}

/// The closed result of one V3 port attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectPortOutcomeV3 {
    /// The worker ran and every parent-side check passed.
    Inspected(Box<GraphicImageObservationV3>),
    /// The attempt produced no acceptable observation.
    Refused(GraphicImageInspectRefusalKindV3),
}

/// The inspection port the V3 use case drives.
///
/// Implementations live in the outer adapter. The inward layer never decodes, never parses hostile
/// SVG and never starts a process itself.
pub trait GraphicImageInspectPortV3 {
    /// Inspects exactly one already-supplied request.
    fn inspect_v3(
        &self,
        request: &GraphicImageInspectRequestV3,
    ) -> GraphicImageInspectPortOutcomeV3;
}

/// Every way one V3 obligation can be refused. Each refuses the **whole** candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicImageInspectRefusalKindV3 {
    /// The reference declared no admitted format, or a non-Entity namespace.
    UnsupportedFormat,
    /// The reference declared AVIF, which this order refuses before any worker starts.
    AvifUnsupported,
    /// The registered favicon role is PNG-only and the declaration was not PNG.
    FaviconNotPng,
    /// The supplied source was empty.
    EmptySource,
    /// The supplied source exceeded the accepted ceiling.
    OversizeSource,
    /// The port produced no acceptable observation.
    PortRefused,
    /// The observation class did not match the declared format.
    FormatMismatch,
    /// The confirmed MIME did not match the declared format.
    MimeMismatch,
    /// The observation's source digest did not match the request.
    SourceDigestMismatch,
    /// The worker identity, confinement limits or reap were not proved.
    UnverifiedWorker,
    /// The confirmed intrinsic geometry was outside the accepted bounds.
    DimensionsOutOfRange,
    /// `width x height` exceeded the accepted pixel ceiling.
    PixelCountOutOfRange,
    /// A decoded still plane exceeded the accepted decoded ceiling.
    DecodedBytesOutOfRange,
    /// A vector document exceeded the canonical ceiling.
    SvgOutputOutOfRange,
    /// The vector facts were internally inconsistent or out of budget.
    SvgFactsInconsistent,
    /// A GIF timeline was internally inconsistent.
    GifTimelineInconsistent,
}

impl GraphicImageInspectRefusalKindV3 {
    /// The stable machine-readable token of this refusal.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::UnsupportedFormat => "unsupported-format-v3",
            Self::AvifUnsupported => "avif-unsupported-v3",
            Self::FaviconNotPng => "favicon-not-png-v3",
            Self::EmptySource => "empty-source-v3",
            Self::OversizeSource => "oversize-source-v3",
            Self::PortRefused => "port-refused-v3",
            Self::FormatMismatch => "format-mismatch-v3",
            Self::MimeMismatch => "mime-mismatch-v3",
            Self::SourceDigestMismatch => "source-digest-mismatch-v3",
            Self::UnverifiedWorker => "unverified-worker-v3",
            Self::DimensionsOutOfRange => "dimensions-out-of-range-v3",
            Self::PixelCountOutOfRange => "pixel-count-out-of-range-v3",
            Self::DecodedBytesOutOfRange => "decoded-bytes-out-of-range-v3",
            Self::SvgOutputOutOfRange => "svg-output-out-of-range-v3",
            Self::SvgFactsInconsistent => "svg-facts-inconsistent-v3",
            Self::GifTimelineInconsistent => "gif-timeline-inconsistent-v3",
        }
    }
}

/// One typed V3 refusal with the exact obligation that caused it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRefusalV3 {
    kind: GraphicImageInspectRefusalKindV3,
    obligation: GraphicAssetObligationV1,
}

impl GraphicImageInspectRefusalV3 {
    /// The closed refusal kind.
    #[must_use]
    pub const fn kind(&self) -> GraphicImageInspectRefusalKindV3 {
        self.kind
    }

    /// The stable token of the kind.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.kind.token()
    }

    /// The authored obligation this refusal answers.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The qualified owner of the refusing obligation.
    #[must_use]
    pub const fn owner(&self) -> &solo_domain::QualifiedOwnerV1 {
        self.obligation.declaring_owner()
    }
}

/// Builds one attributable refusal for the obligation that caused it.
fn refuse_v3(
    kind: GraphicImageInspectRefusalKindV3,
    obligation: &GraphicAssetObligationV1,
) -> GraphicImageInspectionOutcomeV3 {
    GraphicImageInspectionOutcomeV3::Refused(GraphicImageInspectRefusalV3 {
        kind,
        obligation: obligation.clone(),
    })
}

/// The typed payload of one inspected V3 image: still plane, GIF timeline or vector text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImagePayloadV3 {
    /// A decoded still plane.
    Still(StillImageEvidenceV3),
    /// A GIF animation or still GIF.
    Animation(GifAnimationEvidenceV2),
    /// A canonical SVG vector document.
    Vector(GraphicVectorEvidenceV3),
}

/// One inspected image of a V3 candidate. Still inert: it authorizes nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectionV3 {
    obligation: GraphicAssetObligationV1,
    logical_path: String,
    format: GraphicImageFormatV3,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV3,
    worker_pid: u32,
    wall_ms: u64,
}

impl GraphicImageInspectionV3 {
    /// The authored obligation this result answers, unchanged.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The confined logical path the source bytes came from.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The confirmed format.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The canonical MIME of the confirmed format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The SHA-256 computed over exactly the source bytes sent.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The confirmed intrinsic width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The confirmed intrinsic height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The confirmed `width x height` product.
    #[must_use]
    pub const fn pixels(&self) -> u64 {
        self.pixels
    }

    /// The typed payload: exactly one of the three result classes.
    #[must_use]
    pub const fn payload(&self) -> &GraphicImagePayloadV3 {
        &self.payload
    }

    /// The GIF evidence, when this result is an animation.
    #[must_use]
    pub const fn animation(&self) -> Option<&GifAnimationEvidenceV2> {
        match &self.payload {
            GraphicImagePayloadV3::Animation(evidence) => Some(evidence),
            _ => None,
        }
    }

    /// The vector evidence, when this result is an SVG document.
    #[must_use]
    pub const fn vector(&self) -> Option<&GraphicVectorEvidenceV3> {
        match &self.payload {
            GraphicImagePayloadV3::Vector(evidence) => Some(evidence),
            _ => None,
        }
    }

    /// Whether this result is a raster payload.
    #[must_use]
    pub const fn is_raster(&self) -> bool {
        !matches!(self.payload, GraphicImagePayloadV3::Vector(_))
    }

    /// The worker process identifier the parent observed.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The measured wall time of this attempt.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this result authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V3_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// The complete inspected set of one whole V3 candidate. Complete or absent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicImageInspectionSetV3 {
    inspections: Vec<GraphicImageInspectionV3>,
}

impl GraphicImageInspectionSetV3 {
    /// Every inspection, in inventory order.
    #[must_use]
    pub fn inspections(&self) -> &[GraphicImageInspectionV3] {
        &self.inspections
    }

    /// How many independently declared obligations were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inspections.len()
    }

    /// Whether the candidate declared no graphic obligation at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inspections.is_empty()
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V3_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this set is a global publishable snapshot. Always `false`.
    #[must_use]
    pub const fn proves_whole_publication(&self) -> bool {
        false
    }
}

/// The closed result of one whole-candidate V3 inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectionOutcomeV3 {
    /// Every declared obligation was inspected.
    Complete(GraphicImageInspectionSetV3),
    /// Exactly one typed refusal; no partial set exists.
    Refused(GraphicImageInspectRefusalV3),
}

/// Whether one registered property role admits this V3 format.
///
/// `graphic.favicon` stays **PNG-only**, so an `icon.svg` favicon is refused even though the
/// grammar admits the `.svg` extension. The role, never the filename or an inference, decides.
#[must_use]
pub const fn role_admits_format_v3(role: PropertyKey, format: GraphicImageFormatV3) -> bool {
    match role {
        PropertyKey::GraphicFavicon => matches!(format, GraphicImageFormatV3::Png),
        _ => true,
    }
}

/// Whether one authored side is inside the accepted intrinsic bounds.
fn side_in_bounds_v3(side: u32) -> bool {
    (1..=MAX_IMAGE_SIDE_V1).contains(&side)
}

/// Validates one still observation against the request that produced it.
fn validate_still_v3(
    observation: &GraphicImageWorkerObservationV1,
    request: &GraphicImageInspectRequestV3,
) -> Result<(u32, u32, u64, StillImageEvidenceV3), GraphicImageInspectRefusalKindV3> {
    use GraphicImageInspectRefusalKindV3 as Kind;
    let expected = match request.declared_format() {
        GraphicImageFormatV3::Png => GraphicImageFormatV1::Png,
        GraphicImageFormatV3::Jpeg => GraphicImageFormatV1::Jpeg,
        GraphicImageFormatV3::WebP => GraphicImageFormatV1::WebP,
        _ => return Err(Kind::FormatMismatch),
    };
    if observation.confirmed_format != expected
        || observation.observed_mime != request.declared_format().canonical_mime()
    {
        return Err(Kind::FormatMismatch);
    }
    if &observation.source_digest != request.source_digest() {
        return Err(Kind::SourceDigestMismatch);
    }
    if observation.worker_pid == 0 || !observation.limits_verified || !observation.cleanup_verified
    {
        return Err(Kind::UnverifiedWorker);
    }
    if !side_in_bounds_v3(observation.width) || !side_in_bounds_v3(observation.height) {
        return Err(Kind::DimensionsOutOfRange);
    }
    let pixels = u64::from(observation.width) * u64::from(observation.height);
    if pixels == 0 || pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(Kind::PixelCountOutOfRange);
    }
    if observation.decoded_bytes == 0 || observation.decoded_bytes > MAX_IMAGE_DECODED_BYTES_V1 {
        return Err(Kind::DecodedBytesOutOfRange);
    }
    if observation.decoded_digest.as_bytes() == &[0u8; 32] {
        return Err(Kind::DecodedBytesOutOfRange);
    }
    Ok((
        observation.width,
        observation.height,
        pixels,
        StillImageEvidenceV3 {
            decoded_bytes: observation.decoded_bytes,
            decoded_digest: observation.decoded_digest,
        },
    ))
}

/// Validates one GIF timeline against the request that produced it.
fn validate_animation_v3(
    evidence: &GifAnimationEvidenceV2,
    request: &GraphicImageInspectRequestV3,
) -> Result<(u32, u32, u64), GraphicImageInspectRefusalKindV3> {
    use GraphicImageInspectRefusalKindV3 as Kind;
    if evidence.mime != request.declared_format().canonical_mime() {
        return Err(Kind::MimeMismatch);
    }
    if &evidence.source_digest != request.source_digest() {
        return Err(Kind::SourceDigestMismatch);
    }
    if evidence.worker_pid == 0 || !evidence.limits_verified || !evidence.cleanup_verified {
        return Err(Kind::UnverifiedWorker);
    }
    let width = u32::from(evidence.width);
    let height = u32::from(evidence.height);
    if !side_in_bounds_v3(width) || !side_in_bounds_v3(height) {
        return Err(Kind::DimensionsOutOfRange);
    }
    let pixels = u64::from(width) * u64::from(height);
    if pixels == 0 || pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(Kind::PixelCountOutOfRange);
    }
    if evidence.frames.is_empty() {
        return Err(Kind::GifTimelineInconsistent);
    }
    Ok((width, height, pixels))
}

/// Validates one vector observation against the request that produced it.
fn validate_vector_v3(
    evidence: &GraphicVectorEvidenceV3,
    request: &GraphicImageInspectRequestV3,
) -> Result<(u32, u32, u64), GraphicImageInspectRefusalKindV3> {
    use GraphicImageInspectRefusalKindV3 as Kind;
    if !request.declared_format().is_svg() {
        return Err(Kind::FormatMismatch);
    }
    if &evidence.source_digest != request.source_digest() {
        return Err(Kind::SourceDigestMismatch);
    }
    if evidence.worker_pid == 0 || !evidence.limits_verified || !evidence.cleanup_verified {
        return Err(Kind::UnverifiedWorker);
    }
    if !side_in_bounds_v3(evidence.width) || !side_in_bounds_v3(evidence.height) {
        return Err(Kind::DimensionsOutOfRange);
    }
    let pixels = u64::from(evidence.width) * u64::from(evidence.height);
    if pixels == 0 || pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(Kind::PixelCountOutOfRange);
    }
    if evidence.canonical.is_empty() || evidence.canonical_len() > MAX_SVG_CANONICAL_BYTES_V3 as u64
    {
        return Err(Kind::SvgOutputOutOfRange);
    }
    if evidence.output_digest.as_bytes() == &[0u8; 32]
        || evidence.output_digest != sha256_media_worker_bytes_v1(&evidence.canonical)
    {
        return Err(Kind::SvgFactsInconsistent);
    }
    if evidence.elements == 0
        || evidence.elements > MAX_SVG_ELEMENTS_V3
        || evidence.depth == 0
        || evidence.depth > MAX_SVG_NESTING_V3
        || evidence.references > MAX_SVG_REFERENCES_V3
        || evidence.path_commands > MAX_SVG_PATH_COMMANDS_V3
        || evidence.filters > MAX_SVG_FILTER_ELEMENTS_V3
    {
        return Err(Kind::SvgFactsInconsistent);
    }
    Ok((evidence.width, evidence.height, pixels))
}

/// Inspects one whole V3 candidate: every declared obligation, or one attributable refusal.
///
/// The order is fixed and each step precedes the next: format classification, then the AVIF
/// refusal (before any worker), then the registered role policy, then source bounds, then one port
/// attempt, then independent parent validation. A single refusal discards every earlier success, so
/// there is never a usable partial set.
#[must_use]
pub fn inspect_graphic_image_source_set_v3(
    source_set: &GraphicAssetSourceSetV1,
    port: &impl GraphicImageInspectPortV3,
) -> GraphicImageInspectionOutcomeV3 {
    let evidence = source_set.evidence();
    let mut inspections: Vec<GraphicImageInspectionV3> = Vec::with_capacity(evidence.len());

    for item in evidence {
        let obligation = item.obligation();
        let Some(format) = GraphicImageFormatV3::declared_by(obligation.reference()) else {
            return refuse_v3(
                GraphicImageInspectRefusalKindV3::UnsupportedFormat,
                obligation,
            );
        };
        // AVIF is named by the grammar and refused here, before any worker or decode exists.
        if format.is_avif() {
            return refuse_v3(
                GraphicImageInspectRefusalKindV3::AvifUnsupported,
                obligation,
            );
        }
        if !role_admits_format_v3(obligation.role(), format) {
            return refuse_v3(GraphicImageInspectRefusalKindV3::FaviconNotPng, obligation);
        }

        let bytes = item.bytes();
        let length = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if bytes.is_empty() {
            return refuse_v3(GraphicImageInspectRefusalKindV3::EmptySource, obligation);
        }
        if length > MAX_IMAGE_SOURCE_BYTES_V1 {
            return refuse_v3(GraphicImageInspectRefusalKindV3::OversizeSource, obligation);
        }

        let request = GraphicImageInspectRequestV3::of(bytes.to_vec(), format);
        let observation = match port.inspect_v3(&request) {
            GraphicImageInspectPortOutcomeV3::Inspected(observation) => *observation,
            GraphicImageInspectPortOutcomeV3::Refused(kind) => {
                return refuse_v3(kind, obligation);
            }
        };

        let (width, height, pixels, payload, worker_pid, wall_ms) = match observation {
            GraphicImageObservationV3::Still(observation) => {
                if format.is_gif() || format.is_svg() {
                    return refuse_v3(GraphicImageInspectRefusalKindV3::FormatMismatch, obligation);
                }
                let (width, height, pixels, still) = match validate_still_v3(&observation, &request)
                {
                    Ok(checked) => checked,
                    Err(kind) => return refuse_v3(kind, obligation),
                };
                let worker_pid = observation.worker_pid;
                let wall_ms = observation.wall_ms;
                (
                    width,
                    height,
                    pixels,
                    GraphicImagePayloadV3::Still(still),
                    worker_pid,
                    wall_ms,
                )
            }
            GraphicImageObservationV3::Animation(evidence) => {
                if !format.is_gif() {
                    return refuse_v3(GraphicImageInspectRefusalKindV3::FormatMismatch, obligation);
                }
                let (width, height, pixels) = match validate_animation_v3(&evidence, &request) {
                    Ok(checked) => checked,
                    Err(kind) => return refuse_v3(kind, obligation),
                };
                let worker_pid = evidence.worker_pid;
                let wall_ms = evidence.wall_ms;
                (
                    width,
                    height,
                    pixels,
                    GraphicImagePayloadV3::Animation(evidence),
                    worker_pid,
                    wall_ms,
                )
            }
            GraphicImageObservationV3::Vector(evidence) => {
                if !format.is_svg() {
                    return refuse_v3(GraphicImageInspectRefusalKindV3::FormatMismatch, obligation);
                }
                let (width, height, pixels) = match validate_vector_v3(&evidence, &request) {
                    Ok(checked) => checked,
                    Err(kind) => return refuse_v3(kind, obligation),
                };
                let worker_pid = evidence.worker_pid;
                let wall_ms = evidence.wall_ms;
                (
                    width,
                    height,
                    pixels,
                    GraphicImagePayloadV3::Vector(evidence),
                    worker_pid,
                    wall_ms,
                )
            }
        };

        inspections.push(GraphicImageInspectionV3 {
            obligation: obligation.clone(),
            logical_path: item.logical_path().to_owned(),
            format,
            mime: format.canonical_mime(),
            source_digest: *request.source_digest(),
            width,
            height,
            pixels,
            payload,
            worker_pid,
            wall_ms,
        });
    }

    GraphicImageInspectionOutcomeV3::Complete(GraphicImageInspectionSetV3 { inspections })
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-118 R11` — the V4 whole-candidate six-format composition.
//
// V4 adds **no** decoding rule of its own. It iterates the acquired obligation set once, in order,
// and dispatches each declared format either to the already-accepted V3 five-format port or to the
// R10 confined AVIF V1 port plus the existing trusted-parent check. The V3 entrypoints, their
// discriminants, tokens and the historic `avif-unsupported-v3` refusal are deliberately unchanged:
// the AVIF arm exists only here.
// ---------------------------------------------------------------------------------------------

/// Whether a V4 inspection set authorizes publication. Always `false`.
pub const GRAPHIC_IMAGE_INSPECTION_V4_GRANTS_PUBLICATION_AUTHORITY: bool = false;

/// Whether a V4 inspection set proves the whole publication. Always `false`.
pub const GRAPHIC_IMAGE_INSPECTION_V4_PROVES_WHOLE_PUBLICATION: bool = false;

/// One AVIF plane exactly as the trusted parent received and re-hashed it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvifPlaneEvidenceV4 {
    ordinal: u32,
    stride: u64,
    rows: u64,
    length: u64,
    sha256: MediaWorkerDigestV1,
}

impl AvifPlaneEvidenceV4 {
    /// The plane ordinal, `0..plane_count`.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// The claimed canonical stride the parent re-derived.
    #[must_use]
    pub const fn stride(&self) -> u64 {
        self.stride
    }

    /// The claimed canonical row count the parent re-derived.
    #[must_use]
    pub const fn rows(&self) -> u64 {
        self.rows
    }

    /// The claimed byte length the parent verified against its own buffer.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// The parent's own digest of exactly those received bytes.
    #[must_use]
    pub const fn sha256(&self) -> MediaWorkerDigestV1 {
        self.sha256
    }
}

/// The parent-checked AVIF half of one V4 record.
///
/// Everything here is what the trusted parent itself verified: the closed V1 envelope, its own
/// source and plane digests, its own aggregate framing, and the worker's orientation **claim**.
/// None of it proves image correctness, display semantics or publication authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvifEvidenceV4 {
    checked: MediaWorkerCheckedEnvelopeV1,
    orientation: MediaWorkerOrientationV1,
    planes: Vec<AvifPlaneEvidenceV4>,
    wall_ms: u64,
}

impl AvifEvidenceV4 {
    /// The closed, parent-checked V1 envelope.
    #[must_use]
    pub const fn checked(&self) -> &MediaWorkerCheckedEnvelopeV1 {
        &self.checked
    }

    /// The orientation the worker claimed; `Identity` is only reachable for the one accepted
    /// four-plane YUVA class.
    #[must_use]
    pub const fn orientation(&self) -> MediaWorkerOrientationV1 {
        self.orientation
    }

    /// Every transported plane, in order.
    #[must_use]
    pub fn planes(&self) -> &[AvifPlaneEvidenceV4] {
        &self.planes
    }

    /// The observed wall time of the confined child.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this evidence authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }
}

/// The typed payload of one V4 record: retained V3 evidence or a parent-checked AVIF envelope.
///
/// The union is deliberately closed and asymmetric: a GIF timeline, a canonical SVG text document,
/// a decoded still plane and a four-plane YUVA envelope are different facts and are never flattened
/// into one invented RGBA shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImagePayloadV4 {
    /// The unchanged V3 still, GIF or vector evidence.
    Legacy(GraphicImagePayloadV3),
    /// A parent-checked AVIF envelope with its transported planes.
    Avif(AvifEvidenceV4),
}

/// Every way one V4 obligation can be refused. Each refuses the **whole** candidate.
///
/// The legacy arm preserves the accepted V3 category verbatim, so a caller can still tell
/// `unsupported-format-v3` from `favicon-not-png-v3`; the media arm preserves the accepted V1
/// parent category, so a transport refusal is never flattened into a decode failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicImageInspectRefusalKindV4 {
    /// The unchanged V3 category, including its historic AVIF refusal for the V3 API itself.
    Legacy(GraphicImageInspectRefusalKindV3),
    /// The accepted V1 trusted-parent category for the confined AVIF worker.
    Media(MediaWorkerRefusalKindV1),
}

impl GraphicImageInspectRefusalKindV4 {
    /// The stable machine-readable token of this refusal.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Legacy(kind) => kind.token(),
            Self::Media(kind) => kind.code(),
        }
    }

    /// Whether this refusal came from the V3 five-format path.
    #[must_use]
    pub const fn is_legacy(self) -> bool {
        matches!(self, Self::Legacy(_))
    }

    /// Whether this refusal came from the confined AVIF V1 path.
    #[must_use]
    pub const fn is_media(self) -> bool {
        matches!(self, Self::Media(_))
    }
}

/// One typed V4 refusal with the exact obligation that caused it.
///
/// The obligation is retained for **diagnosis inside the process**; its rendering is deliberately
/// not part of any diagnostic string. [`Self::token`] is the only public diagnostic, and it carries
/// no path, reference, buffer, worker identifier or declaration text.
#[derive(Clone, PartialEq, Eq)]
pub struct GraphicImageInspectRefusalV4 {
    kind: GraphicImageInspectRefusalKindV4,
    obligation: GraphicAssetObligationV1,
}

impl GraphicImageInspectRefusalV4 {
    /// The closed refusal kind.
    #[must_use]
    pub const fn kind(&self) -> GraphicImageInspectRefusalKindV4 {
        self.kind
    }

    /// The stable token of the kind; the only public diagnostic text.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        self.kind.token()
    }

    /// The authored obligation this refusal answers, for internal attribution.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The qualified owner of the refusing obligation.
    #[must_use]
    pub const fn owner(&self) -> &solo_domain::QualifiedOwnerV1 {
        self.obligation.declaring_owner()
    }
}

impl std::fmt::Debug for GraphicImageInspectRefusalV4 {
    /// Renders only the closed category token: no path, reference, owner text or worker value.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GraphicImageInspectRefusalV4")
            .field("token", &self.kind.token())
            .finish()
    }
}

/// Builds one attributable V4 refusal for the obligation that caused it.
fn refuse_v4(
    kind: GraphicImageInspectRefusalKindV4,
    obligation: &GraphicAssetObligationV1,
) -> GraphicImageInspectionOutcomeV4 {
    GraphicImageInspectionOutcomeV4::Refused(GraphicImageInspectRefusalV4 {
        kind,
        obligation: obligation.clone(),
    })
}

/// One inspected image of a V4 candidate. Still inert: it authorizes nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicImageInspectionV4 {
    obligation: GraphicAssetObligationV1,
    logical_path: String,
    format: GraphicImageFormatV3,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV4,
    worker_pid: u32,
    wall_ms: u64,
}

impl GraphicImageInspectionV4 {
    /// The authored obligation this result answers, unchanged.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The confined logical path the source bytes came from.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The confirmed format, from the closed authored extension grammar.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The canonical MIME of that format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The parent's own digest of the acquired source bytes.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The confirmed intrinsic or displayed width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The confirmed intrinsic or displayed height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// `width × height` under checked arithmetic.
    #[must_use]
    pub const fn pixels(&self) -> u64 {
        self.pixels
    }

    /// The typed payload: retained V3 evidence or a parent-checked AVIF envelope.
    #[must_use]
    pub const fn payload(&self) -> &GraphicImagePayloadV4 {
        &self.payload
    }

    /// The parent-checked AVIF evidence, when this record is the AVIF arm.
    #[must_use]
    pub const fn avif(&self) -> Option<&AvifEvidenceV4> {
        match &self.payload {
            GraphicImagePayloadV4::Avif(evidence) => Some(evidence),
            GraphicImagePayloadV4::Legacy(_) => None,
        }
    }

    /// The orientation **claim** this record carries, if it carries one at all.
    ///
    /// Only the confined AVIF arm can make such a claim; the retained V3 still/GIF/SVG evidence has
    /// no orientation field, which is why this is `None` rather than an invented value.
    #[must_use]
    pub const fn orientation_claim(&self) -> Option<MediaWorkerOrientationV1> {
        match &self.payload {
            GraphicImagePayloadV4::Avif(evidence) => Some(evidence.orientation),
            GraphicImagePayloadV4::Legacy(_) => None,
        }
    }

    /// The real child process that produced this record.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The observed wall time of that child.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this record authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        false
    }
}

/// The complete inspected set of one whole V4 candidate. Complete or absent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicImageInspectionSetV4 {
    inspections: Vec<GraphicImageInspectionV4>,
}

impl GraphicImageInspectionSetV4 {
    /// Every inspection, in the original inventory order.
    #[must_use]
    pub fn inspections(&self) -> &[GraphicImageInspectionV4] {
        &self.inspections
    }

    /// How many independently declared obligations were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inspections.len()
    }

    /// Whether the candidate declared no graphic obligation at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inspections.is_empty()
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V4_GRANTS_PUBLICATION_AUTHORITY
    }

    /// Whether this set is a global publishable snapshot. Always `false`.
    #[must_use]
    pub const fn proves_whole_publication(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_V4_PROVES_WHOLE_PUBLICATION
    }
}

/// The closed result of one whole-candidate V4 inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphicImageInspectionOutcomeV4 {
    /// Every declared obligation was inspected.
    Complete(GraphicImageInspectionSetV4),
    /// Exactly one typed refusal; no partial set exists or can be recovered.
    Refused(GraphicImageInspectRefusalV4),
}

/// The checked facts of one retained V3 record, as V4 needs them.
struct LegacyCheckedV4 {
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV3,
    worker_pid: u32,
    wall_ms: u64,
}

/// Runs exactly one accepted V3 attempt for a non-AVIF declaration and validates its observation.
///
/// The sequence mirrors the accepted V3 loop check for check and validation for validation; it is
/// used only by V4, so the V3 entrypoint keeps its own body untouched.
fn inspect_legacy_v4(
    format: GraphicImageFormatV3,
    bytes: &[u8],
    port: &impl GraphicImageInspectPortV3,
) -> Result<(GraphicImageInspectRequestV3, LegacyCheckedV4), GraphicImageInspectRefusalKindV3> {
    use GraphicImageInspectRefusalKindV3 as Kind;
    let request = GraphicImageInspectRequestV3::of(bytes.to_vec(), format);
    let observation = match port.inspect_v3(&request) {
        GraphicImageInspectPortOutcomeV3::Inspected(observation) => *observation,
        GraphicImageInspectPortOutcomeV3::Refused(kind) => return Err(kind),
    };
    let checked = match observation {
        GraphicImageObservationV3::Still(observation) => {
            if format.is_gif() || format.is_svg() {
                return Err(Kind::FormatMismatch);
            }
            let (width, height, pixels, still) = validate_still_v3(&observation, &request)?;
            LegacyCheckedV4 {
                width,
                height,
                pixels,
                payload: GraphicImagePayloadV3::Still(still),
                worker_pid: observation.worker_pid,
                wall_ms: observation.wall_ms,
            }
        }
        GraphicImageObservationV3::Animation(evidence) => {
            if !format.is_gif() {
                return Err(Kind::FormatMismatch);
            }
            let (width, height, pixels) = validate_animation_v3(&evidence, &request)?;
            LegacyCheckedV4 {
                width,
                height,
                pixels,
                worker_pid: evidence.worker_pid,
                wall_ms: evidence.wall_ms,
                payload: GraphicImagePayloadV3::Animation(evidence),
            }
        }
        GraphicImageObservationV3::Vector(evidence) => {
            if !format.is_svg() {
                return Err(Kind::FormatMismatch);
            }
            let (width, height, pixels) = validate_vector_v3(&evidence, &request)?;
            LegacyCheckedV4 {
                width,
                height,
                pixels,
                worker_pid: evidence.worker_pid,
                wall_ms: evidence.wall_ms,
                payload: GraphicImagePayloadV3::Vector(evidence),
            }
        }
    };
    Ok((request, checked))
}

/// The checked facts of the one confined-AVIF attempt V4 makes for an AVIF declaration.
struct AvifCheckedV4 {
    checked: MediaWorkerCheckedEnvelopeV1,
    orientation: MediaWorkerOrientationV1,
    planes: Vec<AvifPlaneEvidenceV4>,
    pixels: u64,
}

/// Runs exactly one confined AVIF child over the acquired bytes and the accepted parent check.
///
/// The request, the single child launch and the trusted-parent validation are the accepted R10
/// sequence; V4 only wraps its outcome in the closed V4 category.
fn inspect_avif_v4(
    obligation: &GraphicAssetObligationV1,
    bytes: &[u8],
    avif: &impl MediaWorkerPortV1,
) -> Result<AvifCheckedV4, GraphicImageInspectRefusalKindV4> {
    use GraphicImageInspectRefusalKindV3 as LegacyKind;
    use GraphicImageInspectRefusalKindV4 as Kind;
    let request = match MediaWorkerRequestV1::for_source(bytes) {
        Ok(request) => request,
        Err(MediaWorkerRefusalKindV1::SourceEmpty) => {
            return Err(Kind::Legacy(LegacyKind::EmptySource));
        }
        Err(MediaWorkerRefusalKindV1::SourceOversize) => {
            return Err(Kind::Legacy(LegacyKind::OversizeSource));
        }
        Err(kind) => return Err(Kind::Media(kind)),
    };
    let observation = match avif.inspect_avif(&request) {
        MediaWorkerRunV1::Observed(observation) => observation,
        MediaWorkerRunV1::Refused(kind) => return Err(Kind::Media(kind)),
    };
    let checked = check_observation_v1(obligation, &request, &observation).map_err(Kind::Media)?;
    let envelope = observation.envelope();
    let planes = envelope
        .planes()
        .iter()
        .map(|claim| AvifPlaneEvidenceV4 {
            ordinal: claim.ordinal(),
            stride: claim.stride(),
            rows: claim.rows(),
            length: claim.length(),
            sha256: claim.sha256(),
        })
        .collect::<Vec<AvifPlaneEvidenceV4>>();
    let pixels = u64::from(checked.width()).saturating_mul(u64::from(checked.height()));
    Ok(AvifCheckedV4 {
        checked,
        orientation: envelope.orientation(),
        planes,
        pixels,
    })
}

/// The format one typed reference declares, in either admitted namespace.
///
/// The accepted [`GraphicImageFormatV3::declared_by`] deliberately admits only the Entity namespace
/// because every caller before `SOLO-WO-118 R18` was Entity-only. The mixed composition must
/// classify a `platform` reference with the **same** closed extension grammar and no additional
/// rule, so this helper reuses [`GraphicImageFormatV3::accepted_extensions`] and adds nothing: no
/// media sniffing, no MIME guess, no file access and no relaxed admission.
fn format_of_reference(reference: &AssetReferenceV1) -> Option<GraphicImageFormatV3> {
    let text = reference.reference();
    let segment = text.rsplit('/').next().unwrap_or(text);
    let (_, extension) = segment.rsplit_once('.')?;
    if extension.is_empty() {
        return None;
    }
    GraphicImageFormatV3::ALL
        .into_iter()
        .find(|format| format.accepted_extensions().contains(&extension))
}

/// The checked facts of one declared graphic source, as either V4 composition needs them.
///
/// Extracted from the accepted V4 loop so the Entity-only composition and the R18 mixed
/// composition share **one** implementation of the format, role, bound, AVIF and legacy checks.
/// Nothing about those checks is reordered, relaxed or duplicated.
struct V4ItemFactsV1 {
    format: GraphicImageFormatV3,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV4,
    worker_pid: u32,
    wall_ms: u64,
}

/// Runs the accepted per-item V4 checks over one declared source.
///
/// `avif_carrier` is the authored obligation the parent-checked AVIF envelope records as its own
/// provenance. It is `Some` for every authored obligation and `None` for a root-layer Platform
/// registration, which has no EFS owner; an AVIF root registration is therefore refused rather than
/// given a fabricated carrier record.
fn inspect_item_v4(
    role: PropertyKey,
    reference: &solo_domain::AssetReferenceV1,
    bytes: &[u8],
    avif_carrier: Option<&GraphicAssetObligationV1>,
    legacy: &impl GraphicImageInspectPortV3,
    avif: &impl MediaWorkerPortV1,
) -> Result<V4ItemFactsV1, GraphicImageInspectRefusalKindV4> {
    use GraphicImageInspectRefusalKindV3 as LegacyKind;
    use GraphicImageInspectRefusalKindV4 as Kind;

    let format = match format_of_reference(reference) {
        Some(format) => format,
        None => return Err(Kind::Legacy(LegacyKind::UnsupportedFormat)),
    };
    if !role_admits_format_v3(role, format) {
        return Err(Kind::Legacy(LegacyKind::FaviconNotPng));
    }
    let length = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if bytes.is_empty() {
        return Err(Kind::Legacy(LegacyKind::EmptySource));
    }
    if length > MAX_IMAGE_SOURCE_BYTES_V1 {
        return Err(Kind::Legacy(LegacyKind::OversizeSource));
    }

    if format.is_avif() {
        let carrier = match avif_carrier {
            Some(carrier) => carrier,
            None => return Err(Kind::Legacy(LegacyKind::UnsupportedFormat)),
        };
        let checked = inspect_avif_v4(carrier, bytes, avif)?;
        return Ok(V4ItemFactsV1 {
            format,
            mime: format.canonical_mime(),
            source_digest: checked.checked.source_sha256(),
            width: checked.checked.width(),
            height: checked.checked.height(),
            pixels: checked.pixels,
            payload: GraphicImagePayloadV4::Avif(AvifEvidenceV4 {
                checked: checked.checked,
                orientation: checked.orientation,
                planes: checked.planes,
                wall_ms: 0,
            }),
            // The closed V1 envelope carries no process identifier: the adapter reports the
            // PIDs it actually observed, and none is invented here.
            worker_pid: 0,
            wall_ms: 0,
        });
    }

    let (request, checked) = match inspect_legacy_v4(format, bytes, legacy) {
        Ok(checked) => checked,
        Err(kind) => return Err(Kind::Legacy(kind)),
    };
    Ok(V4ItemFactsV1 {
        format,
        mime: format.canonical_mime(),
        source_digest: *request.source_digest(),
        width: checked.width,
        height: checked.height,
        pixels: checked.pixels,
        payload: GraphicImagePayloadV4::Legacy(checked.payload),
        worker_pid: checked.worker_pid,
        wall_ms: checked.wall_ms,
    })
}

/// Inspects one whole candidate across all six declared formats, once, in one order.
///
/// Each obligation is dispatched on its authored extension alone: PNG/JPEG/WebP/GIF/SVG through the
/// unchanged V3 port, AVIF through the confined V1 port plus the existing trusted-parent check. The
/// first failure returns one attributed refusal and **no** earlier record survives.
#[must_use]
pub fn inspect_graphic_image_source_set_v4(
    source_set: &GraphicAssetSourceSetV1,
    legacy: &impl GraphicImageInspectPortV3,
    avif: &impl MediaWorkerPortV1,
) -> GraphicImageInspectionOutcomeV4 {
    let evidence = source_set.evidence();
    let mut inspections: Vec<GraphicImageInspectionV4> = Vec::with_capacity(evidence.len());

    for item in evidence {
        let obligation = item.obligation();
        let facts = match inspect_item_v4(
            obligation.role(),
            obligation.reference(),
            item.bytes(),
            Some(obligation),
            legacy,
            avif,
        ) {
            Ok(facts) => facts,
            Err(kind) => return refuse_v4(kind, obligation),
        };
        inspections.push(GraphicImageInspectionV4 {
            obligation: obligation.clone(),
            logical_path: item.logical_path().to_owned(),
            format: facts.format,
            mime: facts.mime,
            source_digest: facts.source_digest,
            width: facts.width,
            height: facts.height,
            pixels: facts.pixels,
            payload: facts.payload,
            worker_pid: facts.worker_pid,
            wall_ms: facts.wall_ms,
        });
    }

    GraphicImageInspectionOutcomeV4::Complete(GraphicImageInspectionSetV4 { inspections })
}

// ---------------------------------------------------------------------------
// `SOLO-WO-118 R18` — one mixed Entity/Platform whole-candidate inspection
// ---------------------------------------------------------------------------
//
// This is the composed path the Owner asked for: the accepted R17 cascade decides the governing
// source per role/slot, the accepted Entity reader acquires Entity sources, the accepted Platform
// reader acquires approved Platform sources, and the accepted V4 inspector checks every acquired
// source with the same six-format worker proof. Nothing about the two existing entry points changes:
// `inspect_graphic_image_source_set_v4` stays Entity-only and `read_graphic_asset_bytes_v1` still
// refuses a Platform obligation, so neither legacy refusal can be reported as a Platform read.

/// Whether a successful mixed inspection authorizes publication. Always `false`.
pub const MIXED_GRAPHIC_INSPECTION_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Whether a successful mixed inspection proves a globally consistent public snapshot. Always
/// `false`: the Platform set and the Entity set are separate confined reads, not one snapshot.
pub const MIXED_GRAPHIC_INSPECTION_PROVES_WHOLE_PUBLICATION_V1: bool = false;

/// The closed reason a mixed inspection refused one authored source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MixedGraphicInspectRefusalKindV1 {
    /// One of the accepted per-source media/role/format checks refused this source.
    Image(GraphicImageInspectRefusalKindV4),
    /// A root-layer registration selected an AVIF source. The accepted AVIF envelope records an
    /// authored EFS obligation as its own provenance and a root registration has none, so this
    /// composition refuses rather than fabricate a carrier. No approved `1.0.0` asset is AVIF.
    RootRegistrationAvifUnsupported,
    /// The acquired Platform evidence did not cover an obligation the plan requires, so the
    /// verified approved binding cannot vouch for it.
    BindingDoesNotCoverObligation,
}

impl MixedGraphicInspectRefusalKindV1 {
    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::Image(kind) => kind.token(),
            Self::RootRegistrationAvifUnsupported => "root-registration-avif-unsupported",
            Self::BindingDoesNotCoverObligation => "binding-does-not-cover-obligation",
        }
    }
}

/// One refused mixed inspection, attributed to the exact authored obligation that failed.
///
/// The diagnostic names the namespace, role, slot, declaring owner and authored ordinal only. It
/// carries no bytes, no path, no resolved value and no internal provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MixedGraphicInspectRefusalV1 {
    kind: MixedGraphicInspectRefusalKindV1,
    obligation: Box<MixedGraphicObligationV1>,
}

impl MixedGraphicInspectRefusalV1 {
    /// The closed refusal reason.
    #[must_use]
    pub const fn kind(&self) -> MixedGraphicInspectRefusalKindV1 {
        self.kind
    }

    /// The authored obligation that failed.
    #[must_use]
    pub const fn obligation(&self) -> &MixedGraphicObligationV1 {
        &self.obligation
    }

    /// The stable, safe detail token.
    #[must_use]
    pub const fn detail(&self) -> &'static str {
        self.kind.detail()
    }
}

impl core::fmt::Display for MixedGraphicInspectRefusalV1 {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "mixed-graphic-inspection-refused: {}",
            self.detail()
        )
    }
}

impl std::error::Error for MixedGraphicInspectRefusalV1 {}

/// One inspected source of a mixed candidate.
///
/// It records the exact namespace and logical path the bytes came from, the confirmed format and
/// canonical MIME, the accepted media facts, the real child process that produced them and the
/// authored obligation they answer. It carries no source bytes, no filesystem path, no internal
/// value and no publication authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphicMixedInspectionV1 {
    obligation: MixedGraphicObligationV1,
    namespace: AssetNamespace,
    logical_path: String,
    format: GraphicImageFormatV3,
    mime: &'static str,
    source_digest: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    pixels: u64,
    payload: GraphicImagePayloadV4,
    worker_pid: u32,
    wall_ms: u64,
}

impl GraphicMixedInspectionV1 {
    /// The authored obligation this record answers.
    #[must_use]
    pub const fn obligation(&self) -> &MixedGraphicObligationV1 {
        &self.obligation
    }

    /// The explicit namespace the bytes were read from.
    #[must_use]
    pub const fn namespace(&self) -> AssetNamespace {
        self.namespace
    }

    /// The confined logical path inside that namespace's own root.
    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    /// The confirmed format.
    #[must_use]
    pub const fn format(&self) -> GraphicImageFormatV3 {
        self.format
    }

    /// The canonical MIME of that format.
    #[must_use]
    pub const fn mime(&self) -> &'static str {
        self.mime
    }

    /// The parent's own digest of the acquired source bytes.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The confirmed width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The confirmed height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// `width × height` under checked arithmetic.
    #[must_use]
    pub const fn pixels(&self) -> u64 {
        self.pixels
    }

    /// The typed payload: retained V3 evidence or a parent-checked AVIF envelope.
    #[must_use]
    pub const fn payload(&self) -> &GraphicImagePayloadV4 {
        &self.payload
    }

    /// The real child process that produced this record.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The observed wall time of that child.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this source was read from the Entity namespace.
    #[must_use]
    pub const fn is_entity(&self) -> bool {
        matches!(self.namespace, AssetNamespace::Entity)
    }

    /// Whether this source was read from the Platform namespace.
    #[must_use]
    pub const fn is_platform(&self) -> bool {
        matches!(self.namespace, AssetNamespace::Platform)
    }

    /// Whether this record authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        MIXED_GRAPHIC_INSPECTION_GRANTS_PUBLICATION_AUTHORITY_V1
    }
}

/// The complete inspected set of one whole mixed candidate. Complete or absent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GraphicMixedInspectionSetV1 {
    inspections: Vec<GraphicMixedInspectionV1>,
}

impl GraphicMixedInspectionSetV1 {
    /// Every inspection, in plan order.
    #[must_use]
    pub fn inspections(&self) -> &[GraphicMixedInspectionV1] {
        &self.inspections
    }

    /// How many authored sources were inspected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inspections.len()
    }

    /// Whether the candidate declared no graphic source at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inspections.is_empty()
    }

    /// How many inspected sources came from the Entity namespace.
    #[must_use]
    pub fn entity_count(&self) -> usize {
        self.inspections
            .iter()
            .filter(|inspection| inspection.namespace == AssetNamespace::Entity)
            .count()
    }

    /// How many inspected sources came from the Platform namespace.
    #[must_use]
    pub fn platform_count(&self) -> usize {
        self.inspections
            .iter()
            .filter(|inspection| inspection.namespace == AssetNamespace::Platform)
            .count()
    }

    /// The distinct real child processes that produced these records, ascending.
    ///
    /// A zero identifier (the closed AVIF envelope carries none) is never counted as a process.
    #[must_use]
    pub fn distinct_worker_pids(&self) -> Vec<u32> {
        let mut pids: Vec<u32> = self
            .inspections
            .iter()
            .map(|inspection| inspection.worker_pid)
            .filter(|pid| *pid != 0)
            .collect();
        pids.sort_unstable();
        pids.dedup();
        pids
    }

    /// Whether this set authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        MIXED_GRAPHIC_INSPECTION_GRANTS_PUBLICATION_AUTHORITY_V1
    }

    /// Whether this set is a global publishable snapshot. Always `false`.
    #[must_use]
    pub const fn proves_whole_publication(&self) -> bool {
        MIXED_GRAPHIC_INSPECTION_PROVES_WHOLE_PUBLICATION_V1
    }
}

/// The closed result of one whole-candidate mixed inspection.
///
/// Every arm other than `Complete` carries exactly one typed refusal and **no** records: a source
/// that fails anywhere in the plan discards every earlier successful acquisition and worker run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MixedGraphicCandidateOutcomeV1 {
    /// Every authored obligation across both namespaces was acquired and inspected.
    Complete(GraphicMixedInspectionSetV1),
    /// The plan itself was ambiguous or over its bound.
    PlanRefused(MixedGraphicPlanRefusalV1),
    /// The accepted Entity reader refused one Entity obligation.
    EntityReadRefused(GraphicAssetReadRefusalV1),
    /// The accepted Platform reader refused one Platform obligation.
    PlatformReadRefused(PlatformAssetReadRefusalV1),
    /// The verified approved binding refused the acquired Platform evidence.
    BindingRefused(PlatformGraphicBaselineRefusalV1),
    /// One authored source failed its accepted media/role/format checks.
    InspectRefused(MixedGraphicInspectRefusalV1),
}

impl MixedGraphicCandidateOutcomeV1 {
    /// The complete set, when every obligation succeeded.
    #[must_use]
    pub const fn complete(&self) -> Option<&GraphicMixedInspectionSetV1> {
        match self {
            Self::Complete(set) => Some(set),
            _ => None,
        }
    }

    /// Whether any arm refused.
    #[must_use]
    pub const fn is_refused(&self) -> bool {
        !matches!(self, Self::Complete(_))
    }

    /// The stable refusal token of whichever arm refused, if any.
    #[must_use]
    pub fn refusal_detail(&self) -> Option<&str> {
        match self {
            Self::Complete(_) => None,
            Self::PlanRefused(refusal) => Some(refusal.detail()),
            Self::EntityReadRefused(refusal) => Some(refusal.detail()),
            Self::PlatformReadRefused(refusal) => Some(refusal.detail()),
            Self::BindingRefused(refusal) => Some(refusal.detail()),
            Self::InspectRefused(refusal) => Some(refusal.detail()),
        }
    }
}

/// The trusted inputs the Platform half of one mixed candidate needs.
///
/// The values are the caller's already adopted Platform root capability token, its independently
/// selected library version, the catalogue model it parsed and the exact catalogue bytes the
/// Platform reader transferred. Nothing here discovers, guesses or rewrites any of them.
#[derive(Clone, Copy)]
pub struct PlatformSourceRequestV1<'a> {
    /// The adopted Platform root capability token, consumed by the accepted reader.
    pub selection: PlatformRootSelectionV1,
    /// The caller's independently selected library version.
    pub selected_version: &'a str,
    /// The parsed catalogue model, checked against the transferred bytes by the reader.
    pub catalogue: &'a PlatformAssetCatalogueV1,
    /// The exact catalogue bytes the Platform reader transferred.
    pub catalogue_bytes: &'a [u8],
}

/// Acquires and inspects **every** authored graphic source of one mixed candidate.
///
/// This is the one composed path. It performs, in this order and without shortcuts:
///
/// 1. the accepted Entity acquisition for the Entity-namespace subset, through
///    [`read_graphic_asset_bytes_v1`], which keeps its confinement, link-count, identity, transfer,
///    boundedness and repeated-obligation checks and still refuses any `platform` obligation;
/// 2. the accepted Platform acquisition for the Platform-namespace subset, through
///    [`read_pinned_platform_sources_v1`], over a stable **unique** request set, with the catalogue
///    pin recomputed from the transferred bytes and each source digest checked against the
///    catalogue's own declaration;
/// 3. the accepted R17 binding, through [`approved_platform_graphic_binding_v1`], so the acquired
///    Platform evidence is vouched for by exact approved identifier and digest;
/// 4. the accepted V4 six-format inspection of every acquired source, through the same per-item
///    checks the Entity-only composition uses.
///
/// The candidate is all-or-nothing: the first failing source returns one attributed refusal and
/// **no** record survives, whether the failure is an Entity source, a Platform source, an
/// unapproved identity or a media check. Nothing here publishes, derives, routes or authorizes.
#[must_use]
pub fn inspect_mixed_graphic_candidate_v1<E, P, L, A>(
    plan: &MixedGraphicPlanV1,
    entity: &E,
    platform: &P,
    request: PlatformSourceRequestV1<'_>,
    legacy: &L,
    avif: &A,
) -> MixedGraphicCandidateOutcomeV1
where
    E: GraphicAssetReadPort + ?Sized,
    P: PlatformAssetLibraryReadPortV1 + ?Sized,
    L: GraphicImageInspectPortV3,
    A: MediaWorkerPortV1,
{
    // 1. Entity-namespace acquisition through the accepted Entity gate.
    let entity_inventory = plan.entity_inventory_v1();
    let entity_set: Option<GraphicAssetSourceSetV1> = if entity_inventory.is_empty() {
        None
    } else {
        match read_graphic_asset_bytes_v1(&entity_inventory, entity) {
            GraphicAssetReadSetOutcomeV1::Complete(set) => Some(set),
            GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
                return MixedGraphicCandidateOutcomeV1::EntityReadRefused(refusal);
            }
        }
    };

    // 2. Platform-namespace acquisition through the accepted pinned Platform reader, over a stable
    //    unique request set in plan order.
    let mut platform_ids: Vec<&str> = Vec::new();
    for obligation in plan.obligations() {
        if obligation.namespace() != AssetNamespace::Platform {
            continue;
        }
        let id = obligation.reference().reference();
        if !platform_ids.contains(&id) {
            platform_ids.push(id);
        }
    }
    let platform_set: Option<PlatformAssetSourceSetV1> = if platform_ids.is_empty() {
        None
    } else {
        let mut requests: Vec<PlatformAssetRefV1> = Vec::with_capacity(platform_ids.len());
        for id in &platform_ids {
            match PlatformAssetRefV1::from_identifier(id) {
                Ok(reference) => requests.push(reference),
                Err(refusal) => {
                    return MixedGraphicCandidateOutcomeV1::PlatformReadRefused(refusal);
                }
            }
        }
        match read_pinned_platform_sources_v1(
            request.selection,
            request.selected_version,
            request.catalogue,
            &requests,
            platform,
        ) {
            PlatformLibraryReadOutcomeV1::Complete(set) => Some(set),
            PlatformLibraryReadOutcomeV1::Refused(refusal) => {
                return MixedGraphicCandidateOutcomeV1::PlatformReadRefused(refusal);
            }
        }
    };

    // 3. The accepted R17 binding over exactly the evidence that was acquired.
    if let Some(set) = platform_set.as_ref()
        && let Err(refusal) = approved_platform_graphic_binding_v1(
            request.selected_version,
            request.catalogue_bytes,
            request.catalogue,
            set,
        )
    {
        return MixedGraphicCandidateOutcomeV1::BindingRefused(refusal);
    }

    // 4. Whole-plan inspection, in plan order, over the acquired bytes only.
    let mut inspections: Vec<GraphicMixedInspectionV1> = Vec::with_capacity(plan.len());
    let mut entity_position = 0_usize;
    for obligation in plan.obligations() {
        // A root-layer registration has no authored EFS obligation to record as the accepted AVIF
        // envelope's provenance. This composition refuses that combination explicitly instead of
        // fabricating a carrier; no approved `1.0.0` asset is AVIF.
        if obligation.declaring_owner().is_none()
            && format_of_reference(obligation.reference())
                .is_some_and(GraphicImageFormatV3::is_avif)
        {
            return MixedGraphicCandidateOutcomeV1::InspectRefused(MixedGraphicInspectRefusalV1 {
                kind: MixedGraphicInspectRefusalKindV1::RootRegistrationAvifUnsupported,
                obligation: Box::new(obligation.clone()),
            });
        }

        let acquired = match obligation.namespace() {
            AssetNamespace::Entity => entity_set.as_ref().and_then(|set| {
                let evidence = set.evidence().get(entity_position)?;
                entity_position += 1;
                Some((
                    evidence.bytes().to_vec(),
                    evidence.logical_path().to_owned(),
                ))
            }),
            AssetNamespace::Platform => platform_set
                .as_ref()
                .and_then(|set| set.exact(obligation.reference().reference()))
                .map(|evidence| {
                    (
                        evidence.bytes().to_vec(),
                        obligation.reference().reference().to_owned(),
                    )
                }),
        };
        let Some((bytes, logical_path)) = acquired else {
            return MixedGraphicCandidateOutcomeV1::InspectRefused(MixedGraphicInspectRefusalV1 {
                kind: MixedGraphicInspectRefusalKindV1::BindingDoesNotCoverObligation,
                obligation: Box::new(obligation.clone()),
            });
        };

        let carrier = obligation.authored_obligation_v1();
        let facts = match inspect_item_v4(
            obligation.role(),
            obligation.reference(),
            &bytes,
            carrier.as_ref(),
            legacy,
            avif,
        ) {
            Ok(facts) => facts,
            Err(kind) => {
                return MixedGraphicCandidateOutcomeV1::InspectRefused(
                    MixedGraphicInspectRefusalV1 {
                        kind: MixedGraphicInspectRefusalKindV1::Image(kind),
                        obligation: Box::new(obligation.clone()),
                    },
                );
            }
        };

        inspections.push(GraphicMixedInspectionV1 {
            obligation: obligation.clone(),
            namespace: obligation.namespace(),
            logical_path,
            format: facts.format,
            mime: facts.mime,
            source_digest: facts.source_digest,
            width: facts.width,
            height: facts.height,
            pixels: facts.pixels,
            payload: facts.payload,
            worker_pid: facts.worker_pid,
            wall_ms: facts.wall_ms,
        });
    }

    MixedGraphicCandidateOutcomeV1::Complete(GraphicMixedInspectionSetV1 { inspections })
}
