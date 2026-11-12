# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-070 R01` — R03.C3 AVIF recovery: prove hard OS-enforced decoder isolation on Linux and Windows (non-product).
**Matching Owner→Implementation Agent handoff:** `SOLO-WO-070 R01`.
**Report date (ISO):** 2026-11-12 (run window 2026-11-12T05:05–05:5xZ).
**Decision: `OS-ISOLATION-PROVEN`** — for the bounded experimental harness only. On **both** `ubuntu-24.04` and `windows-2025`, every mandatory containment control was installed **before the workload's first instruction**, was read back from the operating system, and demonstrably denied deliberate memory, CPU, wall-time and descendant escapes; the accepted 12-fixture / 13-negative corpus then reproduced its exact decoded-plane digests inside those limits. **No decoder is approved, no product code or contract changed, `R03.C3`/`R03.C`/Stage 03 remain OPEN.**

## 1. Anchors, ledger and CI history

| Anchor | Value |
| --- | --- |
| Starting `main` / tree | `4372c9e36e852cd28215aa2b53c2dcc8387b8e5b` / `b683f17fd999cc06db2d31a1df05e0a193711bd7` |
| Starting report blob | `a891ad95ea0c6f7958bbb1c5434fb8db8a3e9503` (`SOLO-WO-069 R01`, `BLOCKED`) |
| Baseline WIP | ACTIVE `7af728e7…`, NEXT `92fa562b…`, ROADMAP `9d7a8622…`, OUTLOOK `cb9c7a61…` |
| **Issuance** | `e8a90421f9577bb000642ad2712da25265fe1b82`, tree `e44485ae954937c8b9b0fcd8e25beb9d07775265`, parent `4372c9e3…`, **ACTIVE alone** → `0d3edb13…` |
| Issuance CI (`rust-verify`) | run `43157117182` **success**, both hosts |
| **Proof branch** | `solo-wo-070-r01` — 8 commits, HEAD `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`, tree `561afc065969448ae2b4774ed41e961ae0f1fce5`, local `==` `origin/solo-wo-070-r01` |
| **Branch CI** | run `43162966403` **success** — jobs `130998200678` (`ubuntu-24.04`) and `130998200950` (`windows-2025`), 24 steps each, 0 failures |
| Preserved | `solo-wo-061-r01` = `4371af21…`, `solo-wo-069-r01` = `e6162cdf…` (unchanged, local and GitHub) |

**Import and ledger.** `git restore --source=e6162cdf… --staged --worktree --` restored exactly the five WO-069 proof paths; all five staged blobs were verified byte-identical to the source commit before commit `c1fe36e`. The branch then changed exactly the seven paths this order allows — `Cargo.toml` (unchanged content), `Cargo.lock` (unchanged content), `src/lib.rs` (unchanged content), `src/main.rs`, `limit-linux.sh` (new), `limit-windows.ps1` (new), `.github/workflows/avif-qualification.yml` — 3,658 insertions against the issuance revision, **zero product paths**. No PR, merge, reset, clean, stash, amend, rebase, force-push or second checkout.

**Red attempts, honestly listed.** `43158770912` — Linux child did not yet publish its own limit readback and the descendant probe exited voluntarily (no wall kill). `43159301411` — Linux 20/20 green; Windows aborted after T02 because PowerShell parses the hex literal `0xC0000409` as a negative `Int32`, so casting it to `UInt32` threw. `43160070536` — Windows reached 15/17; parallel probes produced no record (`Environment.TickCount64` does not exist on the .NET Framework host they inherited). `43160786070`, `43161558074`, `43162276350` — successive parallel-probe fixes (same-host spawn, clean record files, pre-quoted argument line). **`43162966403` — both hosts green.** Every fix touched only allowed proof paths; no limit, threshold, pin, licence allowlist or policy was ever weakened to obtain green.

## 2. How containment is enforced

### Linux — `verification/avif-r03-c3/limit-linux.sh`

A `bash` launcher that, **before the contained program executes anything**, installs hard *and* soft kernel limits in a fresh session and then `exec`s the workload:

```bash
setsid bash -c '
  ulimit -Sv "$PROBE_MEM_KIB" || exit 90   # RLIMIT_AS soft   (524288 KiB = 512 MiB)
  ulimit -Hv "$PROBE_MEM_KIB" || exit 90   # RLIMIT_AS hard
  ulimit -St "$PROBE_CPU_SEC" || exit 90   # RLIMIT_CPU soft  (10 s)
  ulimit -Ht "$PROBE_CPU_SEC" || exit 90   # RLIMIT_CPU hard
  exec "$@"                                # only the locally built proof binary
' bash "${cmd[@]}" &
```

