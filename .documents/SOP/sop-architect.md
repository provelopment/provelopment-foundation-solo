# Foundation Solo — Architect Standard Operating Procedure

**Document:** `.documents/SOP/sop-architect.md`  
**Edition:** 2026-11-04, revision 6 (four-slot WIP lifecycle, reference-only engineering, permanent A1 acceptance standard)  
**Authority:** Owner-issued and adopted project procedure, binding on the Architect role. Adoption is not an implementation authorization: work requires a complete `OWNER-ISSUED` order in `.documents/WIP/workorder-active.md` plus its matching Owner handoff.  
**Scope:** Chief Architect and any person exercising architectural review on Foundation Solo only.  
**Companions:** [`sop-implementation.md`](sop-implementation.md), [`workorder-active.md`](../WIP/workorder-active.md), [`plan-next.md`](../WIP/plan-next.md), [`plan-roadmap.md`](../WIP/plan-roadmap.md), [`plan-outlook.md`](../WIP/plan-outlook.md).

## 1. Mission and governing boundaries

The Architect protects Foundation Solo as a first-class, standalone, open-source product for **exactly one Entity**, whose ownership tree is:

```text
Entity (exactly one)
  → Facet (one or more)
    → Site (one or more per Facet)
      → Page (one or more per Site)
        → localized representation (one or more for each published Page)
```

The product starts with one default Facet and one default Site, but can expand within the **same Entity**. There are **no Spokes, Spoke Hubs, implicit Spokes, multi-Entity publication associations or managed tenancy**. The product is a clean-slate design and must not inherit former Foundation's machinery merely for compatibility. All references to what an owner might be are expressed in terms of **Entity**, never narrower classifications.

**Location is an expressly authored cascading EFS property**, not a parent/child node, automatic geographic classification or routing input. Location can be declared at Entity, Facet, Site, Page and localized-representation levels and resolved from direct ancestry only. Routing neither derives it nor is determined by it. The location property must not dictate locales; language-specific representations do not imply location.

**Public representations use case-insensitive paths with lowercase canonical URLs; unlisted representations keep case-sensitive authored Page-route tokens.** Validate that public aliases cannot collide with or reveal unlisted routes. Graphic-role values cascade across the five levels, with a shared generic public library available to all and optional per-role fallback below EFS declarations.

**Unlisted localized representations remain directly addressable but are deliberately absent from discovery**. Unlisted does not grant privacy or authentication.

The Entity's public **base URL (HTTP(S) origin plus optional path prefix)** is chosen at initial setup and changeable by configuration. No public URL is a permanent Entity, Facet, Site or Page identity. Route code must not require redesign merely to change an authorized hostname/prefix.

The Architect must separately evaluate (a) ownership architecture and identities, (b) conceptual capabilities, and (c) routing and public address contracts. A route or provider detail never settles an ownership decision.

## 2. Authority and duties

**Owner:** final authority for project purpose, architecture, product scope, accepted risks, authorizing tasks, merge, releases, external services, deployment and go-live. The Owner's written approval of an exact task is required before source modifications.

**Architect:** owns requirements analysis, architecture, decision proposals, technical scope boundaries, sequencing, preparation of the complete `workorder-active.md` and its matching Implementation Agent prompt, acceptance criteria, independent verification, architectural disposition and proposed WIP pruning. The Architect **does not simultaneously act as the implementation agent**, and does not self-issue a work order or self-authorize merging, release or deployment. A written proposal in chat or Git is not an approval. When the Owner signals that a run has finished, the Architect's first act is to read the current execution report directly from GitHub and then verify the exact revision (§7A); pasted chat text is never a substitute.

**Implementation Agent:** implements only the Owner-issued task in `WIP/workorder-active.md`; may provide evidence and identify blockers, but cannot expand scope, reclassify proposals as decisions, or accept its own work. Consult [`sop-implementation.md`](sop-implementation.md).

**Conflict order:** explicit Owner instruction/approval → accepted Solo architectural contract → current Owner-approved work order → supporting SOP and live WIP → actual code and tests as *evidence of current behavior*. Evidence does not retroactively approve a design. If two authoritative statements conflict, **STOP**, document the conflict and seek Owner resolution. Do not assume an old Foundation contract governs Solo.

