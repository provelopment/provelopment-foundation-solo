# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-070 R01**  
**Title:** R03.C3 AVIF recovery — prove hard OS-enforced decoder isolation on Linux and Windows (non-product)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner→Implementation Agent handoff ID:** `SOLO-WO-070 R01`  
**Execution authority:** Execute this issued order exactly after verifying this file and the matching Owner handoff. The Chief Architect has **permanent read-only GitHub access** and performs no repository writes. The Implementation Agent performs the authorized operations inside the sole verified Solo checkout. **This order does not approve a decoder, alter the accepted image contract, integrate product media code, close R03.C3, release or deploy anything.**

## 1. Required result and exact starting revision

**Required outcome:** Extend the retained, independently verified AVIF qualification harness to prove—or accurately refute—whether untrusted AVIF decoding can be confined by **hard, measured operating-system controls** on both `ubuntu-24.04` and `windows-2025`. Implement a separately executable, per-image supervisor that enforces limits **before the decoder begins executing**, kills the entire contained workload on time/resource violations, returns no image after any abnormal termination, and refuses when containment cannot be established. Demonstrate actual memory, CPU, wall-clock and descendant-containment behavior with deliberate fault-injection probes, then rerun the existing AVIF positive and negative corpus inside those limits. Record a single evidence-backed verdict: **`OS-ISOLATION-PROVEN`**, **`BLOCKED`**, or **`UNVERIFIED`**. `OS-ISOLATION-PROVEN` is a result for the bounded experimental harness, **never** production decoder approval.

**Starting remote `main` HEAD:** `4372c9e36e852cd28215aa2b53c2dcc8387b8e5b`  
**Starting tree:** `b683f17fd999cc06db2d31a1df05e0a193711bd7`  
**Current canonical report blob:** `a891ad95ea0c6f7958bbb1c5434fb8db8a3e9503` (`SOLO-WO-069 R01`, honestly `BLOCKED`).  
**Current ACTIVE blob:** `7af728e7057ba8fc9101a5684f84273d8f92172a` (`SOLO-WO-069 R01`).  
**Neutral NEXT:** `92fa562b1af0c54391851aa5c79deb1631127069`; **ROADMAP:** `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; **OUTLOOK:** `cb9c7a610cf3c537557e060ca9656e44a053e69d`.  
**Retained evidence branch:** `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee` (tree `8865f49c34e6f0b362144e4913264b95c96092e0`).  
**Other preserved branch:** `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`.  
**Existing final `rust-verify`:** `43155025111`, both hosts green at `4372c9e3…`. **Existing qualification workflow:** `43154012989`, both hosts green at the retained proof branch head.

Read `README.md`, `AGENTS.md`, both SOPs (especially Architect §§6A, 6B, 7A, 7B, 8; Implementation §§4, 5A–5C, 7A–7B), Architecture §§5A, 6.2I, 6A.8, all four WIP files, the exact WO-069 execution report, and the five files on `solo-wo-069-r01`. Treat the published source and CI evidence as the authority; do **not** rely on chat summaries.

**Preserve all accepted invariants.** PNG/JPEG/WebP/AVIF remain required; input ≤20 MiB, dimensions 1–8192, pixels ≤32,000,000; enforce safe format/byte validation, whole-candidate failure, no implicit platform artwork/role fallback, no publication from unlisted/internal/suppressed owners and no product unsafe exception. The experiment does not narrow the accepted format or bit-depth contract. The exact codec pins stay `oxideav-avif =0.0.11`, `oxideav-av1 =0.1.23`, `oxideav-core =0.1.43`; no alternate dependency, licence waiver, policy relaxation or cargo scanner substitution.

## 2. Exactly authorized file and Git operations

### A. `main` — exactly two paths

1. `.documents/WIP/workorder-active.md`: the Owner installs this complete issued R01 file. Stage **only this file**, commit the byte-identical issuance to `main` and push normally to `origin/main`. Never edit its approved scope during the run.
2. `.documents/Reports/execution-report.md`: overwrite **only at run end**, commit **only this report** to `main`, push, remotely read back the blob/tree/parent and require completed green existing `rust-verify` on both hosts at that exact HEAD. Do not embed the report's own future commit SHA within its contents.

No other mainline path is writable. `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`, Architecture, About, SOPs, `Cargo.toml`, root `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, all production crates/tests and `.github/workflows/rust-verify.yml` remain byte-identical. WIP always contains **exactly four files**.

