//! Inward Platform source-read tests over a fake port.
//!
//! These tests prove the policy the use case owns, without opening any file at run time: the
//! selected version, the independent catalogue pin, membership, per-source digest recomputation,
//! forged port observations, ordering, duplicate and alias refusal, whole-set refusal and the
//! closed authority flags. The approved catalogue and a few approved icons are embedded at compile
//! time, so the bytes are the real approved bytes and no runtime filesystem read is involved. The
//! real native path is proved separately by the adapter suite and by the two-host `n127` run.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use solo_application::{
    APPROVED_PLATFORM_ASSET_COUNT_V1, APPROVED_PLATFORM_LIBRARY_VERSION_V1, PhysicalIdentityV1,
    PlatformAssetCatalogueEntryV1, PlatformAssetCatalogueV1, PlatformAssetDeclarationV1,
    PlatformAssetLibraryReadPortV1, PlatformAssetLicenceV1, PlatformAssetProvenanceV1,
    PlatformAssetReadRefusalKindV1, PlatformAssetRefV1, PlatformAssetSourceV1,
    PlatformCatalogueSourceV1, PlatformLibraryReadOutcomeV1, PlatformRootProvenanceV1,
    PlatformRootSelectionV1, read_pinned_platform_sources_v1, sha256_media_worker_bytes_v1,
};
use solo_domain::{AssetNamespace, AssetReferenceV1};

/// The approved product library, read once from the tracked tree for these fixtures.
///
/// The port under test is a fake; only the *bytes* come from the real approved product files, so
/// no filesystem reader is exercised here.
fn product_asset(relative: &str) -> Vec<u8> {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../platform/assets")
        .join(relative);
    std::fs::read(&path).unwrap_or_else(|error| panic!("{} is unreadable: {error}", path.display()))
}

/// The three approved sources this suite requests, by exact catalogue identifier.
const APPROVED_IDS: [&str; 3] = [
    "icons/icon-about.svg",
    "icons/icon-home.svg",
    "icons/icon-clients.svg",
];

/// The approved catalogue bytes, read once from the tracked product tree.
fn approved_catalogue_bytes() -> &'static [u8] {
    use std::sync::OnceLock;
    static BYTES: OnceLock<Vec<u8>> = OnceLock::new();
    BYTES
        .get_or_init(|| product_asset("catalogue-1.0.0.json"))
        .as_slice()
}

/// One approved source's bytes, read once from the tracked product tree.
fn source_bytes(id: &str) -> &'static [u8] {
    use std::sync::OnceLock;
    static SOURCES: OnceLock<BTreeMap<String, Vec<u8>>> = OnceLock::new();
    let sources = SOURCES.get_or_init(|| {
        APPROVED_IDS
            .iter()
            .map(|id| {
                let name = id
                    .split_once('/')
                    .map(|(_, name)| name)
                    .expect("an icons/<name> identifier");
                ((*id).to_owned(), product_asset(&format!("icons/{name}")))
            })
            .collect()
    });
    sources.get(id).map_or(&[], |bytes| bytes.as_slice())
}

/// One fake source body.
#[derive(Clone)]
enum FakeSource {
    Present {
        bytes: Vec<u8>,
        length_delta: i64,
        links: u64,
        identity: PhysicalIdentityV1,
    },
    Refused(PlatformAssetReadRefusalKindV1),
}

/// A fake Platform reader: it answers exactly what a test configured, and records every call.
struct FakePort {
    catalogue: Option<Vec<u8>>,
    catalogue_links: u64,
    sources: BTreeMap<String, FakeSource>,
    calls: RefCell<Vec<String>>,
}

