# Foundation Solo

> **Status:** New, independent open-source project. The architecture and operating documentation are published as a **public source repository**, [`provelopment/provelopment-foundation-solo`](https://github.com/provelopment/provelopment-foundation-solo), 2026-11-04. Publication covers **source and documentation only**: there is **no implemented application, no verified test suite, no released version and no deployment**.

## Start here

A new Architect or Implementation Agent can begin with this file and [`AGENTS.md`](AGENTS.md). Read in this order:

1. **This file** — product overview, current status and document map.
2. **[`AGENTS.md`](AGENTS.md)** — mandatory agent entry point: authorization gates, write boundary and handoff protocol.
3. **[Product definition](.documents/About/FOUNDATION-SOLO.md)** and **[architecture charter](.documents/Architecture/ARCHITECTURE.md)** — purpose, and settled versus open architecture.
4. **[Architect SOP](.documents/SOP/sop-architect.md)** and **[Implementation SOP](.documents/SOP/sop-implementation.md)** — role procedures; the SOPs hold the single version-control procedure (§6B, §5B) and the mandatory reporting obligation (§7A, §5C).
5. **[ACTIVE](.documents/WIP/workorder-active.md)**, **[NEXT](.documents/WIP/plan-next.md)**, **[PLAN](.documents/WIP/plan-outlook.md)** — the only executable authority, the one next candidate, and the pruned outlook.
6. **The current execution report** — [`.documents/Reports/execution-report.md`](.documents/Reports/execution-report.md), also readable on GitHub at <https://github.com/provelopment/provelopment-foundation-solo/blob/main/.documents/Reports/execution-report.md>. It is overwritten after **every** run, including failed and read-only runs, and is the Architect's default review source.

**Current phase:** Phase 0 is **delivered and PAUSED, pending independent Architect review**. `plan-*.md` files are reference only; only an `OWNER-ISSUED` `workorder-active.md` with a matching Owner handoff authorizes work. No application implementation, framework selection, release or deployment is authorized while paused.

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

**Location is an explicitly authored cascading EFS property, not a routing input.** Entity location `Australia` can be overridden by Facet `Indonesia` and then by Sites `Bali` or `Jakarta`; their `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` localized Page representations do not infer or alter location.

**Public URL case policy:** Public Page URLs resolve case-insensitively and have lowercase canonical paths; `de-DE` and `de-de` identify the same supported full locale. **Unlisted Page-route tokens preserve exact case** (`abc`, `Abc`, `ABC` are distinct) and may never be exposed by a public case-insensitive alias. The distinction is based on the effective visibility of the localized representation, including Page-wide defaults and per-representation overrides.

**Unlisted Page representations are retained.** A Page-level visibility default may mark all representations unlisted, and each representation can explicitly override it. Unlisted URLs are reachable at their exact authorized spelling but excluded from discovery; unlisted is not private. Locale syntax is normalized consistently even for unlisted content; only its authored route tokens retain case sensitivity.

**The Entity's base URL is configurable at initial creation** as a valid HTTP(S) origin with an optional base path, such as `https://a.abc.com/something` or `https://www.abc.com/another-something`. Changing a configured public address must not change the Entity's identity or require routing architecture changes. Exact normalization, aliases and route-collision rules remain technical decisions.

## Project documents
**Planning versus execution:** The three live WIP files use semantic filenames: `workorder-active.md` for the sole issued work order (or `NONE`), `plan-next.md` for the one next candidate, and `plan-outlook.md` for pruned short/medium/long horizons. `plan-*.md` is never executable; `workorder-*.md` is executable only after exact Owner issuance plus matching handoff. These are the **only** files in WIP.


All authoritative Solo documentation resides in **`.documents/`**, except the two root architectural entry points `README.md` and `AGENTS.md` and the narrowly necessary repository-standard licensing, tooling and `.github/` community files (which create no parallel policy authority).

| Read | Purpose |
| --- | --- |
| [Product definition](.documents/About/FOUNDATION-SOLO.md) | Product identity, desired capabilities, exclusions and fixed goals |
| [Architecture charter](.documents/Architecture/ARCHITECTURE.md) | EFS hierarchy, properties, location independence, base URL and routing, unlisted content, open contracts |
| [Architect SOP](.documents/SOP/sop-architect.md) | Owner decision flow, bounded work orders, independent review and WIP pruning |
| [Implementation SOP](.documents/SOP/sop-implementation.md) | Read-only onboarding, authorization, Git preflight, implementation, testing, reporting and stop rules |
| [ACTIVE](.documents/WIP/workorder-active.md) | **Only** current authorized task (or none), execution evidence and review disposition |
| [NEXT](.documents/WIP/plan-next.md) | **One** immediate candidate, not implementation permission |
| [PLAN](.documents/WIP/plan-outlook.md) | Pruned short-, medium- and long-term work **and documentation** outlook |
| [AGENTS.md](AGENTS.md) | Mandatory entry-point instructions for every agent and contributor |
| [Execution report](.documents/Reports/execution-report.md) | **One** current run report, overwritten every run; the Architect's default review source |

**Strict single-root discipline:** Implementation agents may not use additional Git worktrees, shadow checkouts, nested clones or temporary directories outside Solo. Agent-created scratch and redirectable caches belong under Solo `.temp/` and must be promptly cleaned; the SOP forbids deleting pre-existing unknown files. Parent-repository `.gitignore` changes are outside Solo agent permissions.

**Graphics and assets:** A generic, public, platform-provided graphic library is available to the whole Installation. Authored Entity graphic roles and selections cascade through all five EFS levels; optional generic role fallbacks are beneath the authored cascade. Explicit suppression must not silently re-enable a generic default.

**Strict WIP rule:** `.documents/WIP/` contains exactly `workorder-active.md`, `plan-next.md` and `plan-outlook.md`. No additional task log, backlog, work-order, report, historical archive or dated status file. Git history supplies completed-task traceability; durable approved architectural facts live in the existing canonical product/architecture documents.

## Installation, authoring and lifecycle

The goal is to establish one Entity and its chosen base URL, then author and publish content by modifying validated configuration, Pages and assets—not core application code for normal authoring. A small default configuration should expand into multi-Facet/Site and multilingual content without adding hidden publishing layers.

Solo's implementation technology, executable commands, file layout, content manifest, ID grammar, release contract and deployment method have **not yet been accepted or implemented** by this documentation package. Apache-2.0 is the Owner-approved license. Do not borrow historical Foundation commands and present them as Solo's commands.

## Roles and authorization

The **Owner** approves architecture, source-changing work and separate merge/release/deployment decisions. The **Chief Architect** develops bounded proposals, requirements and independent acceptance; the **Implementation Agent** executes only an exact Owner-issued order. The Architect and implementation role are distinct.

An agent may initialize **read-only** without a work order. Any **implementation** change — source write, Git initialization, dependency install, commit, push or external operation — requires explicit authorized scope in `.documents/WIP/workorder-active.md` and an Owner handoff. The **one** exception is the standing report-only permission: with no active order an agent may overwrite, commit and push **only** `.documents/Reports/execution-report.md`, after the repository-safety checks in the SOPs. That exception never authorizes implementation, WIP edits, dependency changes or a second checkout. `plan-next.md` and `plan-outlook.md` never authorize implementation. See [AGENTS.md](AGENTS.md) §2A and the version-control and reporting sections of both SOPs (§6B/§7A and §5B/§5C) before acting.

## Independence
## Open-source repository hygiene

The root [`LICENSE`](LICENSE) contains **Apache License 2.0**, expressly approved by the Owner for Foundation Solo. The repository is published publicly and GitHub **private vulnerability reporting is enabled** for it (see [`SECURITY.md`](.github/SECURITY.md)). Copyright attribution, any required `NOTICE`/third-party acknowledgments and an adopted supported-version/support policy remain outstanding and must be settled before any release; license approval alone is not release authorization.

Standard community-facing files live in [`.github/`](.github/) so GitHub recognizes them: [`CONTRIBUTING.md`](.github/CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](.github/CODE_OF_CONDUCT.md), [`SECURITY.md`](.github/SECURITY.md), and a PR template. Root `.gitignore`, `.gitattributes` and `.editorconfig` establish simple tool-neutral hygiene. These are platform-recognized file-placement exceptions, not alternatives to the exclusive Solo architecture/operations documentation in `.documents/`. The private vulnerability-reporting channel is enabled; a supported-version/support policy is still not adopted.


Solo is an independent repository and product. Historic Foundation and other Provelopment repositories can be read as reference but are **not governing Solo documents and are not authorized write targets**. All operational knowledge needed for Solo must be documented here. The Owner's intended local root is `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; agents must verify their actual available workspace.

**Next action:** Phase 0 governance is delivered and **paused pending independent Architect review** of the published [execution report](.documents/Reports/execution-report.md) on GitHub. The project resumes only on a new Owner-issued work order; [NEXT](.documents/WIP/plan-next.md) records `NONE — PAUSED`. No application source, framework selection, release or deployment is authorized by this README.
