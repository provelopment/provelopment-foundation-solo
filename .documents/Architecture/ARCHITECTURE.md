# Foundation Solo — Architecture Charter

> **Status:** Owner-aligned architecture, 2026-11-04 revision 5. Explicit Owner choices are accepted; remaining mechanisms marked **recommended** or **open** are not implementation authority.  
> **Relationship:** [Product definition](../About/FOUNDATION-SOLO.md) is the purpose and scope authority; this document defines the architectural model and invariant boundaries.  
> **Approval:** Only the explicitly identified Owner-stated constraints are settled. Specific mechanisms labeled **proposed** or **open** require a recorded decision before implementation.  
> **Scope:** Product architecture, conceptual functionality, public routing, data ownership, validation and verification approach. This is not a work order.

## 1. Primary architectural decision

Foundation Solo is a standalone application foundation for **exactly one Entity per Installation**. Unlike the multi-Entity Foundation, there is no separate publishing hierarchy. The content model is first class and is **not** an adapter around an implicit Spoke:

```text
Solo Installation (one autonomous software/deployment lifecycle)
  └── Entity [exactly 1; permanent content owner]
        ├── Facet [1..* per Entity]
        │     └── Site [1..* per Facet]
        │           └── Page [1..* per Site]
        │                 └── Localized Representation [1..* per published Page]
        └── Facet [optional additional Facet]
              └── ...
```

**Cardinality invariant:** exactly one Entity; at least one Facet, at least one Site in each Facet, at least one Page in each Site, and at least one representation for each published Page. Every child has exactly one direct parent. Published validity and authoring drafts are distinct concerns, and the accepted `R00.A` decision in §1.3 keeps them apart: incomplete drafts may exist **only outside** the published graph, so a draft never weakens published validation and never becomes published merely by being present.

### 1.1 Identity and ownership

| Node | Identity and role | Must not be confused with |
| --- | --- | --- |
| Installation | One autonomous deployed/adopted copy of Solo | Entity ID, hostname, content ownership |
| Entity | One permanent owner of all authored content in the Installation | Domain, URL, account, legal type, geographic context |
| Facet | Distinct context inside that Entity; Facet ID unique within Entity | Separate Entity, routing prefix, domain |
| Site | Content context within Facet; Site ID unique within Facet | Domain, physical location, locale, country code |
| Page | Language-neutral content identity; Page ID unique within Site | URL alone or a translation |
| Localized representation | Authored content for one Page and one locale | New Page, geographic identity or market |

**Identity, grammar and stability (`R00.A`, accepted).** Display labels may change without changing permanent content identity, and no route, filename, hostname, public address, locale or provider account may mint identity implicitly. Paths may be used as deliberate locators, never as the definition of identity. Each Entity, Facet, Site and Page carries an explicitly persisted technical ID. For these four identity-bearing levels only, an ID is a canonical lowercase ASCII token matching `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` with a maximum of **63 ASCII characters** — no leading or trailing hyphen, no consecutive hyphens, no whitespace, underscore, uppercase alias, non-ASCII character, slash, dot or percent escape, and no silent case-fold normalization. An invalid authored ID is refused with a specific diagnostic; it is never repaired, normalized or derived from another value. This grammar governs **IDs only**; public Page-route tokens keep their separately accepted public/unlisted casing behavior (§3.2B).

**Uniqueness scope (`R00.A`, accepted).** Exactly one Entity ID exists per Installation; Facet IDs are unique within that Entity; Site IDs are unique within their **own** Facet; Page IDs are unique within their **own** Site. Reusing a token such as `main` or `home` under different valid parents is legitimate because the owning scope differs, and different levels may legitimately carry the same token. Duplicate authored IDs inside one scope are invalid. No ID-alias mechanism is established; any future aliasing needs separate Owner acceptance and must not create ambiguous identity claims. There is exactly one owner, so no cross-Entity ownership resolution, registry, synchronization or tenancy permission model is necessary.

**Durable reference (`R00.A`, accepted).** Authoring and publication declarations persist those IDs together with explicit parent ownership. Changing labels, public addresses, the set of localized representations, the base URL or EFS properties never mints a replacement ID. A parent move, identity change or deletion is an explicit, later-approved migration with reference checks; no filename move, route change or implicit inference silently reparents or recreates a node. The concrete reference format and any migration tooling remain open. A localized representation is identified **within its Page** by its explicitly authored canonical full region-qualified locale tag under §3.2, and is never a second Page identity.

### 1.2 Separate concerns, always

1. **Architecture:** Entity/Facet/Site/Page/representation identity, cardinality, ownership, inheritance and installation lifecycle.
2. **Conceptual functionality:** authoring, rendering, localization, presentation, properties, visibility, discovery and integrations.
3. **Routing:** explicit mapping of a public host/path request to a selected Facet/Site/Page/representation within the already-known Entity.

An implementation design must describe and verify these independently. Public route strings are **addresses, not definitions** of identity, location or ownership.

### 1.3 Publication authority, source integrity and fail-closed validation

