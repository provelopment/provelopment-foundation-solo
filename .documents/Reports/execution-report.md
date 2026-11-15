# Execution Report

**Work order:** `SOLO-WO-118 R07` — confined AVIF associated-alpha + essential-`irot` display-semantics slice.
**Required outcome:** `R03C-AVIF-ASSOCIATED-ALPHA-IROT-DISPLAY-PIXELS-TWO-HOST-N122-VERIFIED`.

**status=STOPPED** — the mandatory independent **display-RGBA colour+alpha parity** of §3.3 cannot be attained with the pinned decoder and the immutable R03 goldens. The required positive `abc_color_irot_alpha_irot.avif` decodes to a display RGBA whose SHA-256 is **not** the pinned `rgba_display_sha256`, while the pinned value is reproduced **exactly** by the *negative* `abc_color_irot_alpha_NOirot.avif`. The ten-sample alpha-plane delta is real, measured on the pinned production decoder from the immutable fixture bytes, and cannot be removed inside this order's ledger (no re-pinning, no second codec, no vendor change). No production or test change was pushed: the branch holds only the A15 ACTIVE import, so no unproven product code exists.

## 1. Transaction actually performed

| Step | SHA | Notes |
|---|---|---|
| baseline (R14) | `93832aa04c94522110cfda341ac8fa46874343e6` | verified `origin/main`, only ACTIVE dirty |
| **A15** (`main`) | `830d57ffe8eea2e9e13659e2bcb94dba8b2fc16d` — tree `0040c4360dc2ee5fe3cff4cdb501f3c7c916f04e`, parent `93832aa0` | ACTIVE blob `cb7443f8bc0f9af7074a6d2743517c43f2867f24` identical in working tree, index, commit and `origin/main` |
| **T15.0** (branch) | `405b11c92568f8a8d5943dc4be3b5d36193db219`, parent `81a7bf55` | single-path `git restore --source=A15`, identical blob, ACTIVE-only |
| T15 technical | **none** | stopped before any product/test write was retained |
| **R15** | this report on `main` | ACTIVE-only predecessor A15 |

The technical branch `solo-wo-118-avif-v4-r01` is **clean at T15.0**; the exploratory probe used to measure the pinned decoder was a temporary local edit to the ledger's own test file and was reverted (`git status --porcelain` empty) before any push. The only pushes in this order are A15 ACTIVE, T15.0 ACTIVE, and this report on `main`.

## 2. What the real container actually declares (measured, pinned `oxideav-avif 0.0.16`)

`parse_header` + `assoc_by_id` + `find_alpha_item_id` on the immutable fixture bytes, with source digests verified against the manifest (`abc_color_irot_alpha_irot.avif` = `b371cc88…9abd3b` ✓; `abc_color_irot_alpha_NOirot.avif` = `f2c8cd6d…65ffb9a` ✓; `clap_irot_imir_non_essential.avif` = `33f869fc…fd39fc1` ✓).

**`abc_color_irot_alpha_irot.avif`** — primary item 1 (`av01`), alpha auxiliary item 2 via `auxl`→`auxC` alpha URN; primary `Ispe{512×256}`, `Pixi[8,8,8]`; alpha `Ispe{512×256}`, `Pixi[8]`:

| item | property | `essential` |
|---|---|---|
| 1 | `Av1C` | true |
| 1 | **`Irot{angle:1}`** | **true** |
| 1 | `Ispe`, `Pixi`, `Colr(Nclx 1/13/2 full-range)` | false |
| 2 | `Av1C` | true |
| 2 | **`Irot{angle:1}`** | **true** |
| 2 | `Ispe`, `Pixi`, `AuxC(alpha)` | false |

**`abc_color_irot_alpha_NOirot.avif`** — identical except that **item 2 has no `Irot` at all** (only `Ispe`, `Pixi`, `Av1C`, `AuxC`), while item 1 keeps the essential `Irot{angle:1}`. This is exactly the missing alpha-transform association the order names.

**`clap_irot_imir_non_essential.avif`** — primary 1 and alpha 2 both declare `Clap` (clean aperture 8×10, offsets 2/−6), `Irot{angle:1}` and `Imir{axis:1}`, and **every one of those transformative associations has `essential=false`**; `Pixi` declares 10 bits per channel. It is refusable on the essential flag alone, independently of pixel decode.

## 3. What the pinned decoder actually produces

`oxideav_avif::decode` on each immutable source:

| fixture | geometry | format | planes | plane SHA-256 (Y, U, V, A) |
|---|---|---|---|---|
| positive `…alpha_irot` | **256×512** | `Yuva444P`, 8-bit | 4 × 131,072 B | `5de419d1…cbdd`, `ea0ff906…be9`, `ea0ff906…be9`, `45b5b954…b5a9` |
| negative `…alpha_NOirot` | **256×512** | `Yuva444P`, 8-bit | 4 × 131,072 B | `5de419d1…cbdd`, `ea0ff906…be9`, `ea0ff906…be9`, `96b18d51…6ff6` |

