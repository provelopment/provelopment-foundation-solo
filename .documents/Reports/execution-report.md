# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-091 R01` — INTEGRATE: corrected Linux session readiness and reap-fail-closed proof.
**Matching activation:** `SOLO-WO-091 R01`. **Date (ISO):** 2026-11-13.

**Claimed outcome, submitted for independent review only: `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-INTEGRATED`.**

**Three dispositions, kept deliberately distinct.**

* `SOLO-WO-089 R01` — **permanently STOPPED, NOT ACCEPTED, NOT retrospectively compliant.** Both original reasons stand: **(A)** technical files were edited before the mandatory issuance CI gate; **(B)** the early-readiness-refusal path discarded the `Child::wait()` result and joined readers without a proven leader exit. Its report blob `ee6417c1ab76aad44fc836de5b230e87d4160dea` and its frozen branch `solo-wo-089-r01` `1236915cab6e87b17ce84813430897b273fc3230` (tree `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7`) were not rewritten, moved, deleted or rehabilitated.
* `SOLO-WO-090 R01` — **ACCEPT**, outcome `LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-PROVEN`, on the new corrective branch only. That acceptance does not reach back to WO-089.
* `SOLO-WO-091 R01` (this run) — a bounded **integration-only** promotion of exactly two already-accepted Git blobs into `main`, with zero source authoring.

**Nonclaims.** No real operating-system forced `Child::wait()` failure was injected or observed; the failed-reap classification is proven by deterministic pure tests and by source structure, not by a real kernel reap failure. No process-tree or descendant cleanup, no persistent PID pinning, no race-free identity after observation, no group-signalling authority, no Windows Job Object enforcement, no host sandbox, no AVIF or media correctness, no publication, release or deployment. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; containment, media-correctness, publication and release flags remain **false**.

## 1. Preflight — PASS, before any action

Sole checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; one worktree; one remote `origin`, fetch and push both `https://github.com/provelopment/provelopment-foundation-solo.git`; no stash; no untracked path; exactly four `.documents/WIP/` files. `git status --porcelain` showed **only** ` M .documents/WIP/workorder-active.md` — the complete, unmodified Owner-installed `SOLO-WO-091 R01` (22 748 bytes, LF), never edited, trimmed or auto-formatted. `.temp/solo-wo-091-run/` was verified **absent** and was created only after the issuance gate returned green.

| Baseline identity | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `0fc2da6fd41699acb9add8c9369a6d83ef9c994a` | exact (local, remote-tracking, GitHub) |
| Baseline tree | `5754e30acb2e435c64939e9f7596a24484f81bbe` | exact |
| Sole parent of that report commit | `2d0314ab30a4bea75e979be6ab61add497689c22` | exact |
| Canonical report blob | `6a36487b1e668b6f53957881c0ebc8ea18bbac6e` | exact |
| Previously tracked ACTIVE blob | `4aac00be56053978fd11fbc97b975590ae642e06` | exact |
| `plan-next.md` / `plan-roadmap.md` / `plan-outlook.md` | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| `rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| `solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` | exact |
| `solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` | exact |
| **Native seam** | `1e698245493f6d10ebb9636282698a797a242865` | exact |
| **Façade (pre-promotion)** | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | exact |
| `solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| **Host integration test (pre-promotion)** | `a97c09954e96fbddd6380fb301be182584c183ea` | exact |
| `tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` | exact |
| Arch graph checker | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` | exact |
| Workspace graph checker | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` | exact |

**Seventeen of seventeen anchors exact, zero mismatches.** **Twelve** refs at baseline — `main` plus eleven branches — identical across local `refs/heads`, `origin/*` and `git ls-remote --heads origin` (`remote_count=12`, `main` counted once, not also as a remote-tracking duplicate). Accepted promotion source `solo-wo-090-r01` = `0eb3fcf645851ae752bd59e12fe32b69604197d2`, tree `71a26a58558fcbe6a18ae819c0fe9cdffa9de1d3`, parent `052fa5e38de7f45535e6012f4063b6a9320cc9be`, façade `a1f2bdb9960a4befc44efab746640e4d19bb624b`, test `5444013571a71efde758422fdf9d8fada79ef53a` — all exact. Stopped `solo-wo-089-r01` exact and untouched. No branch/ref drift, no extra worktree, no recognized-but-unexpected local material.

**Owner-local ignored inventory (agent-observed only — GitHub cannot verify ignored paths):** `.temp/solo-wo-073-wip/` = **4 files / 46 062 bytes**; plus `architecture-fixtures/`, `toolchain-installer/` and `target/`. All preserved and never read as code.

## 2. ACTIVE-only issuance — PASS, before any promotion

