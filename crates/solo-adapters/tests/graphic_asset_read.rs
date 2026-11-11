//! Real-filesystem tests for the `R03.C2` confined Entity asset reader.
//!
//! Every case creates its own uniquely named directory under the runner's temporary directory,
//! drives the **real** native boundary through the public application trait, and removes what it
//! created. No case touches repository content, an Owner file or any shared state, and no tracked
//! artwork is used: the fixtures are disposable byte strings.
//!
//! Host differences are stated honestly rather than skipped. On Linux the fixture filesystem is
//! the R03 snapshot filesystem, because the reader takes an immutable clone and a filesystem
//! without reflink could only refuse; a missing variable is a loud failure. A file symlink needs
//! `SeCreateSymbolicLinkPrivilege` on Windows and the case fails loudly if the runner cannot
//! create one, while a directory junction needs no privilege and is used for the intermediate
//! reparse case.

use std::fs;
use std::path::{Path, PathBuf};

use solo_adapters::{ContentRootCapability, RootProvenanceV1, TrustedDirectoryHandle};
use solo_application::{
    CandidateBodyV1, EntityAssetSourceV1, GraphicAssetReadPort, GraphicAssetReadRefusalKind,
    MAX_GRAPHIC_SOURCE_BYTES, PhysicalIdentityV1, SourceReadPort, SourceReadRefusalKind,
};
use solo_domain::{AssetNamespace, AssetReferenceV1, SourceCandidateKind, SourceCandidatePathV1};

/// The trust assertion every fixture makes: the operator designated this directory.
const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

/// A task-owned scratch tree with a `content/assets` root, removed on drop.
struct Scratch {
    base: PathBuf,
}

/// The directory a scratch tree is created in.
///
/// On Linux every fixture is created on the R03 snapshot fixture filesystem, because the reader
/// takes an immutable clone and a filesystem without reflink could only produce a refusal. The
/// variable is therefore required on that host: a missing value is a loud failure, never a quiet
/// run on an unverified filesystem. Other hosts keep the runner temp directory.
fn scratch_parent() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
            Some(root) if !root.is_empty() => PathBuf::from(root),
            _ => panic!(
                "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; refusing to run the real-filesystem suite on a filesystem that was never verified to support an immutable clone"
            ),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::env::temp_dir()
    }
}

impl Scratch {
    fn new(label: &str) -> Self {
        Self::on(scratch_parent(), label)
    }

