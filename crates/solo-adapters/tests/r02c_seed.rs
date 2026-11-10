//! R02.C real-fixture proof: the checked-in starter seed and the expanded one-Entity
//! fixture driven through the accepted parser, ownership graph and source-stage candidate.
//!
//! Every case reads the **committed** source bytes above and passes them to the accepted
//! boundaries: [`solo_adapters::parse_manifest_bytes`], the pure domain
//! [`solo_domain::EfsOwnershipDraft`] graph and
//! [`solo_adapters::assemble_declared_source_set_v1`]. The seed is portable *data*: nothing
//! here installs, publishes, serves or renders it.
//!
//! **What the graph construction is.** Tests transform the already parsed declarations into a
//! pure domain draft using the fixture's known already-canonical full locale keys. That is a
//! test-owned transformation, not the missing production parse-to-graph bridge and not a
//! general BCP47 validator.
//!
//! **Fixture ownership.** A scratch root is claimed with an exclusive `create_dir`; an already
//! taken name is skipped and never adopted, reset or removed, so `Drop` can only remove a tree
//! this test created. Checked-in fixtures are only ever read.

use std::fs;
use std::path::{Path, PathBuf};

use solo_adapters::{
    ContentRootCapability, DeclaredCaseMatching, DeclaredVisibility, MANIFEST_LOGICAL_IDENTITY,
    ManifestErrorKind, ParsedManifestCandidateV1, RootProvenanceV1, SourceSetRefusalV1,
    StagingRootCapability, assemble_declared_source_set_v1, parse_manifest_bytes,
};
use solo_application::fingerprint_of;
use solo_domain::{
    EfsLineageErrorKind, EfsOwnershipDraft, EfsOwnershipGraph, EfsPageTarget, EntityIdV1,
    FacetDraft, FacetIdV1, LocaleKeyErrorKind, LocaleKeyV1, PageDraft, PageIdV1, SiteDraft,
    SiteIdV1, SourceCandidateKind,
};

const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

/// The committed starter seed: one Entity, one Facet, one Site, one Page, one representation.
const STARTER_MANIFEST: &[u8] =
    include_bytes!("../../../examples/starter/content/publication.json");
/// The one real starter body. Its declared JSON alternative is intentionally absent.
const STARTER_MARKDOWN: &[u8] =
    include_bytes!("../../../examples/starter/content/pages/main-home.en-001.md");

/// The committed expanded synthetic fixture: two Facets, three Sites, four Pages, six
/// representations.
const EXPANDED_MANIFEST: &str = include_str!("fixtures/r02c/expanded/content/publication.json");
const BALI_HOME_EN: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/bali-home.en-001.md");
const BALI_HOME_ID: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/bali-home.id-id.md");
const BALI_ABOUT_EN: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/bali-about.en-001.md");
const BALI_ABOUT_ID: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/bali-about.id-id.json");
const JAKARTA_HOME_EN: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/jakarta-home.en-001.md");
const COMPANY_HOME_DE: &[u8] =
    include_bytes!("fixtures/r02c/expanded/content/pages/company-home.de-de.md");

/// The six declared bodies in authored declaration order.
const EXPANDED_BODIES: [(&str, SourceCandidateKind, &[u8]); 6] = [
    (
        "pages/bali-home.en-001.md",
        SourceCandidateKind::Markdown,
        BALI_HOME_EN,
    ),
    (
        "pages/bali-home.id-id.md",
        SourceCandidateKind::Markdown,
        BALI_HOME_ID,
    ),
    (
        "pages/bali-about.en-001.md",
        SourceCandidateKind::Markdown,
        BALI_ABOUT_EN,
    ),
    (
        "pages/bali-about.id-id.json",
        SourceCandidateKind::Json,
        BALI_ABOUT_ID,
    ),
    (
        "pages/jakarta-home.en-001.md",
        SourceCandidateKind::Markdown,
        JAKARTA_HOME_EN,
    ),
    (
        "pages/company-home.de-de.md",
        SourceCandidateKind::Markdown,
        COMPANY_HOME_DE,
    ),
];

/// How many alternative names a scratch tree may try before failing closed.
const FIXTURE_ATTEMPTS: usize = 16;

/// The directory a scratch tree is created in.
///
/// On Linux every scratch tree is created on the already configured R03 snapshot fixture
/// filesystem, because the accepted reader takes an immutable clone. A missing value is a loud
/// failure, never a quiet run on an unverified filesystem. Every other host keeps the runner
/// temp directory.
fn fixture_parent() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        match std::env::var_os("SOLO_TEST_SOURCE_FS_ROOT") {
            Some(root) if !root.is_empty() => PathBuf::from(root),
            _ => panic!(
                "SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; \
                 refusing to run the real-filesystem suite on a filesystem that was never \
                 verified to support an immutable clone"
            ),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::env::temp_dir()
    }
}

/// A task-owned content tree, removed on drop.
struct Scratch {
    base: PathBuf,
}

impl Scratch {
    fn candidate(tag: &str, attempt: usize) -> PathBuf {
        fixture_parent().join(format!("solo-r02c-{}-{tag}-{attempt}", std::process::id()))
    }

