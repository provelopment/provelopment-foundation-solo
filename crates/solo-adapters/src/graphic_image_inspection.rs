//! Confined offline still-image inspection (`R03.C3`): the one real PNG/JPEG/WebP adapter.
//!
//! This module is the outward half of the image-inspection gate. It holds **no decoder call in the
//! trusted parent**: every decode happens inside a single-use child process that this module starts
//! through the safe supervisor in [`crate::media_worker_host`], which in turn installs the platform
//! confinement through the one approved native seam.
//!
//! **The bytes flow exactly once.** The parent owns the acquired source bytes, hashes them itself,
//! writes them to the child's stdin and closes it. The child confirms its own confinement, checks
//! the claimed source digest, identifies the real format from the file signature, refuses animation
//! and ambiguity, decodes the complete still image with the pinned `image` codec and writes one
//! framed observation followed by exactly the decoded RGBA8 plane. The parent re-derives the format
//! and geometry from the source bytes it owns, re-hashes the decoded bytes it actually received,
//! compares both digests, and only then reports an observation.
//!
//! **What this proves.** That these exact bytes are a complete, bounded, non-animated PNG, JPEG or
//! WebP still image with the declared MIME, and that the decode happened inside a child whose
//! platform bounds the parent verified. **What it does not prove:** colour management, orientation,
//! auxiliary-chunk or trailing-data conformance, licensing, catalogue membership, a derivative, a
//! route, a browser surface, publication authority, a hostile-worker syscall sandbox, or race-free
//! process-identity pinning.
//!
//! **No path, command or destination.** The request carries bytes, one closed format token and the
//! digest. This module never receives a file path, never reads a file, writes no output file,
//! generates no URL and publishes nothing.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use solo_application::{
    GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY, GraphicAssetObligationInventoryV1,
    GraphicAssetReadSetOutcomeV1, GraphicAssetSourceSetV1, GraphicImageFormatV1,
    GraphicImageInspectPortOutcomeV1, GraphicImageInspectPortV1, GraphicImageInspectRefusalKindV1,
    GraphicImageInspectRequestV1, GraphicImageInspectionOutcomeV1, GraphicImageInspectionSetV1,
    GraphicImageWorkerObservationV1, MAX_IMAGE_DECODED_BYTES_V1, MAX_IMAGE_PIXELS_V1,
    MAX_IMAGE_SIDE_V1, MAX_IMAGE_SOURCE_BYTES_V1, MediaWorkerDigestV1,
    inspect_graphic_image_source_set_v1, read_graphic_asset_bytes_v1, sha256_media_worker_bytes_v1,
};

use crate::media_worker_host::{
    ImageWorkerRunV1, run_confined_image_worker_mode_v1, run_confined_image_worker_v1,
};
use crate::source_read::ContentRootCapability;

/// The one reserved internal flag that starts the confined image worker.
///
/// It is dispatched before the public adapter, appears in no help output, is not a product surface
/// and accepts only the closed tokens below.
pub const INTERNAL_IMAGE_WORKER_FLAG_V1: &str = "--internal-image-worker-v1";

/// The fixed child mode that performs the real decode.
pub const IMAGE_WORKER_MODE_DECODE_V1: &str = "decode";

/// A closed test seam: decode correctly but claim a different source digest.
pub const IMAGE_WORKER_MODE_WRONG_SOURCE_V1: &str = "wrong-source";

/// A closed test seam: decode correctly but claim a different output digest.
///
/// It exists so the parent's independent output-digest check can be proved load-bearing. Removing
/// that parent-side guard must make the `n097_*` sensitivity test fail.
pub const IMAGE_WORKER_MODE_WRONG_OUTPUT_V1: &str = "wrong-output";

/// A closed test seam: decode correctly but send one byte fewer than claimed.
pub const IMAGE_WORKER_MODE_SHORT_OUTPUT_V1: &str = "short-output";

/// A closed test seam: decode correctly but append one trailing byte after the plane.
pub const IMAGE_WORKER_MODE_TRAILING_V1: &str = "trailing";

/// A closed test seam: refuse before reading or decoding anything.
pub const IMAGE_WORKER_MODE_REFUSE_V1: &str = "refuse";

/// A closed test seam: hold past the fixed wall deadline without writing a frame.
pub const IMAGE_WORKER_MODE_HANG_V1: &str = "hang";

/// A closed test seam: probe whether one numeric parent handle is visible to this child.
///
/// The token carries the parent value and a guaranteed-invalid control as two decimal u32 values
/// separated by a colon. Only the reserved internal mode reaches it, and only the integration
/// suite supplies it; nothing here accepts authored content or a public argument.
pub const IMAGE_WORKER_MODE_PROBE_HANDLE_V1: &str = "probe-handle";

/// A closed test seam: probe whether one named parent variable is visible to this child.
pub const IMAGE_WORKER_MODE_PROBE_ENVIRONMENT_V1: &str = "probe-environment";

/// The exact first token of the bounded handle-isolation probe answer.
pub const N102_HANDLE_PROBE_PREFIX_V1: &str = "SOLO-N102-HANDLE-V1";

/// The exact first token of the bounded environment-isolation probe answer.
pub const N102_ENVIRONMENT_PROBE_PREFIX_V1: &str = "SOLO-N102-ENV-V1";

/// The closed reserved mode token for one N102 object-identity event probe.
pub const IMAGE_WORKER_MODE_PROBE_EVENT_V1: &str = "probe-event";

/// The exact first token of the child's bounded event-probe answer line.
pub const N102_EVENT_PROBE_PREFIX_V1: &str = "SOLO-N102-EVENT-V1";

/// The largest accepted length of one N102 probe parameter, in bytes.
pub const MAX_N102_PROBE_PARAMETER_BYTES_V1: usize = 64;

/// The exact first token of the worker's framed observation line.
pub const IMAGE_WORKER_FRAME_PREFIX_V1: &str = "SOLO-IMAGE-WORKER-V1";

/// The largest accepted length of the worker's framed observation line, in bytes.
pub const MAX_IMAGE_WORKER_FRAME_BYTES_V1: usize = 1024;

/// The largest accepted total stdout of one worker attempt, in bytes.
pub const MAX_IMAGE_WORKER_STDOUT_BYTES_V1: u64 = MAX_IMAGE_DECODED_BYTES_V1 + 4096;

/// The child exit status that means the child refused before decoding.
pub const IMAGE_WORKER_EXIT_REFUSED_V1: u8 = 4;

/// The child exit status that means the child's own confinement readback failed.
pub const IMAGE_WORKER_EXIT_UNCONFINED_V1: u8 = 5;

/// The child exit status that means the source was not a valid still image of the declared format.
pub const IMAGE_WORKER_EXIT_INVALID_MEDIA_V1: u8 = 6;

/// The child exit status that means the decode itself failed.
pub const IMAGE_WORKER_EXIT_DECODE_FAILED_V1: u8 = 7;

/// The geometry and confirmed still-image format of one byte stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StillImageShapeV1 {
    /// The format the bytes' own signature identifies.
    pub(crate) format: GraphicImageFormatV1,
    /// The width declared by the container header.
    pub(crate) width: u32,
    /// The height declared by the container header.
    pub(crate) height: u32,
}

/// Why a byte stream is not an admissible still image of a known format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ImageShapeRefusalV1 {
    /// The signature names no admitted format.
    UnknownFormat,
    /// The structure ended before a mandatory field or terminator.
    Truncated,
    /// The structure is internally inconsistent or carries trailing bytes.
    Malformed,
    /// The stream is an animation (APNG, animated WebP) or ambiguous between still and animated.
    Animated,
    /// The stream uses a JPEG coding process this slice does not admit.
    UnsupportedCoding,
    /// A declared side length is zero.
    EmptyGeometry,
}

fn byte_at(bytes: &[u8], index: usize) -> Result<u8, ImageShapeRefusalV1> {
    bytes
        .get(index)
        .copied()
        .ok_or(ImageShapeRefusalV1::Truncated)
}

fn slice_at(bytes: &[u8], start: usize, len: usize) -> Result<&[u8], ImageShapeRefusalV1> {
    let end = start
        .checked_add(len)
        .ok_or(ImageShapeRefusalV1::Truncated)?;
    bytes.get(start..end).ok_or(ImageShapeRefusalV1::Truncated)
}

fn u16_be_at(bytes: &[u8], start: usize) -> Result<u16, ImageShapeRefusalV1> {
    let pair = slice_at(bytes, start, 2)?;
    Ok(u16::from_be_bytes([pair[0], pair[1]]))
}

fn u32_be_at(bytes: &[u8], start: usize) -> Result<u32, ImageShapeRefusalV1> {
    let quad = slice_at(bytes, start, 4)?;
    Ok(u32::from_be_bytes([quad[0], quad[1], quad[2], quad[3]]))
}

fn u16_le_at(bytes: &[u8], start: usize) -> Result<u16, ImageShapeRefusalV1> {
    let pair = slice_at(bytes, start, 2)?;
    Ok(u16::from_le_bytes([pair[0], pair[1]]))
}

fn u32_le_at(bytes: &[u8], start: usize) -> Result<u32, ImageShapeRefusalV1> {
    let quad = slice_at(bytes, start, 4)?;
    Ok(u32::from_le_bytes([quad[0], quad[1], quad[2], quad[3]]))
}

fn u24_le_at(bytes: &[u8], start: usize) -> Result<u32, ImageShapeRefusalV1> {
    let triple = slice_at(bytes, start, 3)?;
    Ok(u32::from(triple[0]) | (u32::from(triple[1]) << 8) | (u32::from(triple[2]) << 16))
}

/// The PNG file signature, byte for byte.
const PNG_SIGNATURE_V1: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];

/// Identifies the format of one byte stream from its own signature.
///
/// This is the parent's and the child's independent format decision. It never consults an
/// extension, a MIME string from the worker, or any caller-supplied hint.
pub(crate) fn shape_by_signature_v1(bytes: &[u8]) -> Option<GraphicImageFormatV1> {
    if bytes.len() >= 8 && bytes[..8] == PNG_SIGNATURE_V1 {
        return Some(GraphicImageFormatV1::Png);
    }
    if bytes.len() >= 3 && bytes[0] == 0xff && bytes[1] == 0xd8 && bytes[2] == 0xff {
        return Some(GraphicImageFormatV1::Jpeg);
    }
    if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some(GraphicImageFormatV1::WebP);
    }
    None
}

/// Parses one PNG chunk stream, refusing animation, truncation and trailing bytes.
///
/// The chunk walk is bounded by the buffer and by checked arithmetic, requires `IHDR` first,
/// requires `IEND` last, and requires that `IEND` ends exactly at the buffer end, so a stream that
/// appends unaccounted data after the terminator is refused instead of being called clean.
fn shape_of_png_v1(bytes: &[u8]) -> Result<StillImageShapeV1, ImageShapeRefusalV1> {
    if bytes.len() < PNG_SIGNATURE_V1.len() || bytes[..PNG_SIGNATURE_V1.len()] != PNG_SIGNATURE_V1 {
        return Err(ImageShapeRefusalV1::UnknownFormat);
    }
    let mut offset = PNG_SIGNATURE_V1.len();
    let mut shape: Option<(u32, u32)> = None;
    loop {
        let length = u32_be_at(bytes, offset)?;
        let kind = slice_at(bytes, offset + 4, 4)?;
        let data_start = offset + 8;
        let data_len = usize::try_from(length).map_err(|_| ImageShapeRefusalV1::Malformed)?;
        let data_end = data_start
            .checked_add(data_len)
            .ok_or(ImageShapeRefusalV1::Malformed)?;
        let next = data_end
            .checked_add(4)
            .ok_or(ImageShapeRefusalV1::Malformed)?;
        slice_at(bytes, data_start, data_len)?;
        if next > bytes.len() {
            return Err(ImageShapeRefusalV1::Truncated);
        }

        if shape.is_none() {
            if kind != b"IHDR" || data_len != 13 {
                return Err(ImageShapeRefusalV1::Malformed);
            }
            let width = u32_be_at(bytes, data_start)?;
            let height = u32_be_at(bytes, data_start + 4)?;
            if width == 0 || height == 0 {
                return Err(ImageShapeRefusalV1::EmptyGeometry);
            }
            shape = Some((width, height));
        } else if kind == b"acTL" {
            // An animation control chunk means an APNG, whatever the frame count claims.
            return Err(ImageShapeRefusalV1::Animated);
        }

        if kind == b"IEND" {
            if next != bytes.len() {
                return Err(ImageShapeRefusalV1::Malformed);
            }
            let (width, height) = shape.ok_or(ImageShapeRefusalV1::Malformed)?;
            return Ok(StillImageShapeV1 {
                format: GraphicImageFormatV1::Png,
                width,
                height,
            });
        }
        offset = next;
    }
}

/// Parses one JPEG marker stream, refusing unsupported coding processes and a missing terminator.
///
/// Only baseline (`SOF0`), extended sequential (`SOF1`) and progressive (`SOF2`) Huffman-coded
/// still images are admitted; arithmetic, lossless and hierarchical processes are refused rather
/// than handed to a decoder that might accept them under different semantics. The stream must end
/// exactly with `FFD9`, so data appended after the image is refused instead of being called clean.
fn shape_of_jpeg_v1(bytes: &[u8]) -> Result<StillImageShapeV1, ImageShapeRefusalV1> {
    if bytes.len() < 4 || bytes[0] != 0xff || bytes[1] != 0xd8 {
        return Err(ImageShapeRefusalV1::UnknownFormat);
    }
    if bytes[bytes.len() - 2] != 0xff || bytes[bytes.len() - 1] != 0xd9 {
        return Err(ImageShapeRefusalV1::Truncated);
    }

    let mut offset = 2usize;
    let mut shape: Option<(u32, u32)> = None;
    loop {
        if byte_at(bytes, offset)? != 0xff {
            return Err(ImageShapeRefusalV1::Malformed);
        }
        let mut marker_at = offset;
        while byte_at(bytes, marker_at)? == 0xff {
            marker_at += 1;
        }
        let marker = byte_at(bytes, marker_at)?;
        offset = marker_at + 1;

        match marker {
            // A stuffed zero or a second start-of-image outside entropy data is malformed.
            0x00 | 0xd8 => return Err(ImageShapeRefusalV1::Malformed),
            // A terminator before any scan is admissible only when the geometry was already read.
            0xd9 => {
                let (width, height) = shape.ok_or(ImageShapeRefusalV1::Malformed)?;
                return Ok(StillImageShapeV1 {
                    format: GraphicImageFormatV1::Jpeg,
                    width,
                    height,
                });
            }
            // Standalone markers carry no length field.
            0x01 | 0xd0..=0xd7 => continue,
            _ => {}
        }

        let declared = u16_be_at(bytes, offset)?;
        if declared < 2 {
            return Err(ImageShapeRefusalV1::Malformed);
        }
        let body_len = usize::from(declared) - 2;
        let body_start = offset + 2;
        slice_at(bytes, body_start, body_len)?;
        let body_end = body_start
            .checked_add(body_len)
            .ok_or(ImageShapeRefusalV1::Malformed)?;

        match marker {
            0xc0..=0xc2 => {
                // The frame header starts with one precision byte, so the sampled height
                // and width are the next two big-endian pairs; reading from the header start
                // would mistake precision for the high byte of the height.
                let height = u32::from(u16_be_at(bytes, body_start + 1)?);
                let width = u32::from(u16_be_at(bytes, body_start + 3)?);
                let components = byte_at(bytes, body_start + 5)?;
                if width == 0 || height == 0 {
                    return Err(ImageShapeRefusalV1::EmptyGeometry);
                }
                if components == 0 {
                    return Err(ImageShapeRefusalV1::Malformed);
                }
                shape = Some((width, height));
            }
            0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf => {
                return Err(ImageShapeRefusalV1::UnsupportedCoding);
            }
            0xda => {
                // Start of scan: the geometry is mandatory before any entropy-coded data.
                let (width, height) = shape.ok_or(ImageShapeRefusalV1::Malformed)?;
                return Ok(StillImageShapeV1 {
                    format: GraphicImageFormatV1::Jpeg,
                    width,
                    height,
                });
            }
            _ => {}
        }

        offset = body_end;
    }
}

/// Parses one RIFF/WebP container, refusing animation and any trailing or truncated structure.
///
/// The RIFF size field must account for the whole buffer exactly, so a stream with appended data is
/// refused. `ANIM`/`ANMF` chunks and the `VP8X` animation flag are refused, which is how an animated
/// WebP is kept out of a still-image contract that has no frame semantics.
fn shape_of_webp_v1(bytes: &[u8]) -> Result<StillImageShapeV1, ImageShapeRefusalV1> {
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WEBP" {
        return Err(ImageShapeRefusalV1::UnknownFormat);
    }
    let riff_size = u64::from(u32_le_at(bytes, 4)?);
    let total = u64::try_from(bytes.len()).map_err(|_| ImageShapeRefusalV1::Malformed)?;
    if riff_size.checked_add(8) != Some(total) {
        return Err(ImageShapeRefusalV1::Malformed);
    }

    let mut offset = 12usize;
    let mut shape: Option<(u32, u32)> = None;
    while offset < bytes.len() {
        let fourcc = slice_at(bytes, offset, 4)?;
        let size = u32_le_at(bytes, offset + 4)?;
        let size_len = usize::try_from(size).map_err(|_| ImageShapeRefusalV1::Malformed)?;
        let data_start = offset + 8;
        slice_at(bytes, data_start, size_len)?;
        let data_end = data_start
            .checked_add(size_len)
            .ok_or(ImageShapeRefusalV1::Malformed)?;
        let padded_end = data_end
            .checked_add(size_len & 1)
            .ok_or(ImageShapeRefusalV1::Malformed)?;
        if padded_end > bytes.len() {
            return Err(ImageShapeRefusalV1::Truncated);
        }

        if fourcc == b"ANIM" || fourcc == b"ANMF" {
            return Err(ImageShapeRefusalV1::Animated);
        }

        if shape.is_none() {
            match fourcc {
                b"VP8 " => {
                    if size_len < 10 {
                        return Err(ImageShapeRefusalV1::Truncated);
                    }
                    if byte_at(bytes, data_start + 3)? != 0x9d
                        || byte_at(bytes, data_start + 4)? != 0x01
                        || byte_at(bytes, data_start + 5)? != 0x2a
                    {
                        return Err(ImageShapeRefusalV1::Malformed);
                    }
                    let width = u32::from(u16_le_at(bytes, data_start + 6)? & 0x3fff);
                    let height = u32::from(u16_le_at(bytes, data_start + 8)? & 0x3fff);
                    if width == 0 || height == 0 {
                        return Err(ImageShapeRefusalV1::EmptyGeometry);
                    }
                    shape = Some((width, height));
                }
                b"VP8L" => {
                    if size_len < 5 {
                        return Err(ImageShapeRefusalV1::Truncated);
                    }
                    if byte_at(bytes, data_start)? != 0x2f {
                        return Err(ImageShapeRefusalV1::Malformed);
                    }
                    let packed = u32::from(byte_at(bytes, data_start + 1)?)
                        | (u32::from(byte_at(bytes, data_start + 2)?) << 8)
                        | (u32::from(byte_at(bytes, data_start + 3)?) << 16)
                        | (u32::from(byte_at(bytes, data_start + 4)?) << 24);
                    let width = (packed & 0x3fff) + 1;
                    let height = ((packed >> 14) & 0x3fff) + 1;
                    shape = Some((width, height));
                }
                b"VP8X" => {
                    if size_len < 10 {
                        return Err(ImageShapeRefusalV1::Truncated);
                    }
                    if byte_at(bytes, data_start)? & 0x02 != 0 {
                        return Err(ImageShapeRefusalV1::Animated);
                    }
                    let width = u24_le_at(bytes, data_start + 4)?
                        .checked_add(1)
                        .ok_or(ImageShapeRefusalV1::Malformed)?;
                    let height = u24_le_at(bytes, data_start + 7)?
                        .checked_add(1)
                        .ok_or(ImageShapeRefusalV1::Malformed)?;
                    shape = Some((width, height));
                }
                _ => return Err(ImageShapeRefusalV1::UnknownFormat),
            }
        }

        offset = padded_end;
    }

    let (width, height) = shape.ok_or(ImageShapeRefusalV1::Malformed)?;
    Ok(StillImageShapeV1 {
        format: GraphicImageFormatV1::WebP,
        width,
        height,
    })
}

