# Execution Report

**Work order:** `SOLO-WO-118 R17` — complete N127 evidence symmetry and implement the governed Platform-first graphic cascade.
**Bounded required outcome:** `R03C-PLATFORM-FIRST-TYPED-GRAPHIC-BASELINE-SUPERSESSION-N128-TWO-HOST-V1`.

```
owner_81_assets_approved=true
platform_library_source_installed=true
platform_source_read_verified=true
foundation_donor_asset_inventory_paths=142/142
foundation_canonical_assets_accounted=101/101
additional_assets_delivered=false
typed_platform_graphic_baseline_implemented=true
automatic_product_graphic_defaults_active=false
public_cascade_delivery=false
publication_authority=false
derived_output_publication=false
release_authority=false
r03c_open=true
stage03_open=true
```

**Status: `COMPLETE` for this bounded outcome only.** The opt-in typed root-baseline and explicit-supersession slice now exists, is wired to a verified approved catalogue binding, and passes its focused two-host N128 gate; the R16 Ubuntu N127 verdict omission is repaired; every retained gate and the independent AVIF oracle are green at one SHA. Nothing was published, derived, released, deployed or merged, no product default was registered, and the 20 pending Foundation sources remain undelivered.

## 1. Transaction, SHAs and changed paths

| Step | SHA | Validation |
|---|---|---|
| baseline | main R24 `d2cc9786531bfaa6d70fc0d247aa4a3e6ec4fa21`; technical T24 `a8e770f7de22049b89a7f31e353090be24e69cf4` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four WIP files, empty stash, clean trees, **30** remote heads (the 30-head R16 observation is unchanged) |
| **A25** (main, ACTIVE only) | `40485af5c61dcf0fffe8603f1917ad629c101160`, tree `020593ec7e977318fedc644bf3b2b5450fe0deca`, parent `d2cc9786` | ACTIVE blob `fd5bcfd2130850223d0884a8d3230558b9607bef` identical in worktree, index, commit and `origin/main` |
| **T25.0** (branch, ACTIVE only) | `e138880febd0181d8dd4f74142d43aea4e9e32e6`, parent `a8e770f7` | same blob; `git diff T24..T25.0` = ACTIVE only |
| **T25** (branch, the R17 change) | `cbc47b13f74282e18cfcd9ab7235b048fdbb88e8`, tree `17954648df16ddf48e640ef8c9ad41b0b29c342b`, parent `e138880f` | exactly the eight ledgered paths below |
| **R25** (main, report only) | this commit (SHA in the Owner handoff) | parent `40485af5`, one path |

`crates/solo-domain/src/property_registry.rs` and `crates/solo-cli/tests/media_worker_host_smoke.rs` were **not** needed and were **not** touched: every closed type lives in the graphic-specific resolution layer, and no existing N093–N127 body changed.

| Path | Δ | Role |
|---|---|---|
| `crates/solo-domain/src/property_resolution.rs` | +1834 / −2 | the closed typed graphic slice; the two deletions are the widened import list |
| `crates/solo-domain/src/lib.rs` | +14 / −4 | exports for the new seam |
| `crates/solo-domain/tests/property_resolution.rs` | +1005 | additive `mod n128_domain` (six cases, facts 1–7 and 9) |
| `crates/solo-application/src/platform_graphic_baseline.rs` | +181 (new) | inward verified-catalogue binding |
| `crates/solo-application/tests/platform_graphic_baseline.rs` | +363 (new) | additive N128 case 07 (fact 10) |
| `crates/solo-application/src/lib.rs` | +6 | module and re-exports |
| `.github/workflows/rust-verify.yml` | +93 / −0 | **one** Linux N127 verdict echo + two additive N128 gates |
| `.documents/Architecture/ARCHITECTURE.md` | +8 / −2 | the three closed decisions replace R16's open items |

`git diff --name-only T25.0..T25 -- platform/assets .gitattributes crates/solo-cli` is **empty**. The 84 product files, the immutable `1.0.0` catalogue (`ad8ea718…`, 81 entries), the MIT notice, the provenance record, the donor fixtures, the reader, the decoder, the worker/supervisor, the architecture-checker policy and every dependency, lockfile and policy file are byte-unchanged.

**Architecture, measured (not assumed):** `architecture=legal packages=4 targets=39 files=76 edges=93` on Ubuntu and on Windows. R16 reported 38 targets / 74 files: the growth is exactly the one new application module and the one new application integration target, and `packages` and `edges` are unchanged.

