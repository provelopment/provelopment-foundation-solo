# Foundation Solo — Active Work Order

## SOLO-WO-030 R01 — Stage 01 R01.B Hexagonal Source-Boundary Enforcement

## Approved Scope — OWNER-ISSUED

### 1. Objective and reviewed starting state

Implement the **R01.B** package currently selected in `.documents/WIP/plan-next.md`: a fail-closed, executable Rust source-boundary checker **in addition to** the existing real Cargo metadata graph guard. The checker must prove that the existing four-crate Hexagonal dependency rules cannot be defeated through import aliases, re-exports, qualified paths, module/include indirection, macro-produced imports or unsupported dynamic loading. Preserve the independently accepted `R01.A` Rust workspace, `BootstrapPreflightV1`, real CLI adapter, sole production composition root, verified Entity ID and narrow base-candidate preflight. No decorative port, new product capability or second production runtime may appear.

This is **R01.B only**, not the entire Stage 01 exit. Do not execute `R01.C` (CI, HTTP/browser/production smoke), Stage 02 onward, publication, deployment, provider setup, release or merge. `R01.B` remains in ROADMAP until a later independent Architect acceptance and separate Owner-controlled closure.

| Reviewed identity | Value |
| --- | --- |
| Repository | `provelopment/provelopment-foundation-solo` — existing local checkout, branch `main`, sole `origin` |
| Starting HEAD | `88333c4030dc906e98b9c52c784b799ff2c51928` |
| Starting tree | `4cdd1755a4c1a52310abef9d93e312b992aa6144` |
| Existing ACTIVE | `.documents/WIP/workorder-active.md`, neutral blob `ea5f20b736483c2e102456be1cea9104059fc81a` |
| Issued candidate | `.documents/WIP/plan-next.md`, blob `eab7d40b5517292c15614c74fadbc6586dc2bc4c` |
| ROADMAP | `.documents/WIP/plan-roadmap.md`, blob `d01780d4af965b342521e3594f7a1e4a7dfc551d` |
| Prior canonical report | `.documents/Reports/execution-report.md`, blob `5accbf9962b764be573c5493a7296aa9fcc886fd` |

The previous reconciliation `SOLO-WO-029 R01` was independently **ACCEPTED** at the starting HEAD. `R01.A` is absent from ROADMAP; the remaining Stage 01 packages are `R01.B` and `R01.C`. The only expected pre-existing tracked change is the Owner's installation of **this complete `SOLO-WO-030 R01`** in ACTIVE; it replaces the neutral heading. The Owner's installation is the authorization; do not add an additional approval ceremony or modify the installed scope.

Follow the required reading order in `README.md` and `AGENTS.md`: About, Architecture, both SOPs, all four WIP files, the canonical execution report, then relevant code. Binding detail: Architecture §§5A, 6A.2, 6A.3, 6A.4; Architect SOP §§6, 6A, 6B, 7, 7A, 7B, 8; Implementation SOP §§4–7A. Architecture §6A.3 is the checker contract; this order only bounds its implementation. Architect SOP §6B alone governs Git mechanics.

### 2. Local-only preflight and issue transition

Before creating or changing any other file, installing tooling or fetching dependencies, establish the actual existing Solo repository root (expected Owner-local location `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`). Verify: `main`, starting HEAD/tree above, `origin`, local/remote tracking, exactly one checkout/worktree, Git status including ignored/untracked, stashes, symlinks/reparse points, source-file ownership, and absence of unapproved or Owner-owned dirty material. Do not create another clone, branch, worktree, nested repository or shadow source tree. The installed ACTIVE may differ from neutral baseline; any **other** unexplained difference is a STOP.

