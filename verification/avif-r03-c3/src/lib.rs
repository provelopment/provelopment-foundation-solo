//! Standalone AVIF qualification probe — **non-product evidence only**.
//!
//! This crate exists to answer one bounded question: does the exact published
//! `oxideav-avif 0.0.11` / `oxideav-av1 0.1.23` / `oxideav-core 0.1.43` candidate
//! produce real decoded pixels, and can Solo's 20 MiB / 1–8192 / 32,000,000-pixel
//! contract be enforced before and during decoding?
//!
//! Nothing here is product code: it is not in the four-crate workspace, it is not
//! referenced by the product, and it grants no decoder approval.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

/// SHA-256, implemented here so the probe adds no hashing dependency. Correctness
/// is asserted by the NIST/FIPS vectors in the unit tests and, at run time, by
/// comparing every digest with the host tool over the very same bytes.
pub mod sha256 {
    const K: [u32; 64] = [
        0x428a_2f98, 0x7137_4491, 0xb5c0_fbcf, 0xe9b5_dba5, 0x3956_c25b, 0x59f1_11f1, 0x923f_82a4,
        0xab1c_5ed5, 0xd807_aa98, 0x1283_5b01, 0x2431_85be, 0x550c_7dc3, 0x72be_5d74, 0x80de_b1fe,
        0x9bdc_06a7, 0xc19b_f174, 0xe49b_69c1, 0xefbe_4786, 0x0fc1_9dc6, 0x240c_a1cc,
        0x2de9_2c6f, 0x4a74_84aa, 0x5cb0_a9dc, 0x76f9_88da, 0x983e_5152, 0xa831_c66d,
        0xb003_27c8, 0xbf59_7fc7, 0xc6e0_0bf3, 0xd5a7_9147, 0x06ca_6351, 0x1429_2967,
        0x27b7_0a85, 0x2e1b_2138, 0x4d2c_6dfc, 0x5338_0d13, 0x650a_7354, 0x766a_0abb,
        0x81c2_c92e, 0x9272_2c85, 0xa2bf_e8a1, 0xa81a_664b, 0xc24b_8b70, 0xc76c_51a3,
        0xd192_e819, 0xd699_0624, 0xf40e_3585, 0x106a_a070, 0x19a4_c116, 0x1e37_6c08,
        0x2748_774c, 0x34b0_bcb5, 0x391c_0cb3, 0x4ed8_aa4a, 0x5b9c_ca4f, 0x682e_6ff3,
        0x748f_82ee, 0x78a5_636f, 0x84c8_7814, 0x8cc7_0208, 0x90be_fffa, 0xa450_6ceb,
        0xbef9_a3f7, 0xc671_78f2,
    ];

    /// Digest `data` into a 32-byte SHA-256 value.
    pub fn digest(data: &[u8]) -> [u8; 32] {
        let mut h: [u32; 8] = [
            0x6a09_e667, 0xbb67_ae85, 0x3c6e_f372, 0xa54f_f53a, 0x510e_527f, 0x9b05_688c,
            0x1f83_d9ab, 0x5be0_cd19,
        ];
        let bit_len = (data.len() as u64).wrapping_mul(8);
        let mut msg: Vec<u8> = Vec::with_capacity(data.len() + 72);
        msg.extend_from_slice(data);
        msg.push(0x80);
        while msg.len() % 64 != 56 {
            msg.push(0);
        }
        msg.extend_from_slice(&bit_len.to_be_bytes());

        for chunk in msg.chunks_exact(64) {
            let mut w = [0u32; 64];
            for i in 0..16 {
                w[i] = u32::from_be_bytes([
                    chunk[i * 4],
                    chunk[i * 4 + 1],
                    chunk[i * 4 + 2],
                    chunk[i * 4 + 3],
                ]);
            }
            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }
            let mut a = h[0];
            let mut b = h[1];
            let mut c = h[2];
            let mut d = h[3];
            let mut e = h[4];
            let mut f = h[5];
            let mut g = h[6];
            let mut hh = h[7];
            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let t1 = hh
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let t2 = s0.wrapping_add(maj);
                hh = g;
                g = f;
                f = e;
                e = d.wrapping_add(t1);
                d = c;
                c = b;
                b = a;
                a = t1.wrapping_add(t2);
            }
            h[0] = h[0].wrapping_add(a);
            h[1] = h[1].wrapping_add(b);
            h[2] = h[2].wrapping_add(c);
            h[3] = h[3].wrapping_add(d);
            h[4] = h[4].wrapping_add(e);
            h[5] = h[5].wrapping_add(f);
            h[6] = h[6].wrapping_add(g);
            h[7] = h[7].wrapping_add(hh);
        }

        let mut out = [0u8; 32];
        for (i, word) in h.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }

    fn hex_digit(v: u8) -> char {
        match v {
            0..=9 => (b'0' + v) as char,
            _ => (b'a' + (v - 10)) as char,
        }
    }

    /// Lowercase hex of the digest of `data`.
    pub fn hex(data: &[u8]) -> String {
        let d = digest(data);
        let mut s = String::with_capacity(64);
        for b in d.iter() {
            s.push(hex_digit(b >> 4));
            s.push(hex_digit(b & 0x0f));
        }
        s
    }

    #[cfg(test)]
    mod tests {
        use super::hex;

        /// Published NIST/FIPS-180 test vectors.
        #[test]
        fn nist_vectors() {
            assert_eq!(
                hex(b""),
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            );
            assert_eq!(
                hex(b"abc"),
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
            );
            assert_eq!(
                hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
            );
        }

        /// The one-million-'a' vector, exercising multi-block padding.
        #[test]
        fn million_a() {
            let data = vec![b'a'; 1_000_000];
            assert_eq!(
                hex(&data),
                "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
            );
        }
    }
}

