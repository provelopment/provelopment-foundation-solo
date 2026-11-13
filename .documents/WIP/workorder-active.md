# SOLO-WO-090 R01 — CORRECTIVE: Session Readiness Reap-Failure Truth and Fresh-Gate Requalification

**Authority:** Owner-issued work order only after the Owner has installed this entire file as `.documents/WIP/workorder-active.md` in the sole checkout, and explicitly activated `SOLO-WO-090 R01`. This document is not an agent authorization until both acts occur.

**Chief Master Architect disposition preceding this order:** `SOLO-WO-089 R01` = **STOP**, not accepted. The two independently identified reasons are (A) technical files were edited before the mandatory issuance CI gate, although restored before issuance and never committed to `main`; and (B) the new early-readiness-refusal path discards the result of `Child::wait()` and then joins readers as though the child were known to be gone. Neither historical breach is erased, reinterpreted as compliance, or retroactively accepted by this order. The exact WO-089 report and technical branch must remain preserved as evidence.

**One narrow authorized objective:** On a fresh, isolated `solo-wo-090-r01` technical branch created only *after* the new ACTIVE-only mainline issuance and successful two-host issuance CI, import the two WO-089 branch files **only as an explicitly unaccepted corrective starting candidate**, then repair the session-readiness early-refusal cleanup so that a failed/unverified reap cannot be masked by an earlier readiness refusal and reader joins cannot be attempted on a leader whose exit/reap was not established. Re-prove the readiness-before-query barrier with real Linux tests and deterministic cleanup-negative tests. Nothing from this order is integrated to `main`.

**Permitted claimed result, only after all mandatory gates:** `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-PROVEN` — on `solo-wo-090-r01` **only**. This is a new corrective result, NOT `SOLO-WO-089 ACCEPT`, NOT integrated code, NOT process-group authority, NOT descendant containment, NOT a production sandbox. `R03.C3`, `R03.C`, and Stage 03 remain **OPEN**, and media correctness, containment, publication and release flags remain false.

## 1. Exact authority, repository and permanent scope

Repository: `provelopment/provelopment-foundation-solo`, sole Owner checkout:

`C:\Users\Sean\Documents\Provelopment\00.foundation-solo`

There must be exactly one checkout/worktree and one remote `origin`, with identical fetch/push URL `https://github.com/provelopment/provelopment-foundation-solo.git`. No second clone, worktree, alternate repository or shadow source tree. Keep exactly four `.documents/WIP/` files: `workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`.

**Authorized permanent changes across all refs:**

1. On `main`, an **ACTIVE-only issuance commit** changing only `.documents/WIP/workorder-active.md`, followed by fresh successful original Ubuntu and Windows push CI.
2. On the **new** isolated technical branch `solo-wo-090-r01`, **only** `crates/solo-adapters/src/media_worker_host.rs` and `crates/solo-cli/tests/media_worker_host_smoke.rs`; an exact two-blob import of the stopped WO-089 candidate followed by bounded correction and tests. No technical commits or merges on `main`.
3. On `main`, one **report-only final commit** changing only `.documents/Reports/execution-report.md`, followed by fresh successful original Ubuntu and Windows push CI.

All other tracked paths, including the native seam `media_worker_host_native.rs`, adapter root `lib.rs`, CLI root `main.rs`, `Cargo.lock`, manifests, workflow, graph/unsafe guards, Charter and three WIP plans, are frozen. No extra report copy, fifth WIP file, guard rewrite, workflow change, dependency addition, new native root, process-group signal, new `unsafe`, new private CLI mode, descendant manipulation, media decoding, publication, PR, tag, release or deployment. Preserve every historical/proof branch, including the **stopped** `solo-wo-089-r01`, unchanged. Do not amend the WO-089 report or its history.

## 2. Mandatory exact baseline preflight — STOP on any mismatch

