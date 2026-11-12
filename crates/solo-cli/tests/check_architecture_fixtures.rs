//! Ordinary `#[test]` gate for the architecture checker, run by
//! `cargo test --workspace --all-targets --locked`.
//!
//! This target is the reason a violation can never be "only reported": the real tree must
//! satisfy the boundary contract, every forbidden fixture must be refused by its expected
//! category, and every legal twin must be accepted — as ordinary test cases.

mod architecture_checks;

use std::path::{Path, PathBuf};

use architecture_checks::graph::TargetKind;
use architecture_checks::source::{
    self, CheckoutAnchor, DiscoveryFs, EntryKind, FixtureWorld, FsReader, RealDiscoveryFs,
};
use architecture_checks::{
    Category, EXIT_HARNESS_FAILURE, EXIT_LEGAL, EXIT_REFUSED, EXIT_USAGE, FIXTURES, FixtureOutcome,
    POSITIVES, Violation, check_real_tree, classify_fixture_result, evaluate_fixture,
    evaluate_positive, fixture_by_id, fixture_exit_code, fixture_ids, fixture_outcome_text,
    self_test,
};

#[test]
fn the_real_tree_satisfies_the_source_boundary_contract() {
    let report = check_real_tree().unwrap_or_else(|violations| {
        panic!("the real tree violates the boundary contract: {violations:#?}")
    });

    assert_eq!(report.packages, 4, "exactly four workspace packages");
    assert!(
        report.units >= 8,
        "every declared target must be checked, saw {}",
        report.units
    );
    assert!(
        report.files >= 14,
        "every reachable source file must be parsed, saw {}",
        report.files
    );
    assert!(report.edges >= 6, "the real graph must be non-trivial");
}

#[test]
fn every_forbidden_fixture_is_refused_by_its_expected_category() {
    for fixture in &FIXTURES {
        let found = evaluate_fixture(fixture.id)
            .unwrap_or_else(|| panic!("fixture `{}` is not registered", fixture.id));

        assert!(
            !found.is_empty(),
            "fixture `{}` ({}) was accepted: the guard is not load-bearing",
            fixture.id,
            fixture.class
        );
        assert!(
            found
                .iter()
                .any(|violation| violation.category == fixture.expected),
            "fixture `{}` must be refused as {}, saw {:#?}",
            fixture.id,
            fixture.expected.as_str(),
            found
                .iter()
                .map(|violation| violation.category.as_str())
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn legal_twins_are_accepted() {
    for id in POSITIVES {
        let found = evaluate_positive(id).unwrap_or_else(|| panic!("`{id}` is not registered"));
        assert!(found.is_empty(), "`{id}` must be accepted, saw {found:#?}");
    }
}

#[test]
fn the_self_test_matrix_is_complete_and_passing() {
    let ids = fixture_ids();
    assert_eq!(
        ids.len(),
        FIXTURES.len(),
        "every required negative class must have a fixture"
    );
    assert!(
        ids.contains(&"checker-bypass"),
        "the bypass probe must exist"
    );

    let cases = self_test();
    let failed: Vec<&str> = cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| case.id.as_str())
        .collect();
    assert!(failed.is_empty(), "self-test failures: {failed:?}");
    assert_eq!(
        cases.len(),
        FIXTURES.len() + POSITIVES.len() + 1,
        "the matrix covers the real tree, every fixture and every legal twin"
    );
}

// ---------------------------------------------------------------------------
// Fixture-result semantics: a correct refusal is never confusable with an
// unexpected acceptance, and `status != 0` alone is not evidence.
// ---------------------------------------------------------------------------

#[test]
fn fixture_outcomes_and_exit_codes_are_distinguishable() {
    // Distinct statuses, so an accepted forbidden fixture cannot masquerade as a refusal.
    assert_eq!(EXIT_LEGAL, 0);
    assert_eq!(EXIT_REFUSED, 1);
    assert_eq!(EXIT_USAGE, 2);
    assert_eq!(EXIT_HARNESS_FAILURE, 3);
    assert_ne!(EXIT_REFUSED, EXIT_HARNESS_FAILURE);
    assert_ne!(EXIT_HARNESS_FAILURE, EXIT_LEGAL);

    let fixture = fixture_by_id("direct-outward").expect("fixture must exist");
    let refused = evaluate_fixture(fixture.id).expect("fixture must be registered");
    assert_eq!(
        classify_fixture_result(&refused),
        FixtureOutcome::Refused,
        "a forbidden construct must classify as refused"
    );
    assert_eq!(fixture_exit_code(FixtureOutcome::Refused), EXIT_REFUSED);
    assert_eq!(
        fixture_exit_code(FixtureOutcome::Accepted),
        EXIT_HARNESS_FAILURE
    );

    let refused_text = fixture_outcome_text(fixture, &refused);
    assert!(
        refused_text.contains("outcome=refused"),
        "refusal text must say so: {refused_text}"
    );
    assert!(
        refused_text.contains("category_match=true"),
        "the expected category must be reported: {refused_text}"
    );

    // The pathological case: if the guard ever accepted the fixture, the text and status
    // must both change.
    let accepted_text = fixture_outcome_text(fixture, &[]);
    assert!(accepted_text.contains("outcome=accepted"));
    assert!(accepted_text.contains("category_match=false"));
    assert_ne!(refused_text, accepted_text);
    assert_eq!(
        classify_fixture_result(&[]),
        FixtureOutcome::Accepted,
        "an empty refusal set is an acceptance, never a pass"
    );
    assert_ne!(fixture_exit_code(FixtureOutcome::Accepted), EXIT_REFUSED);
}

#[test]
fn every_fixture_reports_its_expected_category_and_outcome() {
    for fixture in &FIXTURES {
        let found = evaluate_fixture(fixture.id)
            .unwrap_or_else(|| panic!("fixture `{}` is not registered", fixture.id));

        assert_eq!(
            classify_fixture_result(&found),
            FixtureOutcome::Refused,
            "fixture `{}` was accepted",
            fixture.id
        );

        let text = fixture_outcome_text(fixture, &found);
        assert!(
            text.contains("outcome=refused") && text.contains("category_match=true"),
            "fixture `{}` must report a refused outcome with its category: {text}",
            fixture.id
        );
        assert!(
            found
                .iter()
                .any(|violation| violation.category == fixture.expected),
            "fixture `{}` must be refused as {}, saw {:?}",
            fixture.id,
            fixture.expected.as_str(),
            categories(&found)
        );
    }
}

fn categories(violations: &[Violation]) -> Vec<&'static str> {
    violations
        .iter()
        .map(|violation| violation.category.as_str())
        .collect()
}

// ---------------------------------------------------------------------------
// Exclusive, non-destructive fixture ownership.
//
// Every directory this target touches is inspected as an **unresolved** entry before it is
// used, each fixture root is created exclusively (never adopted), and teardown removes only
// paths this test positively created — links first, then files and *empty* directories,
// deepest first. `remove_dir_all` is never used and no shared parent is treated as owned.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
enum ScratchError {
    /// A component of the path is a link, junction or other reparse point.
    Redirected,
    /// A component exists but could not be inspected.
    Uninspectable,
    /// An intermediate component is not a real directory.
    NotADirectory,
    /// The requested fixture directory already exists; it is never adopted.
    AlreadyExists,
}

/// Validates one existing path as a real, non-redirected directory, inspecting every
/// original component from the volume root down.
fn validate_real_directory(path: &Path) -> Result<(), ScratchError> {
    match source::inspect_original_chain(path) {
        Ok(()) => {}
        Err(source::ConfinementError::Escape) => return Err(ScratchError::Redirected),
        Err(source::ConfinementError::Unresolvable) => return Err(ScratchError::Uninspectable),
    }
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => Ok(()),
        Ok(_) => Err(ScratchError::NotADirectory),
        Err(_) => Err(ScratchError::Uninspectable),
    }
}

