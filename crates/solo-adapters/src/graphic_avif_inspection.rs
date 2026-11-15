//! The first real confined AVIF coded-plane worker and its trusted-parent port
//! (`SOLO-WO-118 R05` §3, Architecture §6A.8).
//!
//! **What exists here.** One safe adapter module with exactly two halves:
//!
//! * [`run_avif_worker_child_v1`] runs *inside* the already confined `solo-cli` child. It proves
//!   its own confinement profile first, reads one bounded owned source from stdin, checks the
//!   caller's digest, preflights the container with the pinned decoder's typed inspection, decodes
//!   and emits one canonical V1 envelope plus the exact concatenated planes — all built in memory
//!   before a single byte is written.
//! * [`ConfinedAvifInspectAdapterV1`] runs in the trusted parent. It launches only the trusted
//!   executable through the existing single-use supervisor, parses exactly one envelope with the
//!   unchanged strict parser, splits the received bytes into exactly the claimed planes and reports
//!   only honestly observed host facts.
//!
//! **Two admitted classes.** The long-standing coded-plane class is `color_model=yuv`,
//! `has_alpha=false`, `orientation=unverified` at 8/10/12 bits in 4:4:4 or 4:2:0. `SOLO-WO-118 R10`
//! adds exactly one more: an 8-bit YUVA 4:4:4 still whose primary and alpha items *both* associate
//! one essential `irot` quarter turn, reported as `color_model=yuva`, `has_alpha=true`,
//! `orientation=identity` with four full-resolution planes in displayed orientation. The class is
//! chosen by parsed item properties, never by a filename, digest, role, OS or environment value.
//!
//! **What this is not.** It is not grid, sequence, clean-aperture, mirror, premultiplied-alpha,
//! 4:2:2/4:2:0-alpha, 10/12-bit-alpha, resized-alpha, publication or asset-library authority. The
//! resulting record is nonpublishing evidence; `MediaWorkerCheckedEnvelopeV1` still proves no media
//! correctness and grants no publication authority. The accepted V3 `avif-unsupported-v3` refusal is
//! untouched: no V3 path reaches this module.

//!
//! **Refusals are whole.** Every failure returns one fixed exit status and writes nothing, so a
//! refusal can never look like a partial or display-ready success.

use std::io::{Read, Write};
use std::path::PathBuf;

use solo_application::{
    GraphicAssetSourceSetV1, IMAGE_WORKER_WALL_MS_V1, MAX_IMAGE_PIXELS_V1,
    MAX_IMAGE_SOURCE_BYTES_V1, MediaWorkerBoundaryOutcomeV1, MediaWorkerColorModelV1,
    MediaWorkerDigestV1, MediaWorkerExitV1, MediaWorkerLimitsV1, MediaWorkerObservationV1,
    MediaWorkerOrientationV1, MediaWorkerPlaneClaimV1, MediaWorkerPortV1, MediaWorkerRefusalKindV1,
    MediaWorkerRequestV1, MediaWorkerRunV1, inspect_media_worker_source_set_v1,
    media_worker_output_sha256_v1, sha256_media_worker_bytes_v1,
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
    color_model: MediaWorkerColorModelV1,
    has_alpha: bool,
    orientation: MediaWorkerOrientationV1,
    planes: Vec<CheckedPlaneV1>,
}

