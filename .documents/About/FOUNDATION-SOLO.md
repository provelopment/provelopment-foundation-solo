# Foundation Solo — Product Definition and Intent

> **Status:** Owner-aligned product definition — revision 6, 2026-11-06. Owner-stated principles are distinguished from mechanisms that are deferred to a named later implementation stage; the R00.D physical publication contract is now specified in the Architecture Charter.  
> **Scope:** Product purpose, identity, capabilities, non-goals and guiding principles.  
> **Authority:** This document captures the Owner's stated direction and distinguishes it from proposals still awaiting agreement. It does not authorize implementation.  
> **Companion:** [Architecture](../Architecture/ARCHITECTURE.md) · [Project README](../../README.md) · [Agent instructions](../../AGENTS.md) · [Architect SOP](../SOP/sop-architect.md) · [Implementation SOP](../SOP/sop-implementation.md).

## 1. What Foundation Solo is

**Foundation Solo is an independent, open-source, configuration-first website foundation for one Entity.** An Entity may be any independently identifiable owner of a website and its authored content. Its nature, legal form, purpose, activities, audience and geographic presence are not assumptions of the platform.

Each Foundation Solo Installation represents **exactly one Entity**, with a flexible EFS content hierarchy:

```text
Entity (exactly one)
  └── Facet (one or more)
        └── Site (one or more in each Facet)
              └── Page (one or more in each Site)
                    └── Localized representation (one or more per published Page)
```

An Entity is the permanent owner of the content. A Facet organizes a coherent context or activity of that same Entity. A Site organizes a distinct content context within a Facet; it is **not** a domain, geographic area or locale by definition. A Page has a language-neutral identity. A localized representation is the actual authored language-specific Page content.

A fresh Installation should start with **one default Facet, one default Site, and usable starter Pages**. The Entity can subsequently add Facets, Sites and Pages without adopting another publishing system or creating another Entity.

Foundation Solo is intended to retain EFS's expressive, reusable content model while removing the operational and authoring overhead needed by Foundation's multi-Entity publishing model. **Solo is a first-class product, not a Spoke configured to appear single, a reduced-function mode of Foundation, or a compatibility wrapper around its multi-Entity runtime.**

## 2. What Solo is intended to achieve

1. **Simplicity at entry:** create, configure, author, preview and publish a useful website without studying an operator/tenant topology.
2. **Depth without mandatory complexity:** provide a small default shape that can expand into multiple Facets, Sites, Pages and languages within the single Entity.
3. **Powerful authorship:** allow validated declarative Page sections and safe Markdown, avoiding the need to edit application code for ordinary content.
4. **Reuse without duplication:** permit additive direct-ancestry inheritance of shared Entity-level property values, explicit suppression of inherited contributions, and registered local additions; identified location-record extension is deliberate, not implicit geographic merging.
5. **Independence and portability:** enable a Solo Installation and its authored content to be operated or moved without a required Provelopment service, account, managed ingress or provider.
6. **Accessible, secure publication:** deliver reliable navigation, responsive presentation, valid content, standards-aware metadata, controlled public discovery and clear failures.
7. **Maintainability:** isolate domain concepts from framework, storage, delivery and integrations so the product can evolve without multiplying special cases.

The measure of success is not merely fewer folders or configuration keys. It is **less conceptual and operational burden for the same useful website capabilities**.

## 3. Content model and meanings

### Entity

One persistent content owner per Installation. The Entity carries an explicitly authored technical identifier that is stable and independent of display name, domain, URL, locale, physical place or filesystem path, and the author supplies that identifier at establishment — Solo never derives or guesses it. The accepted identifier grammar, the parent-scoped uniqueness rules and the durable-reference rule are recorded in [ARCHITECTURE.md](../Architecture/ARCHITECTURE.md) §1.1; concrete schema and reference formats remain open.

### Facet

A coherent subcontext **inside the same Entity**, not a separate content owner. Facets make it possible to organize substantial content without imposing a one-website-per-activity or one-domain-per-Facet rule. A simple Installation needs only the default Facet; adding Facets is an intentional authoring action.

