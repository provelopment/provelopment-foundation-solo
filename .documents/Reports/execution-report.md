# Execution Report

**Work order:** `SOLO-WO-113 R01` — complete atomic mixed-format GIF image inspection and deliberate-red sensitivity.
**Date:** 2026-11-14. **Approved outcome sought:** `R03C-GIF-MIXED-V2-WHOLE-OWNER-AND-RED-SENSITIVITY-VERIFIED`.
**status=STOPPED — authority/scope conflict: the required real GIF obligation cannot be declared under the frozen domain registry, and the closed write ledger authorizes no domain change.**

No merge, release, deployment, tag or Stage closure occurred. No GIF or N102 source is integrated into `main`. Nothing in this run claims `ACCEPT`.

## 1. Why this order stopped

`SOLO-WO-113 R01` requires, as case 1 of §4 and as ACCEPT criteria 3 and 4, a **real whole-Entity mixed PNG/JPEG/WebP/GIF** candidate inspected through "the true confined source-read and real worker path", with an Entity whose distinct declaring owners "supply a real PNG, JPEG, WebP and GIF". That is impossible with the frozen domain:

* `crates/solo-domain/src/property_registry.rs:623` declares
  `pub const ACCEPTED_IMAGE_EXTENSIONS: [&str; 5] = ["png", "jpg", "jpeg", "webp", "avif"];`
* `AssetReferenceV1::parse` (same file, line 643) rejects every other extension at line 688 with `PropertyRefusalKind::UnsupportedAssetExtension`.

Observed, not inferred — the probe is a real executed test assertion at `crates/solo-application/src/graphic_image_inspection.rs` (`n113_01_the_v2_format_enum_stays_closed_and_extension_driven`), which failed exactly as follows before the blocked assertion was removed from the preserved draft:

```text
thread 'graphic_image_inspection::v2_tests::n113_01_the_v2_format_enum_stays_closed_and_extension_driven' panicked at crates/solo-application/src/graphic_image_inspection.rs:
assertion `left == right` failed
  left: None
 right: Some(Gif)
```

Direct equivalent: `AssetReferenceV1::parse(AssetNamespace::Entity, "assets/a.gif")` returns `PropertyDeclarationRefusalV1 { kind: UnsupportedAssetExtension }`, so `PropertyValueV1::graphic(...)`, `AuthoredPropertyValueV1::new(...)`, `collect_graphic_asset_obligations_v1(...)` and therefore `test_inventory_v1(&[(Page, "logo", "assets/x.gif", "public")])` can never produce a GIF obligation — it returns `None`. Consequently:

* no `GraphicAssetSourceSetV1` can contain a `.gif` obligation, so the V2 policy's GIF branch and the real confined GIF worker are unreachable through the real source-read → inspection composition;
* the mandated mixed PNG/JPEG/WebP/**GIF** whole-owner evidence, the real deliberate-red RED/BLUE GIF sensitivity, and the required V2 two-host CI gate cannot be produced at all;
* the only alternative would be to fabricate an inventory or a source set, which §4 and §122 explicitly forbid ("never bypass the real source/read/probe gate by constructing a fake *host* result and calling it end-to-end").

**The registry file is outside the closed ledger.** §2 authorizes exactly seven paths (`.documents/WIP/workorder-active.md`, `.documents/Reports/execution-report.md`, `.github/workflows/rust-verify.yml`, `crates/solo-application/src/graphic_image_inspection.rs`, `crates/solo-application/src/lib.rs`, `crates/solo-adapters/src/graphic_image_inspection.rs`, `crates/solo-adapters/src/lib.rs`, plus `crates/solo-cli/tests/media_worker_host_smoke.rs` and `crates/solo-cli/tests/architecture_checks/graph.rs`); `crates/solo-domain/src/property_registry.rs` is not among them, and §49 forbids any other permanent source change. Editing it would be an unauthorized permanent write, which §124 classifies as a STOP, not a judgement call for the Implementation Agent.

**Smallest Owner decision required:** authorize the minimal domain change that adds `gif` to `ACCEPTED_IMAGE_EXTENSIONS` (the `graphic.favicon` PNG-only rule at line 1407 uses `is_png()` and stays exactly as accepted), and add that file to the ledger — or amend the order to drop the real whole-owner GIF obligation. No other product, security or dependency decision is needed to continue.

## 2. What was completed and verified

| Step | Commit | Tree | Parent | State |
|---|---|---|---|---|
| A authority (ACTIVE only) | `e698ad808925872aef31d42246dffcb0930a059d` | `6f087db778261d4ae3e4d737a09a5ef8d3a80ce2` | `24425032` | committed, pushed to `origin/main` |
| B0 ten-blob import | `ee295aea74ac39d3416f61911a5fff1d0dfb534b` | `6da3debb9b6f5cd0fb4b8e69a188b9d802b9d646` | `e698ad80` | committed, on the branch |
| C1-DRAFT (not an accepted checkpoint) | `79c4dccf62c4428266cb7dc7482a5eb431f0b04d` | `1be86f4335a2188406bb5e7c045f129e24c186a3` | `ee295aea` | committed, on the branch |

Branch `solo-wo-113-gif-v2-r01` is pushed at `79c4dccf`; the 25 pre-existing refs are unchanged, giving 26 refs. ACTIVE was installed as blob `f5604df33905aa887d59a54e07ef387f923176ba`, byte-identical to the Owner-installed worktree file. `main` remains at A; this report is the only other main commit.

Preflight was fully verified before acting: `HEAD == main == origin/main == == 24425032fcf1a681006d6d51576ef4ff24231132`, tree `a280bfe8`, report blob `ce8540b1`, old ACTIVE blob `b677df64` before replacement, WO-112 branch `solo-wo-112-gif8-host-r01 == 40b03c73` locally and remotely, 25 refs, one worktree, empty stash, one `origin`, exactly four WIP files, and every frozen governance blob (`AGENTS.md` `6eae1da8`, Architecture `f66096ec`, both SOPs, architecture checker `1c99d348`, unsafe checker `57d75796`, root `Cargo.toml` `14505be4`, toolchain `9ec55b3b`, ROADMAP `9d7a8622`, NEXT `92fa562b`, OUTLOOK `cb9c7a61`).

## 3. B0 — exact ten-blob import, zero mismatches

Restored by named-file `git restore --source=40b03c73…`, never merged or cherry-picked. For every path the source `git rev-parse 40b03c73:<path>`, the filtered worktree `git hash-object --path=<path>` and the committed `git rev-parse B0:<path>` were compared with the ordered list: **10/10 source, 10/10 worktree, 10/10 committed blobs, zero mismatches.**

| Path | Blob |
|---|---|
| `.github/workflows/rust-verify.yml` | `b120e0227a1a7e69914b2a68245c802073b9a8a4` |
| `Cargo.lock` | `cf1051675576ab0e00ce7acbd5dbe139882c31b4` |
| `crates/solo-adapters/Cargo.toml` | `3610f06ff58cdea6ce1b3993544f0c2e286087ea` |
| `crates/solo-adapters/src/graphic_gif_inspection.rs` | `785d08d0e5a9009be1cbb015eb1b88362f6ab311` |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | `8932724113c1a27941e080fea7037fb24a4e2e74` |
| `crates/solo-adapters/src/lib.rs` | `08b832372196a3f6a97316da116b7af1d71dd35b` |
| `crates/solo-adapters/src/media_worker_host.rs` | `894bc6220d90161ce98bb9c0c848ff208388f666` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `9a26e195b295974b54f7e2da7ce7ae8fbc510c4a` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `dc6d2770038a14199d2f366faab64db4e3d6e98b` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `b3d650140ec7a32e5230a03d0178332222517fcd` |

After B0 the branch carries the accepted WO-112 technical source verbatim: the eight N108 canaries, the corrected N102 event-identity validation and the frozen Windows unsafe/handle/Job-Object boundary.

## 4. C1 draft — real, green, and explicitly not an accepted checkpoint

Because the ledger *does* authorize the application and adapter files, the inward V2 work was completed and preserved on the branch so an Owner decision can resume without rework. It is committed as `79c4dccf` under a message that states it is not an accepted checkpoint, and it contains **no failing or ignored test**.

What it implements, entirely within the authorized application/adapter files:

* `GraphicImageFormatV2` — closed `Png | Jpeg | WebP | Gif`, additive and separate from the untouched three-variant `GraphicImageFormatV1`, with canonical MIME, accepted lowercase extensions, and `as_still_v1()` as the only bridge (GIF never becomes a still).
* Closed inward GIF evidence: `GifFrameEvidenceV2` (index, authored rect, delay, disposal, transparency, interlace, local palette, composite digest) and `GifAnimationEvidenceV2` (signature, `image/gif`, canvas, frames, `Absent|Finite(n)|Infinite`, cycle ms, composed-pixel budget, source digest, transcript length/digest, PID, limits, cleanup, wall). `GifLoopV2` and `GifDisposalV2` are inward-owned; no adapter type or decoder handle crosses the boundary.
* `GraphicImageInspectRequestV2` (digest computed inward from the carried bytes), `GraphicImageObservationV2::{Still,Animation}`, `GraphicImageInspectPortOutcomeV2` and the one-method `GraphicImageInspectPortV2`.
* `GraphicImageInspectRefusalKindV2` — 16 unique typed tokens including `favicon-not-png`, `gif-frame-count-out-of-range`, `gif-frame-geometry-out-of-range` and `gif-timeline-inconsistent`; `GraphicImageInspectRefusalV2` attributes owner, role and authored local index only.
* Result records `GraphicImageInspectionV2` / `GraphicImageInspectionSetV2` / `GraphicImageInspectionOutcomeV2` with `grants_publication_authority() == false` and `proves_whole_publication() == false`.
* `inspect_graphic_image_source_set_v2` — atomic whole-candidate policy over the real `GraphicAssetSourceSetV1`: format, `role_admits_format_v2` (favicon PNG-only, read from the registered `PropertyKey`), empty/oversize source, port attempt, then every claim, with `validate_still_observation_v2` and `validate_gif_evidence_v2` doing the checks production uses (format, MIME, source digest, child identity/limits/reap, 1..=8192 sides, intrinsic 32 M pixels, GIF frame order/rect/delay-sum/cycle/composed budget/transcript bounds, and an all-zero digest refused as impossible).
* Adapter: `ConfinedGraphicInspectAdapterV2` routes still formats to the unchanged `ConfinedImageInspectAdapterV1` and GIF to `inspect_gif_bytes_with_executable_v1`; `inspect_entity_graphic_images_v2` composes the real confined source read with the V2 policy and returns `EntityImageOutcomeV2`, keeping source-read and inspection refusals distinguishable.
* The required closed GIF sensitivity seam was factored exactly as §4 prefers: one private `inspect_gif_bytes_in_mode_v1(executable, bytes, mode)` used by production (always `Decode`) and by the new `inspect_gif_bytes_with_test_mode_v1` (`WrongSourceDigest` / `WrongTranscriptDigest` only), both through the identical parent validator.

Local evidence at `79c4dccf` (WSL Ubuntu 26.04.1, same mounted checkout):

| Command | Outcome |
|---|---|
| `cargo fmt --all -- --check` | clean |
| `cargo check --workspace --all-targets --locked` | clean |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | clean |
| `cargo test -p solo-application --lib --locked` | **15 passed**, including 5 `n113_` V2 tests |
| `cargo test -p solo-adapters --lib --locked graphic_gif` | 14 passed |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n108_` | 8 passed; 0 failed; 0 ignored (unchanged) |
| `cargo test -p solo-cli --test unsafe_boundary --locked` | 22 passed |
| `cargo test -p solo-cli --test check-architecture --locked` | `architecture=legal packages=4 targets=33 files=63 edges=70` |
| `cargo test -p solo-cli --test check_architecture_fixtures --locked` | 35 passed |

The five V2 tests drive the **real** inward validator with fabricated observations (never a shadow implementation): falsified still format, MIME, source digest, unproved limits, unproved reap, zero PID, zero and 8193 sides, 8192×8192 pixel product, wrong decoded length and an all-zero decoded digest; falsified GIF signature, MIME, source digest, unproved limits, zero PID, zero side, 8192×8192 intrinsic product, zero frames, out-of-order indices, rectangle outside canvas, zero frame side, wrong cycle, over-limit cycle, wrong composed budget, `Finite(0)`, zero transcript and an all-zero composite digest; plus closed-enum, favicon-PNG-only, unique-token and no-publication-authority assertions.

## 5. What was deliberately not done

* **No C2 matrix, no deliberate-red capture, no D gate, no technical CI dispatch.** Each of those depends on a real GIF obligation that cannot exist, so producing them would require a fabricated inventory — explicitly forbidden.
* **No `solo-domain` edit.** The registry file is outside the closed ledger; changing it without authorization would be an unauthorized permanent write.
* **No GIF/N102 integration into `main`**, no route, renderer, publication, release, tag, deployment or Stage closure.
* No dependency, feature, toolchain, workflow-gate, `deny.toml`, Architecture, SOP, WIP or Owner-data change. `Cargo.lock` and `crates/solo-adapters/Cargo.toml` are the B0 import blobs, unmodified.

## 6. Preservation, cleanup and nonclaims

`main` is A plus this report only; `main` still carries only the PNG/JPEG/WebP V1 product code and no GIF source, no GIF test and no V2 gate, exactly as WO-112 left it. Branch `solo-wo-113-gif-v2-r01` at `79c4dccf` holds B0 and the labelled draft, unmerged. All 25 earlier refs keep their exact SHAs; ACTIVE remains the Owner-installed `f5604df3`; the four WIP files, both SOPs, `AGENTS.md`, Architecture, both checkers, root manifests, `deny.toml`, `.gitignore`, provider/release/deploy definitions and all Owner-owned or ignored material are untouched. Task scratch was confined to the Cline temporary workspace and removed; no `git clean` and no deletion of unknown material was performed.

**Nonclaims:** no product GIF acceptance or integration, no mixed-format V2 verification, no deliberately forged-digest sensitivity proof, no safe true-vector SVG, no AVIF codec proof, no `platform/assets/` catalogue, no derived outputs, no route or publication authority, no release. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. The Implementation Agent does not declare `ACCEPT`.

## 6A. Fresh main CI at the report-only commit

Run `43599132318` at the first report-only commit:

* **attempt 1** — `verify (ubuntu-24.04)` success, `verify (windows-2025)` **failure** in `Windows N102 isolation canaries (Windows only)`: `n102::n102_01_extra_inheritable_handle_is_excluded` panicked with `the non-inheritable control resolved in the child, so a numeric match is ambiguous reuse`, so the step threw `the N102 canaries failed (exit 101)` after `3 passed; 1 failed`.
* **attempt 2** — same SHA, no byte change: **both jobs success**.

That failure is the **known main-only N102 numeric-canary flake**, not a regression from this run: `main` deliberately does not carry the corrected N102 event-identity validation, which exists only on the preserved technical branches (`solo-wo-111-gif4-n102-r01` `cdf0ceeb`, `solo-wo-112-gif8-host-r01` `40b03c73`), and the report-only commit changes no product, test or workflow byte — `main` differs from A by `.documents/Reports/execution-report.md` alone. It is disclosed here rather than concealed, and it is not a licence to integrate the N102 fix into `main` under this order. The fresh both-host run at the governing report commit itself is given in chat with the exact SHA, because a report can never embed its own commit.



## 7. Exact next Owner decision

Either:

1. **Authorize the minimal domain change** — add `"gif"` to `ACCEPTED_IMAGE_EXTENSIONS` in `crates/solo-domain/src/property_registry.rs` (line 623), optionally extend the constant to `[&str; 6]`, retain the existing `graphic.favicon` PNG-only rule unchanged, and add that file to the WO-113 ledger — after which the draft on `solo-wo-113-gif-v2-r01` continues directly into C2 (whole-owner mixed matrix, negative matrix, deliberate-red capture) and D (two-host gate).

2. **Or amend the order's scope** to state explicitly that this V2 gate is policy/adapter-level only and that no real GIF obligation is required, accepting that ACCEPT criteria 3 and 4 cannot be satisfied as written.

No other blocker exists; the draft is green, the import is exact, and the pre-existing N108/N102/N097 evidence is preserved.



