# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-080 R01**  
**Title:** R03.C3 — exact five-blob promotion of the independently proven Linux pre-exec limit smoke onto `main` (integration only)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact Owner → Implementation Agent activation ID:** `SOLO-WO-080 R01`  
**Authority:** This complete Owner-installed ACTIVE file becomes executable only upon the Owner's separate matching handoff. The Implementation Agent alone performs expressly authorized filesystem/Git/GitHub writes, CI triggering and reporting. The Chief Master Architect has permanent **READ-ONLY** GitHub authority and independently reviews the final exact revision. This is an integration work order for **already accepted** code, not a new implementation, merge, release, deployment or Stage 03 closure.

## 1. Decision, bounded objective and result vocabulary

Promote **exactly five** source/test Git blobs from the accepted `SOLO-WO-079 R01` technical-branch commit `380393ddacf37fc1a6398805235ca2d5dbd9677e` to `main` **byte-for-byte** via path-scoped staging. Keep the original `SOLO-WO-079` implementation branch and all historical branches unchanged. Do **not** merge, cherry-pick or rebase the whole technical branch. After the five-path integration, require the existing, original, push-triggered `rust-verify` workflow to complete successfully on both `ubuntu-24.04` and `windows-2025` at the **exact integration SHA**. Publish a separate canonical report-only commit on `main` and require another complete both-host successful workflow at that **exact report SHA**.

The promoted code proves only the accepted, fixed, trusted-child smoke: the Linux parent re-executes the same trusted `solo-cli` in a fixed private mode; a positively gated native `pre_exec` boundary calls `setsid` and installs hard and soft `RLIMIT_AS = 536,870,912` bytes and `RLIMIT_CPU = 10` consumed-CPU seconds **before `exec`**; the resulting child reads back its own limits and group identity; the parent revalidates the bounded evidence, requires normal zero exit, checks a 15,000 ms wall deadline and reaps the single leader. Windows compiles a fail-closed `HOST_UNSUPPORTED` path. Public CLI accepted/refused byte-contracts remain unchanged.

**Result vocabulary:** `LINUX-PREEXEC-LIMIT-SMOKE-INTEGRATED` **only** if all S080 and G080 gates below pass, including exact blob equality and both successful workflow revisions; `BLOCKED` if a mandatory requirement actually fails; `UNVERIFIED` if material evidence cannot be obtained. The result is a **single benign launch and readback**, **not** `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED`, sandboxing, a released feature or an AVIF decoder. Limit **enforcement under stress** is not tested; `RLIMIT_AS` is virtual address space, not RSS nor aggregate child-tree memory; `RLIMIT_CPU` is CPU time, not wall time; process group creation does not prove descendant cleanup. The existing bound of the parent reader threads after leader exit is **not** an independent proof against inherited-pipe descendants. The assumptions around `current_exe()` identity, parent attestation, hostile child behavior, filesystem/network/credential access and process-tree cleanup remain unverified. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; media-worker publication and media-correctness authority remain false.

**Strict integration-only rule:** No new Rust source code, implementation edits, test edits, configuration, dependencies, lock changes, checker changes, workflow changes, Cargo graph changes, refactoring or bug-fix commits are authorized. The five imported files must match the accepted branch Git blob identities exactly. If any mandated gate fails after promotion, **STOP** and report `BLOCKED`/`UNVERIFIED`; do not repair it under this order by changing a sixth path or editing an imported file. A future order must separately authorize any correction. Do not invent a new technical branch, new clone or second worktree.

## 2. Immutable preflight before any non-Owner edit

In the **one** actual checkout, inspect the Git root, `git status --porcelain`, current branch, `HEAD`, `origin/main`, `HEAD^{tree}`, worktrees, remotes, `git ls-files`, WIP inventory and ignored `.temp` inventory. The **only** permissible initial dirty tracked path is the Owner-installed replacement of `.documents/WIP/workorder-active.md`, whose first lines identify this **exact** `OWNER-ISSUED — SOLO-WO-080 R01` and matching activation. All other tracked and untracked state must be clean or explicitly safely resolved **without** destructive operations; wrong anchors or unknown material mean STOP. Do not presume the workstation has a linker: CI is the build/test authority, and the lack of local compiler/linker is not by itself a STOP if exact CI evidence is obtainable.

