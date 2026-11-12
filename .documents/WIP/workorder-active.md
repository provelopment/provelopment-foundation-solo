# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-078 R01**  
**Title:** R03.C3 — integrate the independently accepted Linux native `getrlimit` readback seam and its exact 41-edge CI contract into `main`, without widening capability  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-078 R01`  
**Authority:** This complete order becomes operative **only after the Owner installs it** at `.documents/WIP/workorder-active.md` in the sole checkout and delivers the matching handoff. The Chief Master Architect is **GitHub READ-ONLY**. Only the Implementation Agent may perform the bounded repository/Git actions expressly authorized here. This order is an explicit **separate Owner authorization to promote the eight exact, accepted implementation/CI blobs** listed below onto `main`; it is not blanket permission to merge branches or introduce new native operations.

## 1. Exact approved scope and endpoint

Promote to the real `main` branch **exactly eight files, byte-for-byte as already reviewed and successfully executed**, from the preserved technical branch `solo-wo-076-r01` commit `40a40bab1d017e61340fe53399b49fb2ab335c3b`. The source proves only Linux **readback of the calling process's already-existing** `RLIMIT_AS` and `RLIMIT_CPU` ceilings through `libc::getrlimit`; two Linux-gated `unsafe` blocks with adjacent `// SAFETY:` rationales; no process spawn, no `setrlimit`, no limit mutation, no signal, no OS supervisor, no AVIF parser/decoder and no public capability. The separate `.github/workflows/rust-verify.yml` blob changes **exactly two** assertions from `edges=40` to `edges=41`, preserving every strict checker, test, security, fail-closed, reproducibility and release-smoke step.

The outcome, if every gate passes **again on `main`**, is **`LINUX-NATIVE-READBACK-INTEGRATED`**, not a new resource-containment claim. The prior accepted result `LINUX-NATIVE-READBACK-PROVEN` is supported by branch run `43204471496` on both hosts, and this order must independently verify the **promoted, identical** bytes under normal push-triggered CI. It must NOT label the source `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED`, `SANDBOX-PROVEN` or a release. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; media-worker constants granting publication and proving correctness remain `false`.

**Why this is a single eight-file integration:** the new `solo-adapters → libc` Linux-only direct edge changes the real-tree count to **41**; the original `main` workflow pins **40**, while the accepted branch's workflow correctly pins **41**. Neither the code nor the revised workflow may be promoted alone. The branch also has its own historical ACTIVE file. Do **not** merge or cherry-pick the whole branch; select only the eight named paths. `main` retains this newly issued WO-078 ACTIVE and receives a fresh canonical report, not the branch's WO-077 ACTIVE or any older report.

## 2. Preflight — fixed source, target and topology anchors

Before *any* agent write other than the Owner-installed ACTIVE replacement, verify the actual sole checkout and compare **full hashes** (never infer an abbreviated SHA). Required initial condition: `main == origin/main == HEAD`, clean except for the Owner-installed complete `SOLO-WO-078 R01` ACTIVE replacement, one worktree and one remote `origin=https://github.com/provelopment/provelopment-foundation-solo.git`, precisely four files in `.documents/WIP/`, and no unrecognized tracked or untracked material. Confirm no previously installed `SOLO-WO-078 R01` issuance, no unrelated active session or unexpected in-progress Git operation.

