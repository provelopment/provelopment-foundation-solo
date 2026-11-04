# Foundation Solo — workorder-active (Current Work Order)

**Purpose:** The only live executable task slot and evidence/disposition surface.  
**Work Order ID:** `SOLO-WO-001`  
**Status:** **OWNER-ISSUED ONLY UPON OWNER INSTALLATION AND MATCHING HANDOFF.** This file is an issuance-ready scope, not a self-authorizing Architect instruction. It becomes effective **only when the Owner deliberately approves and installs these exact bytes** at the specified Solo path **and** sends an explicit matching handoff naming `SOLO-WO-001`. Without both acts the implementation agent must STOP.  
**Drafted:** 2026-11-04  
**Procedure:** [`sop-architect.md`](../SOP/sop-architect.md) · [`sop-implementation.md`](../SOP/sop-implementation.md)  
**WIP invariant:** Exactly `workorder-active.md`, `plan-next.md`, `plan-outlook.md` in this directory. Do not create an additional work-order or report file in Solo.

## Current work

**Candidate objective:** Establish the existing `00.foundation-solo` folder as **one independent local Git repository** on branch `main` with **exactly one initial commit** containing the Owner-installed approved documentation/hygiene files. No application source, build system, remote, GitHub mutation, push, release or deployment. The Entity architecture is not being implemented under this order.

## Approved Scope — immutable upon Owner issuance

| Field | Frozen scope |
| --- | --- |
| Work Order ID | `SOLO-WO-001` |
| Approval state | **`OWNER-ISSUED`**, conditional on the Owner's deliberate installation of this exact file and matching `SOLO-WO-001` handoff; an Architect's download/attachment is not approval |
| Owner approval/handoff | An explicit Owner message naming `SOLO-WO-001` and this exact installed work order; no earlier planning message is sufficient |
| Target working directory | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — agent must independently verify actual path and filesystem boundary |
| Target repository | **New, standalone, local `00.foundation-solo/.git/` only**; not the enclosing `provelopment-root` |
| Branch | **`main`** — created by the authorized `git init -b main` |
| Starting HEAD | **None**; the agent's read-only onboarding found no Solo `.git`; this exact work order expressly permits the no-HEAD bootstrap exception |
| Permanent writes permitted | **Only** Git's internal `.git/**` created within the verified Solo root, including its index, object, ref and commit data; an initial commit may read/stage the 17 already Owner-installed files but must not modify their contents |
| Source-file modifications permitted | **None** by the agent. The Owner installs/approves any replacement architecture/SOP/WIP files before execution. |
| Commit authority | **Exactly one** local initial commit to `main`; proposed message: `chore: establish Foundation Solo documentation baseline` |
| Remote / push / PR / merge | **Not authorized**. Do not add `origin`, create a remote repository, authenticate to GitHub, fetch, push, open PRs or merge. |
| Dependencies / frameworks / build / deploy | **Not authorized**. No package manager, code generation, application source, test framework, provider account, release or deployment. |
| Additional working trees / temporary files | **Forbidden**: no `git worktree add`, linked worktree, clone, shadow checkout, alternate Git directory, nested repo, `.temp/` creation, external temp/cache writes, or generated artifact. |
| Other repositories | **Read-only** as needed for evidence. No write/commit/reset/staging anywhere in `provelopment-root` or sibling repositories. |
| Execution evidence | **Report in the agent's chat/handoff only.** Do not edit `workorder-active.md`, `plan-next.md`, `plan-outlook.md` or other tracked files after the initial commit; preserve a clean one-commit baseline. The Owner/Architect will reconcile disposition/WIP separately after review. |

### Mandatory preflight gates — fail means STOP before `git init`

1. Confirm the Owner-issued `workorder-active.md` contains exactly this scope and `OWNER-ISSUED`; confirm the Owner's matching `SOLO-WO-001` handoff. Confirm exactly three WIP files.
2. Confirm the current working directory physically resolves to the designated Solo root; inspect for junctions/symlinks/redirects that might make a Git write escape Solo. Confirm `00.foundation-solo/.git` **does not exist** and Git's current enclosing top level is the **parent** `C:/Users/Sean/Documents/Provelopment`, not a pre-existing nested Solo repository.
3. From the parent root, verify `git check-ignore -v -- 00.foundation-solo/README.md` names the anchored `/00.foundation-solo/` rule. **Also verify `provelopment-root/.gitignore` is committed and clean** (the previous read-only report found that rule effective but uncommitted). No Solo agent is allowed to commit or edit that parent file; if still dirty or absent, STOP and report to Owner.
4. From the parent root verify `git ls-files -- 00.foundation-solo` is empty. The parent may have unrelated untracked files; preserve them without staging, deleting, cleaning or altering them. Record the parent HEAD and parent `.gitignore` status before execution for after comparison.
5. Verify the Solo folder contains **exactly the 17 expected approved files** in the roster below, and no other existing files, code, nested `.git`, `.temp` or unknown artifacts. This is a names-only expectation based on the updated documentation package; the actual installed bytes are the Owner-authorized baseline. Do not replace files with an older ZIP or re-run documentation generation.
6. Verify Git is available, can initialize locally, and a usable commit author identity is already configured without modifying user/global/system configuration. Check for configured hooks, signing or helpers that could cause writes outside Solo; ensure none will run. If this cannot be ensured without an unapproved tool or identity change, STOP and report. No prompts for credentials or network actions.

