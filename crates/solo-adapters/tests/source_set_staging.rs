//! Real-host adversaries for the protected staging store.
//!
//! Every case drives the actual store on the real filesystem of whichever host runs it: no mock, no
//! in-memory spool and no assertion about intent. What is proved is that
//!
//! * a staged member has **no directory entry at all** while it lives, so nothing can reopen,
//!   replace, truncate or redirect it;
//! * the caller's own staging directory gains **no staging-owned entry** on any path, and every
//!   foreign or Owner entry survives byte for byte;
//! * an adopted root that is replaced, unlinked or reached through a symlink or junction is
//!   refused rather than written into;
//! * disposal is bound to the exact object this store owns, so an empty or populated impostor at a
//!   former name survives untouched;
//! * the number of open objects follows the staged size, not the number of members.
//!
//! **Fixture ownership.** Each fixture claims a unique directory with an exclusive `create_dir`,
//! never adopts an existing one, and removes only the tree it created.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use solo_adapters::{StagingFaultV1, StagingFaultsV1, StagingRootCapability};

/// Distinguishes fixtures inside one test process.
static FIXTURE_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// A task-owned directory tree, removed on drop.
struct Fixture {
    base: PathBuf,
}

impl Fixture {
    /// Claims a unique fixture directory beneath the host temp directory.
    fn new(tag: &str) -> Self {
        for _ in 0..16 {
            let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let base = std::env::temp_dir().join(format!(
                "solo-wo051-{tag}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&base) {
                Ok(()) => return Self { base },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("a staging fixture directory could not be created: {error}"),
            }
        }
        panic!("fixture names were already taken; nothing was removed and nothing was reused")
    }

    /// The staging root the store is given.
    fn staging(&self) -> PathBuf {
        self.base.join("staging")
    }

    /// Creates the staging root and returns its adopted capability.
    fn root(&self) -> StagingRootCapability {
        fs::create_dir_all(self.staging()).expect("create the fixture staging root");
        StagingRootCapability::adopt(&self.staging()).expect("adopt the fixture staging root")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Only ever removes a tree this fixture created exclusively.
        let _ = fs::remove_dir_all(&self.base);
    }
}

/// The names visible inside one directory, sorted for a stable comparison.
fn entries(path: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(path)
        .expect("list the fixture directory")
        .map(|entry| {
            entry
                .expect("read a fixture directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    names.sort();
    names
}
#[test]
fn a_staged_member_is_nameless_and_reads_back_exactly() {
    let fixture = Fixture::new("round-trip");
    let root = fixture.root();
    let before = entries(&fixture.staging());
    let mut store = root.begin().expect("begin a private staging store");

    let first = vec![0x41_u8; 4096];
    let second = b"the second staged member".to_vec();
    let first_body = store.stage_body(&first).expect("stage the first member");
    let second_body = store.stage_body(&second).expect("stage the second member");

    let mut read = Vec::new();
    store
        .reader(first_body)
        .expect("a reader for the first member")
        .read_whole(&mut read)
        .expect("read the first member");
    assert_eq!(read, first);
    let mut read = Vec::new();
    store
        .reader(second_body)
        .expect("a reader for the second member")
        .read_whole(&mut read)
        .expect("read the second member");
    assert_eq!(read, second);

    store.finalize().expect("seal the store");

    // Linux links nothing at all. On Windows the task-owned shard names may still be enumerable as
    // delete-pending entries while the handles that keep the members readable are open, so entry
    // equality is asserted there only after the last handle closes. Bytes stay readable here.
    #[cfg(target_os = "linux")]
    assert_eq!(
        entries(&fixture.staging()),
        before,
        "staging must not create an entry in the adopted root"
    );
    let mut read_after_sealing = Vec::new();
    store
        .reader(first_body)
        .expect("a reader after finalization")
        .read_whole(&mut read_after_sealing)
        .expect("read the first member after finalization");
    assert_eq!(read_after_sealing, first);

    drop(store);
    assert_eq!(
        entries(&fixture.staging()),
        before,
        "cleanup must not leave or delete an entry in the adopted root"
    );
}

#[test]
fn staging_root_adoption_refuses_unsafe_shapes() {
    let fixture = Fixture::new("adopt");
    fs::create_dir_all(fixture.staging()).expect("create the fixture area");

    assert_eq!(
        StagingRootCapability::adopt(Path::new("relative/staging"))
            .expect_err("a relative staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );
    assert_eq!(
        StagingRootCapability::adopt(&fixture.base.join("absent"))
            .expect_err("a missing staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );

    let file = fixture.base.join("not-a-directory");
    fs::write(&file, b"x").expect("write the fixture file");
    assert_eq!(
        StagingRootCapability::adopt(&file)
            .expect_err("a file must not be adopted as a staging root")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );

    // A parent-traversal component is refused before any resolution.
    assert_eq!(
        StagingRootCapability::adopt(&fixture.base.join("..").join("solo-wo051-escape"))
            .expect_err("a traversal component must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );
}

#[cfg(unix)]
#[test]
fn a_symlinked_staging_root_is_refused_rather_than_followed() {
    let fixture = Fixture::new("symlink");
    let real = fixture.base.join("real-staging");
    fs::create_dir(&real).expect("create the real staging directory");
    let inner = real.join("inner");
    fs::create_dir(&inner).expect("create the inner directory");
    let link = fixture.base.join("linked-staging");
    std::os::unix::fs::symlink(&real, &link).expect("create the fixture symlink");

    assert_eq!(
        StagingRootCapability::adopt(&link)
            .expect_err("a symlinked staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );
    // An intermediate symlink component is refused by the kernel, not merely the final one.
    assert_eq!(
        StagingRootCapability::adopt(&link.join("inner"))
            .expect_err("an intermediate symlink component must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn no_staging_owned_entry_ever_appears_in_the_retained_root() {
    let fixture = Fixture::new("zero-entry");
    let root = fixture.root();
    let staging = fixture.staging();
    fs::create_dir(staging.join("owner-kept")).expect("create an Owner-owned sentinel");
    fs::write(
        staging.join("owner-kept").join("note.md"),
        b"Owner material",
    )
    .expect("write Owner material");
    let before = entries(&staging);

    let mut store = root.begin().expect("begin a private staging store");
    let body = vec![0x5a_u8; 8192];
    let staged = store.stage_body(&body).expect("stage a member");
    // Observed *during* staging, not only afterwards.
    assert_eq!(entries(&staging), before, "a staged write created a name");

    let mut read = Vec::new();
    store
        .reader(staged)
        .expect("a reader")
        .read_whole(&mut read)
        .expect("read the member");
    assert_eq!(read, body);
    store.finalize().expect("seal the store");
    assert_eq!(entries(&staging), before, "finalize created a name");
    drop(store);
    assert_eq!(entries(&staging), before, "drop created or removed a name");
    assert_eq!(
        fs::read(staging.join("owner-kept").join("note.md")).expect("Owner material"),
        b"Owner material"
    );
}
#[cfg(target_os = "linux")]
#[test]
fn a_retained_root_that_was_unlinked_cannot_receive_a_staged_byte() {
    let fixture = Fixture::new("root-unlink");
    let capability = fixture.root();
    let staging = fixture.staging();

    // The root is removed and recreated at the same name: the store still holds the old
    // descriptor, so the replacement must never receive a staged byte.
    fs::remove_dir(&staging).expect("remove the adopted root");
    fs::create_dir(&staging).expect("create a replacement root");
    let before = entries(&staging);

    let mut store = capability.begin().expect("begin a private staging store");
    let refusal = store
        .stage_body(b"must not land anywhere")
        .expect_err("an unlinked root must refuse a staged write");
    assert_eq!(refusal.code(), "SOURCE_UNSAFE_PATH");
    assert_eq!(
        entries(&staging),
        before,
        "a refused stage must not touch the replacement root"
    );
}

#[test]
fn injected_write_failures_refuse_through_the_production_path() {
    // A write failure at the second member, after the first was staged successfully.
    let fixture = Fixture::new("fault-write");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(1, StagingFaultV1::Write))
        .expect("begin with an injected write fault");
    assert!(store.stage_body(b"member one").is_ok());
    let refusal = store
        .stage_body(b"member two")
        .expect_err("the injected write failure must refuse");
    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_IO_ERROR");
    assert_ne!(refusal.code(), "SOURCE_NONE_EFFECTIVE");

    // A short write must be refused rather than accepted as a shorter member.
    let fixture = Fixture::new("fault-short");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::ShortWrite))
        .expect("begin with an injected short write");
    assert_eq!(
        store
            .stage_body(b"a body that must not be accepted in part")
            .expect_err("a short write must refuse")
            .code(),
        "SOURCE_OBSERVATION_IO_ERROR"
    );

    // A flush failure after a full write must also refuse.
    let fixture = Fixture::new("fault-flush");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::Flush))
        .expect("begin with an injected flush failure");
    assert_eq!(
        store
            .stage_body(b"a body whose flush fails")
            .expect_err("a flush failure must refuse")
            .code(),
        "SOURCE_OBSERVATION_IO_ERROR"
    );
}

#[test]
fn an_injected_finalization_failure_refuses_the_whole_set() {
    let fixture = Fixture::new("fault-finalize");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::Finalize))
        .expect("begin with an injected finalization fault");
    store.stage_body(b"member one").expect("stage a member");

    assert_eq!(
        store
            .finalize()
            .expect_err("injected finalization must refuse")
            .code(),
        "SOURCE_OBSERVATION_IO_ERROR"
    );
    assert!(
        store.finalize().is_err(),
        "a refused store must not become acceptable on a second attempt"
    );
}
#[test]
fn repeated_bounded_reads_return_the_same_bytes_after_sealing() {
    let fixture = Fixture::new("sealed-reads");
    let root = fixture.root();
    let before = entries(&fixture.staging());
    let mut store = root.begin().expect("begin a private staging store");

    let first = vec![0x77_u8; 4096];
    let second = vec![0x88_u8; 4096];
    let first_body = store.stage_body(&first).expect("stage the first body");
    let second_body = store.stage_body(&second).expect("stage the second body");
    store.finalize().expect("seal the store");

    for _ in 0..3 {
        let mut read = Vec::new();
        store
            .reader(first_body)
            .expect("reader")
            .read_whole(&mut read)
            .expect("read the first body");
        assert_eq!(read, first);
        let mut read = Vec::new();
        store
            .reader(second_body)
            .expect("reader")
            .read_whole(&mut read)
            .expect("read the second body");
        assert_eq!(read, second);
    }

    // A sealed store refuses further staging rather than accepting a late member.
    assert_eq!(
        store
            .stage_body(b"late")
            .expect_err("a sealed store must refuse")
            .code(),
        "SOURCE_UNSUPPORTED"
    );
    // Linux never links a name. Windows may still enumerate its own delete-pending shard names
    // while the readable handles are open, so equality is required there after the last close.
    #[cfg(target_os = "linux")]
    assert_eq!(
        entries(&fixture.staging()),
        before,
        "the sealed read lifetime must not create a name"
    );
    drop(store);
    assert_eq!(
        entries(&fixture.staging()),
        before,
        "releasing the sealed store must leave the adopted root as it was"
    );
}

#[test]
fn an_unwind_during_staging_touches_only_its_own_objects() {
    let fixture = Fixture::new("unwind");
    let capability = fixture.root();
    let root_path = fixture.staging();
    fs::create_dir_all(root_path.join("owner-kept")).expect("create an Owner-owned sentinel");
    let before = entries(&root_path);

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut store = capability.begin().expect("begin a private staging store");
        store.stage_body(b"partial member").expect("stage a member");
        panic!("simulated abrupt failure inside the staging transaction");
    }));

    assert!(outcome.is_err(), "the transaction must have unwound");
    assert_eq!(
        entries(&root_path),
        before,
        "an unwound transaction must leave the adopted root exactly as it was"
    );
}

#[cfg(unix)]
#[test]
fn a_real_permission_denial_refuses_as_permission_denied() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = Fixture::new("permission");
    let capability = fixture.root();
    let mut store = capability.begin().expect("begin a private staging store");

    // A real denial from the host, not a classified error code: the adopted root is made
    // read-only for its owner, so creating an anonymous inode fails with EACCES.
    let mut permissions = fs::metadata(fixture.staging())
        .expect("root metadata")
        .permissions();
    permissions.set_mode(0o500);
    fs::set_permissions(fixture.staging(), permissions).expect("make the root read-only");

    let refusal = store
        .stage_body(b"denied body")
        .expect_err("a denied staging write must refuse");

    // Restore first so the fixture can always clean up after itself, even on the failing path.
    let mut permissions = fs::metadata(fixture.staging())
        .expect("root metadata")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(fixture.staging(), permissions).expect("restore the root");

    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_PERMISSION_DENIED");
}

#[test]
fn open_object_count_follows_staged_size_not_member_count() {
    let fixture = Fixture::new("scaling");
    let root = fixture.root();
    let before = entries(&fixture.staging());
    let mut store = root.begin().expect("begin a private staging store");

    let body = vec![0x33_u8; 64];
    let mut staged = Vec::new();
    for _ in 0..500 {
        staged.push(store.stage_body(&body).expect("stage a small body"));
    }

    // Five hundred members, one open object: the handle count is a function of the staged size,
    // not of how many representations a manifest declares.
    assert_eq!(store.object_count(), 1);
    assert_eq!(
        store.staged_bytes(),
        u64::try_from(staged.len() * body.len()).expect("a small fixture total")
    );
    // Linux creates no namespace at all; Windows owns a temporary one that must already hold no
    // shard name while the transaction is still open.
    #[cfg(target_os = "linux")]
    assert_eq!(
        entries(&fixture.staging()),
        before,
        "no staging-owned entry may appear in the adopted root"
    );

    #[cfg(windows)]
    {
        // Windows may still enumerate its own delete-pending shard names while the retained handles
        // keep the members readable. Every extra entry must be task-owned and must not be openable,
        // and no foreign or Owner entry may be added, removed or altered.
        let mut extra: Vec<String> = entries(&fixture.staging())
            .into_iter()
            .filter(|name| !before.contains(name))
            .collect();
        extra.sort();
        assert_eq!(
            extra.len(),
            store.object_count(),
            "only one pending name per staged object may appear"
        );
        for name in &extra {
            assert!(
                name.starts_with("shard-"),
                "a staging-owned name must carry the module's prefix, saw {name}"
            );
            assert!(
                std::fs::File::open(fixture.staging().join(name)).is_err(),
                "a pending shard name must not be openable by a second opener"
            );
        }
    }

    for staged_body in &staged {
        assert!(
            store
                .reader(*staged_body)
                .expect("a reader per member")
                .matches(&body)
                .expect("compare every member")
        );
    }
}
/// The filesystem type of the mount holding `path`, read from the kernel's own mount table.
#[cfg(target_os = "linux")]
fn filesystem_type(path: &Path) -> Option<String> {
    let canonical = fs::canonicalize(path).ok()?;
    let table = fs::read_to_string("/proc/self/mountinfo").ok()?;
    let mut best: Option<(usize, String)> = None;
    for line in table.lines() {
        let Some((left, right)) = line.split_once(" - ") else {
            continue;
        };
        let fields: Vec<&str> = left.split_whitespace().collect();
        if fields.len() < 5 {
            continue;
        }
        let mount_point = fields[4];
        if !canonical.starts_with(mount_point) {
            continue;
        }
        let fstype = right.split_whitespace().next().unwrap_or_default();
        let length = mount_point.len();
        if best.as_ref().is_none_or(|(seen, _)| length >= *seen) {
            best = Some((length, fstype.to_owned()));
        }
    }
    best.map(|(_, fstype)| fstype)
}

/// The mandatory real-Btrfs proof.
///
/// It consumes the fixture the existing workflow already mounts and exports. A missing fixture is a
/// **failure** in CI and only a reported local limitation outside it; it is never silently skipped
/// where the proof is required.
#[cfg(target_os = "linux")]
#[test]
fn the_mandatory_btrfs_fixture_stages_anonymous_objects() {
    let Some(fixture_root) = std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") else {
        if std::env::var_os("CI").is_some() {
            panic!(
                "SOLO_TEST_SOURCE_FS_ROOT must name the CI Btrfs fixture; this adversary may not be \
                 skipped in CI"
            );
        }
        eprintln!("btrfs fixture absent in this local environment: host-specific case not run");
        return;
    };

    let base = PathBuf::from(fixture_root).join(format!(
        "solo-wo051-btrfs-{}-{}",
        std::process::id(),
        FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&base).expect("create the task-owned Btrfs child");
    assert_eq!(
        filesystem_type(&base).as_deref(),
        Some("btrfs"),
        "SOLO_TEST_SOURCE_FS_ROOT must be on a real Btrfs mount"
    );

    let before = entries(&base);
    let capability = StagingRootCapability::adopt(&base).expect("adopt the Btrfs child");
    let mut store = capability.begin().expect("begin a private staging store");
    let body = vec![0x5a_u8; 32 * 1024];
    let staged = store
        .stage_body(&body)
        .expect("an O_TMPFILE shard must be creatable on Btrfs");
    assert_eq!(entries(&base), before, "Btrfs gained a staging-owned entry");

    store.finalize().expect("seal the store");
    let mut read = Vec::new();
    store
        .reader(staged)
        .expect("a reader")
        .read_whole(&mut read)
        .expect("read the staged member on Btrfs");
    assert_eq!(read, body);
    drop(store);
    assert_eq!(entries(&base), before, "Btrfs cleanup touched a name");
    fs::remove_dir(&base).expect("remove the task-owned Btrfs child");
}
/// An impostor installed at the namespace's own former name must survive untouched.
///
/// Both the empty and the populated case are exercised: the store may only ever dispose of the
/// object behind its retained handle, never the entry a name currently resolves to.
#[cfg(windows)]
#[test]
fn an_impostor_at_the_namespace_name_survives_untouched() {
    for populated in [false, true] {
        let fixture = Fixture::new(if populated {
            "impostor-full"
        } else {
            "impostor-empty"
        });
        let capability = fixture.root();
        let mut store = capability.begin().expect("begin a private staging store");
        // A pre-existing directory at the name a prior namespace mechanism used, plus a foreign
        // sentinel. This design creates no directory at all, so neither may be touched.
        let namespace = fixture.staging().join("solo-source-stage-impostor");
        fs::create_dir(&namespace).expect("create the impostor directory");
        if populated {
            fs::write(namespace.join("impostor.txt"), b"not staged")
                .expect("populate the impostor");
        }
        let sentinel = fixture.staging().join("owner-sentinel.txt");
        fs::write(&sentinel, b"owner").expect("write a foreign sentinel");
        let before = entries(&namespace);

        // The transaction must succeed beside them, and the impostor must never receive a byte.
        let staged = store
            .stage_body(b"adversary round")
            .expect("stage beside an impostor");
        assert!(
            store
                .reader(staged)
                .expect("a reader")
                .matches(b"adversary round")
                .expect("compare")
        );
        assert_eq!(
            entries(&namespace),
            before,
            "an impostor must never receive a staged member"
        );

        drop(store);
        assert!(namespace.exists(), "an impostor must survive store cleanup");
        assert_eq!(
            entries(&namespace),
            before,
            "an impostor must be untouched after cleanup"
        );
        if populated {
            assert_eq!(
                fs::read(namespace.join("impostor.txt")).expect("impostor content"),
                b"not staged"
            );
        }
    }
}

/// An intermediate junction must be refused, or its creation must be provably denied.
#[cfg(windows)]
#[test]
fn an_intermediate_junction_component_is_refused() {
    let fixture = Fixture::new("junction");
    let real = fixture.base.join("real");
    fs::create_dir_all(real.join("child")).expect("create the real tree");
    let junction = fixture.base.join("junction");

    let created = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(&junction)
        .arg(&real)
        .output()
        .expect("run mklink for the junction adversary");

    if !created.status.success() {
        let stderr = String::from_utf8_lossy(&created.stderr);
        // Only the exact applicable privilege denial is an honest UNVERIFIED; anything else fails.
        assert!(
            stderr.contains("1314") || stderr.to_ascii_lowercase().contains("privilege"),
            "junction creation failed for a non-privilege reason: {stderr}"
        );
        eprintln!(
            "intermediate junction adversary UNVERIFIED: host privilege denied ({})",
            stderr.trim()
        );
        return;
    }

    assert!(
        fs::metadata(&junction).expect("junction metadata").is_dir(),
        "the created junction must resolve to a directory for the adversary to be real"
    );
    let refusal = StagingRootCapability::adopt(&junction.join("child"))
        .expect_err("an intermediate junction component must be refused");
    assert_eq!(refusal.code(), "SOURCE_UNSAFE_PATH");
    eprintln!("intermediate junction adversary executed: adoption refused the redirect");
}

/// The environment variable that turns this test body into the aborting child.
const ABORT_CHILD_ROOT: &str = "SOLO_WO051_ABORT_CHILD_ROOT";

/// A bounded child that stages real bytes and then dies without unwinding.
#[test]
fn a_bounded_abort_child_leaves_no_recoverable_stage_plaintext() {
    if let Some(root) = std::env::var_os(ABORT_CHILD_ROOT) {
        let root = PathBuf::from(root);
        let capability = StagingRootCapability::adopt(&root).expect("adopt in the child");
        let mut store = capability.begin().expect("begin in the child");
        store
            .stage_body(&vec![0xa5_u8; 16 * 1024])
            .expect("stage in the child");
        fs::write(root.join("child-staged.marker"), b"staged").expect("write the child marker");
        // No unwinding, no destructors: the host must clean up what the process owned.
        std::process::abort();
    }

    let fixture = Fixture::new("abort");
    let staging = fixture.staging();
    fs::create_dir_all(&staging).expect("create the fixture root");
    let status = std::process::Command::new(std::env::current_exe().expect("current exe"))
        .args([
            "--exact",
            "a_bounded_abort_child_leaves_no_recoverable_stage_plaintext",
            "--nocapture",
        ])
        .env(ABORT_CHILD_ROOT, &staging)
        .status()
        .expect("run the abort child");

    assert!(!status.success(), "the child must have terminated abruptly");
    assert!(
        staging.join("child-staged.marker").exists(),
        "the child must really have staged a member before dying"
    );

    // Nothing recoverable survives: only the marker, and any directory the host left behind is
    // empty rather than holding an orphaned shard.
    for name in entries(&staging) {
        if name == "child-staged.marker" {
            continue;
        }
        let path = staging.join(&name);
        assert!(
            path.is_dir(),
            "the aborted transaction left a recoverable entry: {name}"
        );
        assert!(
            entries(&path).is_empty(),
            "the aborted transaction left content inside {name}"
        );
    }
}
#[cfg(windows)]
#[test]
fn a_pending_shard_name_cannot_be_reopened_changed_renamed_or_mapped() {
    let fixture = Fixture::new("pending-attacker");
    let root = fixture.root();
    let staging = fixture.staging();
    let before = entries(&staging);
    let mut store = root.begin().expect("begin a private staging store");
    let body = vec![0x5a_u8; 8192];
    let staged = store.stage_body(&body).expect("stage a member");

    // The name is not part of the API, so the test derives it the only way an attacker could: by
    // listing the caller's own directory after the store wrote. It is the actual pending name.
    let mut observed: Vec<String> = entries(&staging)
        .into_iter()
        .filter(|name| !before.contains(name))
        .collect();
    observed.sort();
    assert_eq!(
        observed.len(),
        1,
        "exactly one pending name is expected while the handle is live"
    );
    let name = observed.remove(0);
    assert!(name.starts_with("shard-"), "unexpected staging name {name}");
    let pending = staging.join(&name);
    let alias = staging.join("attacker-alias.bin");

    // Every reach an attacker has, attempted for real against the observed name.
    let read = fs::File::open(&pending).is_ok();
    let write = fs::OpenOptions::new().write(true).open(&pending).is_ok();
    // A writable mapped view needs a writable file handle first, so acquiring one IS the attack.
    let mapping_handle = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&pending)
        .is_ok();
    let deleted = fs::remove_file(&pending).is_ok();
    let renamed = fs::rename(&pending, &alias).is_ok();
    assert!(
        !read && !write && !mapping_handle && !deleted && !renamed,
        "a pending shard must be unreachable: read={read} write={write} mapped={mapping_handle} delete={deleted} rename={renamed}"
    );
    assert!(
        !alias.exists(),
        "a renamed reachable alias must never exist"
    );

    // The bytes are unchanged and still readable through the legitimate retained handle.
    let mut read_back = Vec::new();
    store
        .reader(staged)
        .expect("a reader")
        .read_whole(&mut read_back)
        .expect("read the member");
    assert_eq!(
        read_back, body,
        "refused attacks must not change a staged byte"
    );

    store.finalize().expect("seal the store");
    assert!(
        store
            .reader(staged)
            .expect("a sealed reader")
            .matches(&body)
            .expect("compare")
    );

    drop(store);
    assert!(
        !pending.exists(),
        "the pending name must disappear when its last handle closes"
    );
    assert_eq!(
        entries(&staging),
        before,
        "the adopted root must be exactly as it was after release"
    );
}

#[cfg(windows)]
#[test]
fn a_root_moved_and_replaced_at_its_path_never_receives_a_staged_byte() {
    let fixture = Fixture::new("root-swap");
    let root = fixture.root();
    let staging = fixture.staging();
    let moved = fixture.base.join("moved-root");
    let mut store = root.begin().expect("begin a private staging store");

    // Move the adopted directory aside and put a fresh impostor at the same pathname.
    if fs::rename(&staging, &moved).is_err() {
        // The host refused to move a directory this store holds: that refusal is itself the
        // confinement evidence, and the impostor case cannot be constructed on this host.
        assert!(
            !moved.exists(),
            "a refused move must leave the directory where it was"
        );
        drop(store);
        return;
    }
    fs::create_dir_all(&staging).expect("create the impostor root");
    let before_impostor = entries(&staging);

    let body = b"adversary round".to_vec();
    if let Ok(staged) = store.stage_body(&body) {
        let mut read = Vec::new();
        store
            .reader(staged)
            .expect("a reader")
            .read_whole(&mut read)
            .expect("read the member");
        assert_eq!(read, body, "a staged member must read back exactly");
    }
    assert_eq!(
        entries(&staging),
        before_impostor,
        "an impostor root must never receive a staged member"
    );

    drop(store);
    assert_eq!(
        entries(&staging),
        before_impostor,
        "cleanup must not create or remove anything in the impostor root"
    );
    assert!(moved.exists(), "the moved directory must never be deleted");
}

#[cfg(windows)]
#[test]
fn a_preexisting_file_at_a_shard_name_is_refused_and_left_byte_identical() {
    let fixture = Fixture::new("name-collision");
    let root = fixture.root();
    let staging = fixture.staging();
    let foreign = staging.join("shard-0-collision.bin");
    fs::write(&foreign, b"owner-bytes").expect("write a foreign file at the shard name");

    // The production native `FILE_CREATE` must refuse rather than adopt the existing object.
    let refusal = root
        .create_shard_at("shard-0-collision.bin")
        .expect_err("a pre-existing shard name must be refused");
    assert!(
        matches!(
            refusal.code(),
            "SOURCE_UNSAFE_PATH"
                | "SOURCE_OBSERVATION_IO_ERROR"
                | "SOURCE_OBSERVATION_PERMISSION_DENIED"
        ),
        "unexpected refusal {}",
        refusal.code()
    );
    assert_eq!(
        fs::read(&foreign).expect("read the foreign file"),
        b"owner-bytes",
        "a foreign object must remain byte-identical"
    );
    assert_eq!(entries(&staging), vec!["shard-0-collision.bin".to_owned()]);
}

#[cfg(windows)]
#[test]
fn a_host_denied_disposition_refuses_before_any_plaintext_is_written() {
    let fixture = Fixture::new("disposition-denied");
    let root = fixture.root();
    let staging = fixture.staging();
    let before = entries(&staging);
    let mut store = root
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::Disposition))
        .expect("begin a store with a denied disposition");

    // The seam creates the object without `DELETE`, so this is the host denying the real
    // `FileDispositionInfoEx` call, not a simulated value.
    let refusal = store
        .stage_body(b"synthetic-body")
        .expect_err("a denied disposition must refuse the member");
    assert!(
        matches!(
            refusal.code(),
            "SOURCE_OBSERVATION_IO_ERROR"
                | "SOURCE_OBSERVATION_PERMISSION_DENIED"
                | "SOURCE_UNSAFE_PATH"
        ),
        "unexpected refusal {}",
        refusal.code()
    );
    // Nothing was staged and no member exists to read.
    assert_eq!(
        store.object_count(),
        0,
        "a refused shard must not be retained"
    );
    assert_eq!(store.staged_bytes(), 0, "no plaintext byte may be counted");

    drop(store);
    assert_eq!(
        entries(&staging),
        before,
        "the empty owned file must be removed by closing its own handle"
    );
}