/// Creates one directory level exclusively: it must not already exist, and the caller has
/// already validated its parent.
fn create_one_level(path: &Path) -> Result<(), ScratchError> {
    match std::fs::create_dir(path) {
        Ok(()) => validate_real_directory(path),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            Err(ScratchError::AlreadyExists)
        }
        Err(_) => Err(ScratchError::Uninspectable),
    }
}

/// Establishes a **shared** directory that may already exist: created exclusively when
/// absent, and inspected (never adopted, never rewritten) when present. A redirected or
/// non-directory shared path is refused.
fn ensure_shared_directory(path: &Path) -> Result<(), ScratchError> {
    match std::fs::create_dir(path) {
        Ok(()) => validate_real_directory(path),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            validate_real_directory(path)
        }
        Err(_) => Err(ScratchError::Uninspectable),
    }
}

/// A fixture root this test created and therefore owns.
struct OwnedScratch {
    root: PathBuf,
    dirs: Vec<PathBuf>,
    files: Vec<PathBuf>,
    links: Vec<PathBuf>,
}

impl OwnedScratch {
    /// The checkout root of this repository, from Cargo's own manifest location.
    fn checkout() -> PathBuf {
        std::fs::canonicalize(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join(".."),
        )
        .expect("the checkout root must resolve")
    }

    /// Creates a fixture root under the checkout's real `.temp` scratch root.
    fn create(label: &str) -> Self {
        let checkout = Self::checkout();
        let temp = checkout.join(".temp");
        ensure_shared_directory(&temp).expect("the scratch root must be a real directory");

        let parent = temp.join("architecture-fixtures");
        ensure_shared_directory(&parent).expect("the fixture parent must be a real directory");

        Self::create_in_with(&parent, label, &checkout).expect("the fixture root must be creatable")
    }

    /// Creates one exclusively owned fixture root with **exactly** this name.
    ///
    /// An existing entry is refused (`AlreadyExists`) even when it looks empty: a
    /// directory that merely has the right name is never adopted as task-owned.
    fn create_exact(parent: &Path, name: &str) -> Result<Self, ScratchError> {
        validate_real_directory(parent)?;
        let root = parent.join(name);
        create_one_level(&root)?;
        Ok(Self {
            dirs: vec![root.clone()],
            root,
            files: Vec::new(),
            links: Vec::new(),
        })
    }

    /// Creates one exclusively owned fixture root inside an already validated parent,
    /// retrying with a fresh name rather than ever adopting an existing directory.
    fn create_in(parent: &Path, label: &str) -> Result<Self, ScratchError> {
        Self::create_in_with(parent, label, parent)
    }

