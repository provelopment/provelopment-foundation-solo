# Execution Report

**status=COMPLETE — pending independent Chief Master Architect review** for the bounded outcome `R03C-GIF-FOUR-REAL-HOST-CANARIES-AND-N102-EVENT-IDENTITY-VERIFIED`.

Four real confined GIF-worker cases now execute and pass on Ubuntu 24.04 **and** Windows 2025 at one technical SHA with identical fixture and transcript digests, the two-host N108 gate requires exactly those four qualified cases, and the Windows N102 handle-isolation canary now proves **kernel object identity** instead of a numeric coincidence, passing **12/12 consecutive native Windows runs**. No GIF or N102 correction is integrated into `main`. No release, deployment, tag, publication or self-acceptance.

## 1. Preflight (verified)

`HEAD == main == origin/main == git ls-remote == 6e834854ae8ea898706a1902b9139e5c8c279e1e`, tree `9b4f66b5`, sole parent `503dc5ec`; report `a03f1dcf`; prior tracked ACTIVE `69de6f22`; NEXT `92fa562b`, ROADMAP `9d7a8622`, OUTLOOK `cb9c7a61`; exactly four WIP files; **23** remote refs, with `solo-wo-110-gif-host2-r01 = 275cf6f1` (tree `ae631d73`, parent `0752b3e4`), `solo-wo-109-gif-proof-r01 = 1be3750b`, `solo-wo-108-gif-worker-r01 = bb3ef4ab`, `solo-wo-107-gif-r01 = efc384e1`; one origin, one worktree, empty stash. The only tracked dirt was the Owner-installed WO-111 ACTIVE `f44911cd`, staged byte-for-byte with no normalisation. Frozen governance (`f66096ec`, `1483c9be`, `51bb3792`, `6eae1da8`, `1c99d348`, `57d75796`) and root/technical baseline paths untouched; no waiver, skip, dependency change or new feature.

## 2. Commits, trees and blobs

| Step | Commit | Tree | Parent | Key blobs |
|---|---|---|---|---|
| A authority | `e7f8ed6c663368370e9a79f26a1b253faa976d76` | `0491495d` | `6e834854` | ACTIVE `f44911cd` |
| B0 import (9 paths) | `20775dd295c1278bfb2d9bbfed06e182f88eeb14` | `a26caf55` | `e7f8ed6c` | all nine exact |
| **C1** n108_03 | `e2f2d744147bac6c6766ede3d51ef19a196f7e71` | `9ea7fe06` | `20775dd2` | smoke `d603a208` |
| **C2** n108_04 | `cee2cf41b8efc02361becc9529f1406767023ee4` | `e67f515d` | `e2f2d744` | smoke `2c059848` |
| **C3** N102 identity | `4bef4cccbdd67044d75d1a029a092eb0eb62f365` | `5dbfc4c5` | `cee2cf41` | image `89327241`, host `b967d041`, native `9a26e195`, lib `08b83237`, smoke `fbb6b549` |
| **D** gates | `371c15de81bad746f7e684e59cb2145750b4789c` | `29e92e40` | `4bef4ccc` | workflow `cd6bd2da` |

**Final technical head: `371c15de81bad746f7e684e59cb2145750b4789c`** on `solo-wo-111-gif4-n102-r01`.

B0 restored exactly from `275cf6f1`, verifying both `git rev-parse <sha>:<path>` and local `git hash-object` — **zero mismatches**: workflow `9810641a`, `Cargo.lock` `cf105167`, adapter manifest `3610f06f`, codec `785d08d0`, image inspection `a3096585`, adapter lib `1d3e354e`, host `3f6eebd3`, graph `dc6d2770`, smoke `a1ec89c6`. The native file was deliberately **not** imported and stayed at the pinned main blob `d411eaf4` until C3's narrow correction. No entire-branch merge.

## 3. C1 — `n108_03_gif89a_previous_background_palette_interlace_offset`

One test, four real supervised children, deterministic inline fixtures built by a test-only port of the accepted codec's own `BitPackerV1` / `lzw_encode_v1` / `table_bits_v1` / `table_bytes_v1` / `write_sub_blocks_v1` / `GifSpecV1` builder (pinned eight-bit LZW minimum code size so dictionary growth cannot mask framing; no encoder dependency, no raw decoder exposure, no hardcoded first-frame stream).

| Case | Assertion | Observed |
|---|---|---|
| RestorePrevious | 2×2 red → 2×2 blue disposed `Previous` → transparent reveal; digests red, blue, red; frame 1 disposal token `Previous` | pass, real child |
| RestoreBackground, **nonzero BLUE global background index** | 2×2 red → 1×1 red at (1,1) disposed `Background` → reveal; last composite exactly three red + one **opaque blue**, explicitly not all-red and not transparent | pass, real child |
| Local palette | GREEN/WHITE local table overrides RED/BLUE global; composite equals 2×2 opaque green; `local_palette=true` | pass, real child |
| Interlaced, nonzero offset | 2×4 screen, 1×4 frame at `left=1`, interlace bit, stored `[0,2,1,3]`; right column RED/GREEN/BLUE/WHITE, left column zero alpha; `interlaced=true`, offsets `(1,0,1,4)` preserved | pass, real child |

