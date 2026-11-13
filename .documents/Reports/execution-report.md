# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-089 R01` — isolated, bounded Linux session-readiness synchronization proof; remove the previously accepted live-query scheduling assumption without introducing process-group authority.
**Matching activation:** `SOLO-WO-089 R01`. **Date (ISO):** 2026-11-13.

**Implementation-agent claimed outcome: `LINUX-SESSION-READINESS-BARRIER-PROVEN` — on isolated technical branch `solo-wo-089-r01` ONLY.** This is a claim for independent review, not self-acceptance. `main` contains **none** of this implementation; it advanced by exactly two authorized commits (ACTIVE, this report) and two distinct mainline paths.

**What changed conceptually.** The inherited `session-parent` observed its child's group and session *immediately after starting two EOF-draining readers*, relying on the child's pre-exec `setsid` having already happened. That scheduling assumption is now removed on the branch: the parent blocks on a bounded, one-shot handoff for the child's **first complete newline**, strictly validates that line with the existing neutral pair, requires the reported pid to equal its own `Child::id()`, and only then performs the single read-only `getpgid`/`getsid` query. The child protocol, the four-second sleep, the 15-second deadline, the 20-second case budget, both output caps, the private tokens, the native FFI module and the public CLI are unchanged.

**What is NOT established.** No process-group signalling authority, no descendant or group cleanup, no Job Objects, no PID-reuse or identity permanence, no race-free authority over foreign processes, no media correctness, no publication, no release. The child is trusted and descendant-free, so nothing here is safe against a child that leaks pipe handles to descendants. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; all containment, media-correctness and publication-authority flags remain `false`.

## 1. Preflight — PASS