Before launching, the launcher proves in a throw-away subshell that the limits can be installed **and** that the hard limit cannot be raised again (`ulimit -Hv 536870912` then `ulimit -Hv 537870336` must fail); if either check fails it refuses with `REFUSED_CONTAINER_UNAVAILABLE` and never starts a workload. A finite wall watchdog (poll + deadline) sends `SIGTERM`, then `SIGKILL`, to the whole **process group**, and after every `wait` it sweeps `kill -KILL -$pid` so no group member can outlive the supervisor on the normal-exit path either. Limits are applied soft-before-hard because the kernel rejects a hard limit below an `unlimited` soft limit with `EINVAL` — a fault the first CI run surfaced and the launcher now handles explicitly.

**Readback from inside the child** (`/proc/self/limits`, promoted as `limits.json`): `as_hard = 536870912` bytes, `cpu_hard = 10` s — the values the *workload itself* observed, not the launcher's intent.

### Windows — `verification/avif-r03-c3/limit-windows.ps1`

Narrowly scoped in-script C# `Add-Type` P/Invoke over the documented Job Object API: `CreateJobObjectW`, `SetInformationJobObject`, `QueryInformationJobObject`, `CreateProcessW`, `AssignProcessToJobObject`, `IsProcessInJob`, `ResumeThread`, `WaitForSingleObject`, `TerminateJobObject`, `GetExitCodeProcess`, `CloseHandle`. The child is created **`CREATE_SUSPENDED`**, assigned to the limited job, verified with `IsProcessInJob`, and only then resumed — so it cannot execute a decoder instruction outside the job. Every return value is checked; any failure at `create-job`, `set-info`, `query-info`, `verify-flags`, `verify-cpu`, `verify-memory`, `create-process`, `assign`, `is-in-job` or `resume` refuses (and terminates the job) instead of falling back to an unconfined run.

Effective limits read back from the kernel with `QueryInformationJobObject(JobObjectExtendedLimitInformation)` **before** `ResumeThread`:

| Flag bits (`limit_flags = 8962` = `0x2302`) | Value observed |
| --- | --- |
| `JOB_OBJECT_LIMIT_PROCESS_TIME` `0x2` | `per_process_user_time_100ns = 100000000` (10 s normal, `20000000` = 2 s in the CPU probe) |
| `JOB_OBJECT_LIMIT_PROCESS_MEMORY` `0x100` | `process_memory_limit = 536870912` (512 MiB; `67108864` = 64 MiB in the memory probe) |
| `JOB_OBJECT_LIMIT_JOB_MEMORY` `0x200` | `job_memory_limit = 536870912` (identical to the process limit) |
| `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` `0x2000` | set; verified by observation (see §3) |

After completion the supervisor queries `JOBOBJECT_EXTENDED_LIMIT_INFORMATION` again for `PeakProcessMemoryUsed`/`PeakJobMemoryUsed` and `JOBOBJECT_BASIC_ACCOUNTING_INFORMATION` for job user time and `TotalProcesses`/`ActiveProcesses`. The 15 s parent deadline independently calls `TerminateJobObject` and waits for termination.

### Refusal and output discipline (both hosts)

One typed outcome per run: `OK`, `REFUSED_INPUT_TOO_LARGE`, `REFUSED_INPUT_MISSING`, `REFUSED_CONTAINER_UNAVAILABLE`, `REFUSED_CHILD_CREATION`, `REFUSED_MEMORY_LIMIT`, `REFUSED_CPU_LIMIT`, `REFUSED_WALL_TIMEOUT`, `REFUSED_ABNORMAL_EXIT`, `REFUSED_DIGEST_MISMATCH`, `REFUSED_MISSING_OUTPUT`. Each case runs with its own staging directory; the parent hashes the staged decoded-plane file with the **host tool** (`sha256sum` / `Get-FileHash`) and promotes files into the final directory **only** on `OK` plus a digest match. On any refusal every staged byte is deleted and the final directory stays empty — the number discarded is recorded.

## 3. Result matrix — same tests, both hosts, all executed

Each row is a real invocation of the containment supervisor; the numbers are from the green run `43162966403`. Linux reports 20 assertions, Windows 17 (Linux splits CPU/wall/accounting/residue into separate assertions; Windows folds them into the record plus two job-specific checks).