/// Identifies and structurally validates one byte stream.
pub(crate) fn shape_of_bytes_v1(bytes: &[u8]) -> Result<StillImageShapeV1, ImageShapeRefusalV1> {
    match shape_by_signature_v1(bytes) {
        Some(GraphicImageFormatV1::Png) => shape_of_png_v1(bytes),
        Some(GraphicImageFormatV1::Jpeg) => shape_of_jpeg_v1(bytes),
        Some(GraphicImageFormatV1::WebP) => shape_of_webp_v1(bytes),
        None => Err(ImageShapeRefusalV1::UnknownFormat),
    }
}

/// The decoded RGBA8 plane size of one admissible geometry, or `None` on overflow.
pub(crate) fn plane_bytes_v1(width: u32, height: u32) -> Option<u64> {
    u64::from(width)
        .checked_mul(u64::from(height))?
        .checked_mul(4)
}

/// Whether one geometry is inside the accepted contract.
pub(crate) fn geometry_admissible_v1(width: u32, height: u32) -> bool {
    if width == 0 || height == 0 || width > MAX_IMAGE_SIDE_V1 || height > MAX_IMAGE_SIDE_V1 {
        return false;
    }
    match u64::from(width).checked_mul(u64::from(height)) {
        Some(pixels) => pixels <= MAX_IMAGE_PIXELS_V1,
        None => false,
    }
}

/// The closed child modes reachable through the reserved flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImageWorkerModeV1 {
    /// The only productive mode: decode and report honestly.
    Decode,
    /// Decode, then claim a different source digest than the one supplied.
    WrongSource,
    /// Decode, then claim a different output digest than the bytes sent.
    WrongOutput,
    /// Decode, then send one byte fewer than claimed.
    ShortOutput,
    /// Decode, then append one byte after the claimed plane.
    Trailing,
    /// Refuse immediately, before reading any source byte.
    Refuse,
    /// Hold well past the fixed wall deadline without writing a frame.
    Hang,
    /// Probe this child's own handle table for one supplied numeric value.
    ProbeHandle,
    /// Probe this child's own environment for one supplied variable name.
    ProbeEnvironment,
    /// Attempt one bounded event-object signalling probe from inside this confined child.
    ProbeEvent,
}

impl ImageWorkerModeV1 {
    /// Parses one closed mode token, with the bounded probe parameter when that mode carries one.
    fn from_token(token: &str) -> Option<(Self, Option<&str>)> {
        if let Some(rest) = token.strip_prefix("probe-handle:") {
            return Some((Self::ProbeHandle, Some(rest)));
        }
        if let Some(rest) = token.strip_prefix("probe-environment:") {
            return Some((Self::ProbeEnvironment, Some(rest)));
        }
        if let Some(rest) = token.strip_prefix("probe-event:") {
            return Some((Self::ProbeEvent, Some(rest)));
        }
        let mode = match token {
            IMAGE_WORKER_MODE_DECODE_V1 => Self::Decode,
            IMAGE_WORKER_MODE_WRONG_SOURCE_V1 => Self::WrongSource,
            IMAGE_WORKER_MODE_WRONG_OUTPUT_V1 => Self::WrongOutput,
            IMAGE_WORKER_MODE_SHORT_OUTPUT_V1 => Self::ShortOutput,
            IMAGE_WORKER_MODE_TRAILING_V1 => Self::Trailing,
            IMAGE_WORKER_MODE_REFUSE_V1 => Self::Refuse,
            IMAGE_WORKER_MODE_HANG_V1 => Self::Hang,
            _ => return None,
        };
        Some((mode, None))
    }

    /// Whether this mode is one of the two closed N102 isolation probes.
    fn is_n102_probe(self) -> bool {
        matches!(
            self,
            Self::ProbeHandle | Self::ProbeEnvironment | Self::ProbeEvent
        )
    }
}

/// Parses one canonical `u32` decimal for the closed N102 event probe.
///
/// Only plain ASCII digits are accepted: sign characters, separators, whitespace, extra colons and
/// any overflow are refused, so a parameter can never be reinterpreted by a different numeric
/// parser in the parent than the one the child used.
#[cfg(windows)]
fn n102_canonical_u32_v1(text: &str) -> Option<u32> {
    if text.is_empty() || text.len() > 10 || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

/// The bounded answer line for one closed N102 isolation probe, or `None` when it cannot be
/// answered honestly on this host or from this parameter.
///
/// Windows only: the probes read this child's own real kernel handle table and real environment
/// block. No handle value, variable value or environment block is ever printed.
#[cfg(windows)]
fn n102_probe_answer_v1(mode: ImageWorkerModeV1, parameter: Option<&str>) -> Option<String> {
    let parameter = parameter?;
    if parameter.len() > MAX_N102_PROBE_PARAMETER_BYTES_V1 || !parameter.is_ascii() {
        return None;
    }
    let pid = std::process::id();
    match mode {
        ImageWorkerModeV1::ProbeHandle => {
            let (value, control) = parameter.split_once(':')?;
            let value: u32 = value.parse().ok()?;
            let control: u32 = control.parse().ok()?;
            let (live, control_live) =
                crate::media_worker_host::n102_probe_handle_v1(value, control);
            Some(format!(
                "{} live={} control_live={} pid={pid}\n",
                N102_HANDLE_PROBE_PREFIX_V1,
                u8::from(live),
                u8::from(control_live)
            ))
        }
        ImageWorkerModeV1::ProbeEnvironment => {
            let (canary, system_root) = crate::media_worker_host::n102_probe_environment_v1(
                std::ffi::OsStr::new(parameter),
            );
            Some(format!(
                "{} canary={} system_root={} pid={pid}\n",
                N102_ENVIRONMENT_PROBE_PREFIX_V1,
                u8::from(canary),
                u8::from(system_root)
            ))
        }
        ImageWorkerModeV1::ProbeEvent => {
            let (candidate, control) = parameter.split_once(':')?;
            let candidate = n102_canonical_u32_v1(candidate)?;
            let control = n102_canonical_u32_v1(control)?;
            let (attempted, witness) =
                crate::media_worker_host::n102_probe_event_v1(candidate, control);
            if !witness {
                // Without a positive local witness the child cannot honestly claim to have probed
                // anything, so it refuses instead of printing an unusable answer.
                return None;
            }
            Some(format!(
                "{} attempted={} witness={} pid={pid}\n",
                N102_EVENT_PROBE_PREFIX_V1,
                u8::from(attempted),
                u8::from(witness)
            ))
        }
        _ => None,
    }
}

/// Off Windows there is no N102 native boundary, so both probes refuse rather than pretend.
#[cfg(not(windows))]
fn n102_probe_answer_v1(_mode: ImageWorkerModeV1, parameter: Option<&str>) -> Option<String> {
    // The bound is validated on every host so the closed contract stays identical.
    if parameter?.len() > MAX_N102_PROBE_PARAMETER_BYTES_V1 {
        return None;
    }
    None
}

/// The pinned `image` format value named by one admitted format.
fn codec_format_v1(format: GraphicImageFormatV1) -> image::ImageFormat {
    match format {
        GraphicImageFormatV1::Png => image::ImageFormat::Png,
        GraphicImageFormatV1::Jpeg => image::ImageFormat::Jpeg,
        GraphicImageFormatV1::WebP => image::ImageFormat::WebP,
    }
}

/// Runs the reserved image-worker mode, or returns `None` when the flag is absent.
///
/// A present flag with any wrong token shape is a fixed refusal, never a fallback to the public
/// CLI: the reserved mode is closed and the caller's exit status carries the decision.
pub fn run_internal_image_worker_v1(args: &[std::ffi::OsString]) -> Option<u8> {
    let flag = args.get(1)?;
    if flag != std::ffi::OsStr::new(INTERNAL_IMAGE_WORKER_FLAG_V1) {
        return None;
    }
    // The GIF worker is a separate closed token: it shares the reserved flag and the confined
    // supervisor, but never the still-image frame, format enum or RGBA plane.
    if args.get(2).and_then(|token| token.to_str()) == Some(GIF_WORKER_TOKEN_V1) {
        let Some(claimed) = args
            .get(3)
            .and_then(|token| token.to_str())
            .and_then(MediaWorkerDigestV1::parse_lower_hex)
        else {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        };
        let mode = match args.get(4).and_then(|token| token.to_str()) {
            None | Some(GIF_WORKER_MODE_DECODE_V1) => GifWorkerModeV1::Decode,
            Some(GIF_WORKER_MODE_WRONG_SOURCE_V1) => GifWorkerModeV1::WrongSource,
            Some(GIF_WORKER_MODE_WRONG_TRANSCRIPT_V1) => GifWorkerModeV1::WrongTranscript,
            Some(_) => return Some(IMAGE_WORKER_EXIT_REFUSED_V1),
        };
        if args.len() > 5 {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        }
        return Some(run_gif_worker_child_v1(claimed, mode));
    }
    // The SVG vector worker is its own closed token: same reserved flag and supervisor, a
    // completely different payload grammar.
    if args.get(2).and_then(|token| token.to_str()) == Some(SVG_WORKER_TOKEN_V1) {
        let Some(claimed) = args
            .get(3)
            .and_then(|token| token.to_str())
            .and_then(MediaWorkerDigestV1::parse_lower_hex)
        else {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        };
        let mode = match args.get(4).and_then(|token| token.to_str()) {
            None | Some(SVG_WORKER_MODE_DECODE_V1) => SvgWorkerModeV1::Decode,
            Some(SVG_WORKER_MODE_WRONG_SOURCE_V1) => SvgWorkerModeV1::WrongSource,
            Some(SVG_WORKER_MODE_WRONG_OUTPUT_V1) => SvgWorkerModeV1::WrongOutput,
            Some(_) => return Some(IMAGE_WORKER_EXIT_REFUSED_V1),
        };
        if args.len() > 5 {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        }
        return Some(run_svg_worker_child_v1(claimed, mode));
    }
    // The AVIF still worker is its own closed token: the accepted five-format V3 arm below stays
    // exactly as it is, so no V3 path can reach this decoder.
    if args.get(2).and_then(|token| token.to_str())
        == Some(crate::graphic_avif_inspection::AVIF_WORKER_TOKEN_V1)
    {
        let Some(claimed) = args
            .get(3)
            .and_then(|token| token.to_str())
            .and_then(MediaWorkerDigestV1::parse_lower_hex)
        else {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        };
        if args.len() > 4 {
            return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
        }
        return Some(crate::graphic_avif_inspection::run_avif_worker_child_v1(
            claimed,
        ));
    }
    let Some(format) = args
        .get(2)
        .and_then(|token| token.to_str())
        .and_then(|token| {
            GraphicImageFormatV1::ALL
                .into_iter()
                .find(|format| format.worker_token() == token)
        })
    else {
        return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
    };
    let Some(claimed) = args
        .get(3)
        .and_then(|token| token.to_str())
        .and_then(MediaWorkerDigestV1::parse_lower_hex)
    else {
        return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
    };
    let (mode, parameter) = match args.get(4) {
        None => (ImageWorkerModeV1::Decode, None),
        Some(token) => match token.to_str().and_then(ImageWorkerModeV1::from_token) {
            Some(pair) => pair,
            None => return Some(IMAGE_WORKER_EXIT_REFUSED_V1),
        },
    };
    if args.len() > 5 {
        return Some(IMAGE_WORKER_EXIT_REFUSED_V1);
    }
    Some(run_image_worker_child_v1(format, claimed, mode, parameter))
}

/// The child half: prove confinement, then decode exactly one owned bounded byte stream.
///
/// The order matters. Confinement is established **first**, so no untrusted byte is parsed before
/// the bounds are proved. Only then is the source read, its digest checked, its structure validated
/// and its complete still image decoded. Every failure returns a fixed exit status and writes
/// nothing to stdout, so a refusal can never look like a partial success.
fn run_image_worker_child_v1(
    format: GraphicImageFormatV1,
    claimed: MediaWorkerDigestV1,
    mode: ImageWorkerModeV1,
    parameter: Option<&str>,
) -> u8 {
    let Some(confinement) = crate::media_worker_host::child_image_worker_confinement_v1() else {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    };
    if !confinement.matches_profile_v1() {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    }
    if mode == ImageWorkerModeV1::Refuse {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    if mode == ImageWorkerModeV1::Hang {
        // No frame, no bytes: only the parent's own deadline can end this.
        std::thread::sleep(std::time::Duration::from_millis(
            solo_application::IMAGE_WORKER_WALL_MS_V1 * 2,
        ));
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    if mode.is_n102_probe() {
        // The probes answer one bounded line and exit; they never read source bytes or decode.
        let Some(answer) = n102_probe_answer_v1(mode, parameter) else {
            return IMAGE_WORKER_EXIT_REFUSED_V1;
        };
        let stdout = std::io::stdout();
        let mut out = stdout.lock();
        if out.write_all(answer.as_bytes()).is_err() || out.flush().is_err() {
            return IMAGE_WORKER_EXIT_REFUSED_V1;
        }
        return 0;
    }

    let mut source: Vec<u8> = Vec::new();
    let limit = MAX_IMAGE_SOURCE_BYTES_V1 + 1;
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    if handle
        .by_ref()
        .take(limit)
        .read_to_end(&mut source)
        .is_err()
    {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    let transferred = u64::try_from(source.len()).unwrap_or(u64::MAX);
    if source.is_empty() || transferred > MAX_IMAGE_SOURCE_BYTES_V1 {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    let actual = sha256_media_worker_bytes_v1(&source);
    if actual != claimed {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    let shape = match shape_of_bytes_v1(&source) {
        Ok(shape) => shape,
        Err(_) => return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1,
    };
    if shape.format != format || !geometry_admissible_v1(shape.width, shape.height) {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    }

    let decoded = match image::load_from_memory_with_format(&source, codec_format_v1(format)) {
        Ok(decoded) => decoded,
        Err(_) => return IMAGE_WORKER_EXIT_DECODE_FAILED_V1,
    };
    if decoded.width() != shape.width || decoded.height() != shape.height {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    }
    let plane = decoded.into_rgba8().into_raw();
    let Some(expected) = plane_bytes_v1(shape.width, shape.height) else {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    };
    if u64::try_from(plane.len()).unwrap_or(u64::MAX) != expected {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    }

    let out_digest = sha256_media_worker_bytes_v1(&plane);
    let claimed_source = match mode {
        ImageWorkerModeV1::WrongSource => {
            let mut altered = *claimed.as_bytes();
            altered[0] ^= 0xff;
            MediaWorkerDigestV1::from_bytes(altered)
        }
        _ => claimed,
    };
    let claimed_output = match mode {
        ImageWorkerModeV1::WrongOutput => {
            let mut altered = *out_digest.as_bytes();
            altered[0] ^= 0xff;
            MediaWorkerDigestV1::from_bytes(altered)
        }
        _ => out_digest,
    };

    let declared_len = if mode == ImageWorkerModeV1::ShortOutput {
        expected - 1
    } else {
        expected
    };
    let frame = format!(
        "{prefix} {fields} format={format} mime={mime} width={width} height={height} decoded={decoded} src_sha256={source_sha} out_sha256={out_sha}\n",
        prefix = IMAGE_WORKER_FRAME_PREFIX_V1,
        fields = confinement.frame_fields_v1(),
        format = format.worker_token(),
        mime = format.canonical_mime(),
        width = shape.width,
        height = shape.height,
        decoded = declared_len,
        source_sha = claimed_source.to_hex(),
        out_sha = claimed_output.to_hex(),
    );

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    if out.write_all(frame.as_bytes()).is_err() {
        return IMAGE_WORKER_EXIT_DECODE_FAILED_V1;
    }
    let body: &[u8] = if mode == ImageWorkerModeV1::ShortOutput {
        &plane[..plane.len() - 1]
    } else {
        &plane
    };
    if out.write_all(body).is_err() {
        return IMAGE_WORKER_EXIT_DECODE_FAILED_V1;
    }
    if mode == ImageWorkerModeV1::Trailing && out.write_all(&[0x00]).is_err() {
        return IMAGE_WORKER_EXIT_DECODE_FAILED_V1;
    }
    if out.flush().is_err() {
        return IMAGE_WORKER_EXIT_DECODE_FAILED_V1;
    }
    0
}

/// The exact frame keys this host's worker must emit, in order.
#[cfg(target_os = "linux")]
const IMAGE_FRAME_KEYS_V1: [&str; 15] = [
    "host",
    "as_soft",
    "as_hard",
    "cpu_soft",
    "cpu_hard",
    "pid",
    "pgid",
    "sid",
    "format",
    "mime",
    "width",
    "height",
    "decoded",
    "src_sha256",
    "out_sha256",
];

/// The exact frame keys the Windows worker must emit, in order.
#[cfg(windows)]
const IMAGE_FRAME_KEYS_V1: [&str; 10] = [
    "host",
    "job",
    "pid",
    "format",
    "mime",
    "width",
    "height",
    "decoded",
    "src_sha256",
    "out_sha256",
];

/// One worker frame that parsed strictly against this host's closed key list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParsedImageFrameV1 {
    pub(crate) format: GraphicImageFormatV1,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) decoded: u64,
    pub(crate) claimed_pid: u32,
    pub(crate) source_digest: MediaWorkerDigestV1,
    pub(crate) output_digest: MediaWorkerDigestV1,
    pub(crate) limits_claimed: bool,
}

/// Splits one worker stdout buffer into its strict frame line and the exact remaining bytes.
///
/// The line must appear inside [`MAX_IMAGE_WORKER_FRAME_BYTES_V1`] and the buffer must not exceed
/// [`MAX_IMAGE_WORKER_STDOUT_BYTES_V1`], so neither a long line nor an oversized worker can make the
/// parent accept without bound.
pub(crate) fn split_image_worker_output_v1(stdout: &[u8]) -> Option<(&str, &[u8])> {
    if u64::try_from(stdout.len()).unwrap_or(u64::MAX) > MAX_IMAGE_WORKER_STDOUT_BYTES_V1 {
        return None;
    }
    let newline = stdout
        .iter()
        .take(MAX_IMAGE_WORKER_FRAME_BYTES_V1 + 1)
        .position(|byte| *byte == b'\n')?;
    if newline > MAX_IMAGE_WORKER_FRAME_BYTES_V1 {
        return None;
    }
    let line = std::str::from_utf8(&stdout[..newline]).ok()?;
    Some((line, &stdout[newline + 1..]))
}

/// Parses one frame line strictly: exact prefix, exact keys in exact order, exact value shapes.
pub(crate) fn parse_image_worker_frame_v1(line: &str) -> Option<ParsedImageFrameV1> {
    let mut tokens = line.split(' ');
    if tokens.next()? != IMAGE_WORKER_FRAME_PREFIX_V1 {
        return None;
    }
    let mut values: Vec<&str> = Vec::with_capacity(IMAGE_FRAME_KEYS_V1.len());
    for expected in IMAGE_FRAME_KEYS_V1 {
        let token = tokens.next()?;
        let (key, value) = token.split_once('=')?;
        if key != expected || value.is_empty() {
            return None;
        }
        values.push(value);
    }
    if tokens.next().is_some() {
        return None;
    }

    let mut index = 0usize;
    let mut next = || {
        let value = values.get(index).copied();
        index += 1;
        value
    };

    let host = next()?;
    #[cfg(target_os = "linux")]
    let limits_claimed = {
        if host != "linux" {
            return None;
        }
        let address_soft = next()?.parse::<u64>().ok()?;
        let address_hard = next()?.parse::<u64>().ok()?;
        let cpu_soft = next()?.parse::<u64>().ok()?;
        let cpu_hard = next()?.parse::<u64>().ok()?;
        address_soft == address_hard
            && address_soft == crate::media_worker_host::PROFILE_V1_AS_BYTES
            && cpu_soft == cpu_hard
            && cpu_soft == crate::media_worker_host::PROFILE_V1_CPU_SECONDS
    };
    #[cfg(windows)]
    let limits_claimed = {
        if host != "windows" {
            return None;
        }
        next()? == "member"
    };
    #[cfg(not(any(target_os = "linux", windows)))]
    let limits_claimed = {
        let _ = host;
        return None;
    };

    let claimed_pid = next()?.parse::<u32>().ok()?;
    #[cfg(target_os = "linux")]
    {
        let pgid = next()?.parse::<u32>().ok()?;
        let sid = next()?.parse::<u32>().ok()?;
        if pgid != claimed_pid || sid != claimed_pid {
            return None;
        }
    }
    let format_token = next()?;
    let format = GraphicImageFormatV1::ALL
        .into_iter()
        .find(|format| format.worker_token() == format_token)?;
    if next()? != format.canonical_mime() {
        return None;
    }
    let width = next()?.parse::<u32>().ok()?;
    let height = next()?.parse::<u32>().ok()?;
    let decoded = next()?.parse::<u64>().ok()?;
    let source_digest = MediaWorkerDigestV1::parse_lower_hex(next()?)?;
    let output_digest = MediaWorkerDigestV1::parse_lower_hex(next()?)?;

    Some(ParsedImageFrameV1 {
        format,
        width,
        height,
        decoded,
        claimed_pid,
        source_digest,
        output_digest,
        limits_claimed,
    })
}

/// The real confining parent adapter for one fixed worker executable.
///
/// The executable is supplied by the trusted composition root (this process's own image) or by a
/// test, and is only ever used to start the one reserved internal mode. It is never taken from an
/// authored value, an environment variable or a request, so no input can name a program.
#[derive(Debug, Clone)]
pub struct ConfinedImageInspectAdapterV1 {
    executable: PathBuf,
}

impl ConfinedImageInspectAdapterV1 {
    /// Binds the adapter to one already-resolved executable path.
    #[must_use]
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
        }
    }

    /// Binds the adapter to this process's own image, which is how the composition root wires it.
    #[must_use]
    pub fn for_current_exe() -> Option<Self> {
        std::env::current_exe().ok().map(Self::new)
    }

    /// The executable this adapter starts. Never influenced by authored input.
    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }
}

impl GraphicImageInspectPortV1 for ConfinedImageInspectAdapterV1 {
    fn inspect(&self, request: &GraphicImageInspectRequestV1) -> GraphicImageInspectPortOutcomeV1 {
        let declared = request.declared_format();
        let run = match run_confined_image_worker_v1(
            &self.executable,
            declared.worker_token(),
            &request.source_digest().to_hex(),
            request.bytes(),
        ) {
            Ok(run) => run,
            Err(refusal) => {
                return GraphicImageInspectPortOutcomeV1::Refused(refusal.outcome_kind_v1());
            }
        };
        validate_image_worker_run_v1(run, request)
    }
}

/// Runs one worker attempt in exactly one closed mode and validates the result.
///
/// This is the closed test seam the `n097_*` sensitivity proof uses. It can start only the fixed
/// worker with a closed mode token, so it cannot become a generic execution path; the validation is
/// the same single function the port uses, which is what makes the digest guard provably
/// load-bearing.
#[must_use]
pub fn inspect_image_bytes_with_mode_v1(
    executable: &Path,
    bytes: &[u8],
    format: GraphicImageFormatV1,
    mode_token: &str,
) -> GraphicImageInspectPortOutcomeV1 {
    let request = GraphicImageInspectRequestV1::of(bytes.to_vec(), format);
    match run_confined_image_worker_mode_v1(
        executable,
        format.worker_token(),
        &request.source_digest().to_hex(),
        Some(mode_token),
        request.bytes(),
        &mut crate::media_worker_host::N102FaultRunV1::production(),
    ) {
        Ok(run) => validate_image_worker_run_v1(run, &request),
        Err(refusal) => GraphicImageInspectPortOutcomeV1::Refused(refusal.outcome_kind_v1()),
    }
}

/// Validates one completed supervised run against the request it answered.
///
/// This is the whole parent-side check, used by the port and by the closed test seam so there is
/// exactly one validation path: exit status, confinement and cleanup facts, strict frame schema,
/// independent format and geometry re-derivation, decoded length and the two digests.
fn validate_image_worker_run_v1(
    run: ImageWorkerRunV1,
    request: &GraphicImageInspectRequestV1,
) -> GraphicImageInspectPortOutcomeV1 {
    let declared = request.declared_format();

    // Host facts first: an unproved bound or an unconfirmed reap is never a media result.
    if run.exit_code() != Some(0) || !run.cleanup_verified() || !run.limits_verified() {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::UnverifiedWorker,
        );
    }

    let Some((line, body)) = split_image_worker_output_v1(run.stdout()) else {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::PortRefused,
        );
    };
    let Some(frame) = parse_image_worker_frame_v1(line) else {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::PortRefused,
        );
    };
    if !frame.limits_claimed || frame.claimed_pid != run.child_pid() {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::UnverifiedWorker,
        );
    }
    if frame.format != declared {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::FormatMismatch,
        );
    }

    // The parent re-derives the format and geometry from bytes it owns, so a worker can neither
    // relabel a polyglot nor inflate a claim about the source.
    let Ok(shape) = shape_of_bytes_v1(request.bytes()) else {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::PortRefused,
        );
    };
    if shape.format != declared || shape.width != frame.width || shape.height != frame.height {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::FormatMismatch,
        );
    }
    if !geometry_admissible_v1(frame.width, frame.height) {
        let kind = if frame.width == 0
            || frame.height == 0
            || frame.width > MAX_IMAGE_SIDE_V1
            || frame.height > MAX_IMAGE_SIDE_V1
        {
            GraphicImageInspectRefusalKindV1::DimensionsOutOfRange
        } else {
            GraphicImageInspectRefusalKindV1::PixelCountOutOfRange
        };
        return GraphicImageInspectPortOutcomeV1::Refused(kind);
    }

    let Some(expected) = plane_bytes_v1(frame.width, frame.height) else {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
        );
    };
    let received = u64::try_from(body.len()).unwrap_or(u64::MAX);
    if frame.decoded == 0
        || frame.decoded > MAX_IMAGE_DECODED_BYTES_V1
        || frame.decoded != expected
        || received != frame.decoded
    {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange,
        );
    }

    // The decoded digest is recomputed here over exactly the bytes this parent received. A worker
    // claim is never evidence, and this comparison is the guard the sensitivity proof removes.
    let received_digest = sha256_media_worker_bytes_v1(body);
    if received_digest != frame.output_digest {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::OutputDigestMismatch,
        );
    }
    if frame.source_digest != *request.source_digest() {
        return GraphicImageInspectPortOutcomeV1::Refused(
            GraphicImageInspectRefusalKindV1::SourceDigestMismatch,
        );
    }

    GraphicImageInspectPortOutcomeV1::Inspected(Box::new(GraphicImageWorkerObservationV1 {
        confirmed_format: frame.format,
        observed_mime: declared.canonical_mime(),
        source_digest: frame.source_digest,
        width: frame.width,
        height: frame.height,
        decoded_bytes: frame.decoded,
        decoded_digest: received_digest,
        worker_pid: run.child_pid(),
        limits_verified: run.limits_verified(),
        cleanup_verified: run.cleanup_verified(),
        wall_ms: run.wall_ms(),
    }))
}

