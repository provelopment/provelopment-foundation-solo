# Execution Report

**status=COMPLETE — pending independent Architect review** for the bounded outcome `R03C-GIF-N108-FIRST-TWO-REAL-HOST-CANARIES-VERIFIED`. Two actual host tests now drive the real compiled `solo-cli` under the production confined supervisor and pass on Ubuntu 24.04 **and** Windows 2025 at one technical SHA, with identical fixture and transcript digests. The pre-existing intermittent Windows `n102_01` canary failed on the first attempt at that same SHA and passed on the immediate retry; both runs are disclosed below and neither is relabelled. No GIF code is integrated into `main`, which remains PNG/JPEG/WebP only. No release, deployment, publication or self-acceptance; six further N108 cases, atomic V2, deliberate-red proof and the true-vector SVG remain mandatory future work.

## 1. Preflight (verified, unchanged)

`HEAD == main == origin/main == git ls-remote == ea63f2808b5c94e4c0eb3194cb37a1a03b95b898`, tree `3569c2c3`, sole parent `c9776506`; report `b54a29f6`; prior ACTIVE `43dc6a56`; NEXT `92fa562b`, ROADMAP `9d7a8622`, OUTLOOK `cb9c7a61`; four WIP files; 22 remote refs with `solo-wo-109-gif-proof-r01 = 1be3750b` (tree `7ae47460`), `solo-wo-108-gif-worker-r01 = bb3ef4ab`, `solo-wo-107-gif-r01 = efc384e1`; one origin, one worktree, empty stash. Only tracked dirt was the Owner-installed WO-110 ACTIVE `69de6f22`, staged byte-for-byte. Frozen governance and native host `d411eaf4` untouched; no waiver, skip or dependency addition.

## 2. Commits and blobs

| Step | Commit | Tree | Parent | Key blobs |
|---|---|---|---|---|
| A authority | `b5d0f63ad5c6489234431532ab9e429641147815` | `03a38f1e` | `ea63f280` | ACTIVE `69de6f22` |
| B0 import | `8128437b49e6e1715a904b5185270da5b59d735a` | `8df8a05e` | `b5d0f63a` | eight exact checkpoints |
| C1 n108_01 | `ce9c739cba5c56f60e5201715960cec91069754d` | `6cecdae7` | `8128437b` | lib `1d3e354e`, smoke `598813fd` |
| C2 n108_02 | `0752b3e4b0dbee1b5351f900747caf026e59577e` | `52f9540b` | `ce9c739c` | smoke `a1ec89c6` |
| D CI gate | `275cf6f1845b90e1944ce69dadd441b436ad8f6e` | `ae631d73` | `0752b3e4` | workflow `9810641a` |

B0 imported from `1be3750b` with `git hash-object` equality on all eight: workflow `866c9e51`, `Cargo.lock` `cf105167`, adapter manifest `3610f06f`, GIF codec `785d08d0`, adapter worker `a3096585`, adapter lib `3f7ab0b3`, host `3f6eebd3`, graph `dc6d2770` — zero mismatches. **Final technical head `275cf6f1845b90e1944ce69dadd441b436ad8f6e`** on `solo-wo-110-gif-host2-r01`.

Only two source edits followed the import: `lib.rs` re-exports the already-existing trusted seam `inspect_gif_bytes_with_executable_v1` and typed `GifFrameEvidenceV2` (raw `inspect_gif_animation_v1` stays crate-private), and `media_worker_host_smoke.rs` gains `#[cfg(any(target_os = "linux", windows))] mod n108`. No change was needed to the worker, the parser, the supervisor or the GIF codec: the preserved WO-109 system worked on the first real invocation.

## 3. The two real host canaries

Fixtures are inline deterministic bytes; no external asset, dependency or file. Both call `inspect_gif_bytes_with_executable_v1(Path::new(CARGO_BIN_EXE_solo-cli), &bytes)`, the same parent validation as production, with the real `INTERNAL_IMAGE_WORKER_FLAG_V1` child in `gif` mode.

**`n108::n108_01_gif87a_still_confined_and_parent_verified`** — extension-free GIF87a, 1×1, red. Asserts signature `GIF87a`, 1×1, one frame, not animated, `loop == Absent`, source digest equal to the parent-computed SHA-256, first composite digest equal to SHA-256 of `[255,0,0,255]`, nonzero transcript, nonzero child PID, and both `limits_verified` and `cleanup_verified`.

**`n108::n108_02_gif89a_animation_keeps_frames_delays_and_loop`** — three-frame 1×1 GIF89a with NETSCAPE2.0 infinite loop and three Graphic Control Extensions (10 cs keep, 20 cs background+transparent, 5 cs keep+transparent). Asserts signature `GIF89a`, `frame_count == 3`, animated, `loop == Infinite`, `one_cycle_delay_ms == 350`, ordered indexes `0,1,2`, delays `[10,20,5]`, disposals `[Keep, Background, Keep]`, composite digests `[red, red, transparent]`, source digest, nonzero transcript, nonzero PID, limits and cleanup. No first-frame substitution and no forced decoder agreement: the asserted compositing follows the codec documented transparent-background rule.

