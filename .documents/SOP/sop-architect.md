# Foundation Solo — Architect Standard Operating Procedure

**Document:** `.documents/SOP/sop-architect.md`  
**Edition:** 2026-11-04, revision 5 (Owner URL-case, graphic-cascade and Solo-bootstrap decisions)  
**Authority:** Owner-issued project procedure upon acceptance and installation. Until then, a review draft; **not** an implementation authorization.  
**Scope:** Chief Architect and any person exercising architectural review on Foundation Solo only.  
**Companions:** [`sop-implementation.md`](sop-implementation.md), [`workorder-active.md`](../WIP/workorder-active.md), [`plan-next.md`](../WIP/plan-next.md), [`plan-outlook.md`](../WIP/plan-outlook.md).

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

**Architect:** owns requirements analysis, architecture, decision proposals, technical scope boundaries, sequencing, candidate work orders, acceptance criteria, independent verification, architectural disposition and proposed WIP pruning. The Architect **does not simultaneously act as the implementation agent**, and does not self-issue a work order or self-authorize merging, release or deployment. A written proposal in chat or Git is not an approval.

**Implementation Agent:** implements only the Owner-issued task in `WIP/workorder-active.md`; may provide evidence and identify blockers, but cannot expand scope, reclassify proposals as decisions, or accept its own work. Consult [`sop-implementation.md`](sop-implementation.md).

**Conflict order:** explicit Owner instruction/approval → accepted Solo architectural contract → current Owner-approved work order → supporting SOP and live WIP → actual code and tests as *evidence of current behavior*. Evidence does not retroactively approve a design. If two authoritative statements conflict, **STOP**, document the conflict and seek Owner resolution. Do not assume an old Foundation contract governs Solo.

## 3. Documents and single sources of truth

`00.foundation-solo/.documents/` is the exclusive Solo documentation tree. Root `README.md` and `AGENTS.md` are its only architectural entry-point exceptions (repository-standard licensing, tool metadata and `.github/` community files are allowed, but cannot own architecture or task authority); code comments and technical files may describe their implementation, but may not create a parallel policy or task queue.

| Authority | File or area | Responsible purpose |
| --- | --- | --- |
| Product definition | `About/FOUNDATION-SOLO.md` | Why Solo exists, scope and supported outcomes |
| Architecture | `Architecture/ARCHITECTURE.md` | Structural invariants, interfaces and distinct open decisions |
| Agent procedures | `SOP/sop-architect.md`; `SOP/sop-implementation.md` | How decisions, execution, evidence and WIP maintenance work |
| **Current authorized execution** | `WIP/workorder-active.md` | One active work order, its bounded execution notes and independent disposition |
| **Immediate next candidate** | `WIP/plan-next.md` | The sole next candidate, never execution authority |
| **Forward outlook** | `WIP/plan-outlook.md` | Pruned short-, medium- and long-term goals and documentation outlook |

**Hard WIP limit:** `WIP/` contains **exactly** `workorder-active.md`, `plan-next.md` and `plan-outlook.md`. No `Reports/`, `Work-Orders/`, `Archive/`, `Completed/`, dated snapshots, per-agent logs, duplicate task boards, or extra Markdown files inside WIP or elsewhere as a competing WIP system. This naming rule does **not** create new directories or authorize per-task plan/workorder archives. The immutable Git history preserves closed work. Approved durable architecture is integrated into its canonical product/architecture authority, not left buried in a completed WIP record. A later dedicated decision register, if genuinely necessary, requires Owner approval and is *not* an alternative task queue.

## 4. Initiating architectural work

