# Execution Report

**status=STOPPED — GIF partial engineering, NOT ACCEPTED, not publishable; SVG still pending**

`SOLO-WO-107 R01` requires checkpoint set **B1+B2+B3+C+D+E** for `R03C-FULL-ANIMATED-GIF-INSPECTION-VERIFIED-AND-INTEGRATED`. **B1 is delivered, committed, pushed and green on both hosts at one exact SHA.** B2 (confined worker path and versioned evidence frame), B3 (atomic V2 mixed inventory), C (mandatory `n107_*` two-host gate), D (deliberate-red proof) and E (main integration) are **not** delivered. No GIF capability is claimed as accepted, integrated or publishable.

## 1. Preflight — all 23 frozen pins exact

`HEAD == main == origin/main == git ls-remote origin main == 78f4548a2b5b38de83ccc13179bf336d085c1772`, tree `240fd75c0a39c8fe3f395265fe7a050ceaf8f23b`, sole parent `c90d56b1aba2412b74690eeca08a29c5c1277343`; previous report `b6dfea76`; previously tracked ACTIVE `24caae4d`; Owner-installed WO-107 ACTIVE `5a120b3e45b1369fc0215cdbfc761760c5aa835b` installed byte-for-byte and never normalised; Architecture `f66096ec`; Architect SOP `1483c9be`; Implementation SOP `51bb3792`; `AGENTS.md` `6eae1da8`; NEXT `92fa562b`; ROADMAP `9d7a8622`; OUTLOOK `cb9c7a61`; root manifest `14505be4`; prior `Cargo.lock` `5a18352b`; toolchain `9ec55b3b`; adapter manifest `9c7d526d`; workflow `995bd28e`; adapter root `c009e428`; worker `90d67201`; confined host `c6d78139`; native seam `d411eaf4`; app inspection `bdba09c1`; app root `707f2f08`; smoke `88b4c6ab`; CLI main `ae1395b3`; graph `da7d7ee7`; AST checker `1c99d348`; unsafe guard `57d75796`. Nineteen remote refs; no `solo-wo-106-*`/`solo-wo-107-*` pre-existing; one worktree, empty stash, four WIP files; the only tracked dirt was the Owner-installed ACTIVE.

## 2. A — authority, and issuance CI

- **A** = `cf20f8736fb7c0fc0abfd53b6728c4cd8dfbbdc7`, tree `a6f3d1c7259639d2f9a5101095cc54b1c79113f7`, sole parent `78f4548a`, changing only `.documents/WIP/workorder-active.md` (`5a120b3e`).
- Issuance `push` run **`43577248439` GREEN on both hosts** at `cf20f873`.

## 3. B1 — first real-code checkpoint (delivered)

Branch `solo-wo-107-gif-r01` from green A:

- **B1** = `efc384e17f496522484dff09e0559b4724876829`, tree `37d061a3c9b3460aadaa7bc3698cb67d399df69e`, sole parent `cf20f8736fb7c0fc0abfd53b6728c4cd8dfbbdc7`. Commit message: `GIF decoder checkpoint - nonpublishing, NOT ACCEPTED`.
- Changed blobs: `crates/solo-adapters/src/graphic_gif_inspection.rs` `87602f3b2a33bb3baf1a16bc7c5a4b18530e6b30` (new), `crates/solo-adapters/src/lib.rs` `97b8f0ed5ea1ee4b474a2904ef385356c4d9bd13`, `Cargo.lock` `cf1051675576ab0e00ce7acbd5dbe139882c31b4`, `crates/solo-adapters/Cargo.toml`, `crates/solo-cli/tests/architecture_checks/graph.rs`, `.github/workflows/rust-verify.yml`.
- Branch `workflow_dispatch` run **`43577791335` GREEN on both hosts at `efc384e1`**: Ubuntu job `132395748559` success, Windows job `132395748911` success. That run exercised the full mandatory pipeline — N097 image tests, the four Windows N102 canaries, architecture, cargo-deny, release smoke and reproducibility — with the new codec present.

What the code actually does (not a label, not a stub):

1. `scan_container_v1` independently walks the **container grammar**: `GIF87a`/`GIF89a` signature, logical screen descriptor, global colour table, graphic-control/comment/application extensions, image descriptors with local colour tables, LZW data sub-block chains, and a trailer that must be the exact last byte. Unknown extension labels, non-NETSCAPE/ANIMEXTS application extensions, duplicate repeat extensions, zero-size frames, rectangles outside the logical screen, missing trailer and any trailing byte are typed refusals.
2. `inspect_gif_animation_v1` decodes **every** frame with `gif 0.14.2` under `ColorOutput::RGBA`, an explicit per-frame `MemoryLimit::Bytes(256 MiB)`, `check_frame_consistency(true)`, `check_lzw_end_code(true)` and `allow_unknown_blocks(false)`, then composites onto one bounded RGBA canvas honouring transparency, offsets and each frame's authored disposal (`keep`, `background`, `previous`), hashing the **fully composited canvas** per frame.
3. `GifAnimationEvidenceV1` is the closed, ordered transcript: signature, MIME, dimensions, frame count, loop semantics (`absent` / `finite(n)` / `infinite`), one-cycle duration, aggregate composited pixels, source SHA-256, and per-frame delay in **authored centiseconds**, rectangle, disposal, transparency index, interlace, palette source and composite SHA-256, plus a transcript SHA-256.

