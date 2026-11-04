# AGENTS.md — Mandatory Foundation Solo Agent Entry Point

> **Edition:** 2026-11-04 — initial Owner-review package.  
> **Scope:** Every architect, implementation agent, reviewer and contributor in the independent `00.foundation-solo` repository.  
> **Notice:** Reading this document or finding a proposed task is **not** authorization to edit files.

## 1. First: verify where you are

The intended Owner-local root is `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`. Verify your **actual accessible workspace**, repository boundary, branch, HEAD and worktree status. Solo **is** an independent Git repository on branch `main` with a single worktree and exactly one remote, `origin`, pointing at the public GitHub repository `provelopment/provelopment-foundation-solo`; confirm that URL and authorize no other remote. Do not assume the Windows path is available on your runtime. Do not modify a parent workspace or sibling repository.

Foundation Solo is an independent product with **exactly one Entity** and the hierarchy **Entity → Facet → Site → Page → localized representation**. Each Entity/Facet/Site contains one or more children at the next required level; only localized representations have locales. One default Facet/Site initially; additional ones when authored. **No Spoke Hub, Spoke, hidden implicit Spoke, multi-Entity publication or tenant management.**

## 2. Required reading, in this order

1. [`README.md`](README.md) — product overview and document map.
2. [`.documents/About/FOUNDATION-SOLO.md`](.documents/About/FOUNDATION-SOLO.md) — scope and purpose.
3. [`.documents/Architecture/ARCHITECTURE.md`](.documents/Architecture/ARCHITECTURE.md) — settled and open architecture.
4. [Architect SOP](.documents/SOP/sop-architect.md) and [Implementation SOP](.documents/SOP/sop-implementation.md) — mandatory role-specific procedures.
5. [ACTIVE](.documents/WIP/workorder-active.md), [NEXT](.documents/WIP/plan-next.md), [PLAN](.documents/WIP/plan-outlook.md) — exact current authority, immediate candidate and pruned outlook.
6. [Execution report](.documents/Reports/execution-report.md) — the **one** current run report, overwritten after every run; read it before acting.
7. Actual code, tests, manifests and only the additional product references pertinent to the authorized task.

**Full-locale contract:** Only explicit full, region-qualified locale tags (e.g. `en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`) identify published representations. Bare `en`, `de`, `id` or `zh` must not become authored/public keys. Locale-less entry checks supported saved preference, browser preference and the declared default in that order. Fallback is permitted only to a real public representation of the **same** Page; the declared default is preferred. Do not infer location or Page identity from locale.

**Temporary workspace restriction:** The agent may not create additional Git worktrees (`git worktree add`, linked worktrees), alternate checkouts, nested repository clones, or shadow working directories. All agent-created files—tests, scratch, downloads, generated files, caches and intermediate outputs—must stay **inside the verified Solo root**, and ephemeral material should use Solo root `.temp/` only. Remove agent-owned temporary material as soon as unnecessary and verify no leftover temporary directories/files at task closure. Unknown/Owner-owned contents must never be deleted automatically; stop and report. See the Implementation SOP for exact cleanup gates and tool side-effect controls.

**One authority per concern:** `About/` defines product purpose; `Architecture/` defines EFS and routing constraints; `SOP/` defines roles and procedure; `WIP/workorder-active.md` contains **the only active implementation authorization**. `NEXT` and `PLAN` are non-authorizing. Documents from other repositories are not Solo governance.

## 2A. Handoff protocol — how a run begins and ends

**Authorization has exactly two forms, and nothing else.** *Implementation* changes — source, dependencies, WIP authority, other documentation, or any Git operation beyond the report — require `.documents/WIP/workorder-active.md` labelled `OWNER-ISSUED` with a unique `SOLO-WO-*` ID **and** a matching Owner handoff. *Reporting* is the single standing exception defined below. The Owner alone installs the order; `plan-*.md` files are reference only; neither agent self-issues an order, starts the next one, or approves its own work.

**Verify before you act.** Confirm the actual workspace root (not the parent workspace, a symlink or a junction), `git rev-parse --show-toplevel`, branch `main`, `HEAD`, `git status --porcelain` apart from a permitted order installation, exactly one worktree, exactly one remote `origin` = `provelopment/provelopment-foundation-solo`, exactly three WIP files, and whether `.documents/Reports/execution-report.md` exists and is current. Any unexplained mismatch is a **STOP**.

