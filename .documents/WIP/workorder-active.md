# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-082 R01**  
**Title:** R03.C3 — integrate the independently accepted Linux wall-deadline leader-kill/reap negative onto `main` by exact two-blob promotion  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-082 R01`  
**Authority:** This Owner-installed file is the sole executable `.documents/WIP/workorder-active.md` instruction after the Owner has installed it and explicitly transmitted its matching handoff. The Implementation Agent alone may perform the specified local filesystem, Git and GitHub changes. The Chief Master Architect remains **READ-ONLY** on GitHub. This order authorizes **integration of exactly two already accepted blobs**; it does not authorize new engineering, altered implementation bytes, a branch merge, a release, a decoder, or broader media/publication authority.

## 1. Decision, accepted input and fixed outcome

The Chief Master Architect independently **ACCEPTED** the narrow outcome `LINUX-WALL-DEADLINE-LEADER-REAP-PROVEN` from `SOLO-WO-081 R01` after inspecting the canonical report, real source and changed-path ledger, both-host GitHub CI and actual Ubuntu evidence of an owned child terminated by `SIGKILL` and reaped. The accepted implementation is preserved, not merged, at **exact** technical branch commit:

- Branch: `solo-wo-081-r01`.
- Accepted commit: `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`.
- Accepted tree: `2dabf3e5a5a9b3d2b067e627a3f63343296195c4`.
- Parent: `4aa96ea2239fd4b6cea91cf96b90c0457bd648eb`.
- Final branch manual-dispatch CI: run `43240118731`, `head_sha=cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`, `ubuntu-24.04` job `131257949845` **success**, `windows-2025` job `131257949636` **success**.
- The prior red branch run `43238905991` is disclosed history, corrected before the accepted revision. Do not erase it from an honest evidence ledger.

**Deliver only:** `LINUX-WALL-DEADLINE-LEADER-REAP-INTEGRATED` if—and only if—the exact two accepted blobs are on `main`, the normal push-triggered **complete** `rust-verify` workflow passes on Ubuntu and Windows at the exact integration commit, and the separate final report-only commit also passes on both hosts. A clean branch CI from WO-081 is **necessary background, not a substitute for mainline integration CI**.

**Precisely what is proven:** One *trusted, descendant-free* Linux child, already launched under the accepted fixed `setsid`/`RLIMIT_AS`/`RLIMIT_CPU` pre-exec boundary, verifies its own limits and group identity before a deliberate **20,000 ms sleep**. Its owning parent uses a **2,000 ms wall deadline**, calls safe `Child::kill` on **its own leader only**, obtains a real `SIGKILL = 9` wait status, reaps that leader, and emits a fixed bounded evidence line only after validation. Windows refuses the two wall modes without starting a child. Existing `N076`/`N079` behavior remains unchanged.

**Nonclaims—remain OPEN:** no descendants or process-group signalling/sweep; no hostile-child resistance; no generic process runner; no memory allocation or CPU-burn stress; no `RLIMIT_AS` or `RLIMIT_CPU` **exhaustion** proof; no aggregate memory quota or resident-memory claim; no Windows Job Object; no filesystem/network/credential sandbox; no immutable executable identity; no AVIF parse/decode or media verification; no publication authority, release, deployment or Stage 03 completion. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. The media-worker public-authority and media-correctness flags remain `false`.

## 2. Exact identity and authorization preflight — S082-01

**Before any implementation or Git write**, enter the Owner's **single existing** checkout, confirm the installed file begins `OWNER-ISSUED — SOLO-WO-082 R01` and the separate handoff is **exactly** `SOLO-WO-082 R01`, and read back *full* Git identities—not truncated display hashes. All must match:

| Anchor at start | Mandatory exact value |
| --- | --- |
| `HEAD`, `main`, `origin/main` | `13bf1c56dc1e974fb1acda7c86d9dca7fd74cae9` — all equal |
| `HEAD^{tree}` | `ec826f902736f0f2f3c4fc720fa6b36fe7ec3cb1` |
| Prior canonical report | `.documents/Reports/execution-report.md` blob `e5bc3309c1e53610784620d0584f0fb3acd49765` |
| Tracked, **pre-installation** ACTIVE | `.documents/WIP/workorder-active.md` blob `a9d2a84db8ef7cb2eb6d85d823aaefc0ecbf89b6` |
| Architecture Charter | `.documents/Architecture/ARCHITECTURE.md` blob `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT plan | `.documents/WIP/plan-next.md` blob `92fa562b1af0c54391851aa5c79deb1631127069` |
| ROADMAP plan | `.documents/WIP/plan-roadmap.md` blob `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| OUTLOOK plan | `.documents/WIP/plan-outlook.md` blob `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `rust-verify` | `.github/workflows/rust-verify.yml` blob `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` — both original `edges=41` pins |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| Adapter manifest | `crates/solo-adapters/Cargo.toml` blob `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| Adapter root | `crates/solo-adapters/src/lib.rs` blob `1550d34f820642bb5573e06d4fccb758109fd26c` |
| Linux native pre-exec seam | `crates/solo-adapters/src/media_worker_host_native.rs` blob `d10151e40e807c9831491ba0da0eb44d08a81589` |
| **Main's existing safe host façade** | `crates/solo-adapters/src/media_worker_host.rs` blob `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` |
| CLI root | `crates/solo-cli/src/main.rs` blob `b3c89ad0ce256f69e5763855559d20757554c461` |
| **Main's existing host integration test** | `crates/solo-cli/tests/media_worker_host_smoke.rs` blob `1f127001359e5f0ba941f92906f3280e433d8b9f` |
| Unsafe boundary guard | `crates/solo-cli/tests/unsafe_boundary.rs` blob `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| Graph checker | `crates/solo-cli/tests/architecture_checks/graph.rs` blob `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| Workspace graph tests | `crates/solo-cli/tests/workspace_graph.rs` blob `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| **Proof branch** | `solo-wo-081-r01` local and `origin/solo-wo-081-r01` both exactly `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; tree `2dabf3e5a5a9b3d2b067e627a3f63343296195c4` |
| Earlier proof branches | `solo-wo-079-r01` = `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` = `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Historical branches | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` = `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Git layout | one checkout, **one worktree**, exactly one remote `origin` to `https://github.com/provelopment/provelopment-foundation-solo.git`; no shadow code, worktree, clone or alternate remote |
| WIP layout | exactly `workorder-active.md`, `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md` in `.documents/WIP/` |

