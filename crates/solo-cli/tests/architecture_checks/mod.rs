//! Shared, test-only architecture enforcement for the Foundation Solo workspace.
//!
//! Two layers, both mandatory (Architecture §6A.3):
//!
//! * [`graph`] — the real Cargo dependency graph read from `cargo metadata`, with the
//!   inward-only rule set and an in-memory model for negative fixtures.
//! * [`source`] — a fail-closed `syn` AST checker over every Rust source file reachable
//!   from the four packages and their declared targets, including aliases, re-exports,
//!   qualified paths, `crate`/`self`/`super` routes, `#[path]`, `include!`, macro
//!   arguments and dynamic-loading attempts.
//!
//! The checker deliberately has **no** disable switch, environment override or
//! partial-scan success path: [`check_real_tree`] is what both the `check-architecture`
//! process and the ordinary `cargo test` gate call.
//!
//! This is a shared, test-only library compiled into three test targets
//! (`check-architecture`, `check_architecture_fixtures` and `workspace_graph`). Each
//! target exercises a different subset of it, so per-target unused analysis would report
//! the rest as dead code. That is expected for a shared checker; nothing here is linked
//! into the shipped `solo-cli` binary, and the allowance is scoped to this module only.
#![allow(dead_code)]

pub mod graph;
pub mod source;

use std::collections::BTreeSet;
use std::fmt;

/// Stable, bounded category for a refusal. Never derived from user data, so a
/// diagnostic can be quoted in a report without leaking input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    // Graph level
    MetadataUnavailable,
    GraphPackageSet,
    GraphIllegalEdge,
    GraphUnapprovedDependency,
    GraphCycle,
    GraphExternalReach,
    UnsupportedTarget,
    // Source level: layering and identity
    LayerOutwardReference,
    UndeclaredCrate,
    UnapprovedExternalCrate,
    PrivateCrateInternal,
    UnresolvedPath,
    // Source level: module and content loading
    ModuleSourceUnresolved,
    ModuleSourceAmbiguous,
    ModuleSourceEscape,
    ConditionalModulePath,
    IncludeUnsupported,
    IncludeEscape,
    HiddenSource,
    UnreadableSource,
    UnparsableSource,
    UnsupportedItem,
    // Source level: macros, unsafe and dynamic loading
    MacroUnsupported,
    DynamicLoading,
    UnsafeCode,
    ForeignBlock,
}

impl Category {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MetadataUnavailable => "metadata-unavailable",
            Self::GraphPackageSet => "graph-package-set",
            Self::GraphIllegalEdge => "graph-illegal-edge",
            Self::GraphUnapprovedDependency => "graph-unapproved-dependency",
            Self::GraphCycle => "graph-cycle",
            Self::GraphExternalReach => "graph-external-reach",
            Self::UnsupportedTarget => "unsupported-target",
            Self::LayerOutwardReference => "layer-outward-reference",
            Self::UndeclaredCrate => "undeclared-crate",
            Self::UnapprovedExternalCrate => "unapproved-external-crate",
            Self::PrivateCrateInternal => "private-crate-internal",
            Self::UnresolvedPath => "unresolved-path",
            Self::ModuleSourceUnresolved => "module-source-unresolved",
            Self::ModuleSourceAmbiguous => "module-source-ambiguous",
            Self::ModuleSourceEscape => "module-source-escape",
            Self::ConditionalModulePath => "conditional-module-path",
            Self::IncludeUnsupported => "include-unsupported",
            Self::IncludeEscape => "include-escape",
            Self::HiddenSource => "hidden-source",
            Self::UnreadableSource => "unreadable-source",
            Self::UnparsableSource => "unparsable-source",
            Self::UnsupportedItem => "unsupported-item",
            Self::MacroUnsupported => "macro-unsupported",
            Self::DynamicLoading => "dynamic-loading",
            Self::UnsafeCode => "unsafe-code",
            Self::ForeignBlock => "foreign-block",
        }
    }
}

/// One refusal: a stable category, a repository-relative location and a short reason.
///
/// Ordering is total so that output is byte-stable across runs and machines.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Violation {
    pub category: Category,
    pub location: String,
    pub detail: String,
}