    /// Claims a fresh scratch tree, skipping — never reusing — an existing name.
    fn new(tag: &str) -> Self {
        for attempt in 0..FIXTURE_ATTEMPTS {
            let base = Self::candidate(tag, attempt);
            match fs::create_dir(&base) {
                Ok(()) => {
                    let guard = Self { base };
                    guard.layout();
                    return guard;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("a scratch directory could not be created: {error}"),
            }
        }
        panic!(
            "{FIXTURE_ATTEMPTS} scratch names were already taken; \
             nothing was removed and nothing was reused"
        )
    }

    fn layout(&self) {
        fs::create_dir(self.content()).expect("create the scratch content directory");
        fs::create_dir(self.base.join("staging")).expect("create the scratch staging directory");
        fs::create_dir(self.content().join("pages")).expect("create the scratch pages directory");
    }

    fn content(&self) -> PathBuf {
        self.base.join("content")
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.content().join(relative)
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        if let Some(parent) = self.path(relative).parent() {
            fs::create_dir_all(parent).expect("create a scratch parent directory");
        }
        fs::write(self.path(relative), bytes).expect("write a scratch body");
    }

    fn staging_root(&self) -> StagingRootCapability {
        StagingRootCapability::adopt(&self.base.join("staging"))
            .expect("adopt the scratch staging root")
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
        // Only ever removes a tree this value created exclusively.
        let _ = fs::remove_dir_all(&self.base);
    }
}

/// A scratch tree holding exactly the six committed expanded bodies.
fn expanded_scratch(tag: &str) -> Scratch {
    let scratch = Scratch::new(tag);
    for (path, _, bytes) in EXPANDED_BODIES {
        scratch.write(path, bytes);
    }
    scratch
}

/// Parses bytes that must be accepted.
fn accepted(bytes: &[u8]) -> ParsedManifestCandidateV1 {
    match parse_manifest_bytes(bytes) {
        Ok(parsed) => parsed,
        Err(error) => panic!("the committed manifest must parse, refused with {error}"),
    }
}

/// Parses bytes that must be refused and returns the registered refusal kind.
fn refused(bytes: &[u8]) -> ManifestErrorKind {
    match parse_manifest_bytes(bytes) {
        Ok(parsed) => panic!(
            "the malformed bytes must be refused, accepted with {} nodes",
            parsed.node_count()
        ),
        Err(error) => {
            assert_eq!(error.code(), "PREFLIGHT_MANIFEST_INVALID");
            error.kind()
        }
    }
}

/// Replaces one exactly-once substring of a checked-in manifest copy.
fn mutated(source: &str, from: &str, to: &str) -> String {
    assert_eq!(
        source.matches(from).count(),
        1,
        "the mutation anchor {from} must occur exactly once"
    );
    source.replace(from, to)
}

/// Rewrites the locale of the unlisted Bali Home representation, anchored on its route.
fn with_unlisted_locale(source: &str, locale: &str) -> String {
    let anchor = source
        .find("/id-id/Hidden7A")
        .expect("the unlisted route is present");
    let window_start = anchor.saturating_sub(240);
    let window = &source[window_start..anchor];
    let patched = window.replacen(
        "\"locale\": \"id-ID\"",
        &format!("\"locale\": \"{locale}\""),
        1,
    );
    assert_ne!(
        patched, window,
        "the unlisted representation locale must be present"
    );
    format!(
        "{}{}{}",
        &source[..window_start],
        patched,
        &source[anchor..]
    )
}

/// A compact manifest whose `facets` member is supplied by the caller.
fn with_facets(facets: &str) -> String {
    format!(
        r#"{{ "schemaVersion": "1.0.0", "propertyRegistryVersion": "1.0.0",
             "platformAssetLibraryVersion": "1.0.0",
             "base": {{ "canonical": "https://example.org/site" }},
             "entity": {{ "id": "example", "label": "Example", "facets": {facets} }} }}"#
    )
}

/// Builds the pure domain draft from the already parsed declarations.
///
/// The fixture authors canonical full locale keys, so wrapping them here asserts exactly that
/// and never canonicalises, folds or completes a value.
fn draft_from(parsed: &ParsedManifestCandidateV1) -> EfsOwnershipDraft {
    let entity = EntityIdV1::parse(parsed.entity().id().as_str())
        .expect("the parsed Entity identifier is already canonical");
    let mut draft = EfsOwnershipDraft::new(entity);
    for facet in parsed.entity().facets() {
        let mut facet_draft = FacetDraft::new(
            FacetIdV1::parse(facet.id().as_str()).expect("a canonical Facet identifier"),
        );
        for site in facet.sites() {
            let mut site_draft = SiteDraft::new(
                SiteIdV1::parse(site.id().as_str()).expect("a canonical Site identifier"),
            );
            for page in site.pages() {
                let mut page_draft = PageDraft::new(
                    PageIdV1::parse(page.id().as_str()).expect("a canonical Page identifier"),
                );
                for representation in page.representations() {
                    page_draft.add_representation(
                        LocaleKeyV1::from_already_canonical(representation.locale())
                            .expect("the fixture authors canonical full locale keys"),
                    );
                }
                site_draft.add_page(page_draft);
            }
            facet_draft.add_site(site_draft);
        }
        draft.add_facet(facet_draft);
    }
    draft
}

fn graph_from(parsed: &ParsedManifestCandidateV1) -> EfsOwnershipGraph {
    draft_from(parsed)
        .build()
        .expect("the committed fixture is a valid one-Entity structure")
}

