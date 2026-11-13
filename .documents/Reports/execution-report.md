# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-087 R01` — prove independent, read-only Linux live-child process-group **and** session identity from both the trusted child and its owning parent, on an isolated branch only.
**Matching activation:** `SOLO-WO-087 R01`. **Date (ISO):** 2026-11-13.

**Implementation-agent claimed outcome: `LINUX-LIVE-CHILD-SESSION-ATTESTATION-PROVEN`, on branch `solo-wo-087-r01` only.** This is a claim for independent review, not self-acceptance.

**What is proved.** One trusted, descendant-free Linux child — spawned by this same executable through the **unchanged** pre-exec boundary — independently read its own four ceiling halves and its own kernel `pid == pgid == sid`. While that child was demonstrably still live, the owning parent separately obtained that same child's group and session ids from the real kernel, using only its own `Child` handle's pid, with `try_wait` liveness readbacks immediately before and immediately after the query. The child then slept its fixed four seconds, exited **normally** with status zero, and the parent reaped the leader it had spawned. Both independent views agreed exactly.

**What is NOT proved.** This is an **observation of a live child at this check**, not a persistent identity pin, not protection from PID reuse after the check, and not containment. There is **no** process-group signal, no `killpg`, no `kill(-pid, …)`, no `pidfd_send_signal`, no descendant sweep, no process-tree cleanup, no race-free persistent group handle, no arbitrary PID or executable input, no Windows process control or Job Object, no AVIF parsing, decoder or publication, and no release or deployment. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; all media-correctness and publication flags remain `false`. The permitted vocabulary of this order is exhausted by the single phrase claimed above: this is **not** `LINUX-GROUP-KILL-PROVEN`, `DESCENDANT-CLEANUP-PROVEN`, `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED` or Stage 03 complete.

**Two mainline commits, two distinct tracked paths.** `main` advanced from `b6b2506e…` by two ordered commits: (1) the Owner-installed ACTIVE file only, (2) this canonical report only. No implementation, workflow, Cargo, guard, architecture or WIP-plan byte changed on `main`. The three technical files exist **only** on `solo-wo-087-r01` and were never merged, cherry-picked or path-promoted to `main`.

## 1. Preflight — PASS (`S087-01`)

Sole Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, verified as the real worktree root (`git rev-parse --show-toplevel` = `C:/Users/Sean/Documents/Provelopment/00.foundation-solo`). Exactly one worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git` (fetch and push identical), no stash, exactly four `.documents/WIP/` filenames (`workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`), and `git status --porcelain` showing **only** ` M .documents/WIP/workorder-active.md` — the Owner-installed order and nothing else.

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `b6b2506e592eb9475116d21f012df28880b8d3c5` | exact, local and via GitHub `git/ref/heads/main` |
| `HEAD^{tree}` | `c7844e86db8556d7ecce4f0bcda6c1def14f1d57` | exact |
| Canonical report blob | `22b703c635085293f41e525a9bd54152c1af04d0` | exact |
| Previous tracked ACTIVE blob | `381c4e6061488cd7af1586aa2c425d589cc41d74` | exact |
| Charter `ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact: `92fa562b1af0c54391851aa5c79deb1631127069`, `9d7a86226878f490fd5e26c6813a81e5f73f3f37`, `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact; both literal `edges=41` checks present |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| `solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` | exact |
| `solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` | exact |
| Native module to extend | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| Façade to extend | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | exact |
| Integration test to extend | `038c3193e96633f34e0d34baffab7d04b348573d` | exact |
| `solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| Unsafe guard `tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` | exact |
| Arch graph `tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` | exact |
| Workspace graph `tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` | exact |

**Branch name was free.** `git rev-parse --verify refs/heads/solo-wo-087-r01` and `refs/remotes/origin/solo-wo-087-r01` both returned nothing, and `git ls-remote --heads origin` listed only `main` plus the eight historical branches. The name was created once, from the green issuance SHA, and never reused, moved or rewritten.

**Preserved historical/proof branches** — verified locally **and** on `origin` before and after this run: `solo-wo-085-r01` `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`; `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b`; `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`.

