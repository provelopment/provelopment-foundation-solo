# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-081 R01` — prove a bounded Linux wall-deadline termination of one already-limited, trusted child (isolated technical-branch evidence only).
**Matching activation ID:** `SOLO-WO-081 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `LINUX-WALL-DEADLINE-LEADER-REAP-PROVEN`.**

One deliberately overdue, non-descending, trusted Linux child was started under the **same verified** pre-exec ceilings and new session, verified its own four ceiling halves and a positive `pgid == pid` **before** sleeping, and was then stopped by its **owning parent's 2 000 ms wall watchdog**. The parent killed only the `Child` handle it created, obtained a real signal termination **`SIGKILL = 9`** from that leader's wait status, and **reaped** it. The complete, unmodified two-host `rust-verify` workflow is green at the exact final code-branch SHA `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`.

**Nonclaims — these are load-bearing, not boilerplate.** Nothing here proves a child-**tree** sweep, any **process-group** signal, resistance to a hostile child, a combined or aggregate memory quota, `RLIMIT_AS` enforcement under stress, `RLIMIT_CPU` exhaustion (a 20-second *sleep* consumes no CPU — it exercises **wall time only**), Windows Job Objects, any process or container sandbox, executable identity or anti-replacement, filesystem or network confinement, AVIF decode, media correctness, publication or release. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and no implementation was merged into `main`. The child is trusted and forbidden to fork; the one signal that was sent came from the safe owned-handle `Child::kill`, never from a PID, a group or an external process.

## 1. Preflight — `S081-01` PASS

Single real checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, branch `main`, one worktree, one `origin` (`https://github.com/provelopment/provelopment-foundation-solo.git`), exactly four `.documents/WIP/` files, and **only** the Owner-installed ACTIVE dirty.

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `86ba671e354a3a5cac2d83e5248493312fdee62b` | exact |
| `HEAD^{tree}` | `80eeaf3caf88f2a88bc3a44a8abe9e7ccb120b7d` | exact |
| Canonical prior report blob | `346b8fe573dc61fa2e9ad051b1d08031730b1c02` | exact |
| Previous ACTIVE tracked blob | `c8ce3bb9c664a873bc200c6d7fdec2fdb8637a30` | exact |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| Workflow | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact, two `edges=41` gates |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / crate root | `47d732f2…` / `1550d34f…` | exact |
| Native readback/pre-exec file | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| Safe host façade | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | exact |
| CLI root | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| Host smoke integration test | `1f127001359e5f0ba941f92906f3280e433d8b9f` | exact |
| Unsafe guard / graph / workspace graph | `b88e7b42…` / `7a5beada…` / `25a91b2c…` | exact |
| `solo-wo-079-r01` / `solo-wo-076-r01` | `380393dd…` / `40a40bab…` | unmoved |
| `solo-wo-061-r01` / `solo-wo-069-r01` / `solo-wo-070-r01` | `4371af21…` / `e6162cdf…` / `d270f95c…` | unmoved |
| `solo-wo-081-r01` | **absent** locally and remotely | absent (0 `ls-remote` rows) |
| Prior report CI | run `43232869223`, ubuntu `131232468033`, windows `131232467666` | both success |

The Owner-installed ACTIVE was 136 lines starting `**OWNER-ISSUED — SOLO-WO-081 R01**` with three occurrences of the matching activation token, and its content governs the two-branch strategy used below. `.temp/` had no top-level files; its inherited directories and the `solo-wo-073-wip` four files / 46 062 bytes are preserved untouched and were not compiled, copied or executed. Local `target/` was left intact. The workstation still has **no linker** (`cargo check` fails on the `serde`/`zmij` build scripts: "the msvc targets depend on the msvc linker but `link.exe` was not found"), so branch CI is the approved compiler and test host exactly as §6 permits; `rustfmt 1.10.0` and `cargo 1.99.1` are available locally and were used for real formatting checks.

## 2. Issuance — `S081-02` PASS