/// Mandatory input-byte ceiling from the accepted contract.
pub const MAX_INPUT_BYTES: u64 = 20 * 1024 * 1024;
/// Mandatory per-dimension ceiling from the accepted contract.
pub const MAX_DIMENSION: u64 = 8_192;
/// Mandatory decoded-pixel ceiling from the accepted contract.
pub const MAX_PIXELS: u64 = 32_000_000;
/// Default hard wall-clock budget for a single case, enforced by the parent.
pub const DEFAULT_CASE_TIMEOUT_MS: u64 = 15_000;

/// The twelve published AVIF fixtures named by `SOLO-WO-068 R01`, as relative
/// paths inside the `oxideav-avif-0.0.11` package. Discovery is compared against
/// this list so a silently disappearing fixture fails the run instead of
/// shrinking the proof.
pub const EXPECTED_FIXTURES: [&str; 12] = [
    "tests/fixtures/alpha_video.avif",
    "tests/fixtures/bbb_alpha.avif",
    "tests/fixtures/black420.avif",
    "tests/fixtures/gray32.avif",
    "tests/fixtures/kimono_rotate90.avif",
    "tests/fixtures/midgray.avif",
    "tests/fixtures/monochrome.avif",
    "tests/fixtures/red.avif",
    "tests/fixtures/white16.avif",
    "tests/fixtures/fuzz/y_plane_divergence_match.avif",
    "tests/fixtures/fuzz/y_plane_roundtrip_avif1.avif",
    "tests/fixtures/fuzz/y_plane_roundtrip_avif2.avif",
];

/// Which contract guard a case reached. Reported verbatim so a refusal can never
/// be credited to the wrong layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guard {
    ZeroLength,
    InputBytes,
    Dimension,
    Pixels,
}

impl Guard {
    pub fn name(self) -> &'static str {
        match self {
            Guard::ZeroLength => "contract.zero_length",
            Guard::InputBytes => "contract.max_input_bytes",
            Guard::Dimension => "contract.max_dimension",
            Guard::Pixels => "contract.max_pixels",
        }
    }
}

/// Caller-side input-size guard, applied before any parsing happens.
pub fn contract_input_len(len: usize) -> Result<(), Guard> {
    if len == 0 {
        return Err(Guard::ZeroLength);
    }
    if len as u64 > MAX_INPUT_BYTES {
        return Err(Guard::InputBytes);
    }
    Ok(())
}

/// Caller-side geometry guard: `u64` checked arithmetic, never `u32` multiplication.
pub fn contract_geometry(width: u64, height: u64) -> Result<(), Guard> {
    if width == 0 || height == 0 || width > MAX_DIMENSION || height > MAX_DIMENSION {
        return Err(Guard::Dimension);
    }
    let pixels = width.checked_mul(height).ok_or(Guard::Pixels)?;
    if pixels > MAX_PIXELS {
        return Err(Guard::Pixels);
    }
    Ok(())
}

/// Whether a case must decode, must be refused, or is recorded observationally.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseKind {
    Positive,
    NegativeMustRefuse,
    NegativeObservation,
}

impl CaseKind {
    pub fn expectation(self) -> &'static str {
        match self {
            CaseKind::Positive => "must_decode",
            CaseKind::NegativeMustRefuse => "must_refuse",
            CaseKind::NegativeObservation => "observation_only",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            CaseKind::Positive => "positive",
            _ => "negative",
        }
    }
}

/// One probe case.
#[derive(Debug, Clone)]
pub struct Case {
    pub id: String,
    pub kind: CaseKind,
}

/// The controlled negative matrix. Every non-observation case must be refused;
/// `neg:trailing_garbage` is observational because 0.0.11 exposes no strict mode.
pub const NEGATIVES: [(&str, CaseKind); 13] = [
    ("neg:empty", CaseKind::NegativeMustRefuse),
    ("neg:random_short", CaseKind::NegativeMustRefuse),
    ("neg:ftyp_truncated", CaseKind::NegativeMustRefuse),
    ("neg:header_only", CaseKind::NegativeMustRefuse),
    ("neg:av1c_only", CaseKind::NegativeMustRefuse),
    ("neg:truncated_60pct", CaseKind::NegativeMustRefuse),
    ("neg:truncated_85pct", CaseKind::NegativeMustRefuse),
    ("neg:tile_corrupt", CaseKind::NegativeMustRefuse),
    ("neg:invalid_brand", CaseKind::NegativeMustRefuse),
    ("neg:trailing_garbage", CaseKind::NegativeObservation),
    ("neg:declared_width_8193", CaseKind::NegativeMustRefuse),
    ("neg:declared_pixels_over_32m", CaseKind::NegativeMustRefuse),
    ("neg:input_over_20mib", CaseKind::NegativeMustRefuse),
];

