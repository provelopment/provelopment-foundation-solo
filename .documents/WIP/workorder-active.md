# SOLO-WO-118 R02 — COMPLETE AVIF V4 FROM VERIFIED T9; INDEPENDENT PIXEL ORACLES AND TWO-HOST N118 PROOF

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Single ACTIVE path:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-V4-8-10-12-ALPHA-ORIENTATION-N118-TWO-HOST-VERIFIED`.

## 1. Execute the correction, not another exploratory investigation

Finish the genuine AVIF V4 six-format media inspection capability on the **existing** `solo-wo-118-avif-v4-r01` technical branch, starting from its proved T9 checkpoint. The previous `SOLO-WO-118 R01` run correctly established that `oxideav-avif =0.0.16` can decode the seven pinned AVIF inputs, including 8/10/12-bit planes and alpha, but **did not** deliver its V4 application port, confined worker path, strict media policy, N118 proof, reference oracle or hosted technical CI. Deliver those now. Preserve all preexisting source/worker/architecture/security guards and the accepted V1/V2/V3 behavior. This order **does not** integrate technical AVIF into `main`, publish an asset, close `R03.C`/Stage 03, create public HTTP, release or deploy.

**Verified remote starting state (15 November 2026):**

- `main == origin/main`: `15d25381c26b81d808386f34f0bfcdffca00c8a2` (**R9**, sole canonical report STOPPED), parent `d7319fdfc3c63a58bbf5a0884f188700c2684afe` (**A9**); the prior accepted SVG integration is `ec31089a578fd5d1b7c973541d013b4df9be5310` (**R8**).
- Sole active AVIF technical branch `solo-wo-118-avif-v4-r01`: `3d52df2051ef8c4d0b4bb8194d8f2539b0997c71` (**T9**), tree `2e5a425dc1a5666c2d24cfe6e0bf7201621ed279`, parent A9. It differs from A9 only in `Cargo.lock`, `crates/solo-adapters/Cargo.toml` and `crates/solo-adapters/tests/graphic_avif_inspection.rs`.
- Current branch has 28 remote heads: original 27 and this one AVIF branch. Keep the accepted SVG technical branch at `d82ae76273b57b37014d3b4c557b8debd5299eea`; frozen `ad7b74df8638ff56c7137f33d9242da54583f17b` and `40b03c73b0bc8544359438daea6218caa496e85d` unchanged.
- R9 baseline CI `43680718664`, attempt 1: native Ubuntu job `132710518438` and Windows job `132710518221` were both successful. **That is a baseline, not V4 proof.**
- Current R01 ACTIVE is Git blob `d8659a2d93b08414770220ab4f48a4fe9192ff0d`. This R02 replaces it as the single executable instruction at the Owner checkout. Never import that older ACTIVE from the technical branch over R02.

Before acting, read the current full `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I, 6.2K, 6A.8, both SOPs, all four WIP files, the **one** `.documents/Reports/execution-report.md`, the complete relevant application/adapter/CI sources and the T9 changes. Preserve exactly one Entity and the EFS five-level ownership contract. Source/unsafe/worker host rules and the A1 security/version/licence policy remain binding. The Chief Master Architect is GitHub **read-only**; all source/Git writes are made solely by the activated Implementation Agent within this order.

## 2. Single-checkout safety; exact three checkpoint phases

Verify the one existing Windows 11 / WSL Ubuntu 26.04 checkout, branch, `origin`, index, tree, stash, four WIP files and all 28 remote refs **before editing**. Use these actual commands in the existing checkout:

```bash
set -euo pipefail
pwd
git rev-parse --show-toplevel
git rev-parse HEAD
git branch --show-current
git remote -v
git status --porcelain=v1
git stash list
git worktree list --porcelain
git ls-remote --heads origin
```

