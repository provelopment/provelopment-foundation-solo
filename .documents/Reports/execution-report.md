# Execution Report

**Order:** `SOLO-WO-056 R01` - implement `R03.A` five-level typed property registry, direct-ancestry resolution, provenance and safe public projection.
**Date:** 2026-11-10.
**Root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified `git rev-parse --show-toplevel`; single checkout, single worktree, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`).
**Disposition: `COMPLETE`** - every in-scope behaviour of this order is implemented and verified. This is **not** a self-acceptance: `R03.A` awaits the Chief Architect's independent `ACCEPT / REVISE / STOP`. `R03.B` and `R03.C` remain unimplemented, Stage 03 remains OPEN, ACTIVE was not cleared and `ROADMAP` was not advanced.

## 1. Revision ledger

| Item | Value |
| --- | --- |
| Starting `origin/main` | `8a0365ca5b14628448ff698b6b46c67090537f6c` (tree `4dde7d8b2cc815119cd6ffcf4f73610c62b5bf13`) |
| Authorization transition (`ACTIVE` + cleared `NEXT`) | `c087a4503eee571ec233e262dc5e0d4247445912` (tree `0d05b21ad2c12084e14aa1fdc0270477e0e197a2`) |
| Implementation branch | `solo-wo-056-r01` (one branch in the existing checkout; no PR, no second checkout) |
| Branch commit 1 - domain | `246180d459985e8cdb320b96e1ea900075a973f9` |
| Branch commit 2 - application | `0d224b6c2d5f6e41561036cc631810b8998b7b71` |
| Branch commit 3 - adapters + end-to-end tests | `efedd199343e360edbb995b0619d7420e1742c7e` |
| Branch commit 4 - small typed error | `13e1f1f663f81f08f7a3b29bd1e0831ac33d6c13` |
| Branch commit 5 - test helper naming | `2c780264ea0ef30383cebba7933b8fb4483d6123` |
| Branch commit 6 - keyed refusals + inheritance events | `5a4a7b2c71b2ede382a1c9e1c4386ab9c47fcf70` |
| **Substantive `main`** (fast-forward only) | `5a4a7b2c71b2ede382a1c9e1c4386ab9c47fcf70` (tree `7f3510a9cd184766bafb870a353f00a53651c852`) |
| Report-only commit | this file's own commit - SHA returned in chat only, never embedded here |

`git merge --ff-only solo-wo-056-r01` produced `c087a45..5a4a7b2` with no merge commit, no squash and no rebase. `main == origin/main` was verified after the push. The branch is **retained** for separate Owner-authorized later pruning.

## 2. Changed paths (authorized ledger only)

Eleven tracked paths changed, all **pure additions** (`5646` insertions, `0` deletions):

| Path | Delta |
| --- | --- |
| `crates/solo-domain/src/property_registry.rs` | +1604 (new) |
| `crates/solo-domain/src/property_resolution.rs` | +771 (new) |
| `crates/solo-domain/src/lib.rs` | +19 (module declarations and re-exports only) |
| `crates/solo-domain/tests/property_resolution.rs` | +1446 (new) |
| `crates/solo-application/src/property_resolution.rs` | +142 (new) |
| `crates/solo-application/src/lib.rs` | +5 (module declaration and exports only) |
| `crates/solo-application/tests/property_resolution.rs` | +278 (new) |
| `crates/solo-adapters/src/property_resolution.rs` | +684 (new) |
| `crates/solo-adapters/src/publication_manifest.rs` | +12 (read-only `InertPropertiesV1::entries()` only) |
| `crates/solo-adapters/src/lib.rs` | +5 (module declaration and exports only) |
| `crates/solo-adapters/tests/property_resolution.rs` | +680 (new) |

Plus the two WIP files in the authorization commit: `.documents/WIP/workorder-active.md` (the Owner-installed order, blob `413359d7`) and `.documents/WIP/plan-next.md` (cleared back to its neutral title-only content, blob `92fa562b`).

**Frozen and verified unchanged:** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.github/` (including the unchanged `rust-verify` workflow), `crates/solo-cli/`, all existing R01/R02 source modules, tests and fixtures, the starter seed and expanded fixtures, `README.md`, `AGENTS.md`, `.documents/About/`, `.documents/Architecture/`, both SOPs, `LICENSE`, `.documents/WIP/plan-roadmap.md` (blob `4c7ef021`) and `.documents/WIP/plan-outlook.md` (blob `cb9c7a61`). No crate, dependency, schema, asset, server, frontend or route engine was added.

## 3. What was implemented

