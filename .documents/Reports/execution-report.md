# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-059 R01` — implement R03.B: identified inherited-location extension, exact field-level provenance and independent five-level geography.
**Session date:** 2026-11-11.
**Root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified `git rev-parse --show-toplevel`; one checkout, one worktree, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`).
**Disposition:** implementation and every in-scope gate are complete and green. The Implementation Agent does **not** self-ACCEPT; the Chief Architect reviews this report independently. **Stage 03 remains OPEN**, `R03.B` remains in ROADMAP until a separate Owner-issued closure, and `R03.C` is untouched.

## 1. Revision ledger

| Step | Commit | Tree | Paths in the commit | Two-host CI |
| --- | --- | --- | --- | --- |
| Starting `main` = `origin/main` | `7ae6bedc4f846310af86e2bce6a1904f606f22fc` | `1459944c4e2e73fdcf2c20805a914efdec3ea828` | — | run `42957731037` green (prior report) |
| 1. Authorization (ACTIVE installed, NEXT cleared) | `742afbc275134050f5ea38f651b68cfd182ffc26` | `daeaa93ed9549ac141e35910c658d39ce0ef0ddf` | `workorder-active.md` (`52d0fda3…`), `plan-next.md` (`92fa562b…`) | run `42967005798` **success** |
| 2. Branch `solo-wo-059-r01` (first attempt) | `b43b831e971fc029bc00859272c986dd1de936ee` | `fe12d8c392ce44a0b4fdd6c1c509faadb140b320` | the eight authorized code/test paths | run `42970181822` **failure** — three of my own test expectations, corrected below |
| 3. Branch corrected | `f40cb6f0b5c3a7a86d4edfc952e5d5f275b2ee5c` | `b9c88986ef86c42c1a9ea2670ea04e7f2ed6b8d9` | adapter/domain test files only | run `42970933402` **success** |
| 4. Substantive `main` (fast-forward) | `f40cb6f0b5c3a7a86d4edfc952e5d5f275b2ee5c` | `b9c88986ef86c42c1a9ea2670ea04e7f2ed6b8d9` | none new (fast-forward) | run `42971561588` **success** (fresh push run) |
| 5. This report | report-only commit on `main` | — | `.documents/Reports/execution-report.md` | fresh run at report HEAD; ID returned in chat |

`main` was fast-forwarded to the verified branch (no commit, no squash, no rebase, no force), pushed normally, and `origin/main` equals local `HEAD`. Branch `solo-wo-059-r01` is **retained** for later, separately Owner-authorized pruning. One worktree, one remote, two branches.

**Disclosed correction.** The first branch run failed at the workspace test step on both hosts because three *test expectations I wrote* were wrong, not because the implementation was wrong: a portable source path used uppercase (`main-home.id-ID.md`), a per-fact field count was asserted where an overlay value was meant, and the same-node extension case was expected as "not inherited" when the direct-ancestor rule refuses it first. All three were corrected in the authorized test files only, re-verified, and the green runs above are at the final substantive SHA.

## 2. Exact changed-path ledger (eight authorized paths)

| Path | Change |
| --- | --- |
| `crates/solo-domain/src/property_registry.rs` | `LocationFieldsV1::overlay` (bounded typed overlay, `coordinates` atomic), `LocationFactV1::with_overlay` (identity/kind/exposure preserved), `LocationExtensionRequestV1::new` now validates a nonempty `set`, `AuthoredPropertyValueV1::new` now refuses a public extension wrapper, `is_supported_in_r03a` replaced by `is_location_extension`, new `PropertyRefusalKind::ExtensionExposureForbidden` |
| `crates/solo-domain/src/property_resolution.rs` | identified in-place `extend` in the existing direct-parent resolver, per-field origin updates, `Extended` events, typed `LocationExtensionCause` (four causes), `LOCATION_INVALID_EXTEND`, refusal now carrying the offending declaring owner and registered key, boxed to stay small |
| `crates/solo-domain/src/lib.rs` | one re-export (`LocationExtensionCause`) |
| `crates/solo-domain/tests/property_resolution.rs` | R03B-01…09 domain matrix; two R03A extension tests converted |
| `crates/solo-application/src/property_resolution.rs` | documentation truthfulness only (no behavioural change, no new port, no publication authority) |
| `crates/solo-application/tests/property_resolution.rs` | converted use-case test plus R03B-11 fail-closed test |
| `crates/solo-adapters/src/property_resolution.rs` | `extend` translation now validates through the typed request; closure mapping to `LOCATION_INVALID_EXTEND`; resolution refusal now reports the **actual** offending owner and key plus the typed cause |
| `crates/solo-adapters/tests/property_resolution.rs` | three converted boundary tests plus R03B-10 (real JSON geography) and R03B-11 (unlisted fail-closed) |

No file was added, renamed or deleted. `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.cargo/config.toml`, `.github/`, `.gitignore`, `README.md`, `AGENTS.md`, `.documents/About/`, `.documents/Architecture/`, both SOPs, `plan-roadmap.md`, `plan-outlook.md`, the parser modules and all R01/R02 fixtures are untouched.

## 3. Implementation summary (what actually changed in behaviour)

- **In-place extension.** `resolve_key_from` still copies the immediate parent's effective collection, then applies the node's `currentContext` operations in authored order. An `extend` locates exactly one fact in the inherited prefix by `(original owner, original factId)`, overlays only the members named in `set` (`LocationFieldsV1::overlay`, `coordinates` as one atomic field), rewrites only those members' `fieldOrigins` to `(this owner, this local index)`, keeps `factId`, `kind`, original owner/index, whole-record `exposure` and list position, and records one `Extended(owner, localIndex, targetFactId)` event.
- **Ancestry.** A target must satisfy `target_owner.is_ancestor_of(current)` (strict direct-chain ancestor). The Entity root cannot extend because it has no ancestor; a current-node, sibling, other-Facet, other-Entity, descendant or different-locale target refuses before any identity lookup.
- **Identity.** Lookup never matches on `factId` alone, a label, a Page ID or a locale: two inherited facts with the same `factId` under different original owners stay two facts and only the fully qualified one changes.
- **Suppression and same-node rules.** A node's own `suppress` is remembered long enough to refuse a target it removed with the accurate `target-suppressed` cause; a fact removed by an intervening ancestor is refused as `target-not-inherited`; a locally added fact is never an eligible target (the direct-ancestor rule refuses it, and the inherited-prefix guard enforces §6.2H.5 structurally).
- **Conflicts.** Two extensions of one original fact at one node refuse (`duplicate-extension-target`) even with an identical `set`; nothing is last-wins or idempotently merged.
- **Codes.** All invalid-operation outcomes return `LOCATION_INVALID_EXTEND` with a stable cause token (`target-not-ancestor`, `target-not-inherited`, `target-suppressed`, `duplicate-extension-target`). The obsolete `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A` code no longer exists anywhere in the tree. Malformed envelopes remain closed typed **declaration** refusals (`PROPERTY_DECLARATION_REFUSED`), never resolution outcomes.
- **Typed-API bypasses closed.** `LocationExtensionRequestV1::new` refuses an empty `set` (`EmptyLocationFields`) and `AuthoredPropertyValueV1::new` refuses `(extension, Exposure::Public)` (`ExtensionExposureForbidden`), so neither forbidden state is expressible through the public typed API; the adapter's `Internal` placeholder stays inert and resolution never reads it for exposure.
- **Diagnostics.** The refusal now carries the offending declaring owner and the registered key, so a diagnostic names the node whose declaration failed; the adapter prefers that owner over the requested target. Only structural identifiers, the registered key and stable tokens appear — no authored field value, place text, coordinate, mailbox or path.
- **Preserved behaviour.** The other eight registered keys, `propertyRegistryVersion = "1.0.0"`, cardinality, suppression, inheritance, public projection and whole-graph fail-closed validation are unchanged.

## 4. `R03B-01`–`R03B-12` evidence matrix

| ID | Test(s) | Direct evidence | Result |
| --- | --- | --- | --- |
| **R03B-01** | `r03b_01_page_extension_updates_the_inherited_fact_in_place` | one fact in its original slot; `factId`, `kind`, origin owner+index and exposure unchanged; `set` overwrote `locality` and added `district`; public original fact, with the internal case in `r03b_09` | PASS |
| **R03B-02** | `r03b_02_consecutive_extensions_overlay_all_ten_fields_across_five_levels`, `application_executes_an_identified_extension_without_publication_authority` | Entity→Facet→Site→Page→representation; all ten members present in canonical order; untouched fields keep earlier origins; four `extended` and one `added` event; the use case publishes nothing and its projection carries overlaid data with no provenance | PASS |
| **R03B-03** | `r03b_03_mixed_add_extend_add_keeps_order_and_records_the_real_sequence` | inherited first, adds at local `0` and `2`, extension at local `1`; sequence `added/inherited/added/extended/added` with `Extended(…, 1, bali-base)` and no phantom `added`; repeated resolution value-identical; explicit `none` vs absent declared-distinct and effective-identical | PASS |
| **R03B-04** | `r03b_04_invalid_extension_targets_refuse_with_typed_causes`, `r03b_04_sibling_extension_refuses_the_whole_graph`, `r03b_04_extend_without_an_inherited_target_refuses`, `r03b_04_extension_in_another_site_fails_the_whole_candidate` | Entity root, current node, sibling Site, other Facet, other Entity, descendant, wrong full locale → `target-not-ancestor`; missing fact, right id under the wrong original owner, fact removed by an intervening `suppress` → `target-not-inherited`; every case `LOCATION_INVALID_EXTEND`, no fallback | PASS |
| **R03B-05** | `r03b_05_suppression_is_final_for_extensions` | `suppress`+`extend` at one node → `target-suppressed`; descendant of a suppressed ancestor → `target-not-inherited`; `suppress`+`add` valid and records `suppressed`; a same-node `add` is not targetable | PASS |
| **R03B-06** | `r03b_06_duplicate_extensions_refuse_and_same_ids_stay_distinguishable` | two identical extensions → `duplicate-extension-target`; two facts sharing `factId` under Entity and Facet remain two, only the named Facet fact changes, the other keeps its fields | PASS |
| **R03B-07** | `r03b_07_effective_maximum_and_duplicate_add_refusals_are_unchanged` | 16 effective facts stay 16 through a valid extension; a 17th `internal` add refuses with `CardinalityExceeded` before projection; duplicate independent `add` identity refuses with `DuplicateLocationFact` | PASS |
| **R03B-08** | `r03b_08_typed_boundary_refuses_empty_set_and_public_wrapper`, `r03b_08_extend_syntax_refusals_are_unchanged` | typed: empty `set` → `EmptyLocationFields`, public wrapper → `ExtensionExposureForbidden`, internal placeholder accepted; JSON: empty/unknown `set`, `null`, wrong type, invalid coordinates/country code/locale/lineage, extra member, authored `exposure`, missing `set` all refuse as declarations | PASS |
| **R03B-09** | `r03b_09_internal_facts_stay_unprojected_after_extension` | internal fact extended at Page and representation stays `internal`; projection empty and carrying no origin/event; the public sibling branch projects only its own fact; refusal text contains no authored or inherited place text | PASS |
| **R03B-10** | `r03b_10_real_json_geography_extends_without_identity_or_locale_effects` | real strict JSON: Entity Australia fact extended by Facet (administrative area), Site (locality), Page (district), `en-001` (postal code), `id-ID` (named place), plus an independent Jakarta premises fact; `en-001` has six fields and no `namedPlace`, `id-ID` has the extension and two facts and no sibling `postalCode`; Entity ID, Page ID, base URL and both authored route strings unchanged; no geographic inference and no locale-driven selection | PASS |
| **R03B-11** | `r03b_11_unlisted_invalid_extension_fails_the_whole_candidate`, `application_fails_closed_on_an_invalid_sibling_extension` | an invalid `extend` on an **unlisted** `id-ID` representation refuses the whole candidate for a valid `en-001` request, names that unlisted declaration rather than the requested target, and echoes no content; the control manifest resolves and the unlisted branch keeps its own inherited copy | PASS |
| **R03B-12** | whole workspace CI | every pre-existing R03A/R01/R02 test, the real-tree architecture checker with its 23-fixture refusal self-test, the failure tripwire, the locked release build and the CLI accepted/refused smoke pass on both hosts; converted tests replaced 1:1 below | PASS |

## 5. Legacy-test adaptation ledger (no coverage silently deleted)

| File | Original test (R03A expectation) | Replacement |
| --- | --- | --- |
| domain | `r03a_10_recognized_extend_is_refused_by_name_while_add_works` | `r03b_01_page_extension_updates_the_inherited_fact_in_place` (positive in-place assertions; the original add-only assertions retained) |
| domain | `r03a_10_whole_graph_refuses_a_recognized_extension_anywhere` | `r03b_04_sibling_extension_refuses_the_whole_graph` (still refuses the whole graph, now with the R03.B code, cause and offending owner) |
| application | `application_surfaces_the_named_extension_refusal` | `application_executes_an_identified_extension_without_publication_authority` (positive) plus `application_fails_closed_on_an_invalid_sibling_extension` (negative) |
| adapters | `r03a_09_recognized_extend_is_refused_by_name` | `r03b_04_extend_without_an_inherited_target_refuses` (same JSON bytes, new code and cause) |
| adapters | `r03a_09_extension_anywhere_refuses_the_whole_candidate` | `r03b_04_extension_in_another_site_fails_the_whole_candidate` (same JSON bytes, names the offending owner) |
| adapters | `r03a_09_extend_syntax_refusals` | `r03b_08_extend_syntax_refusals_are_unchanged` (all original syntax cases kept; two closed-JSON cases added) |

`#[test]` counts: domain `property_resolution` 16 → 24, application 7 → 8, adapters 14 → 16. Eleven tests added, six converted in place, none removed; repository-wide `#[test]` count is now 386.

