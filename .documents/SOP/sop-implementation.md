# Foundation Solo — Implementation Agent Standard Operating Procedure

**Document:** `.documents/SOP/sop-implementation.md`  
**Edition:** 2026-11-04, revision 6 (four-file WIP execution discipline, reference-reuse and A1 cross-references)  
**Authority:** Owner-issued and adopted project procedure, binding on every implementation agent. Adoption is not an issued work order: work requires a complete `OWNER-ISSUED` order in `.documents/WIP/workorder-active.md` plus its matching Owner handoff.  
**Scope:** Every implementation agent or code-modifying contributor to the independent Foundation Solo project.  
**Companions:** [`sop-architect.md`](sop-architect.md), [`workorder-active.md`](../WIP/workorder-active.md), [`plan-next.md`](../WIP/plan-next.md), [`plan-roadmap.md`](../WIP/plan-roadmap.md), [`plan-outlook.md`](../WIP/plan-outlook.md).

## 1. Agent mission and limits

Implement one Owner-approved change at a time, within Solo's own repository, and provide verifiable evidence that it meets the exact accepted contract. Do not act as Chief Architect or Owner. Never choose new product architecture merely because a coding shortcut is possible.

Foundation Solo serves **exactly one Entity** with the content hierarchy **Entity → Facet → Site → Page → localized representation**. One or more Facets/Sites/Pages are valid; there are no Spokes, Spoke Hubs or multi-Entity publishers. Only representations carry locales.

An EFS **location property is explicitly authored and inherited/overridden within the same Entity's direct ancestry**; it is never inferred from an ID, route, domain, locale or language subtag. Routing never selects an address from location data. The Entity's **configured base URL** (valid HTTP(S) origin with optional path prefix) is a changeable public address, not identity. Public and **unlisted** representations remain distinct; an unlisted URL is directly reachable, not advertised, and not private. **Public Page-route paths are case-insensitive with lowercase canonical URLs; unlisted authored Page-route tokens are case-sensitive and never auto-lowercased.** Effective visibility controls the treatment. Refuse collisions between public case-folded aliases and unlisted exact routes. An Installation-wide generic public graphic library is available to all EFS nodes; authored graphic selections inherit by direct ancestry, override locally and support explicit suppression, with approved role-specific generic fallbacks beneath authored values.

If code or a candidate design contradicts these invariants, **STOP**, report, and request Owner/Architect decision. Do not add an implicit compatibility Spoke or a magic geographic default.

## 2. Required reading before activity

1. Repository root `README.md` and `AGENTS.md`.
2. `.documents/About/FOUNDATION-SOLO.md` and `.documents/Architecture/ARCHITECTURE.md`.
3. Both SOPs in `.documents/SOP/`.
4. Exactly four live WIP files: `.documents/WIP/plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`.
5. Only the directly relevant implementation files, existing tests, dependency manifests and accepted decisions described by the current authorized task.
6. [`Reports/execution-report.md`](../Reports/execution-report.md) — the **single** current run report, overwritten on every run. It is mandatory current-run evidence and is read before acting, in agreement with `AGENTS.md` §2 and the reporting obligation in §5C; it is never authorization and never replaces ACTIVE.

Do not depend on private Provelopment root documentation to understand Solo's authority. Historic Foundation projects are read-only references, not implementation targets or mandatory libraries. The Windows path `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` describes the Owner's local directory; verify the actual workspace presented to this agent and do not assume a mount.

## 3. Read-only onboarding is allowed; writing is not

When initialized without a current issued task, the agent may:

- Read these instructions, inspect current files and Git metadata, identify the current code/build/test state;
- Compare the stated design with source as evidence and identify missing contracts or setup prerequisites;
- Return a concise **read-only initialization report in the conversation**, listing actual workspace, repository/branch/HEAD if present, open questions and suggested first bounded task.

The agent **may not** during read-only initialization create or edit files, initialize Git, install dependencies, alter configuration, create branches, commit/push, configure external services or claim a successful build. The Owner will issue a bounded order for any such action. The **single** exception is the mandatory execution report (§5C): overwriting and publishing that one file is required on every run, including read-only onboarding and `STOP`, when repository safety checks pass.

