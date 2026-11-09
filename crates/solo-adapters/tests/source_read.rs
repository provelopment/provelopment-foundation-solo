//! Real-filesystem tests for the capability-confined source reader.
//!
//! Every case creates its own uniquely named directory under the runner's temporary
//! directory, exercises the **real** native code through the public API, and removes what it
//! created. No case touches repository content, an Owner file or any other shared state.
//!
//! Cases that need a link distinguish the hosts honestly: on Linux a symlink is created
//! directly, and on Windows a directory junction is used where the shape allows one because
//! junctions need no privilege, while the file-symlink case needs the runner to hold
//! `SeCreateSymbolicLinkPrivilege` and fails loudly rather than being skipped.

use std::fs;
use std::path::{Path, PathBuf};

use solo_adapters::{ContentRootCapability, RootProvenanceV1, TrustedDirectoryHandle};
use solo_application::{
    CandidateBodyV1, CandidateProbeV1, SourceReadOutcomeV1, SourceReadPort, SourceReadRefusalKind,
    read_declared_source_v1,
};
use solo_domain::{
    SourceCandidateDraftV1, SourceCandidateKind, SourceCandidatePathV1, SourceCandidateSetV1,
};

/// The trust assertion every fixture makes: the operator designated this directory.
const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

/// A task-owned scratch tree with a `content/pages` root, removed on drop.
struct Scratch {
    base: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        let base =
            std::env::temp_dir().join(format!("solo-source-read-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("content").join("pages"))
            .expect("create scratch content root");
        Self { base }
    }

    fn content(&self) -> PathBuf {
        self.base.join("content")
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.content().join(relative)
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        if let Some(parent) = self.path(relative).parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(self.path(relative), bytes).expect("write fixture");
    }

    fn capability(&self) -> ContentRootCapability {
        let capability = ContentRootCapability::adopt_bootstrap_path(&self.content(), TRUSTED)
            .expect("adopt the scratch content root");

        // The trust input is explicit and recorded, never inferred from the path.
        assert_eq!(capability.provenance(), TRUSTED);
        capability
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn locator(path: &str, kind: SourceCandidateKind) -> SourceCandidatePathV1 {
    match SourceCandidatePathV1::parse(path, kind) {
        Ok(value) => value,
        Err(error) => panic!("{path} must be an accepted locator: {error}"),
    }
}

fn declaration(json: Option<&str>, markdown: Option<&str>) -> SourceCandidateSetV1 {
    let mut draft = SourceCandidateDraftV1::new();
    if let Some(path) = json {
        draft
            .json(locator(path, SourceCandidateKind::Json))
            .expect("json declared once");
    }
    if let Some(path) = markdown {
        draft
            .markdown(locator(path, SourceCandidateKind::Markdown))
            .expect("markdown declared once");
    }
    draft.build().expect("a non-empty declaration")
}

fn chosen(outcome: SourceReadOutcomeV1) -> solo_application::ReadSourceCandidateV1 {
    match outcome {
        SourceReadOutcomeV1::Chosen(choice) => choice,
        SourceReadOutcomeV1::Refused(refusal) => {
            panic!("expected a chosen body, got refusal {refusal}")
        }
    }
}

fn refused(outcome: SourceReadOutcomeV1) -> (SourceReadRefusalKind, String) {
    match outcome {
        SourceReadOutcomeV1::Refused(refusal) => (refusal.kind(), refusal.code().to_owned()),
        SourceReadOutcomeV1::Chosen(choice) => {
            panic!("expected a refusal, got the {} body", choice.member())
        }
    }
}

const JSON_BODY: &[u8] = br#"{"contentVersion":"1.0.0"}"#;
const MARKDOWN_BODY: &[u8] = b"# Title\n\nA body.\n";

#[test]
fn a_declared_json_leaf_is_read_with_its_bytes_and_identity() {
    let scratch = Scratch::new("json-leaf");
    scratch.write("pages/main-home.en-001.json", JSON_BODY);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/main-home.en-001.json"), None);

    let choice = chosen(read_declared_source_v1(&capability, &candidates));

    assert_eq!(choice.member(), SourceCandidateKind::Json);
    assert_eq!(choice.logical_path(), "pages/main-home.en-001.json");
    assert_eq!(choice.bytes(), JSON_BODY);
    // The body lives on the same device as the adopted root, and has its own identity.
    assert_eq!(
        choice.identity().container(),
        capability.identity().container()
    );
    assert_ne!(choice.identity(), capability.identity());
    // The mechanism names the native path used, so the report can state it exactly.
    assert!(capability.mechanism().len() > 8);
    assert!(!choice.is_publication_authorized());
    assert!(!choice.is_verified_filesystem_snapshot());
}

#[test]
fn a_declared_markdown_leaf_is_read_when_json_is_confirmed_absent() {
    let scratch = Scratch::new("markdown-leaf");
    scratch.write("pages/only.en-001.md", MARKDOWN_BODY);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/only.en-001.json"), Some("pages/only.en-001.md"));

    let choice = chosen(read_declared_source_v1(&capability, &candidates));

    assert_eq!(choice.member(), SourceCandidateKind::Markdown);
    assert_eq!(choice.bytes(), MARKDOWN_BODY);
}

#[test]
fn an_oversized_markdown_is_never_read_when_json_exists() {
    // 1 MiB is the Markdown bound, so reading this file would be refused. Because JSON wins,
    // a successful outcome proves the Markdown body was never read at all.
    let scratch = Scratch::new("json-wins");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    scratch.write("pages/p.en-001.md", &vec![b'#'; 1024 * 1024 + 16]);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let choice = chosen(read_declared_source_v1(&capability, &candidates));

    assert_eq!(choice.member(), SourceCandidateKind::Json);
    assert_eq!(choice.bytes(), JSON_BODY);
}

#[test]
fn an_oversized_json_is_refused_rather_than_truncated() {
    let scratch = Scratch::new("oversized-json");
    scratch.write("pages/big.en-001.json", &vec![b' '; 2 * 1024 * 1024 + 1]);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/big.en-001.json"), None);

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::UnsafeLocation);
    assert_eq!(code, "SOURCE_UNSAFE_PATH");
}

#[test]
fn an_empty_json_leaf_is_present_with_no_bytes() {
    // Emptiness is a body-validity question for the accepted preflight, not a read refusal:
    // the reader reports the file faithfully.
    let scratch = Scratch::new("empty-json");
    scratch.write("pages/empty.en-001.json", b"");
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/empty.en-001.json"), None);

    let choice = chosen(read_declared_source_v1(&capability, &candidates));

    assert!(choice.bytes().is_empty());
}

#[test]
fn two_declared_members_that_are_one_object_are_refused() {
    let scratch = Scratch::new("hardlink-collision");
    scratch.write("pages/x.en-001.json", JSON_BODY);
    fs::hard_link(
        scratch.path("pages/x.en-001.json"),
        scratch.path("pages/y.en-001.md"),
    )
    .expect("create a hardlink fixture");
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/x.en-001.json"), Some("pages/y.en-001.md"));

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
    assert_eq!(code, "SOURCE_COLLISION");
}

#[test]
fn a_leaf_with_a_second_directory_entry_is_refused() {
    // The declared file itself is fine, but another name references the same object, so
    // ownership of this candidate is ambiguous and it is refused rather than shared.
    let scratch = Scratch::new("hardlinked-leaf");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    fs::hard_link(
        scratch.path("pages/p.en-001.json"),
        scratch.path("pages/elsewhere.json"),
    )
    .expect("create a hardlink fixture");
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let (kind, _code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
}

#[test]
fn a_leaf_that_is_a_link_is_refused() {
    let scratch = Scratch::new("leaf-link");
    scratch.write("pages/real.en-001.json", JSON_BODY);
    create_file_link(
        &scratch.path("pages/real.en-001.json"),
        &scratch.path("pages/link.en-001.json"),
    );
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/link.en-001.json"), None);

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::UnsafeLocation);
    assert_eq!(code, "SOURCE_UNSAFE_PATH");
}

#[test]
fn an_intermediate_directory_that_is_a_link_is_refused() {
    let scratch = Scratch::new("parent-link");
    scratch.write("elsewhere/deep.en-001.json", JSON_BODY);
    create_directory_link(&scratch.path("elsewhere"), &scratch.path("pages/linked"));
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/linked/deep.en-001.json"), None);

    let (kind, _code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::UnsafeLocation);
}

#[test]
fn a_root_that_is_a_link_is_refused() {
    let scratch = Scratch::new("root-link");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    let linked_root = scratch.base.join("content-link");
    create_directory_link(&scratch.content(), &linked_root);

    let adopted = ContentRootCapability::adopt_bootstrap_path(&linked_root, TRUSTED);

    match adopted {
        Ok(_) => panic!("a root that is a link must not be adopted"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

#[test]
fn an_absent_root_is_refused() {
    let scratch = Scratch::new("absent-root");
    let missing = scratch.base.join("does-not-exist");

    match ContentRootCapability::adopt_bootstrap_path(&missing, TRUSTED) {
        Ok(_) => panic!("a missing root must not be adopted"),
        Err(refusal) => assert_ne!(refusal.kind(), SourceReadRefusalKind::NoneEffective),
    }
}

#[test]
fn a_directory_in_place_of_a_leaf_is_refused() {
    let scratch = Scratch::new("directory-leaf");
    fs::create_dir_all(scratch.path("pages/dir.en-001.json")).expect("create a directory fixture");
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/dir.en-001.json"), None);

    let (kind, _code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::UnsafeLocation);
}

#[test]
fn a_missing_intermediate_directory_is_a_confirmed_absence() {
    let scratch = Scratch::new("missing-parent");
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/missing/deep.en-001.json"), None);

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    // A path that does not exist is absence, not a failure: nothing was declared to exist.
    assert_eq!(kind, SourceReadRefusalKind::NoneEffective);
    assert_eq!(code, "SOURCE_NONE_EFFECTIVE");
}

/// A real content-root reader whose declared leaf is changed once, immediately after the
/// probe has reported it.
///
/// This is the only deterministic way to exercise the probe-to-read race against a real
/// filesystem: the mutation happens between the two calls `read_declared_source_v1` makes,
/// so the object the reader opens is not the object the probe announced.
struct ChangedAfterProbe<'a> {
    inner: &'a ContentRootCapability,
    leaf: PathBuf,
    /// The file renamed over the leaf once the probe has reported the original.
    staged: PathBuf,
    changed: std::cell::Cell<bool>,
}

impl SourceReadPort for ChangedAfterProbe<'_> {
    fn probe_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateProbeV1 {
        let probe = self.inner.probe_candidate(member, locator);
        if probe.located().is_some() && !self.changed.get() {
            self.changed.set(true);
            fs::rename(&self.staged, &self.leaf)
                .expect("replace the leaf with another object after its probe");
        }
        probe
    }

    fn read_candidate(
        &self,
        member: SourceCandidateKind,
        locator: &SourceCandidatePathV1,
    ) -> CandidateBodyV1 {
        self.inner.read_candidate(member, locator)
    }
}

// A same-size in-place rewrite is deliberately **not** claimed as a host case. It is only
// observable through the recorded change timestamps, and on Windows those advance on a system
// timer tick, so a rewrite microseconds after the probe can be genuinely indistinguishable from
// it. The rule itself — a body that disagrees with its probe is refused — is proved
// deterministically by the application-level fake-port test, and the always-observable host
// case below proves the refusal on real filesystems of both hosts.

#[test]
fn a_leaf_replaced_by_another_object_between_probe_and_read_is_refused() {
    let scratch = Scratch::new("replaced-leaf");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    // A different object, same size, same name, renamed over the declared leaf after the
    // probe reported the original: a new physical identity, so the read cannot agree.
    let mut replacement = JSON_BODY.to_vec();
    replacement[1] = b'X';
    assert_eq!(replacement.len(), JSON_BODY.len());
    scratch.write("pages/replacement.tmp", &replacement);
    let port = ChangedAfterProbe {
        inner: &capability,
        leaf: scratch.path("pages/p.en-001.json"),
        staged: scratch.path("pages/replacement.tmp"),
        changed: std::cell::Cell::new(false),
    };

    let (kind, code) = refused(read_declared_source_v1(&port, &candidates));

    assert!(
        port.changed.get(),
        "the fixture must actually have replaced the leaf between probe and read"
    );
    assert_eq!(kind, SourceReadRefusalKind::ChangedDuringRead);
    assert_eq!(code, "SNAPSHOT_CHANGED");
}

#[test]
fn a_rewrite_between_reads_is_observed_and_never_served_from_memory() {
    let scratch = Scratch::new("rewrite");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let first = chosen(read_declared_source_v1(&capability, &candidates));

    let rewritten: &[u8] = br#"{"contentVersion":"1.0.0","note":"second"}"#;
    scratch.write("pages/p.en-001.json", rewritten);

    let second = chosen(read_declared_source_v1(&capability, &candidates));

    assert_eq!(first.bytes(), JSON_BODY);
    assert_eq!(second.bytes(), rewritten);
    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[cfg(unix)]
fn create_file_link(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("create a symlink fixture");
}

#[cfg(unix)]
fn create_directory_link(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("create a directory symlink fixture");
}

#[cfg(windows)]
fn create_file_link(target: &Path, link: &Path) {
    // A file symlink needs `SeCreateSymbolicLinkPrivilege`. The hosted Windows runner holds
    // it, and if a host does not, this fails loudly rather than being skipped: an
    // unexercised mandatory case must never look like a pass.
    std::os::windows::fs::symlink_file(target, link).unwrap_or_else(|error| {
        panic!("a leaf symlink fixture is required on this host and could not be created: {error}")
    });
}

#[cfg(windows)]
fn create_directory_link(target: &Path, link: &Path) {
    // A junction is a reparse point that needs no privilege, so the intermediate and root
    // cases exercise this host's reparse refusal without elevation. The two paths travel as
    // environment variables rather than being interpolated into a command line, so a path
    // containing spaces, quotes or parentheses can neither restructure the command nor turn
    // into a differently-named fixture.
    let status = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "New-Item -ItemType Junction -Path $env:SOLO_JUNCTION_LINK \
             -Target $env:SOLO_JUNCTION_TARGET -ErrorAction Stop | Out-Null",
        ])
        .env("SOLO_JUNCTION_LINK", link)
        .env("SOLO_JUNCTION_TARGET", target)
        .status()
        .expect("run PowerShell to create a junction fixture");
    assert!(status.success(), "junction fixture creation failed");

    // Prove the fixture really is a reparse point *before* any refusal is claimed from it: an
    // ordinary directory here would make the following assertion meaningless. The attribute
    // is checked directly, because a junction is a reparse point that `is_symlink` does not
    // necessarily report.
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    let metadata = fs::symlink_metadata(link).expect("the junction fixture exists");
    assert!(
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0,
        "the fixture must be a reparse point, not an ordinary directory"
    );
}

// ---------------------------------------------------------------------------
// R02 §1 — the trusted, already-opened directory handle.
//
// The handle is the object under test. Nothing below hands the reader a path and then
// describes that as a handle transfer.
// ---------------------------------------------------------------------------

#[cfg(unix)]
fn open_directory_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::fd::OwnedFd;
    OwnedFd::from(fs::File::open(path).expect("open the content root as a directory handle"))
}

#[cfg(windows)]
fn open_directory_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::OwnedHandle;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    let file = fs::File::options()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)
        .expect("open the content root as a directory handle");
    OwnedHandle::from(file)
}

