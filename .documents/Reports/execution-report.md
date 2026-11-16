# Execution Report

**Work order:** `SOLO-WO-118 R18` — one real mixed Entity/Platform whole-candidate transaction with full-owner, two-root, six-format source validation.
**Bounded outcome:** `R03C-MIXED-ENTITY-PLATFORM-ONE-CANDIDATE-CONFINED-SOURCE-INSPECTION-N129-TWO-HOST-V1`.

```
owner_81_assets_approved=true
platform_library_source_installed=true
platform_source_read_verified=true
typed_platform_graphic_baseline_implemented=true
mixed_entity_platform_whole_candidate_verified=true
automatic_product_graphic_defaults_active=false
additional_assets_delivered=false
public_cascade_delivery=false
publication_authority=false
derived_output_publication=false
release_authority=false
r03c_open=true
stage03_open=true
```

**Status: `COMPLETE` for this bounded outcome only.** One composed transaction now builds a closed source-obligation plan from a whole validated candidate, acquires Entity sources through the accepted Entity reader and Platform sources through the accepted pinned Platform reader, and inspects **every** acquired source through the same accepted six-format worker proof — on native Ubuntu and native Windows. Two real defects were found and repaired forward; both are reported below. Nothing was published, derived, released, deployed or merged, no production default was registered, and the 20 pending Foundation sources remain undelivered.

## 1. Transactions, SHAs and changed paths

| Step | SHA | Validation |
|---|---|---|
| baseline | main R25 `787041c6b6711645cb675f0de992abc975038cd1`; technical T25 `cbc47b13f74282e18cfcd9ab7235b048fdbb88e8` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four WIP files, empty stash, clean trees, **30** remote heads (unchanged; the two Dependabot app heads are untouched) |
| **A26** (main, ACTIVE only) | `101c370fc8305d7559180045239e177905dbf0d4`, tree `9db71caa738da1c7a860680c11d78f0b13ea9499`, parent `787041c6` | ACTIVE blob `497db5b17480ac2f9750265494ea8725ccf63647` identical in worktree, index, commit and `origin/main` |
| **T26.0** (branch, ACTIVE only) | `1744509339e0bd529776d56e2224e156e45830a4`, parent `cbc47b13` | same blob; `git diff T25..T26.0` = ACTIVE only |
| **T26** (branch, first implementation) | `e4c55dd6da1c54a819d7bdaf342b1816e08260d8`, tree `70f1cf2da76052761343df2f297a1d94d44d26ca` | **failed its technical run honestly** (§2) |
| **T26.1** (branch, forward repair — the evidence SHA) | `8015a840a6c2c0c55e993f60da28f3931f3706bc`, tree `ac6e76dd562bf4d7d774e4780c57b8d8aa3178bd`, parent `e4c55dd6` | all three technical jobs green |
| **R26** (main, report only) | this commit (SHA in the Owner handoff) | parent `101c370f`, one path |

Changed paths, all inside the ledger:

| Path | Δ | Role |
|---|---|---|
| `crates/solo-application/src/graphic_asset_inventory.rs` | +414 / −2 | the closed mixed obligation plan (types, builder, Entity-subset view) |
| `crates/solo-application/src/graphic_image_inspection.rs` | +~660 / −55 | the one composed mixed driver; the accepted V4 loop body extracted verbatim into `inspect_item_v4` so both compositions share it; `format_of_reference` (repair) |
| `crates/solo-application/src/lib.rs` | +9 / −2 | exports |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | +383 / −0 | the composed outward entry point over the two adopted roots and the closed host-test seam |
| `crates/solo-adapters/src/lib.rs` | +4 / −1 | exports |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | +1003 / −0 | **additive** `mod n129` only; no existing N093–N128 body changed (the diff has zero deletions) |
| `.github/workflows/rust-verify.yml` | +97 / −0 | additive Linux and Windows N129 gates |
| `.documents/Architecture/ARCHITECTURE.md` | +11 / −0 | §6.2I.2 truth note (source-validation scope only) |

