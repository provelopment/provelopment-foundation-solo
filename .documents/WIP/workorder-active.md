# Foundation Solo — Current Work Order

**Work Order ID:** `SOLO-WO-003`  
**Edition:** `R01` — bounded reconciliation following the Chief Architect's `REVISE` disposition of R00  
**Title:** Remove residual reporting-authority contradictions and finalize Phase 0 handoff  
**Approval state:** **CANDIDATE — NOT ISSUED.** This document becomes `OWNER-ISSUED` only when the Owner reviews it, deliberately installs these exact bytes as `.documents/WIP/workorder-active.md` in the verified Solo checkout, replacing the paused `NONE` state, **and** sends an explicit matching `SOLO-WO-003 R01` handoff. The Implementation Agent may not treat this download alone as authority.  
**Target:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — verify actual physical Solo Git root  
**Remote:** `https://github.com/provelopment/provelopment-foundation-solo.git` — public, existing `origin` only  
**Branch:** `main`  
**Expected starting HEAD:** `23bb5daa33c1813403fb3f0c44941f0cc281ab2f` (independently checked remote `main` at drafting)  
**Phase:** Phase 0 correction only. **STOP and PAUSE** on delivery. No Phase 1 authorization.

## 1. Architect disposition and purpose

The Architect independently inspected the R00 report at `.documents/Reports/execution-report.md` on GitHub and verified the R00 two-commit sequence (`cc98b717ce96bb75c95d63efff3aaae574c4493e` → `23bb5daa33c1813403fb3f0c44941f0cc281ab2f`), the single report file, WIP triad, and published entry points. R00 was marked **REVISE** because a few residual statements in the published documents still give conflicting instructions about whether evidence belongs in `workorder-active.md` and whether a report-only commit/push requires a normal active work order.

**Objective:** Remove only these contradictions, keep the already-working GitHub-first report procedure unambiguous for a freshly initialized agent, publish a replacement factual report for this R01 run, and return WIP to `NONE — PHASE 0 PAUSED; awaiting Architect acceptance`. A completed R01 is not self-acceptance; Phase 0 closes only after independent Architect review.

## 2. Frozen approved scope — exact allowed paths

| Path | Authorized change |
| --- | --- |
| `README.md` | Reconcile the blanket sentence in **Roles and authorization** that currently says any commit/push requires an active work order: make clear that implementation/source changes still require Owner-issued scope and handoff, **except** the already-approved standing permission to overwrite/commit/push **only** `.documents/Reports/execution-report.md` after safety checks. Point to `AGENTS.md` and both SOPs. Do not alter product rules. |
| `AGENTS.md` | In §2A and any competing authorization wording, distinguish **implementation changes** (Owner-issued active order required) from the **report-only** exception, using one clear rule and no new authority. Retain mandatory reading path and all permissions restrictions. |
| `.documents/SOP/sop-architect.md` | In §6 replace the stale requirement that a work order provide reporting fields **in ACTIVE** and the stale execution **Sequence** instructing the agent to supply its primary evidence **in ACTIVE**: require primary evidence in `.documents/Reports/execution-report.md`, with optional short WIP status/evidence only if the exact issued order permits that edit. In §6 initialization wording and any other blanket no-commit/no-push language, explicitly preserve §7A's standing report-only exception. Ensure §6, §7A, §8 and §9 agree without changing §6B's sole version-control authority. |
| `.documents/SOP/sop-implementation.md` | In §4 explicitly distinguish the only authorization for **implementation** from the standing **report-only** exception in §5C. Update any stale references to evidence in ACTIVE (including the final handoff checklist in §11) so the mandatory report is primary and WIP evidence optional only if the order authorizes it. Reconcile §3, §4, §5C, §8 and §11 without creating a competing Git procedure or weakening STOP rules. |
| `.documents/WIP/workorder-active.md` | Owner installation of this exact issued R01 text is the **only** permitted pre-run tracked change. Keep its approved scope frozen during execution. After successful content publication, neutralize to `NONE — PHASE 0 PAUSED; awaiting Architect acceptance` as part of the final reporting commit. On failure, preserve the exact issued order and report the STOP instead of claiming closure. |
| `.documents/Reports/execution-report.md` | **Overwrite the existing one report** with truthful R01 outcome, original R00 `REVISE` reason, changes, tests/validations, initial and content commit SHAs, final WIP state, clean-tree/remote verification, limitations and stop/pause. Do not create additional reports or embed the report's own final commit SHA inside its content. |
| `.git/**` | Ordinary bounded Git operations to create **two** new fast-forward commits and push `main` to the existing `origin`, verify remote contents and clean state. No new remote/branch. |

