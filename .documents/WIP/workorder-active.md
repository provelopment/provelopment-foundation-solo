# SOLO-WO-118 R11 — COMPOSE THE SIX EXISTING CONFINED GRAPHIC FORMATS INTO ONE WHOLE-CANDIDATE V4 INERT INSPECTION

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's one checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required bounded outcome:** `R03C-SIX-FORMAT-REAL-CONFINED-WHOLE-CANDIDATE-V4-N124-TWO-HOST-VERIFIED`.

## 1. Binding architectural ruling and accepted evidence

Chief Master Architect independently **ACCEPTED `SOLO-WO-118 R10` only for N122**, the confined associated-alpha/essential-`irot` AVIF class. Final technical branch `solo-wo-118-avif-v4-r01` = **`f4dcf9c24e00ca86151003c6dc6dcce9d3bb3035`**, push run `43726215854` (attempt 1): native Ubuntu `132848601350` SUCCESS, native Windows `132848601340` SUCCESS, independent `avif-reference-oracle` `132848601061` SUCCESS. R18 report-only `main` = **`52d243e74ec172ca658fd262a3fea5c418dbab7e`**, baseline run `43726889744`, Ubuntu `132850666718` and Windows `132850666543` SUCCESS. `oxideav-avif =0.0.18` official-registry SHA checksum `fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee` remains pinned; no other graph or lock drift. Technical branch architecture `packages=4 targets=34 files=66 edges=93`, 28 remote heads.

Accepted R10 facts are immutable input to this order:

- **N120:** `white_1x1.avif` and fox 8/10/12-bit stills match all independent R03 coded-plane golden SHA-256 values and the *text-framed* reference aggregate; no reinterpretation as the binary V1 product aggregate.
- **N121:** two real-host tests pass; exactly **ten** remaining refusal labels, twelve fake-claim refusals; four non-alpha coded Y/U/V positives with `orientation=unverified`, no publication authority. **Do not restore** the now-valid positive `alpha-irot.avif` to N121's hostile list.
- **N122:** all **three** real-host cases pass; correctly associated 8-bit YUVA444P with essential `irot=1` on both primary and alpha is the **one** accepted alpha class; real child transfers four 256×512 displayed planes, `orientation=identity` *as a claim*, independent R03 displayed RGBA digest `950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`; alpha range 128..255, exactly 123952 nonopaque; invalid `NOirot` and nonessential `clap/irot/imir` refuse on parsed metadata; fifteen counterfeit cases refuse; private late failure discards whole set.
- **N123:** fixed upstream decoder parity is green, `alpha_differences=0`, `rgb_differences=0`; the independent R03+R08 oracle still measures both reference fixtures; `pristine_manifest_matches=true` and `mutation_rejections=3/3`.
- **N116:** existing V3 whole-source reader and `ConfinedGraphicInspectAdapterV3` really inspect **PNG, JPEG, WebP, GIF and safe static SVG** on both hosts; `GraphicImageFormatV3::ALL` already names six formats, but `inspect_graphic_image_source_set_v3` and its V3 adapter *intentionally* refuse AVIF with token `avif-unsupported-v3`. **Keep that historic V3 API and N116 refusal exactly working.**

**This R11 deliverable:** one new **V4 whole-candidate inert six-format use case** over one real `GraphicAssetSourceSetV1` acquired through the existing confined `ContentRootCapability` reader. Inspect every authored obligation in order, including internal, unlisted and representation-suppressed obligations, choosing exactly the accepted existing child path for each declared format: PNG/JPEG/WebP/GIF/SVG through the **unchanged V3 port**, AVIF through the **R10 AVIF V1 port**. Require every host/geometry/identity/transport validator already accepted by its respective contract. Return either **one complete typed six-format set** or **one typed, attributable refusal with zero partial records**. No publishing, HTTP, decoded derivative, inferred rights, substitution or Platform acquisition. Preserve each earlier API as a standalone truth.

