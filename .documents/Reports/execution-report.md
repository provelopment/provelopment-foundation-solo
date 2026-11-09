# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08 |
| Work order | **`SOLO-WO-045 R01`** — R02.B first bounded slice |
| Owner handoff | Received, verbatim `SOLO-WO-045 R01` — **second,** identical handoff against an **unchanged** order |
| Starting HEAD / tree | `5fdbb9c855361aa0ec4bb4447e65852d6d7bd256` / `0d0f8d4dcd8910706fbdd15f794265e258e3033f` (clean, remote-equal) |
| Issuance (previous run, stands) | `211378a4a9f78c074c8099076f0055bff6b80515` / tree `9b1733bc31bad6868583d7089023a8d50504ca9e` |
| NEXT neutralisation (previous run, stands) | `5184cd90578796e4a8809ee0c40a49394f64bae8` |
| Final revision of this run | given in the chat handback after publication, never embedded here |
| **Status** | **`STOP` unchanged** — authorization is byte-identical to the run that stopped; no scope amendment was issued |

## Outcome

**I did not implement anything, again, and the reason is not a lack of effort or a tooling limit: the authorization is materially unchanged.** `workorder-active.md` is byte-identical to the order I stopped against — blob `a75e3e350685bfb54d2215d0e0fdf98c3f64ee88`, 26,472 bytes, `OWNER-ISSUED`, same `SOLO-WO-045 R01`, same frozen §1 ledger, same §3E wording. A repeated handoff against an unchanged order restates the same authority; it does not add, widen or waive a permission, and it does not convert an impossible scope into a possible one.

So this run did the only useful thing available: it re-verified that the blocker is still exactly true at this exact revision, and it did the deeper work of **closing every remaining escape hatch** that could have let §1 be satisfied while the mandatory gate stayed green. That analysis is the substance of this report — it is what makes the decision below final rather than provisional.

No source file was created or modified. `crates/solo-adapters/Cargo.toml` still declares exactly `solo-application` and `solo-domain`; `Cargo.lock`, `Cargo.toml`, every architecture-checker file, the 23 fixtures and `rust-verify.yml` are byte-identical. **No red CI run was created.** ACTIVE remains fully installed, NEXT remains neutral, ROADMAP remains byte-identical with `R02.B` and `R02.C` open.

## The blocker, restated exactly

§1 and §3E require `serde = "=1.0.229"` and `serde_json = "=1.0.154"` as **normal** dependencies of `solo-adapters`. `crates/solo-cli/tests/workspace_graph.rs` (blob `8fa4b4d0bde840782abb0c9e24c84bac8f06c947`, unmodified) contains `only_the_approved_external_dependencies_are_declared`, an unconditional `#[test]` — no `cfg`, no `#[ignore]`, no feature gate — which walks the **real** `cargo metadata` and, for every non-workspace dependency of every workspace package, asserts:

| Line | Assertion | Status if §1 is implemented |
| --- | --- | --- |
| 311–314 | `assert_eq!(name, APPROVED_DEV_OWNER, "only {APPROVED_DEV_OWNER} may declare an external dependency…")` — `APPROVED_DEV_OWNER` is the literal `"solo-cli"` | **Fails**: the edge is declared by `solo-adapters` |
| 315–319 | `assert_eq!(dependency["kind"].as_str(), Some("dev"), "…must be a development dependency, never shipped")` | **Fails**: a `[dependencies]` entry reports `kind: null` |
| 320–323 | `assert!(APPROVED_EXTERNAL_DEV.contains(&target))` — the list is `["serde_json", "syn"]` | `serde_json` passes, `serde` **fails** |
| 324–327 | `assert!(dependency["rename"].is_null())` | Satisfied — no alias declared |

It runs inside the mandatory `cargo test --workspace --all-targets --locked` (`.github/workflows/rust-verify.yml` line 552) on both hosts, and it is the only place this invariant exists. Its file is absent from the §1 ledger, and §1 states *"**Everything else is read-only**… all other Rust source/tests/manifests"*.

## Every escape hatch, closed

Because the same instruction arrived twice, the decisive question is not "did you try?" but "is there any conforming way?" Each candidate mechanism was tested against the actual assertion semantics above. None survives.

| Candidate mechanism | Why it cannot satisfy §1 while the gate stays green |
| --- | --- |
| Declare the dependency as `optional = true` and enable it by feature | Cargo lists optional dependencies in `packages[].dependencies` (`optional: true`, `kind: null`), so 311–314 and 315–319 both still fail. The parser would also be absent from the default build. |
| Declare it only under `[target.'cfg(…)'.dependencies]` | Target-specific entries are still listed, with a `target` field and `kind: null`. Same two failures. |
| Declare the version once in root `[workspace.dependencies]` and inherit it | Inheritance supplies a version constraint only. The member still declares a real, normal dependency edge, which is exactly what the assertion inspects. |
| Vendor `serde` locally or pin it by `path`/`[patch]` | The dependency is still a non-workspace package named `serde`, so the assertion still applies and still fails; it would additionally introduce a non-registry component that §1 and §4 forbid outright. |
| Rename or alias the dependency | `packages[].dependencies[].name` carries the **real** package name, and 324–327 independently refuse a rename. |
| Declare it as a normal dependency of `solo-cli` instead | 315–319 requires `solo-cli`'s externals to be `dev`-kind, so that fails too — and `solo-adapters` could not use `solo-cli` (an outward edge the checker already refuses). |
| Declare it as an adapter **dev**-dependency | Not available to the shipped library target, and §1 says "normal dependencies of the outer adapter". §3A/§3E need `serde`'s visitor and `serde_json`'s value model in shipped code. |
| Relax only `architecture_checks/graph.rs`, as §3E permits | **Insufficient by construction.** The failing assertions hardcode `name == APPROVED_DEV_OWNER` and `kind == "dev"`; they never consult the checker's policy. No constant, allowlist or helper added to `graph.rs` can change their verdict. The decisive consequence: §3E's single permitted edit cannot resolve the conflict. |
| Host the parser in `solo-cli` instead of the adapter | Contradicts the fixed path `crates/solo-adapters/src/publication_manifest.rs` in §1. |
| Hand-write a duplicate-key-aware parser with no `serde` | Contradicts §3A's explicit requirement to *"Use `serde`'s visitor/map-access path so duplicate keys are refused **before** a `serde_json::Value`…"*. |
| Skip, ignore, delete or rewrite the test | The file is read-only under §1, and weakening a refusal gate is forbidden by §6 in the strongest terms. |