**Minimum default model (`R00.A`, accepted).** The author explicitly supplies the single Entity ID; Solo never guesses it from a name or hostname. A new usable Installation is seeded with exactly one default Facet (ID `main`, label `Main`), exactly one child Site (ID `main`, label `Main Site`) and a starter Page (ID `home`, label `Home`) carrying at least one real authored localized representation in an explicitly selected supported full locale. The repeated IDs are valid because the scopes differ. These are initial authoring defaults, **not** reserved words, required route segments, geographic labels or new ownership levels, and labels stay editable without an identity change. No locale is inferred or hardcoded here: the Site/default-locale configuration and public Home routing belong to `R00.B`. The starter Page is a usable authored Page — never an automatically published undeclared source file and never an empty placeholder — and further starter Pages are not mandated.

**Published graph (`R00.A`, accepted).** The published graph holds validated, directly reachable representations, **including declared unlisted ones**, because visibility governs discovery rather than membership. Partially authored content may exist only as a non-published draft under the rule below, never as an exception to published cardinality.

**Drafts outside the published set (`R00.A`, accepted).** Authors may retain incomplete content or work-in-progress declarations **outside** the authoritative published set. Such drafts do not count toward published cardinality, have no public route or discovery rights, and are never promoted implicitly by file presence, a directory scan or a label such as `draft`. Explicit admission to the published set requires a complete, valid declaration that passes exactly the same validation as any other published item. Draft storage locations and draft metadata format remain `R00.D`.

**One authoritative publication declaration (`R00.A`, accepted).** A single explicit, versioned publication manifest is the **sole** authority for the one Entity and the published EFS ownership graph: stable IDs, direct-parent associations, the published Page set, supported locale representations and explicit source references. Only explicitly admitted, complete entries publish; a file, property, route declaration or a label such as `draft` never publishes anything by itself. The logical semantics are accepted here, while the physical file format, field names, locator syntax, content-section schema and manifest schema version remain for `R00.D` and adjacent contracts.

**Source integrity (`R00.A`, accepted).** Every published localized representation binds explicitly to an authored content-source reference belonging to its declared Page. That reference must resolve to exactly one existing, valid and permitted authored source inside the eventual validated source boundary; implicit path matching, first-file-wins, search-order fallback and sibling source substitution are all refused. Two distinct published claims must not silently share one authored source, and any intended sharing or generated content needs its own later explicit contract. Undeclared files never publish; declared missing, unsafe or ambiguous sources fail validation; generated output and public assets never become the authored manifest authority.

**Fail-closed refusal specification (`R00.A`, accepted).** Validation must reject, with an actionable diagnostic that names the affected declaration or ID and the reason: a second Entity; missing or invalid IDs; duplicate IDs in the relevant parent scope; a missing, unknown or ambiguous parent; an orphan Site, Page or representation; an illegal cross-parent binding; an empty published Facet, Site or Page; a missing published representation; a duplicate locale within one Page after canonical normalization; undeclared content publication; an absent, duplicate-claimed, unsafe or ambiguous authored source reference; and publication inferred from a directory or filename. Errors are never hidden by inventing a default parent, choosing the first match, reassigning identity or redirecting to a sibling, and no diagnostic exposes server-private paths or secrets.

### 1.4 Conceptual identity and publication examples

These examples are **conceptual specification evidence only**: they are not executed validation, not a manifest schema and not a final file format. Schema, locator syntax, tooling and runtime validation belong to `R00.D`, `R00.E` and Stage 02.

A minimal valid default shape, with an Entity ID the author supplies explicitly:

```text
Entity  <explicitly authored id>        label: any display name
  └── Facet main                        label: Main
        └── Site main                   label: Main Site
              └── Page home             label: Home
                    └── representation <selected full locale>    e.g. en-001
```

An expanded shape that reuses tokens legitimately, because each owning scope differs:

```text
Entity acme
  ├── Facet main
  │     ├── Site main     → Pages home, about
  │     └── Site bali     → Pages home, about     # same ids, own Site scope
  └── Facet hotels
        └── Site main     → Pages home, rooms     # same ids, own Site scope
```

Refusal examples — each states required behaviour, not a passing test:

| Invalid declaration | Required refusal |
| --- | --- |
| A second Entity in the manifest | Exactly one Entity is permitted per Installation |
| `Site` id `Main`, `home_page`, `-main`, `main--site`, `main site`, or a 64-character token | Not a valid identity token; no repair, case-fold or truncation is attempted |
| Two Facets with id `main` in one Entity | Duplicate id within the same owning scope |
| A Site whose declared Facet id does not exist | Unknown parent; no default parent is invented |
| Two representations of one Page claiming the same source | Duplicate source claim; no first-file-wins resolution |
| A representation whose declared source is missing or outside the permitted boundary | Declared source is absent or not permitted |
| A published Page with no representation | A published Page requires at least one representation |
| `de-DE` and `de-de` as two representations of one Page | Duplicate locale after canonical normalization |
| A source file present in the tree but absent from the manifest | Undeclared source does not publish |