#[cfg(unix)]
fn open_file_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::fd::OwnedFd;
    OwnedFd::from(fs::File::open(path).expect("open the leaf as a handle"))
}

#[cfg(windows)]
fn open_file_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::windows::io::OwnedHandle;
    OwnedHandle::from(fs::File::open(path).expect("open the leaf as a handle"))
}

/// Opens a reparse point as itself, so a junction can be presented to the reader unchanged.
#[cfg(windows)]
fn open_reparse_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::OwnedHandle;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let file = fs::File::options()
        .read(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .expect("open the reparse point as a handle");
    OwnedHandle::from(file)
}

#[test]
fn a_transferred_directory_handle_is_verified_and_reads_correctly() {
    let scratch = Scratch::new("trusted-handle");
    scratch.write("pages/p.en-001.json", JSON_BODY);

    // The same directory reached two ways must report one identity: the handle is the
    // authority and the bootstrap path agrees with it.
    let by_path = scratch.capability();
    let by_handle = ContentRootCapability::from_trusted_directory(
        open_directory_handle(&scratch.content()),
        TRUSTED,
    )
    .expect("a real directory handle must be accepted");

    assert_eq!(by_handle.provenance(), TRUSTED);
    assert_eq!(by_handle.identity(), by_path.identity());
    assert!(!by_handle.mechanism().is_empty());

    let candidates = declaration(Some("pages/p.en-001.json"), None);
    let choice = chosen(read_declared_source_v1(&by_handle, &candidates));
    assert_eq!(choice.bytes(), JSON_BODY);
}