### Approved baseline — exact 17 pre-existing file paths

Only the following existing paths may be explicitly staged, after re-enumerating and verifying them:

```text
.editorconfig
.gitattributes
.gitignore
.github/CODE_OF_CONDUCT.md
.github/CONTRIBUTING.md
.github/PULL_REQUEST_TEMPLATE.md
.github/SECURITY.md
AGENTS.md
LICENSE
README.md
.documents/About/FOUNDATION-SOLO.md
.documents/Architecture/ARCHITECTURE.md
.documents/SOP/sop-architect.md
.documents/SOP/sop-implementation.md
.documents/WIP/plan-next.md
.documents/WIP/plan-outlook.md
.documents/WIP/workorder-active.md
```

**Crucial:** Stage explicitly by this exact path list from **inside Solo**. Do not use `git add -A`, `git add .`, a wildcard, or an ambiguous parent directory; never stage anything through the parent repository. If any file is missing, additional files appear, or the Owner-installed work order differs from the handoff, STOP rather than silently substituting contents.

### Authorized execution — only after all gates pass

1. From the verified Solo root, run `git init -b main`. Verify `git rev-parse --show-toplevel` resolves **inside Solo**, not to the enclosing root.
2. Inspect local Git state and effective hook/signing configuration; do not modify user/global Git config or invoke external helpers. Disable any hooks/signing for this single documentation-only commit through invocation-local `git -c` options if needed, provided those options do not write outside Solo. If a safe local-only commit cannot be made, STOP.
3. Stage only the 17 listed files by exact path; verify the staged file list and their count (17); do **not** stage ignored/unknown material. The exact content must be the Owner-installed approved files, including the issued work order and the `plan-next.md` issuance state.
4. Create **one** local root commit on `main`, without hooks/network activity and without a remote. Do not amend, rebase, branch, tag, push, change source documents or produce a second commit.
5. Run read-only after-checks. Return evidence in chat to Owner; the implementation agent must **not** write back to WIP or create a report file.

### Acceptance evidence — provide commands and actual outputs

- Solo root and independent repository: `Get-Location`; `git rev-parse --show-toplevel`; `Test-Path .git`; `git branch --show-current`.
- Initial history: `git rev-list --count HEAD` returns `1`; `git log -1 --format='%H%n%s%n%an <%ae>'` records exact commit, message and author; `git status --porcelain=v1` is empty.
- Tracked inventory: `git ls-files` matches the roster above **exactly** (17 paths); report paths and count.
- Isolation: `git remote -v` empty; `git worktree list --porcelain` reports **only the main Solo worktree**; no nested clones, linked worktrees or shadow directories.
- Safety: parent `git check-ignore -v` still returns `/00.foundation-solo/`; parent HEAD and `.gitignore` status are unchanged **during Solo execution**; no parent/sibling writes.
- Hygiene: no `.temp/`, `.cache/`, external temporary files, tool-generated files or new build directories; no source files, dependencies or unapproved manifests.
- WIP: the same three WIP filenames; no extra task artifacts.
- Proportional verification: **there is no application or test suite to run**. This task proves repository isolation and clean baseline only; do not claim application tests passed.

### Stop conditions

Stop **without broad cleanup, reset, reflog rewriting or alternate checkout** on any of these: approval/handoff mismatch; parent guard uncommitted/ineffective; existing Solo `.git`; unexpected Solo files; parent-tracked Solo content; uncertain physical root or symlink/junction; user/global Git writes required; enabled external hooks/signing/network; commit author identity absent; conflict or collision; any path outside Solo; extra linked worktree; inability to produce one clean commit. If an operation has already partially created Solo `.git`, preserve it, report the exact state and seek specific Owner direction—do **not** auto-delete/reinitialize it.

## Execution Evidence — to be reported in chat, not edited by agent under this work order

- **Agent:** Awaiting execution after Owner installation and handoff.
- **Actual Solo/parent preflight and parent guard state:** Pending.
- **Initial HEAD:** None, if independently confirmed.
- **Final HEAD / one-commit evidence:** Pending.
- **17-file inventory and clean-status evidence:** Pending.
- **No remote, no secondary worktree, no temporary artifacts:** Pending.
- **Deviations/stops:** Pending.

## Architect Disposition — independent review required

- **Disposition:** `NOT REVIEWED`.
- **Exact revision and evidence:** Awaiting the agent's first-hand report and, if needed, Owner-supplied local verification. A later GitHub push is outside this scope.
- **Owner closure / WIP reset:** Not authorized by this bootstrap order. The Owner reviews and separately directs WIP reconciliation after the Architect's disposition. No task-history files may be added.
