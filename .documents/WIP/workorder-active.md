# SOLO-WO-118 R17 — COMPLETE N127 EVIDENCE SYMMETRY AND IMPLEMENT A GOVERNED PLATFORM-FIRST GRAPHIC CASCADE

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Sole Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `.documents/WIP/workorder-active.md` in that checkout  
**Bounded required outcome:** `R03C-PLATFORM-FIRST-TYPED-GRAPHIC-BASELINE-SUPERSESSION-N128-TWO-HOST-V1`.

## 1. Authority, pins and disposition

The Chief Master Architect independently **ACCEPTS R16** for its explicitly bounded result, NOT for an already-running cascade or for rights in other Foundation artwork. Canonical `main` is **R24 `d2cc9786531bfaa6d70fc0d247aa4a3e6ec4fa21`**, report-only; technical branch `solo-wo-118-avif-v4-r01` is **T24 `a8e770f7de22049b89a7f31e353090be24e69cf4`**. Technical run **43822728379** attempt 1: Ubuntu **133149298969** success, Windows **133149299183** success, AVIF reference oracle **133149299224** success. R24 main baseline **43824517031**: Ubuntu **133155025354**, Windows **133155025128**, both success. Technical architecture: **packages=4 targets=38 files=74 edges=93**. The existing installed Platform library consists of precisely 81 Owner-approved MIT Tabler SVG icons plus three pinned metadata files at `platform/assets/` (84 product files), version `1.0.0`; catalogue SHA-256 **`ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`**. All bytes are immutable for this work order.

R16 independently reconciled the Foundation donor at commit **`fa599973a88ff77f6a8e012f3266c3c139b52c17`**: **142/142 artwork paths** = 101 canonical (84 icons, ten role placeholders, seven social/platform marks) + 41 mirrors/fixtures. Exactly **81** icons are approved and installed; **20 canonical sources remain pending**: `icon-services.svg`, two duplicate aliases, ten neutral placeholders and seven third-party social marks. R16 decision record: `.documents/Reports/foundation-donor-full-artwork-and-cascade-decision.md`; full ledger: `.documents/Reports/foundation-donor-full-artwork-inventory.json`. This is exhaustive **accounting**, NOT authorization to ship the other 20. The donor social-mark provenance expressly says **PARKED — NOT INTEGRATED / Not approved**, with conditions P-1…P-4. Preserve that distinction. The Owner's prior instruction to include all publicly available Foundation art defines the desired delivery scope, not an implicit third-party trademark licence or permission to invent missing rights.

The Owner also explicitly directed that Solo's global Platform library sit **above the Entity** as the root of graphic cascading, and that a closer **explicitly superseding** selection win, with valid suppression and no resurrection after an invalid closer declaration. R16 records that as a *future target* in Architecture §6.2I.1. **This R17 is the Owner-issued implementation order for the typed, safe resolution seam.** It is **not** permission to turn arbitrary neutral icons into business logos, site favicons, banners or backgrounds or to release/publicly serve assets. It is also not approval to rewrite the already accepted non-graphic cascade.

**One known evidence-format defect:** Ubuntu N127 executes all four forged-log checks successfully but omits the literal `n127_gate mutation=rejected` after the final check; Windows emits it (twice in the currently tracked workflow). This is a missing terminal echo, not a security defect. Correct it minimally here; do not re-engineer N127 or repeat historical 81-icon negative matrices.

## 2. Exact transaction / stop discipline

1. Preflight one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, no dirty tracked files, no unexpected stash, four Owner WIP files, and the **actual current** remote-head count. A pre-existing new Dependabot head is a disclosed environmental observation, not grounds for rewriting the baseline. Record the 30-head R16 observation and any honest later difference.
2. On **main** from R24, commit only this entire issued order to `.documents/WIP/workorder-active.md` as **A25**. Check worktree, index, commit and `origin/main` contain one identical ACTIVE blob. Do not modify the canonical report in A25.
3. On the existing technical branch from T24, restore the exact ACTIVE blob from A25 and commit **T25.0** (ACTIVE only). Make allowed source, test, architecture and CI changes on that technical branch only. Commit final **T25** and push this branch. Do not merge, rebase, force-push, create a second worktree, create a new technical branch, change tags or releases, or destroy Owner scratch.
4. Require one fresh **all-green technical** workflow run at the exact T25 SHA: native Ubuntu, native Windows, independent AVIF reference oracle; require the new **N128** focused evidence and refusal gate on both native hosts. Do not equate a test-body result with a dedicated downstream gate. Fail closed and STOP if the order cannot be satisfied inside this ledger. Keep failed attempts and repairs transparent.
5. On **main** (A25 parent), commit exactly `.documents/Reports/execution-report.md` as report-only **R25**. Include SHA, branch, differences, raw job IDs, real log evidence, mutations, architecture counts, provenance and every still-open flag. Obtain a fresh two-host main baseline at R25. No technical source, Platform product asset or code from branch is merged onto `main` by this report.

