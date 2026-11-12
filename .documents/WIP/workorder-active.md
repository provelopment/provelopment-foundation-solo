# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-073 R01**  
**Title:** R03.C3 — implement and prove the two-host native process-containment boundary for a trusted, decoder-free internal worker probe  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-073 R01`  
**Authority:** This complete Owner-issued work order is the sole executable scope after Owner installation at `.documents/WIP/workorder-active.md` and delivery of the matching handoff. The Chief Master Architect has permanent **READ-ONLY GitHub access**. All repository and Git writes stated here are for the Implementation Agent alone. No codec, public-media verifier, publisher, deployment or release is approved.

## 1. Approved Scope — exact deliverable and non-goals

Implement the **first real, two-host, nonpublishing OS process-containment primitive** in the outer `solo-adapters` crate, and exercise it by spawning the **same trusted `solo-cli` executable** in a fixed internal, decoder-free test mode. Demonstrate that process limits are installed and verified **before** the internal workload executes; that the host refuses failed limit/launch/exit/timeout cases; and that child descendants and task-owned output do not survive refusal. The module remains an experimental *product-code process primitive*, not an AVIF decode adapter or security sandbox. The existing `MediaWorkerPortV1` remains **unimplemented** by a real decoder; keep `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1` and `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1` constant `false`.

**Explicit Owner architectural decision incorporated in this issued order.** Permit exactly **one additional audited first-party native-code file**, `crates/solo-adapters/src/media_worker_host_native.rs`, for *Linux and Windows process-launch and resource-containment operations only*. It may contain narrowly scoped, safety-documented `unsafe` calls on those target OSes. This replaces the prior *zero additional unsafe* rule **solely for that exact file and responsibility**; the pre-existing, separate `crates/solo-adapters/src/source_read_native.rs` Windows source-read exception remains unchanged. Update the Architecture Charter §6A.8 and the load-bearing unsafe/source checkers in this **same scope** so the rule is enforced mechanically and negative fixtures prove no third file, unscoped native call, wildcard exemption or crate-wide `unsafe` allowance can pass. Owner installation and issuance of this exact work order is authorization for precisely this decision; there is no other native or architectural discretion for the agent.

**No actual AVIF bytes are processed in this order.** No `oxideav-*` dependency, copied experimental shell/PowerShell script, AVIF library integration, media parser, MIME proof, real pixel-plane exchange, release or deployment. No new binary or fifth crate: `solo-cli` remains the single executable and composition root. No external `bash`, PowerShell, `prlimit`, `setsid` executable, C# `Add-Type` or mutable PATH-lookup shell is allowed as the product supervisor. The host module must use Rust's existing OS bindings and the pinned Cargo graph. All stress modes are internal to the same binary, finite, deliberately limited and refused unless their **actual own-process limits** are verified first. Do not create or serve an HTTP endpoint. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, even when all gates pass.

## 2. Exact starting revision and immutable anchors

Before the first non-Owner write, verify in the **single real checkout**:

| Item | Required value |
| --- | --- |
| `main == origin/main == HEAD` | `66706bbb772673c9f2885dbfa945473cf727deb8` |
| `HEAD^{tree}` | `6825aee5590312dbe9f51f9a10a9ae530e861e06` |
| Canonical report blob | `360fad651b4b6e46dbad71e9c0bc0f261cc742af` — `SOLO-WO-072 R01` |
| Prior ACTIVE blob at HEAD | `c307e6e35100263e6716ae1150576e0ba42bfb9f` |
| Architecture blob | `88b16fd91ffe4618801b4f956340674c27c7e047` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Root `Cargo.lock` blob | `2f9e5357…` (verify its complete SHA from HEAD; do not infer from this abbreviated display) |
| Historical Git branches | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` = `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Previous CI | `rust-verify` run `43174880904` — both hosts successful at baseline HEAD |

Required reading **before edits**: `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2I, 6A.3, 6A.8, both SOPs (Architect §§5–8, Implementation §§4–5C), all four WIP files, the exact canonical report, `crates/solo-application/src/media_worker.rs`, `crates/solo-adapters/src/media_worker_protocol.rs`, the current root and adapter Cargo manifests, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `rust-verify.yml`, the existing Windows source-read native module, the AST architecture checker and unsafe-boundary tests, and `solo-cli`'s existing command dispatch and tests. Never use a historical repository as authority.

