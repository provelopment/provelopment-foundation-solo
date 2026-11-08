# Foundation Solo — Active Work Order

## SOLO-WO-034 R01 — Accepted R01.B Administrative Closure and R01.C NEXT Selection

## Approved Scope — OWNER-ISSUED

### 1. Objective, acceptance basis and immutable starting identity

Perform the **administrative closure only** of the independently accepted Stage 01 **R01.B — Hexagonal Source-Boundary Enforcement**, then choose the earliest remaining dependency-ready ROADMAP package, **R01.C**, as the single NEXT candidate. Remove exactly the accepted R01.B ROADMAP obligation; preserve the still-open Stage 01 gate and all remaining work; replace the executed ACTIVE order with the neutral ACTIVE heading. This is **not** R01.C implementation and **not** Stage 01 acceptance.

**Chief Architect disposition:** `ACCEPT` for `SOLO-WO-033 R01` at the published GitHub revision `36a88be5c4a8eb1337cd66fc12cc09456920c9d0`. Together with the previous inspected R01.B implementation and bounded corrections (`SOLO-WO-030` through `SOLO-WO-033`), that disposition authorizes proposing closure **only after** the Owner installs this complete order and explicitly hands off matching `SOLO-WO-034 R01`. The agent must not independently grant acceptance, infer closure authority from a report, or rewrite the past. Accepted evidence is historical and remains in Git; do not insert completion commentary into WIP.

| Baseline identity | Exact value |
| --- | --- |
| Sole target repository / branch | `provelopment/provelopment-foundation-solo` / existing `main` |
| Starting GitHub HEAD | `36a88be5c4a8eb1337cd66fc12cc09456920c9d0` |
| Starting GitHub tree | `0ec9166d7c0bf5196c8d645338eed2e3e18cee8d` |
| Accepted final R01.B source correction | `c34e323ca0b7579430cdb259e4260989c7d1edbb` / tree `addc3cb1f5f43aa5bc9d1c5abc4cf3dc6802cea2` |
| Final published R01.B report | `.documents/Reports/execution-report.md`: blob `7852156e16e898dec765e4dceca8bf30eec3624a` |
| Previous committed ACTIVE | `.documents/WIP/workorder-active.md`: blob `272ef3fca5c812b2839c682486aaa962b2e8e60c` (`SOLO-WO-033 R01`) |
| ROADMAP | `.documents/WIP/plan-roadmap.md`: blob `d01780d4af965b342521e3594f7a1e4a7dfc551d` |
| NEXT | `.documents/WIP/plan-next.md`: blob `92fa562b1af0c54391851aa5c79deb1631127069` (neutral, 49 bytes) |
| OUTLOOK | `.documents/WIP/plan-outlook.md`: blob `cb9c7a610cf3c537557e060ca9656e44a053e69d` (neutral, 52 bytes) |
| Checker source / real fixture tests | `crates/solo-cli/tests/architecture_checks/source.rs` `92f6e0e3dea70857cd2ce242741c2c32ed26eed4`; `crates/solo-cli/tests/check_architecture_fixtures.rs` `93fe5733dbc1379e6b29f142275b77293022c0ec` |
| Checker shared module / graph | `crates/solo-cli/tests/architecture_checks/mod.rs` `89dfea91bd82a9b5623f6d437ec787f42fe4aa3b`; `graph.rs` `e328dd6a9f54c47e3eae60053446d52edacc74b7` |
| Toolchain/locked graph | `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`; `Cargo.lock` `05a02666894f4f0bf5528acf118a049e5edb853e` |

Read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, both SOPs, all four WIP files and the **one** canonical execution report in their required order, followed by only the relevant accepted R01.B source/test evidence. Binding: Architect SOP §§6, 6A, 6B, 7, 7A, 7B, 8; Implementation SOP §§4–5C, 8–10. This order grants **no** independent policy-making permission and does not supersede their safety, Git or file-purity rules.

### 2. Authority, repository, safety and parity preflight

Before any **write**, install, package fetch or source execution:

1. Verify the one **existing** Solo checkout (Owner-local location previously reported as `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, but derive the actual location from Git). Confirm `origin` is the intended Solo GitHub repository, branch `main`, exactly the HEAD and tree above, remote tracking, one worktree, no hidden second checkout/branch worktree, stash, unresolved conflict or unsafe link/reparse path. Inventory tracked/staged/unstaged/untracked/ignored files without deleting, resetting, cleaning or altering anything. Never write to `provelopment-root`, `provelopment-foundation`, `provelopment-web-1` or any other repository.
2. Confirm the **only** expected pre-existing tracked change from that baseline is the Owner's installation of the **complete** `SOLO-WO-034 R01` text into `.documents/WIP/workorder-active.md`; confirm `OWNER-ISSUED` and the matching Owner handoff. Preserve the Owner-installed file **byte for byte** in the initial local preservation commit. Do not proceed on another order, unidentified worktree dirt, altered report/ROADMAP/NEXT/OUTLOOK, or unexpected remote HEAD.
3. Verify WIP holds **exactly four files and no directory**: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`. Baseline ROADMAP contains **one** R01.B bullet and **one** R01.C bullet, no R01.A, unchanged Stage 01 prerequisite/exit gate, and all other stages; NEXT has only `# Foundation Solo — plan-next (Next Candidate)\n`; OUTLOOK has only its existing heading. No second order or candidate is active.
4. Verify the blobs named in §1, plus read-only identities: `README.md` `3f3005d680d9ed182a02e574744054aaae55deb2`; `AGENTS.md` `32562b6730e26a37b9febae63efd7d5c6a5866ac`; About `1d80f62769053ba4e6ca51adf2d11c68e724c30e`; Architecture `03e8923f834b50fc28e1b56e70f73017525cfa5b`; Architect SOP `8760be323b6ed85695a9dcffd3e52f94628f0f07`; Implementation SOP `a577116082bf8133a6056667aa52fe4ffc2ae94d`; root `Cargo.toml` `14505be49a281ba0495eabea5ca386548cc36347`; `crates/solo-cli/Cargo.toml` `1ca5e114a6e034aae386ea9d7adab3a81b4ae8bc`; `.cargo/config.toml` `74a4fb6c0f595ebc64841b26e7961967294f7d3e`; `crates/solo-cli/tests/check_architecture.rs` `0bad0e027f41b6cf6029df38174fc15531c187da`; `workspace_graph.rs` `8fa4b4d0bde840782abb0c9e24c84bac8f06c947`; `cli_smoke.rs` `921cbbe91fb0a75919014e91455fbc080dfd111c`; production `crates/solo-cli/src/main.rs` `6faa9fae29351123a49d45c5637ac4fbb1b615c8`.
5. Perform an **observational architecture/product-document parity check**, not a rewrite: the accepted four-crate inward-only Rust/hexagonal structure, the single CLI composition root and BootstrapPreflight vertical slice, pinned Rust/Cargo/dependency rules, the real Cargo graph and fail-closed `syn` source-boundary enforcement and its negative fixtures are consistent with the extant Architecture §§5A, 6A.2–6A.4 and About. R01.B adds **no new Owner-approved product contract** requiring a separate authority change. If a meaningful accepted durable rule is missing or contradicted, **STOP** and request a separately specified documentation correction; do not silently add a fifth permanent path. Do not turn historical test observations into architecture doctrine.
6. Inspect the already disclosed **foreign scratch**, including `.temp/rt` and `.temp/w032` if present. They are **not** part of this closure and must be preserved; do not delete them or remove shared `.temp`. Any current task scratch must be positively owned and cleaned under SOP §6A/§5A, with a truthful disposition.

### 3. Exact administrative transformation — no implementation

**A. Preserve the issued order in Git history.** After verifying §2 and while this full Owner-installed order is the **only** expected tracked change, stage that file alone and create a **local-only** preservation commit (no intermediate push). Its exact blob must be the Owner-installed bytes. Keep the complete issued order in Git history. This is the same accountable closure mechanism used for accepted R01.A (`SOLO-WO-029 R01`). No other file may enter that preservation commit.

**B. ROADMAP — remove exactly one accepted R01.B bullet.** In `.documents/WIP/plan-roadmap.md`, delete the **single entire line**, including its line terminator, that is exactly:

```text
- **R01.B:** introduce domain/application/port/adapter boundaries and one composition root, with explicit import-boundary enforcement including aliases and re-exports; ports must serve actual use cases rather than decorative abstractions.
```

Do not rewrite, reflow, rename or reorder anything else. Verify that removing this one line from the baseline byte stream reproduces the new ROADMAP **byte for byte**. Preserve R01.C, the Stage 01 heading/prerequisite/exit gate, every later stage, Markdown spacing and LF policy unchanged. Do **not** mark Stage 01 accepted or add a completed R01.B narrative: the pending R01.C package and Stage 01 exit gate remain.

**C. NEXT — select one dependency-ready R01.C candidate, not an order.** Replace the neutral NEXT with **exactly** the following UTF-8, BOM-free, LF-only block, with one final LF, no extra commentary:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## Stage 01 — R01.C: Verification Infrastructure and CI

**Objective:** Establish the remaining Stage 01 unit, integration, architecture, production-mode and browser-smoke verification infrastructure plus reproducible CI, with failures observable rather than hidden, for the existing Foundation Solo Rust skeleton. Do not treat readiness for a browser smoke test as authority to create a browser product.

**Prerequisites and readiness:** Stage 00 architecture/toolchain choices and Stage 01 R01.A are accepted. Stage 01 R01.B was independently accepted at GitHub revision `36a88be5c4a8eb1337cd66fc12cc09456920c9d0`. The Rust 2024 four-crate workspace (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`), pinned `rustc 1.99.1`, shared `Cargo.lock`, real `BootstrapPreflightV1` CLI-to-application vertical slice, inward-only Cargo graph checker and fail-closed `syn =3.0.9` source checker already exist. Agent-reported GNU host regressions include 107 passing tests and two raw-identical release builds; those are historical, not newly rerun CI evidence. CI and browser-smoke infrastructure have not yet been implemented or accepted.

**Candidate scope and deliverables:** Design a bounded Rust build/test CI workflow and corresponding local reproducibility checks, exercising the existing unit, integration, real CLI, production-mode release-binary and architecture refusal gates with locked dependencies. Add negative/mutation evidence that the CI/test gate fails on a deliberately forbidden edge, failed test or bypass instead of reporting false success; preserve the sole CLI composition root, current port/adapter boundary and safe scratch cleanup without a hidden compatibility runtime. Specify honest browser smoke infrastructure/readiness for a future actual browser entry point; do not fabricate browser evidence, launch an HTTP service or build a UI solely to create a smoke target. Include explicit supported-host coverage and exact non-run limitations. This is a planning candidate only: every implementation path and external CI operation requires a new separately Owner-issued order.

**Verifiable exit conditions:** A fresh clean-room checkout can obtain the exact approved locked toolchain/dependencies and reproduce format, strict lint, unit/integration/architecture, source-negative and production CLI smoke gates through CI; meaningful failure injection produces a failing CI status. No suppressed warnings, ignored test failures, undisclosed network dependency, phantom browser success, second runtime or unsafe cleanup. Browser-specific tests remain explicitly unverified until a genuine authorized browser surface exists. R01.C and the Stage 01 exit gate remain open pending independent acceptance; no Stage 02+, provider, deployment, release or public distribution work.
```

This text is **non-executable** by design. It identifies the next work package without granting permission to install CI, modify `.github/`, build browser features, provision accounts or run any future stage. No separate R01.C work order may be issued or executed as part of this closure run.

**D. ACTIVE — neutralize after preservation.** Replace `.documents/WIP/workorder-active.md` with **exactly** this one-line, 40-byte UTF-8 LF-terminated text (no BOM, no blank second line):

```text
# Foundation Solo — Active Work Order
```

The accepted R01.B history remains recoverable from the earlier local preservation commit and prior published commits, not from a live WIP slot. No `SOLO-WO-033`, `SOLO-WO-034`, execution status, Architect disposition, placeholder or justification remains in the neutral ACTIVE file. Its emptiness means no current implementation authority, **not** a project pause.

**E. OUTLOOK and durable contracts.** Leave `.documents/WIP/plan-outlook.md`, About, Architecture, SOPs, README, AGENTS, production source, Cargo configuration and all other files completely unchanged. Record the parity check in the canonical report; if the governing contract is actually inconsistent, **STOP**, do not expand the edit scope by interpretation.

### 4. Exact permitted permanent paths and Git/operational authority

| Path | Operation allowed |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Preserve Owner-installed complete `SOLO-WO-034 R01` byte-for-byte in the first local commit, then replace it with the exact neutral heading in the closure commit |
| `.documents/WIP/plan-roadmap.md` | Delete **only** the one exact R01.B bullet in §3B; byte-identical remainder required |
| `.documents/WIP/plan-next.md` | Replace the neutral candidate slot with the exact R01.C block in §3C, with no other candidate |
| `.documents/Reports/execution-report.md` | Overwrite exactly the one canonical report with factual closure/validation/publication evidence |

Every other permanent path is **read-only**, including OUTLOOK, About, Architecture, both SOPs, all Rust files/tests, Cargo manifests/lock, `.cargo/`, `rust-toolchain.toml`, README/AGENTS, other repositories, CI/browser/security/licensing and `.gitignore`. No extra WIP files, archived orders, report copies, product edits, CI setup or changes to unrelated stages.

**Expressly permitted** after the matching Owner handoff: read-only verification; the **three bounded local commits** described in §§3A and 5 (issued-order preservation, WIP reconciliation, canonical report); staging only authorized files; overwriting the single report; use of positively owned disposable Solo `.temp/` or actual current Cline-provided task scratch for safe byte/hash comparisons with cleanup; one **final** ordinary push to the already configured `origin/main` and remote readback under Architect SOP §6B. More than three local commits are permitted **only** if §6B safety requires them and every committed path still matches this ledger; explain any variance. Do not conduct an intermediate push.

**Not permitted:** build/toolchain installation or fetch, new packages/dependencies, product/source/test modification, CI/workflow write, provider or credential operations, second clone/worktree/checkout, branch/PR/merge, force push, reset/clean/rebase/history rewrite, deletion of unrelated or foreign scratch, broad file formatting, any release/deployment, or R01.C implementation. Documentation closure does **not** require pretending to rerun previously accepted Rust tests. Local read-only Git/diff/hash checks are expected; all code-bearing checks are `NOT RUN` with a scope reason.

### 5. Validation gates, canonical report and final publication

After the local preservation commit, perform only §3B–E, locally commit those WIP reconciliation changes, and run byte-level checks against the immutable starting Git blobs. Overwrite/commit the **single** canonical report; verify exact combined diff and working-tree safety, then push once and verify GitHub HEAD/tree. Follow the SOP's single Git procedure rather than inventing an alternate sequence. A safe report-only `STOPPED` publication may be appropriate under §7A; do not publish unauthorized or unsafe incomplete closure.

| Gate | Mandatory evidence for PASS |
| --- | --- |
| **J-01 — Accepted identity and authority** | Published accepted revision/head/tree and R01.B `ACCEPT`, Owner-issued ID-matched full ACTIVE, one clean Solo checkout apart from Owner installation, no unexpected changed paths or foreign-data risk |
| **J-02 — ROADMAP exact reduction** | Original R01.B line occurs once and is removed once; old ROADMAP minus that line equals new ROADMAP bytes; R01.C, Stage 01 gate and all later stage bytes unchanged |
| **J-03 — NEXT readiness and purity** | NEXT matches §3C **byte for byte**, contains exactly one R01.C candidate and no execution authority, R01.B no longer listed as pending NEXT |
| **J-04 — ACTIVE/WIP closure** | Preservation commit contains Owner-installed full order unchanged; final ACTIVE is exact 40-byte neutral heading; exactly four WIP files, no subdirectory; OUTLOOK blob unchanged |
| **J-05 — Parity and no implementation drift** | Architecture/About/SOP parity checked and unchanged; only four permitted documentation paths differ from baseline; code/Cargo/lock/toolchain/checker identities identical; Rust/CI/browser checks honestly marked NOT RUN, not passed by reuse |
| **J-06 — Report, scratch and publication** | One report with exact hashes and factual gate observations; owned temp cleaned or precise unresolved blocker; foreign scratch preserved; clean final checkout; one final push and matching remote `main` HEAD/tree verified |

**Report content** at `.documents/Reports/execution-report.md`: ISO date and `SOLO-WO-034 R01`, Owner handoff and actual root/branch, baseline HEAD/tree and accepted R01.B review identity, exact Owner-installed ACTIVE blob and local preservation commit, byte-exact ROADMAP removed line, NEXT match, ACTIVE neutral-byte proof, WIP inventory and OUTLOOK unchanged, parity audit, changed-path and unchanged-blob ledger, local Git and verification commands/results, status `COMPLETE`/`STOPPED`/`FAILED`, scratch inventory/preservation/cleanup, one-final-push and remote identity verification, explicit remaining R01.C/Stage 01 obligations and implementation exclusions. The report must **not** contain its own final report-commit SHA, private absolute paths, secrets or credentials; return that SHA and GitHub report link only in the agent chat handback. No second report, history paragraph in a WIP file or completed-stage badge.

Report every applicable gate as `PASS`, `FAIL`, `UNVERIFIED` or `NOT APPLICABLE` with reason. Do not re-run Rust builds merely to inflate closure evidence; do not import agent-reported past GNU checks as freshly run results. MSVC, Unix, security advisory scanning and browser smoke remain historical `UNVERIFIED`/`NOT RUN` where relevant, and **R01.C plus Stage 01 exit gate remain unfinished**.

**Final sequence:** safe preflight → initial local preservation commit of the installed ACTIVE → minimal ROADMAP/NEXT/ACTIVE reconciliation → local reconciliation commit → exact byte/blob, four-file WIP and parity checks → canonical report overwrite and commit → final diff/clean-tree/scratch audit → **one final push** to `origin/main` → verify published SHA/tree and report. Do not clear, accept or implement R01.C in this run.

### 6. STOP conditions

STOP without unrelated edits or destructive commands on mismatched Owner order, HEAD/tree drift, second checkout, non-Owner dirty changes, wrong/duplicate ROADMAP bullet, unexpected WIP file or active candidate, unverifiable byte preservation, changed OUTLOOK, substantive architecture/About contradiction, missing accepted authority, unowned scratch/redirect risk, changed read-only source/Cargo/config/SOP files, unsafe Git publication or absent permissions. Preserve all unknown and Owner-owned material. When safe, publish a truthful `STOPPED` report under the standing single-report rule; otherwise give the exact blocker and the minimum Owner decision needed. Never manufacture a green closure or claim a release, Stage 01 acceptance or next-order authority.
