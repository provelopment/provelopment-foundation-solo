# SOLO-WO-118 R08 — INDEPENDENTLY ADJUDICATE ASSOCIATED-ALPHA IROT BEFORE ANY DECODER REPAIR

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Single ACTIVE:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-ASSOCIATED-ALPHA-REFERENCE-AND-DECODER-ROOT-CAUSE-ADJUDICATED`.

## 1. Binding architectural decision

`SOLO-WO-118 R07` is **REVISE / status=STOPPED** for its incomplete N122 requirement. It revealed a credible, *not yet independently hosted-reproduced*, associated-alpha disagreement. The Chief Master Architect reviewed R15's exact report, R03's independent oracle, the original AOMedia/libavif fixture README and the pinned `oxideav-avif v0.0.16` implementation. **Do not change the R03 goldens, accept arbitrary tolerance, use the invalid `NOirot` fixture as a positive, or infer that a successful codec decode proves correct display semantics.** The valid positive remains `abc_color_irot_alpha_irot.avif`; the negative with no alpha `irot` association stays invalid by media policy even if its rendered hash coincides with a reference.

There is a concrete suspected mechanism to adjudicate: versioned `oxideav-avif` `src/decoder.rs` `decode_item_output_inner` decodes an alpha auxiliary recursively with its own item transforms, calls `alpha_as_gray` to fit it to the primary coded extent, attaches it, then applies the primary item's transforms to the combined image (around upstream pinned source lines 839–883 and 1299–1331). With essential `Irot{angle:1}` on *both* primary and alpha, this can rotate/resample alpha twice. That **is a hypothesis, not a licensed change to the codec**. Independent libavif `avifdec` produced an **unrotated** 512×256 PNG from the positive in R03; the separately pinned 256×512 display digest was computed by rotating **all** its decoded RGBA bytes. Whether this is correct alpha handling requires adversarial confirmation, not changing the reference to match another decoder.

**Deliver this R08 as a focused reproducible technical finding:** remeasure *both* known fixtures with the same independently pinned Ubuntu `avifdec`/dav1d/Pillow tools, compare *both* raw reference RGBA buffers and their whole-image anti-clockwise rotations, reproduce *both* Rust decoder results on both native hosts, enumerate the exact ten sample differences and their spatial coordinates, trace the upstream double-transform mechanism, and publish a falsifiable, evidence-backed correction recommendation. If independent results prove the R03 positive's pin erroneous, present the specific evidence but **do not repin in this order**. If they corroborate the R03 positive, identify the minimum credible decoder correction and its code/licence/graph risks for the next Owner-authorized product order. This order is an **adjudication prerequisite**, **not N122 implementation**, an alpha-enabled product promise, V4 or Stage 03 completion.

Preserve the accepted N121 confined coded-plane worker, legacy V3 AVIF refusal, all source-reader/host protections and security gates. No new third-party implementation dependency or new process, local vendor fork, binary artifact in Git, runtime fallback, transcoding, network fetch in product or rewrite of user media.

## 2. Verified starting heads, single checkout and transaction

Require exact remote heads **before** any write:

- `origin/main` = `ee447a33c17108418e2c2c9f48f457affbc3f993` (R15 canonical STOPPED report), parent A15 = `830d57ffe8eea2e9e13659e2bcb94dba8b2fc16d`.
- `origin/solo-wo-118-avif-v4-r01` = `405b11c92568f8a8d5943dc4be3b5d36193db219` (T15.0 ACTIVE-only), parent T14 final = `81a7bf55f238f231f3fcdd41f6adf628b4a25e97`.
- Accepted graph: four packages, ten normal adapter roots, 53 reachable locked packages, `edges=93` on branch and `edges=81` on main. No new dependency or `Cargo.lock` change.
- Last accepted T14 run `43709995330`: Ubuntu `132799270408`, Windows `132799270213`, reference job `132799270515`, all green; R15 baseline run `43712508222`: Ubuntu and Windows green. **Neither proves an alpha fix.**

Use the existing *single* Owner checkout, one worktree, one origin, empty stash, no extraneous dirty files, exactly four WIP files and the existing **28** remote heads. Verify `git status`, `git worktree list`, `git stash list`, `git ls-remote --heads origin` and the HEADs. No clone, new branch, remote, worktree, PR, tag, merge, rebase, force push, `reset --hard` or indiscriminate `git clean`. Preserve all protected refs and Owner files.

1. **A16 (`main`)** — after Owner installs this ACTIVE, commit and push **only** `.documents/WIP/workorder-active.md` from clean R15. Verify working/index/commit/origin Git blob SHA exactly. Record full A16 SHA, tree, parent and blob.
2. **T16.0 (existing AVIF branch)** — from clean verified T15.0, restore only the A16 ACTIVE Git blob with `git restore --source="$A16" --staged --worktree -- .documents/WIP/workorder-active.md`. Assert index blob equals A16 blob, commit/push ACTIVE-only T16.0. Do not import the old R07 order.
3. **T16 technical** — add the specific test-only referee (§3), run local host-independent gates, push the evidence branch. A **fresh** branch-push Actions run at one exact T16 SHA must pass `verify (ubuntu-24.04)`, `verify (windows-2025)` and the **extended** `avif-reference-oracle` as three independent jobs. Record the actual tool identities, test logs, job IDs and attempted failures, not mere green badges. Correct forward if a test fails without relaxing requirements.
4. **R16 (`main`)** — return to clean main at A16, overwrite/commit/push **only** `.documents/Reports/execution-report.md`, stating the genuinely reproduced facts and their limits. Verify fresh R16 Ubuntu and Windows baseline jobs. Give R16 full SHA in chat only, not its own report.

The Architect has GitHub **read-only** access; only the Owner-activated Implementation Agent writes or pushes the repository. Scratch outputs belong solely to task-created paths under `$RUNNER_TEMP` or locally under task-owned `.temp/`, with bounded cleanup. No write to `main` product paths.

## 3. Exact authorized permanent technical ledger

```text
.documents/WIP/workorder-active.md                           # A16 byte-identical import only
.github/workflows/rust-verify.yml                            # extend existing oracle job; retain existing jobs
crates/solo-cli/tests/fixtures/avif/alpha_transform_referee.py  # NEW, independent test-only referee
crates/solo-adapters/tests/graphic_avif_inspection.rs         # extend R04 golden test with n123 diagnostic
```

No `reference-goldens.json`, `reference_oracle.py`, fixture/provenance bytes, `Cargo.lock`, crate manifest, codec source, vendor directory, image-worker production source, media-worker application, protocol, host, native reader, CLI, domain grammar, architecture checker, existing N093–N122 tests/expected values, source/unsafe scanner, `deny.toml` or other WIP/roadmap/SOP/architecture changes. No new git-tracked image, RGBA dump, reconstructed PNG, binary or secret. Do not alter existing oracle PASS semantics or skip/waive any failed job.

### 3.1 Independent reference-side adversarial comparison

Inside the existing `avif-reference-oracle` job, retain the exact pinned tool installation, source SHA, hash, version and environment verification and unchanged R03 manifest/mutation steps. The **same** pinned `avifdec --codec dav1d --jobs 1 --depth 8` tool shall now also decode the *negative* `abc_color_irot_alpha_NOirot.avif` into its **own** task-owned PNG. It already decodes the valid positive to `$RUNNER_TEMP/measured/abc_color_irot_alpha_irot.avif.png`. Do not overwrite either, and do not call the negative a valid reference positive.

The new `alpha_transform_referee.py` must be Python 3 stdlib plus the **already pinned** `/usr/bin/python3` Pillow, closed command/arity, deterministic canonical JSON, with no network or repository writes. It takes the fixed manifest path and the two **explicit** reference-PNG paths plus their corresponding original AVIF paths; prevent aliasing the two inputs, enforce exact byte-source hashes for both (positive from `fixtures`, negative from `negatives`), and fail closed for source mismatch, PNG invalidity, unexpected geometry, overflow, missing alpha or malformed expectations. Inspect manifest's real schema before accessing its keys; do not invent fields. Recompute the positive PNG SHA, raw coded RGBA SHA, 512×256 dimensions, alpha min/max/non-opaque count, and the 256×512 whole-buffer 90° anticlockwise rotated RGBA SHA. Require exact match to the **immutable** R03 positive pin. Verify `rotation_inverse(rotation(pixels)) == pixels` without image heuristics. Independently compute and record the **negative** coded RGBA digest, alpha statistics, rotated RGBA digest, 131,072-sample alpha differences relative to the positive in both coded and rotated coordinates, and an ordered bounded coordinate inventory of all changed alpha positions. Compare colour RGB channels separately from alpha; count and label each disagreement. Do **not** require positive and negative independent PNG to match; the evidence must show whether they do.

Report literal machine-checkable lines after real checks only: `n123_reference positive_pin=matched ...`, `n123_reference negative_measured=true ...`, `n123_reference same_coded_rgba=<true|false> alpha_differences=<integer> ...`, `n123_reference cleanup=complete`. The referee's pass means **accurate measurement** of the two independent reference outputs, not that the source is V4-compatible. Deliberate in-memory or `$RUNNER_TEMP` changes to copied positive manifest/source digest and expected RGBA digest must produce nonzero referee refusal; the authentic sources/manifest are never mutated. Require `n123_reference mutation_rejections=2/2` from actual result codes. The existing `avif-reference-oracle` job must still report the R03 `pristine_manifest_matches=true` and `mutation_rejections=3/3` unchanged. Cleanup any newly created referee outputs in a guaranteed `always()` step, without deleting existing oracle tools before they are used.

### 3.2 Rust-side reproducible differential — host-independent

Add one test named `n123_associated_alpha_irot_decoder_differential_is_measured` to **existing** `crates/solo-adapters/tests/graphic_avif_inspection.rs`. Preserve its two accepted R04 tests verbatim in semantics. From the two pinned AVIF source fixtures, verify source hashes against the manifest, call *exactly the locked decoder* (`oxideav-avif =0.0.16`), read actual decoded geometry/format, full Y/U/V/A planes, separate per-plane SHA-256 and fallible `try_to_rgba8()`. Compare the two outputs pixel by pixel: number, bounded positions and values of differing **alpha** samples; separately count any RGB differences. Print exact computed positive/negative RGBA digests and nonopaque counts, 4-plane digests, geometry and the actual relationship of each digest to the immutable R03 positive pin. Determine and explain if alpha appears to have been transformed twice, particularly whether `decode_item_output(... alpha_id ...)` acts on `Irot{angle:1}` before `alpha_as_gray` resizes and `decode_item_output(... primary ...)` applies another `Irot{angle:1}`. Reference the actual installed registry sources with path + version + function and lines in the execution report. **Do not change the decoder, alter the raw sample or make an `assert_eq!` to endorse the negative fixture as a positive.**

If R15's reported results recur, the expected *diagnostic* is positive `a03a44d4ed770e3f9448889ec3df6577d86ffce02f893309d09629de4a4690fa`, negative `950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`, valid positive 123,962 vs negative 123,952 nonopaque alpha samples, exactly 10 differing samples and no different RGB values. That is a **reproduction target**, not a pretext to edit the checker: if actual results differ, print and report the disagreement and diagnose it. The test must assert input identities, structural correctness and accurate enumeration; the full report must preserve any contradictory data. It must not grant media correctness or publication authority.

On **both Ubuntu and Windows native** `verify` jobs, add one small load-bearing `cargo test -p solo-adapters --test graphic_avif_inspection --locked n123_ -- --nocapture` step after normal test setup, with explicit exit check and one real assertion line `n123_decoder measured=true` from the test. Require exactly `1 passed, 0 failed, 0 ignored`, and reject at least one corrupted copy of the real captured log (deleted test line/evidence) using the same checker before printing `n123_gate authentic=accepted` and `n123_gate mutation=rejected`. This test needs no Btrfs source root; it reads fixed files and decodes in process, separately from the retained N121 real-worker tests.

### 3.3 Required decision record — no auto-fix

In R16 report make a structured **evidence table** for each positive and negative fixture: source SHA, container transform/essential flags, independent PNG coded dimensions/hash, independent coded RGBA digest, independently rotated RGBA digest, alpha statistics, Rust decoded geometry, three colour-plane SHA values, alpha-plane SHA, Rust RGBA digest and alpha count, whether colour/alpha matched independently, and changed-alpha coordinate count. Distinguish `avifdec`'s unrotated PNG output from oriented display facts. Quote the original pinned AOMedia/libavif donor `tests/data/README.md` distinction: valid correctly associated alpha `irot` versus invalid missing-alpha-`irot` (inspect at donor commit `37a5461042aa3b1eb105193724ee41bfd2478d0b`). Use **exact** pinned v0.0.16 upstream paths/functions to trace any double transform, crop/resample, possible colour-conversion alternative, and whether the two AVIF sources contain identical coded image and alpha payloads; independently check this rather than assuming because file lengths differ by one byte.

If and only if evidence supports it, recommend one *specific* correct-by-construction repair in the product dependency/adapter boundary (which frame is transformed once, whether the attachment should be made before a single primary transform, whether any resize is avoidable) and a test that would **fail before repair** and pass **after**. Identify the exact minimum next-order file paths, any source vendoring/forking change, Cargo.lock/architecture closure impact, license/MIT obligations and two-host N122 validation. If independent evidence instead demonstrates that R03's source-mapped reference is incorrect, identify exact proof and a *separate* Owner-reviewed reference correction procedure; **do not edit the reference in R08**. Never claim both contradictory explanations resolved if they are not.

## 4. Local-first, hosted acceptance and report

Before each technical push run:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked --no-run
```

