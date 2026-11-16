# SOLO-WO-118 R20 — BIND PREPUBLICATION GRAPHIC PREPARATION TO TYPED PAGE/REPRESENTATION VISIBILITY (R04.C1), WITHOUT SERVING OR ROUTING

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's one Windows checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Technical branch (existing; do not create another):** `solo-wo-118-avif-v4-r01`  
**Canonical ACTIVE path:** `.documents/WIP/workorder-active.md`  
**Required bounded outcome:** `R04C1-TYPED-REPRESENTATION-VISIBILITY-GRAPH-BOUND-UNPUBLISHED-GRAPHIC-PREFLIGHT-N131-TWO-HOST-V1`.

## 1. Chief Master Architect disposition, exact pins and authority

The Chief Master Architect independently **ACCEPTS R19 only for its bounded, unpublished graphic package**. This is not permission to route, serve, publish, merge, release, deploy or close R03.C / Stage 03. Read the canonical R27 report at `main` commit **`e82c9d847dc7685797124d2a4aebd7e68cf67329`** (report Git blob `7990efa69a85b26244da8aa771ee16ab70ac7669`). The final technical branch head is **`07b2052a48d237959b1f79ce6c14fc91b45dc769`**. The failed first R19 technical commit `8e88529501b390964f96fccafeb4d5873f6dcd06` and red run **`43853803633`** remain historical evidence, not the accepted state.

R19 technical run **`43856345609`**, attempt 1, at the exact accepted SHA:

| Job | Native job ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133261472669` | **success**; N130 4/4, authentic and four mutated logs correctly handled |
| `verify (windows-2025)` | `133261472522` | **success**; N130 4/4, authentic and four mutated logs correctly handled |
| `avif-reference-oracle` | `133261472204` | **success**; independent pristine/three-mutation oracle |

R27 **main** fresh baseline run **`43859397010`**: Ubuntu job `133271917576` **success** and Windows job `133271918059` **success**. Technical architecture on both hosts: `packages=4 targets=39 files=77 edges=93`.

R19 established `prepare_mixed_graphic_package_v1` with whole-candidate source acquisition, six-format confined inspection, a single in-flight byte handoff, exact Platform-root registration identity, sanitized canonical SVG, raster pass-through, content-addressed in-memory outputs, final suppression/internal filtering and zero partial outputs. The current `PrepublicationDisplayEligibilityV1::{Public,Unlisted,Internal}` is expressly labelled **untrusted test-only input**, not real manifest visibility or route authority. R20 closes that *particular* gap at the typed EFS preflight boundary; it does **not** implement the full Stage 04 route/locale/HTTP pipeline.

**Owner direction to preserve:** the pinned Platform `1.0.0` library is the highest graphic baseline only for **explicitly registered role/slot defaults**; a closer validated `Supersede` wins, suppression is final, and invalid closer assets refuse without fallback. Production default registration remains empty unless the Owner separately names exact approved role/slot → source pairs. No icon is inferred to be a favicon, logo, banner or background from its filename.

## 2. Canonical contract and precise R20 scope

Binding sources, **at the frozen baseline**:

- `.documents/Architecture/ARCHITECTURE.md` §3.2A (four-case Page `visibilityDefault` / representation `visibility` rule, public versus unlisted), §3.2B (unlisted exact-case route, not yet implemented), §§6.2B, 6.2J, 6.2K, and §6.2I.3 (R19's explicitly untrusted display-eligibility input).
- `.documents/WIP/plan-roadmap.md`, Stage 04 `R04.C` and `R04.A`–`D`, and the still-open `R03.C` delivery gate.
- `crates/solo-domain/src/efs_ownership.rs` for accepted EFS membership and precise qualified Page/representation identity; `crates/solo-domain/src/property_resolution.rs` for qualified owners and the accepted graphic cascade.
- `crates/solo-application/src/graphic_delivery_preparation.rs` and its single candidate-scoped, unpublished in-memory package API. The full media proof and catalogue pin must be **reused**, not re-implemented.

**Owner-accepted visibility semantics; do not reinterpret:**

| Page `visibilityDefault` | Representation `visibility` | Effective representation state |
|---|---|---|
| omitted (default `public`) | omitted | `public` |
| `public` | explicit `unlisted` | `unlisted` |
| explicit `unlisted` | omitted | `unlisted` |
| explicit `unlisted` | explicit `public` | `public` |

The **only** representation visibility values are `public` and `unlisted`. `internal` is a **property-value exposure**, not a third valid manifest representation visibility. A representation with effective `unlisted` is **not secret or access-controlled**: the future approved route system may serve it at its **exact, declared, case-sensitive** direct URL, with `noindex, nofollow`, but it must be absent from automatic navigation, sitemap, `hreflang`, language switch, `x-default`, aliases, locale-less landing, and missing-translation fallback. There is **no real route or URL in this work order**, so R20 may report only intent/eligibility, never actual HTTP, crawler or browser behavior.

**Bounded implementation:** build a pure, closed, graph-bound typed **visibility preflight** for the exact Page/representation membership of one already validated EFS graph; bind the resulting effective state to the **same** R17/R19 candidate and selected qualified representation; then call the **unchanged R19 in-memory preparation** through a new opt-in, visibility-derived preflight entry point. A caller may not promote an arbitrary untrusted display enum into a publication or route capability. This is **R04.C1**, not a claim to complete `R04.C`, `R04.A`, `R04.D` or `R03.C`.

## 3. Exact positive behavior to implement

### 3.1 Domain: typed, complete visibility ledger

Implement a closed, I/O-free module (suggested path `crates/solo-domain/src/representation_visibility.rs`) using existing `EfsOwnershipGraph`, `QualifiedOwnerV1` and Page/representation IDs:

1. Accept typed **manifest-structured Page declarations**: exactly one Page visibility declaration per Page in the EFS graph, with an optional Page default and zero or more exact representation overrides **within that Page**. The absence of a Page default means `public`, and absence of a representation override inherits the Page default. Expose a strict `public|unlisted` parser for eventual manifest use; reject any third token (`internal`, `private`, `hidden`, empty, mixed case or unknown). Do not add a second source for visibility (no body front matter, JSON content, Markdown, filename, route, locale preference, geographic field, or CSS override). *This work order does not claim an actual strict JSON manifest parser exists.*
2. Bind **complete EFS membership**, not just counts: enumerate every Page and every representation of the exact validated graph using their full Entity→Facet→Site→Page→locale identity; compute one effective result for **every** representation in stable graph order. Refuse missing or duplicate Page entries, unknown or cross-Entity Page, wrong-Facet/Site Page, foreign representation, duplicate or conflicting representation override, and any incomplete ledger. No sibling or same-named Page in another Site shares a declaration.
3. Return an immutable `RepresentationVisibilityLedgerV1` (names may be refined for idiomatic Rust) with a closed effective `Public|Unlisted` value, exact owner, recorded origin (`page-default` or `representation-override`), `is_discoverable_intent` true **only** for `public`, and `robots_intent` equivalent to `noindex,nofollow` for `unlisted`. These are *typed policy facts*, **not response headers**. No URL, route, path or publication authority exists on the ledger.
4. Domain constructors must reject fabricated graph membership before an application may use the ledger; expose no public setter allowing an external caller to overwrite an effective state after validation. Keep closure/equality sufficiently strong to prove that a ledger for another Entity graph, sibling Site, locale or Page is not accepted for the current candidate.
5. Preserve the existing property-value `Exposure::Public/Internal`, and preserve the R17 root-default `GraphicCascadeInputV1` unchanged. Internal winners are still omitted by property projection even when the representation is `public`. Do not confuse Page unlisted with content/private property exposure.

### 3.2 Application: bind the ledger to R19 *without* opening delivery

Implement an opt-in application wrapper (suggested `crates/solo-application/src/graphic_visibility_preflight.rs`) around **existing** `prepare_mixed_graphic_package_v1`; it must not copy the R19 staging algorithm:

1. Require one `GraphicCascadeInputV1`, the exact mixed plan of that candidate, one fully qualified **representation** target, the graph-bound visibility ledger and the accepted existing Entity/Platform/media ports and limits. Compare the ledger's **whole owner membership** to the candidate's validated graph, not just a selected ID or entry count. Refuse foreign or partial ledgers and any selected owner that is not a representation **before opening any root or launching any worker**. A caller's claim that a representation is `public` is insufficient.
2. Derive the R19 prepublication display intent **only** from the ledger's effective state: `Public → PrepublicationDisplayEligibilityV1::Public`; `Unlisted → PrepublicationDisplayEligibilityV1::Unlisted`. Do **not** treat `internal` as a manifest visibility. Preserve the accepted R19 method as an explicitly **test-only/inert preparation seam** for parity; do not relabel it as production authority or call it directly from a delivery adapter.
3. For a `public` representation, invoke the accepted complete mixed validation and immutable in-memory staging. This proves the staging selection, not serving. For `unlisted`, retain the existing R19 conservative **empty, nondiscoverable prepublication package** while still validating the complete candidate (no silent bypass of invalid assets). Record that this is merely a preflight hold: once R04.A exact-route authority exists, unlisted direct delivery may become permitted under a *separate* order; R20 cannot assert it is permanently forbidden.
4. Expose a closed, result-only preflight view with exact owner identity, effective visibility, discovery/robots *intent*, package summary/digests where applicable and **all** `publication_authority=false`, `route_authority=false`, `release_authority=false`, `derived_output_publication=false`. No source path, private property value, hidden qualified owner, hostname, unapproved alias, raw rights-holder data, or token that could be passed as a public URL may be exposed from a public-intent view. Keep internal diagnostics safe and attributable without serializing secret details.
5. The wrapper must not open another source handle, re-read bytes, re-run SVG sanitization, re-encode rasters, mint a default, change content keys or bypass R18/R19's late-failure zero-partial behavior. If eligibility binding fails, **zero readers and zero workers**. For a late mixed-candidate refusal, return **no partial entry/payload**.

### 3.3 Boundary and vocabulary: visibility ≠ discovery ≠ authorization

- `public` = eligible to be considered for future discovery, **only after** fully validated routes, publication and content authority are added. R20 does not create public discovery documents.
- `unlisted` = **not** automatically discoverable; someday direct-serveable only by an explicit exact-case authorized route with `noindex,nofollow`, not by a guessed alias or preference. R20 cannot produce such a route.
- `internal` = applies to property **values**, never a third representation state; its graphic material is validated but never exported by a public projection.
- A content key `sha256/<digest>.<ext>` is an immutable identity, **not** an address. No prefix, origin, route or filesystem location may be minted from it.
- The R19 prepublication package remains **in memory**. No `public/`, `dist/`, `static/`, `generated/`, temporary output tree, HTTP handler, server, route manifest, canonical URL, sitemap, robots file, release archive or deployment is authorized in product code.

## 4. Focused N131 acceptance — risk-proportionate, **5 cases on both native hosts**

Use an additive `mod n131` in the existing `crates/solo-cli/tests/media_worker_host_smoke.rs` (or separate precisely scoped domain/application integration targets **only if** the architecture checker requires them). Keep all existing N093–N130 bodies and their CI gate scripts byte-identical. **No repeat 81-icon graphics campaign.** Native tests may reuse the accepted existing `SOLO_TEST_SOURCE_FS_ROOT` fixture setup and accepted SVG/PNG/WebP sample bytes; every actual source access must still use the separate accepted confined root capabilities.

| Case | Required fact | Minimum proof |
|---|---|---|
| `n131_01` | Four-way Page default/representation override | Prove the **four exact rows** in §2, omitted default=public, complete graph record coverage, origin/provenance for effective state, two distinct Pages with identical ID in sibling Sites remain isolated. |
| `n131_02` | Unlisted vs internal, future exact-route intent | Show unlisted `discoverable=false`, `robots_intent=noindex,nofollow`, no automatic entry/alias/fallback, but **not private** and not permanently non-serveable; explicit `internal` rejected as a representation token while internal property winner stays hidden. No actual URL or HTTP output. |
| `n131_03` | Identity and authority refusals | Wrong Entity, Site, Page, locale, duplicate Page or override, missing Page, incomplete ledger, and foreign graph/target refuse. One misbound target refuses **before** any reader/worker count increments. |
| `n131_04` | One real public-intent graphic preflight | Using the real accepted installed Platform icon and one Entity source, derive a `public` representation from the ledger, invoke the existing R19 mixed source/worker/package path, confirm exact source→output SHA chain, canonical SVG stays vector, content keys unchanged, and every authority boolean false. Use representative sources, not 81 inspections. |
| `n131_05` | Whole-candidate + visibility privacy + legacy parity | The same complete candidate with an effective unlisted target yields zero discoverable staged records while still checking invalid internal/superseded/unlisted declarations; late invalid source refuses whole with zero partial; an unlisted target cannot silently be selected as a public sibling; an empty baseline/legacy candidate preserves its accepted prior result; no property/private resurrection. |

**Gate authenticity and forgery:** add one Linux and one Windows dedicated N131 CI step after N130, each running exactly `n131_` with `--nocapture --test-threads=1`, requiring exactly five test names, 5/5 success, one `n131_evidence case=01..05` marker each and grounded tokens (`default=public`, `override=win`, `unlisted_discoverable=false`, `robots=noindex,nofollow`, `internal_representation=refused`, `foreign_ledger=refused`, `before_read=true`, `svg_vector=true`, `source_pin=true`, `zero_partial=true`, `publication=false`, `route=false`). Use actual printed assertions rather than scripted evidence substitutes. Then reject **at least four directly altered copies of the authentic captured log**: deleted case, forced `unlisted_discoverable=true`, flipped `publication=true`/`route=true`, and forged `zero_partial=false` or wrong source digest. Emit `n131_gate authentic=accepted` **after** authentic verification, and `n131_gate mutation=rejected` **after** all mutation refusals. Reject if any earlier case is ignored or failed, and preserve an unequivocal nonzero CI exit on gate failure. No hard-coded verdict echo, no filtering, `continue-on-error`, `|| true`, test skip, or advisory conversion.

**Existing regression proof retained:** the workspace locked suite; N124/N125/N126/N127/N128/N129/N130 authentic+mutation gates on native Ubuntu/Windows; native filesystem confinement canaries; architecture self-test; locked policy/deny; fmt; Clippy; safe-code boundary; independent AVIF oracle; reproducibility and release-CLI smoke **as CI verification only** (not authority to release). Verify the actual architecture summary for this change, don't invent expected counts. A source-only typed feature calls for the focused five-case matrix plus retained gates, **not** a second 81× hostile image campaign.

## 5. Change ledger — tightly confined; stop if expansion needed

**Permitted on technical branch**, only as required for the specified implementation:

1. `crates/solo-domain/src/representation_visibility.rs` (**new**) and `crates/solo-domain/src/lib.rs` (additive exports only).
2. `crates/solo-application/src/graphic_visibility_preflight.rs` (**new**) and `crates/solo-application/src/lib.rs` (additive exports only).
3. `crates/solo-cli/tests/media_worker_host_smoke.rs` (additive `mod n131` only), **or** narrowly new domain/application test targets when needed for pure testing (state those exact paths before the first source edit; no pre-existing case deletion).
4. `.github/workflows/rust-verify.yml` (additive Linux/Windows N131 gates only; all other steps exact).
5. `.documents/Architecture/ARCHITECTURE.md` (small truth amendment to §6.2I.3 and §3.2A declaring **typed visibility preflight implemented, public route/strict JSON manifest/parser/HTTP still pending**, not contradicting Stage 04 contracts).
6. The canonical ACTIVE file and canonical execution report, each on its prescribed branch/transaction only. No separate policy register, no generated product file.

**Protected and not to be edited:** `platform/assets/**`, `.gitattributes`, the immutable `1.0.0` catalogue, MIT notice, donor provenance and review fixtures, Owner decision, `Cargo.toml`, `Cargo.lock`, `deny.toml`, the architecture checker/policies, existing Entity/Platform readers, native root/worker/decoder, R12 strict parser, accepted legacy property registry/resolution, accepted R17 cascade logic, R18 mixed inspection and R19 package implementation, every existing N093–N130 test body and gate. Do **not** add new external dependencies to solve visibility: its pure model requires none. If implementing the wrapper demands modifying any protected body, **STOP** with a precise blocker and forward proposal rather than quietly widening this order.

**No product defaults, new artwork or rights assumptions:** the approved `platform/assets/` tree remains exactly 84 files (81 Tabler MIT SVGs + catalogue + notice + provenance), catalogue SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`. The separate R16 Foundation ledger remains 142/142 paths, 101 canonical, 20 outstanding: Provelopment original `icon-services.svg`, two byte-identical aliases, ten neutral placeholders and seven parked third-party platform marks subject to P-1…P-4. No additional artwork is authorized by this order; **public visibility of the donor repository is not redistribution approval**.

## 6. Required execution transaction (single checkout, no silent authority)

Use the one existing checkout only. Reopen both SOPs and the four `.documents/WIP/` references; read the current ACTIVE and the canonical R27 report. Preflight: one `origin`, one worktree, four WIP files, clean tracked trees, empty stash, no pending local changes belonging to the Owner. Verify exact `origin/main` **`e82c9d847dc7685797124d2a4aebd7e68cf67329`** and technical **`07b2052a48d237959b1f79ce6c14fc91b45dc769`**. If branch heads moved, the preflight is dirty or the required provenance is absent, **STOP** with evidence; never rebase or reset away another actor's work. Observe all remote heads and leave unrelated Dependabot heads alone.

Follow the established **A28 → T28.0 → T28 → R28** discipline:

1. **A28 (main, ACTIVE-only).** Install this exact work-order text at `.documents/WIP/workorder-active.md`. Verify byte identity worktree/index/commit/origin; make one commit changing **only ACTIVE**. Do not commit a product source on `main`.
2. **T28.0 (technical branch, ACTIVE-only).** Advance the existing technical branch with the **identical ACTIVE Git blob**; compare against the preceding accepted technical head and require ACTIVE as the only changed path. Do not create another technical branch or worktree.
3. **T28 (technical implementation).** Commit only the §5 technical ledger. The original first implementation commit and any forward repair commits must remain visible; no force push, no red-run deletion and no gate waiver. Obtain **one fresh run at one final technical SHA** with native Ubuntu verify, native Windows verify and independent AVIF oracle all **success**; N131 all five cases on each host with authentic/mutation verdicts and preserved prior gates. If a gate cannot run inside the approved ledger, STOP and report exact cause. Do not mark success by running locally on an unsupported filesystem.
4. **R28 (main, report-only).** Write `.documents/Reports/execution-report.md` with exact A28/T28.0/T28(+repairs) SHA, parent, tree, changed-path ledger, native run/job IDs, N131 evidence and previous-gate verdicts, exact product unchanged hashes, architecture summary, all refusal and privacy outcomes, and remaining stage/rights gates. Commit **only that report** on `main`, leaving ACTIVE unchanged. Push; obtain and append to Owner handoff the **fresh two-host main baseline** at R28 after its report commit. A report cannot know its own future SHA, so provide it accurately in the handoff, not a fabricated report cell.

Run the accepted continuous checks at every final technical SHA, not selective unit tests standing in for the workflow. Native Ubuntu/Windows CI logs, not an unsupported local filesystem, are the authority for real source/worker evidence. Test output may display deliberately failing adversarial child trials as part of a **successful** guarded job; describe those precisely rather than calling a green job red or concealing a real red run.

Use only ignored task-owned `.temp/` scratch, remove only the Agent's own disposable files and leave Owner scratch untouched. One `origin`, one checkout, four WIP files, no merge, no release, no deployment, no new branch, no tag, no PR and no external provider action.

## 7. Required evidence vocabulary and stop/accept decisions

The final execution report must explicitly distinguish:

```text
status=COMPLETE|STOPPED
r19_unpublished_package_accepted=true
page_representation_visibility_preflight_implemented=true|false
visibility_ledger_complete=true|false
visibility_bound_to_graphic_candidate=true|false
n131_two_host_gates_verified=true|false
strict_json_manifest_visibility_parser_implemented=false
stage04_route_claim_validation_implemented=false
stage04_exact_unlisted_http_delivery=false
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

R20 can be **ACCEPTED** only for the bounded preflight slice if these claims match the actual code and native logs, exactly one approved candidate cannot borrow another's visibility ledger, five N131 cases/gates pass on both hosts, and the independent AVIF oracle passes at the same technical SHA. It cannot complete `R04.C` (actual strict manifest and route eligibility are not built), `R04.A`, `R04.D`, `R03.C`, Stage 03, public asset delivery or a release. If any protected prior gate breaks, a caller can fabricate eligibility or route authority, a late source fails open, or a privacy-sensitive value enters a view, record **STOPPED** and issue the smallest justified repair proposal.

**Next order after a valid R20 acceptance:** implement the actual safe Stage 04 route/locale/visibility binding and production-mode HTTP vertical slice in bounded stages, including exact-case unlisted direct access and noindex/nofollow, before enabling any public serving. Separately obtain the Owner's named rights/role decisions for the 20 still-pending Foundation assets and the exact production default registrations. Neither future decision is silently conferred by R20.

**Owner-facing status at issuance:** R19 **ACCEPTED (unpublished package only)**; R20 **OWNER-ISSUED**; actual public routing and serving **CLOSED**; `R03.C` / Stage 03 **OPEN**.
