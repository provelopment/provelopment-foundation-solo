//! `R03.C3` adapter tests: the strict worker-envelope protocol parser.
//!
//! Every case supplies bytes directly — no file, no child process, no network — and asserts the
//! exact stable refusal category, never a bare `is_err()`. The accepted baseline is one complete
//! version 1 object whose digests are the real SHA-256 values of the plane bytes it describes, so
//! a mutated expectation is caught rather than merely shape-checked.
//!
//! Nothing here is media evidence: a parsed envelope is an untrusted claim, and these tests prove
//! strict parsing only.

use solo_adapters::{MediaWorkerProtocolRefusalKindV1, parse_media_worker_envelope_v1};
use solo_application::{
    MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1, MediaWorkerColorModelV1, MediaWorkerFormatV1,
    MediaWorkerMimeV1, MediaWorkerOrientationV1, MediaWorkerStatusV1, sha256_media_worker_bytes_v1,
};

/// The three deterministic planes the accepted baseline claims: 1024 + 256 + 256 bytes.
fn planes() -> Vec<Vec<u8>> {
    vec![
        vec![0x11u8; 32 * 32],
        vec![0x22u8; 16 * 16],
        vec![0x33u8; 16 * 16],
    ]
}

/// The documented V1 aggregate encoding, hashed directly so the fixture is independent of the
/// application helper it is meant to exercise.
fn aggregate_hex(planes: &[Vec<u8>]) -> String {
    let dimensions = [(0u32, 32u64, 32u64), (1, 16, 16), (2, 16, 16)];
    let mut encoded: Vec<u8> = Vec::new();
    for (index, (ordinal, stride, rows)) in dimensions.iter().enumerate() {
        encoded.extend_from_slice(&ordinal.to_le_bytes());
        encoded.extend_from_slice(&stride.to_le_bytes());
        encoded.extend_from_slice(&rows.to_le_bytes());
        encoded.extend_from_slice(&(planes[index].len() as u64).to_le_bytes());
        encoded.extend_from_slice(&planes[index]);
    }
    sha256_media_worker_bytes_v1(&encoded).to_hex()
}

/// The source body the claim describes.
fn source() -> Vec<u8> {
    let mut body = vec![0x21u8; 299];
    body[0] = 0x00;
    body[4] = b'f';
    body[8] = b'a';
    body
}

/// The accepted baseline: one complete, internally consistent version 1 envelope.
fn valid_json() -> String {
    let planes = planes();
    let p0 = sha256_media_worker_bytes_v1(&planes[0]).to_hex();
    let p1 = sha256_media_worker_bytes_v1(&planes[1]).to_hex();
    let p2 = sha256_media_worker_bytes_v1(&planes[2]).to_hex();
    let output = aggregate_hex(&planes);
    let source_sha = sha256_media_worker_bytes_v1(&source()).to_hex();
    format!(
        r#"{{"version":1,"status":"decoded","format":"avif","mime":"image/avif","source_length":299,"source_sha256":"{source_sha}","width":32,"height":32,"bit_depth":8,"color_model":"yuv","has_alpha":false,"is_grid":false,"is_sequence":false,"orientation":"identity","planes":[{{"ordinal":0,"stride":32,"rows":32,"length":1024,"sha256":"{p0}"}},{{"ordinal":1,"stride":16,"rows":16,"length":256,"sha256":"{p1}"}},{{"ordinal":2,"stride":16,"rows":16,"length":256,"sha256":"{p2}"}}],"output_sha256":"{output}"}}"#
    )
}

/// The stable category a refused input produces, or a panic when it was accepted.
fn refusal_kind(bytes: &[u8]) -> MediaWorkerProtocolRefusalKindV1 {
    match parse_media_worker_envelope_v1(bytes) {
        Ok(_) => panic!("expected a refusal, got an accepted envelope"),
        Err(refusal) => refusal.kind(),
    }
}

/// The stable token a mutated baseline must produce.
fn refused_as(json: &str, expected: MediaWorkerProtocolRefusalKindV1) {
    assert_eq!(
        refusal_kind(json.as_bytes()),
        expected,
        "expected {} for {json}",
        expected.code()
    );
}