## 3. Explicit closed technical choices resolving R16's three open questions

These are the **Architect's deliberately conservative implementation choices for this work order**, derived from the Owner's governing direction. They are not misrepresented as having been individually dictated by the Owner.

**3.1 Registration separate from pinned catalogue.** Define a small closed, **role-keyed Platform graphic baseline registration table** in the typed property-resolution contract. Each record has one already-registered graphic `PropertyKey`, one validated slot ID, one exact `AssetReferenceV1` explicitly in namespace `platform`, and a provenance/pin association to the approved `platformAssetLibraryVersion: "1.0.0"` and the verified catalogue. **Do not add a `default` field to the 1.0.0 JSON catalogue, alter its 81 entries or change the `platform/assets/` product tree.** Table registration is trusted application/installation configuration, never inferred from filenames, the inventory count, public URLs, owner location, sorted order or asset presence. Duplicate role/slot or case collision refuses. A default is eligible only when the catalogue and actual separate Platform read validation prove the exact ID, format, digest and role compatibility. No implicit use of the Entity root for Platform sources. Pass a verified pure approved-set membership/binding inward; do not use `rightsReviewed=true` from the old inert R12 catalogue or claim an asset read itself grants publication authority.

**3.2 Named slots, not ordinals.** A slot ID is a stable, explicit lowercase ASCII technical identifier: **`[a-z][a-z0-9-]{0,62}`** (1–63 bytes; no leading/trailing hyphen or adjacent hyphens). **`primary`** is the only permitted slot for a singular graphic role. A collection such as backgrounds uses one or more explicitly registered *named* slots, e.g. `surface-main`; an unqualified add does not guess which slot to replace. Invalid, duplicate or cross-role slot keys refuse. Do not infer slot identities from array indices, filenames or rendering order, and never reorder unrelated collection slots. This schema is a new opt-in typed resolution slice: do not silently reinterpret accepted historical declarations as `supersede`.

**3.3 No implicit default for an unregistered role.** Registered role/slot **with an exact approved source** can supply one root-level default; unregistered roles and unregistered slots remain **absent**, not assigned an arbitrary icon. This is the only interpretation compatible with the Owner's requirement for responsible generic reuse and the explicit role/mime contract (`graphic.favicon` is PNG only, whereas the 81 initial icons are SVG). A library asset being **available** does not mean it is **selected**. No default is invented for `graphic.logo`, `graphic.favicon`, `graphic.banner`, `graphic.background` or `graphic.social-preview`. The table may start **empty in the production configuration** until specific suitable default artwork/role mappings are cleared; inject explicit test-only registrations to prove the precedence semantics. Do not claim a non-empty global default set exists when it does not.

**3.4 Explicit closer supersession.** Introduce a *closed typed operation* in the graphic-specific authoring/resolution layer to **supersede one named role/slot**, never an implicit "last-write-wins" conversion. An explicitly superseding closer Entity/Facet/Site/Page/representation selection takes precedence over the root default or inherited slot for the same role/slot; a still-closer explicit supersession takes precedence in turn. Additive contributions continue to append according to the currently registered cardinality/slot rules; a second conflicting same-owner selection and singular inherited+local **without** an explicit supersession remain refusals. Explicit suppression discards inherited/default contributions within its role scope, then permits valid explicitly authored local entries. `currentContext: none` is not silently a supersede. **Do not change `location`, SEO, contact, direct media, routing or EFS ownership semantics.** Keep legacy V1 APIs functioning for existing callers; expose an unmistakably named new cascade entry point and do not silently flip every existing caller into migration mode.

**3.5 Privacy and invalidity.** A selected/declared Platform/Entity asset is never inferred from basename, a route or a role; no automatic fallback on missing, wrong-class, unsafe, overlong, wrong-version or invalid bytes. A closer invalid or ineligible selection is a **whole-candidate refusal**, not a reason to revive a Platform default. Validate **every authored obligation**, including suppressed, internal and unlisted owners, before a successful candidate result. For a private/internal closer value, do not replace it with a public Platform baseline at projection time; separately refuse unsafe public use while preserving valid internal resolution semantics. Visibility is authoritative, not a display preference. A Platform source can remain a root-layer value *without* becoming an Entity ancestor or a new EFS tier. Do not turn the Platform library itself into a publisher.

