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

use solo_adapters::{ContentRootCapability, RootProvenanceV1};
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
        let base = scratch_parent().join(format!("solo-asset-read-{}-{label}", std::process::id()));
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
