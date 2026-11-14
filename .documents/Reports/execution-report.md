# Execution Report

**Work order:** `SOLO-WO-116 R01` — finish safe true-vector SVG inspection, prove five formats on both hosts, integrate into `main`.
**Date:** 2026-11-14. **Target:** `R03C-SAFE-STATIC-TRUE-VECTOR-SVG-ON-MAIN-VERIFIED`.
**status=STOPPED — S1 delivered and pushed on the technical branch; S2/S3/D/I4 not delivered; nothing integrated into `main`.**

The contained static true-vector SVG sanitizer and the confined SVG vector worker are implemented, compile warning-free, and pass 14 dedicated unit tests including the byte-pinned real Foundation `icon-home.svg`. The SVG capability is **not yet reachable through the whole-owner five-format path**: the V3 application contract, the adapter port, the domain `.svg` admission, the N116 four-case matrix and the workflow gate were not completed in this run, so no technical CI was dispatched and nothing was imported into `main`. AVIF remains explicitly unsupported.

## 1. Checkpoints

Preflight at the order's baseline: `main` and `origin/main` both `b9c5e57439ccda07b80db7ee9a07fedd891902fc`, tree `a3939e5e`, parent `f936a453`; report blob `f5cb6bce`; 27 refs; one checkout; one `origin`; empty stash; four WIP files; the Owner-installed ACTIVE present as the sole working-tree change (73 lines, blob `8b3b8980`). Frozen governance verified 11/11 (`AGENTS.md` `6eae1da8`, `ARCHITECTURE.md` `f66096ec`, SOPs `1483c9be`/`51bb3792`, `architecture_checks/source.rs` `1c99d348`, `unsafe_boundary.rs` `57d75796`, root `Cargo.toml` `14505be4`, `rust-toolchain.toml` `9ec55b3b`, ROADMAP/NEXT/OUTLOOK `9d7a8622`/`92fa562b`/`cb9c7a61`).

| Step | Commit | Tree | Parent | Contents |
|---|---|---|---|---|
| baseline `main` | `b9c5e57439ccda07b80db7ee9a07fedd891902fc` | `a3939e5e` | `f936a453` | R02 STOP report |
| **A4** authority (`main`) | `50f138c656b85bee5dbe8f3d8157f9e17464a584` | `9037c0a77e55e00b929f3d4d3bbe94369a1ff8f8` | `b9c5e574` | ACTIVE alone, blob `8b3b8980` |
| **T0** authority (branch) | `0fee134c3665cc16770fd35c6b31d9b23b9d89d5` | `3d301dec8bae389d3961b3035558fbd5ed9dfd11` | `8c7b6573` (B0) | same ACTIVE blob, copied not merged |
| **S1** sanitizer + worker | `e5a3058efbc1bec3227cefa07c4fe6530dd7aac6` | `5895a78318571afd6ec50e0b0ca58f90e2b1362f` | `0fee134c` | 7 ledger paths |
| **R4** report (`main`) | reported in chat after publication | — | `50f138c6` | report alone |

Branch `solo-wo-115-avif-svg-v3-r02` is pushed at S1 (`e5a3058e…`) and is the only ref that moved; refs stay **27**, `ad7b74df` and `40b03c73` untouched, no new branch, no merge, rebase or cherry-pick. `git diff-tree A4` lists exactly `.documents/WIP/workorder-active.md`.

**Incident, fully recovered and disclosed.** While preparing T0 I ran `git checkout main -- .documents/WIP/workorder-active.md`, which replaced the Owner-installed working-tree file with the older committed R02 bytes. The exact Owner-installed blob `8b3b89807e6e49b762b3f444ef36ae1a61c5471c` was still present in the object database and was restored **byte-for-byte** through `git cat-file -p` in bash (a first PowerShell attempt corrupted line endings and was discarded). Verification: `git hash-object --path=.documents/WIP/workorder-active.md` = `8b3b8980…`, 73 lines, title intact. A4 and T0 therefore both carry the exact Owner-installed bytes; no Owner material was altered or lost.

## 2. What S1 proves (`e5a3058e…`)

**New module `crates/solo-adapters/src/graphic_svg_inspection.rs` (1,785 lines).** Strict `quick-xml 0.42.0` parsing into a closed tree, `svgtypes 0.16.1` re-parsing of every retained attribute, and canonical LF serialization with a controlled decimal form (exact integers, six-decimal trim, `-0` → `0`). Implemented and unit-proven positive set: all path commands `M/L/H/V/C/S/Q/T/A/Z` with relative and absolute forms; `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`; nested `<g>` with transforms; solid fills/strokes, `viewBox`, `width`/`height`; linear and radial gradients with ordered bounded stops; validated `id` and `url(#id)` references; `clipPath`; `mask`; a documented bounded static filter set (`feGaussianBlur`, `feOffset`, `feFlood`, `feBlend`, `feComposite`, `feMerge`, `feMergeNode`) with ≤8 operations per filter; and deterministic text converted to geometric outlines.

