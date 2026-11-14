# Execution Report

**Work order:** `SOLO-WO-112 R01` — complete GIF `n108_05`–`n108_08` real-host acceptance canaries.
**Date:** 2026-11-14. **Bounded outcome:** `R03C-GIF-EIGHT-REAL-HOST-CANARIES-VERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

Eight real supervised GIF-worker canaries now run on Ubuntu 24.04 and Windows 2025 through the existing two-host gate, which was extended from exactly four to exactly eight named executions without weakening any other gate. Both hosts passed at the first reported attempt of run `43595784309`. No GIF or N102 source is integrated into `main`; no release, tag, deployment, publication or self-acceptance.

## 1. Preflight (verified before acting)

`HEAD == main == origin/main == git ls-remote origin main == 9bf16d56ed33cfdcad4933578010684722e6c925`, tree `25defce379c985736b6bb2dd34b10022a9bc3927`, sole parent `49c849b5`, canonical report blob `c3492075`. The only dirty tracked path was the Owner-installed `SOLO-WO-112 R01` ACTIVE file, preserved byte-for-byte as blob `b677df647db1d8f468f946b93c5e76c7468b5c0e` (worktree raw hash identical, so no line-ending change). Accepted technical head `solo-wo-111-gif4-n102-r01 == origin == cdf0ceebc4a140db3d5a7dcd5b2fa8b76ea119b1`, tree `712d259a`, parent `371c15de` — imported from, never merged. One `origin`, one worktree, empty stash, 24 remote refs before branch creation, exactly four WIP files, no unowned dirt or untracked risk, no extra checkout.

Frozen blobs re-verified identical: Architecture `f66096ec`, Architect SOP `1483c9be`, Implementation SOP `51bb3792`, `AGENTS.md` `6eae1da8`, architecture source checker `crates/solo-cli/tests/architecture_checks/source.rs` `1c99d348`, unsafe checker `57d75796`, root `Cargo.toml` `14505be4`, `rust-toolchain.toml` `9ec55b3b`; `plan-roadmap.md` `9d7a8622`, `plan-next.md` `92fa562b`, `plan-outlook.md` `cb9c7a61`.

## 2. Branch and checkpoint map

Branch `solo-wo-112-gif8-host-r01` (one new branch; 25 remote refs afterwards, every earlier ref frozen).

| Step | Commit | Tree | Parent |
|---|---|---|---|
| A authority (ACTIVE only) | `dc266f551adb44de04120e8569b2ed425efc75ef` | `87037ce9e98ef41cdc92d3034fbeb70ec1a385d0` | `9bf16d56` |
| B0 ten-blob import | `0057f67e381e4e2ee5cf77d105fd706477b7c320` | `0e6e2092cb4db4010f7cd3cf1e7fda4fa21d5f48` | `dc266f55` |
| C1 `n108_05` | `8819985cae96fcbce9163f2ca35ad2e6b88e3995` | `8521bbbdf5d9870f032723b2f7429316cb421d8d` | `0057f67e` |
| C2 `n108_06` | `8283c4cd40c70badd6c82da6fec05a211c2f1e03` | `bb63d5e31e9d3281a26b0fc0aef25d5c46ec3b60` | `8819985c` |
| C3 `n108_07` | `4c1251c631ff9ed1e641dc80c3ae283bd244c779` | `80354523afbef1784c59d4ecf2f0ba4dafe7a01c` | `8283c4cd` |
| C4 `n108_08` | `dc4e518015939e2d6037a5f1e1b50a8dec02332c` | `9999c53e1842b1deb7f34c9337d7aa4ba11c88f6` | `4c1251c6` |
| **D gate** | `40b03c73b0bc8544359438daea6218caa496e85d` | `00c3cf3282359c927878b2fca97a7fb22b3895ec` | `dc4e5180` |

Smoke-test blobs: main baseline `88b4c6ab`, B0 `fbb6b549`, C1 `723def9a`, C2 `2e2aa94d`, C3 `6627f025`, C4 and D `b3d65014`. Workflow blob at D `b120e0227a1a7e69914b2a68245c802073b9a8a4`. **Corrected technical head: `40b03c73b0bc8544359438daea6218caa496e85d`.**

## 3. B0 — exact import, zero mismatches

Ten named paths restored from `cdf0ceeb` by `git restore --source`, never merged or cherry-picked. For each path the source `git rev-parse cdf0ceeb:<path>`, the filtered worktree `git hash-object --path=<path>` and the committed `git rev-parse HEAD:<path>` were all compared with the pinned value: **10/10 source checks, 10/10 worktree checks, 10/10 committed blob checks, zero mismatches.**

| Path | Blob |
|---|---|
| `.github/workflows/rust-verify.yml` | `cd6bd2da76b6381f2226a065a92a6db72b9b80af` |
| `Cargo.lock` | `cf1051675576ab0e00ce7acbd5dbe139882c31b4` |
| `crates/solo-adapters/Cargo.toml` | `3610f06ff58cdea6ce1b3993544f0c2e286087ea` |
| `crates/solo-adapters/src/graphic_gif_inspection.rs` | `785d08d0e5a9009be1cbb015eb1b88362f6ab311` |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | `8932724113c1a27941e080fea7037fb24a4e2e74` |
| `crates/solo-adapters/src/lib.rs` | `08b832372196a3f6a97316da116b7af1d71dd35b` |
| `crates/solo-adapters/src/media_worker_host.rs` | `894bc6220d90161ce98bb9c0c848ff208388f666` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `9a26e195b295974b54f7e2da7ce7ae8fbc510c4a` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `dc6d2770038a14199d2f366faab64db4e3d6e98b` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `fbb6b549198a95c7fac718646a804623428a32d6` |

No production source, adapter, workflow gate, dependency or governance byte was edited after the import. The corrected Windows N102 event-object-identity validation, PID equality and single-record framing are preserved unchanged.

## 4. C1 — `n108_05_gif89a_absent_finite_infinite_loops_and_zero_delay`

Five deterministic 1×1 GIF89a specimens, each sent through the real `inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), …)` seam (never the raw codec, never a fake worker):

| Case | Authored | Accepted evidence |
|---|---|---|
| absent extension | `loop_count: None`, delay 0 cs | `GifLoopV1::Absent`, cycle 0 ms, 1 frame, `!is_animated`, delay 0 cs, RED digest |
| finite one | `Some(1)`, delay 1 cs | `Finite(1)` (not `Absent`), cycle 10 ms, authored 1 cs retained |
| finite two | `Some(2)`, delay 5 cs | `Finite(2)`, cycle 50 ms |
| infinite | `Some(0)`, delay 0 cs | `Infinite`, cycle 0 ms, still one frame, never counted as multiple |
| mixed delays | absent, delays `[0, 7, 3]` cs | exactly 3 frames, cycle 100 ms, exact `[0,7,3]`, no browser-minimum normalisation |

Every accepted result also proved `source_digest() == sha256_media_worker_bytes_v1(bytes)`, a positive bounded transcript, a nonzero real PID, and `limits_verified` + `cleanup_verified`. Local run: five distinct child PIDs `51760, 51764, 51768, 51772, 51776`.

## 5. C2 — `n108_06_gif_resource_budget_and_frame_rectangle_refusals`

Compact in-memory specimens, all through the same real supervised seam; the 1×1 control stayed accepted (`pid=52183`).

| Case | Specimen | Refusal token |
|---|---|---|
| 1025 frames | `0..=1024` short RED frames | `gif-worker-supervisor-refused` |
| canvas side 8193 | 8193×1 screen, one 1×1 frame | `gif-worker-supervisor-refused` |
| 134,217,728 composed pixels | 8192×8192 screen, two 1×1 frames (`2 × 67,108,864`) | `gif-worker-supervisor-refused` |
| 3,932,100 ms duration | six frames at 65,535 cs (`6 × 655,350`) | `gif-worker-supervisor-refused` |
| rectangle outside canvas | 2×2 screen, authored 2×2 frame at `left=1` | `gif-worker-supervisor-refused` |
| source over ceiling | exactly `MAX_GIF_SOURCE_BYTES_V1 + 1` bytes | `gif-worker-source-rejected` (parent-side, no child spawn) |

Every specimen returned `Err` — never success, never an empty or partial observation — and the aggregate case stayed compact (no 8192×8192 bitmap is materialised; the container scan refuses before any canvas allocation). No resource rule was bypassed or relaxed.

## 6. C3 — `n108_07_gif_application_repeat_and_lzw_corruption_refusals`

Real-worker positive controls first: `gif87a_red_pixel()` and `gif89a_three_frames()` still accepted (PIDs `52461`, `52465`; source digests `a0679760…`, `dc425aea…`, matching the pinned `n108_01`/`n108_02` values). Controlled malformed variants, each with the exact mutated offsets asserted before mutation:

| Case | Exact verified fact | Result |
|---|---|---|
| duplicate NETSCAPE repeat | 19-byte block at offset 19, `NETSCAPE2.0`, `[3,1,0,0,0]`; second copy spliced at 38 | refused |
| unknown application id | `[22..33]` was `NETSCAPE2.0`, replaced with `NOTSCAPE2.0` | refused |
| malformed Graphic Control Extension | `[38..41] == [0x21, 0xf9, 4]`, block size mutated to 3 | refused |
| LZW without end code | length 35, min code size 2, payload `[31..33] == [0x44, 0x01]`, terminator at 33, trailer at 34; payload replaced with a clear-plus-index stream lacking the end code the strict decoder demands | refused |
| missing sub-block terminator | verified terminator byte at 33 became a one-byte block | refused |
| truncated compressed sub-block | verified block size at 30 raised from 2 to 4 | refused |

All six returned the closed `gif-worker-supervisor-refused` token with no incomplete observation, and all nine `n108_04` refusals are retained unchanged.

## 7. C4 — `n108_08_gif_source_transcript_and_pixel_change_evidence`

Three independently constructed 1×1 GIF89a fixtures (shared RED/BLUE palette, `loop_count: None`, one GCE): **A** RED delay 0, **B** BLUE delay 0, **C** RED delay 5. A ran twice through separate supervised children. Assertions: A1/A2/B/C each prove real PID, limits and reap; `A.pixel == C.pixel == sha256([255,0,0,255])` and `B.pixel == sha256([0,0,255,255])` computed independently of the decoder (`canvas_v1`); `A` vs `B` and `A` vs `C` differ in both source and transcript digests; `C` keeps A's pixels but its cycle is 50 ms versus 0; A1 and A2 agree on source, transcript, transcript length, pixel and cycle while their PIDs differ. Local PIDs `52744, 52748, 52752, 52756`.

This is a deterministic real-worker integrity cross-check, explicitly **not** the later deliberately forged-red whole-owner V2 sensitivity acceptance.

## 8. D — exact eight-case two-host gate

Only the two existing `N108 real GIF worker canaries` steps changed (17 insertions, 9 deletions in `.github/workflows/rust-verify.yml`; every other step, gate, selector, retry policy and negative control untouched). Both hosts now assert the eight fully-qualified names above, exactly eight executed cases (`n108_0[1-8]_`), `test result: ok. 8 passed; 0 failed; 0 ignored`, and the absence of any `FAILED` or `ignored` N108 case. The Rust command is unchanged: `cargo test -p solo-cli --test media_worker_host_smoke --locked n108_ -- --nocapture --test-threads=1`. Bash keeps `set -euo pipefail` and its `grep` gates; PowerShell keeps its `$LASTEXITCODE` check.

**Gate sensitivity proven locally against the exact workflow text** (14 extracted Bash lines, 20 extracted PowerShell lines, disposable `.temp/wo112/` only, workflow never edited to simulate failure, no fixture left tracked):

| Injected condition | Linux gate | Windows gate |
|---|---|---|
| correct eight-case output | pass (0) | pass (0) |
| one required name missing | fail (1) | fail (1) |
| one case `FAILED` | fail (1) | fail (1) |
| seven-case count | fail (1) | fail (1) |
| one case `ignored` | fail (1) | fail (1) |

## 9. Local verification before any hosted dispatch

WSL **Ubuntu 26.04.1 LTS**, on the same mounted checkout:

| Command | Outcome |
|---|---|
| `cargo fmt --all -- --check` | clean (after `cargo fmt --all`) |
| `cargo check --workspace --all-targets --locked` | 0 errors |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | clean |
| `cargo test -p solo-adapters --lib --locked graphic_gif` | 14 passed |
| `cargo test -p solo-adapters --lib --locked n102_event` | 3 passed |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n108_` | **8 passed; 0 failed; 0 ignored** |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n097_` | 3 passed, 8 loud refusals (see limits) |
| `cargo test -p solo-cli --test unsafe_boundary --locked` | 22 passed |
| `cargo test -p solo-cli --test check-architecture --locked` | `architecture=legal packages=4 targets=33 files=63 edges=70` |
| `cargo test -p solo-cli --test check_architecture_fixtures --locked` | 35 passed |
| `cargo check --workspace --all-targets --locked --target x86_64-pc-windows-msvc` | clean type-check |
| `cargo test --workspace --all-targets --locked --no-fail-fast` | green except the same 8 N097 refusals |

Each C1–C4 case was committed only after its own focused local pass, and D only after the eight-case run and the negative-gate proof.

**Disclosed limits, not hidden:** this Windows 11 host has no MSVC linker (`link.exe` absent) and no `mkfs.btrfs` or passwordless `sudo`, so native Windows tests and the reflink-dependent suites cannot run locally. The eight N097 real-filesystem cases refuse **loudly by design** (`SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem …`) rather than running on an unverified filesystem; they and the `snapshot_proofs` set are exercised by the hosted Ubuntu job, which builds the disposable Btrfs fixture. No test was skipped, filtered, weakened or deleted locally.

## 10. Hosted proof at the exact technical SHA

Run `43595784309`, **attempt 1**, no retry — https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43595784309

| Job | Result | N108 gate | N102 gates |
|---|---|---|---|
| `verify (ubuntu-24.04)` `132449849631` | **success** | step 19 success, `8 passed; 0 failed; 0 ignored` | n/a |
| `verify (windows-2025)` `132449849704` | **success** | step 20 success, `8 passed; 0 failed; 0 ignored` | step 21 four canaries success, step 22 12/12 repeats success |

All 36 applicable steps per host passed: pinned checkout, pinned `1.99.1` toolchain (`rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`), locked fetch, cargo-deny 0.20.2, advisory negative probe, package-set gates, disposable Btrfs fixture, format, strict Clippy, full workspace tests, N097 evidence, N108 eight-case gates, N102 canaries, architecture checker and refusal self-test, test-failure tripwire, release build, release CLI smoke, forbidden-import refusal with restore, byte-for-byte reproducible double release build, and guaranteed fixture cleanup.

**Deterministic digests identical between Ubuntu and Windows** (only host-specific PIDs differ):

| Fixture | Source SHA-256 | Transcript SHA-256 |
|---|---|---|
| `n108_01` | `a06797608309dae0cf51654e275db7ad6a179845c29542443aeee6b4b12536b1` | `7e0b25b3c3a9ec30431db004e59c093e2ac2d0799773f131d64a665a780d834b` |
| `n108_02` | `dc425aeadeee8daa6324c0c6093710c69f6ad6bcf6d58c12571e88df1632b84f` | `8c02ba6fe2d4fea8edabba2378db8a696bd166b5fba72315a93924341019ac33` |
| `n108_03` restore-previous | `91635990bdb2edaabdbf0eb97399def3009a0a415236422cbc5440711d3d3300` | `8e0ef6881dc3429d5f4c4e082053b769463f430aee478bd95105266bc913af6d` |
| `n108_05` absent | `aef0685e1c2625c61fac6647d9d7dd908b5ecc7eb3ebcf46e857f55788484915` | `370a685d7526ccf614ab4e456958f8845a607076010af7e18bc824ae25db61a1` |
| `n108_05` finite one | `bca60135cdc1d42af5c4a73f8566d0624f1d85957173ed113542270c28e2f878` | `1651fc0b1b04fcc257fed9d1449617495d2412f54b1575f9e2b63199d8c12f58` |
| `n108_05` finite two | `bb84d9576897210a5bb0b9359b368643b4caf5233f07f9f4a86b671732ca0f33` | `18e651007f18a1c99f1a800cd382a051a38846d1cadcbb36df8c4b0691871e2a` |
| `n108_05` infinite | `5f775900be475b4c98fbdfea6b4307f4ae252b97cd14a1a63465f51c40580265` | `7ea3507e896432c7f2f28f305fbb2cc02020c5e3883421d123d841a3ee1d947c` |
| `n108_05` mixed `[0,7,3]` | `d6c2ee91389406160c9add7ae3eb567859ede5391451128493b22358509d0452` | `4f5fc8b8ba1ed589a48943cec62a478c3429b3aefde3c57ba24ded52da035ec1` |
| `n108_08` A/A2 (RED) | `aef0685e…4915` | `370a685d…b61a1` |
| `n108_08` B (BLUE) | `00f1198a007d6c57459a5ce2055b7d835b9fdfb0c2dc7bc33dbb61d3c5c937a9` | `5ab694498d977a6f9da6784b42d1b3414cd0b670384b6f60f3fcc44cb2aef0ea` |
| `n108_08` C (RED, 5 cs) | `a5081acf2018b57fc87c8cfa36048a8d385985b4a3a44cb198e17606c83b5cc5` | `ad43d6e7f035cdf9669ba4f6a937a94b839e4c04912f3b2e2e27c4ba18834d0d` |

Pixel digests: A1/A2/C `34aaa746c25a0f105c4316bbb1f009aa359f49582656ee97d73c58132d563423` (RED), B `b7d1b3a1104cc86b1cea310793cf777002db0517281d135a02de079b0ea87c23` (BLUE) — identical across hosts, matching the independently computed `sha256_media_worker_bytes_v1` of the 1×1 RGBA pixels. Real child PIDs: Ubuntu `5811, 5816, 5821, 5838, 5842` (`n108_01`–`n108_04`) and `5946, 5950, 5954, 5958` (`n108_08`); Windows `9452, 3108, 4512, 6480, 1368` and `7656`.

Controlled refusals in both logs: `frames-1025`, `canvas-8193`, `composed-134217728`, `duration-3932100`, `frame-outside-canvas` → `gif-worker-supervisor-refused`; `source-over-ceiling` → `gif-worker-source-rejected`; `duplicate-repeat`, `unknown-application`, `short-graphic-control`, `unterminated-lzw`, `missing-sub-block-terminator`, `truncated-compressed-block` → `gif-worker-supervisor-refused`.

**Windows N102 preserved unchanged:** `n102_01 extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true child_pid=4636`; the four-canary gate passed; twelve repeats with distinct PIDs `8516, 2604, 2128, 2596, 9396, 7004, 8524, 3188, 3160, 7736, 9452, 5712` ended `N102 repeat 12/12 passed`.

## 11. A1 component-security and version review

First-party trust surfaces are unchanged by this order: the CLI composition root, the confined child supervisor (`media_worker_host_native` on Windows unsafe handle/Job-Object boundary, `rustix` `openat2` `RESOLVE_*` on Linux), the GIF container scanner and strict transcript validator, and the test-only fixture encoder (no decoding authority, crate-private raw decoder).

Pinned direct dependencies (exact versions, no ranges): `serde =1.0.229`, `serde_json =1.0.154`, `image =0.25.10` with only `png`/`jpeg`/`webp` and no default features, `gif =0.14.2` (`std` only), Linux-only `rustix =1.1.6` (`fs`,`std`) and `libc =0.2.191`, Windows-only `windows-sys =0.61.2` with the ten enumerated Wdk/Win32 features, dev-only `syn =3.0.9` (`full`,`visit`) plus `serde_json`. `Cargo.lock` holds 43 packages and was imported byte-for-byte, so this order added no dependency, version, feature flag or toolchain change.

Scanner/toolchain/action pins: `cargo-deny 0.20.2` from the verified release archive (sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`, asset id `471598214`), RustSec advisory database commit `4ef2df8e806a65e2f1ecc14f56f51249062b23d6` fetched `2026-11-14T12:38:17Z`, toolchain `1.99.1` from `rust-toolchain.toml`, and `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` pinned by full commit SHA with credentials not persisted.