**3.6 Safety bounds / migration.** Keep a finite max table size, bounded validated IDs and deterministic traversal, and output typed internal provenance (root registration/owner, exact slot and namespace, source digest or catalogue binding, ordered supersession lineage). No raw donor paths, private source bytes or internal/provenance records leak to public projections. The *legacy* additive graph resolver must remain available unchanged for existing callers; introduce and prove the graphic-enabled path and a conscious selection/migration flag. R17 may truthfully report `typed_platform_graphic_baseline_implemented=true` if the new typed path operates and passes, but `automatic_product_graphic_defaults_active=false` if the product default table is empty, `public_cascade_delivery=false` and `release_authority=false` until a later explicit adoption. Use one small authoritative representation of registration, not duplicate ad-hoc lookup/alias registries.

## 4. Write ledger (strict)

Allowed technical-branch paths after T25.0:

```
.documents/WIP/workorder-active.md                       # ACTIVE only from A25
.documents/Architecture/ARCHITECTURE.md                   # replace R16's open schema items with these 3 precise decisions; retain migration truth
.github/workflows/rust-verify.yml                         # ONE Linux N127 terminal echo + additive focused N128 native gates
crates/solo-domain/src/property_registry.rs               # ONLY if a typed graphic supersession operation genuinely needs a closed type here
crates/solo-domain/src/property_resolution.rs             # additive new graphic-enabled path; no non-graphic D08 regressions
crates/solo-domain/src/lib.rs                              # exports for new typed seam
crates/solo-domain/tests/property_resolution.rs            # additive N128-domain acceptance cases only
crates/solo-application/src/lib.rs                          # exports or narrowly necessary verified catalogue binding
crates/solo-application/src/platform_graphic_baseline.rs   # NEW only if application boundary needs an actual approved catalogue/role binding
crates/solo-application/tests/platform_graphic_baseline.rs # NEW only with that module
crates/solo-cli/tests/media_worker_host_smoke.rs           # additive N128 cross-root/whole-owner tests only if necessary
.documents/Reports/execution-report.md                    # R25 main only; NEVER include in T25
```

If implementation can be smaller, **omit optional files**; do not add files to fill the ledger. If an existing parser needs an alteration, or a cross-crate feature requires another file, **STOP and report the missing authorization**, rather than changing a source/file outside this ledger or silently weakening a contract. No Cargo manifests, lockfiles, decoder, sanitizer, worker/supervisor, native source reader, Platform reader, existing N093–N127 test body, existing N127 mutation check, architecture checker policy, `platform/assets/`, fixtures, Foundation source artwork, `.gitattributes` or candidate catalogue changes. No new external dependency, `unsafe` or format support. Report actual graph counts after any new file/target, never bake in R16's 38/74/93 as a false expected result.

**N127 one-line repair** — after the *fourth and final* guarded mutated-log check of the **Linux** N127 step, add exactly:

```bash
echo 'n127_gate mutation=rejected'
```

Do not print it before the checks, add a fake marker to test stdout, weaken the checks or change the N127 test subject. The Windows step already prints the required verdict, so no Windows change is needed to satisfy this order.

## 5. N128 proof proportional to actual risk

The new risk is **how the root baseline affects inherited selection, privacy and source identity**, not whether 81 previously validated SVGs can be decoded again. **Do not repeat N126/N127's 81× worker matrix or invent a per-icon adversarial campaign for a pure cascade change.** Keep the established native N127 and AVIF gates unchanged; build a **focused** new N128 matrix on both native hosts with real typed domain/application composition and one or a few existing pinned Platform sources only where actual read binding is needed. Synthetic values are permitted to test pure state transitions but never masquerade as approved live Platform source evidence.

**Minimum ten distinct acceptance/refusal facts (combine into ~4–6 well-named tests, not 81):**

