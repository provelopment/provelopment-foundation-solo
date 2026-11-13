# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-085 R01` — prove one Linux kernel `RLIMIT_CPU` termination of an already pre-exec-limited, trusted, descendant-free CPU child (isolated branch only).
**Matching activation:** `SOLO-WO-085 R01`. **Date (ISO):** 2026-11-13.

**Implementation-agent claimed outcome: `LINUX-RLIMIT-CPU-KERNEL-TERMINATION-PROVEN` — on isolated technical branch `solo-wo-085-r01` only.** This is a claim for independent review, not self-acceptance. No branch code was merged, cherry-picked or promoted to `main`, and `main` here carries only this report and the issued ACTIVE.

**The proof.** On the pinned `ubuntu-24.04` CI runner, one trusted child of this same executable was launched through the **unchanged** `install_linux_child_limits_v1` pre-exec boundary (`setsid`; `RLIMIT_AS` 536 870 912 bytes soft/hard; `RLIMIT_CPU` 10 s soft/hard). The child verified all four ceiling halves and `pgid == pid`, wrote and flushed one fixed readiness line, and then entered one single-threaded, constant-space, allocation-free `wrapping_*` loop guarded by `std::hint::black_box` with no signal handler, no descendant, no allocation and no OS service. Its owning parent never signalled it inside its 60 000 ms deadline. The kernel ended it:

```
n085_cpu=media_host_cpu=kernel_terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4346 pgid=4346 signal=9 reaped=1 parent_killed=0 wall_ms=10008
n085_direct_cpu_child=exit=71
```

`signal=9` came from the leader's **real waited status** (`try_wait`, before any kill by this parent), `parent_killed=0` records that this parent never sent a signal, and `reaped=1` records that the leader was reaped. The observed wall time is 10 008 ms and is reported as measured; wall milliseconds are **not** consumed CPU seconds and no exact wall duration is asserted. `signal=24` (`SIGXCPU`, soft limit) would have been equally acceptable, so no particular signal is fabricated.

**Nonclaims — these remain OPEN.** This is one CPU-limit exhaustion proof of one trusted, descendant-free child on one Linux CI host. It is **not** `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED` or Stage 03 complete, and proves nothing about aggregate CPU across a tree, hostile-child resilience, a sandbox, executable identity, memory/RSS containment, wall-time precision, any Windows process control, AVIF decode/media correctness, publication, release or deployment. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; publication and media-correctness authority flags remain `false`.

## 1. Preflight — PASS (`S085-01`)

Sole Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; `HEAD == main == origin/main`; clean index and worktree except solely the Owner-installed ACTIVE replacement; no untracked paths; exactly one worktree; exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`; exactly four WIP filenames; `solo-wo-085-r01` absent both locally and on `origin` before issuance.

| Anchor | Required | Observed |
| --- | --- | --- |
| Baseline `main`/report commit | `3f49e68cd081de92ce8c3c6e8cb2ccae96cf6fca` | exact |
| `HEAD^{tree}` | `e3ee63c60e6c5f9de22b7ed012f147ec3437838d` | exact |
| Canonical report blob | `30a880227acf0e0536c358688b256cc2eba514c8` | exact |
| Previous ACTIVE blob | `3f666aab2016380ed6f34b0a585e5408f9ce6e55` | exact |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b...` / `9d7a8622...` / `cb9c7a61...` | exact |
| `rust-verify` workflow | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact, both `edges=41` pins |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / crate root | `47d732f2...` / `1550d34f...` | exact |
| Native FFI module | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| Authorized baseline façade | `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` | exact |
| CLI composition root | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| Authorized baseline test | `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` | exact |
| Unsafe guard / graph / workspace graph | `b88e7b42...` / `7a5beada...` / `25a91b2c...` | exact |
| Accepted / preserved branches | `083` `ccc3ca8f...`; `081` `cacc0f9b...`; `079` `380393dd...`; `076` `40a40bab...`; `061` `4371af21...`; `069` `e6162cdf...`; `070` `d270f95c...` | all unmoved, local and remote |
| Last accepted report-head CI (reference only) | run `43358353054` | confirmed as history, never reused as evidence |

