# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-007`  
**Edition:** `R07`  
**Title:** Owner-approved Solo `.temp/` use and bounded execution-harness logging policy  
**Approval marker:** `OWNER-ISSUED` **only upon Owner installation of this exact file and direct matching R07 handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting GitHub `origin/main` HEAD:** `b5fba11293931bc65de2ad0a7d94b5543d50f718`  
**Matching handoff:** `SOLO-WO-007 R07 — Implementation Agent Handoff`  
**Work classification:** Bounded governance correction of the existing R00.A work-order sequence; not a new ROADMAP package, not product implementation, not R00.A closure.  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP`; R00.A and Stage 00 closure remain separately Owner-authorized.

> **Owner issue boundary.** The R06 order was installed locally but the agent correctly stopped before command execution; the published GitHub `main` remained at the R05 report commit named above. This **one** complete R07 order supersedes that local R06 order **only** when the Owner replaces `.documents/WIP/workorder-active.md` with this exact R07 file and directly delivers the matching R07 handoff. R05/R06 prompts are not authorization. The Agent may not self-issue, edit the approved order or silently change its starting SHA.

**Frozen Approved Scope:** §§1–7. This order makes the explicit Owner policy choices in §2, permits only the exact repository edits in §3 and requires the checks in §7. The Agent may stage and publish the Owner-installed ACTIVE file but must not rewrite it or self-accept.

## 1. Owner decision and bounded objective

The Owner has created the local working directory `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.temp` and directs that the Implementation Agent be free to use it for disposable work, **without adding it to Git** and **cleaning its contents when no longer needed**. In portable project policy this directory is `<VerifiedSoloRoot>/.temp/`; do not hardcode the Windows path as a requirement on another host.

The existing `.gitignore` already has `.temp/` and is **read-only** under this order. Existing Architect SOP §6A and Implementation SOP §5A already refer to `.temp/`, but retain unnecessary per-order grant and task-closure-empty restrictions, and do not clearly distinguish agent-controlled scratch from unavoidable platform-managed execution logs. Make those rules clear, proportional and consistent by replacing **exactly** the two designated SOP sections with §2's approved text; no third competing SOP or altered source document.

**Explicitly presented Owner policy change:** For this order, unavoidable *host-managed* logs created by the IDE/Cline command-execution infrastructure outside the agent's control are separately classified as platform artifacts, **not an agent-selected scratch location and not by themselves an agent write-boundary failure** when transparently disclosed. This is a narrow prospective governance clarification, **not** authorization for an agent, its commands, or redirectable subprocess output to write outside Solo. It does not erase the historical R03/R05 `STOP` findings or authorize reading, editing or deleting host-managed logs. If the Owner does not approve this distinction, do not issue R07; the runner requires Owner-level reconfiguration first. Creating `.temp/` alone does **not** redirect a host's capture logs there.

No application code, technology choice, new tests, source schema, provider operation, WIP closure or new ROADMAP package is authorized. The accepted R00.A identity/publication contract and C1–C6 documentary decisions remain unchanged.

## 2. Exact SOP replacement text — no improvisation

### A. Architect SOP §6A — replace this entire section only

In `.documents/SOP/sop-architect.md`, replace everything beginning `## 6A. Bounded repository and temporary-workspace governance` and ending immediately before `## 6B. Version Control Procedure — Git and GitHub` with **exactly**:

```markdown
## 6A. Bounded repository and temporary-workspace governance

**One editable root.** The Implementation Agent's permanent project writes are confined to the verified Solo root and to paths authorized by the current Owner-issued order (or the standing report-only exception). Extra Git worktrees, linked or cloned checkouts, nested repositories and shadow source trees are forbidden, including under `.temp/`. The parent's repository and sibling repositories remain read-only.

**Owner-approved disposable workspace.** The existing `<SoloRoot>/.temp/` is the standing, freely usable scratch location for an Implementation Agent carrying out an otherwise authorized task. It may hold agent-created interim files, downloads, logs, exports, test fixtures, caches and tool output as needed. No separate per-file or per-order permission is needed to use `.temp/` for *temporary* task material; this permission does not authorize permanent deliverables, implementation changes, additional checkouts, a new task, or writes outside Solo. The path is resolved from the verified repository root, not a hardcoded user's directory.

**Never include scratch in Git.** `.temp/` is ignored by root `.gitignore` and must remain excluded from all commits. Never deliberately stage or force-add its contents; verify that no files below `.temp/` are tracked or staged. Ignoring a path is not permission to store secrets in it or to bypass a work-order path ledger.

**Keep it useful, not accumulated.** Clean agent-owned temporary contents as soon as their task purpose ends. A needed intermediary may remain across phases or runs only while its purpose is real; identify retained items, why they are still needed and their next cleanup checkpoint in the one canonical execution report. No automatic requirement to empty `.temp/` while justified materials are still needed. Do not delete unfamiliar, pre-existing or Owner-owned material, and never use a broad destructive cleanup to manufacture a clean state. Report unexplained residue rather than silently removing it.

**Control what the agent can control.** Direct agent-created output and redirectable tool, cache or subprocess writes into Solo, ordinarily under `.temp/`; never elect to create outside-root scratch. Host-managed diagnostic or command-capture logs written automatically by the IDE/agent execution infrastructure, where the agent cannot redirect or control their location, are platform artifacts, **not an approved working directory**. Their mere generation is not by itself an Implementation Agent scope breach under this adopted distinction; disclose known instances and limits candidly. Do not deliberately use, read, change or delete such host logs as task artifacts, and do not send secrets or confidential data into command output. If the agent can redirect a log, it must be directed inside Solo. If external writes are agent-controlled, sensitive material may have been exposed, or the platform behavior cannot be distinguished safely, STOP and escalate.

**Work-order and review obligation.** The Architect specifies authorized permanent output paths and meaningful verification/cleanup gates, but must not require an additional temporary-file permission for ordinary use of `.temp/`. At review, check no unauthorized project paths changed, no extra checkout exists, no `.temp/` content is tracked, no obsolete agent-owned scratch has accumulated, and any genuinely retained or platform-managed artifacts are disclosed without a false `PASS`. Architect SOP §6B remains the sole authority for version-control mechanics.

**Parent guard.** Solo is an independent Git repository on `main`; the parent repository's committed `/00.foundation-solo/` ignore rule and absence of parent-tracked Solo files are verified. Parent `.gitignore` edits remain outside Solo agent authority. `origin` is the verified Solo GitHub repository; only ordinary, expressly authorized Git publication is permitted under §6B. Do not re-baseline published history.
```

### B. Implementation SOP §5A — replace this entire section only

In `.documents/SOP/sop-implementation.md`, replace everything beginning `## 5A. Mandatory one-root workspace and temporary-file discipline` and ending immediately before `## 5B. Version Control Procedure — Git and GitHub` with **exactly**:

```markdown
## 5A. Mandatory one-root workspace and temporary-file discipline

**One checkout, no shadow copies.** Work only in the verified Solo repository root. Do not create or use additional linked Git worktrees, cloned checkouts, nested repositories, branch-specific working directories or shadow source trees, including inside `.temp/`. An Owner-issued work order is required for permanent implementation changes; read-only onboarding and the single report-only permission retain their existing authorization limits.

**Standing permission to use `.temp/`.** The Owner has provided `<VerifiedSoloRoot>/.temp/` as the Implementation Agent's discretionary disposable workspace when carrying out an otherwise authorized run. Use it as needed for your own temporary files, downloads, tools' redirectable caches, test output, logs and intermediate work without seeking approval for each scratch operation or naming each file in the work order. This permission is for temporary work **only**: it authorizes no unlisted permanent edit, other repository, external destination, second checkout, dependency install or new task. Confirm the physical location stays inside the verified Solo root; do not escape through a symlink or junction.

**Do not put `.temp/` in Git.** The root `.gitignore` contains `.temp/`. Confirm it is ignored and that no `.temp/` contents are already tracked or staged; never force-add them. Commit only the current order's permitted permanent paths. If unexpected tracked scratch or an ignore failure is found, STOP instead of silently fixing unrelated repository state.

**Clean up when no longer needed.** Keep `.temp/` useful; promptly delete only the temporary contents you created once they have no remaining purpose. A still-needed file may survive an intermediate phase or the end of a run, but the execution report must name the retained material, its purpose, and when it should next be reviewed or removed. Do not accumulate abandoned outputs, caches or logs. Never delete unknown, pre-existing or Owner-owned contents, use `git clean`, or broadly clear the directory to make a check pass; explain a blocker when safe cleanup is impossible. An empty `.temp/` at the end is desirable, but a justified and disclosed retention is not automatically a violation.

**Direct tool outputs into Solo.** Where a command, helper or subprocess can choose the location for scratch, output, temporary files or caches, keep those writes inside Solo and preferentially in `.temp/`. Do not use OS `%TEMP%`, the home directory or parent/sibling repositories for agent-selected scratch. Do not change host or IDE settings outside Solo without separate Owner authority.

**Host-managed capture is distinct.** IDE/Cline execution infrastructure may itself create diagnostic or command-capture logs outside Solo that the Implementation Agent cannot redirect or control. Under Architect SOP §6A, such automatic platform artifacts are not an approved workspace and do not, by their mere existence, fail the agent's one-root project-write gate. Disclose known occurrences, contents sensitivity and uncertainty in the canonical report; do not intentionally direct task files there, inspect/delete the platform logs or misrepresent their existence. Any redirectable output must use Solo; any agent-controlled outside-root write or suspected confidential exposure is a STOP. The historical R03/R05 dispositions remain unchanged.

**Evidence.** Record the initial and final `.temp/` inventory (or why a reliable inventory was unavailable), tracked/ignored status, agent-created and safely cleaned items, purposeful retained items and the next cleanup checkpoint, known platform-managed logs and any unresolved risk. Distinguish reported local observations from independently readable GitHub evidence. A claim of a clean Git tree does not prove `.temp/` has no garbage, and an ignored directory is not an additional source-control or WIP authority.
```