### Site

A content context **inside one Facet**. It may concern a service, a collection, a physical premises, a programme, a public-facing initiative or a general context. It is not intrinsically geographic or equivalent to a hostname. One default Site is sufficient to begin, while additional Sites retain their own Page populations.

### Page

A stable, language-neutral identity and authored route within its Site. A Page ID is unique within its own Site and survives label, public-address and translation changes. A Page may carry several localized representations; adding a translation does not create a new Page identity. Pages can include ordinary, nested and specialized content structures, governed by a deliberately bounded schema.

### Localized representation

The only locale-bearing node in the ownership tree. Each published Page has at least one usable representation. A representation is identified **within its Page** by its explicitly authored canonical full region-qualified locale tag; it is never a second Page identity. Locale tags communicate content language and conventions; they do **not** establish physical location, geographic coverage or Site identity.

**One Page, multiple full locales:** a Page such as `about` is owned once. `en-001` (international English), `de-DE` (German as used in Germany), `id-ID` (Indonesian as used in Indonesia), and `zh-Hans-CN` (Simplified Chinese under a deliberately authored locale convention) are **separate authored representations** of that Page. `zh-Hant-TW` is another legitimate deliberate locale choice. Solo does **not** use language-only identifiers such as `en`, `de`, `zh` or `id` for authored or public representations. Canonical full tags may be rendered in lowercase path form; locale keys are addresses, never Page identities or place facts. An `en-001` representation does not make a Site worldwide or change its inherited location.

Browser language preferences may be shorter than authored full locale identifiers. Solo accepts an explicitly configured preference map such as `de` → `de-DE` when matching browser headers, **never** as permission to author a shortened locale or add a `/de/` route. Ambiguous language-only preferences require explicit mapping; no geography or inferred default region is allowed.

For locale-less entry, the agreed order is **supported saved preference → supported `Accept-Language` → authored full-locale default**, with an explicit full-locale URL taking precedence over both preferences. The preferred missing-translation result is a truthful **same-Page** redirect to an expressly permitted public default representation; otherwise not-found. No implicit translation, cross-Page/Site substitution, unlisted fallback or region/location inference is allowed. Precise negotiation mapping/redirect syntax remains a technical contract for approval.

**Visibility and Page routes:** Visibility is authored as a Page-wide default with a per-representation override, including JSON and Markdown parity; unlisted content is **not** an authentication or secrecy guarantee, and a high-entropy URL alone never makes information private. A public localized route is **case-insensitive by default** with a lowercase canonical spelling, but an author may deliberately choose **case-sensitive** matching for an individual localized route, keeping intentional Unicode capitalisation and permitting genuinely distinct public Pages such as `About` and `about`. Unlisted representations always remain exact-case — `SecretAb7` is distinct from `secretab7` and `SECRETAB7` — and can never be reached through a public case-insensitive alias. Locale tags keep conventional canonical syntax in authored data and normalized lowercase URL syntax in every case, without ever altering a case-sensitive Page route. Authors may publish validated Unicode route text and optional localized slugs, so one Page may appear under different spellings in different full locales. Conflicting or ambiguous claims are refused rather than resolved by declaration order. The binding detail lives in [Architecture](../Architecture/ARCHITECTURE.md) §3.2B.

### Publication declarations, identity stability and drafts

**Owner-accepted (`R00.A`).** Each Entity, Facet, Site and Page carries an explicitly persisted technical ID with a canonical lowercase-token grammar (maximum 63 ASCII characters, no repair or case-folding), and identifiers are unique inside their owning scope — Facet within Entity, Site within its own Facet, Page within its own Site — so reusing the token `main` or `home` under different valid parents is legitimate. Identifiers are independent of names, routes, addresses, locales and locations, and changing any of those never mints a replacement ID.