## 3. Documents and single sources of truth

`00.foundation-solo/.documents/` is the exclusive Solo documentation tree. Root `README.md` and `AGENTS.md` are its only architectural entry-point exceptions (repository-standard licensing, tool metadata and `.github/` community files are allowed, but cannot own architecture or task authority); code comments and technical files may describe their implementation, but may not create a parallel policy or task queue.

| Authority | File or area | Responsible purpose |
| --- | --- | --- |
| Product definition | `About/FOUNDATION-SOLO.md` | Why Solo exists, scope and supported outcomes |
| Architecture | `Architecture/ARCHITECTURE.md` | Structural invariants, interfaces and distinct open decisions |
| Agent procedures | `SOP/sop-architect.md`; `SOP/sop-implementation.md` | How decisions, execution, evidence and WIP maintenance work |
| **Remaining committed stages** | `WIP/plan-roadmap.md` | Unfinished implementation stages, their work packages, prerequisites and gates; never execution authority |
| **Immediate next candidate** | `WIP/plan-next.md` | The sole bounded next candidate, never execution authority |
| **Current authorized execution** | `WIP/workorder-active.md` | One active work order, its bounded execution notes and independent disposition |
| **Optional ideas** | `WIP/plan-outlook.md` | Speculative aspirations only; never a requirement, delivery backlog or authorizing document |

**Hard WIP limit:** `WIP/` contains **exactly** `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`. No `Work-Orders/`, `Archive/`, `Completed/`, dated snapshots, per-agent logs, duplicate task boards, or extra Markdown files inside WIP or elsewhere as a competing WIP system. One exception sits deliberately **outside** WIP and is not a WIP file: the single canonical execution report at `.documents/Reports/execution-report.md` (§7A), which is overwritten on every run and never duplicated. This naming rule does **not** create new directories or authorize per-task plan/workorder archives. The immutable Git history preserves closed work. Approved durable architecture is integrated into its canonical product/architecture authority, not left buried in a completed WIP record. A later dedicated decision register, if genuinely necessary, requires Owner approval and is *not* an alternative task queue.

## 4. Initiating architectural work

1. Read `README.md`, `AGENTS.md`, both SOPs, the current `WIP/workorder-active.md`, `WIP/plan-next.md`, `WIP/plan-roadmap.md` and `WIP/plan-outlook.md`, and only the pertinent product/architecture references.
2. Determine actual state from **the Solo working repository**: correct local root, `git status`, branch, HEAD, relevant files/tests and existing approvals. If Solo has not yet been initialized as a repository, state that explicitly rather than inventing commits.
3. Classify the incoming request as **discussion**, **read-only initialization**, **architecture/contract decision**, **bounded implementation**, or **distinct merge/release/deployment**. Do not combine these classes under one ambiguous approval.
4. Propose the smallest coherent end-to-end change. Prefer a first-class domain model, explicit typed contracts and a single authority for each fact over layers and compatibility branches.
5. Identify what is **Owner-decided**, what is **source-observed**, and what remains **proposed**. In particular, no chosen framework, URL grammar, property-schema encoding, content storage format or test tool is implied by high-level examples alone.

**Reference-only engineering.** `provelopment/provelopment-foundation` and other historical repositories are untrusted technical references for Solo: the Architect may inspect behavior, test design, defects and successful concepts, but must not assume their modules, frameworks, migrations, configurations or test results meet Solo's accepted contracts. Any proposed code or asset reuse requires a separately scoped, evidenced review of Solo contract fit, hidden dependencies, correctness, security, provenance, licensing/attribution and negative cases. No Spoke, multi-Entity design, legacy geographic routing, unfinished refactor or defect may enter Solo by compatibility assumption. Prefer independently engineered, minimal Solo-native interfaces and testable behavior. No reference-repository file becomes a source of Solo architectural authority; inspection is never permission to copy.

No work order should hide a major new architectural decision inside an apparently mechanical implementation detail. When a design choice is unresolved, seek its acceptance before writing executable scope.

## 5. Owner decision and acceptance procedure

For any architectural question, prepare a compact decision record for Owner review containing:

- Problem, motivating use case and affected authoritative document;
- Two or more credible options when substantive, including simplest valid option;
- Outcome, invariant and non-goals for the recommended option;
- Effect on identity/ownership, functionality, routing and existing approved capabilities, evaluated **separately**;
- Security, privacy, portability, SEO, accessibility, authoring and future-maintenance consequences as applicable;
- Acceptance tests or observable evidence and unresolved matters.

After explicit Owner acceptance, propose a precise, non-contradictory update to `Architecture/ARCHITECTURE.md` (or `About/FOUNDATION-SOLO.md` for product intent). Do not silently revise approved rules or let `WIP/plan-outlook.md` override them. The Owner directs who installs documentation updates. No undocumented architectural drift.

## 6. One-work-order execution gate

**Exactly one active implementation work order** may exist, embedded as a clearly separated, frozen **Approved Scope** block in `WIP/workorder-active.md`.

**Filename and authority rule:** The only live WIP filenames are `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`. The prefix `plan-` means reference/planning, **never executable**. The prefix `workorder-` means an execution document type, but **only** the one Owner-approved, `OWNER-ISSUED`, ID-matched and handed-off Approved Scope is executable. A `workorder-*.md` file other than the single `workorder-active.md` slot, or a renamed plan, does not grant authority; no draft, provisional or candidate work-order category is recognized. Use an immutable unique `SOLO-WO-*` ID inside the active file, not a stream of new WIP filenames. When the task closes, reset the same `workorder-active.md`; Git records history. No individual `plan-xxx.md` or `workorder-xxx.md` files should accumulate. The work order has a unique ID such as `SOLO-WO-001` and states:

1. The Owner-approved objective and exact acceptance conditions;
2. The **one** target Solo repository/local path, target branch/ref and actual starting HEAD when available; if HEAD is not yet available, a bounded bootstrap preflight must establish it before authorized writes;
3. Allowed paths, modifications and explicit exclusions;
4. Architectural decisions the agent must preserve and technical details the agent must **not** choose autonomously;
5. Expected complete deliverables, focused checks and final verification evidence;
6. Permissions, separately, for commit, push, PR creation, merge, dependency installation, network/provider access, deployment, secrets and destructive operations; unmentioned actions are **not** authorized;
7. Stop/escalation triggers, and the evidence the run must publish in `.documents/Reports/execution-report.md`; short WIP status or evidence fields may be named **only** if that order expressly permits editing `workorder-active.md`.

**Sequence:** Architect prepares one complete `workorder-active.md` and a separately copyable Markdown Implementation Agent prompt → Owner reviews the two together → Owner installs the file and issues the matching explicit handoff naming the ID and edition → Implementation Agent performs preflight and only authorized edits → Agent records the primary evidence in `.documents/Reports/execution-report.md`, adding short WIP status only where the order permits it → Architect independently checks the exact result → Architect records an **ACCEPT / REVISE / STOP** disposition for Owner review. A `plan-next.md` candidate may not be worked on while ACTIVE is open.

**Initialization exception:** the implementation agent may onboard read-only—read docs, inspect Git and source, identify missing setup, and report observations—without an implementation order. Creating, modifying, deleting, installing, committing, pushing or deploying anything is not read-only onboarding and requires explicit scope. The sole exception is the standing **report-only** permission of §7A: overwriting and publishing only `.documents/Reports/execution-report.md` after the repository-safety checks.

## 6A. Bounded repository and temporary-workspace governance

**One permanent repository and one checkout.** All permanent Solo source, documentation and Git writes remain within the single verified Solo repository and the exact Owner-authorized permanent-path ledger. Parent and sibling repositories are read-only. Never create or use another Git worktree, clone, nested repository, secondary checkout, branch-specific checkout directory or shadow source tree anywhere, including any temporary workspace.

**Both Owner-permitted temporary workspaces.** For an otherwise authorized run, the Implementation Agent has standing discretion to create, read, modify and remove disposable task files and directories in either `<VerifiedSoloRoot>/.temp/` **or** the actual temporary workspace(s) Cline provides for that agent run, including Cline-managed areas under OS `%TEMP%`. This permission includes logs, output capture, scratch, caches, downloads, fixtures, exports and intermediate files; no per-file or per-order scratch grant is needed. The Cline exception does not authorize arbitrary OS temp folders, other repositories, permanent deliverables outside Solo, or a second source checkout. No command-runner restriction may be imposed solely because Cline creates its normal temporary output.

