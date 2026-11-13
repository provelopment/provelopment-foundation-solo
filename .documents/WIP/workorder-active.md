# SOLO-WO-095 R01 — INTEGRATE ACCEPTED LINUX GROUP/DESCENDANT LIFECYCLE PROOF

**Authority:** `OWNER-ISSUED — SOLO-WO-095 R01`
**Matching Owner activation:** `SOLO-WO-095 R01`
**Required outcome:** `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-INTEGRATED`
**Repository:** `provelopment/provelopment-foundation-solo`
**Only approved local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`
**Target:** the existing `main` and the existing `origin` only.

## 1. Execution objective and immutable acceptance basis

Integrate the **exact three already verified technical Git blobs** from the independently accepted `SOLO-WO-094 R01` green branch `solo-wo-093-r01` at commit `0dc45185110e6136bd63dc7edf774bc52b753c9e` into `main`. Perform **zero new source authorship**. Preserve the branch's actual code and tests byte-for-byte at Git-object level. The required output is the accepted, bounded, fixed Linux group/descendant lifecycle proof on `main`, with fresh, successful `push`-triggered Ubuntu and Windows verification at the integration SHA and final report SHA.

The Chief Master Architect's independent `ACCEPT` for WO-094 is limited to `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-VERIFIED`: a fixed Linux harness with one group leader, one descendant, checked leader reap, one bounded group-directed signal after identity checks, observed descendant non-running state and Windows refusal. This acceptance **does not** mean general process-tree containment, descendant reaping, race-free PID/PGID identity, a hostile-worker sandbox, aggregate resource bounds, AVIF media correctness, publication or release.

Keep `SOLO-WO-093 R01` **STOPPED / NOT ACCEPTED**, `SOLO-WO-094 R01` **ACCEPTED only on its green technical branch**, and `solo-wo-094-negative-r01` **intentionally RED / evidence only**. Its one-guard mutation must never enter `main`. The historic `SOLO-WO-089 R01` also remains permanently STOPPED / NOT ACCEPTED; the `SOLO-WO-090 R01` corrective branch remains accepted and unchanged.

This is **integration-only**. Do not close `R03.C3`, `R03.C` or Stage 03, edit ROADMAP/NEXT/OUTLOOK, implement the next proof, choose a codec, create a product, merge a PR, release or deploy. Those obligations stay **OPEN**. Do not self-ACCEPT.

## 2. Frozen baseline and mandatory preflight

Before any write, staging, branch action or scratch creation, check every actual identity against this pinned state:

| Item | Required exact identity |
| --- | --- |
| `HEAD`, checked-out `main`, local `main`, `origin/main`, `git ls-remote origin main` | `2ef0f26bd85259a2a0e97a8a5a55125a28b5ad12` |
| Main tree | `6e8e3e82a34625e62f090be2fe88802abb625c07` |
| Main sole parent | `8b643f7f9506f9602714fc62bd9f08d539851d15` |
| Existing tracked ACTIVE blob (WO-094) | `178818bd779cb2299afb4ab52d6220a2268882f3` |
| Existing report blob | `a437b5832199fec480a5615ef04ff1aec3ef6758` |
| Neutral NEXT | `92fa562b1af0c54391851aa5c79deb1631127069` |
| ROADMAP | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| OUTLOOK | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| Architecture checker entry | `0bad0e027f41b6cf6029df38174fc15531c187da` |
| Baseline main façade | `a1f2bdb9960a4befc44efab746640e4d19bb624b` |
| Baseline main native seam | `1e698245493f6d10ebb9636282698a797a242865` |
| Baseline main host smoke | `5444013571a71efde758422fdf9d8fada79ef53a` |
| Green source branch `solo-wo-093-r01` | `0dc45185110e6136bd63dc7edf774bc52b753c9e` |
| Green source tree | `60f986ddddd2f6b6bd6ab201a6e336aee8c501c7` |
| Green source sole parent | `64054a19df44ea728be2db2c791117a5db4a117c` |
| Negative evidence branch `solo-wo-094-negative-r01` | `0e9b3677cf774b9e31ac8fc52605e88112522be6` |
| Negative evidence tree | `84c8d55a38767a287f6203faf563b7e116ec4977` |

**Three accepted source Git blobs, exact and immutable:**

| Path | Required source Git blob | Expected integration numstat against baseline `main` |
| --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `7eed6e158c9374a6c2055ddd98dd3287357b814a` | `1579 129` |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `eacf51c5353ab3278969961aa136a0951cb7ad67` | `229 7` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `22ee67e94cc96a70e8e3a47db9e575464b53cd07` | `262 0` |

Confirm **one** verified Solo checkout/worktree, exactly **one** remote `origin` with both fetch and push URL `https://github.com/provelopment/provelopment-foundation-solo.git`, no stash, no additional clone, no unexplained untracked/staged/dirty path, and exactly **four** files in `.documents/WIP/`: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md` (no subdirectory or fifth WIP file). The **only allowed initial dirty path** is the complete Owner-installed, unstaged ` M .documents/WIP/workorder-active.md` containing **this exact order**, with matching Owner activation. Do not edit, trim, rephrase, normalize or reformat Owner-installed ACTIVE bytes.

