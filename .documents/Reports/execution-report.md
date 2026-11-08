# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08 |
| Work order | **`SOLO-WO-045 R01`** — R02.B first bounded slice (strict manifest declarations, source-locator rules) |
| Owner handoff | Received, verbatim `SOLO-WO-045 R01` — matched the order's §7 text exactly |
| Required starting HEAD / tree | `47b9b0796c2266a6b429822b7e8ebdeefc7f9450` / `1b9b21c8713ca58318584f13dbba18864b9ffc4f` — **both matched exactly** |
| §5.1 issuance revision | `211378a4a9f78c074c8099076f0055bff6b80515` / tree `9b1733bc31bad6868583d7089023a8d50504ca9e` |
| §5.1 NEXT revision | `5184cd90578796e4a8809ee0c40a49394f64bae8` / tree `ea210988c1e555010f2fbe0ade1c7406561d0dbb` |
| Final revision of this run | given in the chat handback after publication, never embedded here |
| **Status** | **`STOP`** — §5.1 complete and green; §5.2 **not started**; a hard scope contradiction blocks the approved slice |

## Outcome first

**I stopped before writing any implementation code, and I am not claiming any part of R02.B.** The approved slice cannot reach a green mandatory gate without editing a file that the same order declares read-only. Under §6 of this order that is a mandatory STOP — *"needed path outside §1"* and *"mandatory CI/red checker"* — and under §3E, *"If this cannot be proved under the exact locked graph, STOP instead of loosening the checker."*

To be unambiguous about what this run did **not** do: no `source_locator.rs` was created, no `publication_manifest.rs` was created, `crates/solo-adapters/Cargo.toml` and `Cargo.lock` are **byte-identical** to the baseline, no architecture-checker file was touched, no test was weakened, no fixture was removed, and **no red CI run was created**. The tree was left in the state the order prescribes for a non-accepted outcome: full ACTIVE installed, NEXT neutral, ROADMAP untouched.

## The blocker, with first-hand evidence

§1 and §3E require promoting `serde = "=1.0.229"` and `serde_json = "=1.0.154"` from the locked graph into **normal** dependencies of `solo-adapters`. One existing test forbids exactly that, unconditionally, over the **real** metadata, and its file is not in the §1 ledger.

`crates/solo-cli/tests/workspace_graph.rs`, function `only_the_approved_external_dependencies_are_declared`, iterates the real `cargo metadata` for every workspace package and asserts, for **every** non-workspace dependency:

| Line | Assertion | Effect of the required promotion |
| --- | --- | --- |
| 311–314 | `assert_eq!(name, APPROVED_DEV_OWNER, "only {APPROVED_DEV_OWNER} may declare an external dependency, found {name} -> {target}")` | **Fails.** `APPROVED_DEV_OWNER` is the literal `"solo-cli"`; the new edge is declared by `solo-adapters` |
| 315–319 | `assert_eq!(dependency["kind"].as_str(), Some("dev"), "{name} -> {target} must be a development dependency, never shipped")` | **Fails.** A `[dependencies]` entry reports `kind: null`, never `"dev"` |
| 320–323 | `assert!(APPROVED_EXTERNAL_DEV.contains(&target))` | `APPROVED_EXTERNAL_DEV` is `["serde_json", "syn"]`, so `serde_json` would pass and `serde` would fail |
| 324–327 | `assert!(dependency["rename"].is_null())` | Satisfied — no alias is declared |

The test is unconditional (`#[test]` with no `cfg`, no `#[ignore]`, no feature gate) and runs inside the mandatory step `cargo test --workspace --all-targets --locked` (`.github/workflows/rust-verify.yml` line 552), on both `ubuntu-24.04` and `windows-2025`. It is the **only** test in the repository asserting this invariant, so there is no second place the rule could be satisfied instead.

**Why no compliant workaround exists.** Declaring the parsers as `dev`-dependencies contradicts §1 ("as normal dependencies of the outer adapter") and §3A/§3E, which need `serde`'s visitor and `serde_json`'s value model in shipped adapter code. Hosting the parser in `solo-cli` contradicts the fixed path `crates/solo-adapters/src/publication_manifest.rs`. Hand-writing a duplicate-key-aware JSON parser instead of `serde` contradicts §3A's explicit *"Use `serde`'s visitor/map-access path so duplicate keys are refused **before** a `serde_json::Value`…"*. Editing the test is the only remaining route, and `crates/solo-cli/tests/workspace_graph.rs` — blob `8fa4b4d0bde840782abb0c9e24c84bac8f06c947`, unmodified (`git status` clean for that path) — is covered twice by the prohibition: it is absent from the §1 ledger, and §1 states *"**Everything else is read-only**… all other Rust source/tests/manifests"*.

I did not create a demonstration red run to prove this. The order forbids a gated failure being produced to force a green badge, and the assertion above is decidable from the committed source at the exact revision; an intentional failing push would have added red CI evidence without adding information.

## What §5.1 completed, verified on both hosts

The issuance work that the order permits independently of the blocker was completed first, exactly as §5.1 specifies, and both revisions are green.

| Commit | Content | Role | CI (Ubuntu / Windows) |
| --- | --- | --- | --- |
| `211378a` | `workorder-active.md` recorded byte-for-byte as handed off, blob `a75e3e350685bfb54d2215d0e0fdf98c3f64ee88`; nothing else staged | issuance history | [run `42503168016`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503168016) · jobs [`128800082733`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503168016/job/128800082733) / [`128800082847`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503168016/job/128800082847) — 22 ok / 9 skipped / 0 failed and 19 ok / 12 skipped / 0 failed |
| `5184cd9` | `plan-next.md` cleared to its retained one-line title plus LF, blob `92fa562b1af0c54391851aa5c79deb1631127069` (49 bytes), verified byte-exact before commit | NEXT neutral while its candidate is issued | [run `42503190317`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503190317) · jobs [`128800146540`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503190317/job/128800146540) / [`128800146583`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42503190317/job/128800146583) — same counts, **success** on both hosts |