Budgets enforced with checked arithmetic: 20 MiB source, 1..=8192 per side, 32,000,000 canvas pixels, `MAX_GIF_FRAMES_V1=1024`, `MAX_GIF_TOTAL_COMPOSITED_PIXELS_V1=128_000_000`, `MAX_GIF_ONE_CYCLE_DELAY_MS_V1=3_600_000`, 1 MiB trace. Infinite repeat is preserved, never rewritten; a zero centisecond delay is preserved as authored.

## 4. Executable evidence produced locally and in hosted CI

Seven deterministic, self-generated unit tests exist in the new module and **all seven pass** (`cargo test -p solo-adapters --lib --locked graphic_gif`: `7 passed; 0 failed`):

| Test | What it proves |
|---|---|
| `gif87a_still_is_one_frame_and_not_an_animation` | GIF87a still accepted, `frame_count=1`, `is_animated=false`, loop `absent`, 0 ms cycle, composite digest equals SHA-256 of the single red RGBA pixel |
| `gif89a_multi_frame_preserves_delays_disposal_and_composition` | 3-frame GIF89a: authored delays `10/20/5` cs sum to **350 ms**, loop `infinite`, disposal `keep`/`background`/`keep`; frame 0 digest = red, frame 1 digest = red (transparent draw carried frame 0 forward), frame 2 digest = fully cleared canvas — the only way to obtain that pair is to honour `restore to background`; transcript digest equals SHA-256 of the transcript bytes |
| `finite_repeat_count_is_preserved_exactly` | `NETSCAPE2.0` count 3 → loop `finite(3)`, not normalised |
| `trailing_bytes_after_the_trailer_are_refused` | one appended byte → `gif-trailing-data` |
| `unsupported_extension_block_is_refused` | plain-text extension `0x21 0x01` → `gif-unsupported-extension` |
| `truncated_stream_is_refused` | cut trailer → `gif-truncated-container` |
| `oversized_logical_screen_is_refused` | 8193-pixel side → `gif-canvas-out-of-range` |

Local gates on `efc384e1`: `cargo fmt --all -- --check` clean; `cargo clippy -p solo-adapters --all-targets --locked -- -D warnings` **0 errors**; `cargo check -p solo-adapters --all-targets --target x86_64-pc-windows-msvc --locked` clean (type-check only — not Windows runtime proof); architecture `architecture=legal packages=4 targets=33 files=63 edges=70`. The five `source_read_native::snapshot_proofs` cases fail **locally only** because this WSL filesystem offers no reflink support; they are unchanged pre-existing environment-dependent cases and pass on hosted CI, which the green run above confirms.

## 5. Dependency audit

`gif = { version = "=0.14.2", default-features = false, features = ["std"] }`, `solo-adapters` only. `std` is required because the crate is `#![cfg(feature = "std")]` and the audited decode API is absent without it; no other feature was added, and `image =0.25.10` keeps its three original features. Locked closure grows by exactly two nodes — `gif` and `weezl` — recorded in `APPROVED_ADAPTER_CLOSURE` (now 39) and `APPROVED_ADAPTER_EXTERNAL_NORMAL` (now 7); the observed edge count rose from 68 to 70 with no wildcard or skip. Licence, advisory, ban and source policy are enforced by the hosted `cargo-deny` gates that passed in run `43577791335`; the pre-existing non-blocking `license-not-encountered` warning is unchanged.

## 6. Exact unmet gates

| Gate | State |
|---|---|
| **B2** confined single-use worker `gif` path, separate versioned bounded evidence frame, parent recomputation of transcript SHA-256, real `n107_*` two-host worker tests | **Not delivered** — the codec is a library module; the worker still dispatches only `png`/`jpeg`/`webp` |
| **B3** `GraphicMedia*V2` atomic mixed PNG/JPEG/WebP/GIF whole-owner inventory (internal, unlisted, suppressed, duplicate, wrong namespace, no fallback) | **Not delivered** — `crates/solo-application/src/graphic_media_inspection.rs` does not exist |
| **C** mandatory both-host `n107_` command plus named-case execution assertion in the workflow | **Not delivered** — workflow carries only the graph-count update |
| **D** `solo-wo-107-negative-gif-r01` deliberate-red forged transcript/frame-digest proof | **Not delivered** — no red branch, no mutation, no log |
| **E** exact-blob integration to `main` with fresh two-host CI | **Not performed** — nothing integrated; `main` remains at A |

The blocker is execution capacity in this run, not a governance contradiction: the remaining scope (worker protocol extension, V2 policy, host tests, red proof, integration and final report CI) is a further substantial increment. I stopped rather than weaken the evidence standard, and deliberately did **not** delete the working checkpoint.

## 7. Preservation, cleanup, nonclaims

`main` is still exactly A `cf20f873` with an empty tracked diff; the only remote advance is the new `solo-wo-107-gif-r01` ref (`efc384e1`), so there are now 20 remote refs and all 18 pre-existing non-main refs are byte-identical. Frozen and untouched: native seam, confined host, source-read and inventory boundaries, `media_worker.rs`, root manifest, Architecture, SOPs, AST checker, unsafe guard, `deny.toml`, toolchain, NEXT/ROADMAP/OUTLOOK, Owner `.temp/solo-wo-073-wip/`, retained patches and WSL cache. No scratch files were left behind outside the shared Cargo registry cache.

**Nonclaims:** no GIF inspection is reachable from the worker, the CLI or any public surface; nothing accepts, validates, sanitises or delivers GIF through the product; no animation is publicly served. No SVG work of any kind was performed and the binding §6.2I true-vector SVG commitment remains entirely open and next. No AVIF, catalogue, delivery, release, deploy, tag or self-acceptance. `R03.C`, `R03.C3` and Stage 03 remain open.
