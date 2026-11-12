# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-068 R01` — R03.C3 recovery, bounded decoder-selection and dependency/licence/security feasibility qualification (nonimplementing).
**Matching Owner→Implementation Agent handoff:** `SOLO-WO-068 R01`.
**Session date (ISO):** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo`; one worktree, sole `origin`.
**Disposition: `COMPLETE` for the bounded *research task*** (as §4E permits) with the **four-format route `UNVERIFIED`/`BLOCKED` on AVIF**, three of four roots `STATICALLY PROMISING — NOT IMPLEMENTATION-APPROVED`, and **no** implementation, dependency, lock, policy, workflow or branch change. `R03.C3` is **not** implemented, no media verification is claimed, `R03.C` and Stage 03 remain **OPEN**, and this report is **not** a self-ACCEPT.

## 1. Anchors verified

| Anchor | Value |
| --- | --- |
| Baseline `main`/`origin/main`/HEAD | `017272d61aa9a00057a4a486ce5e51e4a85dae0b`, tree `dfb67e808d4b4e8a067571814188dea61d80d791` |
| Baseline report blob | `ff67067f4576e05040b4479dab30d06e77c8aab6` (the accepted `SOLO-WO-067 R01 — STOPPED` revision) |
| Baseline WIP | ACTIVE `6acea804…`, NEXT `92fa562b…` (neutral), ROADMAP `9d7a8622…`, OUTLOOK `cb9c7a61…` |
| Branches | `main` and the divergent retained `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-067-r01` correctly absent |
| Pre-issuance difference | only the Owner-installed ACTIVE (`cb3f9af82aaeb5116eb8254ccee9c0ddf187bf7b`, 23,294 bytes, 126 lines); handoff ID matched |
| Checkouts / worktrees / remotes | 1 / 1 / 1, no sequencer, conflicts or unknown material |
| **Issuance** | `a5085350e172c7ea3d1ca248e722645236f96891`, tree `bf123dcec07492910924953f4cd7ce0288bb680b`, parent `017272d6…`, **ACTIVE alone**; NEXT/ROADMAP/OUTLOOK/report byte-identical |
| Issuance CI | run `43144651351` **success** — Ubuntu `130939511437` (407 passed), Windows `130939511050` (412 passed), 33 steps each, 0 failed / 0 ignored |

`[license-not-encountered]` (`Unicode-3.0`) is still emitted on both hosts: inherited and **UNVERIFIED**, not suppressed.

## 2. Method, probe identity and honest local limits

Probe manifest (`probe-four`, Solo `.temp/` scratch, independent `[workspace]`, resolved against crates.io, then deleted):

```toml
[package]
name = "probe-four"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
zune-png = "=0.5.2"
zune-jpeg = "=0.5.15"
image-webp = "=0.2.4"
oxideav-avif = "=0.0.11"
```

Resolution: **26 packages** (25 third-party + the probe root), lock SHA-256 prefix `6EEC86EAB1DF5C05`, zero duplicate package names. Sources read from the published crate tarballs unpacked by Cargo in the registry cache, not from docs.rs or READMEs alone.

**Local limits (recorded, not worked around).** This host has no `link.exe`, no Visual Studio, no Windows Kits/SDK and no MinGW, so **no Rust binary can be linked and no decode test can be executed here**; `cargo-deny` 0.20.2 is also **absent**, and installing it or a linker would violate §2's no-installation rule. Consequently §4C.4 pixel execution and §4C.2 candidate-only scanner run are **UNVERIFIED by measurement**, and every decoder conclusion below rests on exact published manifests, locked checksums, and the crates' own source, tests and fixtures. No runtime success is implied anywhere in this report.

## 3. Four-root qualification matrix

| Root | Exact version · licence · MSRV | Provenance | Assessment |
| --- | --- | --- | --- |
| **PNG** `zune-png` | `0.5.2` · `MIT OR Apache-2.0 OR Zlib` · 1.87.0 | checksum `5a321146…aefcea80`, published 2026-03-11, not yanked, OSV/RustSec: none | **`STATICALLY PROMISING — NOT IMPLEMENTATION-APPROVED`** |
| **JPEG** `zune-jpeg` | `0.5.15` · `MIT OR Apache-2.0 OR Zlib` · 1.75.0 | checksum `27bc9d5b…7f8dbd7296`, published 2026-03-26, not yanked, OSV/RustSec: none | **`STATICALLY PROMISING — NOT IMPLEMENTATION-APPROVED`** |
| **WebP** `image-webp` | `0.2.4` · `MIT OR Apache-2.0` · 1.80.1 | checksum `525e9ff3…1909e8c3`, published 2025-08-27, not yanked, OSV/RustSec: none | **`STATICALLY PROMISING — NOT IMPLEMENTATION-APPROVED`** (one documented internal bound gap) |
| **AVIF** `oxideav-avif` | `0.0.11` · `MIT` · 1.80 | checksum `3c940990…73387e91`, published 2026-09-01, not yanked, OSV/RustSec: none | **`UNVERIFIED` / `BLOCKED`** — see §6 |
| (pulled) `oxideav-av1` | `0.1.23` · `MIT` · **1.95** | checksum `7dec6727…b561591d`, published **2026-11-08**, not yanked, OSV/RustSec: none, **152 downloads** | provenance risk; see §6 |

All five locked checksums were compared against the crates.io published checksums and **match exactly**. Every MSRV is satisfied by the pinned `1.99.1` toolchain. No yanked release, no advisory in either RustSec or OSV for the four roots or their three `oxideav` companions, and no crate in the graph carries a `links` key, a `build.rs`, `asm!`/`global_asm!` or `extern "C"`.

## 4. Combined graph, duplicates, licences and accepted-pin impact

Resolved graph (25 third-party packages): `bytemuck 1.25.2`, `byteorder-lite 0.1.0`, `image-webp 0.2.4`, `itoa 1.0.19`, `memchr 2.8.3`, `oxideav-av1 0.1.23`, `oxideav-avif 0.0.11`, `oxideav-core 0.1.43`, `proc-macro2 1.0.107`, `quick-error 2.0.1`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `simd-adler32 0.3.10`, `syn 3.0.10`, `thiserror 2.0.21`, `thiserror-impl 2.0.21`, `unicode-ident 1.0.27`, `zmij 1.0.23`, `zune-core 0.5.3`, `zune-inflate 0.2.54`, `zune-jpeg 0.5.15`, `zune-png 0.5.2`.

- **Duplicate versions: none.** Unlike the superseded `image 0.25.10` route, every package name resolves exactly once, so `[bans] multiple-versions = "deny"` is not challenged by this graph.
- **New versus the accepted 18-package closure: 14** — `bytemuck`, `byteorder-lite`, `image-webp`, `oxideav-av1`, `oxideav-avif`, `oxideav-core`, `quick-error`, `simd-adler32`, `thiserror`, `thiserror-impl`, `zune-core`, `zune-inflate`, `zune-jpeg`, `zune-png`. Eleven others (`itoa`, `memchr`, `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_derive`, `serde_json`, `syn`, `unicode-ident`, `zmij`) are already accepted.
- **Accepted pins that would move in a merged graph:** `itoa 1.0.18 → 1.0.19` and `syn 3.0.9 → 3.0.10` — semver-compatible patch moves of transitive crates Solo does not pin directly; recorded, not applied.
- **Licence fit against the unchanged `allow = [Apache-2.0, MIT, Unlicense, Unicode-3.0]`:** every new expression holds an allowed branch — `bytemuck` `Zlib OR Apache-2.0 OR MIT`, `byteorder-lite` `Unlicense OR MIT`, `image-webp` `MIT OR Apache-2.0`, `oxideav-*` `MIT`, `quick-error` `MIT/Apache-2.0`, `simd-adler32` `MIT`, `thiserror(+impl)` `MIT OR Apache-2.0`, `zune-*` `MIT OR Apache-2.0 OR Zlib`. **No `deny.toml` licence entry would be needed**, and no `MPL-2.0`, GPL, AGPL, SSPL or commercial expression appears in the graph. The scanner verdict itself is **UNVERIFIED** because `cargo-deny` could not be run here.
- **Build/toolchain surface:** none of the eight codec packages ships a build script. The only build scripts/proc-macros are `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_json`, `syn`, `zmij` (all already accepted) plus **new** `thiserror` (build script) and `thiserror-impl` (proc-macro) — ordinary and permissive, needing no external tool. No NASM, no `cc`, no compiler, no FFI, no assembly.
- **Unsafe surface (third-party SIMD/arena kernels):** `zune-jpeg` 79 blocks/fns, `zune-png` 12, `oxideav-core` 7, `oxideav-av1` 2, `zune-core` 0, `zune-inflate` 0, `image-webp` 0, `oxideav-avif` 0. No first-party Solo `unsafe` is implied; the application's `#![forbid(unsafe_code)]` is untouched.
- **Notice/attribution:** `zune-*` ship Apache/MIT/Zlib licences, `image-webp` and `thiserror` Apache/MIT, `oxideav-*` a single MIT `LICENSE`; `zune-inflate 0.2.54` ships **no** licence file although its manifest declares `MIT OR Apache-2.0 OR Zlib` — a minor packaging note for the future notice review, not a conflict.

