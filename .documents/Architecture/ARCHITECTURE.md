# Foundation Solo — Architecture Charter

> **Status:** Owner-aligned architecture, 2026-11-06 revision 8. The Owner-accepted physical publication profile (`O01`–`O12`, Appendix-A literals) is integrated in §6.2–§6.2N and supersedes the previous OPEN register, and the Owner-issued conditional software structure and toolchain selection (`E01`–`E10`) is recorded in §6A. Explicit Owner choices are accepted; remaining mechanisms marked **recommended**, **proposed** or deferred to a named later stage are not implementation authority.  
> **Relationship:** [Product definition](../About/FOUNDATION-SOLO.md) is the purpose and scope authority; this document defines the architectural model and invariant boundaries.  
> **Approval:** Only the explicitly identified Owner-stated constraints are settled. Specific mechanisms labeled **proposed** or **open** require a recorded decision before implementation.  
> **Scope:** Product architecture, conceptual functionality, public routing, data ownership, validation and verification approach. This is not a work order.
> **`R00.D` codification status:** the Owner-accepted physical content and publication contract (`D01`–`D12`, including the later `D03` dual-candidate and `D05` standard-Markdown/expandable-module amendments) is recorded in §1.5–§1.7, §2.3–§2.4, §3.1–§3.2C, §3.3B and §6.1–§6.5. The twelve former physical gates `O01`–`O12` are now **settled** by the accepted first physical profile in §6.2–§6.2N, so this Charter is a **complete physical publication specification**: it states every authoring key, type, enum, bound, reference rule, refusal and module contract the future implementation must satisfy. It is **not** executable code: the `.schema.json` artifacts, validators, media pipeline, module implementations, assets and every framework, provider and toolchain choice belong to `R00.E`, `R00.F`, `R00.G` and later implementation packages (§6.5).
> **`R00.E` codification status:** the Owner-issued **conditional** software structure and toolchain selection (`E01`–`E10`) is recorded in §6A, including the exact package pins, the four-package inward-only compile graph, the versioned Rust→TypeScript browser wire contract, the sealed content-addressed publication and compare-and-swap activation contract, the future test and CI architecture and the twenty-two module port/validator/negative register. It is a **documentary selection with unexecuted proof gates**: no package is installed, no schema artefact exists, no test has run and no build exists in this repository, and the four Owner hold points `C1`–`C4` are explicitly `UNVERIFIED`. §6A may not revise `D01`–`D12`, `O01`–`O12` or the §6.2 literals, and no agent may describe this section as executable conformance.


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

**One authoritative publication declaration (`R00.A`, accepted).** A single explicit, versioned publication manifest is the **sole** authority for the one Entity and the published EFS ownership graph: stable IDs, direct-parent associations, the published Page set, supported locale representations and explicit source-selection declarations (§1.6). Only explicitly admitted, complete entries publish; a file, property, route declaration or a label such as `draft` never publishes anything by itself. The logical semantics are accepted here, and the single-manifest **strict UTF-8 JSON** format with a mandatory root `schemaVersion` is settled (§1.5). The manifest's filename and location, exact subordinate field layout, initial `schemaVersion` value, source-locator serialization, content-section schema and JSON Schema implementation and dialect remain **OPEN** as listed in §6.2.

**Source integrity (`R00.A`, accepted; source-selection wording superseded by the Owner-amended `D03` in §1.6).** Every published localized representation binds explicitly to an authored **source-selection declaration** belonging to its declared Page: at most two explicitly named candidate locators — one JSON and one Markdown — from which exactly **one effective authored source** is selected. That selected source must resolve to exactly one existing, valid and permitted authored body inside the validated source boundary; implicit path matching, filename or extension discovery, directory scanning, file iteration order, first-file-wins, search-order fallback and sibling source substitution are all refused, and no unselected candidate body may be merged with, or substitute for, the selected one. Two distinct published claims must not share one canonical authored source **candidate**, including a candidate that is not currently present, and any intended source sharing or generated source content needs its own later explicit contract. At least one candidate must therefore be declared, and a genuinely absent preferred JSON candidate is permitted whenever a valid declared Markdown candidate can be selected; candidate absence is never itself a refusal, while an unsafe, ambiguous or duplicate-claimed candidate is. Undeclared files never publish; an unsafe, ambiguous or duplicate-claimed candidate, an invalid or inaccessible selected or preferred resource, and any failure to select exactly one valid effective source all fail validation; generated output and public assets never become the authored manifest authority.

**Fail-closed refusal specification (`R00.A`, accepted).** Validation must reject, with an actionable diagnostic that names the affected declaration or ID and the reason: a second Entity; missing or invalid IDs; duplicate IDs in the relevant parent scope; a missing, unknown or ambiguous parent; an orphan Site, Page or representation; an illegal cross-parent binding; an empty published Facet, Site or Page; a missing published representation; a duplicate locale within one Page after canonical normalization; undeclared content publication; a duplicate-claimed, unsafe or ambiguous declared source candidate, or any failure to select exactly one valid effective source (§1.6); and publication inferred from a directory or filename. Errors are never hidden by inventing a default parent, choosing the first match, reassigning identity or redirecting to a sibling, and no diagnostic exposes server-private paths or secrets.

### 1.4 Conceptual identity and publication examples

These examples are **conceptual specification evidence only**: they are not executed validation, and no file is claimed to exist. Their concrete schema, locator and version literals are now settled by the accepted first physical profile (§6.2A–§6.2C), and complete worked manifest, Markdown and JSON examples appear in §6.2M; tooling and runtime validation are deferred to `R00.E` and Stage 02.

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

### 1.5 The publication manifest — sole authority, encoding and version (`R00.D` `D01`)

**Owner-accepted.** Exactly one directly authored, strict-JSON publication manifest is the sole authoritative publication declaration for the Installation. It declares the one Entity's complete published EFS ownership graph — Entity → Facet(s) → Site(s) → Page(s) → localized representation(s) — together with each representation's explicit source-selection binding (§1.6), including representations that are effectively unlisted. Drafts (§1.3) remain outside this graph.

- **Encoding and refusal.** The manifest is strict UTF-8 JSON. Malformed JSON, duplicate object members, an unsupported schema version and unsupported fields are refused. There is no tolerant parsing, no acceptance of unknown fields and no automatic repair.
- **Mandatory root `schemaVersion`.** The manifest is strict UTF-8 JSON with a **mandatory root field named literally `schemaVersion`** — an accepted field name, not an illustrative suggestion — versioned **independently** of the Solo product release, the JSON Schema dialect, the JSON-content schema and the module contracts. Unsupported version values, unknown fields, duplicate keys and malformed JSON fail closed; schema evolution is explicit and validated; these version identities never silently collapse into one. Its actual initial **value**, the concrete schema artifact and the remaining subordinate field layout stay **OPEN** (§6.2).
- **Single authority.** No auto-discovery, runtime overlay, secondary manifest, generated file, registry or public asset gains publication authority. One invalid published declaration invalidates the whole proposed publication.

**Declared EFS declarations and identities (`D02`).** The manifest expresses the ownership tree physically as one nested structure: the Entity contains its Facets, each Facet its Sites, each Site its Pages, and each Page its localized representation declarations. Immediate containment **is** the direct-parent association; competing parent fields, orphan relationships and cross-parent claims are invalid. The accepted `R00.A` ID grammar, 63-character limit and parent-scoped uniqueness are unchanged (§1.1), and invalid authored IDs are refused rather than normalized, repaired or derived. A usable starter Installation still consists of an Owner-supplied Entity ID, a `main` Facet, a `main` Site, a `home` Page and real content in an explicitly chosen supported full locale. In-memory indexes, derived registries and generated output are never authoritative, and cross-node references use complete qualified EFS ownership lineage rather than an ambiguous short ID. Physical movement of a declaration is not an implicit migration; reparenting, ID change and deletion require explicit, later-approved migrations with reference integrity.

**Physical literals — settled by the accepted first profile (`O01`/`O02`; §6.2A–§6.2B).** The manifest is exactly `content/publication.json`: strict UTF-8 JSON without BOM, mandatory root `schemaVersion: "1.0.0"`, the closed five-key root of §6.2B, maximum 2 MiB, unknown fields refused. The JSON Schema dialect is Draft 2020-12 with `unevaluatedProperties: false` and asserted formats, and the future artifact is `contracts/publication/1.0.0.schema.json`. The EFS container shape, representation container and qualified-reference object serialization are fixed in §6.2B, and migration semantics in §6.2K. These are accepted literals, not installed code.

### 1.6 Declared source candidates and exactly one effective source (Owner-amended `D03`)

**Owner-accepted — final amendment, superseding every earlier single-source-file statement in this Charter.** Each published representation declares one authoritative source-selection contract containing at most two **explicitly named** candidate locators: one JSON candidate and one Markdown candidate. The author must declare at least one candidate, and declaring both permits an automatic Markdown-to-JSON upgrade later without editing the manifest.

- **Never discovered.** Candidate paths are never inferred from file names, route text, Page ID, locale, directory scans, globbing, file iteration order, extension guessing or a neighbouring representation.
- **Exactly one effective source.** When supplied and genuinely present, the JSON candidate is the **only** selected source. When no JSON candidate is declared, or the declared JSON candidate is genuinely absent, the explicitly designated Markdown candidate is the sole selected source if it is present and valid. When both are present, JSON wins and the Markdown body is unselected: the two bodies are never merged and file iteration order has no influence. An unselected Markdown body need not be parsed, but its declared locator and ownership are still validated.
- **Errors are not absence.** Access denial, an invalid locator, unsafe redirection or symlink traversal, unreadability, ambiguity and I/O error **block publication**; they are never treated as absence and never licence a Markdown fallback. Invalid JSON, an unsupported schema, an invalid module, insecure content or a wrong file type blocks the entire proposed publication — Solo never falls back to Markdown because existing JSON is invalid.
- **Exclusivity.** Duplicate or overlapping claims to the same canonical authored source across distinct published representations — **including a candidate that is not currently present** — are refused, and multiple spellings or indirections may not evade the check. Intended source sharing or generated source content requires its own later Owner-approved contract.
- **No implicit publication, no identity change.** A file the manifest does not claim cannot create a Page or representation, and an incomplete draft stays outside published authority until explicitly admitted and fully validated. Changing a file path, or switching a representation from Markdown to JSON, mints no new EFS identity and alters no route, visibility or property authority.
- **Selected-source evidence.** The selected-source identity and a safe diagnostic explaining the JSON-first choice are available for authoring and verification.

**Source-selection cases — conceptual contract cases, not executed tests:**

| Declared candidates and state | Required behaviour |
| --- | --- |
| JSON candidate absent; Markdown present and valid | Markdown is the effective source |
| JSON present and valid; Markdown absent | JSON is the effective source |
| JSON present and valid; Markdown present and valid | JSON is the effective source; bodies never merged |
| JSON candidate present but invalid | Publication refused; no Markdown fallback |
| JSON candidate unreadable or permission denied; Markdown valid | Publication refused; not treated as absence |
| Neither candidate present or declared | Publication refused |
| Two representations reserving the same candidate, including an absent one | Publication refused as a duplicate source claim |

**Physical literals — settled by the accepted first profile (`O03`; §6.2C).** The declaration key is `sourceCandidates` with optional string members `json` and `markdown`, never `null`, never an empty object and no additional keys; locators are portable `pages/…` paths relative to `content/` with exact lowercase extensions; existence is a first-class safely confirmed result; an absent candidate is expressed by omission only and `null` is refused; hash pinning is not required; duplicate, alias and hardlink claims refuse even when the path is absent; and validation runs against an immutable snapshot whose change invalidates the candidate.

### 1.7 Authoring responsibilities: JSON and Markdown bodies under manifest authority (`D04`)

**Owner-accepted.** A localized representation has one effective body source, either declarative JSON or standalone Markdown (§1.6). JSON is the advanced full-featured Page authoring format; Markdown is approachable for simple Pages and non-technical authors. Both produce a common typed and validated content representation and pass the same safety, publication and visibility rules, so identical supported constructs have equivalent semantics. Full expressive parity is **not** required.

**Manifest authority is not negotiable by a content file.** The manifest declarations alone own EFS identity, locale, membership, source candidates, Page `visibilityDefault`, representation `visibility`, public route and alias claims, locale fallback and property publication authority (§3.2, §3.2A, §3.2C, §2.3). A source file cannot override or contradict that authority through front matter, YAML or JSON fields, and there is no content-source merging, overlay, first/last-wins precedence or secondary publication setting. JSON-first selection decides **which body** is effective; it never transfers metadata authority to that body.

**Physical literals — settled by the accepted first profile (`O04`/`O05`; §6.2D–§6.2E).** A JSON body is strict UTF-8 with mandatory `contentVersion: "1.0.0"`, a mandatory `title` and a mandatory `blocks` node array; a Markdown body is CommonMark 0.31.2 plus GFM 0.29-gfm with raw HTML refused. The common normalized content model is exactly the closed body grammar of §6.2E, and there is **no** content-only metadata channel: any body attempt to set publication metadata is refused, never silently ignored as an override.

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

Examples of useful property families are presentation and identity assets, authored titles/descriptions, contact channels, **location/place facts**, opening-hours facts, external action destinations and other explicitly typed authored facts. The Owner requires location to be supported as a cascading property at all five EFS levels. The exact property field catalogue, physical types and encoding and the physical representation of the two accepted declaration elements are now directed by §2.3, with their literals remaining **OPEN** (`R00.D`, §6.2) — never inferred from this example.

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
- **No implicit primary.** A collection of several facts never yields a single implicit "primary" location; a downstream output that requires exactly one must use an explicitly governed selection or refuse ambiguous input. The physical record identifier format, extension-patch encoding and registry field catalogue remain **OPEN** `R00.D` literals (§2.4, §6.2), while a logical record reference must stay stable across routing and location-text changes.

A request such as `https://example.org/bali/en-001/home` **must not derive** Bali from `/bali`, `bali` as a Site ID, the hostname or the `en-001` tag. If Bali is displayed as location, the source must be an explicit property in the selected EFS ancestry. Changing `location` alone must not change route resolution or canonical URL. Conversely, changing base URL must not change `location`.

**No implicit multi-premises hierarchy:** several Site contexts, or several structured location facts on one node, may describe different premises without creating a geographic Site tier or a mandatory location level. A collection of facts is ordinary property data — its physical encoding and expected rendering remain later choices — and neither imposes geographic Site identity.

### 2.2 Location and localization remain independent

Entity, Facet, Site and Page are language-neutral. The localized representation is the only locale-bearing content node. For example, an Entity may publish a `fr-FR` Page representation with an address in Indonesia without that address changing to France, without creating a France Site, and without changing its route by geography. A region subtag is part of a locale tag, **not a geographic locator**.

### 2.3 Declared property encoding direction (`R00.D` `D08`)

**Owner-accepted declaration direction.** Each of the five EFS levels — Entity, Facet, Site, Page and localized representation — may carry an optional **directly inline `properties` object** attached to that EFS node itself. `properties` is the accepted exact key name; its entries are declarations keyed by **registered property identity**, so the object belongs to its owner and is not a central authoritative property ledger or a separate publishing structure. The governed registry (§2) remains the sole definition authority for value type, allowed EFS levels, effective cardinality, equality and uniqueness, maximum public exposure, permitted special operations and validation. Unknown properties and conflicting or incompatible definitions refuse publication.

- **Absence versus declaration.** A property wholly absent at a node implicitly inherits its direct parent's effective result, exactly as in §2. A property declared at a node carries the two independent accepted elements — `inheritance: inherit | suppress` and `currentContext: none | zero-or-more typed local values` — where `none` applies only to the local contribution. At the Entity root the inherited input is empty.
- **Resolution and ordering.** `inherit` carries the ancestor's effective values ordered inherited-first and then locally authored; `suppress` clears the inherited input while still permitting valid local additions. There is never a nearest-value-wins rule, silent deduplication, arbitrary merge or declaration-order precedence.
- **Cardinality and ancestry.** A registered maximum applies to the **effective** collection, so a singular inherited plus local selection violates cardinality unless a valid suppression or registered special operation applies. Inheritance follows immediate EFS ancestry only; siblings, locale, location facts and routes confer no ancestry.
- **Provenance and projection.** Resolved values retain their per-value EFS origin, inclusion or suppression decision and any permitted field provenance, each with an explicit exposure classification inside the registered maximum. Only the validated public projection feeds public HTML, scripts, JSON, metadata, feeds and presentation modules, and omitting an internal value never causes an arbitrary public ancestor to substitute for it.
- **Fail closed.** Every invalid or effectively unlisted property declaration still blocks complete publication; `internal` exposure is not secrecy for material in a public repository or delivery configuration.

**Physical literals — settled by the accepted first profile (`O07`; §6.2G).** `properties` is the exact inline key on all five levels; every declaration is the closed `{ "inheritance": "inherit"|"suppress", "currentContext": "none" | [ … ] }`; `propertyRegistryVersion: "1.0.0"` is mandatory at manifest root; exposure is exactly `public|internal`; and the initial nine-key catalogue, effective cardinalities, uniqueness rules and the derived provenance record are fixed in §6.2G. No unlisted key becomes a built-in property by appearing anywhere in this Charter.

### 2.4 Structured location facts and identified-record extension (`R00.D` `D09`)

**Owner-accepted.** `location` is an optional registered property at every EFS level whose current context is zero or more complete structured facts; no premises, country, city, postal address or coordinate is mandatory, and there is no mandatory location tier (§2.1). Distinct **semantic types** represent world region, country, state or province, locality, district, named place or area, postal components, explicit coordinates, association versus premises, and governed typed custom metadata — never one ambiguous universal region field.

- **Fact identity.** A fact has a stable authored fact identity, its original qualified EFS owner, its type, its registered fields and a whole-record exposure classification.
- **`add` creates independently.** A new independent fact uses an explicit `add` operation; it is never implicitly merged with an inherited fact.
- **`extend` modifies exactly one identified record in place.** An `extend` operation identifies **exactly one accessible inherited record** by its original fully qualified identity and supplies bounded, explicit field additions or replacements. It updates that fact in place without appending a duplicate or changing its identity or order, preserves original provenance for untouched fields and records local field provenance for modified fields.
- **Refusals.** Missing, cross-branch, ambiguous, suppressed or circular targets, conflicting same-node extensions and invalid fields are refused. Suppression excludes inherited facts, so extending a suppressed record is invalid.
- **Exposure.** An internal inherited fact or internal inherited field can never be silently promoted to public: whole-record exposure applies and no implicit per-field publication policy exists.
- **No implicit primary.** Several facts create no implicit primary record; any one-record consumer requires an explicit governed selection. Geographic correctness is authored responsibility — Solo validates types and safe serialization but neither geocodes nor claims external verification.
- **No inference either way.** Neither URL, Site ID, hostname, language, region tag nor coordinates may infer or modify a location, and location may never infer or modify a route (§2.1).

**Physical literals — settled by the accepted first profile (`O08`; §6.2H).** The `add` and `extend` envelopes, the `factId` grammar, the closed `LocationFields` catalogue with WGS84 coordinates object `{ "latitude": number, "longitude": number }`, `QualifiedOwner` contiguous lineage, whole-record exposure, in-place field provenance and the one-`extend`-per-target-per-node rule are fixed in §6.2H. The previously proposed key names are confirmed there as canonical syntax.

## 3. Conceptual functionality — what Solo must be able to do

### 3.1 Author Pages safely and flexibly

Offer two complementary authored Page formats under one manifest authority (§1.7): declarative JSON as the **advanced, expansive** design format, and standard Markdown as the approachable format for simple and text-forward Pages. Both are validated into one common typed content representation, both obey the same safety, publication and visibility rules, and neither may override manifest-owned identity, locale, route, visibility or property authority. Which body is effective is decided solely by the source-selection contract (§1.6) — the JSON-first rule of the Owner-amended `D03` — never by file iteration order, and never by parsing both bodies together.

### 3.1A Accessible mainstream Markdown (Owner-amended `D05`)

**Owner-accepted.** Markdown means standard, familiar Markdown: the CommonMark baseline plus **widely recognized mainstream extensions** — for example GitHub Flavored Markdown tables, strikethrough and task lists. It is deliberately not an extensible programming surface.

- **No Solo-specific Markdown.** No Solo directives, no front-matter control language, no MDX, no arbitrary components and no programming-like component language.
- **No interpreted raw HTML.** Scripts, inline event handlers, embedded frames and unrestricted HTML or CSS never execute. Raw markup is refused or rendered inert according to the profile fixed in §6.2, never trusted.
- **Full Page authoring, not a reduced teaser format.** Markdown can author an ordinary complete Page within its recognized feature set; full expressive parity with JSON is **not** required (§1.7).
- **Safety and accessibility are content rules, not renderer preferences.** Heading structure, link text, image alternatives, page language and any motion behaviour follow the same accessible and safe-authoring obligations as JSON content.
- **Links and media** resolve under the separate locator, link and resource contract in §3.1C, not through Markdown-specific indirection.

**Physical literals — settled by the accepted first profile (`O04`; §6.2D).** CommonMark 0.31.2 plus GFM 0.29-gfm; raw HTML blocks and inline nodes are refused rather than sanitized; exactly one H1 as the first semantic block; heading levels may not skip; anchors derive from visible heading text under the pinned Unicode profile with `section` fallback and `-2`, `-3` suffixes; links resolve against the current canonical Page URL; the Markdown image base is `content/assets/`; and permitted click-through schemes are exactly `https`, `http` and `mailto`.

### 3.1B Advanced modular JSON design library (`D05`)

**Owner-accepted.** JSON is the **advanced and expansive** validated declarative design format. It is deliberately **not** capped at the originally illustrative seventeen section families: a versioned registered library may grow to include containers; responsive grids, columns, stacks, split and bento layouts; layered, full-bleed and sticky compositions; typography and brand presets; background colours, gradients, images and video; overlays; text and image watermarks; decoration and patterns; hero variants; media; galleries; lightboxes; image and content carousels; comparisons; mockups; responsive images; animation, transition, reveal, hover, parallax and marquee behaviours with motion controls; tabs, accordions, modals and filters; cards, features, steps, counters, statistics, pricing, testimonials, portfolios, FAQ and contact actions; and approved bounded specialized functionality. These are **capability families, not an authorization to ship any module unreviewed**, and the earlier illustrative section list in this Charter's history was a starting point, never a ceiling.

**Named, versioned, trusted Solo modules.** Advanced functionality is reached by **naming a registered Solo module and supplying strictly validated typed values**. Each module has a typed input schema, child and composition constraints, format limits, documented accessible behaviour and any explicitly approved safe fallback. Modules are governed and versioned, an addition should not break installations that do not reference it, and an installation using a module requires explicit compatible-contract treatment before its behaviour changes (§6). A module may never: create additional EFS nodes, mint publication routes, bypass property exposure, fetch unsafe assets, load arbitrary code or introduce capabilities expressly excluded from product scope — notably advanced form builders (see §3.4).

- **No executable authored content.** No arbitrary JavaScript, no interpreted HTML, no unrestricted CSS, no remote plugins and no untrusted dynamic code may be reached through authored JSON.
- **Strict Hexagonal Architecture applies.** EFS and publication decisions remain domain and application rules; a module is an adapter-facing capability, never the owner of core behaviour (see §5).
- **Mandatory quality policy per module.** Accessible keyboard behaviour, reduced-motion support, controls for autoplay and animation, contrast, responsive behaviour, asset loading and performance bounds are binding policy topics for every module, not optional styling.
- **Illustrative names are not products.** Naming a capability family in this Charter proves no module exists, and no sample module identifier or version value is a shipped or approved artifact.

**Owner-approved design reference register — all seven retained:** <https://swiftguest.com/> , <https://sitelings.ai> , <https://www.untilnow.com.au/> , <https://www.vectapp.io/> , <https://northfoundry.co/> , <https://siftp.com/> , <https://www.myumbrella.ai/> . They inform the flexibility of design capability. No exact design replication, copyrighted content reuse, proprietary code reuse or automatic feature extraction is claimed or authorized, and this register lives here rather than in a parallel ungoverned requirements queue.

**Physical literals — settled by the accepted first profile (`O05`; §6.2E).** The first installed registry is exactly the twenty-two `1.0.0` module contracts tabulated in §6.2E.2, each with closed `props`, defaults only where enumerated, and an explicit child policy; node depth ≤12, ≤250 nodes per body, child arrays ≤24, and `span` permitted only under `layout.grid`/`layout.bento`; `presentation` tokens and their CSS-pixel distances are fixed in §6.2E.1. Naming a future capability family still installs nothing.

### 3.1C Safe locators, links and resource separation (`D11`)

**Owner-accepted.** Three different semantic contracts are never conflated: **source locators** (where authored body content lives), **typed asset references** (which governed media a declaration selects) and **public links** (what a published Page points at). Logical namespaces are not EFS owners: authored content sources resolve only inside their permitted authored-content namespace, Entity-owned source assets and the read-only shared generic public library occupy two separate explicit namespaces (§3.3A), and drafts and derived output are never publication source authority.

- **Portable locators.** Prefer portable relative, `/`-separated source locators. Refuse absolute paths, drive letters, UNC paths, home expansion, empty or dot and dot-dot segments, backslash ambiguity, control and null bytes, URL-escaped path-decoding ambiguities, unsafe query or fragment components in filesystem locators, unsupported indirection and symlink traversal.
- **Resolved containment.** Containment inside the assigned boundary must be verified on the **resolved** target; a string prefix comparison alone is insufficient.
- **Declared, not discovered.** Both `D03` candidate locators are validated, and cross-representation source ownership is enforced even when a candidate is currently absent (§1.6). For JSON-first selection only genuine preferred-JSON absence permits Markdown; an unsafe or unreadable preferred file is a refusal.
- **Never authority.** A direct path into secrets, configuration, drafts, implementation code or generated assets is never acceptable as source authority.
- **Public addresses.** Public URL generation uses the accepted canonical route or an authorized action together with the configured base prefix, applied exactly **once** (§4.1, §4.4). A storage path never becomes a public URL.
- **Markdown links.** Standard syntax only: `[text](relative-reference)`, `![alt](relative-media)`, recognized external `HTTP(S)`, safe `mailto:` actions and validated same-Page fragment references. Image references map through a documented author-friendly media base into validated Entity assets.
- **Refusals.** Unsafe schemes (for example `javascript:`), ambiguous targets, unapproved remote image loading and arbitrary server-side fetching fail. An intentionally authored direct link to a known unlisted URL may be permitted under the classified safe-link rules, but it never creates automatic discovery (§3.2A, §4.4).
- **Identity and ports.** Changing a source path never changes EFS identity. Modules reach resources only through approved application ports, and external provider retrieval requires separate permission and a bounded adapter (§5).

**Physical literals — settled by the accepted first profile (`O03`/`O11`; §6.2C, §6.2K).** Locators are portable lowercase-ASCII relative paths of ≤240 bytes and ≤8 segments with no escapes, traversal or reserved names; namespaces are `content/pages/`, `content/assets/`, `content/drafts/`, read-only `platform/assets/` and product `contracts/`; containment is proven by atomic no-follow traversal and a string prefix never proves it; permitted external schemes and `mailto` limits are fixed in §6.2D.7 and §6.2J.9; and the typed media reference grammar is §6.2I.

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

**For implementation contract acceptance:** The route-case, Unicode, collision, redirect and locale-selection behaviour above is Owner-accepted (`R00.B`, recorded in §3.2B), and its physical encoding — the manifest and field schema literals, the pinned Unicode 18.0.0 profile and the collision ledger — is now settled (`R00.D`, §3.2C and §6.2J). What remains is implementation rather than policy: the toolchain that binds that Unicode data (`R00.E`) and the compiled validators, HTTP adapters, redirect/cache behaviour and rendered language selector of later implementation stages. None of this is permission to start source work.

- Language switching stays on the exact same Page and Site; unavailable variants are not advertised as available.
- A declared default locale must be a supported full tag. Missing fallbacks never promote an unlisted representation.
- `lang`, canonical, `hreflang`, sitemap and language alternates must describe the real public representation; no absent/unlisted variants are advertised.
- Dictionaries may localize labels and navigation without introducing localization at Entity, Facet, Site or Page level.

### 3.2A Unlisted representations and Page-wide convenience

**Owner-accepted behavior:** Every localized representation has an effective `public` or `unlisted` visibility state. An entire Page can be made unlisted conveniently, and an individual representation may be explicitly public or unlisted even when the rest of the Page differs. Visibility is a discovery decision, not authentication.

**Owner-accepted authoring authority and precedence:** The Page declaration has an optional `visibilityDefault`, defaulting to `public`; a representation may declare a `visibility` override. These are the selected configuration names, and both are **direct inline fields of their own manifest declarations** — `visibilityDefault` on the Page declaration and `visibility` on each localized representation declaration — never a setting inside content JSON or Markdown, a central override table or a `publication` wrapper; they remain subject to coherent schema validation and implementation proof. The effective state is the representation's own explicit value, otherwise the Page default. Setting `visibilityDefault: unlisted` marks all variants unlisted **unless the author explicitly sets a variant public**. The validated Page/representation declaration is the one authority for both JSON and Markdown content. There must be no contradictory second visibility setting inside a content file.

| Page default | Representation override | Effective state |
| --- | --- | --- |
| omitted (`public`) | omitted | `public` |
| `public` | `unlisted` | `unlisted` |
| `unlisted` | omitted | `unlisted` |
| `unlisted` | `public` | `public` |

An unlisted representation serves at its **exact authorized URL** and carries `robots: noindex, nofollow`; it is absent from generated navigation, sitemap, `hreflang`, language-switch options, `x-default` and any other automatic discovery. It is **not private**: no secrets or access-controlled material may rely on `unlisted`. An explicitly authored direct link can intentionally point to a known unlisted URL only under the eventual safe-link contract; the platform does not automatically advertise it. Missing-translation fallback must never select an unlisted representation. Behavior must be equivalent for JSON and Markdown content.

**Owner-accepted Home entry, and remaining implementation details:** a fully unlisted Home Page is permitted. The locale-less base entry prefers an explicitly designated **real public landing Page**; where no public landing representation is available it returns not-found rather than revealing an unlisted Home, and the landing selection must be authored, unambiguous and collision-checked (`B10`). The default seeded `main`/`main`/`home` Page retains its stable EFS identity regardless of how the public entry is configured. The physical declaration is settled: `visibilityDefault` is a direct Page field and `visibility` a direct representation field, each `public|unlisted` with `public` as the Page default (§6.2B, §6.2J), while the public route-token grammar is accepted in §3.2B (`B04`) and restated physically in §6.2J.3. The visibility field placement and override precedence above are approved.