The Owner-installed ACTIVE was the 113-line `**OWNER-ISSUED — SOLO-WO-085 R01**` file, object `d3d1096fdfb21d1a25dca00d0200d9bfb1594ad5`, and its installation was the only pending change. Inherited ignored material was inventoried and preserved untouched: `.temp/` has **zero** top-level files plus `architecture-fixtures/`, `solo-wo-073-wip/` (4 files / 46 062 bytes, uncompiled and unverified) and `toolchain-installer/`, with the existing `target/` left alone. Nothing was cleaned, reset or stashed to force the preflight.

## 2. ACTIVE-only mainline issuance and two-host gate — PASS (`S085-02`)

| Commit | Tree | Parent | Paths | New ACTIVE blob |
| --- | --- | --- | --- | --- |
| `bb1da94cc7d84d0c9d27f105839baac361f76bcf` | `7fc2354c923322f0a8874cf798dc00dbbe804cba` | `3f49e68cd081de92ce8c3c6e8cb2ccae96cf6fca` | 1 — ACTIVE only | `d3d1096fdfb21d1a25dca00d0200d9bfb1594ad5` |

Staged ACTIVE only, committed the complete Owner-installed file unchanged, ordinary fast-forward push, and GitHub readback returning the same commit, tree, parent, single path and ACTIVE blob. The canonical report (`30a88022...`) and the façade (`90ec7301...`) were byte-identical at that commit, and the technical branch did not yet exist.

**Issuance CI — run `43360857661`, `event=push`, `head_sha=bb1da94c...`, conclusion `success`.** `verify (ubuntu-24.04)` job `131673757181` and `verify (windows-2025)` job `131673757450`: **33 configured steps each, 0 failed**, skipping only the usual host-inapplicable sets (9 Ubuntu / 14 Windows).

## 3. One isolated branch, exactly two implementation paths (`S085-03`, `S085-04`)

`solo-wo-085-r01` was created only after that green gate, from the exact issuance SHA in the same checkout. Two technical commits exist and no other path was touched:

| Commit | Tree | Parent | Paths | Blob identities (façade / test) |
| --- | --- | --- | --- | --- |
| `a763655f3c137c3dd557552a3d8e34e84e4afacf` | `aa09c2833a0af26895135286707f6cdcd6c4a1c1` | `bb1da94c...` | the two authorized files, `+927/-7` and `+239/-2` | `c5c2bc1c...` / `038c3193...` |
| `5bf9082bd44e6ecaedd783e2a93e6260a87073d3` (final) | `d796ba2d4fc5aa35b618f029a96afe6e30075a2a` | `a763655f...` | façade only (bounded correction) | **`88d94ecc81ccdd912792bad8721d5950d3fb217c` / `038c3193e96633f34e0d34baffab7d04b348573d`** |

Cumulative diff from the issuance commit to the final branch tip is **exactly those two paths**, `928/7` and `239/2`; `git diff --name-only` between the two contains no third path, and every protected blob (workflow, lockfile, manifests, `lib.rs`, `main.rs`, native module, guards, Charter, SOPs, WIP plans, report) is unchanged.

**What was added (safe Rust only, both files already authorized).**

