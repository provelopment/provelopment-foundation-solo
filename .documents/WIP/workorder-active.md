# Foundation Solo — Active Work Order

## SOLO-WO-040 R02 — A1 Version-Policy Evidence and Documentation Correction

**Status:** OWNER-ISSUED only when the Owner installs this complete file and gives the matching handoff.  
**Matching Owner handoff:** `SOLO-WO-040 R02`  
**Repository / branch / remote:** `provelopment/provelopment-foundation-solo` / `main` / `origin`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Required starting GitHub HEAD:** `19f1e9191d7e75fe775dff12917b053843571aa1`  
**Required starting tree:** `a3de01b30a11d5f80b664b8fff9998f29bbc3025`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Correct the bounded evidence and documentation defects identified by the independent Chief Architect's `REVISE` disposition on `SOLO-WO-040 R01`, without reopening or replacing the already implemented permanent A1 version-currency rule, Dependabot configuration, current version pins, or accepted security gates. The R01 run is **not** an Architect `ACCEPT`; this is an Owner-issued R02 correction within the same `SOLO-WO-040` objective. It corrects factual documentation, not the product or its dependencies. The Agent may not self-accept the order or consume any ROADMAP obligation.

**Exact permanent-path ledger — only these three paths:**

| Path | Authorized operation |
| --- | --- |
| `README.md` | **EDIT** only the stale factual passage under **“Installation, authoring and lifecycle”** that denies the actual engineering baseline. Preserve the surrounding product and future-feature contracts, links and structure. |
| `.documents/WIP/workorder-active.md` | **OWNER-INSTALLED ONLY:** stage, commit and publish the entire Owner-installed `SOLO-WO-040 R02` file **byte-for-byte unchanged** as issuance history. The Agent must not author, amend, clear, annotate or shorten its approved scope. |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the one canonical report with a new, truthful `SOLO-WO-040 R02` run report: accurate correction record, required classifications, change-scope evidence and actual CI evidence. Do not create a second report. |

**Every other permanent path is read-only.** In particular, do not change `.github/dependabot.yml`, `.github/workflows/rust-verify.yml`, either SOP, `.documents/Architecture/ARCHITECTURE.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, any crate manifest/source/test, `deny.toml`, `rust-toolchain.toml`, other `.github/` files, ROADMAP, NEXT, OUTLOOK, About, licence/notice files, GitHub settings or any other repository. No dependency, Rust, scanner, action, runner, build tool, product-contract version or lockfile upgrade; no extra lint/scanner suppression or new automated checks. The permanent A1 §7B(e2) and Implementation SOP §7B, accepted scanner fallback and weekly proposal-only Dependabot configuration remain intact. No release, merge, deployment, stage closure or WIP reconciliation.

### 2. Required factual corrections

**A. README contradiction — one bounded paragraph only.** The paragraph beginning “Solo's implementation technology is selected only conditionally” in `README.md` under “Installation, authoring and lifecycle” currently asserts: “There is no application code, installed toolchain, dependency manifest, test suite, CI job, release or deployment.” This is contradicted by the repository's four real Rust crates, workspace manifests and tracked `Cargo.lock`, pinned Rust 1.99.1 toolchain configuration, tests, and existing two-host `rust-verify` workflow. Replace the misleading absence claim with the **actual distinction**:

- The **engineering baseline exists and is verified**: the four-crate inward-only Rust workspace (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`), recorded exact toolchain pin, manifests/lockfile, executable checks/tests, Linux/Windows mandatory CI, and its verified security/architecture/reproducibility/CLI smoke evidence.
- The **future Solo EFS product/application, its JSON/Markdown publication validators, real website/browser/HTTP surface, installation/release/deployment and later §6A/§6C promises remain unimplemented/unverified**. Do not claim a globally installed toolchain on a user's machine merely because the repository pins it and hosted CI executes it.
- Keep the paragraph consistent with the accurate status near README's top and `AGENTS.md` §7. Preserve the accepted Entity/Facet/Site/Page documentation and all unrelated wording.

**B. Exact-version error in the report.** The R01 report's future, **selected-but-not-installed** browser test stack wrongly spelled `@axe-core/playwright` as `14.4.0`. The binding `.documents/Architecture/ARCHITECTURE.md` §6A.2 instead selects **`@axe-core/playwright =4.14.0`**. The R02 canonical report must record the previous error and the correct selected-but-not-installed `=4.14.0` *without* editing the Architecture pin, installing npm or claiming the current latest npm release was freshly verified unless it really was. Do not change version numbers in order to make them look newer. If any other selected stack row is repeated, derive it from the unchanged canonical Architecture text and label currency claims by evidence/date.

