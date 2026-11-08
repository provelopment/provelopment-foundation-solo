# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, from approximately 16:00 |
| Work order | **`SOLO-WO-044 R01`** — `R02.A` One-Entity Ownership and Identity Domain Core |
| Matching handoff | Received — `SOLO-WO-044 R01` |
| Required starting HEAD / tree | `594f0e163aec6bec66550fe786365a341f0a4e48` / `b55d8c6d5e80170c050ba236cfb7938bd6ba0248` — **both matched exactly** |
| Issuance commit / tree | `01e6c1367988f6eb6f63100d552baa0d93fd4482` / `ed6844340676323cd57a8ff7eba699b9f3b6e33d` |
| NEXT transition commit / tree | `376901853f6f4400199f3f8c924b2e01bb006bea` / `375d5d6faa1b24f142f6cda5b5b67a959cc37571` |
| **Accepted source revision / tree** | **`1d4d962294999a749d21b6d305ed45f9066f45a9`** / **`cd5006bbce7d3002cff72d6ae143c6f1d688a44d`** |
| Source blobs at that revision | `efs_ownership.rs` `c9bb27518963ee0eb8b7ec4c12cc834851d169fc`; `lib.rs` `28110d1ba11f31eea668eab79ad90010eac05a8f`; `tests/efs_ownership.rs` `c2a152eb55dd7e0db58459550bead34dcb79e80f` |
| **Final report revision** | given in the chat handback after publication, not embedded here |
| **Status** | **`COMPLETE` (evidence phase only)** — R02.A implemented and green on both hosts; package closure **not** performed; `ACCEPT` is **not** self-awarded |

**Deliberately not done, pending the Architect's independent review of this exact revision:** the `R02.A` bullet is still in Stage 02, NEXT holds no `R02.B` candidate, and ACTIVE still carries the issued order. §4.5 and §4.6 of the order remain unexecuted.

## Authorization and preflight

The order was found `OWNER-ISSUED` with a matching handoff whose text reproduced the delivered handoff exactly. Preflight confirmed the intended root as the repository top level, branch `main`, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, **exactly one worktree, zero stash**, no nested or shadow checkout, and the required starting HEAD and tree above.

**Sixteen baseline blobs:** fifteen matched exactly; the sixteenth, `.documents/WIP/workorder-active.md`, is the order's own expected replacement (`7305674f24cfa202aa115703c3e0e8d8b3fc0b9c`, 20,566 bytes) and is the only preflight tracked difference. The two new `efs_ownership.rs` paths did not exist. The WIP directory held exactly four files and no subdirectory.

## What R02.A implements

`crates/solo-domain/src/efs_ownership.rs` is a `std`-only, `unsafe`-free pure-domain model; `solo-domain` still declares **zero dependencies**.

- **Identity without a second grammar.** `FacetIdV1`, `SiteIdV1` and `PageIdV1` each delegate to `EntityIdV1::parse` through one private helper, so the accepted `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` / 63-byte rule exists exactly once in the crate. Nothing is repaired, case-folded, slugified, truncated or inferred. `EntityIdV1` itself is reused unchanged.
- **Containment is parentage.** `EfsOwnershipDraft::new(entity)` is the only root, so a second Entity is unrepresentable rather than merely refused. Facets contain Sites, Sites contain Pages, Pages contain representation keys; there is no parent field, discriminator or second graph.
- **Build-or-refuse.** `build()` validates the whole tree and returns `EfsOwnershipGraph` or `EfsGraphError`. The published graph exposes no mutator, no interior mutability and no draft accessor, so checked cardinality cannot be invalidated afterwards. Traversal preserves authored child order.
- **Parent-scoped uniqueness.** BTree sets scoped per level, so `main` may repeat under different parents and across levels.
- **Qualified lineage.** `resolve_page`/`resolve_representation` take the complete `entity → facet → site → page` (+ locale) lineage and return typed `EfsLineageError` kinds; a wrong Site or Facet is reported as unresolved rather than distinguished from absence.
- **Locale keys are membership keys.** `LocaleKeyV1::from_already_canonical` checks structural completeness only — non-empty, ASCII, `[A-Za-z0-9-]`, no leading/trailing/doubled hyphen, and at least one subtag separator so a bare `en`/`de`/`id`/`zh` cannot become an authored key. It never canonicalizes. The BCP47/Unicode 18 validator remains future work, and no raw authored string becomes publishable through this model.
- **Accepted bounds enforced.** ≤10,000 EFS nodes (Entity and representations included), ≤500 Pages, ≤100 representations per Page, refused deterministically with no partial result.
- **No seed.** An empty draft is refused; `main`/`main`/`home` is never manufactured — that belongs to `R02.C`.

`lib.rs` changed only by one module declaration, one `pub use` block and one doc-list entry; the two new `efs_ownership` paths are the only additions.

## Complete positive and negative R02.A ledger

`crates/solo-domain/tests/efs_ownership.rs` holds **27 `#[test]` cases**, all through the public API only. The workspace now contains **127 tests** in total (100 pre-existing, unchanged).

