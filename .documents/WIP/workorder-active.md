# SOLO-WO-100 R01 — COMPLETE WINDOWS HANDLE-ALLOWLISTED IMAGE WORKER AND VERIFY BOTH HOSTS

**Authority:** `OWNER-ISSUED — SOLO-WO-100 R01` — effective only when the Owner installs these *complete bytes* in the single canonical ACTIVE slot and sends the matching `SOLO-WO-100 R01` activation to the Implementation Agent.  
**Required outcome:** `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED` — subject to later independent Chief Master Architect ACCEPT / REVISE / STOP.  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Sole authorized checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (WSL uses this same filesystem checkout, not another clone).  
**Main target:** `main`. **Existing local technical branch to continue:** `solo-wo-099-image-r01`. **One allowed intentionally red negative branch:** `solo-wo-100-negative-r01`.  
**Scope:** Finish, prove and integrate the bounded, nonpublishing PNG/JPEG/WebP inspection started in WO-097, restored under WO-098 and corrected under WO-099. Do not restart, narrow the required host coverage, or accept unsupported containment claims.

## 1. Architect ruling and fixed technical decisions

1. **WO-099 is STOPPED / NOT ACCEPTED as a complete image inspection.** Independently confirmed progress: its A issuance, B exact-path `unsafe_boundary.rs` repair and D truthful STOP-report commits are each green on both GitHub hosts. The correction to the Windows/Linux per-declaration native allowance is now the approved baseline. It does not require another change, another native file, or a new architectural exception.
2. **WO-100 is one final, code-bearing continuation.** Complete the already reported local branch at `f6e1137250d1204d588742b49ebce029714763a6` after verifying its identity and diff. The branch is **not GitHub-verifiable until the Agent publishes it**. The Agent must verify the local identity, file set and WSL evidence directly, disclose any gap and STOP on unexplained local drift. DO NOT manufacture or reconstruct the branch merely to satisfy this pin.
3. The remaining mandatory safety boundary is **exact Windows handle inheritance and minimal environment**. Use the existing `windows-sys =0.61.2` APIs in the *already reserved* `crates/solo-adapters/src/media_worker_host_native.rs`, with **`STARTUPINFOEXW`**, `InitializeProcThreadAttributeList`, `UpdateProcThreadAttribute(PROC_THREAD_ATTRIBUTE_HANDLE_LIST)`, `DeleteProcThreadAttributeList`, `CreateProcessW` with `CREATE_SUSPENDED | EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT`, a parent-created minimal UTF-16 environment, verified Job Object limits/membership and controlled resume. Merely clearing inheritance on parent pipe ends while setting `bInheritHandles=TRUE` is NOT sufficient.
4. **No relaxation** of full decoding, content/hash/size/format checks, all-obligation refusal, panic/timeout/cleanup checks, existing source confinement, architecture and unsafe-boundary tests, strict lint, component security, exact CI steps or two-host proof. No generic or unsafe in-process fallback.
5. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** even after this bounded image-inspection slice, pending the other media, sandbox, catalogue and publication obligations. No AVIF, video, WebVTT, platform artwork, public route/HTTP/browser, publication, release or deployment.

## 2. Exact independent starting state — preflight BEFORE any write

Main current frozen head is **`3af6e00af6c60601b199e5d35611ec773e13f558`**, tree **`aa86132e848b763ebd6ed2a759565a56f42920c6`**, sole parent **`bcd18f3f042d234ab65a89c5742173a1b2e3c07e`**. Prove `HEAD == main == origin/main == git ls-remote origin main` at this exact commit before installing a technical change. The only pre-commit dirt may be the complete, unstaged Owner-installed WO-100 ACTIVE; never rewrite or normalize Owner bytes.

Baseline Git blob pins at this head:

| Path / fact | Exact required identity |
| --- | --- |
| `.documents/Reports/execution-report.md` (WO-099 STOPPED) | `7bb1f9e45dab3c46b9f420623c1d2fc5f35295ed` |
| `.documents/WIP/workorder-active.md` tracked WO-099 | `c1ac9e8512a623473cd158791da1e5c14df15676` |
| `.documents/WIP/plan-next.md` neutral | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Architecture/ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| `.documents/SOP/sop-architect.md` | `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` |
| `.documents/SOP/sop-implementation.md` | `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |
| `AGENTS.md` | `6eae1da8c721e13cea0c6689a1665e98fa5a9abb` |
| root `Cargo.toml` / `Cargo.lock` | `14505be49a281ba0495eabea5ca386548cc36347` / `b1ee56da60f36536548586528db371b7cbe508e4` |
| `rust-toolchain.toml` / `.github/workflows/rust-verify.yml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` / `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| `crates/solo-cli/tests/unsafe_boundary.rs` (accepted governance repair) | `57d7579616fc0bab96dcb1e371584f20cfd5d8f9` |
| `crates/solo-cli/tests/architecture_checks/source.rs` / `graph.rs` | `df1f8758b201214e5a4066823e83449f9c6cef6f` / `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| `crates/solo-adapters/src/lib.rs` / `media_worker_host.rs` / `media_worker_host_native.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` / `7eed6e158c9374a6c2055ddd98dd3287357b814a` / `eacf51c5353ab3278969961aa136a0951cb7ad67` |
| `crates/solo-application/src/graphic_asset_read.rs` / `media_worker.rs` | `6499421d6e718e9d46d64e45fb152f34e2989cc1` / `13811c5811c2170b811ed4a4705e0d940d6d21e7` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `22ee67e94cc96a70e8e3a47db9e575464b53cd07` |

Check remote refs: exactly 14 existing branches (main + 13 non-main); specifically `solo-wo-093-r01=0dc45185110e6136bd63dc7edf774bc52b753c9e`, `solo-wo-094-negative-r01=0e9b3677cf774b9e31ac8fc52605e88112522be6`, stopped `solo-wo-089-r01=1236915cab6e87b17ce84813430897b273fc3230`, accepted `solo-wo-090-r01=0eb3fcf645851ae752bd59e12fe32b69604197d2`. No `solo-wo-099-image-r01` or `solo-wo-100-negative-r01` remote ref. Confirm all 13 other original branch SHAs via snapshot; never modify them.

In the **one** checkout: local branch `solo-wo-099-image-r01` must point to **`f6e1137250d1204d588742b49ebce029714763a6`**, parent **`4b72282519f9193eed4a2409b64b58eec807a824`**, whose ancestry begins at governance repair **`bcd18f3f042d234ab65a89c5742173a1b2e3c07e`**. The older local `solo-wo-098-image-r01` remains at `ed938267001e4a1b20cc3001af8fafb3c82f40b5`. The retained patches exist and have exact Agent-reported sizes: `.temp/solo-wo-098-wip/unverified-attempt.patch` **244,647 bytes** and `.temp/solo-wo-098-wip/wo099-branch-work.patch` **180,065 bytes**. Treat these as **untrusted evidence to inspect, not automatic execution instructions**. Verify `git diff bcd18f3f..f6e11372 --name-only`, `git show --stat`, and local `git status` without writing. No unauthorized tracked or untracked additions. Preserve `.temp/solo-wo-073-wip/` (agent-reported four files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/`, `.temp/solo-wsl-target/` and Cline data. No global cleanup, shadow checkout, new remote, extra worktree or stash. If the local branch is unexpectedly missing/drifted, STOP and request explicit replacement authorization rather than silently recovering by guesswork.

**Only** the full, matching installed WO-100 handoff authorizes writes. One worktree, one `origin` with exact `https://github.com/provelopment/provelopment-foundation-solo.git` fetch/push URL, clean base other than ACTIVE, empty stash, all WIP four files/no WIP subdirs. STOP for any unexplained discrepancy.

## 3. Exact writable ledger and version-control restrictions

**Document paths:** `.documents/WIP/workorder-active.md` — Owner-installed byte-identical WO-100, **Commit A only**, remains ACTIVE until a later separately authorized closure; `.documents/Reports/execution-report.md` — final truth report **Commit D only**. NEXT/ROADMAP/OUTLOOK never change in WO-100.

**Exactly twelve technical paths** may differ on the technical branch and be promoted onto `main` (no extras):