**C. Pull-request inventory error.** The R01 report incorrectly claimed “the repository has 0 pull requests in any state.” GitHub's public PR inventory at review showed **one historical closed, unmerged PR, #1**, authored before this work (2026-11-07, `SOLO-WO-019 R02`). The R02 report must distinguish that historical PR from **zero new Dependabot update PRs observed from the initial configuration-triggered jobs**; freshly query the actual PR inventory and bot authors before stating a count, date and disposition. Do not conflate “zero open update PRs” with “no PR ever existed,” and do not create, close, approve or merge a PR.

**D. Dependabot warning attribution and classification.** R01's warning paragraph asserted that both Dependabot jobs carried an `ubuntu-latest` migration notice. Independently retrieved **raw updater** logs for the two successful jobs (`128727737592`, Cargo; `128727737337`, GitHub Actions) instead contain these *non-fatal updater/runtime* messages:

- `rehash: warning: skipping ca-certificates.crt,it does not contain exactly one certificate or CRL` (certificate-store rehash inside the updater container);
- `Failed to parse GITHUB_REGISTRIES_PROXY environment variable` (updater diagnostic with an empty proxy environment value).

Neither message, on its own, proves an outdated installed Solo dependency or an actionable `rust-verify` version/deprecation warning. The R02 report must identify source, severity/classification, whether it affects Solo's source/CI/toolchain, and disposition without suppressing or hiding either message. Recheck the **actual** job logs/available annotations. The `ubuntu-latest` Ubuntu 26 migration may be discussed as a **separate upstream runner-image announcement/proposal** with an exact source, but do not claim it appeared in those jobs unless an actual annotation/log supports it; any unavailable annotation evidence is `UNVERIFIED`, not silently invented. Solo's own matrix explicitly pins `ubuntu-24.04` and `windows-2025`; no runner migration is approved by R02. Record whether any actual actionable version/deprecation warning remains, and distinguish a benign updater warning from a Solo package or runner defect.

All four corrections must be represented accurately in the newly overwritten report. The R01 report remains visible in Git history; do **not** rewrite that history, backdate evidence, treat an Agent assertion as independent verification, or report source or final tests as passed until run results exist.

### 3. Preflight and exact baseline

Before any write, read `README.md`, `AGENTS.md`, both SOPs (especially Architect §§6B, 7A, 7B(e1/e2), 8 and Implementation §§4, 5C, 7B, 9), canonical Architecture §6A.2, all four WIP files, the current published report, Dependabot YAML and `rust-verify` workflow. Verify the **single** Owner checkout, `main`, sole intended `origin`, branch and remote HEAD/tree, one worktree, no stash and no unrelated dirty or untracked Owner material. The **only expected local tracked difference** from the stated published baseline is this Owner-installed ACTIVE replacement. If HEAD/tree or any read-only file drifted, STOP without rebasing or guessing; retain all Owner data and publish a factual failed/stopped report where safe under SOP §7A.

**Baseline Git blobs at the required starting HEAD:**

| Path | Required original blob |
| --- | --- |
| `README.md` | `d17f19b8a7cc0be35eb0c0375e7e7bec57c2735e` |
| `.documents/Reports/execution-report.md` | `d9e2a2fdcb7b09df5f61073ced8fc2593f353b1e` |
| `.documents/WIP/workorder-active.md` (before Owner installation) | `dbbb2b4d88b3d8856c7f963c78f980db092a5ada` |
| `.documents/SOP/sop-architect.md` | `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` |
| `.documents/SOP/sop-implementation.md` | `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |
| `.documents/Architecture/ARCHITECTURE.md` | `4b050f6345db8730c5ca818cddd7129544ab8865` |
| `AGENTS.md` | `6eae1da8c721e13cea0c6689a1665e98fa5a9abb` |
| `.github/dependabot.yml` | `1874726fc2406575605b3c7ac467f45402fb7f13` |
| `.github/workflows/rust-verify.yml` | `5426c1f5da6a084a9fa41e70b2d85d8741578d08` |
| `Cargo.lock` | `05a02666894f4f0bf5528acf118a049e5edb853e` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.documents/WIP/plan-roadmap.md` | `e6ea66cffa14d75598ce051b284f4ce12a16f67b` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |

Check that `.documents/WIP/` contains **exactly four** files, with no WIP subdirectory. Verify the entire Owner-installed `SOLO-WO-040 R02` document remains intact, and record its byte count, Git blob and matching handoff. Do not edit that order after installation.

### 4. Verification and acceptance conditions

