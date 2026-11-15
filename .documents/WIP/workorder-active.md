# SOLO-WO-118 R03 — PIN INDEPENDENT AVIF PIXEL ORACLES; CHARACTERIZE RETAINED N093 REFUSAL

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Single ACTIVE path:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-INDEPENDENT-REFERENCE-ORACLE-PINNED-N093-TRUTHFULLY-CHARACTERIZED`.

## 1. Mission, scope correction, immutable starting facts

Deliver **one bounded prerequisite** to the outstanding AVIF V4 capability: independent, reproducible 8/10/12-bit AVIF pixel/reference facts, with a strict test-only generator, provenance, a load-bearing recheck and a separately recorded N093 diagnostic. This order deliberately **does not** attempt the V4 child codec, application port, N118 seven-case product suite or Stage 03 closure. R01 and R02 attempted all those independent steps in one run and stopped twice; do not repeat that execution pattern. A successful R03 provides the exact reference data that the next V4 code-bearing order can use without guessing pixel results or AVIF transforms.

**Pinned read-only evidence:** `origin/main=3310b7aaf91ec911dc7a9c7f4e315f46cd94bb44` (R10 report), current AVIF technical branch `solo-wo-118-avif-v4-r01=68eda9337d66737e07494d95418017047b16be57` (T10A), R02 owner authority `61cf7dbc22ae696d2d7d4a53f6d88d3049e6c727`, T9 source `3d52df2051ef8c4d0b4bb8194d8f2539b0997c71`. T10A already provides exact approved `APPROVED_ADAPTER_EXTERNAL_NORMAL[10]`, adapter closure `[53]`, `edges=93`, `oxideav-avif =0.0.16` and the genuine exploratory decode test. Preserve them byte-for-byte except the precise R03 test/provenance additions explicitly allowed below. There are **28** existing remote branch heads; add none. Previous green full baseline: R9 Actions `43680718664`. R10 Actions `43683197657` is **NOT GREEN** on Ubuntu.

The exact R10 Ubuntu failure was a real refusal: `n093_01_the_real_parent_terminates_its_owned_group_and_observes_the_descendant` returned exit 3, `HOST_DESCENDANT_UNVERIFIED`, at `crates/solo-cli/tests/media_worker_host_smoke.rs:1553`; Windows passed. Source in `crates/solo-adapters/src/media_worker_host.rs:3711–3761` distinguishes `/proc` running, zombie, gone and recycled identity; the refusal cannot be called a pass, a proven resource leak or a proven harmless flake. **Ruling:** retain the N093 gate intact, reproduce/characterize its host observation separately, do not weaken, skip, ignore, lengthen or bypass it, and never present that red run as technical V4 acceptance. An oracle job may pass independently while the full verification job remains red; the report must say so explicitly.

Before acting read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G/I/K, 6A.8, both SOPs, all four WIP documents, the current one execution report, and the exact current `rust-verify.yml` and AVIF fixture provenance. The one-Entity/EFS, source-confinement, current runtime/worker bounds and all first-party `unsafe` restrictions remain binding. The Chief Master Architect only reads GitHub; the activated Implementation Agent owns permitted Git changes.

## 2. Safe forward-only Git choreography

Use only the current verified Windows 11 / WSL Ubuntu 26.04 checkout and the existing AVIF branch; never clone, create a worktree/branch, reset, rebase, force-push, cherry-pick, merge, delete unfamiliar files, `git clean`, or create another report/WIP/archive. Start on `main` with exactly the Owner-installed replacement ACTIVE as the sole intended local change; verify root, remote, index, `git status --porcelain=v1`, `git stash list`, `git worktree list --porcelain`, all remote heads and pinned tips. One `origin`; one worktree; empty stash; exactly four WIP files; protect all remote refs. On mismatch STOP safely and report.

1. **A11** — commit/push **only** the Owner-installed `.documents/WIP/workorder-active.md` on `main`, starting from exact `3310b7aaf91ec911dc7a9c7f4e315f46cd94bb44`. Verify working-tree, index, committed and remote Git blob equality.
2. **T11.0** — switch the existing technical branch only after `main` is clean and A11 remote-confirmed. Assert starting branch `HEAD=68eda9337d66737e07494d95418017047b16be57`. Restore **only** `.documents/WIP/workorder-active.md` from full A11 SHA into the clean branch index/worktree, assert `A11:path == :path`, commit/push that one path. Never restore the stale R02 order from technical ancestry.
3. **T11.1 oracle discovery** — add the strict test-only reference parser and conditional Ubuntu oracle job defined in §§3–4, committing only allowed paths. Run local parser self-tests and retained local tests before pushing. Push normally to the existing branch; the branch-push run must perform the independent oracle discovery and record package, library, pixel and orientation facts. This **discovery run is not final oracle acceptance**.
4. **T11.2 exact pins** — fill in measured, noninvented output hashes, source identities, toolchain/package identities and rights in `reference-goldens.json` and the existing provenance Markdown; change the reference job from discovery to **pin-enforcing verification** and add the mutation sensitivity checks. Run locally, commit/push allowed paths, and inspect the next actual branch-run job logs. Continue with forward corrections to the same paths until the **oracle job passes** at a single exact technical SHA. Do not revise goldens to match `oxideav` or skip a mismatching fixture.
5. **R11** — from a clean tree switch to `main` at A11; overwrite/commit/push **only** `.documents/Reports/execution-report.md` with exact oracle and N093/full CI results. Record R11 SHA in the chat handoff, never inside its own report. Observe the R11 baseline CI, and report its real result even if N093 makes it red. No product implementation or technical-branch integration into main in this order.

Preflight/verification commands (run in the approved checkout; all command exit codes recorded):

```bash
set -euo pipefail
git rev-parse --show-toplevel
git branch --show-current
git rev-parse HEAD
git remote -v
git status --porcelain=v1
git stash list
git worktree list --porcelain
git ls-remote --heads origin
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
```

Host-dependent snapshot tests without a prepared Btrfs `SOLO_TEST_SOURCE_FS_ROOT` must be recorded **NOT RUN LOCALLY**, never silently ignored. No newly red test may be edited out. A real surprise or branch/ref mismatch is STOP, not a bypass.

## 3. Closed permanent-write ledger and independent reference contract

**On `main` only:** `.documents/WIP/workorder-active.md` (A11) and `.documents/Reports/execution-report.md` (R11). **On the existing technical branch only:**

```text
.documents/WIP/workorder-active.md                                 # byte-identical A11 import only
.github/workflows/rust-verify.yml                                    # additive branch oracle job; preserve all existing gates
crates/solo-cli/tests/fixtures/avif/reference_oracle.py             # NEW, test-only Python standard-library parser
crates/solo-cli/tests/fixtures/avif/reference-goldens.json          # NEW, pinned machine-checkable facts
crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md       # existing documentation; append verified evidence
```

Do **not** modify `Cargo.lock`, any Rust source, the exploratory decode probe, dependency allowlists, `media_worker_host.rs`, N093/N116 test source, `Cargo.toml`, root toolchain, `deny.toml`, source/unsafe checker, existing seven `.avif` binaries, SVG fixtures, any other WIP file, the accepted Stage 03 architecture or any other file. Preserve `edges=93` in the technical workflow, `edges=81` in main; preserve all existing Linux/Windows full gates. No production dependency or test-only native library may enter Cargo. No oracle binary or unlicensed derivative media may be committed. No provider, PR, release, tag or deployment.

### Reference tool: *independent* of `oxideav`

Use **Ubuntu 24.04** test-only `libavif-bin` / `avifdec` with `dav1d` as the AV1 codec. Ubuntu Noble publishes `libavif-bin` under `universe` (distribution package reference: `https://packages.ubuntu.com/noble/libavif-bin`; distribution catalogued `1.0.4-1ubuntu3` as of this review, **not an assumed live runner version**). `avifdec` supports `--codec dav1d`, `--jobs 1`, Y4M output and PNG output per the upstream documentation: `https://github.com/AOMediaCodec/libavif/blob/main/doc/avifdec.1.md`. Inspect the *actual runner binary's* `--help` before invoking flags; do not assume `--size-limit` or `--dimension-limit` exists in that version.

