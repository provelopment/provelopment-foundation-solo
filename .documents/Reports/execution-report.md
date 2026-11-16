# Execution Report

**Work order:** `SOLO-WO-118 R20` — the typed Page/representation visibility preflight, bound to the unchanged unpublished graphic package.
**Bounded outcome:** `R04C1-TYPED-REPRESENTATION-VISIBILITY-GRAPH-BOUND-UNPUBLISHED-GRAPHIC-PREFLIGHT-N131-TWO-HOST-V1`.

```
status=COMPLETE
r19_unpublished_package_accepted=true
page_representation_visibility_preflight_implemented=true
visibility_ledger_complete=true
visibility_bound_to_graphic_candidate=true
n131_two_host_gates_verified=true
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

**Status: `COMPLETE` for the bounded `R04.C1` preflight slice only.** One closed, I/O-free typed visibility ledger now resolves the accepted four-case Page/representation matrix over an already validated EFS graph, and one application preflight binds that ledger to exactly one validated graphic candidate before deriving the display case and calling the accepted R19 package path. Both are proved by five focused cases on native Ubuntu and native Windows. This is **not** `R04.C`: no strict JSON manifest parser, no route-claim validation and no HTTP delivery exist. Nothing was served, routed, published, released, deployed, merged or defaulted, `R03.C` and Stage 03 stay open, and the 20 pending Foundation sources remain undelivered.

## 1. Transactions, SHAs and changed paths

| Step | SHA | Validation |
|---|---|---|
| baseline | main R27 `e82c9d847dc7685797124d2a4aebd7e68cf67329` (report blob `7990efa69a85b26244da8aa771ee16ab70ac7669`); technical T27.1 `07b2052a48d237959b1f79ce6c14fc91b45dc769` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four WIP files, empty stash, clean tracked trees, **30** remote heads observed (the unrelated Dependabot app heads are untouched) |
| **A28** (main, ACTIVE only) | `571b50de931a23f9e5d5a5d2054fdfe66c5970fa`, tree `cb1b844382e05548a458407b3feef8039fd07d25`, parent `e82c9d84` | ACTIVE blob `c29b644976faf18f2e7317be6434afd9678a54a8`, file sha256 `8e47a7fcb4b8d85d9a0dc1bd49cee991a7bdef76d8834c43660d3c5f93bc17d2`, identical in worktree, index, commit and `origin/main`; exactly one path |
| **T28.0** (branch, ACTIVE only) | `3e6e270788c65e6523c49afcd6d75d5ece554c44`, tree `88d0f62326841cf450df5ed7c3dfe1788b2e239f`, parent `07b2052a` | the identical blob; `git diff T27.1..T28.0` = ACTIVE only; pushed |
| **T28** (branch, first implementation) | `8b02f6e4b685ec336873ea47a364eb79a4e083e9`, tree `d2e2557d9b1c260131b84a376a112029e7c6a642`, parent `3e6e2707` | **failed its technical run honestly** (§3); pushed |
| **T28.1** (branch, first forward repair) | `b905f237a9a1288b797fbdc098bfe0cd01eb15b8`, tree `1d7a2f915f8052aabe2661753126b1e228b7c0d7`, parent `8b02f6e4` | **failed again, one case**; pushed |
| **T28.2** (branch, second forward repair — the evidence SHA) | `c9722f20b5f05b0044d39c6dbc066375b3ce3ede`, tree `06a993f67b887b42ac927e4465fcabb4d9864fef`, parent `b905f237` | **all three native jobs green** (§4); pushed |
| **R28** (main, report only) | this commit (SHA in the Owner handoff) | parent `571b50de`, exactly one path |

Changed paths across `T28.0..T28.2`, all inside the ledger and **all additive — zero deletions anywhere**:

| Path | Δ | Role |
|---|---|---|
| `crates/solo-domain/src/representation_visibility.rs` | **+648 / −0 (new)** | the closed visibility model: strict `public|unlisted` parser, Page declarations, immutable effective ledger, closed refusal set, graph-bound builder |
| `crates/solo-domain/src/lib.rs` | +9 / −0 | additive exports |
| `crates/solo-application/src/graphic_visibility_preflight.rs` | **+494 / −0 (new)** | the opt-in preflight over the unchanged R19 preparation, with a closed result-only view |
| `crates/solo-application/src/lib.rs` | +33 / −0 | additive exports plus the bounded `preflight_domain` vocabulary module |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | +1168 / **−0** | additive `mod n131` only: no N093–N130 body or helper changed |
| `.github/workflows/rust-verify.yml` | +129 / **−0** | additive Linux and Windows N131 gates |
| `.documents/Architecture/ARCHITECTURE.md` | +3 / **−0** | §3.2A and §6.2I.3 truth amendments |

`git diff --name-only T28.0..T28.2 -- platform .gitattributes Cargo.toml Cargo.lock deny.toml crates/solo-domain/src/property_registry.rs crates/solo-domain/src/property_resolution.rs crates/solo-application/src/graphic_delivery_preparation.rs crates/solo-application/src/graphic_image_inspection.rs crates/solo-application/src/graphic_asset_inventory.rs crates/solo-adapters` is **empty**. The R19 package, the R18 mixed inspection, the R17 cascade, the accepted legacy registry/resolution, both readers, the native root/worker/decoder, the R12 strict catalogue parser, the architecture checker and policies, the locked dependency graph and every product byte are unchanged. The approved tree is still exactly 84 files and the catalogue digest is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`.

