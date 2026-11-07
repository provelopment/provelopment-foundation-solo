//! `EntityIdV1` acceptance and refusal, including length boundaries and every
//! coercion the accepted identity contract forbids.

use solo_domain::{ENTITY_ID_MAX_BYTES, EntityIdErrorKind, EntityIdV1};

#[test]
fn accepts_valid_identifiers() {
    for candidate in ["solo", "entity-7", "a", "a1", "entity-7-beta-2026"] {
        let parsed = EntityIdV1::parse(candidate).expect("valid identifier must be accepted");
        assert_eq!(parsed.as_str(), candidate);
        assert_eq!(parsed.to_string(), candidate);
    }
}

#[test]
fn accepts_both_length_boundaries() {
    let minimal = EntityIdV1::parse("a").expect("one byte is the minimum");
    assert_eq!(minimal.as_str().len(), 1);

    let maximal = "a".repeat(ENTITY_ID_MAX_BYTES);
    let parsed = EntityIdV1::parse(&maximal).expect("63 bytes is still valid");
    assert_eq!(parsed.as_str().len(), ENTITY_ID_MAX_BYTES);
    assert_eq!(parsed.as_str(), maximal);
}

#[test]
fn refuses_identifiers_one_byte_over_the_limit() {
    let too_long = "a".repeat(ENTITY_ID_MAX_BYTES + 1);
    let error = EntityIdV1::parse(&too_long).expect_err("64 bytes must be refused");
    assert_eq!(error.kind(), EntityIdErrorKind::TooLong);
    assert_eq!(error.code(), "PREFLIGHT_ENTITY_ID_INVALID");
    assert_eq!(error.detail(), "too-long");
}

#[test]
fn refuses_empty_identifier() {
    let error = EntityIdV1::parse("").expect_err("empty must be refused");
    assert_eq!(error.kind(), EntityIdErrorKind::Empty);
}

#[test]
fn refuses_uppercase_without_coercing_it() {
    let error = EntityIdV1::parse("Main").expect_err("case must never be repaired");
    assert_eq!(error.kind(), EntityIdErrorKind::InvalidStart);
    assert!(EntityIdV1::parse("ENTITY").is_err());
    assert!(EntityIdV1::parse("Solo").is_err());
}

#[test]
fn refuses_leading_digit_or_separator() {
    for candidate in ["1x", "7", "_a", "-abc", ".a"] {
        let error = EntityIdV1::parse(candidate).expect_err("must refuse");
        assert!(
            matches!(
                error.kind(),
                EntityIdErrorKind::InvalidStart | EntityIdErrorKind::InvalidCharacter
            ),
            "unexpected refusal kind for {candidate}: {:?}",
            error.kind()
        );
    }
}

#[test]
fn refuses_internal_forbidden_characters() {
    for candidate in ["a.b", "a/b", "a_b", "a%2Fb", "a b", "a\tb", "a+b", "a:b"] {
        let error = EntityIdV1::parse(candidate).expect_err("must refuse");
        assert_eq!(
            error.kind(),
            EntityIdErrorKind::InvalidCharacter,
            "unexpected refusal kind for {candidate}"
        );
    }
}

#[test]
fn refuses_non_ascii_without_transliteration() {
    for candidate in ["münchen", "ソロ", "entity–7"] {
        let error = EntityIdV1::parse(candidate).expect_err("must refuse");
        assert_eq!(error.kind(), EntityIdErrorKind::NonAscii);
    }
}

#[test]
fn refuses_hyphen_edges_and_runs() {
    let trailing = EntityIdV1::parse("entity-").expect_err("must refuse");
    assert_eq!(trailing.kind(), EntityIdErrorKind::TrailingHyphen);

    let doubled = EntityIdV1::parse("entity--7").expect_err("must refuse");
    assert_eq!(doubled.kind(), EntityIdErrorKind::ConsecutiveHyphens);
}

#[test]
fn refusal_diagnostics_never_echo_authored_input() {
    let authored = "Bad\u{7}ID\nwith-controls";
    let error = EntityIdV1::parse(authored).expect_err("must refuse");
    let rendered = error.to_string();
    assert!(rendered.starts_with("PREFLIGHT_ENTITY_ID_INVALID: "));
    assert!(!rendered.contains("Bad"));
    assert!(!rendered.contains('\u{7}'));
    assert!(!rendered.contains('\n'));
}

#[test]
fn identity_is_stable_across_repeated_parses() {
    let first = EntityIdV1::parse("entity-7").expect("valid");
    let second = EntityIdV1::parse("entity-7").expect("valid");
    assert_eq!(first, second);
}
