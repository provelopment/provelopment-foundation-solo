# Foundation Solo — Current Work Order

**Work Order ID:** `SOLO-WO-004`  
**Edition:** `R00` — Architect-authored candidate for Owner review  
**Title:** Reconcile canonical architecture, contact-module scope and non-operational status documentation  
**Approval state:** `NOT ISSUED — OWNER REVIEW REQUIRED`. This document becomes `OWNER-ISSUED` **only** when the Owner reviews these exact contents, deliberately installs the file at `.documents/WIP/workorder-active.md` replacing the former contents, and sends a separate matching `SOLO-WO-004` handoff. A downloadable draft, a GitHub commit or this label alone is not authorization.  
**Repository:** `provelopment/provelopment-foundation-solo` — public GitHub repository, existing `origin`  
**Target local root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — verify actual physical path and Git root; do not assume access  
**Target branch:** `main`  
**Expected starting HEAD:** `7ad0a7346ea29a4040c272f357fa8147682109c0` (Architect-verified remote `main` as of 2026-11-04; revalidate before changing anything)  
**Task class:** **Documentation and governance reconciliation only.** No application implementation.  
**Authoring workflow:** Architect writes this complete replacement file → Owner reviews and installs it into the **existing** WIP slot → Owner sends matching handoff → Implementation Agent performs bounded edits, Git commits/pushes and one execution report → Architect independently reviews the report on GitHub.

## 1. Objective and accepted Owner instructions

Remove documented inconsistencies and codify the following Owner decisions in **existing canonical documentation**, without creating new authorities or making technical choices by accident:

1. **Stale parent-ignore statement:** `.documents/Architecture/ARCHITECTURE.md` §7 must stop describing `/00.foundation-solo/` in the parent `.gitignore` as uncommitted. The Solo agent previously verified the parent local commit `2548cc7153f026456bded6f76a2d5d74a567a24e`, an effective ignore rule and zero parent-tracked Solo files. The parent's *remote* publication is not part of this task and must not be invented or altered. Remove the obsolete bootstrap gate from architectural open decisions.
2. **Approved visibility schema:** `.documents/About/FOUNDATION-SOLO.md` must no longer call Page `visibilityDefault` and representation `visibility` recommended/unapproved. These field names, the default `public`, the explicit override rule and JSON/Markdown authoring parity are Owner-approved; only the detailed versioned source encoding, placement and validation mechanisms remain open. Match `.documents/Architecture/ARCHITECTURE.md` §3.2A.
3. **Single report exception:** Correct Architecture §8's obsolete blanket prohibition on Reports directories. `.documents/Reports/execution-report.md` is the **only** permitted current execution-report file, replaced each run and pushed to GitHub when safe; it sits outside the exactly-three-file WIP folder. Do not add any second report, reports archive, or historical status file.
4. **No repository-wide conversational pause status:** The Owner explicitly does **not** want the fact that the Owner and Architect are waiting for further discussion recorded as a project status. Remove `PAUSED`, `pending Architect acceptance`, `Owner reopening` and similar *current-project status* instructions from the live README, AGENTS, SOPs and WIP where they conflate ordinary lack of authority with a documented project pause. A task-free ACTIVE slot is simply `NONE — no current Owner-issued work order`; NEXT may contain **one non-executable candidate** and PLAN retains pruned horizons. No agent starts implementation without a newly Owner-issued work order and matching handoff. Historical Git commits are not rewritten; the execution report may factually identify this correction without treating a conversational pause as product state.
5. **Strict Hexagonal Architecture is mandatory:** Foundation Solo **MUST** follow strict Hexagonal Architecture (Ports and Adapters), rather than merely being 'expected to keep' a domain-centered boundary. Define enforceable inward dependency direction, pure domain rules, application use cases and ports, infrastructure/delivery adapters, and architectural verification. Do **not** choose a framework, runtime, language, storage or deployment provider in this task.
6. **Exact contact-module scope:** Solo's planned contact capability **includes both** safe `mailto:` action handling and a **simple contact form**, via replaceable ports/adapters appropriate to strict hexagonal boundaries. **Advanced contact-form functionality is explicitly excluded** from Solo — not a hidden built-in, an optional module, or a dormant feature. Record this in Product Definition, Architecture and the forward implementation outlook. Essential simple-form validation, abuse prevention and truthful delivery evidence remain required; do not invent delivery guarantees, secrets or providers.

