# Foundation Solo — Active Work Order

## SOLO-WO-037 R02 — R01.C Security Gate Evidence Correction

**Status:** OWNER-ISSUED  
**Matching handoff:** `SOLO-WO-037 R02`  
**Repository:** `provelopment/provelopment-foundation-solo`, `main`, `origin`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Starting GitHub HEAD:** `d41ac2067a0dab6c2f7add0890cac14320b3a7f4`  
**Starting tree:** `0f9538666035e1d4dd6241e1e11662f13b313455`  
**Starting workflow blob:** `270891755eb1979c1db48db560b0126d1dbbd023`  
**Starting canonical report blob:** `b19dc0d1518847d1f7d62c4a25772e1adee88000`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Correct the network-provenance wording that blocked independent acceptance of `SOLO-WO-037 R01`. The security implementation and its independently observed CI checks passed on both hosts; this `R02` corrects inaccurate evidence text only. Do not treat this correction as acceptance or closure of R01.C or Stage 01.

The present workflow incorrectly states in **both** job summaries that “only three steps use the network” while separately naming the network-enabled `cargo fetch --locked`. On Ubuntu, the test-only `cargo generate-lockfile` in the `RUSTSEC-2026-0009` negative probe also has `CARGO_NET_OFFLINE: "false"` and may reach the registry. The workflow header additionally calls the security gate the “only other networked step”, and some comments imply that **all** Cargo commands after the product fetch are offline. These are inaccurate or incomplete descriptions of the actual behavior, not reasons to change behavior.

**Required textual correction:**

1. In `.github/workflows/rust-verify.yml`, update **only directly relevant comments and the Linux/Windows `Job summary` prose** so all descriptions agree: checkout, `rustup toolchain install`, the **network-enabled locked Cargo dependency fetch**, the security gate’s verified scanner download and RustSec data refresh, and the **Ubuntu-only disposable negative fixture's** `cargo generate-lockfile`/metadata resolution are network-capable operations. Keep a meaningful distinction between production-workspace Cargo dependency acquisition, the security/advisory data path, and the test-only vulnerability fixture. State that ordinary post-fetch **product** build/lint/test/check commands run with `CARGO_NET_OFFLINE=true` and `--locked` where applicable; do not misdescribe the exception for the deliberately isolated fixture. Avoid an unqualified count, “only network step”, “all other Cargo commands offline”, or claims of system-wide network denial by a Cargo environment variable.
2. Preserve all actual workflow operations byte-for-byte: `on` events/schedule, matrix and hosts, action commit pins, runtime/tool versions, `permissions`, `with`, environment values, `if` predicates, **every substantive shell/PowerShell invocation and exit-code assertion** (apart from the specifically authorized job-summary output strings), downloaded artifact digests, negative probe, advisory database freshness, security classes and existing Rust/architecture/reproducibility gates. Correct comments and summary text, not an executable step label or any functional field. Preserve accurate `Browser smoke: UNVERIFIED / NOT RUN` language. Do not mask or suppress warnings; identify them truthfully from actual new jobs.
3. Overwrite `.documents/Reports/execution-report.md` for the `R02` run, explicitly describing the before/after discrepancy, confirming that the corrected summaries make no exhaustive network-count claim, and distinguishing prior `R01` security evidence from gates **actually rerun** on `R02` and the final published HEAD. Report exact results, warnings (including whether GitHub produces version/deprecation warnings), network exceptions, version/provenance constraints, advisory freshness, and `NOT RUN`/`UNVERIFIED` limits. Do **not** claim the Owner's additional **latest-stable/prefer-LTS, no unresolved version/deprecation warnings, safe future upgrades** requirement has already been integrated into the permanent A1 policy: that is a separately scoped policy/maintenance change, not an executable part of this correction.

### 2. Permanent-path ledger

| Path | Allowed operation |
| --- | --- |
| `.github/workflows/rust-verify.yml` | **EDIT only** the factual comments and both job-summary prose described in §1; preserve executable semantics |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the sole canonical report with `R02` factual evidence |
| `.documents/WIP/workorder-active.md` | **STAGE/COMMIT** exactly this complete Owner-installed `R02` order; **do not edit or clear** it |

**Every other permanent path is read-only**, specifically `deny.toml`, Cargo manifests/lock, `rust-toolchain.toml`, `.cargo/config.toml`, Rust code/tests, `README.md`, `AGENTS.md`, About/Architecture, both SOPs, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, all other `.github/` files and the license. Do not add a work-order/report archive, workflow, component, fixture project or other WIP file. ROADMAP retains R01.C and Stage 01; NEXT remains its neutral single heading; OUTLOOK unchanged.

### 3. Preflight, execution and proof

