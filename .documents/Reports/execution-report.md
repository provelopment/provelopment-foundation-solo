# Execution Report

**Work order:** `SOLO-WO-116 R03` — finish the N116 four-case real-host proof and green technical CI.
**Date:** 2026-11-15. **Outcome sought:** `R03C-SVG-N116-FOUR-CASE-TWO-HOST-TECHNICAL-VERIFIED`.
**status=STOPPED — the four N116 cases, the load-bearing 4/4 gate and one proved S1 defect fix are delivered on the technical branch, but the two-host run is not green. Nothing is integrated into `main`.**

Two technical dispatches were made against the exact branch head. Both reached the general workspace suite and both failed on **real, pre-existing integration gaps that the earlier checkpoints never surfaced** — not on the N116 cases themselves in the vector dimension, and not on any waived gate. The blockers are enumerated in §4 with the smallest decision each one needs.

## 1. Checkpoints

Preflight: `main`/`origin/main` both `0d715f9841fbbd94d0317310e98f109c9ad30dc3`, tree `21b7124d`, parent `605cd1b5`; technical branch at S2 `87d66cd8`, tree `1c2eda48`, parent `dee8bef6` → T0 `0fee134c` → B0 `8c7b6573`; **27** refs; one checkout; one `origin`; empty stash; four WIP files; clean tree apart from the Owner-installed ACTIVE `1e1c5647`. Governance verified 11/11 at the frozen blobs.

| Step | Ref | Commit | Parent |
|---|---|---|---|
| baseline | `main` | `0d715f9841fbbd94d0317310e98f109c9ad30dc3` | `605cd1b5` |
| **A6** | `main` | `6f5a69b9a2d14f99fd3fb8da0f21406087e987db` | `0d715f98` |
| **T2** | branch | `29726fc7c8a4bc53876622a204883d35852a0ac8` | `87d66cd8` |
| **S3** | branch | `c46a8f379e7a9116718ff028f2cfef619a5c6653` | `29726fc7` |
| **D (correction)** | branch | `82d9dbc3e10df3a0a56c881bd6b22f5bc8146080` | `c46a8f37` |
| **R6** | `main` | reported in the handoff (cannot contain its own SHA) | `6f5a69b9` |

A6 and T2 both carry ACTIVE blob `1e1c5647`, restored by byte-safe `git cat-file blob` redirection and verified in the index and on the remote. **The destructive `git checkout main --` was not used.** Refs remain **27**; `ad7b74df` and `40b03c73` unchanged. S3 + D change only `crates/solo-cli/tests/media_worker_host_smoke.rs`, `.github/workflows/rust-verify.yml` and — for the proved defect — `crates/solo-adapters/src/graphic_svg_inspection.rs`.

**Line-count correction (order §1):** `graphic_svg_inspection.rs` is blob `aac743e2…` at S1/S2. `git show <sha>:path | wc -l` reports **1,865**, not the 1,973 asserted in the order; the **blob** is authoritative and unchanged by this run apart from the defect fix below.

## 2. Delivered on the technical branch