Require one `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo` (optional `.git`), one worktree, no surprise Owner changes, no stash, exactly four WIP files, and exactly the two pinned heads. A safe initial dirty-tree exception is **only** the Owner-installed replacement ACTIVE file on `main`. No clone, extra worktree, branch, archive, unapproved source edits, destructive checkout/reset, force-push or `git clean`; preserve every unfamiliar local item. Task scratch lives only in Solo `.temp/` or the runner-provided Cline temp, then clean **task-owned** scratch safely. Stop and report any different remote state rather than overwrite it.

Execute **in this order**, advancing refs by ordinary forward commits/pushes:

1. **A10 on `main` — ACTIVE authority only.** Confirm starting `HEAD=15d25381...` and that the sole intended working-tree change is this installed `.documents/WIP/workorder-active.md`. Commit/push only that path. Verify its Git blob is identical in the working tree, index, commit, and `origin/main`. Capture full A10 SHA and tree.
2. **T10 technical continuation on the existing `solo-wo-118-avif-v4-r01` branch.** Switch only after A10 is clean and its push verified. Require original branch head exactly `3d52df2051ef8c4d0b4bb8194d8f2539b0997c71`. **First propagate just the issued ACTIVE bytes from A10** into this branch by an exact single-path `git restore --source=<full-A10-SHA> --staged --worktree -- .documents/WIP/workorder-active.md`, after verifying the branch is clean; commit/push the ACTIVE-only branch checkpoint. Verify its blob equals `A10:.documents/WIP/workorder-active.md`. Then complete only §3 source/test/CI paths through forward technical commits, local checks before each technical push and fresh two-host CI at the single final technical SHA. No product files on `main` under this order.
3. **R10 on `main` — canonical report only.** After the technical outcome is fully proved, move back to a verified clean `main`; its expected head is A10. Overwrite/commit/push **only** `.documents/Reports/execution-report.md` with exact current-run evidence. Reverify full native two-host baseline CI at R10 SHA. Provide R10 full SHA in the final chat handoff, never in its own report. On a real blocker publish `status=STOPPED`, exact evidence and next needed decision, without claiming the technical goal.

**Example of branch-safe ACTIVE propagation** (after A10 and branch-tip verification; replace the placeholder with the actual complete A10 SHA):

```bash
A10='<full A10 SHA>'
git switch solo-wo-118-avif-v4-r01
test "$(git rev-parse HEAD)" = 3d52df2051ef8c4d0b4bb8194d8f2539b0997c71
test -z "$(git status --porcelain)"
git restore --source="$A10" --staged --worktree -- .documents/WIP/workorder-active.md
test "$(git rev-parse ':.documents/WIP/workorder-active.md')" = \
     "$(git rev-parse "$A10:.documents/WIP/workorder-active.md")"
test "$(git diff --cached --name-only)" = '.documents/WIP/workorder-active.md'
# Commit/push this one authorized path, then run the local-first technical sequence.
```

Do not use `git checkout main --` or restore the older ACTIVE from T9. Never merge/cherry-pick the technical branch into `main` under this order. If a CI error occurs, fix and forward-commit **within the ledger**, rerun local checks, and dispatch new exact-SHA jobs; retain prior red proof honestly.

## 3. Closed edit ledger and exact corrections

**Allowed technical-branch paths** (including their existing T9 modifications):

```text
.documents/WIP/workorder-active.md                         # exact A10 blob ONLY
Cargo.lock
crates/solo-adapters/Cargo.toml
crates/solo-adapters/src/graphic_avif_inspection.rs         # new AVIF child-only codec
crates/solo-adapters/src/graphic_image_inspection.rs         # existing confined child/V3 adapter
crates/solo-adapters/src/lib.rs
crates/solo-adapters/src/media_worker_protocol.rs            # strict typed envelope
crates/solo-application/src/graphic_image_inspection.rs      # additive V4 application/port
crates/solo-application/src/media_worker.rs                  # inward parent verifier
crates/solo-application/src/lib.rs
crates/solo-cli/tests/architecture_checks/graph.rs           # exact graph allowlists
crates/solo-cli/tests/media_worker_host_smoke.rs             # 7 real N118 tests
crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md # pinned independent goldens
crates/solo-adapters/tests/graphic_avif_inspection.rs        # convert exploratory probe to assertions
.github/workflows/rust-verify.yml                            # two-host N118 gate
```