**Stage 03 remains OPEN.** This is six-format source inspection **only**. The Owner-reviewed `platform/assets/` catalogue, version `1.0.0`, source admission and rights/provenance, privacy-safe immutable derived output, other Stage 03 gates and eventual closing order are later work. Do not claim R03.C/Stage 03/V4 publication closure from this inspection milestone.

## 2. Exact A19 → T19.0 → T19 → R19 Git transaction

1. Preflight in the **single Owner checkout**: one `origin`, one worktree, exact 28 remote heads, four `.documents/WIP/` files, clean `main` and technical branches, empty stash, no foreign scratch. Check `git rev-parse HEAD`, remote heads, branch upstream, `git status --porcelain`, `git stash list`, and canonical ACTIVE path; stop on divergence rather than reset/clean/force.
2. **A19 (`main`):** start from *exact* R18 full SHA `52d243e74ec172ca658fd262a3fea5c418dbab7e`; only the Owner-installed new `.documents/WIP/workorder-active.md` may be modified. Commit and push **ACTIVE alone**. Prove its full Git blob matches working tree, index, commit and `origin/main`. Record complete A19 SHA, parent, tree and blob.
3. **T19.0:** switch to existing technical branch `solo-wo-118-avif-v4-r01` at *exact* `f4dcf9c24e00ca86151003c6dc6dcce9d3bb3035`. Import **only** A19's canonical ACTIVE blob using `git restore --source=<full-A19-SHA> -- .documents/WIP/workorder-active.md`, commit and push that one path. Verify index/commit blob equality. Do not merge `main`.
4. **T19 technical:** make genuine production V4 composition first (after permitted local test sketches), then real-host N124 and CI; forward commits on the **same technical branch**. No branch/tag/PR/merge/deploy, no reset/force, no `git clean`, no source-reader workaround. Before **each technical push**, run applicable local-first gates (§6). Final *push-triggered* CI must contain all three success jobs at **one exact final technical SHA**, and N124 authentic+mutation evidence on both native hosts.
5. **R19 (`main`):** after technical outcome, return to clean main at A19 and commit/push **only** `.documents/Reports/execution-report.md`, reporting COMPLETE solely if every new acceptance gate is met, otherwise STOPPED with actual failure. Run fresh two-host `main` baseline CI. Do not write R19's self-SHA into its own report; give full R19 SHA in chat. Never self-accept Stage 03.

The Architect has **read-only GitHub access**; only Owner-activated Implementation Agent makes these writes.

## 3. Closed permanent file ledger and immutable boundaries

Allowed technical paths (only these; no `mod`/source auto-generation outside them):

```text
.documents/WIP/workorder-active.md                     # import T19.0 only
crates/solo-application/src/graphic_image_inspection.rs # add separate V4 types/use case; retain V3 semantics
crates/solo-application/src/media_worker.rs             # at most pub(crate) visibility for existing checker, no validator weakening
crates/solo-application/src/lib.rs                      # re-export new V4 API
crates/solo-application/tests/media_worker.rs           # optional additive fake-port invariant tests
crates/solo-adapters/src/graphic_image_inspection.rs    # add separate V4 composing adapter and entrypoint
crates/solo-adapters/src/lib.rs                          # re-export V4 adapter and entrypoint
crates/solo-cli/tests/media_worker_host_smoke.rs        # add four new N124 real-host cases in same test target
.github/workflows/rust-verify.yml                       # additive N124 Linux/Windows load-bearing gates
```

**Immutable:** all Cargo tomls, `Cargo.lock`, `deny.toml`; all production codec/child/supervisor/protocol implementations including `graphic_avif_inspection.rs`, `graphic_image_inspection`'s already accepted V1/V2/V3 *behavior*, `media_worker_host.rs` and native, `source_read.rs` and native, `solo-cli/src/main.rs`; R03/R08 fixtures, goldens, independent reference programs and licence/provenance; N093/N097/N102/N108/N113/N116/N120/N121/N122/N123 test semantics, gates and existing evidence tokens; security/architecture checker, donor refs, Platform assets and all non-ACTIVE WIP/SOP/roadmap/architecture files. You may **add** V4 code to the existing graphic-image-inspection files, but may not weaken or repurpose their V3 entrypoints. No new dependency, lock change, second decoder, `unsafe`, filesystem/network/OS primitive, unapproved binary, wildcard, source-root inference, arbitrary directory, or fake result acceptance. Do not change the four-crate graph. Expected architecture remains `packages=4 targets=34 files=66 edges=93`; measure, do not fake it.

