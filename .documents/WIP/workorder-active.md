# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-084 R01**  
**Title:** R03.C3 — integrate the independently accepted Linux `RLIMIT_AS` reservation-refusal proof by exact two-blob promotion to `main`  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner → Implementation Agent activation:** `SOLO-WO-084 R01`  
**Authority:** This entire document becomes ACTIVE only after the Owner installs it at `.documents/WIP/workorder-active.md` and separately sends the exact matching activation message. The Chief Master Architect is permanently GitHub **READ-ONLY**. Only the Owner-activated Implementation Agent is permitted to make the expressly authorized Git commits and pushes. An ACCEPT of an earlier isolated proof is not a standing merge, release or publication authorization.

## 1. Owner decision, result, and closed scope

`SOLO-WO-083 R01` has been independently **ACCEPTED** at preserved technical branch `solo-wo-083-r01` commit `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`: one trusted, Linux pre-exec-limited child proved that **one** fixed, fallible `Vec::<u8>::try_reserve_exact(805306368)` capacity request returned `Err` under an observed `RLIMIT_AS` ceiling of 536870912 bytes. The child verified both soft/hard `RLIMIT_AS` halves and both `RLIMIT_CPU` halves first, and `pgid == pid`; no bytes were touched, filled, indexed or made resident by the test. Its accepted branch CI run `43258192188` was successful on Ubuntu and Windows, and the final report on `main` passed both hosts at `5eb882786d68ea45c624f195706a2ed92de94c3b` (run `43259910017`). No source from WO-083 is currently on `main`.

**Authorize integration only:** copy **exactly two already-reviewed Git blobs**, with no byte edits, from that immutable technical-branch commit onto `main` in one implementation integration commit. Keep all earlier accepted code, architecture checks, security gates, workflow, lockfile, manifests, native unsafe boundary, CLI composition root, and behavior unchanged. Require fresh complete two-host CI **on `main` at the exact integration commit**, then a separate report-only mainline commit and fresh complete two-host CI on its exact HEAD.

**Permitted success vocabulary:** `LINUX-RLIMIT-AS-RESERVATION-REFUSAL-INTEGRATED` after the exact blob match and all mandatory CI/cleanup/report gates pass. This is **not** `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED`, AVIF validation, a product release or publication. It is still only one Linux virtual-address-space capacity-refusal test, not page touching, RSS/OOM enforcement, CPU-time exhaustion, aggregate quotas, descendant cleanup, hostile-child isolation or a Windows Job Object. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; media correctness/publication authority flags stay `false`.

**Non-negotiable constraints:** Do not write any new source code, reformat, normalize line endings, patch assertions, alter process-control APIs, change the allocator request, change a timeout, add a dependency, change `unsafe`/FFI, touch a workflow, change a branch history, merge/cherry-pick the whole proof branch, release, deploy, or start another order. An unrelated or new CI failure is a STOP/REVISE report, **not** permission for corrective edits under this order.

## 2. Exact preflight — STOP on mismatch

Use the one existing Owner-local checkout only. Before staging anything, independently establish full actual values from Git and `origin` (not shortened captions). Require `HEAD == main == origin/main` at the exact baseline below, exact baseline tree, clean index and working tree **except solely** the Owner-installed new ACTIVE replacement, exactly one worktree and one remote `origin` at `https://github.com/provelopment/provelopment-foundation-solo.git`, no unknown tracked/untracked files, and exactly the four WIP filenames `workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`. Verify both target files currently carry the baseline blobs. Compare the locally installed new ACTIVE with the order's matching ID, and ensure its installation is the only pending change. Do not use `reset`, `clean`, stash, force-push or destructive checkout to manufacture a clean preflight.