1. Registered root default visible at Entity and descends through Facet/Site/Page/representation without a fictitious Platform EFS owner; unregistered graphic role absent, all non-graphic D08 values untouched.
2. Explicit nearer supersede wins for a singular role, then a still-nearer supersede wins; provenance/namespace/slot chain correct; the old additive singular conflict still refuses when supersede is not authored.
3. Root graphic suppress clears baseline; descendant suppress remains final; an explicit valid local value after suppress is eligible, not a fallthrough; `none` is not replacement.
4. Collection: named-slot supersession replaces only that slot, append preserves other slots/order and maximum; wrong-slot, duplicate slot and same-owner conflicts refuse deterministically.
5. Unregistered/wrong-role registration; favicon SVG mismatch; invalid/missing/version-mismatched catalogue or source; Entity/Platform namespace alias and cross-root read attempts **refuse before any public claim**. Never use a filename to infer a registered role.
6. Internal and unlisted authored declarations, including subsequently suppressed material, remain in whole-owner source validation; no private-to-public reappearance when hidden; a malicious late declaration refuses the **entire** candidate, zero earlier partial result.
7. A legacy (no opt-in table or no new-operation caller) project obtains **byte-identical previous graphic and non-graphic resolution semantics**; no "new defaults" appear merely because `platform/assets/` is installed.
8. N128 authenticity gate accepts a real terminal log for each host and **rejects at least three** directly mutated versions: forged winning precedence/slot, forged public/internal or fallback claim, and missing/failed/partial case. No hardcoded pass echo unless the actual instrumented test plus rejection guard passes.
9. No public-derived output, release, deployment, new role fallback or automatic brand identity claim from a successful test. The separate existing Entity reader still refuses `AssetNamespace::Platform`; use only the new approved Platform root for approved Platform sources.
10. Where a dynamic selector picks a verified Platform source, bind it to the installed immutable 1.0.0 catalogue's **exact** identifier/digest and one accepted confined read; wrong identity or unapproved alias is refused. R17 is not another artwork rights approval.

Both native hosts must pass the focused N128 tests **and** their authentic/mutated-log checks, every retained N093–N127 gate (including the repaired Linux N127 `mutation=rejected` marker), the architecture and safe-code checks, full workspace tests and the independent AVIF oracle. Do not skip test suites, reduce security posture, suppress CI failures or require N128 to start 81 new children. If a Linux-only native Btrfs feature is unavailable locally, disclose and use native CI as authoritative without fake local success. Maintain exact, minimal reporting of PIDs/roots where applicable, no private path disclosures.

## 6. Asset completeness without inventing redistribution rights

The comprehensive Foundation donor ledger already exists and is **accepted as accounting evidence**. Do not fetch or install the 20 pending sources, increase the `1.0.0` catalogue, create an alias table, move mirrors/test fixtures into product, use social marks, or assume their legal/trademark clearance in R17. Keep these explicit **separate Owner gates**:

- `icon-services.svg`: Provelopment-original artwork, requires a specific ownership/licensing grant for Solo redistribution.
- `icon-status-info.svg`, `icon-volunteer.svg`: identical bytes to approved representatives; require an explicit **catalogue alias policy** and new version. No duplicate digest within `1.0.0`.
- Ten neutral role placeholders: require per-file redistribution authority and role-appropriate mapping. They are *not* brand identity approvals. They are the likely future source of eligible default role images once cleared.
- Seven official third-party platform/social marks: donor explicitly parks them; trademark/use-basis conditions P-1…P-4 remain outstanding. A repo being public or labelled "royalty-free" does not supersede the actual brand-owner terms. Avoid importing into product or an unrestricted public code repository before authorization.

**No semantic shortcut:** the Owner's high-level instruction to bring ALL public Foundation art is a desired end state, and the exact ledger accounts for each path. Releasing mark bytes or presenting a placeholder as an approved brand to satisfy a total would violate the accepted boundary. A later Owner decision may clear a named subset or exclude a mark, with honest status.

## 7. Reporting and stopping conditions

The final R25 report must specify the **actual** product state and exact flags:

```
owner_81_assets_approved=true
platform_library_source_installed=true
platform_source_read_verified=true
foundation_donor_asset_inventory_paths=142/142
foundation_canonical_assets_accounted=101/101
additional_assets_delivered=false
typed_platform_graphic_baseline_implemented=<measured true/false>
automatic_product_graphic_defaults_active=false
public_cascade_delivery=false
publication_authority=false
derived_output_publication=false
release_authority=false
r03c_open=true
stage03_open=true
```

Record that the old `1.0.0` catalogue remains unchanged and that the R17 default registration is a **separate typed contract**, not a new catalogue field. Include at least the legacy-vs-new cascade tests, all ten required acceptance facts, the representative pinned source check, pre/post technical and report-only diffs, real N127 Linux and Windows marker evidence, N128 authentic and adversarial gate result, raw CI job IDs, exact native architecture, environmental constraints and the explicit remaining rights/role decision. If the new path is only pure/type-checked but not integrated, say so and report a STOP rather than COMPLETE. No claims of published website output or end-user default art. R03.C and Stage 03 remain **OPEN**.

**Success condition:** the opt-in typed root-baseline + explicit supersession path is implemented, source identity remains separate and verified, non-graphic semantics are unchanged, the targeted native N128 gates pass on both hosts, and the existing N127 Linux verdict prints only after genuine mutation rejection. **Failure condition:** any out-of-ledger parser/schema change required, unverified binary/root evidence, regression in old gates, or new public/release claim → STOP and hand back the smallest precise next authorization; do not improvise.
