//! Architecture guard over the **real** Cargo dependency graph, plus in-memory
//! negative fixtures proving the guard actually rejects forbidden edges.
//!
//! This is a graph-level guard only. The full source-level checker for aliases,
//! re-exports, macros, `include!`, qualified paths and dynamic loading is deferred to
//! `R01.B`; nothing here may be described as completing that package.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::process::Command;

use serde_json::Value;

const WORKSPACE: [&str; 4] = [
    "solo-domain",
    "solo-application",
    "solo-adapters",
    "solo-cli",
];

/// The single approved external crate, and only as a `solo-cli` dev-dependency.
const APPROVED_EXTERNAL_DEV: &str = "serde_json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DepKind {
    Normal,
    Dev,
    Build,
}

impl DepKind {
    fn from_metadata(kind: Option<&str>) -> Self {
        match kind {
            Some("dev") => Self::Dev,
            Some("build") => Self::Build,
            _ => Self::Normal,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Dev => "dev",
            Self::Build => "build",
        }
    }
}

/// One directed dependency edge, named by the **true** package, with any authored
/// rename recorded separately so that an alias can never disguise an edge.
#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    kind: DepKind,
    alias: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct Graph {
    nodes: Vec<String>,
    edges: Vec<Edge>,
}

fn is_workspace(name: &str) -> bool {
    WORKSPACE.contains(&name)
}

/// The inward-only rule set for this slice.
fn edge_allowed(edge: &Edge) -> bool {
    match edge.from.as_str() {
        // The pure domain depends on nothing at all: not std-plus-crates, not a
        // sibling, and never on an outer layer.
        "solo-domain" => false,
        // The application layer may depend on the domain only.
        "solo-application" => edge.to == "solo-domain",
        // The adapter may depend on the application and the domain.
        "solo-adapters" => edge.to == "solo-domain" || edge.to == "solo-application",
        // The composition root may depend on inward workspace crates, and on the one
        // approved external crate as a dev-dependency only.
        "solo-cli" => {
            is_workspace(&edge.to)
                || (edge.kind == DepKind::Dev && edge.to == APPROVED_EXTERNAL_DEV)
        }
        // Internals of a third-party crate are outside this slice's boundary rule.
        _ => true,
    }
}

fn is_inner_layer(name: &str) -> bool {
    matches!(name, "solo-domain" | "solo-application" | "solo-adapters")
}

fn reachable_from(graph: &Graph, root: &str) -> (BTreeSet<String>, bool) {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut queue: VecDeque<String> = VecDeque::from([root.to_owned()]);
    let mut cyclic = false;

    while let Some(node) = queue.pop_front() {
        for edge in graph.edges.iter().filter(|edge| edge.from == node) {
            if edge.to == root {
                cyclic = true;
            }
            if seen.insert(edge.to.clone()) {
                queue.push_back(edge.to.clone());
            }
        }
    }

    (seen, cyclic)
}

/// Checks a graph against the inward-only contract.
///
/// `guard_enabled` exists so a test can demonstrate that the negatives below are
/// load-bearing: with the guard switched off every forbidden fixture would pass, so
/// disabling it fails this suite rather than silently weakening it.
fn check(graph: &Graph, guard_enabled: bool) -> Result<(), Vec<String>> {
    if !guard_enabled {
        return Ok(());
    }

    let mut violations: BTreeSet<String> = BTreeSet::new();

    for edge in &graph.edges {
        if is_workspace(&edge.from) && !edge_allowed(edge) {
            let alias = edge
                .alias
                .as_ref()
                .map_or_else(String::new, |name| format!(" (aliased as {name})"));
            violations.insert(format!(
                "forbidden {} edge {}{} -> {}",
                edge.kind.label(),
                edge.from,
                alias,
                edge.to
            ));
        }
    }

    for root in WORKSPACE {
        if !graph.nodes.iter().any(|node| node == root) {
            continue;
        }
        let (reached, cyclic) = reachable_from(graph, root);
        if cyclic {
            violations.insert(format!("cycle detected through {root}"));
        }
        if !is_inner_layer(root) {
            continue;
        }
        for node in &reached {
            if is_workspace(node) {
                let probe = Edge {
                    from: root.to_owned(),
                    to: node.clone(),
                    kind: DepKind::Normal,
                    alias: None,
                };
                if !edge_allowed(&probe) {
                    violations.insert(format!(
                        "transitive forbidden reachability {root} -> {node}"
                    ));
                }
            } else {
                violations.insert(format!("transitive external reachability {root} -> {node}"));
            }
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations.into_iter().collect())
    }
}

