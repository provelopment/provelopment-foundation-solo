//! The first real confined AVIF coded-plane worker and its trusted-parent port
//! (`SOLO-WO-118 R05` §3, Architecture §6A.8).
//!
//! **What exists here.** One safe adapter module with exactly two halves:
//!
//! * [`run_avif_worker_child_v1`] runs *inside* the already confined `solo-cli` child. It proves
//!   its own confinement profile first, reads one bounded owned source from stdin, checks the
//!   caller's digest, preflights the container with the pinned decoder's typed inspection, decodes
//!   and emits one canonical V1 envelope plus the exact concatenated Y/U/V bytes — all built in
//!   memory before a single byte is written.
//! * [`ConfinedAvifInspectAdapterV1`] runs in the trusted parent. It launches only the trusted
//!   executable through the existing single-use supervisor, parses exactly one envelope with the
//!   unchanged strict parser, splits the received bytes into exactly the claimed three planes and
//!   reports only honestly observed host facts.
//!
//! **What this is not.** It is not display orientation, alpha association, grid, sequence,
//! publication or asset-library authority. The claim is deliberately `orientation=unverified`,
//! `color_model=yuv`, `has_alpha=false`, `is_grid=false`, `is_sequence=false`, and the resulting
//! record is nonpublishing evidence of coded planes only. The accepted V3 `avif-unsupported-v3`
//! refusal is untouched: no V3 path reaches this module.
//!
//! **Refusals are whole.** Every failure returns one fixed exit status and writes nothing, so a
//! refusal can never look like a partial or display-ready success.

use std::io::{Read, Write};
use std::path::PathBuf;

use solo_application::{
    GraphicAssetSourceSetV1, IMAGE_WORKER_WALL_MS_V1, MAX_IMAGE_PIXELS_V1,
    MAX_IMAGE_SOURCE_BYTES_V1, MediaWorkerBoundaryOutcomeV1, MediaWorkerDigestV1,
    MediaWorkerExitV1, MediaWorkerLimitsV1, MediaWorkerObservationV1, MediaWorkerPlaneClaimV1,
    MediaWorkerPortV1, MediaWorkerRefusalKindV1, MediaWorkerRequestV1, MediaWorkerRunV1,
    inspect_media_worker_source_set_v1, media_worker_output_sha256_v1,
    sha256_media_worker_bytes_v1,
};

use crate::graphic_image_inspection::{
    IMAGE_WORKER_EXIT_DECODE_FAILED_V1, IMAGE_WORKER_EXIT_INVALID_MEDIA_V1,
    IMAGE_WORKER_EXIT_REFUSED_V1, IMAGE_WORKER_EXIT_UNCONFINED_V1,
};
use crate::media_worker_host::run_confined_image_worker_v1;
use crate::media_worker_protocol::parse_media_worker_envelope_v1;

/// The one closed format token this worker answers to.
pub const AVIF_WORKER_TOKEN_V1: &str = "avif";

/// The maximum accepted size of the JSON header line, matching the still-image frame ceiling.
const MAX_AVIF_HEADER_BYTES_V1: usize = 1024;

/// The maximum accepted width or height of one coded AVIF still.
const MAX_AVIF_DIMENSION_V1: u32 = 8_192;

/// One fully checked coded plane: canonical stride, row count and the actual bytes.
struct CheckedPlaneV1 {
    ordinal: u32,
    stride: u64,
    rows: u64,
    bytes: Vec<u8>,
}

/// The complete decoded claim, built before any output is attempted.
struct CheckedStillV1 {
    width: u32,
    height: u32,
    bit_depth: u8,
    planes: Vec<CheckedPlaneV1>,
}

/// Sample width in bytes for an admitted coded depth: 8-bit is one byte, 10/12-bit two.
fn sample_bytes_v1(depth: u8) -> Option<u64> {
    match depth {
        8 => Some(1),
        10 | 12 => Some(2),
        _ => None,
    }
}