fn facet_id(value: &str) -> FacetIdV1 {
    FacetIdV1::parse(value).expect("a canonical Facet identifier")
}

fn site_id(value: &str) -> SiteIdV1 {
    SiteIdV1::parse(value).expect("a canonical Site identifier")
}

fn page_id(value: &str) -> PageIdV1 {
    PageIdV1::parse(value).expect("a canonical Page identifier")
}

#[test]
fn the_starter_seed_declares_exactly_one_structure_and_one_full_locale() {
    let parsed = accepted(STARTER_MANIFEST);

    assert_eq!(parsed.logical_identity(), MANIFEST_LOGICAL_IDENTITY);
    assert_eq!(parsed.entity().id().as_str(), "example");
    assert_eq!(parsed.entity().facets().len(), 1);
    assert_eq!(parsed.entity().facets()[0].id().as_str(), "main");
    assert_eq!(parsed.entity().facets()[0].sites().len(), 1);

    let site = &parsed.entity().facets()[0].sites()[0];
    assert_eq!(site.id().as_str(), "main");
    assert_eq!(site.default_locale(), "en-001");
    assert_eq!(
        site.language_preferences().to_vec(),
        vec![("en".to_owned(), "en-001".to_owned())]
    );
    assert_eq!(site.pages().len(), 1);

    let page = &site.pages()[0];
    assert_eq!(page.id().as_str(), "home");
    assert_eq!(page.representations().len(), 1);

    let representation = &page.representations()[0];
    assert_eq!(representation.locale(), "en-001");
    assert_eq!(representation.route().path(), "/en-001/home");

    let candidates = representation.source_candidates();
    assert_eq!(
        candidates.json().map(|value| value.as_str()),
        Some("pages/main-home.en-001.json")
    );
    assert_eq!(
        candidates.markdown().map(|value| value.as_str()),
        Some("pages/main-home.en-001.md")
    );

    assert_eq!(parsed.node_count(), 5);
    assert_eq!(parsed.page_count(), 1);
    assert_eq!(parsed.representation_count(), 1);

    // The declaration is inert: parsing proves shape only, never publication authority.
    assert!(!parsed.is_publication_authorized());

    let landing = parsed
        .base()
        .landing()
        .expect("the starter seed declares its landing target");
    assert_eq!(landing.entity(), "example");
    assert_eq!(landing.facet(), "main");
    assert_eq!(landing.site(), "main");
    assert_eq!(landing.page(), "home");
    assert_eq!(landing.locale(), None);
    assert_eq!(parsed.base().canonical(), "https://example.org/site");

    // The selected body really exists in the repository and is readable authored Markdown.
    assert!(!STARTER_MARKDOWN.is_empty());
    assert!(STARTER_MARKDOWN.starts_with(b"# "));
}

#[test]
fn the_starter_seed_stages_exactly_its_declared_markdown_body() {
    let scratch = Scratch::new("starter");
    scratch.write("pages/main-home.en-001.md", STARTER_MARKDOWN);
    assert!(
        !scratch.path("pages/main-home.en-001.json").exists(),
        "the declared preferred JSON must genuinely be absent in this scratch tree"
    );
    let capability = scratch.capability();

    let candidate =
        assemble_declared_source_set_v1(STARTER_MANIFEST, &capability, &scratch.staging_root())
            .expect("a confirmed absent preferred JSON permits the declared Markdown body");

    assert_eq!(candidate.entity(), "example");
    assert_eq!(candidate.len(), 1);
    let member = candidate.get(0).expect("the single declaration");
    assert_eq!(member.member(), SourceCandidateKind::Markdown);
    assert_eq!(member.logical_path(), "pages/main-home.en-001.md");
    assert_eq!(member.lineage().entity(), "example");
    assert_eq!(member.lineage().facet(), "main");
    assert_eq!(member.lineage().site(), "main");
    assert_eq!(member.lineage().page(), "home");
    assert_eq!(member.lineage().locale(), "en-001");

    let staged = candidate
        .member_bytes(0)
        .expect("the staged body is readable")
        .expect("the candidate holds this member");
    assert_eq!(staged.as_slice(), STARTER_MARKDOWN);
    assert_eq!(
        member.byte_length(),
        u64::try_from(STARTER_MARKDOWN.len()).expect("a short authored body")
    );
    assert_eq!(member.fingerprint(), fingerprint_of(&staged));

    // A staged candidate is not publication authority and not a filesystem snapshot.
    assert!(!member.is_publication_authorized());
    assert!(!member.is_verified_filesystem_snapshot());
    assert!(!candidate.is_publication_authorized());
    assert!(!candidate.is_verified_filesystem_snapshot());
}

