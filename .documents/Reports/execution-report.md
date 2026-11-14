# Execution Report

**status=COMPLETE — pending independent Chief Master Architect review**

**Order:** `OWNER-ISSUED — SOLO-WO-105 R01`. **Required outcome:** `R03C-WINDOWS-N102-ISOLATION-CANARIES-VERIFIED-AND-INTEGRATED`. All four real Windows canaries execute and pass on hosted `windows-2025`, the deliberate-red sensitivity proof is real and load-bearing, and the exact verified technical blobs are integrated on `main` with fresh two-host CI. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; no SVG/GIF codec exists; no release, deploy or self-acceptance occurred.

## 1. Preflight (all 21 pins exact)

`HEAD == main == origin/main == ls-remote origin main == 0d66aae7e0db34095f71de1d7d055d9b1712b7ed`; tree `c90476432db0f7d578b79b7635cf87f92c3ccbd1`; parent `7205027bbb656fe07205b4c54d2575f151d0c173`; report `807cd3fd`; tracked ACTIVE `ff4308e6`; NEXT `92fa562b` / ROADMAP `9d7a8622` / OUTLOOK `cb9c7a61`; Architecture `f66096ec`; SOPs `1483c9be` / `51bb3792`; `AGENTS.md` `6eae1da8`; `Cargo.toml` `14505be4` / `Cargo.lock` `5a18352b` / toolchain `9ec55b3b`; workflow `d7d8990c`; native seam `47fd94ae` / supervisor `43398847` / decoder `65b0cb29`; root `e99f3fa9` / CLI `17221fac` / smoke `c716d8b2`; checker `1c99d348` / guard `57d75796` / graph `da7d7ee7`; app policy `bdba09c1`; retained red branch `43616ae5` (tree `3cc54080`, parent `23da19fb`). 17 remote refs with no `solo-wo-105-*`; one worktree, one `origin`, empty stash, four WIP files; the only preflight dirt was the Owner-installed ACTIVE, installed byte-for-byte.

## 2. Commits

| Step | Commit | Tree | Sole parent | Notes |
|---|---|---|---|---|
| **A** authority | `cdb643415ad72a5eacd5f72fc560f6184f6f2113` | `6d08e38e` | `0d66aae7` | ACTIVE `8241b5af`; Architecture `f66096ec` untouched. `push` run **`43570835633` GREEN** both hosts. |
| **C** branch | `solo-wo-105-canary-r01`: `421302b9`, `23ceb618`, `4a9a613f`, `1d87d603`, `31b50e8b`, `ce1ea288`, **`ba1767dd959606cf3623e3703ca0b796b609fa36`** (tree `2004e508`) | — | — | Green `workflow_dispatch` **`43572910547`**: Ubuntu `132380936884`, Windows `132380937166`. |
| **E** negative | `solo-wo-105-negative-r01`: `3ea52000` → **`d345022d8d221f55bc7fc3a1a4797913db1ed0b5`** (tree `51d67392`, parent `3ea52000`, mutated blob `650915c5`), never promoted | — | — | Red `workflow_dispatch` **`43573525610`**, Windows job `132381885404`. |
| **D** integration | **`628d69a67e89c9e1f9be117bd4e5a9e1e3d98e1d`** | **`2004e50873188dd048d396818fa08640afc49ad6`** | `cdb64341` | Tree identical to the green branch head: byte-identical content across the seven scoped blobs. `push` run **`43573827513`**. |

## 3. What replaced the rejected WO-104 seam

`std::thread_local!` and every process-global atomic selector are gone; the accepted AST checker and the unsafe guard were never edited.

- **Per-run fault context.** `N102FaultRunV1` (safe supervisor, cross-platform) holds `requested`, `reached`, `cleanup_verified`; `N102FaultContextV1` (positively Windows-gated native) carries the same triple into `spawn`, `resume` and `wait_ms` by `&mut`. Each boundary records itself via one-shot `inject_here(boundary)`, so evidence is never inferred from the request. Every production run constructs `production()` and can never inject; only the closed `run_n102_fault_attempt_v1(executable, step)` entry can request `1..=11`, and `0`/out-of-range is rejected while the worker modes and probe answers stay internal.
- **Checked cleanup after `CreateProcessW` (step 9).** The old `return None` after a successful create but before job assignment merely dropped handles, and a job cannot kill an unassigned child. It now terminates the **owned process handle** and requires `WaitForSingleObject == WAIT_OBJECT_0` before recording verified cleanup.
- **Assignment failure (real and step 10).** Terminates the job *and* the owned process, recording verified cleanup only from the observed process exit; a refused `resume` runs the same terminate-and-observe path before refusing.
- **Injected wait failure (step 11).** Cleanup is no longer suppressed by the injection: `terminate_and_observe_v1()` runs and is observed first.
- **Handle probe accuracy.** The extra handle is a real inheritable `CreateEventW` object absent from the two-entry `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`; the ambiguity control is a second event in the same parent table created with `bInheritHandle = 0`, whose numeric value can never be inherited. Values are converted without truncation and no raw handle, canary value or environment block is printed.
- **CI gate.** Qualified `^test n102::<name> \.\.\.` matches, an explicit `... FAILED` refusal and the exact `test result: ok. 4 passed; 0 failed; 0 ignored` count; nothing is skipped and every original gate is unchanged (the workflow diff versus `main` is insertion-only).