## 3. Exact file and operation ledger

**On `main`, permanently writable paths only:**

1. `.documents/WIP/workorder-active.md` — Owner installs this exact text; agent **only** stages and commits it unchanged as the first, one-file issuance commit.
2. `.documents/Architecture/ARCHITECTURE.md` — one narrowly localized addition in §6A.8 recording the exact native-file exception in §1 and its limits. No other accepted Architecture text or rule changes.
3. `crates/solo-adapters/Cargo.toml` — Linux-target-only exact `libc = "=0.2.191"` direct dependency; add **only** `Win32_System_JobObjects` and `Win32_System_Threading` features to the existing Windows-target `windows-sys = "=0.61.2"` declaration, plus any binding feature conclusively shown necessary for the named APIs. No new package or version.
4. `Cargo.lock` — only the deterministic, `--locked`-compatible regeneration required by the feature/declaration changes; never substitute a package version or add a second version. Existing `libc 0.2.191`, `rustix 1.1.6`, `windows-sys 0.61.2` must remain the exact existing packages.
5. `crates/solo-adapters/src/media_worker_host.rs` — new **safe** typed host-runner façade, fixed smoke modes and closed refusal/observation types. No raw handles, `unsafe`, native imports or raw arbitrary-executable adapter API in this file.
6. `crates/solo-adapters/src/media_worker_host_native.rs` — new, the **only new** tightly restricted native module for Linux and Windows `unsafe`/`libc`/`windows-sys` access. No media parsing or publication.
7. `crates/solo-adapters/src/lib.rs` — declare/re-export the narrow host façade and gate the new native module with `#[cfg_attr(any(target_os = "linux", windows), allow(unsafe_code))]` **only on that module**; retain `#![deny(unsafe_code)]` and the existing `source_read_native` gating unchanged.
8. `crates/solo-cli/src/main.rs` — add a strict, *internal only*, exact-argument `--internal-media-host-probe-v1` dispatch **ahead** of the existing public CLI parsing. Preserve every current `bootstrap-preflight` route and exit/stdio behavior. The internal mode is **not** a general media CLI, decoder or publisher.
9. `crates/solo-cli/tests/media_worker_host.rs` — new real `solo-cli` executable integration tests on both hosts, using the Cargo-provided test binary location, never an unverified executable from PATH.
10. `crates/solo-adapters/tests/media_worker_host.rs` — new safe API/refusal/host-contract tests (no fixture reads outside the task scratch).
11. `crates/solo-cli/tests/unsafe_boundary.rs` — mechanically enforce exactly the original Windows source-read exception **plus** the new platform-gated native process module. Preserve and expand the existing refusal-injection tests.
12. `crates/solo-cli/tests/architecture_checks/source.rs` — extend the AST unsafe-scope rule for the one new Linux/Windows native module, while preserving the exact original source-read module rule and all existing alias/macro/dynamic-load checks.
13. `crates/solo-cli/tests/check_architecture_fixtures.rs` — only if required to cover the newly permitted Linux unsafe scope and prove negatives against the real checker.
14. `crates/solo-cli/tests/architecture_checks/graph.rs` and `crates/solo-cli/tests/workspace_graph.rs` — **only** the minimal exact-root/closure updates for `libc 0.2.191` as a Linux-target normal dependency of `solo-adapters`; no wildcard package, dev, transitive or build allowance.
15. `.documents/Reports/execution-report.md` — one canonical replacement at completion, report-only final commit.

No other path, including application/domain modules, old `source_read_native.rs`, `media_worker_protocol.rs`, existing `media_worker.rs`, published graphic fixtures, root `Cargo.toml`, other WIP, README/AGENTS/SOP, `deny.toml`, toolchain, existing CI workflow or historical proof branch, may be modified. No new external Git branches, second checkout/worktree, nested clone, PR, merge, tag, release, deployment, secret/provider operation or GitHub settings change. Existing branch refs are immutable.