Verify all **14 existing remote branch refs** and their starting SHAs, including `main`, `solo-wo-093-r01`, `solo-wo-094-negative-r01`, stopped `solo-wo-089-r01` at `1236915cab6e87b17ce84813430897b273fc3230` and accepted `solo-wo-090-r01` at `0eb3fcf645851ae752bd59e12fe32b69604197d2`. Every non-main ref must remain unchanged. Read the original source, corrected final branch and negative-branch diffs; verify the three accepted blobs are **from the green branch**, never the mutation branch.

Independently verify the prior accepted evidence: green branch `workflow_dispatch` run `43419376192` at exact SHA `0dc45185…`, Ubuntu job `131865233532` and Windows job `131865233959` both success, including the real Linux `n093_group=terminated` and `n093_refusal=` observations; intentionally red mutation run `43419899358` at `0e9b3677…`, both hosts failing specifically at `n093_05_signal_reap_observation_and_deadline_facts_stay_distinct`. The old uncompiled and failed correction commits remain intact. A green run for a different SHA is not accepted provenance.

Inspect and preserve all ignored Owner material, including `.temp/solo-wo-073-wip/` (last agent-observed **4 files / 46,062 bytes**), `architecture-fixtures/`, `toolchain-installer/`, `target/`, Cline logs and any unrecognized files. Local/ignored inventory is **agent-observed**, not independently GitHub-provable. Do not run `git clean`, destructive reset, aggressive checkout, history rewrite or force-push to make a gate pass.

**STOP before staging** on wrong root, authority, WIP count, revision/blob/ref, branch provenance, dirty path, missing accepted CI evidence, unsafe filesystem state or ambiguous approval. Do not silently repair a baseline.

## 3. Exact permissions, file ledger and prohibitions

Only these **five** permanent tracked paths may change during this order:

1. `.documents/WIP/workorder-active.md` — the complete Owner-installed WO-095 text in an issuance-only commit; **retain this exact issued order** through handoff.
2. `crates/solo-adapters/src/media_worker_host.rs` — promote only the accepted source Git blob shown in §2.
3. `crates/solo-adapters/src/media_worker_host_native.rs` — promote only the accepted native Git blob shown in §2.
4. `crates/solo-cli/tests/media_worker_host_smoke.rs` — promote only the accepted test Git blob shown in §2.
5. `.documents/Reports/execution-report.md` — overwrite once with the canonical execution report and publish in a report-only commit.

Every other permanent path is **read-only and blob-identical**: `plan-next.md`, ROADMAP, OUTLOOK, all About/Architecture/SOP documents, `README.md`, `AGENTS.md`, all Cargo manifests, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, workflows, architecture guard and fixtures, unrelated source/tests, root policy/licence and release/security files. Do not create or delete any permanent file. Preserve the source branch and negative branch without new commits, resets or movement. No fourth integration source path.

**Authorized Implementation Agent Git operations:** stage the exact files below, make **three ordinary linear commits** on existing `main` with prescribed sole-parent order, push those commits fast-forward to existing `origin/main`, and read GitHub commit/Actions data. GitHub Actions execution under the repository's existing automatic `push` trigger is authorized. Small task-owned disposable files may be created only in Solo `.temp/` or the active Cline-provided temporary workspace and must be removed by known exact paths after use. The Chief Master Architect has **read-only** GitHub authority and performs **none** of these writes.

