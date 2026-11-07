# Foundation Solo — Active Work Order

## SOLO-WO-028 R01 — Stage 01 R01.A Scratch-Root Safety Correction

## Approved Scope — OWNER-ISSUED

### 1. Objective and reviewed baseline

Correct the sole unresolved defect identified in the Chief Architect's independent `REVISE` review of `SOLO-WO-027 R01`: the CLI smoke-test scratch-root validator calls `canonicalize` before inspecting for a symlink or Windows reparse point. Inspecting the **resolved target** cannot establish that the original `.temp` directory entry was not redirected. A junction/symlink targeting another `.temp` directory *inside* the checkout could evade the current checks.

Implement a fail-closed check of the **unresolved filesystem entry** before following it, retain checkout containment validation, and prove the fix through an adversarial filesystem regression. This work completes only the outstanding `R01.A` correction. Preserve the accepted Rust four-crate architecture, CLI and application behavior, exact Windows GNU deterministic-link configuration, Cargo graph, lockfile and dependency evidence. Do not implement `R01.B`, `R01.C`, any subsequent stage, provider integration or release activity.

| Reviewed identity | Exact value |
| --- | --- |
| Repository | `provelopment/provelopment-foundation-solo`, existing local `main` |
| Starting HEAD | `49955538c6b32a9d81e7464fba28a72d2cd04927` |
| Starting tree | `b5311d10251e974d1d26b65a7eec5b507e2dd56a` |
| Reviewed corrected source commit | `f75f1b0ef18fffa438c6fca59063fd5c08e0b581` |
| Current test source, Git blob | `crates/solo-cli/tests/cli_smoke.rs` — `b9d3b80b872eb92da025e4661c969eb8e6cf04e4` |
| Current canonical report, Git blob | `.documents/Reports/execution-report.md` — `9fd446c9410e885522cbbfe8de5339f776204c2c` |
| Current ACTIVE, Git blob | `.documents/WIP/workorder-active.md` — `d5ad2a26e015176a2b4c8896777ad20d90352d96` |
| Current NEXT, Git blob | `.documents/WIP/plan-next.md` — `92fa562b1af0c54391851aa5c79deb1631127069` |

**Governing instructions:** Read `README.md`, `AGENTS.md`, About, Architecture, Architect SOP, Implementation SOP, all four WIP files and the canonical execution report in the order mandated by `AGENTS.md`. Apply Architect SOP §§6, 6A, 6B, 7A, 7B and 8, and Implementation SOP's safety, testing and reporting rules. This order does not amend those documents.

### 2. Local preflight

Work in the existing verified Solo checkout (expected Owner-local path `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`); establish the actual root from local evidence. Before any writes or installations, verify:

- Current local `main` HEAD/tree equal the baseline above; one checkout/worktree, one correct `origin`, no unauthorized branch, stash, nested repository, unexplained tracked/untracked changes, reparse-point escape or Owner-owned material at risk. The **only** expected pre-existing tracked change is the Owner's replacement of the previous ACTIVE with this exact `SOLO-WO-028 R01` work order.
- `.documents/WIP/` has exactly four files: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`. ROADMAP retains `R01.A`, `R01.B` and `R01.C`; NEXT contains only its neutral heading; OUTLOOK is unchanged.
- The inspected `cli_smoke.rs`, canonical report and installed `workorder-active.md` match the reviewed identities, allowing only the specified Owner-installed ACTIVE replacement. Other governing/code/manifest files must not drift. In particular verify `Cargo.lock` Git blob `d64704daf4f1affcdcb1c2572a240cf833d97948`, `.cargo/config.toml` `74a4fb6c0f595ebc64841b26e7961967294f7d3e` and `crates/solo-cli/src/main.rs` `6faa9fae29351123a49d45c5637ac4fbb1b615c8`.
- The exact pinned Rust `1.99.1` toolchain, target and build environment are identified before running tests. The last report's Windows GNU result is evidence of that run only; MSVC remains unverified.

STOP if the baseline, ownership, repository or WIP state is unclear. Never reset, clean or overwrite unknown data to make a preflight pass.

### 3. Required correction

Modify the existing scratch-root validation in `crates/solo-cli/tests/cli_smoke.rs` and nothing else in product/test source.

1. From the canonical **single existing checkout** root, construct the direct child path `checkout.join(".temp")` without resolving that child's filesystem entry.
2. Inspect that original path with `symlink_metadata` **before any `canonicalize`, traversal or use of it as a scratch directory**. On Windows, reject `FILE_ATTRIBUTE_REPARSE_POINT` (`0x0400`) regardless of whether the entry is also a directory; on other platforms reject symbolic links. Reject non-directories and metadata/permission errors rather than treating them as an empty or safe root.
3. If `.temp` is missing, create only that direct child using an ownership-aware, non-following operation, then inspect the newly created **unresolved** entry before use. Do not blindly follow an existing junction via `create_dir_all`; never delete or replace an existing entry. A benign race or replacement between checks must not be described as fully race-safe without evidence; fail closed on detected inconsistencies.
4. Only after the entry passes its unresolved-path check, resolve the effective path and verify it remains the actual checkout's direct `.temp` root. Preserve path containment and existing safeguards. Do not silently accept another directory merely because the resolved path is inside the checkout or has the basename `.temp`.
5. Maintain exclusive, uniquely named scratch children, non-recursive cleanup of positively owned empty children only, no deletion of the shared `.temp` root or foreign contents, no general OS-temp fallback and no second checkout. The real CLI must still create no file in its working directory.
6. Use a small testable helper if needed so the same production test-path validation can be exercised against a controlled fixture root. Do not add a new dependency, expose a production option, or modify unrelated behavior.

**Required adversarial regression:** In disposable, positively owned Solo `.temp/` fixture space, build a test scenario in which the *candidate* `.temp` entry is a real symlink/junction/reparse point and resolves to another `.temp` directory **within the same permitted checkout**. The old implementation's post-resolution-only checks would have accepted that arrangement. Demonstrate that the revised validator instead refuses the original entry **before using the redirected directory**; validation itself must not modify or delete the link, its target or any existing contents. The test harness may remove only its own positively identified fixture link and target during cleanup. On Windows, use a genuine available Windows directory reparse-point/junction mechanism; on other platforms, use a genuine directory symlink. Fixture setup and cleanup must be bounded to positively owned paths and must not change the real checkout `.temp` entry. A synthetic boolean test alone does not establish this end-to-end refusal.

Retain or strengthen the existing positive tests for genuine nonredirected `.temp`, exclusive creation, collision preservation, no output-side effects and non-recursive cleanup. If the host cannot safely produce or verify the real redirected-entry fixture, record the negative gate as **UNVERIFIED** and STOP; never silently skip it or call the gate passed. No new global OS privileges, policy changes or unapproved system configuration are authorized to manufacture the fixture.

### 4. Exact allowed permanent changes and permissions

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Preserve the Owner-installed text unchanged; include in final local commit |
| `crates/solo-cli/tests/cli_smoke.rs` | Edit scratch-root validation and narrowly related positive/adversarial tests only |
| `.documents/Reports/execution-report.md` | Overwrite the single canonical report with current run facts and evidence |

No other permanent path may change. In particular, leave `Cargo.toml`, `Cargo.lock`, all crate manifests, `rust-toolchain.toml`, `.cargo/config.toml`, `crates/solo-cli/src/main.rs`, all other Rust files, README/AGENTS/About/Architecture/SOP, ROADMAP, NEXT, OUTLOOK, `.gitignore`, security and license files unchanged. Do not create another WIP file, alternate report, CI workflow, PR, branch or product artifact.

**Allowed:** local inspection, local edits to listed paths, existing pinned toolchain and locked-dependency use; conditional installation of the *same* verified pinned Rust toolchain and dependencies only into approved task-owned disposable scratch if absent; bounded fixture creation in Solo `.temp/` or an actual Cline-provided task temporary workspace; locally executed tests and builds; local Git commits of listed permanent paths; one final push of completed work to existing `origin/main` after local tests and report are complete. Follow Architect SOP §6B for Git mechanics and remote confirmation.

**Denied:** intermediate GitHub publication or edits; additional clone/worktree/branch/PR, destructive Git cleanup, force push, rebase or history rewrite, provider credentials/accounts/network configuration, global OS configuration changes, new dependency, toolchain/package version change, merge, release or deployment. Read-only access required by trusted tool setup is bounded to already approved sources. Unlisted permanent edits and external actions are unauthorized.

### 5. Required verification and acceptance gates

Run against the corrected locally committed source state; capture actual commands, exits, OS/target, compiler and dependency identity, and any warning or skip:

```text
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --workspace --release --locked
cargo metadata --format-version 1 --locked --all-features
cargo tree --workspace --all-features --locked
```

Once a trusted cache exists, also run applicable `--locked --offline` test/build checks. Reconfirm a real compiled `bootstrap-preflight` success (`entity-7`, `https://EXAMPLE.com:443/alpha`) and refusal (`Main`), plus the existing genuine CLI/architecture negative suites. Compare `Cargo.lock` and `.cargo/config.toml` with their unchanged baseline identities. Revalidate deterministic Windows GNU binary output using two independent clean task-owned cache/target builds on the same source and pinned target; compare raw binary hashes exactly, without timestamp normalization. If the relevant compiler/linker is unavailable, distinguish this precisely as UNVERIFIED and STOP rather than implying an old run's results are new results.