Use `git fetch origin` **without force** as required to establish remote refs; reject divergence, unknown histories, unexpected remotes and altered branches. The only preflight dirtiness allowed is the **Owner-installed replacement** of `.documents/WIP/workorder-active.md`. Verify its full content, `git hash-object` identity and exact filename. Every other modified, staged, unknown/untracked or unsafe item causes **STOP**—preserve it, do not reset, clean, stash, discard or hide it. Distinguish pre-existing ignored `.temp/`/`target/` from unrecognized work, inventory them under the SOP, and never execute or stage inherited scratch. The prior `.temp/solo-wo-073-wip/` contains **4 files / 46,062 bytes** as agent-reported unverified reference; confirm safely and preserve, do not treat it as accepted source. No archive is assumed.

Confirm the accepted branch contains **exactly** the two final target blobs given in §4, and that comparing its exact issuance ancestor `e725c29a0b8daaf5939084abe8ed015a2b3002b9` to its final commit yields only those two modifications. Confirm the accepted branch's ACTIVE is the prior issuance blob; **do not import its WIP, report or history**. Confirm `main` is code-identical to the WO-080 integration except for subsequent WIP/report commits. If any identity differs or local vs remote is uncertain, **STOP** and publish an honest canonical report if safe.

## 3. Mainline issuance first — S082-02

On `main` only, stage and commit **exactly** `.documents/WIP/workorder-active.md` containing this newly installed Owner file. One commit, one parent, one changed path; do not sneak any implementation or report into issuance. Push by ordinary fast-forward to `origin/main`; independently read back the exact commit SHA, tree, parent, path ledger and new ACTIVE blob from GitHub.

**Hard gate:** require the normal push-triggered `.github/workflows/rust-verify.yml` CI at the **exact issuance SHA** to conclude **success** on both `ubuntu-24.04` and `windows-2025`. Read exact run ID, event/head SHA, both job IDs and 33 configured steps per host; allowed skips are strictly the host-inapplicable steps. Never reuse the WO-081 branch run or a previous main run as issuance CI. If issuance CI is unavailable or red, **STOP**; do not promote code.

After issuance, ensure `HEAD == main == origin/main == <issuance SHA>` and working/index trees are clean. Do not create any new branch: this is an **integration-only mainline order**.

## 4. Only permitted implementation promotion — S082-03