1. Read `README.md`, `AGENTS.md`, both SOPs, the current `WIP/workorder-active.md`, `WIP/plan-next.md` and `WIP/plan-outlook.md`, and only the pertinent product/architecture references.
2. Determine actual state from **the Solo working repository**: correct local root, `git status`, branch, HEAD, relevant files/tests and existing approvals. If Solo has not yet been initialized as a repository, state that explicitly rather than inventing commits.
3. Classify the incoming request as **discussion**, **read-only initialization**, **architecture/contract decision**, **bounded implementation**, or **distinct merge/release/deployment**. Do not combine these classes under one ambiguous approval.
4. Propose the smallest coherent end-to-end change. Prefer a first-class domain model, explicit typed contracts and a single authority for each fact over layers and compatibility branches.
5. Identify what is **Owner-decided**, what is **source-observed**, and what remains **proposed**. In particular, no chosen framework, URL grammar, property-schema encoding, content storage format or test tool is implied by high-level examples alone.

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

**Filename and authority rule:** The only live WIP filenames are `workorder-active.md`, `plan-next.md` and `plan-outlook.md`. The prefix `plan-` means reference/planning, **never executable**. The prefix `workorder-` means an execution document type, but **only** the one Owner-approved, `OWNER-ISSUED`, ID-matched and handed-off Approved Scope is executable. A draft `workorder-*.md` or a renamed plan does not grant authority. Use an immutable unique `SOLO-WO-*` ID inside the active file, not a stream of new WIP filenames. When the task closes, reset the same `workorder-active.md`; Git records history. No individual `plan-xxx.md` or `workorder-xxx.md` files should accumulate. The work order has a unique ID such as `SOLO-WO-001` and states:

1. The Owner-approved objective and exact acceptance conditions;
2. The **one** target Solo repository/local path, target branch/ref and actual starting HEAD when available; if HEAD is not yet available, a bounded bootstrap preflight must establish it before authorized writes;
3. Allowed paths, modifications and explicit exclusions;
4. Architectural decisions the agent must preserve and technical details the agent must **not** choose autonomously;
5. Expected complete deliverables, focused checks and final verification evidence;
6. Permissions, separately, for commit, push, PR creation, merge, dependency installation, network/provider access, deployment, secrets and destructive operations; unmentioned actions are **not** authorized;
7. Stop/escalation triggers; evidence and reporting fields to fill in the same `workorder-active.md`.

**Sequence:** Architect drafts a single candidate → Owner reviews exact contents → Owner installs or expressly approves the exact `workorder-active.md` approved-scope block and issues an explicit handoff naming the ID/ref → Implementation Agent performs preflight and only authorized edits → Agent supplies evidence in the execution section of `workorder-active.md` → Architect independently checks the exact result → Architect records an **ACCEPT / REVISE / STOP** disposition for Owner review. A `plan-next.md` candidate may not be worked on while ACTIVE is open.

**Initialization exception:** the implementation agent may onboard read-only—read docs, inspect Git and source, identify missing setup, and report observations—without an implementation order. Creating, modifying, deleting, installing, committing, pushing or deploying anything is not read-only onboarding and requires explicit scope.

## 6A. Bounded repository and temporary-workspace governance

The implementation agent is **strictly confined to a single editable Solo root**. It is forbidden to create or use additional Git worktrees, cloned checkout directories, nested working repositories or alternate source trees, including under Solo `.temp/`. All its file writes, scratch work, downloads, caches and generated artifacts must remain inside Solo and within the task's permission ledger. Redirect temporary tool outputs into Solo `.temp/` where safe, and clean all temporary directories the agent creates as soon as they are no longer needed. No external OS/user/global-cache temp directories may be used for agent-initiated work. If a tool cannot contain its writes, do not run it absent a redesigned Owner-approved contract.

An Architect work order must specify: (a) permissible permanent output paths; (b) whether/how temporary output under `.temp/` is allowed; (c) safe tool/cache controls; (d) cleanup and final proof. A bootstrap work order that only permits Git initialization must not be construed as authorizing `.temp/`, installs or other file writes. An agent must not delete unknown or Owner-controlled files to meet a cleanliness target: STOP and report instead. Independently check the absence of linked worktrees, escaped writes and leftover agent-created temp before accepting a task.