**No implementation authorization arises from accepting the preceding contracts.** They are documentation requirements for later separate design/implementation orders.

## 2. Frozen scope and file permissions

All permanent file modifications must be inside the **single verified Solo root**, confined to the following existing paths and ordinary `.git/**` metadata. There is exactly **one** allowed report file, also listed here.

| Path | Authorized change |
| --- | --- |
| `.documents/Architecture/ARCHITECTURE.md` | Correct stale parent-ignore/bootstrap note and Reports exception; make strict hexagonal architecture a non-negotiable design contract with inward dependencies and verification; explicitly include `mailto:` + simple contact form and exclude advanced contact forms; reflect contact-specific open implementation details without choosing technology. Preserve all approved EFS, locale, URL, unlisted and graphic behavior. |
| `.documents/About/FOUNDATION-SOLO.md` | Reconcile approved Page/representation visibility names and precedence; describe strict hexagonal architecture as product quality; list the `mailto:` and simple-form module as included targets, advanced forms explicitly excluded; separate approved requirements from still-open mechanics. |
| `.documents/SOP/sop-architect.md` | Remove or generalize project-*pause* wording (not authority gates); require architectural review and work-order acceptance to enforce strict hexagonal boundaries and the contact inclusion/exclusion; retain §6B as the sole VCS procedure and §7A's GitHub-first report procedure. No new status system. |
| `.documents/SOP/sop-implementation.md` | Remove current-project pause instructions if present; require enforcement and evidence of hexagonal boundaries for future code tasks and no advanced form implementation; preserve §5B/§5C, Owner authority, reporting, clean-tree and confinement. |
| `README.md` | Replace transient `PAUSED`/reopening language with neutral, accurate `no current issued work order` and project status. Keep the complete two-file agent-entry sequence and report link; mention hexagonal architecture and the contact scope as contracts, not implemented features. |
| `AGENTS.md` | Remove current-project pause/reopening wording, preserve handoff readiness, distinguish `NONE` from permission, and point to canonical architecture for strict hexagonal and contact-module rules. Preserve the one-root, reporting and issuance rules. |
| `.documents/WIP/plan-next.md` | Remove `NONE — PAUSED` as a conversation-state marker. Restore **one** concise, non-executable next architectural candidate (IDs, minimal authoring, URL grammar, typed cascade, module boundaries as appropriate), or `NONE` if genuinely no candidate; no executable task or new work-order ID. Do not use this file to authorize Phase 1. |
| `.documents/WIP/plan-outlook.md` | Prune pause/opening-gate and completed bootstrap language; retain short/medium/long outcomes and documentation outlook. Place strict hexagonal design/testing and the planned `mailto:` plus simple-contact-form module at sensible milestones, with advanced form explicitly excluded. Retain unrelated accepted future EFS outcomes. |
| `.documents/WIP/workorder-active.md` | **Owner installs this exact issued text.** Preserve its frozen contents until the scope and evidence have been published in the first content commit. On verified successful delivery, replace it with a neutral `NONE — no current Owner-issued work order` text; no `PAUSED`, 'waiting for Architect acceptance' or artificial project-state label. On failure keep the issued order and report the STOP, do not falsely clear it. |
| `.documents/Reports/execution-report.md` | **Overwrite the existing single file** with a factual `SOLO-WO-004` report for this run, publish it, and verify read-back. Do not create another Reports file. Report status is about the **run** (`COMPLETE`, `STOPPED`, `FAILED`) rather than treating the project as paused. |
| `.git/**` | Ordinary safe staging, two bounded commits, fast-forward pushes, upstream/ref checks and remote read-back as detailed below. |

**Explicit exclusions:** No changes to `.github/**`, `LICENSE`, `.gitignore`, `.gitattributes`, `.editorconfig`, or any other path; no new source files, tests, package manifests, dependencies, frameworks, CI, releases, branches, PRs, deployments or provider/account settings; no edits to `provelopment-root`, `provelopment-foundation`, `provelopment-web-1`, or other repositories; no extra report, WIP file, directory tree, clone or worktree. Do not normalize or reformat unrelated content.