impl Violation {
    pub fn new(category: Category, location: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            category,
            location: location.into(),
            detail: detail.into(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}: {}",
            self.category.as_str(),
            self.location,
            self.detail
        )
    }
}

/// Deduplicated, sorted refusal set.
pub fn sort_unique(violations: Vec<Violation>) -> Vec<Violation> {
    let set: BTreeSet<Violation> = violations.into_iter().collect();
    set.into_iter().collect()
}

// ---------------------------------------------------------------------------
// Process contract for the test-only checker.
//
// A deliberately illegal fixture must be *observably* refused: "nonzero" alone cannot
// distinguish a policy refusal from the harness failing to refuse it, so acceptance gets
// its own status and its own outcome word.
// ---------------------------------------------------------------------------

/// The real tree is legal (default gate and `--self-test` success).
pub const EXIT_LEGAL: u8 = 0;
/// A forbidden fixture was correctly refused by the policy, or the real tree is illegal.
pub const EXIT_REFUSED: u8 = 1;
/// The command line was not understood; no flag can weaken the checker.
pub const EXIT_USAGE: u8 = 2;
/// A forbidden fixture was **accepted**: the checker failed, this is not evidence.
pub const EXIT_HARNESS_FAILURE: u8 = 3;

/// What actually happened when a fixture was evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureOutcome {
    /// The forbidden construct was refused, as required.
    Refused,
    /// The forbidden construct was accepted: a checker failure, never a pass.
    Accepted,
}

pub fn classify_fixture_result(violations: &[Violation]) -> FixtureOutcome {
    if violations.is_empty() {
        FixtureOutcome::Accepted
    } else {
        FixtureOutcome::Refused
    }
}

pub fn fixture_exit_code(outcome: FixtureOutcome) -> u8 {
    match outcome {
        FixtureOutcome::Refused => EXIT_REFUSED,
        FixtureOutcome::Accepted => EXIT_HARNESS_FAILURE,
    }
}

/// Stable, machine-checkable outcome text: the word `refused` (with the category that
/// caught it) is never confusable with `accepted`.
pub fn fixture_outcome_text(fixture: &Fixture, violations: &[Violation]) -> String {
    let outcome = classify_fixture_result(violations);
    let status = match outcome {
        FixtureOutcome::Refused => "refused",
        FixtureOutcome::Accepted => "accepted",
    };
    let matched = violations
        .iter()
        .any(|violation| violation.category == fixture.expected);
    format!(
        "fixture={} class=\"{}\" outcome={} expected={} findings={} category_match={}",
        fixture.id,
        fixture.class,
        status,
        fixture.expected.as_str(),
        violations.len(),
        matched
    )
}

/// What the real-tree check actually inspected, for the report and for the process
/// output. Counts are observations, never a claim of compiler-level proof.
#[derive(Debug, Clone, Copy)]
pub struct RealTreeReport {
    pub packages: usize,
    pub units: usize,
    pub files: usize,
    pub edges: usize,
}

/// The one enforcement entry point: real Cargo graph **plus** real source tree.
///
/// There is no parameter, environment variable or CLI flag that can weaken it.
pub fn check_real_tree() -> Result<RealTreeReport, Vec<Violation>> {
    let metadata = graph::metadata().map_err(|violation| vec![violation])?;
    let packages = graph::packages(&metadata).map_err(|violation| vec![violation])?;

    let real = graph::real_graph(&metadata);
    let mut violations: Vec<Violation> = Vec::new();
    if let Err(found) = graph::check(&real) {
        violations.extend(found);
    }

    let source_report = match source::check_packages(&packages) {
        Ok(report) => Some(report),
        Err(found) => {
            violations.extend(found);
            None
        }
    };

    if !violations.is_empty() {
        return Err(sort_unique(violations));
    }

    let source_report = source_report.unwrap_or_default();
    Ok(RealTreeReport {
        packages: packages.len(),
        units: source_report.units,
        files: source_report.files,
        edges: real.edges.len(),
    })
}

/// One negative case: a representative forbidden construct and the category that must
/// catch it.
pub struct Fixture {
    pub id: &'static str,
    pub class: &'static str,
    pub expected: Category,
}

