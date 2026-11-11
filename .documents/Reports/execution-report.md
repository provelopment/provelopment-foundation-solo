# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-065 R02` — correct the three identified `R03.C2` gaps: repeated-reference byte consistency, and complete asset-route native refusal proofs, preserving the accepted source-reader boundary.
**Matching Owner handoff:** `SOLO-WO-065 R02`.
**Session date:** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo`; one worktree, sole `origin`, no second checkout.
**Disposition:** **COMPLETE** for the three proven corrections only. `SOLO-WO-065 R01` was `REVISE`, not accepted; **`R03.C2` is still not independently ACCEPTED**, **`R03.C` and Stage 03 remain OPEN**, and no release, deployment, publication, catalogue, decoder or platform library exists or is authorized. This report is **not** a self-ACCEPT.

## 1. Starting baseline (verified before any write)

| Anchor | Verified value |
| --- | --- |
| `HEAD == main == origin/main` | `e66c3240b9b69d0473dd0cbc64eec5cffd7c9765`, tree `a8d2ca6fa87f83bc50b10eee54c059cb4ff76dfb` |
| Report blob at baseline | `b05118bac203ec66573d59b5745b247dda2d9f40` (the R01 report) |
| ACTIVE before Owner install | `7c3797c61eb7eef6700228b224baddf26cf7dc8b` (frozen R01 order) |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `97df81e5cf63443d11a8f4414934d4fd05edd0d8` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Branches at preflight | exactly `main`, `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`, `solo-wo-065-r01` = `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`; no R02 branch existed |
| Corrective targets | `graphic_asset_read.rs` `cbd51edfc236f8beaab9ed137a877a9e2d2f0954`; its tests `b74d7d28dddbe396808c35684149d30bdc66b241`; adapter tests `301aa18a16dacf0f0b20ffb2ca34994bf5a3d7b2` |
| Dirty state | only the Owner-installed complete R02 `ACTIVE` (working blob `a40536aec3cc509b9a15613025470257a14195dc`, 28,881 bytes, 136 lines); no untracked or unrelated tracked change; no merge/rebase/cherry-pick/sequencer state |

## 2. Issuance on `main` (ACTIVE alone)

| Item | Value |
| --- | --- |
| **Issuance commit** | **`373763622cfdf9438da0888943ff799b1be6d992`**, tree `a26518e834cf84bf8151f115b64283ee6ef0ec58`, parent `e66c3240…` |
| Changed paths | `.documents/WIP/workorder-active.md` only → `a40536ae…` |
| NEXT / ROADMAP / OUTLOOK | unchanged (`92fa562b…`, `97df81e5…`, `cb9c7a61…`) |
| Issuance CI | **`43035364938`** — `success`; `130572654906` (`ubuntu-24.04`, 33 steps, 9 skips) and `130572653756` (`windows-2025`, 33 steps, 14 skips), **400 / 402 passed**, 0 failed, 0 ignored |

The `NEXT` slot stayed byte-identical to its neutral heading, so no candidate was created or consumed by this correction.

## 3. Three technical paths — exact corrective diff

| Path | Before | After | Change |
| --- | --- | --- | --- |
| `crates/solo-application/src/graphic_asset_read.rs` | `cbd51edf…` | **`6499421d6e718e9d46d64e45fb152f34e2989cc1`** | +21 / −8: repeated-reference comparison now uses **exact bytes** |
| `crates/solo-application/tests/graphic_asset_read.rs` | `b74d7d28…` | **`e9ea65c53c3ecd554d7fc44846b428b6ff96d801`** | +182 / −0: three `r03c2_04b_*` regressions |
| `crates/solo-adapters/tests/graphic_asset_read.rs` | `301aa18a…` | **`8d33a6d82c29b1618af282f75d3937d32a0f3953`** | +279 / −2: seven real asset-route OS proofs |

**Technical commit:** `2de75c1e3f96a84b32f8cd21f491d925225d62d1`, tree `766f3b690be78c60cf05a472ef8f6769432af5d9`, parent `37376362…`, exactly the three paths above (482 insertions, 10 deletions total).

**Implementation of S065R02-04.** `seen_paths` changed from `BTreeMap<String, (PhysicalIdentityV1, u64, u64)>` to `BTreeMap<String, usize>` holding the index of the **first evidence record**, so a repeated logical reference is compared against the bytes that record already owns — no second 20 MiB copy and no digest substitution. A repeat is refused when identity differs (`IdentityCollision`) or when length, retained fingerprint or **`first.bytes() != bytes.as_slice()`** differ (`ChangedDuringRead`), with fingerprint equality explicitly not accepted as content evidence. Insertion records `evidence.len()` before the push, so the index always names the first occurrence. No accessor, category, bound, ordering, port signature or publication/media flag changed, and the refusing obligation is the second one that detected the inconsistency.

**Frozen and untouched after this order:** `source_read_native.rs` `b0ea31db…`, adapter `source_read.rs` `fb6f005a…`, application `source_read.rs` `93bc39d4…`, `graphic_asset_inventory.rs` `7de9bc73…`, both `lib.rs` (`b337a3f6…`, `107f3579…`), `Cargo.lock` `2f9e5357…`, `crates/solo-adapters/Cargo.toml` `6109224c…`, `deny.toml` `3063e864…`, `rust-verify.yml` `eb78dad2…`, `rust-toolchain.toml` `9ec55b3b…`, NEXT, ROADMAP and OUTLOOK. No native/OS code, workflow, dependency, manifest, Architecture, artwork or Platform catalogue change exists.

## 4. S065R02-01…12 acceptance matrix

Every row is **PASS** with direct evidence from the runs below.

| Row | Result | Evidence |
| --- | --- | --- |
| **S065R02-01** | PASS | §1: `e66c3240…`/`a8d2ca6f…`, four WIP slots, three branches at their exact tips, one checkout/worktree/`origin`, only the Owner-installed R02 ACTIVE dirty, no sequencer state. |
| **S065R02-02** | PASS | §2: ACTIVE alone at `37376362…`, remote readback, run `43035364938` green on both hosts, NEXT/ROADMAP/OUTLOOK unchanged. |
| **S065R02-03** | PASS | §3: exactly three technical paths on `solo-wo-065-r02`, branch created only at the issued-green main `37376362…`; whole-order diff vs baseline is ACTIVE plus those three paths (four revisions); no published history rewritten, no older branch moved. |
| **S065R02-04** | PASS | `r03c2_04b_same_reference_with_equal_fingerprint_and_different_bytes_is_refused` — equal length (10), identical identity, **deliberately identical supplied fingerprint 42**, different bytes → `ChangedDuringRead` / `ASSET_SOURCE_CHANGED`, refusal attributed to the **second** obligation (site, `graphic.background`, index 0), both calls observed, no partial set, diagnostic contains no path, filename or bytes. `…_equal_bytes_stays_two_ordered_observations` — two evidence records in order, two calls, both owners preserved. `…_equal_bytes_and_different_fingerprint_is_inconsistent` — fingerprint remains a real check. Identity change (`IdentityCollision`) and distinct-path/one-identity (`IdentityCollision`) remain covered by `r03c2_04` and `r03c2_07`. |
| **S065R02-05** | PASS | `r03c2_08b_trusted_directory_handle_reads_the_same_assets_with_one_root_identity` (both hosts) — real `OwnedFd`/`OwnedHandle` transfer into `from_trusted_directory`, `OperatorDesignatedContentRoot`, **equal root identity** to the bootstrap capability, exact `assets/brand/logo.png` bytes, length, `links == 1`, equal leaf identity, stable re-read. `…_transferred_regular_file_handle_is_refused` and `…_bootstrap_root_that_is_a_file_or_a_link_is_refused` — `UnsafeLocation` in every case, with no asset opened through a substituted root and no OS path in the typed refusal. |
| **S065R02-06** | PASS | `r03c2_08b_bootstrap_root_that_is_a_file_or_a_link_is_refused` (real symlink on Linux, real junction on Windows) and `r03c2_08b_transferred_reparse_handle_is_refused` (Windows, `FILE_FLAG_OPEN_REPARSE_POINT` + `FILE_FLAG_BACKUP_SEMANTICS`) → `UnsafeLocation`. Existing namespace-isolation, intermediate symlink/junction and hardlink cases remain green. No silent skip: the junction fixture asserts `FILE_ATTRIBUTE_REPARSE_POINT`. |
| **S065R02-07** | PASS | `r03c2_10b_asset_read_on_a_non_reflink_filesystem_is_unsupported` (Ubuntu) — a small `assets/logo.png` under the workflow's 16 MiB `tmpfs` (`SOLO_TEST_UNSUPPORTED_FS_ROOT`) drives the **asset port** and refuses `Unsupported` / `ASSET_SOURCE_UNSUPPORTED` with no `Present` bytes and no copying fallback; the same route on the accepted Btrfs fixture returns the exact bytes; a missing variable panics loudly. No workflow or native change. |
| **S065R02-08** | PASS | `r03c2_10b_asset_read_is_refused_while_a_writer_holds_the_leaf` (Windows) — a pre-existing writable handle makes the asset route refuse with the **actual** frozen mapping category `OtherIo` / `ASSET_SOURCE_IO_ERROR` (never invented as `PermissionDenied`), then succeeds with exact bytes, `links == 1` and stable identity after release. `r03c2_10b_pinned_asset_handle_denies_a_competing_writer_and_delete` records the accepted share-read guarantee. No sleeps, no scheduling dependence; the R02 Windows mapped-view and Page-writer tests remain green. |
| **S065R02-09** | PASS | Baseline `400 / 402`; measured after correction `407` (Ubuntu: +3 app, +4 adapter) and `411` (Windows: +3 app, +6 adapter), 0 failed, 0 ignored. All original `r03c2_01…10` suites, prior R01/R02/R03.A/B/C1 suites, architecture real-tree (`architecture=legal packages=4 edges=40`) with its refusal self-test, source namespace isolation, 20 MiB exact/oversize, `cargo fmt --check`, strict Clippy, release build, CLI smoke and the Ubuntu byte-reproducible double build all ran green on both hosts. No total was pre-declared. |
| **S065R02-10** | PASS | §5: `cargo-deny 0.20.2` passed on both hosts with a fresh database (`db_commit=fd6f39814c4d3c359fdbe223d0280dec4933f78f fetched_at=2026-11-11T14:18:33Z`), 18/18 OSV-clean and 18/18 not-yanked third-party packages, unchanged 4 + 18 graph, `rustix = "=1.1.6"` still pinned, no waiver or policy change. The inherited `Unicode-3.0` `license-not-encountered` warning is still emitted and remains **UNVERIFIED**; CI is **not** warning-free. |
| **S065R02-11** | PASS | Branch tip `2de75c1e…` run `43037123017` green on both hosts (jobs `130578787213` Ubuntu, `130578786648` Windows, 33 steps each, 9/14 skips), then a strictly conditional fast-forward-only move of `main` to that exact tip (`37376362…` → `2de75c1e…`, single parent, no merge commit), pushed, then substantive `main` run `43038273943` green on both hosts (jobs `130582864514` Ubuntu, `130582864002` Windows). R02 branch retained; `solo-wo-061-r01` and `solo-wo-065-r01` tips unchanged. No PR, release or deployment. |
| **S065R02-12** | PASS | This single canonical report is committed alone, read back from GitHub, and a third both-host run is required at its exact HEAD (run id reported in the chat handoff, never inside this file). ACTIVE remains the frozen R02 document, NEXT neutral, ROADMAP/OUTLOOK unchanged, `R03.C` and Stage 03 OPEN. |

## 5. CI history — every run this order

| Phase | Run | Revision | Result |
| --- | --- | --- | --- |
| Issuance | `43035364938` | `37376362…` | **success** — Ubuntu `130572654906` 400 passed / Windows `130572653756` 402 passed; 0 failed, 0 ignored, 9/14 host skips |
| Correction branch | `43037123017` | `2de75c1e…` | **success on the first attempt** — Ubuntu `130578787213` **407** passed / Windows `130578786648` **411** passed; 0 failed, 0 ignored, 9/14 skips |
| Substantive `main` | `43038273943` | `2de75c1e…` | **success** — Ubuntu `130582864514` 407 / Windows `130582864002` 411; 0 failed, 0 ignored, 9/14 skips |
| Final report HEAD | reported in chat handoff | report commit | required green on both hosts |

**No red run occurred in this order.** The R01 history it corrects is unchanged: branch run `43028102915` (R01) was the earlier disclosed failure and is not part of this order's corrective work.

Host skips are the same host-inapplicable set as before: counterpart-host steps, the Ubuntu-only advisory negative probe and byte-reproducible double build, and the R03 snapshot filesystem provisioning. No test was marked `#[ignore]`d or silently skipped in place of a passing assertion, and every new asset-route proof executed on the host where it is meaningful (Ubuntu: 7 new; Windows: 9 new).

