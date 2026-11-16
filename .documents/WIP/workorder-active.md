# SOLO-WO-118 R22 — ENFORCE MANIFEST-BOUND UNLISTED, ENTRY, LANDING AND SAME-PAGE FALLBACK POLICY BEFORE ROUTING (R04.C3)

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Sole Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Existing technical branch:** `solo-wo-118-avif-v4-r01`  
**Canonical ACTIVE destination:** `.documents/WIP/workorder-active.md`  
**Bounded outcome:** `R04C3-MANIFEST-BOUND-VISIBILITY-DEPENDENT-ROUTE-POLICY-PREFLIGHT-N133-TWO-HOST-NONPUBLISHING-V1`.

## 1. Chief Master Architect disposition and immutable starting pins

**SOLO-WO-118 R21: ACCEPT, bounded result only.** The Chief Master Architect independently fetched R29's canonical execution report, compared the actual technical and `main` changes, verified the real N132 output and forged-log verdicts in both native job logs, and verified the independent AVIF oracle and fresh R29 `main` CI. Acceptance covers the strict-parser-to-validated-EFS-graph-and-visibility-ledger bridge, **not** semantically validated routes, full BCP47, a publisher or R04.C closure.

| Authority | Exact pin / confirmed result |
|---|---|
| `origin/main` R29 | `28e803449e407c9c2f0b7ee45c6434a381c165f6` |
| `origin/solo-wo-118-avif-v4-r01` T29 | `f02aa20d24c5d7d87fed4d391cb6bc74f6d3b69e` |
| Canonical R29 report Git blob | `136d7a9b10e9b60442f51da496d61135b3cf2b21` |
| R21 technical workflow, final | `43878336393`, attempt 1; 3/3 success |
| Native Ubuntu | `133337210986`, N132 4/4, authentic and mutations rejected |
| Native Windows | `133337211553`, N132 4/4, authentic and mutations rejected |
| Independent AVIF oracle | `133337211574`, pristine and 3/3 hostile probes |
| Fresh R29 `main` baseline | run `43880314082`; Ubuntu `133344145296` success, Windows `133344145658` success |
| Technical architecture | four packages, 39 targets, 80 files, 93 edges, both hosts |
| Retained R14/R15 catalogue pin | `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`, immutable `1.0.0` |
| Retained N127 all-icon aggregate | `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca` |

R21 adds **only** `crates/solo-adapters/src/manifest_visibility_bridge.rs` as its implementation file. The accepted `parse_manifest_bytes` in `crates/solo-adapters/src/publication_manifest.rs` remains the **sole** strict, duplicate-aware manifest parser. The accepted `ManifestVisibilitySnapshotV1` retains (1) the very `ParsedManifestCandidateV1` produced by that parser, (2) one validated complete EFS graph, (3) one complete effective `RepresentationVisibilityLedgerV1` and (4) the digest of the supplied manifest bytes. The bridge currently enforces a deliberately conservative *canonical-shaped* full-locale profile, not full Unicode/BCP47 conformance. Its `check_manifest_efs_scope_v1` verifies complete EFS membership only; it does not authenticate property/source declarations.

**Owner directions remain binding.** The installed 81 Owner-approved MIT Tabler SVGs remain the complete `platformAssetLibraryVersion: "1.0.0"` product library (84 files including catalogue, MIT notice and provenance). Platform-first graphic baseline applies only to explicitly registered role/slot defaults; a valid closer explicit supersession wins, suppression is final, an invalid nearer declaration never falls back. The production default table remains **empty**. The other 20 canonical Foundation sources are individually accounted for but **not** approved for installation or distribution. Risk-based testing: do not rerun an 81-icon adversarial campaign for an inert policy change; retain N127 and all existing gates instead.

## 2. Scope and design boundary

**Goal:** From **one and the same** accepted R21 `ManifestVisibilitySnapshotV1`, produce a closed, deterministic, **nonpublishing** route-*policy* preflight for every localized representation and related Page/base entry. Enforce now the visibility-dependent authoring rules whose inputs are already available in the strict parsed candidate and the immutable visibility ledger. This must refuse an unsafe authored claim **before** a later route resolver, source reader or publisher can use it. Do not claim a resolved route, canonical URL, collision-free route ledger, validated Unicode path, accepted locale preference, content validity or active public response.