An authorized test-only Ubuntu job may use networked APT for `libavif-bin` and `/usr/bin/python3`'s `python3-pil`; first discover installed/available exact versions, then use apt's **exact version selection** for final CI and read-back verification. Record both package/archive SHA-256 (download the pinned `.deb` to `$RUNNER_TEMP` where obtainable), `avifdec` full-path SHA-256, `avifdec --version`, `ldd` output plus hashes and package versions of actual `libavif`/`libdav1d` libraries, `/usr/bin/python3 --version`, the Pillow package version and license, the checkout SHA, source AVIF fixture hashes and the exact command line. If the available runner lacks a supported `dav1d` codec or cannot establish provenance, fail this oracle job and report the missing tool; never substitute `oxideav` or install anything on the rootless Owner WSL host. No floating runner result may be used as the final pinned oracle.

**Two distinct reference domains — never conflate them:**

- **8/10/12-bit raw YUV plane oracles**: call `avifdec --codec dav1d --jobs 1 <input.avif> <RUNNER_TEMP>/<fixture>.y4m` for white 1×1, fox 8, fox 10 and fox 12. Y4M retains coded depth; parse this independently with `reference_oracle.py` and hash **exact plane pixel bytes** (Y, U, V) rather than the Y4M container. Record width, height, chroma (`420` or `444` as actually observed), depth, each plane's dimensions, byte count, SHA-256 and an ordered typed aggregate digest; preserve 10/12-bit little-endian 16-bit samples. One frame only: reject missing/multiple `FRAME` records and trailing garbage. Any mismatch from the known fox `1204×800` or bits `8/10/12` is a refusal.
- **Display orientation and alpha**: `avifdec --codec dav1d --jobs 1 --depth 8 <abc_color_irot_alpha_irot.avif> <RUNNER_TEMP>/positive.png`; use pinned `/usr/bin/python3` + Pillow to inspect *decoded RGBA pixels*, assert reported display dimensions **256×512**, calculate SHA-256 of `Image.open(png).convert('RGBA').tobytes()` in row-major order, and record alpha minimum/maximum and count of non-opaque alpha samples. Record both the original PNG file SHA-256 and the **RGBA byte digest**; these are different hashes. Confirm orientation with the PNG display geometry plus the independently inspected AVIF transform association; **Y4M alone is not an orientation oracle**, and a transposed raw plane digest does not prove correct display coordinates. For the `NOirot` and `clap_irot_imir_non_essential` test-only negatives, record their exact source hashes and the *normative refusal reason from the pinned upstream README and Architecture*; optional reference decoder output never promotes an invalid-policy source to positive.