On `main` the only allowed permanent changes are the A10 ACTIVE and R10 canonical report. All other files **remain byte-identical** to the accepted baseline: no `main.rs`, `media_worker_host_native.rs`, `source_read_native.rs`, architecture/source/unsafe checker implementation, `workspace_graph.rs`, `deny.toml`, root `Cargo.toml`, new binary fixtures, new dependencies beyond pinned T9, SOP/Architecture/roadmap/other WIP, outside-Solo or platform assets. The seven immutable `crates/solo-cli/tests/fixtures/avif/*.avif` files remain test-only, never published. No new unsafe/native boundary or shell execution path.

### 3A. Complete the exact dependency graph — *already authorized by R01*

T9 already pins `oxideav-avif = "=0.0.16"` in `solo-adapters` with `default-features = false, features = ["registry"]`. `Cargo.lock` gained **six package names**, with no existing version displacement: `oxideav-avif 0.0.16`, `oxideav-av1 0.1.24`, `oxideav-heif 0.0.15`, `oxideav-core 0.1.43`, `thiserror 2.0.21`, `thiserror-impl 2.0.21`. Validate these exact facts against *actual* `cargo metadata --locked --all-features --format-version 1` and `Cargo.lock`, not only the agent report. Validate resolved features, MSRV, MIT/licence/NOTICE and `cargo-deny`/RustSec sources; no version downgrade, new `*-sys`, native library, bypass or blanket allowlisting.

Edit only the shared `crates/solo-cli/tests/architecture_checks/graph.rs` declarations: add `"oxideav-avif"` as exactly one new `APPROVED_ADAPTER_EXTERNAL_NORMAL` member (**9→10**) and enumerate each new package in the actual adapter transitive closure. The T9 package list implies **47→53** distinct external closure names, namely the root plus **five** transitives (`oxideav-av1`, `oxideav-heif`, `oxideav-core`, `thiserror`, `thiserror-impl`); the earlier report's phrase "four transitive names" is a counting mistake. Verify the set by traversing `cargo metadata` resolve edges; if actual graph differs, **record the exact difference and refuse unapproved nodes**. Keep all names sorted as the existing style, the graph fail-closed, and both fixture and real-tree tests unchanged. In `.github/workflows/rust-verify.yml` replace both retained `edges=81` assertions with **the single exact value measured by `cargo test -p solo-cli --test check-architecture --locked` after the authorized product+dependency changes**; never wildcard or remove either assertion. Do not claim the changed graph is validated until the AVIF implementation and two-host gates below are green.

### 3B. Real bounded AVIF child decoding and strict MIME/container policy

Use the **existing** `crates/solo-cli/src/main.rs` internal dispatch, `solo_adapters::run_internal_image_worker_v1`, `media_worker_host::run_confined_image_worker_v1`, `ContentRootCapability`/confined-source port, and application `MediaWorkerRequestV1`, `MediaWorkerPortV1::inspect_avif`, `inspect_media_worker_source_set_v1`, `MediaWorkerEnvelopeV1`, `media_worker_output_sha256_v1`, and `ConfinedGraphicInspectAdapterV3`. Read their actual implementations before altering them. The compiled worker binary, arguments, environment, inherited handles, stdin/stdout framing, deadlines, termination and native limits are already owned by these audited components. **Extend their closed AVIF token path; never add another launcher, source reader or publication path.**

The parent does only already-authorized confined source acquisition, source length `1..=20 MiB`, digest and single-use worker invocation. **Do not parse hostile AVIF headers in the unbounded parent.** Inside the already OS-limited/reverified child, validate actual `ftyp`/AVIF brands, primary item, `av1C`, codec, dimensions, pixel product, essential transforms, alpha association, grid/sequence refusal, colour model and coded depth, then fully decode with the pinned `oxideav_avif::decode(&bytes)` API when policy admits it. The T9 decoder returning pixels for invalid-policy fixtures proves only that policy must be separately enforced; do not treat decoder `Ok` as permission to accept. Use `oxideav_avif::inspect(...)` / exact 0.0.16 typed container information **inside the child** if supported; inspect the installed crate's actual versioned API rather than guessing field names. If the decoder API allocates before exposing required geometry, demonstrate the checked predecode path or refuse the unsupported case; an OS memory limit alone does not prove the application-level preallocation contract.

