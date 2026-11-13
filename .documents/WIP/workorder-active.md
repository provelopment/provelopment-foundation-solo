# SOLO-WO-093 R01 — R03.C3 FIXED LINUX PROCESS-GROUP AND DESCENDANT LIFECYCLE PROOF

**Authority:** `OWNER-ISSUED — SOLO-WO-093 R01`
**Matching Owner activation:** `SOLO-WO-093 R01`
**Required outcome:** `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-PROVEN`
**Repository:** `provelopment/provelopment-foundation-solo`
**Sole local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`
**Implementation branch:** `solo-wo-093-r01`, created at this order's issuance commit.
**Reporting branch:** existing `main`, for issuance and the sole execution report.

## 1. Execute the exact approved scope

Implement and verify the single bounded `R03.C3` candidate selected by accepted `SOLO-WO-092 R01`: a **private, fixed Linux process-group/descendant lifecycle proof** using exactly one directly owned group leader and one fixed descendant. Demonstrate that the parent validates both real kernel identities while the leader is alive, sends **one** group-directed termination only to the verified group it created, reaps its own leader, and independently observes that the test descendant is no longer running. Prove a real Linux negative path that refuses an invalid/incomplete readiness event **without issuing any group signal**. Refuse every uncertain state. Preserve every accepted earlier probe.

This is a **test harness**, not an AVIF decoder, generalized supervisor, public CLI, production-safe group isolation or published media capability. Leave `R03.C3`, `R03.C` and Stage 03 **OPEN**. Do not merge the technical branch into `main`; independent acceptance and a later Owner-issued integration order govern any promotion.

Read and obey `AGENTS.md`, `.documents/SOP/sop-architect.md` §§6, 6A, 6B, 7B and 8, `.documents/SOP/sop-implementation.md` §§4–11, and the binding `.documents/Architecture/ARCHITECTURE.md` §§6.2G, 6.2I, 6A.8. The earlier N076/N079/N081/N083/N085/N087/N089/N090 evidence remains binding; do not weaken it.

## 2. Exact preflight and preserved Git objects

**Before writing, staging, creating scratch or changing branches**, establish the real repository root and verify these exact Git identities against the local object database, `origin/main`, GitHub, and `git ls-remote`:

| Item | Exact required Git identity |
| --- | --- |
| Starting `HEAD == main == origin/main` | `a4f62b057916dc592f4fa59b8fdf8aca17315857` |
| Starting tree | `6db880ed2a09e59d8b13a3449681239aeaf205b7` |
| Starting sole parent | `01f9cac113d1fc23a1080380d517cef261eacb6f` |
| ACTIVE neutral blob | `2ef2db63df17cdb00513f47beca1bc199d381e06` |
| NEXT candidate blob | `1090ff418b669dfb70ce8dda0881cd2b837f8008` |
| ROADMAP blob | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| OUTLOOK blob | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Canonical report blob | `86808db9193a54278a66a10eefd1d4fea02503e1` |
| Architecture blob | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| Native host seam blob | `1e698245493f6d10ebb9636282698a797a242865` |
| Safe host façade blob | `a1f2bdb9960a4befc44efab746640e4d19bb624b` |
| Host smoke-test blob | `5444013571a71efde758422fdf9d8fada79ef53a` |
| `Cargo.lock` blob | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `rust-toolchain.toml` blob | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| CI workflow blob | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| Architecture checker entry blob | `0bad0e027f41b6cf6029df38174fc15531c187da` |

Verify the issued ACTIVE is the **only** unstaged dirty path (` M .documents/WIP/workorder-active.md`), its exact Owner-installed contents carry this ID and matching activation, and no other untracked/staged content exists. Require **one** checkout/worktree, **one** remote `origin` (both URLs `https://github.com/provelopment/provelopment-foundation-solo.git`), `main` checked out, no stash, and **exactly four** WIP files: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`. The prior historical refs, including `solo-wo-089-r01` at `1236915cab6e87b17ce84813430897b273fc3230` and `solo-wo-090-r01` at `0eb3fcf645851ae752bd59e12fe32b69604197d2`, must be preserved. Verify branch `solo-wo-093-r01` does not already exist locally or remotely.