`git diff --name-only A26..T26.1 -- platform/assets .gitattributes Cargo.toml Cargo.lock deny.toml crates/solo-cli/src` is **empty**. The immutable `1.0.0` catalogue (`ad8ea718…`, 81 entries), the 84 product files, the worker/supervisor, the codec parsers, `source_read_native.rs`, `property_registry.rs`, `property_resolution.rs`, the R12 catalogue parser and the donor fixtures are byte-unchanged.

**Architecture, measured:** `architecture=legal packages=4 targets=39 files=76 edges=93` on both hosts — unchanged from R17, because R18 added no new file or target (the N129 cases live inside the existing CLI test target and the seam inside existing modules).

## 2. Two real defects found and repaired forward (nothing hidden)

**First technical run `43841183938` at T26 `e4c55dd6` failed.** Ubuntu job `133209957326` and Windows job `133209957151` both failed inside `Test (workspace, all targets, locked)`; the independent oracle `133209956820` passed. Of the five N129 cases, `n129_05` passed on Ubuntu and `n129_01`–`n129_04` failed at their first completion assertion. The actual messages, not a paraphrase:

1. `InspectRefused(... Image(Legacy(UnsupportedFormat)) ..., reference: AssetReferenceV1 { namespace: Platform, reference: "icons/icon-home.svg" })` for cases 01, 02 and 03. **Cause:** the accepted `GraphicImageFormatV3::declared_by` deliberately admits only the `entity` namespace (its own doc comment says a non-Entity namespace is never admitted), so the mixed composition treated every Platform source as an unsupported format. **Repair (T26.1):** a new private `format_of_reference` reuses `accepted_extensions` and drops only that namespace precondition. `declared_by` and every Entity-only caller are untouched, so `inspect_graphic_image_source_set_v4` keeps its exact behaviour — proved by the retained N124–N127 gates on both hosts.
2. `BindingRefused(PlatformGraphicBaselineRefusalV1 { kind: CatalogueUnapproved })` in case 04, where the test expected the reader to refuse a mutated `catalogue_bytes` argument. **Cause:** the accepted Platform reader reads the catalogue **through its own port**, so a mutated argument cannot influence it; the independent pin inside `approved_platform_graphic_binding_v1` caught it instead. **Repair (T26.1):** the case now asserts the refusal the composition really produces — a *stronger* statement, because the pin is enforced by a second independent computation.

Neither repair weakened a check, relaxed a bound or skipped a case.