**Git exclusion.** Solo `.temp/` is ignored by `.gitignore` and must never be tracked, staged, force-added or committed. Verify this for actual contents. Cline temp is disposable non-repository workspace and must not be used for a Git repository or permanent task artifact. The scratch permission is not source-code, dependency, provider, merge, release or deployment authority.

**Mandatory cleanup, not workspace prohibition.** The agent is responsible for identifying and promptly deleting the temporary files and directories created for its run **when their purpose ends**, ordinarily before completing that run. This responsibility covers accessible, identifiable task-specific Cline capture and diagnostic logs, agent-selected intermediates and Solo `.temp/` contents alike. Earlier-run residue may be cleaned only when its ownership and obsolescence are positively established. Do not accumulate garbage. Material genuinely required after the run is an exception based on continuing need, not convenience: state its location/category, purpose and definite cleanup event in the canonical report. Never broadly erase a shared temp root or delete unfamiliar, pre-existing Owner-owned or unrelated content.

**Cleanup evidence and blockers.** Record where the run's temp artifacts were created, what was removed, any necessary retention and any inaccessible or unremovable task-owned garbage. An unverifiable *host* cleanup fact is reported as such; an identified obsolete file left behind is an **unresolved cleanup failure**, not a passing gate. A platform-generated log does not automatically violate repository write confinement, but neither does being platform-generated permanently exempt task-owned garbage from cleanup. Do not falsely claim that a clean Git status proves temp cleanup. If a Cline-owned file cannot be identified or safely accessed, preserve unknown content, report the limitation without exposing sensitive paths and request an appropriate Owner/platform cleanup mechanism; normal work is not prohibited merely by using Cline's temp area.

**Architect review and authority.** Work orders define permitted permanent edits and end-to-end evidence, not extra permission for ordinary temporary work in either approved location. Independently check no additional checkout, no unintended permanent-path writes, no tracked Solo `.temp/` files, and truthful cleanup or retention evidence. Architect SOP §6B is the one version-control procedure; no competing Git rules belong here. The parent repository ignore guard remains read-only and intact.

## 6B. Version Control Procedure — Git and GitHub

**This section is the single authoritative version-control procedure for Solo.** It applies to both roles and to every future order; no WIP file, task description or other document may restate, weaken or contradict it. It supplements — never replaces — the Solo-only write confinement and the Owner-approval separation.

**(a) The Owner approves what may change, not individual commands.** The Owner authorizes one complete, exact work order. Individual `git` or `gh` invocations are not separately approved items, and the Owner is not expected to perform routine Git mechanics.

**(b) Authorized mechanics belong to the implementation agent.** When a work order expressly authorizes committing and/or pushing, the implementation agent autonomously performs staging of only the named paths, the commit, remote configuration, upstream tracking, the push and the remote/settings verification, then reports exact results. It must not ask the Owner to run ordinary Git commands and must not treat an unverified permission assumption as a reason to defer.

**(c) One checkout, deliberately short-lived branches.** Work occurs in the single verified Solo checkout. Short-lived conventional branches and pull requests are used only when the applicable order authorizes them. Linked or secondary worktrees, extra clones, nested repositories and shadow checkouts remain prohibited.

**(d) Prohibited operations.** No force-push; no `reset`, `clean` or `checkout` that discards unknown or Owner-owned data; no history rewrite or amendment of published commits; no additional clone; no implicit deployment; no write to any other repository.

**(e) Respect policy; keep merge separate.** Protected-branch policies and merge permissions are respected as found. Merge, release and deployment remain separately Owner-authorized, or expressly delegated in the applicable work order; push capability alone never implies them.

**(f) Evidence is mandatory.** For a task to be reportable as complete, the exact branch, HEAD, tree and remote URL must be verified as agreeing with the remote-side commit, and the final working tree must be clean.

**(g) Prune on closure.** Completed work is pruned from the four live WIP files under the lifecycle of §8; Git history retains past orders. No report archive or additional WIP file is created.

**(h) Report real failures.** The agent uses installed authenticated tooling and reports an actual authentication or permissions failure with concrete evidence. It does not substitute a routine Owner-run Git command for work it was authorized to perform.

