# SOLO-WO-118 R06 — PROVE THE EXISTING CONFINED AVIF WORKER ON BOTH NATIVE HOSTS; NO SOURCE-READER CHANGE

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Single executable ACTIVE:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1-N121-ACCEPTED`.

## 1. Independent Architect review, correction and mission

`SOLO-WO-118 R05` is **REVISE**, not complete. The verified technical branch contains the *real* AVIF worker and parent adapter, added at T13.1 `e580b25250fd5717949326a279a414edcef16007`; branch run `43706976479` passed Ubuntu 24.04 `132790054004`, Windows 2025 `132790053715` and independent AVIF reference-oracle job `132790053944`. Those retained green checks establish build/regression integrity, **not** live AVIF worker acceptance. The two required real-host N121 cases and their CI gate were not committed. The canonical R13 report honestly calls `status=STOPPED`.

**Correct the specific erroneous diagnosis in R13.** `crates/solo-domain/src/property_registry.rs` already declares `ACCEPTED_IMAGE_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "webp", "avif", "gif", "svg"]` (lines 631–632 at T13.1); `crates/solo-application/src/graphic_asset_read.rs::EntityAssetPathV1::from_reference` uses that very constant and never contains an AVIF-specific denial. The existing *passing* `crates/solo-application/tests/graphic_asset_read.rs::r03c2_09_portable_boundary_refuses_prohibited_spellings_without_repair` explicitly admits `x/y.avif` (lines 863–876). **Do not add AVIF to the extension list again. Do not change the domain parser, source reader, native confinement or any guard.**

The observed `GraphicAssetReadRefusalKind::Unsupported` is **not** an unsupported-extension diagnostic; that spelling fails with `UnsafeAssetReference`. `Unsupported` is obtained when the existing `ContentRootCapability::read_entity_asset` maps `NativeError::Unsupported`, including an unsupported immutable source-read mechanism. The retained Ubuntu test helpers explicitly require `SOLO_TEST_SOURCE_FS_ROOT` naming a verified snapshot-capable Btrfs filesystem; on the Owner's WSL machine this prerequisite is reported absent. **The local missing Btrfs environment is not evidence that `.avif` is denied.** The Actions `verify (ubuntu-24.04)` job already has an `R03 snapshot-test filesystems (Ubuntu only, disposable)` step *before* the complete workspace tests and N116 worker tests; Windows 2025 uses its real approved native source reader. Run the actual N121 cases on both supported hosted environments. If a hosted read still returns `Unsupported`, collect the *real* mechanism, fixture-root, setup and native-refusal facts and STOP safely; do not invent an alternative reader, bypass the immutable-copy constraint, or silently skip the case.

**Mission:** Restore and finish exactly two N121 real-host cases for four already independently pinned AVIF coded-plane positives, strong negative/forgery refusals, and a genuinely load-bearing two-host CI log verifier with a deliberate mutation that turns green to red. Test the existing `ConfinedAvifInspectAdapterV1` plus `inspect_media_worker_source_set_v1` through real `read_graphic_asset_bytes_v1(inventory, &root)` and real supervised `solo-cli` child processes on the actual native hosts. Repair **only demonstrated defects** inside this order's tiny ledger. Consume unchanged R03 pinned `reference-goldens.json` and R04 golden assertions. R05 proved the child only compiled; this order must prove it *runs and is independently checked*.

**Invariants:** coded Y/U/V planes, correct 8/10/12-bit samples, **orientation=unverified**, no alpha, no sequence/grid, no publication authority. This is **not** full six-format V4, alpha/orientation support, Platform catalogue, `R03.C` completion, Stage 03 closure, new routes, publishing, release or deployment. The accepted five-format V3 retains its AVIF refusal. Do not reinterpret an 'unverified' orientation as an accepted display transform. Preserve source ownership, complete EFS five-level obligations, private/internal/suppressed checks, immutable source byte acquisition and all no-fallback rules.

### Exact starting facts; verify before any write

- `origin/main`: **`81eb8cc376bee48bbbf497566af16d595e36845f`** (R13 STOPPED report). Its ACTIVE-only predecessor is A13 `dbba90c9df9c1d58417fbb567d4b126cdb4b07e9`.
- Existing sole AVIF technical branch `solo-wo-118-avif-v4-r01`: **`e580b25250fd5717949326a279a414edcef16007`** (T13.1). Its ACTIVE-only predecessor is `14f300a2b691bb53f8c2d9eb0a420fdc48c0adbd`.
- Technical dependency is already authorized: `oxideav-avif = "=0.0.16"`, normal roots `[10]`, transitive closure `[53]`, `edges=93`; no new dependency/version/feature, no `*-sys`, no scanner/license override.
- Exact technical source changes in T13.1: new `crates/solo-adapters/src/graphic_avif_inspection.rs`; edits only to `graphic_image_inspection.rs`, adapter `lib.rs`, application `media_worker.rs`. The R04 8/10/12-bit tests and the R03 independently generated reference oracle are preserved. Current technical architecture measurement: `architecture=legal packages=4 targets=34 files=66 edges=93`.
- Existing **28** remote heads, one checkout/worktree and origin, four canonical WIP files, one overwritten canonical report, clean checkout/stash in R13 report. Preserve the SVG technical branch `solo-wo-115-avif-svg-v3-r02` at `d82ae76273b57b37014d3b4c557b8debd5299eea` and old `ad7b74df8638ff56c7137f33d9242da54583f17b` / `40b03c73b0bc8544359438daea6218caa496e85d` refs.

Read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, binding `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G/I/K, 6A.8, both SOPs, all four WIP files, last canonical report and complete relevant sources before action. **Chief Master Architect has GitHub read-only authority only; the Owner-activated Implementation Agent alone may commit/push.**