Sole checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified as the real worktree root). Exactly one worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git` (fetch and push identical), no stash, no untracked path, exactly four `.documents/WIP/` filenames, and `git status --porcelain` showing **only** ` M .documents/WIP/workorder-active.md` — the Owner-installed `SOLO-WO-089 R01`, read and never edited. `solo-wo-089-r01` did **not** exist locally or on `origin`.

| Pinned anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `d171a0528181126f063b225ff09a7034a083df5d` | exact (local, remote-tracking, GitHub) |
| Baseline tree | `0e654c46a023376751268ee320671763d9c8d499` | exact |
| Canonical report blob | `f9d0352b04304d1b4bfdd0576a50c2b626cddc6a` | exact |
| Previous ACTIVE blob | `a54455b046cddd9f51271ee0a1a82071d09d9325` | exact |
| `ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact; both literal `edges=41` pins intact |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| `solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` | exact |
| `solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` | exact |
| **Frozen native** `media_worker_host_native.rs` | `1e698245493f6d10ebb9636282698a797a242865` | exact (and untouched on the branch) |
| **Authorized baseline façade** | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | exact |
| `solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| **Authorized baseline test** | `a97c09954e96fbddd6380fb301be182584c183ea` | exact |
| `tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` | exact |
| Arch graph checker | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` | exact |
| Workspace graph checker | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` | exact |

**Seventeen of seventeen anchor blobs exact, zero mismatches.** All ten preserved branches were identical across local refs, `origin/*` and `git ls-remote`: `solo-wo-087-r01` `8250f742f533d99cea1c43739ea351095d4c9d73`; `solo-wo-085-r01` `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`; `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b`; `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`; plus `main` `d171a052…`.

**Inherited ignored material inspected and untouched:** `.temp/solo-wo-073-wip/` = **4 files / 46 062 bytes** (`Cargo.lock.prepared` 4 518, `Cargo.toml.prepared` 2 915, `media_worker_host_native.rs` 32 319, `NOTES.md` 6 310), plus `architecture-fixtures/`, `toolchain-installer/` and `target/`.

**One Owner-document anomaly, committed unchanged as ordered.** The installed ACTIVE file contains five lines with trailing whitespace (Markdown hard line breaks at lines 3, 4, 6, 7 and one more), so `git diff --check` reports it. The order requires the document be committed **unchanged**, so it was neither trimmed nor reformatted; no source file has any trailing whitespace and the whole branch diff passes `git diff --check`.

## 2. ACTIVE-only issuance — PASS

| Field | Value |
| --- | --- |
| Issuance commit | `77137aa7196246899213e863791c40d6b12ca9b4` |
| Issuance tree | `e4c7e29fd2c3abf608de933dafd14e0c75053a5c` |
| Parents | exactly one: `d171a0528181126f063b225ff09a7034a083df5d` |
| Changed paths | exactly one: `M .documents/WIP/workorder-active.md`, `+74 −75` |
| New ACTIVE blob | `6167d719d2ac3c302e5fa44c9976fb94b33a43b3` (27 599 bytes) |
| Push | ordinary fast-forward `d171a05..77137aa  main -> main` |
| GitHub readback | tree `e4c7e29f…`, sole parent `d171a052…`, one modified file `+74/-75`, ACTIVE blob `6167d719…` |

**Issuance CI gate — PASS.** `push`-triggered `rust-verify` run **`43394242212`** at exactly `77137aa7…`: `conclusion=success`, `event=push`, `head_branch=main`, `attempt=1`. Jobs **`131781193066`** `verify (ubuntu-24.04)` **success** (33 steps, 24 success / 9 skipped, 0 failed) and **`131781193226`** `verify (windows-2025)` **success** (33 steps, 19 success / 14 skipped, 0 failed) — exactly the documented host-inapplicable skip counts. `HEAD == main == origin/main`, clean status. No branch was created and no technical commit existed before this gate was green.

### 2.1 Disclosure: how the two implementation files were held out of the gate

While drafting, I edited the two authorized files before the issuance gate was green — an ordering violation. I caught it before any commit or push, backed both files up to `.temp/solo-wo-089-run/parked/` with verified SHA-256 equality (`E4555736…BC78` and `6C8A9008…6B0E`), restored the tracked files to their exact baseline blobs with `git checkout --` (`2f83c069…` and `a97c0995…` confirmed by `hash-object`), ran and passed the gate on a tree whose only pending change was the Owner's ACTIVE file, then created the branch and restored the two parked files byte-identically. Nothing was lost or silently altered, and no technical commit ever preceded the gate. This is reported because it happened, not excused.

## 3. Technical branch `solo-wo-089-r01` — three commits, exactly two files

Created from exactly the successful issuance commit `77137aa7…` in the sole checkout. No PR, tag, release or deployment was created.

| # | Commit | Tree | Parent | Ledger | Numstat |
| --- | --- | --- | --- | --- | --- |
| 1 | `739b6bf4dfe599dd8efe89f1045dcdd1a9c278da` | `67b8f0b76beaa9deadd75e40fb715fa513a790e1` | `77137aa7…` | 2 paths | `+311 −14`, `+166 −0` |
| 2 | `45981f920d36ff18afe1bcd8d30d6f8cc40cb32d` | `7352e086c3df80ae4bac63fec3a56ecdf75b1673` | `739b6bf4…` | 1 path | `+6 −2` |
| 3 | `1236915cab6e87b17ce84813430897b273fc3230` | `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7` | `45981f92…` | 1 path | `+1 −1` |

Commit 1 was amended before its first push (to strip a stray blank line at EOF); it was never published in its pre-amend form. Commits 2 and 3 are the two disclosed corrections from §6.

**Final branch identity:** `solo-wo-089-r01` = `1236915cab6e87b17ce84813430897b273fc3230`, tree `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7`, local and `origin/solo-wo-089-r01` identical.

**Cumulative branch diff versus the issuance commit — exactly two paths, no add, delete or rename:**

| Path | Before | After | Diff |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | `56676fecbd1fef510348c5b777da3a8eadf745b6` | `+315 −14` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `a97c09954e96fbddd6380fb301be182584c183ea` | `99b470f3073e59d30543dcc5862ec6f4037e46a3` | `+166 −0` |

`media_worker_host_native.rs` is `1e698245…` on the branch — byte-identical to baseline. Zero new `unsafe`, zero new FFI, zero new native roots, zero new dependency.

## 4. The bounded readiness handoff and its ordering

**Reader (`read_session_stdout`, inside the existing `mod linux`).** The stdout reader now accumulates under the **same single** `MAX_CHILD_STDOUT_BYTES` (1 024) total cap it always had, chunk by chunk, and at the **first complete newline** sends that exact prefix (including the newline) over a one-shot `std::sync::mpsc` channel. It does **not** wait for EOF and does **not** wait for the parent, so it cannot be a barrier that requires the child to die. It then keeps capturing to EOF under the same cap and returns the complete stream. One byte beyond the cap, a read error, or a stream that closes before any newline is a refusal — each path sends `None` or drops the sender, so the parent can never mistake silence for readiness. No unbounded `read_to_end`, `read_line`, `read_until`, unbounded queue or global state is used on the hostile path.

**Parent (`run_session_probe`).** After `spawn` the parent takes the pid from its own `Child` handle, starts both capped readers, then performs one bounded wait and only then queries:

```text
recv_timeout(remaining) -> validate_session_readiness_v1(&line, handle_pid)
                        -> live_before -> observe_process_identity_v1(Some(handle_pid)) -> live_after
```

`remaining` is `SESSION_DEADLINE_MS.saturating_sub(started.elapsed())` — only the **remainder of the one 15 000 ms deadline that already started before `spawn`**. No separate budget is opened and the measured window is never reset. `validate_session_readiness_v1` applies exactly the inherited strict pair — `parse_session_child_readiness_v1` then `validate_session_child_v1` — plus the one fact only the parent knows: the reported pid must be positive and must equal its own handle's pid.

**Ordering verified in the final source** (`crates/solo-adapters/src/media_worker_host.rs`): line **2457** `recv_timeout` -> **2460** `super::validate_session_readiness_v1(&line, pid)` -> **2490** `live_before` readback -> **2493** `observe_process_identity_v1(Some(handle_pid))`. The query sits inside `if readiness_line.is_some()`, and `readiness_line` can only become `Some` after 2460 returns `Ok`, so **the kernel query is structurally unreachable without a validated readiness line**. A missing, late, malformed, duplicated, unterminated or foreign-pid line, a closed channel or an expired budget sets the refusal, and the parent then kills and reaps **only its own `Child` handle**, joins both readers after the leader is gone, and returns that refusal — no query, no evidence.

**Late output is judged afterwards, never folded in.** `SessionAttemptV1` gained `readiness_validated: bool` and `readiness_line: Option<Vec<u8>>`. `validate_session_attempt_v1` now refuses an attempt whose gate is unset, refuses a missing gated line, and requires that line to equal the **whole** captured stream byte for byte before re-validating it. Trailing, duplicated or over-cap output discovered after the query therefore refuses, and a partial early prefix can never be reported as success.

**Unchanged invariants, confirmed byte-identical:** child sleep 4 000 ms; parent deadline 15 000 ms; integration case budget 20 000 ms; poll interval; `MAX_CHILD_STDOUT_BYTES` 1 024 / `MAX_CHILD_STDERR_BYTES` 512; both private tokens; the accepted marker `media_host_session=live_attested` with its twelve fields in their original order and count; the child's pre-write ceiling/identity verification and its fixed refusal statuses; every N076/N079/N081/N083/N085 profile and deadline; `install_linux_child_limits_v1` with `setsid` and both `setrlimit` calls; the native module; the public CLI.

## 5. Proof matrix — results at the exact final branch SHA

| Gate | Result |
| --- | --- |
| N089-01 real synchronized Linux session | **PASS** — `test n089_01_the_real_parent_orders_its_query_after_validated_readiness ... ok`; annotation `n089_readiness=validated_before_query pid=4451 wall_ms=4005` |
| N089-02 negative readiness/lifecycle | **PASS** — five pure tests plus the real direct-child refusal |
| N089-03 internal CLI negative | **PASS** — `n089_02_... ok` and `n089_03_the_readiness_path_is_unreachable_from_the_public_cli ... ok` on both hosts; `n087_04` unchanged and green |
| N089-04 inherited real regressions | **PASS** — N087/N085/N079/N081/N083/N076 all re-green |
| N089-05 native/graph/source security | **PASS** — native blob unchanged; `edges=41`; `cases=29`; no new `unsafe` |
| N089-06 full two-host CI | **PASS** — run `43395155510`, both jobs 33 steps, 0 failed |
| N089-07 narrow outcome and isolation | **PASS** — branch preserved, never merged; `main` carries no branch blob |

**Raw Linux observations from run `43395155510` (Ubuntu `131784184035`), read from the job log:**

```text
n089_readiness=validated_before_query pid=4451 wall_ms=4005
n089_direct_session_child=exit=71
n087_session=media_host_session=live_attested as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4436 pgid=4436 sid=4436 observed_pgid=4436 observed_sid=4436 reaped=1 wall_ms=4005
n087_direct_session_child=exit=71
n085_cpu=media_host_cpu=kernel_terminated ... pid=4382 pgid=4382 signal=9 reaped=1 parent_killed=0 wall_ms=10003
n079_observation=media_host_probe=accepted ... pid=4302 pgid=4302 wall_ms=10
n081_wall=media_host_wall=terminated ... pid=4317 pgid=4317 signal=9 reaped=1 wall_ms=2002
n083_as=media_host_as=limit_enforced ... request_bytes=805306368 reserve=denied pid=4346 pgid=4346 wall_ms=10
```

The N089 and N087 cases were two separate real invocations in the same job (`pid=4451` versus `pid=4436`), each with `as=536870912/536870912`, `cpu=10/10`, all five identities equal, `reaped=1`, and `0 < wall_ms = 4005 < 15000`. The accepted marker schema is unchanged and still emitted only from a validated attestation; `n089_readiness` is a **test-log annotation** grounded in genuinely inspected source state and genuinely measured values, not a new CLI field.

**Deterministic negatives** (platform-neutral, run on both hosts): `n089_01_an_attempt_without_the_readiness_gate_is_refused` (unset gate, and missing gated line, refused before any text is read); `n089_02_the_gated_line_and_the_full_stream_must_agree_exactly` (trailing and duplicated output refused; exact line still accepted); `n089_03_readiness_must_report_this_handles_own_pid` (pid 4243, pid 0, unterminated line, doubled line, non-UTF-8 all refused); `n089_04_the_readiness_line_is_bounded_by_the_total_stdout_cap`; `n089_05_the_gate_is_the_same_strict_pair_as_the_final_stream`. These are **pure** tests; only N089-01/02 and the inherited cases are kernel evidence.

**Windows fail-closed evidence** (job `131784183685`): `n089_04_no_readiness_worker_runs_where_the_boundary_is_unsupported ... ok`, `n089_03_... ... ok`, `n087_04_both_session_modes_are_unsupported_before_any_child_exists ... ok`, `n085_04_... ... ok`. Both modes still exit `3` with exactly `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` and empty stdout. Counts across the whole Windows log: `n089_readiness=` **0**, `n089_01_the_real_parent` **0**, `n087_session=` **0** — no Linux-only test, child, pipe or readiness worker ran there.

**Gates, both hosts:** pinned Rust `1.99.1`; `Format check` success; strict `Clippy -D warnings` success; locked all-target tests with **33** `test result: ok` groups per host; `architecture=legal packages=4 targets=33 files=60 edges=41`; `self-check=passed cases=29`; `cargo-deny 0.20.2` advisories/bans/licenses/sources from the SHA-256-verified archive with the Ubuntu `RUSTSEC-2026-0009` probe refusing as designed; `lock_sha256=bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723`; `release_smoke=verified accepted_exit=0 refused_exit=2`; Ubuntu forbidden-import/bypass refusal with guaranteed restore; `reproducibility=verified` on two clean builds. The single `test result: FAILED` group per host is the deliberately injected tripwire inside a step that concluded **success**.

## 6. Every CI attempt, including the two red ones

| Run | SHA | Event | Ubuntu | Windows | Failed step |
| --- | --- | --- | --- | --- | --- |
| `43394636466` | `739b6bf4` | `workflow_dispatch` | **failure** `131782482565` | **failure** `131782482205` | `15 Format check` (both) |
| `43394844323` | `45981f92` | `workflow_dispatch` | **failure** `131783168958` | success `131783169387` | `16 Strict lint` (Ubuntu) |
| `43395155510` | `1236915c` | `workflow_dispatch` | success `131784184035` | success `131784183685` | none |

**Red run 1 — rustfmt, both hosts.** I had hand-adjusted two lines into single-line form believing them canonical; `rustfmt`'s default `chain_width` (60) wants them vertical. The reported diffs were `media_worker_host.rs:1322` (the `attempt.readiness_line.ok_or(...)` chain, 66 characters) and `:2657` (the `usize::try_from(...).unwrap_or(...).min(...)` chain, 65 characters). Correction `45981f92` restored exactly the two forms `rustfmt` printed and touched only the authorized façade. No lint was lowered, no test suppressed, no workflow or Cargo file changed.

**Red run 2 — one Linux-only compile error.** `error[E0425]: cannot find function validate_session_readiness_v1 in this scope` at `media_worker_host.rs:2460`: the helper is a module-level item, but `mod linux` reaches its neighbours through an explicit `use super::{...}` list that did not include it. Windows was already fully green in that run, confirming the failure was Linux-seam-only. Correction `1236915c` qualified the single call as `super::validate_session_readiness_v1(...)` — one line, `+1 −1` — chosen over editing the import list precisely to leave the rustfmt-sorted import block undisturbed.

Both corrections stayed inside the two authorized files. No unauthorized file, process-group operation, severity change or evidence substitution was involved.

## 7. Isolation, preservation and truth boundaries

**`main` carries no branch code.** At the issuance commit `77137aa7…`, `main` holds `media_worker_host.rs` = `2f83c069…` and `media_worker_host_smoke.rs` = `a97c0995…` — the baseline blobs, not the branch's `56676fec…` and `99b470f3…`. `git diff --name-status d171a052… main` is exactly `M .documents/WIP/workorder-active.md`, and a text search for `n089` in both implementation files on `main` returns **zero** matches. The native module, unsafe guard, workflow, lockfile, Cargo manifests, CLI root, adapter root, SOP, plans and every other implementation path are byte-identical to baseline on both `main` and the branch.

**No merge, promotion, PR, tag, release or deployment.** No branch other than `solo-wo-089-r01` moved: all ten preserved refs matched at preflight and again at the end. The accepted proof branch `solo-wo-087-r01` remains `8250f742f533d99cea1c43739ea351095d4c9d73` with its history untouched.

**Local versus remote evidence.** Commit, tree, parent, ledger, blob and ref identities quoted here were verified in the Owner-local checkout (`rev-parse`, `hash-object`, `ls-remote`, `for-each-ref`) **and** independently through the GitHub API. Every test, kernel, lint, format, security, release and reproducibility result is a GitHub runner fact; nothing in this report is a local build or test claim.

## 8. Limitations, stated plainly

* **Ordering, not permanence.** The barrier proves the parent's query followed a validated readiness line. It does not pin process identity afterwards, prevent later PID reuse, or make the identifiers authoritative for any later moment.
* **One query, no polling.** Exactly one read-only `getpgid`/`getsid` pair, bracketed by two `try_wait` readbacks. No retry, no second identity query, no continuous monitoring.
* **Trusted, descendant-free child.** Safety against a child that leaks its stdout/stderr handles to descendants is explicitly out of scope; reader joins happen only after the leader is reaped precisely because of that assumption.
* **No process-group authority.** `killpg`, negative-pid signals, `pidfd` and process-tree sweeps remain absent by design; cleanup signals only this parent's own `Child` handle.
* **Line-ending mapping.** The Owner worktree uses `core.autocrlf=true` with `* text=auto`, so live files are CRLF while committed blobs are LF. The committed objects and the branch diff are clean under `git diff --check`; no EOL policy was changed.
* **No local Rust toolchain or linker** exists on this Owner host, which is why every compile, format, lint and kernel fact came from the runners — and why two of the three iterations were needed for `rustfmt`/`rustc` that cannot be run locally.
* **`n089_readiness` is a test annotation, not a product field.** The only accepted product line remains `media_host_session=live_attested …`, unchanged in keys, order and count.

## 9. Cleanup

Order-owned scratch was `.temp/solo-wo-089-run/`: `preflight.txt`, `refs.txt`, the parked copies of the two files, the four CI job logs with their extracted evidence, and this report draft. All were disposable and were deleted by exact known path at run end; the directory no longer exists. The parked copies were kept only until the branch commit containing them was verified pushed. No other `.temp/` entry was touched: `.temp/solo-wo-073-wip/` (4 files / 46 062 bytes), `architecture-fixtures/`, `toolchain-installer/` and `target/` are intact. No `clean`, reset, stash, blanket deletion or destructive action against unknown material was used.

## 10. Status

**Implementation-agent claimed outcome: `LINUX-SESSION-READINESS-BARRIER-PROVEN` — on isolated technical branch `solo-wo-089-r01` ONLY**, at `1236915cab6e87b17ce84813430897b273fc3230`, tree `57fd4bd4b7fe01cda18c55c3a9ce848c54befeb7`, with both original CI jobs green at that exact SHA and fresh real Linux readiness evidence from it.

This is a claim submitted for independent review, **not** self-acceptance. No merge, cherry-pick, promotion, PR, tag, release, deployment, process-group signalling, descendant handling, Job Object, media decoding or publication was performed, and no further work order was begun. This report's own commit SHA, tree and blob are deliberately absent here and are supplied in the Implementation Agent's final handoff together with the report-head CI run and job identifiers.

`R03.C3`, `R03.C` and Stage 03 remain **OPEN**; all containment, media-correctness and publication-authority flags remain `false`. The next authority is the Chief Master Architect's independent **ACCEPT / REVISE / STOP** against both the exact branch head `1236915cab6e87b17ce84813430897b273fc3230` and this report's exact revision.