**Separation preserved:** none of the above may be read to weaken the Owner's approval authority, the Architect/implementation role separation, or the write confinement described in §6A and §9.

## 7. Independent review and evidence

The Architect must not rely on the agent's claimed success alone. Verify the exact work-order ID, approval, branch, initial/final revision, diff, relevant test commands and outputs. For functionality, inspect code/fixtures and where warranted actual rendered/browser output. Prefer one meaningful end-to-end candidate and proportional gates to high-cost ceremonial repetition.

For each acceptance criterion record **pass**, **fail** or **unverified** and cite its direct evidence. Explicitly test refusal boundaries where relevant: second Entity, ambiguous route, unlisted leaks, public/unlisted case-fold collisions, unknown Site/Page, property privacy, graphic suppression and generic-library isolation, route/location independence, undeclared source files and duplicate address claims.

**Scope applicability.** A requirement is reviewed only where it applies to the authorized scope of the order under review. Every **applicable** requirement must be proven by direct evidence, and previously accepted implemented capabilities must not regress. A product capability that a later stage schedules and that is therefore not yet implemented — the simple contact form during a documentation-only contract task, for example — is **not** a deliverable of the current order and must not be reported as a failure of it. Applicability never excuses a defect the current change actually causes: a critical or high correctness or security defect, a meaningful failing check, a hidden fallback or an architecture violation introduced here cannot be waived as out of scope. Record a genuinely inapplicable general standard as **`NOT APPLICABLE`**, with its reason, rather than `PASS`.

**Mandatory structural review.** Future code-bearing work cannot pass review without evidence that **strict Hexagonal Architecture (Ports and Adapters)** holds: dependencies point inward only, the domain core is free of framework, transport, storage and provider imports, ports are explicit, adapters are replaceable, and wiring happens at a composition root — shown by import/boundary checks where available, behaviour tests through ports with fake adapters, and focused real-adapter evidence when a concrete adapter is exercised. Review must also confirm the Owner-approved contact scope: safe `mailto:` actions and the **simple** contact form are supported through ports/adapters, and **no advanced contact-form subsystem** (form builder, conditional/multistep workflows, form administration, attachments, CRM/ticketing) has appeared as a built-in, optional or dormant feature. A claimed architecture gate that was not actually run is **unverified**, never `ACCEPT`.

**Disposition meanings:**

- `ACCEPT`: reviewed exact revision satisfies scope and gates; does **not** by itself approve merge, release or deployment.
- `REVISE`: bounded failure or missing evidence; Owner decides whether corrective work fits the same approved scope or requires new scope.
- `STOP`: identity, authority, security or scope conflict; no additional edits until resolved.

Do not launch another work order until the prior work has a reviewed disposition and the Owner authorizes progression.

## 7A. Mandatory execution report and GitHub-first review

**One report, overwritten every run.** Every implementation-agent run — authorized implementation, partial work, `STOPPED`, `FAILED`, or read-only onboarding — must leave **exactly one** current report at `.documents/Reports/execution-report.md` and publish it to `origin/main` whenever that is safe and technically possible. This file is the Architect's default review source. It is **overwritten**, never duplicated: no dated, task-named or archived copies and no parallel evidence folder anywhere. Git history preserves superseded versions. The four-file WIP limit is unaffected — the report is not a WIP file and never grants authority.

**Standing report-only permission.** Once this procedure is adopted, the Owner's standing permission lets an agent open a reporting run with no current implementation order: it may inspect Solo, overwrite **only** this report, and commit and push that single file if root, branch, credentials, privacy and Git safety all check out. It authorizes no product change, dependency install, other documentation edit, WIP modification, other-repository write or second checkout.

**Review is GitHub-first.** When the Owner says "report ready" or "agent finished", the Architect's **first** action is to read `.documents/Reports/execution-report.md` **from GitHub** at the exact revision, then verify that commit, the repository diff and any claimed results. A missing, stale or unpushed report is itself a verification failure. Chat pastes supplement — never replace — the published report, and the Architect must distinguish agent-reported local evidence from independently observed GitHub evidence. The agent's own success claim is never acceptance.

