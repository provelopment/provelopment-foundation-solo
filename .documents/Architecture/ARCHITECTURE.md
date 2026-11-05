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

Resolution is **not** a nearest-value-wins rule. The Owner-accepted logical property contract (`R00.C`) is an explicit, additive declaration model over the governed property system.

**Governed and extensible registry (C01).** The property registry supplies built-in and explicitly registered custom definitions. Each definition carries a stable property identity, a type, allowed EFS levels, cardinality, validation, a maximum public visibility and any permitted specialized operations. Unknown or conflicting definitions fail validation. A custom definition may introduce typed authored facts, but it can never bypass the platform's mandatory safety, publication or ownership invariants.

**Two independent declaration elements (C02, C03).** A wholly absent property declaration at a node implicitly inherits the ancestor's effective result. An explicitly declared property carries two independent elements: `inheritance: inherit | suppress` and `currentContext: none | zero-or-more typed local values` — `none` belongs only to current context, never to inheritance. The effective result is the direct ancestor's effective values when inheritance is `inherit` (otherwise an empty inherited set) **plus** the current local contributions, ordered inherited-first and then locally authored. At the Entity root the inherited input is empty, and a narrower valid declaration may add local values after a higher-level suppression. An explicit `inherit` + `none` declaration differs in provenance from total absence but yields the same values.

- **Direct ancestry only:** inheritance traverses the selected node's direct parent chain inside the single Entity — never a sibling Site, Facet, Page or representation, never another Entity and never a geographic or route-derived hierarchy. There is no automatic cross-branch fallback.
- **Effective cardinality and uniqueness:** a registered maximum applies to the effective collection, not merely to local contributions. Duplicate equality and uniqueness are defined per registered property; an exceeded maximum, an ambiguity or a uniqueness conflict fails validation — never a silent first/last win, silent deduplication or arbitrary recursive object merge. Ordered effective values retain their original declaration identity and origin.
- **Per-value provenance (C04):** the domain-resolved result preserves each value's EFS origin, its inclusion or suppression decision and, where the contract permits, record-field provenance. Private source locations, author-only metadata and sensitive value contents must not leak through public diagnostics.
- **Validated public projection (C05):** each value carries an exposure classification inside its registered type's maximum public exposure, and inheritance preserves that classification. Renderers, metadata, generated feeds and client serialization consume only the validated **public projection**, never an unfiltered internal result; an internal value is neither emitted nor replaced by a public ancestor merely because the projection omitted it. `internal` is not secrecy for material present in a public Git repository or in delivery infrastructure configuration.
- **Fail closed (C10):** a failed property, cross-property, security or publication check blocks any published graph; incomplete drafts may exist only outside it. There is no automatic correction, tolerant acceptance of unknown fields, fallback or partial invalid publication, and an unlisted representation is still published, directly reachable and fully validated rather than secret.

Examples of useful property families are presentation and identity assets, authored titles/descriptions, contact channels, **location/place facts**, opening-hours facts, external action destinations and other explicitly typed authored facts. The Owner requires location to be supported as a cascading property at all five EFS levels. The exact property field catalogue, physical types and encoding and the physical representation of the two accepted declaration elements remain `R00.D` technical contracts to adopt, not inferences from the example.

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

**Owner-accepted conceptual example — the two-field additive contract, not an adopted schema:**

```text
Entity                     currentContext: one fact — Australia
  └── Facet: hotels        inheritance: inherit   currentContext: one fact — Indonesia
        ├── Site: bali     inheritance: suppress  currentContext: one fact — Bali
        │     └── Page: home
        │           └── en-001, de-DE, zh-Hans-CN, id-ID representations
        └── Site: jakarta  inheritance: suppress  currentContext: one fact — Jakarta
              └── Page: home
                    └── en-001, de-DE, zh-Hans-CN, id-ID representations
```

Facet `hotels` therefore has **two** effective facts — Australia (originating at the Entity) and Indonesia (originating at the Facet) — because inheritance is additive rather than a replacement of the ancestor value. Each Site declares `inheritance: suppress` and therefore has only its own fact; a Site that wanted the wider association would declare `inheritance: inherit` instead and keep all three. Suppression blocks inherited contributions and never forbids locally authored ones, and it does not mean the node must display nothing. A Page or representation may author its own facts, suppress the inherited ones, or leave the property wholly absent to inherit. A locale never supplies a location, and no syntax shown here is final: the physical encoding of both logical elements remains `R00.D`.

