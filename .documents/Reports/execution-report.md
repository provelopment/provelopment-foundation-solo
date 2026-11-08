# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08 |
| Work order | **`SOLO-WO-044 R01`** — `R02.A` One-Entity Ownership and Identity Domain Core, §§4.1–4.6 executed in one run |
| Owner handoff | Received, matching `SOLO-WO-044 R01` |
| Chief Architect disposition | **`ACCEPT`**, reviewed at GitHub HEAD `fca29dc6a03a9cbcca7d2e8dd46d8ba3030d33df`, tree `1e6949a7c2eed9da5d5b7a3fa3d9e507f73f7ab9` |
| Required starting HEAD / tree | `594f0e163aec6bec66550fe786365a341f0a4e48` / `b55d8c6d5e80170c050ba236cfb7938bd6ba0248` — both matched exactly |
| **Accepted source revision** | **`1d4d962294999a749d21b6d305ed45f9066f45a9`** / tree `cd5006bbce7d3002cff72d6ae143c6f1d688a44d` |
| Closure revision (ROADMAP + NEXT) | `4488e39e32a4bf57caba2538248c6e4182d24db7` / tree `33dc76631e4454fc7d2ac5790b0cede6b5e1efe6` |
| ACTIVE neutralisation revision | `490926f7bffd44d3b51d6f8ca13fdcca386db2e8` / tree `f65996598bbaae407836e76c8d9779d4f301f430` |
| **Final canonical report revision** | given in the chat handback after publication, never embedded here |
| **Status** | **`COMPLETE`** — R02.A accepted and closed; ACTIVE neutral; NEXT holds the single `R02.B` candidate; Stage 02 remains open |

## Authorization and closure record

Authorization came from the installed `OWNER-ISSUED` order **and** the Chief Architect's `ACCEPT` of the published evidence at the exact revision above. Nothing was self-approved, and §4.5/§4.6 ran only after that `ACCEPT`.