## 2. The property system is a core EFS capability

The proposed property resolution model follows the most useful current EFS principle: **one predictable cascade over the five content levels**.

```text
Entity → Facet → Site → Page → Localized Representation
  least specific                          most specific
```

For each supported property key, the nearest explicitly authored declaration on the target's direct ancestry controls the effective value. If no node declares it, it is absent. The proposed semantics are:

- **Absent key:** inherit from the nearest ancestor that declares it.
- **Explicit suppression:** deliberately remove an inherited effective value; a `null` marker is a *candidate* syntax, not a finalized schema choice.
- **Composite property:** treat as one typed declaration unless a specific property contract expressly authorizes field-level merging. No arbitrary deep merge.
- **Scope:** never inherit from a sibling Site, Facet, Page or representation; never invent another Entity.
- **Provenance:** diagnostics should identify the declaration level and whether the effective value was inherited, local or suppressed.
- **Public projection:** an inherited value does not become public merely through inheritance. Explicit authorization/visibility rules govern data exposed in HTML, structured data, client code and feeds.

Examples of useful property families are presentation and identity assets, authored titles/descriptions, contact channels, **location/place facts**, opening-hours facts, external action destinations and other explicitly typed authored facts. The Owner requires location to be supported as a cascading property at all five EFS levels. The exact property field catalogue, types, allowed structured values, privacy handling and suppression encoding remain technical contracts to adopt, not inferences from the example.

### 2.1 Non-negotiable location rule

**Location is authored EFS property data and has no routing authority.** This is an architectural invariant for all five levels, not only Sites.

- Place association, physical address, coordinates and other geospatial or premises-related facts—where supported—are **optional explicit property declarations**.
- A Site can represent a physical premises or a nonphysical context. Absence of place data is valid and does not mean a global or worldwide region.
- No mandatory `Location` owner node, geographic Site tier, region/country hierarchy or location selector exists in Solo.
- **The router must not read location, address, coordinates, country, place name, hours or geospatial properties to select any Facet, Site, Page, locale, canonical URL or navigation destination.**
- **The property resolver must not inspect URLs, routes, hostnames, path prefixes, language subtags or Site names to infer a location or fill a missing location value.**
- A Site ID such as `jakarta` is merely an authored identifier. An `id` that looks like a country code is not parsed as one. The locale `en-ID` is language metadata, not a place claim.
- Explicitly authored place facts may be intentionally rendered on Pages or in permitted structured metadata, but this display is separate from identity, route selection and publication authority.
- No automatic location detection, geo-IP redirect, reverse geocoding, derived timezone, DST scheduler, currency, nearest-Site selector or regional route is silently enabled.

**Owner's accepted conceptual example — values/identifiers illustrate the cascade, not an adopted schema:**

```text
Entity                 location: Australia
  └── Facet: hotels     location: Indonesia
        ├── Site: bali     location: Bali
        │     └── Page: home
        │           ├── en-001 representation (inherits Bali)
        │           ├── de-DE representation (inherits Bali)
        │           ├── zh-Hans-CN representation (inherits Bali)
        │           └── id-ID representation (inherits Bali)
        └── Site: jakarta  location: Jakarta
              └── Page: home
                    ├── en-001 representation (inherits Jakarta)
                    ├── de-DE representation (inherits Jakarta)
                    ├── zh-Hans-CN representation (inherits Jakarta)
                    └── id-ID representation (inherits Jakarta)
```

A more specific Page/representation may explicitly override the effective location, but a locale never supplies a default location. If a location-independent Site should not display the inherited Entity location, it must be able to explicitly suppress the inherited value through a deliberately validated rule. The current Foundation's `null` suppression is a useful reference; Solo's exact syntax remains unapproved.

A request such as `https://example.org/bali/en-001/home` **must not derive** Bali from `/bali`, `bali` as a Site ID, the hostname or the `en-001` tag. If Bali is displayed as location, the source must be an explicit property in the selected EFS ancestry. Changing `location` alone must not change route resolution or canonical URL. Conversely, changing base URL must not change `location`.

**No implicit multi-premises hierarchy:** multiple Site contexts or an eventual structured/multi-value location property may represent different premises, but the encoding and expected rendering require later acceptance. Neither imposes geographic Site identity.

### 2.2 Location and localization remain independent

Entity, Facet, Site and Page are language-neutral. The localized representation is the only locale-bearing content node. For example, an Entity may publish a `fr-FR` Page representation with an address in Indonesia without that address changing to France, without creating a France Site, and without changing its route by geography. A region subtag is part of a locale tag, **not a geographic locator**.

## 3. Conceptual functionality — what Solo must be able to do

### 3.1 Author Pages safely and flexibly

Offer two complementary authored Page formats:

- **Declarative JSON Pages** validated by a bounded schema; sections are data, not arbitrary executable components. Proposed initial candidate section families from EFS: hero, prose, media, gallery, actions, callout, cards, features, columns, steps, statistics, quote, table, FAQ, list, divider and contact form.
- **Safe Markdown** for straightforward authored prose and text-forward Pages, with a predictable title/heading contract, classified link destinations and no unrestricted executable HTML.

