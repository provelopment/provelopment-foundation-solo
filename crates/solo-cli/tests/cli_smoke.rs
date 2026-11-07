//! End-to-end tests through the **actual compiled process**, capturing real exit
//! statuses and streams rather than simulating them.
//!
//! Scratch material is created only inside this checkout's own `.temp/` root, with
//! exclusive creation, symlink/reparse-point validation and non-recursive cleanup
//! that can only ever remove a child this test positively created.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const BIN: &str = env!("CARGO_BIN_EXE_solo-cli");

/// The Solo checkout root, resolved from the compile-time manifest path.
const SOLO_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

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
    assert!(
        stderr_of(&output).contains("PREFLIGHT_ENTITY_ID_INVALID"),
        "unexpected diagnostic: {:?}",
        stderr_of(&output)
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

// ---------------------------------------------------------------------------
// Scratch confinement: authorized root, exclusive creation, safe cleanup.
// ---------------------------------------------------------------------------

/// Resolves the one authorized scratch root for this checkout.
///
/// The root is this Solo checkout's own `.temp/`, derived from the compile-time
/// manifest path. It must stay inside the canonical checkout and must not be
/// redirected by a symlink or reparse point; there is deliberately no fallback to a
/// general OS temporary directory.
fn scratch_root() -> PathBuf {
    let checkout = std::fs::canonicalize(SOLO_ROOT).expect("the Solo checkout root must resolve");
    let root = checkout.join(".temp");
    std::fs::create_dir_all(&root).expect("the authorized task scratch root must be creatable");
    let resolved = std::fs::canonicalize(&root).expect("the scratch root must resolve");

    assert!(
        resolved.starts_with(&checkout),
        "scratch root {resolved:?} escaped the Solo checkout {checkout:?}"
    );
    assert!(
        !is_redirected(&resolved),
        "scratch root {resolved:?} is a symlink or reparse point"
    );

    resolved
}

#[cfg(windows)]
fn is_redirected(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    /// `FILE_ATTRIBUTE_REPARSE_POINT`
    const REPARSE: u32 = 0x0400;
    std::fs::symlink_metadata(path)
        .map_or(true, |metadata| metadata.file_attributes() & REPARSE != 0)
}

#[cfg(not(windows))]
fn is_redirected(path: &Path) -> bool {
    std::fs::symlink_metadata(path).map_or(true, |metadata| metadata.file_type().is_symlink())
}

/// A uniquely named scratch child that this test positively created.
struct ScratchChild {
    path: PathBuf,
}

impl ScratchChild {
    /// Creates a fresh, never-reused child under the authorized scratch root.
    fn create() -> Self {
        let root = scratch_root();
        let path = root.join(unique_child_name());
        Self::create_at(&path).expect("exclusive scratch child")
    }

    /// Creates exactly `path`, refusing anything that already exists.
    ///
    /// `create_dir` is used rather than `create_dir_all` precisely because it fails
    /// when the target exists, which is what makes creation exclusive.
    fn create_at(path: &Path) -> std::io::Result<Self> {
        std::fs::create_dir(path)?;
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for ScratchChild {
    fn drop(&mut self) {
        // Non-recursive by design: only an empty child created by this test can be
        // removed, so foreign or pre-existing material can never be deleted here.
        let _ = std::fs::remove_dir(&self.path);
    }
}

fn unique_child_name() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    format!(
        "solo-cli-smoke-{}-{}-{}",
        std::process::id(),
        nanos,
        sequence
    )
}

fn entry_count(path: &Path) -> usize {
    std::fs::read_dir(path).expect("readable directory").count()
}

#[test]
fn scratch_root_is_inside_the_checkout_and_not_redirected() {
    let root = scratch_root();
    let checkout = std::fs::canonicalize(SOLO_ROOT).expect("checkout root resolves");

    assert!(root.starts_with(&checkout));
    assert!(!is_redirected(&root));
    assert_eq!(
        root.file_name().and_then(|name| name.to_str()),
        Some(".temp"),
        "scratch must use this checkout's own .temp root, with no OS-temp fallback"
    );
}

#[test]
fn a_successful_run_creates_no_files_in_its_working_directory() {
    let child = ScratchChild::create();
    let directory = child.path().to_path_buf();
    assert_eq!(entry_count(&directory), 0, "the child starts empty");

    let output = Command::new(BIN)
        .args([
            "bootstrap-preflight",
            "--entity-id",
            "solo",
            "--base-url",
            "https://example.com",
        ])
        .current_dir(&directory)
        .output()
        .expect("binary must run");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        entry_count(&directory),
        0,
        "the preflight must not write any file"
    );

    drop(child);
    assert!(
        !directory.exists(),
        "the test must leave no task-owned residue behind"
    );
}

#[test]
fn an_existing_scratch_child_is_refused_and_preserved() {
    let root = scratch_root();
    let existing = root.join(unique_child_name());
    std::fs::create_dir(&existing).expect("pre-existing material");
    let sentinel = existing.join("preexisting-material.txt");
    std::fs::write(&sentinel, b"keep me").expect("sentinel");

    match ScratchChild::create_at(&existing) {
        Ok(_) => panic!("an existing scratch child must be refused, never reused"),
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists),
    }

    assert!(
        existing.is_dir(),
        "pre-existing directory must be preserved"
    );
    assert_eq!(
        entry_count(&existing),
        1,
        "pre-existing material must never be removed"
    );
    assert_eq!(
        std::fs::read(&sentinel).expect("sentinel must still be readable"),
        b"keep me"
    );

    std::fs::remove_file(&sentinel).expect("scoped sentinel cleanup");
    std::fs::remove_dir(&existing).expect("scoped child cleanup");
    assert!(!existing.exists());
}

#[test]
fn cleanup_never_removes_foreign_material_recursively() {
    let child = ScratchChild::create();
    let directory = child.path().to_path_buf();
    let foreign = directory.join("foreign-material.txt");
    std::fs::write(&foreign, b"not mine to delete").expect("foreign material");

    drop(child);

    assert!(
        directory.exists(),
        "cleanup must not remove a non-empty directory"
    );
    assert!(
        foreign.exists(),
        "cleanup must never delete material it did not create"
    );

    std::fs::remove_file(&foreign).expect("scoped foreign cleanup");
    std::fs::remove_dir(&directory).expect("scoped child cleanup");
    assert!(!directory.exists());
}