**Registry (`solo-domain::property_registry`).** Exactly the nine accepted keys (`location`, the five `graphic.*` roles, `seo.title`, `seo.description`, `contact.email`) at `propertyRegistryVersion == "1.0.0"`, with allowed levels (`seo.*` refuses at Entity and Facet), effective maxima (`location` 16, `graphic.background` 4, all others 1) and closed, explicitly exposed value types: single-line plain text (no control characters, no markup delimiters, scalar bounds 140/300), one safe addr-spec mailbox, a typed asset reference (`entity|platform` namespace, portable relative path, `png/jpg/jpeg/webp/avif`, PNG-only favicon, no remote URL, no path escape, no active SVG/GIF), and section 6.2H location facts (closed `LocationFields`, uppercase ISO alpha-2 `countryCode`, finite in-range WGS84 coordinates, technical-ID `factId`, `association|premises`). Declarations are closed envelopes with exactly `inheritance` and `currentContext`, where the literal `"none"` is modelled separately from an absent declaration, and a refused declaration carries its registered key.

**Resolution (`solo-domain::property_resolution`).** A validated input binds every declaration to a node of the accepted one-Entity `EfsOwnershipGraph` (a wrong Entity, wrong Facet/Site/Page, sibling branch or undeclared locale refuses), then a direct-ancestry resolver runs: absent carries the parent's effective set, explicit `inherit` carries and appends, explicit `suppress` clears and appends, `"none"` adds nothing. The registry maximum and uniqueness rules are enforced at **each effective level before** public filtering; distinct location facts are identified by `(originOwner, factId)`; a repeated background source or repeated original fact identity refuses; a recognized `location` `extend` refuses by name. Provenance is exactly the section 6.2G.6 shape (`values` with `data`, `exposure`, `originOwner`, `originLocalIndex`, optional location `fieldOrigins`; `resolutionEvents` with `owner`, `operation`, `localIndex`, optional `targetFactId`).

**Public projection (`project_public_properties_v1`).** A separate entry point and type that emits only `public` values in effective order, with no owner, exposure, origin, field origin or resolution event, and with no ancestor retry: a filtered-away or suppressed value stays absent.

**Application (`solo-application::property_resolution`).** One inbound use case over the validated graph and typed declarations returning a candidate that carries the bound input, the full internal record and the projection, plus `PROPERTY_CANDIDATE_GRANTS_PUBLICATION_AUTHORITY = false`. No outbound port, no I/O, no adapter or JSON import.

**Adapter (`solo-adapters::property_resolution`).** A read-only `InertPropertiesV1::entries()` accessor and `validate_manifest_property_candidate_v1`, which walks the parsed Entity containment, translates every authored `properties` object for the same owner (Entity, Facet, Site, Page and each declared representation, listed or unlisted) into typed declarations, and drives the application use case. `parse_manifest_bytes` is neither renamed nor promoted and stays inert.

### Two design decisions worth review

1. **`inherited` events are recorded only when a declaration actually carries inherited values.** A root declaration has nothing to inherit, so an Entity-level `inherit` records its own `added` events alone; a descendant that inherits a non-empty set records `inherited` (with `localIndex: null`) before its own `added` events. This keeps the event stream explanatory instead of emitting an `inherited` event with nothing inherited. Absent versus explicit `inherit`/`"none"` remains distinguishable in the typed model itself (declaration present/absent plus the `"none"` literal), which is what the order requires.
2. **Representations of the same Page share that Page's properties.** A Page-level value reaches every one of its locales; `id-ID` therefore inherits a value authored at the Page, while a Page under a different Site or Facet does not. Isolation is between Sites, Facets and Pages, never between locale *membership keys* of one Page.

## 4. Mandatory acceptance matrix (`R03A-01` - `R03A-12`)

26 new tests at the three authorized paths (domain 12, application 5, adapters 9); every case below names the test that proves it and the assertion it makes. All of them executed green in runs `42853919842` (branch) and `42855602900` (substantive `main`).

