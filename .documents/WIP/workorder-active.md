# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-007`  
**Edition:** `R08`  
**Title:** Owner-corrected temporary-workspace freedom and mandatory task-owned garbage cleanup  
**Approval marker:** `OWNER-ISSUED` **only upon exact Owner installation and matching direct handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `e7323f23e7aa221a5e0d14394d2e1fcff5a51559`  
**Matching handoff:** `SOLO-WO-007 R08 — Implementation Agent Handoff`  
**Work classification:** Narrow corrective continuation of the existing `R00.A` documentation/governance work; no new ROADMAP package, code, release, deployment or closure.  
**Review leading to this edition:** Chief Architect `REVISE` on R07's **policy fidelity to clarified Owner direction**, not on the correctness of its published three-file content commit and separate report commit.  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP` on published evidence; R00.A closure, ROADMAP pruning and next-candidate selection remain separately Owner-authorized.

> **Issue boundary:** This is the **only** current replacement work order. The Owner must install the exact approved R08 bytes at `.documents/WIP/workorder-active.md` and separately deliver the matching R08 handoff. Earlier R07 and local R06 handoffs are not executable authority. The Implementation Agent may publish but **must not edit, append to, dispose of or clear** this frozen ACTIVE text. The approval marker in an unattached copy is not itself authorization.

**Frozen Approved Scope:** §§1–7, including all exact replacement text, ledger, gates and exclusions, are one indivisible Owner decision.

## 1. Clarified Owner decision and purpose

The Owner **does not restrict the Implementation Agent to Solo `.temp/` for all temporary work**. The agent must be able to use, without per-file/per-run permission, **both** (a) the existing `<VerifiedSoloRoot>/.temp/` and (b) **Cline's own temporary workspace(s) made available for the active agent run**, even when the Cline area is under OS `%TEMP%` outside Solo. Both are authorized disposable working locations for ordinary temporary files, subdirectories, tool outputs, logs, downloads, fixtures, generated intermediates and caches needed to perform an otherwise authorized task. Use the tool normally; automatic command-output capture is not a reason to refuse work.

The Owner's **non-negotiable requirement is cleanup**: files and folders **the agent or its run creates** become garbage when their task purpose ends and **must be safely removed then**, normally at the end of the run, rather than accumulating. This includes identifiable run-specific Cline command-capture/diagnostic logs when safely accessible. The agent may inspect and delete such task-owned files in either authorized temporary area. Do not delete unknown, shared, pre-existing Owner-owned or unrelated material. Do not make a blanket exception that treats all Cline logs as permanent and untouchable. If an artifact remains genuinely required, report the exact relevant purpose and a concrete cleanup trigger; if an obsolete task-owned file is inaccessible or cannot be cleaned, disclose it as an **unresolved cleanup failure**, not `PASS` or `UNVERIFIED` masquerading as success. Do not prevent authorized implementation solely because Cline uses its own temp directory.

**No extra repository mechanisms whatsoever.** The one verified Solo checkout remains the only source working tree. No Git worktrees (linked or otherwise), clones, nested repositories, secondary checkouts, branch-specific checkout directories or shadow source trees in **either** temporary area or anywhere else. Neither temporary location grants permission to modify permanent files outside Solo, write to parent/sibling repositories, create another task, bypass the work-order ledger, or merge/release/deploy.

`.temp/` is already ignored in the root `.gitignore` at line 20; **do not edit `.gitignore`**. No `.temp/` item may be tracked, staged or force-added. Cline temp outside Solo is not a place to put a second Git repository or permanent deliverable. Keep secrets out of logs and Git; never print secrets just because temporary output is permitted. The Cline location is identified from the actual Cline execution environment, **not** by granting unrestricted access to all OS temp, home, parent or sibling directories.