| Item | Exact required value |
| --- | --- |
| `main == origin/main == HEAD` | `7576607a9603000e9a07b51aa056756302084e54` |
| `HEAD^{tree}` | `e1ff34d22af5b0c5b1c1b3a50a7a805e69228e18` |
| Canonical report blob | `4c51db7db3c89076e556c6dbda5bffd431060f7f` — accepted `SOLO-WO-077 R01` |
| Prior ACTIVE blob at HEAD | `39fafdd139b1a37d5fde7fd8d58e1202134fc531` |
| Architecture Charter blob | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK blobs | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `main` original workflow blob | `eb78dad240bb7adb3e0ef9535870897176c99843` — `edges=40` in two places |
| `main` original `Cargo.lock` blob | `2f9e53576df117f2020ae064d6973304f20833f1` |
| `main` original adapter manifest / crate root | `6109224cd3e3e89487884631f2b07c68ad8149c3` / `4c5732983ce5a0ced0a7ebc9e25af2e24149f5ee` |
| `main` original graph checker / unsafe guard / workspace graph | `268fcf9a33d7e41e304173586952c8341acfbda4` / `5c59a71bf4bf31808dde45be18fa123ba3fb7c44` / `69fa6837f52a1dff72a4a3bb0057021bf544aa4e` |
| Accepted branch name and tip (local **and** remote) | `solo-wo-076-r01` = `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Accepted branch tree | `84de1cbfc192336529a6b073641c8ef6a9e74247` |
| Branch predecessor / exact approved corrective commit | `b63284afda35d4b874758e2719117bfa88a8dbb4` / `40a40bab1d017e61340fe53399b49fb2ab335c3b` |
| Accepted branch CI | `rust-verify` run **`43204471496`** at exactly `40a40bab…`, `workflow_dispatch`; `131136025121` Ubuntu and `131136025260` Windows both success |
| Most recent main CI | `rust-verify` **`43205675510`** at `7576607a…`; both hosts success |
| Historic branches preserved | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` = `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |

Check the exact path set and all eight source blobs in §3 against the remote technical-branch commit. Verify that the worktree has **no** `crates/solo-adapters/src/media_worker_host_native.rs` on `main` before promotion and that the branch contains it. Verify direct branch-vs-main comparison; it is expected that the historical ACTIVE and report revision differ and are **not** included in the code promotion. The branch itself is an immutable evidence anchor for this order: no new commit, rebasing, force-push, deletion, change to branch tracking or further work on it.

**Required reading:** `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6A.3, 6A.8, both Solo SOPs (Architect §§6A–8; Implementation §§4–5C), all four WIP files, the exact WO-077 canonical report and WO-076 prior report from GitHub history, the accepted branch `Cargo.lock`, `solo-adapters` manifest/lib/native source, both guard/graph tests, `.github/workflows/rust-verify.yml` from **both** refs, `deny.toml`, `rust-toolchain.toml`, and the existing application `media_worker` boundary. Read the accepted branch's actual source; do not treat locally retained `.temp/solo-wo-073-wip/` research as authority.

**Local material:** inventory `.temp/` and actual task temp independently. The old WO-073 research files are inherited and unverified, reported as four files / 46,062 bytes in the preceding report; preserve them untouched, regardless of their current verified contents. The two previously mentioned Owner archives were absent from `.temp/` in the last two reports: do not fabricate, move, delete or presume their presence. Never use `git clean`, `reset --hard`, stash, rebase, force-push, second checkout, linked worktree, alternate remote or path-wide blind overwrite. Stop on unexpected state.

## 3. EXACT eight-path implementation and workflow promotion ledger

Only these eight permanently changed **code/infrastructure** paths on `main` are authorized, and the committed new blobs **must exactly match** the accepted technical-branch blobs:

| # | Repository-relative path | Exact required **post-promotion** Git blob SHA |
| --- | --- | --- |
| 1 | `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| 2 | `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| 3 | `crates/solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` |
| 4 | `crates/solo-adapters/src/lib.rs` | `bd1d4c58f5a91f38af63e84e143c850d6372ca64` |
| 5 | `crates/solo-adapters/src/media_worker_host_native.rs` (**new**) | `2df6d0e98395f21fc72294606e9a97e954f97c63` |
| 6 | `crates/solo-cli/tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| 7 | `crates/solo-cli/tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` |
| 8 | `crates/solo-cli/tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |

Additional two permanent paths **and only at their own phases**:

9. `.documents/WIP/workorder-active.md` — the Owner-installed **complete WO-078 text**. The agent commits it **unchanged and alone** as the issuance commit on `main`.
10. `.documents/Reports/execution-report.md` — replace the single canonical report once after the code CI result, with a **report-only** final commit on `main`.