**Structured location fact contract (C06).**

- `location` is an ordinary registered EFS property whose current context is a collection of **zero or more complete structured location facts**. Zero facts are valid, and no country, city, street, postal code or coordinate is universally required; partial but meaningful authored metadata is permitted and a full postal address is never demanded.
- Supported concepts are **distinct semantic types** — world region, administrative region/state/province, country, locality/city, district or local area, named place or area, postal components and explicit coordinates — never one ambiguous universal `region` field. A governed custom typed location metadata field is permitted under C01.
- A fact may describe a geographic association or a physical premises; neither is inferred from address detail, and no missing value is ever inferred from a URL, route, EFS ID, locale, hostname, country, coordinates, a neighbouring record or another language representation.
- **Whole records inherit.** The ordinary C03 rule carries complete ancestor facts unchanged and local facts are appended as independent entries. Merely authoring `city: Perth` locally must not silently attach a `country: Australia` from an unrelated ancestor fact; a descendant that wants only its own facts declares `inheritance: suppress`.
- **Identified-record extension.** A current-context location operation may extend **exactly one accessible inherited fact**, identified by a stable reference to that fact's original authored identity, together with explicitly specified field additions or replacements. It replaces that specific fact **in place** in the effective ordered collection instead of appending a duplicate; the original identity remains the target, changed fields receive local field provenance and unchanged fields retain their original origin. The reference must resolve through the selected direct ancestry: a missing, ambiguous, suppressed, cross-branch or circular target is invalid, and if suppression excludes the target then extending it is invalid. Matching by geographic name, free text or coordinates is never performed.
- **Whole-record exposure.** Exposure classification applies to each structured location value. An internal fact, and an inherited internal field carried into an explicit extension, stays outside the public projection; extending or relabelling a record never silently promotes an internal field to public, and per-field visibility is not assumed from per-value visibility. A newly authored, independently validated public fact — or a future expressly accepted finer-grained projection contract — is required instead.
- **Facts and truth.** Authors are responsible for factual geographic correctness; Solo validates declared types, structural integrity, privacy constraints and safe serialization, not real-world truth. A future optional external verification module would need its own Owner decision and a replaceable port, and could never determine public routes or silently rewrite authored facts.
- **No implicit primary.** A collection of several facts never yields a single implicit "primary" location; a downstream output that requires exactly one must use an explicitly governed selection or refuse ambiguous input. The physical record identifier format, extension-patch encoding and registry field catalogue remain `R00.D` choices, while a logical record reference must stay stable across routing and location-text changes.

A request such as `https://example.org/bali/en-001/home` **must not derive** Bali from `/bali`, `bali` as a Site ID, the hostname or the `en-001` tag. If Bali is displayed as location, the source must be an explicit property in the selected EFS ancestry. Changing `location` alone must not change route resolution or canonical URL. Conversely, changing base URL must not change `location`.

**No implicit multi-premises hierarchy:** several Site contexts, or several structured location facts on one node, may describe different premises without creating a geographic Site tier or a mandatory location level. A collection of facts is ordinary property data — its physical encoding and expected rendering remain later choices — and neither imposes geographic Site identity.

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

**Owner-accepted optional locale-less Page entry routes (`B03`).** The configured base entry above is required; **in addition**, an author **may** explicitly declare locale-less entry routes for individual Pages, for example `/about` relative to the configured base prefix. Such entries are optional public entry addresses for those Pages only: never required for every Page, never a substitute for a Page's full-locale canonical URLs, and never a locale-less canonical representation.