## 6. Two-host CI evidence (unchanged `rust-verify` workflow)

| Run | Revision | Event | `verify (ubuntu-24.04)` | `verify (windows-2025)` |
| --- | --- | --- | --- | --- |
| `42967005798` | `742afbc2` authorization | push | success | success |
| `42970181822` | `b43b831e` first branch attempt | workflow_dispatch | **failure** (workspace tests, my three wrong expectations) | **failure** (same) |
| `42970933402` | `f40cb6f0` corrected branch | workflow_dispatch | success | success |
| `42971561588` | `f40cb6f0` substantive `main` | push | success, 33 steps, 0 failed, 9 skipped | success, 33 steps, 0 failed, 14 skipped |
| report HEAD | report commit | push | verified separately; run ID in chat | verified separately |

Skips at substantive `main` are exactly the counterpart-host variants: the Ubuntu job skips the nine Windows steps; the Windows job skips the nine Linux steps plus the five Ubuntu-only steps (RustSec negative probe, disposable snapshot filesystems and cleanup, forbidden-import restoration proof, byte-identical double release build). Every applicable mandatory gate ran on both hosts: pinned toolchain install and version assertion, locked dependency fetch, `cargo-deny 0.20.2` with a fresh RustSec refresh, workspace package-set gate, format, strict Clippy, the full locked all-target test suite, the real-tree architecture checker with its 23-fixture refusal self-test, the failure tripwire, the locked release build, and the CLI accepted/refused smoke. No warning was introduced and no gate was skipped on its own host.