The Owner's parent repository is **outside** the Solo agent's write boundary. Verified baseline: Solo **is** an independent Git repository on `main` whose committed root is the `SOLO-WO-001` baseline; the parent's `/00.foundation-solo/` ignore rule is **committed** and confirmed by `git check-ignore`, and the parent tracks no Solo files. `origin` is the verified public GitHub repository, so ordinary authorized pushes are expected to be performed by the implementation agent under §6B. The parent guard remains Owner/parent-operator territory and the Solo agent must not modify it. Keep the existing root commit intact rather than re-baselining published history.

## 6B. Version Control Procedure — Git and GitHub

**This section is the single authoritative version-control procedure for Solo.** It applies to both roles and to every future order; no WIP file, task description or other document may restate, weaken or contradict it. It supplements — never replaces — the Solo-only write confinement and the Owner-approval separation.

**(a) The Owner approves what may change, not individual commands.** The Owner authorizes one complete, exact work order. Individual `git` or `gh` invocations are not separately approved items, and the Owner is not expected to perform routine Git mechanics.

**(b) Authorized mechanics belong to the implementation agent.** When a work order expressly authorizes committing and/or pushing, the implementation agent autonomously performs staging of only the named paths, the commit, remote configuration, upstream tracking, the push and the remote/settings verification, then reports exact results. It must not ask the Owner to run ordinary Git commands and must not treat an unverified permission assumption as a reason to defer.

**(c) One checkout, deliberately short-lived branches.** Work occurs in the single verified Solo checkout. Short-lived conventional branches and pull requests are used only when the applicable order authorizes them. Linked or secondary worktrees, extra clones, nested repositories and shadow checkouts remain prohibited.

**(d) Prohibited operations.** No force-push; no `reset`, `clean` or `checkout` that discards unknown or Owner-owned data; no history rewrite or amendment of published commits; no additional clone; no implicit deployment; no write to any other repository.

**(e) Respect policy; keep merge separate.** Protected-branch policies and merge permissions are respected as found. Merge, release and deployment remain separately Owner-authorized, or expressly delegated in the applicable work order; push capability alone never implies them.

**(f) Evidence is mandatory.** For a task to be reportable as complete, the exact branch, HEAD, tree and remote URL must be verified as agreeing with the remote-side commit, and the final working tree must be clean.

**(g) Prune on closure.** Completed work is pruned from the three live WIP files; Git history retains past orders. No report archive or additional WIP file is created.

**(h) Report real failures.** The agent uses installed authenticated tooling and reports an actual authentication or permissions failure with concrete evidence. It does not substitute a routine Owner-run Git command for work it was authorized to perform.

**Separation preserved:** none of the above may be read to weaken the Owner's approval authority, the Architect/implementation role separation, or the write confinement described in §6A and §9.

## 7. Independent review and evidence

The Architect must not rely on the agent's claimed success alone. Verify the exact work-order ID, approval, branch, initial/final revision, diff, relevant test commands and outputs. For functionality, inspect code/fixtures and where warranted actual rendered/browser output. Prefer one meaningful end-to-end candidate and proportional gates to high-cost ceremonial repetition.

For each acceptance criterion record **pass**, **fail** or **unverified** and cite its direct evidence. Explicitly test refusal boundaries where relevant: second Entity, ambiguous route, unlisted leaks, public/unlisted case-fold collisions, unknown Site/Page, property privacy, graphic suppression and generic-library isolation, route/location independence, undeclared source files and duplicate address claims.

**Disposition meanings:**

- `ACCEPT`: reviewed exact revision satisfies scope and gates; does **not** by itself approve merge, release or deployment.
- `REVISE`: bounded failure or missing evidence; Owner decides whether corrective work fits the same approved scope or requires new scope.
- `STOP`: identity, authority, security or scope conflict; no additional edits until resolved.

