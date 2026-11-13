# SOLO-WO-096 R01 — Close accepted WO-095 and select the next bounded R03.C image-inspection candidate

**Authority:** `OWNER-ISSUED — SOLO-WO-096 R01` (executable only after the Owner installs this exact file in the sole ACTIVE slot and gives the matching handoff).  
**Matching Owner activation:** `SOLO-WO-096 R01`.  
**Required outcome:** `WO-095-CLOSED-R03C-IMAGE-INSPECTION-NEXT-SELECTED`.  
**Repository:** `provelopment/provelopment-foundation-solo`; **only checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; **branch:** `main`.  
**Nature of work:** accepted-work closure and four-slot WIP reconciliation **only**. No code, asset, dependency, workflow, release, deployment or implementation of NEXT.

## 1. Binding decision and precise scope

The Chief Master Architect independently **ACCEPTED** `SOLO-WO-095 R01` at exact `main` revision `2883e57d302cc3ce7d01556ef6587596e15afb23`, tree `0ff781194c6fc428434418504ef3aff1d86205e4`. That order integrated, as exact accepted Git blobs, one **bounded Linux process-group/descendant lifecycle test harness**. The three accepted `main` source blobs and fresh, green two-host CI are verified, including the real Linux `n093_group=terminated` and refusal evidence; the intentionally red negative branch remains excluded from `main`.

**Closure decision:** clear the issued WO-095 from ACTIVE and select one actual next engineering candidate. Do **not** close `R03.C3`, `R03.C`, or Stage 03: the integrated lifecycle proof is a prerequisite, not complete AVIF correctness, decoder confinement, platform catalogue delivery, whole-candidate validation or publication. The existing ROADMAP already states only the still-unfinished composite `R03.C` package; no independently finished ROADMAP line can be deleted without losing outstanding requirements. Therefore **ROADMAP remains byte-identical** rather than being rewritten with completion commentary or prematurely truncated. OUTLOOK also remains byte-identical. The historical STOPPED `SOLO-WO-093 R01` and `SOLO-WO-089 R01` are never rehabilitated.

SOPs and contracts remain binding: `AGENTS.md`; `.documents/SOP/sop-architect.md` §§6, 6A, 6B, 7, 8; `.documents/SOP/sop-implementation.md` §§5, 5A, 5C, 8–11; `.documents/Architecture/ARCHITECTURE.md` §§6.2G, 6.2I, 6A.8; and the live ROADMAP. Nothing in this order changes them. Follow the single-authority Git/GitHub procedure of Architect SOP §6B; this order authorizes the limited file edits and their ordinary commit/push mechanics only.

## 2. Mandatory exact preflight — before any permanent write

In the one verified Solo checkout, with `main` checked out and remote `origin` fetch **and** push both exactly `https://github.com/provelopment/provelopment-foundation-solo.git`, verify:

| Tracked item | Required exact identity at starting `main` |
| --- | --- |
| `HEAD == main == origin/main == ls-remote origin main` | `2883e57d302cc3ce7d01556ef6587596e15afb23` |
| main tree; sole parent | `0ff781194c6fc428434418504ef3aff1d86205e4`; `eedffc76408db8ecbd59f4d72d6dc635e030e7e2` |
| `.documents/WIP/workorder-active.md` | `18826717b609209e0b1aefbd92239024fb858c40` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `9eebd358282ac8354ccea1143b56135b482566da` |
| `.documents/Architecture/ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` |
| `.github/workflows/rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| `Cargo.lock`; `rust-toolchain.toml` | `b1ee56da60f36536548586528db371b7cbe508e4`; `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| Integrated `media_worker_host.rs` | `7eed6e158c9374a6c2055ddd98dd3287357b814a` |
| Integrated `media_worker_host_native.rs` | `eacf51c5353ab3278969961aa136a0951cb7ad67` |
| Integrated `media_worker_host_smoke.rs` | `22ee67e94cc96a70e8e3a47db9e575464b53cd07` |