- **Identity before language.** Each declared locale-less entry route identifies exactly one stable language-neutral Page, within its actual Entity/Facet/Site parent scope, **before** any language selection. Ownership is never inferred from the entry spelling, a translation, a Site name, a location property or a filesystem position.
- **Real destination after selection.** Once a locale is selected by the accepted order (supported saved preference → explicitly mapped `Accept-Language` → applicable authored default), the entry redirects to that Page's **actual authored, published, public full-locale canonical route**, which may use a localized slug. A destination is never built by inserting a locale prefix, translating a word or guessing a similar slug.
- **Missing representation.** Where the Page has no representation in the selected locale, only its accepted `B07` same-Page, public-only fallback applies; a Page configured as strict/not-found stays strict, and a different Page or Site is never substituted.
- **Visibility.** A locale-less entry never exposes, selects or advertises an effectively unlisted representation and never invents a public one. Where no authorized public destination exists, the result is not-found.
- **Case and Unicode.** The entry route is an authored route claim, so it passes the same `B04` NFC normalization, controlled decoding, declaration-level `B05` matching policy and collision validation as every other claim. There is no separate universal case rule for locale-less entries.
- **Collisions.** Every locale-less entry belongs to the complete prepublication claim ledger and must be unambiguous against localized canonical routes, aliases and public/unlisted claims. Neither declaration order nor a convenient first match may choose a destination.
- **HTTP and caching.** Locale-less entry selection uses the accepted `B08` temporary redirect — ordinarily `302` for safe navigation requests — together with the `B12` protection against shared caching of preference-dependent responses; redirect targets derive only from validated routing authority.
- **Independence.** Declaring an entry creates no Entity, Facet, Site, Page, translation or content-source authority. EFS identities and the single versioned publication manifest remain decisive.

**Conceptual cases — specification, not executed tests.** A Page with stable ID `about` in one declared Site publishes `en-001` at `/en-001/about` and `de-DE` at `/de-de/ueber-uns`, and the author declares the locale-less entry `/about` for that same Page. A visitor with a supported saved `de-DE` preference is temporarily redirected to `/de-de/ueber-uns`, because the destination is read from the Page declaration rather than invented from the requested path; a visitor with `en-001` preference receives `/en-001/about`; and a direct request to `/en-001/about` stays in English regardless of saved preference, browser language or physical location. Refusals: two Pages claiming the same effective `/about` entry, an entry overlapping another route or alias under its effective matching policy, or an entry that identifies a Page by guessing a localized slug are refused at validation; a Page whose selected locale exists only unlisted with no authorized public fallback returns not-found without disclosing the unlisted route; and an entry that would resolve a Page in a different Facet or Site because another Page shares its ID is refused and diagnosed, with no sibling selection.

**Owner-accepted robustness rule, with exact storage/configuration schema pending:** Supported full tags match after canonical normalization. A browser header may contain a bare tag such as `de`; this is *input preference syntax*, not an authored or published Solo locale. It may resolve only through an **explicitly configured preference mapping** such as `de → de-DE`. If no supported exact match or authored mapping exists, continue to the next preference and finally the default. Never guess between `de-DE`, `de-AT` and `de-CH` or infer one from a location property. The browser is redirected to the actual full-locale public address, not served an implicit `/de/` URL.

**Owner-accepted missing-translation policy:** Only another **authored representation of the exact same Page in the same Site and Facet** may answer a missing translation, and not if the target is unlisted. The Owner prefers fallback to the declared default representation rather than an immediate not-found when that fallback is valid. No automatic translation, cross-Page substitute, cross-Site substitute or invented `hreflang` is allowed.

**Owner-accepted fallback behavior, with exact configuration keys and redirect-status implementation pending:** Define a full-locale default for the applicable Site; a Page may explicitly opt out with a strict/not-found policy or nominate another *public* same-Page fallback. If a requested representation is missing, perform a **temporary redirect** to the actual authorized fallback representation's canonical full-locale URL; do not render a language other than the one in the URL. If no acceptable public representation exists, return not-found with a diagnostic. A public representation must never be silently replaced with a different existing representation. Validation should refuse a declared fallback target that does not exist or is unlisted.

**Owner-accepted addressing distinction:** Full locale tags are authored with canonical casing (e.g., `zh-Hans-CN`), and the public locale path segment uses a deterministic lowercase representation of that full tag (e.g., `zh-hans-cn`). The URL segment is an address, **not** Page identity or a location, and locale-segment normalization is deliberately independent of the Page-route case policy (§3.2B): a case-sensitive Page route is never lowercased by it. Example configured bases:

```text
https://a.abc.com/something/en-001/about
https://a.abc.com/something/de-de/about
https://a.abc.com/something/id-id/about
https://a.abc.com/something/zh-hans-cn/about
```