R07's published SOP wording still prohibited agent-elected Cline scratch, required redirectable output into Solo and forbade inspection/deletion of host-managed logs. Architect SOP §9, Implementation SOP §5 and its checklist, AGENTS.md and README carry further contradictory Solo-only wording. This R08 correction makes the complete live governance consistent with the clarified Owner requirements **without weakening the single-checkout prohibition or the separate permanent-write authority**.

## 2. Exact primary SOP replacement text

### 2.A Architect SOP §6A — replace section through, not including, `## 6B.`

Replace the entire `.documents/SOP/sop-architect.md` §6A with **exactly** this Markdown:

```markdown
## 6A. Bounded repository and temporary-workspace governance

**One permanent repository and one checkout.** All permanent Solo source, documentation and Git writes remain within the single verified Solo repository and the exact Owner-authorized permanent-path ledger. Parent and sibling repositories are read-only. Never create or use another Git worktree, clone, nested repository, secondary checkout, branch-specific checkout directory or shadow source tree anywhere, including any temporary workspace.

**Both Owner-permitted temporary workspaces.** For an otherwise authorized run, the Implementation Agent has standing discretion to create, read, modify and remove disposable task files and directories in either `<VerifiedSoloRoot>/.temp/` **or** the actual temporary workspace(s) Cline provides for that agent run, including Cline-managed areas under OS `%TEMP%`. This permission includes logs, output capture, scratch, caches, downloads, fixtures, exports and intermediate files; no per-file or per-order scratch grant is needed. The Cline exception does not authorize arbitrary OS temp folders, other repositories, permanent deliverables outside Solo, or a second source checkout. No command-runner restriction may be imposed solely because Cline creates its normal temporary output.

**Git exclusion.** Solo `.temp/` is ignored by `.gitignore` and must never be tracked, staged, force-added or committed. Verify this for actual contents. Cline temp is disposable non-repository workspace and must not be used for a Git repository or permanent task artifact. The scratch permission is not source-code, dependency, provider, merge, release or deployment authority.

**Mandatory cleanup, not workspace prohibition.** The agent is responsible for identifying and promptly deleting the temporary files and directories created for its run **when their purpose ends**, ordinarily before completing that run. This responsibility covers accessible, identifiable task-specific Cline capture and diagnostic logs, agent-selected intermediates and Solo `.temp/` contents alike. Earlier-run residue may be cleaned only when its ownership and obsolescence are positively established. Do not accumulate garbage. Material genuinely required after the run is an exception based on continuing need, not convenience: state its location/category, purpose and definite cleanup event in the canonical report. Never broadly erase a shared temp root or delete unfamiliar, pre-existing Owner-owned or unrelated content.

**Cleanup evidence and blockers.** Record where the run's temp artifacts were created, what was removed, any necessary retention and any inaccessible or unremovable task-owned garbage. An unverifiable *host* cleanup fact is reported as such; an identified obsolete file left behind is an **unresolved cleanup failure**, not a passing gate. A platform-generated log does not automatically violate repository write confinement, but neither does being platform-generated permanently exempt task-owned garbage from cleanup. Do not falsely claim that a clean Git status proves temp cleanup. If a Cline-owned file cannot be identified or safely accessed, preserve unknown content, report the limitation without exposing sensitive paths and request an appropriate Owner/platform cleanup mechanism; normal work is not prohibited merely by using Cline's temp area.

**Architect review and authority.** Work orders define permitted permanent edits and end-to-end evidence, not extra permission for ordinary temporary work in either approved location. Independently check no additional checkout, no unintended permanent-path writes, no tracked Solo `.temp/` files, and truthful cleanup or retention evidence. Architect SOP §6B is the one version-control procedure; no competing Git rules belong here. The parent repository ignore guard remains read-only and intact.
```

### 2.B Implementation SOP §5A — replace section through, not including, `## 5B.`

Replace the entire `.documents/SOP/sop-implementation.md` §5A with **exactly** this Markdown:

```markdown
## 5A. Mandatory one-checkout and temporary-workspace cleanup discipline

**One repository checkout, always.** Use the verified Solo root as the only editable repository. Never create or use linked Git worktrees, additional checkouts, cloned or nested repositories, shadow source trees or branch-specific checkout directories, including inside either temporary workspace. Only the current Owner-issued task authorizes permanent edits; the standing report-only exception stays narrow.

**Work freely in either temporary area.** The Owner permits use of both `<VerifiedSoloRoot>/.temp/` and the temporary workspace(s) actually provided to this run by Cline, including Cline-managed temporary locations outside Solo. You may create, read, write, reorganize and remove the temporary files and folders needed for the task, including tool caches, downloads, command capture, logs, generated output, test fixtures and intermediates, without asking for additional scratch permission. Use the available Cline runner normally. Do not treat other OS temp folders, user-profile folders or unrelated workspaces as a general-purpose destination; the exception is the identifiable Cline-provided task area.

**Keep scratch out of Git and out of permanent scope.** Solo `.temp/` is ignored by root `.gitignore`; confirm that no content beneath it is tracked, staged or committed, and never use force-add. Neither scratch area is a second codebase or a place for permanent deliverables. Permanent source and documentation changes must remain within the authorized Solo path ledger. Do not modify parent or sibling repositories, create another worktree/checkout or expose secrets in command output.

**Clean when done, normally before ending the run.** Keep track of the disposable files/directories created by your actions and by the current Cline execution where task ownership is identifiable. Remove each one as soon as it is no longer needed, including agent-owned Cline capture logs when accessible. Before declaring a run complete, inspect the relevant task-owned parts of **both** temp areas and clean no-longer-needed material. Do not leave garbage for later sessions. Do not use `git clean`, delete an entire shared temp root, or remove unfamiliar, pre-existing Owner-owned or unrelated files; delete only positively identified task-owned material. A genuinely necessary item may remain only with its purpose and a specific future cleanup trigger recorded in the canonical report. If known obsolete task-owned files cannot be removed safely or are inaccessible, report an outstanding cleanup failure rather than claiming full cleanup.

**No artificial Cline STOP.** Cline's ordinary host-managed command-capture logs and other task output within its supplied temp workspace are permitted work artifacts, not an automatic outside-root violation and not a reason to avoid using the runner. Host-generated files are not exempt from the cleanup obligation where identifiable and controllable. Do not force changes to external IDE/OS settings just to redirect an already permitted Cline temp path. If an actual permanent outside-Solo write, a second checkout, deletion risk to unknown material, suspected secret exposure or unbounded/unrelated temp target is discovered, STOP and report rather than conceal it.

**Record evidence honestly.** In the one canonical execution report, state the Solo `.temp/` Git-ignore/tracked/staged checks, the run's Cline temp area by safe category (not private full path), inventories sufficient to identify task-owned files, cleanup actions and results, any retained item with purpose/trigger, and any unresolved garbage. Distinguish local observations from GitHub evidence. A clean Git working tree does not substitute for cleaning disposable files and folders.
```

## 3. Required narrow alignment edits — no contradictory instructions left live

Perform **only** these additional edits, without changing unrelated text:

1. **Architect SOP §7A**, replace only its `**Clean Git expectation.**` paragraph with:
   > **Clean Git expectation.** A normally completed run leaves `git status --porcelain` empty, one verified Solo checkout, no **unneeded task-owned temporary garbage** in either Solo `.temp/` or its Cline-provided temporary workspaces, and all changed documentation including the report committed and visible on GitHub. Any genuinely still-needed temporary material must be disclosed with a cleanup trigger; known unremovable obsolete task material is an unresolved cleanup failure. A push never implies merge, release or deployment.
2. **Architect SOP §9**, replace only its `**Workspace confinement:**` paragraph with:
   > **Workspace confinement:** All **permanent** file creation, edits, deletes and Git changes are confined to the verified Solo root and the Owner-issued work-order paths. The **only standing disposable workspace exceptions** are Solo `.temp/` and the temporary areas actually provided by Cline to the active task, governed by §6A; normal task-owned cleanup in those areas is permitted and required. Do not mutate other repositories or unrelated OS/user directories. Do not create a second checkout/worktree anywhere. Inspect tool effects and enforce the Owner's real permission boundaries; written instructions supplement rather than replace them.