Core checked arithmetic — apply to actual child header geometry *before* decoded-buffer allocation and again to independently verified parent claims:

```rust
fn verified_dimensions(width: u32, height: u32) -> Option<u64> {
    if !(1..=8_192).contains(&width) || !(1..=8_192).contains(&height) {
        return None;
    }
    let pixels = u64::from(width).checked_mul(u64::from(height))?;
    (pixels <= 32_000_000).then_some(pixels)
}
fn verified_source_size(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && u64::try_from(bytes.len())
            .is_ok_and(|n| n <= 20 * 1024 * 1024)
}
fn verified_plane_length(stride: u64, rows: u64, actual: usize) -> bool {
    stride.checked_mul(rows)
        == u64::try_from(actual).ok()
}
```

Require complete decoded sample values, 8-bit byte planes and **unaltered 10/12-bit sample precision** in little-endian 16-bit row-packed planes; use checked row/column geometry for YUV 4:2:0/4:2:2/4:4:4, odd-size rounding and alpha. Reject incorrectly associated alpha, non-essential unsupported transforms in the committed negatives, animated AVIS/unsupported grids instead of first-frame substitution, invalid `clap`/`irot`/`imir`, malformed/truncated containers, wrong file extension/magic/MIME, overflow, decompression bomb and unsupported colour metadata. The positive `abc_color_irot_alpha_irot.avif` must produce correctly oriented and alpha-associated final pixels; `abc_color_irot_alpha_NOirot.avif` and `clap_irot_imir_non_essential.avif` must refuse **by explicit authored media-policy validation**, even though T9's raw decoder returns planes for them.

Preserve the native worker reference bound: 536,870,912-byte process address/commit profile, 10 CPU seconds, 15,000 ms outer wall deadline, normal exit, verified OS limit readback, no dangerous inherited environment/handle, bounded stdout and protocol, complete descendant kill/reap and temporary cleanup on every path. Do not exaggerate CPU time granularity or confuse these limits with a complete network/filesystem sandbox. Native syscall-level network/filesystem denial remains **UNVERIFIED** until independently proven, which is not required for V4 *technical* verification but **precludes production-security and Stage 03 closure**. Any actual discovered unowned resource/credential/handle leak is a blocker.

### 3C. Closed worker protocol and parent-owned verification

Reuse the current 16-field duplicate/unknown-key-refusing `media_worker_protocol.rs` parser and `MediaWorkerEnvelopeV1` **without silently widening V1**. Add a versioned V4 observation extension inside the permitted files only when needed for the precisely testable alpha/colour/transform claims. Worker result: exact version, `format=avif`, `mime=image/avif`, source length+SHA-256, width/height, actual bit depth, normalized colour model/full-range status, alpha/orientation/grid/sequence facts, ordered plane claims `(ordinal,stride,rows,length,sha256)`, aggregate hash, **and the actual complete plane bytes**. The outer parent may not accept a text-only claim.

The parent recomputes source SHA-256 and the hashes of **bytes actually received**, compares the source digest, confirmed declared format/MIME, all length/stride/row/plane-count claims, canonical output digest, depth/range/alpha/geometry semantics, worker identity, limits, normal exit, elapsed deadline and successful reap. `MediaWorkerCheckedEnvelopeV1` is evidence only, never publication authority. Reject duplicates, unknown keys, missing fields, excessive frames, short/trailing payloads, spoofed PID/digest, forged output, malformed packet, bad ordinal, changed bit depth or inconsistent `orientation`/`is_grid`/`is_sequence`. No fallback after a refusal. Preserve every existing V1/V2/V3 API/test and the deliberate V3 `AvifUnsupported` negative, which **must remain true**.

