//! Cargo dependency-graph enforcement: the **real** workspace graph plus an
//! in-memory model so negative fixtures exercise the *same* rule set.
//!
//! This is the graph half of the accepted layering (Architecture §6A.3). It is not a
//! substitute for the source checker in [`super::source`]: a legal Cargo graph can
//! still be defeated by an alias, re-export, qualified path, `#[path]`, `include!` or
//! macro-produced reference, which only the AST checker can see.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

use super::{Category, Violation};

/// The four — and only four — workspace packages.
pub const WORKSPACE: [&str; 4] = [
    "solo-domain",
    "solo-application",
    "solo-adapters",
    "solo-cli",
];

/// External crates approved for exactly one purpose: `solo-cli` development and test.
/// Anything else reaching the locked graph is an unapproved dependency.
pub const APPROVED_EXTERNAL_DEV: [&str; 2] = ["serde_json", "syn"];

/// The only package permitted to declare an approved external dev-dependency.
pub const APPROVED_DEV_OWNER: &str = "solo-cli";

/// External crates approved as **normal** (shipped) dependencies of exactly one owner:
/// the outer adapter. The list is closed and named by root, so a root that is not
/// written here is unapproved whatever version or feature selection it carries.
///
/// `rustix` and `windows-sys` are the two target-gated native source-read roots approved
/// by the Owner for `SOLO-WO-047 R02`. They are disjoint by target — `rustix` is declared
/// only for `cfg(target_os = "linux")` and `windows-sys` only for `cfg(windows)` — so a
/// single host never compiles both, and the locked graph therefore carries exactly one
/// `windows-sys`, which is what keeps `multiple-versions = "deny"` satisfied.
pub const APPROVED_ADAPTER_EXTERNAL_NORMAL: [&str; 5] =
    ["libc", "rustix", "serde", "serde_json", "windows-sys"];

/// The only package permitted to declare an approved external normal dependency.
pub const APPROVED_ADAPTER_EXTERNAL_OWNER: &str = "solo-adapters";

/// The exact audited locked dependency closure reachable from the four approved adapter
/// roots, taken from the one tracked `Cargo.lock`.
///
/// It is enumerated rather than computed so that a new transitive component cannot
/// appear silently: an unlisted node is an unapproved reachability and is refused.
pub const APPROVED_ADAPTER_CLOSURE: [&str; 18] = [
    "bitflags",
    "errno",
    "itoa",
    "libc",
    "linux-raw-sys",
    "memchr",
    "proc-macro2",
    "quote",
    "rustix",
    "serde",
    "serde_core",
    "serde_derive",
    "serde_json",
    "syn",
    "unicode-ident",
    "windows-link",
    "windows-sys",
    "zmij",
];

/// The single reviewed owner/kind/root rule for external dependency declarations.
///
/// Returns the **only** dependency kind `owner` may use to declare the external package
/// `target`, or `None` when that declaration is not permitted at all. Both the Cargo
/// graph guard and the real-metadata assertion call this one function, so the two layers
/// cannot drift into divergent copied allowlists.
#[must_use]
pub fn external_declaration_kind(owner: &str, target: &str) -> Option<DepKind> {
    if owner == APPROVED_DEV_OWNER && APPROVED_EXTERNAL_DEV.contains(&target) {
        return Some(DepKind::Dev);
    }
    if owner == APPROVED_ADAPTER_EXTERNAL_OWNER
        && APPROVED_ADAPTER_EXTERNAL_NORMAL.contains(&target)
    {
        return Some(DepKind::Normal);
    }
    None
}

/// Dependency kind as declared in the manifest and reported by Cargo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DepKind {
    Normal,
    Dev,
    Build,
}

impl DepKind {
    pub fn from_metadata(kind: Option<&str>) -> Self {
        match kind {
            Some("dev") => Self::Dev,
            Some("build") => Self::Build,
            _ => Self::Normal,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Dev => "dev",
            Self::Build => "build",
        }
    }
}

