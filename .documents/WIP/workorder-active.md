# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-077 R01**  
**Title:** R03.C3 — reconcile the exact native-readback CI edge-count pin and complete independent two-host proof of the preserved Linux `getrlimit` seam (workflow-only correction on existing technical branch)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-077 R01`  
**Execution authority:** This complete `OWNER-ISSUED` order becomes operative **only after** the Owner installs it at `.documents/WIP/workorder-active.md` in the sole verified checkout and explicitly hands off `SOLO-WO-077 R01`. The Implementation Agent alone makes the specifically authorized local/GitHub writes. The Chief Master Architect has GitHub **READ-ONLY** access. Do not self-issue, self-ACCEPT, merge, deploy, release, or start a following order.

## 1. Approved scope — single cause, single correction

Correct precisely **two stale literal approved-edge-count assertions** in the existing `.github/workflows/rust-verify.yml` on the **preserved technical branch** `solo-wo-076-r01`: the Linux real-tree checker assertion at baseline workflow line 660 and its Windows counterpart at line 672 must each require `edges=41`, **not** `edges=40`. Nothing else in that workflow may change: preserve the requirement for `architecture=legal packages=4`, the real checker process exit gate, every fail-closed error path, the 29-case self-test checks, the security gates, all pinned versions/actions, existing jobs, branches, triggers, permissions, negative controls, release build, and every test and smoke step. The count remains an **exact fixed count**, not a wildcard, substring `edges=4`, greater-than comparison, derived leniency, suppressed failure, or disabled gate.

Why this is an authorized correction: the accepted locked graph already contains `libc 0.2.191` as a transitive package, and `SOLO-WO-076 R01` added **one and only one new direct Linux-target `solo-adapters → libc` edge** on isolated branch `8e1d5645835e50850d575de1c22d463c1efe7980`. Real Ubuntu and Windows checker logs at that exact branch SHA report `architecture=legal packages=4 targets=32 files=58 edges=41`. The checker itself accepted the graph, but the separate workflow count assertion stopped both jobs before downstream tests/build/tripwires. This work order adjusts only the two stale workflow expectations and reruns the **unchanged, real** guards. It grants **no** new dependency, permission, source edit, unsafe allowance, media verifier, worker supervisor, decoder, process launch, process limit mutation, publication, release, merge, or deployment.

**Outcome vocabulary:** `LINUX-NATIVE-READBACK-PROVEN` only if the preserved native source and exact locked graph are unchanged, both corrected branch jobs finish successfully, and their logs directly re-prove the three real Ubuntu `N076` tests plus Windows negative/untouched behavior. The statement is narrowly about reading the *calling process's existing* Linux `RLIMIT_AS`/`RLIMIT_CPU` values through `getrlimit`, not setting/enforcing a child's limits. Otherwise use `BLOCKED` for an observed necessary failure or `UNVERIFIED` for absent/indeterminate evidence. **No** `LINUX-HOST-CONTAINMENT-PROVEN`, `HOST-CONTAINMENT-PROVEN`, `MEDIA-VERIFIED`, sandbox/escape proof, product adoption, or Stage 03 closure. `R03.C3`, `R03.C`, and Stage 03 remain **OPEN**.

## 2. Immutable anchors and mandatory reading before edits

Verify the actual checkout, remote identity, all named refs and file blobs **before** editing or creating a commit. The sole permitted initial working-tree difference is the Owner-installed replacement of ACTIVE with this exact order.

| Evidence | Expected exact value |
| --- | --- |
| `main == origin/main == HEAD` before issuance | `37832b7a6d79a0c5cbb6530d45ea83d8212f4641` |
| `main` tree | `8967116c3201d6d94a64a3ff69bd6b78c30b28a5` |
| `main` canonical report blob (`SOLO-WO-076 R01`, `BLOCKED`) | `cb56f54b2a1a647831496123764f6f48ffd8417b` |
| Prior ACTIVE blob at HEAD | `9ba2a7937396a3032a393a31500909c2e8158586` |
| Architecture Charter blob | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Existing technical branch (local and `origin`) | `solo-wo-076-r01` = `8e1d5645835e50850d575de1c22d463c1efe7980`, tree `363f0f99b6666e18c1961992646959b59fcb39da` |
| Technical branch immediate parent | `f1bd7ccd72d144e4e9c403f1813c5e83605f7168` |
| Technical branch base (WO-076 issuance) | `5b085d4a0d9b259da80021adb4d8c133462d4616` |
| Workflow at both `main` and technical branch | `eb78dad240bb7adb3e0ef9535870897176c99843` |
| Native readback file at technical branch | `crates/solo-adapters/src/media_worker_host_native.rs` = `2df6d0e98395f21fc72294606e9a97e954f97c63` |
| Technical branch `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| Technical branch adapter manifest / crate root | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` / `bd1d4c58f5a91f38af63e84e143c850d6372ca64` |
| Technical branch graph / unsafe guard / graph tests | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` / `b88e7b42ab03bd78557e58d4c54b957b59369d6c` / `25a91b2c212805633fcf0fd80e4aaea11c595ba3` |
| Historical branches (must not move) | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` = `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Previous main CI at report HEAD | `43201596763`: Ubuntu job `131126131562`, Windows job `131126130631`, both success |
| Previous branch blocker | `43200373681` at `8e1d5645…`: Ubuntu job `131121910857`, Windows job `131121911134`, both fail only at the workflow's real-tree approved edge-count assertion after earlier gates pass |