| Step | Result |
| --- | --- |
| §4.1 issuance | `01e6c1367988f6eb6f63100d552baa0d93fd4482` — the order stored byte-identically, nothing else staged; ACTIVE blob unchanged at `7305674f…` |
| §4.1 NEXT transition | `376901853f6f4400199f3f8c924b2e01bb006bea` — NEXT cleared to the retained one-line title, blob `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP untouched |
| §4.2–4.3 implementation | `2be7688` → `be847fd` → `1d4d962` (see the defect record below); accepted source revision `1d4d962` |
| §4.3 evidence report | published at `fca29dc6a03a9cbcca7d2e8dd46d8ba3030d33df`, blob `f6430a209a560115006bc6136386be43e04bc3ff` |
| §4.4 independent review | **`ACCEPT`** from the Chief Architect against that revision |
| §4.5 close R02.A | `4488e39e32a4bf57caba2538248c6e4182d24db7` — exactly one ROADMAP line deleted, NEXT populated with the single `R02.B` candidate, nothing else staged |
| §4.6 neutralise + report | `490926f7bffd44d3b51d6f8ca13fdcca386db2e8` — ACTIVE restored to its exact 40-byte heading, blob `ea5f20b736483c2e102456be1cea9104059fc81a`, published alone |

**§4.5 diff, verified line by line.** `git diff --numstat` reported `0 insertions, 1 deletion`; the unified diff contained exactly one removed line — `- **R02.A:** implement immutable, language-neutral ownership identities and cardinality; one Entity only, one or more Facets/Sites/Pages and at least one representation per published Page.` The Stage 02 heading, its prerequisite, the `R02.B` and `R02.C` bullets, the Stage 02 exit gate and all **twelve** remaining stages (Stage 02–13, one `Exit gate` each) are byte-identical. **Stage 02 is still open**, with `R02.B` and `R02.C` still to deliver.

**§4.5 NEXT content.** One bounded, non-authorizing `R02.B` candidate marked `READY`: the strict, versioned, single-manifest authority and provenance-safe candidate/source-selection contract already recorded in `ARCHITECTURE.md` §1.5–§1.6 and §6.2B–§6.2C, with its own acceptance and refusal criteria. `R02.B` was **not** implemented, no dependency, schema, route or architecture decision was taken, and NEXT confers no execution authority.

## Corrected test evidence — reconciling 127 with 134 / 135

The earlier evidence report stated that the workspace contained **127 tests**. That figure was wrong as a statement about execution and is corrected here rather than quietly restated.

- **What the raw all-target logs show.** **134 passing test executions on Ubuntu** and **135 on Windows**, including the **27 R02.A cases**. These are the authoritative execution counts.
- **Where 127 came from.** It was a *static* count of `#[test]` attributes found only at column 1 of files under `crates/`. That scan silently skipped indented attributes inside inline `#[cfg(test)]` modules and one indented case, so it undercounted and was never an execution count. Presenting it as "127 tests in total" was my error, not a gate failure.
- **Reconciliation now verified from source.** An any-indentation scan finds **136** `#[test]` attributes: `solo-adapters/tests/bootstrap_cli.rs` 11, `solo-application/tests/bootstrap_preflight.rs` 9, `solo-cli/src/main.rs` 8 (inline module), `solo-cli/tests/check_architecture_fixtures.rs` 25, `solo-cli/tests/ci_gate_tripwire.rs` 1, `solo-cli/tests/cli_smoke.rs` 15, `solo-cli/tests/workspace_graph.rs` 16, `solo-domain/tests/base_candidate.rs` 13, `solo-domain/tests/efs_ownership.rs` **27**, `solo-domain/tests/identity.rs` 11.
- **Why the hosts differ by one.** Host-conditional compilation removes cases that do not apply: `crates/solo-adapters/tests/bootstrap_cli.rs` line 147 carries `#[cfg(windows)]` immediately above a `#[test]`, and further `#[cfg(windows)]`/`#[cfg(unix)]` attributes gate helpers in `check_architecture_fixtures.rs` and `cli_smoke.rs`. 136 minus the cases excluded on Linux gives **134**; 136 minus the case excluded on Windows gives **135**, matching the raw logs exactly. The one-test host difference is therefore explained by deliberate platform gating, **not** by a missing or skipped gate — no step uses `continue-on-error`, none masks an exit code, and no test is `#[ignore]`d.
- **My own re-measurement limit.** This agent still cannot read log bodies (**HTTP 403**) or the rendered log (sign-in required), so the 134/135 execution counts are the Chief Architect's raw-log retrieval, and the source scan above is my independent reconciliation of them, not a substitute claim.


## Complete R02.A positive and negative ledger

`crates/solo-domain/tests/efs_ownership.rs` holds **27 `#[test]` cases**, all through the public API only. Locale values are deliberately supplied already-canonical fixture keys (`en-001`, `de-DE`, `id-ID`, `zh-Hans-CN`); that is a fixture convention for membership testing, never a claim that this model validates or canonicalizes a locale.

