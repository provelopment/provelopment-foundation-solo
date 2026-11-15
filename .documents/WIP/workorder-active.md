# SOLO-WO-118 R04 — DELIVER CONFINED 8/10/12-BIT AVIF STILL DECODING WITH INDEPENDENT PARENT VERIFICATION

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Single active file:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-CONFINED-8-10-12-STILL-PLANE-ORACLE-TWO-HOST-VERIFIED`.

## 1. Mission and exact authority

Implement and **actually exercise** one additive, closed AVIF **ordinary still-image** decode adapter using the existing single-use confined image-worker machinery and the existing inward `MediaWorkerPortV1`/`inspect_media_worker_source_set_v1` contract. The implementation must accept the four pinned **unrotated, non-alpha** reference positives (white 1×1 and the fox 8-, 10- and 12-bit still images), return complete native-depth Y/U/V planes from the actual worker, and make the trusted parent independently verify source identity, worker state, plane geometry, bytes and SHA-256 against the **already accepted R03 independent `avifdec` goldens**. Prove on **native Ubuntu 24.04 and Windows 2025 at the identical technical SHA**, with four real named N120 host tests and a load-bearing CI gate.

This is the next bounded **product implementation** step, not another decoder feasibility probe. `oxideav-avif =0.0.16` and graph roots 10/closure 53/`edges=93` are **already accepted on the branch**; do not repeat their selection or rewrite their version pins. The external reference job, `reference_oracle.py`, `reference-goldens.json`, and fixture provenance were independently accepted as `SOLO-WO-118 R03`; consume them without regenerating or changing goldens. **Do not implement or claim the complete six-format V4 inspection, associated-alpha orientation, automatic output publishing, the Platform asset catalogue, R03.C closure, Stage 03 closure, Stage 04 routing, release or deployment.** Those remain separate mandatory follow-on gates. In this R04 slice, alpha/rotation, grids and AVIS sequences **refuse** rather than being silently accepted or converted to a still. The accepted five-format V3 API and its AVIF refusal remain unchanged.

Binding sources: `AGENTS.md`; `.documents/About/FOUNDATION-SOLO.md`; `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I, 6.2K, 6A.8 (especially the AVIF out-of-process boundary); both Architect and Implementation SOPs; four canonical WIP files; existing report; the source files named below. No second Entity, source/unsafe boundary, external-fetch route, native library, codec dependency or unapproved architecture change. Chief Master Architect's GitHub access is **read-only**; Implementation Agent performs all repository writes.

**Authoritative starting heads, verify full SHAs before any write:**

- `main` and `origin/main`: **`e66f721118c64d62e29b000f1be67a68791a27a2`** (R11 report; baseline `edges=81`).
- Existing technical branch `solo-wo-118-avif-v4-r01`: **`808233516ef280d319f206553915998820cbd9d5`** (T11.2c; `edges=93`). This branch already contains the single exact `oxideav-avif =0.0.16` dependency, valid `Cargo.lock`, sorted fail-closed graph allowlists, R03 oracle, one exploratory decode probe and the independent oracle CI job. Preserve all.
- R03 exact-head CI run `43688309266` (`verify` Ubuntu success, Windows success, `avif-reference-oracle` success); R11 `main` run `43688808792` (both verify jobs success). These are prerequisites only, **not R04 proof**.
- There are **28** existing remote branch heads, including the AVIF technical branch, the preserved SVG branch `solo-wo-115-avif-svg-v3-r02` at `d82ae76273b57b37014d3b4c557b8debd5299eea` and the frozen `ad7b74df8638ff56c7137f33d9242da54583f17b` and `40b03c73b0bc8544359438daea6218caa496e85d`. No new branch is needed.

## 2. Single-checkout topology: A12 → T12 → R12

The only checkout is the established Windows 11 / WSL Ubuntu 26.04 worktree. Before acting record `pwd`, `git rev-parse --show-toplevel`, `git rev-parse HEAD`, `git branch --show-current`, `git remote -v`, `git status --porcelain=v1`, `git stash list`, `git worktree list --porcelain`, `git ls-remote --heads origin`, and the names of `.documents/WIP/*`. Require the one correct `origin`, exactly one worktree, an empty stash, 28 correct remote heads, exactly four WIP files, no foreign/untracked collision, and a clean tree **except the Owner-installed ACTIVE** on `main`. Never delete unrelated data; no new clone/worktree/branch, rebase, `reset --hard`, `git clean`, force, deletion of an existing ref, PR, tag, release or merge. Task scratch only inside the checkout's `.temp/` or the tool-provided task temp; delete only task-owned scratch.

