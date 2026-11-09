//! Real-adapter integration proof for the assembled source-stage candidate.
//!
//! Every case drives the actual accepted path — strict manifest parser, adopted content-root
//! capability, real confined source read, accepted JSON-first selection and selected-body
//! preflight — over task-owned synthetic fixtures on the real filesystem of whichever host runs
//! it. Nothing here touches Owner content, lists a directory to discover sources, or keeps a
//! fixture after the test.
//!
//! **Fixture ownership.** A fixture root is claimed with an **exclusive** `create_dir`; an
//! already-taken name is skipped and never removed, reset, adopted or written into, and the guard
//! exists before any child directory or body file, so `Drop` can only ever remove a tree this test
//! created. There is no destructive pre-clean and no deletion of unknown material.

use std::fs;
use std::path::PathBuf;

use solo_adapters::{
    ContentRootCapability, RootProvenanceV1, SourceSetRefusalV1, assemble_declared_source_set_v1,
};
use solo_application::fingerprint_of;
use solo_domain::SourceCandidateKind;

const TRUSTED: RootProvenanceV1 = RootProvenanceV1::OperatorDesignatedContentRoot;

/// How many alternative names a fixture may try before failing closed.
const FIXTURE_ATTEMPTS: usize = 16;

/// A JSON body that passes the accepted selected-body preflight.
fn json_body(title: &str) -> Vec<u8> {
    format!(r#"{{"contentVersion":"1.0.0","title":"{title}","blocks":[{{}}]}}"#).into_bytes()
}

/// A Markdown body that passes the accepted selected-body preflight.
fn markdown_body(title: &str) -> Vec<u8> {
    format!("# {title}\n\nBody copy.\n").into_bytes()
}

/// The directory a fixture tree is created in.
///
/// On Linux every fixture is created on the R03 snapshot fixture filesystem, because the reader
/// takes an immutable clone and a filesystem without reflink could only produce a refusal. A
/// missing value is a loud failure, never a quiet run on an unverified filesystem. Every other
/// host keeps the runner temp directory.
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
    /// The `attempt`-th candidate name for `tag` in this host's fixture parent.
    fn candidate(tag: &str, attempt: usize) -> PathBuf {
        fixture_parent().join(format!(
            "solo-source-set-{}-{tag}-{attempt}",
            std::process::id()
        ))
    }

    /// Claims a fresh fixture tree, skipping — never reusing — an existing name.
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
                Err(error) => panic!("a fixture directory could not be created: {error}"),
            }
        }
        panic!(
            "{FIXTURE_ATTEMPTS} fixture names were already taken; \
             nothing was removed and nothing was reused"
        )
    }

    /// Creates the child tree beneath a base this value already owns.
    fn layout(&self) {
        fs::create_dir(self.base.join("content")).expect("create the fixture content directory");
        fs::create_dir(self.content().join("pages")).expect("create the fixture pages directory");
    }

    fn content(&self) -> PathBuf {
        self.base.join("content")
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.content().join(relative)
    }

    /// Writes one body beneath the guarded tree.
    fn write(&self, relative: &str, bytes: &[u8]) {
        if let Some(parent) = self.path(relative).parent() {
            fs::create_dir_all(parent).expect("create a fixture parent directory");
        }
        fs::write(self.path(relative), bytes).expect("write a fixture body");
    }

    /// Creates a second directory entry for the same physical object.
    fn hard_link(&self, existing: &str, additional: &str) {
        fs::hard_link(self.path(existing), self.path(additional))
            .expect("create the fixture hardlink");
    }

    fn capability(&self) -> ContentRootCapability {
        let capability = ContentRootCapability::adopt_bootstrap_path(&self.content(), TRUSTED)
            .expect("adopt the fixture content root");
        assert_eq!(capability.provenance(), TRUSTED);
        capability
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // Only ever removes a tree this value created exclusively: a name that was already taken
        // is skipped by `new` and never reaches here.
        let _ = fs::remove_dir_all(&self.base);
    }
}

