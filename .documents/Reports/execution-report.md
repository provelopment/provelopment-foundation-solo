# Execution Report

**Work order:** `SOLO-WO-118 R23` — implement the pinned Unicode 18 / ICU4X locale and route-key validation boundary (`R04.A1` / `R04.B1`).
**Bounded outcome:** `R04AB1-UNICODE18-ICU4X-LOCALE-ROUTE-LITERAL-VALIDATION-N134-TWO-HOST-NONPUBLISHING-V1`.

```
status=STOPPED
blocked_reason=architecture_checker_dependency_allowlist_outside_authorized_ledger
```

**Why this run stopped, in one sentence:** the order authorizes five new ICU4X normal dependencies on `solo-adapters` (§2.1) and simultaneously keeps the mandatory repository architecture checker active (§4, §5.1) while listing `crates/solo-cli/**` as *strictly outside scope* (§3) — and that checker's approved-edge and approved-closure allow-lists live only in `crates/solo-cli/tests/architecture_checks/graph.rs`, so the authorized dependency change cannot pass the mandatory gate without an edit the order forbids. The order's own instruction for exactly this case is to STOP and report the precise blocker instead of trespassing.

## 1. Transactions actually performed

| Step | SHA | State |
|---|---|---|
| accepted baseline | `main` R30 `9f43ffe28250858e3347e9e2083d40f762a8dd31`; technical T30 `b3cbdeb589423b222bd8db9e683005f9b824b2a2` | verified; one `origin`, one worktree, four WIP files, empty stash, clean trees, **30** remote heads observed |
| **A31** (main, ACTIVE only) | `2587e599917ef1bc85df1c74f4332dc5e839c3f9`, tree `7047c49ac8f0cb75f7706e42b238167a718b8310`, parent `9f43ffe2` | ACTIVE blob `7c6d505b0cb570250455723fe7428d47bf79a0ff`, sha256 `2bb9b1bfb77d0844f63b02a8804d11ec24543d69b5f9f2fc46edb2f5e4d8b25d`, 30 585 bytes / 251 lines, byte-identical worktree → index → commit → `origin/main`; exactly one path |
| **T31.0** | `eca36d0a7da73d8aacce4b5b18682231daaf0971`, tree `b73fd5a765c298f02a91b09d1f73063ed47e7c60`, parent `b3cbdeb5` | the **identical** ACTIVE blob; `git diff` = ACTIVE only; pushed |
| **T31** | **not committed, not pushed** | the local work reached a green state except the out-of-scope gate; the unverified tree was discarded so `origin` holds only verified work |
| **R31** | this commit (SHA in the Owner handoff) | parent `2587e599`, report only |

The technical branch therefore remains at **T31.0 `eca36d0a`**, identical to its remote, with a clean worktree. No RED workflow was pushed, no history was rewritten, and no branch, tag, worktree, PR or merge was created.

## 2. What was implemented and locally proven before the stop

The work existed only in the local tree and is fully described here so it can be reproduced under an authorizing order.

| Path | Local change |
|---|---|
| `crates/solo-adapters/Cargo.toml` | +12: the five exact `=2.4.0` ICU4X normal dependencies, `default-features = false`, `compiled_data` (no feature added to `icu_locale_core`) |
| `Cargo.lock` | +312/−0, one `cargo`-generated resolution; no manual edit |
| `crates/solo-adapters/src/manifest_unicode_route_literals.rs` | **new, 834 lines**: closed `validate_manifest_unicode_route_literals_v1`, typed `ValidatedRouteLiteralV1` records, closed `RouteLiteralRefusalV1`, pure no-I/O |
| `crates/solo-adapters/src/lib.rs` | +7: one `mod` line and the public re-exports |
| `crates/solo-adapters/tests/publication_manifest.rs` | +672/−0: additive `mod n134_manifest_unicode_route_literals` with exactly five cases; the 24 pre-existing tests byte-identical |

**Resolved Unicode graph, verified locally:** one coherent ICU4X **2.4.0** family — `icu_casemap`, `icu_casemap_data`, `icu_collections`, `icu_locale`, `icu_locale_core`, `icu_locale_data`, `icu_locale_fallback`, `icu_locale_fallback_data`, `icu_normalizer`, `icu_normalizer_data`, `icu_properties`, `icu_properties_data`, `icu_provider` (13 packages, all exactly `2.4.0`), plus `displaydoc`, `litemap`, `potential_utf`, `stable_deref_trait`, `synstructure`, `tinystr`, `utf8_iter`, `utf_types`, `writeable`, `yoke`, `yoke-derive`, `zerofrom`, `zerofrom-derive`, `zerotrie`, `zerovec`, `zerovec-derive`. Five direct edges only; no native or platform ICU, no `icu` umbrella crate, no wildcard, no second Unicode version. Data pins: **Unicode 18 / CLDR 49** via `icu_*_data 2.4.0`.