**For implementation contract acceptance:** The route-case, Unicode, collision, redirect and locale-selection behaviour above is now Owner-accepted (`R00.B`, recorded in §3.2B). What remains open is technical encoding rather than policy: the physical manifest and field schema (`R00.D`), the pinned Unicode data version and toolchain (`R00.E`), and the compiled validators, HTTP adapters, redirect/cache behaviour and rendered language selector (later implementation stages). None of this is permission to start source work.

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

**Owner-accepted Home entry, and remaining implementation details:** a fully unlisted Home Page is permitted. The locale-less base entry prefers an explicitly designated **real public landing Page**; where no public landing representation is available it returns not-found rather than revealing an unlisted Home, and the landing selection must be authored, unambiguous and collision-checked (`B10`). The default seeded `main`/`main`/`home` Page retains its stable EFS identity regardless of how the public entry is configured. Still open: the versioned JSON/Markdown declaration schema and source-validation encoding (`R00.D`); the public route-token grammar itself is accepted in §3.2B (`B04`). The visibility field placement and override precedence above are approved.


### 3.2B Owner-accepted public route-case, Unicode and collision contract

**Default public behaviour (`B05-B`).** Matching for a public localized route claim is **case-insensitive by default**, and its canonical Page-route spelling is the deterministic Unicode-aware lowercase form of that authored claim, normalized to NFC. Case-insensitivity is a default property of the claim, not a universal law that every public URL must obey.

**Author-selected case-sensitive public behaviour (`B05-B`).** An author may explicitly choose **case-sensitive** matching for an individual localized route claim. The proposed logical setting is `caseMatching: insensitive | sensitive`, defaulting to `insensitive` for public claims; its concrete schema representation belongs to `R00.D`. A sensitive claim's canonical Page-route spelling **preserves the authored NFC-normalized Unicode capitalization**. Because the setting is attached to the authored localized route claim, `About`, `about` and `aBout` may deliberately identify **three distinct public Pages** in one locale, and German `Morgen` and `morgen` may remain distinct, provided every relevant claim is sensitive and validation proves no conflict. No Site-wide, Facet-wide or locale-wide case-sensitive switch exists, and there is no hidden global mode.

**Unlisted representations.** Effective unlisted representations always require **exact-case** authored Page-route matching; an unlisted claim can never be matched case-insensitively, so `xY7AbC` is not equivalent to `xy7abc` or `XY7ABC`, while two unlisted claims such as `abc` and `Abc` remain independently valid when their exact keys differ and nothing else is ambiguous. These characters are never lowercased or case-folded during matching, redirects, source lookup or URL generation. All variants of an unlisted route stay outside generated navigation, sitemap, language alternates and every other automatic discovery, and serve `noindex, nofollow`. An unlisted URL is not secret or access-controlled merely because it is hard to guess.

**Locale handling is independent.** Full-locale segments always normalize by the accepted full BCP 47 rules regardless of Page-route matching policy, so `/DE-DE/Morgen` normalizes its locale segment while a sensitive Page route keeps `Morgen`; a case-sensitive Page slug is never lowercased by locale normalization. Page-route matching and locale-segment matching are separate operations, and a wrong-case sensitive request produces no implicit case-based redirect to another claim.

**Validated Unicode routes (`B04`).** Intentional Unicode Page-route text and optional localized representation slugs are first-release capabilities, not a later enhancement: there is no ASCII-only authoring restriction. The logical contract requires valid Unicode text normalized to NFC for comparison and canonical output; well-defined UTF-8 URI encoding with one controlled percent-decoding boundary; separate treatment of decoded text and encoded delimiters; refusal of malformed encodings, encoded path separators, backslash ambiguity, dot-segment traversal, unsafe control characters, ambiguous multiple decoding and claims that normalize into prohibited conflicts; deliberate handling of combining marks, non-Latin scripts, meaningful Unicode capitalization and script-sensitive characters; consistent version-controlled Unicode/casing behaviour across validation, matching and URL generation; and actionable diagnostics for dangerous confusable claims without silently translating or equating lookalike words. The public-route grammar is deliberately distinct from the accepted EFS technical-ID grammar and its 63-character limit. Any Unicode data/profile version that must be pinned belongs to `R00.E`.

**One comparison pipeline.** Comparisons use a single reproducible NFC-based pipeline: insensitive comparisons apply full Unicode caseless behaviour with explicitly documented and tested locale-specific tailoring where applicable, while sensitive comparisons apply NFC-normalized exact comparison without case folding. Canonical URL construction is a distinct step from comparison.