**Architecture, measured:** `architecture=legal packages=4 targets=39 files=79 edges=93` — two new module files, no new target, package or dependency edge (77 → 79 files only).

## 2. What the preflight is, and what it deliberately is not

- **The exact accepted matrix, computed over the graph itself.** One declaration per Page of an already validated `EfsOwnershipGraph`, an optional Page default (`public` when omitted) and zero or more **exact** representation overrides inside that Page produce one effective state per representation: omitted/omitted → `public`; `public`/`unlisted` → `unlisted`; `unlisted`/omitted → `unlisted`; `unlisted`/`public` → `public`. Membership is enumerated from the graph, so a sibling Site, a same-named Page in another Site and a Page in another Entity can never share a declaration, and the ledger covers **every** representation in stable graph order.
- **A closed vocabulary.** `RepresentationVisibilityV1` has exactly two members. The strict parser admits `public` and `unlisted` and refuses `internal`, `private`, `hidden`, an empty token, mixed case and anything unknown with a closed cause. `internal` is a property-value exposure, never a third representation state, and the accepted R17 root-default candidate is untouched.
- **Facts, not capabilities.** Each entry records the closed effective value, its provenance (`page-default` or `representation-override`), `is_discoverable_intent` (true only for `public`) and `robots_intent` (`index,follow` / `noindex,nofollow`). These are typed policy facts: the ledger carries no URL, route, path, origin, hostname, response header or publication authority, and `grants_publication_authority()`/`routes_any_url()` are constant `false`.
- **Bound before anything is opened.** The preflight requires the target to be exactly one localized representation, the ledger's Entity to be the candidate's Entity, and **every Page and representation the candidate declares** to be governed by that ledger. A foreign, partial or misbound ledger refuses with zero readers and zero workers; the accepted composition then runs exactly once, with the display case derived from the ledger alone.
- **No second algorithm.** The wrapper does not re-acquire, re-inspect, re-sanitise, re-encode, re-key, mint a default or copy the R19 staging rules. A `public` representation runs the accepted whole-candidate validation and immutable in-memory staging; an `unlisted` representation runs the **same** complete validation and keeps the accepted conservative empty, nondiscoverable package. A late failure yields no partial entry, no partial payload and no partial preflight.
- **Visibility ≠ discovery ≠ authorization.** `public` is eligibility to be *considered* for future discovery after routes, publication and content authority exist; `unlisted` is not secret and not permanently non-serveable, but it is never automatically discoverable and this order can mint no route for it; `internal` never leaves the property layer. A content key stays an identity, never an address.
- **The production-facing seam is bounded.** `crates/solo-application/src/lib.rs` gains a documented `preflight_domain` module that re-exports, unchanged, the exact domain vocabulary the preflight's public signature speaks. It re-implements nothing, wraps nothing and adds no dependency edge; the architecture guard proves the crate graph is unchanged. The R19 display case remains an explicitly inert test seam and is still labelled as such.