| Case | Evidence |
| --- | --- |
| `R03A-01` Registry and closed encoding | domain `r03a_01_registry_declares_exactly_the_nine_accepted_keys` asserts the nine exact names, `PROPERTY_REGISTRY_VERSION == "1.0.0"`, `from_name` exactness (`SEO.Title`, `seo.keywords`, `custom` -> `None`), `location` 16, `graphic.background` 4, every other key 1, and `seo.*` false at Entity/Facet but true at Site/Page/representation. domain `r03a_01_closed_encoding_refuses_wrong_level_and_unsafe_values` asserts `WrongLevel` for Entity-level `seo.title`, `DuplicatePropertyEntry`, `WrongEntryShape`, `FaviconNotPng`, `EmptyPlainText`, `ActiveMarkup`, `ControlCharacter`, `PlainTextTooLong` and nine malformed mailboxes. adapters `r03a_09_closed_envelope_forms_refuse` asserts `InvalidCurrentContext` for `null`, `EmptyContextArray` for `[]`, `InvalidInheritance` for `"maybe"`, `MissingEnvelopeField`, `ExtraEnvelopeField`, and that an explicit `suppress`/`"none"` is still valid. |
| `R03A-02` Five-level ancestry | domain `r03a_02_five_levels_inherit_only_direct_ancestors` authors `graphic.background` at all five levels, asserts four effective values with origins `[entity, facet, site, page]`, then changes only the Site value and shows the bali branch changes while the jakarta branch stays `[entity, facet]`. application `application_resolves_all_five_levels_without_publication_authority`. adapters `r03a_09_real_manifest_bytes_resolve_all_five_levels` does the same through real JSON bytes. |
| `R03A-03` Explicit empty and suppression | domain `r03a_03_absent_and_explicit_none_are_distinct_and_suppression_is_final` asserts: absent Page -> `declaration_for` is `None` and events `[Added]`; explicit `inherit`/`"none"` -> declaration present with `is_none_literal()` and events `[Added, Inherited(null)]`; `suppress`/`"none"` -> zero effective values, events `[Added, Suppressed]`, and zero public values; `suppress` + local value -> one value owned by the Page; a downstream `inherit` never restores the suppressed ancestor. |
| `R03A-04` Ordering and provenance | domain `r03a_04_ordering_provenance_and_field_origins_are_exact` asserts inherited-first ordering, exact `originOwner`/`originLocalIndex` per value, the internal exposure of the second fact, `fieldOrigins` naming the adding Site with its array index for `countryCode` and `locality`, no field origins on a non-location value, the exact event stream `[Added(0), Added(1), Inherited(null), Added(0)]` with owners, and `resolved == resolved` on repetition. adapters `r03a_09_real_manifest_bytes_resolve_all_five_levels` asserts the same origins and field origins from authored JSON. |
| `R03A-05` Effective cardinality | domain `r03a_05_effective_cardinality_is_enforced_before_public_filtering` asserts `CardinalityExceeded` for inherited title + local `inherit` title, for the same conflict where the ancestor is `internal`, for a fifth `graphic.background` layer, for a seventeenth `location` fact, and `TooManyLocalEntries` for a seventeen-entry local array. adapters `r03a_09_effective_cardinality_and_duplicates_refuse` proves the same through the typed refusal at the adapter boundary. |
| `R03A-06` Duplicate and typed refusal | domain `r03a_06_duplicate_and_malformed_entries_refuse_without_repair` asserts `DuplicateGraphicSource`, `DuplicateLocationFact`, `InvalidAssetReference` (`../escape.png`, remote URL, drive path, missing extension), `UnsupportedAssetExtension` (`svg`, `gif`, upper-case `PNG`), `InvalidFactId`, `InvalidCountryCode` (`id`, `IDN`), `InvalidLocationField`, `InvalidCoordinates` (91, -181, NaN) and `EmptyLocationFields`. adapters `r03a_09_closed_typed_entries_refuse` repeats thirteen such refusals from authored JSON, including `InvalidExposure`, `ExtraEntryField`, `InvalidAssetNamespace`, `UnknownLocationField` and `InvalidLocationKind`. |
| `R03A-07` Exposure | domain `r03a_07_public_projection_omits_internal_values_and_cannot_resurrect` asserts one effective internal title with `public_value_count() == 0`, an empty public projection whose debug text contains neither the internal text nor `originLocalIndex`, no resurrection of a suppressed public ancestor, and an explicit `suppress` + public replacement published in its own place with the Page as origin. application `application_public_projection_omits_internal_values`. |
| `R03A-08` Qualified isolation | domain `r03a_08_identical_page_ids_under_different_sites_and_locales_never_mix` uses three Pages all named `home`; the bali `en-001` representation sees the value, the jakarta Site's `home`, the corporate Facet's `home` and the Site above the Page do not, and the same Page's `id-ID` representation inherits the Page-level value with the Page as origin. domain `r03a_08_wrong_lineage_and_second_entity_refuse` asserts `UnknownOwner` for an undeclared locale and a wrong Page-under-Site lineage, `DeclarationOwnerUnknown` for a second Entity and for a cross-Facet Site, and `DuplicateDeclarationOwner`. application `application_refuses_an_owner_outside_the_graph` and `application_refuses_a_declaration_owner_outside_the_graph`. |
| `R03A-09` Actual adapter boundary | adapters `r03a_09_real_manifest_bytes_resolve_all_five_levels` starts from real strict JSON with nested inline `properties`, asserts `parsed.is_publication_authorized() == false`, runs `parse_manifest_bytes` -> `validate_manifest_property_candidate_v1` -> application/domain -> public projection, and checks all five owners, `declared_owner_count() == 5`, `candidate.is_publication_authorized() == false` and nine resolved records. It is joined by `r03a_09_unknown_property_refuses_by_category` (`PROPERTY_UNKNOWN_KEY`, owner in the message, authored name never echoed), `r03a_09_wrong_level_declaration_refuses` (`PROPERTY_DECLARATION_REFUSED` with `Some(SeoTitle)` and the Entity owner), `r03a_09_closed_envelope_forms_refuse`, `r03a_09_closed_typed_entries_refuse`, `r03a_09_unlisted_sibling_declaration_fails_closed` and `r03a_09_committed_starter_seed_stays_inert`. The unchanged R02.C fixture suite (`r02c_seed`, 17 cases) stayed green in the same runs. |
| `R03A-10` Deferred specialists remain honest | domain `r03a_10_recognized_extend_is_refused_by_name_while_add_works` proves the declaration is accepted (`validate_declared_properties_v1` succeeds, `has_recognized_location_extension()` is true) and the resolution returns `LocationExtensionNotImplementedInR03A` with code `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A`, while an `add`-only input resolves one fact with its `factId`, kind and set fields. application `application_surfaces_the_named_extension_refusal`. adapters `r03a_09_recognized_extend_is_refused_by_name` proves the same from authored JSON and asserts the refusal is *not* a declaration refusal. No asset file, catalogue, fallback, HTTP output or release is claimed. |
| `R03A-11` Invariants and security | domain `r03a_11_property_resolution_leaves_identity_untouched` asserts the Entity id, facet count, Page id and locale key are unchanged after resolving, that the internal replacement is the only effective value and stays internal, and that neither the internal nor the suppressed ancestor mailbox appears in the public projection. adapters `r03a_09_unlisted_sibling_declaration_fails_closed` proves fail-closed behaviour: one invalid declaration on an **unlisted** representation refuses the whole candidate, and the message names the qualified owner and key without echoing the authored text. |
| `R03A-12` Existing baseline and CI | All existing R01/R02 suites, the architecture checker with its refusal self-test, the unsafe-boundary and workspace-graph checks, the deliberate tripwire, the Linux Btrfs fixture and its cleanup, the Windows native guards, the locked release build, the release CLI smoke and the two byte-identical builds ran unchanged and green on both designated hosts at the substantive SHA (`42855602900`) and the branch SHA (`42853919842`). |