## 6. Fresh A1 assessment

| Item | Finding |
| --- | --- |
| Locked graph | unchanged **4 first-party + 18 third-party** packages; `Cargo.lock` `2f9e5357…` and `crates/solo-adapters/Cargo.toml` `6109224c…` are byte-identical to the baseline; `rustix` still `"=1.1.6"`. |
| RustSec | CI `cargo-deny 0.20.2` passed on both hosts with a fresh database: `db_commit=fd6f39814c4d3c359fdbe223d0280dec4933f78f fetched_at=2026-11-11T14:18:33Z`; archive integrity verified by the workflow; no waiver, ignore or policy relaxation. |
| OSV / GHSA | fresh sweep this session: **18/18** locked third-party versions returned no advisories (`api.osv.dev`, crates.io ecosystem). |
| Yank / provenance | fresh sweep this session: **18/18** versions report `yanked = false`; CI re-verified the locked fetch and lockfile checksums. |
| Toolchain / runners | pinned `1.99.1` with `rustfmt` + `clippy`; workflow and hosted runner images unchanged. |
| New-code security shape | the correction compares in-memory bytes that are already owned; no new syscall, handle, path resolution, allocation strategy, unsafe, FFI, dependency or network access. Test-only code adds handle fixtures and a Windows `OpenOptions` writer, in disposable task-owned directories. |
| Inherited warning | `warning[license-not-encountered]` for `Unicode-3.0` at `deny.toml:51` still emitted on both hosts. Remains **UNVERIFIED** and untouched; CI is **not** warning-free. Inherited `itoa`/`syn` patch lags unchanged and outside this order. |