/// The full ordered case list: every discovered fixture, then the fixed negatives.
pub fn build_cases(fixtures: &[String]) -> Vec<Case> {
    let mut cases: Vec<Case> = Vec::with_capacity(fixtures.len() + NEGATIVES.len());
    for f in fixtures {
        cases.push(Case {
            id: format!("pos:{f}"),
            kind: CaseKind::Positive,
        });
    }
    for (name, kind) in NEGATIVES.iter() {
        cases.push(Case {
            id: (*name).to_string(),
            kind: *kind,
        });
    }
    cases
}

/// Locate the unpacked `oxideav-avif-0.0.11` package directory.
pub fn locate_package(explicit: Option<&Path>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        if p.is_dir() {
            return Ok(p.to_path_buf());
        }
        return Err(format!(
            "explicit package dir is not a directory: {}",
            p.display()
        ));
    }
    let mut roots: Vec<PathBuf> = Vec::new();
    if let Some(cargo_home) = std::env::var_os("CARGO_HOME") {
        roots.push(PathBuf::from(cargo_home));
    }
    if let Some(profile) = std::env::var_os("USERPROFILE") {
        roots.push(PathBuf::from(profile).join(".cargo"));
    }
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".cargo"));
    }
    for root in roots {
        let src = root.join("registry").join("src");
        let entries = match std::fs::read_dir(&src) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let candidate = entry.path().join("oxideav-avif-0.0.11");
            if candidate.is_dir() {
                return Ok(candidate);
            }
        }
    }
    Err("oxideav-avif-0.0.11 package directory not found; set OXIDEAV_AVIF_PKG_DIR".to_string())
}

/// Enumerate every `.avif` under the package's `tests/fixtures` tree, as sorted
/// package-relative paths using `/` separators.
pub fn discover_fixtures(pkg: &Path) -> Result<Vec<String>, String> {
    let base = pkg.join("tests").join("fixtures");
    let mut found: Vec<String> = Vec::new();
    walk_fixtures(&base, &base, &mut found)?;
    found.sort();
    Ok(found)
}

fn walk_fixtures(dir: &Path, base: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries =
        std::fs::read_dir(dir).map_err(|e| format!("read_dir({}): {e}", dir.display()))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_fixtures(&path, base, out)?;
        } else {
            let is_avif = path
                .extension()
                .map(|x| x.eq_ignore_ascii_case("avif"))
                .unwrap_or(false);
            if is_avif {
                let rel = path
                    .strip_prefix(base)
                    .map_err(|e| format!("strip_prefix: {e}"))?;
                let rel = rel.to_string_lossy().replace('\\', "/");
                out.push(format!("tests/fixtures/{rel}"));
            }
        }
    }
    Ok(())
}

/// Find the first occurrence of `needle` in `hay`.
pub fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    let last = hay.len() - needle.len();
    for i in 0..=last {
        if &hay[i..i + needle.len()] == needle {
            return Some(i);
        }
    }
    None
}

fn set_u32_be(buf: &mut [u8], off: usize, value: u32) -> Result<(), String> {
    let end = off
        .checked_add(4)
        .ok_or_else(|| "offset overflow".to_string())?;
    if end > buf.len() {
        return Err(format!("patch offset {off} out of range"));
    }
    buf[off..end].copy_from_slice(&value.to_be_bytes());
    Ok(())
}

/// Build one controlled negative input from an authorized fixture's bytes.
pub fn build_negative(name: &str, base: &[u8]) -> Result<Vec<u8>, String> {
    let mdat =
        find_sub(base, b"mdat").ok_or_else(|| "base fixture has no mdat box".to_string())?;
    match name {
        "neg:empty" => Ok(Vec::new()),
        "neg:random_short" => {
            let mut v = Vec::with_capacity(32);
            for i in 0..32usize {
                v.push(((i * 37 + 11) % 251) as u8);
            }
            Ok(v)
        }
        "neg:ftyp_truncated" => Ok(base[..base.len().min(12)].to_vec()),
        "neg:header_only" => Ok(base[..(mdat + 4).min(base.len())].to_vec()),
        "neg:av1c_only" => Ok(base[..mdat.min(base.len())].to_vec()),
        "neg:truncated_60pct" => {
            let cut = ((base.len() * 60) / 100).max(1).min(base.len());
            Ok(base[..cut].to_vec())
        }
        "neg:truncated_85pct" => {
            let cut = ((base.len() * 85) / 100).max(1).min(base.len());
            Ok(base[..cut].to_vec())
        }
        "neg:tile_corrupt" => {
            let mut v = base.to_vec();
            let start = mdat + 8;
            let end = (start + 16).min(v.len());
            if start < end {
                for b in v[start..end].iter_mut() {
                    *b ^= 0xff;
                }
            }
            Ok(v)
        }
        "neg:invalid_brand" => {
            let mut v = base.to_vec();
            if v.len() < 20 {
                return Err("base fixture too short for a brand patch".to_string());
            }
            // Overwrite the major brand *and every* compatible brand so the file's
            // brand set is genuinely unclassifiable, not merely unusual.
            v[8..12].copy_from_slice(b"xxxx");
            let box_len = u32::from_be_bytes([v[0], v[1], v[2], v[3]]) as usize;
            let end = box_len.min(v.len());
            let mut off = 16usize;
            while off + 4 <= end {
                v[off..off + 4].copy_from_slice(b"zzzz");
                off += 4;
            }
            Ok(v)
        }
        "neg:trailing_garbage" => {
            let mut v = base.to_vec();
            v.extend_from_slice(&[0xa5u8; 64]);
            Ok(v)
        }
        "neg:declared_width_8193" => {
            let mut v = base.to_vec();
            let ispe =
                find_sub(&v, b"ispe").ok_or_else(|| "base fixture has no ispe".to_string())?;
            set_u32_be(&mut v, ispe + 8, 8_193)?;
            Ok(v)
        }
        "neg:declared_pixels_over_32m" => {
            let mut v = base.to_vec();
            let ispe =
                find_sub(&v, b"ispe").ok_or_else(|| "base fixture has no ispe".to_string())?;
            set_u32_be(&mut v, ispe + 8, 8_192)?;
            set_u32_be(&mut v, ispe + 12, 8_192)?;
            Ok(v)
        }
        "neg:input_over_20mib" => {
            let mut v: Vec<u8> = Vec::with_capacity(MAX_INPUT_BYTES as usize + 1);
            v.extend_from_slice(base);
            v.resize(MAX_INPUT_BYTES as usize + 1, 0u8);
            Ok(v)
        }
        other => Err(format!("unknown negative case: {other}")),
    }
}

