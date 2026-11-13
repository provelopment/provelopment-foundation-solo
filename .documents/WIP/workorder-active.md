# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-087 R01**  
**Title:** R03.C3 — prove independent read-only Linux live-child process-group and session identity from the owned parent and trusted child (isolated branch only)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner → Implementation Agent activation:** `SOLO-WO-087 R01`  
**Authority:** This entire work order becomes ACTIVE only after the Owner installs this full file at `.documents/WIP/workorder-active.md` and separately sends the matching `SOLO-WO-087 R01` activation prompt. The Chief Master Architect is **permanently read-only** on GitHub; only the Owner-activated Implementation Agent may make expressly authorized Solo Git changes. This order is not a standing permission to start other work, promote a branch, release or publish media.

## 1. Owner decision, objective and hard boundaries

The Chief Master Architect independently **ACCEPTED** `SOLO-WO-086 R01` as **`LINUX-RLIMIT-CPU-KERNEL-TERMINATION-INTEGRATED`** at `main` report commit `b6b2506e592eb9475116d21f012df28880b8d3c5`, after auditing two accepted blobs on `main`, three commit ledgers, and two-host CI at issuance, integration and report HEAD. The accepted Linux run showed one trusted, descendant-free, CPU-burning child reaped after kernel `SIGKILL=9` with `parent_killed=0` under its fixed 10-second CPU ceiling. That result, and the earlier N076/N079/N081/N083 proofs, remain limited; there is **no descendant cleanup, process-group termination or containment authority**.

**One new, deliberately read-only proof:** demonstrate that the pre-existing fixed `setsid` pre-exec installer places a trusted, descendant-free Linux child into a process group **and** session whose IDs equal its exact positive PID; both the child's own observations and a **separate parent kernel observation of its still-live, directly owned `Child` handle** must independently agree. The parent must prove it observed a live child before normal termination, validate one fixed readiness line, require normal zero exit and a reaped leader, and issue one bounded, explicitly *attestation-only* evidence line. The test child performs no media work, allocates no large memory, burns no CPU and launches no descendants.

**Why this is the next gate:** `pgid == pid` read solely inside the child, or merely calling `setsid`, is not enough to demonstrate an independent parent view. A process-group **signal** or descendant sweep must not be introduced until separately authorized, with a reviewed stronger lifetime/ownership plan. This order establishes read-only identity observations, **not** a race-free persistent group handle or permission to send a negative-PID signal. The kernel's identifiers can be reused later; checking a live child twice does not eliminate process-reuse races after those checks.

**Permitted success vocabulary:** `LINUX-LIVE-CHILD-SESSION-ATTESTATION-PROVEN`, on the isolated `solo-wo-087-r01` branch only, after all requirements and CI gates pass. It is **not** `LINUX-GROUP-KILL-PROVEN`, `DESCENDANT-CLEANUP-PROVEN`, `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED` or Stage 03 complete. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; all media correctness and publication flags remain `false`.

**Strict negative limits:** No `killpg`, `kill(-pid, ...)`, `pidfd_send_signal`, signal sent to a group, descendant fork/spawn, process-tree sweep, arbitrary PID or executable input, untrusted worker, plugin, external program, AVIF byte parser, decoder, release, publication, Windows Job Object, Windows process control, network use, filesystem write or credentials. Existing `Child::kill` may be used **only** as bounded cleanup of the directly owned leader on a refusal/timeout, never as successful identity evidence. Do not change installed pre-exec `setsid`/`setrlimit` semantics, limits, any N076/N079/N081/N083/N085 constant or assertion, public CLI behavior, architecture graph or CI workflow.

## 2. Exact, blocking preflight

Work only inside the one existing Owner checkout. Verify, from real Git, that `HEAD == main == origin/main` equals the exact baseline; worktree/index are clean except solely the newly Owner-installed ACTIVE replacement; exactly one remote `origin` with URL `https://github.com/provelopment/provelopment-foundation-solo.git` for fetch and push; exactly one worktree; no unknown tracked, staged or untracked file; exactly four `.documents/WIP/` files (`workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`). Record remote branch list and refs before any modification. Confirm `solo-wo-087-r01` does not already exist on either local or remote; if it does, STOP, never reuse, overwrite or move it.