## 7. Branches, checkout and hygiene

| Item | State |
| --- | --- |
| `main` | `2de75c1e3f96a84b32f8cd21f491d925225d62d1`, tree `766f3b690be78c60cf05a472ef8f6769432af5d9`; local == `origin/main`; single parent (no merge commit) |
| New branch | `solo-wo-065-r02` **retained** at `2de75c1e…` locally and on GitHub |
| `solo-wo-065-r01` | unchanged `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb` (local and remote) |
| `solo-wo-061-r01` | unchanged `4371af21dce67f33cf1df0384738bf353ca80fc6` (local and remote) |
| Worktree / remote | exactly one checkout, one worktree, sole `origin`; no second clone, no stash left behind |
| Working tree | clean at every published revision (`git status --porcelain --untracked-files=all` empty); no `git clean`, reset, force, amend, rebase or cherry-pick |
| Live cache | ignored `target/` preserved throughout |
| Inherited archives | Solo `.temp/` still **2 files / 289,253,806 bytes**, untouched |
| Task scratch | only disposable test fixtures under runner/task-owned roots, removed by each `Scratch` drop; the task-owned Cline CI log files created by this run's waits were deleted, none remain |
| Unknown material | nothing unknown or Owner-owned deleted, moved or overwritten |

## 8. Remaining gates and handoff

