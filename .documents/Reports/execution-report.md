# Execution Report

**Work order:** `SOLO-WO-118 R09` — pin the published upstream alpha-composition repair and prove genuine N123 RED → GREEN on both hosts.
**Required outcome:** `R03C-AVIF-UPSTREAM-ALPHA-COMPOSITION-REPAIR-PINNED-RED-GREEN-TWO-HOST-V1`.

**status=COMPLETE** for that outcome only. `oxideav-avif` moved from `=0.0.16` to the published, checksum-verified `=0.0.18`; the tightened N123 test failed for a real reason before the upgrade (14,460 wrong alpha samples) and now reproduces the immutable independent reference display RGBA **exactly**, with zero differing alpha samples, on both native hosts and in the unchanged R03 oracle. The confined child still refuses every `has_alpha=true` image, including the now-correctly-decoding valid fixture. **No N122, V4, R03.C, Stage 03, catalogue, publication, release or deployment claim is made.**

## 1. Transaction

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` = `b74e5e3ec760a8d8ab60e8e8af64c7ded7b04a43`, branch = `448a7a04d6b587d9756b70ef1b8efe5a5d7a159d` | as required by §2 |
| **A17** (`main`) | `fe37f71171d1997c9a5bddc51393c7ec7bab6803` — tree `4cc962b4c4acd5122922aa6a435aabe250fbe7bb`, parent `b74e5e3e` | ACTIVE blob `08a29efa250cce4c686b73bdd9c5c60d2c723e6a` identical in worktree, index, A17 commit and `origin/main` |
| **T17.0** (branch) | `abbe4234b34ee241dbc18dff43cd24350b543243`, parent `448a7a04` | single-path `git restore --source=A17`; index blob identical to A17's; staged path set = ACTIVE only |
| **T17** (branch) | `d31fc0d94b9dd5b92c0e48d1528ac45bfc748777` | the one final technical commit; `4 files changed, 128 insertions(+), 39 deletions(-)` |

Final technical SHA `d31fc0d94b9dd5b92c0e48d1528ac45bfc748777` on `solo-wo-118-avif-v4-r01`: push run `43719845299`, **attempt 1** (fresh push, no rerun) — `verify (ubuntu-24.04)` `132829229170` ✅, `verify (windows-2025)` `132829229404` ✅, `avif-reference-oracle` `132829229355` ✅, all `success`. No failed attempt needed to be retried.

## 2. The one authorised upgrade

`crates/solo-adapters/Cargo.toml`:

```toml
oxideav-avif = { version = "=0.0.18", default-features = false, features = ["registry"] }
```

`cargo update -p oxideav-avif --precise 0.0.18` resolved purely from the official registry, and `Cargo.lock` changed in exactly two lines:

```diff
 name = "oxideav-avif"
-version = "0.0.16"
+version = "0.0.18"
 source = "registry+https://github.com/rust-lang/crates.io-index"
-checksum = "f36a98615006596baed023d26c16d58bf9327126c919b716ebfdf8122fee8c78"
+checksum = "fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee"
 dependencies = ["oxideav-av1", "oxideav-core", "oxideav-heif"]
```

- Registry checksum equals the Architect's `fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee` exactly; source is the crates.io index, not a Git URL or tag.
- Dependency names remain exactly `oxideav-av1`, `oxideav-core`, `oxideav-heif`; locked `oxideav-av1 0.1.24`, `oxideav-core 0.1.43`, `oxideav-heif 0.0.15` unchanged, as is every other package version, checksum, source and feature set. Total locked packages unchanged (57). No `[patch.crates-io]`, no vendor directory, no wildcard, no duplicate node.
- Installed crate metadata verified locally: `version = "0.0.18"`, `license = "MIT"`, `edition = "2024"`, `rust-version = "1.95"` — inside the accepted `1.99.1` toolchain and the already permitted MIT licence in strict `deny.toml`. The dated upstream fix is recorded in the dependency comment for the outstanding later `NOTICE`/provenance obligation, which this order does **not** close.

## 3. Genuine RED before GREEN (same command, same fixtures, only the lock changed)

Red, with the tightened test and the lock still at 0.0.16:

```text
$ cargo test -p solo-adapters --test graphic_avif_inspection --locked n123_associated_alpha_irot_decoder_differential_is_measured -- --nocapture
exit 101
n123_evidence positive_rgba=a03a44d4ed770e3f9448889ec3df6577d86ffce02f893309d09629de4a4690fa positive_alpha_min=128 positive_alpha_max=255 positive_alpha_nonopaque=123962 negative_rgba=950be879…84cc744 negative_alpha_nonopaque=123952 pinned_display=950be879…84cc744
n123_evidence alpha_differences=14460 rgb_differences=0 coordinates=156,133:128>132;157,133:128>156;…
n123_evidence positive_matches=false negative_matches=true same_between_sources=false
panicked at …: the valid associated-alpha source must decode to the immutable reference display RGBA
  left: "a03a44d4…4690fa"   right: "950be879…84cc744"