**Mandated distinction honoured:** the five-level trace uses `contact.email` and `graphic.background`, never `seo.title`; `seo.title` carries the Site -> Page -> representation singular/suppression case, and an Entity-level or Facet-level `seo.title` is a mandatory refusal (`r03a_01_closed_encoding_refuses_wrong_level_and_unsafe_values`, `r03a_09_wrong_level_declaration_refuses`).

## 5. CI evidence (unchanged `rust-verify` workflow)

| Run | Revision | Result |
| --- | --- | --- |
| `42850427568` | `c087a450` authorization transition (`main`) | Ubuntu success, Windows success |
| `42850442030` | `efedd199` first branch attempt | **failure** - strict lint: `clippy::result_large_err` at four adapter functions (the refusal type embedded a full qualified owner) |
| `42851850519` | `13e1f1f6` after boxing the refusal owner | **failure** - test compile: `E0618` because the test helper `parsed` was shadowed by `let parsed = ...` |
| `42852757877` | `2c780264` after renaming the helper | **failure** - four test assertions: `inherited` recorded at a root declaration, a missing key on a wrong-level refusal, and two expectations of mine that wrongly denied Page-level inheritance to the same Page's other locale |
| `42853919842` | `5a4a7b2c` verified branch head | Ubuntu success, Windows success, **0 failed steps**, 33 steps, skips 9 / 14 |
| `42855602900` | `5a4a7b2c` substantive `main` | Ubuntu success, Windows success, **0 failed steps**, skips 9 / 14 |
| this report's own run | report HEAD | dispatched and verified separately; SHA returned in chat |

