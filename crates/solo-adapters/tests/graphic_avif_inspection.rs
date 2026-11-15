//! Real AVIF still assertions against the independently pinned R03 goldens
//! (`SOLO-WO-118 R04` §3.1; the ledger path that replaces the former feasibility probe).
//!
//! Four unrotated, non-alpha positives are decoded from immutable fixture bytes by the pinned
//! pure-Rust decoder, and every canonical Y/U/V plane is hashed and compared with the
//! **independent** `avifdec`/`dav1d` facts pinned by `SOLO-WO-118 R03`. Layout is derived from the
//! coded chroma token and checked twice — against the manifest and against the decoder's own
//! stride and length — so wrongly packed planes cannot pass by echoing the manifest.
//!
//! Proves: the accepted decoder reproduces independently measured pixel bytes for these four
//! stills. Does not prove: the confined worker, parent verification, orientation, alpha, grids,
//! sequences, publication authority or any V4 closure claim.

use std::path::PathBuf;

use serde_json::Value;
use solo_application::{MediaWorkerDigestV1, sha256_media_worker_bytes_v1};

/// The four positives this intermediate slice accepts, as `(fixture, coded chroma, depth)`.
const POSITIVES: [(&str, &str, u64); 4] = [
    ("white_1x1.avif", "444", 8),
    ("fox-profile0.8bpc.yuv420.avif", "420jpeg", 8),
    ("fox-profile0.10bpc.yuv420.avif", "420p10", 10),
    ("fox-profile2.12bpc.yuv420.avif", "420p12", 12),
];

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../solo-cli/tests/fixtures/avif")
        .join(name)
}

fn goldens() -> Value {
    // A fixed repo-relative test-fixture path, never a caller-supplied or external one, and it
    // is only ever read: the accepted R03 manifest is input to this assertion, never an output.
    let path = fixture("reference-goldens.json");
    let text = std::fs::read_to_string(&path).expect("the pinned oracle manifest must be readable");
    serde_json::from_str(&text).expect("the pinned oracle manifest must parse")
}

fn digest(hex: &str) -> MediaWorkerDigestV1 {
    MediaWorkerDigestV1::parse_lower_hex(hex).expect("pinned digest is canonical lowercase hex")
}

fn subsampling(chroma: &str) -> (u64, u64) {
    match chroma {
        "444" => (1, 1),
        "420" | "420jpeg" | "420p10" | "420p12" => (2, 2),
        other => panic!("unexpected coded chroma token {other}"),
    }
}

fn sample_bytes(depth: u64) -> u64 {
    match depth {
        8 => 1,
        10 | 12 => 2,
        other => panic!("unexpected coded depth {other}"),
    }
}

/// The documented oracle aggregate framing, rebuilt from the decoded planes.
fn aggregate(planes: &[(u64, &str, u64, u64, &[u8])]) -> MediaWorkerDigestV1 {
    let mut framed: Vec<u8> = Vec::new();
    for (ordinal, name, stride, rows, data) in planes {
        let header = format!("PLANE|{ordinal}|{name}|{stride}|{rows}|{}\n", data.len());
        framed.extend_from_slice(header.as_bytes());
        framed.extend_from_slice(data);
    }
    sha256_media_worker_bytes_v1(&framed)
}

