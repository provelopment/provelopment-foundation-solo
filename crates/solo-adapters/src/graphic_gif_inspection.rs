//! Bounded, nonpublishing GIF87a/GIF89a **full-animation** inspection (`SOLO-WO-107 R01`).
//!
//! This module is the one safe-Rust GIF codec path. It does three separate things, and none of
//! them substitutes for another:
//!
//! 1. `scan_container_v1` independently walks the **container grammar** byte by byte — header,
//!    logical screen descriptor, global colour table, extension blocks, image descriptors, data
//!    sub-blocks and the exact trailer — so a decoder that merely stops reading cannot grant
//!    acceptance of an appended polyglot tail, an unknown extension block or a missing trailer.
//! 2. `inspect_gif_animation_v1` decodes **every** frame through the pinned `gif 0.14.2` decoder
//!    with frame consistency, LZW end-code and unknown-block refusal enabled, then composites each
//!    frame onto one bounded RGBA canvas with the authored disposal method and hashes the result.
//! 3. `GifAnimationEvidenceV1` is a closed, bounded, deterministic transcript of what was actually
//!    composed: ordered frames, authored centisecond delays, offsets, palettes, transparency,
//!    interlace, disposal, per-frame composite SHA-256, loop semantics and aggregate budgets.
//!
//! Deliberate non-goals: nothing here rasterises to a still, drops a frame, normalises a zero
//! centisecond delay, converts an infinite repeat into a finite one, or claims publication
//! authority. The validated original GIF bytes remain the animation artifact; this module only
//! proves what those bytes mean.

use std::io::Cursor;
use std::num::NonZeroU64;

use solo_application::{MediaWorkerDigestV1, sha256_media_worker_bytes_v1};

/// Maximum accepted GIF source, matching the approved 20 MiB media source ceiling.
pub const MAX_GIF_SOURCE_BYTES_V1: u64 = solo_application::MAX_IMAGE_SOURCE_BYTES_V1;

/// Maximum accepted number of composed frames in one animation timeline.
pub const MAX_GIF_FRAMES_V1: usize = 1024;

/// Maximum sum of `canvas_pixels` over all actually composed frames.
pub const MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1: u64 = 128_000_000;

/// Maximum authored duration of one animation cycle, in milliseconds.
pub const MAX_GIF_ONE_CYCLE_DELAY_MS_V1: u64 = 3_600_000;

/// Maximum bytes of structured evidence/trace this module will emit.
pub const MAX_GIF_TRACE_BYTES_V1: usize = 1024 * 1024;

pub const GIF_MIME_V1: &str = "image/gif";
pub const GIF87A_SIGNATURE_V1: &str = "GIF87a";
pub const GIF89A_SIGNATURE_V1: &str = "GIF89a";

/// Closed refusal vocabulary. Every variant is a whole-asset refusal: no partial timeline exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GifInspectionRefusalV1 {
    EmptySource,
    SourceTooLarge,
    InvalidSignature,
    TruncatedContainer,
    MalformedLogicalScreenDescriptor,
    MalformedBlock,
    UnsupportedExtension,
    UnknownApplicationExtension,
    MissingTrailer,
    TrailingData,
    FrameCountOutOfRange,
    CanvasOutOfRange,
    FrameRectangleOutOfRange,
    TotalCompositedPixelsExceeded,
    OneCycleDurationExceeded,
    TraceTooLarge,
    DecodeFailed,
    LzwFailed,
    BackgroundUndefined,
    BackgroundIndexOutOfRange,
    ExtensionNotPermittedInGif87a,
}

impl GifInspectionRefusalV1 {
    /// Stable refusal token: ASCII, lower-case, kebab-cased, never containing source data.
    pub const fn token(self) -> &'static str {
        match self {
            Self::EmptySource => "gif-empty-source",
            Self::SourceTooLarge => "gif-source-too-large",
            Self::InvalidSignature => "gif-invalid-signature",
            Self::TruncatedContainer => "gif-truncated-container",
            Self::MalformedLogicalScreenDescriptor => "gif-malformed-logical-screen-descriptor",
            Self::MalformedBlock => "gif-malformed-block",
            Self::UnsupportedExtension => "gif-unsupported-extension",
            Self::UnknownApplicationExtension => "gif-unknown-application-extension",
            Self::MissingTrailer => "gif-missing-trailer",
            Self::TrailingData => "gif-trailing-data",
            Self::FrameCountOutOfRange => "gif-frame-count-out-of-range",
            Self::CanvasOutOfRange => "gif-canvas-out-of-range",
            Self::FrameRectangleOutOfRange => "gif-frame-rectangle-out-of-range",
            Self::TotalCompositedPixelsExceeded => "gif-total-composited-pixels-exceeded",
            Self::OneCycleDurationExceeded => "gif-one-cycle-duration-exceeded",
            Self::TraceTooLarge => "gif-trace-too-large",
            Self::DecodeFailed => "gif-decode-failed",
            Self::LzwFailed => "gif-lzw-failed",
            Self::BackgroundUndefined => "gif-background-undefined",
            Self::BackgroundIndexOutOfRange => "gif-background-index-out-of-range",
            Self::ExtensionNotPermittedInGif87a => "gif-extension-not-permitted-in-gif87a",
        }
    }

    /// The refusal is a resource/budget refusal rather than a structural one.
    pub const fn is_budget(self) -> bool {
        matches!(
            self,
            Self::SourceTooLarge
                | Self::FrameCountOutOfRange
                | Self::CanvasOutOfRange
                | Self::TotalCompositedPixelsExceeded
                | Self::OneCycleDurationExceeded
                | Self::TraceTooLarge
        )
    }
}