`plan-roadmap.md` remains byte-identical at `18975332c2820170a32558c4ee224176667b91ab` — the `R02.B` and `R02.C` bullets, the Stage 02 heading and exit gate and all twelve remaining stages are untouched. **Stage 02 and R02.B remain open.**

## Preflight evidence for this order

All nineteen §2 baseline blobs matched at the starting HEAD, so no drift existed before I acted: `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`, `execution-report.md`, `ARCHITECTURE.md`, `solo-domain/src/lib.rs`, `solo-domain/src/efs_ownership.rs`, `solo-adapters/Cargo.toml`, `solo-adapters/src/lib.rs`, `architecture_checks/graph.rs`, `architecture_checks/source.rs`, `architecture_checks/mod.rs`, root `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `rust-verify.yml`, `sop-architect.md`, `sop-implementation.md` — `mismatches=0`. All four required absences held (`source_locator.rs` source and test, `publication_manifest.rs` source and test). `workorder-active.md` was the sole working-tree change, at its neutral baseline blob `ea5f20b7…`, i.e. exactly the permitted Owner installation. Root, branch `main`, single `origin` at `provelopment/provelopment-foundation-solo`, one worktree, zero stash, no second checkout.

## Verification boundary — what is and is not proven

**Proven first-hand this run:** the file/remote/worktree/stash state; every baseline blob; the required absences; the byte-exact content of both issuance commits; the `workspace_graph.rs` code, its line numbers, its baseline blob and its clean working-tree status; the absence of any `cfg`, `#[ignore]` or feature gate on that test; the workflow line that runs it; and both-host CI on the two §5.1 revisions.

**Not proven, and not claimed:** no Rust toolchain is installed on the Owner host in this run (the previous run's disposable `.temp` tree was removed and no new one was acquired, because the blocker was established before any build was warranted), so no local `cargo` command was executed and **no local test result is claimed**. There is therefore no local `cargo test` pass, no local architecture-checker run and no local lock-graph inspection in this report. The blocker argument is a source-level proof over the committed blobs, not an executed experiment; the Chief Architect can confirm it at the exact revision by reading the four assertions, and would see the same failure in the first hosted run of any promotion commit.

**Unchanged, therefore not re-audited:** the 15-package locked graph, the four-category `cargo-deny` gate, the RustSec digest, OSV/crates.io currency data and the disposable advisory probe. Because `Cargo.toml` and `Cargo.lock` are byte-identical to the baseline, the previously accepted dependency and security posture carries forward **unchanged by this run** — it was not re-measured here and must not be read as freshly verified. The `unicode-ident` Unicode-3.0 attribution work remains an open future-release obligation and is not waived. GitHub alert endpoints returning 401 remain `UNVERIFIED`; no new alert state was observed.

## Run history and cleanup

This is the second consecutive run in which the Owner host held no linker, and the first in which a scope contradiction — not a tooling limit — stopped the work. The prior accepted run's record (three red revisions, the temporary diagnostic commit, the two malformed commit subjects and their disclosure) remains in Git history and in the previous revision of this report.

Cleanup: the temporary commit-message files written under the OS temp area for this run were deleted; no `.temp/` toolchain or scratch directory was created; no `target/` directory exists in the worktree; no new WIP file, report copy, fixture directory or archive was added. No secrets, credentials or personal data were read or written. No PR, merge, branch, stash, force-push, rebase, amend, tag, release, workflow dispatch or settings change occurred, and no other repository was touched.


## The smallest Owner decision this needs

The scope as written cannot be executed green. Two bounded resolutions exist, and choosing between them is an Owner decision, not mine:

1. **Amend the §1 ledger to include `crates/solo-cli/tests/workspace_graph.rs`** with an exactly specified change: replace the blanket *"only `solo-cli` may declare an external dependency"* invariant with the reviewed adapter allowlist (`serde`, `serde_json` as normal adapter roots, with the existing `solo-cli` dev-only assertions retained verbatim), and add negative cases proving `solo-domain` and `solo-application` still cannot declare or reach any external package, that an unlisted adapter external is still refused, and that an aliased or renamed declaration is still refused. The 23 mandatory refusal fixtures and the workflow's `if ($refused -ne 23)` check must stay exactly as they are.
2. **Split the order**: deliver the dependency-free addition first (the pure lexical `source_locator.rs` in `solo-domain` with its tests — it needs no external crate at all), and defer the manifest parser with its `serde`/`serde_json` promotion to a later order issued after decision 1 is settled.

Either way, a further order amendment or a new order is required before implementation resumes. Issuance §5.1 stands complete on both hosts and needs no repetition.

## Conclusion

**`SOLO-WO-045 R01` is stopped at §5.2, before implementation, for a scope contradiction that cannot be resolved inside the permitted write set.** ACTIVE remains fully installed, NEXT is neutral, ROADMAP is byte-identical with `R02.B` and `R02.C` still open, and Stage 02 remains open. No implementation of the approved slice exists, and none is implied by this report.

The order instructs, on a STOP, to *"preserve ROADMAP R02.B, the issued ACTIVE and a factual report; do not erase failure evidence or self-authorize a revision."* That is exactly the state recorded here. Resuming requires the Owner decision above plus a matching handoff; the Chief Architect's disposition on this revision is `ACCEPT` of the stop and its evidence, `REVISE` with amended scope, or `STOP`.

**`agent report ready`**