Before any technical edit, branch creation, import, test fixture, scratch recreation or issuance action, inspect the *actual* sole checkout and remote. The **only** permitted local dirty path is the Owner-installed complete ACTIVE replacement (` M .documents/WIP/workorder-active.md`). The previous tracked version is pinned below; the Owner-installed replacement is not to be repaired, trimmed, synthesized or auto-formatted. No staged or untracked file, no stash, unknown tracked edit, branch drift, incomplete Owner installation, or unrecognized ignored material may be silently corrected. Verify the new branch does **not** exist locally or remotely.

| Mandatory identity at baseline `main` | Exact immutable value |
| --- | --- |
| `HEAD == main == origin/main` | `29ccec0b276c5a180f38cbe121781ea8618a82ac` |
| Baseline tree | `2be96bc4ba88fd1107a6ebb3ffccadcc75a787d8` |
| Current canonical WO-089 report blob | `ee6417c1ab76aad44fc836de5b230e87d4160dea` |
| Previous tracked ACTIVE blob | `6167d719d2ac3c302e5fa44c9976fb94b33a43b3` |
| `.documents/Architecture/ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| **Frozen native seam** `crates/solo-adapters/src/media_worker_host_native.rs` | `1e698245493f6d10ebb9636282698a797a242865` |
| **Mainline façade before correction** `crates/solo-adapters/src/media_worker_host.rs` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` |
| **Mainline host integration test** `crates/solo-cli/tests/media_worker_host_smoke.rs` | `a97c09954e96fbddd6380fb301be182584c183ea` |
| `crates/solo-cli/tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| `crates/solo-cli/tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |

**Stopped-candidate source, not an accepted artifact:** `solo-wo-089-r01` must be unchanged at `1236915cab6e87b17ce84813430897b273fc3230`, tree `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7`. The two exact candidate blobs are façade `56676fecbd1fef510348c5b777da3a8eadf745b6` and host integration test `99b470f3073e59d30543dcc5862ec6f4037e46a3`. Their use is expressly **conditional on** successful new issuance, branch creation and fresh correction; prior green CI does not confer acceptance. The candidate branch must never be moved, rewritten or merged.

**Other historical branches to pin and preserve without movement:**

| Branch | Exact SHA |
| --- | --- |
| `solo-wo-087-r01` | `8250f742f533d99cea1c43739ea351095d4c9d73` |
| `solo-wo-085-r01` | `5bf9082bd44e6ecaedd783e2a93e6260a87073d3` |
| `solo-wo-083-r01` | `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2` |
| `solo-wo-081-r01` | `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` |
| `solo-wo-079-r01` | `380393ddacf37fc1a6398805235ca2d5dbd9677e` |
| `solo-wo-076-r01` | `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| `solo-wo-061-r01` | `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| `solo-wo-069-r01` | `e6162cdf733ea3136e96cbc139e46a6daf4054ee` |
| `solo-wo-070-r01` | `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |

Inventory ignored Owner/inherited materials **without touching them**: `.temp/solo-wo-073-wip/` (last agent-reported 4 files, 46,062 bytes), `.temp/architecture-fixtures/`, `.temp/toolchain-installer/`, `target/` and unrelated prior-session/Cline logs. Verify their actual state locally; GitHub cannot attest to ignored files. No `git reset`, `git clean`, `git checkout --` for resetting modified files, stash, forced push, rebase, history rewrite, branch deletion, destructive cleanup or secondary worktree. If preflight differs: **STOP** and report it; do not normalize it.

## 3. Fresh issuance gate — no technical edits before its success

The historical WO-089 sequencing violation is not permission to repeat it. **Before issuance CI is green, the agent must not edit, generate or park any technical source/test changes, import the candidate, create a technical branch, or create a technical scratch patch.** Inspecting read-only sources or pinning Git identities is allowed; editing is not. Confirm only the Owner-installed ACTIVE path is dirty.

Stage **only** `.documents/WIP/workorder-active.md`. Commit that complete installed document **unchanged** on `main` in one ACTIVE-only commit, sole parent exactly `29ccec0b276c5a180f38cbe121781ea8618a82ac`, and push by ordinary fast-forward. Record full issuance SHA, tree, ACTIVE blob, changed-path ledger and readback from GitHub. Require the original `.github/workflows/rust-verify.yml` **push** event at exactly the issuance SHA, `head_branch=main`, `conclusion=success`, and both original jobs `verify (ubuntu-24.04)` and `verify (windows-2025)` individually green: 33 configured steps per host, no failed steps, only 9 Ubuntu / 14 Windows inapplicable skips. Inspect the real logs and record run/job IDs. **This gate must finish green before the first technical edit, the first candidate import or branch creation.** A failed issuance gate is **STOP**, not a reason to fix source code.

After issuance succeeds, prove `HEAD == main == origin/main == <issuance SHA>`, clean index/worktree, expected tracked path ledger and unchanged branch refs. Only then may the technical phase start.

## 4. New isolated corrective branch and auditable candidate import

Create one new, previously nonexistent branch `solo-wo-090-r01` **from the successful issuance commit** in the same sole checkout/worktree, with no preexisting technical edits. Verify its base SHA and parentage. The permanent branch diff and every revision may affect **only** these two paths:

- `crates/solo-adapters/src/media_worker_host.rs`
- `crates/solo-cli/tests/media_worker_host_smoke.rs`

**Explicitly authorized candidate import, only after the new branch exists:** while the new branch is clean at the issuance base, execute this exact, path-scoped source restore (no wildcard, no whole-branch checkout):

```text
git restore --source=1236915cab6e87b17ce84813430897b273fc3230 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