## 3. Two fixture defects found by the native runs, repaired forward

No production defect was found. Both failures were **in my own N131 test fixture**, and both were repaired forward without touching production code, without weakening an assertion and without deleting a case.

**First technical run `43866967937` at T28 `8b02f6e4` failed.** The oracle `133297897003` passed; Ubuntu `133297897005` and Windows `133297897156` both failed inside `Test (workspace, all targets, locked)` (step 17), with `n131_02`, `n131_04` and `n131_05` failing at the shared `prepared()` assertion for a *package* refusal. A local read-only diagnostic of the same candidate showed the cause exactly: the fixture registered `graphic.logo` as the approved Platform **root default** *and* appended a `logo` selection at the representation, which the accepted R17 cascade refuses as `SlotAlreadyFilled { key: GraphicLogo, owner: …bali/home/en-001 }`. **Repair (T28.1):** the N131 candidates now take the logo from the approved root registration itself and author only the favicon and social-preview selections — the accepted R19 shape — so the plan is one root registration plus two authored selections.

**Second technical run `43868157149` at T28.1 `b905f237` failed** with only `n131_05` failing: its deliberately broken candidate still appended a `logo` selection, so the cascade refused the candidate *before* any source was read and the case never reached the missing-source refusal it exists to prove. **Repair (T28.2):** the broken fixture declares only the missing favicon, so the refusal is a genuine late Entity read refusal (`SourceRefused`, zero partial).

Both failed runs remain in history with their job IDs and conclusions.

