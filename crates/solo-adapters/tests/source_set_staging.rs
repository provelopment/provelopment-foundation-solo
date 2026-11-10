//! Real-host adversaries for the protected staging store.
//!
//! Every case here drives the actual store on the real filesystem of whichever host runs it: no
//! mock, no in-memory spool and no assertion about intent. What is proved is that
//!
//! * a staged member has **no directory entry at all** while it lives, so nothing can reopen,
//!   replace, truncate or redirect it;
//! * creating an entry with the object's former name inside the namespace does not change one
//!   byte of the member that was already staged;
//! * the store removes only the namespace it created and never touches pre-existing material;
//! * an unprovable staging location is refused instead of followed;
//! * the number of open objects follows the staged size, not the number of members.
//!
//! **Fixture ownership.** Each fixture claims a unique directory with an exclusive `create_dir`,
//! never adopts an existing one, and removes only the tree it created.

use std::fs;
use std::fs::OpenOptions;
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
                "solo-staging-{tag}-{}-{sequence}",
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
    let mut store = root.begin().expect("begin a private staging store");

    let first = b"{\"contentVersion\":\"1.0.0\"}".to_vec();
    let second = vec![0x5a_u8; 4096];
    let first_body = store.stage_body(&first).expect("stage the first body");
    let second_body = store.stage_body(&second).expect("stage the second body");

    // Observed state: the namespace exists and holds no directory entry at all.
    assert_eq!(store.visible_object_entries().expect("list"), 0);
    assert_eq!(
        store.staged_bytes(),
        u64::try_from(first.len() + second.len()).expect("a small fixture total")
    );
    assert_eq!(store.max_member_bytes(), 4096);

    let first_reader = store
        .reader(first_body)
        .expect("a reader for the first body");
    let mut read_back = Vec::new();
    first_reader
        .read_whole(&mut read_back)
        .expect("read the first body");
    assert_eq!(read_back, first);
    assert!(
        first_reader
            .matches(&first)
            .expect("compare the first body")
    );

    let second_reader = store
        .reader(second_body)
        .expect("a reader for the second body");
    assert!(
        second_reader
            .matches(&second)
            .expect("compare the second body")
    );

    // A bounded chunk read starts where it is told to and never past the member's own end.
    let mut chunk = [0_u8; 16];
    let read = second_reader
        .read_chunk(4090, &mut chunk)
        .expect("a bounded tail read");
    assert_eq!(read, 6);
    assert_eq!(&chunk[..6], &[0x5a_u8; 6]);
    assert!(
        second_reader.read_chunk(4097, &mut chunk).is_err(),
        "a read past the member's recorded length must be refused, not clamped"
    );
}

#[test]
fn a_new_entry_with_a_former_object_name_cannot_change_a_staged_member() {
    let fixture = Fixture::new("tamper");
    let root = fixture.root();
    let mut store = root.begin().expect("begin a private staging store");

    let body = vec![0x11_u8; 2048];
    let staged = store.stage_body(&body).expect("stage the body");

    // The namespace name is not part of the public API, so the test finds it the only way an
    // adversary could: by listing the staging root.
    let namespaces = entries(&fixture.staging());
    assert_eq!(namespaces.len(), 1, "exactly one private namespace");
    let namespace = fixture.staging().join(&namespaces[0]);

    // Attempt the classic attack: write and replace the object's former path.
    let former = namespace.join("shard-0.bin");
    fs::write(&former, b"tampered").expect("an adversary may create a new entry");
    assert_eq!(entries(&namespace), vec!["shard-0.bin".to_owned()]);

    // The staged member is unaffected: the new entry is an unrelated object, and the member this
    // store holds was never reachable by that name.
    let reader = store.reader(staged).expect("a reader for the staged body");
    assert!(
        reader.matches(&body).expect("compare the staged body"),
        "the staged bytes must be unchanged by a same-named new entry"
    );
    assert_eq!(
        fs::read(&former).expect("read the adversary's own file"),
        b"tampered".to_vec()
    );
}

#[test]
fn dropping_the_store_removes_only_its_own_namespace() {
    let fixture = Fixture::new("cleanup");
    let root = fixture.root();

    // Pre-existing material this store must never touch.
    let kept = fixture.staging().join("owner-kept");
    fs::create_dir(&kept).expect("create the pre-existing directory");
    fs::write(kept.join("sentinel.txt"), b"keep me").expect("write the sentinel");

    {
        let mut store = root.begin().expect("begin a private staging store");
        store.stage_body(&[0x22_u8; 1024]).expect("stage a body");
        let namespaces = entries(&fixture.staging());
        assert_eq!(
            namespaces.len(),
            2,
            "the store's namespace and the sentinel directory"
        );
    }

    // The namespace is gone, and the pre-existing material is exactly as it was.
    assert_eq!(entries(&fixture.staging()), vec!["owner-kept".to_owned()]);
    assert_eq!(
        fs::read(kept.join("sentinel.txt")).expect("the sentinel still exists"),
        b"keep me".to_vec()
    );
    // The adopted root itself is never removed: it belongs to the caller.
    assert!(fixture.staging().is_dir());
}