Binding Architecture:
- §3.2A: Page `visibilityDefault`, representation `visibility`, the four-case precedence matrix, unlisted reachable *only later* at an exact authorized address, `noindex,nofollow`, no discovery, and `internal` remaining a property exposure (not a representation state).
- §3.2B: public insensitive/sensitive claim policy, exact-case unlisted claims, public/unlisted collision restrictions, no guessed route alias and no unlisted translation fallback.
- §6.2B: strict authored `route.caseMatching`, `route.aliases`, Page `entryPaths`, `missingTranslation`, `base.landing`, with full qualified targets.
- §6.2J.5–7 and §6.2K: no unlisted aliases, no public entry/landing exposure of an unlisted-only Page, no fabricated fallback, no early authority, refusal before unsafe use.
- The future Unicode 18 / ICU4X / full BCP47 / IDNA, route collision, routing and HTTP obligations under §6A.7 and R04.A/B/D **remain unimplemented** and must not be simulated with ASCII `lowercase`, OS locale tables, `serde_json::Value` or ad hoc percent decoding.

### 2.1 Mandatory policy semantics

1. **One source of truth.** Public API consumes `&ManifestVisibilitySnapshotV1` created by R21; it does **not** accept caller-supplied booleans, a free-form Page/representation set, a replacement graph or a second visibility ledger. Traverse `snapshot.parsed().entity()` in exact authored containment order; each representation's effective visibility must be **looked up in the accepted ledger by fully qualified owner**, not recomputed from parsed strings. Every Page/representation in the graph must have exactly one policy record. Retain the manifest SHA as an internal provenance identity, not an authority claim. Refuse any impossible mismatch; never skip a Page or use an unqualified `home` key.
2. **Unlisted claims.** An effectively unlisted representation MUST have **no authored** `route.caseMatching` and **zero** `route.aliases`. Any caseMatching (including `sensitive` or `insensitive`) or alias is an attributed refusal. An unlisted canonical `route.path` remains an *inert authored string*, not a route that can be served, normalized, matched or discovered. Record its only eligible future match policy as `exact-unlisted`, with `noindex,nofollow` intent and `discoverable=false`. **Unlisted is not private/authenticated**; it may later have direct exact-case authorized delivery once R04.A/C/D is implemented.
3. **Public claims.** An effectively public representation may omit `caseMatching` (future default `insensitive`) or author `insensitive|sensitive` as the accepted parser permits; aliases remain syntactically declared and are not promoted into routes. Preserve whether caseMatching was actually present and the closed intended policy. Never silently change an unlisted representation into public because it has an alias, and never use declaration order or a guessed alias as conflict resolution. Path Unicode/NFC, route collisions, prefix, host, aliases' actual path validity and URL normalization remain explicitly unproved.
4. **Page `entryPaths`.** A Page with **zero public** effective representations cannot contribute an automatically discoverable locale-less entry: if it authors nonempty `entryPaths`, refuse with a Page-attributed closed cause. A mixed Page with at least one public representation may retain its `entryPaths` only as **pending inert claims**; this order does not select a locale, test paths for collisions or produce any redirect target. No Page/Site sibling fallback may make an unlisted-only Page eligible. An empty `entryPaths` is fine for an entirely unlisted Page.
5. **Base `landing`.** A present authored landing must name an **existing exact qualified Page** in the snapshot graph. Never infer the Facet, Site, Page or a landing Page from a slug, ID coincidence, locale, Site default, location or title. Compute one inert **landing-availability intent** from the Page's effective visibility: eligible for future consideration only if at least one public representation exists on the exact declared Page. A valid but entirely unlisted landing must never become discoverable or choose a sibling; report `unavailable-no-public-representation` (a future base-entry not-found intent), **not** an invented public route. An absent landing stays absent and means future base entry not-found. An unknown/partially bound landing refuses. No base URL or canonical address is emitted.
6. **Page `missingTranslation`.** Keep the accepted closed parser's three forms: `strict`, `default`, and explicit `locale`. `strict` creates no fallback. For explicit `locale`, require the target to be exactly a **declared representation of this same qualified Page** whose effective state is `public`; require its token to satisfy the current R21 restricted locale profile (or a separately accepted full locale validator; this order does **not** introduce one). Missing, foreign, unlisted, unsupported or ambiguous explicit targets refuse before any publication step. For `default`, inspect the enclosing Site's actual declared `defaultLocale` and this same Page's representations: record an available public target **only if one exists exactly**; otherwise record a conditional future not-found intent. **Do not reject a valid Page merely because its default locale lacks an eligible public representation**: the user may never request a missing translation. Do not invent other-locale, another Page or sibling-Site fallbacks. No `302`, URL, `Vary` or header is generated.
7. **Safe closed results.** Expose only typed per-representation policy facts (exact qualified owner, effective visibility, origin, closed comparison-policy *intent*, discovery and robots intent, alias count or equivalent inert metadata) plus exact Page-level fallback/entry/landing availability intents. No source paths, raw asset references, arbitrary author text, hidden property contents, hostnames or routes may leak through an outward/public-intent view. Error diagnostics may name safe structural field locations and stable tokens, never user-controlled text. Every outcome/method reports **`publication_authority=false`, `route_authority=false`, `serves_http=false`, `derived_output_publication=false`, `release_authority=false`, `deployment_authority=false`**. This is not a `RouteLedger`, `Published`, `ReadyToServe` or sealed candidate.
8. **Whole-input refusal.** A malformed, foreign, incomplete or contradictory input returns *one* typed refusal and **no partial policy records**. Reject before any source read or media child (the API should have no source/worker port at all). This is pure, bounded, deterministic work; no mutable global registry, filesystem, network, process, environment variable or clock is permissible.