// ---------------------------------------------------------------------------
// In-memory fixtures: the guard must reject every forbidden shape.
// ---------------------------------------------------------------------------

fn legal_graph() -> Graph {
    let edge = |from: &str, to: &str, kind: DepKind| Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        kind,
        alias: None,
    };

    Graph {
        nodes: [
            "solo-domain",
            "solo-application",
            "solo-adapters",
            "solo-cli",
            "serde_json",
        ]
        .iter()
        .map(|name| (*name).to_owned())
        .collect(),
        edges: vec![
            edge("solo-application", "solo-domain", DepKind::Normal),
            edge("solo-adapters", "solo-application", DepKind::Normal),
            edge("solo-adapters", "solo-domain", DepKind::Normal),
            edge("solo-cli", "solo-adapters", DepKind::Normal),
            edge("solo-cli", "solo-application", DepKind::Normal),
            edge("solo-cli", "serde_json", DepKind::Dev),
            edge("serde_json", "itoa", DepKind::Normal),
        ],
    }
}

fn fixture_with(extra: Edge) -> Graph {
    let mut graph = legal_graph();
    graph.edges.push(extra);
    graph
}

fn edge(from: &str, to: &str, kind: DepKind) -> Edge {
    Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        kind,
        alias: None,
    }
}

#[test]
fn the_legal_fixture_passes() {
    check(&legal_graph(), true).expect("the inward-only shape must be accepted");
}

#[test]
fn rejects_a_domain_edge_to_an_adapter() {
    let errors = check(
        &fixture_with(edge("solo-domain", "solo-adapters", DepKind::Normal)),
        true,
    )
    .expect_err("must reject");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-domain") && error.contains("solo-adapters"))
    );
}

#[test]
fn rejects_an_application_edge_to_the_cli() {
    let errors = check(
        &fixture_with(edge("solo-application", "solo-cli", DepKind::Normal)),
        true,
    )
    .expect_err("must reject");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-application") && error.contains("solo-cli"))
    );
}

#[test]
fn rejects_an_application_edge_to_an_adapter() {
    let errors = check(
        &fixture_with(edge("solo-application", "solo-adapters", DepKind::Normal)),
        true,
    )
    .expect_err("must reject");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-application") && error.contains("solo-adapters"))
    );
}

#[test]
fn rejects_a_domain_edge_to_an_external_crate() {
    let errors = check(
        &fixture_with(edge("solo-domain", "memchr", DepKind::Normal)),
        true,
    )
    .expect_err("must reject");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-domain") && error.contains("memchr"))
    );
}

#[test]
fn rejects_a_renamed_illegal_edge() {
    let mut renamed = edge("solo-adapters", "solo-cli", DepKind::Normal);
    renamed.alias = Some("upstream".to_owned());
    let errors = check(&fixture_with(renamed), true).expect_err("must reject");
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-adapters") && error.contains("solo-cli")),
        "a rename must not hide the edge: {errors:?}"
    );
}

#[test]
fn rejects_illegal_dev_and_build_edges() {
    for kind in [DepKind::Dev, DepKind::Build] {
        let errors = check(
            &fixture_with(edge("solo-application", "solo-cli", kind)),
            true,
        )
        .expect_err("must reject");
        assert!(errors.iter().any(|error| error.contains(kind.label())));
    }
}

#[test]
fn rejects_a_cycle_between_workspace_crates() {
    let errors = check(
        &fixture_with(edge("solo-domain", "solo-application", DepKind::Normal)),
        true,
    )
    .expect_err("must reject");
    assert!(errors.iter().any(|error| error.contains("cycle")));
}

#[test]
fn disabling_the_guard_would_make_these_negatives_pass() {
    let forbidden = fixture_with(edge("solo-domain", "solo-adapters", DepKind::Normal));

    // Proof that the guard is load-bearing rather than decorative: switched off it
    // accepts the very graph every negative test above rejects, so removing or
    // bypassing it fails this suite instead of weakening the contract.
    check(&forbidden, false).expect("a disabled guard accepts anything");
    check(&forbidden, true).expect_err("the enabled guard must reject the same graph");
}

// ---------------------------------------------------------------------------
// The real workspace: read actual Cargo metadata, not a hand-written description.
// ---------------------------------------------------------------------------

