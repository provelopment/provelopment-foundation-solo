# Foundation Solo — Implementation Agent Standard Operating Procedure

**Document:** `.documents/SOP/sop-implementation.md`  
**Edition:** 2026-11-04, revision 6 (four-file WIP execution discipline, reference-reuse and A1 cross-references)  
**Authority:** Owner-issued and adopted project procedure, binding on every implementation agent. Adoption is not an issued work order: work requires a complete `OWNER-ISSUED` order in `.documents/WIP/workorder-active.md` plus its matching Owner handoff.  
**Scope:** Every implementation agent or code-modifying contributor to the independent Foundation Solo project.  
**Companions:** [`sop-architect.md`](sop-architect.md), [`workorder-active.md`](../WIP/workorder-active.md), [`plan-next.md`](../WIP/plan-next.md), [`plan-roadmap.md`](../WIP/plan-roadmap.md), [`plan-outlook.md`](../WIP/plan-outlook.md).

## 1. Agent mission and limits

Implement one Owner-approved change at a time, within Solo's own repository, and provide verifiable evidence that it meets the exact accepted contract. Do not act as Chief Architect or Owner. Never choose new product architecture merely because a coding shortcut is possible.

Foundation Solo serves **exactly one Entity** with the content hierarchy **Entity → Facet → Site → Page → localized representation**. One or more Facets/Sites/Pages are valid; there are no Spokes, Spoke Hubs or multi-Entity publishers. Only representations carry locales.

An EFS **location property is explicitly authored and inherited/overridden within the same Entity's direct ancestry**; it is never inferred from an ID, route, domain, locale or language subtag. Routing never selects an address from location data. The Entity's **configured base URL** (valid HTTP(S) origin with optional path prefix) is a changeable public address, not identity. Public and **unlisted** representations remain distinct; an unlisted URL is directly reachable, not advertised, and not private. **Public localized route claims match case-insensitively by default with lowercase canonical spellings, an author may explicitly select case-sensitive matching for an individual claim (preserving authored Unicode capitalisation), and unlisted authored Page-route tokens are always case-sensitive and never auto-lowercased.** Effective visibility and the claim's declared matching policy control the treatment, and locale segments normalize independently of Page-route case. Refuse ambiguous claim overlaps, collisions between public case-folded aliases and unlisted exact routes, and duplicate canonical addresses. An Installation-wide generic public graphic library is available to all EFS nodes; authored graphic selections inherit by direct ancestry, override locally and support explicit suppression, with approved role-specific generic fallbacks beneath authored values.

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
**Strict permanent-write confinement:** Permanent source, documentation, configuration and Git changes belong only within the verified Solo root and the current order's exact path-and-operation ledger. For disposable task files, **both** Solo `.temp/` and the active Cline-provided temp area are permitted under §5A, with required cleanup. No parent/sibling repository writes or additional worktrees/checkouts. Inspect tool effects, symlinks and destinations; an unknown or unrelated write target is not authorized. The Owner's permissions and the SOP remain complementary safeguards.


Before every authorized edit, capture and report:

- Actual filesystem root, `git rev-parse --show-toplevel` where applicable, remote URLs and repository ownership;
- Branch/ref, `git rev-parse HEAD` where applicable, worktree status, untracked files and existing changes;
- Approved starting revision and whether it matches the actual tree;
- Existing code/tests and toolchain commands derived **from the repository**, not guessed from the older Foundation;
- File ownership, generation boundaries, secrets, local-only files and any potential overwrite/collision.

If repository initialization itself is the task, record that no prior HEAD exists and carry out only the expressly authorized bootstrap operations. Do not stage/delete/reset unrelated Owner files or force Git history. Do not use `git clean`, destructive checkout/reset, force-push or rewrite a shared branch without separate and specific Owner permission.

**Do not write** to `provelopment-root`, `provelopment-foundation`, `provelopment-web-1` or other repositories during a Solo order. Reading reference material is allowed when useful and available.

