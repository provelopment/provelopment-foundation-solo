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

use solo_domain::{AssetNamespace, PropertyKey, QualifiedOwnerV1};

use crate::graphic_asset_inventory::GraphicAssetObligationV1;
use crate::graphic_asset_read::{GraphicAssetSourceSetV1, MAX_GRAPHIC_SOURCE_BYTES};
use crate::media_worker::{MediaWorkerDigestV1, sha256_media_worker_bytes_v1};

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