fn metadata() -> Value {
    let manifest_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--all-features",
        ])
        .current_dir(manifest_dir)
        .output()
        .expect("cargo metadata must be runnable");

    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("cargo metadata must emit valid JSON")
}

fn real_graph(value: &Value) -> Graph {
    let mut id_to_name: BTreeMap<String, String> = BTreeMap::new();
    let mut nodes: Vec<String> = Vec::new();

    for package in value["packages"].as_array().expect("packages array") {
        let name = package["name"].as_str().expect("package name").to_owned();
        let id = package["id"].as_str().expect("package id").to_owned();
        id_to_name.insert(id, name.clone());
        nodes.push(name);
    }

    let mut edges: Vec<Edge> = Vec::new();

    // Declared edges, which carry the precise dependency kind and any rename.
    for package in value["packages"].as_array().expect("packages array") {
        let from = package["name"].as_str().expect("package name").to_owned();
        for dependency in package["dependencies"]
            .as_array()
            .expect("dependencies array")
        {
            edges.push(Edge {
                from: from.clone(),
                to: dependency["name"]
                    .as_str()
                    .expect("dependency name")
                    .to_owned(),
                kind: DepKind::from_metadata(dependency["kind"].as_str()),
                alias: dependency["rename"].as_str().map(str::to_owned),
            });
        }
    }

    // Resolved edges, including transitive and feature-activated ones.
    if let Some(resolve) = value.get("resolve").filter(|value| !value.is_null()) {
        for node in resolve["nodes"].as_array().expect("resolve nodes") {
            let Some(from) = id_to_name
                .get(node["id"].as_str().expect("node id"))
                .cloned()
            else {
                continue;
            };
            for dependency in node["deps"].as_array().expect("node deps") {
                let Some(to) = id_to_name
                    .get(dependency["pkg"].as_str().expect("dep pkg"))
                    .cloned()
                else {
                    continue;
                };
                let kind = dependency["dep_kinds"]
                    .as_array()
                    .and_then(|kinds| kinds.first())
                    .map_or(DepKind::Normal, |kind| {
                        DepKind::from_metadata(kind["kind"].as_str())
                    });
                edges.push(Edge {
                    from: from.clone(),
                    to,
                    kind,
                    alias: None,
                });
            }
        }
    }

    Graph { nodes, edges }
}

#[test]
fn real_workspace_contains_exactly_the_four_required_crates() {
    let value = metadata();
    let mut id_to_name: BTreeMap<String, String> = BTreeMap::new();
    for package in value["packages"].as_array().expect("packages array") {
        id_to_name.insert(
            package["id"].as_str().expect("package id").to_owned(),
            package["name"].as_str().expect("package name").to_owned(),
        );
    }

    let members: BTreeSet<String> = value["workspace_members"]
        .as_array()
        .expect("workspace_members array")
        .iter()
        .filter_map(|id| id_to_name.get(id.as_str().expect("member id")).cloned())
        .collect();

    let expected: BTreeSet<String> = WORKSPACE.iter().map(|name| (*name).to_owned()).collect();
    assert_eq!(
        members, expected,
        "the workspace must hold exactly these crates"
    );
}

#[test]
fn real_dependency_graph_is_inward_only() {
    let value = metadata();
    let graph = real_graph(&value);
    if let Err(violations) = check(&graph, true) {
        panic!("the real workspace violates the inward-only contract: {violations:#?}");
    }
}

#[test]
fn only_the_approved_external_dependency_is_declared() {
    let value = metadata();

    for package in value["packages"].as_array().expect("packages array") {
        let name = package["name"].as_str().expect("package name");
        if !is_workspace(name) {
            continue;
        }
        for dependency in package["dependencies"]
            .as_array()
            .expect("dependencies array")
        {
            let target = dependency["name"].as_str().expect("dependency name");
            if is_workspace(target) {
                continue;
            }
            assert_eq!(
                (name, target, dependency["kind"].as_str()),
                ("solo-cli", APPROVED_EXTERNAL_DEV, Some("dev")),
                "only the approved dev-dependency may be declared, found {name} -> {target}"
            );
        }
    }
}

#[test]
fn real_locked_graph_resolves_without_floating_versions() {
    let value = metadata();
    let resolve = value["resolve"]
        .as_object()
        .expect("resolve object present");
    let nodes = resolve["nodes"].as_array().expect("resolve nodes");

    assert!(!nodes.is_empty(), "the resolved graph must not be empty");
    assert!(
        nodes.iter().all(|node| node["id"].is_string()),
        "every resolved node must carry an identity"
    );
}