/// The admitted sub-sampling pair: only 4:2:0 and 4:4:4 are in scope for this slice.
fn subsampling_v1(pair: Option<(bool, bool)>) -> Option<(u64, u64)> {
    match pair {
        Some((true, true)) => Some((2, 2)),
        Some((false, false)) => Some((1, 1)),
        _ => None,
    }
}

/// The canonical (columns, rows) of one plane at a sub-sampling, or `None` on overflow.
fn plane_extent_v1(width: u64, height: u64, sub_x: u64, sub_y: u64) -> Option<(u64, u64)> {
    let columns = width
        .checked_add(sub_x.checked_sub(1)?)?
        .checked_div(sub_x)?;
    let rows = height
        .checked_add(sub_y.checked_sub(1)?)?
        .checked_div(sub_y)?;
    Some((columns, rows))
}

/// Preflights one owned source with the pinned decoder, then decodes and checks the real planes.
///
/// The preflight refuses every out-of-scope class **before** any pixel buffer is allocated: a
/// sequence brand, a grid primary, an alpha auxiliary, monochrome, an unsupported depth, an
/// unadmitted sub-sampling and any out-of-bounds geometry. Classification comes from the actual
/// container's typed inspection, never from a file name, digest or fixed pixel table.
fn decode_checked_still_v1(source: &[u8]) -> Result<CheckedStillV1, u8> {
    let info =
        oxideav_avif::inspect::inspect(source).map_err(|_| IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    if !info.brands.is_image
        || info.brands.is_sequence
        || info.is_grid
        || info.has_alpha
        || info.monochrome
    {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }
    let width = u64::from(info.width);
    let height = u64::from(info.height);
    if info.width == 0
        || info.height == 0
        || info.width > MAX_AVIF_DIMENSION_V1
        || info.height > MAX_AVIF_DIMENSION_V1
    {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }
    let pixel_count = width
        .checked_mul(height)
        .ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    if pixel_count > MAX_IMAGE_PIXELS_V1 {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }
    let depth = info.bit_depth.ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    let per_sample = sample_bytes_v1(depth).ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    if !info.bits_per_channel.is_empty()
        && (info.max_bit_depth() != depth || info.num_channels() != 3)
    {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }
    let (sub_x, sub_y) =
        subsampling_v1(info.chroma_subsampling).ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    let (y_columns, y_rows) =
        plane_extent_v1(width, height, 1, 1).ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    let (c_columns, c_rows) =
        plane_extent_v1(width, height, sub_x, sub_y).ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    let layout = [
        (y_columns, y_rows),
        (c_columns, c_rows),
        (c_columns, c_rows),
    ];

    let image = oxideav_avif::decode(source).map_err(|_| IMAGE_WORKER_EXIT_DECODE_FAILED_V1)?;
    if image.width != info.width || image.height != info.height || image.bit_depth != depth {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }
    if image.planes.len() != 3 {
        return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
    }

    let mut planes = Vec::with_capacity(3);
    for (ordinal, (columns, rows)) in layout.iter().enumerate() {
        let plane = &image.planes[ordinal];
        let stride = columns
            .checked_mul(per_sample)
            .ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
        let length = stride
            .checked_mul(*rows)
            .ok_or(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
        if u64::try_from(plane.stride).ok() != Some(stride)
            || u64::try_from(plane.data.len()).ok() != Some(length)
        {
            return Err(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1);
        }
        planes.push(CheckedPlaneV1 {
            ordinal: u32::try_from(ordinal).map_err(|_| IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?,
            stride,
            rows: *rows,
            bytes: plane.data.clone(),
        });
    }

    Ok(CheckedStillV1 {
        width: info.width,
        height: info.height,
        bit_depth: depth,
        planes,
    })
}

/// Renders the closed 16-field V1 envelope. No key is ever added, omitted or reordered.
fn render_envelope_v1(
    source_length: u64,
    source_sha256: MediaWorkerDigestV1,
    still: &CheckedStillV1,
    claims: &[MediaWorkerPlaneClaimV1],
    output_sha256: MediaWorkerDigestV1,
) -> String {
    let planes = claims
        .iter()
        .map(|claim| {
            format!(
                "{{\"ordinal\":{},\"stride\":{},\"rows\":{},\"length\":{},\"sha256\":\"{}\"}}",
                claim.ordinal(),
                claim.stride(),
                claim.rows(),
                claim.length(),
                claim.sha256().to_hex()
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    format!(
        "{{\"version\":1,\"status\":\"decoded\",\"format\":\"avif\",\"mime\":\"image/avif\",\
         \"source_length\":{source_length},\"source_sha256\":\"{}\",\"width\":{},\"height\":{},\
         \"bit_depth\":{},\"color_model\":\"yuv\",\"has_alpha\":false,\"is_grid\":false,\
         \"is_sequence\":false,\"orientation\":\"unverified\",\"planes\":[{planes}],\
         \"output_sha256\":\"{}\"}}",
        source_sha256.to_hex(),
        still.width,
        still.height,
        still.bit_depth,
        output_sha256.to_hex()
    )
}

/// Runs the confined AVIF child: confinement first, then exactly one bounded owned source.
///
/// No untrusted byte is touched until the fixed address/commit and CPU profile has been proved by
/// readback, and nothing reaches stdout unless the complete claim and payload were built in memory
/// and passed every local check.
pub(crate) fn run_avif_worker_child_v1(claimed: MediaWorkerDigestV1) -> u8 {
    let Some(confinement) = crate::media_worker_host::child_image_worker_confinement_v1() else {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    };
    if !confinement.matches_profile_v1() {
        return IMAGE_WORKER_EXIT_UNCONFINED_V1;
    }

    let mut source: Vec<u8> = Vec::new();
    let limit = MAX_IMAGE_SOURCE_BYTES_V1.saturating_add(1);
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
    let source_sha256 = sha256_media_worker_bytes_v1(&source);
    if source_sha256 != claimed {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    let still = match decode_checked_still_v1(&source) {
        Ok(still) => still,
        Err(exit) => return exit,
    };

    let mut plane_bytes: Vec<Vec<u8>> = Vec::with_capacity(still.planes.len());
    let mut claims: Vec<MediaWorkerPlaneClaimV1> = Vec::with_capacity(still.planes.len());
    for plane in &still.planes {
        let length = u64::try_from(plane.bytes.len()).unwrap_or(u64::MAX);
        claims.push(MediaWorkerPlaneClaimV1::new(
            plane.ordinal,
            plane.stride,
            plane.rows,
            length,
            sha256_media_worker_bytes_v1(&plane.bytes),
        ));
        plane_bytes.push(plane.bytes.clone());
    }
    let output_sha256 = media_worker_output_sha256_v1(&claims, &plane_bytes);
    let header = render_envelope_v1(transferred, source_sha256, &still, &claims, output_sha256);
    if header.len() > MAX_AVIF_HEADER_BYTES_V1 {
        return IMAGE_WORKER_EXIT_REFUSED_V1;
    }

    // The whole answer is assembled before the first byte reaches stdout.
    let payload_total: usize = plane_bytes.iter().map(Vec::len).sum();
    let mut frame: Vec<u8> = Vec::with_capacity(header.len() + 1 + payload_total);
    frame.extend_from_slice(header.as_bytes());
    frame.push(b'\n');
    for bytes in &plane_bytes {
        frame.extend_from_slice(bytes);
    }
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    if out.write_all(&frame).is_err() || out.flush().is_err() {
        return IMAGE_WORKER_EXIT_DECODE_FAILED_V1;
    }
    0
}

/// The trusted-parent port: the only production path that can start AVIF decoding.
///
/// The executable is an already-trusted image path (the running `solo-cli`); no authored program,
/// command line, environment value or output destination is ever accepted.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfinedAvifInspectAdapterV1 {
    executable: PathBuf,
    /// Every child process this port actually observed, in launch order. It records a fact the
    /// supervisor reported; it grants nothing and is never consulted by any decision.
    observed_pids: std::cell::RefCell<Vec<u32>>,
}

impl ConfinedAvifInspectAdapterV1 {
    /// Binds the one trusted executable this port may launch.
    #[must_use]
    pub fn new(executable: PathBuf) -> Self {
        Self {
            executable,
            observed_pids: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// The child process identifiers this port really observed, in launch order.
    #[must_use]
    pub fn observed_pids_v1(&self) -> Vec<u32> {
        self.observed_pids.borrow().clone()
    }

    /// Runs the inward whole-candidate boundary over one acquired obligation set.
    #[must_use]
    pub fn inspect_source_set_v1(
        &self,
        set: &GraphicAssetSourceSetV1,
    ) -> MediaWorkerBoundaryOutcomeV1 {
        inspect_media_worker_source_set_v1(set, self)
    }
}

impl MediaWorkerPortV1 for ConfinedAvifInspectAdapterV1 {
    fn inspect_avif(&self, request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1 {
        let run = match run_confined_image_worker_v1(
            &self.executable,
            AVIF_WORKER_TOKEN_V1,
            &request.source_sha256().to_hex(),
            request.source(),
        ) {
            Ok(run) => run,
            // An unavailable executable, a failed confined spawn, an expired deadline, an oversize
            // response, an unproved confinement or an unconfirmed reap are one transport refusal.
            Err(_) => {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerUnavailable);
            }
        };

        let exit = match run.exit_code() {
            Some(0) => MediaWorkerExitV1::Normal,
            _ => MediaWorkerExitV1::Abnormal,
        };
        if run.child_pid() == 0 {
            return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerUnavailable);
        }
        self.observed_pids.borrow_mut().push(run.child_pid());
        let limits_verified = run.limits_verified();
        let cleanup_verified = run.cleanup_verified();
        let wall_deadline_met = run.wall_ms() <= IMAGE_WORKER_WALL_MS_V1;

        let stdout = run.stdout();
        let Some(newline) = stdout
            .iter()
            .take(MAX_AVIF_HEADER_BYTES_V1 + 1)
            .position(|byte| *byte == b'\n')
        else {
            return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerProtocolInvalid);
        };
        if newline > MAX_AVIF_HEADER_BYTES_V1 {
            return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerProtocolInvalid);
        }
        let envelope = match parse_media_worker_envelope_v1(&stdout[..newline]) {
            Ok(envelope) => envelope,
            Err(_) => {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerProtocolInvalid);
            }
        };

        // The received payload must be exactly the claimed planes, in order, with no spare byte.
        let payload = &stdout[newline + 1..];
        let mut total: u64 = 0;
        for claim in envelope.planes() {
            let Some(sum) = total.checked_add(claim.length()) else {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerOutputOversize);
            };
            total = sum;
        }
        if u64::try_from(payload.len()).ok() != Some(total) {
            return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }

        let mut planes: Vec<Vec<u8>> = Vec::with_capacity(envelope.planes().len());
        let mut offset: usize = 0;
        for claim in envelope.planes() {
            let Ok(length) = usize::try_from(claim.length()) else {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
            };
            let Some(end) = offset.checked_add(length) else {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
            };
            if end > payload.len() {
                return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
            }
            planes.push(payload[offset..end].to_vec());
            offset = end;
        }
        if offset != payload.len() {
            return MediaWorkerRunV1::Refused(MediaWorkerRefusalKindV1::WorkerPlaneInvalid);
        }

        MediaWorkerRunV1::Observed(MediaWorkerObservationV1::new(
            limits_verified,
            exit,
            wall_deadline_met,
            cleanup_verified,
            !cleanup_verified,
            MediaWorkerLimitsV1::reference(),
            envelope,
            planes,
        ))
    }
}