**Not authorized:** any source authoring, code regeneration, `cargo fmt` rewrite, source normalization, dependency/toolchain installation or upgrade, workflow/guard change, new branch, PR, merge, cherry-pick, rebase, stash, amend, force-push, reset, blanket clean, tags, release, deployment, hosting/provider/DNS action, credentials access, separate repo/worktree, deletion of unknown material, or ANY write outside the five listed permanent paths. The local MSVC linker is absent; do not install one. Mandatory compilation/test evidence comes from GitHub's existing Ubuntu and Windows CI, not from a claimed local build.

## 4. Commit A — issued-order record and exact-head CI

On the verified starting `main`, stage **only** the Owner-installed ACTIVE file. Preserve its Git-filter-aware blob exactly. These commands are operational examples; comply with the canonical Version Control Procedure in `sop-architect.md` §6B:

```powershell
git add -- .documents/WIP/workorder-active.md
git diff --cached --name-status
git diff --cached --check
git commit -m "Issue SOLO-WO-095 R01: integrate accepted Linux group lifecycle proof"
git push origin main
```

The staged/committed change set must be **exactly one modified path**, ACTIVE. The sole parent must be `2ef0f26bd85259a2a0e97a8a5a55125a28b5ad12`. Any `git diff --cached --check` complaint about deliberate Owner-authored Markdown hard line breaks is recorded accurately rather than modifying the Owner-issued bytes; any unrelated whitespace error is a STOP. Verify actual Commit A SHA/tree/sole parent/staged blob/readback through GitHub.

Require a **new** `push`-triggered `.github/workflows/rust-verify.yml` run at **exact Commit A SHA**, with completed successful Ubuntu `ubuntu-24.04` and Windows `windows-2025` jobs, 33 configured steps per job, all host-specific skips explained, and no failed mandatory gate. Read the jobs/logs, verify `head_sha`, event, branch, attempt, conclusions, security advisories and material warnings. **No integration until Commit A is green.**

## 5. Commit B — exact-blob promotion only

Remain on the existing `main` at Commit A with no other changes. Promote **only** the three accepted blobs from the immovable green source commit. Use the already-present Git objects in the single verified checkout:

```powershell
git restore --source=0dc45185110e6136bd63dc7edf774bc52b753c9e --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-adapters/src/media_worker_host_native.rs crates/solo-cli/tests/media_worker_host_smoke.rs
git diff --cached --name-status
git diff --cached --numstat
git diff --cached --check
```

Check that the staged ledger contains **exactly three `M` paths** and the numstat matches §2, with **no extra path**. The exact index Git blob for every staged file must equal the accepted source commit blob. Verify using Git object identity, not a raw Windows worktree SHA-256 susceptible to CRLF/LF mapping:

```powershell
$src = '0dc45185110e6136bd63dc7edf774bc52b753c9e'
$paths = @(
  'crates/solo-adapters/src/media_worker_host.rs',
  'crates/solo-adapters/src/media_worker_host_native.rs',
  'crates/solo-cli/tests/media_worker_host_smoke.rs'
)
foreach ($path in $paths) {
  $expected = git rev-parse "${src}:$path"
  $staged = git rev-parse ":$path"
  if ($LASTEXITCODE -ne 0 -or $expected -ne $staged) {
    throw "STOP: source/index Git blob mismatch at $path"
  }
  "blob_exact $path $staged"
}
```

Additionally check `git hash-object --path=<path> <path>` under the configured filters, and record the result. A different **physical** Windows `--no-filters` hash may reflect CRLF/LF mapping only; disclose it honestly without 'repairing' line endings. **Never** cherry-pick, merge, apply a patch, reformat, regenerate, retype, normalize or edit the three source files. Preserve the accepted `group_wait_signal_v1(child.wait())?` checked-reap rule, native Linux group signal boundary, `n093_07` negative helper test, `n093_05` refusal matrix and its all-host coverage, Windows `HOST_UNSUPPORTED` and the N090 regression at their **exact accepted bytes**.