    fn create_in_with(parent: &Path, label: &str, anchor: &Path) -> Result<Self, ScratchError> {
        validate_real_directory(anchor)?;
        validate_real_directory(parent)?;

        for attempt in 0..8u32 {
            let name = format!(
                "{label}-{}-{}-{attempt}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|elapsed| elapsed.as_nanos())
                    .unwrap_or(0)
            );
            let root = parent.join(name);
            match create_one_level(&root) {
                Ok(()) => {
                    return Ok(Self {
                        dirs: vec![root.clone()],
                        root,
                        files: Vec::new(),
                        links: Vec::new(),
                    });
                }
                Err(ScratchError::AlreadyExists) => continue,
                Err(error) => return Err(error),
            }
        }
        Err(ScratchError::AlreadyExists)
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    /// Creates one owned nested directory level, recording it for teardown.
    fn dir(&mut self, relative: &str) {
        let mut current = self.root.clone();
        for component in relative.split('/') {
            current.push(component);
            if self.dirs.contains(&current) {
                continue;
            }
            create_one_level(&current)
                .unwrap_or_else(|error| panic!("fixture directory `{relative}`: {error:?}"));
            self.dirs.push(current.clone());
        }
    }

    /// Writes a new file, refusing to overwrite anything that already exists.
    fn write(&mut self, relative: &str, text: &str) -> PathBuf {
        let path = self.path(relative);
        if let Some((parent, _)) = relative.rsplit_once('/') {
            let parent = parent.to_owned();
            self.dir(&parent);
        }
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap_or_else(|error| panic!("fixture file `{relative}` must be new: {error}"));
        std::io::Write::write_all(&mut file, text.as_bytes()).expect("fixture file write");
        drop(file);
        self.files.push(path.clone());
        path
    }

    fn link_dir(&mut self, relative: &str, target: &Path) -> bool {
        let link = self.path(relative);
        if let Some((parent, _)) = relative.rsplit_once('/') {
            let parent = parent.to_owned();
            self.dir(&parent);
        }
        assert!(
            std::fs::symlink_metadata(&link).is_err(),
            "refusing to create a link over an existing entry: {relative}"
        );
        if create_dir_link(&link, target) {
            self.links.push(link);
            true
        } else {
            false
        }
    }
}

impl Drop for OwnedScratch {
    fn drop(&mut self) {
        // Links first, with the link-only operation, so nothing is ever deleted through a
        // redirect into its target.
        for link in &self.links {
            if !remove_dir_link(link) {
                eprintln!(
                    "cleanup: could not remove fixture link {}",
                    link.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
        for file in &self.files {
            if std::fs::remove_file(file).is_err() && std::fs::symlink_metadata(file).is_ok() {
                eprintln!(
                    "cleanup: could not remove fixture file {}",
                    file.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
        let mut dirs = self.dirs.clone();
        dirs.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
        for dir in dirs {
            if let Err(error) = std::fs::remove_dir(&dir)
                && dir.is_dir()
            {
                // Deliberately left in place: refusing beats deleting foreign material.
                eprintln!(
                    "cleanup: fixture directory not empty or not removable: {} ({error})",
                    dir.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
    }
}

#[cfg(windows)]
fn create_dir_link(link: &Path, target: &Path) -> bool {
    let output = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output();

    match output {
        Ok(output) if output.status.success() => true,
        Ok(output) => {
            eprintln!(
                "mklink failed: status={:?} stdout={} stderr={}",
                output.status.code(),
                String::from_utf8_lossy(&output.stdout).trim(),
                String::from_utf8_lossy(&output.stderr).trim()
            );
            false
        }
        Err(error) => {
            eprintln!("mklink could not be spawned: {error}");
            false
        }
    }
}

#[cfg(windows)]
fn remove_dir_link(link: &Path) -> bool {
    // A junction is removed with `rmdir`, which deletes the link and never its target.
    std::fs::remove_dir(link).is_ok()
}

#[cfg(unix)]
fn create_dir_link(link: &Path, target: &Path) -> bool {
    std::os::unix::fs::symlink(target, link).is_ok()
}

#[cfg(unix)]
fn remove_dir_link(link: &Path) -> bool {
    std::fs::remove_file(link).is_ok() || std::fs::remove_dir(link).is_ok()
}

#[cfg(not(any(windows, unix)))]
fn create_dir_link(_link: &Path, _target: &Path) -> bool {
    false
}

#[cfg(not(any(windows, unix)))]
fn remove_dir_link(_link: &Path) -> bool {
    false
}

/// A spec anchored at a fixture checkout, with the package at `crates/<package>`.
fn real_spec(anchor: &Path, package: &str) -> source::UnitSpec {
    let world = FixtureWorld::standard();
    let mut spec = world.spec(package, TargetKind::Lib, "lib.rs", &[]);
    spec.anchor = anchor.to_path_buf();
    spec.package_root = anchor.join("crates").join(package);
    spec.root_file = spec.package_root.join("src").join("lib.rs");
    spec.label_root = "fixture".to_owned();
    spec
}

fn snapshot(dir: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    let mut files = std::collections::BTreeMap::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(current) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&current) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if let Ok(bytes) = std::fs::read(&path) {
                files.insert(path.to_string_lossy().into_owned(), bytes);
            }
        }
    }
    files
}

fn assert_target_preserved(before: &std::collections::BTreeMap<String, Vec<u8>>, dir: &Path) {
    assert_eq!(
        before,
        &snapshot(dir),
        "the redirected target must be preserved byte-for-byte"
    );
}

// ---------------------------------------------------------------------------
// Genuine source redirection, through the revised owner-safe fixture lifecycle.
// ---------------------------------------------------------------------------

fn fixture_checkout(scratch: &mut OwnedScratch) {
    scratch.write("checkout/.git/HEAD", "ref: refs/heads/main\n");
    scratch.write("checkout/Cargo.toml", "[workspace]\nmembers = []\n");
}

/// A lexically in-package `include!` whose directory component is a real redirect.
#[test]
fn a_redirected_include_is_refused_before_its_content_is_consumed() {
    let mut scratch = OwnedScratch::create("include-redirect");
    fixture_checkout(&mut scratch);
    scratch.write(
        "outside/forbidden.inc",
        "// redirected payload: must never be consumed\nuse solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 1 }\n",
    );
    let target = scratch.path("outside");

    if !scratch.link_dir("checkout/crates/solo-application/src/alias", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             include! escape gate cannot be evidenced here"
        );
    }
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "include!(\"alias/forbidden.inc\");\n",
    );

    let anchor = scratch.path("checkout");
    let package_root = anchor.join("crates").join("solo-application");
    let lexical = package_root.join("src").join("alias").join("forbidden.inc");
    assert!(
        lexical.starts_with(&package_root),
        "the literal include path is lexically inside the package"
    );
    let effective = std::fs::canonicalize(&lexical).expect("effective path must resolve");
    assert!(
        !effective.starts_with(std::fs::canonicalize(&package_root).expect("root must resolve")),
        "the effective path must escape the package through the redirect"
    );

    let before = snapshot(&target);
    let found = source::check_unit(&real_spec(&anchor, "solo-application"), &FsReader);

    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::IncludeEscape),
        "the redirected include must be refused as include-escape, saw {:?}",
        categories(&found)
    );
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "the redirected content must never be scanned, saw {:?}",
        categories(&found)
    );
    assert!(
        lexical.exists(),
        "the redirect must be preserved, not deleted"
    );
    assert_target_preserved(&before, &target);
}

/// A `#[path]` module whose redirected directory escapes the package.
#[test]
fn a_redirected_path_module_is_refused() {
    let mut scratch = OwnedScratch::create("path-redirect");
    fixture_checkout(&mut scratch);
    scratch.write(
        "outside/escape_mod.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 2 }\n",
    );
    let target = scratch.path("outside");

    if !scratch.link_dir("checkout/crates/solo-application/src/alias", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             #[path] escape gate cannot be evidenced here"
        );
    }
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "#[path = \"alias/escape_mod.rs\"]\nmod escaped;\n",
    );

    let before = snapshot(&target);
    let found = source::check_unit(
        &real_spec(&scratch.path("checkout"), "solo-application"),
        &FsReader,
    );

    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "the redirected module must be refused as module-source-escape, saw {:?}",
        categories(&found)
    );
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "the redirected module content must never be scanned, saw {:?}",
        categories(&found)
    );
    assert_target_preserved(&before, &target);
}

/// A default `mod` whose `name/mod.rs` lives behind a redirected directory.
#[test]
fn a_redirected_default_module_is_refused() {
    let mut scratch = OwnedScratch::create("default-mod-redirect");
    fixture_checkout(&mut scratch);
    scratch.write(
        "outside/mod.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 3 }\n",
    );
    let target = scratch.path("outside");

    if !scratch.link_dir("checkout/crates/solo-application/src/escaped", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             default-module escape gate cannot be evidenced here"
        );
    }
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "mod escaped;\n",
    );

    let before = snapshot(&target);
    let found = source::check_unit(
        &real_spec(&scratch.path("checkout"), "solo-application"),
        &FsReader,
    );

    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "the redirected default module must be refused as module-source-escape, saw {:?}",
        categories(&found)
    );
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "the redirected module content must never be scanned, saw {:?}",
        categories(&found)
    );
    assert_target_preserved(&before, &target);
}