**Publication collision rules (accepted):**

1. Build and validate the complete set of canonical routes, permitted aliases and locale-less entry claims before publication.
2. Route matching must never use declaration order or first-match-wins to resolve ambiguity.
3. Identical effective claims competing for different destinations are invalid.
4. Different sensitive public claims with distinct exact normalized keys may coexist even if their caseless keys match, provided no other claim creates ambiguity.
5. Any insensitive public claim overlapping a sensitive or another insensitive claim is invalid when the claims cannot be resolved unambiguously; hidden precedence must never rescue it.
6. The stricter public/unlisted **caseless-collision refusal** is preserved, including where the public route itself is sensitive.
7. Exact-case distinctness between safely normalized unlisted routes is preserved whenever their exact keys differ.
8. Different full-locale namespaces may deliberately map identical route strings to different Pages.
9. Alias and locale-less entry claims pass equivalent overlap checks and never capture an unintended Page or reveal unlisted content.
10. Duplicate normalized canonical addresses and nonunique canonical targets fail validation; overlapping claims pointing at the same representation are not accepted merely because they agree, unless an explicit nonambiguous alias contract authorizes the relationship.

**Redirect semantics (`B08`) and caching (`B12`).** A stable public canonical spelling correction is ordinarily a `308` where method and safety rules permit; a preference-dependent locale-less entry and a valid missing-translation fallback are temporary, ordinarily `302`; sensitive Page-route casing differences are never silently corrected by redirecting to another sensitive claim. Redirect targets are constructed only from validated internal route authority, never from request data. Preference-dependent locale-selection responses must not be shared-cached (a conservative nonshared/no-store policy is acceptable), while explicit full-locale public responses stay deterministic, unaffected by visitor cookies and ordinarily cacheable.

**Illustrative outcomes — conceptual contract cases, not executed tests:**

| Case | Required behaviour |
| --- | --- |
| Insensitive public `about`; requests `about`, `About`, `ABOUT` | Same Page; lowercase canonical URL |
| Sensitive public `About`, `about`, `aBout` on three different Pages | All three may coexist and resolve separately |
| Sensitive `Morgen` and `morgen` on different German Pages | Both permitted when collision-free |
| Sensitive `Morgen`; request `MORGEN` undeclared | No guessed match; not-found |
| Insensitive `morgen` plus sensitive `Morgen` | Reject the overlapping claims |
| Sensitive public `Morgen` plus unlisted `morgen` | Reject under the retained public/unlisted collision policy |
| Unlisted `abc` and `Abc` | Independently valid exact-case claims when otherwise unambiguous |
| `/en-001/about` and `/de-de/about` | May identify different authored Pages |
| `/DE-DE/Morgen` | Locale casing normalized; sensitive Page-route spelling preserved |
| NFC-equivalent route spellings | One normalized comparison; conflicting declarations refused |
### 3.3 Present one coherent public website

A configurable presentation shell should provide accessible header/navigation/footer, branding, a responsive design system, language controls, internal links, optional Site navigation, content sections and appropriate asset handling. Additional Facets or Sites are *content organization*, not automatic visitor-facing switchers. Public navigation must be authored or explicitly derived under a defined, opted-in policy.

### 3.3A Cascading graphic properties and the shared public library

Solo has **one global, generic, public graphic library** supplied by the platform and available to every Facet, Site, Page and representation in the Installation. It is **not** an EFS ownership node, a publisher or an alternative Entity. It must be provider-neutral and replaceable/maintainable under a clear source-asset versus generated-asset contract.

An Entity-authored graphic selection is a **cascading EFS property** governed by the same accepted inheritance contract as every other property (`R00.C`): `inheritance: inherit | suppress` plus an additive current context of zero or more typed local selections — not a nearest-value-wins rule and not a silent replacement of an inherited asset.

**Registered roles (C07).** Each graphic role is registered with a defined purpose, accepted asset classes, effective cardinality, applicable EFS levels and allowed use contexts. A role extension requires governed registration; an arbitrary unregistered role is invalid. The initial **candidate** core roles are `logo`, `favicon`, `banner`, `background` and `socialPreview` — candidates only, since listing them approves no physical name, cardinality, format or default. Ordinary inline Page media need not be global cascading roles.