If every index-object check and path ledger is exact, commit the integrated tree on `main`:

```powershell
git commit -m "Integrate accepted Linux group and descendant lifecycle proof"
git push origin main
```

Commit B must have **exactly Commit A as its sole parent** and change exactly the three technical paths. Read back Commit B SHA/tree/sole parent, its changed-path and numstat ledger, and each full Git blob from GitHub. Compare the new `main` three Git blobs directly with the pinned green source branch's three Git blobs. Confirm the negative branch's deliberate mutation is **absent**, no source authored at integration, and the original green, negative and historical branches all remain at their frozen refs.

Require fresh `push`-triggered `rust-verify` CI **at exactly Commit B**; both Ubuntu and Windows jobs must succeed. Inspect the *actual* logs and demonstrate:

- Linux fixed group evidence `n093_group=terminated` with positive distinct leader/descendant PIDs, `leader_pid=pgid=sid`, `signal=9`, `leader_reaped=1`, `descendant_nonrunning=1`, `0 < wall_ms < 15000`.
- Linux invalid readiness refusal `n093_refusal=invalid_readiness=HOST_OUTPUT_INVALID` with no group signal/evidence; direct mode refusal; successful `n093_01/02/03`, `n093_05`, `n093_07` and `n093_06` checks.
- Linux `n090_reap=reaped_before_join` and earlier N079/N081/N083/N085/N087/N089 regressions, with host-appropriate bounded facts.
- Windows `n093_04` unsupported before child execution; N093 pure tests and N090 regressions pass; **no** Linux group success line is emitted on Windows.
- Four-crate package graph, architecture checker (`packages=4 targets=33 files=60 edges=41`), refusal self-check (`cases=29`), format, strict Clippy, all-target locked tests, security gate, release CLI accepted/refused smoke `0/2`, test-injection tripwire and Linux reproducibility as the unchanged workflow requires.
- Current RustSec and component-security gate results, genuine skips, known `Unicode-3.0` `license-not-encountered` warning accurately identified as an unused-allowance configuration warning, **not** a vulnerability. Distinguish the intentional RustSec advisory-negative probe from a vulnerability on the accepted locked graph. Record exact tool versions, CI run/job IDs, fresh advisory database commit/time, and any new material finding.

Any failure is a **STOP**, not permission to edit the accepted blobs or change a workflow or a dependency. Do not attribute the earlier branch's green dispatch to this integrated `main` commit: only the new actual `push` CI at Commit B proves the integrated head.

## 6. Commit C — canonical report and final exact-head CI

After Commit B's CI is independently green, overwrite **only** `.documents/Reports/execution-report.md`. Do not create dated, task-specific, backup or parallel reports. Keep the report public-safe, factual and concise, with:

1. `status=COMPLETE`, `OWNER-ISSUED — SOLO-WO-095 R01` and **claimed outcome** `LINUX-FIXED-GROUP-DESCENDANT-LIFECYCLE-INTEGRATED`, **pending independent Chief Master Architect ACCEPT / REVISE / STOP** (never self-ACCEPT).
2. Starting SHA/tree/parent and all pinned critical blobs; sole checkout, one remote, four WIP files, only Owner-installed ACTIVE initially dirty and matching activation.
3. Complete A (issuance) and B (integration) SHA/tree/sole-parent and permanent-file/numstat/blob ledgers, with the prescribed report-only Commit C sequence named but **without inventing or embedding Commit C's future SHA or tree**; stage, filter and GitHub readback proof for all three source files, negative-mutation exclusion, and preservation of all branch refs except `main`. Supply the actual Commit C SHA/tree/parent exclusively in the final chat handoff.
4. Separate exact-head CI runs for Commit A and B with both job IDs, real Linux N093 positive/negative/N090 evidence, Windows refusal, warnings/skips and security/version evidence. The original accepted WO-094 `workflow_dispatch` run and intentionally red mutation are immutable provenance, not new integration tests.
5. No source authorship, no code/contract/roadmap/dependency/CI/architecture change beyond the three accepted blobs, no unrelated edit, merge, PR, release or deployment. Explicitly note no descendant reap, race-free PID/PGID control, hostile-worker sandbox, complete AVIF media correctness or production capability; `R03.C3`, `R03.C`, Stage 03 remain OPEN.
6. Honest local-only ignored-material preservation, task-owned scratch inventory/cleanup, one-worktree clean-tree check, remaining limitations, known `Unicode-3.0` warning and any exact-head deviation or failure.