If a necessary change is outside the ledger, stop with the exact named blocker and proposed minimum next authorization; do not silently widen the ledger.

## 4. V4 application contract — one complete whole-candidate result or one refusal

Implement **new, explicitly V4-named public contracts** in the existing `crates/solo-application/src/graphic_image_inspection.rs`, re-exported in `solo-application/src/lib.rs`. Keep the V3 exports, discriminants, tokens and `avif-unsupported-v3` refusal untouched.

### 4.1 Closed classification, roles and source bounds

- Reuse `GraphicImageFormatV3::declared_by(obligation.reference())`, its existing closed `ALL = [Png,Jpeg,WebP,Gif,Svg,Avif]`, canonical MIME and lowercase exact extension mapping. The extension identifies the **authored expected format**, never proves the actual source; every format's original confined worker checks content/magic/MIME. Never dispatch from the media byte signature alone. `.jpg`/`.jpeg` are both JPEG; a wrong extension/magic must refuse, not silently decode under a different type.
- Enforce the existing registered role rule `graphic.favicon` **PNG-only**, even in V4; do not expand role/extension grammar. A Platform-namespaced reference cannot be read as Entity by this operation and is never a silent skip.
- Check zero and greater-than-20MiB source lengths before port calls, as V3/V1 already do, with checked arithmetic; retain `MAX_IMAGE_SIDE_V1=8192`, `MAX_IMAGE_PIXELS_V1=32_000_000` and original per-path output bounds. A single source-read failure refuses **before any claimed inspection result**, not `Complete([])`.
- Iterate the **entire** `GraphicAssetSourceSetV1::evidence()` once in original order. Never filter internal/unlisted/suppressed declarations or skip repeated owners with distinct obligations. No implicit/default assets, no format fallback, no dedup masking a declared obligation.

### 4.2 Reuse existing inward validation rather than re-implementing decoders

Use the two **existing outbound ports**, passed to the V4 use case, to preserve Hexagonal direction:

- `GraphicImageInspectPortV3` for **Png/Jpeg/WebP/Gif/Svg**; build one real `GraphicImageInspectRequestV3` from exactly the acquired bytes and the declared format, call the V3 port once per non-AVIF obligation, and apply the same V3 inward `validate_still_v3`, `validate_animation_v3`, `validate_vector_v3` checks, confirmed MIME, parent-computed source digest, worker PID, confinement limits, cleanup, geometry/bytes and GIF/SVG detail. **Do not call the whole-set V3 entrypoint on a fabricated singleton source set**; its source type owns private evidence. Share/refactor a private per-item V3 validator only if it preserves every existing V3 output/refusal; otherwise use the same private validators directly in the additive V4 branch. Never let V3's `AvifUnsupported` be bypassed in the original V3 API.
- `MediaWorkerPortV1::inspect_avif` for **Avif**; construct `MediaWorkerRequestV1::for_source` on the same acquired bytes, invoke exactly one confined AVIF child and run the **existing** `check_observation_v1(obligation,request,observation)` trusted-parent check (a narrowly scoped `pub(crate)` visibility change in `media_worker.rs` is permitted; its content, check order and refusal categories may not change). Verify exact source digest/length, normal exit, limits, deadline, reap/no leak, closed 16-field envelope, canonical 3-plane YUV or accepted 4-plane YUVA, correct plane bytes/digests and binary V1 aggregate. The accepted 3-plane 8/10/12 class keeps `orientation=unverified`; the sole accepted YUVA class keeps `orientation=identity` as a worker claim. Do not declare image correctness or publication authority from the envelope.
- V4 shall have a typed, finite **payload union** distinguishing retained V3 still/GIF/SVG evidence from one parent-checked AVIF envelope, with exact owner, role, authored reference, source SHA and MIME, measured dimensions and pixel count and the original byte-evidence linkage. Avoid declaring a single fake decoded RGBA8 shape for SVG, GIF or high-depth AVIF. A V4 inspection set exposes `.len()`, an ordered immutable slice and `grants_publication_authority() = false` / `proves_whole_publication() = false`; individual records do not grant publication either.
- A V4 refusal carries a closed typed category and the **attributed declaring obligation** for diagnosis. Internally, preserve distinctions for read vs inspect failures and legacy vs AVIF checked refusals. Any human-facing/public rendering must be **path-free and value-free**: no private source path, raw authored reference, worker stderr, buffers, PID, or internal/suppressed declaration text in a public diagnostic. Do not turn a V4 refusal into a partial result. Return no earlier checked records after a late refusal; drop them.

