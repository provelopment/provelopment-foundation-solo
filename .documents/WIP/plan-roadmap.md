# Foundation Solo — Implementation Roadmap

## Stage 00 — Architectural contract dossier (decisions only)

**Outcome:** every technical choice blocking the first coherent Solo implementation is recorded as an explicit Owner decision; no executable application code, dependency install, release or deployment.

**Remaining packages**

- **R00.A — Identity and publishing declarations:** settle stable Entity/Facet/Site/Page IDs and their scope, default Facet/Site names, whether incomplete drafts can exist outside the publishable model, authoritative manifest/content boundaries, source-to-declaration integrity and fail-closed duplicate/orphan detection.
- **R00.B — Address/locale contract:** settle configured HTTP(S) origin and optional base-path normalization, path/segment encoding, Site/Page routing and Home conventions, redirect status and aliases, route collisions, locale preference mapping, error representation and full-locale URL construction; preserve the settled public case-insensitivity and exact-case unlisted Page-token distinction.
- **R00.C — Property/graphic contract:** settle typed values, property declaration/suppression/public projection and provenance encoding; the five-level location cascade; graphic-role catalogue, authored/generic asset references and fallback safety.
- **R00.D — Content and publication contract:** settle versioned JSON/Markdown declarations, `visibilityDefault`/representation `visibility` placement, bounded Page sections, Markdown safety, author link classification, content source precedence and asset path authority.
- **R00.E — Software structure and toolchain:** compare credible framework/language/build/testing choices, choose simplest verifiably strict Ports-and-Adapters design, dependency/port catalogue, composition root, boundary test method, CI and browser proof architecture. Do not inherit Foundation's Next.js/pnpm choice by default.
- **R00.F — Contact and integrations:** specify safe `mailto:`; simple form field set, validation, consent/privacy, rate/abuse policy, delivery-failure behavior, storage/retention and port responsibilities. Explicitly exclude advanced contact forms. Define whether and how static booking/directions links and privacy-respecting analytics meet the first release.
- **R00.G — Lifecycle and nonfunctional standards:** settle initial setup/preview/content-update/upgrade/recovery responsibilities, immutable release identity, deployment portability, supported versions and security response, accessibility/performance targets and precise acceptance matrix.

**Exit gate:** documented, consistent Owner-approved contracts in canonical About/Architecture documents; a decision matrix showing accepted versus open non-blocking matters; a minimal first code-bearing complete work order prepared for Owner review. No important design decision is delegated silently to an implementation agent. **No code exists solely because this stage is accepted.**

## Stage 01 — Reproducible engineering and hexagonal skeleton

**Prerequisite:** Stage 00 architecture/toolchain choices needed for the skeleton accepted.

**Remaining packages**

- **R01.A:** establish the selected minimal source layout, strictly typed configuration, pinned tooling/dependencies and deterministic local build, without borrowing another repository's runtime.
- **R01.B:** introduce domain/application/port/adapter boundaries and one composition root, with explicit import-boundary enforcement including aliases and re-exports; ports must serve actual use cases rather than decorative abstractions.
- **R01.C:** establish unit, integration, architecture and production-mode/browser smoke infrastructure plus CI; ensure tests report failures, no hidden compatibility runtime, and reproducible workspace-cleanup behavior.

**Exit gate:** clean-room dependency installation and build, one genuine request-to-application-port or equivalent minimal vertical-slice proof, green static checks and architecture test that fails on deliberately injected forbidden imports. No premature feature shell or provider dependency.

## Stage 02 — One-Entity EFS identity and publication authority

**Prerequisite:** skeleton and accepted ID/manifest contracts.

**Remaining packages**

- **R02.A:** implement immutable, language-neutral ownership identities and cardinality; one Entity only, one or more Facets/Sites/Pages and at least one representation per published Page.
- **R02.B:** implement the single validated authoring/publication manifest and provenance-safe source references; declaration alone does not imply successful publication, and filename presence never self-publishes.
- **R02.C:** implement simple default installation seed and expanded synthetic fixtures for multiple Facets and Sites; preserve one Entity even with complex nesting.

**Exit gate:** creation/reading of valid one-Entity structures; refusal of second Entity, duplicates, orphans, absent representations, undeclared source publication and cross-parent contamination. Identity unaffected by label, locale, route or base URL changes. Core tests use no provider/framework dependencies.

## Stage 03 — Typed properties, location independence and graphics

**Prerequisite:** stable EFS ancestry and declaration model.

**Remaining packages**

- **R03.A:** implement five-level direct-ancestry property resolution: absent inherits, explicit narrower declaration overrides, explicit suppression remains suppressed; retain provenance and enforce private/public projection.
- **R03.B:** implement authored `location` values at all five levels and demonstrate Australia → Indonesia → Bali/Jakarta and optional representation-level overrides, with no route/language/URL or geolocation effects.
- **R03.C:** implement graphic role declarations, public shared generic library, authored asset ownership, inheritance/override/suppression, and optional role-specific generic fallback strictly beneath authored hierarchy.

