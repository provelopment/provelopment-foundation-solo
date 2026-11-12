# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-073 R01` — R03.C3: implement and prove the two-host native process-containment boundary for a trusted, decoder-free internal worker probe.
**Matching handoff:** `SOLO-WO-073 R01`. **Report date (ISO):** 2026-11-12 (run window 2026-11-12T07:40–08:35Z).

**Outcome: `UNVERIFIED`.** The issuance half of this order is complete and proven. The code half — the guarded native containment module, the safe supervisor façade, the six internal probe modes, the two-host proof matrix, the checker extensions and the Charter paragraph — was **not** completed, **not** compiled, **not** executed and **not** published. No product code from this order exists on `main`. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and no `HOST-CONTAINMENT-PROVEN` claim is made for any host, probe or gate.

This report deliberately records a **stop**, not a pass. Nothing here relaxes a limit, weakens a checker, skips a gate or invents evidence.

## 1. Preflight and issuance — verified

| Item | Verified value |
| --- | --- |
| `main == origin/main == HEAD` at start | `66706bbb772673c9f2885dbfa945473cf727deb8`, tree `6825aee5590312dbe9f51f9a10a9ae530e861e06` |
| Starting blobs | report `360fad651b4b6e46dbad71e9c0bc0f261cc742af`; Architecture `88b16fd91ffe4618801b4f956340674c27c7e047`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; `Cargo.lock` `2f9e53576df117f2020ae064d6973304f20833f1` |
| Prior ACTIVE blob | `c307e6e35100263e6716ae1150576e0ba42bfb9f`; installed ACTIVE `ebd56fe0e5c0fe4c233f2b7f8163fa4fd7a3ec59` — complete `OWNER-ISSUED — SOLO-WO-073 R01` contents, handoff matched |
| Repository shape | one checkout/worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, clean `main`, four WIP files, historical branches `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` unchanged |
| Issuance commit | `211476f93f47a5dde1a2d95f170406744baef5b6` · tree `a501318082b5b2f47081095c76ffe9724d027203` · parent `66706bbb…` · ACTIVE blob `ebd56fe0…` · **one path only** |
| Issuance CI | `rust-verify` run **43177281834**, jobs `131044493181` (ubuntu-24.04) and `131044492677` (windows-2025): **success**, 33/33 steps, 0 failures |
| Remote readback | commit, tree, parent and ACTIVE blob re-read from the GitHub API and identical to the local values |

`S073-01` ✓ and `S073-02` ✓ are met. Nothing else in §8 is.

## 2. What this run actually verified (useful, reusable evidence)

The native work was researched to the point where the remaining risk is compilation and host behaviour, not API shape. Everything below was read from the **vendored pinned crates** in this checkout's registry, not recalled:

* `libc 0.2.191` (Linux, `x86_64-unknown-linux-gnu`): `setsid() -> pid_t`, `setrlimit(c_int, *const rlimit) -> c_int`, `kill(pid_t, c_int) -> c_int`, `rlimit { rlim_cur, rlim_max }`, `RLIMIT_CPU = 0`, `RLIMIT_AS = 9`, `SIGTERM = 15`, `SIGKILL = 9`, `PR_SET_PDEATHSIG = 1`.
* `windows-sys 0.61.2`: `CreateJobObjectW`, `SetInformationJobObject`, `AssignProcessToJobObject`, `IsProcessInJob`, `QueryInformationJobObject`, `TerminateJobObject`, `CreateProcessW`, `ResumeThread`, `WaitForSingleObject`, `GetExitCodeProcess`, `TerminateProcess`, `GetCurrentProcess`, `CloseHandle` — with exact parameter and return types (`HANDLE = *mut c_void`, `PCWSTR = *const u16`, `PWSTR = *mut u16`, `BOOL = i32`, `WAIT_EVENT = u32`).
* Exact structures and constants: `JOBOBJECT_BASIC_LIMIT_INFORMATION { PerProcessUserTimeLimit: i64, …, LimitFlags: u32, … }`, `JOBOBJECT_EXTENDED_LIMIT_INFORMATION { BasicLimitInformation, IoInfo, ProcessMemoryLimit: usize, JobMemoryLimit: usize, … }`, `JOBOBJECT_BASIC_ACCOUNTING_INFORMATION { TotalUserTime: i64, TotalKernelTime: i64, …, TotalProcesses, ActiveProcesses, … }`, `JOB_OBJECT_LIMIT_PROCESS_TIME = 2`, `JOB_OBJECT_LIMIT_PROCESS_MEMORY = 256`, `JOB_OBJECT_LIMIT_JOB_MEMORY = 512`, `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 8192`, `JobObjectExtendedLimitInformation = 9`, `JobObjectBasicAccountingInformation = 1`, `CREATE_SUSPENDED = 4`, `CREATE_NO_WINDOW`, `WAIT_OBJECT_0`, `WAIT_TIMEOUT`, `STILL_ACTIVE`. The accounting structure and `CreateProcessW`/`STARTUPINFOW`/`PROCESS_INFORMATION` are gated behind exactly the two features this order authorizes.
* The checker surfaces that must be extended are located and understood: `Facts { unsafe_windows_scoped, unsafe_unscoped, windows_scope_depth }` with `unsafe_scope_verdict(location, unscoped, windows_scoped)` in `architecture_checks/source.rs`; the textual `violations()` scanner and `has_ungated_allowance()` in `unsafe_boundary.rs`; `APPROVED_ADAPTER_EXTERNAL_NORMAL` (4 entries) and `APPROVED_ADAPTER_CLOSURE` (already contains `libc`) in `architecture_checks/graph.rs`; and the real-tree/self-test matrix in `check_architecture_fixtures.rs`, whose counts are computed from `FIXTURES`/`POSITIVES`, so inline negative tests — not new fixtures in the read-only `architecture_checks/mod.rs` — are the compliant way to prove the Linux scope.
* The Charter insertion point is exact: Architecture §6A.8 rule 6 ends at line 1377 and `**Single portable sealed artefact.**` begins at line 1379, so the one-paragraph exception belongs at 1379 with the blockquote marker the order specifies.
* The authorized manifest delta was **prepared, applied and validated offline, then reverted**: `libc = "=0.2.191"` as a Linux-target normal dependency of `solo-adapters`, plus `Win32_System_JobObjects` and `Win32_System_Threading` on the existing `windows-sys` declaration. `cargo metadata --offline` accepted it and produced a **one-line** `Cargo.lock` change (the single `solo-adapters → libc` edge) with exactly one `libc 0.2.191` and one `windows-sys 0.61.2` package — so `S073-05` is achievable and its exact shape is now known.

## 3. What was produced, and why it was withheld

One file was written in full and is preserved as task scratch: `media_worker_host_native.rs`, 724 lines, containing the Linux `pre_exec` boundary (`setsid` plus hard/soft `RLIMIT_AS`/`RLIMIT_CPU`, returning one fixed non-allocating failure and nothing else), the process-group `SIGTERM`→`SIGKILL` sweep with its `kill(-pgid, 0)` emptiness proof, the `/proc/self/limits` readback, and the Windows sequence `CreateJobObjectW` → four limits → suspended `CreateProcessW` with `bInheritHandles = FALSE` → `AssignProcessToJobObject` → `IsProcessInJob` plus full `QueryInformationJobObject` readback → `ResumeThread` only after every check → Job accounting, `TerminateJobObject`, and exactly-once handle close, with error-path termination of a never-resumed child. It carries **27 `unsafe` blocks and 27 adjacent `// SAFETY:` notes** and it **parses** under `rustfmt` edition 2024.

It was **not** committed, because on its own it cannot be correct: the safe façade, the module wiring, the CLI dispatch, the two test suites and the checker extensions that must admit it do not exist, so committing it would either fail to compile or be rejected by the very guards this order requires to stay strict. Publishing unverified `unsafe` FFI to `main` to discover its defects remotely was judged worse than stopping: a defect there is a process-leak or containment-failure risk, not a lint nit.

## 4. Gate status, stated exactly

