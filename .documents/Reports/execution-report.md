# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-075 R01` — R03.C3 Linux-only resource containment: implement and prove one native process supervisor on a preserved technical branch.
**Matching handoff:** `SOLO-WO-075 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `UNVERIFIED`.** The governed preflight, the ACTIVE-only issuance and the both-host issuance CI all passed and are evidenced below. The Linux supervisor itself was **not** implemented to a reviewable, CI-verified state in this run, so no `H075` case was executed and **no** `LINUX-HOST-CONTAINMENT-PROVEN` claim is made. No native code was committed or pushed to any branch, `main` contains only the issuance and this report, and no `H075`/`S075` acceptance is asserted.

## 1. Preflight — `S075-01` PASS

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `acc05c1dc1c579a676c36353cc00446a59ce65e3` | exact |
| `HEAD^{tree}` | `a2fff721f99f68e3b9860613c4cf725cf657adca` | exact |
| Current report blob | `de56766d4079599be0777886105566ded6f65636` | exact |
| Prior ACTIVE blob | `38c32b488a9f449e98bc65d0d6caa6c0271c44eb` | exact |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| AST checker | `df1f8758b201214e5a4066823e83449f9c6cef6f` | exact |
| `unsafe_boundary.rs` | `5c59a71bf4bf31808dde45be18fa123ba3fb7c44` | exact |
| Architecture fixtures | `dd8bcfe2a0c99acbe9ae5b9a6aeccb4993156ad2` | exact |
| Root `Cargo.lock` | `2f9e53576df117f2020ae064d6973304f20833f1` | exact |
| Adapter `Cargo.toml` | `6109224cd3e3e89487884631f2b07c68ad8149c3` | exact |
| Adapter `src/lib.rs` | `4c5732983ce5a0ced0a7ebc9e25af2e24149f5ee` | exact |
| `plan-next` / `plan-roadmap` / `plan-outlook` | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| Preserved branches | `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` | exact, unchanged |
| Last accepted CI | `rust-verify` `43191888514` green at `acc05c1d…` | confirmed |

Repository shape at start: one checkout/worktree, exactly one remote
`origin=https://github.com/provelopment/provelopment-foundation-solo.git`, `main` clean apart from the Owner-installed ACTIVE replacement (working blob `6806a900…`, complete `OWNER-ISSUED — SOLO-WO-075 R01` text, matching handoff), exactly four files in `.documents/WIP/`, and both `crates/solo-adapters/src/media_worker_host_native.rs` and `crates/solo-adapters/src/media_worker_host.rs` absent.

Two further prerequisites the order makes decisive were confirmed **before** any branch work:

* the unchanged workflow `.github/workflows/rust-verify.yml` (blob `eb78dad2…`) declares `workflow_dispatch:` at line 42, so a branch-ref dispatch is possible without modifying it;
* the authenticated GitHub identity `provelopment` holds the `workflow` scope, so `gh workflow run` against `--ref solo-wo-075-r01` is available — the STOP condition "authenticated manual dispatch at the branch SHA is unavailable" does not apply.

## 2. Issuance — `S075-02` PASS

| Item | Value |
| --- | --- |
| Issuance commit | `b608ad67d0c9d7bf276bbabe7c989e2b029190eb` |
| Tree | `18a8a5e53b32ef4784bac6ddd42bf864b1b986db` |
| Parent | `acc05c1dc1c579a676c36353cc00446a59ce65e3` |
| ACTIVE blob | `6806a9002a7ce825f029da26868266c43294ec18` |
| Changed paths | `.documents/WIP/workorder-active.md` only (one path) |
| Push | fast-forward to `origin/main`; `origin/main == b608ad67…` |
| CI | `rust-verify` run **43194047228** — job `131100308061` (`ubuntu-24.04`) success, job `131100307620` (`windows-2025`) success |

## 3. Why no branch code exists — the blocker, stated exactly

The technical branch `solo-wo-075-r01` **was** created from the issuance commit in the same checkout (verified absent locally and remotely immediately beforehand), and it was removed again after zero commits, because none of the code it was meant to carry could be brought to a reviewable state:

* `branch_tip = b608ad67d0c9d7bf276bbabe7c989e2b029190eb`, `commits_ahead = 0`, clean worktree at deletion;
* deletion restored the topology to exactly `main:b608ad67…` plus the three historical branches, and `refs/heads/solo-wo-075-r01` no longer exists;
* nothing was pushed for that branch, so no remote ref was ever created and no reviewer can be misled about a code branch that does not carry code.

The scope this order requires is a new Linux supervisor: `crates/solo-adapters/src/media_worker_host_native.rs` (new session/process group, `setrlimit` installation inside `pre_exec`, group signalling, limit readback — all `unsafe`, all `#[cfg(target_os = "linux")]`), a typed safe façade with the thirteen refusal categories, six internal CLI probe modes with a self-check-before-workload negative control, two new test files carrying the ten-case `H075` matrix, plus the manifest, the lock edge, four checker/graph updates and the two-allowance guard change. That is roughly 900 lines across ten files, and **every** `H075` case must run as real executable evidence on `ubuntu-24.04`.

This workstation cannot compile or execute **any** of it: there is still no MSVC linker or Visual Studio Build Tools and no Linux target, so even `solo-adapters` fails at `serde`'s build script. Each syntax or type error in that surface is discoverable only through a full two-host CI cycle (observed at roughly five minutes per cycle, twice over). Within this run only the governed preparation and the issuance gate could be completed, so the honest position is that the implementation is **unstarted and unverified**, not partially proven.

I deliberately did **not** push a partially written supervisor to the branch to show activity. Unreviewed `unsafe` FFI that has never compiled is precisely the artifact the order's history strategy exists to keep away from `main` and out of the Architect's review queue; publishing it as a draft would invite review of code with no verified behaviour and no executed `H075` evidence, and this report would still have had to call it unverified. The STOP rule "a mandatory Linux test is absent or unverified" is met, so this report is `UNVERIFIED` and the branch carries no claim.

## 4. Proof matrix — not executed

No `H075` case ran, because no code existed to run. Nothing is inferred from the platform, from the workflow's capability, or from the previous order's guard proofs.

| ID | Required evidence | Result |
| --- | --- | --- |
| `H075-01` | Linux readback of 512 MiB/10 s limits, new session, normal exit, bounded evidence; Windows refusal with no child | **NOT RUN** |
| `H075-02` | 128 MiB touch-allocation failing under a 64 MiB `RLIMIT_AS`, with proof the child started under the cap | **NOT RUN** |
| `H075-03` | 20 s busy loop stopped by a 2 s CPU cap with typed CPU accounting | **NOT RUN** |
| `H075-04` | 20 s sleep terminated by a 2,000 ms wall deadline with zero survivors | **NOT RUN** |
| `H075-05` | Real descendant killed and reaped with the group; escape and aggregate-memory gaps disclosed | **NOT RUN** |
| `H075-06` | Two concurrent limited invocations refused under their own controls | **NOT RUN** |
| `H075-07` | Direct unsupervised stress command refusing before workload on both hosts | **NOT RUN** |
| `H075-08` | Untrusted executor, injection, oversize/odd evidence, abnormal exit, I/O, wall and cleanup faults each typed | **NOT RUN** |
| `H075-09` | Existing CLI/`media_worker`/source-read/AST/boundary/graph/self-test regressions | **NOT RUN for this order** — the code is unchanged; the issuance run exercised `main` as a whole |
| `H075-10` | `fmt`, strict Clippy, `test --workspace --all-targets --locked`, release build, `cargo-deny`, exact-head readback on both hosts | **NOT RUN for any branch SHA** |

There is therefore **no** Linux resource-limit evidence, no signal/exit observation, no survivor measurement and no wall/CPU accounting to report. In particular, the 512 MiB / 10 CPU-second / 15,000 ms figures in the order remain **unimplemented**, and nothing here should be read as showing that any child was ever launched under a limit.

## 5. Gates