Total permanently changed paths across `main` for this order must be **exactly 10** if completed: one ACTIVE + eight promoted implementation/CI paths + one report. The exact eight-path code commit must contain **no** other file. Preserve all other tracked paths and all three historical branch refs and the accepted technical branch tip unchanged. In particular, do not edit the Charter; either SOP; `README.md`; `AGENTS.md`; other WIP plans; the application/domain crate; `media_worker.rs`; `media_worker_protocol.rs`; `source_read_native.rs`; `architecture_checks/source.rs`, `mod.rs` or fixtures; `deny.toml`; toolchain; root Cargo manifest; any other workflow; or any publication code.

**Specific invariant review on staged code, before committing:**

- `Cargo.lock` changes only by the added `"libc",` dependency entry under `solo-adapters`, with existing exact resolved `libc 0.2.191`, `windows-sys 0.61.2` and all other packages/versions/sources/lock records unchanged.
- The adapter's direct `libc = "=0.2.191"` is **Linux-target-only**. The approved graph root/fixture knows that one new external edge. No normal dependency is added to an inward crate, no second native package is introduced, no Windows binding feature changes.
- Adapter crate root keeps `#![deny(unsafe_code)]`, unchanged Windows source-reader exception, and exactly the one Linux-specific module-scoped exception for `media_worker_host_native.rs`. The native file contains exactly two positively Linux-gated `unsafe` expressions, each locally justified, and only observes fixed `getrlimit(RLIMIT_AS/RLIMIT_CPU)` resource values. It does not spawn, change limits, send signals, touch a path, open input, publish or provide a general-purpose native API.
- Existing guards admit **only** the exact two native files and their respective positive OS gates; no other import/unsafe permission broadens. The previously accepted negative tests remain active.
- The workflow changes only at the two literal counts, lines corresponding to 660 and 672 of the accepted revision: `edges=40` to **`edges=41`**. Exactly two `edges=41` and zero `edges=40` assertions remain. Replacing the two literals back to `40` must reconstruct the old `main` workflow Git blob `eb78dad240bb7adb3e0ef9535870897176c99843`; do not relax the checker, skip steps, make the count dynamic or hide failures.

## 4. Exact Git/CI sequence — no implicit merge

**4A — Preflight and issuance on `main`.** In the only checkout, run `git rev-parse --show-toplevel`, `git status --porcelain=v1 --untracked-files=all`, `git rev-parse HEAD`, `git rev-parse 'HEAD^{tree}'`, `git worktree list`, `git remote -v`, `git branch -avv`, and `git rev-parse HEAD:<path>` for each anchor. Confirm complete installed ACTIVE and matching Owner handoff. Stage/commit **only** `.documents/WIP/workorder-active.md` with message `Issue SOLO-WO-078 R01: integrate accepted Linux readback seam`. Push fast-forward to `origin/main`, re-fetch/read back the issuance commit, parent, tree and ACTIVE blob, and require the original `rust-verify` to complete successfully on **both** Ubuntu and Windows at that issuance SHA. Do not begin code promotion on failing issuance CI.

**4B — One exact source promotion commit on `main`.** Still on a clean, fully synchronized `main` at the issuance SHA, with the accepted remote branch tip unchanged, materialize only the eight named paths from source `40a40bab1d017e61340fe53399b49fb2ab335c3b` using exact path-scoped Git restore/copy mechanics. This order **expressly authorizes** path-scoped `git restore --source=<accepted-sha> -- <eight exact paths>` **only after** a clean-worktree preflight and only for this source; it does NOT authorize a whole-tree checkout, a merge, a branch cherry-pick, a reset, or discarding any unknown edits. The previous approved branch ACTIVE must not be imported. Confirm that exactly the eight paths are changed, with one new native file, and that staged blob SHA for every path equals the corresponding §3 SHA (`git rev-parse :<path>` after staging). Verify `git diff --cached --check`, full staged name-status, original-vs-staged manifest and workflow diffs, and no other index/write artifact. Commit all eight paths **as exactly one commit** with message `feat: integrate proven Linux native readback seam (SOLO-WO-078)`. Push fast-forward to `origin/main`, read back code commit parent/tree/blob list and all eight blob SHAs remotely, and require CI on **the exact pushed code SHA** on both hosts.