#[test]
fn a_present_preferred_json_wins_without_changing_page_identity() {
    let scratch = Scratch::new("starter-json");
    scratch.write("pages/main-home.en-001.md", STARTER_MARKDOWN);
    let json = br#"{"contentVersion":"1.0.0","title":"Home","blocks":[{}]}"#;
    scratch.write("pages/main-home.en-001.json", json);
    let capability = scratch.capability();

    let candidate =
        assemble_declared_source_set_v1(STARTER_MANIFEST, &capability, &scratch.staging_root())
            .expect("a valid present preferred JSON is selected");

    assert_eq!(candidate.len(), 1);
    let member = candidate.get(0).expect("the single declaration");
    assert_eq!(member.member(), SourceCandidateKind::Json);
    assert_eq!(member.logical_path(), "pages/main-home.en-001.json");
    // Identity is the declaration, not the chosen format: it is unchanged by the upgrade.
    assert_eq!(member.lineage().entity(), "example");
    assert_eq!(member.lineage().facet(), "main");
    assert_eq!(member.lineage().site(), "main");
    assert_eq!(member.lineage().page(), "home");
    assert_eq!(member.lineage().locale(), "en-001");

    let staged = candidate
        .member_bytes(0)
        .expect("the staged body is readable")
        .expect("the candidate holds this member");
    assert_eq!(staged.as_slice(), json);
}

#[test]
fn a_present_but_invalid_preferred_json_refuses_the_whole_starter_candidate() {
    let scratch = Scratch::new("starter-json-invalid");
    scratch.write("pages/main-home.en-001.md", STARTER_MARKDOWN);
    scratch.write(
        "pages/main-home.en-001.json",
        br#"{"contentVersion":"9.9.9","title":"Home","blocks":[{}]}"#,
    );
    let capability = scratch.capability();

    let refusal =
        assemble_declared_source_set_v1(STARTER_MANIFEST, &capability, &scratch.staging_root())
            .expect_err("a present-but-invalid preferred JSON must be refused");

    assert!(
        matches!(refusal, SourceSetRefusalV1::Body { declaration: 0, .. }),
        "expected a selected-body refusal, got {refusal}"
    );
    assert_eq!(refusal.declaration(), Some(0));
}

#[test]
fn the_expanded_fixture_declares_two_facets_three_sites_four_pages_and_six_representations() {
    let parsed = accepted(EXPANDED_MANIFEST.as_bytes());

    assert_eq!(parsed.entity().id().as_str(), "example");
    assert_eq!(parsed.node_count(), 16);
    assert_eq!(parsed.page_count(), 4);
    assert_eq!(parsed.representation_count(), 6);

    let facets = parsed.entity().facets();
    assert_eq!(facets.len(), 2);
    assert_eq!(facets[0].id().as_str(), "visitors");
    assert_eq!(facets[1].id().as_str(), "company");
    // The Entity location declaration is inert authored data in this stage.
    assert_eq!(
        parsed
            .entity()
            .properties()
            .expect("the Entity declares inert properties")
            .len(),
        1
    );

    let visitors = &facets[0];
    assert_eq!(visitors.sites().len(), 2);
    assert_eq!(visitors.sites()[0].id().as_str(), "bali");
    assert_eq!(visitors.sites()[0].default_locale(), "en-001");
    assert_eq!(
        visitors.sites()[0].language_preferences().to_vec(),
        vec![
            ("en".to_owned(), "en-001".to_owned()),
            ("id".to_owned(), "id-ID".to_owned())
        ]
    );
    assert_eq!(visitors.sites()[1].id().as_str(), "jakarta");
    assert_eq!(visitors.sites()[1].default_locale(), "en-001");

    let bali_pages = visitors.sites()[0].pages();
    assert_eq!(bali_pages.len(), 2);
    assert_eq!(bali_pages[0].id().as_str(), "home");
    assert_eq!(bali_pages[1].id().as_str(), "about");
    assert_eq!(
        bali_pages[0].visibility_default(),
        DeclaredVisibility::Public
    );

    // Page identifiers are scoped: `home` is a distinct ownership node under each Site.
    assert_eq!(visitors.sites()[1].pages()[0].id().as_str(), "home");
    assert_eq!(
        facets[1].sites()[0].pages()[0].id().as_str(),
        "home",
        "the Company Site reuses the Page identifier legitimately"
    );

    // Authored declaration order of the four Bali representations with inert route metadata.
    let mut declared = Vec::new();
    for page in bali_pages {
        for representation in page.representations() {
            let candidates = representation.source_candidates();
            declared.push((
                representation.locale().to_owned(),
                representation.route().path().to_owned(),
                candidates.markdown().map(|value| value.as_str().to_owned()),
                candidates.json().map(|value| value.as_str().to_owned()),
                representation.visibility(),
                representation.route().case_matching(),
            ));
        }
    }
    assert_eq!(
        declared,
        vec![
            (
                "en-001".to_owned(),
                "/en-001/home".to_owned(),
                Some("pages/bali-home.en-001.md".to_owned()),
                None,
                None,
                Some(DeclaredCaseMatching::Insensitive),
            ),
            (
                "id-ID".to_owned(),
                "/id-id/Hidden7A".to_owned(),
                Some("pages/bali-home.id-id.md".to_owned()),
                None,
                Some(DeclaredVisibility::Unlisted),
                Some(DeclaredCaseMatching::Insensitive),
            ),
            (
                "en-001".to_owned(),
                "/en-001/about".to_owned(),
                Some("pages/bali-about.en-001.md".to_owned()),
                None,
                None,
                Some(DeclaredCaseMatching::Insensitive),
            ),
            (
                "id-ID".to_owned(),
                "/id-id/TentangKami".to_owned(),
                None,
                Some("pages/bali-about.id-id.json".to_owned()),
                None,
                Some(DeclaredCaseMatching::Sensitive),
            ),
        ]
    );

    let jakarta = &visitors.sites()[1].pages()[0].representations()[0];
    assert_eq!(jakarta.locale(), "en-001");
    assert_eq!(jakarta.route().path(), "/en-001/jakarta");
    assert_eq!(
        jakarta
            .source_candidates()
            .markdown()
            .map(|value| value.as_str()),
        Some("pages/jakarta-home.en-001.md")
    );
    let company = &facets[1].sites()[0].pages()[0].representations()[0];
    assert_eq!(company.locale(), "de-DE");
    assert_eq!(company.route().path(), "/de-de/unternehmen");
    assert_eq!(
        company
            .source_candidates()
            .markdown()
            .map(|value| value.as_str()),
        Some("pages/company-home.de-de.md")
    );

    let landing = parsed
        .base()
        .landing()
        .expect("the expanded fixture declares its landing target");
    assert_eq!(landing.facet(), "visitors");
    assert_eq!(landing.site(), "bali");
    assert_eq!(landing.page(), "home");
    assert!(!parsed.is_publication_authorized());
}

