# Execution Report

**Work order:** `SOLO-WO-118 R03` — pin independent AVIF pixel oracles; characterize the retained N093 refusal.
**Date:** 2026-11-15. **Required outcome:** `R03C-AVIF-INDEPENDENT-REFERENCE-ORACLE-PINNED-N093-TRUTHFULLY-CHARACTERIZED`.

**status=COMPLETE for the R03 bounded scope.** Independent reference facts now exist, are pinned in a closed-key manifest, and are re-checked by the same load-bearing verifier that rejects three mutated manifests. This is **not** AVIF V4 capability, **not** Stage 03 acceptance and **not** publication authority.

## 1. Commits

| Step | Commit | Tree | Parent |
|---|---|---|---|
| baseline (R10) | `3310b7aaf91ec911dc7a9c7f4e315f46cd94bb44` | — | A10 |
| **A11** (`main`, ACTIVE only) | `4d11e96085bb21b8c6a727435fafabecd1c118a1` | `3284606a53d1cb4b18978f7538447427a706e184` | `3310b7aa` |
| **T11.0** (branch, ACTIVE import) | `b18ccc343abba4f4438c756246dce09a3c8d42c8` | — | `68eda933` |
| **T11.1** parser + discovery job | `fc2d8f1fa809744bdd32e0d2d00ec797a9fd7140` | `7a974670` | `b18ccc34` |
| **T11.2a** accept `420jpeg` | `6f6b042cf978f662149d6903b693805e0639dc42` | — | `fc2d8f1f` |
| **T11.2b** audited `irot` mapping | `be825046f75029236c5cf555cc4d2bfac7182438` | — | `6f6b042c` |
| **T11.2c** pinned goldens + provenance | **`808233516ef280d319f206553915998820cbd9d5`** | `2fb62fe0` | `be825046` |
| **R11** | given in the chat handoff only | — | `4d11e960` |

A11 committed only `.documents/WIP/workorder-active.md` (blob `4bbc841b`, identical in tree, index, commit, `origin/main`). T11.0 imported that exact blob by single-path `git restore --source=<A11>`; the blob matched `A11:` before commit. Preserved byte-for-byte from T10A: `APPROVED_ADAPTER_EXTERNAL_NORMAL[10]`, closure `[53]`, `edges=93`, `oxideav-avif =0.0.16`, the exploratory decode probe. No `Cargo.lock`, Rust source, `Cargo.toml`, `deny.toml`, `media_worker_host.rs`, N093/N116 source or fixture binary was touched.

## 2. The independent reference is real and pinned

`avif-reference-oracle` ran on real `ubuntu-24.04` with `libavif`'s `avifdec` + `dav1d` and Pillow, entirely inside `$RUNNER_TEMP`, with no secrets, no write token and a guaranteed cleanup step:

| Fact | Pinned value |
|---|---|
| `avifdec` | `1.0.4` (dav1d 1.4.1, libgav1 0.18.0, aom v3.8.2, rav1e 0.7.1, svt v1.7.0) |
| `avifdec` binary SHA-256 | `845c02f6194140824189018c87d4fff400655070615466f6610f3904ba4cc096` |
| `libavif-bin` / `libdav1d*` | `1.0.4-1ubuntu3` / `1.4.1-1build1` |
| `python3-pil` / Pillow | `10.2.0-1ubuntu1.5` / `10.2.0` |
| `/usr/bin/python3` | `3.12.3` |

The final run installed by **exact APT version** (`libavif-bin=…`, `python3-pil=…`) read from the manifest, and would have failed on any drift.

