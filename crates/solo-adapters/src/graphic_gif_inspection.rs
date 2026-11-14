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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GifContainerFactsV1 {
    signature: &'static str,
    width: u16,
    height: u16,
    frames: usize,
    loop_semantics: GifLoopV1,
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
    let mut at = 13usize;
    if packed & 0x80 != 0 {
        let entries = 1usize << (usize::from(packed & 0x07) + 1);
        at = at
            .checked_add(entries * 3)
            .ok_or(Refusal::TruncatedContainer)?;
        if at > source.len() {
            return Err(Refusal::TruncatedContainer);
        }
    }
    let canvas_pixels = u64::from(width) * u64::from(height);
    let mut frames = 0usize;
    let mut loop_semantics = GifLoopV1::Absent;
    let mut repeat_seen = false;
    loop {
        let Some(&introducer) = source.get(at) else {
            return Err(Refusal::MissingTrailer);
        };
        match introducer {
            0x21 => {
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
pub fn inspect_gif_animation_v1(
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
        frames.push(GifFrameEvidenceV1 {
            delay_centiseconds: frame.delay,
            left: frame.left,
            top: frame.top,
            width: frame.width,
            height: frame.height,
            disposal,
            transparent_index: frame.transparent,
            interlaced: frame.interlaced,
            local_palette: frame.palette.is_some(),
            composite_digest,
        });
        match disposal {
            GifDisposalV1::Background => {
                for row in 0..frame_height {
                    let destination_row = (top + row) * canvas_stride;
                    for column in 0..frame_width {
                        let at = destination_row + (left + column) * 4;
                        canvas[at..at + 4].fill(0);
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

    struct FrameSpecV1 {
        delay: u16,
        disposal: u8,
        index: u8,
        transparent: Option<u8>,
    }

    /// Deterministic, self-generated one-pixel LZW payload: clear, pixel, end-of-information.
    fn lzw_one_pixel_v1(index: u8) -> [u8; 2] {
        let mut bits = 0u32;
        let mut width = 0u8;
        for code in [4u16, u16::from(index), 5u16] {
            bits |= u32::from(code) << width;
            width += 3;
        }
        [(bits & 0xFF) as u8, ((bits >> 8) & 0xFF) as u8]
    }

    /// Build a tiny, complete, self-generated GIF with a two-entry global palette.
    fn build_gif_v1(
        version: &[u8; 6],
        width: u16,
        height: u16,
        frames: &[FrameSpecV1],
        loop_count: Option<u16>,
    ) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(version);
        out.extend_from_slice(&width.to_le_bytes());
        out.extend_from_slice(&height.to_le_bytes());
        out.extend_from_slice(&[0x80, 0x00, 0x00]);
        out.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        if let Some(count) = loop_count {
            out.extend_from_slice(&[0x21, 0xFF, 11]);
            out.extend_from_slice(b"NETSCAPE2.0");
            out.extend_from_slice(&[3, 1]);
            out.extend_from_slice(&count.to_le_bytes());
            out.push(0);
        }
        for frame in frames {
            let packed = (frame.disposal << 2) | u8::from(frame.transparent.is_some());
            out.extend_from_slice(&[
                0x21,
                0xF9,
                4,
                packed,
                (frame.delay & 0xFF) as u8,
                (frame.delay >> 8) as u8,
                frame.transparent.unwrap_or(0),
                0,
            ]);
            out.push(0x2C);
            out.extend_from_slice(&[0, 0, 0, 0, 1, 0, 1, 0, 0]);
            out.push(2);
            let payload = lzw_one_pixel_v1(frame.index);
            out.push(2);
            out.extend_from_slice(&payload);
            out.push(0);
        }
        out.push(0x3B);
        out
    }

    fn still_frames_v1() -> Vec<FrameSpecV1> {
        vec![FrameSpecV1 {
            delay: 0,
            disposal: 1,
            index: 0,
            transparent: None,
        }]
    }

    #[test]
    fn gif87a_still_is_one_frame_and_not_an_animation() {
        let bytes = build_gif_v1(b"GIF87a", 1, 1, &still_frames_v1(), None);
        let evidence = inspect_gif_animation_v1(&bytes).expect("still GIF87a is accepted");
        assert_eq!(evidence.signature(), GIF87A_SIGNATURE_V1);
        assert_eq!(evidence.mime(), GIF_MIME_V1);
        assert_eq!(evidence.frame_count(), 1);
        assert!(!evidence.is_animated());
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(evidence.one_cycle_delay_ms(), 0);
        assert_eq!(evidence.total_composited_pixels(), 1);
        assert_eq!(
            evidence.source_digest(),
            &sha256_media_worker_bytes_v1(&bytes)
        );
        assert_eq!(
            evidence.frames()[0].composite_digest(),
            &sha256_media_worker_bytes_v1(&[255, 0, 0, 255])
        );
    }

    #[test]
    fn finite_repeat_count_is_preserved_exactly() {
        let bytes = build_gif_v1(b"GIF89a", 1, 1, &still_frames_v1(), Some(3));
        let evidence = inspect_gif_animation_v1(&bytes).expect("finite repeat is accepted");
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Finite(3));
    }

    #[test]
    fn trailing_bytes_after_the_trailer_are_refused() {
        let mut bytes = build_gif_v1(b"GIF89a", 1, 1, &still_frames_v1(), None);
        bytes.push(0x00);
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::TrailingData)
        );
    }

    #[test]
    fn unsupported_extension_block_is_refused() {
        let mut bytes = build_gif_v1(b"GIF89a", 1, 1, &still_frames_v1(), None);
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
        let bytes = build_gif_v1(b"GIF89a", 1, 1, &still_frames_v1(), None);
        let truncated = &bytes[..bytes.len() - 2];
        assert_eq!(
            inspect_gif_animation_v1(truncated),
            Err(GifInspectionRefusalV1::TruncatedContainer)
        );
    }

    #[test]
    fn oversized_logical_screen_is_refused() {
        let bytes = build_gif_v1(b"GIF89a", 8193, 1, &still_frames_v1(), None);
        assert_eq!(
            inspect_gif_animation_v1(&bytes),
            Err(GifInspectionRefusalV1::CanvasOutOfRange)
        );
    }
    #[test]
    fn gif89a_multi_frame_preserves_delays_disposal_and_composition() {
        let frames = vec![
            FrameSpecV1 {
                delay: 10,
                disposal: 1,
                index: 0,
                transparent: None,
            },
            FrameSpecV1 {
                delay: 20,
                disposal: 2,
                index: 0,
                transparent: Some(0),
            },
            FrameSpecV1 {
                delay: 5,
                disposal: 1,
                index: 0,
                transparent: Some(0),
            },
        ];
        let bytes = build_gif_v1(b"GIF89a", 1, 1, &frames, Some(0));
        let evidence = inspect_gif_animation_v1(&bytes).expect("multi-frame GIF89a is accepted");
        assert_eq!(evidence.signature(), GIF89A_SIGNATURE_V1);
        assert_eq!(evidence.frame_count(), 3);
        assert!(evidence.is_animated());
        assert_eq!(evidence.loop_semantics(), GifLoopV1::Infinite);
        assert!(evidence.loop_semantics().is_infinite());
        assert_eq!(evidence.one_cycle_delay_ms(), 350);
        assert_eq!(evidence.total_composited_pixels(), 3);
        assert_eq!(evidence.frames()[0].delay_centiseconds(), 10);
        assert_eq!(evidence.frames()[1].delay_centiseconds(), 20);
        assert_eq!(evidence.frames()[2].delay_centiseconds(), 5);
        assert_eq!(evidence.frames()[0].disposal(), GifDisposalV1::Keep);
        assert_eq!(evidence.frames()[1].disposal(), GifDisposalV1::Background);
        assert_eq!(evidence.frames()[1].transparent_index(), Some(0));
        let red = sha256_media_worker_bytes_v1(&[255, 0, 0, 255]);
        let cleared = sha256_media_worker_bytes_v1(&[0, 0, 0, 0]);
        assert_eq!(evidence.frames()[0].composite_digest(), &red);
        assert_eq!(evidence.frames()[1].composite_digest(), &red);
        assert_eq!(evidence.frames()[2].composite_digest(), &cleared);
        assert_ne!(
            evidence.frames()[1].composite_digest(),
            evidence.frames()[2].composite_digest()
        );
        let transcript = evidence.transcript_v1();
        assert!(transcript.contains("frames=3"));
        assert!(transcript.contains("frame=0 delay_cs=10"));
        assert!(transcript.contains("frame=1 delay_cs=20"));
        assert!(transcript.contains("frame=2 delay_cs=5"));
        assert_eq!(
            evidence.transcript_digest(),
            &sha256_media_worker_bytes_v1(transcript.as_bytes())
        );
    }
}