From the **immutable accepted source commit** `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`, promote **exactly these two Git blobs** to the current clean `main` via path-scoped Git restore:

| Exact repository path | Existing main blob | **Required accepted blob** | Change |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | **`aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0`** | M; expected `775` additions / `4` deletions |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1f127001359e5f0ba941f92906f3280e433d8b9f` | **`1846af8459c21063bd2c2d7e112c05b9baf599ae`** | M; expected `255` additions / `0` deletions |

Authorized conceptual command, from the clean `main` issuance commit (use host-correct PowerShell syntax and **literal paths**, no wildcards):

```text
git restore --source=cacc0f9b10f6b43c38c4b72e8cda6e92310eef19 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

This is **not** authorization for `git merge`, `cherry-pick`, `rebase`, whole-tree checkout, reset, stash, force push, a new worktree, source copy from `.temp/` or editing any implementation byte. The source and tests must be identical in Git **object identity**, not merely equivalent behavior or normalized text. Do **not** apply rustfmt rewrites to the copied files. Only path-scoped restore from the **full pinned commit** is authorized.

Before committing, prove and record all of the following:

1. `git diff --cached --name-status` is exactly those **two modified** paths, neither added nor deleted, and no third path exists in either staged or unstaged diffs.
2. `git diff --cached --numstat` is exactly `775/4` for the façade and `255/0` for the test; `git diff --cached --check` is clean.
3. `git rev-parse :crates/solo-adapters/src/media_worker_host.rs` is `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0`, and `git rev-parse :crates/solo-cli/tests/media_worker_host_smoke.rs` is `1846af8459c21063bd2c2d7e112c05b9baf599ae`.
4. The two stage blobs also equal `git rev-parse cacc0f9b10f6b43c38c4b72e8cda6e92310eef19:<exact path>` individually.
5. `git ls-files -o --exclude-standard` is empty and no unrelated path is modified, staged, untracked or imported.

Commit **one integration commit** with only these two paths, e.g. `feat: integrate proven Linux wall deadline leader reap (SOLO-WO-082)`. Push `main` via ordinary fast-forward. Independently re-read the remote integration SHA, tree, parent **equal to issuance SHA**, exact two-path diff, and both final remote blobs. Require local `HEAD`, `main`, `origin/main` and GitHub commit to match and the working/index trees to be clean. If any equality fails, **STOP** before another write; do not patch accepted bytes to make a test pass.

## 5. Permanent path authority and invariants — S082-04

The **complete permanent changed-path ledger for the entire WO-082** is exactly four paths across three commits:

- `.documents/WIP/workorder-active.md` — **issuance commit only**, installed Owner bytes, one path.
- `crates/solo-adapters/src/media_worker_host.rs` — **integration commit only**, exact promoted blob.
- `crates/solo-cli/tests/media_worker_host_smoke.rs` — **integration commit only**, exact promoted blob.
- `.documents/Reports/execution-report.md` — **final report-only commit**, overwrite the sole canonical report.

**Everything else is prohibited from change**, including `.documents/Architecture/ARCHITECTURE.md`, SOPs, README/AGENTS, all three WIP plan files, other reports, any workflow, Cargo manifests/lockfile, `deny.toml`, toolchain, all other first-party source, `crates/solo-adapters/src/media_worker_host_native.rs`, `crates/solo-adapters/src/lib.rs`, `crates/solo-cli/src/main.rs`, architecture/unsafe guards and fixtures, protocol, media-worker code, domain/application and old source-read paths. Do not edit archived/historical branches or change any technical-branch commit.

Maintain the exact accepted invariants: the only Linux native file still has its existing four positively gated, locally justified `unsafe` expressions; no new native FFI, direct dependency, target-specific feature, process handle abstraction, OS signal action, new executable input, generic runner or public CLI contract. The two imported source files must hold zero executable `unsafe`. No `killpg`, process-group signalling, descendant fork/spawn or cleanup claim is authorized. The pinned `rust-verify` workflow remains unchanged with exact **41** dependency edges and **four** workspace crates. No fake PASS, relaxed guard, skipped CI step, downgraded safety requirement or unbounded test is acceptable.

## 6. Exact mainline integration evidence — S082-05 through S082-08