/// One decoded plane exactly as the decoder returned it.
#[derive(Debug, Clone)]
pub struct Plane {
    pub stride: u64,
    pub data: Vec<u8>,
}

/// A decoded image with its own pixel bytes.
#[derive(Debug, Clone)]
pub struct Decoded {
    pub width: u64,
    pub height: u64,
    pub bit_depth: u64,
    pub is_grid: bool,
    pub has_alpha: bool,
    pub planes: Vec<Plane>,
}

impl Decoded {
    pub fn pixel_bytes(&self) -> u64 {
        self.planes.iter().map(|p| p.data.len() as u64).sum()
    }
}

/// Deterministic serialization of the decoded planes: it is both the fingerprint
/// input and the raw output file, so the host tool hashes the very same bytes.
pub fn serialize_planes(planes: &[Plane]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for (i, p) in planes.iter().enumerate() {
        out.extend_from_slice(&(i as u32).to_le_bytes());
        out.extend_from_slice(&p.stride.to_le_bytes());
        out.extend_from_slice(&(p.data.len() as u64).to_le_bytes());
        out.extend_from_slice(&p.data);
    }
    out
}

/// Outcome of one decode attempt.
pub enum Outcome {
    Decoded(Decoded),
    RefusedByContract(Guard),
    RefusedByDecoder(String),
    Panicked(String),
}

/// Decode one input with the exact pinned candidate, applying the caller-side
/// contract before the decoder is entered whenever the container can be inspected.
pub fn decode(bytes: &[u8]) -> Outcome {
    if let Err(g) = contract_input_len(bytes.len()) {
        return Outcome::RefusedByContract(g);
    }

    // Pre-decode geometry gate: a successful container inspection is what proves
    // this guard is reachable *before* any pixel allocation happens.
    let mut inspect_error = String::new();
    match oxideav_avif::inspect::inspect(bytes) {
        Ok(info) => {
            if let Err(g) = contract_geometry(u64::from(info.width), u64::from(info.height)) {
                return Outcome::RefusedByContract(g);
            }
        }
        Err(e) => inspect_error = format!("inspect: {e}"),
    }

    // A decoder panic must never be reported as a clean refusal.
    std::panic::set_hook(Box::new(|_| {}));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        use oxideav_avif::decoder::AvifDecoder;
        use oxideav_core::{CodecId, Decoder};

        let mut dec = AvifDecoder::new(CodecId::new("avif"));
        let info = dec
            .decode_file(bytes)
            .map_err(|e| format!("decode_file: {e}"))?;
        let frame = dec
            .receive_frame()
            .map_err(|e| format!("receive_frame: {e}"))?;
        let vf = match frame {
            oxideav_core::Frame::Video(v) => v,
            _ => return Err("decoder returned a non-video frame".to_string()),
        };
        let mut planes: Vec<Plane> = Vec::new();
        for p in vf.image_planes().iter() {
            planes.push(Plane {
                stride: p.stride as u64,
                data: p.data.clone(),
            });
        }
        let bit_depth = info
            .bits_per_channel
            .first()
            .copied()
            .map(u64::from)
            .unwrap_or(0);
        Ok::<Decoded, String>(Decoded {
            width: u64::from(info.width),
            height: u64::from(info.height),
            bit_depth,
            is_grid: info.is_grid,
            has_alpha: info.has_alpha,
            planes,
        })
    }));
    let _ = std::panic::take_hook();

    match result {
        Ok(Ok(d)) => {
            if let Err(g) = contract_geometry(d.width, d.height) {
                return Outcome::RefusedByContract(g);
            }
            Outcome::Decoded(d)
        }
        Ok(Err(e)) => {
            let mut msg = format!("decode: {e}");
            if !inspect_error.is_empty() {
                msg.push_str(" | ");
                msg.push_str(&inspect_error);
            }
            Outcome::RefusedByDecoder(msg)
        }
        Err(payload) => Outcome::Panicked(panic_message(payload)),
    }
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

