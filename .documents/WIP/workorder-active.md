# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-005`  
**Edition:** `R02` — bounded correction following Chief Architect `REVISE` on R01  
**Title:** Correct four-slot governance terminology, execution-document purity and report accuracy  
**Approval marker:** `OWNER-ISSUED`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `b20a77a380a279053e924b0f742de135c1e94a4f`  
**Matching Owner handoff:** `SOLO-WO-005 R02 — Implementation Agent Handoff`  
**Work class:** Documentation-only, precise correction to the completed R01 migration.  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP`; separate Owner-authorized WIP closure.

## 1. Objective and findings to correct

Correct the independently verified, bounded defects in the published `SOLO-WO-005 R01` result without reopening its four-slot migration or changing Solo's product or technical contracts:

1. `AGENTS.md` still references the obsolete `PLAN` name, and labels `workorder-*.md` as a *candidate* execution type. Its description of what the Architect provides uses *draft* terminology.
2. Architect SOP §2, §6 and §7A still refer to candidate/draft work orders. The Architect must instead provide one **complete file named `workorder-active.md`** and a **separate, directly copyable Markdown Implementation Agent prompt** for Owner review. The Owner is the approval/installation/handoff gate; never put redundant instructions in a work order attempting to prove who installed it.
3. `plan-next.md` contains general authorization/handoff commentary in its prerequisites, contrary to its single-candidate, execution-only purpose.
4. Stage 00's exit gate in `plan-roadmap.md` uses the disallowed expression *work-order candidate*; it must name a complete work order for Owner review instead.
5. The published R01 execution report's last limitation falsely states that frozen ACTIVE contains `NOT ISSUED — OWNER REVIEW REQUIRED`. The R01 ACTIVE blob actually has an `OWNER-ISSUED` approval marker. The next canonical report must explicitly correct the record and state accurate evidence.

The R01 implementation commit is `6b51484edecb7382b8de302591a523012556e159` and its report commit is `b20a77a380a279053e924b0f742de135c1e94a4f`. The rest of that migration remains preserved. No stage is accepted or removed, and no NEXT item is executed by this documentation correction.

## 2. Exclusive permitted permanent paths

| Path | Authorized change |
| --- | --- |
| `AGENTS.md` | Replace obsolete `NEXT and PLAN` with accurate ROADMAP/NEXT/OUTLOOK terminology; remove the *candidate execution document* description of `workorder-*.md`; describe the Architect's output as a complete `workorder-active.md` and copyable Markdown handoff, not a draft work order. Keep the Owner-issued ACTIVE + matching handoff execution gate intact. |
| `.documents/SOP/sop-architect.md` | In §2, §6 and §7A, remove candidate/draft terminology **when describing work orders**; specify the exact file name and matching prompt as Architect deliverables for Owner review; keep Owner-only issuance and independent review. Correct its status/authority header only if needed to avoid describing the currently adopted SOP as an unaccepted review draft. No other SOP contract may change. |
| `.documents/SOP/sop-implementation.md` | Where §4 describes draft work orders, replace that description with the accurate rule that files outside the Owner-issued ACTIVE and its matching handoff never grant authority. Correct its status/authority header only if needed to avoid describing the currently adopted SOP as an unaccepted review draft. Preserve all implementation, report and Git gates. |
| `.documents/WIP/plan-next.md` | Replace only the prerequisites paragraph's standing authorization/handoff explanation with task-specific dependency readiness (R00.A has no prior stage prerequisite). Preserve its single R00.A objective, deliverables and acceptance. |
| `.documents/WIP/plan-roadmap.md` | In the Stage 00 exit gate only, replace *work-order candidate* with *complete work order prepared for Owner review*. Preserve the 14 stages, all work packages, all other text and their pending status. |
| `.documents/WIP/workorder-active.md` | Preserve this exact Owner-installed R02 work order as the sole issued order; publish its frozen bytes unchanged in the content commit. Do not neutralize or append commentary before independent review. |
| `.documents/Reports/execution-report.md` | Overwrite with a new, factual R02 execution report, explicitly correcting the R01 report's false `NOT ISSUED` claim and showing verified diff/validation/publication results. |

No changes are authorized to `README.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, `.documents/WIP/plan-outlook.md`, application files, dependencies, workflows, `.github/**`, `LICENSE`, other repositories or other files. If those require edits for consistency, STOP and report the exact need rather than widening scope.

## 3. Exact content requirements

1. All four WIP names and their responsibilities must remain accurate: ROADMAP (remaining stages), NEXT (one bounded next candidate), ACTIVE (one issued work order), OUTLOOK (optional ideas only). The only work-order filename in WIP is `workorder-active.md`. No category of *draft*, *provisional* or *candidate* **work order** is recognized. The term *candidate* remains appropriate for the `plan-next.md` item, which is not a work order.
2. The Architect supplies a fully written `workorder-active.md` and a separate directly copyable Markdown prompt with the identical `SOLO-WO-*` ID and edition. The Owner reviews the two together and alone controls installation and handoff. Maintain the existing matching handoff requirement in the SOPs; do not insert unnecessary conditional-installation or origin-of-file boilerplate into this or subsequent work orders.
3. `plan-next.md` and `plan-roadmap.md` must continue to contain only unfinished execution/stage content. Remove only the identified inappropriate or inconsistent prose; do not add process rules, acceptance history, release notes or generic governance to those files.
4. The single current report must correctly distinguish the R01 source-verified `OWNER-ISSUED` marker from the inaccurate R01 report statement. Identify the relevant prior commit and actual R02 evidence. Do not silently rewrite published Git history or claim R01 was never authorized.
5. Preserve in substance the already-approved one-Entity EFS, five-level property/location/graphics cascade, full locale tags, safe content/publication, configurable base URL, public/unlisted routing and discovery, strict Hexagonal Architecture, simple contact form and `mailto:`, excluded advanced forms, Apache-2.0 and separate release/deployment authority. Preserve the four-file WIP rule, single canonical execution report, one-root/one-worktree boundary and standing report-only exception.