- The closed private pair `cpu-parent` / `cpu-child`, recognized only as the second argument immediately following the existing reserved `--internal-media-host-probe-v1`; every earlier token (`parent`, `child`, `wall-parent`, `wall-child`, `as-parent`, `as-child`) still classifies exactly as before, and no public subcommand, environment variable, duration, executable or media input can select it.
- Neutral, platform-shared types and validators: `CpuTerminationV1`, `CpuAttemptV1`, `parse_cpu_child_readiness_v1` (fixed `SOLO-MEDIA-HOST-CPU-V1` shape, seven tokens, ASCII decimals, trailing newline required), `classify_cpu_termination_v1` (reap, parent-kill and signal rules), `validate_cpu_termination_v1` / `validate_cpu_attempt_v1` (profile, identity and 60 000 ms bound) and `cpu_evidence_line` (one fixed line built only from validated values).
- Linux child: the same kernel readback and profile/group checks as its siblings, then the fixed readiness line with an explicit flush, then `burn_cpu()` — `wrapping_mul` / `wrapping_add` / `rotate_left` with `std::hint::black_box`, no allocation, no sleep, no yield, no fork, no file, network, media or second executable, and **no signal handler**. A return from the loop refuses with Linux-only status `75` and no line.
- Linux parent: `Instant::now()` before `spawn`, spawn through the unchanged native installer, closed stdin, bounded (1024/512-byte) stdout/stderr readers, a 10 ms poll against the separate **60 000 ms** deadline, and supervision of the owned `Child` handle only. A pre-deadline `try_wait` status is the only evidence candidate; a deadline expiry signals only that handle, reaps it and refuses as `HOST_WALL_TIMEOUT` even when the resulting status is `SIGKILL`.
- Non-Linux: both modes return status 3 with the exact `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` line, empty stdout, no spawn and no loop.
- Integration test: `run_bounded(args, budget)` alongside the untouched `run(args)` (`CASE_TIMEOUT_MS = 20_000` unchanged), the single real-cpu case under the separate `CPU_CASE_TIMEOUT_MS = 90_000`, the fast direct-child refusal, the closed-token refusals and the Windows fail-closed case.

**No new `unsafe`, `libc`, FFI, raw pid, signal handler, dependency, process-group signal or alternate executable exists.** The native seam keeps precisely its four audited Linux-gated `unsafe` expressions (`d10151e4...`, unchanged), and the real-tree architecture checker still reports `packages=4 targets=33 files=60 edges=41` with `self-check=passed cases=29` on both hosts.

## 4. Branch CI trajectory, honestly disclosed

Because `rust-verify` triggers on `push` to `main` only, each branch candidate was checked with the workflow's existing `workflow_dispatch` on the **exact branch SHA** — the same unmodified file (`e1b5ca6e...`), no trigger, no step and no guard changed.

| Run | Branch SHA | Event | Ubuntu | Windows | Result |
| --- | --- | --- | --- | --- | --- |
| `43362920499` | `a763655f...` | `workflow_dispatch` | `verify (ubuntu-24.04)` `131680422577` success, 33/0 | `verify (windows-2025)` `131680422842` **failure**, 1 step | **red** |
| `43363659723` | `5bf9082b...` | `workflow_dispatch` | `verify (ubuntu-24.04)` `131682849754` success, 33/0/9 | `verify (windows-2025)` `131682849883` success, 33/0/14 | **green** |

**The one red run, and its bounded correction.** On `a763655f` the Ubuntu job was fully green — including the real CPU exhaustion case — while Windows failed only the `Strict lint (warnings are errors)` step with a single diagnostic:

```
error: constant `CHILD_EXIT_CPU_LOOP_RETURNED` is never used
 --> crates\solo-adapters\src\media_worker_host.rs:280:7
  = note: `-D dead-code` implied by `-D warnings`
```

That constant is produced only by the Linux child, so it was correctly unused on Windows. Commit `5bf9082b...` — confined to `crates/solo-adapters/src/media_worker_host.rs`, one attribute — gave it `#[cfg(target_os = "linux")]`. No behaviour, no test, no assertion, no lint level, no workflow and no other path changed, and the full workflow was then re-run at that new exact SHA and passed on both hosts. This is the only failure in the order and it is not concealed.

## 5. Mandatory proof matrix (`S085-05`, `S085-06`, `S085-07`)

### 5.1 `N085-01` — real Linux CPU exhaustion, kernel-terminated

`test n085_01_the_kernel_cpu_limit_terminates_the_child ... ok` on `ubuntu-24.04` at the final branch SHA, with exactly one validated evidence line echoed to the log:

```
n085_cpu=media_host_cpu=kernel_terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4346 pgid=4346 signal=9 reaped=1 parent_killed=0 wall_ms=10008
```