### 3.2B Owner-accepted public route-case, Unicode and collision contract

**Default public behaviour (`B05-B`).** Matching for a public localized route claim is **case-insensitive by default**, and its canonical Page-route spelling is the deterministic Unicode-aware lowercase form of that authored claim, normalized to NFC. Case-insensitivity is a default property of the claim, not a universal law that every public URL must obey.

**Author-selected case-sensitive public behaviour (`B05-B`).** An author may explicitly choose **case-sensitive** matching for an individual localized route claim. The setting is `caseMatching: "insensitive" | "sensitive"`, optional on public claims and defaulting to `insensitive`; it must be **omitted** on an effectively unlisted representation, whose matching is always the exact authored NFC spelling (§6.2J.5). A sensitive claim's canonical Page-route spelling **preserves the authored NFC-normalized Unicode capitalization**. Because the setting is attached to the authored localized route claim, `About`, `about` and `aBout` may deliberately identify **three distinct public Pages** in one locale, and German `Morgen` and `morgen` may remain distinct, provided every relevant claim is sensitive and validation proves no conflict. No Site-wide, Facet-wide or locale-wide case-sensitive switch exists, and there is no hidden global mode.

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
### 3.2C Declared addressing, routing and locale declarations (`R00.D` `D07`)

**Owner-accepted declaration direction.** Public addressing is authored at a specific level, and no level may borrow another level's authority:

- **Install root:** declares one canonical HTTP(S) origin with an optional safe path prefix, plus any separately authorized base-address aliases — never credentials, query, fragment, unsafe prefix or `Host`/forwarding-header-derived authority. A base change neither remints identity nor requires moving authored source files.
- **Site:** declares its explicit supported full default locale and the mappings used for browser language preferences.
- **Page:** owns its optional locale-less entry routes and its missing-translation policy.
- **Localized representation:** owns its full canonical region-qualified BCP 47 locale, its explicit canonical route claim, any permitted explicit route aliases and, per claim, its `caseMatching` policy (§3.2B).
- **Derived ledger, one authority.** A complete derived ledger of canonical, alias and locale-less entry claims is validated before publication. No independent registry, hand-maintained index or second declaration file creates routing authority, and the manifest remains the sole publication authority (§1.5).

**Policy is unchanged by `D07`.** Full-locale representation identity, locale-less entry precedence, explicit full-locale precedence over preferences, same-Page public-only fallback, the public/unlisted case contract, Unicode route handling, collision refusal, redirect status and cache safety, and base-prefix-once addressing are exactly as accepted in §3.2, §3.2A, §3.2B and §4.1–§4.4. Nothing here revises them, and no declaration may contradict them.

**Physical literals — settled by the accepted first profile (`O10`; §6.2J).** The closed `base` object, `Site.defaultLocale`, `languagePreferences`, Page `entryPaths`, `missingTranslation`, the representation `route` object with `path`/`caseMatching`/`aliases`, the route-token lexical grammar, the pinned Unicode 18.0.0 profile, base-prefix-once composition and the four typed link variants are all fixed in §6.2J. Adapter, matcher and framework choices remain `R00.E`.

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

### 3.3B Declared graphic selections and typed asset sources (`R00.D` `D10`)

**Owner-accepted declaration direction.** Graphic selections **are** property declarations under §2.3, carrying exactly the same `inheritance` and `currentContext` semantics; there is no separate graphic inheritance mechanism. Registered roles govern the allowed asset classes, applicable EFS levels, use contexts, effective cardinality and exposure.

- **Candidate roles only.** The initial candidate core roles are `logo`, `favicon`, `banner`, `background` and `socialPreview`. Their final exact names, accepted formats and cardinalities are **not** automatically selected by accepting this contract.
- **Singularity fails loudly.** For a singular role, inherited plus locally authored entries producing more than one effective selection is a cardinality failure — never a silent override. `suppress` may remove inherited selections and then accept a valid local one.
- **Two declared, typed source namespaces.** One Entity-owned authored source namespace, and one read-only shared generic platform public library available throughout the Installation without becoming an EFS node, an alternate Entity or a publisher. Namespace ownership is **declared**, never inferred from a public URL, file path, locale or generated output.
- **Fail closed.** Any unregistered, unsafe, ambiguous, wrong-class or missing declared asset fails validation, and no arbitrary remote fetch occurs.
- **Zero automatic fallback.** No role is approved for automatic generic fallback initially. Explicitly selecting a generic library asset is an authored value, not fallback, and fallback never resurrects an invalid private or suppressed value (§3.3A).
- **Derived output.** Source assets are authoritative; transformed and optimized public files are derived outputs carrying no authorship authority.
- **Direct module media.** Advanced JSON modules may accept **direct typed media asset references** — for watermarks, backgrounds, galleries, carousels and layered heroes — without turning every media item into a cascading graphic property. Module inputs remain validated, and no arbitrary file or remote fetch is allowed.

**Physical literals — settled by the accepted first profile (`O09`; §6.2I).** The role cardinalities, the `{ "namespace": "entity"|"platform", "reference": "…" }` asset reference, the platform library pinned by `platformAssetLibraryVersion: "1.0.0"`, the image and video allowlists with byte, dimension and duration bounds, required WebVTT captions, and the refusal of active SVG, GIF and remote fetch are fixed in §6.2I. **No artwork, codec bundle or asset filename is mandated here**, and no automatic generic fallback exists.

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

This contract does **not** by itself select a framework, language, directory tree, application server, persistence model or hosting platform, and it does not justify a ceremonial abstraction layer with no boundary purpose. The language, package layout, framework and browser/build toolchain **are** now selected conditionally in §6A (`E01`–`E10`); an application server, persistence model and hosting platform remain **unselected** and belong to the later deployment decision. Where §6A and this section overlap, §6A fixes the concrete packages and enforcement technique while this section remains the binding boundary contract. The simplest implementation that provably preserves these boundaries is preferred.

Solo must have no runtime dependency on the existing Foundation, its demonstration installation, Spokes, Provelopment root governance files or an external managed tenant router. Reference code may be evaluated and deliberately reused only after checking compatibility, provenance, licensing and fit for Solo's clean architecture.

## 5A. Security by design — cross-cutting risk, integrity and verification

**Security is a design obligation, not a hardening stage.** It applies from architectural planning through implementation, release and maintenance of every Solo capability. The **single binding security acceptance standard is A1**, defined in [Architect SOP](../SOP/sop-architect.md) §7B: a code-bearing task cannot pass with an untriaged critical or high security or correctness defect, a privacy leak, unsafe authored execution, exposed credentials, or missing applicable refusal and negative evidence. This section fixes only the architectural boundaries that standard is applied against. It is **not** a second policy, and it maintains no advisory watchlist and no separate risk register. A genuinely confidential vulnerability is reported through GitHub private vulnerability reporting ([`.github/SECURITY.md`](../../.github/SECURITY.md)), never a public issue, pull request, commit message or chat log. **No control described in this section is implemented**, and nothing here certifies security: this repository still contains no application code, dependency manifest, CI configuration or deployment.

**Applicability is assessed, never assumed.** A security question is judged against Solo's real trust boundaries at the exact revision under review, and vendor-wide severity is never presented as Solo-specific risk. Remediating an existing finding requires its own bounded Owner-issued order; this section authorises nothing.

**Trust boundaries and attack surface to analyse.** The public request and routing boundary; authored content including Markdown and JSON bodies; configuration and server-only secrets; adapters and outbound integrations such as storage, email transport, providers and analytics; the published release artefact and its update path; and any persisted data. Location, locale and route facts are never security inputs, and unlisted visibility is never access control.

**Preventive constraints.** Validate at the boundary and fail closed: malformed, ambiguous or unauthorized input is refused rather than coerced into a plausible result. Keep secrets and server-only configuration out of authored content, public projections, diagnostics and source control. Apply least privilege to every adapter, provider and operator action, and treat provider responses as untrusted input. Preserve the Ports-and-Adapters boundary above so that security policy stays in the domain core instead of leaking into — or being assumed from — an adapter. Authorized content never overwrites generated output, and no authored filename publishes itself.

**Supply chain and update integrity.** Adopt a dependency or provider only after checking its provenance, licence, maintenance status and security posture; pin what is adopted and record the decision. A release must remain an immutable, verifiable artefact distinct from an Installation and from a deployment. Update design must verify artefact integrity before activation, apply change atomically through staging with recoverable rollback, and never silently discard authored content or a last known good state.

**Verification expectation.** A1 requires objective evidence at the exact reviewed revision: the applicable negative and refusal cases, import/boundary checks, secret-isolation and public-projection checks, and release/update integrity checks **once those capabilities exist**. An unrun check is reported `NOT RUN` or `NOT APPLICABLE` with its reason, never `PASS`, and a defect is never treated as resolved without tested evidence.

**Reassessment.** Revisit these boundaries whenever a dependency, framework, adapter or provider appears or changes, a deployment or release/update mechanism is introduced, a trust boundary moves, or a release candidate is reviewed. Response cadence, triage ownership, supported versions and provider choices remain Owner decisions in the packages that already own them ([`plan-roadmap.md`](../WIP/plan-roadmap.md) `R00.G`, `R10`, `R11.A`); this section invents no such commitment.

## 6. Validation and acceptance approach

These are **proposed verification obligations**, not passing tests:

| Area | Evidence the implementation should supply |
| --- | --- |
| Exactly one Entity | A valid one-Entity Installation succeeds; a second Entity is refused |
| Expandable EFS | Multiple Facets and Sites render correctly; duplicate IDs are refused in the correct scope |
| Page/locale integrity | One Page has multiple real representations; no cross-Page/Site fallback |
| Property cascade | Direct-ancestry additive `inheritance`/`currentContext` resolution; absent declaration inherits; `suppress` blocks inherited contributions but retains valid local values; per-value provenance, effective cardinality and invalid-declaration refusal verified |
| Location independence | Route results unchanged when place properties change; locale and location never determine each other; Australia → Indonesia → Bali/Jakarta cascade demonstrated |
| Routing integrity | Host-only and prefixed base URLs configured at creation, later config-only URL change, correct prefix handling/canonicals; unknown/colliding claims fail; an insensitive public claim resolves mixed-case requests to one lowercase canonical Page while a sensitive claim preserves authored capitalization and distinct Pages; sensitive/insensitive overlap, public/unlisted and Unicode-normalization collisions are refused; locale segments normalize independently of Page-route case |
| Safe authoring | Malformed JSON/Markdown, unsafe URLs and invalid assets are refused or safely rendered |
| Public projection | Private or suppressed inherited facts never leak into HTML, metadata, scripts or indexes |
| SEO and discovery | Canonicals, sitemap, unlisted `noindex` and `hreflang` reflect real published representations; no unlisted discovery leakage |
| Graphic cascade and shared library | Shared generic assets are available throughout one Installation; graphic roles use additive direct-ancestry resolution with effective role cardinality; `suppress` may retain valid local selections but blocks inherited values and automatic fallback; typed Entity/generic sources remain distinct |
| Visitor quality | Accessible keyboard/mobile behavior and actual served-page smoke evidence |
| Lifecycle | Independent release vs authored content; deterministic build; deployment/recovery plan |
| Hexagonal boundary | Dependencies point inward only; the domain core carries no framework, transport, storage or provider imports; use cases run through explicit ports with replaceable adapters wired at a composition root |
| Contact scope | The included safe `mailto:` action and the **simple** contact form validate input and report honest delivery status; no advanced contact-form subsystem is present in any form |

At implementation time, tests must exercise both the ordinary one-Facet/one-Site path and an expanded multi-Facet/multi-Site case. A green unit gate is not a substitute for observing the user-facing rendered result.

### 6.1 Integrated validation, module compatibility and migration (`R00.D` `D12`)

**Owner-accepted.** One versioned core manifest schema plus governed, versioned subordinate contracts — content structure, property definitions, graphic roles and advanced JSON modules — are enforced by **one coordinating validation process** that covers structural syntax and every graph, source, routing, ancestry, exposure, media and capability rule. Registered extensions never become separate publication authorities, and no arbitrary code is loaded.

- **Module contract.** A module declaration names a registered module and supplies typed values; its contract records the stable name, version, typed inputs with defaults and bounds, permitted output and capabilities, required application ports, accessibility and safety restrictions, and platform compatibility.
- **Hexagonal independence.** Domain and application contracts remain independent of frameworks and providers (§5); a module never owns EFS or publication decisions.
- **Fail closed.** An unknown module, an incompatible version, an undeclared field or an unsupported schema fails closed with an actionable, safe diagnostic.
- **Additive safety.** Adding an optional compatible module must not change unrelated installations, while changing a supported module version requires explicit compatibility proof.
- **Migration.** A breaking schema or module change requires a new major contract version and an explicit reviewed migration; IDs, routes, properties and source claims are never silently rewritten.
- **Separate version identities.** The manifest schema, the JSON-content schema, the module contracts, registry revisions and the Solo product release are separately identified and never silently collapse into one another.
- **Whole-publication proof.** The whole published graph — including unlisted representations — is validated with exactly one selected source per representation, safe media and a public-only projection before activation, and there is no partial publish on error. A previously validated deployment may continue serving if a new candidate is rejected under later deployment policy, and no successful publish is claimed without complete proof.

**Mandatory validation groups:** (1) strict UTF-8 JSON and a supported schema; (2) one valid published EFS ownership graph; (3) explicit, collision-free JSON and Markdown source candidates with exactly one selected safe source; (4) a safe validated body with registered modules; (5) additive property cascade, location facts, graphics and public-only projection; (6) locale, base, alias and all route collision checks; (7) safe public assets, links, metadata and discovery; (8) atomic all-or-nothing candidate disposition. Diagnostics identify the affected EFS node or field and the reason, never a sensitive path. The exact ordering may be optimized only while preserving full coverage.

**Physical literals — settled by the accepted first profile (`O06`/`O11`; §6.2F, §6.2K).** Version matching is an exact enumerated tuple with no ranges, wildcards or `latest`; the registry is product-distributed, read-only and deny-by-default; the mandatory validation stages, the closed diagnostic envelope, the enumerated mandatory diagnostic codes with family fallbacks, the immutable candidate snapshot, atomic activation and the Owner-reviewed migration procedure are fixed in §6.2K. Implemented validators and test gates remain Stage 01/02 implementation work under the §6A selection (§6A.6, §6A.13).

### 6.2 Accepted first physical publication profile — `O01`–`O12` settled

**Register status.** Each of the twelve former OPEN gates is **settled** by the Owner-accepted first physical profile in §6.2A–§6.2N. This table is the single index of those decisions; the normative keys, types, enumerations, bounds, refusals and module contracts live in the referenced subsections. No residual gate below refers to a physical choice that Appendix A settles, and no other document restates these literals.

| # | Accepted contract (settled) | Normative home |
| --- | --- | --- |
| O01 | `content/publication.json`, strict UTF-8 JSON without BOM, exact `schemaVersion: "1.0.0"`, separate `1.0.0` content/registry/module versions, JSON Schema Draft 2020-12, 2 MiB manifest and per-source bounds, resource limits. | §6.2A |
| O02 | Closed five-key manifest root, closed nested Entity → Facet → Site → Page → representation grammar, `id`/`label` grammar, `sourceCandidates`, `missingTranslation`, `route`, structured qualified targets. | §6.2B |
| O03 | Portable lowercase-ASCII locators ≤240 bytes/≤8 segments, namespace roots, reserved-name refusal, no-follow containment, immutable validated snapshot, candidate and asset collision safety. | §6.2C |
| O04 | CommonMark 0.31.2 + GFM 0.29-gfm, raw HTML refused, single H1 and heading rules, deterministic anchors, link resolution against canonical URL, `content/assets/` image base, safe scheme allowlist. | §6.2D |
| O05 | Closed JSON body grammar, `presentation` envelope and tokens, node depth/count/child limits, and the exact twenty-two `1.0.0` module contracts with closed `props` and child policy. | §6.2E |
| O06 | Product-distributed read-only module registry, dotted module-ID grammar, exact installed contract versions, deny-by-default, no remote/eval/custom module, additive updates and new-major migration. | §6.2F |
| O07 | `properties` inline on all five levels, closed two-element declaration envelope, `propertyRegistryVersion: "1.0.0"`, `public|internal` exposure, nine-key initial catalogue with cardinalities, derived provenance record. | §6.2G |
| O08 | `add`/`extend` envelopes, `factId` identity, closed `LocationFields` catalogue, WGS84 coordinates, contiguous `QualifiedOwner` lineage, whole-record exposure, per-node single extension. | §6.2H |
| O09 | `{ namespace, reference }` asset references, `platformAssetLibraryVersion: "1.0.0"`, PNG/JPEG/WebP/AVIF and MP4/WebM allowlists with byte/dimension/duration bounds, WebVTT captions, no SVG/GIF/remote fetch, zero automatic fallback. | §6.2I |
| O10 | Closed `base`/`defaultLocale`/`languagePreferences`/`entryPaths`/`route` declarations, Unicode 18.0.0 profile, case-insensitive default with opt-in sensitive claims, exact-case unlisted, derived collision ledger, typed links, derived metadata. | §6.2J |
| O11 | Exact compatibility tuple, ordered fail-closed validation stages, closed diagnostic envelope with enumerated codes and family fallbacks, immutable candidate snapshot, atomic activation, Owner-reviewed migration. | §6.2K |
| O12 | One-to-one D/O/`V`/`P` traceability, worked examples with deliberate refusals, and explicit separation of R00.D literals from `R00.E`/`R00.F`/`R00.G` obligations. | §6.2L, §6.2M, §6.2N, §6.5 |

**Disposition rule (`R00.D`).** The R00.D physical specification is now **complete and ready for administrative closure review**, not closed: closure requires the Chief Architect's independent `ACCEPT` of this revision and a separate Owner-issued administrative order. Completeness here means every authoring key, type, bound, refusal and module contract a future implementation must satisfy is written down in this Charter. It does **not** mean that schemas, validators, modules, assets, media processing, framework selection, provider accounts or tests exist; those are `R00.E`, `R00.F`, `R00.G` and later implementation packages (§6.5), and no implementation-readiness claim may exceed this paragraph.

### 6.2A Files, schema identities, encoding and resource limits (`O01`)

1. **Manifest.** The sole publication authority is exactly `content/publication.json`, a single slash-separated relative location under the Installation root. No second manifest, directory-discovery path, link or alias may serve that role. The file is UTF-8 without BOM and RFC 8259 strict: duplicate keys, `NaN`/`Infinity`, invalid UTF-8 and non-JSON syntax are refused, as are unknown fields; the root must be a non-null object and the file may not exceed **2 MiB**. Parser recovery, duplicate-key overwrite and YAML are refused. Manifest UTF-8 normalization is not implied — authored IDs, paths and locales are validated by their own rules.
2. **Bodies.** A JSON Page body is strict UTF-8 JSON with mandatory root `contentVersion: "1.0.0"`, **maximum 2 MiB** per selected source, unknown fields refused and no execution. A Markdown body is UTF-8 without BOM, **maximum 1 MiB**. Authored data and delivered assets live under different roots and can never override one another.
3. **Namespace roots**, relative to the verified Installation or product package: `content/pages/` for Page candidate bodies; `content/assets/` for Entity-owned assets; `content/drafts/` for drafts that are never scanned or published; product-distributed read-only `platform/assets/` for shared generic public assets; product-distributed `contracts/` for validated schemas and module registries — never publication manifests. Generated immutable outputs live in a separate output namespace that is never an input root, and secrets or server-only configuration live outside all of them.
4. **Candidate locators** are relative to `content/`, must begin `pages/` and must carry the selected extension: `pages/main-home.en-001.md` resolves to `content/pages/main-home.en-001.md`, and no candidate may reference the manifest itself. Entity asset `reference` strings are relative to `content/assets/` and platform asset `reference` strings to product `platform/assets/`. Physical mapping never guesses a suffix and never scans a directory.
5. **Future artifacts and dialect.** The first implementation must supply `contracts/publication/1.0.0.schema.json`, `contracts/content/1.0.0.schema.json`, `contracts/properties/1.0.0.schema.json` and one registry/schema per named module at `contracts/modules/<module-id>/1.0.0.schema.json`. These are future code-stage paths and **are not created by this Charter**. The dialect is **JSON Schema Draft 2020-12**; a conforming validator must enforce `unevaluatedProperties: false` (or an equivalent complete closed evaluated shape) and asserted formats through explicit domain checks or format assertion, because a keyword silently treated as an annotation never satisfies a security assertion.
6. **Version strings.** Every schema-version value is `MAJOR.MINOR.PATCH` with three decimal numeric components, no prefix, no leading zero and no range expression in authored data. Manifest, content, property-registry, module and platform-library versions are independent. `1.0.0` is the first exact supported baseline and implies no installed software; code generation, validator dependency and build tooling belong to `R00.E`/Stage 02.
7. **Resource limits** for contract validation: ≤10,000 EFS nodes including representations, ≤500 Pages, ≤100 representations per Page; ≤250 module nodes per JSON body, module nesting depth ≤12 and ≤100 entries per property across an effective direct-ancestry result unless a narrower registered bound applies. A refusal uses exact constrained field or path diagnostics with no partial publication, automatic source rewrite or lossy truncation.
8. **No implied publication.** A data file not referenced by a validated manifest declaration has **zero** publication effect even when it sits inside the content tree, and the platform generic library never becomes a second authoring or publishing authority.

### 6.2B Manifest grammar, EFS identity, publication and source declarations (`O02`)

**Root.** Exactly five required keys and nothing else: `schemaVersion` (string), `propertyRegistryVersion` (string), `platformAssetLibraryVersion` (string), `base` (object), `entity` (object).

**`base`.** `canonical` is required and, exactly like every element of `aliases`, is a **validated absolute HTTP or HTTPS base URL** with a DNS hostname or IP plus the authority and prefix restrictions of §6.2J.1: no credentials, query, fragment, control character or encoded separator, an IDNA-conformant lowercase canonical host with default ports normalized, and a 0–5 segment ASCII-safe prefix. A `http:` base is **schema-valid authored data** and is not restricted to loopback or to a development-only configuration; HTTPS remains strongly recommended, and a separate deployment security policy may disallow non-HTTPS deployment without redefining this authored schema. The manifest never derives authority from the incoming `Host` header or any forwarding header, and the canonical prefix is applied exactly once. `aliases` is optional (default `[]`) and holds 0–8 distinct canonical URL bases. `landing` is an optional qualified Page reference; omitting it makes the base entry return not-found. Canonical and alias normalization, collision refusal and redirect processing are §6.2J.1.

**Nested EFS nodes.** `entity` has required `id`, required `label` (non-empty plain display string), required `facets` (non-empty array) and optional `properties`. A Facet has required `id`, `label`, `sites` (non-empty array) and optional `properties`. A Site has required `id`, `label`, `defaultLocale` (full region-qualified tag), required `pages` (non-empty array), optional `languagePreferences` (object mapping accepted request-preference strings to authored full locales) and optional `properties`. A Page has required `id`, `label`, `representations` (non-empty array), optional `visibilityDefault` (`public|unlisted`, default `public`), optional `entryPaths` (array of authored locale-less paths, default empty), optional `missingTranslation` (policy object) and optional `properties`. A representation has required `locale`, required `route` (object), required `sourceCandidates` (object), optional `visibility` (`public|unlisted`) and optional `properties`. **Direct node containment is parentage:** a `parentId`, ancestor pointer, type discriminator or secondary linked graph is forbidden.

**Identity.** `id` matches `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` and is 1–63 ASCII characters. `label` is plain Unicode text of 1–120 scalar values after NFC normalization; labels never create identity, route, locale or location. Facet IDs are unique in the Entity, Site IDs unique within their Facet, Page IDs unique within their Site, with exactly one Entity. Representation locales are unique per Page after canonical BCP47 normalization, and every child has exactly one containing parent.

**Qualified targets** (`landing` or a typed link target) are an object `{ "entity": "<id>", "facet": "<id>", "site": "<id>", "page": "<id>" }` with all four fields required and no shorthand; `entity` must equal root `entity.id` and the full lineage must resolve exactly. A qualified representation target adds required `locale`. Bare Page IDs and guessed Site selection are refused.

**`sourceCandidates`.** One or both optional members `json` and `markdown`, string locators, never `null`, never an empty object, no additional keys. `json` must end `.json` and `markdown` must end `.md`, and both must point beneath `pages/`. JSON-present wins even when Markdown is valid; only genuine JSON absence selects the declared Markdown. Any existent invalid JSON rejects; unselected Markdown contents are not parsed, although its locator declaration and uniqueness claim are still validated. Neither candidate existing rejects. The same canonical candidate claimed by distinct representations rejects, including an absent path, and case-folded, alias or hardlink collisions refuse. Candidate selection never moves EFS identity, routes or properties and never confers publication on a draft or unlisted representation. A generated or remote source can never be designated.

**`missingTranslation`.** Exactly one of `{ "mode": "default" }` (also the default when the field is absent), `{ "mode": "strict" }`, or `{ "mode": "locale", "fallbackLocale": "<full locale>" }`. In `default` the Site default representation of the **same Page** must exist and be public for a fallback to succeed; otherwise the request is not-found and an authored fallback that cannot be satisfied is refused. In `locale` the `fallbackLocale` must be an existing public representation of the same Page, validated at publish time. In `strict` a missing representation is not-found. An actually present public representation always wins, and missing or unlisted content never silently swaps in another Page or Site. Redirect statuses are §6.2J.8.

**`route`.** Required `path` (full locale-qualified canonical path relative to the configured base); optional `caseMatching` (`insensitive|sensitive`, allowed only on public claims and **required to be omitted** on effectively unlisted ones); optional `aliases` (array of `{ "path": "…", "caseMatching": "insensitive|sensitive" }`, empty by default). `entryPaths` are authored locale-less public route strings and may be empty even for a Home Page. An effectively unlisted representation may not author route aliases — only its exact canonical route stays directly reachable — and no public entry path may expose it.

**Closed keys everywhere.** Every object key not enumerated here, or not expressly registered for the corresponding property or module `props`, is refused — including `draft`, a file-level `visibility`, a source-level `locale`, meta-overrides and unregistered section types. Array order is authored and is **never** a collision-resolution priority; every acceptance or refusal decision is deterministic and independent of directory order.

### 6.2C Portable sources, asset paths, snapshots and collision safety (`O03`)

1. **Locator form.** A locator or reference is a **single unescaped logical path string**, never a URL: relative slash-separated segments with no leading or trailing slash, no consecutive slashes, no backslash, no `.`/`..` segment, no colon, tilde expansion, query, fragment, percent escape, NUL, control character, encoded separator or absolute/drive/UNC path. Source components are **lowercase portable ASCII** matching `[a-z][a-z0-9._-]*`, 1–80 bytes, no trailing dot or space, no consecutive dots, at least one letter; a full reference is ≤240 bytes with depth ≤8. `.json`, `.md`, image and video extensions are lowercase, exact and dot-prefixed. Unicode is supported in **public routes and displayed content**, not in source filenames for this first portable profile.
2. **Reserved and aliasing forms.** Windows reserved basenames/stems (`con`, `prn`, `aux`, `nul`, `com1`–`com9`, `lpt1`–`lpt9`) are refused even with extensions, as are platform-specific aliases and case-insensitive equivalences. A logical path claims exactly one canonical resource, and an invalid path is never normalized into validity.
3. **Containment and snapshots.** The source adapter must use an atomic, no-follow, directory-confined traversal under the correct root: symlinks, junctions and reparse points are refused along every component and at the leaf, together with mount/root escapes, directory aliasing, unintended hardlink identity-sharing and race/TOCTOU uncertainty. When containment or canonical ownership cannot be proven the adapter refuses; **a string-prefix test never proves containment**. Source validation operates on one consistent snapshot — immutable byte copies or file-descriptor-backed reads with verified metadata and content hash — before activation, and a source changing during validation invalidates that candidate snapshot and requires a new full attempt.
4. **Claim uniqueness.** Canonical candidate claims are compared by root namespace, exact portable path and underlying filesystem identity/alias/case equivalence; two distinct published representations can never reserve one file, present or absent. Absence is a first-class, safely confirmed result and never a catch-all for permission or I/O errors. Unselected Markdown body contents are not parsed when valid JSON is effective, but its declared locator and ownership are validated even when absent.
5. **Namespaces never overlap.** Entity and platform namespaces are disjoint. A graphic or media `reference` includes the **full relative filename with extension** and resolves without globbing, inferred suffixes, MIME-only substitution or remote fetching. All authoring and discovery output derives from declared, validated references and routes; generated outputs are never read back as original sources.

### 6.2D Versioned Markdown, HTML policy, headings, anchors and author links (`O04`)

1. **Profile.** CommonMark 0.31.2 core plus the GitHub Flavored Markdown 0.29-gfm extension grammar for tables, strikethrough, autolinks, task-list markers and raw-HTML rejection as below. The profile is pinned so behaviour cannot drift with whatever parser an implementation happens to install. No footnotes, math, diagrams, directives, MDX, Solo-specific block syntax or front-matter publication controls in the first profile; standard blockquotes, lists, code fences, headings, emphasis, links and images remain fully supported.
2. **Raw HTML is refused as content** — not interpreted, sanitized or passed through: a parser-recognized raw-HTML block or inline raw-HTML node is an authoring diagnostic that blocks publication, so literal HTML examples must sit in fenced or inline code to render as escaped text. Parser HTML output is disabled and plain text is escaped in the final renderer. No executable code in fenced code blocks. Task-list checkboxes are read-only accessible presentation, never form controls with submission or persisted state.
3. **Heads and titles.** Standalone Markdown must contain exactly one non-empty H1 providing the body display title, and it must be the first non-blank semantic block (a comment is raw HTML and therefore invalid). H2–H6 may follow with no jump greater than one level; a second H1 refuses. JSON content uses its mandatory root display title and may use H2–H6 modules beneath it, and no renderer may mint a duplicate visible H1. This title is content display text, never EFS identity or canonical-URL authority, and no front matter may override plain summary or SEO properties, which are governed separately by §6.2G.
4. **Anchors.** A heading anchor derives solely from visible heading text: NFC normalize, lowercase under the pinned §6.2J.4 profile, keep Unicode letters and numbers plus hyphens, collapse each whitespace run to one `-`, strip other punctuation and trim hyphens; an empty result becomes `section`, and repeats take `-2`, `-3`, … in document order. Every `#fragment` target must equal one generated real anchor, and an empty or missing target refuses. Renderer IDs stay safe and unique, and unlisted fragment links never create discovery inventory.