| # | Case | Expectation |
| --- | --- | --- |
| 1 | `builds_minimal_single_entity_graph` | one Facet → one Site → one Page → one key builds; traversal returns exactly those values |
| 2 | `builds_expanded_graph_with_multiple_facets_sites_and_pages` | 2 Facets × 2 Sites × Pages with 1–2 keys each build; every level and key verified |
| 3 | `refuses_empty_graph_and_invents_no_starter_seed` | `NoFacet` at Entity level; no `main`/`home` seeded |
| 4 | `refuses_facet_without_site` | `FacetWithoutSite` at Facet level |
| 5 | `refuses_site_without_page` | `SiteWithoutPage` at **Site** level |
| 6 | `refuses_page_without_representation` | `PageWithoutRepresentation` at Page level |
| 7 | `refuses_duplicate_facet_id_inside_the_one_entity` | `DuplicateFacetId` |
| 8 | `refuses_duplicate_site_id_within_one_facet` | `DuplicateSiteId` |
| 9 | `refuses_duplicate_page_id_within_one_site` | `DuplicatePageId` |
| 10 | `accepts_the_same_token_under_different_parents` | `main`/`docs` Sites in two Facets and `home` in four Sites all accepted |
| 11 | `accepts_the_same_token_at_different_levels` | `main` as Facet, Site **and** Page accepted |
| 12 | `validates_every_non_entity_identity_level` | 8 refusals × 3 levels: empty, non-ASCII, uppercase, leading hyphen, dot, trailing hyphen, doubled hyphen, 64 bytes |
| 13 | `accepts_and_refuses_the_identifier_length_boundary` | 63 bytes accepted, 64 refused, at all three levels |
| 14 | `identity_is_independent_of_labels_addresses_and_locale_fixtures` | changing display label, base address and locale fixture leaves every ID identical |
| 15 | `resolves_page_only_by_complete_lineage` | full four-level lineage resolves and returns the Page |
| 16 | `refuses_a_second_entity_and_unresolvable_ancestors` | `UnknownEntity`, `UnknownFacet`, wrong-facet Site and wrong-Site Page all refused |
| 17 | `resolves_representation_and_refuses_an_absent_locale_key` | declared keys resolve; undeclared `id-ID` → `UnknownRepresentation` |
| 18 | `refuses_duplicate_locale_key_on_one_page` | `DuplicateLocaleKey` |
| 19 | `accepts_the_same_locale_key_on_different_pages` | `en-001` on two Pages accepted |
| 20 | `refuses_bare_language_keys_without_completing_them` | `en`, `de`, `id`, `zh` → `NotFullLocale`, never completed |
| 21 | `refuses_malformed_locale_keys` | empty, trailing space, inner space, underscore, en dash, leading hyphen, trailing hyphen, doubled hyphen |
| 22 | `accepts_one_hundred_representations_and_refuses_one_more` | 100 accepted, 101 → `TooManyRepresentations` |
| 23 | `accepts_five_hundred_pages_and_refuses_one_more` | 500 accepted, 501 → `TooManyPages` |
| 24 | `refuses_a_graph_larger_than_the_accepted_node_budget` | 9,503 nodes accepted; 10,003 → `TooManyNodes`, no partial graph |
| 25 | `preserves_authored_order_instead_of_sorting_it` | Facets and keys returned in authored order, not lexical order |
| 26 | `refuses_duplicates_independently_of_declaration_order` | duplicate Facet ID refused at every position |
| 27 | `diagnostics_never_echo_authored_input` | four exact `code: detail` strings, none containing the authored value |

**What R02.A delivers.** `crates/solo-domain/src/efs_ownership.rs` is `std`-only, `unsafe`-free and adds no dependency: `FacetIdV1`/`SiteIdV1`/`PageIdV1` each delegate to `EntityIdV1::parse` so the accepted grammar exists once; `EfsOwnershipDraft` is the only root, so a second Entity is unrepresentable; `build()` validates the whole tree and returns a complete immutable graph or a typed refusal; uniqueness is scoped per level; qualified lookup needs the full `entity → facet → site → page` (+ locale) lineage; the accepted 10,000-node, 500-Page and 100-representation bounds are enforced with no partial result; and `LocaleKeyV1` is a membership key whose constructor never canonicalizes. `lib.rs` changed only by one module declaration, one `pub use` block and one doc-list entry.


## CI evidence — every revision verified on both hosts

| Revision | Role | Run / jobs | Ubuntu | Windows |
| --- | --- | --- | --- | --- |
| `1d4d962` | accepted source | [run `42499276090`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090) — jobs [`128788271268`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090/job/128788271268) / [`128788271030`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090/job/128788271030) | success 22 / 9 / 0 | success 19 / 12 / 0 |
| `fca29dc` | evidence report (reviewed) | [run `42499801715`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499801715) — jobs [`128789860469`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499801715/job/128789860469) / [`128789860754`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499801715/job/128789860754) | success 22 / 9 / 0 | success 19 / 12 / 0 |
| `4488e39` | **§4.5 closure** (ROADMAP + NEXT) | [run `42501119773`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42501119773) — jobs [`128793876244`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42501119773/job/128793876244) / [`128793876006`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42501119773/job/128793876006) | success 22 / 9 / 0 | success 19 / 12 / 0 |

