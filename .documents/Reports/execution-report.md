# Execution Report

**status=STOPPED**

**Order:** `OWNER-ISSUED — SOLO-WO-093 R01` — R03.C3 FIXED LINUX PROCESS-GROUP AND DESCENDANT LIFECYCLE PROOF. **Matching Owner activation:** `SOLO-WO-093 R01`. **Date (ISO):** 2026-11-13.

**No outcome is claimed.** `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-PROVEN` is **not** claimed, and nothing here is self-accepted. The implementation exists on its own branch but its mandatory `push`-triggered **branch-head** two-host CI evidence cannot be produced under this order's own frozen constraints, so the mandatory gate is **UNVERIFIED**. Per §7 this is a **STOP**: no merge, no workflow change, no pull request, no self-repair.

## 1. Why this stopped

The order requires, before the report, "a fresh **branch-head** `push`-triggered `rust-verify` on **both** Ubuntu and Windows at this exact technical SHA". The repository has exactly one workflow, `.github/workflows/rust-verify.yml` (blob `e1b5ca6e1738a5d5129a8788e24d51aee02421d1`), whose complete trigger set is:

```yaml
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]
  schedule:
    - cron: "17 4 * * *"
```

A push to `solo-wo-093-r01` therefore produces **no run at all**; the branch was pushed at `17:57` and repeated `actions/runs?branch=solo-wo-093-r01` queries returned an empty set, while the five preceding `main` pushes each have their own run. The order simultaneously forbids the only two remedies: "No modification of source/checker/workflow/dependencies for CI convenience" (**CI change required → STOP**) and "Do not self-merge, open a PR, cherry-pick, rebase, amend, force-push".

There is no third route. The requirement is unsatisfiable within this order's stated boundary. I did **not** widen the trigger, weaken a gate, open a PR, or push the technical commit to `main`.

**Independent second blocker, same gate.** The mandated local gates in §5.5 (`cargo clippy … -D warnings`, `cargo test …`, `cargo test -p solo-cli --test check-architecture`, the architecture negative self-test) cannot execute on the Owner's checkout either: the only installed toolchain is `1.99.1-x86_64-pc-windows-msvc`, and this machine has **no linker** — `link.exe` is absent, and there is no Visual Studio, Build Tools, LLVM/`lld-link` or MinGW/MSYS installation. `cargo check` fails at the first dependency build script:

```text
error: linker `link.exe` not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
```

Installing a linker or an additional target is a **toolchain change**, which the order forbids and lists as a STOP trigger. Consequently the §5.4 failure-sensitivity mutation ("invert or bypass one new acceptance/refusal check … to demonstrate that the intended test rejects it") could not be executed, and is reported **UNVERIFIED**, not passed.

Only `cargo fmt --all -- --check` could run, and it **passed** (`rustfmt` parses sources and links nothing).

## 2. What was completed before the blocker

**Preflight — PASS.** Sole checkout, `main` checked out, one worktree, one remote `origin` (fetch and push both the approved URL), no stash, exactly four WIP files, and the only dirty path was the unaltered Owner-installed `SOLO-WO-093 R01` ACTIVE. All **sixteen** baseline identities matched exactly: start `HEAD == main == origin/main == a4f62b057916dc592f4fa59b8fdf8aca17315857`, tree `6db880ed…`, sole parent `01f9cac1…`, ACTIVE-neutral `2ef2db63…`, NEXT `1090ff41…`, ROADMAP `9d7a8622…`, OUTLOOK `cb9c7a61…`, report `86808db9…`, Architecture `c941c7f9…`, native seam `1e698245…`, façade `a1f2bdb9…`, smoke test `54440135…`, `Cargo.lock` `b1ee56da…`, `rust-toolchain.toml` `9ec55b3b…`, workflow `e1b5ca6e…`, architecture-checker entry `0bad0e02…`. Branch `solo-wo-093-r01` did not exist locally or remotely.

**Commit A — issued, published and green.** `14b65dbb79d7da908a961abb8db91d5b91ea55d2`, tree `cc3878583cbb11e8c6cb4d1b7e69869ec6e5f41a`, sole parent `a4f62b05…`, exactly two modified paths (ACTIVE `+166/−1` → blob `7a465788a5cad6457c3b27ad13b1423284df7785`, unaltered Owner text; NEXT `+0/−12` → the required neutral blob `92fa562b1af0c54391851aa5c79deb1631127069`), clean `git diff --cached --check`, ordinary fast-forward push. Its own `push`-triggered run **`43411702516`** is green at exactly that SHA: Ubuntu job `131839447016` success, Windows job `131839447241` success (33 steps each).