/// Every required negative class from the work order §5, plus one positive twin each.
pub const FIXTURES: [Fixture; 23] = [
    Fixture {
        id: "direct-outward",
        class: "direct outward dependency",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "renamed-alias",
        class: "renamed crate/alias",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "reexport-chain",
        class: "re-export chain",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "qualified-path",
        class: "qualified/reference path",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "private-internal",
        class: "crate/self/super and private sibling",
        expected: Category::PrivateCrateInternal,
    },
    Fixture {
        id: "undeclared-crate",
        class: "undeclared workspace crate",
        expected: Category::UndeclaredCrate,
    },
    Fixture {
        id: "path-attribute-escape",
        class: "module loading via #[path]",
        expected: Category::ModuleSourceEscape,
    },
    Fixture {
        id: "include-escape",
        class: "module loading via include!",
        expected: Category::IncludeEscape,
    },
    Fixture {
        id: "missing-module",
        class: "unresolved module source",
        expected: Category::ModuleSourceUnresolved,
    },
    Fixture {
        id: "macro-generated",
        class: "macro-generated edge",
        expected: Category::MacroUnsupported,
    },
    Fixture {
        id: "macro-argument",
        class: "forbidden path inside macro arguments",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "dynamic-load",
        class: "dynamic/foreign load",
        expected: Category::DynamicLoading,
    },
    Fixture {
        id: "unsafe-block",
        class: "unsafe escape hatch",
        expected: Category::UnsafeCode,
    },
    Fixture {
        id: "unparsable-source",
        class: "unsupported/invalid input",
        expected: Category::UnparsableSource,
    },
    Fixture {
        id: "syn-outside-cli",
        class: "approved tool used outside solo-cli",
        expected: Category::UnapprovedExternalCrate,
    },
    Fixture {
        id: "dev-in-runtime",
        class: "development dependency in a shipped target",
        expected: Category::UnapprovedExternalCrate,
    },
    Fixture {
        id: "graph-illegal",
        class: "illegal dev/build/transitive graph",
        expected: Category::GraphIllegalEdge,
    },
    Fixture {
        id: "graph-unapproved-dev",
        class: "illegal dev/build/transitive graph",
        expected: Category::GraphUnapprovedDependency,
    },
    Fixture {
        id: "checker-bypass",
        class: "checker bypass",
        expected: Category::LayerOutwardReference,
    },
    Fixture {
        id: "cfg-attr-path",
        class: "conditional module path with a valid default",
        expected: Category::ConditionalModulePath,
    },
    Fixture {
        id: "cfg-attr-nested",
        class: "nested conditional module path",
        expected: Category::ConditionalModulePath,
    },
    Fixture {
        id: "path-duplicate",
        class: "duplicate #[path] declarations",
        expected: Category::ModuleSourceAmbiguous,
    },
    Fixture {
        id: "path-nonliteral",
        class: "non-literal #[path] value",
        expected: Category::ModuleSourceUnresolved,
    },
];

fn source_case(
    package: &str,
    kind: graph::TargetKind,
    source: &str,
) -> (source::UnitSpec, source::MapReader) {
    let world = source::FixtureWorld::standard();
    let spec = world.spec(package, kind, "lib.rs", &["serde_json", "syn"]);
    let reader = source::MapReader::new().with(spec.root_file.clone(), source);
    (spec, reader)
}