**Temporary material:** only the verified root's ignored `.temp/`, ordinary ignored `target/` (cache), GitHub runner temporary area and the actual Cline task temp are available. Never touch unknown/Owner-owned archives or use `git clean`, force/reset/rebase/stash/cherry-pick. Delete only positively identified task-owned scratch, record both temp inventories. External fetches are limited to locked crates.io packages already resolved by the reviewed pins, pinned Rust 1.99.1 and the existing verified `cargo-deny 0.20.2` RustSec refresh. Do not download or execute native codec tools, compiler replacements or scripts from random URLs.

## 4. Required Git and CI sequence

**4A. Preflight.** From the actual Owner-local root run `git rev-parse --show-toplevel`, `git rev-parse HEAD`, `git rev-parse 'HEAD^{tree}'`, `git status --porcelain=v1 --untracked-files=all`, `git worktree list`, `git remote -v`, `git branch -avv` and exact `git rev-parse HEAD:<path>` for the anchors. Require one checkout/worktree, exactly one `origin` URL `https://github.com/provelopment/provelopment-foundation-solo.git`, clean `main == origin/main`, and **only** the Owner-installed ACTIVE change. Verify complete `OWNER-ISSUED — SOLO-WO-073 R01` contents and handoff. STOP on drift, dead terminal or an unexpected file, and preserve all existing data.

**4B. Issuance.** Stage and commit **only** the Owner-installed `.documents/WIP/workorder-active.md`, with message `Issue SOLO-WO-073 R01: native worker containment boundary`; non-force push to `origin/main`; read back exact parent/tree/ACTIVE blob; require completed green original `rust-verify` on **ubuntu-24.04 and windows-2025** at that exact SHA before further changes. If unavailable/red, STOP and publish only a safe, factual report.

**4C. One bounded code-and-architecture implementation.** Make only §3 changes; inspect the actual diff before every commit. One coherent change commit `feat: add guarded native worker containment probe (SOLO-WO-073)`, push normally and require complete both-host `rust-verify`. If an actual compiler/fixture defect is found, repair only allowed paths in separately described correction commits and rerun the complete two-host gate at the **final code HEAD**. Never weaken limits, tests, architecture checker or scanner to reach green. Do not commit an unreviewed change outside the ledger.

**4D. Independent fidelity audit.** Compare code HEAD to issuance SHA: exact changed-path list, cargo package names/versions/features, lockfile, all `unsafe` occurrences and their scopes, real-tree and injected-negative checker outcomes, process-group/job logic, input/output policy and all CI job step conclusions. A green build alone cannot prove OS enforcement.

**4E. Report.** On clean `main`, overwrite only `.documents/Reports/execution-report.md`, commit `Report: SOLO-WO-073 R01 native host containment evidence`, push and read back exact final parent/tree/blob. Require successful **unchanged existing** `rust-verify` on both hosts at the final report HEAD. Put the future report commit SHA only in the chat handoff, not inside the report itself.

## 5. Fixed native-process and test-only worker contract

### 5A. Safe host API and denial categories

Expose a narrowly typed, nonpublishing `run_internal_media_host_probe_v1(executable: &Path, probe: MediaWorkerHostProbeV1, scratch: &Path) -> MediaWorkerHostOutcomeV1` from `solo-adapters`. `MediaWorkerHostProbeV1` is a **closed enum** with only `ReadLimits`, `Harmless`, `Touch128MiB`, `Spin20Seconds`, `Sleep20Seconds`, `SpawnSleeper`. Do not permit arbitrary command strings, CLI switches, paths, requested memory sizes or environment values from untrusted inputs; the executable path is an explicitly trusted **test/composition-root** input and must not come from media, manifest, route, URI or provider. A missing/untrusted executable and any identity or capability uncertainty refuse. Tests supply the Cargo-built `solo-cli` path. The façade does not implement `MediaWorkerPortV1` or claim to verify any AVIF media.

`MediaWorkerHostOutcomeV1` must be a closed `Observed`/`Refused` type; `Observed` includes the exact effective caps, normal exit, finite elapsed time, process-tree termination/accounting and bounded trusted-child readback. `Refused` uses stable categories: `HOST_UNAVAILABLE`, `HOST_EXECUTABLE_UNTRUSTED`, `HOST_LIMIT_UNAVAILABLE`, `HOST_LAUNCH_FAILED`, `HOST_MEMORY_LIMIT`, `HOST_CPU_LIMIT`, `HOST_WALL_TIMEOUT`, `HOST_EXIT_ABNORMAL`, `HOST_DESCENDANTS_REMAIN`, `HOST_OUTPUT_INVALID`, `HOST_CLEANUP_FAILED`. Do not propagate raw stderr, private paths, untrusted bytes or native error strings to public diagnostics. No observation of a child is a successful media envelope, product decoder approval or publication authority.