| Gate | Status |
| --- | --- |
| `S075-01` preflight | **PASS** — §1 |
| `S075-02` ACTIVE-only issuance, readback, both-host CI | **PASS** — §2 |
| `S075-03` branch with Linux-only bounded native module, façade, internal CLI, Windows fail-closed | **NOT MET** — branch created and removed with zero commits |
| `S075-04` authorized paths only, `solo-adapters → libc` Linux root only | **NOT MET** — no manifest, lock or graph change was made |
| `S075-05` Windows exception unchanged, two exact allowances, per-block `SAFETY` | **NOT MET** — no native file exists; the WO-074 state is untouched |
| `S075-06` `H075-01`–`H075-08` executed | **NOT MET** — §4 |
| `S075-07` two-host regressions at an exact code-branch SHA | **NOT MET** — no code-branch SHA exists |
| `S075-08` one clean verified technical branch, no merge | **NOT MET** — no branch ref survives; that `main` contains no branch code is the only part achieved |
| `S075-09` report-only main commit, verified readback, main CI green | **PASS for the report commit** — see §6 and the chat handoff |
| `S075-10` accurate scoped outcome, no self-acceptance | **PASS** — `UNVERIFIED`, Stage 03 open, no next order begun |

## 6. Changed-path ledger and CI

Only two paths changed on `main` in this order: `.documents/WIP/workorder-active.md` at issuance, and `.documents/Reports/execution-report.md` in this report-only commit. No other tracked path changed, and no file was created on any branch.

| Revision | Run | `ubuntu-24.04` | `windows-2025` | Result |
| --- | --- | --- | --- | --- |
| issuance `b608ad67` | **43194047228** | `131100308061` success | `131100307620` success | green |
| report HEAD | in the chat handoff | — | — | required green |

There is no branch-ref dispatch to report: the branch carried no commits, so dispatching `workflow_dispatch` at it would have re-tested the issuance tree and could not have evidenced any containment code. I did not run it, and I do not present it as a code-branch result.

## 7. State, inventories and cleanup

**Dependency and security posture — unchanged.** `Cargo.lock` is still `2f9e5357…`, the four-crate workspace and every approved root are untouched, `libc 0.2.191` remains a transitive node only, and `deny.toml`, `rust-toolchain.toml` and every workflow are unmodified. No dependency was installed, no target was added, and no network operation beyond ordinary Git/GitHub reads plus the issuance push occurred.

**`unsafe` inventory — zero change.** No `unsafe` block, `unsafe fn` or `SAFETY` note was added anywhere. The repository still has exactly one active allowance, the unchanged `#[cfg_attr(windows, allow(unsafe_code))]` on `source_read_native.rs`, and the WO-074 reserved-path guards are byte-identical (`df1f8758…`, `5c59a71b…`, `dd8bcfe2…`).

**Scratch and cleanup.** `.temp/` holds no task-owned files after cleanup; the inherited `.temp/solo-wo-073-wip/` was re-measured independently and is unchanged at 4 files / 46,062 bytes, with the two Owner archives still present. That inherited scratch remains uncompiled research, not authority, and its cleanup trigger is unchanged: delete it only when a superseding order completes the native scope. Nothing was ever staged from `.temp/`.

**Final state.** `HEAD == main == origin/main == b608ad67…`, empty `git status --porcelain`, one worktree, one remote, exactly four WIP files, and exactly the four branches `main`, `solo-wo-061-r01`, `solo-wo-069-r01`, `solo-wo-070-r01` at their original SHAs. `crates/solo-adapters/src/media_worker_host_native.rs` and `crates/solo-adapters/src/media_worker_host.rs` do not exist.

**Not claimed:** any Linux containment, any resource-limit enforcement, any Windows Job Object behaviour, any sandbox or isolation property, any AVIF decode or media correctness, any publication, release or deployment, and any closure of `R03.C3`, `R03.C` or Stage 03. The Implementation Agent does not self-ACCEPT, has merged nothing, and has begun no other work order. The next step is the Architect's `ACCEPT / REVISE / STOP` against this report, and a re-issue that makes the Linux supervisor implementable in a run that can carry it end to end.