1. `.github/workflows/rust-verify.yml` — new mandatory exact N097 gate and genuinely measured architecture counts only, no disabled prior gates.
2. `Cargo.lock` — exact reviewed image dependency graph only.
3. `crates/solo-adapters/Cargo.toml` — `image =0.25.10`, default features disabled, only png/jpeg/webp; existing windows-sys 0.61.2 minimum existing feature additions for Win32 APIs, no new direct packages.
4. `crates/solo-adapters/src/graphic_image_inspection.rs` — image codec, parent validation and test-safe seams.
5. `crates/solo-adapters/src/lib.rs` — existing newly approved module allowance shape and module/exports; never crate-wide unsafe.
6. `crates/solo-adapters/src/media_worker_host.rs` — safe supervisor and fixed closed internal modes.
7. `crates/solo-adapters/src/media_worker_host_native.rs` — **sole native process-control file**, positive per-OS-gated unsafe with adjacent `// SAFETY:` and strict handle ownership; extend existing Windows seam, no additional native file.
8. `crates/solo-application/src/graphic_image_inspection.rs` — inward typed all-or-refused policy.
9. `crates/solo-application/src/lib.rs` — narrow public exports for the inward contract.
10. `crates/solo-cli/src/main.rs` — closed internal-mode composition, public CLI unchanged.
11. `crates/solo-cli/tests/architecture_checks/graph.rs` — exact audited `image` root and locked graph closure, no widened permissive rule.
12. `crates/solo-cli/tests/media_worker_host_smoke.rs` — true two-host tests and mandatory digest-guard mutation sensitivity.

**Protected and read-only:** `crates/solo-cli/tests/unsafe_boundary.rs` (already approved and green), `architecture_checks/source.rs`, `check_architecture.rs`, existing `graphic_asset_read.rs`, AVIF protocol/application modules, source-read internals, Architecture, SOP, AGENTS, README, other Cargo files, `deny.toml`, toolchain, action pins, deployment/provider configs and all Owner data. Any needed addition outside the 12-path technical ledger is a STOP—not implied authorization. No unlicensed fixture downloads or new test file: use generated in-memory data and the accepted fixture-bearing workflow.

**Permitted Git actions:** one ordinary main issuance commit (ACTIVE only); continue **existing** local `solo-wo-099-image-r01` from its verified frozen head, ordinary local commits there, first push after local mandatory gates succeed; `workflow_dispatch` existing `rust-verify.yml` on branch at exact green candidate SHA; creation/push of exactly `solo-wo-100-negative-r01` from the green technical head with one deliberate guard-removal change; dispatch its intentional RED run; exact Git blob restore of 12 verified green technical paths into main for one integration commit; one canonical report-only final commit. No PR/merge, cherry-pick, rebase, amend, reset --hard, force push, branch deletion, tag, release or deploy. No writes by Architect. The negative branch must never be merged or promoted.

## 4. Commit A — issue the new authority, prove baseline green

At main `3af6e00a...`, stage **only** installed `.documents/WIP/workorder-active.md`, verify exact Owner bytes, one `M` path, no other path touched. Its Markdown trailing-hard-break whitespace may be preserved and specifically disclosed as in prior orders; the Agent must not normalize Owner text. Commit forward only and push `main`, read back full SHA/tree/sole parent/ACTIVE blob and new exact-SHA two-host `push` CI run attempt 1. Both Ubuntu and Windows must be green before branch execution. This is a green baseline with the accepted fixed unsafe checker; do not change it on main during branch repairs. Do not edit NEXT.

## 5. Mandatory direct Windows correction — implementation recipe, not a vague suggestion

Work **only** on the verified local `solo-wo-099-image-r01` and primarily in `crates/solo-adapters/src/media_worker_host_native.rs`. The existing WO-099 typecheck and partial fixes are not acceptance; re-check source before touching it. Preserve `OwnedHandleV1`, its correct ownership/cleanup, failed-readback refusal before resume, required 4 Job Object limits (`PROCESS_TIME`, `PROCESS_MEMORY`, `JOB_MEMORY`, `KILL_ON_JOB_CLOSE`), the Linux boundary, and all exact new/old tests. Every unsafe block must be inside its positively `#[cfg(windows)]` or Linux scope with a local SAFETY rationale.

### 5.1 Exact handle allowlist — `STARTUPINFOEXW`

