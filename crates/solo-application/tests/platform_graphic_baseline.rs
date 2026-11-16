//! `SOLO-WO-118 R17` — N128 case 07: the inward verified binding a graphic baseline must hold.
//!
//! The risk proved here is **source identity**, not cascade logic: a registered root default is only
//! eligible when the transferred catalogue bytes match the accepted pin, the exact identifier is
//! declared, its declared digest equals the evidence of one accepted confined read, and the media
//! class the role requires is proven. An unread or mis-identified asset is refused rather than
//! substituted.
//!
//! The bytes are the real approved product bytes and the port is a fake, exactly as the accepted
//! `platform_asset_read` suite does: no filesystem reader is exercised here, and the native path is
//! proved separately by the two-host run.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use solo_application::{
    APPROVED_PLATFORM_ASSET_COUNT_V1, APPROVED_PLATFORM_LIBRARY_VERSION_V1,
    PLATFORM_GRAPHIC_BASELINE_GRANTS_PUBLICATION_AUTHORITY_V1, PhysicalIdentityV1,
    PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1, PlatformAssetDeclarationV1,
    PlatformAssetLibraryReadPortV1, PlatformAssetLicenceV1, PlatformAssetProvenanceV1,
    PlatformAssetReadRefusalKindV1, PlatformAssetRefV1, PlatformAssetSourceSetV1,
    PlatformAssetSourceV1, PlatformCatalogueSourceV1, PlatformGraphicBaselineRefusalKindV1,
    PlatformRootProvenanceV1, PlatformRootSelectionV1, approved_platform_graphic_binding_v1,
    read_pinned_platform_sources_v1,
};
use solo_domain::{
    AssetNamespace, AssetReferenceV1, GraphicCascadeRefusalKind, PlatformGraphicBaselineRecordV1,
    PlatformGraphicBaselineV1, PropertyKey,
};

/// The two approved sources this suite reads, by exact catalogue identifier.
const READ_IDS: [&str; 2] = ["icons/icon-home.svg", "icons/icon-about.svg"];

/// The approved product library, read from the tracked tree for these fixtures.
fn product_asset(relative: &str) -> Vec<u8> {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../platform/assets")
        .join(relative);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{} is unreadable: {error}", path.display()))
}

/// The approved catalogue bytes.
fn approved_catalogue_bytes() -> &'static [u8] {
    use std::sync::OnceLock;
    static BYTES: OnceLock<Vec<u8>> = OnceLock::new();
    BYTES
        .get_or_init(|| product_asset("catalogue-1.0.0.json"))
        .as_slice()
}

/// One approved source's real bytes.
fn source_bytes(id: &str) -> Vec<u8> {
    product_asset(id)
}

/// Extracts the declared `(id, sha256)` pairs from the approved catalogue bytes, in order.
fn declared_pairs(bytes: &[u8]) -> Vec<(String, String)> {
    fn value_after(bytes: &[u8], key: &str, from: usize) -> Option<(String, usize)> {
        let needle = format!("\"{key}\": \"").into_bytes();
        let start = from
            + bytes[from..]
                .windows(needle.len())
                .position(|window| window == needle.as_slice())?;
        let value_start = start + needle.len();
        let end = value_start + bytes[value_start..].iter().position(|byte| *byte == b'"')?;
        Some((
            String::from_utf8(bytes[value_start..end].to_vec()).ok()?,
            end,
        ))
    }
    let mut pairs = Vec::new();
    let mut cursor = 0;
    while let Some((id, after_id)) = value_after(bytes, "id", cursor) {
        let Some((digest, after_digest)) = value_after(bytes, "sha256", after_id) else {
            break;
        };
        pairs.push((id, digest));
        cursor = after_digest;
    }
    pairs
}

/// The approved 81-entry catalogue model, built from the pinned bytes.
fn approved_catalogue() -> (Vec<(String, String)>, PlatformAssetCatalogueV1) {
    let pairs = declared_pairs(approved_catalogue_bytes());
    assert_eq!(pairs.len(), APPROVED_PLATFORM_ASSET_COUNT_V1);
    let licence = PlatformAssetLicenceV1::new("MIT", "Paweł Kuna", "pinned MIT notice")
        .expect("a bounded licence assertion");
    let provenance = PlatformAssetProvenanceV1::new("Tabler Icons v3.46.0", "pinned provenance")
        .expect("a bounded provenance assertion");
    let entries = pairs
        .iter()
        .map(|(id, digest)| {
            PlatformAssetCatalogueEntryV1::new(
                &PlatformAssetDeclarationV1::of(id, digest, "image", "image/svg+xml", 24, 24),
                licence.clone(),
                provenance.clone(),
            )
            .expect("an approved entry")
        })
        .collect();
    let catalogue = PlatformAssetCatalogueV1::new(APPROVED_PLATFORM_LIBRARY_VERSION_V1, entries)
        .expect("the approved catalogue satisfies the R12 contract");
    (pairs, catalogue)
}

