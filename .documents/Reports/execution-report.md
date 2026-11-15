# Execution Report

**Work order:** `SOLO-WO-118 R10` — activate one correctly associated alpha + essential-`irot` AVIF class through the real confined worker and complete N122 on both hosts.
**Required outcome:** `R03C-AVIF-ASSOCIATED-ALPHA-IROT-CONFINED-FOUR-PLANE-N122-TWO-HOST-VERIFIED`.

**status=COMPLETE** for that bounded outcome only. One 8-bit YUVA 4:4:4 still with one straight full-resolution alpha auxiliary and one essential `irot` quarter turn on both items now passes the real confined child and the trusted-parent four-plane checks; its composite is the immutable independent R03 display digest. Both defective association fixtures are refused by metadata policy although the codec renders them. **No V4, `R03.C`, Stage 03, catalogue, publication, release or deployment claim is made.**

## 1. Transaction

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` = `4c14e0debab58ab5f169a0da4668907691347d64` (R17), branch = `d31fc0d9` (T17) | as §2 requires; 28 remote heads, one worktree, four WIP files, empty stash |
| **A18** (`main`) | `06fd4df5e13dd80f3a55bac7ba824d9d4bb21812` — tree `b0f3fd7c9a7a7eb0b69c071a2258019b56a7b061`, parent `4c14e0de` | ACTIVE blob `488f5c04ad47d87dd9b86dab217f262462e8749b` identical in worktree, index, A18 and `origin/main` |
| **T18.0** (branch) | `fcb262939947b07fd5ec6586e2d1a32f4f79101c`, parent `d31fc0d9` | single-path `git restore --source=A18`; index blob identical to A18's; staged set = ACTIVE only |
| **T18** (branch) | `f4dcf9c24e00ca86151003c6dc6dcce9d3bb3035` | one technical commit, `6 files changed, 1994 insertions(+), 50 deletions(-)` |
| **R18** (`main`) | this report's commit (SHA in chat) | parent `06fd4df5`, only this file changed |

Technical branch `f4dcf9c2` = push run **`43726215854` attempt 1**: `verify (ubuntu-24.04)` `132848601350` ✅, `verify (windows-2025)` `132848601340` ✅, `avif-reference-oracle` `132848601061` ✅. No failed attempt, no rerun, no `workflow_dispatch`.

Changed paths (exactly the ledger): `.documents/WIP/workorder-active.md` (T18.0 only), `crates/solo-adapters/src/graphic_avif_inspection.rs`, `crates/solo-application/src/media_worker.rs`, `crates/solo-application/tests/media_worker.rs`, `crates/solo-adapters/tests/graphic_avif_inspection.rs`, `crates/solo-cli/tests/media_worker_host_smoke.rs`, `.github/workflows/rust-verify.yml`. Root and crate `Cargo.toml`, `Cargo.lock`, `deny.toml`, all `oxideav-*` versions/checksums/features, host supervisor, protocol, source reader, CLI main, R03 fixtures/JSON/Python/provenance, R08 referee and every WIP/SOP/Architecture document are byte-identical to T17; the accepted dependency is still `oxideav-avif = "=0.0.18"` with checksum `fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee` and **zero lockfile drift**.

## 2. The product change

**Child policy (`graphic_avif_inspection.rs`).** `decode_checked_still_v1` still inspects typed metadata and the confinement profile first, then routes on the parsed container alone: an attached alpha auxiliary selects one narrow branch, everything else keeps the untouched coded-plane path. `associated_alpha_policy_v1` requires

- a single still `av01` primary, exactly one `auxl` reference from the auxiliary to it and no other `auxl` source, `find_alpha_item_id` identifying that auxiliary;
- on **both** items exactly one `Irot{angle:1}` **marked essential**, and no `Imir`, no `Clap`, and no property outside `{av1C, ispe, pixi, colr}` (plus `auxC` alpha on the auxiliary only) — `prem`, `iscl`, unknown or essential-unknown extensions all refuse;
- matching `ispe` 512×256 on both items, `pixi [8,8,8]` primary / `[8]` auxiliary, one `av1C` each;
- 8-bit, YUV 4:4:4, `inspect` extents equal to the coded extent, displayed extent = the swapped coded extent, both ≤ 8192 and ≤ 32 M pixels.

`decode_checked_alpha_still_v1` then requires the decoded picture to be **8-bit `Yuva444P` at the swapped extents with four row-packed planes** (stride = displayed width, rows = displayed height, length = stride × rows, ≤ 256 MiB) before building the claim. The envelope writer now renders the class tokens (`yuva`/`has_alpha=true`/`identity`) from the validated claim; the coded class still writes `yuv`/`false`/`unverified` unchanged.

**Parent (`media_worker.rs`).** `check_observation_v1` keeps every host-fact, dimension, depth, plane-count, per-claim arithmetic and total-bound check **before** any layout arithmetic (so `WORKER_OUTPUT_OVERSIZE` keeps its documented priority). `canonical_yuva_layout_matches_v1` then admits exactly four full-resolution planes for `color_model=Yuva` at **8 bits** with **`orientation=identity`**; 10/12-bit, `transposed`/`unverified`, sub-sampled, padded or three-plane alpha claims are `WORKER_PLANE_INVALID`. The parser, the 16-field V1 envelope, the host supervisor and `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1=false` / `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1=false` are unchanged. The parent verifies transport, bytes and canonical shape; the child alone decides source policy.

**Local proof of the production policy** (`#[cfg(test)] mod alpha_class_tests` inside the same allowed file): the valid fixture is admitted with four 131 072-byte planes and a header that still fits the 1024-byte frame **and parses with the unchanged strict parser**; `abc_color_irot_alpha_NOirot.avif` and `clap_irot_imir_non_essential.avif` are refused as invalid media **although `oxideav_avif::decode` returns a picture for both**; `white_1x1.avif` keeps the `yuv`/`false`/`unverified` class.