**Four real-host cases** in a sibling support module of `n097`, reusing its real `SOLO_TEST_SOURCE_FS_ROOT` Btrfs root, the real confined source reader, the real `BIN` and the accepted three-frame GIF89a fixture:
`n116_01_mixed_five_formats_real_host` (five distinct `(owner, role, local index)` obligations across Entity→Representation, five formats in declared order, distinct real PIDs, GIF timeline, vector result, AVIF refused); `n116_02_safe_vector_semantics_and_cross_host_digest` (all mandated constructs, `M/L/H/V/C/S/Q/T/A/Z`, six shapes, transforms, gradients, clip/mask, blur+composite filter, text→paths, the pinned `icon-home.svg` normalised to LF, idempotence, and two independently pinned canonical digests `0403e067…` and `ce9ea48a…`); `n116_03_active_svg_and_hidden_obligation_refuse_whole` (thirteen hostile documents, later-corrupt whole-set discard, suppressed declaration still inspected, PNG-only favicon for `.svg` and `.gif`, missing source); `n116_04_svg_forgery_and_deliberate_red_oracle` (`WrongSource`→`SourceDigestMismatch`, `WrongOutput`→`OutputDigestMismatch`, genuine path still green, `SOLO_N116_RED_ORACLE=wrong` switching only the test's expected digest).

**Load-bearing gate:** three steps in `rust-verify.yml` (Linux, Windows, and a shared RED step). Each runs `cargo test -p solo-cli --test media_worker_host_smoke --locked n116_ -- --nocapture --test-threads=1`, then a checker that requires the four exact names as `... ok`, exactly four matching lines, `test result: ok. 4 passed; 0 failed; 0 ignored`, no `FAILED|ignored`, and seven evidence tokens the cases actually print. The checker is then run against five mutated copies of its own log — one case removed, one `FAILED`, one `ignored`, one duplicated, one `3 passed; 1 filtered out` — and the step fails if any degraded log is accepted. The RED step requires the switched run to exit non-zero and the unmodified run to pass. All 37 retained steps are preserved; three are added (40 total).

**Proved S1 defect, fixed in-ledger** (`graphic_svg_inspection.rs`, blob `aac743e2…` → new): a document whose only content was `<text>` was refused with `NotCanonical`. Diagnosis by real failing test plus a temporary, uncommitted byte-diff diagnostic showed two causes: the glyph emitter wrote `M0 0h2v2h-2z` while the canonical path writer emits `M 0 0 h 2 v 2 h -2 z` and separates sub-paths with a space; and the reported facts were measured on the source tree while `<text>` expands into `<g>/<title>/<path>` during serialisation, so `elements` disagreed and the parent refused with `FrameMalformed`. The minimal fix emits the canonical spelling and separator and re-measures the bounded canonical output. The diagnostic was reverted before commit; `solo-adapters` SVG unit tests remain **14/14**.

**Local evidence at `82d9dbc3`:** `cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` and `cargo clippy --workspace --all-targets --locked -- -D warnings` zero errors; `n116_02` and `n116_04` **pass** with the pinned digests; `SOLO_N116_RED_ORACLE=wrong` exits **101** and the unmodified run exits **0**; domain+application suites all green; `n116_01`/`n116_03` cannot run locally because this WSL has no Btrfs `SOLO_TEST_SOURCE_FS_ROOT`, and the five `source_read_native::snapshot_proofs` cases are refused for the same reason — stated, not waived.

## 3. Hosted runs (both hosts, same head SHA, attempt 1 each)

**D attempt 1 — run `43621074854` at `c46a8f379e7a9116718ff028f2cfef619a5c6653`:** `failure`. Ubuntu `132526670343`, Windows `132526670471`, both failing the step *Test (workspace, all targets, locked)*. `n116_02` and `n116_04` passed on the host; `n116_01` and `n116_03` failed with `GraphicAssetReadRefusalV1 { kind: MissingSource, … role: Some(GraphicFavicon) }` — the confined reader resolves references relative to `<root>/assets/`, so my declared text `assets/icon.png` doubled the prefix.

**D attempt 2 — run `43621661869` at `82d9dbc3e10df3a0a56c881bd6b22f5bc8146080`:** `failure`. Ubuntu `132528489392`, Windows `132528489627`, again at *Test (workspace, all targets, locked)*. That correction fixed the reference form and adopted the accepted three-frame GIF89a fixture, and the run then exposed three genuine, previously unobserved integration gaps (§4). No N116 case was waived, skipped, renamed or weakened, and no gate was masked.

## 4. Exact remaining blockers

**(1) The architecture dependency allowlist refuses the S1 dependencies — out of ledger.** `workspace_graph::only_the_approved_external_dependencies_are_declared` fails with `solo-adapters may not declare the external dependency quick-xml as normal` (`left: None`), and `real_dependency_graph_is_inward_only` fails with it. The approved `quick-xml =0.42.0` / `svgtypes =0.16.1` declarations that S1 introduced were never added to the allowlist in `crates/solo-cli/tests/workspace_graph.rs`, a file this order does not permit. **Smallest Owner decision needed:** authorize that one allowlist edit (adding exactly those two crates as approved adapter externals), or name the file that should own it.

**(2) The architecture self-test matrix is incomplete for the same reason.** `check_architecture_fixtures::the_real_tree_satisfies_the_source_boundary_contract` and `::the_self_test_matrix_is_complete_and_passing` fail (2 of 35). This is the fixture-side mirror of blocker (1) and should resolve with it; if a numeric count in `crates/solo-cli/tests/architecture_checks/graph.rs` also needs adjusting, that file *is* in the ledger and I will do it under the same authorization.

**(3) A retained N113 assertion is now genuinely stale.** `n097::n113_03_declared_format_and_favicon_refusals` fails with `svg must be refused by the authored declaration validators`. S2 — accepted by the Architect for continuation — admitted lowercase `.svg` to the grammar, so this expectation can no longer hold; the assertion is stale, not the product. R03 permits me to add only the four `n116_*` functions to that file and to correct the eight listed implementation files, so I did not touch it. **Smallest Owner decision needed:** authorize that one stale assertion's correction to "uppercase `.svg` refuses and lowercase `.svg` is admitted as syntax only, then refused by the PNG-only favicon role", which is exactly what the V3 policy already enforces and what `n116_03` proves.

**(4) `n116_03`'s five-owner inventory is rejected by the real validators.** The case panics at `inventory_of(...)` → `a validated test inventory`, i.e. one of my declared `(owner, role)` combinations violates a real registry cardinality (most likely `banner` on a Site or `background` on a Page). This one is **mine and in-ledger**: it needs a corrected role/owner assignment in the test only, no product change, and no new decision.

With (1)–(3) authorized and (4) fixed, the remaining path is one forward branch commit and one fresh same-SHA two-host dispatch. Nothing in that path weakens a security, architecture, RustSec, N108/N113/N102, reproducibility or release gate.

## 5. Status and cleanliness

One checkout, one `origin`, empty stash, four WIP files, no second worktree; no reset, `git clean`, force-push, tag, release, deployment, PR or provider write; no out-of-ledger file touched; the ten B0 fixtures, `Cargo.lock`, `crates/solo-adapters/Cargo.toml`, `deny.toml`, governance, native host, CLI composition root and root manifest are unmodified. Task scratch under `.temp/w3/` was deleted at the end of the run; Owner `.temp/` content preserved.

| Gate | State |
|---|---|
| A6 / T2 authority bytes | **PASS** — `6f5a69b9` and `29726fc7`, both ACTIVE `1e1c5647` |
| Four N116 cases exist with exact names | **PASS** — all four real, real reader, real child |
| N116 4/4 gate + five-log sensitivity + RED | **PASS locally** (02/04 and RED/GREEN); **not yet reached on a host** |
| S1 `<text>` canonical defect | **PROVED and FIXED** in-ledger; 14/14 SVG units green |
| Two-host technical green at one SHA | **NOT ACHIEVED** — runs `43621074854`, `43621661869` both failed on §4 blockers |
| Integration into `main` | **None, by design** |

`status=STOPPED — the four N116 cases, the exact-count gate and one proved S1 canonical-text defect fix are on the branch at 82d9dbc3; the two-host run is blocked by three out-of-ledger or Owner-decision items (§4 items 1–3) plus one in-ledger fixture correction (§4 item 4).` `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; AVIF still refuses; SVG is **not** production-integrated; I do not self-accept.