**Commit B — implemented and published on its branch only.** `f85caf35a00b103a47cf83356e66df278ccca682`, tree `8ef4c768236d21729661ba2cdcf6a2615e2592d9`, sole parent `14b65dbb…` (Commit A), exactly three modified paths, clean `git diff --cached --check`, pushed to `origin solo-wo-093-r01` as a new branch with no force. `main` was **not** advanced and the branch was **not** merged, cherry-picked or opened as a PR.

Exact Commit B file ledger and blobs:

| Path | Change | Git blob |
| --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `+1527/−130` | `bd3cf9cc6aa79d4c9aa191607109f5f3a72672dc` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `+219/−1` | `61cfd0598cc995db4879cd39ad6470574b945015` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `+256/−0` | `0064d2a474ff757719a2cd83ed1c07da4e8f2992` |

No fourth path changed. `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, the workflow, the architecture checker and its fixtures, `crates/solo-cli/src/main.rs`, `crates/solo-adapters/src/lib.rs`, ROADMAP and OUTLOOK are byte-identical to the baseline.

## 3. What the branch actually contains (submitted as evidence, not as acceptance)

**Native seam, `#[cfg(target_os = "linux")]` only.** Two additions inside the positively gated module, each unsafe block carrying an adjacent `// SAFETY:` rationale, with no new native root, no new `unsafe` allowance and no generic signaller:

- `terminate_owned_group_v1(&mut Child, u32) -> Result<GroupSignalOutcomeV1, GroupSignalRefusalV1>` — derives the leader pid **inside** the seam from the caller's own handle, refuses `pid <= 1`, a self-referential descendant, a non-live leader, a mismatched `pgid`/`sid`, and a verified group equal to this process's own group; rechecks `try_wait()` immediately before the call; negates with `checked_neg`; issues exactly one `libc::kill(-leader_pid, SIGKILL)`. It returns "signal was issued" only — never a reap and never a descendant exit.
- `observe_process_state_v1(u32) -> Result<ProcessStateV1, ProcessStateRefusalV1>` — one bounded (`4096` bytes) read of the fixed `/proc/<pid>/stat` path, parsing only the fixed state character and the fixed `starttime` identity, distinguishing `Running`/`Zombie`/`Gone` and refusing anything truncated, non-UTF-8 or off-schema.

**Façade.** Five new closed internal tokens (`group-parent`, `group-leader`, `group-descendant`, `group-invalid-parent`, `group-invalid-leader`) reachable only behind the reserved flag in second position; the new marker `media_host_group=terminated`; the new refusal `HOST_DESCENDANT_UNVERIFIED`; strict closed parsers and validators for both readiness records; and `validate_group_attempt_v1`, which keeps four facts **distinct** — a signal *request*, a leader *reap*, a descendant *non-running observation*, and a descendant *reap*, the last being unrepresentable because no field or value can express it. The single 15 000 ms budget starts before leader spawn; the negative path reaches only `cleanup_owned_leader`, which reuses the accepted `SOLO-WO-090 R01` rule (checked `Child::wait()` permits joining; a failed reap overrides the refusal) and issues **zero** group signals. `GROUP_DESCENDANT_LIFETIME_MS = 8 000` and `GROUP_LEADER_HOLD_MS = 10 000` bound every task-created process.

**Tests written but never executed.** `n093_01`, `n093_02`, `n093_03` (Linux) and `n093_04` (Windows), plus the `n093_05` unit matrix and `n093_06` regression in the smoke file, and three `n093_05_*` pure unit tests in the façade that assert the acceptance rule across wrong identities, a missing signal, a missing reap, a leader not proved live, an absent or wrong signal status, an unobserved descendant, a recycled start-time identity, a zero measurement, an at-deadline measurement, and both closed record schemas.

**Explicit non-claims.** No process-tree containment, no hostile-descendant sandbox, no aggregate resource containment, no AVIF decoding, no descendant **reaping** (the parent holds no handle for it), no resolution of the PID/PGID reuse window (a pre-call `try_wait` narrows it but does not pin it), no Windows group support, no publication and no release.


## 4. Evidence ledger