## 2. Single-checkout A14 → T14.0 → T14 → R14 transaction

Inspect in the **one** existing Windows 11 / WSL Ubuntu 26.04 checkout:

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
git ls-files '.documents/WIP/*'
```

Require exact expected heads and one `origin` named `https://github.com/provelopment/provelopment-foundation-solo` (optional `.git`), empty stash, exactly four WIP files (`plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, `workorder-active.md`), no unexpected Owner edits and the single original checkout. The **only** permitted initial dirty exception is Owner-installed replacement ACTIVE on `main`. No new checkout, worktree, technical branch, stash, extra WIP/report archive, force push, rebase, destructive reset, bulk cleanup, `git clean`, unapproved ref mutation or off-repo edit. Only task-owned `.temp/` files may be safely cleaned.

1. **A14 / `main`:** require `HEAD=81eb8cc376bee48bbbf497566af16d595e36845f`. Commit/push **only** installed `.documents/WIP/workorder-active.md`. Verify exact blob equality across working file, index, new A14 commit and `origin/main`; report full A14 SHA/tree/parent. No product/report edit at this stage.
2. **T14.0 / existing branch:** switch from clean pushed A14 to `solo-wo-118-avif-v4-r01`, requiring `HEAD=e580b25250fd5717949326a279a414edcef16007` and clean tree. Propagate the *exact new* ACTIVE blob from A14 by single-path:
   ```bash
   git restore --source="$A14" --staged --worktree -- .documents/WIP/workorder-active.md
   test "$(git rev-parse ':.documents/WIP/workorder-active.md')" = "$(git rev-parse "$A14:.documents/WIP/workorder-active.md")"
   test "$(git diff --cached --name-only)" = '.documents/WIP/workorder-active.md'
   ```
   Commit/push this ACTIVE-only checkpoint. Verify that the old R05 ACTIVE is not restored over R06.
3. **T14 / same branch:** restore and complete exactly §3's two native test cases, implement §4's two-host CI evidence gate, then run the local-first and hosted checks. Product code should remain byte-identical to T13.1 **unless a new, demonstrated real failing test requires an authorized fix**. Use ordinary forward technical commits and re-run relevant local checks before every push; do not merge technical changes into `main` in this order. Dispatch the **branch push** event at a single final full T14 SHA so its three-job oracle+Linux+Windows suite runs; the oracle job is push-scoped and may be skipped by `workflow_dispatch` alone.
4. **R14 / `main`:** after genuine technical result is measured, return to the now-clean A14 `main`, overwrite/commit/push **only** `.documents/Reports/execution-report.md`; reverify a new native two-host baseline run at full R14 SHA. Give complete R14 SHA in final agent chat, not inside its own report. If the two N121 tests or a mandatory gate are red, report `status=STOPPED` with detailed honest evidence rather than hiding or disabling any case.