**A12 — `main`:** starting at exact R11 head, commit and push **only** the newly installed `.documents/WIP/workorder-active.md`. Require the working-tree, staged, committed and `origin/main` Git blob all agree. Record the complete A12 SHA, tree and parent; do not change source or the report in A12.

**T12.0 — existing technical branch:** from a clean A12 checkout `git switch solo-wo-118-avif-v4-r01`. Require exact previous branch HEAD `808233516ef280d319f206553915998820cbd9d5` and clean status. Propagate **only** issued ACTIVE via `git restore --source="$A12" --staged --worktree -- .documents/WIP/workorder-active.md`; require `git rev-parse ':.documents/WIP/workorder-active.md'` equals `git rev-parse "$A12:.documents/WIP/workorder-active.md"`, and the index includes **exactly that one path**. Commit/push the ACTIVE-only checkpoint. This is not a merge and must not import the old R03 work order back over the new one.

**T12 technical:** keep the same technical branch and make forward source/test/CI commits only within §3. Run applicable local checks **before every technical push**; the final technical SHA needs a fresh, actual **branch push-triggered** `rust-verify.yml` run with `verify (ubuntu-24.04)`, `verify (windows-2025)` and the already branch-scoped `avif-reference-oracle` all succeeding. Do not use `workflow_dispatch` as the sole acceptance trigger: the reference job's current condition is `github.event_name == 'push'`. Do not merge source into `main` in this order.

**R12 — `main`:** after final technical verification, restore the clean checkout to `main` at A12; overwrite and commit/push **only** `.documents/Reports/execution-report.md`; run a fresh native two-host `main` CI and report both job conclusions. Provide R12's full SHA in the chat handoff, not inside its own report. If a genuine blocking result occurs, report `status=STOPPED` rather than falsely complete, and preserve all code/refs; do not excuse missing work as a completed technical milestone.

## 3. Closed implementation ledger and concrete design

**Allowed permanent technical-branch paths, and no others:**

```text
.documents/WIP/workorder-active.md                           # exact A12 Git blob only
crates/solo-adapters/src/graphic_avif_inspection.rs           # new safe Rust AVIF codec/worker helper
crates/solo-adapters/src/graphic_image_inspection.rs          # add one closed AVIF child dispatch
crates/solo-adapters/src/lib.rs                               # module + narrow exports
crates/solo-application/src/media_worker.rs                   # strengthen parent validator, no V1 widening
crates/solo-cli/tests/media_worker_host_smoke.rs              # N120 4-case real host suite
crates/solo-adapters/tests/graphic_avif_inspection.rs         # replace feasibility-only test with assertions
.github/workflows/rust-verify.yml                             # N120 gate on both hosts
```

`Cargo.lock`, every `Cargo.toml`, the dependency graph checker, `workspace_graph.rs`, existing oracle Python/JSON/provenance and binary fixtures, `media_worker_host.rs`, `media_worker_host_native.rs`, `source_read_native.rs`, `media_worker_protocol.rs`, `solo-cli/src/main.rs`, the unsafe/source guards, old N102/N108/N113/N116 tests, root configuration, SOP, Architecture, roadmap and all other WIP paths are **immutable**. If a structural need genuinely requires a forbidden path, preserve the branch and give exact failure evidence; never smuggle a change outside this ledger. On `main` only A12 ACTIVE and R12 report may change.

### 3.1 Use the existing worker; decode only after confinement

Read the complete current implementations before editing:

```text
crates/solo-cli/src/main.rs (already dispatches solo_adapters::run_internal_image_worker_v1)
crates/solo-adapters/src/graphic_image_inspection.rs (reserved flag, strict format tokens, worker children)
crates/solo-adapters/src/media_worker_host.rs (run_confined_image_worker_v1, limits, N102 isolation)
crates/solo-application/src/media_worker.rs (MediaWorkerPortV1, V1 strict claim validator)
crates/solo-adapters/src/media_worker_protocol.rs (existing strict 16-field envelope parser)
crates/solo-cli/tests/fixtures/avif/reference-goldens.json (R03 independently pinned facts)
crates/solo-adapters/tests/graphic_avif_inspection.rs (T9 decoder probe)
```