The codec applies the item transform itself, so the returned geometry is already the display geometry 256×512 with four full-size 4:4:4 planes of 131,072 bytes — exactly the §3.1 target. **Colour planes are byte-identical between the two fixtures; only the alpha plane differs** (the positive's alpha is additionally transformed). The required policy (essential `Irot{angle:1}` on primary **and** alpha, one `auxl`/`auxC` alpha, matching depths, no `clap`/`imir`, no non-essential transform) is therefore fully decidable, and it would refuse both negatives for their real metadata defects rather than for decode failure.

## 4. The decisive counterexample (§3.3 step 3)

| quantity | pinned R03 | measured on the required **positive** | measured on the **negative** |
|---|---|---|---|
| alpha plane range | `128..255` | `128..255` ✓ | `128..255` |
| alpha non-opaque samples | `123952` | **`123962`** (ten more) | **`123952`** ✓ |
| display RGBA (`try_to_rgba8`, 256×512, 524,288 B) | `950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744` | **`a03a44d4ed770e3f9448889ec3df6577d86ffce02f893309d09629de4a4690fa`** ✗ | **`950be879…84cc744`** ✓ |

The independently pinned display RGBA **and** the pinned alpha statistics both equal what this decoder produces for `abc_color_irot_alpha_NOirot.avif` — the file **without** the alpha `irot` association — and not for the required positive. For the positive, exactly **10 of 131,072** alpha samples differ (its alpha plane is transformed, and the edge handling of that transform disagrees with the independent `avifdec`/dav1d reference), which changes the whole-RGBA digest. `try_to_rgba8` succeeds; the failure is semantic parity, not conversion availability.

## 5. Why this is a genuine blocker rather than a test problem

The order forbids exactly the three actions that would manufacture a green: changing the pinned `rgba_display_sha256`, tolerating arbitrary pixels or dropping the colour check, and replacing the independent comparison with a self-comparison. The remaining permitted levers are all excluded by the same ledger: the decoder is a pinned third-party crate (`oxideav-avif =0.0.16`, no vendor modification), no second codec may be introduced, the reference goldens/Python/provenance are immutable, and the product may not special-case a source digest, filename or environment value. Under those constraints the pinned semantic correctness cannot be demonstrated for the one required positive.

Two readings are consistent with the measurement and only the Architect can choose between them:

1. **The reference for this profile corresponds to the alpha-untransformed file.** The pinned display digest and alpha counts are byte-exact for `…alpha_NOirot`, so `950be879…` may have been derived from a source whose alpha plane was not rotated. If so, the R03 entry for this profile needs an independent re-derivation (by the Architect, with the unchanged oracle) for the *associated-alpha* source before N122 can assert it.
2. **The pinned decoder's alpha-transform edge handling differs by ten samples** from `avifdec`/dav1d for a correctly associated alpha. Then either a separately authorized vendor/decoder decision is required, or the accepted profile must be narrowed to a class where the two agree.

Either resolution is outside this order's ledger, so the work stops here rather than weakening the gate. No V4, `R03.C`, Stage 03, Platform-library, publication, release or deployment claim is made, and the V3 `avif-unsupported-v3` refusal, the accepted R06 coded-plane capability and every N093–N121 test remain untouched and green.

## 6. Local-first verification actually run

On the technical branch at clean T15.0 (product byte-identical to the accepted T14 final):

| command | exit | result |
|---|---|---|
| `cargo fmt --all -- --check` | 0 | clean |
| `cargo check --workspace --all-targets --locked` | 0 | clean |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 | clean |
| `cargo test -p solo-cli --test check-architecture --locked` | 0 | `architecture=legal packages=4 targets=34 files=66 edges=93` |
| `cargo test -p solo-cli --test check_architecture_fixtures --locked` | 0 | all fixtures pass |
| `cargo test -p solo-cli --test workspace_graph --locked` | 0 | all pass |
| `cargo test -p solo-adapters --test graphic_avif_inspection --locked` | 0 | 2 passed, 0 failed |
| `cargo test -p solo-application --test media_worker --locked` | 0 | 11 passed, 0 failed |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked --no-run` | 0 | builds |

On `main` at A15 the same battery is green with the main-line measurement `targets=33 files=64 edges=81`, which is correct there because `main` has never carried the AVIF technical source. Local WSL **NOT RUN LOCALLY** for any source-dependent case: without a verified Btrfs `SOLO_TEST_SOURCE_FS_ROOT` the real reader returns `NativeError::Unsupported`, so no N121/N122 positive can run here; local Windows is likewise unavailable (`link.exe` absent). No fallback reader, fake source set, `#[ignore]`, targeted `--skip` or arbitrary directory was used. **No hosted run was attempted for R07**, because the parity gate fails on the pinned decoder before any host test could legitimately pass; publishing a red N122 suite would not add evidence the local measurement lacks.

## 7. Repository state and required Owner decision

`main` = A15 `830d57ff` + this report; branch `solo-wo-118-avif-v4-r01` = T15.0 `405b11c9` (clean, ACTIVE-only); **28** remote heads; one worktree, one `origin`, exactly four WIP files, empty stash, both trees clean, task scratch (`.temp/a15.sh`, `.temp/stop.sh`) removed; the protected `d82ae762`, `ad7b74df` and `40b03c73` refs untouched. Files changed in this order: `.documents/WIP/workorder-active.md` (A15, T15.0) and this report. No source, test, workflow, manifest, lock file, golden, fixture or guard was modified.

**Required decision (smallest necessary):** either (a) authorize the Architect to re-derive the R03 display-RGBA/alpha reference for the *associated-alpha* profile with the unchanged independent oracle, or (b) authorize a decoder/vendor fix decision for the ten-sample alpha-transform delta. On that authority, the R07 slice — which is otherwise fully specified and locally decidable: essential `Irot{angle:1}` on primary **and** alpha, one `auxl`/`auxC` alpha, matching depths, no `clap`/`imir`, 256×512 four-plane YUVA 4:4:4 at 8 bits — can be implemented and proven without any other architectural change.

**Status:** `status=STOPPED` — `R03C-AVIF-ASSOCIATED-ALPHA-IROT-DISPLAY-PIXELS-TWO-HOST-N122-VERIFIED` **not** achieved, with the precise measured counterexample above.
