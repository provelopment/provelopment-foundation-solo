# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-084 R01` — integrate the independently accepted Linux `RLIMIT_AS` reservation-refusal proof by exact two-blob promotion to `main`.
**Matching activation:** `SOLO-WO-084 R01`. **Date (ISO):** 2026-11-13.

**Implementation-agent claimed outcome: `LINUX-RLIMIT-AS-RESERVATION-REFUSAL-INTEGRATED`.** This is a claim for independent review, **not** self-acceptance. The two accepted blobs of `solo-wo-083-r01` (`ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`) are now on `main` byte-identically, and the complete original `rust-verify` workflow passed fresh on both hosts at the exact integration and report commits.

**What is integrated.** One trusted, Linux pre-exec-limited child verifies all four inherited ceiling halves exactly (`as_soft = as_hard = 536870912`, `cpu_soft = cpu_hard = 10`) and its own positive `pgid == pid`, and only then makes exactly one fallible `Vec::<u8>::new().try_reserve_exact(805306368)` capacity request, which is 256 MiB larger than its whole 512 MiB address-space ceiling; the kernel/allocator returns `Err`, no byte is touched, the child exits zero, its owning parent re-validates the denial inside its 15 000 ms deadline and emits one bounded `media_host_as=limit_enforced ... reserve=denied ...` record, and Windows refuses both internal modes as `HOST_UNSUPPORTED` before any child exists.

**Fresh mainline evidence at integration commit `343f223fe00241b73b5f1e75d6e7e66b1938167a` (new run, not the WO-083 branch run):**

```
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4518 pgid=4518 wall_ms=10
n083_direct_as_child=exit=71
```

The earlier branch run recorded `pid=4519`; this mainline run is independently produced with `pid=4518`, so it is genuinely fresh evidence.

**Nonclaims - remain OPEN.** This is still only one Linux virtual-address-space *capacity* refusal: no page touching, resident memory, RSS, OOM, aggregate quota, CPU-time exhaustion, descendant cleanup, process-group signalling, hostile-child isolation, sandbox, Windows Job Object, immutable executable identity, AVIF/decode or media correctness, publication, release or deployment is proved. It is **not** `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN` or `MEDIA-VERIFIED`. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and media publication/correctness authority flags remain `false`. No source was authored, edited, reformatted or regenerated in this order: only two already-reviewed Git blobs were copied and three commits were made (ACTIVE, integration, report).

## 1. Preflight - PASS (`S084-01`)

Sole Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; `HEAD == main == origin/main`; clean index and worktree except solely the Owner-installed ACTIVE replacement; no untracked files; exactly one worktree; exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`; exactly the four WIP filenames.

| Anchor | Required | Observed |
| --- | --- | --- |
| Baseline `HEAD == main == origin/main` | `5eb882786d68ea45c624f195706a2ed92de94c3b` | exact |
| Baseline tree | `84f9c070cca9313be76b5a02585c0abc17093003` | exact |
| Canonical report blob | `3ed55d433031be4f9c64e0b4161276ac8b34d8a6` | exact |
| Previously tracked ACTIVE | `9f23156531357eebd7590e62dfad6f3ce26146ff` | exact |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b...` / `9d7a8622...` / `cb9c7a61...` | exact |
| `rust-verify` workflow | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact, both `edges=41` pins |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / crate root | `47d732f2...` / `1550d34f...` | exact |
| Native FFI module | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| Baseline façade | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | exact |
| CLI composition root | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| Baseline host smoke test | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | exact |
| Unsafe guard / graph / workspace graph | `b88e7b42...` / `7a5beada...` / `25a91b2c...` | exact |
| Accepted proof branch `solo-wo-083-r01` local and remote | `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`, tree `b299b005c03657344213632eb0e0e4623023f694` | exact, both |
| Proof branch parent / WO-083 issuance | `5247a769a1d5d39510b7a8d9165d1451b75c4e1f` | exact |
| Proof branch own ACTIVE | `9f23156531357eebd7590e62dfad6f3ce26146ff` | exact |
| Proof branch one-commit diff | exactly the two §4 files, `780/5` and `212/0` | exact |
| Preserved `081` / `079` / `076` | `cacc0f9b...` / `380393dd...` / `40a40bab...` | unmoved |
| Preserved `061` / `069` / `070` | `4371af21...` / `e6162cdf...` / `d270f95c...` | unmoved |