**Local gate results (WSL Ubuntu, pinned `rustc 1.99.1`):** `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets --locked -- -D warnings` **0 errors**; `cargo build --workspace --release --locked` **success**; `cargo test -p solo-adapters --test publication_manifest --locked` **29 passed / 0 failed**; the N134 selection **5 passed / 0 failed** with five real `n134_evidence` lines; `cargo-deny 0.20.2` (the same pinned release archive the workflow uses) reported `advisories ok, bans ok, licenses ok, sources ok` — **no new license exception was needed**, because ICU4X is `Unicode-3.0`, which `deny.toml` already allows and which this branch now actually encounters (so the harmless `Unicode-3.0` license-not-encountered warning the order reports for the main-only graph does not appear here; `deny.toml` was not touched).

**Real ICU4X 2.4.0 behaviour the new tests pinned, measured rather than assumed:** NFC maps `e`+U+0301 to `é`; full fold maps `Straße` → `strasse` while root lowercase maps it to `straße`, so `Straße`/`STRASSE` share a fold key but never a spelling; Greek `ΣΟΦΟΣ` folds to `σοφοσ` while its lowercase spelling keeps the final `ς`; `İstanbul` folds to `i`+U+0307+`stanbul`, i.e. the default non-Turkic mapping; `de-de`, `EN-001` and the deprecated alias `iw-IL` (→ `he-IL`) are closed noncanonical refusals; `en` has no region; `de-DE-1901` and `en-US-u-ca-buddhist` fall outside the language/script/region profile. The N134 cases also proved the whole-input refusals for 20 unsafe path shapes (leading/trailing/doubled separator, wrong-case locale segment, nine segments, 61-scalar segment, 180-scalar Page token, leading/trailing `-`/`_`, dot segments, percent and encoded-delimiter material, backslash, DEL, a bidi format control, NUL and an emoji) with safe structural locations and no partial result, and the positive acceptance of internal `-`/`_`, Devanagari, Japanese and digits under the pinned property data.

## 3. The precise blocker

`cargo test -p solo-cli --test check-architecture` — the mandatory real-tree architecture checker that every native job runs — reports **34 findings**, every one of them caused by the authorized ICU4X edges:

- **5 × `graph-unapproved-dependency`**: `solo-adapters` → `icu_casemap`, `icu_locale`, `icu_locale_core`, `icu_normalizer`, `icu_properties` are each a "forbidden normal edge".
- **29 × `graph-external-reach`**: the ICU4X closure is outside the enumerated audited closure — `displaydoc`, `icu_casemap`, `icu_casemap_data`, `icu_collections`, `icu_locale`, `icu_locale_core`, `icu_locale_data`, `icu_locale_fallback`, `icu_locale_fallback_data`, `icu_normalizer`, `icu_normalizer_data`, `icu_properties`, `icu_properties_data`, `icu_provider`, `litemap`, `potential_utf`, `stable_deref_trait`, `synstructure`, `tinystr`, `utf8_iter`, `utf_types`, `writeable`, `yoke`, `yoke-derive`, `zerofrom`, `zerofrom-derive`, `zerotrie`, `zerovec`, `zerovec-derive`.

Both findings are resolvable **only** by editing `crates/solo-cli/tests/architecture_checks/graph.rs`:

```
pub const APPROVED_ADAPTER_EXTERNAL_NORMAL: [&str; 10] = [ … ];  // → [&str; 15], + the five icu_* roots
pub const APPROVED_ADAPTER_CLOSURE:        [&str; 53] = [ … ];   // → [&str; 82], + the 29 nodes above
```

That file sits inside `crates/solo-cli/**`, which §3 lists as **strictly outside scope**, and no authorized file carries this authority: the two constant arrays are the only approval mechanism, and no configuration file or fixture the ledger permits can substitute for them. Weakening, skipping or silencing the checker is forbidden by this order and by `AGENTS.md` §6, so the run stopped instead.

