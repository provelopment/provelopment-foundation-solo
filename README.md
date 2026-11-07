# Foundation Solo

> **Status:** New, independent open-source project. The architecture and operating documentation are published as a **public source repository**, [`provelopment/provelopment-foundation-solo`](https://github.com/provelopment/provelopment-foundation-solo), 2026-11-04. Publication covers **source and documentation only**: there is **no implemented application, no verified test suite, no released version and no deployment**.

## Start here

A new Architect or Implementation Agent can begin with this file and [`AGENTS.md`](AGENTS.md). Read in this order:

1. **This file** — product overview, current status and document map.
2. **[`AGENTS.md`](AGENTS.md)** — mandatory agent entry point: authorization gates, write boundary and handoff protocol.
3. **[Product definition](.documents/About/FOUNDATION-SOLO.md)** and **[architecture charter](.documents/Architecture/ARCHITECTURE.md)** — purpose, and settled versus open architecture.
4. **[Architect SOP](.documents/SOP/sop-architect.md)** and **[Implementation SOP](.documents/SOP/sop-implementation.md)** — role procedures; the single authoritative version-control procedure is Architect SOP §6B, to which Implementation SOP §5B binds, and the mandatory reporting obligation is §7A and §5C.
5. **[ACTIVE](.documents/WIP/workorder-active.md)**, **[NEXT](.documents/WIP/plan-next.md)**, **[ROADMAP](.documents/WIP/plan-roadmap.md)** and **[OUTLOOK](.documents/WIP/plan-outlook.md)** — the only executable authority, the one next candidate, the remaining committed stages, and optional ideas.
6. **The current execution report** — [`.documents/Reports/execution-report.md`](.documents/Reports/execution-report.md), also readable on GitHub at <https://github.com/provelopment/provelopment-foundation-solo/blob/main/.documents/Reports/execution-report.md>. It is overwritten after **every** run, including failed and read-only runs, and is the Architect's default review source.
7. **[Security policy](.github/SECURITY.md)** — the private vulnerability reporting channel. Security is a cross-cutting requirement planned before code rather than a late stage: [Architecture](.documents/Architecture/ARCHITECTURE.md) §5A fixes the design boundaries, the single binding acceptance standard is **A1** in [Architect SOP](.documents/SOP/sop-architect.md) §7B, and [Implementation SOP](.documents/SOP/sop-implementation.md) §7A mirrors it for agents. There is no separate advisory register or watchlist.

**Current status:** governance documentation is delivered. Work is authorized **only** by the order currently installed in [`workorder-active.md`](.documents/WIP/workorder-active.md) **while** it carries the `OWNER-ISSUED` marker **and** a matching Owner handoff has been delivered; independent Chief Architect `ACCEPT / REVISE / STOP` follows the **published** [execution report](.documents/Reports/execution-report.md) at its exact revision, never a chat claim. A neutral ACTIVE slot holds no current order — that is the ordinary absence of authority, not a project freeze or a reopening gate. `plan-*.md` files are reference only. Two contracts are settled for later implementation and recorded canonically in [ARCHITECTURE.md](.documents/Architecture/ARCHITECTURE.md): **strict Hexagonal Architecture (Ports and Adapters)** with inward-only dependencies, and the contact scope — safe `mailto:` actions and a **simple** contact form included, **advanced contact forms excluded**. Nothing here is implemented; no framework, release or deployment is chosen.

**Foundation Solo is a first-class, configuration-first website foundation for exactly one Entity.** An Entity may have any purpose or identity. Solo deliberately avoids presuming a particular type of owner.

## The EFS content model
**Page language:** Each Page is identified once, independently of its localized representations. Published representation keys use **full locale tags** such as `en-001`, `de-DE`, `id-ID` and `zh-Hans-CN`; bare `en`, `de`, `id`, `zh` are not valid authored/public locale keys. Explicit full-locale URLs win. For locale-less entry: supported saved preference → supported `Accept-Language` → declared default. Missing translations preferably redirect to an expressly authorized, real **public representation of the same Page** at its actual URL; otherwise not-found. No inferred language, location or translation. Unlisted behavior remains per representation with convenient Page-wide default. See [architecture language and visibility contracts](.documents/Architecture/ARCHITECTURE.md#32-localize-only-what-is-localized).


```text
Entity — exactly one per Installation
  └── Facet — one or more per Entity
        └── Site — one or more per Facet
              └── Page — one or more per Site
                    └── Localized representation — one or more per published Page
```

An initial Installation has one default Facet and Site; additional Facets, Sites and Pages are available without a second Entity or a new publishing hierarchy. Solo has **no Spokes, Spoke Hubs, shared-tenant ingress or multi-Entity publication system**.

**Location is explicitly authored EFS property metadata, never a routing input.** Direct-ancestry inheritance is additive: an Entity fact `Australia` and a Facet fact `Indonesia` can both remain effective when the Facet selects `inherit`; a Site may select `suppress` and contribute `Bali` or `Jakarta` alone. Structured location records may be explicitly extended by identified reference, never implicitly completed from a route, locale, name or geography; `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` do not alter authored location.

**Public route case and language freedom:** a public localized route is **case-insensitive by default** with a lowercase canonical spelling, while the author may deliberately choose **case-sensitive** matching for an individual localized route, whose canonical spelling keeps intentional capitalisation; **unlisted Page routes always stay exact-case**. Validated Unicode routes and optional localized slugs are first-release capabilities, so one Page may be published under different route spellings in different full locales. `de-DE` and `de-de` remain one supported full locale, and locale normalization never lowercases a case-sensitive Page route. Ambiguous overlaps, public/unlisted caseless collisions and duplicate canonicals refuse publication. Any visitor can choose an actually published language without an account, and an explicit full-locale URL always wins over preference or location.

**Unlisted Page representations are retained.** A Page-level visibility default may mark all representations unlisted, and each representation can explicitly override it. Unlisted URLs are reachable at their exact authorized spelling but excluded from discovery; unlisted is not private. Locale syntax is normalized consistently even for unlisted content; only its authored route tokens retain case sensitivity.

**The Entity's base URL is configurable at initial creation** as a valid HTTP(S) origin with an optional base path, such as `https://a.abc.com/something` or `https://www.abc.com/another-something`. Changing a configured public address must not change the Entity's identity or require routing architecture changes. **R00.B now accepts** safe standards-based base-address normalization, explicitly authorized optional aliases, validated Unicode localized route claims, author-selected public case sensitivity and fail-closed collision rules; the physical schema, the pinned Unicode/toolchain implementation and deployment/provider wiring remain later work, and no router or deployment is yet implemented.

## Project documents
**Planning versus execution:** The four live WIP files use semantic filenames and have deliberately different jobs: `plan-roadmap.md` holds only the still-pending implementation stages, `plan-next.md` the single bounded next candidate, `workorder-active.md` the sole issued work order (or no order), and `plan-outlook.md` optional ideas that may never be built. ROADMAP → NEXT → ACTIVE is the only progression; OUTLOOK sits outside it. `plan-*.md` is never executable; `workorder-*.md` is executable only after exact Owner issuance plus matching handoff. These are the **only** files in WIP.


All authoritative Solo documentation resides in **`.documents/`**, except the two root architectural entry points `README.md` and `AGENTS.md` and the narrowly necessary repository-standard licensing, tooling and `.github/` community files (which create no parallel policy authority).

| Read | Purpose |
| --- | --- |
| [Product definition](.documents/About/FOUNDATION-SOLO.md) | Product identity, desired capabilities, exclusions and fixed goals |
| [Architecture charter](.documents/Architecture/ARCHITECTURE.md) | EFS hierarchy, properties, location independence, base URL and routing, unlisted content, open contracts |
| [Architect SOP](.documents/SOP/sop-architect.md) | Owner decision flow, bounded work orders, independent review and WIP pruning |
| [Implementation SOP](.documents/SOP/sop-implementation.md) | Read-only onboarding, authorization, Git preflight, implementation, testing, reporting and stop rules |
| [ACTIVE](.documents/WIP/workorder-active.md) | **Only** current authorized task (or none), execution evidence and review disposition |
| [ROADMAP](.documents/WIP/plan-roadmap.md) | Still-pending implementation stages, work packages and gates; never implementation permission |
| [NEXT](.documents/WIP/plan-next.md) | **One** bounded immediate candidate, not implementation permission |
| [OUTLOOK](.documents/WIP/plan-outlook.md) | Optional ideas and aspirations only; never a requirement, backlog or delivery promise |
| [AGENTS.md](AGENTS.md) | Mandatory entry-point instructions for every agent and contributor |
| [Execution report](.documents/Reports/execution-report.md) | **One** current run report, overwritten every run; the Architect's default review source |

**Strict one-checkout discipline:** No additional Git worktrees, shadow checkouts, nested clones or alternate source trees, including within temporary folders. For otherwise authorized work, Implementation Agents may use **both** Solo's Git-ignored `.temp/` and Cline's own task temporary workspace without separate per-file permission. Disposable, identifiable task-owned files and folders must be removed when no longer needed, ordinarily by the end of the run; never accumulate garbage or delete unfamiliar/Owner-owned material. Permanent project writes remain confined to approved Solo paths. See both SOPs for cleanup and evidence rules; the parent `.gitignore` remains outside Solo agent authority.

**Graphics and assets:** A generic, public, platform-provided graphic library is available to the whole Installation. Authored Entity graphic roles and selections cascade through all five EFS levels. **No automatic generic role fallback is approved for any role**: every automatic role default is disabled unless expressly approved, so an unresolved role renders nothing rather than silently substituting a generic graphic, and explicit suppression never re-enables such a default.

**Strict WIP rule:** `.documents/WIP/` contains exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`. No additional task log, backlog, work-order, report, historical archive or dated status file. Git history supplies completed-task traceability; durable approved architectural facts live in the existing canonical product/architecture documents.

## Installation, authoring and lifecycle

The goal is to establish one Entity and its chosen base URL, then author and publish content by modifying validated configuration, Pages and assets—not core application code for normal authoring. A small default configuration should expand into multi-Facet/Site and multilingual content without adding hidden publishing layers.

Solo's implementation technology is **selected only conditionally** in [ARCHITECTURE.md](.documents/Architecture/ARCHITECTURE.md) §6A and still owes all of its conformance proof; the JSON/Markdown and manifest schemas are **specified** in §6.2 but no schema file, validator or executable command exists; and the lifecycle, release, support, accessibility and performance decisions are **Owner-approved documentary contracts** in §6C that are **not implemented**. There is no application code, installed toolchain, dependency manifest, test suite, CI job, release or deployment. The Entity/Facet/Site/Page **identifier grammar and publication-declaration semantics *are* accepted** as recorded decisions in §1.1–§1.4; that is a specification, not an implemented validator. Apache-2.0 is the Owner-approved license. Do not borrow historical Foundation commands and present them as Solo's commands.

## Roles and authorization

The **Owner** approves architecture, source-changing work and separate merge/release/deployment decisions. The **Chief Architect** develops bounded proposals, requirements and independent acceptance; the **Implementation Agent** executes only an exact Owner-issued order. The Architect and implementation role are distinct.

An agent may initialize **read-only** without a work order. Any **implementation** change — source write, Git initialization, dependency install, commit, push or external operation — requires explicit authorized scope in `.documents/WIP/workorder-active.md` and an Owner handoff. The **one** exception is the standing report-only permission: with no active order an agent may overwrite, commit and push **only** `.documents/Reports/execution-report.md`, after the repository-safety checks in the SOPs. That exception never authorizes implementation, WIP edits, dependency changes or a second checkout. `plan-roadmap.md`, `plan-next.md` and `plan-outlook.md` never authorize implementation. See [AGENTS.md](AGENTS.md) §2A and the version-control and reporting sections of both SOPs (§6B/§7A and §5B/§5C) before acting.

## Independence

Solo is an independent repository and product. Historic Foundation and other Provelopment repositories can be read as reference but are **not governing Solo documents and are not authorized write targets**. All operational knowledge needed for Solo must be documented here. The Owner's intended local root is `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; agents must verify their actual available workspace.

## Open-source repository hygiene

The root [`LICENSE`](LICENSE) contains **Apache License 2.0**, expressly approved by the Owner for Foundation Solo. The repository is published publicly and GitHub **private vulnerability reporting is enabled** for it (see [`SECURITY.md`](.github/SECURITY.md)). Copyright attribution, any required `NOTICE`/third-party acknowledgments and an adopted supported-version/support policy remain outstanding and must be settled before any release; license approval alone is not release authorization.

Standard community-facing files live in [`.github/`](.github/) so GitHub recognizes them: [`CONTRIBUTING.md`](.github/CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](.github/CODE_OF_CONDUCT.md), [`SECURITY.md`](.github/SECURITY.md), and a PR template. Root `.gitignore`, `.gitattributes` and `.editorconfig` establish simple tool-neutral hygiene. These are platform-recognized file-placement exceptions, not alternatives to the exclusive Solo architecture/operations documentation in `.documents/`. The private vulnerability-reporting channel is enabled; a supported-version/support policy is still not adopted.

**Next action:** only the order currently installed in [ACTIVE](.documents/WIP/workorder-active.md) is authorized, and only while it remains `OWNER-ISSUED` with its matching Owner handoff. [NEXT](.documents/WIP/plan-next.md) holds at most one non-executable candidate and [ROADMAP](.documents/WIP/plan-roadmap.md) the remaining committed stages. Work begins only with a newly Owner-issued `SOLO-WO-*` order installed in ACTIVE plus a matching handoff; the [execution report](.documents/Reports/execution-report.md) on GitHub is the current evidence. No application source, framework selection, release or deployment is authorized by this README.