1. Read `README.md`, `AGENTS.md`, the relevant Architecture and both SOPs, the four WIP files, current canonical report and workflow. Confirm correct one Solo checkout, root, branch, sole `origin`, HEAD/tree, matching Owner handoff, clean index except this Owner-installed ACTIVE replacement, no unexpected tracked/untracked work, and exactly four WIP files. Verify the starting workflow/report blobs above, plus `deny.toml` blob `3063e864c28be1e77787f43f1b6f561d543c8f4a`, `Cargo.lock` blob `05a02666894f4f0bf5528acf118a049e5edb853e`, NEXT `92fa562b1af0c54391851aa5c79deb1631127069`, ROADMAP `e6ea66cffa14d75598ce051b284f4ce12a16f67b`, and OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`. STOP on drift or unexplained changes.
2. Apply only text changes in §1. Audit the entire workflow diff and, where practical, parse both YAML revisions to prove all executable logic, invocations, failure assertions, and script bodies **outside the permitted literal summary-output edits** are identical (including the Linux negative fixture's explicit `CARGO_NET_OFFLINE: "false"` and its `cargo generate-lockfile`). Search the **entire** workflow for remaining false or exhaustive network-count/offline claims. Preserve every security policy and warning gate.
3. Commit and push **only** the authorized paths to `origin/main` under Architect SOP §6B. Verify the changed-workflow source revision triggers the **real** both-host GitHub Actions checks. Require all applicable steps to succeed and no version/deprecation/platform warning to go untriaged; check the security gate on both hosts, the Ubuntu advisory negative-refusal diagnostic and the existing architecture/tripwire/smoke/reproducibility tests. Report any host-inapplicable skips as such; a new failed mandatory step is a blocker, not a permissible skip. Record the actual current `cargo-deny` version and RustSec fetch revision in the run, not an earlier run's values.
4. Publish the updated canonical report and verify its exact GitHub bytes. The final report-only push triggers another Actions run: **independently verify Ubuntu and Windows at the final GitHub HEAD**, not just at the previous source revision, and provide final HEAD/tree and run/job links in the chat handback. The report cannot contain its own final commit SHA, which is supplied in the handback.
5. Record evidence for the authorized-path diff, workflow semantic identity, network accounting, CI results, warnings, local `NOT RUN`s, repository status, single checkout/worktree and task-owned scratch cleanup. Use only the existing Solo `.temp/` or Cline-provided task workspace for disposable work; preserve unknown or foreign material. Do not claim GitHub's platform-managed runner internals are audited or that a green advisory scan proves absence of all vulnerabilities.

**Acceptance conditions:** (a) the header, directly relevant comments and both summaries accurately name the network-capable operations and exceptions, with no false “only N steps” or “all other Cargo commands” claim; (b) workflow functionality and `deny.toml`/lock/pins are unchanged; (c) the canonical report is accurate; (d) the only permanent changes are in the three paths above; (e) both source-revision and final-report-HEAD GitHub Actions runs pass on both hosts, including the live advisory negative probe, with no untriaged consequential warning or missing mandatory step; (f) WIP remains four-file and unclosed. Record `PASS`, `FAIL` or `UNVERIFIED` for each, with independent GitHub evidence.

### 4. Permissions, exclusions and STOP

**Authorized:** read-only GitHub/upstream verification; edits only to factual workflow comments/job-summary text and canonical report; staging the unchanged installed `R02` order; task-owned disposable files under the authorized temporary-workspace rules; focused static diff/YAML checks and GitHub Actions verification; normal commit/push of the named files to the existing `origin/main` subject to preflight.

**Not authorized:** executable workflow changes, dependency/action/toolchain upgrades, scanner or security-policy edits, any Rust code/test edits, version-policy/SOP updates, WIP transition or closure, creating branches/PRs/clones/worktrees, GitHub settings, secret or provider actions, destructive Git operations, merge, release or deployment. The Owner's new A1 version-management requirement remains an explicit future policy/upgrade-management obligation; do not silently implement it through this `R02`.

**STOP** for baseline drift, non-authorized edits, missing approval/ID, failed or uninspectable CI, security finding, unresolved version/deprecation warning, inaccurate evidence, inability to read final remote HEAD or unsafe cleanup. Publish a factual `STOPPED`/`FAILED` canonical report when repository safety permits. Retain R01.C, Stage 01 and this ACTIVE order; the Implementation Agent never declares Architect `ACCEPT`.

### 5. Owner handback

Return the exact source and final commit/tree, canonical report link, both sets of Ubuntu/Windows run and job URLs, corrected wording summary, all mandatory results including the advisory-negative probe, warning/deprecation inventory, unchanged-security-policy/lock and path audit, clean status and scratch cleanup evidence. Leave ACTIVE as `SOLO-WO-037 R02`, NEXT neutral and ROADMAP R01.C open. Then report **“agent report ready”** for independent Chief Architect disposition.
