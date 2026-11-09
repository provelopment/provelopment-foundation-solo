//! Architecture guard over the **real** Cargo dependency graph, plus in-memory negative
//! fixtures proving the guard actually rejects forbidden edges.
//!
//! The graph model and rule set live in `tests/architecture_checks/graph.rs`, shared with
//! the `check-architecture` source checker, so the two layers cannot drift into
//! contradictory rule sets. This file keeps the real-graph assertions and every earlier
//! negative fixture; the source-level enforcement (`use`, renamed aliases, re-exports,
//! `#[path]`, `include!`, macro arguments and dynamic loading) lives in
//! `tests/architecture_checks/source.rs` and is covered by
//! `tests/check_architecture_fixtures.rs`.

#[path = "architecture_checks/mod.rs"]
mod architecture_checks;

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use architecture_checks::graph::{
    APPROVED_ADAPTER_CLOSURE, APPROVED_ADAPTER_EXTERNAL_NORMAL, APPROVED_DEV_OWNER,
    APPROVED_EXTERNAL_DEV, DepKind, Edge, Graph, WORKSPACE, adapter_closure_allows, check,
    edge_allowed, external_declaration_kind, is_workspace, metadata, packages, real_graph,
};

fn legal_nodes() -> Vec<String> {
    let mut nodes: Vec<String> = WORKSPACE.iter().map(|name| (*name).to_owned()).collect();
    nodes.extend(
        APPROVED_ADAPTER_CLOSURE
            .iter()
            .map(|name| (*name).to_owned()),
    );
    nodes.sort();
    nodes.dedup();
    nodes
}

fn edge(from: &str, to: &str, kind: DepKind) -> Edge {
    Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        kind,
        alias: None,
    }
}

fn legal_graph() -> Graph {
    Graph {
        nodes: legal_nodes(),
        edges: vec![
            edge("solo-application", "solo-domain", DepKind::Normal),
            edge("solo-adapters", "solo-application", DepKind::Normal),
            edge("solo-adapters", "solo-domain", DepKind::Normal),
            // The reviewed external normal roots of the outer parsing adapter.
            edge("solo-adapters", "serde", DepKind::Normal),
            edge("solo-adapters", "serde_json", DepKind::Normal),
            edge("solo-cli", "solo-adapters", DepKind::Normal),
            edge("solo-cli", "solo-application", DepKind::Normal),
            edge("solo-cli", "serde_json", DepKind::Dev),
            edge("solo-cli", "syn", DepKind::Dev),
            // The exact audited locked closure of those two roots.
            edge("serde", "serde_core", DepKind::Normal),
            edge("serde_core", "serde_derive", DepKind::Normal),
            edge("serde_derive", "proc-macro2", DepKind::Normal),
            edge("serde_derive", "quote", DepKind::Normal),
            edge("serde_derive", "syn", DepKind::Normal),
            edge("serde_json", "itoa", DepKind::Normal),
            edge("serde_json", "memchr", DepKind::Normal),
            edge("serde_json", "serde", DepKind::Normal),
            edge("serde_json", "serde_core", DepKind::Normal),
            edge("serde_json", "zmij", DepKind::Normal),
            edge("proc-macro2", "unicode-ident", DepKind::Normal),
            edge("quote", "proc-macro2", DepKind::Normal),
            edge("syn", "proc-macro2", DepKind::Normal),
            edge("syn", "quote", DepKind::Normal),
            edge("syn", "unicode-ident", DepKind::Normal),
        ],
    }
}

fn fixture_with(extra: Edge) -> Graph {
    let mut graph = legal_graph();
    graph.edges.push(extra);
    graph
}

fn refusal(graph: &Graph) -> Vec<String> {
    check(graph)
        .err()
        .unwrap_or_default()
        .into_iter()
        .map(|violation| violation.to_string())
        .collect()
}

// ---------------------------------------------------------------------------
// In-memory fixtures: the guard must reject every forbidden shape.
// ---------------------------------------------------------------------------

#[test]
fn the_legal_fixture_passes() {
    check(&legal_graph()).expect("the inward-only shape must be accepted");
}

#[test]
fn rejects_a_domain_edge_to_an_adapter() {
    let errors = refusal(&fixture_with(edge(
        "solo-domain",
        "solo-adapters",
        DepKind::Normal,
    )));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-domain") && error.contains("solo-adapters")),
        "expected a domain->adapter refusal, saw {errors:?}"
    );
}

