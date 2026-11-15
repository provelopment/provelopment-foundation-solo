# SOLO-WO-118 R01 — IMPLEMENT AND VERIFY SIX-FORMAT AVIF V4 TECHNICAL CAPABILITY ON NATIVE LINUX AND WINDOWS

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Active authority:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-V4-EIGHT-TEN-TWELVE-ALPHA-ORIENTATION-TWO-HOST-TECHNICAL-VERIFIED`.

## 1. Mission — deliver real implementation and evidence

Implement a **new, additive V4 six-format image inspection capability** accepting PNG/JPEG/WebP/GIF/static SVG and conformant AVIF **after actual confined-source acquisition, full bounded decoding, independent parent verification and complete whole-candidate validation**. Verify authentic 8-, 10- and 12-bit AVIF, alpha, orientation and negative media on native Ubuntu 24.04 and Windows 2025 at one exact technical commit. A real AVIF path must reach a distinct, single-use, resource-bounded worker and decode complete pixels; a metadata-only success, copied/partial planes, a known-file lookup or an AVIF renamed as a different format does not meet this order.

**Preserve the accepted baseline:** `main`/`origin/main` at `ec31089a578fd5d1b7c973541d013b4df9be5310` (report R8), and the archived SVG technical branch `solo-wo-115-avif-svg-v3-r02` at `d82ae76273b57b37014d3b4c557b8debd5299eea`. R8 accepted the exact-blob SVG integration at `45a6775bdccfad49feb1f50aad75bd49bcec9d5b`. Its integrated `rust-verify.yml`, N116 4/4, six mutation probes, N102/N108/N113, RustSec, architecture, snapshot, reproduction and full tests are load-bearing. Latest accepted main verification: Actions run `43676603255`, both native jobs successful. **Do not alter the technical branch, release or deploy.**

**Architecture:** `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I, 6.2K, 6A.8, both SOPs, all four WIP files and the one `.documents/Reports/execution-report.md` are binding. The roadmap's Stage 03 remains **OPEN**. This work order delivers AVIF technical capability; **it does not deliver or claim** the still-outstanding Platform `platform/assets/` catalogue, final whole-publication/sealed asset outputs, Stage 03 closure, Stage 04 HTTP, or production release. The Chief Master Architect has GitHub **read-only** access and does not operate the checkout.

## 2. Exact execution topology and safe Git sequencing

Operate in the existing **single** Windows 11 / WSL Ubuntu 26.04 checkout only. Do not clone or add a second worktree. At entry run, capture and inspect:

```bash
pwd
git rev-parse --show-toplevel
git branch --show-current
git rev-parse HEAD
git remote -v
git worktree list --porcelain
git status --porcelain=v1
git stash list
git ls-remote --heads origin
```

Require exactly one `origin` for `https://github.com/provelopment/provelopment-foundation-solo` (the `.git` suffix is equivalent); 27 existing remote branch refs including protected `ad7b74df8638ff56c7137f33d9242da54583f17b` and `40b03c73b0bc8544359438daea6218caa496e85d`; exactly four WIP files; no competing task, unrelated modification, untracked collision or stash. A verified clean checkout may have **only** the newly Owner-installed ACTIVE file as its intended initial change. Report any mismatch rather than destroying Owner data. Never run `git clean`, `reset --hard`, rebase, force-push, unsafe checkout, or deletion of unfamiliar files. Temp files are restricted to Solo `.temp/` or the active Cline-provided task temp area and must be removed at the end if task-owned.

Create precisely these forward checkpoints:

- **A9, `main`:** commit/push **only** this exact Owner-installed `workorder-active.md`; confirm its Git blob identical in working tree, index, commit and `origin/main`. It must not be imported from a technical branch. Require fresh native baseline verification of A9 or prove unchanged baseline from R8 plus mandatory local checks before proceeding.
- **T9, new branch:** create one branch **`solo-wo-118-avif-v4-r01`** from A9 in this same checkout. Make only the §3 permitted source and CI changes. Test locally before each technical push; progress by forward commits. Do not import any branch history, create parallel branches, or merge product code into `main` in this order. Dispatch the branch's `rust-verify.yml` on its exact full SHA and obtain Ubuntu/Windows success on that same SHA, including the new N118 load-bearing gate and every old gate.
- **R9, `main`:** after the technical head is proved, return to `main` only from a verified clean technical working tree. Overwrite, commit and push **only** `.documents/Reports/execution-report.md`. Verify the fresh native Linux/Windows baseline at the report commit. Record `R9` full SHA in the chat handoff (never inside its own report). Preserve the technical branch at the proved T9 SHA for a later exact-blob integration order.