## 7. A1 — current-source and version-currency review (code-bearing order)

**Applicable inventory** (installed and locked, verified locally against the actual tree):

| Component | Locked / pinned | Current upstream | Assessment |
| --- | --- | --- | --- |
| `serde` | 1.0.229 | latest release `v1.0.229` | current |
| `serde_json` | 1.0.154 | crates.io `max_stable_version` 1.0.154 | current |
| `windows-sys` | 0.61.2 | crates.io `max_stable_version` 0.61.2 | current |
| `syn` (dev, `solo-cli` tests) | 3.0.9 | latest release `3.0.10` | one patch behind, dev-only, reachable only in test builds; no advisory, yank or unsoundness implied |
| Rust toolchain | 1.99.1 (pinned, rustfmt+clippy) | latest release `1.99.1` | current |
| `actions/checkout` | `3d3c42e5aac5ba805825da76410c181273ba90b1` | that SHA **is** tag `v7.0.1`, the current release | current, SHA-pinned |
| `cargo-deny` | 0.20.2 (SHA-256 verified archive) | latest release `0.20.2` | current |
| Runners | `ubuntu-24.04`, `windows-2025` | supported hosted images | current |

The lockfile holds 22 packages across four first-party crates; the workspace declares no runtime dependency of its own beyond `serde`/`serde_json`/`windows-sys` and the two dev-only crates. **Advisories:** the authoritative signal is the CI gate that resolves the current RustSec database at run time — it passed on both hosts at the substantive revision, and the Ubuntu `RUSTSEC-2026-0009` negative probe proved the gate still fails closed. **No upgrade, pin change or manifest edit was made**; `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` and the workflow are byte-identical to the authorization revision. The `syn` patch lag is recorded for a separately Owner-issued dependency order, not actioned here.