Suggested executable API (names may vary only for accurate Rust conventions; semantics may not):

```rust
pub fn inspect_graphic_image_source_set_v4(
    sources: &GraphicAssetSourceSetV1,
    legacy: &impl GraphicImageInspectPortV3,
    avif: &impl MediaWorkerPortV1,
) -> GraphicImageInspectionOutcomeV4;
```

The V4 inbound orchestration does **not** depend on `solo-adapters`, OS resources or decoder crates; it may use crate-private V3 and V1 trusted validators.

### 4.3 V4 outer adapter: one real read, closed dispatcher, no new child

In `crates/solo-adapters/src/graphic_image_inspection.rs` add a **separate** `ConfinedGraphicInspectAdapterV4` or equivalent and `inspect_entity_graphic_images_v4(root,inventory,trusted_executable)`. It must:

1. Run exactly **one** `read_graphic_asset_bytes_v1(inventory,root)` on the real capability; surface a source-read refusal unchanged and never fall back to `std::fs` or a looser reader.
2. Compose the existing `ConfinedGraphicInspectAdapterV3` for the first five formats and `ConfinedAvifInspectAdapterV1` for AVIF, **only** the already-trusted supplied executable; no second process binary or decoder in the parent.
3. Drive the V4 inward whole-set use case and return a closed `EntityImageOutcomeV4::{Complete,ReadRefused,Refused}` (or equivalent); the output neither exposes a rendered asset nor publishes anything.
4. Preserve V3's legacy entrypoint and legacy no-AVIF behavior exactly. Preserve real child PID evidence where available for tests, without treating a PID or the worker's own digest as correctness.

No requirement in this order to support Platform namespace or produce immutable publication variants. Those require a future Owner-reviewed catalogue and explicit delivery authority.

## 5. Exactly four new authentic native-host N124 cases

Add one new `n124` module inside the existing `crates/solo-cli/tests/media_worker_host_smoke.rs`; reuse N116's **real `Root`/`ContentRootCapability`**, `inventory_of`/`suppressed_inventory_of`, existing pinned PNG/JPEG/WebP bytes, 3-frame GIF fixture, canonical rich SVG or LF-pinned Foundation icon and the **immutable** AVIF fixtures. The tests **must call the real V4 adapter**, not only fake ports or constructors. Do not add a Cargo target or codec dependency. For the AVIF display-RGBA oracle rely on retained N122 adapter-level `try_to_rgba8` reference test and assert the actual N124 transferred four-plane digests against the N122 expected full-plane values, which in turn link to the independent R03 composite. Never label a pinned digest literal as a just-computed RGBA value in N124.

