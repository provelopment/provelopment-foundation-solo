# SOLO-WO-118 R07 — COMPLETE THE CONFINED AVIF ASSOCIATED-ALPHA + IROT DISPLAY-SEMANTICS SLICE

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's single local ACTIVE:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-ASSOCIATED-ALPHA-IROT-DISPLAY-PIXELS-TWO-HOST-N122-VERIFIED`.

## 1. Architectural decision, actual progress and strict scope

Chief Master Architect independently **ACCEPTED** `SOLO-WO-118 R06` as the completed **coded-plane-only** milestone. This is NOT V4 acceptance. At the verified technical SHA `81a7bf55f238f231f3fcdd41f6adf628b4a25e97`, native Ubuntu 24.04 and Windows 2025 each ran real N121 2/2; the independently pinned R03 `avifdec`/dav1d oracle passed in the same branch push run `43709995330` (Ubuntu job `132799270408`, Windows `132799270213`, oracle `132799270515`). The canonical R14 report is `93832aa04c94522110cfda341ac8fa46874343e6` on `main`; its Ubuntu and Windows baseline run `43710838725` succeeded. The previously reported AVIF extension denial was erroneous: `ACCEPTED_IMAGE_EXTENSIONS` already includes `avif`; `NativeError::Unsupported` on the local non-Btrfs source filesystem was a mechanism refusal. Preserve the domain grammar and the real native reader byte-for-byte.

**This R07 deliverable:** extend the real confined `avif` worker and the trusted parent's **inert V1 media-worker check** for precisely one additional valid media class: **8-bit YUVA 4:4:4 with an essential, correctly associated 90° anticlockwise `irot` (angle 1)**. The one pinned positive is `abc_color_irot_alpha_irot.avif`, from the accepted libavif fixtures. Prove actual source-byte→confined-child→four actual oriented planes→parent rehash and geometry→independent **display-oriented RGBA** oracle, with native Ubuntu/Windows N122 cases. Refuse `abc_color_irot_alpha_NOirot.avif` and `clap_irot_imir_non_essential.avif` **for their actual item-property defects even though the raw decoder can yield four planes**. Maintain existing four N121 non-alpha 8/10/12-bit positives, their coded `orientation=unverified`, existing exact plane oracles, all trusted OS limits and all previously accepted tests.

**Do not overclaim.** R07 is *one tested associated-alpha orientation profile*, not all AVIF transforms, alpha formats, colour conversion classes, 10/12-bit alpha, grids, AVIS, the full six-format V4 application port, `R03.C`, Stage 03, Platform library, publication, routes, release or deployment. The already accepted V3 five-format API **must retain** its `AvifUnsupported` refusal, and no R07 result grants publication authority. **Do not merge any technical product change into `main`.** No media file may be accepted by filename, fixed source hash, extension alone, an environment override, or a test-only trusted digest: admissibility is determined from bounded, parsed, actual source bytes inside the confined child.

### 1.1 Exact pinned third-party feasibility surface (no dependency discovery needed)

Inspect the **already locked** `oxideav-avif =0.0.16` source in local Cargo registry before editing, not the moving `master` branch. The versioned public APIs expose `oxideav_avif::parse_header(&source)` with `hdr.meta.primary_item_id`, `hdr.meta.assoc_by_id(id)`, `hdr.meta.properties`/`Property::{Irot,Imir,Clap}`, `PropertyAssociation {index, essential}`, `oxideav_avif::find_alpha_item_id(&hdr.meta, primary)`, `oxideav_avif::inspect::inspect(&source)` and actual decoded frames through `oxideav_avif::decode(&source)`; `AvifImage::try_to_rgba8()` is fallible and must not silently substitute a fallback conversion. Versioned implementation runs the item's transformation pipeline **after** alpha attachment in a decoded primary. **Do not assume this implementation's alpha/rotation result is correct simply because `decode()` succeeded.** The accepted independent reference must settle pixel correctness.

R03's **immutable** `crates/solo-cli/tests/fixtures/avif/reference-goldens.json` states:

- Valid positive source SHA-256 `b371cc88244a873131e4d10ff9363d71ce4f41cf333bd4a491b38d970d9abd3b`; `abc_color_irot_alpha_irot.avif`.
- Independent `avifdec` PNG is **unrotated coded 512×256**; coded RGBA SHA-256 `5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82`.
- Independently audited `irot=1` anti-clockwise rotation of the **entire** reference RGBA gives **display 256×512**, `rgba_display_sha256=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`.
- Alpha has range `128..255`, exactly `123952` non-opaque samples. These are independent test references, **never** publication authority or production pixel caches.
- Invalid negatives: `abc_color_irot_alpha_NOirot.avif` SHA-256 `f2c8cd6ded641c68d13b3363417a62288a5eb335870de8d0b9da5093865ffb9a` (missing alpha transform association); `clap_irot_imir_non_essential.avif` SHA-256 `33f869fcf2a879913eb394982b8fc03e9a60c25831aa37622ddefa656fd39fc1` (non-essential `clap`/`irot`/`imir` transforms). Decode success is **not** authorization for these negative cases.

No modification of these goldens, reference Python, donor provenance, fixture bytes or pinned `libavif-bin`/dav1d/Pillow identity is permitted. If the candidate `try_to_rgba8()` disagrees with the independently pinned oriented RGBA SHA, report the actual computed hash and affected class; investigate precise colour-conversion/alpha/orientation differences **without re-pinning the independent oracle**. This order requires a real independent **colour AND alpha** orientation proof; **a self-consistent candidate hash, mere 256×512 geometry or an unrotated Y4M digest does not satisfy it**. If exact byte parity proves unattainable under the allowed product changes, STOP with the real discrepancy rather than calling it a pass.

## 2. Repository authority and exact A15 → T15.0 → T15 → R15 transaction

**Verified starting heads** (check full SHA again just before writing):

- `origin/main` = **`93832aa04c94522110cfda341ac8fa46874343e6`** (R14 canonical ACCEPTABLE report), whose ACTIVE-only parent lineage includes A14 `a11a9042b58509e6cba6b57e05c740bdb8a52491`.
- Existing AVIF branch **`solo-wo-118-avif-v4-r01`** = **`81a7bf55f238f231f3fcdd41f6adf628b4a25e97`**; T14.0 ACTIVE import `bc95cc69c028872b2c4ce332229d1c16b7ac0f87`; superseded red technical checkpoint `53ab18ce7751f5b6a83cbcd7557de36e6ac52619` remains in history, not re-run as a valid pass.
- Branch `oxideav-avif=0.0.16`, external adapter roots `[10]`, closure `[53]`, expected architecture baseline `architecture=legal packages=4 targets=34 files=66 edges=93`; existing seven test-only AVIF binaries; R03 and R04 fixed reference assertions; N121 gate + its log-mutation proof.
- 28 remote heads; one original checkout/worktree, one `origin`, no stash, **exactly four** WIP files. Preserve the accepted SVG technical head `d82ae76273b57b37014d3b4c557b8debd5299eea` and the `ad7b74df` / `40b03c73` protected historical refs without changing them.