/// Which stage refused one real Entity image candidate.
///
/// The two stages stay distinguishable on purpose: a confined acquisition refusal and an
/// inspection refusal are different facts, and collapsing them would hide which gate failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageRefusalV1 {
    /// The confined `R03.C2` source acquisition refused the candidate.
    SourceRead(solo_application::GraphicAssetReadRefusalV1),
    /// The image-inspection policy or the confined worker refused the candidate.
    Inspection(solo_application::GraphicImageInspectRefusalV1),
}

/// The closed result of one real source-read → image-inspection composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageOutcomeV1 {
    /// Every declared obligation was acquired and inspected.
    Complete(GraphicImageInspectionSetV1),
    /// Exactly one typed refusal from the stage that refused.
    Refused(EntityImageRefusalV1),
}

impl EntityImageOutcomeV1 {
    /// Whether this outcome authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        GRAPHIC_IMAGE_INSPECTION_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// Acquires and then inspects every declared Entity graphic source of one inventory.
///
/// This is the real composition the `R03.C3` slice exists for: the confined `assets/` reader
/// produces owned bytes with physical identity, and the image gate turns them into an inert,
/// decoded result or one whole-candidate refusal. Public, internal, unlisted and suppressed
/// obligations all take the identical path, because the inventory is the complete authored set.
/// Nothing here filters by exposure and nothing here publishes.
#[must_use]
pub fn inspect_entity_graphic_images_v1(
    root: &ContentRootCapability,
    inventory: &GraphicAssetObligationInventoryV1,
    executable: &Path,
) -> EntityImageOutcomeV1 {
    let source_set: GraphicAssetSourceSetV1 = match read_graphic_asset_bytes_v1(inventory, root) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            return EntityImageOutcomeV1::Refused(EntityImageRefusalV1::SourceRead(refusal));
        }
    };

    let adapter = ConfinedImageInspectAdapterV1::new(executable);
    match inspect_graphic_image_source_set_v1(&source_set, &adapter) {
        GraphicImageInspectionOutcomeV1::Complete(set) => EntityImageOutcomeV1::Complete(set),
        GraphicImageInspectionOutcomeV1::Refused(refusal) => {
            EntityImageOutcomeV1::Refused(EntityImageRefusalV1::Inspection(refusal))
        }
    }
}

/// The real V2 four-format adapter: the accepted still port plus the real confined GIF worker.
///
/// One instance is bound to one already-resolved executable path, exactly like the V1 adapter, so
/// nothing about which program runs can be influenced by authored content. Each request is routed
/// by its declared format only:
///
/// * PNG/JPEG/WebP delegate to [`ConfinedImageInspectAdapterV1`] unchanged — the same confined
///   still decoder, the same parent validation and the same observation shape. No still path is
///   forked.
/// * GIF runs the real contained GIF worker through [`inspect_gif_bytes_with_executable_v1`] and is
///   translated into the inward animation evidence; no decoder handle and no adapter type crosses
///   into the application crate.
///
/// The adapter reports what the host proved. The application policy re-checks every claim, so a
/// successful return here is never by itself an accepted result.
pub struct ConfinedGraphicInspectAdapterV2 {
    executable: PathBuf,
    /// A closed forged-evidence mode, selectable only by the sensitivity seam.
    ///
    /// Production construction leaves this `None`, which always means `decode`.
    test_mode: Option<GifWorkerTestModeV1>,
}

impl ConfinedGraphicInspectAdapterV2 {
    /// Binds the adapter to one already-resolved executable path.
    #[must_use]
    pub fn new(executable: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            test_mode: None,
        }
    }

    /// Binds the adapter to this process's own image, which is how a composition root wires it.
    #[must_use]
    pub fn for_current_exe() -> Option<Self> {
        std::env::current_exe().ok().map(Self::new)
    }

    /// The closed sensitivity seam: the same adapter, with GIF inspection forced into one of the two
    /// existing forged-evidence modes.
    ///
    /// It exists only so a host test can prove that a correctly framed but wrongly digested worker
    /// answer cannot pass the V2 adapter or produce a partial candidate. There is no way to select
    /// `decode`, a mode string or any other behaviour through it, and production never calls it.
    #[must_use]
    pub fn with_test_mode(executable: impl Into<PathBuf>, mode: GifWorkerTestModeV1) -> Self {
        Self {
            executable: executable.into(),
            test_mode: Some(mode),
        }
    }

    /// The executable this adapter starts. Never influenced by authored input.
    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }
}

/// Maps one accepted V1 still refusal onto its V2 counterpart.
///
/// The mapping is one-to-one and exhaustive over the V1 set, so a new V1 refusal kind cannot be
/// silently swallowed as `PortRefused`.
const fn map_still_refusal_v2(
    kind: GraphicImageInspectRefusalKindV1,
) -> solo_application::GraphicImageInspectRefusalKindV2 {
    use solo_application::GraphicImageInspectRefusalKindV2 as V2;
    match kind {
        GraphicImageInspectRefusalKindV1::UnsupportedFormat => V2::UnsupportedFormat,
        GraphicImageInspectRefusalKindV1::EmptySource => V2::EmptySource,
        GraphicImageInspectRefusalKindV1::OversizeSource => V2::OversizeSource,
        GraphicImageInspectRefusalKindV1::PortRefused => V2::PortRefused,
        GraphicImageInspectRefusalKindV1::FormatMismatch => V2::FormatMismatch,
        GraphicImageInspectRefusalKindV1::MimeMismatch => V2::MimeMismatch,
        GraphicImageInspectRefusalKindV1::SourceDigestMismatch => V2::SourceDigestMismatch,
        GraphicImageInspectRefusalKindV1::DimensionsOutOfRange => V2::DimensionsOutOfRange,
        GraphicImageInspectRefusalKindV1::PixelCountOutOfRange => V2::PixelCountOutOfRange,
        GraphicImageInspectRefusalKindV1::DecodedBytesOutOfRange => V2::DecodedBytesOutOfRange,
        GraphicImageInspectRefusalKindV1::OutputDigestMismatch => V2::OutputDigestMismatch,
        GraphicImageInspectRefusalKindV1::UnverifiedWorker => V2::UnverifiedWorker,
    }
}

/// Translates one parent-validated GIF observation into the inward animation evidence.
fn gif_animation_evidence_v2(
    observation: &GifWorkerObservationV1,
) -> solo_application::GifAnimationEvidenceV2 {
    use solo_application::{GifDisposalV2, GifFrameEvidenceV2, GifLoopV2};
    let frames = observation
        .frames()
        .iter()
        .map(|frame| GifFrameEvidenceV2 {
            index: frame.index(),
            delay_centiseconds: frame.delay_centiseconds(),
            left: frame.left(),
            top: frame.top(),
            width: frame.width(),
            height: frame.height(),
            disposal: match frame.disposal() {
                crate::graphic_gif_inspection::GifDisposalV1::Unspecified => {
                    GifDisposalV2::Unspecified
                }
                crate::graphic_gif_inspection::GifDisposalV1::Keep => GifDisposalV2::Keep,
                crate::graphic_gif_inspection::GifDisposalV1::Background => {
                    GifDisposalV2::Background
                }
                crate::graphic_gif_inspection::GifDisposalV1::Previous => GifDisposalV2::Previous,
            },
            transparent_index: frame.transparent_index(),
            interlaced: frame.interlaced(),
            local_palette: frame.local_palette(),
            composite_digest: *frame.composite_digest(),
        })
        .collect();
    let loop_semantics = match observation.loop_semantics() {
        crate::graphic_gif_inspection::GifLoopV1::Absent => GifLoopV2::Absent,
        crate::graphic_gif_inspection::GifLoopV1::Finite(count) => GifLoopV2::Finite(count),
        crate::graphic_gif_inspection::GifLoopV1::Infinite => GifLoopV2::Infinite,
    };
    solo_application::GifAnimationEvidenceV2 {
        signature: observation.signature(),
        mime: crate::graphic_gif_inspection::GIF_MIME_V1,
        width: observation.width(),
        height: observation.height(),
        frames,
        loop_semantics,
        one_cycle_delay_ms: observation.one_cycle_delay_ms(),
        total_composited_pixels: observation.total_composited_pixels(),
        source_digest: *observation.source_digest(),
        transcript_digest: *observation.transcript_digest(),
        transcript_len: observation.transcript_len(),
        worker_pid: observation.worker_pid(),
        limits_verified: observation.limits_verified(),
        cleanup_verified: observation.cleanup_verified(),
        wall_ms: observation.wall_ms(),
    }
}

