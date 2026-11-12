# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-078 R01` — integrate the independently accepted Linux native `getrlimit` readback seam and its exact 41-edge CI contract into `main`, without widening capability.
**Matching handoff:** `SOLO-WO-078 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `LINUX-NATIVE-READBACK-INTEGRATED`.** The eight reviewed implementation and CI blobs from accepted technical commit `40a40bab1d017e61340fe53399b49fb2ab335c3b` are now on `main` **byte-for-byte identical** to their accepted versions, and the normal push-triggered `rust-verify` workflow passed the complete gate set on **both** `ubuntu-24.04` and `windows-2025` at that exact integration commit. The three real Ubuntu kernel readback tests ran and passed on `main`; Windows compiled and passed everything without compiling or running the Linux-only module.

**What this claims and does not claim.** It claims the promoted source reads back the calling process's *already-existing* `RLIMIT_AS` and `RLIMIT_CPU` ceilings through `libc::getrlimit`, and that `main` now verifies the real 41-edge graph under a 41-edge pin. It does **not** claim `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED`, `SANDBOX-PROVEN` or a release. There is no process spawn, no `setrlimit` or limit mutation, no signal, no OS supervisor, no Windows Job Object, no AVIF parser/decoder and no publication path. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; the media-worker constants granting publication authority and proving media correctness remain `false`.

## 1. Preflight — `S078-01` PASS

Every anchor was read as a full hash from the single real checkout before any write.

| Evidence | Required | Observed |
| --- | --- | --- |
| `main == origin/main == HEAD` | `7576607a9603000e9a07b51aa056756302084e54` | exact |
| `HEAD^{tree}` | `e1ff34d22af5b0c5b1c1b3a50a7a805e69228e18` | exact |
| Canonical report blob (accepted WO-077) | `4c51db7db3c89076e556c6dbda5bffd431060f7f` | exact |
| Prior ACTIVE blob at HEAD | `39fafdd139b1a37d5fde7fd8d58e1202134fc531` | exact |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `main` workflow (pre-promotion) | `eb78dad240bb7adb3e0ef9535870897176c99843` | exact |
| `main` `Cargo.lock` | `2f9e53576df117f2020ae064d6973304f20833f1` | exact |
| `main` adapter manifest / crate root | `6109224cd3e3e89487884631f2b07c68ad8149c3` / `4c5732983ce5a0ced0a7ebc9e25af2e24149f5ee` | exact |
| `main` graph checker / unsafe guard / workspace graph | `268fcf9a33d7e41e304173586952c8341acfbda4` / `5c59a71bf4bf31808dde45be18fa123ba3fb7c44` / `69fa6837f52a1dff72a4a3bb0057021bf544aa4e` | exact |
| Accepted branch local **and** remote | `solo-wo-076-r01` = `40a40bab1d017e61340fe53399b49fb2ab335c3b`, tree `84de1cbfc192336529a6b073641c8ef6a9e74247` | exact |
| Branch predecessor | `b63284afda35d4b874758e2719117bfa88a8dbb4` | exact |
| Historical branches | `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` | unmoved |
| Accepted branch CI (recorded) | `43204471496` at `40a40bab…`, `131136025121` / `131136025260` both success | consistent |
| Most recent `main` CI before this order | `43205675510` at `7576607a…`, both hosts success | confirmed |

One worktree, one remote `origin=https://github.com/provelopment/provelopment-foundation-solo.git`, exactly four files in `.documents/WIP/`, no unrecognized tracked or untracked material, and **the only** initially dirty path was the Owner-installed WO-078 ACTIVE replacement (working blob `f5fa0420e3b1b0d1457551ededafbf479c066002`). `crates/solo-adapters/src/media_worker_host_native.rs` was **absent** on `main` and present on the branch. The branch was treated as an immutable evidence anchor: no new commit, rebase, force-push, deletion or tracking change was made to it at any point.