Do not launch another work order until the prior work has a reviewed disposition and the Owner authorizes progression.

## 8. WIP pruning — mandatory on every transition

**The WIP is a live control surface, not a history library.** Maintain these three files at every accepted planning change, closure and handoff:

- **ACTIVE:** only the currently issued task (or `None`), frozen scope, concise latest evidence, blocker and disposition. Do not accumulate past task reports or multiple work orders; after Owner-reviewed closure, reset to a neutral template and rely on Git history for the old record.
- **NEXT:** only **one** actionable next candidate, with dependencies and a concise ready/not-ready assessment. It is a proposal, not an authorization. Replace it when completed, rejected or superseded; do not append a backlog.
- **PLAN:** three concise horizons: **short term** (next foundational milestone), **medium term** (subsequent capability milestone) and **long term** (strategic direction). Each horizon contains outcomes, dependencies and **documentation to create/review/update/remove**. Remove completed, rejected or invalidated items; move relevant work forward as horizons shift. Never retain completed tasks as crossed-out historical rows.

After the Architect's acceptance, recommend a single coherent WIP reconciliation for Owner approval: preserve durable product facts in About/Architecture; reset ACTIVE; promote the next candidate; prune and rebase PLAN. Every transition must keep links consistent and explain what was pruned in the commit/short handoff—not by adding a history log to WIP. If WIP contents are stale or contradictory, repair WIP authority before authorizing new implementation.

## 9. Repository, dependency and deployment controls
**Workspace confinement:** The implementation agent may read the broader Provelopment workspace as necessary for context, subject to access permissions. All file creation, edits, deletion, writes through scripts or tools, and Git mutations must remain strictly within the verified `00.foundation-solo/` root **and only within paths permitted by the issued work order**. No parent or sibling workspace mutations. If an operation might write outside Solo (package cache, generated output, Git metadata, installer, provider integration), do not execute it without confirming safe containment and explicit Owner scope. The Owner supplies the actual operating-system/tool permission boundary; written instructions supplement rather than replace enforced restrictions.

**Open-source hygiene:** Repository-standard files are narrow, necessary exceptions to the `.documents/` architecture/governance documentation home: root `LICENSE`, `.gitignore`, `.gitattributes`, `.editorconfig`, and host-recognized `.github/` community policy/templates. They may not create parallel architecture, SOP or WIP authority. The Owner has expressly approved **Apache-2.0** and the root `LICENSE` contains it. Copyright/attribution, any required `NOTICE` or third-party acknowledgments, and a working private vulnerability intake require verification before public distribution. A license choice is not a mere code-style matter.


Solo is an independent repository and product. `provelopment-root`, `provelopment-foundation` and `provelopment-web-1` may be read for context; **no writes** to any are in scope for Solo, absent a separately explicit Owner authorization identifying that repository. Avoid relying on root governance or third-party manuals at runtime or in SOP.

The Architect must require the agent to establish the real Git root, ordinary working-tree status and absence of secondary/linked worktrees. Never invent a branch, commit SHA, CI workflow or executable command. Do not assume the local Windows path is accessible from an agent's environment. Provider operations, DNS, secrets, billing, real data transfer, PR merge and release need their own explicit approvals; a code task is not deployment permission.

## 10. Stop conditions and maintenance

STOP and notify Owner upon second-Entity design, disguised Spokes, routing/property coupling, unauthorized public exposure, uncontrolled unlisted discovery, ambiguous base URL semantics, a missing approved task, unrelated repo changes, destructive edits, unauthorized provider operations, incompatible requirements or unverifiable evidence.

Review this SOP whenever the operating model changes; any revision requires Owner acceptance and corresponding updates to `AGENTS.md` / `sop-implementation.md` when needed. This charter should remain practical and finite; details specific to a single task belong in ACTIVE, not in permanent SOP.