fn refusal_code(refusal: &SourceSetRefusalV1) -> &'static str {
    refusal.code()
}

/// One Entity, one Facet, one Site, one Page and three full-locale representations.
const THREE_LOCALES: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.test" },
  "entity": {
    "id": "acme",
    "label": "Acme",
    "facets": [
      { "id": "main", "label": "Main", "sites": [
        { "id": "main", "label": "Main site", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "representations": [
            { "locale": "en-001", "route": { "path": "/" }, "sourceCandidates": { "json": "pages/home.en-001.json" } },
            { "locale": "de-DE", "route": { "path": "/de/" }, "sourceCandidates": { "json": "pages/home.de-de.json" } },
            { "locale": "id-ID", "route": { "path": "/id/" }, "sourceCandidates": { "json": "pages/home.id-id.json" } }
          ] }
        ] }
      ] }
    ]
  }
}"#;

/// One Entity expanded to two Facets, three Sites, three Pages that reuse the id `home`, and a
/// locale reused across different Pages.
const EXPANDED: &str = r#"{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": { "canonical": "https://example.test" },
  "entity": {
    "id": "acme",
    "label": "Acme",
    "facets": [
      { "id": "main", "label": "Main", "sites": [
        { "id": "main", "label": "Main site", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Home", "representations": [
            { "locale": "en-001", "route": { "path": "/a/" }, "sourceCandidates": { "json": "pages/main-home.en-001.json" } },
            { "locale": "de-DE", "route": { "path": "/b/" }, "sourceCandidates": { "json": "pages/main-home.de-de.json" } }
          ] },
          { "id": "about", "label": "About", "representations": [
            { "locale": "en-001", "route": { "path": "/c/" }, "sourceCandidates": { "json": "pages/main-about.en-001.json" } }
          ] }
        ] },
        { "id": "shop", "label": "Shop site", "defaultLocale": "en-001", "pages": [
          { "id": "home", "label": "Shop home", "representations": [
            { "locale": "en-001", "route": { "path": "/d/" }, "sourceCandidates": { "json": "pages/shop-home.en-001.json" } }
          ] }
        ] }
      ] },
      { "id": "secondary", "label": "Secondary", "sites": [
        { "id": "main", "label": "Secondary site", "defaultLocale": "id-ID", "pages": [
          { "id": "home", "label": "Secondary home", "representations": [
            { "locale": "id-ID", "route": { "path": "/e/" }, "sourceCandidates": { "json": "pages/secondary-home.id-id.json" } }
          ] }
        ] }
      ] }
    ]
  }
}"#;

/// A single-declaration manifest, with the supplied `sourceCandidates` member for one Page.
fn one_page(candidates: &str) -> String {
    format!(
        r#"{{ "schemaVersion": "1.0.0", "propertyRegistryVersion": "1.0.0", "platformAssetLibraryVersion": "1.0.0",
             "base": {{ "canonical": "https://example.test" }},
             "entity": {{ "id": "acme", "label": "Acme", "facets": [
               {{ "id": "main", "label": "Main", "sites": [
                 {{ "id": "main", "label": "Main site", "defaultLocale": "en-001", "pages": [
                   {{ "id": "home", "label": "Home", "representations": [
                     {{ "locale": "en-001", "route": {{ "path": "/" }}, "sourceCandidates": {candidates} }} ] }} ] }} ] }} ] }} }}"#
    )
}