test result: FAILED. 0 passed; 1 failed
```

Green, identical command, after only the dependency upgrade:

```text
$ cargo test -p solo-adapters --test graphic_avif_inspection --locked n123_associated_alpha_irot_decoder_differential_is_measured -- --nocapture
exit 0
n123_evidence payloads colour_equal=true alpha_equal=true primary_ids=1/1 alpha_ids=2/2
n123_evidence positive_rgba=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744 positive_alpha_min=128 positive_alpha_max=255 positive_alpha_nonopaque=123952 negative_rgba=950be879…84cc744 negative_alpha_nonopaque=123952 pinned_display=950be879…84cc744
n123_evidence alpha_differences=0 rgb_differences=0 coordinates=none
n123_evidence positive_matches=true negative_matches=true same_between_sources=true
n123_evidence plane_alpha_differences=0 rgba_alpha_equals_plane=true nonopaque_delta=0
n123_decoder repair_verified=true
test result: ok. 1 passed; 0 failed; 0 ignored
```

The two sources carry byte-identical coded colour and alpha payloads (primary item 1, alpha item 2), so this delta isolates transform composition. Geometry on both: 256×512, `Yuva444P`, 8-bit, four canonical stride-256 planes of 131,072 bytes; the RGBA alpha channel equals plane 4 sample for sample. A raw decode match for the negative never promotes its missing association to an acceptance.

## 4. Retained behaviour: R03 reference, N120 goldens, N121 real child

- **R03 oracle, untouched** (`reference-goldens.json`, `reference_oracle.py`, `alpha_transform_referee.py`, fixtures, provenance all byte-identical): `pristine_manifest_matches=true`, `mutation_rejections=3/3`, and the R08 referee in the same job still reports `n123_reference same_coded_rgba=true same_display_rgba=true alpha_differences=0 rgb_differences=0 coordinates=none`, `positive_pin=matched`, `alias_refusal=verified`, `cleanup=complete`. The pinned `avifdec 1.0.4`/dav1d 1.4.1/Pillow 10.2.0 tool identity is unchanged — no reference data was re-pinned.
- **N120 goldens byte-exact after the upgrade**: `white_1x1.avif` (1×1, 8-bit, 444) and the three fox fixtures (1204×800 at 8-bit 420jpeg, 10-bit 420p10, 12-bit 420p12) all report `aggregate_matches=true`; the whole `graphic_avif_inspection` file ran `3 passed; 0 failed` locally and inside both native `verify` jobs' workspace test step. The alpha-enabled valid fixture did **not** change the 8/10/12-bit plane goldens.
- **N121 real child, unaltered and green on both hosts**: `n121_01_four_coded_plane_goldens_cross_worker_and_parent` and `n121_02_real_worker_and_hostile_claim_refusals` ran `2 passed; 0 failed` natively (Ubuntu `132829229170`, Windows `132829229404`), with `records=4`, `planes=12`, `depths=8,10,12`, `orientation=unverified publication=false`, `counterfeits=12 counterfeits_refused=true`, `whole_set_discarded=true`, `unconfined_decoded=false`, and the refusal labels still including **`alpha-irot.avif`** and **`alpha-noirot.avif`**. The repaired decoder therefore did **not** widen the worker's accepted media classes: the valid fixture now decodes correctly in the test-only inspection path and is still refused by the confined product child, exactly as §1 requires.
- **V3 refusal, favicon and publication properties** untouched (no file in that scope changed).

## 5. Native and CI proof, including forged-log rejection

The N123 gate was rewritten from proving an old defect to proving a repair. On each host the authentic single-test log must contain `test n123_associated_alpha_irot_decoder_differential_is_measured`, exactly one `^test n123_…` line, `test result: ok. 1 passed; 0 failed; 0 ignored`, `n123_decoder repair_verified=true`, `positive_matches=true`, `alpha_differences=0 rgb_differences=0`, `plane_alpha_differences=0`, `positive_rgba=950be879…84cc744`, `positive_alpha_min=128`, `positive_alpha_max=255`, `positive_alpha_nonopaque=123952` — and must **not** contain the stale `n123_decoder measured=true` or the historical `alpha_differences=14460`. Four forged copies are rejected on each host before the gate prints its verdict: repaired evidence deleted, the differential rewritten to the historical `14460`, the positive digest rewritten to the pre-repair `a03a44d4…`, and a false `0 passed; 1 failed` count — then `n123_gate mutation=rejected`.

Runtime evidence, `verify (ubuntu-24.04)` and `verify (windows-2025)`:

```text
n123_evidence positive_rgba=950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744 positive_alpha_min=128
  positive_alpha_max=255 positive_alpha_nonopaque=123952 negative_rgba=950be879…84cc744 negative_alpha_nonopaque=123952
