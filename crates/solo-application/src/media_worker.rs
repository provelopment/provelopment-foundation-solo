//! Trusted-parent AVIF worker boundary (`R03.C3`, nonpublishing application slice).
//!
//! This module is the pure application half of the AVIF worker contract bound in Architecture
//! §6A.8. It owns the inward port a future operating-system worker adapter must implement, the
//! closed set of typed refusals that port can produce, a pure-Rust SHA-256 and the parent-side
//! envelope validator that decides whether a worker's *claim* may be retained at all.
//!
//! **What a success proves.** One [`MediaWorkerCheckedEnvelopeV1`] proves only that a claimed
//! envelope was internally consistent with the source bytes the parent already owned and with
//! the plane bytes the port handed back: the claimed source digest and byte count matched the
//! parent's own recomputation, the claimed geometry was inside the accepted bounds, the plane
//! ordinals, lengths and `stride × rows` arithmetic agreed, every per-plane digest and the
//! aggregate output digest matched the bytes actually received, and the host observations the
//! port reported were all positive. It proves **nothing** about pixels, colour, orientation,
//! MIME/magic agreement, AVIF or AV1 conformance, licensing, isolation on a real host or
//! publication authority. [`MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1`] and
//! [`MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1`] are the machine-checkable statement of that
//! boundary, and the retained type cannot override either.
//!
//! **What this module never does.** It opens no file, starts no process, imports no codec, OS,
//! network or adapter type, parses no AVIF `ftyp`/`av1C`/pixel data and offers no fallback. Every
//! observation comes from the inward port the caller supplies. A host fact such as "limits
//! verified" is a *reported* observation: the existence of the boolean, or of a fake that sets
//! it, is never an operating-system attestation.
//!
//! **Whole-candidate discipline.** The result is either every eligible envelope checked or one
//! refusal. A single failing format, source-byte gate, transport refusal, host observation,
//! envelope, plane or digest discards every earlier checked envelope, so a caller can never
//! mistake a partly checked inventory for a verified one.

use std::fmt;

use solo_domain::{PropertyKey, QualifiedOwnerV1};

use crate::graphic_asset_inventory::GraphicAssetObligationV1;
use crate::graphic_asset_read::GraphicAssetSourceSetV1;

/// The only worker protocol version this boundary understands.
pub const MEDIA_WORKER_PROTOCOL_VERSION_V1: u32 = 1;

/// The maximum number of source bytes one AVIF source may contribute.
///
/// This restates the accepted 20 MiB graphic source ceiling for the worker boundary: exactly this
/// many bytes is allowed, one byte more is refused, and a zero-byte source is refused before any
/// worker is consulted.
pub const MEDIA_WORKER_MAX_SOURCE_BYTES_V1: u64 = 20 * 1024 * 1024;

/// The maximum accepted size of one complete metadata envelope, whitespace included.
///
/// This bounds the parse, not the media: a worker that needs more than this to describe one still
/// image is refused rather than trusted.
pub const MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1: usize = 64 * 1024;

/// An internal defensive bound on the total claimed decoded output of one source.
///
/// It is **not** a new permitted image format, a released resource promise or a claim that a
/// decoder can honour it; it exists so the parent refuses an envelope whose plane lengths sum to
/// an implausible total instead of trusting the worker's arithmetic.
pub const MEDIA_WORKER_MAX_DECODED_BYTES_V1: u64 = 256 * 1024 * 1024;

/// The reference child address-space/commit profile bound in Architecture §6A.8.
pub const MEDIA_WORKER_REFERENCE_MEMORY_BYTES_V1: u64 = 536_870_912;

/// The reference child CPU-second profile bound in Architecture §6A.8.
pub const MEDIA_WORKER_REFERENCE_CPU_SECONDS_V1: u64 = 10;

/// The reference parent-observed wall deadline bound in Architecture §6A.8.
pub const MEDIA_WORKER_REFERENCE_WALL_MS_V1: u64 = 15_000;

/// Whether retaining one checked envelope authorizes publication of any kind.
///
/// Always `false`. A checked envelope is a data-handling result: it is not a publication
/// candidate, not a sealed artefact, not a route, not a visibility decision and not an
/// entitlement.
pub const MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false;

/// Whether retaining one checked envelope proves the media itself.
///
/// Always `false`. Every positive case this boundary can produce comes from a fabricated
/// protocol fixture or a fake port, so pixel correctness, colour semantics, orientation, MIME and
/// magic agreement and AVIF conformance all remain unverified.
pub const MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1: bool = false;

/// The smallest accepted image dimension, in pixels, per side.
const MEDIA_WORKER_MIN_DIMENSION_V1: u32 = 1;

/// The largest accepted image dimension, in pixels, per side.
const MEDIA_WORKER_MAX_DIMENSION_V1: u32 = 8_192;

/// The largest accepted decoded pixel count.
const MEDIA_WORKER_MAX_PIXELS_V1: u64 = 32_000_000;

/// The only source extension this slice dispatches to a worker.
const MEDIA_WORKER_DISPATCH_EXTENSION_V1: &str = "avif";

/// A SHA-256 digest, held as exactly thirty-two bytes.
///
/// Equality is byte equality, so a digest parsed from an untrusted envelope is only ever equal to
/// a digest this crate computed over the same bytes. The type carries no media claim of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaWorkerDigestV1([u8; 32]);

impl MediaWorkerDigestV1 {
    /// Wraps thirty-two already computed digest bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Parses exactly sixty-four lowercase hexadecimal characters.
    ///
    /// Uppercase, whitespace, a `0x` prefix, any other character, a short value and a long value
    /// are all rejected rather than normalised, because a lenient digest spelling would let two
    /// different textual claims name one digest.
    #[must_use]
    pub fn parse_lower_hex(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();
        if bytes.len() != 64 {
            return None;
        }
        let mut out = [0u8; 32];
        for (index, slot) in out.iter_mut().enumerate() {
            let high = lowercase_hex_digit(bytes[index * 2])?;
            let low = lowercase_hex_digit(bytes[index * 2 + 1])?;
            *slot = (high << 4) | low;
        }
        Some(Self(out))
    }