| Step | Commit | Tree | Parent | Paths |
| --- | --- | --- | --- | --- |
| Issuance (`Issue SOLO-WO-081 R01: prove bounded Linux wall-deadline kill and reap`) | `e725c29a0b8daaf5939084abe8ed015a2b3002b9` | `aba482a7e6c16ce0f7ecc8f98d687a4b97d1200a` | `86ba671e354a3a5cac2d83e5248493312fdee62b` | 1 — ACTIVE only, blob `a9d2a84db8ef7cb2eb6d85d823aaefc0ecbf89b6` |

Ordinary fast-forward push; the GitHub API returned the same `sha`, `tree`, `parent` and the single changed file, and the remote ACTIVE blob was read back as `a9d2a84d…`. No report was included.

**Issuance CI — run `43236339897`, event `push`, `head_sha` = `e725c29a…`, conclusion `success`; `verify (ubuntu-24.04)` job `131244587031` and `verify (windows-2025)` job `131244587575`, 33 steps each, 0 failures.**

## 3. Technical branch and implementation — `S081-03`, `S081-04` PASS

`solo-wo-081-r01` was created **once**, in the same checkout, from the exact issuance SHA (`git switch -c solo-wo-081-r01 e725c29a…`) — no clone, no second worktree, no shadow source. Exactly **two** files changed in the whole branch:

| Path | Before (issuance) | After (final code SHA) | Numstat |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | `775 / 4` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1f127001359e5f0ba941f92906f3280e433d8b9f` | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | `255 / 0` |

Every other identity stayed byte-identical on the branch: `Cargo.lock` `b1ee56da…`, `media_worker_host_native.rs` `d10151e4…`, workflow `e1b5ca6e…`, unsafe guard `b88e7b42…`, `solo-cli/src/main.rs` `b3c89ad0…`, adapter `lib.rs` `1550d34f…`, adapter manifest `47d732f2…`, architecture graph `7a5beada…`. No third path was touched, no dependency or feature was added, and the native module was **not** edited — the existing `install_linux_child_limits_v1`, `read_limit_ceilings_v1` and `own_process_group_v1` are reused unchanged.

**Two closed sub-modes beneath the already reserved flag.** `wall-parent` and `wall-child` join `parent`/`child` in the same closed classifier, so every other token shape still refuses with the existing fixed `INTERNAL_MODE_INVALID`; neither token is reachable through the public subcommand, an environment variable, a manifest or parsed media. The composition root is unchanged: `main.rs` already dispatches `run_internal_media_host_probe_v1` first, so the new arms needed no CLI edit.

**The child (stress-before-limits rule).** `wall-child` performs the identical kernel readback first — both `RLIMIT_AS` halves at 536 870 912 and both `RLIMIT_CPU` halves at 10 — then reads `getpgid(0)` and requires a positive `pgid == own pid`. Only after those hold does it write **one** fixed `SOLO-MEDIA-HOST-WALL-V1 …` readiness line, **flush it**, and `std::thread::sleep` for exactly 20 000 ms. A direct, unsupervised invocation therefore refuses with one of the three pre-existing child statuses, prints **zero** stdout and never reaches the sleep. It forks nothing, allocates nothing, burns no CPU, opens no source, performs no media work, makes no network call and launches no other executable; its only zero-exit path is the fallback for a parent that violated its deadline, which the parent classifies as `HOST_EXIT_ABNORMAL` and never as a termination.

**The parent.** The `Instant` timer starts **before** `Command::spawn`; a bounded 10 ms poll loop watches the one child against a fixed **2 000 ms** deadline. On the deadline, while the handle is still live, it calls `Child::kill()` on **that handle only** and `Child::wait()` for the leader's real status. `status.signal()` is read through the safe `std::os::unix::process::ExitStatusExt`; the proof requires exactly `SIGKILL = 9` plus a completed reap. The exit-versus-timeout race is handled by an explicit outcome type: a child that finishes by itself is `CompletedBeforeDeadline` and refuses; a rejected kill is `KillFailed`; an unobservable status is `Unobservable`; only a successful kill-then-wait can carry a signal. Both capped readers are joined **after** leader termination, so a live child holding its own pipe ends cannot block the join. No process-group kill, no guessed PID, no second child and no descendant exist anywhere in the path.

**Platform-neutral refusal logic.** Parsing of the readiness line, the wall-interval bounds, the signal classification and the evidence formatting were deliberately placed in the neutral layer, and the Linux runner returns only raw material (`WallAttemptV1`: captured bytes, signal, reaped flag, measured wall time). Every refusal is therefore decided by ordinary checked Rust that is compiled and unit-tested on **both** hosts, and the same code carries no platform-conditional dead weight.

**Evidence marker.** Success prints exactly one line beginning `media_host_wall=terminated` — never `media_host_probe=accepted` — carrying the child's full-width ceilings, its positive pid, its matching pgid, `signal=9`, `reaped=1` and the parent-measured `wall_ms`. `signal` and `reaped` are written from the validated values, never from child text. Every non-proven outcome emits no evidence and one closed refusal line at status `3`, with no OS error text and no child output.

**`unsafe` inventory unchanged.** `media_worker_host_native.rs` still holds exactly its four Linux-gated `unsafe` expressions (two `getrlimit`/`assume_init` from `SOLO-WO-076 R01`, two `pre_exec`/`getpgid` from `SOLO-WO-079 R01`), each with an adjacent `// SAFETY:` rationale; its blob is unchanged. The façade contains **zero** `unsafe`, `libc`, `windows_sys`, `killpg`, `fork`, `prctl` or `ptrace` code (its only two matches for the word "unsafe" are prose in the module documentation), and the integration test contains none at all.