A single explicit, **versioned publication manifest** is the sole authority for the one Entity and the published EFS graph: stable IDs, direct-parent ownership, the published Page set, supported locale representations and one explicit source-selection declaration per representation. Presence of a file, property or route publishes nothing by itself, and no directory scan or `draft` label admits content. Each representation declares at most two named body candidates — preferred JSON and Markdown — and exactly one valid effective source is selected, JSON-first: a genuinely absent JSON candidate is permitted when a valid declared Markdown candidate can be selected, while present-but-invalid or inaccessible JSON fails publication and no selectable effective source fails. Ambiguous or duplicate-claimed candidates, orphan or duplicate declarations, and a second Entity are refused with a diagnostic naming the offender rather than repaired by a default, a first match or a sibling substitution. The binding detail is [Architecture](../Architecture/ARCHITECTURE.md) §1.6.

**Incomplete drafts** may exist only *outside* the published graph. They carry no public route or discovery rights, do not count toward published cardinality and cannot be promoted implicitly; admission always requires a complete, valid declaration that passes the same validation as any other published item.

**Published cardinality:** exactly one Entity, at least one Facet per Entity, at least one Site per published Facet, at least one Page per published Site, at least one valid representation per published Page, and exactly one declared direct parent per child. Declared **unlisted** representations belong to the published graph, because visibility governs discovery rather than membership. A new usable Installation is seeded with one default Facet `main`, one child Site `main` and a starter Page `home` with real authored content in an explicitly selected full locale; these are authoring defaults, never reserved words, route segments or location facts. The logical semantics are accepted here, while manifest and source formats, JSON/Markdown encoding, draft storage and schema versions remain open technical contracts. See [Architecture](../Architecture/ARCHITECTURE.md) §1.1, §1.3 and §1.4 for the recorded rules and conceptual cases.

## 4. Location is an authored cascading EFS property — never routing

**Owner-settled invariant:** `location` is a property of the EFS content model, available at **any of its five levels**, subject to one consistent additive inheritance rule. It is **not** a sixth ownership node, a compulsory physical address, a domain or a geographic routing convention. Explicit location data may be broad or narrow without redefining any node's identity.

For example, one Entity could author the following direct-ancestry location values:

```text
Entity                     currentContext: one fact — Australia
  └── Facet: hotels        inheritance: inherit   currentContext: one fact — Indonesia
        ├── Site: bali     inheritance: suppress  currentContext: one fact — Bali
        │     └── Pages → localized representations: en-001, de-DE, zh-Hans-CN, id-ID
        └── Site: jakarta  inheritance: suppress  currentContext: one fact — Jakarta
              └── Pages → localized representations: en-001, de-DE, zh-Hans-CN, id-ID
```

The example uses **illustrative human-readable values and Site labels, not a final schema or required hierarchy**. Inheritance is **additive**: the Facet therefore carries two facts — Australia originating at the Entity and Indonesia of its own — while each Site declaring `inheritance: suppress` carries only its own. Any node may have **no location property** at all, inherit additively, add local facts, suppress inherited facts, or explicitly extend one identified inherited fact. A representation using `zh-Hans-CN` does not create a location in China, nor does `de-DE` change the authored location to Germany. Language and place remain independent.