impl solo_application::GraphicImageInspectPortV2 for ConfinedGraphicInspectAdapterV2 {
    fn inspect_v2(
        &self,
        request: &solo_application::GraphicImageInspectRequestV2,
    ) -> solo_application::GraphicImageInspectPortOutcomeV2 {
        use solo_application::{
            GraphicImageInspectPortOutcomeV2 as Outcome,
            GraphicImageInspectRefusalKindV2 as Refusal, GraphicImageObservationV2 as Observation,
        };
        let declared = request.declared_format();
        if declared.is_gif() {
            let outcome = match self.test_mode {
                None => inspect_gif_bytes_with_executable_v1(&self.executable, request.bytes()),
                Some(mode) => {
                    inspect_gif_bytes_in_mode_v1(&self.executable, request.bytes(), mode.mode())
                }
            };
            return match outcome {
                Ok(observation) => Outcome::Inspected(Box::new(Observation::Animation(Box::new(
                    gif_animation_evidence_v2(&observation),
                )))),
                Err(_refusal) => Outcome::Refused(Refusal::PortRefused),
            };
        }
        let Some(still_format) = declared.as_still_v1() else {
            return Outcome::Refused(Refusal::UnsupportedFormat);
        };
        let still_request =
            GraphicImageInspectRequestV1::of(request.bytes().to_vec(), still_format);
        match ConfinedImageInspectAdapterV1::new(&self.executable).inspect(&still_request) {
            GraphicImageInspectPortOutcomeV1::Inspected(observation) => {
                Outcome::Inspected(Box::new(Observation::Still(observation)))
            }
            GraphicImageInspectPortOutcomeV1::Refused(kind) => {
                Outcome::Refused(map_still_refusal_v2(kind))
            }
        }
    }
}

/// Which stage refused one real V2 four-format candidate.
///
/// The two stages stay distinguishable: a confined acquisition refusal and an inspection refusal are
/// different facts, and collapsing them would hide which gate failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageRefusalV2 {
    /// The confined `R03.C2` source acquisition refused the candidate.
    SourceRead(solo_application::GraphicAssetReadRefusalV1),
    /// The V2 four-format policy, or the confined worker behind it, refused the candidate.
    Inspection(solo_application::GraphicImageInspectRefusalV2),
}

/// The closed result of one real source-read → V2 four-format inspection composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageOutcomeV2 {
    /// Every declared obligation was acquired and inspected.
    Complete(solo_application::GraphicImageInspectionSetV2),
    /// Exactly one typed refusal from the stage that refused.
    Refused(EntityImageRefusalV2),
}

impl EntityImageOutcomeV2 {
    /// Whether this outcome authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        solo_application::GRAPHIC_IMAGE_INSPECTION_V2_GRANTS_PUBLICATION_AUTHORITY
    }
}

/// Acquires and then inspects every declared Entity graphic source through the V2 four-format gate.
///
/// This is the real composition `SOLO-WO-113 R01` exists for: the confined `assets/` reader produces
/// owned bytes with physical identity, and the V2 policy turns them into inert still or animation
/// results — or exactly one whole-candidate refusal. Public, internal, unlisted and
/// descendant-suppressed obligations all take the identical path, because the inventory is the
/// complete authored set. Nothing here filters by exposure and nothing here publishes.
#[must_use]
pub fn inspect_entity_graphic_images_v2(
    root: &ContentRootCapability,
    inventory: &GraphicAssetObligationInventoryV1,
    executable: &Path,
) -> EntityImageOutcomeV2 {
    let source_set: GraphicAssetSourceSetV1 = match read_graphic_asset_bytes_v1(inventory, root) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            return EntityImageOutcomeV2::Refused(EntityImageRefusalV2::SourceRead(refusal));
        }
    };
    let adapter = ConfinedGraphicInspectAdapterV2::new(executable);
    match solo_application::inspect_graphic_image_source_set_v2(&source_set, &adapter) {
        solo_application::GraphicImageInspectionOutcomeV2::Complete(set) => {
            EntityImageOutcomeV2::Complete(set)
        }
        solo_application::GraphicImageInspectionOutcomeV2::Refused(refusal) => {
            EntityImageOutcomeV2::Refused(EntityImageRefusalV2::Inspection(refusal))
        }
    }
}

/// The authored format one host-test mixed candidate uses (`SOLO-WO-118 R18`).
///
/// A **slot-aware** candidate uses R17 slot selections; a **legacy** candidate uses the accepted
/// property envelope's unqualified graphic values, which is the byte-identical parity case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMixedFormatV1 {
    /// R17 slot selections (`supersede`/`append`), the opt-in format.
    SlotAware,
    /// The accepted legacy envelope format, which carries no slot identity.
    LegacyEnvelope,
    /// Both formats at once. The closed obligation plan must refuse this ambiguity rather than
    /// double-count or silently reinterpret one of them.
    Both,
}

/// One authored declaration for the closed R18 host-test seam.
#[derive(Debug, Clone, Copy)]
pub struct TestMixedDeclarationV1 {
    /// The EFS owner that authored it.
    pub owner: TestOwnerV1,
    /// The registered graphic role token: `logo`, `favicon`, `banner`, `background` or
    /// `social-preview`.
    pub role: &'static str,
    /// The explicit slot: `primary` for a singular role, any named slot for `background`.
    pub slot: &'static str,
    /// The explicit namespace: `entity` or `platform`. Never inferred from the reference text.
    pub namespace: &'static str,
    /// The authored reference text, with no namespace prefix.
    pub reference: &'static str,
    /// Whether the slot-aware selection supersedes rather than appends.
    pub supersede: bool,
    /// `public` or `internal`.
    pub exposure: &'static str,
}

/// The closed result of the R18 host-test seam.
///
/// It carries the accepted plan plus the resolved facts a host test needs, all as plain strings, so
/// the `solo-cli` integration-test crate can prove the real cascade and the real plan without a
/// `solo-domain` dependency of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestMixedCandidateV1 {
    plan: solo_application::MixedGraphicPlanV1,
    refusal: Option<&'static str>,
    slot_aware: bool,
    winners: Vec<(String, String)>,
    logo_lineage: usize,
    banner_suppressed: bool,
}

impl TestMixedCandidateV1 {
    /// The accepted plan, or an empty plan when the candidate itself was refused.
    #[must_use]
    pub const fn plan(&self) -> &solo_application::MixedGraphicPlanV1 {
        &self.plan
    }

    /// The stable token of the first construction/plan refusal, when the candidate was refused.
    #[must_use]
    pub const fn refusal(&self) -> Option<&'static str> {
        self.refusal
    }

    /// Whether the candidate is in the opt-in slot-aware format.
    #[must_use]
    pub const fn is_slot_aware(&self) -> bool {
        self.slot_aware
    }

    /// The representation's winning `(role, reference)` pairs, in registry then slot order.
    #[must_use]
    pub fn winners(&self) -> &[(String, String)] {
        &self.winners
    }

    /// The `graphic.logo` winner's lineage length at the representation, when one resolved.
    #[must_use]
    pub const fn logo_lineage(&self) -> usize {
        self.logo_lineage
    }

    /// Whether `graphic.banner` carries a recorded suppression at or above the representation.
    #[must_use]
    pub const fn banner_suppressed(&self) -> bool {
        self.banner_suppressed
    }
}

/// Builds one **real** validated R17 mixed candidate for the host tests (`SOLO-WO-118 R18`).
///
/// It runs the production constructors — the validated declaration stage, the verified approved
/// binding, the closed registration table, the slot selections, the whole-candidate resolution and
/// the closed obligation plan — and returns the accepted plan plus the resolved facts as plain
/// strings. It opens no file, reads no catalogue and grants no authority; the host test supplies
/// the real roots and the real reader separately.
#[must_use]
pub fn test_mixed_candidate_v1(
    registration: Option<(&'static str, &str)>,
    format: TestMixedFormatV1,
    declarations: &[TestMixedDeclarationV1],
) -> TestMixedCandidateV1 {
    match build_test_candidate_v1(registration, format, declarations) {
        Ok(built) => TestMixedCandidateV1 {
            plan: built.plan,
            refusal: None,
            slot_aware: built.slot_aware,
            winners: built.winners,
            logo_lineage: built.logo_lineage,
            banner_suppressed: built.banner_suppressed,
        },
        Err(token) => TestMixedCandidateV1 {
            plan: solo_application::MixedGraphicPlanV1::empty(),
            refusal: Some(token),
            slot_aware: format == TestMixedFormatV1::SlotAware,
            winners: Vec::new(),
            logo_lineage: 0,
            banner_suppressed: false,
        },
    }
}

/// The real validated candidate the host tests can build, kept opaque to the caller.
///
/// `SOLO-WO-118 R19` needs the accepted `GraphicCascadeInputV1` itself plus its scoped owner so the
/// real delivery-preparation path can run. The candidate stays here, inside the adapter crate: the
/// `solo-cli` test crate never names a domain type and never constructs one.
pub struct TestBuiltCandidateV1 {
    /// The validated R17 candidate.
    pub input: solo_domain::GraphicCascadeInputV1,
    /// The one representation owner the package is scoped to.
    pub target: solo_domain::QualifiedOwnerV1,
    /// The whole-graph mixed plan.
    pub plan: solo_application::MixedGraphicPlanV1,
    /// Whether the candidate opted into R17 slot awareness.
    pub slot_aware: bool,
    /// The representation's winning `(role, reference)` pairs.
    pub winners: Vec<(String, String)>,
    /// The `graphic.logo` winner's lineage length.
    pub logo_lineage: usize,
    /// Whether `graphic.banner` carries a recorded suppression.
    pub banner_suppressed: bool,
}

/// Builds one real validated candidate, or the stable token of its first refusal.
fn build_test_candidate_v1(
    registration: Option<(&'static str, &str)>,
    format: TestMixedFormatV1,
    declarations: &[TestMixedDeclarationV1],
) -> Result<TestBuiltCandidateV1, &'static str> {
    use solo_application::collect_mixed_graphic_obligations_v1;
    use solo_domain::{ApprovedPlatformAssetV1, ApprovedPlatformGraphicBindingV1};
    use solo_domain::{
        AuthoredPropertyValueV1, CurrentContextV1, DeclaredOwnerPropertiesV1, EfsOwnershipDraft,
        EntityIdV1, FacetDraft, FacetIdV1, GraphicCascadeInputV1, GraphicSlotSelectionV1,
        Inheritance, LocaleKeyV1, OwnerGraphicSelectionsV1, PageDraft, PageIdV1,
        PlatformAssetMediaClassV1, PlatformGraphicBaselineRecordV1, PlatformGraphicBaselineV1,
        PropertyDeclarationV1, PropertyKey, PropertyValueV1, SiteDraft, SiteIdV1,
        validate_declared_properties_v1,
    };

    let entity = EntityIdV1::parse("example").expect("a canonical Entity identifier");
    let mut page = PageDraft::new(PageIdV1::parse("home").expect("a canonical Page identifier"));
    page.add_representation(
        LocaleKeyV1::from_already_canonical("en-001").expect("a canonical full locale"),
    );
    let mut site = SiteDraft::new(SiteIdV1::parse("bali").expect("a canonical Site identifier"));
    site.add_page(page);
    let mut facet =
        FacetDraft::new(FacetIdV1::parse("main").expect("a canonical Facet identifier"));
    facet.add_site(site);
    let mut draft = EfsOwnershipDraft::new(entity.clone());
    draft.add_facet(facet);
    let graph = match draft.build() {
        Ok(graph) => graph,
        Err(_) => return Err("graph-invalid"),
    };

    // The verified approved binding: one asset when a registration is requested, otherwise a
    // binding that registers nothing. The digest text is the caller's, so a host test can pin the
    // real catalogue value or deliberately supply an unapproved one.
    let asset_id = registration.map_or("icons/icon-home.svg", |(id, _)| id);
    let asset_digest = registration.map_or_else(|| "0".repeat(64), |(_, digest)| digest.to_owned());
    let class = if asset_id.ends_with(".png") {
        PlatformAssetMediaClassV1::Png
    } else {
        PlatformAssetMediaClassV1::Svg
    };
    let asset = match ApprovedPlatformAssetV1::new(asset_id, &asset_digest, class) {
        Ok(asset) => asset,
        Err(_) => return Err("binding-invalid"),
    };
    let binding = match ApprovedPlatformGraphicBindingV1::new(
        "1.0.0",
        "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7",
        vec![asset],
    ) {
        Ok(binding) => binding,
        Err(_) => return Err("binding-invalid"),
    };
    let baseline = match registration {
        None => PlatformGraphicBaselineV1::empty(binding),
        Some((id, digest)) => {
            let reference = match solo_domain::AssetReferenceV1::parse(
                solo_domain::AssetNamespace::Platform,
                id,
            ) {
                Ok(reference) => reference,
                Err(_) => return Err("registration-invalid"),
            };
            let record = match PlatformGraphicBaselineRecordV1::new(
                PropertyKey::GraphicLogo,
                "primary",
                reference,
                digest,
            ) {
                Ok(record) => record,
                Err(refusal) => return Err(refusal.detail()),
            };
            match PlatformGraphicBaselineV1::new(binding, vec![record]) {
                Ok(baseline) => baseline,
                Err(refusal) => return Err(refusal.detail()),
            }
        }
    };
    let mut declared: Vec<DeclaredOwnerPropertiesV1> = Vec::new();
    let mut blocks: Vec<OwnerGraphicSelectionsV1> = Vec::new();
    for owner in [
        TestOwnerV1::Entity,
        TestOwnerV1::Facet,
        TestOwnerV1::Site,
        TestOwnerV1::Page,
        TestOwnerV1::Representation,
    ] {
        let entries: Vec<&TestMixedDeclarationV1> = declarations
            .iter()
            .filter(|declaration| declaration.owner == owner)
            .collect();
        if entries.is_empty() {
            continue;
        }
        let wants_selections = matches!(
            format,
            TestMixedFormatV1::SlotAware | TestMixedFormatV1::Both
        );
        let wants_envelope = matches!(
            format,
            TestMixedFormatV1::LegacyEnvelope | TestMixedFormatV1::Both
        );
        if wants_selections {
            let mut selections = Vec::new();
            for entry in &entries {
                let Some(role) = role_of(entry.role) else {
                    return Err("role-invalid");
                };
                let Some(namespace) = namespace_of(entry.namespace) else {
                    return Err("namespace-invalid");
                };
                let Ok(reference) =
                    solo_domain::AssetReferenceV1::parse(namespace, entry.reference)
                else {
                    return Err("reference-invalid");
                };
                let Some(exposure) = exposure_of(entry.exposure) else {
                    return Err("exposure-invalid");
                };
                let selection = if entry.supersede {
                    GraphicSlotSelectionV1::supersede(role, entry.slot, reference, exposure)
                } else {
                    GraphicSlotSelectionV1::append(role, entry.slot, reference, exposure)
                };
                match selection {
                    Ok(selection) => selections.push(selection),
                    Err(refusal) => return Err(refusal.detail()),
                }
            }
            match OwnerGraphicSelectionsV1::new(owner.owner_v1(), selections) {
                Ok(block) => blocks.push(block),
                Err(refusal) => return Err(refusal.detail()),
            }
        }
        if wants_envelope {
            {
                let mut per_role: Vec<(PropertyKey, Vec<AuthoredPropertyValueV1>)> = Vec::new();
                for entry in &entries {
                    let Some(role) = role_of(entry.role) else {
                        return Err("role-invalid");
                    };
                    let Some(namespace) = namespace_of(entry.namespace) else {
                        return Err("namespace-invalid");
                    };
                    let Ok(reference) =
                        solo_domain::AssetReferenceV1::parse(namespace, entry.reference)
                    else {
                        return Err("reference-invalid");
                    };
                    let Some(exposure) = exposure_of(entry.exposure) else {
                        return Err("exposure-invalid");
                    };
                    let Ok(authored) =
                        AuthoredPropertyValueV1::new(PropertyValueV1::graphic(reference), exposure)
                    else {
                        return Err("authored-invalid");
                    };
                    match per_role.iter_mut().find(|(key, _)| *key == role) {
                        Some((_, values)) => values.push(authored),
                        None => per_role.push((role, vec![authored])),
                    }
                }
                let entries_v1: Vec<(PropertyKey, PropertyDeclarationV1)> = per_role
                    .into_iter()
                    .map(|(role, values)| {
                        (
                            role,
                            PropertyDeclarationV1::new(
                                Inheritance::Inherit,
                                CurrentContextV1::Values(values),
                            ),
                        )
                    })
                    .collect();
                match DeclaredOwnerPropertiesV1::new(owner.owner_v1(), entries_v1) {
                    Ok(set) => declared.push(set),
                    Err(refusal) => return Err(refusal.detail()),
                }
            }
        }
    }

    let validated = match validate_declared_properties_v1(&graph, declared) {
        Ok(validated) => validated,
        Err(refusal) => return Err(refusal.detail()),
    };
    let input = match GraphicCascadeInputV1::new(validated, baseline, blocks) {
        Ok(input) => input,
        Err(refusal) => return Err(refusal.detail()),
    };
    let slot_aware = input.is_slot_aware();
    let plan = match collect_mixed_graphic_obligations_v1(&input) {
        Ok(plan) => plan,
        Err(refusal) => return Err(refusal.detail()),
    };
    let resolved = match input.resolve_graphic_cascade_all_v1() {
        Ok(resolved) => resolved,
        Err(failure) => return Err(failure.detail()),
    };
    let target = TestOwnerV1::Representation.owner_v1();
    let Some(record) = resolved.for_owner(&target) else {
        return Err("resolution-incomplete");
    };
    let mut winners: Vec<(String, String)> = Vec::new();
    let mut logo_lineage = 0_usize;
    let mut banner_suppressed = false;
    for role in PropertyKey::ALL.into_iter().filter(|key| key.is_graphic()) {
        let Some(role_record) = record.role(role) else {
            continue;
        };
        for slot in role_record.slots() {
            winners.push((role.name().to_owned(), slot.source().reference().to_owned()));
            if role == PropertyKey::GraphicLogo {
                logo_lineage = slot.lineage().len();
            }
        }
        if role == PropertyKey::GraphicBanner && role_record.suppressed_at().is_some() {
            banner_suppressed = true;
        }
    }
    Ok(TestBuiltCandidateV1 {
        input,
        target,
        plan,
        slot_aware,
        winners,
        logo_lineage,
        banner_suppressed,
    })
}

/// Maps one host-test role token to the registered graphic role.
fn role_of(token: &str) -> Option<solo_domain::PropertyKey> {
    solo_domain::PropertyKey::from_name(&format!("graphic.{token}"))
}

/// Maps one host-test namespace token to the explicit namespace.
fn namespace_of(token: &str) -> Option<solo_domain::AssetNamespace> {
    solo_domain::AssetNamespace::from_name(token)
}

/// Maps one host-test exposure token to the authored exposure.
fn exposure_of(token: &str) -> Option<solo_domain::Exposure> {
    solo_domain::Exposure::from_name(token)
}

/// Which EFS owner a host-test declaration is authored on.
///
/// It exists so the `solo-cli` integration-test crate — which has no `solo-domain` dependency and
/// whose manifest is outside this order's ledger — can still exercise the **real** validated
/// inventory path rather than a hand-built stand-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOwnerV1 {
    /// The Entity itself.
    Entity,
    /// The single Facet directly under the Entity.
    Facet,
    /// One Site inside the single Facet.
    Site,
    /// One Page inside that Site.
    Page,
    /// The `en-001` representation of that Page.
    Representation,
}