### 3D. Add the separate, six-format whole-candidate V4

Implement inward-owned `GraphicImageInspectPortV4` and `inspect_graphic_image_source_set_v4` in `crates/solo-application/src/graphic_image_inspection.rs`; implement V4 adapter in permitted `graphic_image_inspection.rs` and `graphic_avif_inspection.rs`. For PNG/JPEG/WebP/GIF/SVG reuse existing V3 inspection with identical formats, immutable animation/vector semantics and no new legacy bypass. For AVIF use the real confined decoder with verified 8/10/12-bit, alpha/rotation and plane digests. `GraphicImageFormatV4::ALL` should be exactly the six accepted formats, and any direct/outside-platform unregistered reference must refuse; `graphic.favicon` remains **PNG only**. Every declared owner (Entity, Facet, Site, Page, representation) and every hidden/internal/suppressed/unlisted obligation is inspected without filtering. One later bad AVIF causes **the entire** mixed six-format candidate to refuse, discarding earlier success, without sibling leakage or automatic Platform fallback. Return no publicly discoverable output, no route and **no publication authority** from this inspection-only work.

## 4. Independent AVIF reference — run off the local rootless WSL host

**No `sudo` or system-package installation is required on the Owner's WSL machine.** The unavailable local independent decoder is not a reason to stop implementing. Prepare reference hashes on the **native Ubuntu 24.04 GitHub Actions runner**, in a **separate, explicitly logged test-only oracle preparation step**; keep all install/download/cache/derived files in runner temporary storage, never in shipped product source, and never link a reference decoder into any Solo crate. The workflow is an authorized file in §3, so add the narrowly required network-provenance exception for a pinned test-only tool. The final CI must pin its version, binary hash and relevant shared-decoder library identities; any drift fails rather than silently regenerating goldens. A first discovery run to measure exact versions and hash identities is **NOT** a green V4 acceptance run.

The Ubuntu runner has an administrator-capable test environment. **After checking installed availability**, use its package manager solely for the reference tool (not product dependencies). In the first oracle-preparation step, record actual package repository, exact package version, package archive SHA-256, licence and transitive shared library versions; lock those values into `LICENSE-AND-PROVENANCE.md` and enforce them on the later final verification run. Do not install a floating tool in final CI without exact identity checks. Example commands for the runner's test-only preparation shell (inspect the actual `avifdec --help` before using flags):

```bash
set -euo pipefail
if ! command -v avifdec >/dev/null 2>&1; then
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends libavif-bin
fi
command -v avifdec
avifdec --version
avifdec --help
sha256sum "$(readlink -f "$(command -v avifdec)")"
dpkg-query -W -f='${Package}=${Version}\n' 'libavif*' 'libdav1d*' || true
ldd "$(command -v avifdec)"
# Reference 8/10/12-bit planes: Y4M retains source depth.
# Ensure the installed CLI actually supports --codec dav1d; otherwise refuse that oracle.
avifdec --codec dav1d --jobs 1 --size-limit 32000000 --dimension-limit 8192 \
  crates/solo-cli/tests/fixtures/avif/fox-profile0.10bpc.yuv420.avif \
  "$RUNNER_TEMP/fox10-reference.y4m"
```

**Correctness of reference extraction matters.** Parse `YUV4MPEG2` header, `FRAME` marker, format, dimensions and exact remaining payload **outside the candidate decoder**. Extract row-packed Y/U/V planes using independently calculated chroma geometry, record SHA-256 of each actual plane and the typed aggregate. For 10/12-bit Y4M retain little-endian 16-bit values; verify samples fit coded depth. Validate 8-bit and 12-bit the same way. Do **not** hash the entire Y4M file and call that a plane hash, confuse its header with decoded pixels, or rely on self-generated `oxideav` outputs as an independent oracle.