| Gate | Must demonstrate | Result if absent |
| --- | --- | --- |
| **S-01 Checkout/scope** | Exact starting identity, Owner-installed ACTIVE, one verified checkout, only three permitted changed paths | STOP on drift/unauthorized edit |
| **S-02 Raw entry check** | `symlink_metadata` checks the original `.temp` entry for Windows reparse status or Unix symlink **before resolution or use**; directory/containment checks remain | FAIL if a redirected entry may be accepted |
| **S-03 Adversarial refusal** | Real junction/symlink whose resolved destination remains inside checkout is refused; validation preserves the target and existing material | FAIL/UNVERIFIED; no synthetic-only substitute |
| **S-04 Scratch ownership** | Genuine root, collision refusals, exclusive children, safe nonrecursive cleanup and zero test residue | FAIL on unsafe deletion/fallback |
| **S-05 No regressions** | fmt/lint/tests, actual CLI behavior, strict inward graph and pinned locked release build green; binary raw-hash reproducibility preserved | FAIL/UNVERIFIED on mandatory lost evidence |
| **S-06 Reporting/publication** | One factual canonical report, all permanent paths authorized, local tree clean, one final successful push and remote identity verified | FAIL on premature push, extra artifact or false result |

The full source-boundary checker belongs to `R01.B`; CI/browser to `R01.C`; MSVC, publication and product-stage checks remain not passed. Report those as `UNVERIFIED`, `NOT RUN` or `NOT APPLICABLE` with their reasons, not as passing tests. Reopen a previously reported `PASS` if the current run introduces contradictory evidence; do not bypass the A1 standard.

### 6. Local completion, canonical report and STOP

Complete source edits, regression tests, reproducible builds and evidence **locally**. Make local commits only for the permitted files, run final checks against the committed implementation identity, overwrite `.documents/Reports/execution-report.md`, and locally commit that report. Confirm all four WIP files remain present, ROADMAP still carries `R01.A`/`R01.B`/`R01.C`, NEXT remains neutral, ACTIVE retains this work order, no scratch residue or unauthorized path remains, and the working tree is clean.

Make **one final push** of the completed safe commits to the existing `origin/main`; verify its result under Architect SOP §6B. In the report include the work-order identity, starting and final source HEAD/tree, actual changed files, raw-versus-resolved `.temp` proof, real redirected-entry fixture creation/refusal and target-preservation evidence, positive test results, commands/exits, reproducibility hashes, environment limits, cleanup and factual `COMPLETE`, `STOPPED` or `FAILED` status. Do not embed the report commit's own SHA inside the report; provide final commit/URL in the chat handback. The agent does not self-award Architect `ACCEPT`.

STOP on an unsafe checkout/Owner file, unresolved reparse/symlink metadata, inability to safely create the real negative fixture, failed mandatory check, redirected-root acceptance, foreign-data damage, unapproved permanent change, nondeterministic binary output, or unsafe final publication. Preserve unknown content, give the exact blocker and smallest Owner decision, and publish a truthful STOPPED report at the end only when safe under the SOP. Do not clear ACTIVE, remove `R01.A` from ROADMAP, choose NEXT, merge, release or deploy; those are separate Owner-controlled transitions after independent review.