The final section vocabulary and versioning contract must be selected before implementation. JSON/Markdown overlap and precedence must be explicit and must never vary by file iteration order.

### 3.2 Localize only what is localized

**Owner-accepted:** A Page has one language-neutral identity; only its localized representations have a locale. Foundation Solo uses **full, explicitly authored BCP 47 locale tags with a region** in its published authoring contract. A bare language code such as `en`, `de`, `id` or `zh` is **not** a valid Solo representation ID or public locale path key. Script subtags can be included when needed.

```text
Facet: hotels / Site: bali / Page: about
  ├── en-001       International English
  ├── de-DE        German as used in Germany
  ├── id-ID        Indonesian as used in Indonesia
  └── zh-Hans-CN   Chinese, Simplified script, China convention

Additional deliberate representation example: zh-Hant-TW
```

These tags specify an **authored linguistic/formatting convention**, not the Entity's, Facet's, Site's or Page's physical location, intended geographic coverage, address, routing origin or identity. `en-001` is international English, **not** a worldwide Site or an inherited location. An Indonesian Site may publish `de-DE` while its effective `location` remains Bali. The choice of a country subtag is never made by inspecting the EFS location property, hostname, Site ID or URL.

**Owner-accepted locale entry policy:** For an explicit supported full-locale URL, use that locale; it must not be overridden by a cookie or browser header. For entry with no locale segment, choose, in order: (1) a supported saved visitor preference, (2) a supported browser `Accept-Language` preference, (3) the explicitly authored default full locale. Unknown or unsupported explicit locale paths must fail rather than masquerade as a different locale. **No geo-IP or location inference.**

**Owner-accepted robustness rule, with exact storage/configuration schema pending:** Supported full tags match after canonical normalization. A browser header may contain a bare tag such as `de`; this is *input preference syntax*, not an authored or published Solo locale. It may resolve only through an **explicitly configured preference mapping** such as `de → de-DE`. If no supported exact match or authored mapping exists, continue to the next preference and finally the default. Never guess between `de-DE`, `de-AT` and `de-CH` or infer one from a location property. The browser is redirected to the actual full-locale public address, not served an implicit `/de/` URL.

**Owner-accepted missing-translation policy:** Only another **authored representation of the exact same Page in the same Site and Facet** may answer a missing translation, and not if the target is unlisted. The Owner prefers fallback to the declared default representation rather than an immediate not-found when that fallback is valid. No automatic translation, cross-Page substitute, cross-Site substitute or invented `hreflang` is allowed.

**Owner-accepted fallback behavior, with exact configuration keys and redirect-status implementation pending:** Define a full-locale default for the applicable Site; a Page may explicitly opt out with a strict/not-found policy or nominate another *public* same-Page fallback. If a requested representation is missing, perform a **temporary redirect** to the actual authorized fallback representation's canonical full-locale URL; do not render a language other than the one in the URL. If no acceptable public representation exists, return not-found with a diagnostic. A public representation must never be silently replaced with a different existing representation. Validation should refuse a declared fallback target that does not exist or is unlisted.

**Owner-accepted addressing distinction:** Full locale tags are authored with canonical casing (e.g., `zh-Hans-CN`), and the public locale path segment uses a deterministic lowercase representation of that full tag (e.g., `zh-hans-cn`). The URL segment is an address, **not** Page identity or a location. Example configured bases:

```text
https://a.abc.com/something/en-001/about
https://a.abc.com/something/de-de/about
https://a.abc.com/something/id-id/about
https://a.abc.com/something/zh-hans-cn/about
```

**For implementation contract acceptance:** The exact route grammar, normalization/error handling, locale preference storage and temporary redirect status are still open technical choices. The principles above are not permission to start source work.

- Language switching stays on the exact same Page and Site; unavailable variants are not advertised as available.
- A declared default locale must be a supported full tag. Missing fallbacks never promote an unlisted representation.
- `lang`, canonical, `hreflang`, sitemap and language alternates must describe the real public representation; no absent/unlisted variants are advertised.
- Dictionaries may localize labels and navigation without introducing localization at Entity, Facet, Site or Page level.

### 3.2A Unlisted representations and Page-wide convenience

**Owner-accepted behavior:** Every localized representation has an effective `public` or `unlisted` visibility state. An entire Page can be made unlisted conveniently, and an individual representation may be explicitly public or unlisted even when the rest of the Page differs. Visibility is a discovery decision, not authentication.

**Owner-accepted authoring authority and precedence:** The Page declaration has an optional `visibilityDefault`, defaulting to `public`; a representation may declare a `visibility` override. These are the selected configuration names, subject to coherent schema validation and implementation proof. The effective state is the representation's own explicit value, otherwise the Page default. Setting `visibilityDefault: unlisted` marks all variants unlisted **unless the author explicitly sets a variant public**. The validated Page/representation declaration is the one authority for both JSON and Markdown content. There must be no contradictory second visibility setting inside a content file.