    /// The digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The digest as sixty-four lowercase hexadecimal characters.
    #[must_use]
    pub fn to_hex(&self) -> String {
        let mut text = String::with_capacity(64);
        for byte in self.0 {
            text.push(hex_digit(byte >> 4));
            text.push(hex_digit(byte & 0x0f));
        }
        text
    }
}

/// One lowercase hexadecimal digit value, or `None` for anything else.
fn lowercase_hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

/// One lowercase hexadecimal character for a nibble.
fn hex_digit(nibble: u8) -> char {
    match nibble {
        0..=9 => char::from(b'0' + nibble),
        _ => char::from(b'a' + (nibble - 10)),
    }
}

/// SHA-256 over raw bytes, with no third-party import.
///
/// The parent hashes the exact source bytes it already owns and the exact plane bytes the port
/// handed back. A digest supplied by a worker is never trusted without rehashing the
/// corresponding owned bytes here.
#[must_use]
pub fn sha256_media_worker_bytes_v1(bytes: &[u8]) -> MediaWorkerDigestV1 {
    let mut state = Sha256StateV1::new();
    state.update(bytes);
    state.finish()
}

/// The SHA-256 round constants, identical to the accepted Solo-owned reference routine.
const SHA256_K_V1: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// An incremental SHA-256 state.
///
/// Incremental hashing exists so the aggregate output digest can be computed over the canonical
/// plane encoding without materialising a second copy of up to 256 MiB of decoded output.
#[derive(Debug)]
struct Sha256StateV1 {
    state: [u32; 8],
    block: [u8; 64],
    block_len: usize,
    total_bytes: u64,
}

impl Sha256StateV1 {
    /// The FIPS-180 initial hash value.
    const fn new() -> Self {
        Self {
            state: [
                0x6a09_e667,
                0xbb67_ae85,
                0x3c6e_f372,
                0xa54f_f53a,
                0x510e_527f,
                0x9b05_688c,
                0x1f83_d9ab,
                0x5be0_cd19,
            ],
            block: [0u8; 64],
            block_len: 0,
            total_bytes: 0,
        }
    }

    /// Absorbs more message bytes.
    fn update(&mut self, mut data: &[u8]) {
        self.total_bytes = self.total_bytes.saturating_add(data.len() as u64);
        while !data.is_empty() {
            let take = (64 - self.block_len).min(data.len());
            self.block[self.block_len..self.block_len + take].copy_from_slice(&data[..take]);
            self.block_len += take;
            data = &data[take..];
            if self.block_len == 64 {
                let block = self.block;
                self.compress(&block);
                self.block_len = 0;
            }
        }
    }

    /// Applies the length-preserving padding bytes.
    fn pad(&mut self, byte: u8) {
        self.block[self.block_len] = byte;
        self.block_len += 1;
        if self.block_len == 64 {
            let block = self.block;
            self.compress(&block);
            self.block_len = 0;
        }
    }