- **Inheritance is additive over direct ancestry, never "nearest wins".** A wholly absent property inherits the ancestor result; an explicit declaration combines `inheritance: inherit | suppress` with a current context of zero or more local facts, and the effective collection is inherited-first then locally authored. Nothing is inherited from another Site, unrelated Page or sibling representation.
- **Zero or more complete structured facts.** `location` holds an optional collection of structured facts using distinct typed concepts — world region, administrative region/state, country, locality/city, district, named place, postal components, explicit coordinates — with no universally required field and no requirement for a full postal address. Several facts never imply a single implicit "primary"; a consumer requiring exactly one must select explicitly or refuse ambiguous input.
- **Per-value provenance and exposure.** Each effective value keeps its originating EFS level and its inclusion or suppression trace, and only a validated public projection reaches HTML, metadata, feeds or client code. An internal fact — or an internal inherited field carried into an extension — is never emitted and is never replaced by a public ancestor.
- **One identified-record extension exception.** A node may extend **exactly one** inherited fact identified by a stable reference to that fact's original authored identity, replacing it in place with explicit field additions or replacements: changed fields take local field provenance while unchanged fields keep their origin. Missing, ambiguous, suppressed, cross-branch or circular targets are refused, and matching by place name, free text or coordinates is never attempted.
- **Authored accuracy versus structural validation.** Solo validates declared types, structure, privacy constraints and safe serialization — not real-world truth. No geocoder, external verification service or inference fills a missing value, rewrites an authored fact or selects a route.
- **Location may be authored at any level**, including Page or localized representation; a narrower declaration does not re-identify the owner or rewrite ancestors.
- **No inference in either direction.** A route, base URL, hostname, Site ID, Page slug, physical directory or locale must not supply or guess a location. A location value must not select a Facet, Site, Page, locale, canonical URL, navigation destination, or public base path.
- **No implicit geolocation engine.** Explicitly authored place facts can inform a deliberate display, structured metadata or user-selected external map link, but do not automatically cause geo-IP redirect, geocoding, visitor proximity selection, timezone/DST computation, scheduling or currency detection.
- **No forced geography.** A Site may be purely digital or address-less. An inherited value must be suppressible where inappropriate rather than mistakenly presented as that Site's own physical address.

The logical contract above — governed registration, additive two-field inheritance, per-value provenance and exposure, structured zero-to-many facts, identified-record extension and fail-closed publication — is **Owner-accepted (`R00.C`)**. Precise property keys, physical value types and their encoding, the concrete field catalogue and the physical representation of both declaration elements remain **`R00.D` technical decisions**. Location-as-cascading-data and no-routing-inference are **not** open choices.

## 4A. A configurable Entity base URL

**Owner-settled outcome:** A new Entity can choose its public **base URL at creation**, including an ordinary hostname and an optional authored path prefix. The same EFS content should be publishable under, for example, `https://a.abc.com/something/…` or `https://www.abc.com/another-something/…`. The full chosen address is **configuration**, never the Entity's identity, and may be changed without modifying routing code or reorganizing EFS content.

**Owner-accepted (`R00.B`):** standards-based safe normalization and validation of that address (`B01`); validated Unicode public routes (`B04`); public claims that are case-insensitive by default with explicitly author-selected case-sensitive claims and fail-closed collision refusal (`B05`); condition-appropriate redirects (`B08`); and one authoritative canonical address with optional, explicitly authorized aliases (`B09`). Public links, canonical URLs, assets and discovery metadata always derive from the effective configured address, and a configuration-only address change never alters EFS identity or requires routing redesign.

**Settled (`R00.D`):** the physical declaration and manifest schema, file and field names, locator grammar, content grammar, module catalogue, property and location catalogues, asset profile and route/Unicode literal profile are codified in [Architecture](../Architecture/ARCHITECTURE.md) §6.2–§6.2N. **Still open:** the pinned Unicode data binding and toolchain choices (`R00.E`), and the runtime matcher, HTTP adapters, language selector and cache wiring. External DNS, TLS, host verification and provider routing are separate deployment responsibilities. None of this is implemented, tested or deployed.

## 4B. Public and unlisted Pages
**Owner-accepted granularity and approved names:** Each localized representation has its own effective visibility, and an entire Page can be made unlisted conveniently. The approved authoring names are Page-level **`visibilityDefault`** (defaulting to `public`) and representation-level **`visibility`** (an explicit override). The effective state is the representation's own `visibility`, otherwise the Page's `visibilityDefault`; `visibilityDefault: unlisted` marks all variants unlisted unless a variant explicitly declares `public`. One validated Page/representation declaration is the single authority for both JSON and Markdown sources, with no contradictory second visibility setting inside a content file. Both are **direct inline fields of that same authoritative manifest declaration** — `visibilityDefault` on the Page and `visibility` on each localized representation — never a body-source setting, a central override table or a `publication` wrapper. Only the versioned schema encoding and validation mechanics remain open technical details.


