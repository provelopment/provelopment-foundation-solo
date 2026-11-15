# Execution Report

**Work order:** `SOLO-WO-116 R04` — correct the demonstrated N116 host failures and prove complete SVG technical CI.
**Date:** 2026-11-15. **Outcome:** `R03C-SVG-N116-FOUR-CASE-TWO-HOST-TECHNICAL-VERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

Both native hosts pass the **complete** `rust-verify.yml` at one identical technical SHA, including the real N116 4/4 gate, its six-log mutation proof, the deliberate RED/GREEN oracle and every retained security, architecture, N102/N108/N113, snapshot and reproducibility gate. Nothing is integrated into `main`; Stage 03 remains open; AVIF still refuses before any worker.

## 1. Commits

| Step | Ref | Commit | Tree | Parent |
|---|---|---|---|---|
| baseline | `main` | `6737e256d5bd68f2d42b2e072c847203331002e3` | `2607d6a2` | `6f5a69b9` |
| **A7** | `main` | `3eca7a4cb4c3366e69bf5cb626bf149d9f411b64` | `4bd4ab4b` | `6737e256` |
| **T3** | branch | `2438d3852239d89718c26d6b24e764b4fd963a2b` | `4cb6056c` | `82d9dbc3` |
| **C1** | branch | `15822c07f2de3a8e39127b388292835bee3544e5` | `6887d1c5` | `2438d385` |
| **C2** | branch | `6146efb2f331559ab3b1574a18e4893abcac8bc9` | `ea0e699f` | `15822c07` |
| **D7** | branch | `3e5a218d366b55d46f7368f9195d49779e29cadf` | `0b2ab222` | `6146efb2` |
| **D7c** | branch head | `d82ae76273b57b37014d3b4c557b8debd5299eea` | `b6daae33` | `3e5a218d` |
| **R7** | `main` | given in the handoff (a report cannot contain its own SHA) | — | `3eca7a4c` |

A7 and T3 carry the identical Owner-installed ACTIVE blob `6f69a87b`, restored by byte-safe `git cat-file blob` redirection and verified in the working tree, index and remote. **`git checkout main -- …` was not used**; the stash is empty; 27 refs remain with `ad7b74df`/`40b03c73` untouched; `main` changed only ACTIVE + this report from the R6 baseline.

## 2. Ledger actually written

| Path | Change | Why |
|---|---|---|
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `APPROVED_ADAPTER_EXTERNAL_NORMAL` 7→**9** (`quick-xml`, `svgtypes`), `APPROVED_ADAPTER_CLOSURE` 39→**47** | §3 C1 |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | stale `n113_03` `.svg` case; invalid `n116_03` favicon inventory | §3 C2, C3 |
| `.github/workflows/rust-verify.yml` | three N116 steps re-authored; two retained architecture edge pins 70→81 | §2.3 proved script defect + §2.1 proved count correction |

`workspace_graph.rs` was **not** edited: all three previously failing assertions resolved from the single shared policy change, exactly as §2.4 predicted. `Cargo.lock` (`d4eeb395`), `crates/solo-adapters/Cargo.toml`, `deny.toml`, `.gitattributes`, governance (11/11 frozen blobs), `graphic_svg_inspection.rs` (`ded4d3d0`), `graphic_image_inspection.rs`, all application/domain/adapter production code, native host, CLI entrypoint, the ten B0 fixtures and their provenance are **unchanged**.

**Independently derived closure.** `cargo metadata --locked --format-version 1 --all-features` over the real resolve graph gives adapter normal roots `gif, image, libc, quick-xml, rustix, serde, serde_json, svgtypes, windows-sys` and a third-party closure of **47** names — the original 39 plus exactly `arrayvec 0.7.8`, `euclid 0.22.14`, `kurbo 0.13.1`, `polycool 0.4.0`, `quick-xml 0.42.0`, `siphasher 1.0.4`, `smallvec 1.16.5`, `svgtypes 0.16.1`. The two workspace members reachable through the graph are excluded by the checker's own pre-existing rule. No other drift was found. Result: `workspace_graph` **32/32**, `check_architecture_fixtures` **35/35**.

**C2 — the stale N113 expectation.** The property-stage list now asserts uppercase `vector.SVG` instead of lowercase `vector.svg`, and three stage-specific facts replace the vanished refusal: lowercase `vector.svg` **is** admitted by a valid non-favicon declaration; the same source is then refused by the V2 four-format inspector as `UnsupportedFormat`; and an `.svg` favicon is refused at the **declaration validator** by the registered PNG-only role (measured `test_inventory_v1(...) == None`, strictly stronger than an inspection-stage refusal). All other N113 invalid-MIME, GIF, AVIF, oversized, missing, corrupted and worker-forgery cases are untouched, and the function name and count are unchanged.

**C3 — the invalid `n116_03` inventory, diagnosed not guessed.** A bounded test-local probe constructed each declaration individually through the real validator and printed `single_ok`/`suppressed_ok` per entry; it was removed before the final commit. Finding: the `late`, `hidden`, `missing`, `hostile` and `mixed` inventories were all valid — only the two favicon entries were impossible, because `(favicon, *.svg)` and `(favicon, *.gif)` are rejected by the registry at declaration. `n116_03` now asserts that declaration-stage refusal and proves the *same* SVG bytes inspect successfully under a non-favicon role, so the refusal is attributed to the role rather than the format gate. Five real independent owners, the hidden/suppressed inspection, the confined missing-file refusal, the hostile matrix, the whole-set discard and the RED oracle are all preserved; no fake source set, no `#[ignore]`, no production schema change.