The conclusion is therefore forced, not chosen: **satisfying §1 inevitably fails a test in an unlisted, read-only file, and no order-compliant adjustment to the permitted paths can prevent it.** That is the order's own STOP condition — *"needed path outside §1"* and *"mandatory CI/red checker"* — reinforced by §3E's *"STOP instead of loosening the checker"*.

## The decision, with ready-to-approve wording

Two bounded routes exist. I have drafted both; I have installed **neither**, because choosing between them is Owner authority and self-installing an amendment would be self-authorization.

**Route 1 — amend the §1 ledger.** Add one row to the §1 table:

> **`crates/solo-cli/tests/workspace_graph.rs`** — **Narrow invariant replacement only.** Replace the blanket *"only `solo-cli` may declare an external dependency"* assertion with a reviewed two-owner table: `solo-cli` may declare only `dev`-kind externals from `APPROVED_EXTERNAL_DEV`, and `solo-adapters` may declare only `normal`-kind externals from a new `APPROVED_ADAPTER_EXTERNAL` = `["serde", "serde_json"]`. Retain the no-rename assertion for both owners. Add negative cases proving `solo-domain` and `solo-application` may declare neither kind, an unlisted adapter external is refused, a `dev`-kind adapter external is refused, and a renamed declaration is refused. Preserve the 23 mandatory refusal fixtures and the workflow's `if ($refused -ne 23)` check unchanged.

Paired with the corresponding closed allowlist in `architecture_checks/graph.rs` (already permitted by §1), this keeps a single source of truth and makes both the direct-edge rule and the transitive-closure rule explicit rather than implicit.

**Route 2 — split the order.** Deliver the dependency-free half first: `crates/solo-domain/src/source_locator.rs` plus `crates/solo-domain/tests/source_locator.rs`, needing no external crate at all, therefore touching nothing this blocker covers. Defer `publication_manifest.rs` and the `serde`/`serde_json` promotion to a new order issued after Route 1 is settled.

Either route needs a further Owner-issued amendment or a new order plus a matching handoff before implementation resumes. Nothing in this report authorizes either.


## State ledger and honest limitations

**This run changed exactly one file:** `.documents/Reports/execution-report.md`. Verified unchanged against the starting revision: `workorder-active.md` (`a75e3e35…`, order still installed), `plan-next.md` (`92fa562b…`, neutral, 49 bytes), `plan-roadmap.md` (`18975332…`, `R02.B` and `R02.C` intact), `plan-outlook.md` (`cb9c7a61…`), `crates/solo-adapters/Cargo.toml`, `Cargo.lock`, root `Cargo.toml`, `workspace_graph.rs` (`8fa4b4d0…`), `architecture_checks/graph.rs`, `source.rs`, `mod.rs`, `solo-domain/src/lib.rs`, `rust-verify.yml` and `ARCHITECTURE.md`. Repository: one worktree, zero stash, sole `origin` at `provelopment/provelopment-foundation-solo`, clean tree, local `main` equal to `origin/main`.

**Not proven, and not claimed:** no Rust toolchain is installed on the Owner host in this run, so no `cargo` command was executed and **no local test, clippy, format or checker result is claimed**. The blocker argument is a source-level proof over committed blobs plus documented `cargo metadata` semantics, not an executed experiment. Because `Cargo.toml` and `Cargo.lock` are untouched, the previously accepted dependency and security posture — the 15-package locked graph, the four-category `cargo-deny` gate, the RustSec digest, OSV/crates.io currency data and the disposable advisory probe — carries forward **unchanged by this run** and was not re-measured; it must not be read as freshly verified. GitHub alert endpoints previously returning 401 remain `UNVERIFIED`. The `unicode-ident` Unicode-3.0 attribution obligation remains open and is not waived.

**Cleanup:** no `.temp/` toolchain or scratch directory was created or removed; no `target/` directory exists; no WIP file, report copy, fixture directory or archive was added; no OS-temp scratch remains from this run. No PR, merge, branch, stash, force-push, rebase, amend, tag, release, workflow dispatch, secret access or settings change occurred, and no other repository was touched.

## Conclusion

**`SOLO-WO-045 R01` remains stopped at §5.2, before implementation, for a scope contradiction that cannot be resolved inside the permitted write set.** The repeated handoff restated the same authority and changed nothing; the blocker was re-verified and every escape hatch is now closed in writing. Stage 02 and `R02.B`/`R02.C` remain open, ACTIVE remains installed, NEXT remains neutral, and no implementation of the approved slice exists.

The order instructs, on a STOP, to *"preserve ROADMAP R02.B, the issued ACTIVE and a factual report; do not erase failure evidence or self-authorize a revision."* That is precisely the recorded state. Resuming requires Route 1 or Route 2 above, authorized by the Owner with a matching handoff. The Chief Architect's disposition on this revision is `ACCEPT` of the stop and its evidence, `REVISE` with amended scope, or `STOP`.

**`agent report ready`**