Existing `n121` native source-read tests require the verified disposable Btrfs `SOLO_TEST_SOURCE_FS_ROOT` on Linux; run those in native CI, and record local lack as **NOT RUN LOCALLY**. Windows-native linkage without `link.exe` is likewise NOT RUN LOCALLY, not invented success. The real workspace test, N093/N102/N108/N113/N116/N121 gates, cargo-deny/RustSec, snapshot/architecture/unsafe/clean-tripwire checks and three-job pipeline must remain intact. CI at one final technical SHA must be wholly green on both hosts and the extended pinned oracle, including authentic plus corrupt-log checks. Do not weaken the architecture checker or change its expected `packages=4 targets=34 files=66 edges=93` branch measurement unless an authorized extra file genuinely changes it; this order adds only a Python script and Rust test in existing file, so the number should remain stable.

`status=COMPLETE` **only for** `R03C-AVIF-ASSOCIATED-ALPHA-REFERENCE-AND-DECODER-ROOT-CAUSE-ADJUDICATED` after: both independent PNGs measured against pinned tool identity, positive matches unchanged R03 reference; positive and negative Rust outputs reproduced on native Ubuntu and Windows; coordinate-precise alpha delta and causal source trace documented; genuine test-log mutation/manifest refusal proof; full three-job CI green at one technical SHA; canonical report published and fresh R16 main baseline hosts green. If genuine independent disagreement remains, report it and make a narrower supported decision rather than manufacturing parity. A diagnostic `COMPLETE` **does not mean R07's N122 or V4 is complete**. Stop if the independently pinned tool is unavailable, any required security gate stays red, or there is an unapproved necessity to alter product code to adjudicate; record the concrete barrier.

**After R08:** the Architect issues a separately authorized minimal decoder/adapter repair or reference correction order based on the independently tested causal finding, then resumes the original N122 real confined alpha + rotation proof, six-format V4 integration, immutable lawful Platform 1.0.0 catalogue and whole-candidate Stage 03 closure. None is waived by a diagnostic result.

**Execute the independent adversarial referee; preserve every previously accepted gate.**