**Every run ends with the report.** Overwrite `.documents/Reports/execution-report.md` and publish it to `origin/main` when safe — including after a `STOP`, a failure, or a read-only onboarding. It is the Architect's default review source, is never duplicated, dated or archived, and never grants authority. Never embed the report's own final commit SHA inside it; give that SHA in chat after publication.

**Standing report-only permission.** With no issued order an agent may inspect Solo and overwrite, commit and push **only** that one report, after the repository-safety checks above. This is the sole exception to "an order is required before a commit or push". It never authorizes an implementation change, dependency install, other documentation edit, WIP edit, other-repository write or second checkout.

**Where the Architect reads it.** Directly from GitHub at the exact revision — <https://github.com/provelopment/provelopment-foundation-solo/blob/main/.documents/Reports/execution-report.md>. A missing, stale or unpushed report is itself a verification failure, and chat pastes never substitute for it. The Architect then returns `ACCEPT / REVISE / STOP` against that revision and may draft — never issue — the single next order for Owner approval.

**Clean tree, honestly reported.** A completed run leaves an empty `git status --porcelain`, one checkout, no task-created temporary files, and all documentation including the report on GitHub. Unfinished or unsuitable code stays local only when its own order expressly allows it, as a named local branch within the same checkout and declared in the report. Never delete unknown work to manufacture cleanliness; **STOP** instead.

## 3. Non-negotiable product constraints

- **One Entity:** no second Entity, disguised tenant or alternate publisher identity.
- **EFS ownership:** Facet and Site are content contexts, not public route segments or geographic types by definition. Page identity is language-neutral; each published Page has one or more localized representations.
- **Cascading location:** an explicitly authored EFS property at Entity, Facet, Site, Page or representation level, with nearest direct-ancestor declaration taking precedence. A location may be Australia at Entity, Indonesia at Facet, Bali or Jakarta at different Sites. No location is inferred from a URL, ID, locale, hostname or route. Location never selects a route, canonical, language or Page.
- **Configurable base URL:** the Entity's initial configuration explicitly selects an HTTP(S) origin and optional base path; later valid changes must be configuration-only and not re-identify content. Validation and collision rules are required; exact schema awaits acceptance.
- **Public URL case policy:** public paths case-fold to a single lowercase canonical Page URL; unlisted Page-route tokens retain exact case. Never lowercase all paths before determining effective visibility, and refuse public/unlisted case-fold collisions. Full locale tags normalize consistently on both paths.
- **Graphics:** a generic public graphic library is available across the single Installation; authored graphic roles cascade through Entity, Facet, Site, Page and representation; explicit suppression prevents inherited and generic fallbacks.
- **Unlisted content:** exact authorized URL remains reachable; no generated discovery, sitemap, navigation or language alternate advertises an unlisted representation; noindex is applied. Unlisted is **not** an access-control/privacy mechanism.
- **Publication safety:** authored source does not self-publish by filename; ambiguous or missing declarations must fail clearly. No sibling fallback, unsafe executable authored content, private property leakage or generated-over-authored overwrites.
- **Separate concerns:** architecture/ownership, functionality, routing and provider delivery are distinct. Do not derive identity from a public address.
- **Clean slate:** no legacy Foundation compatibility machinery, shared ingress, account/CRM/payment platform or external-service dependency unless separately and specifically approved.

If any instruction conflicts with these accepted requirements, STOP and escalate to Owner/Architect rather than silently weaken them.

## 4. Role and authorization gates
**Filename distinction and live slots:** `plan-*.md` = planning/reference, **not an instruction to execute**; `workorder-*.md` = candidate execution document, executable only when formally Owner-issued with a matching ID and handoff. The entire WIP contains exactly `workorder-active.md`, `plan-next.md` and `plan-outlook.md`; no additional plan/workorder files, task history, or report archive may accumulate.

**Implementation-agent confinement:** The agent may read relevant files elsewhere in Provelopment but has **no write permission outside `00.foundation-solo/`**. The parent repository's `/00.foundation-solo/` ignore rule is **committed** and independently verified with `git check-ignore`; changing it remains outside Solo agent scope, and only the Owner or a separately authorized parent operator may edit/commit the parent `.gitignore`. Even within Solo it may edit only allowed work-order paths. No external writes through generated output, installers, symlinks or Git. Report any permission-boundary uncertainty before proceeding.


