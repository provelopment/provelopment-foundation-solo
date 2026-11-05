# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-007`  
**Edition:** `R03`  
**Title:** R00.A acceptance correction — eliminate duplicate Git guidance and repair execution-evidence accuracy  
**Approval marker:** `OWNER-ISSUED` **only upon Owner installation and matching direct handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `99748354b3e62659a4760b55c27c77f0feafed47`  
**Matching Owner handoff:** `SOLO-WO-007 R03 — Implementation Agent Handoff`  
**Roadmap scope:** Stage 00, **R00.A only** — bounded correction to the already delivered `SOLO-WO-007 R02` documentation run; no new roadmap package.  
**Work class:** Documentation/evidence correction only; **no application code, toolchain, runtime validator, software testing, release or deployment**.  
**Chief Architect disposition prompting correction:** `REVISE` against R02 content commit `3ff9a0cf916999a96cb37dcdcfc0c586e1949e3f` and report commit `99748354b3e62659a4760b55c27c77f0feafed47`.  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP` at the exact new published revision. Acceptance does **not** authorize WIP closure.

> **Owner issue boundary.** This is the **one complete replacement ACTIVE work order**, edition R03, for Owner review. It supersedes R02 **only if** the Owner installs these exact bytes in `.documents/WIP/workorder-active.md` and separately sends the matching R03 handoff. The presence of this document, its filename, a status marker, an earlier R02 handoff, or a chat discussion alone grants no permission to execute. The Owner's installation is the only permitted initial local change to ACTIVE; the Implementation Agent must not self-issue, rebase, amend, or rewrite this order.

**Frozen Approved Scope:** §§1–7 below are indivisible. The agent may publish this Owner-installed text exactly, but may not change its content, append execution notes, clear ACTIVE or mark acceptance.

## 1. Objective and known acceptance failures

Repair **only two** failed conditions of the R02 Chief Architect review, without repeating or redesigning the accepted R00.A semantic contract:

1. **C1 / acceptance gate 6 — duplicate Git guidance remains.** `.documents/SOP/sop-implementation.md` §5B removed its numbered duplicate list but replaced it with a long paragraph re-enumerating most of Architect SOP §6B. This still creates a maintainable competing paraphrase. Replace the whole §5B body with the concise **exact** binding reference in §2.A below, keeping Architect SOP §6B the sole operative version-control procedure.
2. **Evidence integrity / acceptance gate 11 — R02 execution-report assertions are inaccurate.** The R02 report calls the §2 scope “all twelve decisions,” although the frozen R02 order numbers **13 items**: **11 numbered technical decisions (§2.A–C, items 1–11)** and **two specification/non-decision directives (§2.D, items 12–13)**. It also reports that its stale-phrase search returned zero matches for broad terms such as `ID grammar` and `not yet been accepted or implemented`, even though legitimate occurrences remain in the current documents. The new canonical report must explicitly correct these statements, report actual searches and their findings accurately, and distinguish **legitimate remaining mentions** from **genuinely stale authoritative claims**. Historical R02 commits must not be changed or rewritten.

**Already delivered and not reopened:** R00.A identity grammar, parent-scoped uniqueness, permanent IDs, initial `main`/`main`/`home` authoring defaults, full-locale representation identity, published cardinality including unlisted representations, drafts outside the published graph, one versioned publication authority, source integrity, and the fail-closed refusal specification. Their R02 architecture/product text is already recorded in the canonical authorities and remains read-only for this correction.

**Acceptance boundary:** This R03 run corrects the outstanding documentation/evidence defects. It must not represent a validator, manifest parser, executable tests, implemented website, deployment, or completed Stage 00. `R00.B–R00.G` remain pending even if `R00.A` is later independently accepted and closed.

## 2. Required corrections — exact and bounded

### A. Finish C1 without duplicating the procedure

Replace **the entire content of §5B**, from its heading up to but not including the `## 5C. Mandatory execution report — every run` heading, in `.documents/SOP/sop-implementation.md`, with **exactly** the following Markdown (normal LF line endings; no extra procedural summary):