All eight §3 source blobs were verified at the remote-accepted commit **before** promotion: workflow `e1b5ca6e…`, lock `b1ee56da…`, manifest `47d732f2…`, crate root `bd1d4c58…`, native `2df6d0e9…`, graph `7a5beada…`, unsafe guard `b88e7b42…`, workspace graph `25a91b2c…`.

## 2. Issuance — `S078-02` PASS

| Item | Value |
| --- | --- |
| Issuance commit | `2ce9b4f0ecd99bd7ca29f41a83eaab579ee61e93` |
| Tree | `30f5933ebc153b8a91a6ccbff7cdd2add6028346` |
| Parent | `7576607a9603000e9a07b51aa056756302084e54` |
| ACTIVE blob | `f5fa0420e3b1b0d1457551ededafbf479c066002` (committed unchanged and alone) |
| Changed paths | `.documents/WIP/workorder-active.md` — `M`, one path |
| GitHub readback | sha/tree/parent/ACTIVE blob all match; `files=.documents/WIP/workorder-active.md` |
| CI | run **`43208852877`**, event `push`, at exactly this SHA — `131150866309` (`ubuntu-24.04`) success, `131150865804` (`windows-2025`) success, 33 configured steps each, **0 failures** |

## 3. The eight-path promotion — `S078-03`, `S078-04`, `S078-05` PASS

**Mechanism.** On a clean `main` at the issuance SHA, with the accepted remote branch tip unchanged, the eight paths were materialised with the explicitly authorized path-scoped mechanism only:

```
git restore --source=40a40bab1d017e61340fe53399b49fb2ab335c3b --staged --worktree -- <eight exact paths>
```

No whole-tree checkout, no merge, no branch cherry-pick, no `reset`, no `stash`, no `clean`, no second worktree, no force-push. The branch's own historical ACTIVE and its WO-077 report were **not** imported.

| # | Path | `main` original blob | Promoted blob (must equal accepted) | Status | numstat |
| --- | --- | --- | --- | --- | --- |
| 1 | `.github/workflows/rust-verify.yml` | `eb78dad240bb7adb3e0ef9535870897176c99843` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | M | 2 / 2 |
| 2 | `Cargo.lock` | `2f9e53576df117f2020ae064d6973304f20833f1` | `b1ee56da60f36536548586528db371b7cbe508e4` | M | 1 / 0 |
| 3 | `crates/solo-adapters/Cargo.toml` | `6109224cd3e3e89487884631f2b07c68ad8149c3` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` | M | 5 / 0 |
| 4 | `crates/solo-adapters/src/lib.rs` | `4c5732983ce5a0ced0a7ebc9e25af2e24149f5ee` | `bd1d4c58f5a91f38af63e84e143c850d6372ca64` | M | 13 / 7 |
| 5 | `crates/solo-adapters/src/media_worker_host_native.rs` | *(absent)* | `2df6d0e98395f21fc72294606e9a97e954f97c63` | **A** | 197 / 0 |
| 6 | `crates/solo-cli/tests/architecture_checks/graph.rs` | `268fcf9a33d7e41e304173586952c8341acfbda4` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` | M | 2 / 2 |
| 7 | `crates/solo-cli/tests/unsafe_boundary.rs` | `5c59a71bf4bf31808dde45be18fa123ba3fb7c44` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` | M | 89 / 26 |
| 8 | `crates/solo-cli/tests/workspace_graph.rs` | `69fa6837f52a1dff72a4a3bb0057021bf544aa4e` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` | M | 4 / 2 |

**Staged equality was proven before committing** — `git rev-parse :<path>` for all eight returned exactly the §3 values (`ALL_EIGHT_STAGED_BLOBS_MATCH=True`), the staged path list was those eight and nothing else, `git diff --cached --check` exited 0, and `git ls-files -o --exclude-standard` was empty. The remote tree at the integration commit independently reports the same eight blob SHAs, with statuses `modified ×7, added ×1`.