## 3. Mandatory architectural contract to document

### 3.1 Strict hexagonal architecture — mandatory, not aspirational

Record this as an **Owner-decided binding rule** in Architecture §5, with reinforcing links from Product/SOP/entry points:

- **The domain core is independent** of delivery frameworks, rendering libraries, HTTP, persistence, filesystem, email and external service implementations. It owns invariants such as Entity/Facet/Site/Page ownership, localization, visibility, properties, safe route claims and domain validation.
- **Application use cases** orchestrate domain behavior behind explicit inbound/outbound **ports** (interfaces/contracts at appropriate domain/application boundaries). An inbound adapter invokes an application port; an outbound adapter implements a port the application uses. External types and runtime/provider details are translated at the adapter boundary.
- **Dependencies point inward.** Core and use-case policies cannot import concrete infrastructure, web-framework code, database models, HTTP transport, provider SDKs or delivery adapters. Adapters depend on the inner contracts, not vice versa. A composition root wires concrete adapters without contaminating the core.
- Delivery technologies (HTTP, UI, static rendering, filesystem/content loading, external integrations and email transport) are **adapters**; changing one must not require rewriting EFS domain decisions.
- Architecture verification is required in future code-bearing work: explicit import/dependency boundary checks where practicable and tests through ports with fake adapters, plus focused actual-adapter integration evidence when relevant. No claims of passing architecture tests while this repository has no code/tests.
- These rules do **not** decide the directory layout, framework, language, persistence model, renderer, email provider or hosting platform. Avoid a ceremonial abstraction layer with no boundary purpose.

### 3.2 Contact capability — included/excluded boundary

Record as an **Owner-approved future module requirement**, not a completed implementation:

- **Included: `mailto:` handling.** Author-safe, validated email addresses and URI construction/encoding for outbound email actions; the visitor's mail client handles the actual sending. Do not misrepresent opening a mail client as verified message delivery.
- **Included: simple contact form.** A straightforward visitor inquiry form, with a bounded set of fields to be determined in a later contract, validation, safe server-side handling, proportionate anti-abuse controls and honest delivery/receipt status. Use an application use case and an outbound delivery port with replaceable adapters; never embed provider SDK/secrets or mail-transport details in EFS domain objects or authored Page sections.
- **Explicitly excluded: advanced contact forms.** No built-in or optional advanced form-builder, arbitrary conditional/multistep workflows, complex form administration, attachments, CRM/ticketing pipelines or other advanced-form subsystem. Essential validation, spam/rate controls, accessibility and security for the **simple** form are not 'advanced form' features.
- Exact contact form fields, delivery provider/adapter, spam controls and data-retention configuration remain **technical design decisions** subject to a later Owner-approved contract/work order. Do not implement or assume a provider in this task.

### 3.3 Preserve settled behavior while reconciling documents

Retain untouched in meaning: exactly one Entity; the Entity → Facet → Site → Page → representation hierarchy; five-level authored location/graphic cascade with suppression and a global public generic graphic library; configurable HTTP(S) base origin/prefix independent of identity; full authored region-qualified locale tags and explicit browser preference mapping; same-Page public missing-translation fallback; public lowercase/case-insensitive URLs versus unlisted case-exact Page tokens; fail-closed collision handling; Page `visibilityDefault` and representation `visibility` precedence; unlisted discovery safety; Apache-2.0; and the existing no-other-repository/no-second-worktree/no-temp-clutter rules.

## 4. Mandatory preflight — STOP on any failed gate