| Page default | Representation override | Effective state |
| --- | --- | --- |
| omitted (`public`) | omitted | `public` |
| `public` | `unlisted` | `unlisted` |
| `unlisted` | omitted | `unlisted` |
| `unlisted` | `public` | `public` |

An unlisted representation serves at its **exact authorized URL** and carries `robots: noindex, nofollow`; it is absent from generated navigation, sitemap, `hreflang`, language-switch options, `x-default` and any other automatic discovery. It is **not private**: no secrets or access-controlled material may rely on `unlisted`. An explicitly authored direct link can intentionally point to a known unlisted URL only under the eventual safe-link contract; the platform does not automatically advertise it. Missing-translation fallback must never select an unlisted representation. Behavior must be equivalent for JSON and Markdown content.

**Open implementation details:** The versioned JSON/Markdown declaration schema, whether a fully unlisted Home Page is allowed when the public base has no other entry, route-token grammar, and source validation behavior remain to be designed and separately authorized. The visibility field placement and override precedence above are approved.


### 3.2B Owner-accepted URL case policy: public versus unlisted

**Visible/public representations:** Path matching for publicly listed Pages is **case-insensitive** for the supported route character set. Canonical public URLs use lowercase path segments, including normalized full-locale tags (`de-DE` in authoring becomes `/de-de/`). Alternate spellings such as `/DE-DE/ABOUT`, `/de-DE/About` and `/de-de/about` must identify the **same public representation** and use the same canonical URL. The recommended mechanism is to redirect noncanonical public spellings to the lowercase canonical address; the specific safe redirect status and encoded-path rules are open technical details. This applies after the configured public base URL is resolved; the base-path canonicalization policy must be deterministic and consistent with public links, sitemap and metadata.

**Unlisted representations:** Authored **Page-route tokens** are case-sensitive and case-preserving, so an unlisted token `xY7AbC` is not equivalent to `xy7abc` or `XY7ABC`. These characters must never be lowercased or case-folded as part of route matching, redirects, source lookup or URL generation. Full-locale identifiers, including unlisted URLs' locale segments, remain normalized by the same BCP 47 locale rules, so case sensitivity applies to the unpredictable Page-route part rather than treating `de-DE` as a different language from `de-de`. All variants of an unlisted route must remain outside automatic navigation, sitemap, language-alternate and other generated discovery. An unlisted URL is not secret or protected simply because it is hard to guess.

**Mixed visibility:** A Page's `visibilityDefault` and a representation's `visibility` override determine the **effective visibility of that representation**. Case-insensitive public matching must not run as a blanket whole-path lowercase step before visibility is resolved. The route validator must refuse any public case-folded alias that could capture an unlisted exact-case path in the same effective route/locale scope, and it must refuse ambiguous public/public or unlisted/unlisted claims. No manifest ordering or fallback may decide such a collision. Public aliases must never accidentally serve or disclose unlisted content.

**Illustrative cases, not an adopted path grammar:**

```text
Public:   /something/DE-DE/About     -> /something/de-de/about (same public Page)
Public:   /something/de-de/about     -> /something/de-de/about
Unlisted: /something/de-de/Xa9B2     -> exact authored unlisted Page
Unlisted: /something/de-de/xa9b2     -> not the same Page; reject or 404 unless independently declared
```

### 3.3 Present one coherent public website

A configurable presentation shell should provide accessible header/navigation/footer, branding, a responsive design system, language controls, internal links, optional Site navigation, content sections and appropriate asset handling. Additional Facets or Sites are *content organization*, not automatic visitor-facing switchers. Public navigation must be authored or explicitly derived under a defined, opted-in policy.

### 3.3A Cascading graphic properties and the shared public library

Solo has **one global, generic, public graphic library** supplied by the platform and available to every Facet, Site, Page and representation in the Installation. It is **not** an EFS ownership node, a publisher or an alternative Entity. It must be provider-neutral and replaceable/maintainable under a clear source-asset versus generated-asset contract.

An Entity-authored graphic selection is a **cascading EFS property**, resolved using exactly the same nearest-explicit-ancestor and explicit-suppression rules as other properties. Supported graphics can include logos, favicons, banners, backgrounds, media and icon roles as they are deliberately specified. A role's effective selection is determined in this order:

1. Explicit local graphic declaration on the localized representation, Page, Site, Facet or Entity, whichever is nearest to the requested node.
2. Otherwise an inherited declaration from its direct EFS ancestry.
3. Otherwise an **optional, role-specific generic-library default**, only if such a default is explicitly approved for that role.
4. An **explicit suppression** means *no asset for that role*, never an automatic fall-through to a generic default.

Generic-library files may also be referenced deliberately by any node without duplicating their source bytes across Facets/Sites. Entity-authored graphic files remain Entity-owned; shared generic assets are read-only platform resources. Never infer graphic selection from location or route, and never allow private/unlisted authored content or metadata to leak through automatic public asset discovery. Asset references must validate safe paths and deterministic namespace/source ownership. **The graphic-role registry, file formats, reference syntax and individual default choices remain open**, not implicitly inherited from historical Foundation.

