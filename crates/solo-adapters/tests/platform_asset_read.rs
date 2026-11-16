//! Real-filesystem tests for the separate Platform root capability.
//!
//! Each test copies the **tracked** product library into a scratch Platform root on the verified
//! fixture filesystem (Linux: the disposable Btrfs mount the R03 canaries prove; Windows: the
//! runner's temporary directory) and then drives the real capability. Test-only copying is setup
//! for the genuine operating-system boundary: it is not a production reader, not a fallback and
//! not proof of installation. No tracked product byte is ever mutated.

use std::fs;
use std::path::{Path, PathBuf};

use solo_adapters::{
    PlatformLicensingDocumentV1, PlatformRootCapabilityV1, parse_platform_asset_catalogue_v1,
};
use solo_application::{
    PlatformAssetLibraryReadPortV1, PlatformCatalogueSourceV1, PlatformRootProvenanceV1,
    sha256_media_worker_bytes_v1,
};

/// The approved catalogue's canonical LF SHA-256.
const CATALOGUE_SHA256: &str = "ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7";

/// The retained Tabler MIT notice's SHA-256.
const MIT_SHA256: &str = "b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db";

/// The tracked product library, read only as the copy source.
fn product_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../platform")
}

/// The directory a scratch tree is created in.
///
/// On Linux this must be the verified fixture filesystem: the reader takes an immutable clone, and
/// a filesystem without reflink could only refuse. A missing value is a loud failure, never a
/// quiet run on an unverified filesystem.
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

/// A disposable Platform root holding one copy of the tracked product library.
struct Scratch {
    base: PathBuf,
    platform: PathBuf,
}

impl Scratch {
    fn new(label: &str) -> Self {
        let base =
            scratch_parent().join(format!("solo-platform-read-{}-{label}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let platform = base.join("platform");
        copy_tree(&product_root(), &platform);
        Self { base, platform }
    }

    fn capability(&self) -> PlatformRootCapabilityV1 {
        PlatformRootCapabilityV1::adopt_bootstrap_path(
            &self.platform,
            PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot,
        )
        .expect("the scratch Platform root is adoptable")
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.base);
    }
}

/// Copies a directory tree of regular files, refusing to dereference links.
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create the destination directory");
    for entry in fs::read_dir(from).expect("read the source directory") {
        let entry = entry.expect("a directory entry");
        let file_type = entry.file_type().expect("a file type");
        assert!(
            !file_type.is_symlink(),
            "the tracked product tree holds no link"
        );
        let target = to.join(entry.file_name());
        if file_type.is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            assert!(file_type.is_file(), "the tracked product tree holds files");
            fs::copy(entry.path(), &target).expect("copy one product file");
        }
    }
}

#[test]
fn the_product_catalogue_and_its_sources_are_read_byte_exactly() {
    let scratch = Scratch::new("exact");
    let capability = scratch.capability();
    assert!(
        !capability.mechanism().is_empty(),
        "a native mechanism is reported"
    );
    #[cfg(target_os = "linux")]
    assert!(capability.mechanism().contains("ficlone"));
    #[cfg(windows)]
    assert!(capability.mechanism().contains("ntcreatefile"));

    let bytes = match capability.read_catalogue(2 * 1024 * 1024) {
        PlatformCatalogueSourceV1::Present { bytes, .. } => bytes,
        PlatformCatalogueSourceV1::Refused(kind) => {
            panic!("the catalogue must be readable: {kind}")
        }
    };
    assert_eq!(
        sha256_media_worker_bytes_v1(&bytes).to_hex(),
        CATALOGUE_SHA256
    );
    let catalogue = parse_platform_asset_catalogue_v1(&bytes, "1.0.0")
        .expect("the strict R12 parser accepts the approved catalogue");
    assert_eq!(catalogue.len(), 81);

    for entry in catalogue.entries() {
        let expected = fs::read(
            scratch
                .platform
                .join("assets/icons")
                .join(entry.id().split_once('/').map(|(_, name)| name).unwrap()),
        )
        .expect("the copied product source");
        match capability.read_platform_source(entry.id(), 20 * 1024 * 1024) {
            solo_application::PlatformAssetSourceV1::Present {
                bytes,
                length,
                links,
                ..
            } => {
                assert_eq!(bytes, expected, "{}: exact product bytes", entry.id());
                assert_eq!(
                    sha256_media_worker_bytes_v1(&bytes).to_hex(),
                    entry.sha256().to_hex()
                );
                assert_eq!(u64::try_from(bytes.len()).unwrap(), length);
                assert_eq!(links, 1);
            }
            solo_application::PlatformAssetSourceV1::Refused(kind) => {
                panic!("{}: must be readable: {kind}", entry.id())
            }
        }
    }

    let notice = capability
        .read_licensing_document(PlatformLicensingDocumentV1::MitNotice, 1024 * 1024)
        .expect("the retained notice is readable");
    assert_eq!(sha256_media_worker_bytes_v1(&notice).to_hex(), MIT_SHA256);
}