1. Read root `README.md` and `AGENTS.md`, Product Definition and Architecture, both SOPs, all three WIP files and the current canonical execution report. Verify actual Solo physical root and `git rev-parse --show-toplevel` are the same one local Solo directory, not parent or symlink/junction.
2. Confirm the **Owner actually installed this exact `SOLO-WO-004 R00`** as the sole `.documents/WIP/workorder-active.md` and sent the matching Owner handoff. Compare installed contents to the provided candidate and verify no extra terms inserted; the Owner-installed file is the one approved authority. `plan-*.md` files do not authorize execution.
3. Confirm branch `main`; local `HEAD` equals full `7ad0a7346ea29a4040c272f357fa8147682109c0`; the existing remote is **exactly** `origin = https://github.com/provelopment/provelopment-foundation-solo.git`; remote `main` matches local starting HEAD; no other remote, linked/secondary worktree, extra branch, or unrecognized files. Confirm current canonical report is tracked and WIP has exactly three filenames. If current branch or remote advanced unexpectedly, **STOP** instead of rebasing/resetting by guesswork.
4. Inspect `git status --porcelain=v1`. The **only** permitted pre-run working-tree modification is the Owner's replacement of `.documents/WIP/workorder-active.md`. No staged, other modified, untracked, unknown or unsafe contents are permitted. Do not delete or override Owner work to make the gate pass.
5. Verify the parent repository still ignores the Solo subtree, that the ignore rule is committed and effective, and that the parent tracks no Solo paths. Parent/sibling repositories are **read-only** to this Solo agent. Do not run any command that commits/pushes parent changes.
6. Verify available authenticated GitHub push/read access to this exact public Solo repository without disclosing credentials; understand whether any hook, installer, shell, CLI, cache or credential helper would write outside Solo. Use invocation-local Git/gh configuration; scratch only in Solo `.temp/` if actually required, and clean it. Do not create a second checkout or global tool config.
7. Inspect existing architecture/product/SOP/WIP content, verify the contradictions named in §1, and ensure the proposed limited changes have no hidden framework/schema choice, private data or licensing problem. If a substantive new architecture decision is needed beyond §3, stop and report instead of inventing it.

## 5. Authorized execution sequence

**Phase A — Canonical reconciliation, review, first commit and push**