## 4. Real Windows evidence (`43572910547`, job `132380937166`)

```
test n102::n102_01_extra_inheritable_handle_is_excluded ... n102_01 extra_handle_excluded=true control_live=false child_pid=3996
ok
test n102::n102_02_parent_environment_is_excluded ... n102_02 parent_canary_excluded=true system_root_present=true line=SOLO-N102-ENV-PARENT-V1 canary_present=0 system_root_present=1 pid=1340 status=0
ok
test n102::n102_03_native_failures_refuse_and_cleanup ... n102_03 injected_steps_reached=11 cleanup_observed=[(1,false),(2,false),(3,false),(4,false),(5,false),(6,false),(7,false),(8,false),(9,true),(10,true),(11,true)]
ok
test n102::n102_04_a_concurrent_worker_is_unaffected_by_a_fault ... n102_04 concurrent_probe_pid=7792 unaffected=true
ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out
```

- **Handle isolation:** the inheritable extra handle is unreachable while the non-inheritable control resolves nothing, so the exclusion is not numeric coincidence; the reported pid is the child's own.
- **Environment isolation:** the helper parent carries `SOLO_N102_PARENT_CANARY`, launches the real worker through the production minimal UTF-16 environment, and the child reports `canary_present=0` with `SystemRoot` still present.
- **Every one of the 11 boundaries** was reached at the native boundary itself, produced a typed refusal and yielded no successful inspection. Cleanup is `false` for steps 1–8 (no child existed) and `true` for 9, 10 and 11, i.e. only post-create paths claim verified cleanup, each from an observed wait.
- **Noninterference:** with faults running on a second thread a concurrent confined probe stayed unaffected, and `n097_01`–`n097_10` all passed in the same run, so the WO-104 cross-test leakage is gone.

## 5. Deliberate-red sensitivity proof (`43573525610`, job `132381885404`)

Mutation in the closed helper path only: the production minimal block is built and then discarded (`drop(environment)`) and `CreateProcessW` receives `std::ptr::null()`, so the child inherits the ambient parent environment. Format and strict lint passed; the unchanged test then failed **at execution**:

```
test n102::n102_02_parent_environment_is_excluded ... FAILED
assertion `left == right` failed: the helper parent refused instead of observing:
SOLO-N102-ENV-PARENT-V1 canary_present=1 system_root_present=1 pid=3916 status=61
  left: Some(61)
```

`canary_present=1` is the canary actually observed in the child — not a spawn, timeout, parser, lint or checker failure. `n102_03` and `n102_04` still passed in that same run, so the failure is specific and load-bearing. The mutation is not promoted.

## 6. Preservation, cleanup and limits

After integration: Architecture `f66096ec`, checker `1c99d348`, guard `57d75796`, `Cargo.lock` `5a18352b`, the adapter manifest and every unlisted path are unchanged; `solo-wo-104-canary-r01` `43616ae5` and all 15 earlier non-main refs are untouched; the ACTIVE file remains the Owner-issued WO-105 bytes for separate Owner closure; four WIP files, one worktree, one origin, empty stash. Scratch stayed inside ignored `.temp/` and the single temporary log created for the negative analysis was deleted.

Local limits, stated plainly: no MSVC linker exists on this machine, so every Windows fact above is a hosted `windows-2025` observation; the reflink-only `snapshot_proofs` and `n097` real-filesystem cases remain LOCAL-UNVERIFIED here while passing on the hosted runners. `n102_01`'s first hosted attempt reported a live numeric value, which was resolved by adding the non-inheritable control handle rather than by weakening the assertion. Job Objects and RLIMITs remain resource and lifecycle confinement, not a general network, filesystem or syscall sandbox.

