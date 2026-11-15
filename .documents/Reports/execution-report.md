# Execution Report

**Work order:** `SOLO-WO-118 R08` — independently adjudicate the associated-alpha `irot` disagreement before any decoder repair.
**Required outcome:** `R03C-AVIF-ASSOCIATED-ALPHA-REFERENCE-AND-DECODER-ROOT-CAUSE-ADJUDICATED`.

**status=COMPLETE** for that adjudication only. The question R07 left open is settled by measurement on both sides, not by argument: the immutable R03 reference is **correct and reproducible**, and the pinned Rust decoder **deviates** by transforming the alpha auxiliary twice with a lossy intermediate resample. The R07 inference that the pinned digest "belonged to the negative fixture" is **refuted**. No N122, V4, Stage 03, publication or product-repair claim is made here.

## 1. Transaction

| Step | SHA | Notes |
|---|---|---|
| baseline (R15) | `ee447a33c17108418e2c2c9f48f457affbc3f993` | verified `origin/main`, only ACTIVE dirty |
| **A16** (`main`) | `021bdc3900ec9f11b7b3014ecb15bb3708bfc5da` — tree `b013d85fb104f738e166dc4cc44e42a8390a7648`, parent `ee447a33` | ACTIVE blob `a8919044a1f843026352c610a2c70c5017180874` identical in worktree, index, commit and `origin/main` |
| **T16.0** (branch) | `d800cb77b823ca397084cc97ce992954845a6563`, parent `405b11c9` | single-path `git restore --source=A16`, identical blob, ACTIVE-only |
| T16.1 … T16.4 (branch) | `06f24c20` → `63434b75` → `2988a5a0` → **`448a7a04d6b587d9756b70ef1b8efe5a5d7a159d`** | referee + Rust differential + two-host gates, then three forward corrections (§6) |
| **R16** | this report on `main` | ACTIVE-only predecessor A16 |

Final technical SHA `448a7a04d6b587d9756b70ef1b8efe5a5d7a159d`: push run `43716332607`, attempt 2 — `verify (ubuntu-24.04)` `132821014884` ✅, `verify (windows-2025)` `132821013943` ✅, `avif-reference-oracle` `132821015756` ✅. Attempt 1 of the same run failed **only** on the inherited `Windows N102 event-identity repeatability (12 runs)` stress gate; the AVIF jobs were green in attempt 1 too. That inherited intermittent is recorded, not waived (§7).


## 2. Independent reference measurement (pinned `avifdec 1.0.4` / dav1d 1.4.1 / Pillow 10.2.0)

`alpha_transform_referee.py` measured **both** reference PNGs produced by the unchanged pinned tool inside the unchanged `avif-reference-oracle` job, from the immutable sources, with closed arity, exact source digests, rotation-invertibility proof, aliasing refusal and in-memory manifest mutations:

```text
n123_reference positive_pin=matched png_sha256=20c06c7c94427c97ef962b5529a633a42f90f4c05bebc1c99ef0b813fd12f97b
  rgba_sha256=5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82
  rgba_display_sha256=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744
  alpha_min=128 alpha_max=255 alpha_non_opaque=123952 coded=512x256 display=256x512
n123_reference negative_measured=true rgba_sha256=5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82
  rgba_display_sha256=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744
  alpha_min=128 alpha_max=255 alpha_non_opaque=123952 coded=512x256
n123_reference same_coded_rgba=true same_display_rgba=true alpha_differences=0 rgb_differences=0 coordinates=none truncated=false
n123_reference mutation_rejections=2/2
n123_reference alias_refusal=verified
n123_reference cleanup=complete
```

The pinned R03 facts (`pristine_manifest_matches=true`, `mutation_rejections=3/3`) still hold unchanged in the same job.

**What this means.** The independent tool produces **byte-identical** reference pixels for the valid associated-alpha positive and the invalid missing-association negative: identical coded RGBA, identical whole-buffer anticlockwise display RGBA, identical alpha statistics, zero differing samples. `avifdec` writes the **coded** 512×256 PNG (it does not apply `irot` to the file), and the pinned 256×512 display digest is recovered by the audited whole-buffer rotation. The reference does **not** rotate the alpha item separately; it treats the primary transform as applying to the composite. Therefore:

1. The R03 pin `950be879…` **is** the correct display image of the valid positive — it was not derived from the negative.
2. R15's statement that "the pinned display RGBA equals what the decoder produces for the negative" was a true measurement with a wrong inference: both sources share one reference image, so reproducing it from the negative says nothing about provenance.
3. **R15's "exactly ten differing alpha samples" was wrong** and is corrected here: ten is the *net non-opaque count difference* (123,962 − 123,952). The actual decoded alpha disagreement is **14,460 differing samples** (11.03 % of 131,072), as the new differential test measures and asserts.