#[test]
fn the_expanded_fixture_stages_every_declared_body_byte_identically() {
    let scratch = expanded_scratch("expanded-stage");
    let capability = scratch.capability();

    let candidate = assemble_declared_source_set_v1(
        EXPANDED_MANIFEST.as_bytes(),
        &capability,
        &scratch.staging_root(),
    )
    .expect("every declared representation has its authored body");

    assert_eq!(candidate.entity(), "example");
    assert_eq!(candidate.len(), EXPANDED_BODIES.len());

    for (index, (path, kind, body)) in EXPANDED_BODIES.iter().enumerate() {
        let member = candidate.get(index).expect("one member per declaration");
        assert_eq!(member.logical_path(), *path);
        assert_eq!(member.member(), *kind);
        let staged = candidate
            .member_bytes(index)
            .expect("the staged body is readable")
            .expect("the candidate holds this member");
        assert_eq!(
            staged.as_slice(),
            *body,
            "member {index} must be byte-identical"
        );
        assert_eq!(member.fingerprint(), fingerprint_of(&staged));
        assert_eq!(
            member.byte_length(),
            u64::try_from(body.len()).expect("a short authored body")
        );
        assert!(!member.is_publication_authorized());
        assert!(!member.is_verified_filesystem_snapshot());
    }
    assert!(candidate.get(EXPANDED_BODIES.len()).is_none());
}

#[test]
fn the_expanded_fixture_builds_one_entity_graph_and_resolves_every_lineage() {
    let parsed = accepted(EXPANDED_MANIFEST.as_bytes());
    let graph = graph_from(&parsed);

    assert_eq!(graph.entity().as_str(), "example");
    assert_eq!(graph.facets().len(), 2);
    assert_eq!(graph.facets()[0].id().as_str(), "visitors");
    assert_eq!(graph.facets()[0].sites().len(), 2);
    assert_eq!(graph.facets()[0].sites()[1].id().as_str(), "jakarta");
    assert_eq!(graph.facets()[1].id().as_str(), "company");

    let home = page_id("home");
    let about = page_id("about");
    let bali = site_id("bali");
    let jakarta = site_id("jakarta");
    let global = site_id("global");
    let visitors = facet_id("visitors");
    let company = facet_id("company");

    let bali_home = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &visitors,
            site: &bali,
            page: &home,
        })
        .expect("the Bali Home Page resolves");
    let memberships: Vec<&str> = bali_home
        .representations()
        .iter()
        .map(LocaleKeyV1::as_str)
        .collect();
    assert_eq!(memberships, vec!["en-001", "id-ID"]);

    let jakarta_home = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &visitors,
            site: &jakarta,
            page: &home,
        })
        .expect("the Jakarta Home Page resolves");
    assert_eq!(jakarta_home.representations().len(), 1);

    let company_home = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &company,
            site: &global,
            page: &home,
        })
        .expect("the Company Home Page resolves");
    assert_eq!(company_home.representations()[0].as_str(), "de-DE");

    graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &visitors,
            site: &bali,
            page: &about,
        })
        .expect("the Bali About Page resolves");

    // Every declared representation resolves through its complete lineage and locale key.
    let en = LocaleKeyV1::from_already_canonical("en-001").expect("a canonical full locale");
    let id = LocaleKeyV1::from_already_canonical("id-ID").expect("a canonical full locale");
    let de = LocaleKeyV1::from_already_canonical("de-DE").expect("a canonical full locale");
    for (facet, site, page, locale) in [
        (&visitors, &bali, &home, &en),
        (&visitors, &bali, &home, &id),
        (&visitors, &bali, &about, &en),
        (&visitors, &bali, &about, &id),
        (&visitors, &jakarta, &home, &en),
        (&company, &global, &home, &de),
    ] {
        graph
            .resolve_representation(
                EfsPageTarget {
                    entity: graph.entity(),
                    facet,
                    site,
                    page,
                },
                locale,
            )
            .expect("the declared representation resolves");
    }
}

