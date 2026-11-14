# Execution Report

**Work order:** `SOLO-WO-115 R02` — complete production AVIF + safe true-vector SVG, with pinned fixtures and Foundation-sourced library authority.
**Date:** 2026-11-14. **Target:** `R03C-SIX-FORMAT-AVIF-SVG-ON-MAIN-VERIFIED`.
**status=STOPPED — B0 proven on the technical branch; B1/B2/B3/D/I not delivered; nothing integrated into `main`.**

Checkpoint **A** (Owner authority) and checkpoint **B0** (pinned donor fixtures, provenance, native-prerequisite pinning) are complete and independently verifiable. The remaining package could not be implemented *and proved* within this run, and the order forbids integrating partial work, so `main` carries only the authority and this report. No codec, sanitizer, application, adapter, worker, workflow or notice byte was written; the technical branch holds the verified B0 checkpoint only.

## 1. Authority and checkpoints

Preflight at the order's exact baseline: local and remote `main` both `52773d25416b09a8f3e65b9d5f73130bc921bf5c`, tree `837129ed2f44b38e1033f9a30385b24a0caddfeb`, parent `b71832c7ff2e79311d644c6cfcf84a51d14712c5`; report blob `0c9d3182`; R01 ACTIVE `b895fe3d`; 26 refs; one checkout; one `origin`; empty stash; four WIP files; clean tree apart from the Owner-installed R02 ACTIVE (80 lines, raw and filtered blob `5defdd486f03d951f7a7653d52c518a7c96dc07c`). Frozen governance verified **11/11** (`AGENTS.md` `6eae1da8`, `ARCHITECTURE.md` `f66096ec`, Architect SOP `1483c9be`, Implementation SOP `51bb3792`, `architecture_checks/source.rs` `1c99d348`, `unsafe_boundary.rs` `57d75796`, root `Cargo.toml` `14505be4`, `rust-toolchain.toml` `9ec55b3b`, ROADMAP/NEXT/OUTLOOK `9d7a8622`/`92fa562b`/`cb9c7a61`).

| Step | Commit | Tree | Parent | Contents |
|---|---|---|---|---|
| baseline `main` | `52773d25416b09a8f3e65b9d5f73130bc921bf5c` | `837129ed` | `b71832c7` | accepted R01 STOP report |
| **A** authority | `f936a4539df8cd0f10896616d6ba9885636ab737` | `d2adb6e130b14e731edd7ac73f5417327853c942` | `52773d25` | ACTIVE alone, `5defdd48` |
| **B0** fixtures | `8c7b6573a9a97e1208a66aae62c18627e6d8ac0a` | `357c646857d4578ad945d5ff6d01c89d2d7df347` | `f936a453` | 7 AVIF + 2 SVG fixtures + provenance |
| **R** report | reported in chat after publication | — | `f936a453` | report alone on `main` |

`git diff-tree --name-only A` lists exactly `.documents/WIP/workorder-active.md`. Technical branch `solo-wo-115-avif-svg-v3-r02` was created once from exact A and is preserved at B0 (`8c7b6573…`); remote refs are now **27**, with `solo-wo-113-gif-v2-r01` `ad7b74df` and `solo-wo-112-gif8-host-r01` `40b03c73` untouched. **B1, B2, B3, D and I were not started.**

## 2. Proven evidence — B0 (`8c7b6573…`)

All seven AVIF specimens and both SVG specimens were fetched from the exact pinned commits and verified three ways: upstream Git blob, raw SHA-256 of the stored bytes, and — after `git add` — the **index** blob. All **9/9** index blobs equal the pinned upstream blobs, so the repository stores them byte-for-byte despite the Windows worktree.