## 5A. Mandatory one-checkout and temporary-workspace cleanup discipline

**One repository checkout, always.** Use the verified Solo root as the only editable repository. Never create or use linked Git worktrees, additional checkouts, cloned or nested repositories, shadow source trees or branch-specific checkout directories, including inside either temporary workspace. Only the current Owner-issued task authorizes permanent edits; the standing report-only exception stays narrow.

**Work freely in either temporary area.** The Owner permits use of both `<VerifiedSoloRoot>/.temp/` and the temporary workspace(s) actually provided to this run by Cline, including Cline-managed temporary locations outside Solo. You may create, read, write, reorganize and remove the temporary files and folders needed for the task, including tool caches, downloads, command capture, logs, generated output, test fixtures and intermediates, without asking for additional scratch permission. Use the available Cline runner normally. Do not treat other OS temp folders, user-profile folders or unrelated workspaces as a general-purpose destination; the exception is the identifiable Cline-provided task area.

**Keep scratch out of Git and out of permanent scope.** Solo `.temp/` is ignored by root `.gitignore`; confirm that no content beneath it is tracked, staged or committed, and never use force-add. Neither scratch area is a second codebase or a place for permanent deliverables. Permanent source and documentation changes must remain within the authorized Solo path ledger. Do not modify parent or sibling repositories, create another worktree/checkout or expose secrets in command output.

**Clean when done, normally before ending the run.** Keep track of the disposable files/directories created by your actions and by the current Cline execution where task ownership is identifiable. Remove each one as soon as it is no longer needed, including agent-owned Cline capture logs when accessible. Before declaring a run complete, inspect the relevant task-owned parts of **both** temp areas and clean no-longer-needed material. Do not leave garbage for later sessions. Do not use `git clean`, delete an entire shared temp root, or remove unfamiliar, pre-existing Owner-owned or unrelated files; delete only positively identified task-owned material. A genuinely necessary item may remain only with its purpose and a specific future cleanup trigger recorded in the canonical report. If known obsolete task-owned files cannot be removed safely or are inaccessible, report an outstanding cleanup failure rather than claiming full cleanup.

**No artificial Cline STOP.** Cline's ordinary host-managed command-capture logs and other task output within its supplied temp workspace are permitted work artifacts, not an automatic outside-root violation and not a reason to avoid using the runner. Host-generated files are not exempt from the cleanup obligation where identifiable and controllable. Do not force changes to external IDE/OS settings just to redirect an already permitted Cline temp path. If an actual permanent outside-Solo write, a second checkout, deletion risk to unknown material, suspected secret exposure or unbounded/unrelated temp target is discovered, STOP and report rather than conceal it.

**Record evidence honestly.** In the one canonical execution report, state the Solo `.temp/` Git-ignore/tracked/staged checks, the run's Cline temp area by safe category (not private full path), inventories sufficient to identify task-owned files, cleanup actions and results, any retained item with purpose/trigger, and any unresolved garbage. Distinguish local observations from GitHub evidence. A clean Git working tree does not substitute for cleaning disposable files and folders.

## 5B. Version Control Procedure — Git and GitHub

**Binding authority:** [`sop-architect.md`](sop-architect.md) §6B is the **single authoritative Solo version-control procedure**. The Implementation Agent must read and follow that section in full. This §5B creates no parallel procedure, independent Git permission or substitute summary.

**Implementation-role cross-references:** Verify authorization under §4; perform repository and working-tree preflight under §5; enforce single-root and temporary-file discipline under §5A; and publish the canonical execution report under §5C. These role-specific requirements supplement, and never relax, Architect SOP §6B.

## 5C. Mandatory execution report — every run

**One canonical report.** Write, then overwrite on every later run, exactly `.documents/Reports/execution-report.md`. Never create dated, task-named, archived or parallel report files, and never a second evidence folder. Git history retains earlier versions. `WIP/` still contains exactly four files; the report is not one of them and grants no authority. Common Git rules remain in `sop-architect.md` §6B — this section adds only the reporting obligation.