## 3. Exact path-and-operation ledger

| Path | Allowed operation |
| --- | --- |
| `.documents/SOP/sop-architect.md` | **Phase A:** replace only §6A with the exact §2.A text; §6B and all other sections unchanged. |
| `.documents/SOP/sop-implementation.md` | **Phase A:** replace only §5A with the exact §2.B text; §5B, §5C and all other sections unchanged. |
| `.documents/WIP/workorder-active.md` | The **Owner** installs this complete R07 file; agent may stage/publish its exact bytes unchanged in Phase A; leave it issued. |
| `.documents/Reports/execution-report.md` | **Phase B only:** overwrite the existing canonical report with truthful R07 execution and incident-context evidence; no second report. |
| `<VerifiedSoloRoot>/.temp/` | Standing temporary scratch permission per the approved SOP text; **ignored and never staged or committed**; agent-owned obsolete content cleaned, needed residuals justified in report. |
| `.git/**` | Minimal ordinary Git metadata for the two approved commits and fast-forward publication, governed exclusively by Architect SOP §6B. |

**Read-only:** `.gitignore` (already ignores `.temp/`), `AGENTS.md` (already identifies `.temp/`), README, Product, Architecture, ROADMAP, NEXT, OUTLOOK, `.github/**`, LICENSE, all source/test/tool files and all parent/sibling repositories. No new source file, WIP file, report, document authority or archive. No fresh branch, worktree, clone or remote; no unrelated install, provider write, merge, release, tag or deploy; no destructive reset/clean/checkout, history rewrite, amend or force-push. Never modify or purge host-managed OS logs. The only network operation authorized is normal Solo GitHub publication/readback.

## 4. Preflight, historical status and STOP conditions

1. Read root README/AGENTS, Product, Architecture, both SOPs, all four WIP files and the **GitHub-published** current report. Confirm the exact Owner-installed R07 ACTIVE file and direct matching `SOLO-WO-007 R07` handoff. Prior R06 was **local STOP with no commits or report**; GitHub still shows issued R05 and the R05 canonical report. The Owner's installation of R07 is the sole initial authorized working-tree change.
2. Verify actual root and `git rev-parse --show-toplevel`, branch `main`, exactly one worktree and one remote `origin` for `provelopment/provelopment-foundation-solo`; local HEAD = `origin/main` = GitHub API `main` HEAD = **`b5fba11293931bc65de2ad0a7d94b5543d50f718`**. If mismatched, STOP; do not silently rebase or infer approval.
3. Confirm the only initial dirty path is Owner's unstaged replacement of ACTIVE with exact R07; no unexpected staged/modified/untracked changes. Check exactly four WIP paths and one report, `.gitignore` has `.temp/`, `git check-ignore` identifies `.temp/` as ignored, and `git ls-files` does not show tracked/staged `.temp/` content. The Owner-provided folder may already contain files; **inventory and preserve unfamiliar items**, do not assume empty or delete them. If ignored/tracked state or ownership is uncertain, STOP and report.
4. Capture read-only baseline blobs from published R05: Architect SOP `5f8aa8e78c75bbea73bac2a88753e1679aa930a6`; Implementation SOP `39cd894c95ff67ac858fb89b559e435a3a9d7ad9`; ROADMAP `84f428b8d409c1665fc49d03a0e0e310c818b328`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; published R05 ACTIVE `6c559ca971360f6efd9a348dcd996d07af56e813`. Preserve all other published binding document blobs.
5. Distinguish agent-controlled temp/scratch from automatic platform-managed command-capture logs *before* using tools. R07 expressly adopts the §2 boundary prospectively if installed by the Owner; no need to claim that creating `.temp/` redirects the host logs. If an agent-selected or redirectable output writes outside Solo, or there is a credible leak of confidential data, **STOP** immediately, even if later removed. Neither the historical R03 nor R05 `STOP` is reversed; R06 STOP remains a truthful chat-only attempt.
6. If any authorization, HEAD, repository identity, exact replacement, unexpected dirty state, material conflict, unapproved file write, confidentiality or safety gate fails, STOP and preserve Owner data. Write/publish a `STOPPED` report only when safe and within the standing report-only permission; never conceal a failure or manufacture a nominal successful two-commit run.

