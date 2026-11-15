# Execution Report

**Work order:** `SOLO-WO-118 R04` — confined 8/10/12-bit AVIF still decoding with independent parent verification.
**Date:** 2026-11-15. **Required outcome:** `R03C-AVIF-CONFINED-8-10-12-STILL-PLANE-ORACLE-TWO-HOST-VERIFIED`.

**status=STOPPED.** The ordered R04 slice is **not** delivered. One ledger-scoped, locally and remotely verified increment exists and is green, but the confined AVIF worker, the trusted-parent verification of a V1 envelope, the four named N120 real-host tests and the N120 CI gate were **not** implemented in this run. No V4, `R03.C`, Stage 03 or publication claim is made, and no gate was weakened to manufacture a green.

## 1. Commits (all verified by full SHA)

| Step | Commit | Tree | Parent |
|---|---|---|---|
| baseline (R11) | `e66f721118c64d62e29b000f1be67a68791a27a2` | — | A11 |
| **A12** (`main`, ACTIVE only) | `c7862da36146107d56dedc728603bc43e1ecbe6b` | `f1c655ae16b7e024af96e88d16c65d569fb27656` | `e66f7211` |
| **T12.0** (branch, ACTIVE import) | `4544b9e47e6ac6755b516d22daaed3a660f5b3b8` | — | `80823351` |
| **T12.1** (branch, real golden assertions) | **`cd3866d41db7feb044e0a619847dc9927d420245`** | `81aefb0f` | `4544b9e4` |
| **R12** | given in the chat handoff only | — | `c7862da3` |

A12 committed only `.documents/WIP/workorder-active.md`; the working-tree, index, commit and `origin/main` blobs all read `1e38739f4de3278afe79c9bd5920f9476f9b6c6e`. T12.0 restored that exact blob by single-path `git restore --source=<A12>` and committed exactly one path. The R03 work order was **not** re-imported over the new one.

## 2. Hosted result at `cd3866d4`

Branch push-triggered run **`43701486229`**, `head_sha = cd3866d41db7feb044e0a619847dc9927d420245`, attempt 1, overall **success**: `verify (ubuntu-24.04)` success, `verify (windows-2025)` success, `avif-reference-oracle` success. `edges=93` and every pre-existing gate were preserved; the oracle job re-verified the pinned goldens unchanged.

**This is not R04 proof.** Those jobs are green because the tree still contains only the R03 oracle plus the increment in §3; they do **not** exercise a confined AVIF worker, because none was added.

## 3. What T12.1 actually delivers (and proves)

`crates/solo-adapters/tests/graphic_avif_inspection.rs` — the ledger path that replaces the R01 feasibility probe with real assertions:

- The four unrotated, non-alpha positives (`white_1x1`, fox 8/10/12-bit) are decoded from immutable fixture bytes by the accepted pure-Rust decoder.
- Every canonical Y/U/V plane is hashed and compared with the **independent** `avifdec`/`dav1d` values pinned by R03, per plane and as the documented aggregate. All four match exactly: `n120_oracle case=goldens fixture=… width=… height=… depth=… chroma=… planes=3 aggregate_matches=true`.
- Layout is **derived** from the coded chroma token (`444` → 1×1 sub-sampling; `420jpeg`/`420p10`/`420p12` → 2×2) and checked twice: against the manifest and against the decoder's own `stride` and plane length. Depth 8 uses one byte per sample, 10/12 two, with the independently pinned `1204/602` and `2408/1204` strides.
- The three alpha/transform fixtures decode to **4 planes** and are asserted to sit outside the accepted four; R04's plain-still path must refuse them, and they are never promoted to positives.

**Proved:** for these four stills the accepted decoder reproduces independently measured pixels byte-for-byte, so an oracle-faithful still path is available to the next order. **Not proved:** anything about the confined worker, parent verification, orientation, alpha handling, grids, sequences or publication.

**Gate integrity note.** The first version used `include_str!` for the manifest; the architecture checker refused it (`include-unsupported: source-loading macro include_str is refused`) and failed the gate. The test now uses a fixed repo-relative `std::fs::read_to_string` on a fixture path, never a caller-supplied one. The gate was **not** relaxed.

## 4. Deliberately **not** delivered — the remaining R04 scope

1. `crates/solo-adapters/src/graphic_avif_inspection.rs`: the safe child codec (confinement first, bounded stdin at `20 MiB + 1`, `inspect` preflight, `decode`, canonical 16-field envelope plus the exact Y/U/V payload written once).
2. The single closed `avif` token and child dispatch in `graphic_image_inspection.rs`, plus the narrow `lib.rs` module and exports.
3. `ConfinedAvifInspectAdapterV1` and the strengthened `check_observation_v1` dimension/stride binding in `solo-application/src/media_worker.rs`, with the swapped-chroma, forged-depth, short-payload, hostile-JSON, false-exit and false-cleanup negatives.
4. The four named N120 tests (`n120_01`…`n120_04`) with real children, Btrfs `Root` fixtures and the RED/GREEN oracle proof, and their gate on both hosts.

Because none of these exist, the R04 acceptance conditions are unmet; this report does not describe the run as a completed technical milestone.

## 5. Local evidence and state

Local gates before the T12.1 push (`cd3866d4`): `fmt` 0, `check --workspace --all-targets --locked` 0, `clippy -D warnings` 0, `check-architecture` 0 reporting `edges=93`, `check_architecture_fixtures` 0, `workspace_graph` 32/32, `graphic_avif_inspection` **2 passed**. A full `cargo test --workspace --locked` run locally reported 84 passed and the five `source_read_native::snapshot_proofs` failing with `SOLO_TEST_SOURCE_FS_ROOT must name an R03 snapshot fixture filesystem`: that is the known **NOT RUN LOCALLY** Btrfs set, unchanged and not caused by this increment.

`main` is `c7862da3` plus this report; the technical branch is `cd3866d4`; remote heads remain **28**; one worktree; empty stash; four WIP files; both trees clean. Permanent writes stayed inside the ledger (ACTIVE plus the single test path). No `Cargo.lock`, `Cargo.toml`, decoder source, `media_worker*.rs`, protocol, N093/N102/N116 source, oracle Python/JSON/provenance, fixture binary or architecture document changed. Task scratch removed.

**Status:** `status=STOPPED` — ordered R04 slice incomplete; A12/T12.0/T12.1 are preserved, verified and pushed, and the next Owner-issued order must resume at `cd3866d4` with items 1–4 of §4.