| Field | Value |
| --- | --- |
| Issuance commit | `57b31d0f5036f574e86eb54f43eb2ddfc36b55ae` |
| Issuance tree | `7ca6bf078ecb571d5231ccf84ee982cfe46cf071` |
| Sole parent | `0fc2da6fd41699acb9add8c9369a6d83ef9c994a` |
| Changed paths | exactly one `M .documents/WIP/workorder-active.md`, `+85 −93` |
| New ACTIVE blob | `78faa1b3a5ce2e025d7c5aafc0c0db0b3154e7c1` |
| Push | ordinary fast-forward `0fc2da6..57b31d0  main -> main` |
| GitHub readback | tree `7ca6bf07…`, sole parent `0fc2da6f…`, one file `+85/-93`, ACTIVE blob `78faa1b3…` |

**Issuance CI gate — PASS.** `push`-triggered `rust-verify` run **`43403442303`** at exactly `57b31d0f…`: `conclusion=success`, `event=push`, `head_branch=main`, `attempt=1`. Jobs **`131811775177`** `verify (ubuntu-24.04)` **success** (33 configured steps, 24 success / 9 skipped, 0 failed) and **`131811774673`** `verify (windows-2025)` **success** (33 steps, 19 success / 14 skipped, 0 failed). Real logs inspected. Afterwards `HEAD == main == origin/main == 57b31d0f…`, clean index and worktree, and unchanged source blobs. **No promotion, no scratch and no technical action occurred before this green.**

## 3. Two-path accepted-blob promotion — five independent proofs

Executed on clean `main` at the issuance SHA, verbatim:

```text
git restore --source=0eb3fcf645851ae752bd59e12fe32b69604197d2 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

| Proof | Requirement | Observed |
| --- | --- | --- |
| **1** | exactly two `M` rows; numstat `491 14` / `268 0`; `--check` silent; nothing unstaged or untracked | exact; `cached_count=2`, `cached_check=0`, `unstaged=[]`, `untracked=[]` |
| **2** | index object == accepted-branch object == pinned accepted blob, both paths | `100644 a1f2bdb9… 0` and `100644 54440135… 0`; branch `a1f2bdb9…` / `54440135…` |
| **3** | filter-aware `git hash-object` == accepted blob, both paths | `a1f2bdb9…` / `54440135…` — identical |
| **4** | 15 protected blobs unchanged versus issuance | 15 checked, **0** mismatches |
| **5** | GitHub comparison of staged/index/branch/committed objects | all four agree on both paths (see §4) |

**Physical line-ending observation, reported as mapping and not as an edit.** With `core.autocrlf=true` and `* text=auto`, the physical worktree streams contain CRLF (`crlf=true`) and the `--no-filters` physical hashes are `dc57811c15a7c72d6c18b99e3bf3d5e69130cc83` (façade, 209 500 bytes) and `c32acb30ac5fc790097088298267d8a0c5e1660b` (test, 58 358 bytes). Those differ from the canonical LF Git blobs purely by line-ending mapping. No Git attribute was toggled, nothing was normalized or re-encoded, and no accepted source byte was edited.

## 4. Integration commit and its complete two-host CI

| Field | Value |
| --- | --- |
| Integration commit | `4b9d115246f5e7070c9981bd123a3de9f8901519` |
| Integration tree | `e0e686c1680625a6fec3ff7c209820cc86a20bfb` |
| Sole parent | `57b31d0f5036f574e86eb54f43eb2ddfc36b55ae` |
| Ledger | exactly two `M` rows: `+491 −14` façade, `+268 −0` test |
| Committed façade blob | `a1f2bdb9960a4befc44efab746640e4d19bb624b` |
| Committed test blob | `5444013571a71efde758422fdf9d8fada79ef53a` |
| Native seam at integration | `1e698245493f6d10ebb9636282698a797a242865` (unchanged) |
| Push | ordinary fast-forward `57b31d0..4b9d115  main -> main` |

**GitHub readback:** tree `e0e686c1…`, sole parent `57b31d0f…`, two files `+491/-14` and `+268/-0`; façade content blob `a1f2bdb9…` and test content blob `54440135…` at the integration commit **and** at `0eb3fcf6…` on the accepted branch — the same objects on both sides of the promotion. Baseline→integration paths are exactly three: ACTIVE plus the two implementation files. No merge, no cherry-pick, no `git apply`, no whole-tree checkout, no reformat, no normalization.

| CI stage | Run | Ubuntu job | Windows job | Result |
| --- | --- | --- | --- | --- |
| Issuance | `43403442303` | `131811775177` success | `131811774673` success | green, 33 steps each, 0 failed |
| **Integration** | **`43403903155`** | **`131813336491` success** | **`131813336133` success** | **green, 33 steps each, 0 failed, 9/14 skips** |
| Report head | see §9 handoff | — | — | required separately |

Both integration runs are `push`-triggered at `4b9d115246f5…`, `head_branch=main`, `attempt=1`. The earlier branch run `43400642204` is **not** attributed to this integration.

## 5. Fresh real Linux evidence at the integration SHA

Read from Ubuntu job `131813336491` of run `43403903155` — a genuinely new execution at `4b9d1152…`, not a replay of branch or WO-090 output. The PIDs below are distinct from every earlier run (WO-090 used 4708/4666/4649/4597/4517/4531/4561); that is a freshness observation only, since the order does not require PID uniqueness and none is claimed as proof.

```text
n090_reap=reaped_before_join pid=4712 wall_ms=4008
n089_readiness=validated_before_query pid=4668 wall_ms=4010
n087_session=media_host_session=live_attested as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4655 pgid=4655 sid=4655 observed_pgid=4655 observed_sid=4655 reaped=1 wall_ms=4008
n085_cpu=media_host_cpu=kernel_terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4601 pgid=4601 signal=9 reaped=1 parent_killed=0 wall_ms=10004
n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4522 pgid=4522 wall_ms=10
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4535 pgid=4535 signal=9 reaped=1 wall_ms=2004
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4565 pgid=4565 wall_ms=10
n089_direct_session_child=exit=71
n087_direct_session_child=exit=71
```

**Positive synchronized proof (`N090-03`).** The accepted product line is unchanged: `media_host_session=live_attested` with the same fixed field names and order, `RLIMIT_AS` soft/hard `536870912`, `RLIMIT_CPU` soft/hard `10`, `pid=pgid=sid=observed_pgid=observed_sid>0`, normal exit zero, no parent kill, `reaped=1`, and `wall_ms=4008` — strictly between 0 and `15000`. The child flushed its complete fixed readiness line and the parent validated it before the sole read-only group/session query, bracketed by live-before and live-after checks. `n090_reap=…` and `n089_readiness=…` are **test-log annotations** describing inspected source state and measured values; they are not additional product or protocol fields.

**The production failed-reap decision rule is present in the integrated source.** The early-readiness-refusal path uses `SessionReapV1::from_wait_outcome_v1(child.wait())` and `SessionCleanupPlanV1::classify_v1(kill_ok, reap, refusal)`; only `Reaped` may enter the reader-join phase, and `Unreaped` reports `SmokeRefusalV1::HostReapFailed` regardless of the initial readiness refusal or of whether the kill was accepted. Ordering is unchanged: capped `recv_timeout` on the remainder of the single 15 000 ms deadline → strict readiness validation → live-before → the sole `getpgid`/`getsid` pair → live-after, with the query structurally unreachable without a validated line.

**Deterministic tests — five production-linked, all green on both hosts:** `n090_01_a_confirmed_reap_keeps_the_refusal_and_permits_joining`, `n090_02_a_failed_reap_overrides_the_readiness_refusal`, `n090_03_a_kill_result_cannot_stand_in_for_a_reap`, `n090_04_the_cleanup_rule_is_total_over_every_refusal` (truth table over 3 refusals × 2 kill results × 2 reap outcomes), `n090_05_a_reader_failure_cannot_become_success`.

**Linux real corrective test:** `n090_01_the_real_parent_reaps_before_joining_after_the_correction ... ok`.

**Negatives, all still closed.** Direct unsupervised `session-child` exits `71` before any readiness line and before the four-second window (two independent observations above). No arbitrary executable or PID input exists. Malformed, absent, delayed, foreign-PID, duplicated, trailing and over-cap readiness all refuse; a missing gate, failed or closed handoff, abnormal exit, missed deadline, reader failure and unverified reap all refuse. Retained `n089_01_an_attempt_without_the_readiness_gate_is_refused ... ok` and `n089_02_the_gated_line_and_the_full_stream_must_agree_exactly ... ok` re-prove the barrier and the exact early-equals-full comparison. Caps and budgets unchanged: stdout `1024`, stderr `512`, child sleep `4000` ms, parent deadline `15000` ms from before spawn, integration budget `20000` ms, existing poll interval, and all older N076/N079/N081/N083/N085 deadlines intact. No descendant handling, no process-group signal, no `pidfd`, no new native API.

**Windows (`N090-05`).** `n090_02_both_session_modes_stay_unsupported_after_the_correction ... ok`, plus inherited `n089_04_no_readiness_worker_runs_where_the_boundary_is_unsupported ... ok` and `n087_04_both_session_modes_are_unsupported_before_any_child_exists ... ok`. Both private session modes remain `HOST_UNSUPPORTED` at exit `3` with empty stdout and the exact fixed diagnostic; no spawn, no readiness worker, no Linux-only test executed, and **zero** `n090_reap=`, `n089_readiness=` and `n087_session=` positive lines in the entire Windows log.

**All original gates, unmodified and green on both hosts.** Rust pinned to `1.99.1`; `cargo fmt --all -- --check`; strict Clippy with warnings as errors; locked `cargo test --all-targets` giving **33** `test result: ok` groups per host; `architecture=legal packages=4 targets=33 files=60 edges=41` with the literal workflow graph pins; architecture self-check `cases=29`; forbidden-import and checker-bypass negative tests; `cargo-deny 0.20.2` with a freshly refreshed and verified RustSec DB, `DENY_PROBE_RESULT=refused`; `release_smoke=verified accepted_exit=0 refused_exit=2`; `reproducibility=verified` across clean builds; `lock_sha256=bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723`; all N076/N079/N081/N083/N085/N087/N089 regressions and the public CLI byte/exit contracts unchanged. The single `test result: FAILED` group per host is the intentionally injected architecture tripwire, proven to reject, inside a step and both jobs that concluded **success**; it was not suppressed.

## 6. Preservation

`main` advanced by exactly two commits since baseline — issuance `57b31d0f` and integration `4b9d1152` — and the tracked-path ledger from baseline to integration is exactly **three** paths: `.documents/WIP/workorder-active.md`, `crates/solo-adapters/src/media_worker_host.rs`, `crates/solo-cli/tests/media_worker_host_smoke.rs`.

All twelve refs were re-verified after the integration push: `main` at `4b9d1152…`, and `solo-wo-061-r01` `4371af21…`, `069` `e6162cdf…`, `070` `d270f95c…`, `076` `40a40bab…`, `079` `380393dd…`, `081` `cacc0f9b…`, `083` `ccc3ca8f…`, `085` `5bf9082b…`, `087` `8250f742…`, stopped `089` `1236915c…`, accepted `090` `0eb3fcf6…` — every one unmoved locally, in `origin/*` and in `ls-remote`. Only `main` moved. No branch deletion, force push, rebase, history rewrite or second checkout.

## 7. Limitations

* **Simulated, not injected.** The failed-reap branch is proven by deterministic pure tests and by source ordering. A real `Child::wait()` failure was never manufactured on an OS process, and a normal positive run does **not** prove that a negative kernel reap failure occurred.
* **Reap truth, not containment.** The rule guarantees truthful reporting and forbids pre-reap joins; it does not make an unreaped leader safe and does not contain descendants.
* **Ordering, not permanence.** One read-only query is ordered after one validated line; identity is not pinned, PID reuse is not prevented, and no later action is authorized.
* **Line endings.** Physical worktree files are CRLF while committed blobs are LF; filter-aware Git object identity governs equivalence, and the `--no-filters` divergences in §3 are mapping only.
* **Local toolchain.** No local Rust toolchain or linker exists on this Owner host, so all compile, format, lint, kernel, security, release and reproducibility facts are GitHub runner facts. Ignored-file inventory is the only local-only class of observation here.
* **No integration authorship.** This run promoted accepted blobs only; it did not design, correct or re-verify the source by writing it.

## 8. Cleanup

Task-owned scratch was `.temp/solo-wo-091-run/` — two CI job logs and this report draft — created only after the issuance gate was green, recorded, and removed by exact known path at run end. No other temporary path was created or touched: `.temp/solo-wo-073-wip/` (4 files / 46 062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/` and prior-session logs are intact. No blanket deletion, reset, clean, stash or destructive command was used.

## 9. Status

**`LINUX-SESSION-READINESS-REAP-FAIL-CLOSED-INTEGRATED`** is claimed for independent review. The accepted WO-090 correction now lives on `main` at `4b9d115246f5e7070c9981bd123a3de9f8901519` as the exact two accepted blobs, with the complete original two-host suite green at the exact integration commit and fresh real Linux evidence from it.

This is **not** self-acceptance, **not** `SOLO-WO-089 ACCEPT`, and **not** any claim of containment, sandboxing, media correctness, publication, release or deployment. The WO-089 STOP remains permanent. This report's own future commit SHA, tree and blob are deliberately absent; they are supplied in the Implementation Agent's handoff together with the report-head CI run and job identifiers. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; all containment, media-correctness, publication and release flags remain **false**. The next authority is the Chief Master Architect's independent **ACCEPT / REVISE / STOP** at this report's exact revision and integration head `4b9d115246f5e7070c9981bd123a3de9f8901519`.