In the **existing Windows native module**, add a small RAII owner for exactly **one initialized attribute list** and its correctly aligned backing storage. Use official `windows_sys::Win32::System::Threading` names and signatures present in 0.61.2; **do not invent signatures** or add an FFI dependency. The required sequence is:

1. Call `InitializeProcThreadAttributeList(null_mut(), 1, 0, &mut bytes_needed)` for the documented **size probe**. This first call is expected to return 0; demand a nonzero, sane bounded size. A surprising success or zero/oversized value refuses.
2. Allocate correctly aligned, stable backing storage, e.g. checked `Vec<usize>` sized by `(bytes_needed + align_of::<usize>() - 1) / size_of::<usize>()` (checked arithmetic), and do not reallocate while Windows holds its pointer. Initialize the real list with `InitializeProcThreadAttributeList(list_ptr, 1, 0, &mut bytes_needed)`. On failure, refuse and free without calling `DeleteProcThreadAttributeList` on an uninitialized list; on success, RAII invokes `DeleteProcThreadAttributeList` **exactly once** before releasing backing storage, on every exit.
3. The child **only** requires two inherited pipe handles: its **stdin read** and its **stdout write**; `hStdError` may point to the same stdout write handle to preserve the currently accepted bounded output contract, so do not create an extra undeclared stderr handle. Build one fixed `[HANDLE; 2] = [stdin_read, stdout_write]` array and keep it live through `CreateProcessW`; both entries must be valid, distinct, inheritable child ends. The parent ends (`stdin_write`, `stdout_read`) must be noninheritable. No provider handles, process/job handles, parent stderr, arbitrary files, tokens, sockets or ambient std handles are admitted.
4. Call `UpdateProcThreadAttribute(list_ptr, 0, PROC_THREAD_ATTRIBUTE_HANDLE_LIST as usize, handles.as_ptr().cast(), size_of_val(&handles), null_mut(), null())` using the **actual `windows-sys` ABI types**. A failure refuses. Keep the array and list storage alive until process creation returns. If Win32 reports unsupported host capability, refuse; never use the old unrestricted `STARTUPINFOW` fallback.
5. Use `STARTUPINFOEXW` (not `STARTUPINFOW`), set `StartupInfo.cb = size_of::<STARTUPINFOEXW>()` checked into `u32`, `dwFlags=STARTF_USESTDHANDLES`, `hStdInput=stdin_read`, `hStdOutput=hStdError=stdout_write`, and `lpAttributeList=list_ptr`.
6. Call `CreateProcessW` with explicit, validated full `lpApplicationName` for this one trusted executable, writable UTF-16 quoted command line, no generic executable/path selection, `bInheritHandles=TRUE` **only in combination with the validated handle list**, flags **`CREATE_SUSPENDED | EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT`**, a **non-null owned minimal environment pointer** and an explicit controlled working directory (prefer the verified executable parent, no arbitrary sourced path). The `lpStartupInfo` argument is the address of `startup.StartupInfo` because the Win32 API accepts the base structure pointer. No executing code before creation succeeds, job limits are read back, assignment is confirmed and explicit `ResumeThread` succeeds.
7. On every early return, close each owned process/thread/job/pipe handle exactly once, terminate the owned suspended process when one exists, require a checked wait, and avoid a pipe-reader join until the child is confirmed exited. An unconfirmed wait/cleanup is a typed refusal and must never be reported as successful containment. Retain precise error classifications instead of an unexplained success flag.

**Illustrative layout only (confirm exact pointer types against rust-std/windows-sys 0.61.2 before compiling):**

```rust
// Inside the positively #[cfg(windows)] native module only:
let mut startup: STARTUPINFOEXW = unsafe { std::mem::zeroed() };
startup.StartupInfo.cb = u32::try_from(std::mem::size_of::<STARTUPINFOEXW>()).ok()?;
startup.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
startup.StartupInfo.hStdInput = stdin_read;
startup.StartupInfo.hStdOutput = stdout_write;
startup.StartupInfo.hStdError = stdout_write;
startup.lpAttributeList = attributes.list_ptr();  // live initialized RAII owner
let handles: [HANDLE; 2] = [stdin_read, stdout_write];
// Check UpdateProcThreadAttribute(... PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
//    handles.as_ptr().cast(), size_of_val(&handles), ...) == TRUE.
// Only then CreateProcessW(..., TRUE,
//    CREATE_SUSPENDED | EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT,
//    minimal_environment.as_ptr().cast(), controlled_cwd.as_ptr(),
//    &startup.StartupInfo, &mut process_info).
// Confirm real Job Object limits and membership before ResumeThread.
```

