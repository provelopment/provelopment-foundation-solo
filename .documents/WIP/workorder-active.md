# Foundation Solo — Active Work Order

## SOLO-WO-029 R01 — R01.A Accepted Closure and R01.B NEXT Selection

## Approved Scope — OWNER-ISSUED

### 1. Objective

Perform **only the administrative WIP reconciliation** for independently accepted Stage 01 package `R01.A`: remove its completed obligation from ROADMAP, clear ACTIVE, and select the dependency-ready `R01.B` package as the **one** NEXT candidate. Overwrite the one canonical execution report with the actual reconciliation evidence. This is **not** authority to execute `R01.B`, `R01.C` or any product implementation.

The Chief Architect returned **`ACCEPT`** for `SOLO-WO-028 R01` against GitHub revision `ff55dc702587cc8bfef98c8c8a3d0665ccfc162b`. The accepted implementation source commit is `5ae69bb9f396bb8bd33a62501e620f062d78a79c`. Preserve the accepted code, pinned toolchain, lockfile, quality evidence, and all deferred verification limits. This work order closes **R01.A only**; Stage 01 remains in ROADMAP because `R01.B`, `R01.C` and the stage exit gate remain incomplete.

**Repository:** `provelopment/provelopment-foundation-solo`, existing local checkout on `main`.

**Reviewed starting HEAD:** `ff55dc702587cc8bfef98c8c8a3d0665ccfc162b`.

**Reviewed starting tree:** `1e090cdc2d5fb07474af2bdbd4d37e03dd9e7194`.

**Owner-local root expected:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; determine and verify the actual accessible root, not merely this expected spelling.

Read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, both SOPs, all four WIP files, and `.documents/Reports/execution-report.md` in the mandatory order established by `AGENTS.md`. In particular apply Architect SOP §§6, 6A, 6B, 7A, 7B, 8 and Implementation SOP §§4–5C. Do not restate or override the authoritative Git procedure in Architect SOP §6B.

### 2. Exact local preflight

Before **any** repository write:

- Verify the one existing Solo checkout: actual Git root, branch `main`, `HEAD` and tree equal to the identities above; only one worktree and sole correct `origin`; working tree, staged/unstaged/untracked/ignored contents, stashes, and symlink/reparse safety. No second clone, worktree, branch, nested repository, or modification of a sibling Provelopment repository.
- The **only** permitted pre-existing tracked difference is the Owner's complete installation of this `SOLO-WO-029 R01` in `.documents/WIP/workorder-active.md` in place of `SOLO-WO-028 R01`. No unrelated dirty file, unknown Owner material or divergent remote state may be silently discarded, staged or overwritten. Confirm the installed ACTIVE identifies exactly this work order and the matching Owner handoff.
- Verify `.documents/WIP/` contains **exactly** `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`, with no subdirectories/extra files. ROADMAP contains `R01.A`, `R01.B`, `R01.C` under Stage 01; NEXT is heading-only; OUTLOOK is heading-only; ACTIVE contains the previous accepted `SOLO-WO-028 R01` in the starting commit.
- Verify these reviewed Git blob identities from the starting commit:

| Canonical file | Starting blob SHA |
| --- | --- |
| `README.md` | `3f3005d680d9ed182a02e574744054aaae55deb2` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `.documents/About/FOUNDATION-SOLO.md` | `1d80f62769053ba4e6ca51adf2d11c68e724c30e` |
| `.documents/Architecture/ARCHITECTURE.md` | `03e8923f834b50fc28e1b56e70f73017525cfa5b` |
| `.documents/SOP/sop-architect.md` | `8760be323b6ed85695a9dcffd3e52f94628f0f07` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/plan-roadmap.md` | `4f46dfcc4fcb699a491f6df0847b902eb805df78` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/workorder-active.md` (before Owner replacement) | `dd9e99858d1ddf4bb262ce4f7775773e8590de57` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `693bdca78ba9b2a715de88b9167b606e26b212ee` |
| `Cargo.toml` | `14505be49a281ba0495eabea5ca386548cc36347` |
| `Cargo.lock` | `d64704daf4f1affcdcb1c2572a240cf833d97948` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.cargo/config.toml` | `74a4fb6c0f595ebc64841b26e7961967294f7d3e` |

- Re-read the independently accepted source/report at the fixed starting revision. Confirm the `R01.A` deliverables were accepted at this exact HEAD and that the `R01.B` source-level enforcement work remains a pending package. If the accepted disposition is genuinely unavailable or contradicted by the checked revision, **STOP** rather than inventing approval.

### 3. Required WIP reconciliation — exact edits

**A. ROADMAP:** In `.documents/WIP/plan-roadmap.md`, under `## Stage 01 — Reproducible engineering and hexagonal skeleton`, delete **only** the single existing bullet:

```text
- **R01.A:** establish the selected minimal source layout, strictly typed configuration, pinned tooling/dependencies and deterministic local build, without borrowing another repository's runtime.
```

Do not remove Stage 01, its prerequisite, `R01.B`, `R01.C` or its stage exit gate. Do not change any other Stage or package. Preserve the rest of the ROADMAP byte-for-byte (apart from the exact removed line and its line terminator), including heading order and original line ending policy. Do not insert a completed-work summary or historical note into ROADMAP.

**B. NEXT:** Replace the heading-only `.documents/WIP/plan-next.md` with **exactly** this single candidate, adjusting only Markdown line-ending encoding to normal tracked LF; no other candidate, commentary or planning policy:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## Stage 01 — R01.B: Hexagonal Source-Boundary Enforcement

**Objective:** Complete strict, fail-closed Rust source-level import and module-boundary enforcement for the existing four-crate workspace and its real CLI-to-application use case; prove illegal references cannot pass by aliasing, re-exporting, indirection or bypassing the guard.

**Prerequisites and readiness:** Stage 00 architecture/toolchain choices are accepted; `R01.A` was independently accepted at source revision `ff55dc702587cc8bfef98c8c8a3d0665ccfc162b`. The Rust 2024 workspace (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`), `rustc 1.99.1`, shared `Cargo.lock`, Cargo-metadata dependency graph guard, versioned inbound `BootstrapPreflightV1`, real CLI adapter and single composition root already exist. Architecture §6A.2 selects `syn =3.0.9` for source-level checking. The pending work is the missing AST/source-boundary proof, not recreation of the existing workspace.

**Candidate scope and deliverables:** Implement the Architecture §6A.3 `check-architecture` source checker alongside the real Cargo graph check. Enforce inward-only references, private adapter internals, `use`, `pub use`, renamed aliases, qualified paths, `extern`, `#[path]`, `include!`, module expansion and source-loading/dynamic-loading refusal. Handle inspectable supported generated/macro forms, fail closed on unresolvable forms, and add direct, alias, re-export, macro, qualified-path and dynamic negative fixtures, plus a guard-bypass/disabled negative. Preserve the existing real application port, test fakes where needed and the sole CLI composition root; do not introduce ornamental ports or later-stage product functionality.

**Verifiable exit conditions:** Legal existing source and graph return exit `0`; each forbidden fixture returns nonzero; bypassing/disabling the checker fails the suite. The check is invoked on the real source tree, not a grep-only approximation. Locked Rust format, Clippy, unit/integration/architecture and release-build checks pass with exact dependency and warning evidence, including negative tests and source/ref hashes. No implementation of R01.C CI, HTTP/browser, production publication, new provider, or unrelated stages.
```

This NEXT content is a **candidate** to be evaluated under a separate future Owner-issued order, **not** permission to implement it in this run. The candidate's detailed implementation path ledger and any allowed development dependencies belong to that future order; do not choose or install those now.

**C. ACTIVE:** Replace `.documents/WIP/workorder-active.md` with **exactly**:

```text
# Foundation Solo — Active Work Order
```

The file must be UTF-8 with one terminal LF, no BOM, no status essay and no remaining `SOLO-WO-029` or prior scope text. It is the neutral/no-work-order slot. The final repository history must still retain the **complete installed `SOLO-WO-029 R01`** as the prior local commit described in §5. Do not leave ACTIVE containing an old or new executable order after this closure.

**D. OUTLOOK and canonical product authorities:** Do not change OUTLOOK, About, Architecture or either SOP. No new durable product decision was made by the `R01.A` engineering implementation: the already accepted Rust/workspace/hexagonal rules are in Architecture §6A.2–6A.4. Record that parity check in the canonical execution report; do not duplicate accepted product contracts or run results in WIP.

**E. REPORT:** Overwrite only `.documents/Reports/execution-report.md` with the factual reconciliation run evidence described in §6. This report is not a WIP file; do not make a second report or archive.

### 4. Permanent path ledger and permissions

| Path | Operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Keep Owner-installed `SOLO-WO-029 R01` bytes unchanged through local issuance-history commit; then replace with exact neutral heading in the closure commit |
| `.documents/WIP/plan-roadmap.md` | Remove only the accepted `R01.A` Stage 01 bullet in §3A |
| `.documents/WIP/plan-next.md` | Replace heading-only file with exactly the single bounded `R01.B` candidate in §3B |
| `.documents/Reports/execution-report.md` | Overwrite with one factual closure/reconciliation report |

**Allowed:** local read-only review, local edits limited to the above permanent files, owned disposable scratch inside Solo `.temp/` or actual Cline-provided task temp (with cleanup), local Git staging and commits of those four files, and **one final push** of completed local commits to the existing `origin/main`, with final remote verification under Architect SOP §6B. No source build or package fetch is required for this document-only reconciliation.

**Denied:** editing application/tests/Cargo/lock/toolchain/config/README/AGENTS/About/Architecture/SOP/OUTLOOK, any other permanent file or other repository; dependency/toolchain installs; feature or checker implementation; intermediate push or GitHub file mutation; new branch, clone, worktree, pull request or issue; merge, release, deployment, host/provider changes, credentials/secrets, DNS/TLS, global OS changes, destructive Git cleanup, reset, rebase, history rewrite or force push. Do not install dependencies or start `R01.B` merely because NEXT describes it. Unlisted operations are not authorized.

### 5. Local execution and historical integrity

1. After preflight, **locally commit the exact Owner-installed ACTIVE file alone**, preserving the full `SOLO-WO-029 R01` instruction bytes in Git history. Do not publish this intermediate commit or treat it as an extra work order. Record its local SHA/tree for the report. This is a local-only issuance-history commit; do not modify the work-order text in that commit.
2. Apply ROADMAP, NEXT and ACTIVE edits in §3 locally. Independently compare the ROADMAP against the baseline: the removed `R01.A` line is the **sole** change; `R01.B`, `R01.C`, Stage 01 exit and all other stages are unchanged. Verify NEXT is exactly the specified candidate and ACTIVE is exactly the neutral heading. Check WIP file count remains four, with no subdirectories.
3. Verify unchanged product source, manifests, `Cargo.lock`, `.cargo/config.toml` and governing documents against their baseline Git identities; no new package was downloaded and no executable test was run. Mark Rust tests/builds **NOT RUN — documentation-only closure; accepted prior code unchanged**. Do not portray prior run results as freshly executed checks.
4. Overwrite the single report with factual closure evidence, locally commit the reconciled WIP/report changes, inspect the resulting local commit(s), all changed paths and the clean worktree. Use only the normal local Git procedure; do not reset, discard unknown files or stage unrelated material.
5. **After** all local edits, checks, report and commits are complete, **push all authorized commits once** to the existing `origin/main`. Confirm that remote `main` resolves to the final local HEAD/tree and all four WIP files and the report have the expected content. No earlier push, PR or separate online publication sequence.

If any mandatory check fails, STOP and preserve safe local state; if the canonical report can be safely published under the SOP, report `STOPPED` truthfully without falsely closing `R01.A` or asserting the final publication succeeded.

### 6. Acceptance evidence and handback

The one canonical execution report must include: date, `SOLO-WO-029 R01`, verified root/branch/starting HEAD/tree, separate local installed-order history commit SHA, final source and report commit/tree identities (excluding **the report's own final commit SHA** from its contents), exact changed files, a before/after Stage 01 ROADMAP excerpt or equivalent line-diff evidence, exact NEXT candidate ID and readiness basis, exact neutral ACTIVE byte check, WIP inventory, unchanged governing/source/lockfile identities, actual local Git command/results, final one-time push result, working tree and approved scratch cleanup, and status `COMPLETE`, `STOPPED` or `FAILED`. The report's final commit SHA and link belong in chat only.

| Gate | Required direct evidence |
| --- | --- |
| **L-01 — Acceptance and source** | R01.A independently `ACCEPT` at the reviewed baseline; one checkout, correct `main` and `origin`, only Owner-installed ACTIVE preflight change |
| **L-02 — ROADMAP** | Exactly one `R01.A` bullet removed; `R01.B`/`R01.C`, stage exit, all other stages remain identical |
| **L-03 — NEXT** | Exactly one dependency-ready `R01.B` candidate with bounded objective, prerequisites, deliverables and exit; no implementation performed |
| **L-04 — ACTIVE/WIP** | ACTIVE neutral exact single heading + LF; exactly four WIP files; OUTLOOK unchanged; issued-work-order bytes preserved in local Git history |
| **L-05 — No unintended change** | Source/manifests/lock/config, About/Architecture/SOP and all non-allowlisted paths remain unchanged; code test/build gates marked NOT RUN with scope-specific reason |
| **L-06 — Evidence and publication** | Single factual canonical report, clean local tree, task-owned scratch cleaned, one safe final push and final remote identity confirmation |

The Implementation Agent reports facts and does not independently `ACCEPT` R01.B or close Stage 01. The Chief Architect reviews the exact published reconciliation and returns `ACCEPT`, `REVISE` or `STOP`. Starting R01.B still requires a **separate** Owner-issued work order and matching handoff.

### 7. STOP conditions

STOP on an unverified accepted disposition, changed starting HEAD/tree, wrong repository/remote/worktree, conflicting Owner-installed order, unexpected file/ownership/symlink state, non-authorized path or report, ROADMAP changes beyond removing R01.A, more or fewer than four WIP files, multiple NEXT candidates, a new technical decision, inability to preserve exact prior order in local history, failure to produce a clean local state, or inability to publish safely at the end. Do not silently substitute a different package, move accepted work to OUTLOOK, amend architecture, implement R01.B or manufacture successful Git evidence.