**Owner-settled capability:** Solo retains **unlisted** content. A Page may have localized representations with `public` or `unlisted` discovery behavior. The implementation must support making an entire Page effectively unlisted by keeping all of its representations unlisted, while retaining the finer representation-level capability drawn from EFS.

An unlisted representation is **directly reachable through its exact authorized URL** but is **not** included in generated navigation, sitemaps, language alternates or other discovery. It carries the appropriate `noindex` policy. An unlisted Page is **not** private, password-protected or access-controlled; it cannot be used to store secrets. The versioned schema encoding and validation of the approved `visibilityDefault`/`visibility` declaration remain open technical details — not a reason to remove the behavior.

### Cascading graphics and one generic public graphic library

The public, generic graphic library is a **shared platform resource available throughout the single Entity Installation**, not a second Entity, a Facet, a location-based resource or an ownership tier. An authored graphic selection (logo, icon, banner, illustration, background or image reference where a role is supported) is an EFS property resolved under the same additive contract as every other property: `inheritance: inherit | suppress` plus zero or more local selections, at Entity, Facet, Site, Page and localized-representation levels.

- **Registered roles.** Roles are registered with a defined purpose, accepted asset classes, effective cardinality, applicable levels and allowed contexts; the core candidates `logo`, `favicon`, `banner`, `background` and `socialPreview` are candidates only, and an unregistered role is invalid.
- **No silent override.** For a singular role, inheriting a selection and adding a local one produces a cardinality failure instead of replacing the inherited asset, and an effective maximum always applies to the combined result.
- **Suppression is not emptiness.** `suppress` blocks inherited graphics, while locally authored selections remain valid and effective.
- **Default-deny fallback.** A generic-library default is **not** assumed: automatic fallback is governed per role with **zero roles approved initially**, applies only when no effective authored graphic and no relevant suppression boundary exists, and never rescues an invalid or internal value with a generic substitute.

Generic assets can also be explicitly referenced from any level — that is an authored choice, not fallback. The shared library does not make author-owned artwork globally mutable, and generated public assets do not become the source authority. The finalized role catalogue, asset names, formats, references and any future per-role defaults remain design contracts to establish.

## 5. Desired retained capabilities

The following are **target capabilities to assess and specify for Solo**, drawing on the useful current EFS product; they are not claims that Solo has already implemented them.

| Capability | Intended outcome |
| --- | --- |
| One-Entity EFS ownership | Exactly one Entity; flexible Facets, Sites, Pages and localized representations |
| Cascading properties | Additive inheritance from Entity through localized representation with explicit `inherit`/`suppress` plus local values, effective-cardinality validation, per-value provenance and safe public projection |
| Declarative Page authoring | Validated JSON Pages built from an expansive library of named, versioned, strictly validated Solo modules, with predictable semantics and safe destinations |
| Safe Markdown | Straightforward prose authoring in standard Markdown — CommonMark plus widely recognized extensions — with no Solo-specific syntax and without executable content |
| Multilingual content | Declared language-specific representations, deliberate language selection, truthful fallbacks and metadata |
| Navigation and Site organization | Explicit authored menus, internal links and Site/Page selection; no inferred cross-Site links |
| Configurable base URL | An authored HTTP(S) origin and optional path prefix selected at Entity setup and changeable without altering content identities or routing implementation |
| Brand and presentation | Replaceable identity assets, theme/layout choices, coherent responsive shell and accessible controls |
| Content assets and dictionaries | Owner-scoped authored files, predictable precedence, no silent overwrites |
| SEO and discoverability | Correct titles, descriptions, canonical addresses, Open Graph, structured data when explicitly supported, sitemap, `robots` and `hreflang` |
| Page visibility | Public and unlisted per localized representation; directly reachable unlisted URLs omitted from discovery and marked noindex; no false privacy claim |
| Contact actions and simple inquiry form | Safe authored `mailto:` actions plus a straightforward contact form with validation, proportionate anti-abuse controls and honest delivery status, behind replaceable ports/adapters |
| Other useful integrations | External booking links, maps/directions links and privacy-respecting analytics through small, clear interfaces |
| Installation lifecycle | Standalone establishment, authored updates, deliberate upgrades, reproducible releases and recoverable deployment |
| Validation and diagnostics | Errors name incorrect authored input; malformed declarations fail closed rather than guess or silently fall back |