    /// Applies the FIPS-180 padding and trailing length, then finishes the digest.
    fn finish(mut self) -> MediaWorkerDigestV1 {
        let bit_len = self.total_bytes.saturating_mul(8);
        self.pad(0x80);
        while self.block_len != 56 {
            self.pad(0x00);
        }
        for byte in bit_len.to_be_bytes() {
            self.pad(byte);
        }

        let mut out = [0u8; 32];
        for (index, word) in self.state.iter().enumerate() {
            out[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        MediaWorkerDigestV1::from_bytes(out)
    }

    /// The FIPS-180 compression function for one sixty-four-byte block.
    fn compress(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for (index, word) in w.iter_mut().take(16).enumerate() {
            let base = index * 4;
            *word = u32::from_be_bytes([
                block[base],
                block[base + 1],
                block[base + 2],
                block[base + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K_V1[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        for (slot, value) in self.state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }
}

/// Every way this boundary can refuse, with one stable token each.
///
/// The set is closed on purpose: a caller can branch on the category without parsing a message,
/// and no variant ever carries a path, a raw source byte, a plane byte or an untrusted string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerRefusalKindV1 {
    /// The acquired source was zero bytes long.
    SourceEmpty,
    /// The acquired source exceeded the accepted 20 MiB ceiling.
    SourceOversize,
    /// No worker could be reached, or its host result could not be trusted after cleanup.
    WorkerUnavailable,
    /// The worker's effective limits were not installed, were not read back, or were not the
    /// fixed reference profile.
    WorkerLimitUnverified,
    /// The worker did not report a normal exit.
    WorkerExitAbnormal,
    /// The parent-observed wall deadline was exceeded.
    WorkerWallTimeout,
    /// At least one descendant of the worker survived the run.
    WorkerDescendantsRemain,
    /// The envelope was structurally invalid, or named an unsupported protocol version.
    WorkerProtocolInvalid,
    /// The envelope's source byte count or source digest disagreed with the parent's own.
    WorkerSourceMismatch,
    /// The claimed geometry was outside the accepted bounds.
    WorkerDimensionsInvalid,
    /// A plane's ordinals, lengths, stride arithmetic, ownership or colour mapping disagreed.
    WorkerPlaneInvalid,
    /// A per-plane or aggregate digest disagreed with the parent's own rehash.
    WorkerDigestMismatch,
    /// The claimed decoded output exceeded the defensive envelope bound.
    WorkerOutputOversize,
    /// The source or the envelope named a format this slice does not verify.
    WorkerFormatUnsupported,
    /// A claim the boundary cannot accept as verified media evidence.
    WorkerMediaUnverified,
}

impl MediaWorkerRefusalKindV1 {
    /// The stable diagnostic token. It carries no path, byte, digest or handle.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SourceEmpty => "SOURCE_EMPTY",
            Self::SourceOversize => "SOURCE_OVERSIZE",
            Self::WorkerUnavailable => "WORKER_UNAVAILABLE",
            Self::WorkerLimitUnverified => "WORKER_LIMIT_UNVERIFIED",
            Self::WorkerExitAbnormal => "WORKER_EXIT_ABNORMAL",
            Self::WorkerWallTimeout => "WORKER_WALL_TIMEOUT",
            Self::WorkerDescendantsRemain => "WORKER_DESCENDANTS_REMAIN",
            Self::WorkerProtocolInvalid => "WORKER_PROTOCOL_INVALID",
            Self::WorkerSourceMismatch => "WORKER_SOURCE_MISMATCH",
            Self::WorkerDimensionsInvalid => "WORKER_DIMENSIONS_INVALID",
            Self::WorkerPlaneInvalid => "WORKER_PLANE_INVALID",
            Self::WorkerDigestMismatch => "WORKER_DIGEST_MISMATCH",
            Self::WorkerOutputOversize => "WORKER_OUTPUT_OVERSIZE",
            Self::WorkerFormatUnsupported => "WORKER_FORMAT_UNSUPPORTED",
            Self::WorkerMediaUnverified => "WORKER_MEDIA_UNVERIFIED",
        }
    }

    /// The fixed, bounded explanation. It never repeats an authored or worker-supplied value.
    #[must_use]
    pub const fn detail(self) -> &'static str {
        match self {
            Self::SourceEmpty => "the acquired source is empty and was never sent to a worker",
            Self::SourceOversize => "the acquired source exceeds the documented byte bound",
            Self::WorkerUnavailable => "no trustworthy worker result was available",
            Self::WorkerLimitUnverified => {
                "the worker's effective resource limits were not verified as required"
            }
            Self::WorkerExitAbnormal => "the worker did not report a normal exit",
            Self::WorkerWallTimeout => "the worker exceeded the parent-observed wall deadline",
            Self::WorkerDescendantsRemain => "a worker descendant survived the run",
            Self::WorkerProtocolInvalid => "the worker envelope is not a valid version 1 envelope",
            Self::WorkerSourceMismatch => {
                "the worker envelope disagrees with the parent's own source identity"
            }
            Self::WorkerDimensionsInvalid => {
                "the worker envelope's claimed geometry is outside the accepted bounds"
            }
            Self::WorkerPlaneInvalid => {
                "the worker envelope's plane structure is inconsistent with the received bytes"
            }
            Self::WorkerDigestMismatch => {
                "a worker digest disagrees with the parent's rehash of the owned bytes"
            }
            Self::WorkerOutputOversize => "the claimed decoded output exceeds the defensive bound",
            Self::WorkerFormatUnsupported => {
                "this slice verifies only one source format for every obligation"
            }
            Self::WorkerMediaUnverified => {
                "the worker claim cannot be accepted as verified media evidence"
            }
        }
    }
}

impl fmt::Display for MediaWorkerRefusalKindV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

/// One typed refusal, naming the authored obligation it concerns and nothing more.
///
/// The coordinates are the ones the author actually wrote. No path, byte, digest, handle or
/// authored value is carried, so a diagnostic can be rendered without leaking a source location,
/// a private declaration or worker-supplied text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaWorkerRefusalV1 {
    kind: MediaWorkerRefusalKindV1,
    declaring_owner: Option<Box<QualifiedOwnerV1>>,
    role: Option<PropertyKey>,
    local_index: Option<usize>,
}

impl MediaWorkerRefusalV1 {
    /// A refusal that concerns no single obligation.
    #[must_use]
    pub const fn of(kind: MediaWorkerRefusalKindV1) -> Self {
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
        kind: MediaWorkerRefusalKindV1,
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
    pub const fn kind(&self) -> MediaWorkerRefusalKindV1 {
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

impl fmt::Display for MediaWorkerRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.detail())
    }
}

impl std::error::Error for MediaWorkerRefusalV1 {}

/// The fixed resource profile one worker run is required to have been launched under.
///
/// These are the reference values bound in Architecture §6A.8. Carrying them in the contract keeps
/// "the run used the agreed budget" checkable, but a matching value is still only a *reported*
/// observation until a real supervisor proves it on a real host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaWorkerLimitsV1 {
    memory_bytes: u64,
    cpu_seconds: u64,
    wall_ms: u64,
}

impl MediaWorkerLimitsV1 {
    /// The reference engineering profile.
    #[must_use]
    pub const fn reference() -> Self {
        Self {
            memory_bytes: MEDIA_WORKER_REFERENCE_MEMORY_BYTES_V1,
            cpu_seconds: MEDIA_WORKER_REFERENCE_CPU_SECONDS_V1,
            wall_ms: MEDIA_WORKER_REFERENCE_WALL_MS_V1,
        }
    }

    /// An explicit profile, for a fake host observation that reports something other than the
    /// reference profile.
    #[must_use]
    pub const fn new(memory_bytes: u64, cpu_seconds: u64, wall_ms: u64) -> Self {
        Self {
            memory_bytes,
            cpu_seconds,
            wall_ms,
        }
    }

    /// The child address-space/commit bound, in bytes.
    #[must_use]
    pub const fn memory_bytes(&self) -> u64 {
        self.memory_bytes
    }

    /// The child CPU-second bound.
    #[must_use]
    pub const fn cpu_seconds(&self) -> u64 {
        self.cpu_seconds
    }