**Workflow defect, proved before editing.** The R03 gate anchored `... ok$` and used doubly-escaped regexes. `cargo test --nocapture` interleaves a case's captured stdout onto its own `test <name> ...` line, so that anchor can never hold, and the over-escaped `... FAILED|ignored` test matched nothing. The three N116 steps were re-authored with **fixed-string** checks only — exact names, exact count 4, the exact `test result: ok. 4 passed; 0 failed; 0 ignored` line, absence of `... FAILED`/`... ignored`/`test result: FAILED`, and the seven evidence tokens — plus six mutated-log probes (missing, FAILED, ignored, duplicate, three-of-four, wrong digest). A local extraction of the committed checker accepted the authentic 4/4 log and rejected all six mutations.

## 3. D7 — the two-host technical proof

**Run `43674236111`, attempt 1, `head_sha = d82ae76273b57b37014d3b4c557b8debd5299eea`.**
`verify (ubuntu-24.04)` **success** (job `132690745816`, 29 steps ok); `verify (windows-2025)` **success** (26 steps ok). Platform-specific steps are `skipped` on the other host and are not claimed as executed.

N116 on Ubuntu: `n116_01_mixed_five_formats_real_host`, `n116_02_safe_vector_semantics_and_cross_host_digest`, `n116_03_active_svg_and_hidden_obligation_refuse_whole` and `n116_04_svg_forgery_and_deliberate_red_oracle` all reported `ok`, with `test result: ok. 4 passed; 0 failed; 0 ignored`, then `n116_gate authentic=accepted` and `n116_gate mutations=rejected-all`. The Windows job ran the same gate step and passed it.

Retained gates green on both jobs: cargo-deny 0.20.2 (advisories/bans/licences/sources), the RustSec negative probe, workspace package-set gate, disposable Btrfs snapshot filesystems, format, strict lint, the full locked workspace test suite, confined image-inspection evidence, N108 8/8, V2 N113 4/4, N102 4/4 plus the 12/12 native repeat, the real-tree architecture checker (now `edges=81`) and the byte-identical reproducible release build.

Attempts on the way to green, all forward commits on the same branch, none masked: R03 `43621074854` and `43621661869` (previously reported); R04 `43672557526` at `15822c07` (only `n113_03`, one stale favicon inventory); `43673000853` at `6146efb2` (only the N116 gate's own `ok$` anchor); `43673671059` at `3e5a218d` (only the retained `edges=70` pin — N116 4/4, mutations, RED, N108, V2, N102 and cargo-deny were all green there); one redundant dispatch `43674206493` fired before the edge-pin edit was committed and repeated the identical `edges=70` failure; final `43674236111` green.

## 4. Local verification on this host

`cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` and `cargo clippy --workspace --all-targets --locked -- -D warnings` zero errors; `workspace_graph` 32/32; `check_architecture_fixtures` 35/35; `n116_02` and `n116_04` pass with the pinned digests `0403e067…` / `ce9ea48a…`; `SOLO_N116_RED_ORACLE=wrong` exits **101** and the unset run exits **0**; SVG unit suites 14/14; domain/application/adapter property suites green.

**Local limitations, stated not waived:** this WSL host has no Btrfs `SOLO_TEST_SOURCE_FS_ROOT`, so `n113_`, `n116_01`, `n116_03` and the five `source_read_native::snapshot_proofs` cases cannot execute here. They are **not** skipped in hosted Linux — the Ubuntu job ran them for real and passed.

## 5. Status

`status=COMPLETE — pending independent Chief Master Architect review.` The accepted PNG/JPEG/WebP/GIF implementation, complete GIF animation, S1 sanitizer (blob `ded4d3d0`), S2 V3 five-format product code, the ten B0 fixtures and all retained gates are intact. AVIF is **unsupported and refused before any worker**; SVG is **not** integrated into `main`, and no release, tag, PR, merge, deployment or Stage 03 closure occurred. `main` differs from its R6 baseline only by ACTIVE and this report. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; I do not self-accept.

**Edge-count correction, disclosed.** The two retained architecture-checker steps pinned `edges=70`. A real run of `cargo test --locked -p solo-cli --test check-architecture` prints `architecture=legal packages=4 targets=33 files=64 edges=81`, and re-running with the pre-edit `graph.rs` constants stashed **still** printed 81, proving the drift comes from the already-approved S1 `quick-xml`/`svgtypes` adapter dependencies and not from the C1 policy edit. Both literals now pin the single exact measured value **81**; the assertion remains an exact count and nothing was loosened, skipped or made configurable. This is the one edit outside the three N116 steps and is flagged for independent review.

