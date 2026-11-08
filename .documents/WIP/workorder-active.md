# Foundation Solo — Active Work Order

## SOLO-WO-031 R01 — R01.B Source-Loading Fail-Closed Correction

## Approved Scope — OWNER-ISSUED

### 1. Objective and exact reviewed baseline

Correct the two outstanding source-loading failures from the Chief Architect's independent **REVISE** review of `SOLO-WO-030 R01`, and strengthen the fixture-result evidence. This is a **bounded correction within Stage 01 `R01.B`**, not a new feature or an authorization to close the package:

1. The `check-architecture` `include!` path is checked lexically but read without checking its effective filesystem destination, allowing a symlink or Windows junction/reparse-point path that escapes the package root to be opened and scanned as though it were confined.
2. Module resolution handles direct literal `#[path = "..."]` but not a conditional source selection such as `#[cfg_attr(windows, path = "../../other-source.rs")] mod alternate;`. If a valid default `alternate.rs` also exists, the checker can inspect only the default while the compiler selects another source file. Malformed/uninspectable path attributes must likewise never silently fall back to the default.
3. The `--fixture` command currently returns violation exit `1` both when a forbidden fixture is correctly refused **and** when it is unexpectedly accepted. Distinguish those outcomes; an arbitrary nonzero status alone is not evidence of a refusal.

Preserve the accepted `R01.A` implementation and the existing `R01.B` four-package Cargo graph checker, `syn =3.0.9` development-only dependency, normal CLI, 19 prior negative fixture classes, four legal twins and default legal-tree gate. Do not introduce a second checker, second Rust workspace crate or production binary. No Stage 01 `R01.C`, CI, browser, HTTP, Stage 02+, publication, provider, release or deployment work.

| Reviewed identity | Exact value |
| --- | --- |
| Repository / local branch | `provelopment/provelopment-foundation-solo` / existing `main` |
| Starting HEAD | `4abb80ea43cbd2c30c04d6200a2f24fd87af668f` |
| Starting tree | `2ad346afb2a0ed0206e233a33c9c0e8ed7ce1cfe` |
| `R01.B` implementation commit | `844f878c2e0214267d64629aa3f9eae42f975eb1` |
| Prior execution report blob | `86371c43f9b61cb4d625da5425061f95660090bb` |
| Prior issued ACTIVE blob | `da65750c08bd4d6d69258c7a6511de289235def2` (`SOLO-WO-030 R01`) |
| NEXT / ROADMAP / OUTLOOK blobs | `92fa562b1af0c54391851aa5c79deb1631127069` / `d01780d4af965b342521e3594f7a1e4a7dfc551d` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Checker source blob | `crates/solo-cli/tests/architecture_checks/source.rs`: `9efc69c5e6dd759aad9f50eb5b76d289c1c7f709` |
| Fixture registry blob | `crates/solo-cli/tests/architecture_checks/mod.rs`: `a5119190b2bd87a196521666fc4e468a4dca83f7` |
| Checker process / standard fixture-test blobs | `check_architecture.rs`: `39c77a59447397f1c64bdc36a4e08a7f3b3dd095`; `check_architecture_fixtures.rs`: `6a5e540cb62baaa2c94af29858ba71251259169a` |

The previous agent's report is **not** Architect acceptance: the review outcome is `REVISE`. `R01.B` and `R01.C` remain in ROADMAP; NEXT is the neutral single heading; ACTIVE is the prior issued order until Owner installation of this one. Neither the prior `COMPLETE` report nor this correction independently closes `R01.B`.

Read `README.md`, `AGENTS.md`, About, Architecture, both SOPs, all four WIP files, the canonical report, then relevant source in the mandatory order. Binding architectural requirements are Architecture §§5A, 6A.2–6A.4 and the Architect SOP §§6, 6A, 6B, 7, 7A, 7B, 8; the Implementation SOP governs role-specific preflight, testing, reporting and cleanup. This order does not override them. Architect SOP §6B remains the sole Git procedure.

### 2. Local checkout and authority preflight

