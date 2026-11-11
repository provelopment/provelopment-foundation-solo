# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-063 R01` — recover, validate and conditionally integrate the R03.C1 authored graphic-obligation inventory on the accepted patched `rustix 1.1.6` baseline.
**Matching handoff:** `SOLO-WO-063 R01`.
**Session date:** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo` (one worktree, sole `origin`).
**Disposition:** **COMPLETE** for this bounded R03.C1 slice. The five authorized paths are integrated on `main` with fresh **Ubuntu/Windows** green runs at the recovery-branch SHA, the fast-forward substantive `main` SHA and the final report SHA. This is **not** an acceptance of R03.C or Stage 03 and **no self-ACCEPT** is claimed.

## 1. Baseline, issuance and branch ledger

| Item | Value |
| --- | --- |
| Verified starting `main` == `origin/main` | `084161d601577d62e85df831542e83de82628639`, tree `9cb58c5fad5f968294c5fcd3519cb62df647715f` |
| Baseline ACTIVE / NEXT / ROADMAP / OUTLOOK | `ff903266…` / `92fa562b…` / `31a0ba2e…` / `cb9c7a61…` |
| Baseline adapter manifest / lock / policy / workflow / ignore | `6109224cd3e3e89487884631f2b07c68ad8149c3` (`rustix = "=1.1.6"`) / `2f9e53576df117f2020ae064d6973304f20833f1` / `3063e864…` / `eb78dad2…` / `f45dd495…` |
| **Issuance commit** (ACTIVE only) | **`fdac2bfaf9fb03ecc3ac68ffa6e06378bbd3edd4`**, tree `b9d76df6006dc8a4fbba458bca6c1443b4cac65c`, ACTIVE blob `4ec5c24511a9420d7ed782f7316e9d14b0b091da` |
| **Recovery branch** | `solo-wo-063-r01`, created at the issuance SHA |
| **Cherry-pick** of `4371af21dce67f33cf1df0384738bf353ca80fc6` | **`e09b761ec9345ac32a247f5a410466afc07aca28`**, tree `3273b69b14537f5beb22aa9b2a4b86d1104fceca`, single parent `fdac2baf…` |
| Correction 1 (strict-lint) | **`cbe3ff7853a159b84758c3e2bdf90d646d14fefa`**, tree `894355c1c0428a4004e596c28ef912066c8abbda` |
| Correction 2 (fixture cardinality) | **`42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1`**, tree `e3ba61cffb24e13451f9b8a732b09282cdd5bf27` |
| **Substantive `main`** (fast-forward only) | `42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1`, tree `e3ba61cffb24e13451f9b8a732b09282cdd5bf27`, single parent — no merge commit |

The cherry-pick applied cleanly with no conflict, no sequencer residue and exactly the five authorized paths. The original branch `solo-wo-061-r01` was never checked out, rewritten, fast-forwarded or pruned.

## 2. Five-path recovery and correction mapping

| Path | Original blob | Recovered blob | Final blob | Correction |
| --- | --- | --- | --- | --- |
| `crates/solo-application/src/graphic_asset_inventory.rs` | `7de9bc7399854b2059cbec17ee23c1e1ae637159` | **identical** | `7de9bc73…` | none |
| `crates/solo-application/src/lib.rs` | `1f5115569b0c65bef3e9f44abd6a4a022f9fd081` | **identical** | `1f511556…` | none |
| `crates/solo-adapters/tests/property_resolution.rs` | `332372ef2f5d9ae290477ea6836f4157d5c84cd6` | **identical** | `332372ef…` | none |
| `crates/solo-domain/tests/property_resolution.rs` | `5fea4e4748f493bb43f7f80189081c11bb75e162` | **identical** | `5fea4e47…` | none |
| `crates/solo-application/tests/graphic_asset_inventory.rs` | `49b0038d49277862329b00d9c8da9fdd172467a3` | **identical** | `1a666397d60c12f4234196b7c7c530d0e816639e` | Corrections 1 and 2 |

The recovered tree differs from the original source tree **only** in the four baseline files (report, ACTIVE, `Cargo.lock`, adapter manifest) — the patched dependency baseline plus the new authority/report. No dependency, asset, catalogue, licence or publication path entered the recovery.

**Correction 1 — `R03C1-10` (strict lint).** The mandatory gate is `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`. Before: `assert!(!GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY, "…")` produced `error: this assertion has a constant value` (`-D clippy::assertions-on-constants`) on both hosts. After: the constant is bound to `inventory_grants_publication` and then asserted false — the accepted `R03A-02` idiom already used in `crates/solo-application/tests/property_resolution.rs:127`. The runtime assertion is unchanged in meaning and still fails if the inventory ever becomes publication authority; nothing was deleted, skipped or relaxed, and the constant remains `false`.

**Correction 2 — `R03C1-01` / `R03C1-06` (fixture cardinality).** Before: the `de-DE` and `jakarta`/`en-001` declarations in `R03C1-01`, and both Page declarations in `R03C1-06`, used `inherit` while also authoring a second value for a **singular** role, so whole-graph validation refused the fixture: `PropertyResolutionRefusalV1 { kind: CardinalityExceeded, owner: example/main/bali/home/de-DE, property: Some(GraphicLogo) }` and `… owner: example/main/bali/home, property: Some(GraphicLogo) }`. After: those four declarations keep exactly the same owner, role, value, local index and exposure and switch only `inherit` → `suppress`, the legal suppress-plus-local replacement form that `R03C1-05`/`R03C1-07` require to remain legal. Every asserted obligation, ordering, count and row is unchanged, and **no cardinality rule, production file or accepted refusal was modified**. These were never-executed fixture defects, not evidence of a product defect.

## 3. CI evidence — issuance, recovery branch and substantive `main`

| Phase | Run | Revision | Result |
| --- | --- | --- | --- |
| Issuance (`main` push) | **`43003869297`** | `fdac2bfa` | success — jobs `130466389364` (Ubuntu) / `130466389827` (Windows), 33 steps each, 0 failures, 9/14 counterpart skips, 370/372 tests, RustSec `31cf5a86…` |
| Recovery branch, attempt 1 | `43005468178` | `e09b761e` | **failure, triaged and corrected** — both hosts failed only `Strict lint (warnings are errors)`; 21/18 later steps skipped |
| Recovery branch, attempt 2 | `43006480925` | `cbe3ff78` | **failure, triaged and corrected** — both hosts failed only `Test (workspace, all targets, locked)` with the `r03c1_01`/`r03c1_06` fixture refusals |
| **Recovery branch, final** | **`43007496491`** | `42e9588f` | **success** — jobs `130478519214` (Ubuntu) / `130478518886` (Windows), 33 steps each, 0 failures, 9/14 counterpart skips |
| **Substantive `main`** | **`43008517097`** | `42e9588f` | **success** — jobs `130481931719` (Ubuntu) / `130481931995` (Windows), 0 failures, 9/14 counterpart skips |

Both green runs show **14/14 named `r03c1_*` tests executed and passing on each host** (not merely present in the source), with identical totals:

| Host | Test total | Named R03C1 tests | Security gate |
| --- | --- | --- | --- |
| `ubuntu-24.04` | **384 passed, 0 failed, 0 ignored** | 14/14 `ok` | `security_gate=pass`, RustSec DB `901cf64f2d04dc7f9f26bbbd50a3010378de034d` fetched `10:54:56Z` |
| `windows-2025` | **386 passed, 0 failed, 0 ignored** | 14/14 `ok` | `security_gate=pass`, same DB commit, fetched `10:55:00Z` |

The 384/386 totals are exactly the accepted baseline 370/372 plus the 14 recovered R03.C1 tests. Every mandatory step passed on its host: pinned toolchain, locked fetch and exact graph, `cargo-deny 0.20.2` advisories/bans/licenses/sources, package-set gate, `cargo fmt --all -- --check`, strict Clippy, workspace/all-target locked tests, architecture checker plus refusal self-test and forbidden-import/bypass refusal, test-failure tripwire, locked release build, real CLI accepted/refused smoke, and the Ubuntu-only advisory negative probe (`RUSTSEC-2026-0009` refused) plus byte-for-byte double release build. Skips are only the documented counterpart-host/Ubuntu-only steps (9 on Ubuntu, 14 on Windows) — `NOT APPLICABLE` for that host, never counted as a pass.

**Warnings and local limits:** the inherited `warning[license-not-encountered]` for `Unicode-3.0` at `deny.toml:51` is emitted by the passing gate on both hosts, remains non-fatal and **UNVERIFIED**, and was neither remediated nor suppressed. Local Windows test execution is **NOT RUN** — this workstation has no MSVC `link.exe` (`error: linker 'link.exe' not found`), so the two-host CI test logs above are the behavioral evidence; local `cargo fmt --all -- --check` and `cargo clippy -p solo-application --all-targets --all-features --locked -- -D warnings` both exit 0 at the corrected tip.

## 4. Frozen R03C1-01…12 matrix — actual outcomes

| ID | Outcome | Named execution evidence (both hosts, 14/14 green) |
| --- | --- | --- |
| R03C1-01 | **PASS** (after Correction 2) | `r03c1_01_inventory_covers_all_five_levels_in_graph_and_registry_order` — nine authored obligations across Entity/Facet/Site/Page/representation, graph-parent → registry-key → local-index order, full qualified owners |
| R03C1-02 | **PASS** | `r03c1_02_all_five_roles_keep_namespace_reference_exposure_and_index` — all five roles, exact namespace, unchanged reference text, exposure and index |
| R03C1-03 | **PASS** | `r03c1_03_inventory_is_identical_for_any_requested_target_and_never_infers_geography` (application) and `r03c1_03_sibling_sites_and_two_locales_inventory_is_target_independent` (adapters, real JSON) |
| R03C1-04 | **PASS** | `r03c1_04_internal_and_descendant_suppressed_sources_stay_inventoried_and_unpublished` — internal, unlisted and descendant-suppressed sources inventoried while the public projection still hides them |
| R03C1-05 | **PASS** | `r03c1_05_suppress_without_a_local_value_adds_no_obligation` — `suppress`+`none` adds nothing, `suppress`+local adds exactly one, absent key stays distinct |
| R03C1-06 | **PASS** (after Correction 2) | `r03c1_06_same_filename_at_two_owners_and_two_namespaces_stays_distinct` — identical text at three owners, `entity` vs `platform` kept distinct, no substitution |
| R03C1-07 | **PASS** | `r03c1_07_graphic_cardinality_duplicates_and_suppression_are_unchanged`, `r03c1_07_graphic_suppression_and_internal_filtering_are_unchanged`, `r03c1_07_five_registered_graphic_roles_keep_exact_origin` (domain) — exact typed refusals preserved before public filtering |
| R03C1-08 | **PASS** | `r03c1_08_real_json_graphic_inventory_covers_all_five_levels`, `r03c1_08_invalid_graphic_on_an_unlisted_representation_refuses_before_inventory` (adapters, real strict JSON) |
| R03C1-09 | **PASS** | `r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested` — closed extension grammar, favicon PNG-only, escapes and active formats refused, no media-byte claim |
| R03C1-10 | **PASS** (after Correction 1) | `r03c1_10_inventory_is_nonpublishing_repeatable_and_empty_safe` — nonpublishing, repeatable, empty and all-internal safe, no default or generated asset |
| R03C1-11 | **PASS** | Full regression: 384/386 workspace tests green, including R01/R02/R03.A/R03.B suites, architecture checker on the real checkout, architecture refusal self-test, forbidden-import/bypass refusal, tripwire and security gates |
| R03C1-12 | **PASS** for issuance, recovery-branch and substantive `main`; the report-head run is verified after publication and named in the handoff | Runs `43003869297`, `43007496491`, `43008517097` — actual non-skipped two-host evidence with full code-bearing A1 review below |

## 5. Fresh A1 review (2026-11-11)

| Item | Finding |
| --- | --- |
| Locked graph | **22 packages** (4 first-party, 18 external) — identical to the accepted baseline; `Cargo.lock` blob `2f9e5357…` and adapter manifest `6109224c…` are **unchanged** by this order; `rustix 1.1.6` with checksum `e5e76742…` remains the locked version |
| RustSec freshness | CI fetched the advisory database fresh twice during the green runs: DB commit `901cf64f2d04dc7f9f26bbbd50a3010378de034d` at `10:54:56Z` (Ubuntu) and `10:55:00Z` (Windows); `RUSTSEC-2026-0486` remains absent for `rustix 1.1.6` and no waiver or ignore exists in `deny.toml` |
| OSV sweep | Independent `querybatch` over all **18** external locked packages at `2026-11-11T18:07:16+07:00`: **0** advisories |
| Security policy | `deny.toml` blob `3063e864…` unchanged — `unsound = "all"`, no ignored advisory; `rust-toolchain.toml` unchanged at Rust `1.99.1`; workflow blob `eb78dad2…` unchanged |
| Residual non-actionable lags | `itoa 1.0.18` (→1.0.19) and `syn 3.0.9` (→3.0.10) patch lags persist and are explicitly outside this order; no dependency change was made |
| Licence warning | `Unicode-3.0` `license-not-encountered` (`deny.toml:51`) remains present, non-fatal and **UNVERIFIED** |
| Architecture | Recovery keeps strict Hexagonal direction: `solo-domain` std-only, `solo-application` imports domain + std only, adapter test uses real strict parsing; verified by the architecture checker and refusal self-test on both hosts, not just by compilation |
| Boundaries | No filesystem read, network, decoder, catalogue, provider, renderer or publication call was added; the module is a pure application-level use case over the validated candidate and every obligation reports `grants_publication_authority() == false` |

## 6. Branch, scope and workspace state

| Item | Value |
| --- | --- |
| `solo-wo-061-r01` (original, unaccepted source) | local and `origin` both **`4371af21dce67f33cf1df0384738bf353ca80fc6`**, tree `89c6176a5f159069eccab9eb0d4ed67e88eaba4f` — byte- and commit-identical, never merged, rewired, fast-forwarded or pruned |
| `solo-wo-063-r01` (recovery) | local and `origin` both `42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1`; retained, not pruned |
| GitHub branches | exactly `main`, `solo-wo-061-r01`, `solo-wo-063-r01` |
| Authorized whole-order diff on `main` since the baseline | issuance ACTIVE; `crates/solo-application/src/graphic_asset_inventory.rs` (new); `crates/solo-application/src/lib.rs` (+5); `crates/solo-application/tests/graphic_asset_inventory.rs` (new, 919 lines); `crates/solo-adapters/tests/property_resolution.rs` (+366/−4); `crates/solo-domain/tests/property_resolution.rs` (+355); then this report |
| Forbidden paths | `Cargo.toml` (all), `Cargo.lock`, `deny.toml`, `.github/`, `rust-toolchain.toml`, `.cargo/`, `.gitignore`, `platform/`, assets/artwork/media, ROADMAP, NEXT, OUTLOOK, About/Architecture/SOP, README, AGENTS — all unchanged (`Cargo.lock` `2f9e5357…`, manifest `6109224c…`, `deny.toml` `3063e864…`, workflow `eb78dad2…`, ignore `f45dd495…`, NEXT `92fa562b…`, ROADMAP `31a0ba2e…`, OUTLOOK `cb9c7a61…`) |
| WIP slots | exactly four: ACTIVE `4ec5c245…` (frozen `SOLO-WO-063 R01`), NEXT neutral, ROADMAP with Stage 03 OPEN and R03.C pending, OUTLOOK neutral |
| Root / remotes / worktrees | one checkout, one worktree, exactly one remote `origin` = the public repository; no second clone, nested repo or extra remote |
| Working tree | clean at the completed revisions (`git status --porcelain` and `--untracked-files=all` empty) |
| Temp / caches | Solo `.temp/` unchanged at **2 files / 289,253,806 bytes** (`architecture-fixtures/`, `toolchain-installer/`), retained under its documented later cleanup trigger; live ignored root `target/` build cache preserved and untouched (`.gitignore:24` `/target/`); task-owned Cline temp logs from this run removed; no unknown or Owner-owned material deleted |

## 7. What is still NOT delivered

R03.C remains **OPEN** and this slice delivers only the authored obligation inventory. Not implemented, not claimed and not authorized here: actual asset existence or confined file reading, media-byte/magic/dimension validation, any decoder, the pinned `platform/assets/` catalogue or artwork, responsive derivatives, published asset metadata, automatic platform fallback for any role (prohibited by `ARCHITECTURE.md` §6.2I.1), routes/HTTP/renderer, and any release, deployment or Stage 03 closure. `platformAssetLibraryVersion: "1.0.0"` remains a contract pin, not delivered artwork.

## 8. Handoff

`COMPLETE` is reported only for the bounded `SOLO-WO-063 R01` R03.C1 slice, with two triaged in-scope corrections disclosed above (`R03C1-10` lint idiom, `R03C1-01`/`R03C1-06` fixture cardinality) and no scope broadening. The Chief Architect independently reads this report at the published revision, reviews the exact diffs and the green two-host logs, and issues `ACCEPT`, `REVISE` or `STOP`. ACTIVE stays frozen, no self-ACCEPT is implied, and administrative closure of either retained branch requires separate Owner authorization.
