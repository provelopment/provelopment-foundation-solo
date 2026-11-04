# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-006`  
**Edition:** `R00`  
**Title:** Close accepted `SOLO-WO-005 R02` and reconcile the live WIP slots  
**Approval marker:** `OWNER-ISSUED`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `87608ee01bf3a898345aee0d64f1c7e57620f34f`  
**Matching Owner handoff:** `SOLO-WO-006 R00 — Implementation Agent Handoff`  
**Work class:** Accepted-work closure only; no application or architectural implementation.  
**Accepted work:** `SOLO-WO-005 R02`, independently dispositioned `ACCEPT` by the Chief Architect against content commit `005f3b61b2b3e685448994a50533dd0976de8c0a` and report commit `87608ee01bf3a898345aee0d64f1c7e57620f34f`.  
**Closure decision:** The Owner authorized proceeding with the accepted-order closure reconciliation. This order defines its exact authorized operations.

## 1. Exact objective and end state

Record this closure instruction in immutable Git history, close the already accepted documentation work, and leave the four WIP slots consistent with Architect SOP §8. This task is **not** acceptance of an implementation roadmap package and does **not** initiate the next package.

At successful completion:

- `.documents/WIP/workorder-active.md` contains **exactly** `# Foundation Solo — Active Work Order` followed by one newline; no active order, placeholder essay, previous order body or completed-work record remains.
- `.documents/WIP/plan-next.md` remains byte-for-byte unchanged and still identifies `R00.A — Identity and publishing declarations` as the single unissued NEXT item.
- `.documents/WIP/plan-roadmap.md` remains byte-for-byte unchanged, retaining every pending Stage 00–13 package and gate; **nothing is pruned** because `SOLO-WO-005` was governance work, not acceptance of `R00.A` or any other roadmap package.
- `.documents/WIP/plan-outlook.md` remains byte-for-byte unchanged and title-only, with no invented ideas.
- The one canonical `.documents/Reports/execution-report.md` is overwritten with a truthful, published closure-run report. Earlier issued work orders remain available **only through Git history**, not a WIP archive.

## 2. Exclusive permanent write permissions

| Path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Phase A: commit the complete Owner-installed `SOLO-WO-006 R00` text unchanged, preserving issuance evidence; Phase B: replace with the exact single-line title-plus-newline in §1 after Phase A is remotely verified. |
| `.documents/Reports/execution-report.md` | Phase B: overwrite the existing single report with factual closure evidence; publish it with the neutral ACTIVE file in the second commit. |
| `.git/**` | Normal safe metadata writes for inspection, staging named paths, **exactly two** ordinary linear commits, fast-forward pushes to verified `origin/main`, and remote read-back under Architect SOP §6B. |