Before any write, read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, the binding `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I, 6.2K and 6A.8, both Architect/Implementation SOPs, the four WIP files, the canonical report and all affected Rust/CI code. Use **one** existing Windows 11 / WSL Ubuntu 26.04 checkout; do not clone, worktree, make a new branch, merge, force push, rebase, reset destructively, stash, `git clean`, delete unrelated files, create PRs or tag/release/deploy.

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

1. **A15 main:** on `main` at the verified R14 full SHA, require only the Owner-installed new `.documents/WIP/workorder-active.md` dirty. Commit/push **only** ACTIVE. Confirm the exact Git blob in working tree, index, commit, `origin/main`; record A15 full SHA/tree/parent. No product or report change.
2. **T15.0 branch:** switch to existing branch, require its exact T14 final SHA and clean tree. Copy **only** A15's ACTIVE via:
   ```bash
   git restore --source="$A15" --staged --worktree -- .documents/WIP/workorder-active.md
   test "$(git rev-parse ':.documents/WIP/workorder-active.md')" = "$(git rev-parse "$A15:.documents/WIP/workorder-active.md")"
   test "$(git diff --cached --name-only)" = '.documents/WIP/workorder-active.md'
   ```
   Commit/push one ACTIVE-only branch checkpoint. No R06 ACTIVE import and no cherry-pick/merge.
3. **T15 technical:** on this same branch, make **production code changes** in §3, then real host tests in §4, then CI gate in §5. Before every technical push run host-independent local Rust/architecture checks. Push ordinary forward technical commits; no new branch. The **final** branch push must produce `verify (ubuntu-24.04)`, `verify (windows-2025)` and `avif-reference-oracle` success **at one exact full T15 SHA**, with the new N122 test and mutated-log evidence on both native hosts. The reference job is branch-`push` scoped; `workflow_dispatch` alone does not count.
4. **R15 main:** only after measuring the technical outcome, switch to now-clean main at A15 and overwrite/commit/push **only** `.documents/Reports/execution-report.md`. Verify fresh Ubuntu/Windows main baseline CI. Give R15 full SHA in the handoff chat rather than writing its self-SHA in its own report. `status=STOPPED` is legitimate only with a real, named code/test/host/oracle failure or unapproved essential scope; do not STOP at another diagnostic/test-only increment, or because local WSL lacks Btrfs. Do not self-accept Stage 03.

**Chief Master Architect's GitHub access is read-only; Owner-activated Implementation Agent alone writes and pushes.** Do not make a file edit outside the ledger in §3. Task scratch belongs only in job-owned `$RUNNER_TEMP` or local task-created `.temp/` and must be removed without touching other material.

## 3. Bounded permanent technical file ledger and real product functionality

```text
.documents/WIP/workorder-active.md                    # exact A15 blob only
crates/solo-adapters/src/graphic_avif_inspection.rs    # essential-transform policy + YUVA child planes
crates/solo-application/src/media_worker.rs            # parent canonical YUVA geometry and inert checked facts
crates/solo-application/tests/media_worker.rs          # additive YUVA/claim/refusal tests
crates/solo-adapters/tests/graphic_avif_inspection.rs  # additive pinned independent RGBA pixel parity
crates/solo-cli/tests/media_worker_host_smoke.rs       # exactly three new N122 real-host tests
.github/workflows/rust-verify.yml                       # additive two-host N122 gate
```

No `Cargo.toml`, `Cargo.lock`, dependency graph/root/closure/checker, `deny.toml`, codec pin, `media_worker_host.rs`, `media_worker_host_native.rs`, `media_worker_protocol.rs`, `source_read.rs`, `source_read_native.rs`, `property_registry.rs`, source/property grammar, `solo-cli/src/main.rs`, reference goldens/Python/provenance, fixture binaries, N093/N102/N108/N113/N116/N121 source semantics, other WIP/SOP/architecture or asset/library files. The existing closed `avif` token already dispatches correctly; do not introduce a second token or a new executable, `unsafe`, native process API, network access, fallback reader or media fetch. The V1 protocol remains the **same exact sixteen fields**.

### 3.1 Prove the item-transform policy **inside the confined child, before pixel decode**

Extend the existing `decode_checked_still_v1` only for the one bounded alpha/irot profile, while preserving its existing no-alpha 4:4:4/4:2:0, 8/10/12 path. The child must still check OS confinement **first**, bound stdin to `20 MiB + 1`, verify actual source SHA, and inspect the *real* AVIF bytes under that same confinement. In the typed `parse_header`/`Meta` tree, prove rather than assume:

- A legitimate still-image AVIF brand, resolvable one primary `av01` image item with mandatory `av1C`, not `avis`, grid or other unsupported derived primary, no unsupported essential property on the elected primary or alpha item; declared width/depth and actual decoded plane geometry remain within 8192 per side, 32,000,000 total pixels and 256 MiB decoded bytes by checked arithmetic, **including geometry after 90° rotation**. Never allocate decoded pixel buffers on an unbounded, unverified source. Retain 512 MiB address/commit, 10 CPU seconds, 15 s wall and verified reap.
- **Exactly one** appropriate alpha auxiliary attached by the inspected `auxl`/`auxC` relation to that primary, not a free-floating or replacement auxiliary; `pixi`/`av1C` bit depths and alpha dimensional/format compatibility genuinely verified. Refuse a missing, duplicate, wrong-depth, undecodable or unassociated alpha item, rather than flatten to opaque.
- Enumerate `ipma` property associations for **both** the primary and its alpha item. Require a correctly associated, **essential** `Irot {angle: 1}` on the primary and the corresponding alpha image item; require the same geometric transform semantics on colour and alpha. Explicitly refuse `NOirot` even if `oxideav_avif::decode` returns four planes. Explicitly refuse the `clap_irot_imir_non_essential` fixture because its claimed transformative properties are **non-essential**, even if a decoder applies them. Refuse unexpected `clap`, `imir`, unknown/unsupported essential transforms, different transform order, missing alpha association, and extra transform combinations rather than silently accepting them. Use `PropertyAssociation.essential` and `Property::{Irot,Imir,Clap}` from **0.0.16**, not raw filename or a heuristic byte search.
- The existing codec applies item transformations itself. **Do not blindly rotate it again.** Confirm actual returned geometry **256×512** and actual four *display-oriented* planes Y/U/V/A at 8-bit 4:4:4, with each plane canonical stride/rows/byte-length `256 × 512` (131,072 bytes) and actual sample range. For the admissible alpha profile, return `color_model=yuva`, `has_alpha=true`, `is_grid=false`, `is_sequence=false`, `orientation=identity` **only when** the final planes are independently shown to be displayed correctly. For the four unchanged plain stills retain `color_model=yuv`, `orientation=unverified`. No other alpha/transform class becomes accepted by implication.
- Build all claims and all four actual plane byte buffers, compute each SHA-256 and the exact existing V1 aggregate digest, render **one** bounded UTF-8 16-key JSON line followed by **only** the actual Y/U/V/A bytes, and do not write any stdout on a refusal. Keep the existing 1,024-byte header ceiling, exact receive-length checks and the still-worker stdout cap. No extra print, key, untrusted path, environment switch or decoded-pixel cache.

**Safety/decoder discrepancy:** The pinned 0.0.16 decoder may decode a source while not respecting the transform association policy. The product's independent validation must refuse such a source before treating it as an admissible transform. Any source that cannot be proven satisfies the entire declared policy refuses; never turn the uncertainty into an `unverified` *successful display*.

### 3.2 Trusted-parent V1 checks remain *checks of claims*, not media correctness

Keep the unchanged strict `parse_media_worker_envelope_v1` (16 root keys, exact four plane keys) and the existing `ConfinedAvifInspectAdapterV1` real supervisor. Update **only** the inward `check_observation_v1` so that, for `color_model=Yuva`/`has_alpha=true`, it accepts the precisely admitted 8-bit **four canonical full-size 4:4:4 planes**, validates actual independent received byte lengths and hashes and bounded output, and requires `orientation=Identity`, `!is_grid`, `!is_sequence`. Refuse `Yuva` at 10/12 bits or 4:2:0/4:2:2 in this narrow R07 slice, contradictory alpha boolean, malformed 4th plane, wrong alpha stride/rows, altered alpha byte, short/trailing bytes, nonidentity/unknown orientation claim, out-of-bounds or wrong-bit-depth. Preserve existing V1 first-failure precedence (notably `WORKER_OUTPUT_OVERSIZE` before canonical-layout rejection) and all current fake-port negative expectations. Preserve the R05 `Yuv` 3-plane path, `orientation=Unverified` and all N121 behavior without a change.

If necessary, add **read-only getters** on the inward `MediaWorkerCheckedEnvelopeV1` for its *checked claim* `has_alpha` and `orientation` so the tests can distinguish it. Document that the parent cannot independently attest item-property semantics by hashing data it merely received; these getters preserve untrusted media semantics, **never** claim finished correctness. Keep `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1=false` and `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1=false` unchanged. No public route, derived asset or publisher may consume this as a finished media product.

### 3.3 Independent display-orientation proof is mandatory, not just metadata parity

The R03 independent oracle used pinned `avifdec 1.0.4`/dav1d 1.4.1 + Pillow 10.2.0, and found **coded** PNG RGBA width 512, height 256. It recorded display SHA by an audited 90° anticlockwise rotation of the **whole RGBA**, alpha included. The positive R07 integration test must:

1. Obtain **actual four plane bytes from the real child** via the existing recording-port seam used by N121, prove their geometries and hashes against the parent independently recomputing their received bytes; prove the child/source identities, verified OS limits and checked cleanup.
2. On the *same immutable source bytes*, use an isolated test-only direct call to the **same pinned Rust decoder** to verify its 4 oriented output planes equal the worker-transferred planes byte for byte. This comparison links the actual worker transport to a media object whose conversion can be tested; it is **not** an independent reference itself.
3. Convert the directly decoded valid oriented image **fallibly** to packed 8-bit RGBA with the pinned codec's `try_to_rgba8()` — no silent BT.601 fallback, no 8/10/12 depth coercion. Independently hash that exact RGBA buffer with Solo's SHA-256 and require **exact equality** to R03 pinned `rgba_display_sha256=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744` at `256×512`. Verify the same source would not match `rgba_sha256=5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82` (the *coded*, unrotated frame), and independently count the RGBA alpha range `128..255` and `123952` nonopaque samples. This checks **colour and alpha** jointly after the same orientation mapping; the R03 oracle remains unchanged.
4. If the exact independent hash is unequal, **do not manufacture a green by changing the expected SHA, tolerating arbitrary pixels, removing colour checks or replacing the test with a self-comparison**. Distinguish genuine codec colour rounding vs incorrect orientation/alpha if possible, then report actual evidence and `status=STOPPED` if the pinned implementation cannot satisfy the required semantic correctness under this ledger. The next Architect order will then address the demonstrated discrepancy. Do not introduce a second codec or modify the vendor package without separate authority.

## 4. Exactly three new real-host N122 cases — honest new evidence

Add a new `n122` test module inside the **existing** `crates/solo-cli/tests/media_worker_host_smoke.rs`, and add focused inward fake-port negatives in the existing app test. Reuse N121's actual `Root`, `test_inventory_v1`, `ContentRootCapability`, `read_graphic_asset_bytes_v1`, compiled `BIN`, recording-port/real `ConfinedAvifInspectAdapterV1`, source hashing and fixed `reference-goldens.json` parsing. Do not create a new artificial reader or accept a fake as proof of a real worker.

1. **`n122_01_valid_alpha_irot_confined_display_rgba_oracle`**: source `abc_color_irot_alpha_irot.avif` read by the real confined reader in a nonfavicon graphic role, actual child launched/reaped under OS profile, parent receives four full YUVA 4:4:4 planes at **256×512 / 8 bit**, alpha present, orientation identity as a *claim*; actual worker planes equal direct decoded test-only product planes and independent pinned **display** RGBA digest plus alpha range/count exactly per §3.3; source SHA and output digests verified, no release/publication authority. Capture the real PID. Never assert the unrotated PNG is the display image.
2. **`n122_02_mismatched_alpha_and_nonessential_transform_refused`**: both pinned negatives travel through **real** reader and **real confined** `avif` worker; both refuse with no accepted partial record even if raw `oxideav_avif::decode` could return pixels. Assert the policy branches inspect **primary and alpha essential associations**, not merely whether the decoder produces a fourth plane. Repeat the positive only for an authentic correct association. Add adversarial V1 fake envelopes with wrong alpha count, fake 4th-plane stride, wrong 4th-plane bytes/digest, `has_alpha=false` with four planes, `has_alpha=true` with three, and wrong orientation; each is a typed refusal, never an accepted checked claim. Direct unconfined invocation remains forbidden.
3. **`n122_03_five_level_avif_mix_and_late_private_failure`**: through the real reader and children, combine the accepted R06 four still sources and valid alpha+irot source across Entity/Facet/Site/Page/representation as five separately authored obligations. Verify the four non-alpha `orientation=unverified` contracts, one alpha `orientation=identity`, five distinct real child PIDs, all full-plane validations and exact checked record count. Then replace the **last** representation-owned `internal` + suppressed alpha source with the missing-association negative and assert **one whole-source-set refusal and zero partial checked results**, no fallback or public/private leakage. This is still an **AVIF-only** source set; six-format V4 composition is later.

Each N122 case prints **one** `n122_evidence case=01|02|03 ...` line after all its real assertions. Evidence includes actual positive child PIDs, precise hash/geometry checks, negative category count and whole-set discard; never print a fabricated success token when tests are unavailable locally. Real child and source gate must run natively under Ubuntu's CI-provisioned disposable Btrfs and Windows' real reader. Local WSL without `SOLO_TEST_SOURCE_FS_ROOT` cannot run source-dependent positives; report **NOT RUN LOCALLY**, not skip/waive/green. Local Windows without `link.exe` likewise reports unavailable, while hosted native Windows proof is mandatory.

## 5. CI gating, retained protections and local-first verification

Before **each technical push**, run applicable local checks in the original checkout:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked n122_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

The architecture target is **`check-architecture`**, hyphenated. Because the closed ledger adds only tests in already counted modules/files, expected architecture remains `architecture=legal packages=4 targets=34 files=66 edges=93`; verify actual and do not wildcard the number. Local no-Btrfs host tests are explicitly **NOT RUN LOCALLY**, while all non-source local checks are run. No test ignore, `|| true` around a required gate, `continue-on-error`, reduced test selection to hide failures, new dependency/unsafe authorization, loosened source/image/worker limits, or stdout assertion bypass.

Extend `.github/workflows/rust-verify.yml` with **load-bearing Linux and Windows N122 steps** after Ubuntu's retained R03 Btrfs setup and without changing any older step. Run `cargo test -p solo-cli --test media_worker_host_smoke --locked n122_ -- --nocapture --test-threads=1` as a genuine command, capture its exit code and authentic stdout, and require: exactly **3 passed, 0 failed, 0 ignored**; the exact three named N122 tests each once; the actual `n122_evidence case=01`, `02`, `03` with the pinned displayed RGBA digest/evidence; no skipped/ignored/failure. Use existing interleaving-safe N121 checker pattern, **not** a fragile `... ok$` line when `--nocapture` interleaves output.

The *same* verifier must reject **at least three distinct mutated scratch copies** of the authentic N122 log (missing test case, corrupted display RGBA digest, false successful count/ignored case), genuinely demonstrating RED sensitivity before emitting `n122_gate mutations=rejected`. It may print `n122_gate authentic=accepted` only after the live N122 command and verifier both succeeded. Preserve N121 2/2 and its mutations on both hosts, all N093/N102 and older N097/N108/N113/N116, cargo-deny/RustSec, architecture/unsafe/forbidden import, Btrfs snapshot, image, release/reproducibility, tripwire and reference-oracle gates completely. The independent R03 oracle third job must stay enabled, load-bearing and pinned; do not remeasure/repin its data.

Final **push-triggered** technical run at the same full T15 SHA must have **all three** jobs successful, both N121/N122 authentic and negative-mutant evidence on **both** hosts, and R03 independent manifest still pinned. Any red host or oracle job at the final SHA means no acceptance; remedy a real failing test forward, then run a fresh full pass. Known intermittent inherited N093/N102 refusals must be recorded rather than dismissed or waived.

## 6. Canonical R15 report, acceptance criteria and next boundary

Overwrite only `.documents/Reports/execution-report.md` on main. Record exact A15 ACTIVE blob/SHA/tree/parent and T15.0 transfer, every technical SHA and final T15 head, full file ledger, local commands/exits and any unavailable host tests, the exact property-association policy observed for all three alpha fixtures (including `essential` flags), original independent R03 coded/displaced RGBA hashes, the **real** positively decoded display RGBA hash and alpha range/count, actual worker parent PIDs/limits/reap, four plane SHA-256 values, the typed counterfeit refusals, full five-level candidate/late private failure, N122 3/3 on both hosts and actual mutated-log RED proof, N121 2/2 retention, current architecture real measurement, all fixed security/snapshot gates, final push run/attempt/full SHA/job IDs and exact N122 log lines, main R15 two-host CI run, one worktree/28 refs/four WIP/empty stash/clean trees/scratch cleanup. Do not invent R15 self-SHA in report; include it in the final chat handoff.

`status=COMPLETE` for **this narrow R07 outcome** only if the **production child policy**, **actual parent four-plane checks**, **real N122 3/3 native host positive/negatives**, **exact independently pinned colour+alpha displayed RGBA SHA**, **three authentic/mutated-log passes in both CI host logs**, and **all three final technical CI jobs** succeed at one identical SHA. `status=STOPPED` with the precise counterexample is required if any is missing. No V4 or Stage 03 acceptance from this milestone.

**Next work:** after R07 passes, implement the new application-owned **six-format GraphicImageInspectPortV4** and full N118 seven-case whole-candidate suite, then separately integrate V4 into `main`, build the lawful immutable `platformAssetLibraryVersion: "1.0.0"` Platform `platform/assets/` catalogue, verify privacy-safe derived outputs, and independently close the Stage 03 exit matrix. All those remain OPEN now.

**Execute the genuine confined associated-alpha/irot product slice. Do not replace it with another test-only milestone.**