impl FakePort {
    fn new() -> Self {
        Self {
            catalogue: None,
            catalogue_links: 1,
            sources: BTreeMap::new(),
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }

    fn with_catalogue(mut self, bytes: &[u8]) -> Self {
        self.catalogue = Some(bytes.to_vec());
        self
    }

    fn with_source(mut self, id: &str, bytes: &[u8], ordinal: u64) -> Self {
        self.sources.insert(
            id.to_owned(),
            FakeSource::Present {
                bytes: bytes.to_vec(),
                length_delta: 0,
                links: 1,
                identity: PhysicalIdentityV1::new(7, ordinal, 0),
            },
        );
        self
    }

    fn with_source_body(mut self, id: &str, source: FakeSource) -> Self {
        self.sources.insert(id.to_owned(), source);
        self
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
                    links: self.catalogue_links,
                    fingerprint: 0,
                }
            }
        }
    }

    fn read_platform_source(&self, asset_id: &str, max_bytes: u64) -> PlatformAssetSourceV1 {
        self.calls.borrow_mut().push(asset_id.to_owned());
        match self.sources.get(asset_id) {
            None => PlatformAssetSourceV1::Refused(PlatformAssetReadRefusalKindV1::MissingSource),
            Some(FakeSource::Refused(kind)) => PlatformAssetSourceV1::Refused(*kind),
            Some(FakeSource::Present {
                bytes,
                length_delta,
                links,
                identity,
            }) => {
                if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > max_bytes {
                    return PlatformAssetSourceV1::Refused(
                        PlatformAssetReadRefusalKindV1::OversizeSource,
                    );
                }
                let length = u64::try_from(bytes.len())
                    .unwrap_or(u64::MAX)
                    .saturating_add_signed(*length_delta);
                PlatformAssetSourceV1::Present {
                    bytes: bytes.clone(),
                    identity: *identity,
                    length,
                    links: *links,
                    fingerprint: 0,
                }
            }
        }
    }
}

/// Extracts the declared `(id, sha256)` pairs from the approved catalogue bytes, in order.
///
/// A deliberately tiny scanner rather than a JSON dependency: the document's exact bytes are
/// pinned, so the two keys are unambiguous, and this keeps the inward crate dependency-free.
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

fn reference(identifier: &str) -> PlatformAssetRefV1 {
    PlatformAssetRefV1::from_identifier(identifier).expect("a valid Platform identifier")
}

fn selection() -> PlatformRootSelectionV1 {
    PlatformRootSelectionV1::adopt(PlatformRootProvenanceV1::OperatorDesignatedPlatformRoot)
}

fn refusal_kind(outcome: &PlatformLibraryReadOutcomeV1) -> PlatformAssetReadRefusalKindV1 {
    outcome
        .refusal()
        .copied()
        .unwrap_or_else(|| panic!("expected a refusal, got {outcome:?}"))
        .kind()
}

/// A port holding the approved catalogue and the three embedded approved sources.
fn approved_port() -> FakePort {
    let mut port = FakePort::new().with_catalogue(approved_catalogue_bytes());
    for (ordinal, id) in APPROVED_IDS.iter().enumerate() {
        port = port.with_source(
            id,
            source_bytes(id),
            u64::try_from(ordinal).unwrap_or(0) + 1,
        );
    }
    port
}

fn approved_requests() -> Vec<PlatformAssetRefV1> {
    APPROVED_IDS.iter().map(|id| reference(id)).collect()
}

#[test]
fn the_approved_catalogue_pins_and_admits_its_own_sources() {
    let (pairs, catalogue) = approved_catalogue();
    assert_eq!(catalogue.len(), APPROVED_PLATFORM_ASSET_COUNT_V1);
    assert_eq!(catalogue.version(), APPROVED_PLATFORM_LIBRARY_VERSION_V1);
    assert_eq!(
        sha256_media_worker_bytes_v1(approved_catalogue_bytes()).to_hex(),
        solo_application::APPROVED_PLATFORM_CATALOGUE_SHA256_V1
    );

    let port = approved_port();
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &approved_requests(),
        &port,
    );
    let set = outcome.complete().unwrap_or_else(|| {
        panic!("the approved sources must be admitted: {outcome:?}");
    });

    assert_eq!(set.len(), APPROVED_IDS.len());
    assert_eq!(set.source_namespace(), AssetNamespace::Platform);
    assert!(!set.grants_publication_authority());
    assert!(!set.proves_media_correctness());
    assert!(!set.proves_global_snapshot());

    // The evidence is in request order, and every digest was recomputed from the bytes here.
    for (position, id) in APPROVED_IDS.iter().enumerate() {
        let bytes = source_bytes(id);
        let evidence = &set.evidence()[position];
        assert_eq!(evidence.asset_id(), *id);
        assert_eq!(evidence.bytes(), bytes);
        let declared = pairs
            .iter()
            .find(|(candidate, _)| candidate == id)
            .map(|(_, digest)| digest.clone())
            .expect("a declared digest");
        assert_eq!(evidence.source_sha256(), declared);
        assert_eq!(
            evidence.source_sha256(),
            sha256_media_worker_bytes_v1(bytes).to_hex()
        );
        assert_eq!(evidence.length(), u64::try_from(bytes.len()).unwrap_or(0));
        assert_eq!(set.exact(id).map(|found| found.asset_id()), Some(*id));
    }

    // One catalogue read, then exactly one source read per requested asset.
    assert_eq!(
        port.calls(),
        vec![
            "catalogue".to_owned(),
            APPROVED_IDS[0].to_owned(),
            APPROVED_IDS[1].to_owned(),
            APPROVED_IDS[2].to_owned(),
        ]
    );
}