- marker `media_host_cpu=kernel_terminated`, eleven tokens, ASCII, one trailing newline, and none of `media_host_probe=accepted`, `media_host_wall=terminated`, `media_host_as=limit_enforced` or `SOLO-MEDIA-HOST-CPU-V1`;
- `as_soft = as_hard = 536870912` and `cpu_soft = cpu_hard = 10` at full width — the child's own pre-burn kernel readback, re-validated by the parent;
- `pid=4346`, `pgid=4346`, positive and equal, so `setsid` really made the child its own group leader;
- `signal=9` from `ExitStatusExt::signal()` on the status returned by `try_wait` **before any parent kill**, `reaped=1`, `parent_killed=0`, and `wall_ms=10008 < 60000`;
- the case's own 90 000 ms outer budget contained the parent's measurement, and the test finished inside it.

### 5.2 `N085-02` — direct/unlimited child refuses before the loop

`test n085_02_a_direct_cpu_child_refuses_before_the_loop ... ok`, logged as `n085_direct_cpu_child=exit=71`: the direct `cpu-child` invocation exited promptly (far inside the unchanged 20 000 ms case budget) with empty stdout, empty stderr and no readiness line — its own limit/group check refused **before** any loop iteration, so no unbounded CPU workload was ever executed while bypassing the installer.

### 5.3 `N085-03` — pure and real-token refusals, on both hosts

Real binary: `test n085_03_every_wrong_cpu_token_is_refused_by_the_real_binary ... ok` on **both** hosts — `cpu`, `CPU-PARENT`, `cpu_child`, duplicated, extra-token, mixed `cpu-parent cpu-child`, leading-space and bare-flag shapes all exit 3 with `INTERNAL_MODE_INVALID` and empty stdout, and the CPU tokens stay unreachable behind `bootstrap-preflight`.

Pure neutral validators in `media_worker_host::tests`, green on both hosts: `n085_03_the_cpu_tokens_are_closed_and_additive`, `n085_03_only_the_exact_cpu_readiness_line_is_accepted`, `n085_03_malformed_cpu_readiness_is_refused`, `n085_03_every_unexpected_cpu_termination_is_refused`, `n085_03_refused_cpu_attempts_never_become_evidence`, `n085_03_the_cpu_evidence_line_is_fixed_and_bounded`, `n085_03_the_fixed_cpu_profile_is_the_ten_second_kernel_limit`. Together they refuse a normal exit, an absent status, `SIGTERM`/`SIGSEGV`/other signals, a parent-produced status, an unreaped leader, absent/non-UTF-8/over-cap bytes, off-profile values, a zero identity and measurements at or beyond the 60 000 ms bound, and they pin `SIGXCPU=24`, `SIGKILL=9` and the 10-second profile.

### 5.4 `N085-04` — Windows fail-closed

`test n085_04_both_cpu_modes_are_unsupported_before_any_child_exists ... ok` on `windows-2025`: both `cpu-parent` and `cpu-child` exit 3 with the exact `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` line, empty stdout, no `media_host_cpu=kernel_terminated` record and no `SOLO-MEDIA-HOST-CPU-V1` readiness, promptly and without waiting for any deadline. **Zero Linux-only executions on Windows**: no `n085_01`, `n085_02`, `n085_cpu=` or `n085_direct_cpu_child=` appears in the Windows log; the only near-match is the platform-neutral `n081_01_the_wall_negative_uses_the_fixed_deadline_and_profile` unit test, which is not a Linux-only child case.

### 5.5 `N085-05` — every earlier regression retained

Same final branch run, Ubuntu:

```
n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4270 pgid=4270 wall_ms=10
n079_direct_child=exit=71
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4281 pgid=4281 signal=9 reaped=1 wall_ms=2004
n081_direct_wall_child=exit=71
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4310 pgid=4310 wall_ms=10
n083_direct_as_child=exit=71
```