## 3. Strict small edit ledger; actual N121 host tests

**Permanently authorized technical-branch paths only:**

```text
.documents/WIP/workorder-active.md                 # exact A14 blob ONLY
crates/solo-cli/tests/media_worker_host_smoke.rs   # two N121 real-host cases and test-local helpers
.github/workflows/rust-verify.yml                   # load-bearing two-host N121 gate
crates/solo-adapters/src/graphic_avif_inspection.rs # only to fix an observed R06 worker/port defect
crates/solo-adapters/src/graphic_image_inspection.rs# only to fix an observed dispatch defect
crates/solo-adapters/src/lib.rs                     # only for an observed narrow export defect
crates/solo-application/src/media_worker.rs        # only to correct an observed parent-verifier defect
crates/solo-application/tests/media_worker.rs      # only for relevant parent regression assertions
```

**Strictly immutable:** `crates/solo-domain/src/property_registry.rs`, `crates/solo-application/src/graphic_asset_read.rs`, `crates/solo-adapters/src/source_read.rs`, `crates/solo-adapters/src/source_read_native.rs`, `crates/solo-adapters/src/media_worker_host.rs`, `media_worker_host_native.rs`, `media_worker_protocol.rs`, `Cargo.lock`, all Cargo manifests, architecture/source/unsafe/workspace guard code, `.documents/Architecture`, SOPs, roadmap, root, `solo-cli/src/main.rs`, R03 reference Python/JSON/provenance, all binary fixtures, N093/N102/N108/N113/N116 original assertions, `deny.toml`. No new dependency or alternative I/O path. On `main` only A14 ACTIVE and R14 report. The extension grammar is **already correct** and must not be modified.

### N121-01: exact four positive real-child/parent results

Add exactly the native integration test named **`n121_01_four_coded_plane_goldens_cross_worker_and_parent`** under a test module in `crates/solo-cli/tests/media_worker_host_smoke.rs`. Construct the real validated complete inventory from the accepted `test_inventory_v1`/`TestOwnerV1` helpers; each of the four distinctly authored Entity/Facet/Site/Page/Representation-owner obligations must refer to the corresponding pre-existing AVIF bytes in the one task-owned `assets/` root. Five-level ownership remains validated; exactly **four** positive AVIF obligations means use four **distinct owners**, not a fifth invented or duplicated fixture. For example use Entity/Facet/Site/Page, then cover Representation separately in N121-02. Do not use a private constructor or fabricated `GraphicAssetSourceSetV1` to bypass acquisition.

Use `ContentRootCapability` and the **real** `solo_application::read_graphic_asset_bytes_v1(&inventory, &root)` (with the capability as the `GraphicAssetReadPort`). Confirm the result is `Complete` and contains the four byte-identical obligations; if the result is `Unsupported`, record the actual native host and filesystem mechanism rather than attributing it to `.avif` grammar. Then use real `ConfinedAvifInspectAdapterV1` with `MediaWorkerPortV1::inspect_avif` and inward `inspect_media_worker_source_set_v1` over the complete set. Each actual confined execution must have distinct positive child PID, normal exit, verified OS limits, wall-deadline evidence, confirmed cleanup, exact typed source length/digest, complete three-plane byte buffers, canonical width/height and 8/10/12 depth, 4:4:4/4:2:0 strides and rows, per-plane hashes and aggregate digest matching the **unchanged** independent R03 fixture golden. The parent must produce four checked records in authored inventory order, no partial set or published value. `orientation=unverified`, `has_alpha=false`, `is_grid=false`, `is_sequence=false` are **unverified-geometry coded-plane** claims, not display orientation or publication.

You may wrap the actual adapter in a **test-only recording port** that forwards each call and retains a clone of the *actual returned observation and actual plane bytes* before the inward validator consumes it, so one four-obligation pass proves both received pixels and final checked records. Do not construct fake positives or infer the child output from a precomputed fixture hash. The four test inputs are exactly:

```text
white_1x1.avif
fox-profile0.8bpc.yuv420.avif
fox-profile0.10bpc.yuv420.avif
fox-profile2.12bpc.yuv420.avif
```