**Every run reports.** A run is any session: authorized implementation, read-only inspection or onboarding, an attempted preflight, success, partial work, or `STOP`/failure. Reporting is mandatory in all of them.

**Standing report-only permission.** With no current implementation order you may still verify the Solo root, branch, remote and repository safety; overwrite **only** this report; commit and push **that single file** to the existing Solo `origin/main`; and report. This permission does not extend to product changes, dependencies, other documentation, WIP authority, other repositories or a second checkout.

**Report content.** ISO session date; work-order ID and edition, or `NO ACTIVE ORDER`; status `COMPLETE | STOPPED | FAILED | READ-ONLY`; objective and scope; exact root and branch; starting/source commit; changed files; commands or actions with results; checks marked pass/fail/not run; limitations; any local-only commits; unexpected state; temporary-file cleanup; report-publication verification; and an explicit next action or stop point. Keep it concise, prefer Git URLs over long logs, and include **no** secrets, access tokens, confidential personal details, unsafe vulnerability reproduction data or unredacted credentials — the report is public.

**Never self-reference the commit.** Do not embed the report's own final commit SHA in its content; that is self-referential. State the known preceding content commit, publish, then give the resulting report-commit SHA and GitHub link only in the chat handoff.

**Failure still reports.** A `STOP` does not waive the report. If the work failed but reporting is safe, publish a factual `STOPPED`/`FAILED` report. If GitHub is unavailable, repository access is unauthorized, or unrelated dirty/Owner-owned material makes a safe commit impossible, preserve everything, avoid reset/clean/force, keep the local report if safely writable, and state the exact blocker in chat. **Never claim the remote report was updated without independently checking it.**

**Clean-tree gate and the local-only exception.** Normal completion leaves `git status --porcelain` empty, one verified checkout, no obsolete task-owned scratch in Solo `.temp/` or Cline's task temporary area, and all changed documentation including the report committed and present on GitHub; any justified temporary retention or known cleanup failure is documented under §5A. Unfinished or not-publicly-safe code stays local **only** when its own order expressly authorizes it, as identified commit(s) on a named local branch in the same checkout — never a second worktree, clone, shadow copy, stash or unexplained untracked state. Return to a clean `main`, push the documentation and report from `main`, and name the branch and reason in the report without exposing sensitive code; mark the handoff as dependent on that workspace. If unknown content, an unsafe branch switch, a non-fast-forward push, missing network or any real obstacle appears, **STOP** rather than destroy data.

## 6. Implementing a bounded task

1. State the exact target behavior from accepted Solo requirements; separate **architecture/identity**, **functionality**, and **routing** in the change description.
2. Inspect the nearest authoritative code once it exists. Prefer a single source of truth and a minimal coherent implementation with simple, typed contracts.
3. Keep framework/provider dependencies out of pure content-identity, property and localization rules. Do not introduce another Entity scope, hidden publisher layer or provider-bound entity identifier.
4. Implement only approved files and features, preserving approved behaviors. Avoid opportunistic refactoring, formatting changes, dependency upgrades or undocumented new features.
5. For configurable base URL support, use authored origin and prefix; validate safe forms and route collisions. Never derive Entity identity or location from that address, and never require a source-code edit for an authorized host/prefix change.
6. Resolve location solely through the direct **Entity → Facet → Site → Page → representation** ancestry using accepted additive `inheritance`/`currentContext` semantics: a Facet can inherit the Entity's Australia fact and add Indonesia; a Site can explicitly suppress inherited facts and contribute Bali or Jakarta; an identified inherited record may be extended only under its accepted validated extension rule. Full locale tags `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` never infer or modify location. Do not mint language-only published keys or use location to determine routes. Public/internal projection and suppression must follow the accepted property contract.
7. Preserve the accepted route-case rules: public localized claims match insensitive by default with lowercase canonical spellings, an author-selected sensitive claim preserves authored capitalisation, unlisted Page-route tokens always keep exact spelling including unpredictable mixed-case values, and locale segments normalize independently of Page-route case; never apply a blanket lowercase operation before the claim policy and effective visibility are resolved; refuse ambiguous claim overlaps, public/unlisted caseless collisions and duplicate canonicals. Preserve unlisted behavior: direct serving at exact authorized URL, `noindex` and exclusion from generated discovery including navigation, sitemap and `hreflang` where applicable. No unlisted page may be treated as access-controlled/private.
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
| Property cascade | Additive direct-ancestry inheritance and valid local values; `suppress` excludes inherited contributions but permits local ones; effective collection cardinality and duplicate/refusal cases verified; per-value provenance and restricted public projection do not leak internal facts |
| Location independence | Changing location leaves routing, canonical and locale selection unchanged; changing base URL leaves location unchanged |
| Base URL | Host plus optional path prefix works; valid config-only change works; an insensitive public claim resolves uppercase variants to its lowercase canonical Page while a sensitive claim stays distinct, unlisted case-exact routes remain distinct, and overlapping or mixed-visibility alias claims are refused |
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

