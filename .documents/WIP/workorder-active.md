# SOLO-WO-118 R21 — BRIDGE THE EXISTING STRICT PUBLICATION MANIFEST TO THE TYPED EFS VISIBILITY LEDGER, WITHOUT GRANTING PUBLICATION AUTHORITY

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's sole checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Technical branch:** `solo-wo-118-avif-v4-r01` (existing; no new branch or worktree)  
**Canonical ACTIVE destination:** `.documents/WIP/workorder-active.md`  
**Bounded outcome:** `R04C2-STRICT-MANIFEST-TO-EFS-VISIBILITY-BRIDGE-N132-TWO-HOST-NONPUBLISHING-V1`.

## 1. Chief Master Architect adjudication and exact starting authority

**SOLO-WO-118 R20: ACCEPT for its bounded R04.C1 typed Page/representation visibility-preflight result.** The Architect independently fetched the canonical R28 report, technical and main diffs, final native CI logs, and the two disclosed unsuccessful runs. This is **not** acceptance of complete R04.C, R04.A, R04.D, R03.C or Stage 03 closure.

| Item | Exact pin or observed proof |
|---|---|
| `origin/main` R28 | `40107f18c177d4769b4eec18f89e0492163015c9` |
| `origin/solo-wo-118-avif-v4-r01` T28.2 | `c9722f20b5f05b0044d39c6dbc066375b3ce3ede` |
| R20 final technical run | `43869043157`, attempt 1, at T28.2 |
| Technical Ubuntu | job `133305019449`, **success**, N131 5/5 and authentic/mutation verdicts |
| Technical Windows | job `133305019388`, **success**, N131 5/5 and authentic/mutation verdicts |
| Independent AVIF oracle | job `133305018923`, **success**, pristine oracle, 3/3 mutations |
| Fresh R28 main run | `43871506160` at R28; Ubuntu `133313610561` **success**, Windows `133313610796` **success** |
| Technical architecture | `packages=4 targets=39 files=79 edges=93` on **both** native hosts |
| Approved pinned Platform library | exactly 81 Owner-approved Tabler MIT SVGs + `catalogue-1.0.0.json` + MIT notice + provenance, **84 unchanged product files** |
| Catalogue SHA-256 | `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` |
| N127 aggregate SHA-256 | `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca` |

The two earlier R20 runs are **historical RED**, not the accepted result: `43866967937` at T28 `8b02f6e4b685ec336873ea47a364eb79a4e083e9`, and `43868157149` at T28.1 `b905f237a9a1288b797fbdc098bfe0cd01eb15b8`. Both failed because the N131 fixtures authored a conflicting same-slot logo alongside a registered root default. The accepted forward repair T28.2 changed test fixtures only. Preserve both failed runs in the history; do not rewrite commits, suppress failures or reclassify them as production defects.

**Critical discovery, independently inspected:** `crates/solo-adapters/src/publication_manifest.rs` **already contains** the strict, duplicate-aware, inert `parse_manifest_bytes(&[u8]) -> ParsedManifestCandidateV1` parser. It accepts the closed five-key `content/publication.json` shape, handles duplicate decoded object keys with a custom Serde visitor, enforces 2 MiB, checks the exact three independent `1.0.0` version strings, returns nested `EntityDeclarationV1` / `FacetDeclarationV1` / `SiteDeclarationV1` / `PageDeclarationV1` / `RepresentationDeclarationV1`, and records `DeclaredVisibility::{Public,Unlisted}`. It deliberately **does not** claim canonical BCP47, route authority, content/source proof or property registry validation. **Do not build a rival manifest parser** or pretend that R20 lacked this existing parser. R21's missing piece is the *inward conversion and binding* of this accepted inert parsed candidate to the separately accepted R02 EFS ownership graph and R20 visibility ledger.

Owner directives preserved: the Platform library is the highest graphic baseline only for **explicitly registered role/slot defaults**; a closer valid `Supersede` wins, suppression is final, an invalid nearer source refuses instead of reviving a root asset. Current production default table is empty. All Foundation artwork remains fully accounted for; the 20 unapproved canonical assets remain undelivered. Test effort must scale with new likelihood and consequence, **not** with the number of existing SVG files.

## 2. Exactly what to build — R04.C2, not the full publication stack