**The crucial orientation caveat:** libavif's Y4M writer can output *unrotated* planes and explicitly warns when orientation is ignored. **Do not use Y4M output as proof of the displayed orientation of `abc_color_irot_alpha_irot.avif`.** Obtain independent `avifdec` PNG output at supported bit depth, inspect its actual display dimensions and pixel values through a separate pinned PNG reader, and/or construct a manually audited pixel-coordinate mapping from the raw reference planes plus the correctly parsed `irot` transform. Verify **both colour and alpha** after rotation (alpha must follow the identical geometry); compare with actual V4 results. A tool that ignores transform or alpha cannot serve as an oracle for those properties. If independent tools disagree, preserve disagreement and refuse that fixture rather than edit a golden to fit the candidate. Upstream reference: `https://github.com/AOMediaCodec/libavif/blob/main/doc/avifdec.1.md` and `https://github.com/AOMediaCodec/libavif/blob/main/apps/shared/y4m.c`.

Record in the permitted `LICENSE-AND-PROVENANCE.md`: each immutable fixture name, source SHA-256, independent decoder provenance/identity, reference command, actual dimensions, depth, colour/alpha/orientation, independent Y/U/V/(A) raw-plane digests, any display-oriented RGB(A) digest and the method applied to recover correct orientation. **Only independently obtained values may become the hardcoded N118 oracles**. First hosted reference measurements may be used to pin exact values, followed by a fresh, clean same-SHA verification after committing the pins. Distinguish *reference tool correctness*, *candidate correctness* and *public presentation*; none proves another by assertion.

## 5. Mandatory seven real-host tests; load-bearing workflow

Implement exactly seven qualified `n118::n118_*` tests in `crates/solo-cli/tests/media_worker_host_smoke.rs` on real confined source snapshots and the actual compiled `solo-cli`, with exact names:

1. `n118_01_six_formats_five_efs_levels_real_host`: distinct qualified EFS owners across five levels, exactly six formats, real separate worker PIDs, retained three-frame GIF and true scalable SVG, full AVIF plane evidence, no partial/public result.
2. `n118_02_eight_ten_twelve_golden_planes`: real 8/10/12-bit decodes, exact independent per-plane/aggregate hashes, stride, decoded sample ranges, dimensions and same reference across hosts.
3. `n118_03_alpha_rotation_and_invalid_transform`: valid `abc_color_irot_alpha_irot` with **independently oriented** colour and alpha, rejected missing alpha association and unsupported nonessential transforms, no accepted output for either negative fixture.
4. `n118_04_hostile_media_and_early_limits`: empty and oversized source refused **before worker**, mismatched AVIF file/magic/observed MIME, truncated ISOBMFF, bad `ftyp`/`av1C`, missing primary, missing/corrupt alpha, geometry overflow, unsupported AVIS/grid/codec/transform, bogus pixel range and bomb refused whole.
5. `n118_05_worker_forgery_and_resource_escape`: forged source/plane/aggregate hashes, short/trailing payload, bad 16-field envelope, PID/limit/reap/timeout/process failure, real inheritance/handle/environment/descendant controls. No assertion of unproved syscall sandboxing.
6. `n118_06_hidden_suppressed_late_failure_discards_whole`: a later corrupt/internal/unlisted/suppressed AVIF refuses a previously valid mixed six-format candidate; no sibling bleed, role fallback, internal exposure or partial asset.
7. `n118_07_red_green_and_digest_sensitivity`: a one-bit independent golden corruption makes the **real** test fail nonzero, then the unchanged test passes; a forged worker claim never passes parent validation.

The existing N097/N102/N108/N113/N116 suites, snapshot/source refusals, RustSec and cargo-deny, format/clippy, graph/fixtures, typed protocol and reproducible builds must remain **unchanged and green**. Test-only mocks supplement but never replace host proof. Do not rename, ignore, skip or weaken old tests. No `continue-on-error`, test filtering that conceals a failure, broad `if` skip or ad hoc local shortcut.