1. Apply only the exact content changes in §2–§3, with concise edits at the existing authoritative sections. This includes making `README.md` and `AGENTS.md` durable entry points for a replacement agent, removing temporary project pause language wherever it is live in the authorized files, and synchronizing Product Definition, Architecture, SOPs and WIP planning. Do **not** alter the frozen Owner-installed `workorder-active.md` during this phase.
2. Check Markdown links and heading references, the resolved visibility schema declarations, no remaining **current project-state** `PAUSED` instructions in the edited live documents, the strict hexagonal requirement, and positive/exclusion coverage for the two contact channels. Ensure the single report rule and exactly three WIP rule remain mutually consistent. Prefer targeted validation and record actual results.
3. Inspect the full intended diff and stage **only** these nine explicitly authorized paths: `.documents/Architecture/ARCHITECTURE.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/SOP/sop-architect.md`, `.documents/SOP/sop-implementation.md`, `README.md`, `AGENTS.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, and the Owner-installed `.documents/WIP/workorder-active.md`. Create **one ordinary documentation commit** on `main`, preserving the prior history, and fast-forward push to `origin/main`. Independently read back the remote exact commit, changed-file list and **frozen Owner-installed work-order blob**. If scope or remote verification fails, stop and report safely without rewriting history.

**Phase B — Final report, neutral ACTIVE, second commit and push**

4. After Phase A succeeds and is verified, overwrite **only** `.documents/Reports/execution-report.md` with the factual execution report. Include starting HEAD, content commit SHA, task ID, date, changed files, check results including not-run, acceptance coverage, deviations, Git tree/hygiene evidence, publication/read-back status and any limitations. Do not write confidential data into this **public** file. Do not embed the report's **own** commit SHA in itself; that hash is reported after publication in the agent's chat handoff.
5. Replace `.documents/WIP/workorder-active.md` with a concise **neutral** `NONE — no Owner-issued work order currently active` file. It must not say `PAUSED`, `waiting for Architect`, `Owner reopening`, or `Phase 0 pause`. The previously Owner-issued complete scope must remain retrievable in the first commit's Git history. The NEXT and PLAN files already updated in Phase A must remain coherent with this neutral ACTIVE state. Do not claim the Architect independently accepted the run yet.
6. Stage **only** `.documents/Reports/execution-report.md` and `.documents/WIP/workorder-active.md`; make **one** ordinary report/closure commit on `main` and fast-forward push to `origin/main`. Verify that remote `main` equals local `HEAD`, that the remote report is readable and is the only file under `.documents/Reports/`, and that the WIP folder still contains exactly its three approved files. Verify that changed documentation, `README.md`, `AGENTS.md` and both SOPs are readable on GitHub at the exact final revision.
7. Prove a clean final Solo working tree (`git status --porcelain=v1` empty), a single checkout/worktree, the exact single `origin` remote, no extra local-only branch or dangling task artifact, no task-created `.temp`/cache or extra report, and no change to parent/sibling repositories. Only delete known files created by this task once no longer needed. Do not run `git clean`, reset, force push or history rewrite to manufacture cleanliness. Report final commit hash and direct GitHub report URL in chat.

**If any phase STOPs or fails:** Preserve unknown and Owner-owned data and the actual Git state. Overwrite and publish the **single** factual `STOPPED`/`FAILED` report through the standing report-only permission **only where safe and technically possible**, without staging unrelated changes, hiding a dirty tree, clearing ACTIVE to `NONE` or exceeding authorized scope. If remote reporting is unsafe/unavailable, report the exact local/remote state and blocker in chat, and do not claim publication. Do not start another task.

## 6. Acceptance evidence and independent review

The agent's final report must allow the Architect to verify at least:

1. Matching `SOLO-WO-004 R00` Owner issuance and clean preflight anchored at `7ad0a7346ea29a4040c272f357fa8147682109c0`.
2. Exactly the authorized Phase A and Phase B paths, two fast-forward commits, intact previous Git history, matching final local/remote HEAD, and frozen issued order preserved in first commit.
3. Architecture clearly says **strict Hexagonal Architecture (Ports and Adapters)** is REQUIRED and states verifiable inward dependency rules; Product and SOPs are consistent; no implementation technology chosen.
4. Product visibility names/default/override precedence documented as approved, with unapproved details explicitly limited to technical schema/versioning/validation choices.
5. Contact `mailto:` **included**; simple contact form **included**; advanced contact forms **excluded** (no optional/hidden subsystem), with the plan retaining later implementation of these capabilities.
6. Obsolete parent-ignore-uncommitted and report-prohibition statements removed or corrected, including the single Report file and three WIP files being disjoint.
7. No live project-wide `PAUSED`, 'pending Architect acceptance' or 'reopen before continuing' project status in the **current** edited README, AGENTS, SOPs or WIP; neutral ACTIVE `NONE` and NEXT non-executable candidate are correct. The report's run status may say awaiting independent review, but that must not become project state.
8. Links and prose consistency checked; test/build/CI not claimed if absent; no app/source/dependency/third-party code or provider changes.
9. Remote report overwritten/read back at the final commit, local clean tree and hygiene with appropriate distinction between first-hand GitHub checks and locally observed filesystem facts.

**Architect review protocol:** Upon Owner message 'agent report is ready', the Architect reads the **latest** `.documents/Reports/execution-report.md` **directly from GitHub**, verifies the exact report commit and corresponding diff and returns `ACCEPT / REVISE / STOP`. The Owner alone issues any subsequent work order. The Architect and implementation agent must not start a new task merely because this one succeeds.

## 7. Explicit authorization ledger

| Operation | Authorized under `SOLO-WO-004`? |
| --- | --- |
| Edit the nine listed Owner-issued/architecture/product/SOP/README/AGENTS/WIP paths in Phase A | **YES**, precisely as scoped |
| Overwrite existing single report and neutralize ACTIVE in Phase B after verified success | **YES**, precisely as scoped |
| Make and fast-forward push **two** ordinary bounded commits to Solo `origin/main`, with remote read-back | **YES** |
| Safely publish one factual STOP/FAILED report when needed, within existing standing report-only permission | **YES** |
| Create other reports, WIP files, code, dependencies, CI or architecture/runtime choices | **NO** |
| Create a second checkout/worktree, clone, extra branch, shadow folder or surviving temporary artifact | **NO** |
| Touch parent/sibling repository, other GitHub repository, accounts, secrets or provider settings | **NO** |
| Force-push, reset/clean unknown data, amend, rebase, merge, PR, release, deployment, DNS | **NO** |
| Start another work order or implement future contact/hexagonal modules | **NO** |

**Owner issuance remains decisive:** The Owner reviews and installs this file **as `workorder-active.md`** and sends a matching handoff. Until then it is a non-executable Architect candidate. Agent report and GitHub publication are evidence, not self-approval.