5. **Links.** An ordinary `[text](reference)` resolves under RFC 3986 URI-relative rules against the **current canonical Page URL**, never against a disk source path. A leading `/` is origin-root relative and must resolve inside the configured approved base prefix; a plain relative reference follows standard URL resolution and then passes the D07 ledger and target-visibility classification. Internal targets must be authorized published routes — an explicit safe link to a known unlisted route is permitted but never auto-generated — and unknown, ambiguous or out-of-base internal targets refuse. Fragment-only references are same-Page. There is no automatic locale switching and no guessed same-Site slug, and URLs inside code spans stay inert.
6. **Images.** An ordinary `![meaningful alt](images/example.webp)` refers **only** to the Entity asset `content/assets/images/example.webp`, whatever directory the `.md` source sits in: the author-friendly Markdown image base is exactly `content/assets/`. An image reference is a safe slash-separated portable path with no leading `/`, no `..` and no scheme, and it is validated as a typed Entity asset with known MIME and use rights. A purely decorative image may be explicitly marked with empty alt, which the renderer must then mark decorative; otherwise alt text is required. Server-side remote image fetching is forbidden and an external HTTP image in Markdown is refused.
7. **External targets.** Only safe absolute `https://`, `http://` and `mailto:` schemes are accepted as click-through navigation or action, never as server fetch. Userinfo credentials, protocol-relative `//`, `javascript:`, `data:`, `file:`, `blob:`, `vbscript:` and unsafe or malformed control/percent encodings are refused. `mailto:` permits one validated addr-spec recipient and an optional governed `subject` parameter only, with no CR/LF injection, no secrets and no auto-send; no send result is claimed. `tel:` and custom schemes are not approved in this profile. An external `target=_blank`, if used, requires `rel="noopener noreferrer"`, and nothing implies an external destination is trusted.
8. **Diagnostics.** A Markdown diagnostic carries the safe EFS lineage, the source locator and the line and column of the actually selected content, and never a raw server-private root, a secret or untrusted HTML in an error page.

### 6.2E Advanced JSON body grammar, module envelopes and the first installed module catalogue (`O05`)

#### 6.2E.1 Body, node and presentation grammar

**Body root** — exactly: mandatory `contentVersion: "1.0.0"`; mandatory non-empty plain-string `title` (1–140 Unicode scalar values); optional plain-string `summary` (≤500 scalar values); mandatory `blocks` array of **1–250** nodes. The single visible H1 comes from `title`: when the first block is `content.hero` the renderer places that root H1 inside the hero, otherwise it places it once before the first block. A hero never declares a second H1, and `summary` is body copy that never changes manifest-owned metadata, routing, publication, locale or visibility. All authored text is inert escaped data — no interpolation, HTML execution, JS/CSS injection, remote include or template evaluation. The node count includes nodes nested in tabs, accordion and modal items.

**Node** — a closed object with mandatory `module` (registered dotted lowercase ID), mandatory `contractVersion` (exact version string), mandatory `props` (closed typed object), optional `children` (only where the registry type expressly permits child nodes), optional `presentation`, and optional `span` (only as a child of `layout.grid` or `layout.bento`). `children` is an ordered array of registered nodes and is forbidden for leaf modules. Maximum node depth **12**, ≤**250** nodes in total, maximum child-array length **24**, and no node may refer to itself cyclically. A missing module is never inferred from a filename or component name. Unknown node fields, unknown props, an unknown module version, forbidden children or a version mismatch refuse publication of the **whole** proposed graph.

**`presentation`** — a closed optional object: `foreground` optional `#RRGGBB` or `#RRGGBBAA`; `background` optional typed fill; `padding` optional `none|xs|sm|md|lg|xl`; `radius` optional `none|sm|md|lg`; `textAlign` optional `start|center|end`. No arbitrary CSS property, selector, `url(...)`, class name, expression or inherited unvalidated script. A fill is exactly one of `{ "kind": "solid", "color": "#RRGGBB" }`; `{ "kind": "linearGradient", "angle": 0..360, "stops": [ { "color": "#RRGGBB", "position": 0..100 }, … ] }` with 2–5 strictly increasing ordered stops starting at `0` and ending at `100`; or `{ "kind": "image", "asset": <typed asset>, "overlay": "#RRGGBBAA" }` with optional overlay. The renderer must enforce meaningful contrast and overlay/focus visibility and **may refuse** an unsafe combination rather than silently change colours. Token distances in CSS pixels are `none`=0, `xs`=4, `sm`=8, `md`=16, `lg`=24, `xl`=40; radius values are `none`=0, `sm`=4, `md`=10, `lg`=20. These remain semantic inputs, never author-supplied CSS.

**`span`** — a closed `{ "columns": { "base": 1..4, "md": 1..4, "lg": 1..4 }, "rows": 1..3 }`; at each breakpoint the requested column span must not exceed its containing grid or bento column count and may not introduce unbounded overlap. Absent `span` defaults to one column and one row per breakpoint. A responsive-columns object is a closed `{ "base": 1..4, "md": 1..4, "lg": 1..4 }` with `base ≤ md ≤ lg`; breakpoints are `md` ≥768 CSS px and `lg` ≥1200 CSS px, below `md` uses `base`, and no author-supplied CSS media query exists.

**`props`** are independently schema-closed and bounded. **R** marks a required field and every unmentioned field is rejected. "Text" means an inert string of 1–500 scalar values unless a narrower bound is stated, non-empty when required; `asset` always means a typed §6.2I reference and `link` a classified §6.2J.9 link. Arrays preserve authored order and never trigger automatic source lookup.

#### 6.2E.2 Initially registered Solo modules (`1.0.0`, exact names)

Each row's closed `props`, defaults, child policy and effects are binding; the first installed registry contains **only** these twenty-two IDs.

| Module ID | Closed typed `props` (R = required) | Child policy and effects |
| --- | --- | --- |
| `layout.stack` | `direction` optional `vertical\|horizontal` (default `vertical`); `gap` optional spacing token (default `md`); `align` optional `start\|center\|end\|stretch` (omitted default `stretch`) | **R** `children` 1–24 registered nodes; a horizontal stack must wrap safely on small viewports. |
| `layout.grid` | `columns` **R** responsive-columns object; `gap` optional spacing token (default `md`) | **R** `children` 1–24 with an optional `span` per direct child bounded to the active column count; an overflowing span refuses rather than overflowing focus or reading order. |
| `layout.bento` | `columns` **R** responsive-columns object; `gap` optional spacing token (default `md`) | **R** `children` 1–16 with optional `span` per child; auto-flow follows document order and visual placement never reorders semantics or keyboard order. |
| `layout.layers` | `minHeight` optional `sm\|md\|lg` (default `md`) on its **own closed layer-height scale** — `sm` **240**, `md` **400**, `lg` **640 CSS pixels** as a **minimum** at every breakpoint, never a maximum, fixed height or padding token; `align` optional `start\|center\|end` (omitted default `center`) placing each layer's natural-height contents on the **vertical block axis** (`start` top, `center` middle, `end` bottom) | **R** `children` 1–12; children share one relative layer region, keep authored DOM and reading order, and paint in authored order with later children visually above earlier ones — paint order never creates semantic or keyboard precedence; the horizontal axis fills the available content width; content may expand the region, but essential content must never be clipped and no mandatory control, focus target or interactive layer may be concealed, otherwise validation **refuses** instead of permitting silent overlap, clipping or a renderer-chosen fallback. |
| `content.heading` | `text` **R**; `level` **R** integer 2–6 | Leaf; no second H1 and no heading-level jump greater than one in actual document order. |
| `content.prose` | `markdown` **R** UTF-8 text of 1–20,000 scalars, parsed as §6.2D safe Markdown **without headings or active HTML** | Leaf; no front matter, no route or visibility metadata and no module directives. |
| `content.actions` | `items` **R** array 1–8 of `{ "label": text, "link": <§6.2J.9 link> }`; optional per-item `variant` `primary\|secondary\|text`, **omitted default `text`** | Leaf; links are navigational or actions, never POST/transaction/remote-fetch commands. `text` is the neutral inline/link-like treatment: it must stay visibly interactive, carry a clear accessible name and visible focus, and never silently become a primary CTA. `primary`/`secondary` are author-selected relative visual-emphasis variants only — never separate link authority, destination or HTTP method — and no variant overrides navigation, exposure, accessibility or routing constraints. |
| `content.hero` | `eyebrow` optional text; `tagline` optional text (≤500); `action` optional closed `{ "label": text, "link": <link> }` | Leaf; the root Page `title` supplies the visible H1. `presentation.background` accepts hero fills including validated media assets. |
| `media.image` | `asset` **R** §6.2I image reference; `alt` **R** 0–300 (empty only for a purely decorative image); `fit` optional `contain\|cover` (default `cover`); `caption` optional text | Leaf; responsive delivery derives from the validated source with no layout shift lacking dimensions. |
| `media.video` | `asset` **R** video reference; `poster` **R** image reference; `captions` **R** WebVTT reference; `captionsLocale` **R** full BCP47 region-qualified locale; `caption` optional visible text; `controls` **R** literal `true`; `autoplay` optional literal `false` only; `loop` optional boolean (default `false`) | Leaf; playback is an explicit visitor action with validated captions, keyboard controls and accessible fallback, and no external embed or remote URL. |
| `media.gallery` | `items` **R** array 1–24 of `{ "asset": <image>, "alt": 0–300, "caption": optional text }`; `columns` optional 1–4 (default 3) | Leaf; responsive presentation that never navigates automatically into unlisted content. |

| `media.carousel` | `slides` **R** array 2–12 of `{ "asset": <image>, "alt": 0–300, "caption": optional text }`; `controls` literal `true` and always present; `autoplay` optional boolean (default `false`); `intervalMs` permitted **only** when effective `autoplay` is `true`, integer 6000–20000, and when `autoplay` is `true` with the field omitted its default is exactly **10000 ms**; with `autoplay` false or absent a declared `intervalMs` is prohibited and **refuses publication** rather than being ignored; **slide index 0** is active on initial render, Previous/Next move exactly one authored slide and wrap cyclically, and autoplay, hover/focus and Pause behaviour is the carousel paragraph below | Leaf; always-visible pause/previous/next controls, pause on focus or hover, reduced motion disables autoplay and transitions, and no script from Page data. |
| `design.watermark` | exactly one of `text` (1–80) or `asset` (image), `opacity` **R** number 0.05–0.4, `position` optional `center\|topLeft\|topRight\|bottomLeft\|bottomRight` (omitted default `center`) | Leaf decorative layer ignored by screen-reader content; never the sole source of essential text and never a contrast break. |
| `design.motion` | `effect` **R** `fade\|slide\|reveal\|marquee\|parallax`; `durationMs` optional integer 100–3000, omitted default **800** for `fade`, `slide`, `reveal` and `parallax` and **3000** for `marquee`; `direction` optional `up\|down\|left\|right` **only** for `slide` or `marquee`, omitted default `up` for `slide` and `left` for `marquee`, and prohibited on `fade`, `reveal` and `parallax` where it **refuses publication** rather than being ignored | **R** `children` 1–8; all content present without animation, reduced motion renders every effect statically, and a marquee pauses on focus and is never essential. |
| `interaction.tabs` | `tabs` **R** array 2–8 of `{ "id": <technical ID>, "label": text, "content": [1..12 nodes] }`; `initialTab` **R** one exact member ID | No top-level `children`; nested nodes live only in each tab's `content`. Real keyboard tablist with roving focus and visible labels/panels; each tab ID unique within the node. |
| `interaction.accordion` | `items` **R** array 1–12 of `{ "id": <technical ID>, "title": text, "content": [1..12 nodes] }`; `singleOpen` optional boolean (default `true`); `initialOpen` optional and, when omitted, **all panels start collapsed**; when present it must name one actual item ID | No top-level `children`; correct disclosure/ARIA and keyboard behaviour; hidden panels never hide public Pages from discovery. |
| `interaction.modal` | `triggerLabel` **R** text; `content` **R** array 1–12 nodes; `closeLabel` optional text (default `Close`) | No top-level `children`; the dialog traps focus while open, closes by Escape and a visible close control, restores focus, and never overlays an authentication gate. |
| `content.stats` | `items` **R** array 1–12 of `{ "label": text, "value": text }` | Leaf; static authored facts only — no unapproved analytics, timer or network call. |
| `content.testimonials` | `items` **R** array 1–12 of `{ "quote": text ≤1000, "attribution": text ≤160 }` | Leaf; attributed authored content only — no inferred reviews and no fetched third-party claims. |
| `content.pricing` | `plans` **R** array 1–6 of `{ "name": text, "priceText": text, "features": [1..15 text], "action": optional { "label": text, "link": <link> } }` | Leaf; presentation only — never checkout, billing or payments. |
| `content.timeline` | `items` **R** array 1–24 of `{ "label": text, "description": text ≤1000, "when": optional text }` | Leaf; chronological display exactly as authored, never a location or timezone engine. |
| `interaction.contact` | `labels` **R** closed `{ "name": text 1–80, "email": text 1–80, "message": text 1–80, "submit": text 1–80 }`; `notice` **R** closed `{ "label": text 1–160, "link": <existing classified typed link resolving to a public Page representation or a validated `https:` privacy-information URL> }`; `feedback` **R** closed with exactly the seven keys `invalidRequest`, `notConfigured`, `rateLimited`, `acceptedForDelivery`, `rejectedDownstream`, `temporarilyUnavailable`, `indeterminate`, each text 1–240 | Leaf; the sole simple-form Page-enablement mechanism (§6B.8). No children, custom action URL, recipient, address, provider, credential, upload, conditional or multistep form, CRM or executable content; at most one node per representation. |

A future module expands the library **only by a new registered contract** — richer comparisons, advanced layered heroes, filters, video variants, responsive art direction and new interaction patterns included — without imposing a ceiling on JSON's design vocabulary, but the first `1.0.0` installed registry holds **only** the twenty-two IDs above. Listing a future capability is not installing it. Module inputs can never express an arbitrary plugin, dynamic HTML/JS/CSS, server-side fetch, custom event handler, query execution, form builder, CRM, commerce or unapproved provider integration.

**First-profile optional-value rule.** Within the twenty-two contracts above, a non-behavioral optional item — `caption`, `tagline`, `action`, `eyebrow`, `when`, or a presentation override — is simply **absent** when omitted: no authored content, link, SEO value, asset, route claim or extra control is synthesized in its place. An optional numeric or enum item carries only the explicit default written in its row (`gap` = `md`, `fit` = `cover`, `singleOpen` = `true`, `loop` = `false`, `autoplay` = `false`, action `variant` = `text`, `minHeight` = `md` as a 400 CSS-pixel minimum, `align` = `stretch` on `layout.stack` and `center` on `layout.layers`, watermark `position` = `center`, `autoplay`-true `intervalMs` = `10000`, `durationMs` = `800` or `3000` for `marquee`, `direction` = `up` for `slide` and `left` for `marquee`), and no adapter may substitute a different default. An omitted accordion `initialOpen` means every panel starts collapsed. A prop that is unrecognized, or authored where its row prohibits it — `intervalMs` with autoplay false or absent, `direction` on `fade`, `reveal` or `parallax`, `span` outside `layout.grid`/`layout.bento` — **refuses publication** and is never silently ignored.

**Observable motion meaning (`design.motion`, first profile).** `fade` reveals from opacity 0 to 1 on first viewport entry. `slide` enters from a **16 CSS-pixel** offset opposite its declared or default direction and settles in its document-flow position. `reveal` uncovers its content **top to bottom** on first entry. Those three run once, never repeat, and keep the entire underlying content accessible even before entry. `marquee` presents its content statically until the visitor starts motion with an **always-visible Start/Pause control**; once started it scrolls the child track continuously in the declared or default direction, consuming `durationMs` for each **24 CSS pixels** of travel, and pauses on pointer hover, keyboard focus or Pause. `parallax` follows the exact normalized scroll-progress mapping, ±24 CSS-pixel bound and linear interpolation defined in the paragraph immediately below, not any time-based autoplay. All five effects render **statically** under a reduced-motion preference, no essential content depends on movement, no keyboard focus is hidden or trapped, and no authored JS/CSS is accepted. The concrete animation framework remains a replaceable adapter choice; these visible meanings and defaults are normative.

**Parallax progress and smoothing (`design.motion` `effect: parallax`, first profile).** For an element whose rendered bounding rectangle has viewport-relative top `T` and positive height `E` in a viewport of positive height `H`, the clamped normalized progress is exactly `p = clamp((H - T) / (H + E), 0, 1)` — `p = 0` when the element's **top** reaches the viewport **bottom**, and `p = 1` when its **bottom** reaches the viewport **top**. The desired vertical translation is exactly `y = (2p - 1) × 24` CSS pixels, so `p = 0` gives **−24 px**, `p = 0.5` gives **0** and `p = 1` gives **+24 px**. `durationMs` keeps R02's **800 ms** default and authored **100–3000 ms** range, and now means the time for a **linear interpolation from the currently displayed translation to the latest desired translation**, restarting from the current interpolated position whenever the target changes, with **no overshoot**. Parallax has **no** `direction` prop and authoring one refuses. Nonpositive or nonfinite element or viewport geometry renders the element **statically** rather than dividing by zero or emitting an unsafe transform. Under a reduced-motion preference, or when no scroll-capable viewport exists, translation is exactly **0** and all content remains visible and keyboard-accessible.

**Carousel initial, manual and autoplay behaviour (`media.carousel`, first profile).** On initial render **slide index 0** — the first authored `slides[]` item — is active. Previous and Next each move exactly **one** authored slide and **wrap cyclically**: `previous` from index 0 activates the last slide and `next` from the last activates index 0. Non-active slides never expose hidden focusable controls and are never announced as current content, while author-supplied `alt` and `caption` semantics are preserved. Controls stay present and keyboard-operable with the accessible names `Previous`, `Next` and a stateful `Start`/`Pause`. When `autoplay` is omitted or `false`, advancement never starts automatically; the Start control may begin a visitor-requested slideshow at the **same 10000 ms** default when no `intervalMs` is authored, and no `intervalMs` field is permitted in the authored object unless effective `autoplay` is `true`. When `autoplay: true` is authored, advancement begins automatically at the R02 validated or derived interval **only when reduced motion is not requested**, each advancement resets the complete interval, and cycling wraps. Hover or focus temporarily pauses a running carousel and it resumes when that hover or focus ends **only if** the visitor has not manually paused; pressing Pause or activating Previous/Next stops automatic advancement until the visitor explicitly presses Start again. Under a reduced-motion preference the Start/Pause control stays **visible but cannot enable automatic cycling**, while manual Previous/Next remain operable. No undeclared asset is preloaded or fetched, and off-screen focus never leaks through inactive slides. Slide validation, the 2–12 bound, `controls: true` and every R02 `intervalMs` positive and refusal rule are unchanged.

**Layer geometry and painting (`layout.layers`, first profile).** `minHeight` is its own closed layer-height scale, not a spacing or padding token: `sm` = **240**, `md` = **400** and `lg` = **640 CSS pixels**, applied as a **minimum** at every breakpoint unless larger natural content expands the region, so the omitted default `md` means a minimum of 400 CSS pixels. The scale never implies a maximum, a fixed height or a padding value, and a breakpoint-dependent height token would require a new versioned contract. `align` positions each layer's natural-height contents on the **vertical block axis** within that shared region — `start` top, `center` middle, `end` bottom — while the horizontal axis fills the available content width; no independent cross-axis enum, raw `z-index` or authored CSS field exists. Children keep authored DOM and reading order and paint in authored order with later children visually above earlier ones, so paint order never creates semantic or keyboard precedence. When a layer set cannot be rendered accessibly — obscuring mandatory controls or essential content, clipping essential material or leaving a hidden focus target — validation refuses it (`MODULE_INVALID_PROPS`) rather than allowing a renderer-chosen overlap, clipping or fallback.

#### 6.2E.3 Accessibility and performance refusal boundaries

Exactly one visible H1, meaningful heading order, keyboard operability, visible focus, declared alternative text and captions, reduced-motion respect, explicit user control over animated modules, accessible tab/accordion/modal semantics, no essential content reachable only by hover, no auto-playing audio or video and no off-screen focus traps. Contrast must meet the project-approved accessibility target when `R00.G` specifies one; until then the renderer must refuse a **known unsafe** presentation rather than claim that a numeric WCAG threshold has been adopted. The eventual module implementation requires proportional per-module browser, keyboard and assistive-technology tests plus layout-shift and resource tests — this Charter produces specifications, never claims of passing them.

### 6.2F Module trust, installation, contract pinning and upgrade behaviour (`O06`)

1. **Eligible modules only.** Only **product-distributed, explicitly registered Solo modules** are eligible. The registry is a read-only product component whose built-in IDs and versions are declared in the published product contract and which **never** publishes Pages. No remote npm-style URL, package path, runtime `eval`/`import`, authored source code, network-fetched module or user-authored JS module declaration is accepted; custom modules require a separate Owner-reviewed code-bearing extension policy, never an implicit `custom` field.
2. **Identity and versions.** Module IDs are lowercase dotted technical names matching `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*(?:\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*)+$`, maximum 80 ASCII characters. Contract versions are exact three-component strings per §6.2A.6, and every module node must name an **installed compatible exact version**; wildcards, `latest`, semver ranges, silent major upgrades and nearest-version guessing are refused. A product may support several exact module contract versions concurrently with version-specific validators and adapters, and a required version that is unavailable fails publication.
3. **Registry entry content.** Each entry binds an ID plus exact contract version to its **closed props schema**, default values, child and nesting rules, `presentation` and media capabilities, required application ports, public projection and accessibility/security contract. Only defaults expressly listed in §6.2E.2 exist; otherwise a missing required prop refuses. A registry entry can never alter EFS identity, routes, property inheritance, source choice or visibility.
4. **Product updates.** An update may add optional modules or a new compatible contract without changing the published behaviour of an installation that does not select them, and a compatible minor or patch release is not authorized to reinterpret existing serialized inputs, default values or security posture. An incompatible input or behaviour change requires a **new major** contract, the preserved old decoder where supported, and an explicit Owner-reviewed migration that revalidates the manifest and every affected content file before activation. A removed required contract causes refusal, never substitution.
5. **Dependency rule.** The domain core and application use cases define EFS and validation policy, public projection and named ports; read-only source adapters, asset adapters, module renderer adapters and request adapters implement those ports and are composed at a single root. Modules receive only validated, public-projected props and can never reach filesystem, network or secret stores by escaping the module port. A module needing a newly privileged external integration cannot ship by adding a JSON ID; it needs its own approved product, port and security contract.
6. **Release honesty.** §6.2E.2 is the exact first built-in `1.0.0` catalogue and §6.2K is manifest/content/registry compatibility. A product release may implement that catalogue incrementally in later authorized packages, but it **cannot claim a complete R00.D-conformant public JSON authoring implementation while any mandatory declared module lacks its contract or fails the accepted tests**. A deliberately smaller release needs a newly Owner-accepted supported-capabilities profile, never silent omission.

### 6.2G Governed property catalogue, envelopes, exposure and provenance (`O07`)

1. **Registry pin.** Manifest root requires `propertyRegistryVersion: "1.0.0"`, versioned independently of `schemaVersion`, and the installed product must register that exact contract. Every Entity, Facet, Site, Page and representation may carry an optional direct `properties` object; property names are exact registered keys and an unknown key rejects the whole publication.
2. **Declaration envelope.** Each property declaration is a **closed** `{ "inheritance": "inherit"|"suppress", "currentContext": "none" | [ <typed values/operations> ] }`; both fields are required whenever the property is declared. An array must have length ≥1, and the literal string `"none"` expresses no local contribution —an empty array and `null` are invalid. Absence of the whole property key implicitly inherits. `inherit` takes prior effective values in stable inherited-first then local order; `suppress` drops inherited values but **may** add local values. Domain validation applies the registry's effective maximum and uniqueness rules after resolution, and there is no nearest-value-wins, silent de-duplication, merge or post-filter fallback.
3. **Exposure.** Exactly two exposure values exist: `public` and `internal`. Every **new independently authored** property value or fact carries an explicit `exposure`. A registry entry declares its maximum allowed exposure (`public` or `internal`) and its permitted EFS levels, and data can never elevate from `internal` to `public` through inheritance, extension, module, renderer, alias or omission. `internal` describes **runtime output projection**, not secrecy of public source files, and no per-field public-visibility exception exists for location in this profile.
4. **Initial catalogue** — exact keys, entry value shapes, effective cardinalities and allowed levels:

| Key | New local entry value | Effective cardinality | Allowed levels / notes |
| --- | --- | --- | --- |
| `location` | §6.2H typed `add` or `extend` operation envelope | 0–16 distinct effective facts | All five; an `extend` updates an identified inherited fact in place and never becomes a seventeenth fact |
| `graphic.logo` | `{ "source": <§6.2I image asset>, "exposure": "public"|"internal" }` | 0–1 | All five; inherited plus local without `suppress` refuses |
| `graphic.favicon` | Same envelope, favicon-permitted format | 0–1 | All five; never an unverified remote image |
| `graphic.banner` | Same envelope | 0–1 | All five |
| `graphic.background` | Same envelope | 0–4 | All five; inherited-first layered order |
| `graphic.social-preview` | Same envelope | 0–1 | All five; safe public metadata projection only |
| `seo.title` | `{ "text": <plain text 1–140>, "exposure": "public"|"internal" }` | 0–1 | Site, Page, representation only |
| `seo.description` | `{ "text": <plain text 1–300>, "exposure": "public"|"internal" }` | 0–1 | Site, Page, representation only |
| `contact.email` | `{ "address": <valid mailbox>, "exposure": "public"|"internal" }` | 0–1 | All five; implies no contact submission endpoint |

Every other property name is unregistered in the first release. The catalogue expands only by explicit versioned registry approval, never by adding an implicit manifest publisher. A registered exposure maximum controls which projection may contain a value; for the types above both `public` and `internal` are permitted, and `internal` values stay out of all publicly rendered HTML, JSON, metadata, feeds, scripts and module props. `location` follows whole-record exposure (§6.2H.6), and a private graphic selection can never be resurrected by a public fallback.

5. **Equality and uniqueness.** Each effective location `factId` is unique per original qualified owner and ancestry, and repeated identical facts or duplicate extensions violating §6.2H reject. For graphic, SEO and contact values, local and inherited records are counted **before** public projection, the maximum is enforced on the full effective collection, and duplicates refuse where the registry marks uniqueness — removing private values from public output must never mask a cardinality error. `graphic.background` permits up to four ordered layers, but a duplicate exact source reference inside one effective background collection refuses as authoring ambiguity.

6. **Derived provenance (internal only).** A resolved property result is a closed derived record `{ "property": <registered key>, "values": [ <ProvenanceValue>… ], "resolutionEvents": [ <ResolutionEvent>… ] }`. `ProvenanceValue` has exactly `data` (the fully resolved typed value), `exposure` (`public|internal`), `originOwner` (the original complete qualified EFS owner), `originLocalIndex` (the zero-based original local-array index) and, **for `location` only**, optional `fieldOrigins` mapping each currently set location field name to `{ "owner": <QualifiedOwner>, "localIndex": non-negative integer }`. `ResolutionEvent` has exactly `owner` (<QualifiedOwner>), `operation` (`inherited|added|suppressed|extended`), `localIndex` (non-negative integer, or `null` for implicit inheritance) and optional `targetFactId` for `extended`; events are ordered by direct ancestry then local declaration order. An empty effective result may retain suppression events with no values. This record is a read-only diagnostic model: it is never a manifest-authored field and never exposed directly in a public response. The only renderer input is an explicit validated safe public projection that preserves authorized order and never substitutes arbitrary ancestor values after filtering. A registry maximum of `public` allows an explicitly authored `public` or `internal`; a maximum of `internal` permits only `internal`. All first built-ins permit both classifications, but `internal` values never enter public output.
7. **Closed entries.** All property entry objects are closed and type-checked. `seo.*` values never override the Page's identity, route, locale or visibility; they are ordinary authorized property values. A JSON or Markdown body may not declare authoritative SEO metadata to override the registry or the manifest. Direct media inside module props need not be a graphic role (§6.2I.5).

### 6.2H Location fact fields, stable identity, `add`/`extend` and safe geometry (`O08`)

1. **Never a place tier.** `location` is a §6.2G property and **never** an EFS node, mandatory place tier, route, locale-selection signal or geocoding input. A property `currentContext` array may contain `add` or `extend` operations. The Entity root cannot `extend` because it has no inherited value. No location fact is ever generated from a hostname, ID, route, locale or coordinates, and several facts never yield a guessed primary.
2. **`add` envelope** — exactly `{ "operation": "add", "factId": "<technical-id>", "kind": "association"|"premises", "fields": <LocationFields>, "exposure": "public"|"internal" }`. `factId` uses the accepted technical-ID grammar (1–63 characters) and is stable within its original owner. The **full original owner** — the node's complete EFS lineage, derived from containment and the representation locale where applicable — is never authored a second time. Identity is the pair `(origin qualified EFS lineage, factId)` and no different record may reuse that exact pair. `kind` distinguishes a conceptual association from premises and never implies a complete street address.
3. **`LocationFields`** is a **closed typed non-empty** object whose allowed optional members are `worldRegion` (plain text ≤80), `countryCode` (ISO 3166-1 alpha-2, uppercase ASCII, never inferred), `administrativeArea` (≤120), `locality` (≤120), `district` (≤120), `namedPlace` (≤160), `addressLine1` and `addressLine2` (≤200 each), `postalCode` (≤32) and `coordinates` (closed `{ "latitude": number in [-90,90], "longitude": number in [-180,180] }` with finite decimal WGS84 values, no elevation or accuracy inferred). For `association` every field is allowed with no postal or coordinate requirement; for `premises` a partial address is allowed and no country, postal code or coordinate is compulsory. An empty fields object refuses, and omitted geographic levels are never auto-filled.
4. **`extend` envelope** — exactly `{ "operation": "extend", "target": { "owner": <QualifiedOwner>, "factId": "<id>" }, "set": <NonemptyPartialLocationFields> }`. `QualifiedOwner` is a closed object with required `entity` technical ID and optional subsequent **contiguous** lineage `facet`, `site`, `page`, `locale` — `site` requires `facet`, `page` requires `facet` and `site`, and `locale` requires every ancestor plus a full region-qualified tag. The target must be an accessible **direct ancestor**, never the current node or a sibling, and must identify exactly one presently inherited unsuppressed location fact. `set` carries only allowed typed `LocationFields` members and may modify or append those member values; it may never remove fields, change `factId`, `kind`, origin owner or `exposure`, or append a duplicate fact.