## 3. N122 evidence — three real cases, two native hosts

Both native logs carry `test result: ok. 3 passed; 0 failed; 0 ignored` for `n122_`, each case name exactly once, each `n122_evidence case=0N` exactly once:

```text
Ubuntu  case=01 pids=[7253] distinct_pids=1 planes=4 geometry=256x512 depth=8 orientation=identity
        planes_match_pinned=true display_sha_pinned=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744
        alpha_nonopaque=123952 coded_sha_pinned=5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82
        records=1 publication=false proves_media=false limits=verified reaped=true
Ubuntu  case=02 refused=2 labels=["alpha-noirot.avif", "clap-irot-imir.avif"] counterfeits=15
        counterfeits_refused=true protocol=[(missing-key, MissingField), (duplicate-key, DuplicateField),
        (unknown-key, UnknownField)] positive_ok=true whole_set_discarded=true unconfined_decoded=false planes=4
Ubuntu  case=03 pids=[7274, 7278, 7282, 7286, 7290] distinct_pids=5 records=5 planes=16
        levels=entity,facet,site,page,representation alpha_planes=4 alpha_orientation=identity
        alpha_display_sha=950be879…c84cc744 non_alpha_depths=8,10,12 whole_set_discarded=true
        partial_records=0 private_path_absent=true publication=false proves_media=false
Windows case=01 pids=[6932] … case=03 pids=[7472, 2032, 9764, 7408, 2508] distinct_pids=5 — identical tokens
```

- **case 01** acquires the immutable source through the real reader, launches **one** real confined child under the 512 MiB / 10 CPU-second / 15-second profile, confirms normal exit, verified limits, reap and no leak, and receives the four 256×512 planes whose digests are exactly the pinned displayed planes; the parent's own rehash and aggregate framing match. `orientation=identity` is only a claim, `proves_media_correctness=false`, `grants_publication_authority=false`.
- **case 02** sends both defective originals through the real child and gets two policy refusals, re-runs the valid positive successfully, and refuses **15** forged V1 claims by typed category (four planes with `has_alpha=false`; three planes with `has_alpha=true`; false alpha digest; false alpha bytes; sub-sampled fourth plane; 10-bit YUVA; `unverified` and `transposed` orientation; bad source digest; bad aggregate; short fourth plane; trailing byte; false exit; false limits; false cleanup) plus the three unchanged parser key mutations and a direct unconfined invocation that exits nonzero with empty stdout.
- **case 03** mixes the four unchanged coded-plane stills (Entity, Facet, Site, Page) with the suppressed `internal` representation-level alpha still: five distinct child PIDs, five checked records, `3×yuv/unverified` plus one `4×yuva/identity`, every per-plane digest against the independent golden. Replacing that last source with the defective association refuses the **whole set** with zero partial records and a refusal that contains no asset path, filename or staging tag.

**The independent pixel oracle** lives where the pinned decoder does: `solo-adapters`' `n122_oracle_valid_associated_alpha_irot_matches_the_independent_r03_display_golden` re-derives the typed facts (both items `av01`, one essential `irot{1}` each, `ispe` 512×256, `pixi [8,8,8]`/`[8]`, one `auxl` from item 2 to item 1) and `n122_oracle_display_and_coded_composites_are_distinct_and_pinned` measures `try_to_rgba8()` = **524 288 bytes**, alpha **min 128 / max 255**, **123 952** non-opaque samples, display digest **`950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`**, and — after undoing the quarter turn — the distinct coded digest **`5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82`**; both run on both native hosts in the same job. **Honest deviation:** §4 case 1 asked the `media_worker_host_smoke` target to call the codec directly, but `crates/solo-cli/Cargo.toml` is immutable in this order and `solo-cli` has no codec dependency. The composite is therefore measured in `solo-adapters` over the same immutable bytes while case 01 proves byte-identical transport of exactly those four pinned planes; the case line prints `display_sha_pinned=` rather than claiming a measurement it did not take.


## 4. Retained evidence and the authorised N121 migration