/// Runs one fixture through the **same** routines the real tree uses.
///
/// `None` means the identifier is unknown (a caller error, never a pass).
pub fn evaluate_fixture(id: &str) -> Option<Vec<Violation>> {
    use graph::TargetKind;

    let found = match id {
        "direct-outward" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "use solo_adapters::CliOutcome;\npub fn f(_: CliOutcome) {}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "renamed-alias" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "extern crate solo_adapters as upstream;\npub fn f(_: upstream::CliOutcome) {}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "reexport-chain" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "mod inner {\n    pub use solo_adapters::CliOutcome;\n}\npub use inner::CliOutcome;\n",
            );
            source::check_unit(&spec, &reader)
        }
        "qualified-path" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "pub fn f() -> ::solo_adapters::CliOutcome {\n    loop {}\n}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "private-internal" => {
            let (spec, reader) = source_case(
                "solo-cli",
                TargetKind::Test,
                "use solo_adapters::bootstrap_cli::parse_request;\npub fn f() { let _ = parse_request; }\n",
            );
            source::check_unit(&spec, &reader)
        }
        "undeclared-crate" => {
            let world = source::FixtureWorld::standard();
            let mut spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
            spec.workspace_roots.remove("solo_adapters");
            let reader = source::MapReader::new()
                .with(spec.root_file.clone(), "use solo_adapters::CliOutcome;\n");
            source::check_unit(&spec, &reader)
        }
        "path-attribute-escape" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "#[path = \"../../solo_domain/src/lib.rs\"]\nmod stolen;\n",
            );
            source::check_unit(&spec, &reader)
        }
        "include-escape" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "include!(\"../../solo_domain/src/lib.rs\");\n",
            );
            source::check_unit(&spec, &reader)
        }
        "missing-module" => {
            let (spec, reader) = source_case("solo-application", TargetKind::Lib, "mod absent;\n");
            source::check_unit(&spec, &reader)
        }
        "macro-generated" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "macro_rules! hide {\n    () => {\n        use solo_adapters::CliOutcome;\n    };\n}\nhide!();\n",
            );
            source::check_unit(&spec, &reader)
        }
        "macro-argument" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "pub fn f() {\n    let _ = format!(\"{}\", solo_adapters::CliOutcome::run);\n}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "dynamic-load" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "use libloading::Library;\npub fn f(_: Library) {}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "unsafe-block" => {
            let (spec, reader) = source_case(
                "solo-domain",
                TargetKind::Lib,
                "pub fn f() {\n    unsafe {\n    }\n}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "unparsable-source" => {
            let (spec, reader) =
                source_case("solo-application", TargetKind::Lib, "pub fn broken( {\n");
            source::check_unit(&spec, &reader)
        }
        "syn-outside-cli" => {
            let (spec, reader) =
                source_case("solo-application", TargetKind::Lib, "use syn::Item;\n");
            source::check_unit(&spec, &reader)
        }
        "dev-in-runtime" => {
            let (spec, reader) = source_case(
                "solo-cli",
                TargetKind::Bin,
                "use serde_json::Value;\npub fn f(_: Value) {}\n",
            );
            source::check_unit(&spec, &reader)
        }
        _ => return remaining_fixture(id),
    };

    Some(found)
}