/// Independent expectation for a fixture, derived from AV1/YCbCr colour semantics
/// (not from this decoder's own output): a solid-colour fixture must decode to
/// constant luma and correspondingly neutral or directional chroma.
pub fn oracle_name(rel: &str) -> &'static str {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    match file {
        "black420.avif" => "uniform_black_neutral_chroma",
        "white16.avif" => "uniform_white_high_luma",
        "red.avif" => "red_high_cr_low_cb",
        "midgray.avif" => "uniform_gray_neutral_chroma",
        "gray32.avif" => "uniform_gray_neutral_chroma",
        _ => "none",
    }
}

fn min_max(data: &[u8]) -> (u8, u8) {
    let mut lo = 255u8;
    let mut hi = 0u8;
    for b in data.iter() {
        if *b < lo {
            lo = *b;
        }
        if *b > hi {
            hi = *b;
        }
    }
    (lo, hi)
}

fn mean(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let sum: u64 = data.iter().map(|b| u64::from(*b)).sum();
    sum as f64 / data.len() as f64
}

/// Apply the independent oracle. Returns `(verdict, detail)` where verdict is
/// `PASS`, `FAIL`, `SKIPPED` (no independent expectation, or >8-bit) or `NONE`.
pub fn apply_oracle(name: &str, d: &Decoded) -> (&'static str, String) {
    if name == "none" {
        return ("NONE", "no independent expectation".to_string());
    }
    if d.bit_depth > 8 {
        return (
            "SKIPPED",
            format!("oracle calibrated for 8-bit; decoded depth {}", d.bit_depth),
        );
    }
    if d.planes.is_empty() {
        return ("FAIL", "no planes decoded".to_string());
    }
    let luma = min_max(&d.planes[0].data);
    let luma_span = luma.1.saturating_sub(luma.0);
    match name {
        "uniform_black_neutral_chroma" => {
            if luma.1 > 24 {
                return (
                    "FAIL",
                    format!("black luma max {} exceeds 24", luma.1),
                );
            }
            if d.planes.len() >= 3 {
                let cb = min_max(&d.planes[1].data);
                let cr = min_max(&d.planes[2].data);
                if cb.0 < 118 || cb.1 > 138 || cr.0 < 118 || cr.1 > 138 {
                    return (
                        "FAIL",
                        format!(
                            "chroma not neutral: cb {}..{} cr {}..{}",
                            cb.0, cb.1, cr.0, cr.1
                        ),
                    );
                }
                return ("PASS", "luma<=24 and both chroma planes neutral".to_string());
            }
            ("PASS", "luma<=24 (single plane)".to_string())
        }
        "uniform_white_high_luma" => {
            if luma.0 < 200 {
                return ("FAIL", format!("white luma min {} below 200", luma.0));
            }
            ("PASS", format!("luma>={} (min) and constant span {luma_span}", luma.0))
        }
        "uniform_gray_neutral_chroma" => {
            if luma_span > 1 {
                return (
                    "FAIL",
                    format!("gray luma not constant: span {luma_span} ({}..{})", luma.0, luma.1),
                );
            }
            if d.planes.len() >= 3 {
                let cb = min_max(&d.planes[1].data);
                let cr = min_max(&d.planes[2].data);
                if cb.0 < 118 || cb.1 > 138 || cr.0 < 118 || cr.1 > 138 {
                    return (
                        "FAIL",
                        format!(
                            "gray chroma not neutral: cb {}..{} cr {}..{}",
                            cb.0, cb.1, cr.0, cr.1
                        ),
                    );
                }
            }
            ("PASS", format!("constant luma span {luma_span} with neutral chroma"))
        }
        "red_high_cr_low_cb" => {
            if d.planes.len() < 3 {
                return ("FAIL", "expected 3 planes for a chroma oracle".to_string());
            }
            let cb = mean(&d.planes[1].data);
            let cr = mean(&d.planes[2].data);
            if cr <= cb + 40.0 {
                return (
                    "FAIL",
                    format!("red chroma direction wrong: mean cb {cb:.1} cr {cr:.1}"),
                );
            }
            ("PASS", format!("mean cb {cb:.1} << mean cr {cr:.1}"))
        }
        _ => ("NONE", "unrecognised oracle".to_string()),
    }
}

/// One case's full evidence record.
#[derive(Debug, Clone)]
pub struct CaseRecord {
    pub case_id: String,
    pub kind: String,
    pub expectation: String,
    pub input_sha256: String,
    pub input_len: u64,
    pub result: String,
    pub guard: String,
    pub error: String,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub pixels: Option<u64>,
    pub bit_depth: Option<u64>,
    pub is_grid: Option<bool>,
    pub has_alpha: Option<bool>,
    pub plane_strides: Vec<u64>,
    pub plane_lengths: Vec<u64>,
    pub pixel_bytes: Option<u64>,
    pub decoded_sha256: String,
    pub oracle: String,
    pub oracle_verdict: String,
    pub oracle_detail: String,
    pub status: String,
    pub note: String,
    pub elapsed_ms: u64,
    pub peak_rss_bytes: Option<u64>,
    pub timed_out: bool,
    pub raw_output_file: String,
}

/// Replace anything path-like or unsafe for a file name.
pub fn sanitize_id(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out
}