#[test]
fn every_declared_representation_is_assembled_with_its_lineage() {
    let scratch = Scratch::new("three-locales");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    scratch.write("pages/home.de-de.json", &json_body("Start"));
    scratch.write("pages/home.id-id.json", &json_body("Beranda"));
    let capability = scratch.capability();

    let candidate = assemble_declared_source_set_v1(THREE_LOCALES.as_bytes(), &capability)
        .expect("every declared representation is present and valid");

    assert_eq!(candidate.entity(), "acme");
    assert_eq!(candidate.len(), 3);
    assert!(!candidate.is_empty());

    let expected = [
        ("en-001", "pages/home.en-001.json", json_body("Home")),
        ("de-DE", "pages/home.de-de.json", json_body("Start")),
        ("id-ID", "pages/home.id-id.json", json_body("Beranda")),
    ];
    for (index, (locale, path, body)) in expected.iter().enumerate() {
        let member = candidate.get(index).expect("one member per declaration");
        assert_eq!(member.lineage().entity(), "acme");
        assert_eq!(member.lineage().facet(), "main");
        assert_eq!(member.lineage().site(), "main");
        assert_eq!(member.lineage().page(), "home");
        assert_eq!(member.lineage().locale(), *locale);
        assert_eq!(member.member(), SourceCandidateKind::Json);
        assert_eq!(member.logical_path(), *path);
        assert_eq!(member.bytes(), body.as_slice());
        assert_eq!(member.fingerprint(), fingerprint_of(member.bytes()));
        assert_eq!(member.body().kind(), SourceCandidateKind::Json);
        assert!(!member.is_publication_authorized());
        assert!(!member.is_verified_filesystem_snapshot());
    }
    assert!(candidate.get(3).is_none());
    assert!(!candidate.is_publication_authorized());
    assert!(!candidate.is_verified_filesystem_snapshot());
}

#[test]
fn an_expanded_manifest_assembles_every_representation_exactly_once_in_order() {
    let scratch = Scratch::new("expanded");
    let bodies = [
        ("pages/main-home.en-001.json", "Home"),
        ("pages/main-home.de-de.json", "Start"),
        ("pages/main-about.en-001.json", "About"),
        ("pages/shop-home.en-001.json", "Shop"),
        ("pages/secondary-home.id-id.json", "Beranda"),
    ];
    for (path, title) in bodies {
        scratch.write(path, &json_body(title));
    }
    let capability = scratch.capability();

    let candidate = assemble_declared_source_set_v1(EXPANDED.as_bytes(), &capability)
        .expect("an expanded single-Entity manifest assembles completely");
    assert_eq!(candidate.len(), bodies.len());

    // Declared order, one member per declaration, with the qualified lineage that keeps the three
    // different `home` Pages apart.
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
                "acme".to_owned(),
                "main".to_owned(),
                "main".to_owned(),
                "home".to_owned(),
                "en-001".to_owned()
            ),
            (
                "acme".to_owned(),
                "main".to_owned(),
                "main".to_owned(),
                "home".to_owned(),
                "de-DE".to_owned()
            ),
            (
                "acme".to_owned(),
                "main".to_owned(),
                "main".to_owned(),
                "about".to_owned(),
                "en-001".to_owned()
            ),
            (
                "acme".to_owned(),
                "main".to_owned(),
                "shop".to_owned(),
                "home".to_owned(),
                "en-001".to_owned()
            ),
            (
                "acme".to_owned(),
                "secondary".to_owned(),
                "main".to_owned(),
                "home".to_owned(),
                "id-ID".to_owned()
            ),
        ]
    );

    // Every declared representation is present exactly once, and the order is the manifest's.
    let paths: Vec<&str> = candidate
        .iter()
        .map(|member| member.logical_path())
        .collect();
    let declared: Vec<&str> = bodies.iter().map(|(path, _)| *path).collect();
    assert_eq!(paths, declared);

    // Assembling the same inputs again yields the same order and count.
    let again = assemble_declared_source_set_v1(EXPANDED.as_bytes(), &capability)
        .expect("a second attempt over unchanged sources must also complete");
    assert_eq!(again.len(), candidate.len());
    let again_paths: Vec<&str> = again.iter().map(|member| member.logical_path()).collect();
    assert_eq!(again_paths, declared);
}