**Explicitly NOT authorized:** modifying `plan-next.md`, `plan-outlook.md`, `About/`, `Architecture/`, other `.documents/` files, `LICENSE`, `.github/`, `.gitignore`, `.editorconfig`, `.gitattributes`, application/source/tests/manifests, dependencies, CI, deployments, security/account settings, GitHub issues/PRs, other repositories, or any architectural/product contract. The current NEXT and PLAN already correctly state PAUSED; leave them byte-for-byte unchanged. No worktree/clone/shadow checkout, new report/plan/workorder file, force push, amend/rebase, destructive reset/clean, stash, external temporary directories, or history rewrite.

## 3. Mandatory preflight — STOP if any gate fails

1. Confirm the Owner intentionally installed this exact `SOLO-WO-003 R01` as the **sole** `.documents/WIP/workorder-active.md` and provided a matching explicit R01 handoff. Read `README.md`, `AGENTS.md`, both SOPs, the three WIP slots, the report, and necessary architecture constraints.
2. Verify Solo is the physical repository top level, not the parent, a symlink or junction; `main` is checked out; local and `origin/main` equal expected `23bb5daa33c1813403fb3f0c44941f0cc281ab2f`; no unexpected divergent remote revision; exactly one worktree, exactly one `origin`, no local-only work or untracked/ignored extras. The **only** permitted pre-run dirty file is the Owner-installed `workorder-active.md`.
3. Verify `provelopment-root` still ignores Solo via a committed `/00.foundation-solo/` rule and parent tracks no Solo content; **do not change or clean parent state**.
4. Confirm `.documents/WIP` has exactly three files (`workorder-active.md`, `plan-next.md`, `plan-outlook.md`) and `.documents/Reports` has exactly **one** existing `execution-report.md`. Confirm that the reported R00 two-commit publication exists in Git history and review the exact installed-order diff to guard against unrelated Owner edits.
5. Verify CLI/MCP GitHub authentication, safe use of the existing Git credential helper and Git tooling, no unsanctioned external writes, safe temp/cache containment and no destructive hook/signing behavior. If any privacy/publication risk, unexplained data, unknown Owner-owned files or repository mismatch exists, STOP and report; do not erase data or improvise.

## 4. Implementation and publication sequence

**A — Narrow document reconciliation.** Edit only the four entry-point/SOP paths listed in §2. Make the authority hierarchy logically consistent: an Owner-issued order permits bounded **implementation**; a **standing, independent report-only exception** permits only the one report's safe commit/push when no implementation order exists; the single GitHub report is the primary evidence; ACTIVE is frozen during execution and may contain evidence only if that exact order expressly permits. Inspect the full text for remaining direct contradictions. Keep `sop-architect.md` §6B the single Git procedure. Validate that all relative document links resolve in the published tree.

**B — First commit.** Stage exactly `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md`, `.documents/SOP/sop-implementation.md`, and the Owner-installed `.documents/WIP/workorder-active.md`. Review the complete staged diff; no other staged paths. Create **one** ordinary commit preserving all previous history and fast-forward push to `origin/main`. Verify GitHub's actual HEAD and the five changed files, and record this content SHA. Do **not** overwrite the report or reset ACTIVE before this issued-order snapshot is published.