Extend `.github/workflows/rust-verify.yml` with Ubuntu and Windows N118 gates and an N118 RED/GREEN oracle step. Each host runs:

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked n118_ -- --nocapture --test-threads=1
```

For each gate require the **seven exact names**, exactly seven matched test-start lines, literal `test result: ok. 7 passed; 0 failed; 0 ignored`, absence of FAILED/ignored, all seven semantic evidence tokens, and the pinned reference digests. Use fixed substring matching for individual `test <name> ` lines because `--nocapture` interleaves stdout with Rust's test display; do **not** require `... ok$` on those lines. Run the *same* checker against seven corrupted log variants (missing test, FAILED, ignored, duplicate test, 6/7 count, wrong digest, missing refusal evidence); every variant must be rejected. Include deliberate real RED (`n118_07` with a test-only wrong-oracle setting) and clean GREEN runs. Retain all N116 case/mutation checks and both exact `edges=` assertions, substituting only the honestly remeasured numeric value.

Example boundary checks that must remain load-bearing:

```bash
# Run in the real repo, with the actual source-root prerequisites where available.
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-cli --test media_worker_host_smoke --locked n118_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

**Local-first:** on Windows 11 and/or existing WSL Ubuntu 26.04 use this list before technical commits/hosted dispatch, record every exit code, and fix code/test defects locally. On WSL lacking a real Btrfs `SOLO_TEST_SOURCE_FS_ROOT`, run the host-independent checks and label snapshot-dependent tests **NOT RUN LOCALLY**, never passed/skipped/waived. Hosted Ubuntu **must** create/use the real disposable Btrfs source root and actually run all source/snapshot/N113/N116/N118 cases. Hosted Windows actually runs native handle/resource/worker checks. The independent oracle is constructed on the authorized hosted Ubuntu runner, not faked or replaced by candidate output on rootless WSL.

For final technical acceptance, dispatch the branch workflow explicitly at its exact remote SHA:

```bash
gh workflow run rust-verify.yml --ref solo-wo-118-avif-v4-r01
gh run list --workflow rust-verify.yml --branch solo-wo-118-avif-v4-r01 --limit 5
# Then inspect the exact run/job head_sha, both actual job logs, step outcomes, N118 gate and oracles.
```

**Acceptance requires one new complete native Ubuntu 24.04 + Windows 2025 technical run at one identical final branch SHA:** both jobs success, real N118 7/7 and seven mutant refusals, RED/GREEN, matching independent pinned reference hashes, all existing N116/N113/N108/N102/snapshot/security/architecture/reproducibility gates. A green baseline run on `main` is not this proof. Any local/CI failure is repaired in the closed technical ledger and followed by fresh same-SHA full verification; incomplete features or absent independent goldens yield STOPPED, not COMPLETE.

## 6. Canonical report, exclusions and exit

Overwrite the **only** `.documents/Reports/execution-report.md` on `main` with exact A10, ACTIVE-transfer, T10 and R10 commits/trees/parents, changed-file ledger, local commands/result codes, approved lock graph/allowlist/edge count, third-party version/licence/NOTICE/advisory evidence, no-added-unsafe proof, independent oracle source/version/binary and shared-library hashes, fixture values, actual 8/10/12/alpha/rotation decoded pixel/hash comparison, host worker PID/limits/reap/digest/forgery results, N118 7/7+7 mutation/RED and retained suite evidence, technical and report CI runs/job IDs/head SHAs, and honest WSL limitations. Preserve one worktree, empty stash, exactly four WIP files, 28 refs, no new archive, test-only scratch cleanup. Never invent R10's self-SHA in its own content: include full SHA in final chat handoff.

**`status=COMPLETE` means all *technical* AVIF V4 obligations above have been fulfilled and independently evidenced, not that the product is released or Stage 03 is closed.** Explicitly keep `R03.C` and Stage 03 **OPEN** pending separately authorized technical integration, lawful version-pinned Platform `1.0.0` asset catalogue, whole-candidate asset/public exposure/derived-output delivery, and final Stage 03 exit proof. AVIF cannot be described as integrated into `main` under this order. If the same-SHA native run is not green or any security/oracle/media predicate cannot be established, publish `status=STOPPED` with the precise counterexample; do not self-accept. The Chief Master Architect will independently inspect the report, changed blobs and both host logs to issue `ACCEPT / REVISE / STOP`.