## 5. Reconciliation with the `SOLO-WO-067 R01` STOP

| Earlier blocker | Status now | Basis |
| --- | --- | --- |
| `avif-decode 3.0.0` → `rav1d` `asm`, NASM + `cc` build surface | **Avoided, not waived** | `rav1d`, `nasm-rs`, `cc` absent from the new graph; `oxideav-av1` is pure Rust with no build script and no assembly. The old root stays disqualified exactly as reported. |
| `avif-parse 2.1.0` `MPL-2.0` transitive | **Avoided, not waived** | No `MPL-2.0` expression in the 26-package graph; no licence entry added or waived. |
| `png 0.18.1` forcing two `miniz_oxide` versions | **Avoided, not waived** | `png`, `flate2`, `miniz_oxide` all absent; `zune-png` inflates through `zune-inflate`/`simd-adler32`. `[bans] multiple-versions = "deny"` remains intact and unmodified. |

Nothing in `deny.toml`, the architecture checkers, the workflow or the toolchain was edited, and no advisory, duplicate-version or licence check was relaxed. The superseded STOP revision stays in Git history and is not reclassified.

## 6. AVIF — the decisive unresolved component

**Evidence that a real decoder exists.** `oxideav-avif 0.0.11` (23,210 LoC, 20 files, 804 `#[test]`s) exposes `decoder::AvifDecoder::decode_file(&[u8]) -> Result<AvifInfo>`, which walks the HEIF meta boxes, decodes the primary `av01` item (or the grid primary) through `oxideav-av1`, applies alpha compositing and the `clap`/`irot`/`imir` transforms, and returns `AvifFrame`/`AvifPlane` pixels. It ships **12 committed AVIF fixtures** (`alpha_video`, `bbb_alpha`, `black420`, `gray32`, `kimono_rotate90`, `midgray`, `monochrome`, `red`, `white16`, `y_plane_divergence_match`, `y_plane_roundtrip_avif1/2`) plus a 171 KB `integration.rs` and `fuzz_regressions.rs`. `oxideav-av1 0.1.23` (137,184 LoC, 67 files, 2,104 `#[test]`s) contains a full AV1 entropy stack — `cdf.rs` alone is ~26,000 lines covering partition, skip, intra/inter modes, CDEF and loop restoration — and a spec-driver `decode_av1_spec`. Only the **encoder** returns `Error::NotImplemented`, which is what the contradictory upstream notes describe.