**One finding was mine and is already fixed.** My first draft used a local binding named `region`, which the checker's dynamic-loading rule refuses (`dynamic-loading: … (region)`); renaming it to `area_code` cleared that finding before the stop. It is recorded because it was a real defect in my code, distinct from the Owner-level blocker.

**Nothing else was defective.** The only other local failures are five `source_read_native::snapshot_proofs::*` tests that panic with `SOLO_TEST_UNSUPPORTED_FS_ROOT must name an R03 snapshot fixture filesystem` — an environment precondition absent from this WSL checkout, in a module this order does not touch.

## 4. Smallest required Owner decision

Authorize **one** additive change, in one file, so the mandatory checker can approve the dependency set this order already mandates:

> Extend `crates/solo-cli/tests/architecture_checks/graph.rs` only — `APPROVED_ADAPTER_EXTERNAL_NORMAL` from 10 to 15 entries (adding `icu_casemap`, `icu_locale`, `icu_locale_core`, `icu_normalizer`, `icu_properties`) and `APPROVED_ADAPTER_CLOSURE` from 53 to 82 entries (adding the 29 nodes in §3) — with no other edit to `crates/solo-cli/**`, no relaxation of any check, and the same audit evidence for each new node.

With that authorization the remainder is a small delta: the module, the `lib.rs` lines, the five N134 cases, the two native gate steps, the `ARCHITECTURE.md` note and this report all already exist in a locally green state and are described precisely enough above to be reinstated.

## 5. Truthful completion facts

```text
manifest_route_visibility_policy_preflight_implemented=true
icu4x_2_4_0_resolved_and_locked_locally=true
icu4x_2_4_0_published=false
unicode18_route_literal_keys_verified_locally=true
unicode18_route_literal_keys_verified_on_two_hosts=false
n134_five_cases_passed_locally=5/5
n134_two_host_gates_verified=false
architecture_checker_green=false
architecture_checker_findings=34
unicode18_complete_official_corpus_conformance=false
uts39_complete_confusable_gate=false
idna_base_validator_implemented=false
complete_route_collision_ledger_implemented=false
request_locale_negotiation_implemented=false
whole_manifest_semantic_validation=false
http_delivery_implemented=false
publication_authority=false
public_route_authority=false
derived_output_publication=false
release_authority=false
deployment_authority=false
r04a_open=true
r04b_open=true
r04c_open=true
r04d_open=true
r03c_open=true
stage03_open=true
```

No claim is made that Unicode 18 official-corpus conformance, UTS #39 confusables, IDNA, the whole-route collision ledger, request-language negotiation or any delivery surface exists. N134 was designed as bounded evidence, and even that bounded evidence is **not** published or two-host verified by this run.

## 6. Preservation and cleanliness

**Fresh post-R31 `main` two-host baseline:** run `43896133841` at `e4f27b124e3ac784ab477eb0aeae7826919ddf17` — `verify (ubuntu-24.04)` job `133400068516` **success**, `verify (windows-2025)` job `133400067888` **success**. That run confirms the governance-only `main` tree (A31 + this report) still passes every existing gate and every native job, including the architecture checker, because `main` carries no dependency or source delta. The baseline for this follow-up report commit itself is supplied in the Owner handoff, exactly as `R30` did, since a report cannot know its own not-yet-created SHA.

`main` remains governance/report-only: A31 (ACTIVE) and this report, nothing else, with **zero** `crates/`, `Cargo.lock`, platform or asset delta. The technical branch holds T31.0 (ACTIVE) and nothing more, unmerged. The 81 Owner-approved Tabler SVGs, the immutable `1.0.0` catalogue (`ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`), the MIT notice and provenance, the donor fixtures, `deny.toml`, `.gitattributes`, `publication_manifest.rs`, `manifest_visibility_bridge.rs`, `manifest_route_visibility_policy.rs`, `crates/solo-domain/**`, `crates/solo-application/**` and `crates/solo-cli/**` are all byte-unchanged; the 20 canonical Foundation sources remain unapproved and undelivered; no automatic product role default is registered. One checkout, one `origin`, four WIP files, empty stash, clean trees, **30** remote heads observed — no new branch, tag, PR or worktree. Task-owned scratch (`.temp` transaction scripts and case files) was deleted; the Owner's pre-existing `.temp` entries were left untouched.

**Status `STOPPED` means only that this bounded `R04.A1`/`R04.B1` outcome is not delivered.** Nothing was served, routed, published, released, deployed or merged.



