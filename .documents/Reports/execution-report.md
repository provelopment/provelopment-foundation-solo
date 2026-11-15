# Execution Report

**Work order:** `SOLO-WO-117 R01` — integrate the exact verified SVG V3 technical blobs into `main` and prove full two-host CI.
**Date:** 2026-11-15. **Required outcome:** `R03C-SVG-V3-EXACT-BLOB-MAIN-INTEGRATION-TWO-HOST-VERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

The 23 authorized technical paths are now on `main` as byte-identical Git blobs transferred from the pinned source commit `d82ae76273b57b37014d3b4c557b8debd5299eea`, with no branch merge, no ancestry import and no technical-branch WIP. One complete native Ubuntu 24.04 and Windows 2025 `rust-verify.yml` run at the single integration SHA `45a6775bdccfad49feb1f50aad75bd49bcec9d5b` concluded **success** on both jobs, attempt 1.

## 1. Commits and tips

| Step | Commit | Tree | Parent |
|---|---|---|---|
| predecessor (`main`) | `da63eaa2e732c2587856cee3bfbce36bc34a27f1` | — | `3eca7a4c` |
| **A8** | `5e4afaebe48b9220e8d7d582aff79deacbe54e3e` | `a673aab9e51dcfd13ac9af79b1f6e104ab09d59a` | `da63eaa2` |
| **I8** | `45a6775bdccfad49feb1f50aad75bd49bcec9d5b` | `9555b8d7c048eeca5e8ea3b2153b315a14224b08` | `5e4afae` |
| **R8** | given in the chat handoff only (a report cannot contain its own SHA) | — | `45a6775b` |

**A8** committed and pushed exactly one path, `.documents/WIP/workorder-active.md`; the working-tree, index, commit and `origin/main` blob are all `c0396734dacbf8a4b6c33b22825817276db8342d`. No technical-branch ACTIVE text was carried over, and `workorder-active.md` was never restored from the technical branch.

**I8** committed and pushed exactly the 23 paths of §3; `git diff --name-only 5e4afae 45a6775b` is precisely that set — no additions, no omissions, no other file touched. The transfer used the prescribed `git restore --source="$TECH" --staged --worktree`, executed only on a verified clean `main`, and every binary AVIF fixture moved as a Git object, never through shell redirection.

**Source and destination tips:** technical branch `solo-wo-115-avif-svg-v3-r02` remains at `d82ae76273b57b37014d3b4c557b8debd5299eea`, unchanged; `main` advanced `da63eaa2 → 5e4afae → 45a6775b` by ordinary fast-forward. No merge commit, cherry-pick, rebase, reset, force-push, tag, release, PR or deployment occurred.

## 2. Exact 23-path blob equality

Every path below satisfies `TECH:path == :path == HEAD:path`.

- `.github/workflows/rust-verify.yml`
- `Cargo.lock` — `d4eeb3956f3d21dc2fdd3a194e4ffbbea42e81f8`
- `crates/solo-adapters/Cargo.toml` — `47537c5c1539b57109a37de56b05cfc9c71cbfcc`
- `crates/solo-adapters/src/graphic_image_inspection.rs`
- `crates/solo-adapters/src/graphic_svg_inspection.rs` — `ded4d3d067e3967a5c5c992d1d0db6465b2d5d83`
- `crates/solo-adapters/src/lib.rs`
- `crates/solo-adapters/tests/property_resolution.rs`
- `crates/solo-application/src/graphic_image_inspection.rs`
- `crates/solo-application/src/lib.rs`
- `crates/solo-cli/tests/architecture_checks/graph.rs`
- `crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md`
- `crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_NOirot.avif`
- `crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_irot.avif`
- `crates/solo-cli/tests/fixtures/avif/clap_irot_imir_non_essential.avif`
- `crates/solo-cli/tests/fixtures/avif/fox-profile0.10bpc.yuv420.avif`
- `crates/solo-cli/tests/fixtures/avif/fox-profile0.8bpc.yuv420.avif`
- `crates/solo-cli/tests/fixtures/avif/fox-profile2.12bpc.yuv420.avif`
- `crates/solo-cli/tests/fixtures/avif/white_1x1.avif`
- `crates/solo-cli/tests/fixtures/svg/TABLER-ICONS-MIT.txt`
- `crates/solo-cli/tests/fixtures/svg/icon-home.svg`
- `crates/solo-cli/tests/media_worker_host_smoke.rs` — `e315043a66db19b64028f4364806ffb52aae3c48`
- `crates/solo-domain/src/property_registry.rs`
- `crates/solo-domain/tests/property_resolution.rs`

The equality loop passed for all 23 paths both at staging and at `HEAD`, and `git diff --cached --stat` reported exactly `23 files changed, 4639 insertions(+), 23 deletions(-)`. Ten donor fixture/provenance files are included. `Cargo.lock` is byte-identical to the verified technical source, and no governance document, SOP, `deny.toml`, root `Cargo.toml`, source/unsafe checker or unchanged file was modified.

## 3. Local verification before dispatch

Run on WSL Ubuntu against the staged 23-file integration, before committing `I8`:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | rc 0 |
| `cargo check --workspace --all-targets --locked` | rc 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | rc 0, no warnings |
| `cargo test -p solo-cli --test workspace_graph --locked` | `ok. 32 passed; 0 failed` |
| `cargo test -p solo-cli --test check_architecture_fixtures --locked` | `ok. 35 passed; 0 failed` |
| `cargo test -p solo-cli --test check-architecture --locked` | rc 0 — `architecture=legal packages=4 targets=33 files=64 edges=81` |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n116_02 -- --nocapture` | `ok. 1 passed`, `rich_canonical_sha256=0403e067…`, `icon_canonical_sha256=ce9ea48a…` |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n116_04 -- --nocapture` | `ok. 1 passed` |
| `cargo test -p solo-adapters --locked` | `84 passed; 5 failed` — the five `source_read_native::snapshot_proofs` cases only |
| `cargo test --workspace --all-targets --locked` | aborts on that same adapter target; no other failure appears |

One command in the prescribed list was initially mistyped by me as `--test check_architecture` (underscore), which cargo rejects with `no test target named 'check_architecture'`; re-running the real target `--test check-architecture` passed and printed the integrated `edges=81` report. That was a command-name error, not a code or gate failure, and no repository file was involved.

**NOT RUN LOCALLY, stated not waived:** this WSL host has no prepared `SOLO_TEST_SOURCE_FS_ROOT` Btrfs fixture, so the five `source_read_native::snapshot_proofs::*` cases panic in `fixture()` with `SOLO_TEST_SOURCE_FS_ROOT must name an R03 snapshot fixture filesystem`, and `n113_`, `n116_01` and `n116_03` cannot execute here for the same reason — exactly the prerequisites this order anticipates. They were **not** skipped, ignored or faked, and all of them executed for real and passed in the hosted Ubuntu job below. No product code was patched, bypassed or weakened in response to any local result; the five failures are pre-existing environmental preconditions, identical on `main` before this integration.

**Retained-gate verification on the integrated tree:** the imported workflow still contains the four exact N116 tests, the two host gate steps plus the deliberate RED step, the six mutated-log rejections, the independently pinned canonical SVG digests and the exact real-tree `edges=81` checks on both hosts. `Cargo.lock` is identical to the pinned technical source; `quick-xml =0.42.0` and `svgtypes =0.16.1` remain normal roots of `solo-adapters` only, under the unchanged 47-name fail-closed closure, with both negative architecture fixtures still effective.

## 4. Two-host integration proof

**Run `43676077686`, attempt 1, `head_sha = 45a6775bdccfad49feb1f50aad75bd49bcec9d5b`** — triggered by the `I8` push to `main`, no manual retry.

| Job | Id | Conclusion | Successful steps | Platform-inapplicable (skipped) |
|---|---|---|---|---|
| `verify (ubuntu-24.04)` | `132696414596` | **success** | 29 | 14 |
| `verify (windows-2025)` | `132696414394` | **success** | 26 | 17 |

Across both jobs, **zero** steps ended in any state other than `success` or `skipped`. Every skipped step is one host's copy of a platform-exclusive gate (Linux-only cargo-deny, Btrfs snapshot fixtures, advisory probe, package-set, N108 and the V2/V3 Linux steps; the matching Windows-only N102, Windows cargo-deny and package-set steps on Ubuntu). No legitimate skip is claimed as performed work, and no required test is absent.

**N116 exact 4/4 on Ubuntu**, measured from the real compiled `solo-cli`:

- `n116_01_mixed_five_formats_real_host` — `ok`, `formats=png,jpeg,webp,gif,svg`, `pids=[6464, 6468, 6472, 6476, 6480]`, `gif_frames=3`, `vector_canonical_sha256=0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba`, `avif=avif-unsupported-v3`, `partial=false publication=false`
- `n116_02_safe_vector_semantics_and_cross_host_digest` — `ok`, `rich_canonical_sha256=0403e067…`, `icon_canonical_sha256=ce9ea48a3d7bc41673f969727c49eb5dd111d27451d27adab403faf3fa3362d5`
- `n116_03_active_svg_and_hidden_obligation_refuse_whole` — `ok`, `refusals=14`, `late_refusal=port-refused-v3`, `hidden_refused=true`, `favicon_svg=declaration-refused`, `favicon_gif=declaration-refused`, `partial=false fallback=none`
- `n116_04_svg_forgery_and_deliberate_red_oracle` — `ok`, `forged_source=source-digest-mismatch`, `forged_output=output-digest-mismatch`

Aggregate `test result: ok. 4 passed; 0 failed; 0 ignored`, then `n116_gate authentic=accepted` and `n116_gate mutations=rejected-all`, so the missing, FAILED, ignored, duplicate, three-of-four and wrong-digest variants were each refused rather than merely printed. The Windows job ran the same gate step and the same deliberate RED sensitivity step, both `success`, and its digest assertions matched the same pinned values, so the canonical vector digest is identical across hosts.

**Retained gates green on both jobs:** cargo-deny 0.20.2 advisories/bans/licences/sources, the RUSTSEC-2026-0009 negative probe, workspace package-set gate, disposable Btrfs snapshot-test filesystems, format check, strict Clippy with warnings as errors, the complete locked workspace test suite (including the five snapshot proofs and the N113 canaries on Ubuntu), confined image-inspection evidence, N108 8/8 GIF canaries, V2 four-format gate, V3 five-format gate, deliberate RED oracle, N102 isolation canaries with the 12/12 event-identity repeatability, the real-tree architecture checker, and the byte-identical reproducible release build.

## 5. Exclusions honoured and repository state

No AVIF decoding, graphics-policy change, platform asset, `R03.C` completion, Stage 03 closure, browser/HTTP route, contact-system change, release, PR, deployment or provider action occurred. AVIF is present only as refused-before-worker capability with the ten donor fixtures and their provenance; the SVG integration is one bounded step of the still-open Stage 03.

Repository after `R8`: one worktree, one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo`, empty stash, clean index and worktree, exactly four WIP files, and all **27** remote branch refs unchanged (including `solo-wo-115-avif-svg-v3-r02` at `d82ae762`, `solo-wo-113-gif-v2-r01` at `ad7b74df`, `solo-wo-112-gif8-host-r01` at `40b03c73`). No report archive, no second checkout, no extra WIP or Report file, and task-owned temporary script material was deleted at run end.

**Status:** `status=COMPLETE — pending independent Chief Master Architect review.` I do not self-accept; the Chief Master Architect must verify the published report, the exact `I8`/`R8` trees and the two-host logs before returning `ACCEPT`, `REVISE` or `STOP`.