Do not create a PR, merge, release, tag, deployment, provider account, new Entity, another checkout, alternate work-order filename, report archive, or a fifth WIP file. The sole canonical report is overwritten every run, including a truthful STOPPED outcome if necessary; an unresolved substantive issue is not a false COMPLETE.

## 3. Permission ledger and technical implementation

**The only permanent source/configuration write paths on the technical branch** are:

```text
Cargo.lock
crates/solo-adapters/Cargo.toml
crates/solo-adapters/src/graphic_avif_inspection.rs            [new]
crates/solo-adapters/src/graphic_image_inspection.rs
crates/solo-adapters/src/lib.rs
crates/solo-adapters/src/media_worker_protocol.rs
crates/solo-application/src/graphic_image_inspection.rs
crates/solo-application/src/media_worker.rs
crates/solo-application/src/lib.rs
crates/solo-cli/tests/architecture_checks/graph.rs
crates/solo-cli/tests/media_worker_host_smoke.rs
crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md
.github/workflows/rust-verify.yml
```

**One additional test-only path is allowed:** `crates/solo-adapters/tests/graphic_avif_inspection.rs` (new). New fixture binaries, other dependency manifests, the source/unsafe boundary checker, `workspace_graph.rs`, `deny.toml`, root Cargo/toolchain files, native source/process-control files, SOPs, Architecture, roadmap, other WIP and application/production files outside this ledger are **not** authorized. Existing seven AVIF fixture binaries are **immutable**. If the selected codec requires an additional first-party native source change or weakening an architecture guard, STOP and report the exact need rather than smuggling the change in.

**3.1 Dependency and licence control.** Start with the no-C codec candidate **`oxideav-avif = "=0.0.16"`** as one explicit, exact adapter-only dependency, with the minimum decode feature set; resolve its required `oxideav-av1`, `oxideav-heif` and `oxideav-core` transitives at their exact Cargo.lock versions. This is an Owner-issued *implementation selection for this work order*, not permission to follow floating versions. Verify the published 0.0.16 crate's genuine API, manifest, MSRV, source, direct/transitive crate versions, allowed feature graph, security advisories, licences/NOTICE requirements and native Linux/Windows compatibility **before editing product code**. Do not copy an example for another crate version. Do not invoke or expose encoder APIs, enable extra hardware-acceleration/external-plugin/scripting/native `*-sys` features, or add unnecessary dependencies; a decoder crate's built-in public encoder symbols do not themselves justify an invented feature gate. Include the exact component/security/upgrade comparison and reason for accepting the current pins in the report; newly introduced unsound/yanked/vulnerable/unlicensed nodes are refusal, not something to silence in `deny.toml`. Update `APPROVED_ADAPTER_EXTERNAL_NORMAL` and `APPROVED_ADAPTER_CLOSURE` in the shared `architecture_checks/graph.rs` from the **actual locked normal graph**; keep the allowlists enumerated and exact, do not use permissive wildcards, and correct a derived fixed-edge-count assertion in the workflow only to the independently measured real-tree value. A version/API/security incompatibility is a STOP, not an unapproved alternative codec. Do not adopt the older research-only `oxideav-avif =0.0.11` or its historical pins by convenience.

**3.2 Reuse the existing trust boundary.** Begin with these actual source interfaces and read their complete code before editing:

```text
solo_application::MediaWorkerRequestV1::for_source(...)
solo_application::MediaWorkerPortV1::inspect_avif(...)
solo_application::inspect_media_worker_source_set_v1(...)
solo_adapters::media_worker_protocol::parse_media_worker_envelope_v1(...)
solo_application::GraphicImageInspectPortV3::inspect_v3(...)
solo_application::inspect_graphic_image_source_set_v3(...)
solo_adapters::graphic_image_inspection::ConfinedGraphicInspectAdapterV3
solo_adapters::media_worker_host::run_confined_image_worker_v1(...)
```