| Stored path | Upstream donor @ commit | Git blob (index == upstream) | Raw SHA-256 | Bytes |
|---|---|---|---|---|
| `avif/white_1x1.avif` | libavif @ `37a5461` | `9d5c466444de5680b539673b26f5c770e98dacc0` | `ea4e43d1f07e4c00de16c13afa32376111bb306e51f08212cf4c1b6064df3667` | 305 |
| `avif/abc_color_irot_alpha_irot.avif` | libavif @ `37a5461` | `e5f8141cfa06ac705f3bcbec89d0e6470788bb6b` | `b371cc88244a873131e4d10ff9363d71ce4f41cf333bd4a491b38d970d9abd3b` | 10597 |
| `avif/abc_color_irot_alpha_NOirot.avif` | libavif @ `37a5461` | `e9dbdb49a30240ca7841af4d4577fa95942936ea` | `f2c8cd6ded641c68d13b3363417a62288a5eb335870de8d0b9da5093865ffb9a` | 10596 |
| `avif/clap_irot_imir_non_essential.avif` | libavif @ `37a5461` | `bcbe87dd30d567a7e74473ac91f5849f24a7f14c` | `33f869fcf2a879913eb394982b8fc03e9a60c25831aa37622ddefa656fd39fc1` | 653 |
| `avif/fox-profile0.8bpc.yuv420.avif` | av1-avif/Link-U | `cf23e6ea9fd5965ba665282581d1898996e9b7e1` | `a0ae1ad1aea81291730f42259593b297a8444699f4553de8dc25da2db56a40d9` | 63479 |
| `avif/fox-profile0.10bpc.yuv420.avif` | av1-avif/Link-U | `025c185783831a08193557d764f280b79a55e27b` | `1564a6cd6fd3350d6b02604c74894bb51f8133ff40fddcb86e521c88130c69f1` | 64420 |
| `avif/fox-profile2.12bpc.yuv420.avif` | av1-avif/Link-U | `4299cf745f7d6fd57a033d5fbb00f94b32cf1aa8` | `ce734812bb76659dbaf7c8a96321e12a4281fb7829180cadb685c14b9310fe45` | 65007 |
| `svg/icon-home.svg` | foundation @ `fa59997` | `3d738af7ff081cd372e408600ce025e464a37aac` | `cb742a3b1aa0007ec0c3f26f898e5833c6e67476e136b8eae3c52e2f2c751b5c` | 482 |
| `svg/TABLER-ICONS-MIT.txt` | foundation @ `fa59997` | `3e82379dab3fe93d9ee22251949604ed63ddea39` | `b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db` | 1073 |

`crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md` records per-file purpose, upstream path, blob, SHA-256, byte count and licence, and states explicitly that these are **test inputs only**, never platform assets or published media.

**Licence findings that correct an assumption carried from R01.**

* libavif's `LICENSE` (blob `350eb9d15ce1553438b09b8cd14c803e86da316b`) is **BSD-2-Clause**, `Copyright 2019 Joe Drago`, *not* Apache-2.0. The four libavif fixtures are "same as libavif" per the pinned `tests/data/README.md` (blob `655bc2244728f288d6b77d60bfafbe294e0ed126`).
* The three `fox` files are **CC-BY-SA 4.0**, author/origin Link-U, via `AOMediaCodec/av1-avif` `testFiles/Link-U` @ `77bd20d5…` (attribution README blob `243fa900216f4a060c95d1a79ff002ef6516f9b0`, which states "All images are licensed under CC-BY-SA 4.0"). They are **not** relicensed under Solo's Apache-2.0; ShareAlike attaches to those bytes.
* The Foundation icon is Tabler-derived **MIT** (`Copyright (c) 2020-2026 Paweł Kuna`), with the donor's own `icon-provenance.json` (blob `a74d234417d5bb63ff05e2b64f93d757864f3d46`) cited but not copied.

**Native prerequisite pinned exactly.** The dav1d source archive at the order's pinned commit was fetched and hashed:

* URL: `https://code.videolan.org/videolan/dav1d/-/archive/32cf02af50f32af108a3b281c452788dccdac648/dav1d-32cf02af50f32af108a3b281c452788dccdac648.tar.gz`
* **archive SHA-256 `82ef9029404dcc4992a774bbb3fd7ea7769745246969656bcd36155bd766d152`**, 1,708,752 bytes, HTTP 200.
* `COPYING` blob **`875b138ecf6f2b2b8e402c51ea541d7740246504`** — exactly the blob the order requires — BSD-2-Clause, `Copyright © 2018-2019, VideoLAN and dav1d authors`.
* `meson.build` confirms `version: '1.5.0'` and `meson_version: '>= 0.49.0'`.

**Branch baseline is green at B0** (fixtures add no compiled code): `cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` finished; `cargo clippy … -D warnings` **0 findings**; `check-architecture` → `architecture=legal packages=4 targets=33 files=63 edges=70`; `unsafe_boundary` **22/22**.

## 3. What is not delivered, and why

**Not delivered:** B1 (AVIF child decode + observations), B2 (SVG sanitizer), B3 (V3 six-format whole-owner matrix, red→green probes), D (technical two-host CI), I (integration). `main` therefore still contains **only** PNG/JPEG/WebP/GIF V1/V2; AVIF and SVG remain **unimplemented**, exactly as at the R01 STOP.

The blocker is scope-versus-proof, not a missing authorization. R02 resolved every R01 product question (D1 pinned fixtures, D2 code-defined glyphs, D3 pinned native dav1d, D4 mirror refusal), and the B0 work proves the inputs are lawful and fixed. What remains is a substantial engineering program that must be built *and* verified to this repository's standard before any of it may be integrated:

1. `graphic_avif_inspection.rs` — container/plane decode in the confined child, `irot` applied through the safe `mp4parse::image_rotation()` accessor, `imir` refused via the non-dereferencing `is_null()` presence check, inconsistent associated-alpha refusal, 20 MiB / 8192-side / 32 M-pixel / checked-budget limits, canonical RGBA8 output and digest.
2. A reproducible **native dav1d build on both hosted OSes** (Ubuntu 24.04 and native Windows 2025 with MSVC, Meson, Ninja, NASM), each pinned by exact tool identity and hash, with Cargo forced offline for the production build and the `dav1d-sys` network fallback provably never taken.
3. `graphic_svg_inspection.rs` — strict `quick-xml`/`svgtypes` parse, reject-by-default allowlist, the full mandated construct set (all path commands, shapes, groups/transforms, gradients, clipping, masks, bounded static filters, deterministic text), the original 5×7 glyph repertoire, all fourteen budgets, cycle detection, `currentColor` resolution, non-executable `class` validation, and a byte-identical canonical serializer.
4. V3 application port/types, adapter composition, closed child dispatch in `media_worker_host.rs`/`main.rs`, the N115 real six-format suite with red→green oracles, architecture counts, `MEDIA-CODEC-NOTICES.md`, then green D → I → R on both hosts.

Each can only be *proved* through repeated hosted Ubuntu/Windows runs; the AVIF and native half cannot be compiled at all on this Owner-local WSL (no `libdav1d-dev`, no `sudo`), which R02 explicitly permits, but which makes every AVIF iteration a remote round-trip. Implementing all of it and then proving it — including cross-host byte-identical SVG digests and a genuinely native dav1d build on Windows MSVC — is not achievable within this run's remaining execution window. Writing security-critical parsing and FFI-adjacent code that could not be compiled, tested and proven here would violate §2's "never integrate partial work" and the prohibition on simulated green gates, so none was written.

**Smallest Owner decision to unblock delivery:** split R02 into two sequentially issued orders that are each independently deliverable and provable — (i) `R03C-SVG-TRUE-VECTOR-VERIFIED`: B2 + the V3 SVG arm + local and two-host proof, fully verifiable on this host; then (ii) `R03C-AVIF-NATIVE-VERIFIED`: B1 + the native two-host dav1d build + the V3 AVIF arm + N115, inherently hosted-only. Alternatively, extend this order's execution window so both halves can be completed and iterated to green in one continuous run.