    /// The parent-observed wall deadline, in milliseconds.
    #[must_use]
    pub const fn wall_ms(&self) -> u64 {
        self.wall_ms
    }

    /// Whether this profile is exactly the reference profile.
    #[must_use]
    pub fn is_reference(&self) -> bool {
        *self == Self::reference()
    }
}

/// One already-acquired source, offered to a worker port under the fixed reference limits.
///
/// The request deliberately carries **no** path, authored URL, Entity identifier, route, exposure,
/// locale or operating-system handle: a worker cannot learn where the bytes came from, and it
/// cannot be asked to resolve anything else. Construction is the parent-side gate, so a zero-byte
/// or oversize source is refused before the port is ever called.
#[derive(Clone, PartialEq, Eq)]
pub struct MediaWorkerRequestV1<'a> {
    source: &'a [u8],
    source_length: u64,
    source_sha256: MediaWorkerDigestV1,
    limits: MediaWorkerLimitsV1,
}

impl<'a> MediaWorkerRequestV1<'a> {
    /// Validates the source-byte gate and hashes the exact bytes the parent owns.
    ///
    /// # Errors
    ///
    /// Returns [`MediaWorkerRefusalKindV1::SourceEmpty`] for zero bytes and
    /// [`MediaWorkerRefusalKindV1::SourceOversize`] for more than
    /// [`MEDIA_WORKER_MAX_SOURCE_BYTES_V1`]. Exactly the maximum is allowed.
    pub fn for_source(source: &'a [u8]) -> Result<Self, MediaWorkerRefusalKindV1> {
        let source_length =
            u64::try_from(source.len()).map_err(|_| MediaWorkerRefusalKindV1::SourceOversize)?;
        if source_length == 0 {
            return Err(MediaWorkerRefusalKindV1::SourceEmpty);
        }
        if source_length > MEDIA_WORKER_MAX_SOURCE_BYTES_V1 {
            return Err(MediaWorkerRefusalKindV1::SourceOversize);
        }
        Ok(Self {
            source,
            source_length,
            source_sha256: sha256_media_worker_bytes_v1(source),
            limits: MediaWorkerLimitsV1::reference(),
        })
    }

    /// The borrowed source bytes. They are never mutated and never leave the parent.
    #[must_use]
    pub const fn source(&self) -> &'a [u8] {
        self.source
    }

    /// The exact source byte count the parent measured.
    #[must_use]
    pub const fn source_length(&self) -> u64 {
        self.source_length
    }

    /// The parent's own SHA-256 of exactly those bytes.
    #[must_use]
    pub const fn source_sha256(&self) -> MediaWorkerDigestV1 {
        self.source_sha256
    }

    /// The fixed reference limits the run must be launched under.
    #[must_use]
    pub const fn limits(&self) -> MediaWorkerLimitsV1 {
        self.limits
    }
}

impl fmt::Debug for MediaWorkerRequestV1<'_> {
    /// Renders the length and digest only, so a diagnostic can never dump source bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MediaWorkerRequestV1")
            .field("source_length", &self.source_length)
            .field("source_sha256", &self.source_sha256.to_hex())
            .field("limits", &self.limits)
            .finish_non_exhaustive()
    }
}

/// The one outbound port this slice needs.
///
/// A future adapter owns the operating-system process boundary: creating the child, installing
/// and reading back limits, observing the wall deadline, reaping the tree and returning the
/// independently owned plane bytes. This trait is deliberately narrow — it cannot be asked to run
/// an arbitrary executable, resolve a path, read a file or return a native handle.
pub trait MediaWorkerPortV1 {
    /// Inspects one AVIF source and reports what the host observed plus the worker's claim.
    fn inspect_avif(&self, request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1;
}

/// How one worker process ended.
///
/// The two variants are distinct on purpose: an abort, a signal, a crash or a timeout is never
/// rendered as a normal exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerExitV1 {
    /// The child exited with a successful status of its own.
    Normal,
    /// The child aborted, was killed, was signalled or failed to start.
    Abnormal,
}

impl MediaWorkerExitV1 {
    /// The stable diagnostic token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Abnormal => "abnormal",
        }
    }
}

/// One host observation plus the untrusted claim and the independently owned plane bytes.
///
/// Every positive flag here is a *reported* observation. A fake port that sets them proves the
/// parent's data handling only; it is not an operating-system attestation and not isolation
/// evidence, and this type is documented that way so no caller can mistake it for one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaWorkerObservationV1 {
    limits_verified: bool,
    exit: MediaWorkerExitV1,
    wall_deadline_met: bool,
    descendants_reaped: bool,
    leak_observed: bool,
    effective_limits: MediaWorkerLimitsV1,
    envelope: MediaWorkerEnvelopeV1,
    planes: Vec<Vec<u8>>,
}