Implement **one additive, no-I/O conversion boundary** in the outer adapter layer, operating on the supplied exact publication-manifest bytes and returning a **nonpublishing manifest visibility snapshot**. The new surface must reuse the *existing* `parse_manifest_bytes`, existing public declared-node accessors, `EfsOwnershipDraft`/`FacetDraft`/`SiteDraft`/`PageDraft`, `build_representation_visibility_ledger_v1`, and the R20 visibility types. It should not parse authored content twice, parse a source file, read a root handle, create a URL, select a Page body, validate HTTP input or invoke a media worker.

### 2.1 Source and result contract

A reasonable API, with names free to vary for type soundness, is:

```rust
pub fn parse_manifest_visibility_snapshot_v1(
    supplied_manifest_bytes: &[u8],
) -> Result<ManifestVisibilitySnapshotV1, ManifestVisibilityBridgeRefusalV1>;
```

The **closed** result should own or otherwise lifetime-safely bind: (a) the original `ParsedManifestCandidateV1` produced by the existing parser, not a rebuilt look-alike; (b) one `EfsOwnershipGraph` built from that parsed Entity's *complete* containment tree; (c) one `RepresentationVisibilityLedgerV1` built from those same Pages and their exact visibility declarations; (d) the checked SHA-256 of the **supplied** manifest bytes as a provenance identity, using the existing trusted digest helper where appropriate; and (e) constant `false` claims for publication, route, HTTP serving, derived output, release and deployment. Getter APIs may expose read-only typed facts for later composing stages; the type must have no public constructor that accepts an arbitrary foreign graph or ledger as if they came from the manifest. No source bytes, private property content or unvalidated raw strings may become public-visible metadata by this operation.

**Use the existing parser.** No new JSON vocabulary, independent parser, `serde_json::Value` trust bypass, default unknown-field policy, `unwrap_or` reparsing-as-validity, or handwritten replacement for its decoded-key duplicate checks. The manifest remains the **sole** logical publication manifest at `content/publication.json`, but being parsed does not grant publication authority.

### 2.2 Exact graph/visibility conversion

Traverse **every** parsed Facet, Site, Page and representation, in authored containment order. Preserve full EFS-qualified identity and duplicate/limit refusals by constructing the accepted `EfsOwnershipDraft` and calling `.build()`; do **not** synthesize `main`, a Page, a missing locale or a route. Build exactly one `PageVisibilityDeclarationV1` for every declared Page, with `DeclaredVisibility::Public/Unlisted` converted to the closed R20 two-state type. Preserve every exact representation override with its actual canonical-shaped membership key. Feed the complete declarations and *that exact built graph* to `build_representation_visibility_ledger_v1`. Require the ledger to cover the built graph's entire representation membership; an incomplete or mismatched conversion refuses **all**, never an acceptable partial graph/ledger pair.

`PageDeclarationV1::visibility_default()` already resolves the omitted default to `public`: this converter may truthfully report the **effective** Page default, but may not invent a new fact about whether the default was explicitly authored. `RepresentationDeclarationV1::visibility()` retains `Option` and its absence must remain distinct from an explicit override. `internal`/`private` is never a third representation visibility. The existing R20 `robots_intent()` is a **policy fact**, not an HTTP header, route or content-access decision; unlisted is **not private**.

**Locale safety restriction:** the existing `LocaleKeyV1::from_already_canonical` explicitly validates only *membership shape*, not canonical BCP47. Never pass arbitrary raw manifest locales into it while claiming that locale validation occurred. For this bounded adapter bridge, conservatively accept the unambiguous **canonical-shaped base profile**: lowercase two/three-letter language, optional title-cased four-letter script, and mandatory uppercase two-letter or three-digit region (e.g. `en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`). Reject mixed-case, bare languages, inconsistent spellings, extensions or an ambiguous duplicate as **locale-not-yet-validated**, with no silent case-folding or region inference. This is intentionally **narrower** than all valid BCP47: full ICU4X/Unicode-18 canonicalization, locale aliases, extensions and full locale equivalence remain separate R04.B/E05 work. Record the restriction prominently in code and report; a currently unsupported but valid BCP47 variant is a safe refusal, **not** a claim that the standard forbids it. Apply the same restricted check to the Site `defaultLocale` (but do not invent a default representation or a new cross-Page membership policy). No URL, route or source-selection legitimacy follows from an accepted membership key.