The locally installed ACTIVE was the 139-line `**OWNER-ISSUED — SOLO-WO-084 R01**` file, object `3f666aab2016380ed6f34b0a585e5408f9ce6e55`, and its installation was the only pending change. Remote refs were read before any commit: `main` `5eb88278...`, `solo-wo-083-r01` `ccc3ca8f...` plus the six preserved branches at their exact §2 SHAs. Ignored/inherited material was inventoried without modification: `.temp/` has **zero** top-level files and the three directories `architecture-fixtures/`, `solo-wo-073-wip/` (4 files / 46 062 bytes) and `toolchain-installer/`; `target/` exists. Nothing was used as source and nothing was cleaned to manufacture a pass.

## 2. ACTIVE-only issuance and first two-host CI - PASS (`S084-02`)

| Commit | Tree | Parent | Paths | New ACTIVE blob |
| --- | --- | --- | --- | --- |
| `568d16ea5f1ff43dd9e5a51e0f505dc1726d0375` | `fda72d40470c8efd21b6b856b0ca98b22a663165` | `5eb882786d68ea45c624f195706a2ed92de94c3b` | 1 - ACTIVE only | `3f666aab2016380ed6f34b0a585e5408f9ce6e55` |

Staged ACTIVE only, committed the entire Owner-installed file unchanged, ordinary fast-forward push, GitHub readback returning the same commit, tree, parent, single file and ACTIVE blob. The canonical report (`3ed55d43...`), the façade (`aba0cb1a...`) and every other tracked path were byte-identical at that commit.

**Issuance CI - run `43356626242`, `event=push`, `head_sha=568d16ea5f1ff43dd9e5a51e0f505dc1726d0375`, conclusion `success`.** `verify (ubuntu-24.04)` job `131660070243` and `verify (windows-2025)` job `131660070768`: **33 configured steps each, 0 failed steps**, skipping only their usual host-inapplicable sets (9 Ubuntu / 14 Windows). Local `HEAD == main == origin/main` at that exact SHA and a clean index/worktree were re-verified before source staging.

## 3. Exactly two source paths - byte-identical promotion (`S084-03`)

Performed from the clean `main` at the issuance commit with the single prescribed literal command, no wildcard and no tree merge:

```text
git restore --source=ccc3ca8fb37d52b405a2854e4962fb79aff06ab2 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

| Authorized path | Baseline blob on `main` | Staged / worktree / integrated blob | Diff |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | **`90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f`** | `M`, +780 / -5 |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | **`30dffd08dfbd1c8f79edc1c35b7fe092653ea91b`** | `M`, +212 / -0 |

**Five mandatory pre-commit proofs - all pass:**

1. `git diff --cached --name-status` was exactly two `M` rows for those two paths; `git diff --cached --numstat` was exactly `780 5` and `212 0`; `git diff --cached --check` produced no output (exit 0).
2. `git rev-parse :<path>` returned `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` and `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b`, each equal to `git rev-parse ccc3ca8fb37d52b405a2854e4962fb79aff06ab2:<path>` for the same path.
3. `git hash-object <path>` (the prescribed form, which applies Git's attribute/end-of-line machinery by default) returned **the same two blob SHAs** for the worktree files, so the worktree object is the accepted blob with no formatting drift.
4. `git ls-files -o --exclude-standard` was empty and `git status --porcelain` showed only the two authorized staged `M` entries - no unstaged, unknown or third file.
5. `git diff --name-only <issuance-sha> --cached` listed only those two paths, `git diff --name-only` (unstaged) was empty, and every protected non-ledger blob was identical between `HEAD` and the index.

**Line-ending mechanism, disclosed precisely (host configuration, not content drift).** This Windows checkout has `core.autocrlf=true` with `.gitattributes` `* text=auto`, so Git stores LF in the canonical blob while a checkout materialises CRLF: worktree file sizes are 104 237 / 29 493 bytes against blob sizes 101 890 / 28 707 bytes, a difference of exactly 2 347 and 786 carriage returns (the files' line counts). Git's own clean filter maps the worktree bytes back to the exact accepted blobs above, `git hash-object --no-filters` reports the raw CRLF streams (`9a807585...` / `be02e67f...`), `git status` reports no modification, and the committed blobs - the only thing CI, the architecture checker and any other host see - are byte-identical to the accepted branch. No file was re-encoded, rewritten or normalized by this order. The same host mechanism explains the already-documented raw-checkout `Cargo.lock` SHA-256 difference in §5.

## 4. Two-file integration commit - PASS (`S084-04`)

| Commit | Tree | Parent | Paths | Numstat | Blobs |
| --- | --- | --- | --- | --- | --- |
| `343f223fe00241b73b5f1e75d6e7e66b1938167a` | `3504c8fd723916c6464c615e063c560c5fa6a533` | `568d16ea5f1ff43dd9e5a51e0f505dc1726d0375` (issuance) | exactly the two `M` files | `780/5`, `212/0` | `90ec7301...`, `30dffd08...` |

Both files were committed together in one commit whose parent is exactly the issuance commit. GitHub readback returned the same SHA, tree, parent, the two file names, and content blobs `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` / `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` - equal to the accepted branch blobs. The canonical report (`3ed55d43...`) and the workflow (`e1b5ca6e...`) were still identical at that commit. Ordinary fast-forward push only; `main == origin/main`, clean worktree, **no branch created or modified**, and neither the proof branch's prior ACTIVE nor its prior report was imported.

**Permitted tracked ledger actually used: four paths across three commits** - ACTIVE in the issuance commit, exactly the two implementation files in the integration commit, and the canonical report in the final report-only commit. No WIP plan, Charter, SOP, workflow, `Cargo.lock`, manifest, native module, crate root, CLI root, guard, README or other tracked file changed.

## 5. Fresh full integration CI on `main` - PASS (`S084-05`, `S084-06`, `S084-07`)

**Run `43357351786`, `event=push`, `head_branch=main`, `head_sha=343f223fe00241b73b5f1e75d6e7e66b1938167a` - the exact integration commit - conclusion `success`.** `verify (ubuntu-24.04)` job `131662443489` and `verify (windows-2025)` job `131662443089`, each with its original **33 configured steps, 0 failed steps**, skipping only the documented host-inapplicable sets (9 Ubuntu / 14 Windows). Neither the WO-083 branch run nor the issuance run was reused as integration evidence.

### 5.1 Ubuntu - freshly produced at the integration SHA

```
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4518 pgid=4518 wall_ms=10
n083_direct_as_child=exit=71
n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4473 pgid=4473 wall_ms=10
n079_direct_child=exit=71
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4486 pgid=4486 signal=9 reaped=1 wall_ms=2005
n081_direct_wall_child=exit=71
```

- `n083_01_the_child_reserves_over_the_ceiling_and_is_refused ... ok`: `as_soft=536870912`, `as_hard=536870912`, `cpu_soft=10`, `cpu_hard=10`, `request_bytes=805306368`, literal `reserve=denied`, positive `pid=4518`, `pgid == pid`, `wall_ms=10 < 15000`, and a normal child exit whose leader the parent reaped.
- `n083_02_a_direct_as_child_refuses_before_the_reservation ... ok`: the real new result is `exit=71` with no accepted output (the limit check refuses before the 768 MiB request).
- `n083_03_every_wrong_reservation_token_is_refused_by_the_real_binary ... ok` plus all six pure `n083_03` cases (closed/additive tokens, exact-line acceptance, malformed/duplicate/reordered/`reserve=granted`/wrong-size refusals, abnormal/reap/non-UTF-8/over-deadline classifications, fixed bounded evidence, over-ceiling magnitude).
- Retained regressions, all green and unchanged: `N076-01..03` Linux `getrlimit` tests; `N079` pre-exec smoke (`pid=4473 wall_ms=10`) and direct-child refusal (`exit=71`); `N081` fixed 2 000 ms wall deadline with real `SIGKILL` (`pid=4486 signal=9 reaped=1 wall_ms=2005`) and its direct-child refusal (`exit=71`). The `N079` 512 MiB/10 s profile, `N081` 2 s watchdog and `N083` 768 MiB request are untouched.

### 5.2 Windows - fail-closed negative

- `n083_04_both_reservation_modes_are_unsupported_before_any_child_exists ... ok`: both `as-parent` and `as-child` return status 3 with `HOST_UNSUPPORTED` and empty stdout before any child or reservation exists.
- All six platform-neutral `n083_03` cases plus `n083_03_every_wrong_reservation_token_is_refused_by_the_real_binary` pass; the original Windows `N079`/`N081` refusal tests pass.
- **Zero** occurrences of `n083_01_`, `n083_02_`, `n083_as=`, `n083_direct_as_child=`, `n079_01_the_real`, `n079_04_a_direct`, `n081_01_the_wall_parent` or `n081_03_a_direct` in the Windows log, and no N076 kernel test ran there.

### 5.3 Both-host architecture, security and downstream gates

| Gate | Ubuntu `131662443489` | Windows `131662443089` |
| --- | --- | --- |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Workflow `edges=41` fail-closed assertions | unchanged, satisfied | unchanged, satisfied |
| Refusal self-test | `self-check=passed cases=29` | `self-check=passed cases=29` |
| Dynamic-load / checker-bypass negatives | refused as designed | refused as designed |
| Pinned `1.99.1`, fmt, strict Clippy `-D warnings`, locked workspace tests | pass | pass |
| `cargo-deny 0.20.2` security gate | archive `9f12ed4c...`, RustSec `af7b3ec3...` fetched `2026-11-13T03:12:26Z`, advisories/bans/licenses/sources ok | archive `975a2214...`, same RustSec fetched `2026-11-13T03:12:44Z`, all ok |
| Failure tripwires (intentionally injected, expected) | detected, workflow green | detected, workflow green |
| Release build + CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | `release_smoke=verified accepted_exit=0 refused_exit=2` |
| Forbidden-import injection with exact restoration | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | host-inapplicable |
| Two independent clean release builds | `reproducibility=verified`, `clean_release_bytes=654096` | host-inapplicable |
| Raw checkout `Cargo.lock` SHA-256 | `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` (unchanged before/after the gate) | `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e` (unchanged before/after the gate) |

The single cross-host Git identity for the lockfile remains blob `b1ee56da60f36536548586528db371b7cbe508e4`; the two raw SHA-256 values differ only through the same line-ending mechanism described in §3. The intentionally injected failures inside negative tests are the workflow's own expected tripwires, not failing mandatory runs. No workflow, dependency graph, unsafe boundary or checker was changed, no new `unsafe` code exists, and the graph still has `edges=41`.

## 6. Report-only commit and final two-host readback - PASS (`S084-08`)

Only `.documents/Reports/execution-report.md` was overwritten with this one canonical report - no second report, copy, archive or extra WIP file - committed with the integration commit as its exact parent and pushed by ordinary fast-forward. GitHub readback of the commit, tree, parent, report blob and single changed file, `HEAD == main == origin/main`, and a second fresh push-triggered full `rust-verify` at that exact report HEAD are reported in the Implementation Agent's final handoff message, because this file cannot embed the identity of its own not-yet-created commit. At that final HEAD the two integrated source blobs were re-read and still equal the approved accepted branch blobs, and the workflow, `Cargo.lock`, native module and guards were re-read as unchanged.

## 7. Gate table

| Gate | Status | Evidence |
| --- | --- | --- |
| `S084-01` preflight | **PASS** | §1 - baseline/tree/report/ACTIVE and all §2 anchors exact; proof branch and its two blobs exact; one worktree/remote, four WIP files, only the installed ACTIVE dirty |
| `S084-02` ACTIVE-only issuance + first CI | **PASS** | §2 - `568d16ea...`, one path, remote readback, run `43356626242` green both hosts |
| `S084-03` literal two-blob promotion and five proofs | **PASS** | §3 - `M+M` with `780/5` and `212/0`, staged and worktree objects equal to `90ec7301...` / `30dffd08...` |
| `S084-04` two-file integration commit | **PASS** | §4 - `343f223f...`, parent = issuance, remote readback, accepted blobs, protected paths unchanged |
| `S084-05` new integration push-run + real Linux denial | **PASS** | §5.1 - run `43357351786` at the exact `main` SHA, `n083_as=... reserve=denied pid=4518 pgid=4518 wall_ms=10` |
| `S084-06` Windows negative + regressions | **PASS** | §5.2 - `n083_04 ... ok`, status 3 `HOST_UNSUPPORTED`, zero Linux-only execution; `N076`/`N079`/`N081` intact |
| `S084-07` architecture, refusal, security, downstream | **PASS** | §5.3 - `packages=4 edges=41`, `cases=29`, deny + fresh RustSec, tripwires, release smoke, restore, reproducibility |
| `S084-08` report-only commit + second CI | **PASS** | §6 - one path, parent = integration, remote blob readback, fresh two-host run at the report HEAD |
| `S084-09` preservation, cleanliness, cleanup | **PASS** | §8 - one worktree/remote, four WIP files, clean status, task scratch disposed, Owner scratch preserved, final blobs identical |
| `S084-10` narrow vocabulary and honest disclosure | **PASS** | Only `LINUX-RLIMIT-AS-RESERVATION-REFUSAL-INTEGRATED` is claimed; no containment, media, Stage or release claim; no self-acceptance |

**No failing or red CI run occurred in this order**: all three runs (issuance, integration, report HEAD) were green on both hosts on their first attempt, so there is no red iteration to disclose and no corrective commit exists. No gate is `FAIL`, `BLOCKED`, `UNVERIFIED` or `NOT RUN`.

## 8. Final state, cleanup, limitations and handoff (`S084-09`, `S084-10`)

**Repository.** Verified single Solo checkout, one worktree, exactly one remote `origin`, exactly four WIP filenames, clean index and working tree on `main`. Three commits were made: ACTIVE issuance `568d16ea...` (parent `5eb88278...`), the two-file integration `343f223f...` (parent `568d16ea...`) and this report-only commit (parent `343f223f...`) - four tracked paths total, exactly the permitted ledger. No `reset`, `clean`, stash, history amendment, force push, rebase, merge, cherry-pick, second worktree/clone or alternate remote was used, and no tag, release, deployment, PR or AVIF pipeline was touched.

**Preservation.** `solo-wo-083-r01` remains byte-for-byte preserved locally and on `origin` at `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`, tree `b299b005c03657344213632eb0e0e4623023f694`, with its own earlier ACTIVE `9f231565...` and its own prior report unchanged; nothing was pushed to it and it was not merged, cherry-picked or path-imported as a whole. `solo-wo-081-r01` (`cacc0f9b...`), `solo-wo-079-r01` (`380393dd...`), `solo-wo-076-r01` (`40a40bab...`), `solo-wo-061-r01` (`4371af21...`), `solo-wo-069-r01` (`e6162cdf...`) and `solo-wo-070-r01` (`d270f95c...`) are at their exact §2 SHAs.

**Cleanup.** This order created no task-owned temporary artifacts other than short-lived report-assembly scratch inside Solo `.temp/`, which was deleted immediately after assembly; `.temp/` was inspected directly and again has **zero** top-level files, with the inherited `architecture-fixtures/`, `solo-wo-073-wip/` (4 files / 46 062 bytes, still uncompiled and unverified) and `toolchain-installer/` directories plus the existing `target/` preserved untouched. No Owner, unknown or shared material was deleted or moved, and cleanliness was verified by direct inspection rather than inferred from `git status`.

**Limitations, stated truthfully.** The Owner-local Windows workstation still has no usable `link.exe`, so no local `cargo check`/`build`/`test` was possible: **all** compilation, lint, test, security and kernel evidence in this report comes from the remote two-host GitHub CI at the exact SHAs named, and only Git-level evidence (blob/numstat/status/readback) was produced locally. The worktree line-ending mapping described in §3 is a pre-existing host configuration; Git-visible content is byte-identical to the accepted blobs, but a reviewer will see CRLF bytes on this host and LF in the repository. This integration still proves only one refused capacity request: no RSS, page-touch, OOM, CPU-time, aggregate, descendant, sandbox, Job Object, decode, media-correctness, publication or release property follows from it, and `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

**Independent review.** The Implementation Agent has **not** self-accepted, has merged no other branch, published no AVIF output, released and deployed nothing, and has not begun another work order. The Chief Master Architect is asked to audit this canonical report, the three commits and their exact blob identities, and both green two-host runs, then return **ACCEPT / REVISE / STOP**.
