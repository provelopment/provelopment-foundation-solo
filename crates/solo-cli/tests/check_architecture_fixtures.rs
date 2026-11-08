//! Ordinary `#[test]` gate for the architecture checker, run by
//! `cargo test --workspace --all-targets --locked`.
//!
//! This target is the reason a violation can never be "only reported": the real tree must
//! satisfy the boundary contract, every forbidden fixture must be refused by its expected
//! category, and every legal twin must be accepted — as ordinary test cases.

mod architecture_checks;

use std::path::{Path, PathBuf};

use architecture_checks::graph::TargetKind;
use architecture_checks::source::{self, FixtureWorld, FsReader};
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

// ---------------------------------------------------------------------------
// Real filesystem redirects. These use genuine links on an exclusively owned
// scratch tree, exercising the **same** checking routines as the real tree.
// ---------------------------------------------------------------------------

/// An exclusively owned scratch tree. Dropping it removes only what this test created,
/// and never through a link.
struct Scratch {
    root: PathBuf,
    links: Vec<PathBuf>,
}

impl Scratch {
    fn new(label: &str) -> Self {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        // `mklink` rejects a path containing `..`, so the scratch root is canonical.
        let workspace = std::fs::canonicalize(&workspace).unwrap_or(workspace);
        let unique = format!(
            "{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|elapsed| elapsed.as_nanos())
                .unwrap_or(0)
        );
        let root = workspace
            .join(".temp")
            .join("architecture-fixtures")
            .join(unique);
        std::fs::create_dir_all(&root).expect("scratch root must be creatable");
        Self {
            root,
            links: Vec::new(),
        }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn write(&self, relative: &str, text: &str) -> PathBuf {
        let path = self.path(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("scratch directory must be creatable");
        }
        std::fs::write(&path, text).expect("scratch file must be writable");
        path
    }

    fn link_dir(&mut self, relative: &str, target: &Path) -> bool {
        let link = self.path(relative);
        if let Some(parent) = link.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if create_dir_link(&link, target) {
            self.links.push(link);
            true
        } else {
            false
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // Remove every link first: a recursive delete must never traverse a redirect into
        // its target.
        for link in &self.links {
            let _ = remove_dir_link(link);
        }
        if self.root.is_dir() {
            let _ = std::fs::remove_dir_all(&self.root);
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

fn real_spec(root: &Path) -> source::UnitSpec {
    let world = FixtureWorld::standard();
    let mut spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
    spec.package_root = root.to_path_buf();
    spec.root_file = root.join("src").join("lib.rs");
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

fn categories(violations: &[Violation]) -> Vec<&'static str> {
    violations
        .iter()
        .map(|violation| violation.category.as_str())
        .collect()
}

/// A lexically in-package `include!` whose directory component is a real redirect.
#[test]
fn a_redirected_include_is_refused_before_its_content_is_consumed() {
    let mut scratch = Scratch::new("include-redirect");
    let target = scratch.path("outside");
    std::fs::create_dir_all(&target).expect("target directory");
    scratch.write(
        "outside/forbidden.inc",
        "// redirected payload: must never be consumed\nuse solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 1 }\n",
    );
    scratch.write("package/src/lib.rs", "include!(\"alias/forbidden.inc\");\n");

    if !scratch.link_dir("package/src/alias", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             include! escape gate cannot be evidenced here"
        );
    }

    let package_root = scratch.path("package");
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
    let found = source::check_unit(&real_spec(&package_root), &FsReader);

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
            .any(|violation| violation.category == Category::UnparsableSource),
        "the redirected source must not be parsed at all, saw {:?}",
        categories(&found)
    );
    // The payload carries a forbidden reference: if the file were consumed, that
    // reference would be reported too. Its absence proves the content was never read.
    assert!(
        !found
            .iter()
            .any(|violation| violation.category == Category::LayerOutwardReference),
        "the redirected content must never be scanned, saw {:?}",
        categories(&found)
    );
    assert!(
        lexical.exists(),
        "the redirect itself must be preserved, not deleted"
    );
    assert_target_preserved(&before, &target);
}

/// A `#[path]` module whose redirected directory escapes the package.
#[test]
fn a_redirected_path_module_is_refused() {
    let mut scratch = Scratch::new("path-redirect");
    let target = scratch.path("outside");
    std::fs::create_dir_all(&target).expect("target directory");
    scratch.write(
        "outside/escape_mod.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 2 }\n",
    );
    scratch.write(
        "package/src/lib.rs",
        "#[path = \"alias/escape_mod.rs\"]\nmod escaped;\n",
    );

    if !scratch.link_dir("package/src/alias", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             #[path] escape gate cannot be evidenced here"
        );
    }

    let before = snapshot(&target);
    let found = source::check_unit(&real_spec(&scratch.path("package")), &FsReader);

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
    let mut scratch = Scratch::new("default-mod-redirect");
    let target = scratch.path("outside");
    std::fs::create_dir_all(&target).expect("target directory");
    scratch.write(
        "outside/mod.rs",
        "use solo_adapters::CliOutcome;\npub fn leaked(_: CliOutcome) -> u8 { 3 }\n",
    );
    scratch.write("package/src/lib.rs", "mod escaped;\n");

    if !scratch.link_dir("package/src/escaped", &target) {
        panic!(
            "UNVERIFIED: this host could not create a genuine directory redirect, so the \
             default-module escape gate cannot be evidenced here"
        );
    }

    let before = snapshot(&target);
    let found = source::check_unit(&real_spec(&scratch.path("package")), &FsReader);

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
    let scratch = Scratch::new("cfg-attr-default");
    scratch.write(
        "package/src/lib.rs",
        "#[cfg_attr(windows, path = \"alt_win.rs\")]\nmod alternate;\n",
    );
    scratch.write("package/src/alternate.rs", "pub fn from_default() {}\n");
    scratch.write("package/src/alt_win.rs", "pub fn from_conditional() {}\n");

    let package_root = scratch.path("package");
    assert!(
        package_root.join("src").join("alternate.rs").is_file(),
        "the default source must exist, so passing merely because it exists is not the test"
    );

    let found = source::check_unit(&real_spec(&package_root), &FsReader);
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
    let scratch = Scratch::new("missing-module");
    scratch.write("package/src/lib.rs", "mod missing;\n");

    let found = source::check_unit(&real_spec(&scratch.path("package")), &FsReader);
    assert!(
        found
            .iter()
            .any(|violation| violation.category == Category::ModuleSourceUnresolved),
        "a missing module must be refused as module-source-unresolved, saw {:?}",
        categories(&found)
    );
}

/// Legal in-package module and `include!` loading must keep working on the real filesystem.
#[test]
fn legal_in_package_loading_still_succeeds() {
    let scratch = Scratch::new("legal-loading");
    scratch.write(
        "package/src/lib.rs",
        "include!(\"fragment.inc\");\nmod inner;\n",
    );
    scratch.write("package/src/fragment.inc", "pub fn fragment() {}\n");
    scratch.write("package/src/inner.rs", "pub fn inner() {}\n");

    let found = source::check_unit(&real_spec(&scratch.path("package")), &FsReader);
    assert!(
        found.is_empty(),
        "legal in-package loading must be accepted, saw {:?}",
        categories(&found)
    );
}