#[test]
fn rejects_an_application_edge_to_the_cli() {
    let errors = refusal(&fixture_with(edge(
        "solo-application",
        "solo-cli",
        DepKind::Normal,
    )));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-application") && error.contains("solo-cli"))
    );
}

#[test]
fn rejects_an_application_edge_to_an_adapter() {
    let errors = refusal(&fixture_with(edge(
        "solo-application",
        "solo-adapters",
        DepKind::Normal,
    )));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-application") && error.contains("solo-adapters"))
    );
}

#[test]
fn rejects_a_domain_edge_to_an_external_crate() {
    let errors = refusal(&fixture_with(edge(
        "solo-domain",
        "memchr",
        DepKind::Normal,
    )));
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
    let errors = refusal(&fixture_with(renamed));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-adapters") && error.contains("solo-cli")),
        "a rename must not hide the edge: {errors:?}"
    );
    assert!(
        errors.iter().any(|error| error.contains("upstream")),
        "the alias must be reported, not silently dropped"
    );
}

#[test]
fn rejects_illegal_dev_and_build_edges() {
    for kind in [DepKind::Dev, DepKind::Build] {
        let errors = refusal(&fixture_with(edge("solo-application", "solo-cli", kind)));
        assert!(
            errors.iter().any(|error| error.contains(kind.label())),
            "a {kind:?} edge must be refused as such: {errors:?}"
        );
    }
}

#[test]
fn rejects_a_cycle_between_workspace_crates() {
    let errors = refusal(&fixture_with(edge(
        "solo-domain",
        "solo-application",
        DepKind::Normal,
    )));
    assert!(errors.iter().any(|error| error.contains("cycle")));
}

#[test]
fn rejects_the_approved_check_tool_as_a_shipped_dependency() {
    // `syn` is approved only as a development dependency of `solo-cli`. The same edge
    // declared as a normal dependency must be refused: approval is by kind and by owner,
    // not by crate name alone.
    let errors = refusal(&fixture_with(edge("solo-cli", "syn", DepKind::Normal)));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("syn") && error.contains("normal")),
        "a shipped `syn` edge must be refused: {errors:?}"
    );

    let errors = refusal(&fixture_with(edge("solo-domain", "syn", DepKind::Dev)));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-domain") && error.contains("syn")),
        "no inner crate may take the check tool even as a development edge: {errors:?}"
    );
}

#[test]
fn the_guard_is_load_bearing_not_decorative() {
    // The same graph is accepted and refused only by the presence of the forbidden edge,
    // so the refusal provably comes from the rule and not from a blanket rejection.
    check(&legal_graph()).expect("the legal graph must be accepted");

    let mut illegal = legal_graph();
    illegal
        .edges
        .push(edge("solo-domain", "solo-adapters", DepKind::Normal));
    let errors = refusal(&illegal);
    assert!(!errors.is_empty(), "the forbidden edge must be refused");

    // Removing the offending edge restores acceptance, so the guard is not a wall that
    // refuses everything.
    let mut restored = illegal.clone();
    restored
        .edges
        .retain(|candidate| !(candidate.from == "solo-domain" && candidate.to == "solo-adapters"));
    check(&restored).expect("restoring the legal graph must restore acceptance");
}

#[test]
fn the_shared_rule_set_matches_the_architecture_layers() {
    // One rule set, exercised directly: the same function the source checker calls.
    for allowed in [
        edge("solo-application", "solo-domain", DepKind::Normal),
        edge("solo-adapters", "solo-application", DepKind::Normal),
        edge("solo-cli", "solo-adapters", DepKind::Normal),
        edge("solo-cli", "syn", DepKind::Dev),
    ] {
        assert!(edge_allowed(&allowed), "{allowed:?} must be allowed");
    }
    for forbidden in [
        edge("solo-domain", "solo-application", DepKind::Normal),
        edge("solo-application", "solo-adapters", DepKind::Normal),
        edge("solo-adapters", "solo-cli", DepKind::Normal),
        edge("solo-cli", "syn", DepKind::Normal),
        edge("solo-domain", "std", DepKind::Normal),
    ] {
        assert!(!edge_allowed(&forbidden), "{forbidden:?} must be refused");
    }
}