### B. One technical branch — `solo-wo-070-r01`

Create this new branch **from the verified WO-070 issuance commit on `main`** in the sole existing checkout. Check first that no same-named local or remote branch exists. Restore **only the five previously accepted test-harness paths** from the exact preserved `solo-wo-069-r01` commit using Git's per-path restore; this is not a second checkout, clone or a merge:

```powershell
git restore --source=e6162cdf733ea3136e96cbc139e46a6daf4054ee --staged --worktree -- `
  verification/avif-r03-c3/Cargo.toml `
  verification/avif-r03-c3/Cargo.lock `
  verification/avif-r03-c3/src/lib.rs `
  verification/avif-r03-c3/src/main.rs `
  .github/workflows/avif-qualification.yml
```

Confirm all five staged blobs match their source commit before further changes. Commit these five paths together as a traceable import of the prior proof, then modify the existing workflow to trigger only `solo-wo-070-r01`. Preserve the original `solo-wo-069-r01` branch byte-for-byte.

**Only these branch paths may be created or modified:**

- `verification/avif-r03-c3/Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs` — original standalone proof and bounded fault-injection additions;
- `verification/avif-r03-c3/limit-linux.sh` — Linux containment launcher/test driver;
- `verification/avif-r03-c3/limit-windows.ps1` — Windows containment launcher/test driver, including narrowly scoped test-only Windows Job Object native interop;
- `.github/workflows/avif-qualification.yml` — exclusively the new branch trigger and two-host containment tests.

No other file, workflow, fixture, root workspace, source or policy change. The standalone proof retains its own `[workspace]`, `publish = false`, exactly pinned three oxideav roots and `#![forbid(unsafe_code)]` for its Rust sources. No new third-party runtime root or new vendor source. Do not create a PR, merge, delete or move historical branches, tag/release/deploy, use `git reset`, `git clean`, stash, amend, rebase, force-push or create another checkout/worktree.

Use only Solo's ignored `.temp/` and the active Cline task temporary workspace for generated case bytes, output planes, logs and scratch. Preserve existing Owner-owned archives and ordinary `target/`; remove only positively identified task-owned disposable files. Git operations are authorized solely within this repository on the two specified refs, with non-force pushes to `origin` and readback. External network access is limited to the exact existing Cargo acquisition, pinned toolchain and verified scanner/advisory refresh already used by the proof workflow. No provider/service account, elevated system configuration, general tool installation or unrelated repository operation is authorized.

## 3. Mandatory preflight and issuance

1. Verify actual root via `git rev-parse --show-toplevel`, `git status --porcelain --untracked-files=all`, `git worktree list`, `git remote -v`, `git branch -avv`, `git rev-parse HEAD`, `git rev-parse HEAD^{tree}`. Confirm one checkout/worktree, one remote `origin`, clean `main == origin/main` at the exact §1 SHA/tree except **only** the Owner-installed ACTIVE replacement, no sequencer/conflicts, exact WIP inventory and report. Check the installed `SOLO-WO-070 R01` and matching Owner handoff. STOP on unrelated material or unexplained drift; never discard data.
2. Stage only ACTIVE; commit `Issue SOLO-WO-070 R01: bounded AVIF isolation proof`; push `origin/main`; verify exact remote parent/tree/blob and untouched other WIP/report paths. Wait for **completed successful** original `rust-verify` both `ubuntu-24.04` and `windows-2025` on the issuance SHA before beginning the branch.
3. Create `solo-wo-070-r01`, restore and validate the five exact WO-069 proof blobs as §2B, and keep the old branches intact. The committed probe `Cargo.lock` and original 12 positive pixel digests are the reproducibility baseline. No product codec adoption.

## 4. Fixed experimental resource contract

Run **one decoder invocation per isolated child**. The following limits are **the enforced test thresholds of this order**, not a new product promise:

| Limit | Normal isolated decode | Deliberate failure probe |
| --- | --- | --- |
| Source bytes | ≤20,971,520 bytes, checked before launch | 20,971,521 bytes: refuse before child |
| Dimensions | each 1–8,192 and checked `width × height ≤32,000,000` before decoder | 8,193 side and >32M pixels: caller refusal |
| OS memory / commit | **536,870,912 bytes (512 MiB)** hard child/job budget | **67,108,864 bytes (64 MiB)** hard budget while attempting to touch **134,217,728 bytes (128 MiB)** |
| OS CPU time | **10 seconds** hard per decode | Busy CPU workload above **2 seconds** under a **2-second** CPU cap |
| Elapsed wall time | **15 seconds** hard per decode | **20-second sleep** with a **2-second** wall cap |
| Parallelism | one untrusted decode process per request | two injected simultaneous children must not escape a total job/group policy |

A normal success has exit status 0, a complete decoded output and matching pre-existing pixel digest. A memory/CPU/wall-limit violation, inability to set/verify limits, abnormal exit or missing/partial output must produce **only an explicit typed refusal**; no success record, publishable output or retained partial plane file. The parent may create temporary output only in an inaccessible staging area; it atomically promotes results to the proof's final case output **after** verified normal completion. Do not equate measured RSS/working set with an enforced cap.

### Linux implementation — set limits before execution

Implement `limit-linux.sh` as an auditable `bash` containment launcher. Apply **hard and soft** kernel `RLIMIT_AS` and `RLIMIT_CPU` to the child before `exec`, use a new process group/session, retain the parent watchdog's finite wall timer and terminate/reap the whole contained process group on timeout. The essential resource-setting step must be equivalent to:

```bash
# These values are experimental caps, not derived from observed RSS.
ulimit -Hv 524288     # 512 MiB address space in KiB
ulimit -Sv 524288
ulimit -Ht 10         # CPU seconds
ulimit -St 10
exec "$@"              # only the pinned, locally built proof binary
```

For the injected memory and CPU probes, apply their stricter values **before exec**. Confirm the actual hard limits inside the child (e.g. `/proc/self/limits`) and that a child cannot raise them. `RLIMIT_AS` is virtual-address-space enforcement, **not** an RSS measurement; report that difference. Do not treat `/proc/<pid>/status` peak observations as a cap. If this hosted Linux environment cannot enforce the limits or contain/reap the workload, record `BLOCKED` rather than falling back to a watchdog-only run.

### Windows implementation — Job Object, suspended child and hard quotas

Implement `limit-windows.ps1` with narrowly scoped in-script C# `Add-Type` P/Invoke for the **documented Windows Job Object API**. Use `CreateJobObjectW`, `SetInformationJobObject(JobObjectExtendedLimitInformation)`, `CreateProcessW` with `CREATE_SUSPENDED`, `AssignProcessToJobObject`, `ResumeThread`, `WaitForSingleObject`, `TerminateJobObject`, `QueryInformationJobObject`, `GetExitCodeProcess`, and handle closing with `CloseHandle`. The child must be in the limited Job Object **before its first decoder instruction runs**. Use:

```csharp
// Set on JOBOBJECT_EXTENDED_LIMIT_INFORMATION.BasicLimitInformation.LimitFlags.
const uint JOB_OBJECT_LIMIT_PROCESS_TIME = 0x00000002;
const uint JOB_OBJECT_LIMIT_PROCESS_MEMORY = 0x00000100;
const uint JOB_OBJECT_LIMIT_JOB_MEMORY = 0x00000200;
const uint JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 0x00002000;
// PerProcessUserTimeLimit is in 100-ns units; 10 s = 100_000_000.
// Set ProcessMemoryLimit AND JobMemoryLimit to 536_870_912 bytes.
// Verify every native API return value before ResumeThread.
```

A 15-second parent deadline independently calls `TerminateJobObject`, waits for termination, and discards all partial outputs. Query and log the effective job limits, peak committed memory (`PeakJobMemoryUsed`/`PeakProcessMemoryUsed` where supported), CPU-time limit outcome and process exit/termination reason. Demonstrate deliberate memory, CPU and wall-time denials through actual Job Object enforcement; demonstrate the job's descendant-wide memory/kill boundary rather than only killing the initial PID. If nested Job Objects or runner policy block assignment, query or enforcement, report `BLOCKED` — no unsuspended `Start-Process`/`Get-Process` fallback, no administrator changes, no global host resource setting, no first-party Rust `unsafe` exception.