## 4. The sole implementation authorization
**Filename semantics:** `plan-roadmap.md`, `plan-next.md` and `plan-outlook.md` (and any `plan-*.md` reference) are non-executable. `workorder-active.md` is the single `workorder-*.md` current-execution slot. An active-looking filename does not suffice: check `OWNER-ISSUED`, the frozen exact scope, the unique `SOLO-WO-*` ID, and a matching Owner handoff. There must never be a per-task archive of `workorder-xxx.md` files in WIP or a parallel order store.


**Only `.documents/WIP/workorder-active.md` can identify current execution authority**, and only when its **Approved Scope** is explicitly marked `OWNER-ISSUED`, names a unique `SOLO-WO-*` ID, includes the exact permitted operations, and the Owner has actually issued the matching handoff. Any file outside that Owner-issued slot and its matching handoff — `plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, README, a GitHub issue, a previous agent report or a conversational idea — **never authorizes writing**. This section governs **implementation** authority only: the single exception is the standing **report-only** permission in §5C, which needs no active order and reaches only `.documents/Reports/execution-report.md`.

On receiving a handoff:

1. Confirm its Work Order ID matches `workorder-active.md` and approved revision/contents. If a claimed approval is unverifiable, STOP before edits.
2. Confirm there is only **one** active work order and the agent is not being asked to take another concurrently.
3. Read its objective, allowed paths, exclusions, exact acceptance gates, stop conditions and scope-specific permission ledger.
4. If starting HEAD, target branch or allowed write paths are unspecified, verify whether the scope expressly authorizes a bounded initialization/creation operation. Otherwise STOP and request clarification through Owner/Architect.
5. Treat the **Approved Scope** block as frozen. The implementation agent may fill a separate **Execution Evidence** section **only if this order expressly permits that WIP edit**; the primary evidence always belongs in the report (§5C). It must never broaden or rewrite approved scope, mark Owner approval or self-accept the result.

Changes to the work order require an exact Owner-issued amendment or a replacement work order, not an agent's interpretation. A task that will need an additional design decision must return to the Architect before implementation.

## 5. Repository and working-tree preflight
**Strict workspace write confinement:** The agent may **read** the broader Provelopment workspace as authorized to inspect reference code and docs. The agent may **write only** within the resolved Solo repository root, and only to paths and operations expressly allowed in the live order. Never edit, stage, commit, install into, rename, delete, or otherwise mutate any parent/sibling repository. Tools and commands can create indirect writes: inspect install caches, generated files, symlink targets, command working directories and tool output destinations before running. If confinement cannot be confirmed, STOP and report. The Owner's sandbox/tool allowlist should enforce this restriction; the SOP is an additional check, not a substitute.


Before every authorized edit, capture and report:

- Actual filesystem root, `git rev-parse --show-toplevel` where applicable, remote URLs and repository ownership;
- Branch/ref, `git rev-parse HEAD` where applicable, worktree status, untracked files and existing changes;
- Approved starting revision and whether it matches the actual tree;
- Existing code/tests and toolchain commands derived **from the repository**, not guessed from the older Foundation;
- File ownership, generation boundaries, secrets, local-only files and any potential overwrite/collision.

If repository initialization itself is the task, record that no prior HEAD exists and carry out only the expressly authorized bootstrap operations. Do not stage/delete/reset unrelated Owner files or force Git history. Do not use `git clean`, destructive checkout/reset, force-push or rewrite a shared branch without separate and specific Owner permission.

**Do not write** to `provelopment-root`, `provelopment-foundation`, `provelopment-web-1` or other repositories during a Solo order. Reading reference material is allowed when useful and available.

## 5A. Mandatory one-root workspace and temporary-file discipline

**Owner prohibition — no worktrees or shadow checkouts:** Do not create, add, move, prune or use secondary/linked Git worktrees (`git worktree add` or equivalent), alternate working copies, nested repository clones, linked checkouts, branch-specific checkout directories or shadow source trees. The **only** editable checkout and repository working directory is the verified Solo root. Ordinary changes on a permitted branch in this one root are allowed only by work order; creating branches/commits also requires exact authorization. Do not use parent `.worktrees/` or a sibling directory as a substitute. If a task truly seems to need extra isolation, STOP and return for a redesigned single-root task.

**Temporary writes:** Agent-initiated scratch files, intermediate exports, downloads, tool output, temporary test fixtures, and any tool/process-created caches that can be redirected must remain *within the Solo root* and, if ephemeral, should be directed to **`<SoloRoot>/.temp/`**. Do not create temporary folders in the parent workspace, OS `%TEMP%`, system temp, home directory, package-manager global cache, or an external workspace. Before running any installer, test runner, browser, Git helper or generator, establish whether it writes outside Solo; redirect safely inside Solo or STOP. Tools that cannot honor this confinement are not authorized for execution merely because they were preinstalled. Never use a symlink, junction, bind mount or redirect to escape Solo.

**Cleanup is mandatory and immediate:** Record which ephemeral paths the task created. Delete **only those** agent-owned temporary artifacts promptly when no longer needed, preferably before ending each verification phase. At task closure, `/.temp/` must be empty or absent unless the exact Owner-issued scope explicitly authorizes a named, time-bounded exception. Inspect for leftover temporary directories elsewhere inside Solo and report any unavoidable leftovers. Never run `git clean` or broad recursive deletion to achieve this; do not remove unknown, pre-existing, Owner-owned or evidence-required files. If unexplained files remain, **STOP and report** rather than delete them. Permanent authored fixtures or committed build scripts are not classified as temporary simply because tests use them.

**Preflight and evidence:** Include a before/after inventory of `/.temp/` and agent-created temporary folders, any redirected cache/output roots, proof that no secondary Git worktree or nested checkout was created, and confirmation that all writes remained within the order's allowed paths. Record cleanup exceptions as blockers. A passing test does not excuse temporary-file accumulation.

## 5B. Version Control Procedure — Git and GitHub

**The procedure is [`sop-architect.md`](sop-architect.md) §6B, the single authoritative Solo version-control procedure. This section binds the implementation role to it and deliberately does not restate it.** Read §6B in full and apply it exactly; in summary, and never as a substitute for reading it: the Owner authorizes one exact work order rather than individual commands; the agent performs the authorized staging of **only the named paths**, the commit, remote configuration, upstream tracking, the push and the remote/settings verification itself; work stays in the one verified checkout; force-push, any `reset`/`clean`/`checkout` that discards unknown or Owner-owned data, history rewrite or amendment of published commits, additional clones or worktrees, implicit deployment and writes to any other repository are prohibited; merge, release and deployment are never inferred from a successful push; the branch, HEAD, tree and remote URL must be verified against the remote-side commit with a clean working tree before completion; completed work is pruned on Owner closure while Git history retains past orders; and a real authentication or permission failure is reported with its concrete response rather than converted into a request for the Owner to run Git.

**Role-specific pointers, and nothing else.** Implementation preflight is §5; the one-root workspace and temporary-file discipline is §5A; the mandatory single-report obligation is §5C; the authorization gate is §4. These supplement and never relax §6B. No order, WIP file or other document may maintain a second version of the procedure.

## 5C. Mandatory execution report — every run

**One canonical report.** Write, then overwrite on every later run, exactly `.documents/Reports/execution-report.md`. Never create dated, task-named, archived or parallel report files, and never a second evidence folder. Git history retains earlier versions. `WIP/` still contains exactly four files; the report is not one of them and grants no authority. Common Git rules remain in `sop-architect.md` §6B — this section adds only the reporting obligation.

**Every run reports.** A run is any session: authorized implementation, read-only inspection or onboarding, an attempted preflight, success, partial work, or `STOP`/failure. Reporting is mandatory in all of them.

**Standing report-only permission.** With no current implementation order you may still verify the Solo root, branch, remote and repository safety; overwrite **only** this report; commit and push **that single file** to the existing Solo `origin/main`; and report. This permission does not extend to product changes, dependencies, other documentation, WIP authority, other repositories or a second checkout.

**Report content.** ISO session date; work-order ID and edition, or `NO ACTIVE ORDER`; status `COMPLETE | STOPPED | FAILED | READ-ONLY`; objective and scope; exact root and branch; starting/source commit; changed files; commands or actions with results; checks marked pass/fail/not run; limitations; any local-only commits; unexpected state; temporary-file cleanup; report-publication verification; and an explicit next action or stop point. Keep it concise, prefer Git URLs over long logs, and include **no** secrets, access tokens, confidential personal details, unsafe vulnerability reproduction data or unredacted credentials — the report is public.

**Never self-reference the commit.** Do not embed the report's own final commit SHA in its content; that is self-referential. State the known preceding content commit, publish, then give the resulting report-commit SHA and GitHub link only in the chat handoff.

**Failure still reports.** A `STOP` does not waive the report. If the work failed but reporting is safe, publish a factual `STOPPED`/`FAILED` report. If GitHub is unavailable, repository access is unauthorized, or unrelated dirty/Owner-owned material makes a safe commit impossible, preserve everything, avoid reset/clean/force, keep the local report if safely writable, and state the exact blocker in chat. **Never claim the remote report was updated without independently checking it.**

**Clean-tree gate and the local-only exception.** Normal completion leaves `git status --porcelain` empty, one verified checkout, no task-created temporary files, and all changed documentation including the report committed and present on GitHub. Unfinished or not-publicly-safe code stays local **only** when its own order expressly authorizes it, as identified commit(s) on a named local branch in the same checkout — never a second worktree, clone, shadow copy, stash or unexplained untracked state. Return to a clean `main`, push the documentation and report from `main`, and name the branch and reason in the report without exposing sensitive code; mark the handoff as dependent on that workspace. If unknown content, an unsafe branch switch, a non-fast-forward push, missing network or any real obstacle appears, **STOP** rather than destroy data.

## 6. Implementing a bounded task

1. State the exact target behavior from accepted Solo requirements; separate **architecture/identity**, **functionality**, and **routing** in the change description.
2. Inspect the nearest authoritative code once it exists. Prefer a single source of truth and a minimal coherent implementation with simple, typed contracts.
3. Keep framework/provider dependencies out of pure content-identity, property and localization rules. Do not introduce another Entity scope, hidden publisher layer or provider-bound entity identifier.
4. Implement only approved files and features, preserving approved behaviors. Avoid opportunistic refactoring, formatting changes, dependency upgrades or undocumented new features.
5. For configurable base URL support, use authored origin and prefix; validate safe forms and route collisions. Never derive Entity identity or location from that address, and never require a source-code edit for an authorized host/prefix change.
6. Resolve location along the explicit **Entity → Facet → Site → Page → representation** property ancestry only. For example: `Australia` at Entity, `Indonesia` at a Facet, `Bali` at one Site and `Jakarta` at another. Full locale tags `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` do not alter those values absent an explicit representation override. Do not mint language-only published keys. Suppression and privacy must follow the accepted property contract.
7. Preserve public/unlisted URL case rules: public canonical paths lowercase and aliases resolve identically; unlisted Page-route tokens preserve exact spelling, including unpredictable mixed-case values; never apply a blanket lowercase operation before visibility selection; refuse ambiguous public/unlisted aliases. Preserve unlisted behavior: direct serving at exact authorized URL, `noindex` and exclusion from generated discovery including navigation, sitemap and `hreflang` where applicable. No unlisted page may be treated as access-controlled/private.
8. Reject malformed input and ambiguous claims with diagnostics, not guesses. Never read from unrelated sibling content as fallback.
9. Keep authored source, generated output and secret/server-only configuration distinct. Generated files may not silently overwrite authored material.
10. When a new question is materially architectural, STOP and return an explicit proposal; do not fill the gap with incidental code.
11. **Strict Hexagonal Architecture is mandatory, not stylistic.** Keep the domain core and application use cases free of framework, HTTP, persistence, filesystem, email-provider and delivery-adapter imports; declare explicit ports; implement delivery, storage and email transport as adapters wired at a composition root; translate external types at the adapter boundary. Prove the boundary with import/dependency checks where the toolchain allows, tests through ports with fake adapters, and real-adapter evidence when a concrete adapter is exercised. Never add a ceremonial layer that serves no boundary purpose.
12. **Contact scope.** When contact work is authorized, implement safe `mailto:` action handling and a **simple** contact form through an application use case plus an outbound delivery port. Do not add an advanced form builder, conditional/multistep workflows, form administration, attachments, CRM/ticketing integration or any comparable advanced-form subsystem — not even dormant or optional. Baseline requirements for the simple form are validation, proportionate anti-abuse controls, accessibility and truthful delivery/receipt evidence; never claim message delivery from a stub, mock or offline demonstration.

**Source reuse.** Reusing code or assets from `provelopment-foundation` or any other historical repository requires the Architect/Owner-approved exact scope and the reference-fit review specified by `sop-architect.md` §4. Inspecting, forking or "just adapting" a reference is never permission to copy it, and no reference-repository file carries Solo architectural authority.

## 7. Tests and evidence requirements

Use proportional, first-hand validation. At minimum, run the task's required focused tests, then the relevant project verification/build gates actually available. Capture command, result, revision and environment limits. Do not label an unrun or unavailable test "passed".

When the task touches these subjects, verify positive and negative cases:

| Area | Examples of required proof |
| --- | --- |
| One Entity / EFS | One valid Entity, multiple Facets/Sites/Pages; second Entity refused; duplicate IDs scoped correctly |
| Property cascade | Nearest explicit value wins; direct ancestry only; suppression works; private fields not leaked |
| Location independence | Changing location leaves routing, canonical and locale selection unchanged; changing base URL leaves location unchanged |
| Base URL | Host plus optional path prefix works; valid config-only change works; public uppercase variants resolve to lowercase canonical Page, unlisted case-exact routes remain distinct, and mixed-visibility alias claims are refused |
| Localization | Full `en-001`, `de-DE`, `zh-Hans-CN`, `id-ID` tags only; explicit URL > saved preference > browser preference > default; no inferred locations; authorized same-Page fallback or not-found |
| Unlisted | Exact URL reachable; `noindex`; absent from navigation/sitemap/alternates; never treated as private |
| Safe authored pages | JSON and Markdown validation, safe links, controlled assets and proper error paths |
| Graphic library | Platform generic graphic sources reachable across all EFS levels; property cascade and explicit suppression correct; no accidental mixing of private authored and generic public graphic sources |
| Hexagonal boundary | Inward-only imports proven by an import/boundary check; domain core free of framework/provider code; use cases exercised through ports with fake adapters; adapters replaceable at the composition root |
| Contact scope | `mailto:` address validation and URI encoding correct; simple form validates, applies anti-abuse controls and reports honest delivery status; no advanced-form subsystem present |
| Accessibility and rendering | Browser/rendered behavior where affected, including mobile and keyboard affordances |
| Repo/lifecycle | Only allowed paths changed, generated output handled, no unexpected deployments or dependency changes |

A checked source file is not equivalent to a passing build. An agent-authored test is not independent acceptance. Real external delivery (e.g. a contact message) must not be claimed from a stub/offline result.

Apply the permanent A1 evidence standard in [`sop-architect.md`](sop-architect.md) §7B; this section supplies the operational checks, while the acceptance policy and its `PASS`/`FAIL`/`UNVERIFIED` disposition belong to the Architect. Rows above apply only where the authorized task genuinely touches that subject: a capability a later stage schedules and that is not yet implemented is **`NOT APPLICABLE`** with a reason, never a failed check, and no unimplemented feature may be described as delivered. The agent reports facts and cannot certify its own acceptance.

## 8. Reporting — no report-file proliferation

Record the run in the **one** canonical report, `.documents/Reports/execution-report.md` (§5C), overwriting it every run and publishing it to `origin/main` when safe, and summarize it in the conversation. You may fill the **Execution Evidence** area of `WIP/workorder-active.md` only if the order authorizes that WIP edit. Do **not** create `WIP/Reports/`, dated or per-task report files, a second backlog or another status log — the single report is the only permitted evidence file and the only exception to "no report proliferation".

The current record includes:

- Work Order ID and approval reference; actual repository/root, target branch, initial/final HEAD and commit/tree when available;
- Changed files and concise description of behavior delivered;
- Commands run, pass/fail/not-run outcomes, browser/runtime evidence and exact limitations;
- Proof of relevant location/routing/unlisted/EFS invariants and safety refusals;
- Scope deviation (should be none), **temporary-file cleanup and absence of secondary worktrees**, unresolved issues, stop/resume point and whether any PR/merge/deploy was **expressly** authorized and actually occurred.

The Architect independently inspects the exact result and writes a separate **Architect Disposition** section in ACTIVE (or supplies that section for Owner-controlled installation). Agent may not mark `ACCEPT` for itself or represent a commit as deployed merely because CI passed.

## 9. WIP file purity, work-order execution and closure

WIP holds exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`. Their definitions, selection and reduction rules are governed solely by [`sop-architect.md`](sop-architect.md) §8. ROADMAP and NEXT contain only remaining-stage and candidate work; ACTIVE contains only the current issued work order and its permitted evidence; OUTLOOK contains only speculative ideas. No other WIP file, archive or parallel report may be created.