n123_evidence alpha_differences=0 rgb_differences=0 coordinates=none
n123_decoder repair_verified=true
n123_gate authentic=accepted
n123_gate mutation=rejected
```

The Linux verifier logic was extracted from the committed workflow and executed locally against the real captured log before the push (`n123_gate authentic=accepted`, `n123_gate mutation=rejected`), so the gate was proven rather than assumed.


## 6. Local verification and honest limits

Recorded before the push, in the single Owner checkout: `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` 0; `workspace_graph` `32 passed`; `check_architecture_fixtures` 0; `check-architecture` 0 with the unchanged branch measurement `architecture=legal packages=4 targets=34 files=66 edges=93`; `graphic_avif_inspection` `3 passed` (two N120 tests plus the repaired N123); `media_worker` `11 passed`; host-smoke `--no-run` builds; `cargo test --workspace --all-targets --locked` → **84 passed, 5 failed**, all five being `source_read_native::snapshot_proofs` panics at `crates/solo-adapters/src/source_read_native.rs:2017` because this WSL host has no verified disposable Btrfs `SOLO_TEST_SOURCE_FS_ROOT`. Those real-source N121/snapshot cases and Windows native linkage (`link.exe` absent) are **NOT RUN LOCALLY** and were proven natively in the hosted jobs at `d31fc0d9`.

## 7. Repository state and completion boundary

`main` = A17 `fe37f711` + this report; technical branch = `d31fc0d9` (clean); **28** remote heads, one worktree, one `origin`, exactly four `.documents/WIP/` files, empty stash, no new branch/tag/PR/release/deploy, no force push, no `reset --hard`, no `git clean`. Files changed by the order: `.documents/WIP/workorder-active.md` (A17, T17.0), `crates/solo-adapters/Cargo.toml`, `Cargo.lock`, `crates/solo-adapters/tests/graphic_avif_inspection.rs`, `.github/workflows/rust-verify.yml`, and this report. Production runtime source `crates/solo-adapters/src/graphic_avif_inspection.rs` needed **no** API-compatibility change — 0.0.18 compiled against the existing invoked surface — so it stays byte-identical to T16.4. Task scratch (`.temp/a17.sh`) removed.

`status=COMPLETE` **only** for `R03C-AVIF-UPSTREAM-ALPHA-COMPOSITION-REPAIR-PINNED-RED-GREEN-TWO-HOST-V1`: the registry upgrade is exact with no other dependency drift, the valid fixture's corrected RGBA digest and zero alpha delta are proven on both hosts, the 8/10/12-bit goldens remain exact, real N121 2/2 and both alpha refusals remain intact, and all three branch jobs are green at one SHA. **N122 is not started, alpha acceptance is not enabled, and V4 / R03.C / Stage 03 closure is not claimed.** The next order must explicitly add the associated-alpha YUVA444P/essential-`irot` worker and parent policy, including the transparent migration of N121's current valid-alpha refusal, before any six-format integration.