**Exit gate:** property resolution and negative isolation tests, no sibling leakage, no private or suppressed values exposed in public output, no fallback after explicit suppression. Changing location never changes Page identity, route, locale or canonical. No cross-Entity generic asset ownership is created.

## Stage 04 — Routing, localization, visibility and first real publication

**Prerequisite:** accepted base-address, locale and visibility contracts; identity and property cores.

**Remaining packages**

- **R04.A:** implement safe configurable public base origin/prefix, route claim validation, public lowercase canonical paths, case-insensitive public lookup and exact-case unlisted Page-route tokens; reject mixed public/unlisted collision and encoded-path ambiguity *before* serving content.
- **R04.B:** implement full canonical BCP 47 with regions (`en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`), locale-less selection by supported saved preference → mapped `Accept-Language` → authored default, explicit full-locale preference precedence and truthful same-Page public-only fallback.
- **R04.C:** implement Page `visibilityDefault` (default public) plus representation `visibility`, per-representation effective state, direct exact unlisted access and noindex/nofollow; refuse unlisted fallback/discovery and invalid mixed claims.
- **R04.D:** deliver **one genuine production-mode vertical slice** from configured HTTP(S) base URL and request through route resolution, Page/representation validation, public output, error/not-found and canonical response. Do not postpone this proof to a future front-end migration.

**Exit gate:** browser/HTTP positive and negative matrix for base URLs with/without prefixes, config-only URL change, full-locale variants, unknown paths, missing representations, public case aliases, mixed visibility, exact-case unlisted tokens, no locale/location inference, and 404 versus 500 correctness. Tests use the real request path, not a legacy fixture fallback.

## Stage 05 — Safe authoring and complete Page rendering

**Prerequisite:** real published Page pipeline and accepted authoring schemas.

**Remaining packages**

- **R05.A:** implement strict versioned declarative JSON Page sections and safe Markdown authoring, with clear bounded semantics, page title/heading and accessible content requirements.
- **R05.B:** implement source discovery tied to the manifest, parse diagnostics naming exact authored paths/fields, deterministic source precedence, dictionaries, and safe link/asset references; refuse executable authored content.
- **R05.C:** implement reusable section rendering and preview/validation author workflow with one consistent publication result for JSON and Markdown, including identical visibility behavior.

**Exit gate:** real JSON and Markdown Pages render under one router; unknown section types, malformed documents, unsafe URLs, duplicate source claims, path traversal, raw executable content and absent declarations fail clearly without fallback prose or generated-over-authored overwrite.

## Stage 06 — Accessible responsive presentation and navigation

**Prerequisite:** real content renderer, URL resolver, property/graphic system.

**Remaining packages**

- **R06.A:** implement coherent theme/brand/layout primitives, accessible responsive shell and configured headers/footers, using composable design tokens and EFS graphic inheritance.
- **R06.B:** implement authored navigation, Site/Page selection and true same-Page language switching; do not invent links to unavailable variants or infer destination from place/locale.
- **R06.C:** implement mobile/keyboard/assistive-tech control behavior, loading/error/not-found presentation and no-JavaScript/limited-adapter graceful behavior where applicable.

**Exit gate:** desktop and mobile real-browser evidence; keyboard traversal, focus, accessible labels/headings and representative manual accessibility checks; multi-Facet/multi-Site and multilingual navigation; no Spoke or geographic selector. Authoring normal Page content does not require application code changes.

## Stage 07 — SEO, public discovery and asset delivery

**Prerequisite:** stable published addresses, rendered Pages and graphics.

**Remaining packages**

- **R07.A:** canonical, title/description, language, Open Graph and supported structured metadata from the effective published Page and base URL; no fictional translations, locations or canonical addresses.
- **R07.B:** sitemap, robots, `hreflang`, `x-default`, public navigation/index inventory and asset discovery from *real public* representation authority only.
- **R07.C:** safe authored asset delivery, graphic library fallback and explicit suppression; prevent path traversal, private property leakage, orphaned/generated-over-authored asset exposure and incorrect cache/metadata identity.

**Exit gate:** crawler-facing and browser-facing evidence for every accepted locale/visibility class. Unlisted representations are reachable only at their authorized address, noindex/nofollow, and absent from all automatic discovery. A configured base URL change consistently changes all public URL outputs without re-identifying content.

## Stage 08 — Safe contact actions and simple inquiry form

**Prerequisite:** stable UI, URL policy, ports/adapters and privacy contract.

**Remaining packages**

- **R08.A:** safe `mailto:` authoring and rendering, including invalid/malicious destination refusal and honest no-provider behavior.
- **R08.B:** minimal form UX, field validation, accessibility, anti-abuse/rate strategy and privacy/retention behavior; application use case depends on a narrowly defined outbound delivery port.
- **R08.C:** replaceable delivery adapter(s), test fake and failure semantics—do not claim delivery when only accepted/queued/failed; ensure secrets stay outside authored/public data.