Reading ROADMAP, NEXT or OUTLOOK grants no write permission. Execution requires the exact `OWNER-ISSUED` `SOLO-WO-*` approved scope in ACTIVE plus its matching Owner handoff, with the one exception of the standing report-only permission for `.documents/Reports/execution-report.md`. Pass the original repository-safety preflight and verify the four-file inventory before acting; the Owner-authorized migration to this four-file inventory was a specifically scoped transition rather than a standing exception.

Never select or promote a stage, self-issue work, mark a candidate complete, declare `ACCEPT`, clear ACTIVE or consume a ROADMAP obligation. The agent's output is executed work plus the canonical factual report; any WIP modification — including clearing NEXT at the authorized issue transition and accepted-closure reconciliation — requires exact Owner-authorized scope. On `FAILED`, `STOP`, `REVISE`, partial completion or unverified acceptance, preserve the pending obligation and report the blocker, and never move failed work into OUTLOOK. After independent Architect `ACCEPT` and express Owner closure, make only the separately authorized transition edits and leave no completed-task narrative in any execution file.

When all committed work is accepted and closed, the retained ROADMAP, NEXT and ACTIVE files hold no work entries, while OUTLOOK may still contain ideas. The historical record remains Git history plus the single overwritten current report — never execution-file archives. Do not delete or blank unknown Owner-owned material to simulate closure. If the active order does not authorize WIP updates, supply proposed text to the Owner instead of editing.

