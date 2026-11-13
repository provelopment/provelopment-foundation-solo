# SOLO-WO-091 R01 — INTEGRATE: Corrected Linux Session Readiness and Reap-Fail-Closed Proof

**Authority:** Owner-issued work order, effective **only** after the Sole Owner has installed this entire file, unchanged, as `.documents/WIP/workorder-active.md` in the one approved checkout and separately activated the matching token `SOLO-WO-091 R01`. The existence of this file, an Architect ACCEPT, or an agent's own initiative is **not** activation. This is one bounded **integration-only** order, not authorization to implement additional code.

**Chief Master Architect's independent starting disposition:** `SOLO-WO-090 R01` = **ACCEPT**, outcome `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-PROVEN` on isolated technical branch `solo-wo-090-r01` at `0eb3fcf645851ae752bd59e12fe32b69604197d2`. This acceptance applies to the new corrective WO-090 only. **`SOLO-WO-089 R01` remains permanently STOPPED, NOT ACCEPTED, NOT retrospectively compliant.** Its historical sequencing breach and its unchecked early-refusal reap remain part of the record. Do not rewrite, delete or rehabilitate its report/branch.

**One authorized objective:** Promote exactly two accepted Git **blobs** from the frozen WO-090 branch into `main` with no source edit, reformat, normalization, patch replay, merge, cherry-pick or test modification. Prove source identity at the Git object level, require the complete original two-host CI suite at (1) ACTIVE-only issuance, (2) the exact two-file integration commit and (3) the separate canonical report-only main commit. Preserve every existing branch and Owner artifact. The only permitted successful agent claim is **`LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-INTEGRATED`**, submitted for independent Architect review; no self-ACCEPT.

**Nonclaims:** Not `SOLO-WO-089 ACCEPT`; not a demonstration of actual forced `Child::wait()` failure on an OS process, process-tree/descendant cleanup, persistent PID pinning, race-free identity after observation, group-signalling authority, Windows Job Object enforcement, host sandboxing, AVIF/media correctness, publication, release or deployment. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; containment, media correctness, publication and release flags remain **false**.

## 1. Repository, authority and exact immutable baseline

Approved repository: `provelopment/provelopment-foundation-solo`.

Approved and **only** checkout: `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`. Use **one** existing worktree, **one** remote `origin` with identical fetch/push URL `https://github.com/provelopment/provelopment-foundation-solo.git`, no second clone/worktree, no shadow source tree. Keep exactly four `.documents/WIP/` files: `workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`.

**Expected pristine mainline baseline, before the Owner-installed ACTIVE replacement:**

- Commit: `0fc2da6fd41699acb9add8c9369a6d83ef9c994a`.
- Tree: `5754e30acb2e435c64939e9f7596a24484f81bbe`.
- Sole parent of that report-only commit: `2d0314ab30a4bea75e979be6ab61add497689c22`.
- Current canonical report `.documents/Reports/execution-report.md` blob: `6a36487b1e668b6f53957881c0ebc8ea18bbac6e`.
- Previously tracked `.documents/WIP/workorder-active.md` blob: `4aac00be56053978fd11fbc97b975590ae642e06`.

**Pin every baseline file by full Git blob ID; do not shorten hashes in the agent's mandatory readback:**

| Path | Required baseline blob |
| --- | --- |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Architecture/ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `1e698245493f6d10ebb9636282698a797a242865` |
| `crates/solo-adapters/src/media_worker_host.rs` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `a97c09954e96fbddd6380fb301be182584c183ea` |
| `crates/solo-cli/tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| `crates/solo-cli/tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |

**Accepted and immutable promotion source** `solo-wo-090-r01`:

- Branch HEAD commit: `0eb3fcf645851ae752bd59e12fe32b69604197d2`.
- Branch tree: `71a26a58558fcbe6a18ae819c0fe9cdffa9de1d3`.
- Accepted façade blob: `a1f2bdb9960a4befc44efab746640e4d19bb624b`.
- Accepted host integration-test blob: `5444013571a71efde758422fdf9d8fada79ef53a`.
- Its latest parent: `052fa5e38de7f45535e6012f4063b6a9320cc9be`; corrective branch history must not move or be rewritten.