```markdown
## 5B. Version Control Procedure — Git and GitHub

**Binding authority:** [`sop-architect.md`](sop-architect.md) §6B is the **single authoritative Solo version-control procedure**. The Implementation Agent must read and follow that section in full. This §5B creates no parallel procedure, independent Git permission or substitute summary.

**Implementation-role cross-references:** Verify authorization under §4; perform repository and working-tree preflight under §5; enforce single-root and temporary-file discipline under §5A; and publish the canonical execution report under §5C. These role-specific requirements supplement, and never relax, Architect SOP §6B.
```

Do not modify Architect SOP §6B: it remains the authority for permitted mechanics, forbidden destructive operations, branch and remote checks, and distinct merge/release/deployment authorization. Do not add another Git procedure to `AGENTS.md`, README, WIP or any other file. Only this §5B text edit is permitted in the Implementation SOP.

### B. Repair the current report honestly — no historical rewriting

Overwrite **only** `.documents/Reports/execution-report.md` in the designated report phase. This is the canonical record of the **R03 run**, not an edited or backdated R02 report. It must:

- Identify `SOLO-WO-007 R03`, the matching Owner handoff, actual session date, required starting revision `99748354b3e62659a4760b55c27c77f0feafed47`, resulting Phase A content revision, and actual publication result.
- State that the independently reviewed R02 commits remain intact in Git history; explicitly acknowledge and correct the two R02 report inaccuracies. Count **13 numbered R02 §2 items**, distinguishing **11 technical decisions** and **2 conceptual-example/non-decision directives**, rather than claiming 12 decisions.
- Re-run targeted **read-only** phrase searches against the relevant current published/working documents (`README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, and both SOPs) and identify the searched phrases, actual matches (path, line and relevant context or an equivalent inspectable result), and which matches are legitimate. In particular, `ID grammar` and `not yet been accepted or implemented` must **not** be described as absent where present. Check genuine obsolete claims in context, rather than inferring staleness from matching words alone. Do not introduce unrelated edits to “make the search pass.”
- State exactly what §5B changed and confirm Architect SOP §6B is unchanged. Provide the pre/post §5B structural comparison and evidence that no duplicated procedural summary remains.
- Distinguish **directly observed GitHub commit/blob/contents evidence** from **agent-reported local preflight/cleanup evidence**. Record commands actually run and their meaningful outputs/results; do not portray local observations as independently verified by the Chief Architect.
- Record exact changed paths, HEAD/branch/origin, initial dirty paths, one-worktree/root checks, parent-ignore read-only result, temporary-file inventory and cleanup, WIP/report inventory, document link checks and Markdown/document integrity checks actually performed.
- Clearly label executable tests, browser, CI, security scan, release and deployment as **NOT RUN / NOT APPLICABLE** for this documentation-only task, with reasons. Do not manufacture a test pass.
- Include a criterion-by-criterion factual evidence table for §7, using **PASS / FAIL / UNVERIFIED** for checks actually within scope, and **NOT APPLICABLE** with a reason for genuinely out-of-scope software checks. The agent does not issue the independent Architect disposition.
- Never embed the report commit's own final SHA in the report; give it only in the agent's conversation after publication.

Do not retrospectively edit the prior report commit or erase its historical claims. Correction is accomplished through this truthful current report and normal forward-only Git history.

## 3. Exact path-and-operation permission ledger

| Path | Authorized operation |
| --- | --- |
| `.documents/SOP/sop-implementation.md` | **Only** replace §5B with the exact §2.A text; no other subsection or SOP change. |
| `.documents/WIP/workorder-active.md` | The Owner installs this R03 replacement; agent may **only** stage and publish it unchanged in Phase A. No self-edit, evidence addition or clearance. |
| `.documents/Reports/execution-report.md` | In Phase B, overwrite the existing single canonical report with truthful **R03** evidence as §2.B directs; no second report. |
| `.git/**` | The minimal ordinary, scoped Git operations needed for the two authorized commits and fast-forward publication/readback, strictly under Architect SOP §6B. |

**Everything else is read-only**, especially `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, `.documents/SOP/sop-architect.md`, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, `.github/**`, `LICENSE`, tooling/source/test files, parent and sibling repositories. The original `R00.A` contract, accepted architectural invariants, and all R00.B–G planning remain frozen. No created/renamed/moved document, additional WIP slot, duplicate report, alternate authority or archive is allowed.

**Explicit permission decisions:** reading the Solo repository and approved reference material = YES; only the two permanent edits listed above plus publication of the untouched Owner-installed R03 ACTIVE = YES; one scoped Phase A content commit and one separate Phase B report commit = YES; normal fast-forward pushes to **verified Solo `origin/main`** and GitHub remote readback = YES; agent-owned scratch under Solo `.temp/` only when necessary and safely contained/cleaned = YES. New branches, linked/extra worktrees, clones, shadow checkouts, additional remotes, any outside-Solo write/cache, dependency installation, code or tests, provider/account or DNS operations, secrets access, PR/merge, release/tag/deployment, force-push/history rewrite/amend, destructive reset/clean/checkout, and Owner-data deletion = **NO**. Git transport/readback is the only authorized network activity.

## 4. Required preflight and STOP conditions

1. Read `README.md`, `AGENTS.md`, the Product Definition and Architecture Charter, both SOPs, all four WIP files and the **current GitHub-published** execution report. Confirm the exact `SOLO-WO-007 R03` text is installed in ACTIVE, marked `OWNER-ISSUED`, **and** the Owner has directly delivered the **matching R03 handoff**. The prior R02 handoff is insufficient. If not, STOP.
2. Verify the actual physical Solo root, `git rev-parse --show-toplevel`, branch `main`, exactly one worktree, exactly one remote `origin` addressing `provelopment/provelopment-foundation-solo`, and initial local `HEAD` = `origin/main` = GitHub API `main` HEAD = **`99748354b3e62659a4760b55c27c77f0feafed47`**. If HEAD has changed, STOP; do not silently rebase the approved order.
3. Verify the only initial working-tree difference is the **unstaged Owner-installed change** to `.documents/WIP/workorder-active.md`; no unexpected staged, untracked or modified paths. Confirm four WIP files and exactly one canonical report. Verify ROADMAP, NEXT and OUTLOOK Git blob IDs match the following published baseline: ROADMAP `84f428b8d409c1665fc49d03a0e0e310c818b328`, NEXT `92fa562b1af0c54391851aa5c79deb1631127069`, OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`. Capture the current R02 ACTIVE blob `0ec26094ec08f35fb1b4b59bc302c423f38400dc` as historical baseline; the new R03 blob is intentionally different only by Owner replacement.
4. Confirm the R02 published SOP §5B still contains the duplicated long-form Git summary and the R02 report still contains the cited inaccurate assertions. If the supposed defects are absent or a contract changed, STOP and seek review rather than inventing work or broadening scope.
5. Inspect tool side effects, path/symlink boundaries, local report safety and the parent's ignore guard read-only. Do not delete pre-existing unknown files. Any authority conflict, unsafe publish state, uncontained write, unexpected dirty path, external operation, missing evidence or need for a non-allowed file is a STOP with a truthful report when safe under the standing reporting rule.

## 5. Ordered implementation and publication

**Phase A — Exact SOP correction and issue record.** Replace only Implementation SOP §5B by §2.A; do not modify any other substantive document. Confirm full §6B remains unchanged and the new §5B is purely a binding cross-reference, with no Git rule paraphrase. Inspect Markdown headings, link targets, whitespace and exact diff, and compare all non-allowed blobs with the baseline. Stage **only** `.documents/SOP/sop-implementation.md` and the Owner-installed, byte-identical `.documents/WIP/workorder-active.md`. Make **one ordinary content commit**, fast-forward push to verified Solo `origin/main`, and read back the exact commit, both paths and unchanged authorities on GitHub. If either file's changed content differs from this approved boundary, STOP rather than publish it as complete.

**Phase B — Corrected current execution evidence.** Overwrite only `.documents/Reports/execution-report.md` as specified in §2.B, with truthful retrospective correction statements and the actual current-run evidence. Stage only this report, make **one ordinary report commit**, fast-forward push to verified Solo `origin/main`, verify current remote HEAD and the report blob, and provide the report commit SHA only in the chat handback. Capture source/report revision links and actual results; do not self-accept or modify ACTIVE after publishing.

**Final state:** `workorder-active.md` contains the complete issued R03 order unchanged; ROADMAP, NEXT and OUTLOOK retain their baseline bytes; only the Implementation SOP §5B correction and the canonical report differ as authorized, in addition to the Owner-installed ACTIVE replacement; a normal completion leaves a clean local tree, one worktree and no agent-created temporary residue. STOP after reporting for independent Chief Architect acceptance review.

**Failure path:** On genuine blocker or failed check, preserve Owner-owned material and frozen scope; publish a factual `STOPPED`/`FAILED` canonical report only if safe under the SOP's standing reporting permission. Never repair a failure by changing an unapproved file, deleting unknown content, weakening the SOP, rewriting historical commits, or suppressing a negative result.

## 6. Non-negotiable architecture and lifecycle boundary

This order **does not change** the accepted one-Entity EFS hierarchy, ID grammar or parent-scoped uniqueness, full-locale and same-Page behavior, explicit location independence, visibility and public/unlisted route-case rules, property/graphic inheritance and suppression, strict Hexagonal Architecture, safe `mailto:` plus simple contact form with advanced-form exclusion, Apache-2.0, or the approved R00.A declaration/source-integrity semantics.

No executable application, manifest parser, provider adapter, test suite, validation engine, feature-completion proof, PR, release or deployment may be claimed. In particular, **Stage 00 remains open** with packages `R00.B–R00.G` pending. When the corrected R03 evidence receives independent `ACCEPT`, only the accepted **`R00.A` package** is eligible for a **separate, explicit Owner-authorized closure/reconciliation**. The Implementation Agent cannot consume that package, clear ACTIVE, or select NEXT under this work order; the Chief Architect does not self-issue or perform the closure.

## 7. Independent Chief Architect acceptance gates

The agent publishes factual evidence; the **Chief Architect** independently returns `ACCEPT / REVISE / STOP` against the exact GitHub revision. For each applicable gate record `PASS / FAIL / UNVERIFIED` with direct evidence. Where a general standard is genuinely irrelevant to a documentation-only task, record `NOT APPLICABLE` with a reason, never as PASS.

1. **Authority and baseline:** exact issued `SOLO-WO-007 R03`, directly matching Owner handoff, correct baseline `99748354…`, root, main, one worktree, one remote and expected initial dirty state; no unapproved replacement/rebase.
2. **C1 complete:** Implementation SOP §5B is **exactly** the short §2.A reference and role-specific pointers; no long summary or second Git procedure; Architect SOP §6B remains untouched and controlling.
3. **R02 count correction:** canonical R03 report states **13 numbered items**, distinguishing 11 decisions and two examples/non-decision directives; historical R02 miscount is acknowledged without Git history rewriting.
4. **Evidence search truth:** the current report gives actual phrase-search terms and observed matches/legitimate contexts, admits that `ID grammar` and `not yet been accepted or implemented` can remain in valid text, and makes no falsely universal zero-match claim.
5. **Architectural preservation:** original R00.A Product/Architecture text, accepted Solo contracts, README, AGENTS, Architect SOP and unrelated SOP sections are unchanged. No new decisions, weakened rules, code, tooling or software-proof claims.
6. **WIP and report integrity:** exactly four WIP files; ACTIVE byte-identical to Owner-installed R03 and remains issued; ROADMAP, NEXT and OUTLOOK blobs unchanged; exactly one overwritten canonical report; no competing order or archive.
7. **Git and scope hygiene:** only the two expected Phase A paths and only the report in Phase B, two ordinary forward-only commits, correct remote readbacks, no outside-root writes, unsafe cleanup, branch/worktree surprises or unexplained dirty state. Local findings correctly attributed to agent evidence.
8. **Report quality and acceptance boundary:** accurate starting/content/reported revisions, actual checks and limitations, no concealed failures or invented passes, clear NOT RUN software gates, no self-acceptance and no premature R00.A or Stage 00 closure.

**Completion boundary:** Independent `ACCEPT` may follow only when all applicable gates are substantiated without unresolved material failures or unjustified `UNVERIFIED` status. After that, the **Owner must separately authorize** the one-package R00.A closure; neither this R03 handoff nor its success authorizes closure, Stage 00 completion, R00.B execution, release or deployment.