impl TestOwnerV1 {
    fn owner_v1(self) -> solo_domain::QualifiedOwnerV1 {
        use solo_domain::{EntityIdV1, FacetIdV1, PageIdV1, QualifiedOwnerV1, SiteIdV1};
        let entity = EntityIdV1::parse("example").expect("canonical test Entity");
        let facet = FacetIdV1::parse("main").expect("canonical test Facet");
        let site = SiteIdV1::parse("bali").expect("canonical test Site");
        let page = PageIdV1::parse("home").expect("canonical test Page");
        match self {
            Self::Entity => QualifiedOwnerV1::entity(entity),
            Self::Facet => QualifiedOwnerV1::facet(entity, facet),
            Self::Site => QualifiedOwnerV1::site(entity, facet, site),
            Self::Page => QualifiedOwnerV1::page(entity, facet, site, page),
            Self::Representation => QualifiedOwnerV1::representation(
                entity,
                facet,
                site,
                page,
                solo_domain::LocaleKeyV1::from_already_canonical("en-001")
                    .expect("canonical test locale"),
            ),
        }
    }
}

/// Builds one real, whole-graph validated inventory for host tests.
///
/// Each entry is `(owner, role, entity reference, exposure)`, where the role is `logo`, `favicon`,
/// `banner`, `background` or `social-preview` and the exposure is `public` or `internal`. It runs the
/// same validators the production inventory uses, so a test cannot smuggle in an invalid
/// declaration; it produces only an inert inventory, decodes nothing and grants no authority. `None`
/// means the specification was itself invalid.
#[must_use]
pub fn test_inventory_v1(
    declarations: &[(TestOwnerV1, &str, &str, &str)],
) -> Option<GraphicAssetObligationInventoryV1> {
    test_inventory_with_inheritance_v1(declarations, solo_domain::Inheritance::Inherit)
}

/// Builds one validated inventory whose declarations all **suppress** inheritance.
///
/// A suppressing declaration excludes every inherited value for its key while keeping its own
/// authored values, which is how a descendant narrows an inherited selection. It runs the same
/// validators as production, is used only by the V2 host tests, decodes nothing and grants no
/// authority. `None` means the specification was itself invalid.
#[must_use]
pub fn test_inventory_suppressed_v1(
    declarations: &[(TestOwnerV1, &str, &str, &str)],
) -> Option<GraphicAssetObligationInventoryV1> {
    test_inventory_with_inheritance_v1(declarations, solo_domain::Inheritance::Suppress)
}

fn test_inventory_with_inheritance_v1(
    declarations: &[(TestOwnerV1, &str, &str, &str)],
    inheritance: solo_domain::Inheritance,
) -> Option<GraphicAssetObligationInventoryV1> {
    use solo_application::{
        PropertyResolutionRequestV1, collect_graphic_asset_obligations_v1,
        resolve_declared_properties_v1,
    };
    use solo_domain::{
        AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CurrentContextV1,
        DeclaredOwnerPropertiesV1, EfsOwnershipDraft, Exposure, FacetDraft, LocaleKeyV1, PageDraft,
        PropertyDeclarationV1, PropertyKey, PropertyValueV1, QualifiedOwnerV1, SiteDraft,
    };

    let entity = solo_domain::EntityIdV1::parse("example").ok()?;
    let mut page = PageDraft::new(solo_domain::PageIdV1::parse("home").ok()?);
    page.add_representation(LocaleKeyV1::from_already_canonical("en-001").ok()?);
    let mut site = SiteDraft::new(solo_domain::SiteIdV1::parse("bali").ok()?);
    site.add_page(page);
    let mut facet = FacetDraft::new(solo_domain::FacetIdV1::parse("main").ok()?);
    facet.add_site(site);
    let mut draft = EfsOwnershipDraft::new(entity);
    draft.add_facet(facet);
    let graph = draft.build().ok()?;

    let mut owners: Vec<(QualifiedOwnerV1, Vec<(PropertyKey, PropertyDeclarationV1)>)> = Vec::new();
    for (slot, role, reference, exposure) in declarations {
        let key = match *role {
            "logo" => PropertyKey::GraphicLogo,
            "favicon" => PropertyKey::GraphicFavicon,
            "banner" => PropertyKey::GraphicBanner,
            "background" => PropertyKey::GraphicBackground,
            "social-preview" => PropertyKey::GraphicSocialPreview,
            _ => return None,
        };
        let exposure = match *exposure {
            "public" => Exposure::Public,
            "internal" => Exposure::Internal,
            _ => return None,
        };
        let parsed = AssetReferenceV1::parse(AssetNamespace::Entity, reference).ok()?;
        let value = PropertyValueV1::graphic(parsed);
        let authored = AuthoredPropertyValueV1::new(value, exposure).ok()?;
        let declaration =
            PropertyDeclarationV1::new(inheritance, CurrentContextV1::Values(vec![authored]));
        let owner = slot.owner_v1();
        match owners.iter_mut().find(|(existing, _)| *existing == owner) {
            Some((_, entries)) => entries.push((key, declaration)),
            None => owners.push((owner, vec![(key, declaration)])),
        }
    }

    let declared: Vec<DeclaredOwnerPropertiesV1> = owners
        .into_iter()
        .map(|(owner, entries)| DeclaredOwnerPropertiesV1::new(owner, entries))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let target = TestOwnerV1::Page.owner_v1();
    let candidate =
        resolve_declared_properties_v1(PropertyResolutionRequestV1::new(&graph, declared, target))
            .ok()?;
    collect_graphic_asset_obligations_v1(&candidate).ok()
}

/// The one closed GIF worker token. It is deliberately **not** a `GraphicImageFormatV1` variant:
/// the still-image enum keeps its three accepted formats until a separately ordered V2 policy.
pub const GIF_WORKER_TOKEN_V1: &str = "gif";

/// The separate, versioned GIF evidence frame. It never reuses the still-image frame.
pub const GIF_WORKER_FRAME_PREFIX_V1: &str = "SOLO-GIF-WORKER-V1";

pub const GIF_WORKER_MODE_DECODE_V1: &str = "decode";

/// A closed test seam: compose correctly but claim a different source digest.
pub const GIF_WORKER_MODE_WRONG_SOURCE_V1: &str = "gif-wrong-source";

/// A closed test seam: compose correctly but claim a different transcript digest.
///
/// This exists so the parent's independent transcript hashing is provably load-bearing.
pub const GIF_WORKER_MODE_WRONG_TRANSCRIPT_V1: &str = "gif-wrong-transcript";

/// The fixed GIF stdout cap: the bounded trace plus the bounded header, never caller-chosen.
pub const MAX_GIF_WORKER_STDOUT_BYTES_V1: u64 =
    crate::graphic_gif_inspection::MAX_GIF_TRACE_BYTES_V1 as u64 + 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GifWorkerModeV1 {
    Decode,
    WrongSource,
    WrongTranscript,
}

fn flipped_digest_v1(digest: MediaWorkerDigestV1) -> MediaWorkerDigestV1 {
    let mut altered = *digest.as_bytes();
    altered[0] ^= 0xff;
    MediaWorkerDigestV1::from_bytes(altered)
}

/// The child half of the confined GIF worker: prove confinement, then decode every frame.
///
/// Confinement is established before a single untrusted byte is read, exactly as the still-image
/// worker does. The raw GIF decoder is crate-internal and reachable only from here, so a hostile
/// animation is never parsed in the trusted parent.
fn run_gif_worker_child_v1(claimed: MediaWorkerDigestV1, mode: GifWorkerModeV1) -> u8 {
    let Some(confinement) = crate::media_worker_host::child_image_worker_confinement_v1() else {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    };
    if !confinement.matches_profile_v1() {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    }

    let mut source: Vec<u8> = Vec::new();
    let limit = crate::graphic_gif_inspection::MAX_GIF_SOURCE_BYTES_V1 + 1;
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    if handle
        .by_ref()
        .take(limit)
        .read_to_end(&mut source)
        .is_err()
    {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    let transferred = u64::try_from(source.len()).unwrap_or(u64::MAX);
    if source.is_empty() || transferred > crate::graphic_gif_inspection::MAX_GIF_SOURCE_BYTES_V1 {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    let actual_source = sha256_media_worker_bytes_v1(&source);
    if actual_source != claimed {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    let Ok(evidence) = crate::graphic_gif_inspection::inspect_gif_animation_v1(&source) else {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    };
    let transcript = evidence.transcript_v1();
    if transcript.len() > crate::graphic_gif_inspection::MAX_GIF_TRACE_BYTES_V1 {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    }
    let actual_transcript = sha256_media_worker_bytes_v1(transcript.as_bytes());

    let claimed_source = match mode {
        GifWorkerModeV1::WrongSource => flipped_digest_v1(actual_source),
        GifWorkerModeV1::Decode | GifWorkerModeV1::WrongTranscript => actual_source,
    };
    let claimed_transcript = match mode {
        GifWorkerModeV1::WrongTranscript => flipped_digest_v1(actual_transcript),
        GifWorkerModeV1::Decode | GifWorkerModeV1::WrongSource => actual_transcript,
    };
    let loop_count = match evidence.loop_semantics() {
        crate::graphic_gif_inspection::GifLoopV1::Finite(count) => count.to_string(),
        _ => "none".to_string(),
    };
    let header = format!(
        "{prefix} format={format} mime={mime} confinement=verified pid={pid} width={width} height={height} frames={frames} loop={lp} loop_count={lc} one_cycle_delay_ms={delay} total_composited_pixels={pixels} source_sha256={source} transcript_len={length} transcript_sha256={transcript}\n",
        prefix = GIF_WORKER_FRAME_PREFIX_V1,
        format = GIF_WORKER_TOKEN_V1,
        mime = crate::graphic_gif_inspection::GIF_MIME_V1,
        pid = std::process::id(),
        width = evidence.width(),
        height = evidence.height(),
        frames = evidence.frame_count(),
        lp = evidence.loop_semantics().token(),
        lc = loop_count,
        delay = evidence.one_cycle_delay_ms(),
        pixels = evidence.total_composited_pixels(),
        source = claimed_source.to_hex(),
        length = transcript.len(),
        transcript = claimed_transcript.to_hex(),
    );
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    if out.write_all(header.as_bytes()).is_err()
        || out.write_all(transcript.as_bytes()).is_err()
        || out.flush().is_err()
    {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    0
}
/// Which gate refused one confined GIF worker observation. Every refusal is whole-asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GifWorkerRefusalV1 {
    SourceRejected,
    SupervisorRefused,
    WorkerExited,
    UnverifiedConfinement,
    OutputTooLarge,
    FrameMalformed,
    TranscriptMalformed,
    TranscriptDigestMismatch,
    SourceDigestMismatch,
    IdentityMismatch,
}

impl GifWorkerRefusalV1 {
    /// Stable ASCII refusal token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::SourceRejected => "gif-worker-source-rejected",
            Self::SupervisorRefused => "gif-worker-supervisor-refused",
            Self::WorkerExited => "gif-worker-exited",
            Self::UnverifiedConfinement => "gif-worker-unverified-confinement",
            Self::OutputTooLarge => "gif-worker-output-too-large",
            Self::FrameMalformed => "gif-worker-frame-malformed",
            Self::TranscriptMalformed => "gif-worker-transcript-malformed",
            Self::TranscriptDigestMismatch => "gif-worker-transcript-digest-mismatch",
            Self::SourceDigestMismatch => "gif-worker-source-digest-mismatch",
            Self::IdentityMismatch => "gif-worker-identity-mismatch",
        }
    }
}

/// One ordered, parent-validated frame record from the strict transcript.
///
/// Every field is parsed from fixed-position tokens in the received bytes and cross-checked
/// against the parent's own geometry and arithmetic, so a host test can assert authored timing,
/// disposal, palette, interlace, offsets and composite digests without any raw decoder access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GifFrameEvidenceV2 {
    index: usize,
    delay_centiseconds: u16,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    disposal: crate::graphic_gif_inspection::GifDisposalV1,
    transparent_index: Option<u8>,
    interlaced: bool,
    local_palette: bool,
    composite_digest: MediaWorkerDigestV1,
}

impl GifFrameEvidenceV2 {
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
    #[must_use]
    pub const fn delay_centiseconds(&self) -> u16 {
        self.delay_centiseconds
    }
    #[must_use]
    pub const fn left(&self) -> u16 {
        self.left
    }
    #[must_use]
    pub const fn top(&self) -> u16 {
        self.top
    }
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
    #[must_use]
    pub const fn disposal(&self) -> crate::graphic_gif_inspection::GifDisposalV1 {
        self.disposal
    }
    #[must_use]
    pub const fn transparent_index(&self) -> Option<u8> {
        self.transparent_index
    }
    #[must_use]
    pub const fn interlaced(&self) -> bool {
        self.interlaced
    }
    #[must_use]
    pub const fn local_palette(&self) -> bool {
        self.local_palette
    }
    #[must_use]
    pub const fn composite_digest(&self) -> &MediaWorkerDigestV1 {
        &self.composite_digest
    }
}

/// A typed, nonpublishing, parent-validated observation of one complete GIF animation.
///
/// It is only ever produced by the trusted parent after the confined child returned exit 0 and
/// the parent re-verified identity, digests, geometry, arithmetic and the complete transcript
/// grammar. It carries no raw decoder handle and grants no publication authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GifWorkerObservationV1 {
    signature: &'static str,
    width: u16,
    height: u16,
    frames: Vec<GifFrameEvidenceV2>,
    loop_semantics: crate::graphic_gif_inspection::GifLoopV1,
    one_cycle_delay_ms: u64,
    total_composited_pixels: u64,
    source_digest: MediaWorkerDigestV1,
    transcript_digest: MediaWorkerDigestV1,
    transcript_len: usize,
    worker_pid: u32,
    limits_verified: bool,
    cleanup_verified: bool,
    wall_ms: u64,
}

impl GifWorkerObservationV1 {
    #[must_use]
    pub const fn signature(&self) -> &'static str {
        self.signature
    }
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }
    #[must_use]
    pub fn frames(&self) -> &[GifFrameEvidenceV2] {
        &self.frames
    }
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
    #[must_use]
    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1
    }
    #[must_use]
    pub const fn loop_semantics(&self) -> crate::graphic_gif_inspection::GifLoopV1 {
        self.loop_semantics
    }
    #[must_use]
    pub const fn one_cycle_delay_ms(&self) -> u64 {
        self.one_cycle_delay_ms
    }
    #[must_use]
    pub const fn total_composited_pixels(&self) -> u64 {
        self.total_composited_pixels
    }
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }
    #[must_use]
    pub const fn transcript_digest(&self) -> &MediaWorkerDigestV1 {
        &self.transcript_digest
    }
    #[must_use]
    pub const fn transcript_len(&self) -> usize {
        self.transcript_len
    }
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }
    #[must_use]
    pub const fn limits_verified(&self) -> bool {
        self.limits_verified
    }
    #[must_use]
    pub const fn cleanup_verified(&self) -> bool {
        self.cleanup_verified
    }
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }
}
const GIF_WORKER_KEYS_V1: [&str; 14] = [
    "format",
    "mime",
    "confinement",
    "pid",
    "width",
    "height",
    "frames",
    "loop",
    "loop_count",
    "one_cycle_delay_ms",
    "total_composited_pixels",
    "source_sha256",
    "transcript_len",
    "transcript_sha256",
];

fn decimal_v1(text: &str) -> Option<u64> {
    if text.is_empty() || text.len() > 20 || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if text.len() > 1 && text.starts_with('0') {
        return None;
    }
    text.parse::<u64>().ok()
}

fn bounded_u16_v1(text: &str) -> Option<u16> {
    decimal_v1(text).and_then(|value| u16::try_from(value).ok())
}

fn lower_hex_digest_v1(text: &str) -> Option<MediaWorkerDigestV1> {
    if text.len() != 64
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return None;
    }
    MediaWorkerDigestV1::parse_lower_hex(text)
}

/// The bounded parent-held GIF header facts: signature prefix and logical screen size.
///
/// This reads ten fixed bytes. It is not a decoder, a compositor or a GIF parser.
fn parent_gif_screen_v1(source: &[u8]) -> Option<(&'static str, u16, u16)> {
    let signature = match source.get(0..6)? {
        b"GIF87a" => "GIF87a",
        b"GIF89a" => "GIF89a",
        _ => return None,
    };
    let width = u16::from_le_bytes([*source.get(6)?, *source.get(7)?]);
    let height = u16::from_le_bytes([*source.get(8)?, *source.get(9)?]);
    if width == 0 || height == 0 {
        return None;
    }
    Some((signature, width, height))
}

/// The supervised GIF observation for the current executable.
///
/// # Errors
///
/// A closed [`GifWorkerRefusalV1`]; see [`inspect_gif_bytes_with_executable_v1`].
pub fn inspect_gif_bytes_v1(source: &[u8]) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1> {
    let Ok(executable) = std::env::current_exe() else {
        return Err(GifWorkerRefusalV1::SupervisorRefused);
    };
    inspect_gif_bytes_with_executable_v1(&executable, source)
}

/// The one narrow trusted composition-root seam.
///
/// It exists because an integration test's `current_exe` is the Rust harness, not the product
/// image. The caller supplies the already-known executable path — `CARGO_BIN_EXE_solo-cli` in the
/// host tests and `current_exe` in production. Neither the path nor any argument may derive from
/// authored content, and this function shares exactly the same parent validation.
///
/// # Errors
///
/// A closed [`GifWorkerRefusalV1`]: rejected source, unreadable header, refused spawn, unproven
/// confinement, non-zero exit, oversized output, malformed header or transcript, forged digest,
/// identity mismatch or impossible cross-field arithmetic.
/// The closed worker argument token for one parent-selected GIF mode.
///
/// `Decode` is production and passes no mode token at all; the two forged modes reuse the tokens the
/// accepted child already recognises. Nothing here is free text.
const fn gif_parent_mode_token_v1(mode: GifWorkerModeV1) -> Option<&'static str> {
    match mode {
        GifWorkerModeV1::Decode => None,
        GifWorkerModeV1::WrongSource => Some(GIF_WORKER_MODE_WRONG_SOURCE_V1),
        GifWorkerModeV1::WrongTranscript => Some(GIF_WORKER_MODE_WRONG_TRANSCRIPT_V1),
    }
}