### 3.4 Support useful public functionality

Targets include SEO/discovery, public and unlisted Pages, optionally authored contact facts, contact/inquiry integration, links to maps/directions or external booking services, and configurable privacy-respecting analytics. Integration contracts should be small ports/adapters, independent of the content model; no CRM, account management, payments, commerce, scheduling inventory or external service is required for basic publication.

**Owner-decided contact scope — included.** Solo's planned contact capability includes **both** (a) safe `mailto:` action handling — validated author-supplied addresses and correctly encoded URIs where the visitor's own mail client performs the sending — and (b) a **simple contact form** with a bounded field set, validation, safe server-side handling, proportionate anti-abuse controls and honest delivery/receipt status. Both are implemented as an application use case plus explicit ports with replaceable adapters; no provider SDK, credential or mail-transport detail may appear in EFS domain objects or authored Page sections.

**Explicitly excluded — advanced contact forms.** Solo will **not** contain a built-in or optional advanced form builder, arbitrary conditional or multistep workflows, complex form administration, file attachments, CRM/ticketing pipelines or any comparable advanced-form subsystem; none of these may exist as a hidden default, dormant option or optional module. Essential validation, spam/rate controls, accessibility and security for the **simple** form are baseline requirements of that simple form, not advanced-form features. Exact fields, delivery adapter/provider, abuse controls and retention configuration remain later technical design decisions under a separate Owner-approved contract.

A form must not claim successful delivery when it has merely validated a request or handled an offline demonstration. Server-side policies, secrets and receipt/delivery evidence are separate from authored Page content.

### 3.5 Preserve author ownership and portability

The single Entity owns its configuration, content and source assets. Generated/public output is derived and should not be edited as the authority. Upgrading a platform release is different from changing content. The system should support repeatable setup, source-verifiable upgrades and recoverability without a particular deployment provider.

## 4. Routing — deliberately separate from the content model

### 4.1 Configurable base URL as an authored address

**Owner-settled behavior:** At initial Entity establishment, the author chooses an effective public **base URL** consisting of a valid HTTP(S) origin and an **optional path prefix**. E.g.:

```text
https://a.abc.com/something
https://www.abc.com/another-something
```

Both are valid *forms* for discussion; no particular hostname or prefix is baked into the product. The chosen base URL is a **public addressing configuration**, not a permanent EntityID, a Facet/Site identity, a location or a localization rule. A later legitimate base URL change must be possible by editing this configuration **without moving authored content, minting another Entity, changing its EFS IDs or restructuring routing code**. The router must work for a bare origin and a prefixed origin through the same implementation.

The URL contract must validate scheme, hostname, port when applicable, normalized and safe prefix segments, reserved namespaces, collisions with Page routes, relative/absolute link handling and correct path boundary matching. Exact aliases, redirects, canonical origin and environment/preview behavior still require accepted technical contracts. External DNS, host verification and provider routing are deployment operations, separate from configuring Solo's public-address model.

### 4.2 Proposed request-resolution pipeline

```text
incoming request URL (host + path)
  → validate the configured public base URL / separately approved aliases
  → remove exactly the configured safe base-path prefix (if any)
  → match one explicit content route within this one Entity
  → identify authored Facet + Site + language-neutral Page
  → select or resolve an authorized localized representation
  → read and validate its authored source and effective EFS properties
  → render content, links and metadata using the effective canonical base URL
```

No Spoke or Spoke Hub resolution exists. The Entity is already known by the Installation; **neither location properties nor Entity/Site names** are router selectors. Route declarations are validated, not discovered by arbitrary file iteration or first matching array entry.

### 4.3 Simple defaults, explicit expansion

A default Facet and Site should support concise URLs without forcing internal IDs into the public route. Multiple Facets and Sites can have explicit, collision-checked route declarations; adding one must not unpredictably change existing published URLs. **An EFS node's ID is not necessarily its public path segment.** A path label resembling a country, city or locale is never a geographic assertion.

Examples are illustrative, not adopted URL grammar:

```text
base = https://a.abc.com/something
  /en-001/about       → https://a.abc.com/something/en-001/about
  /id-id/contact      → https://a.abc.com/something/id-id/contact

base = https://www.abc.com/another-something
  /en-001/about       → https://www.abc.com/another-something/en-001/about
  /id-id/contact      → https://www.abc.com/another-something/id-id/contact
```

The same Page/representation IDs may be published at either chosen base URL without identity changes. Locale placement and representation fallback are independent technical decisions. Unknown/conflicting routes must fail clearly, never borrow another Site or guess from `location`.

### 4.4 Canonical URLs, asset addresses and unlisted discovery

Canonical, Open Graph, language alternates, generated links, assets and sitemap entries must be composed with the **configured effective base URL**. A configured path prefix must be preserved **exactly once**; it must not be accidentally dropped or doubled. Server routing and generated metadata must use the same public-address authority.