fn remaining_fixture(id: &str) -> Option<Vec<Violation>> {
    use graph::TargetKind;

    let found = match id {
        "graph-illegal" => {
            let graph = graph::Graph {
                nodes: ["solo-domain", "solo-adapters"]
                    .iter()
                    .map(|name| (*name).to_owned())
                    .collect(),
                edges: vec![graph::Edge {
                    from: "solo-domain".to_owned(),
                    to: "solo-adapters".to_owned(),
                    kind: graph::DepKind::Normal,
                    alias: Some("inward".to_owned()),
                }],
            };
            graph::check(&graph).err().unwrap_or_default()
        }
        "graph-unapproved-dev" => {
            let mut graph = graph::Graph {
                nodes: ["solo-application", "solo-cli", "serde_json", "syn"]
                    .iter()
                    .map(|name| (*name).to_owned())
                    .collect(),
                edges: Vec::new(),
            };
            // An external crate reached from an inner layer, plus an approved tool
            // declared as a *normal* (shipped) dependency instead of a dev one.
            graph.edges.push(graph::Edge {
                from: "solo-application".to_owned(),
                to: "serde_json".to_owned(),
                kind: graph::DepKind::Normal,
                alias: None,
            });
            graph.edges.push(graph::Edge {
                from: "solo-cli".to_owned(),
                to: "syn".to_owned(),
                kind: graph::DepKind::Normal,
                alias: Some("parser".to_owned()),
            });
            graph::check(&graph).err().unwrap_or_default()
        }
        // A forbidden reference hidden behind every short-circuit a lazy checker might
        // take: a `#[cfg(test)]` inline module, a function body, a closure and a macro
        // argument. Skipping any one of those branches makes this fixture pass, which
        // fails the self-test instead of silently weakening the contract.
        "checker-bypass" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "#[cfg(test)]\nmod deep {\n    fn wrapper() {\n        let _text = format!(\"{}\", {\n            let inner = || solo_adapters::CliOutcome::run;\n            let _ = inner();\n            \"value\"\n        });\n    }\n}\n",
            );
            source::check_unit(&spec, &reader)
        }
        // A conditional `#[path]` with a **real default** present: passing merely because
        // the default resolves is the exact defect this correction removes.
        "cfg-attr-path" => {
            let world = source::FixtureWorld::standard();
            let spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
            let src = spec.package_root.join("src");
            let reader = source::MapReader::new()
                .with(
                    spec.root_file.clone(),
                    "#[cfg_attr(windows, path = \"alternate_windows.rs\")]\nmod alternate;\n",
                )
                .with(src.join("alternate.rs"), "pub fn from_default() {}\n")
                .with(
                    src.join("alternate_windows.rs"),
                    "pub fn from_windows() {}\n",
                );
            source::check_unit(&spec, &reader)
        }
        "cfg-attr-nested" => {
            let world = source::FixtureWorld::standard();
            let spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
            let src = spec.package_root.join("src");
            let reader = source::MapReader::new()
                .with(
                    spec.root_file.clone(),
                    "#[cfg_attr(all(windows, not(test)), cfg_attr(unix, path = \"nested.rs\"))]\nmod nested;\n",
                )
                .with(src.join("nested.rs"), "pub fn from_default() {}\n")
                .with(src.join("nested_alt.rs"), "pub fn from_nested() {}\n");
            source::check_unit(&spec, &reader)
        }
        "path-duplicate" => {
            let world = source::FixtureWorld::standard();
            let spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
            let src = spec.package_root.join("src");
            let reader = source::MapReader::new()
                .with(
                    spec.root_file.clone(),
                    "#[path = \"first.rs\"]\n#[path = \"second.rs\"]\nmod dup;\n",
                )
                .with(src.join("first.rs"), "pub fn a() {}\n")
                .with(src.join("second.rs"), "pub fn b() {}\n");
            source::check_unit(&spec, &reader)
        }
        "path-nonliteral" => {
            let world = source::FixtureWorld::standard();
            let spec = world.spec("solo-application", TargetKind::Lib, "lib.rs", &[]);
            let src = spec.package_root.join("src");
            let reader = source::MapReader::new()
                .with(
                    spec.root_file.clone(),
                    "#[path = concat!(\"non\", \"literal.rs\")]\nmod nonlit;\n",
                )
                .with(src.join("nonliteral.rs"), "pub fn c() {}\n");
            source::check_unit(&spec, &reader)
        }
        _ => return None,
    };

    Some(found)
}

/// Positive twins: the same checker must accept the legal forms the real tree uses.
pub const POSITIVES: [&str; 5] = [
    "legal-application",
    "legal-cli-test-dev-deps",
    "legal-macro-usage",
    "legal-inline-module",
    "legal-cfg-attr-harmless",
];

pub fn evaluate_positive(id: &str) -> Option<Vec<Violation>> {
    use graph::TargetKind;

    let found = match id {
        "legal-application" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "use std::fmt;\nuse solo_domain::{BaseCandidateV1, EntityIdV1};\npub fn f(_: EntityIdV1, _: BaseCandidateV1, _: fmt::Formatter<'_>) {}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "legal-cli-test-dev-deps" => {
            let (spec, reader) = source_case(
                "solo-cli",
                TargetKind::Test,
                "use serde_json::Value;\nuse syn::Item;\nuse solo_adapters::{CliOutcome, run};\npub fn f(_: Value, _: Item, _: CliOutcome) { let _ = run; }\n",
            );
            source::check_unit(&spec, &reader)
        }
        "legal-macro-usage" => {
            let (spec, reader) = source_case(
                "solo-domain",
                TargetKind::Lib,
                "pub fn f() {\n    assert!(matches!(Some(1u8), Some(_)));\n    let values: Vec<u8> = vec![1, 2, 3];\n    let _ = format!(\"{}:{}\", values.len(), concat!(\"a\", \"b\"));\n}\n",
            );
            source::check_unit(&spec, &reader)
        }
        "legal-inline-module" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "mod inner {\n    use core::fmt;\n    pub fn f(_: fmt::Formatter<'_>) {}\n}\npub use inner::f;\n",
            );
            source::check_unit(&spec, &reader)
        }
        // A harmless `cfg_attr` that cannot select source must stay legal: the correction
        // refuses conditional *paths*, not conditional compilation in general.
        "legal-cfg-attr-harmless" => {
            let (spec, reader) = source_case(
                "solo-application",
                TargetKind::Lib,
                "#[cfg_attr(test, allow(dead_code))]\nmod inner {\n    pub fn helper() {}\n}\npub use inner::helper;\n",
            );
            source::check_unit(&spec, &reader)
        }
        _ => return None,
    };

    Some(found)
}