#[test]
fn a_transferred_handle_to_a_regular_file_is_refused() {
    let scratch = Scratch::new("trusted-file");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    let file_handle = open_file_handle(&scratch.path("pages/p.en-001.json"));

    match ContentRootCapability::from_trusted_directory(file_handle, TRUSTED) {
        Ok(_) => panic!("a handle naming a regular file must not become a content root"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

#[cfg(windows)]
#[test]
fn a_transferred_handle_to_a_junction_is_refused() {
    let scratch = Scratch::new("trusted-junction");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    let linked = scratch.base.join("content-junction");
    create_directory_link(&scratch.content(), &linked);

    match ContentRootCapability::from_trusted_directory(open_reparse_handle(&linked), TRUSTED) {
        Ok(_) => panic!("a handle naming a reparse point must be refused"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

// ---------------------------------------------------------------------------
// R02 §4 — physical ambiguity in every declared candidate, not just the winner.
// ---------------------------------------------------------------------------

#[test]
fn a_leaf_hardlinked_from_an_unrelated_directory_is_refused() {
    let scratch = Scratch::new("foreign-hardlink");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    // A second directory entry for the same object, created somewhere else entirely.
    fs::create_dir_all(scratch.path("pages/holding")).expect("create the unrelated directory");
    fs::hard_link(
        scratch.path("pages/p.en-001.json"),
        scratch.path("pages/holding/other.json"),
    )
    .expect("create a hardlink from an unrelated directory entry");

    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), None);

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
    assert_eq!(code, "SOURCE_COLLISION");
}

#[test]
fn an_unselected_hardlinked_markdown_is_refused_while_json_would_have_won() {
    let scratch = Scratch::new("unselected-hardlink");
    scratch.write("pages/p.en-001.json", JSON_BODY);
    scratch.write("pages/p.en-001.md", MARKDOWN_BODY);
    // The Markdown the selection would never read is the ambiguous one.
    fs::create_dir_all(scratch.path("pages/holding")).expect("create the unrelated directory");
    fs::hard_link(
        scratch.path("pages/p.en-001.md"),
        scratch.path("pages/holding/copy.md"),
    )
    .expect("create a hardlink to the unselected member");

    let capability = scratch.capability();
    let candidates = declaration(Some("pages/p.en-001.json"), Some("pages/p.en-001.md"));

    let (kind, code) = refused(read_declared_source_v1(&capability, &candidates));

    // The ambiguity belongs to the candidate, not to the selection, so a perfectly good JSON
    // body is not returned either.
    assert_eq!(kind, SourceReadRefusalKind::IdentityCollision);
    assert_eq!(code, "SOURCE_COLLISION");
}

// ---------------------------------------------------------------------------
// R03 stage II — the D2 primitives on the real fixture filesystems.
//
// These run before the product path is changed and prove the kernel contract it will rely
// on: an anonymous `O_TMPFILE | O_EXCL` inode on a reflink-capable filesystem accepts one
// `FICLONE` from a confined source and keeps its bytes after the source is overwritten, while
// the same call on a filesystem without reflink returns a real unsupported error. Neither
// case may be simulated: the fixture roots are supplied by the workflow and a missing root is
// a loud failure, never a quiet fallback to the checkout filesystem.
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod d2_primitives {
    use rustix::fd::OwnedFd;
    use rustix::fs::{Mode, OFlags, SeekFrom};
    use rustix::io::Errno;
    use std::fs;
    use std::path::{Path, PathBuf};

    const SOURCE: &[u8] = b"{\"slug\":\"p\",\"body\":\"original\"}";
    const REWRITTEN: &[u8] = b"{\"slug\":\"p\",\"body\":\"rewritten\"}";

    /// The fixture root must be named explicitly: a D2 proof may never quietly run on a
    /// filesystem that was never verified to support the mechanism.
    fn fixture_root(variable: &str) -> PathBuf {
        match std::env::var_os(variable) {
            Some(value) if !value.is_empty() => PathBuf::from(value),
            _ => panic!(
                "{variable} must name the R03 snapshot fixture filesystem; refusing to run a D2 proof on an unverified filesystem"
            ),
        }
    }

    struct Probe {
        root: PathBuf,
    }

    impl Probe {
        fn new(variable: &str, tag: &str) -> Self {
            let root =
                fixture_root(variable).join(format!("solo-r03-probe-{}-{tag}", std::process::id()));
            let _ = fs::remove_dir_all(&root);
            fs::create_dir(&root).expect("create the probe directory on the fixture filesystem");
            Self { root }
        }
    }

    impl Drop for Probe {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    fn open_directory(path: &Path) -> OwnedFd {
        rustix::fs::open(
            path,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .expect("open the probe directory")
    }

    fn open_regular(path: &Path) -> OwnedFd {
        rustix::fs::open(
            path,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .expect("open the probe source")
    }

    /// The candidate destination: an unnamed, unlinkable inode on the directory's own
    /// filesystem, cloned once from the source. No name is ever created.
    fn clone_into_unnamed(directory: &OwnedFd, source: &OwnedFd) -> Result<OwnedFd, Errno> {
        let destination = rustix::fs::openat(
            directory,
            c".",
            OFlags::TMPFILE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
        )?;
        rustix::fs::ioctl_ficlone(&destination, source)?;
        Ok(destination)
    }

    fn read_from_start(fd: &OwnedFd) -> Vec<u8> {
        rustix::fs::seek(fd, SeekFrom::Start(0)).expect("rewind the clone");
        let mut buffer = vec![0_u8; SOURCE.len() + 64];
        let mut filled = 0_usize;
        while filled < buffer.len() {
            let read = rustix::io::read(fd, &mut buffer[filled..]).expect("read the clone");
            if read == 0 {
                break;
            }
            filled += read;
        }
        buffer.truncate(filled);
        buffer
    }

    #[test]
    fn an_anonymous_clone_on_the_supported_fixture_keeps_its_bytes_after_a_source_rewrite() {
        let probe = Probe::new("SOLO_TEST_SOURCE_FS_ROOT", "supported");
        let source_path = probe.root.join("body.json");
        fs::write(&source_path, SOURCE).expect("write the probe source");

        let directory = open_directory(&probe.root);
        let source = open_regular(&source_path);

        let clone = clone_into_unnamed(&directory, &source)
            .expect("a reflink-capable fixture filesystem must clone an unnamed inode");

        // The clone is its own inode sharing extents with the source.
        let source_stat = rustix::fs::fstat(&source).expect("stat the source");
        let clone_stat = rustix::fs::fstat(&clone).expect("stat the clone");
        assert_ne!(
            source_stat.st_ino, clone_stat.st_ino,
            "the clone must be its own inode"
        );
        assert_eq!(clone_stat.st_size, source_stat.st_size);

        // The source is overwritten in place with bytes of a different length.
        fs::write(&source_path, REWRITTEN).expect("overwrite the probe source in place");

        // The already-cloned bytes do not move: they are one instant of the source.
        assert_eq!(read_from_start(&clone), SOURCE);

        // And no name was ever created for the destination.
        assert_eq!(
            fs::read_dir(&probe.root)
                .expect("list the probe directory")
                .count(),
            1,
            "the snapshot destination must stay unnamed"
        );
    }

    #[test]
    fn a_filesystem_without_reflink_refuses_the_anonymous_clone() {
        let probe = Probe::new("SOLO_TEST_UNSUPPORTED_FS_ROOT", "unsupported");
        let source_path = probe.root.join("body.json");
        fs::write(&source_path, SOURCE).expect("write the probe source");

        let directory = open_directory(&probe.root);
        let source = open_regular(&source_path);

        match clone_into_unnamed(&directory, &source) {
            Ok(_) => panic!(
                "this fixture filesystem accepted a clone, so it cannot prove the unsupported-refusal path"
            ),
            Err(error) => assert!(
                error == Errno::OPNOTSUPP || error == Errno::NOTSUP,
                "expected a real unsupported-filesystem error, got {error:?}"
            ),
        }
    }
}