5. **Application order.** A node's local `currentContext` operations apply in authored array order **only after** the inherited effective location result is established. At most one `extend` per target fact is allowed **at one node**, and two conflicting same-node extensions refuse rather than last-wins. A node may add independent facts and extend inherited ones, but cannot extend a fact it has just added at the same node — use a single `add` with the complete intended fields instead. A target suppressed by a local `inheritance: suppress` cannot be extended. Across descendants an already-extended inherited fact keeps its original identity, and its effective fields and field provenance follow the path without duplication. A missing, cross-branch, circular or suppressed target fails. Output order stays inherited-first with in-place updated records, followed by new `add` facts.
6. **Exposure is whole-record.** An `extend` can never supply a new `exposure`; an inherited `internal` fact stays `internal` even when the changed fields look harmless. Public projection omits whole internal facts and there is no field-level publication exception. Internal **field** provenance uses exactly the single §6.2G.6 `fieldOrigins` shape and nothing else: each currently set location field maps to `{ "owner": <QualifiedOwner>, "localIndex": non-negative integer }`, naming the node and the precise local `add` or `extend` array position that **last explicitly authored that field**. On an `add`, every populated field carries the adding node's owner and that operation's array index. When an inherited fact is extended, only the fields explicitly present in `set` receive the extending node's owner and that `extend` operation's array index, while untouched fields keep their prior origin. The operation kind is recorded **only** in §6.2G.6 `resolutionEvents`; a field origin has **no** `operation` member and no second serialized provenance shape exists. `originOwner` and `originLocalIndex` remain the original fact origin, an `extend` never erases the origins of untouched fields, and this derived record is internal only — never publicly projected. A consumer needing one specific fact must use an explicit fully qualified identified selector or fail as ambiguous: there is no declaration-order "primary" and no geographic-correctness claim.
7. **Geometry.** Validation accepts only finite WGS84 latitude and longitude inside the ranges above; **no** reprojection, reverse geocoding, distance calculation, geofence, physical-presence verification or visitor-location lookup is implied, and unknown data is never synthesized. `null`, arbitrary custom geographic keys, nested unregistered metadata, unsupported units and fabricated defaults refuse. Future custom metadata requires a separately registered typed extension; no open map is approved.

### 6.2I Graphic roles, asset source grammar, initial media formats and platform library (`O09`)

1. **Roles and reference shape.** Every graphic role of §6.2G uses D08 additive inheritance and suppression, with the §6.2G cardinalities and **no** automatic platform fallback for any role. An authored `source` is a required closed typed asset reference `{ "namespace": "entity"|"platform", "reference": "<portable-relative-file-with-extension>" }` with no `url` or external-fetch field. Module media uses the **same** typed reference directly without coercion into a cascading graphic role. The generic library is one installed read-only namespace — never an EFS node, alternate Entity or publisher.
2. **Library pin.** Manifest root requires `platformAssetLibraryVersion: "1.0.0"`, independent of manifest, registry and product versions. A platform asset resolves only inside the **exact pinned** `platform/assets/` catalogue of the running compatible product version, whose immutable catalogue must enumerate every available asset ID (the exact portable relative filename), hash, media class, dimensions and licensing/provenance; it cannot discover arbitrary files or substitute an "equivalent" generic asset. **No asset filename or artwork is mandated by this Charter:** the implementer must supply a lawful initial read-only library and publish its complete version-pinned catalogue under an Owner-reviewed asset delivery package before claiming the library is usable, and an Installation selecting an absent library version or asset is invalid rather than silently upgraded. That delivery is a later code-bearing package gate, not permission to invent artwork.
3. **Formats.** Images admitted initially are `.png` (`image/png`), `.jpg`/`.jpeg` (`image/jpeg`), `.webp` (`image/webp`) and `.avif` (`image/avif`) — no active SVG, GIF, external URL, script, HTML, font or unapproved container. `graphic.favicon` permits PNG only; logo, banner, background, social-preview, gallery, watermark, poster and ordinary inline media permit the approved images. Video modules permit `.mp4` (`video/mp4`) with H.264/AVC video plus optional AAC-LC audio, or `.webm` (`video/webm`) with VP9 video plus optional Opus audio, each verified by bounded media inspection with no inferred alternative codec. Required timed captions are `.vtt` (`text/vtt`, UTF-8 without BOM, ≤500 KiB) in the same validated asset namespace with safe WebVTT cue/timestamp parsing and no active HTML/script, CSS, remote reference or unsupported cue command. No remote embedded video, executable media or unapproved third-party iframe exists.
4. **Bounds.** Maximum source image 20 MiB with width and height 1–8192 pixels and ≤32,000,000 decoded pixels; maximum source video 200 MiB and ≤180 seconds, with metadata and decoding verified. A product may apply stricter operational limits **only** if expressly documented and compatible with this contract. Mismatched extension/MIME/magic, malformed media, decompression bombs, active payloads and unsafe dimensions are refused, and acceptance is by authoritative source bytes inspected through a bounded media adapter, never by extension alone. A corrupted **declared** asset rejects publication even when its declaring representation is unlisted, while an unreferenced asset never publishes by presence.

5. **Derived outputs and exposure.** Derived responsive variants, resized images and output URLs are generated from validated source bytes and manifest authority; they are immutable and cache-safe, provenance-linked, never overwrite input and never become new authored source or a fallback for a missing original. No unlisted or internal media gains an automatic sitemap or navigation entry. Public use of a **cascading graphic property** requires its `exposure: "public"`, while a **direct module media reference** carries no property-value exposure and becomes public-eligible only through a validated, explicitly authored public Page or module input. A module never gains access to an internal or suppressed property value, or to an unreferenced file. The platform library is globally reusable without copying its bytes into Entity-owned source directories, and the namespace distinction is never inferred from filenames or public addresses.
6. **Refusals.** A failed singular graphic cardinality, an invalid `exposure`, a wrong asset class, a missing or unsafe source, an unsupported format, an unknown role, a duplicate background source or a forbidden namespace **blocks the entire candidate publication**. `suppress` plus one local value legitimately replaces a singular inherited selection, explicitly authored platform asset usage is not automatic fallback, and direct module images carry their own alt/decorative policy under §6.2E.

### 6.2J Base addresses, routes, locale selection, Unicode, link targets and discovery (`O10`)

1. **Base.** `base.canonical` is one absolute `http:` or `https:` URL with a DNS hostname or IP and an optional safe prefix, no credentials, query, fragment, control character or encoded separator; a trailing slash normalizes to a canonical no-trailing-slash base except at the origin root. HTTPS production is strongly recommended, but **HTTP remains a valid declared R00.B address** subject to separately approved deployment security. Host syntax and port are validated, the canonical host is lowercase IDNA-conformant ASCII with default ports normalized, and no `Host` header, forwarding header, geo-IP or provider account ever creates authority. `base.aliases` is an array of 0–8 distinct fully declared bases with equivalent safe structure, none equal to the canonical base after normalization; aliases redirect only to the authoritative canonical origin and prefix and never select a second Entity or Page. Base prefixes have 0–5 strict ASCII-safe segments matching `[a-z][a-z0-9-]*`, never `.` or `..`, with no percent escapes, and canonical output includes the prefix **exactly once**.
2. **Locales.** `Site.defaultLocale` and every representation `locale` are canonical BCP47 tags with a **mandatory region subtag** and optional script — `en-001`, `id-ID`, `de-DE`, `zh-Hans-CN` — and bare `en` or `de` are never publication keys. Canonical authored case is language lowercase, script title-case and region uppercase, while the public locale path uses the **lowercase full tag** including hyphen separators. Locale identity and path case are independent of Unicode Page-route case. On one Site, `languagePreferences` maps a request preference tag to an **already supported full locale**; map keys may be bare languages or canonical full tags, canonicalized without ambiguity, values must be supported full tags within that Site, nothing is inferred from physical location, and duplicate normalized keys with conflicting targets refuse.
3. **Route paths.** A representation `route.path` is an authored base-relative path beginning with exactly `/<lowercase-full-locale-segment>/` followed by 1–8 Page-route segments, with no trailing slash other than a deliberately authored root-like special entry (which a representation may not use). The first segment after the leading `/` must be the normalization of that representation's declared `locale`. Page-route segments permit NFC Unicode letters, marks and decimal digits plus internal `-` and `_`, and may not be empty, `.`/`..`, leading or trailing hyphen, control or bidi control, slash or backslash, undecodable URI data or an encoded delimiter. Locale segments are never inferred from location. The Page-route token portion is ≤160 Unicode scalars with each segment ≤60, and unsafe HTML/JS URI parsing or ambiguous double decoding rejects before matching.

4. **Unicode profile.** Authored route-text normalization and case comparison use **Unicode 18.0.0**, with NFC and full Unicode Default Case Folding (not ASCII-only lowercasing) for insensitive comparison, then NFC again. A public-insensitive canonical Page-route spelling is NFC followed by Unicode 18 lowercase mapping, while a sensitive claim preserves its authored NFC exact case. No locale-specific tailoring is enabled in this first global route-comparison profile — that is an explicit deterministic choice, not an unreviewed implicit default — and a future change to Unicode data or tailoring requires collision-regression tests over every authored route claim plus a versioned migration and revalidation. The toolchain binding that produces exactly these results is selected in `R00.E`, which may not silently pick a different Unicode version. Confusable lookalikes are diagnosed with explicit UTS #39-style checks without inventing alias equivalence, and dangerous control or bidi ambiguity rejects.
5. **Case policy.** An authored `route.caseMatching` on a **public** representation is optional, defaults to `insensitive`, and may be explicitly `sensitive`; a sensitive path preserves case as canonical and a wrong-case request returns not-found, never a guessed redirect. An effectively **unlisted** representation ignores public match modes and **always** matches the authored NFC Unicode Page-route spelling exactly, so `caseMatching` must be omitted for it. Unlisted variants have no route aliases in this first profile. Locale-segment normalization stays independent and lowercase even on unlisted or sensitive Pages. Public aliases use the authored case policy and may never point at, collide caselessly with or reveal an unlisted representation.
6. **Collision ledger.** Validation derives a **complete route ledger** over canonical localized claims, every alias, all Page `entryPaths`, the configured base entry and base aliases. Alias objects have only `path` and optional `caseMatching`, and an alias path must be a full matching locale path targeting its owning representation. `entryPaths` are base-relative locale-less public entry paths of 1–8 Page-route segments, never a segment reserved as a full representation locale, belonging to their containing qualified Page rather than a guessed ID. Every accepted R00.B collision rule is enforced — insensitive versus sensitive overlap, public versus unlisted caseless overlap, Unicode normalization equivalence, alias collisions, locale-less entry conflicts, base-prefix preservation and no first-match priority. Two distinct sensitive public paths differing only in case may coexist when neither overlaps an insensitive claim, and two distinct unlisted paths differing only in exact case may coexist when neither collides with a public claim. An invalid claim blocks **whole publication** before any HTTP serving.
7. **Entry and landing.** An optional `base.landing` qualified Page reference selects only an authored Page with an eligible public localized representation; a fully unlisted Home may exist but must not be exposed, and an absent or unavailable public landing makes the base entry return not-found. Base landing and Page `entryPaths` are locale-less **temporary preference-dependent redirect entry points**, never alternate canonical Pages. Selection order is: explicit full-locale URL wins; else a supported saved full-locale preference; else a supported exact or mapped `Accept-Language` match; else the authored Site default. A missing translation applies the same Page's validated `missingTranslation` policy only and redirects to an actual public authored canonical route, while a strict policy or no valid target yields not-found. There is no cross-Site, cross-Facet or cross-Page substitution, and the current request Host or remote source can never select a different Entity, locale or Page by inference.

8. **Statuses.** A safe canonical correction and base-alias normalization use a permanent canonical redirect (normally `308`) only when semantically stable; preference-dependent locale-less entry and permitted missing-translation fallback use temporary redirects (normally `302`) with `Vary` or equivalent no-shared-cache protection for the inputs actually consulted. A malformed or unsafe URI returns `400`; an unknown route, locale or representation returns `404`; a wrong method returns `405` where applicable; an untrusted or invalid host is refused with an appropriate `421`/`400` and never derives canonical authority; an unexpected server failure returns `5xx` with no silent sibling fallback. This states the HTTP contract, not a requirement to choose a framework before `R00.E`.
9. **Typed links.** JSON links use one **closed** object of these variants: `{ "kind": "page", "target": <QualifiedRepresentation> }`; `{ "kind": "external", "url": "https://…" | "http://…" }`; `{ "kind": "email", "address": "<addr-spec>", "subject": optional plain text ≤160 }`; `{ "kind": "fragment", "id": "<existing same-Page anchor>" }`. Each link is validated before output; a `page` link requires the full four-level qualified Page lineage plus an explicit full target `locale` and an actually published representation. An explicit Page link may point at an intentionally known unlisted representation but is never auto-discovered, and automatic navigation, locale switch, sitemap, `hreflang` and `x-default` **never** include unlisted content. An `external` link can never trigger a server-side fetch, forward a redirect from arbitrary request input or acquire publication-metadata authority. Markdown link forms map onto these classified destinations through §6.2D without any new Markdown syntax.
10. **Derived metadata.** Public metadata (`canonical`, alternates, `hreflang`, `x-default`, sitemap, robots, Open Graph and assets) derives **only** from the validated effective public representation and approved public projection — never from body front matter, a guessed counterpart, unlisted content or arbitrary source paths. Unlisted direct responses always carry `noindex, nofollow`, are excluded from every generated discovery surface and are **not** access control. The public `lang` is the actual full authored locale, the canonical base prefix is applied exactly once to every generated URL, and source paths or asset ownership never determine EFS identity or public address.

### 6.2K Validation ordering, compatibility, diagnostics, snapshots and migration (`O11`)

1. **Compatibility tuple.** Version compatibility is an explicit tuple of `manifest.schemaVersion`, `manifest.propertyRegistryVersion`, `manifest.platformAssetLibraryVersion`, each selected JSON `contentVersion`, each node's `contractVersion` and the product release's advertised supported-contract matrix; the first profile expects the exact `1.0.0` of each independent contract. No automatically widened semver range, `latest` or implicit default version exists. A product may support additional **enumerated exact versions** only when their compatibility and migration behaviour is explicitly documented and tested; any unknown value is a refusal.
2. **Validation stages**, all fail-closed and never reordered to skip a later check: strict JSON parsing, schema version and exact keys; EFS identity and nested cardinality; candidate declaration and canonical ownership; one JSON-first selected safe body; Markdown or JSON content profile and module registry; effective properties including location and graphics with per-value provenance; asset safety and exposure; complete routing, locale, visibility and link-ledger collisions; public projection; final immutable candidate snapshot and atomic activation. No stage may permit partially published output.

3. **Candidate snapshot and activation.** A proposed publication is one immutable **candidate snapshot** of validated manifest bytes, source and asset content identities and the complete derived model. If any referenced resource changes while validation runs, that candidate is invalidated and revalidated. Only a fully validated candidate may activate, in one atomic step; a failed candidate cannot remove unaffected published Pages from the currently active validated snapshot, and a previously deployed snapshot may keep serving unchanged without that being called acceptance of the invalid candidate. Cache metadata and URLs correspond to the exact activated snapshot and configured base.
4. **Diagnostics.** Every validator error has the closed envelope `{ "code": <registered code>, "severity": "error", "node": <qualified EFS reference or null>, "fieldPath": "<JSON Pointer or declared source offset>", "message": "<safe short explanation>" }` in internal author tooling; a public response uses a generic error status and leaks no hidden path, secret, private content or serialized internal property fact. Code families are `MANIFEST_*`, `EFS_*`, `SOURCE_*`, `CONTENT_*`, `MODULE_*`, `PROPERTY_*`, `LOCATION_*`, `ASSET_*`, `ROUTE_*`, `LOCALE_*`, `LINK_*`, `EXPOSURE_*`, `COMPAT_*` and `SNAPSHOT_*`. The exact mandatory codes are `MANIFEST_VERSION`, `MANIFEST_UNKNOWN_FIELD`, `MANIFEST_DUPLICATE_KEY`, `EFS_DUPLICATE_ID`, `SOURCE_NONE_EFFECTIVE`, `SOURCE_INVALID_PREFERRED_JSON`, `SOURCE_COLLISION`, `SOURCE_UNSAFE_PATH`, `MODULE_UNAVAILABLE`, `MODULE_INVALID_PROPS`, `PROPERTY_CARDINALITY`, `LOCATION_INVALID_EXTEND`, `ASSET_INVALID`, `ROUTE_COLLISION`, `ROUTE_INVALID_ENCODING`, `LOCALE_INVALID`, `LINK_UNSAFE`, `EXPOSURE_LEAK`, `COMPAT_UNSUPPORTED` and `SNAPSHOT_CHANGED`. Every refusal maps to one stable code; a refusal not more narrowly classified by a named mandatory code uses exactly that family's `<FAMILY>_INVALID` code (`MANIFEST_INVALID`, `EFS_INVALID`, `SOURCE_INVALID`, `CONTENT_INVALID`, `MODULE_INVALID`, `PROPERTY_INVALID`, `LOCATION_INVALID`, `ASSET_INVALID`, `ROUTE_INVALID`, `LOCALE_INVALID`, `LINK_INVALID`, `EXPOSURE_INVALID`, `COMPAT_INVALID`, `SNAPSHOT_INVALID`), with family selection determined by the first invalid governing contract — never by first-match routing or publication fallback. New narrower codes require an explicitly versioned registry extension and no unregistered `GENERIC_ERROR` exists. Aggregation may report several independent errors, but it can never select a convenient first source or route as recovery.
5. **Compatible extension and migration.** A compatible extension adds a **new versioned optional** module or property contract without reinterpreting previously valid values, and no existing manifest silently changes meaning. If a manifest, property registry, module contract or library version becomes unsupported, validation refuses before public activation rather than upgrading implicitly. A breaking change requires a new major version and an Owner-reviewed deterministic migration: read the previous immutable manifest and source snapshot, write a separate non-authoritative migration candidate, report the exact identity, path, route, source and property changes, obtain deliberate acceptance, run complete validation and only then activate. The original is preserved until an explicitly accepted replacement exists, with no silent EFS ID reminting, Page reparenting, route renaming, asset substitution or selected-source fallback.
6. **Ports and adapters.** Strict Hexagonal Architecture holds: domain and application contracts import no framework, persistence, HTTP, filesystem or provider module; parsing, source, asset, module, render and HTTP delivery are replaceable adapters wired by a composition root; import constraints and tests include aliases, re-exports and dynamic module loading. Every module or external integration is deny-by-default until approved by the Owner's scope and validated ports. Later application tests must prove full candidate verification and rollback against fake and real adapters, and documentation-only runs report `NOT APPLICABLE` for them.

### 6.2L Traceability and what "R00.D specification complete" means (`O12`)

1. **Decided and binding:** `O01`–`O11` as the exact first profile; `D01`–`D12` as previously accepted; §6.2B's literal keys; §6.2E.2's twenty-two module contracts; §6.2G's property catalogue; §6.2H's identified location operations; §6.2I's asset namespaces and media profile; §6.2J's Unicode and route policy; and §6.2K's version and snapshot semantics. `O12` is the check that every such choice has a reviewed canonical paragraph here and no contradictory OPEN claim survives. Future code must implement this profile rather than interpolate a preferred framework convention.
2. **Not settled by R00.D — later product decisions:** provider account, DNS, TLS and deployment; concrete library artwork, provenance and codecs supplied as released assets; browser implementation detail and actual code artifacts; full lifecycle and upgrade-support commitments; and numerical WCAG or performance release targets. These belong to `R00.F`, `R00.G` and subsequent implementation packages and may not change the exact physical publication schema, URL, case or locale outcomes, permissions or module contract without new Owner approval. They must never be confused with a missing R00.D literal — the latter blocks acceptance. The implementation language, framework, package layout, build and test toolchain and CI platform are **no longer open**: they are recorded as a conditional selection in §6A with unexecuted proof gates, and they likewise may not change any R00.D literal.
3. **One-to-one index.** Every `D01`–`D12` and `O01`–`O12` has a canonical requirement entry with accepted behaviour, invalid or negative case, exact field or schema anchor and the future test or evidence gate, as tabulated in §6.3 (conceptual cases `V01`–`V28` plus `P01`–`P14` in §6.2N) and §6.5 (D/O matrix). All twenty-two initial modules appear with ID, fields, types, defaults, child and interaction rules and reference behaviour in the same authority. None of these rows is a claim that a test has run.
4. **No implementer guesswork.** Any unmentioned or unknown data field fails, any absent required field fails, and optional defaults exist **only** where §6.2E.2 enumerates them. Unknown module, role or version fails. No module may infer Page publication, routes, exposure, fallback, geometry, locale, current host or filesystem source. No generic `metadata`, `custom`, `rawHtml`, `script`, `sourceUrl` or `pluginConfig` escape hatch is approved.
5. **Approval meaning.** Documentary acceptance of this profile means the R00.D physical specification is **ready for administrative closure review**; it does not mean application code exists or a product release is accepted. A separate Owner-directed closure order clears ACTIVE and advances NEXT to the earliest dependency-ready `R00.E` candidate after the Chief Architect's independent `ACCEPT`, and ROADMAP is never consumed inside this specification work.

### 6.2M Normative syntax examples (`O05`, `O09`, `O10`)

**These are normative syntax examples, not installed files.** They fix key names, types and the structural relationship between contracts; every referenced body and asset must exist, pass strict validation and sit in its correct namespace before anything publishes. No file here is claimed to exist in this repository.

#### 6.2M.1 Minimal starter installation — one Facet, one Site, one Page, dual candidates

`content/publication.json`:

```json
{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {
    "canonical": "https://example.org/site",
    "aliases": [],
    "landing": { "entity": "example", "facet": "main", "site": "main", "page": "home" }
  },
  "entity": {
    "id": "example",
    "label": "Example Entity",
    "facets": [{
      "id": "main",
      "label": "Main",
      "sites": [{
        "id": "main",
        "label": "Main Site",
        "defaultLocale": "en-001",
        "languagePreferences": { "en": "en-001" },
        "pages": [{
          "id": "home",
          "label": "Home",
          "representations": [{
            "locale": "en-001",
            "route": { "path": "/en-001/home" },
            "sourceCandidates": {
              "json": "pages/main-home.en-001.json",
              "markdown": "pages/main-home.en-001.md"
            }
          }]
        }]
      }]
    }]
  }
}
```

Valid initial state: the declared JSON file is genuinely **absent**, and `content/pages/main-home.en-001.md` contains:

```markdown
# Welcome to Example

This is a real authored Home Page with **safe Markdown**.

[Contact the team](mailto:hello@example.org)

![Project image](images/project.webp)
```

Publishing additionally requires a valid `content/assets/images/project.webp` while that image reference is present. Adding a valid JSON body at the **already-declared** `content/pages/main-home.en-001.json` changes the effective source to JSON only, leaving Page identity, public address, locale and visibility unchanged. If that newly present JSON is malformed, publication **refuses** and the Markdown file is never used to conceal the error.

#### 6.2M.2 Expanded installation — two Facets, three Sites, four Pages, locations, graphic suppression, aliases, locales and unlisted variant

```json
{
  "schemaVersion": "1.0.0",
  "propertyRegistryVersion": "1.0.0",
  "platformAssetLibraryVersion": "1.0.0",
  "base": {
    "canonical": "https://example.org/foundation",
    "aliases": ["https://old.example.org/foundation"],
    "landing": { "entity": "example", "facet": "visitors", "site": "bali", "page": "home" }
  },
  "entity": {
    "id": "example",
    "label": "Example Entity",
    "properties": {
      "location": {
        "inheritance": "inherit",
        "currentContext": [{
          "operation": "add", "factId": "indonesia", "kind": "association",
          "fields": { "countryCode": "ID" }, "exposure": "public"
        }]
      },
      "graphic.logo": {
        "inheritance": "inherit",
        "currentContext": [{
          "source": { "namespace": "entity", "reference": "branding/default-logo.png" },
          "exposure": "public"
        }]
      }
    },
    "facets": [
      {
        "id": "visitors", "label": "Visitors",
        "properties": {
          "location": {
            "inheritance": "inherit",
            "currentContext": [{
              "operation": "extend",
              "target": { "owner": { "entity": "example" }, "factId": "indonesia" },
              "set": { "namedPlace": "Bali" }
            }]
          },
          "graphic.logo": {
            "inheritance": "suppress",
            "currentContext": [{
              "source": { "namespace": "entity", "reference": "branding/visitor-logo.webp" },
              "exposure": "public"
            }]
          }
        },
        "sites": [
          {
            "id": "bali", "label": "Bali Site", "defaultLocale": "en-001",
            "languagePreferences": { "en": "en-001", "id": "id-ID" },
            "pages": [
              {
                "id": "home", "label": "Home", "visibilityDefault": "public",
                "representations": [
                  {
                    "locale": "en-001", "route": { "path": "/en-001/home" },
                    "sourceCandidates": { "json": "pages/bali-home.en-001.json", "markdown": "pages/bali-home.en-001.md" }
                  },
                  {
                    "locale": "id-ID", "visibility": "unlisted",
                    "route": { "path": "/id-id/Hidden7A" },
                    "sourceCandidates": { "markdown": "pages/bali-home.id-id.md" }
                  }
                ]
              },
              {
                "id": "about", "label": "About", "entryPaths": ["/about"],
                "missingTranslation": { "mode": "default" },
                "representations": [
                  {
                    "locale": "en-001", "route": { "path": "/en-001/about" },
                    "sourceCandidates": { "markdown": "pages/bali-about.en-001.md" }
                  },
                  {
                    "locale": "id-ID",
                    "route": { "path": "/id-id/TentangKami", "caseMatching": "sensitive" },
                    "sourceCandidates": { "json": "pages/bali-about.id-id.json" }
                  }
                ]
              }
            ]
          },
          {
            "id": "jakarta", "label": "Jakarta Site", "defaultLocale": "en-001",
            "pages": [{
              "id": "home", "label": "Jakarta Home",
              "representations": [{
                "locale": "en-001", "route": { "path": "/en-001/jakarta" },
                "sourceCandidates": { "markdown": "pages/jakarta-home.en-001.md" }
              }]
            }]
          }
        ]
      },
      {
        "id": "company", "label": "Company",
        "sites": [{
          "id": "global", "label": "Global Site", "defaultLocale": "de-DE",
          "pages": [{
            "id": "home", "label": "Company Home",
            "representations": [{
              "locale": "de-DE", "route": { "path": "/de-de/unternehmen" },
              "sourceCandidates": { "markdown": "pages/company-home.de-de.md" }
            }]
          }]
        }]
      }
    ]
  }
}
```

This manifest has exactly **one Entity**, two Facets, three Sites and four Pages: Page ID `home` is reused correctly inside three distinct Site scopes, beside the Bali `about` Page. Full-locale keys and route spellings stay distinct from physical places. The Bali `id-ID` Home representation is **unlisted** and exact-case: addressable, excluded from generated discovery and excluded from every missing-translation fallback. The `visitors` Facet extends only its inherited Entity location fact and suppresses and replaces the singular inherited logo without creating a new Entity. The Product must never infer that a visitor on the Jakarta Site is located in Jakarta.

#### 6.2M.3 Advanced JSON body selected by the expanded manifest

For the declared `content/pages/bali-home.en-001.json`, this is a valid first-profile JSON body, assuming its declared Entity assets exist and pass validation:

```json
{
  "contentVersion": "1.0.0",
  "title": "Visit our destinations",
  "summary": "An accessible layered public Page",
  "blocks": [
    {
      "module": "content.hero", "contractVersion": "1.0.0",
      "props": { "eyebrow": "Discover", "tagline": "Explore the places we serve" },
      "presentation": {
        "foreground": "#FFFFFF",
        "background": { "kind": "solid", "color": "#123456" },
        "padding": "xl", "radius": "md"
      }
    },
    {
      "module": "layout.grid", "contractVersion": "1.0.0",
      "props": { "columns": { "base": 1, "md": 2, "lg": 2 }, "gap": "lg" },
      "children": [
        {
          "module": "media.image", "contractVersion": "1.0.0",
          "props": {
            "asset": { "namespace": "entity", "reference": "images/bali-view.webp" },
            "alt": "View of the coastline", "fit": "cover"
          }
        },
        {
          "module": "content.prose", "contractVersion": "1.0.0",
          "props": { "markdown": "Welcome to our **real authored** content. [About us](../about)." }
        }
      ]
    },
    {
      "module": "media.carousel", "contractVersion": "1.0.0",
      "props": {
        "slides": [
          { "asset": { "namespace": "entity", "reference": "images/one.webp" }, "alt": "A garden" },
          { "asset": { "namespace": "entity", "reference": "images/two.webp" }, "alt": "A trail" }
        ],
        "controls": true, "autoplay": false
      }
    }
  ]
}
```

The carousel uses direct typed media references rather than a `graphic` property, so its values modify neither public routing nor the representation's property authority. The typed `content.prose` relative link `../about` resolves from `/foundation/en-001/home` to the authored locale-less entry `/foundation/about` inside the configured base; real navigation resolves through the D07 route ledger and never guesses a translated slug. With the prefix `/foundation`, the resolver handles the base-aware `/about` author link consistently, producing neither a wrong host nor a double prefix.

#### 6.2M.4 Required invalid fixtures — deliberate refusals

- **Invalid JSON priority (refused).** If `content/pages/bali-home.en-001.json` exists containing `{ "contentVersion": "1.0.0", "blocks": [{ "module": "unknown.module" }] }` — also missing `title` and `props` — the **entire candidate publication** is rejected and the simultaneously declared valid Markdown file is never used. Diagnostic: `CONTENT_INVALID` and/or `MODULE_UNAVAILABLE`, never `SOURCE_NONE_EFFECTIVE`.
- **Singular graphic cardinality (refused).** If the Bali Site inherits Facet `graphic.logo` and also declares a local `graphic.logo` with `inheritance: "inherit"` instead of `suppress`, the effective count is 2 and publication refuses with `PROPERTY_CARDINALITY`, even when one value is `internal` or a module would choose a different logo.
- **Route exposure collision (refused).** If a public insensitive alias claims `/id-id/hidden7a` while the exact-case unlisted Home claims `/id-id/Hidden7A`, publication refuses with `ROUTE_COLLISION` for a public/unlisted caseless collision; an unlisted representation is never exposed through an alias.
- **Location extension failure (refused).** If the Company Facet attempts to extend a fact originally added inside the Visitors Facet, the cross-branch extension is rejected with `LOCATION_INVALID_EXTEND`; a suppressed ancestral fact is never a valid extension target.

