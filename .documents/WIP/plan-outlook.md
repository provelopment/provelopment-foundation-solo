# Foundation Solo — plan-outlook (Short / Medium / Long Term)

**Purpose:** A compact, pruned **short-, medium- and long-term outlook**, including what documentation must be maintained. This file is directional; it is **not** an implementation order or a second queue.  
**Status:** Initial planning outlook — proposals, not Owner-approved technical contracts.  
**Last reviewed:** 2026-11-04 (Owner URL-case/graphic decisions; Phase 0 governance delivered and paused pending Architect review)  
**Execution authority:** Only a matched Owner-issued [`workorder-active.md`](workorder-active.md) order. Immediate candidate: [`plan-next.md`](plan-next.md).
**Filename contract:** all `plan-*.md` material is non-executable. A plan cannot be promoted into authorization by renaming it; the Architect must produce an exact approved `workorder-active.md` scope and the Owner must issue a matching handoff.

## Planning discipline

- **Short term:** next foundational decision and executable milestone; adapt to completed facts rather than fixed dates.
- **Medium term:** the following coherent capability milestone, dependent on short-term evidence.
- **Long term:** strategic product quality and sustainable release/operation outcomes; not speculative feature accumulation.
- Each horizon identifies outcomes, dependency/acceptance hinge and a **documentation outlook** (author/update/review/retire). Entries are compact themes, not detailed task orders.
- Review and prune this file **at every work-order acceptance, Owner change in priorities, and before each new issue**. Move unfinished relevant items toward the near horizon when dependencies clear. Delete completed, rejected, invalidated and redundant items entirely; do not strike through or retain an archive.
- Do not create more files under `WIP/`, additional status boards elsewhere or dated plan copies. Git records transitions. Detailed approved work belongs in ACTIVE, not here.

## Short-term outlook — establish a reliable Solo foundation

**Intended outcomes**

1. **Phase 0 governance is delivered and paused** pending independent Architect review. Keep the single canonical execution report, the clean-tree gate, the report-only standing permission and the Solo-only write confinement (with no-worktree/no-temp-junk controls) enforced on every future run.
2. Once the Owner reopens the project, resolve the smallest blocking technical choices for the first cohesive implementation: Entity/Facet/Site/Page identity and ID grammar; the minimal authoring data structure; project bootstrap/toolchain and test discipline; authored base URL and route grammar; cascading property semantics including location and graphic roles; and unlisted representation semantics.
3. Apache-2.0 is Owner-approved and the repository is published publicly with GitHub private vulnerability reporting enabled. Settle copyright/attribution and any required third-party notices, and adopt a supported-version/support policy, **before** any release; maintain minimal repository hygiene.
4. Issue **one** bounded Owner-approved work order for the first implementation checkpoint; prove its output before proceeding.

**Dependencies / gate:** Architect acceptance of the Phase 0 execution report, then an explicit Owner reopening; Owner accepts technical contracts and exact work-order scope; no unrelated repository is changed.

**Documentation outlook**

- **Review/update:** `About/FOUNDATION-SOLO.md` and `Architecture/ARCHITECTURE.md` with newly accepted rules only; avoid making examples into schema decisions.
- **Maintain:** `SOP/sop-architect.md`, `SOP/sop-implementation.md`, `AGENTS.md`, this WIP triad for unambiguous authorization and handoff, and `.documents/Reports/execution-report.md` as the single current evidence file.
- **Review:** Owner-approved Apache-2.0 `LICENSE`, `.github/` community-health files, `.gitignore`, `.gitattributes`, `.editorconfig`; the private vulnerability intake is enabled, so keep `SECURITY.md` accurate as the support policy develops. Inspect `.temp/` cleanup and no-worktree evidence in each implementation acceptance.
- **Consolidate/remove:** obsolete or contradictory drafts once accepted facts have an authoritative home. Do not create a second work-order file.

## Medium-term outlook — prove the end-to-end EFS experience

**Intended outcomes**

1. Demonstrate one Entity with its default Facet/Site and optional additional Facets/Sites, each containing real Pages and localized representations.
2. Prove typed property inheritance and explicit location override/suppression without deriving geography or locales from routes; demonstrate the Australia → Indonesia → Bali/Jakarta example using authored full-locale tags.
3. Prove configurable hostname plus optional base path, a configuration-only base URL change, **full-locale URL segments**, explicit browser language-to-full-tag preference maps and preferred same-Page default fallback; public lowercase canonical paths/case-insensitive access versus exact-case unlisted route tokens; Page-wide/per-representation visibility; plus fail-closed mixed-visibility route collision validation.
4. Establish an accessible presentation shell, shared generic public graphic library and EFS-cascading graphic roles (including explicit suppression), and relevant SEO/asset behavior for at least one genuinely rendered example.

**Dependencies / gate:** accepted schemas and routes; focused plus negative tests; evidence from exact code revision and an actual rendered result.

**Documentation outlook**

- **Update:** canonical architecture with approved data, source layout, visibility, canonical URL, schema, test and adapter contracts, each in its single existing authority.
- **Prepare only as needed:** bounded authoring and setup guidance under `.documents/` after actual commands and configurations exist; do not invent commands before implementation.
- **Prune:** intermediate speculative alternatives after decisions; move actual work into NEXT/ACTIVE only when it is ready for a single scope.

## Long-term outlook — usable, maintainable open-source product

**Intended outcomes**

1. A first-class independent Solo installation and content-update experience with clear verification, recovery, and controlled product upgrades.
2. High-quality accessibility, performance, security, safe public data projection, localization and transparent diagnostics across expanded EFS configurations.
3. Optional, deliberately scoped public integrations—contact, links/actions, analytics—without mandatory external accounts or a multi-Entity administration platform.
4. A reproducible, licensable, provider-neutral release and understandable new-adopter experience, separately authorized when ready.

**Dependencies / gate:** passed prior implementation milestones, independently reviewed evidence, acceptance of release/lifecycle doctrine and explicit Owner authority for any deployment or publication.

**Documentation outlook**

- **Maintain:** straightforward user-facing installation, customization, content authoring, testing and deployment instructions backed by working code.
- **Review:** architecture and SOP for unnecessary abstractions, duplicated rules and obsolete wording; retain one canonical home per topic.
- **Retire/replace:** draft statements, outdated setup instructions and experiments that no longer describe the accepted product; never keep historical progress notes in WIP.

## Transition rule

At closure of each accepted checkpoint: **(1)** record durable decisions in their canonical About/Architecture location; **(2)** reset ACTIVE only with Owner authorization; **(3)** replace NEXT with the sole actual next candidate (or `NONE`); **(4)** rebase all three horizons here and remove completed/obsolete text; **(5)** verify the folder still has **exactly three** files. Do not retain completed work as a WIP archive.
