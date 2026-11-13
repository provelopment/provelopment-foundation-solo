# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-086 R01**  
**Title:** R03.C3 — integrate the independently accepted Linux `RLIMIT_CPU` kernel-termination proof by exact two-blob promotion to `main`  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner → Implementation Agent activation:** `SOLO-WO-086 R01`  
**Authority:** This order becomes ACTIVE only after the Owner installs this complete file as `.documents/WIP/workorder-active.md` in the sole checkout and separately activates the Implementation Agent using the matching order ID. The Chief Master Architect is permanently **GitHub READ-ONLY**; the Owner-activated Implementation Agent alone performs the permitted commits, pushes, CI dispatch if needed and canonical reporting. An independent ACCEPT does not by itself confer merge, publication or release permission.

## 1. Owner decision and closed objective

`SOLO-WO-085 R01` has been independently **ACCEPTED** as `LINUX-RLIMIT-CPU-KERNEL-TERMINATION-PROVEN` at preserved isolated branch `solo-wo-085-r01`, exact commit `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`, tree `d796ba2d4fc5aa35b618f029a96afe6e30075a2a`. That branch's successful two-host workflow_dispatch run `43363659723` at the exact code SHA proved one trusted, descendant-free Linux child reached the 10-second soft/hard `RLIMIT_CPU` limit under the unchanged pre-exec installer. The child verified its own four limit halves and `pgid == pid` before its single-threaded, allocation-free CPU loop; the parent observed real `signal=9`, `reaped=1`, `parent_killed=0`, `wall_ms=10008`, and **did not cause** that termination. Windows refused both CPU internal modes before creating a child. Earlier branch run `43362920499` had a disclosed Windows Clippy `dead_code` failure, fixed by a single Linux-only attribute inside the accepted façade; the final exact branch run was green on both hosts. The `main` report-head run `43364662038` was also green, but `main` has **none** of WO-085's technical source changes.

**Owner authorizes INTEGRATION ONLY:** transplant exactly the TWO Git blobs below from that immutable accepted branch SHA to `main` without any modification. Prove their Git object identities, obtain a fresh full two-host `rust-verify` run triggered by the **main integration commit**, then overwrite the one canonical execution report in a separate **report-only** main commit, and obtain a second fresh two-host run at that report HEAD. Do **not** create a technical branch; the accepted proof branch remains untouched and unmerged. No new functionality, source revision, refactoring, lint relaxation, feature, guard adjustment, workflow change, dependency change, media processing or publication is authorized.

**Allowed outcome:** `LINUX-RLIMIT-CPU-KERNEL-TERMINATION-INTEGRATED` — only after every applicable gate below passes at the exact integrated SHA and both report/CI checkpoints. It means one Linux kernel-enforced CPU-time limit for one trusted, descendant-free child is integrated, **not** whole host containment. It does not prove aggregate CPU budgets, hostile-child resistance, descendant or group termination, exhaustion across a tree, a Windows Job Object, RSS/aggregate memory confinement, executable identity, AVIF correctness, media publication or release. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and public media/correctness/publication authority flags remain **false**.

**STOP rules:** Do not treat a missing gate, changed SHA, red CI, non-ledger path, unclean preflight, changed branch, changed blob, unknown Owner data, source mismatch, or worktree encoding ambiguity as an invitation to improvise or patch. STOP with precise evidence and a canonical report. A failing run on an unchanged accepted blob may require a **new** Owner authorization; this order does not permit corrective code edits. The Implementation Agent never self-ACCEPTs and never begins a new order.

## 2. Exact immutable preflight — STOP on mismatch