/// A conditional `#[path]` where the default source also exists on disk.
#[test]
fn a_conditional_module_path_is_refused_even_with_a_valid_default() {
    let mut scratch = OwnedScratch::create("cfg-attr-default");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "#[cfg_attr(windows, path = \"alt_win.rs\")]\nmod alternate;\n",
    );
    scratch.write(
        "checkout/crates/solo-application/src/alternate.rs",
        "pub fn from_default() {}\n",
    );
    scratch.write(
        "checkout/crates/solo-application/src/alt_win.rs",
        "pub fn from_conditional() {}\n",
    );

    let anchor = scratch.path("checkout");
    assert!(
        anchor
            .join("crates/solo-application/src/alternate.rs")
            .is_file(),
        "the default source must exist, so passing merely because it exists is not the test"
    );

    let found = source::check_unit(&real_spec(&anchor, "solo-application"), &FsReader);
    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ConditionalModulePath),
        "a conditional module path must be refused, saw {:?}",
        categories(&found)
    );
}

/// A required module with no source file must fail closed on the real filesystem too.
#[test]
fn a_missing_real_module_source_is_refused() {
    let mut scratch = OwnedScratch::create("missing-module");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "mod missing;\n",
    );

    let found = source::check_unit(
        &real_spec(&scratch.path("checkout"), "solo-application"),
        &FsReader,
    );
    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceUnresolved),
        "a missing module must be refused as module-source-unresolved, saw {:?}",
        categories(&found)
    );
}

/// A genuine nonredirected package root keeps working, including `include!` and modules.
#[test]
fn legal_in_package_loading_still_succeeds() {
    let mut scratch = OwnedScratch::create("legal-loading");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "include!(\"fragment.inc\");\n#[cfg_attr(test, allow(dead_code))]\nmod inner;\n",
    );
    scratch.write(
        "checkout/crates/solo-application/src/fragment.inc",
        "pub fn fragment() {}\n",
    );
    scratch.write(
        "checkout/crates/solo-application/src/inner.rs",
        "pub fn inner() {}\n",
    );

    let found = source::check_unit(
        &real_spec(&scratch.path("checkout"), "solo-application"),
        &FsReader,
    );
    assert!(
        found.is_empty(),
        "legal in-package loading must be accepted, saw {:?}",
        categories(&found)
    );
}

// ---------------------------------------------------------------------------
// Package-root anchoring: the root itself, and an ancestor between the checkout
// anchor and the root, must be refused even when canonicalisation looks consistent.
// ---------------------------------------------------------------------------

#[test]
fn a_redirected_package_root_is_refused_before_its_source_is_used() {
    let mut scratch = OwnedScratch::create("root-redirect");
    fixture_checkout(&mut scratch);
    // The redirect target stays *inside* the same fixture checkout, the harder case.
    scratch.write(
        "checkout/real-target/src/lib.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 4 }\n",
    );
    let anchor = scratch.path("checkout");
    let target = anchor.join("real-target");

    if !scratch.link_dir("checkout/crates/solo-application", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine package-root redirect, so the \
             root-anchoring gate cannot be evidenced here"
        );
    }

    let package_root = anchor.join("crates").join("solo-application");
    let root_file = package_root.join("src").join("lib.rs");
    // The former defect's false-positive condition: the redirected root and the file inside
    // it canonicalise to internally consistent paths, so containment alone would pass.
    let effective_root = std::fs::canonicalize(&package_root).expect("root must resolve");
    let effective_file = std::fs::canonicalize(&root_file).expect("file must resolve");
    assert!(
        effective_file.starts_with(&effective_root),
        "the redirected root and its source must canonicalise consistently"
    );
    let entry = std::fs::symlink_metadata(&package_root).expect("entry must exist");
    assert!(
        is_a_redirect(&entry),
        "the package-root entry itself must be a genuine redirect"
    );

    let before = snapshot(&target);
    let found = source::check_unit(&real_spec(&anchor, "solo-application"), &FsReader);

    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "a redirected package root must be refused, saw {:?}",
        categories(&found)
    );
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "the redirected source must never be consumed, saw {:?}",
        categories(&found)
    );
    assert_target_preserved(&before, &target);
    assert!(
        package_root.exists(),
        "the package-root redirect must be preserved, not deleted"
    );
}

#[test]
fn a_redirected_package_ancestor_is_refused() {
    let mut scratch = OwnedScratch::create("ancestor-redirect");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/real-crates/solo-application/src/lib.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 5 }\n",
    );
    let anchor = scratch.path("checkout");
    let target = anchor.join("real-crates");

    if !scratch.link_dir("checkout/crates", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine `crates` redirect, so the \
             ancestor-anchoring gate cannot be evidenced here"
        );
    }

    let package_root = anchor.join("crates").join("solo-application");
    let entry = std::fs::symlink_metadata(&package_root).expect("package entry must exist");
    assert!(
        !is_a_redirect(&entry) && entry.is_dir(),
        "after resolution the package root itself looks like an ordinary directory"
    );

    let before = snapshot(&target);
    let found = source::check_unit(&real_spec(&anchor, "solo-application"), &FsReader);

    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "a redirected ancestor must be refused, saw {:?}",
        categories(&found)
    );
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "source behind a redirected ancestor must never be consumed, saw {:?}",
        categories(&found)
    );
    assert_target_preserved(&before, &target);
}

