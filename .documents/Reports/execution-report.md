# Execution Report

**status=COMPLETE**

**Order:** `OWNER-ISSUED — SOLO-WO-094 R01` — CORRECT AND VERIFY THE ISOLATED LINUX GROUP LIFECYCLE PROOF. **Matching Owner activation:** `SOLO-WO-094 R01`. **Claimed outcome:** `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-VERIFIED` — **pending independent Chief Master Architect review**. The Implementation Agent does not self-accept.

**Why this order exists.** `SOLO-WO-093 R01` remains **STOPPED and NOT ACCEPTED**: its branch-head `push` CI gate could not run, because `.github/workflows/rust-verify.yml` limits `push` to `main`, and its technical commit `f85caf35a00b103a47cf83356e66df278ccca682` was **never compiled by anyone**. This run corrects that branch and proves it under the workflow's already-existing `workflow_dispatch` trigger, with the workflow **byte-identical** (`e1b5ca6e1738a5d5129a8788e24d51aee02421d1`). Nothing is merged; `main` carries documentation only.

## 1. Preflight — PASS

Sole checkout at the approved root, `main` checked out, one worktree, one remote whose fetch and push URLs are both `https://github.com/provelopment/provelopment-foundation-solo.git`, empty stash, exactly four WIP files, and the only dirty item the complete Owner-installed unstaged ` M .documents/WIP/workorder-active.md` (23,535 bytes, blob `178818bd779cb2299afb4ab52d6220a2268882f3`), staged later byte-for-byte unchanged with a clean `git diff --cached --check` (exit `0`, no hard-line-break findings).

All required starting identities matched exactly: `HEAD == main == origin/main == ls-remote origin main == 8acde13e3624437d5c0b8c88b1b625bb61bf1b91`; main tree `8a02f1e42511b356fc9cd7a6a167d21b3f97fcdb`; sole parent `14b65dbb79d7da908a961abb8db91d5b91ea55d2`; tracked ACTIVE `7a465788a5cad6457c3b27ad13b1423284df7785`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; report `657c09867fecbdc651280fd597d01b96b0290bf3`; Architecture `c941c7f90cc1b1955f9d60b348a3b1092cafe97f`; `Cargo.lock` `b1ee56da60f36536548586528db371b7cbe508e4`; `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`; workflow `e1b5ca6e…`; architecture-checker entry `0bad0e02…`; technical branch `solo-wo-093-r01` at `f85caf35…` with tree `8ef4c768236d21729661ba2cdcf6a2615e2592d9`, sole parent `14b65dbb…` and blobs `bd3cf9cc…` / `61cfd059…` / `0064d2a4…`. Thirteen remote refs were present and preserved; `solo-wo-094-negative-r01` did not exist locally or remotely.

## 2. Commit A — issued order on `main`

`8b643f7f9506f9602714fc62bd9f08d539851d15`, tree `8f3b0f7ab9cd3ed4b3f35123a6f2fea19a323c9e`, **sole parent** `8acde13e3624437d5c0b8c88b1b625bb61bf1b91`, exactly **one** `M` path (`.documents/WIP/workorder-active.md`, `+123/−107`), ACTIVE blob `178818bd779cb2299afb4ab52d6220a2268882f3` identical to the installed bytes, ordinary fast-forward push. GitHub readback matched. Its own `push`-triggered run **`43418395749`** is green at exactly that SHA: Ubuntu job `131861943865`, Windows job `131861944274`.

## 3. The three real defects this order corrected

The `SOLO-WO-093 R01` code had never been compiled. Verification found three defects, all inside the three authorized technical paths, each now fixed forward-only on the same branch:

1. **Linux compile blocker — missing native exports.** The façade imported `GroupSignalRefusalV1`, `ProcessStateV1`, `observe_process_state_v1` and `terminate_owned_group_v1` from `media_worker_host_native`, but that module's bottom `pub(crate) use linux::{…}` listed only four older entries, so none of the four resolved. The list now exports all eight required items; the definitions and every `unsafe` block remain inside the positively `#[cfg(target_os = "linux")]`-gated module, and the misleading "three entry points" comment now describes the actual fixed exports. No Windows counterpart and no broadened unsafe boundary was introduced.
2. **A failed leader reap could reach a reader join as success.** `probe_group_after_readiness` converted `child.wait()` into `(leader_reaped, leader_signal)` and still returned `Ok(GroupAttemptV1)`, after which the caller joined the capped readers unconditionally. Production now calls the new pure boundary `group_wait_signal_v1(child.wait())?`, whose every `Err` becomes `HOST_REAP_FAILED`, so a failed reap cannot reach a success attempt or a join; the `Err` path goes through the accepted `SessionReapV1`/`SessionCleanupPlanV1` rule, where only a **subsequently confirmed** `Child::wait()` permits joining.
3. **Unproved descendant cleanup joined behind an unconfirmed wait.** The group leader's own failure branch killed and waited on its descendant and joined regardless of the wait result. It now joins only when `descendant.wait()` is confirmed, so an unconfirmed wait cannot block that leader on an open pipe.

Two further correctness defects surfaced only in real CI and were corrected forward-only as well: a **guarded `matches!`** in the leader was unparseable by the source-boundary checker (`error: macro-unsupported … macro arguments are not parseable and cannot be inspected`), now expressed as an `is_some_and` closure; and three Linux-only timing constants were gated too widely, producing Windows `never used` warnings under `-D warnings`. A direct-invocation expectation was also corrected to the N087 range convention, because a direct child fails its inherited **ceilings** (71) before its identity.

## 4. Positive technical-branch CI — PASS (the proof)

Final technical head **`0dc45185110e6136bd63dc7edf774bc52b753c9e`**, tree `60f986ddddd2f6b6bd6ab201a6e336aee8c501c7`, correction ledger vs `f85caf35…` exactly three paths: façade `+69/−16` (blob `7eed6e158c9374a6c2055ddd98dd3287357b814a`), native `+10/−6` (`eacf51c5353ab3278969961aa136a0951cb7ad67`), smoke `+13/−7` (`22ee67e94cc96a70e8e3a47db9e575464b53cd07`).

Dispatched run **`43419376192`** — `event=workflow_dispatch`, `head_branch=solo-wo-093-r01`, `head_sha=0dc45185…`, `run_attempt=1`, **conclusion success**: Ubuntu job `131865233532` success (33 steps, 9 skipped), Windows job `131865233959` success (33 steps, 14 skipped). Every skip is a host-inapplicable cross-pair (Linux-only steps skipped on Windows and the mirrored Windows steps on Linux: `cargo-deny`, the advisory fixture, the byte-reproducible build, the R03 snapshot filesystems); no mandatory gate was skipped on its own host.

Real Ubuntu kernel evidence, copied from the log:

```text
n093_group=terminated leader_pid=4501 descendant_pid=4504 pgid=4501 sid=4501 signal=9 leader_reaped=1 descendant_nonrunning=1 wall_ms=2
n093_refusal=invalid_readiness=HOST_OUTPUT_INVALID stdout_bytes=0 group_signal=none leader_cleanup=checked wall_ms=10
n093_direct=group-leader exit=71
n093_direct=group-invalid-leader exit=71
n090_reap=reaped_before_join pid=4494 wall_ms=4006
```

Distinct real PIDs, `leader_pid == pgid == sid == 4501`, `descendant_pid = 4504 ≠ 4501`, `signal=9`, leader reaped, the same descendant observed non-running, and the N090 regression unchanged. Passing on Ubuntu: `n093_01`, `n093_02`, `n093_03`, `n093_06`, all four `n093_05` unit cases, `n093_07`, and `n090_01`. Windows executed `n093_04_every_group_mode_is_unsupported_before_any_child_exists`, `n093_06` and the four `n093_05` cases, with **no** Linux-success evidence on that host.