/// Geometry facts about a decoded frame. `orientation` records whether the plane
/// buffers match the reported dimensions or are transposed relative to them, which
/// is a real property of transform-bearing files rather than a harness error.
pub struct GeometryReport {
    pub issues: Vec<String>,
    pub orientation: String,
}

fn geometry_report(d: &Decoded) -> GeometryReport {
    let mut issues: Vec<String> = Vec::new();
    let mut orientation = String::from("matches_reported_dimensions");
    if d.planes.is_empty() {
        issues.push("no planes decoded".to_string());
    }
    // 1 plane = monochrome, 3 = Y/Cb/Cr, 4 = Y/Cb/Cr/alpha.
    if !matches!(d.planes.len(), 1 | 3 | 4) {
        issues.push(format!("unexpected plane count {}", d.planes.len()));
    }
    let bps: u64 = if d.bit_depth > 8 { 2 } else { 1 };

    // Reference geometry for the chroma/alpha checks, taken from the luma plane.
    let mut ref_w = d.width;
    let mut ref_h = d.height;
    if let Some(p0) = d.planes.first() {
        if p0.stride == 0 {
            issues.push("luma stride is zero".to_string());
        } else {
            let rows = p0.data.len() as u64 / p0.stride;
            let normal = p0.stride >= d.width.saturating_mul(bps) && rows >= d.height;
            let transposed = p0.stride >= d.height.saturating_mul(bps) && rows >= d.width;
            if normal {
                // as reported
            } else if transposed {
                orientation = String::from("transposed_relative_to_reported_dimensions");
                ref_w = d.height;
                ref_h = d.width;
            } else {
                issues.push(format!(
                    "luma stride {x} x rows {rows} inconsistent with reported {w}x{h}",
                    x = p0.stride,
                    w = d.width,
                    h = d.height
                ));
            }
        }
    }

    for (i, p) in d.planes.iter().enumerate().skip(1) {
        if p.stride == 0 {
            issues.push(format!("plane {i} stride is zero"));
            continue;
        }
        let rows = p.data.len() as u64 / p.stride;
        let full_resolution = p.stride >= ref_w.saturating_mul(bps) && rows >= ref_h;
        let subsampled = p.stride >= ref_w.div_ceil(2).saturating_mul(bps)
            && rows >= ref_h.div_ceil(2);
        if !full_resolution && !subsampled {
            issues.push(format!(
                "plane {i} geometry inconsistent (stride {}, rows {rows}, reference {ref_w}x{ref_h})",
                p.stride
            ));
        }
    }

    GeometryReport { issues, orientation }
}

/// Evaluate one case end to end and return its evidence record.
pub fn evaluate_case(case: &Case, bytes: &[u8], raw_dir: Option<&Path>) -> CaseRecord {
    let started = std::time::Instant::now();
    let mut rec = CaseRecord {
        case_id: case.id.clone(),
        kind: case.kind.label().to_string(),
        expectation: case.kind.expectation().to_string(),
        input_sha256: sha256::hex(bytes),
        input_len: bytes.len() as u64,
        result: String::new(),
        guard: String::new(),
        error: String::new(),
        width: None,
        height: None,
        pixels: None,
        bit_depth: None,
        is_grid: None,
        has_alpha: None,
        plane_strides: Vec::new(),
        plane_lengths: Vec::new(),
        pixel_bytes: None,
        decoded_sha256: String::new(),
        oracle: String::new(),
        oracle_verdict: String::new(),
        oracle_detail: String::new(),
        status: String::new(),
        note: String::new(),
        elapsed_ms: 0,
        peak_rss_bytes: None,
        timed_out: false,
        raw_output_file: String::new(),
    };

    match decode(bytes) {
        Outcome::Decoded(d) => {
            let geometry = geometry_report(&d);
            rec.result = "DECODED".to_string();
            rec.width = Some(d.width);
            rec.height = Some(d.height);
            rec.pixels = d.width.checked_mul(d.height);
            rec.bit_depth = Some(d.bit_depth);
            rec.is_grid = Some(d.is_grid);
            rec.has_alpha = Some(d.has_alpha);
            rec.plane_strides = d.planes.iter().map(|p| p.stride).collect();
            rec.plane_lengths = d.planes.iter().map(|p| p.data.len() as u64).collect();
            rec.pixel_bytes = Some(d.pixel_bytes());
            let serialized = serialize_planes(&d.planes);
            rec.decoded_sha256 = sha256::hex(&serialized);
            if let Some(dir) = raw_dir {
                let file = format!("{}.planes.bin", sanitize_id(&case.id));
                let path = dir.join(&file);
                if std::fs::write(&path, &serialized).is_ok() {
                    rec.raw_output_file = file;
                }
            }
            if case.kind == CaseKind::Positive {
                let oracle = oracle_name(&case.id);
                let (verdict, detail) = apply_oracle(oracle, &d);
                rec.oracle = oracle.to_string();
                rec.oracle_verdict = verdict.to_string();
                rec.oracle_detail = detail;
                if !geometry.issues.is_empty() {
                    rec.status = "FAIL".to_string();
                    rec.note = format!(
                        "geometry_orientation={}; plane geometry: {}",
                        geometry.orientation,
                        geometry.issues.join("; ")
                    );
                } else if verdict == "FAIL" {
                    rec.status = "FAIL".to_string();
                    rec.note = "independent oracle rejected the decoded pixels".to_string();
                } else {
                    rec.status = "PASS".to_string();
                    rec.note = format!("geometry_orientation={}", geometry.orientation);
                }
            } else {
                rec.result = "ACCEPTED".to_string();
                rec.oracle = "n/a".to_string();
                rec.oracle_verdict = "NONE".to_string();
                if case.kind == CaseKind::NegativeObservation {
                    rec.status = "UNVERIFIED".to_string();
                    rec.note = "accepted; 0.0.11 exposes no strict-mode/full-consumption switch"
                        .to_string();
                } else {
                    rec.status = "FAIL".to_string();
                    rec.note = "input that must be refused was accepted".to_string();
                }
            }
        }
        Outcome::RefusedByContract(g) => {
            rec.result = "REFUSED_BY_HARNESS_CONTRACT".to_string();
            rec.guard = g.name().to_string();
            if case.kind == CaseKind::Positive {
                rec.status = "FAIL".to_string();
                rec.note = "valid fixture was refused by our own contract guard".to_string();
            } else {
                rec.status = "PASS".to_string();
            }
        }
        Outcome::RefusedByDecoder(e) => {
            rec.result = "REFUSED_BY_DECODER".to_string();
            rec.guard = "decoder".to_string();
            rec.error = e;
            rec.status = if case.kind == CaseKind::Positive {
                "FAIL".to_string()
            } else {
                "PASS".to_string()
            };
        }
        Outcome::Panicked(p) => {
            rec.result = "PANIC".to_string();
            rec.error = p;
            rec.status = "FAIL".to_string();
            rec.note = "decoder panicked; a panic is never a clean refusal".to_string();
        }
    }

    rec.elapsed_ms = started.elapsed().as_millis() as u64;
    rec
}