No other repository path may be edited, staged, moved, deleted, created or committed. In particular, `plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, `README.md`, `AGENTS.md`, both SOPs, About, Architecture, `.github/**`, `LICENSE`, toolchain files and application code are **read-only**. No other repository may be written.

**Explicit operation permissions:** commit = **YES**, two scoped commits; push = **YES**, both ordinary fast-forward pushes to Solo `origin/main`; PR/merge = **NO**; dependency install/build/toolchain changes = **NO**; external provider, DNS, secrets and deployment = **NO**; release/tag = **NO**; destructive Git/reset/clean, history rewrite, amend, rebase or force push = **NO**. No extra worktree, branch, clone, nested checkout or shadow working directory. If any agent-created temporary material is necessary, confine it within the verified Solo root under `.temp/`, delete only known agent-created material promptly, and report the cleanup result.

## 3. Preflight and STOP gates

Before any permanent write:

1. Read current `README.md`, `AGENTS.md`, the Architect and Implementation SOPs, About, Architecture, the four WIP files, and the current published execution report. Verify this complete `SOLO-WO-006 R00` in ACTIVE, its `OWNER-ISSUED` marker and the separate, matching Owner handoff.
2. Verify the physical Solo root equals `git rev-parse --show-toplevel` and is not a symlink or junction; verify branch `main`, one worktree, one correct remote `origin`, and local `HEAD` = `origin/main` = GitHub API `main` = `87608ee01bf3a898345aee0d64f1c7e57620f34f`. Verify the parent ignore guard without writing to the parent or siblings.
3. The **only** allowed initial dirty-tree entry is the Owner-installed modification of `.documents/WIP/workorder-active.md`; nothing else may be staged, modified or untracked. The committed baseline has four WIP files and exactly one canonical report outside WIP.
4. Confirm the prior R02 content/report commits are real and linear, the current report records its submitted evidence, and the Owner handoff expressly refers to the Chief Architect's `ACCEPT` and the Owner-approved closure. Read and record the baseline Git blobs of NEXT, ROADMAP and OUTLOOK for subsequent byte-parity checks.
5. Confirm the exact ACTIVE end-state bytes from §1, the destination's regular-file type, Git safety and safe push capability. A missing authorization, changed baseline, unknown file, root/remote discrepancy, unexpected architecture contradiction or unsafe publication is **STOP**, not permission to improvise.

Preserve unknown or Owner-owned material. No deletion or destructive workaround is authorized. If the prior acceptance or separate closure authority cannot be confirmed from this work order together with its matching Owner handoff, STOP and report.

## 4. Ordered execution and publication

**Phase A — Preserve the issued closure order.** Stage **only** `.documents/WIP/workorder-active.md` with the full, unchanged `SOLO-WO-006 R00` content. Make one ordinary commit on `main`; fast-forward push it to the verified Solo `origin/main`. Read back the exact GitHub commit and ACTIVE blob and confirm both match the Owner-installed file. This immutable Git revision preserves the full closure authorization even after ACTIVE is cleared. Do **not** edit ACTIVE or the report before Phase A publication and verification succeeds.

**Phase B — Enact the already authorized, post-acceptance closure.** Replace ACTIVE with exactly the single Markdown heading and newline in §1. Overwrite **only** the existing execution report with the closure-run evidence: work-order ID and handoff, original HEAD, Phase A commit, Chief Architect R02 acceptance reference, why no ROADMAP work was consumed, preservation of NEXT/ROADMAP/OUTLOOK blob IDs, changed paths, Git/worktree/parent/temp checks, exact ACTIVE neutral state, checks performed and not performed, limitations and remote verification. The report must **not** embed its own final commit SHA. Stage **only** ACTIVE and the canonical report; make one ordinary commit and fast-forward push to Solo `origin/main`. Read back final HEAD, ACTIVE and report from GitHub. Confirm local working tree clean, exactly four WIP files, one report outside WIP and zero unknown leftovers. Provide the final report commit SHA and GitHub report URL in chat.

**Failure or STOP:** If Phase A cannot be published and verified, do not clear ACTIVE. If Phase B cannot safely complete, preserve the exact actual state and report it; do not silently reset, revert, rewrite, delete or self-authorize repairs. Publish a factual `STOPPED` or `FAILED` report only when safe and permitted by the standing report-only/reporting rules. Do not claim closure unless the neutral ACTIVE and report have been verified on the remote.

## 5. Independent acceptance gates

Report every condition as `PASS`, `FAIL` or `UNVERIFIED`, with direct evidence:

1. Correct `SOLO-WO-006 R00` identity, unique issued ACTIVE and matching separate Owner handoff; prior R02 Architect `ACCEPT` and Owner closure authority; exact preflight SHA and permitted initial dirty inventory.
2. Exactly two linear, ordinary, remotely verified commits: first publishes the frozen complete closure order; second modifies only ACTIVE and the single current execution report. No other file is changed.
3. Final ACTIVE bytes are precisely `# Foundation Solo — Active Work Order\n` (UTF-8, LF); no `OWNER-ISSUED` order remains active. Full R02 and this closure order remain retrievable at their earlier Git revisions.
4. NEXT, ROADMAP and OUTLOOK Git blobs are unchanged; NEXT retains `R00.A` only, ROADMAP retains all 14 Stages 00–13 and their pending work, and OUTLOOK remains title-only. No roadmap item or NEXT candidate was executed, accepted, removed or promoted.
5. All four WIP files and exactly one current report outside WIP exist; no duplicate plans, reports, task logs or archives, no unexplained dirty files, extra worktrees or outside-Solo changes.
6. The published report truthfully distinguishes GitHub-verifiable commit/file evidence from locally observed safety evidence, includes the existing Chief Architect acceptance, and does not claim build, unit, browser, CI, release or deployment tests that were not performed.

**Completion boundary:** Stop after the closure commits, remote verification and report. Do not begin `R00.A`, draft another local order, change NEXT/ROADMAP, approve your own work, merge, release or deploy. The Chief Architect independently reviews this closure at its exact GitHub revision before any next execution handoff.