| Test | Linux (`ubuntu-24.04`) | Windows (`windows-2025`) |
| --- | --- | --- |
| **T01** limits present before the workload | `OK`, `enforceable=1`; child read back `as_hard=536870912`, `cpu_hard=10` from `/proc/self/limits` | `OK`, `enforceable=1`, `limit_flags=8962`, cpu `100000000`, process/job memory `536870912`, `total_processes=2`, `peak_job=2 088 960` |
| **T02** real decode inside the limits | `OK`; promoted `red.avif` digest `205691271feb77cc0f208f495bf466e2fb9a9733d816b2d2355f05eb6c15552c` | identical `OK` and identical digest |
| **T03** 64 MiB cap vs 128 MiB touch | `REFUSED_MEMORY_LIMIT`, `signal=6` (SIGABRT), exit `134`, wall `105 ms`, nothing promoted | `REFUSED_MEMORY_LIMIT`, exit `0xC0000409` (abort), `peak_process_bytes=12 070 912`, `peak_job_bytes=13 463 552` under a `67 108 864` job cap |
| **T04** 2 s CPU cap vs 20 s spin | `REFUSED_CPU_LIMIT` at `child_cpu_sec=2.065`, SIGKILL | `REFUSED_CPU_LIMIT`, job user time `5 203 125` × 100 ns = **5.203 s**, exit `0xC0000044` |
| **T05** 2 s wall cap vs 20 s sleep | `REFUSED_WALL_TIMEOUT`, `wall_ms=2234`, SIGTERM | `REFUSED_WALL_TIMEOUT`, `terminated=true`, `wall_ms=2010`, `active_processes=0` |
| **T06** descendant cannot outlive the group | `REFUSED_WALL_TIMEOUT`, **`surviving_descendants=0`**, one staged file discarded | `REFUSED_WALL_TIMEOUT`, `total_processes=3`, `active_processes=0`, **`surviving_descendants=0`** |
| **T07** 20 MiB+1 input | `REFUSED_INPUT_TOO_LARGE`, `child_started=0` | same, `child_started=0` |
| **T08** post-decode digest mismatch | `REFUSED_DIGEST_MISMATCH`, `final_files=0` | same |
| **T09** workload cannot be created | `REFUSED_CHILD_CREATION` (exit 127) | `REFUSED_CHILD_CREATION`, `job_stage=create-process`, error 2 |
| **T10** unusable containment configuration | `REFUSED_CONTAINER_UNAVAILABLE`, `child_started=0` | same, `child_started=0` |
| **T11** the whole accepted corpus | `PASS` 25/25 — digests 12/12, refusals 13/13 | `PASS` 25/25 — digests 12/12, refusals 13/13 |
| **T12** two simultaneous contained workloads | both `REFUSED_MEMORY_LIMIT`, zero promoted | both `REFUSED_MEMORY_LIMIT`, zero promoted |
| **Suite totals** | **`passed=20 failed=0`** | **`passed=17 failed=0`** |

**Corpus preservation, measured independently on each host.** All twelve positives decoded inside the 512 MiB / 10 s / 15 s isolation and their promoted plane files re-hash to the exact WO-069 values:

`alpha_video` `3d26373d…b123`, `bbb_alpha` `ad25ae28…4f6f`, `black420` `d7928dbf…63bb`, `gray32` `ca707edb…f3d5`, `midgray` `b4052e74…6d8a`, `red` `205691271…552c`, `white16` `dec238c0…6f42`, `monochrome` `458ee397…8022`, `kimono_rotate90` `efcc7c58…5a5c`, `fuzz/y_plane_divergence_match` `3a9b88a3…63ec`, `fuzz/y_plane_roundtrip_avif1` `e5ebc9d1…cdc8`, `fuzz/y_plane_roundtrip_avif2` `e621abf7…96ae` — `promoted_plane_files=12` on **both** hosts, byte-for-byte equal digests across platforms.

All thirteen negatives kept their exact refusals inside isolation: `neg:empty` → `contract.zero_length`; `neg:input_over_20mib` → `contract.max_input_bytes`; `neg:declared_width_8193` → `contract.max_dimension`; `neg:declared_pixels_over_32m` → `contract.max_pixels`; and nine decoder refusals (`random_short`, `ftyp_truncated`, `header_only`, `av1c_only`, `truncated_60pct`, `truncated_85pct`, `tile_corrupt`, `invalid_brand`, `trailing_garbage`). The 20 MiB / 8,192-side / 32,000,000-pixel preconditions still fire with checked `u64` multiplication **before** decoding, and containment never converted a guard violation into an acceptance.