**Warnings.** `cargo-deny 0.20.2` still emits exactly one `warning[license-not-encountered]` per host for `"Unicode-3.0"` at `deny.toml:51` (2 log lines in the substantive run: 1 Ubuntu, 1 Windows), with `licenses ok: 0 errors, 1 warnings` and `bans ok: 0 errors`. It is the same pre-existing, inherited, non-vulnerability unused-allowance lint reported by the two previous orders, byte-identical `deny.toml`, and its graph/allowance mismatch remains **`UNVERIFIED`** pending that separate dependency-policy investigation. It was neither silenced nor fixed; the Unicode attribution/NOTICE duty stays a release prerequisite.

## 8. Security and privacy evidence

- **No exposure elevation.** An extension cannot carry or change exposure: the typed wrapper forbids `public`, the resolver copies the fact's own exposure, and `r03b_09` proves an internal fact stays internal through two descendant extensions and is absent from the projection with all provenance.
- **No content leakage.** Refusals carry only the code, the stable cause token, the structural owner description and the registered key; `r03b_09` and `r03b_11` assert the rendered refusal contains no authored or inherited locality text.
- **Fail-closed.** One invalid declaration anywhere — including on an unlisted representation the request never visits — prevents any candidate and any public projection (`r03b_11`, `application_fails_closed_on_an_invalid_sibling_extension`), and whole-graph membership is still required.
- **No new surface.** No port, adapter, HTTP, filesystem, routing, locale-selection, geocoding or publication authority was added; `PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY` remains `false` and is asserted.
- **Hexagonal boundary held.** Only `solo-domain` (pure) gained behaviour; the application and adapter changes are translation and diagnostics. The architecture checker passed on the real tree on both hosts.

