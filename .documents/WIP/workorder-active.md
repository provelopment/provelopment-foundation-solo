# SOLO-WO-118 R05 — DELIVER THE FIRST REAL CONFINED AVIF CODED-PLANE WORKER AND TRUSTED-PARENT PATH

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Canonical ACTIVE path:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1`.

## 1. Decision and precise mission

`SOLO-WO-118 R04` was independently reviewed and **REVISE** was issued. Its real Rust comparison against the R03 independent goldens is retained as useful evidence, but **no AVIF worker or trusted-parent path was implemented**. This R05 order exists to deliver that missing runtime vertical slice, not another parser investigation, feasibility test, pinned oracle or benchmark. **The first technical source checkpoint must contain working production runtime changes** in the AVIF worker, child dispatch and parent port; a test-only commit does not discharge this order.

Deliver one *nonpublishing, coded-plane-only* AVIF inspection through the existing **single-use, confined `solo-cli` child**, the existing `MediaWorkerPortV1` and the existing `inspect_media_worker_source_set_v1` parent validator. Four authentic still inputs — `white_1x1.avif` and `fox-profile0.{8,10}bpc.yuv420.avif` plus `fox-profile2.12bpc.yuv420.avif` (use the exact on-disk names in §4) — must return complete, exact native-depth Y/U/V bytes across the worker protocol; the trusted parent must validate the received bytes, geometry, source identity, limits and cleanup; *real native Ubuntu 24.04 and Windows 2025* must exercise and accept the same implementation on one full branch commit SHA. The accepted pinned `reference-goldens.json` supplies independent **test expectations**, never production allowlists, cached media output or a decoder shortcut.

**Narrow but truthful semantics:** This order proves **decoded coded planes**, not user-display orientation or alpha-transform correctness. `MediaWorkerOrientationV1::Unverified` must be used for the R05 AVIF claim; do not invent `identity`/correctly oriented display pixels from an API that does not attest transforms. Refuse `has_alpha=true`, `is_grid=true`, AVIS/sequence and any unsupported/ambiguous coding before accepting; refuse known transforms where detected. If a source's transform semantics cannot be proved, its R05 record can never be treated as display-ready, correct for site output or published. A later V4 gate must explicitly prove displayed geometry, alpha association, transforms and whole-candidate policy. The accepted V3 `AvifUnsupported` refusal remains unchanged. No Stage 03, `R03.C` or V4 completion claim is authorized.

**Starting verified remote state:**

- `origin/main` = **`c6775e37b7d23c397d8afef4b99a8e76b4c7488e`** (R12 canonical STOPPED report); predecessor A12 `c7862da36146107d56dedc728603bc43e1ecbe6b`.
- Existing sole AVIF branch **`solo-wo-118-avif-v4-r01`** = **`cd3866d41db7feb044e0a619847dc9927d420245`** (T12.1). Its A12 ACTIVE-import predecessor is `4544b9e47e6ac6755b516d22daaed3a660f5b3b8`.
- Technical branch retains the R03 pinned independent oracle, R04 Rust golden assertions, `oxideav-avif = "=0.0.16"` with its six locked nodes, `APPROVED_ADAPTER_EXTERNAL_NORMAL[10]`, `APPROVED_ADAPTER_CLOSURE[53]`, real-tree `edges=93` and unchanged N116/architecture guards. The branch push run [43701486229](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43701486229) was green in Ubuntu, Windows and oracle **without a worker**. The R12 `main` run [43701944219](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43701944219) was green on both hosts, also **not worker proof**.
- Existing **28** remote branch heads; preserve `solo-wo-115-avif-svg-v3-r02` at `d82ae76273b57b37014d3b4c557b8debd5299eea`, `ad7b74df8638ff56c7137f33d9242da54583f17b`, `40b03c73b0bc8544359438daea6218caa496e85d` and all other heads without modification.

Read `AGENTS.md`, both SOPs, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I, 6.2K, 6A.8, the three WIP plans, the single ACTIVE, the current report and the actual source/test code **in full** before editing. The Chief Master Architect only performs GitHub read-only verification. All repo writes belong to the Implementation Agent after Owner activation.

## 2. Single-checkout transaction: A13 → T13.0 → T13 → R13

In the **existing single Windows 11 / WSL Ubuntu 26.04 checkout** inspect:

```bash
pwd
git rev-parse --show-toplevel
git branch --show-current
git rev-parse HEAD
git remote -v
git status --porcelain=v1
git stash list
git worktree list --porcelain
git ls-remote --heads origin
git ls-files '.documents/WIP/*'
```

Require one original `origin` (GitHub URL with optional `.git`), one worktree, empty stash, 28 known remote branches, exactly the four WIP files (`plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, `workorder-active.md`) and a clean checkout except the precisely installed new ACTIVE on `main`. Do not clone, create a worktree or branch, force, rebase, `reset --hard`, `git clean`, delete Owner data, discard unknown files, create a PR or change other refs. Scratch is confined to `.temp/` or the agent's assigned task temp; delete **only task-created** scratch.