/// Loop semantics exactly as authored. `Absent` and `Finite(1)` are deliberately distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GifLoopV1 {
    /// No NETSCAPE/ANIMEXTS repeat extension was present: the file plays once.
    Absent,
    /// A repeat extension with a finite count was present, including a count of `1`.
    Finite(u16),
    /// A repeat extension requested unlimited repetition; preserved exactly, never rewritten.
    Infinite,
}

impl GifLoopV1 {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Finite(_) => "finite",
            Self::Infinite => "infinite",
        }
    }

    pub const fn is_infinite(self) -> bool {
        matches!(self, Self::Infinite)
    }
}

/// Disposal of one frame, using the on-file semantics rather than a rendering policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GifDisposalV1 {
    /// `DisposalMethod::Any` (0): the decoder is not required to act.
    Unspecified,
    /// `DisposalMethod::Keep` (1): leave the composed canvas as drawn.
    Keep,
    /// `DisposalMethod::Background` (2): clear the frame rectangle to transparent.
    Background,
    /// `DisposalMethod::Previous` (3): restore the canvas as it was before this frame.
    Previous,
}

impl GifDisposalV1 {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::Keep => "keep",
            Self::Background => "background",
            Self::Previous => "previous",
        }
    }
}

/// One composed frame: the authored values plus the digest of the fully composited canvas.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GifFrameEvidenceV1 {
    delay_centiseconds: u16,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    disposal: GifDisposalV1,
    transparent_index: Option<u8>,
    interlaced: bool,
    local_palette: bool,
    composite_digest: MediaWorkerDigestV1,
}

impl GifFrameEvidenceV1 {
    pub const fn delay_centiseconds(&self) -> u16 {
        self.delay_centiseconds
    }

    pub const fn left(&self) -> u16 {
        self.left
    }

    pub const fn top(&self) -> u16 {
        self.top
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub const fn disposal(&self) -> GifDisposalV1 {
        self.disposal
    }

    pub const fn transparent_index(&self) -> Option<u8> {
        self.transparent_index
    }

    pub const fn interlaced(&self) -> bool {
        self.interlaced
    }

    pub const fn uses_local_palette(&self) -> bool {
        self.local_palette
    }

    /// SHA-256 of the complete logical canvas after this frame was composited.
    pub const fn composite_digest(&self) -> &MediaWorkerDigestV1 {
        &self.composite_digest
    }
}
/// The closed, bounded result of inspecting one complete GIF stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GifAnimationEvidenceV1 {
    signature: &'static str,
    width: u16,
    height: u16,
    frames: Vec<GifFrameEvidenceV1>,
    loop_semantics: GifLoopV1,
    one_cycle_delay_ms: u64,
    total_composited_pixels: u64,
    source_digest: MediaWorkerDigestV1,
    transcript_digest: MediaWorkerDigestV1,
}

impl GifAnimationEvidenceV1 {
    pub const fn signature(&self) -> &'static str {
        self.signature
    }

    pub const fn mime(&self) -> &'static str {
        GIF_MIME_V1
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }

    pub fn frames(&self) -> &[GifFrameEvidenceV1] {
        &self.frames
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// True only for a genuine multi-frame timeline; a still GIF is never dressed as an animation.
    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1
    }

    pub const fn loop_semantics(&self) -> GifLoopV1 {
        self.loop_semantics
    }

    /// Authored duration of exactly one playback cycle in milliseconds.
    pub const fn one_cycle_delay_ms(&self) -> u64 {
        self.one_cycle_delay_ms
    }

    pub const fn total_composited_pixels(&self) -> u64 {
        self.total_composited_pixels
    }

    pub const fn source_digest(&self) -> &MediaWorkerDigestV1 {
        &self.source_digest
    }

    /// SHA-256 of the exact deterministic transcript bytes this evidence describes.
    pub const fn transcript_digest(&self) -> &MediaWorkerDigestV1 {
        &self.transcript_digest
    }

    /// The canonical, strictly ordered ASCII transcript. Bounded by `MAX_GIF_TRACE_BYTES_V1`.
    pub fn transcript_v1(&self) -> String {
        let mut out = String::with_capacity(256 + self.frames.len() * 192);
        out.push_str("SOLO-GIF-ANIMATION-V1\n");
        out.push_str(&format!("signature={}\n", self.signature));
        out.push_str(&format!("mime={GIF_MIME_V1}\n"));
        out.push_str(&format!("width={}\nheight={}\n", self.width, self.height));
        out.push_str(&format!("frames={}\n", self.frames.len()));
        out.push_str(&format!("loop={}\n", self.loop_semantics.token()));
        if let GifLoopV1::Finite(count) = self.loop_semantics {
            out.push_str(&format!("loop_count={count}\n"));
        }
        out.push_str(&format!("one_cycle_delay_ms={}\n", self.one_cycle_delay_ms));
        out.push_str(&format!(
            "total_composited_pixels={}\n",
            self.total_composited_pixels
        ));
        out.push_str(&format!("source_sha256={}\n", self.source_digest.to_hex()));
        for (index, frame) in self.frames.iter().enumerate() {
            let transparent = match frame.transparent_index {
                Some(value) => format!("{value}"),
                None => "none".to_string(),
            };
            out.push_str(&format!(
                "frame={index} delay_cs={} rect={},{},{},{} disposal={} transparent={} interlaced={} local_palette={} composite_sha256={}\n",
                frame.delay_centiseconds,
                frame.left,
                frame.top,
                frame.width,
                frame.height,
                frame.disposal.token(),
                transparent,
                frame.interlaced,
                frame.local_palette,
                frame.composite_digest.to_hex(),
            ));
        }
        out.push_str("end=SOLO-GIF-ANIMATION-V1\n");
        out
    }
}