/// A fake Platform reader that answers exactly what a test configured.
struct FakePort {
    catalogue: Option<Vec<u8>>,
    sources: BTreeMap<String, (Vec<u8>, u64)>,
    calls: RefCell<Vec<String>>,
}

impl FakePort {
    fn new() -> Self {
        Self {
            catalogue: None,
            sources: BTreeMap::new(),
            calls: RefCell::new(Vec::new()),
        }
    }

    fn with_catalogue(mut self, bytes: &[u8]) -> Self {
        self.catalogue = Some(bytes.to_vec());
        self
    }

    fn with_source(mut self, id: &str, bytes: Vec<u8>) -> Self {
        // Every source gets its own physical identity, exactly as a real directory read would.
        let ordinal = u64::try_from(self.sources.len()).unwrap_or(0) + 1;
        self.sources.insert(id.to_owned(), (bytes, ordinal));
        self
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl PlatformAssetLibraryReadPortV1 for FakePort {
    fn read_catalogue(&self, max_bytes: u64) -> PlatformCatalogueSourceV1 {
        self.calls.borrow_mut().push("catalogue".to_owned());
        match &self.catalogue {
            None => PlatformCatalogueSourceV1::Refused(
                PlatformAssetReadRefusalKindV1::CatalogueUnavailable,
            ),
            Some(bytes) => {
                if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
                    return PlatformCatalogueSourceV1::Refused(
                        PlatformAssetReadRefusalKindV1::CatalogueOversize,
                    );
                }
                PlatformCatalogueSourceV1::Present {
                    bytes: bytes.clone(),
                    identity: PhysicalIdentityV1::new(7, 0, 0),
                    length: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
                    links: 1,
                    fingerprint: 0,
                }
            }
        }
    }

    fn read_platform_source(&self, asset_id: &str, max_bytes: u64) -> PlatformAssetSourceV1 {
        self.calls.borrow_mut().push(asset_id.to_owned());
        match self.sources.get(asset_id) {
            None => PlatformAssetSourceV1::Refused(PlatformAssetReadRefusalKindV1::MissingSource),
            Some((bytes, ordinal)) => {
                if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
                    return PlatformAssetSourceV1::Refused(
                        PlatformAssetReadRefusalKindV1::OversizeSource,
                    );
                }
                PlatformAssetSourceV1::Present {
                    bytes: bytes.clone(),
                    identity: PhysicalIdentityV1::new(7, *ordinal, 0),
                    length: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
                    links: 1,
                    fingerprint: 0,
                }
            }
        }
    }
}

/// A port holding the approved catalogue and the real bytes of the two read identifiers.
fn approved_port() -> FakePort {
    let mut port = FakePort::new().with_catalogue(approved_catalogue_bytes());
    for id in READ_IDS {
        port = port.with_source(id, source_bytes(id));
    }
    port
}