#[test]
fn n120_oracle_four_positives_match_the_independent_r03_goldens() {
    let manifest = goldens();
    assert_eq!(manifest["schema"], "solo-avif-oracle-v1");
    assert_eq!(manifest["state"], "pinned");

    for (name, chroma, depth) in POSITIVES {
        let bytes = std::fs::read(fixture(name)).expect("fixture bytes must be readable");
        let pinned = &manifest["fixtures"][name];
        assert_eq!(pinned["kind"], "y4m", "{name} must be a raw-plane golden");
        assert_eq!(
            digest(pinned["source_sha256"].as_str().expect("src hex")),
            sha256_media_worker_bytes_v1(&bytes),
            "{name} source bytes must equal the pinned source digest"
        );

        let image = oxideav_avif::decode(&bytes).expect("accepted decoder must decode the still");
        let width = u64::from(image.width);
        let height = u64::from(image.height);
        assert_eq!(u64::from(image.bit_depth), depth, "{name} coded depth");
        assert_eq!(
            width,
            pinned["width"].as_u64().expect("width"),
            "{name} width"
        );
        assert_eq!(
            height,
            pinned["height"].as_u64().expect("height"),
            "{name} height"
        );
        assert_eq!(
            pinned["depth"].as_u64().expect("depth"),
            depth,
            "{name} depth"
        );
        assert_eq!(
            pinned["chroma"].as_str().expect("chroma"),
            chroma,
            "{name} chroma"
        );

        let (sub_x, sub_y) = subsampling(chroma);
        let per_sample = sample_bytes(depth);
        let layout = [
            ("Y", width, height),
            ("U", width.div_ceil(sub_x), height.div_ceil(sub_y)),
            ("V", width.div_ceil(sub_x), height.div_ceil(sub_y)),
        ];

        assert_eq!(image.planes.len(), 3, "{name} must be a three-plane still");
        assert_eq!(
            image.planes.len(),
            pinned["planes"].as_array().expect("planes").len(),
            "{name} pinned plane count"
        );

        let mut claims: Vec<(u64, &str, u64, u64, &[u8])> = Vec::new();
        for (ordinal, (plane_name, columns, rows)) in layout.iter().enumerate() {
            let plane = &image.planes[ordinal];
            let ordinal_u64 = ordinal as u64;
            let stride = columns * per_sample;
            let length = stride * rows;
            let actual_stride = u64::try_from(plane.stride).expect("stride fits u64");
            let actual_length = u64::try_from(plane.data.len()).expect("length fits u64");

            let pinned_plane = &pinned["planes"][ordinal];
            assert_eq!(pinned_plane["ordinal"].as_u64(), Some(ordinal_u64));
            assert_eq!(pinned_plane["name"].as_str(), Some(*plane_name));
            assert_eq!(pinned_plane["columns"].as_u64(), Some(*columns));
            assert_eq!(pinned_plane["rows"].as_u64(), Some(*rows));
            assert_eq!(pinned_plane["stride"].as_u64(), Some(stride));
            assert_eq!(pinned_plane["length"].as_u64(), Some(length));

            assert_eq!(actual_stride, stride, "{name} {plane_name} decoded stride");
            assert_eq!(actual_length, length, "{name} {plane_name} decoded length");
            assert_eq!(
                digest(pinned_plane["sha256"].as_str().expect("plane hex")),
                sha256_media_worker_bytes_v1(&plane.data),
                "{name} {plane_name} pixels must equal the independent golden"
            );
            claims.push((
                ordinal_u64,
                plane_name,
                stride,
                *rows,
                plane.data.as_slice(),
            ));
        }

        assert_eq!(
            digest(pinned["aggregate_sha256"].as_str().expect("aggregate hex")),
            aggregate(&claims),
            "{name} aggregate must equal the independent golden"
        );

        println!(
            "n120_oracle case=goldens fixture={name} width={width} height={height} \
             depth={depth} chroma={chroma} planes=3 aggregate_matches=true"
        );
    }
}

#[test]
fn n120_oracle_unsupported_fixtures_are_never_accepted_as_the_four_positives() {
    for name in [
        "abc_color_irot_alpha_irot.avif",
        "abc_color_irot_alpha_NOirot.avif",
        "clap_irot_imir_non_essential.avif",
    ] {
        assert!(
            !POSITIVES.iter().any(|(positive, _, _)| *positive == name),
            "{name} must not be among the accepted positives"
        );
        let bytes = std::fs::read(fixture(name)).expect("fixture bytes must be readable");
        match oxideav_avif::decode(&bytes) {
            Ok(image) => {
                let planes = image.planes.len();
                println!(
                    "n120_oracle case=unsupported fixture={name} decodes=true planes={planes}"
                );
                assert!(
                    planes != 3 || u64::from(image.bit_depth) != 8,
                    "{name} must not silently look like an accepted plain still"
                );
            }
            Err(error) => {
                println!("n120_oracle case=unsupported fixture={name} decodes=false error={error}");
            }
        }
    }
}