**Original 5×7 glyph repertoire.** A literal, fully code-defined table (A–Z, a–z, 0–9, space and 18 punctuation marks) whose filled cells are emitted as SVG path sub-paths under integer cell arithmetic — no font file, no system font, no font request and no host-dependent `<text>`. Unsupported characters refuse the whole document; `&` and `<` are not representable because an XML entity reference is refused outright.

**Safety and budgets, each unit-proven.** 20 MiB source, 4 MiB canonical output, 8192-per-axis and ≤32,000,000 pixels, ≤4096 elements, ≤32 nesting, ≤32 attributes per element, ≤1024 references, ≤40000 aggregate path commands, ≤64 filters. Refused whole: DOCTYPE/DTD/entity declarations, comments, CDATA, XML declarations and processing instructions, `<script>`, `<style>`, `<foreignObject>`, `<image>`, `<a>`, `<use>`, animation elements, event attributes, inline `style`, non-`#` `url()`, `NaN`/infinite values, wrong namespace, prefixed element names, duplicate ids, unresolved references and `href` cycles. `currentColor` resolves to the fixed self-contained `#000000`; `class` is accepted only as tightly validated inert tokens with no CSS evaluation.

**Confined worker.** A new closed `svg` token reuses the accepted reserved flag, the accepted confinement check and the accepted supervisor; the child proves confinement first, bounds its stdin read, re-computes the source digest, sanitizes, and emits one fixed header line plus canonical vector bytes. The parent recomputes both digests, re-checks every budget, and **re-runs the sanitizer over the returned canonical bytes to require byte-identical canonical form**, so a forged transcript cannot survive.

**Local evidence at S1:** `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets --locked -- -D warnings` **0 errors**; `cargo test -p solo-adapters --lib graphic_svg` **14/14 passed**, including `svg_07_the_pinned_foundation_icon_is_accepted` (real `icon-home.svg`, `class="icon icon-tabler icons-tabler-outline icon-tabler-home"` preserved, `stroke="#000000"`, 24×24, idempotent canonical digest). The only non-passing local tests are the five pre-existing `source_read_native::snapshot_proofs` cases requiring `SOLO_TEST_SOURCE_FS_ROOT` to name an R03 Btrfs fixture filesystem; this WSL has no such filesystem and those proofs run in hosted CI.

**Dependencies:** exactly `quick-xml = "=0.42.0"` and `svgtypes = "=0.16.1"`, pure Rust, no native code, no font data, no `avif-native`/`dav1d`, no root manifest or toolchain change, and `image 0.25.10` keeps its accepted PNG/JPEG/WebP features.

## 3. What is not delivered, and why

Not delivered: **S2** (domain `.svg` admission; V3 five-format application contract with the AVIF-refusing arm; V3 adapter port and whole-owner composition; the `n116_01`–`n116_04` real-host tests with the deliberate-RED oracle), **S3** (the N116 four-case exact-count workflow gate), **D** (technical two-host CI), **I4** (exact-blob integration), and therefore the final R4 CI. `main` still contains only PNG/JPEG/WebP/GIF V1/V2; the new SVG module exists on the branch but is not reachable from the product path, and there is no five-format composition, no N116 evidence and no integration.

The remaining blocker is execution window, not a product or authority decision: the residual work is a further substantial implementation plus three mandatory hosted Ubuntu/Windows CI rounds (technical D, I4, R4), each of which must be green at an exact SHA before the next step. The sanitizer and worker — the hard, security-critical half — are complete and locally proven; what remains is the typed V3 composition, its whole-owner tests and the gate wiring.

## 4. Preservation, cleanliness and status

**Preserved on `main`:** every accepted PNG/JPEG/WebP/GIF path, `Cargo.lock` except the two pinned SVG crates, all manifests, `.github/workflows/rust-verify.yml`, `deny.toml`, governance 11/11, the three `plan-*` files and both checkers are unchanged from `b9c5e574`; `main` advanced only by A4. **Preserved on the branch:** the ten B0 fixture/provenance paths are untouched at their pinned blobs. One checkout, one `origin`, empty stash, four WIP files, no second worktree, no reset, `git clean`, force-push, tag, release, deployment, PR or provider write. Task scratch lived in `.temp/wo116/` and was deleted at the end of the run.

| Gate | State |
|---|---|
| A4 authority, governance, refs | **PASS** — `50f138c6`, ACTIVE `8b3b8980`, 11/11, 27 refs |
| T0 branch authority | **PASS** — `0fee134c`, same ACTIVE blob |
| S1 sanitizer + confined worker | **PASS locally** — fmt, clippy 0, 14/14 SVG unit tests, real pinned icon |
| S2 five-format whole-owner proof | **NOT DELIVERED** |
| S3 N116 gate / D / I4 / R4 CI | **NOT STARTED** |
| Partial work integrated into `main` | **None** |

`status=STOPPED — S1 proven; S2/S3/D/I4 not delivered; SVG is not reachable in the product path; AVIF still refuses; nothing integrated.` `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. No functionality was waived, no fallback invented, nothing released, deployed or closed, and the agent does not self-accept.