#[test]
fn the_selected_version_must_be_supplied_and_approved() {
    let (_, catalogue) = approved_catalogue();
    let port = approved_port();
    for version in ["", "1.0.1", "2.0.0", "1.0", "v1.0.0"] {
        let outcome = read_pinned_platform_sources_v1(
            selection(),
            version,
            &catalogue,
            &approved_requests(),
            &port,
        );
        assert_eq!(
            refusal_kind(&outcome),
            PlatformAssetReadRefusalKindV1::UnselectedVersion,
            "{version:?} must be refused"
        );
    }
    // No default version exists: the caller always supplies one, and an empty request is refused.
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &[],
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::EmptyRequestedSet
    );
}

#[test]
fn an_unapproved_catalogue_is_refused_even_when_self_consistent() {
    let (pairs, catalogue) = approved_catalogue();
    // A single flipped byte keeps the document readable but is not the approved catalogue.
    let mut mutated = approved_catalogue_bytes().to_vec();
    let position = mutated
        .windows(8)
        .position(|window| window == b"\"assets\"")
        .expect("the assets key");
    mutated[position] = b' ';
    assert_ne!(
        sha256_media_worker_bytes_v1(&mutated).to_hex(),
        solo_application::APPROVED_PLATFORM_CATALOGUE_SHA256_V1
    );

    let port = FakePort::new().with_catalogue(&mutated).with_source(
        APPROVED_IDS[0],
        source_bytes(APPROVED_IDS[0]),
        1,
    );
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &[reference(APPROVED_IDS[0])],
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::CatalogueUnapproved
    );
    // The refusal was taken before any source read.
    assert_eq!(port.calls(), vec!["catalogue".to_owned()]);
    assert_eq!(pairs.len(), APPROVED_PLATFORM_ASSET_COUNT_V1);
}

#[test]
fn a_missing_or_aliased_catalogue_is_refused_before_any_source_read() {
    let (_, catalogue) = approved_catalogue();
    let requests = approved_requests();

    let absent = FakePort::new().with_source(APPROVED_IDS[0], source_bytes(APPROVED_IDS[0]), 1);
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[requests[0].clone()],
            &absent,
        )),
        PlatformAssetReadRefusalKindV1::CatalogueUnavailable
    );
    assert_eq!(absent.calls(), vec!["catalogue".to_owned()]);

    let linked = FakePort {
        catalogue: Some(approved_catalogue_bytes().to_vec()),
        catalogue_links: 2,
        sources: BTreeMap::new(),
        calls: RefCell::new(Vec::new()),
    };
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[requests[0].clone()],
            &linked,
        )),
        PlatformAssetReadRefusalKindV1::IdentityCollision
    );
}

#[test]
fn an_entity_reference_and_an_unsafe_identifier_are_refused() {
    let entity = AssetReferenceV1::parse(AssetNamespace::Entity, "icons/icon-about.svg")
        .expect("a valid Entity grammar reference");
    assert_eq!(
        PlatformAssetRefV1::from_reference(&entity)
            .expect_err("an Entity reference is not a Platform source")
            .kind(),
        PlatformAssetReadRefusalKindV1::EntityNamespaceRefused
    );
    let platform = AssetReferenceV1::parse(AssetNamespace::Platform, "icons/icon-about.svg")
        .expect("a valid Platform grammar reference");
    assert_eq!(
        PlatformAssetRefV1::from_reference(&platform)
            .expect("a Platform reference")
            .as_str(),
        "icons/icon-about.svg"
    );
    for unsafe_identifier in [
        "",
        "icons/Icon-About.svg",
        "icons/icon-about.png",
        "icons/icon-about",
        "assets/icons/icon-about.svg",
        "platform/icons/icon-about.svg",
        "icons/../icons/icon-about.svg",
        "icons/sub/icon-about.svg",
        "content/assets/icons/icon-about.svg",
        "icons/icon about.svg",
        "icons/.icon-about.svg",
        "icons/icon-about.svg/",
        "/icons/icon-about.svg",
    ] {
        assert_eq!(
            PlatformAssetRefV1::from_identifier(unsafe_identifier)
                .expect_err("an unsafe identifier must be refused exactly as written")
                .kind(),
            PlatformAssetReadRefusalKindV1::UnsafeAssetIdentifier,
            "{unsafe_identifier:?}"
        );
    }
}