| Pinned anchor | Required Git identity |
| --- | --- |
| Baseline `HEAD == main == origin/main` | `b6b2506e592eb9475116d21f012df28880b8d3c5` |
| Baseline tree | `c7844e86db8556d7ecce4f0bcda6c1def14f1d57` |
| Canonical report `.documents/Reports/execution-report.md` | `22b703c635085293f41e525a9bd54152c1af04d0` |
| Previous `.documents/WIP/workorder-active.md` | `381c4e6061488cd7af1586aa2c425d589cc41d74` |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` (both original `edges=41` checks) |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| **Native module to be extended** `crates/solo-adapters/src/media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` |
| **Safe façade to be extended** `crates/solo-adapters/src/media_worker_host.rs` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` |
| **Integration test to be extended** `crates/solo-cli/tests/media_worker_host_smoke.rs` | `038c3193e96633f34e0d34baffab7d04b348573d` |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` |
| Unsafe boundary guard `crates/solo-cli/tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| Architecture graph `crates/solo-cli/tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| Workspace graph `crates/solo-cli/tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |

Existing historical/proof branches, all immutable: `solo-wo-085-r01` `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`; `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b`; `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`. Preserve all locally and on `origin`, including their full history. No force push, branch deletion or rebase.

Inventory ignored inherited scratch **without using or modifying it**. Preserve `.temp/solo-wo-073-wip/` (historical four uncompiled files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, the existing `target/`, and all unrelated or older Cline temp logs. Only current-run-owned scratch may be removed by exact known path. A clean Git status does not prove cleanup.

**STOP on any SHA, path, ref, worktree, remote, ownership or WIP mismatch.** Do not improvise a repair or silently reset/clean/stash. Verify this Owner-installed ACTIVE file and matching activation ID before any write.

## 3. ACTIVE-only issuance on `main`; compulsory two-host CI

After a clean preflight, stage and commit **only** `.documents/WIP/workorder-active.md` with the entire Owner-installed `SOLO-WO-087 R01` document unchanged. One parent (baseline), one changed path, no other source, report or WIP plan changes. Push to `origin/main` by ordinary fast-forward. Verify exact SHA, tree, parent, changed-path ledger and new ACTIVE Git blob from GitHub. Require a new push-triggered `rust-verify` run at exactly the issuance SHA with `verify (ubuntu-24.04)` and `verify (windows-2025)` both `success`, 33 configured steps each, zero failures, only the host-inapplicable steps skipped (historically nine Ubuntu and fourteen Windows). Record run and job IDs. No implementation branch or technical edit until the issuance gate passes. Recheck `HEAD == main == origin/main`, clean status and unchanged historic refs.

If issuance CI fails or its actual head SHA differs, STOP and publish the truthful canonical failure report (report-only where safe) instead of proceeding to technical work.

## 4. Sole authorized technical branch and three-file permanent ledger

Create **one** technical branch `solo-wo-087-r01` at exactly the green issuance SHA, using the existing checkout only; never add a second worktree or clone. Implementation changes are allowed **only** on that branch and **only** in these three already-existing files:

1. `crates/solo-adapters/src/media_worker_host_native.rs` — minimal Linux-gated **read-only** process-group and session query; no state mutation; at most two necessary new native `unsafe` call expressions, each adjacent to a full `// SAFETY:` rationale, inside the existing positive Linux gate. No new module, allowance, crate, dependency, Cargo feature or guard bypass.
2. `crates/solo-adapters/src/media_worker_host.rs` — safe fixed parent/child internal-mode orchestration, typed refusal, group/session observations, exact bounded parser and evidence. No `unsafe`, no `libc` or new OS root, no arbitrary command/PID/session input.
3. `crates/solo-cli/tests/media_worker_host_smoke.rs` — bounded real-binary Linux positive/negative, pure validator and Windows fail-closed cases without weakening existing tests.

All other tracked implementation, workflow, Cargo, architecture, guard, documentation and WIP paths are **immutable**. On `main` the only permanent changes for this WO are the issued ACTIVE file and later the canonical report. The three technical files must **never** be merged, cherry-picked or path-promoted to `main` in this order.

The native module's existing four Linux `unsafe` expressions, `setsid` and two fixed `setrlimit` calls must remain intact; new source may add **no more than two** justified `unsafe` expressions for read-only kernel queries. The existing guard already permits the precise native path when Linux-gated and justified. If the unchanged guard cannot verify the implementation, STOP or REVISE within this authorized three-file scope; do not edit the guard or relax it. Preserve the same workspace graph `packages=4 targets=33 files=60 edges=41`.

## 5. Implementation contract — independent, read-only identity attestation

### 5.1 Linux native observation, no signals

Extend the existing positive `#[cfg(target_os = "linux")]` native module with one closed, typed read-only function or equivalent minimal safe wrapper for the kernel **`getpgid` and `getsid`** observations of (a) the calling process (`pid=0` semantics) and (b) a **single positive PID obtained only from this parent's newly spawned, directly owned `Child::id()`**. Validate positive group/session IDs, checked numeric conversion to `pid_t` (refuse zero, negative, nonrepresentable external child PIDs), and success statuses **before** conversion of results. On native failure report one fixed typed refusal with no raw errno, path, process metadata or OS error text. The `pid=0` self-query is only for the fixed trusted child; the nonzero query comes only from the parent's child handle, never from CLI, an authored file, network or environment.

The only native operations added are identity **observations**. No `kill`, `killpg`, `pidfd`, `waitpid`, `fork`, `exec`, signal handler or mutable process-group/session control is authorized. Continue to install the unchanged `setsid`/512 MiB `RLIMIT_AS`/10-second `RLIMIT_CPU` profile with the existing pre-exec installer; the new observation must not execute inside the `pre_exec` closure.

### 5.2 Exactly two new closed internal mode tokens

Add `session-parent` and `session-child` behind the already-reserved exact `--internal-media-host-probe-v1` flag, with no arguments beyond the single literal mode. No alternative spelling, enum-to-public-CLI exposure, user-provided executable or PID, environment-dependent workload or generic runner. Reuse the existing internal report dispatch through the **unchanged** CLI root and adapter root. Unknown, extra, duplicated or misplaced tokens must refuse exactly as existing closed-mode semantics require. All existing parent/child, wall, AS and CPU modes remain unchanged.

### 5.3 Fixed trusted child

The fixed `session-child` is launched exclusively by this same executable's `session-parent`, with closed stdin and captured bounded stdout/stderr, through the **unchanged** Linux pre-exec installer. Before producing any output or sleeping, it must verify the four kernel ceiling halves (`as_soft=536870912`, `as_hard=536870912`, `cpu_soft=10`, `cpu_hard=10`) and require its own positive `pid == pgid == sid` from the kernel. A direct unsupervised call must refuse promptly, with no stdout/stderr, before sleeping. A mismatch, failed readback, formatting failure or other defect is a fixed nonzero refusal; it cannot produce a readiness line.

Upon success, write and flush exactly **one short, fixed readiness line** with a new unambiguous `SOLO-MEDIA-HOST-SESSION-V1` marker and the four ceilings plus `pid`, `pgid`, `sid`; then sleep **exactly 4,000 ms** of wall time without CPU burn, allocation stress, file/network access, forks, threads or descendants, and exit normally with status zero. Do not lengthen or alter any old child sleep, loop, limit or deadline. This four-second sleep creates a finite window for an independent live parent kernel query and is not a timeout/CPU-limit negative test.

### 5.4 Parent's truly live, independently observed child

The fixed `session-parent` spawns exactly one trusted `session-child` through `std::env::current_exe` and the unchanged Linux pre-exec installer. It must obtain the **exact positive PID from the `Child` handle** and then verify `try_wait() == Ok(None)` both **immediately before and immediately after** the read-only native `getpgid(child_pid)` and `getsid(child_pid)` observations. Require both readbacks positive and `observed_pgid == observed_sid == child_pid`. A child that exited before/during the observation, any mismatch or any kernel query failure is a refusal, never proof. This evidence is an observation of a live child **at this check**, not a persistent identity pin or protection from PID reuse after the observation.

Use a **15,000 ms parent wall deadline** measured from before spawn and a total integration-test deadline of **20,000 ms** (the existing ordinary case budget). Capture the child's output within the established **1,024-byte stdout / 512-byte stderr caps**; do not print or pass through untrusted child bytes on refusal. After the live kernel observation, wait for the fixed child to exit **normally with code zero**, reap the owned leader, then strictly validate the one flushed child line, full-width limits, exact positive PID against the original handle, `pgid == sid == pid`, and equality to the parent-observed group and session. Reject missing, partial, duplicated, reordered, over-cap, non-UTF-8, extra-line or malformed fields; wrong values; un-reaped or signalled/abnormal exit; or wall time >= deadline. Only then emit exactly one bounded parent line with a distinct `media_host_session=live_attested` marker and full-width `as_soft`, `as_hard`, `cpu_soft`, `cpu_hard`, `pid`, `pgid`, `sid`, `observed_pgid`, `observed_sid`, `reaped=1`, `wall_ms` fields, in a fixed order.

On failure or timeout, fail closed with one existing or narrowly added **typed fixed refusal**, no success line. If a live owned leader requires cleanup, signal **only the `Child` handle** and reap it; never signal its group, descendant or unrelated PID. Ensure finite reader handling under the deliberately descendant-free fixed child; joining a pipe reader after the leader exits does **not** prove general descendant pipe-inheritance safety. Do not claim a reliable process-tree cleanup primitive or an immutable/reuse-proof session identity. A successful run must terminate via the child's normal exit, **not** via the parent's kill.

### 5.5 Windows and every non-Linux target

Both new modes fail closed with internal status `3`, exact stable diagnostic `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n`, empty stdout, and no created child, session API, sleep or native syscall. Neutral parsing, classification and validation must compile and be exercised on Windows. Neither Linux native module nor Linux-only real tests may run there; previous public CLI bytes and statuses stay identical.

## 6. Required direct proof matrix, refusal cases and gates

Use narrowly named `N087-*` real-binary and pure cases in the existing integration-test file and safe façade unit-test module. Keep all old assertions intact; do not rename, disable, suppress or weaken N076/N079/N081/N083/N085 tests. New tests must not perform an unbounded read, hang or spawn descendants.

- **N087-01 (Ubuntu, real binary):** call only `--internal-media-host-probe-v1 session-parent`; assert normal success status 0, exactly one bounded evidence line, empty stderr, real parent observation `observed_pgid == observed_sid == pid > 0`, child `pgid == sid == pid`, four exact limits, successful normal leader reap, `0 < wall_ms < 15000`, no group kill and no forbidden marker. Emit one labelled `n087_session=` line straight to the runner's captured diagnostic output for independent log review. Require the child was alive before **and** after the parent's native group/session observations. Do not assert an exact wall-time equality to four seconds; report the measured value.
- **N087-02 (Ubuntu, real binary):** direct `session-child` invoked without the pre-exec installer refuses **before its four-second sleep**, promptly (well inside the ordinary 20,000 ms outer budget), with nonzero fixed exit and empty stdout/stderr. Emit `n087_direct_session_child=exit=<code>`; assert the documented existing fixed refusal class, not any success.
- **N087-03 (both hosts, pure + real binary):** new token classifier is closed/additive; refuse wrong, extra, repeated, malformed and misplaced modes; exact readiness line is required and rejects negative/zero/overflow, wrong `sid`, `pgid`, `pid`, ceilings, reordered/extra/duplicated fields and CRLF; require parent-observed equality and the `live-before/live-after` flags in the neutral validator; require normal zero exit and leader reap; refuse `parent_killed` or timeout, malformed/over-cap output, and any unsupported session/group mismatch. A unit test must prove a forged identity line alone is never accepted in absence of real parent observation. Evidence wording must never be `media_host_probe=accepted`, `media_host_cpu=kernel_terminated` or `media_host_as=limit_enforced`.
- **N087-04 (Windows, real binary):** both `session-parent` and `session-child` return status 3 with exact `HOST_UNSUPPORTED` and empty stdout before any child exists, promptly. The Windows job must include no Linux native N087 real observation and no Linux-only N076/N079/N081/N083/N085 real positive cases. Run all pure refusals and token-shape checks on Windows.
- **N087-05 (both hosts, regression):** all applicable older N076/N079/N081/N083/N085 tests remain green, including Linux kernel `RLIMIT_CPU` signal with `parent_killed=0`, `RLIMIT_AS` capacity refusal, wall leader-kill/reap, direct unsupervised negatives and unchanged public CLI bytes/exits. Enforce `architecture=legal packages=4 targets=33 files=60 edges=41`, `self-check=passed cases=29`, original negative architecture/security tripwires, strict clippy, formatting, locked all-target tests, cargo-deny advisories/bans/licenses/sources, release smoke, Ubuntu forbidden-import restoration and reproducible clean release builds.

Do not modify `.github/workflows/rust-verify.yml` to accommodate a new test or skip, and do not relax architecture, lints, feature flags or security policy. Use the existing workflow dispatch for branch validation.

## 7. Branch CI, evidence exactness and revisions

After bounded changes to only the three authorized technical files, inspect `git diff --name-status`, numstat and `git diff --check`, verify unchanged protected blobs, and commit/push on only `solo-wo-087-r01` by ordinary fast-forward. Run the complete, unchanged `rust-verify` via `workflow_dispatch` **at the exact technical-branch commit SHA**, not a previous head. Require both `verify (ubuntu-24.04)` and `verify (windows-2025)` conclusions `success` with 33 configured steps, zero failures and only host-inapplicable skips. Record exact SHA, tree, parent, changed paths, Git blob identities, run ID, both job IDs, logs and observed signal/readback/test lines.

If CI fails, disclose **every** red run and actual failure. Correction is permitted solely inside the same three authorized technical files, with a new bounded commit on the same branch and full CI at the exact new tip; do not suppress tests or loosen controls. A failure that needs a workflow, guard, Cargo, root, ABI, new dependency, extra file, broader native signal API or architectural change is **STOP/REVISE**, not an implicit scope expansion. No `main` implementation commit, PR, merge, cherry-pick, path promotion or branch-rewrite is authorized.

If a new native unsafe expression lacks a valid positive Linux AST gate and adjacent `// SAFETY:` rationale, or if source/guards/architecture show an extra capability, STOP immediately. A refusal to send a group signal is a **correct** result of this order's scope, not a missing feature to sneak into the implementation.

## 8. Canonical report-only main commit and final both-host CI

After the technical branch and its exact-HEAD CI are recorded, return to clean `main` at the issuance SHA. Overwrite **only** `.documents/Reports/execution-report.md`, the existing single canonical report; create no report archive, second WIP file or report directory. Publish one report-only `main` commit, with precisely one parent (issuance) and precisely one changed path (canonical report). Push by ordinary fast-forward. Require another fresh complete `rust-verify` **push** run at the exact final report commit SHA on Ubuntu and Windows, both green, 33 configured steps each. This mainline run will not contain the branch-only N087 source and therefore must **not** be cited as N087 functional proof; its job is mainline nonregression and report-head integrity.

Record in the final handoff (the report cannot know its own SHA before commit): issuance SHA/tree/ACTIVE blob/CI; branch commit chain and 3-file ledger; final branch SHA/tree/blobs and direct live Linux kernel group/session observations; one exact two-host branch run and its jobs/logs; report-only main SHA/tree/report blob/CI; historic branch refs; protected anchors; clean working tree and actual task-owned cleanup. Declare all unverified local or runtime facts as unverified. Never imply CI tested N087 on `main` before a separately authorized integration order.

If the branch is incomplete, CI red, or a mandatory observation fails, publish a truthful blocked or failed report where safe, with **no false PASS**. Do not omit the canonical report merely because the implementation did not succeed. Do not self-ACCEPT.

## 9. Cleanup, preservation, separation and STOP conditions

Preserve the sole checkout, exactly four WIP files, `target/`, all historical and proof branches, Owner data, and inherited ignored `.temp/solo-wo-073-wip/` (4 files / 46,062 bytes) without compiling, staging or using it as source. The historical Cline `proceed-while-running-*.log` files may belong to other sessions; do **not** delete any such unknown logs. Current-run-owned scratch may be created only in approved Solo ignored scratch or the permitted Cline task temp area; inventory and remove only files definitely owned by this run, without broad deletion of `.temp/` or shared temp roots. Identify remaining current-run debris explicitly; clean Git status does not prove cleanup. Record exact current-run artifact paths/categories and deletion result without disclosing secrets.

No second checkout/worktree, no destructive `git clean`, reset, stash, rewrite, force push, hidden patch or other repository, no release or deploy. The Implementation Agent's permission stops after reporting. Chief Architect has **READ-ONLY** GitHub access and alone independently reviews final exact evidence for **ACCEPT / REVISE / STOP**; the agent must not self-accept or start `SOLO-WO-088`.

**STOP if:** the Owner handoff or baseline mismatches; a new branch name already exists; issuance CI is not fully green; the three-path ledger is exceeded; unreviewed `unsafe` or FFI enters; any native signal/process-control mutation beyond existing leader `Child::kill` is introduced; a descendant or arbitrary executable/PID enters; live parent attestation cannot be directly proved; child cannot be normally reaped; Windows starts a child; mandatory test/security/architecture CI cannot be satisfied within scope; report-head CI is missing/red; any branch or Owner scratch is at risk. Preserve evidence, publish a truthful report where safe, and return to the Owner without unapproved corrections.