This snippet **does not** approve skipping any initialization, handle, environment, termination or checked-wait step. Compile and verify the actual exact signatures, pointer lifetimes, RAII Drop behavior and error routes. Microsoft documents `STARTUPINFOEX` + `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` as the explicit handle-inheritance control; `bInheritHandles=TRUE` without that list is not equivalent.

### 5.2 Minimal noninherited environment

Build a fresh **double-NUL-terminated UTF-16** environment block and pass its pointer as `lpEnvironment`; **do not pass null** or snapshot/copy the full parent environment. Allow only the minimum documented OS loader/runtime values that the trusted, fixed internal worker demonstrably requires, starting from `SystemRoot` (and `WINDIR` only if proven necessary). For each admitted value: derive it from a trusted Windows OS query or one explicitly named OS variable, reject missing/empty/invalid UTF-16/interior-NUL/oversized values, validate an absolute safe path where appropriate, and preserve its precise case-insensitive key. Never forward `PATH`, `USERPROFILE`, `HOME`, `TEMP`, `TMP`, `APPDATA`, `GITHUB_*`, tokens, repo/Owner-specific secrets, `RUST*`, `CARGO*`, cloud/provider variables or arbitrary inherited names. Windows-created process environment grammar must be obeyed (sorted names if multiple; correctly formed `name=value\0` pairs and a final terminating `\0`). Do not rely on ambient current directory; pass a deliberately selected valid cwd bound to the trusted executable location and verify that this cannot select a different executable. If the worker requires more keys on Windows, justify and test each fixed key, never fall back to inheriting all environment values.

**Security limit honesty:** Job Objects and handle/environment minimization **do not constitute** filesystem, network or syscall sandboxing. Continue to refuse where the selected OS control is unavailable; no claim of complete hostile-worker sandbox, race-free Linux process group pinning or aggregate host resource guarantee.

### 5.3 Windows negative proof — make allowlisting load-bearing

Within the existing `media_worker_host_smoke.rs` plus narrowly necessary closed test seams in the already authorized adapter/native files, prove on **actual Windows**:

- A deliberately created **extra inheritable canary handle**, not present in the attribute list, is **not accessible** from the child. Use a distinct, safe kernel event or pipe handle and pass its numeric value only through a dedicated test token/protocol; child verification must establish the handle is invalid/not inherited, not merely assert a local array contains two members. If the numerical value could alias a different valid handle, disambiguate via an unambiguous canary operation/identity rather than accepting a false positive.
- An injected parent-only environment canary (fixed harmless test variable) is **absent** in the child; required minimal keys work. The positive worker must run with the sanitized environment, not an alternate test-only spawn path.
- The exact closed worker is created **suspended**, fresh job constructed with all four limits installed/read back, actual process assignment and membership confirmed, then and only then resumed; actual PID, reported OS limits, wall time, normal exit and checked wait/reap are captured in bounded nonsecret `n100_*` log markers.
- Failed size probe/allocation/attribute init/update, failed pipe validation, unexpected inherited canary, invalid/missing env, denied Job Object set/readback, failed assignment/membership, `ResumeThread` failure, worker nonzero/panic/timeout, failed termination/wait, output/hash mismatch all refuse with **no positive inspection result**. Use test-safe dependency/injection seams where kernel failure cannot deterministically be induced; never replace all real-host observation with fakes.
- The Windows runner must execute true `n097_*` PNG/JPEG/WebP positive and adversarial tests. Actual host measured outputs—not copied text—must appear in its logs. Linux must show its own real readback/termination proof, and previous N079–N093 tests continue green. No N097 true-positive may be claimed from an in-process decode or `cargo check` alone.

## 6. Fast local feedback loop — mandatory before first technical push