**Invariant review of the promoted bytes.**

* `Cargo.lock` changes only by the single added entry `+ "libc",` under the `solo-adapters` dependency list. No package, version, source, checksum or other lock record is added, removed or altered; `libc 0.2.191` and `windows-sys 0.61.2` remain the existing locked resolutions.
* The adapter manifest adds one direct dependency under the existing `[target.'cfg(target_os = "linux")'.dependencies]` section — `libc = { version = "=0.2.191", default-features = false }` — so it is Linux-target-only, exact-version, feature-free. `rustix` and the `cfg(windows)` `windows-sys` block are untouched, no dependency is added to an inward crate, and no Windows binding feature changes.
* The crate root keeps `#![deny(unsafe_code)]`, keeps the pre-existing `#[cfg_attr(windows, allow(unsafe_code))]` source-reader exception, and adds exactly one module-scoped Linux exception: `#[cfg(target_os = "linux")]` plus `#[cfg_attr(target_os = "linux", allow(unsafe_code))] mod media_worker_host_native;`.
* The new native file (197 lines) sits entirely inside a positively `#[cfg(target_os = "linux")]`-gated module and contains exactly **two** executable `unsafe` expressions, each with an adjacent `// SAFETY:` rationale: line 120 `unsafe { libc::getrlimit(selector, raw.as_mut_ptr()) }` (rationale at line 115) and line 133 `unsafe { raw.assume_init() }` (rationale at line 129). The only other `unsafe` occurrences in the file are two doc-comment references. It spawns nothing, sets no limit, sends no signal, opens no input, touches no path, and exposes no general-purpose native API (`spawn|setrlimit|pre_exec|fork|execv` occurrences: 0).
* The guards admit only the exact two positively gated native files; no other import or unsafe permission broadens, and the previously accepted negative fixtures remain active.
* The workflow changes exactly two literals — line 660 `grep -q 'edges=40'` → `grep -q 'edges=41'` and line 672 `if ($text -notmatch 'edges=40')` → `if ($text -notmatch 'edges=41')`. The promoted blob contains exactly **two** `'edges=41'` and **zero** `'edges=40'` assertions, is LF-only (0 CRLF) and 68,534 bytes.

**Inverse proof (`S078-05`).** Replacing both `'edges=41'` literals back to `'edges=40'` on the promoted blob reproduces the pre-promotion `main` workflow **exactly**: the reconstructed bytes hash to `eb78dad240bb7adb3e0ef9535870897176c99843` (68,534 bytes). No checker was relaxed, no step skipped, no count made dynamic and no failure hidden.

## 4. Integration commit — one commit, eight paths

| Item | Value |
| --- | --- |
| Message | `feat: integrate proven Linux native readback seam (SOLO-WO-078)` |
| Commit | `0b736a8ffccb56c3c22feb53050e43c806ef072e` |
| Tree | `4fd71d98cb3a2edd8402c27d35a58230651e6f4e` |
| Parent | `2ce9b4f0ecd99bd7ca29f41a83eaab579ee61e93` (the issuance commit) |
| Paths | exactly the eight above — 7 × `M`, 1 × `A`, nothing else |
| Push | fast-forward to `origin/main`; `main == origin/main` at this SHA |
| Readback | remote sha/tree/parent identical; remote file list and all eight remote tree blob SHAs equal §3 |

## 5. Integration CI on `main` — `S078-06`, `S078-07`, `S078-08` PASS

Normal `push` trigger, no manual dispatch, no workflow or rule edit.

| Fact | Value |
| --- | --- |
| Run | **`43209765883`** — conclusion **success**, event `push` |
| Head SHA | `0b736a8ffccb56c3c22feb53050e43c806ef072e` — exactly the integration commit |
| `ubuntu-24.04` job | `131154076931` — success, 33 configured steps: **24 ok, 0 failed, 9 skipped** (all nine are Windows-only counterparts) |
| `windows-2025` job | `131154076937` — success, 33 configured steps: **19 ok, 0 failed, 14 skipped** (all fourteen are Linux-only counterparts) |