### 6.2N Minimum physical-profile positive and refusal ledger (`P01`–`P14`)

**Documentary, not executed.** Each row is a normative positive and refusal **specification**, not a passing test. §6.3 keeps the `V01`–`V28` conceptual cases unchanged; these `P` cases add the physical-profile obligations that later code-bearing work must implement as real tests, refusal cases included.

| ID | Positive outcome and required refusal |
| --- | --- |
| `P01` | An exact `content/publication.json` with `schemaVersion: "1.0.0"` is accepted; an absent, malformed, duplicate-key, unknown-field or otherwise versioned manifest is refused (`MANIFEST_VERSION`, `MANIFEST_UNKNOWN_FIELD`, `MANIFEST_DUPLICATE_KEY`). |
| `P02` | A valid five-level nested graph with correctly scoped reused IDs is accepted; duplicate IDs, orphans, a second Entity and a bare-language locale are refused (`EFS_DUPLICATE_ID`, `EFS_INVALID`, `LOCALE_INVALID`). |
| `P03` | JSON absent with valid Markdown selects Markdown; JSON valid with Markdown valid selects JSON; present-but-invalid JSON, a permission-denied read and both candidates missing are refused (`SOURCE_INVALID_PREFERRED_JSON`, `SOURCE_NONE_EFFECTIVE`). |
| `P04` | Two candidate paths reserved for a future JSON upgrade are accepted; a duplicate candidate claim, including an absent case-folded or hardlink alias, is refused (`SOURCE_COLLISION`). |
| `P05` | An absent `properties` key inherits, a local `inherit` appends, `suppress` resets then adds local values; a singular collision, a private-value leak and an unknown key are refused (`PROPERTY_CARDINALITY`, `EXPOSURE_LEAK`, `PROPERTY_INVALID`). |
| `P06` | An identified accessible inherited fact is updated in place by `extend` with field origin preserved; a missing, suppressed, sibling, cyclic or multiply-extended target is refused (`LOCATION_INVALID_EXTEND`). |
| `P07` | An author-selected platform asset and an Entity asset stay distinct and both publish; a wrong MIME, a missing asset, active SVG, an oversized image and an automatic generic fallback are refused (`ASSET_INVALID`). |
| `P08` | Recognized CommonMark/GFM constructs and the exact heading and anchor rules are accepted; raw HTML, a second H1, an unsupported directive and an unsafe URL are refused (`CONTENT_INVALID`, `LINK_UNSAFE`). |
| `P09` | Each of the twenty-two §6.2E.2 modules publishes with minimal valid props and its child policy; omitted optional behaviour resolves to the exact written default (`autoplay` true with `intervalMs` omitted gives 10000 ms, effect-specific `durationMs` and `direction` defaults apply, watermark `position` resolves `center`, stack `align` resolves `stretch`, layers `align` `center` on the vertical block axis with `minHeight` `md` as a 400 CSS-pixel minimum, omitted action `variant` resolves `text`, omitted accordion `initialOpen` leaves all panels collapsed), parallax uses exactly `p = clamp((H - T) / (H + E), 0, 1)` and `y = (2p - 1) × 24` CSS pixels, the carousel starts at index 0 with cyclic single-slide manual navigation and the specified hover/focus/Pause behaviour, every effect stays bounded and renders statically under reduced motion, and an unknown ID, version or prop, a conditionally prohibited prop (`intervalMs` without autoplay, `direction` on `fade`/`reveal`/`parallax`), incompatible content, arbitrary script and an advanced form builder are refused (`MODULE_UNAVAILABLE`, `MODULE_INVALID_PROPS`). |
| `P10` | Public insensitive and sensitive claims plus exact-case unlisted claims with independent locale folding are accepted; collisions, encoded slashes, path ambiguity, unsafe bidi and a false canonical redirect are refused (`ROUTE_COLLISION`, `ROUTE_INVALID_ENCODING`). |
| `P11` | Declared Page links, a safe `mailto:`, an external HTTPS destination and an intentional explicit unlisted link are accepted; a server-side remote fetch, a dangerous scheme and public discovery of unlisted content are refused (`LINK_UNSAFE`, `EXPOSURE_LEAK`). |
| `P12` | A complete candidate activates atomically; a source modified during validation, a partial candidate, an unknown registry and an incompatible module are rejected without touching the last valid publication (`SNAPSHOT_CHANGED`, `COMPAT_UNSUPPORTED`). |
| `P13` | The minimal one-Entity starter and the expanded two-Facet multi-Site locale installation are both consistent; an Entity, locale or Page inferred from host, route or source filename is refused. |
| `P14` | A verified requirement-to-O/D mapping with physical and runtime obligations kept separate is accepted; an undocumented literal, a stale contradictory OPEN gate and a fake executed-test claim are refused. |

### 6.3 Conceptual contract-case ledger (`V01`–`V28`)

**Documentary, not executed.** Every row below is a normative positive and refusal **specification** with its governing contract named. None of these rows is a passing test, a schema-validator run, a browser result or a runtime publication claim; no such evidence exists in this documentation-only repository. Later code-bearing work must implement them as real tests, including the refusal cases. The physical-profile cases `P01`–`P14` in §6.2N must be read alongside this ledger, and §6.5 maps both sets onto `D01`–`D12` and `O01`–`O12`.

| ID | Required positive and refusal coverage | Governing contract |
| --- | --- | --- |
| V01 | One Entity; a second Entity refused; nested one-parent ownership accepted; orphan and empty published cardinalities refused. | §1, §1.5 |
| V02 | Correct scoped ID reuse accepted; invalid token and duplicate same-parent ID refused; identity survives address, source and base changes. | §1.1, §1.5 |
| V03 | Full-locale representation identity accepted; duplicate normalized locales refused; a bare-language published key refused. | §3.2 |
| V04 | Direct JSON candidate and Markdown candidate explicitly declared; no folder-scan publication; drafts stay isolated. | §1.6, §1.3 |
| V05 | JSON absent with valid Markdown selects Markdown; both valid selects JSON; invalid or inaccessible JSON refuses; both absent refuses. | §1.6 |
| V06 | Duplicate source claims refused, including an absent candidate path and its aliases; no sibling substitute and no body merge. | §1.6, §3.1C |
| V07 | Equivalent common content semantics from JSON and Markdown accepted; invalid JSON sections or modules, unsupported Markdown directives and raw active HTML refused. | §1.7, §3.1A |
| V08 | Markdown restricted to the CommonMark and GFM safe subset with no Solo-specific Markdown; advanced JSON only through typed modules, with no arbitrary executable input. | §3.1A, §3.1B |
| V09 | Module name, version and typed-input compatibility enforced; unknown or incompatible inputs and unauthorised ports refused; excluded advanced forms refused. | §3.1B, §6.1 |
| V10 | Page default and representation visibility resolve by the exact four-case matrix; unlisted served only at its exact route with `noindex, nofollow` and no generated discovery. | §3.2A |
| V11 | Canonical bare-origin and prefixed-base composition correct; prefix applied exactly once; a base alias never becomes independent identity. | §4.1, §4.4 |
| V12 | Locale selection order saved preference, mapped browser preference, authored default; an explicit full locale overrides both; language-only preference requires an explicit mapping. | §3.2, §3.2C |
| V13 | Same-Page public-only fallback and authored locale-less entry accepted; unlisted and cross-Site/cross-Page fallback refused. | §3.2 |
| V14 | Public insensitive claims resolve to a lowercase canonical spelling; sensitive claims preserve authored Unicode case; unlisted claims are exact-case; locale folding stays separate. | §3.2B |
| V15 | Route text normalized to NFC with controlled decoding; alias collisions, public/unlisted caseless collisions and sensitive/insensitive overlap refused. | §3.2B |
| V16 | Property absence, `inherit`, `suppress` and `none` behave additively and in order; effective maximum and uniqueness enforced; no nearest-value-wins. | §2, §2.3 |
| V17 | Per-value origin and field provenance retained; internal values never leak into HTML, metadata or modules; suppression honoured. | §2.3 |
| V18 | Location independent fact `add` accepted; same-record in-place `extend` accepted; suppressed, cross-branch, missing, ambiguous and circular targets refused. | §2.4 |
| V19 | Location confers no route or locale authority; several facts create no implicit primary; no mandatory premises or address is required. | §2.1, §2.4 |
| V20 | Graphic singular collision versus suppression; typed Entity and generic library assets stay distinct; zero automatic fallback. | §3.3A, §3.3B |
| V21 | Direct module media references (watermarks, carousels, backgrounds, layered heroes) without EFS role confusion; safe asset validation. | §3.3B |
| V22 | Safe content-relative paths accepted; symlink, traversal and alias ambiguity refused; drafts, generated output and secrets excluded as source authority. | §3.1C |
| V23 | Standard Markdown links and images; valid internal, HTTP(S), `mailto:` and fragment handling; dangerous scheme refusal; base prefix applied once. | §3.1C |
| V24 | Unknown schema, registry or module version refused; compatible additive upgrades accepted; breaking changes require explicit migration. | §6.1 |
| V25 | Whole-graph all-or-nothing publication including unlisted representations; an invalid new candidate never partially replaces prior output. | §6.1 |
| V26 | Strict Hexagonal dependency policy enforced; every external resource access passes through an explicit validated application port. | §5 |
| V27 | All seven Owner design-reference URLs accurately retained and associated with the advanced design capability families. | §3.1B |
| V28 | Consistency with accepted `R00.A`–`R00.C` and the Product Definition; no hidden Spokes, no geography-derived routing, no advanced forms and no implied deployment. | §1, §3.4, Product |

### 6.4 `D01`–`D12` decision index

Each Owner-accepted decision and its canonical home in this Charter. Chief Architect disposition remains independent of this index.

| Decision | Contract | Canonical home |
| --- | --- | --- |
| `D01` | One authored JSON manifest is sole publication authority; encoding and version refusal. | §1.5 |
| `D02` | Nested EFS declarations, IDs, direct parentage, qualified references. | §1.5 (with §1.1) |
| `D03` | **Owner-amended:** at most two explicit JSON/Markdown candidates, JSON-first, exactly one effective source, invalid JSON refuses. | §1.6 (reconciled in §1.3) |
| `D04` | JSON and Markdown authoring responsibilities under manifest authority. | §1.7 |
| `D05` | **Owner-amended:** mainstream extended Markdown, no Solo syntax or active HTML; expansive named/versioned JSON modules. | §3.1, §3.1A, §3.1B |
| `D06` | Page `visibilityDefault` and representation `visibility` authority. | §3.2A |
| `D07` | Scoped base, Site locale, Page entry and representation route declarations. | §3.2C (policy in §3.2–§3.2B, §4.1–§4.4) |
| `D08` | Keyed property declarations over a governed registry. | §2.3 (semantics in §2) |
| `D09` | Structured location facts with identified-record `extend`. | §2.4 (invariants in §2.1) |
| `D10` | Cascading graphic roles and typed asset-source namespaces. | §3.3B (policy in §3.3A) |
| `D11` | Safe locators, links and resource separation. | §3.1C |
| `D12` | Integrated validation, module compatibility and migration. | §6.1 |

Each decision's physical realization, its governing `O` gate and the cases that verify it are tabulated in §6.5 together with the remaining `R00.E`, `R00.F` and `R00.G` obligations.

### 6.5 Requirement matrix and remaining stage obligations

**`D01`–`D12` realization map.** Accepted behaviour is unchanged; the anchor column names where its exact literals now live, and the case column names the documentary specifications in §6.3 and §6.2N.

| Decision | Physical anchor | Cases |
| --- | --- | --- |
| `D01` | §6.2A.1–§6.2A.2, §6.2B (root), §6.2K.1 | `P01`, `P12` |
| `D02` | §6.2B (nested nodes, identity, qualified targets) | `V01`, `V02`, `V03`, `P02` |
| `D03` | §1.6, §6.2B (`sourceCandidates`), §6.2C.4 | `V04`, `V05`, `V06`, `P03`, `P04` |
| `D04` | §1.7, §6.2D, §6.2E.1 | `V07`, `P08` |
| `D05` | §3.1A, §3.1B, §6.2D, §6.2E.2, §6.2F | `V08`, `V09`, `V27`, `P08`, `P09` |
| `D06` | §3.2A, §6.2B (`visibilityDefault`/`visibility`), §6.2J.5, §6.2J.10 | `V10`, `P10`, `P11` |
| `D07` | §3.2C, §6.2J.1–§6.2J.8 | `V11`–`V15`, `P10` |
| `D08` | §2.3, §6.2G | `V16`, `V17`, `P05` |
| `D09` | §2.4, §6.2H | `V18`, `V19`, `P06` |
| `D10` | §3.3B, §6.2I | `V20`, `V21`, `P07` |
| `D11` | §3.1C, §6.2C, §6.2D.5–§6.2D.7, §6.2J.9 | `V22`, `V23`, `P07`, `P11` |
| `D12` | §6.1, §6.2K | `V24`, `V25`, `V26`, `P12` |

**`O01`–`O12` coverage.** Each gate's normative home is the §6.2 table; the verifying specifications are `O01`→`P01`; `O02`→`P02`, `P13`; `O03`→`P04`, `P07`; `O04`→`P08`; `O05`→`P09`, `P13`; `O06`→`P09`, `P12`; `O07`→`P05`; `O08`→`P06`; `O09`→`P07`; `O10`→`P10`, `P11`; `O11`→`P12`; `O12`→`P14`.

**Remaining obligations — implementation, not R00.D literals.** None of these may alter the literals above without a new Owner decision:

- **`R00.E` — toolchain and platform (selection recorded in §6A; implementation and executable proof outstanding).** The implementation language, framework, package layout, build, test and CI toolchain, the JSON Schema Draft 2020-12 validator enforcing asserted formats, the Markdown parser pinned to CommonMark 0.31.2 plus GFM 0.29-gfm, the Unicode 18.0.0 data binding and the import-boundary enforcement technique are **selected** in §6A (`E01`–`E10`) and still owe their unexecuted `C1`–`C4` proof. Outstanding implementation remains: bounded media inspection adapters; the request/matcher and HTTP delivery adapters; persistence and the source/asset adapters; the `contracts/**` schema and module-registry artifacts; the Owner-reviewed platform asset delivery package with its version-pinned catalogue, artwork, licensing and provenance; and the migration tooling.
- **`R00.F` — contact and integrations decisions.** This package owns **documentary decisions only**, settled in §6B: the contact actions, the three-field envelope and its bounds, delivery outcomes, abuse and rate policy, privacy notice, deduplication, retention posture and analytics scope. It retains **no** hidden implementation, deployment or DNS/TLS obligation. Implementation is allocated elsewhere: the contact form, `mailto:` action and delivery adapter belong to **Stage 08 (`R08.A`–`R08.C`)**; static booking and directions links with analytics-off belong to **Stage 09 (`R09.A`–`R09.C`)**; establishment and deployment/update/recovery belong to `R00.G`'s lifecycle decision contract with later **Stage 10** implementation and **Stage 11** security/nonfunctional evidence. `R00.F` is **not yet closed** and remains pending independent review and separate Owner authorization.
- **`R00.G` — quality, lifecycle and release.** Numeric accessibility and performance targets, per-module browser/assistive-technology and layout-shift tests, release/support policy, third-party notices and copyright/attribution.

No implementation-readiness claim may exceed §6.2's disposition rule, and none of the obligations above is evidence that code, schemas, assets or tests exist.

## 6A. `R00.E` — Owner-issued conditional software structure and toolchain contract (unexecuted C1 proof gates)

**Status and authority.** This section records the **Owner-issued, conditionally retained** software-structure and toolchain selection for Solo (`SOLO-WO-016 R02`, `E01`–`E10`). It is an architectural selection and an implementation obligation. It is **not** an accepted executable conformance result: no package was installed, no schema artefact was generated, no test suite was run in this repository, and no build exists. Every executable proof named below is a **non-waivable future gate**, not a completed one.

**What this section may not do.** It does not revise, reinterpret or relax `D01`–`D12`, `O01`–`O12`, §6.2A–§6.2N, `P01`–`P14` or `V01`–`V28`. Where this section and an earlier accepted literal appear to differ, the earlier accepted literal governs and the difference must be reported for Owner/Architect resolution rather than resolved by implementation convenience. Nothing here creates an Entity, route, module, property, exposure or locale behaviour that §6.2 does not already fix.

**Owner hold points.** The Owner retained this stack **conditionally** on four hold points, each of which is a future executable proof obligation and each explicitly **`UNVERIFIED`** at present:

| Condition | Subject | Current status |
| --- | --- | --- |
| **C1** | Exact parser / JSON Schema / Unicode 18 / Markdown profile proof against official corpora | **`UNVERIFIED` — NOT RUN.** Documentary design only; upstream release documentation is capability evidence, never conformance evidence. |
| **C2** | Provider-neutral static/artifact/request-gateway delivery with **no mandatory always-on process** | Specified; no provider tested. |
| **C3** | Enforceable inward-only dependency direction plus a safe versioned Rust→TypeScript browser wire format | Specified; not executed. |
| **C4** | Full requirement → port/adapter/validator/test ledger for all twenty-two modules with deliberately failing fixtures | Mapped; not executed. |

**Prohibited claims.** No agent, document or report may describe C1 as passing, describe upstream notes as a test result, describe a mocked or skipped test as evidence, or describe `R00.E` as accepted. Documentary completeness is a precondition for review, not acceptance. Section §6A.13 lists the decisions this section deliberately leaves to `R00.F` and `R00.G`.

### 6A.1 `E01` — Architectural shape, delivery neutrality and the retained comparison

**Shape.** Rust 2024 is the publication and domain language. The product is a **publish-time validated, content-addressed artefact pipeline** plus a **request-time transport-neutral response resolver**. HTTP adapters are optional and replaceable: a long-lived Axum process, a platform-specific on-demand function or a compatible request-aware gateway may each host the same resolver. A permanently running Rust process, a Node runtime, a database, a hosting provider, a CMS or a background watcher is **never** a prerequisite of the *published content artefact*.

**One validated snapshot.** Safe public pages and approved interactions are pre-rendered to HTML, CSS tokens and versioned client assets from exactly one validated immutable snapshot. Request-time rendering, where a host needs it, calls the same typed `PublicRenderer` over the same sealed snapshot and never reads mutable sources.

**When a resolver is mandatory.** The authoritative request resolver is required whenever a host cannot guarantee every declared locale, case, canonical-redirect, cache and unlisted rule of §3.2B, §4 and §6.2J. An ordinary static object bucket that only serves case-sensitive keys is **not** a complete Solo delivery for an arbitrary R00.D installation; it may serve as Mode A only after its real configuration passes the whole route contract (§6A.9).

**Presentation split.** Maud escapes server-generated and pre-rendered structural HTML inside an adapter only. Declaratively registered modules (§6.2E.2) render progressively enhanced HTML. Small TypeScript controllers manage the client-only behaviour of the interactive modules from **validated public** versioned enhancement envelopes (§6A.5). Excluded: React, Next.js, any SPA or client router, authored JavaScript/CSS/HTML, client-side EFS authority and framework-default route matching.

**Retained comparison (why this shape, not an inherited one).** These are the recorded alternatives and their decisive trade-offs; they are not inherited from another Foundation project.

| Alternative | Decisive advantage | Decisive reason it was not selected |
| --- | --- | --- |
| (a) Next.js / React / TypeScript | productive visual ecosystem | pulls framework routing, multi-runtime coupling and hydration into ordinary content pages |
| (b) all-TypeScript Node | one language end to end | still owes independent proof of Unicode 18 conformance and Windows/Linux filesystem confinement, with no compensating structural benefit |
| (c) exclusively static generator | lowest hosting cost | cannot universally enforce preference-dependent redirects, sensitive/insensitive matching, exact-case unlisted URLs and aliases |
| (d) Rust SPA with hydration | strong typing | needless client router and accessibility dependence for content |
| (e) **Rust publish-time pipeline with optional request gateway — selected** | one authoritative core; static assets plus on-demand dynamic routing where a host requires it | more toolchain pieces to pin and prove |

Cost, performance thresholds and provider selection are **not** decided here; they belong to `R00.G` and to the separately authorised deployment decision.

### 6A.2 `E01` — Frozen package and version plan, conditional compatibility and supply chain

**Declarative until a code-bearing stage installs tooling.** These are exact pins and trust boundaries. A version existing upstream is **not** verification; each row is verified only when the corresponding executable gate in §6A.12 and §6A.14 passes on a real revision.

| Purpose | Selected exact pin / trust boundary |
| --- | --- |
| Rust compiler + Cargo | Rust **2024 edition**, `rustc 1.99.1`, exact `rust-toolchain.toml`; `cargo build --release --locked` |
| Rust workspace | four packages — `solo-domain`, `solo-application`, `solo-adapters`, `solo-cli` (binary and composition root); one tracked shared `Cargo.lock`; Cargo feature graph also audited |
| HTTP transport only | `axum =0.8.9`, `tokio =1.53.2` in the outer adapter and CLI only; optional transport, never a core or publication dependency |
| Escaping HTML adapter | `maud =0.27.0` in an adapter only; pre-render at publication; emitted HTML static and inert until opted-in behaviour loads |
| Strict JSON parse | `serde =1.0.229`, `serde_json =1.0.154` plus a handwritten duplicate-aware streaming `Visitor`; arbitrary-precision number parsing enabled where strict numeric checks require it, cross-checked against canonical number fixtures; untrusted JSON never materialises into `Value` first |
| Schema engine | `jsonschema =0.60.2`, `default-features = false`, `arbitrary-precision` opted in if required; local pinned schema IDs only; **no** `resolve-http`/`resolve-file`; Draft 2020-12 and asserted formats expressly selected |
| Source-derived schemas | `schemars =1.2.3`, build/dev only; deterministic generation cross-checked against the accepted Charter and typed Rust representations; never overrides the canonical contract |
| Markdown grammar | `comrak =0.56.0`, AST inspection and an explicit safe renderer; pinned CommonMark 0.31.2 plus admitted GFM 0.29-gfm features |
| Unicode 18 core | `icu_normalizer`, `icu_casemap`, `icu_locale`, `icu_locale_core`, `icu_properties` — **all `=2.4.0`** with the exact bundled Unicode 18 data; CLDR 49; locked transitive data crates; UTS #39 v18 security-data fixture |
| IDNA | `idna =1.1.0`, `idna_adapter =1.2.2`; data-version/algorithm compatibility verified separately for host canonicalisation only — never equated with public-route case mapping |
| Capability filesystem | `cap-std =4.0.3` as a starting point plus explicit per-OS no-follow, identity and race checks; an unsafe or unprovable filesystem refuses |
| Image / video | `image =0.25.10` with approved codec features only; native AVIF decoder `libdav1d` reviewed per OS; `ffprobe` from **FFmpeg 9.0.2** verified offline with an LGPL-compatible build when video arrives; no arbitrary system fallback |
| Architecture checking | the Cargo workspace dependency graph **plus** `syn =3.0.9` (dev/check tool) for alias, re-export, module and dynamic negative fixtures; unresolvable source/macro loading forms prohibited |
| Property/adversarial tests | `proptest =1.12.0` (dev only) with versioned official or owned fixtures |
| Browser build | Node.js **`26.13.0` LTS** pinned, TypeScript **`=7.0.2`**, Vite **`=8.3.6`**, npm lockfile v3 and `npm ci`; Node is **absent** from the production artefact and runtime |
| Safe browser wire types | `ts-rs =12.0.1` dev-only generator from the Rust `BrowserEnhancementV1` DTOs, with serde compatibility and generated byte-diff checking; TypeScript strict typecheck plus runtime decoding and cross-language fixtures |
| Browser/a11y proof | `@playwright/test =1.64.1`, `@axe-core/playwright =4.14.0` — development only, pinned browser binaries, plus independent keyboard and manual checks |
| Licensing/audit | `cargo-deny =0.20.2` as a pinned checked tool; exact direct dependencies, tracked locks and third-party notice/licence/advisory review |
| CI | GitHub Actions with full action-commit SHA pins; Ubuntu and Windows validation; no deployment or authentication secrets |

**Compatibility is conditional, not a proven build.** Official ICU4X 2.4 release notes identify Unicode 18; Comrak documents CommonMark and GFM support; `jsonschema` documents Draft 2020-12 **with incomplete support for some keywords**. Before any conformant-validator claim, the executable gates of §6A.6, §6A.7 and §6A.9 must run and their exact outputs preserved. Any unsupported required keyword or profile is a **`FAIL`** and a **STOP** for an Owner-reviewed version or implementation change — never a silent switch to another validator library, a different Unicode version or a relaxed `unevaluatedProperties` requirement.

**Reproducibility and supply chain.** `Cargo.lock` and `package-lock.json` tracked; direct `=version` pins; transitive versions frozen by lock; signed or hashed installer inputs; no dynamic Git branches and no floating actions; network denied for schema/module resolution and for publication; production and development separated; `--locked` and `npm ci` everywhere. All transitive and native dependencies and notices are reviewed, including any licence exception requiring separate approval. A high or critical exploitable vulnerability, or an unsupported OS build, blocks the relevant acceptance. Runtime uses immutable closed artefact bytes and pre-built client bundles only — never `node_modules`, transpilation or network package fetching. No global tool is assumed.

### 6A.3 `E02` / C3 — Compile-time package boundaries, source tree and inward-only enforcement

**Four packages, no decorative package.** The dependency direction is enforced by the workspace itself, not by convention.

```text
Cargo.toml                         # [workspace] exact members, dependency policy
Cargo.lock
rust-toolchain.toml
crates/
  solo-domain/src/                 # identifiers, EFS, properties, locality, routes, visibility, registry semantics; no IO
  solo-application/src/            # inbound use cases, owned outbound ports, public DTOs, publication/resolve/project
  solo-adapters/src/               # confined sources, JSON/schema, Markdown, ICU, asset, rendering, snapshot, HTTP
  solo-cli/src/                    # sole composition root and publish/validate/preview/serve entrypoints
browser/src/                       # progressive TS, no domain route logic
browser/generated/                 # tracked generated TS public wire declarations, not authored content
contracts/                         # generated closed JSON schemas and exact registry, built from normative literals
content/                           # Installation input; one author-controlled publication.json authority
platform/assets/                   # later product-compiled licensed generic catalogue
scripts/  tests/  browser/tests/  .github/workflows/   # later authorised implementation only
```

**Allowed Cargo edges.** `solo-domain → std` only, with no framework, transport, filesystem or provider dependency. `solo-application → solo-domain` plus small pure serialisers and port abstractions. `solo-adapters → solo-domain + solo-application` plus outward libraries. `solo-cli → all three`, for composition only.

**Forbidden, without exception.** Any `solo-domain` dependency on application, adapters or CLI. Any `solo-application` dependency on adapters or CLI, or on Axum, Tokio, filesystem or media libraries. Adapters importing sibling adapter internals except through approved public composition routing. Browser code importing the internal or publication model, credentials, the private projection, routing implementation or the filesystem. Inner crates keep `#![forbid(unsafe_code)]`.

**Enforcement is layered, and a grep-only rule does not pass.** The Cargo package graph is the primary enforcement, including renamed crate dependencies and re-export chains. A `syn =3.0.9` checker plus `cargo metadata` graph checks additionally cover intra-adapter policy, `use` and `pub use`, resolved aliases, absolute or fully-qualified type references, `#[path]`, `include!`, `extern` and dynamic loading. Unknown macro-expanded or generated dependencies, and `libloading`/`dlopen` or non-literal dynamic browser imports, are **prohibited**, never silently ignored. Generated Rust relevant to the check is expanded or inspected where supported; if the checker cannot inspect a supported form, that form is forbidden rather than exempted.

**Negative meta-tests are mandatory.** Fixtures deliberately insert a forbidden direct edge, an alias, a re-export, a macro-produced edge, a qualified path reference and a dynamic edge; `check-architecture` **must** return non-zero for each and must return zero once the legal graph is restored. A refusal test that still passes when the checker is disabled is itself a failure.

### 6A.4 `E02` / C3 — Owned ports, the composition root and delivery independence

**Inbound ports** in `solo-application`. Commands never accept Axum, Node or OS types:

- `ValidateCandidate(InstallationLease) -> ValidatedCandidate | Diagnostics`
- `SealPublication(ValidatedCandidate) -> SealedArtifact | Diagnostics`
- `ActivatePublication(SealedArtifactId, ExpectedActiveId) -> ActiveId | error`
- `ResolveRequest(SnapshotId, CanonicalRequest) -> ResolvedResponse | error`
- `RenderPublic(SnapshotId, QualifiedRepresentation) -> RenderedPublicPage | error`
- later `SubmitSimpleContact(ValidatedInquiry, AbuseContext) -> truthful delivery outcome` (policy reserved to `R00.F`)

A public response envelope contains status, normalised headers, an immutable body reference or bytes and the cache class — never a framework response object.

**Outbound ports** in `solo-application`, each with exactly one replaceable adapter: `SourceRead` (present / confirmed-absent / unsafe / error, with immutable bytes and identity), `AssetInspect` (typed immutable inspected media), `UnicodeProfile` (Unicode 18 mapping, normalisation, locale, host), `JsonDocumentParser`, `SchemaValidator` (Draft 2020-12 over a local closed schema), `MarkdownProfile` (safe AST), `PublicRenderer` (approved public projection only), `ArtifactWriter` (content-addressed immutable files and hashes, no activation), `ActiveSnapshot` (current immutable identity and pin lease), `ArtifactReader` (verified sealed artefact by ID), `ResponseDelivery` (transport-neutral response, optional HTTP/static/edge adapter) and the reserved `ContactDelivery` for `R00.F`.

No port is a decorative abstraction: each is introduced together with a real use case and fake-plus-real adapter proof at the code stage that needs it. External types are converted at the adapter boundary. **Source and media adapters can never themselves publish, route or decide visibility.**

**Single composition root.** `solo-cli` is the only place where concrete adapters are wired to inner contracts. No second wiring path, no test-only bypass that survives into production, and no framework container that resolves inner dependencies implicitly.

**Delivery independence.** `ResolveRequest` and `RenderedPublicPage` are pure use-case contracts over sealed data. An Axum adapter may run a local preview/test server or a provider-neutral process; an on-demand gateway adapts the same envelopes. A gateway may not read mutable `content/`, rebuild publication, or infer its base from `Host` or forwarded headers. A static-file adapter may deliver pre-rendered exact canonicals, immutable assets and safe independent responses **only if** its real hosting configuration passes every required route, redirect, locale and visibility test; otherwise it must compose a request-aware resolver. No implicit CDN rewrite rule is accepted as equivalent without measured proof.

