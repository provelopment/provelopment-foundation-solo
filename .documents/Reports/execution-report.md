# Execution Report

**Work order:** `SOLO-WO-118 R02` — complete AVIF V4 from verified T9; independent pixel oracles and two-host N118 proof.
**Date:** 2026-11-15. **Required outcome:** `R03C-AVIF-V4-8-10-12-ALPHA-ORIENTATION-N118-TWO-HOST-VERIFIED`.

**status=STOPPED — the V4 capability, N118 proof and oracle pins are not delivered; no self-acceptance is claimed.**

This run completed and verified the dependency-graph correction (§3A) and re-proved the real decoder on the current branch, but **did not** deliver the V4 application port, the confined AVIF policy/worker wiring, the seven `n118_*` host tests, the N118 workflow gate or the pinned independent oracle. The technical branch is therefore **green on every retained gate** yet **short of the ordered capability**. I did not ship speculative AVIF policy code, because the refusal semantics it must enforce (essential/unsupported transforms, AVIS and grid refusal) have to be *verified* against `oxideav-avif` 0.0.16 before they become a gate; guessing them would manufacture a false pass.

## 1. Commits (all forward, all pushed)

| Step | Commit | Tree | Parent |
|---|---|---|---|
| baseline R9 (`main`) | `15d25381c26b81d808386f34f0bfcdffca00c8a2` | — | `d7319fdf` (A9) |
| **A10** (`main`) | `61cf7dbc22ae696d2d7d4a53f6d88d3049e6c727` | `558361ce226981ff93c10b486f5df2f806301bb3` | `15d25381` |
| **T10.0** (branch, ACTIVE propagation) | `ea92e04c8c2061603f17242bd21b73109dc15578` | — | `3d52df20` (T9, preserved) |
| **T10A** (branch, §3A graph correction) | `68eda9337d66737e07494d95418017047b16be57` | `a884ea5fc20ac5f01950a73480a34d44d2e09757` | `ea92e04c` |
| **R10** | given in the chat handoff only | — | `61cf7dbc` |

A10 committed **only** `.documents/WIP/workorder-active.md`; blob `b03528783d3ae3fbc9887b1c5ea85b08a3942f51` is identical in the working tree, index, commit and `origin/main`. The branch received that same blob by exact single-path `git restore --source=<A10>` (T10.0), verified equal to `A10:.documents/WIP/workorder-active.md`; the older R01 ACTIVE was never imported. T9 `3d52df20` remains the recorded checkpoint ancestor, and the previously verified decoder work is intact.

## 2. §3A is complete and independently measured