1. **Factual correction.** A minimal diff to the single README passage explicitly distinguishes verified engineering foundation from the as-yet-unimplemented EFS product and avoids claiming the developer's local toolchain is installed. No stale global denial of existing manifests, tests or CI remains in the modified passage. The unchanged `AGENTS.md` current-state entry and repository tree agree.
2. **Evidence/report precision.** The new canonical `R02` report (a) records `@axe-core/playwright =4.14.0` as the Architecture-selected, not-installed value; (b) states the actual historical PR #1 and true zero/new Dependabot-PR observation with a fresh query; (c) classifies the actual certificate-rehash and proxy-variable Dependabot diagnostics; and (d) treats `ubuntu-latest` migration as an upstream notice unless personally observed in the specified job annotations/logs. Do not claim false `PASS`, fabricated tests or present-day npm version freshness based solely on an uninstalled architectural pin.
3. **No change to gates or policy.** The A1 §7B(e2) governance, both SOPs, Architecture exact pins, Dependabot config, `rust-verify`, action SHA, Rust toolchain, locked Cargo graph, full component security policy and first-party source remain **byte-identical**. Inspect actual changed paths at source and final HEAD. The only permanent changes from starting HEAD are README, Owner-installed ACTIVE and one overwritten report. No regressions, bypasses, masked warnings, GitHub setting changes or new PRs are introduced by this task.
4. **Exact CI evidence.** At the new source commit and again at the final **report-only HEAD**, inspect the actual completed GitHub `rust-verify` jobs on both `ubuntu-24.04` and `windows-2025`; all host-applicable mandatory steps must pass, including the live verified `cargo-deny` binary, fresh RustSec fetch, all four advisory/bans/licences/sources checks, strict Rust build/lint/tests, architecture/refusal and smoke checks, and Ubuntu `RUSTSEC-2026-0009` negative probe and byte-identical release builds. Read job logs and warning annotations when available. Fail/STOP honestly on any missing/red mandatory gate, version-support/deprecation issue or material untriaged warning; no repeated report-only commits to manufacture success. Browser/HTTP evidence is `NOT RUN` because no authorized application/browser surface exists.
5. **Repository hygiene and WIP.** Commit/push only permitted paths, verify final remote HEAD/tree and clean local status. ROADMAP retains **R01.C** and **Stage 01** open; NEXT stays neutral, OUTLOOK unchanged; four WIP files only; installed ACTIVE unmodified. Remove only task-owned scratch in approved temporary locations, preserve all unknown or foreign data. No closure or reconciliation is authorized in this order.

The Agent's normal report cannot contain its own final commit SHA, which must be given in chat after pushing. Describe the source-vs-final CI jobs independently and disclose unavailable details as `UNVERIFIED` or `NOT RUN`, rather than inheriting the prior run's claims. Where the original R01 version ledger is cited, preserve its **November 8, 2026** observation date and separate that historical check from newly researched facts.

### 5. Permissions, stop rules and handback

**Permitted:** read the intended Solo checkout and public GitHub/upstream evidence; edit only `README.md`; stage and commit the complete Owner-installed ACTIVE document unchanged; overwrite the single canonical report; perform read-only verification, safe task-local scratch work and cleanup; commit and push only the three authorized paths to the existing `origin/main`, provided repository safety and GitHub permissions permit. Normal Git mechanics are delegated per Architect SOP §6B, not to the Owner. No dependency installation, provider changes, secrets, alternate checkout, worktree, branch/PR creation, merge, force-push, release or deployment. No editing other repositories, no policy/feature expansion and no WIP closure. Dependabot's native proposal behavior is existing configuration, not new authority from this order.

**STOP/escalate** on mismatch of order ID/edition or Owner handoff, HEAD/tree/blob drift, unexplained dirty material, inability to accurately verify a material statement, unexpected changes beyond the ledger, an upstream warning requiring a functional update, or failure to execute mandatory CI. Preserve everything and publish a truthful `FAILED`/`STOPPED` report when safe; do not edit forbidden paths to repair a symptom.

**Handback:** exact `SOLO-WO-040 R02` ID; Owner-installed ACTIVE blob; source commit/tree and final published HEAD/tree; three-path diff, four factual corrections, live PR inventory, raw Dependabot warning classification, source and final Linux/Windows CI run/job links with actual pass/fail/skips, current advisory DB evidence, unrun evidence, and final cleanup/working-tree facts. End with **“agent report ready”** for the Chief Architect's independent disposition. Neither Agent nor order declares `ACCEPT` or closes Stage 01.

### 6. Exact Owner handoff (copy separately)

> Align with README.md and AGENTS.md, then execute work order SOLO-WO-040 R02 at .documents\WIP\workorder-active.md.