## 4. Final technical proof — run `43869043157`, attempt 1, at `c9722f20`

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133305019449` | **success** (65 steps) |
| `verify (windows-2025)` | `133305019388` | **success** (65 steps) |
| `avif-reference-oracle` | `133305018923` | **success** |

**Every retained gate is green on both hosts at that one SHA.** Ubuntu: format (15), N125 (33), N126 (35), N127 (37), N128 (39), N129 (41), N130 (43), **N131 (46)**, N124 (47), architecture checker on the real checkout (50). Windows: N124 (32), N125 (34), N126 (36), N127 (38), N128 (40), N129 (42), N130 (44), **N131 (45)**, architecture (51). The independent AVIF oracle passed at the same SHA, and the measured architecture line is `packages=4 targets=39 files=79 edges=93` on both hosts.

**The N131 gate is fail-closed by construction.** Each host runs exactly the five cases with `--nocapture --test-threads=1`, then requires exactly five recognized test names, `test result: ok. 5 passed; 0 failed; 0 ignored`, exactly one `n131_evidence case=01..05` marker per case, the absence of `FAILED`/`ignored` lines, and the grounded tokens `rows=4 default=public override=win`, `sibling_isolated=true`, `unlisted_discoverable=false`, `robots=noindex,nofollow`, `internal_representation=refused`, `internal_hidden=true`, `foreign_ledger=refused`, `partial_ledger=refused`, `before_read=true`, `svg_vector=true`, `source_pin=true`, `zero_partial=true`, `legacy_parity=true`, the catalogue pin, `publication=false`, `route=false`, at least four 64-hex `output_sha=` values with none zeroed and no `unlisted_entries` other than zero. Only then does it print `n131_gate authentic=accepted`. It then re-checks **four directly altered copies of that same genuine log** — a deleted case, a forced `unlisted_discoverable=true`, a flipped `publication=true`/`route=true`, and a forged `zero_partial=false` plus `source_pin=false` — and prints `n131_gate mutation=rejected` only if all four are refused. The step exits non-zero on any failure, so the green step on each host is the certificate; the verdict lines live in the step log at this exact revision. Both gate bodies were dry-run locally before commit, in bash and in PowerShell, against a synthetic log: authentic accepted, all four mutations rejected, exit 0.

**N131 — five cases, genuinely executed on both native hosts.**

| Case | What the native run proves |
|---|---|
| **N131_01** | A four-representation, three-Page, two-sibling-Site graph yields a complete ledger in stable order (`entries=4`), with provenance split exactly `origin_page_default=2` / `origin_override=2`. All four accepted rows hold: `bali/home/en-001` is `public` by Page default, `bali/home/de-DE` is `unlisted` by override, `ubud/home/en-001` is `unlisted` by its own Page default, `bali/about/en-001` is `public` by override. The two same-named `home` Pages stay isolated, a Page the graph does not own is never covered, a representation owner is never a Page cover, and the strict parser admits exactly `public` and `unlisted`. |
| **N131_02** | A real public preflight over the approved library plus one Entity source stages only the two public winners: the `internal` social preview is authored, validated and inspected yet never exported (`internal_hidden=true`, `entries=2`). The sibling `unlisted` representation is `unlisted_discoverable=false` with `robots=noindex,nofollow` while keeping its exact typed identity, so it is not treated as private. `internal`, `private` and `hidden` are refused as representation tokens (`non-manifest-exposure`), as are empty, mixed-case and unknown tokens. No staged identity is an address. |
| **N131_03** | Nine closed domain refusals — a representation carrying a Page declaration, duplicate and missing Page declarations, wrong Entity, wrong Facet, wrong Site, unknown Page, an override outside its own Page and a doubly-overridden representation — plus three application refusals over an **empty** Entity root: a Page target (`target-not-representation`), a complete ledger from another Entity (`ledger-foreign-entity`) and a complete ledger for a smaller graph (`ledger-partial`). Because an empty root would refuse a read, receiving visibility refusals is the proof that binding precedes every reader and worker (`before_read=true`). `target-not-in-ledger` remains a defensive arm: with membership proved first, a complete ledger governs every representation of a validated candidate, so no fixture can reach it without failing a stronger check first. |
| **N131_04** | One real public-intent preflight: the approved Platform asset stays a **vector** (`image/svg+xml`), its source digest equals the catalogue pin (`source_pin=true`), its canonical output digest differs from it, and the content key is `sha256/<output>.svg`; both Entity rasters are byte-exact pass-through with keys ending `.png`/`.webp`; the total is the exact sum; every authority boolean is false. |
| **N131_05** | The same whole candidate under a ledger that makes the target `unlisted` stages **zero** discoverable payloads and zero bytes while still validating every source, and a `public` sibling with the same Page identifier does not replace it (`sibling_public_not_selected=true`). The untouched R19 seam over the same sources produces **identical content keys** (`legacy_parity=true`). A missing Entity source refuses the whole candidate with no partial preflight (`zero_partial=true`). |

## 5. What R20 does **not** establish

- **`R04.C` is not complete.** No production code parses `visibilityDefault` or `visibility` from a manifest; the strict JSON manifest visibility parser is absent (`strict_json_manifest_visibility_parser_implemented=false`) and the parser that exists is still the R12 catalogue reader. The typed preflight is the model that parser must feed.
- **No route and no delivery.** Route-claim validation is absent (`stage04_route_claim_validation_implemented=false`), exact-case unlisted HTTP delivery is absent (`stage04_exact_unlisted_http_delivery=false`), and this order produced no URL, handler, server, route manifest, canonical tag, sitemap, `robots` file, response header or discovery document.
- **No publication or release authority.** `public_cascade_delivery=false`, `publication_authority=false`, `public_route_authority=false`, `derived_output_publication=false`, `release_authority=false`. The package is still in memory only, and holding a `public` preflight is a prepublication fact, not permission to serve.
- **No new product behaviour.** `automatic_product_graphic_defaults_active=false`: the production registration table is still empty, no role was bound to an approved asset, and no filename, locale, route or hostname was allowed to imply one.
- **`R04.A`, `R04.D`, `R03.C` and Stage 03 remain open** (`r03c_open=true`, `stage03_open=true`).

## 6. Rights, assets and what remains open

- The installed library is still exactly the 81 Owner-approved MIT Tabler icons plus the catalogue, the complete MIT notice and the provenance record (84 files), with catalogue SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` at `platformAssetLibraryVersion: "1.0.0"`. **No catalogue field, entry, product byte or rights flag changed**, and no additional artwork was installed or authorized (`additional_assets_delivered=false`).
- The R16 Foundation ledger remains 142/142 paths (101 canonical, 20 outstanding), unchanged and not regenerated: the Provelopment original `icon-services.svg` (no recorded redistribution grant), two byte-identical aliases (no alias model and no new versioned catalogue), ten neutral placeholders (no per-file role or redistribution decision) and seven parked third-party platform marks (**PARKED — NOT INTEGRATED / Not approved**, conditions P-1…P-4 outstanding). Public visibility of the donor repository is not redistribution approval.

