# Execution Report

**Work order:** `SOLO-WO-115 R01` — complete AVIF + safe true-vector SVG inspection, real host proof and controlled `main` integration.
**Date:** 2026-11-14. **Approved outcome sought:** `R03C-SIX-FORMAT-AVIF-AND-TRUE-VECTOR-SVG-PRODUCTION-VERIFIED-ON-MAIN`.
**status=STOPPED — no codec source, manifest, lockfile, workflow or notice byte was written; `main` carries only the Owner authority and this report.**

Execution stopped at the order's own dependency-and-fixture gate, before checkpoint **B1**. The three blockers in §2 are each independently sufficient, are reproducible from the exact repository state, and each maps to an explicit STOP clause in the installed order (the native-`libdav1d` gate in §3 and the deterministic-text clause in §4). Nothing was simulated, no partial work was integrated, and no test, expectation, `#[ignore]` or workflow gate was weakened. §3 records capability facts established during the investigation so a resumed order can be written against verified rather than assumed behaviour.

## 1. Authority, preflight and checkpoint A

Owner installed the exact R01 text at `.documents/WIP/workorder-active.md` (87 lines, raw `git hash-object` and Git-filtered blob both `b895fe3dc65274ea7b50e4efc0abb3b15d6f32c0`) and issued the matching activation handoff.

Preflight verified: one Windows/WSL Solo checkout; one `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`; local and remote `main` both `98917578af9ce0201416461896bc0cd1a66da109`, tree `599586ec2857e53cffb9a790b837e432c98a3752`, parent `06217642`; canonical report blob `2cd116c3aa6f4a9d0ccfe068191de29a9b5ed631`; 26 remote branch refs; one worktree; empty stash; exactly four WIP files; clean tree apart from the Owner-installed ACTIVE. Frozen governance verified 11/11 at their pinned blobs — `AGENTS.md` `6eae1da8`, `ARCHITECTURE.md` `f66096ec`, Architect SOP `1483c9be`, Implementation SOP `51bb3792`, architecture source checker `crates/solo-cli/tests/architecture_checks/source.rs` `1c99d348`, unsafe checker `crates/solo-cli/tests/unsafe_boundary.rs` `57d75796`, root `Cargo.toml` `14505be4`, `rust-toolchain.toml` `9ec55b3b`, ROADMAP/NEXT/OUTLOOK `9d7a8622`/`92fa562b`/`cb9c7a61`.

| Step | Commit | Tree | Parent | Contents |
|---|---|---|---|---|
| initial `main` | `98917578af9ce0201416461896bc0cd1a66da109` | `599586ec` | `06217642` | accepted WO-114 integration |
| **A** authority | `b71832c7ff2e79311d644c6cfcf84a51d14712c5` | `f590f31d44f1216bd5cad835c5777eb8be59a586` | `98917578` | ACTIVE only, `b895fe3d`, raw == filtered == GitHub |
| **R** report | reported in chat after publication | — | `b71832c7` | report only |

`git diff-tree --name-only A` lists exactly `.documents/WIP/workorder-active.md`. **B1 (AVIF), B2 (SVG), B3 (V3 matrix), D (technical CI) and I (integration) were not started.** The technical branch `solo-wo-115-avif-svg-v3-r01` was **not created**, so the remote ref count remains **26**; `solo-wo-113-gif-v2-r01` `ad7b74df`, `solo-wo-112-gif8-host-r01` `40b03c73` and every other branch tip are untouched. No product, test, workflow, manifest, lockfile or notice file differs from the accepted WO-114 state.

## 2. The three blockers, with reproducible evidence

### Blocker 1 — the required AVIF positive fixtures cannot be produced lawfully inside this ledger

§5 of the order makes real 8-bit, 10-bit and 12-bit still AVIF decoding — with and without alpha and with orientation/rotation — **required cross-host positives**, and forbids completion without them. The only authorized fixture routes are "deterministic *source-code-defined* examples in the authorized test file, or … exact approved existing public-fixture bytes with source licence/hashes documented in the one report", while "no new fixture file", "no external `ffmpeg`/ImageMagick/SVG converters" and no `ravif` encoder apply.