**Both hosts:** Add three test-only child modes to the standalone proof executable (not to product code): `--stress-allocate-bytes 134217728` (touch every allocated page and emit a marker only on completed allocation), `--stress-spin-ms 20000` (use `std::hint::black_box` to prevent optimizing away CPU work), and `--stress-sleep-ms 20000`. These modes run **only** within the new supervisor. Force-verify that 512 MiB permits a harmless control invocation, the 64 MiB memory probe cannot succeed, 2-second CPU limit stops the busy loop, and the 2-second wall timer stops sleep. Also verify a child attempting to outlive its supervisor is terminated, with no surviving process, output marker, leaked handle or published partial result. Assert refusal type, OS enforcement evidence and cleanup, not merely a nonzero exit code.

## 5. Exact qualification matrix and safety conditions

1. Inside the normal 512 MiB / 10 CPU s / 15 wall s isolation on **both hosts**, rerun the same **12 published AVIF fixtures** and the **13 negative cases** from `solo-wo-069-r01`; require the same known real decoded-plane SHA-256 results for all 12 positives, no missing fixture, and fail-closed negatives. Recheck after inspection that the exact 20 MiB, 8192-side and 32M-pixel preconditions fire, with checked `u64` multiplication, before decoding. No metadata-only success, hidden fallback or acceptance after a guard violation.
2. Implement and execute the normal-limit control plus memory-exhaustion, CPU-exhaustion and elapsed-time-exhaustion tests separately on Linux and Windows; prove **the limit was installed before the process ran**. Include a descendant-containment test, and prove zero retained partial public/result output after each forced failure. A successful codec decode does not cancel a later isolation failure.
3. Verify actual child process CPU consumption and its termination/refusal reason; record Linux `RLIMIT_AS`/`RLIMIT_CPU` values, Windows Job Object flags/limits/peaks and source commands/APIs used. Record parent and child IDs only as non-sensitive ephemeral IDs. Do not hide Windows measurement gaps.
4. Preserve known quality limits explicitly: all 12 positive fixtures are 8-bit stills; 10/12-bit, grid and AVIS remain **UNVERIFIED** unless independently reproduced with permissible test vectors. Record `kimono_rotate90` plane orientation separately from the declared 1024×722 geometry; do not claim that its correctness is proven by a digest match. No independent alpha/transform golden exists yet; do not award that check `PASS` without a genuine external expectation.
5. Repeat exact standalone graph identity/checksum verification and pinned `cargo-deny 0.20.2` with **unchanged** `deny.toml` on both CI hosts. Record fresh advisory database date/commit, yanks, licence/provenance and the continuing `Unicode-3.0` notice obligation. Do not substitute `oxideav` versions, relax Cargo policy, accept suspicious origin claims or add native codec tools. Existing upstream descriptions conflict; an OS sandbox does not resolve provenance or full AVIF conformance.
6. Run real refusal tests for container-unavailable, limit-setting failure, child-creation failure, worker crash/abort, timeout, over-memory, excessive CPU, invalid media, post-decode digest mismatch and partial-output cleanup. Each is a distinct fail-closed outcome. Test at the actual allowed boundaries rather than writing tests whose chosen input is rejected earlier for a different cause.

**Do not run uncontrolled exhaustion, CPU loops or attacker-like inputs outside the verified OS containment.** Stress inputs must be generated by the trusted proof binary itself, with the supervisor already enforcing resource limits. No hidden fallback to unbounded `AvifDecoder::decode_file` in the same process as a persistent publisher or test coordinator.

## 6. Branch CI and exact acceptance gates

Change `.github/workflows/avif-qualification.yml` on `solo-wo-070-r01` to trigger pushes **only** to `solo-wo-070-r01`, with `permissions: contents: read`, SHA-pinned checkout, `persist-credentials: false`, the pinned Rust 1.99.1 toolchain, locked/offline product-independent probe build, the same verified candidate scanner and **two full host jobs** (`ubuntu-24.04`, `windows-2025`). Require successful pre-exec OS limit verification, normal-limit corpus, deliberate resource-failure probes and cleanup tests. Do not swallow a functional failure with `continue-on-error`, `|| true`, an optional skip or a passing summary after a failing command. Publish readable, privacy-safe JSONL case evidence and exact host/job/commit identities in Actions logs, with no raw untrusted payload, secrets or private absolute file paths.