## 7. Fresh `main` baseline, environment and cleanup

- **Fresh `main` two-host baseline after R28:** the `rust-verify` run triggered by this report commit is reported in the Owner handoff with its run and job IDs and its exact SHA — a report cannot know its own future SHA, so it is delivered in chat instead of being fabricated here. As an already-verified `main` baseline for this report's parent, the A28 push produced two green `rust-verify` runs at `571b50de`: `43862373307` (`verify (ubuntu-24.04)` `133282190766` **success**, `verify (windows-2025)` `133282190432` **success**) and `43870440392` (`verify (ubuntu-24.04)` `133309873751` **success**, `verify (windows-2025)` `133309873451` **success**). `main` carries only governance: A28 (ACTIVE only) and R28 (report only). The technical branch was never merged, so `main` still holds no product bytes, no `platform/assets/` and no crate code.
- Local constraints, disclosed honestly: this machine has neither a reflink-capable filesystem nor a Windows linker, so the host-dependent cases cannot execute locally — they refuse loudly on the missing `SOLO_TEST_SOURCE_FS_ROOT`, exactly as the retained N129/N130 cases do. Everything that is pure (the whole ledger, the four rows, nine domain refusal arms and three application refusal arms, the strict token parser) was executed locally and is green; the two real-source cases, the confined workers and every filesystem proof ran **only** on the hosted native hosts, and no local result is presented as native evidence. Both N131 gate bodies were dry-run locally in bash and PowerShell before commit.
- Repository state: `main` = A28 `571b50de` then R28 (report only); technical = T28.0 `3e6e2707` → T28 `8b02f6e4` → T28.1 `b905f237` → **T28.2 `c9722f20`**; all pushed, trees clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, **30** remote heads observed (no quota claimed). No new branch, tag, PR or worktree; no force-push, rebase, reset, `git clean`, merge or history rewrite. Both failed runs are preserved and reported in §3.
- Task scratch lived only in ignored, untracked `.temp/` (transaction scripts, the two gate dry-run harnesses, their synthetic logs and one temporary diagnostic test that was removed before the final commit) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched and no unknown or Owner-owned material was removed.

**Next true dependencies after acceptance.**

1. **The actual Stage 04 vertical slice**, in bounded stages: the strict JSON manifest parser that produces these typed declarations, then route-claim validation (including exact-case unlisted claims), then a production-mode HTTP surface with `noindex, nofollow` and no automatic discovery — before any public serving is enabled.
2. **The Owner's recorded rights/role grant for the 20 pending Foundation sources**, and separately the exact production default registrations (`role/slot → approved asset`). Neither is conferred by this order, and `automatic_product_graphic_defaults_active` stays `false` until the Owner names those pairs explicitly.
3. **`R03.C` / Stage 03 closure** remains a separate, later decision; R20 does not move it.




