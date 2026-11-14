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