The **only permitted initial dirty path** is unstaged ` M .documents/WIP/workorder-active.md` holding the complete Owner-installed **WO-096 R01** supplied here, with its unique `OWNER-ISSUED` authority and a matching explicit Owner activation. Read it byte-for-byte and preserve it without normalizing, shortening, overwriting or substituting a different edition. The baseline ACTIVE Git blob above describes the *tracked* WO-095 at `HEAD`, not the dirty Owner-installed WO-096 content; do not compare the two as if they must be equal.

Require exactly **one worktree, one remote, empty stash, four files and no subdirectory in `.documents/WIP/`**, no unexplained tracked or untracked change, and no pending merge/rebase/cherry-pick. Require exactly **14 remote branches**: `main`, `solo-wo-061-r01`, `-069-r01`, `-070-r01`, `-076-r01`, `-079-r01`, `-081-r01`, `-083-r01`, `-085-r01`, `-087-r01`, `-089-r01`, `-090-r01`, `-093-r01`, `-094-negative-r01` (each shortened suffix expands to `solo-wo-`). Specifically verify `solo-wo-093-r01 = 0dc45185110e6136bd63dc7edf774bc52b753c9e`, `solo-wo-094-negative-r01 = 0e9b3677cf774b9e31ac8fc52605e88112522be6`, permanently stopped `solo-wo-089-r01 = 1236915cab6e87b17ce84813430897b273fc3230`, and accepted `solo-wo-090-r01 = 0eb3fcf645851ae752bd59e12fe32b69604197d2`; compare all remaining refs with their observed starting identities at the end. Do not create, delete, change or reset a branch other than moving `main` by the expressly authorized fast-forward commits below.