Verify `.documents/WIP/` holds exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md` and no other entry. NEXT contains exactly the single accepted `R01.B` candidate; ROADMAP contains `R01.B` and `R01.C` under Stage 01, but not `R01.A`; OUTLOOK remains unchanged. ACTIVE must contain this sole `SOLO-WO-030 R01` work order with its full scope.

Check the following source identities before editing. Mismatched source, unauthorized Git state or uncertain ownership is a STOP; never discard it through reset/clean/checkout.

| Baseline file | Expected Git blob SHA |
| --- | --- |
| `README.md` | `3f3005d680d9ed182a02e574744054aaae55deb2` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `.documents/About/FOUNDATION-SOLO.md` | `1d80f62769053ba4e6ca51adf2d11c68e724c30e` |
| `.documents/Architecture/ARCHITECTURE.md` | `03e8923f834b50fc28e1b56e70f73017525cfa5b` |
| `.documents/SOP/sop-architect.md` | `8760be323b6ed85695a9dcffd3e52f94628f0f07` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/plan-next.md` | `eab7d40b5517292c15614c74fadbc6586dc2bc4c` |
| `.documents/WIP/plan-roadmap.md` | `d01780d4af965b342521e3594f7a1e4a7dfc551d` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `5accbf9962b764be573c5493a7296aa9fcc886fd` |
| `Cargo.toml` | `14505be49a281ba0495eabea5ca386548cc36347` |
| `Cargo.lock` | `d64704daf4f1affcdcb1c2572a240cf833d97948` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.cargo/config.toml` | `74a4fb6c0f595ebc64841b26e7961967294f7d3e` |
| `crates/solo-cli/Cargo.toml` | `f79640686742074bd06cb38ddeed4aa50dfff329` |
| `crates/solo-cli/tests/workspace_graph.rs` | `f5a4ea459859b522da90228810eda7739a468b06` |
| `crates/solo-cli/src/main.rs` | `6faa9fae29351123a49d45c5637ac4fbb1b615c8` |
| `crates/solo-cli/tests/cli_smoke.rs` | `921cbbe91fb0a75919014e91455fbc080dfd111c` |

**Issued candidate transition:** After the preflight succeeds, replace `.documents/WIP/plan-next.md` with exactly one line and a single trailing LF:

```text
# Foundation Solo — plan-next (Next Candidate)
```

Do not edit ROADMAP, OUTLOOK or the installed ACTIVE content. The NEXT transition records that the selected work is now in ACTIVE, **not** that `R01.B` is accepted. Preserve the full installed ACTIVE text in the local commit and final push.

### 3. Exact implementation architecture

**Keep four and only four Cargo packages:** `solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`; Rust 2024 edition, resolver 3, exact pinned `rustc 1.99.1`, one tracked root `Cargo.lock`, `#![forbid(unsafe_code)]` as already established. The production binary remains solely `solo-cli`, with its current `bootstrap-preflight` interface and composition root unchanged. No runtime dependency, second application entrypoint, provider, outbound port, HTTP layer or persisted Installation is authorized.

**Test/check-only dependency:** Add to `[dev-dependencies]` of **`crates/solo-cli/Cargo.toml`** the already Architecture-selected exact `syn =3.0.9`, with the minimum verified AST traversal features needed (`full`, `visit`, plus defaults where required). Existing `serde_json = "=1.0.154"` remains dev-only for Cargo metadata. `syn 3.0.9` and its known supporting crates already have entries in the starting lockfile; update the root `Cargo.lock` using Cargo, not by hand, to reflect the approved direct dev-dependency. Confirm no unapproved crate/version has entered the locked graph. Review the exact registry checksum, licence/attribution, advisory state and Rust 1.99.1 compatibility before treating it as accepted. If an additional package, different pin, breaking compiler requirement or incompatible feature is necessary, STOP rather than substituting it.

**Checker location and invocation:** Implement a **test-only** `check-architecture` executable in the existing `solo-cli` package as a custom integration-test target, not as a second shipped binary or workspace package:

```toml
[[test]]
name = "check-architecture"
path = "tests/check_architecture.rs"
harness = false
```

Its standard invocation is:

```text
cargo test --locked -p solo-cli --test check-architecture
```