Every case also asserts signature, geometry, frame count, source digest, nonzero transcript, nonzero child PID, `limits_verified` and `cleanup_verified`.

## 4. C2 — `n108_04_gif_extension_signature_trailer_and_mislabeled_input_refuse`

Positive controls (the known-good `gif87a_red_pixel()` and `gif89a_three_frames()`) are still accepted by the real child. Nine malformed inputs are refused with a **typed** `GifWorkerRefusalV1` and never a partial observation:

| Input | Refusal token |
|---|---|
| `BAD87a` signature | `gif-worker-source-rejected` (parent-held signature validation) |
| GIF89a-only GCE spliced into GIF87a (offset 19 = first image descriptor, asserted from the fixture) | `gif-worker-supervisor-refused` |
| truncated (terminator + trailer removed) | `gif-worker-supervisor-refused` |
| extra byte after trailer | `gif-worker-supervisor-refused` |
| unsupported plain-text extension `0x21 0x01` | `gif-worker-supervisor-refused` |
| logical-screen background index 5 with a two-entry table | `gif-worker-supervisor-refused` |
| PNG / JPEG / WebP mislabelled as `gif` | `gif-worker-source-rejected` |

A child-declared refusal reaches the trust boundary through the closed supervisor refusal (`ImageWorkerRunRefusalV1`), which the seam reports as `gif-worker-supervisor-refused`; parent-side rejection reports `gif-worker-source-rejected`. Both are closed typed refusals — no `Ok`, no fake worker, no local decoder call, no large allocation. The unchanged N097 PNG/JPEG/WebP suites still pass.

## 5. C3 — N102 numeric aliasing replaced by object identity

**Diagnosis (not a claim of an actual leak):** the previous canary asked the child whether the transmitted *integer* resolved in its own handle table. A child can hold an unrelated handle of the same value, which is exactly the intermittency seen in WO-108 `43579699908` and WO-110 `43585708101` attempt 1.

**Correction:** the parent creates a real anonymous manual-reset `CreateEventW` object with `bInheritHandle = TRUE` (still excluded from the two-entry `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`), plus a non-inheritable control event; both stay nonsignaled, owned and alive until after the child is fully reaped. A new closed reserved mode `probe-event:<candidate>:<control>` (canonical ASCII `u32` only, ≤64-byte parameter, overflow/punctuation/extra arguments refused) makes the confined child **first prove the API on a private event it owns** (`SetEvent` then `WaitForSingleObject(..,0) == WAIT_OBJECT_0`) and then attempt `SetEvent` exactly once on each transmitted candidate. The child emits only `SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=<u32>`; the attempt outcome is deliberately not decisive and never printed. The parent then asks the kernel about its **own original** objects: `WaitForSingleObject(original, 0)` must return `WAIT_TIMEOUT`. A reused integer cannot signal them; a genuinely inherited event handle would signal the same kernel object and fail the test. `WAIT_FAILED`/`None` is never read as "not signaled". A separate fresh parent event proves `SetEvent` → `Some(true)` really observes a state change.

`N102ExtraInheritableV1` gained only `signaled_v1()` and the test-only `signal_parent_v1()`; `n102_probe_event_v1` is a bounded free function in the existing approved native module (three `unsafe` blocks, each with an adjacent `// SAFETY:` rationale). Production spawn, the two-handle allowlist, job limits, environment, lifecycle and cleanup are unchanged; `n102_probe_handle_v1` and the other three N102 cases are untouched.

**Native Windows evidence (job `132430316774`):** `n102_01 extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true child_pid=7272`, four-case canary step `test result: ok. 4 passed; 0 failed; 0 ignored`, and the dedicated repeat step logged `N102 repeat 1/12 … 12/12 passed` and `12/12 consecutive native Windows runs passed` — each repetition independently asserting the evidence line and `ok. 1 passed; 0 failed; 0 ignored`. No retry, filter, `continue-on-error` or weakened assertion exists.

Safe parser tests added in the existing adapter test module (2 tests) refuse wrong protocol prefixes, missing/reordered/repeated fields, an extra field, a second line, non-canonical booleans, signed/zero/over-wide pid and non-ASCII input; all pass.

## 6. Host observations, both hosts

`n108_01` source `a06797608309dae0cf51654e275db7ad6a179845c29542443aeee6b4b12536b1` / transcript `7e0b25b3c3a9ec30431db004e59c093e2ac2d0799773f131d64a665a780d834b`; `n108_02` source `dc425aeadeee8daa6324c0c6093710c69f6ad6bcf6d58c12571e88df1632b84f` / transcript `8c02ba6fe2d4fea8edabba2378db8a696bd166b5fba72315a93924341019ac33` — both unchanged from the WO-110 pins.

