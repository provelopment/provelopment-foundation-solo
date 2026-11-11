# Execution Report

**Order:** `SOLO-WO-056 R01` - implement `R03.A` five-level typed property registry, direct-ancestry resolution, provenance and safe public projection; **revised** per the Chief Architect's `REVISE` directive of 2026-11-10 against substantive `5a4a7b2c` and report `768f6beb`.
**Date:** 2026-11-10.
**Root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified `git rev-parse --show-toplevel`; one checkout, one worktree, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`).
**Disposition: `COMPLETE`** for the corrected scope - all three mandatory corrections are implemented and verified on both hosts, and every accepted `R03.A` invariant is preserved. This is **not** a self-acceptance: `R03.A` still awaits the Architect's independent `ACCEPT / REVISE / STOP`. `R03.B` and `R03.C` remain unimplemented, Stage 03 remains OPEN, ACTIVE was not cleared and `ROADMAP` was not advanced.

## 1. Revision ledger

| Item | Value |
| --- | --- |
| Starting `origin/main` (reviewed baseline) | `8a0365ca5b14628448ff698b6b46c67090537f6c` (tree `4dde7d8b2cc815119cd6ffcf4f73610c62b5bf13`) |
| Authorization transition | `c087a4503eee571ec233e262dc5e0d4247445912` (tree `0d05b21ad2c12084e14aa1fdc0270477e0e197a2`) |
| Branch `solo-wo-056-r01`, first cycle | `246180d` -> `0d224b6` -> `efedd19` -> `13e1f1f` -> `2c78026` -> `5a4a7b2c` (reviewed substantive) |
| Safe reconciliation before the corrections | branch fast-forwarded to the published main/report `768f6beb22439d38bcc0e1eeef57b8238e2ed3ed`; no history rewrite, no new branch |
| Correction 1 (domain: whole-graph) | `16e32c592633988e92e9c9e06baaed5284008aa4` |
| Correction 2 (application: fail-closed use case) | `a2f22030ea598ae27c928d505e1180756279b126` |
| Correction 3 (adapter: membership binding + extend syntax) | `fa724977f4e7b76493c386da3501f92e1d2206e7` |
| **Corrected substantive `main`** (fast-forward only) | `fa724977f4e7b76493c386da3501f92e1d2206e7` (tree `6aae1296d8bd9a03d66cb7e04feb1034940c6c58`) |
| Report-only commit | this file's own commit - SHA returned in chat only, never embedded here |

`git merge --ff-only` was used for both the reconciliation (`5a4a7b2..768f6be`, report file only) and the integration (`768f6be..fa72497`): no merge commit, no squash, no rebase, no force. `main == origin/main` was verified after each push. The branch is **retained** for separate Owner-authorized later pruning.

## 2. Changed paths

This correction cycle changed **8 tracked paths**, all authorized code/test paths: `+1039` / `-70`.

| Path | Delta this cycle |
| --- | --- |
| `crates/solo-domain/src/property_resolution.rs` | +175 / -16 |
| `crates/solo-domain/src/property_registry.rs` | +44 / -10 |
| `crates/solo-domain/src/lib.rs` | +3 / -2 |
| `crates/solo-domain/tests/property_resolution.rs` | +208 / -5 |
| `crates/solo-application/src/property_resolution.rs` | +61 / -11 |
| `crates/solo-application/tests/property_resolution.rs` | +133 / -3 |
| `crates/solo-adapters/src/property_resolution.rs` | +100 / -22 |
| `crates/solo-adapters/tests/property_resolution.rs` | +315 / -1 |

Cumulative from the reviewed baseline, the same eleven authorized paths as before are involved (the report path aside). **Frozen and verified unchanged across the whole order:** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.github/` (workflow untouched), `crates/solo-cli/`, every existing R01/R02 source module, test and fixture, the starter seed and expanded fixtures, `README.md`, `AGENTS.md`, `.documents/About/`, `.documents/Architecture/`, both SOPs and `LICENSE`. `ROADMAP` (blob `4c7ef021`), `OUTLOOK` (blob `cb9c7a61`), the issued ACTIVE (blob `413359d7`) and neutral NEXT (blob `92fa562b`) are unchanged from the authorization revision.