## 2. ACTIVE-only issuance on `main` — PASS (`S087-02`, `S087-03`)

The Owner-installed `SOLO-WO-087 R01` document was committed **unchanged**, alone, on `main`.

| Field | Value |
| --- | --- |
| Issuance commit | `e829ce57bc4f2e9f796b5969f239570c3e7d54d2` |
| Issuance tree | `77389e49df999488a01627557ac10afee3a6afdd` |
| Parents | exactly one: `b6b2506e592eb9475116d21f012df28880b8d3c5` |
| Changed paths | exactly one: `M .documents/WIP/workorder-active.md`, `+87 −84` |
| New ACTIVE blob | `90eff7424785a3ad3226d88f65bff1a3f58acbc5` |
| Push | ordinary fast-forward `b6b2506..e829ce5  main -> main` |
| GitHub readback | tree `77389e49…`, one modified path, ACTIVE blob `90eff742…` |

**Issuance CI gate — PASS.** Push-triggered `rust-verify` run `43380933349` at exactly `e829ce57…`: conclusion `success`, `event=push`, `branch=main`, `attempt=1`. Jobs `131738258668` `verify (windows-2025)` **success** (33 steps, 19 success / 14 skipped, 0 failed) and `131738258983` `verify (ubuntu-24.04)` **success** (33 steps, 24 success / 9 skipped, 0 failed). The skipped counts are exactly the historical host-inapplicable ones. No implementation branch or technical edit was made until this gate was green; `HEAD == main == origin/main`, worktree clean and historical refs unchanged were re-checked immediately afterwards.

## 3. Technical branch, ledger and every red run

One branch, `solo-wo-087-r01`, created at `e829ce57…` in the existing checkout. No second worktree, clone or nested repository was ever created.

**Final branch tip: `8250f742f533d99cea1c43739ea351095d4c9d73`, tree `bb249a4231b85e50d8a3ce9096721b4a8998cd09`, pushed by ordinary fast-forward and tracking `origin/solo-wo-087-r01` at the same SHA.**

**Three-file permanent ledger** — `git diff --name-status e829ce57…8250f74` returns exactly three `M` rows and nothing else:

| Path | Final Git blob | Total change vs issuance |
| --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `1e698245493f6d10ebb9636282698a797a242865` | `+93 −4` |
| `crates/solo-adapters/src/media_worker_host.rs` | `2f83c0695da5a49b4539d4472dfc2e2a460e1691` | `+883 −5` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `a97c09954e96fbddd6380fb301be182584c183ea` | `+242 −0` |

**Bounded commit chain on the branch** (five commits, all inside the three authorized files, each pushed before its CI run):

1. `3d0703458f1bf3f4ebc85037b6e9bc6252492bdd` — the implementation, tree `51ce2d55f072c62ebc47122432b6a6808a5bad70`.
2. `6144dc2eb665fee9a1b8cdb145f3886d408a0c14` — rustfmt canonical form, tree `1f8d04e32c11d560a64d90a230f442f04f0eaea4`.
3. `d8ebe5c0b1121b3d6e0d9b68176ee73ee9cc1cb8` — neutral fixed-window assertion, tree `a5c6b7d7bfc0ec29c6ddcdfbf49e1ce65060a56f`.
4. `118643b773246d3ece19a44ec52499c341501383` — one wrapped assertion, tree `5f0c71e3cb475f6582e49732dc68ebfbe02c4349`.
5. `8250f742f533d99cea1c43739ea351095d4c9d73` — Linux-gated child-sleep constant, tree `bb249a4231b85e50d8a3ce9096721b4a8998cd09`.

**Every red run is disclosed.** Four consecutive branch runs failed before the final green one; each failure was inside the authorized three-file scope and each correction was a new bounded commit on the same branch with full CI at the new tip:

| Run | SHA | Result | Actual failure |
| --- | --- | --- | --- |
| `43382369941` | `3d07034…` | **failure** | `Format check` on **both** hosts. `cargo fmt --all -- --check` reported four cosmetic diffs: one `use` re-pack, one split `Command::spawn` chain, one doubled blank line and two vertically expanded `vec![…]` literals. |
| `43382695063` | `6144dc2…` | **failure** | Ubuntu **success** (full suite). Windows failed `Strict lint (warnings are errors)` with exactly one error: ``constant `SESSION_CHILD_SLEEP_MS` is never used``. |
| `43383157003` | `d8ebe5c…` | **failure** | `Format check` on **both** hosts: one `assert_eq!` whose arguments exceeded rustfmt's 60-character call-argument heuristic. |
| `43383424669` | `118643b…` | **failure** | Ubuntu **success**. Windows `Strict lint` again reported the **same** `SESSION_CHILD_SLEEP_MS` dead-code error, because a `#[cfg(test)]`-only reference does not make a `pub` constant live in the crate's own non-test compilation. Root cause fixed by gating the constant to Linux and splitting its assertion by host. |
| `43383835725` | `8250f74…` | **success** | Final branch gate, both hosts green (below). |

No test was suppressed, skipped or weakened to obtain green, and no workflow, lint level, Cargo setting, guard or architecture rule was changed at any point.


## 4. Implementation as delivered (`S087-04` to `S087-07`)

### 4.1 Native module — read-only identity observation, no signals

Inside the **existing positive** `#[cfg(target_os = "linux")] mod linux` gate, and only inside it, one closed function was added: `observe_process_identity_v1(target: Option<u32>) -> Result<ProcessIdentityV1, NativeIdentityRefusalV1>`. It performs exactly **two** new `unsafe` call expressions — `libc::getpgid(pid)` and `libc::getsid(pid)` — each with its own adjacent `// SAFETY:` rationale.