With no special arguments, this command must inspect **the real single checkout** and return process exit `0` only when both actual Cargo graph and source-level checks succeed; on a violation it must return nonzero with stable, bounded, repository-relative diagnostics. A `--self-test` mode must exercise the same checker over the negative matrix in §5 and return `0` only when every forbidden fixture is refused; a `--fixture <id>` mode must run one known forbidden fixture **through that same check path** and return nonzero. Do not interpret these deliberately nonzero fixture processes as a failed normal build. A working test-only checker is not an alternative production composition root. The ordinary `cargo test --workspace --all-targets --locked` must execute the legal-tree gate by default.

Organize the implementation under the precise test paths in §6. Reuse/refactor the existing Cargo graph verification carefully; retain its real metadata checks, negative-injection evidence and the previously established graph refusal tests. Avoid duplicated contradictory rule sets. The new `syn` dev edge must be explicitly approved **only** in `solo-cli` development/test context; it must not be whitelisted for domain/application/adapters or for any runtime target.

### 4. Source-boundary enforcement contract

Implement a **real AST-based, fail-closed** checker of all Rust source reachable from the four workspace packages and declared Cargo targets (including library/binary, `#[cfg(test)]` modules, integration tests and any declared module paths). Use the actual Cargo package identities and dependency aliases/kinds resolved through `cargo metadata --locked --all-features`, not filename substrings or a hand-written graph. File enumeration must be deterministic, confined to the verified checkout, and aware of out-of-line and inline modules; reject out-of-root, symlink/reparse escapes, hidden/unknown sources, unsupported build-script/source-loading paths and unreadable or unparsable required files. Tests may use their approved dev-dependencies; production source may not import them.

Enforce, at minimum:

1. **Real graph and workspace:** exactly four existing workspace packages, no illegal outward or cyclic normal/dev/build edge or renamed dependency, no unapproved external transitive reachability from inner crates. Preserve the existing real-graph negative suite; the new checker also fails when the real graph is illegal.
2. **Layer direction:** `solo-domain` may use `std`/`core`/`alloc` only; `solo-application` may depend on `solo-domain`, not adapters, CLI, transport, filesystem or provider implementation; `solo-adapters` may depend inward on application/domain but not CLI or other adapter internals across forbidden module boundaries; `solo-cli` may compose inward interfaces and in development may use only expressly approved test dependencies. Detect forbidden references regardless of their spelling or how they are reached.
3. **Import and reference resolution:** inspect `use`/`pub use` trees (including globs), `extern crate`, `as` aliases, nested scopes and re-export chains, fully qualified and absolute paths appearing in expressions, types, traits, attributes and macro arguments, and `crate`/`self`/`super`-relative module paths. A re-export must not launder a forbidden dependency or expose private adapter internals. An unresolved alias/re-export is a refusal, not an implicit permission.
4. **Module and source loading:** traverse inline modules and deterministically resolvable out-of-line `mod` declarations; inspect `#[path]`, literal `include!`, and supported generated source rather than skipping their content. Reject traversal, ambiguous/conflicting module locations, out-of-root include, symlink escapes, and dynamic or unresolvable `#[path]`/`include!` expressions. Do not turn file existence alone into a source authority or use uncontrolled OS path expansion.
5. **Macros and generated forms:** inspect safely analyzable standard macro forms used by the accepted source; explicitly bound and document any audited harmless macro allowlist. Where a macro can introduce imports, items or source, inspect its actual supported expansion or refuse it. In particular, a `macro_rules!` or proc-macro-produced illegal reference cannot pass by hiding the edge in token streams. Unknown expansions, unrecognized `Verbatim` AST nodes and unresolved generated paths fail closed; do not make arbitrary macro token text a permission escape. Do not ban the existing audited harmless expressions merely to claim a working checker.
6. **Dynamic/foreign imports:** refuse `libloading`, `dlopen`/equivalent dynamic loading, unapproved `extern`/FFI loading and constructed source/module paths as a bypass; cover disguised paths and aliases rather than only raw literal substring searches. A disallowed form remains refused even under a disabled Cargo feature or conditional compile branch inspected by the checker.
7. **Diagnostic safety:** emit deterministic error category, relative source path/fixture ID and concise failure reason, without secrets, absolute Owner paths, raw authored control sequences or a panic/stack dump. A malformed fixture must fail with an error, never cause a green exit. No `--disable`, environment override, permissive fallback or partial-scan success mode may exist in the delivered default checker.