#[test]
fn parses_one_complete_envelope_exactly() {
    let planes = planes();
    let envelope = parse_media_worker_envelope_v1(valid_json().as_bytes())
        .expect("the baseline envelope parses");

    assert_eq!(envelope.protocol_version(), 1);
    assert_eq!(envelope.status(), MediaWorkerStatusV1::Decoded);
    assert_eq!(envelope.format(), MediaWorkerFormatV1::Avif);
    assert_eq!(envelope.mime(), MediaWorkerMimeV1::ImageAvif);
    assert_eq!(envelope.source_length(), 299);
    assert_eq!(
        envelope.source_sha256(),
        sha256_media_worker_bytes_v1(&source())
    );
    assert_eq!(envelope.width(), 32);
    assert_eq!(envelope.height(), 32);
    assert_eq!(envelope.bit_depth(), 8);
    assert_eq!(envelope.color_model(), MediaWorkerColorModelV1::Yuv);
    assert!(!envelope.has_alpha());
    assert!(!envelope.is_grid());
    assert!(!envelope.is_sequence());
    assert_eq!(envelope.orientation(), MediaWorkerOrientationV1::Identity);
    assert_eq!(envelope.planes().len(), 3);
    assert_eq!(envelope.planes()[0].ordinal(), 0);
    assert_eq!(envelope.planes()[0].stride(), 32);
    assert_eq!(envelope.planes()[0].rows(), 32);
    assert_eq!(envelope.planes()[0].length(), 1024);
    assert_eq!(
        envelope.planes()[0].sha256(),
        sha256_media_worker_bytes_v1(&planes[0])
    );
    assert_eq!(envelope.planes()[2].ordinal(), 2);
    assert_eq!(
        envelope.output_sha256().to_hex(),
        aggregate_hex(&planes),
        "the aggregate digest is preserved exactly as claimed"
    );
}

#[test]
fn enforces_the_envelope_size_boundary_including_whitespace() {
    let baseline = valid_json();

    assert_eq!(
        refusal_kind(b""),
        MediaWorkerProtocolRefusalKindV1::Empty,
        "an empty body is refused before parsing"
    );

    let padded = format!(
        "{baseline}{}",
        " ".repeat(MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1 - baseline.len())
    );
    assert_eq!(
        padded.len(),
        MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1,
        "the fixture reaches exactly the documented bound"
    );
    assert!(
        parse_media_worker_envelope_v1(padded.as_bytes()).is_ok(),
        "trailing whitespace is allowed up to and including the bound"
    );

    let oversize = format!("{padded} ");
    assert_eq!(oversize.len(), MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1 + 1);
    assert_eq!(
        refusal_kind(oversize.as_bytes()),
        MediaWorkerProtocolRefusalKindV1::Oversize,
        "one byte past the bound is refused before parsing"
    );
}