**Normal reference limits per process:** 536,870,912 bytes virtual address space (Linux) or committed process/job memory (Windows), 10 **CPU seconds**, and a parent-observed wall deadline of 15,000 ms. **Fault-injection limits:** 67,108,864 bytes while touching 134,217,728 bytes; 2 CPU seconds for a 20-second busy loop; and 2,000 ms wall for a 20-second sleep. The finite wall watchdog must kill/reap a process tree. The smaller probes use the same exact native containment path as normal work, with values selected solely by this enum. Windows periodic CPU enforcement may overrun the 2-second setting; record the actual CPU time and enforce a separate finite outer wall deadline. Linux `RLIMIT_AS` is *virtual address space*, not RSS; Windows Job limits use *commit*, not RSS. A per-process RLIMIT by itself is not an aggregate Linux process-tree memory or PID limit; claim no such aggregate guarantee.

### 5B. Linux implementation, exact safety boundary

In `media_worker_host_native.rs` behind positive `#[cfg(target_os = "linux")]`, use `std::process::Command` only for the **trusted current Cargo-built executable**, with `std::os::unix::process::CommandExt::pre_exec`. Within its single documented unsafe boundary, call `libc::setsid()` (new session) and `libc::setrlimit(libc::RLIMIT_AS, ...)`, `libc::setrlimit(libc::RLIMIT_CPU, ...)` to apply **hard and soft** values **before exec**. Optionally use `prctl(PR_SET_PDEATHSIG, SIGKILL)` for parent-loss protection; if implemented, require an exact checked parent-identity race test, not a best-effort assumption. `pre_exec` executes between fork and exec: it may **only** perform the listed async-signal-safe native calls and return a fixed failure; no heap allocation, Rust logging, mutex, file I/O, env expansion or panicking code in the callback.