/// The two forged-evidence modes this crate exposes for sensitivity testing only.
///
/// Deliberately narrower than [`GifWorkerModeV1`]: there is no public way to request `Decode`
/// through the test seam, and no public way to request either forged mode through production.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GifWorkerTestModeV1 {
    /// Compose correctly but claim a different source digest.
    WrongSourceDigest,
    /// Compose correctly but claim a different transcript digest.
    WrongTranscriptDigest,
}

impl GifWorkerTestModeV1 {
    /// The internal closed mode this test mode selects.
    const fn mode(self) -> GifWorkerModeV1 {
        match self {
            Self::WrongSourceDigest => GifWorkerModeV1::WrongSource,
            Self::WrongTranscriptDigest => GifWorkerModeV1::WrongTranscript,
        }
    }
}

/// Runs one GIF worker attempt in exactly one closed mode and applies the production validation.
///
/// This is the single implementation of the GIF inspect path: the production entry point always
/// selects `Decode`, and the bounded test seam can select only the two existing forged modes. Both
/// go through the identical parent validator, so the seam cannot prove anything production does not
/// already enforce.
fn inspect_gif_bytes_in_mode_v1(
    executable: &std::path::Path,
    source: &[u8],
    mode: GifWorkerModeV1,
) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1> {
    use GifWorkerRefusalV1 as Refusal;
    if source.is_empty()
        || u64::try_from(source.len()).unwrap_or(u64::MAX)
            > crate::graphic_gif_inspection::MAX_GIF_SOURCE_BYTES_V1
    {
        return Err(Refusal::SourceRejected);
    }
    let screen = parent_gif_screen_v1(source).ok_or(Refusal::SourceRejected)?;
    let source_digest = sha256_media_worker_bytes_v1(source);
    let source_hex = source_digest.to_hex();
    let run = match gif_parent_mode_token_v1(mode) {
        None => run_confined_image_worker_v1(executable, GIF_WORKER_TOKEN_V1, &source_hex, source),
        Some(mode_token) => run_confined_image_worker_mode_v1(
            executable,
            GIF_WORKER_TOKEN_V1,
            &source_hex,
            Some(mode_token),
            source,
            &mut crate::media_worker_host::N102FaultRunV1::production(),
        ),
    }
    .map_err(|_| Refusal::SupervisorRefused)?;
    if !run.limits_verified() || !run.cleanup_verified() || run.child_pid() == 0 {
        return Err(Refusal::UnverifiedConfinement);
    }
    if run.exit_code() != Some(0) {
        return Err(Refusal::WorkerExited);
    }
    if u64::try_from(run.stdout().len()).unwrap_or(u64::MAX) > MAX_GIF_WORKER_STDOUT_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    let mut observation =
        validate_gif_worker_frame_v1(run.stdout(), source_digest, run.child_pid(), screen)?;
    observation.limits_verified = true;
    observation.cleanup_verified = true;
    observation.wall_ms = run.wall_ms();
    Ok(observation)
}

/// The production GIF inspection seam for an already-known executable.
///
/// # Errors
///
/// A closed [`GifWorkerRefusalV1`]: rejected source, unreadable header, refused spawn, unproven
/// confinement, non-zero exit, oversized output, malformed header or transcript, forged digest,
/// identity mismatch or impossible cross-field arithmetic.
pub fn inspect_gif_bytes_with_executable_v1(
    executable: &std::path::Path,
    source: &[u8],
) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1> {
    inspect_gif_bytes_in_mode_v1(executable, source, GifWorkerModeV1::Decode)
}

/// The closed GIF sensitivity seam: run the real worker in one of the two forged-evidence modes.
///
/// It exists only so a test can prove that the parent's independent source and transcript hashing
/// rejects a correctly composed but wrongly digested answer. It cannot select `Decode`, cannot
/// carry a caller-chosen mode string and cannot bypass any validation.
///
/// # Errors
///
/// A closed [`GifWorkerRefusalV1`], never a partially accepted animation.
pub fn inspect_gif_bytes_with_test_mode_v1(
    executable: &std::path::Path,
    source: &[u8],
    mode: GifWorkerTestModeV1,
) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1> {
    inspect_gif_bytes_in_mode_v1(executable, source, mode.mode())
}

/// One strict cursor over the received transcript lines. No searching, no early break.
struct TranscriptCursorV1<'a> {
    lines: &'a [&'a str],
    at: usize,
}

impl<'a> TranscriptCursorV1<'a> {
    fn next_line(&mut self) -> Result<&'a str, GifWorkerRefusalV1> {
        let line = *self
            .lines
            .get(self.at)
            .ok_or(GifWorkerRefusalV1::TranscriptMalformed)?;
        self.at += 1;
        Ok(line)
    }

    fn expect(&mut self, exact: &str) -> Result<(), GifWorkerRefusalV1> {
        if self.next_line()? == exact {
            Ok(())
        } else {
            Err(GifWorkerRefusalV1::TranscriptMalformed)
        }
    }

    fn value(&mut self, key: &str) -> Result<&'a str, GifWorkerRefusalV1> {
        self.next_line()?
            .strip_prefix(key)
            .and_then(|rest| rest.strip_prefix('='))
            .ok_or(GifWorkerRefusalV1::TranscriptMalformed)
    }
}