* The target is converted by a separate checked helper: `None` becomes the kernel's own `0` "calling process" selector (used **only** by the fixed trusted child), and `Some(pid)` must be a strictly positive value a `libc::pid_t` can represent, refused **before** any call otherwise. No wrapped negative pid can reach the kernel.
* Each kernel status is validated (`<= 0` is the kernel's error report) **before** any result is converted, and the converted values are validated again as positive `u32`. A refusal carries no `errno`, path, process metadata or OS error text.
* **Exactly two new `unsafe` expressions** were added, with the file's existing four (`getrlimit`, `assume_init`, `pre_exec`, `getpgid(0)`) untouched: the file now contains **6 `unsafe {` blocks and 7 `// SAFETY:` notes**, so the unchanged guard's "blocks ≤ notes" rule holds. `setsid` and the two fixed `setrlimit` calls are byte-identical to the accepted baseline; nothing was added to the `pre_exec` closure.
* No `kill`, `killpg`, `pidfd`, `waitpid`, `fork`, `exec`, signal handler, group/session mutation, new module, allowance, crate, dependency, Cargo feature or guard bypass was introduced.

### 4.2 Safe façade — two closed modes and the neutral contract

`InternalProbeModeV1` gained exactly two variants, `SessionParent` and `SessionChild`, reachable only through the already-reserved `--internal-media-host-probe-v1` flag in second position with exactly one literal token. The CLI root and adapter root are **unchanged**: the existing dispatch gained two arms, and `crates/solo-cli/src/main.rs` was not touched.

New fixed contract items (no existing constant, assertion or behaviour altered):

| Name | Value |
| --- | --- |
| Parent token / child token | `session-parent` / `session-child` |
| Child readiness marker | `SOLO-MEDIA-HOST-SESSION-V1` |
| Parent evidence marker | `media_host_session=live_attested` |
| Child sleep | `SESSION_CHILD_SLEEP_MS = 4_000` (Linux-gated: only the Linux child sleeps) |
| Parent deadline | `SESSION_DEADLINE_MS = 15_000` |
| Child refusal statuses | `76` identity readback failed, `77` not its own group/session leader (Linux-gated) |

`parse_session_child_readiness_v1` accepts exactly one line with the marker plus seven ordered fields (`as_soft as_hard cpu_soft cpu_hard pid pgid sid`), plain ASCII decimals, one trailing newline, and refuses a second line, `\r`, `\t`, a doubled space, a missing/duplicate/reordered/extra field, a sign, a prefix or an over-long value. `validate_session_child_v1` requires the four exact profile ceilings and a strictly positive `pid == pgid == sid`.

`validate_session_attempt_v1` is the neutral decision point and refuses, in this order, a parent kill, a missing reap, any non-zero or signalled exit, a child not proved live both before **and** after the observation, a measurement at or beyond 15 000 ms, unreadable/over-cap/non-UTF-8/off-schema bytes, an off-profile or non-identical child line, and any disagreement between the child's reported pid and the parent's own handle pid or between either identity and the parent's independent kernel observation. `session_evidence_line` then emits exactly one line in a fixed field order: marker, `as_soft as_hard cpu_soft cpu_hard pid pgid sid observed_pgid observed_sid reaped wall_ms`.

### 4.3 Linux parent supervision and the fixed child

`run_session_probe` spawns exactly one `session-child` of `std::env::current_exe()` through the **unchanged** `install_linux_child_limits_v1`, with `stdin` null and both output pipes capped at the established 1 024 / 512 bytes. It takes the **exact pid from its own `Child` handle**, requires it non-zero, then performs `try_wait() == Ok(None)` immediately **before** and immediately **after** the single read-only `getpgid(child_pid)` / `getsid(child_pid)` pair, and requires both identities to be positive and equal to that pid. It then waits for a normal exit inside the 15 000 ms deadline, reaps the leader, and joins the capped readers only after the leader is gone. Only this parent's own `Child` handle is ever signalled, and only as bounded cleanup on a refusal or deadline; a successful run terminates by the child's own normal exit, and `parent_killed` can never become evidence.

`run_session_child` verifies the four kernel ceiling halves and its own positive `pid == pgid == sid` **before** writing anything and **before** sleeping, so a direct unsupervised call refuses promptly with a fixed nonzero status and no output. On success it writes and flushes the one readiness line, sleeps exactly 4 000 ms of plain wall time (no CPU burn, no allocation stress, no file or network access, no fork, no thread, no descendant), and exits zero.

### 4.4 Windows and every non-Linux target

Both new modes refuse with internal status `3`, empty stdout and the exact stable diagnostic `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n`, before any child, session query, sleep or native syscall. Neutral parsing, classification and validation still compile and run there. No Linux native module and no Linux-only real test is compiled on Windows, and every previously published public CLI byte and status is unchanged.


## 5. Final branch CI — PASS, at the exact tip (`S087-07`)

`workflow_dispatch` run **`43383835725`** at exactly `8250f742f533d99cea1c43739ea351095d4c9d73`: conclusion `success`, `event=workflow_dispatch`, `branch=solo-wo-087-r01`, `attempt=1`.

| Job | ID | Conclusion | Steps | Skipped / success | Failed |
| --- | --- | --- | --- | --- | --- |
| `verify (ubuntu-24.04)` | `131747551543` | **success** | 33 | 9 / 24 | 0 |
| `verify (windows-2025)` | `131747551816` | **success** | 33 | 14 / 19 | 0 |

The skip counts are exactly the historical host-inapplicable set; no step was skipped for any other reason. `push` does not trigger this workflow for non-`main` branches, which is why the branch gate was dispatched explicitly at the tip, as the order instructs.

### 5.1 `N087-01` — the real parent's independent live-child attestation (Ubuntu)

The integration case calls only `--internal-media-host-probe-v1 session-parent` and `test n087_01_the_parent_independently_attests_a_live_child_session ... ok`. The runner-captured diagnostic line was:

```text
n087_session=media_host_session=live_attested as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4640 pgid=4640 sid=4640 observed_pgid=4640 observed_sid=4640 reaped=1 wall_ms=4010
```

Read directly from that line: the child's own kernel identities and the **parent's separate kernel observations** all equal `4640 > 0`; the four ceilings are the exact profile; `reaped=1`; `wall_ms=4010` satisfies `0 < wall_ms < 15 000`; the exit was normal zero; there is exactly one evidence line and empty stderr; and no foreign marker (`media_host_probe=accepted`, `media_host_wall=terminated`, `media_host_as=limit_enforced`, `media_host_cpu=kernel_terminated`, `SOLO-MEDIA-HOST-SESSION-V1`) appears. Because the neutral validator refuses any attempt lacking `live_before && live_after`, a status of zero is itself the proof that the child was alive on both sides of the observation. The measured wall time is reported, not asserted equal to four seconds.

### 5.2 `N087-02` — direct unsupervised child refuses before its sleep (Ubuntu)

`test n087_02_a_direct_session_child_refuses_before_its_sleep ... ok`, with the captured line `n087_direct_session_child=exit=71`. The case asserts the process exited promptly (`elapsed_ms < 4 000`), with empty stdout and empty stderr, and that the status is one of the child's own documented fixed refusals (`71..=73 | 76 | 77`); it asserts no success. `71` is the existing limits-mismatch class, which is what an unsupervised child sees because no pre-exec boundary installed its ceilings.

### 5.3 `N087-03` — closure, purity and refusal (both hosts)

Eight `n087_03_*` cases ran on Ubuntu and seven on Windows; the difference is exactly the Linux-only fixed-window assertion. They cover: closed/additive token classification with every earlier mode re-verified unchanged; every wrong, extra, repeated, misplaced, upper-cased, underscore-spelled, space-padded and flag-only token refused as `INTERNAL_MODE_INVALID`; unreachability behind a public subcommand; the exact readiness line accepted while twelve shape defects and five off-profile/non-identical lines are refused; a forged child line proving **not** evidence without a matching real parent observation; every incomplete lifecycle refused (parent kill, missing reap, non-zero/signalled exit, not-live-before, not-live-after, at-deadline measurement, absent/non-UTF-8/over-long output, wrong handle pid, wrong observed identity); and the evidence line's fixed marker, field order and non-borrowing of every earlier result's wording. The fixed 15 000 ms deadline and both markers are pinned on every host, and the 4 000 ms window on Linux only.

### 5.4 `N087-04` — Windows fail-closed (Windows)

`test n087_04_both_session_modes_are_unsupported_before_any_child_exists ... ok`. Both `session-parent` and `session-child` returned status `3`, with empty stdout, no `media_host_session=live_attested` record, no `SOLO-MEDIA-HOST-SESSION-V1` line, prompt return well inside 10 000 ms, and `stderr == "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n"`. **Zero Linux-only execution on Windows**, confirmed from the log rather than badges: `n087_session=` count `0`, and the Linux-only `n085_cpu=`, `n076_01`, `n083_01` counts all `0`. `n085_04_both_cpu_modes_are_unsupported_before_any_child_exists ... ok` still passes.

### 5.5 `N087-05` — every earlier proof still green on Ubuntu

Re-executed in the same run, unchanged: `n085_cpu=media_host_cpu=kernel_terminated … pid=4584 pgid=4584 signal=9 reaped=1 parent_killed=0 wall_ms=10004`; `n085_direct_cpu_child=exit=71`; `n079_observation=media_host_probe=accepted … pid=4506 pgid=4506 wall_ms=11`; `n081_wall=media_host_wall=terminated … signal=9 reaped=1 wall_ms=2006`; `n083_as=media_host_as=limit_enforced … request_bytes=805306368 reserve=denied pid=4550 pgid=4550 wall_ms=10`. All 34 `test result:` groups on Ubuntu are `ok` with `0 failed`.

**Downstream gates verified on both hosts:** `architecture=legal packages=4 targets=33 files=60 edges=41` (both literal `edges=41` checks intact, so the workspace graph is unchanged), `self-check=passed cases=29`, strict Clippy with warnings as errors, formatting, locked all-target tests, `cargo-deny 0.20.2` advisories/bans/licenses/sources with a fresh RustSec refresh, `release_smoke=verified accepted_exit=0 refused_exit=2`, the Ubuntu forbidden-import restoration tripwire, and `reproducibility=verified` for the byte-reproducible clean release build.


## 6. Preservation and protected state

`main` on this report's parent is `e829ce57…` (tree `77389e49…`): exactly two tracked paths differ from the baseline, the issued ACTIVE file and this report. Every other pinned anchor in §1 is byte-identical — the three technical files were never written on `main`, so `main` still carries the previously accepted `d10151e4…` / `88d94ecc…` / `038c3193…` blobs.

Worktree and control state: one worktree, one remote `origin` (correct URL, fetch and push identical), no stash, exactly four WIP filenames, `git status --porcelain` empty at the end of the run, and no destructive operation (no `git clean`, reset, rebase, force push, history rewrite, or branch deletion) anywhere. The eight historical/proof branches and `main` are at the same SHAs as before the run.

Ignored inherited scratch was inventoried and left untouched: `.temp/solo-wo-073-wip/` (**4 files / 46 062 bytes**: `Cargo.lock.prepared` 4 518, `Cargo.toml.prepared` 2 915, `media_worker_host_native.rs` 32 319, `NOTES.md` 6 310 — never compiled, staged or used as source), plus `architecture-fixtures/`, `toolchain-installer/` and the existing `target/`.

## 7. Limitations, stated plainly

* **The observation is point-in-time.** The parent proves the child's group and session at one check, bracketed by two liveness readbacks. Kernel identifiers can be reused later; this is not a persistent identity pin and removes no post-check PID-reuse race.
* **Narrow scheduling window.** §5.4 of the order prescribes observing immediately after taking the pid from the handle and bracketing that single query with `try_wait` checks. The child's `setsid` runs in the pre-exec closure microseconds after `fork`, and the parent performs two capped-reader thread creations and a `try_wait` before querying, so the query lands after the child has almost certainly become a session leader. That ordering is a scheduling assumption, not a synchronisation primitive: the implementation deliberately does **not** poll, retry or tolerate a mismatch, because the order requires a single observation whose mismatch is a refusal. A future order could require synchronising on the child's readiness line instead, which would remove the assumption at the cost of partial-pipe reading.
* **Single leader only.** Nothing here observes, signals or reaps any descendant; the fixed child is required to be descendant-free, and joining a reader after that leader exits does not prove general descendant pipe-inheritance safety.
* **No group authority.** The absence of `killpg` and negative-pid signals is the **correct** result of this order's scope, not a missing feature.
* **No local Rust toolchain on the Owner host.** `cargo`, `rustfmt` and `clippy` are absent, so every compile, format, lint, test, kernel and release observation in this report comes from the real `ubuntu-24.04` and `windows-2025` runners. Nothing local was used to assert a build or test outcome, and the four red runs above are the direct consequence of formatting and lint being decided only by the runners.
* **Line endings.** The worktree is `core.autocrlf=true` with `* text=auto`, so live files are CRLF while committed blobs are LF; `git diff --check` is clean and the committed blobs are the canonical LF form. No accepted file's EOL policy was changed.
* **`main` did not test this implementation.** Run `43383835725` tested the branch only. The report-head run below exists for mainline non-regression and report integrity, and must never be cited as N087 functional proof.

## 8. Cleanup — honest inventory

Current-run-owned scratch (`S087` only) was `.temp/solo-wo-087-run/`: the preflight capture, issuance and per-job ledgers, the four failing-run logs and their extracted rustfmt diffs, the two final job logs, the evidence extract, and this report's draft. Each file was disposable evidence for this run and was removed by exact known path at run end; the directory no longer exists. Nothing was removed from `.temp/` broadly, from any shared temp root, or from another session's files. Historical `proceed-while-running-*.log` files under the Cline temp area pre-date this run and were **not** deleted.

Retained deliberately: the inherited ignored scratch listed in §6 (Owner/inherited material with no cleanup trigger inside this order) and the existing `target/`. No other debris is known.

## 9. Status

**Implementation-agent claimed outcome: `LINUX-LIVE-CHILD-SESSION-ATTESTATION-PROVEN` on `solo-wo-087-r01` at `8250f742f533d99cea1c43739ea351095d4c9d73`, branch CI green on both hosts.** This is a claim submitted for independent review, **not** self-acceptance. No merge, cherry-pick, path promotion to `main`, pull request, tag, release or deployment was performed, and no further work order was begun.

`R03.C3`, `R03.C` and Stage 03 remain **OPEN**. All media-correctness, containment and publication flags remain `false`. The next authority is the Chief Master Architect's independent **ACCEPT / REVISE / STOP** against the exact branch and report revisions recorded here.