No requirement for accounts, tenant administration, payments, scheduling inventory, a CMS service or any specific deployment provider is implied by this capability list.

## 6. Explicitly excluded complexity

Foundation Solo **does not contain** Spoke Hubs, Spokes, a hidden implicit Spoke, cross-Spoke navigation, multi-Entity publication, many-to-many publisher/Entity associations, shared-tenant ingress, publisher-specific asset/receiver namespacing, multi-Entity transfer or managed-tenant administration.

It does not need legacy Foundation migration branches or compatibility subsystems: this is a new product designed from a blank slate. Existing Foundation repositories provide **technical reference**, not automatic architecture or source-code inheritance.

A Solo Installation may be deployed by many independent adopters **as separate Installations**, but each Installation continues to own exactly one Entity. This does not transform Solo into a multi-Entity hosting runtime.

**Advanced contact forms are explicitly excluded.** Solo will not contain a built-in or optional advanced form builder, arbitrary conditional or multistep workflows, complex form administration, file attachments, CRM/ticketing pipelines or any comparable advanced-form subsystem — not as a hidden default, a dormant feature or an optional module. Essential validation, spam/rate controls, accessibility and security belong to the **simple** contact form itself and are not advanced-form features.

## 7. Product quality expectations

- **First-class single-Entity architecture:** no unused multi-Entity abstractions preserved merely for historical parity.
- **Strict Hexagonal Architecture is mandatory:** Ports and Adapters with dependencies pointing inward only. The domain core stays framework-, provider- and transport-independent; application use cases sit behind explicit ports; delivery, storage and email/provider details are replaceable adapters wired at a composition root. Future code-bearing work must verify the boundary, not merely assert it.
- **Configuration first:** separate authored configuration/content/assets from platform code; favor unsurprising defaults.
- **Orthogonal concepts:** ownership hierarchy, conceptual functionality and public routing are separately defined, implemented and verified.
- **No inferred authority:** paths do not create identities; properties do not grant publication rights; content files do not self-publish by mere presence.
- **Security by design, planned before code:** security is a proactive, continuous product obligation across planning, implementation, release and maintenance — never a late hardening phase, and never something that can be added after the architecture is fixed. Preventive controls, refusal behaviour and objective tests are designed before the code that would need them, and no security guarantee is claimed that the current repository cannot evidence. The binding acceptance standard is **A1** ([Architect SOP](../SOP/sop-architect.md) §7B), applied against the design boundaries in [Architecture](../Architecture/ARCHITECTURE.md) §5A; confidential reports go to private vulnerability reporting, and there is no separate advisory register.
- **Secure authoring:** validate declarations, sanitize Markdown, classify URLs, protect server-only configuration and refuse malformed or unsafe inputs.
- **Accessibility and responsive behavior:** work from semantic markup, keyboard support, readable layouts and accessible interactions from the beginning.
- **Provider neutrality:** public functionality must not require one hosting vendor or Provelopment-managed infrastructure.
- **Pragmatic, A1-grade extensibility with core stability:** every Solo capability must remain additively extensible through explicit versioned ports, governed product-distributed registries and interchangeable adapters, without reinterpreting already valid content or redefining established EFS and module contracts. Unknown or unauthorized contracts fail closed, and removing or substituting an extension must leave unrelated functionality intact. This is an essential requirement of **all** functionality, not an optional per-feature add-on; the binding implementation and proof obligations are recorded in [Architecture](../Architecture/ARCHITECTURE.md) §6C.6.
- **Proportional engineering:** use focused checks during development and meaningful end-to-end evidence for material behavior, without redundant expensive gates.