## 2. The three closed decisions, as implemented

R16 left three questions open. The order closed them conservatively and the slice implements exactly those answers:

1. **Registration is a separate closed table, never a catalogue field.** `PlatformGraphicBaselineRecordV1` pairs an already-registered graphic `PropertyKey`, one validated slot, one exact `AssetReferenceV1` in namespace `platform` and the declared digest of that asset; `PlatformGraphicBaselineV1::new` binds the table to a verified `ApprovedPlatformGraphicBindingV1` (library version, independently recomputed catalogue digest, approved asset set with media class) and refuses a non-graphic role, a non-Platform reference, a non-`primary` slot on a singular role, an unapproved identifier, a digest that is not the catalogue's own, a media class the role forbids, a duplicate role/slot, a role over its slot capacity, or a table past 64 records. The `1.0.0` catalogue gained nothing. **The production table is empty**, so `automatic_product_graphic_defaults_active=false`.
2. **Slots are explicit lowercase identifiers.** `GraphicSlotIdV1` parses `[a-z][a-z0-9-]{0,62}` only — 1–63 bytes, no leading, trailing or adjacent hyphen — so an uppercase spelling cannot exist and a case collision is impossible rather than detected. `primary` is the only slot a singular role may use; `graphic.background` uses named slots bounded by its registered maximum of four. No index, filename or rendering order ever names a slot, an unqualified add never guesses one, and superseding one slot leaves its siblings and their order untouched.
3. **An unregistered role stays absent.** Only a registered role/slot with an exact approved source carries a root default. `graphic.logo`, `graphic.favicon`, `graphic.banner`, `graphic.background` and `graphic.social-preview` are all unregistered in production, so none of them inherits anything: availability, count, sort order, route and filename are not selection.

**The typed surface** (all in `solo-domain`, pure and I/O-free): `GraphicCascadeInputV1` (validated declarations + baseline + per-owner selection blocks), `GraphicSlotSelectionV1` with the closed `GraphicSlotOperationV1::{Supersede, Append}` and an authored `Exposure`, `OwnerGraphicSelectionsV1`, `ResolvedGraphicOwnerV1`/`ResolvedGraphicRoleV1`/`GraphicSlotResolutionV1` with `GraphicSourceV1` carrying kind, namespace, reference, exposure and an optional `GraphicPlatformBindingV1` (library version, catalogue digest, asset id, asset digest) plus an ordered `GraphicLineageStepV1` chain whose first step is `PlatformRoot` and never an EFS owner, `GraphicCascadeRefusalKind` (17 closed reasons), `GraphicCascadeFailureV1` (graphic or unchanged legacy), and `project_public_graphic_slots_v1`, a filtered view that emits only public winners and no provenance.

**Semantics proved, not asserted:** a registered default appears at the Entity and descends unchanged through Facet, Site, Page and representation without a fictitious Platform EFS owner; the nearer explicit supersede wins and a still-nearer one wins in turn with the full lineage; an `append` into an occupied slot refuses exactly as the legacy additive singular conflict does, and a sibling branch never sees another branch's supersede; `suppress` clears root-layer and inherited values for its key, a valid local value after suppression is eligible, and the literal `none` never acts as a supersede; an internal winner is resolved internally, is absent from every public projection, and is never replaced by a public root default; one late invalid declaration refuses the **whole** candidate so no earlier owner is returned; and a candidate that registers nothing and selects nothing is **byte-identical** to the accepted legacy graph, which is why installing `platform/assets/` cannot change an existing project.

**Unchanged non-graphic semantics:** `location`, `seo.title`, `seo.description` and `contact.email` are resolved by the legacy algorithm itself (`ValidatedPropertyInputV1::resolve_key_from`) inside the new path, exposed as `ResolvedGraphicOwnerV1::non_graphic`, and asserted unchanged by N128 case 01. `crates/solo-domain/src/property_registry.rs` was not modified; the legacy `resolve`, `resolve_all`, refusals, `ResolutionEventV1` provenance and `project_public_properties_v1` keep their exact behaviour and their existing tests.


## 3. N128 on both native hosts — the focused cascade proof