This is a *new-order* authorization to import **unaccepted** candidate objects for correction, not approval of WO-089. Prove the staged ledger contains only these two `M` rows, exact candidate blobs (`56676fecbd1fef510348c5b777da3a8eadf745b6` and `99b470f3073e59d30543dcc5862ec6f4037e46a3`), and exact filter-aware worktree hashes. `git diff --cached --check` must be empty. The Owner Windows worktree may hold CRLF because of `core.autocrlf=true` and `* text=auto`; filter-aware Git object identity, not raw `--no-filters` CRLF bytes, governs equivalence. Record raw EOL differences if observed without rewriting source. Commit the imported **candidate seed only on the technical branch** as a separate, clearly labelled unaccepted seed commit, then perform the correction in subsequent branch-only commits. No candidate seed commit may land on `main` or be described as an independently accepted implementation. Preserve source branch and all its history unchanged.

No detached cherry-pick/merge, no return to pre-issuance technical edits, no mass restore and no change to any other permanent path. Bounded iterative ordinary fast-forward branch pushes are allowed; all attempted CI SHAs must be disclosed.

## 5. Mandatory corrective implementation — truthful early-refusal cleanup

Work strictly within the safe façade and host integration test files. The inherited WO-089 implementation already provides a one-shot bounded readiness handoff: the capped stdout reader sends the first complete newline prefix; the parent waits only for the remaining portion of its one 15-second deadline, validates the exact child readiness line and `Child::id()` PID, and only then makes the single `getpgid`/`getsid` pair bracketed by `try_wait()`. **Preserve this successful ordering and existing accepted output format.** Do not add new modes, native FFI, new `unsafe`, process-group signalling, descendants, arbitrary pid/executable inputs, persistent identity claims, retries or polling of the identity query.

### 5.1 Repair the observed defect

In the inherited `run_session_probe()` early-readiness-refusal branch, the current code effectively does:

```rust
let _ = child.kill();
let _ = child.wait();
let _ = join_reader(out_reader);
let _ = join_reader(err_reader);
return Err(refusal);
```

This is **prohibited in the corrected outcome**: it discards the wait result and joins readers without proof the child actually exited. Replace it with an auditable, explicitly checked, bounded leader-only cleanup path whose production behavior satisfies **all** of the following:

1. On missing/invalid/late readiness, do **not** execute `observe_process_identity_v1`. Refuse without any `media_host_session=live_attested` evidence.
2. An attempted `Child::kill()` must target **only** the directly owned `Child` handle. Record the kill result appropriately; a failed kill is never evidence of a terminated child. It may be legitimate for kill to fail if the leader exited naturally, but this cannot skip reaping.
3. Obtain an actual checked `Child::wait()` outcome. **Only `Ok(status)` establishes a completed leader reap**, regardless of the preceding kill result. A `wait()` error must be reported as `SmokeRefusalV1::HostReapFailed` (or the exact existing equivalent for an unobservable reap), **overriding** the original readiness refusal. Never silently retain `HostOutputInvalid` or `HostWallTimeout` as though cleanup were proven.
4. **Join stdout/stderr reader handles only after a confirmed successful leader reap.** If `wait()` fails, do not join potentially blocked readers. Do not claim they were joined or that a child/descendant was contained; return a truthful terminal lifecycle refusal, disclose the unresolved cleanup risk and STOP if a real runner encounters it. Do not broaden authority to kill process groups or descendants to resolve it.
5. On successful reap, join every created reader and ensure a reader panic or read failure cannot produce accepted evidence. The original readiness refusal remains a refusal even if cleanup succeeded; never convert cleanup into success. A reader failure may use the existing typed output-refusal vocabulary, but must not override a failed reap.
6. Preserve the fixed caps (stdout 1,024 bytes; stderr 512 bytes), first-complete-newline semantics, strict profile/PID checks, exact whole-stream post-reap comparison, no-EOF-before-query ordering, one 15,000 ms deadline started before spawn, four-second child sleep, 20,000 ms integration-test budget, and existing poll interval. Do not introduce a new open-ended wait loop, nested wall budget, pipe join before proven leader exit, or a `wait()` result discarded with `let _ =` on this new early-refusal path.
7. The neutral accepted attempt still requires a validated readiness gate, matching entire stdout, matching child and parent kernel identities, live before/after, normal zero exit, completed reap, no parent kill, and measured wall time strictly below deadline. All previously accepted public/internal record bytes and statuses remain fixed.

If an actual system failure makes a confirmed reap impossible, do not claim safe cleanup, do not hang on a reader join, and STOP with truthful evidence. This corrective order does not grant resources to engineer a general-purpose process supervisor or descendant-safe pipeline.

### 5.2 Prove that real cleanup and test classification are linked

Add a small, safe, production-used decision helper or equally auditable ordinary Rust control flow that maps the actual checked wait outcome to terminal cleanup classification. It must be **exercised by deterministic tests** capable of simulating a failed reap without manipulating real OS PIDs, kernel signals, external executables, fork, FFI or race timing. Tests should prove the *actual production decision rule*, not only construct `SessionAttemptV1` objects with artificial flags after cleanup has been skipped.

At minimum test: readiness refusal + confirmed reap remains refusal; readiness refusal + failed reap becomes `HostReapFailed`; a kill error cannot be mistaken for a successful reap; a failed reap prevents the reader-join phase; a successful reap permits bounded joining; an output-reader failure cannot become success; no readiness gate can authorize the native query; early line must equal final entire stream. A purely simulated test is labelled pure. A real, correctly behaved trusted child is not a way to force `wait()` to error; **do not** manufacture an OS failure through unsafe, hostile or potentially orphaning tests.

Retain N089's real Linux readiness-before-query integration test and its traceable `n089_readiness=validated_before_query` **test annotation**; retain N087's exact original accepted line and Linux kernel attestation. Add auditable `n090_` test names for the corrective lifecycle rule and real successful synchronized behavior, where appropriate, without inventing a new product evidence field or mode. The correction must be verifiable from both source ordering and fresh real CI logs at the final branch SHA.

## 6. Mandatory evidence matrix — final branch SHA only