impl CaseRecord {
    /// One-line machine-readable JSON record (no absolute paths, no raw bytes).
    pub fn to_json_line(&self) -> String {
        format!(
            concat!(
                "{{\"case_id\":\"{}\",\"kind\":\"{}\",\"expectation\":\"{}\",",
                "\"input_sha256\":\"{}\",\"input_len\":{},\"result\":\"{}\",\"guard\":\"{}\",",
                "\"error\":\"{}\",\"width\":{},\"height\":{},\"pixels\":{},\"bit_depth\":{},",
                "\"is_grid\":{},\"has_alpha\":{},\"plane_strides\":{},\"plane_lengths\":{},",
                "\"pixel_bytes\":{},\"decoded_sha256\":\"{}\",\"oracle\":\"{}\",",
                "\"oracle_verdict\":\"{}\",\"oracle_detail\":\"{}\",\"status\":\"{}\",",
                "\"note\":\"{}\",\"elapsed_ms\":{},\"peak_rss_bytes\":{},\"timed_out\":{},",
                "\"raw_output_file\":\"{}\"}}"
            ),
            json_escape(&self.case_id),
            json_escape(&self.kind),
            json_escape(&self.expectation),
            self.input_sha256,
            self.input_len,
            json_escape(&self.result),
            json_escape(&self.guard),
            json_escape(&self.error),
            opt_u64(self.width),
            opt_u64(self.height),
            opt_u64(self.pixels),
            opt_u64(self.bit_depth),
            opt_bool(self.is_grid),
            opt_bool(self.has_alpha),
            u64_list(&self.plane_strides),
            u64_list(&self.plane_lengths),
            opt_u64(self.pixel_bytes),
            self.decoded_sha256,
            json_escape(&self.oracle),
            json_escape(&self.oracle_verdict),
            json_escape(&self.oracle_detail),
            json_escape(&self.status),
            json_escape(&self.note),
            self.elapsed_ms,
            opt_u64(self.peak_rss_bytes),
            self.timed_out,
            json_escape(&self.raw_output_file),
        )
    }