Inspect but do not alter inherited ignored files, particularly `.temp/solo-wo-073-wip/` (previously 4 files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/`, Cline's prior logs and other unknown Owner materials. A clean `git status` does not prove their preservation. A mismatched baseline, extra worktree/remote/WIP file, unexpected dirty material or missing Owner handoff is **STOP**.

## 3. Sole allowed paths and permissions

**Permanent changes are limited to exactly these five paths:**

1. `.documents/WIP/workorder-active.md` — only the Owner-installed issued content at issuance; do not rewrite or annotate it.
2. `.documents/WIP/plan-next.md` — remove the issued candidate, retaining only the exact original heading `# Foundation Solo — plan-next (Next Candidate)` plus one LF.
3. `crates/solo-adapters/src/media_worker_host_native.rs` — Linux-gated, narrowly checked native group signal and kernel observations.
4. `crates/solo-adapters/src/media_worker_host.rs` — private fixed modes, typed observations/refusals, bounded parent/child supervision and internal dispatch.
5. `crates/solo-cli/tests/media_worker_host_smoke.rs` — actual Linux positive/negative integration tests and Windows fail-closed tests.

The **one additional reporting path** is `.documents/Reports/execution-report.md`, overwritten only after branch verification, then published from `main`.

Every other file is **read-only**, including ROADMAP, OUTLOOK, About, Architecture, SOPs, `README.md`, `AGENTS.md`, `crates/solo-cli/src/main.rs`, architecture checker and source guards, every other crate/file, workflows, manifests, `Cargo.lock`, deny configuration, toolchain, licence, notices and security configuration. No new permanent file, fixture, dependency, `unsafe` allowance, workflow, branch other than `solo-wo-093-r01`, workspace, report, package, media file or alternate public entry point is authorized.

**Git authority:** three distinct actions only: (A) ordinary issuance commit and fast-forward push on `main`, (B) exactly one source/test implementation commit and ordinary push on the new `solo-wo-093-r01` branch, (C) one canonical report-only commit and fast-forward push on `main` after B CI. Do not self-merge, open a PR, cherry-pick, rebase, amend, force-push or change existing historical refs. No release, deployment, network provider operation, secrets access, tooling installation, dependency upgrade or destructive Git operation is authorized. Existing approved CI may use its configured network dependency/security acquisition and existing toolchain steps. Task-owned disposable scratch is allowed only under Solo `.temp/` or the actual Cline-provided task temporary area and must be cleaned by known path.

## 4. Commit A — record issuance and clear NEXT

At the pinned starting HEAD, stage the **unaltered** Owner-installed ACTIVE. In the same issuance transition replace `.documents/WIP/plan-next.md` with exactly this content, UTF-8 LF and one terminal newline:

```markdown
# Foundation Solo — plan-next (Next Candidate)
```

Stage **only** ACTIVE and NEXT, require exactly two modified paths and no extra ledger entry. Compare the staged ACTIVE Git blob with the installed file after normal declared Git filters; never trim, normalize or rewrite issued content merely to silence a formatting warning. Require the NEXT blob to match its earlier neutral Git object `92fa562b1af0c54391851aa5c79deb1631127069`. Run and record `git diff --cached --check`; any finding must be attributed to its exact line and resolved without changing Owner-issued text, or **STOP** if it represents an actual unsafe change.

Create one normal mainline issuance commit with sole parent `a4f62b057916dc592f4fa59b8fdf8aca17315857` and message `Issue SOLO-WO-093 R01 and clear R03.C3 NEXT candidate`; push `origin main` without force. Verify the new commit, sole parent, tree, two-path ledger, and blobs from GitHub. Require its **own** fresh `push`-triggered `rust-verify` run at that exact SHA on `main`, Ubuntu and Windows both successful. **Do not create the branch or edit technical files until this CI gate is green.**

## 5. Commit B — one bounded technical implementation branch

Create `solo-wo-093-r01` **from Commit A**, inside the same verified checkout, and change **only** the three authorized technical paths. Preserve existing public CLI dispatch in `crates/solo-cli/src/main.rs` byte-for-byte. Reuse `run_internal_media_host_probe_v1`, `classify_internal_arguments_v1`, the fixed `INTERNAL_PROBE_FLAG_V1` and `InternalProbeReportV1`; introduce only closed additive private mode tokens in `media_worker_host.rs`. Name the new evidence marker `media_host_group=terminated` and keep all N079–N090 markers and output schemas unchanged.

### 5.1 Fixed positive harness — required exact behavior

Implement one internal `group-parent` mode that re-executes **this same** executable into one `group-leader` mode, using the existing `install_linux_child_limits_v1(&mut command)` before `spawn`: `setsid`, `RLIMIT_AS` soft/hard 536,870,912 bytes and `RLIMIT_CPU` soft/hard 10 seconds. The leader validates its own limits and `pid == pgid == sid` before work. It starts **exactly one** fixed `group-descendant` of the same executable without creating another session; the descendant inherits that group and both limits, validates them before sleeping, and has a finite fixed wall lifetime (8,000 ms maximum). No grandchild of the descendant, shell, arbitrary command, user path, user-provided PID, signal number, media input, command-line parameter, environment-configured limit or network operation is permitted.

Establish one bounded readiness handoff to the supervising parent **before any group signal**, with a single exact newline-terminated record containing the leader's real PID, the descendant's real PID, both observed group/session identities, and the four limit halves. The leader must obtain the descendant PID from its own `Child::id()` and verify that the descendant has actually emitted its fixed readiness line. The parent independently checks its **directly owned leader** through `Child::id()` and `try_wait()`, reads the kernel's leader and descendant identities, confirms `leader_pid == leader_pgid == leader_sid == descendant_pgid == descendant_sid`, confirms `descendant_pid != leader_pid`, verifies both were live while observed, and rejects any missing/overlong/duplicated/malformed/trailing readiness content. Apply the existing 1,024-byte stdout / 512-byte stderr caps and a single 15,000 ms parent deadline starting **before** leader spawn. Use bounded readers and do not create new deadlines after readiness.

Only then, from the existing positively Linux-gated native seam, send **exactly one** `SIGKILL` to that verified **owned group** and record whether the real syscall succeeded. Recheck leader liveness and group/session identities immediately before the call. Do not use a caller-provided negative PID, `kill(0, …)`, `kill(-1, …)`, a PID glob, process table sweep or generic arbitrary-PGID termination API. Do not treat syscall success as leader-reap or descendant-exit evidence. `Child::try_wait()` / checked `Child::wait()` must establish the **actual direct leader reap**; if a reap cannot be established, return `HOST_REAP_FAILED` and never join readers whose pipe may still be open.

Independently observe the descendant's real Linux **non-running** state after the group signal, using bounded kernel-backed evidence for the **same observed PID identity**: a verified `/proc/<pid>/stat` transition to zombie (`Z`) or disappearance after a previously observed live state is admissible for this fixed fixture. Preserve and compare the observed process start-time identity to avoid presenting a clearly reused PID as the same descendant. Implement the state parsing in the Linux-gated native adapter, with strict bounds and typed failure; no `kill(pid, 0)` result, `kill(-pgid, …)` success, leader wait, pipe EOF, log string or fixed sleep alone may count as descendant-termination proof. If `/proc` is unavailable, malformed, recycled or otherwise inconclusive, **refuse**. A zombie is **non-running, not parent-reaped**: the parent cannot claim it reaped that descendant. The leader's own `SIGKILL` wait status must also be checked; normal exit, a different signal or an ambiguous wait refuses.

The accepted positive line is emitted **only after** every required observation and cleanup, for example:

```text
media_host_group=terminated leader_pid=4712 descendant_pid=4715 pgid=4712 sid=4712 signal=9 leader_reaped=1 descendant_nonrunning=1 wall_ms=731
```

Numbers above are illustrative field shape **only**, never expected live PIDs or timing. Require a closed exact parser/validator with duplicate, missing, extra, forged-identity, wrong-limit, wrong-signal, missing-EOF and out-of-budget negatives. The parent controls all evidence fields; child text cannot authorize a signal or become acceptance by itself. No partial evidence line may be printed on failure.

### 5.2 Native signal safety — use this specific implementation boundary

In `media_worker_host_native.rs`, put **all** new native code and every native import, `libc::kill`, `getpgid`/`getsid`, and `/proc`-specific observation under an explicit `#[cfg(target_os = "linux")]` boundary. Every new `unsafe` block requires its adjacent concrete `// SAFETY:` rationale. **Keep `media_worker_host.rs` and tests free of `unsafe`, raw syscalls and native bindings.** Do not edit or weaken an architecture checker or its fixtures.

The native termination function must take the **actual `&mut std::process::Child` handle** owned by this parent, derive the leader PID *inside* the function, and take the one independently established descendant PID solely for identity validation. Do not expose a generic PGID/signaller accepting arbitrary group numbers. Its checked core must implement this shape:

```rust
let leader_pid = i32::try_from(owned_child.id()).map_err(|_| Refusal::BadIdentity)?;
if leader_pid <= 1 || descendant_pid == owned_child.id() || descendant_pid <= 1 {
    return Err(Refusal::BadIdentity);
}
if !matches!(owned_child.try_wait(), Ok(None)) {
    return Err(Refusal::LeaderNotLive);
}
// Query BOTH real identities through the gated native seam, and require:
// leader.pgid == leader.sid == leader_pid;
// descendant.pgid == descendant.sid == leader_pid;
// verified_leader_group != verified_current_process_group;
// Recheck owned_child.try_wait() == Ok(None) immediately before signalling.
let target = leader_pid.checked_neg().ok_or(Refusal::BadIdentity)?;
// SAFETY: SIGKILL targets only the just-observed positive, owned group leader;
// never PID 0, -1, a foreign group or a caller-selected group.
let rc = unsafe { libc::kill(target, libc::SIGKILL) };
if rc != 0 { return Err(Refusal::SignalRejected); }
// Return "signal was issued" only. No reap/descendant success is implied.
```

`Refusal` and observation types in the snippet are placeholders for concrete **closed**, reviewed private enum/type names; implement the checks in code, not in comments. Ensure `libc::pid_t` conversion is checked and the current process group is positively observed. The pre-signal `try_wait` bracket limits stale-identity risk, **but is not a race-free PID/PGID lifetime pin**; disclose that limit and never claim robust hostile-descendant containment or general process-tree safety. Do not send any signal if even one required identity is absent, invalid or contradictory.

### 5.3 Required real Linux negative and cleanup behavior

Add a **separate fixed invalid-readiness path** (one closed parent token and one fixed leader token, with **no descendant created**) using the same pre-exec limits. Make that leader emit one deliberately invalid readiness line, so the real parent refuses `HOST_OUTPUT_INVALID` **before** group identity authority or signalling. Prove in source structure and real test instrumentation that this negative path executes **zero** group-target `kill` calls; it may kill and reap only its own directly held `Child` leader. All negative cleanup results must be checked: a failed leader reap overrides the original refusal as `HOST_REAP_FAILED`. Never block on an unbounded pipe join or claim a negative run succeeded because a timeout occurred.

For the positive path, if any pre-signal condition fails, do not send a group signal. Reap the directly owned leader through checked bounded cleanup, keep the descendant's own life fixed/finite, and report refusal rather than guessing about the descendant. On a signal or observation error, no success is authorized even if the leader exits. Capture which cleanup facts were independently observed; do not conceal a potentially surviving descendant. **STOP execution and report** if a task-created process cannot be safely confirmed ended within the fixed outer budget or if cleanup would require signalling an unverified foreign group. Do not leave a long-running test descendant behind.

### 5.4 Cross-host and regression tests

In the existing `media_worker_host_smoke.rs`, add at least these tests with closed, stable `N093` identifiers:

- **N093-01, real Linux:** fixed positive parent; independently parsed evidence, observed and distinct PIDs, five matching group/session identities, `SIGKILL=9`, leader reaped, descendant observed non-running, `0 < wall_ms < 15,000`; print a concise **test annotation** `n093_group=...` (never change the actual internal protocol).
- **N093-02, real Linux:** invalid-readiness negative yields exactly one typed refusal, no accepted stdout and **zero group signals**, with verified leader cleanup; print `n093_refusal=...` as a test annotation, not a protocol field.
- **N093-03, real Linux:** direct unsupervised child/leader invocations and all wrong, duplicate, extra or misordered tokens refuse without launching grandchildren or emitting evidence.
- **N093-04, real Windows:** every new parent/leader/descendant/negative token returns `HOST_UNSUPPORTED` before spawn, kernel query, sleep or signal. Existing N087/N089/N090 Windows behavior stays unchanged.
- **N093-05, unit matrix on both hosts:** failed/missing wait, wrong leader/descendant identities, stale group, false signal success, unverifiable descendant state, abnormal signal, malformed readiness and reader failure all refuse; no synthetic mocked result is described as a real OS failure.
- **N093-06, regression:** N076–N090 tests and the public CLI behaviour pass unchanged; existing markers, limits, deadline values and test annotations retain their meaning.

Keep one bounded case execution under the smoke harness's existing 20,000 ms normal per-case limit. All output is fixed, capped and checked before printing. Require a real failure-sensitive negative: invert or bypass one new acceptance/refusal check in **task-owned disposable scratch only** to demonstrate that the intended test rejects it; restore the exact tracked tree before commit. Do not modify permanent guards or production code to make a red gate green.

### 5.5 Technical commit and exact-head gates

Before the technical commit, run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-targets --locked --no-fail-fast`, `cargo test --locked -p solo-cli --test check-architecture`, and the existing architecture negative self-test. Execute new real Linux cases on a real Linux host; **a Windows-only local machine or mocked syscalls cannot satisfy N093-01/N093-02**. CI on Ubuntu is the required independent real-kernel execution. No modification of source/checker/workflow/dependencies for CI convenience.

Stage **only** the three technical files, require three `M` paths and a clean `git diff --cached --check`. Create exactly one branch technical commit with sole parent Commit A, message `Prove bounded Linux group and descendant lifecycle in fixed harness`; push `origin solo-wo-093-r01` without force. Verify branch HEAD/tree/parent, all three Git blobs, no fourth changed path, and a fresh **branch-head** `push`-triggered `rust-verify` on **both** Ubuntu and Windows at this exact technical SHA, all mandatory steps successful and skips explained. Check actual Ubuntu logs contain **real** `n093_group=` and `n093_refusal=` lines with their corresponding integration tests; check Windows executes `N093-04` and contains **no** Linux-success evidence. Do not call a run on `main` proof of branch code. **Do not merge/cherry-pick the branch.**

## 6. Commit C — single canonical report from `main`

After branch CI passes, switch safely **in the same checkout** back to `main` at Commit A. Require no uncommitted technical edits, no stash and no change to branch history. Overwrite **only** `.documents/Reports/execution-report.md` with the current factual `SOLO-WO-093 R01` execution report, identifying status `COMPLETE` and **claimed** outcome `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-PROVEN`, submitted for independent Architect review and **not self-accepted**.

The report must include the complete preflight and branch ledger; issuance SHA/tree/parent/two-path stats and two-host CI; technical branch SHA/tree/parent/three-path stats and two-host CI; all existing 12 historical refs unchanged; exact code/test Git blobs; actual real Linux positive/negative output with pid, pgid, sid, elapsed and exit status; Windows unsupported cases; architecture graph, safety source guard, negative self-tests and regression; warnings, license notice, RustSec/OSV/security status and current-component-version assessment under SOP §7B(e1)/(e2); honest difference between kernel-observed exit and descendant **reaping**; remaining PID/PGID race, aggregate resource, sandbox, AVIF, Windows and publication limitations; `R03.C3`, `R03.C` and Stage 03 OPEN; task-owned scratch creation/cleanup inventory and preserved pre-existing ignored material. Use **only** one report, with no archive or parallel files. Never insert the report's own unknown future SHA into its content.

Commit **only** that report on `main`, with sole parent Commit A and message `Report SOLO-WO-093 R01 fixed Linux group proof for review`; ordinary fast-forward push to existing `origin/main`. Verify report SHA/tree/parent/blob from GitHub and require a separate fresh two-host `push` CI at the exact report HEAD. Confirm `HEAD == main == origin/main`, clean working tree, one checkout, one remote, preserved branch `solo-wo-093-r01` at its exact published technical SHA, no stash and all old branch refs unchanged. The code must **not** appear in `main` or the report commit.

## 7. Required acceptance evidence and hard STOPs

Record each required fact as `PASS`, `FAIL`, `UNVERIFIED` or reasoned `NOT APPLICABLE`. All mandatory applicable evidence must be `PASS`; no case may treat a CI badge as a substitute for actual branch-head Linux kernel logs. Report source provenance and the exact Unix/Linux host, Windows runner, `rustc 1.99.1`, locked dependencies and advisories; preserve warnings instead of silencing them. Specifically distinguish (a) a signal **request**, (b) leader **reap**, (c) descendant **non-running observation**, and (d) descendant **reaping**, which this order does not and cannot establish via the parent's `Child` handle. Do not claim PID reuse is solved by a pre-call liveness check.

**STOP immediately** if: the Owner-issued identity or any baseline/ref/blob differs; extra worktree/remote/stash/WIP path or dirty file exists; issuance CI is not green **before** code edits; an unauthorized fourth technical path is needed; a native operation escapes the positively Linux-gated seam or gains a new `unsafe` allowance; `kill(0, ...)`, `kill(-1, ...)`, arbitrary process/group signalling or process scan appears; a leader/descendant is not positively identified before signalling; a parent could be killed by its own group signal; a task child remains unconfirmed running; the negative uses a group signal; any real Linux proof is missing; Windows runs a purported Linux worker; one mandatory test/security check fails or is unverified; a dependency/toolchain/CI/Charter change is required; any push/branch ancestry is wrong; a secret or Owner-owned scratch file would be exposed/deleted; or the order cannot be fulfilled within the stated boundary.

On STOP, do not silently repair beyond this authority, rewrite commits, manufacture success or merge. Preserve source/history, use the standing report-only permission only where safe, and return the exact blocker and surviving processes/cleanup observations. Do not self-ACCEPT, close `R03.C3`, start media decoding, change NEXT again or release/deploy anything.

**Final handoff:** return the one exact GitHub report URL, the issuance/main-report and technical-branch full commit/tree/parent identities, per-commit file ledgers and blobs, the three distinct CI run IDs and six job IDs, real Linux N093 proof/refusal log lines, Windows unsupported results, cleanup, warnings and precise nonclaims. End for independent Chief Master Architect `ACCEPT / REVISE / STOP`.