Still unimplemented and outside this order: authoritative media-byte/format/MIME, dimension and pixel-limit verification; an Owner-reviewed lawful `platformAssetLibraryVersion: "1.0.0"` catalogue with asset IDs, hashes, media class, dimensions and licence/provenance; whole-candidate validation that includes that pinned Platform library; and privacy-safe public use with immutable derived outputs. `namespace: "platform"` remains deliberately unverifiable and refuses the whole attempt; no catalogue, artwork, route, HTTP surface, renderer, release or deployment exists.

**Remaining limitations, stated honestly.** Local executable tests are **NOT RUN** on this host (no MSVC `link.exe`), so all functional evidence is the CI runs above; local `cargo fmt --check` and strict `clippy -p solo-application` pass. The Linux non-reflink refusal is proven only through the workflow's own `tmpfs` fixture, which is exactly the required environment; a critical mapped-view assertion at the Windows asset path is only reachable through the existing native module, so it remains covered by the accepted R02 native mapped-view proof rather than duplicated here. Media correctness is never asserted. The `Unicode-3.0` warning remains unverified.

**Handoff:** `SOLO-WO-065 R02 — COMPLETE` for the three correction proofs, with every applicable acceptance row directly evidenced. The Implementation Agent does **not** self-ACCEPT: `R03.C2` awaits independent Chief Architect `ACCEPT / REVISE / STOP` against this report's exact remote HEAD, `R03.C` and Stage 03 remain OPEN, ACTIVE stays the frozen Owner-issued R02 document, NEXT stays neutral, and ROADMAP and OUTLOOK are unchanged.