**Exit gate:** form happy path, invalid field, malicious input, rate abuse, disabled/misconfigured provider, delivery failure, privacy and retention tests plus browser proof. No form builder, conditional/multistep logic, attachment handling, CRM/ticketing, advanced administration or dormant advanced-form module.

## Stage 09 — Approved small integrations and user actions

**Prerequisite:** stable safe-link contract and adapter boundaries.

**Remaining packages**

- **R09.A:** safe authored external booking/actions and maps/directions links; these are links, not booking inventory, transactions, timezone or geolocation engines.
- **R09.B:** optional privacy-respecting analytics adapter or a documented no-analytics mode, according to the Stage 00 accepted first-release scope; no forced provider account or visitor surveillance.
- **R09.C:** validate integration enable/disable paths, configuration-only provider substitution and no-provider behavior.

**Exit gate:** safe links and refusal tests; provider absence does not break the site; privacy and outbound intent behavior verified. Do not add accounts, payments, scheduling inventory, dashboards, CRM or managed ingress.

## Stage 10 — Installation, content updates, release and recovery lifecycle

**Prerequisite:** sufficiently integrated publishable Solo product and accepted lifecycle contracts.

**Remaining packages**

- **R10.A:** one-Entity standalone establishment with chosen base origin/prefix, default Facet/Site, usable starter Pages, and validated authoring assets/configuration without normal user edits to core application code.
- **R10.B:** distinguish product source, immutable release, independent Installation, authored content updates, upgrades, deployment and rollback; define version/compatibility rules and safe migrations only where needed.
- **R10.C:** implement reproducible release-payload construction/verification and clean-room establishment/update/upgrade/recovery tests; avoid Foundation's Spoke-centric lifecycle and unknown provider assumptions.

**Exit gate:** establish a new Installation from an exact product artifact, change content/branding and configured public base without changing core or Entity ID, rebuild deterministically, perform supported upgrade/rollback or recovery, and refuse invalid/mismatched payloads. No real provider deployment or release publication without separate Owner approval.

## Stage 11 — Cross-system hardening and nonfunctional assurance

**Prerequisite:** completed integrated capability surfaces.

**Remaining packages**

- **R11.A:** security review of manifest, routes, content parser, HTML/Markdown, URL handling, contact, property privacy, assets, dependencies, secrets and error disclosure; address material findings with bounded work orders.
- **R11.B:** accessibility and performance review against Owner-approved thresholds; cross-browser/responsive checks, meaningful user flows and truthful fallback/error UX.
- **R11.C:** resilience, diagnostics, deterministic CI, representative multi-Site/full-locale stress matrices and documentation coherence; remove dead code, needless abstractions and unjustified compatibility machinery rather than carrying defects forward.

**Exit gate:** no unresolved release-blocking security or correctness findings, passing exact-head CI and production-mode browser matrix, checked nonfunctional budgets, independent regression evidence and explained/resolved warning/skip inventory. Defect resolution is actual tested correction, never suppressed assertions.

## Stage 12 — Final end-to-end candidate and documentation acceptance

**Prerequisite:** all mandatory capabilities implemented and independently accepted.

**Remaining packages**

- **R12.A:** complete product acceptance matrix mapped to Solo goals and accepted architecture: simple single-Entity start, expanded Facets/Sites/Pages, full locales, location cascade, public/unlisted case safety, graphics, content authoring, contact, links, SEO and lifecycle.
- **R12.B:** complete first-adopter installation, authoring, customization, preview, accessibility, test, release verification, upgrade/recovery and security/support documentation with only *actually verified* commands.
- **R12.C:** independently inspect exact source commit, release candidate bytes, CI, browser/prod evidence, security/attribution and required notices; report `ACCEPT / REVISE / STOP` and disclose every remaining limitation. No hidden release/deployment permission.

**Exit gate:** a reproducible, verified Solo release candidate; every first-release requirement evidenced; accepted documentation consistent with delivered behavior; Owner-approved quality/support/licensing prerequisites satisfied; and an independent Architect `ACCEPT` on the exact candidate revision.

## Stage 13 — First product release and distribution verification

**Prerequisite:** Stage 12 independently accepted at an exact revision; separate explicit Owner authorization for any release, publication, provider action or deployment.

**Remaining packages**

- **R13.A:** verify the exact immutable product artifact, source provenance, reproducible verification instructions, copyright/attribution, required third-party notices, supported-version policy and vulnerability-reporting arrangements against the accepted release contract.
- **R13.B:** when separately authorized, publish the verified Solo product release without implicitly deploying it, altering an adopter Installation or upgrading any existing deployment.
- **R13.C:** independently verify the published release identity, checksums/digests, provenance, accessibility/security/quality evidence and adopter retrieval/installability from the actual published artifact; correct any discrepancy under a separate bounded authorization.

**Exit gate:** all required release conditions are independently verified; the released product corresponds to the accepted source/artifact and is reproducibly installable. A live production deployment is not claimed without separate Owner authorization and evidence. If the Owner has not authorized a required release, this stage remains outstanding.
