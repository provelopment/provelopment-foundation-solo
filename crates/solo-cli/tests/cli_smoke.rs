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
//
// The validator inspects the **unresolved** `.temp` entry before anything resolves
// or uses it. Inspecting only the resolved target cannot prove the original entry
// was not redirected: a junction or symlink pointing at another `.temp` directory
// would present a plain directory after resolution. Resolution therefore happens
// strictly after the raw-entry check has passed, and the resolved path must still be
// this checkout's own direct `.temp` root.

/// Why a candidate scratch root was refused.
#[derive(Debug, PartialEq, Eq)]
enum ScratchRootError {
    /// The entry could not be inspected; metadata and permission errors fail closed.
    Unreadable,
    /// The entry is a Windows reparse point or a Unix symbolic link.
    Redirected,
    /// The entry exists but is not a directory.
    NotADirectory,
    /// The resolved path is not this checkout's direct `.temp` root.
    NotTheCheckoutTemp,
}

/// Detects a redirected directory entry from its **unresolved** metadata.
#[cfg(windows)]
fn is_redirected_entry(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    /// `FILE_ATTRIBUTE_REPARSE_POINT`, independent of the directory bit.
    const REPARSE_POINT: u32 = 0x0400;
    metadata.file_attributes() & REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_redirected_entry(metadata: &std::fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

fn inspect_unresolved(metadata: &std::fs::Metadata) -> Result<(), ScratchRootError> {
    if is_redirected_entry(metadata) {
        return Err(ScratchRootError::Redirected);
    }
    if !metadata.is_dir() {
        return Err(ScratchRootError::NotADirectory);
    }
    Ok(())
}

/// Ensures `checkout_root`'s direct `.temp` child is a genuine, unredirected
/// directory and returns its resolved path.
///
/// The direct child path is constructed without resolving that entry. It is
/// inspected with `symlink_metadata` first; only a passing entry is resolved, and the
/// resolved path must still be exactly `<checkout>/.temp`. A missing child is created
/// with `create_dir`, which never follows an existing entry and never replaces one.
fn ensure_scratch_root(checkout_root: &Path) -> Result<PathBuf, ScratchRootError> {
    let checkout =
        std::fs::canonicalize(checkout_root).map_err(|_| ScratchRootError::Unreadable)?;
    let candidate = checkout.join(".temp");

    match std::fs::symlink_metadata(&candidate) {
        Ok(metadata) => inspect_unresolved(&metadata)?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir(&candidate).map_err(|_| ScratchRootError::Unreadable)?;
            let created =
                std::fs::symlink_metadata(&candidate).map_err(|_| ScratchRootError::Unreadable)?;
            inspect_unresolved(&created)?;
        }
        Err(_) => return Err(ScratchRootError::Unreadable),
    }

    let resolved = std::fs::canonicalize(&candidate).map_err(|_| ScratchRootError::Unreadable)?;
    if resolved != candidate {
        return Err(ScratchRootError::NotTheCheckoutTemp);
    }
    Ok(resolved)
}

/// Creates a genuine directory redirect: a junction on Windows, a symlink elsewhere.
#[cfg(windows)]
fn create_directory_link(target: &Path, link: &Path) {
    // `mklink /J` creates a real directory junction, a reparse point that needs no
    // elevation and no policy change.
    let output = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output()
        .expect("cmd must be available to create a junction");
    assert!(
        output.status.success(),
        "junction creation failed: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[cfg(not(windows))]
fn create_directory_link(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("directory symlink creation");
}

/// Removes the redirect itself, never the directory it points at.
#[cfg(windows)]
fn remove_directory_link(link: &Path) {
    std::fs::remove_dir(link).expect("removing a junction removes only the link");
}

#[cfg(not(windows))]
fn remove_directory_link(link: &Path) {
    std::fs::remove_file(link).expect("removing a symlink removes only the link");
}

/// Resolves the one authorized scratch root for this checkout.
///
/// Derived from the compile-time manifest path, validated by [`ensure_scratch_root`],
/// with no fallback to a general OS temporary directory.
fn scratch_root() -> PathBuf {
    ensure_scratch_root(Path::new(SOLO_ROOT))
        .expect("this checkout's own .temp root must be a genuine unredirected directory")
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

    assert_eq!(
        root,
        checkout.join(".temp"),
        "the scratch root must be exactly this checkout's direct .temp"
    );

    let entry = std::fs::symlink_metadata(&root).expect("entry metadata");
    assert!(
        !is_redirected_entry(&entry),
        "the unresolved .temp entry must not be a reparse point or symlink"
    );
    assert!(entry.is_dir());
}

#[test]
fn a_missing_temp_child_is_created_and_then_inspected() {
    let fixture = ScratchChild::create();
    let fixture_root = fixture.path().to_path_buf();

    let resolved = ensure_scratch_root(&fixture_root).expect("missing .temp is created");
    assert_eq!(
        resolved.file_name().and_then(|name| name.to_str()),
        Some(".temp")
    );
    assert!(resolved.is_dir());
    assert_eq!(entry_count(&resolved), 0);

    let entry = std::fs::symlink_metadata(&resolved).expect("entry metadata");
    assert!(!is_redirected_entry(&entry));

    std::fs::remove_dir(&resolved).expect("scoped fixture cleanup");
    assert!(!resolved.exists());
}

#[test]
fn a_non_directory_temp_child_is_refused() {
    let fixture = ScratchChild::create();
    let fixture_root = fixture.path().to_path_buf();
    let not_a_directory = fixture_root.join(".temp");
    std::fs::write(&not_a_directory, b"not a directory").expect("fixture file");

    assert_eq!(
        ensure_scratch_root(&fixture_root),
        Err(ScratchRootError::NotADirectory)
    );

    assert!(
        not_a_directory.is_file(),
        "the refused entry must be left intact"
    );
    std::fs::remove_file(&not_a_directory).expect("scoped fixture cleanup");
}

#[test]
fn a_redirected_temp_resolving_inside_the_checkout_is_refused() {
    // Everything below is positively owned fixture space inside this checkout's real
    // `.temp`; the real `.temp` entry itself is never touched.
    let fixture = ScratchChild::create();
    let container = fixture.path().to_path_buf();

    let target = container.join("target");
    std::fs::create_dir(&target).expect("fixture target");
    let real_temp = target.join(".temp");
    std::fs::create_dir(&real_temp).expect("fixture target .temp");
    let material = real_temp.join("existing-material.txt");
    std::fs::write(&material, b"must survive validation").expect("fixture material");

    let candidate_root = container.join("candidate");
    std::fs::create_dir(&candidate_root).expect("fixture candidate root");
    let redirected = candidate_root.join(".temp");
    create_directory_link(&real_temp, &redirected);

    // The arrangement the previous post-resolution-only check would have accepted:
    // the resolved destination is a directory, is not itself redirected, and lies
    // inside this permitted checkout.
    let checkout = std::fs::canonicalize(SOLO_ROOT).expect("checkout root resolves");
    let resolved_view = std::fs::canonicalize(&redirected).expect("the link resolves");
    assert!(resolved_view.is_dir());
    assert!(resolved_view.starts_with(&checkout));
    assert_eq!(
        resolved_view.file_name().and_then(|name| name.to_str()),
        Some(".temp")
    );
    let resolved_entry = std::fs::symlink_metadata(&resolved_view).expect("resolved metadata");
    assert!(
        !is_redirected_entry(&resolved_entry),
        "the destination looks like an ordinary directory, which is why resolution-only checks fail"
    );

    // The revised validator refuses the original unresolved entry, before using it.
    assert_eq!(
        ensure_scratch_root(&candidate_root),
        Err(ScratchRootError::Redirected)
    );

    // Refusal must not have modified or deleted the link, its target or contents.
    let link_entry = std::fs::symlink_metadata(&redirected).expect("link must survive");
    assert!(
        is_redirected_entry(&link_entry),
        "the redirect itself must still be in place"
    );
    assert!(real_temp.is_dir(), "the target directory must survive");
    assert_eq!(
        std::fs::read(&material).expect("material must survive"),
        b"must survive validation"
    );

    // Scoped cleanup of only this test's own fixture: link first, then its target.
    remove_directory_link(&redirected);
    assert!(!redirected.exists(), "the fixture link is gone");
    assert!(
        real_temp.is_dir(),
        "removing the link must not remove its target"
    );
    std::fs::remove_file(&material).expect("scoped material cleanup");
    std::fs::remove_dir(&real_temp).expect("scoped target .temp cleanup");
    std::fs::remove_dir(&target).expect("scoped target cleanup");
    std::fs::remove_dir(&candidate_root).expect("scoped candidate cleanup");
    assert_eq!(entry_count(&container), 0, "no fixture residue remains");
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