## 3. Rust decoder measurement (`oxideav-avif =0.0.16`, locked, unmodified)

`n123_associated_alpha_irot_decoder_differential_is_measured`, on both hosts:

```text
n123_evidence payloads colour_equal=true alpha_equal=true primary_ids=1/1 alpha_ids=2/2
n123_evidence positive_planes=["5de419d1…cbdd","ea0ff906…be9","ea0ff906…be9","45b5b954…b5a9"]
             negative_planes=["5de419d1…cbdd","ea0ff906…be9","ea0ff906…be9","96b18d51…6ff6"]
n123_evidence positive_rgba=a03a44d4ed770e3f9448889ec3df6577d86ffce02f893309d09629de4a4690fa
             positive_alpha_nonopaque=123962 negative_rgba=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744
             negative_alpha_nonopaque=123952 pinned_display=950be879…84cc744
n123_evidence alpha_differences=14460 rgb_differences=0
             coordinates=156,133:128>132;157,133:128>156;158,133:128>167;159,133:128>154;160,133:128>129;115,134:128>129;…
n123_evidence plane_alpha_differences=14460 rgba_alpha_equals_plane=true nonopaque_delta=10
n123_evidence pin_relation positive_matches=false negative_matches=true same_between_sources=false
n123_decoder measured=true
```

**Payload identity was checked, not assumed**: the files differ in total length by one byte, yet the **coded colour payload and the coded alpha payload are byte-identical** (`colour_equal=true alpha_equal=true`; primary item 1, alpha item 2 in both). The only difference is the `ipma` association — the essential `Irot{angle:1}` on item 2 in the positive, absent in the negative. Any pixel delta therefore comes purely from transform handling.

Decoded geometry is identical and correct in both cases: 256×512, `Yuva444P`, 8-bit, four planes at canonical stride 256 and 131,072 bytes each. Colour planes are byte-identical between the sources (`rgb_differences=0`); only the alpha plane differs, and the packed RGBA alpha channel is exactly the transferred plane, sample for sample (`rgba_alpha_equals_plane=true`).

## 4. Evidence table (both fixtures, both sides)

| quantity | positive `abc_color_irot_alpha_irot.avif` | negative `abc_color_irot_alpha_NOirot.avif` |
|---|---|---|
| source SHA-256 | `b371cc88…9abd3b` | `f2c8cd6d…65ffb9a` |
| container transforms / `essential` | item 1: `Av1C` **ess**, `Irot{angle:1}` **ess**, `Ispe`/`Pixi`/`Colr` non-ess · item 2: `Av1C` **ess**, `AuxC(alpha)` non-ess, **`Irot{angle:1}` ess** | item 1 identical · item 2 **no `Irot` at all** |
| coded payloads | colour and alpha payloads byte-identical to the negative | identical to the positive |
| independent PNG (coded, `avifdec`) | 512×256, `20c06c7c…2f97b` | 512×256, **`20c06c7c…2f97b`** |
| independent coded RGBA | `5102863c…c88f82` | **`5102863c…c88f82`** |
| independent rotated RGBA (256×512 display) | `950be879…84cc744` | **`950be879…84cc744`** |
| independent alpha range / non-opaque | `128..255` / `123952` | `128..255` / `123952` |
| Rust decoded geometry / format | 256×512 `Yuva444P` 8-bit, 4 × 131,072 B | 256×512 `Yuva444P` 8-bit, 4 × 131,072 B |
| Rust Y / U / V plane SHA | `5de419d1…` / `ea0ff906…` / `ea0ff906…` | identical |
| Rust alpha plane SHA | `45b5b954…b5a9` | `96b18d51…6ff6` |
| Rust `try_to_rgba8` digest (524,288 B) | **`a03a44d4…4690fa`** | **`950be879…84cc744`** |
| Rust alpha non-opaque samples | 123,962 | 123,952 |
| colour agreement with the independent reference | exact: Rust's RGBA for the alpha-untransformed source equals the independent display digest byte for byte, so the colour conversion itself agrees; RGB samples differing between the two sources = 0 | exact, as measured |
| alpha agreement with the independent reference | **no** — 14,460 differing samples against 0 | yes |
| differing alpha coordinates | **14,460** (first 64 enumerated in display coordinates, e.g. `156,133:128→132`, `157,133:128→156`) | none |