### 6A.5 `E02` / C3 — The versioned safe Rust→TypeScript browser wire contract

**One channel.** Exactly one public **Rust-owned** closed `BrowserEnhancementV1` DTO union, exported from `solo-application`, is the only server-to-browser behaviour channel. The envelope has exactly `schemaVersion: "1.0.0"`, a safe instance ID generated from the validated Page/module position, a fixed `kind` enumeration (`tabs`, `accordion`, `modal`, `carousel`, `motion`) and the minimum whitelisted public control fields the corresponding §6.2E.2 module requires — for example carousel autoplay, interval and initial index, or motion effect, direction and duration.

**Nothing else crosses.** No raw `props`, manifest paths, EFS private data, asset filesystem IDs, internal provenance, secrets, computed user preferences, third-party URLs, or HTML/CSS/JS strings. Static HTML carries the actual escaped text and validated links and assets; the behaviour envelope carries no ordinary content that could leak. Non-interactive modules export **no** browser behaviour payload at all. The only payload-carrying modules are the five browser-enhancement kinds enumerated above; a server-rendered form such as `interaction.contact` (§6.2E.2) is among the non-enhanced modules and exports no `BrowserEnhancementV1` payload.

**Generation and drift control.** Rust `serde` and `schemars` derive the closed JSON Schema; `ts-rs =12.0.1` generates tracked TypeScript discriminated unions from the same Rust DTO definitions into a scratch directory, and a byte comparison rejects stale or divergent TypeScript or schema. The target TypeScript compiler treats generated files as read-only. Generation can never widen a union, add a field or rewrite an accepted literal.

**Serialisation surface.** The envelope is serialised only into a bounded `data-solo-envelope` HTML attribute, JSON-escaped and attribute-escaped by the **single** trusted renderer, with no raw `<`, `>`, `&` or script-terminator breakout. There is **no inline `<script>` tag**, no runtime evaluation and no executable authored value.

**Runtime decoding is mandatory.** Generated types are a compile-time aid, not a runtime validator; `ts-rs` alone never satisfies this boundary. The client decoder checks the exact `schemaVersion` and `kind`, length and finite integer/enumeration limits, the safe instance ID, uniqueness and DOM ownership, and fails **inertly** to the accessible no-JS presentation on any bad envelope.

**Required boundary tests, all deliberately failing where they must.** Serialisation parity between Rust and TypeScript; unknown keys; `__proto__`; invalid Unicode; wrong versions; integer overflow; XSS-breakout strings; forged private fields; mismatched DOM ownership and disabled script execution on refusal.

**No-JS baseline.** Server output keeps semantic content and functioning links, navigation and safe `mailto:`. Tabs and accordion expose their content through an accessible no-JS presentation, with JavaScript only progressively managing visibility. Modal content has a readable non-JS path. A carousel shows slide 0 with manual visual fallback instructions only where that is non-deceptive. Motion never hides content by default. With controllers enabled, the exact §6.2E.2 interactivity becomes testable. No app store, guest accounts or authored behaviour scripts are introduced.

### 6A.6 `E03` / `E04` / C1 — Strict parsing, Draft 2020-12 proof and semantic validation

**Canonical authority is unchanged.** `content/publication.json` with exactly the §6.2B root fields and `schemaVersion: "1.0.0"`, the direct nested EFS graph, qualified references, JSON-first declared `sourceCandidates` and all twenty-two `1.0.0` modules. Nothing in this toolchain overrides §6.2, and publication is never permitted by directory scanning. Schemas are validated against their semantic source; they are not generated as a replacement authoring policy.

**C1-a — strict parser proof (future executable gate).** Bounded input: manifest and JSON ≤2 MiB, Markdown ≤1 MiB. Checked strict UTF-8 with no BOM and no surrogate errors. A streaming Serde `DeserializeSeed`/`Visitor` detects duplicate **decoded** keys in every nested object *before* any `Value` or domain type exists, including `{"x":1,"\u0078":2}`. Rejected: trailing tokens, comments, JSON5/YAML, leading plus, NaN/Infinity, enormous depth or numbers, malformed escapes. Controlled recursion, byte budgets and safe line/column diagnostics are required. Any panic or failure becomes a candidate refusal, never an active-store mutation. `serde_json::Value` used directly is **not** a duplicate-key-safe parser and never satisfies this gate on its own.

**C1-b — JSON Schema proof (future executable gate, conditionally selected engine).** `jsonschema =0.60.2` is selected *conditionally* because upstream documentation explicitly notes incomplete keyword support in newer drafts. The Stage 01 proof must run the pinned official `JSON-Schema-Test-Suite/tests/draft2020-12/` mandatory applicable cases and Solo's own cases for `$schema`, `$id`, `$ref`, `unevaluatedProperties: false` across `allOf`/`anyOf`/`oneOf`/conditionals, `unevaluatedItems`, `properties`, `required`, `pattern`, `min`/`max`, `if`/`then`/`else`, `dependentSchemas`, array bounds, asserted `format` and nested module envelopes. Compilation uses the exact `should_validate_formats(true)` / `should_ignore_unknown_formats(false)` options (the API itself verified at implementation) with an **offline pinned resource registry only**. Default features including `resolve-http` and `resolve-file` are disabled, and no remote or file URI reference may resolve outside the product-pinned catalogue. `$id` and schema hash are validated, unsupported vocabulary and unknown `$ref` are refused. If a required keyword cannot be correctly enforced, the gate is **`FAIL`** and the work **STOPs** for a new Owner-reviewed `R00.E` edition — never an unreviewed library or dialect change and never deletion of `unevaluatedProperties`. Cases must show an extra key being rejected **after** `$ref`/composition; a trivial `additionalProperties: false` case alone is insufficient. Because `format` may be annotation-only in general JSON Schema, assertion is explicitly required *and* routes, locales, IDNA, URIs, mail addresses and coordinates are separately validated in the domain.

**C1-c — division between schema and domain.** Schemas own closed field shape and types, required members, allowed enumerations and version strings, local conditional properties, and numeric and array bounds. Pure domain/application checks own cardinality, direct parent and qualified lineage, identity stability, normalised-locale uniqueness, actual route and visibility collisions, source sharing and JSON-first behaviour, inherited property cardinality and exposure, identified location extension and provenance, real asset bytes and safety, truthful link destinations, heading and fragment references, graphic role suppression and the final public projection. No schema library is assumed able to prove filesystem containment, codec safety or truthful public links. `schemaVersion`, `propertyRegistryVersion`, `platformAssetLibraryVersion`, the JSON `contentVersion`, each installed module `contractVersion` and product compatibility are **separate exact** supported values; any unsupported tuple is `COMPAT_UNSUPPORTED` (or a more precise accepted code) and is never coerced.

**Generation and source of truth.** The accepted §6.2 literals are the semantic authority. Implementation-owned Rust closed type and registry declarations encode them and generate `contracts/publication/1.0.0.schema.json`, `contracts/content/1.0.0.schema.json`, `contracts/properties/1.0.0.schema.json`, `contracts/modules/<exact id>/1.0.0.schema.json` for all twenty-two modules, plus the public UI wire schema and TypeScript declarations. Generated artefacts are tracked at the code stages and compared byte-for-byte against a deterministic scratch regeneration; generation never overwrites authored files during a check. A generator cannot widen unknown keys, numeric bounds or the twenty-two item registry. Golden normative fixtures and a separate Charter-literal cross-check are mandatory, and the engine fails closed when the exact vocabulary is unsupported — partial acceptance never activates a publication.

**Diagnostics and activation.** The exact ordered §6.2K validation stages and closed diagnostic envelope and code family apply, with stable deterministic order and safe field paths, over the **whole** graph including unlisted representations. This section claims no Stage 00 proof; stage 01/02 acceptance requires recorded pinned tool versions, fixture identifiers, executed commands with exit codes and unmodified raw sample inputs.

**Unsupported-keyword rule (non-waivable).** An unsupported required keyword is never waived, emulated loosely, downgraded to a warning or replaced by a hand-written approximation inside the validator. It is a refusal and a STOP until an Owner-reviewed decision changes the profile.

### 6A.7 `E05` / C1 — Unicode 18 binding, locale, URL/IDNA and the route oracle

**C1-d — standard data binding (documented compatibility, not conformance).** ICU4X `icu_normalizer`, `icu_casemap`, `icu_locale`, `icu_locale_core` and `icu_properties` at exactly `=2.4.0` with their locked versioned data packages are selected because ICU4X 2.4 reports Unicode 18 / CLDR 49. This is **documented upstream compatibility, not yet direct conformance evidence.**

**Mapping profile, exactly as §6.2J.4 requires.** NFC from ICU4X for normalisation. For insensitive route comparison, full **default** case folding followed by NFC. For the insensitive canonical Page-route spelling, Unicode 18 **lowercase mapping** followed by NFC. For sensitive and unlisted Page-route tokens, exact NFC with authored case preserved. **No locale-specific route tailoring** in this first profile. Rust `std` `to_lowercase()`, JavaScript `toLocaleLowerCase()`, OS Unicode tables and outdated ICU are forbidden for authoritative routes. Full case folding, lowercase mapping and IDNA are three distinct operations and are never substituted for one another. The Unicode 18 property table validates letters, marks and decimal digits for route tokens, rejects controls and bidi, and enforces the §6.2J source grammars.

**Executable golden conformance set (future gate).** Pin the official Unicode 18.0.0 `NormalizationTest.txt`, `CaseFolding.txt`, `SpecialCasing.txt`, the Unicode property source and UTS #39 `confusables.txt` by upstream exact version with recorded file SHA-256 at acceptance. The corpus must include the complete applicable NFC conformance data, default fold mappings including multi-scalar expansions, code points new to 18, H1 anchors and the actual route claims. Expected values must come from an independent reference derivation, **not** from calling the same production helper. Include `Straße`/`STRASSE`, Greek sigma and final sigma, dotted I, NFC composed and decomposed forms, Turkish I without locale tailoring, the non-Latin additions in 18, combining marks, cross-script confusable lookalikes (diagnosed, never invented as an equivalence) and forbidden bidi controls. UTS #39 skeletons come from the **pinned 18.0.0 UTS security data**, not from an assumed ICU4X built-in API; the derived read-only lookup table is explicitly versioned and hashed, with the ambiguity refusals and warnings §6.2J requires. An unknown Unicode data version or any discrepancy refuses publication and STOPs.

**Locale binding.** Site default and representation identity remain explicit full **region-qualified** BCP 47 with optional canonical script (`en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`); a bare `en` or `de` never becomes a public representation key. `languagePreferences` maps only declared request/browser preference forms to actually supported full locales — no inferred region or place and no implicit `en`→`en-001` fallback. The locale segment normalises independently to the lowercase full path tag while a sensitive or unlisted Page portion keeps its authored case. Selection order is saved supported preference, then accepted and mapped `Accept-Language`, then the Site default; an explicit full-locale URL always wins; only a real same-Page public missing translation falls back, otherwise not-found. Malformed locale and `q`-value parsing and rejection are deterministic, with no dependence on a variable OS locale registry.

**URL and IDNA.** `idna =1.1.0` with `idna_adapter =1.2.2` canonicalises the host and is tested against the accepted IDNA/UTS46 base-host cases independently of Unicode Page-route processing. DNS name or IP, authority and the optional 0–5 segment ASCII base prefix are authored; authority is never derived from `Host`, `X-Forwarded-*`, geolocation or locale. One controlled percent-decode of the raw request target, validation of encoded separators, dot segments, malformed UTF-8, double decoding, port and host, and then comparison against the validated claim ledger — with **no** server or framework path repair before core validation. The complete canonical/alias/entry claim ledger is built at publication; distinct sensitive `About`/`about` and unlisted `Abc`/`abc` are permitted where safe; sensitive/insensitive overlap and the stronger public/unlisted caseless collision are refused, even for public-sensitive claims; there is no first-match precedence. Canonical corrections and base aliases use a stable `308` where allowed, locale/preference/fallback use `302` nonshared no-store, unknown is `404`, bad encoding `400`, unsafe host `400`/`421`, wrong method `405` and unexpected failure `5xx`. Cache and canonical targets always come from the active sealed artefact, never from the incoming URL or a data filename. Mutation and invariant tests show location never affects route, Page identity or locale.

**C1-e — route oracle proof (future gate).** An executable table-driven fixture matrix maps authored routes to the expected response tuple — status, canonical `Location`, `Cache-Control`/`Vary`, Page ownership, exposure and body hash — across Windows and Linux and across delivery adapters. It uses independent fixtures for `de-DE`/`de-de` locale folding, Unicode 18 NFC, UTS #39 warnings and percent-encoding, and refuses the ambiguous public/unlisted cases. It asserts that a direct full-locale request ignores cookies, that preference-dependent entry is never shared-cached, that exactly one base prefix applies, that a sensitive wrong-case request is not-found, and that an unlisted exact URL is `noindex, nofollow` with no generated discovery. The suite must **fail** when given a deliberately older Unicode profile, a lowercasing-as-folding substitution, or a disabled collision check.

### 6A.8 `E06` / C2 — Confinement, bounded media and the sealed publication artefact

**Source boundary.** The §6.2C locator rules are unchanged: portable lowercase-ASCII relative locators, ≤240 bytes, ≤8 segments, with source extensions governed by the accepted Charter. Absolute, UNC, traversal, dot, backslash, percent, reserved Windows stem, case/short-name/Unicode alias, symlink/junction/reparse, mount escape, hardlink identity sharing and unprovable races are all refused. The namespaces `content/pages`, `content/assets`, non-published `content/drafts`, the product-distributed `platform/assets` and `contracts` stay separate. Only the manifest claims publication; a genuinely absent JSON candidate may select a *declared* Markdown source, while an inaccessible or corrupt JSON source is an error and never treated as absence.

`SourceRead` works through handle-relative confinement using `cap-std =4.0.3` plus OS-specific verified descriptor, reparse, file-identity and link logic. Linux `openat`/safe no-follow restrictions and Windows handle/reparse/volume/alias checks must each be **actually exercised on their own platform**; an unsupported safety guarantee fails closed. Immutable copied bytes plus metadata and hash are held per candidate, resampled where required, and any change is rejected as `SNAPSHOT_CHANGED`. String-prefix safety fiction is prohibited.

**Asset pipeline.** Typed Entity and platform asset references only. Initial bounds: PNG/JPEG/WebP/AVIF ≤20 MiB, ≤8192 px per side, ≤32 M pixels; MP4 AVC with optional AAC-LC or WebM VP9 with optional Opus ≤200 MiB and ≤180 s; WebVTT ≤500 KiB through a safe parser. No SVG, GIF, external URL, unapproved embed/codec or inferred generic artwork. MIME, magic bytes, codec and decoded structure are verified in sandboxed bounded offline inspectors, and any decompression bomb, panic or timeout refuses the **whole** publication. Native AVIF and FFmpeg are used only after platform build and licence/notice proof. Derived immutable assets are outputs, never authored sources and never an undeclared fallback. Generic role fallback remains **zero** per §6.2I. Every published derived asset carries its exact content hash, MIME type, content length, provenance and public-eligible exposure.

**Single portable sealed artefact.** `SealPublication` consumes **one fully validated immutable candidate** covering every published representation, public and unlisted, and emits a read-only *content-addressed* directory or object set containing: the manifest schema/registry/product compatibility tuple and normalised safe base; the complete route/locale/visibility/alias/entry claim ledger (a derived structure, never a second authoring manifest); pre-rendered public HTML and response metadata for each authorised representation including the exact unlisted `noindex, nofollow` response, plus hashed browser assets; immutable typed media outputs and hashes; public-only discovery data derived from authorised public representations; and the object digests, total artefact digest and public exposure boundary. Internal source values, private values and provenance are never exported into a publicly retrievable file. Generated metadata has no publication authority — the installation's `content/publication.json` remains the sole source authority — and in-memory validated models containing internal facts stay private during seal generation.

### 6A.9 `E06` / C2 — Activation, interchangeable adapters and honest delivery modes

**Activation.** `ArtifactWriter` writes into a staging namespace that is not live and not public, fsyncs and verifies as supported, re-reads every size and digest and produces a **sealed immutable snapshot ID**. `ActiveSnapshot.compare_and_swap(expected_id, new_id)` is the **only** activation operation: after all checks pass a **single atomic pointer** changes, and on mismatch or error the old ID stays intact. Identity is the hash of a canonical versioned seal index over sorted paths, digests, contract tuple and base — never mtime or directory-listing order. Request resolution pins the active snapshot ID at entry and uses that same ID for every page, asset, redirect, canonical, cookie and cache decision in that response, with no mutable source access and no unvalidated fallback. Activation is refused when the expected artefact is missing or corrupt, or when the host cannot make the pointer change atomic. Revalidation and staged replacement never destroy the last valid seal, and shared caching of preference-selected redirects is forbidden.

**Three interchangeable activation adapters, with capability criteria.** (1) A local filesystem adapter for preview and local use: same-filesystem staging plus rename of a *pointer file*, with `fsync` and directory durability where demonstrable; it claims no atomic directory swap on every OS and, where stable durability is unavailable, is labelled process-local preview only. (2) An object-store/CDN adapter, as a **future provider-specific implementation only**: write immutable objects, then a conditional strong ETag or equivalent linearisable pointer compare-and-swap; a host whose eventual consistency or partial visibility prevents pinning is refused. (3) An in-memory `Arc` pointer for tests and transient on-demand invocations, explicitly not a durable or multi-instance publication guarantee. Retention, recovery, provider, SLA and durable persistence policy remain `R00.G`; this section fixes interfaces, invariants and refusal conditions only.

**Delivery modes.** Mode A, static exact-route artefact serving, may serve verified pre-rendered canonicals and assets without a Solo server **only where the real hosting configuration enforces every authored route, alias, canonical, cache, not-found and exact-case unlisted outcome**. Mode B, a request-aware resolver, is required for any behaviour a host cannot otherwise guarantee. The same transport-neutral `ResolveRequest` runs in a long-lived Axum server, an on-demand function or a supported gateway. Neither Node nor Axum is required to host an already sealed static file set, and **no always-on Rust process is mandated by the product architecture**. A provider that cannot support the required route semantics cannot honestly claim complete Solo conformance for that Installation — it must fail rather than simulate. The later `R00.F` contact form will require an approved replaceable submission endpoint; no serverless vendor or paid service is preselected here. Supported deployment modes and recovery commitments remain `R00.G`.

**Delivery proof (future gate).** Before any platform or mode is claimed supported, the identical fixture set `RequestCase[] → ResponseSpec` runs against (i) the application fake store, (ii) a real artefact-file preview, (iii) actual production-mode Axum and (iv) the candidate static or on-demand provider once chosen. Coverage includes `302`/`308`/`400`/`404`/`405`/`421`/`5xx`, case and locale, unlisted headers and discovery, old-versus-new snapshot responses under concurrent activation, torn staging, stale or mismatched compare-and-swap, missing or corrupt objects, hash mismatch, failed media or source validation, restart, cache identity and cold start. Deliberately removing an object, disabling compare-and-swap or disabling case-sensitivity in the handler must make the gate fail. A passing production server never proves a generic static provider's correctness.

### 6A.10 `E07` / C1 — Markdown profile and safe rendering

**Parser decision.** `comrak =0.56.0` inside a dedicated profile adapter, selected for **CommonMark 0.31.2 plus admitted GFM 0.29-gfm** extensions: tables, task lists, strikethrough and autolinks. Footnotes, math, diagrams, directives, front matter, MDX and every custom syntax are forbidden. Exactly those admitted extensions are enabled; the accepted profile is not "whatever the parser defaults produce".

**C1 Markdown proof (future gate).** Execute the pinned upstream CommonMark 0.31.2 and GFM 0.29-gfm conformance fixtures plus Solo refusal fixtures, recording passed and failed counts, expected-versus-actual AST and the exact selected parser options. If Comrak differs from the accepted grammar, an explicit bounded compatibility adapter is required **and tested**, or the work STOPs for an Owner-reviewed amendment. The accepted profile is never quietly replaced by a different grammar.

**AST refusal happens before rendering.** Raw HTML block and inline HTML nodes are rejected at their original source offsets *before* any rendering, even where a default renderer would sanitise or silently omit them. HTML inside fenced or inline code renders escaped. The rule does not rely on the removed or deprecated GFM tag filter.

**Links, media and anchors.** Rejected schemes and forms: `javascript:`, `data:`, `file:`, `blob:`, `vbscript:`, protocol-relative, malformed percent encoding, userinfo credentials and unknown schemes. Only validated `http`/`https` click-through and a governed single-recipient `mailto` with optional `subject` are allowed, and external links or images are never fetched server-side. A normal Markdown link resolves per RFC 3986 against the **canonical Page URL**, then against the route ledger, visibility and base-prefix safety; its fragment must name an anchor that is actually generated. The image form `images/x.webp` always maps to `content/assets/images/x.webp` and never relative to the `.md` source path, with alt/decorative semantics and media-namespace checks applied.

**Headings and anchors.** A standalone Markdown body's first non-blank semantic block is exactly one non-empty H1, with later headings H2–H6 and no level jump greater than one. A JSON body's `title` produces exactly one H1, which a hero may host. `content.prose` forbids headings. The anchor algorithm is the accepted §6.2D rule: visible heading text → NFC → Unicode 18 lowercase → keep Unicode letters, numbers and hyphens → collapse whitespace runs to `-` → strip other punctuation → trim hyphens, with an empty result becoming `section` and duplicates suffixed `-2`, `-3` in document order.

**Rendering safety.** Every interpolated value and attribute is escaped, URLs are built only from typed validated targets, and there is no authored inline script or style and no `dangerouslySetInnerHTML` analogue. The renderer emits semantic headings, inert code and read-only accessible task checkboxes. Diagnostics identify safe EFS lineage, the logical locator and source line/column — never an absolute private path.

### 6A.11 `E08` — The twenty-two registered `1.0.0` modules: rendering and behaviour decision

The installed registry stays the immutable product-owned `ModuleContract {id, exact_version, closed_schema_id, defaults, allowed_children, allowed_presentation, required_ports, public_projection, renderer_id, behavior_spec}` catalogue of §6.2E.2. It contains **exactly** the twenty-two IDs below at `1.0.0`, each with its full §6.2E.2 field grammar, defaults, bounds, conditional refusals and behaviour **by reference to those governing literals**, encoded into typed definitions, generated closed schema and versioned golden contract fixtures. No literal in this table narrows or widens §6.2E.2; where a summary here omits a field, §6.2E.2 still binds.

**The technical decision fixed here:** publish-time Maud pre-rendering for structural content (reusable by an optional request-time renderer); a progressively enhanced TypeScript controller **only** for inherently interactive modules; no React runtime, no authored JavaScript/CSS/HTML and no runtime module loading. All renderer arguments are validated typed **public-projected** values, and direct filesystem, network or secret access from any module is forbidden. Each registry entry carries one positive fixture, one unknown-field fixture, one invalid-default/child fixture, one keyboard/accessibility/visibility fixture where applicable, and a pixel or DOM regression where meaningful.

| Registered module (exact ID) | Rendering/behaviour adapter and required extra tests |
| --- | --- |
| `layout.stack` | server-rendered logical flex order; default vertical, gap `md`, `align=stretch`; horizontal wraps; 1–24 children |
| `layout.grid` | server-rendered CSS grid with safe responsive `base`/`md`/`lg` spans, 1–24 children, any span beyond the column count refused |
| `layout.bento` | server-rendered ordered CSS grid with a dense-looking layout and no semantic reordering, 1–16 children, overflow refused |
| `layout.layers` | server-rendered relative-positioned paint order, 1–12 children; default `minHeight=md` (**400 px minimum**), `align=center` on the **vertical** axis; obscured essential or focusable content refused |
| `content.heading` | server-rendered H2–H6 only; document-wide no level jump and no second H1 |
| `content.prose` | server-rendered sanitised **validated AST**, 1–20,000 scalars, headings and raw HTML refused |
| `content.actions` | server-rendered typed safe links, 1–8; omitted `variant` is exactly `text`; visible focus and label; no form or transaction implied |
| `content.hero` | server-rendered root title as the single H1 inside the hero with safe optional eyebrow/tagline/action/background; no duplicate title |
| `media.image` | server-rendered image with required alt or decorative semantics, validated dimensions, default `fit=cover`, no layout shift |
| `media.video` | server-rendered native accessible controlled video, poster plus full-locale captions, verified MP4/WebM, `controls=true`, no `autoplay=true` and no external embed |
| `media.gallery` | server-rendered ordered responsive 1–24 images, default three columns, validated captions and alt |
| `media.carousel` | server-rendered first index **0**, 2–12 slides; TypeScript previous/next single-step cyclic wrap, visible Start/Pause, exact autoplay/hover/focus/manual-pause state machine; `intervalMs` only when autoplay is true, default 10000 ms, reduced motion disables cycling |
| `design.watermark` | server-rendered inert decorative text or asset, **exactly one**, bounded opacity and position defaulting to centre, no contrast or essential-information harm |
| `design.motion` | server-rendered visible static content plus optional fade/slide/reveal/marquee/parallax with exact defaults and a reduced-motion branch; parallax `p = clamp((H−T)/(H+E), 0, 1)`, `y = (2p−1)×24 px`, 800 ms default or 3000 ms for marquee, no overshoot, marquee has visitor start and pause, conditional direction fields refused |
| `interaction.tabs` | server-rendered valid first/initial selected panel; TypeScript ARIA tablist and roving tabindex, arrow/Home/End keys, 2–8 unique IDs, exact `initialTab`; without JavaScript all public content stays available |
| `interaction.accordion` | server-rendered native disclosure or semantic buttons plus TypeScript, 1–12 items, `singleOpen=true` default, omitted `initialOpen` means all closed, keyboard and accessible IDs |
| `interaction.modal` | server-rendered content and visible trigger; TypeScript dialog focus trap, Escape and close, focus restore, scroll handling; without JavaScript a non-hidden accessible content path |
| `content.stats` | server-rendered 1–12 inert authored facts; no analytics counters |
| `content.testimonials` | server-rendered attributed static quotes; no fetched review assertions |
| `content.pricing` | server-rendered 1–6 authored text plans and optional approved links; never billing or checkout |
| `content.timeline` | server-rendered 1–24 authored items in authored order; no inferred place or timezone semantics |

### 6A.12 `E08` / C4 — Module-to-port, validator and deliberate-negative register

Shared parsers, schema validator, module registry and public renderer validate and render every named module; each row below names the module's **additional** use-case, adapter or safety gate and its deliberate refusal. No module can invoke a source, HTTP, network or provider path directly, and a typed asset in props is resolved only through the application-approved `AssetInspect` port during publication.

Every row additionally requires: invalid exact contract version, unknown prop, forbidden child, extra envelope field, malformed presentation, internal-value injection, and an output containing no user-authored code.

| Module | Additional validator/port | Deliberate negative case | Visible / client evidence |
| --- | --- | --- | --- |
| `layout.stack` | node child resolver plus responsive renderer | 0 or 25 children, invalid gap or direction | horizontal wrap and authored reading order |
| `layout.grid` | closed responsive columns/span validator | span exceeding base/md/lg width | DOM order, 768/1200 breaks, no focus overflow |
| `layout.bento` | bounded grid solver | 17 children or illegal span | DOM order independent of visual packing |
| `layout.layers` | validated layout and overlap safety gate | covered essential control or clipping, invalid `minHeight` | vertical alignment, 240/400/640 minimum, visible focus |
| `content.heading` | cross-node heading validator | H1 or level jump, unknown level | H2–H6 with exactly one H1 globally |
| `content.prose` | `MarkdownProfile` plus link gate | HTML, heading, unsafe link | CommonMark/GFM, safe inert markup |
| `content.actions` | typed `ResolveLink` through the application | unsafe external scheme, missing label, 9 items | omitted variant is `text`, focus treatment |
| `content.hero` | title/H1 host plus safe media gate | duplicate H1, unsafe background | exactly one H1 in hero, escaped text |
| `media.image` | `AssetInspect` plus public asset projector | wrong MIME, oversize, invalid alt | known size, accessible alt or decorative |
| `media.video` | `AssetInspect` plus WebVTT validator | missing captions, forbidden autoplay or codec | accessible native controls, no autoplay |
| `media.gallery` | multiple `AssetInspect` validations | 25 images, bad member alt or asset | responsive authored order and captions |
| `media.carousel` | `AssetInspect` plus typed `BrowserEnhancementV1` | interval when autoplay is false, bad 2–12 count, inactive focus | slide 0, cyclic step, start/pause/hover/focus/reduced motion |
| `design.watermark` | typed decorative projection plus contrast gate | both or neither of text/asset, opacity above 0.4 | ignored by assistive content, no concealment |
| `design.motion` | enhancement enumeration plus duration/direction gate | direction on fade or parallax, out-of-range duration | all five effects, parallax p/y interpolation, reduced motion |
| `interaction.tabs` | recursively closed tab content plus enhancement guard | duplicate tab ID or invalid `initialTab` | roving focus, Home/End, no-JS accessible content |
| `interaction.accordion` | recursively closed item content plus enhancement guard | duplicate ID, unknown `initialOpen` | none open initially, `singleOpen` semantics, keyboard |
| `interaction.modal` | recursively closed content plus enhancement guard | malformed trigger, unknown content module | focus trap, Escape, restore, no-JS content path |
| `content.stats` | inert public-facts projector | 13 entries or an extra statistic field | no timer, network or analytics |
| `content.testimonials` | bounded authored-quote projector | overlength quote or unknown review provider | attributable escaped content |
| `content.pricing` | safe action-link and plan renderer | seventh plan, 16 features, unsafe link | presentation only, no transaction |
| `content.timeline` | bounded authored-sequence renderer | 25 events or unknown timestamp semantics | authored chronological order only |
| `interaction.contact` | server-rendered semantic HTML form from public-projected localized `labels`, `notice` and `feedback` only; submissions post to the server-owned **public** form action with non-secret CSRF and attempt control tokens, going to `SubmitSimpleContact` over the reserved §6A.4 ports; the internal recipient or address, delivery-provider configuration, credentials, internal provider endpoint and transport diagnostics never appear in public output | unknown prop or extra field, a second `interaction.contact` node in one representation, a notice link that is unlisted/private/unsafe/non-`https:` external, missing delivery configuration, or a CSRF/origin failure | escaped inert labels and feedback; the form is fully operable without JavaScript; it exports no `BrowserEnhancementV1` payload and no outcome copy may claim inbox receipt |

