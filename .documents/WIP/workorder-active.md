# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-011`  
**Edition:** `R01`  
**Title:** R00.C — Canonical codification of Owner-accepted property/location/graphic contract  
**Approval marker:** `OWNER-ISSUED`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Target branch:** `main`  
**Required starting `origin/main` HEAD:** `f787203859d419081065cb7cb9175d131a0d6bd0`  
**Matching handoff:** `SOLO-WO-011 R01 — Implementation Agent Handoff`  
**Classification:** Documentation-only architectural codification  
**Quality standard:** A1 documentary consistency, exact scope confinement and GitHub-verifiable evidence  
**Independent review:** Chief Architect `ACCEPT / REVISE / STOP`  
**Closure and NEXT transition:** Owner, through a separate subsequent order

## Frozen Approved Scope — Sections 1–9

The following is the complete, single-task Approved Scope. The Implementation Agent must verify the exact `OWNER-ISSUED` file installed by the Owner in this canonical ACTIVE slot **and** its separately delivered matching direct Owner handoff before edits. The Agent must then **execute the authorized work**; routine Git mechanics do not require a separate Owner request. The scope is frozen at issuance. Do not rely on another attachment or companion file for task authority or required decision detail; everything required to codify R00.C is below.

## 1. Objective, authority and deliverable

Codify **all ten accepted C01–C10 R00.C logical decisions** in the **existing** Foundation Solo Architecture Charter and Product Definition. The current canonical files contain nearest-explicit-declaration wording that must be reconciled with the accepted additive `inheritance` / `currentContext` model and its location-specific identified-record extension. This is documentation codification, **not product implementation, schema selection, application development, or administrative closure of R00.C**.

The Owner has accepted: C01 governed extensible property registration; C02 absence and explicit state separation; C03 the Owner-designed two-field additive/blocked inheritance contract; C04 per-value provenance; C05 per-value public/internal projection; C06 zero-to-many structured location facts with explicit identified-record extension; C07 extensible role catalogue; C08 typed Entity and platform generic asset references; C09 default-deny role fallback; C10 incomplete drafts outside strictly valid published graph. The normative details and refusal cases are **integral to this Approved Scope** in §§4–6; they are not delegated to another document.

**Keep separate:** (a) EFS identity/ownership, (b) functionality and property/graphic semantics, and (c) routing/locale/public address. Location and graphics never create another EFS owner node or choose a route. All accepted R00.A and B01–B12 remain binding.

## 2. Exact permanent-path and permission ledger

| Path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs this exact Approved Scope. Implementation Agent publishes the **installed bytes unchanged** in a separate Phase A order-only commit; leaves them unchanged for review. |
| `.documents/Architecture/ARCHITECTURE.md` | Phase B: narrowly update §2, §2.1, §3.3A, short §3.4 cross-reference and §7 open/accepted status; protect §2.2 and R00.A/R00.B content. |
| `.documents/About/FOUNDATION-SOLO.md` | Phase B: narrowly update §4, its §4B graphics subsection, and only obsolete R00.C statements in §5 and §8. |
| `.documents/Reports/execution-report.md` | Phase C: overwrite the **single** canonical report, then publish as one separate report-only commit. |
| `.git/**` | Ordinary metadata for the three authorized commits and fast-forward pushes under Architect SOP §6B. |

