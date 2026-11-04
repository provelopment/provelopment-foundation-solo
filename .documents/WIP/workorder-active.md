# Foundation Solo — Current Work Order

**Work Order ID:** `SOLO-WO-002`  
**Edition:** `R01` — correction of namespace ownership after the original Gate 5 stop; **same work order ID, not a new task**  
**Title:** Establish the public Solo GitHub repository and a self-sufficient Git workflow  
**Approval state:** `OWNER-ISSUED` **only when** the Owner has reviewed this exact R01 document, installed it as `.documents/WIP/workorder-active.md` in the Solo working root **replacing the stopped R00 edition of `SOLO-WO-002`**, and sent an explicit matching `SOLO-WO-002 R01` re-issuance handoff. The previous R00 handoff does **not** authorize execution under R01. This downloadable candidate, by itself, is not authorization.  
**Target repository:** Foundation Solo ONLY — `00.foundation-solo` at its independently verified local Git root  
**Target branch:** `main`  
**Expected starting HEAD:** `b924729c67a926ebd0f0dc19dcd4a56750a97bd5` (agent-reported `SOLO-WO-001` root commit; verify independently before changes)  
**Authorized new GitHub repository:** **`provelopment/provelopment-foundation-solo` — PUBLIC; owned by the existing personal GitHub account `provelopment` (`owner.type: User`)**  
**Status:** R01 correction candidate awaiting Owner review, replacement of the current WIP file, and explicit matching R01 handoff. Previous R00 execution stopped at preflight with no writes or GitHub repository creation; R01 does not expand the remaining task.

## Correction record (R00 → R01)

The original `SOLO-WO-002` stopped correctly at Gate 5: GitHub confirmed that `provelopment` is a **personal User account**, not an organization. The Owner's intended public repository name is unchanged. **R01 expressly authorizes this account to own the repository.** No account conversion, new organization, transfer, alternate account, or new repository name is authorized. All other boundaries, seven documentation-edit targets, the single-commit rule, and evidence/STOP requirements remain intact.

The agent reported zero writes under R00. On resumption, verify this state anew: the one expected Owner-installed WIP document modification, baseline HEAD `b924729…`, no Solo remote, and no target GitHub repository. A changed state is a STOP, not permission to overwrite.

## 1. Objective and authority

Turn the Owner-approved local 17-file, documentation-only Solo repository into **one public, independently managed GitHub repository** owned by the existing **`provelopment` personal GitHub account**. Preserve its original Git history, make `origin/main` track local `main`, configure a working private vulnerability-reporting channel, and make only the necessary **Solo-local documentation/SOP hygiene updates** so subsequent implementation orders can use Git and GitHub without asking the Owner to perform routine version-control commands.

The Owner expressly authorizes this **public GitHub repository creation and initial push**. This is **source-repository publication**, **not** an application release, a deployed website, a DNS change, or authorization to publish confidential material. It does not authorize product implementation or decisions about frameworks, EFS schemas, or application architecture.

**Role and authorization rule:** The Architect authors this order; the Owner reviews and installs it in the one WIP slot and issues the matching handoff; the Implementation Agent executes it autonomously within the frozen scope. The agent must not revise this Approved Scope. Agent reports do not constitute independent Architect acceptance.

## 2. Approved Scope — frozen upon Owner issuance