#[test]
fn an_unprovable_staging_root_is_refused_instead_of_followed() {
    let fixture = Fixture::new("unprovable");

    // A relative path can never be adopted: it does not name a location that was verified.
    let relative = StagingRootCapability::adopt(Path::new("relative/staging"));
    assert_eq!(
        relative
            .expect_err("a relative staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );

    // A directory that does not exist is refused, never created and never replaced by a system
    // temp fallback.
    let missing = StagingRootCapability::adopt(&fixture.base.join("absent"));
    assert_eq!(
        missing
            .expect_err("a missing staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );

    // A file is not a staging root.
    let file = fixture.base.join("not-a-directory");
    fs::write(&file, b"x").expect("write the fixture file");
    let not_a_dir = StagingRootCapability::adopt(&file);
    assert_eq!(
        not_a_dir
            .expect_err("a file must not be adopted as a staging root")
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
    let link = fixture.base.join("linked-staging");
    std::os::unix::fs::symlink(&real, &link).expect("create the fixture symlink");

    let refused = StagingRootCapability::adopt(&link);
    assert_eq!(
        refused
            .expect_err("a symlinked staging root must be refused")
            .code(),
        "SOURCE_UNSAFE_PATH"
    );
}

#[test]
fn a_staged_object_never_exists_as_a_path_an_adversary_could_open() {
    let fixture = Fixture::new("no-path");
    let root = fixture.root();
    let mut store = root.begin().expect("begin a private staging store");
    let body = vec![0x44_u8; 1024];
    store.stage_body(&body).expect("stage the body");

    let namespaces = entries(&fixture.staging());
    assert_eq!(namespaces.len(), 1, "exactly one private namespace");
    let namespace = fixture.staging().join(&namespaces[0]);

    // The name the object would have had if it were created by path never exists, on either host:
    // Linux creates an `O_TMPFILE` inode and Windows detaches the name immediately. There is
    // therefore no path for a hostile same-principal writer to open, replace, truncate or map.
    for candidate in ["shard-0.bin", "shard-1.bin"] {
        let path = namespace.join(candidate);
        assert!(
            !path.exists(),
            "a staging object must have no directory entry at all"
        );
        assert_eq!(
            OpenOptions::new()
                .write(true)
                .open(&path)
                .err()
                .map(|error| error.kind()),
            Some(std::io::ErrorKind::NotFound),
            "no writable alias may be obtainable by name"
        );
    }

    // The bytes remain available through the retained handle, unchanged.
    let second = store.stage_body(&body).expect("stage a second body");
    let reader = store.reader(second).expect("a reader for the second body");
    assert!(reader.matches(&body).expect("compare the body"));
}

#[test]
fn a_substituted_namespace_is_refused_before_any_body_is_staged() {
    let fixture = Fixture::new("substitution");
    let root = fixture.root();
    let mut store = root.begin().expect("begin a private staging store");

    let namespaces = entries(&fixture.staging());
    assert_eq!(namespaces.len(), 1);
    let namespace = fixture.staging().join(&namespaces[0]);

    // Replace the namespace with a different directory that has the same name: the classic
    // redirection an identity check must catch and a pathname check cannot.
    fs::remove_dir(&namespace).expect("remove the store's own empty namespace");
    fs::create_dir(&namespace).expect("create an impostor namespace");

    let refusal = store
        .stage_body(b"first body")
        .expect_err("a substituted namespace must be refused");
    assert_eq!(refusal.code(), "SOURCE_UNSAFE_PATH");
}

#[test]
fn injected_write_failures_refuse_through_the_production_path() {
    // A write failure at the second member, after the first was staged successfully.
    let fixture = Fixture::new("fault-write");
    let root = fixture.root();
    let mut store = root
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
    let refusal = store
        .stage_body(b"a body that must not be accepted in part")
        .expect_err("a short write must refuse");
    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_IO_ERROR");

    // A flush failure after a full write must also refuse.
    let fixture = Fixture::new("fault-flush");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::Flush))
        .expect("begin with an injected flush failure");
    let refusal = store
        .stage_body(b"a body whose flush fails")
        .expect_err("a flush failure must refuse");
    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_IO_ERROR");
}

#[test]
fn an_injected_finalization_failure_refuses_the_whole_set() {
    let fixture = Fixture::new("fault-finalize");
    let mut store = fixture
        .root()
        .begin_with_faults(StagingFaultsV1::inject(0, StagingFaultV1::Finalize))
        .expect("begin with an injected finalization fault");
    store.stage_body(b"member one").expect("stage a member");

    let refusal = store
        .finalize()
        .expect_err("injected finalization must refuse");
    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_IO_ERROR");
    assert!(
        store.finalize().is_err(),
        "a refused store must not become acceptable on a second attempt"
    );
}

#[test]
fn repeated_bounded_reads_return_the_same_bytes_after_sealing() {
    let fixture = Fixture::new("sealed-reads");
    let root = fixture.root();
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
}

/// The Unix device+inode pair alone cannot distinguish a directory from a successor that reuses
/// its freed inode number, so the namespace-substitution adversary is exercised on Linux and the
/// gap it exposes is recorded rather than asserted away.
#[cfg(unix)]
#[test]
fn replacing_the_staging_root_between_adopt_and_begin_is_refused() {
    let fixture = Fixture::new("root-replacement");
    let root_path = fixture.staging();
    let capability = StagingRootCapability::adopt(&root_path).expect("adopt the staging root");

    // Replace the root itself, keeping its name. The retained handle still refers to the deleted
    // directory, so the only thing that can distinguish them is the identity comparison.
    fs::remove_dir_all(&root_path).expect("remove the adopted root");
    fs::create_dir_all(&root_path).expect("create a replacement root");

    let refusal = match capability.begin() {
        Ok(_) => panic!("a root replaced after adoption must be refused"),
        Err(refusal) => refusal,
    };
    assert_eq!(refusal.code(), "SOURCE_UNSAFE_PATH");
}

#[test]
fn an_unwind_during_staging_removes_only_its_own_namespace() {
    let fixture = Fixture::new("unwind");
    let capability = fixture.root();
    let root_path = fixture.staging();
    fs::create_dir_all(root_path.join("owner-kept")).expect("create an Owner-owned sentinel");

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut store = capability.begin().expect("begin a private staging store");
        store.stage_body(b"partial member").expect("stage a member");
        panic!("simulated abrupt failure inside the staging transaction");
    }));

    assert!(outcome.is_err(), "the transaction must have unwound");
    assert_eq!(
        entries(&root_path),
        vec!["owner-kept".to_owned()],
        "an unwound transaction must remove only the namespace it created"
    );
}