- **Inheritance and cardinality:** for a singular role, `inherit` plus a local selection producing two effective entries is a **cardinality failure**, never a silent last/first win. For a collection role the effective collection is inherited-first and then locally authored, and a registered maximum applies to the effective result.
- **Suppression:** `suppress` blocks inherited graphic contributions while still permitting valid locally authored selections; it does not mean "display nothing" regardless of local values, so it never forces an empty result over a valid local asset.
- **Typed sources, one library (C08):** graphic values are explicit, safe, typed source-asset references. Distinguish **Entity-authored source assets** owned within the sole Entity Installation from read-only **generic platform public library assets** — logical source namespaces, not alternate EFS identity nodes, publishers or ownership tiers. Ownership is never inferred from a URL, filesystem path, route, locale or generated output. Missing, ambiguous, unsafe or unapproved sources fail validation, and no arbitrary remote fetching occurs.
- **Fallback is default-deny (C09):** explicitly choosing a generic platform asset is a normal authored value, not fallback. Generic **automatic** fallback is separately governed per role, with **zero roles approved for a fallback initially**, and it is eligible only when there is no effective authored graphic, no relevant inherited or local suppression boundary, no invalid declaration and no private value omitted by the public projection. Any invalid reference, cardinality or visibility violation is a refusal, never a fallback trigger, and fallback is never inferred from geography, locale, route, asset filename or provider.

Generic-library files may also be referenced deliberately by any node without duplicating their source bytes across Facets/Sites. Source assets remain authoritative and transformed or optimized public files are derived outputs; public discovery, including unlisted-content handling, must not expose private asset metadata or unrelated unpublished assets. **The finalized role catalogue, physical asset formats, reference syntax, processing controls and any future per-role default approvals remain open**, not implicitly inherited from historical Foundation.

### 3.4 Support useful public functionality

Targets include SEO/discovery, public and unlisted Pages, optionally authored contact facts, contact/inquiry integration, links to maps/directions or external booking services, and configurable privacy-respecting analytics. Integration contracts should be small ports/adapters, independent of the content model; no CRM, account management, payments, commerce, scheduling inventory or external service is required for basic publication.

**Owner-decided contact scope — included.** Solo's planned contact capability includes **both** (a) safe `mailto:` action handling — validated author-supplied addresses and correctly encoded URIs where the visitor's own mail client performs the sending — and (b) a **simple contact form** with a bounded field set, validation, safe server-side handling, proportionate anti-abuse controls and honest delivery/receipt status. Both are implemented as an application use case plus explicit ports with replaceable adapters; no provider SDK, credential or mail-transport detail may appear in EFS domain objects or authored Page sections.

**Public versus internal recipients — cross-reference (`R00.C`).** A `mailto:` destination is delivered in public markup and is visible to the browser, so an address used that way must be intentionally public and author-approved; it cannot truthfully be described as secret merely because the link is labelled "Email us" or revealed after interaction. An internal recipient address must never be serialized into the public property projection or embedded in public HTML, JS, JSON or URL parameters: a simple form may show a public purpose or recipient alias while the server-side application use case resolves the actual delivery destination through validated configuration and a validated port. Recipient disclosure, delivery mechanics, abuse controls, truthful receipt, privacy and retention for that form remain `R00.F` decisions; nothing here introduces a form builder, workflow system, CRM or mail provider.

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

The URL contract must validate scheme, host, applicable port, normalized and safe prefix segments, reserved namespaces, collisions with Page routes, relative/absolute link handling and correct path boundary matching, and must refuse credentials, query and fragment components, malformed encodings, ambiguous separators and traversal (`B01`). One configured canonical public address is authoritative; optional, explicitly authorized and validated **aliases** are supported, including legitimate address-migration cases, and an alias never becomes another Entity, a Site, a location-derived route authority or an automatically trusted hostname (`B09`). Generated canonical, metadata and discovery URLs always use the effective canonical address, and a configured prefix is preserved **exactly once** across routing, links, metadata and assets. Redirect status and caching behaviour are accepted in §3.2B (`B08`, `B12`), and arbitrary incoming `Host` or forwarding headers are never canonical-address authority. External DNS, TLS, host verification and provider routing remain deployment operations, separate from configuring Solo's public-address model.

### 4.2 Proposed request-resolution pipeline

