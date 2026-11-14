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
        matches!(self, Self::ProbeHandle | Self::ProbeEnvironment)
    }
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

/// Which EFS owner a host-test declaration is authored on.
///
/// It exists so the `solo-cli` integration-test crate — which has no `solo-domain` dependency and
/// whose manifest is outside this order's ledger — can still exercise the **real** validated
/// inventory path rather than a hand-built stand-in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOwnerV1 {
    /// The Entity itself.
    Entity,
    /// One Site inside the single Facet.
    Site,
    /// One Page inside that Site.
    Page,
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
            Self::Site => QualifiedOwnerV1::site(entity, facet, site),
            Self::Page => QualifiedOwnerV1::page(entity, facet, site, page),
        }
    }
}

/// Builds one real, whole-graph validated inventory for host tests.
///
/// Each entry is `(owner, role, entity reference, exposure)`, where the role is `logo`, `favicon`
/// or `banner` and the exposure is `public` or `internal`. It runs the same validators the
/// production inventory uses, so a test cannot smuggle in an invalid declaration; it produces only
/// an inert inventory, decodes nothing and grants no authority. `None` means the specification was
/// itself invalid.
#[must_use]
pub fn test_inventory_v1(
    declarations: &[(TestOwnerV1, &str, &str, &str)],
) -> Option<GraphicAssetObligationInventoryV1> {
    use solo_application::{
        PropertyResolutionRequestV1, collect_graphic_asset_obligations_v1,
        resolve_declared_properties_v1,
    };
    use solo_domain::{
        AssetNamespace, AssetReferenceV1, AuthoredPropertyValueV1, CurrentContextV1,
        DeclaredOwnerPropertiesV1, EfsOwnershipDraft, Exposure, FacetDraft, Inheritance,
        LocaleKeyV1, PageDraft, PropertyDeclarationV1, PropertyKey, PropertyValueV1,
        QualifiedOwnerV1, SiteDraft,
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
        let declaration = PropertyDeclarationV1::new(
            Inheritance::Inherit,
            CurrentContextV1::Values(vec![authored]),
        );
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