Read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§6A.3/6A.8, `.documents/SOP/sop-architect.md` §§6A/6B/7/7A/7B and `.documents/SOP/sop-implementation.md` §§4–5C, **all four live WIP files**, the current canonical report, the precise failing job logs, the full workflow around its architecture and downstream gates, the technical branch's seven-path diff and native source, the Cargo graph guard, unsafe-boundary guard, two-host CI run/jobs, and its locked Cargo manifests. The prior agent's `.temp/solo-wo-073-wip/` is **unverified research scratch**, not execution authority or code to copy.

## 3. Exactly authorized permanent writes and their branches

**On `main`:**

1. `.documents/WIP/workorder-active.md` — the Owner alone installs this order. The Agent only stages and commits it **byte-for-byte unchanged** as a one-path issuance commit, after preflight.
2. `.documents/Reports/execution-report.md` — replace the one canonical report only after the technical-branch proof or a genuine STOP; publish one report-only commit on `main`.

**On the existing `solo-wo-076-r01` technical branch, and nowhere else:**

3. `.documents/WIP/workorder-active.md` — **one exact, explicitly authorized `git cherry-pick -x` of the new WO-077 ACTIVE-only issuance commit from `main`**, to bring the branch's current ACTIVE slot into agreement with the Owner-issued WO-077 handoff. The branch currently contains WO-076 in that slot; this specific copied issuance is necessary before running new work there. Do not cherry-pick the WO-076 report, later main commits, product code, another branch or anything else. If the exact cherry-pick conflicts or changes any other file, STOP rather than resolving a broadened diff. The branch's copy of ACTIVE must have the same blob as newly issued main ACTIVE.
4. `.github/workflows/rust-verify.yml` — **one workflow-only commit** changing exactly these two source occurrences and no other byte:
   - Linux real-tree step, current line 660: `grep -q 'edges=40' "$RUNNER_TEMP/arch-real.txt"` → `grep -q 'edges=41' "$RUNNER_TEMP/arch-real.txt"`.
   - Windows real-tree step, current line 672: `if ($text -notmatch 'edges=40') {` → `if ($text -notmatch 'edges=41') {`.

The observed checker count `41` is not inferred from a test stub; require the actual locked graph and real-tree checker evidence. Prove a **two-occurrence, literal-only diff** against the untouched workflow blob. Leave all errors, actual compiler/test commands, OS conditions, `workflow_dispatch:`, permissions and pinned dependencies intact. Do not modify `.github/workflows/rust-verify.yml` on `main`, because `main` still has `edges=40` and no Linux direct `libc` edge.

**All other permanent paths are READ-ONLY**, including the seven previously committed technical-branch code/graph/guard files; no edits whatsoever to Cargo manifests, `Cargo.lock`, application, domain, adapter code, native `unsafe`, test source, graph checkers, Architecture Charter, old source reader, `deny.toml`, Rust toolchain, other workflows, any other WIP file, About/README/AGENTS/SOP, fixtures, proof branches or historical branch refs. No new Git branch, worktree, clone, tag, PR, merge, rebase, force-push, history rewrite, cross-repository write, release or deployment. This order grants exactly the single issuance cherry-pick described above as a bounded exception; it grants **no general cherry-pick permission**.