The parsed manifest's `base`, route claims, aliases, missing-translation rules, `sourceCandidates`, inert `properties` and labels must **not be silently re-validated, discarded from the parsed candidate, or claimed production-ready**. Their existing *syntactic* checks remain intact; semantic conformance remains expressly pending. Keep the actual parsed candidate in the returned snapshot if necessary so no later code is led to believe it has a fully validated publication model.

### 2.3 Candidate-scope check — narrow and explicit

Provide a **separate, nonpublishing scope-check** that accepts a `ManifestVisibilitySnapshotV1` and an already validated `GraphicCascadeInputV1`, returning either `scope=exact-EFS-membership` or a closed refusal. Compare **complete**, fully qualified Entity/Facet/Site/Page/representation membership, not only the Entity ID, target owner, count, common Page ID or prefix. Require same membership and order under the accepted deterministic graph; a sibling Site's `home` and an identically named Page in another Facet must not pass as the same owner. A foreign Entity, a missing/unlisted Page, a changed representation locale, a missing sibling or an extra owner refuses **before any reader or worker**. A valid empty graphical contribution does not remove an EFS owner. This check **does not** claim that the graphic candidate's inert properties, source bytes, routes and other manifest fields were themselves derived from these bytes; it binds **EFS ownership scope only**. Do not name the result `Published`, `ValidatedManifest` or `ReadyToServe`, and do not accidentally widen the current R20 preflight's authority.

This scope check is safe and reusable in a later authorized real pipeline. Do not modify the R20 `prepare_graphic_visibility_preflight_v1` logic, the R19 package, R18 mixed inspection, R17 cascade or legacy property resolution under this order merely to turn the bridge into a speculative new full publication path. If it is necessary to expose a `ValidatedPropertyInputV1::owners()` slice, use the already existing public getter; no mirrored graph parser or new EFS ID registry is permitted.

### 2.4 Closed refusals and safe diagnostics

A failure in the existing manifest parser must preserve its exact `ManifestErrorKind` and a safe location, wrapped (not erased) by the bridge. Additional closed bridge reasons should cover malformed/noncanonical-shaped locale, EFS graph construction refusal, R20 ledger construction refusal, unexpected cardinality/membership inconsistency and candidate scope mismatch. Reuse typed existing errors where possible; diagnostic text is fixed ASCII and safe qualified field paths, never raw authored property values, secret data, source bytes, or internal ancestry serialized as a public response. No panic, partial snapshot, fallback to a second manifest, alternative `.json` source, library asset or previous ledger. Input with zero, duplicate, unsupported-version or overflow data refuses with no public side effect.

## 3. Strict allowed-path ledger

Only the **following** tracked paths may change, in the prescribed transaction steps. No broad `git add .` and no collateral dependency/security changes:

| Path | Authorization |
|---|---|
| `.documents/WIP/workorder-active.md` | **A29** on main and **T29.0** on technical, byte-for-byte identical Owner-issued order; no later edits |
| `crates/solo-adapters/src/manifest_visibility_bridge.rs` | **NEW** safe, no-I/O adapter; parsed manifest → graph + visibility ledger + narrow candidate-scope check |
| `crates/solo-adapters/src/lib.rs` | minimal new module and public re-exports; no architecture or unsafe allowance change |
| `crates/solo-adapters/tests/publication_manifest.rs` | **additive only** `mod n132_manifest_visibility` or equivalent four-case test module; all existing tests intact |
| `.github/workflows/rust-verify.yml` | additive two-native-host **N132** authenticity/mutation gate steps only; preserve all existing jobs and security gates |
| `.documents/Architecture/ARCHITECTURE.md` | one compact truth note near §3.2A / §6.2I.3: parser-to-graph bridge and intentionally restricted locale profile, no publication/route authority |
| `.documents/Reports/execution-report.md` | **R29** on `main` after completed technical proof, and this path **only** |