Create `graphic_avif_inspection.rs` as **safe adapter-only code** using the already pinned `oxideav_avif::inspect` and `oxideav_avif::decode`; do not add `unsafe`, a native dependency, another child supervisor or filesystem reader. Extend the **existing** `run_internal_image_worker_v1` handler with **one closed `avif` format token** (with source SHA-256 argument and no arbitrary path, command or URL). The only way to reach AVIF decoding must be the confined child. The child first calls `child_image_worker_confinement_v1()` and verifies the fixed `536_870_912` bytes address/commit and 10 CPU-second profile. Only then read bounded stdin (`20 MiB + 1` hard stop), confirm a nonempty source and caller-provided SHA-256, and parse any hostile HEIF/AV1 bytes. The parent must call existing `run_confined_image_worker_v1(executable, "avif", source_digest, source)` with its 15,000 ms deadline, bounded stdout/stderr and verified exit/reap; no product code may call `oxideav_avif::decode` in the trusted parent. The existing CLI public arguments remain unchanged and direct unsupervised internal-worker calls must refuse.

Inside child, validate `ftyp`/AVIF brand plus real primary item/`av1C`, reject missing or mismatched declared AVIF format/MIME, and **preflight** with `oxideav_avif::inspect` before invoking `decode`. Use real documented 0.0.16 `AvifInfo` fields (`width`, `height`, `bits_per_channel`, `max_bit_depth()`, `is_grid`, `has_alpha`, `brands`, `colour`). Require sides 1–8192 and checked product ≤32,000,000 **before pixel-plane allocation**, depth exactly 8, 10 or 12, `is_grid=false`, supported single-still AVIF brand (not `avis`), and `has_alpha=false` for this R04 slice. Any declared or detected transform/alpha association whose full displayed behavior cannot be proved in R04 **refuses**, never silently ignored. Do **not** special-case the fixture name, content digest or reference values inside production code; classification derives from actual container and decoded structure. The fixed negative fixtures with alpha and/or transforms must remain refused **as unsupported by this intermediate slice**, not mislabelled corrupt and not used as successful V4 coverage. Treat AVIS, grids, unsupported essential properties, invalid codec data, weird colour modes and unsafe buffers as typed refusal. If necessary to detect unsupported transforms, parse their bounded **container metadata inside the child** through the pinned decoder's trusted typed structures; do not implement a permissive, unbounded generic box parser or trust extension/brand alone.

Decoded Y/U/V plane bytes must be complete, row-packed and in canonical component order; for 8-bit use one byte/sample; for 10/12-bit use little-endian 16-bit **without quantization**. Check each stride, row count and length using checked arithmetic; for 4:2:0 the U/V dimensions are `(width+1)/2` and `(height+1)/2`, with checked rounding; for 4:4:4 all are full dimensions. The actual four independent R03 positive goldens are `white_1x1.avif` (1×1/8/444), `fox-profile0.8bpc.yuv420.avif` (1204×800/8/420jpeg), `fox-profile0.10bpc.yuv420.avif` (1204×800/10/420p10), and `fox-profile2.12bpc.yuv420.avif` (1204×800/12/420p12). R03 hashes are **test expectations only** and may not drive decoding or be substituted for received pixels.

### 3.2 Exact protocol and trustworthy parent check

Build the full output **privately before writing any stdout**: one bounded length-delimited/line-delimited version-1 JSON claim in the existing `MediaWorkerEnvelopeV1` **16-field** grammar followed by the exact complete Y/U/V byte payload. No plaintext partial success, stderr content dumps, unexpected trailing byte, spare frame, missing plane, invalid UTF-8 or oversized output may be accepted. Serialize a closed JSON object with fields identical to `media_worker_protocol.rs` (`version`, `status`, `format`, `mime`, `source_length`, `source_sha256`, `width`, `height`, `bit_depth`, `color_model`, `has_alpha`, `is_grid`, `is_sequence`, `orientation`, `planes`, `output_sha256`). Use existing V1 tokens `avif`, `image/avif`, `decoded`, `yuv`, `identity`, and the previously specified `MediaWorkerPlaneClaimV1` ordinal/stride/rows/length/sha256. Every decoded-plane length and the aggregate `media_worker_output_sha256_v1` must be computed **from the actual byte payload**. No extra claim key is permitted; preserve `parse_media_worker_envelope_v1` unchanged.

