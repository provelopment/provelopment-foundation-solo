# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-008`  
**Edition:** `R01`  
**Title:** R00.A accepted-package closure and three-slot WIP reconciliation  
**Approval marker:** `OWNER-ISSUED` **only upon exact Owner installation and separately delivered matching handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `c32393511f9ac33fa068ecff74e4b0656da8026e`  
**Matching direct handoff:** `SOLO-WO-008 R01 — Implementation Agent Handoff`  
**Work class:** Owner-authorized **closure of accepted Stage 00 package R00.A only**, not closure of Stage 00 or execution of R00.B.  
**Architect acceptance basis:** Independent `ACCEPT` for `SOLO-WO-007 R08`, content `78c1a27dd058b81ec387d60ade8979a7f5aa3c95`, canonical report `c32393511f9ac33fa068ecff74e4b0656da8026e`, together with the already verified R00.A identity/publication contract and C1–C6 documentary corrections.  
**Completion authority:** Independent Chief Architect review of this *closure execution* after publication; the Implementation Agent does not self-accept, select or issue another order.

> **Owner issue boundary.** The Owner has separately directed preparation of R00.A closure following independent acceptance; this draft becomes executable only after the Owner installs **these exact bytes** in `.documents/WIP/workorder-active.md` and directly delivers the matching `SOLO-WO-008 R01` handoff. The prior `SOLO-WO-007 R08` order, previous handoffs, a roadmap entry, or this document by itself do not authorize execution. The Agent may publish the Owner-installed closure order **unchanged**, then neutralize ACTIVE only in the explicitly authorized closure phase below. There is never a second active work order.

**Frozen Approved Scope:** §§1–7, including the exact WIP replacement text, path ledger, publication sequence, exclusions and acceptance gates. Do not extend this authorization by inference.

## 1. Objective and closure decision

Reconcile the **single accepted R00.A package** under Architect SOP §8 and the Owner's separate closure authorization:

1. Remove **only** the completed `R00.A — Identity and publishing declarations` package from `.documents/WIP/plan-roadmap.md`. Preserve the entire Stage 00 heading, outcome, remaining six packages **R00.B–R00.G**, Stage 00 exit gate, and all Stages 01–13 without change. **Stage 00 is not complete and must remain open.**
2. Replace the title-only `.documents/WIP/plan-next.md` with the **single bounded, non-executable R00.B candidate** specified exactly in §2.B. It proposes work for future Owner review; it neither accepts an unresolved technical decision nor authorizes R00.B execution.
3. Replace `.documents/WIP/workorder-active.md` with the **single neutral title line** in §2.C, removing the just-issued closure order from the live WIP slot only **after first recording that exact issued order in Git history**. This is the Owner-authorized closure of the *execution slot*, not the agent's own acceptance decision.
4. Publish one factual final run report at `.documents/Reports/execution-report.md`, preserving Git history and identifying the accepted evidence, exact reconciliation, and final repository state.

**Durable decisions already integrated:** The accepted R00.A technical IDs, parent scopes, starter defaults, cardinality, draft exclusion, authoritative versioned publication declaration, authored-source integrity, fail-closed refusals, conceptual evidence and preserved non-decisions are already present in the canonical Product/Architecture documents. R08's agreed temporary-workspace and SOP governance is published. Re-read and confirm; **do not duplicate or modify the canonical documents** in a procedural closure. If a genuine missing or contradictory accepted durable rule is found, **STOP** for Owner/Architect review rather than inventing a reconciliation edit.

**Historical disposition remains historical:** The R03 and R05 `STOP` findings and R06's unpublished local `STOP` are not converted to successes by closing the later accepted R08 work. No execution report archive, completion narrative in ROADMAP/NEXT/ACTIVE, or Git history rewrite is permitted.

## 2. Exact WIP reconciliation — no discretionary content changes

### A. ROADMAP — delete exactly one package line

From `.documents/WIP/plan-roadmap.md` under `## Stage 00 — Architectural contract dossier (decisions only)` / `**Remaining packages**`, delete **only** this exact Markdown line, including its newline:

```markdown
- **R00.A — Identity and publishing declarations:** settle stable Entity/Facet/Site/Page IDs and their scope, default Facet/Site names, whether incomplete drafts can exist outside the publishable model, authoritative manifest/content boundaries, source-to-declaration integrity and fail-closed duplicate/orphan detection.
```