`n108_03` and `n108_04` were produced identically on both hosts:

| Fixture | Source SHA-256 | Transcript SHA-256 |
|---|---|---|
| restore-previous | `91635990bdb2edaabdbf0eb97399def3009a0a415236422cbc5440711d3d3300` | `8e0ef6881dc3429d5f4c4e082053b769463f430aee478bd95105266bc913af6d` |
| background (blue index) | `7bc2081e115c27cc6bf554956036472eb939b7fea82245ba59632679fe1565f5` | `c1c6a33d21411ee7d51fe87f20372a8fa8810fe69bd55b1ddc59df16c5f452fe` |
| local-palette | `25d3d821aedaf21ee7bce1d99d66dff71b19473bfab8e98537e3e6cc402fda51` | `9aa9d186e2cdcaaf2590e334739e09a44a5eaebba3e3764cf9cb656fbe1b6a90` |
| interlace-offset | `e1f5dc87408e10d19176d8c2f268a3d82ac590f5e34c0dbaac31bc4065ecfb0c` | `9acbdc806453cbc4715464ee17cd19c1c9481a1d2835244c2ea565e6219fa0ae` |

Real child PIDs: Ubuntu `5417, 5421, 5425, 5429` (n108_03) and `5434, 5438` (n108_04 controls); Windows `9944, 8728, 1992, 4740` and `1048, 3172`. Only PIDs differ across hosts; every digest is byte-identical. Logged metadata is bounded: frame counts, durations, PIDs, digests and refusal tokens — no source bytes, paths, handles, secrets or environment.

## 7. CI evidence

| Run | SHA | Kind | Ubuntu | Windows |
|---|---|---|---|---|
| `43588402467` | `e7f8ed6c` (A) | issuance | success `132427725137` | success `132427724873` |
| `43589260788` attempt 1 | `371c15de` (D) | technical branch | **success** `132430317050` | **success** `132430316774` |

Technical branch steps on Windows: 18 N097 success, 20 N108 (Windows) success, 21 N102 four-canary success, 22 N102 12-run repeatability success. On Ubuntu: 18 N097 success, 19 N108 (Linux) success. Nothing failed and no retry was needed at this head. Local emulation of the exact Bash gate against real test output yielded `count=4`, the result line and no failures — gate logic verified before dispatch.

Everything else in both jobs was green: format, strict Clippy, workspace check, cargo-deny 0.20.2 with RustSec refresh plus the advisory negative probe, package-set gates, real-tree architecture checker and its refusal fixtures, test-failure tripwires, release build, release CLI smoke, forbidden-import refusal, and the byte-for-byte reproducible double release build.

## 8. Local verification and honest limits

WSL: `cargo fmt --all -- --check` clean; strict Clippy (workspace, all targets, `-D warnings`) clean; `cargo check --workspace --all-targets --locked` 0 errors; `n108_` 4 passed; `graphic_gif` 14 passed; `n102_event_answer` 2 passed; `unsafe_boundary` 22 passed; architecture `legal packages=4 targets=33 files=63 edges=70`.

Two local limits, neither substituted for hosted proof: (1) this Windows session has no MSVC linker, so a native local `cargo test` cannot link; the Windows code was instead type-checked with `cargo check --workspace --all-targets --locked --target x86_64-pc-windows-msvc` (clean), and all native Windows behaviour above comes from hosted `windows-2025`. (2) The known WSL reflink limitation still fails five `source_read_native::snapshot_proofs` cases locally; they pass hosted and were not skipped.

Cosmetic observation: the committed repeatability step's script echo begins with a stray UTF-8 BOM (visible as an odd glyph in the raw log). PowerShell tolerated it, the step ran and passed, and no gate depends on it.

## 9. Not claimed, preserved, remaining

No GIF integration, product acceptance, whole-owner V2, deliberate-red sensitivity proof, release, deployment, tag or self-acceptance; `main` remains PNG/JPEG/WebP only and the R03.C3 / R03.C / Stage 03 gates stay OPEN. The binding true-vector sanitized SVG requirement is unchanged and unimplemented.

Preserved exactly: all 22 prior non-main refs at their frozen SHAs (`solo-wo-107-gif-r01 = efc384e1`, `solo-wo-108-gif-worker-r01 = bb3ef4ab`, `solo-wo-109-gif-proof-r01 = 1be3750b`, `solo-wo-110-gif-host2-r01 = 275cf6f1`, and 18 older), ignored Owner data and WSL cache; the only new ref is `solo-wo-111-gif4-n102-r01` (24 refs = 23 preflight + 1). ACTIVE remains the exact Owner WO-111 bytes. Task scratch lived only in owned ignored `.temp/wo111/` and was removed.

Still mandatory in later separately authorised orders: `n108_05`–`n108_08`, atomic mixed-format PNG/JPEG/WebP/GIF V2 whole-owner inspection, deliberate-red digest-sensitivity proof, exact-blob GIF integration, and the Owner-approved safe true-vector SVG package.