## 3. Final technical proof — run `43842585265` at `8015a840`

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133214672270` | **success** |
| `verify (windows-2025)` | `133214672019` | **success** |
| `avif-reference-oracle` | `133214671630` | **success** |

Every retained gate in that job was green on both hosts: `n127_gate authentic=accepted`, `n127_gate mutation=rejected`, `n128_gate authentic=accepted`, `n128_gate mutation=rejected`, and the `n124`/`n125`/`n126` mutation verdicts. The N127 aggregate is still `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca` with 81 real confined children, and the architecture line is still `packages=4 targets=39 files=76 edges=93` on both hosts.


**N129 — five cases, identical on both native hosts.** Ubuntu (from `133214672270`), reproduced verbatim by Windows (`133214672019`):

```text
n129_evidence case=01 obligations=4 entity=3 platform=1 distinct_pids=4 platform_path=icons/icon-home.svg platform_mime=image/svg+xml platform_sha=cb742a3b1aa0007ec0c3f26f898e5833c6e67476e136b8eae3c52e2f2c751b5c catalogue_sha=ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7 winner_logo=mark.svg lineage=2 internal_inspected=true publication=false
n129_evidence case=02 obligations=4 internal_inspected=true superseded_inspected=true late_missing=whole-refusal late_corrupt=whole-refusal zero_partial=true publication=false
n129_evidence case=03 platform_path=icons/icon-home.svg entity_path=assets/icon-home.svg platform_sha=cb742a3b1aa0007ec0c3f26f898e5833c6e67476e136b8eae3c52e2f2c751b5c entity_sha=ebbb6ec79c779257941f907235d9670e63d94f1fa2d67f680c693c6b22e510b1 distinct_objects=true entity_fallback=false platform_fallback=false hardlink=refused publication=false
n129_evidence case=04 unapproved_id=refused catalogue_pin=independent unselected_version=refused svg_favicon=refused unsafe_reference=refused ambiguous_format=refused entity_only_entrypoint=preserved publication=false
n129_evidence case=05 legacy_parity=true slot_identity_absent=true deterministic_plan=true entity_only_parity=true whole_candidate_blocked=true zero_partial=true publication=false
```

| Case | What the native run actually proves |
|---|---|
| **N129_01** | One test-only root Platform registration (`icons/icon-home.svg`) plus three authored Entity selections across the five levels: an Entity **PNG** favicon, an Entity **SVG** logo that explicitly supersedes the Platform default at the Site, and an Entity **WebP** social preview authored `internal`. Four obligations planned, four real confined children (`distinct_pids=4`) on each host, exact namespace and logical path per record, the real approved source digest, canonical MIME, the catalogue pin, and the closer supersession governing `graphic.logo` (`winner_logo=mark.svg`, `lineage=2`) while the superseded root default was still inspected. |
| **N129_02** | The same plan proves the complete declared set (superseded and `internal` included), then two late failures — a missing Entity source and a corrupt Entity source at the Representation — each refuse the **whole** candidate with zero partial records. |
| **N129_03** | Entity `assets/icon-home.svg` and Platform `icons/icon-home.svg` are both read from their own roots with different digests (`cb742a3b…` vs `ebbb6ec7…`); an absent Entity source is never served from Platform and vice versa; a hard link making two authored paths one object is refused by the native Entity reader (`hardlink=refused`). |
| **N129_04** | An identifier the approved catalogue does not declare, an unselected library version, a mutated catalogue argument (caught by the binding's independent pin), an approved SVG in the PNG-only favicon role, an Entity reference outside the portable grammar (`con.png`) and a candidate mixing the two authored formats all refuse; the accepted Entity-only entry point keeps its own semantics (`entity_only_entrypoint=preserved`). |
| **N129_05** | A legacy parity candidate carries no slot identity, is inspected through the same driver, and the accepted Entity-only entry point reports the same source digest and format for the same file; the plan is deterministic across two runs; an unrelated-owner failure blocks the whole candidate with no partial set; no record leaves its own namespace root and no record authorizes publication. |

**Gate integrity.** Each host ran a dedicated N129 step after the workspace tests with `--nocapture --test-threads=1`, required all five test names, exactly five `n129_evidence` markers, the `ok. 5 passed` total, the pinned catalogue digest, both namespace paths, `zero_partial=true` and `publication=false`, and then rejected four directly altered copies of the same log: a deleted case, a swapped namespace/wrong-root claim, a flipped `zero_partial`/`publication` claim, and an altered catalogue pin with a swapped obligation count. Verdicts: `n129_gate authentic=accepted` then `n129_gate mutation=rejected` on both hosts. No verdict is printed before the real checks, and there is no `continue-on-error`, no `|| true` masking and no token echo substitute.

**Honest limits of the proof.** No 81-icon campaign was repeated: N127 still proves the entire installed vector set. One approved Platform identity per candidate is exercised, exactly as the order asks, and the reported `distinct_pids=4` are the real PNG, SVG, WebP and Platform-SVG children of that candidate — not a fabricated count. The Linux gate logic was additionally dry-run locally against a synthetic instrumented log (authentic accepted, all four mutations rejected) before it was committed; the real five-case run is the hosted result above.


## 4. What the implementation actually is

**One plan, from authored inputs only.** `collect_mixed_graphic_obligations_v1` orders the obligations deterministically: the root-layer Platform registrations first, then each EFS owner parent-first, and inside every owner its authored legacy values (parity candidates only) followed by its authored slot selections. Each obligation keeps the explicit namespace, the registered role, the slot (absent only for a legacy value, which has none), the qualified declaring owner (absent only for a root registration, which is not an EFS node), the authored ordinal, the authored exposure and the original typed reference. A registration that a closer `suppress` or `supersede` prevents from governing is still planned and still verified, because declaring it was an authored act. A candidate mixing the opt-in slot format with legacy unqualified graphic values is refused (`ambiguous-graphic-format`) rather than double-counted.

**Two roots, two capabilities, no substitution.** The Entity subset is handed to the accepted `read_graphic_asset_bytes_v1` through the accepted `ContentRootCapability`; that gate still refuses any `platform` obligation, so the composition can only pass it Entity work. The Platform subset is handed to the accepted `read_pinned_platform_sources_v1` over a stable unique request set, with the catalogue pin recomputed from the transferred bytes and every source digest compared with the catalogue's own declaration, and the resulting evidence is additionally bound by `approved_platform_graphic_binding_v1`. No external path is ever built from an identifier, and neither root can serve the other namespace.

**One media proof for both namespaces.** The accepted V4 loop body was extracted verbatim into `inspect_item_v4`, so the Entity-only composition and the mixed composition share one implementation of: extension-derived format, role/format compatibility (including the PNG-only favicon rule), non-empty and bounded source, then the PNG/JPEG/WebP/GIF/SVG worker port or the confined AVIF port with its trusted-parent checks. `inspect_graphic_image_source_set_v4` still calls it with the same inputs and produces the same records, which the retained N124 gate proves on both hosts. No second decoder exists and the parent never calls an SVG sanitizer.

**Whole-candidate refusal.** The first failing source returns exactly one typed refusal naming the namespace, role, slot, declaring owner and authored ordinal; no record survives, and there is no partial set, no cross-namespace fallback and no fallback to a root default.

**One known bounded limitation, disclosed in code and in architecture.** A root-layer Platform registration whose approved asset is AVIF is refused (`root-registration-avif-unsupported`) rather than inspected, because the accepted AVIF envelope records an authored EFS obligation as its own provenance and a root registration has none; the composition will not fabricate one. No approved `1.0.0` asset is AVIF, so nothing deliverable is lost today; admitting one later requires a separate order to extend that envelope.

## 5. Rights, assets and what remains open

- The installed library is still exactly the 81 Owner-approved MIT Tabler icons plus the catalogue, the complete MIT notice and the provenance record at `platform/assets/`; the catalogue digest is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` at `platformAssetLibraryVersion: "1.0.0"`. **No catalogue field, entry, product byte or rights flag changed**, and the production registration table is still empty (`automatic_product_graphic_defaults_active=false`).
- The 20 pending canonical sources remain undelivered: `icon-services.svg` (no recorded redistribution grant), the two byte-duplicate aliases (no alias model and no new versioned catalogue), the ten neutral placeholders (no per-file role/redistribution decision) and the seven third-party platform marks (donor record **PARKED — NOT INTEGRATED / Not approved**, conditions P-1…P-4 outstanding). No rights were invented; the R16 142/142 ledger remains the accounting evidence and was not regenerated.
- `public_cascade_delivery=false`, `publication_authority=false`, `derived_output_publication=false`, `release_authority=false`, `additional_assets_delivered=false`, `r03c_open=true`, `stage03_open=true`. No preview, route, derived asset, release, deployment or merge was produced.


