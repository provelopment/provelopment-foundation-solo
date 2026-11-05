# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-009`  
**Edition:** `R01`  
**Title:** R00.B — Canonical address, locale and Unicode route-contract codification  
**Approval marker:** `OWNER-ISSUED` **only upon exact Owner installation and separately delivered matching handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `7d6fae688b75d39d630a7626e4183f6a3078dfcc`  
**Matching direct handoff:** `SOLO-WO-009 R01 — Implementation Agent Handoff`  
**Work classification:** Decisions-only canonical documentation reconciliation; no application implementation  
**Quality standard:** A1, applied proportionately to documentary contract fidelity and verifiable publication evidence  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP` against GitHub-published changes; Owner retains closure and further-work authorization.

> **Owner issuance boundary.** This is a prepared work order, not self-executing authorization. It becomes executable only if the Owner installs the complete approved text in `.documents/WIP/workorder-active.md` and separately delivers the matching `SOLO-WO-009 R01 — Implementation Agent Handoff`. A conversational decision, this draft, NEXT, ROADMAP, or a previous handoff is insufficient. The Implementation Agent must not interpret its own work as accepted or close R00.B.

**Frozen Approved Scope:** §§1–9 below. Only the Owner may approve amendments. The Implementation Agent shall not silently extend or reinterpret the accepted decisions.

## 1. Objective, authority and boundaries

Integrate the Owner's accepted R00.B decisions B01–B12 into Solo's existing canonical product and architecture documents, reconcile contradictory repository summaries and procedural invariants, and publish a single factual execution report.

The specific architectural amendment is **B05-B**: public routes remain case-insensitive by default, but the author can deliberately select case-sensitive matching for an individual localized route claim. Validated Unicode routes, meaningful capitalization, localized slugs, explicit route ownership, strict collision refusal and visitor-controlled language selection are first-release requirements.

Preserve R00.A without modification to its substance: one Entity, explicit stable EFS IDs and parent associations, versioned publication manifest as sole publication authority, real authored representations, source integrity and fail-closed refusal. Preserve strict Hexagonal Architecture, separation of address from ownership and location, Page-level `visibilityDefault`, representation-level `visibility`, exact-case unlisted behavior and the four-slot WIP lifecycle.

This task **does not** create code, tests, dependency manifests, schema files, new architecture records, WIP files, releases, deployments, preview services or a new work order. It must not select a programming language, framework, hosting provider, database or implementation library.

Stage 00 remains open; R00.B remains pending independent review and Owner-authorized closure. R00.C–R00.G remain pending. Do not consume the ROADMAP R00.B obligation or replace the NEXT candidate.

## 2. Binding Owner decision ledger — B01 to B12

Treat the following outcomes as Owner-selected architecture, not options for the agent to reopen.

### B01 — Configurable base address

Use standards-based normalization with strict validation for one authored effective canonical HTTP(S) origin and optional path prefix.

Validate scheme, host, applicable port, path boundaries and prefix safety. Normalize well-defined equivalences without guessing an intended unsafe address. Refuse credentials, query and fragment components in the configured base, malformed encodings, ambiguous separators and traversal. Preserve the prefix exactly once across routing, links, metadata and assets.

A valid base-address change is configuration-only and never changes Entity/EFS identity or requires redesigning the router. Never use arbitrary incoming `Host` or forwarding headers as canonical-address authority.

### B02 — Explicit Page routes and localized routes

Public routes are explicitly authored addresses, never technical IDs or permanent Page identities.

The first release supports a Page-level route/default path with optional **localized representation-specific route declarations**. The initial default may be shared across languages; authors may deliberately assign different slugs to individual localized representations.

Each route claim resolves to the exact declared Entity/Facet/Site/Page ancestry and its actual representation. The same language-neutral Page may have differently spelled canonical routes in different full locales.

No route, title, translation, Site name, Facet name, hostname or location may implicitly create, move or rename a Page. Technical IDs remain stable when routes change.

The physical JSON/Markdown manifest schema and final field placement belong to R00.D. Record the accepted logical contract without inventing a second publication authority.

### B03 — Locale-less entry routes

The Installation's configured base entry supports the already accepted locale-selection sequence:

1. Supported saved visitor preference.
2. Supported, explicitly matched/mapped browser `Accept-Language`.
3. Applicable authored default full locale.

Additional locale-less Page entry routes are permitted **only when explicitly declared** and validated. Each such claim must identify one specific language-neutral Page and Site before preference selection, then redirect to that Page's actual public representation in the selected or authorized fallback locale.

No locale-less route may guess a Page from its translated slug, derive geography, publish an unlisted Page, or create a locale-less canonical representation.

Explicit full-locale URLs always win over preferences.

### B04 — Validated Unicode routes

Allow intentional Unicode Page-route strings in the first release. Do not impose an ASCII-only authoring restriction.

The logical route contract shall require:

- Valid Unicode text normalized to NFC for authored route comparison and canonical output.
- Well-defined UTF-8 URI encoding and one controlled percent-decoding boundary.
- Separate treatment of decoded text and encoded URI delimiters.
- Refusal of malformed encodings, encoded path separators, backslash-based ambiguity, dot-segment traversal, unsafe controls, ambiguous multiple decoding and route claims that normalize into prohibited conflicts.
- Deliberate handling of combining marks, non-Latin scripts, meaningful Unicode capitalization and script-sensitive characters.
- Consistent, version-controlled Unicode/casing behavior across validation, matching and URL generation.
- Detection and actionable diagnostics for dangerous confusable route claims, without silently translating or equating words merely because they look alike.

Do not equate the approved EFS technical-ID grammar or its 63-character limit with the separate public-route grammar.

Any lower-level Unicode data/profile selection that must be pinned to a future toolchain shall be identified as an explicit R00.E prerequisite, not guessed during this documentation run.

### B05 — Author-selected case matching and strict collision refusal

**Default public behavior:** Case-insensitive matching for each public localized route claim unless the author explicitly selects otherwise. Its canonical Page-route path uses the accepted deterministic Unicode-aware lowercase spelling.

**Optional public behavior:** An author may choose case-sensitive matching for an individual localized route claim. Its canonical Page-route path preserves the authored NFC-normalized Unicode capitalization. The author may deliberately assign `About`, `about` and `aBout` to distinct public Pages when all relevant claims use sensitive matching and validation proves no conflict.

**Unlisted behavior:** Effective unlisted representations always require exact-case authored Page-route matching; an unlisted route cannot enable case-insensitive lookup. Its exact authorized URL is directly reachable but excluded from generated discovery, with `noindex, nofollow`. Unlisted remains nonprivate.

**Locale handling:** Full-locale segments always normalize by Solo's accepted full BCP 47 rules, regardless of Page-route matching. Page-route matching is separate from locale-segment matching.

**Claim-level granularity:** Matching is associated with the authored localized route claim; no automatic Site-wide, Facet-wide or locale-wide case-sensitive setting is introduced. The proposed logical setting is `caseMatching: insensitive | sensitive`, defaulting to `insensitive` for public claims. Its concrete schema representation remains R00.D.

**Unicode comparison:** Specify a single reproducible NFC-based comparison pipeline. Insensitive comparisons use full Unicode caseless behavior with explicitly documented and tested locale-specific tailoring when applicable; sensitive comparisons use NFC-normalized exact comparison, without case folding. Canonical URL construction is distinct from comparison. The chosen Unicode data version must be pinned before executable implementation.

**Publication collision rules:**

1. Build and validate the complete set of canonical routes, permitted aliases and locale-less entry claims before publication.
2. Route matching must never use declaration order or first-match-wins to resolve ambiguity.
3. Identical effective claims competing for different destinations are invalid.
4. Different sensitive public claims with distinct exact normalized keys may coexist even if their caseless keys match, provided no other claim creates ambiguity.
5. Any insensitive public claim overlapping a sensitive or another insensitive claim is invalid when the claims cannot be resolved unambiguously; do not use hidden precedence to rescue it.
6. Preserve the stricter existing public/unlisted **caseless-collision refusal**, including where the public route itself is sensitive.
7. Preserve exact-case distinctness between safely normalized unlisted routes when their exact keys differ.
8. Different full-locale namespaces may deliberately map identical route strings to different Pages.
9. Alias and locale-less entry claims must pass equivalent overlap checks and never capture an unintended Page or reveal unlisted content.
10. Duplicate normalized canonical addresses and nonunique canonical targets fail validation. Do not accept multiple overlapping claims merely because they happen to point to the same representation unless an explicit nonambiguous alias contract authorizes the relationship.

Never lowercase the entire incoming path before determining the effective route claim. A wrong-case sensitive request has no implicit case-based redirect.

**Illustrative expected outcomes:**

| Case | Required behavior |
|---|---|
| Public insensitive `about`: `about`, `About`, `ABOUT` | Same Page; canonical lowercase URL |
| Public sensitive `About`, `about`, `aBout`, three different Pages | All three may coexist and resolve separately |
| Public sensitive `Morgen` and `morgen`, different German Pages | Both permitted when collision-free |
| Public sensitive `MORGEN` not declared | No guessed match; not-found |
| Public insensitive `morgen` plus public sensitive `Morgen` | Reject overlapping claims |
| Public sensitive `Morgen` plus unlisted `morgen` | Reject under retained conservative public/unlisted collision policy |
| Unlisted `abc` and `Abc` | Independently valid exact-case claims if otherwise unambiguous |
| `/en-001/about` and `/de-de/about` | Can identify different authored Pages |
| `/DE-DE/Morgen` | Normalize locale casing while preserving sensitive Page-route spelling |
| NFC-equivalent route spellings | One normalized comparison; refuse conflicting declarations |

### B06 — Preference matching

Support exact published full-locale matching and only explicitly authored preference mappings for shorter or otherwise unsupported browser language ranges.

Respect valid preference ordering and HTTP quality weights. Never infer a preferred region or translation from IP address, geographic location, Site ID, hostname, physical address or country property.

A saved preference is subordinate to an explicitly requested full-locale URL. Provide a usable account-free language-selection experience and allow a deliberate choice to persist through supported navigation without trapping visitors in a geographically selected language.

### B07 — Missing translation

Use a Page-controlled fallback policy, with valid public same-Page default fallback preferred.

Fallback is permitted only when the source route or explicit Page entry claim identifies the intended Page unambiguously. If translated slugs differ and no declaration identifies a Page for the requested path, do not guess that Page from a similarly spelled route.

A valid fallback redirects temporarily to the **actual canonical public representation of the same Page, Facet and Site**. Page-level strict/not-found and explicitly nominated public same-Page alternatives are permitted. Existing unlisted representations are not replaced by public content; missing translations cannot fallback to unlisted or another Page.

No automatic translation or rendering of a different language beneath the originally requested locale URL.

### B08 — Redirect semantics

Use condition-appropriate redirects:

- Stable public canonical spelling corrections: ordinarily `308`, where method/safety rules permit.
- Preference-dependent locale-less entry: temporary, ordinarily `302`.
- Valid missing-translation fallback: temporary, ordinarily `302`.
- Sensitive Page-route casing differences: never silently correct them by redirecting to another sensitive claim.

Keep GET/HEAD Page navigation separate from contact and other non-Page request methods. Construct redirect targets only from validated internal route authority; no open redirects or unapproved alias targets.

### B09 — Canonical address and aliases

One configured canonical public address is authoritative.

Support optional, explicitly authorized and validated aliases, including legitimate address migration use cases. An alias never becomes another Entity, Site, location-derived route authority or automatically trusted hostname.

Validate aliases against route conflicts and visibility boundaries; generated canonical, metadata and discovery URLs always use the effective canonical address. External DNS/TLS/hosting arrangements are distinct deployment responsibilities and are not performed by this order.

### B10 — Unlisted Home and public entry

Permit a fully unlisted Home Page.

Prefer an explicitly designated **real public landing Page** for the locale-less base entry when the nominal Home is unlisted. If no public landing representation is available, the locale-less base entry does not reveal an unlisted Home and returns not-found.

The landing selection must be authored, unambiguous and collision-checked. The default seeded `main/main/home` retains its stable EFS identity regardless of how the public entry is configured.

### B11 — Safe failures

Refuse malformed, unsupported, ambiguous and unauthorized routing without substituting a different Page.

Distinguish publication-validation failures from HTTP request failures.

Use accurate HTTP semantics: malformed unsafe requests may return `400`, absent routes or unsupported explicit locales return `404`, unsupported methods return `405`, unauthorized request authorities are refused appropriately (including `421` where applicable), and genuine internal failures remain server errors.

Errors and diagnostics must not disclose unlisted discovery inventories, private source locations or secrets.

### B12 — Cache safety

Distinguish visitor-preference-dependent redirects from deterministic explicit full-locale Page responses.

Prevent shared caching of personalized locale-selection responses; a conservative nonshared/no-store policy is acceptable. Do not let a visitor's cookies alter the language of an explicit full-locale response.

Permit ordinary safe caching of public canonical representations where applicable. Do not conflate unlisted discovery policy with authentication or confidentiality.

## 3. Explicit visitor language-control invariant

Codify the Owner's stated first-release usability outcome:

A visitor located anywhere—including Indonesia—must be able to select English or another actually supported public language without logging in, and then reach that representation through its explicit full-locale URL.

The initial public entry may consider a supported saved preference and browser language, but never physical location.

Every published public Page experience must provide an accessible, visible language-selection route where multiple real public same-Page representations exist. The selector follows the stable Page identity and the actual localized canonical URLs, not a guessed change to a slug.

A visitor selecting `/en-001/...` must not be returned to Indonesian merely because their browser prefers Indonesian, their location is Indonesia, or a saved preference disagrees.

Where localized slugs differ, simply replacing a locale URL segment is **not guaranteed** to find the counterpart unless an explicit validated shared-path alias exists. The supported same-Page language selector must nevertheless work without a login.

Only real public representations may be offered as available language alternatives. Missing or unlisted representations cannot be invented or advertised.

The user-experience outcome is binding at contract level now; actual rendered selector implementation and accessibility evidence remain scheduled for later stages.

## 4. Canonical documentation edits — exact path ledger

The Implementation Agent may change **only** the following permanent files for the operations stated.

| Path | Authorized modification |
|---|---|
| `.documents/Architecture/ARCHITECTURE.md` | Primary normative incorporation of all B01–B12 outcomes. Replace existing §3.2B universal case rule with the approved default/sensitive/unlisted contract; align §3.2, §3.2A where needed, §§4.1–4.4, verification table and §7 open/accepted decision inventory. |
| `.documents/About/FOUNDATION-SOLO.md` | Align product-level multilingual author freedom, optional localized routes, validated Unicode, public-case exception, visitor choice, Home entry and aliases. Update the accepted-versus-open distinction, avoiding duplication of detailed routing algorithms. |
| `README.md` | Replace outdated public-case summary and briefly describe explicit locale addresses, author-selected sensitive public routes and visitor language freedom. Retain its role as an entry point rather than a competing architectural authority. |
| `AGENTS.md` | Replace the universal public lowercase/case-fold invariant with the accepted claim-level default/exception and strict refusal rules. Do not alter authorization or role boundaries. |
| `.documents/SOP/sop-architect.md` | Update outdated public-case invariants and applicable independent-review examples, including Unicode and sensitive-route refusal cases. Preserve all Owner, WIP, report, A1 and Git procedures. |
| `.documents/SOP/sop-implementation.md` | Update obsolete public routing invariants and verification cases; preserve execution authorization, preflight, reporting and cleanup governance exactly in meaning. |
| `.documents/WIP/plan-roadmap.md` | Make **text-only precision adjustments** to the pending R00.B description and later routing/localization/visitor-navigation acceptance packages where their existing wording would falsely require unconditional case-insensitive public routing or forbid localized Unicode slugs. Keep R00.B listed as pending; preserve all stages, stage structure, prerequisites, package count and exit gates. |
| `.documents/WIP/workorder-active.md` | Owner installs this complete issued order. Agent may stage and publish the **unchanged installed bytes** in the initial authorization-preservation phase only. No discretionary edit or completion/acceptance note. |
| `.documents/Reports/execution-report.md` | Overwrite the single canonical report with accurate execution evidence in the reporting phase. Do not add another report. |
| `.git/**` | Minimal ordinary Git metadata needed for the expressly authorized publication sequence, solely under Architect SOP §6B. |

**Read-only:** `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, all other permanent repository paths, `.github/**`, LICENSE, `.gitignore`, `.gitattributes`, any code/tooling/dependencies and all other repositories.

Do not create a fifth WIP file, another report, a separate decision-register file, an archive, a second checkout or an implementation directory.

The approved route behavior belongs durably in the Architecture Charter, with concise references elsewhere. Do not paste multiple incompatible copies of the full contract into each document.

## 5. Documentation fidelity and conflict-resolution rules

The documentation must clearly distinguish:

1. **Accepted product and architecture outcomes:** B01–B12, including B05-B's newly authorized exception and first-release Unicode/localized-slug capability.
2. **Physical schema details reserved to R00.D:** concrete JSON field locations, file formats, manifest version encoding and source locator syntax.
3. **Toolchain/runtime selections reserved to R00.E:** library/runtime choice and the exact Unicode data version pinned for implementation.
4. **Later implementation stages:** compiled validators, HTTP adapters, actual language selector, redirects, caching, accessibility tests and deployed delivery.

R00.D/E may decide representation and implementation mechanics but **must not re-decide the accepted routing behavior**.

The following preexisting blanket statements must be corrected consistently wherever they appear in authorized files:

- All public routes are unconditionally case-insensitive.
- All public Page-route canonicals are unconditionally lowercase.
- Only unlisted content can use exact-case Page-route matching.
- Public mixed-case requests always identify the same Page.
- Public route collision checking needs only one unconditional lowercase key.

The replacement must say that public routes are insensitive **by default**, sensitive **by explicit localized-route author choice**, and effectively unlisted routes remain exact-case. Canonical spelling follows the selected policy. Preserve the lowercase canonical **full-locale segment** independently.

Do not change established ID grammar, visibility-field names, five-level property inheritance, one-Entity cardinality, publication authority, report procedures or Owner permissions.

Where a genuinely unresolved detail could alter observable routing semantics, **STOP** and report it instead of selecting a new architectural rule. Correct ordinary grammar, cross-references and terminology within the stated document scope without inventing additional product features.

## 6. Baseline and mandatory preflight

All project source-of-truth inspection for Architect review is GitHub-based. The Implementation Agent must independently follow the repository's existing safety and Git procedures before executing any write.

The required `main` baseline is:

`7d6fae688b75d39d630a7626e4183f6a3078dfcc`

Its GitHub file blobs are:

| Path | Required baseline Git blob |
|---|---|
| `README.md` | `1294bb12aa0eed370304cbc64a829a3f32d8fcac` |
| `AGENTS.md` | `30c5b02678d91140a1c74dc06aaefd333f42776e` |
| `.documents/About/FOUNDATION-SOLO.md` | `3f4e557e7b5b67c544bf2ce8ce718e5aec706fe8` |
| `.documents/Architecture/ARCHITECTURE.md` | `28c0629549259185a715d4291ea5d065cffa3ec8` |
| `.documents/SOP/sop-architect.md` | `6bea0dbcab31995a7d3f8020e8bfde8d6c195c75` |
| `.documents/SOP/sop-implementation.md` | `b56b14a0fec4b0b1b6342445eaa86ebf3a0d807d` |
| `.documents/WIP/plan-roadmap.md` | `5ebb53f7444e1ad59bc81eb0358cbb408295b17c` |
| `.documents/WIP/plan-next.md` | `375aee17fc9f8ae5355c119394fb8dca030cab23` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/WIP/workorder-active.md` before Owner installation | `ea5f20b736483c2e102456be1cea9104059fc81a` |
| `.documents/Reports/execution-report.md` | `ee722cf9ca88d9a51e0bdb44e8e87efacce4678b` |

Verify that the actual starting repository, branch and `origin/main` match this exact approved baseline, that the installed order matches the separately issued handoff, and that the Owner-installed ACTIVE replacement is the only expected initial permanent modification. Preserve unrelated or unknown material.

If the baseline has moved or the issuance cannot be verified, **STOP**. Do not silently rebase the order, rewrite history, use a second checkout or broaden the scope.

The Implementation Agent shall follow Architect SOP §6B, Implementation SOP §§4–5C and all applicable established safety controls. This order does not replace them with an alternative procedure.

## 7. Authorized execution and publication sequence

**Phase A — Preserve issued authority.**

After exact issuance and preflight verification, stage the unchanged Owner-installed `.documents/WIP/workorder-active.md` alone, make one ordinary commit, fast-forward push to the verified Solo `origin/main`, and verify the published blob and remote HEAD.

Do not begin Phase B unless the exact issued order is preserved on GitHub.

**Phase B — Codify the canonical R00.B contract.**

Edit only the seven documentation paths listed in §4: Architecture, Product, README, AGENTS, both SOPs and ROADMAP.

Check their combined consistency against §§2–5. Confirm NEXT and OUTLOOK are byte-identical to the baseline and ACTIVE remains the exact issued order.

Stage only those seven paths. Publish one ordinary content commit by an authorized fast-forward push, then independently read back the GitHub commit, changed paths and updated content.

Do not implement source code, update schema files, close R00.B, replace NEXT, clear ACTIVE or start R00.C.

**Phase C — Publish the canonical execution report.**

Overwrite only `.documents/Reports/execution-report.md` with a factual summary of the issued authority, initial baseline, actual changed files, GitHub commit SHAs, documentation consistency checks, conceptual positive/refusal cases, warnings, applicable A1 gates and not-run software checks.

Publish one separate ordinary report commit using the standing report-only authorization and verify its availability on GitHub. Do not include the report commit's own SHA inside the report; return it separately in the Implementation Agent handback.

Only these three scoped publication phases are authorized. No new branch, PR, merge, tag, release, deployment, dependency installation, provider operation, secrets access, history rewrite, destructive operation or other-repository write is authorized.

On failure, stop safely and report the actual state; do not manufacture a three-commit success or hide a partial outcome.

## 8. Documentary A1 acceptance gates

The Implementation Agent reports evidence. Only the Chief Architect independently assigns disposition.

| Gate | Required evidence |
|---|---|
| A1 — Authority | Exact `SOLO-WO-009 R01` issuance, approved source revision and matching handoff |
| A2 — Scope integrity | Only authorized permanent paths changed; source, toolchain, NEXT and OUTLOOK unchanged |
| A3 — Complete B01–B12 contract | Each accepted decision is represented coherently; nothing remains wrongly classified as undecided |
| A4 — B05-B fidelity | Default insensitive; explicit sensitive; unlisted exact-case; per localized route; Unicode-sensitive canonical behavior |
| A5 — Collision safety | Positive and refusal examples cover sensitive/sensitive, insensitive/sensitive, public/unlisted, Unicode normalization, aliases and cross-locale independence |
| A6 — Language freedom | Explicit full-locale URLs always win; no geographic override or login requirement; real same-Page language switching with localized slugs |
| A7 — R00.A and boundaries | EFS ownership, ID stability, manifest authority, visibility inheritance, source integrity and Hexagonal Architecture remain unchanged |
| A8 — Documentation parity | Architecture, Product, README, AGENTS, SOPs and ROADMAP have no remaining contradictory public-case or locale policy |
| A9 — WIP preservation | ROADMAP retains R00.B and Stage 00; NEXT stays R00.B; ACTIVE stays issued; OUTLOOK unchanged; exactly four WIP files |
| A10 — GitHub evidence | Issued order, documentation commit and final report published in the authorized sequence with exact paths and revisions |
| A11 — Honest verification | Conceptual cases distinguished from executed tests; no invented application, browser, CI, release or deployment proof |
| A12 — Governance | No self-acceptance, extra WIP records, second checkout, unauthorized provider action or closure |

For each gate, report factual `PASS`, `FAIL`, `UNVERIFIED` or genuinely `NOT APPLICABLE`, with reasons and evidence. Actual software/build/browser tests remain `NOT RUN` for this documentary task because no application implementation is authorized.

The final documentation must include enough positive and refusal examples for later Stage 04 and Stage 06 work orders to implement the contract without choosing policy independently.

## 9. STOP conditions and completion handback

STOP on any of these conditions:

- Missing Owner installation or mismatching direct handoff.
- Baseline revision, publication authority or path inventory mismatch.
- An unexpected modification that cannot be safely preserved under the existing SOP.
- A need to change accepted R00.A identity/publication semantics.
- A proposed alternative to the Owner-approved B01–B12 outcomes.
- Inability to reconcile public/unlisted collision safety with author-selectable sensitive public routes.
- An unresolved Unicode or locale semantic choice that materially changes the accepted observable behavior.
- A need for code, schema, dependency, provider, release or deployment changes.
- A request to self-accept, clear ACTIVE, prune ROADMAP R00.B or promote NEXT.
- Any prohibited Git, repository, privacy or security operation.

The implementation handback must contain the actual commit and report links, exact changed paths, checks and their results, any blockers, and an explicit statement that independent Chief Architect review remains outstanding.

**Required terminal state on successful execution:**

- The accepted R00.B contract is coherently incorporated into the authorized existing documents.
- ACTIVE still contains the exact Owner-issued `SOLO-WO-009 R01`.
- ROADMAP still lists R00.B; NEXT still proposes R00.B; OUTLOOK is unchanged.
- The one current execution report is published on GitHub.
- No source code or release/deployment operation has occurred.
- The Implementation Agent stops for independent Chief Architect review.

This task's success is **documentation codification**, not Stage 00 completion, implemented routing or authorization to start another work order.