## 4. CI history — every iteration disclosed

| # | Branch commit | Tree | Run | Result |
| --- | --- | --- | --- | --- |
| 1 | `4aa96ea2239fd4b6cea91cf96b90c0457bd648eb` | `7cf9d9bced5b276f255a28a098699bcd78897a6a` | `43238905991` (`workflow_dispatch`) | **RED** — strict lint on both hosts |
| 2 | `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` | `2dabf3e5a5a9b3d2b067e627a3f63343296195c4` | `43240118731` (`workflow_dispatch`) | **GREEN** — both hosts, 0 failures |

**The red iteration, honestly reported.** Revision 1 (`4aa96ea2…`, two files, `769/4` and `255/0`) failed the `Strict lint (warnings are errors)` step on `verify (ubuntu-24.04)` job `131253655934` and `verify (windows-2025)` job `131253655164`. The single diagnostic was `error: this assertion has a constant value` at `crates/solo-adapters/src/media_worker_host.rs:1357` — `clippy::assertions_on_constants`, promoted to an error by `-D warnings`, on a runtime `assert!(WALL_DEADLINE_MS < WALL_CHILD_SLEEP_MS, …)`. The correction was one bounded commit inside the authorized file that turned the whole fixed-number drift guard into the `const { assert!(…) }` block Clippy itself suggested, which is strictly stronger: the compiler now evaluates those fixed numbers for every build on every host. No gate was weakened, none was skipped, no assertion was deleted, and the change stayed inside the two authorized paths.

**Final code-branch CI.** Run `43240118731`, `event=workflow_dispatch`, `head_sha` **exactly** `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` (verified equal to the branch tip before reading any job), conclusion `success`: `verify (ubuntu-24.04)` job `131257949845` and `verify (windows-2025)` job `131257949636`, 33 steps each, **0 failures**, only the documented host-inapplicable skips.

## 5. Real Ubuntu evidence — `S081-05` PASS

The test prints the actual validated observation to the CI log, exactly as `N079` already does:

```
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4489 pgid=4489 signal=9 reaped=1 wall_ms=2008
n081_wall_timing=wall_ms=2008 case_elapsed_ms=2018 signal=9 reaped=1
n081_direct_wall_child=exit=71
```

```
test n081_01_the_wall_parent_kills_and_reaps_its_own_overdue_child ... ok
test n081_02_the_termination_is_the_short_deadline_not_the_full_sleep ... ok
test n081_03_a_direct_wall_child_and_every_wrong_wall_token_are_refused ... ok
```