Only declared public representations may be advertised through generated discovery. An unlisted representation still serves on its exact authorized URL, with `noindex`, but is absent from generated navigation, sitemap and `hreflang`. Unlisted is not a protected/private URL.

Physical location properties can supply deliberate visible facts or permitted structured data, but must **never determine the base URL, canonical URL, route claim, Page selection or locale**.

## 5. Deployment, release and software boundaries — strict Hexagonal Architecture

A Solo **product release** is an independently reviewable platform artifact; a Solo **Installation** is one Entity's configured adoption of that artifact; a **deployment** is the runtime serving it. These identities must not collapse into one. Initial setup, content updates, product upgrades and deployment/rollback should be separate controllable operations.

**Strict Hexagonal Architecture (Ports and Adapters) is a binding Owner-decided contract, not an aspiration.** It is mandatory for all Solo application code; it is not a preference an implementation may trade away for convenience. The required shape is:

```text
Domain core:        identities, hierarchy, properties, Page schema, locale and route contracts; pure rules and invariants
Application:        use cases that orchestrate domain behaviour behind explicit inbound/outbound ports
Ports:              interfaces/contracts declared at the domain/application boundary
Adapters:           inbound (request handling, UI, content loading) and outbound (rendering, storage, email transport, integrations, deployment)
Composition root:   wires concrete adapters to the inner contracts
```

- **Dependencies point inward, only.** The domain core and application policies must not import web-framework code, database or storage models, HTTP transport, provider SDKs, filesystem details or delivery adapters. Adapters depend on the inner contracts; the core never depends on an adapter or on a runtime/provider detail.
- **The core owns the decisions.** EFS ownership, localization, visibility, properties, safe route claims and domain validation live in the domain core even when a framework or datastore is used elsewhere.
- **Ports are explicit and minimal.** An inbound adapter invokes an application port; an outbound adapter implements a port the application uses. External types, payload shapes and provider details are translated at the adapter boundary and never leak inward.
- **Delivery is replaceable.** HTTP, static rendering, content loading, email transport and external integrations are adapters: changing one must not require rewriting EFS domain decisions.
- **Verification is required in code-bearing work.** Enforce dependency direction with explicit import/boundary checks where the toolchain allows it, test behaviour through ports with fake adapters, and add focused real-adapter integration evidence where relevant. No architecture-test or build claim may be made while this repository remains documentation-only.

This contract does **not** select a framework, language, directory tree, application server, persistence model or hosting platform, and it does not justify a ceremonial abstraction layer with no boundary purpose. The simplest implementation that provably preserves these boundaries is preferred.

Solo must have no runtime dependency on the existing Foundation, its demonstration installation, Spokes, Provelopment root governance files or an external managed tenant router. Reference code may be evaluated and deliberately reused only after checking compatibility, provenance, licensing and fit for Solo's clean architecture.

## 6. Validation and acceptance approach

These are **proposed verification obligations**, not passing tests:

| Area | Evidence the implementation should supply |
| --- | --- |
| Exactly one Entity | A valid one-Entity Installation succeeds; a second Entity is refused |
| Expandable EFS | Multiple Facets and Sites render correctly; duplicate IDs are refused in the correct scope |
| Page/locale integrity | One Page has multiple real representations; no cross-Page/Site fallback |
| Property cascade | Nearest declaration wins; absence inherits; explicit suppression works; provenance is correct |
| Location independence | Route results unchanged when place properties change; locale and location never determine each other; Australia → Indonesia → Bali/Jakarta cascade demonstrated |
| Routing integrity | Host-only and prefixed base URLs configured at creation, later config-only URL change, correct prefix handling/canonicals; unknown/colliding claims fail; public mixed-case requests identify one lowercase canonical Page, and case aliases cannot capture an unlisted route |
| Safe authoring | Malformed JSON/Markdown, unsafe URLs and invalid assets are refused or safely rendered |
| Public projection | Private or suppressed inherited facts never leak into HTML, metadata, scripts or indexes |
| SEO and discovery | Canonicals, sitemap, unlisted `noindex` and `hreflang` reflect real published representations; no unlisted discovery leakage |
| Graphic cascade and shared library | One platform public graphic asset is reachable throughout Solo; local/inherited graphic properties override appropriately; explicit suppression prevents fallback; content owner assets remain correctly sourced |
| Visitor quality | Accessible keyboard/mobile behavior and actual served-page smoke evidence |
| Lifecycle | Independent release vs authored content; deterministic build; deployment/recovery plan |
| Hexagonal boundary | Dependencies point inward only; the domain core carries no framework, transport, storage or provider imports; use cases run through explicit ports with replaceable adapters wired at a composition root |
| Contact scope | The included safe `mailto:` action and the **simple** contact form validate input and report honest delivery status; no advanced contact-form subsystem is present in any form |

At implementation time, tests must exercise both the ordinary one-Facet/one-Site path and an expanded multi-Facet/multi-Site case. A green unit gate is not a substitute for observing the user-facing rendered result.

## 7. Decisions to make before an implementation work order