/// Independently scanned container facts, established before any pixel is decoded.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GifContainerFactsV1 {
    signature: &'static str,
    width: u16,
    height: u16,
    frames: usize,
    loop_semantics: GifLoopV1,
    background_index: Option<u8>,
    background_rgb: Option<[u8; 3]>,
    interlaced_flags: Vec<bool>,
    local_palette_flags: Vec<bool>,
}

fn read_le_u16_v1(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

/// Advance past one GIF data sub-block chain and return the index just after its terminator.
fn skip_sub_blocks_v1(bytes: &[u8], start: usize) -> Result<usize, GifInspectionRefusalV1> {
    let mut at = start;
    loop {
        let Some(&size) = bytes.get(at) else {
            return Err(GifInspectionRefusalV1::TruncatedContainer);
        };
        at += 1;
        if size == 0 {
            return Ok(at);
        }
        at = at
            .checked_add(usize::from(size))
            .ok_or(GifInspectionRefusalV1::TruncatedContainer)?;
        if at > bytes.len() {
            return Err(GifInspectionRefusalV1::TruncatedContainer);
        }
    }
}
/// Walk the GIF container grammar and return only what the bytes themselves prove.
///
/// This is intentionally independent of the decoder: it decides where the stream truly ends,
/// rejects unknown or unsupported extension blocks, refuses a duplicate or ambiguous repeat
/// extension, and checks every frame rectangle against the logical screen before any decode.
fn scan_container_v1(source: &[u8]) -> Result<GifContainerFactsV1, GifInspectionRefusalV1> {
    use GifInspectionRefusalV1 as Refusal;
    if source.is_empty() {
        return Err(Refusal::EmptySource);
    }
    if u64::try_from(source.len()).unwrap_or(u64::MAX) > MAX_GIF_SOURCE_BYTES_V1 {
        return Err(Refusal::SourceTooLarge);
    }
    if source.len() < 13 {
        return Err(Refusal::TruncatedContainer);
    }
    let signature = match &source[0..6] {
        b"GIF87a" => GIF87A_SIGNATURE_V1,
        b"GIF89a" => GIF89A_SIGNATURE_V1,
        _ => return Err(Refusal::InvalidSignature),
    };
    let width = read_le_u16_v1(source, 6);
    let height = read_le_u16_v1(source, 8);
    if width == 0 || height == 0 {
        return Err(Refusal::MalformedLogicalScreenDescriptor);
    }
    let packed = source[10];
    let background_index = source[11];
    let mut background_rgb: Option<[u8; 3]> = None;
    let mut at = 13usize;
    if packed & 0x80 != 0 {
        let entries = 1usize << (usize::from(packed & 0x07) + 1);
        let table_end = at
            .checked_add(entries * 3)
            .ok_or(Refusal::TruncatedContainer)?;
        if table_end > source.len() {
            return Err(Refusal::TruncatedContainer);
        }
        if usize::from(background_index) >= entries {
            return Err(Refusal::BackgroundIndexOutOfRange);
        }
        let start = at + usize::from(background_index) * 3;
        background_rgb = Some([source[start], source[start + 1], source[start + 2]]);
        at = table_end;
    }
    let canvas_pixels = u64::from(width) * u64::from(height);
    let mut frames = 0usize;
    let mut interlaced_flags: Vec<bool> = Vec::new();
    let mut local_palette_flags: Vec<bool> = Vec::new();
    let mut loop_semantics = GifLoopV1::Absent;
    let mut repeat_seen = false;
    loop {
        let Some(&introducer) = source.get(at) else {
            return Err(Refusal::MissingTrailer);
        };
        match introducer {
            0x21 => {
                if signature == GIF87A_SIGNATURE_V1 {
                    return Err(Refusal::ExtensionNotPermittedInGif87a);
                }
                let Some(&label) = source.get(at + 1) else {
                    return Err(Refusal::TruncatedContainer);
                };
                at += 2;
                match label {
                    0xF9 => {
                        if source.get(at) != Some(&4) || source.get(at + 5) != Some(&0) {
                            return Err(Refusal::MalformedBlock);
                        }
                        at += 6;
                    }
                    0xFE => at = skip_sub_blocks_v1(source, at)?,
                    0xFF => {
                        if source.get(at) != Some(&11) {
                            return Err(Refusal::MalformedBlock);
                        }
                        at += 1;
                        let Some(identifier) = source.get(at..at.saturating_add(11)) else {
                            return Err(Refusal::TruncatedContainer);
                        };
                        let application = &identifier[0..8];
                        let authentication = &identifier[8..11];
                        at += 11;
                        let after = skip_sub_blocks_v1(source, at)?;
                        let repeat_form = (application == b"NETSCAPE" && authentication == b"2.0")
                            || (application == b"ANIMEXTS" && authentication == b"1.0");
                        if !repeat_form {
                            return Err(Refusal::UnknownApplicationExtension);
                        }
                        if source.get(at) != Some(&3) || source.get(at + 1) != Some(&1) {
                            return Err(Refusal::MalformedBlock);
                        }
                        if repeat_seen {
                            return Err(Refusal::MalformedBlock);
                        }
                        repeat_seen = true;
                        let count = read_le_u16_v1(source, at + 2);
                        loop_semantics = if count == 0 {
                            GifLoopV1::Infinite
                        } else {
                            GifLoopV1::Finite(count)
                        };
                        at = after;
                    }
                    _ => return Err(Refusal::UnsupportedExtension),
                }
            }
            0x2C => {
                if source.len() < at + 10 {
                    return Err(Refusal::TruncatedContainer);
                }
                let left = u64::from(read_le_u16_v1(source, at + 1));
                let top = u64::from(read_le_u16_v1(source, at + 3));
                let frame_width = u64::from(read_le_u16_v1(source, at + 5));
                let frame_height = u64::from(read_le_u16_v1(source, at + 7));
                let descriptor = source[at + 9];
                if frame_width == 0 || frame_height == 0 {
                    return Err(Refusal::MalformedBlock);
                }
                if left + frame_width > u64::from(width) || top + frame_height > u64::from(height) {
                    return Err(Refusal::FrameRectangleOutOfRange);
                }
                interlaced_flags.push(descriptor & 0x40 != 0);
                local_palette_flags.push(descriptor & 0x80 != 0);
                at += 10;
                if descriptor & 0x80 != 0 {
                    let entries = 1usize << (usize::from(descriptor & 0x07) + 1);
                    at = at
                        .checked_add(entries * 3)
                        .ok_or(Refusal::TruncatedContainer)?;
                    if at > source.len() {
                        return Err(Refusal::TruncatedContainer);
                    }
                }
                if at >= source.len() {
                    return Err(Refusal::TruncatedContainer);
                }
                at += 1;
                at = skip_sub_blocks_v1(source, at)?;
                frames += 1;
                if frames > MAX_GIF_FRAMES_V1 {
                    return Err(Refusal::FrameCountOutOfRange);
                }
                let composed = u64::try_from(frames)
                    .unwrap_or(u64::MAX)
                    .saturating_mul(canvas_pixels);
                if composed > MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1 {
                    return Err(Refusal::TotalCompositedPixelsExceeded);
                }
            }
            0x3B => {
                if at + 1 != source.len() {
                    return Err(Refusal::TrailingData);
                }
                if frames == 0 {
                    return Err(Refusal::MalformedBlock);
                }
                return Ok(GifContainerFactsV1 {
                    signature,
                    width,
                    height,
                    frames,
                    loop_semantics,
                    background_index: background_rgb.map(|_| background_index),
                    background_rgb,
                    interlaced_flags,
                    local_palette_flags,
                });
            }
            _ => return Err(Refusal::MalformedBlock),
        }
    }
}
fn canvas_admissible_v1(width: u16, height: u16) -> bool {
    let side_ok = |side: u16| (1..=8192).contains(&side);
    if !side_ok(width) || !side_ok(height) {
        return false;
    }
    let pixels = u64::from(width) * u64::from(height);
    pixels <= solo_application::MAX_IMAGE_PIXELS_V1
}

fn disposal_of_v1(disposal: gif::DisposalMethod) -> GifDisposalV1 {
    match disposal {
        gif::DisposalMethod::Any => GifDisposalV1::Unspecified,
        gif::DisposalMethod::Keep => GifDisposalV1::Keep,
        gif::DisposalMethod::Background => GifDisposalV1::Background,
        gif::DisposalMethod::Previous => GifDisposalV1::Previous,
    }
}

/// Decode **every** frame, composite each one onto one bounded canvas and hash the result.
///
/// The returned evidence is the complete ordered timeline; there is no path that keeps only the
/// first frame, normalises a delay, or reports a still image for an animation.
pub(crate) fn inspect_gif_animation_v1(
    source: &[u8],
) -> Result<GifAnimationEvidenceV1, GifInspectionRefusalV1> {
    use GifInspectionRefusalV1 as Refusal;
    let facts = scan_container_v1(source)?;
    if !canvas_admissible_v1(facts.width, facts.height) {
        return Err(Refusal::CanvasOutOfRange);
    }
    let Some(frame_memory_limit) = NonZeroU64::new(solo_application::MAX_IMAGE_DECODED_BYTES_V1)
    else {
        return Err(Refusal::DecodeFailed);
    };
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    options.set_memory_limit(gif::MemoryLimit::Bytes(frame_memory_limit));
    options.check_frame_consistency(true);
    options.check_lzw_end_code(true);
    options.allow_unknown_blocks(false);
    let mut decoder = match options.read_info(Cursor::new(source)) {
        Ok(decoder) => decoder,
        Err(error) => return Err(decode_refusal_v1(&error)),
    };
    let canvas_pixels = u64::from(facts.width) * u64::from(facts.height);
    let canvas_stride = usize::from(facts.width) * 4;
    let canvas_bytes = usize::try_from(canvas_pixels)
        .ok()
        .and_then(|value| value.checked_mul(4));
    let Some(canvas_bytes) = canvas_bytes else {
        return Err(Refusal::CanvasOutOfRange);
    };
    if u64::try_from(canvas_bytes).unwrap_or(u64::MAX)
        > solo_application::MAX_IMAGE_DECODED_BYTES_V1
    {
        return Err(Refusal::CanvasOutOfRange);
    }
    let mut canvas = vec![0u8; canvas_bytes];
    let mut previous_canvas: Option<Vec<u8>> = None;
    let mut frames: Vec<GifFrameEvidenceV1> = Vec::with_capacity(facts.frames);
    let mut total_delay_ms = 0u64;
    let mut total_composited_pixels = 0u64;
    loop {
        let frame = match decoder.read_next_frame() {
            Ok(Some(frame)) => frame,
            Ok(None) => break,
            Err(error) => return Err(decode_refusal_v1(&error)),
        };
        if frames.len() >= MAX_GIF_FRAMES_V1 {
            return Err(Refusal::FrameCountOutOfRange);
        }
        let left = usize::from(frame.left);
        let top = usize::from(frame.top);
        let frame_width = usize::from(frame.width);
        let frame_height = usize::from(frame.height);
        if frame_width == 0 || frame_height == 0 {
            return Err(Refusal::MalformedBlock);
        }
        if left + frame_width > usize::from(facts.width)
            || top + frame_height > usize::from(facts.height)
        {
            return Err(Refusal::FrameRectangleOutOfRange);
        }
        let disposal = disposal_of_v1(frame.dispose);
        let frame_stride = frame_width * 4;
        let expected = frame_height * frame_stride;
        if frame.buffer.len() != expected {
            return Err(Refusal::DecodeFailed);
        }
        if disposal == GifDisposalV1::Previous {
            if u64::try_from(canvas_bytes).unwrap_or(u64::MAX)
                > solo_application::MAX_IMAGE_DECODED_BYTES_V1
            {
                return Err(Refusal::CanvasOutOfRange);
            }
            previous_canvas = Some(canvas.clone());
        }
        for row in 0..frame_height {
            let source_row = row * frame_stride;
            let destination_row = (top + row) * canvas_stride;
            for column in 0..frame_width {
                let source_at = source_row + column * 4;
                if frame.buffer[source_at + 3] == 0 {
                    continue;
                }
                let destination_at = destination_row + (left + column) * 4;
                canvas[destination_at..destination_at + 4]
                    .copy_from_slice(&frame.buffer[source_at..source_at + 4]);
            }
        }
        let composite_digest = sha256_media_worker_bytes_v1(&canvas);
        total_composited_pixels = total_composited_pixels
            .checked_add(canvas_pixels)
            .ok_or(Refusal::TotalCompositedPixelsExceeded)?;
        if total_composited_pixels > MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1 {
            return Err(Refusal::TotalCompositedPixelsExceeded);
        }
        let delay_ms = u64::from(frame.delay)
            .checked_mul(10)
            .ok_or(Refusal::OneCycleDurationExceeded)?;
        total_delay_ms = total_delay_ms
            .checked_add(delay_ms)
            .ok_or(Refusal::OneCycleDurationExceeded)?;
        if total_delay_ms > MAX_GIF_ONE_CYCLE_DELAY_MS_V1 {
            return Err(Refusal::OneCycleDurationExceeded);
        }
        let frame_index = frames.len();
        let interlaced = facts
            .interlaced_flags
            .get(frame_index)
            .copied()
            .unwrap_or(false);
        let local_palette = facts
            .local_palette_flags
            .get(frame_index)
            .copied()
            .unwrap_or(false);
        frames.push(GifFrameEvidenceV1 {
            delay_centiseconds: frame.delay,
            left: frame.left,
            top: frame.top,
            width: frame.width,
            height: frame.height,
            disposal,
            transparent_index: frame.transparent,
            interlaced,
            local_palette,
            composite_digest,
        });
        match disposal {
            GifDisposalV1::Background => {
                let (Some(background_rgb), Some(background_index)) =
                    (facts.background_rgb, facts.background_index)
                else {
                    return Err(Refusal::BackgroundUndefined);
                };
                let paint = if frame.transparent == Some(background_index) {
                    [0u8, 0, 0, 0]
                } else {
                    [
                        background_rgb[0],
                        background_rgb[1],
                        background_rgb[2],
                        0xff,
                    ]
                };
                for row in 0..frame_height {
                    let destination_row = (top + row) * canvas_stride;
                    for column in 0..frame_width {
                        let at = destination_row + (left + column) * 4;
                        canvas[at..at + 4].copy_from_slice(&paint);
                    }
                }
            }
            GifDisposalV1::Previous => {
                if let Some(restored) = previous_canvas.take() {
                    canvas = restored;
                }
            }
            GifDisposalV1::Unspecified | GifDisposalV1::Keep => {}
        }
    }
    if frames.len() != facts.frames {
        return Err(Refusal::DecodeFailed);
    }
    let decoded_repeat = decoder.repeat();
    let loop_semantics = match (facts.loop_semantics, decoded_repeat) {
        (GifLoopV1::Infinite, gif::Repeat::Infinite) => GifLoopV1::Infinite,
        (GifLoopV1::Finite(expected), gif::Repeat::Finite(actual)) if expected == actual => {
            GifLoopV1::Finite(expected)
        }
        (GifLoopV1::Absent, gif::Repeat::Finite(_)) => GifLoopV1::Absent,
        _ => return Err(Refusal::DecodeFailed),
    };
    let mut evidence = GifAnimationEvidenceV1 {
        signature: facts.signature,
        width: facts.width,
        height: facts.height,
        frames,
        loop_semantics,
        one_cycle_delay_ms: total_delay_ms,
        total_composited_pixels,
        source_digest: sha256_media_worker_bytes_v1(source),
        transcript_digest: MediaWorkerDigestV1::from_bytes([0u8; 32]),
    };
    let transcript = evidence.transcript_v1();
    if transcript.len() > MAX_GIF_TRACE_BYTES_V1 {
        return Err(Refusal::TraceTooLarge);
    }
    evidence.transcript_digest = sha256_media_worker_bytes_v1(transcript.as_bytes());
    Ok(evidence)
}

fn decode_refusal_v1(_error: &gif::DecodingError) -> GifInspectionRefusalV1 {
    GifInspectionRefusalV1::DecodeFailed
}
#[cfg(test)]
mod tests {
    use super::*;

    /// LSB-first GIF bit packer used only to build deterministic fixtures.
    struct BitPackerV1 {
        bytes: Vec<u8>,
        accumulator: u32,
        bits: u8,
    }

    impl BitPackerV1 {
        fn new() -> Self {
            Self {
                bytes: Vec::new(),
                accumulator: 0,
                bits: 0,
            }
        }

        fn write(&mut self, code: u16, width: u8) {
            self.accumulator |= u32::from(code) << self.bits;
            self.bits += width;
            while self.bits >= 8 {
                self.bytes.push((self.accumulator & 0xFF) as u8);
                self.accumulator >>= 8;
                self.bits -= 8;
            }
        }

        fn finish(mut self) -> Vec<u8> {
            if self.bits > 0 {
                self.bytes.push((self.accumulator & 0xFF) as u8);
            }
            self.bytes
        }
    }

    /// Real, minimal, deterministic GIF LZW encoder (no crate helper, no encoder dependency).
    fn lzw_encode_v1(min_code_size: u8, indices: &[u8]) -> Vec<u8> {
        let clear = 1u16 << min_code_size;
        let end = clear + 1;
        let mut packer = BitPackerV1::new();
        let mut code_size = min_code_size + 1;
        packer.write(clear, code_size);
        let mut table: std::collections::HashMap<(u16, u8), u16> = std::collections::HashMap::new();
        let mut next = end + 1;
        let mut current: Option<u16> = None;
        for &pixel in indices {
            match current {
                None => current = Some(u16::from(pixel)),
                Some(prefix) => {
                    if let Some(&known) = table.get(&(prefix, pixel)) {
                        current = Some(known);
                    } else {
                        packer.write(prefix, code_size);
                        table.insert((prefix, pixel), next);
                        next += 1;
                        if next == (1u16 << code_size) && code_size < 12 {
                            code_size += 1;
                        }
                        current = Some(u16::from(pixel));
                    }
                }
            }
        }
        if let Some(prefix) = current {
            packer.write(prefix, code_size);
        }
        packer.write(end, code_size);
        packer.finish()
    }

    fn table_bits_v1(entries: usize) -> (u8, usize) {
        let mut size = 2usize;
        while size < entries {
            size *= 2;
        }
        (
            u8::try_from(size.trailing_zeros())
                .unwrap_or(1)
                .saturating_sub(1),
            size,
        )
    }

    fn table_bytes_v1(palette: &[[u8; 3]]) -> (u8, Vec<u8>) {
        let (bits, entries) = table_bits_v1(palette.len().max(2));
        let mut bytes = Vec::with_capacity(entries * 3);
        for entry in palette {
            bytes.extend_from_slice(entry);
        }
        while bytes.len() < entries * 3 {
            bytes.push(0);
        }
        (bits, bytes)
    }

    fn write_sub_blocks_v1(out: &mut Vec<u8>, data: &[u8]) {
        for chunk in data.chunks(255) {
            out.push(u8::try_from(chunk.len()).unwrap_or(255));
            out.extend_from_slice(chunk);
        }
        out.push(0);
    }

    struct FrameSpecV1 {
        left: u16,
        top: u16,
        width: u16,
        height: u16,
        delay: u16,
        disposal: u8,
        transparent: Option<u8>,
        local_palette: Option<Vec<[u8; 3]>>,
        interlaced: bool,
        indices: Vec<u8>,
    }

    fn frame_v1(width: u16, height: u16, indices: &[u8]) -> FrameSpecV1 {
        FrameSpecV1 {
            left: 0,
            top: 0,
            width,
            height,
            delay: 0,
            disposal: 1,
            transparent: None,
            local_palette: None,
            interlaced: false,
            indices: indices.to_vec(),
        }
    }

    struct GifSpecV1 {
        version: &'static [u8; 6],
        width: u16,
        height: u16,
        background_index: u8,
        global_palette: Option<Vec<[u8; 3]>>,
        loop_count: Option<u16>,
        graphic_control: bool,
        frames: Vec<FrameSpecV1>,
    }
    impl GifSpecV1 {
        fn to_bytes(&self) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend_from_slice(self.version);
            out.extend_from_slice(&self.width.to_le_bytes());
            out.extend_from_slice(&self.height.to_le_bytes());
            let mut packed = 0u8;
            let mut table = Vec::new();
            let mut global_entries = 2usize;
            if let Some(palette) = &self.global_palette {
                let (bits, bytes) = table_bytes_v1(palette);
                let (_, entries) = table_bits_v1(palette.len().max(2));
                global_entries = entries;
                packed |= 0x80 | bits;
                table = bytes;
            }
            out.push(packed);
            out.push(self.background_index);
            out.push(0);
            out.extend_from_slice(&table);
            if let Some(count) = self.loop_count {
                out.extend_from_slice(&[0x21, 0xFF, 11]);
                out.extend_from_slice(b"NETSCAPE2.0");
                out.extend_from_slice(&[3, 1]);
                out.extend_from_slice(&count.to_le_bytes());
                out.push(0);
            }
            for frame in &self.frames {
                if self.graphic_control {
                    let gce = (frame.disposal << 2) | u8::from(frame.transparent.is_some());
                    out.extend_from_slice(&[
                        0x21,
                        0xF9,
                        4,
                        gce,
                        (frame.delay & 0xFF) as u8,
                        (frame.delay >> 8) as u8,
                        frame.transparent.unwrap_or(0),
                        0,
                    ]);
                }
                out.push(0x2C);
                out.extend_from_slice(&frame.left.to_le_bytes());
                out.extend_from_slice(&frame.top.to_le_bytes());
                out.extend_from_slice(&frame.width.to_le_bytes());
                out.extend_from_slice(&frame.height.to_le_bytes());
                let mut descriptor = 0u8;
                let mut local = Vec::new();
                let mut entries = global_entries;
                if let Some(palette) = &frame.local_palette {
                    let (bits, bytes) = table_bytes_v1(palette);
                    let (_, local_entries) = table_bits_v1(palette.len().max(2));
                    entries = local_entries;
                    descriptor |= 0x80 | bits;
                    local = bytes;
                }
                if frame.interlaced {
                    descriptor |= 0x40;
                }
                out.push(descriptor);
                out.extend_from_slice(&local);
                let _ = entries;
                // Fixtures pin an eight-bit LZW minimum code size so the dictionary can never grow
                // inside these tiny images: the fixture proves framing, compositing and grammar, not
                // dictionary growth, and an accidental growth off-by-one must not mask a real bug.
                let min_code_size = 8u8;
                out.push(min_code_size);
                let data = lzw_encode_v1(min_code_size, &frame.indices);
                write_sub_blocks_v1(&mut out, &data);
            }
            out.push(0x3B);
            out
        }
    }

    const RED: [u8; 3] = [255, 0, 0];
    const GREEN: [u8; 3] = [0, 255, 0];
    const BLUE: [u8; 3] = [0, 0, 255];
    const WHITE: [u8; 3] = [255, 255, 255];

    fn rgba_v1(colour: [u8; 3]) -> [u8; 4] {
        [colour[0], colour[1], colour[2], 255]
    }

    fn canvas_v1(pixels: &[[u8; 4]]) -> Vec<u8> {
        let mut out = Vec::with_capacity(pixels.len() * 4);
        for pixel in pixels {
            out.extend_from_slice(pixel);
        }
        out
    }

    fn two_colour_spec_v1(frames: Vec<FrameSpecV1>) -> GifSpecV1 {
        GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: Some(0),
            graphic_control: true,
            frames,
        }
    }
    #[test]
    fn gif87a_still_is_extension_free_and_not_an_animation() {
        let bytes = GifSpecV1 {
            version: b"GIF87a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: false,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&bytes).expect("extension-free GIF87a is accepted");
        assert_eq!(evidence.signature(), GIF87A_SIGNATURE_V1);
        assert_eq!(evidence.frame_count(), 1);
        assert!(!evidence.is_animated());
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(evidence.one_cycle_delay_ms(), 0);
        assert_eq!(evidence.frames()[0].disposal(), GifDisposalV1::Keep);
        assert_eq!(
            evidence.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[rgba_v1(RED)]))
        );
    }

    #[test]
    fn gif87a_with_a_graphic_control_extension_is_refused() {
        let bytes = GifSpecV1 {
            version: b"GIF87a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::ExtensionNotPermittedInGif87a)
        );
    }

    #[test]
    fn gif89a_multi_frame_preserves_delays_disposal_and_composition() {
        let mut first = frame_v1(1, 1, &[0]);
        first.delay = 10;
        let mut second = frame_v1(1, 1, &[0]);
        second.delay = 20;
        second.disposal = 2;
        second.transparent = Some(0);
        let mut third = frame_v1(1, 1, &[0]);
        third.delay = 5;
        third.transparent = Some(0);
        let bytes = two_colour_spec_v1(vec![first, second, third]).to_bytes();
        let evidence = inspect_gif_animation_v1(&bytes).expect("multi-frame GIF89a accepted");
        assert_eq!(evidence.frame_count(), 3);
        assert!(evidence.is_animated());
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Infinite);
        assert_eq!(evidence.one_cycle_delay_ms(), 350);
        assert_eq!(evidence.total_composited_pixels(), 3);
        let red = sha256_media_worker_bytes_v1(&canvas_v1(&[rgba_v1(RED)]));
        let cleared = sha256_media_worker_bytes_v1(&canvas_v1(&[[0, 0, 0, 0]]));
        assert_eq!(evidence.frames()[0].composite_digest(), &red);
        // The digest is taken immediately after drawing, so the transparent draw leaves the
        // canvas as frame 0 left it; the background disposal is then observable in frame 2.
        assert_eq!(evidence.frames()[1].composite_digest(), &red);
        assert_eq!(evidence.frames()[2].composite_digest(), &cleared);
        let transcript = evidence.transcript_v1();
        assert!(transcript.contains("frame=1 delay_cs=20"));
        assert!(transcript.contains("disposal=background"));
        assert_eq!(
            evidence.transcript_digest(),
            &sha256_media_worker_bytes_v1(transcript.as_bytes())
        );
    }

    #[test]
    fn gif89a_background_disposal_restores_the_screen_background_colour() {
        let mut fill = frame_v1(2, 2, &[0, 0, 0, 0]);
        fill.delay = 5;
        let mut over = frame_v1(1, 1, &[0]);
        over.left = 1;
        over.top = 1;
        over.delay = 5;
        over.disposal = 2;
        let mut reveal = frame_v1(1, 1, &[0]);
        reveal.transparent = Some(0);
        reveal.delay = 5;
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 1,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![fill, over, reveal],
        }
        .to_bytes();
        let evidence =
            inspect_gif_animation_v1(&bytes).expect("opaque background disposal accepted");
        let all_red = sha256_media_worker_bytes_v1(&canvas_v1(&[
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(RED),
        ]));
        let restored = sha256_media_worker_bytes_v1(&canvas_v1(&[
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(BLUE),
        ]));
        assert_eq!(evidence.frames()[0].composite_digest(), &all_red);
        // Frame 1 draws red into a red canvas, so its own digest is unchanged; the opaque
        // background restore becomes visible only in frame 2.
        assert_eq!(evidence.frames()[1].composite_digest(), &all_red);
        assert_eq!(evidence.frames()[2].composite_digest(), &restored);
        assert_ne!(&all_red, &restored);
    }
    #[test]
    fn gif89a_restore_previous_restores_the_saved_canvas() {
        let fill_red = frame_v1(2, 2, &[0, 0, 0, 0]);
        let mut fill_blue = frame_v1(2, 2, &[1, 1, 1, 1]);
        fill_blue.disposal = 3;
        let mut reveal = frame_v1(1, 1, &[0]);
        reveal.transparent = Some(0);
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![fill_red, fill_blue, reveal],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&bytes).expect("restore-previous accepted");
        let reds = sha256_media_worker_bytes_v1(&canvas_v1(&[
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(RED),
            rgba_v1(RED),
        ]));
        let blues = sha256_media_worker_bytes_v1(&canvas_v1(&[
            rgba_v1(BLUE),
            rgba_v1(BLUE),
            rgba_v1(BLUE),
            rgba_v1(BLUE),
        ]));
        assert_eq!(evidence.frames()[0].composite_digest(), &reds);
        assert_eq!(evidence.frames()[1].composite_digest(), &blues);
        assert_eq!(evidence.frames()[1].disposal(), GifDisposalV1::Previous);
        assert_eq!(evidence.frames()[2].composite_digest(), &reds);
    }

    #[test]
    fn gif89a_local_palette_overrides_the_global_table() {
        let mut frame = frame_v1(2, 2, &[0, 0, 0, 0]);
        frame.local_palette = Some(vec![GREEN, WHITE]);
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 2,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&bytes).expect("local palette accepted");
        assert!(evidence.frames()[0].uses_local_palette());
        assert_eq!(
            evidence.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[
                rgba_v1(GREEN),
                rgba_v1(GREEN),
                rgba_v1(GREEN),
                rgba_v1(GREEN),
            ]))
        );
    }

    #[test]
    fn gif89a_interlaced_offset_frame_composites_exactly() {
        let mut frame = frame_v1(1, 4, &[0, 2, 1, 3]);
        frame.left = 1;
        frame.interlaced = true;
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 2,
            height: 4,
            background_index: 0,
            global_palette: Some(vec![RED, GREEN, BLUE, WHITE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&bytes).expect("interlaced offset accepted");
        assert!(evidence.frames()[0].interlaced());
        assert_eq!(evidence.frames()[0].left(), 1);
        assert_eq!(
            evidence.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&canvas_v1(&[
                [0, 0, 0, 0],
                rgba_v1(RED),
                [0, 0, 0, 0],
                rgba_v1(GREEN),
                [0, 0, 0, 0],
                rgba_v1(BLUE),
                [0, 0, 0, 0],
                rgba_v1(WHITE),
            ]))
        );
    }

    #[test]
    fn gif89a_loop_semantics_and_authored_delays_are_preserved() {
        let absent = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&absent).expect("absent loop accepted");
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(evidence.one_cycle_delay_ms(), 0);
        let finite = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: Some(2),
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        let evidence = inspect_gif_animation_v1(&finite).expect("finite loop accepted");
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Finite(2));
    }

    #[test]
    fn background_index_out_of_range_is_refused() {
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 5,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::BackgroundIndexOutOfRange)
        );
    }

    #[test]
    fn background_disposal_without_a_global_table_is_refused() {
        let mut frame = frame_v1(1, 1, &[0]);
        frame.disposal = 2;
        frame.local_palette = Some(vec![RED, BLUE]);
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 1,
            height: 1,
            background_index: 0,
            global_palette: None,
            loop_count: None,
            graphic_control: true,
            frames: vec![frame],
        }
        .to_bytes();
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::BackgroundUndefined)
        );
    }

    #[test]
    fn trailing_bytes_after_the_trailer_are_refused() {
        let mut bytes = two_colour_spec_v1(vec![frame_v1(1, 1, &[0])]).to_bytes();
        bytes.push(0x00);
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::TrailingData)
        );
    }

    #[test]
    fn unsupported_extension_block_is_refused() {
        let mut bytes = two_colour_spec_v1(vec![frame_v1(1, 1, &[0])]).to_bytes();
        bytes.pop();
        bytes.extend_from_slice(&[0x21, 0x01, 4, 0, 0, 0, 0, 0]);
        bytes.push(0x3B);
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::UnsupportedExtension)
        );
    }

    #[test]
    fn truncated_stream_is_refused() {
        let bytes = two_colour_spec_v1(vec![frame_v1(1, 1, &[0])]).to_bytes();
        assert_eq!(
            inspect_gif_animation_v1(&bytes[..bytes.len() - 2]),
            Err(GifInspectionRefusalV1::TruncatedContainer)
        );
    }

    #[test]
    fn oversized_logical_screen_is_refused() {
        let bytes = GifSpecV1 {
            version: b"GIF89a",
            width: 8193,
            height: 1,
            background_index: 0,
            global_palette: Some(vec![RED, BLUE]),
            loop_count: None,
            graphic_control: true,
            frames: vec![frame_v1(1, 1, &[0])],
        }
        .to_bytes();
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::CanvasOutOfRange)
        );
    }
}
