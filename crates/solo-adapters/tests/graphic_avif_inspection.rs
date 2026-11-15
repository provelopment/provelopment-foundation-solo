//! Real AVIF decode probe for the seven immutable fixtures (`SOLO-WO-118 R01` §4).
//!
//! This file is the only permitted new test-only adapter path. It decodes the actual
//! fixture bytes with the pinned pure-Rust decoder and records the structural facts the
//! V4 acceptance matrix needs: dimensions, coded depth, pixel layout, plane count,
//! per-plane stride/length and a content fingerprint. Every assertion here is about
//! genuinely decoded pixels, never about a file name or a container label.

use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../solo-cli/tests/fixtures/avif")
        .join(name)
}

fn fingerprint(bytes: &[u8]) -> u64 {
    // FNV-1a, 64-bit. A diagnostic fingerprint only: it proves the probe distinguishes
    // different decodes, and it is never used as a security or oracle digest.
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Decode one fixture and print every structural fact the V4 matrix consumes.
fn probe(name: &str) -> Option<oxideav_avif::api::AvifImage> {
    let path = fixture(name);
    let bytes = std::fs::read(&path).expect("fixture must be readable");
    match oxideav_avif::decode(&bytes) {
        Ok(image) => {
            let planes: Vec<String> = image
                .planes
                .iter()
                .map(|plane| {
                    format!(
                        "len={} stride={} fnv={:016x}",
                        plane.data.len(),
                        plane.stride,
                        fingerprint(&plane.data)
                    )
                })
                .collect();
            println!(
                "avif_probe fixture={name} ok=true width={} height={} bit_depth={} format={:?} \
                 planes={} range={:?} matrix={} primaries={} transfer={} planes_detail=[{}]",
                image.width,
                image.height,
                image.bit_depth,
                image.format,
                image.planes.len(),
                image.color.range,
                image.color.matrix,
                image.color.primaries,
                image.color.transfer,
                planes.join("; ")
            );
            Some(image)
        }
        Err(error) => {
            println!("avif_probe fixture={name} ok=false error={error}");
            None
        }
    }
}

#[test]
fn avif_probe_reports_real_decoded_facts() {
    // The five positive fixtures must decode to real pixels.
    let white = probe("white_1x1.avif").expect("1x1 8-bit fixture must decode");
    assert_eq!((white.width, white.height), (1, 1));
    assert_eq!(white.bit_depth, 8);

    let eight = probe("fox-profile0.8bpc.yuv420.avif").expect("8-bit fox must decode");
    assert_eq!(eight.bit_depth, 8);
    assert!(eight.planes.len() >= 3);

    let ten = probe("fox-profile0.10bpc.yuv420.avif").expect("10-bit fox must decode");
    assert_eq!(ten.bit_depth, 10);

    let twelve = probe("fox-profile2.12bpc.yuv420.avif").expect("12-bit fox must decode");
    assert_eq!(twelve.bit_depth, 12);

    let alpha = probe("abc_color_irot_alpha_irot.avif").expect("alpha+irot fixture must decode");
    assert!(
        alpha.planes.len() >= 4,
        "the positive alpha fixture must decode an associated alpha plane, got {}",
        alpha.planes.len()
    );

    // The two negative fixtures are recorded for their real decoded or refused facts.
    probe("abc_color_irot_alpha_NOirot.avif");
    probe("clap_irot_imir_non_essential.avif");
}