fn is_a_redirect(metadata: &std::fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes() & 0x0000_0400 != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

// ---------------------------------------------------------------------------
// Scratch ownership: exclusive creation, collision preservation, and a
// redirected synthetic scratch ancestor refused before any write or delete.
// ---------------------------------------------------------------------------

#[test]
fn an_existing_fixture_directory_is_never_adopted_or_deleted() {
    let mut scratch = OwnedScratch::create("scratch-collision");
    scratch.dir("parent");
    // Pre-existing content that is NOT ours: it must survive untouched.
    scratch.write("parent/case/sentinel.txt", "foreign material\n");
    let parent = scratch.path("parent");
    let sentinel = scratch.path("parent/case/sentinel.txt");
    let before = std::fs::read(&sentinel).expect("sentinel must be readable");

    match OwnedScratch::create_exact(&parent, "case") {
        Err(ScratchError::AlreadyExists) => {}
        Ok(_) => {
            panic!("an existing fixture directory must be refused, but was adopted")
        }
        Err(other) => panic!("unexpected refusal reason: {other:?}"),
    }

    assert_eq!(
        std::fs::read(&sentinel).expect("sentinel must still exist"),
        before,
        "a refused collision must leave foreign material byte-for-byte intact"
    );
}

#[test]
fn a_redirected_scratch_ancestor_is_refused_before_any_write() {
    let mut scratch = OwnedScratch::create("scratch-redirect");
    // A synthetic "checkout" whose `.temp` is a genuine redirect to another directory.
    scratch.dir("case/checkout/target");
    let target = scratch.path("case/checkout/target");
    let redirected_root = scratch.path("case/checkout/.temp");

    if !scratch.link_dir("case/checkout/.temp", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine scratch redirect, so the \
             scratch-ownership gate cannot be evidenced here"
        );
    }

    match OwnedScratch::create_in(&redirected_root, "fixture") {
        Err(ScratchError::Redirected) => {}
        Ok(_) => panic!("a redirected scratch ancestor must be refused, but was used"),
        Err(other) => panic!("unexpected refusal reason: {other:?}"),
    }

    assert!(
        !target.join("fixture").exists(),
        "nothing may be created through a redirected scratch ancestor"
    );
    assert_eq!(
        snapshot(&target).len(),
        0,
        "the redirect target must stay empty: no writes, no deletions"
    );
    assert!(
        redirected_root.exists(),
        "the synthetic scratch redirect must be preserved"
    );
}

// ---------------------------------------------------------------------------
// Discovery must fail closed for every entry, including non-`.rs` names.
// ---------------------------------------------------------------------------

/// Wraps the real inspector and injects one deterministic metadata failure, so the failure
/// travels the production error-handling path rather than a mock's own logic.
struct FaultyDiscoveryFs {
    real: RealDiscoveryFs,
    fail_metadata_for: std::ffi::OsString,
}

impl DiscoveryFs for FaultyDiscoveryFs {
    fn list(&self, directory: &Path) -> std::io::Result<Vec<std::ffi::OsString>> {
        self.real.list(directory)
    }

    fn kind(&self, path: &Path) -> std::io::Result<EntryKind> {
        self.real.kind(path)
    }

    fn redirected(&self, path: &Path) -> std::io::Result<bool> {
        if path.file_name() == Some(self.fail_metadata_for.as_os_str()) {
            return Err(std::io::Error::other("injected metadata failure"));
        }
        self.real.redirected(path)
    }
}

#[test]
fn a_discovery_metadata_failure_on_a_non_source_entry_is_refused() {
    let mut scratch = OwnedScratch::create("discovery-metadata");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "pub fn ok() {}\n",
    );
    scratch.write("checkout/crates/solo-application/notes.txt", "not rust\n");
    let package_root = scratch.path("checkout/crates/solo-application");
    let anchor = scratch.path("checkout");

    let inspector = FaultyDiscoveryFs {
        real: RealDiscoveryFs,
        fail_metadata_for: std::ffi::OsString::from("notes.txt"),
    };
    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &inspector);

    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::UnreadableSource),
        "an uninspectable non-`.rs` entry must be refused, saw {:?} / inventory {:?}",
        categories(&violations),
        found
    );
}

#[test]
fn the_real_discovery_accepts_a_clean_package() {
    let mut scratch = OwnedScratch::create("discovery-clean");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "pub fn ok() {}\n",
    );
    scratch.write("checkout/crates/solo-application/README.md", "docs\n");
    let package_root = scratch.path("checkout/crates/solo-application");
    let anchor = scratch.path("checkout");

    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &RealDiscoveryFs);

    assert!(
        violations.is_empty(),
        "a clean package must not produce refusals, saw {:?}",
        categories(&violations)
    );
    assert_eq!(
        found.len(),
        1,
        "only the Rust source is inventoried: {found:?}"
    );
}

#[test]
fn a_redirected_discovery_directory_is_refused_and_preserved() {
    let mut scratch = OwnedScratch::create("discovery-redirect");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "pub fn ok() {}\n",
    );
    scratch.write("outside/hidden.rs", "use solo_adapters::CliOutcome;\n");
    let target = scratch.path("outside");
    let package_root = scratch.path("checkout/crates/solo-application");
    let anchor = scratch.path("checkout");

    if !scratch.link_dir("checkout/crates/solo-application/vendor", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine discovery redirect, so the \
             discovery-redirect gate cannot be evidenced here"
        );
    }

    let before = snapshot(&target);
    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &RealDiscoveryFs);

    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "a redirected discovery directory must be refused, saw {:?}",
        categories(&violations)
    );
    assert!(
        !found
            .iter()
            .any(|path| path.to_string_lossy().contains("hidden")),
        "a redirected directory must never be inventoried: {found:?}"
    );
    assert_target_preserved(&before, &target);
}

// ---------------------------------------------------------------------------
// The checkout anchor itself is verified from real metadata, the Git root and
// the `crates/<package>` layout.
// ---------------------------------------------------------------------------

#[test]
fn the_anchor_requires_a_workspace_manifest_a_git_root_and_crates_layout() {
    let mut scratch = OwnedScratch::create("anchor-verify");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "pub fn ok() {}\n",
    );
    let anchor = scratch.path("checkout");
    let spec = real_spec(&anchor, "solo-application");

    let package = |root: PathBuf| architecture_checks::graph::PackageTargets {
        name: "solo-application".to_owned(),
        root,
        deps: Vec::new(),
        targets: Vec::new(),
        lib_crate: Some("solo_application".to_owned()),
    };

    // A real, well-formed fixture checkout is accepted.
    assert!(
        CheckoutAnchor::verify(&anchor, &[package(spec.package_root.clone())]).is_ok(),
        "a nonredirected checkout with crates/<package> must anchor"
    );

    // The same package claimed from a path that is not crates/<package> is refused.
    let misplaced = package(anchor.join("src").join("solo-application"));
    assert!(
        CheckoutAnchor::verify(&anchor, &[misplaced]).is_err(),
        "a package root outside crates/<package> must be refused"
    );

    // An anchor with a workspace manifest but no Git root is not the verified checkout.
    let plain = scratch.path("plain");
    scratch.dir("plain");
    scratch.write("plain/Cargo.toml", "[workspace]\n");
    assert!(
        CheckoutAnchor::verify(
            &plain,
            &[package(anchor.join("crates").join("solo-application"))]
        )
        .is_err(),
        "an anchor that is not a Git working tree root must be refused"
    );
}

// ---------------------------------------------------------------------------
// Discovery-root pre-enumeration (SOLO-WO-033 R01).
//
// These call the production `source::rust_files_under` directly and instrument the
// `DiscoveryFs` seam, so they prove the *ordering*: a redirected or uninspectable
// directory is refused before even its first `list` call, not after its contents have
// been enumerated.
// ---------------------------------------------------------------------------