| Required anchor | Exact Git value to confirm |
| --- | --- |
| `HEAD == main == origin/main` | `7f9ed68348998bef6557c657c0be3580bb8d5fe8` |
| `HEAD^{tree}` | `b4bee9da3043e9805f917f5431c06fd5fe2bbf4f` |
| Current canonical report blob | `b780054e45ced229cd6774d36527551583e5f66b` (`SOLO-WO-079 R01`, accepted bounded branch proof) |
| Previous ACTIVE blob at starting HEAD | `a8da8eef897ba57d9e097b0a791fd1dfe60cb044` |
| Architecture Charter blob | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `rust-verify` workflow blob | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` (two exact `edges=41` assertions) |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| Adapter manifest / root | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` / `bd1d4c58f5a91f38af63e84e143c850d6372ca64` |
| Mainline native **readback-only** file | `2df6d0e98395f21fc72294606e9a97e954f97c63` |
| Mainline CLI root | `6faa9fae29351123a49d45c5637ac4fbb1b615c8` |
| Existing architecture graph / unsafe guard / workspace graph | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` / `b88e7b42ab03bd78557e58d4c54b957b59369d6c` / `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Accepted source branch, local and remote | `solo-wo-079-r01` at `380393ddacf37fc1a6398805235ca2d5dbd9677e`, tree `18215c5ad9134e05baab3330bd378ec18b06e6ec` |
| Previous technical branch (immutable) | `solo-wo-076-r01` at `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Historical branches (immutable) | `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Previous branch CI, exact proof | `rust-verify` run `43226954406`, Ubuntu `131212490637`, Windows `131212489774`, both success at accepted `380393dd…` |
| Previous main report CI | run `43228440597`, Ubuntu `131217623871`, Windows `131217623506`, both success at starting HEAD |
| Repository topology | one checkout/worktree, exactly one `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, exactly four WIP files |

The two new files `crates/solo-adapters/src/media_worker_host.rs` and `crates/solo-cli/tests/media_worker_host_smoke.rs` **must be absent from starting `main`** and present on the accepted technical branch. Verify the exact five branch blobs listed below *before* staging. Independently read the actual accepted code and Architecture §6A.8 plus SOP `sop-architect.md` §§6A–7B, §8, and `sop-implementation.md` §§4–5C. Confirm the branch was not changed or superseded since the Architect accepted it. Do not delete or rewrite either technical branch. For `.temp`, verify `.temp/solo-wo-073-wip/` is still the four retained historical reference files totalling 46,062 bytes when present; it is uncompiled/unaccepted source and **must not be imported**. The earlier report found no Owner archives at `.temp/` top level; independently inventory rather than assuming presence or deleting unfamiliar files.

## 3. EXACT permitted permanent path ledger — seven unique paths total

The **only** permanent writes under this order are:

1. `.documents/WIP/workorder-active.md` — Owner-installed replacement, issued **alone** in the initial commit; no Agent editing/rewording of Owner source or other WIP file.
2. `crates/solo-adapters/src/media_worker_host_native.rs` — exact accepted revised native blob.
3. `crates/solo-adapters/src/media_worker_host.rs` — exact accepted new safe façade blob.
4. `crates/solo-adapters/src/lib.rs` — exact accepted module declaration/export blob.
5. `crates/solo-cli/src/main.rs` — exact accepted private internal CLI dispatch blob.
6. `crates/solo-cli/tests/media_worker_host_smoke.rs` — exact accepted new smoke/refusal test blob.
7. `.documents/Reports/execution-report.md` — one final canonical report-only commit after successful integration CI.

The five code files **must be staged and committed as one single integration commit** with exactly their accepted blob identities:

| Exact permitted code path | Starting `main` | Accepted technical-branch blob (required on integration commit and report HEAD) |
| --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `2df6d0e98395f21fc72294606e9a97e954f97c63` | **`d10151e40e807c9831491ba0da0eb44d08a81589`** |
| `crates/solo-adapters/src/media_worker_host.rs` | **ABSENT** | **`97efbbf0c7de425cd00f79f126ed05c9f8b13e8d`** |
| `crates/solo-adapters/src/lib.rs` | `bd1d4c58f5a91f38af63e84e143c850d6372ca64` | **`1550d34f820642bb5573e06d4fccb758109fd26c`** |
| `crates/solo-cli/src/main.rs` | `6faa9fae29351123a49d45c5637ac4fbb1b615c8` | **`b3c89ad0ce256f69e5763855559d20757554c461`** |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | **ABSENT** | **`1f127001359e5f0ba941f92906f3280e433d8b9f`** |

All other tracked paths are **immutable**: especially `.github/workflows/rust-verify.yml`, `Cargo.lock`, every `Cargo.toml`, `deny.toml`, `rust-toolchain.toml`, `.documents/Architecture/ARCHITECTURE.md`, the three plan files, all SOP/AGENTS/security files, the entire source-reader and media-worker protocol/application/domain code, the existing checker and negative fixture files, other CLI code and all existing test suites. The exact four-crate set, named external normal roots (including Linux-only pinned `libc =0.2.191`), `41` approved dependency edges, source-scope restrictions and the two existing OS-gated native `allow(unsafe_code)` exceptions remain untouched. Do not modify `.gitignore`, CI settings or secrets.

No other permanent file or directory may be created, changed, removed, staged, committed, force-added or pushed. The five files must **not** be patched, rustfmt-rewritten or normalised by the Agent after copying. Running read-only fmt/check/test commands is allowed; if a tool attempts to rewrite them, inspect and STOP rather than accepting a blob mismatch. All user/Owner content, current local `target/` and historical branches are protected.

## 4. Required Git sequence (separate approval for each class is already in this Owner-issued order)

**4A — Owner-installed issuance on `main`:** After exact §2 preflight, stage **only** `.documents/WIP/workorder-active.md`, confirm the staged file is the entire matching Owner-issued WO-080 text, make one issuance commit with that **single changed path**, normal fast-forward push to `origin/main`, then independently verify the remote commit SHA/tree/parent/ACTIVE blob and exact one-path ledger. Require the **unchanged** push-triggered `rust-verify` workflow to succeed on both hosts at this issuance SHA before proceeding. Failure or wrong commit means STOP; never substitute a green check at a different ref.

**4B — Accepted-branch blob equality:** Keep the existing checkout on `main`. Verify local and remote `solo-wo-079-r01` still equal `380393ddacf37fc1a6398805235ca2d5dbd9677e` and all five blobs equal §3. Verify clean main after issuance. Use exactly the authorized **path-scoped** staging technique, naming the five paths literally in the pathspec (not `.` or a directory):

```bash
git restore --source=380393ddacf37fc1a6398805235ca2d5dbd9677e --staged --worktree -- \
  crates/solo-adapters/src/media_worker_host_native.rs \
  crates/solo-adapters/src/media_worker_host.rs \
  crates/solo-adapters/src/lib.rs \
  crates/solo-cli/src/main.rs \
  crates/solo-cli/tests/media_worker_host_smoke.rs