**Evidence against relying on it.** (i) The crates.io description of `oxideav-av1 0.1.23` reads *"Pure-Rust AV1 codec — orphan-rebuild scaffold pending clean-room re-implementation"*, which **disagrees** with the crate's own README (byte-identical third-party pixel parity); §4D classifies source disagreement as **`UNVERIFIED`**. (ii) It was published **2026-11-08, three days before this session, with 152 downloads** — materially weaker provenance than any other component. (iii) `oxideav-avif 0.0.11` was published 2026-09-01 and declares `oxideav-av1 = "0.1"`, so the resolved `0.1.23` is **newer than the release the container was built against**: a pairing never co-published. (iv) `oxideav-core`'s `DecoderLimits` (`max_pixels_per_frame`, `max_alloc_bytes_per_frame`, `max_alloc_count_per_frame`) has **zero call sites** in `oxideav-avif` and `oxideav-av1`, so no auditable whole-working-set budget is wired into the AVIF path. (v) No decode was executed here, so no pixel output, checksum or conformance result exists from this session.

**Bounds that do exist:** `oxideav_av1::frame_driver::MAX_PICTURE_SIZE = 35_651_584` is enforced pre-allocation in `tile_list.rs` (`output_w * output_h > MAX_PICTURE_SIZE` → error), and `SpecDecodeSession::set_max_picture_size` exists but is **not exposed through `oxideav-avif`**. The container does expose header-level inspection (`inspect`, `AvifInfo`), so an adapter could reject oversized images between header parse and decode — but the 32,000,000-pixel contract and the 8192 cap would still need an explicit caller-side check, and total allocation stays unbounded inside the crate.

**Conclusion:** AVIF is **`BLOCKED` as a qualified component today** and **`UNVERIFIED` as a working decoder**. Nothing container-level was promoted to a pixel proof.