* No `.avif` bytes exist anywhere in the resolved dependency sources: `find ~/.cargo/registry/src -iname '*.avif' | wc -l` → **0**.
* `image-0.25.10` ships **no** test assets at all — its published contents are `CHANGES.md Cargo.toml Cargo.lock LICENSE-APACHE LICENSE-MIT README.md benches src` (no `tests/` directory, zero bundled `.png/.jpg/.gif`).
* The order names **no** approved fixture byte sequence or SHA-256, so "exact approved existing public-fixture bytes" has no authorized referent; selecting third-party samples myself is precisely the "silently copy third-party artwork or a remote file" the order forbids.
* Producing valid AVIF requires an AV1 intra encoder. Every available encoder is excluded by the order (`ravif` is reachable only through the forbidden `avif` feature; `ffmpeg`, ImageMagick and external codec binaries are expressly banned). Hand-authoring a conformant AV1 keyframe — OBU framing, sequence header, tile group, entropy-coded coefficients — inside Rust source cannot be done correctly, and an incorrect fixture would prove nothing.
* A refusal-only AVIF path is not a substitute: the order states that unsupported required features must be a **STOP**, "not silently downgraded to a smaller canary acceptance".

**Consequence:** real AVIF decode cannot be proven on either host, because no lawful input bytes can be created or named. This alone blocks B1, D and I, and the §7 acceptance criteria.

### Blocker 2 — deterministic SVG text has no lawful glyph source (the order's own STOP clause)

§4 requires the positive construct set to include "**deterministic safe text representation**" converted to "deterministic SVG **vector glyph outlines** from a *fixed, internally specified and provenance-verified* glyph repertoire", with "no system-font dependency", no ambient/remote fonts and no host-dependent glyph selection — and then states: "If that representation cannot be given a fixed lawful glyph source/algorithm within the authorized code and notices, STOP for an Owner decision rather than claiming text is complete."

The installed ledger provides no path for a glyph source. A real outline repertoire is a font program — a third-party asset with its own licence and notice obligations — and the only authorized new documentation path is `.documents/About/MEDIA-CODEC-NOTICES.md`, "new **only if required by actual verified third-party licence/notice obligations**". No font file, glyph table, repertoire definition or outline algorithm is named or authorized anywhere in the order, and a new fixture or asset file is expressly forbidden.

**Consequence:** text cannot be represented as provenance-verified vector outlines, and silently dropping or refusing all text would contradict the mandated positive construct set. This is exactly the condition under which the order directs a STOP.

### Blocker 3 — native `libdav1d` is a substantive gate that cannot be satisfied or verified here

The authorized AVIF route is `image =0.25.10` with its opt-in `avif-native` feature, which resolves `mp4parse 0.17.0` and `dav1d 0.11.1` → `dav1d-sys 0.8.3`. `dav1d-sys` declares `[package.metadata.system-deps.dav1d] version = "1.3.0"` and probes a **system** library through `system-deps`/`pkg-config`. A scratch probe of exactly that feature set reproduced the gate:

```
thread 'main' panicked at dav1d-sys-0.8.3/build.rs:82:10:
called `Result::unwrap()` on an `Err` value: PkgConfig(
  pkg-config exited with status code 1
  > pkg-config --libs --cflags dav1d 'dav1d >= 1.3.0'
  Package 'dav1d' was not found in the pkg-config search path.
  The file `dav1d.pc` needs to be installed and the PKG_CONFIG_PATH environment variable
  must contain its parent directory.
```

Local host facts: `/usr/lib/x86_64-linux-gnu/libdav1d.so.7` (runtime) is present, but `libdav1d-dev` is **not installed** (`apt-cache policy libdav1d-dev` → `Installed: (none)`, candidate `1.5.3-1`) and **sudo is unavailable** (`sudo -n true` → unavailable). The `avif-native` feature therefore cannot be compiled, let alone executed, in this checkout — so no local-first evidence for B1 is possible, and none was fabricated.

For the hosted Windows job there is no pkg-config-metadata-bearing dav1d distribution; meeting §3 would require, per job, a pinned `meson`/`ninja`/`nasm`/Python toolchain, a pinned dav1d source build and a hand-authored `.pc` — permitted only as a fully captured provenance exercise with "exact version, upstream publisher, download URL, SHA-256, build/install tools, transitive native dependencies, LGPL/other licence and NOTICE/compliance … fully captured and actually verified in both hosted jobs, with no arbitrary package-manager latest/fallback download". That is a substantive gate, not an incidental step, and it cannot be attempted honestly from a host that cannot build the feature at all.