Stage and commit only that report:

```powershell
git add -- .documents/Reports/execution-report.md
git diff --cached --name-status
git diff --cached --check
git commit -m "Report SOLO-WO-095 R01 integrated Linux group lifecycle evidence"
git push origin main
```

Commit C must have **Commit B as its sole parent** and **exactly one** report-file change. Never write the report's own final commit SHA into itself. Verify Commit C's SHA/tree/parent/diff/report Git blob on GitHub, then require a new successful `push`-triggered Ubuntu+Windows `rust-verify` at **the exact Commit C SHA**. Do not treat the earlier Commit B run as a report-head run. Record Commit C run/job IDs and final URL in the **chat handoff**, not inside its self-referential report.

## 7. Final acceptance ledger, cleanup, STOP and handoff

Verify **all** before claiming complete:

- Starting `main` is followed by **exactly three linear commits**: A modifies only ACTIVE; B modifies exactly the three accepted technical paths, with the three required **source-identical Git blobs**; C modifies only the canonical report. Exactly **five distinct permanent paths** across the complete run. No fourth technical or WIP edit.
- Fresh **three separate** `push`-triggered CI runs at A, B and C; each completes successfully on Ubuntu and Windows. Real Linux group proof and invalid-readiness refusal execute **after promotion** at Commit B, and the report head is independently green.
- `main`/`origin/main`/`HEAD`/remote readback agree on exact Commit C; `git status --porcelain` empty, no stash, one worktree, one `origin` with verified URLs and no unknown untracked material. All fourteen starting branch refs unchanged except `main`; accepted source branch, intentionally red mutation branch, stopped historical branches unchanged.
- All four original WIP files remain, with ROADMAP/OUTLOOK/NEXT **blob-identical** to baseline. ACTIVE remains the **issued WO-095** (no neutralization or self-closure); report is the only current canonical report and was published. Original architecture, dependencies, guards, workflow, toolchain and unrelated source/tests remain unchanged.
- `n093_group` and `n093_refusal` actually appear in integrated Linux logs; negative branch's mutation does **not** appear in integrated source; accepted bounded semantics and no-publication claims remain truthful.
- Only precisely task-owned disposable materials from Solo `.temp/` and actual Cline-provided task temp were removed by exact path. Pre-existing Owner-owned/ignored material, including `.temp/solo-wo-073-wip/` (historically 4 files / 46,062 bytes), remains intact; distinguish agent-observed local claims from GitHub-verifiable facts.

**STOP immediately** on wrong Owner handoff, ref/tree/blob/WIP identity, extra local change, altered accepted source/negative branch, any index/GitHub blob mismatch, any extra staged/integrated file, missing or red exact-head CI, failure to execute new N093 Linux tests, Windows success behavior rather than refusal, unsafe group-target ambiguity, a failed or missing reap/negative refusal, unexpected source change, new material security warning/advisory, toolchain/CI/dependency drift, unknown ignored-file deletion, remote conflict or lack of a safe fast-forward. Never waive, downgrade, amend or silently 'repair' a STOP. Preserve all evidence, and publish one truthful STOPPED canonical report only where repository safety and the narrow report permission allow it. Do not claim full success if a mandatory condition is unverified.

**Final handoff:** Return the exact final canonical report GitHub URL, all A/B/C full SHA/tree/sole-parent and source-blob identities, cumulative path and numstat ledgers, the **three CI run IDs plus six job IDs** with verified status and warning/skip inventory, Linux and Windows test evidence, prior green and red branch proof identity, branch-preservation ledger, scratch cleanup and explicit nonclaims. The Implementation Agent must stop for the Chief Master Architect's independent `ACCEPT / REVISE / STOP`. The Owner alone authorizes subsequent closure, reconciliation, another order, release or deployment.