### 2.2 Conservative compatibility constraints

The existing strict parser already retains authored `base`, `route.path`, `route.aliases`, `entryPaths`, `missingTranslation` and exact visibility fields with read-only accessors. **Do not edit `publication_manifest.rs` or introduce another JSON parser.** The R21 `ManifestVisibilitySnapshotV1` and EFS graph must remain unchanged. The new pure policy checker should operate on their read-only accessors and optionally new internal helper methods in its own file. Do not add a dependency merely to compare route strings. Full Unicode 18/ICU4X and IDNA are separately governed; no dependency changes in R22.

The policy checker may be an additive module `crates/solo-adapters/src/manifest_route_visibility_policy.rs`, exported minimally in `crates/solo-adapters/src/lib.rs`. No new crate, CLI, HTTP adapter, source reader, media worker, decoder or generic routing abstraction is authorized. Keep the future route-claim ledger and URL authority visibly separate from this intent ledger. Existing tests must pass unchanged.

## 3. Exact technical changed-path ledger

**Allowed technical source changes, and no others:**

| Path | Permitted change |
|---|---|
| `crates/solo-adapters/src/manifest_route_visibility_policy.rs` | **New file only:** pure manifest-bound route/visibility policy preflight, closed typed records/refusals, no I/O or source/route authority. |
| `crates/solo-adapters/src/lib.rs` | Minimal additive module declaration and exports only; no reorganization of existing exports. |
| `crates/solo-adapters/tests/publication_manifest.rs` | Additive `mod n133_manifest_route_visibility` with four focused tests; no existing test/helper deletion or weakening. |
| `.github/workflows/rust-verify.yml` | Additive native Ubuntu/Windows N133 authenticity/mutation gates; retain all earlier steps, jobs and pins. |
| `.documents/Architecture/ARCHITECTURE.md` | Concise truth note that this is an inert pre-route policy gate, not validated routing or serving. Do not rewrite R04.A/B contracts. |
| `.documents/WIP/workorder-active.md` | Identical Owner-issued ACTIVE bytes in A30 and T30.0 only, as required below. |
| `.documents/Reports/execution-report.md` | **Main R30 only**: replace canonical report after completed final technical proof. |

Strictly unchanged: `platform/assets/**` (all 84 files), the immutable `1.0.0` catalogue, MIT/provenance, donor inventory and fixture bytes, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `.gitattributes`, all domain/application source, `crates/solo-adapters/src/publication_manifest.rs`, `manifest_visibility_bridge.rs`, all native filesystem/media/worker/decoder/codec modules, current tests N093–N132, the R12 parser, `plan-*.md`, CLI binaries and production registration defaults. No new third-party dependency, license assertion or worktree.

**STOP** if this design cannot be safely implemented inside the permitted paths. Ask the Owner for a revised work order instead of changing a binding contract, introducing a homemade Unicode/locale router, or weakening any security gate.

## 4. Required transaction A30 → T30.0 → T30 → R30

Perform a read-only preflight at exact pins above: one `origin`, one working tree in the named Owner checkout, four WIP files, empty stash/unresolved index, clean tracking, same existing branch names. Report the actual number of remote heads as an observation rather than a quota. Protect all Owner-owned `.temp` entries and unrelated Dependabot heads. Compare existing `main`, technical branch and ACTIVE bytes with the exact expected source; STOP on drift, foreign changes or conflicting work.