```text
incoming request URL (host + path)
  → validate the configured public base URL / separately approved aliases
  → remove exactly the configured safe base-path prefix (if any)
  → normalize the full-locale segment per the accepted BCP 47 rules
  → resolve each candidate claim's declared matching policy and effective visibility
  → match one explicit route claim (insensitive by default, sensitive by author choice, exact-case when unlisted)
  → identify authored Facet + Site + language-neutral Page
  → select or resolve an authorized localized representation
  → read and validate its authored source and effective EFS properties
  → render content, links and metadata using the effective canonical base URL
```

No Spoke or Spoke Hub resolution exists. The Entity is already known by the Installation; **neither location properties nor Entity/Site names** are router selectors. Route declarations are validated, not discovered by arbitrary file iteration or first matching array entry, and matching never lowercases the entire incoming path before the effective claim policy and visibility are resolved (§3.2B).

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

The same Page/representation IDs may be published at either chosen base URL without identity changes. A language-neutral Page may carry **different canonical route spellings in different full locales**, and one identical route string used in two different full-locale namespaces may deliberately identify two different Pages (`B02`, §3.2B). Unknown, conflicting or ambiguous route claims must fail clearly, never borrow another Site or guess from `location`.

### 4.4 Canonical URLs, asset addresses and unlisted discovery

Canonical, Open Graph, language alternates, generated links, assets and sitemap entries must be composed with the **configured effective base URL**. A configured path prefix must be preserved **exactly once**; it must not be accidentally dropped or doubled. Server routing and generated metadata must use the same public-address authority.

Only declared public representations may be advertised through generated discovery. An unlisted representation serves only at its exact authorized **case-sensitive** URL, with `noindex, nofollow`, and is absent from generated navigation, sitemap, `hreflang`, `x-default` and language-switch options; it can never be reached through an insensitive public alias. Unlisted is not a protected/private URL.

**Refusal semantics (`B11`).** Malformed, unsupported, ambiguous or unauthorized routing is refused without substituting a different Page. Publication-validation failures are kept distinct from HTTP request failures, and HTTP semantics stay accurate: malformed unsafe requests may return `400`, absent routes or unsupported explicit locales `404`, unsupported methods `405`, and unauthorized request authorities are refused appropriately, including `421` where applicable, while genuine internal failures remain server errors. Errors and diagnostics never disclose unlisted discovery inventories, private source locations or secrets.

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
| Routing integrity | Host-only and prefixed base URLs configured at creation, later config-only URL change, correct prefix handling/canonicals; unknown/colliding claims fail; an insensitive public claim resolves mixed-case requests to one lowercase canonical Page while a sensitive claim preserves authored capitalization and distinct Pages; sensitive/insensitive overlap, public/unlisted and Unicode-normalization collisions are refused; locale segments normalize independently of Page-route case |
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
2. **Address, route and locale contract — decided by `R00.B`, encoding still open:** recorded in §3.2, §3.2B and §4.1–§4.4 are one configured canonical HTTP(S) origin with optional prefix and optional authorized aliases, explicit Page and optional localized routes, validated Unicode route text, default-insensitive public matching with author-selected case-sensitive claims, exact-case unlisted matching, independent full-locale normalization, the ten publication collision rules, `308`/`302` redirect semantics, cache safety and locale-less entry/fallback behaviour. **Still technical:** the physical declaration schema and field placement (`R00.D`), the pinned Unicode data version and toolchain (`R00.E`), and the implemented matcher, HTTP adapter, redirect/cache behaviour and rendered language selector (later stages).
3. **Typed property, location and graphic contract — decided by `R00.C`, encoding still open:** accepted and recorded in §2, §2.1 and §3.3A are the governed extensible property registry, the independent `inheritance`/`currentContext` declaration elements with direct-ancestry-only resolution and effective-cardinality validation, per-value provenance and exposure classification with a validated restricted public projection, zero-to-many complete structured location facts with explicit identified-record extension and no geographic inference, registered graphic roles, typed Entity versus generic-platform asset references, default-deny role fallback and fail-closed publication. **Still technical:** the physical schema and field encoding (`R00.D`), the final property and location field catalogue, the finalized role catalogue with physical names, cardinality, formats and asset processing controls (`R00.D`/`R00.E`), and every implemented resolver, validator, selector and adapter in later stages. No automatic graphic fallback is approved for any role initially.
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