#[test]
fn an_unknown_or_withheld_identifier_is_refused_without_a_source_read() {
    let (_, catalogue) = approved_catalogue();
    let port = approved_port();
    let unknown = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &[reference("icons/icon-not-in-the-library.svg")],
        &port,
    );
    assert_eq!(
        refusal_kind(&unknown),
        PlatformAssetReadRefusalKindV1::UnapprovedAssetIdentifier
    );
    assert_eq!(
        unknown.refusal().and_then(|refusal| refusal.position()),
        Some(0)
    );
    // A case-aliased spelling is refused exactly as written: no folding, no repair, no read.
    assert!(PlatformAssetRefV1::from_identifier("icons/ICON-about.svg").is_err());
    assert_eq!(port.calls(), vec!["catalogue".to_owned()]);
    for withheld in [
        "icons/icon-services.svg",
        "icons/icon-status-info.svg",
        "icons/icon-volunteer.svg",
    ] {
        assert!(catalogue.exact_entry(withheld).is_none(), "{withheld}");
    }
}

#[test]
fn a_source_digest_is_recomputed_here_and_never_taken_from_the_port() {
    let (_, catalogue) = approved_catalogue();
    let id = APPROVED_IDS[0];
    // Equal length, different body: only a real digest can refuse it.
    let mut swapped = source_bytes(APPROVED_IDS[0]).to_vec();
    let last = swapped.len() - 1;
    swapped[last] ^= 0x01;
    assert_eq!(swapped.len(), source_bytes(APPROVED_IDS[0]).len());

    let port = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source(id, &swapped, 1);
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &[reference(id)],
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::SourceDigestMismatch
    );
}

#[test]
fn forged_length_link_count_and_identity_observations_are_refused() {
    let (_, catalogue) = approved_catalogue();
    let id = APPROVED_IDS[0];
    let bytes = source_bytes(APPROVED_IDS[0]);

    // A port that under-reports the length cannot pass the transferred-count check.
    let short = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source_body(
            id,
            FakeSource::Present {
                bytes: bytes.to_vec(),
                length_delta: -1,
                links: 1,
                identity: PhysicalIdentityV1::new(7, 1, 0),
            },
        );
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[reference(id)],
            &short,
        )),
        PlatformAssetReadRefusalKindV1::ChangedDuringRead
    );

    // A hardlinked source is ambiguous ownership, even when the bytes are correct.
    let linked = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source_body(
            id,
            FakeSource::Present {
                bytes: bytes.to_vec(),
                length_delta: 0,
                links: 2,
                identity: PhysicalIdentityV1::new(7, 2, 0),
            },
        );
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[reference(id)],
            &linked,
        )),
        PlatformAssetReadRefusalKindV1::IdentityCollision
    );

    // Two distinct approved identifiers resolving to one physical object are refused.
    let aliased = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source_body(
            APPROVED_IDS[0],
            FakeSource::Present {
                bytes: source_bytes(APPROVED_IDS[0]).to_vec(),
                length_delta: 0,
                links: 1,
                identity: PhysicalIdentityV1::new(9, 9, 9),
            },
        )
        .with_source_body(
            APPROVED_IDS[1],
            FakeSource::Present {
                bytes: source_bytes(APPROVED_IDS[1]).to_vec(),
                length_delta: 0,
                links: 1,
                identity: PhysicalIdentityV1::new(9, 9, 9),
            },
        );
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[reference(APPROVED_IDS[0]), reference(APPROVED_IDS[1]),],
            &aliased,
        )),
        PlatformAssetReadRefusalKindV1::IdentityCollision
    );

    // A port that refuses outright is mapped through unchanged.
    let denied = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source_body(
            id,
            FakeSource::Refused(PlatformAssetReadRefusalKindV1::PermissionDenied),
        );
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[reference(id)],
            &denied,
        )),
        PlatformAssetReadRefusalKindV1::PermissionDenied
    );

    // A port that reports an oversize length is refused as oversize, never truncated.
    let oversize_length =
        i64::try_from(solo_application::MAX_PLATFORM_SOURCE_BYTES_V1).unwrap_or(i64::MAX);
    let oversize = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source_body(
            id,
            FakeSource::Present {
                bytes: bytes.to_vec(),
                length_delta: oversize_length,
                links: 1,
                identity: PhysicalIdentityV1::new(7, 3, 0),
            },
        );
    assert_eq!(
        refusal_kind(&read_pinned_platform_sources_v1(
            selection(),
            APPROVED_PLATFORM_LIBRARY_VERSION_V1,
            &catalogue,
            &[reference(id)],
            &oversize,
        )),
        PlatformAssetReadRefusalKindV1::OversizeSource
    );
}