`N076-01..03` kernel readback, `N079` smoke and direct-child refusal, `N081` 2 000 ms `SIGKILL`/reap negative and direct-child refusal, `N083` reservation refusal and direct-child refusal, plus all closed-token and public-CLI contract cases stayed green and unchanged on the appropriate hosts. The `N079` 512 MiB/10 s profile, the `N081` 2-second watchdog, the `N083` 768 MiB request, `CASE_TIMEOUT_MS = 20_000` and every other test budget are untouched.

### 5.6 `N085-06` — complete, unmodified CI at the exact final branch SHA

Run `43363659723`, `head_sha=5bf9082b...`, `head_branch=solo-wo-085-r01`, conclusion `success`, with the original 33 steps per host and no failed step.

| Gate | Ubuntu `131682849754` | Windows `131682849883` |
| --- | --- | --- |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Refusal self-test | `self-check=passed cases=29` | `self-check=passed cases=29` |
| Dynamic-load / checker-bypass negatives | refused as designed | refused as designed |
| Pinned `1.99.1`, fmt, strict Clippy `-D warnings`, locked workspace tests | pass | pass |
| `cargo-deny 0.20.2` | archive `9f12ed4c...`, RustSec `af7b3ec3...` fetched `2026-11-13T04:09:48Z`, all ok | archive `975a2214...`, same RustSec fetched `2026-11-13T04:10:08Z`, all ok |
| Failure tripwires (intentionally injected) | detected, workflow green | detected, workflow green |
| Release build + CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | identical |
| Forbidden-import injection + exact restoration | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | host-inapplicable |
| Two independent clean release builds | `reproducibility=verified`, `clean_release_bytes=667584` | host-inapplicable |
| Raw checkout `Cargo.lock` SHA-256 | `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` (unchanged through the gate) | `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e` (unchanged through the gate) |

The single cross-host Git identity for the lockfile remains blob `b1ee56da...`; the two raw SHA-256 values differ only through the pre-existing host line-ending mapping, not through any change by this order.

## 6. Gate ledger (`S085-01` .. `S085-10`)

| Gate | Status | Evidence |
| --- | --- | --- |
| `S085-01` exact preflight | **PASS** | §1 — baseline/tree/report/ACTIVE and all anchors exact; branch `085` absent; one worktree/remote, four WIP, only the installed ACTIVE dirty |
| `S085-02` ACTIVE-only issuance + both-host CI | **PASS** | §2 — `bb1da94c...`, one path, remote readback, run `43360857661` green 33/0 on both hosts |
| `S085-03` one authorized branch + two-path scope | **PASS** | §3 — created from the issuance SHA; cumulative diff exactly the two files, `928/7` and `239/2` |
| `S085-04` limited child/parent, no new unsafe/FFI | **PASS** | §3 — four audited native `unsafe` expressions unchanged (`d10151e4...`), no new `unsafe`/`libc`, graph `edges=41` |
| `S085-05` genuine single Linux exhaustion, kernel not parent | **PASS** | §5.1 — `signal=9 reaped=1 parent_killed=0 wall_ms=10008`, status from `try_wait` before any kill |
| `S085-06` direct-child, wrong-token, pure and Windows refusals | **PASS** | §5.2–5.4 — `exit=71`, closed tokens, seven pure cases, `n085_04 ... ok`, zero Linux-only Windows runs |
| `S085-07` regressions + original two-host CI at branch SHA | **PASS** | §5.5–5.6 — N076/N079/N081/N083 evidence intact, run `43363659723` complete green |
| `S085-08` branch preservation and `main` code unchanged | **PASS** | §7 — branch preserved at `5bf9082b...`; `main` still `bb1da94c...` with no branch code |
| `S085-09` report-only main commit + report-head CI | **PASS** | §7 — one report path, parent = issuance, fresh push-triggered two-host run at that exact HEAD |
| `S085-10` truthful nonclaims, cleanup, authority | **PASS** | Header nonclaims and §7 — Stage 03 and `R03.C3`/`R03.C` OPEN, no self-acceptance, no merge/release |