#[test]
fn rejects_duplicate_keys_at_every_nesting_level() {
    let baseline = valid_json();

    let root_duplicate = baseline.replacen(r#""version":1,"#, r#""version":1,"version":1,"#, 1);
    refused_as(
        &root_duplicate,
        MediaWorkerProtocolRefusalKindV1::DuplicateField,
    );

    let plane_duplicate = baseline.replacen(r#""ordinal":0,"#, r#""ordinal":0,"ordinal":0,"#, 1);
    refused_as(
        &plane_duplicate,
        MediaWorkerProtocolRefusalKindV1::DuplicateField,
    );

    let trailing_plane_duplicate =
        baseline.replacen(r#""length":1024,"#, r#""length":1024,"length":1024,"#, 1);
    refused_as(
        &trailing_plane_duplicate,
        MediaWorkerProtocolRefusalKindV1::DuplicateField,
    );
}

#[test]
fn rejects_unknown_keys_at_every_nesting_level() {
    let baseline = valid_json();

    refused_as(
        &baseline.replacen(r#""version":1,"#, r#""version":1,"extra":1,"#, 1),
        MediaWorkerProtocolRefusalKindV1::UnknownField,
    );
    refused_as(
        &baseline.replacen(r#""ordinal":0,"#, r#""ordinal":0,"extra":1,"#, 1),
        MediaWorkerProtocolRefusalKindV1::UnknownField,
    );
}

/// One plane-object fixture for the structural cases.
fn plane_json(ordinal: u32, stride: u64, rows: u64, length: u64, sha: &str) -> String {
    format!(
        r#"{{"ordinal":{ordinal},"stride":{stride},"rows":{rows},"length":{length},"sha256":"{sha}"}}"#
    )
}

/// The accepted baseline with a substituted plane array.
fn json_with_planes(planes_json: &str) -> String {
    let source_sha = sha256_media_worker_bytes_v1(&source()).to_hex();
    let output = aggregate_hex(&planes());
    format!(
        r#"{{"version":1,"status":"decoded","format":"avif","mime":"image/avif","source_length":299,"source_sha256":"{source_sha}","width":32,"height":32,"bit_depth":8,"color_model":"yuv","has_alpha":false,"is_grid":false,"is_sequence":false,"orientation":"identity","planes":{planes_json},"output_sha256":"{output}"}}"#
    )
}

#[test]
fn rejects_trailing_documents_and_extra_input() {
    let baseline = valid_json();

    refused_as(
        &format!("{baseline}{baseline}"),
        MediaWorkerProtocolRefusalKindV1::TrailingData,
    );
    refused_as(
        &format!("{baseline} 5"),
        MediaWorkerProtocolRefusalKindV1::TrailingData,
    );
    refused_as(
        &format!("{baseline}// a comment"),
        MediaWorkerProtocolRefusalKindV1::TrailingData,
    );
}

#[test]
fn rejects_invalid_utf8_and_malformed_json() {
    assert_eq!(
        refusal_kind(&[b'{', 0xff, b'}']),
        MediaWorkerProtocolRefusalKindV1::Syntax,
        "an invalid UTF-8 byte sequence is a syntax refusal"
    );
    for malformed in ["{", "}", "{\"version\":1", "{\"version\":}", "version"] {
        assert_eq!(
            refusal_kind(malformed.as_bytes()),
            MediaWorkerProtocolRefusalKindV1::Syntax,
            "{malformed} is not one complete JSON object"
        );
    }
}

#[test]
fn rejects_a_non_object_root() {
    for root in ["[]", "5", "\"x\"", "null", "true", "[{}]"] {
        let kind = refusal_kind(root.as_bytes());
        assert!(
            matches!(
                kind,
                MediaWorkerProtocolRefusalKindV1::Syntax
                    | MediaWorkerProtocolRefusalKindV1::FieldValue
            ),
            "{root} must be refused as a shape problem, got {}",
            kind.code()
        );
    }
}

#[test]
fn rejects_wrong_contract_tokens() {
    let baseline = valid_json();
    for (from, to) in [
        (r#""version":1,"#, r#""version":2,"#),
        (r#""status":"decoded","#, r#""status":"ok","#),
        (r#""format":"avif","#, r#""format":"png","#),
        (r#""mime":"image/avif","#, r#""mime":"image/png","#),
        (r#""color_model":"yuv","#, r#""color_model":"rgb","#),
        (
            r#""orientation":"identity","#,
            r#""orientation":"flipped","#,
        ),
        (r#""bit_depth":8,"#, r#""bit_depth":300,"#),
    ] {
        refused_as(
            &baseline.replacen(from, to, 1),
            MediaWorkerProtocolRefusalKindV1::FieldValue,
        );
    }
}

#[test]
fn rejects_malformed_digests() {
    let baseline = valid_json();
    let lower = sha256_media_worker_bytes_v1(&source()).to_hex();

    refused_as(
        &baseline.replacen(&lower, &lower.to_uppercase(), 1),
        MediaWorkerProtocolRefusalKindV1::FieldValue,
    );
    refused_as(
        &baseline.replacen(&lower, &lower[2..], 1),
        MediaWorkerProtocolRefusalKindV1::FieldValue,
    );
    refused_as(
        &baseline.replacen(&lower, &"z".repeat(64), 1),
        MediaWorkerProtocolRefusalKindV1::FieldValue,
    );
}

#[test]
fn rejects_negative_floating_overflowing_and_coerced_numbers() {
    let baseline = valid_json();

    for (from, to) in [
        (r#""source_length":299,"#, r#""source_length":-299,"#),
        (r#""source_length":299,"#, r#""source_length":299.5,"#),
        (
            r#""source_length":299,"#,
            r#""source_length":18446744073709551616,"#,
        ),
        (r#""source_length":299,"#, r#""source_length":"299","#),
        (r#""width":32,"#, r#""width":-1,"#),
        (r#""width":32,"#, r#""width":1.0,"#),
        (r#""has_alpha":false,"#, r#""has_alpha":1,"#),
        (r#""has_alpha":false,"#, r#""has_alpha":"false","#),
        (r#""length":1024,"#, r#""length":1.024e3,"#),
    ] {
        refused_as(
            &baseline.replacen(from, to, 1),
            MediaWorkerProtocolRefusalKindV1::FieldValue,
        );
    }
}

#[test]
fn rejects_surplus_missing_and_empty_plane_lists() {
    let sha = sha256_media_worker_bytes_v1(&planes()[0]).to_hex();

    refused_as(
        &json_with_planes("[]"),
        MediaWorkerProtocolRefusalKindV1::FieldValue,
    );

    let five = (0..5)
        .map(|ordinal| plane_json(ordinal, 32, 32, 1024, &sha))
        .collect::<Vec<_>>()
        .join(",");
    refused_as(
        &json_with_planes(&format!("[{five}]")),
        MediaWorkerProtocolRefusalKindV1::FieldValue,
    );

    let four = (0..4)
        .map(|ordinal| plane_json(ordinal, 32, 32, 1024, &sha))
        .collect::<Vec<_>>()
        .join(",");
    assert!(
        parse_media_worker_envelope_v1(json_with_planes(&format!("[{four}]")).as_bytes()).is_ok(),
        "four planes parse structurally; colour-model agreement is the parent's decision"
    );
}

#[test]
fn rejects_missing_root_and_plane_keys() {
    let baseline = valid_json();

    refused_as(
        &baseline.replacen(r#""is_grid":false,"#, "", 1),
        MediaWorkerProtocolRefusalKindV1::MissingField,
    );
    refused_as(
        &baseline.replacen(r#""rows":32,"#, "", 1),
        MediaWorkerProtocolRefusalKindV1::MissingField,
    );
    refused_as(
        &baseline.replacen(r#""orientation":"identity","#, "", 1),
        MediaWorkerProtocolRefusalKindV1::MissingField,
    );
}
