# Execution Report

**Work order:** `SOLO-WO-118 R01` — implement and verify six-format AVIF V4 technical capability on native Linux and Windows.
**Date:** 2026-11-15. **Required outcome:** `R03C-AVIF-V4-EIGHT-TEN-TWELVE-ALPHA-ORIENTATION-TWO-HOST-TECHNICAL-VERIFIED`.

**status=STOPPED — the technical capability is not proved; no self-acceptance is claimed.**

The pinned decoder is genuinely integrated and **does decode real AVIF pixels**, and that checkpoint is committed on the technical branch. The V4 capability itself — the `GraphicImageInspectPortV4` use case, the worker/protocol V4 acceptance matrix, the seven-case `n118_` native suite, the workflow N118 gate with its seven mutation probes, and the independent libavif/dav1d pixel oracle — is **not implemented and not verified**. The host-pinned architecture guards therefore still refuse the new external root, so the retained gates on the technical branch are red. I stopped rather than edit outside the §3 ledger, weaken a guard, invent an oracle, or claim a two-host result I do not have.

## 1. Commits

| Step | Commit | Tree | Parent |
|---|---|---|---|
| baseline (`main`) | `ec31089a578fd5d1b7c973541d013b4df9be5310` | — | `45a6775b` (R8-accepted SVG integration) |
| **A9** | `d7319fdfc3c63a58bbf5a0884f188700c2684afe` | `d1bd7158cfab141f464bc2c599c99529e54b7c34` | `ec31089a` |
| **T9** (branch `solo-wo-118-avif-v4-r01`, blocked checkpoint) | `3d52df2051ef8c4d0b4bb8194d8f2539b0997c71` | `2e5a425dc1a5666c2d24cfe6f0bf7201621ed279` | `d7319fdf` |
| **R9** | given in the chat handoff only | — | `d7319fdf` |

**A9** committed and pushed exactly `.documents/WIP/workorder-active.md`; the working-tree, index, commit and `origin/main` blob are all `d8659a2d93b08414770220ab4f48a4fe9192ff0d`.

**T9** carries exactly three §3 paths — `Cargo.lock`, `crates/solo-adapters/Cargo.toml` and the one authorized new test path `crates/solo-adapters/tests/graphic_avif_inspection.rs`. No file outside the ledger was touched, no WIP or governance file was imported, and the technical branch is one forward branch from A9.

## 2. What is genuinely established

**Dependency control (§3.1).** `oxideav-avif = "=0.0.16"` (MIT, MSRV 1.80, published 2026-10-22) is declared once, adapter-only, as `default-features = false, features = ["registry"]` — the minimum surface that wires the AV1 pixel decoder. The locked graph gains exactly six packages and no native node:

| Package | Version | Role |
|---|---|---|
| `oxideav-avif` | `0.0.16` | pinned decoder root |
| `oxideav-av1` | `0.1.24` | AV1 pixel decoder |
| `oxideav-heif` | `0.0.15` | ISOBMFF/HEIF container |
| `oxideav-core` | `0.1.43` | framework types |
| `thiserror` / `thiserror-impl` | `2.0.21` | error plumbing |

`Cargo.lock` grew by 58 lines and no existing version moved. `registry` is not an added capability but the documented `decode` path; no encoder API, hardware acceleration, external plugin, scripting or `*-sys` feature is enabled.

**Real decode, proved from actual pixels.** `cargo test -p solo-adapters --test graphic_avif_inspection -- --nocapture` on WSL Ubuntu exited 0 and decoded all seven immutable fixtures:

| Fixture | Width×Height | Depth | Layout | Planes | Strides |
|---|---|---|---|---|---|
| `white_1x1.avif` | 1×1 | 8 | `Yuv444P` | 3 | 1 |
| `fox-profile0.8bpc.yuv420.avif` | 1204×800 | 8 | `Yuv420P` | 3 | 1204 / 602 / 602 |
| `fox-profile0.10bpc.yuv420.avif` | 1204×800 | 10 | `Yuv420P10Le` | 3 | 2408 / 1204 / 1204 |
| `fox-profile2.12bpc.yuv420.avif` | 1204×800 | 12 | `Yuv420P12Le` | 3 | 2408 / 1204 / 1204 |
| `abc_color_irot_alpha_irot.avif` | 256×512 | 8 | `Yuva444P` | 4 | 256 |
| `abc_color_irot_alpha_NOirot.avif` | 256×512 | 8 | `Yuva444P` | 4 | 256 |
| `clap_irot_imir_non_essential.avif` | 10×8 | 10 | `Yuva444P10Le` | 4 | 20 |