No applicable step was skipped and nothing was skipped because of an earlier failure. Evidence read from the logs rather than the run status:

**Real-tree architecture count on both hosts — `architecture=legal packages=4 targets=32 files=58 edges=41`**, produced by both the Linux `grep -q 'edges=41'` gate and the Windows `-notmatch 'edges=41'` gate, with the checker's own process exiting zero and `architecture=legal packages=4` still required. The 29-case refusal self-test passed on both hosts: `self-check=passed cases=29`, `ok dynamic-load refused as dynamic-loading (1 finding(s))`, `ok checker-bypass refused as layer-outward-reference (1 finding(s))`.

**The three real Ubuntu kernel readback tests on `main` — verbatim:**

```
test media_worker_host_native::linux::tests::n076_01_getrlimit_rlimit_as_and_rlimit_cpu_observe_the_real_kernel ... ok
test media_worker_host_native::linux::tests::n076_02_ceilings_keep_their_full_range_and_repeat_consistently ... ok
test media_worker_host_native::linux::tests::n076_03_a_rejected_status_is_a_typed_refusal_not_a_partial_observation ... ok
```

These observe the ceilings the process already had. No limit is installed, so nothing here is an enforcement claim.

**Windows negative (`S078-07`).** The `windows-2025` log contains **zero** `n076_` occurrences, so the Linux-only module is neither compiled nor executed there; every normal suite, guard and gate passed, and the pinned toolchain is asserted as `rustc 1.99.1 (58cf8f9c3 2026-10-13)` / `cargo 1.99.1 (5f94df478 2026-08-27)`.

**Remaining gates, all on their applicable hosts.** `cargo fmt --all -- --check`; strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`; `cargo test --workspace --all-targets --locked`; the workspace package-set gate; the pinned `cargo-deny 0.20.2` gate with a fresh RustSec refresh on both hosts — `component security gate passed: cargo-deny 0.20.2 archive sha256 9f12ed4c…; RustSec 5fe96cf51fde2fd4e749d51ac8bdc59edbdce77b fetched 2026-11-12T11:30:55Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`; the Ubuntu-only RUSTSEC-2026-0009 advisory negative probe; both test-failure tripwires; the release build; `release_smoke=verified accepted_exit=0 refused_exit=2` on **both** hosts; the Ubuntu forbidden-import/bypass refusal — `injected forbidden import refused with category 'undeclared-crate' and exit 1`, `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` — with guaranteed byte-for-byte restore; and the Ubuntu two-clean-release-build comparison, which is byte-identical on raw bytes.

**Git identity versus OS file bytes (recorded distinctly, as required).** The `Cargo.lock` **Git blob** is `b1ee56da60f36536548586528db371b7cbe508e4` on both platforms. The raw checkout-file SHA-256 legitimately differs by line-ending handling: Ubuntu `lock_sha256=bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723`, Windows `lock_sha256=afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e`. The lockfile was not altered within either job — each job re-hashes the file before and after the security gate.

## 6. Gates

| Gate | Status |
| --- | --- |
| `S078-01` identity & authority | **PASS** — §1, all anchors exact, sole checkout/remote, four WIP files, scratch inventoried |
| `S078-02` issuance | **PASS** — §2, `2ce9b4f0…`, run `43208852877` green on both hosts |
| `S078-03` eight exact blobs | **PASS** — §3/§4, staged and remote blobs equal the accepted SHAs, no merge/cherry-pick |
| `S078-04` component/unsafe graph | **PASS** — §3 invariants, two gated `unsafe` expressions, negative fixtures green in CI |
| `S078-05` workflow integrity | **PASS** — two literal changes, inverse reconstructs `eb78dad2…` |
| `S078-06` Linux readback | **PASS** — `N076-01..03` `ok` on real Ubuntu CI at the integration commit |
| `S078-07` Windows negative | **PASS** — zero `n076_` occurrences; all normal gates green |
| `S078-08` complete two-host regression | **PASS** — run `43209765883`, `edges=41`, 29-case self-test, deny, fmt/clippy/tests, release build, smoke 0/2, forbidden-import/restore, byte-identical rebuilds |
| `S078-09` report and cleanup | **PASS** — one report-only commit, remote readback, two-host CI at the report SHA, clean single checkout, no branch movement |
| `S078-10` accurate product scope | **PASS** — `LINUX-NATIVE-READBACK-INTEGRATED` only; containment/Job Objects/media/publication/release/Stage 03 unclaimed |

## 7. First-party `unsafe` admitted by this order

Exactly two, both inside `crates/solo-adapters/src/media_worker_host_native.rs`, both inside a positively `#[cfg(target_os = "linux")]`-gated module, each with an adjacent rationale, and both read-only observations of fixed resource selectors:

| Line | Expression | Purpose |
| --- | --- | --- |
| 120 | `unsafe { libc::getrlimit(selector, raw.as_mut_ptr()) }` | read `RLIMIT_AS` / `RLIMIT_CPU` for the calling process |
| 133 | `unsafe { raw.assume_init() }` | make the fully initialised `libc::rlimit` readable after a zero status |

The pre-existing Windows source-reader exception in the crate root is unchanged and unrelated. No other `unsafe` permission is added anywhere, and `#![deny(unsafe_code)]` remains in force.

## 8. Final state, limitations and cleanup

**`main` now carries the seam and its matching pin.** HEAD after the integration commit was `0b736a8ffccb56c3c22feb53050e43c806ef072e` (tree `4fd71d98cb3a2edd8402c27d35a58230651e6f4e`), clean and equal to `origin/main`. This order changed exactly ten permanent paths in total: the ACTIVE issuance, the eight promoted implementation/CI paths, and this one report. Everything else is preserved — Charter, both SOPs, `README.md`, `AGENTS.md`, the three plan files, the application/domain crates (`media_worker.rs`, `media_worker_protocol.rs`), `source_read_native.rs`, `architecture_checks/source.rs`/`mod.rs` and fixtures, `deny.toml`, the toolchain, the root manifest and every other workflow.

**Branches.** `solo-wo-076-r01` is untouched at `40a40bab1d017e61340fe53399b49fb2ab335c3b` with no new commits, and the three historical branches remain at `4371af21…`, `e6162cdf…` and `d270f95c…`. Nothing was deleted, merged, tagged, released or deployed, and there is still one checkout, one worktree and one remote.

**Honest limitations.** No gate failed in this order, so there is no new red run to disclose; earlier runs `43199345760` and `43200373681` were the WO-076 red runs already recorded and corrected before this order. This workstation still has no usable linker and no Linux target, so **all** compile, lint, test and kernel-readback evidence above comes from remote two-host CI; locally only Git object, diff, blob-equality and byte-level inverse facts were established. The readback observes ceilings that already exist and proves no enforcement, no child limit, no watchdog, no process-group or descendant control, no Windows Job Object, no sandboxing, no decoding, no media correctness and no publication.

**Cleanup.** The only task-owned material created in this run was a handful of `.temp/` report fragments and two short-lived `.temp/` workflow-blob copies used for the byte-level inverse proof; all were deleted after use and `.temp/` has no top-level files remaining. The inherited `.temp/solo-wo-073-wip/` research material is preserved untouched at **4 files / 46,062 bytes** as unverified reference only, with its cleanup trigger deferred to a later native containment order. No Owner archive was present in `.temp/`, and none was fabricated, moved or deleted. No unknown or Owner-owned material was removed.

The Implementation Agent has not self-ACCEPTed, has begun no other order, and stops here for independent Chief Architect disposition: **ACCEPT / REVISE / STOP**.
