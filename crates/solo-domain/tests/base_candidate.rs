//! `BaseCandidateV1` canonicalization and refusal across the narrow engineering
//! subset, including explicit unsupported-by-this-preflight forms.

use solo_domain::{BaseCandidateErrorKind, BaseCandidateV1, BaseScheme, MAX_PREFIX_SEGMENTS};

#[test]
fn canonicalizes_the_required_example() {
    let parsed = BaseCandidateV1::parse("https://EXAMPLE.com:443/alpha").expect("must accept");
    assert_eq!(parsed.as_str(), "https://example.com/alpha");
    assert_eq!(parsed.host(), "example.com");
    assert_eq!(parsed.scheme(), BaseScheme::Https);
    assert_eq!(parsed.port(), None);
    assert_eq!(parsed.segments(), ["alpha"]);
}

#[test]
fn accepts_a_cleartext_origin() {
    let parsed = BaseCandidateV1::parse("http://example.org").expect("must accept");
    assert_eq!(parsed.as_str(), "http://example.org");
    assert_eq!(parsed.scheme(), BaseScheme::Http);
    assert!(parsed.segments().is_empty());
}

#[test]
fn collapses_the_origin_only_slash() {
    for authored in ["http://example.org", "http://example.org/"] {
        let parsed = BaseCandidateV1::parse(authored).expect("must accept");
        assert_eq!(parsed.as_str(), "http://example.org");
    }
}

#[test]
fn drops_a_single_terminal_slash_from_a_prefix() {
    let parsed = BaseCandidateV1::parse("https://example.com/alpha/").expect("must accept");
    assert_eq!(parsed.as_str(), "https://example.com/alpha");
}

#[test]
fn removes_default_ports_and_preserves_nondefault_ports() {
    assert_eq!(
        BaseCandidateV1::parse("http://example.com:80")
            .expect("must accept")
            .as_str(),
        "http://example.com"
    );
    let kept = BaseCandidateV1::parse("https://example.com:8443/a").expect("must accept");
    assert_eq!(kept.as_str(), "https://example.com:8443/a");
    assert_eq!(kept.port(), Some(8443));
}

#[test]
fn accepts_the_maximum_prefix_depth() {
    let parsed = BaseCandidateV1::parse("https://example.com/a/b/c/d/e").expect("five segments");
    assert_eq!(parsed.segments().len(), MAX_PREFIX_SEGMENTS);

    let too_deep = BaseCandidateV1::parse("https://example.com/a/b/c/d/e/f").expect_err("six");
    assert_eq!(too_deep.kind(), BaseCandidateErrorKind::Invalid);
    assert_eq!(too_deep.detail(), "segment-too-many");
}

#[test]
fn accepts_digits_and_hyphens_inside_labels_and_segments() {
    let parsed = BaseCandidateV1::parse("https://a-b.example-1.com/a1-b2").expect("must accept");
    assert_eq!(parsed.as_str(), "https://a-b.example-1.com/a1-b2");
}

#[test]
fn uppercase_scheme_is_never_repaired() {
    let error = BaseCandidateV1::parse("HTTPS://example.com").expect_err("must refuse");
    assert_eq!(error.detail(), "scheme-case");
}

#[test]
fn canonicalization_is_idempotent() {
    let first = BaseCandidateV1::parse("https://EXAMPLE.com:443/alpha").expect("must accept");
    let second = BaseCandidateV1::parse(first.as_str()).expect("canonical form re-parses");
    assert_eq!(first, second);
}

#[test]
fn refuses_structurally_invalid_forms() {
    let cases = [
        ("", "empty"),
        ("example.com", "scheme"),
        ("//example.com", "scheme"),
        ("ftp://example.com", "scheme"),
        ("HTTPS://example.com", "scheme-case"),
        ("https://", "host-missing"),
        ("https:///a", "host-missing"),
        ("https://user:pw@example.com", "credentials"),
        ("https://example.com?q=1", "query"),
        ("https://example.com#frag", "fragment"),
        ("https://example.com\\a", "backslash"),
        ("https://exa mple.com", "control-or-whitespace"),
        ("https://exa\tmple.com", "control-or-whitespace"),
        ("https://-bad.example", "label-edge"),
        ("https://bad-.example", "label-edge"),
        ("https://a..b", "label-empty"),
        ("https://example.com:", "port-missing"),
        ("https://example.com:0", "port-out-of-range"),
        ("https://example.com:65536", "port-out-of-range"),
        ("https://example.com:080", "port-leading-zero"),
        ("https://example.com:8a", "port-not-numeric"),
        ("https://example.com//a", "segment-empty"),
        ("https://example.com/a//b", "segment-empty"),
        ("https://example.com/./a", "segment-traversal"),
        ("https://example.com/../a", "segment-traversal"),
        ("https://example.com/%2e%2e/a", "percent-encoded"),
        ("https://example.com/Alpha", "segment-character"),
        ("https://example.com/a_b", "segment-character"),
        ("https://example.com/1a", "segment-character"),
    ];

    for (authored, expected_detail) in cases {
        let error = BaseCandidateV1::parse(authored).expect_err("must be refused");
        assert_eq!(
            error.kind(),
            BaseCandidateErrorKind::Invalid,
            "unexpected class for {authored}"
        );
        assert_eq!(error.detail(), expected_detail, "for {authored}");
        assert_eq!(error.code(), "PREFLIGHT_BASE_INVALID", "for {authored}");
    }
}

#[test]
fn refuses_oversized_host_and_label() {
    let long_label = format!("https://{}.example", "a".repeat(64));
    let error = BaseCandidateV1::parse(&long_label).expect_err("must be refused");
    assert_eq!(error.detail(), "label-too-long");

    let long_host = format!("https://{}", "abcdefghij.".repeat(26));
    let error = BaseCandidateV1::parse(&long_host).expect_err("must be refused");
    assert!(
        matches!(
            error.detail(),
            "host-too-long" | "label-too-long" | "label-empty"
        ),
        "unexpected detail {}",
        error.detail()
    );
}

#[test]
fn refuses_forms_this_preflight_does_not_implement() {
    let cases = [
        ("https://münchen.example", "unicode-host"),
        ("https://xn--mnchen-3ya.example", "idna-label"),
        ("https://192.168.0.1", "ip-literal"),
        ("https://[::1]", "ip-literal"),
        ("http://[2001:db8::1]:8080/a", "ip-literal"),
    ];

    for (authored, expected_detail) in cases {
        let error = BaseCandidateV1::parse(authored).expect_err("must be refused");
        assert_eq!(
            error.kind(),
            BaseCandidateErrorKind::Unsupported,
            "unexpected class for {authored}"
        );
        assert_eq!(error.detail(), expected_detail, "for {authored}");
        assert_eq!(error.code(), "PREFLIGHT_BASE_UNSUPPORTED", "for {authored}");
    }
}

#[test]
fn refusal_diagnostics_never_echo_authored_input() {
    let error =
        BaseCandidateV1::parse("https://exa\u{1b}[31mmple.example").expect_err("must refuse");
    let rendered = error.to_string();
    assert!(rendered.starts_with("PREFLIGHT_BASE_INVALID: "));
    assert!(!rendered.contains('\u{1b}'));
    assert!(!rendered.contains("mple"));
}