**Owner:** final authority. Approves exact architecture, source scope, work orders, merge, release, deployment and provider actions separately.

**Chief Architect:** designs, sequences and reviews; drafts one bounded work order and independently verifies exact implementation evidence; does not implement the same task, issue its own approval or release/deploy unilaterally. Follow [sop-architect.md](.documents/SOP/sop-architect.md).

**Implementation Agent:** can perform **read-only onboarding** without an issued order. May create, modify, install, commit, push or perform external actions **only** within an exact matching Owner-issued work order in [`WIP/workorder-active.md`](.documents/WIP/workorder-active.md) and a matching Owner handoff — except the report-only permission of §2A, which reaches only `.documents/Reports/execution-report.md`. When an order expressly authorizes version-control work, the agent performs the routine Git/GitHub mechanics itself — staging only the named paths, committing, configuring the remote, setting upstream tracking, pushing and verifying the remote and repository settings — rather than asking the Owner to run ordinary Git commands. Push permission never implies authorization to merge, release or deploy. Cannot self-approve scope, architecture or completion. Follow [sop-implementation.md](.documents/SOP/sop-implementation.md).

**Reviewer:** reports evidence, discrepancies and recommendations; a comment is not authorization.

## 5. Exactly three WIP files — permanent operational rule
**Owner-approved license:** Root `LICENSE` is Apache-2.0 as expressly selected by the Owner. Copyright/attribution and any required third-party `NOTICE` must still be settled before a release; GitHub **private vulnerability reporting is enabled** and verified for the public repository. Repository-standard `.github/` and root tooling metadata are narrow exceptions to `.documents/`, not alternate governance.


`.documents/WIP/` contains **only**:

1. [`workorder-active.md`](.documents/WIP/workorder-active.md): one current issued work order (or `NONE`), frozen approved scope, temporary execution evidence and independent disposition.
2. [`plan-next.md`](.documents/WIP/plan-next.md): one candidate for next activity, no backlog or execution permission.
3. [`plan-outlook.md`](.documents/WIP/plan-outlook.md): short-, medium- and long-term outcomes and **documentation outlook**, continuously pruned.

Never create `WIP/Reports`, `WIP/Work-Orders`, `WIP/Archive`, per-task or dated reports, or any duplicate status folder elsewhere. The **single** exception outside WIP is `.documents/Reports/execution-report.md` — overwritten every run, never duplicated, and neither a WIP file nor a source of authority. Git history retains past versions. On Owner-approved closure, reset ACTIVE, replace NEXT and prune PLAN; permanent accepted facts go into the relevant About/Architecture authority.

## 6. Source/quality and stopping rules

- Verify repository, ref, start/end Git revisions, allowed paths, and pre-existing work before touching anything. Never invent a test command or imply tests passed without executing them.
- Use narrowly scoped, comprehensible code changes; verify positive behavior and negative/refusal cases. Preserve EFS hierarchy, location/routing independence, authored base URL flexibility, unlisted content, property privacy and accessibility.
- No other repository changes: especially `provelopment-root`, `provelopment-foundation` or `provelopment-web-1` without a separate explicit Owner-issued authorization naming that scope.
- No unauthorized destructive Git operations, deletions, history rewrites, license changes, network/provider setup, secrets access, PR merge, release or deployment.
- When missing an approval, architectural contract, safe route policy or correct working state: **STOP**, record the actual blocker, and ask for the smallest necessary Owner decision. Do not silently invent a substitute.

## 7. Current state

This package supplies planning and operating documents plus a published source repository — **not** verified implementation. Solo is an independent Git repository on `main`; `origin` is the public GitHub repository `provelopment/provelopment-foundation-solo`, owned by the personal `provelopment` account, with default branch `main` and GitHub private vulnerability reporting enabled. The parent `/00.foundation-solo/` ignore guard is committed and verified, so the enclosing workspace tracks no Solo files. There is **no application code, toolchain, dependency manifest, test suite, release or deployment**; do not describe any of those as existing. **Phase 0 is delivered and PAUSED, pending independent Architect review** of the current [execution report](.documents/Reports/execution-report.md). [`workorder-active.md`](.documents/WIP/workorder-active.md) records `NONE — PHASE 0 PAUSED`, and [`plan-next.md`](.documents/WIP/plan-next.md) records `NONE — PAUSED`; no executable task exists. Do not start Phase 1, draft its order, select a framework or write application code until the Owner issues a new work order.