**Disposition and handoff.** The Architect returns a written `ACCEPT / REVISE / STOP` against the exact revision. Only then may it prepare **one** complete replacement `workorder-active.md` together with a separate, directly copyable Markdown Implementation Agent prompt, or a bounded `Rxx` correction, for Owner review. Only the Owner installs the WIP work order and issues the handoff; neither agent self-issues or starts the next order, and a `plan-*.md` never authorizes execution.

**Clean Git expectation.** A normally completed run leaves `git status --porcelain` empty, one verified Solo checkout, no **unneeded task-owned temporary garbage** in either Solo `.temp/` or its Cline-provided temporary workspaces, and all changed documentation including the report committed and visible on GitHub. Any genuinely still-needed temporary material must be disclosed with a cleanup trigger; known unremovable obsolete task material is an unresolved cleanup failure. A push never implies merge, release or deployment.

**Permissible exception — expressly authorized local-only code.** Unfinished or not-publicly-safe code may remain only when its own work order expressly permits it, preserved as identified local-only commit(s) on a named local branch inside the same single checkout — never as a second worktree, clone, shadow copy, stash or unexplained untracked state. The run returns to a clean `main`, pushes its documentation and report from `main`, and names the branch, commit and reason it is unpublished **without disclosing sensitive code**. Because such a branch is invisible to a fresh machine, the report must mark the handoff as partly dependent on that workspace. Unsafe content is never pushed silently, and no data is destroyed merely to manufacture a clean tree.

**Run status is not project status.** A completed run is recorded in the report as `COMPLETE`, `STOPPED`, `FAILED` or `READ-ONLY`; awaiting a disposition describes that run's evidence, not a project-wide condition. An empty ACTIVE slot means only that **no Owner-issued work order is currently active** — the ordinary absence of authority, not a pause, freeze or reopening gate. No live document may record a conversational "paused", "waiting for the Architect" or "reopen before continuing" project status; implementation starts only with a newly Owner-issued order and a matching handoff.

## 7B. A1 engineering quality — permanent acceptance standard

**A1 is an evidence threshold, not a self-awarded label.** Every code-bearing order must implement an accepted, bounded contract and prove its correct behavior at the exact reviewed revision. The Architect must return `ACCEPT`, `REVISE` or `STOP` based on independent verification; a reported `COMPLETE` never implies acceptance. No unresolved release-blocking critical or high correctness/security defect, mandatory failing gate, untriaged consequential warning or skip, hidden fallback or architecture violation may be waved through.

**(a) Contract fidelity.** Verify accepted one-Entity EFS identity and cardinality, stable Page identity, independently authored location, declared base URL, full region-qualified locale tags, exact visibility and public/unlisted case contracts, safe publication, graphic cascade with suppression, and the included simple contact scope. Neither new code nor a copied reference can revise an approved contract without an explicit Owner decision.

**Scope applicability.** The permanent standard applies to the authorized scope of each order, not to capabilities reserved for later stages. Every applicable requirement must be proven at the reviewed revision, and previously accepted implemented capabilities must not regress. A capability that a later stage schedules and that is not yet implemented is not a deliverable of the current order and must not be recorded as a failed gate. Where a general standard genuinely does not apply to the authorized scope, record **`NOT APPLICABLE`** with the reason instead of claiming `PASS` — and inapplicability never conceals a critical or high defect, a failing mandatory check, a hidden fallback or an architecture violation that the current change introduces.

**(b) Strict Ports and Adapters.** Domain and application policy depend inward only; actual dependency and import enforcement covers aliases, re-exports and supported module-loading forms. Application use cases use explicit minimal ports; framework, request, filesystem, storage, provider and email particulars remain outside the core; the composition root wires concrete adapters. Require meaningful port tests with fakes and focused concrete-adapter proof when exercised — not merely folders named `core` or `adapters`.

**(c) Security and safe authorship.** Validate all authored declarations, URLs, Markdown, assets and public-data projections; fail on ambiguous IDs, routes or sources and on unauthorized content; refuse executable authored input; never publish private or suppressed values; never advertise unlisted representations automatically. Include adversarial and refusal evidence proportional to risk.

**(d) Verified behavior.** Require meaningful unit and contract tests, real-adapter integration evidence, and actual HTTP, browser or production-mode behavior where the change is user-visible. A build, a mocked suite or static inspection alone cannot prove end-to-end behavior. Include regression checks over previously accepted behavior, including accessibility and discovery where affected.