- **N120** four goldens re-run byte-exact through the upgraded class routing: `white_1x1.avif` (1×1, 8-bit, 444) and the three fox stills (1204×800 at 8-bit 420jpeg, 10-bit 420p10, 12-bit 420p12) still report `aggregate_matches=true` with unchanged per-plane digests, and the "unsupported files are never accepted as the four positives" test is unchanged and green.
- **N123** is untouched and still green on both hosts: `n123_decoder repair_verified=true`, `alpha_differences=0 rgb_differences=0`, display digest `950be879…c84cc744`, nonopaque 123952, plus its authentic/mutated log gates.
- **R03 oracle** unchanged and green: `pristine_manifest_matches=true`, `mutation_rejections=3/3`, R08 referee `same_coded_rgba=true same_display_rgba=true alpha_differences=0 rgb_differences=0`, alias refusal and cleanup.
- **N121 migration (authorised, not a waiver):** the valid `abc_color_irot_alpha_irot.avif` entry was removed from the hostile vector only; the other ten vectors — empty, oversize, favicon-not-PNG, bad magic, truncated, `alpha-noirot.avif`, `clap-irot-imir.avif`, whole-set discard, Platform namespace, unconfined invocation — remain, N121 is still **two real tests, 2 passed / 0 failed / 0 ignored**, and `n121_evidence case=02` now honestly prints `refused=10` with the full label list and no `alpha-irot.avif` claim. Its four 3-plane goldens, four child PIDs and twelve planes are unchanged, the `counterfeits=12` count is unchanged, and its three mutation probes still reject. Both host verifiers now require the exact ten-label list and reject `refused=11` and any `alpha-irot.avif` token.
- **N093, N102, N108, N113, N116**, the 23-fixture architecture refusal self-test, the real-tree architecture gate (`architecture=legal packages=4 targets=34 files=66 edges=93`, unchanged), the forbidden-import/unsafe gates, the byte-reproducible release build and the release smoke all pass on both hosts at `f4dcf9c2`.
- **Gate logic, proven before the push:** the exact Linux `n121`/`n122` verifier functions were extracted from the committed workflow and executed against synthetic logs in the printed shape — `authentic=accepted` and `mutation=rejected` in both, and the hosted logs then reproduced those verdicts natively (`n122_gate authentic=accepted`, `n122_gate mutation=rejected`, `n121_gate authentic=accepted`, `n121_gate mutation=rejected`, `n123_gate authentic=accepted`, `n123_gate mutation=rejected`).

## 5. Local verification and honest limits

Recorded in the single Owner checkout before the push: `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` 0; `workspace_graph` `32 passed`; `check_architecture_fixtures` `35 passed`; `check-architecture` 0 with `architecture=legal packages=4 targets=34 files=66 edges=93`; `graphic_avif_inspection` `5 passed` (two N120, N123, two N122 oracles); `media_worker` `14 passed` (11 retained + 3 new); host-smoke `--no-run` builds; the three new in-crate policy tests pass; `cargo test --workspace --all-targets --locked` → **87 passed, 5 failed**, the five failures being the `source_read_native::snapshot_proofs` panics at `crates/solo-adapters/src/source_read_native.rs:2017` because this WSL host has no verified disposable Btrfs `SOLO_TEST_SOURCE_FS_ROOT`. `n121_` and `n122_` also refuse to run locally for the same documented reason (`SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem`), and Windows linkage is absent (`link.exe`): all of these are **NOT RUN LOCALLY** and were proven natively in the hosted jobs above. No `#[ignore]`, `--skip`, `continue-on-error`, masked exit or filesystem substitute was introduced.

## 6. Repository state and completion boundary

`main` = A18 `06fd4df5` + this report; technical branch = `f4dcf9c2` (clean); **28** remote heads, one worktree, one `origin`, exactly four `.documents/WIP/` files, empty stash, no new branch/tag/PR/release/deploy, no force push, no `reset --hard`, no `git clean`. Task scratch (`.temp/a18.sh`, `.temp/gate_check.py`) removed; three temporary probe test targets used to measure typed metadata and pins were deleted before the technical commit, so the graph is unchanged at `targets=34 files=66 edges=93`.

`status=COMPLETE` **only** for `R03C-AVIF-ASSOCIATED-ALPHA-IROT-CONFINED-FOUR-PLANE-N122-TWO-HOST-VERIFIED`: one real confined alpha positive accepted with the independently pinned display composite, both defective transform fixtures refused by typed metadata policy through the real child, trusted-parent four-plane structural/host checks and every forged negative refused, all three N122 cases passing natively on Ubuntu and Windows, the four N120 goldens, N121 2/2 with the honest ten-label refusal list and N123's repaired zero-delta differential all green, and all three branch jobs plus the fresh `main` baseline green. **Nothing here claims V4, `R03.C`, Stage 03 closure, the six-format candidate, any Platform catalogue, publication, release or deployment**; premultiplied, 4:2:0/4:2:2-alpha, 10/12-bit-alpha, resized-alpha, mirror, clean-aperture, grid and sequence classes remain explicitly unsupported.