1. **A30**: on `main`, install this issued work order verbatim into the canonical ACTIVE path, commit **that one path only**, push and verify its blob byte-exact in worktree/index/commit/`origin/main`. No crate code on `main`.
2. **T30.0**: on existing `solo-wo-118-avif-v4-r01` at T29, install the **identical** ACTIVE blob and commit **only** it. Verify the exact A30/T30.0 ACTIVE blob match and no out-of-ledger diff.
3. **T30**: implement the additive, bounded policy preflight, N133 tests and native gates, and one architecture truth note **only** within §3. Format/lint/test locally as feasible, and push a technical commit. If real hosted CI finds an issue, preserve the RED attempt and repair **forward** as T30.1/T30.2; never amend, rebase, force-push, silently skip or relax tests.
4. Obtain **one final three-success technical run at exactly one final technical SHA**: Ubuntu `verify`, Windows `verify`, independent AVIF reference oracle. Require N133 4/4 on both hosts, authentic and four genuinely rejected corrupted copies on both, and retained N093–N132 gates. Exact job logs, not Agent paraphrases, are proof. Architecture checker, strict Clippy, locked dependencies, byte/release/worker canaries remain active.
5. **R30**: on `main` parented to A30, modify **only** `.documents/Reports/execution-report.md`, push, then capture a **fresh two-job** main Ubuntu/Windows success baseline at exact R30. Keep technical branch unmerged. No additional WIP or release artifact.

No branch, worktree, tag, PR, merge, deployment, public URL, generated bundle or publication may be created. Task-specific `.temp` scratch may be used in the one checkout and removed on completion, without removing Owner-owned scratch.

## 5. N133 — four proportionate acceptance cases, on both native hosts

This is a policy-only, no-I/O change. Test **real strict manifest JSON bytes → existing R21 snapshot → new pre-route policy**; a model-only fabricated struct is insufficient. Retain the old 16+N132 publication-manifest tests unchanged. Place new tests additively in the existing adapter target and name them `n133_01_*` through `n133_04_*`. Each emits exactly one genuine `n133_evidence case=0X` line (after checks) for the dedicated `--nocapture --test-threads=1` CI step. Keep total N133 count **4/4**, not a new 81-asset or six-format media matrix.

**N133_01 — full four-case visibility matrix, case intent, sibling identity.** A strict JSON manifest with at least four representations across two sibling Sites and a mixed Page must yield one record per representation. Include public-insensitive omitted mode, public-sensitive authored mode, an unlisted Page-default, and a representation-unlisted override. Prove public intended mode vs unlisted `exact-unlisted`, source-of-visibility provenance, `noindex,nofollow` for unlisted, `discoverable=false`, sibling `home` identities never cross, stable order, R21 manifest SHA intact and all authority false. The representation path and base remain **unvalidated authored strings**: do not print a fabricated canonical URL.

**N133_02 — unlisted leakage and bad authoring blocked.** With actual strict JSON bytes, prove any effective-unlisted `route.caseMatching` is refused whether `sensitive` or `insensitive`, any unlisted `route.aliases` is refused, any Page `entryPaths` on a fully unlisted Page is refused. A mixed Page's `entryPaths` may be retained as pending only because its **own** real public representation exists, never because of a sibling. An internal representation visibility literal remains refused by the retained parser. No invalid claim produces a partial policy; show before-read/no-port semantics.

**N133_03 — exact landing and same-Page missing-translation guard.** Include valid exact qualified landing on a Page with public representation, absent landing, an exact but fully unlisted Page (unavailable/not-found intent, no route) and a wrong/foreign qualified landing (refusal). Exercise `missingTranslation: strict`, `default` when same Page has a public Site-default representation, `default` when none is available (conditional future not-found **without rejecting otherwise valid authored Page**), and explicit `locale` with a real public same-Page target. Refuse explicit target that is missing, unlisted, foreign, noncanonical for the R21 bridge profile or declared only on a same-named sibling Page. Show **no URL, no redirect, no locale preference inference** and no fallback to another Page/Site.

**N133_04 — deterministic whole-policy, foreign/partial and authority refusal.** Identical source bytes produce identical manifest SHA, complete records, eligibility and order; modifying only a harmless label changes the SHA while preserving governed policy. Refusals do not leak partial records. The policy cannot accept a caller-supplied replacement visibility ledger or a graph from another Entity because its only input is the accepted R21 snapshot. The already accepted R21 graph/scope check still refuses a foreign/sibling/missing ownership candidate. Verify complete coverage, closed safe diagnostic tokens, `publication=false`, `route=false`, `http=false`, `release=false`, `deployment=false`, no source bytes and no output tree. Distinguish this policy preflight from a true route-claim collision ledger or a validated manifest.