Implement a narrow `ConfinedAvifInspectAdapterV1` in the new safe module implementing existing inward `MediaWorkerPortV1::inspect_avif`. The request arrives as bounded, owned source bytes from the existing `GraphicAssetSourceSetV1` / `inspect_media_worker_source_set_v1` path; the parent must calculate SHA-256 itself, launch only the approved child, parse exactly one closed JSON envelope with the existing parser and read exactly its bounded plane byte count. Before constructing `MediaWorkerObservationV1`, verify source SHA, exact frame boundary, actual received bytes, no extra byte, normal exit, worker identity, limits verified, elapsed wall budget, all descendants cleaned/reaped. Keep the parent source/decode separation unidirectional: worker may claim dimensions, but parent independently checks 1–8192 and 32M pixels, exact depth, 4:2:0/4:4:4 canonical rows and byte strides (including 16-bit little-endian samples), `has_alpha=false`, `is_grid=false`, `is_sequence=false`, `orientation=identity`, plane count 3, each digest and aggregate digest **on received bytes**. Strengthen `check_observation_v1` in the existing application module where its current `stride×rows`-only checks do not bind plane shape to dimensions; do not widen the V1 wire grammar or weaken any negative. Add narrow tests for swapped chroma, noncanonical stride/rows, forged depth, short payload, hostile JSON, false normal-exit and false cleanup claims. This V1 checked result is **nonpublishing** and is not a V4 graphic candidate.

No child creates a filename, temp file, route, URL, public artifact, subprocess, network request or persistent decoded data. All supplied bytes are confined source data; never infer publication authority or global snapshot from a successful codec result. OS resource limits and N102 negative tests do **not** establish general filesystem/network syscall isolation; record that as **UNVERIFIED** and leave production publication blocked.

## 4. Load-bearing named N120 real-host suite

Create exactly these **four** real tests in `crates/solo-cli/tests/media_worker_host_smoke.rs`, using the real built CLI child and the approved actual `ContentRootCapability` source reader. Reuse the existing Btrfs-backed `Root` fixtures and preserve existing N116 and N102 tests. Tests on WSL lacking `SOLO_TEST_SOURCE_FS_ROOT` are explicitly NOT RUN LOCALLY, never silently skipped; Ubuntu Actions supplies the real Btrfs fixture and Windows runs native confined source/worker verification. A fake port alone is not host proof.

1. **`n120_01_four_independent_still_goldens_real_host`** — four real, separate bounded child processes; source/plane hashes from the actual bytes match the unchanged R03 `reference-goldens.json`, exact width/height/bit depth/stride/rows, 8/10/12 precision, normal exit, proved limits/reap and distinct positive worker IDs. A structurally correct but pixel-wrong decoder must fail.
2. **`n120_02_unsupported_and_hostile_media_refuse_whole`** — real-source early empty and >20 MiB refuse **before spawn**, wrong extension/MIME/magic, truncated or malformed AVIF, AVIS/grid, invalid dimensions, alpha/rotated positive-for-future fixture, two R03 malformed-policy fixtures, unsupported properties and corrupt later owner refuse without partial result. None is accepted as first frame, static substitute, truncated input or nonempty success set.
3. **`n120_03_forgery_transport_and_boundary_refusals`** — exact source/output/one-plane digest forgery, reversed planes/rows/stride, duplicate/unknown JSON key, extra trailing/short payload, nonzero worker exit, timeout, unverified limits/cleanup, direct internal-flag invocation and absent executable all refuse. Provide **closed test-only modes** with no general arbitrary command or production bypass; real normal decode remains green. The existing N093/N102 tests remain mandatory and unchanged.
4. **`n120_04_deliberate_red_green_pixel_oracle`** — run the same real child against the real immutable image bytes; a **test-only** environment-controlled corruption of one pinned reference digest makes the *test itself exit nonzero*; the pristine rerun passes; there is no production oracle override. Print `n120_oracle mode=RED rejected=true` only when the malicious test run really fails and print `n120_oracle mode=GREEN accepted=true` for a real accepted run.