The pinned donor distinction, quoted from the immutable provenance record (`crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md`, source commit `37a5461042aa3b1eb105193724ee41bfd2478d0b`): the positive is `abc.png` with **a valid association** between the `irot` transformative property and the alpha auxiliary item, and `abc_color_irot_alpha_NOirot.avif` is the specimen "**missing** an association between the `irot` transformative property and the alpha auxiliary image item" — an inconsistent associated-alpha transform that Solo refuses by authored policy regardless of what any decoder renders. The negative stays invalid; no measurement here promotes it.

## 5. Causal trace inside the pinned decoder

All paths and lines are the installed registry source `oxideav-avif-0.0.16/src/`, crate licence **MIT**, unchanged by this order.

1. `decoder.rs:795` `decode_item_output_inner(hdr, item_id, depth, ctx, with_alpha)` decodes the primary, then at `decoder.rs:842` decodes the alpha auxiliary **recursively**: `decode_item_output(hdr, alpha_id, depth + 1, ctx, false)?`.
2. That recursion returns through the same function's tail loop `decoder.rs:861–885` `for prop in transforms_for(&hdr.meta, item_id)`, so the **alpha item's own** `Irot{angle:1}` is applied and the alpha becomes 256×512.
3. `decoder.rs:847` then calls `alpha_as_gray(&alpha, image.width, image.height)` while `image` is still the **coded** 512×256 primary. `alpha_as_gray` (`decoder.rs:1299–1330`) cannot resize by transformation: because `(aw, ah) != (master_w, master_h)` it takes the nearest-neighbour path `sc = c·aw/master_w`, `sr = r·ah/master_h`, decimating the already-rotated alpha back to 512×256 — losing three quarters of its samples and repeating values. This is visible in the data: the positive's alpha is a constant `128` (the alpha floor) in the enumerated positions where the negative carries real samples.
4. `decoder.rs:856` `compose::attach_alpha(&image, &alpha)` attaches that resampled alpha as the fourth plane.
5. `decoder.rs:861–885` applies the **primary's** `Irot{angle:1}` to the composite, rotating colour and the once-rotated-then-resampled alpha together.

Net effect for a correctly associated alpha: **two transforms and one lossy resample**. For the missing-association negative step 2 does nothing, `alpha_as_gray` copies 1:1, and the single primary rotation yields the reference image exactly (`950be879…`). Because colour is decoded once and transformed once in both cases, Y/U/V are byte-identical across the two sources (`rgb_differences=0`), which is exactly what a double-transformed-alpha-only defect predicts.

The ten-sample non-opaque delta versus the 14,460 differing samples is consistent with this mechanism: the double transform mostly permutes alpha values inside a similar coverage region rather than changing overall coverage.


## 6. Recommendation, and the corrections made during this order

**Decision record.** The evidence supports exactly one reading: the R03 source-mapped reference is **correct**, and the pinned decoder's associated-alpha transform pipeline is **wrong** for this profile. There is no live "either/or": the reference needs no correction and the decoder needs a repair. Nothing was re-pinned, no tolerance was introduced, the vendor source was not modified and the invalid fixture was not accepted.

**Recommended repair (next Owner-authorized order, not performed here).** Decode the alpha auxiliary **without** its own transformative properties, attach it at the master's **coded** extents, and apply the master's transforms **once** to the composite. Concretely, in `oxideav-avif`'s `decode_item_output_inner`: give auxiliary decoding a "no item transforms" mode (the recursive call at `decoder.rs:842` must not run `transforms_for(alpha_id)`), and replace the nearest-neighbour resize in `alpha_as_gray` (`decoder.rs:1299`) with a strict refusal when coded extents differ, since a compliant `auxl` alpha shares the master's coded extents. There is **no adapter-only repair**: the double transform happens inside `decode()`, so `crates/solo-adapters/src/graphic_avif_inspection.rs` can neither observe nor undo it.

**Test that fails before the repair and passes after it** — the test added here, with assertions strengthened from "measurement" to "correctness": `n123_associated_alpha_irot_decoder_differential_is_measured` must then assert `alpha_differences == 0`, `rgb_differences == 0`, `positive_rgba == 950be879…84cc744` and `alpha_non_opaque == 123952`. Today the first measures 14,460 and the third is `a03a44d4…`, so the strengthened form is a real red-before / green-after gate. The two-host N122 proof then follows with the valid positive accepted (`orientation=identity`, four canonical YUVA planes, display digest equal to the pin) and both negatives refused on their metadata.