Do not build a second public source reader, worker launcher, shell command, publication pathway or permissive decoder. The existing `crates/solo-cli/src/main.rs` **already dispatches** `solo_adapters::run_internal_image_worker_v1(&arguments)`; extend that existing worker handler and its closed AVIF token path **without modifying `main.rs`**. Build the AVIF decoder in `graphic_avif_inspection.rs` so **all hostile AVIF parsing** (HEIF `ftyp`, `av1C`, orientation, grids and image sequence inspection included) occurs **after** the already-admitted single-use confined child has installed and verified its resource limits; the trusted parent receives only owned source bytes and performs the source-length check first. Keep the worker executable identity, bounded stdin/stdout, closed argument vocabulary, environment/handle isolation, limits readback, bounded wall deadline and cleanup semantics from the existing media-host supervisor. Every normal success requires `536_870_912` byte child address/commit profile, 10 CPU seconds, 15,000 ms parent wall deadline, verified normal exit, verified termination/reap and no leaked descendant. Do not claim this resource profile independently supplies network/filesystem sandboxing. Re-run and extend the existing native N102 inherited environment/handle, process-tree, lifecycle and bounded-protocol negative tests on both hosts. Supply only a fixed child executable, bounded source bytes and closed arguments; do not furnish credential paths, file handles, output destinations, shells, proxies or network clients. **Report OS-level filesystem/network syscall prohibition as UNVERIFIED unless it is independently demonstrated**; no production-security, publication, deployment or Stage 03 closure is awarded by this technical order. A found actual unauthorized access, leaked credential/handle or uncontrolled descendant is a STOP.

**3.3 Decode content, not a label.** Input is one source digest plus the complete confined bytes. Enforce ≤20 MiB **before worker launch**. Under worker limits require `image/avif` by actual ISOBMFF/AVIF container, valid primary item, codec agreement and full decode; 1–8192 width/height and checked `width * height <= 32_000_000` before decoded allocations; 8, 10 or 12 actual coded depth; correct chroma subsampling/full-vs-limited range and full image plane lengths/digests; independent alpha association and correct `irot`/`imir`/`clap` geometric outcomes on positive valid fixtures. Preserve 10/12-bit sample precision in little-endian 16-bit planes; do not silently downconvert to RGBA8, crop without proof, replace with a still frame of an AVIS sequence, flatten alpha, ignore a malformed essential transform or claim unsupported HDR management. For the initial V4 **still-image** acceptance, AVIS animated sequences and unsupported derived constructs must fail closed with a typed refusal; no first-frame substitution. The valid alpha-plus-rotation input `abc_color_irot_alpha_irot.avif` is a positive fixture. The deliberately inconsistent `abc_color_irot_alpha_NOirot.avif` and `clap_irot_imir_non_essential.avif` are negative fixtures per their pinned provenance; do not invert those expectations to make the decoder green.

Use checked geometry **before any pixel buffer allocation**, following this exact safe-Rust pattern (share this bounded contract in the worker, not an unbounded parent parser):

```rust
fn allowed_image_geometry(width: u32, height: u32) -> Option<u64> {
    if !(1..=8_192).contains(&width) || !(1..=8_192).contains(&height) {
        return None;
    }
    let pixels = u64::from(width).checked_mul(u64::from(height))?;
    (pixels <= 32_000_000).then_some(pixels)
}

fn source_is_bounded(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && u64::try_from(bytes.len())
            .is_ok_and(|length| length <= 20 * 1024 * 1024)
}
```

**3.4 Validate at both ends of the typed protocol.** Reuse the currently closed `MediaWorkerEnvelopeV1` / `MediaWorkerObservationV1` and its 16-field, strict duplicate/unknown-key-refusing parser. The child emits only an exact protocol version, verified `avif`/`image/avif`, source SHA-256, actual dimensions, coded depth, color-model, alpha, grid/sequence/transform facts, ordered plane claims, per-plane hashes and aggregate hash, followed by exactly the claimed bounded complete pixel bytes. The parent independently re-hashes **the bytes actually received**, checks source identity/digest, every plane ordinal, stride×rows, exact length, sample depth and geometry, product limits, output hash, process exit, budget attestation and absence of trailing bytes. A bare successful `inspect()` or an envelope without decoded payload is never success. Reuse `solo_application::inspect_media_worker_source_set_v1`'s parent verification and strengthen it where the new acceptance matrix proves a gap; preserve the old V1 structural contract rather than silently widening it. Do not trust claims of orientation, alpha or colour geometry without an independently pinned reference pixel oracle.