## 7. Resource-bound feasibility against the accepted contract

| Format | Dimension bound before allocation | Caller-owned output buffer | Animation handling | Working-set bound |
| --- | --- | --- | --- | --- |
| PNG | **Yes** — `readers.rs:38-50` rejects `width > options.max_width()` / `height > max_height()` during header parse | **Yes** — `decode_into(&mut [u8])` with `TooSmallOutput(len, got)`; `output_buffer_size()` gives the exact need | Detectable (`acTL`, `is_animated()`), so animated PNG can be refused | Bounded by caller allocation plus inflate window; per-buffer, one image at a time |
| JPEG | **Yes** — `headers.rs:215-216` rejects `width > max_width()` (and height) at header parse | **Yes** — `decode_into(&mut out)`; `output_buffer_size()` | N/A (single frame) | Bounded by caller allocation |
| WebP | **Yes** by caller after `dimensions()` (decoder exposes `dimensions()`, `has_alpha()`, `is_animated()`, `num_frames()`) | **Yes** — `read_image(&mut [u8])` | Detectable and refusable via `is_animated()`/`num_frames()` | **Partial** — `set_memory_limit()` defaults to `usize::MAX` and its own source notes *"TODO: Some allocations currently ignore this limit"* |
| AVIF | **Only via the crate's fixed 35.65 M-pixel ceiling**; the stricter 32 M / 8192 contract needs an adapter check after `inspect`, before `decode_file` | **No** — `decode_file` returns owned planes | Grid/alpha/`avis` paths exist; sequence handling would need explicit exclusion | **Unbounded/`UNVERIFIED`** — `DecoderLimits` unwired |

The two-phase pattern (parse headers → enforce 20 MiB / 1–8192 / ≤32,000,000 with checked arithmetic → decode into a pre-sized caller buffer) is achievable for PNG, JPEG and WebP with no library change. For AVIF only the dimension half is reachable; the allocation half is not, so §4C.5's requirement is **UNVERIFIED/FAIL** for that format while remaining **PASS-able** for the other three.

## 8. Deltas a future code order would need (documented, **not** applied)

- `crates/solo-cli/tests/architecture_checks/graph.rs`: `APPROVED_ADAPTER_EXTERNAL_NORMAL` 4 → 8 roots (`+ image-webp, oxideav-avif, zune-jpeg, zune-png`); `APPROVED_ADAPTER_CLOSURE` 18 → 32 entries (`+` the 14 new packages). Existing inward, dev/test, alias, unapproved-root and source checks must stay.
- `crates/solo-cli/tests/workspace_graph.rs`: matching positive fixtures and negative tests for the four new roots, an aliased crate and an undeclared transitive.
- `.github/workflows/rust-verify.yml`: the exact `edges=…` assertion must be **re-measured** (new roots and transitive edges change it); no other workflow change is permitted.
- `Cargo.lock`: `+14` packages and the two patch moves in §4.
- `deny.toml`: **no change anticipated** for licences (every expression holds an allowed branch); `[bans] multiple-versions`, sources and advisories stay untouched.
- `crates/solo-adapters/Cargo.toml`: four exact roots with explicit feature sets; `default-features` analysis is required because `zune-png` defaults to `sse`+`std` and `zune-jpeg` to `x86`+`neon`+`std`, while `zune-core` exposes `set_use_unsafe(false)` and `set_strict_mode(true)` for a conservative posture.

## 9. Acceptance matrix