| Area | Authorized | Prohibited |
| --- | --- | --- |
| Local repository | Work inside the **existing** Solo Git repository on `main`; inspect and commit the bounded Solo documentation changes below | `git init` again; new clones; linked worktrees; nested repositories; alternate checkouts; operations in parent/sibling repositories |
| GitHub repository | **Create exactly one** empty **public**, **personal-account-owned** repository with exact full name `provelopment/provelopment-foundation-solo` (`owner.login: provelopment`, `owner.type: User`), via authenticated GitHub CLI or installed MCP, then configure it and push Solo | Different owner/name/visibility; organization creation or account conversion; template/fork/import; remote auto-generated README/license/gitignore; deletion/transfer of repo; creating another repository |
| Security setting | Enable **private vulnerability reporting** on the new repo and verify it is enabled **before pushing tracked content** | Inventing a confidential contact; publishing code before the private reporting channel is confirmed; broad security/configuration changes |
| Git remote | Add a single `origin` pointing to the exact repository; push `main` and set upstream; verify local and remote commit/tree match | Force-push; history rewrite; replace/reset existing history; unknown remote; unrelated remote operations |
| Permanent local files permitted | The Owner-installed `.documents/WIP/workorder-active.md`; bounded updates to `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md`, `.documents/SOP/sop-implementation.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, and `.github/SECURITY.md` **only as described below**; Git's own `.git/**` | Any other file modifications, new documentation directories, new WIP files, source/test/toolchain files, or license changes |
| Commits | **One** new ordinary local commit on `main`, containing the Owner-installed current order plus the authorized documentation/SOP changes, then push it with the existing root commit. If a GitHub command itself requires an additional safe metadata commit, STOP and explain; don't improvise history | Amend/rebase/squash, rewriting root commit, tags, extra branches, PR creation/merge, or second commit under this order |
| External resources | Existing GitHub CLI/MCP authentication; GitHub repo creation and the strictly specified repository settings | Provider deployment, DNS, billing, paid services, CI workflow creation, secrets rotation, account-wide settings, other repositories |
| Evidence | Terminal/chat completion report with exact commands/results. No new log/report file | Invented passing checks, copied secret tokens, or modifications to frozen Approved Scope |

**Absolute filesystem boundary:** The Implementation Agent may **read** the broader Provelopment workspace for evidence but may **write only inside the verified Solo directory**, and only to the authorized local paths. GitHub writes are limited to the new named GitHub repository and its settings. The agent must not write, commit, push, clean, reset or otherwise mutate `provelopment-root`, Foundation, Web 1, or other repositories.

## 3. Mandatory preflight — STOP on any failed gate

1. Read `README.md`, `AGENTS.md`, the two Solo SOPs, relevant architecture/product documents, and **all three** WIP files. Verify this **`SOLO-WO-002 R01`** order was installed by the Owner as the **only** `workorder-active.md`, the Owner's separate **matching R01 re-issuance** handoff has arrived, and WIP has exactly three filenames: `workorder-active.md`, `plan-next.md`, `plan-outlook.md`.
2. Physically verify Solo is the Git top-level directory, not a symlink/junction and not the enclosing `provelopment-root`; branch is `main`; `HEAD` equals the expected full `b924729c67a926ebd0f0dc19dcd4a56750a97bd5`; `git rev-list --count HEAD` is 1; its tracked root inventory has the previously verified **17 paths**. No extra worktrees, clones, tags or branches. **No remote** exists at preflight.
3. Because the Owner replaced `workorder-active.md`, the **only permitted worktree change** before agent edits is that **one tracked file**. If any other path is changed, staged, untracked (apart from an explicitly known, ignored Owner-controlled path) or unknown, STOP; never clean or overwrite it. Inspect the installed work order diff so unrelated edits cannot be slipped into the GitHub publication.
4. Verify that the parent repository's **committed** `.gitignore` includes `/00.foundation-solo/`, that `git check-ignore` confirms protection, and that the parent tracks no Solo files. Do not require a clean parent worktree beyond the scoped ignore file; unrelated Owner data is not the Solo agent's responsibility.
5. Verify GitHub authentication and permissions **without disclosing credentials**, and independently confirm that `provelopment/provelopment-foundation-solo` does **not** already exist. A 404 is not sufficient when authorization is uncertain; distinguish nonexistent from inaccessible. If the repo exists or the result is ambiguous, STOP before creation. Confirm the authenticated GitHub principal is **`provelopment`** and the namespace is the existing **personal account** (`type: User`); confirm permissions to create its **public** repository and administer that repository’s private vulnerability reporting. This personal namespace is **expressly permitted** by the Owner under R01, but no other namespace or account is permitted. Do not request conversion to an organization or prompt the Owner for ordinary shell commands.
6. **Public-history and hygiene review:** inspect the full tracked tree and existing root commit, including WIP, for credentials, private data, confidential internal material, unlicensed artwork/code, and unapproved third-party content. The first root commit will also become public; fixing only the working tree cannot remove sensitive data from that history. Verify Apache-2.0 `LICENSE` is complete and Owner-approved; determine whether any third-party `NOTICE` attribution is actually required (do not invent a notice). If an actual privacy/licensing issue is found, STOP before public creation and report exact concerns without revealing secrets. Local path examples and ordinary commit-author metadata are part of the public history: flag them in the report, but do not rewrite the approved baseline.
7. Confirm no uncontained tool side effects: no credential setup, global Git configuration writes, external scratch/cache creation, signing helper, installation, or hook can be required. Use existing authentication, GitHub CLI/MCP, and in-Solo `.temp/` for disposable scratch only if necessary. If a tool cannot comply with the Solo-only filesystem boundary, switch to an already-installed compatible tool or STOP. Never create another checkout/worktree.

## 4. Authorized execution — single coherent task

**Phase A — GitHub repository and security channel (authorized personal namespace)**

1. **Using the existing `provelopment` personal account (not an organization),** create **one empty public** GitHub repository at exactly `https://github.com/provelopment/provelopment-foundation-solo` using an authenticated, noninteractive operation such as `gh repo create provelopment/provelopment-foundation-solo --public --description "An open-source, single-Entity EFS website foundation"`. **Do not** use `--add-readme`, `--license`, `--gitignore`, `--template`, `--clone`, `--source` or `--push` at this step. An empty public repository is intentional so its security setting can be configured before its documentation/history is pushed. If creation partially succeeds, do not create it again or delete it; inspect/report the exact state.
2. **Immediately** enable GitHub private vulnerability reporting for this repository using the existing authenticated CLI or MCP. One documented CLI method is `gh api --method PUT repos/provelopment/provelopment-foundation-solo/private-vulnerability-reporting`. The endpoint is repository-scoped and works for a public repository owned by a personal account when the principal has admin rights; verify the actual result rather than assuming success. Independently query `gh api repos/provelopment/provelopment-foundation-solo/private-vulnerability-reporting` and require `enabled: true`. If unavailable, STOP **before any content push**; leave the remote as-is and report the precise blocker, without deleting it or claiming the security channel works. No public issue should be used for vulnerability details.

**Phase B — Minimal documentation and SOP reconciliation (Solo root only)**

Make limited updates **only** to the seven authorized tracked files below, preserving all product and architecture contracts:

- `README.md`: replace now-false statements that the repository or Git baseline is not initialized, that the parent ignore rule remains uncommitted, or that the project has no GitHub remote. Clearly distinguish **public source repository** from **no implemented application, release or deployment**. Adjust only the repository/publication-readiness wording that this order actually resolves; retain any genuinely outstanding copyright/third-party notice and support-policy requirements.
- `AGENTS.md`: update any stale repository status; name `origin` as the verified Solo public GitHub repository; clarify that the implementation agent performs authorized routine Git/GitHub commands itself, without asking the Owner to perform Git mechanics, while retaining the physical Solo-only write rule, no-worktrees rule and no unauthorized merge/deployment.
- `.documents/SOP/sop-architect.md` and `.documents/SOP/sop-implementation.md`: add or reconcile a **single consistent Version Control Procedure** covering these mandatory rules: (a) the Owner approves **what may change** by one complete exact work order, not individual Git commands; (b) when a work order expressly authorizes commits/pushes, the implementation agent performs staging, commit, upstream tracking, push, remote verification and proportional checks autonomously; (c) future source tasks use one checkout, short-lived conventional branches/PRs only if authorized by the relevant order, and no linked worktrees; (d) no force-push, reset/cleanup of unknown data, history rewrite, extra clone, implicit deployment, or external repository write; (e) protected branch policies and merge permissions are respected; merges and releases remain separately Owner-authorized or **expressly delegated in the applicable work order**, not inferred from push permission; (f) exact branch/HEAD/tree/remote verification and a clean final working tree are evidence requirements; (g) completed work is pruned from the three live WIP files, with Git history retaining past orders; (h) an agent uses installed authenticated tooling and reports a real permissions failure, not a routine request for Owner to run Git commands. **Do not weaken** the two-role / Owner-approval separation or write confinement.
- `.github/SECURITY.md`: only **after** Phase A independently confirms private vulnerability reporting is enabled, replace obsolete draft statements that it is unconfigured with accurate instructions directing reporters to GitHub's private vulnerability reporting. Clearly state that **there are no published application versions yet**; do not invent security response times, confidential email contacts or a release support commitment.
- `.documents/WIP/plan-next.md`: replace the obsolete `SOLO-WO-001` bootstrap candidate with **one non-authorizing next candidate**: architectural acceptance of the first implementation contracts (Entity/Facet/Site/Page IDs, minimal authoring data structure, base URL/route grammar, and cascading property types), explicitly without selecting a framework or authorizing code. Do not add a second work order or planning archive.
- `.documents/WIP/plan-outlook.md`: prune outdated pre-Git-bootstrap and parent-ignore blockers, update the short-term outlook to reflect the newly available GitHub baseline, retain concise short/medium/long horizons, and keep the documentation outlook. Do not add completed-task history.

Keep `.documents/WIP/workorder-active.md` **exactly as installed by the Owner** throughout execution; it is the frozen issued order. Report execution evidence in the final agent response rather than rewriting its Approved Scope or adding report files. Do **not** modify `.documents/Architecture/ARCHITECTURE.md`, `.documents/About/FOUNDATION-SOLO.md`, `LICENSE`, other `.github/` files or any application/source code.

**Phase C — Commit, publish and verify**

1. Inspect the complete intended diff and require that changed paths are **only** the Owner-installed work-order replacement and the seven named files; no secrets or unrelated edits. Stage **only those exact paths**—never blind `git add -A` or broad staging. Create **one ordinary local commit** on `main` with a factual message such as `chore: publish Solo repository and align Git governance`. Preserve the `SOLO-WO-001` root commit intact.
2. Add exactly one Git remote named `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo.git` (or the equivalent authenticated GitHub URL for this same repository). Verify that the remote has no unrelated commits; the new repo was created empty.
3. Push local `main` to `origin/main` and set upstream. A GitHub CLI/MCP push equivalent is acceptable, but must publish the **same local commit/tree and its existing parent**, without merge, force or history replacement. Verify the GitHub repository's **actual** `nameWithOwner`, **PUBLIC** visibility, default branch `main`, `origin/main` SHA equal to local `HEAD`, readable `README.md`, `LICENSE`, and the three WIP files, plus the private vulnerability reporting state `enabled: true`. If the default branch is not `main` after a successful push, set **only this new repository's** default branch to `main`, then verify.
4. Validate the actual Solo working tree is clean and has just its one main worktree, no added shadow directories, no extra remotes, no secret-bearing generated files, and no new WIP files. Remove only disposable files **created by this task** inside Solo `.temp/`, and remove an empty task-created `.temp/` directory; never delete pre-existing or Owner-owned material. Confirm the parent repo HEAD, ignore rule and scoped status are unchanged **during Solo execution**. End without beginning a new task.

## 5. Acceptance evidence — actual outputs, not assertions

The final agent response must include:

1. Exact `SOLO-WO-002 R01` issuance and matching re-issuance handoff verification, with the previous R00 stop accounted for, and the preflight local baseline (`b924729…`, branch `main`, original tracked 17 files, Owner-installed work-order diff only), with any identified publication risks.
2. Repository URL and actual GitHub **owner login/type (`provelopment`, `User`)**, visibility and default branch; how the agent verified its authority and that no existing repository was overwritten.
3. Proof that private vulnerability reporting is enabled (API result or comparable authoritative response), and `.github/SECURITY.md` accurately describes it.
4. Local final HEAD, parent chain (unchanged original root commit), new commit ID, `git remote -v`, `git branch -vv`, remote `main` commit ID, and direct remote file/tree verification. Confirm these agree.
5. Exact file-diff summary; assertion backed by `git diff --name-only`/`git show --stat` that only allowed paths were modified. No code, dependency or license change.
6. GitHub pull/push and permission results; exact documentation/SOP rules changed; status of next candidate and three WIP slots. No fabricated CI/build/test claims—there remains no runnable application under this order.
7. Proof of clean local Solo status, single worktree, no extra branches/remotes, no surviving agent-created `.temp`/cache, no out-of-Solo file writes, and no change to parent/sibling repositories.
8. Any deviation, failed setting, unexpected public data, permissions limitation or unfinished part reported plainly with a safe stop point. Never claim remote publication complete if it failed.

**Architect review gate:** The Architect will independently inspect the new public GitHub repository, exact commits and documents, and return `ACCEPT`, `REVISE` or `STOP`. Agent self-report is not final acceptance. The next active order is written by the Architect, reviewed and **installed by the Owner as the replacement of this same `workorder-active.md` file**. Git history holds `SOLO-WO-001` and `SOLO-WO-002`; no additional WIP files are created.

## 6. Stop conditions and recovery boundary

STOP rather than guessing if: Owner has not installed/handed off this exact order; preflight baseline is unexpected; public-history scan finds confidential/unlicensed material; creation authority in the **approved personal `provelopment` namespace** or private vulnerability reporting permission is missing; remote name or repository conflicts; a GitHub action would create a repository under a different owner or account; a command requires an out-of-root file write; an unanticipated hook/signature/credential prompt would escape Solo; branch protection blocks the permitted push; a remote commit differs unexpectedly; the parent/sibling repo would be touched; or required temp cleanup would destroy unknown files.

If a partial external state exists, **preserve and report it**: don't delete the public repo, force-push, rewrite local history, silently switch repositories, expand access or perform an unauthorized workaround. When possible, finish the remainder of this exact approved scope with the same authenticated tooling; if a material gate still fails, stop and give a concrete unblock proposal. Do not ask the Owner to run ordinary Git commands as a substitute for tools already available to the agent.

## 7. Authorization ledger

| Action | Approval under this exact order |
| --- | --- |
| Create public `provelopment/provelopment-foundation-solo` in the existing personal `provelopment` account (`User`) | **YES**, after preflight |
| Enable private vulnerability reporting in that repository | **YES** |
| Make the seven specified Solo-document changes | **YES**, bounded by §4 |
| Commit once on local `main`; add `origin`; push `main`; set default `main` | **YES** |
| Create additional WIP files or revise Approved Scope | **NO** |
| Modify any other local repository or GitHub repo | **NO** |
| Worktree/clone, force-push, rebase, reset, amend, history rewrite | **NO** |
| Code, dependencies, CI, branch protection, PR/merge, release, deploy, DNS | **NO** |
| Account conversion, organization creation, paid services, account-wide settings, other secrets/security config | **NO** |

**Owner issuance record:** The Owner’s deliberate installation of this exact R01 file and a **new, explicit `SOLO-WO-002 R01` handoff** are the authority. The earlier R00 handoff does not carry over. An Architect-authored candidate, an earlier chat instruction or a GitHub issue/plan alone cannot issue it.