## 7A. Security obligations

These are the implementation-side duties for the security lifecycle owned by [Architect SOP](sop-architect.md) §7C and [Architecture](../Architecture/ARCHITECTURE.md) §5A. They bind only where the authorised scope touches security-relevant behaviour; the Architect owns policy and acceptance, and §6B/§5B remains the sole version-control procedure.

- **Before implementing**, check the [security register](../Security/security-issues.md) and the relevant public advisories for the exact stack being introduced or changed, and record the applicability conclusion for the revision actually present. Never claim an audit, scan or dependency review that was not executed.
- **During implementation**, apply the secure-by-default constraints of Architecture §5A: validate at the boundary and fail closed, refuse rather than coerce malformed or ambiguous input, keep secrets and server-only configuration out of authored content, public output, logs and diagnostics, apply least privilege to adapters and providers, and keep security policy in the domain core rather than in an adapter.
- **Evidence, not assertion.** Map each applicable security condition to the command or inspection that demonstrates it at the exact revision, and include the negative and refusal cases the task actually touches. An unavailable, skipped or inapplicable check is reported `NOT RUN` or `NOT APPLICABLE` with its reason — never `PASS`.
- **Sensitive findings stay private.** A suspected vulnerability in Solo is reported through GitHub private vulnerability reporting per [`.github/SECURITY.md`](../../.github/SECURITY.md), not in a pull request, issue, commit message, chat log or the public register. Do not publish exploit recipes, working payloads or confidential detail in any artefact.
- **The register is evidence, not a queue.** Update an entry only within an authorised order, keep its identifier stable, state only what was actually verified, and never mark a finding `Resolved` without tested evidence. It never authorises remediation and never becomes a fifth WIP file.
- **Report honestly.** Where this repository has no code, manifest, CI or deployment, the applicable security checks are `NOT APPLICABLE` with that reason: a documentation-only result is not a security certification.

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
- [ ] Task-owned temporary/cache/output files used only the authorized Solo `.temp/` or current Cline-provided temporary workspace; no obsolete task-owned files/folders remain without an explicitly reported cleanup failure, and any justified retention has a recorded cleanup trigger.
- [ ] Identity, property, base URL, locale and unlisted constraints remain intact.
- [ ] Relevant successful and refusal tests were run and evidenced, or named as unverified.
- [ ] `.documents/Reports/execution-report.md` was overwritten with this run's factual evidence, published to `origin/main`, and independently read back — or the exact blocker was reported.
- [ ] Any WIP Execution Evidence was written **only** where this order expressly permitted it, and the report remains the primary evidence; no self-approval is claimed.
- [ ] No unauthorized merge, release, deploy, account or other-repository changes occurred.
- [ ] Architect has the exact revision and findings needed for independent review.