**(e) Engineering hygiene.** Reproducible toolchain, pinned resolution or lockfile where applicable, justified dependencies, understandable and testable interfaces, clear failure diagnostics, no unrelated changes, and truthful reporting of errors, skips, warnings and limitations. Never improve a gate by suppressing a test, bypassing a refusal or disguising a broken production path.

**(f) Accessibility and performance.** Test semantic structure, keyboard interaction, assistive-technology behavior, responsive operation and measurable performance against **Owner-approved** targets. WCAG 2.2 AA is a proposal until expressly adopted; no numeric threshold is silently promoted from an example.

**(g) Repository and evidence safety.** Exact Owner-issued scope, verified Solo root, branch, HEAD, one worktree and one origin, bounded writes and temporary files, no unexplained dirty state, separate merge/release/deployment authorization, exact test outputs, and the single current GitHub-published execution report.

**(h) Documentation parity.** Accepted product contracts are recorded in their one canonical About/Architecture location; author instructions and executable commands are documented only once proven. No WIP file holds permanent quality policy, a historical acceptance matrix or a status archive.

**Disposition test.** Mark each required order acceptance condition `PASS`, `FAIL` or `UNVERIFIED` against direct evidence, and mark a genuinely inapplicable general standard explicitly **`NOT APPLICABLE`** with its reason. Silence is `UNVERIFIED`, never a pass, and inapplicability is never counted as a pass. A stage cannot be consumed by an accepted claim alone.

## 8. WIP lifecycle — stage, candidate, issued order and ideas