| Gate | Required observation | Failure consequence |
| --- | --- | --- |
| `N090-01` sequencing | Clean preflight, ACTIVE-only issuance, two-host issuance green **before** branch/import/technical edit; imported seed is a new branch-only commit with exact candidate blob readback. | STOP on repeat of WO-089 ordering violation, even if later restored. |
| `N090-02` production cleanup | Early readiness refusal never queries kernel; confirmed `wait()` precedes reader joins; real failure to reap maps to `HostReapFailed`, takes precedence over initial refusal, and cannot be masked. Production-used decision rule covered by deterministic negative tests. | STOP on unchecked wait, pre-reap reader join or fake cleanup evidence. |
| `N090-03` synchronized positive | Fresh real Ubuntu `session-parent` proof with unchanged `media_host_session=live_attested` fixed fields; N089 `n089_readiness=` annotation and `n090_` proof; ceilings AS soft/hard 536870912, CPU soft/hard 10; `pid=pgid=sid=observed_pgid=observed_sid>0`; normal exit zero, `reaped=1`, wall time within 15s. | No fabricated output; no claim that a repeating PID disproves freshness. |
| `N090-04` refusals | Direct unsupervised session child refuses before readiness/sleep; malformed, absent, delayed, foreign-PID, duplicated, trailing and over-cap readiness are fail-closed; verify the early and full-stream comparison and each reader cap. | Pure tests distinguished from kernel evidence; no unsafe timeout injection. |
| `N090-05` Windows | Both private session modes remain `HOST_UNSUPPORTED` status 3, empty stdout, no child, no readiness worker, and zero Linux `n089_readiness=`/`n090` real-kernel markers. Pure tests may run. | No Windows capability claim. |
| `N090-06` regressions | N076 real kernel readbacks, N079 limit smoke, N081 wall termination/reap, N083 virtual-address-space denial, N085 CPU kernel termination without parent kill, N087 live identity, N089 barrier, public CLI byte/exit contracts all unchanged and green. | No weakened/deleted tests or existing deadlines. |
| `N090-07` architecture/security | Native seam unchanged at blob `1e698245...`; all protected paths unchanged; no new `unsafe`/FFI/root/edge; architecture `packages=4 targets=33 files=60 edges=41`, self-check `cases=29`, security/tripwire/release/reproducibility gates green. | Any required guard/workflow/manifest modification = STOP. |
| `N090-08` final isolated branch CI | Original `workflow_dispatch` at exact final `solo-wo-090-r01` SHA, Ubuntu and Windows jobs each green with 33 configured steps and only 9/14 inapplicable skips; inspect raw logs. | Any mandatory unresolved red run = STOP; no self-ACCEPT. |

Windows runner remains the authoritative compiler/linter evidence if the Owner machine lacks the local Rust toolchain. Avoid making local build claims in that case. Preserve the current cargo-deny, dynamic-loader, forbidden-import, architecture and release-smoke negative tests in their original form. An intentionally injected `test result: FAILED` in the negative tripwire is acceptable **only** when its enclosing step and both final jobs succeed.

## 7. CI iteration discipline and truthful report

The candidate seed is **not a green acceptance gate**. Fix and prove the correction on `solo-wo-090-r01`; push revisions by ordinary fast-forward only. Use the original `workflow_dispatch` at an exact branch SHA; branch pushes alone do not trigger the original workflow. Verify `head_sha`, `event`, branch, both job IDs, all 33 configured steps, actual failing steps and relevant logs. Disclose every red, cancelled, superseded or partially green attempt with exact SHA and cause. Corrections may touch only the two authorized files. Never change workflows, Cargo, lints, guards, architecture checks, historic branches or public contracts to obtain a passing result. No self-ACCEPT.

After final branch CI is green, preserve the entire new branch history and ref unchanged. Return the sole checkout to `main` at the issuance SHA with a clean index/worktree; verify the three protected mainline media-host blobs are still the accepted WO-088 originals (native `1e698245...`, façade `2f83c069...`, test `a97c0995...`), not the WO-089 candidate or WO-090 corrections. Do not merge, cherry-pick, path-promote or release implementation code under this order.