| Gate | Status |
| --- | --- |
| `S073-01` preflight, handoff, one root/worktree/origin, branch inventory | **PASS** (§1) |
| `S073-02` ACTIVE-only issuance, remote readback, both-host CI at issuance SHA | **PASS** (§1) |
| `S073-03` one-paragraph Architecture native exception | **NOT DONE** — insertion point located, text prepared from the order, not applied |
| `S073-04` exactly two admitted native files with a two-rule checker and negatives | **NOT DONE** |
| `S073-05` locked graph preserved with only the named additions | **NOT COMMITTED** — prepared and validated offline as a one-line `Cargo.lock` delta, then reverted |
| `S073-06` real Linux pre-exec hard/soft limits and process-group proof | **NOT IMPLEMENTED, NOT EXECUTED** |
| `S073-07` real Windows suspended creation, Job assignment/readback, cleanup | **NOT IMPLEMENTED, NOT EXECUTED** |
| `S073-08` `H073-01`–`H073-08` on both hosts | **NOT EXECUTED**; `H073-09` and `H073-10` rest on the unchanged pre-existing gates only |
| `S073-09` existing CLI/application/media-worker behaviour and tripwires unchanged | **PASS incidentally** — no source file changed, so the pre-existing suites and guards are exactly as they were at the baseline |
| `S073-10` final code HEAD green on both hosts | **N/A** — no code HEAD exists |
| `S073-11` one report-only commit with both-host CI green at the report HEAD | **PASS** for this report (exact SHA in the chat handoff) |

## 5. The blocker, stated plainly

1. **No local compiler for either target OS.** This host has the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain but **no MSVC linker and no Visual Studio Build Tools**, and no Linux target. Any crate that must link — which includes `serde`'s build scripts, and therefore `solo-adapters` and `solo-cli` — cannot be built, linted or tested here at all. Installing a linker or a second toolchain is outside this order's permissions and its dependency policy. Only `rustfmt`, `cargo metadata` and read-only inspection are available locally.
2. **The remaining surface is larger than one run could prove.** Still outstanding: the safe façade with six closed probes and eleven refusal categories, the supervisor and watchdog, the six worker-mode bodies, the internal CLI dispatch, two new test suites that spawn the real binary on both hosts, three checker files with new negative tests, and the Charter paragraph — every line unverifiable locally, each correction costing a full two-host CI cycle.
3. **No shortcut was taken.** No limit was relaxed, no checker weakened, no stress mode was executed anywhere, no unverified `unsafe` reached `main`, and no gate was declared passed on inference.

## 6. Preserved work and the safe resume path

Task-owned scratch, `.temp/solo-wo-073-wip/` (4 files, 46,062 bytes), is **retained deliberately** with a `NOTES.md` recording exactly what is done, what is not and how to resume: `media_worker_host_native.rs` (724 lines, parses, 27/27 `unsafe`-to-`SAFETY` pairing), `Cargo.toml.prepared` and `Cargo.lock.prepared` (the validated one-line lock delta), and the notes. Its cleanup trigger is the next Owner-issued order that completes or supersedes this scope; until then it is reference only and is **not** authority.

Recommended split, so each part is independently provable:

1. **Checker and Charter first, with no native code**: the §6A.8 paragraph, the two-rule `unsafe_scope_verdict` with positive Linux gating, the two-module `unsafe_boundary` rule with its negatives, and the `libc` graph allowance. Pure text and safe code, green on both hosts before any `unsafe` exists.
2. **Then the native module, façade, probes and proof matrix** — ideally Linux and Windows as separate orders, so one host's failure cannot hide behind the other's success.

## 7. Preserved state, cleanup and what is deliberately not claimed

Final repository state: `main == origin/main`, empty `git status --porcelain`, one worktree, one remote, four WIP files, and the four branches unchanged (`main`, `solo-wo-061-r01`, `solo-wo-069-r01`, `solo-wo-070-r01`). The only permanent change from this order is the ACTIVE issuance commit and this report. `Cargo.toml`, `Cargo.lock`, `deny.toml`, the toolchain file, the CI workflow, `ARCHITECTURE.md` and every source file are byte-identical to the baseline commit. `.temp/` holds the two inherited Owner archives (`mingw.zip`, `rustup-init.exe`) untouched, plus the four retained task-owned files above; `target/` was left alone.

Not claimed, and not to be inferred from this report: any native containment, any `H073` case, any `unsafe` approval in the shipped tree, any Windows Job Object behaviour, any Linux `RLIMIT` behaviour, any process-tree termination evidence, any AVIF decoding, any media correctness, any sandbox or security clearance, any release, or any closure of `R03.C3`, `R03.C` or Stage 03.

**Next smallest action** (Owner/Architect, not taken here): decide whether to re-issue this scope as the two smaller orders above. The Implementation Agent does not self-ACCEPT, does not retry this order on its own authority, and has begun no other order.