Findings: **advisories, bans, licences and sources all ok** on the unchanged locked graph, with no suppression; the advisory negative probe independently refused `time 0.3.46` under `RUSTSEC-2026-0009` with exit 1, proving live detection rather than a vacuous pass. One benign `warning[license-not-encountered]` on `deny.toml:51` reports that the allow-listed `Unicode-3.0` licence is not currently used by any locked package — an allow-list hygiene note, not a finding, and no licence obligation is unmet. No advisory, deprecation or support notice applies to a shipped component in this order; no component, action or dependency pin was changed. Residual observation for the Architect, recorded rather than acted on: `cargo-deny` itself is fetched at run time from a hash-verified archive, so its availability depends on that release remaining downloadable.

## 12. Preservation, cleanup and explicit nonclaims

`main` is descended from authority `dc266f55` (which descends from `9bf16d56`) and this report-only commit changes **only** `.documents/Reports/execution-report.md`; `main` still carries only PNG/JPEG/WebP product code, with `media_worker_host.rs` at the accepted `c6d78139` baseline form and no GIF/N102 integration. All 24 earlier remote refs are frozen at their exact SHAs — including `solo-wo-111-gif4-n102-r01` at `cdf0ceeb` and the first accepted technical head `371c15de` — plus the one new branch, for **25** refs total. Owner-installed ACTIVE bytes remain blob `b677df64`; the four WIP files are unchanged (`plan-roadmap` `9d7a8622`, `plan-next` `92fa562b`, `plan-outlook` `cb9c7a61`); Architecture, both SOPs, `AGENTS.md`, both checkers, root manifests, `deny.toml`, `.gitignore`, provider/release/deploy definitions and all Owner-owned/ignored data are untouched.