#[test]
fn wrong_facet_site_page_or_entity_never_resolves_and_never_borrows_a_sibling() {
    let parsed = accepted(EXPANDED_MANIFEST.as_bytes());
    let graph = graph_from(&parsed);

    let visitors = facet_id("visitors");
    let bali = site_id("bali");
    let global = site_id("global");
    let home = page_id("home");
    let guide = page_id("guide");
    let shop = facet_id("shop");
    let other_entity = EntityIdV1::parse("other-entity").expect("a canonical Entity identifier");
    let de = LocaleKeyV1::from_already_canonical("de-DE").expect("a canonical full locale");

    let entity_refusal = graph
        .resolve_page(EfsPageTarget {
            entity: &other_entity,
            facet: &visitors,
            site: &bali,
            page: &home,
        })
        .expect_err("a second Entity is never this graph");
    assert_eq!(entity_refusal.kind(), EfsLineageErrorKind::UnknownEntity);

    let facet_refusal = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &shop,
            site: &bali,
            page: &home,
        })
        .expect_err("an undeclared Facet never resolves");
    assert_eq!(facet_refusal.kind(), EfsLineageErrorKind::UnknownFacet);

    let site_refusal = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &visitors,
            site: &global,
            page: &home,
        })
        .expect_err("a Site owned by another Facet never resolves through this one");
    assert_eq!(site_refusal.kind(), EfsLineageErrorKind::UnknownSite);
    assert_eq!(site_refusal.code(), "PREFLIGHT_EFS_LINEAGE_UNRESOLVED");

    let page_refusal = graph
        .resolve_page(EfsPageTarget {
            entity: graph.entity(),
            facet: &visitors,
            site: &bali,
            page: &guide,
        })
        .expect_err("an undeclared Page never resolves");
    assert_eq!(page_refusal.kind(), EfsLineageErrorKind::UnknownPage);

    let representation_refusal = graph
        .resolve_representation(
            EfsPageTarget {
                entity: graph.entity(),
                facet: &visitors,
                site: &bali,
                page: &home,
            },
            &de,
        )
        .expect_err("a locale the Page does not declare never resolves");
    assert_eq!(
        representation_refusal.kind(),
        EfsLineageErrorKind::UnknownRepresentation
    );

    // A wrong ancestor is refused rather than swapped for a sibling that would have matched.
    assert_eq!(graph.facets()[1].id().as_str(), "company");
    assert_eq!(graph.facets()[0].sites()[1].id().as_str(), "jakarta");
}

#[test]
fn a_bare_language_key_is_never_a_representation_membership() {
    for bare in ["en", "id", "de"] {
        let refusal = LocaleKeyV1::from_already_canonical(bare)
            .expect_err("a bare primary subtag is never a full region-qualified locale");
        assert_eq!(refusal.kind(), LocaleKeyErrorKind::NotFullLocale);
    }
    // The committed fixture stays inside the accepted full-locale contract.
    let parsed = accepted(EXPANDED_MANIFEST.as_bytes());
    for facet in parsed.entity().facets() {
        for site in facet.sites() {
            for page in site.pages() {
                for representation in page.representations() {
                    let key = LocaleKeyV1::from_already_canonical(representation.locale())
                        .expect("every authored key is a canonical full locale");
                    assert!(
                        key.as_str().contains('-'),
                        "a membership key must stay region-qualified"
                    );
                }
            }
        }
    }
}

#[test]
fn structural_negatives_around_the_single_entity_shape_are_refused() {
    let source = EXPANDED_MANIFEST;

    // An invented root-level Entity collection is outside the closed five-key root.
    assert_eq!(
        refused(
            mutated(
                source,
                "\"schemaVersion\": \"1.0.0\",",
                "\"entities\": [],\n  \"schemaVersion\": \"1.0.0\",",
            )
            .as_bytes()
        ),
        ManifestErrorKind::UnknownKey
    );

    // A second Entity cannot be expressed: the single `entity` member takes one object.
    assert_eq!(
        refused(TWO_ENTITIES.as_bytes()),
        ManifestErrorKind::NotAnObject
    );

    // An unauthorized parent hint is not part of the closed Page shape.
    assert_eq!(
        refused(
            mutated(
                source,
                "\"label\": \"About\",",
                "\"label\": \"About\", \"parent\": \"home\",",
            )
            .as_bytes()
        ),
        ManifestErrorKind::UnknownKey
    );

    // Duplicate scoped identifiers, each refused inside its own scope.
    assert_eq!(
        refused(mutated(source, "\"id\": \"company\"", "\"id\": \"visitors\"").as_bytes()),
        ManifestErrorKind::DuplicateScopedId
    );
    assert_eq!(
        refused(mutated(source, "\"id\": \"jakarta\"", "\"id\": \"bali\"").as_bytes()),
        ManifestErrorKind::DuplicateScopedId
    );
    assert_eq!(
        refused(mutated(source, "\"id\": \"about\"", "\"id\": \"home\"").as_bytes()),
        ManifestErrorKind::DuplicateScopedId
    );
    assert_eq!(
        refused(with_unlisted_locale(source, "en-001").as_bytes()),
        ManifestErrorKind::DuplicateLocale
    );

    // Required-but-empty collections are refused instead of being seeded implicitly.
    assert_eq!(
        refused(with_facets("[]").as_bytes()),
        ManifestErrorKind::EmptyCollection
    );
    let facet_without_site = r#"{ "id": "visitors", "label": "Visitors", "sites": [] }"#;
    assert_eq!(
        refused(with_facets(&format!("[{facet_without_site}]")).as_bytes()),
        ManifestErrorKind::EmptyCollection
    );
    let site_without_page =
        r#"{ "id": "bali", "label": "Bali", "defaultLocale": "en-001", "pages": [] }"#;
    assert_eq!(
        refused(
            with_facets(&format!(
                r#"[{{ "id": "visitors", "label": "Visitors", "sites": [{site_without_page}] }}]"#
            ))
            .as_bytes()
        ),
        ManifestErrorKind::EmptyCollection
    );
}