impl CheckedStillV1 {
    /// The coded-plane class: three or more Y/U/V planes, no alpha, no orientation claim.
    fn coded_yuv_v1(width: u32, height: u32, bit_depth: u8, planes: Vec<CheckedPlaneV1>) -> Self {
        Self {
            width,
            height,
            bit_depth,
            color_model: MediaWorkerColorModelV1::Yuv,
            has_alpha: false,
            orientation: MediaWorkerOrientationV1::Unverified,
            planes,
        }
    }
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

/// The one admitted associated-alpha class (`SOLO-WO-118 R10` §3.1).
///
/// Decided entirely from parsed item properties — never from a file name, digest, role or host —
/// and *before* any pixel buffer exists:
///
/// * a single still `av01` primary with exactly one `auxl`-linked `av01` auxiliary whose `auxC`
///   names the alpha URN, and no second auxiliary of any kind;
/// * on **both** items exactly one associated `irot{angle: 1}` marked essential, no `imir`, no
///   `clap`, and no other property at all outside the recognised descriptive/coding set;
/// * matching `ispe` extents on both items, `pixi` `[8, 8, 8]` on the primary and `[8]` on the
///   auxiliary, one `av1C` each, an alpha `auxC` only on the auxiliary;
/// * 8-bit YUV 4:4:4 with a full-resolution straight (never premultiplied) alpha plane.
///
/// Anything else — including a source the decoder can happily render — is refused here.
fn associated_alpha_policy_v1(
    info: &oxideav_avif::AvifInfo,
    source: &[u8],
) -> Result<(u32, u32), u8> {
    const REJECT: u8 = IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    if !info.brands.is_image
        || info.brands.is_sequence
        || info.is_grid
        || info.monochrome
        || info.bit_depth != Some(8)
        || info.chroma_subsampling != Some((false, false))
        || info.bits_per_channel != [8, 8, 8]
    {
        return Err(REJECT);
    }
    let header = oxideav_avif::parse_header(source).map_err(|_| REJECT)?;
    let meta = &header.meta;
    let primary_id = meta.primary_item_id.ok_or(REJECT)?;
    let primary = meta.item_by_id(primary_id).ok_or(REJECT)?;
    if primary.item_type != oxideav_avif::parser::ITEM_TYPE_AV01 {
        return Err(REJECT);
    }
    // Exactly one auxiliary, and it must be the alpha the crate itself identifies.
    let alpha_id = oxideav_avif::find_alpha_item_id(meta, primary_id).ok_or(REJECT)?;
    if alpha_id == primary_id {
        return Err(REJECT);
    }
    let alpha = meta.item_by_id(alpha_id).ok_or(REJECT)?;
    if alpha.item_type != oxideav_avif::parser::ITEM_TYPE_AV01 {
        return Err(REJECT);
    }
    let mut auxl_from_alpha = 0usize;
    for iref in &meta.irefs {
        if iref.reference_type != *b"auxl" {
            continue;
        }
        // The primary may not itself point at anything, and the auxiliary must point at exactly
        // the primary. Any other `auxl` source is a second, ambiguous auxiliary.
        if iref.from_id == primary_id {
            return Err(REJECT);
        }
        if iref.from_id != alpha_id {
            return Err(REJECT);
        }
        auxl_from_alpha += 1;
        if iref.to_ids.as_slice() != [primary_id] {
            return Err(REJECT);
        }
    }
    if auxl_from_alpha != 1 {
        return Err(REJECT);
    }

    let primary_extent = associated_item_extent_v1(meta, primary_id, false)?;
    let alpha_extent = associated_item_extent_v1(meta, alpha_id, true)?;
    if primary_extent != alpha_extent {
        return Err(REJECT);
    }
    // The primary's `ispe` is the coded extent; `inspect` reports it unchanged. The displayed
    // extent is the coded extent swapped by the single quarter turn, and both must be in bounds.
    if (info.width, info.height) != primary_extent {
        return Err(REJECT);
    }
    let (displayed_width, displayed_height) = (primary_extent.1, primary_extent.0);
    if displayed_width == 0
        || displayed_height == 0
        || displayed_width > MAX_AVIF_DIMENSION_V1
        || displayed_height > MAX_AVIF_DIMENSION_V1
    {
        return Err(REJECT);
    }
    let pixels = u64::from(displayed_width)
        .checked_mul(u64::from(displayed_height))
        .ok_or(REJECT)?;
    if pixels > MAX_IMAGE_PIXELS_V1 {
        return Err(REJECT);
    }
    Ok((displayed_width, displayed_height))
}

/// The `ispe` extent of one item in the admitted class, after checking its whole association list.
///
/// `auxiliary` selects the auxiliary's narrower allowlist (`auxC` alpha required, one 8-bit
/// channel) and forbids `auxC` on the primary. Every other property — `imir`, `clap`, `iscl`,
/// `prem`, an unknown or essential-unknown extension, a second `ispe`/`pixi`/`irot` — is refused.
fn associated_item_extent_v1(
    meta: &oxideav_avif::Meta,
    item_id: u32,
    auxiliary: bool,
) -> Result<(u32, u32), u8> {
    const REJECT: u8 = IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    let association = meta.assoc_by_id(item_id).ok_or(REJECT)?;
    let mut extent: Option<(u32, u32)> = None;
    let mut bits: Option<Vec<u8>> = None;
    let mut av1c = 0usize;
    let mut auxc = 0usize;
    let mut irot = 0usize;
    for entry in &association.entries {
        let Some(property) = meta.properties.get(entry.index as usize) else {
            return Err(REJECT);
        };
        match property {
            oxideav_avif::Property::Av1C(_) => av1c += 1,
            oxideav_avif::Property::Ispe(ispe) => {
                if extent.replace((ispe.width, ispe.height)).is_some() {
                    return Err(REJECT);
                }
            }
            oxideav_avif::Property::Pixi(pixi) => {
                if bits.replace(pixi.bits_per_channel.clone()).is_some() {
                    return Err(REJECT);
                }
            }
            oxideav_avif::Property::Colr(_) => {}
            oxideav_avif::Property::AuxC(aux) => {
                if !auxiliary || aux.aux_type != oxideav_avif::AUX_URN_ALPHA_MPEG {
                    return Err(REJECT);
                }
                auxc += 1;
            }
            oxideav_avif::Property::Irot(value) => {
                if !entry.essential || value.angle != 1 || irot > 0 {
                    return Err(REJECT);
                }
                irot += 1;
            }
            _ => return Err(REJECT),
        }
    }
    if av1c != 1 || irot != 1 || auxc != usize::from(auxiliary) {
        return Err(REJECT);
    }
    let expected_bits: &[u8] = if auxiliary { &[8] } else { &[8, 8, 8] };
    if bits.as_deref() != Some(expected_bits) {
        return Err(REJECT);
    }
    extent.ok_or(REJECT)
}

/// Decodes the admitted associated-alpha still and checks the four real displayed planes.
fn decode_checked_alpha_still_v1(
    source: &[u8],
    info: &oxideav_avif::AvifInfo,
) -> Result<CheckedStillV1, u8> {
    const REJECT: u8 = IMAGE_WORKER_EXIT_INVALID_MEDIA_V1;
    let (displayed_width, displayed_height) = associated_alpha_policy_v1(info, source)?;
    let image = oxideav_avif::decode(source).map_err(|_| IMAGE_WORKER_EXIT_DECODE_FAILED_V1)?;
    if image.format != oxideav_avif::PixelFormat::Yuva444P
        || image.bit_depth != 8
        || image.width != displayed_width
        || image.height != displayed_height
        || image.planes.len() != 4
    {
        return Err(REJECT);
    }
    let stride = u64::from(displayed_width);
    let rows = u64::from(displayed_height);
    let length = stride.checked_mul(rows).ok_or(REJECT)?;
    let mut planes = Vec::with_capacity(4);
    let mut total: u64 = 0;
    for (ordinal, plane) in image.planes.iter().enumerate() {
        if u64::try_from(plane.stride).ok() != Some(stride)
            || u64::try_from(plane.data.len()).ok() != Some(length)
        {
            return Err(REJECT);
        }
        total = total.checked_add(length).ok_or(REJECT)?;
        planes.push(CheckedPlaneV1 {
            ordinal: u32::try_from(ordinal).map_err(|_| REJECT)?,
            stride,
            rows,
            bytes: plane.data.clone(),
        });
    }
    // The decoded output ceiling is checked against the real buffers, not a claim.
    if total > MAX_IMAGE_PIXELS_V1.saturating_mul(4) {
        return Err(REJECT);
    }
    Ok(CheckedStillV1 {
        width: displayed_width,
        height: displayed_height,
        bit_depth: 8,
        color_model: MediaWorkerColorModelV1::Yuva,
        has_alpha: true,
        orientation: MediaWorkerOrientationV1::Identity,
        planes,
    })
}

/// Preflights one owned source with the pinned decoder, then decodes and checks the real planes.
///
/// The preflight refuses every out-of-scope class **before** any pixel buffer is allocated: a
/// sequence brand, a grid primary, an unadmitted alpha composition, monochrome, an unsupported
/// depth, an unadmitted sub-sampling and any out-of-bounds geometry. Classification comes from the
/// actual
/// container's typed inspection, never from a file name, digest or fixed pixel table.
fn decode_checked_still_v1(source: &[u8]) -> Result<CheckedStillV1, u8> {
    let info =
        oxideav_avif::inspect::inspect(source).map_err(|_| IMAGE_WORKER_EXIT_INVALID_MEDIA_V1)?;
    // The class is chosen by the parsed container alone: an attached alpha auxiliary selects the
    // one narrow associated-alpha branch, and everything else keeps the original coded-plane
    // behaviour unchanged.
    if info.has_alpha {
        return decode_checked_alpha_still_v1(source, &info);
    }
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

    Ok(CheckedStillV1::coded_yuv_v1(
        info.width,
        info.height,
        depth,
        planes,
    ))
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
         \"bit_depth\":{},\"color_model\":\"{}\",\"has_alpha\":{},\"is_grid\":false,\
         \"is_sequence\":false,\"orientation\":\"{}\",\"planes\":[{planes}],\
         \"output_sha256\":\"{}\"}}",
        source_sha256.to_hex(),
        still.width,
        still.height,
        still.bit_depth,
        still.color_model.as_str(),
        still.has_alpha,
        still.orientation.as_str(),
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

#[cfg(test)]
mod alpha_class_tests {
    //! Local, host-independent proof of the child's *production* admission policy against the
    //! immutable R03 fixtures. These run the real preflight and the real decoder; they never
    //! emulate a confined child, a host, or a parent.

    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../solo-cli/tests/fixtures/avif")
            .join(name);
        std::fs::read(&path).expect("the pinned R03 fixture must be readable")
    }

    /// The rendered envelope for one admitted source, exactly as the child would build it.
    fn envelope_v1(source: &[u8]) -> String {
        let still = decode_checked_still_v1(source).expect("the admitted class decodes");
        let mut plane_bytes: Vec<Vec<u8>> = Vec::with_capacity(still.planes.len());
        let mut claims: Vec<MediaWorkerPlaneClaimV1> = Vec::with_capacity(still.planes.len());
        for plane in &still.planes {
            let length = u64::try_from(plane.bytes.len()).expect("length fits");
            claims.push(MediaWorkerPlaneClaimV1::new(
                plane.ordinal,
                plane.stride,
                plane.rows,
                length,
                sha256_media_worker_bytes_v1(&plane.bytes),
            ));
            plane_bytes.push(plane.bytes.clone());
        }
        let output = media_worker_output_sha256_v1(&claims, &plane_bytes);
        let header = render_envelope_v1(
            u64::try_from(source.len()).expect("length fits"),
            sha256_media_worker_bytes_v1(source),
            &still,
            &claims,
            output,
        );
        assert!(
            header.len() <= MAX_AVIF_HEADER_BYTES_V1,
            "the four-plane header must fit the unchanged 1024-byte frame"
        );
        assert!(
            parse_media_worker_envelope_v1(header.as_bytes()).is_ok(),
            "the four-plane header must satisfy the unchanged strict V1 parser"
        );
        header
    }

    #[test]
    fn the_valid_associated_alpha_irot_still_is_admitted_with_four_display_planes() {
        let source = fixture("abc_color_irot_alpha_irot.avif");
        let still = decode_checked_still_v1(&source).expect("the valid fixture is admitted");
        assert_eq!(
            (still.width, still.height, still.bit_depth),
            (256, 512, 8),
            "a quarter turn swaps the coded 512x256 into a displayed 256x512"
        );
        assert_eq!(still.color_model, MediaWorkerColorModelV1::Yuva);
        assert!(still.has_alpha);
        assert_eq!(still.orientation, MediaWorkerOrientationV1::Identity);
        assert_eq!(still.planes.len(), 4, "Y, U, V and A");
        for (ordinal, plane) in still.planes.iter().enumerate() {
            assert_eq!(plane.ordinal, u32::try_from(ordinal).expect("ordinal"));
            assert_eq!(plane.stride, 256, "full-resolution stride");
            assert_eq!(plane.rows, 512, "full-resolution rows");
            assert_eq!(plane.bytes.len(), 131_072, "256x512 bytes per plane");
        }
        let header = envelope_v1(&source);
        assert!(
            header.contains("\"color_model\":\"yuva\",\"has_alpha\":true"),
            "the admitted class is claimed as YUVA: {header}"
        );
        assert!(
            header.contains("\"orientation\":\"identity\""),
            "the admitted class claims displayed orientation: {header}"
        );
    }

    #[test]
    fn both_defective_transform_fixtures_are_refused_before_pixels() {
        for name in [
            "abc_color_irot_alpha_NOirot.avif",
            "clap_irot_imir_non_essential.avif",
        ] {
            let source = fixture(name);
            // The decoder itself can render both of them; the policy, not the codec, is the gate.
            assert!(
                oxideav_avif::decode(&source).is_ok(),
                "{name}: the pinned decoder still renders this source"
            );
            assert_eq!(
                decode_checked_still_v1(&source).err(),
                Some(IMAGE_WORKER_EXIT_INVALID_MEDIA_V1),
                "{name}: defective item properties must refuse as invalid media"
            );
        }
    }

    #[test]
    fn a_non_alpha_still_keeps_the_unchanged_coded_plane_class() {
        let source = fixture("white_1x1.avif");
        let still = decode_checked_still_v1(&source).expect("the non-alpha class still decodes");
        assert_eq!(still.color_model, MediaWorkerColorModelV1::Yuv);
        assert!(!still.has_alpha);
        assert_eq!(still.orientation, MediaWorkerOrientationV1::Unverified);
        assert_eq!(still.planes.len(), 3);
        let header = envelope_v1(&source);
        assert!(
            header.contains("\"color_model\":\"yuv\",\"has_alpha\":false"),
            "the coded-plane class tokens are unchanged: {header}"
        );
        assert!(header.contains("\"orientation\":\"unverified\""));
    }
}