impl MediaWorkerObservationV1 {
    /// Records one observation exactly as the port reported it.
    ///
    /// This is a plain aggregate, not a validator: a hostile or incorrect observation is
    /// constructed here and then refused by [`inspect_media_worker_source_set_v1`].
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        limits_verified: bool,
        exit: MediaWorkerExitV1,
        wall_deadline_met: bool,
        descendants_reaped: bool,
        leak_observed: bool,
        effective_limits: MediaWorkerLimitsV1,
        envelope: MediaWorkerEnvelopeV1,
        planes: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            limits_verified,
            exit,
            wall_deadline_met,
            descendants_reaped,
            leak_observed,
            effective_limits,
            envelope,
            planes,
        }
    }

    /// Whether the port reported that the required limits were installed and read back.
    #[must_use]
    pub const fn limits_verified(&self) -> bool {
        self.limits_verified
    }

    /// How the child ended.
    #[must_use]
    pub const fn exit(&self) -> MediaWorkerExitV1 {
        self.exit
    }

    /// Whether the parent-observed wall deadline was met.
    #[must_use]
    pub const fn wall_deadline_met(&self) -> bool {
        self.wall_deadline_met
    }

    /// Whether the whole process tree was reaped.
    #[must_use]
    pub const fn descendants_reaped(&self) -> bool {
        self.descendants_reaped
    }

    /// Whether a resource, staging or handle leak was observed.
    #[must_use]
    pub const fn leak_observed(&self) -> bool {
        self.leak_observed
    }

    /// The effective limits the port reports the child ran under.
    #[must_use]
    pub const fn effective_limits(&self) -> MediaWorkerLimitsV1 {
        self.effective_limits
    }

    /// The untrusted metadata claim.
    #[must_use]
    pub const fn envelope(&self) -> &MediaWorkerEnvelopeV1 {
        &self.envelope
    }

    /// The independently owned plane bytes, in the order the port produced them.
    #[must_use]
    pub fn planes(&self) -> &[Vec<u8>] {
        &self.planes
    }
}

/// What one worker attempt produced.
///
/// A transport refusal is a first-class outcome, never an empty or "normal" observation, so an
/// abort, a timeout or an unreachable worker can never be mistaken for a successful run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaWorkerRunV1 {
    /// The parent observed a completed run and holds the claim plus owned plane bytes.
    Observed(MediaWorkerObservationV1),
    /// The transport refused before any envelope could be considered.
    Refused(MediaWorkerRefusalKindV1),
}

/// The only envelope status this boundary accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerStatusV1 {
    /// The worker claims it produced a decoded result.
    Decoded,
}

impl MediaWorkerStatusV1 {
    /// The exact wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Decoded => "decoded",
        }
    }
}

/// The only source format this slice verifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerFormatV1 {
    /// AVIF, the sole format this nonpublishing slice dispatches.
    Avif,
}

impl MediaWorkerFormatV1 {
    /// The exact wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Avif => "avif",
        }
    }
}

/// The only MIME token this boundary accepts alongside an AVIF claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerMimeV1 {
    /// `image/avif`.
    ImageAvif,
}

impl MediaWorkerMimeV1 {
    /// The exact wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImageAvif => "image/avif",
        }
    }
}

/// The colour models whose plane counts this boundary can check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerColorModelV1 {
    /// One luma plane.
    Gray,
    /// Three planes: luma and two chroma.
    Yuv,
    /// Four planes: luma, two chroma and alpha.
    Yuva,
}

impl MediaWorkerColorModelV1 {
    /// The exact wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gray => "gray",
            Self::Yuv => "yuv",
            Self::Yuva => "yuva",
        }
    }

    /// How many planes this model requires.
    #[must_use]
    pub const fn plane_count(self) -> usize {
        match self {
            Self::Gray => 1,
            Self::Yuv => 3,
            Self::Yuva => 4,
        }
    }

    /// Whether this model requires an alpha plane.
    #[must_use]
    pub const fn has_alpha(self) -> bool {
        matches!(self, Self::Yuva)
    }
}

/// The orientation fact a worker may claim.
///
/// No variant is a correctness statement. `Transposed` records that the worker says the raw planes
/// are transposed relative to the reported geometry; it is preserved exactly as claimed and never
/// used to infer that a plane's stride matches the displayed width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaWorkerOrientationV1 {
    /// The worker claims the planes are in displayed orientation.
    Identity,
    /// The worker claims the planes are transposed relative to the displayed geometry.
    Transposed,
    /// The worker makes no orientation claim.
    Unverified,
}

impl MediaWorkerOrientationV1 {
    /// The exact wire token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Transposed => "transposed",
            Self::Unverified => "unverified",
        }
    }

    /// The exact wire token for the three accepted spellings.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "identity" => Some(Self::Identity),
            "transposed" => Some(Self::Transposed),
            "unverified" => Some(Self::Unverified),
            _ => None,
        }
    }
}

impl MediaWorkerColorModelV1 {
    /// The colour model for the three accepted wire spellings.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "gray" => Some(Self::Gray),
            "yuv" => Some(Self::Yuv),
            "yuva" => Some(Self::Yuva),
            _ => None,
        }
    }
}

/// One claimed plane inside an envelope.
///
/// The claim is data, not evidence: `length` is compared against the bytes actually received and
/// against `stride × rows`, and `sha256` is compared against the parent's own rehash.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaWorkerPlaneClaimV1 {
    ordinal: u32,
    stride: u64,
    rows: u64,
    length: u64,
    sha256: MediaWorkerDigestV1,
}

impl MediaWorkerPlaneClaimV1 {
    /// Records one claim exactly as reported.
    #[must_use]
    pub const fn new(
        ordinal: u32,
        stride: u64,
        rows: u64,
        length: u64,
        sha256: MediaWorkerDigestV1,
    ) -> Self {
        Self {
            ordinal,
            stride,
            rows,
            length,
            sha256,
        }
    }

    /// The zero-based plane ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    /// The claimed row stride, in bytes.
    #[must_use]
    pub const fn stride(&self) -> u64 {
        self.stride
    }

    /// The claimed row count.
    #[must_use]
    pub const fn rows(&self) -> u64 {
        self.rows
    }

    /// The claimed plane byte length.
    #[must_use]
    pub const fn length(&self) -> u64 {
        self.length
    }

    /// The claimed SHA-256 of exactly this plane's raw bytes.
    #[must_use]
    pub const fn sha256(&self) -> MediaWorkerDigestV1 {
        self.sha256
    }
}