Run the **existing unmodified**, push-triggered `rust-verify` workflow at the integration commit and independently verify `event=push`, branch=`main`, `head_sha=<full exact integration SHA>`, run conclusion `success`, and **both** jobs `verify (ubuntu-24.04)` and `verify (windows-2025)` completed successfully. Each has 33 configured steps; each skipped step must be inapplicable to that host, never a hidden failure. Inspect the actual logs, not only job badges or the prior branch run.

The minimum mandatory observations are:

**G082-01 — identity and integrity.** GitHub's exact two-path commit ledger and two integrated blob SHAs equal §4; workflow blob is still `e1b5ca6e1738a5d5129a8788e24d51aee02421d1`; `Cargo.lock` still `b1ee56da60f36536548586528db371b7cbe508e4`; native module, adapter and CLI roots and guards remain their §2 blobs; accepted branch and older branches are unchanged. Zero additional dependency edges; architecture `packages=4 targets=33 files=60 edges=41` on **both** hosts.

**G082-02 — real Ubuntu wall deadline at *new mainline SHA*.** `n081_01_the_wall_parent_kills_and_reaps_its_own_overdue_child` and `n081_02_the_termination_is_the_short_deadline_not_the_full_sleep` run **and pass** on `ubuntu-24.04`. CI log includes a fresh `n081_wall=media_host_wall=terminated ...` line with exactly fixed `as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10`, positive `pid=pgid`, `signal=9`, `reaped=1`, `wall_ms` in the test's accepted short-deadline range, and timing evidence from `n081_wall_timing=`. **Never copy historical PID or wall time into the fresh result.** Report the actual newly observed values. This is leader kill/reap only; do not claim a group or tree sweep.

**G082-03 — Unix direct-negative and parser refusals.** `n081_03_a_direct_wall_child_and_every_wrong_wall_token_are_refused` passes on Ubuntu, with `n081_direct_wall_child=exit=...` (one of the established closed refusal statuses) and no success evidence or 20-second unsupervised sleep. Pure `n081_01`, `n081_03` and four `n081_04` tests run on **both** hosts; malformed/partial readiness, non-SIGKILL, un-reaped status, wrong profile/identity and timing violations remain refusals. Do not redefine these tests or their assertions.

**G082-04 — Windows fail-closed.** `n081_05_both_wall_modes_are_unsupported_before_any_child_exists` runs and passes on Windows: both modes return status **3** with exact `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED`, empty stdout, promptly and without a child. No `n081_wall=`, Linux `n081_01/02/03` integration names, Linux `n079_01/04` or `n076_` execution on Windows. Pure cross-platform refusals still run.

**G082-05 — all accepted regressions.** `N076-01..03` actual kernel readback; `N079-01` benign pre-exec child observation (`512 MiB`, `10 CPU s`, `pgid==pid`), `N079-04` direct-child refusal; `N079-03/05/06` expected cross-platform cases, private CLI mode and *unchanged public CLI bytes/exits* all pass on appropriate hosts. No process group, descendant, stress allocation, CPU burn or decoder introduced.

**G082-06 — architecture/security.** `architecture=legal packages=4 targets=33 files=60 edges=41` on both hosts with the original literal `edges=41` gates; `self-check=passed cases=29` and negative dynamic-load/checker-bypass refusals. Pinned Rust `1.99.1`, complete `cargo fmt --all -- --check`, strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, workspace locked all-target tests, and `cargo-deny 0.20.2` security gate with advisory refresh, bans/licenses/sources all pass on both hosts. Preserve negative advisory probe where applicable.

**G082-07 — downstream, restoration, reproducibility.** Both hosts pass the test-failure tripwire, release-build gate and release CLI smoke `accepted_exit=0 refused_exit=2`. Ubuntu's forbidden-import/bypass negative refuses with a nonzero status and restores the touched source byte-for-byte; Ubuntu's two independent clean release builds match on **raw binary bytes**. This is CI test-building, not release/deployment authority. Record raw checkout `Cargo.lock` SHA-256 separately by host, if observed; its cross-platform line-ending difference does not alter its one Git blob identity.

**G082-08 — CI exactness.** Integrating commits cannot be accepted using earlier branch CI or another branch/revision. Both host jobs must be green at the new integration SHA **and** both host jobs must be green again at the final report-only `main` HEAD. If a mandatory test is absent/filtered/skipped, a job red or run SHA differs, mark the gate **UNVERIFIED** or **BLOCKED**, preserve state and report honestly. No opportunistic test/tool/workflow fix is authorized by this integration order; a required source correction needs a new Owner decision.

## 7. Canonical report publication — S082-09