**STOPPED WO-089 source must remain frozen for historical evidence** at `solo-wo-089-r01` `1236915cab6e87b17ce84813430897b273fc3230`, tree `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7`. The accepted WO-090 blobs, **not** the stopped WO-089 seed (`56676fec...`/`99b470f...`), are the only promotion source.

Preserve these other exact branch refs (no branch movement, deletion, force push, rebasing or re-creation):

| Branch | Immutable head |
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

There are **12** refs at baseline: `main` plus eleven preserved branches (`061`, `069`, `070`, `076`, `079`, `081`, `083`, `085`, `087`, stopped `089`, accepted `090`). Verify all via local refs, `origin/*` and actual `git ls-remote --heads origin`; do not accidentally count the `main` remote-tracking ref as an extra branch.

## 2. Mandatory Owner-local preflight and STOP rules

Before any action: verify the actual repository root, `HEAD == main == origin/main` equals the exact baseline, exact tree and every pinned Git blob, one worktree, one origin, four WIP files, and no stash/untracked path. The **only** pending local change must be the Owner-installed, **complete and unmodified** `SOLO-WO-091 R01` replacement at `.documents/WIP/workorder-active.md`, shown as ` M` in `git status --porcelain`. A pending edit to any other path, an unexpected staged change, new branch/ref drift, unknown local material or missing pin is an immediate **STOP**, not permission to repair the repository.

Inventory ignored material directly, not via `git status`, including `.temp/solo-wo-073-wip/` (last agent-reported **four files / 46,062 bytes**), `.temp/architecture-fixtures/`, `.temp/toolchain-installer/`, `target/`, prior-session Cline logs and any other unknown files. Protect all inherited material and all historical branches. The Architect cannot independently inspect Owner-local ignored paths; report these as agent-observed only.

**No `git reset`, `git clean`, `git checkout --` used as a destructive fix, stash, rebase, force push, branch deletion, second checkout/worktree, broad temp cleanup, or overwrite of unrelated Owner data.** Do not create technical scratch, edit/import source files, prepare an implementation branch or make technical commits before the full issuance gate is green. Use task-owned temporary evidence scratch only after that gate, after verifying its path does not already exist. Do not run unknown code from inherited `.temp`.

## 3. ACTIVE-only issuance on `main` and mandatory green CI

Stage **only** `.documents/WIP/workorder-active.md`; commit the complete Owner-installed text unchanged. Sole parent = `0fc2da6fd41699acb9add8c9369a6d83ef9c994a`; commit diff contains exactly one modified path, no other change. Record issuance commit SHA, tree, parent, full ACTIVE blob and numstat, confirm by fresh independent GitHub readback and ordinary fast-forward push.

Require **fresh** original `.github/workflows/rust-verify.yml` `push` CI for **exact issuance SHA**, `head_branch=main`, `conclusion=success`. Both original jobs must separately succeed: `verify (ubuntu-24.04)` and `verify (windows-2025)`, each with **33 configured steps**, zero failed, only **9 Ubuntu / 14 Windows** host-inapplicable skips. Read real logs, not only summary. Confirm clean `HEAD == main == origin/main == <issuance>` and exact unchanged source blobs. **No promotion before this CI gate.** If it fails or is unverified, STOP; never fix source under an issuance failure.

## 4. Exactly two path-scoped accepted-blob promotions, no source changes

After issuance CI is green, while on clean `main` at the issuance SHA, perform this **one literal path-scoped** promotion command (no wildcard or whole-tree command):