3. **Implementation SOP §5**, replace only its opening `**Strict workspace write confinement:**` paragraph with:
   > **Strict permanent-write confinement:** Permanent source, documentation, configuration and Git changes belong only within the verified Solo root and the current order's exact path-and-operation ledger. For disposable task files, **both** Solo `.temp/` and the active Cline-provided temp area are permitted under §5A, with required cleanup. No parent/sibling repository writes or additional worktrees/checkouts. Inspect tool effects, symlinks and destinations; an unknown or unrelated write target is not authorized. The Owner's permissions and the SOP remain complementary safeguards.
4. **Implementation SOP §5C**, in `**Clean-tree gate and the local-only exception.**`, replace **only the first sentence** with:
   > Normal completion leaves `git status --porcelain` empty, one verified checkout, no obsolete task-owned scratch in Solo `.temp/` or Cline's task temporary area, and all changed documentation including the report committed and present on GitHub; any justified temporary retention or known cleanup failure is documented under §5A.
   Preserve the rest of that paragraph, including the local-only code branch exception and STOP rule, unchanged.
5. **Implementation SOP final checklist**, replace only the line beginning `- [ ] All agent-created temporary/cache/output paths were confined to Solo;` with:
   > - [ ] Task-owned temporary/cache/output files used only the authorized Solo `.temp/` or current Cline-provided temporary workspace; no obsolete task-owned files/folders remain without an explicitly reported cleanup failure, and any justified retention has a recorded cleanup trigger.
6. **`AGENTS.md` §2**, replace the `**Temporary workspace restriction:**` paragraph with:
   > **Temporary workspace and cleanup:** One verified Solo checkout only: no extra Git worktrees, clones, nested repositories, shadow checkouts or second source trees anywhere. The Implementation Agent may freely use Solo `.temp/` **and** the active Cline-provided temporary workspace, including Cline's OS-temp task area, for disposable task materials. Solo `.temp/` is excluded from Git; never track or commit its contents. The agent must promptly delete identifiable task-owned files and folders when no longer needed, normally by run end, including accessible Cline task logs. Do not delete unknown, shared or Owner-owned material; report any necessary retention with a cleanup trigger and any unresolved garbage. Architect SOP §6A and Implementation SOP §5A govern details.
7. **`AGENTS.md` §2A**, replace the `**Clean tree, honestly reported.**` paragraph with:
   > **Clean tree and cleanup, honestly reported.** A completed run leaves empty `git status --porcelain`, one checkout, no unnecessary task-owned temporary files or folders in Solo `.temp/` **or** the task's Cline temporary area, and all required documentation published. Disclose justified retained items and their cleanup triggers, or any unresolved cleanup failure. Unknown/Owner-owned content must not be deleted merely to manufacture a clean state; STOP if safe cleanup would require destructive action against such material.
8. **`AGENTS.md` §4**, replace the `**Implementation-agent confinement:**` paragraph with:
   > **Implementation-agent confinement:** Permanent project writes and Git mutations are confined to the verified Solo root and exact Owner-issued paths; parent and sibling repositories are read-only, and the parent ignore guard is outside Solo permissions. The only standing **temporary-workspace** exceptions are Solo `.temp/` and Cline-provided task temp, with the cleanup rules in both SOPs. Neither permits a second checkout/worktree or unrelated permanent write. Report uncertain ownership or cleanup rather than deleting unknown material.