**Practical boundary:** AST scanning is not proof of arbitrary compiler macro expansion. If a required macro/source-loading form cannot be checked conclusively with the approved tools and feature set, **reject that form** and report its exact class. Do not claim full compiler semantic resolution where it was not performed. The existing production/test source must pass without weakening the rule or silently ignoring an entire target/directory. Cargo metadata is necessary but not a substitute for the source checker, and grep/string matching alone is insufficient.

### 5. Negative, bypass and regression evidence

For each negative class below, exercise a syntactically representative in-memory or safely owned disposable fixture with the **same** graph/source checking routines used by the real-tree command; do not create another checkout or mutate working product source to fabricate evidence. A fixture run with `--fixture <id>` must have an observed **nonzero** exit, and `--self-test` must verify the entire matrix with an exit `0` only when the expected refusals occur. The implementation may use an internal typed fixture registry rather than new fixture files.

| Required negative class | Prohibited example to detect |
| --- | --- |
| Direct outward dependency | Domain/application directly imports an adapter/CLI or a forbidden external crate |
| Renamed crate/alias | `extern crate` or `use ... as ...` disguises the forbidden target |
| Re-export chain | `pub use` or a glob exports a forbidden dependency via another module |
| Qualified/reference path | `outer::Type`, `::outer::method`, or a type/trait/attribute path bypasses explicit `use` |
| `crate`/`self`/`super` and private sibling | Indirect module path reaches an unapproved adapter internal or prohibited outward module |
| Module loading | `#[path]`, out-of-line module, literal `include!` with forbidden content; traversal or unresolved source form is refused |
| Macro-generated edge | Macro definition/invocation produces a forbidden import/reference or cannot be safely expanded |
| Dynamic/foreign load | `libloading`/`dlopen`, an unsafe external loader, or a disguised dynamic edge |
| Illegal dev/build/transitive graph | Named, aliased, feature or transitive edge breaks the accepted Cargo graph |
| Checker bypass | Deliberately disabling/short-circuiting the guard makes the **self-test fail**, not pass |
| Unsupported/invalid input | Unparseable source, missing module, unknown macro or unreadable target fails closed |

Also prove positive cases: current `R01.A` domain/application/adapters/CLI boundaries, normal standard-library imports and safe audited macros, legal `solo-cli` dev-only `serde_json` and `syn`, and the existing CLI process success/refusal outcomes all remain green. The existing graph guard must continue to fail on deliberate forbidden edges and on bypass, not merely print a warning. Report explicitly which macro-expansion and compiler-resolution classes were inspected versus conservatively refused. Do not call a synthetic fixture a checked real workspace.

### 6. Exact permanent path ledger and operation permissions

Only these permanent paths may change under the verified Solo root:

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner-installed complete work order; **leave bytes unchanged**, include in final local commit |
| `.documents/WIP/plan-next.md` | Replace issued `R01.B` candidate with the exact neutral heading in §2 |
| `crates/solo-cli/Cargo.toml` | Add only the `syn =3.0.9` exact **dev** dependency/features and `[[test]]` checker target specified in §3; keep existing binary and other dependencies |
| `Cargo.lock` | Cargo-generated locked update for the authorized `syn` dev-dependency, no other unapproved package/version |
| `crates/solo-cli/tests/workspace_graph.rs` | Constrained refactor/extension so the real graph guard and negative fixtures account for the approved dev edge, with no weakening of earlier guards |
| `crates/solo-cli/tests/check_architecture.rs` | Create the sole test-only checker invocation and exit-code interface |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Create optional regular test target for negative/bypass regressions if needed |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | Create shared test-only checker module for Cargo metadata and source AST logic |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | Optional split of shared graph logic |
| `crates/solo-cli/tests/architecture_checks/source.rs` | Optional split of shared `syn` and module-resolution logic |
| `.documents/Reports/execution-report.md` | Overwrite the one canonical report with current run evidence |

