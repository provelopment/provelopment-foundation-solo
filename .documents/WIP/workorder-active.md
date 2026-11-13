# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-088 R01**  
**Title:** R03.C3 — integrate the independently accepted Linux live-child session attestation via exact three-blob promotion to `main`  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner → Implementation Agent activation:** `SOLO-WO-088 R01`  
**Authority:** This document becomes executable only after the Owner installs its complete contents at `.documents/WIP/workorder-active.md` in the sole checkout and separately sends the matching activation message. The Chief Master Architect has **read-only GitHub authority**. Only the Owner-activated Implementation Agent may perform the bounded local changes and GitHub writes specified here. Neither a previous acceptance nor possession of this file authorizes any unlisted action.

## 1. Decision, objective, and hard boundary

Independent Chief Master Architect disposition for `SOLO-WO-087 R01`: **ACCEPT — `LINUX-LIVE-CHILD-SESSION-ATTESTATION-PROVEN`**, on the isolated branch only, at immutable commit **`8250f742f533d99cea1c43739ea351095d4c9d73`**, tree **`bb249a4231b85e50d8a3ce9096721b4a8998cd09`**. That branch's final `rust-verify` workflow-dispatch run **`43383835725`**, Ubuntu job `131747551543` and Windows job `131747551816`, passed both hosts. The canonical WO-087 report is on `main` at **`ce4448a4bb3b680aa0d275cd802df3555ffb666e`**, and its push CI run `43384468566` passed both hosts. Crucially, the report-head CI did **not** execute the new N087 implementation because the three code files remained branch-only.

**Authorize integration ONLY.** Copy the accepted Git blobs of exactly three files from that immutable branch commit to `main`, with **zero implementation edits, reformatting or regeneration**. Demand staged/working-tree Git-object equality to the accepted blobs, one three-file integration commit, fresh complete two-host CI **on `main` at that exact integration SHA**, then one separate report-only main commit and a second fresh two-host CI at its exact SHA. The proof branch and all prior branch refs must remain untouched. This is a path-scoped promotion, **not** a branch merge or cherry-pick.

**Permitted success statement only after every gate:** `LINUX-LIVE-CHILD-SESSION-ATTESTATION-INTEGRATED`. This means that `main` has the accepted bounded read-only live-child attestation and it was independently re-executed on the Linux CI runner. It **does not** establish process-group signalling authority, safe group cleanup, descendant control, persistence of PID/session identity, PID-reuse race exclusion, race-free synchronization, host containment, arbitrary process supervision or AVIF/media correctness. The earlier N087 implementation deliberately performs one live query without polling/retry or a readiness synchronization barrier; preserve that limitation rather than claiming to have fixed it. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** and all media-correctness, containment and publication-authority flags remain **false**.

**Hard STOP/REVISE rules.** No new implementation, fixes, tests, additional scripts, packages, branches, OS calls, safety allowances, guard changes, workflow changes, process signals, child/descendant functionality, release, publication or deployment. A mismatch, a mandatory CI failure, or remote/main drift is not permission to correct code under this order. Do not amend, rewrite history, reset, force-push, use a second worktree/clone, wipe ignored material or delete an inherited file. No subsequent order may start under this authorization.

## 2. Exact mandatory preflight — STOP on mismatch