/// Reads the two approved identifiers through the accepted read use case.
fn read_set(port: &FakePort) -> PlatformAssetSourceSetV1 {
    let requests: Vec<PlatformAssetRefV1> = READ_IDS
        .iter()
        .map(|id| PlatformAssetRefV1::from_identifier(id).expect("a valid identifier"))
        .collect();
    let (_, catalogue) = approved_catalogue();
    let outcome = read_pinned_platform_sources_v1(
        PlatformRootSelectionV1::adopt(PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &requests,
        port,
    );
    outcome
        .complete()
        .cloned()
        .unwrap_or_else(|| panic!("the accepted read must succeed, got {outcome:?}"))
}

/// `N128-07` — the baseline binds only to an exact approved identifier with a matching digest and
/// one accepted confined read, and every identity substitution is refused.
#[test]
fn n128_07_binding_binds_exact_identity_and_refuses_substitution() {
    let port = approved_port();
    let (pairs, catalogue) = approved_catalogue();
    let set = read_set(&port);
    assert_eq!(
        port.calls().len(),
        3,
        "one catalogue read and exactly two source reads"
    );
    assert_eq!(set.len(), READ_IDS.len());

    // The verified binding contains exactly the identifiers that one accepted read produced.
    let binding = approved_platform_graphic_binding_v1(
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        approved_catalogue_bytes(),
        &catalogue,
        &set,
    )
    .expect("the accepted read binds");
    assert_eq!(binding.assets().len(), READ_IDS.len());
    assert_eq!(
        binding.library_version(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1
    );
    let declared = |id: &str| {
        pairs
            .iter()
            .find(|(candidate, _)| candidate == id)
            .map(|(_, digest)| digest.clone())
            .expect("a declared identifier")
    };
    let home_digest = declared("icons/icon-home.svg");
    let about_digest = declared("icons/icon-about.svg");
    assert_eq!(
        binding
            .exact("icons/icon-home.svg")
            .expect("bound")
            .sha256(),
        home_digest
    );
    assert_eq!(
        binding
            .exact("icons/icon-about.svg")
            .expect("bound")
            .sha256(),
        about_digest
    );
    assert!(
        binding.exact("icons/icon-clients.svg").is_none(),
        "an asset that was never read is not bound"
    );

    // A registered default resolves only through that exact binding.
    let reference = AssetReferenceV1::parse(AssetNamespace::Platform, "icons/icon-home.svg")
        .expect("a valid Platform reference");
    let record = PlatformGraphicBaselineRecordV1::new(
        PropertyKey::GraphicLogo,
        "primary",
        reference,
        &home_digest,
    )
    .expect("a bound registration");
    let baseline = PlatformGraphicBaselineV1::new(binding.clone(), vec![record])
        .expect("the verified table is eligible");
    assert_eq!(baseline.binding().catalogue_sha256().len(), 64);

    // A digest substituted from a different approved asset refuses.
    let wrong_digest = PlatformGraphicBaselineRecordV1::new(
        PropertyKey::GraphicLogo,
        "primary",
        AssetReferenceV1::parse(AssetNamespace::Platform, "icons/icon-home.svg")
            .expect("a valid reference"),
        &about_digest,
    )
    .expect("a syntactically valid record");
    assert_eq!(
        PlatformGraphicBaselineV1::new(binding.clone(), vec![wrong_digest])
            .expect_err("a substituted digest must refuse")
            .kind(),
        GraphicCascadeRefusalKind::BaselineDigestMismatch
    );

    // An approved but unread identifier is not eligible: availability is not a read.
    let unread = PlatformGraphicBaselineRecordV1::new(
        PropertyKey::GraphicLogo,
        "primary",
        AssetReferenceV1::parse(AssetNamespace::Platform, "icons/icon-clients.svg")
            .expect("a valid reference"),
        &declared("icons/icon-clients.svg"),
    )
    .expect("a syntactically valid record");
    assert_eq!(
        PlatformGraphicBaselineV1::new(binding.clone(), vec![unread])
            .expect_err("an unread asset must not become a default")
            .kind(),
        GraphicCascadeRefusalKind::BaselineAssetUnapproved
    );

    // The use case itself refuses an unpinned document, an unselected version and an empty read.
    assert_eq!(
        approved_platform_graphic_binding_v1(
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            b"{}",
            &catalogue,
            &set,
        )
        .expect_err("an unpinned catalogue must refuse")
        .kind(),
        PlatformGraphicBaselineRefusalKindV1::CatalogueUnapproved
    );
    assert_eq!(
        approved_platform_graphic_binding_v1(
            "2.0.0",
            approved_catalogue_bytes(),
            &catalogue,
            &set,
        )
        .expect_err("an unselected version must refuse")
        .kind(),
        PlatformGraphicBaselineRefusalKindV1::UnselectedVersion
    );
    let empty_port = FakePort::new().with_catalogue(approved_catalogue_bytes());
    let empty_requests = [
        PlatformAssetRefV1::from_identifier("icons/icon-about.svg").expect("a valid identifier")
    ];
    let empty_outcome = read_pinned_platform_sources_v1(
        PlatformRootSelectionV1::adopt(PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &empty_requests,
        &empty_port,
    );
    assert!(
        empty_outcome.complete().is_none(),
        "a missing source refuses the read set"
    );

    // The domain accepts the verified baseline as an opt-in slice and nothing else changes.
    println!(
        "n128_evidence case=07 binding=exact read=2 digest_matched=true unread_refused=true \
         wrong_digest_refused=true catalogue_pin=independent version=1.0.0 \
         publication_authority={}",
        PLATFORM_GRAPHIC_BASELINE_GRANTS_PUBLICATION_AUTHORITY_V1
    );
}