Independent **plane** facts (parsed and hashed by `reference_oracle.py`, never the product's own aggregate): `white_1x1` 1×1 `444`/8-bit; fox 1204×800 at `420jpeg`/8-bit, `420p10`, `420p12`, with chroma `602×400`, 8-bit strides `1204/602` and high-depth strides `2408/1204`. All four depths and geometries agree with the candidate decoder's *structure* while being derived from different bytes by different code — the product is not the oracle.

Independent **display** facts for `abc_color_irot_alpha_irot.avif`: alpha `128..255`, `123952` non-opaque samples, coded PNG `512×256` (`png_sha256 20c06c7c…`, coded RGBA `5102863c…`). **libavif did not apply `irot`**, so the recorded display geometry `256×512` is recovered by an explicitly audited mapping — `irot = 1` as a 90° anticlockwise rotation over the full RGBA buffer, giving `rgba_display_sha256 950be879…`. Colour **and** alpha follow the same geometry because the whole buffer is rotated. Disagreement with the tool is preserved and stated, not edited away.


## 3. Hosted result at the single pinned technical SHA `80823351`

Run **`43688309266`** (attempt 1), `head_sha = 808233516ef280d319f206553915998820cbd9d5`, overall **success**:

| Job | Conclusion |
|---|---|
| `avif-reference-oracle` | **success** |
| `verify (ubuntu-24.04)` | **success** |
| `verify (windows-2025)` | **success** |

Oracle evidence tokens from that exact run: `parser_refusals=verified`, `pristine_manifest_matches=true`, `mutation_rejected=source_sha256`, `mutation_rejected=plane_sha256`, `mutation_rejected=rgba_sha256`, `mutation_rejections=3/3`, `oracle_cleanup=complete`. No `discovery_only=true` appears, i.e. the accepted run was **pin-enforced**, not a discovery measurement. The same checker produced the manifest in the earlier discovery run (`43687088743`) and refuses to write it in acceptance mode.

`reference_oracle.py` self-tests **21/21** locally and in CI: hand-stated 4:4:4/8-bit and 4:2:0/10-bit geometry, per-plane digests, the documented aggregate framing, and nine checked refusals — two frames, trailing byte, missing `FRAME`, unknown tag, unknown chroma, zero width, over-bound width, empty input, and a sample above coded depth. Unemitted documented tokens (`420mpeg2`) are still refused (exit 2).

Local Rust gates before each push: `fmt` 0, `check` 0, `clippy -D warnings` 0, `workspace_graph` 32/32, `check_architecture_fixtures` 0, `check-architecture` 0 with `edges=93`, decode probe 0. The five `source_read_native::snapshot_proofs` cases remain **NOT RUN LOCALLY** (no Btrfs `SOLO_TEST_SOURCE_FS_ROOT`); they are neither skipped nor fabricated.

## 4. N093 / N102 truthfully characterized

- **R10's red N093 is not reproduced here.** `verify (ubuntu-24.04)` passed in every branch run of this order (`43685609524`, `43686298451`, `43687088743`, `43688309266`), so `n093_01_…HOST_DESCENDANT_UNVERIFIED` did not recur at these SHAs. It stays recorded as an **OPEN, unexplained host-observation refusal** — not waived, not relabelled a harmless flake, and not a pass. No N093 test, native worker source, timeout, filter or skip was changed.
- A different inherited host test flipped once: run `43685609524` failed `verify (windows-2025)` at the **Windows N102 isolation canaries** on `n102::n102_04_a_concurrent_worker_is_unaffected_by_a_fault` (3 passed, 1 failed); both later runs passed. Recorded, untouched, unsuppressed.
- Because both hosts are green at `80823351`, the R03 scope is complete; the unexplained N093/N102 sensitivity remains a separate corrective item. The next V4 completion still requires two-host `verify` green at one exact SHA.

## 5. Boundaries, cleanup and open items

Technical branch `solo-wo-118-avif-v4-r01` is at `808233516ef280d319f206553915998820cbd9d5`; remote heads remain **28**; one worktree; empty stash; exactly four WIP files; `main` is `4d11e960` plus this report. Permanent writes stayed inside the ledger: `.documents/WIP/workorder-active.md`, `.github/workflows/rust-verify.yml`, `reference_oracle.py`, `reference-goldens.json`, `LICENSE-AND-PROVENANCE.md`. `push.branches` now lists `[main, solo-wo-118-avif-v4-r01]`; every pre-existing gate, both `edges=93` assertions and all N116 checks are unchanged. No PR, merge, tag, release, deployment, provider action or report archive. Task scratch (`.temp/*.sh`, `.temp/manifest.json`, `.temp/getman.ps1`, `__pycache__`) was removed.

**OPEN:** the AVIF V4 child codec, application port, N118 seven-case suite and Stage 03 closure remain unimplemented — this order deliberately forbade attempting them. These pinned facts are the oracle the next V4 order must consume; `R03.C` and `R03.C3` remain OPEN, and nothing here authorizes publication, a public route or a shipped product dependency.

**Status:** `status=COMPLETE` for the R03 bounded scope, pending independent Chief Master Architect review.