```

On Windows PowerShell use valid command-line continuation syntax or a single line, **not** a literal POSIX backslash continuation copied blindly. Check `git diff --cached --name-status`, `git diff --cached --check`, `git diff --cached --numstat`, `git ls-files -o --exclude-standard`, `git status --porcelain` and `git rev-parse :<each path>` for every staged blob. Require exactly **3 modified + 2 added** paths, no other staged/dirty files and **five exact SHA matches**. A merge, checkout that swaps entire trees, `cherry-pick`, staged `.` wildcard, rebase, extra worktree, clone, branch write, force-push, or trust in display names instead of Git blobs is forbidden. No code edit or “cleanup” is authorized. If exact staging is unavailable, STOP.

**4C — Single integration commit on `main`:** Commit just the five staged paths once (clear bounded message such as `feat: integrate proven Linux preexec limit smoke (SOLO-WO-080)`), normal fast-forward push, re-read the commit's exact SHA/tree/parent, per-path status/blob and `origin/main` from remote; verify no changed path outside the five. The five remote blob SHAs must still match §3, and no other tracked blob may have changed relative to issuance. Confirm existing technical/historical branch refs remain byte-for-byte unchanged.

**4D — Mandatory integration CI:** At the **integration commit** obtain the original `rust-verify` push-run, confirm its `head_sha` exactly equals that integration SHA and `event=push`, both `ubuntu-24.04` and `windows-2025` jobs report `success` and all **applicable** configured steps actually executed. Inspect logs for the G080 proof matrix below, including the own-child kernel readback and Windows refusal. If any applicable step fails or is skipped owing to earlier failure, STOP. This is intentionally a CI re-proving order on `main` rather than reuse of the previously green technical-branch run. **Do not** silently dispatch against the wrong SHA or accept only job-level status.

**4E — Canonical report only:** On clean `main` at the accepted integration SHA, overwrite only `.documents/Reports/execution-report.md` with an honest complete WO-080 report. Commit just that one file, fast-forward push, read back exact remote report SHA/tree/parent/blob/path; verify `main == origin/main == HEAD` and normal clean Git status. Require the original push-triggered `rust-verify` on **both** hosts again at the report commit SHA. Return the full final SHA/tree/blob and the run/job identifiers in the chat handoff — the report cannot embed its own future SHA. No new branch, PR, merge, release or next order is authorized.

**Failure discipline:** If a late CI failure occurs, do not hide it or claim rollback as harmless. Preserve commits and Owner state, disclose the exact red run/step, do not reset/rewrite published history, and publish a factual `BLOCKED`/`UNVERIFIED` report if safe. No unauthorized corrective source/file changes are allowed.

## 5. Mandatory behavior and security review (no changes allowed)

The exact accepted source bytes must still demonstrate the following after import:

- **Native scope:** `crates/solo-adapters/src/media_worker_host_native.rs` is the sole process-native boundary. The two pre-existing `getrlimit`/`assume_init` `unsafe` expressions plus two added `pre_exec`/`getpgid` expressions are each protected by positive `#[cfg(target_os = "linux")]` and a local `// SAFETY:` rationale. The only `pre_exec` operations are `setsid` and the two fixed `setrlimit` syscalls. No global allow, no FFI in safe façade or CLI, no new native root, no Windows process control and no dynamic loading.
- **Linux exact profile:** Address-space hard/soft both **536870912 bytes** and CPU hard/soft both **10 seconds**; the child reports both, and the parent checks *all* fields rather than trusting child text. Session/process group identity requires `pgid == pid`, both positive. This proves the new group for the one benign child but not descendant accounting or kill coverage.
- **Closed command:** The parent launches the same `current_exe()` with the private `--internal-media-host-probe-v1 child` two-token mode, no caller-specified executable, shell, command, path, environment-controlled limit, network, source content or media operation; stdin closed; bounded stdout/stderr; one fixed accepted line; malformed, partial, duplicate, overlong, non-UTF-8, wrong-exit or wrong-value output is refused.
- **Fail-closed deadline and reaping:** The one leader has a parent wall deadline of 15000 ms, `try_wait` loop, kill+wait on deadline and closed refusals. This order does **not** assert a tested timeout-kill, safety against inherited pipes, survivor-free process tree, hostile group escape or general sandbox. Note these limitations explicitly even when CI is green.
- **Windows:** No Linux-native `unsafe` or `libc` code is linked or executed; parent and child internal modes return `HOST_UNSUPPORTED` at exit 3 without launching a child. Legacy public CLI exit 0/2 behavior and byte-exact payloads remain unchanged on both hosts. Existing Windows source-read native boundary unchanged.
- **Architecture/supply chain:** Four crates only; strict inward port/adapter flow; exactly 41 graph edges; existing two gated module allowances; unchanged pinned `libc`, `rustix`, `windows-sys`, Cargo.lock, deny policy, workflow, source-boundary guards and all refusal fixtures. Network/secret/publication authority remains unchanged.