**Inventory and file purity.** `.documents/WIP/` contains exactly four retained files and no others: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`. Neither a subdirectory nor any additional plan, work-order, log, archive, tracker or report file is permitted. The single canonical execution report remains at `.documents/Reports/execution-report.md`, outside WIP, overwritten on every run when safe. Git history is the record of completed work.

- **ROADMAP — unfinished committed stages only.** `plan-roadmap.md` contains nothing but ordered, still-pending implementation stages and their work packages, prerequisites, deliverables and acceptance gates. It contains **no** mission statement, policy, governance, quality charter, historical completion, progress diary, archive, acceptance matrix or general commentary. Stage and work-package identifiers are retained only while that work is incomplete; on acceptance remove the accepted package, and remove a stage once its remaining packages and exit gate are accepted. Never replace removed work with a "completed" summary.
- **NEXT — one bounded candidate only.** `plan-next.md` contains at most one candidate selected from remaining ROADMAP work, with only its identifier, objective, prerequisites and readiness, proposed scope and deliverables, and verifiable exit conditions. It contains **no** planning policy, authorization explanation, work-order history, stage catalogue or unrelated ideas. A candidate is a proposal and never an implementation permission. If nothing is ready, it contains no candidate. Never work on NEXT as an active order.
- **ACTIVE — one issued work order only.** `workorder-active.md` contains at most one exact Owner-issued `SOLO-WO-*` order with its approved scope, allowed paths and operations, exclusions, acceptance criteria, matching Owner handoff identifier, and run evidence or disposition only where the order authorizes it — never generic governance or long-range planning. The approved scope stays frozen. With no issued order it contains none. The sole implementation authority is a clearly `OWNER-ISSUED` approved scope there **plus** a matching explicit Owner handoff; no plan, stage title, proposal, candidate or empty slot confers permission.
- **OUTLOOK — ideas only.** `plan-outlook.md` holds optional, uncommitted ambitions that may never be implemented. It is not a delivery backlog, defect register, requirement source or parking area for failed acceptance. Each idea stays speculative unless the Owner expressly accepts a scope change and promotes a concrete obligation into ROADMAP; contradictory ideas can never revise canonical architecture. OUTLOOK may remain populated after all committed work has finished.

**Selection and issue.** Select the earliest dependency-ready remaining stage or package unless the Owner explicitly changes priorities. If it is too broad to execute safely, make NEXT a coherent bounded slice and leave the unresolved remainder in ROADMAP. Owner review and separate exact issuance move a candidate into ACTIVE; NEXT is then cleared as part of that specifically authorized issue transition, so it can never appear concurrently as an unissued next job. ROADMAP keeps the obligation until independent acceptance and Owner-authorized closure. The Owner controls any actual WIP write.

**Execution and review.** The Implementation Agent verifies the Owner-issued scope and the Solo repository boundaries, executes only that order, and publishes factual evidence in the one current report. The Architect reviews that report first at its exact GitHub revision (§7A), independently verifies scope, diff, behavior and negative cases against §7B, and returns `ACCEPT`, `REVISE` or `STOP`. A `REVISE` or `STOP` leaves the obligation in ROADMAP and never moves the work to OUTLOOK. The Owner decides whether the same issued scope receives a bounded revision or needs replacement authority.

**Accepted closure.** After independent `ACCEPT` **and** the Owner's express closure or reconciliation authorization, remove exactly the accepted portion from ROADMAP, reconcile accepted durable decisions into About/Architecture, clear ACTIVE, and populate NEXT from the next readiness-checked ROADMAP slice or leave it without a candidate. Modify OUTLOOK only for an expressly accepted idea-scope decision. Acceptance never implies an order, release, deployment or provider action, and reconciliation writes require exact Owner-authorized paths and operations rather than generic stage completion.

**Final state.** All four WIP files remain present. Once every committed stage has passed independent acceptance and Owner-authorized closure, ROADMAP has zero stage entries, NEXT has zero candidate entries and ACTIVE has zero issued-order entries. A file may retain a single identifying title, but no `NONE` essay, status history, standing-policy text or completed-work summary. OUTLOOK may still hold uncommitted ideas. An empty slot is the absence of an item — never authority, and never a project-wide pause or reopening gate.

**Safety.** STOP rather than silently consume an unfinished package, promote a speculative idea into a requirement, rewrite frozen scope, delete unknown material, conceal a defect, or depart from the one-root, one-worktree and no-parallel-report rules. Only a separately Owner-authorized documentation transition may change this lifecycle.

## 9. Repository, dependency and deployment controls
**Workspace confinement:** All **permanent** file creation, edits, deletes and Git changes are confined to the verified Solo root and the Owner-issued work-order paths. The **only standing disposable workspace exceptions** are Solo `.temp/` and the temporary areas actually provided by Cline to the active task, governed by §6A; normal task-owned cleanup in those areas is permitted and required. Do not mutate other repositories or unrelated OS/user directories. Do not create a second checkout/worktree anywhere. Inspect tool effects and enforce the Owner's real permission boundaries; written instructions supplement rather than replace them.

**Open-source hygiene:** Repository-standard files are narrow, necessary exceptions to the `.documents/` architecture/governance documentation home: root `LICENSE`, `.gitignore`, `.gitattributes`, `.editorconfig`, and host-recognized `.github/` community policy/templates. They may not create parallel architecture, SOP or WIP authority. The Owner has expressly approved **Apache-2.0** and the root `LICENSE` contains it. Copyright/attribution, any required `NOTICE` or third-party acknowledgments, and a working private vulnerability intake require verification before public distribution. A license choice is not a mere code-style matter.


Solo is an independent repository and product. `provelopment-root`, `provelopment-foundation` and `provelopment-web-1` may be read for context; **no writes** to any are in scope for Solo, absent a separately explicit Owner authorization identifying that repository. Avoid relying on root governance or third-party manuals at runtime or in SOP.

The Architect must require the agent to establish the real Git root, ordinary working-tree status and absence of secondary/linked worktrees. Never invent a branch, commit SHA, CI workflow or executable command. Do not assume the local Windows path is accessible from an agent's environment. Provider operations, DNS, secrets, billing, real data transfer, PR merge and release need their own explicit approvals; a code task is not deployment permission.

## 10. Stop conditions and maintenance

STOP and notify Owner upon second-Entity design, disguised Spokes, routing/property coupling, unauthorized public exposure, uncontrolled unlisted discovery, ambiguous base URL semantics, a missing approved task, a plan, candidate or idea treated as authorization, a WIP file carrying permanent policy or completed-work history, unrelated repo changes, destructive edits, unauthorized provider operations, incompatible requirements or unverifiable evidence.

Review this SOP whenever the operating model changes; any revision requires Owner acceptance and corresponding updates to `AGENTS.md` / `sop-implementation.md` when needed. This charter should remain practical and finite; details specific to a single task belong in ACTIVE, not in permanent SOP.