Write one comprehensive **canonical** report by overwriting `.documents/Reports/execution-report.md` on `main`, with explicit reference to the **stopped WO-089** and its two independent reasons. Report every gate, exact baseline/issuance/seed/correction/final branch SHAs and trees, parent relationships, before/after blobs, complete path ledgers, correction source and realistic failure semantics, actual checked wait and reader-join rule, N089/N090 real-vs-pure evidence, Ubuntu/Windows CI run and job IDs (including failures), RustSec/security and lock identities, unchanged protected paths, branches, limitations and cleanup. Differentiate local-only assertions from independently verifiable GitHub facts. The report must not assert that WO-089 was accepted or retrospectively compliant. The report cannot include its own future commit SHA.

Commit **only** the report path on `main`, with sole parent the successful ACTIVE issuance commit, by ordinary fast-forward push. Require a fresh original push-triggered `rust-verify` CI at the exact report-head SHA, Ubuntu and Windows each success, 33 configured steps and only 9/14 host-inapplicable skips; inspect real logs and report job IDs in the final handoff. This mainline run is report integrity and baseline nonregression, NOT N090 functional proof (the code remains on the technical branch). Verify no N090 branch-only positive tests or new markers are attributed to `main`.

Final `main` diff from baseline `29ccec0b...` must contain **exactly two** permanent paths: Owner-installed ACTIVE and canonical report; **exactly two** mainline commits. The new technical branch has only the two authorized files. Keep all eleven existing branch refs intact and add only the one new technical branch. If a required gate cannot be truthfully passed, STOP and present a blocked status; do not manufacture a green handoff or authorize a third mainline implementation commit.

## 8. Owner scratch preservation, cleanup and STOP policy

Inventory ignored material before any task scratch. After the green issuance gate, any scratch for this order must be under a fresh owned path such as `.temp/solo-wo-090-run/`, created only if it did not exist. Delete only precisely known order-owned disposable scratch after all jobs using it have ended. No broad directory clean, history rewrite, unknown-file deletion, or local ignored-material mutation. Preserve `.temp/solo-wo-073-wip/` (agent-reported 4 files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/` and historical Cline logs; report actual local inventory without pretending GitHub can verify it. Verify exactly one worktree and remote, no stash, clean final tracked state, exactly four WIP files, and `HEAD == main == origin/main` at the report SHA.

**Hard STOP triggers:** baseline/blob/ref mismatch; existing `solo-wo-090-r01`; technical edit/import before issuance gate green; unrecognized local dirty material; use of destructive checkout/reset/clean; unauthorized permanent path; source candidate hash mismatch; missing/late readiness accepted as success; native query before validation; `Child::wait()` ignored or its failure masked; reader joined before a confirmed leader reap; actual unreaped child or failed lifecycle; unbounded blocking; native/unsafe/FFI or group-signal/descendant operation; test skipped/weakened; mandatory unresolved CI failure; main code promotion; branch history rewrite; unsupported containment/publication claim. STOP at the boundary; do not self-authorize a workaround, blanket cleanup or another order.

## 9. Final handoff and acceptance boundary

Return one canonical report URL at its **exact final SHA** and the complete actual commit SHA/tree/parent and file-blob identities for: baseline main, ACTIVE-only issuance, candidate seed on `solo-wo-090-r01`, every correction, final branch, and report-only main. Provide every CI run/job ID and outcome, exact real Linux readiness observation and regressions, Windows negatives, deterministic failure-to-reap decision proof, changed-path ledgers, preservation and cleanup facts, and limitations. Preserve stopped `solo-wo-089-r01` unchanged and acknowledge its STOP disposition permanently.

If and only if all gates pass, the Implementation Agent may **claim for review**, not self-accept: `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-PROVEN`, isolated branch `solo-wo-090-r01` only. Await the Chief Master Architect's independent **ACCEPT / REVISE / STOP**. Do not begin integration, another order or any release. `R03.C3`, `R03.C` and Stage 03 remain OPEN.