§3 states the required outcome plainly: "If these conditions cannot be met using only this ledger and the candidates above, report `STOPPED` with the concrete smallest Owner decision; do not simulate a green native AVIF gate or claim proof using the research harness."

## 3. Verified capability findings (to de-risk the resumed order)

These were established by source inspection of the exact resolved crates and by a disposable scratch probe kept entirely outside the repository. They are recorded so the next order can be drafted against proven behaviour.

**Exact resolved graph of the authorized candidate set** — `cargo metadata` on `image =0.25.10` (`avif-native`) + `quick-xml =0.42.0` + `svgtypes =0.16.1` resolves **76 packages**, including `image 0.25.10`, `dav1d 0.11.1`, `dav1d-sys 0.8.3`, `mp4parse 0.17.0`, `quick-xml 0.42.0`, `svgtypes 0.16.1`, `av-data 0.4.4`, `system-deps 7.0.8`, `pkg-config 0.3.34`, `byteorder 1.5.0`, `bytes 1.12.1`, `fallible_collections 0.4.9`, `toml 1.2.1`, `cfg-expr 0.20.11`, `target-lexicon 0.13.5`. Nothing was added to the repository; `Cargo.lock` on `main` is unchanged.

**`image` 0.25.10 AVIF decoder (`src/codecs/avif/decoder.rs`) genuinely covers the pixel work:**
* alpha is decoded as a second `dav1d` picture (`alpha_picture: Option<dav1d::Picture>`, `ctx.alpha_item_coded_data()`), with a hard 4:0:0 layout requirement;
* bit depth is asserted to be 8, 10 or 12 (`assert!(bit_depth == 8 || bit_depth == 10 || bit_depth == 12)`), with `yuv.rs`/`ycgco.rs` providing 4:2:0/4:2:2/4:4:4 YUV and YCgCo → RGBA conversion at 8, 10 and 12 bits;
* decoding is driven by `mp4parse::read_avif(ParseStrictness::Normal)` plus `dav1d::Decoder`, i.e. a real container plus real AV1 decode, not a heuristic.

**Two decoder limitations that matter for the order's requirements:**
* the module contains **no** `irot`/`imir`/transform handling at all (no reference anywhere in `decoder.rs`), so container-declared rotation and mirroring are silently *not applied*. The order forbids exactly that kind of silent reinterpretation, so a resumed order must either apply the transform explicitly or refuse such files atomically.
* `mp4parse 0.17.0` does expose the data needed: `AvifContext::image_rotation() -> Result<ImageRotation>` is a **safe** enum accessor (so `irot` can be applied without new `unsafe`), while `AvifContext::image_mirror_ptr() -> Result<*const ImageMirror>` returns a **raw pointer**, so applying `imir` would require dereferencing it and therefore new `unsafe`, which the order forbids. Detecting and refusing mirroring via `is_null()` is possible without dereferencing.

**Native dependency behaviour** — `dav1d-sys 0.8.3` requires a system `dav1d >= 1.3.0` through `pkg-config`; its `build.rs` also contains an `add_build_internal` closure that would `git clone --depth 1 -b 1.5.0 https://code.videolan.org/videolan/dav1d.git` — a build-time network source acquisition that any resumed order must ensure can never trigger (a missing `.pc` must fail closed). In the probe the pkg-config error surfaced as a hard failure rather than a silent clone, which is the behaviour the product build should be required to guarantee explicitly.

## 4. Smallest Owner decisions required to resume