## 3. The three mandated corrections

### 3.1 Whole-graph, fail-closed resolution

**Domain.** `ValidatedPropertyInputV1` now has two resolution entry points over the same validated data: `resolve(target)` (one owner, walking that owner's direct chain through the existing parent-reuse helper) and `resolve_all()` (every owner of the one Entity). `resolve_all` traverses the graph's own deterministic parent-first owner order, derives each owner's parent by chain prefix and a prebuilt position index, consumes only that parent's resolved record, and returns `ResolvedPropertyGraphV1` - which exists **only** if every owner resolved. Declarations are indexed once into a `BTreeMap`, so no owner triggers a rescan of the declaration list; nothing is quadratic and each owner sees only its validated immediate parent.

**Manifest ↔ graph membership.** The adapter now builds the parsed manifest's **complete** owner inventory - Entity, every Facet, Site, Page and every declared representation, including owners that declare no `properties` - and binds it to the request with `with_required_membership`. The use case calls `require_matching_owner_membership_v1`, which refuses a graph whose membership differs in size or in any member (`OwnerMembershipMismatch`), so a narrower or wider graph cannot be used even when the differing owners carry no declarations.

**Application.** `resolve_declared_properties_v1` validates declarations, applies the membership binding when supplied, then calls `resolve_all_owners_v1` and only afterwards selects the requested owner (`UnknownOwner` when absent) and projects it publicly. A candidate can therefore no longer exist while an unrelated Site, Page or unlisted representation is invalid. The requested target's result and public-projection API are unchanged; the candidate additionally exposes the whole-graph record (`resolved_graph()`, `resolved_owner_count()`).

**Preserved:** the direct-ancestry semantics (absent carries, `inherit` carries and appends, `suppress` clears and appends, `"none"` adds nothing), effective cardinality and uniqueness **before** filtering, ordering, internal provenance, the separate public projection, and the accepted `EfsOwnershipGraph` as the only ownership source. No independent ownership graph was introduced.

### 3.2 Typed empty-array bypass closed

`DeclaredOwnerPropertiesV1::new` now refuses `CurrentContextV1::Values(vec![])` with `PropertyRefusalKind::EmptyContextArray` (carrying the key), so the typed API cannot be used to smuggle an explicitly empty array past the JSON adapter. `CurrentContextV1::None` remains valid and remains distinct from an absent declaration; `Values([])` is never coerced into `None`.

### 3.3 `location.extend` syntax validation completed

`translate_location_extend` now validates the whole accepted envelope before producing the recognized unsupported-operation request:

* `target` and `owner` are closed objects; `owner` requires `entity` and accepts only `facet`, `site`, `page`, `locale` in a **contiguous** lineage (`site` needs `facet`, `page` needs `facet` and `site`, `locale` needs the whole chain), each parsed through the accepted identifier or locale-key boundary (`EntityIdV1`, `FacetIdV1`, `SiteIdV1`, `PageIdV1`, `LocaleKeyV1::from_already_canonical`).
* `set` is validated by the **same** `translate_location_fields` used by `add`: closed field names, plain-text bounds, uppercase ISO alpha-2 country codes, finite in-range WGS84 coordinates, non-empty.
* Missing, extra, malformed or structurally incomplete members refuse with a typed category.
* A syntactically valid extension is still recognized as `LocationExtensionRequestV1` (now carrying the validated target owner and `set` fields) and still produces `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A` when a resolution reaches it. No identified-record mutation, field merge or extension provenance was implemented - that remains `R03.B`.

## 4. Acceptance matrix

**37 tests** at the three authorized paths (domain 16, application 7, adapters 14), all executed green on both hosts in `42941833853` (branch) and `42942318551` (substantive `main`). The previous 26 tests are preserved - two were extended rather than weakened - and 11 are new.

| Case | Evidence |
| --- | --- |
| `R03A-01` Registry and closed encoding | domain `r03a_01_registry_declares_exactly_the_nine_accepted_keys` (nine exact names, version `1.0.0`, exact lookup, maxima 16/4/1, `seo.*` refused at Entity and Facet) and `r03a_01_closed_encoding_refuses_wrong_level_and_unsafe_values` (`WrongLevel`, `DuplicatePropertyEntry`, `WrongEntryShape`, `FaviconNotPng`, plain-text and nine mailbox refusals). adapters `r03a_09_closed_envelope_forms_refuse`. |
| `R03A-02` Five-level ancestry | domain `r03a_02_five_levels_inherit_only_direct_ancestors` (four effective values with `[entity, facet, site, page]` origins, sibling branch unaffected, one changed branch only). application `application_resolves_all_five_levels_without_publication_authority`. adapters `r03a_09_real_manifest_bytes_resolve_all_five_levels`. |
| `R03A-03` Explicit empty and suppression | domain `r03a_03_absent_and_explicit_none_are_distinct_and_suppression_is_final` and the **new** `r03a_03_explicit_empty_array_refuses_in_the_typed_api` (empty array -> `EmptyContextArray` with the key, `"none"` still valid, absence still distinct). |
| `R03A-04` Ordering and provenance | domain `r03a_04_ordering_provenance_and_field_origins_are_exact` and the **new** `r03a_04_whole_graph_preserves_each_branch_original_values` (two Sites keep their own values, order, `originOwner`, `originLocalIndex`; whole-graph resolution is deterministic across repetition). |
| `R03A-05` Effective cardinality | domain `r03a_05_effective_cardinality_is_enforced_before_public_filtering` plus the **new** `r03a_05_whole_graph_refuses_a_conflict_in_an_unrelated_site` (the bali target resolves alone; the jakarta conflict refuses the graph). application `application_refuses_a_conflict_in_an_unrelated_site`. adapters `r03a_09_unrelated_site_conflict_refuses_the_whole_candidate` and `r03a_09_effective_cardinality_and_duplicates_refuse`. |
| `R03A-06` Duplicate and typed refusal | domain `r03a_06_duplicate_and_malformed_entries_refuse_without_repair`. adapters `r03a_09_closed_typed_entries_refuse` and the **new** `r03a_09_extend_syntax_refusals`. |
| `R03A-07` Exposure | domain `r03a_07_public_projection_omits_internal_values_and_cannot_resurrect` (internal values never projected, no ancestor resurrection, explicit replacement published). application `application_public_projection_omits_internal_values`. |
| `R03A-08` Qualified isolation | domain `r03a_08_identical_page_ids_under_different_sites_and_locales_never_mix` (three Pages named `home`; only the bali `en-001` branch sees its value; the same Page's `id-ID` inherits the Page-level value with the Page as origin) and `r03a_08_wrong_lineage_and_second_entity_refuse`. application `application_refuses_an_owner_outside_the_graph` and the **new** `application_refuses_a_manifest_membership_mismatch` (missing and extra owners both refuse; the exact inventory resolves with `resolved_owner_count() == 8`). adapters `r03a_09_graph_membership_mismatch_refuses`. |
| `R03A-09` Actual adapter boundary | adapters `r03a_09_real_manifest_bytes_resolve_all_five_levels`, `r03a_09_unknown_property_refuses_by_category`, `r03a_09_wrong_level_declaration_refuses`, `r03a_09_closed_envelope_forms_refuse`, `r03a_09_closed_typed_entries_refuse`, `r03a_09_unlisted_sibling_declaration_fails_closed`, `r03a_09_committed_starter_seed_stays_inert`, plus the **new** whole-graph set below. The unchanged R02.C `r02c_seed` suite (17 cases) stayed green in the same runs. |
| `R03A-10` Deferred specialists remain honest | domain `r03a_10_recognized_extend_is_refused_by_name_while_add_works` and the **new** `r03a_10_whole_graph_refuses_a_recognized_extension_anywhere`. application `application_surfaces_the_named_extension_refusal`. adapters `r03a_09_recognized_extend_is_refused_by_name`, the **new** `r03a_09_extension_anywhere_refuses_the_whole_candidate`, and `r03a_09_extend_syntax_refusals` (which also proves a complete, typed extension is still recognized). |
| `R03A-11` Invariants and security | domain `r03a_11_property_resolution_leaves_identity_untouched`. adapters `r03a_09_unlisted_sibling_declaration_fails_closed` (one invalid unlisted declaration fails the whole candidate, diagnostics name the owner and key without echoing authored text). |
| `R03A-12` Existing baseline and CI | All existing R01/R02 suites, the architecture checker with its refusal self-test, the unsafe-boundary and workspace-graph checks, the tripwire, the Linux Btrfs fixture and cleanup, the Windows native guards, the locked release build, the CLI smoke and the byte-identical builds ran unchanged and green at `42941833853` and `42942318551`. |

**The directive's five failing-first tests, mapped 1:1**

| Directive test | Test(s) |
| --- | --- |
| 1. Valid Site A target + inherited-plus-local singular conflict in Site B rejects the candidate | adapters `r03a_09_unrelated_site_conflict_refuses_the_whole_candidate`; application `application_refuses_a_conflict_in_an_unrelated_site`; domain `r03a_05_whole_graph_refuses_a_conflict_in_an_unrelated_site` |
| 2. The same conflict on an unlisted representation rejects | adapters `r03a_09_unlisted_representation_conflict_refuses` |
| 3. A valid target must not succeed against a graph with extra or missing manifest owners | adapters `r03a_09_graph_membership_mismatch_refuses`; application `application_refuses_a_manifest_membership_mismatch` |
| 4. A valid multi-Site candidate preserves each branch's values, order and provenance | domain `r03a_04_whole_graph_preserves_each_branch_original_values` |
| 5. A recognized unsupported `extend` anywhere must not produce a false whole-graph success | domain `r03a_10_whole_graph_refuses_a_recognized_extension_anywhere`; adapters `r03a_09_extension_anywhere_refuses_the_whole_candidate` |

**Section 2 test (typed empty array):** domain `r03a_03_explicit_empty_array_refuses_in_the_typed_api` asserts `EmptyContextArray` and the key, and that `"none"` stays valid and distinct.

**Section 3 tests (`extend` syntax):** adapters `r03a_09_extend_syntax_refusals` covers a malformed coordinate set (`InvalidCoordinates`), an unknown field (`UnknownLocationField`), a mistyped field (`WrongEntryShape`), a lower-case country code (`InvalidCountryCode`), an empty `set` (`EmptyLocationFields`), a missing `set` (`MissingEntryField`), an owner with `page` but no `facet`/`site` (`InconsistentOwner`), a non-canonical locale (`InconsistentOwner`), an extra owner member (`ExtraEntryField`), and a genuinely valid extension that is still refused by name at resolution.

## 5. CI evidence (unchanged `rust-verify` workflow)

| Run | Revision | Result |
| --- | --- | --- |
| `42850427568` | `c087a450` authorization transition | Ubuntu success, Windows success |
| `42850442030` | `efedd199` first cycle | failure - `clippy::result_large_err` (refusal type carried a full qualified owner) |
| `42851850519` | `13e1f1f6` | failure - `E0618` (test helper shadowed by a local binding) |
| `42852757877` | `2c780264` | failure - four test expectations (root `inherited` event, missing key on a wrong-level refusal, two over-strict isolation expectations) |
| `42853919842` | `5a4a7b2c` branch | Ubuntu success, Windows success |
| `42855602900` | `5a4a7b2c` substantive main | Ubuntu success, Windows success |
| `42856849005` | `768f6beb` report HEAD (previous cycle) | Ubuntu success, Windows success |
| `42941833853` | `fa724977` **corrected branch** | Ubuntu success, Windows success, **0 failed steps**, 33 steps, skips 9 / 14 |
| `42942318551` | `fa724977` **corrected substantive main** | Ubuntu success, Windows success, **0 failed steps**, skips 9 / 14 |
| this report's own run | report HEAD | dispatched and verified separately; SHA returned in chat |

At `42941833853` and `42942318551` every host-specific step passed on its designated host with the unchanged workflow: pinned toolchain install and exact version assertions, locked fetch, the `cargo-deny 0.20.2` archive gate with a fresh RustSec refresh, the `RUSTSEC-2026-0009` advisory negative probe, the workspace package-set gate, the disposable Btrfs fixtures with guaranteed cleanup, format, strict lint, the full locked all-target test run, the real-tree architecture checker with its refusal self-test, the tripwire, the locked release build, the CLI smoke, the forbidden-import refusal with restore, and two byte-identical release builds. Skipped steps are exactly the counterpart host's variants (9 on Ubuntu, 14 on Windows); no applicable check was skipped and no material warning was silenced.

All three new test binaries executed in both green runs (`Running tests/property_resolution.rs` three times: domain 16 cases, application 7, adapters 14) alongside the unchanged `r02c_seed` suite. Every intermediate failure above is disclosed and was fixed in a separate commit before re-verification.

## 6. A1 dependency, security and version-currency evidence

**Sources and query date.** All package versions were read on **2026-11-10** from the crates.io registry API (`/api/v1/crates/<name>/versions`, newest first) and the Rust release feed (`api.github.com/repos/rust-lang/rust/releases/latest`); the action tag was resolved through the GitHub API for the pinned commit; runner and tool versions come from the workflow definition and the run logs of `42941833853` / `42942318551`; the advisory position comes from the in-run RustSec refresh performed by the pinned `cargo-deny 0.20.2` gate itself.

**Toolchain and tooling.** `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, `rustup 1.29.3 (edbc831ce 2026-11-02)`, single installed toolchain `1.99.1-x86_64-pc-windows-msvc`. The latest published Rust release is **1.99.1** (2026-10-16), so the pin **is** the current stable release; no newer supported release, LTS branch or security-only update exists to move to. `cargo-deny` pin `0.20.2` is the newest published version (2026-07-09) and its release archive is digest-pinned in the workflow (`9f12ed4c...`).

**Pinned actions and runners.** `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` = tag **v7.0.1**; it is the only external action used. Runner images observed: `ubuntu-24.04` and `windows-2025-vs2026` (image version `20261103.298.1`, runner `2.338.2`). Both are current GitHub-hosted images; nothing is deprecated.

**Dependency graph (locked, unchanged by this order).** Normal, direct: `serde 1.0.229`, `serde_json 1.0.154` (adapters), `rustix 1.1.5` (Linux only), `windows-sys 0.61.2` (Windows only). Normal, transitive: `serde_core 1.0.229`, `itoa 1.0.18`, `memchr 2.8.3`, `zmij 1.0.23`, `bitflags 2.13.2`, `linux-raw-sys 0.12.1`, `windows-link 0.2.1`. Build/dev edges: `syn 3.0.9`, `proc-macro2 1.0.107`, `quote 1.0.47`, `unicode-ident 1.0.27`. `solo-domain` and `solo-application` have **no** external dependency at all.

**Currency inventory and dispositions (all 2026-11-10).**

| Package | Pinned | Newest published | Position | Disposition |
| --- | --- | --- | --- | --- |
| Rust / rustc / cargo | 1.99.1 | 1.99.1 (2026-10-16) | current stable | none needed |
| rustup | 1.29.3 | local toolchain manager; not part of the product graph | current installed | none |
| cargo-deny | 0.20.2 | 0.20.2 (2026-07-09) | current | none |
| actions/checkout | `3d3c42e5...` | tag v7.0.1 for that commit | pinned tag | none |
| `serde` | 1.0.229 | 1.0.229 | current | none |
| `serde_core` | 1.0.229 | 1.0.229 | current | none |
| `serde_json` | 1.0.154 | 1.0.154 | current | none |
| `itoa` | 1.0.18 | 1.0.19 (2026-11-09) | one patch behind (transitive via `serde_json`) | **not upgraded** - dependency changes are outside this order; recorded for a future Owner dependency order |
| `memchr` | 2.8.3 | 2.8.3 | current | none |
| `zmij` | 1.0.23 | 1.0.23 | current | none |
| `rustix` | 1.1.5 | 1.1.6 | one patch behind (direct, Linux only) | **not upgraded** - pinned in `Cargo.toml`; out of scope |
| `bitflags` | 2.13.2 | 2.13.2 | current | none |
| `linux-raw-sys` | 0.12.1 | 0.12.1 | current | none |
| `windows-sys` | 0.61.2 | 0.61.2 | current | none |
| `windows-link` | 0.2.1 | 0.100.0 overall; 0.2.1 is the newest `0.2.x` | major line ahead; `0.100.0` is incompatible with the pinned `windows-sys 0.61.2` | **not applicable**; no action |
| `syn` | 3.0.9 | 3.0.10 (2026-11-09) | one patch behind (build/dev edge only) | **not upgraded** - out of scope |
| `proc-macro2` | 1.0.107 | 1.0.107 | current | none |
| `quote` | 1.0.47 | 1.0.47 | current | none |
| `unicode-ident` | 1.0.27 | 1.0.27 | current | none |

**Advisory applicability.** The authoritative check is the in-run `cargo-deny 0.20.2` gate over a freshly fetched RustSec database, which reported **no advisory** against the locked graph in both green runs at `fa724977`; the deliberate `RUSTSEC-2026-0009` negative probe still fails as designed, proving the database was live rather than stale. No `yanked` version, no known-vulnerable crate, no licence or ban violation was reported at the substantive SHA. No dependency, lockfile, toolchain or deny-policy change was made in this order, so nothing outside the accepted pins could have been introduced.

## 7. Verification limits, scratch and final state

**Local, on the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain:** `cargo fmt --all -- --check` clean; `cargo check --locked -p solo-domain --all-targets` and `-p solo-application --all-targets` clean; `cargo clippy --locked -p solo-domain --all-targets -- -D warnings` and the same for `solo-application` clean, including all 23 of their new and existing tests. **NOT RUN locally:** `cargo clippy --workspace` and `cargo test` - this workstation has no MSVC `link.exe`, so `serde`'s build script cannot link and the `solo-adapters` crate plus every test binary is compiled and executed only in CI. The adapter corrections and all 37 tests are therefore verified by the two green two-host runs above; nothing is inferred from an unexecuted local command.

**Scratch and Cline task material.** Solo `.temp/` was not used. The local build cache `target/` (untracked and not covered by this repository's `.gitignore`) was created by the local `cargo` runs and has been deleted again before the final state check; if the IDE's language server recreates it, it remains untracked and is not committed. The Cline task temporary area (`%LOCALAPPDATA%\Temp\cline`) held 20 tool-generated `proceed-while-running-*.log` files from this and the preceding cycle; all 20 were deleted successfully, none remained, and no unknown or Owner-owned material was touched. No retained items and no cleanup limitation remain.

**Final state at the report commit.** `git status --porcelain` empty; one checkout at the Owner's root; one worktree; sole `origin`; local and remote `main` identical at the substantive revision; branches `main` plus the retained `solo-wo-056-r01`; the four WIP files unchanged from the authorization revision (ACTIVE still holds the issued order, NEXT is neutral, ROADMAP and OUTLOOK untouched); `Cargo.lock`, `Cargo.toml`, `rust-toolchain.toml`, `deny.toml`, `.github/` and `crates/solo-cli/` byte-identical to the reviewed baseline.

## 8. Scope boundaries and honest status

**Implemented and verified in this cycle:** whole-graph fail-closed validation with deterministic parent-first traversal and indexed declarations, manifest↔graph membership binding including owners without properties, the typed empty-array refusal, complete `extend` syntax validation (closed contiguous owner lineage, typed `set`, retained request data), and 11 new tests with all prior `R03.A` behaviour preserved.

**Deliberately not implemented, and not claimed:** `R03.B` identified-record extension and field mutation (the recognized request is carried and refused by name); `R03.C` graphics assets, catalogue, fallback and delivery (only the typed declaration reference is validated); routing, locale canonicalization, visibility decisions, HTML/browser output, deployment and release. No whole-product or full `O08`/`O09` conformance is claimed.

**WIP authority untouched beyond the authorized transition:** `ROADMAP` still lists Stage 03 with `R03.A` as its first package, Stage 03 remains OPEN, Stages 04-13 are unchanged, `NEXT` remains neutral, and ACTIVE was not cleared. The Implementation Agent did not self-ACCEPT, did not advance any stage and did not issue a next order; the Architect's independent review of exactly `fa724977f4e7b76493c386da3501f92e1d2206e7` and this report decides what happens next.