These names are test-only fixture selectors; no production allowlist or source hash special-casing is permitted. For tests on Linux, create the task-owned root strictly **under `SOLO_TEST_SOURCE_FS_ROOT`**, the verified Btrfs root provisioned by CI; on Windows use the current approved runner temp helper and native source reader. The existing `Root` helper in the N097 and N116 integration code documents this OS split; reuse its semantics without modifying the accepted tests or deleting foreign directories. Assert root provenance and containment, and safe cleanup of test-owned roots.

### N121-02: negative/forgery and private obligation matrix

Add exactly **`n121_02_real_worker_and_hostile_claim_refusals`**. Construct a real `assets/` root under the same trusted test filesystem. Require actual, non-skipped refusals for: a malformed AVIF (truncation and incorrect `ftyp`/magic), zero source bytes, an over-20-MiB source, a declared `graphic.favicon` AVIF forbidden by the PNG-only property role, the alpha+rotation positive-for-later-V4 AVIF, the missing-alpha-transform association negative, the nonessential `clap/irot/imir` negative, and at least one later **internal or descendant-suppressed** authored obligation whose bad bytes discard the complete earlier success. Include a representation-level obligation in this case. Platform references must remain refused at the source-acquisition boundary; no fallback to the Platform namespace, another owner or another media format. Not all listed failures happen at the same boundary: explicitly state whether refusal came from authored property grammar, source acquisition, child preflight, process host or inward parent verifier; don't mislabel a source-reader size refusal as worker decoding.

For forged metadata create **test-local counterfeit V1 claims/host observations** against a genuinely acquired source set (or use the existing inward validator negative constructors) and assert the documented typed refusal for source digest, plane or aggregate SHA-256, swapped U/V layout where observable, wrong depth, invalid stride/rows, short/trailing payload, missing/duplicate/unknown envelope JSON key, bad exit, false limits, false cleanup and timeout. A test-local counterfeit may prove **inward refusal rules**, but all authentic positives and source-policy negatives must execute the actual child where applicable. Compare to the existing strict 16-field parser **unchanged**. Never claim that rehashing a counterfeit claim is an independent semantic decoder oracle, or that the plain coded-plane V1 contract proves transformed-display correctness. A direct call to the reserved worker without correct OS confinement must refuse before media parsing.

**Important inherited V1 compatibility:** existing inward tests deliberately preserve `MediaWorkerOrientationV1::Transposed` as a **claim only** (`crates/solo-application/tests/media_worker.rs`), not a display-correct statement. Do **not** globally ban it, change the V1 interpretation or weaken that retained test to make R06 pass. The actual R05 AVIF worker must continue reporting `Unverified`. Existing V3 AVIF refusal (`avif-unsupported-v3`) and all accepted PNG/JPEG/WebP/GIF/SVG behaviors remain true.

The two cases must each output one `n121_evidence case=01 ...` / `n121_evidence case=02 ...` record **after** their real assertions, describing real PIDs, dimensions/digests and categories, without source content, paths, credentials or third-party attribution omitted. Test output is evidence only; the CI parser must verify that the tests actually passed, not count fabricated print markers.

## 4. Load-bearing hosted verification and honest local-first behavior