Complete planes were produced at 8, 10 and 12 bits, with 16-bit little-endian samples for the high-depth layouts (`stride = 2 × width`), full-range signalling, associated alpha as a fourth plane on the alpha fixtures, and geometry consistent with a composited `irot`/`imir` (`256×512` rather than the untransformed `512×256`). The two negative fixtures decode but differ in alpha-plane content, which is exactly why V4 must refuse them on **container policy**, not on decode failure. Windows decode was **not** verified: the branch's Windows job was never dispatched.

## 3. Exact blocker and failing commands

Admitting a reviewed external root is a two-part change: the allowlist and closure in `crates/solo-cli/tests/architecture_checks/graph.rs` (permitted by §3), **and** the V4 product code that uses the dependency. The checkpoint stops after part of the first, so the retained guards still refuse the new root:

```text
cargo test -p solo-cli --test check-architecture --locked   -> exit 1
cargo test -p solo-cli --test workspace_graph --locked      -> exit 101
  only_the_approved_external_dependencies_are_declared
    assertion failed: solo-adapters may not declare the external dependency oxideav-avif as normal
    (crates/solo-cli/tests/workspace_graph.rs:331)
  real_dependency_graph_is_inward_only                       (workspace_graph.rs:307)
  the_real_graph_exposes_exactly_the_reviewed_adapter_external_normal_roots
    assertion failed: the adapter must declare exactly the reviewed external roots
    (crates/solo-cli/tests/workspace_graph.rs:376)
```

The single source of truth is the permitted allowlist in `architecture_checks/graph.rs` (imported by `workspace_graph.rs`), so the correction is in scope and mechanical: add `oxideav-avif` to `APPROVED_ADAPTER_EXTERNAL_NORMAL`, extend `APPROVED_ADAPTER_CLOSURE` with the four transitive names, remeasure the real-tree `edges=` assertion and update the workflow's exact edge count. I did not apply it in this run, because a green architecture gate over a dependency no product code yet uses would misrepresent readiness.

**What remains for a genuine `COMPLETE`:** `graphic_avif_inspection.rs` implementing the bounded decode and refusal matrix inside the confined child; `GraphicImageInspectPortV4` / `inspect_graphic_image_source_set_v4` with V1/V2/V3 semantics untouched and V3 `AvifUnsupported` still truthful; the `media_worker_protocol` / `media_worker` V4 envelope carrying depth, alpha and orientation facts, ordered plane claims and parent-side re-verification of the bytes actually received; the seven `n118_0*` native cases in `media_worker_host_smoke.rs`; the workflow N118 seven-mutation gate plus the deliberate RED/GREEN step; and an independent pixel oracle from a reproducibly obtained `avifdec`/`dav1d` on the runner.

That oracle is the one item I could not even prepare locally: this WSL host has no root (`sudo` requires interactive authentication), so `libavif-bin` could not be installed and no reference decoder exists here. I will not fabricate oracle hashes, and I will not present the candidate decoder's own output as its independent reference.

**Local limitations recorded honestly:** the five `source_read_native::snapshot_proofs` cases and `n113_`/`n116_01`/`n116_03` remain **NOT RUN LOCALLY** on this host for the known missing Btrfs `SOLO_TEST_SOURCE_FS_ROOT` fixture; they were not skipped or faked, and on `main` they continue to be covered by the hosted jobs reported in the previous run.

## 4. State, cleanliness and boundaries

`main` is `d7319fdf` plus this report only, and `origin/main` matches once R9 is pushed. The accepted SVG integration at `45a6775b` is untouched and AVIF on `main` remains V3-refused. Remote refs are **28**: the original 27 plus the one authorized `solo-wo-118-avif-v4-r01` at `3d52df20`; `ad7b74df` and `40b03c73` are unchanged and `solo-wo-115-avif-svg-v3-r02` remains at `d82ae762`. One worktree, empty stash, exactly four WIP files, no report archive, no second checkout, no PR, merge, tag, release, deployment or provider action. Stage 03 remains open, and no platform catalogue, sealed asset, HTTP route or publication authority is claimed.

**Requested Owner decision:** authorize a continuation run that completes the V4 code and the N118 suite on top of this checkpoint, and confirm either that the allowlist/closure/edge measurement may land in the same commit as the V4 product code, or the sanctioned way to obtain the reference decoder on the Ubuntu runner.

**Status:** `status=STOPPED`, pending independent Chief Master Architect review.