/// A test-owned manifest whose single `entity` member carries two Entities.
///
/// The one-Entity contract cannot be expressed as a collection, so this shape is refused as a
/// wrong JSON type rather than silently taking the first Entity.
const TWO_ENTITIES: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.org/site" },
  "entity": [
    { "id": "example", "label": "Example", "facets": [] },
    { "id": "second", "label": "Second", "facets": [] }
  ]
}"#;

#[test]
fn an_undeclared_body_is_neither_selected_nor_staged() {
    let scratch = expanded_scratch("undeclared");
    scratch.write(
        "pages/undeclared.en-001.md",
        b"# Undeclared\n\nThis body is not named by any declaration.\n",
    );
    let capability = scratch.capability();

    let candidate = assemble_declared_source_set_v1(
        EXPANDED_MANIFEST.as_bytes(),
        &capability,
        &scratch.staging_root(),
    )
    .expect("declared bodies are present; an extra file changes nothing");

    assert_eq!(candidate.len(), EXPANDED_BODIES.len());
    for member in candidate.iter() {
        assert!(
            !member.logical_path().contains("undeclared"),
            "an undeclared body must never be selected"
        );
    }
    assert_eq!(candidate.staged_object_count(), 1);
}

#[test]
fn a_duplicate_declared_source_claim_is_refused_before_any_body_is_staged() {
    let scratch = expanded_scratch("duplicate-claim");
    let capability = scratch.capability();

    // The Company representation now names the Bali Home body: one logical path, two claims.
    let duplicated = mutated(
        EXPANDED_MANIFEST,
        "\"markdown\": \"pages/company-home.de-de.md\"",
        "\"markdown\": \"pages/bali-home.en-001.md\"",
    );
    assert_eq!(
        refused(duplicated.as_bytes()),
        ManifestErrorKind::DuplicateSourceClaim
    );

    let refusal = assemble_declared_source_set_v1(
        duplicated.as_bytes(),
        &capability,
        &scratch.staging_root(),
    )
    .expect_err("a duplicated logical claim is refused before staging begins");
    assert!(matches!(refusal, SourceSetRefusalV1::Manifest(_)));
    assert_eq!(refusal.code(), "PREFLIGHT_MANIFEST_INVALID");
}

#[test]
fn one_missing_selected_source_refuses_the_whole_expanded_set() {
    let scratch = Scratch::new("missing-body");
    for (path, _, bytes) in EXPANDED_BODIES {
        if path == "pages/company-home.de-de.md" {
            continue;
        }
        scratch.write(path, bytes);
    }
    let capability = scratch.capability();

    let refusal = assemble_declared_source_set_v1(
        EXPANDED_MANIFEST.as_bytes(),
        &capability,
        &scratch.staging_root(),
    )
    .expect_err("a missing selected source refuses the entire candidate");

    assert_eq!(refusal.code(), "SOURCE_NONE_EFFECTIVE");
    assert_eq!(
        refusal.declaration(),
        Some(EXPANDED_BODIES.len() - 1),
        "the refusal names the last declaration rather than publishing a partial sibling"
    );
}

#[test]
fn page_identity_is_independent_of_labels_base_routes_and_representation_count() {
    let baseline = graph_from(&accepted(EXPANDED_MANIFEST.as_bytes()));

    // Labels, the configured base and authored routes change; nothing else does.
    let mut renamed = mutated(
        EXPANDED_MANIFEST,
        "\"label\": \"Bali Site\"",
        "\"label\": \"Coastal Site\"",
    );
    renamed = mutated(
        &renamed,
        "\"label\": \"Example Entity\"",
        "\"label\": \"Renamed Entity\"",
    );
    renamed = mutated(
        &renamed,
        "\"https://example.org/visit\"",
        "\"https://elsewhere.example:8443/other-prefix\"",
    );
    renamed = mutated(&renamed, "\"/en-001/home\"", "\"/en/home-renamed\"");
    renamed = mutated(&renamed, "\"/de-de/unternehmen\"", "\"/de/firma\"");
    let renamed_parsed = accepted(renamed.as_bytes());
    let renamed_graph = graph_from(&renamed_parsed);

    let home = page_id("home");
    let about = page_id("about");
    let bali = site_id("bali");
    let jakarta = site_id("jakarta");
    let global = site_id("global");
    let visitors = facet_id("visitors");
    let company = facet_id("company");

    // The same qualified targets still resolve, so identity survived every such change.
    for (facet, site, page) in [
        (&visitors, &bali, &home),
        (&visitors, &bali, &about),
        (&visitors, &jakarta, &home),
        (&company, &global, &home),
    ] {
        renamed_graph
            .resolve_page(EfsPageTarget {
                entity: renamed_graph.entity(),
                facet,
                site,
                page,
            })
            .expect("identity comes from the declaration, not from a label, base or route");
    }

    let baseline_bali = baseline
        .resolve_page(EfsPageTarget {
            entity: baseline.entity(),
            facet: &visitors,
            site: &bali,
            page: &home,
        })
        .expect("the baseline Bali Home Page resolves");
    let renamed_bali = renamed_graph
        .resolve_page(EfsPageTarget {
            entity: renamed_graph.entity(),
            facet: &visitors,
            site: &bali,
            page: &home,
        })
        .expect("the renamed Bali Home Page resolves");
    // Representation membership is authored data and is unchanged by those edits.
    assert_eq!(
        baseline_bali
            .representations()
            .iter()
            .map(LocaleKeyV1::as_str)
            .collect::<Vec<&str>>(),
        renamed_bali
            .representations()
            .iter()
            .map(LocaleKeyV1::as_str)
            .collect::<Vec<&str>>()
    );

    // Membership differs only where it was authored differently: two keys on Bali Home and one
    // on Jakarta Home, although both Pages carry the identifier `home`.
    let baseline_jakarta = baseline
        .resolve_page(EfsPageTarget {
            entity: baseline.entity(),
            facet: &visitors,
            site: &jakarta,
            page: &home,
        })
        .expect("the Jakarta Home Page resolves");
    assert_eq!(baseline_bali.representations().len(), 2);
    assert_eq!(baseline_jakarta.representations().len(), 1);
    assert_eq!(renamed_parsed.representation_count(), 6);
}