The proof may finish **`OS-ISOLATION-PROVEN`** only when *both hosts* demonstrate every mandatory resource-control and containment test, actual 12/12 pixel digest preservation and 13/13 refusal preservation, strict scanner pass and no unreviewed material security warning. A confirmed failure is **`BLOCKED`**, a genuinely unexecuted essential proof is **`UNVERIFIED`**. In every case, preserve `R03.C3`, `R03.C` and Stage 03 as **OPEN**. No codec or publication capability becomes approved, and no architecture contract changes in this order.

**Acceptance ledger** (report each separately as `PASS`, `FAIL`, `UNVERIFIED` or justified `NOT APPLICABLE`):

| Gate | Evidence required |
| --- | --- |
| `S070-01` | Verified sole checkout, exact baseline/ref/blobs, `OWNER-ISSUED` handoff; safe ACTIVE-only issuance and both-host original `rust-verify` green at issuance SHA. |
| `S070-02` | One new proof branch; exact five-file proof import from `e6162cdf…`; only listed proof files changed; historical branches and product preserved. |
| `S070-03` | Real pre-exec enforced memory, CPU and wall limits on Linux; actual limit readback and negative fault-injection proof. |
| `S070-04` | Real pre-exec Job Object process/job memory, CPU and wall enforcement on Windows, including query evidence and negative fault-injection proof. |
| `S070-05` | Descendant containment, process termination and zero promoted/retained partial output after failures on both hosts. |
| `S070-06` | Normal isolated 12-positive/13-negative corpus on both hosts, exact repeat digests and guarded dimensions/input, no false claims about outstanding high-depth/alpha/rotation correctness. |
| `S070-07` | Exact three-root locked graph; unchanged licence/security/unsafe posture, pinned verified `cargo-deny` on both hosts; provenance gaps and `Unicode-3.0` notice truthfully reported. |
| `S070-08` | Completed two-host branch CI at exact final branch SHA with all mandatory steps actually executed; failing safety probes are not masked. |
| `S070-09` | Single truthful `OS-ISOLATION-PROVEN` / `BLOCKED` / `UNVERIFIED` verdict; no decoder approval, product edit, licence waiver or R03.C3/stage closure. |
| `S070-10` | Report-only final `main` commit, correct remote parent/tree/blob, green existing original `rust-verify` on both hosts, one clean checkout and preserved branches, safe scratch cleanup. |

## 7. Final publication and handoff

At final proof-branch HEAD, record branch SHA/tree, exact changed-path list, both CI run/job IDs, effective kernel/Job Object limits, all pass/fail/unsupported observations, decoded hashes, refusal outcomes and cleanup. Leave the proof branch pushed and intact for independent review. Return safely to clean `main`; overwrite only `.documents/Reports/execution-report.md`, with report date, authorized scope, initial/issuance/report-parent anchors, every acceptance-gate verdict, concrete commands and evidence URLs, OS limit implementation details, result matrix, source/provenance and security assessment, limitations, exact temporary ownership/cleanup and next smallest architectural action. No second report or archive. Commit only the report, push normally, remotely read back and require green `rust-verify` at the **exact final report HEAD** on both hosts. Give the new report SHA/tree/blob, branch SHA/tree and two-host CI run links in the Implementation Agent's chat handoff. Do not self-ACCEPT, close ROADMAP or start a further work order.

**STOP immediately and report safely** on wrong root/remote/ref/sha/handoff, dirty unknown material, branch-name collision, unauthorized path, failed issuance CI, unverified Windows Job Object assignment, unenforceable Linux limits, native API error, unexpected unsafe/system installation, suspicious dependency, unbounded stress execution, permission escalation, destructive cleanup, non-fast-forward push, security-policy change or inability to establish correct process containment. A genuine negative *candidate* outcome may still be reported as `BLOCKED` with truthful, complete evidence; never repair the verdict by weakening a limit or moving the approved threshold. The Owner alone authorizes any later product integration, release, deployment or architectural change. The Chief Architect's independent disposition is `ACCEPT / REVISE / STOP` against the published exact revision.