Preserve pre-existing ignored material, including `.temp/solo-wo-073-wip/` (agent-reported four files, 46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/` and unrelated prior Cline logs. Inspect their current ownership/state without deletion. Follow §5A disposal of only identifiable task-owned scratch in Solo `.temp/` or the actual Cline-provided task area; never use another checkout, worktree, shadow source tree, arbitrary temp area or destructive Git cleanup.

**STOP** if a preflight identity, authority, root, source blob, branch state or worktree condition fails. Do not adapt the order silently to a newer `main`, replace Owner bytes, run `git reset`, or force-push. Publish a truthful STOPPED canonical report where safe under the explicitly authorized report-only path.

## 3. Exact allowed permanent paths and target text

**Only three distinct tracked paths may change during this entire order:**

1. `.documents/WIP/workorder-active.md` — Commit A: record the exact Owner-installed WO-096; Commit B: replace it with precisely the following single heading and one final LF, with no other content:
   ```text
   # Foundation Solo — workorder-active (Active Work Order)
   ```
   The existing canonical neutral Git blob is `2ef2db63df17cdb00513f47beca1bc199d381e06`.
2. `.documents/WIP/plan-next.md` — Commit B only: replace the neutral current file with the **exact** NEXT candidate text in §4 below (UTF-8, LF endings, one final newline). No second candidate, generic policy or self-authorization.
3. `.documents/Reports/execution-report.md` — Commit C only: overwrite the one canonical report. No alternate report, archive, changelog or WIP file.

**Explicitly forbidden:** any change to ROADMAP/OUTLOOK; About, Architecture, SOPs, `AGENTS.md`, README or other documentation; code, fixtures, assets, licences, dependencies, Cargo files, security settings, toolchain, CI/workflow or baseline source; branches other than `main`; PRs, merges, deployments, releases, external provider actions, secrets or installed tooling. No source implementation or next-candidate code is authorized. The Chief Master Architect makes **no** GitHub write; all authorized Git mechanics belong solely to the activated Implementation Agent.

## 4. The one exact NEXT candidate to install

The complete content of `.documents/WIP/plan-next.md` after Commit B shall be exactly:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R03.C — Confined Entity image-byte verification (PNG/JPEG/WebP first)

**Objective:** Deliver the first real, nonpublishing image-integrity gate over authoritative, already-confined Entity graphic source bytes; prove what the source actually contains rather than trusting its extension or an unverified worker claim.

**Prerequisites and readiness:** Existing R03.C1 declaration inventory and R03.C2 `GraphicAssetSourceSetV1` confined acquisition; accepted WO-095 Linux group lifecycle integration on `main`; binding Architecture §§6.2I.3–6.2I.6 and 6A.8. The required sandboxed offline decoder, Windows resource controls, and any codec dependency are **not** assumed implemented or authorized merely by this candidate.

**Proposed bounded scope:** Validate PNG, JPEG and WebP declared Entity images from owned immutable source bytes, with ≤20 MiB source bytes, exact extension/observed MIME/magic/decoded-format agreement, complete bounded decode, dimensions 1–8192 per side and ≤32,000,000 pixels. Build one typed inward inspection contract and replaceable bounded offline adapter, with whole-declared-inventory refusal for any malformed, mismatched, oversized, truncated, bomb-like or unsupported source. Preserve independent EFS owner provenance, internal/suppressed/unlisted obligations, strict namespace boundaries, and no automatic platform/role fallback. Any new decoder/version or host-containment choice requires an exact Owner-issued code-bearing order and current support/security/licence assessment.

**Verifiable exit:** Real on-host PNG/JPEG/WebP positive and adversarial corrupted/oversize/dimension/extension/MIME/magic refusal tests; a failed declared image blocks the complete image candidate without partial publication; no untrusted input reaches an unbounded decoder; exact-head Linux/Windows security, architecture, strict lint, tests and reproducibility evidence, with independently reviewed refusal sensitivity. No asserted media success from header-only inspection or fake-port output.

**Still outstanding in R03.C:** AVIF worker decoding and complete Linux/Windows fail-closed containment/least-privilege proof; approved video/WebVTT media; lawful version-pinned `platform/assets/` catalogue and provenance; whole publishable-candidate validation, derived immutable outputs and privacy-safe public projection. No publication, browser, release, platform artwork, or Stage 03 closure is conferred by this candidate.
```

This candidate is strictly a **non-executable proposal**. It selects the earliest actionable remainder of the accepted `R03.C` media obligation, based on concrete pre-existing source-byte ownership and the newly integrated Linux harness. It does **not** mean Windows Job Objects, AVIF decoding, platform library, or candidate publication are ready; the next code-bearing order must own and prove any such implementation prerequisites. The ROADMAP's full `R03.C` and all later stages stay in place.

## 5. Mandatory three-commit linear execution

All commands are performed inside the one verified checkout by the activated Implementation Agent; use the verified single remote and no unapproved Git operations. `main` may advance by **exactly three linear, non-merge commits**, with a known sole parent at each step.

**Commit A — issuance only.** From pinned initial `main`, stage only the complete Owner-installed ACTIVE. Verify one staged `M` path, its content is the untouched Owner-issued bytes, its commit parent is exactly `2883e57d302cc3ce7d01556ef6587596e15afb23` and a whitespace check has no untriaged findings. If the Owner-approved Markdown uses intentional hard-line-break spaces, disclose any `git diff --cached --check` findings and retain those original bytes; never silently edit the order merely to make the checker green. Commit message: `Issue SOLO-WO-096 R01: accepted WO-095 closure`. Push as an ordinary fast-forward to `origin/main`. Confirm exact commit SHA, tree, sole parent and one-path GitHub diff. Verify the automatically triggered `rust-verify` run at this exact commit has successful Ubuntu and Windows jobs before moving to B; distinguish host-inapplicable skips and known warning notices.

**Commit B — closure plus one NEXT.** Still on clean `main` at Commit A, replace only ACTIVE with the exact neutral heading in §3 and NEXT with the complete literal §4 text. Ensure each file is UTF-8/LF with one terminal newline, no extra WIP file, no ROADMAP/OUTLOOK edits and no hidden test/tool side effects. Verify exactly **two** staged `M` paths, `git diff --cached --check` exit 0, ACTIVE Git blob `2ef2db63df17cdb00513f47beca1bc199d381e06` and NEXT has exactly one H2 candidate. Commit message: `Close accepted WO-095 and select bounded R03.C image verification NEXT`. Push fast-forward. Confirm the expected two-path diff and exact sole parent (Commit A) from GitHub; verify the automatically triggered two-host `rust-verify` run at this exact head succeeded.

**Commit C — canonical report only.** From clean `main` at Commit B, overwrite only `.documents/Reports/execution-report.md` with an accurate `status=COMPLETE` report for `SOLO-WO-096 R01`, required outcome `WO-095-CLOSED-R03C-IMAGE-INSPECTION-NEXT-SELECTED` **pending independent review**. Include the baseline and final exact identities, A/B commits and two-host CI links, single remaining NEXT, explanation of unchanged ROADMAP (the integrated harness proves no full `R03.C` subpackage), unchanged OUTLOOK, all preserved branches, no source/asset/toolchain/dependency/workflow changes, the initial and final Git blobs, limits, warning/skip classification, local-only cleanup and explicit nonclaims. State that WO-095 was previously independently ACCEPTED, but do **not** claim that this administrative closure independently completes `R03.C3` or Stage 03. Never place the report's own future commit SHA or run ID in the report text. Stage only the report, verify clean whitespace and exactly one `M` path. Commit message: `Report SOLO-WO-096 R01 WIP closure and NEXT selection`; push fast-forward; require a fresh successful **two-host** `rust-verify` run at exact Commit C. Read back its exact sole parent B, tree, report blob, one-file diff, event `push`, branch `main`, run attempt and successful jobs.

Do not invent a time estimate or infer successful CI from a prior revision. All three independent `push` runs execute the **unchanged** workflow, not a new code/test obligation. A failed mandatory CI job is investigated and reported as a failure; no CI edits, disables, retries intended to conceal a defect or altered requirements are authorized. Any STOP before C produces a truthful report only when safe, and never pretends closure was completed.

## 6. Final acceptance and STOP conditions

To report `COMPLETE`, verify `HEAD == main == origin/main == ls-remote origin main == Commit C`, empty tracked/untracked status, empty stash, one worktree, one `origin`, exactly four WIP files, exactly one NEXT H2, ACTIVE neutral with exact blob, ROADMAP and OUTLOOK byte-identical, and all three already accepted technical `main` blobs unchanged. The entire baseline-to-C cumulative changed-path set must be **exactly** ACTIVE, NEXT and canonical report, with no deletion or new tracked file. All 14 starting remote branch refs must still exist and have their exact starting SHA except `main`, which alone advances linearly. Retain the green accepted technical branch and the red negative-mutation branch untouched.

Observe no changes in product functionality, stage completion, media correctness, AVIF, Windows Job Object support, source catalogue, public projection, publication, HTTP/browser behavior, release or deployment; all these remain nonclaims. GitHub cannot prove Owner-local ignored-file counts, so distinguish local observations from independently verifiable remote evidence. Remove only clearly task-owned disposable files in the two permitted scratch areas; record limitations and unresolved cleanup faithfully.

**STOP and publish the blocker** if any authority or baseline pin differs, a branch or protected blob changes, an unauthorized permanent path is touched, WIP count differs, NEXT contains multiple candidates, source bytes differ, a CI gate is red/unverified, a remote identity cannot be established, a destructive Git operation would be required, a hidden tool installation/provider action is requested, a secret is exposed, or a task-owned unbounded artifact cannot be safely removed. Do not repair a mismatch outside the exact three-path ledger, self-accept, or execute NEXT as though this closure order grants implementation permission.

Handoff must include canonical report URL at Commit C, A/B/C SHAs/trees/parents/diffs, the three GitHub Actions run IDs and six job IDs, frozen source/branch evidence and explicit PASS/FAIL/UNVERIFIED/NOT APPLICABLE outcomes. End for independent Chief Master Architect `ACCEPT / REVISE / STOP`; only the Owner may install a subsequent implementation work order or authorize any merge, release or deployment.