**Boundary assertions per module.** `BrowserEnhancementV1` DTOs must not inherit arbitrary props or nested raw module source; the browser transport is only the explicit whitelisted controls of the five browser-enhancement modules. **Each of those five — `tabs`, `accordion`, `modal`, `carousel` and `motion` — and only those five** must supply (a) server golden DOM with a valid `data-solo-envelope` and isolated instance scope, (b) build-time generated TypeScript parity fixtures plus runtime decoding, (c) bad, forged and oversize envelope refusal without script execution, and (d) real keyboard, reduced-motion and no-JS tests. Non-interactive modules export **no** browser behaviour payload. **`interaction.contact` is not a browser-enhancement module:** it is a server-rendered, standards-based HTML form that must carry no `data-solo-envelope`, export no `BrowserEnhancementV1` payload, and require no DTO parity fixture or client-runtime decoder; generic closed-module validation, safe rendering, and real keyboard, no-JavaScript form and refusal tests apply to it instead. Its `interaction.` prefix confers no browser-enhancement status, and it must not grow a sixth `kind`, a special browser controller, arbitrary authored script, an extra JSON node property or an alternative Page-enablement mechanism. Every renderer test also asserts the absence of internal properties, filesystem paths, direct module network calls, `innerHTML`, unreviewed `import()` and arbitrary authored styles.

**Binding regardless of summary.** All §6.2E.2 precise props, ranges, conditionals, child counts, presentation tokens, the 768/1200 CSS-px breakpoints, layer geometry and no-essential-overlap rule, carousel state transitions and no-JS/reduced-motion semantics remain binding even where a summary row omits a literal field. `media.carousel` never preloads undeclared assets and inactive slides never leak focus. Tests mutate every schema-closed prop to demonstrate `MODULE_INVALID_PROPS`; an unknown ID or version yields `MODULE_UNAVAILABLE`. The `R00.F` simple inquiry form **is** the registered `interaction.contact` module of §6.2E.2 — never an unregistered, concealed or Page-overlay form — and no advanced form-builder, CRM, commerce or dormant provider feature is ever added. Any further module requires a separately approved versioned registry entry and compatibility matrix, and "twenty-two implemented" may not be claimed until every one has verified browser and contract behaviour.

### 6A.13 `E09` — Test architecture, the C1–C4 proof assignment and honest status

| Condition | Implementation port/adapter | Positive conformance source | Mandatory deliberate rejection / evidence gate | Status during R00.E |
| --- | --- | --- | --- | --- |
| C1 parser | `JsonDocumentParser` / strict Serde visitor | RFC 8259 valid nested UTF-8 JSON and the §6.2B normative examples | duplicate decoded escaped keys, BOM, invalid UTF-8, oversize, trailing garbage; no `Value` materialisation before the key check | **documented, not executed** |
| C1 JSON Schema | `SchemaValidator` / `jsonschema 0.60.2` with an offline Draft 2020-12 registry | pinned official draft2020-12 suite plus every generated §6.2 schema | unsupported `unevaluatedProperties` after `$ref`/`anyOf`, disabled format assertion, bad vocabulary or remote reference; any failing required case STOPs | **documented, not executed** |
| C1 Unicode | `UnicodeProfile` / ICU4X 2.4 with versioned UTS #39 data | Unicode 18 NormalizationTest, CaseFolding, SpecialCasing, UCD and independent route goldens | older Unicode data, locale-sensitive folding, JavaScript lowercase fallback, confusable ambiguity, bidi, double encoding | **documented, not executed** |
| C1 Markdown | `MarkdownProfile` / Comrak 0.56 AST | CommonMark 0.31.2 and accepted GFM 0.29-gfm fixtures plus raw-HTML refusal | invalid HTML, unsupported extension, wrong anchors, heading jump, unsafe URI | **documented, not executed** |
| C2 delivery | `ArtifactWriter`, `ActiveSnapshot`, `ArtifactReader`, `ResolveRequest`, `ResponseDelivery` | sealed artefact and revision-stable output on file preview, on-demand resolver and the chosen provider | stale or missing seal, torn or unsafe compare-and-swap, provider case mismatch, shared-cache preference redirect | **specified, no provider tested** |
| C3 hexagon and wire | Cargo crate DAG, `syn` checker, generated `BrowserEnhancementV1` TypeScript | allowed imports, generated Rust/TS golden wire, browser server-render and TypeScript modes | direct, re-export, alias and dynamic bad edges; cross-serialisation XSS, private leak, forged instance | **specified, not executed** |
| C4 traceability | `ValidateCandidate` and the twenty-two module validators and renderer pairs | `D01`–`D12`, `O01`–`O12`, `V01`–`V28`, `P01`–`P14` ledgers and all twenty-two registered IDs | at least one meaningful failure for every contract, case and module; the test fails if the refusal is disabled | **mapped, not executed** |

**A runtime PASS requires exact test outputs at a later authorised code-stage revision.** Upstream release notes never constitute that PASS. Because Stage 00 forbids adding code or tests merely to demonstrate the stack, C1 is **`UNVERIFIED`/NOT RUN** in this documentary state and cannot be marked accepted on documentary grounds alone. A code-bearing implementation may not claim acceptance while any requisite test is unrun, skipped or failing, and every case keeps an expected diagnostic and a named future test path that is independently checked against the `R00.D` source text.

**Chosen test stack (future implementation).** Rust built-in `cargo test` for domain, unit, integration and contract assertions. `proptest =1.12.0` for bounded Unicode routes, path graphs, identity and property invariants and decode adversaries. Table-driven checked fixture files and golden determinism. A `syn =3.0.9` AST-based `architecture-check` with module-path, alias and re-export resolution plus its mutating negative-fixture meta-tests. `@playwright/test =1.64.1` with pinned Chromium, Firefox and WebKit binaries for keyboard, focus, responsive, reduced-motion and screen-reader-oriented ARIA/DOM checks. `@axe-core/playwright =4.14.0` as a supplementary pinned automated check that never replaces manual keyboard or assistive-technology inspection. **No mock-only acceptance.** For code-bearing changes the run must include real no-framework application composition, actual source and media adapters, a sealed artefact, a real production-equivalent request gateway with actual HTTP requests and cross-browser behaviour. Each run records the exact revision, environment, exit code, stdout summary, skipped/failed list and reproducibility command, with no blanket `skip`, silent warning suppression or fabricated green.

### 6A.14 `E09` — Contract coverage map from accepted rules to pipeline boundaries

Every `D` and `O` entry below remains previously accepted and unchanged; this table only names the boundary that will implement it and the evidence it must later produce.

| Accepted rule group | Concrete pipeline / ownership boundary | Required future evidence (positive **and** negative) |
| --- | --- | --- |
| D01/O01 | strict JSON parser plus local Draft 2020-12 schema and format gates | `P01`: exact `1.0.0` manifest, unknown field, BOM, duplicate escaped key, bad version, oversize and malformed JSON refusals |
| D02/O02 | domain nested one-Entity EFS and qualified-reference validation | `P02`, `P13`, `V01`–`V03`: one default, expanded valid scoped ID reuse, invalid ID, second Entity, orphan, bare locale |
| D03/O03 | source port and alias-safe immutable candidate selector | `P03`–`P04`, `V04`–`V06`, `V22`: valid JSON-first or absent-JSON Markdown, invalid preferred JSON refusal, hardlink/absent-alias/case/permission/symlink/race refusals |
| D04–D05/O04 | selected safe JSON/Markdown AST, links and anchors | `P08`, `V07`–`V08`, `V23`: reference GFM corpus; raw HTML, unsafe schemes, duplicate H1, broken anchor, unsupported directives |
| D05/O05–O06 | exact built-in twenty-two contract registry, schema plus server-render and client adapters | `P09`, `V09`: all twenty-two minimal valid; unknown versions, props, conditional defaults and children; carousel, layers, motion, focus, reduced-motion and contact-module refusals |
| D08/O07 | domain direct-ancestry properties and public projection | `P05`, `V16`–`V17`: inherit/suppress, cardinality before privacy filtering, provenance, internal and suppressed leak negatives |
| D09/O08 | typed location property facts, identified `extend`, provenance | `P06`, `V18`–`V19`: Australia/Indonesia/Bali/Jakarta, valid in-place update, sibling/suppressed/duplicate invalid extension, no routing effects |
| D10/O09 | typed namespace asset inspector and no-fallback graphics | `P07`, `V20`–`V21`: Entity versus platform, no generic fallback, codec/MIME/oversize/corrupt/VTT/SVG/GIF refusals |
| D06–D07/O10 | domain route, locale and visibility policy plus ICU 18 and HTTP ingress | `P10`–`P11`, `V10`–`V15`: insensitive/sensitive/exact unlisted, caseless collisions, NFC and multi-fold, prefix, IDNA, preference/redirect/cache and no-discovery |
| D11/O03/O04/O10 | source, asset and typed link resolver | `P11`, `V22`–`V23`: RFC 3986 relative URL, controlled decode, scheme and base traversal, a known unlisted direct link that produces no generated discovery |
| D12/O11 | ten-stage validation, closed diagnostics and `PublicationStore` atomic swap | `P12`, `V24`–`V25`: unknown version, one malformed item invalidating everything, snapshot change, old publication preserved under concurrent readers |
| O12 / architecture | one canonical D/O/`V`/`P` traceability plus strictly inward ports | `P14`, `V26`–`V28`: no contract drift, alias/re-export/dynamic import injected negatives, no Spokes, advanced form or fictional release/test claim |

**Individual `P01`–`P14` assignment.** The accepted case text is unchanged; this is the implementation-to-test assignment only.

| Charter case | Runtime/fixture gate and required refusal |
| --- | --- |
| `P01` | strict UTF-8 `publication.json` parser, schema version, duplicate key, BOM and unknown key refusal |
| `P02` | nested five-level EFS schema and domain, identity scope and full locale; orphan and second Entity refused |
| `P03` | source port genuine absence versus inaccessible or present-invalid JSON; JSON-first and none-effective refusal |
| `P04` | JSON/Markdown reserved candidate claims, absent and hardlink and caseless-alias collision refusal |
| `P05` | property inheritance, suppression, provenance and pre-projection maximum; private leak refused |
| `P06` | identified fact extend in place with `fieldOrigins`; absent, cross-branch and suppressed double extension refused |
| `P07` | two asset namespaces and media bytes, MIME/codec/dimension/size and no-fallback refusal |
| `P08` | reference CommonMark/GFM grammar and anchors; HTML, directive, H1 and scheme refusal |
| `P09` | twenty-two exact module contracts, defaults, forbidden conditionals, accessible behaviour and reduced motion |
| `P10` | Unicode 18 canonical and caseless route and locale ledger; encoded delimiter, bidi and collision refusal |
| `P11` | typed links, safe `mailto`, known unlisted explicit link; unsafe, remote-fetch and discovery refusal |
| `P12` | exact version tuple, full stage validation, snapshot and atomic activation; old snapshot preserved on failure |
| `P13` | minimal starter and expanded multi-Site/Facet installation; no host, filename or geo-derived Entity |
| `P14` | D/O/`V`/`P`-to-test audit and evidence gate; stale, undecided and fictional-execution claims rejected |

**Individual `V01`–`V28` assignment.** No case is elided by a grouped range; the accepted case text stays in §6.3.

| Charter case | Future boundary / code-stage proof | Specific negative or decisive assertion |
| --- | --- | --- |
| `V01` | domain EFS owner and cardinality validator | second Entity, orphan or empty ancestor rejected |
| `V02` | scoped technical IDs and lineage indexes | invalid 64-byte or uppercase token and duplicate within one parent refused; reuse across parents succeeds |
| `V03` | ICU BCP 47 representation identity | `de-DE`/`de-de` duplicate and bare `de` refused |
| `V04` | source declaration index | directory or draft presence never publishes; both declared candidates recognised |
| `V05` | JSON-first source selector | a present but invalid or inaccessible JSON source never falls back to Markdown |
| `V06` | filesystem identity and claim ledger | the same source, including an absent alias, refused twice; no sibling source chosen |
| `V07` | typed body/AST-to-render adapter | invalid module or raw HTML blocks everything; equivalent JSON and Markdown intent both work |
| `V08` | versioned Markdown and JSON module grammar | no custom Markdown directive and no executable JSON content accepted |
| `V09` | immutable twenty-two module registry | unknown ID, version, prop or privileged port fails; no form builder |
| `V10` | visibility, domain projection and HTTP | four-case Page/representation matrix, unlisted `noindex` and absent discovery |
| `V11` | base URL builder | origin versus prefix applied once; aliases redirect without changing identity |
| `V12` | preference port and request test | saved preference, then mapped `Accept-Language`, then default; direct full locale overrides |
| `V13` | same-Page fallback ledger | cross-Page or hidden-variant substitution refused or not-found |
| `V14` | ICU route matcher | insensitive public, sensitive public and unlisted exact; locale independent |
| `V15` | NFC canonical and caseless route ledger | normalisation collision and public/unlisted alias overlap fail |
| `V16` | pure property cascade | absent, inherit, suppress and none behaviour with correct effective cardinality |
| `V17` | derived provenance and public-only projection | private ancestor or suppressed values never appear in serialised public output |
| `V18` | identified location add/extend | cross-branch, suppressed, duplicate and circular extension fail |
| `V19` | location-routing independence | several location facts never guess a primary or alter route or locale |
| `V20` | property graphic resolver | cardinality and namespace collision; zero automatic library fallback |
| `V21` | module media adapter | a typed direct asset is never converted into an EFS graphic role |
| `V22` | capability filesystem | traversal, junction, symlink, hardlink and race refused; no source from drafts or generated output |
| `V23` | canonical URI-aware Markdown links | fragment, relative link and image correct; dangerous schemes and out-of-prefix targets refused |
| `V24` | compatibility matrix and migration | unknown contract versions refuse; no automatic rewrites |
| `V25` | snapshot activation | an invalid new candidate never alters the active old publication |
| `V26` | architecture checker and port tests | injected direct, alias, re-export and dynamic forbidden imports fail |
| `V27` | documentary diff and presentation tests | all seven design-reference URLs retained and only governed modules express capabilities |
| `V28` | documentary cross-check and release acceptance | no Spokes, geo routing, advanced forms, provider claims or contradictory physical literals |

**Mandatory adversarial and meta-test gates.** Each gate is a refusal gate: it must fail when the protection it covers is removed.

1. Construct a deliberate invalid fixture for every negative statement of `P01`–`P14` and `V01`–`V28`; a refusal test must itself fail if its validator is disabled or if the unsafe fixture is accepted. Track fixture → expected code or status → test path → actual result.
2. Inject a direct adapter import into the domain, a transitive alias, a `pub use` re-export, an out-of-tree `#[path]`, a dynamic import and an illegal module or provider load; the architecture check must fail on each and must pass once the legal build is restored.
3. Alter a JSON source between open, read, hash and validation to prove `SNAPSHOT_CHANGED` with the old publication retained; test nonexistent versus inaccessible JSON as distinct; test persistent claims to absent candidates and an underlying hardlink collision; test a Windows junction and a Linux symlink, mount and case alias on their respective OS, or mark the guarantee UNVERIFIED and STOP.
4. Run the case-route matrix: sensitive `About`/`about` coexist; insensitive `about` rejects a sensitive collision; public-sensitive `Morgen` with unlisted `morgen` rejects; unlisted `abc`/`Abc` stay distinct; `/DE-DE/Morgen` normalises only the locale; confusables are diagnosed without invented equivalence; invalid percent slash, double decode and bidi are refused; full-locale preference never infers geography.
5. Run the property matrix: a private internal ancestor and a suppressed logo never appear in public HTML, JSON props, assets, feeds, metadata, `hreflang` or sitemap; effective cardinality is checked before filtering; cross-branch location extension rejects.
6. Run the module fixture ledger: one fixture and browser test per module ID; randomised forbidden unknown props; prescribed default and conditional refusals; keyboard arrows and Escape with focus retention; reduced motion; carousel idle, start, autoplay, manual pause and hover; parallax at 0, 0.5 and 1 plus the zero-geometry branch; inaccessible layer overlap rejected; contrast checked against the **later Owner-approved `R00.G`** numeric target while a known unsafe presentation is refused now.
7. Run real production-equivalent request-adapter HTTP: a valid one-Entity public Page, prefix and alias, source-selected Markdown and JSON, `302`/`308`/`400`/`404`/`405`/`421`/`5xx`, cache with `Vary` and `no-store` semantics and actual URL construction; test fully disabled JavaScript for content parity and enabled interaction; cover Chromium, Firefox, WebKit and a mobile viewport. Fake HTTP alone never qualifies.
8. Clean-room rebuild: fresh dependencies from the checked lockfiles on Windows and Linux, stable generated contract artefacts, an offline repeat with no network after cache population, and the source revision with artefact SHA-256 logged. A build without runtime, HTTP and browser proof never qualifies as a full release.

**Implementation staging.** `R01.A`–`R01.C` implement the tooling, the core test and architecture gate, and one narrow inbound-request → use case → fake/outbound port slice with the real transport-neutral resolver plus an optional Axum production-equivalent skeleton. Stage 02 completes the manifest and EFS, Stage 03 property, location and graphics, Stage 04 routing and a real full-Page route slice, Stage 05 Markdown and JSON content, Stage 06 all accessible module behaviours and navigation, Stage 07 assets and SEO, Stage 08 the `R00.F` simple contact form, Stage 09 only Owner-approved links and analytics, Stage 10 `R00.G` lifecycle, Stage 11 security and nonfunctional hardening, Stages 12–13 end-to-end accepted release and distribution. A Stage 01 vertical slice may use a deliberately minimal synthetic read-only Page with strict typed identity, but it must never claim completed `R00.D` publication before later validation exists, and it must invoke a real inbound application port through an actually compiled HTTP handler rather than an unrelated static hello-world path. Synthetic stubs are removed or integrated as the real path arrives and never survive as a production fallback.

### 6A.15 `E10` — Future command contracts, generated truth and CI

**None of these commands exists yet and none has been executed.** A code-bearing stage implements them exactly as specified, and no document, report or chat may state that any of them has run in this repository.

| Command | Expected semantic contract |
| --- | --- |
| `rustc --version`, `cargo --version`, `node --version` | log the exact pinned compiler 1.99.1 and Node 26.13.0; refuse unreviewed drift |
| `cargo fmt --all -- --check` | no formatting drift in the workspace |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | static and lint gate across all four crates |
| `cargo test --workspace --all-targets --locked` | pure domain; parser, schema, Unicode and Markdown official corpora; port fakes; assets; snapshot; generated wire; negative injection suites |
| `cargo run --locked -p solo-cli -- validate --installation <root> --format json` | full §6.2K candidate validation without activation, safe diagnostics, non-zero on invalid |
| `cargo run --locked -p solo-cli -- seal --installation <root> --output <staging>` | one hash-verified immutable candidate artefact or non-zero; no writes to authored inputs |
| `cargo run --locked -p solo-cli -- activate --artifact <id> --expect <old-id>` | atomic compare-and-swap on the approved local preview adapter; mismatch fails and retains the old seal |
| `cargo run --locked -p solo-cli -- preview --artifact <id> --bind 127.0.0.1:<port>` | local real Axum HTTP equivalent against a **sealed** artefact, never a mutable content read |
| `cargo run --locked -p solo-cli -- check-contracts` | regenerate all schema, registry and `BrowserEnhancementV1` TypeScript bindings into scratch, compare tracked exact bytes, reject stale or unknown |
| `cargo run --locked -p solo-cli -- check-architecture` | verify the Cargo dependency DAG, `syn` path/re-export/alias/dynamic restrictions and the injected negative fixtures |
| `cargo build --workspace --release --locked` | reproducible publish/CLI and optional gateway build with artefact and lockfile digests recorded |
| `npm ci`, `npm run typecheck`, `npm run build` | npm v3 lock, strict generated TypeScript compile, versioned hashed Vite progressive bundle |
| `npx --no-install playwright test` | actual artefact HTTP and browser tests in Chromium, Firefox and WebKit with keyboard, mobile, reduced-motion and no-JS cases |
| `npm audit --audit-level=high`, `cargo deny check` | supply-chain, licence and advisory checks with explicit failure or triage and no hidden suppression |

**Source-of-truth hierarchy, in order.** (1) The canonical previously accepted §6.2 physical literals; (2) the `solo-domain` and `solo-application` versioned typed contract and twenty-two module registry implementation consistent with those literals; (3) reproducibly generated schema, TypeScript and registry artefacts and fixture snapshots; (4) the sealed public artefact, derived only after complete candidate validation. `schemars` produces schema candidates, `ts-rs` produces public-client union declarations, and a deterministic check compares generated bytes against tracked files; generation never rewrites accepted literals. A stale schema, registry or TypeScript union is a fatal check failure. The client cannot interpret a new browser schema version without a compatibility-tested explicit version switch — an unknown version means inert behaviour, never execution.

**CI platform.** GitHub Actions with full commit-SHA action pins, a read-only default token and no job holding provider secrets on an untrusted pull request. The matrix covers Windows and Ubuntu confined-file tests, codec build and Unicode source-data conformance, the all-crate Rust DAG with injected failures, JSON Schema 2020-12 and Comrak official fixture coverage, build/lock/licence/advisory checks, no-JS and JS rendered HTML, browser accessibility, process and on-demand gateway conformance, artefact and pointer atomicity and cold start, and the optional provider conformance suite only once deployment is separately authorised. Static hosting behaviour is never claimed from a green Axum integration test alone. Each run retains the exact commit SHA, `rustc` and Node versions, suite dataset releases and hashes, generated schema and TypeScript digests, sealed artefact ID, exit codes, warnings and skips and browser versions. A missing required run or an unsupported OS gate is `UNVERIFIED` or `FAIL`, never PASS.

### 6A.16 Preservation, exclusions and what remains reserved

**Preserved exactly.** One Entity per Installation; explicit stable scoped IDs with five-level nested ownership and stable Page/representation identity; no hidden Spokes or tenant ingress; a configurable safe base URL and aliases independent of identity and location; exact full region-qualified BCP 47 locales; case-insensitive public routing by default with NFC lowercase canonicals, author-sensitive public claims with exact-case canonicals, exact-case unlisted routes and the stricter public/unlisted caseless collision refusal; truthful same-Page public-only locale fallback and unlisted `noindex` with no discovery; JSON-first declared candidates with a strict versioned manifest as the only publication authority; additive property inheritance and suppression with internal-only provenance and exposure; explicit location add/extend facts independent of routes; all nine property keys, twenty-two modules and the complete §6.2 physical published profile; typed Entity/platform assets with zero generic fallback; safe CommonMark/GFM; atomic whole-publication validation; no arbitrary authored code; the safe `mailto:` action and the registered **simple contact form** module `interaction.contact`, with advanced forms permanently excluded; the Apache-2.0 project licence; every existing authorship example and every `D`/`O`/`V`/`P` ledger.

**Excluded from `R00.E` and from this repository.** Application source code; any `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json` or `rust-toolchain.toml`; generated schema output or contract generation; generic artwork; caches; snapshots; CI workflow files; a test suite; binaries; provider or API setup; dependency installation; new dossiers or README updates; release, deployment, secrets, launch, or any claim that a test ran.

**Explicitly reserved at the time, now decided.** **At the earlier `R00.E` edition** the contact field set and bounds, the simple-form placement, the abuse and rate policy, the consent and privacy-notice policy, delivery outcomes, retry and deduplication, and analytics scope were **reserved for `R00.F`; they are now decided** in §6B. Provider adoption and applicable provider-specific privacy and retention terms remain later-scoped, as do `R00.G`'s nonfunctional and lifecycle decisions and the applicable implementation stages. Only the unprivileged typed `ContactDelivery` port boundary was selected in `R00.E`, which resolved no contact design. Lifecycle, upgrade, recovery and support policy, numerical accessibility and performance thresholds, availability targets, production TLS and security, supported static versus on-demand deployment modes, snapshot retention and recovery objectives and actual provider capability remain with the lifecycle/nonfunctional package. The portable sealed-artefact and compare-and-swap safety interface and its refusal rules are fixed **now** and may not be silently weakened later. Future stages supply executable schemas, tested codec binaries, media artwork and the product release — none of which is implied by documentary acceptance. No numeric accessibility target is invented here, while a known inaccessible rendering is refused today.

**Disclosed pre-existing wording mismatches.** The inconsistencies disclosed at the published baseline are recorded here rather than silently resolved; the accepted §6.2 literals govern all code-stage behaviour in every case. Item 3, the package-label split, was conclusively corrected under `SOLO-WO-021 R05` and is retained here only as resolved history.

1. **Planning-level graphic fallback.** `ROADMAP` `R03.C` and `R07.C` still mention an optional generic graphic fallback, and its Stage 03 description uses earlier override language, while accepted `D08`/`D10`/`O07`/`O09` require additive inheritance and **zero** automatic graphic-role fallback. `ROADMAP` is read-only in this order; the correction belongs to a separately authorised roadmap-maintenance action before those packages.
2. **Planning-level source discovery.** `ROADMAP` `R05.B` refers to "discovery", which cannot override the manifest-only, explicitly declared, no-scan JSON-first selector of §1.6 and §6.2C.
3. **Package-label split — resolved.** The earlier mismatch, in which the remaining-obligations list in §6.5 grouped hosting provider, DNS, TLS, environment, secret storage, backup and monitoring under its `R00.F` bullet, is corrected under `SOLO-WO-021 R05`: §6.5 now assigns `R00.F` to contact and integration **decisions only**, and deployment and lifecycle matters to `R00.G`. The **set** of reserved matters is unchanged; only the owning label was wrong, and it is now consistent with `ROADMAP` and §6B.12.

**Future executable proof summary.** Nothing in §6A is a completed conformance claim. The gate that must be satisfied before any conformity statement is: C1-a/b/c parser, schema and semantic validation on official and Solo corpora; C1-d/e Unicode 18 and full route-oracle proof including UTS #39 data; the Markdown corpus proof; C3 `check-architecture` with its injected negative fixtures and the generated Rust→TypeScript wire parity and runtime guards; C2 sealed-artefact, compare-and-swap and real delivery proof per supported mode; and C4 the complete `D`/`O`/`P`/`V` and twenty-two-module refusal ledger. Each is `UNVERIFIED` now, and each must fail when its protection is removed.

## 6B. `R00.F` — Owner-issued contact and integrations contract (documentary only)

**Status and authority.** This section records the **Owner-issued** first-release contact-and-integrations contract (`SOLO-WO-021`, edition `R04`). It settles **documentary decisions only**. It is **not** an implemented capability: no form code, provider account, credential, schema artefact, endpoint, test suite or deployment exists, and nothing here is evidence that any was created. Executable proof of the rules below is a **non-waivable future gate** (§6B.10), not a completed one — every `F` case is unrun. The first-release numeric, rate, consent, retention and module-placement policies are **Owner-decided** in §6B.3–§6B.9 and consolidated in §6B.11; only genuinely later-scope provider and deployment matters remain open, and none of them reopens this contract.

### 6B.1 Preserved literals — nothing in this section supersedes them

These accepted contracts are reasserted, not amended:

- the governed **`contact.email`** property — `{ "address": <valid mailbox>, "exposure": "public"|"internal" }`, effective cardinality 0–1, permitted at all five EFS levels, registry maximum exposure `public` (§6.2G.4) — which remains a displayable public address fact and **still implies no contact submission endpoint**;
- the `mailto:` action grammar — one validated addr-spec recipient plus an optional governed `subject` of ≤160 plain-text scalars, no CR/LF injection, no secrets, **no auto-send and no claimed send result** (§6.2D.7) — and the classified `{ "kind": "email", … }` typed link (§6.2J.9);
- the `1.0.0` module registry of §6.2E.2, now **exactly twenty-two** IDs under this order's expressly Owner-approved `O05`/`O06` amendment: the original twenty-one keep their exact contracts unchanged, and `interaction.contact` (§6B.8) is the sole addition;
- exposure classification and validated public projection (§6.2G.3), the §5A security constraints and review protocol, and every `D01`–`D12`, `O01`–`O12` and `E01`–`E10` requirement with the `C1`–`C4` **UNVERIFIED** status.

### 6B.2 Two contact actions that are never conflated

**A public `mailto:` action and a contact-form submission are different use cases.** The `mailto:` destination is authored public content rendered in public markup: it must be intentionally public and author-approved, it opens the visitor's own mail client, and it **must not** be described as server delivery, as a secret, as a tracked submission or as a guaranteed receipt. A form submission is a **server-side** use case whose HTML `action` is a **server-owned, publicly routable submission URL** — necessarily visible in public markup, never a secret, and never an author-supplied custom action URL — accompanied only by non-secret CSRF and attempt control tokens. By contrast the **internal recipient address, delivery-provider identity and configuration, credentials, provider API or SMTP destination and transport diagnostics** are private and must never be serialized into public HTML, JS, JSON, public URL parameters, client-side error text or ordinary diagnostics.

**Configuration absence is a first-class state, not an error to hide.** Where an installation has no configured delivery adapter, or no resolvable recipient, the form is presented as clearly **disabled or unavailable** and is never presented as operational; it must not accept a message and silently discard it. The public `mailto:` action remains independently usable whenever `contact.email` is authored with `public` exposure, because it requires no server-side delivery at all.

### 6B.3 The single simple form — closed request contract

**Exactly one bounded form exists.** It collects exactly three fields — `name`, `email`, `message` — and no other personally identifying field is collected by default. A privacy-notice acknowledgement is **not** part of this settled contract: it may be added only if the Owner defines its legal and operational basis and effect, and consent is never a preselected or pre-ticked checkbox and never a blanket legal-compliance claim. This is a **settled** first-release decision: the module carries a privacy-notice **link** (`notice`) and no acknowledgement control, and a future legally required change is a separately approved amendment.

All three values are **inert untrusted data**. `name` and `message` are never interpreted as markup, never interpolated into a template, never evaluated and never used to construct a header; they are escaped on every output path.

| Key | Type | Required | Normalization and validation |
| --- | --- | --- | --- |
| `name` | plain string | yes | NFC normalize; trim leading and trailing whitespace; 1–100 Unicode scalar values; no control characters; markup characters stay literal data |
| `email` | plain string | yes | trim; exactly one addr-spec under the §6.2J.9 mailbox grammar; ≤254 octets; no display name, comment form, comma list or header injection |
| `message` | plain string | yes | NFC normalize; 1–4,000 Unicode scalar values; control characters other than ordinary whitespace refuse; opaque text, never markup |