/// Delegates every inspection to the real filesystem while recording each enumeration.
struct RecordingDiscoveryFs {
    real: RealDiscoveryFs,
    lists: std::cell::RefCell<Vec<PathBuf>>,
}

impl RecordingDiscoveryFs {
    fn new() -> Self {
        Self {
            real: RealDiscoveryFs,
            lists: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn list_count(&self) -> usize {
        self.lists.borrow().len()
    }

    fn listed(&self, directory: &Path) -> bool {
        let directory = directory.to_path_buf();
        self.lists.borrow().iter().any(|entry| entry == &directory)
    }
}

impl DiscoveryFs for RecordingDiscoveryFs {
    fn list(&self, directory: &Path) -> std::io::Result<Vec<std::ffi::OsString>> {
        self.lists.borrow_mut().push(directory.to_path_buf());
        self.real.list(directory)
    }

    fn kind(&self, path: &Path) -> std::io::Result<EntryKind> {
        self.real.kind(path)
    }

    fn redirected(&self, path: &Path) -> std::io::Result<bool> {
        self.real.redirected(path)
    }
}

/// T-01 — a genuine redirect at the **initial package root** is refused before any `list`.
#[test]
fn a_redirected_discovery_root_is_refused_before_its_first_list() {
    let mut scratch = OwnedScratch::create("discovery-root-pre-list");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/real-target/src/lib.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 7 }\n",
    );
    let anchor = scratch.path("checkout");
    let target = anchor.join("real-target");
    let package_root = anchor.join("crates").join("solo-application");

    if !scratch.link_dir("checkout/crates/solo-application", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine package-root redirect, so the \
             pre-enumeration gate cannot be evidenced"
        );
    }

    // The former false-positive condition: root and child canonicalise consistently.
    let effective_root = std::fs::canonicalize(&package_root).expect("root must resolve");
    let effective_child =
        std::fs::canonicalize(package_root.join("src")).expect("child must resolve");
    assert!(
        effective_child.starts_with(&effective_root),
        "the redirected root and its child must canonicalise consistently"
    );
    let entry = std::fs::symlink_metadata(&package_root).expect("entry");
    assert!(
        is_a_redirect(&entry),
        "the unresolved package-root entry must be a genuine redirect"
    );

    let before = snapshot(&target);
    let inspector = RecordingDiscoveryFs::new();
    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &inspector);

    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "the redirected discovery root must be refused as module-source-escape, saw {:?}",
        categories(&violations)
    );
    assert_eq!(
        inspector.list_count(),
        0,
        "no directory may be enumerated once the root is refused: {:?}",
        inspector.lists.borrow()
    );
    assert!(
        found.is_empty(),
        "no redirected `.rs` file may be inventoried: {found:?}"
    );
    assert_target_preserved(&before, &target);
    assert!(
        package_root.exists(),
        "the package-root redirect must be preserved"
    );
}

/// T-02 — a redirect on the `crates` **ancestor** is refused before enumeration, even
/// though the package-root entry looks like an ordinary directory after resolution.
#[test]
fn a_redirected_discovery_ancestor_is_refused_before_its_first_list() {
    let mut scratch = OwnedScratch::create("discovery-ancestor-pre-list");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/real-crates/solo-application/src/lib.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 8 }\n",
    );
    let anchor = scratch.path("checkout");
    let target = anchor.join("real-crates");
    let package_root = anchor.join("crates").join("solo-application");

    if !scratch.link_dir("checkout/crates", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine `crates` redirect, so the \
             ancestor pre-enumeration gate cannot be evidenced"
        );
    }

    let entry = std::fs::symlink_metadata(&package_root).expect("package entry");
    assert!(
        !is_a_redirect(&entry) && entry.is_dir(),
        "after resolution the package root itself looks like an ordinary directory"
    );

    let before = snapshot(&target);
    let inspector = RecordingDiscoveryFs::new();
    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &inspector);

    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "a redirected ancestor must be refused as module-source-escape, saw {:?}",
        categories(&violations)
    );
    assert!(
        !inspector.listed(&package_root),
        "the package directory must never be enumerated through a redirected ancestor"
    );
    assert_eq!(
        inspector.list_count(),
        0,
        "no enumeration at all may occur behind a refused ancestor: {:?}",
        inspector.lists.borrow()
    );
    assert!(
        found.is_empty(),
        "no file behind a redirected ancestor may be inventoried: {found:?}"
    );
    assert_target_preserved(&before, &target);
}

/// T-03 — every **queued** directory is validated before `list`, not only when it is
/// appended to the queue.
///
/// The injected seam models the deterministic real-world condition "a queued directory
/// becomes uninspectable after the initial classification" by failing the second
/// inspection of one specific child. The failure travels the production pre-list code.
struct FlakyDiscoveryFs {
    real: RealDiscoveryFs,
    target: PathBuf,
    kind_calls: std::cell::Cell<usize>,
    redirect_calls: std::cell::Cell<usize>,
    lists: std::cell::RefCell<Vec<PathBuf>>,
}

impl FlakyDiscoveryFs {
    fn new(target: PathBuf) -> Self {
        Self {
            real: RealDiscoveryFs,
            target,
            kind_calls: std::cell::Cell::new(0),
            redirect_calls: std::cell::Cell::new(0),
            lists: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn listed(&self, directory: &Path) -> bool {
        let directory = directory.to_path_buf();
        self.lists.borrow().iter().any(|entry| entry == &directory)
    }
}

impl DiscoveryFs for FlakyDiscoveryFs {
    fn list(&self, directory: &Path) -> std::io::Result<Vec<std::ffi::OsString>> {
        self.lists.borrow_mut().push(directory.to_path_buf());
        self.real.list(directory)
    }

    fn kind(&self, path: &Path) -> std::io::Result<EntryKind> {
        if path == self.target {
            let calls = self.kind_calls.get() + 1;
            self.kind_calls.set(calls);
            if calls >= 2 {
                return Err(std::io::Error::other("injected queued-directory failure"));
            }
        }
        self.real.kind(path)
    }

    fn redirected(&self, path: &Path) -> std::io::Result<bool> {
        if path == self.target {
            let calls = self.redirect_calls.get() + 1;
            self.redirect_calls.set(calls);
            if calls >= 2 {
                return Err(std::io::Error::other("injected queued-directory failure"));
            }
        }
        self.real.redirected(path)
    }
}

#[test]
fn a_queued_directory_is_validated_before_it_is_listed() {
    let mut scratch = OwnedScratch::create("discovery-queued-pre-list");
    fixture_checkout(&mut scratch);
    scratch.write(
        "checkout/crates/solo-application/src/lib.rs",
        "pub fn ok() {}\n",
    );
    scratch.write(
        "checkout/crates/solo-application/src/nested/hidden.rs",
        "pub fn nested() {}\n",
    );
    let anchor = scratch.path("checkout");
    let package_root = anchor.join("crates").join("solo-application");
    let nested = package_root.join("src").join("nested");

    let inspector = FlakyDiscoveryFs::new(nested.clone());
    let (found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &inspector);

    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::UnreadableSource),
        "a queued directory that cannot be inspected must be refused, saw {:?}",
        categories(&violations)
    );
    assert!(
        !inspector.listed(&nested),
        "the failed pre-enumeration check must prevent listing that directory: {:?}",
        inspector.lists.borrow()
    );
    assert!(
        !found
            .iter()
            .any(|path| path.to_string_lossy().contains("hidden")),
        "nothing from an unvalidated queued directory may be inventoried: {found:?}"
    );
    assert!(
        found
            .iter()
            .any(|path| path.to_string_lossy().ends_with("lib.rs")),
        "the validated root must still be inventoried: {found:?}"
    );
}