## 5. Ordered execution

**Phase A — exact SOP corrections.** Replace only the two designated sections with the literal §2 texts. Independently compare both full blocks with the order, including heading, paragraph sequence and Markdown references; ensure Architect SOP §6B and Implementation SOP §5B are still intact, and the exception for platform-managed logs is narrow and explicit. Verify `.gitignore` unchanged, `.temp/` ignored/not tracked, relevant Markdown links and headings, and all read-only baseline blobs. Stage **only** the two SOPs and Owner-installed ACTIVE. Make one ordinary content commit, fast-forward push to verified `origin/main`, and read back the exact GitHub diff and blobs.

**Phase B — one canonical report.** Overwrite `.documents/Reports/execution-report.md` with exact R07 ID/edition/handoff, real date, starting/content revisions, exact changed paths, evidence of §2 literal implementation, `.temp/` ignore and retention/cleanup state, host-platform logging observed and limitations, remote checks, test gates actually run versus `NOT RUN`, and the preserved R03/R05 `STOP` and R06 unexecuted status. Distinguish agent-local facts from remote-verifiable evidence and never self-accept or clear ACTIVE. Stage **only** that report; make one separate ordinary report commit, fast-forward push and read back final GitHub HEAD/report. Provide report SHA only in chat, never in its own content. STOP after handback for independent review.

**Failure path:** Do not run unrelated tools, create code or use an unapproved location as scratch to bypass a blocker. Do not delete unknown local files or historical host logs. A tool-generated external platform log is disclosed under the newly accepted classification, not suppressed; an agent-controlled out-of-root write still triggers STOP and a truthful report if safely possible.

## 6. R00.A architectural and closure boundary

The accepted R00.A documentation, original R02 §2 13-item index, R03 single Git SOP correction and corrected C1–C6 definitions in the published R05 report remain canonical evidence and must not be rewritten. No product, Architecture, AGENTS, README or ROADMAP edits are permitted. This governance correction is **not** a new product package, a backdated acceptance of R03/R05, executable test pass, R00.A closure, R00.B execution or Stage 00 completion. After independently accepting R07, the Chief Architect may separately consider R00.A acceptance on the accumulated verified documentary evidence; any actual ROADMAP/NEXT/ACTIVE closure still requires a distinct Owner-authorized transition. R00.B–R00.G remain pending.

## 7. Independent acceptance gates

For every applicable condition report `PASS`, `FAIL` or `UNVERIFIED` with exact file, commit and meaningful local evidence. General software checks outside this documentation-only scope are `NOT APPLICABLE` with reasons; unperformed tests are `NOT RUN`, never passed.

1. **Authorization/baseline:** exact installed R07 and directly matching handoff, correct `b5fba112…` HEAD, Solo root/main/one worktree/one origin, expected initial ACTIVE-only modification, and honest report provenance.
2. **Architect SOP §6A:** exact §2.A text only, clear standing `.temp` permission, no per-file grant, no Git tracking, cleanup by continuing need, host-managed logs distinguished, one-root and single Git authority preserved.
3. **Implementation SOP §5A:** exact §2.B text only, no extra checkout, discretionary `.temp` usage, correct ignore/track rules, no garbage retention or unsafe deletion, clear local-versus-platform effects and STOP boundaries.
4. **Git exclusion and temp hygiene:** `.gitignore` already ignores `.temp/`; no files underneath are tracked/staged; inventory of any pre-existing and agent-created temp files is accurate; obsolete agent-owned material is cleaned; justified retained material has a purpose and next checkpoint.
5. **No unauthorized policy drift:** Architect SOP §6B and Implementation SOP §5B/§5C unchanged; README/AGENTS, Product/Architecture, ROADMAP/NEXT/OUTLOOK and all other read-only documents remain byte-identical; prior R03/R05 incident findings not erased.
6. **Publication integrity:** exactly the two SOPs and installed ACTIVE in Phase A, only current report in Phase B, ordinary linear fast-forward pushes, correct GitHub readback, four WIP files, one report, clean tree excluding deliberately ignored scratch, no extra checkout or unauthorized permanent file.
7. **Accurate evidence and execution boundary:** no agent-selected outside-root scratch/tool output; automatic platform logs disclosed without pretending to be `.temp` content; confidential-output risk considered, no false `PASS`, historic R06 inability reported correctly, local-vs-GitHub provenance distinguished, non-applicable tests not claimed passed.
8. **Disposition separation:** implementation agent stops after reporting; independent Chief Architect assigns `ACCEPT / REVISE / STOP`; no self-closure, roadmap pruning, NEXT selection, new package, release or deployment.

**Completion boundary:** No one may treat a successful R07 SOP maintenance run as R00.A closure or Stage 00 acceptance. Subsequent closure of *only* R00.A, if independently accepted, is separately Owner-authorized.
