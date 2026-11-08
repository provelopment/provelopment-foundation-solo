# Foundation Solo — Active Work Order

## SOLO-WO-035 R02 — R01.C CI Evidence Correction

**Status:** OWNER-ISSUED  
**Matching Owner handoff:** `SOLO-WO-035 R02`  
**Repository:** `provelopment/provelopment-foundation-solo` (`origin`)  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Target branch:** `main`  
**Starting HEAD:** `695c4b11f02ab5b08335ca32c044b143ed0bd328`  
**Starting tree:** `196fc4708941457397983e4ecc27a0bbcf6f9b7c`  
**Reviewed workflow blob:** `02a1fa7e0e378e6b2fa2132271fc57e81fbbf6cb`  
**Reviewed report blob:** `0a522f5e52ede59c4c741ea688a457f177ab99f8`

### 1. Approved Scope — OWNER-ISSUED

**Objective:** Correct two factual defects in the R01.C CI evidence wording found by the Chief Architect's `REVISE` review. This is revision `R02` of `SOLO-WO-035`, not new implementation scope. The already verified CI gate logic and product behavior must remain unchanged.

1. **Fix the Linux adversarial-test summary.** In `.github/workflows/rust-verify.yml`, change the Linux job-summary description of the **injected real domain-to-adapter import** from `layer-outward-reference` to **`undeclared-crate`**. The checker actually emits `error: undeclared-crate: ... solo_adapters is not a declared dependency of solo-domain`, exit **1**; GitHub Actions run `42449851656`, Ubuntu job `128636524033`, independently confirms it. Do **not** change the checker or the assertions: the separate `direct-outward`, `renamed-alias`, `reexport-chain` and `checker-bypass` fixtures still correctly require `layer-outward-reference`; `dynamic-load` still requires `dynamic-loading`.
2. **State the actual network boundary.** In the same workflow, correct the Linux and Windows job-summary claims that `cargo fetch --locked` is the *only network step*. It is the only **network-enabled Cargo dependency acquisition**; `actions/checkout` and `rustup toolchain install` can also access the network. After that fetch, Cargo dependency resolution/build/test is offline under `CARGO_NET_OFFLINE=true` and the existing locked-command discipline. Correct directly related misleading workflow comments or step labels, including the phrase `Default-deny network`, which describes a Cargo setting as a system-wide network restriction. Do not change actual environment variables, commands, action pins, permissions, triggers, matrix, test conditions, or failure-propagation behavior.
3. **Publish accurate evidence.** Overwrite `.documents/Reports/execution-report.md` in place for this `R02` run. Explain the two precise text corrections and their verified basis; remove or qualify the report's existing assertions that `cargo fetch --locked` is the only step reaching the network. Distinguish previously observed `R01` execution from checks **actually rerun** under `R02`; record exact revisions, workflow run links, both job conclusions, relevant job steps, observed warnings and non-runs. The report must not claim an independent Architect `ACCEPT`, Stage 01 closure or release.

### 2. Permanent-path ledger

| Path | Authorized operation |
| --- | --- |
| `.github/workflows/rust-verify.yml` | **EDIT only inaccurate evidence prose** described in §1; preserve all functional workflow semantics |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the single canonical factual report for `R02` |
| `.documents/WIP/workorder-active.md` | **STAGE/COMMIT exactly this Owner-installed `R02` text**; the Implementation Agent must not edit it |

**All other permanent paths are read-only.** In particular, leave `Cargo.toml`, `Cargo.lock`, Rust source/tests, `rust-toolchain.toml`, `.cargo/config.toml`, `README.md`, `AGENTS.md`, Architecture/About/SOPs, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, other `.github/` files and security policy unchanged. NEXT remains its neutral single heading; ROADMAP retains R01.C and the Stage 01 gate. Do not add a second report, workflow, WIP file, dependency, test or runtime.

### 3. Execution and required evidence