| Item | State | Exact identity |
| --- | --- | --- |
| Preflight pins (16) | **PASS** | all exact, listed in §2 |
| Commit A | **PASS** | `14b65dbb79d7da908a961abb8db91d5b91ea55d2` / tree `cc3878583cbb11e8c6cb4d1b7e69869ec6e5f41a` / parent `a4f62b05…` |
| Commit A CI (own run) | **PASS** | run `43411702516`, Ubuntu `131839447016`, Windows `131839447241`, `event=push`, `head_branch=main` |
| `cargo fmt --all -- --check` | **PASS** | local, exit `0`, no linker needed |
| Commit B on branch | **PASS** | `f85caf35a00b103a47cf83356e66df278ccca682` / tree `8ef4c768236d21729661ba2cdcf6a2615e2592d9` / parent `14b65dbb…` |
| Commit B three-path ledger | **PASS** | `+1527/−130`, `+219/−1`, `+256/−0`; no fourth path |
| Branch-head push CI (both hosts) | **UNVERIFIED — blocker** | no run exists; workflow has no branch trigger |
| Local `cargo clippy … -D warnings` | **UNVERIFIED — blocker** | no linker on this checkout |
| Local `cargo test --workspace --all-targets` | **UNVERIFIED — blocker** | no linker on this checkout |
| `cargo test -p solo-cli --test check-architecture` | **UNVERIFIED — blocker** | no linker on this checkout |
| Architecture negative self-test | **UNVERIFIED — blocker** | no linker on this checkout |
| Real Linux `n093_group=` / `n093_refusal=` lines | **UNVERIFIED — blocker** | never executed anywhere |
| Failure-sensitivity mutation proof (§5.4) | **UNVERIFIED — blocker** | could not be executed without a compiler |
| Windows `n093_04` unsupported behaviour | **UNVERIFIED — blocker** | never executed |
| Merge / PR / workflow change | **NOT PERFORMED** | forbidden, and not attempted |

No CI job ID is offered for Commit B because no such job exists. Nothing in this report treats a `main` run as branch evidence.

## 5. Cleanup, preservation and surviving state

No task-owned scratch file or directory was created: no `.temp/` folder, no Cline task copy, and the planned disposable mutation workspace was never brought into existence. `.temp/solo-wo-073-wip/` is intact at **4 files / 46 062 bytes**; `architecture-fixtures/`, `toolchain-installer/`, `target/` and prior Cline logs are untouched. No blanket clean, reset, stash or destructive command was used, and no unknown or Owner-owned material was removed.

**Surviving processes: none.** No group, leader or descendant was ever started — neither branch code path ran, because there is no compiler and no branch CI. There is nothing to observe, signal or reap, and no group `kill` was ever issued by any process.

**Preserved deliberately, for the Architect's inspection:** `solo-wo-093-r01` at `f85caf35a00b103a47cf83356e66df278ccca682` (local and `origin`); `main` at Commit A `14b65dbb…`; the workflow, toolchain pin, `Cargo.lock`, `deny.toml`, architecture checker and fixtures all unmodified; `solo-wo-089-r01` at `1236915c…` and `solo-wo-090-r01` at `0eb3fcf6…` unmoved. `git status --porcelain` is empty, there is no stash, one checkout and one remote.


## 6. No change to product, dependencies, roadmap or security boundary

This run changed **no** product behaviour on `main`: `main` advanced by exactly one commit, the Owner-issued ACTIVE record plus the cleared NEXT candidate, both documentation-only. It added no dependency, no toolchain element, no workflow or gate, no Charter or SOP text, no roadmap stage and no release or deployment action. The technical commit exists only on an unmerged branch and is **not** in `main`.

`R03.C3`, `R03.C` and Stage 03 remain **OPEN**. The NEXT candidate was implemented on its branch but is **not** proven and therefore **not** closed. No media decoding, route, host or publication behaviour is asserted.

## 7. The one decision this STOP needs

The smallest Owner decision that unblocks the order, stated without choosing it for the Owner:

1. **Permit the branch-head trigger** — authorize one narrowly scoped workflow change adding `solo-wo-093-r01` (or `solo-wo-*`) to `push.branches`, as a separately approved CI change, after which the existing branch `f85caf35…` can be re-run and audited without touching the code; or
2. **Permit a pull request** from `solo-wo-093-r01` into `main`, so the existing `pull_request` trigger produces the required two-host run at the exact technical SHA; or
3. **Permit a local toolchain/linker installation** on the Owner checkout, so the §5.5 local gates and the §5.4 failure-sensitivity proof can be executed here as written; or
4. **Re-scope the evidence requirement** explicitly, accepting `main`-only CI after a later integration order, which would contradict the present order's own gate wording and so needs an explicit Owner amendment.

Until one of these is decided, the honest state is: **code written and pushed to its isolated branch, gate unverified, nothing merged, nothing accepted.**

---

**This report's own future commit SHA, tree and blob are deliberately absent**, and no run or job identifier is claimed for the report commit itself. The report is the only commit this STOP publishes on `main`, with sole parent Commit A `14b65dbb79d7da908a961abb8db91d5b91ea55d2`; its message was changed to a truthful STOP wording, which is the one intentional deviation from the order's prescribed text and is disclosed here.

Submitted for the Chief Master Architect's independent **ACCEPT / REVISE / STOP** at this report's exact revision.