After **all** integration gates pass, return to (and remain on) clean `main`. Overwrite the **one** canonical `.documents/Reports/execution-report.md` with the actual WO-082 facts. Record:

- Exact WO ID, matching handoff, date and narrowly defined outcome; real preflight and Owner installation evidence.
- Full SHA/tree/parent and changed-path ledger for issuance and integration; new ACTIVE blob; each source blob before/after and staged/remote identity checks; no full-branch merge/cherry-pick.
- Integration run ID, event, exact head SHA, Linux/Windows job IDs, job conclusions, applicable step counts; fresh `n081_wall`/timing/direct refusal lines, Windows negative and `N076/N079` regressions, architecture/security/restore/reproducibility results.
- Actual changed path list across the order, immutable graph/workflow/manifest/native/guards, preserved branch names/full SHAs, single checkout/worktree/remote/four WIP files, clean working/index trees and positive mainline readback.
- Local inability to compile if applicable; CI evidence distinguished from local fmt/Git checks. Current host-specific raw lock hashes distinguished from Git blob identity.
- The earlier WO-081 red lint iteration as disclosed history **without falsely describing it as a WO-082 failure**. New failures, if any, must be disclosed with run/job/diagnostic.
- Inventory and cleanup of current task-owned temporary files in Solo `.temp/` and any controllable agent temp area. Preserve unknown/Owner files, inherited WO-073 four-file research scratch and existing `target/`; do not claim their contents verified. If cleanup is blocked, disclose it.
- Exact scope limits: **no group/tree kill, descendant cleanup, aggregate memory quota, stress, sandbox, Windows Job Objects, AVIF decode, publication, release, Stage 03 closure or new public capability**.

Publish **one separate report-only commit** on `main` (only `.documents/Reports/execution-report.md`) with parent equal to the green integration SHA; push ordinary fast-forward. **Do not put the final report commit's as-yet-uncreated SHA inside the report itself.** Verify via GitHub readback: exact report SHA/tree/parent, only one changed path, report blob and `main == origin/main == HEAD`, clean tree. Require normal push-triggered `rust-verify` success again at the exact **report HEAD** on Ubuntu and Windows, including all applicable steps. Report that run/job identity in the chat handoff.

**The Implementation Agent cannot self-ACCEPT**; the Chief Master Architect independently audits the report HEAD, exact code, CI and state to decide `ACCEPT / REVISE / STOP`. Do not issue or start a further order.

## 8. STOP / refusal and preservation rules

Immediately **STOP**, without bypass or cleanup that destroys evidence, for any missing/mismatched commit/tree/blob; changed/unknown tracked state beyond installed ACTIVE; differing local and remote refs; unknown remote/worktree; unauthorized branch movement; inability to establish accepted blob equality; a third implementation change; altered workflow, Cargo graph, guard, native source, CLI root or other non-ledger file; any request to run stress/decoder/hostile-child/descendant code; any negative or timeout proving only a fabricated status; missing readback-before-sleep proof; an unreaped process; any new real child or group cleanup risk beyond the accepted test; failed security or test gate; wrong CI head/event; a failed final report CI; publication/deployment attempt; secret exposure; non-fast-forward push; or destructive cleanup.

If integration cannot be safely completed, do **not** disguise it as `LINUX-WALL-DEADLINE-LEADER-REAP-INTEGRATED`. Where safe, use the existing standing canonical **report-only** allowance to publish a truthful `BLOCKED`/`UNVERIFIED` report from `main`, never merging unproven code or writing a parallel report. If safe report publication itself is blocked, preserve all work and explain the exact blocker in the final handoff. Do not use `git reset`, `git clean`, stash, force push, branch rewrite, secondary worktree, clone or shadow repo. Only delete positively identified *current-task-owned* temporary files; preserve inherited and Owner-owned material. Keep `main`, the accepted `solo-wo-081-r01`, earlier proof branches and all historical branches independently readable and unchanged except for this order's expressly authorized mainline commits.

**Terminal handoff:** exact current `main` SHA/tree, issuance/integration/report SHAs, trees, parents and blobs, exact two-path ledger, **both** integration and report-run IDs with two host job IDs each, fresh evidence, branch preservation, cleanup state, honest limitations, and explicit request for independent **ACCEPT / REVISE / STOP**. `R03.C3`, `R03.C` and Stage 03 stay **OPEN** regardless of this narrow integration outcome.