**4C — Evidence, no fresh feature edits.** Run the **entire** unchanged-meaning `rust-verify` workflow at the code commit with both native hosts. It must exercise both platform's actual architecture checker and `edges=41` fixed pin, strict tests/guards, cargo-deny, failure tripwires, release build/smoke, and Ubuntu reproducibility/forbidden-import controls. On Ubuntu the three `N076` kernel readback tests must explicitly appear and pass; on Windows the Linux-only module must not compile or run and no `N076` test should appear. No new native work is needed or authorized. If CI fails, **do not alter any of the eight source blobs**, disable checks, skip a required step, make a second source correction commit, or declare success; report `UNVERIFIED` or `BLOCKED` with the precise failure and await a new Owner decision.

**4D — One report-only commit on `main`.** After confirming the code state, overwrite only `.documents/Reports/execution-report.md` with truthful current evidence. Commit and fast-forward push **only** this file, verify the exact report-head SHA/tree/parent/blob via GitHub and require **the complete two-host `rust-verify`** green again at the report SHA if the order is to be called complete. End on clean `main == origin/main`, one checkout/worktree, one remote, exactly four WIP files; branch `solo-wo-076-r01` remains at `40a40bab…` with no new commits, and the three historical branches remain unchanged. Do not delete the branch, do not merge, tag, release or deploy. Report every red run and changed SHA even if corrected within the permissible report-only scope.

**CI flow:** The workflow on `main` is triggered by normal `push`; no manual dispatch, extra workflow or rule edit is necessary or authorized. The normal `cargo fetch --locked` and pinned `cargo-deny 0.20.2`/RustSec workflow network accesses are the existing approved exceptions; no new downloads, dependencies, scripts, tools or provider access. Because the Owner-local host lacks a usable linker and Linux target, **remote two-host CI may be the sole actual compile/test evidence**. Never claim unperformed local compilation.

## 5. Mandatory acceptance matrix

| Gate | Required independent evidence |
| --- | --- |
| `S078-01` — identity & authority | Exact baseline main/branch SHA, trees, full blob anchors, sole verified checkout/remote, correct Owner-issued WO-078 handoff, only Owner-installed ACTIVE initially dirty, four WIP files, all historical refs unchanged, scratch inventoried. |
| `S078-02` — issuance | One ACTIVE-only issuance commit on `main` from `7576607a…`, exact remote readback and green `rust-verify` on both hosts at that issuance SHA. |
| `S078-03` — eight exact blobs | One code commit on `main` changes **exactly** §3 paths; each post-commit Git blob is identical to the named accepted branch SHA; no branch merge/cherry-pick, other source change or changed publication constant. |
| `S078-04` — component/unsafe graph | Four workspace crates, one Linux-only `solo-adapters → libc` normal edge, locked node/versions/source unchanged; real positive Linux source gate and exactly two justified native `unsafe` expressions, original Windows exception intact; negative AST/unsafe/workspace-graph fixtures remain green. |
| `S078-05` — workflow integrity | Exactly two strict literal `40 → 41` changes, byte-for-byte inverse proof of original workflow, no modified trigger/permissions/security step, no skipped or bypassed applicable check. |
| `S078-06` — Linux readback | On actual Ubuntu CI at exact code commit, `N076-01` (real `getrlimit` AS+CPU), `N076-02` (full-width soft/hard/repeat), `N076-03` (nonzero-status typed-refusal source-path test) each run `ok`. No limit installation or enforcement claim. |
| `S078-07` — Windows negative | Windows CI at code commit passes all normal gates and **does not compile or execute** Linux-native tests; no Windows native readback/supervisor claim. |
| `S078-08` — complete two-host regression | At code commit both CI jobs are **success** with all applicable workflow steps, `architecture=legal packages=4 targets=32 files=58 edges=41`, 29-case architecture refusal self-test, pinned `cargo-deny` advisories/bans/licenses/sources, strict format/Clippy/all-target tests, release build, smoke exit 0/2 and Ubuntu forbidden-import/restore and two-build reproducibility. |
| `S078-09` — report and cleanup | One report-only final commit, exact remote readback, two-host successful report SHA CI, clean single checkout on `main`, no branch movement, preserved WIP/plans/owner material and inherited WO-073 scratch with factual current inventory. |
| `S078-10` — accurate product scope | `LINUX-NATIVE-READBACK-INTEGRATED` only if all applicable gates passed; child launch, containment, Windows Job Objects, media validation, publishing, release and Stage 03 closure expressly unclaimed. |