/// T-05 — safety evidence: fixture roots are positively owned, no redirect ever becomes a
/// cleanup root, and pre-existing foreign scratch is never read, changed or deleted.
#[test]
fn discovery_fixtures_are_owned_and_foreign_scratch_is_untouched() {
    let foreign = OwnedScratch::checkout()
        .join(".temp")
        .join("rt")
        .join("tmp")
        .join("system-commandline-sentinel-files");
    let foreign_before = snapshot(&foreign);

    let mut scratch = OwnedScratch::create("discovery-safety");
    fixture_checkout(&mut scratch);
    scratch.write("checkout/real-target/src/lib.rs", "pub fn leaked() {}\n");
    let anchor = scratch.path("checkout");
    let target = anchor.join("real-target");
    let package_root = anchor.join("crates").join("solo-application");

    if !scratch.link_dir("checkout/crates/solo-application", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine redirect, so the ownership \
             evidence cannot be produced"
        );
    }

    // The fixture root is a real directory this test created, never a redirect.
    let root_entry = std::fs::symlink_metadata(&scratch.root).expect("owned root entry");
    assert!(
        root_entry.is_dir() && !is_a_redirect(&root_entry),
        "the fixture root must be an exclusive real directory"
    );
    assert!(
        scratch.root.starts_with(
            OwnedScratch::checkout()
                .join(".temp")
                .join("architecture-fixtures")
        ),
        "the fixture root must live in the owned scratch hierarchy: {}",
        scratch.root.display()
    );

    let before = snapshot(&target);
    let inspector = RecordingDiscoveryFs::new();
    let (_found, violations) =
        source::rust_files_under(&package_root, "fixture", &anchor, &inspector);
    assert!(
        violations
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceEscape),
        "the redirect must still be refused, saw {:?}",
        categories(&violations)
    );

    // The redirect is a link, not an owned cleanup root, and its target is unchanged.
    let link_entry = std::fs::symlink_metadata(&package_root).expect("link entry");
    assert!(
        is_a_redirect(&link_entry),
        "the link must still exist as a link after the refused walk"
    );
    assert_target_preserved(&before, &target);

    // Foreign scratch is never touched by building, walking or cleaning owned fixtures.
    assert_eq!(
        foreign_before,
        snapshot(&foreign),
        "pre-existing foreign scratch must be byte-for-byte untouched"
    );
}

// ---------------------------------------------------------------------------
// The checker's own unsafe-scope rule, proved negatively in place.
// ---------------------------------------------------------------------------

#[test]
fn the_checker_allows_unsafe_only_inside_the_windows_gated_approved_module() {
    let gated = concat!(
        "#[cfg(windows)]\n",
        "mod sys {\n",
        "    fn f() {\n",
        "        let _ = unsafe { core::ptr::null::<u8>() };\n",
        "    }\n",
        "}\n",
    );
    assert!(
        architecture_checks::source::classify_unsafe_scope(
            architecture_checks::source::APPROVED_NATIVE_MODULE,
            gated
        )
        .is_ok(),
        "the approved module may use unsafe inside a Windows gate"
    );
    assert!(
        architecture_checks::source::classify_unsafe_scope("crates/solo-domain/src/lib.rs", gated)
            .is_err(),
        "the same code outside the approved module must be refused"
    );
    assert!(
        architecture_checks::source::classify_unsafe_scope(
            "crates/solo-adapters/src/source_read.rs",
            gated
        )
        .is_err(),
        "the safe wrapper must never be allowed an unsafe block"
    );
}

#[test]
fn the_checker_refuses_unsafe_outside_a_positive_windows_gate() {
    let unscoped = "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }\n";
    let reason = architecture_checks::source::classify_unsafe_scope(
        architecture_checks::source::APPROVED_NATIVE_MODULE,
        unscoped,
    )
    .expect_err("unscoped unsafe must be refused even in the approved module");
    assert!(
        reason.contains("outside a `#[cfg(windows)]` scope"),
        "{reason}"
    );

    let negated = concat!(
        "#[cfg(not(windows))]\n",
        "mod sys { fn f() { let _ = unsafe { core::ptr::null::<u8>() }; } }\n",
    );
    assert!(
        architecture_checks::source::classify_unsafe_scope(
            architecture_checks::source::APPROVED_NATIVE_MODULE,
            negated
        )
        .is_err(),
        "a negated gate is not a Windows gate"
    );

    let other_os = concat!(
        "#[cfg(target_os = \"linux\")]\n",
        "mod sys { fn f() { let _ = unsafe { core::ptr::null::<u8>() }; } }\n",
    );
    assert!(
        architecture_checks::source::classify_unsafe_scope(
            architecture_checks::source::APPROVED_NATIVE_MODULE,
            other_os
        )
        .is_err(),
        "a non-Windows gate is not a Windows gate"
    );

    assert!(
        architecture_checks::source::classify_unsafe_scope(
            architecture_checks::source::APPROVED_NATIVE_MODULE,
            "pub unsafe fn f() {}"
        )
        .is_err(),
        "an ungated unsafe fn must be refused even in the approved module"
    );
}

// ---------------------------------------------------------------------------
// `SOLO-WO-074 R01`: the two-path native scope and positive-gate semantics,
// proved against the real AST verdict rather than a duplicate rule.
// ---------------------------------------------------------------------------