| # | Case | Expectation |
| --- | --- | --- |
| 1 | `builds_minimal_single_entity_graph` | one Facet → one Site → one Page → one key builds; traversal returns exactly those values |
| 2 | `builds_expanded_graph_with_multiple_facets_sites_and_pages` | 2 Facets × 2 Sites × Pages with 1–2 keys each build; every level and key verified |
| 3 | `refuses_empty_graph_and_invents_no_starter_seed` | `NoFacet` at Entity level; no `main`/`home` seeded |
| 4 | `refuses_facet_without_site` | `FacetWithoutSite` at **Facet** level |
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
| 16 | `refuses_a_second_entity_and_unresolvable_ancestors` | `UnknownEntity`, `UnknownFacet`, wrong-facet Site, wrong-Site Page all refused |
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

## Exact commands and outputs

Run on the Owner host with the approved `1.99.1` toolchain fetched into disposable `.temp/w044`, `CARGO_TARGET_DIR` redirected there, `CARGO_NET_OFFLINE=true` for every offline step.

| Command | Result |
| --- | --- |
| `cargo fetch --locked` | fetched the unchanged locked graph; 15 packages |
| `cargo fmt -p solo-domain` then `cargo fmt -p solo-domain -- --check` | **clean**, no diff |
| `cargo check -p solo-domain --all-targets --locked` | **finished**, no warnings |
| `cargo clippy -p solo-domain --all-targets --locked -- -D warnings` | **finished**, zero warnings |
| `cargo test -p solo-domain --locked` | **`NOT RUN` locally** — `error: linker 'link.exe' not found`; this host has no MSVC, mingw or clang toolchain, and acquiring a linker is a new component the order forbids. Test **execution** evidence is therefore the hosted two-host result below, not a local claim. |
| workspace build scripts (`serde`, `syn`, …) | `NOT RUN` locally for the same reason; the workspace-wide gates are hosted-only |

Hosted, at the accepted source revision `1d4d962`: **run [`42499276090`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090)** — [Ubuntu job `128788271268`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090/job/128788271268) **success 22 succeeded / 9 skipped / 0 failed**; [Windows job `128788271030`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42499276090/job/128788271030) **success 19 / 12 / 0**. Every skip is the mirrored host-conditional counterpart of a step that ran on the other host, plus Ubuntu-only steps; no step uses `continue-on-error`, masks an exit or is written `|| true`.

Gate coverage at that revision: locked fetch, `rustup --version`, `rustc`/`cargo` assertion, format, strict Clippy `-D warnings`, all-target workspace tests, the real-tree architecture checker, the 23-fixture refusal matrix, the injected `undeclared-crate` refusal with verified restore, the failure tripwire, the release build, the release CLI positive/refused smoke, Ubuntu two-clean-build byte equality, and the four-category security scan with its disposable advisory negative probe.

Annotations at that revision are **informational only**: the injected-import refusal notice, the `RUSTSEC-2026-0009` probe notice, and the `cargo-deny` component-gate notice per host. No failure or warning annotation exists.


## Component graph, currency and advisories