A green mainline integration should never be described as full host containment. Child stress, smaller resource-cap trip tests, descendant kill/sweep, independent wall watchdog negative, concurrent quota, Windows Job Objects, decoder quality, source-confinement/least-privilege and whole-publication validation are future **Owner-issued** orders, not hidden acceptance criteria of this integration.

## 6. Actual G080 proof matrix — must be observed at the integration SHA on two hosts

| Proof | Mandatory observation / failure condition |
| --- | --- |
| `G080-01` | Exact five staged and remote blob SHAs from §3; 3 M / 2 A, all other files unchanged; accepted technical branch and historical refs preserved. |
| `G080-02` | Real Ubuntu `n079_01_the_real_child_observes_the_installed_limits ... ok` and actual logged observation with `as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=<positive> pgid=<same positive> wall_ms=<15000`; values must come from the real CI log at this **main** SHA, not copied from the prior branch run. |
| `G080-03` | Real Ubuntu direct unsupervised child `n079_04_a_direct_unsupervised_child_is_refused ... ok`, fixed refusal within 71–73, empty stdout and no accepted evidence; code does not claim parent authentication. |
| `G080-04` | All real CLI internal-argument, parser, profile, exit-status, malformed-evidence and binary-refusal tests execute and pass. `N079-02` remains asserted inside `n079_01`, not falsely reported as a second named test. |
| `G080-05` | Windows `n079_05_both_internal_modes_are_unsupported_before_any_child_exists ... ok`, no `n079_01`, `n079_04` or `n076_*` Linux test execution, both public CLI accepted/refused byte contracts unchanged. |
| `G080-06` | Existing Ubuntu `N076-01..03` kernel readback tests still pass, with original source-read and protocol/media-worker test regression intact. |
| `G080-07` | Real-tree architecture on both hosts is `architecture=legal packages=4 targets=33 files=60 edges=41`; the workflow's two unchanged exact `edges=41` checks pass; `self-check=passed cases=29`; dynamic loading and checker-bypass negative controls remain refusing. |
| `G080-08` | Both hosts execute `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-targets --locked`, pinned toolchain and `cargo-deny 0.20.2` component security gate with fresh RustSec. No weakening, hidden skip or new dependency. |
| `G080-09` | Both hosts pass their applicable test-failure tripwires, locked release build, `release_smoke=verified accepted_exit=0 refused_exit=2`; Ubuntu forbidden-import/bypass refusal restores source byte-for-byte and two clean release builds compare byte-identically. Record OS-specific raw `Cargo.lock` SHA-256 values as **host checkout bytes**, never incorrectly equate them with the shared Git blob. |
| `G080-10` | Existing `rust-verify` workflow is successful, with **zero applicable failures/skips**, at both the exact integration SHA and again at the report-only final SHA; output preserves the narrow nonpublishing claim. |