**All other permanent paths are read-only**, including `.documents/WIP/plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, root `README.md` and `AGENTS.md`, both SOPs, `.github/**`, license, tooling, schemas, source and all other repositories. No new document, WIP slot, report/archive, checkout/worktree, nested clone, or shadow source tree.

**Authorization ledger:** ordinary commits and fast-forward pushes of exactly the three phases to verified Solo `origin/main` are **authorized**. Authenticated GitHub readback is **authorized**. Creating branches/PRs, merging, dependency installs/updates, code/test-tool changes, external provider calls, secrets access, paid services, release, deployment, DNS/TLS edits, destructive Git commands and history rewrite/force-push are **not authorized**. No other remote may be modified. SOP §6B is the single Git procedure; this scope does not replace it.

## 3. Baseline and preflight

- Confirm actual Solo checkout root, single worktree, Git toplevel, branch `main`, one expected Solo `origin`, `git status`, HEAD and remote-side `main` HEAD. The required starting HEAD before publication is exactly `f787203859d419081065cb7cb9175d131a0d6bd0`. Do not guess from an enclosing/parent repository or create a checkout.
- Verify baseline Git blobs: Architecture `5ebabc7e056c582035f357e28015f9dc8333d39d`; Product `95b8fd753dc90834811d5050c62a1c5c7a30ef2a`; NEXT `7996a1e568f076062a0030ca7bdc8b97c5674f9f`; ROADMAP `05feb2e30cada5cf013b08b1daf7011ed504eaa4`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; pre-install ACTIVE `ea5f20b736483c2e102456be1cea9104059fc81a`; current prior report `1d5874367e1c608f46162d3d28a73dcf7ffe5716`.
- The only allowed initial working-tree edit is the Owner's **exact installed ACTIVE** file. All other unexpected changes are STOP until separately adjudicated. Verify exact work-order ID/edition, `OWNER-ISSUED`, whole installed text, and matching direct Owner handoff.
- Read `README.md`, `AGENTS.md`, both SOPs, all four WIP slots, current report, Product and Architecture. Verify WIP exactly four files, one canonical report, correct one-checkout boundary and temporary areas. Follow temporary cleanup/reporting obligations in SOP §6A and §5A; do not delete unfamiliar or Owner-owned files.
- If installed content, handoff, baseline or scope conflicts, STOP *that unsafe action* and publish a truthful `STOPPED` report **only if safe** under the standing report-only exception. Do not treat a correctly matched issued order as tentative or ask Owner to perform routine Git operations.

## 4. Accepted logical R00.C contract — binding codification text


The following is the **Owner-accepted logical R00.C contract** to codify in the two existing canonical documents. It does not approve physical JSON/YAML encoding, filesystem layout, framework, provider or executable implementation. `inheritance` and `currentContext` describe accepted logical fields; exact schema representation remains R00.D.

### 1. Single governed property system

- The property registry supplies built-in and explicitly registered custom property definitions. Each definition has a stable property identity, type, allowed EFS levels, cardinality, validation, visibility ceiling, and permitted specialized operations. Unknown or conflicting definitions fail validation. Extensions cannot bypass the platform's mandatory safety, publication or ownership invariants.
- The cascade remains **Entity → Facet → Site → Page → localized representation** within one Entity; inheritance traverses only the selected node's direct parent chain, never siblings or a geographic/route-derived hierarchy.
- A wholly absent property declaration at a node implicitly inherits the effective ancestor result. An explicitly declared property has two independent logical elements: `inheritance: inherit | suppress` and `currentContext: none | zero-or-more typed local values`. `none` belongs **only** to current context, not inheritance. A declaration of inherit+none differs in provenance from total absence but yields the same values.
- An explicit declaration resolves as: effective = (the direct ancestor's effective values when inheritance is `inherit`; otherwise an empty inherited set) plus the current local contributions. At the Entity root, inherited input is empty. A narrower valid declaration can add local values after a higher-level suppression. No automatic cross-branch fallback.
- Collections have a deterministic **inherited-first, then locally-authored** order. Registry-defined maximum cardinality applies to the **effective** collection, not merely local contributions. Duplicate equality and uniqueness are defined per registered property; invalid ambiguity or uniqueness conflict fails validation, never silent first/last wins or silent deduplication. Ordered effective values retain their original declaration identity and origin. Do not arbitrarily perform recursive object merging.
- The domain-resolved result preserves per-value EFS origin, inclusion/suppression decisions and optional permitted record-field provenance. Private source locations, author-only metadata, and sensitive value contents must not leak through public diagnostics.
- Each value has an exposure classification within its registered type's maximum public exposure. Inheritance preserves the classification. Public renderers, metadata, generated feeds and client serialization must consume a validated **public projection**, never an unfiltered internal property result. An internal value is not emitted or replaced by a public ancestor simply because the projection omitted it. The platform does not treat `internal` as secrecy for material in a public Git repository or for delivery infrastructure configuration.
- A failed property, cross-property, security or publication check is a blocking error for any published graph; incomplete drafts can exist only outside it. No automatic correction, fallback, ignoring unknown fields, or publishing partial invalid state. Unlisted representations are still published, directly reachable, and subject to the complete validation gate; they are not secrets.

### 2. Structured location fact contract

- `location` is an ordinary registered EFS property whose **current context is a collection of zero or more complete structured location facts**. Every fact has enough authored information to be meaningful but no universally required country, city, street, postal code or coordinates. Supported optional concepts can include world region, administrative region/state/province, country, locality/city, district/local area, named place/area, postal components and explicit coordinates; define these as distinct semantic types, not one ambiguous universal `region` field. Governed custom typed location metadata is allowed under C01.
- Individual records may describe a geographic association or a physical premises; do not infer either from address detail. Never infer missing values from URL, route, EFS ID, locale, hostname, country, coordinates, neighbouring records or another language representation. No hidden geographic ownership level or automatic geocoder/verification service.
- The ordinary C03 rule carries **whole records** unchanged from ancestors, and local records are added as independent entries. A descendant wishing to use only its authored location records selects `inheritance: suppress`. Merely authoring `city: Perth` must not silently add `country: Australia` from an unrelated ancestor record.
- **Location-specific explicit extension exception:** A current-context location operation may identify **exactly one accessible inherited record** via a stable, unambiguous reference to its original authored identity, plus explicitly specified field additions/replacements. It replaces that specific inherited record **in place** in the effective ordered collection, rather than appending a duplicate. The original record identity remains the target reference; changed fields receive local field provenance and unchanged fields retain original origin. The reference must resolve through the selected direct ancestry only. Missing, ambiguous, suppressed, cross-branch or circular extension targets are invalid; if `inheritance: suppress` excludes the target, an extension of it is invalid. No automatic matching by geographic name/text/coordinates.
- The physical record-ID/ref format, allowed extension patch encoding and exact registry field catalogue are **R00.D schema choices**, but the resolution and refusal semantics above are R00.C normative contracts. A logical record reference must remain stable across routing and location-text changes.
- Whole-record exposure classification applies initially to each structured location value. Any internal record, including an inherited internal field in an explicit extension, remains absent from public projection. Do **not** silently make an internal field public by extending or relabelling the record; a newly authored, independently validated public fact or a future expressly accepted fine-grained projection contract is required. Do not assume per-field visibility from per-value C05.
- Authors take responsibility for factual geographic correctness. Solo validates declared types, structural integrity, privacy constraints and safe serialization, **not** real-world truth. A future optional external location-verification module needs a separate Owner decision and replaceable port; it may not determine public routes or silently rewrite authored facts.
- Do not produce a single implicit “primary” location from a multi-record collection. A downstream output requiring exactly one location must use an explicitly governed selection or refuse ambiguous input.

### 3. Graphic role, source and fallback contract

- Each registered graphic role has defined purpose, accepted asset classes, effective cardinality, applicable EFS levels and allowed use contexts. Initial **candidate** core roles: `logo`, `favicon`, `banner`, `background`, `socialPreview`. Their physical names, precise cardinality and formats are not already approved by listing these candidates. Role extensions require governed registration; arbitrary unregistered roles are invalid. Ordinary inline Page media need not be global cascading roles.
- All graphic-role declarations obey C03's inheritance/current-context collection semantics. For a singular role, `inherit` plus a local asset selection producing two effective entries **fails cardinality validation**; no silent last/first wins. `suppress` blocks inherited graphics but may accompany valid locally authored assets; it does not mean “must display nothing” regardless of local values.
- Graphic values refer to explicit, safe, typed source-asset references. Distinguish **Entity-authored source assets** owned within the sole Entity Installation from read-only **generic-platform public library assets**. These are logical source namespaces, not alternate EFS identity nodes. Do not infer ownership from a URL, filesystem path, route, locale or generated output. Missing/ambiguous/unsafe/unapproved sources fail validation; no arbitrary remote fetching. Physical syntax/paths/file types/processing controls remain to R00.D/R00.E.
- Explicitly choosing a generic-platform asset is a normal authored value, not fallback. Generic **automatic fallback** is separately governed **per role**, with zero roles approved for a fallback initially. Fallback is eligible only if there is no effective authored graphic and no inherited/local suppression boundary relevant to that resolution, no invalid declaration, and no private value omitted by public projection. Any invalid reference or cardinality/visibility violation is a refusal, never a fallback trigger. Do not infer a fallback from geography, locale, route, asset filename or provider.
- Source assets remain authoritative; transformed/optimized/public files are derived outputs. Public discovery, including unlisted-content handling, must not expose private asset metadata or unrelated unpublished assets.

### 4. Contact action/publication boundary (cross-reference only; R00.F owns its final contract)

- An intentionally public, author-approved email address may support safe `mailto:` rendering. A `mailto:` destination **is visible to the browser** and cannot truthfully be described as secret merely because the link is initially labelled “Email us” or revealed after interaction.
- An internal recipient address must never be serialized into the public property projection or embedded in public HTML/JS/JSON/URL parameters. A future basic contact form may show a public purpose/recipient alias while a server-side application use case resolves the actual delivery destination through a validated configuration/port. This is a **requirement to address in R00.F**, not authorization to implement or specify a new form system in R00.C.
- Internal-property classification does not hide material already present in a public Git repository. Delivery recipient configuration, email header disclosure, abuse controls, truthful receipt, privacy and retention remain explicit R00.F concerns. Do not introduce a contact/form builder, dynamic workflow, CRM or mail provider in R00.C.

## 5. Exact canonical-document amendment instructions


### A. `.documents/Architecture/ARCHITECTURE.md`

**§2, immediately after the EFS cascade diagram and existing property-resolution text:** Replace the legacy first-declaration-wins paragraph and its bullets governing `Absent key`, `Explicit suppression`, `Composite property`, `Provenance`, and `Public projection` with compact authoritative text matching §II.1 above. Keep the five-level ownership diagram and scope separation. Mark historical `null` suppression wording only as an unapproved physical-syntax example if it remains; do not present `null` as the chosen encoding.

**§2.1 location rule:** Keep the entire accepted no-inference/no-routing invariant and language-independence §2.2. Reconcile the nearest-location-example narration to show two-field, additive collection semantics: Entity Australia; Facet Indonesia via `inherit`+local gives [Australia, Indonesia], or via `suppress` gives [Indonesia]; Sites Bali/Jakarta using `suppress`+local produce only their own location. Explain how a location fact can explicitly extend a specifically referenced inherited fact as §II.2; examples remain conceptual and not physical schema. Avoid claiming an implicit geographic hierarchy or address completeness.

**§3.3A graphics:** Replace legacy one-nearest-asset/suppression-implies-no-asset algorithm with §II.3 semantics. Preserve one shared generic library, Entity ownership, read-only platform assets, safe references and publication controls. Add the candidate-role list clearly labelled candidates, not finalized schema/role contracts, and state that no initial automatic defaults are approved.

**§3.4 contact:** Retain the preexisting accepted simple `mailto:` + simple form (not advanced forms). Insert a small cross-reference that `mailto:` requires public recipient disclosure, while internal destinations must remain server-only and may be addressed by a server-resolved simple form subject to future R00.F approval. Do not preempt R00.F decisions.

**§7 open decisions:** Adjust only descriptions that incorrectly call the **logical R00.C contracts** open; keep the actual physical schema, formats, role-specific finalized catalogue, stack, and R00.D–G issues expressly open. Preserve accepted B01–B12 text, identity/publication and route sections unchanged.

### B. `.documents/About/FOUNDATION-SOLO.md`

**§4 location:** Replace or annotate the existing nearest-explicit-value-wins and overrides examples to reflect additive C03 semantics, optional structured zero-to-many location facts, explicit suppression, explicit identified record extension and public/internal projection; preserve the no-geographic-routing/location-identity rules word-for-word in meaning. Distinguish authored factual accuracy from structural validation.

**§4B graphics subsection:** Retain the shared platform generic library and five-level cascade; align the resolution and suppress/fallback wording with C03/C07–C09: suppress blocks inheritance, local values remain permitted, no automatic fallback without role-specific approval, invalid assets never get substituted.

**§5 and §8:** Where they describe the old nearest-wins cascade or the R00.C logical decisions as still proposed, update only those descriptions to accepted logical outcomes while retaining all implementation/capability disclaimers. Do not claim production assets, a form, a resolver, a published site or a selected framework exist.

### C. All other paths and contracts

Do **not** change README, AGENTS, either SOP, WIP roadmap/next/outlook, or the Owner-issued ACTIVE as part of the content-codification commit, license, `.github`, application code, tooling, or other repositories. The canonical execution report is the sole separate mandatory report-only exception, governed by the SOP.

## 6. Conceptual positive and refusal acceptance matrix

These scenarios are **documentary contract examples and gates**, not tests already run on application software. Verify both canonical documents express the required results and do not contradict any accepted R00.A or R00.B behavior.


| ID | Scenario | Required result |
| --- | --- | --- |
| P01 | Property absent at Site, Entity has two values | Both inherited; origin Entity preserved |
| P02 | `inherit` plus one local value | Inherited-first collection plus local; local origin preserved |
| P03 | `suppress` plus one local value | Ancestor contributions excluded, local value retained |
| P04 | `suppress` plus `none` | Zero values; suppression trace preserved |
| P05 | Inherit from sibling Site | Refuse/no inheritance; only direct ancestry is eligible |
| P06 | Effective cardinality exceeded | Block publication; do not choose first/last |
| P07 | Duplicate/invalid custom declaration | Fail against registered property contract |
| P08 | Inherited internal email plus public local email | Only permitted public value projected; internal still traceable internally |
| P09 | Public client serialization includes internal value/provenance | Refuse; fail public projection gate |
| L01 | Entity Australia, Facet Indonesia via inherit | Two separate records; no automatic country merge |
| L02 | Site Bali via suppress+local | Only Bali record; no inherited Australia/Indonesia |
| L03 | Site extends uniquely identified inherited Perth record | One updated record in same position; per-field provenance |
| L04 | Extension target missing/ambiguous or excluded by suppression | Validation failure, no nearest-name guess |
| L05 | Extension attempts to expose previously internal field | Refuse public exposure |
| L06 | Locale, URL or Site ID suggests a location | No inferred location or route selection |
| L07 | Zero locations or partial meaningful authored metadata | Valid; do not require a full postal address |
| L08 | Several eligible locations where one is required | No implicit primary; explicit selection or refusal |
| G01 | One Entity logo, Site inherits | Exact Entity selection inherited |
| G02 | Singular role inherited logo plus local logo | Effective cardinality failure, not silent override |
| G03 | Site suppresses inherited logo and selects local | Local logo effective |
| G04 | Suppress+none for banner | No generic default despite otherwise approved role fallback |
| G05 | No authored/suppression plus approved role default | Exactly approved generic source selected |
| G06 | No role default approved | No fallback |
| G07 | Invalid source/unknown role/private graphical value | Refuse/omit according to hard validation/privacy contract; no generic rescue |
| G08 | Shared platform asset selected by two Sites | Both valid within one Entity; no duplicate ownership node |
| V01 | Draft has missing required property | Draft may remain outside publication authority |
| V02 | Same invalid draft is published or made unlisted | Refused; unlisted remains published, not draft/private |
| C01 | Public `mailto:` selected | Address necessarily exposed to client when link is delivered |
| C02 | Internal email used as public `mailto:` | Refused; R00.F may define server-side simple-form recipient mapping |

## 7. Authorized publication sequence

**Phase A — Preserve and publish exact Owner issuance**

1. Validate Owner installation and handoff under §3.
2. Stage only `.documents/WIP/workorder-active.md`; make one ordinary order-preservation commit with its **exact installed bytes**; fast-forward push to verified `origin/main`.
3. Independently read back order bytes, commit changed-path ledger and remote HEAD. Do not start Phase B until these match.

**Phase B — Publish canonical Product and Architecture codification**

1. Update exactly the authorized sections of `.documents/Architecture/ARCHITECTURE.md` and `.documents/About/FOUNDATION-SOLO.md`, implementing §§4–6 **as normative architectural prose**. Preserve R00.A/B and all read-only paths.
2. Inspect full diff and cross-file consistency; search for stale nearest-wins/property/suppression assertions in the authorized affected sections and correct contradictions. Record the section-level positive and refusal matrix. No physical schema, runtime or independent new design is permitted.
3. Stage **only** those two documents, make one ordinary documentation-content commit, fast-forward push and verify GitHub readback of exact committed content and changed paths.

**Phase C — Publish mandatory canonical report**

1. Overwrite only `.documents/Reports/execution-report.md` with this run's factual evidence: work-order ID/edition, Owner handoff, local baseline/preflight, exact Phases A and B SHAs, changed path ledger, blob/readback evidence, G01–G12 results, commands/checks actually run, limitations, cleanup and next review step.
2. Create one **report-only** commit; fast-forward push and verify remote readback. Never embed that report commit's own SHA inside its contents. Give its SHA and GitHub URL in the chat handback.
3. Leave issued ACTIVE unchanged; leave ROADMAP, NEXT and OUTLOOK unchanged. No R00.C closure or R00.D promotion in this run. Finish with clean Git tree and truthful temp-cleanup evidence.

## 8. Acceptance gates (A1 documentary scope)

| Gate | Required direct evidence |
| --- | --- |
| G01 — Authority/baseline | Exact issued ACTIVE, matching direct Owner handoff, expected HEAD and original blobs; actual Solo root/branch/worktree verified |
| G02 — Phase A | One order-only commit, unchanged Owner-installed bytes, GitHub readback |
| G03 — C01/C02/C03 | Governed registry, independent declaration states, two-field inherit/suppress + current 0…N, direct ancestry and effective cardinality; no stale nearest-wins contradiction |
| G04 — C04/C05 | Per-value provenance, suppression trace and validated, restricted public projection including privacy refusal |
| G05 — C06 | Structured optional 0…N location, typed fields, explicit identified-record extension and field provenance, no inference/routing; old illustrative examples reconciled |
| G06 — C07/C08/C09 | Extensible registered roles, validated Entity/generic references, per-role cardinality, explicit suppress+local and default-deny generic fallback |
| G07 — C10 | Draft/published distinction, unlisted still published, fail-closed validation, no silent repair or publication fallback |
| G08 — Contact boundary | Public `mailto:` address disclosure truthful; server-resolved internal simple-form recipient explicitly deferred to R00.F; no advanced forms or email provider added |
| G09 — Preserved contracts | R00.A/B identity/publication/locale/routes, single-Entity, graphic ownership, Hexagonal Architecture and other accepted invariants unaffected |
| G10 — Scope and publication | Exactly one two-document-only Phase B commit; one separate report-only Phase C commit; expected GitHub remote revisions and no other tracked path changes |
| G11 — Repo hygiene | WIP four files, ACTIVE still exact issued order, no second checkout, clean tree and truthful temp cleanup report |
| G12 — Honest evidence | One current report; reviewable gate evidence, no application implementation, schema, tests/build/deploy or R00.C administrative closure claimed |

For **every** gate record `PASS`, `FAIL`, `UNVERIFIED`, or `NOT APPLICABLE`, plus direct evidence and reasons. Review documentary examples in §6 but do not present them as actual runtime tests. Software build/unit/integration/browser tests are `NOT APPLICABLE` to this documentation-only task with an explanation; documentary consistency and Git proof remain applicable.

## 9. STOP conditions, exclusions and required handback

STOP if the exact Owner-issued order and direct matching handoff cannot be verified, local tree/remote baseline mismatches, unknown dirty paths or worktree are present, a required change would exceed the exact file ledger, an accepted contract would be overridden, a material new architectural decision would be needed, unsafe publication cannot be verified, or the authorized Git mechanics actually fail. Do not solve by self-issuing, self-accepting, selecting R00.D, creating a second repo, ignoring failures or asking Owner to perform normal authorized Git commands.

On success provide: exact Phases A/B/C commits and GitHub URLs, initial/final HEAD, full diff/path verification, gate-by-gate results, one canonical report URL, honest local/remote verification and cleanup, and a clear statement that **independent Chief Architect review remains outstanding**. On failure report completed subset, blocker and safe resumption point, publishing the canonical STOP report when safely possible. Do not modify ACTIVE, NEXT or ROADMAP for closure; that requires a later expressly Owner-issued order after `ACCEPT`.