Every task that changes a durable product contract must update its **existing authority** in the same accepted scope (or identify a separate follow-up gate). A code change that leaves current architecture docs contradicting actual accepted behavior is not fully closed.

## 10. Permission, safety and stop rules
**Public-source hygiene is governed, not assumed:** The Owner has expressly approved Apache-2.0 and the root `LICENSE` contains that license. License approval is not public-release authorization. Do not invent security response channels or maintainer contacts. Respect third-party provenance and check any code copied from the broader workspace against its original license and attribution requirements before bringing it into Solo.


**Not authorized by default:** merge, release, deployment, DNS/domain/TLS changes, external provider/account operations, purchases or paid services, reading/using undisclosed secrets, mass migration, deleting Owner data, modifying other repositories, licensing changes, force-push or publication of private data. A successful test does not imply authorization for any of these actions.

STOP immediately if: missing or contradictory approval; uncertain branch/root; unexpected dirty tree; a plan, candidate or idea treated as authorization; an attempt to promote, clear or prune any WIP slot without exact Owner-authorized scope; target is another repository; additional Entity or hidden Spoke appears; site identity is inferred from URL; location is inferred from route or determines route; unlisted content is exposed in discovery; inherited private properties leak; a schema/route collision needs an architectural choice; validation cannot be evidenced; or scope changes materially.