Re-use installed WSL 2 Ubuntu-26.04 on the **same one Windows checkout**; do not create another checkout. Rust 1.99.1 with `rustfmt`, `clippy`, `cc`, and user-scoped MSVC target `rust-std` were Agent-reported already installed. Verify real versions rather than silently reinstalling. Use a build cache under existing `.temp/solo-wsl-target/` without clearing Owner data.

After each bounded source change run, in order (within WSL Linux):

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test -p solo-cli --test unsafe_boundary --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-cli --test check-architecture --locked -- --self-test
cargo test -p solo-cli --test media_worker_host_smoke --locked n097 -- --nocapture --test-threads=1
cargo check --target x86_64-pc-windows-msvc -p solo-adapters --all-targets --locked
```

The **five filesystem fixture suites** may refuse locally without the actual CI R03 filesystem fixture (the Agent reported this accurately). Record `LOCAL-UNVERIFIED` for them rather than installing missing system packages or weakening tests. All mandatory suites must run in two-host GitHub CI. A Windows target `cargo check` in WSL is **type-check only**, not Windows runtime evidence. If the targeted N097 positive tests require CI filesystem fixtures, run the local negative/timeout subsets (`n097_09`, `n097_10`, `n097_11`) and state why positives were locally unverified, but do not skip the full branch CI.

**Do not repeatedly push known compilation/test failures.** However, if Linux-local tests are blocked solely by absent real CI filesystem fixtures, this does not prohibit a single first branch CI once everything else locally checkable is green and native Windows type-check passes. Diagnose and fix all additional test/architecture/Clippy defects inside the ledger before calling any revision verified.

## 7. Technical branch CI and acceptance — must prove both hosts

The technical branch is the **existing local** `solo-wo-099-image-r01`. All branch commits are forward-only descendants of the pinned local `f6e11372…`. Before first remote push, verify the base/parent, that all code changes remain the twelve authorized technical paths relative to `bcd18f3f…`, that the accepted `unsafe_boundary.rs` blob remains unchanged, and that no new tracked file or Owner data is touched. **Never push a half-completed unsafe worker**: all handle/environment gates from §5 must be implemented, explicit tests installed and locally checkable code clean first.

Push the branch and dispatch the existing `rust-verify.yml` using `workflow_dispatch` at the exact technical SHA. Require **both** `ubuntu-24.04` and `windows-2025` jobs to complete successfully, attempt 1 for each run. Inspect actual logs, not just green badges: strict lint; locked fetch; `cargo-deny` 0.20.2 with fresh RustSec DB, advisories/bans/sources/licenses (the pre-existing `license-not-encountered` unused allowance warning may remain disclosed, never silenced); complete workspace tests; `n097` positive and adversarial tests; `n100` Windows handle/env negative canaries; `architecture=legal` actual measured package/target/file/edge counts; self-check and CI tripwire; original N079–N093 fixed evidence; release smoke; reproducibility; job-host applicability. Update workflow's exact architecture count pins only from a **real passing** checker result, and preserve both host security and original mandatory checks. The old baseline was `packages=4 targets=33 files=60 edges=41`; do not blindly guess new counts.

If CI fails, preserve the full logs/SHA; fix locally on the same branch, push another ordinary forward commit only after local checks, dispatch fresh two-host CI. No bypass, output redaction of evidence, compiler warning suppression, added `unsafe` outside the reserved seam, test ignore, or synthetic-success workaround. No exact-head green = no positive verdict, no mutation branch and no main promotion.

## 8. Mandatory deliberate RED sensitivity proof

Only after one exact technical SHA is truly green on **both** hosts, create `solo-wo-100-negative-r01` at that green SHA; remove **only one load-bearing refusal guard** from the *production* parent validator in `crates/solo-adapters/src/graphic_image_inspection.rs`. The preferred mutation is the tested decoded-output digest comparison that refuses a worker's forged digest (old WO-097 `validate_image_worker_run_v1` block `received_digest != frame.output_digest`; confirm the current actual branch). Leave every test, workflow and other source unchanged. Commit only this one file, push red branch, run existing `workflow_dispatch` at exact negative SHA; require compile, format and strict Clippy to pass **before** the targeted `n097_09` forged-output test fails at execution. If this does not fail, STOP and do not claim sensitivity. Keep the red branch as evidence, never merge its mutation. Return to exact green technical branch and re-prove the guard is present and the code Git blob matches its pre-mutation object.

## 9. Main integration, final report, end state

After two-host green branch + intentional red sensitivity, switch back to clean green `main` at Commit A, confirm it has not advanced. Promote **only exact Git blob objects** for all twelve permitted technical paths from the green technical SHA, e.g. path-scoped `git restore --source=<GREEN_SHA> --staged --worktree -- <twelve paths>` where non-existent-at-main new files are intentionally added. Verify for every path `git rev-parse <GREEN_SHA>:<path> == git rev-parse :<path>` (including new files); report the complete path list and object hashes; preserve accepted checker and Architecture exactly. **No merge, cherry-pick, source authorship on main, or negative object.** Make exactly one forward-only main technical integration commit whose parent is new issuance Commit A. Push and require a **new `push` run** at the exact integration SHA, green Ubuntu and Windows, with actual N097 and N100 OS/format evidence (branch green does not substitute for on-main green).

Write **only** `.documents/Reports/execution-report.md` in a final report commit (sole parent integrated main). Report `status=COMPLETE`, required outcome *pending independent Architect review*, and all exact evidence **only if every gate passed**. Otherwise report `status=STOPPED`, actual partial commits, CI and blocker, never pretend success. Do not include the future report commit SHA/run IDs in the report before they exist; disclose the report run and jobs in the handoff. Require a new exact-SHA two-host green report `push` run; red report CI is STOP. The final main Git tree must be clean and current, ACTIVE must still be issued WO-100 (not neutral), NEXT neutral, ROADMAP/OUTLOOK/SOP/Architecture/protected files identical, original 13 non-main branches unchanged, one extra green technical branch and one intentionally-red negative branch preserved, no stash or second checkout, ignored Owner material intact. Cleanup *only* positively task-owned scratch explicitly authorized by this run; retain earlier WO-098/099 patch and cache unless the Owner expressly directs cleanup. Keep one canonical report, exactly four WIP files, no new WIP item.

**Final evidence checklist:** commit SHA/tree/sole parent and exact path/blob ledger for A, each technical revision, negative, integration, final report; exact CI run/job IDs, event/branch/head SHA/attempt per host; architecture counts/self-test/CI tripwire; real Ubuntu/Windows limits/identities/deadlines and confirmed cleanup; Windows separate handle-inheritance and environment canary negatives; fixed-format decoded golden/hash observations and whole-inventory refusal; complete RustSec/security/license notes; deliberate-red assertion after successful compile/Clippy; preserved refs and Owner-local inventory; WSL local checks plus explicit local fixture limitations. Do not call STOPPED WO-097/098/099 accepted. Do not claim full AVIF, Windows filesystem/network sandbox, total publication/media correctness, release or Stage closure.

## 10. Hard STOP conditions and escalation

STOP and publish a truthful report if (a) mismatch in exact main/branch pins or Owner-installed authority; (b) missing or drifted local branch/retained inputs that cannot be proven; (c) need for a 13th technical path or unapproved native crate, system install, extra checkout or broader unsafe exception; (d) an untrusted decoder starts before verified constraints; (e) no explicit handle allowlist or no minimal environment; (f) any possible leak, wrong handle ownership, failed job readback, failed assignment, resumed unverified worker, child survival/uncertain wait, protocol overflow or false positive; (g) any red mandatory architecture/security/test/OS CI after correction attempts; (h) absence of real Windows proof; (i) negative-mutation failure is only at compile/Clippy or does not fail the intended guard test; (j) inability to reproduce exact green technical blobs onto main; (k) need to weaken previous N079–N093 behavior or claim forbidden Stage/product authority. No independent Architect acceptance can be issued based on Agent assurances alone.

**Handoff:** Submit the canonical GitHub report URL, full exact revision/CI/job evidence and explicit `COMPLETE` or `STOPPED` claim for independent Chief Master Architect ACCEPT / REVISE / STOP. The Owner alone installs the order and activates the Implementation Agent. The Chief Master Architect is GitHub read-only and makes no repository or workstation writes.