In the *one existing checkout only*, independently verify from local Git and GitHub (not the agent's earlier report) `HEAD == main == origin/main` at the full baseline SHA; the exact tree and pinned blobs; the existing accepted branch at the same SHA both locally and remotely; the actual `origin` URL; one worktree; exactly the four WIP filenames; and a clean index/worktree **except only the new Owner-installed ACTIVE file**. No unknown tracked, staged or untracked path; no second checkout, clone, linked worktree, stash or shadow repository. Verify prior integration and report CI history, without counting it as new WO-086 evidence. If `main` advanced unexpectedly or any anchor differs, STOP, do not rebase or force-push.

| Anchor | Exact required value |
| --- | --- |
| Baseline `HEAD == main == origin/main` | `09aab85a49a8d44ff72e39c90ae9484a9818c0f8` |
| Baseline `HEAD^{tree}` | `d092f729f80a2833e5f6054423b6f02a9ecf4a2d` |
| Existing canonical `.documents/Reports/execution-report.md` Git blob | `6a510196334cc3a74f5f8532ea77f6fc8925c42c` |
| Previously tracked `.documents/WIP/workorder-active.md` Git blob | `d3d1096fdfb21d1a25dca00d0200d9bfb1594ad5` |
| Architecture Charter `.documents/Architecture/ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| WIP NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1`, both literal `edges=41` checks intact |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` |
| **Main** façade `crates/solo-adapters/src/media_worker_host.rs` | `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` |
| **Main** integration test `crates/solo-cli/tests/media_worker_host_smoke.rs` | `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` |
| Unsafe guard / architecture graph / workspace graph | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` / `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` / `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Accepted source branch `solo-wo-085-r01` | `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`, tree `d796ba2d4fc5aa35b618f029a96afe6e30075a2a` |
| Branch issuance ancestor and first revision | `bb1da94cc7d84d0c9d27f105839baac361f76bcf` / `a763655f3c137c3dd557552a3d8e34e84e4afacf` |
| Branch's own ACTIVE and branch's own prior report | ACTIVE `d3d1096fdfb21d1a25dca00d0200d9bfb1594ad5`, prior report remains on branch; **neither** may be imported |
| Accepted branch façade / test Git blobs | `88d94ecc81ccdd912792bad8721d5950d3fb217c` / `038c3193e96633f34e0d34baffab7d04b348573d` |
| Preserved proof branches | `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Older historical branches | `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Previous CI provenance (history only) | proof `43363659723`; prior main report `43364662038`, both green, **not reused** |

Verify source ancestry: compare `bb1da94cc7d84d0c9d27f105839baac361f76bcf...5bf9082bd44e6ecaedd783e2a93e6260a87073d3` is exactly **two modified implementation paths**, with final cumulative numstat `+928/-7` and `+239/-2`. Check source branch `main.rs`, native module, lock, workflow, guards and WIP/report identities; import neither branch ACTIVE nor branch report. Inventory ignored/inherited `.temp` material by direct inspection. Preserve `.temp/solo-wo-073-wip/` (historically four uncompiled research files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/` and existing `target/` without interpreting them as approved source. Never delete Owner-owned or unfamiliar data to manufacture a clean state.

## 3. ACTIVE-only issuance on `main` — require both-host gate

After all §2 checks succeed, stage **only** `.documents/WIP/workorder-active.md`, commit the Owner-installed file in full, unchanged, with exactly the baseline commit as parent; no source or report file in this commit. Verify its **one-path** ledger, exact commit SHA, parent, tree and new ACTIVE Git blob both locally and by GitHub readback. Push by ordinary fast-forward to `origin/main`, never force. Require a **new** push-triggered `rust-verify` run on the issuance SHA, with `head_branch=main`, `head_sha=issuance SHA`, overall `success`, and BOTH `verify (ubuntu-24.04)` and `verify (windows-2025)` successful. Original workflow has 33 steps per job, normally nine host-inapplicable skips on Ubuntu and fourteen on Windows, zero failed steps. Record real run/job IDs and verified outcomes. No implementation staging is allowed until this issuance CI gate is green and the checkout is again clean at the issuance SHA.

## 4. Precisely TWO authorized implementation paths — accepted Git blobs only

Only these two tracked source paths may change, **together in one integration commit** from the issuance SHA. This is a byte-for-byte *Git object* promotion, not fresh editing:

| Authorized path | Before on main | Required source-branch / index / integrated Git blob | Cumulative diff |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `M`, +928 / -7 |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` | `038c3193e96633f34e0d34baffab7d04b348573d` | `M`, +239 / -2 |

**Only authorized source transfer mechanism:** on clean `main` at the successful issuance SHA, run **one** literal path-scoped restore (no wildcard, no whole-tree checkout, no cherry-pick, no merge, no rebase):

```text
git restore --source=5bf9082bd44e6ecaedd783e2a93e6260a87073d3 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

Five mandatory precommit checks, record actual full outputs/identities:

1. `git diff --cached --name-status` is **exactly two `M` rows** for these two paths; `git diff --cached --numstat` is exactly `928 7` and `239 2`; `git diff --cached --check` is clean. No additional file, other status or untracked input.
2. Each `git rev-parse :<literal-path>` equals its required source-branch blob at `5bf9082b...:<literal-path>` and the exact required SHA above. Check separately for both paths; a Git object mismatch is STOP.
3. Each **filter-aware** `git hash-object <literal-path>` equals its index and accepted branch blob; do not modify the file to obtain this. The Owner Windows checkout previously used `core.autocrlf=true` and `* text=auto`: physical CRLF worktree bytes may differ from the canonical LF Git blob without representing a source change. If observed, report the raw `git hash-object --no-filters` hashes and line-ending facts separately, but **judge promoted source identity by the index, canonical Git blob and filter-aware hash**. Do not silently normalize/rewrite, and STOP if working-tree conversion does not round-trip to the accepted Git blob.
4. `git ls-files -o --exclude-standard` is empty; `git status --porcelain` shows only the two authorized staged `M` entries and no unstaged or third-path changes. Preserve ignored inherited scratch.
5. `git diff --name-only <issuance-SHA> --cached` contains only those paths; verify the workflow, lock, manifest, native file, CLI root, `lib.rs`, guards, three WIP plans, Charter, SOPs and all branches remain unchanged. The source branch's own WIP or report is **never** staged.

Commit exactly these TWO files together with parent equal to the issuance commit, verify SHA/tree/parent, GitHub's exact changed-path/numstat ledger and both blob IDs; push by ordinary fast-forward only. The accepted technical branch remains at its original SHA both locally and on `origin`, never moved, merged, rebased or cherry-picked as a whole. Do not create a new branch or change branch protection/settings.

**Complete permanent edit ledger for WO-086:** (1) ACTIVE only in issuance main commit; (2) exactly two implementation paths in one main integration commit; (3) `.documents/Reports/execution-report.md` only in final report commit. **Exactly four distinct tracked paths and three ordered mainline commits.** No other tracked edit: no workflows, Cargo manifests/lock, native `unsafe` module, CLI root, libraries, guards, other tests, plans, Charter/SOP, README, media inputs, release files, or fixture changes.

## 5. New full Ubuntu + Windows integration CI at the exact `main` integration SHA

Require a **fresh** push-triggered original `rust-verify` workflow with `head_sha=<integration SHA>`, `head_branch=main`, `event=push`, overall `success`, and both `verify (ubuntu-24.04)` and `verify (windows-2025)` `success`. Record run ID, both job IDs, original 33-step structure, actual success/skips and zero failures. Inspect job logs, not just badges. Do not reuse branch, issuance or earlier history.

Mandatory **real Ubuntu** checks:

- **N085-01:** the test `n085_01_the_kernel_cpu_limit_terminates_the_child ... ok` genuinely ran at this integrated SHA, and the CI log contains a fresh `n085_cpu=media_host_cpu=kernel_terminated ...` line. Require `as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10`, positive `pid`, `pgid==pid`, `signal=9` (kernel `SIGKILL`) **or** `signal=24` (`SIGXCPU`), `reaped=1`, `parent_killed=0`, and `0 < wall_ms < 60000`. Compare observations with branch evidence, **without** requiring the PID or wall time to differ — CI process IDs can repeat; the run/SHA/job and log occurrence provide freshness. Wall time is not CPU time and must never be represented as such.
- **N085-02:** direct unsupervised `cpu-child` refuses promptly before any loop with no stdout/stderr and fixed child refusal status; log `n085_direct_cpu_child=exit=...`, test `... ok`. No unsupervised CPU burn.
- **N085-03:** malformed readiness, wrong limits/group, forbidden normal exits or signals, un-reaped leader, parent-killed status, timeout and unknown internal tokens all remain typed refusals. Seven pure neutral cases pass on both hosts, plus real-binary token tests on their respective hosts.
- **N076-01..03**, **N079**, **N081** and **N083** regression suites pass, including actual benign limit readback, deadline `SIGKILL`/leader reap and one `805306368`-byte reservation denied under `536870912`-byte ceiling. Preserve positive/readback, negative and public CLI byte-exact exits/outputs. Require no media or publication implication.

Mandatory **Windows** checks: `n085_04_both_cpu_modes_are_unsupported_before_any_child_exists ... ok`; both new modes return status 3 with fixed `HOST_UNSUPPORTED`, empty stdout, prompt, with **no CPU loop or Linux child execution**. Neutral parser/validator/unit tests still execute and pass. No `n085_cpu=` or Linux-only N085-01/02 execution on Windows, and no Linux-only N076, N079, N081, N083 kernel tests execute there. Existing public CLI preflight success/refusal output and codes unchanged.

Mandatory **both-host original downstream gates:** pinned Rust `1.99.1`; `cargo fmt --all -- --check`; strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`; locked all-target unit/integration tests; real-tree `architecture=legal packages=4 targets=33 files=60 edges=41` and both unchanged literal workflow edge pins; `self-check=passed cases=29`; dynamic-load/checker-bypass refusals; `cargo-deny 0.20.2` with fresh RustSec database check and required advisory/deny negatives; injected test-failure tripwire; `release_smoke=verified accepted_exit=0 refused_exit=2`; Ubuntu forbidden import negative and exact restoration; Ubuntu two clean byte-identical release builds (`reproducibility=verified`). Verify exact lock Git blob; raw Ubuntu/Windows checked-out line endings may differ. These are **CI test-build** gates, not product release permissions.

**CI failure policy:** An actual integration CI failure, undocumented skip, missing N085 kernel evidence, blob drift or unapproved source change is STOP/REVISE. No code fixes, workflow edits, assertion weakening, third path, direct branch modification or backport from elsewhere under WO-086. Report the failure accurately rather than declaring integrated success.

## 6. Canonical report-only `main` commit and second full CI

Only after §5 is completely green, overwrite the one canonical `.documents/Reports/execution-report.md` with the complete factual WO-086 report. Never create a dated report, parallel evidence folder or fifth WIP file. **The report commit changes exactly one path**, the canonical report, with its **only parent** equal to the accepted two-file integration commit. Fast-forward push to `origin/main`; verify SHA/tree/parent/one-path ledger and final report Git blob from GitHub; verify local `HEAD == main == origin/main` and a clean working tree. Require a **fresh** push-triggered `rust-verify` run at the final report HEAD, both Ubuntu and Windows jobs successful with all original applicable steps. Inspect its logs: the real Linux N085 kernel signal/refusal must execute again, alongside N083/N081/N079/N076 regressions, and Windows refusal and cross-platform gates. Distinguish this new evidence from the integration run and from the original WO-085 branch run.

Do not embed a fictional self-commit SHA in the report before the commit exists: put objective identities available at report assembly, and return the final self SHA, tree, report blob, run/job IDs in the final agent handoff. If the final report-head CI is red or absent, do not claim completion; publish a truthful status without unauthorized correction.

## 7. Preservation, cleanup, authority and negative claims

Keep one checkout, worktree and `origin` throughout; no clone, second worktree, shadow checkout or alternate remote. Preserve proof branches `085`, `083`, `081`, `079`, `076` and older `061`, `069`, `070` at their exact §2 SHAs and preserve their history. No PR, merge of whole branch, cherry-pick, tag, release, deployment, production media processing, AVIF decoder or public publication. Do not modify inherited `.temp` content or `target/`; any task-created disposable scratch may live only in approved locations, be explicitly inventoried and removed by safe, targeted deletion. Preserve Owner/unknown files, especially `.temp/solo-wo-073-wip/` four research files / 46,062 bytes, uncompiled and untrusted. Report direct cleanup evidence and any limits; clean Git status alone does not prove ignored scratch cleanup.

A passing CPU signal proves kernel enforcement for one trusted single-threaded child that verified its fixed ceilings, **not** protection from arbitrary workloads. No process-tree/descendant sweep, group signal or hostile-child control; no aggregate CPU or RSS budget; no Windows Job Object; no immutable executable identity, sandbox, network/credential confinement, media correctness/publication, release or deployment. `R03.C3`, `R03.C` and Stage 03 stay OPEN. No expanded authority is inferred from the output marker.

## 8. Final required implementation-agent report and handoff

The canonical report must give independently auditable evidence, not just `PASS` labels: exact baseline, issuance, integration, report and accepted branch SHAs/trees/parents; exact current and accepted Git blobs for all ledger paths; full `git diff --name-status` and `--numstat` at relevant bases; five staged object proofs and any CRLF/filter observations; unchanged protected path identities; every original/new CI run/job ID, event, SHA, host, steps, outcome and relevant log observations; actual N085 child PID/PGID/signal/parent_killed/reaped/elapsed and the separate N083/N081/N079 kernels; all Windows negatives and architecture/security/reproducibility gates; branches, one-checkout/WIP evidence, report-only commit and final cleanup. Preserve any real red iteration disclosures. State no local linker if still true. Report **PASS / FAIL / UNVERIFIED / NOT APPLICABLE** for each relevant gate, and outcome `LINUX-RLIMIT-CPU-KERNEL-TERMINATION-INTEGRATED` only when all applicable mandatory gates pass.

**Implementation Agent must stop after report and handoff.** The Chief Master Architect will independently issue **ACCEPT / REVISE / STOP** based on the exact GitHub revisions and runner logs. Acceptance does not confer release, product publication, deployment, future scope or permission to start WO-087.

## 9. Gate index

| Gate | Required evidence |
| --- | --- |
| `S086-01` | Exact preflight baseline/tree/blobs/branch, one checkout/origin/four WIP, clean except Owner ACTIVE, inherited scratch preserved |
| `S086-02` | ACTIVE-only main issuance, exact parent and GitHub readback, fresh two-host issuance CI |
| `S086-03` | Two literal path-scoped restore, all five staged proofs, object identity and CRLF accounting, no third path |
| `S086-04` | Two-path integration commit on main with parent = issuance; blob/readback exact; accepted branch unmoved |
| `S086-05` | Full exact-SHA integration CI both hosts; real N085 kernel-not-parent termination, leader reaped, direct refusal |
| `S086-06` | Windows N085 fail-closed and pure validators; N076/N079/N081/N083 regressions and unchanged CLI bytes |
| `S086-07` | Original architecture/41 edges, self-check/29, security, fmt, Clippy, locked tests, tripwires, release smoke, restoration, reproducibility |
| `S086-08` | Report-only main commit with exact parent and remote readback, new report-head two-host CI and fresh N085 evidence |
| `S086-09` | Preservation, truthful cleanup, only four permanent paths/three main commits, precise scope/remaining OPEN gates, no self-ACCEPT |

**STOP on unapproved work.** Follow the binding Charter and Architect SOP, particularly the sole version-control procedure §6B and the canonical reporting/independent-review requirements. No narrower order provision weakens those rules.