Use the **single existing Solo checkout** (expected Owner-local location `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; verify actual accessible root). Before any writes, installs, source execution or network fetch:

- Verify Git root, `main`, exact HEAD/tree above, sole `origin` and tracking, one worktree, status including staged/unstaged/untracked/ignored, stashes, directory links/reparse escapes and Owner-owned/unrelated material. Do not clone, create a shadow tree or secondary worktree, reset/clean/discard unknown data or touch a parent/sibling repository.
- The **only expected pre-existing tracked difference** is the Owner's replacement of `SOLO-WO-030 R01` ACTIVE by this **complete** `SOLO-WO-031 R01` work order. Verify the `OWNER-ISSUED` scope and matching Owner handoff before action. Preserve Owner-installed ACTIVE bytes unchanged; commit that file with the implementation.
- Verify WIP contains exactly four files and no subdirectory: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`. ROADMAP still contains `R01.B` and `R01.C` but not `R01.A`; NEXT is the neutral heading; OUTLOOK unchanged. None may be changed in this run other than the Owner's installation of ACTIVE.
- Verify the source/fixture/test blobs above, plus `Cargo.toml` `14505be49a281ba0495eabea5ca386548cc36347`, `Cargo.lock` `05a02666894f4f0bf5528acf118a049e5edb853e`, `crates/solo-cli/Cargo.toml` `1ca5e114a6e034aae386ea9d7adab3a81b4ae8bc`, `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`, `.cargo/config.toml` `74a4fb6c0f595ebc64841b26e7961967294f7d3e`, existing graph module `e328dd6a9f54c47e3eae60053446d52edacc74b7`, graph test `8fa4b4d0bde840782abb0c9e24c84bac8f06c947`, `crates/solo-cli/src/main.rs` `6faa9fae29351123a49d45c5637ac4fbb1b615c8`, and `crates/solo-cli/tests/cli_smoke.rs` `921cbbe91fb0a75919014e91455fbc080dfd111c`. Governing README, AGENTS, About, Architecture and SOPs must also be unchanged.
- Establish the available pinned Rust `1.99.1` host/target. Windows GNU is the previously evidenced target; MSVC/Unix remain unverified until actually tested. Any unexpected drift, unsafe path, missing authority or third-party/Owner material at risk is an immediate **STOP**, with only safe canonical reporting per SOP.

### 3. Required source-loading correction

Correct **the actual checker path**, not a separate fixture-only approximation, inside `crates/solo-cli/tests/architecture_checks/source.rs`. These rules apply to any path from which the checker may inspect Rust source, not just one function:

**A. Filesystem confinement before use.** Validate the effective file and every relevant directory component against the verified package boundary before opening/reading, parsing, traversing or treating it as a reachable source. Check the original filesystem entries for symlink/junction/reparse redirection as appropriate to the host, and canonicalize the **actual source target** rather than relying only on `normalize_lexical(...).starts_with(package_root)`. Reject any redirect or effective path outside the correct package root, including a lexically in-root `include!("alias/forbidden.inc")` whose `alias` is a junction/symlink to a sibling directory. Reject failures to inspect or canonicalize; do **not** use `let Ok(...) = canonicalize(...) else { return; }` or follow an escape and merely record a warning while continuing. Fail closed on detected inconsistencies; never claim to eliminate hostile time-of-check/time-of-use races without platform-supported evidence.

Apply one coherent confinement policy to:

- literal `include!` and its content, including **non-`.rs` source fragments**;
- direct `#[path]` modules and default out-of-line `mod` loading;
- declared Cargo target roots, any module files discovered recursively and the public-surface/glob-re-export source reader;
- discovery of package Rust files (`rust_files_under`) and its directory walking, including unreadable entries, unknown symlink/junction traversal and missed files.

In-memory source fixtures must remain supported without consulting the host filesystem as if they were real files; the **real** filesystem reader must provide real effective-path enforcement. A well-defined reader/confinement abstraction is acceptable within the authorized source file. A failure to prove an effective path is safe is a **violation**, not a silent skip, and diagnostics must remain concise and repository-relative. Preserve legitimate in-package loading and the legal committed source tree.

**B. Conditional and malformed module path selection.** Never silently inspect a default `mod` file when any module attribute could change that module's effective source path. Analyze and unambiguously enumerate all relevant supported conditional paths (including nested `cfg_attr` and `cfg` combinations) **or**, as the simpler fully compliant implementation, conservatively **refuse** module-local `cfg_attr` that may select or obscure `path`, together with unsupported/malformed/duplicate `#[path]` configurations. Even when the default `name.rs` exists, an alternate path must not be left unchecked. Ordinary safe `#[cfg(test)]` modules and harmless attributes already in the committed code should continue to pass; do not ban unrelated conditional compilation just to turn the gate green. Paths chosen by nested conditionals, generated source or unknown attribute forms must be proven checked across relevant variants or refused, never guessed from the current platform alone.

**C. Preserve original checker strength.** Keep the existing graph checks, inward-only rules, source AST traversal and `syn =3.0.9` pin, module/import aliases, re-exports, macro refusals and audited harmless macro handling. No new permissive switch, partial-scan success path, dynamic-import exception, extra production dependency or hidden skip. The current 19 forbidden fixtures and four legal twins must remain valid; no test may be weakened, deleted, ignored or made conditional to manufacture a pass.

### 4. Required adversarial and behavioral evidence

Add focused tests through the **same real source-checking routines** (not a separate mock algorithm):

1. **Real redirected `include!` escape:** Create an exclusively owned disposable fixture **inside Solo `.temp/`** (or the actual Cline task scratch) with a synthetic package root and a sibling target directory, both within approved scratch. Make a genuine Windows directory junction (e.g. `mklink /J`, without elevation or global policy change) or Unix directory symlink from an in-package `alias` path to that sibling. The package root contains `include!("alias/forbidden.inc")`, and the target contains valid parsable Rust source; its extension is deliberately `.inc`. Prove the lexical path lies inside the package while the effective path escapes, and that the real checker **refuses before consuming the redirected source**, preserves the link/target/sentinel contents and causes no foreign changes. If the host cannot safely build a genuine redirect, mark this gate **UNVERIFIED** and STOP, not skipped/passed.
2. **Out-of-line module redirect:** Repeat the relevant escape with a genuine redirected `#[path]` or default `mod` source. Confirm the checker refuses rather than returning an empty diagnostic after canonicalization failure. Test fail-closed error handling for a missing/unreadable/cannot-canonicalize required source where feasible, without relying on OS privileges.
3. **Conditional module path with valid default:** Give `mod alternate;` a genuine default `alternate.rs` **and** a `#[cfg_attr(windows, path = "...")]` or equivalent alternate source. Verify the same checker refuses uninspected conditional path selection or fully inspects each supported variant. Include nested/malformed/non-literal/duplicate path-attribute cases as applicable and keep ordinary `#[cfg(test)]` legal. A test that passes merely because the default exists is a **failure**.
4. **Fixture result semantics:** Retain `--fixture <id>` for deliberately illegal cases, but make a correctly refused case observably distinct from an unexpectedly **accepted** forbidden fixture. For example: `1` = expected policy refusal, `2` = invalid arguments, `3` = forbidden fixture accepted / harness failure (or equivalent documented distinct statuses). Tests must assert the correct refusal **category and outcome text**, not just `status != 0`. Preserve ordinary real-tree `0`/`1` and `--self-test` `0` only when the real source and all negative/positive cases have correct outcomes. Where a `cargo test` wrapper remaps the checker process exit code, report both accurately and do not confuse a wrapper failure with the checker’s own status.
5. Keep the full existing forbidden fixture matrix and all four legal twins passing. Ensure `cargo test --workspace --all-targets --locked` automatically exercises these new negatives, rather than requiring a manual-only test mode. No synthetic-only redirected-path proof; no secondary checkout or tracked fixture outputs.

Test cleanup must remove only positively owned, task-specific fixture links/directories and preserve unknown/pre-existing material. A redirected test path must **never** be used as a cleanup root or trigger deletion of its target. Record actual OS/target limits.

### 5. Exact allowed permanent paths and permissions

| Path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner-installed `SOLO-WO-031 R01`; **preserve full text byte-for-byte** and commit |
| `crates/solo-cli/tests/architecture_checks/source.rs` | Fix real source-path resolution, conditional module attributes and filesystem checks; add narrowly related helper/tests as needed |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | Add the correction's forbidden fixture IDs, categories and self-test wiring, preserving previous fixtures |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Add ordinary test gates for real redirected paths and conditional path refusal; preserve old tests |
| `crates/solo-cli/tests/check_architecture.rs` | Distinguish expected `--fixture` refusal from unexpected acceptance without changing the legal-tree contract |
| `.documents/Reports/execution-report.md` | Overwrite the **single** canonical report with this run's evidence |

All other permanent paths are **read-only**, including ROADMAP, NEXT, OUTLOOK, `Cargo.lock`, manifests, other Rust tests and production code, `graph.rs`, the root `Cargo.toml`, toolchain/config, README, AGENTS, About, Architecture, both SOPs, `.gitignore`, licensing/security/CI/browser files, and all other repositories. No new tracked test-fixture directory, package, dependency or report archive. If a correct fix actually requires another permanent file or unapproved dependency, **STOP** and return the exact reason; do not widen scope yourself.

**Allowed:** read-only inspection, bounded source/test edits only at the listed paths, use of existing Rust `1.99.1` and locked crates, conditional installation of the **same pinned toolchain/locked packages** into approved owned scratch if unavailable, execution of required local Cargo/test/build commands, temporary fixture directories under verified Solo `.temp/` or actual Cline-provided task scratch, local commits of only authorized files, and **one final push** to the existing `origin/main` after all local source work, tests and report are complete. Verify the remote outcome afterward under Architect SOP §6B. No intermediate GitHub actions are needed or permitted for the agent's implementation workflow.

**Denied:** other permanent edits, package/lock/version changes, other-repo writes, unauthorized network/providers/secrets, global OS configuration or privilege changes, additional clone/worktree/branch/PR, force push, reset/clean/rebase/history rewrite, merge, release, deployment, R01.C or WIP closure. Ordinary disposal of positively owned scratch is authorized and required; do not recursively delete unknown material.

### 6. Verification and exit conditions

Complete corrections locally, commit permitted source/ACTIVE changes, then run verification on that exact committed source identity. Required commands include:

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

Run the new and previous forbidden `--fixture <id>` cases with correct observed outcomes and category assertions. Run focused real-filesystem redirected-source and `cfg_attr` regressions. Rerun existing `workspace_graph`, `cli_smoke` and `bootstrap-preflight` real-process success (`entity-7`, `https://EXAMPLE.com:443/alpha` → `0`) and refusal (`Main` → `2`) gates. After trusted caches are populated run relevant `--locked --offline` tests and release build. Confirm `Cargo.lock` and every unchanged file retain their exact baseline Git blobs. Demonstrate two independent fresh-cache/fresh-target Windows GNU release builds with the tracked `.cargo/config.toml`, empty environment-injected `RUSTFLAGS` and exact raw SHA-256 binary equality (no header normalization). If the required host/toolchain or this mandatory reproducibility check is unavailable, mark it **UNVERIFIED** and STOP rather than reusing an earlier run's success.

| Gate | Independent acceptance evidence required |
| --- | --- |
| **F-01 Authority and diff** | Exact starting HEAD/tree, one checkout, only Owner-installed ACTIVE difference before work, only six listed permanent paths touched, ROADMAP/NEXT/OUTLOOK unchanged |
| **F-02 Effective filesystem confinement** | All source-open/traversal paths fail closed on symlink/junction/reparse escapes and unresolved effective paths, including non-`.rs` `include!`, `#[path]` and module/public-surface discovery |
| **F-03 Conditional path selection** | `cfg_attr(path=...)` with a valid default, nested/unsupported/malformed path selection cannot pass unchecked; safe ordinary `cfg(test)` remains legal |
| **F-04 Real negatives and fixture truth** | Genuine host redirect test, target preservation, relevant missing/canonicalization failure, all old/new negatives with expected category, legal twins green, `--fixture` unexpected acceptance distinct, self-test/bypass load-bearing |
| **F-05 Regression/reproducibility** | fmt, strict Clippy, locked/ offline tests and builds, graph guard, CLI exits/output and byte-identical independently built GNU executable; compiler/OS limits accurately stated |
| **F-06 Security/cleanup/report** | No unsafe or hidden fallback, no new runtime dependency, safe relative diagnostics, all owned scratch cleaned or exact blocker disclosed, one canonical report, clean tree, one final push and remote identity confirmation |

Report every gate as `PASS`, `FAIL` or `UNVERIFIED`; general standards genuinely outside this correction as `NOT APPLICABLE` with reason. Do not promote old report claims into newly executed evidence. The prior `cargo-deny` advisory scan, MSVC and Unix builds, R01.C browser/CI, future stages and release remain explicitly **NOT RUN/UNVERIFIED** where applicable. No acceptance of full Stage 01 is claimed.

### 7. Canonical report, final publication and STOP

Overwrite only `.documents/Reports/execution-report.md`. Include this order ID/edition, Owner handoff, actual root/branch, starting and corrected source HEAD/tree, exact changed paths, inspected filesystem paths and rejection logic, which redirect type was actually created, lexical versus effective-path proof, target material-preservation outcome, conditional-module tests, fixture IDs/expected categories/process exits (distinguishing Cargo wrapper exit), regression commands/exits/warnings, raw independent binary hashes, pinned dependency/lock evidence, host limitations, task scratch inventory/cleanup, WIP state, `COMPLETE`/`STOPPED`/`FAILED` status and final one-push publication evidence. The report is public: use repository-relative diagnostics and no credentials, private absolute paths, secrets or unredacted hostile payload. Do **not** embed its own report-commit SHA; provide that SHA in the chat handback after publication.

Sequence: verify local state → change only listed files → run focused safety/negative tests → commit permitted source/unchanged Owner-issued ACTIVE → run full gates on the committed identity → overwrite and commit the report → verify a clean worktree, four WIP files, no unintended permanent change and cleanup → **push all completed safe commits once** to `origin/main` → verify final remote HEAD/tree. Follow Architect SOP §6B for Git operations; no intermediate push. Agent never self-issues Architect `ACCEPT`, clears ACTIVE, removes `R01.B` from ROADMAP, chooses NEXT, merges, releases or deploys.

**STOP** on incorrect baseline/Owner issuance, unchecked redirect or source selection, a fixture unexpectedly accepted, an unverifiable required real redirect, unauthorized path/tool/dependency, unprovable file confinement, secret/foreign-data exposure, failed mandatory test/reproducibility, unsupported macro behavior that would need a weakened policy, unclean task-owned scratch or unsafe publication. Preserve unknown/Owner files and return the smallest concrete correction/Owner decision. Publish a truthful stopped report at end **only when safe** under the standing report procedure; do not conceal failures or publish unsafe incomplete code to manufacture a `COMPLETE` status.