**3.5 One candidate, all declared owners, no fallback.** Preserve V1 three-format, V2 four-format and V3 five-format semantics **unchanged**. Add a separately versioned V4 use-case/port (e.g. `GraphicImageInspectPortV4`, `inspect_graphic_image_source_set_v4`) that supports all six formats, maintaining V3 outcomes for non-AVIF via the existing V3 adapter while adding a real AVIF result carrying parent-verified depth/alpha/orientation and complete output hashes. Do **not** pass a V4 AVIF through V3's deliberate `AvifUnsupported` refusal or delete the V3 negative tests; both versioned behaviors must remain truthful. One mixed six-format candidate must be entirely refused upon one later bad AVIF, even after successful PNG/JPEG/WebP/GIF/SVG. Inspect internal, hidden/unlisted, suppressed and every qualified Entity/Facet/Site/Page/representation declaration before filtering for public projection. Favicon stays PNG-only at declaration. No implicit Platform namespace admission, generic role fallback or public route. V4 still **grants no publication authority**, and the remaining platform-catalogue and sealed-asset steps cannot be reported complete in this order.

## 4. Authoritative AVIF goldens and mandatory refusal matrix

Use the **existing immutable** files under `crates/solo-cli/tests/fixtures/avif/` and their pinned `LICENSE-AND-PROVENANCE.md`: `white_1x1.avif` (8-bit), `fox-profile0.8bpc.yuv420.avif`, `fox-profile0.10bpc.yuv420.avif`, `fox-profile2.12bpc.yuv420.avif`, `abc_color_irot_alpha_irot.avif` (positive alpha/rotation), `abc_color_irot_alpha_NOirot.avif` (negative associated-alpha transform), and `clap_irot_imir_non_essential.avif` (negative transform policy). They are test-only; **never** catalogue or publish them. First independently decode the positive inputs with a pinned external, standards-established reference decoder such as a reproducibly obtained libavif/dav1d CLI used only in the test/oracle-preparation environment; document exact binary/version, SHA-256, licence, host and tool command. Independently derive and commit only the **reference facts and plane/output hashes** in the permitted provenance Markdown (do not fabricate hashes and do not copy the candidate decoder's output as its own oracle). Establish actual pixel geometry, depth, alpha and orientation semantics on each valid fixture. Compare real V4 worker results against those independent oracles on Linux and Windows. A mismatch is a failed gate; do not normalize it away or change the golden to match the product.

Implement a named **N118** native-host suite in `media_worker_host_smoke.rs` and unit/port tests in the two allowed test paths with at least the following real cases, all load-bearing:

1. `n118_01_six_formats_five_efs_levels_real_host` — all six formats, separate qualified owners through Entity→localized representation, distinct worker PIDs where workers are necessary, GIF three-frame and SVG vector outputs unchanged, actual AVIF plane/depth/hash; no fallback or partial candidate.
2. `n118_02_eight_ten_twelve_golden_planes` — real decoded complete 8/10/12-bit positive fixtures; per-plane lengths/strides/sample depth, source and aggregate hashes and independent oracle equality across hosts.
3. `n118_03_alpha_rotation_and_invalid_transform` — valid alpha+rotation positive with correct oriented geometry and alpha pixels; invalid no-irot association and unsupported transform refused with source attribution and no output.
4. `n118_04_hostile_media_and_early_limits` — empty/oversize refusal before worker; mismatched extension/magic/MIME, malformed/truncated ISOBMFF/AV1, missing `ftyp`/`av1C`, invalid dimensions/pixel product, unsupported sequence/grid/colour and decompression bomb all refused; evidence of no partial output.
5. `n118_05_worker_forgery_and_resource_escape` — wrong source/output/plane digest, duplicate/unknown envelope key, altered stride/length, trailing/short output, panic, nonzero exit, timeout, missing limits, inherited handle/env and descendant/resource-limit failure all refused; assert the configured child has no imported network/file path authority. Record the distinction between proven inheritance controls and unproven OS-level syscall isolation; real clean run remains green.
6. `n118_06_hidden_suppressed_late_failure_discards_whole` — a later invalid AVIF declared in an internal, suppressed or unlisted obligation kills the whole six-format candidate even after earlier valid results; no sibling leakage, public promotion, publication authority or role fallback.
7. `n118_07_red_green_and_digest_sensitivity` — a deliberate single-oracle corruption causes a **real test failure**, then unmodified rerun succeeds; source/plane/output hash mutation is caught. Use a test-only environment switch with no production bypass.

A positive test must exercise the actual compiled child and confined source reader on both hosts. A mocked port alone does not count as host proof. Test source files must be opened under the approved snapshot root; WSL without `SOLO_TEST_SOURCE_FS_ROOT` cannot claim host success. Never use `#[ignore]`, `continue-on-error`, floating oracle data, a fake source read, or a test that only prints a success token.

## 5. Local first; hosted CI second

Before *each* technical push, run the applicable checks on Windows 11 and/or the **same checkout** in WSL Ubuntu 26.04. Use the exact real target `check-architecture` (**hyphen**, not underscore):

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked n118_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

Re-run existing N097/N102/N108/N113/N116 locally where the real host prerequisites exist. On WSL missing a prepared Btrfs fixture, explicitly report the snapshot-dependent tests as **NOT RUN LOCALLY** with their actual refusal; still run their unit tests and host-independent N118 cases. Hosted Ubuntu must set up the real disposable Btrfs source filesystem and run every required test for real; Windows must execute the matching native source/worker checks, not merely compile.

Extend `.github/workflows/rust-verify.yml` with a real **N118 exact-seven-case gate** on Ubuntu and Windows. Run the executable N118 suite with `--nocapture`, require exactly seven qualified case names, the literal aggregate `test result: ok. 7 passed; 0 failed; 0 ignored`, seven case evidence tokens and equal independent per-fixture SHA-256 oracles. Use robust fixed-string log checks that tolerate interleaved `--nocapture` stdout; do not anchor individual case lines to `... ok$`. Reuse the accepted N116 checker model: mutate the authentic log by removing a case, marking FAILED, marking ignored, duplicating a case, reducing aggregate count, corrupting a pinned digest, and removing a refusal token; each of all seven mutations must be **rejected** by the same checker. Include the deliberate N118 RED/GREEN test step, plus the entire retained RustSec/cargo-deny, snapshot, architecture, build/reproducibility, N102, N108, N113 and N116 steps. The workflow's exact architecture edge count must equal the genuinely remeasured graph after the approved dependency addition; preserve an exact assertion, not a wildcard.

Dispatch a **fresh** `rust-verify.yml` at the full technical branch SHA, e.g.:

```bash
gh workflow run rust-verify.yml --ref solo-wo-118-avif-v4-r01
gh run list --workflow rust-verify.yml --branch solo-wo-118-avif-v4-r01 --limit 5
# Inspect the actual run's Ubuntu and Windows jobs and logs; verify each recorded head_sha.
```

**Success requires one identical technical HEAD SHA** where native `verify (ubuntu-24.04)` and `verify (windows-2025)` both conclude `success`, every new N118 gate and its mutation/RED sensitivity passes for real, every retained old gate passes, and no unexpected missing/ignored step, warning or unapproved dependency drift is accepted. If a substantive defect is found, fix on the same bounded technical branch, retest locally, then dispatch a **new** same-SHA run. A failed or stale run never qualifies. Report local and hosted test provenance separately.

## 6. Report, truthfulness and boundaries

Overwrite exactly `.documents/Reports/execution-report.md` on `main` after hosted technical verification; commit and push only that report as R9. The report must name actual A9/T9/R9 SHA, tree, parent and changed file set; exact direct/transitive dependencies, licence/security/NOTICE evidence and excluded features; real Goldens and reference-decoder identity; all tested AVIF bit depths, chroma, alpha and oriented geometry facts; separate Linux and Windows process/resource/escape refusal evidence; positive/negative test output, local commands and limitations, CI run/job IDs, head SHA, seven-of-seven N118 and mutation/RED details, retained N116 and A1 gates; and exact branch/main cleanliness, 27 original refs plus the one authorized new technical branch, worktree, WIP, stash and temp cleanup. R9's own SHA is provided in the final chat handoff, not written into the report.

**Mark `status=COMPLETE` only if the entire technical outcome and all mandatory safety/isolation tests are actually proved.** An unresolved requirement, unverified security property, false pixel oracle, dependency incompatibility, wrong SHA, forbidden edit or inadequate host isolation requires `status=STOPPED`, a precise failing command/test and targeted requested correction — never broad claims or bypasses. No implementation agent self-acceptance. The Chief Master Architect will inspect the canonical report, exact commit/file ledger and actual two-host logs and return `ACCEPT`, `REVISE` or `STOP`.

Stage 03 remains open after this technical order even when it succeeds: separately complete controlled AVIF integration into `main`, lawful Owner-reviewed `platformAssetLibraryVersion: "1.0.0"` catalogue/source acquisition, cross-namespace authoritative whole-candidate/public projection and derived immutable media use, and the entire Stage 03 exit matrix. Do **not** prematurely close it, publish a release, provide an HTTP/browser route or claim unverified full-site publication authority.