No blanket permission for other files under `tests/` or `crates/`. Parent `architecture_checks/` is a structural directory only. No edit to the already accepted production Rust files, four crate roots, other crate manifests, `Cargo.toml` at workspace root, `.cargo/config.toml`, `rust-toolchain.toml`, `README.md`, `AGENTS.md`, About, Architecture, SOP, ROADMAP, OUTLOOK, `.gitignore`, security/license files, CI/workflows, browser, content, scripts or other repository. No extra `[[bin]]`, workspace member, generated schema, public manifest, tool package, Rust build script or alternative report.

**Allowed local operations:** read/inspect; code edits to listed paths; exact pinned Rust 1.99.1 tooling in verified task scratch if needed; registry fetch of the **selected** `syn =3.0.9` and already locked dependencies only after provenance review; local tests, Cargo metadata, release builds and negative fixture processes; temporary fixtures/caches/logs only under Solo `.temp/` or the actual Cline-provided task temporary area; local Git staging and commits of allowed paths; **one final push** of completed permitted commits to existing `origin/main` after all local work, tests and canonical report are finished. Follow Architect SOP §§6A and 6B for scratch/Git mechanics. Clean task-owned scratch without deleting unknown data. No second checkout.

**Denied:** intermediate GitHub push/readback/publication, GitHub file editing, new remote/branch/clone/worktree, PR/issue/merge, history rewrite, rebase, force-push, destructive clean/reset/checkout, other-repository writes, new unapproved dependency or toolchain pin, global OS mutation, secrets, credentials, external provider, release/deployment, R01.C work or WIP administrative closure. Unlisted permanent writes or external operations are unauthorized.

### 7. Required verification, acceptance and report

All checks run **locally** against the corrected, committed source identity with pinned Rust 1.99.1 and locked graph; direct commands and exit codes must appear in the canonical report. At minimum:

```text
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --locked -p solo-cli --test check-architecture
cargo test --locked -p solo-cli --test check-architecture -- --self-test
cargo build --workspace --release --locked
cargo metadata --format-version 1 --locked --all-features
cargo tree --workspace --all-features --locked
```

For each forbidden fixture ID, run the same checker target with `-- --fixture <id>` and record its **intentionally nonzero** process exit and safe diagnostic; a fixture unexpectedly returning zero is a failed gate. Re-run the previous `workspace_graph` real and negative tests, and the compiled CLI's success (`entity-7`, `https://EXAMPLE.com:443/alpha`) and input refusal (`Main`) with exact stdout/stderr/status. Verify `syn`/`serde_json` do not appear in the **runtime** dependency closure or shipped executable. Run `--locked --offline` build/tests once trusted caches are populated; exercise lock drift refusal and record limitations honestly.

**Reproducibility:** repeat two independent fresh-cache/fresh-target Windows GNU release builds in the same checkout, same pinned `x86_64-pc-windows-gnu` Rust 1.99.1 and unchanged target-specific `.cargo/config.toml`. Compare raw SHA-256 executable hashes and lock identity; no PE-header normalization or ephemeral linker flag. The acceptance target is exact equality **between both newly built artifacts**, not mandatory equality with a former release artifact after sources/check metadata changed. If the required GNU toolchain cannot run, mark the gate failed/unverified and STOP; do not infer success from prior reports or silently change target.