## 8. What is decided and what remains proposed

### Owner-stated direction captured here

- This is a **new, independent** open-source product named **Foundation Solo**.
- There is **exactly one Entity** per Solo Installation.
- One Entity has **one or more Facets**; each Facet has **one or more Sites**; each Site has **one or more Pages**.
- The default experience begins small and the hierarchy remains expandable.
- There are **no Spoke Hubs or Spokes**.
- Preserve the useful power of EFS while eliminating unnecessary overhead and complexity.
- Start from a blank slate; do not preserve an architectural mechanism merely because Foundation used it.
- **Location is a five-level cascading EFS property** (`R00.C`). Entity `Australia` → Facet `Indonesia` → Site `Bali`/`Jakarta` illustrates **additive** inheritance: a Facet carries both the Entity fact and its own unless it suppresses, and a Site that suppresses carries only its own. Zero or more structured facts are permitted, exactly one inherited fact may be extended by identified reference, and `en-001`, `de-DE`, `zh-Hans-CN`, `id-ID` never imply geography. Routing and location never infer one another.
- **Unlisted Pages are retained**, with directly reachable but undiscoverable localized representations; this does not confer privacy. A Page-wide unlisted authoring convenience is required, with per-representation exceptions retained.
- **Strict Hexagonal Architecture (Ports and Adapters) is a binding decision**, with inward-only dependencies, a provider-independent domain core and verifiable architectural boundaries.
- **The planned contact capability includes safe `mailto:` actions and a simple contact form**, registered as the `interaction.contact` JSON module; advanced contact forms are excluded rather than optional. The bounded contact and integration contract — including its field limits, rate policy, idempotency, privacy-notice and no-retention decisions — is now **specified** in [Architecture](../Architecture/ARCHITECTURE.md) §6B; only provider selection, provider retention terms and deployment remain later work.
- **A configurable base URL is selected at Entity creation**, including an authored origin and optional path prefix. A subsequent base URL change must not alter EFS identities or require code redesign.
- **Entity, Facet, Site and Page carry explicitly authored technical identifiers** with a canonical lowercase-token grammar (maximum 63 ASCII characters), parent-scoped uniqueness — Facet within Entity, Site within its own Facet, Page within its own Site — and durable references that survive label, address, translation and base-URL changes; a localized representation is keyed within its Page by its full locale tag.
- **A new Installation is seeded with the default Facet `main`, Site `main` and starter Page `home`** holding real authored content in an explicitly selected full locale; these are authoring defaults, not reserved words, route segments or location facts.
- **One explicit, versioned publication manifest is the sole publication authority**, with explicit source-integrity binding for every published representation and fail-closed refusal of duplicate, orphan, incomplete, second-Entity and undeclared claims; incomplete drafts may exist only outside the published graph.
- Published Page representations use full locale tags such as `en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`, never bare language-only tags; locale-less entry precedence and preference for explicit same-Page default fallback are Owner-decided.
- **Public routes are explicitly authored addresses and never identities** (`R00.B`): case-insensitive by default, case-sensitive only where an author selects it for an individual localized route, and always exact-case for unlisted representations. Validated Unicode routes, optional localized slugs and one authored real public landing Page for the locale-less entry are accepted; locale segments normalize independently of Page-route case. Any visitor can choose an actually published language without an account and without geographic determination, and an explicit full-locale URL always wins.
- **Apache License 2.0 is Owner-approved** as Solo's open-source license.
- The active work system contains exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md` inside `.documents/WIP/` — remaining committed stages, one next candidate, the sole issued order and optional ideas respectively — with only ROADMAP → NEXT → ACTIVE progressing.
- The project is locally self-contained in `00.foundation-solo`, with standalone project documentation exclusively inside `.documents/` except the root entry files `README.md` and `AGENTS.md`.

### Details still proposed or deferred to a later stage

- Concrete schema artifacts, manifest file/field and locator formats, editable source layout, JSON/Markdown authoring encoding and draft storage are now **specified** for the accepted identity, default-model and publication semantics in [Architecture](../Architecture/ARCHITECTURE.md) §6.2A–§6.2C, §6.2D and §6.2M; producing the actual `.schema.json` files and validators is `R00.E` implementation work.
- Physical encoding of the accepted address/route/locale contract — the manifest and field schema for route claims, aliases and locale preference mappings — is **specified** in §6.2J. Still open: the pinned Unicode data binding and toolchain (`R00.E`), and the implemented matcher, redirect and cache behaviour and rendered language selector. The configurable base URL, route-case and locale outcomes themselves are not open.
- The **initial installed module catalogue** (twenty-two exact `1.0.0` identifiers with closed input schemas, defaults, child rules and composition limits), the **exact Markdown profile** and extensions, Markdown/JSON content parity rules and the versioned encoding of the approved `visibilityDefault`/`visibility` declaration are now **specified** in §6.2D–§6.2F and §6.2B. The capability families and the mainstream-Markdown rule themselves are accepted (`R00.D`, §3.1A–§3.1B); no module is installed or promised by being named, and how much of Foundation's current renderer is adopted or redesigned remains a later implementation choice.
- The physical property catalogue and encoding, the structured location record and extension-patch schema, the physical representation of the two declaration elements and the graphic role catalogue with formats and cardinalities are now **specified** in §6.2G–§6.2I. The logical property, location and graphic contracts — governed registration, additive two-field inheritance, per-value provenance, validated public projection, structured facts and default-deny graphic fallback — remain Owner-accepted as `R00.C`.
- The **contact and integration contract** — the two distinct contact actions, the closed three-field simple form registered as the `interaction.contact` module, the `SubmitSimpleContact` use case over the reserved `ContactDelivery` port, the seven truthful outcomes, the fail-closed abuse and privacy boundaries, the bounded static booking and directions links and the settled no-analytics default — is now **specified** in [Architecture](../Architecture/ARCHITECTURE.md) §6B. No form code, provider, credential, schema or endpoint exists.
- The theme, asset and dictionary contracts. The establishment, lifecycle and release contracts are now **Owner-approved** documentary decisions in [Architecture](../Architecture/ARCHITECTURE.md) §6C; only their implementation remains future work.
- Implementation of the conditionally selected §6A toolchain — language, framework, runtime, package layout, build and test tooling — all of whose conformance proof is still unexecuted. GitHub **private vulnerability reporting is enabled** for the public repository, and the lifecycle, support-response, accessibility, performance and attribution/notice decisions are **Owner-approved** as documentary contracts in §6C; what remains is their implementation and evidence, together with the factual pre-release prerequisites — the Security Maintainer designation, the published supported-version table, verified attribution and any actually required third-party `NOTICE` and SBOM. **Apache-2.0 is Owner-approved.**

**No source implementation, feature-completion claim, deployment or release is authorized by this document.**

## 9. Independent operating governance

Solo's own [Architect SOP](../SOP/sop-architect.md) and [Implementation SOP](../SOP/sop-implementation.md) control authorized work, and the Architect SOP is the single owner of the planning lifecycle. Exactly four files under [WIP](../WIP/): [`plan-roadmap.md`](../WIP/plan-roadmap.md) (the still-pending implementation stages), [`plan-next.md`](../WIP/plan-next.md) (at most one bounded candidate), [`workorder-active.md`](../WIP/workorder-active.md) (the one issued work order, or none) and [`plan-outlook.md`](../WIP/plan-outlook.md) (optional ideas only, never a requirement or delivery backlog). There is no separate work-order directory and no report archive: exactly one current execution report lives at [`Reports/execution-report.md`](../Reports/execution-report.md) — outside the four-file WIP folder, overwritten on every agent run and published whenever that is safe, with Git history preserving earlier revisions. These files do not substitute for the actual approved architecture or implementation evidence.