/// The pre-existing Windows-only source-read boundary.
const SOURCE_READ_MODULE: &str = architecture_checks::source::APPROVED_NATIVE_MODULE;

/// The reserved future process-native boundary. The file does not exist yet.
const RESERVED_PROCESS_MODULE: &str = architecture_checks::source::APPROVED_PROCESS_NATIVE_MODULE;

/// One module whose body contains exactly one `unsafe` block, under `attribute`.
fn gated_unsafe(attribute: &str) -> String {
    format!(
        "{attribute}\nmod sys {{\n    fn f() {{\n        let _ = unsafe {{ core::ptr::null::<u8>() }};\n    }}\n}}\n"
    )
}

/// The real, single unsafe-scope rule, applied to one synthetic source.
fn verdict(location: &str, source: &str) -> Result<(), String> {
    architecture_checks::source::classify_unsafe_scope(location, source)
}

#[test]
fn g074_02_the_source_read_module_remains_windows_only() {
    assert!(
        verdict(SOURCE_READ_MODULE, &gated_unsafe("#[cfg(windows)]")).is_ok(),
        "the source-read boundary must keep accepting Windows-gated unsafe"
    );
    for (label, attribute) in [
        ("Linux", "#[cfg(target_os = \"linux\")]"),
        ("negated Windows", "#[cfg(not(windows))]"),
        ("negated Linux", "#[cfg(not(target_os = \"linux\"))]"),
        ("wrong platform", "#[cfg(target_os = \"macos\")]"),
    ] {
        assert!(
            verdict(SOURCE_READ_MODULE, &gated_unsafe(attribute)).is_err(),
            "the source-read boundary must refuse {label} unsafe, because it is Windows-only"
        );
    }
    assert!(
        verdict(
            SOURCE_READ_MODULE,
            "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }"
        )
        .is_err(),
        "the source-read boundary must refuse unscoped unsafe"
    );
    let reason = verdict(
        SOURCE_READ_MODULE,
        &gated_unsafe("#[cfg(target_os = \"linux\")]"),
    )
    .expect_err("a Linux gate must be refused on the Windows-only file");
    assert!(
        reason.contains("Windows-only source-read boundary"),
        "{reason}"
    );
}

#[test]
fn g074_03_the_reserved_process_path_accepts_only_positive_os_gates() {
    for (label, attribute) in [
        ("Linux", "#[cfg(target_os = \"linux\")]"),
        ("Windows", "#[cfg(windows)]"),
        ("all(test, windows)", "#[cfg(all(test, windows))]"),
        (
            "all(test, Linux)",
            "#[cfg(all(test, target_os = \"linux\"))]",
        ),
        (
            "all(windows, all(test, windows))",
            "#[cfg(all(windows, all(test, windows)))]",
        ),
    ] {
        assert!(
            verdict(RESERVED_PROCESS_MODULE, &gated_unsafe(attribute)).is_ok(),
            "the reserved process path must accept {label}-gated unsafe"
        );
    }
    assert!(
        verdict(
            RESERVED_PROCESS_MODULE,
            "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }"
        )
        .is_err(),
        "the reserved process path must refuse unscoped unsafe"
    );
    assert!(
        verdict(
            RESERVED_PROCESS_MODULE,
            "pub unsafe fn f() {}\n#[cfg(windows)]\npub unsafe fn g() {}"
        )
        .is_err(),
        "one ungated unsafe fn is enough to refuse the whole file"
    );
}

#[test]
fn g074_04_no_ambiguous_predicate_establishes_an_os_proof() {
    for (label, attribute) in [
        (
            "any(windows, feature)",
            "#[cfg(any(windows, feature = \"x\"))]",
        ),
        (
            "any(Linux, feature)",
            "#[cfg(any(target_os = \"linux\", feature = \"x\"))]",
        ),
        ("any(windows, unix)", "#[cfg(any(windows, unix))]"),
        ("not(test)", "#[cfg(not(test))]"),
        ("not(windows)", "#[cfg(not(windows))]"),
        ("target_os = macos", "#[cfg(target_os = \"macos\")]"),
        ("target_family", "#[cfg(target_family = \"windows\")]"),
        ("cfg_attr only", "#[cfg_attr(windows, allow(dead_code))]"),
        ("malformed all()", "#[cfg(all())]"),
        (
            "all(any, Linux)",
            "#[cfg(all(any(unix, windows), target_os = \"linux\"))]",
        ),
        (
            "all(windows, Linux)",
            "#[cfg(all(windows, target_os = \"linux\"))]",
        ),
    ] {
        for location in [RESERVED_PROCESS_MODULE, SOURCE_READ_MODULE] {
            assert!(
                verdict(location, &gated_unsafe(attribute)).is_err(),
                "{label} must not be treated as an OS proof at `{location}`"
            );
        }
    }
}

#[test]
fn g074_05_unsafe_outside_the_two_modules_is_always_refused() {
    let gated = gated_unsafe("#[cfg(windows)]");
    let linux = gated_unsafe("#[cfg(target_os = \"linux\")]");
    for location in [
        "crates/solo-domain/src/lib.rs",
        "crates/solo-application/src/media_worker.rs",
        "crates/solo-adapters/src/media_worker_host.rs",
        "crates/solo-adapters/src/lib.rs",
        "crates/solo-cli/src/main.rs",
        "crates/solo-cli/tests/unsafe_boundary.rs",
        "crates/solo-adapters/src/media_worker_host_native_copy.rs",
        "crates/solo-adapters/src/media_worker_host_native.rs.bak",
        "crates/solo-adapters/src/source_read_native_extra.rs",
        "src/media_worker_host_native.rs",
    ] {
        assert!(
            verdict(location, &gated).is_err(),
            "Windows-gated unsafe must be refused at `{location}`"
        );
        assert!(
            verdict(location, &linux).is_err(),
            "Linux-gated unsafe must be refused at `{location}`"
        );
    }
    assert!(
        verdict(
            "crates/solo-domain/src/lib.rs",
            "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }"
        )
        .is_err(),
        "unscoped unsafe is refused everywhere"
    );

    // A path that arrives with Windows separators must be normalised before the comparison, so
    // an ineligible file cannot become eligible and the exact reserved path still matches.
    assert!(
        verdict(
            "crates\\solo-adapters\\src\\media_worker_host_native_extra.rs",
            &linux
        )
        .is_err(),
        "a normalised-ineligible path must stay ineligible"
    );
    assert!(
        verdict(
            "crates\\solo-adapters\\src\\media_worker_host_native.rs",
            &linux
        )
        .is_ok(),
        "the exact reserved path is recognised after separator normalisation"
    );
}