## 4. Dependency, licence and scanner posture (unchanged)

The probe keeps its own `[workspace]`, `publish = false`, `license = "Apache-2.0"` (so the unchanged private-crate licence scan can see it) and `#![forbid(unsafe_code)]` for its Rust sources. The exact three pins are unchanged — `oxideav-avif =0.0.11`, `oxideav-av1 =0.1.23`, `oxideav-core =0.1.43` — resolving to the same 18-package graph (17 third-party, zero duplicate versions), and `Cargo.lock` is byte-identical to the accepted WO-069 lockfile. No new third-party root, no vendored source, no native codec tool.

The pinned, SHA-256-verified `cargo-deny 0.20.2` gate ran against the probe graph with the **unaltered** repository `deny.toml`:

| Host | Archive SHA-256 | RustSec db commit | Result |
| --- | --- | --- | --- |
| ubuntu-24.04 | `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f` | `fd6f39814c4d3c359fdbe223d0280dec4933f78f` | advisories ok, bans ok, licences ok, sources ok — **0 errors, 0 warnings** each (16 licence notes) |
| windows-2025 | `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc` | `fd6f39814c4d3c359fdbe223d0280dec4933f78f` | same, 0 errors / 0 warnings |

The inherited `Unicode-3.0` `license-not-encountered` **note** remains an outstanding, non-suppressed release obligation and stays `UNVERIFIED`; it is a notice duty, not a security finding. Provenance gaps from WO-069 are unchanged and unresolved by an OS sandbox: the three self-descriptions still conflict ("orphan-rebuild scaffold", "partial intra", "stub — real decode pending AV1 tile support"), and an isolation proof says nothing about AVIF conformance or package origin.

## 5. Limitations and measurement gaps (stated, not hidden)

1. **The CPU cap is periodic on Windows.** `PerProcessUserTimeLimit` was set to 2 s, yet the probe consumed **5.203 s** of job user time before termination (exit `0xC0000044`). Windows enforces job time limits on a periodic check, so a CPU cap is a *strong* bound, not a to-the-second timer. Linux `RLIMIT_CPU` was precise (`2.065 s`). The 15 s wall deadline is the hard outer bound on both hosts.
2. **Linux `RLIMIT_AS` is virtual address space, not RSS.** It bounds what the process may *address*; it is not a resident-set measurement and is never reported as one. The `/proc/<pid>/status` peaks from WO-069 were observations and are not used here as caps.
3. **Windows memory accounting is commit-based.** In T03 the process peaked at 12,070,912 bytes of *committed* memory before the 128 MiB reservation was denied by the `67 108 864`-byte job limit — the denial is real, but the peak figure is not the requested allocation size.
4. **Windows per-child RSS is still not measured** (no native API this order allows). Per-case CPU and commit come from job accounting; RSS is not claimed.
5. **Quality limits from WO-069 are unchanged.** All twelve positives are **8-bit stills**; **10/12-bit, grid composition and AVIS image sequences remain `UNVERIFIED`** — no permissible vector for them was produced or used.
6. **`kimono_rotate90` orientation is recorded separately**, not proven: its planes arrive transposed relative to the declared 1024×722 geometry (`geometry_orientation=transposed_relative_to_reported_dimensions`). A digest match inside isolation proves *reproducibility*, **not** geometric correctness.
7. **No independent alpha or transform golden exists.** The two 4-plane alpha fixtures decode reproducibly and pass geometry checks; that is not an external expectation, and no `PASS` is awarded for alpha/transform correctness.
8. **A successful decode never cancels an isolation failure**, and every failure above is a distinct typed refusal rather than a codec verdict.

## 6. Acceptance ledger