/// A worker's metadata claim about one source.
///
/// This type is **untrusted input**. Holding one proves nothing: every field is compared against
/// the parent's own source bytes, plane bytes and bounds by
/// [`inspect_media_worker_source_set_v1`], and only that validator can produce a
/// [`MediaWorkerCheckedEnvelopeV1`]. There is no `is_valid` flag, no success marker and no
/// constructor that skips validation, because a marker a caller could set would be a claim about
/// evidence rather than evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaWorkerEnvelopeV1 {
    protocol_version: u32,
    status: MediaWorkerStatusV1,
    format: MediaWorkerFormatV1,
    mime: MediaWorkerMimeV1,
    source_length: u64,
    source_sha256: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_model: MediaWorkerColorModelV1,
    has_alpha: bool,
    is_grid: bool,
    is_sequence: bool,
    orientation: MediaWorkerOrientationV1,
    planes: Vec<MediaWorkerPlaneClaimV1>,
    output_sha256: MediaWorkerDigestV1,
}

impl MediaWorkerEnvelopeV1 {
    /// Records one claim exactly as reported, without judging it.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        protocol_version: u32,
        status: MediaWorkerStatusV1,
        format: MediaWorkerFormatV1,
        mime: MediaWorkerMimeV1,
        source_length: u64,
        source_sha256: MediaWorkerDigestV1,
        width: u32,
        height: u32,
        bit_depth: u8,
        color_model: MediaWorkerColorModelV1,
        has_alpha: bool,
        is_grid: bool,
        is_sequence: bool,
        orientation: MediaWorkerOrientationV1,
        planes: Vec<MediaWorkerPlaneClaimV1>,
        output_sha256: MediaWorkerDigestV1,
    ) -> Self {
        Self {
            protocol_version,
            status,
            format,
            mime,
            source_length,
            source_sha256,
            width,
            height,
            bit_depth,
            color_model,
            has_alpha,
            is_grid,
            is_sequence,
            orientation,
            planes,
            output_sha256,
        }
    }

    /// The claimed protocol version.
    #[must_use]
    pub const fn protocol_version(&self) -> u32 {
        self.protocol_version
    }

    /// The claimed status.
    #[must_use]
    pub const fn status(&self) -> MediaWorkerStatusV1 {
        self.status
    }

    /// The claimed format.
    #[must_use]
    pub const fn format(&self) -> MediaWorkerFormatV1 {
        self.format
    }

    /// The claimed MIME token.
    #[must_use]
    pub const fn mime(&self) -> MediaWorkerMimeV1 {
        self.mime
    }

    /// The claimed source byte count.
    #[must_use]
    pub const fn source_length(&self) -> u64 {
        self.source_length
    }

    /// The claimed source digest.
    #[must_use]
    pub const fn source_sha256(&self) -> MediaWorkerDigestV1 {
        self.source_sha256
    }

    /// The claimed width, in pixels.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The claimed height, in pixels.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The claimed bit depth.
    #[must_use]
    pub const fn bit_depth(&self) -> u8 {
        self.bit_depth
    }

    /// The claimed colour model.
    #[must_use]
    pub const fn color_model(&self) -> MediaWorkerColorModelV1 {
        self.color_model
    }

    /// Whether the worker claims an alpha channel.
    #[must_use]
    pub const fn has_alpha(&self) -> bool {
        self.has_alpha
    }

    /// Whether the worker claims a grid image.
    #[must_use]
    pub const fn is_grid(&self) -> bool {
        self.is_grid
    }

    /// Whether the worker claims an image sequence.
    #[must_use]
    pub const fn is_sequence(&self) -> bool {
        self.is_sequence
    }

    /// The claimed orientation fact.
    #[must_use]
    pub const fn orientation(&self) -> MediaWorkerOrientationV1 {
        self.orientation
    }

    /// The claimed planes, in the order reported.
    #[must_use]
    pub fn planes(&self) -> &[MediaWorkerPlaneClaimV1] {
        &self.planes
    }

    /// The claimed aggregate decoded-output digest.
    #[must_use]
    pub const fn output_sha256(&self) -> MediaWorkerDigestV1 {
        self.output_sha256
    }
}

/// One envelope that survived parent-side validation, attributed to its authored obligation.
///
/// This is an **internal structural and provenance check only**. It is not a
/// [`crate::graphic_asset_read::GraphicAssetSourceSetV1`], not a publication candidate, not a
/// sealable artefact and not evidence that the media is correct. Both
/// [`Self::grants_publication_authority`] and [`Self::proves_media_correctness`] are constant
/// `false` and cannot be overridden or configured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaWorkerCheckedEnvelopeV1 {
    obligation: GraphicAssetObligationV1,
    source_length: u64,
    source_sha256: MediaWorkerDigestV1,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_model: MediaWorkerColorModelV1,
    plane_count: usize,
    output_sha256: MediaWorkerDigestV1,
}

impl MediaWorkerCheckedEnvelopeV1 {
    /// The authored obligation this check answers.
    #[must_use]
    pub const fn obligation(&self) -> &GraphicAssetObligationV1 {
        &self.obligation
    }

    /// The verified source byte count, re-measured by the parent.
    #[must_use]
    pub const fn source_length(&self) -> u64 {
        self.source_length
    }

    /// The verified source digest, recomputed by the parent.
    #[must_use]
    pub const fn source_sha256(&self) -> MediaWorkerDigestV1 {
        self.source_sha256
    }

    /// The claimed width that survived the bounds check.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The claimed height that survived the bounds check.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The claimed bit depth, preserved as claimed and never treated as conformance.
    #[must_use]
    pub const fn bit_depth(&self) -> u8 {
        self.bit_depth
    }

    /// The claimed colour model, checked against the plane count.
    #[must_use]
    pub const fn color_model(&self) -> MediaWorkerColorModelV1 {
        self.color_model
    }

    /// How many planes were checked.
    #[must_use]
    pub const fn plane_count(&self) -> usize {
        self.plane_count
    }