### 5.1 Native CI gate and anti-forgery checks

In both real host jobs add a dedicated N133 step, run the genuine `solo-adapters --test publication_manifest n133_` tests in a single capture with `--locked --nocapture --test-threads=1`, and make the step **fail closed** unless all four known case test names, exactly four real evidence lines in order, `test result: ok. 4 passed; 0 failed; 0 ignored`, the grounded case tokens and all expected no-authority flags are present; reject `FAILED`, `ignored`, truncated output, duplicate case, missing case, partial result, changed source SHA or falsified visibility/landing/missing-translation outcomes. Log verdicts **after** successful checks only:

```
n133_gate authentic=accepted
n133_gate mutation=rejected
```

Then mutate **four copies of the captured real test log** (not separate hand-written success tokens): (i) delete one case, (ii) forge an unlisted discovery/alias/case acceptance, (iii) forge landing/fallback to an unlisted or foreign Page or flip a declared refusal, (iv) forge `publication=true`, `route=true`, `zero_partial=false` or a zeroed manifest digest. All four must be rejected by the same verifier or the job fails. Do not use `continue-on-error`, `|| true`, echoes as proof, or mutable prefix comparisons. Preserve N124–N132 authenticity/mutation markers, independent AVIF oracle and all existing CI jobs. A dry-run is supplementary, never a substitute for genuine hosted Ubuntu and Windows evidence.

## 6. Reporting and bounded completion truth

The canonical `.documents/Reports/execution-report.md` at R30 must record:

1. Exact baseline, A30, T30.0, final T30(.x), R30 SHAs; clean branch/worktree evidence, changed-path ledger, file counts and unmodified Platform/catalogue bytes. Include all RED attempts, causes and forward repairs if any.
2. Why the existing strict parser and R21 manifest snapshot are reused unchanged; exact new pre-route types/refusal causes; effective unlisted, entry, landing and same-Page fallback behavior; limits and the restricted locale profile; evidence that no Unicode/IDNA/full route ledger or HTTP authority was invented.
3. N133 four case evidence on **both** native hosts, true authentic+mutation verdicts, retained N124–N132 and the independent oracle at one exact SHA, actual architecture measured on both hosts. No 81-icon rerun except retained N127.
4. Fresh **post-R30** `main` run and both native job IDs/conclusions. Report source/provenance/refusal tokens truthfully, not invented CI tokens.
5. The following explicit booleans or equivalent, all truthful:

```
status=COMPLETE
manifest_visibility_bridge_implemented=true
manifest_bound_route_visibility_policy_preflight_implemented=true
visibility_dependent_unlisted_rules_verified=true
same_page_missing_translation_policy_preflight_verified=true
locale_conformance_full=false
route_ledger_implemented=false
whole_manifest_semantic_validation=false
http_delivery_implemented=false
public_cascade_delivery=false
publication_authority=false
public_route_authority=false
derived_output_publication=false
release_authority=false
automatic_product_graphic_defaults_active=false
additional_assets_delivered=false
r03c_open=true
stage03_open=true
```

6. Remaining dependencies: full R04.A/R04.B validated Unicode-18/BCP47/IDNA route/base/alias collision ledger, R04.C publication semantics, real Stage 04 HTTP/locale/unlisted delivery and sealed candidate, plus separately authorized product default registrations and Owner rights/role decisions for the 20 still-unapproved Foundation assets.

**Status `COMPLETE` means only this R04.C3 nonpublishing pre-route policy outcome.** It does not mean R04.C, R04.A, R04.B, R04.D, R03.C or Stage 03 completed or closed. The team may not serve, route, release, deploy, merge or publish a graphic by virtue of this outcome.

## 7. Explicit Owner authorization limit

This work order permits only the exact bounded work in §§2–6 and the A30 → T30.0 → T30 → R30 governance flow. It authorizes **no** outside licensing/trademark judgment, artwork import, logo/brand default, new Platform catalogue or alias, public preview, HTTP route, manifest activation, CDN, public asset writing, merge, release, deployment or Stage 03 closure. The 20 outstanding canonical Foundation sources remain recorded but undelivered. Any unmet safety prerequisite means **STOP** and an honest report, not a broadened implementation. The Chief Master Architect will independently ACCEPT/REVISE/STOP based on actual Git and native logs.

**Implementation Agent handoff on verified success only:** `SOLO-WO-118 R22 — manifest-bound route/visibility policy preflight COMPLETE (nonpublishing)`.