At `42855602900` and `42853919842` every host-specific step passed on its designated host: pinned toolchain install and exact version assertions, locked package fetch, the `cargo-deny 0.20.2` component gate with a fresh RustSec refresh, the advisory negative probe (`RUSTSEC-2026-0009`, Ubuntu), the workspace package-set gate, the disposable Linux Btrfs snapshot filesystems with their guaranteed cleanup, format check, strict lint, the full locked all-target workspace test run, the architecture checker on the real checkout with its refusal self-test, the deliberate test-failure tripwire, the locked release build, the release CLI smoke, the forbidden-import/bypass refusal with guaranteed restore, and the two byte-identical clean release builds. Skipped steps are exactly the counterpart host's variants (9 on Ubuntu, 14 on Windows); no applicable check was skipped and no material warning was silenced. The three intermediate failures above are disclosed rather than hidden: each was diagnosed from the run log, fixed in a separate commit, and re-verified.

All three new binaries actually executed in the green runs: `Running tests/property_resolution.rs` appears three times (domain 12 cases, application 5, adapters 9) alongside the unchanged `r02c_seed` suite.

## 6. Dependency, security and version-currency findings

No dependency was added, removed, upgraded or downgraded: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` and `deny.toml` are byte-identical to the reviewed baseline, and `git diff` for those paths is empty. The new domain and application code uses the standard library only, so the inward dependency direction and the provider/framework independence of the core are unchanged. The adapter uses only the already-declared `serde`/`serde_json` roots and adds no JSON parsing to the core.

Pinned and unchanged: Rust `1.99.1` (`rustfmt`/`clippy`), `serde 1.0.229`, `serde_json 1.0.154`, `rustix 1.1.5` (Linux), `windows-sys 0.61.2` (Windows), `cargo-deny 0.20.2`. The security gate and the fresh RustSec refresh in the passing runs reported no advisory against the locked graph, and the deliberate advisory negative probe still fails as designed on Ubuntu, proving the database is live. A1 version-currency was therefore satisfied by *not* changing the graph: upgrading a dependency, toolchain component or pin is outside this order's permission and did not happen.

## 7. Verification limits, scratch and final state

**Local, on the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain:** `cargo fmt --all -- --check` clean; `cargo check --locked -p solo-domain --all-targets` and `-p solo-application --all-targets` clean; `cargo clippy --locked -p solo-domain --all-targets -- -D warnings` and the same for `solo-application` clean. **NOT RUN locally:** `cargo clippy --workspace` and `cargo test` - this workstation has no MSVC `link.exe`, so `serde`'s build script cannot be linked and the `solo-adapters` crate plus every test binary can only be compiled and executed in CI. The adapter code and all three test suites are therefore verified by the two-host runs above, not by a local run; nothing is inferred from an unexecuted local command.

**Scratch and cleanup.** Solo `.temp/` was not used. The local build cache `target/` (created by the local `cargo` runs, untracked and not covered by this repository's `.gitignore`) was deleted before the final state check; no tracked path was affected. The Cline task temporary area holds only tool-generated command logs from this run; no Solo material was written there, nothing tracked is affected, and no unknown or Owner-owned material was deleted.

**Final state at the report commit.** `git status --porcelain` empty; one checkout at the Owner's root; one worktree; sole `origin`; local and remote `main` identical; branches `main` plus the retained `solo-wo-056-r01`; the four WIP files unchanged from the authorization revision (ACTIVE still holds the issued order, NEXT is title-only, ROADMAP and OUTLOOK untouched).

## 8. Scope boundaries and honest status

**Implemented and verified:** the `R03.A` typed registry, the five-level direct-ancestry resolver with internal provenance, the separate validated public projection, the application inbound use case and the adapter translation with an end-to-end real-JSON path.

**Deliberately not implemented, and not claimed:** `R03.B` location `extend` (recognized as accepted syntax and refused with `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A`; no in-place identified-record mutation, no field-level extension provenance); `R03.C` graphics assets, catalogue, fallback and delivery (only the typed *declaration* reference is validated); routing, locale canonicalization, visibility decisions, HTML/browser output, a live site, deployment and release. No whole-product or full `O08`/`O09` conformance is claimed.

**WIP authority untouched beyond the authorized transition:** `ROADMAP` still lists Stage 03 with `R03.A` as its first package, Stage 03 remains OPEN, Stage 04-13 are unchanged, `NEXT` remains the neutral title (no candidate selected), and ACTIVE was not cleared. The Implementation Agent did not self-ACCEPT, did not advance any stage and did not issue a next order; the Chief Architect's independent review of exactly `5a4a7b2c` and this report decides what happens next.