The line must occur **exactly once** before the edit and **zero** times afterward. Do not change the `**Remaining packages**` label, Stage 00 outcome or exit gate, any `R00.B`–`R00.G` entry, or any subsequent stage, prerequisite or gate. Do not add a `Completed`, `Closed`, `Accepted` or progress section. A line-oriented exact deletion is the complete authorized ROADMAP edit.

### B. NEXT — replace the entire file with this exact Markdown

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R00.B — Address/locale contract

**Objective:** Prepare an Owner-reviewable decision set for Solo's configured public addresses, routing and full-locale behavior without changing the accepted R00.A identity/publication contract.

**Prerequisites and readiness:** R00.A independently accepted and Owner-authorized for closure; applicable Product and Architecture rules available. R00.B remains pending in Stage 00. This is a decisions-only candidate, not an implementation task.

**Proposed scope:** Specify HTTP(S) origin and optional base-path normalization; path/segment encoding; Site, Page and Home routing; redirect status and aliases; route collision refusal; locale preference mapping; error representations; and construction of full-locale URLs. Preserve the already settled public lowercase canonical/case-insensitive behavior, exact-case unlisted Page tokens, full region-qualified locales, same-Page fallback, and location/routing independence.

**Proposed deliverables:** A bounded R00.B Owner decision proposal, precise proposed updates to the existing canonical Architecture/Product authorities where needed, and conceptual positive/refusal cases identifying accepted rules versus still-open design choices. Do not select a framework, write code or portray proposals as approved decisions.