- `APPROVED_ADAPTER_EXTERNAL_NORMAL`: **9 → 10**, adding exactly `"oxideav-avif"` in sorted position.
- `APPROVED_ADAPTER_CLOSURE`: **47 → 53**, adding exactly `oxideav-av1`, `oxideav-avif`, `oxideav-core`, `oxideav-heif`, `thiserror`, `thiserror-impl` — the root plus the five transitives, which is the correct reading (§3A's own note that R01 mis-said "four" is honoured).
- `cargo test -p solo-cli --test check-architecture --locked` → **rc 0**, real-tree measurement **`edges=93`** (previously `81`; the +12 come from the six new packages' resolve edges).
- `cargo test -p solo-cli --test workspace_graph --locked` → **32 passed, 0 failed** — the fail-closed guard accepts the new root precisely because the single reviewed allowlist changed, and refuses nothing it should accept.
- Both retained workflow assertions replaced with the **exact measured** `edges=93` (bash fixed-string check at `rust-verify.yml:965`, PowerShell `-notmatch 'edges=93'` at `:977`). No wildcard, no removal.

## 3. Decoder re-verified in this run (not quoted from R01)

`cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture` on T10A → **rc 0, `1 passed`**, decoding all seven immutable fixtures from actual bytes:

| Fixture | Dimensions | Depth | Layout | Planes |
|---|---|---|---|---|
| `white_1x1.avif` | 1×1 | 8 | `Yuv444P` | 3 |
| `fox-profile0.8bpc.yuv420.avif` | 1204×800 | 8 | `Yuv420P` | 3 |
| `fox-profile0.10bpc.yuv420.avif` | 1204×800 | 10 | `Yuv420P10Le` | 3 |
| `fox-profile2.12bpc.yuv420.avif` | 1204×800 | 12 | `Yuv420P12Le` | 3 |
| `abc_color_irot_alpha_irot.avif` | 256×512 | 8 | `Yuva444P` | 4 |
| `abc_color_irot_alpha_NOirot.avif` | 256×512 | 8 | `Yuva444P` | 4 |
| `clap_irot_imir_non_essential.avif` | 10×8 | 10 | `Yuva444P10Le` | 4 |

The typed container surface needed for a pre-decode policy exists in the pinned crate: `oxideav_avif::inspect` exposes `AvifInfo { width, height, bits_per_channel, pasp, av1c, obu_bytes, is_grid, has_alpha, brands, colour, mdcv, clli, cclv }` plus `max_bit_depth()`, `num_channels()`, `is_monochrome()` — container-only, so hostile header policy can run inside the child before any pixel allocation.

## 4. What is not delivered (the STOP)

1. **§3B/§3D V4 capability** — `graphic_avif_inspection.rs` (bounded policy + decode + plane digests), `GraphicImageInspectPortV4` / `inspect_graphic_image_source_set_v4`, the V4 adapter arm replacing the deliberate `format.is_avif()` → `AvifUnsupported` refusal *only* in V4 while V3 stays truthful, and the versioned V4 envelope/observation with parent-side re-verification of received bytes.
2. **§5 N118** — the seven exact `n118_0*` host tests, the seven-mutation checker, the deliberate RED/GREEN oracle step, and the Ubuntu/Windows N118 workflow gates.
3. **§4 oracle** — no independent `avifdec`/`dav1d` values were produced, so `LICENSE-AND-PROVENANCE.md` carries no new pinned plane digests. The method is validated and ready: the Ubuntu runner installs `libavif-bin` in a logged test-only step, pins package version, binary SHA-256, `ldd` shared-library identities, extracts 8/10/12-bit Y4M planes parsed **outside** the candidate, and recovers display orientation from PNG output or an audited `irot`/`imir` coordinate mapping (Y4M alone must not be used for orientation, and alpha must follow the same geometry).
4. **Two-host technical CI** — never dispatched at `68eda933`; no V4 result exists on either host. R9's `43680718664` and any R10 baseline remain baseline evidence only.

**Local gate evidence actually run (WSL Ubuntu 26.04, this run):** `cargo fmt --all -- --check` **0**; `cargo check --workspace --all-targets --locked` **0**; `cargo clippy --workspace --all-targets --locked -- -D warnings` **0**; `check-architecture` **0** (`edges=93`); `workspace_graph` **32/32**; `check_architecture_fixtures` **35/35**; `solo-adapters` blocked only by the five known `source_read_native::snapshot_proofs` cases, reported **NOT RUN LOCALLY** — this host still has no `SOLO_TEST_SOURCE_FS_ROOT` Btrfs filesystem, and those cases are neither skipped nor faked.

**Requested Owner decision:** issue the continuation that implements §3B–§3D and §5 on top of the green `68eda933` base, with the oracle step run first on the Ubuntu runner so the recovered plane digests can be pinned before the N118 gates are made load-bearing. Everything the next run needs is now unblocked: the dependency is admitted, the graph is exact at `edges=93`, and the decoder plus its typed inspection API are proven on real bytes.

## 5. State and boundaries

`main` is `61cf7dbc` (A10) plus this report only. Remote heads are **28**: the original 27 plus `solo-wo-118-avif-v4-r01` at `68eda933`; `solo-wo-115-avif-svg-v3-r02` remains `d82ae762`, and `ad7b74df` / `40b03c73` are unchanged. One worktree, empty stash, exactly four WIP files, no report archive, no second checkout, no PR, merge, tag, release, deployment or provider action. The seven pinned `.avif` fixtures stay test-only and unpublished; no `main.rs`, native/source-read/unsafe checker, `workspace_graph.rs`, `deny.toml`, root manifest, SOP, Architecture or roadmap file was touched. Stage 03, `R03.C` and `R03.C3` remain **OPEN**, and this inspection-only work grants no publication authority and no public route.

## 6. R10 baseline CI result (reported as found)

The fresh two-host baseline at R10 was dispatched as run `43683197657` (attempt 1). **Windows `verify (windows-2025)` succeeded; Ubuntu `verify (ubuntu-24.04)` failed** in the inherited step *Test (workspace, all targets, locked)* on one pre-existing host test:

```text
test n093_01_the_real_parent_terminates_its_owned_group_and_observes_the_descendant ... FAILED
panicked at crates/solo-cli/tests/media_worker_host_smoke.rs:1553
test result: FAILED. 50 passed; 1 failed; 0 ignored
```

That test is untouched by this order (R10 changes only this report; `main` differs from R9 solely by documentation), and the same tree passed on R9 (`43680718664`). I therefore record it as an **inherited, environment-sensitive N093 result** rather than a defect I introduce or a pass, and I have not re-run it into green by filtering, skipping or weakening it. R10 is consequently **not** a verified two-host baseline, which is a second, independent reason this report cannot claim completion.

**Status:** `status=STOPPED`, pending independent Chief Master Architect review.