## 4. Cross-host determinism hazards already identified (for the next run)

These were found while preparing B0 and would otherwise cost a failed CI cycle:

1. **CRLF hazard on text fixtures.** The repository `.gitattributes` is `* text=auto`, and its binary list names `png/jpg/jpeg/gif/webp/ico/pdf/woff/woff2/zip` but **not** `.svg`. The index blob for `icon-home.svg` is correctly LF (`3d738af7…`), but a Windows checkout materializes CRLF in the worktree, so a test hashing raw worktree bytes would compute a different SHA-256 on Windows than on Ubuntu. Tests must normalize CRLF→LF for the two text fixtures before hashing, and the canonical serializer must always emit LF. `.gitattributes` is outside this order's ledger, so this must be handled in code rather than by adding an attribute.
2. **Canonical number formatting.** To keep sanitized SVG digests byte-identical across hosts, formatting must depend only on the parsed value (integer when exact, else fixed decimal precision with trailing zeros trimmed and `-0` normalized) and must never re-derive geometry through floating-point arithmetic; the only computed coordinates should be integer glyph cells.
3. **`image`'s AVIF decoder ignores `irot`/`imir`** (verified in the pinned `image-0.25.10` source: no transform reading anywhere in `src/codecs/avif/decoder.rs`), while it does decode alpha, 8/10/12-bit and YUV/YCgCo→RGBA. Rotation must be applied by our own checked code from `mp4parse`; any mirror must refuse atomically.
4. **`dav1d-sys 0.8.3` requires a system `dav1d >= 1.3.0` via `pkg-config`**, and its `build.rs` contains a `git clone` fallback to `code.videolan.org` that must be provably unreachable; the pinned archive hash in §2 is the input to the reproducible build.

## 5. Preservation, cleanliness and status

**Preserved:** the accepted PNG/JPEG/WebP/GIF V1/V2 implementation, all 15 WO-114 integration blobs, `Cargo.lock`, every `Cargo.toml`, `.github/workflows/rust-verify.yml`, `deny.toml`, frozen governance (11/11), all three `plan-*` files, both checkers and the architecture counts are byte-identical to `main` at `52773d25`. No `MEDIA-CODEC-NOTICES.md` was created because no notice duty was triggered by delivered code. `main` advanced only by A and this report; refs moved 26 → **27** solely because the authorized technical branch now exists; `ad7b74df` and `40b03c73` untouched; one checkout, one `origin`, empty stash, four WIP files, no second worktree, no merge/rebase/reset/cherry-pick/force-push/`git clean`, no tag, release, deployment, PR or provider write.

**Scratch:** all work was done in task-owned `.temp/wo115r02/` (fetch, licence, dav1d and verification scripts, downloaded specimens and the dav1d archive) and deleted at the end of the run; Owner `.temp/` directories were left untouched.

| Gate | State |
|---|---|
| Owner installation, A, governance, refs | **PASS** — A `f936a453` ACTIVE-only; 11/11; 27 refs |
| B0 pinned fixtures + provenance | **PASS** — 9/9 index blobs byte-identical; licences recorded |
| B0 baseline green | **PASS** — fmt, check, clippy 0, architecture legal, unsafe 22/22 |
| B1 AVIF / B2 SVG / B3 V3 | **NOT DELIVERED** |
| D / I | **NOT STARTED** — no CI dispatch, no integration |
| Partial work integrated | **None** — zero source/manifest/workflow bytes written |
| Fresh both-host `main` CI at R | Requested at exact R SHA; evidence in chat |

`status=STOPPED — B0 proven; B1/B2/B3/D/I not delivered; AVIF and SVG remain unimplemented; nothing integrated.` `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. No functionality was waived, no fallback invented, nothing released, deployed or closed, and the agent does not self-accept.