fn validate_gif_worker_frame_v1(
    stdout: &[u8],
    source_digest: MediaWorkerDigestV1,
    child_pid: u32,
    screen: (&'static str, u16, u16),
) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1> {
    use crate::graphic_gif_inspection::GIF_MIME_V1;
    use GifWorkerRefusalV1 as Refusal;
    if stdout.contains(&b'\r') {
        return Err(Refusal::FrameMalformed);
    }
    let split = stdout
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(Refusal::FrameMalformed)?;
    let header = std::str::from_utf8(&stdout[..split]).map_err(|_| Refusal::FrameMalformed)?;
    if header.len() > crate::graphic_gif_inspection::MAX_GIF_TRACE_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    let rest = header
        .strip_prefix(GIF_WORKER_FRAME_PREFIX_V1)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or(Refusal::FrameMalformed)?;
    let tokens: Vec<&str> = rest.split(' ').collect();
    if tokens.len() != GIF_WORKER_KEYS_V1.len() {
        return Err(Refusal::FrameMalformed);
    }
    let mut values: Vec<&str> = Vec::with_capacity(tokens.len());
    for (token, key) in tokens.iter().zip(GIF_WORKER_KEYS_V1) {
        let value = token
            .strip_prefix(key)
            .and_then(|rest| rest.strip_prefix('='))
            .ok_or(Refusal::FrameMalformed)?;
        values.push(value);
    }
    if values[0] != GIF_WORKER_TOKEN_V1 || values[1] != GIF_MIME_V1 || values[2] != "verified" {
        return Err(Refusal::FrameMalformed);
    }
    let pid = decimal_v1(values[3])
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(Refusal::FrameMalformed)?;
    if pid != child_pid {
        return Err(Refusal::IdentityMismatch);
    }
    let width = bounded_u16_v1(values[4]).ok_or(Refusal::FrameMalformed)?;
    let height = bounded_u16_v1(values[5]).ok_or(Refusal::FrameMalformed)?;
    if width != screen.1 || height != screen.2 {
        return Err(Refusal::FrameMalformed);
    }
    let frames = decimal_v1(values[6])
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Refusal::FrameMalformed)?;
    if frames == 0 || frames > crate::graphic_gif_inspection::MAX_GIF_FRAMES_V1 {
        return Err(Refusal::FrameMalformed);
    }
    let loop_token = values[7];
    let loop_count = values[8];
    match loop_token {
        "absent" | "infinite" => {
            if loop_count != "none" {
                return Err(Refusal::FrameMalformed);
            }
        }
        "finite" => {
            if bounded_u16_v1(loop_count).is_none() {
                return Err(Refusal::FrameMalformed);
            }
        }
        _ => return Err(Refusal::FrameMalformed),
    }
    let delay = decimal_v1(values[9]).ok_or(Refusal::FrameMalformed)?;
    let pixels = decimal_v1(values[10]).ok_or(Refusal::FrameMalformed)?;
    let claimed_source = lower_hex_digest_v1(values[11]).ok_or(Refusal::FrameMalformed)?;
    if claimed_source != source_digest {
        return Err(Refusal::SourceDigestMismatch);
    }
    let body = &stdout[split + 1..];
    let transcript_len = decimal_v1(values[12])
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Refusal::FrameMalformed)?;
    if transcript_len != body.len() {
        return Err(Refusal::TranscriptMalformed);
    }
    if transcript_len > crate::graphic_gif_inspection::MAX_GIF_TRACE_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    let claimed_transcript = lower_hex_digest_v1(values[13]).ok_or(Refusal::FrameMalformed)?;
    let received = sha256_media_worker_bytes_v1(body);
    if received != claimed_transcript {
        return Err(Refusal::TranscriptDigestMismatch);
    }

    // The digest proves these are the bytes the worker meant to send. The complete grammar below
    // proves those bytes are a consistent animation document; a consistent forged digest does not
    // survive an inconsistent document.
    let records = parse_gif_transcript_v1(body, screen, source_digest, frames)?;
    let delay_sum: u64 = records
        .iter()
        .map(|frame| u64::from(frame.delay_centiseconds()))
        .sum::<u64>()
        .saturating_mul(10);
    if delay_sum != delay {
        return Err(Refusal::TranscriptMalformed);
    }
    let canvas = u64::from(width) * u64::from(height);
    if u64::try_from(records.len()).unwrap_or(u64::MAX) * canvas != pixels {
        return Err(Refusal::TranscriptMalformed);
    }
    let loop_semantics = match loop_token {
        "infinite" => crate::graphic_gif_inspection::GifLoopV1::Infinite,
        "absent" => crate::graphic_gif_inspection::GifLoopV1::Absent,
        _ => crate::graphic_gif_inspection::GifLoopV1::Finite(
            bounded_u16_v1(loop_count).ok_or(Refusal::FrameMalformed)?,
        ),
    };
    Ok(GifWorkerObservationV1 {
        signature: screen.0,
        width,
        height,
        frames: records,
        loop_semantics,
        one_cycle_delay_ms: delay,
        total_composited_pixels: pixels,
        source_digest,
        transcript_digest: received,
        transcript_len,
        worker_pid: pid,
        limits_verified: false,
        cleanup_verified: false,
        wall_ms: 0,
    })
}
/// The complete, strictly ordered transcript grammar.
///
/// Every line is consumed exactly once in a fixed order, every value is canonical, and the
/// document must end with the single terminator line and nothing else. There is no searching,
/// no duplicate tolerance and no early break.
fn parse_gif_transcript_v1(
    body: &[u8],
    screen: (&'static str, u16, u16),
    source_digest: MediaWorkerDigestV1,
    expected_frames: usize,
) -> Result<Vec<GifFrameEvidenceV2>, GifWorkerRefusalV1> {
    use crate::graphic_gif_inspection::{
        GIF_MIME_V1, GifDisposalV1, MAX_GIF_ONE_CYCLE_DELAY_MS_V1,
        MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1, MAX_GIF_TRACE_BYTES_V1,
    };
    use GifWorkerRefusalV1 as Refusal;
    if body.len() > MAX_GIF_TRACE_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    if body.last() != Some(&b'\n') {
        return Err(Refusal::TranscriptMalformed);
    }
    if body
        .iter()
        .any(|byte| *byte == b'\r' || (*byte < 0x20 && *byte != b'\n'))
    {
        return Err(Refusal::TranscriptMalformed);
    }
    let text = std::str::from_utf8(body).map_err(|_| Refusal::TranscriptMalformed)?;
    let trimmed = text
        .strip_suffix('\n')
        .ok_or(Refusal::TranscriptMalformed)?;
    let lines: Vec<&str> = trimmed.split('\n').collect();
    if lines.iter().any(|line| line.is_empty()) {
        return Err(Refusal::TranscriptMalformed);
    }
    let mut cursor = TranscriptCursorV1 {
        lines: &lines,
        at: 0,
    };
    cursor.expect("SOLO-GIF-ANIMATION-V1")?;
    if cursor.value("signature")? != screen.0 {
        return Err(Refusal::TranscriptMalformed);
    }
    if cursor.value("mime")? != GIF_MIME_V1 {
        return Err(Refusal::TranscriptMalformed);
    }
    let width = bounded_u16_v1(cursor.value("width")?).ok_or(Refusal::TranscriptMalformed)?;
    let height = bounded_u16_v1(cursor.value("height")?).ok_or(Refusal::TranscriptMalformed)?;
    if width != screen.1 || height != screen.2 || width == 0 || height == 0 {
        return Err(Refusal::TranscriptMalformed);
    }
    let frames = decimal_v1(cursor.value("frames")?)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Refusal::TranscriptMalformed)?;
    if frames == 0
        || frames > crate::graphic_gif_inspection::MAX_GIF_FRAMES_V1
        || frames != expected_frames
    {
        return Err(Refusal::TranscriptMalformed);
    }
    let loop_token = cursor.value("loop")?;
    let loop_semantics = match loop_token {
        "absent" => crate::graphic_gif_inspection::GifLoopV1::Absent,
        "infinite" => crate::graphic_gif_inspection::GifLoopV1::Infinite,
        "finite" => {
            let count =
                bounded_u16_v1(cursor.value("loop_count")?).ok_or(Refusal::TranscriptMalformed)?;
            if count == 0 {
                return Err(Refusal::TranscriptMalformed);
            }
            crate::graphic_gif_inspection::GifLoopV1::Finite(count)
        }
        _ => return Err(Refusal::TranscriptMalformed),
    };
    let delay =
        decimal_v1(cursor.value("one_cycle_delay_ms")?).ok_or(Refusal::TranscriptMalformed)?;
    if delay > MAX_GIF_ONE_CYCLE_DELAY_MS_V1 {
        return Err(Refusal::TranscriptMalformed);
    }
    let pixels =
        decimal_v1(cursor.value("total_composited_pixels")?).ok_or(Refusal::TranscriptMalformed)?;
    if pixels > MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1 {
        return Err(Refusal::TranscriptMalformed);
    }
    let canvas = u64::from(width) * u64::from(height);
    if u64::try_from(frames).unwrap_or(u64::MAX) * canvas != pixels {
        return Err(Refusal::TranscriptMalformed);
    }
    let claimed_source =
        lower_hex_digest_v1(cursor.value("source_sha256")?).ok_or(Refusal::TranscriptMalformed)?;
    if claimed_source != source_digest {
        return Err(Refusal::SourceDigestMismatch);
    }
    let mut records: Vec<GifFrameEvidenceV2> = Vec::with_capacity(frames);
    let mut total_delay_cs: u64 = 0;
    for index in 0..frames {
        let line = cursor.next_line()?;
        let Some(rest) = line.strip_prefix("frame=") else {
            return Err(Refusal::TranscriptMalformed);
        };
        let mut parts = rest.split(' ');
        if parts.next().and_then(decimal_v1) != u64::try_from(index).ok() {
            return Err(Refusal::TranscriptMalformed);
        }
        let delay_cs = parts
            .next()
            .and_then(|token| token.strip_prefix("delay_cs="))
            .and_then(bounded_u16_v1)
            .ok_or(Refusal::TranscriptMalformed)?;
        let rect = parts
            .next()
            .and_then(|token| token.strip_prefix("rect="))
            .ok_or(Refusal::TranscriptMalformed)?;
        let rect_parts: Vec<&str> = rect.split(',').collect();
        if rect_parts.len() != 4 {
            return Err(Refusal::TranscriptMalformed);
        }
        let left = bounded_u16_v1(rect_parts[0]).ok_or(Refusal::TranscriptMalformed)?;
        let top = bounded_u16_v1(rect_parts[1]).ok_or(Refusal::TranscriptMalformed)?;
        let frame_width = bounded_u16_v1(rect_parts[2]).ok_or(Refusal::TranscriptMalformed)?;
        let frame_height = bounded_u16_v1(rect_parts[3]).ok_or(Refusal::TranscriptMalformed)?;
        if frame_width == 0
            || frame_height == 0
            || u32::from(left) + u32::from(frame_width) > u32::from(width)
            || u32::from(top) + u32::from(frame_height) > u32::from(height)
        {
            return Err(Refusal::TranscriptMalformed);
        }
        let disposal = match parts
            .next()
            .and_then(|token| token.strip_prefix("disposal="))
        {
            Some("unspecified") => GifDisposalV1::Unspecified,
            Some("keep") => GifDisposalV1::Keep,
            Some("background") => GifDisposalV1::Background,
            Some("previous") => GifDisposalV1::Previous,
            _ => return Err(Refusal::TranscriptMalformed),
        };
        let transparent_index = match parts
            .next()
            .and_then(|token| token.strip_prefix("transparent="))
        {
            Some("none") => None,
            Some(value) => Some(
                decimal_v1(value)
                    .and_then(|index| u8::try_from(index).ok())
                    .ok_or(Refusal::TranscriptMalformed)?,
            ),
            None => return Err(Refusal::TranscriptMalformed),
        };
        let interlaced = match parts
            .next()
            .and_then(|token| token.strip_prefix("interlaced="))
        {
            Some("true") => true,
            Some("false") => false,
            _ => return Err(Refusal::TranscriptMalformed),
        };
        let local_palette = match parts
            .next()
            .and_then(|token| token.strip_prefix("local_palette="))
        {
            Some("true") => true,
            Some("false") => false,
            _ => return Err(Refusal::TranscriptMalformed),
        };
        let composite = parts
            .next()
            .and_then(|token| token.strip_prefix("composite_sha256="))
            .and_then(lower_hex_digest_v1)
            .ok_or(Refusal::TranscriptMalformed)?;
        if parts.next().is_some() {
            return Err(Refusal::TranscriptMalformed);
        }
        total_delay_cs = total_delay_cs.saturating_add(u64::from(delay_cs));
        records.push(GifFrameEvidenceV2 {
            index,
            delay_centiseconds: delay_cs,
            left,
            top,
            width: frame_width,
            height: frame_height,
            disposal,
            transparent_index,
            interlaced,
            local_palette,
            composite_digest: composite,
        });
    }
    cursor.expect("end=SOLO-GIF-ANIMATION-V1")?;
    if cursor.at != lines.len() {
        return Err(Refusal::TranscriptMalformed);
    }
    if total_delay_cs.saturating_mul(10) != delay {
        return Err(Refusal::TranscriptMalformed);
    }
    let _ = loop_semantics;
    Ok(records)
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-116 R01` — the contained SVG vector worker.
//
// The SVG child reuses the accepted reserved flag, the accepted confinement check and the accepted
// supervisor; only the payload grammar is new. It emits one fixed header line followed by the
// canonical vector bytes, exactly as the accepted GIF arm emits a header plus its transcript.
// ---------------------------------------------------------------------------------------------

/// The closed reserved token that selects the SVG vector worker.
pub const SVG_WORKER_TOKEN_V1: &str = "svg";

/// The fixed prefix of the SVG worker's single header line.
pub const SVG_WORKER_FRAME_PREFIX_V1: &str = "SOLO-SVG-WORKER-V1";

/// The production SVG worker mode.
pub const SVG_WORKER_MODE_DECODE_V1: &str = "decode";

/// The forged-source SVG worker mode, used only to prove parent-side digest refusal.
pub const SVG_WORKER_MODE_WRONG_SOURCE_V1: &str = "svg-wrong-source";

/// The forged-output SVG worker mode, used only to prove parent-side output digest refusal.
pub const SVG_WORKER_MODE_WRONG_OUTPUT_V1: &str = "svg-wrong-output";

/// The largest accepted SVG worker response: the canonical ceiling plus one header line.
pub const MAX_SVG_WORKER_STDOUT_BYTES_V1: u64 =
    crate::graphic_svg_inspection::MAX_SVG_CANONICAL_BYTES_V1 as u64 + 4096;

/// The closed header keys of the SVG worker frame, in their fixed order.
const SVG_WORKER_KEYS_V1: [&str; 13] = [
    "format",
    "mime",
    "confinement",
    "pid",
    "width",
    "height",
    "elements",
    "depth",
    "references",
    "path_commands",
    "filters",
    "source_sha256",
    "output_len",
];

/// Which SVG worker mode the parent may request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgWorkerModeV1 {
    /// The production mode.
    Decode,
    /// Claim the wrong source digest.
    WrongSource,
    /// Claim the wrong canonical output digest.
    WrongOutput,
}

/// The closed SVG sensitivity seam a test may select. It can never choose a mode by string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgWorkerTestModeV1 {
    /// Claim the wrong source digest.
    WrongSource,
    /// Claim the wrong canonical output digest.
    WrongOutput,
}

impl SvgWorkerTestModeV1 {
    const fn mode(self) -> SvgWorkerModeV1 {
        match self {
            Self::WrongSource => SvgWorkerModeV1::WrongSource,
            Self::WrongOutput => SvgWorkerModeV1::WrongOutput,
        }
    }
}

/// Which gate refused one confined SVG worker observation. Every refusal is whole-asset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SvgWorkerRefusalV1 {
    /// The source was empty or above the accepted ceiling.
    SourceRejected,
    /// The supervisor refused to run the child.
    SupervisorRefused,
    /// The child exited non-zero.
    WorkerExited,
    /// Confinement, cleanup or identity could not be proved.
    UnverifiedConfinement,
    /// The response exceeded the accepted ceiling.
    OutputTooLarge,
    /// The response header was malformed.
    FrameMalformed,
    /// The canonical body length or digest did not match the claim.
    OutputDigestMismatch,
    /// The source digest did not match the parent's own digest.
    SourceDigestMismatch,
    /// The claimed identity did not match the observed child.
    IdentityMismatch,
    /// The returned bytes were not the canonical form of themselves.
    NotCanonical,
}

impl SvgWorkerRefusalV1 {
    /// Stable ASCII refusal token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::SourceRejected => "svg-worker-source-rejected",
            Self::SupervisorRefused => "svg-worker-supervisor-refused",
            Self::WorkerExited => "svg-worker-exited",
            Self::UnverifiedConfinement => "svg-worker-unverified-confinement",
            Self::OutputTooLarge => "svg-worker-output-too-large",
            Self::FrameMalformed => "svg-worker-frame-malformed",
            Self::OutputDigestMismatch => "svg-worker-output-digest-mismatch",
            Self::SourceDigestMismatch => "svg-worker-source-digest-mismatch",
            Self::IdentityMismatch => "svg-worker-identity-mismatch",
            Self::NotCanonical => "svg-worker-not-canonical",
        }
    }
}

/// One parent-validated SVG vector observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SvgWorkerObservationV1 {
    width: u32,
    height: u32,
    elements: usize,
    depth: usize,
    references: usize,
    path_commands: usize,
    filters: usize,
    canonical: Vec<u8>,
    source_digest: MediaWorkerDigestV1,
    output_digest: MediaWorkerDigestV1,
    worker_pid: u32,
    wall_ms: u64,
    limits_verified: bool,
    cleanup_verified: bool,
}

impl SvgWorkerObservationV1 {
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
        self.width as u64 * self.height as u64
    }

    /// The number of retained elements.
    #[must_use]
    pub const fn elements(&self) -> usize {
        self.elements
    }

    /// The maximum element depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// The number of validated references.
    #[must_use]
    pub const fn references(&self) -> usize {
        self.references
    }

    /// The aggregate path-command count.
    #[must_use]
    pub const fn path_commands(&self) -> usize {
        self.path_commands
    }

    /// The number of filters.
    #[must_use]
    pub const fn filters(&self) -> usize {
        self.filters
    }

    /// Exactly the canonical vector bytes the child returned.
    #[must_use]
    pub fn canonical(&self) -> &[u8] {
        &self.canonical
    }

    /// The parent's digest of exactly the source bytes it sent.
    #[must_use]
    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// The parent's own digest of exactly the canonical bytes it received.
    #[must_use]
    pub const fn output_digest(&self) -> &MediaWorkerDigestV1 {
        &self.output_digest
    }

    /// The observed child process identifier.
    #[must_use]
    pub const fn worker_pid(&self) -> u32 {
        self.worker_pid
    }

    /// The measured wall time of the attempt.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether the supervisor proved limits.
    #[must_use]
    pub const fn limits_verified(&self) -> bool {
        self.limits_verified
    }

    /// Whether the supervisor proved cleanup.
    #[must_use]
    pub const fn cleanup_verified(&self) -> bool {
        self.cleanup_verified
    }
}

/// The child half of the SVG worker: prove confinement, then sanitise one owned bounded stream.
fn run_svg_worker_child_v1(claimed: MediaWorkerDigestV1, mode: SvgWorkerModeV1) -> u8 {
    let Some(confinement) = crate::media_worker_host::child_image_worker_confinement_v1() else {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    };
    if !confinement.matches_profile_v1() {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    }

    let mut source: Vec<u8> = Vec::new();
    let limit = crate::graphic_svg_inspection::MAX_SVG_SOURCE_BYTES_V1 + 1;
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    if handle
        .by_ref()
        .take(limit)
        .read_to_end(&mut source)
        .is_err()
    {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    let transferred = u64::try_from(source.len()).unwrap_or(u64::MAX);
    if source.is_empty() || transferred > crate::graphic_svg_inspection::MAX_SVG_SOURCE_BYTES_V1 {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    let actual_source = sha256_media_worker_bytes_v1(&source);
    if actual_source != claimed {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    let Ok(sanitized) = crate::graphic_svg_inspection::sanitize_svg_v1(&source) else {
        return IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    };
    let canonical = sanitized.canonical().to_vec();
    let facts = sanitized.facts();
    let actual_output = sha256_media_worker_bytes_v1(&canonical);

    let claimed_source = match mode {
        SvgWorkerModeV1::WrongSource => flipped_digest_v1(actual_source),
        SvgWorkerModeV1::Decode | SvgWorkerModeV1::WrongOutput => actual_source,
    };
    let claimed_output = match mode {
        SvgWorkerModeV1::WrongOutput => flipped_digest_v1(actual_output),
        SvgWorkerModeV1::Decode | SvgWorkerModeV1::WrongSource => actual_output,
    };
    let header = format!(
        "{prefix} format={format} mime={mime} confinement=verified pid={pid} width={width} height={height} elements={elements} depth={depth} references={references} path_commands={paths} filters={filters} source_sha256={source} output_len={length} output_sha256={output}\n",
        prefix = SVG_WORKER_FRAME_PREFIX_V1,
        format = SVG_WORKER_TOKEN_V1,
        mime = crate::graphic_svg_inspection::SVG_MIME_V1,
        pid = std::process::id(),
        width = facts.width(),
        height = facts.height(),
        elements = facts.elements(),
        depth = facts.depth(),
        references = facts.references(),
        paths = facts.path_commands(),
        filters = facts.filters(),
        source = claimed_source.to_hex(),
        length = canonical.len(),
        output = claimed_output.to_hex(),
    );
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    if out.write_all(header.as_bytes()).is_err()
        || out.write_all(&canonical).is_err()
        || out.flush().is_err()
    {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }
    0
}

/// Maps one selected SVG mode onto its closed child token, or `None` for production.
const fn svg_parent_mode_token_v1(mode: SvgWorkerModeV1) -> Option<&'static str> {
    match mode {
        SvgWorkerModeV1::Decode => None,
        SvgWorkerModeV1::WrongSource => Some(SVG_WORKER_MODE_WRONG_SOURCE_V1),
        SvgWorkerModeV1::WrongOutput => Some(SVG_WORKER_MODE_WRONG_OUTPUT_V1),
    }
}

/// Validates one SVG worker response against the request that produced it.
///
/// The parent never trusts the child's claim: it recomputes the source digest, recomputes the
/// canonical output digest, re-checks every budget, and re-runs the sanitizer over the returned
/// canonical bytes to prove they are exactly the canonical form of themselves. A forged digest
/// therefore cannot survive, and a non-canonical body refuses instead of being normalised.
fn validate_svg_worker_frame_v1(
    stdout: &[u8],
    source_digest: MediaWorkerDigestV1,
    child_pid: u32,
) -> Result<SvgWorkerObservationV1, SvgWorkerRefusalV1> {
    use SvgWorkerRefusalV1 as Refusal;
    if stdout.contains(&b'\r') {
        return Err(Refusal::FrameMalformed);
    }
    let split = stdout
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(Refusal::FrameMalformed)?;
    let header = std::str::from_utf8(&stdout[..split]).map_err(|_| Refusal::FrameMalformed)?;
    let rest = header
        .strip_prefix(SVG_WORKER_FRAME_PREFIX_V1)
        .and_then(|rest| rest.strip_prefix(' '))
        .ok_or(Refusal::FrameMalformed)?;
    let tokens: Vec<&str> = rest.split(' ').collect();
    if tokens.len() != SVG_WORKER_KEYS_V1.len() + 1 {
        return Err(Refusal::FrameMalformed);
    }
    let mut values: Vec<&str> = Vec::with_capacity(tokens.len());
    for (token, key) in tokens.iter().zip(SVG_WORKER_KEYS_V1) {
        let value = token
            .strip_prefix(key)
            .and_then(|rest| rest.strip_prefix('='))
            .ok_or(Refusal::FrameMalformed)?;
        values.push(value);
    }
    let output_digest_token = tokens[SVG_WORKER_KEYS_V1.len()]
        .strip_prefix("output_sha256=")
        .ok_or(Refusal::FrameMalformed)?;
    if values[0] != SVG_WORKER_TOKEN_V1
        || values[1] != crate::graphic_svg_inspection::SVG_MIME_V1
        || values[2] != "verified"
    {
        return Err(Refusal::FrameMalformed);
    }
    let pid = decimal_v1(values[3])
        .and_then(|value| u32::try_from(value).ok())
        .ok_or(Refusal::FrameMalformed)?;
    if pid == 0 || pid != child_pid {
        return Err(Refusal::IdentityMismatch);
    }
    let width = decimal_v1(values[4]).ok_or(Refusal::FrameMalformed)?;
    let height = decimal_v1(values[5]).ok_or(Refusal::FrameMalformed)?;
    if !(1..=8192).contains(&width) || !(1..=8192).contains(&height) {
        return Err(Refusal::FrameMalformed);
    }
    if width * height > 32_000_000 {
        return Err(Refusal::FrameMalformed);
    }
    let elements = decimal_v1(values[6]).and_then(|value| usize::try_from(value).ok());
    let depth = decimal_v1(values[7]).and_then(|value| usize::try_from(value).ok());
    let references = decimal_v1(values[8]).and_then(|value| usize::try_from(value).ok());
    let path_commands = decimal_v1(values[9]).and_then(|value| usize::try_from(value).ok());
    let filters = decimal_v1(values[10]).and_then(|value| usize::try_from(value).ok());
    let (Some(elements), Some(depth), Some(references), Some(path_commands), Some(filters)) =
        (elements, depth, references, path_commands, filters)
    else {
        return Err(Refusal::FrameMalformed);
    };
    if elements == 0
        || elements > crate::graphic_svg_inspection::MAX_SVG_ELEMENTS_V1
        || depth == 0
        || depth > crate::graphic_svg_inspection::MAX_SVG_NESTING_V1
        || references > crate::graphic_svg_inspection::MAX_SVG_REFERENCES_V1
        || path_commands > crate::graphic_svg_inspection::MAX_SVG_PATH_COMMANDS_V1
        || filters > crate::graphic_svg_inspection::MAX_SVG_FILTER_ELEMENTS_V1
    {
        return Err(Refusal::FrameMalformed);
    }
    let claimed_source = lower_hex_digest_v1(values[11]).ok_or(Refusal::FrameMalformed)?;
    if claimed_source != source_digest {
        return Err(Refusal::SourceDigestMismatch);
    }
    let body = &stdout[split + 1..];
    let output_len = decimal_v1(values[12])
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Refusal::FrameMalformed)?;
    if output_len != body.len() {
        return Err(Refusal::OutputDigestMismatch);
    }
    if output_len > crate::graphic_svg_inspection::MAX_SVG_CANONICAL_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    let claimed_output = lower_hex_digest_v1(output_digest_token).ok_or(Refusal::FrameMalformed)?;
    let received = sha256_media_worker_bytes_v1(body);
    if received != claimed_output {
        return Err(Refusal::OutputDigestMismatch);
    }

    let recanonical =
        crate::graphic_svg_inspection::sanitize_svg_v1(body).map_err(|_| Refusal::NotCanonical)?;
    if recanonical.canonical() != body {
        return Err(Refusal::NotCanonical);
    }
    let facts = recanonical.facts();
    if u64::from(facts.width()) != width
        || u64::from(facts.height()) != height
        || facts.elements() != elements
        || facts.depth() != depth
        || facts.references() != references
        || facts.path_commands() != path_commands
        || facts.filters() != filters
    {
        return Err(Refusal::FrameMalformed);
    }

    Ok(SvgWorkerObservationV1 {
        width: u32::try_from(width).map_err(|_| Refusal::FrameMalformed)?,
        height: u32::try_from(height).map_err(|_| Refusal::FrameMalformed)?,
        elements,
        depth,
        references,
        path_commands,
        filters,
        canonical: body.to_vec(),
        source_digest,
        output_digest: received,
        worker_pid: pid,
        wall_ms: 0,
        limits_verified: false,
        cleanup_verified: false,
    })
}

/// The production SVG inspection seam for an already-known executable.
///
/// # Errors
///
/// A closed [`SvgWorkerRefusalV1`], never a partially accepted vector document.
pub fn inspect_svg_bytes_with_executable_v1(
    executable: &std::path::Path,
    source: &[u8],
) -> Result<SvgWorkerObservationV1, SvgWorkerRefusalV1> {
    inspect_svg_bytes_in_mode_v1(executable, source, SvgWorkerModeV1::Decode)
}

/// The closed SVG sensitivity seam: run the real worker in one forged-evidence mode.
///
/// # Errors
///
/// A closed [`SvgWorkerRefusalV1`], never a partially accepted vector document.
pub fn inspect_svg_bytes_with_test_mode_v1(
    executable: &std::path::Path,
    source: &[u8],
    mode: SvgWorkerTestModeV1,
) -> Result<SvgWorkerObservationV1, SvgWorkerRefusalV1> {
    inspect_svg_bytes_in_mode_v1(executable, source, mode.mode())
}

/// Runs one confined SVG worker and validates its response.
fn inspect_svg_bytes_in_mode_v1(
    executable: &std::path::Path,
    source: &[u8],
    mode: SvgWorkerModeV1,
) -> Result<SvgWorkerObservationV1, SvgWorkerRefusalV1> {
    use SvgWorkerRefusalV1 as Refusal;
    if source.is_empty()
        || u64::try_from(source.len()).unwrap_or(u64::MAX)
            > crate::graphic_svg_inspection::MAX_SVG_SOURCE_BYTES_V1
    {
        return Err(Refusal::SourceRejected);
    }
    let source_digest = sha256_media_worker_bytes_v1(source);
    let source_hex = source_digest.to_hex();
    let run = match svg_parent_mode_token_v1(mode) {
        None => run_confined_image_worker_v1(executable, SVG_WORKER_TOKEN_V1, &source_hex, source),
        Some(mode_token) => run_confined_image_worker_mode_v1(
            executable,
            SVG_WORKER_TOKEN_V1,
            &source_hex,
            Some(mode_token),
            source,
            &mut crate::media_worker_host::N102FaultRunV1::production(),
        ),
    }
    .map_err(|_| Refusal::SupervisorRefused)?;
    if !run.limits_verified() || !run.cleanup_verified() || run.child_pid() == 0 {
        return Err(Refusal::UnverifiedConfinement);
    }
    if run.exit_code() != Some(0) {
        return Err(Refusal::WorkerExited);
    }
    if u64::try_from(run.stdout().len()).unwrap_or(u64::MAX) > MAX_SVG_WORKER_STDOUT_BYTES_V1 {
        return Err(Refusal::OutputTooLarge);
    }
    let mut observation =
        validate_svg_worker_frame_v1(run.stdout(), source_digest, run.child_pid())?;
    observation.limits_verified = true;
    observation.cleanup_verified = true;
    observation.wall_ms = run.wall_ms();
    Ok(observation)
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-116 R02` — the V3 five-format composition.
//
// This adapter owns no decoding rule of its own: it selects the already-accepted confined worker
// for each declared format and hands the untrusted response to the inward V3 validator. AVIF is
// refused here with a typed refusal and no worker is ever started for it.
// ---------------------------------------------------------------------------------------------

/// The confined five-format inspection adapter.
pub struct ConfinedGraphicInspectAdapterV3 {
    executable: std::path::PathBuf,
}

impl ConfinedGraphicInspectAdapterV3 {
    /// Binds the adapter to one already-pinned CLI executable.
    #[must_use]
    pub fn new(executable: &std::path::Path) -> Self {
        Self {
            executable: executable.to_path_buf(),
        }
    }
}

impl solo_application::GraphicImageInspectPortV3 for ConfinedGraphicInspectAdapterV3 {
    fn inspect_v3(
        &self,
        request: &solo_application::GraphicImageInspectRequestV3,
    ) -> solo_application::GraphicImageInspectPortOutcomeV3 {
        use GraphicImageInspectPortOutcomeV3 as Outcome;
        use solo_application::GraphicImageInspectRefusalKindV3 as Kind;
        use solo_application::{
            GraphicImageFormatV1, GraphicImageFormatV3, GraphicImageInspectPortOutcomeV3,
            GraphicImageObservationV3, GraphicVectorEvidenceV3,
        };

        let format = request.declared_format();
        if format.is_avif() {
            // The AVIF arm is deliberately dead: no worker, no decode, no claim.
            return Outcome::Refused(Kind::AvifUnsupported);
        }
        if format.is_svg() {
            return match inspect_svg_bytes_with_executable_v1(&self.executable, request.bytes()) {
                Ok(observation) => Outcome::Inspected(Box::new(GraphicImageObservationV3::Vector(
                    GraphicVectorEvidenceV3 {
                        width: observation.width(),
                        height: observation.height(),
                        elements: observation.elements(),
                        depth: observation.depth(),
                        references: observation.references(),
                        path_commands: observation.path_commands(),
                        filters: observation.filters(),
                        canonical: observation.canonical().to_vec(),
                        source_digest: *observation.source_digest(),
                        output_digest: *observation.output_digest(),
                        worker_pid: observation.worker_pid(),
                        wall_ms: observation.wall_ms(),
                        limits_verified: observation.limits_verified(),
                        cleanup_verified: observation.cleanup_verified(),
                    },
                ))),
                Err(_) => Outcome::Refused(Kind::PortRefused),
            };
        }
        if format.is_gif() {
            return match inspect_gif_bytes_with_executable_v1(&self.executable, request.bytes()) {
                Ok(observation) => Outcome::Inspected(Box::new(
                    GraphicImageObservationV3::Animation(gif_animation_evidence_v2(&observation)),
                )),
                Err(_) => Outcome::Refused(Kind::PortRefused),
            };
        }
        let still_format = match format {
            GraphicImageFormatV3::Png => GraphicImageFormatV1::Png,
            GraphicImageFormatV3::Jpeg => GraphicImageFormatV1::Jpeg,
            GraphicImageFormatV3::WebP => GraphicImageFormatV1::WebP,
            _ => return Outcome::Refused(Kind::UnsupportedFormat),
        };
        match inspect_image_bytes_with_mode_v1(
            &self.executable,
            request.bytes(),
            still_format,
            IMAGE_WORKER_MODE_DECODE_V1,
        ) {
            GraphicImageInspectPortOutcomeV1::Inspected(observation) => {
                Outcome::Inspected(Box::new(GraphicImageObservationV3::Still(*observation)))
            }
            GraphicImageInspectPortOutcomeV1::Refused(_) => Outcome::Refused(Kind::PortRefused),
        }
    }
}

/// One whole-owner V3 outcome: every obligation inspected, or one attributable refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageOutcomeV3 {
    /// Every declared obligation was inspected.
    Complete(solo_application::GraphicImageInspectionSetV3),
    /// The confined source read failed with its own typed refusal.
    ReadRefused(solo_application::GraphicAssetReadRefusalV1),
    /// One obligation was refused.
    Refused(solo_application::GraphicImageInspectRefusalV3),
}