**Installed/locked components — 15 packages, unchanged from baseline:** 4 workspace crates (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`, all `0.1.0`) and 11 registry crates, every one of them a **dev/build-time** dependency of the test-and-check tooling, never shipped: `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `zmij 1.0.23`. `solo-domain` declares **no** dependencies; `Cargo.toml`, `Cargo.lock`, `deny.toml` and `rust-toolchain.toml` are byte-identical to the baseline.

| Check | Result |
| --- | --- |
| crates.io latest stable, all 11 | **all `LATEST`** at `2026-11-08T16:23:43Z` |
| OSV query of all 11 exact versions | **0 vulnerabilities** at `2026-11-08T16:23:29Z` |
| RustSec advisory database, hosted | `309392b18f6b17022f82c701f300433126901068`, fetched fresh per host `16:19:27Z` / `16:19:44Z` |
| `cargo-deny 0.20.2`, four categories | `advisories`, `bans`, `licenses`, `sources` **ok** on both hosts; archive digests Linux `9f12ed4c…`, Windows `975a2214…` |
| Disposable negative probe | `time 0.3.46` with `RUSTSEC-2026-0009` **refused**, exit 1, while the unchanged graph passed |
| `actions/checkout` | pinned `v7.0.1` by SHA — **still the latest release** |
| Toolchain | `1.99.1` unchanged; `rustc`/`cargo 1.99.1` asserted per host. Rust publishes no separate LTS track, so none is preferred or owed. Re-deriving "latest stable" from the channel file was inconclusive this run (parse, not network); the previous accepted review is the standing verification. |
| `rustup` installer | **`NOT RE-MEASURED`.** The `rustup --version` line runs and succeeds on both hosts, but this agent's log access returns **HTTP 403**, and the rendered job log requires sign-in. The order's own baseline therefore stands: **Ubuntu `1.29.2`, Windows `1.29.3`**, non-blocking at last review — an update proposal for Ubuntu, current on Windows. |

**Warning classification:** zero compiler, Clippy or rustfmt warnings. Zero failure-level annotations. The three notices per host are expected, deliberate evidence from the security probe and the injected-import refusal, not defects. `unicode-ident 1.0.27` carries Unicode-3.0 attribution work that remains an open **future release** prerequisite and is **not waived** by R02.A.

## Constraints, limits and cleanup

**Access limits, never converted into a pass:** Dependabot and code-scanning alert APIs remain **HTTP 401**, job log bodies **HTTP 403**, the scheduled Dependabot run **`NOT OBSERVED`**, and the rendered log requires sign-in. Browser, HTTP delivery, rendered pages, contact form, accessibility conformance, release and deployment are **`NOT APPLICABLE`** to a pure-domain package. Local test execution is **`NOT RUN`** for the missing-linker reason above.

**Toolchain material:** the approved `1.99.1` toolchain, its `rustfmt`/`clippy` components and `rustup 1.29.3` were installed only inside disposable `.temp/w044`; `x86_64-pc-windows-gnu` and `x86_64-pc-windows-gnullvm` standard targets were added there while trying to obtain a local linker, and no such linker exists, so they were never used. All of it is task-owned and removed at run end; no global installation, no PATH change, no cache outside `.temp/w044`. `CARGO_TARGET_DIR` was redirected so no `target/` directory ever appeared in the worktree.

**WIP discipline at this revision:** exactly four files, no subdirectory. `plan-next.md` is the neutral 49-byte heading (`92fa562b…`) with no candidate while its own order is installed; `plan-roadmap.md` still contains the single `R02.A` bullet and Stage 02 is **open**; `workorder-active.md` still carries the issued order byte-identically; `plan-outlook.md` untouched.

**Scratch:** five OS-temp commit-message files deleted; pre-existing `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` preserved untouched. No secrets, credentials or personal data were read or written; no destructive Git operation, force-push, amend, rebase, merge, tag, release or deployment occurred; no other repository was touched.


## Defects found and fixed inside this run, honestly recorded

The first three hosted attempts were **red**, and the package is green only because they were fixed rather than re-described:

| Revision | State | Cause |
| --- | --- | --- |
| `2be7688` | both hosts **failed** step 16 | the test file used `unreachable!`, which is **not** on the 21-entry audited macro allowlist enforced by the `syn` source-boundary checker, so the workspace source gate refused it; a real leading-hyphen locale gap was also present |
| `be847fd` | both hosts **failed** step 16 | macro removed and the leading hyphen refused; one genuine assertion then failed: `SiteWithoutPage` was mapped to `EfsLevel::Page` instead of `Site` |
| `9b1a288` | both hosts **failed**, as intended | a temporary `::error::` diagnostic written straight to stderr, bypassing harness capture, so the runner published the failing case as an annotation — it named `refuses_site_without_page` on both hosts in a single run |
| `1d4d962` | **both hosts green** | level mapping corrected; diagnostic removed in the same commit; the 27 cases are unchanged apart from the assertion that caught the defect |

Two commit **subjects** in this run (`3769018`, `9b1a288`) contain a literal `` `n`n `` sequence because a PowerShell escape did not expand when the message was built. Both are pushed and rewriting published history is forbidden, so the defect is disclosed here rather than hidden; the final source revision's message is clean.

The diagnostic commit is an accepted cost: with log bodies at 403 there was no other legitimate way to learn which case failed, and guessing would have meant an unknown number of blind red cycles. It changed no assertion, no product code and no gate, and it no longer exists at the accepted revision.

**Path ledger, `594f0e1..1d4d962`, five paths only:** `workorder-active.md` (issuance, +92, blob still `7305674f…`), `plan-next.md` (−12, neutralised to `92fa562b…`), `efs_ownership.rs` (+953, new), `lib.rs` (+13/−1), `tests/efs_ownership.rs` (+860, new). Nothing else in the tree moved: `ARCHITECTURE.md`, both SOPs, `plan-roadmap.md`, `plan-outlook.md`, `README.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, every other `.rs` file, `.github/` and the licences are byte-identical to the baseline.

## Status and what the Architect is being asked to review

**`COMPLETE` — evidence phase only.** R02.A is implemented; its 27 focused cases and every pre-existing gate pass on Ubuntu and Windows at `1d4d962`; every accepted bound, refusal category and lineage rule is exercised.

`ACCEPT` is **not** self-awarded. The `R02.A` bullet remains in ROADMAP, NEXT remains neutral and ACTIVE remains issued until the Chief Architect returns a disposition against this revision. On `ACCEPT` the order's §4.5 and §4.6 follow: delete exactly the `R02.A` bullet, install the single `R02.B` candidate, then neutralise ACTIVE and publish the closure report. On `REVISE` the specific defect is corrected while the bullet and the issued ACTIVE are preserved; on `STOP` the blocker is reported truthfully and nothing is closed.

**`agent report ready`**