Illustrative values for the mandatory raw ABI behavior (use pinned `libc 0.2.191`'s real types, check all return values and preserve errno):

```rust
// Inside the single Linux-only, reviewed pre_exec callback.
let memory = libc::rlimit { rlim_cur: 536_870_912, rlim_max: 536_870_912 };
let cpu = libc::rlimit { rlim_cur: 10, rlim_max: 10 };
if libc::setsid() < 0
    || libc::setrlimit(libc::RLIMIT_AS, &memory) != 0
    || libc::setrlimit(libc::RLIMIT_CPU, &cpu) != 0
{
    return Err(std::io::Error::last_os_error());
}
```

The actual implementation must use the correct `libc::rlim_t` conversion with `try_into()` **before** `pre_exec`, and must not allocate inside the callback; the example is the exact kernel behavior, not a licence to ignore type and async-signal-safety checks. The internal worker reads its **own** `/proc/self/limits` before running any stress mode and refuses if values do not match the run's preinstalled limits. The parent retains the child process handle, records/validates the process-group identity, sends TERM then KILL to **that group only**, and reaps the leader before reporting final status, including on normal exit. Do not kill an unverified PID/group or touch another process. Failure to establish or sweep the group is a refusal. Do not claim Linux cgroup, network, syscall, filesystem or aggregate multi-process limits.

### 5C. Windows implementation, exact safety boundary

In the same new native file behind `#[cfg(windows)]`, use the **existing pinned** `windows-sys =0.61.2` bindings (`Win32_System_JobObjects`, `Win32_System_Threading`, existing Foundation features). The mandatory sequence is: `CreateJobObjectW` → set `JOBOBJECT_EXTENDED_LIMIT_INFORMATION` → `CreateProcessW` with `CREATE_SUSPENDED` and **no inherited handles** → `AssignProcessToJobObject` → `IsProcessInJob` and `QueryInformationJobObject` readback of every effective limit → `ResumeThread` **only after all checks pass** → wait/account/time out → `TerminateJobObject` and drain → `CloseHandle` exactly once per owned native handle. `CREATE_SUSPENDED` prevents first child instruction execution outside its assigned Job Object. Never fall back to `std::process::Command` or an unsuspended process when assignment fails.

Set these exact `LimitFlags` and check every value after `QueryInformationJobObject`:

```rust
const LIMIT_FLAGS: u32 =
    0x0000_0002  // JOB_OBJECT_LIMIT_PROCESS_TIME
    | 0x0000_0100 // JOB_OBJECT_LIMIT_PROCESS_MEMORY
    | 0x0000_0200 // JOB_OBJECT_LIMIT_JOB_MEMORY
    | 0x0000_2000; // JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
// Normal: process/job memory = 536_870_912, PerProcessUserTimeLimit = 100_000_000 (100 ns units).
// Stress: 64 MiB memory or 20_000_000 CPU time units (2 seconds) as applicable.
```

Use actual pinned binding types and signatures, not guessed pointer casts; check every allocation, struct size, return, nullable handle, resume count, overflow and OS error. Every unsafe block has its own adjoining `// SAFETY:` invariant and the minimum possible length. Avoid inheritable handles, unknown command resolution and environment secrets; build a fixed exact command line for the trusted full executable path and a fixed enum-controlled token only. Refuse any unprovable quoting, current executable identity or Job membership. Zero descendant survivors and resource accounting must be proved by the real Job Object API, not guessed from a PID list. Close/terminate on **every** failure including error-path early returns. CPU timing is periodic; the independent wall watchdog is mandatory.

### 5D. Internal worker modes: never accept uncontained stress

Add a **private command dispatch** in `solo-cli` with one exact flag `--internal-media-host-probe-v1` followed by one of the six closed mode tokens and a fixed nonce/limited profile supplied by the trusted host. It must read back actual OS limits **before performing any stress or writing any output**. On Linux use `/proc/self/limits`; on Windows call an exported safe function backed by the new native module to verify Job membership and query the effective effective limits. A direct, uncontained invocation of `Touch128MiB`, `Spin20Seconds`, `Sleep20Seconds` or `SpawnSleeper` must **refuse before executing the workload**. Reject unknown/missing/extra args; never accept a number controlling bytes, CPU, duration, a program path or a filename from the caller.

`ReadLimits` returns a fixed, bounded record of kernel values; `Harmless` returns a bounded normal exit/readback. `Touch128MiB` really touches every allocated page, `Spin20Seconds` consumes real CPU using `std::hint::black_box`, `Sleep20Seconds` sleeps for at most 20 seconds, and `SpawnSleeper` spawns only the **same trusted executable** in the fixed `Sleep20Seconds` mode after its own pre-run containment verification; no shell or arbitrary child. Stress operations must never execute outside the verified parent-installed limits and must not be accessible through authored configuration. Their only outputs are task-owned, private, bounded evidence; no media envelope, URL, public asset, source body or publication seal is produced. Preserve the normal CLI's exact behavior for all existing commands and failure/status codes.

### 5E. No falsely broadened security claim

This implementation proves only **operating-system resource/lifecycle containment of a trusted test binary on the two CI hosts**. Job Objects, `RLIMIT_*` and process groups are **not** a filesystem/network/syscall/credential sandbox; no arbitrary hostile media may run under this supervisor yet. Do not claim least privilege, general host availability, Linux aggregate per-job memory/pid limits, symlink/reparse/TOCTOU protection for real worker staging, secure IPC of actual media bytes, production host conformance or AVIF codec safety. The following remain separate future gates: process-isolated AVIF decoder adoption and provenance, no network and secret capability, secure immutable byte transfer, bounded plane transport, codec correctness (including 10/12-bit, grid, AVIS, alpha and transform), staged whole-candidate validation, licence/notice obligations, and all-format integration.

## 6. Exact Architecture and guard changes

Insert one short normative paragraph in Architecture §6A.8 **after the six AVIF out-of-process rules and before `Single portable sealed artefact`**. Do not rewrite the six accepted rules. The paragraph shall be exactly:

> **Native process-control exception — Owner-issued `SOLO-WO-073 R01`.** This explicit Owner decision supersedes the preceding §6A.8 prohibition on *additional* first-party `unsafe` solely for the new, exactly named `crates/solo-adapters/src/media_worker_host_native.rs` outer adapter: guarded Linux `pre_exec`/`libc` resource and process-group operations and guarded Windows Job Object, suspended process, handle and accounting operations. Every native call and `unsafe` block remains subject to a positive target gate, a local safety invariant and a strict negative-tested source/unsafe-boundary allowlist; all other first-party files remain forbidden. The existing Windows `source_read_native.rs` exception and its exact original purpose remain unchanged. This exception proves no AVIF decoder, media security sandbox, provider capability, release or Stage 03 closure.

Update the **actual** AST checker and the independent `unsafe_boundary` scanner/test so they agree on exactly **two** eligible native paths with different rules: the old source-read module is Windows-only (identical to its previous rule), and the new process module may use target-gated Linux/Windows native calls only. In `source.rs` extend `Facts` to track positive Linux gating separately; do not confuse a `#[cfg(windows)]` token inside a negated condition with a positive gate. Unit tests must prove:

- existing allowed Windows source reader still passes, but Linux or unscoped unsafe in it fails;
- new process native module passes only inside positive `cfg(target_os="linux")` or `cfg(windows)` blocks and only when module-scoped allowance is present;
- an injected unscoped block, copied module under another path, any inner crate, test helper, third module, ungated allow, renamed native import, macro indirection, `#[path]` escape, unknown dynamic or external import is still refused;
- every unsafe block in the new file has a `// SAFETY:` note and is minimal; no crate-wide or wildcard allowance;
- dependency graph remains four crates, single `libc 0.2.191`, single `windows-sys 0.61.2` and pinned previous nodes; only the `solo-adapters` Linux-target `libc` root is newly declared.

Do not replace the checker with a plain string search, disable failure-tripwire or loosen unrelated import/alias/refusal rules. If the existing AST checker cannot be safely extended within the allowed files, **STOP** and report that genuine architectural blocker; never hide an unsafe module from analysis.

## 7. Required two-host proof matrix

Run each case against the **actual newly built product-code supervisor** and the actual `solo-cli` binary, on `ubuntu-24.04` and `windows-2025`, as part of the existing `rust-verify` job's normal `cargo test --workspace --all-targets --locked` (no CI workflow edit). The integration test must refuse unsupported OS hosts rather than silently claim a pass. Each case must assert exact typed category and observed resource/process evidence, not merely a nonzero status. Distinguish Linux/Windows OS-specific result codes while keeping one stable refusal vocabulary.

| ID | Mandatory observation |
| --- | --- |
| `H073-01` | Before-child-exec cap installation, worker's own limit readback (Linux AS + CPU, Windows Job flags/process-job commit + CPU), and a harmless control returning normally under the reference 512 MiB/10 s/15 s profile. |
| `H073-02` | Under a 64 MiB cap, actual 128 MiB touched allocation never completes; typed memory refusal and zero final/staged evidence. |
| `H073-03` | Under a 2 CPU-second cap, a 20-second busy loop cannot complete; typed CPU refusal with real OS CPU/exit/accounting evidence. Disclose Windows periodic overshoot without miscalling it exact 2-second enforcement. |
| `H073-04` | A 20-second sleep under a 2,000 ms wall deadline is killed and reaped; typed wall refusal; parent confirms no remaining child. |
| `H073-05` | Deliberate descendant spawn then parent timeout/exit: descendant is terminated and accounted for, with zero survivors or promoted files. |
| `H073-06` | Two simultaneous *separately isolated* stress invocations both refuse and cannot escape their own lifecycle limits; no unproved global aggregate-memory claim. |
| `H073-07` | Direct, unsupervised stress invocation refuses before allocating, spinning, sleeping or spawning. Test with one direct CLI subprocess and a strict short timeout; fail the job if it executes the stress body. |
| `H073-08` | Invalid/missing arguments, missing/untrusted executable, failed/unsupported limit setting, denied Job assignment or unverifiable Job identity, abnormal exit, output mismatch and cleanup failure lead to typed refusal and no usable success. Use harmless fault injection only; no need to alter host policy. |
| `H073-09` | Existing `bootstrap-preflight` CLI outputs/status; source-read/media-worker application tests; actual AST and unsafe guard; every negative-test tripwire remain green and previously unauthorized paths remain rejected. |
| `H073-10` | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`; `cargo test --workspace --all-targets --locked`; `cargo build --workspace --release --locked`; pinned `cargo-deny 0.20.2` advisories/bans/licenses/sources on **both** hosts, with unaltered `deny.toml`. No unexpected skipped required steps. |

**Stress hygiene:** All memory/CPU stress probes run **only after an OS-containment proof** and use finite hard bounds. Do not execute any stress outside the real supervisor except the negative *direct-invocation refusal* test that must exit before stress. Never write OS-global resource controls, require Administrator/root escalation or tamper with another process. Tests must clean up task-owned processes, private staging and handles even on assertion failure; an unreaped process or unknown scratch is a failing gate.

## 8. Acceptance, report and stop rules

| Gate | Mandatory acceptance evidence |
| --- | --- |
| `S073-01` | Exact preflight, matching installed `OWNER-ISSUED SOLO-WO-073 R01` handoff, one root/worktree/origin, unchanged historical branch inventory and Owner-owned local data. |
| `S073-02` | ACTIVE-only issuance, remote readback and both-host original `rust-verify` success at issuance SHA. |
| `S073-03` | Exact one-paragraph Architecture native exception, with all other Charter bytes unchanged; no changed public contract. |
| `S073-04` | Exactly the new single native file is admitted beside the preserved old source-native file; source/unsafe checker and negative fixtures prove target-gated strict isolation, no broad allow. |
| `S073-05` | Locked Cargo graph/versions and four crate set preserved; only named feature/declaration additions; no new external package, duplicate, policy waiver or suspicious dependency. |
| `S073-06` | Real Linux pre-exec hard/soft `RLIMIT_AS`+`RLIMIT_CPU` and process-group lifecycle proof, in product code rather than the old bash harness. |
| `S073-07` | Real Windows suspended creation, Job Object assignment/readback before resume, process/job commit/CPU/wall/descendant lifecycle proof, native handle cleanup and no shell fallback. |
| `S073-08` | All `H073-01`–`H073-08` safety/refusal cases execute on both accepted hosts; actual out-of-containment direct stress refusal demonstrated. |
| `S073-09` | Existing CLI/application/media-worker behavior and architecture tripwires unchanged; no decoder or media/publish claim and no weakening of the existing refusal policy. |
| `S073-10` | Final code HEAD `rust-verify` green on both hosts, with actual executed integration tests and unchanged pinned security gate. |
| `S073-11` | One report-only final `main` commit, exact remote readback, both-host `rust-verify` green at the report HEAD, clean worktree and safe cleanup. |

**Result vocabulary:** `HOST-CONTAINMENT-PROVEN` **only** for the trusted smoke worker and both real OS hosts after every relevant gate passes; `BLOCKED` on a demonstrated necessary failure; `UNVERIFIED` when essential evidence cannot be obtained. No `MEDIA-VERIFIED`, no chosen decoder, no source-format conformance, no sandbox/security clearance, no publication or stage closure. Keep `R03.C3`, `R03.C`, Stage 03 **OPEN**.

The final canonical report must provide the starting/issuance/code/report-parent commits and trees, unchanged and changed blobs, detailed path ledger, both platform native cap readbacks and failure outcomes, each H073 and S073 gate, exact CI run/job IDs and failures/fixes, dependency graph changes, all `unsafe` occurrences and safety notes, introduced host assumptions, known limits (including Windows CPU periodic overshoot and Linux non-aggregate limits), material unverified security and media claims, branch/worktree preservation, `.temp/` and Cline task temp ownership/cleanup and final remote readback. Do not embed the report's own future SHA in its body.

**Immediate STOP, with safe factual report if possible:** dead terminal, identity/hand-off mismatch, unrecognized local change, unexpected branch/worktree/remote, new package/version beyond allowed graph, required new first-party `unsafe` outside the one path, unsafe block without adequate reasoning, unscoped allowance, weakened checker or negative test, CI failure not correctable within this scope, unverifiable kernel/Job readback, any stress running without verified limits, untrusted executable identity, unexpected network/secret/provider access, non-fast-forward push, unsafe deletion, unreaped child/process group, or inability to safely publish the canonical report. Preserve data and send blocker in chat if the shell cannot publish. The Implementation Agent never self-ACCEPTs or begins another order. The Architect independently returns `ACCEPT / REVISE / STOP` against the exact published report revision.