#[cfg(unix)]
#[test]
fn a_real_permission_denial_refuses_as_permission_denied() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = Fixture::new("permission");
    let mut store = fixture
        .root()
        .begin()
        .expect("begin a private staging store");
    let namespaces = entries(&fixture.staging());
    let namespace = fixture.staging().join(&namespaces[0]);

    // A real denial from the host, not a classified error code: the namespace is made read-only
    // for its owner, so creating the staging object fails with EACCES.
    let mut permissions = fs::metadata(&namespace)
        .expect("namespace metadata")
        .permissions();
    permissions.set_mode(0o500);
    fs::set_permissions(&namespace, permissions).expect("make the namespace read-only");

    let refusal = store
        .stage_body(b"denied body")
        .expect_err("a denied staging write must refuse");

    // Restore first so the fixture can always clean up after itself, even on the failing path.
    let mut permissions = fs::metadata(&namespace)
        .expect("namespace metadata")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&namespace, permissions).expect("restore the namespace");

    assert_eq!(refusal.code(), "SOURCE_OBSERVATION_PERMISSION_DENIED");
}

#[test]
fn open_object_count_follows_staged_size_not_member_count() {
    let fixture = Fixture::new("scaling");
    let root = fixture.root();
    let mut store = root.begin().expect("begin a private staging store");

    let body = vec![0x33_u8; 64];
    let mut staged = Vec::new();
    for _ in 0..500 {
        staged.push(store.stage_body(&body).expect("stage a small body"));
    }

    // Five hundred members, one open object: the handle count is a function of the staged size,
    // not of how many representations a manifest declares.
    assert_eq!(store.object_count(), 1);
    assert_eq!(store.visible_object_entries().expect("list"), 0);
    assert_eq!(
        store.staged_bytes(),
        u64::try_from(staged.len() * body.len()).expect("a small fixture total")
    );

    for staged_body in &staged {
        let reader = store.reader(*staged_body).expect("a reader per member");
        assert!(reader.matches(&body).expect("compare every member"));
    }
}