* **`N081-01`** — one pre-exec-limited child (`as_soft = as_hard = 536870912`, `cpu_soft = cpu_hard = 10`, full width, from the child's own `getrlimit`), `pid = pgid = 4489` both positive (so the child verified its own session/group identity **before** sleeping), `signal=9`, `reaped=1`, and no leaked stderr on the proven path.
* **`N081-02`** — the parent's own measurement is `wall_ms = 2008`, i.e. 8 ms past the configured 2 000 ms deadline and three orders of magnitude away from the child's 20 000 ms sleep; the whole case measured `2018 ms`. A normal completion could not have produced this: the child was still mid-sleep when it was killed. Reaping is evidenced by the `Ok(status)` returned from `Child::wait`, and the code path can produce a signal **only** from that kill-then-wait pair.
* **`N081-03`** — a direct `wall-child` exited `71` promptly with zero stdout and zero stderr, so it refused at its own limit/identity check and never slept; five wrong token shapes (`wall`, `WALL-CHILD`, duplicated `wall-parent`, `wall-child extra`, bare flag) each refused at status `3` with `INTERNAL_MODE_INVALID`; the legacy `child` mode kept its exact prior status `71` and empty streams.
* **Regressions** — `N076-01`, `N076-02`, `N076-03` and the real `N079-01` observation (`media_host_probe=accepted … pid=4470 pgid=4470 wall_ms=1`), `N079-03` (six cases incl. the real binary), `N079-04` (`exit=71`), `N079-06` and every protocol, source-reader and staging suite still pass.
* **Pure, both hosts** — `n081_01_…fixed_deadline_and_profile` (const guard), `n081_03_…closed_and_additive`, and four `n081_04` cases: fourteen malformed readiness lines (empty, marker-only, wrong marker, no trailing newline, missing, extra and duplicated fields, negative, `u64` overflow, second line, carriage return, doubled space, reordered fields), five out-of-profile lines (wrong ceiling, wrong CPU value, wrong pgid, `pid=0`, `u64::MAX` ceilings), every non-`SIGKILL` signal plus a normal exit and an un-reaped leader, both wall-interval violations, and the fixed evidence wording that must never contain `media_host_probe=accepted`.

## 6. Windows and both-host gates — `S081-06`, `S081-07` PASS

On `windows-2025` both new modes refuse **before any child exists**: `test n081_05_both_wall_modes_are_unsupported_before_any_child_exists ... ok`, each returning status `3` with `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED`, empty stdout and a prompt return (no deadline is ever waited for). The Windows log contains **zero** occurrences of `n081_01_the_wall_parent…`, `n081_02_…`, `n081_03_a_direct…`, `n081_wall=`, `n079_01_the_real…`, `n079_04_…` or `n076_…`: no Linux native module, no real Linux wall test and no kernel readback was compiled or executed there. The six pure `n081` cases and `n079_05`/`n079_06` did run and pass, and the public CLI still yields `preflight=ok entity_id=entity-7 base=https://example.com/alpha` at exit `0` and `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` at exit `2` byte-exactly on both hosts.

| Gate | Ubuntu `131257949845` | Windows `131257949636` |
| --- | --- | --- |
| Format, strict Clippy (`-D warnings`), all-target tests | pass | pass |
| Pinned toolchain `1.99.1` | asserted | asserted |
| Component security gate (`cargo-deny 0.20.2`) | archive sha256 `9f12ed4c…`, RustSec `af7b3ec33460b8e2de8d422e16d628518891a519` fetched `2026-11-12T14:43:33Z`, all ok | archive sha256 `975a2214…`, same RustSec commit fetched `2026-11-12T14:44:01Z`, all ok |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Refusal self-test | `self-check=passed cases=29`, dynamic-load and checker-bypass still refusing | `self-check=passed cases=29` |
| Test-failure tripwire | normal pass plus required injected failure | same |
| Release build + CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | `release_smoke=verified accepted_exit=0 refused_exit=2` |
| Forbidden import / restore | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffe…` | not applicable (Ubuntu-only step) |
| Two clean rebuilds | `reproducibility=verified`, `clean_release_bytes=641112` | not applicable (Ubuntu-only step) |

The graph stayed at **four crates and 41 edges**; `targets=33` and `files=60` are unchanged because no file was added — the two new sub-modes live inside existing files. The release binary grew from 623 472 to 641 112 bytes, which is exactly the added code, and two independent clean builds still compared byte-identically.

## 7. Gate table — stated honestly

| Gate | Status | Basis |
| --- | --- | --- |
| `S081-01` | **PASS** | §1 — every anchor and blob exact; clean tree except the Owner ACTIVE; four WIP files; one checkout/remote/worktree; branch absent |
| `S081-02` | **PASS** | §2 — ACTIVE-only issuance `e725c29a…`, remote readback exact, both-host CI green |
| `S081-03` | **PASS** | §3 — one branch from the issuance SHA, only the two authorized files changed, older branches unmoved |
| `S081-04` | **PASS** | §3 — limits and group verified before any sleep, no descendant or untrusted execution, no new `unsafe`, native module byte-identical |
| `S081-05` | **PASS** | §5 — real Linux kill/reap plus pure refusals, with bounded CI-visible readback, signal and wall evidence |
| `S081-06` | **PASS** | §6 — Windows negative and unchanged legacy public/private CLI output |
| `S081-07` | **PASS** | §4 — full original two-host workflow green at the exact final branch SHA, all downstream checks and `N076`/`N079` regressions |
| `S081-08` | **PASS** | §8 — branch preserved at `cacc0f9b…`, **not** merged; `main` still at the issuance SHA; older branches unchanged |
| `S081-09` | **PASS** | §8 — one report-only commit, final two-host `main` CI green, clean one-checkout/one-remote/four-WIP end state, scratch cleaned |
| `S081-10` | **PASS** | narrow vocabulary only; complete transparent CI history; no fabricated enforcement, security, media, publication or Stage 03 claim; no self-ACCEPT |

Nothing in this order reached `FAIL` or `NOT RUN` at its final revision. One red iteration occurred and is disclosed in §4 with its exact diagnostic, job IDs and correction.

## 8. Final state, limitations and cleanup

**Repository.** `main` is back at the **issuance** SHA `e725c29a0b8daaf5939084abe8ed015a2b3002b9` (tree `aba482a7…`) with no implementation merged, cherry-picked, rebased or patched onto it; this report is the only new `main` commit and is pushed as an ordinary fast-forward on top of it. `solo-wo-081-r01` is preserved at `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` (`origin` agrees), and `solo-wo-079-r01` (`380393dd…`), `solo-wo-076-r01` (`40a40bab…`), `solo-wo-061-r01` (`4371af21…`), `solo-wo-069-r01` (`e6162cdf…`) and `solo-wo-070-r01` (`d270f95c…`) are byte-for-byte unchanged. One worktree, one `origin`, four WIP files, no branch beyond the authorized one, no pull request, tag, release or deployment.

**Toolchain limitation.** This Owner-local workstation still cannot link — `cargo check` fails in dependency build scripts because `link.exe` is absent — so every compile, lint, test and kernel statement above comes from the two-host CI at the exact SHAs named. `cargo fmt --all -- --check` and `git diff --check` were run locally and pass on the branch and on `main`.

**Cleanup.** The only task-owned material this run created was disposable report scratch in the ignored `.temp/`, deleted after assembly, so `.temp/` again has no top-level files. The inherited `architecture-fixtures`, `solo-wo-073-wip` (four files / 46 062 bytes) and `toolchain-installer` directories are preserved exactly as found; that scratch remains uncompiled, unverified reference only. No Owner or unknown file was deleted, moved or reinterpreted, and no `git reset`, `git clean`, stash, force-push or destructive checkout was used. Local `target/` was left as it was.

**Remaining risks.** A process-group or descendant sweep, hostile-child resistance, aggregate memory quotas, `RLIMIT_AS`/`RLIMIT_CPU` exhaustion, Windows Job Objects, container sandboxing, executable identity and anti-replacement, filesystem/network confinement, decoder quality, media correctness and whole-publication validation all remain **future Owner-issued work**. This report proves one killed-and-reaped leader of one benign, trusted, descendant-free child and nothing more; it must never be described as host containment, and it does not close `R03.C3`, `R03.C` or Stage 03.

The Implementation Agent has not self-ACCEPTed, has not merged, released or deployed anything, and has not begun another order. It stops here for independent Chief Master Architect disposition: **ACCEPT / REVISE / STOP**, against branch `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` and this report commit.