#[test]
fn staging_budgets_and_evidence_hold_for_the_expanded_fixture() {
    let scratch = expanded_scratch("budgets");
    let capability = scratch.capability();
    let candidate = assemble_declared_source_set_v1(
        EXPANDED_MANIFEST.as_bytes(),
        &capability,
        &scratch.staging_root(),
    )
    .expect("the expanded fixture assembles");

    let total: u64 = EXPANDED_BODIES
        .iter()
        .map(|(_, _, body)| u64::try_from(body.len()).expect("a short authored body"))
        .sum();
    let largest = EXPANDED_BODIES
        .iter()
        .map(|(_, _, body)| u64::try_from(body.len()).expect("a short authored body"))
        .max()
        .expect("six declared bodies");

    assert_eq!(candidate.staged_bytes(), total);
    assert_eq!(candidate.max_staged_body_bytes(), largest);
    // The completed candidate retains no body bytes and holds one nameless staging object.
    assert_eq!(candidate.retained_body_bytes(), 0);
    assert_eq!(candidate.staged_object_count(), 1);
    assert!(
        candidate.max_live_body_bytes() <= 2 * 2 * 1024 * 1024,
        "peak live body bytes stay inside twice the accepted single-body bound"
    );

    // Qualified staging evidence is present for every declaration, in authored order.
    let lineages: Vec<(String, String, String, String, String)> = candidate
        .iter()
        .map(|member| {
            (
                member.lineage().entity().to_owned(),
                member.lineage().facet().to_owned(),
                member.lineage().site().to_owned(),
                member.lineage().page().to_owned(),
                member.lineage().locale().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        lineages,
        vec![
            (
                "example".to_owned(),
                "visitors".to_owned(),
                "bali".to_owned(),
                "home".to_owned(),
                "en-001".to_owned()
            ),
            (
                "example".to_owned(),
                "visitors".to_owned(),
                "bali".to_owned(),
                "home".to_owned(),
                "id-ID".to_owned()
            ),
            (
                "example".to_owned(),
                "visitors".to_owned(),
                "bali".to_owned(),
                "about".to_owned(),
                "en-001".to_owned()
            ),
            (
                "example".to_owned(),
                "visitors".to_owned(),
                "bali".to_owned(),
                "about".to_owned(),
                "id-ID".to_owned()
            ),
            (
                "example".to_owned(),
                "visitors".to_owned(),
                "jakarta".to_owned(),
                "home".to_owned(),
                "en-001".to_owned()
            ),
            (
                "example".to_owned(),
                "company".to_owned(),
                "global".to_owned(),
                "home".to_owned(),
                "de-DE".to_owned()
            ),
        ]
    );
}

#[test]
fn checked_in_fixtures_are_never_mutated_and_no_scratch_survives() {
    // Reading the real paths shows the suite only ever read the committed bytes.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expanded = manifest_dir.join("tests/fixtures/r02c/expanded/content");
    assert_eq!(
        fs::read(expanded.join("publication.json")).expect("the committed manifest is readable"),
        EXPANDED_MANIFEST.as_bytes()
    );
    for (path, _, body) in EXPANDED_BODIES {
        let on_disk = fs::read(expanded.join(path)).expect("the committed body is readable");
        assert_eq!(on_disk.as_slice(), body, "{path} must be unmodified");
    }
    let starter = manifest_dir.join("../../examples/starter/content");
    assert_eq!(
        fs::read(starter.join("publication.json")).expect("the committed seed is readable"),
        STARTER_MANIFEST
    );
    assert_eq!(
        fs::read(starter.join("pages/main-home.en-001.md"))
            .expect("the committed starter body is readable"),
        STARTER_MARKDOWN
    );

    // A scratch tree this test created is gone after drop, and it was never the fixture tree.
    let scratch = Scratch::new("cleanup");
    let base = scratch.base.clone();
    drop(scratch);
    assert!(
        !base.exists(),
        "task-owned scratch must not be retained after the test"
    );
}