**Verifiable exit conditions:** Each R00.B issue has an explicit Owner decision or a clearly named blocker; the resulting accepted address/locale contract is consistent with R00.A and existing Solo invariants; public/unlisted collision and locale/route negative cases are specified; Stage 00 remains open until its other packages and exit gate are accepted.
```

The file must contain **only this one candidate**. It is neither `OWNER-ISSUED` nor an authorization to implement it. Do not add extra candidates, planning policy, Stage 00 history or a status diary.

### C. ACTIVE — replace the entire file with this exact neutral title

```markdown
# Foundation Solo — Active Work Order
```

Retain this file as the third of the four WIP filenames; **do not delete it**. The neutral title has no `SOLO-WO-*` ID, `OWNER-ISSUED` marker, approval text, closure history, acceptance narrative, or execution permission. The complete issued `SOLO-WO-008 R01` order must already have been committed and verified on `origin/main` in Phase A, so its authority and exact content remain retrievable in immutable Git history before ACTIVE is cleared.

### D. OUTLOOK — no change

`.documents/WIP/plan-outlook.md` remains **byte-identical** to the starting baseline. Neither completed R00.A work nor the pending R00.B candidate belongs in OUTLOOK.

## 3. Exact path and operation permission ledger

| Path or location | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs this complete R01 order; Agent may stage/publish **unchanged** in Phase A, then replace **only** with §2.C's neutral title in Phase B. No third edit. |
| `.documents/WIP/plan-roadmap.md` | In Phase B only, delete precisely the one R00.A package line in §2.A. |
| `.documents/WIP/plan-next.md` | In Phase B only, replace title-only stub with exact §2.B candidate. |
| `.documents/Reports/execution-report.md` | In Phase C only, overwrite the single canonical report with truthful closure execution evidence; its own final report-commit SHA appears in chat **only**. |
| `.git/**` | Minimal normal Git metadata for the **three** scoped forward-only commits, authorized fast-forward pushes and readback, under Architect SOP §6B. |
| Solo `.temp/` and actual Cline-provided task temp | Standing discretionary temporary use and ownership-safe cleanup under accepted Architect SOP §6A and Implementation SOP §5A. Solo `.temp/` must never be staged/tracked/committed; no additional checkout anywhere. |

**Every other permanent file is read-only:** Architect and Implementation SOPs, AGENTS, README, Product, Architecture, `.gitignore`, `.gitattributes`, `.github/**`, LICENSE, OUTLOOK, code/test/tooling and parent/sibling repositories. No new WIP slot, report/archive, branch, worktree, clone, remote, shadow checkout, provider/account operation, dependency install, application source, PR/merge, tag/release/deployment, secrets read, destructive Git operation, amend, force-push, rebase, history rewrite or deletion of unknown material. The single existing Solo `main` checkout and verified `origin` are mandatory; the standing report-only permission applies to Phase C after ACTIVE becomes neutral.

**Git authority:** Follow Architect SOP §6B as the **only** detailed version-control procedure; this ledger merely states this order's allowed paths, three publications and prohibited operations. No additional Git procedure is created by this work order.

## 4. Mandatory preflight and immutable baseline

Before any write, read root README and AGENTS, both SOPs, the current Product and Architecture, all four WIP files and the **current GitHub-published execution report**. Verify the accepted R08 source/report commits and the separately delivered matching Owner closure handoff. Check the actual physical Solo root against `git rev-parse --show-toplevel`, exactly one checkout/worktree, branch `main`, exactly one correct `origin`, and no branch/remote surprise.

**Hard revision gate:** Local `HEAD` = `origin/main` = GitHub API `main` = **`c32393511f9ac33fa068ecff74e4b0656da8026e`** before edits. If `main` moved, **STOP** rather than rebase the Owner-approved order. The **only** initial dirty path is the Owner-installed **unstaged** replacement `.documents/WIP/workorder-active.md` and no other unexpected staged, modified or untracked file. Treat permissible ignored temporary material under the accepted cleanup SOP, not as an automatic STOP.

Compare these exact starting Git blobs:

| Path | Required Git blob SHA-1 |
| --- | --- |
| `.documents/WIP/plan-roadmap.md` | `84f428b8d409c1665fc49d03a0e0e310c818b328` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/WIP/workorder-active.md` (R08, before Owner installation) | `bb7e42fc7937bef118dffb7033a967f53ba6ceeb` |
| `.documents/Reports/execution-report.md` (R08) | `7c2b378fe6d12a106095ffd92845a27a379be29a` |
| `.documents/About/FOUNDATION-SOLO.md` | `3f4e557e7b5b67c544bf2ce8ce718e5aec706fe8` |
| `.documents/Architecture/ARCHITECTURE.md` | `28c0629549259185a715d4291ea5d065cffa3ec8` |
| `.documents/SOP/sop-architect.md` | `6bea0dbcab31995a7d3f8020e8bfde8d6c195c75` |
| `.documents/SOP/sop-implementation.md` | `b56b14a0fec4b0b1b6342445eaa86ebf3a0d807d` |
| `AGENTS.md` | `30c5b02678d91140a1c74dc06aaefd333f42776e` |
| `README.md` | `1294bb12aa0eed370304cbc64a829a3f32d8fcac` |
| `.gitignore` | `f51cc5306cdc8e4532422809eede68b93c4a7fd7` |

Verify all four WIP files and exactly one canonical report, the parent ignore guard read-only, and Solo `.temp/` ignored with zero staged/tracked contents. Inventory disposable task material in both approved temp areas; clean only identifiable unneeded task-owned files and preserve unknown/Owner-owned material. Confirm the R00.A decisions are already in canonical Product/Architecture and no contract needs a new decision before this administrative closure. Any mismatch, missing acceptance/approval, unexpected permanent edit, missing line, or duplicate occurrence is a **STOP**, not permission to make a best-guess modification.

## 5. Ordered execution and publication — three ordinary commits

**Phase A — Preserve the exact issued closure authority.** After matching the installed bytes and Owner handoff, stage **only** `.documents/WIP/workorder-active.md` with the Owner-installed, **unchanged** `SOLO-WO-008 R01` text. Create **one ordinary commit**, fast-forward push to verified Solo `origin/main`, and read back the exact order's Git blob, commit and remote HEAD. Record this content commit for Phase B and the final report. Nothing else may be changed in this phase. If the exact Owner-installed file cannot be verified or published, STOP; do not clear ACTIVE.

**Phase B — Owner-authorized three-file WIP closure.** With the issued Phase A order verified on remote, apply **only** §2.A–C. Stage exactly `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md` and `.documents/WIP/workorder-active.md`. Verify the ROADMAP diff is exactly one deleted R00.A package line, NEXT is byte-identical to §2.B, ACTIVE is byte-identical to §2.C, OUTLOOK is unchanged, and the WIP inventory is exactly four files. Create **one ordinary closure commit**, fast-forward push, and read back all three blobs and the correct remote HEAD. Record its SHA for the report. After this commit ACTIVE is deliberately **neutral**, and no further implementation/WIP work is authorized by this closed task; only the previously authorized final report remains.

**Phase C — Canonical closure report under standing report-only permission.** Overwrite **only** `.documents/Reports/execution-report.md`. Document the original Owner authorization, matching handoff, the independently reviewed R08 acceptance, all actual preflight results, both preceding commit SHAs, exact changed-file and WIP inventory, ROADMAP removal and remaining packages, R00.B candidate without execution permission, neutral ACTIVE, unchanged canonical documents and OUTLOOK, GitHub readbacks, temp-workspace ownership/cleanup and any genuinely retained artifacts, warnings/limitations, and actual software checks `NOT RUN` / code-specific general gates `NOT APPLICABLE` with reasons. Clearly distinguish GitHub-readable facts from locally observed facts and historical report statements. Do **not** embed the report commit's own final SHA in its content. Stage only the report, create **one separate ordinary report commit**, fast-forward push and verify the GitHub report and `main` HEAD. Provide the report-commit SHA in the agent's chat handback. STOP for independent review; never self-accept or begin R00.B.

**If a phase fails:** STOP, preserve the last safely published state and Owner material; report accurately through the one canonical report **only if safe and authorized**. Do not make later reconciliation edits to hide a failure, force a commit count, reset, clean, invent acceptance or leave a misleading candidate as an active task.

## 6. Boundaries, non-goals and final state

The only accepted obligation being consumed is **Stage 00 package R00.A**. Stage 00's other packages **R00.B–R00.G** and its exit gate remain, as do all later stages. The completion of a documentary architecture package is **not** proof of executable validators, manifests, site behavior, build, tests, security scanning, release or deployment. Neither product architecture nor SOP governance is changed by this closure. Previously failed execution runs stay in Git history with their original dispositions. The new NEXT candidate is for future review and issuance; it has **no execution authority**.

**Required final WIP shape:** ROADMAP = Stage 00 with R00.B–R00.G plus remaining stages; NEXT = one R00.B candidate from §2.B; ACTIVE = exactly `# Foundation Solo — Active Work Order` with final LF and **no issued order**; OUTLOOK = byte-identical title-only file. Exactly one canonical report remains outside WIP. No parallel task status file, archive, duplicate report or unrelated permanent change is allowed.

**Software and environmental safety:** Use the Owner-permitted Solo `.temp/` or actual Cline-provided task temp freely for disposable work; keep `.temp/` out of Git; clean identified obsolete task-owned files promptly, including accessible task-capture logs, and report any cleanup failure or justified retention. **Never create a second worktree/checkout/clone**, even temporarily. No provider operations, dependency installation, code, merge, release or deployment.

## 7. Independent closure-execution acceptance gates

The Implementation Agent reports factual evidence, not independent acceptance. The Chief Architect evaluates each applicable gate `PASS / FAIL / UNVERIFIED`; genuinely irrelevant code-specific standards are `NOT APPLICABLE` with reasons, and unperformed software checks are `NOT RUN`.

1. **Authority and baseline:** Exact issued `SOLO-WO-008 R01` and direct matching Owner handoff; verified starting `c3239351…`, one root/checkout/remote, sole Owner-installed initial dirty path, baseline blob parity, previously accepted R00.A evidence, four WIP files and one report.
2. **Historical authorization proof:** Phase A commits **only** the unchanged Owner-installed complete closure order; that exact commit and Git blob are verifiable on `origin/main` **before** ACTIVE is cleared.
3. **ROADMAP precision:** Phase B removes **only** the single exact R00.A bullet; retains Stage 00 outcome, R00.B–R00.G, Stage 00 exit gate and all other stage content, with no completed-work narrative.
4. **NEXT precision:** Phase B populates one and only one **non-authorizing** R00.B candidate, matching §2.B byte-for-byte; no acceptance of open technical choices, hidden executable order or extra candidate.
5. **ACTIVE/OUTLOOK and authority purity:** Phase B neutralizes ACTIVE to the exact §2.C title without deleting the file, preserves OUTLOOK byte-identically, maintains exactly four WIP files and one report, and does not begin another task.
6. **Canonical preservation:** Product, Architecture, both SOPs, README, AGENTS, license, `.gitignore` and all other read-only paths remain byte-identical; accepted R00.A decisions are already canonical; no Stage 00 exit, application, toolchain, release or deployment is claimed.
7. **Git and cleanup hygiene:** Exactly one order-only Phase A commit, one three-WIP-file Phase B commit and one report-only Phase C commit; ordinary linear fast-forward on correct `origin/main`, verified blobs/readbacks, clean final Git tree, one checkout and truthful no-garbage temp cleanup/retention evidence.
8. **Evidence and separation:** A truthful canonical report with original acceptance provenance, actual operations/limitations, GitHub-versus-local distinction, software gates honestly labeled, no report self-SHA, no fabricated runtime proof, no agent self-ACCEPT, and no premature R00.B execution or Stage 00 closure.

**Completion boundary:** Successful closure execution may be independently accepted only after verification of the exact published report and revision. The approved R00.A obligation is then absent from ROADMAP, while Stage 00 and R00.B–R00.G remain open. The Architect does not unilaterally issue another task, select additional work, release or deploy.