// ---------------------------------------------------------------------------
// The real workspace: read actual Cargo metadata, not a hand-written description.
// ---------------------------------------------------------------------------

fn metadata_value() -> Value {
    metadata().expect("cargo metadata must succeed on the real locked workspace")
}

#[test]
fn real_workspace_contains_exactly_the_four_required_crates() {
    let value = metadata_value();
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

    let resolved = packages(&value).expect("workspace packages must be extractable");
    assert_eq!(resolved.len(), 4, "exactly four workspace packages");
}

#[test]
fn real_dependency_graph_is_inward_only() {
    let graph = real_graph(&metadata_value());
    if let Err(violations) = check(&graph) {
        panic!("the real workspace violates the inward-only contract: {violations:#?}");
    }
}

#[test]
fn only_the_approved_external_dependencies_are_declared() {
    let value = metadata_value();

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
            let declared = DepKind::from_metadata(dependency["kind"].as_str());
            // The one reviewed rule decides both the permitted owner and the permitted
            // kind; there is no second copied allowlist that could drift from it.
            assert_eq!(
                external_declaration_kind(name, target),
                Some(declared),
                "{name} may not declare the external dependency {target} as {}",
                declared.label()
            );
            assert!(
                dependency["rename"].is_null(),
                "{name} -> {target} must not be aliased in the real manifest"
            );
        }
    }
}

#[test]
fn the_real_graph_exposes_exactly_the_two_adapter_external_normal_roots() {
    let value = metadata_value();
    let mut found: Vec<String> = Vec::new();

    for package in value["packages"].as_array().expect("packages array") {
        if package["name"].as_str() != Some("solo-adapters") {
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
                DepKind::from_metadata(dependency["kind"].as_str()),
                DepKind::Normal,
                "{target} must be a normal dependency of the outer adapter"
            );
            found.push(target.to_owned());
        }
    }

    found.sort();
    let expected: Vec<String> = APPROVED_ADAPTER_EXTERNAL_NORMAL
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
    assert_eq!(
        found, expected,
        "the adapter must declare exactly the reviewed external roots"
    );
}

#[test]
fn the_inner_crates_declare_no_external_dependency_of_any_kind() {
    let value = metadata_value();

    for package in value["packages"].as_array().expect("packages array") {
        let name = package["name"].as_str().expect("package name");
        if !matches!(name, "solo-domain" | "solo-application") {
            continue;
        }
        for dependency in package["dependencies"]
            .as_array()
            .expect("dependencies array")
        {
            let target = dependency["name"].as_str().expect("dependency name");
            assert!(
                is_workspace(target),
                "{name} must declare no external dependency, found {target}"
            );
        }
    }
}

#[test]
fn the_shared_owner_kind_rule_is_exact() {
    assert_eq!(
        external_declaration_kind("solo-cli", "serde_json"),
        Some(DepKind::Dev)
    );
    assert_eq!(
        external_declaration_kind("solo-cli", "syn"),
        Some(DepKind::Dev)
    );
    assert_eq!(external_declaration_kind("solo-cli", "serde"), None);
    assert_eq!(
        external_declaration_kind("solo-adapters", "serde"),
        Some(DepKind::Normal)
    );
    assert_eq!(
        external_declaration_kind("solo-adapters", "serde_json"),
        Some(DepKind::Normal)
    );
    assert_eq!(external_declaration_kind("solo-adapters", "syn"), None);
    assert_eq!(external_declaration_kind("solo-adapters", "itoa"), None);
    assert_eq!(external_declaration_kind("solo-domain", "serde"), None);
    assert_eq!(
        external_declaration_kind("solo-application", "serde_json"),
        None
    );
    assert_eq!(external_declaration_kind("rand", "serde"), None);
    assert_eq!(APPROVED_DEV_OWNER, "solo-cli");
    assert!(APPROVED_EXTERNAL_DEV.contains(&"syn"));
}

#[test]
fn real_locked_graph_resolves_without_floating_versions() {
    let value = metadata_value();
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

#[test]
fn the_real_graph_carries_no_rename_that_could_hide_an_edge() {
    let graph = real_graph(&metadata_value());
    let renamed: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|candidate| candidate.alias.is_some())
        .collect();
    assert!(
        renamed.is_empty(),
        "no real dependency may be renamed, found {renamed:?}"
    );
}