### Required `reference_oracle.py` behavior

Only Python standard library for Y4M parsing/hashing/JSON and self-tests; Pillow import is confined to its `png` command. Define closed commands `y4m <path>`, `png <path>` and `self-test`. Reject unknown options, malformed headers, zero/overbound dimensions, unrecognized chroma/depth, noncanonical frame delimiters, excess/trailing bytes, and unsafe larger-than-checked allocations. Read at most a checked bounded file size, not an unbounded stream. Parse `YUV4MPEG2` ASCII header and exact `FRAME\n`, require **exactly one frame**; allow only documented 420/444 depth 8/10/12 representations actually emitted. For 420, chroma width/height are `ceil(W/2)`/`ceil(H/2)`; for 444, full W/H; sample width is 1 byte for 8-bit or 2 bytes for 10/12-bit, retaining raw little-endian order. Calculate checked total bytes before slicing; hash each Y/U/V plane individually and hash a deterministic aggregate with an explicitly documented unambiguous framing (do **not** copy the product's own aggregate as the oracle). For high depth, reject decoded sample values above `(1<<depth)-1` and lengths that do not match stride×rows. Emit deterministic canonical sorted-key JSON with fixture name, source Git/raw SHA-256, exact tool identity and per-plane results; PNG command emits width, height, mode, RGBA digest and alpha statistics. Unit `self-test` must cover one valid constructed 420 sample, one 10-bit sample, one wrong sample-depth value, truncated/extra frame data, invalid geometry, wrong chroma, swapped plane bytes and a deliberately corrupted expected digest. No source-based hash list masquerading as a pixel oracle.

**Committed `reference-goldens.json`** is a strictly validated, versioned, closed-key JSON manifest of real reference results (schema identifier `solo-avif-oracle-v1`, fixture filename, raw source SHA, 8/10/12-bit facts, per-plane byte digests, independent RGBA facts and tool identities). Test-only and non-authoritative for publication. The final job must **recompute** all evidence from pinned source bytes and pinned reference binary and fail on any difference. Verify a clean manifest positively and prove **at least three mutated manifests** (source SHA changed, plane SHA changed, orientation/RGBA digest changed) are rejected by the same checker; no bypass or `continue-on-error`. Do not write the golden from the verifier in its acceptance mode.

## 4. CI implementation — additive, no hidden verification skip

Modify existing `.github/workflows/rust-verify.yml` as follows:

1. Retain the existing `verify` matrix **unchanged** as to all commands, checks and conclusions. Extend the `push.branches` list **only** to `[main, solo-wo-118-avif-v4-r01]`, so ordinary branch pushes reliably run the actual file at the branch SHA without relying on a `workflow_dispatch` input that may not exist on the default-branch version. Preserve scheduled/main and pull-request behavior, read-only token, offline Cargo fetch boundary, RustSec negative gate, full workspace tests, N093/N116, architecture `edges=93` on the technical branch, test tripwire and reproducibility.
2. Add one **separate** job `avif-reference-oracle` with `if: github.ref == 'refs/heads/solo-wo-118-avif-v4-r01' && github.event_name == 'push'`, `runs-on: ubuntu-24.04`, `timeout-minutes: 15`, `permissions: contents: read`. Reuse the **same pinned `actions/checkout` SHA** as `verify`; set `persist-credentials: false`, no secrets, no external actions and no write permissions. Place all installation, Y4M/PNG outputs, downloaded package archives and temporary evidence in `$RUNNER_TEMP`, clean task-owned material with a guaranteed final cleanup step; do not upload binaries or create an archive.
3. The first branch run collects **discovery** outputs with `sudo apt-get update`, `sudo apt-get install -y --no-install-recommends libavif-bin python3-pil`, captures actual identity and runs reference extraction + self-tests; the submitted report must mark this run **discovery-only**, never as an accepted pinned test.
4. In T11.2, replace floating installs by **exact-version** APT selection and explicit `sha256sum`/package readbacks for every final oracle tool/codec. Compare regenerated canonical results with `reference-goldens.json`, prove the three mutation refusals and the parser negatives. A new branch push runs a fresh **pin-enforced** oracle job. Require `github.sha` to equal the full technical commit identified in the report. A host mismatch, new package version, changed pixel hash, unknown colour encoding or absent proof fails the oracle job rather than regenerating the pinned file.
5. Keep `verify` and `avif-reference-oracle` conclusions separate in the report. A failed `verify` run is **not** technical green, even if `avif-reference-oracle` passes; it also does not turn independently measured oracle bytes into fabricated data. Never claim that an overall failed workflow run is a successful V4 technical verification.

**N093 characterization:** after every hosted run inspect `verify (ubuntu-24.04)` and report its actual N093 result. If `HOST_DESCENDANT_UNVERIFIED` recurs, capture the exact command, stderr, run/job ID, elapsed time and observed `/proc` identity facts that the existing code actually makes available; do not speculate about cause. In this order make **no** edits to N093 tests or native worker source, **no** test-filter or ignore/conditional skip in full verification, **no** extra timeout, **no** suppression. A rerun on the same SHA may be used once for diagnostic comparison; disclose both runs. Repeated failure remains an OPEN separate corrective work-order item. The next V4 technical completion will require full two-host `verify` green at one exact SHA, N093 included, regardless of this oracle-only result.

## 5. Local-first acceptance; honest reporting

Run the listed local Rust checks before each branch push, plus:

```bash
python3 crates/solo-cli/tests/fixtures/avif/reference_oracle.py self-test
# Once the actual reference tool is available in CI, exercise 'y4m' and 'png'
# only on real independent outputs, not on the candidate decoder's output.
```

**R03 scope `COMPLETE` only when:** the existing technical branch contains the exactly issued ACTIVE blob; test-only parser self-tests and deliberate-invalid cases pass; the `avif-reference-oracle` job runs on **real Ubuntu 24.04** at an exact T11 SHA, rechecks pinned `avifdec`+dav1d/Pillow package and binary provenance, regenerates independent Y/U/V and alpha/rotated RGBA hashes, matches the committed golden manifest and rejects all three mutations; all source fixture hashes match unchanged Git blobs; the report discloses true `verify` job results and unresolved N093 status. No V4 adapter, V4 unit suite, N118 or production AVIF claim is permitted by this scope. A `verify` failure is **an explicit OPEN CI blocker**, not silently waived and not a reason to mislabel the independent oracle gate's actual outcome.

**R03 scope `STOPPED`** if its own source/parser/oracle/tool-provenance/host/pin gate fails, if any permanent edit escapes the ledger, or if trust and exact revision cannot be proved. Do not treat `avifdec` output without verified binary and independent plane extraction as a golden. Do not claim the pinned tool as a shipped product dependency. All prior and future Stage 03 obligations remain open.

Overwrite the **one** canonical `.documents/Reports/execution-report.md` on `main` with exact A11/T11/R11 SHAs, parents/trees, changed paths, local command exits, discovered and pinned package hashes/licences, per-fixture dimensions/depth/chroma/plane digests, PNG/RGBA orientation/alpha facts, CI run/job IDs, per-job outcomes, mutation proof, N093 exception analysis, all failed/ignored/skipped cases and temporary cleanup. No second report, no archives, no branch merges. Report R11 SHA in chat only. A clean index/tree and full remote ref inventory must be explicitly recorded, not presumed.

**Hard exclusions:** do not implement V4 production decoding, change role/fallback/visibility rules, integrate technical code into `main`, create platform catalogue assets, close `R03.C`, claim whole-publication/HTTP delivery, release, deploy, create PR, waive N093, modify locked core architecture or assert Stage 03 acceptance. The next Owner-issued technical order will consume these independently pinned facts as its explicit code/test oracle.