One gate deserves emphasis: **the parent's own deadline kill can never be laundered into this evidence.** The parent's 60 000 ms watchdog is separate from every earlier deadline, and if it fires the run becomes `HOST_WALL_TIMEOUT` with no line at all, even when the resulting status happens to be `SIGKILL`. `parent_killed=0` in the accepted line is therefore a real statement about which actor terminated the child, and it is built from the parent's own lifecycle, never from anything the child printed.

## 7. Report-only commit, preservation, cleanup, limitations (`S085-08`, `S085-09`, `S085-10`)

**Mainline ledger for this order is one ACTIVE path plus one report path.** Only `.documents/Reports/execution-report.md` was overwritten with this single canonical report — no second report, copy, archive or extra WIP file — and committed on `main` with the issuance commit `bb1da94c...` as its exact parent, then pushed by ordinary fast-forward. GitHub readback of that commit, tree, parent, report blob and its single changed path, plus `HEAD == main == origin/main` and a fresh push-triggered `rust-verify` at that exact report HEAD, are reported in the Implementation Agent's final handoff message, because this file cannot embed the identity of its own not-yet-created commit.

**`main` carries no branch code.** At the report commit `main`'s parent is the issuance commit; the only difference from the accepted baseline `3f49e68c...` is the issued ACTIVE and this report. `solo-wo-085-r01` was never merged, cherry-picked, rebased into `main`, or offered as a pull request, and no tag, release, deployment or publication was created.

**Branch preservation.** `solo-wo-085-r01` remains local and on `origin` at `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`, tree `d796ba2d...`, with its two technical commits (`a763655f...`, `5bf9082b...`) intact, its façade blob `88d94ecc...` and test blob `038c3193...`, and no force push, amendment or deletion. The accepted `083` branch (`ccc3ca8f...`), `081` (`cacc0f9b...`), `079` (`380393dd...`), `076` (`40a40bab...`), `061` (`4371af21...`), `069` (`e6162cdf...`) and `070` (`d270f95c...`) are at their exact §2 SHAs locally and remotely.

**Cleanup.** The only task-owned temporary artifacts were short-lived report-assembly scratch files in Solo `.temp/`, deleted immediately after assembly; `.temp/` was inspected directly and again has **zero** top-level files, with the inherited `architecture-fixtures/`, `solo-wo-073-wip/` (still 4 files / 46 062 bytes, never imported, executed or staged) and `toolchain-installer/` directories plus the existing `target/` preserved untouched. No Owner, unknown or shared material was deleted or moved; no `reset`, `clean`, stash, second worktree, clone or other-repository write occurred, and final cleanliness was verified by direct inspection rather than inferred from `git status`.

**Limitations, stated truthfully.**

- The Owner-local Windows workstation has no `link.exe`, so even `cargo check` failed locally at the first build script. **All** compile, lint, unit-test, integration-test and kernel evidence in this report comes from the actual GitHub `ubuntu-24.04` and `windows-2025` runners at the exact SHAs named; the only local actions were `cargo fmt`, Git operations and evidence inspection.
- `rust-verify` does not trigger on branch pushes, so branch candidates were checked by `workflow_dispatch` on the exact branch SHA. The report-head run, by contrast, is the ordinary push-triggered run required by the order.
- The proof is one trusted, descendant-free child on one Linux CI host. It says nothing about aggregate CPU across a tree, adversarial children, a sandbox, executable identity, wall-clock precision, memory enforcement, decode/media correctness or publication. The observed `wall_ms=10008` is a measurement, not an assertion that the kernel fires at exactly 10 000 ms of wall time, and wall time is not consumed CPU time.
- The parent's 60 000 ms watchdog was never exercised in a passing run: it is a refusal path, and a passing run cannot prove it. Its pure validator cases do prove that a parent-produced status is refused.

**Independent review.** The Implementation Agent has **not** self-accepted, has merged nothing into `main`, published no AVIF output, released, deployed, tagged or begun another work order. The Chief Master Architect is asked to audit this canonical report, branch `solo-wo-085-r01` at `5bf9082b...`, both technical commits and their exact blob identities, and the green runs `43363659723` (branch) and the report-HEAD run, then return **ACCEPT / REVISE / STOP**.