/// One directed dependency edge, named by the **true** package, with any authored
/// rename recorded separately so that an alias can never disguise an edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub kind: DepKind,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub nodes: Vec<String>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn edge(&self, from: &str, to: &str, kind: DepKind) -> Edge {
        Edge {
            from: from.to_owned(),
            to: to.to_owned(),
            kind,
            alias: None,
        }
    }

    pub fn with_edge(mut self, from: &str, to: &str, kind: DepKind) -> Self {
        let edge = self.edge(from, to, kind);
        self.edges.push(edge);
        self
    }
}

pub fn is_workspace(name: &str) -> bool {
    WORKSPACE.contains(&name)
}

/// The inward-only rule set for this slice. A single source of truth: the real-graph
/// tests and the checker both call this function.
pub fn edge_allowed(edge: &Edge) -> bool {
    match edge.from.as_str() {
        // The pure domain depends on nothing at all: not on a sibling and never on an
        // outer layer or an external crate.
        "solo-domain" => false,
        // The application layer may depend on the domain only.
        "solo-application" => edge.to == "solo-domain",
        // The adapter may depend on the application and the domain, and — as the outer
        // adapter holding the only native boundary — on the approved external roots as
        // *normal* dependencies. It may never depend outward on the composition root.
        "solo-adapters" => {
            edge.to == "solo-domain"
                || edge.to == "solo-application"
                || external_declaration_kind("solo-adapters", &edge.to) == Some(edge.kind)
        }
        // The composition root may depend on inward workspace crates, and — in
        // development only — on the approved external crates.
        "solo-cli" => {
            is_workspace(&edge.to)
                || external_declaration_kind("solo-cli", &edge.to) == Some(edge.kind)
        }
        // Internals of a third-party crate are outside this slice's boundary rule.
        _ => true,
    }
}

/// Whether `root` may reach the external package `node` **transitively** under the
/// reviewed rule.
///
/// Only the outer adapter may reach external code at all, and only inside its exact
/// audited locked closure. The domain and the application may reach none, so this
/// answers `false` for them however the closure is defined.
#[must_use]
pub fn adapter_closure_allows(root: &str, node: &str) -> bool {
    root == APPROVED_ADAPTER_EXTERNAL_OWNER && APPROVED_ADAPTER_CLOSURE.contains(&node)
}

pub fn is_inner_layer(name: &str) -> bool {
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
/// The enforcement path has no disable switch: every caller that can influence a
/// process exit reaches this function unconditionally.
pub fn check(graph: &Graph) -> Result<(), Vec<Violation>> {
    let mut violations: Vec<Violation> = Vec::new();

    for edge in &graph.edges {
        if !is_workspace(&edge.from) || edge_allowed(edge) {
            continue;
        }
        let alias = edge
            .alias
            .as_ref()
            .map_or_else(String::new, |name| format!(" (aliased as {name})"));
        let category = if is_workspace(&edge.to) {
            Category::GraphIllegalEdge
        } else {
            Category::GraphUnapprovedDependency
        };
        violations.push(Violation::new(
            category,
            format!("graph:{}->{}", edge.from, edge.to),
            format!(
                "forbidden {} edge {}{} -> {}",
                edge.kind.label(),
                edge.from,
                alias,
                edge.to
            ),
        ));
    }

    for root in WORKSPACE {
        if !graph.nodes.iter().any(|node| node == root) {
            continue;
        }
        let (reached, cyclic) = reachable_from(graph, root);
        if cyclic {
            violations.push(Violation::new(
                Category::GraphCycle,
                format!("graph:{root}"),
                format!("cycle detected through {root}"),
            ));
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
                    violations.push(Violation::new(
                        Category::GraphIllegalEdge,
                        format!("graph:{root}->{node}"),
                        format!("transitive forbidden reachability {root} -> {node}"),
                    ));
                }
            } else if !adapter_closure_allows(root, node) {
                violations.push(Violation::new(
                    Category::GraphExternalReach,
                    format!("graph:{root}->{node}"),
                    format!("transitive external reachability {root} -> {node}"),
                ));
            }
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        violations.sort();
        Err(violations)
    }
}

fn malformed(field: &str) -> Violation {
    Violation::new(
        Category::MetadataUnavailable,
        "cargo-metadata",
        format!("cargo metadata is missing or malformed: {field}"),
    )
}