```text
git restore --source=0eb3fcf645851ae752bd59e12fe32b69604197d2 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

The exact **two authorized implementation paths** are:

| Path | Main before | Required after | Exact expected numstat |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | `a1f2bdb9960a4befc44efab746640e4d19bb624b` | `491 14` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `a97c09954e96fbddd6380fb301be182584c183ea` | `5444013571a71efde758422fdf9d8fada79ef53a` | `268 0` |

**Five required independent proofs before the integration commit:**

1. `git diff --cached --name-status` returns exactly two `M` rows for these two full paths; `git diff --cached --numstat` returns exactly the two expected numstat rows; `git diff --cached --check` emits nothing. `git diff --name-status` for unstaged content is empty; no unexpected untracked files.
2. For **both paths**, the index Git object `git rev-parse :<path>` equals `git rev-parse 0eb3fcf645851ae752bd59e12fe32b69604197d2:<path>` and the pinned full accepted blob above.
3. For **both paths**, ordinary **filter-aware** `git hash-object <path>` equals the same accepted Git blob; no different accepted bytes, no generated patch, no source editing. On the Owner host with `core.autocrlf=true` and `* text=auto`, physical worktree CRLF streams may differ from canonical LF Git blobs. Report any `--no-filters` difference as physical line-ending mapping, **not** an accepted source edit. Do not toggle Git attributes or normalize/re-encode files.
4. Before commit, reverify all protected blobs (native seam `1e698245...`, CLI root, adapter root, manifest, lock, workflow, Charter, three plans, both guards, canonical report) unchanged against issuance. No unapproved native/FFI, `unsafe`, dependency, graph or process-control change.
5. Re-compare exact expected staged ledger, index object and accepted-branch object directly with GitHub. STOP on any mismatch rather than attempting to edit, normalize, reformat or regenerate the accepted files.

Commit **both files together** as exactly **one integration commit** on `main`, sole parent exactly issuance SHA. Only the two named paths may change. Push by ordinary fast-forward. Independently read integration commit SHA/tree/sole parent/two-path ledger and **both committed Git blob IDs** from GitHub; they must equal the accepted WO-090 branch exactly. Compare baseline-to-integration paths: only ACTIVE and two implementation files, nothing else. The accepted technical branch remains unchanged; do **not** merge, cherry-pick, check out its entire tree or use `git apply`.

## 5. Complete two-host CI at the exact integration commit

Require a **new**, push-triggered, unmodified `rust-verify` run for the **exact integration commit SHA**, on `main`, with `conclusion=success`. Both jobs separately `success`, 33 configured steps, zero failed steps, only 9/14 inapplicable skips. Preserve exact run ID, both job IDs, event/head SHA and actual step inventory. Do **not** attribute an earlier branch run to this integration. Read the actual Ubuntu and Windows job logs and record time-stamped evidence.

**Linux real positive proof:** freshly execute the actual compiled `session-parent` under fixed native pre-exec limits: `RLIMIT_AS` soft/hard both `536870912`; `RLIMIT_CPU` soft/hard both `10`; the child flushes its complete fixed readiness line, parent strictly validates it **before** the sole read-only group/session kernel query, and performs live-before/live-after checks. The accepted evidence line remains exactly `media_host_session=live_attested` with the same fixed field names/order; `pid=pgid=sid=observed_pgid=observed_sid>0`, `reaped=1`, normal exit zero, no parent kill, positive wall time below `15000` ms. **Do not require literal PID uniqueness across runs or exactly 4000 wall ms.**

**New corrective N090 proof:** the exact integrated source must still use `SessionReapV1::from_wait_outcome_v1(child.wait())` and `SessionCleanupPlanV1::classify_v1` on the **early-readiness-refusal path**. Only confirmed `Reaped` may enter the reader-join phase; `Unreaped` reports `HostReapFailed` regardless of initial readiness refusal or kill result. Require all five production-linked deterministic `n090_` truth-table / reader-failure tests on both hosts and the Linux real test `n090_01_the_real_parent_reaps_before_joining_after_the_correction ... ok`. Record real `n090_reap=...` and `n089_readiness=...` test annotations as CI evidence, not additional public/product protocol fields. A **simulated** failed `wait()` is not a real operating-system failed-reap test; say so explicitly.

**Negatives:** direct unsupervised `session-child` exits with fixed refusal before the readiness line/sleep; no arbitrary executable or PID inputs. Invalid/incomplete/foreign-PID/duplicate/trailing/over-cap readiness, missing gate, failed/closed handoff, abnormal exit, missed deadline, reader failure and unverified reap all remain closed refusals. Total stdout cap `1024`, stderr cap `512`, fixed child sleep `4000` ms, parent deadline `15000` ms from before spawn, integration test budget `20000` ms, and all older N076/N079/N081/N083/N085 deadlines/limits are unchanged. No descendant, process-group signal, pidfd or new native API.

**Windows:** both `session-parent` and `session-child` remain `HOST_UNSUPPORTED` at exit `3`, empty stdout and the exact fixed diagnostic; no spawn, readiness worker, Linux-only test or Linux kernel evidence. Confirm new `n090_02` and inherited `n089_04`/`n087_04` tests pass. Confirm zero `n090_reap=`, `n089_readiness=` and `n087_session=` positive lines on Windows.

**All original gates, unmodified:** Rust toolchain pinned to `1.99.1`, `cargo fmt --all -- --check`, strict Clippy warnings-as-errors, locked `cargo test --all-targets`, approved four packages / 33 targets / 60 files / **41 edges** and literal workflow graph pins, architecture self-check **29** cases, forbidden import/checker-bypass negative tests, `cargo-deny 0.20.2` with actually refreshed and verified RustSec DB, release smoke `accepted_exit=0 refused_exit=2`, clean reproducible release builds and unchanged Cargo.lock Git blob. All N076/N079/N081/N083/N085/N087/N089 regressions and public CLI byte/exit contracts stay green. `test result: FAILED` in the intentionally injected architecture tripwire is acceptable only when the negative rejection is proven and its enclosing workflow step/job is successful; do not suppress it.

**Any unresolved mandatory red run, altered/absent functional evidence, unexpected step skip or scope drift is STOP.** Do not edit accepted source to fix CI inside this integration-only order. Distinguish infrastructure retry (if needed, same SHA and full disclosure) from source changes, which require a separately Owner-issued corrective order.

## 6. Canonical report-only main commit and fresh two-host CI

Once integration is complete, exact Git readback matches and **both** integration jobs are green, overwrite **only** `.documents/Reports/execution-report.md` with the single canonical `SOLO-WO-091 R01` execution report. No other documentation, WIP plan, workflow, Cargo file, guard or source may be modified. The report must disclose:

- Permanent **WO-089 STOP**, the two reasons and frozen branch; **WO-090 ACCEPT** applies to the corrected new branch only; WO-091 is the exact two-blob integration.
- The full preflight pins, sole checkout/remote/WIP count, baseline SHA/tree, issuance SHA/tree/parent/ACTIVE blob, integration SHA/tree/parent, exact before/after blobs, path ledgers and every Git identity proof (including filter-aware vs physical CRLF observations).
- Every issuance/integration/report CI run and Ubuntu/Windows job ID; exact SHA/event/outcome/step inventory; actual fresh Linux `n090_reap`, `n089_readiness`, `n087_session` and N076/N079/N081/N083/N085 observations, Windows negatives and all original gates. Never fabricate or infer a kernel PID or wall measurement.
- The actual production `wait()` classification, failed reap precedence, reader-join restriction, checked readiness-before-query ordering and all pure-versus-real limitations. **Do not** claim a real failed `wait()` was injected or that a normal positive run proves the negative kernel failure occurred.
- All preserved remote branch refs, Owner scratch inventory (labeled local-only), exact task-owned cleanup and unresolved limitations. Declare `R03.C3`, `R03.C`, Stage 03 OPEN and containment/media/publication/release flags false.
- If any red, failed or retried job occurred, report all attempts and exact outcomes. No self-ACCEPT or future order.

Commit **only** `.documents/Reports/execution-report.md`, with sole parent = integration commit SHA. Do not include this report's future own SHA in its body. Push ordinary fast-forward. Confirm GitHub report SHA/tree/parent, single-path numstat and new report blob. Recheck all two promoted **code blobs unchanged** at report HEAD, plus all protected anchors and 12 preserved branches.

Require **another fresh** unmodified push-triggered `rust-verify` at the **exact report HEAD**: Ubuntu and Windows individually green, 33 configured steps each, only 9/14 host-inapplicable skips, no failed steps. Read both logs, including fresh **real mainline N090**, N089 and inherited kernels. A passing branch run or integration run does not replace this requirement.

**Final mainline commit sequence and complete tracked-path ledger** from baseline to report HEAD must contain exactly three commits and four distinct permanent paths:

1. ACTIVE-only `.documents/WIP/workorder-active.md`;
2. Integration-only `crates/solo-adapters/src/media_worker_host.rs` and `crates/solo-cli/tests/media_worker_host_smoke.rs`;
3. Report-only `.documents/Reports/execution-report.md`.

No native/FFI changes, no other WIP files or notes, no new branch, no whole-branch merge. Final `HEAD == main == origin/main` at final report SHA with clean index/worktree. Only the `main` ref may move; `solo-wo-090-r01` and the stopped `solo-wo-089-r01` remain unchanged along with all historical branches.

## 7. Local ownership, clean finish and hard STOPs

Any scratch directory must be newly created and demonstrably task-owned **only after issuance green**, for example `.temp/solo-wo-091-run/`; first verify that path is absent, then record exactly what the agent created. Delete only task-owned disposable files after all dependent jobs/readers/handles are finished, by explicit known path. Preserve `.temp/solo-wo-073-wip/` (historically four files / 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/`, existing Cline logs and any unrecognized ignored material. `git status` cannot certify ignored-file preservation; inspect locally and accurately distinguish this from GitHub-verified facts. No blanket cleaning or destructive repository command.