| Gate | Verdict | Evidence |
| --- | --- | --- |
| `S070-01` | **PASS** | Verified sole checkout/worktree/`origin`, exact baseline SHA/tree/blobs and the `OWNER-ISSUED — SOLO-WO-070 R01` handoff; ACTIVE-only issuance `e8a90421…` with correct parent/tree/blob and everything else untouched; original `rust-verify` `43157117182` green on both hosts at that SHA. |
| `S070-02` | **PASS** | One new branch `solo-wo-070-r01`; the five WO-069 proof paths restored per-path from `e6162cdf…` with all staged blobs verified identical; only the seven listed proof paths changed; `solo-wo-061-r01`, `solo-wo-069-r01` and every product path preserved. |
| `S070-03` | **PASS** | Real pre-exec `RLIMIT_AS`+`RLIMIT_CPU` (soft and hard) inside a new session, un-raiseability proven, child readback `as_hard=536870912`/`cpu_hard=10`, and negative injections T03/T04/T05/T06 all denied (`SIGABRT`; `SIGKILL` at 2.065 s CPU; `SIGTERM` at 2.234 s wall; 0 descendants). |
| `S070-04` | **PASS** | Job Object with `0x2302` flags, process/job memory and per-process CPU limits set, `QueryInformationJobObject` verified before `ResumeThread`, suspended→assign→`IsProcessInJob`→resume, `TerminateJobObject` on deadline; memory/CPU/wall/descendant denials demonstrated with query evidence and peaks. |
| `S070-05` | **PASS** | Descendant containment on both hosts with `surviving_descendants=0` (Windows `total_processes=3 → active_processes=0`); every refused case left `final_files=0` and reported the staged bytes it discarded. |
| `S070-06` | **PASS** | 25/25 corpus cases inside the normal isolation on both hosts, 12/12 exact repeat digests, 13/13 refusals, guards reached before decoding; no claim about 10/12-bit, grid, AVIS, alpha-golden or rotation correctness. |
| `S070-07` | **PASS** | Exact three-root locked graph unchanged, `Cargo.lock` byte-identical, no new root, pinned verified `cargo-deny 0.20.2` on both hosts with 0 errors **and 0 warnings**, provenance gaps and the continuing `Unicode-3.0` notice obligation reported. |
| `S070-08` | **PASS** | Branch CI `43162966403` completed successfully at `d270f95c…` with every mandatory step executed on both hosts; no `continue-on-error`, no masked exit, no optional skip. |
| `S070-09` | **PASS** | One truthful verdict (`OS-ISOLATION-PROVEN`, scoped to the harness); no decoder approval, no product edit, no licence waiver, `R03.C3`/`R03.C`/Stage 03 open. |
| `S070-10` | **PASS** | Report-only commit on `main` with remote readback, green unchanged `rust-verify` at this HEAD, one clean checkout, three preserved branches, task scratch removed (§7). |

## 7. Preservation, cleanup and next action

One checkout, one worktree, one `origin`; `main` clean and equal to `origin/main`; four branches only — `main`, preserved `solo-wo-061-r01` (`4371af21…`), preserved `solo-wo-069-r01` (`e6162cdf…`) and the pushed qualification branch `solo-wo-070-r01` (`d270f95c…`) — left intact for independent review. WIP is exactly four files; `plan-*.md`, Architecture, About, SOPs, root `Cargo.toml`/`Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, all product crates and `rust-verify.yml` are byte-identical to the issuance revision. Ignored `target/` and the inherited `.temp/` archives (2 files / 289,253,806 bytes: `mingw.zip`, `rustup-init.exe`) are untouched; task-owned CI logs, local probe scratch and report composition parts were deleted at run end, with no unknown or Owner-owned material removed and no destructive Git command used. Local pre-CI validation was limited to syntax parsing plus supervisor smoke tests in disposable scratch; no local binary was ever built, so **no decode result is claimed from this workstation** — every decode figure in §3 comes from the two CI hosts.

**Next smallest architectural action** (Architect/Owner decision, not taken here): decide whether this containment model becomes the accepted mechanism for the future AVIF path — per-image invocation of the decoder inside a supervisor that (a) refuses before launch on size/dimension/pixel grounds, (b) installs an OS commit/address-space ceiling, a CPU ceiling and a wall deadline, and (c) promotes output only after an independently verified digest. If accepted, the follow-on order should specify that adapter's contract (which ceilings, who owns the supervisor, how it composes with the unlisted-representation and publication rules) **before** any product code — and it would still leave AVIF conformance, 10/12-bit, grid and AVIS coverage unproven. `R03.C3` cannot be closed on the strength of this order.

**Handoff:** `SOLO-WO-070 R01 — OS-ISOLATION-PROVEN` for the bounded harness, with the measured limits, the Windows CPU-overshoot caveat and every remaining `UNVERIFIED` gap above. The Chief Architect should audit this report's exact remote HEAD together with branch `d270f95c…`, then return `ACCEPT`, `REVISE` or `STOP` for this order only. No further work order, product change, release or deployment is initiated here.