/// Runs the real, locked `cargo metadata` for this workspace.
pub fn metadata() -> Result<Value, Violation> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

    let output = Command::new(cargo)
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .arg("--locked")
        .arg("--all-features")
        .arg("--manifest-path")
        .arg(&manifest)
        .output()
        .map_err(|_| {
            Violation::new(
                Category::MetadataUnavailable,
                "cargo-metadata",
                "cargo metadata could not be executed",
            )
        })?;

    if !output.status.success() {
        return Err(Violation::new(
            Category::MetadataUnavailable,
            "cargo-metadata",
            format!(
                "cargo metadata exited with {:?} (locked graph out of date or unavailable)",
                output.status.code()
            ),
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|_| {
        Violation::new(
            Category::MetadataUnavailable,
            "cargo-metadata",
            "cargo metadata output was not valid JSON",
        )
    })
}

/// Target kind of a declared Cargo target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetKind {
    Lib,
    Bin,
    Test,
    Bench,
    Example,
    CustomBuild,
}

impl TargetKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Lib => "lib",
            Self::Bin => "bin",
            Self::Test => "test",
            Self::Bench => "bench",
            Self::Example => "example",
            Self::CustomBuild => "custom-build",
        }
    }

    /// Whether items in this target are test context, where dev-dependencies are legal.
    pub fn is_test_context(self) -> bool {
        matches!(self, Self::Test | Self::Bench)
    }
}