#[test]
fn a_declared_json_body_wins_and_an_invalid_unselected_markdown_is_never_read() {
    let scratch = Scratch::new("json-wins");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    // Bytes the accepted Markdown preflight would refuse. If the unselected body were read and
    // preflighted, this test could not complete.
    scratch.write("pages/home.en-001.md", &[0xff, 0xfe, 0x00, 0x41]);
    let capability = scratch.capability();

    let manifest =
        one_page(r#"{ "json": "pages/home.en-001.json", "markdown": "pages/home.en-001.md" }"#);
    let candidate = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect("a present JSON body is chosen and the Markdown body is never read");

    assert_eq!(candidate.len(), 1);
    let member = candidate.get(0).expect("the single declaration");
    assert_eq!(member.member(), SourceCandidateKind::Json);
    assert_eq!(member.logical_path(), "pages/home.en-001.json");
    assert_eq!(member.bytes(), json_body("Home").as_slice());
}

#[test]
fn markdown_is_selected_only_when_the_declared_json_is_confirmed_absent() {
    let scratch = Scratch::new("json-absent");
    scratch.write("pages/home.en-001.md", &markdown_body("Home"));
    let capability = scratch.capability();

    let manifest =
        one_page(r#"{ "json": "pages/home.en-001.json", "markdown": "pages/home.en-001.md" }"#);
    let candidate = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect("confirmed JSON absence permits the declared Markdown body");

    let member = candidate.get(0).expect("the single declaration");
    assert_eq!(member.member(), SourceCandidateKind::Markdown);
    assert_eq!(member.logical_path(), "pages/home.en-001.md");
    assert_eq!(member.bytes(), markdown_body("Home").as_slice());
}

#[test]
fn a_present_but_invalid_json_body_is_refused_with_no_markdown_fallback() {
    let scratch = Scratch::new("json-invalid");
    scratch.write(
        "pages/home.en-001.json",
        br#"{"contentVersion":"9.9.9","title":"Home","blocks":[{}]}"#,
    );
    scratch.write("pages/home.en-001.md", &markdown_body("Home"));
    let capability = scratch.capability();

    let manifest =
        one_page(r#"{ "json": "pages/home.en-001.json", "markdown": "pages/home.en-001.md" }"#);
    let refusal = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect_err("an invalid present JSON body must be refused, never replaced by Markdown");

    assert!(
        matches!(refusal, SourceSetRefusalV1::Body { declaration: 0, .. }),
        "expected a selected-body refusal, got {refusal}"
    );
}

#[test]
fn one_invalid_representation_refuses_the_whole_set() {
    let scratch = Scratch::new("one-invalid");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    scratch.write(
        "pages/home.de-de.json",
        br#"{"contentVersion":"1.0.0","title":"Start","blocks":[]}"#,
    );
    scratch.write("pages/home.id-id.json", &json_body("Beranda"));
    let capability = scratch.capability();

    let refusal = assemble_declared_source_set_v1(THREE_LOCALES.as_bytes(), &capability)
        .expect_err("one invalid representation must refuse the entire set");

    assert!(
        matches!(refusal, SourceSetRefusalV1::Body { declaration: 1, .. }),
        "expected the second declaration to be the refused body, got {refusal}"
    );
    assert_eq!(refusal.declaration(), Some(1));
    // Nothing was returned, so the two valid representations cannot be observed as a candidate.
    assert!(refusal_code(&refusal).starts_with("PREFLIGHT_"));
}

#[test]
fn two_declarations_hardlinked_to_one_object_are_refused_before_any_body_is_read() {
    let scratch = Scratch::new("hardlink");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    scratch.hard_link("pages/home.en-001.json", "pages/home.de-de.json");
    scratch.write("pages/home.id-id.json", &json_body("Beranda"));
    let capability = scratch.capability();

    let refusal = assemble_declared_source_set_v1(THREE_LOCALES.as_bytes(), &capability)
        .expect_err("two representations sharing one physical object must be refused");

    assert!(
        matches!(
            refusal,
            SourceSetRefusalV1::Read {
                declaration: None,
                ..
            }
        ),
        "expected a set-level reservation refusal, got {refusal}"
    );
    assert_eq!(refusal_code(&refusal), "SOURCE_COLLISION");
}

#[test]
fn an_undeclared_body_is_never_searched_selected_or_included() {
    let scratch = Scratch::new("undeclared");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    scratch.write("pages/home.extra.json", &json_body("Undeclared"));
    let capability = scratch.capability();

    let manifest = one_page(r#"{ "json": "pages/home.en-001.json" }"#);
    let candidate = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect("the declared representation assembles");

    assert_eq!(candidate.len(), 1);
    assert_eq!(
        candidate
            .get(0)
            .map(|member| member.logical_path().to_owned()),
        Some("pages/home.en-001.json".to_owned())
    );
    // The undeclared body is untouched: nothing scanned, nothing consumed, nothing deleted.
    assert_eq!(
        fs::read(scratch.path("pages/home.extra.json")).expect("the undeclared body still exists"),
        json_body("Undeclared")
    );
}

#[test]
fn owned_bytes_and_identity_survive_a_change_to_the_source_after_assembly() {
    let scratch = Scratch::new("owned");
    scratch.write("pages/home.en-001.json", &json_body("Home"));
    let capability = scratch.capability();

    let manifest = one_page(r#"{ "json": "pages/home.en-001.json" }"#);
    let candidate = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect("the declared representation assembles");
    let member = candidate.get(0).expect("the single declaration");
    let before_bytes = member.bytes().to_vec();
    let before_identity = member.identity();
    let before_fingerprint = member.fingerprint();

    // Replace the authored source after assembly. The candidate owns its bytes, so neither the
    // body nor the identity it reported can change underneath the caller.
    scratch.write("pages/home.en-001.json", &json_body("Replaced"));

    assert_eq!(member.bytes(), before_bytes.as_slice());
    assert_eq!(member.identity(), before_identity);
    assert_eq!(member.fingerprint(), before_fingerprint);
    assert_eq!(member.fingerprint(), fingerprint_of(member.bytes()));
}

#[test]
fn a_refused_manifest_is_reported_before_anything_is_read() {
    let scratch = Scratch::new("bad-manifest");
    let capability = scratch.capability();

    let refusal = assemble_declared_source_set_v1(b"{ \"schemaVersion\": ", &capability)
        .expect_err("invalid manifest bytes must be refused");

    assert!(
        matches!(refusal, SourceSetRefusalV1::Manifest(_)),
        "expected a manifest refusal, got {refusal}"
    );
    assert_eq!(refusal.declaration(), None);
    assert_eq!(refusal_code(&refusal), "PREFLIGHT_MANIFEST_INVALID");
}

#[test]
fn a_manifest_declaring_one_locale_twice_is_refused_before_assembly() {
    // Two representations of one Page cannot both claim `en-001`. The accepted strict parser is
    // what refuses this, so the duplicate never reaches a read.
    let scratch = Scratch::new("duplicate-locale");
    let capability = scratch.capability();
    let manifest = r#"{ "schemaVersion": "1.0.0", "propertyRegistryVersion": "1.0.0", "platformAssetLibraryVersion": "1.0.0",
        "base": { "canonical": "https://example.test" },
        "entity": { "id": "acme", "label": "Acme", "facets": [
          { "id": "main", "label": "Main", "sites": [
            { "id": "main", "label": "Main site", "defaultLocale": "en-001", "pages": [
              { "id": "home", "label": "Home", "representations": [
                { "locale": "en-001", "route": { "path": "/a/" }, "sourceCandidates": { "json": "pages/home.en-001.json" } },
                { "locale": "en-001", "route": { "path": "/b/" }, "sourceCandidates": { "json": "pages/home.alt.json" } }
              ] } ] } ] } ] } }"#;

    let refusal = assemble_declared_source_set_v1(manifest.as_bytes(), &capability)
        .expect_err("a duplicated locale must be refused");

    assert!(
        matches!(refusal, SourceSetRefusalV1::Manifest(_)),
        "expected the strict parser to refuse the duplicate, got {refusal}"
    );
}