| Anchor | Exact required value |
| --- | --- |
| Baseline `HEAD == main == origin/main` | `5eb882786d68ea45c624f195706a2ed92de94c3b` |
| Baseline `HEAD^{tree}` | `84f9c070cca9313be76b5a02585c0abc17093003` |
| Current canonical report `.documents/Reports/execution-report.md` | blob `3ed55d433031be4f9c64e0b4161276ac8b34d8a6` |
| Previously tracked `.documents/WIP/workorder-active.md` | blob `9f23156531357eebd7590e62dfad6f3ce26146ff` |
| Architecture Charter | blob `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` — the two literal `edges=41` assertions stay intact |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` |
| **Baseline `crates/solo-adapters/src/media_worker_host.rs`** | **`aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0`** |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` |
| **Baseline `crates/solo-cli/tests/media_worker_host_smoke.rs`** | **`1846af8459c21063bd2c2d7e112c05b9baf599ae`** |
| Unsafe guard `crates/solo-cli/tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| Architecture graph `crates/solo-cli/tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| Workspace graph `crates/solo-cli/tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Accepted proof branch `solo-wo-083-r01` local and remote | `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`, tree `b299b005c03657344213632eb0e0e4623023f694` |
| Proof branch immediate parent / issuance | `5247a769a1d5d39510b7a8d9165d1451b75c4e1f` |
| Preserved `solo-wo-081-r01` | `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` |
| Preserved `solo-wo-079-r01` | `380393ddacf37fc1a6398805235ca2d5dbd9677e` |
| Preserved `solo-wo-076-r01` | `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Preserved `solo-wo-061-r01` | `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Preserved `solo-wo-069-r01` | `e6162cdf733ea3136e96cbc139e46a6daf4054ee` |
| Preserved `solo-wo-070-r01` | `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |

The accepted proof branch is to remain untouched. Check its own ACTIVE is the earlier `9f23156531357eebd7590e62dfad6f3ce26146ff` blob, and that comparing its issuance parent to its tip changes exactly the two files in §4 (one branch commit). No WO-083 report/WIP blob is to be imported. Verify `main` and all branch refs remotely before any commit; record exact real evidence in the report. Inventory ignored/inherited material without modifying it, including the uncompiled `.temp/solo-wo-073-wip/` research (historically 4 files / 46,062 bytes) and existing `target/`. Missing or changed anchors require STOP, not improvisation.

## 3. Mainline ACTIVE-only issuance, then both-host CI

After successful §2 preflight, stage **only** `.documents/WIP/workorder-active.md` and commit the entire Owner-installed `SOLO-WO-084 R01` file unchanged. Confirm exactly one changed path (ACTIVE), no report/source edit, single correct parent = baseline, and verify commit SHA, tree SHA and new ACTIVE blob from GitHub. Push by ordinary fast-forward to `origin/main`. Require the original push-triggered **`rust-verify`** workflow at the exact issuance SHA to conclude `success` with both `verify (ubuntu-24.04)` and `verify (windows-2025)` jobs `success`, no failed steps and only their usual host-inapplicable skips. Record the run ID and both job IDs. No technical integration is authorized before this gate passes.

A failing issuance CI, unexpected commit ledger, remote drift or missing job requires STOP and a truthful report, not source staging. Once successful, reverify local `HEAD == main == origin/main` at the issuance SHA and a clean index/working tree.

## 4. Precisely two source paths — byte-identical promotion

From the clean `main` at the issuance commit, copy **exactly** the following Git blobs from the immutable accepted branch commit. Neither path may be edited, normalized, rewritten, re-formatted or re-generated.

| Authorized permanent integration path | Baseline blob on `main` | Required accepted branch / staged / integrated blob | Required diff |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` | `M`, +780 / -5 |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` | `M`, +212 / -0 |

**Only authorized staging mechanism** (use the single shell/PowerShell line with the exact full SHA and the two literal pathspecs; no wildcard or whole-tree merge):