Use only the verified Solo `.temp/`, ordinary ignored `target/` cache and the actual Cline-provided task scratch for disposable captures. Verify `.temp/` remains ignored and untracked; do not touch the four inherited WO-073 scratch files (previously 46,062 bytes) or any Owner archives if present. Record current inventories without presuming prior local archive files still exist. Clean only identified current-run scratch, never delete unknown Owner material.

## 4. Mandatory sequence (Git operations delegated to Implementation Agent)

**4A — Full preflight.** In the one real checkout use `git rev-parse --show-toplevel`, `git rev-parse HEAD`, `git rev-parse 'HEAD^{tree}'`, `git status --porcelain=v1 --untracked-files=all`, `git worktree list`, `git remote -v`, `git branch -avv`, `git ls-remote --heads origin` and exact `git rev-parse <ref>:<path>` probes. Require one worktree, one `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo.git`, matching handoff and Owner-installed ACTIVE, and every SHA/tree/blob listed in §2. The only initial dirty path is the Owner-installed ACTIVE replacement. If the technical branch has drifted from `8e1d5645…`, or the retained scratch has changed unexpectedly, STOP and report. Do not reset, stash, clean or discard unknown data.

**4B — Main issuance.** On `main` stage and commit exactly `.documents/WIP/workorder-active.md` unchanged, message `Issue SOLO-WO-077 R01: exact native-readback CI edge pin`. Push normally to `origin/main` and re-read the exact SHA/tree/parent/ACTIVE blob and one-path ledger through GitHub. Require successful original `rust-verify` jobs on **both** `ubuntu-24.04` and `windows-2025` at this issuance SHA, with 33 configured steps and no failure. `main` contains no WO-076 native code and needs no workflow change.

**4C — Bring ACTIVE to the preserved proof branch, without merging.** Check the checkout is clean. Switch **within the same checkout** to existing `solo-wo-076-r01` at the exact preserved SHA/tree; confirm tracking `origin/solo-wo-076-r01` and no drift. Apply **only** `git cherry-pick -x <exact WO-077 main issuance SHA>`. Verify its parent is the original branch tip; the only changed path is ACTIVE; the resulting blob exactly equals the new main ACTIVE; all seven WO-076 code/graph/guard blobs and workflow blob are unchanged. Push the advanced branch; read back the exact remote commit/tree/path. Do not require a green CI result at this intermediate cherry-pick: its workflow still contains the known stale `40` pin. This exception cannot be used to claim implementation acceptance.

**4D — Workflow-only correction.** On the clean branch edit only `.github/workflows/rust-verify.yml`, replacing exactly the two matched literals in §3. Use `git diff --check` and an exact byte/diff check that replacing both `41` occurrences back to `40` reconstructs the original workflow blob `eb78dad240bb7adb3e0ef9535870897176c99843` — without disturbing any other byte. Verify the real-tree check still runs its original command and enforces `architecture=legal packages=4`. Stage/commit **only** that YAML file, message `ci: pin approved native readback graph at 41 edges (SOLO-WO-077 R01)`. Push the branch (normal fast-forward), independently verify its exact remote SHA/tree/parent and one-path diff. Verify every WO-076 code/graph/guard blob from §2 is unchanged at the new branch head.

**4E — Exact branch CI, no shortcuts.** Use the existing authenticated GitHub Actions `rust-verify` **manual dispatch** on `--ref solo-wo-076-r01`, since `workflow_dispatch:` already exists; dispatch the completed branch HEAD and verify the run's **head SHA exactly equals the latest workflow-correction commit**, event is `workflow_dispatch`, and the workflow was read from that branch. Do not treat the issuance/main CI or a stale branch run as proof. Require the normal full workflow on **both hosts**, with all applicable mandatory steps successful: locked graph/package set, `cargo fmt --all -- --check`, strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-targets --locked`, pinned `cargo-deny 0.20.2` security gate, actual real-tree architecture exit 0 and **exact** `architecture=legal packages=4 targets=32 files=58 edges=41` output, original refusal self-tests, negative failure tripwire, release build, release CLI smoke, forbidden-import checks and Ubuntu byte-reproducibility checks. Normal OS-inapplicable matrix steps may be skipped only as they were before; an applicable step skipped because of a failure is a failed gate. Recheck actual `N076-01`, `N076-02`, `N076-03` test names and `ok` results on Ubuntu; Windows must compile/test the unchanged normal product and never report a Linux-native readback there. Inspect job logs and artifact evidence, not only green status.