Technical run **`43831041945`**, attempt 1, at `cbc47b13f74282e18cfcd9ab7235b048fdbb88e8`.

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133176180625` | **success** |
| `verify (windows-2025)` | `133176181068` | **success** |
| `avif-reference-oracle` | `133176181009` | **success** |

Seven N128 cases ran inside the workspace suite **and** again in the dedicated gate on both hosts. Each host emitted exactly these lines (identical text on Ubuntu and Windows):

```text
n128_evidence case=01 root=visible_at_entity=true descends=5 unregistered_absent=true non_graphic_unchanged=true binding=1.0.0
n128_evidence case=02 nearer=win still_nearer=win lineage=3 namespace=platform slot=primary append_conflict=refused sibling_isolated=true
n128_evidence case=03 root_suppress=cleared descendant_suppress=final local_after_suppress=eligible none_is_not_supersede=true suppressed_at=recorded
n128_evidence case=04 slot_replaced=1 others_preserved=2 order=stable max=4 append_occupied=refused capacity=refused duplicate_slot=refused wrong_slot=refused
n128_evidence case=05 unregistered=absent wrong_role=refused favicon_svg=refused digest=refused namespace_alias=refused unapproved_asset=refused duplicate=refused slot_grammar=closed filename_inference=false
n128_evidence case=06 internal_hidden=true no_public_reappearance=true late_refusal=whole zero_partial=true legacy_parity=identical defaults_without_optin=false
n128_evidence case=07 binding=exact read=2 digest_matched=true unread_refused=true wrong_digest_refused=true catalogue_pin=independent version=1.0.0 publication_authority=false
```

Gate verdicts, per host: `n128_gate authentic=accepted` then `n128_gate mutation=rejected`. The gate re-derives every token from the instrumented log and rejects four altered copies of it — a forged precedence lineage (`lineage=3`→`1`), a forged privacy/publication claim (`internal_hidden=true`→`false` together with `publication_authority=false`→`true`), a deleted case (`n128_evidence case=04` removed) and a partial result (`ok. 6 passed`→`ok. 5 passed`) — before printing its own verdict. No hardcoded pass echo exists: the authentic branch is reachable only after both test binaries exit zero and every token and the case count are present.

**The exact committed Linux step was dry-run locally** by extracting its `run:` block from `.github/workflows/rust-verify.yml` and executing it with `RUNNER_TEMP` set: `n128_gate authentic=accepted`, `n128_gate mutation=rejected`, exit 0 — so the gate is not merely present but functional.

## 4. The R16 evidence defect is repaired — and every retained gate still runs

The repair is **exactly one line** in the Linux N127 step, after the fourth and final guarded mutated-log check: `echo 'n127_gate mutation=rejected'`. The four checks, the test subject, the tokens and the Windows step are unchanged, and the workflow diff contains **no deletions** (`+93 / −0`).

Both hosts now emit all four verdicts:

| Verdict | Ubuntu `133176180625` | Windows `133176181068` |
|---|---|---|
| `n127_gate authentic=accepted` | present | present |
| `n127_gate mutation=rejected` | **present (was absent in R16)** | present |
| `n128_gate authentic=accepted` | present | present |
| `n128_gate mutation=rejected` | present | present |

Retained and still green at this same SHA on both hosts: N127 (81 real confined children, `aggregate=3c34fe99…`, `natives=11` Linux / `10` Windows), N126, N125, N124, the N102 Windows repeatability canaries, the native filesystem security and Btrfs canaries, the test-failure tripwire, the architecture checker on the real tree, the forbidden-import refusal with guaranteed restore, the byte-reproducible release build and the release CLI smoke evidence, plus the independent AVIF reference oracle.


## 5. The ten required acceptance/refusal facts

| # | Required fact | Case | How it is proved |
|---|---|---|---|
| 1 | Root default visible at the Entity and descending to the representation, no fictitious Platform owner, unregistered roles absent, non-graphic values untouched | 01 | five owners asserted, `lineage[0]` is the root layer, four unregistered roles empty, the inherited SEO value is still present at level 4 |
| 2 | Nearer supersede wins, still-nearer wins, provenance/slot chain correct, and the old additive singular conflict still refuses | 02 | Entity→Facet→Site chain, `lineage=3` with owners and operations, sibling isolation, `SlotAlreadyFilled` |
| 3 | Root suppress clears the baseline; descendant suppress is final; a valid local value after suppress is eligible; `none` is not replacement | 03 | four scenarios, `suppressed_at` recorded, the literal `none` inherits unchanged |
| 4 | Collection: named-slot supersession replaces only that slot, append preserves order and maximum, and wrong-slot, duplicate and same-owner conflicts refuse | 04 | slot order `[surface-main, layer-top, sidebar]`, the untouched slot keeps its default, `SlotAlreadyFilled`, `SlotCapacityExceeded`, `DuplicateGraphicSelection`, `SlotNotPermittedForRole` |
| 5 | Unregistered/wrong-role registration, favicon SVG mismatch, invalid/missing/version-mismatched catalogue or source, Entity/Platform namespace alias and cross-root read attempts refuse before any public claim; no filename infers a role | 05 + 07 | seven distinct refusal kinds, closed slot grammar, and only the registered role filled even with four approved assets bound |
| 6 | Internal and unlisted declarations, including suppressed material, stay in whole-owner validation; no private-to-public reappearance; a malicious late declaration refuses the entire candidate with zero partial result | 06 | the internal winner is absent from the site and representation projections, the ancestor default is intact, and both the whole-graph and the single-owner entry points refuse |
| 7 | A legacy project obtains byte-identical previous graphic and non-graphic semantics; no new default appears merely because `platform/assets/` is installed | 06 | per-owner, per-key reference equality against `resolve_all`, empty slot carriers in parity mode, and the legacy answer unchanged with a registration present |
| 8 | The N128 gate accepts a real terminal log and rejects at least three directly mutated versions; no hardcoded pass echo | §3 | four mutations rejected on both hosts after the authentic acceptance |
| 9 | No public-derived output, release, deployment, new role fallback or automatic brand claim; the Entity reader still refuses `AssetNamespace::Platform` | 05 + retained N127 | a baseline record in the Entity namespace is refused (`BaselineNamespaceAlias`), the production table is empty, `publication_authority=false`, and N127 case 02 still reports `entity_source_reader_refuses_platform=true` on both hosts |
| 10 | A dynamic selector binds to the installed immutable `1.0.0` catalogue's exact identifier/digest and one accepted confined read; a wrong identity or unapproved alias refuses | 07 | real bytes read through `read_pinned_platform_sources_v1` (one catalogue read and exactly two source reads), digest matched per identifier, unread asset refused, substituted digest refused, unpinned catalogue refused, unselected version refused |


## 6. Rights, assets and what remains closed

- The installed library is still exactly the 81 Owner-approved MIT Tabler icons plus the catalogue, the MIT notice and the provenance record at `platform/assets/`; the catalogue digest is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`. **No catalogue field was added, no entry changed and no product byte was touched.**
- The 20 pending canonical sources are still **undelivered**: `icon-services.svg` (no recorded licence), the two byte-identical aliases (no alias policy or new version), the ten neutral placeholders (no per-file redistribution authority or role mapping) and the seven third-party platform marks (the donor record is **PARKED — NOT INTEGRATED / Not approved**, preconditions P-1…P-4 outstanding). No rights were invented; the accepted R16 142/142 ledger remains the accounting evidence.
- `additional_assets_delivered=false`, `automatic_product_graphic_defaults_active=false`, `public_cascade_delivery=false`, `publication_authority=false`, `derived_output_publication=false`, `release_authority=false`, `r03c_open=true`, `stage03_open=true`.