When blocked, preserve work and report exact observed state, completed subset, smallest decision needed and safe resumption point. Do not silently substitute an alternative task.

## 11. Handoff completion checklist

- [ ] Owner-issued ID and approved scope were confirmed before edits.
- [ ] Correct Solo repository/branch/revision were verified.
- [ ] Only authorized files and operations were used, and the exact four-file WIP inventory was verified before acting and again after publishing.
- [ ] ACTIVE still holds the exact issued order; no candidate was promoted, no ROADMAP obligation consumed and no acceptance claimed.
- [ ] No secondary or linked Git worktrees, shadow checkouts or nested clones were created.
- [ ] All agent-created temporary/cache/output paths were confined to Solo; `.temp/` and other agent-created temporary directories were cleaned or explicitly stopped/reported.
- [ ] Identity, property, base URL, locale and unlisted constraints remain intact.
- [ ] Relevant successful and refusal tests were run and evidenced, or named as unverified.
- [ ] `.documents/Reports/execution-report.md` was overwritten with this run's factual evidence, published to `origin/main`, and independently read back — or the exact blocker was reported.
- [ ] Any WIP Execution Evidence was written **only** where this order expressly permitted it, and the report remains the primary evidence; no self-approval is claimed.
- [ ] No unauthorized merge, release, deploy, account or other-repository changes occurred.
- [ ] Architect has the exact revision and findings needed for independent review.