fn target_kind(kinds: &[String]) -> Option<TargetKind> {
    for kind in kinds {
        match kind.as_str() {
            "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro" => {
                return Some(TargetKind::Lib);
            }
            "bin" => return Some(TargetKind::Bin),
            "test" => return Some(TargetKind::Test),
            "bench" => return Some(TargetKind::Bench),
            "example" => return Some(TargetKind::Example),
            "custom-build" => return Some(TargetKind::CustomBuild),
            _ => {}
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct DeclaredDep {
    pub root_ident: String,
    pub package: String,
    pub kind: DepKind,
}

#[derive(Debug, Clone)]
pub struct DeclaredTarget {
    pub name: String,
    pub kind: TargetKind,
    pub src_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PackageTargets {
    pub name: String,
    pub root: PathBuf,
    pub deps: Vec<DeclaredDep>,
    pub targets: Vec<DeclaredTarget>,
    /// Crate name of the package library, e.g. `solo_domain`; `None` for a bin-only
    /// package. Integration tests of a package may name its own library crate.
    pub lib_crate: Option<String>,
}

fn package_names(value: &Value) -> Result<BTreeMap<String, String>, Violation> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for package in value["packages"]
        .as_array()
        .ok_or_else(|| malformed("packages"))?
    {
        map.insert(
            package["id"]
                .as_str()
                .ok_or_else(|| malformed("package id"))?
                .to_owned(),
            package["name"]
                .as_str()
                .ok_or_else(|| malformed("package name"))?
                .to_owned(),
        );
    }
    Ok(map)
}

/// Extracts the workspace packages, their declared dependency roots and targets.
pub fn packages(value: &Value) -> Result<Vec<PackageTargets>, Violation> {
    let id_to_name = package_names(value)?;
    let mut members: BTreeSet<String> = BTreeSet::new();
    for id in value["workspace_members"]
        .as_array()
        .ok_or_else(|| malformed("workspace_members"))?
    {
        let id = id.as_str().ok_or_else(|| malformed("member id"))?;
        let name = id_to_name.get(id).ok_or_else(|| malformed("member name"))?;
        members.insert(name.clone());
    }

    let expected: BTreeSet<String> = WORKSPACE.iter().map(|name| (*name).to_owned()).collect();
    if members != expected {
        return Err(Violation::new(
            Category::GraphPackageSet,
            "graph:workspace",
            format!(
                "workspace members must be exactly {WORKSPACE:?}, found {:?}",
                members.into_iter().collect::<Vec<_>>()
            ),
        ));
    }

    let mut out: Vec<PackageTargets> = Vec::new();
    for package in value["packages"]
        .as_array()
        .ok_or_else(|| malformed("packages"))?
    {
        let name = package["name"]
            .as_str()
            .ok_or_else(|| malformed("package name"))?
            .to_owned();
        if !is_workspace(&name) {
            continue;
        }

        let mut deps: Vec<DeclaredDep> = Vec::new();
        for dependency in package["dependencies"]
            .as_array()
            .ok_or_else(|| malformed("dependencies"))?
        {
            let package_name = dependency["name"]
                .as_str()
                .ok_or_else(|| malformed("dependency name"))?
                .to_owned();
            let root_ident = dependency["rename"]
                .as_str()
                .map_or_else(|| package_name.replace('-', "_"), str::to_owned);
            deps.push(DeclaredDep {
                root_ident,
                package: package_name,
                kind: DepKind::from_metadata(dependency["kind"].as_str()),
            });
        }

        let mut targets: Vec<DeclaredTarget> = Vec::new();
        let mut lib_crate: Option<String> = None;
        for target in package["targets"]
            .as_array()
            .ok_or_else(|| malformed("targets"))?
        {
            let kinds: Vec<String> = target["kind"]
                .as_array()
                .ok_or_else(|| malformed("target kind"))?
                .iter()
                .filter_map(|kind| kind.as_str().map(str::to_owned))
                .collect();
            let kind = target_kind(&kinds).ok_or_else(|| {
                Violation::new(
                    Category::UnsupportedTarget,
                    format!("graph:{name}"),
                    format!("unrecognised target kind {kinds:?}"),
                )
            })?;
            if kind == TargetKind::Lib && lib_crate.is_none() {
                lib_crate = Some(
                    target["name"]
                        .as_str()
                        .ok_or_else(|| malformed("lib target name"))?
                        .replace('-', "_"),
                );
            }
            targets.push(DeclaredTarget {
                name: target["name"]
                    .as_str()
                    .ok_or_else(|| malformed("target name"))?
                    .to_owned(),
                kind,
                src_path: PathBuf::from(
                    target["src_path"]
                        .as_str()
                        .ok_or_else(|| malformed("target src_path"))?,
                ),
            });
        }

        let root = PathBuf::from(
            package["manifest_path"]
                .as_str()
                .ok_or_else(|| malformed("manifest_path"))?,
        )
        .parent()
        .ok_or_else(|| malformed("package root"))?
        .to_path_buf();

        targets.sort_by(|a, b| a.src_path.cmp(&b.src_path));
        deps.sort_by(|a, b| (&a.root_ident, a.kind).cmp(&(&b.root_ident, b.kind)));
        out.push(PackageTargets {
            name,
            root,
            deps,
            targets,
            lib_crate,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// Builds the graph from the resolved node list, keeping renames explicit.
pub fn real_graph(value: &Value) -> Graph {
    let id_to_name = package_names(value).unwrap_or_default();

    let mut nodes: Vec<String> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();

    let Some(resolve) = value["resolve"].as_object() else {
        return Graph { nodes, edges };
    };
    let Some(node_list) = resolve["nodes"].as_array() else {
        return Graph { nodes, edges };
    };

    for node in node_list {
        let Some(from) = id_to_name
            .get(node["id"].as_str().unwrap_or_default())
            .cloned()
        else {
            continue;
        };
        nodes.push(from.clone());

        for dependency in node["deps"].as_array().into_iter().flatten() {
            let Some(to) = id_to_name
                .get(dependency["pkg"].as_str().unwrap_or_default())
                .cloned()
            else {
                continue;
            };
            let alias = dependency["name"].as_str().map(str::to_owned);
            let alias = alias.filter(|name| *name != to.replace('-', "_"));

            let kinds = dependency["dep_kinds"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            if kinds.is_empty() {
                edges.push(Edge {
                    from: from.clone(),
                    to,
                    kind: DepKind::Normal,
                    alias,
                });
                continue;
            }
            for kind in kinds {
                edges.push(Edge {
                    from: from.clone(),
                    to: to.clone(),
                    kind: DepKind::from_metadata(kind["kind"].as_str()),
                    alias: alias.clone(),
                });
            }
        }
    }

    nodes.sort();
    nodes.dedup();
    edges.sort_by(|a, b| {
        (&a.from, &a.to, a.kind, &a.alias).cmp(&(&b.from, &b.to, b.kind, &b.alias))
    });
    Graph { nodes, edges }
}