## 6. Environment, cleanup and the next dependencies

- Local constraints, disclosed honestly: this machine has no reflink-capable filesystem and no Windows linker. The five N129 cases therefore cannot execute locally — they refuse loudly on the missing `SOLO_TEST_SOURCE_FS_ROOT`, and when pointed at a tmpfs they hit the readers' own `Unsupported` refusal. The plan, resolution, ordering and every refusal path ran locally; only the native transfer could not. Hosted native Linux Btrfs and Windows runs are the authoritative evidence, exactly as the order requires, and no local success is reported.
- Repository state: `main` = A26 `101c370f` then R26 (report only); technical = T26.0 `17445093` → T26 `e4c55dd6` → **T26.1 `8015a840`**; all pushed, both trees clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, `git status --porcelain` and `git diff --check` clean, **30** remote heads. No new branch, tag, PR or worktree; no force-push, rebase or history rewrite. The failed attempt is preserved in history and reported in §2.
- Task scratch lived only in untracked `.temp/` (three transaction scripts, the gate dry-run harness, two patch scripts) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched.

**Next true dependencies after acceptance.** (a) The Owner's recorded rights/role grant for the remaining Foundation original, the placeholders and the marks — without it no further artwork can be installed; (b) a separate public-delivery/privacy/immutable-output order before anything this gate validates may be served, routed or released; (c) the still-open production graphic default decision from R17 — the registration table remains empty by design, and filling it is a product-behaviour change that must be authorised explicitly.

