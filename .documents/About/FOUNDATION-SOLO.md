# Foundation Solo — Product Definition and Intent

> **Status:** Owner-aligned product definition — revision 5, 2026-11-04. Owner-stated principles are distinguished from still-proposed technical mechanisms.  
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
4. **Reuse without duplication:** permit shared Entity-level facts and narrower overrides via a predictable property cascade.
5. **Independence and portability:** enable a Solo Installation and its authored content to be operated or moved without a required Provelopment service, account, managed ingress or provider.
6. **Accessible, secure publication:** deliver reliable navigation, responsive presentation, valid content, standards-aware metadata, controlled public discovery and clear failures.
7. **Maintainability:** isolate domain concepts from framework, storage, delivery and integrations so the product can evolve without multiplying special cases.

The measure of success is not merely fewer folders or configuration keys. It is **less conceptual and operational burden for the same useful website capabilities**.

## 3. Content model and meanings

### Entity

One persistent content owner per Installation. The Entity's ID, if represented as a technical identifier, is stable and independent of display name, domain, URL, locale, physical place or filesystem path. Its exact encoding and initial-assignment mechanism have not yet been selected.

### Facet

A coherent subcontext **inside the same Entity**, not a separate content owner. Facets make it possible to organize substantial content without imposing a one-website-per-activity or one-domain-per-Facet rule. A simple Installation needs only the default Facet; adding Facets is an intentional authoring action.

### Site

A content context **inside one Facet**. It may concern a service, a collection, a physical premises, a programme, a public-facing initiative or a general context. It is not intrinsically geographic or equivalent to a hostname. One default Site is sufficient to begin, while additional Sites retain their own Page populations.

### Page

A stable, language-neutral identity and authored route within its Site. A Page may carry several localized representations; adding a translation does not create a new Page identity. Pages can include ordinary, nested and specialized content structures, governed by a deliberately bounded schema.

### Localized representation

The only locale-bearing node in the ownership tree. Each published Page has at least one usable representation. Locale tags communicate content language and conventions; they do **not** establish physical location, geographic coverage or Site identity.

**One Page, multiple full locales:** a Page such as `about` is owned once. `en-001` (international English), `de-DE` (German as used in Germany), `id-ID` (Indonesian as used in Indonesia), and `zh-Hans-CN` (Simplified Chinese under a deliberately authored locale convention) are **separate authored representations** of that Page. `zh-Hant-TW` is another legitimate deliberate locale choice. Solo does **not** use language-only identifiers such as `en`, `de`, `zh` or `id` for authored or public representations. Canonical full tags may be rendered in lowercase path form; locale keys are addresses, never Page identities or place facts. An `en-001` representation does not make a Site worldwide or change its inherited location.

Browser language preferences may be shorter than authored full locale identifiers. Solo accepts an explicitly configured preference map such as `de` → `de-DE` when matching browser headers, **never** as permission to author a shortened locale or add a `/de/` route. Ambiguous language-only preferences require explicit mapping; no geography or inferred default region is allowed.

For locale-less entry, the agreed order is **supported saved preference → supported `Accept-Language` → authored full-locale default**, with an explicit full-locale URL taking precedence over both preferences. The preferred missing-translation result is a truthful **same-Page** redirect to an expressly permitted public default representation; otherwise not-found. No implicit translation, cross-Page/Site substitution, unlisted fallback or region/location inference is allowed. Precise negotiation mapping/redirect syntax remains a technical contract for approval.

**Visibility and Page routes:** Public representations have lowercase canonical URL paths and accept case-insensitive spellings. Unlisted representations remain exact-case for their authored Page-route tokens, including unpredictable mixed-case route strings. For example, `SecretAb7` is distinct from `secretab7` and `SECRETAB7` when unlisted. Locale tags retain conventional canonical syntax in authored data and normalized lowercase URL syntax in all cases; case-sensitive unlisted tokens must not be altered by the public path normalizer. A conflicting public case-folded alias and unlisted exact route must be rejected, not resolved by route order. Visibility is authored as a Page-wide default with a per-representation override, including JSON and Markdown parity. Unlisted content is **not** an authentication or secrecy guarantee; high-entropy URLs alone do not make sensitive information private.

## 4. Location is an authored cascading EFS property — never routing

**Owner-settled invariant:** `location` is a property of the EFS content model, available at **any of its five levels**, subject to one consistent inheritance and override rule. It is **not** a sixth ownership node, a compulsory physical address, a domain or a geographic routing convention. Explicit location data may be broad or narrow without redefining any node's identity.

For example, one Entity could author the following direct-ancestry location values:

```text
Entity
  location: Australia
  └── Facet: hotels
        location: Indonesia        # overrides Australia within this Facet
        ├── Site: bali
        │     location: Bali       # overrides Indonesia for this Site
        │     └── Pages
        │           └── localized representations: en-001, de-DE, zh-Hans-CN, id-ID
        └── Site: jakarta
              location: Jakarta   # overrides Indonesia for this Site
              └── Pages
                    └── localized representations: en-001, de-DE, zh-Hans-CN, id-ID
```

The example uses **illustrative human-readable values and Site labels, not a final schema or required hierarchy**. The Entity and each Facet/Site/Page/representation may have **no location property**, inherit from its direct ancestor, explicitly override it, or—under a validated suppression rule—decline an inherited value. A representation using `zh-Hans-CN` does not create a location in China, nor does `de-DE` change the authored location to Germany. Language and place remain independent.

- **Nearest explicitly authored declaration on direct EFS ancestry wins.** There is no inheritance from another Site, unrelated Page or sibling representation.
- **Location may be authored at any level**, including Page or localized representation; a narrower declaration does not re-identify the owner or rewrite ancestors.
- **No inference in either direction.** A route, base URL, hostname, Site ID, Page slug, physical directory or locale must not supply or guess a location. A location value must not select a Facet, Site, Page, locale, canonical URL, navigation destination, or public base path.
- **No implicit geolocation engine.** Explicitly authored place facts can inform a deliberate display, structured metadata or user-selected external map link, but do not automatically cause geo-IP redirect, geocoding, visitor proximity selection, timezone/DST computation, scheduling or currency detection.
- **No forced geography.** A Site may be purely digital or address-less. An inherited value must be suppressible where inappropriate rather than mistakenly presented as that Site's own physical address.

Precise property keys, value types, multiplicity, validation, suppression syntax and public/private projection remain **technical design choices requiring approval**. The location-as-cascading-data and no-routing-inference principles are **not** open choices.

## 4A. A configurable Entity base URL

**Owner-settled outcome:** A new Entity can choose its public **base URL at creation**, including an ordinary hostname and an optional authored path prefix. The same EFS content should be publishable under, for example, `https://a.abc.com/something/…` or `https://www.abc.com/another-something/…`. The full chosen address is **configuration**, never the Entity's identity, and may be changed without modifying routing code or reorganizing EFS content.

An implementation must validate supported HTTP(S) origins, prefixes and conflicts, and derive public links, canonical URLs, assets and discovery metadata from the effective configured public address. Precise normalization, aliases, redirects, the public route grammar and deployment host wiring remain separately designed; "whatever base URL" does not bypass URL safety or external DNS/provider requirements.

## 4B. Public and unlisted Pages
**Owner-accepted granularity and approved names:** Each localized representation has its own effective visibility, and an entire Page can be made unlisted conveniently. The approved authoring names are Page-level **`visibilityDefault`** (defaulting to `public`) and representation-level **`visibility`** (an explicit override). The effective state is the representation's own `visibility`, otherwise the Page's `visibilityDefault`; `visibilityDefault: unlisted` marks all variants unlisted unless a variant explicitly declares `public`. One validated Page/representation declaration is the single authority for both JSON and Markdown sources, with no contradictory second visibility setting inside a content file. Only the versioned schema encoding, field placement and validation mechanics remain open technical details.


**Owner-settled capability:** Solo retains **unlisted** content. A Page may have localized representations with `public` or `unlisted` discovery behavior. The implementation must support making an entire Page effectively unlisted by keeping all of its representations unlisted, while retaining the finer representation-level capability drawn from EFS.

An unlisted representation is **directly reachable through its exact authorized URL** but is **not** included in generated navigation, sitemaps, language alternates or other discovery. It carries the appropriate `noindex` policy. An unlisted Page is **not** private, password-protected or access-controlled; it cannot be used to store secrets. The versioned schema encoding and validation of the approved `visibilityDefault`/`visibility` declaration remain open technical details — not a reason to remove the behavior.

### Cascading graphics and one generic public graphic library

The public, generic graphic library is a **shared platform resource available throughout the single Entity Installation**, not a second Entity, a Facet, or a location-based resource. An authored graphic selection (e.g., logo, icon, banner, illustration, background or image reference where a role is supported) is an EFS property capable of being defined and inherited or overridden at Entity, Facet, Site, Page and localized-representation levels. Descendants may deliberately use their inherited selection, override it or suppress it.