## 7. Environment, cleanup and the next Owner action

- Local constraints, disclosed honestly: this machine has no reflink-capable filesystem and no Windows linker, so the real-filesystem suites (`d2_product`, `n097`, `n116`, `n121`–`n127`, `source_read_native::snapshot_proofs`) cannot run locally. The N128 matrix was therefore executed **natively on both hosted hosts**, and no local snapshot is reported as native success. The N128 dry run of the exact committed Linux step *was* executed locally and is reported as such.
- One failed local gate was repaired forward rather than waived: the first N128 application run refused with `IdentityCollision` because the fake reader reused one physical identity for every source; each source now has its own ordinal and the accepted read succeeds.
- Repository state: `main` = A25 `40485af5` then R25 (report only); technical = T25 `cbc47b13`; both pushed, both clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, `git status --porcelain` and `git diff --check` clean, **30** remote heads (the Dependabot app heads observed since R16 are untouched, unmerged and unmodified). No new branch, tag, PR or worktree was created; no force-push, rebase or history rewrite occurred.
- Task scratch lived only in untracked `.temp/` (transaction scripts, one step extractor and one patch script) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched.

**Shortest next Owner action.** Decide the production graphic defaults: either (a) keep the registration table empty and advance the next committed roadmap stage, or (b) name the exact role/slot → approved-asset pairs to register as the first automatic baseline, together with an explicit adoption decision, because activating it deliberately changes product behaviour. Everything needed to implement (b) now exists and is proved; what is missing is the Owner's decision about *which* approved default belongs to *which* role — a choice that must never be inferred from a filename.