**Important:** `publication_manifest.rs` itself is **outside** the edit ledger. Its current strict parser, parsed declarations, decoded-key duplicate refusal, version checks and source selection declarations are accepted and sufficient for this bridge. If a genuine missing accessor or a parser defect makes a correct bridge impossible, **STOP** with exact source evidence and request a narrow Owner amendment; do not make an unreviewed change. Also out of scope: `Cargo.toml`, `Cargo.lock`, `.gitattributes`, `deny.toml`, all `platform/assets/**`, all donor fixtures and licensing, `.documents/WIP/plan-*`, `crates/solo-domain/**`, `crates/solo-application/**`, all existing security test bodies, `.github` except the single workflow, CLI runtime, decoder/sanitizer/worker, product release and all public routes. No new dependency version or third-party parser.

## 4. Repository and transaction protocol (mandatory)

Perform a read-only, exact-SHA preflight: `origin/main` must equal R28 `40107f18c177d4769b4eec18f89e0492163015c9`, and `origin/solo-wo-118-avif-v4-r01` must equal T28.2 `c9722f20b5f05b0044d39c6dbc066375b3ce3ede`. Confirm one `origin`, one worktree, no unresolved index/stash, the four approved WIP files and the actual current set of remote heads. Treat head counts as observations, not quotas; do not change unrelated Dependabot branches. Confirm the sole local checkout matches the permitted Windows path. Do not checkout a second clone, create branches/worktrees/tags/PRs or alter unrelated Owner-owned scratch. Stop on unexpected diffs or a conflicting ACTIVE blob.

Exact transaction:

1. **A29** — on `main`, commit **only** `.documents/WIP/workorder-active.md` with this file's bytes. Verify worktree, index, commit and `origin/main` blob equivalence after push. Do not place technical code on `main`.
2. **T29.0** — resume **existing** technical branch at T28.2, install the **identical** ACTIVE blob and commit **only** that path. Verify equality against A29.
3. **T29** — implement only §3 technical ledger paths. Run local no-I/O tests and format/lint checks as possible, without claiming fake real-filesystem success. Push one bounded implementation commit. If CI finds a genuine issue, repair **forward** as T29.1, T29.2 etc.; preserve RED history, never amend/rebase/force-push or weaken a gate.
4. Obtain one fresh **three-success technical CI run at exactly one final SHA**: Ubuntu `verify`, Windows `verify`, and independent AVIF reference oracle. Capture exact run/job IDs, complete success conclusions and the N132 live-log verdicts. N124–N131 remain active and green.
5. **R29** — on `main`, update **only** `.documents/Reports/execution-report.md` as the canonical report, parented to A29. Record all technical commits, failures, native evidence and bound truth. Push and obtain **fresh two-host main baseline** after R29, including exact run/job IDs. `main` remains without `platform/assets/` and technical code. Do not merge.

No Owner authorization is implied for deployment, release, publication, adding artwork, public preview, provider account, automatic root default, Stage 03 closure or `R04.C` closure. A task-owned `.temp/` scratch is allowed only within the existing one-checkout constraints and must be removed without touching Owner files.

## 5. N132: four proportionate acceptance cases, on both hosts

This is a **parser-to-ledger conversion** and EFS scope integrity change. The risk is misbinding private/unlisted identities, malformed declarations or borrowing another graph; it is **not** a change to a decoder or one of the 81 already-approved icons. Four focused cases are sufficient; do **not** write a new 81-asset media matrix or a second native source reader. Place tests additively inside the existing `crates/solo-adapters/tests/publication_manifest.rs` target and name them `n132_01`…`n132_04`, with one genuine instrumented `n132_evidence case=0X` line per case when `--nocapture` is run.