If anything else fails, **STOP** with a factual `BLOCKED`/`UNVERIFIED` report — do not edit any source or third workflow location, suppress errors, weaken tests, add dependencies, change counts again or try another shortcut. A repeat manual dispatch of the **same** exact branch head is allowed only to investigate a clearly transient runner failure; disclose all runs and do not select a convenient green run while hiding red evidence.

**4F — Main canonical report.** With the technical branch preserved and independently verified, return through the single worktree to clean `main`; require that the new main issuance SHA is still its HEAD and that the WIP ACTIVE is the new WO-077 order. Overwrite only `.documents/Reports/execution-report.md` with the exact factual result and reference the technical branch commits, both-host branch CI and the two-byte-span workflow correction. Commit/push **only** that report file to `origin/main`; independently verify SHA/tree/parent/report blob and that branch tip remains unchanged. Require successful original two-host `rust-verify` at the **final main report HEAD**. Never merge the proof branch into main under this order. Branch and main can have different workflow pin values until a later, separately approved integration order unifies the code and its matching CI count.

## 5. Required proofs and acceptance gates

| Gate | Must be directly demonstrated |
| --- | --- |
| `S077-01` | Exact Owner handoff, single checkout/remote/worktree, clean initial main except installed ACTIVE; all main, branch, file, branch-history and CI anchors verified without inference. |
| `S077-02` | Issuance-only main commit, one path, remote readback, both original CI hosts successful. |
| `S077-03` | One expressly authorized issuance-only cherry-pick onto existing technical branch; parent exact; ACTIVE blob equals main issuance; no other path changed; no new branch or merge. |
| `S077-04` | Workflow-only branch commit with exactly **two** `edges=40` → `edges=41` replacements; inverse operation reconstructs original workflow blob; all error checks/commands untouched. |
| `S077-05` | All seven WO-076 code/graph/guard files retain exact prior blob identities, exactly one approved direct Linux `libc` root, original native source still only two justified gated `getrlimit` unsafe blocks, no child launch or new source. |
| `S077-06` | Corrected branch `rust-verify` runs at the exact **new** branch HEAD, both hosts all applicable gates green, and the checked real-tree output is truly `... edges=41`; no skipped applicable gate or substituted checker. |
| `S077-07` | Ubuntu executes all three real-kernel `N076` tests successfully; Windows remains free of the Linux-only module and passes existing suites; no claim of real containment or Windows Job Objects. |
| `S077-08` | One report-only main commit with exact GitHub readback, main original two-host CI green, clean one-worktree state; historical branches and technical proof branch intact, four WIP files, safe scratch cleanup. |
| `S077-09` | Correct scoped outcome, all red runs and corrections disclosed, explicit remaining limitations and R03.C3/R03.C/Stage 03 OPEN, no self-ACCEPT/merge/release. |

Report exact starting, issuance, branch cherry-pick, branch workflow commit, report-parent/final report commits, trees, parent chains, new and unchanged blobs, per-commit/branch changed paths, every run/job ID and relevant successful/failed/skipped step, actual log lines for the kernel tests and architecture count, full locked version/security/unsafe/allowance inventory, local toolchain limitation (no MSVC linker), preserved branch inventory, `.temp/` and Cline-temp cleanup, and local versus independently obtainable remote facts. Do **not** put the report commit's future SHA inside the report body; return it in the agent's chat report after remote readback.

## 6. Immediate STOP conditions and non-goals

STOP on mismatch in authority, HEAD/tree/blobs/branch/remote, extra worktree, unsafe/unknown dirty state, conflict during the sole approved cherry-pick, any edit outside §3, additional YAML change, removed/enfeebled checker or security step, changed dependency or unsafe source, CI head SHA mismatch, inability to run both hosts, missing native tests, unexplained failing downstream gates, unauthorized process/stress/media activity, untrusted executable/network/provider operation, secret exposure, non-fast-forward push, unsafe scratch cleanup, or any attempt to merge proof code to main. Publish an accurate report on main if safe and allowed; otherwise stop without destructive cleanup or false success. Existing WO-073 scratch remains unverified reference only and must not be copied or deleted in this order.

This is a correction-and-proof order, **not** authorization for a real child supervisor. Future `setrlimit`, `pre_exec`, `setsid`, signal/group operations, Windows suspended Job Object launch, AVIF decoding, media composition and publication require **separate Owner-issued orders** and real host evidence. The Implementation Agent must preserve the branch and return its actual evidence for independent Chief Architect disposition: **ACCEPT / REVISE / STOP**.