Each passing case emits its own uniquely parseable `n120_evidence case=01|02|03|04 ...` line with actual machine facts. The N120 test code must not parse the independent manifest from an unbounded arbitrary outside path or alter any checked golden. The immutable manifest is compile-time/test-repository input only. Existing V3 `avif-unsupported-v3`, N116 4/4, all architecture/unsafe, N102, cargo-deny and source-snapshot gates are **unchanged**.

## 5. Local-first and exact CI acceptance

Before each technical push, run in the existing Windows checkout and/or same-checkout WSL, recording exit codes:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-cli --test media_worker_host_smoke --locked n120_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

Run real local N102/N116 host tests where prerequisites exist and the full suite where possible; distinguish WSL missing Btrfs from actual failures. Verify `architecture=legal packages=4 targets=33 files=64 edges=93` at the technical tip and existing source/unsafe guards. Run RustSec/cargo-deny when the exact approved local tooling exists; hosted CI is mandatory and decisive.

Extend only `.github/workflows/rust-verify.yml` to run **the exact four N120 cases** with `--nocapture --test-threads=1` in native Ubuntu and Windows verification. Keep **all** retained gates and the independent `avif-reference-oracle` job intact; don't add ignored tests, `continue-on-error`, loose regex, `|| true`, reduced timeouts, weaker N093/N102 or graph wildcards. The gate must check four distinct qualified test names, exactly `test result: ok. 4 passed; 0 failed; 0 ignored` (allow Rust's appended measured/filtered tail), four per-case evidence tokens, actual R03 digest comparisons, and both expected RED/GREEN outcomes. The RED command must genuinely return nonzero for a deliberately corrupted oracle, then the pristine run must return zero; do not print RED acceptance based only on a string. Adapt to `--nocapture` output interleaving using robust fixed-string/substring matching as in accepted N116, not fragile test-line suffix patterns. Run at least **four** mutated copies of an authentic N120 log — missing case, duplicated case, altered digest/evidence, and changed aggregate count/ignored case — and require all to be rejected by the **same** checker used for the original. Print `n120_gate authentic=accepted`, `n120_gate mutations=rejected-all` only after genuine checks.

At the final T12 full SHA, a **new branch push** must have a single GitHub Actions run with: `verify (ubuntu-24.04)` **success**, `verify (windows-2025)` **success** and `avif-reference-oracle` **success** (not skipped), all with the same exact head SHA. Record run ID, attempt, three job IDs and mandatory logs. A prior red attempt remains recorded; a later green at another SHA never retroactively validates the red SHA. An intermittent N093/N102 refusal is neither waived nor called a pass; record its exact native diagnostic, address a reproducible safety defect only under separately lawful scope, and require a fresh **all-green** run. No attempt to mask the test by rerunning only a filtered passing subset qualifies.

## 6. Final reporting, scope and exit gate

Overwrite the single canonical `.documents/Reports/execution-report.md` only on `main` as R12. Record the actual A12/T12.0/final-T12 SHAs, trees and parents; exactly changed files; original and final `Cargo.lock`, graph sets and edge count; unchanged oracle manifest/tool provenance; exact white/fox per-plane SHA and source SHA proven by native Linux/Windows; raw worker PIDs, limits, bit depths, planes, product-policy refusals, independent parent rehash and RED sensitivity; all four N120 outcomes and log mutations; retained N093/N102/N116/security/snapshot/architecture results; local limitations; three job IDs and run SHA; `main` R12 two-host baseline CI; 28 remote heads, one checkout/worktree, empty stash, four WIP files, clean state and task-owned cleanup. Report actual N093/N102 intermittence without filtering or rewriting old evidence.

`status=COMPLETE` and outcome `R03C-AVIF-CONFINED-8-10-12-STILL-PLANE-ORACLE-TWO-HOST-VERIFIED` are authorized **only** after the actual product path, four N120 native-host cases, all retained security checks and the same-SHA three-job CI are green. Anything less is `status=STOPPED` with exact failed command/log, useful preserved technical work and narrowly identified blocker. **No V4/alpha-orientation/AVIF publication/candidate completeness, R03.C or Stage 03 closure may be claimed on R04.** The follow-on implementation target is the all-owner V4 six-format candidate, valid alpha/rotation and the full seven-case N118 suite, followed by exact-blob integration and the lawful Platform catalogue/exit gates.

**Execute this work order. No further architecture or Owner-decision checkpoint is part of this bounded mission.**