#[test]
fn a_late_failure_refuses_the_whole_set_with_no_partial_evidence() {
    let (_, catalogue) = approved_catalogue();
    // The first two sources are correct; the last approved source is missing.
    let port = FakePort::new()
        .with_catalogue(approved_catalogue_bytes())
        .with_source(APPROVED_IDS[0], source_bytes(APPROVED_IDS[0]), 1)
        .with_source(APPROVED_IDS[1], source_bytes(APPROVED_IDS[1]), 2);

    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &approved_requests(),
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::MissingSource
    );
    assert!(outcome.complete().is_none(), "no partial set may survive");
    assert_eq!(
        outcome.refusal().and_then(|refusal| refusal.position()),
        Some(2)
    );
    let refusal = outcome.refusal().expect("a refusal");
    // The diagnostic names no path and no identifier.
    assert!(!refusal.detail().contains("icons/"), "{}", refusal.detail());
    assert!(!refusal.detail().contains('/'), "{}", refusal.detail());
    assert!(refusal.code().starts_with("PLATFORM_"));
}

#[test]
fn a_duplicate_request_is_refused_before_any_source_read() {
    let (_, catalogue) = approved_catalogue();
    let port = approved_port();
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &catalogue,
        &[reference(APPROVED_IDS[0]), reference(APPROVED_IDS[0])],
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::DuplicateRequestedAsset
    );
    assert_eq!(
        outcome.refusal().and_then(|refusal| refusal.position()),
        Some(1)
    );
    // The duplicate is refused before the catalogue is even read.
    assert!(port.calls().is_empty());
}

#[test]
fn a_catalogue_model_that_disagrees_with_the_approved_bytes_is_refused() {
    let (pairs, _) = approved_catalogue();
    // The bytes are the approved ones; the model declares a digest the document does not.
    let licence = PlatformAssetLicenceV1::new("MIT", "Paweł Kuna", "pinned MIT notice")
        .expect("a bounded licence assertion");
    let provenance = PlatformAssetProvenanceV1::new("Tabler Icons v3.46.0", "pinned provenance")
        .expect("a bounded provenance assertion");
    let undeclared = sha256_media_worker_bytes_v1(b"not the approved source").to_hex();
    let entries = pairs
        .iter()
        .enumerate()
        .map(|(index, (id, digest))| {
            let digest = if index == 0 {
                undeclared.clone()
            } else {
                digest.clone()
            };
            PlatformAssetCatalogueEntryV1::new(
                &PlatformAssetDeclarationV1::of(id, &digest, "image", "image/svg+xml", 24, 24),
                licence.clone(),
                provenance.clone(),
            )
            .expect("an entry with a bounded digest")
        })
        .collect();
    let model = PlatformAssetCatalogueV1::new(APPROVED_PLATFORM_LIBRARY_VERSION_V1, entries)
        .expect("a valid catalogue model");

    let port = approved_port();
    let outcome = read_pinned_platform_sources_v1(
        selection(),
        APPROVED_PLATFORM_LIBRARY_VERSION_V1,
        &model,
        &approved_requests(),
        &port,
    );
    assert_eq!(
        refusal_kind(&outcome),
        PlatformAssetReadRefusalKindV1::CatalogueModelMismatch
    );
    assert_eq!(port.calls(), vec!["catalogue".to_owned()]);
}