* **D1 — AVIF fixtures.** Name the exact approved AVIF byte sequences for the required 8/10/12-bit, alpha, `irot` and `imir` positives (URL + SHA-256 + licence/provenance, permitted as inline `const` bytes in the authorized test file), **or** authorize one named encoder (e.g. `ravif` at a pinned version, or a pinned `libavif`/`avifenc` release) strictly for one-off fixture generation with the resulting bytes committed inline under a documented licence.
* **D2 — SVG text glyphs.** Authorize a lawful, fixed glyph source for deterministic text→vector outlines: either a named OFL/Apache-licensed font file with an authorized permanent path and a `MEDIA-CODEC-NOTICES.md` entry, or an explicit statement that only an internally defined algorithmic glyph repertoire is required, with all other text refused atomically.
* **D3 — native dav1d acquisition.** Confirm the per-host model (Ubuntu: pinned `libdav1d-dev` `.deb` with SHA-256; Windows: pinned dav1d source tarball + pinned `meson`/`ninja`/`nasm`/Python versions and SHAs producing a `.pc`), and require the build to fail closed rather than fall back to the `dav1d-sys` network clone.
* **D4 — mirroring policy.** Confirm that `imir`-bearing AVIF is refused atomically (no new `unsafe`), or authorize a specific safe accessor for mirroring.

Until these are settled, B1's required positives cannot be produced and B2's text requirement cannot be met — so D, I and the §7 acceptance criteria are unreachable, and integrating a partial package is expressly forbidden by the order.

## 5. Preservation, cleanliness and Stage status

**Preserved exactly:** all 15 accepted WO-114 integration blobs and the whole accepted PNG/JPEG/WebP/GIF V1/V2 + N102 baseline are byte-identical to `main` at `98917578`; `Cargo.lock`, every `Cargo.toml`, every source and test file, `.github/workflows/rust-verify.yml`, `deny.toml`, `AGENTS.md`, `ARCHITECTURE.md`, both SOPs, all three `plan-*` files and the architecture/unsafe checkers are unchanged. No `MEDIA-CODEC-NOTICES.md` was created, because no third-party notice obligation actually arose. Remote state: `main` advanced only by A and this report; **26** refs before and after; `solo-wo-113-gif-v2-r01` still `ad7b74df`, `solo-wo-112-gif8-host-r01` still `40b03c73`; one checkout, one `origin`, empty stash, four WIP files, no second worktree, no branch created, no merge, rebase, reset, cherry-pick, force-push, `git clean`, tag, release, deployment, PR or provider access.

**Task-owned scratch:** all investigation material was created under `.temp/wo115/` (probe scripts, a scratch `probe-crate`, its `target/`, and downloaded `.crate` files) and disposable WSL `/tmp` JSON, entirely outside the tracked tree, and was deleted at the end of the run; `.temp/wo115` no longer exists and `git status --porcelain` is empty. Cargo's shared registry cache (`~/.cargo/registry`) received the crates resolved by the probe; that is the tool's own cache rather than repository or task material, and it was not added to the project. Pre-existing Owner `.temp/` directories (`architecture-fixtures`, `solo-win-target`, `solo-wo-073-wip`, `solo-wo-098-wip`, `solo-wo-103-wip`, `solo-wsl-target`, `toolchain-installer`) were left untouched.

**Stage status:** `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, with AVIF and safe true-vector SVG **still unimplemented**; the pinned `platform/assets/` catalogue, derived-output/exposure obligations and Stage 04 routing remain outstanding. No release, deployment or Stage closure is claimed, and no requirement was waived or downgraded.

## 6. Status

| Item | State |
|---|---|
| Owner installation/handoff and A | **PASS** — raw == filtered == GitHub `b895fe3d`; A `b71832c7` ACTIVE-only |
| Frozen governance and protected refs | **PASS** — 11/11 blobs; 26 refs; prior branch tips unchanged |
| B1 production AVIF | **NOT STARTED — blocked** (fixtures + native dav1d) |
| B2 safe true-vector SVG | **NOT STARTED — blocked** (glyph source; order's own STOP clause) |
| B3 / D / I | **NOT STARTED** — no branch, no CI dispatch, no integration |
| Partial work integrated | **None** — zero codec/source/manifest/lock/workflow bytes written |
| Local gates | **Not applicable** — no code changed; the pre-existing baseline was not re-run for this STOP |
| Fresh both-host `main` CI at R | Requested at the exact R SHA; evidence reported in chat, since a report cannot embed its own commit |

`status=STOPPED — dependency and fixture gates not satisfiable under the installed ledger.` The three blockers are reproducible from the exact repository state and from the exact pinned crates; two of them are the order's own declared STOP conditions. The agent does not self-accept, does not claim any AVIF or SVG capability, and awaits an Owner decision on D1–D4 before any resumed order.