    /// The aggregate digest the parent itself recomputed.
    #[must_use]
    pub const fn output_sha256(&self) -> MediaWorkerDigestV1 {
        self.output_sha256
    }

    /// Whether this check authorizes publication. Always `false`.
    #[must_use]
    pub const fn grants_publication_authority(&self) -> bool {
        MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1
    }

    /// Whether this check proves the media itself. Always `false`.
    #[must_use]
    pub const fn proves_media_correctness(&self) -> bool {
        MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1
    }
}

/// The outcome of one whole-source-set worker attempt.
///
/// There is no third case: a set either produced one checked envelope per eligible obligation or
/// produced one refusal and no checked envelopes at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaWorkerBoundaryOutcomeV1 {
    /// Every eligible obligation was checked.
    EnvelopeChecked(Vec<MediaWorkerCheckedEnvelopeV1>),
    /// One obligation, or the transport, refused the whole attempt.
    Refused(MediaWorkerRefusalV1),
}

impl MediaWorkerBoundaryOutcomeV1 {
    /// The checked envelopes, if the attempt succeeded.
    #[must_use]
    pub fn checked(&self) -> Option<&[MediaWorkerCheckedEnvelopeV1]> {
        match self {
            Self::EnvelopeChecked(checked) => Some(checked),
            Self::Refused(_) => None,
        }
    }

    /// The one refusal, if the attempt did not succeed.
    #[must_use]
    pub const fn refusal(&self) -> Option<&MediaWorkerRefusalV1> {
        match self {
            Self::EnvelopeChecked(_) => None,
            Self::Refused(refusal) => Some(refusal),
        }
    }
}

/// The canonical V1 aggregate digest of one claimed plane set and its owned bytes.
///
/// The encoding is fixed and unambiguous: for every plane in ordinal order,
/// `ordinal:u32 LE || stride:u64 LE || rows:u64 LE || length:u64 LE || raw plane bytes`, hashed as
/// one SHA-256 stream. It is a pure function: it judges nothing, and a caller that pairs planes
/// with the wrong buffers gets the digest of that wrong pairing. Pairs beyond the shorter of the
/// two slices are ignored, so a caller must compare lengths itself — which the validator does
/// before calling this.
#[must_use]
pub fn media_worker_output_sha256_v1(
    planes: &[MediaWorkerPlaneClaimV1],
    plane_bytes: &[Vec<u8>],
) -> MediaWorkerDigestV1 {
    let mut state = Sha256StateV1::new();
    for (claim, bytes) in planes.iter().zip(plane_bytes.iter()) {
        state.update(&claim.ordinal.to_le_bytes());
        state.update(&claim.stride.to_le_bytes());
        state.update(&claim.rows.to_le_bytes());
        state.update(&claim.length.to_le_bytes());
        state.update(bytes);
    }
    state.finish()
}

/// Whether one confined logical path or reference text names the format this slice dispatches.
///
/// This is a dispatch decision only. An extension is not a media validation result: the bytes are
/// never parsed here, and a `.avif` name says nothing about the content behind it.
fn names_avif_source(logical_path: &str) -> bool {
    match logical_path.rsplit_once('.') {
        Some((_, extension)) => extension == MEDIA_WORKER_DISPATCH_EXTENSION_V1,
        None => false,
    }
}

/// Inspects every acquired obligation of one complete source set through the worker port.
///
/// Every acquired obligation is visited in the inventory order the set already carries, including
/// `internal`, unlisted and descendant-suppressed declarations, and this slice dispatches only a
/// `.avif` source. Any other image format returns
/// [`MediaWorkerRefusalKindV1::WorkerFormatUnsupported`] for the **entire** attempt: a partial
/// success would falsely suggest that more than one format had been verified.
///
/// Per obligation the parent first applies its own source gate — zero bytes and more than 20 MiB
/// are refused before the port is called, and exactly 20 MiB is allowed through — then builds the
/// request (hashing the exact bytes it owns) and only then consults the port. The returned host
/// observation and claim are re-checked in a fixed order: transport refusal, effective limits,
/// exit status, wall deadline, descendant reaping, leak, protocol version, format, source
/// identity, geometry, bit depth, colour/plane mapping, ordinals, `stride × rows` arithmetic,
/// claimed total output, received plane count and lengths, per-plane digests, aggregate digest.
///
/// One failure discards every earlier checked envelope and returns the first typed refusal, so the
/// result is all-or-refused.
#[must_use]
pub fn inspect_media_worker_source_set_v1<P: MediaWorkerPortV1 + ?Sized>(
    set: &GraphicAssetSourceSetV1,
    port: &P,
) -> MediaWorkerBoundaryOutcomeV1 {
    let mut checked: Vec<MediaWorkerCheckedEnvelopeV1> = Vec::new();
    for evidence in set.evidence() {
        let obligation = evidence.obligation();
        if !names_avif_source(evidence.logical_path()) {
            return MediaWorkerBoundaryOutcomeV1::Refused(MediaWorkerRefusalV1::concerning(
                MediaWorkerRefusalKindV1::WorkerFormatUnsupported,
                obligation,
            ));
        }
        let request = match MediaWorkerRequestV1::for_source(evidence.bytes()) {
            Ok(request) => request,
            Err(kind) => {
                return MediaWorkerBoundaryOutcomeV1::Refused(MediaWorkerRefusalV1::concerning(
                    kind, obligation,
                ));
            }
        };
        let observation = match port.inspect_avif(&request) {
            MediaWorkerRunV1::Observed(observation) => observation,
            MediaWorkerRunV1::Refused(kind) => {
                return MediaWorkerBoundaryOutcomeV1::Refused(MediaWorkerRefusalV1::concerning(
                    kind, obligation,
                ));
            }
        };
        match check_observation_v1(obligation, &request, &observation) {
            Ok(record) => checked.push(record),
            Err(kind) => {
                return MediaWorkerBoundaryOutcomeV1::Refused(MediaWorkerRefusalV1::concerning(
                    kind, obligation,
                ));
            }
        }
    }
    MediaWorkerBoundaryOutcomeV1::EnvelopeChecked(checked)
}