**Minimum next-order surface.** Either an upstream `OxideAV/oxideav-avif` fix (preferred: no Solo file changes, only a version bump in `Cargo.toml`/`Cargo.lock`) or a minimal vendored fork: `vendor/oxideav-avif/{Cargo.toml,src/decoder.rs}`, root `Cargo.toml` `[patch.crates-io]`, `Cargo.lock`, a `deny.toml` source/licence entry, a `LICENSE-AND-PROVENANCE` note, and the architecture checker's expected `targets/files/edges` measurement (a vendored crate root changes the closure). Licence impact is **MIT** for `oxideav-avif` and must be recorded in the third-party `NOTICE` obligation already tracked for release.

**Corrections made during this order (all forward, none relaxing).** (a) The referee initially omitted the positive `source_sha256` from its measured fact map (a real bug surfacing as `KeyError`) — fixed by measuring it. (b) Its anticlockwise coordinate mapping used `height-1-x` instead of `width-1-x`; its own cross-check caught this and it was corrected. (c) The workflow asserted the referee's `alpha_differences` token with a prefix that never appears in output — corrected to the token actually printed. (d) R15's "ten differing samples" is corrected to 14,460 differing samples with a 10-sample net coverage delta.


## 7. Gates, local results and repository state

At the final technical SHA `448a7a04`, push run `43716332607` attempt 2, all three required jobs succeeded: `verify (ubuntu-24.04)` `132821014884`, `verify (windows-2025)` `132821013943`, `avif-reference-oracle` `132821015756`. Both native hosts ran the new differential as a load-bearing step and printed `n123_gate authentic=accepted` and `n123_gate mutation=rejected` after rejecting three corrupted copies of the authentic log (missing `n123_decoder measured=true`, altered `alpha_differences`, false `0 passed; 1 failed` count). Both hosts retained N121 2/2 and its mutations, the N093/N102/N108/N113/N116 gates, cargo-deny/RustSec, the Btrfs snapshot setup, image/release/reproducibility/tripwire and forbidden-import gates; the oracle job retained `pristine_manifest_matches=true` with `mutation_rejections=3/3` alongside the new `n123_reference mutation_rejections=2/2`, `alias_refusal=verified` and `cleanup=complete`.

Attempt 1 of the same run failed only at the inherited `Windows N102 event-identity repeatability (Windows only, 12 runs)` stress step, after that job had already passed the N102 canaries, N121 and the new N123 gate; the same SHA passed it on the fresh attempt 2, and it had passed in run `43714553572`. It is recorded as an inherited host intermittent — not waived, not edited, not skipped.

Local, in the single Owner checkout before the pushes: `cargo fmt --all -- --check` 0, `cargo check --workspace --all-targets --locked` 0, `cargo clippy --workspace --all-targets --locked -- -D warnings` 0, `workspace_graph` 0, `check_architecture_fixtures` 0, `check-architecture` 0 with the unchanged branch measurement `architecture=legal packages=4 targets=34 files=66 edges=93`, `graphic_avif_inspection` 3 passed (two R04 positives plus the new differential), `media_worker` 11 passed, smoke `--no-run` builds. **NOT RUN LOCALLY**: every N121/N122 source-dependent case (no verified Btrfs `SOLO_TEST_SOURCE_FS_ROOT` on this WSL host) and native Windows linkage (`link.exe` absent) — both proven natively in CI instead. The referee cannot run locally (no Pillow, no `avifdec`); it ran only in the pinned CI toolchain, which is the environment R03 itself pinned.

Repository: `main` = A16 `021bdc39` + this report; branch `solo-wo-118-avif-v4-r01` = `448a7a04` (clean); **28** remote heads, one worktree, one `origin`, exactly four WIP files, empty stash, both trees clean; task scratch (`.temp/a16.sh`) removed. Files changed: `.documents/WIP/workorder-active.md` (A16, T16.0), `.github/workflows/rust-verify.yml` (oracle-job extension plus two N123 host gates), `crates/solo-cli/tests/fixtures/avif/alpha_transform_referee.py` (new), `crates/solo-adapters/tests/graphic_avif_inspection.rs` (one added test), and this report. No golden, oracle, fixture, `Cargo.toml`/`Cargo.lock`, codec, production source, N093–N122 test or architecture file was modified.

**Status:** `status=COMPLETE` for `R03C-AVIF-ASSOCIATED-ALPHA-REFERENCE-AND-DECODER-ROOT-CAUSE-ADJUDICATED` only. N122, V4, Stage 03, the Platform catalogue, publication, release and deployment remain **open**, and no decoder repair has been performed or authorized.