Use **only** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`. Independently inspect, not merely infer from a prior report:

- `git rev-parse --show-toplevel` identifies the sole intended checkout; `git worktree list` shows exactly one; `git remote -v` names one remote `origin`, with identical fetch/push URL `https://github.com/provelopment/provelopment-foundation-solo.git`.
- Before staging, `HEAD`, local `main` and `origin/main` agree at the exact baseline below; the index is clean; the only working-tree modification is the **Owner-installed replacement ACTIVE** file; no unknown/untracked files or extra dirty path. Exactly four `.documents/WIP/` files exist: `workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`.
- The installed ACTIVE declares `SOLO-WO-088 R01` and matches the Owner's complete issued file; do not edit or synthesize it. Check historical branches, the accepted proof branch, and all protected anchor blobs using full values, including the remotely fetched values.
- Inventory ignored scratch **without running, staging or modifying it**. Preserve `.temp/solo-wo-073-wip/` (historically four files, 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/` and unrelated prior-session/Cline logs. Any Owner-created data is off-limits. Record any discrepancy before action; do not sanitize it away.

| Pinned source-of-truth anchor | Full required identity at baseline |
| --- | --- |
| `HEAD == main == origin/main` | `ce4448a4bb3b680aa0d275cd802df3555ffb666e` |
| Baseline commit tree | `24144bf4daf646d50b2472085ae761bce863f628` |
| Existing canonical `.documents/Reports/execution-report.md` blob | `b4ebacd1155a3cc5ef32447fd4dd09757191b04d` |
| Previously tracked `.documents/WIP/workorder-active.md` blob | `90eff7424785a3ad3226d88f65bff1a3f58acbc5` |
| `.documents/Architecture/ARCHITECTURE.md` blob | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK blobs | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.github/workflows/rust-verify.yml` blob | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` (both literal `edges=41` checks intact) |
| `Cargo.lock` blob | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` blob | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` blob | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| `crates/solo-cli/src/main.rs` blob | `b3c89ad0ce256f69e5763855559d20757554c461` |
| Existing native boundary `media_worker_host_native.rs` blob | `d10151e40e807c9831491ba0da0eb44d08a81589` |
| Existing safe façade `media_worker_host.rs` blob | `88d94ecc81ccdd912792bad8721d5950d3fb217c` |
| Existing host test `media_worker_host_smoke.rs` blob | `038c3193e96633f34e0d34baffab7d04b348573d` |
| `crates/solo-cli/tests/unsafe_boundary.rs` blob | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| Architecture graph checker blob | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| Workspace graph checker blob | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Accepted proof branch `solo-wo-087-r01` tip/tree | `8250f742f533d99cea1c43739ea351095d4c9d73` / `bb249a4231b85e50d8a3ce9096721b4a8998cd09` |
| Accepted proof branch issuance parent | `e829ce57bc4f2e9f796b5969f239570c3e7d54d2` |

Preserve exact remote heads of all prior branches, checked again at end: `solo-wo-085-r01` `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`; `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b`; `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`. No ref may move except `main` by the three commits expressly prescribed below.

## 3. Issuance: ACTIVE-only `main` commit and two-host gate

After §2 succeeds, stage **only** `.documents/WIP/workorder-active.md`; commit the complete Owner-installed `SOLO-WO-088 R01` without changes to its contents; verify that the one commit has exactly one parent (baseline), exactly one modified path (ACTIVE), and unchanged report and all source blobs. Push to `origin/main` by ordinary fast-forward only. Verify the new issuance SHA, tree SHA, sole parent, exact ACTIVE Git blob, changed-path and numstat ledgers against the remote GitHub commit and contents. Require local `HEAD == main == origin/main` and clean index/worktree before the next step.

**Hard issuance CI gate:** new `push`-triggered `rust-verify`, `head_branch=main`, `head_sha=<exact issuance SHA>`, overall `conclusion=success`, individual `verify (ubuntu-24.04)` and `verify (windows-2025)` jobs both `success` with the unchanged 33-step configuration, zero actual failed steps, only 9 Ubuntu / 14 Windows host-inapplicable skips. Inspect the real logs and record run/job IDs. An incomplete/failed gate is STOP: do not import source.

## 4. Exactly three accepted Git blobs — one path-scoped promotion

The only authorized technical operation copies the three immutable Git objects from accepted branch `8250f742f533d99cea1c43739ea351095d4c9d73` into the index and worktree of clean, verified `main` at the issuance commit:

| Full authorized path | Baseline blob on `main` | Required accepted branch / index / new main blob | Diff from issuance |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` | `1e698245493f6d10ebb9636282698a797a242865` | M, +93 / -4 |
| `crates/solo-adapters/src/media_worker_host.rs` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | M, +883 / -5 |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `038c3193e96633f34e0d34baffab7d04b348573d` | `a97c09954e96fbddd6380fb301be182584c183ea` | M, +242 / -0 |

**The only allowed staging mechanism** (a *single line* in PowerShell; no POSIX backslash continuation, wildcard, `git merge`, `git cherry-pick`, checkout of whole branch, patch replay or other file import):

```text
git restore --source=8250f742f533d99cea1c43739ea351095d4c9d73 --staged --worktree -- crates/solo-adapters/src/media_worker_host_native.rs crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

**All five proofs required BEFORE any integration commit:**

1. `git diff --cached --name-status` shows **exactly three `M` rows**, corresponding to the three literal paths above; `git diff --cached --numstat` is exactly `93 4`, `883 5`, `242 0` at those paths; `git diff --cached --check` has no output. No surprise staging or unstaged changes.
2. For every path, `git rev-parse :<literal-path>` equals the required full accepted Git blob **and** `git rev-parse 8250f742f533d99cea1c43739ea351095d4c9d73:<literal-path>`. Record all three independently.
3. For every path, `git hash-object <literal-path>` (Git-filter-aware, no `-w`) equals the accepted and index object. On this Owner Windows worktree `core.autocrlf=true` and `* text=auto` can create physical CRLF worktree bytes: optional `git hash-object --no-filters <path>` and byte counts may differ. That is **not** a Git blob mismatch; disclose it without manually rewriting, normalizing, reverting or formatting any imported source. The authoritative committed Git objects must be exact.
4. `git ls-files -o --exclude-standard` is empty, and `git status --porcelain` contains exactly the three authorized staged `M` entries, with no extra tracked/untracked/unstaged path. The accepted branch remains unmoved and no branch other than `main` has been altered.
5. The staged path ledger vs the issuance SHA is precisely those three paths, no report/WIP plan/CLI root/Cargo/workflow/guard change; all protected file blobs in §2 are unchanged, except the three expressly imported paths. In particular, the fixed pre-exec `setsid` and both `setrlimit` calls have **not** been edited; the imported native seam has only the accepted read-only `getpgid` and `getsid` additions.

Commit the **three** files together as exactly one integration commit directly on `main`, with exactly the issuance SHA as parent. Ordinary fast-forward push only. Independently verify the remote SHA, tree, parent, exact three-file ledger, each full accepted blob and unchanged guards/workflow/lockfile. Prove `HEAD == main == origin/main`, clean index and worktree. Never import branch ACTIVE or branch report. **No source edit is authorized if a preflight or CI gate fails.**

## 5. Integration CI: prove N087 on the actual `main` implementation SHA

Require a new `push`-triggered complete original `.github/workflows/rust-verify.yml` run with `head_branch=main`, exact `head_sha=<integration SHA>`, `conclusion=success`, both named host jobs `success`, 33 configured steps each, zero failed applicable steps, no unapproved skips. Record run ID, both job IDs, host, event, SHA, and relevant log excerpts. The expected 9/14 skips must be host-inapplicable, **not** proof cases skipped. Review detailed logs, not a green summary alone.

**Fresh Ubuntu `N087` evidence** must come from this integration job, not `43383835725` or an older branch report:

- `n087_01_the_parent_independently_attests_a_live_child_session ... ok`; capture its actual `n087_session=media_host_session=live_attested ...` line, requiring exact soft/hard AS values `536870912/536870912`, exact CPU values `10/10`, real positive `pid`, `pid==pgid==sid==observed_pgid==observed_sid`, `reaped=1`, normal child exit, and strictly `0 < wall_ms < 15000`. The child uses one fixed four-second sleep; **do not** assert an exact wall time or demand an arbitrary unique PID versus previous runs.
- Confirm that the parent independently queried its owned live child's group and session and bracketed the query by the two `try_wait` observations; verify the copied source and neutral validator uphold this protocol. The parent's observation is a **single point-in-time check**; no retry, polling, synchronization repair, persistent identity or race-free authority may be claimed.
- `n087_02_a_direct_session_child_refuses_before_its_sleep ... ok`, with the child's actual refusal exit and empty stdout/stderr; `n087_03` pure and real CLI token/schema/lifecycle refusals `ok`.
- The inherited real Linux tests `N076-01..03`, `N079` positive/direct negative, `N081` kill/reap/direct negative, `N083` one over-ceiling reservation refusal/direct negative, and `N085` kernel CPU termination/direct negative all remain green; do not modify them. In particular, the N085 signal must still be from a child status without parent killing it.

**Windows:** `n087_04_both_session_modes_are_unsupported_before_any_child_exists ... ok` with both private modes exiting code `3` and exact fixed `HOST_UNSUPPORTED`, empty success stdout, no child or session query; N087 pure neutral validator and token tests pass. Confirm no Linux-only `n087_01`, `n087_02`, `n087_session=` and no Linux-only native readback/test execution in Windows logs.

**Both hosts:** original pinned Rust `1.99.1`, fmt, strict Clippy `-D warnings`, locked all-target tests, all original `architecture=legal packages=4 targets=33 files=60 edges=41` checks including two literal `edges=41` workflow pins; `self-check=passed cases=29`; real-tree negative checker/bypass probes; `cargo-deny 0.20.2` and freshly fetched RustSec identity, advisory/bans/licences/sources checks; release build and `release_smoke=verified accepted_exit=0 refused_exit=2`; Ubuntu forbidden-import restore and reproducibility (two clean release builds and identical bytes). A `test result: FAILED` in the deliberately injected CI tripwire is not a real job failure if and only if the tripwire itself proves rejection and the overall job succeeds. Record actual gate observations, host-skips and the Git blob of `Cargo.lock` (checkout raw SHA-256 can differ across line-ending conventions).

**Non-negotiable:** Any mandatory red/unverified integration gate prevents an `INTEGRATED` claim. Do not edit, reformat or silently revert the already-imported accepted source. Stop the technical work; publish a truthful BLOCKED/UNVERIFIED canonical report if necessary, and request a separate new Owner authorization for a correction.

## 6. Canonical report: separate report-only mainline commit, then two-host CI

Only after integration gates pass, replace the single canonical `.documents/Reports/execution-report.md` with a complete factual WO-088 execution report. Include the preflight full anchors, Owner activation, exact issuance/integration SHAs, trees and parents, complete path/numstat/blobs, five promotion proofs, CRLF/filter behavior if present, full run/job IDs, raw Linux `N087` evidence with actual observed IDs/timing, Windows negatives, N076/N079/N081/N083/N085 regressions, security/architecture gates, explicit CI skip accounting, all limitations, preserved branch refs, local-worktree cleanup and any unexpected or failed runs (even if corrected or unrelated). Distinguish verified remote GitHub facts from observations the Agent can verify only in the Owner-local checkout. Do not invent the report commit's own future SHA inside the report; supply that SHA/tree/blob, report-head run and jobs in the final Agent handoff after creation.

Stage only the canonical report. Commit one **report-only** commit directly on `main`, whose sole parent must be the successful integration SHA. Verify exactly one modified path, report blob, tree, parent and ordinary fast-forward push from the remote. No report archive/copy, extra WIP file or implementation change is permitted.

**Final acceptance CI gate:** fresh `push`-triggered `rust-verify` at the **exact final report HEAD** with both Ubuntu and Windows jobs successful, 33 configured steps each, zero failed applicable steps and only documented host-inapplicable skips; independently inspect logs and require **fresh real N087 Ubuntu evidence** plus all N076/N079/N081/N083/N085 regressions, Windows N087 refusal and the full architecture/security/release-smoke/reproducibility gates. Confirm `HEAD == main == origin/main` equals report SHA and no code was changed by the report commit.

**Authorized permanent tracked-file ledger across the whole order:** exactly **five distinct paths** in exactly **three mainline commits**: (1) ACTIVE only; (2) precisely the three accepted implementation paths; (3) canonical report only. No other tracked path, commit, branch or parent is allowed.

## 7. Preservation, cleanup and authority limits

Keep one checkout, one worktree, one `origin`, the four-file WIP structure, all historical/proof branches at §2 full SHAs and all inherited ignored material. Never run or import anything from `.temp/solo-wo-073-wip/` or other inherited scratch, never delete unknown/Owner/prior-session files or an old `proceed-while-running-*.log`, and never use `git clean`, destructive reset, force push, stash, alternate clone or a second worktree to hide a mismatch. Create any disposable evidence only under an explicitly task-owned directory (e.g. `.temp/solo-wo-088-run/`) and delete **only** that directory and files whose ownership was established; inspect it directly, not only through `git status`. Preserve `target/` and all unrelated material. The local environment's lack of `cargo`/Rust toolchain or linker is not permission to bypass or weaken remote CI; report the actual tool availability honestly.

**No new group kill, no descendants, no pidfd, no `killpg`, no negative PID signal, no process-tree sweep, no Windows Job Objects, no sandbox, no content acquisition, no media decoding, no AVIF validation, no publication, release, tag, deployment or PR.** Success remains only one trusted descendant-free child, fixed session and group identity **observed at one point**, followed by normal exit/reap. Nonpersistent identity, post-check reuse risks, and unsynchronized one-query scheduling behavior remain explicitly unresolved. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** with flags `false`.

## 8. Handoff and mandatory STOP behavior

The Implementation Agent must return exact:

- issuance, integration and report commits, trees, parents, path ledgers, three accepted full Git blobs and final report/ACTIVE blobs; remote ref identities, branch preservation and Git-object/CRLF proofs;
- three distinct `rust-verify` run IDs, six job IDs, exact tested SHAs/events, 33-step success/skip/failure inventory per host, kernel N087 observation, Windows denial, N076/N079/N081/N083/N085 regression and downstream gate excerpts;
- actual clean Git status, one-worktree/one-origin/four-WIP check and owned-scratch cleanup, inherited 4-file/46,062-byte research preservation, local-vs-CI evidence distinction and remaining nonclaims.

**STOP on any mismatch, unapproved change, unverified CI, wrong parent, extra path, wrong Git blob, changed proof/historical branch or unsafe operation.** If a failure occurs after a legitimate integration push, do **not** rewrite the branch or fix implementation under this order: preserve the factual state, publish only an authorized truthful report if possible, and request the Owner/Architect to decide the next bounded recovery. The Implementation Agent may claim `LINUX-LIVE-CHILD-SESSION-ATTESTATION-INTEGRATED` **only after** all gates pass but may **never** self-ACCEPT. Do not begin another order.