1. **`n124_01_six_formats_five_levels_real_confined_workers`** — one valid, *actual* six-source inventory covering PNG favicon, JPEG (`.jpg` or `.jpeg`), WebP, GIF with a true three-frame timeline, rich safe static SVG with actual nonempty canonical content, and AVIF with the one accepted alpha/`irot` class. Use five EFS levels (at least one level has **two distinct registered roles**; the inventory validator must genuinely admit this). At least one source is `internal`. Prove original inventory order, six intact authoring obligations, correct six *observed* canonical MIME values, real worker PID >0 and **six distinct children** across both port families, bounded dimensions/bytes, per-source independently computed hashes, GIF timing/loop, SVG fixed canonical SHA `0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba` when the rich fixture is used, AVIF four real planes with display geometry 256×512 and source/plane hashes bound to N122 and R03. No publication authority.
2. **`n124_02_six_format_late_private_alpha_refusal_and_no_partial_records`** — first five valid non-AVIF sources across the hierarchy; final **representation-owned `internal`+suppressed** AVIF is the real invalid `abc_color_irot_alpha_NOirot.avif`, acquired through the real reader and rejected by the real confined AVIF worker despite the decoder rendering it. The V4 outcome must be one attributed refusal with **zero** partial records. Repeat with the valid associated-alpha fixture at that position and prove the same six obligations succeed. Assert public diagnostic serialization is path-free and reveals neither private file name nor source reference; do not infer that a suppressed declaration is absent.
3. **`n124_03_hostile_format_role_and_source_gate_matrix`** — real source/worker refusals for: `.png` authored path containing JPEG/WebP bytes (no content-based format substitution), malformed/truncated AVIF, hostile active SVG, invalid GIF or mismatched MIME where a genuine source permits testing, missing authored file, AVIF/GIF/SVG favicon rejected by the **registered role grammar**, and attempted Platform namespace acquisition through the Entity route. Where the declaration grammar refuses, prove it refused *before* the worker; where a source exists, prove genuine child inspection refuses. Include a **late internal/unlisted/suppressed** hostile declaration. No fallback and no empty success.
4. **`n124_04_seven_format_obligations_two_avif_classes_and_adversarial_ports`** — in one actual *seven-obligation* source set: all five legacy formats plus the R06 white/fox coded-plane AVIF and R10 associated-alpha AVIF (two **distinct authored obligations**, both admitted). Prove seven real child PIDs and one ordered V4 result, non-alpha AVIF `yuv/unverified` with 3 exact planes and alpha AVIF `yuva/identity` with four; verify the mixed candidate contains no fake display-orientation claim for coded planes. Add application-level fake-port assertions (in an allowed existing test file or `#[cfg(test)]` block) that a forged V3 source digest/host/format and a forged V1 source digest/host/plane/digest each refuse the **entire V4 set** with zero partial records, preserving the inward validator's typed refusal. No impossible real fake-child success may be accepted.

Each case must print exactly one `n124_evidence case=01|02|03|04 ...` **after** all assertions, using honest observed values (real PIDs, confirmed format list, count, important digest, privacy/failure flags). The evidence token must never be emitted from a synthetic-only positive. Actual source-reading tests run on Ubuntu CI's **R03 disposable Btrfs** and the **native Windows** source reader. On local WSL without `SOLO_TEST_SOURCE_FS_ROOT`, report **NOT RUN LOCALLY** (never fabricate, skip or substitute arbitrary directories); on Windows without `link.exe`, likewise report unavailability.

Retain N116's real V3 four cases and **legacy `avif-unsupported-v3`**. N121 remains **two cases, ten refusals**; N122 **three cases**, N123 repaired, N120 four independent oracle planes. New V4 must not quietly weaken their gates or change their output semantics.

## 6. Two-host CI, local-first checks and exact technical acceptance

Before **each** technical push run, without skipping or masking any failure:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked --no-run
cargo test --workspace --all-targets --locked
```

On WSL without the verified disposable source-root filesystem, document the **five `source_read_native::snapshot_proofs` failures as unrun local preconditions**, not as passing results; do not add `#[ignore]`, `--skip`, `|| true`, `continue-on-error` or a replacement reader. Actual CI for both OSes must run and pass the required real-source targets. Local test synthesis is *not* hosted acceptance.