    /// A scratch tree created inside an explicitly chosen test-owned parent.
    ///
    /// The Linux non-reflink proof needs a directory on the workflow's `tmpfs` fixture, while
    /// every other case stays on the verified snapshot filesystem.
    fn on(parent: PathBuf, label: &str) -> Self {
        let base = parent.join(format!("solo-asset-read-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("content").join("assets")).expect("create scratch asset root");
        Self { base }
    }

    fn without_assets(label: &str) -> Self {
        let base = scratch_parent().join(format!("solo-asset-read-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("content")).expect("create bare content root");
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
        assert_eq!(capability.provenance(), TRUSTED);
        capability
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn entity_reference(reference: &str) -> AssetReferenceV1 {
    match AssetReferenceV1::parse(AssetNamespace::Entity, reference) {
        Ok(value) => value,
        Err(error) => panic!("{reference} must be an accepted entity reference: {error}"),
    }
}

fn read(capability: &ContentRootCapability, reference: &str) -> EntityAssetSourceV1 {
    capability.read_entity_asset(&entity_reference(reference), MAX_GRAPHIC_SOURCE_BYTES)
}

fn present(outcome: EntityAssetSourceV1) -> (Vec<u8>, PhysicalIdentityV1, u64, u64) {
    match outcome {
        EntityAssetSourceV1::Present {
            bytes,
            identity,
            length,
            links,
            ..
        } => (bytes, identity, length, links),
        EntityAssetSourceV1::Refused(refusal) => {
            panic!("expected confined source bytes, got refusal {refusal}")
        }
    }
}

fn refused(outcome: EntityAssetSourceV1) -> GraphicAssetReadRefusalKind {
    match outcome {
        EntityAssetSourceV1::Refused(refusal) => refusal.kind(),
        EntityAssetSourceV1::Present { bytes, .. } => {
            panic!("expected a refusal, got {} acquired bytes", bytes.len())
        }
    }
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
    // A file symlink needs `SeCreateSymbolicLinkPrivilege`. The hosted Windows runner holds it,
    // and a host that does not fails loudly: an unexercised mandatory case must never look like
    // a pass.
    std::os::windows::fs::symlink_file(target, link).unwrap_or_else(|error| {
        panic!("a leaf symlink fixture is required on this host and could not be created: {error}")
    });
}

#[cfg(windows)]
fn create_directory_link(target: &Path, link: &Path) {
    // A junction needs no privilege and is therefore the honest intermediate-reparse fixture.
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

    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
    let metadata = fs::symlink_metadata(link).expect("the junction fixture exists");
    assert!(
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0,
        "the fixture must be a reparse point, not an ordinary directory"
    );
}

/// `R03C2-08` — a nested `assets/` file beneath an explicitly adopted `content/` capability is
/// read with exactly its bytes, its own reported length, a single link and a stable identity.
#[test]
fn r03c2_08_nested_asset_is_read_with_exact_bytes_length_and_identity() {
    let scratch = Scratch::new("nested");
    let body = b"confined-asset-source-bytes";
    scratch.write("assets/brand/logo.png", body);
    let capability = scratch.capability();

    let (bytes, identity, length, links) = present(read(&capability, "brand/logo.png"));
    assert_eq!(bytes, body.to_vec());
    assert_eq!(length, body.len() as u64);
    assert_eq!(links, 1);
    assert!(!capability.mechanism().is_empty());

    // The same unchanged source re-reads to the same identity, bytes and length.
    let (again, again_identity, again_length, again_links) =
        present(read(&capability, "brand/logo.png"));
    assert_eq!(again, bytes);
    assert_eq!(again_identity, identity);
    assert_eq!(again_length, length);
    assert_eq!(again_links, 1);
}

/// `R03C2-08` — a missing leaf and a missing `assets/` namespace are typed refusals, never a
/// benign absence and never a read of something else.
#[test]
fn r03c2_08_absent_leaf_and_absent_assets_namespace_are_typed_refusals() {
    let scratch = Scratch::new("absent");
    scratch.write("assets/present.png", b"present");
    let capability = scratch.capability();
    assert_eq!(
        refused(read(&capability, "missing.png")),
        GraphicAssetReadRefusalKind::MissingSource
    );

    let bare = Scratch::without_assets("no-assets");
    let bare_capability = bare.capability();
    assert_eq!(
        refused(read(&bare_capability, "brand/logo.png")),
        GraphicAssetReadRefusalKind::MissingSource
    );
}

/// `R03C2-08` — the asset reader resolves only `assets/`, so a Page file of the same logical name
/// is unreachable, and an `assets/` path can never enter the accepted Page reader.
#[test]
fn r03c2_08_asset_reader_cannot_reach_the_pages_namespace() {
    let scratch = Scratch::new("namespaces");
    scratch.write("pages/secret.png", b"page-only-bytes");
    let capability = scratch.capability();

    assert_eq!(
        refused(read(&capability, "secret.png")),
        GraphicAssetReadRefusalKind::MissingSource,
        "a Page file is never returned for an asset reference"
    );

    scratch.write("assets/secret.png", b"asset-bytes");
    let (bytes, _, length, _) = present(read(&capability, "secret.png"));
    assert_eq!(bytes, b"asset-bytes".to_vec());
    assert_eq!(length, 11);

    match SourceCandidatePathV1::parse("assets/secret.png", SourceCandidateKind::Json) {
        Ok(locator) => match capability.read_candidate(SourceCandidateKind::Json, &locator) {
            CandidateBodyV1::Refused(refusal) => {
                assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation)
            }
            other => panic!("the Page reader must refuse an assets locator, got {other:?}"),
        },
        Err(_) => {
            // The Page locator grammar refuses an `assets/` path outright, which is the stronger
            // outcome and equally proves the two namespaces cannot be confused.
        }
    }
}

/// `R03C2-08` — a directory in place of a leaf is refused as an unsafe location, never read.
#[test]
fn r03c2_08_directory_leaf_is_refused_as_an_unsafe_location() {
    let scratch = Scratch::new("directory");
    fs::create_dir_all(scratch.path("assets/folder.png")).expect("create the directory fixture");
    let capability = scratch.capability();
    assert_eq!(
        refused(read(&capability, "folder.png")),
        GraphicAssetReadRefusalKind::UnsafeSourceLocation
    );
}

/// `R03C2-08`/`R03C2-07` — a leaf link, an intermediate directory link and a hardlink are each
/// refused with their own category, while the real file behind them stays readable.
#[test]
fn r03c2_08_leaf_intermediate_and_hard_links_are_refused() {
    let scratch = Scratch::new("links");
    scratch.write("assets/real/logo.png", b"real-bytes");
    let capability = scratch.capability();
    assert!(matches!(
        read(&capability, "real/logo.png"),
        EntityAssetSourceV1::Present { .. }
    ));

    create_file_link(
        &scratch.path("assets/real/logo.png"),
        &scratch.path("assets/leaf-link.png"),
    );
    assert_eq!(
        refused(read(&capability, "leaf-link.png")),
        GraphicAssetReadRefusalKind::UnsafeSourceLocation
    );

    create_directory_link(
        &scratch.path("assets/real"),
        &scratch.path("assets/dir-link"),
    );
    assert_eq!(
        refused(read(&capability, "dir-link/logo.png")),
        GraphicAssetReadRefusalKind::UnsafeSourceLocation
    );

    fs::hard_link(
        scratch.path("assets/real/logo.png"),
        scratch.path("assets/hard-link.png"),
    )
    .expect("create a hardlink fixture");
    assert_eq!(
        refused(read(&capability, "real/logo.png")),
        GraphicAssetReadRefusalKind::IdentityCollision,
        "a second directory entry makes ownership ambiguous"
    );
}

/// `R03C2-08`/`R03C2-09` — prohibited spellings are refused before any native open, even when a
/// file of that name really exists on disk and is reachable by an admitted spelling.
#[test]
fn r03c2_08_prohibited_spellings_are_refused_before_any_open() {
    let scratch = Scratch::new("names");
    scratch.write("assets/Brand/Logo.png", b"uppercase-bytes");
    scratch.write("assets/brand/logo.png", b"lowercase-bytes");
    let capability = scratch.capability();

    assert_eq!(
        refused(read(&capability, "Brand/Logo.png")),
        GraphicAssetReadRefusalKind::UnsafeAssetReference
    );
    assert_eq!(
        refused(read(&capability, "con.png")),
        GraphicAssetReadRefusalKind::UnsafeAssetReference
    );
    assert_eq!(
        refused(read(&capability, "brand/1logo.png")),
        GraphicAssetReadRefusalKind::UnsafeAssetReference
    );

    let (bytes, _, _, _) = present(read(&capability, "brand/logo.png"));
    assert_eq!(bytes, b"lowercase-bytes".to_vec());
}

/// `R03C2-06` — the real bound is exact: 20 MiB is acquired, and 20 MiB + 1 byte is refused
/// before the transfer rather than truncated.
#[test]
fn r03c2_06_real_bound_accepts_twenty_mib_and_refuses_one_byte_more() {
    let scratch = Scratch::new("bound");
    let at_bound = vec![7_u8; MAX_GRAPHIC_SOURCE_BYTES as usize];
    scratch.write("assets/at-bound.png", &at_bound);
    let capability = scratch.capability();

    let (bytes, _, length, _) = present(read(&capability, "at-bound.png"));
    assert_eq!(length, MAX_GRAPHIC_SOURCE_BYTES);
    assert_eq!(bytes.len(), MAX_GRAPHIC_SOURCE_BYTES as usize);

    let over_bound = vec![7_u8; MAX_GRAPHIC_SOURCE_BYTES as usize + 1];
    scratch.write("assets/over-bound.png", &over_bound);
    assert_eq!(
        refused(read(&capability, "over-bound.png")),
        GraphicAssetReadRefusalKind::OversizeSource
    );
}

/// `R03C2-10` — a same-size rewrite is observed by the next read rather than served from memory,
/// and the object identity stays the same because the file was rewritten in place.
#[test]
fn r03c2_10_same_size_rewrite_is_observed_and_never_served_from_memory() {
    let scratch = Scratch::new("rewrite");
    scratch.write("assets/rewrite.png", b"first-bytes");
    let capability = scratch.capability();

    let (first, identity_before, length_before, _) = present(read(&capability, "rewrite.png"));
    assert_eq!(first, b"first-bytes".to_vec());

    scratch.write("assets/rewrite.png", b"other-bytes");
    let (second, identity_after, length_after, _) = present(read(&capability, "rewrite.png"));
    assert_eq!(second, b"other-bytes".to_vec());
    assert_ne!(second, first, "a rewrite is observed, never cached");
    assert_eq!(length_after, length_before);
    assert_eq!(identity_after, identity_before);
}

// ---------------------------------------------------------------------------
// Task-owned handle helpers and host-specific fixtures.
//
// These mirror the accepted R02 trusted-handle and writer fixtures, but every assertion below is
// made through the **R03.C2 `read_entity_asset` interface**, never through a Page locator.
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
    OwnedFd::from(fs::File::open(path).expect("open the regular file as a handle"))
}

#[cfg(windows)]
fn open_file_handle(path: &Path) -> TrustedDirectoryHandle {
    use std::os::windows::io::OwnedHandle;
    OwnedHandle::from(fs::File::open(path).expect("open the regular file as a handle"))
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

/// `S065R02-05` / `R03C2-08` — a transferred, already-open `content/` directory handle anchors the
/// same confined `assets/` route as the bootstrap path, with the same physical root identity and
/// the same leaf bytes, length, link count and identity.
#[test]
fn r03c2_08b_trusted_directory_handle_reads_the_same_assets_with_one_root_identity() {
    let scratch = Scratch::new("asset-trusted-handle");
    scratch.write("assets/brand/logo.png", b"trusted-handle-bytes");
    let by_path = scratch.capability();
    let by_handle = ContentRootCapability::from_trusted_directory(
        open_directory_handle(&scratch.content()),
        TRUSTED,
    )
    .expect("a real directory handle must be accepted as the content root");

    assert_eq!(by_handle.provenance(), TRUSTED);
    assert_eq!(
        by_handle.identity(),
        by_path.identity(),
        "the handle and the bootstrap path name one physical directory"
    );
    assert!(!by_handle.mechanism().is_empty());

    let (path_bytes, path_identity, path_length, path_links) =
        present(read(&by_path, "brand/logo.png"));
    let (handle_bytes, handle_identity, handle_length, handle_links) =
        present(read(&by_handle, "brand/logo.png"));

    assert_eq!(handle_bytes, b"trusted-handle-bytes".to_vec());
    assert_eq!(handle_bytes, path_bytes);
    assert_eq!(handle_length, path_length);
    assert_eq!(handle_links, 1);
    assert_eq!(path_links, 1);
    assert_eq!(handle_identity, path_identity);

    // A second read through the transferred handle still reports the same leaf object.
    let (again, again_identity, _, _) = present(read(&by_handle, "brand/logo.png"));
    assert_eq!(again, handle_bytes);
    assert_eq!(again_identity, handle_identity);
}

/// `S065R02-05`/`S065R02-06` / `R03C2-08` — a purported root that is a regular file, a path
/// symlink or a path junction/reparse point is refused before any asset is opened through it.
#[test]
fn r03c2_08b_bootstrap_root_that_is_a_file_or_a_link_is_refused() {
    let scratch = Scratch::new("asset-unsafe-root");
    scratch.write("plain.txt", b"not a content root");

    match ContentRootCapability::adopt_bootstrap_path(&scratch.path("plain.txt"), TRUSTED) {
        Ok(_) => panic!("a regular file must never become a content root"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }

    let linked = scratch.base.join("content-link");
    create_directory_link(&scratch.content(), &linked);
    match ContentRootCapability::adopt_bootstrap_path(&linked, TRUSTED) {
        Ok(_) => panic!("a link must never become a content root"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

/// `S065R02-05` — a transferred handle that names a **regular file** can never be elevated into a
/// content root, on either host, and therefore can never anchor an `assets/` read.
#[test]
fn r03c2_08b_transferred_regular_file_handle_is_refused() {
    let scratch = Scratch::new("asset-trusted-file");
    scratch.write("plain.txt", b"not a content root");

    match ContentRootCapability::from_trusted_directory(
        open_file_handle(&scratch.path("plain.txt")),
        TRUSTED,
    ) {
        Ok(_) => panic!("a handle naming a regular file must not become a content root"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

/// `S065R02-06` — a transferred **reparse/junction** handle is refused on Windows before it can
/// anchor any asset read.
#[cfg(windows)]
#[test]
fn r03c2_08b_transferred_reparse_handle_is_refused() {
    let scratch = Scratch::new("asset-trusted-junction");
    scratch.write("assets/logo.png", b"behind-a-junction");
    let linked = scratch.base.join("content-junction");
    create_directory_link(&scratch.content(), &linked);

    match ContentRootCapability::from_trusted_directory(open_reparse_handle(&linked), TRUSTED) {
        Ok(_) => panic!("a handle naming a reparse point must be refused"),
        Err(refusal) => assert_eq!(refusal.kind(), SourceReadRefusalKind::UnsafeLocation),
    }
}

/// `S065R02-07` / `R03C2-10` — on Linux the actual `assets/` route must refuse a filesystem that
/// cannot provide the immutable reflink snapshot, and must not copy the bytes as a fallback. The
/// accepted Btrfs fixture stays positive, so the refusal is caused by the filesystem, not by the
/// test shape.
#[cfg(target_os = "linux")]
#[test]
fn r03c2_10b_asset_read_on_a_non_reflink_filesystem_is_unsupported() {
    let parent = match std::env::var_os("SOLO_TEST_UNSUPPORTED_FS_ROOT") {
        Some(root) if !root.is_empty() => PathBuf::from(root),
        _ => panic!(
            "SOLO_TEST_UNSUPPORTED_FS_ROOT must name a filesystem without reflink; the asset refusal path cannot be proven without it"
        ),
    };
    let unsupported = Scratch::on(parent, "unsupported-asset");
    // Small by design: the fixture volume is 16 MiB, and the refusal must not depend on size.
    unsupported.write("assets/logo.png", b"tmpfs-bytes");
    let unsupported_capability = unsupported.capability();
    let outcome = read(&unsupported_capability, "logo.png");
    match outcome {
        EntityAssetSourceV1::Refused(refusal) => {
            assert_eq!(refusal.kind(), GraphicAssetReadRefusalKind::Unsupported);
            assert_eq!(refusal.code(), "ASSET_SOURCE_UNSUPPORTED");
        }
        EntityAssetSourceV1::Present { bytes, .. } => panic!(
            "a filesystem without reflink must refuse rather than copy, got {} bytes",
            bytes.len()
        ),
    }

    // The same route on the accepted reflink fixture still acquires the bytes.
    let supported = Scratch::new("supported-asset");
    supported.write("assets/logo.png", b"btrfs-bytes");
    let (bytes, _, length, links) = present(read(&supported.capability(), "logo.png"));
    assert_eq!(bytes, b"btrfs-bytes".to_vec());
    assert_eq!(length, 11);
    assert_eq!(links, 1);
}

// ---------------------------------------------------------------------------
// Windows — a real pre-existing writer over an `assets/` leaf, proven through the asset port.
//
// A fake port cannot prove a share mode. These cases drive the real Win32 sharing rules on the
// actual `read_entity_asset` route: the reader's leaf handle requests read data with share-read
// only, so an existing writable handle blocks it, and the refusal category is the one the frozen
// native mapping produces for a sharing violation.
// ---------------------------------------------------------------------------

#[cfg(windows)]
mod asset_writer {
    use super::*;
    use std::fs::OpenOptions;
    use std::os::windows::fs::OpenOptionsExt;

    /// `FILE_SHARE_READ` — the share mode the reader requests for a leaf handle.
    const FILE_SHARE_READ: u32 = 0x0000_0001;

    #[test]
    fn r03c2_10b_asset_read_is_refused_while_a_writer_holds_the_leaf() {
        let scratch = Scratch::new("asset-writer");
        scratch.write("assets/written.png", b"writer-visible-bytes");
        let path = scratch.path("assets/written.png");

        // A writer that got there first. The reader shares read only, so this existing write
        // access makes its own leaf open fail: no bytes are returned for the duration.
        let writer = OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("a writer may hold the asset before the reader arrives");
        let capability = scratch.capability();

        match read(&capability, "written.png") {
            EntityAssetSourceV1::Refused(refusal) => {
                // The frozen native mapping reports a sharing violation as `OtherIo`; the
                // assertion records the actual category rather than inventing a stronger one.
                assert_eq!(refusal.kind(), GraphicAssetReadRefusalKind::OtherIo);
                assert_eq!(refusal.code(), "ASSET_SOURCE_IO_ERROR");
            }
            EntityAssetSourceV1::Present { bytes, .. } => panic!(
                "a leaf a writer already holds must not be acquired, got {} bytes",
                bytes.len()
            ),
        }

        drop(writer);

        // Once the writer is released the same route acquires exactly the file's bytes.
        let (bytes, identity, length, links) = present(read(&capability, "written.png"));
        assert_eq!(bytes, b"writer-visible-bytes".to_vec());
        assert_eq!(length, 20);
        assert_eq!(links, 1);
        let (again, again_identity, _, _) = present(read(&capability, "written.png"));
        assert_eq!(again, bytes);
        assert_eq!(again_identity, identity);
    }

    #[test]
    fn r03c2_10b_pinned_asset_handle_denies_a_competing_writer_and_delete() {
        let scratch = Scratch::new("asset-pinned");
        scratch.write("assets/pin.png", b"pinned-bytes");
        let path = scratch.path("assets/pin.png");

        // Exactly the handle the reader takes for a leaf: read access, share-read only.
        let pinned = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ)
            .open(&path)
            .expect("open the asset the way the reader does");

        assert!(
            OpenOptions::new().write(true).open(&path).is_err(),
            "a writer must be denied while the asset handle is pinned"
        );
        assert!(
            fs::remove_file(&path).is_err(),
            "a delete must be denied while the asset handle is pinned"
        );

        drop(pinned);
        let (bytes, _, length, _) = present(read(&scratch.capability(), "pin.png"));
        assert_eq!(bytes, b"pinned-bytes".to_vec());
        assert_eq!(length, 12);
    }
}