Task scratch existed only as owned `.temp/wo112/` (gate-sensitivity harnesses, extracted gate text, synthetic logs) plus a transient `.temp` baseline log and one transient workspace log directory; all identified task-owned material was removed before completion with no `git clean` and no deletion of unknown, shared or Owner material. The working tree was empty before the report commit.

**Nonclaims:** no product GIF acceptance or integration, no mixed-format PNG/JPEG/WebP/GIF V2 whole-owner inspection, no deliberately forged-red digest-sensitivity acceptance, no safe true-vector SVG rendering (never rasterised as a substitute), no public asset publishing, no architectural or contract change, no dependency/feature/toolchain upgrade, no new worker mode, no raw decoder export, no generic fault-injection API, no third-party encoder, no second Entity/Spoke/tenant feature, no PR, merge, rebase, reset, force-push, tag, release or deployment. This order delivers the four remaining N108 host canaries and their two-host gate only.

`R03.C3`, `R03.C` and Stage 03 remain **OPEN**. The Implementation Agent does not declare `ACCEPT` and does not close any of them.

## 13. Remaining committed obligations

Mixed-format PNG/JPEG/WebP/GIF V2 atomic whole-owner inspection; deliberate-red forged-digest sensitivity proof; exact-blob GIF integration into `main`; the Owner-approved safe **true-vector** sanitized SVG package (Architecture §6.2I, still unimplemented). Each requires its own separately authorized order.

**Final note for review:** the eight canaries exercise the production confined supervisor end to end on both hosts, all new digests are deterministic and identical cross-host, every adversarial specimen refuses with the intended closed token, the extended gate is proven sensitive to each negative condition, and no previous gate, test or resource rule was weakened.




