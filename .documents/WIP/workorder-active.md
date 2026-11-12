# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-081 R01**  
**Title:** R03.C3 — prove a bounded Linux wall-deadline termination of one already-limited, trusted child (isolated technical-branch evidence only)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-081 R01`  
**Authority:** This entire Owner-supplied file is the sole executable scope only after the Owner installs its exact contents in `.documents/WIP/workorder-active.md` and delivers the matching activation prompt. The Chief Master Architect is permanently **GitHub READ-ONLY**; only the Implementation Agent may write local repository files, commit or push. This order neither merges any implementation into `main` nor authorizes a release, decoder or publication.

## 1. Approved decision, objective, evidence vocabulary

`SOLO-WO-080 R01` was independently **ACCEPTED** as `LINUX-PREEXEC-LIMIT-SMOKE-INTEGRATED`: `main` has one fixed, trusted `solo-cli` parent/child smoke test which installs a new Linux session and exact `RLIMIT_AS` 536,870,912 bytes and `RLIMIT_CPU` 10 seconds, hard and soft, in `pre_exec`, proves the child's own readback, and reaps the promptly exiting child. **No independent deadline-expiration/kill/reap negative has yet been proven.**

This order proves **one** deliberately overdue, non-descending, trusted Linux child is started **under those same verified limits**, then stopped by its owning parent's **2,000 ms wall watchdog**, and its leader is demonstrably reaped. The child may use only a fixed 20-second sleeping workload **after its own limit and process-group readback succeeds**. On Windows every new mode must refuse before process creation. Existing `N076` and `N079` behavior remains byte-exact.

**Only acceptable complete outcome:** `LINUX-WALL-DEADLINE-LEADER-REAP-PROVEN`, solely if every `N081` required case executes on the real Ubuntu host and the complete unchanged two-host `rust-verify` passes at the **exact final code-branch SHA**. On a real necessary failure use `BLOCKED`; when evidence is incomplete use `UNVERIFIED`, even if source compiles. A green issuance/report-only `main` job is not Linux deadline evidence. The code stays on `solo-wo-081-r01`; there is no product-code merge in this order.

**Nonclaims — mandatory in the report:** no child-tree or descendant sweep, no process-group kill, no hostile-child resistance, no combined/aggregate memory quota, no `RLIMIT_AS` memory-exhaustion proof, no `RLIMIT_CPU` busy-loop proof, no Windows Job Objects, no process/container sandbox, no executable-identity or anti-replacement proof, no filesystem/network confinement, no AVIF decode, no media correctness, no publication, no release, and no closure of `R03.C3`, `R03.C` or Stage 03. A 20-second sleep exercises **wall time**, not CPU exhaustion. The pre-existing child is trusted and not allowed to fork.

## 2. Exact repository and authority preflight — S081-01

Work **only** within the single Owner-local checkout. Before any Git change other than the Owner-installed ACTIVE working-file replacement, read and record exact `git status --porcelain=v1`, `git worktree list --porcelain`, `git remote -v`, all branch refs (`git show-ref --heads` and `git ls-remote --heads origin`), `.documents/WIP/` file inventory, `.temp/` and Cline task-temp inventory, and `git fetch`/`origin/main` identity. Never silently discard or overwrite unknown files. Verify precisely:

| Anchor | Required exact value |
| --- | --- |
| `HEAD == main == origin/main` | `86ba671e354a3a5cac2d83e5248493312fdee62b` |
| `HEAD^{tree}` | `80eeaf3caf88f2a88bc3a44a8abe9e7ccb120b7d` |
| Canonical prior report blob | `346b8fe573dc61fa2e9ad051b1d08031730b1c02` |
| Previous ACTIVE tracked blob | `c8ce3bb9c664a873bc200c6d7fdec2fdb8637a30` |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Workflow | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` (exact two `edges=41` gates) |
| Root `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| Adapter manifest | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| Adapter crate root | `1550d34f820642bb5573e06d4fccb758109fd26c` |
| Linux native readback/pre-exec file | `d10151e40e807c9831491ba0da0eb44d08a81589` |
| Safe host façade | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` |
| CLI root | `b3c89ad0ce256f69e5763855559d20757554c461` |
| Host smoke integration test | `1f127001359e5f0ba941f92906f3280e433d8b9f` |
| Unsafe guard | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| Architecture graph / workspace graph | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` / `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Earlier branch `solo-wo-079-r01` | `380393ddacf37fc1a6398805235ca2d5dbd9677e` |
| Earlier branch `solo-wo-076-r01` | `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Historical branch `solo-wo-061-r01` | `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Historical branch `solo-wo-069-r01` | `e6162cdf733ea3136e96cbc139e46a6daf4054ee` |
| Historical branch `solo-wo-070-r01` | `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| New branch `solo-wo-081-r01` | **ABSENT both locally and remotely** before creation |
| Prior report CI | run `43232869223` at `86ba671e…`: Ubuntu `131232468033` and Windows `131232467666` success |

Require exactly one checkout/worktree and one `origin` pointing to the stated repository, exactly four WIP files (`workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`), and **only** the Owner-installed ACTIVE replacement dirty at preflight. Confirm no other tracked, staged or unknown untracked work. The working ACTIVE must say `OWNER-ISSUED — SOLO-WO-081 R01`, include the exact matching handoff, be complete, and not be inferred from WIP planning files. Check `.temp/solo-wo-073-wip/` if still present: the inherited four-file, 46,062-byte scratch is **uncompiled, unverified reference only**; preserve it unless the Owner issues its cleanup. Do not fabricate missing historical installer archives. Record current `target/` state without destructive cleanup. Any anchor/scope mismatch: STOP and publish an accurate report if safe.

## 3. Two-branch history strategy and path authority

**On `main`, the ONLY authorized permanent paths:**

1. `.documents/WIP/workorder-active.md` — Owner-installed ACTIVE issuance **only**.
2. `.documents/Reports/execution-report.md` — overwritten canonical report in **one separate report-only final commit**.

**On `solo-wo-081-r01`, the ONLY authorized technical implementation paths (two):**

1. `crates/solo-adapters/src/media_worker_host.rs` — extend the existing *safe* fixed-mode host façade, its pure parsers/formatters, and internal unit tests for the one overdue-child case.
2. `crates/solo-cli/tests/media_worker_host_smoke.rs` — real binary Linux/Windows integration tests and strictly bounded CI-visible proof output.

The binary already dispatches its existing closed `run_internal_media_host_probe_v1` function before the public CLI. **Do not edit `crates/solo-cli/src/main.rs`**, adapter `lib.rs`, or the native module to add this case. Reuse the existing `install_linux_child_limits_v1`, `read_limit_ceilings_v1` and `own_process_group_v1` functions unchanged. No new FFI or `unsafe`, no additional direct dependency, new target, workflow edit or checker relaxation is needed or permitted. Absolute authorization boundaries: leave untouched `.github/workflows/*`, `Cargo.lock`, all Cargo manifests, `deny.toml`, `rust-toolchain.toml`, `source_read_native.rs`, `media_worker_host_native.rs`, all architecture guards and fixtures, the Charter, SOPs, WIP plans, protocol, domain/application crates, every public/media source and all historical refs. Any necessary third path is a STOP requiring a new Owner decision, **not** implicit authorization.

**Required sequence:**

A. On `main` with the Owner-installed complete ACTIVE and verified baseline, commit **only** `.documents/WIP/workorder-active.md`, no other file. Ordinary fast-forward push; independently read back issuance commit/tree/parent/blob/one-path ledger from GitHub; require a fresh normal `rust-verify` run on Ubuntu and Windows **green** at this issuance SHA before implementation. Do not put a report in the issuance commit.

B. Ensure `solo-wo-081-r01` is genuinely absent. Create it **once**, from the exact issuance SHA, within the **same single checkout** (`git switch -c solo-wo-081-r01`), no clone/worktree/shadow source. Prepare only the two authorized technical files; format and `git diff --check`; commit transparently to the branch and push it non-force. **Unverified bounded source is explicitly authorized on this isolated technical branch** to obtain real two-host CI compilation; local toolchain inability is not a reason to abandon the code. This permission **never** extends to `main`.

C. The existing workflow declares `workflow_dispatch:` and accepts an explicit branch ref. Dispatch the existing `rust-verify` against `solo-wo-081-r01` after pushing a code revision; read the actual workflow-run `head_sha` and require it to equal the exact candidate branch SHA. Review both job logs, fix only authorized paths on subsequent traceable branch commits if necessary, rerun until all mandatory gates are evidenced or an honest STOP. Keep initial/red runs in the report. Do not edit workflow trigger or replace it with fabricated/test-only CI.

D. Preserve the final code branch and exact remote SHA. Return to a clean `main` at the **issuance** SHA, without merging/cherry-picking/rebasing/fast-forwarding/patching the branch's implementation onto `main`. Overwrite and commit **only** `.documents/Reports/execution-report.md` in one report-only main commit; normal fast-forward push, GitHub commit/tree/blob/readback and **fresh green two-host** `main` CI. The final report must not embed its own future commit SHA. End at clean `HEAD == main == origin/main` (report commit), one worktree, one remote, four WIP files and preserved branches. On STOP, publish a truthful `BLOCKED` or `UNVERIFIED` report when Git safety permits; do not fake an acceptance.

## 4. Single bounded wall-deadline capability (implementation contract)

Keep existing `--internal-media-host-probe-v1 parent|child` byte-identical and all `N076`/`N079` tests unchanged. Add **only two closed internal submode tokens** beneath that already reserved flag:

- `wall-parent` — one trusted supervising parent for the wall-timeout negative.
- `wall-child` — a deliberately overdue **one-process, descendant-free** child.

All other token shapes must continue to refuse with the existing fixed `INTERNAL_MODE_INVALID` code; neither token may be discovered through the public bootstrap CLI, an environment variable, manifest, parsed media or authored path. `wall-parent` may re-execute only `std::env::current_exe()` (trusted **test harness assumption**, not a production trust root), with the exact reserved flag and `wall-child`, `stdin` null, stdout/stderr piped with the **existing 1,024/512-byte caps**, no shell and no PATH search. Use the **unchanged** native pre-exec installer so the child receives a new session/group, `RLIMIT_AS` soft/hard `536870912`, and `RLIMIT_CPU` soft/hard `10` before `exec`. The parent must never execute arbitrary commands or adopt caller-supplied limits.

**Mandatory stress-before-limits rule:** the `wall-child` must first call the already-proven kernel readback, verify all four fixed ceiling values, read `getpgid(0)` and require positive `pgid == own PID`, **before any deliberate sleep**. Direct `wall-child` invocation without those conditions must refuse promptly, with **zero stdout** and **no** 20-second sleep. Only when the readback is valid may the child write **one** fixed, bounded, strictly parseable readiness/limit proof line and flush it; then sleep **exactly 20 seconds** via safe Rust (`std::thread::sleep`). The child must spawn **no descendant**, allocate no stress buffers, burn no CPU, perform no media work, open no source, perform no network access and launch no other executable. It should have an explicit zero-exit fallback only if the parent violates its deadline; such an outcome must **not** count as termination proof.

**Parent negative:** use a **2,000 ms** wall deadline (not 15,000 ms) for this exact mode, with an ordinary safe-Rust `Instant` watchdog and small bounded poll interval. The timer starts before `Command::spawn`; it must not accept a late or unexpectedly successful child as a proven timeout. When the deadline is reached and the exact `Child` handle is still live, call **`Child::kill()` on that handle only** and `Child::wait()` to obtain the leader's real termination status and reap it. A successful proof requires a confirmed signal termination **`SIGKILL = 9`** (via safe `std::os::unix::process::ExitStatusExt`), successful `wait`/reap, and a valid strictly parsed, bounded readiness line from the child's actual kernel readback. Kill or wait errors, missing/incorrect signal, absent/extra/malformed/non-UTF-8 readiness bytes, wrong limits, `pid != pgid`, a child that exits before deadline, or an overrun that cannot be safely reaped are all failures/refusals, **never** acceptance. Carefully handle the exit-vs-timeout race without reporting an ordinary exit as a watchdog kill. Output reader threads must be joined **after leader termination**, and errors or caps must fail closed. Under this order there is **no process-group kill**; never signal a guessed PID, unknown process or other group. No second child and no descendants are permitted.

Use a separate, unmistakable fixed test-evidence marker such as `media_host_wall=terminated` (never `media_host_probe=accepted`) **only after** all proof conditions hold, with bounded structured fields sufficient for CI evidence: the original child's full-width `as_soft`, `as_hard`, `cpu_soft`, `cpu_hard`, its positive `pid` and matching `pgid`, `signal=9`, `reaped=1`, and elapsed milliseconds. For a verified wall-negative, the internal diagnostic mode may exit `0` to mean **the negative test succeeded**, but the evidence must explicitly state `terminated`; it must **not** claim the child completed successfully, media correctness or host containment. Every nonproven outcome emits no accepted evidence and one closed refusal line with status `3`, without raw OS error text or untrusted child output. Keep output concise, fixed and bound to the existing safe façade. A previously accepted `parent` success line must not change.

The wall test's observed duration must be consistent with a real 2-second timeout, rather than immediate refusal: use a robust non-zero lower bound (e.g. at least 1,500 ms) and a bounded upper check (e.g. below 10,000 ms) appropriate to CI scheduling; the code's **configured** deadline remains exactly 2,000 ms, not a claim of hard real-time scheduling. If the host is too slow or unpredictable to demonstrate safe reaping within a reasonable bound, mark it UNVERIFIED and report observations; never widen the deadline silently.

## 5. Mandatory `N081` real-executable tests and both-host gates

Add tests to the existing `crates/solo-cli/tests/media_worker_host_smoke.rs`; keep all `N079` tests and their exact prior assertions. Pure unit tests inside the authorized safe façade are permitted only to prove the closed parser, mode classification, timeout outcome mapping, and evidence rejection; they are **not** substitutes for the real Linux process.

| ID | Required proof | Host |
| --- | --- | --- |
| `N081-01` | Real `solo-cli --internal-media-host-probe-v1 wall-parent` spawns exactly one pre-exec-limited child; child first verified all four limit halves and `pgid == pid`; parent sees verified readiness, reaches the 2,000 ms watchdog, kills its **own** child, obtains `signal=9`, reaps the leader, and prints one fixed `media_host_wall=terminated ... signal=9 reaped=1 ...` evidence line with no leaked stderr | Ubuntu real kernel |
| `N081-02` | Evidence and test assertions show a real elapsed duration compatible with the short deadline and materially less than a 20-second sleep; **no normal child completion or timeout-free acceptance** can substitute; reaping must be evidenced by `Child::wait` status rather than inferred only from message text | Ubuntu real kernel |
| `N081-03` | Direct `wall-child` without the installed limits refuses **before sleeping** with zero stdout and prompt nonzero status; wrong/missing/extra/duplicated wall-mode tokens refuse, and legacy parent/child modes retain exact existing output and status | Ubuntu real binary + cross-host pure tests |
| `N081-04` | Every malformed/duplicate/oversize/negative/overflow/out-of-profile readiness and a non-`SIGKILL`, premature-normal-exit, unreadable stream, kill/reap failure or mismatched identity is a closed refusal in source and pure tests; never output accepted proof on failure. Do not cause uncontrolled real kill/reap failures solely to simulate them | Both hosts, pure where appropriate |
| `N081-05` | On Windows both new wall modes refuse `HOST_UNSUPPORTED` before spawning any child; **zero** Linux native or real Linux wall test names execute. Public CLI still passes its accepted/refused byte contracts | Windows real binary |
| `N081-06` | Existing `N076-01..03`, `N079-01..06`, protocol, source reader, architecture and unsafe-boundary tests all stay green; `Cargo.lock`, manifest, native module, CLI root, workflow and every checker remain unchanged; real-tree output remains `architecture=legal packages=4 targets=33 files=60 edges=41` and `self-check=passed cases=29` on both hosts | Both hosts |
| `N081-07` | `cargo fmt --all -- --check`, strict all-target Clippy, `cargo test --workspace --all-targets --locked`, release build, original release CLI smoke, component security (`cargo-deny 0.20.2`, fresh RustSec), 29-fixture refusal checks, failure tripwires, Ubuntu forbidden-import/restore and raw-byte reproducibility run to completion at the **exact code branch SHA** | Ubuntu + Windows CI |
| `N081-08` | Accurate proof boundaries: one terminated and reaped leader **only**, no child-tree sweep, group-signal, stress memory/CPU, broad sandbox, Windows process control or media/publication authority | Report/source/log |

The test must print the **actual validated** bounded observation to the CI log so an independent reviewer can see the limits, wall duration, termination signal and reap status, as `N079` already does. Do not print arbitrary child stderr, local paths, raw errors or secrets. CI must use `workflow_dispatch` on the branch, tie `head_sha` to its exact final code SHA, and show full job conclusions on both hosts. A green job at an earlier branch commit does not accept later modified code; disclose and fix all red iterations if safe.

## 6. Nonnegotiable failure handling and invariants

- **Native isolation:** No new `unsafe`, no `libc` import or OS signalling code in the safe façade; the existing `media_worker_host_native.rs` remains byte-identical. All child killing uses the owned safe `Child` handle. No `killpg`, `setsid` change, `setrlimit` change, `fork`/`exec` API, `prctl`, `ptrace` or other OS capability expansion.
- **Dependencies/architecture:** Exactly four workspace crates and the existing 41 edges. No new packages, versions, feature flags, target declarations, toolchain/workflow/deny changes or widening of `unsafe_boundary`/AST source rules. Keep all old negative fixtures load-bearing and original public CLI byte-exact.
- **Safety:** Do not run the fixed 20-second sleep until the real child has verified the limits; do not use a shell, PATH program, untrusted binary, arbitrary PID or external group; never mask a `kill`/`wait` error or hang an unbounded pipe join. If a child cannot be contained/terminated/reaped as required, **STOP**, preserve diagnostic state safely and report, not PASS. No cleanup command may target Owner or unknown processes.
- **History:** One sole checkout/worktree and `origin`, one new technical branch and no rewrite/force, no code on main, no accidental history-branch movement. No extra report archive, WIP file or external output. Native scope is not a reason to touch stage/media/public paths.
- **Local build limitation:** If the workstation still cannot link or compile Rust locally, record that plainly. Format and Git source checks locally if available. **Branch CI is explicitly the approved compiler/test host** and iterative bounded pushes to this branch are allowed. Do not label local `cargo metadata` as proof of compiling or running the child.
- **Scratch:** `.temp/` is ignored, never tracked/staged/force-added. Inventory both Solo and Cline task-owned temporary output; delete only positively identified current-run disposable items. Preserve inherited `.temp/solo-wo-073-wip/` (4 files / 46,062 bytes if unchanged), Owner files and any other unfamiliar material; do not run that inherited experimental FFI. No `git clean`, destructive reset, stash, shadow checkout or workspace deletion.

## 7. Mandatory gate sequence and canonical report — S081

| Gate | Required condition |
| --- | --- |
| `S081-01` | All anchored preflight identity, clean worktree except Owner ACTIVE, exact four WIP files and safe branch/remote topology |
| `S081-02` | ACTIVE-only issuance commit and readback; issuance `main` CI green on both hosts |
| `S081-03` | One `solo-wo-081-r01` technical branch created from issuance, only the **two exact** technical files changed |
| `S081-04` | Fixed wall child verifies pre-exec ceilings/group before sleeping, no descendants/untrusted execution; only safe façade/test edits, no new unsafe |
| `S081-05` | `N081-01..04` real Linux kill/reap plus pure refusals with actual bounded CI-visible readback/signal/wall evidence |
| `S081-06` | `N081-05` Windows negative and legacy public/private CLI outputs stay byte-exact |
| `S081-07` | Full original two-host `rust-verify` green at **exact final branch SHA**, all downstream checks and `N076`/`N079` regressions pass |
| `S081-08` | Technical branch preserved at its readback-verifiable remote SHA, **not merged into main**, older branches unchanged |
| `S081-09` | One report-only final `main` commit (plus separate ACTIVE issuance), final two-host `main` CI green, clean one-checkout/one-remote/four-WIP end state and proper scratch cleanup |
| `S081-10` | Narrow outcome vocabulary, complete transparent CI history, no fabricated host-enforcement/security/media/publication/Stage 03 claim, no self-ACCEPT or new order |

Overwrite the **one canonical** `.documents/Reports/execution-report.md` on `main`; no parallel report. Report exact start and issuance/report/code-branch commit and tree SHAs, ACTIVE and report Git blobs, precise changed paths and blob before/after for the two source files, exact unchanged lock/native/workflow/guard identities, every branch commit and red CI, final run/job IDs and head SHA, Linux `N081` observed child-readback/pgid/wall/signal/reap details, Windows unsupported results, regression/security/release/smoke/architecture evidence, `unsafe` inventory, preserved branches, scratch/cleanup inventory, and any limitation or blocker. The final report must not include its own future commit hash. Return the final GitHub report link and full commit identifiers in the chat handoff.

**Immediate STOP** on a mismatched Owner order/handoff or preflight anchor, second checkout/remote, unknown dirty files, unexpected code path, missing/unsafe positive limit proof before the deliberate sleep, unbounded/unreaped child or pipe reader, unauthorised signal or process kill, failure to distinguish timed-out child from normal exit, false positive status, unsafe operation outside the approved native file, workflow/checker/dependency change, failed mandatory host/CI gate, branch run with mismatched SHA, non-fast-forward push, failed cleanup touching unknown files, media/publication/release attempt, or impossible readback. Return `BLOCKED` or `UNVERIFIED`, not a partial PASS. The Implementation Agent cannot self-ACCEPT or begin another order; the Architect's separate disposition is required before any promotion to `main`.