In `.github/workflows/rust-verify.yml` add two **load-bearing** N124 steps — one Linux after its disposable Btrfs setup and one Windows with the real Windows reader — **without changing existing steps**. On each run:

```sh
cargo test -p solo-cli --test media_worker_host_smoke --locked n124_ -- --nocapture --test-threads=1
```

Require the command exit status 0, exactly **`4 passed; 0 failed; 0 ignored`**, four exact function names each once, each of the four distinct `n124_evidence case=NN` tokens once, genuine six- and seven-child evidence, the R03 display digest under the **`pinned` not `computed`** label where appropriate, `whole_set_discarded=true`, `partial_records=0`, `private_path_absent=true`, `publication=false`, and no failed/ignored tests. Use the proven N116/N121/N122 interleaving-safe checker, not a fragile per-line `... ok$` match. **The same** verifier must reject at least **four** deliberately corrupted copies of the actual log: missing case, false 3/4 result, changed displayed SHA or source/plane claim, and false `private_path_absent=true`/whole-set flag. Only after these tests print `n124_gate authentic=accepted` and `n124_gate mutation=rejected` (exact spelling on both hosts).

Retain every earlier mandatory gate as load-bearing: N093/N097/N102/N108/N113/N116/N120/N121/N122/N123, cargo-deny current RustSec and source/licence policy, architecture/forbidden import/unsafe/root graph, real native source-reader fixtures, size/timeout/cleanup, release/reproducibility, failure tripwire, independent R03+R08 `avif-reference-oracle`. The final technical **push** run must show `verify (ubuntu-24.04)` SUCCESS, `verify (windows-2025)` SUCCESS, and `avif-reference-oracle` SUCCESS, **all at the exact same final T19 SHA**; then independently report job IDs and their authentic evidence. No green by rerun while ignoring an unresolved inherited failure: record any intermittent and require a valid success without waiver.

Expected technical architecture remains `architecture=legal packages=4 targets=34 files=66 edges=93`. Do not weaken the checker to get this number. No production decoding in trusted parent. No newly installed tool or network fetch after the accepted `cargo fetch --locked`; preserve the security gates. No project source outside the §3 ledger.

## 7. R19 reporting, stop rules and forward boundary

In canonical `.documents/Reports/execution-report.md`, publish exact A19/T19.0/T19/R19 choreography, full SHAs *except R19 self-SHA in its file*, exact changed paths, local commands and exit statuses, two native hosts' N124 results and log-mutation evidence, three-job final run and two-job main run, confirmed V3 compatibility, N121/N122/N123 exact totals, security/graph/lock no-drift, WSL native-source limitations and clean repo state. Cite actual observed values, not expected values or assumed CI success.

`status=COMPLETE` only if real six-format V4 whole-candidate orchestration exists and all §5–§6 technical gates pass at the **same final technical SHA**. If an authorized design assumption is false (e.g. no valid sixth obligation can be made without violating cardinality), adjust **within the documented existing grammar** and report the verified valid six-obligation layout. If no valid real layout or essential trust path can be implemented under this ledger, **STOP** with named evidence and minimum missing authorization, rather than inventing an API, a success or a golden.

**Never claim Stage 03/R03.C closure here.** Next work package after accepted R11 must address separately authored and lawfully licensed, cryptographically pinned Platform `platform/assets/` library (`platformAssetLibraryVersion: "1.0.0"`), complete asset-ID/hash/format/dimensions/licence/provenance catalogue and guarded Platform namespace acquisition. Then separately prove immutable derived-output/public privacy and full Stage 03 exit criteria. The currently pinned third-party test fixtures (including CC-BY-SA fox, BSD-2 libavif, MIT Tabler icon) are **not** the publishable Platform catalogue. No conversion of tests into public assets without distinct Owner review.

**Architect disposition upon a properly verified R19:** ACCEPT only the bounded six-format inert V4 candidate, then issue the next Owner-executable order. No releases, merges or deployments are authorized.