/// Re-checks one observation and its claim against what the parent itself owns and computed.
///
/// The check order is fixed so a refusal category is deterministic for a given hostile input. Every
/// host fact is evaluated before any envelope field, so a run that did not demonstrably complete
/// under the agreed limits is refused without ever reading its claim.
fn check_observation_v1(
    obligation: &GraphicAssetObligationV1,
    request: &MediaWorkerRequestV1<'_>,
    observation: &MediaWorkerObservationV1,
) -> Result<MediaWorkerCheckedEnvelopeV1, MediaWorkerRefusalKindV1> {
    if !observation.limits_verified() || !observation.effective_limits().is_reference() {
        return Err(MediaWorkerRefusalKindV1::WorkerLimitUnverified);
    }
    if observation.exit() != MediaWorkerExitV1::Normal {
        return Err(MediaWorkerRefusalKindV1::WorkerExitAbnormal);
    }
    if !observation.wall_deadline_met() {
        return Err(MediaWorkerRefusalKindV1::WorkerWallTimeout);
    }
    if !observation.descendants_reaped() {
        return Err(MediaWorkerRefusalKindV1::WorkerDescendantsRemain);
    }
    if observation.leak_observed() {
        // An unreleased handle, an unremoved staging entry or an unclosed resource means the host
        // result cannot be trusted, so it is a transport-level refusal rather than a claim check.
        return Err(MediaWorkerRefusalKindV1::WorkerUnavailable);
    }

    let envelope = observation.envelope();
    if envelope.protocol_version() != MEDIA_WORKER_PROTOCOL_VERSION_V1 {
        return Err(MediaWorkerRefusalKindV1::WorkerProtocolInvalid);
    }
    if envelope.format() != MediaWorkerFormatV1::Avif {
        return Err(MediaWorkerRefusalKindV1::WorkerFormatUnsupported);
    }
    if envelope.source_length() != request.source_length()
        || envelope.source_sha256() != request.source_sha256()
    {
        return Err(MediaWorkerRefusalKindV1::WorkerSourceMismatch);
    }

    let pixel_count = u64::from(envelope.width())
        .checked_mul(u64::from(envelope.height()))
        .ok_or(MediaWorkerRefusalKindV1::WorkerDimensionsInvalid)?;
    if !(MEDIA_WORKER_MIN_DIMENSION_V1..=MEDIA_WORKER_MAX_DIMENSION_V1).contains(&envelope.width())
        || !(MEDIA_WORKER_MIN_DIMENSION_V1..=MEDIA_WORKER_MAX_DIMENSION_V1)
            .contains(&envelope.height())
        || pixel_count > MEDIA_WORKER_MAX_PIXELS_V1
    {
        return Err(MediaWorkerRefusalKindV1::WorkerDimensionsInvalid);
    }

    if !matches!(envelope.bit_depth(), 8 | 10 | 12) {
        return Err(MediaWorkerRefusalKindV1::WorkerMediaUnverified);
    }

    let planes = envelope.planes();
    if planes.len() != envelope.color_model().plane_count()
        || envelope.has_alpha() != envelope.color_model().has_alpha()
    {
        return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
    }
    let received = observation.planes();
    if received.len() != planes.len() {
        return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
    }

    let mut total_output: u64 = 0;
    for (index, claim) in planes.iter().enumerate() {
        let expected_ordinal =
            u32::try_from(index).map_err(|_| MediaWorkerRefusalKindV1::WorkerPlaneInvalid)?;
        if claim.ordinal() != expected_ordinal {
            return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }
        if claim.stride() == 0 || claim.rows() == 0 || claim.length() == 0 {
            return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }
        let stride_rows = claim
            .stride()
            .checked_mul(claim.rows())
            .ok_or(MediaWorkerRefusalKindV1::WorkerPlaneInvalid)?;
        if stride_rows != claim.length() {
            return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }
        // The claimed total is bounded before any byte is read, so an implausible claim cannot
        // make the parent work through a large buffer to discover it.
        total_output = total_output
            .checked_add(claim.length())
            .ok_or(MediaWorkerRefusalKindV1::WorkerOutputOversize)?;
        if total_output > MEDIA_WORKER_MAX_DECODED_BYTES_V1 {
            return Err(MediaWorkerRefusalKindV1::WorkerOutputOversize);
        }
    }

    for (index, claim) in planes.iter().enumerate() {
        let actual = u64::try_from(received[index].len())
            .map_err(|_| MediaWorkerRefusalKindV1::WorkerPlaneInvalid)?;
        if actual != claim.length() {
            return Err(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }
    }

    for (index, claim) in planes.iter().enumerate() {
        if sha256_media_worker_bytes_v1(&received[index]) != claim.sha256() {
            return Err(MediaWorkerRefusalKindV1::WorkerDigestMismatch);
        }
    }
    if media_worker_output_sha256_v1(planes, received) != envelope.output_sha256() {
        return Err(MediaWorkerRefusalKindV1::WorkerDigestMismatch);
    }

    Ok(MediaWorkerCheckedEnvelopeV1 {
        obligation: obligation.clone(),
        source_length: request.source_length(),
        source_sha256: request.source_sha256(),
        width: envelope.width(),
        height: envelope.height(),
        bit_depth: envelope.bit_depth(),
        color_model: envelope.color_model(),
        plane_count: planes.len(),
        output_sha256: envelope.output_sha256(),
    })
}