    /// Short human-readable line for the CI log.
    pub fn summary_line(&self) -> String {
        format!(
            "{} status={} result={} guard={} {}x{} depth={} pixel_bytes={} sha={} oracle={}/{} ms={} peak_rss={}",
            self.case_id,
            self.status,
            self.result,
            if self.guard.is_empty() { "-" } else { &self.guard },
            opt_u64(self.width),
            opt_u64(self.height),
            opt_u64(self.bit_depth),
            opt_u64(self.pixel_bytes),
            if self.decoded_sha256.is_empty() { "-" } else { &self.decoded_sha256 },
            if self.oracle.is_empty() { "-" } else { &self.oracle },
            if self.oracle_verdict.is_empty() { "-" } else { &self.oracle_verdict },
            self.elapsed_ms,
            opt_u64(self.peak_rss_bytes),
        )
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn opt_u64(v: Option<u64>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

fn opt_bool(v: Option<bool>) -> String {
    match v {
        Some(b) => b.to_string(),
        None => "null".to_string(),
    }
}

fn u64_list(v: &[u64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_boundaries() {
        assert_eq!(contract_input_len(0), Err(Guard::ZeroLength));
        assert_eq!(contract_input_len(1), Ok(()));
        assert_eq!(contract_input_len(MAX_INPUT_BYTES as usize), Ok(()));
        assert_eq!(
            contract_input_len(MAX_INPUT_BYTES as usize + 1),
            Err(Guard::InputBytes)
        );
        assert_eq!(contract_geometry(1, 1), Ok(()));
        assert_eq!(contract_geometry(8_192, 8_192), Err(Guard::Pixels));
        assert_eq!(contract_geometry(8_193, 1), Err(Guard::Dimension));
        assert_eq!(contract_geometry(1, 8_193), Err(Guard::Dimension));
        assert_eq!(contract_geometry(0, 1), Err(Guard::Dimension));
        assert_eq!(contract_geometry(8_192, 3_906), Ok(()));
        assert_eq!(contract_geometry(8_192, 3_907), Err(Guard::Pixels));
        assert_eq!(contract_geometry(u64::MAX, 2), Err(Guard::Dimension));
    }

    fn synthetic_container() -> Vec<u8> {
        let mut base: Vec<u8> = Vec::new();
        base.extend_from_slice(&[0, 0, 0, 24]);
        base.extend_from_slice(b"ftypavif");
        base.extend_from_slice(&[0, 0, 0, 0]);
        base.extend_from_slice(&[0, 0, 0, 20]);
        base.extend_from_slice(b"ispe");
        base.extend_from_slice(&[0, 0, 0, 0]);
        base.extend_from_slice(&[0, 0, 0, 2, 0, 0, 0, 2]);
        base.extend_from_slice(&[0, 0, 0, 8]);
        base.extend_from_slice(b"mdat");
        base.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        base
    }

    #[test]
    fn negative_builders_patch_the_intended_field() {
        let base = synthetic_container();

        let w = build_negative("neg:declared_width_8193", &base).expect("width patch");
        let ispe = find_sub(&w, b"ispe").expect("ispe present");
        assert_eq!(&w[ispe + 8..ispe + 12], &8_193u32.to_be_bytes());

        let p = build_negative("neg:declared_pixels_over_32m", &base).expect("pixel patch");
        let ispe = find_sub(&p, b"ispe").expect("ispe present");
        assert_eq!(&p[ispe + 8..ispe + 12], &8_192u32.to_be_bytes());
        assert_eq!(&p[ispe + 12..ispe + 16], &8_192u32.to_be_bytes());

        let t = build_negative("neg:trailing_garbage", &base).expect("trailing");
        assert_eq!(t.len(), base.len() + 64);

        let big = build_negative("neg:input_over_20mib", &base).expect("oversize");
        assert_eq!(big.len() as u64, MAX_INPUT_BYTES + 1);

        assert!(build_negative("neg:empty", &base).expect("empty").is_empty());
        assert!(build_negative("neg:unknown_case", &base).is_err());
    }

    fn solid(planes: Vec<Plane>) -> Decoded {
        Decoded {
            width: 2,
            height: 2,
            bit_depth: 8,
            is_grid: false,
            has_alpha: false,
            planes,
        }
    }

    #[test]
    fn oracle_accepts_solid_colours_and_rejects_mismatches() {
        let black = solid(vec![
            Plane { stride: 2, data: vec![16, 16, 16, 16] },
            Plane { stride: 1, data: vec![128, 128] },
            Plane { stride: 1, data: vec![128, 128] },
        ]);
        assert_eq!(apply_oracle("uniform_black_neutral_chroma", &black).0, "PASS");

        let white = solid(vec![Plane { stride: 2, data: vec![235, 235, 235, 235] }]);
        assert_eq!(apply_oracle("uniform_white_high_luma", &white).0, "PASS");

        let gray = solid(vec![
            Plane { stride: 2, data: vec![128, 128, 128, 128] },
            Plane { stride: 1, data: vec![128, 128] },
            Plane { stride: 1, data: vec![128, 128] },
        ]);
        assert_eq!(apply_oracle("uniform_gray_neutral_chroma", &gray).0, "PASS");

        let red = solid(vec![
            Plane { stride: 2, data: vec![80, 80, 80, 80] },
            Plane { stride: 1, data: vec![90, 90] },
            Plane { stride: 1, data: vec![240, 240] },
        ]);
        assert_eq!(apply_oracle("red_high_cr_low_cb", &red).0, "PASS");

        let not_black = solid(vec![Plane { stride: 2, data: vec![200, 200, 200, 200] }]);
        assert_eq!(
            apply_oracle("uniform_black_neutral_chroma", &not_black).0,
            "FAIL"
        );
        assert_eq!(apply_oracle("none", &not_black).0, "NONE");
    }

    #[test]
    fn plane_serialization_is_deterministic() {
        let planes = vec![
            Plane { stride: 2, data: vec![1, 2, 3, 4] },
            Plane { stride: 1, data: vec![9, 9] },
        ];
        let a = serialize_planes(&planes);
        let b = serialize_planes(&planes);
        assert_eq!(a, b);
        assert_eq!(sha256::hex(&a), sha256::hex(&b));
        assert_eq!(a.len(), 20 + 4 + 20 + 2);
        assert_eq!(
            sanitize_id("pos:tests/fixtures/red.avif"),
            "pos_tests_fixtures_red_avif"
        );
    }

    #[test]
    fn expected_fixture_list_is_exactly_twelve() {
        assert_eq!(EXPECTED_FIXTURES.len(), 12);
        for name in EXPECTED_FIXTURES.iter() {
            assert!(name.ends_with(".avif"));
        }
    }
}
