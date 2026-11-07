//! CLI adapter boundary: argument translation, exit mapping and diagnostic safety.

use std::ffi::OsString;

use solo_adapters::{
    CliOutcome, CliRefusalCode, EXIT_INTERNAL, EXIT_OK, EXIT_REFUSED, parse_request, run,
};
use solo_application::BootstrapPreflightService;

fn argv(tokens: &[&str]) -> Vec<OsString> {
    let mut args = vec![OsString::from("solo-cli")];
    args.extend(tokens.iter().map(OsString::from));
    args
}

fn refusal_of(tokens: &[&str]) -> CliRefusalCode {
    match parse_request(argv(tokens)) {
        Ok(_) => panic!("expected an argument refusal for {tokens:?}"),
        Err(refusal) => refusal.code(),
    }
}

#[test]
fn accepts_either_flag_order() {
    let first = parse_request(argv(&[
        "bootstrap-preflight",
        "--entity-id",
        "entity-7",
        "--base-url",
        "https://example.com",
    ]))
    .expect("valid");

    let second = parse_request(argv(&[
        "bootstrap-preflight",
        "--base-url",
        "https://example.com",
        "--entity-id",
        "entity-7",
    ]))
    .expect("valid");

    assert_eq!(first, second);
    assert_eq!(first.entity_id(), "entity-7");
    assert_eq!(first.base_url(), "https://example.com");
}

#[test]
fn refuses_missing_arguments() {
    assert_eq!(
        refusal_of(&["bootstrap-preflight"]),
        CliRefusalCode::Missing
    );
    assert_eq!(
        refusal_of(&["bootstrap-preflight", "--entity-id", "solo"]),
        CliRefusalCode::Missing
    );
    assert_eq!(
        refusal_of(&["bootstrap-preflight", "--entity-id"]),
        CliRefusalCode::Missing
    );
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--base-url",
            "",
            "--entity-id",
            "solo"
        ]),
        CliRefusalCode::Missing
    );
}

#[test]
fn refuses_duplicated_options() {
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
        ]),
        CliRefusalCode::Duplicate
    );
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
            "--base-url",
            "https://example.org",
        ]),
        CliRefusalCode::Duplicate
    );
}

#[test]
fn refuses_unknown_flags_and_positional_arguments() {
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
            "--verbose",
        ]),
        CliRefusalCode::Unknown
    );
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
            "stray",
        ]),
        CliRefusalCode::Unknown
    );
    assert_eq!(
        refusal_of(&["other-command"]),
        CliRefusalCode::Unknown,
        "an unknown subcommand must be refused"
    );
    assert_eq!(refusal_of(&[]), CliRefusalCode::Missing);
}

#[test]
fn refuses_a_value_that_is_actually_another_flag() {
    assert_eq!(
        refusal_of(&[
            "bootstrap-preflight",
            "--entity-id",
            "--base-url",
            "https://example.com",
        ]),
        CliRefusalCode::Missing
    );
}

#[cfg(windows)]
#[test]
fn refuses_non_utf8_arguments() {
    use std::os::windows::ffi::OsStringExt;

    let mut args = vec![OsString::from("solo-cli")];
    args.push(OsString::from("bootstrap-preflight"));
    args.push(OsString::from("--entity-id"));
    // A lone surrogate cannot be represented as UTF-8 text.
    args.push(OsString::from_wide(&[0xD800]));
    args.push(OsString::from("--base-url"));
    args.push(OsString::from("https://example.com"));

    let refusal = parse_request(args).expect_err("must be refused");
    assert_eq!(refusal.code(), CliRefusalCode::Encoding);
    assert_eq!(refusal.code().as_str(), "PREFLIGHT_ARGS_ENCODING");
}

#[test]
fn renders_exactly_one_success_line() {
    let outcome = run(
        argv(&[
            "bootstrap-preflight",
            "--entity-id",
            "entity-7",
            "--base-url",
            "https://EXAMPLE.com:443/alpha",
        ]),
        &BootstrapPreflightService,
    );

    assert_eq!(outcome.exit_code(), EXIT_OK);
    let stdout = outcome
        .stdout()
        .expect("stdout must carry the success line");
    assert_eq!(
        stdout,
        "preflight=ok entity_id=entity-7 base=https://example.com/alpha\n"
    );
    assert_eq!(stdout.lines().count(), 1);
    assert!(outcome.stderr().is_none());
}

#[test]
fn refusal_outcomes_emit_no_stdout_and_a_safe_diagnostic() {
    let outcome = run(
        argv(&[
            "bootstrap-preflight",
            "--entity-id",
            "Main",
            "--base-url",
            "https://example.com",
        ]),
        &BootstrapPreflightService,
    );

    assert_eq!(outcome.exit_code(), EXIT_REFUSED);
    assert!(outcome.stdout().is_none(), "refusals must not use stdout");

    let stderr = outcome.stderr().expect("a diagnostic is required");
    assert!(stderr.starts_with("error: PREFLIGHT_ENTITY_ID_INVALID: "));
    assert!(stderr.ends_with('\n'));
    assert!(
        !stderr.chars().any(|c| c.is_control() && c != '\n'),
        "diagnostics must not carry control characters: {stderr:?}"
    );
    assert!(!stderr.contains("Main"), "diagnostics must not echo input");
}

#[test]
fn argument_refusals_are_reported_with_their_own_codes() {
    let cases = [
        (
            vec!["bootstrap-preflight", "--entity-id", "solo"],
            "PREFLIGHT_ARGS_MISSING",
        ),
        (
            vec![
                "bootstrap-preflight",
                "--entity-id",
                "solo",
                "--entity-id",
                "solo",
                "--base-url",
                "https://example.com",
            ],
            "PREFLIGHT_ARGS_DUPLICATE",
        ),
        (
            vec![
                "bootstrap-preflight",
                "--entity-id",
                "solo",
                "--base-url",
                "https://example.com",
                "--unknown",
            ],
            "PREFLIGHT_ARGS_UNKNOWN",
        ),
    ];

    for (tokens, expected_code) in cases {
        let outcome = run(argv(&tokens), &BootstrapPreflightService);
        assert_eq!(outcome.exit_code(), EXIT_REFUSED);
        match outcome {
            CliOutcome::Refused { code, detail } => {
                assert_eq!(code, expected_code);
                assert!(!detail.is_empty());
            }
            other => panic!("expected refusal for {tokens:?}, got {other:?}"),
        }
    }
}

#[test]
fn base_refusals_are_distinguished() {
    let invalid = run(
        argv(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "ftp://example.com",
        ]),
        &BootstrapPreflightService,
    );
    let unsupported = run(
        argv(&[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://192.168.0.1",
        ]),
        &BootstrapPreflightService,
    );

    assert_eq!(
        invalid,
        CliOutcome::Refused {
            code: "PREFLIGHT_BASE_INVALID",
            detail: "scheme",
        }
    );
    assert_eq!(
        unsupported,
        CliOutcome::Refused {
            code: "PREFLIGHT_BASE_UNSUPPORTED",
            detail: "ip-literal",
        }
    );
}

#[test]
fn exit_codes_are_distinct_and_stable() {
    assert_eq!(EXIT_OK, 0);
    assert_eq!(EXIT_INTERNAL, 1);
    assert_eq!(EXIT_REFUSED, 2);
    assert_eq!(
        CliOutcome::Internal {
            detail: "unexpected"
        }
        .exit_code(),
        EXIT_INTERNAL
    );
}