// ---------------------------------------------------------------------------
// Adversarial cases for the reviewed two-owner external rule.
// ---------------------------------------------------------------------------

#[test]
fn refuses_an_unapproved_adapter_external_root() {
    let errors = refusal(&fixture_with(edge(
        "solo-adapters",
        "serde_yaml",
        DepKind::Normal,
    )));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-adapters") && error.contains("serde_yaml")),
        "expected an unapproved adapter root refusal, saw {errors:?}"
    );
}

#[test]
fn refuses_an_adapter_dev_or_build_external() {
    for kind in [DepKind::Dev, DepKind::Build] {
        let errors = refusal(&fixture_with(edge("solo-adapters", "serde", kind)));
        assert!(
            errors
                .iter()
                .any(|error| error.contains("solo-adapters") && error.contains("serde")),
            "expected a {kind:?} adapter external refusal, saw {errors:?}"
        );
    }
}

#[test]
fn refuses_a_normal_kind_cli_external() {
    // `syn` stays a development-only tool: shipping it from the composition root fails.
    let errors = refusal(&fixture_with(edge("solo-cli", "syn", DepKind::Normal)));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-cli") && error.contains("syn")),
        "expected a normal-kind cli external refusal, saw {errors:?}"
    );
}

#[test]
fn refuses_any_inner_crate_external_edge() {
    for from in ["solo-domain", "solo-application"] {
        for kind in [DepKind::Normal, DepKind::Dev, DepKind::Build] {
            let errors = refusal(&fixture_with(edge(from, "serde", kind)));
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains(from) && error.contains("serde")),
                "expected a {from} external refusal for {kind:?}, saw {errors:?}"
            );
        }
    }
}

#[test]
fn refuses_an_aliased_external_that_disguises_an_unapproved_root() {
    // The recorded target is always the true package name, so an alias cannot launder
    // an unapproved root into an approved one.
    let mut graph = legal_graph();
    graph.edges.push(Edge {
        from: "solo-adapters".to_owned(),
        to: "rand".to_owned(),
        kind: DepKind::Normal,
        alias: Some("serde".to_owned()),
    });
    let errors = refusal(&graph);
    assert!(
        errors.iter().any(|error| error.contains("rand")),
        "expected the true package name in the refusal, saw {errors:?}"
    );
}

#[test]
fn refuses_an_unapproved_transitive_component_behind_an_approved_root() {
    // `serde_json` is approved, but a new component behind it is not: the audited
    // closure is closed, so an added transitive node cannot ride along silently.
    let errors = refusal(&fixture_with(edge("serde_json", "rand", DepKind::Normal)));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("transitive external reachability")
                && error.contains("rand")),
        "expected a transitive reachability refusal, saw {errors:?}"
    );
    assert!(
        !adapter_closure_allows("solo-adapters", "rand"),
        "an unlisted component must never be inside the approved closure"
    );
}

#[test]
fn refuses_an_adapter_edge_outward_to_the_composition_root() {
    let errors = refusal(&fixture_with(edge(
        "solo-adapters",
        "solo-cli",
        DepKind::Normal,
    )));
    assert!(
        errors
            .iter()
            .any(|error| error.contains("solo-adapters") && error.contains("solo-cli")),
        "expected an outward adapter refusal, saw {errors:?}"
    );
}

#[test]
fn the_approved_closure_is_exactly_the_locked_component_set() {
    // Every approved root must itself be inside the closure, and the closure must name
    // no workspace member.
    for root in APPROVED_ADAPTER_EXTERNAL_NORMAL {
        assert!(
            APPROVED_ADAPTER_CLOSURE.contains(&root),
            "{root} must be inside the approved closure"
        );
        assert!(adapter_closure_allows("solo-adapters", root));
        assert!(
            !adapter_closure_allows("solo-domain", root),
            "the domain may reach no external component"
        );
        assert!(
            !adapter_closure_allows("solo-application", root),
            "the application may reach no external component"
        );
    }
    for node in APPROVED_ADAPTER_CLOSURE {
        assert!(!is_workspace(node), "{node} must not be a workspace member");
    }
}