**C — Report and pause.** After B is verified, overwrite **only** `.documents/Reports/execution-report.md` with factual `SOLO-WO-003 R01` completion evidence. Reset the **same** `workorder-active.md` file to a brief `NONE — PHASE 0 PAUSED; awaiting Architect acceptance` state. Do not touch NEXT or PLAN, which already state paused. No Phase 1 proposals or implementation.

**D — Final report commit.** Stage **only** `.documents/Reports/execution-report.md` and `.documents/WIP/workorder-active.md`. Create **one** ordinary final report/closure commit; fast-forward push `main`. Independently read back the report and relevant entry points/SOPs from GitHub, verify the exact final remote HEAD, both expected new commits and original ancestor chain. Verify report folder contains **one** file, WIP **three**, final Solo `git status --porcelain` empty, one worktree, no agent-created temporary files, unchanged parent/siblings. Report final SHA and GitHub report link **in chat only**, not embedded in the report.

**On any STOP/FAILED condition:** preserve existing work and issued authority; do not claim Phase 0 accepted. Overwrite/publish the one factual STOP report **only if doing so is safely possible** under standing report-only rules and the explicit permissions in this order; otherwise report exactly why publication is impossible in chat, without creating new files or deleting/overwriting unknown content. Do not execute another task.

## 5. Acceptance criteria

- No residual contradiction between the current SOPs/entry points on primary report location, authorized ACTIVE editing, read-only reporting, or standing report-only commit/push.
- One fresh external agent can read `README.md` and `AGENTS.md` and discover the complete current authority, role-specific SOP, WIP state, current report, actual repo, and pause rule without needing prior conversation.
- The one existing report is overwritten with an accurate `SOLO-WO-003 R01` report and is published on GitHub; WIP remains **exactly three files**, with `ACTIVE=NONE`, `NEXT=NONE — PAUSED`, PLAN outlook intact.
- Exactly two new ordinary commits with changed paths limited to §2, correct parentage, source documents and report remotely retrievable, no added application files; no new remote, worktree, clone or external write.
- Local working tree **clean** after the final push; `main` aligned with `origin/main`; no temporary artifacts left and no other repository changed. Local cleanliness is evidenced by actual agent commands and explicitly distinguished from GitHub-verifiable facts.
- No application functionality, framework, build/test claims, new architecture/schema decisions, merge, release or deployment; no Phase 1 start.
- The final report commit SHA may be reported in the final chat but not within its own report body. Any failed check is openly reported and Phase 0 remains pending acceptance.

## 6. Authorization ledger and stop rules

**Upon exact Owner issuance, authorized:** narrow edits to the four named docs; preserving/committing this issued work order; resetting only ACTIVE on success; overwriting the same report; precisely two ordinary fast-forward commits/pushes on existing Solo `main`; read-only verification of the GitHub repository, parent guard and relevant source documents; safe cleanup of disposable agent-owned files *inside Solo only* if created.

**Not authorized:** any work in other repositories; changing provider settings, account or security settings; unknown-data cleanup; extra checkout/worktree/clone; new branch, PR, merge, force-push, release, deployment, toolchain, package installation, or Phase 1 decision/work. The agent must maintain all writes within Solo. If scope unexpectedly expands, the Owner-installed order is missing/unmatched, another revision appears on remote, untracked or unknown material obstructs a clean tree, a push fails or a privacy exposure is suspected, STOP, preserve evidence, and report the smallest required intervention.

## 7. Architect closure condition

The Architect will first read the newly overwritten `.documents/Reports/execution-report.md` **directly from GitHub**, verify the exact published R01 revision, inspect the diffs and give **ACCEPT / REVISE / STOP**. Only on `ACCEPT` is Phase 0 fully **CLOSED**, with Solo staying **PAUSED**. The Architect does **not** draft the Phase 1 work order, initiate new development, or otherwise resume the project without a further Owner instruction. This document is an Architect-authored candidate; installing it and sending the matching handoff is solely the Owner's decision.
