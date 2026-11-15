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

/// The two pinned associated-alpha sources whose decoder results are adjudicated.
const ALPHA_POSITIVE: &str = "abc_color_irot_alpha_irot.avif";
const ALPHA_NEGATIVE: &str = "abc_color_irot_alpha_NOirot.avif";
const ALPHA_PINNED_DISPLAY_SHA: &str =
    "950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744";
const ALPHA_COORDINATE_LIMIT: usize = 64;

fn read_source(name: &str) -> Vec<u8> {
    std::fs::read(fixture(name)).expect("pinned AVIF fixture bytes must be readable")
}

fn plane_hex(image: &oxideav_avif::AvifImage) -> Vec<String> {
    image
        .planes
        .iter()
        .map(|plane| sha256_media_worker_bytes_v1(&plane.data).to_hex())
        .collect()
}

#[test]
fn n123_associated_alpha_irot_decoder_differential_is_measured() {
    let manifest = goldens();
    let positive_bytes = read_source(ALPHA_POSITIVE);
    let negative_bytes = read_source(ALPHA_NEGATIVE);

    // Input identity first: both sources are the exact pinned fixtures, checked against the
    // immutable manifest rather than trusted by name.
    assert_eq!(
        sha256_media_worker_bytes_v1(&positive_bytes).to_hex(),
        manifest["fixtures"][ALPHA_POSITIVE]["source_sha256"]
            .as_str()
            .expect("pinned positive source digest"),
        "the valid associated-alpha source must be the pinned fixture"
    );
    assert_eq!(
        sha256_media_worker_bytes_v1(&negative_bytes).to_hex(),
        manifest["negatives"][ALPHA_NEGATIVE]["source_sha256"]
            .as_str()
            .expect("pinned negative source digest"),
        "the missing-association source must be the pinned invalid fixture"
    );

    // Payload identity, checked rather than assumed: the two fixture files differ in total length
    // by one byte, so only the extracted item payloads can settle whether the coded inputs are
    // the same picture. A difference here would be an alternative explanation for any pixel delta.
    let positive_header = oxideav_avif::parse_header(&positive_bytes).expect("positive header");
    let negative_header = oxideav_avif::parse_header(&negative_bytes).expect("negative header");
    let positive_primary = positive_header
        .meta
        .primary_item_id
        .expect("positive primary item id");
    let negative_primary = negative_header
        .meta
        .primary_item_id
        .expect("negative primary item id");
    let positive_alpha = oxideav_avif::find_alpha_item_id(&positive_header.meta, positive_primary)
        .expect("positive alpha auxiliary");
    let negative_alpha = oxideav_avif::find_alpha_item_id(&negative_header.meta, negative_primary)
        .expect("negative alpha auxiliary");
    let colour_payload_equal = positive_header
        .item_data(positive_primary)
        .expect("positive colour payload")
        == negative_header
            .item_data(negative_primary)
            .expect("negative colour payload");
    let alpha_payload_equal = positive_header
        .item_data(positive_alpha)
        .expect("positive alpha payload")
        == negative_header
            .item_data(negative_alpha)
            .expect("negative alpha payload");
    println!(
        "n123_evidence payloads colour_equal={colour_payload_equal} \
         alpha_equal={alpha_payload_equal} primary_ids={positive_primary}/{negative_primary} \
         alpha_ids={positive_alpha}/{negative_alpha}"
    );

    let positive =
        oxideav_avif::decode(&positive_bytes).expect("positive must decode for diagnosis");
    let negative =
        oxideav_avif::decode(&negative_bytes).expect("negative must decode for diagnosis");

    // Structural correctness of both decoded pictures: the codec applies the primary transform
    // itself, so the returned geometry is the display geometry with four full-size 4:4:4 planes.
    for (label, image) in [("positive", &positive), ("negative", &negative)] {
        assert_eq!(
            (image.width, image.height),
            (256, 512),
            "{label} display extent"
        );
        assert_eq!(image.bit_depth, 8, "{label} depth");
        assert_eq!(
            format!("{:?}", image.format),
            "Yuva444P",
            "{label} must be four-plane 8-bit 4:4:4 with alpha"
        );
        assert_eq!(image.planes.len(), 4, "{label} plane count");
        for plane in &image.planes {
            assert_eq!(plane.stride, 256, "{label} canonical alpha-aware stride");
            assert_eq!(plane.data.len(), 131_072, "{label} canonical plane length");
        }
    }

    let positive_rgba = positive.try_to_rgba8().expect("fallible RGBA conversion");
    let negative_rgba = negative.try_to_rgba8().expect("fallible RGBA conversion");
    assert_eq!(positive_rgba.len(), 524_288, "512x256x4 packed RGBA");
    assert_eq!(negative_rgba.len(), 524_288, "512x256x4 packed RGBA");

    let positive_nonopaque = positive_rgba[3..]
        .iter()
        .step_by(4)
        .filter(|alpha| **alpha != 255)
        .count();
    let negative_nonopaque = negative_rgba[3..]
        .iter()
        .step_by(4)
        .filter(|alpha| **alpha != 255)
        .count();

    // Pixel-by-pixel differential: alpha and RGB are counted separately, and every differing
    // alpha sample is enumerated (bounded) with its display coordinates and both values.
    let mut alpha_differences = 0usize;
    let mut rgb_differences = 0usize;
    let mut coordinates: Vec<(usize, usize, u8, u8)> = Vec::new();
    for pixel in 0..(256 * 512) {
        let base = pixel * 4;
        if positive_rgba[base..base + 3] != negative_rgba[base..base + 3] {
            rgb_differences += 1;
        }
        if positive_rgba[base + 3] != negative_rgba[base + 3] {
            alpha_differences += 1;
            if coordinates.len() < ALPHA_COORDINATE_LIMIT {
                // The packed buffer is the 256x512 display frame: x is the display column.
                coordinates.push((
                    pixel % 256,
                    pixel / 256,
                    positive_rgba[base + 3],
                    negative_rgba[base + 3],
                ));
            }
        }
    }

    let positive_rgba_sha = sha256_media_worker_bytes_v1(&positive_rgba).to_hex();
    let negative_rgba_sha = sha256_media_worker_bytes_v1(&negative_rgba).to_hex();
    let pinned_display = digest(ALPHA_PINNED_DISPLAY_SHA).to_hex();
    let positive_alpha_min = *positive_rgba[3..]
        .iter()
        .step_by(4)
        .min()
        .expect("a 256x512 alpha plane has samples");
    let positive_alpha_max = *positive_rgba[3..]
        .iter()
        .step_by(4)
        .max()
        .expect("a 256x512 alpha plane has samples");

    println!(
        "n123_evidence positive_planes={:?} negative_planes={:?}",
        plane_hex(&positive),
        plane_hex(&negative)
    );
    println!(
        "n123_evidence positive_rgba={positive_rgba_sha} positive_alpha_min={positive_alpha_min} \
         positive_alpha_max={positive_alpha_max} positive_alpha_nonopaque={positive_nonopaque} \
         negative_rgba={negative_rgba_sha} negative_alpha_nonopaque={negative_nonopaque} \
         pinned_display={pinned_display}"
    );

    println!(
        "n123_evidence alpha_differences={alpha_differences} rgb_differences={rgb_differences} \
         coordinates={}",
        if coordinates.is_empty() {
            "none".to_string()
        } else {
            coordinates
                .iter()
                .map(|(x, y, pos, neg)| format!("{x},{y}:{pos}>{neg}"))
                .collect::<Vec<String>>()
                .join(";")
        }
    );

    // Measured relationship to the immutable R03 positive pin. A matching raw decode of the
    // invalid fixture never promotes its metadata association to a valid acceptance.
    let positive_matches_pin = positive_rgba_sha == pinned_display;
    let negative_matches_pin = negative_rgba_sha == pinned_display;
    println!(
        "n123_evidence positive_matches={positive_matches_pin} \
         negative_matches={negative_matches_pin} same_between_sources={}",
        positive_rgba == negative_rgba
    );

    // The independent R03 reference is the authority: its pinned digest and alpha statistics are
    // read from the immutable manifest and must equal the compile-time digest used above.
    let pinned_spec = &manifest["fixtures"][ALPHA_POSITIVE];
    assert_eq!(
        pinned_spec["rgba_display_sha256"]
            .as_str()
            .expect("pinned display digest"),
        ALPHA_PINNED_DISPLAY_SHA,
        "the compile-time pin must be the immutable manifest digest"
    );
    let pinned_alpha_min = u8::try_from(pinned_spec["alpha_min"].as_u64().expect("pinned min"))
        .expect("alpha min fits a byte");
    let pinned_alpha_max = u8::try_from(pinned_spec["alpha_max"].as_u64().expect("pinned max"))
        .expect("alpha max fits a byte");
    let pinned_nonopaque = pinned_spec["alpha_non_opaque_samples"]
        .as_u64()
        .expect("pinned alpha count");

    // The decoder's own colour conversion agrees with the independent tool, so a correct
    // composite must reproduce the reference digest byte for byte on both sources.
    assert_eq!(
        positive_rgba_sha, pinned_display,
        "the valid associated-alpha source must decode to the immutable reference display RGBA"
    );
    assert_eq!(
        negative_rgba_sha, pinned_display,
        "the alpha-untransformed source must decode to the same reference display RGBA"
    );
    assert_eq!(
        positive_rgba, negative_rgba,
        "a correctly associated alpha must not change any composite sample"
    );
    assert_eq!(
        positive_alpha_min, pinned_alpha_min,
        "decoded alpha minimum must equal the reference minimum"
    );
    assert_eq!(
        positive_alpha_max, pinned_alpha_max,
        "decoded alpha maximum must equal the reference maximum"
    );
    assert_eq!(
        u64::try_from(positive_nonopaque).expect("count fits"),
        pinned_nonopaque,
        "the valid source's non-opaque alpha population must equal the reference count"
    );
    assert_eq!(
        u64::try_from(negative_nonopaque).expect("count fits"),
        pinned_nonopaque,
        "the untransformed decode must reproduce the same reference alpha population"
    );

    // Sample-level differential of the transferred alpha plane itself, so the transport-level
    // plane difference and the displayed RGBA alpha difference are never conflated.
    let plane_alpha_differences = positive.planes[3]
        .data
        .iter()
        .zip(negative.planes[3].data.iter())
        .filter(|(left, right)| left != right)
        .count();
    let rgba_alpha_equals_plane = positive.planes[3]
        .data
        .iter()
        .enumerate()
        .all(|(index, value)| positive_rgba[index * 4 + 3] == *value)
        && negative.planes[3]
            .data
            .iter()
            .enumerate()
            .all(|(index, value)| negative_rgba[index * 4 + 3] == *value);
    println!(
        "n123_evidence plane_alpha_differences={plane_alpha_differences} \
         rgba_alpha_equals_plane={rgba_alpha_equals_plane} \
         nonopaque_delta={}",
        positive_nonopaque.abs_diff(negative_nonopaque)
    );

    // Accurate enumeration: the bounded inventory and the counts must describe the same data.
    assert_eq!(
        coordinates.len(),
        alpha_differences.min(ALPHA_COORDINATE_LIMIT),
        "the bounded coordinate inventory must match the enumerated difference count"
    );
    assert!(
        positive_nonopaque.abs_diff(negative_nonopaque) <= alpha_differences,
        "differing alpha samples cannot change more non-opaque samples than they touch"
    );
    assert!(
        rgba_alpha_equals_plane,
        "the packed RGBA alpha channel must be the decoded alpha plane, sample for sample"
    );
    assert_eq!(
        alpha_differences, plane_alpha_differences,
        "the displayed alpha differential must equal the transferred plane differential"
    );
    assert_eq!(
        rgb_differences, 0,
        "only alpha may differ between the two sources; {rgb_differences} RGB samples differed"
    );
    assert_eq!(
        alpha_differences, 0,
        "a repaired decoder must show zero differing alpha samples, not {alpha_differences}"
    );
    assert_eq!(
        plane_alpha_differences, 0,
        "the transferred alpha planes must be identical, not {plane_alpha_differences} samples apart"
    );
    assert!(
        positive_matches_pin && negative_matches_pin,
        "both sources must reproduce the immutable reference digest"
    );
    println!("n123_decoder repair_verified=true");
}