Expected configured steps: 33 per host with ordinary host-inapplicable skips (Ubuntu skips Windows-only steps; Windows skips Linux-only steps). A deliberately injected **failure-tripwire negative control** may log `test result: FAILED` but must be independently proved to be the *expected* refused fixture and the step itself **must succeed**; never misreport a genuine failing suite as a successful negative control. Do not invent process metrics or infer a passed host test from a green status without checking its log.

## 7. Acceptance gates S080 and reporting requirements

| Gate | Mandatory evidence |
| --- | --- |
| `S080-01` | Exact preflight SHA/tree/blobs, proper Owner-installed ACTIVE/handoff, single checkout/remote/worktree, four WIP files, clean unknown-state inventory, accepted branch unmoved. |
| `S080-02` | ACTIVE-only issuance commit pushed; exact remote readback; successful both-host original CI at issuance SHA. |
| `S080-03` | Exactly five path-scoped promotions; staged and remote Git blobs equal all five accepted values, original source branch preserved; no whole branch merge/cherry-pick. |
| `S080-04` | Four Linux-gated/native justified `unsafe` expressions, fixed compile-time source/limit profile, safe façade/CLI code unchanged from accepted branch; no dependency, graph, Charter or workflow change. |
| `S080-05` | Parent/child and strict negative tests actually run on Ubuntu and Windows as applicable; all ten G080 proofs observed in the mainline CI logs at integration SHA. |
| `S080-06` | Integration one-commit five-path ledger; normal push-triggered full CI successful on both hosts at exact integration SHA, including all applicable security and regression steps. |
| `S080-07` | Canonical final report is its own one-path commit, remote blob/tree/parent verified; final report-head original CI successful on both hosts. |
| `S080-08` | No change to the five retained historical/technical branches, no additional worktrees/clones/remotes, clean main, no unknown or Owner data erased; task-owned temp cleanup and inherited scratch inventory evidenced. |
| `S080-09` | Outcome only `LINUX-PREEXEC-LIMIT-SMOKE-INTEGRATED` / `BLOCKED` / `UNVERIFIED`; Linux proof limited to one harmless trusted child; `R03.C3`, `R03.C`, Stage 03 and all unimplemented safety/media gates OPEN. |