**Ratified first-release bounds.** These are Owner-approved first-release values, not interim guesses. `name` allows **1–100 Unicode scalars after NFC and edge trim** and rejects every control character and line break. `email` is exactly one §6.2J.9 addr-spec of **≤254 UTF-8 octets** after trim, and its local part is never lowercased or rewritten. `message` allows **1–4,000 Unicode scalars after NFC with CRLF normalized to LF before counting**, permits ordinary LF and TAB, and rejects other ASCII controls, NUL, header injection and unsafe encoding. The raw HTTP request body is limited to **16 KiB before parsing**, and the decoded UTF-8 payload to **16 KiB after structural decoding**, so encoding expansion cannot evade either ceiling. Owner-controlled future tuning of these values requires its own accepted operational configuration boundary.

**The envelope is closed.** A non-object root, unknown or additional key, repeated key, non-UTF-8 or malformed encoding, wrong type, empty required value, over-limit value, oversized body or unparsable payload refuses the **whole** request. There is no partial acceptance, silent coercion, lossy truncation, field dropping or guess at intent (§1.3, §5A). Server-side validation is mandatory and independent of every client hint: `required`, `maxlength`, `type` and pattern attributes are presentation conveniences only and never authority.

**Decoding and mandatory first-release transport.** The required browser-facing submission is a standard HTML `method="post"` form encoded as **`application/x-www-form-urlencoded` UTF-8**, so the form is fully operable with JavaScript disabled. That media type is accepted with an absent charset parameter or `charset=UTF-8`, the charset label being matched case-insensitively; every other media type — `multipart/form-data`, `application/json`, any other encoding or an unsupported charset — is **refused in the first release**, and no unbounded or silently added parser is accepted. Decoding is strict: a repeated key, an unknown key, a missing or malformed control value, an ambiguous percent escape, invalid UTF-8 or a body breaching either **16 KiB** ceiling refuses the whole request. The earlier conditional JSON reference was decoding guidance only — it never approved a second endpoint, a general JSON inquiry API or a fallback, and any later JSON support requires explicit separate Owner authority.

**One closed five-key transport envelope.** A submission carries exactly three visitor data keys — `name`, `email` and `message` — and exactly two server-owned, non-personal control keys, `csrfToken` and `attemptToken`. Native form encoding places all five in one POST body, so decoding **separates the two controls out before** constructing the closed three-value `ValidatedInquiry`. Both controls are mandatory and must be well formed; they are never authored module props, never additional visitor fields, never counted as visitor data fields, never persisted as personal data and never rendered as personal data. The 16 KiB raw and 16 KiB decoded bounds apply to the **complete wire submission, including the control keys**. No honeypot field, tracker, JSON fallback, attachment, alternative custom action or additional API is introduced.

### 6B.4 Application use case and outbound delivery port

**Typed inbound boundary.** The request reaches the application as an already-decoded typed value — never a framework request object, a raw byte stream, a form map or an OS type. The accepted inbound port name is `SubmitSimpleContact(ValidatedInquiry, AbuseContext) -> truthful delivery outcome`, reserved in §6A.4. This edition permits an equivalent name, but the accepted §6A.4 names are **retained** so that §6A.4 and §6A.12 remain coherent.

`ValidatedInquiry` carries exactly the normalized `name`, `email` and `message`, the resolved non-public `recipientId`, and a safe correlation handle. It never carries a credential, a provider payload, a client-supplied origin claim or the raw request body. `AbuseContext` carries only the minimum signals the §6B.7 controls require and is never persisted as a visitor profile.

**Outbound delivery port.** The reserved `ContactDelivery` port (§6A.4) accepts a validated message plus a `recipientId` **resolved from private installation configuration**, and returns **only the four transport outcomes** `accepted_for_delivery`, `rejected_downstream`, `temporarily_unavailable` and `indeterminate` of §6B.5. The port exposes no provider SDK type, transport handle, credential or vendor error string to the application, and adapter failures are translated into the fixed enumeration.

**Everything external stays in adapters and the composition root.** Provider SDKs, SMTP or API transport, outbound HTTP, credential and secret access, framework objects, persistence and environment access live only in adapters wired at the single `solo-cli` composition root (§6A.4). The domain core and application use cases import none of them and dependencies point inward only (§5, §5A). A `ContactDelivery` adapter holds least privilege for outbound sending and nothing more.

**No location or address inference.** No path, hostname, forwarded header, IP geolocation, locale, route, EFS node, Page identity or authored `location` fact may select, derive or grant authority over a recipient, provider, rate policy or outcome. The recipient comes only from validated private configuration, and any Site/Page context is passed as an approved typed reference, never re-inferred from the request (§2.1, §2.2).

**Request-boundary policy.** Origin and CSRF handling must be defined against an explicit trusted-proxy boundary; client-supplied `Origin`, `Referer`, `X-Forwarded-*` or host values are untrusted unless a configured proxy boundary establishes otherwise (§5A). Response shapes are enumerated from §6B.5 only, uniform in timing and wording where a difference would help an attacker, and never echo provider diagnostics. Replay handling is required wherever a retry could duplicate a message (§6B.5).

**Attempt token, expiry and deduplication.** A **server-issued opaque attempt token** travels as an ordinary form control and is bound to validated server-side state. It expires **no later than 24 hours after issue** and can never become valid again. The same token may authorize **at most one outbound dispatch**, even under concurrent, duplicate or multi-instance requests. A token that is missing, expired, invalid or unverifiable is refused **without any outbound dispatch**. Deduplication keeps only a short-lived **non-content record** — token hash, state, timestamps and a non-reversible request digest — effective for the token's full validity and expiring at that same defined 24-hour boundary; that record is operational adapter state, not contact-message retention, and it never stores a message body or an address. After expiry, or after loss of that deduplication state, the same token must **not** create fresh dispatch authority. Such a refusal maps to the existing generic `invalid_request` form-level outcome of §6B.5, with no new visitor outcome and no provider detail. A genuinely fresh attempt requires a deliberate new visitor action, a newly issued token and the existing duplicate-warning policy. Where durable, atomic cross-instance deduplication is unavailable, the form is presented unavailable rather than making an unverifiable exactly-once promise. This governs at-most-once dispatch only and never promises downstream exactly-once inbox delivery.

### 6B.5 Truthful outcomes — the fixed enumeration

Exactly seven application outcomes exist. They are a **closed** set: no eighth outcome, no provider-specific code and no free-text provider reason reaches a visitor.

**Who may produce which outcome.** The **inbound/application boundary** produces `invalid_request`, `not_configured` and `rate_limited`, and never `ContactDelivery`. The **`ContactDelivery` port** produces only the four transport outcomes `accepted_for_delivery`, `rejected_downstream`, `temporarily_unavailable` and `indeterminate`. The application maps those four into the visitor model, so a transport adapter can never invent a validation or rate-limit verdict. An invalid CSRF or origin check maps to `invalid_request` with a **generic form-level security refusal**, never a fictitious field error that pretends a particular field was wrong.

| Outcome | Meaning | Visitor-facing state |
| --- | --- | --- |
| `invalid_request` | the envelope failed §6B.3 validation, or a CSRF or origin check refused the request | field-specific accessible errors naming the offending field **where it is safe to do so**, never echoing the submitted value; a CSRF or origin refusal instead shows the generic form-level security refusal and reveals no sensitive detail |
| `not_configured` | no delivery adapter, or no resolvable recipient (§6B.2) | form presented unavailable or disabled; not a fault of the visitor |
| `rate_limited` | a §6B.7 abuse control refused the request | bounded accessible refusal with safe retry guidance |
| `accepted_for_delivery` | a configured adapter accepted the message for onward delivery | acknowledgement meaning **accepted for delivery only** |
| `rejected_downstream` | the provider or downstream transport refused it | accessible failure with no provider detail |
| `temporarily_unavailable` | adapter or dependency outage where retry may succeed | accessible retry state |
| `indeterminate` | the result cannot be established | accessible "not confirmed" state that never claims delivery |

An acknowledgement **may only** mean accepted for delivery: it never asserts that a mailbox received, opened or read the message. `accepted_for_delivery` must not be produced by a fake, log-only or no-provider adapter — a demonstration adapter reports `not_configured` or `indeterminate`, never live delivery. `rejected_downstream` and `temporarily_unavailable` must remain distinguishable to the application even where the visitor-facing wording is deliberately uniform.

**Accessible states and safe retry.** Every outcome has a programmatically determinable state, a visible message and an assistive-technology announcement that does not depend on colour alone or on focus theft; error association follows normal form-error conventions, and submitted values are preserved only where preserving them is safe. Retry is offered only where it cannot duplicate a message: after `indeterminate` or `rejected_downstream` the client must not blindly resubmit, and no automated resend is ever performed. A repeated attempt carrying the same attempt token returns its last safely known result **without dispatching again**; only a fresh, deliberately triggered visitor action with a newly issued token begins a new attempt, and it warns about possible duplicates (§6B.4). No wording here may promise downstream exactly-once inbox delivery.

### 6B.6 Privacy, logging and retention decisions

**Secrets and recipient mapping are never authored content.** Delivery credentials, the internal recipient mapping, provider configuration and **internal provider or delivery endpoint detail** stay outside authored and public content, outside the public projection and outside source control (§5A). The browser-visible server-owned form action URL is not such detail and is not a secret (§6B.2).

**Minimum transient processing — no default retention in core.** A submission is processed to validate it and hand it to the delivery port. Solo's core **does not persist** the visitor's name, email, message or request body. Operational limiter and deduplication keys expire with their configured windows — at most **24 hours** for the default attempt record — and are never contact-message retention. A chosen downstream provider may apply its own retention policy, which must be documented when a provider is actually adopted; no provider is selected here (§6B.12). No analytics or form telemetry may silently capture the submitted message, the visitor address or the message body.

**Log restraint.** Ordinary logs and diagnostics must not contain the raw message body, the visitor's email address or any credential; where correlation is needed a generated handle is logged instead. Diagnostics name the failing stage and the safe field or outcome, never a provider response body, a secret, a server-private root or untrusted submitted content (§6.2D.8, §5A).

**Failure handling, provider adoption and data-subject obligations.** A delivery failure must not leave a partial or orphaned record. Exactly **one private recipient per Installation** applies in the first release, resolved only from private configuration. Provider retention, the applicable privacy and legal notice, and deletion terms are **separate mandatory provider-adoption prerequisites** that must be settled before a real delivery provider is activated — not promises made here. This section asserts no jurisdiction's legal compliance and claims none.

### 6B.7 Fail-closed boundary and proportionate abuse control

**Fail closed at the request boundary.** Malformed, ambiguous, unauthorized or oversized input is refused rather than coerced into a plausible result (§1.3, §5A). Server-side validation is authoritative regardless of client hints; the body is size-constrained before parsing; and no submitted content is ever executed, interpreted as markup, used to construct a header or written to a filesystem path.

**Proportionate, first-party controls.** Rate and abuse controls are bounded, first-party and proportionate. This contract does **not** mandate a third-party CAPTCHA, visitor tracking, a visitor account or a persistent global identity system, and must not introduce one by default. Automated and repeated submissions are handled by a defined refusal path, never by silent acceptance.

**Explicit refusal criteria.** A request refuses when the envelope fails §6B.3; when the decoded body exceeds the payload ceiling; when a required field is absent or over-limit; when submissions for the applicable bounded key exceed the configured rate; when the delivery adapter is unavailable and the request cannot be safely queued; or when origin/CSRF validation fails under the defined trusted-proxy boundary. Each criterion becomes a code-bearing verification case in §6B.10.

**Ratified first-release rate policy.** Server-side first-party rate control applies **before delivery**, counting accepted and refused form attempts once they reach the bounded request entrypoint: a default of **3 attempted submissions per verified client network peer in a rolling 15-minute interval**, and **60 attempted submissions per Installation in a rolling one-hour interval**. Exceeding either yields `rate_limited` with no outbound action, and authored Page, locale, route or address headers can never raise an allowance. A client network peer used as the abuse key comes only from the actual trusted transport and proxy configuration — never from raw `X-Forwarded-*`, `Host`, `Origin` or geolocation as authority. Where a deployment cannot identify a trustworthy peer, or cannot enforce these bounds consistently across active instances, the module stays unavailable (`temporarily_unavailable` or `not_configured`) rather than running a silently weakened limiter, a tracking identifier, a third-party CAPTCHA or an account system. These are first-release defaults, not worldwide security guarantees.

### 6B.8 The `interaction.contact` module and its authoring boundary

**Entrypoint and sole enablement mechanism.** The simple form **is** the registered `interaction.contact` module of §6.2E.2, explicitly authored as a node in the representation's selected JSON body under the existing `blocks` and closed registered-node grammar. It is the **sole** form Page-enablement mechanism: there is no Page field, manifest field, route field, parallel system-owned Page overlay, Markdown directive or automatic contact inference. A Markdown Page may expose a safe `mailto:` action but gains no form token, and at most **one** `interaction.contact` node is permitted per representation.

**Owner decision, resolved.** The Owner approved a genuine, explicitly authored module rather than a concealed system overlay, amending the `O05`/`O06` first-profile catalogue to **exactly twenty-two** IDs. This is an explicit **pre-implementation Owner amendment** of the accepted first profile — not a `1.0.0` release migration and not a concealed optional. The original twenty-one contracts are unchanged, and no schema, registry binary, application or module implementation ships here.

**Module props are the only authoring surface.** `labels` (four required localized inert strings `name`, `email`, `message`, `submit`, each 1–80 scalars after NFC), `notice` (a `label` of 1–160 scalars plus an existing classified typed link resolving to a **public** Page representation or a validated `https:` privacy-information URL; unlisted, private, ambiguous, unsafe or non-`https:` external targets refuse) and `feedback` (the seven required outcome strings, each 1–240 scalars) are exactly as registered in §6.2E.2. All values are inert, escaped, never HTML or JS. The notice is an **informational privacy notice**, never a consent assertion and never a checkbox. Authored feedback may not claim inbox receipt, send completion, guaranteed delivery, tracking or legal compliance: publishing and translation review must refuse an objectively contradicted outcome claim where it is detectable, while the normative meanings of the seven outcomes are enforced independently of the copy by §6B.5. The generic `presentation` envelope applies exactly as for any leaf module, and `span` only where the existing grid/bento rule permits it. Unknown props and additional fields refuse the whole body. The two server-owned control keys of §6B.3 — `csrfToken` and `attemptToken` — are wire transport controls, never props: their presence in a native form POST neither extends this closed prop set nor counts as an authored or visitor field.

**Still excluded.** No general form builder, conditional or multistep workflow, form administration surface, attachment, custom action URL, user-provided endpoint, recipient, address, credential, provider or CRM exists under this registration (§3.4).



### 6B.9 Public links, bounded integrations and analytics

**Safe links preserved.** Authored `mailto:`, internal Page and fragment links, and external `http://`/`https://` links remain governed by §3.1C, §6.2D.5–§6.2D.7 and §6.2J.9. The typed `email` variant keeps its single addr-spec plus optional ≤160-scalar `subject`, and no typed link gains a new field here.

**Bounded external actions.** The first release supports **static external booking links and maps/directions links** to author-approved destinations. It supports **no** booking engine, availability or inventory feed, reservation transaction, visitor account, live geolocation service, or server-side fetch of a third-party destination (§3.1C, §6.2J.9). A map or directions link is an ordinary classified external link: it conveys no location authority and never infers a recipient, route or canonical.

**Analytics: off — a settled decision.** No analytics is required or enabled in the first release, and this is not an open blocker. Any analytics provider requires a separate Owner order, must be independently disable-able, can never collect form content, and can never become a publication prerequisite.

### 6B.10 Future executable proof matrix — `F01`–`F17` (all unrun)

Every case below is a **non-waivable future gate**. None has been executed: this repository contains no application code, schema artefact, test suite, provider or deployment, so no row is `PASS`.

| Case | Required proof | Status |
| --- | --- | --- |
| `F01` | the closed five-key wire envelope accepts a valid native-urlencoded inquiry — exactly `name`, `email`, `message`, `csrfToken` and `attemptToken` — and refuses unknown, additional, duplicate and repeated keys (including a repeated or unknown control key), missing or malformed control values, wrong types, empty required values, invalid UTF-8, ambiguous percent escapes, malformed bodies and oversized bodies with no partial acceptance or truncation | `NOT RUN` |
| `F02` | `name`/`email`/`message` normalization behaves exactly as §6B.3 states: NFC, edge trim, CRLF→LF before counting, control-character and header-injection refusal, the ratified 100 / 254-octet / 4,000 limits, and no lowercasing or rewriting of the email local part | `NOT RUN` |
| `F03` | the one obligatory media type is enforced: `application/x-www-form-urlencoded` is accepted with an absent charset parameter or `charset=UTF-8` matched case-insensitively, while `multipart/form-data`, `application/json`, other encodings, unsupported charsets and an unbounded parser are refused; both ceilings hold — **16 KiB raw before parsing and 16 KiB decoded after structural decoding**, applied to the complete wire submission **including the control keys** — so an encoding-expansion body cannot evade the limit | `NOT RUN` |
| `F04` | `interaction.contact` is registered once at `1.0.0` and publishes with minimal valid props; an unknown prop, an extra field, a forbidden child or a wrong contract version refuses the whole body | `NOT RUN` |
| `F05` | **two** `interaction.contact` nodes in one representation refuse publication, and no Page, manifest, route or Markdown token enables the form by any other mechanism | `NOT RUN` |
| `F06` | localized `labels`, `notice.label` and `feedback` values are NFC-validated within 1–80 / 1–160 / 1–240, escaped on every output path, and never interpreted as HTML or JS | `NOT RUN` |
| `F07` | a `notice` link to a public Page representation or a validated `https:` privacy-information URL is accepted, while unlisted, private, ambiguous, unsafe and non-`https:` external targets refuse | `NOT RUN` |
| `F08` | outcome ownership is enforced: the inbound/application boundary emits only `invalid_request`, `not_configured` and `rate_limited`; `ContactDelivery` emits only the four transport outcomes; invalid CSRF/origin maps to a **generic form-level** `invalid_request`; no provider text reaches the visitor | `NOT RUN` |
| `F09` | `accepted_for_delivery` is reachable only from a real adapter with externally evidenced provider acceptance, and is never produced by a fake, log-only or no-provider path; all seven outcomes remain distinguishable to the application | `NOT RUN` |
| `F10` | private recipient isolation: the resolved `recipientId`, internal address, credentials and provider configuration never appear in public HTML, JSON, metadata, client errors or ordinary logs | `NOT RUN` |
| `F11` | with no adapter or no resolvable recipient the form presents `not_configured`/unavailable and never accepts-and-discards a message, while the separate public `mailto:` action stays usable | `NOT RUN` |
| `F12` | the ratified rate policy engages before delivery at 3 attempts per trusted client peer per rolling 15 minutes and 60 per Installation per rolling hour, and forged `Origin`, `Referer`, `X-Forwarded-*` or `Host` claims can neither raise an allowance nor establish the peer | `NOT RUN` |
| `F13` | token replay and expiry: one attempt token authorizes **at most one** dispatch under concurrent, duplicate and multi-instance requests; a missing, expired, invalid or unverifiable token is refused **without** dispatch and maps to the generic form-level `invalid_request`; the token expires no later than 24 hours after issue, can never become valid again, and creates no fresh dispatch authority after expiry or loss of deduplication state; where atomic cross-instance deduplication is unavailable the form is unavailable; after `indeterminate` no blind or automated resend occurs and a fresh attempt needs a newly issued token | `NOT RUN` |
| `F14` | storage and retention: the core persists no name, email, message or request body, and limiter/deduplication records hold no content and expire within 24 hours | `NOT RUN` |
| `F15` | static author-approved booking and maps/directions links behave as classified external links, and no analytics request is made by default | `NOT RUN` |
| `F16` | accessible semantics and no-JavaScript transport: labels programmatically associated, errors associated to their field, assistive-technology announcements for every outcome, full keyboard operability, and a **real native POST submission with JavaScript disabled** that reaches the server and renders each truthful outcome without any client script | `NOT RUN` |
| `F17` | inward-only dependency proof: no provider SDK, transport, framework, storage or environment import in the domain core or application use cases | `NOT RUN` |

### 6B.11 Decided first-release policy, and what is genuinely later-scope

Every first-release `R00.F` decision that affects the accepted `interaction.contact` schema, the visitor-visible outcomes, the required privacy link, the transport boundary, the field limits, the rate policy or the retry proof is **decided** in §6B.3–§6B.9. No provisional or unresolved first-release item remains.

| Decided in this release | Value |
| --- | --- |
| Module placement | registered `interaction.contact` in a twenty-two-ID catalogue (§6B.8) |
| Collected fields | `name`, `email`, `message` only; no extra data field (§6B.3) |
| Field and body limits | 100 scalars / 254 octets / 4,000 scalars; 16 KiB raw **and** 16 KiB decoded (§6B.3) |
| Rate policy | 3 per client peer per 15 min; 60 per Installation per hour (§6B.7) |
| Wire transport | one obligatory native `application/x-www-form-urlencoded` UTF-8 POST; five closed keys — three visitor fields plus `csrfToken` and `attemptToken` (§6B.3) |
| Deduplication | server-issued attempt token, at most one dispatch, 24-hour non-content record; the token expires within 24 h and never regains dispatch authority (§6B.4) |
| Consent | no checkbox or acknowledgement; privacy-notice link only (§6B.8) |
| Core retention | none (§6B.6) |
| Delivery recipient | one private recipient per Installation (§6B.6) |
| Analytics | off (§6B.9) |

**Genuinely later-scope — not first-release blockers.** Delivery provider selection and activation; provider retention, privacy and legal notice, and deletion terms as **provider-adoption prerequisites**; hosting, TLS and secret storage; and any future numeric retuning through an accepted operational configuration boundary. None of these is an unresolved first-release contract decision, and none may be used to reopen §6B.3–§6B.9.

### 6B.12 Preservation, exclusions and what remains reserved

**Preserved exactly.** One Entity per Installation; the Entity → Facet → Site → Page → localized-representation hierarchy; stable Page identity independent of locale, route and location; full region-qualified locale tags; the §3.2B route-case, Unicode and collision contract; unlisted visibility that is never access control; authored location independent of routing; exposure-classified public projection; and the §5A integrity, secrecy and refusal constraints. `D01`–`D12`, `O01`–`O12`, `E01`–`E10` and `C1`–`C4` retain their status unchanged.

**Excluded.** Advanced contact forms, form builders, conditional or multistep workflows, form administration, file attachments, CRM and ticketing pipelines, visitor accounts, a persistent global identity system, booking engines, live inventory, transactions, geolocation services, server-side third-party fetching, mandatory CAPTCHA and default analytics (§3.4, Product §6).

**Reserved to later packages.** Provider selection and activation, transport, hosting, DNS, TLS, secret storage, operational monitoring and backup are **`R00.G` deployment and lifecycle matters implemented in Stage 10** — never `R00.F` obligations; numeric accessibility and performance targets, release and support policy and third-party notices belong to **`R00.G`** and are judged in **Stage 11**; the contact form, `mailto:` action and delivery adapter are implemented in **Stage 08 (`R08.A`–`R08.C`)**; static booking and directions links with analytics-off are implemented in **Stage 09 (`R09.A`–`R09.C`)**; and every schema artefact, adapter, endpoint, test and CI job belongs to `R00.E` and the later code-bearing stages. `R00.F` delivers documentary decisions only and retains no implementation or deployment obligation.

## 7. Decisions to make before an implementation work order

1. **Authoring and publication contract — decided by `R00.A`, physical literals now settled:** accepted and recorded in §1.1, §1.3 and §1.4 are the technical ID grammar and 63-character limit, the parent-scoped uniqueness rules, durable references, the explicitly supplied Entity ID, the seeded `main`/`main`/`home` defaults, drafts outside the published graph, the single versioned manifest as sole publication authority, source integrity and fail-closed refusal. The manifest file and field/locator schema, the editable source layout, the JSON and Markdown authoring encoding, draft isolation and the encoding of the accepted identity semantics are **settled** in §6.2A–§6.2C and the worked examples in §6.2M; only `R00.E` implementation of the stated artifacts remains.
2. **Address, route and locale contract — decided by `R00.B`, physical literals now settled:** recorded in §3.2, §3.2B and §4.1–§4.4 are one configured canonical HTTP(S) origin with optional prefix and optional authorized aliases, explicit Page and optional localized routes, validated Unicode route text, default-insensitive public matching with author-selected case-sensitive claims, exact-case unlisted matching, independent full-locale normalization, the ten publication collision rules, `308`/`302` redirect semantics, cache safety and locale-less entry/fallback behaviour. The declaration keys, the pinned **Unicode 18.0.0** profile and the collision-ledger rules are settled in §6.2J. **Still technical:** the toolchain that binds that Unicode data (`R00.E`) and the implemented matcher, HTTP adapter, redirect/cache behaviour and rendered language selector (later stages).
3. **Typed property, location and graphic contract — decided by `R00.C`, physical literals now settled:** accepted and recorded in §2, §2.1 and §3.3A are the governed extensible property registry, the independent `inheritance`/`currentContext` declaration elements with direct-ancestry-only resolution and effective-cardinality validation, per-value provenance and exposure classification with a validated restricted public projection, zero-to-many complete structured location facts with explicit identified-record extension and no geographic inference, registered graphic roles, typed Entity versus generic-platform asset references, default-deny role fallback and fail-closed publication. The nine-key property catalogue, `add`/`extend` envelopes, role cardinalities and media allowlists are settled in §6.2G–§6.2I. **Still technical:** the platform asset library's artwork, licensing and version-pinned catalogue delivery, plus every implemented resolver, validator, selector and adapter in later stages. No automatic graphic fallback is approved for any role initially.
4. **Page formats, visibility and presentation:** the advanced modular JSON capability families (§3.1B) are instantiated as the twenty-two closed `1.0.0` contracts of §6.2E.2, together with the accepted JSON-first single-effective-source precedence (§1.6), Owner-approved Page-level `visibilityDefault` with representation-level `visibility` override (§6.2B, §6.2J.5), the design tokens of §6.2E.1, the shared read-only generic public library (§6.2I.2) and cascading graphic roles. Field placement, schema encoding and the first module and role catalogues are now decided; only rendering implementation and per-module accessibility/performance testing remain (later stages). Unlisted serving and discovery behaviour is Owner-accepted and restated physically in §6.2J.10.
5. **Integration scope — decided by `R00.F`, documentary contract settled:** the **included** contact scope is the safe `mailto:` action and the **simple** contact form, the latter registered as the twenty-second `1.0.0` module `interaction.contact`; **advanced contact forms are excluded** (see §3.4). The bounded contract is settled in §6B: two distinct contact actions that are never conflated; the closed three-field envelope with ratified limits and 16 KiB raw **and** decoded ceilings; the `SubmitSimpleContact` use case over the reserved `ContactDelivery` port with a privately resolved recipient, and the four transport outcomes held separate from the three boundary outcomes; the seven truthful outcomes; the ratified rate policy and single-dispatch deduplication; log restraint with no core retention and no consent checkbox; static external booking and directions links; and analytics off. **Genuinely later-scope** (§6B.11): provider selection and activation, provider retention and legal terms as provider-adoption prerequisites, and hosting, DNS, TLS and secret storage. None of these may alter §6.2's literals.
6. **Tooling/lifecycle:** release/upgrade strategy — the framework, runtime, package layout, validation/test gates and CI platform are **selected conditionally in §6A and still owe unexecuted conformance proof**, so no build or test claim may be inferred from their selection — plus **Apache-2.0 (Owner approved), copyright/attribution, third-party notices, and the release/support policy around the already-enabled GitHub private vulnerability reporting channel** — intake is enabled for the public repository, while triage ownership, response expectations, coordinated disclosure, a supported-version/support policy and any required third-party notice remain open — and independent Solo repo setup (the parent `/00.foundation-solo/` ignore rule is **committed and verified effective** in the enclosing repository at its recorded local revision, which tracks no Solo paths; the parent remains outside the Solo agent's write boundary, and its remote publication is a separate Owner/parent-operator concern, not a Solo bootstrap gate). These are `R00.E`–`R00.G` obligations, tabulated in §6.5.

**No undefined implementation mechanism should be treated as approved merely because it appears as an example in this document.** The examples in §6.2M are normative at the level of fields and types only, never as evidence that files, schemas or code exist.

## 8. Governance, repository and evidence boundary
**New document naming invariant:** the only live WIP files are `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, and `plan-outlook.md`. Files beginning `plan-` can inform decisions but never authorize source work; `workorder-active.md` only authorizes changes with an exact Owner-issued scope and matching handoff. Do not generate task-specific plan or workorder files to accumulate history.

**Repository standard files:** The Owner-approved Apache-2.0 `LICENSE`, Git hygiene dotfiles, and recognized `.github/` community files are narrow publishing/tooling placement exceptions to `.documents/`. They contain no second source of architectural or operational authority. GitHub **private vulnerability reporting is enabled** for this public repository; copyright/attribution, third-party notices, triage ownership, response expectations, coordinated disclosure and a supported-version/support policy remain outstanding and must be settled before any release.


The Solo repository and its own source are the future first-hand implementation evidence. All standalone Solo documentation belongs under `00.foundation-solo/.documents/`, except the required root `README.md` and `AGENTS.md`. Decisions and approved execution tasks must be recorded in that same project, not delegated to historical Foundation or Provelopment root documentation.

The initial files are intentionally self-contained. Solo operating policy resides in [`SOP/sop-architect.md`](../SOP/sop-architect.md) and [`SOP/sop-implementation.md`](../SOP/sop-implementation.md); the only authorized live work surface is [`WIP/workorder-active.md`](../WIP/workorder-active.md), with the remaining committed stages in [`plan-roadmap.md`](../WIP/plan-roadmap.md), at most one candidate in [`plan-next.md`](../WIP/plan-next.md) and optional ideas only in [`plan-outlook.md`](../WIP/plan-outlook.md). `WIP/` contains **exactly** these four files. No separate `Work-Orders` directory and no archive of reports may accumulate. The single canonical execution report sits deliberately **outside** WIP at [`Reports/execution-report.md`](../Reports/execution-report.md): it is overwritten on every run, including failed and read-only runs, is the only permitted current execution-report file, and is neither a WIP file nor a source of authority. That one report and the four WIP files are disjoint sets.

`provelopment-root`, `provelopment-foundation` and `provelopment-web-1` may be consulted for background, but they are **not normative dependencies** of Foundation Solo and must not be edited as part of Solo implementation without an explicit separate authorization.