1. **A13 / `main`:** with `HEAD=c6775e37...`, commit/push **only** the Owner-installed `.documents/WIP/workorder-active.md`. Confirm its Git blob matches the working tree, index, A13 commit and `origin/main`. Record full SHA/tree/parent. Do not modify source or the report.
2. **T13.0 / existing branch:** from verified clean A13, switch to existing `solo-wo-118-avif-v4-r01`; require `HEAD=cd3866d41db7feb044e0a619847dc9927d420245` and clean tree. Copy **only** the issued ACTIVE from full A13 SHA via:
   ```bash
   git restore --source="$A13" --staged --worktree -- .documents/WIP/workorder-active.md
   test "$(git rev-parse ':.documents/WIP/workorder-active.md')" = "$(git rev-parse "$A13:.documents/WIP/workorder-active.md")"
   test "$(git diff --cached --name-only)" = '.documents/WIP/workorder-active.md'
   ```
   Commit/push the ACTIVE-only branch checkpoint. Never import the older R04 ACTIVE over R05. No merge or cherry-pick.
3. **T13 / working technical continuation:** implement §3 **product runtime first**, then §4 real tests and §5 CI on this same branch through ordinary forward commits. Run applicable local gates before each technical push. A fresh **push-triggered** `rust-verify.yml` at the final **one exact technical SHA** must have `verify (ubuntu-24.04)`, `verify (windows-2025)` and `avif-reference-oracle` all successful (the oracle is currently restricted to the branch's `push` event; `workflow_dispatch` alone will not run it). All retained gates must also pass. No source integration to `main`.
4. **R13 / `main`:** return only after verified clean technical branch. On `main` overwrite/commit/push **only** `.documents/Reports/execution-report.md`; verify the fresh `main` run's native Ubuntu and Windows results. Include full R13 SHA in handoff chat, not in its own canonical report. If and only if a real safety, authorization, API or native blocker prevents completion, report `status=STOPPED` with actual failing code or command, evidence and preserved work. **No test-only checkpoint, report-writing, speculative potential issue or absence of a locally installed oracle is itself a technical STOP condition.** Previous milestones already proved the dependency, codec and independent reference.

## 3. Closed product implementation ledger and contract

Authorized permanent paths on the technical branch:

```text
.documents/WIP/workorder-active.md                       # exactly A13's Git blob, no editing
crates/solo-adapters/src/graphic_avif_inspection.rs       # new, actual child codec/parent port
crates/solo-adapters/src/graphic_image_inspection.rs      # exactly one closed avif worker arm
crates/solo-adapters/src/lib.rs                           # narrow module/export registration
crates/solo-application/src/media_worker.rs               # independent parent geometry/digest checks
crates/solo-application/tests/media_worker.rs             # existing V1 negative tests, if needed
crates/solo-cli/tests/media_worker_host_smoke.rs          # two N121 real-host test cases
crates/solo-adapters/tests/graphic_avif_inspection.rs     # preserve R04 goldens; strengthen if needed
.github/workflows/rust-verify.yml                          # small two-host N121 gate only
```

Only A13 ACTIVE and R13 report on `main`. **No** `Cargo.toml`/`Cargo.lock`, architecture checker, source/unsafe scanner, `media_worker_host.rs`, `media_worker_host_native.rs`, `media_worker_protocol.rs`, `source_read_native.rs`, `solo-cli/src/main.rs`, reference Python/JSON/provenance, AVIF binary fixtures, N093/N102/N108/N113/N116 test behavior, `deny.toml`, foundation architecture/SOP, roadmap or other WIP changes. The already reviewed graph remains `edges=93` on the branch. If a genuinely required source outside this ledger is identified, provide the exact compiler/checker error and security rationale; do not bypass a guard or silently change the ledger.

### 3.1 The child must actually exist

Use **existing** `solo_adapters::run_internal_image_worker_v1` (already dispatched before the public CLI), `media_worker_host::child_image_worker_confinement_v1`, `run_confined_image_worker_v1(executable, "avif", source_sha_hex, source)`, `MediaWorkerRequestV1`, `MediaWorkerPortV1`, `MediaWorkerEnvelopeV1` and `parse_media_worker_envelope_v1`. Do **not** add a new executable, loader, general dispatcher, source reader, native syscall/unsafe, process supervisor, environment-controlled product bypass, network client or persistent output.

Create `graphic_avif_inspection.rs` in safe Rust. Wire **one literal token** `avif` in the reserved worker dispatch: exact flag + token + canonical lower-hex source digest; reject unknown mode/extra arguments, and a direct call outside approved OS limits refuses *before any source parsing*. In the child:

1. Call `child_image_worker_confinement_v1()` and `matches_profile_v1()` **first**. Required profile: `536_870_912` address/commit bytes, 10 CPU seconds; the trusted outer supervisor holds the 15,000 ms wall budget and reaps the child.
2. Read **at most 20 MiB + 1 byte** of stdin; reject empty, extra/oversize, read failure, wrong caller source digest. Compute the digest over the actual bytes.
3. Only **after** the verified confinement/source checks, call the genuine pinned `oxideav_avif::inspect(&bytes)`. Reject absent/invalid AVIF brand/codec `av1C`, `avis` sequence, grid, alpha and unsupported coded depths. Calculate checked width and height, require each in `1..=8_192`, and `width * height <= 32_000_000` **before** calling `oxideav_avif::decode(&bytes)`. If the published 0.0.16 `inspect` API does not establish a needed predecode bound, do not fake it: use the actual typed inspection results and fail closed when uncertainty remains. The four already-proven positives must be individually tested, not accepted by filename, fixed digest or hard-coded pixels.
4. Decode fully and verify actual returned dimensions/depth/chroma layout against checked predecode facts. This R05 output is **coded Y/U/V plane evidence**, not normalized RGB, no silent 10/12→8 conversion, no first-frame substitution, no alpha flattening. Accept only genuine row-packed YUV 4:4:4 or 4:2:0 with three planes and actual 8/10/12-bit precision; 10/12-bit values remain little-endian 16-bit. For 4:2:0 calculate chroma dimensions with checked ceil division; 4:4:4 uses full geometry. Reject noncanonical stride, row count, unexpected plane count, odd length, sample above coded depth, overflow or total decoded length above `256 MiB`. Use no unchecked multiplication for source-derived dimensions.
5. For the **R05 raw coded-plane claim**, set `color_model=yuv`, `has_alpha=false`, `is_grid=false`, `is_sequence=false`, and **`orientation=unverified`**. This expressly **does not attest a display-orientation transformation**; if a transform is detectable but its safe semantics cannot be proved, refuse. All alpha-and-transform fixture inputs must refuse as out-of-scope, not be accepted as V4. Nothing downstream may turn this interim observation into publication. Unknown essential transform/coding instructions must not be silently accepted as proven display facts.
6. Compute actual source and *each* actual plane SHA-256, and compute `media_worker_output_sha256_v1`'s documented `ordinal(u32 LE) || stride(u64 LE) || rows(u64 LE) || length(u64 LE) || raw bytes` framing from those buffers. Build the entire bounded canonical V1 JSON claim and payload **before writing any stdout**, so decode refusal produces no partial published-looking data.

The current `graphic_image_inspection.rs` still-image stdout limit is `MAX_IMAGE_DECODED_BYTES_V1 + 4096` and its legacy line ceiling is 1,024 bytes. Respect a **1,024-byte maximum for the new AVIF JSON header** and the strict overall 64 KiB V1 envelope ceiling; a worker emitting too large a header must refuse. Output one UTF-8 V1 16-field JSON document on the first line **then exactly the concatenated Y/U/V bytes**. The exact closed fields of the *existing* parser are:

`version, status, format, mime, source_length, source_sha256, width, height, bit_depth, color_model, has_alpha, is_grid, is_sequence, orientation, planes, output_sha256`.

Each `planes` entry carries exactly `ordinal, stride, rows, length, sha256`. The required tokens are `1`, `decoded`, `avif`, `image/avif`, `yuv`, `unverified`; booleans/integers must be real JSON types. Never print an extra diagnostic, JSON alias, additional envelope field, trailing byte or secret. The current strict `parse_media_worker_envelope_v1` must remain byte-identical.

### 3.2 The parent must truly own acceptance

Implement `ConfinedAvifInspectAdapterV1` using the current inward `MediaWorkerPortV1::inspect_avif(request)`; bind only an already-trusted fixed `solo-cli` executable (no authored program path). `MediaWorkerRequestV1::for_source` applies the source length bound **before spawn**. Use the existing `run_confined_image_worker_v1` supervisor with literal `"avif"` and the parent's computed source digest. Parse **one exact** newline-framed JSON header with unchanged `parse_media_worker_envelope_v1`; never parse a general stream/second message. The parent must calculate the exact sum of claimed plane lengths with checked arithmetic and split the received stdout into **exactly** the claimed three plane buffers. Reject zero, short, extra or truncated output, forged source digest, invalid UTF-8 header, unknown/duplicate/missing JSON key or bad numeric shape. No worker success from metadata alone.

The actual `ImageWorkerRunV1` supplies stdout bytes, child PID, wall_ms, exit code, limits_verified, cleanup_verified and stderr length. Refuse if any host fact is not established; do not claim independent descendant attestation beyond what the existing supervisor actually verifies. Pass only honestly derived host facts into `MediaWorkerObservationV1`, then call the inward `inspect_media_worker_source_set_v1` verifier over **the complete acquired obligation set**.

Strengthen **only if necessary** `check_observation_v1` in `solo-application/src/media_worker.rs` to enforce width/height and decoded pixel product, three-plane YUV 4:2:0 or 4:4:4 **canonical** row layouts derived from width/height/depth (not trusted stride/rows), real 16-bit sample lengths, ordered plane ordinals, equal U/V shapes, no alpha/grid/sequence, and aggregate bounded output rehash. The existing V1 protocol carries **no explicit chroma-subsampling token**: the parent must independently recognize *only* the two admitted complete layouts from measured geometry, not pretend to prove unknown colour metadata; 1×1 is naturally layout-indistinguishable. A plain source/plane digest is not an independent test oracle, so the R04 pinned per-plane comparisons remain mandatory for the four fixtures. If an old V1 test expects a value that cannot safely satisfy the stronger check, preserve that test, diagnose it and change only the authorized validator/test assertion that corrects a genuine defect; never weaken acceptance.

The `MediaWorkerCheckedEnvelopeV1` is inert validation evidence. It does **not** prove display orientation, correctness of all media classes, a publication-ready result, sibling isolation or global source snapshot, and it grants **no** publication, routing, filesystem/network-sandboxing or asset-library authority. Preserve the V3 `avif-unsupported-v3` tests untouched.

## 4. Focused, real host acceptance — TWO N121 cases, not another probe

Implement exactly these two new named integration cases in `crates/solo-cli/tests/media_worker_host_smoke.rs`, using the real `BIN` compiled `solo-cli`, real confined `ContentRootCapability`, existing `Root`/`inventory_of` helpers and actual `MediaWorkerPortV1` + inward parent boundary. The agent must **not** stop after changing only the existing Rust golden assertions.

1. `n121_01_four_coded_plane_goldens_cross_worker_and_parent`: Four immutable AVIF sources produce **four distinct child PIDs**, each normal exit under the fixed limits/reap and each with complete plane bytes transmitted through stdout and rechecked by the trusted parent. Assert source SHA-256, width/height/8/10/12-bit, per-plane SHA-256, strides/rows and the independently pinned R03 test goldens for all four; assert `orientation=unverified` and publication authority `false`. The Rust codec comparison test alone does not qualify. Reuse R04's exact test fixture names:
   - `white_1x1.avif`
   - `fox-profile0.8bpc.yuv420.avif`
   - `fox-profile0.10bpc.yuv420.avif`
   - `fox-profile2.12bpc.yuv420.avif`
2. `n121_02_real_worker_and_hostile_claim_refusals`: The real child/parent refuses empty and over-20-MiB input **before launch**, non-AVIF magic, truncation, malformed code, an alpha/rotation fixture and both R03 negative alpha/transform fixtures without partial or display-ready result; direct unconfined worker invocation must not decode. At the parent boundary exercise one forged source digest, plane digest, aggregate digest, invalid stride/rows, wrong coded depth, missing/trailing byte, wrong exit status, false limits and false cleanup. Use current in-memory constructed hostile V1 observations as needed for forged metadata, **but real positives and real malformed-media checks must use the actual worker**. Do not add a general test-mode shell/command, alter native host code, or replace unsafe gates.

For each case emit an independently checked single `n121_evidence case=01 ...` or `case=02 ...` line containing actual observations, not mere successful print tokens. Test-oracle comparisons and source fixture reads are fixed test-only paths. `reference-goldens.json`, `reference_oracle.py`, per-source fixture bytes, and their licences/provenance remain unchanged. Where WSL has no Btrfs `SOLO_TEST_SOURCE_FS_ROOT`, report those local native source tests as NOT RUN LOCALLY and still execute host-independent product/parent tests. Ubuntu Actions must use the retained real disposable Btrfs fixture, Windows its genuine native reader.

## 5. Real CI gate, local-first discipline, refusal truth

Run before each technical push:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-cli --test media_worker_host_smoke --locked n121_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

The actual Rust test target is **`check-architecture`**, with a hyphen. The technical graph must report `architecture=legal packages=4 targets=33 files=64 edges=93` if no unapproved source/module change occurred. Run the retained N102, N116, snapshot and component checks where actual local prerequisites exist. Missing local Btrfs fixture is explicitly environmental, never an invented green; Ubuntu hosted CI exercises it for real. Do not use `#[ignore]`, `continue-on-error`, filters that hide old failures, retries with a false result, loosened limits, `|| true` around required commands, or a newly enabled dependency feature.

Edit the existing `.github/workflows/rust-verify.yml` to **run exactly the two N121 cases as a load-bearing gate on each native host** (Linux Bash and Windows PowerShell) using `cargo test -p solo-cli --test media_worker_host_smoke --locked n121_ -- --nocapture --test-threads=1`. Require exactly `test result: ok. 2 passed; 0 failed; 0 ignored` plus each fully qualified test name exactly once, the two actual `n121_evidence` tokens, and no failure/ignored case. Reuse N116's interleaving-safe checker pattern; prove at least **one altered copy of the authentic test log is rejected** (remove one case or corrupt its evidence) by that same checker. Do not print `n121_gate authentic=accepted` until the real checker accepted that real test run; do not print `n121_gate mutation=rejected` unless the corrupted copy was actually rejected. Preserve the branch-scoped `avif-reference-oracle` job as a third mandatory job. Keep all older N093/N102/N108/N113/N116, RustSec, cargo-deny, architecture, strict clippy, snapshot, source/unsafe and clean-reproduction gates intact.

A fresh **branch push** at one exact final T13 SHA must produce success for all three jobs (Ubuntu verify, Windows verify, `avif-reference-oracle`) and the genuine N121 2/2 evidence in both verification logs. A red or skipped job on that SHA is failure. N093 and N102 previously showed intermittent refusals; do not waive them, suppress logs or call a failed CI run passed. Record every failed attempt; only a **fresh wholly green** attempt of the same final technical SHA qualifies. Unexplained recurrence is a blocker for acceptance, not retrospective evidence that the worker is safe.

## 6. Report and done condition

Overwrite only `.documents/Reports/execution-report.md` on `main`, never create an archive. Record full A13, T13.0, final T13 and R13 chat-only SHA, exact changed-file ledger, all local commands and exit codes, exact real source/plane/aggregate hashes for the four fixtures on both native hosts, worker child PIDs and fixed limits/reap proof, refusal tokens, 2/2 N121 tests, mutated-log rejection, retained architecture/unsafe/N093/N102/N116/security gates, oracle pin verification, final CI run/attempt/head SHA and three job IDs, R13 baseline Ubuntu/Windows results, plus one worktree / 28 refs / exactly four WIP / empty stash / clean checkout / task scratch cleanup.

`status=COMPLETE` for `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1` is allowed **only** after the runtime adapter and child exist, the inward parent actually accepts and rechecks the four confined results, both native-host N121 tests are green with actual exit/pixel evidence, and the entire three-job technical CI passes at one unchanged SHA. Anything less is **`status=STOPPED`**, with concrete blocked source or test, not a euphemism for preparatory progress.

**After R05:** finish V4 display-oriented alpha/rotation and complete all-owner six-format N118 negative matrix, integrate exact verified code into `main` under a separate work order, then deliver the version-pinned lawful `platform/assets/` catalogue, privacy-safe derived media and the Stage 03 whole-candidate exit matrix. **Do not close R03.C or Stage 03 in R05.**

**Execute the runtime product slice now.**