/// Reads the real confined sources for one inventory, then inspects the whole candidate.
///
/// The chain is the accepted one, unchanged: validated inventory, real confined source read, one
/// freshly supervised single-use worker per source, then the inward whole-set validator.
#[must_use]
pub fn inspect_entity_graphic_images_v3(
    root: &ContentRootCapability,
    inventory: &GraphicAssetObligationInventoryV1,
    executable: &Path,
) -> EntityImageOutcomeV3 {
    use solo_application::GraphicAssetReadSetOutcomeV1;
    let source_set = match read_graphic_asset_bytes_v1(inventory, root) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            return EntityImageOutcomeV3::ReadRefused(refusal);
        }
    };
    let adapter = ConfinedGraphicInspectAdapterV3::new(executable);
    match solo_application::inspect_graphic_image_source_set_v3(&source_set, &adapter) {
        solo_application::GraphicImageInspectionOutcomeV3::Complete(set) => {
            EntityImageOutcomeV3::Complete(set)
        }
        solo_application::GraphicImageInspectionOutcomeV3::Refused(refusal) => {
            EntityImageOutcomeV3::Refused(refusal)
        }
    }
}

// ---------------------------------------------------------------------------------------------
// `SOLO-WO-118 R11` — the V4 six-format whole-candidate composition.
//
// V4 adds no decoder and no second child. It composes the two already-accepted confined ports —
// the V3 five-format adapter and the R10 AVIF V1 adapter — behind one read of the real confined
// source set, and hands their results to the inward V4 whole-set validator. The V3 entrypoints and
// their historic `avif-unsupported-v3` refusal are untouched.
// ---------------------------------------------------------------------------------------------

/// The composed six-format inspection adapter: five legacy formats plus the confined AVIF class.
pub struct ConfinedGraphicInspectAdapterV4 {
    legacy: ConfinedGraphicInspectAdapterV3,
    avif: crate::graphic_avif_inspection::ConfinedAvifInspectAdapterV1,
}

impl ConfinedGraphicInspectAdapterV4 {
    /// Binds both halves to the one already-trusted `solo-cli` executable.
    ///
    /// No second binary, decoder or process launcher exists: the V3 half starts the accepted
    /// PNG/JPEG/WebP/GIF/SVG child paths and the AVIF half starts the accepted R10 child path.
    #[must_use]
    pub fn new(executable: &Path) -> Self {
        Self {
            legacy: ConfinedGraphicInspectAdapterV3::new(executable),
            avif: crate::graphic_avif_inspection::ConfinedAvifInspectAdapterV1::new(
                executable.to_path_buf(),
            ),
        }
    }

    /// Every child process the AVIF half actually observed, in launch order.
    ///
    /// This is reported host evidence for tests; it grants nothing and is never consulted by a
    /// decision. The V3 half reports its own identifiers inside each observation.
    #[must_use]
    pub fn observed_avif_pids_v1(&self) -> Vec<u32> {
        self.avif.observed_pids_v1()
    }
}

impl solo_application::GraphicImageInspectPortV3 for ConfinedGraphicInspectAdapterV4 {
    fn inspect_v3(
        &self,
        request: &solo_application::GraphicImageInspectRequestV3,
    ) -> solo_application::GraphicImageInspectPortOutcomeV3 {
        self.legacy.inspect_v3(request)
    }
}

impl solo_application::MediaWorkerPortV1 for ConfinedGraphicInspectAdapterV4 {
    fn inspect_avif(
        &self,
        request: &solo_application::MediaWorkerRequestV1<'_>,
    ) -> solo_application::MediaWorkerRunV1 {
        self.avif.inspect_avif(request)
    }
}

/// One whole-owner V4 outcome: every obligation inspected, or one attributable refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityImageOutcomeV4 {
    /// Every declared obligation was inspected across all six formats.
    Complete(solo_application::GraphicImageInspectionSetV4),
    /// The confined source read failed with its own typed refusal.
    ReadRefused(solo_application::GraphicAssetReadRefusalV1),
    /// One obligation was refused; no partial result exists.
    Refused(solo_application::GraphicImageInspectRefusalV4),
}

/// Reads the real confined sources once, then inspects the whole six-format candidate.
///
/// The chain is the accepted one: validated inventory, one real confined source read over the
/// owner's capability, one freshly supervised single-use child per source in exactly the accepted
/// form for that format, then the inward whole-set validator. Nothing here decodes in the parent,
/// infers a format from bytes, or substitutes a reader.
#[must_use]
pub fn inspect_entity_graphic_images_v4(
    root: &ContentRootCapability,
    inventory: &GraphicAssetObligationInventoryV1,
    executable: &Path,
) -> EntityImageOutcomeV4 {
    use solo_application::GraphicAssetReadSetOutcomeV1;
    let source_set = match read_graphic_asset_bytes_v1(inventory, root) {
        GraphicAssetReadSetOutcomeV1::Complete(set) => set,
        GraphicAssetReadSetOutcomeV1::Refused(refusal) => {
            return EntityImageOutcomeV4::ReadRefused(refusal);
        }
    };
    let adapter = ConfinedGraphicInspectAdapterV4::new(executable);
    match solo_application::inspect_graphic_image_source_set_v4(&source_set, &adapter, &adapter) {
        solo_application::GraphicImageInspectionOutcomeV4::Complete(set) => {
            EntityImageOutcomeV4::Complete(set)
        }
        solo_application::GraphicImageInspectionOutcomeV4::Refused(refusal) => {
            EntityImageOutcomeV4::Refused(refusal)
        }
    }
}

/// The authored shape one `SOLO-WO-118 R19` host-test package request uses.
///
/// It is a **test seam**, not a product surface: it builds the same real candidate the R18 seam
/// builds, then runs the real delivery-preparation path over the two real roots. It cannot create a
/// route, a server or a published asset.
pub struct TestPackageSeamV1<'a> {
    /// The optional root-layer Platform registration.
    pub registration: Option<(&'static str, &'a str)>,
    /// The authored format of the candidate.
    pub format: TestMixedFormatV1,
    /// The authored declarations.
    pub declarations: &'a [TestMixedDeclarationV1],
    /// A deliberately foreign plan, to prove the identity binding refuses it.
    pub foreign_plan: Option<&'a solo_application::MixedGraphicPlanV1>,
    /// The test-only prepublication display eligibility case.
    pub eligibility: solo_application::PrepublicationDisplayEligibilityV1,
    /// The bounded package limits.
    pub limits: solo_application::GraphicPackageLimitsV1,
}

/// One prepared entry as the host tests see it: outward facts plus its exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPreparedEntryV1 {
    /// The deterministic content-addressed key.
    pub content_key: String,
    /// The graphic role name.
    pub role: String,
    /// The named slot.
    pub slot: String,
    /// The confirmed MIME type.
    pub mime: &'static str,
    /// The stable format token.
    pub format: &'static str,
    /// The accepted intrinsic width.
    pub width: u32,
    /// The accepted intrinsic height.
    pub height: u32,
    /// The exact output digest.
    pub output_sha256: String,
    /// The exact source digest that passed inspection.
    pub source_sha256: String,
    /// Exactly the staged output bytes.
    pub payload: Vec<u8>,
}

/// The closed result of one host-test package attempt, as plain values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPreparedPackageV1 {
    construction_refusal: Option<&'static str>,
    package_refusal: Option<&'static str>,
    source_refusal: Option<&'static str>,
    entries: Vec<TestPreparedEntryV1>,
    payload_manifest: Vec<(String, u64, String)>,
    total_bytes: u64,
    discoverable: bool,
    publication_authority: bool,
    routes_any_url: bool,
    target_level: usize,
    eligibility: &'static str,
}

impl TestPreparedPackageV1 {
    /// The stable token of a candidate-construction refusal.
    #[must_use]
    pub const fn construction_refusal(&self) -> Option<&'static str> {
        self.construction_refusal
    }

    /// The closed package refusal token.
    #[must_use]
    pub const fn package_refusal(&self) -> Option<&'static str> {
        self.package_refusal
    }

    /// The unchanged mixed-driver refusal token.
    #[must_use]
    pub const fn source_refusal(&self) -> Option<&'static str> {
        self.source_refusal
    }

    /// Every prepared entry, in selection order.
    #[must_use]
    pub fn entries(&self) -> &[TestPreparedEntryV1] {
        &self.entries
    }

    /// The deduplicated payload manifest as `(content key, byte length, digest)`.
    #[must_use]
    pub fn payload_manifest(&self) -> &[(String, u64, String)] {
        &self.payload_manifest
    }

    /// The exact aggregate payload bytes.
    #[must_use]
    pub const fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    /// Whether the outward view may advertise itself as a discovery source.
    #[must_use]
    pub const fn discoverable(&self) -> bool {
        self.discoverable
    }

    /// Whether holding this package grants publication authority. Always `false`.
    #[must_use]
    pub const fn publication_authority(&self) -> bool {
        self.publication_authority
    }

    /// Whether this package assigns a routable address. Always `false`.
    #[must_use]
    pub const fn routes_any_url(&self) -> bool {
        self.routes_any_url
    }

    /// The EFS depth of the scoped owner (evidence that an explicit owner was used).
    #[must_use]
    pub const fn target_level(&self) -> usize {
        self.target_level
    }

    /// The applied display eligibility token.
    #[must_use]
    pub const fn eligibility(&self) -> &'static str {
        self.eligibility
    }

    /// The exact `(role, slot, source digest, output digest)` provenance of every entry.
    #[must_use]
    pub fn provenance(&self) -> Vec<(String, String, String, String)> {
        self.entries
            .iter()
            .map(|entry| {
                (
                    entry.role.clone(),
                    entry.slot.clone(),
                    entry.source_sha256.clone(),
                    entry.output_sha256.clone(),
                )
            })
            .collect()
    }
}

/// `SOLO-WO-118 R19` — builds one unpublished package from the two real roots.
///
/// It composes only already-accepted parts: the validated R17 candidate, the closed mixed plan, the
/// accepted two-root acquisition and confined workers, and the closed delivery-preparation rules. It
/// writes nothing outside memory and grants no authority.
#[must_use]
pub fn prepare_unpublished_graphic_package_from_roots_v1(
    entity_root: &ContentRootCapability,
    platform_root: &crate::platform_asset_read::PlatformRootCapabilityV1,
    seam: &TestPackageSeamV1<'_>,
    request: solo_application::PlatformSourceRequestV1<'_>,
    executable: &Path,
) -> TestPreparedPackageV1 {
    let refused = |token: &'static str| TestPreparedPackageV1 {
        construction_refusal: Some(token),
        package_refusal: None,
        source_refusal: None,
        entries: Vec::new(),
        payload_manifest: Vec::new(),
        total_bytes: 0,
        discoverable: false,
        publication_authority: false,
        routes_any_url: false,
        target_level: 0,
        eligibility: seam.eligibility.as_str(),
    };

    let built = match build_test_candidate_v1(seam.registration, seam.format, seam.declarations) {
        Ok(built) => built,
        Err(token) => return refused(token),
    };
    let plan = seam.foreign_plan.unwrap_or(&built.plan);
    let package_request = solo_application::GraphicPackageRequestV1 {
        input: &built.input,
        plan,
        target: &built.target,
        eligibility: seam.eligibility,
        limits: seam.limits,
        platform: request,
    };
    let adapter = ConfinedGraphicInspectAdapterV4::new(executable);
    let outcome = solo_application::prepare_mixed_graphic_package_v1(
        &package_request,
        entity_root,
        platform_root,
        &adapter,
        &adapter,
    );
    let target_level = built.target.level_index();

    let assembled = match &outcome {
        solo_application::GraphicDeliveryPackageOutcomeV1::Prepared(package) => {
            let view = package.public_intent();
            let entries: Vec<TestPreparedEntryV1> = package
                .entries()
                .iter()
                .map(|entry| TestPreparedEntryV1 {
                    content_key: entry.content_key().to_owned(),
                    role: entry.role().name().to_owned(),
                    slot: entry.slot().to_owned(),
                    mime: entry.mime(),
                    format: format_token_v3(entry.format()),
                    width: entry.width(),
                    height: entry.height(),
                    output_sha256: entry.output_sha256().to_hex(),
                    source_sha256: entry.source_sha256().to_hex(),
                    payload: package
                        .payload(entry.content_key())
                        .map_or_else(Vec::new, |payload| payload.bytes().to_vec()),
                })
                .collect();
            let manifest: Vec<(String, u64, String)> = package
                .payloads()
                .iter()
                .map(|payload| {
                    (
                        payload.content_key().to_owned(),
                        payload.len(),
                        payload.sha256().to_hex(),
                    )
                })
                .collect();
            TestPreparedPackageV1 {
                construction_refusal: None,
                package_refusal: None,
                source_refusal: None,
                entries,
                payload_manifest: manifest,
                total_bytes: package.total_bytes(),
                discoverable: view.discoverable(),
                publication_authority: view.publication_authority(),
                routes_any_url: outcome.routes_any_url(),
                target_level,
                eligibility: seam.eligibility.as_str(),
            }
        }
        solo_application::GraphicDeliveryPackageOutcomeV1::Refused(refusal) => {
            TestPreparedPackageV1 {
                package_refusal: Some(refusal.detail()),
                routes_any_url: outcome.routes_any_url(),
                target_level,
                ..refused("refused")
            }
        }
        solo_application::GraphicDeliveryPackageOutcomeV1::SourceRefused(driver) => {
            TestPreparedPackageV1 {
                source_refusal: Some(source_refusal_token(driver)),
                routes_any_url: outcome.routes_any_url(),
                target_level,
                ..refused("source-refused")
            }
        }
    };
    TestPreparedPackageV1 {
        construction_refusal: None,
        ..assembled
    }
}

/// The stable token of one unchanged mixed-driver refusal.
fn source_refusal_token(
    outcome: &solo_application::MixedGraphicCandidateOutcomeV1,
) -> &'static str {
    use solo_application::MixedGraphicCandidateOutcomeV1 as Outcome;
    match outcome {
        Outcome::Complete(_) => "complete",
        Outcome::PlanRefused(_) => "plan-refused",
        Outcome::EntityReadRefused(_) => "entity-read-refused",
        Outcome::PlatformReadRefused(_) => "platform-read-refused",
        Outcome::BindingRefused(_) => "binding-refused",
        Outcome::InspectRefused(refusal) => refusal.kind().detail(),
    }
}

/// The stable token of one confirmed image format.
const fn format_token_v3(format: solo_application::GraphicImageFormatV3) -> &'static str {
    use solo_application::GraphicImageFormatV3 as Format;
    match format {
        Format::Png => "png",
        Format::Jpeg => "jpeg",
        Format::WebP => "webp",
        Format::Gif => "gif",
        Format::Svg => "svg",
        Format::Avif => "avif",
    }
}

/// `SOLO-WO-118 R18` — acquires and inspects one **mixed** Entity/Platform candidate.
///
/// This is the outward wiring of exactly one composed application path. It takes the two
/// independently adopted roots — the Entity content root and the separate Platform library root —
/// and never substitutes one for the other, never builds an external path from an asset identifier
/// and never infers a namespace from a filename, a route or a locale. Every Entity source is read
/// through the accepted confined Entity reader that `ContentRootCapability` already implements;
/// every Platform source is read through the accepted pinned Platform reader that
/// `PlatformRootCapabilityV1` already implements; every decode happens in the accepted confined
/// worker child.
///
/// A refusal is a whole-candidate refusal: no earlier successful read or worker result survives.
/// Nothing here publishes, derives, routes or authorizes; the returned set is inert evidence.
pub fn inspect_mixed_graphic_candidate_from_roots_v1(
    entity_root: &ContentRootCapability,
    platform_root: &crate::platform_asset_read::PlatformRootCapabilityV1,
    plan: &solo_application::MixedGraphicPlanV1,
    request: solo_application::PlatformSourceRequestV1<'_>,
    executable: &Path,
) -> solo_application::MixedGraphicCandidateOutcomeV1 {
    let adapter = ConfinedGraphicInspectAdapterV4::new(executable);
    solo_application::inspect_mixed_graphic_candidate_v1(
        plan,
        entity_root,
        platform_root,
        request,
        &adapter,
        &adapter,
    )
}