| Gate | Status | Evidence |
| --- | --- | --- |
| `S068-01` | **PASS** | §1: exact baseline SHA/tree/report/WIP/branches, single checkout/worktree/`origin`, only the installed ACTIVE differing, matching handoff. |
| `S068-02` | **PASS** | Issuance `a5085350…` changed **ACTIVE alone**, parent `017272d6…`; remote readback matched; two-host CI green. |
| `S068-03` | **PASS** | NEXT `92fa562b…`, ROADMAP `9d7a8622…`, OUTLOOK `cb9c7a61…` byte-identical at issuance and at this HEAD; four WIP files only; previous STOP not reclassified; no branch, PR or product edit. |
| `S068-04` | **PASS** | All four exact roots assessed from published crate source, manifests, features and resolved target graph; contrary evidence (§6) overrides README marketing. |
| `S068-05` | **PASS** | The three earlier causes are confirmed and **avoided by substitution**, with the old roots still disqualified and no waiver (§5). |
| `S068-06` | **PASS** | Combined graph enumerated with zero duplicates, target-gated edges checked, no build scripts in any codec crate, and future checker/lock/workflow deltas documented without editing them (§4, §8). |
| `S068-07` | **PARTIAL / UNVERIFIED** | Licences, checksums, yank and OSV/RustSec posture classified per package and all five root checksums match crates.io; but no scanner run exists and `oxideav` freshness/provenance is flagged rather than cleared. No security or legal decision is recorded as PASS. |
| `S068-08` | **UNVERIFIED** | No pixel decode was executed — this host cannot link a binary (no `link.exe`, VS, Windows SDK or MinGW). Reason recorded; no header-only or container-only result was promoted. |
| `S068-09` | **PASS** | One ranked recommendation plus an explicit AVIF `BLOCKED`/`UNVERIFIED` classification; nothing promoted to a code order or approval. |
| `S068-10` | **PASS** | Issuance CI `43144651351` and the final report CI: Ubuntu 407 / Windows 412, 0 failed, 0 ignored, no skipped security or architecture step, inherited `Unicode-3.0` warning intact. |
| `S068-11` | **PASS** | Report-only final commit with remote blob/tree/parent readback, preserved divergent branch, four WIP files, unchanged product/lock/policy/workflow, clean worktree, only task-owned scratch removed (§11). |

## 10. Ranked recommendation and Owner decisions

**Rank 1 (recommended) — authorize one bounded AVIF runtime-and-bounds proof order** for `oxideav-avif =0.0.11` with `oxideav-av1 =0.1.23` pinned exactly: run the crate's 12 fixtures plus synthetic 8/10/12-bit, alpha, grid and hostile inputs on both accepted hosts, record decoded dimensions and pixel checksums, and instrument peak allocation against the 32 M-pixel / 8192 contract with an explicit pre-decode adapter check. This is the smallest step that preserves the binding four-format contract, keeps every existing policy intact and resolves the only remaining unknown; it must also settle the "orphan-rebuild scaffold" provenance question, because a three-day-old component with 152 downloads should not enter `main` on a README's word.

**Rank 2 — decide whether the four-format contract may ship as two increments**, i.e. authorize a PNG/JPEG/WebP-only nonpublishing increment using the three `STATICALLY PROMISING` roots while AVIF remains open. Only the Owner can change that scope, and such an increment could never be presented as complete four-format media verification.

**Rank 3 — separately assess `MPL-2.0` obligations** for the `avif-parse`/`avif-decode` route. Even with a licence decision that route stays disqualified by `rav1d`'s NASM assembler requirement, so it cannot unblock C3 alone.

**Rejected background (not executable):** `zenavif` (AGPL-3.0/commercial), `libavif`/`dav1d` (C libraries), `image`'s `avif`/`avif-native` paths (`ravif`/`dav1d`), `rav1d` (`asm`).

**Owner decisions actually needed:** (D1) authorize Rank 1, 2 or 3; (D2) accept or decline the `oxideav` provenance/freshness risk for AVIF specifically; (D3) confirm the two patch-level lock movements (`itoa`, `syn`) are acceptable in a future order. No licence approval is required for these four roots; the inherited `Unicode-3.0` attribution duty remains an open release prerequisite.

## 11. Preservation, cleanup and handoff

One checkout, one worktree, one `origin`; `main == origin/main`, empty `git status --porcelain --untracked-files=all`; branches exactly `main` and the preserved divergent `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6`; WIP exactly four files; ignored `target/` and its build cache untouched; inherited Solo `.temp/` archives preserved at **2 files / 289,253,806 bytes**. Task-owned scratch was exactly one probe directory (3 files, 6,034 bytes: `Cargo.toml`, `Cargo.lock`, `src/lib.rs`), **deleted** after measurement, leaving no `wo068` material anywhere. No `git clean`, reset, force, amend, rebase, cherry-pick, PR, release, deployment or provider action occurred, and no file outside the two writable paths was modified.

**Handoff:** `SOLO-WO-068 R01 — COMPLETE` for the bounded research task. The four formats are **not** qualified: PNG, JPEG and WebP are statically promising, AVIF is blocked on provenance plus runtime/bound proof; `R03.C3`, `R03.C` and Stage 03 stay **OPEN**, and no codec, dependency, licence, policy or workflow decision has been made on the Owner's behalf. The Chief Architect should audit this report's exact remote HEAD and return `ACCEPT`, `REVISE` or `STOP`; a code-bearing order may only be prepared after that acceptance and fresh Owner authorization.