/// One reported self-test case.
pub struct CaseOutcome {
    pub id: String,
    pub detail: String,
    pub passed: bool,
}

/// Fixture identifiers, in declaration order, for the command surface and the report.
pub fn fixture_ids() -> Vec<&'static str> {
    FIXTURES.iter().map(|fixture| fixture.id).collect()
}

pub fn fixture_by_id(id: &str) -> Option<&'static Fixture> {
    FIXTURES.iter().find(|fixture| fixture.id == id)
}

/// The full matrix: the real tree must pass, every forbidden fixture must be refused by
/// its expected category, and every legal twin must be accepted.
///
/// This is the load-bearing proof. If the guard is disabled, short-circuited or made
/// permissive, a negative fixture stops being refused and this matrix reports a failure.
pub fn self_test() -> Vec<CaseOutcome> {
    let mut cases: Vec<CaseOutcome> = Vec::new();

    match check_real_tree() {
        Ok(report) => cases.push(CaseOutcome {
            id: "real-tree".to_owned(),
            detail: format!(
                "{} packages, {} declared targets, {} source files, {} graph edges",
                report.packages, report.units, report.files, report.edges
            ),
            passed: true,
        }),
        Err(found) => cases.push(CaseOutcome {
            id: "real-tree".to_owned(),
            detail: summarise(&found),
            passed: false,
        }),
    }

    for fixture in &FIXTURES {
        match evaluate_fixture(fixture.id) {
            None => cases.push(CaseOutcome {
                id: fixture.id.to_owned(),
                detail: "fixture is not registered".to_owned(),
                passed: false,
            }),
            Some(found) if found.is_empty() => cases.push(CaseOutcome {
                id: fixture.id.to_owned(),
                detail: format!(
                    "accepted, but {} was required: the guard is not load-bearing",
                    fixture.expected.as_str()
                ),
                passed: false,
            }),
            Some(found) => {
                let matched = found
                    .iter()
                    .any(|violation| violation.category == fixture.expected);
                cases.push(CaseOutcome {
                    id: fixture.id.to_owned(),
                    detail: if matched {
                        format!(
                            "refused as {} ({} finding(s))",
                            fixture.expected.as_str(),
                            found.len()
                        )
                    } else {
                        format!(
                            "refused, expected {}: {}",
                            fixture.expected.as_str(),
                            summarise(&found)
                        )
                    },
                    passed: matched,
                });
            }
        }
    }

    for id in POSITIVES {
        match evaluate_positive(id) {
            Some(found) if found.is_empty() => cases.push(CaseOutcome {
                id: id.to_owned(),
                detail: "legal form accepted".to_owned(),
                passed: true,
            }),
            Some(found) => cases.push(CaseOutcome {
                id: id.to_owned(),
                detail: summarise(&found),
                passed: false,
            }),
            None => cases.push(CaseOutcome {
                id: id.to_owned(),
                detail: "positive case is not registered".to_owned(),
                passed: false,
            }),
        }
    }

    cases
}

fn summarise(violations: &[Violation]) -> String {
    let mut text = String::new();
    for violation in violations.iter().take(3) {
        if !text.is_empty() {
            text.push_str("; ");
        }
        text.push_str(&violation.to_string());
    }
    if violations.len() > 3 {
        text.push_str(&format!("; (+{} more)", violations.len() - 3));
    }
    text
}