Security: `cargo-deny 0.20.2` (archive sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`) against RustSec `af7b3ec33460b8e2de8d422e16d628518891a519` fetched `2026-11-13T11:44:05Z` — advisories `ok` (0 errors, 0 warnings, 0 notes), bans `ok`, sources `ok`, licenses `ok` with **1 warning**: `license-not-encountered` for `Unicode-3.0`, an allow-listed licence never encountered in the unchanged locked graph. That warning is pre-existing, unrelated to this order and **not** silenced; this run is therefore **not** described as warning-free. The `advisories FAILED: 1 error` line in the log belongs to the deliberate `RUSTSEC-2026-0009` negative probe that must fail closed.


## 5. Red CI sensitivity proof — intentional, on one isolated branch

Exactly one task-owned branch `solo-wo-094-negative-r01` was created from the green technical revision `0dc45185…` in the same checkout. Its **sole** change is the authorized one-guard mutation in `validate_group_attempt_v1` (façade `+3/−2`, commit `0e9b3677cf774b9e31ac8fc52605e88112522be6`, tree `84c8d55a38767a287f6203faf563b7e116ec4977`, diff vs the green head: **one** path, `M crates/solo-adapters/src/media_worker_host.rs`):

```rust
// MUTATION ONLY on solo-wo-094-negative-r01:
// Deliberately omit the !attempt.descendant_nonrunning term.
if attempt.descendant_start_before == 0
    || attempt.descendant_start_after != attempt.descendant_start_before
{
    return Err(SmokeRefusalV1::HostDescendantUnverified);
}
```

The unchanged unit test `n093_05_signal_reap_observation_and_deadline_facts_stay_distinct` still asserts that `descendant_nonrunning: false` is refused. Dispatched run **`43419899358`** — `event=workflow_dispatch`, `head_branch=solo-wo-094-negative-r01`, `head_sha=0e9b3677…` — is **intentionally RED**, and red only where it must be: both jobs failed at the **`Test (workspace, all targets, locked)`** step (Ubuntu `131866990308`, Windows `131866990167`), i.e. after successful format, build and Clippy, never at compile or lint. The failing assertion, identical on both hosts:

```text
test media_worker_host::tests::n093_05_signal_reap_observation_and_deadline_facts_stay_distinct ... FAILED
thread '…' panicked at crates/solo-adapters/src/media_worker_host.rs:5979:13:
assertion `left == right` failed: this fact must refuse as HostDescendantUnverified: GroupAttemptV1 { …, descendant_start_before: 987654, descendant_start_after: 987654, descendant_nonrunning: false, wall_ms: 731 }
  left: Ok(GroupTerminationV1 { leader_pid: 4712, descendant_pid: 4715, group: 4712, session: 4712, signal: 9, wall_ms: 731 })
 right: Err(HostDescendantUnverified)
```

The mutation let an **unproven descendant become accepted evidence**, and the guarding test caught it on both hosts. This run is evidence of test sensitivity only; it is **never** an acceptance gate and is not green. After the run the checkout returned to `solo-wo-093-r01`, confirmed at `0dc45185…` with `MUTATION ONLY` absent (0 occurrences) and the guard present, so the technical branch contains **none** of the mutation. The negative branch is retained solely as this evidence and must never be merged.

## 6. Evidence ledger

| Item | Judgment | Exact identity |
| --- | --- | --- |
| Preflight, 13 main pins + 3 branch identities | **PASS** | all exact, §1 |
| Commit A on `main` | **PASS** | `8b643f7f…` / tree `8f3b0f7a…` / parent `8acde13e…` / 1 path |
| Commit A `main` CI | **PASS** | run `43418395749`, Ubuntu `131861943865`, Windows `131861944274` |
| Correction commit 1 (exports + reap + guard) | **PASS (superseded by 2)** | `64054a19df44ea728be2db2c791117a5db4a117c`, tree `160ba4f0…`, parent `f85caf35…`, 2 paths |
| Correction commit 1 CI | **FAIL — fixed forward** | run `43418923312` attempted once; Ubuntu `131863728488` failed at Test, Windows `131863728112` failed at strict lint |
| Correction commit 2 | **PASS** | `0dc45185110e6136bd63dc7edf774bc52b753c9e`, tree `60f986dd…`, parent `64054a19…`, 2 paths |
| Final technical branch dispatched CI | **PASS** | run `43419376192`, Ubuntu `131865233532`, Windows `131865233959`, both success |
| Real Linux N093 positive + negative + direct | **PASS** | §4 log lines, distinct PIDs |
| N090 regression on Linux | **PASS** | `n090_reap=reaped_before_join pid=4494 wall_ms=4006` |
| Windows `n093_04` unsupported | **PASS** | executed, no Linux success evidence |
| Local `cargo fmt --all -- --check` | **PASS** | exit `0`, run before every commit |
| Local build / Clippy / tests | **NOT APPLICABLE by order** | no linker on this workstation; explicitly not a gate here |
| Toolchain / linker / target installation | **NONE** | nothing installed |
| `solo-wo-094-negative-r01` sensitivity | **PASS (intentional RED)** | `0e9b3677…`, run `43419899358` |
| Security gate | **PASS with disclosed warning** | §4; `Unicode-3.0` `license-not-encountered` retained |
| Merge / PR / promotion / release | **NOT PERFORMED** | not attempted; branch stays unmerged |


## 7. Final state, nonclaims and cleanup

`main` is documentation-only for this order: one issued-order commit and one report commit, no product, dependency, workflow, `deny.toml`, toolchain, checker-fixture, `main.rs` or `lib.rs` change. The technical work lives **only** on unmerged `solo-wo-093-r01`; the mutation lives **only** on `solo-wo-094-negative-r01`. Both `main` runs are documentation/issuance gates and are **not** described as proof of technical-branch functionality — the `workflow_dispatch` run `43419376192` at `0dc45185…` is that proof.

Preserved refs: `solo-wo-093-r01` advanced to `0dc45185…` (history retains `f85caf35…`), `solo-wo-094-negative-r01` created at `0e9b3677…` (evidence only), and every earlier branch unmoved — `solo-wo-061-r01` `4371af21…`, `-069-` `e6162cdf…`, `-070-` `d270f95c…`, `-076-` `40a40bab…`, `-079-` `380393dd…`, `-081-` `cacc0f9b…`, `-083-` `ccc3ca8f…`, `-085-` `5bf9082d…`, `-087-` `8250f742…`, permanently stopped `solo-wo-089-r01` `1236915cab6e87b17ce84813430897b273fc3230`, accepted `solo-wo-090-r01` `0eb3fcf645851ae752bd59e12fe32b69604197d2`.

Preserved ignored material, inspected and never deleted: `.temp/solo-wo-073-wip/` at **4 files / 46,062 bytes**, `architecture-fixtures/`, `toolchain-installer/`, `target/` and prior Cline logs. No task-owned scratch directory was created in Solo `.temp/` or the Cline task area for this order, so there is nothing to remove beyond the empty state already described; no unknown or Owner-owned material was touched.

**Explicit nonclaims.** No real kernel `wait()` failure was injected — `n093_07` proves only the *classification* of an injected `io::Error` through the production helper. No **descendant reap** is claimed (the parent holds no handle for it); a `Zombie`/`Gone` observation tied to the same pre-signal start time is non-running, not reaping. No race-free PID/PGID pinning is claimed: the pre-call `try_wait` bracket narrows, but does not eliminate, leader-exit-and-reuse. No hostile-worker sandbox, process-tree containment, aggregate resource guarantee, Windows Job Object, AVIF decoding, route, host, publication or release capability is claimed or added. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

## 8. Handoff

Submitted for the Chief Master Architect's independent **ACCEPT / REVISE / STOP** against this report's exact revision. The Agent does not self-accept, and the Owner alone decides any later closure, integration, next order or release.

**This report's own future commit SHA is deliberately absent**, as is any run identifier for the report commit itself.