## 4. Preflight, authorization, safety and STOP gates

Before permanent file writes, read `README.md`, `AGENTS.md`, both SOPs, the current product/architecture contracts, four WIP files and the current GitHub execution report. Verify this exact R02 `OWNER-ISSUED` scope in ACTIVE and the separately supplied matching `SOLO-WO-005 R02 — Implementation Agent Handoff` prompt. Verify the actual non-symlink/non-junction Solo root, branch `main`, exactly one worktree, exactly one remote `origin` naming `provelopment/provelopment-foundation-solo`, and that local HEAD, `origin/main` and GitHub `main` all match the required starting SHA before the first content commit.

The only permitted initial dirty entry is the Owner's replacement of R01 ACTIVE with this R02 text. Four WIP files and one current report must already be tracked; no unrelated modified, staged or untracked material is authorized. Check the parent ignore guard without changing the parent. Validate safe credential use without exposing secrets. STOP on any unexplained mismatch, missing matching handoff, remote advancement, conflicting instruction, unapproved path, unknown Owner material, unsafe operation or source-content ambiguity.

No extra Git worktree, clone, nested checkout, new branch or shadow copy. No dependency install, build, test-runner introduction, provider call, secret, PR, merge, release, deployment, force push, rebase, amend or destructive reset/clean. All agent-created scratch, caches and outputs stay strictly within Solo `.temp/` and must be cleaned if safely agent-owned. Do not delete unknown files. Ordinary scoped Git staging, exactly two commits and two fast-forward pushes to `origin/main` are expressly authorized. Apply Architect SOP §6B rather than inventing another Git procedure.

## 5. Execution and verification

**Phase A — bounded content correction.** Make only the precise edits in §2, preserving the unchanged product and documentation contracts. Check the complete diff against the six authorized non-report paths. Confirm the active order remains byte-for-byte frozen; ROADMAP still holds all Stages 00–13; NEXT still holds R00.A; OUTLOOK is unchanged; WIP remains exactly four files. Run a whole-document search for stale `PLAN`, draft/candidate work-order terminology and inaccurate status claims, distinguishing allowed historical quotations in the frozen ACTIVE from governing instructions. Verify affected Markdown links and section references. Stage only the six non-report paths, commit once and fast-forward push to the verified Solo remote. Read back the published exact file blobs and commit.

**Phase B — mandatory report correction.** Overwrite only `.documents/Reports/execution-report.md`. Report exact starting/content revisions, changed paths and original/updated wording, checks and results, immutable ACTIVE, local safety and cleanup evidence, remote readback, all unperformed tests, and the explicit correction to the false R01 report statement. Stage only this report, commit once, fast-forward push, and independently read back the current report on GitHub. Give the report commit SHA only in the handoff response, not inside the report. Leave ACTIVE issued; do not close or move to another task.

If STOP or FAILED, preserve the Owner-installed scope and all pre-existing material. Publish only a truthful report if safe under the report-only rules, and disclose blockers. No hidden recovery edits or widened permissions.

## 6. Independent acceptance checklist

The Implementation Agent reports each item `PASS`, `FAIL` or `UNVERIFIED` with concrete evidence; only the Chief Architect independently disposes it:

1. Correct R02 identity, exact starting SHA and separate matching Owner handoff; one-root, one-worktree, correct remote and expected dirty-tree evidence.
2. Only the six authorized non-report paths and the one report changed; two ordinary fast-forward published commits; R02 ACTIVE remained byte-identical in the content commit.
3. `AGENTS.md` and both SOPs consistently require a complete `workorder-active.md` artifact plus a separately copyable Markdown handoff for Owner review; no *candidate/draft/provisional work-order* category remains in operative rules, and Owner approval controls installation/handoff.
4. `AGENTS.md` contains no obsolete `PLAN` role; all WIP names remain correct and exactly four tracked WIP files and one report outside WIP remain.
5. NEXT contains only R00.A execution definitions and task-specific prerequisites; ROADMAP retains exactly all pending Stages 00–13 with its Stage 00 exit wording corrected, no other roadmap changes; OUTLOOK unchanged.
6. Report accurately corrects the R01 marker error and does not claim any unexecuted build, test, browser, CI, release or deployment verification.
7. All approved EFS, quality, provenance, authorization, Git, report and closure contracts are preserved; affected cross-references/links work; no unsafe cleanup, outside-repo write or other unexpected change occurred.
8. Remote final HEAD, exact GitHub diff, current report and local clean worktree agree (or the exact limitation is clearly reported). No self-acceptance and no premature WIP closure.

**Closure:** The Agent stops after publishing the evidence. The Architect independently returns `ACCEPT`, `REVISE` or `STOP` for this exact revision; only a further express Owner authorization may subsequently neutralize ACTIVE or reconcile ROADMAP/NEXT. This work order does not authorize that closure step.