If a role has an approved generic-library default, this is the **lowest-priority fallback**, below the Entity/Facet/Site/Page/representation cascade; it never overrides an authored property or an explicit suppression. Generic assets can also be explicitly referenced from any level. The shared library does not make author-owned artwork globally mutable, and generated public assets do not become the source authority. The precise graphic role catalogue, asset names, formats, allowed references and fallback choices remain design contracts to establish.

## 5. Desired retained capabilities

The following are **target capabilities to assess and specify for Solo**, drawing on the useful current EFS product; they are not claims that Solo has already implemented them.

| Capability | Intended outcome |
| --- | --- |
| One-Entity EFS ownership | Exactly one Entity; flexible Facets, Sites, Pages and localized representations |
| Cascading properties | Inheritance from Entity through localized representation, nearest explicit value wins, explicit suppression, provenance and safe public projection |
| Declarative Page authoring | Validated JSON Pages with a bounded catalog of reusable sections, predictable semantics and safe destinations |
| Safe Markdown | Straightforward prose authoring without executable content |
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
- **Secure authoring:** validate declarations, sanitize Markdown, classify URLs, protect server-only configuration and refuse malformed or unsafe inputs.
- **Accessibility and responsive behavior:** work from semantic markup, keyboard support, readable layouts and accessible interactions from the beginning.
- **Provider neutrality:** public functionality must not require one hosting vendor or Provelopment-managed infrastructure.
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
- **Location is a five-level cascading EFS property**. Entity `Australia` → Facet `Indonesia` → Site `Bali`/`Jakarta` illustrates nearest-declaration precedence; `en-001`, `de-DE`, `zh-Hans-CN`, `id-ID` do not imply geography. Routing and location never infer one another.
- **Unlisted Pages are retained**, with directly reachable but undiscoverable localized representations; this does not confer privacy. A Page-wide unlisted authoring convenience is required, with per-representation exceptions retained.
- **Strict Hexagonal Architecture (Ports and Adapters) is a binding decision**, with inward-only dependencies, a provider-independent domain core and verifiable architectural boundaries.
- **The planned contact capability includes safe `mailto:` actions and a simple contact form**; advanced contact forms are excluded rather than optional. Exact fields, delivery adapter and abuse controls remain later technical design decisions.
- **A configurable base URL is selected at Entity creation**, including an authored origin and optional path prefix. A subsequent base URL change must not alter EFS identities or require code redesign.
- Published Page representations use full locale tags such as `en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`, never bare language-only tags; locale-less entry precedence and preference for explicit same-Page default fallback are Owner-decided.
- **Apache License 2.0 is Owner-approved** as Solo's open-source license.
- The active work system contains exactly `workorder-active.md`, `plan-next.md` and `plan-outlook.md` inside `.documents/WIP/`, with short-, medium- and long-term outlook and mandatory pruning.
- The project is locally self-contained in `00.foundation-solo`, with standalone project documentation exclusively inside `.documents/` except the root entry files `README.md` and `AGENTS.md`.

### Proposed details awaiting review

- Exact identifiers, defaults and schema formats for Entity, Facet, Site, Page and representation.
- Exact base URL and route grammar, normalization, default Site treatment, canonical-host and alias behavior. The configurable base URL outcome itself is not open.
- JSON Page section catalogue, Markdown parity, validation and versioned encoding of the approved `visibilityDefault`/`visibility` declaration, and how much of Foundation's current renderer is adopted or redesigned.
- Property catalogue, structured/multiple location value schema, suppression syntax, publication privacy and validation. Location-as-data and inheritance across all five EFS levels are already decided.
- The theme, asset, dictionary, integration, establishment and release contracts.
- Chosen programming language, framework, runtime, test tooling, technical repository/release policies, copyright/attribution and private security-reporting setup. **Apache-2.0 is Owner-approved.**

**No source implementation, feature-completion claim, deployment or release is authorized by this document.**

## 9. Independent operating governance

Solo's own [Architect SOP](../SOP/sop-architect.md) and [Implementation SOP](../SOP/sop-implementation.md) control authorized work. Exactly three files under [WIP](../WIP/): [`workorder-active.md`](../WIP/workorder-active.md) (one active work order, or none), [`plan-next.md`](../WIP/plan-next.md) (one candidate), and [`plan-outlook.md`](../WIP/plan-outlook.md) (pruned short/medium/long outlook, including documentation maintenance). There is no separate work-order directory and no report archive: exactly one current execution report lives at [`Reports/execution-report.md`](../Reports/execution-report.md) — outside the three-file WIP folder, overwritten on every agent run and published whenever that is safe, with Git history preserving earlier revisions. These files do not substitute for the actual approved architecture or implementation evidence.