- **N132_01 — exact manifest → one graph → one complete visibility ledger.** Supply actual strict JSON manifest bytes with one Entity, sibling Sites that both have `home` Pages, an additional `about` Page and four localized representations, the R20 four visibility combinations (`public` default, `unlisted` override, `unlisted` default, `public` override). Require correct fully qualified owners, stable graph order, 4/4 ledger coverage, exact origin from Page vs representation, correct discoverability/robots **intent**, and `manifest_sha256` computed from source bytes. Compare the resulting ledger's facts to the R20 typed constructor over the same graph; there is no new inheritance algorithm. Assert all publication/route/release booleans false.
- **N132_02 — strict parser is retained; no escape hatch.** Negative matrix over the **real existing parser entry point as called by the bridge**: duplicate decoded `visibility` key (`"visibility"` plus `"\u0076isibility"`), unknown nested key, `visibility: "internal"`, malformed UTF-8, BOM, truncated JSON, trailing tokens, wrong `platformAssetLibraryVersion`, root/child missing key and 2 MiB+1. Include a source locale such as `de-de` or `en` and a syntactically plausible but unsupported locale extension; the *bridge* must refuse it without silently claiming BCP47 authority, while the parser's existing acceptance scope stays unchanged. Every refusal must preserve the appropriate closed cause without creating a graph/ledger or mutating any source.
- **N132_03 — foreign/partial/sibling scope is refused.** Start from a separately constructed validated `GraphicCascadeInputV1` with a complete matching owner graph and show `scope=exact-EFS-membership`; then change exactly one dimension at a time: Entity ID, Facet, sibling Site, same-name different-Page owner, missing Page, extra Page, representation locale and missing representation. All mismatches refuse before any read or worker. Explicitly prove that **matching only the Entity and target** would be insufficient, but is *not* the implementation. Do not falsely claim that unrelated authored graphic properties were authenticated from the manifest bytes.
- **N132_04 — binding determinism, provenance and nonauthority.** Reparse identical bytes and confirm byte-identical/equivalent source digest, graph membership and ledger rows; change only an inert, syntactically valid manifest field (e.g. label) and prove source SHA changes, while effective visibility stays a truthful typed fact and no content/route/publication permission emerges. Test that an unlisted representation is **not private** and receives only `noindex,nofollow` intent; internal graphic exposure remains an independent concept, never a representation token. The unchanged parsed candidate's routes and sourceCandidates are still merely inert declarations. No package, public URL, HTTP response, output file, release or deployment.

N132 checks must use **real bytes through the original strict parser**, not model-only simulations. Use the current test file's fixtures as a starting point, preserving every existing test. If a representative previously valid fixture uses `route.path="/"` (syntactically permitted by the inert parser), do not claim it is a conforming published route. If the manifest contains valid-but-not-yet-supported BCP47 extensions, refuse it at the explicitly narrow bridge rather than pretending the standard disallows it.

### N132 native CI gate and forged logs

Add one Linux and one Windows N132 step **after** the retained N131 gate. Each must run the actual existing test target's four N132 tests with `--nocapture --test-threads=1`; accept only the correct exit code, all four exact names, exactly four passing tests, zero ignored/failing tests, exactly one distinct evidence marker for each of 01–04, and anchored case-specific tokens (for example `rows=4`, `scope_exact=true`, `foreign_refused=true`, `duplicate_decoded_key=refused`, `locale_profile=restricted`, `manifest_sha_pinned=true`, `publication=false`, `route=false`, `zero_partial=true`). Do not put acceptance evidence in the gate's own echoed command/script; parse the **captured test output**. Both platforms must reject at least four **directly altered copies of the genuine log**: delete one case, forge a public/unlisted decision, change `scope_exact=true` to `false` or `foreign_refused=true` to `false`, and forge a publication/route or partial-result claim. Only after all mutations are rejected should the step print:

```text
n132_gate authentic=accepted
n132_gate mutation=rejected
```

A failed cargo command, absent evidence, an accepted forged log, a fabricated marker, a skipped test or a test filtered away must fail the job. No `continue-on-error`, `|| true`, permissive missing evidence, `#[ignore]`, test removal or test-only production bypass. Retain unchanged N093–N131 gates and their native security canaries; the independent AVIF oracle must pass. The **new** N132 test need not spawn an SVG child merely to re-prove the old 81-icon sanitizer. Existing N127/N129/N130/N131 steps already exercise those surfaces on the same two hosts. Proportionality is binding.

Expected structural budget: one new adapter module, zero new package/dependency edges and zero new test targets, so report the **observed** architecture count (approximately `4 packages / 39 targets / 80 files / 93 edges`) rather than hardcoding an unmeasured value. Preserve the existing architecture checker and run it on the actual checkout.

## 6. Strict preservation, refusals and rights

**Preserve without editing:** the entire product `platform/assets/` tree, installed 81 SVG contents and catalogue, MIT notice/provenance, existing `.gitattributes`, source-read capabilities, native Linux/Windows filesystem APIs, R17 default/supersession semantics, R18 whole-candidate mixed media inspection, R19 immutable in-memory staging, R20 typed visibility/preflight, legacy Entity-only APIs, media workers, parser, lockfiles, CI security tripwires, and every existing N093–N131 test body. `main` remains governance-only. Do not add a generic fallback, inferred filename/role default, second manifest, secret into public projection, stale source-path read, route or output directory.