| Gate | Passing evidence | Failure sensitivity |
| --- | --- | --- |
| **B-01 — Safety and issue** | Exact baseline, one checkout, permitted Owner-installed ACTIVE, neutral NEXT, only allowed changed paths | Drift, unknown material, extra WIP or unauthorized change → STOP |
| **B-02 — Selected tool and graph** | `syn =3.0.9` dev-only, correct lock provenance, real metadata graph correct for normal/dev/build and aliases | New runtime dependency, illegal edge, unreviewed version → FAIL/STOP |
| **B-03 — Real source boundary** | AST checker invoked on all required **real** Rust targets, legal source exits 0, module/alias/re-export/qualified-path analysis valid | Skipped target, grep-only implementation, permissive unknown → FAIL |
| **B-04 — Negative and bypass** | Every §5 fixture exits nonzero, self-test catches disabled guard, malformed/unsupported forms fail closed | Missed illegal edge or bypass passing → FAIL |
| **B-05 — No regressions** | fmt, strict Clippy, tests, actual CLI, locked/offline release, raw-hash reproducibility and prior graph negatives pass | Failing mandatory check, ignored warning, nonidentical builds → FAIL |
| **B-06 — Security and cleanup** | No new unsafe/import escape, safe diagnostics, exact dependency licence/advisory review, task scratch cleaned | Secret/data exposure, dangerous path, untriaged high risk, residue → STOP |
| **B-07 — Report and final publication** | One current factual report, clean local tree, only allowed paths, one safe final push and confirmed remote identity | Extra artifact, premature push, missing evidence or unsafe publication → FAIL/STOP |

Record `PASS`, `FAIL`, `UNVERIFIED` or justified `NOT APPLICABLE` for each relevant subcheck; `COMPLETE` is the agent's factual work status, **not** Architect `ACCEPT`. Explicitly distinguish source text inspected, generated/macro forms rejected, tested compiler/OS/target and any unrun checks. MSVC, Unix, CI/browser, HTTP, future product validator, Stage 02+ and all provider/release gates remain deferred or unverified as applicable; the full Stage 01 exit is not an `R01.B` completion claim.

**Canonical report:** Overwrite only `.documents/Reports/execution-report.md` with the date, work order, root/branch, starting HEAD/tree, implementation source commit/tree, changed-path ledger, direct dependency/lock provenance, checker entrypoint and source enumeration, real graph result, each negative fixture ID/input class and exit, disabled-guard proof, ordinary CLI exits, `fmt`/Clippy/tests/build/metadata/tree/offline results, fresh binary hashes, security/limitation/cleanup facts, WIP state, and final `COMPLETE`, `STOPPED` or `FAILED`. Do not embed the report's own commit SHA; hand it back in chat.

Complete source changes, inspection, tests, evidence and Git commits **locally**. Commit permitted source/WIP changes, test the committed identity, overwrite and commit the sole report, audit working-tree cleanliness and unchanged governing/product paths, then make **one final push** to `origin/main`. Confirm final remote outcome under Architect SOP §6B; no intermediate GitHub operations. If a mandatory gate fails, STOP, preserve evidence, and publish a truthful stopped report at run end **only where safe under the SOP**—do not publish unsafe incomplete code to manufacture success.

The Implementation Agent does not self-accept, clear ACTIVE, remove `R01.B` from ROADMAP, choose `R01.C` for NEXT, merge, release or deploy. The Chief Architect separately reviews the exact published revision and reports `ACCEPT`, `REVISE` or `STOP`.

### 8. STOP conditions

STOP on unauthorized local baseline or WIP state, missing Owner-installed scope, undocumented source discovery, unable-to-parse generated/macro form that requires approval, illegal import or loader, inability to prove fail-closed negatives, incompatible `syn`/Rust versions, new unapproved dependency, untrusted package/advisory conflict, hidden runtime dependency, unsafe diagnostics or filesystem operation, mandatory test/reproducibility failure, unauthorized file/path, unresolved task-owned scratch or unsafe final publication. Do not soften the boundary, skip files or fixtures, change the source-checker goalposts, create a new runtime, or silently substitute a compiler/tool. Give the smallest concrete Owner/Architect decision needed.