9. **`README.md`**, replace only the paragraph beginning `**Strict single-root discipline:**` with:
   > **Strict one-checkout discipline:** No additional Git worktrees, shadow checkouts, nested clones or alternate source trees, including within temporary folders. For otherwise authorized work, Implementation Agents may use **both** Solo's Git-ignored `.temp/` and Cline's own task temporary workspace without separate per-file permission. Disposable, identifiable task-owned files and folders must be removed when no longer needed, ordinarily by the end of the run; never accumulate garbage or delete unfamiliar/Owner-owned material. Permanent project writes remain confined to approved Solo paths. See both SOPs for cleanup and evidence rules; the parent `.gitignore` remains outside Solo agent authority.

All listed replacements must be applied exactly; no independent paraphrase, unlisted policy change, competing Git procedure or permanent architecture adjustment. **Do not edit `.gitignore`**; `.temp/` is already excluded. **Do not edit AGENTS/README headings, navigation, product claims or other sections** except the exact paragraphs above.

## 4. Exact permissions and baseline

| Path or location | Authorized operation |
| --- | --- |
| `.documents/SOP/sop-architect.md` | Replace **§6A** exactly with §2.A, and only the §7A clean-Git and §9 confinement paragraphs with §3.1–2. Keep §6B and other content intact. |
| `.documents/SOP/sop-implementation.md` | Replace **§5A** exactly with §2.B, the §5 opening paragraph, §5C first clean-tree sentence, and final checklist line with §3.3–5. Leave §5B Git reference and the rest of §5C intact. |
| `AGENTS.md` | Replace exactly three named paragraphs (§2 temporary, §2A clean-tree, §4 confinement), §3.6–8; nothing else. |
| `README.md` | Replace exactly the single named one-checkout paragraph, §3.9; nothing else. |
| `.documents/WIP/workorder-active.md` | The Owner installs this exact R08 text; agent may only stage/publish it **unchanged**, never edit, append evidence or clear. |
| `.documents/Reports/execution-report.md` | Overwrite only this canonical report in separate Phase B, with factual R08 evidence and cleanup results. |
| `.git/**` | Only minimal normal Git metadata for two scoped commits, authorized fast-forward pushes, remote readback; Architect SOP §6B applies. |
| `<VerifiedSoloRoot>/.temp/` | Standing permission to use and clean **agent-owned temporary work**, without staging/tracking, subject to §§1–3. |
| Cline-provided temp areas **actually assigned to the current task** | Standing permission to use and safely clean identifiable task-owned temporary material, including Cline capture logs and verified obsolete historical-run garbage; no unrelated OS temp traversal or deletion. |

**Read-only:** `.gitignore` (already ignores `.temp/`), Product, Architecture, ROADMAP, NEXT, OUTLOOK, `.github/**`, LICENSE, application/test/tooling files, all other documentation and every parent/sibling repository. No branch, worktree, clone, shadow tree, additional remote, new report/archive, dependency installation, code, PR/merge, release, tag, deployment, provider, secret-access action, destructive Git, history rewrite, force-push or broad deletion is authorized.

**Required beginning `origin/main` HEAD:** `e7323f23e7aa221a5e0d14394d2e1fcff5a51559` (R07 report). Verify current GitHub API, local HEAD and `origin/main` before edits. Required previous baseline blobs:

| File | Expected Git blob SHA |
| --- | --- |
| `.documents/SOP/sop-architect.md` | `668895d8ce2e550964074d6adac334ffe5b2f54a` |
| `.documents/SOP/sop-implementation.md` | `7e7fff7174246f329c88f4d824545fd0b064560a` |
| `AGENTS.md` | `fdfa677567b41a5b3d8dc1d5e207064b1cf6a241` |
| `README.md` | `c2ba275bc961f1d7b7123c1cbb542565e4f5ac97` |
| `.gitignore` | `f51cc5306cdc8e4532422809eede68b93c4a7fd7` |
| `.documents/WIP/workorder-active.md` (R07) | `aa09d4802ef93ea3d20b87d8f4dc37b126a5ba5e` |
| `.documents/WIP/plan-roadmap.md` | `84f428b8d409c1665fc49d03a0e0e310c818b328` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` (R07) | `4e3e06019d285828402d2a7669f68dfeadee1084` |

## 5. Mandatory preflight, safe cleanup and STOP

1. Read `README.md`, `AGENTS.md`, Product, Architecture, both SOPs, all four WIP files and the **current GitHub-published report**. Require the complete exact installed `SOLO-WO-007 R08`, `OWNER-ISSUED`, and a **separate, directly delivered matching R08 handoff**. If missing or mismatched, STOP; the R07 handoff is not a substitute.
2. Verify the physical Solo root, branch `main`, exactly **one** worktree and one correct `origin`; local `HEAD` = `origin/main` = GitHub API `main` = the required SHA. The only initial dirty path may be the Owner's **unstaged installation** of R08 ACTIVE; nothing else staged, modified or unexpectedly untracked. Confirm four WIP files, one report, parent ignore guard read-only, and all §4 baseline blobs. If a remote advance or unexpected condition is found, STOP rather than silently rebase or delete it.
3. Verify `.gitignore` excludes `.temp/`, and no files beneath it are tracked or staged. Identify the Cline-provided task temporary location from the actual execution environment; do **not** assume permission to delete unrelated files anywhere in `%TEMP%`. An existing `.temp/` directory and a Cline directory are not inherently STOP conditions or prohibited workspaces.
4. At start, inventory accessible task-owned files in **both** temporary locations. Treat old identifiable unused task output as **cleanup candidates**, not a reason to block the run. Remove safely only what is positively agent/run-owned and no longer useful; preserve unfamiliar/shared/Owner-owned material. During work, maintain ownership awareness, use either allowed temporary area freely, and delete disposable intermediates promptly when finished.
5. Before end-of-run success, perform **cleanup in both** areas of task-owned files/folders no longer needed, including safely accessible Cline capture logs. If a log is only created after a shell command returns, perform a last safe cleanup pass when available. Do not run unsafe broad deletes or delete unknown files. Document counts/categories of created, removed, still-needed and inaccessible/unremovable items without public private paths or secrets. An identified obsolete task-owned artifact left behind is a cleanup failure; do not turn it into `PASS` by saying Cline created it.
6. STOP for wrong authority/HEAD, unbounded/unrelated permanent writes, a second worktree/checkout, unsafe deletion risk, a true product/authority conflict, or inability to report truthfully. **Do not STOP solely because Cline uses an authorized temp area**. If safe cleanup cannot be achieved, continue only as far as necessary to preserve work/evidence and publish a factual `FAILED` or `STOPPED` report when safe; no self-acceptance. Never discard unknown Owner data to obtain a clean state.

## 6. Execution and two-phase publication

**Phase A — exact governance correction.** Apply §§2–3 literally to only the four allowed permanent content files; leave `.gitignore` and every other authority unchanged. Check the before/after diff has only the two specified SOP sections/paragraphs, three AGENTS paragraphs and one README paragraph; no duplicate Git procedure, no weakened single-checkout rule. Verify relevant Markdown links/headings, all four WIP files, ignored `.temp/`, absence of tracked/staged scratch and all unmodified baseline blobs. Stage **only** `.documents/SOP/sop-architect.md`, `.documents/SOP/sop-implementation.md`, `AGENTS.md`, `README.md`, and the **unchanged Owner-installed R08 ACTIVE file**. Make **one ordinary content commit**, fast-forward push to the correct Solo `origin/main` and verify remote commit, file list and blobs.

**Phase B — single truthful evidence report.** Overwrite only `.documents/Reports/execution-report.md` with R08 facts: issued order, handoff, start/content revisions, precise path/diff inventory, exact section/paragraph checks, independent GitHub readback versus locally observed facts, preflight/one-worktree/root/ignore checks, **both temporary workspaces' use and cleanup evidence**, previous R07 policy mismatch and historical R03/R05 `STOP` still preserved, any relevant old garbage safely removed, genuinely necessary retention and precise cleanup trigger, and **any inaccessible or unremoved obsolete task-owned debris as FAIL**. Mark unavailable software, browser, CI, release and deployment tests `NOT RUN`, general inapplicable code standards `NOT APPLICABLE` with reason. Do not invent tests or claim software features. Stage **only the report**; one ordinary separate report commit, fast-forward push, read back GitHub `main`/report and give the final report commit SHA **only in chat**.

No report archive, other WIP path changes or additional task file. The public report must not expose private local file paths, secrets or sensitive Cline logs. Leave ACTIVE issued, ROADMAP/NEXT/OUTLOOK byte-identical; do not self-accept or consume R00.A.

**Failure path:** On a genuine STOP condition preserve Owner material and the authorized changes/evidence; publish a factual report when safe under the standing reporting procedure. No history rewrite, force-push, destructive `git clean`, sweeping temp-root deletion or misleading `COMPLETE` status. A disclosed failure remains a failure until resolved, but working normally with Cline's temporary space is expressly allowed.

## 7. Independent Chief Architect acceptance gates

For every applicable criterion give file/section/commit evidence and `PASS / FAIL / UNVERIFIED`. `NOT APPLICABLE` with reason only for genuinely out-of-scope general standards; never turn a cleanup failure into a pass.

1. **Authority and baseline:** exact Owner-installed `SOLO-WO-007 R08` plus matching direct handoff, required `e7323f23…` starting SHA, verified single Solo checkout, correct branch/remote, expected initial dirty state and ten baseline blobs.
2. **Full temporary-workspace permission:** Architect §6A and Implementation §5A **explicitly permit agent use and cleanup of both Solo `.temp/` and Cline's task-provided temp**, including outside-root Cline files/logs, without per-file permission, while forbidding arbitrary unrelated directories and permanent unauthorized edits.
3. **Mandatory garbage removal:** task-created temporary files/folders in **both** places are removed once obsolete, normally at run end; identifiable leftover historical garbage is safely addressed; justified retention has purpose/cleanup event; **known inaccessible obsolete garbage is honestly a failure**, not a false PASS; unknown/Owner material is protected.
4. **No worktrees and Git exclusion:** exactly one Solo working checkout; no linked/additional worktrees, clones, nested repositories or shadow source tree anywhere; `.temp/` ignored and no contents tracked/staged/committed; `.gitignore` unchanged.
5. **Cross-document consistency:** exactly §6A, §7A clean paragraph and §9 confinement in Architect SOP; exactly §5A, §5 confinement, §5C first sentence and final checklist line in Implementation SOP; exactly three AGENTS paragraphs and one README paragraph. No leftover Solo-only-temp contradiction; Architect §6B and Implementation §5B/§5C reporting semantics intact.
6. **No unapproved product/stage effects:** Product, Architecture, ROADMAP/NEXT/OUTLOOK and all other read-only blobs unchanged; one Entity/Hexagonal/localization/visibility/contact/legal contracts intact; no implementation choice, new task, release or deployment.
7. **Git, WIP and publication hygiene:** one permitted Phase A five-file content commit and one Phase B report-only commit, linear fast-forward, independently verified GitHub readback, four WIP files/one report, frozen published R08 ACTIVE and clean Git working tree; owned temp garbage separately addressed, not inferred from `git status`.
8. **Report honesty and authority:** actual temp creation/cleanup/retention evidence from both locations, all warnings and unresolved failures reported, R03/R05 historical incidents not rewritten, R07 policy mismatch acknowledged, software gates not falsely passed, no self-acceptance or early WIP closure.

**Completion boundary:** An independent `ACCEPT` is possible only with all applicable gates satisfied. It may substantiate the completed **R00.A documentation/governance work**; only a later, separately Owner-authorized closure may remove **R00.A** from ROADMAP, clear ACTIVE and select NEXT. Stage 00 still has pending `R00.B–R00.G` and must not be closed.