Add an N121 section to the **existing** `rust-verify.yml` `verify` job, after the `R03 snapshot-test filesystems` setup and before cleanup on Ubuntu; use Windows' existing native source capabilities. Run exactly the two N121 tests **with the real full integration test target** on each host:

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked n121_ -- --nocapture --test-threads=1
```

Use Bash and PowerShell host-specific commands as needed. Capture real stdout+stderr with propagated exit code, and require `2 passed; 0 failed; 0 ignored`, the **two full test names exactly once**, and independently asserted `n121_evidence case=01` and `case=02` emitted after the actual real-worker checks. Detect any extra unexpected `n121_` tests, duplicate rows, `FAILED`, `ignored`, missing evidence or missing complete test-result line. Follow N116's existing native log checker, not a grep that could accidentally count commands or inactive prose as successful tests. Have the **same log verifier** reject at least one modified scratch copy of the authentic log, such as deleting a case or corrupting a verified evidence token; show `n121_gate authentic=accepted` **only after** the real log passed and `n121_gate mutation=rejected` **only after** the modified log failed. Do not alter the genuine log in place. The mutation is a deliberate test of the checker and must not become a false hosted red result.

On the Owner's WSL Linux machine **without** `SOLO_TEST_SOURCE_FS_ROOT` a real-source N121 positive **cannot** pass; that is a documented **NOT RUN LOCALLY** case. Do **not** install a fallback reader, accept an unproven filesystem, set the variable to an arbitrary directory, fake a source set, or pass a targeted `--skip`/`#[ignore]` as proof. Instead genuinely perform host-independent local commands before technical pushes:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked --no-run
```

If an actual verified Btrfs fixture already exists on WSL, use it and run N121 locally too; do not assume it exists. On the Owner's Windows shell where `link.exe` is unavailable, record native local NOT RUN LOCALLY and obtain actual Windows-host proof from Actions. It is acceptable to report an attempted full local workspace test as red **because the missing Btrfs fixture**; never call that a passing suite. The **hosted `cargo test --workspace --all-targets --locked --no-fail-fast` must run every original test plus both N121 cases without skips**. Retain and verify all N093/N102/N108/N113/N116 cases, 23+ architecture negative fixtures, RustSec/cargo-deny scans, both `edges=93` assertions, the unsafe/source guards and exact workflow graph. No softened assertions, ignore filters, `continue-on-error`, `|| true` on mandatory steps, retry of one failed case as if that made the entire job green, or bypass of the immutable source-read mechanism.

The single final T14 branch push must yield three **successful** jobs on the same full `head_sha`: `verify (ubuntu-24.04)`, `verify (windows-2025)` and `avif-reference-oracle`, with N121 **2/2 real pass** and mutated-log refusal on **both** verify hosts. The R03 independent oracle must confirm pristine pinned manifest and `mutation_rejections=3/3`; neither N093 nor N102 instability may be waived. If a red intermediate attempt is fixed within the ledger by a forward commit, report it and require a genuinely new **all-green** final attempt at the new exact technical SHA. If hosted native `Unsupported` persists, stop with precise `NativeError`/capability observations, not guessed `.avif` grammar issues.

## 5. Report, independent acceptance and hard limits

At R14 publish one canonical `.documents/Reports/execution-report.md` on `main`, `status=COMPLETE` **only if** both N121 cases and the whole three-job technical run are proven at one exact SHA. Report:

- Full A14, T14.0, final T14 commit SHA/tree/parent and R14 full SHA **in chat only**, Git blobs and exact changed-file ledger; verify one origin/worktree, clean tree/stash, 28 remote heads, exactly four WIP files and no task leftovers.
- A factually corrected analysis of `ACCEPTED_IMAGE_EXTENSIONS` and the former local `Unsupported`, including actual Btrfs/native source mechanisms from the **passing** Ubuntu and Windows host logs; do not claim the source-reader contract was widened or that AVIF was missing from the grammar.
- Four source identities, source lengths, bit depths, plane geometry and **per-plane and aggregate SHA-256** compared to R03 goldens; distinct observed PIDs, real host limit and cleanup evidence, four parent-checked records, exact complete-set and non-publication result. Mask host paths/environment details; no fixture bytes or secrets in diagnostics.
- Actual N121 **2/2** and `n121_gate` accepted/mutation-refused tokens from Ubuntu and Windows, plus each meaningful negative typed refusal; `avif-reference-oracle` pristine pin and `3/3` mutation refusals; all local results, Windows/WSL environmental limitations and any red attempted jobs recorded honestly.
- Exact final branch CI run ID/attempt/head SHA/three job IDs and conclusions; fresh R14 main Ubuntu/Windows baseline run ID and outcomes. R14 baseline still does **not** integrate the AVIF technical source; avoid conflating main baseline and technical acceptance.

**Stop conditions:** any need to modify a strictly immutable path, another dependency, a production source-read fallback, changed security/architecture rules, unverified native worker completion, mismatched decoded reference bytes, unknown extra file/ref, a missing native-host guarantee, or failing mandatory test. No workaround by lowering gates. If all acceptance checks are green, `status=COMPLETE` qualifies the **bounded coded-plane AVIF worker milestone only**, not V4 or Stage 03.

After R06: independently design/prove alpha+orientation V4 and full six-format whole-candidate N118 matrix, integrate exact trusted technical source into `main` under separate authority, deliver the pinned lawful `platform/assets/` library, derived/public privacy discipline and final Stage 03 exit matrix. **No publication or Stage 03 closure in this work order.**

**EXECUTE THE NATIVE WORKER PROOF, NOT AN EXTENSION-LIST PATCH.**