1. Read `README.md`, `AGENTS.md` and the relevant Solo SOPs. Verify the actual Git root, `main`, sole intended `origin`, one checkout/worktree, starting HEAD/tree and listed blobs. At preflight, the only expected local tracked difference is the Owner's replacement of ACTIVE with **this complete `SOLO-WO-035 R02`**; investigate and **STOP** on any other unexplained difference or revision drift. Do not rewrite the installed ACTIVE file.
2. Apply only §1's textual corrections. Review the complete diff to prove the workflow's executable gates, negative-test categories, shell code, security boundaries and event triggers are functionally identical to the verified `R01` implementation. Check that neither workflow job summary nor the canonical report still makes either false claim.
3. Commit and push only the authorized paths to the existing `origin/main` under Architect SOP §6B. Confirm GitHub Actions actually evaluates the changed workflow. Verify **both** `ubuntu-24.04` and `windows-2025` jobs succeed for the resulting source revision and all applicable required steps ran; any genuine failure is a blocker, not a permitted skip. Confirm the Linux `undeclared-crate` injection outcome and retained expected fixture categories in real job logs/annotations when accessible. Record actual limits if a log or summary rendering is inaccessible; never infer its contents from a green result alone.
4. Publish the canonical `R02` report and read it back from GitHub. A report-only final push also triggers CI: verify both jobs at the **final published HEAD** and distinguish that revision from the preceding corrected-workflow revision. Include final commit/tree in the handback (do not invent the report's own commit SHA inside the report).
5. Record the actual local checks run, GitHub job results, textual diff, unchanged-source/lock/WIP checks, warnings, runner/provenance limits, and task-owned scratch cleanup. Preserve foreign temporary material. Keep the browser/HTTP/Node/Playwright gates `UNVERIFIED / NOT RUN`, because the product surface remains unimplemented.

**Acceptance:** the `undeclared-crate` injected-import summary agrees with the actual checker; both host summaries accurately delimit Cargo-only offline operation without falsely denying checkout/toolchain network; the report is consistent; no functional code/CI behavior or unauthorized path changes; both hosts pass the corrected workflow and final report commit; no hidden failed or skipped mandatory gate. If any condition cannot be verified, record `FAILED`, `STOPPED` or `UNVERIFIED` honestly and do not claim `COMPLETE`.

### 4. Permissions and exclusions

**Authorized:** edit only the workflow's evidence wording, overwrite the one report, stage the Owner-installed ACTIVE unchanged, use the existing Solo checkout and permitted task-owned `.temp/` or Cline scratch, run relevant read-only inspections/verification and the existing pinned CI, commit and push the named paths to `origin/main`, and inspect GitHub Actions results. Ordinary Git mechanics follow Architect SOP §6B; temporary-workspace governance follows §6A.

**Not authorized:** edits to production/test Rust, checker logic, Cargo manifests/lock, workflow executable commands, network/security permissions, branch or provider settings, other repositories, architecture or WIP plans; additional dependencies/tools, clones/worktrees/branches/PRs, secret use, destructive Git operations, history rewriting, merge, release or deployment. No automatic R01.C/Stage 01 reconciliation or ACTIVE clearing.

**STOP** for an unexpected baseline or dirty state; a needed change outside the ledger; altered CI behavior or a missing/failed mandatory job; unsupported evidence or inability to inspect the final published revision; unsafe temporary cleanup; or a policy/credential blocker. Preserve state and report the specific issue. Do not broaden this correction into a new task.

### 5. Owner handback

Return the exact `R02` source and final report commit SHAs, final tree, report link, both final-HEAD GitHub Actions run/job URLs and conclusions, allowed-path audit, test/summary evidence, warnings and any limitations. Keep `SOLO-WO-035 R02` ACTIVE, NEXT neutral, ROADMAP R01.C open and OUTLOOK unchanged. The next handoff to the Chief Architect is **“agent report ready”** for a new independent `ACCEPT / REVISE / STOP` review.