1. **Authoring and publication contract — decided by `R00.A`, format still open:** accepted and recorded in §1.1, §1.3 and §1.4 are the technical ID grammar and 63-character limit, the parent-scoped uniqueness rules, durable references, the explicitly supplied Entity ID, the seeded `main`/`main`/`home` defaults, drafts outside the published graph, the single versioned manifest as sole publication authority, source integrity and fail-closed refusal. **Still open:** the concrete manifest file and field/locator schema, the editable source layout, JSON/Markdown authoring encoding, draft storage and the encoding of the accepted identity/publication semantics.
2. **Route grammar and base URL validation:** single authored base URL at establishment (origin + optional prefix), safe normalization, config-only changes, default Site paths, full-locale path segment and preference negotiation per §3.2, alias policy, discovery metadata and multi-Site collisions. **Owner accepted:** public paths are case-insensitive with lowercase canonicals; unlisted Page-route tokens remain exact-case; collisions must fail; short browser preference tokens require explicit mappings; same-Page default fallback is preferred. **Still technical:** specific URL grammar, redirect codes, path encoding, mapping storage and collision algorithm.
3. **Typed property contract:** exact schema, explicit suppression, safe projection and structured/multiple location facts. **Location as a five-level cascade** is already Owner-decided; its encoding is not.
4. **Page formats, visibility and presentation:** approved section catalog, JSON/Markdown precedence, **Owner-approved Page-level `visibilityDefault` with representation-level `visibility` override**, design tokens, shared generic public library, cascading graphic roles and dictionary precedence. Field placement and behavior are decided; schema encoding, valid graphic roles and defaults remain technical choices. Unlisted serving/discovery behavior itself is Owner-accepted.
5. **Integration scope:** the **included** contact scope is the safe `mailto:` action and the **simple** contact form; **advanced contact forms are excluded** (see §3.4). Remaining technical choices are the field set, delivery adapter/provider, anti-abuse controls, retention, and the booking/map-link and analytics defaults.
6. **Tooling/lifecycle:** framework, runtime, validation/test gates, release/upgrade strategy, **Apache-2.0 (Owner approved), copyright/attribution, third-party notices, and the release/support policy around the already-enabled GitHub private vulnerability reporting channel** — intake is enabled for the public repository, while triage ownership, response expectations, coordinated disclosure, a supported-version/support policy and any required third-party notice remain open — and independent Solo repo setup (the parent `/00.foundation-solo/` ignore rule is **committed and verified effective** in the enclosing repository at its recorded local revision, which tracks no Solo paths; the parent remains outside the Solo agent's write boundary, and its remote publication is a separate Owner/parent-operator concern, not a Solo bootstrap gate).

**No undefined implementation mechanism should be treated as approved merely because it appears as an example in this document.**

## 8. Governance, repository and evidence boundary
**New document naming invariant:** the only live WIP files are `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, and `plan-outlook.md`. Files beginning `plan-` can inform decisions but never authorize source work; `workorder-active.md` only authorizes changes with an exact Owner-issued scope and matching handoff. Do not generate task-specific plan or workorder files to accumulate history.

**Repository standard files:** The Owner-approved Apache-2.0 `LICENSE`, Git hygiene dotfiles, and recognized `.github/` community files are narrow publishing/tooling placement exceptions to `.documents/`. They contain no second source of architectural or operational authority. GitHub **private vulnerability reporting is enabled** for this public repository; copyright/attribution, third-party notices, triage ownership, response expectations, coordinated disclosure and a supported-version/support policy remain outstanding and must be settled before any release.


The Solo repository and its own source are the future first-hand implementation evidence. All standalone Solo documentation belongs under `00.foundation-solo/.documents/`, except the required root `README.md` and `AGENTS.md`. Decisions and approved execution tasks must be recorded in that same project, not delegated to historical Foundation or Provelopment root documentation.

The initial files are intentionally self-contained. Solo operating policy resides in [`SOP/sop-architect.md`](../SOP/sop-architect.md) and [`SOP/sop-implementation.md`](../SOP/sop-implementation.md); the only authorized live work surface is [`WIP/workorder-active.md`](../WIP/workorder-active.md), with the remaining committed stages in [`plan-roadmap.md`](../WIP/plan-roadmap.md), at most one candidate in [`plan-next.md`](../WIP/plan-next.md) and optional ideas only in [`plan-outlook.md`](../WIP/plan-outlook.md). `WIP/` contains **exactly** these four files. No separate `Work-Orders` directory and no archive of reports may accumulate. The single canonical execution report sits deliberately **outside** WIP at [`Reports/execution-report.md`](../Reports/execution-report.md): it is overwritten on every run, including failed and read-only runs, is the only permitted current execution-report file, and is neither a WIP file nor a source of authority. That one report and the four WIP files are disjoint sets.

`provelopment-root`, `provelopment-foundation` and `provelopment-web-1` may be consulted for background, but they are **not normative dependencies** of Foundation Solo and must not be edited as part of Solo implementation without an explicit separate authorization.
