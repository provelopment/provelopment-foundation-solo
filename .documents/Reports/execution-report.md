# Execution Report

**Work order:** `SOLO-WO-118 R22` — enforce manifest-bound unlisted, entry, landing and same-Page fallback policy before routing (`R04.C3`).
**Bounded outcome:** `R04C3-MANIFEST-BOUND-VISIBILITY-DEPENDENT-ROUTE-POLICY-PREFLIGHT-N133-TWO-HOST-NONPUBLISHING-V1`.

```
status=COMPLETE
manifest_visibility_bridge_implemented=true
manifest_bound_route_visibility_policy_preflight_implemented=true
visibility_dependent_unlisted_rules_verified=true
same_page_missing_translation_policy_preflight_verified=true
page_entry_and_landing_availability_intent_implemented=true
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

**Status: `COMPLETE` for the bounded `R04.C3` nonpublishing pre-route policy outcome only.** One additive, pure, no-I/O preflight now derives a closed route/visibility *policy* from the accepted R21 manifest snapshot and refuses unsafe authored claims before any route resolver, source reader or publisher could exist. It is **not** a route ledger, a validated manifest, a canonical URL or a delivery surface: `R04.C` as a whole, `R04.A`, `R04.B`, `R04.D`, `R03.C` and Stage 03 remain open. Nothing was served, routed, published, released, deployed or merged, and the 20 pending Foundation sources remain undelivered.

## 1. Transactions, SHAs and exact equivalence

| Step | SHA | Validation |
|---|---|---|
| baseline | main R29 `28e803449e407c9c2f0b7ee45c6434a381c165f6`; technical T29 `f02aa20d24c5d7d87fed4d391cb6bc74f6d3b69e` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree in the Owner checkout, four WIP files, empty stash, clean trees, **30** remote heads observed (unrelated Dependabot heads untouched) |
| **A30** (main, ACTIVE only) | `ca743ec14ad42700bd620714aa3150aa4cadd62c`, tree `f07c12ad97fb99ca76435c9b341dd5e43636f9d3`, parent `28e80344` | ACTIVE blob `7e203116d970d8fff361426578d9fe819b4e28ed`, file sha256 `e54f9798b6419b7bd34cd3e0c9ca0c56e7a769b32b39c2a00992b66f07e88bb7`, 24 867 bytes / 153 lines, identical in worktree, index, commit and `origin/main`; exactly one path |
| **T30.0** (branch, ACTIVE only) | `56906f8b88cb1e23bed5ae3ab251eca0c346e961`, tree `3bcf8451263bc65edcf23da10482d799a5d4514d`, parent `f02aa20d` | the **identical** blob; `git diff T29..T30.0` = ACTIVE only; pushed |
| **T30** (branch, implementation) | `b3cbdeb589423b222bd8db9e683005f9b824b2a2`, tree `0782c0d68ac67a82bc871ba6598ce1221184744d`, parent `56906f8b` | **all three native jobs green on the first attempt** (§3); pushed |
| **R30** (main, report only) | this commit (SHA in the Owner handoff) | parent `ca743ec1`, exactly one path |

**No RED attempt exists for this order:** T30 passed all three hosted jobs on attempt 1, and no repair commit (T30.1/T30.2) was needed. Two defects were found and fixed **locally, before committing**, and are disclosed honestly: an intermediate test-block placement that nested the new module inside the previous test body (fixed by relocating the block to the end of the file, with no test weakened or removed), and a helper that inserted a JSON member inside the `representations` array (fixed to insert it before that member). Neither reached a commit or CI.

Changed paths across `T30.0..T30` — exactly the authorised ledger, nothing else:

| Path | Δ | Role |
|---|---|---|
| `crates/solo-adapters/src/manifest_route_visibility_policy.rs` | **+902 / −0 (new)** | the pure pre-route policy preflight, its closed records and refusals |
| `crates/solo-adapters/src/lib.rs` | +8 / −0 | one module line and the public re-exports |
| `crates/solo-adapters/tests/publication_manifest.rs` | **+767 / −0** | additive `mod n133_manifest_route_visibility` only; all 20 pre-existing tests byte-identical |
| `.github/workflows/rust-verify.yml` | +128 / −0 | additive Linux and Windows N133 authentic/mutation gates |
| `.documents/Architecture/ARCHITECTURE.md` | +2 / **−1** | the compact R22 truth note plus **one replaced sentence** of the R20 note that R22 makes stale (disclosed in §2) |

`crates/solo-adapters/src/publication_manifest.rs`, `manifest_visibility_bridge.rs`, every `crates/solo-domain/**` and `crates/solo-application/**` source, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `.gitattributes`, `platform/assets/**` (all 84 files), the MIT notice and provenance, all donor and licensing fixtures, every existing test body, `.documents/WIP/plan-*` and the CLI runtime are **unchanged**: `git diff --name-only T30.0..T30` lists only the five paths above. The catalogue is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` at immutable `1.0.0`, and the N127 aggregate is still `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`.

**Architecture, measured on the real checkout:** `architecture=legal packages=4 targets=39 files=81 edges=93` on both native hosts — exactly one new file, zero new packages, targets or dependency edges, as budgeted.

## 2. Why the accepted parser and the R21 snapshot are reused unchanged

`publication_manifest.rs` and `manifest_visibility_bridge.rs` are **not edited** by this order. The new module consumes exactly one accepted `ManifestVisibilitySnapshotV1` and nothing else: there is no parameter for a ledger, a boolean, a page set, a replacement graph or a manifest path, and no port of any kind. Traversal follows `snapshot.parsed().entity()` in authored containment order, and every representation's effective visibility and provenance are **looked up in the accepted R21 ledger by fully qualified owner** — never recomputed from the authored strings. Because the only input is that snapshot, a caller cannot inject a foreign visibility ledger or another Entity's graph; the already accepted `check_manifest_efs_scope_v1` still refuses a foreign, sibling-renamed or missing ownership candidate, and N133_04 re-proves that at this revision.

**New closed types.** `RepresentationRoutePolicyV1` (exact owner, effective visibility, provenance, `RouteComparisonIntentV1`, authored-`caseMatching` flag, inert alias count), `PageRoutePolicyV1` (exact Page owner, public-representation count, `EntryPathsPolicyV1`, `MissingTranslationPolicyV1`), `LandingPolicyV1` with `LandingAvailabilityV1`, and `ManifestRouteVisibilityPolicyV1`. The comparison intent is closed to `public-insensitive`, `public-sensitive` and `exact-unlisted`; discovery is `index,follow` only for public; robots is an *intent* string, never a header.

**New closed refusal causes** (`ManifestRoutePolicyRefusalKindV1`, code `PREFLIGHT_MANIFEST_ROUTE_POLICY_INVALID`): `unlisted-case-matching`, `unlisted-aliases`, `unlisted-only-entry-paths`, `landing-unknown-target`, `missing-translation-target-unknown`, `missing-translation-target-unlisted`, `missing-translation-target-not-canonical`, `membership-inconsistent`. Every refusal names a safe structural index path (for example `entity.facets[0].sites[0].pages[0].representations[2].route.caseMatching`) and no authored value, route string, source reference or property content. A refusal yields **no** plan, so no partial record set can escape.

**Behaviour enforced.** An effectively **unlisted** representation may author neither `route.caseMatching` (sensitive *or* insensitive) nor any alias, and receives `exact-unlisted`, `noindex,nofollow` and `discoverable=false`; its authored path stays an inert string and unlisted is explicitly **not** private or authenticated. A **public** representation may omit `caseMatching` (future `insensitive`) or author either literal; its aliases stay declared and are never promoted, and no alias or declaration order is used as conflict resolution. A Page with **zero public** representations may not author `entryPaths`; a mixed Page keeps them only as `pending-inert` claims with no locale chosen and no target produced. An authored `base.landing` must name an existing exact qualified Page (and, when it names a locale, a declared representation of that same Page) — otherwise `landing-unknown-target`; its availability is `eligible-exact-page` only when that exact Page has a public representation, otherwise `unavailable-no-public-representation`, and an absent landing stays `absent`. `missingTranslation` is resolved **inside one Page only**: `strict` creates nothing, `default` names the enclosing Site's declared `defaultLocale` only when that declaration on **this** Page is public and otherwise records `default-conditional-not-found` *without* rejecting the Page, and an explicit `locale` must be canonical-shaped for the restricted profile and be a declared, public representation of this same Page — otherwise `missing-translation-target-unknown`, `-unlisted` or `-not-canonical`. A same-named sibling Page's locale is never consulted.

**Deliberate limits.** The locale check mirrors the accepted R21 restricted `canonical-shaped-base` profile (lowercase two/three-letter language, optional Title-cased four-letter script, mandatory uppercase two-letter or three-digit region) and is narrower than valid BCP47. No Unicode 18 / ICU4X / IDNA / NFC normalisation, no percent decoding, no ASCII case folding, no route-collision detection, no host or prefix handling, no canonical URL, redirect, `Vary` or `robots` header, and no locale preference inference exists here; those remain `R04.A`/`R04.B`/`R04.D` and Stage 04 work. Every plan reports `publication_authority=false`, `route_authority=false`, `serves_http=false`, `publishes_derived_output=false`, `release_authority=false` and `deployment_authority=false`, and the type exposes no path, URL, source or authored free text (N133 asserts this against the plan's rendered form).

**Disclosed documentation correction.** The R20 §3.2A note ended by saying only that `R21` bridges authored visibility and that semantic authority is unbuilt; that sentence now also records that `R22` adds the inert pre-route policy gate described above — hence the single deletion in `ARCHITECTURE.md`. No `R04.A`/`R04.B` contract was rewritten and no requirement was relaxed.

## 3. Final technical proof — run `43885719190`, attempt 1, at `b3cbdeb5`

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133363180568` | **success** (69 steps) |
| `verify (windows-2025)` | `133363180876` | **success** (69 steps) |
| `avif-reference-oracle` | `133363181138` | **success** |

Every retained gate is green on both hosts at that one SHA — Ubuntu: N125 (33), N126 (35), N127 (37), N128 (39), N129 (41), N130 (43), N131 (46), N132 (47), **N133 (49)**, N124 (51), architecture on the real checkout (54); Windows: N124 (32), N125 (34), N126 (36), N127 (38), N128 (40), N129 (42), N130 (44), N131 (45), N132 (48), **N133 (50)**, architecture (55). Strict Clippy, locked dependency resolution, `cargo-deny`, format, the byte-reproducible release build, the release CLI smoke and the worker canaries all remain active in those jobs. The 69-step count is the previous 67 plus the two new N133 steps.

**N133 acceptance evidence (real bytes, both hosts).**

| Case | Native run proves |
|---|---|
| **N133_01** | Six representations across two sibling Sites and one mixed Page yield six records in authored order: `public-insensitive` (omitted `caseMatching`), `public-sensitive` (authored), and three `exact-unlisted` rows with `page-default`/`representation-override` provenance, `noindex,nofollow` and `discoverable=false`. The two `home` Pages stay distinct owners with distinct decisions; the mixed Page shows two public representations, the unlisted-only Page zero; `entryPaths` are `pending-inert`; the manifest digest is pinned (`manifest_sha_pinned=true`); every authority flag is false; and the plan's rendered form contains no authored path, base URL, source candidate or label. |
| **N133_02** | Real bytes prove an effectively unlisted `route.caseMatching` is refused as `sensitive` **and** as `insensitive` (location `…representations[2].route.caseMatching`), an unlisted `route.aliases` is refused, and a fully unlisted Page's `entryPaths` is refused — while a mixed Page's entry claims are retained as pending because its own representation is public. `visibility: "internal"` is still refused by the retained strict parser (`InvalidEnumValue`). A refused claim produces no plan, so `zero_partial=true`, and the check has no port, so it refuses `before_read`. |
| **N133_03** | Landing: exact eligible Page, absent, exact-but-fully-unlisted (`unavailable-no-public-representation`, no reroute) and five inexact/foreign targets refused as `landing-unknown-target`, with no locale inferred for a resolved landing. `missingTranslation`: `strict`; `default` naming the public same-Page Site-default locale; `default` conditional not-found both when that locale is unlisted and when it is undeclared on the Page (the Page still builds); explicit public same-Page target; and four explicit refusals — missing, unlisted, non-canonical (`de-de`) and declared only on a same-named sibling Page. The plan emits no URL, redirect, `Vary` or `Location`. |
| **N133_04** | Identical bytes give an identical plan, digest and record set; a label-only change moves the digest while every governed fact stays equal; the accepted R21 scope check still refuses sibling-renamed ownership (`kuta`, `ubud2`) while accepting the exact membership; coverage equals the ledger, Page count and representation count; every exposed token is from a closed set; all six authority flags are false; and neither source bytes nor any output artifact appear. |

**The N133 gate is fail-closed on both shells.** Each host runs `cargo test -p solo-adapters --test publication_manifest --locked n133_ -- --nocapture --test-threads=1`, then requires exactly the four accepted test names, `test result: ok. 4 passed; 0 failed; 0 ignored`, exactly one `n133_evidence case=01..04` marker each, no `FAILED` or `ignored` line, the grounded tokens (`records=6`, `sibling_isolated=true`, `exact_unlisted=3`, `unlisted_aliases=refused`, `unlisted_only_entry_paths=refused`, `landing_unlisted_unavailable=1`, `default_conditional=2`, `explicit_refusals=4`, `foreign_scope=refused`, `manifest_sha_pinned=true`, `locale_profile=restricted`, `zero_partial=true`, `no_source_bytes=true`, `publication=false`, `route=false`, `http=false`, `release=false`, `deployment=false`) and no zeroed digest. Only then does it print `n133_gate authentic=accepted`. It then re-checks **four directly altered copies of that same genuine log** — a deleted case, a forged unlisted acceptance, a forged landing/fallback acceptance, and a forged authority/partial/digest claim — and prints `n133_gate mutation=rejected` only if all four are refused. Supplementary local dry-runs reproduced both verdicts (the Linux gate body end-to-end with real `cargo test`; the Windows body against the genuine captured log under `pwsh`), but they are not presented as native evidence.

## 4. Fresh `main` baseline, cleanliness and scratch

- **Fresh post-R30 two-job `main` baseline:** the `rust-verify` run triggered by this report commit is supplied in the Owner handoff with its run ID, both native job IDs, their success conclusions and its exact SHA — a report cannot know its own not-yet-created SHA, and none is invented here. Its verified parent is A30 `ca743ec1`.
- **`main` carries governance only for this work:** A30 (ACTIVE only) and R30 (report only). No technical code, no `crates/` change and no `platform/assets/` change was merged, and the technical branch remains unmerged.
- **Repository state at handoff:** `main` = A30 `ca743ec1` then R30; technical = T30.0 `56906f8b` → **T30 `b3cbdeb5`**; both pushed, trees clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, **30** remote heads observed. No new branch, tag, PR or worktree; no force-push, rebase, reset, `git clean`, merge or history rewrite.
- **Scratch:** task-owned scratch lived only in ignored, untracked `.temp/` (transaction scripts and the N133 gate dry-run harness with its captured log) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched.

## 5. Remaining dependencies after this outcome

1. **`R04.A`/`R04.B`** — the validated Unicode-18 / ICU4X / BCP47 / IDNA locale binding and the route/base/alias **collision ledger**, including the exact-case unlisted claim contract and the public/unlisted caseless-collision refusal, none of which this order implements or simulates.
2. **The rest of `R04.C`** — whole-manifest semantic validation, real Content JSON/Markdown selection, schema-generated publication authority and the immutable sealed candidate.
3. **Stage 04** — the actual HTTP/locale/unlisted delivery surface with `noindex,nofollow`, which this preflight deliberately does not reach.
4. **Owner decisions still outstanding** — the recorded rights/role grant for the 20 unapproved Foundation canonical sources, explicit `role/slot → approved asset` production default registrations (the table is still empty), and a separate authorization before any public serving, release or deployment.
5. **`R04.D`, `R03.C` and Stage 03** remain open and are not advanced by this report.

**Status `COMPLETE` means only this `R04.C3` nonpublishing pre-route policy outcome.** No route was resolved, no manifest was activated, no graphic was served, published, released or deployed, and no graphic default was registered.