Counts are `succeeded / skipped / failed`. Every skip is the mirrored host-conditional counterpart of a step that ran on the other host, plus Ubuntu-only steps; no step uses `continue-on-error`, masks an exit code or is written `|| true`. Gate coverage at each revision: locked fetch, `rustup --version`, `rustc`/`cargo` assertion, format, strict Clippy `-D warnings`, all-target workspace tests, the real-tree architecture checker, the 23-fixture refusal matrix, the injected `undeclared-crate` refusal with verified restore, the failure tripwire, the release build, the release CLI positive/refused smoke, Ubuntu two-clean-build byte equality, and the four-category security scan with its disposable advisory negative probe.

Annotations at every green revision are **informational only**: the injected-import refusal notice, the `RUSTSEC-2026-0009` probe notice and the `cargo-deny` component-gate notice per host. No failure or warning annotation exists.

## Component graph, currency and advisories — unchanged by this closure

**15 locked packages, byte-identical to the accepted baseline:** 4 workspace crates (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`, all `0.1.0`) and 11 registry crates used only by test/check tooling: `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `zmij 1.0.23`. `solo-domain` declares no dependencies; `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml` and `.cargo/config.toml` are unchanged, and **no dependency was added, removed or upgraded by this closure**.

| Check | Result |
| --- | --- |
| crates.io latest stable, all 11 | **all `LATEST`** at `2026-11-08T16:23:43Z` |
| OSV query of all 11 exact versions | **0 vulnerabilities** at `2026-11-08T16:23:29Z` |
| RustSec advisory database, hosted | `309392b18f6b17022f82c701f300433126901068`, refreshed per host at each run |
| `cargo-deny 0.20.2`, four categories | `advisories`, `bans`, `licenses`, `sources` **ok** on both hosts |
| Disposable negative probe | `time 0.3.46` with `RUSTSEC-2026-0009` **refused**, exit 1, while the unchanged graph passed |
| `actions/checkout` | pinned `v7.0.1` by SHA — still the latest release |
| Toolchain | `1.99.1` unchanged; Rust publishes no separate LTS track, so none is preferred or owed |
| `rustup` installer | **`NOT RE-MEASURED`** — see the access limits below; the accepted baseline stands: Ubuntu `1.29.2`, Windows `1.29.3`, non-blocking |

**Warning classification:** zero compiler, Clippy or rustfmt warnings at every green revision, zero failure-level annotations, and no warning suppressed or downgraded. **`unicode-ident 1.0.27` carries Unicode-3.0 attribution work that remains an open future-release prerequisite and is not waived by R02.A or by this closure.**


## Run history preserved — the red revisions are not hidden

R02.A did not go green on the first attempt, and the record stays intact:

| Revision | State | Cause |
| --- | --- | --- |
| `2be7688` | both hosts **failed** step 16 | the test file used `unreachable!`, which is **not** on the 21-entry audited macro allowlist enforced by the `syn` source-boundary checker, so the workspace source gate refused it; a real leading-hyphen locale gap was also present |
| `be847fd` | both hosts **failed** step 16 | macro removed and the leading hyphen refused; one genuine assertion then failed: `SiteWithoutPage` was mapped to `EfsLevel::Page` instead of `Site` |
| `9b1a288` | both hosts **failed**, as intended | a temporary `::error::` diagnostic written straight to stderr, bypassing harness capture, so the runner published the failing case as an annotation — it named `refuses_site_without_page` on both hosts in a single run |
| `1d4d962` | **both hosts green** | level mapping corrected; the diagnostic removed in the same commit; the 27 cases unchanged apart from the assertion that caught the defect |

Two commit **subjects** in this order (`3769018`, `9b1a288`) contain a literal `` `n`n `` sequence because a PowerShell escape did not expand when the message was built. Both are published and rewriting history is forbidden, so the defect is disclosed rather than hidden; every later message is clean.

The diagnostic commit was a deliberate accepted cost: with log bodies at 403 there was no other legitimate way to learn which case failed, and guessing would have cost an unknown number of blind red cycles. It changed no assertion, no product code and no gate, and it does not exist at the accepted revision.

## Access limits, never converted into a pass

**This agent could not read:** Dependabot and code-scanning alert APIs (**HTTP 401**); job log bodies (**HTTP 403**); the rendered job log (sign-in required). Therefore the RustSec/`cargo-deny`/probe evidence above comes from the workflows' own published annotations of successful runs, the OSV and crates.io results are live API reads, the **134/135 test-execution counts are the Chief Architect's raw-log retrieval**, and the `rustup` versions remain **`NOT RE-MEASURED`** at Ubuntu `1.29.2` / Windows `1.29.3` as accepted.

The scheduled Dependabot run remains **`NOT OBSERVED`**. Browser, HTTP delivery, rendered pages, contact form, accessibility conformance, release and deployment remain **`NOT APPLICABLE`** to this pure-domain package and are not claimed. Local test execution was **`NOT RUN`**: this host has no MSVC, mingw or clang linker, and acquiring one is a new component the order forbids; `cargo fetch --locked`, `cargo fmt`, `cargo check -p solo-domain --all-targets` and `cargo clippy -p solo-domain --all-targets -- -D warnings` were run locally and were clean, while all workspace-wide gates are hosted-only evidence.

## Scope discipline, cleanup and final state

**Changed paths against the accepted baseline `594f0e16`, six only:** `.documents/Reports/execution-report.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-roadmap.md`, `crates/solo-domain/src/efs_ownership.rs` (new), `crates/solo-domain/src/lib.rs`, `crates/solo-domain/tests/efs_ownership.rs` (new). `workorder-active.md` no longer appears because it is back to its baseline blob. `ARCHITECTURE.md`, both SOPs, `plan-outlook.md`, `README.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, every other `.rs` file, `.github/` and the licences are byte-identical to the baseline.

**WIP purity:** exactly four files, no subdirectory, no archive, worklist or second report. `plan-roadmap.md` `18975332…` (14,773 bytes, `R02.A` gone, Stage 02 open with `R02.B`/`R02.C`); `plan-next.md` `012c5c1a…` (2,033 bytes, single `R02.B` `READY` candidate); `plan-outlook.md` `cb9c7a61…` untouched; `workorder-active.md` `ea5f20b7…` (40 bytes, neutral). Empty ACTIVE means only that no work is authorized — not a project pause or a reopening gate.

**Cleanup:** the disposable `.temp/w044` toolchain tree (1,133 MB), its temporary `x86_64-pc-windows-gnu` and `x86_64-pc-windows-gnullvm` targets, all nine OS-temp commit-message files and every scratch file were removed; no `target/` directory ever appeared in the worktree because `CARGO_TARGET_DIR` was redirected; pre-existing `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` were preserved untouched. No secrets, credentials or personal data were read or written; no destructive Git operation, force-push, amend, rebase, merge, PR, tag, release or deployment occurred; no other repository was touched.

## Conclusion and what remains

**`SOLO-WO-044 R01` is complete.** `R02.A` is implemented at `1d4d962`, independently accepted, closed out of Stage 02, and verified green on Ubuntu and Windows at the source, evidence and closure revisions; the order is neutralised and the single `R02.B` candidate stands ready. Stage 02 remains **open** with `R02.B` (single validated manifest and provenance-safe source selection) and `R02.C` (default seed and expanded synthetic fixtures) still to deliver.

Nothing beyond that is authorized by this report. `R02.B` was not implemented, Stage 02 was not closed, Architecture was not changed, no dependency was added and no further work order was begun. `R02.B` needs its own Owner-issued order and matching handoff, and NEXT is reference only.

**`agent report ready`**