**Mandatory host-observation correction:** a Git **blob** SHA for `Cargo.lock` must match the §3 value on both platforms; raw **checkout-file SHA-256** may differ between runner operating systems because of text-file line endings. Do not silently assert identical OS file-byte hashes unless logs verify it. The accepted WO-077 run showed Ubuntu's raw `Cargo.lock` SHA-256 as `bf421c6e…`, while the Windows job printed `afffb921…`; this is not license to change the lockfile. Report these distinctly from Git object identity and confirm they were not altered within each job.

## 6. Report contract, preservation and immediate STOP

The new single canonical report must include: this order ID and matching handoff, preflight full SHA/tree/blob inventory and local/remote branch status, the Owner-installed ACTIVE issuance SHA/tree/parent/blob, all eight original and promoted blobs with exact diff/path inventory, original and new workflow blobs and inverse test, the promotion commit SHA/tree/parent, Linux/Windows rust-verify run/job IDs at issuance, code and report commits, executable results of all three `N076` cases, actual real-tree count and 29-case self-test, locked-graph/cargo-deny evidence, each `S078` gate pass/fail/not-run, any failed or skipped mandatory steps, OS file-byte SHA distinction, all first-party unsafe occurrences admitted by this order, non-goals, `.temp/`/task-temp ownership/cleanup, preserved branch SHAs and final clean Git state. The report must identify the revision it describes, but must not embed its own not-yet-created final commit SHA. Return that final exact SHA/tree/blob to the Owner in chat.

**Retained scratch:** the inherited `.temp/solo-wo-073-wip/` material remains unverified and not authority. Do not copy, change or delete it in this integration task; preserve its cleanup trigger for a later native containment order. Preserve any Owner archives if present; record their actual inventory, not a historical assumption. Task-owned ephemeral material may be removed only after positive ownership verification; never use wholesale deletion or untrusted file matching.

**STOP with a safe factual report when possible** on: wrong identity or unauthorized checkout; missing/altered branch evidence; unexpected dirty path; changed remote/historical branch; failure of original issuance CI; promotion differing by **even one** blob byte or one extra path; inability to verify staged Git objects; `getrlimit` code needing change; native/unsafe/graph discrepancy; workflow diff not exactly the two pinned counts; new package/version, license or security violation; any mandatory CI failure or skip not explained by its OS-conditional counterpart; inability to prove runner jobs belong to the exact revision; unapproved process spawn or `setrlimit`; nondeterministic output or failed cleanup; secret/provider/network beyond authorized CI; any non-fast-forward push; or unsafe deletion. A green CI status at the report-only main commit without successful code-commit CI is insufficient.

**No further authority:** The agent must not start a Linux supervisor, Windows Job Object work, a codec, integration of the unfinished WO-073 scratch, a deployment, a release or the next order. The agent never self-ACCEPTs. The Chief Master Architect independently reviews the exact report, code revision and CI and returns **ACCEPT / REVISE / STOP**. The Owner alone installs and activates the next work order.