**Precisely 20 outstanding Foundation canonical sources remain NOT approved for installation by this order:** the original `icon-services.svg`, the two duplicate-icon aliases, ten neutral role placeholders and seven third-party platform marks (`PARKED — NOT INTEGRATED`, P-1…P-4). The Owner's request to account for all publicly available Foundation artwork is already met by the 142/142 frozen R16 ledger; it is not a blanket grant of redistribution or trademark rights. The 81 already approved Tabler icons remain installed. Do not invent approval for the original icon, a new catalogue alias policy, placeholder-to-logo/favicon mapping or a third-party mark; do not mutate pinned library `1.0.0` into `1.1.0` without a separate order.

**Publication gate still closed.** R21 never produces the production semantic route ledger, full Unicode 18/BCP47 locale binding, schema-generated publication authority, real Content JSON/Markdown selection, HTTP listener, public assets, canonical URLs, immutable activated snapshot or release. The R20 `public`/`unlisted` decisions remain **prepublication typed policy facts** until those other stages are accepted. `R04.C` as a whole remains OPEN; `R04.A`, `R04.B`, `R04.D`, `R03.C` and Stage 03 remain OPEN. The newly built source digest is an internal provenance pin, **not** a signed author grant or source approval. Never add flags, methods or doc language that grant serving/release or claim a full production-manifest validation.

## 7. Canonical report and acceptance evidence (mandatory)

Write `.documents/Reports/execution-report.md` at R29 with:

1. Explicit `status=COMPLETE` **only if** every bounded R21 gate is actually satisfied; otherwise `status=STOPPED` and exact blocker. State `manifest_visibility_bridge_implemented`, `manifest_efs_visibility_scope_verified`, `n132_two_host_gates_verified`, `locale_conformance_full=false`, `route_ledger_implemented=false`, `whole_manifest_semantic_validation=false`, `public_cascade_delivery=false`, `publication_authority=false`, `public_route_authority=false`, `derived_output_publication=false`, `release_authority=false`, `automatic_product_graphic_defaults_active=false`, `additional_assets_delivered=false`, `r03c_open=true`, `stage03_open=true`.
2. Exact baseline, A29/T29.0/T29 (and repairs)/R29 SHAs, parent/tree/ACTIVE blob equivalence, exactly changed paths and no out-of-ledger changes; verified one worktree/origin and remote head observation. The report cannot invent its own not-yet-known commit SHA; supply that separately in Owner handoff.
3. Final technical run/attempt and **all three** native job IDs/conclusions at one exact technical SHA; native N132 case lines and gate verdicts on both hosts, retained N124–N131 verdicts, architecture counts and the independent AVIF oracle. Include any failed attempts and their real causes, repaired forward.
4. Genuine N132 API facts, strict parser reuse, source SHA binding, canonical-shaped *restricted* locale admission, before-read mismatches, four hostile-log mutations rejected and **zero** public authority. Distinguish a syntactically accepted manifest from a semantically validated publication candidate.
5. A fresh post-R29 two-job `main` baseline with exact job IDs. No claim of product/Platform delivery on `main`; no merge or deployment. Stash/worktree cleanliness and task scratch disposition.
6. Outstanding R04.B/R04.A and full R04.C semantic work, the 20 unapproved Foundation asset decisions, the still-empty production default registry and the explicit future Owner authorization required before public serving/route activation.

**Owner handoff phrase:** `SOLO-WO-118 R21 — manifest-to-visibility bridge COMPLETE (nonpublishing)`, only on verified success. If the exact prerequisites do not hold, STOP; do not downgrade security checks or expand the ledger on your own.

## 8. Explicit authorization boundary

This file is a bounded Owner-issued implementation order for the **manifest-to-typed-visibility bridge only**. Its handoff grants no write permission beyond §3, no approval of the 20 pending Foundation assets, no rights inference, no public-graphic default registration, no source distribution decision, no merge, no public preview/HTTP serving, no release/deployment, no closure of `R04.C`, `R03.C` or Stage 03. Chief Master Architect will independently review the exact report, diffs and native logs before any ACCEPT/STOP disposition or next authorization.