### Executed results, both hosts

| | Ubuntu 24.04 (job `132419616625`) | Windows 2025 (job `132420508297`, attempt 2) |
|---|---|---|
| `n108_01` | pass, child pid `5366` | pass, child pid `9284` |
| `n108_02` | pass, child pid `5371` | pass, child pid `8248` |
| result line | `test result: ok. 2 passed; 0 failed; 0 ignored;` | `test result: ok. 2 passed; 0 failed; 0 ignored;` |
| fixture source SHA-256 (01 / 02) | `a06797608309dae0cf51654e275db7ad6a179845c29542443aeee6b4b12536b1` / `dc425aeadeee8daa6324c0c6093710c69f6ad6bcf6d58c12571e88df1632b84f` | identical |
| transcript SHA-256 (01 / 02) | `7e0b25b3c3a9ec30431db004e59c093e2ac2d0799773f131d64a665a780d834b` / `8c02ba6fe2d4fea8edabba2378db8a696bd166b5fba72315a93924341019ac33` | identical |

Cross-host source **and** transcript digests are equal; only PIDs and timings differ. Each case printed one bounded `n108_gif=` line with frame count, duration, PID and the two digests; no source bytes, paths, environment or handles were logged. Confinement evidence is the parent's own: nonzero child PID, `limits_verified`, `cleanup_verified`, exit code 0 and a reaped child, all required before an observation exists at all — the child's own claim is not evidence. The smaller GIF stdout cap is applied by the reader while reading on both hosts.

## 4. Mandatory two-host CI gate and CI results

Step D adds two dedicated steps immediately after the existing N097 evidence step and before the untouched N102 step: `N108 real GIF worker canaries (Linux)` (`shell: bash`, `set -euo pipefail`) and `(Windows)` (`shell: pwsh`). Each runs `cargo test -p solo-cli --test media_worker_host_smoke --locked n108_ -- --nocapture --test-threads=1`, fails on nonzero exit, asserts each of the two exact qualified names, asserts exactly two `^test n108::n108_0[12]_…` lines, asserts `test result: ok. 2 passed; 0 failed; 0 ignored;`, and rejects any `FAILED` line. N097, the four N102 canaries, architecture, cargo-deny, release smoke and reproducibility are unchanged.

| Run | SHA | Ubuntu | Windows |
|---|---|---|---|
| Issuance `43585543116` | `b5d0f63a` | success (job `132418600000`-series) | success |
| Branch attempt 1 `43585708101` | `275cf6f1` | **success** `132419616625` (N108 step 19 success) | **failure** `132419616744` at step 21 |
| Branch attempt 2 (same SHA) | `275cf6f1` | **success** `132420509552` | **success** `132420508297` (N108 step 20, N102 step 21 success) |

**Disclosed irregularity, not waived.** Attempt 1's Windows failure was the *unchanged, pre-existing* `n102::n102_01_extra_inheritable_handle_is_excluded`, panicking at `media_worker_host_smoke.rs:2460` with `test result: FAILED. 3 passed; 1 failed`, exactly the historical WO-108 issuance failure `43579699908`. Every N108 step in every run passed; the N102 case is unrelated to this order and outside its permitted edits, so it was neither disabled, renamed, retried inside the test nor called green. An immediate rerun of the failed jobs at the identical SHA produced two green jobs (Windows `132420508297`, Ubuntu `132420509552`). Both attempts are reported as they happened; the canary remains active and mandatory. This is the second observed occurrence of the same intermittent Windows resolution failure and is recorded for the Architect as an unresolved reliability question, not a passing gate.

## 5. Local verification before dispatch

`cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` 0 errors; `cargo clippy --workspace --all-targets --locked -- -D warnings` clean; `n108_` 2 passed / 0 failed / 0 ignored; `graphic_gif` 14 passed; `unsafe_boundary` 22 passed; architecture `legal packages=4 targets=33 files=63 edges=70`; workflow YAML parsed (34 steps, both N108 steps present). Local WSL reproductions printed the same two digest pairs as CI.

## 6. Not claimed, preserved, remaining

No GIF integration, acceptance, product capability, release, deployment, tag or self-acceptance. `main` remains PNG/JPEG/WebP. The binding true-vector sanitized SVG requirement (Architecture §6.2I) is unchanged and unimplemented. `solo-wo-109-gif-proof-r01`, `solo-wo-108-gif-worker-r01` and `solo-wo-107-gif-r01` are untouched; the only new ref is `solo-wo-110-gif-host2-r01` (23 refs: 22 preserved + 1). ACTIVE remains the exact Owner WO-110 bytes. Still mandatory: `n108_03`–`n108_08`, atomic mixed-owner V2, deliberate-red digest sensitivity, exact-blob integration and the SVG package. Task scratch lived only in owned ignored `.temp/wo110/` and was removed. Local WSL limitation unchanged (reflink-dependent `snapshot_proofs` cases fail locally, pass hosted).