```text
git restore --source=ccc3ca8fb37d52b405a2854e4962fb79aff06ab2 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

Perform and log all **five mandatory proofs before commit**:

1. `git diff --cached --name-status` is **exactly two `M` rows** with the two authorized paths; `git diff --cached --numstat` is precisely `780 5` and `212 0`; `git diff --cached --check` is empty.
2. `git rev-parse :<each-exact-path>` equals the two required accepted blob SHAs and equals `git rev-parse ccc3ca8fb37d52b405a2854e4962fb79aff06ab2:<that-path>` individually.
3. Hash the **worktree object** for each file with `git hash-object <path>` (without re-writing it); it too equals the staged and accepted Git blob. No CRLF or formatting drift is permissible.
4. `git ls-files -o --exclude-standard` is empty and `git status --porcelain` shows only those two authorized staged entries. No dirty unstaged, unknown or third file exists.
5. `git diff --name-only <issuance-sha> --cached` and the final precommit audit show **only those two paths**, with the source branch and all protected non-ledger blob SHAs unchanged.

Commit these two files **together in one implementation integration commit** on `main`, with exactly the issuance commit as its parent. Verify its exact commit SHA, tree, parent, changed paths, numstat and both path blobs via GitHub readback; fast-forward push only. Reverify `main == origin/main` and clean worktree. No branch is to be created or modified. Never import the proof branch's prior ACTIVE or prior report.

**Permanently permitted tracked-file ledger for this order:** (1) ACTIVE in issuance commit; (2) exactly these two implementation files in integration commit; (3) canonical `.documents/Reports/execution-report.md` in final report-only commit. This means **four paths total across three commits**. No WIP plan, Charter, SOP, workflow, `Cargo.lock`, manifest, native module, crate root, CLI root, tests/guards outside the two-path ledger, README or other tracked file may change.

## 5. Fresh full integration CI on `main` at the integration SHA

Require a new push-triggered `rust-verify` run whose **`head_sha` equals the integration commit**, whose `head_branch` is `main`, whose overall conclusion is `success`, and whose Ubuntu and Windows jobs are individually `success`, each with its original 33 configured steps, zero failed steps and only documented host-inapplicable skips (normally 9 Ubuntu / 14 Windows). Do not reuse WO-083 branch CI or issuance CI as integration evidence. Inspect logs, not only summary badges. Disclose the exact run ID and job IDs.

Required direct **Ubuntu** evidence, freshly produced at the new integration SHA:

- `n083_01_the_child_reserves_over_the_ceiling_and_is_refused ... ok` with a real `n083_as=media_host_as=limit_enforced ...` line. Validate `as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied`, a positive `pid`, `pgid == pid`, `wall_ms < 15000`, and normal child exit with successful reaping. Record the actual new run-specific `pid`, `pgid` and timing; never reuse branch run `pid=4519 wall_ms=10` as supposedly fresh evidence.
- `n083_02_a_direct_as_child_refuses_before_the_reservation ... ok`, with the actual direct-child status and no accepted output. The old branch observed `exit=71`; validate the new host's actual real result, do not invent it.
- `n083_03_every_wrong_reservation_token_is_refused_by_the_real_binary ... ok` plus all six `n083_03` pure parser, profile, evidence and malformed-input cases.
- Retain `N076-01..03` Linux `getrlimit` tests, `N079` pre-exec smoke and direct-child refusal, and `N081` fixed wall-deadline `SIGKILL=9 reaped=1` real test. Record their new CI evidence and all applicable earlier regressions. Nothing here authorizes changing the `N081` 2-second watchdog, `N079` 512 MiB/10-second profile, or `N083` 768 MiB request.

Required **Windows** evidence: both new `as-parent` and `as-child` refuse `HOST_UNSUPPORTED` at status 3 with empty stdout before any child/reservation exists; `n083_04_both_reservation_modes_are_unsupported_before_any_child_exists ... ok`. All six platform-neutral `n083_03` cases and original Windows N079/N081 refusal tests pass. No Linux-only `n083_01`, `n083_02`, `n083_as=`, `n083_direct_as_child=`, N076 kernel tests, or Linux-only N079/N081 real-child cases run on Windows.

**Both hosts:** real-tree checker must report `architecture=legal packages=4 targets=33 files=60 edges=41`, workflow's `edges=41` fail-closed assertions unchanged, `self-check=passed cases=29`, dynamic-load and checker-bypass negatives refuse, pinned Rust 1.99.1, locked fmt/strict Clippy `-D warnings`/workspace tests, `cargo-deny 0.20.2` with security-source refresh and all advisories/bans/licenses/sources accepted, required failure-tripwires, release build and `release_smoke=verified accepted_exit=0 refused_exit=2`, Ubuntu forbidden-import injection and exact restoration, and Ubuntu two independent clean release builds identical by bytes. The expected intentionally injected failure within negative tests is not a failing mandatory workflow run; distinguish it carefully. Record Git `Cargo.lock` blob `b1ee56da…` as single cross-host identity while allowing the already-observed raw checkout SHA-256 difference caused by line endings (Ubuntu `bf421c6e…`, Windows `afffb921…`); verify each host's lock bytes are unchanged by the security gate.

The integration changes no workflow and no protected unsafe boundary. **Any** unresolved mandatory failure, changed graph edge, new unsafe code, missing Linux evidence, ambiguous or un-reaped child, or failed Windows negative requires STOP, not a patch outside the two blob-copy scope. If a correction would alter even one accepted byte, return `REVISE` for separate Owner approval; do not silently replace the accepted source.

## 6. Report-only commit on `main` and final both-host readback

After **successful** integration CI, overwrite **only** `.documents/Reports/execution-report.md` with one complete canonical report for `SOLO-WO-084 R01` (no second report, copy, archive or extra WIP file). The canonical report must state the exact order ID, authorization and nonclaims, all preflight anchors, initial remote refs, issuance SHA/tree/parent/ACTIVE blob and CI IDs, accepted proof branch identity, both target source blob SHAs before/after, path-scoped mechanism and the five staging proofs, integration SHA/tree/parent/files/numstat, complete exact integration CI run/job IDs and log evidence, no authorized non-ledger changes, all security/architecture gates, any actual failures, and final local cleanup/branch inventory. Report actual local linker limitations truthfully: if Owner-local `link.exe` remains unavailable, compilation and kernel proof came **from remote two-host CI**, not from local Rust execution.

Commit **only** `.documents/Reports/execution-report.md` on `main` (one path), with the integration commit as its **exact parent**, and push by ordinary fast-forward. Re-fetch its commit SHA/tree/parent/report blob/only changed file from GitHub, check `HEAD == main == origin/main`, and require a second fresh **push-triggered** full `rust-verify` at that **exact report HEAD** with both hosts fully successful. Inspect its jobs and logs, not only a status badge; independently check the real Linux AS reservation refusal still ran on the final report HEAD, and Windows unsupported-negative still passes. Send the report commit's SHA/tree/blob and final CI run/job IDs in the agent's final chat response; the report need not embed the SHA of its own not-yet-created commit.

At end, explicitly re-read both integrated source blobs at the **final report HEAD** and ensure they still exactly match the two approved accepted branch blobs; re-read workflow/lock/native/guards. Confirm preservation of `solo-wo-083-r01` and all prior branch tips. No tag, release, deployment, PR, public AVIF pipeline or new work-order execution is authorized.

## 7. Cleanup and Owner-data safety

Stay within the verified **single** Solo checkout; no alternate worktree/clone, other repository/remote, extra WIP file, overwrite of any source outside the precise ledger, destructive `git clean`/`reset`/checkout, stash of unknown data, history amendment or force push. Temporary report or CI-log scratch, if needed, belongs only in ordinary approved temporary locations and must be accounted for at closure. Clean up only positively identified **task-created** temporary artifacts. Preserve all Owner-owned/unknown files and inherited `.temp/solo-wo-073-wip/` (historically four uncompiled/unverified files totaling 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, any Owner archives and `target/` untouched unless the Owner later explicitly authorizes otherwise. Do not claim `.temp` top-level empty without actually inspecting it, and do not infer temp cleanliness from `git status`.

Confirm clean final index, clean worktree and one remote/worktree/four WIP filenames. If a temp artifact cannot be safely removed, give exact non-sensitive retention/failure evidence; do not erase unknown Owner data to create a passing report. Historical branches remain preserved at the exact §2 anchors. If any fact is not independently observable (for example the GitHub connector cannot establish local checkout state), agent must report its own local direct evidence, not attribute it to Architect-side verification.

## 8. Exact acceptance gates

| Gate | Mandatory evidence | Required result |
| --- | --- | --- |
| `S084-01` | Full baseline SHA/tree/blob/branch/preflight, root, one worktree/remote, exactly four WIP, Owner-installed ACTIVE only dirty | PASS or STOP |
| `S084-02` | ACTIVE-only issuance commit, exact parent/tree/one path, GitHub readback, first new both-host CI green | PASS or STOP |
| `S084-03` | Literal path-scoped two-blob promotion and all five staged/index/worktree checks, `M+M` with `780/5` and `212/0` | PASS or STOP |
| `S084-04` | Exactly two-file integration commit, parent=issuance, remote readback, matching exact accepted blobs, protected paths unchanged | PASS or STOP |
| `S084-05` | New integration push-run at exact `main` SHA, both hosts complete green, genuine Linux N083 capacity denial | PASS or STOP |
| `S084-06` | Windows N083-04 unsupported negative, Linux-only cases not run; N076/N079/N081 regressions intact | PASS or STOP |
| `S084-07` | Architecture `packages=4 edges=41`, refusal cases=29, fmt/clippy/tests/security/tripwires/release smoke/restore/reproducibility all applicable gates | PASS or STOP |
| `S084-08` | Canonical report-only main commit parent=integration, remote blob and only-path proof, second new report-HEAD two-host CI complete green | PASS or STOP |
| `S084-09` | Final branch preservation, one worktree/remote, four WIP, clean status, task scratch disposed and Owner scratch preserved, final two blobs identical | PASS or STOP |
| `S084-10` | Only `LINUX-RLIMIT-AS-RESERVATION-REFUSAL-INTEGRATED`, no broader containment/media/Stage/release claims, all failures honestly recorded, no self-ACCEPT | PASS or STOP |

No applicable gate may be called PASS because of a prior run or agent assertion. A required new run not yet executed is `NOT RUN`/`UNVERIFIED`, not PASS; a failed run is `FAIL` or `BLOCKED`, not concealed. On any STOP condition, preserve the current work honestly, publish a truthful canonical report on `main` **only if safe**, and return the evidence to the Owner/Architect without unauthorized fixes, source drift or another order. Do not self-ACCEPT.

## 9. Mandatory final handoff

Return one concise report-ready message: exact `SOLO-WO-084 R01` identity, **implementation-agent claimed** outcome (not self-acceptance), original baseline, issuance/integration/report SHA-tree-parent triples, issued ACTIVE and final report blobs, two-file source ledger and exact final blob equality, three CI run IDs with Ubuntu and Windows job IDs and exact run `head_sha`, fresh `n083_as`/`n083_direct_as_child` values from integrated `main`, N081 kill/reap and N079 readback regressions, Windows fail-closed tests, architecture/security/downstream results, all nonpassing run disclosure, final remote branch refs, 4 WIP/one-worktree/cleanup evidence and limitations. The Chief Master Architect independently audits the canonical report SHA and returns **ACCEPT / REVISE / STOP**. The Implementation Agent must not self-ACCEPT, merge any other branch, publish AVIF, release, or begin another work order.

**Owner authority preserved. GitHub writes belong solely to the Owner-activated Implementation Agent; the Chief Master Architect remains READ-ONLY.**