## 9. Local verification status (honest, per check)

| Check | Result | Note |
| --- | --- | --- |
| `cargo fmt --all -- --check` | **PASS** | exit 0, no diff |
| `cargo clippy --locked -p solo-domain -p solo-application --all-targets --all-features -- -D warnings` | **PASS** | no lints |
| `cargo check --locked -p solo-domain --tests -p solo-application` | **PASS** | type-check of the new domain and application tests |
| `cargo clippy` / `cargo check` including `solo-adapters` | **NOT RUN locally** | this workstation has the pinned MSVC toolchain but **no linker** (`link.exe` absent, no GNU target installed), so dependency build scripts cannot link; `solo-adapters` was compiled, linted and tested on both CI hosts |
| `cargo test` (any target) | **NOT RUN locally** | same linker limitation; the complete locked all-target suite ran on `ubuntu-24.04` and `windows-2025` at the substantive revision with 0 failures |
| `cargo-deny` gate | **NOT RUN locally** | executed by CI on both hosts with a fresh RustSec refresh; passed |
| A1 registry/version queries | **PASS** | crates.io and GitHub release metadata recorded in §7 |
| `syn` patch-lag reachability | **VERIFIED** | dev-dependency of `solo-cli` tests only |
| Inherited `Unicode-3.0` allowance cause | **UNVERIFIED** | unchanged and unexplained by the matched graph; needs its own Owner order |

No local check is reported as passing that was not actually executed, and no CI step is described as a local run.

## 10. Repository state, temporary material and cleanup

- `main` = `origin/main` = `f40cb6f0b5c3a7a86d4edfc952e5d5f275b2ee5c` (tree `b9c88986ef86c42c1a9ea2670ea04e7f2ed6b8d9`) before this report commit; `git status --porcelain` and `--untracked-files=all` are empty.
- Four WIP files only. ACTIVE holds the **frozen issued order** (blob `52d0fda32ac082101782875070d0f0ceb5961337`) and was deliberately **not** neutralized — this order grants no disposition edit. NEXT is the neutral heading (`92fa562b1af0c54391851aa5c79deb1631127069`). ROADMAP (`754ce1b7…`) and OUTLOOK (`cb9c7a61…`) are byte-identical to the authorization revision, so `R03.B` is still listed and Stage 03 is still open.
- Branch `solo-wo-059-r01` retained at `f40cb6f0` for later Owner-authorized pruning; one worktree, one remote, no tag, no PR, no merge commit.
- Root `target/` remains ignored by the accepted `/target/` rule and was neither deleted nor altered.
- `.temp/` is unchanged at **289,253,806 bytes** across 2 files (`architecture-fixtures/`, `toolchain-installer/`), byte-for-byte the state reported by the previous order. The mingw/rustup archives stay a possible future Windows-GNU/offline setup resource; the cleanup trigger is unchanged — delete after the next successful GNU deterministic-build verification, or when the Owner confirms an offline copy is no longer needed.
- Task-owned scratch: two transient Cline terminal-progress logs appeared in the Cline task temp area during long CI waits and both were deleted; that area is empty. The report draft used disposable `.temp` parts and was consumed into the single report path; those parts are deleted, so no helper file, archive or duplicate report exists.

## 11. Explicit restrictions observed

No dependency, lockfile, toolchain, `deny.toml`, `.cargo/config.toml`, workflow, `.github/`, `.gitignore` or other-repository change; no new crate, build script, port, framework, renderer, route, locale rule, geocoder or graphics catalogue; no `R03.C` implementation; no Stage 03 closure; no ROADMAP or OUTLOOK edit and no NEXT edit beyond the authorized issuance clearing; no ACTIVE disposition change; no force-push, reset, clean, rebase, squash, history rewrite, branch deletion, tag, PR, merge commit, release or deployment; no secrets, provider, DNS or billing action; no second checkout or worktree. The Implementation Agent does **not** self-ACCEPT. Disposition now rests with the Chief Architect's independent review of this report at its published revision.
