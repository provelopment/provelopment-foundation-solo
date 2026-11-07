//! End-to-end tests through the **actual compiled process**, capturing real exit
//! statuses and streams rather than simulating them.

use std::path::Path;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_solo-cli");

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("the compiled solo-cli binary must be executable")
}

fn stdout_of(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout must be UTF-8")
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr must be UTF-8")
}

#[test]
fn required_success_case_canonicalizes_the_authored_base() {
    let output = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "entity-7",
        "--base-url",
        "https://EXAMPLE.com:443/alpha",
    ]);

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout_of(&output),
        "preflight=ok entity_id=entity-7 base=https://example.com/alpha\n"
    );
    assert_eq!(stderr_of(&output), "");
}

#[test]
fn required_refusal_case_reports_an_identity_code() {
    let output = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "Main",
        "--base-url",
        "https://example.com",
    ]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(stdout_of(&output), "", "a refusal must not write stdout");
    let stderr = stderr_of(&output);
    assert!(
        stderr.contains("PREFLIGHT_ENTITY_ID_INVALID"),
        "unexpected diagnostic: {stderr:?}"
    );
}

#[test]
fn accepts_a_cleartext_origin() {
    let output = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "solo",
        "--base-url",
        "http://example.org",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout_of(&output),
        "preflight=ok entity_id=solo base=http://example.org\n"
    );
}

#[test]
fn accepts_five_prefix_segments_and_a_nondefault_port() {
    let deep = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "solo",
        "--base-url",
        "https://example.com/a/b/c/d/e",
    ]);
    assert_eq!(deep.status.code(), Some(0));
    assert!(stdout_of(&deep).contains("base=https://example.com/a/b/c/d/e"));

    let ported = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "solo",
        "--base-url",
        "https://example.com:8443/alpha",
    ]);
    assert_eq!(ported.status.code(), Some(0));
    assert!(stdout_of(&ported).contains("base=https://example.com:8443/alpha"));
}

#[test]
fn accepts_the_reversed_flag_order() {
    let output = run(&[
        "bootstrap-preflight",
        "--base-url",
        "https://example.com",
        "--entity-id",
        "entity-7",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        stdout_of(&output),
        "preflight=ok entity_id=entity-7 base=https://example.com\n"
    );
}

#[test]
fn entity_identity_survives_a_changed_base_address() {
    let first = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "entity-7",
        "--base-url",
        "https://example.com/alpha",
    ]);
    let second = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "entity-7",
        "--base-url",
        "https://other.example/beta",
    ]);

    assert_eq!(first.status.code(), Some(0));
    assert_eq!(second.status.code(), Some(0));
    assert!(stdout_of(&first).contains("entity_id=entity-7"));
    assert!(stdout_of(&second).contains("entity_id=entity-7"));
}

#[test]
fn refuses_malformed_and_unsupported_bases_with_distinct_codes() {
    let invalid = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "solo",
        "--base-url",
        "https://user:pw@example.com",
    ]);
    assert_eq!(invalid.status.code(), Some(2));
    assert!(stderr_of(&invalid).contains("PREFLIGHT_BASE_INVALID"));

    let unsupported = run(&[
        "bootstrap-preflight",
        "--entity-id",
        "solo",
        "--base-url",
        "https://xn--mnchen-3ya.example",
    ]);
    assert_eq!(unsupported.status.code(), Some(2));
    assert!(stderr_of(&unsupported).contains("PREFLIGHT_BASE_UNSUPPORTED"));
}

#[test]
fn refuses_unusable_argument_sets() {
    let cases: [&[&str]; 4] = [
        &["bootstrap-preflight", "--entity-id", "solo"],
        &[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
        ],
        &[
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
            "--unknown",
        ],
        &["bootstrap-preflight", "--entity-id", "solo", "--base-url"],
    ];

    for args in cases {
        let output = run(args);
        assert_eq!(output.status.code(), Some(2), "for {args:?}");
        assert_eq!(stdout_of(&output), "", "for {args:?}");
        assert!(
            stderr_of(&output).starts_with("error: PREFLIGHT_"),
            "unexpected diagnostic for {args:?}"
        );
    }
}

#[test]
fn a_successful_run_creates_no_files_in_the_working_directory() {
    let scratch = std::env::temp_dir().join(format!("solo-cli-probe-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("scratch directory");
    let before = directory_len(&scratch);

    let output = Command::new(BIN)
        .args([
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
        ])
        .current_dir(&scratch)
        .output()
        .expect("binary must run");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        directory_len(&scratch),
        before,
        "the preflight must not write any file"
    );

    std::fs::remove_dir_all(&scratch).expect("scratch cleanup");
}

fn directory_len(path: &Path) -> usize {
    std::fs::read_dir(path).expect("readable directory").count()
}