**STOP immediately** on wrong baseline/tree/blob/ref; missing Owner activation; extra tracked/local dirty path; unexpected WIP count; pre-issuance technical action; red or unverified issuance CI; wrong accepted-branch source or wrong staged/worktree/committed blob; CRLF 'repair', generated change or reformat; any third integration path; changed native seam, lint/guard/workflow/dependency/architecture; integration without both-host CI; report before integration CI; wrong report parent or extra report path; red/unverified report-head CI; changed technical/historical branch; no actual N090 Linux execution after promotion; missing/incorrect reap classification; unrecognized ignored-file deletion; unapproved process-group/descendant/Windows/media/publication/release action; unsafe overclaim; or any other disputed mandatory evidence.

On STOP, do not retroactively ACCEPT or silently fix; make no further unauthorized commits, preserve evidence, and return a truthful blocked report if publication of a report-only status is still possible under Owner authority. An unresolved failure cannot be waived by the Implementation Agent. **Never self-ACCEPT.**

## 8. Exact handoff requirements

Return the one canonical report URL at its exact final SHA, and an evidence ledger containing **full** baseline, issuance, integration and report commit SHA/tree/sole-parent identities; full ACTIVE/report and source/test Git blobs; precise permanent per-commit and cumulative diff stats; every CI run/job ID with SHA/event/conclusion/33-step status; actual fresh Linux `n090_`, `n089_`, `n087_`, N076/N079/N081/N083/N085 observations; Windows fail-closed cases; invariant architecture graph `4/33/60/41`, self-check `29`, RustSec/deny, negative tripwires, release and reproducibility gates; accepted branch preservation; stopped WO-089 historical record; local scratch preservation/cleanup; actual limitations and nonclaims.

**Permitted agent claim only after all gates:** `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-INTEGRATED`, pending independent Chief Master Architect **ACCEPT / REVISE / STOP**. The Agent must then stop. No further order, release, deployment, publication or Stage 03 closure is authorized.