Overwrite the **one** canonical `.documents/Reports/execution-report.md` with: full exact starting report and ACTIVE values, issuance/integration/report-parent SHA/tree/blob/remote-readback ledger, five accepted source blob identities on branch and `main`, 3M/2A integration numstat, all other bytes immutable, actual Linux limit/group/wall evidence, direct unsupervised refusal, exact negative-test and Windows results, cargo graph and security proof, all CI run/job IDs and failed steps/corrections (if any), real toolchain limitations, unchanged `target/`, ignored scratch state and cleanup, branch preservation, CI at report HEAD, a truthful outcome, and a clear list of remaining containment/media risks. Do **not** embed the report commit's future own SHA inside the report; return it in chat. Report `S080` and `G080` **PASS / FAIL / NOT RUN** honestly. If any mandatory gate cannot be proven, do not assert the success vocabulary.

## 8. Repository and Owner-data safety / STOP conditions

Protect the sole Owner checkout, single origin, exactly four `.documents/WIP/` files, all previously accepted branches, and all unmodified materials. `.temp/` is ignored, never stage or force-add it. Preserve the inherited `.temp/solo-wo-073-wip/` four files / 46,062 bytes as **untrusted reference only**, until a later **separately authorized** native-containment task supersedes them; do not copy, execute, delete or quietly reinterpret them. The absence of earlier Owner archive files is not a cleanup task; do not recreate them. Track and remove **only positively identified disposable material created by this run** (including task-owned Cline temporary output where accessible) without wholesale temp deletion. Leave `target/` and all unrelated files intact. Avoid `git reset`, `git clean`, stash, force-push, destructive checkout, hidden branch revision or unknown-file deletion.

**STOP immediately** for a wrong root/branch/ref/blob, missing matching handoff, missing or drifted technical-branch source, any unknown/Owner dirty material at risk, unexpected staged or changed permanent path, any of the five files differing from the accepted blob, untrusted child execution, unauthorized native syscall/code/test mutation, weakened checker/CI/security gate, added dependency/version, failed/unverified integration or report-head CI, unsafe cleanup, secret exposure, inability to confirm exact remote SHA/parent/tree, non-fast-forward push, uncontained child/hung process or any unintended publishing/release action. Do not circumvent a stop by reducing the required test count, switching workflows, asserting a skipped gate passed or moving code to another branch. Publish a factual `BLOCKED` or `UNVERIFIED` report from `main` when safe and return the evidence. No self-ACCEPT, no merge, no release, no deploy and no next work order before independent Chief Architect disposition and separate Owner authorization.
