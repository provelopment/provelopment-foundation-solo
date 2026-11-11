# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-057 R01**  
**Title:** Administrative closure of independently accepted R03.A; select R03.B; inspect and safely clean the Solo `.temp/` workspace  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Owner handoff ID:** `SOLO-WO-057 R01`  
**Approved Scope:** Execute the exact accepted-closure reconciliation, bounded temporary-workspace cleanup, and verification below. This is an **Owner-issued operational order**. It is **not** authorization to implement R03.B, alter product code, upgrade components, release or deploy. The Chief Architect uses GitHub **read-only** and performs independent review of the published result.

## 1. Outcome and accepted authority

Close **only** implementation package **R03.A**, independently disposed **ACCEPT** by the Chief Architect against the corrected substantive commit `fa724977f4e7b76493c386da3501f92e1d2206e7` (tree `6aae1296d8bd9a03d66cb7e04feb1034940c6c58`) and the separately published canonical execution-report commit `e45324484e49cb77c406a6c18c7d86cdb7492ce9` (tree `bf95f37a...`; report blob `17e8518e861ba96d34068b46c15d38384304ae8a`). At issue, `origin/main` is exactly `e45324484e49cb77c406a6c18c7d86cdb7492ce9`; establish this as the real baseline before any write.

Perform all of the following together:

1. In `.documents/WIP/plan-roadmap.md`, remove the **accepted R03.A package bullet** and no other accepted or pending package. Retain **Stage 03 OPEN**, retain R03.B, retain R03.C and the Stage 03 exit gate, and retain all of Stages 04–13 unchanged. Make the narrowly specified R03.B bullet clarification in §3 to distinguish its **remaining** identified-record-extension work from the R03.A `add`-fact registry already accepted.
2. Populate `.documents/WIP/plan-next.md` with exactly one bounded next **candidate, R03.B**, using §3's specified content. NEXT is a planning candidate, **never** an authorization to begin its implementation.
3. Reset `.documents/WIP/workorder-active.md` to the single neutral heading specified in §3 **only at the final accepted-closure documentation transition**. Do not leave the completed work order, an acceptance diary, a `NONE` paragraph, or a status essay in ACTIVE; the issued order is retained in immutable Git history.
4. Inspect **`C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.temp`** comprehensively. Clean **only** contents positively established to be obsolete, disposable task-owned files/directories. Preserve still-used, unknown, Owner-owned or unrelated material; document each justified retention and its specific cleanup event. Quantify before/after occupied bytes and explain any residual large content. The outcome is a **maintained clean temp workspace**, not deletion of the `.temp` directory itself or a cosmetic clean Git status.
5. Independently verify that the previously completed implementation branch `solo-wo-056-r01` is fully merged into the current `main` history. Prune **only that completed branch** locally and remotely **if** the exact safety conditions in §5 are met. Otherwise preserve it and report the concrete reason; no other ref may be deleted.
6. Overwrite and publish **only** the canonical `.documents/Reports/execution-report.md` for this closure, verify all requested permanent edits, the cleanup, the required two-host unchanged CI at both the closure and report commits, and the final clean repository state.

**Explicit exclusions:** no R03.B source implementation or tests, no R03.C work, no contract or architecture rewrites, no unapproved rewrite of ROADMAP, no new WIP slots or report archives, no release/tag/deployment, no dependency changes, no other repository changes. The accepted R03.A implementations and tests remain byte-identical.

## 2. Mandatory source, identity and safety preflight

Read and follow `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` (especially §§1, 5A, 6.2G–6.2I), `.documents/SOP/sop-architect.md` (§§6, 6A, 6B, 7A, 7B, 8), `.documents/SOP/sop-implementation.md` (§§4–5C, 8–11), all four existing WIP files and the canonical report. Read the previous R03.A report **from the exact reviewed GitHub revision**, and read the Owner's matching activation handoff. The implementation agent shall not reinterpret the Architect's acceptance as permission for a further implementation stage.

At the one actual local root verify all of these before permanent writes or destructive cleanup:

- `git rev-parse --show-toplevel` identifies **only** the Sole local checkout stated above; exactly one Git worktree, exactly one `origin` set to the stated repository, no nested clone or shadow source checkout, and no unrelated Owner-owned dirty/untracked contents requiring Git operations.
- Installed Owner-issued `workorder-active.md` is **this exact `SOLO-WO-057 R01`**. Before the first commit, the **only** expected tracked worktree edit relative to starting `origin/main` is the Owner's replacement of `workorder-active.md`; ROADMAP, NEXT, OUTLOOK, report and source remain baseline-exact. Do not silently overwrite unexpected changes or use `git reset`, `git clean` or destructive checkout.
- Baseline `main` and `origin/main` both resolve to `e45324484e49cb77c406a6c18c7d86cdb7492ce9`. Original accepted ACTIVE blob is `413359d7837e064cfbd6566a7304376163994f8c`; NEXT is neutral blob `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP blob `4c7ef0211dffc5e29ffc64d475b8fc9d11af213e`; OUTLOOK blob `cb9c7a610cf3c537557e060ca9656e44a053e69d`; execution report blob `17e8518e861ba96d34068b46c15d38384304ae8a`. Verify the four-file WIP inventory and no extra WIP directory/file.
- Inspect Git ignore and tracking. `.temp/` must be ignored by `.gitignore`; `git ls-files -- .temp` and index/staged verification must establish no tracked or staged `.temp` content. An existing file inside `.temp` is **not** thereby assumed obsolete. Preserve any conflicts.

If the baseline has moved or the worktree contains unexpected Owner material, **STOP**; report state and avoid cleanup that could cause loss. Do not auto-rebase, force-reset or delete unknown work to match this order.

## 3. Exact permanent file-operation ledger and resulting content

Only these **four** permanent tracked paths may be changed; no other tracked or untracked permanent path is granted write permission:

| Allowed path | Exact operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | First publish this Owner-installed `SOLO-WO-057 R01` in an order-only authorization commit on `main`. At the **final closure transition**, replace its entire content with the single neutral title line shown below. |
| `.documents/WIP/plan-roadmap.md` | Remove precisely the accepted R03.A package bullet under Stage 03 and clarify precisely the R03.B package bullet as shown below. Preserve every other byte/line, including Stage 03 heading, prerequisite, R03.C bullet, exit gate and Stages 04–13. |
| `.documents/WIP/plan-next.md` | Replace the neutral title-only file with exactly the R03.B candidate text below. Do not treat it as active work or authorization. |
| `.documents/Reports/execution-report.md` | Overwrite once as the single canonical, public-safe factual **closure execution report**, in a report-only final commit. Do not create a parallel log or report file. |

**ROADMAP Stage 03:** delete this exact accepted bullet (and its terminating newline):

```markdown
- **R03.A:** implement five-level direct-ancestry property resolution: absent inherits, explicit narrower declaration overrides, explicit suppression remains suppressed; retain provenance and enforce private/public projection.
```

Replace the existing `R03.B` bullet **only** with:

```markdown
- **R03.B:** complete authored `location` behavior at all five EFS levels by implementing identified inherited-fact `extend` with direct-ancestor targeting, in-place field updates, exact field-level provenance, ordering, duplicate/conflict and suppressed-target refusals; demonstrate independently authored Australia → Indonesia → Bali/Jakarta association/premises facts and optional Page/representation variants, with no effects on Page identity, route, language, URL or visitor geolocation.
```

The remaining `R03.C` bullet, exit gate, stage status and later stages stay unchanged. This wording documents unfinished scope, not an additional architecture decision.

**NEXT:** replace the complete file with the following exact candidate, ending with one newline:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R03.B — Authored location extension and independent geography

**Objective:** Complete the accepted §6.2H `location` property behavior by implementing the presently recognized but unsupported `extend` operation over the R03.A five-level typed registry and direct-ancestry resolver. Preserve location as an authored property, never an owner tier, route selector, locale or geolocation inference.

**Prerequisites/readiness:** R03.A is independently ACCEPTED at substantive `fa724977f4e7b76493c386da3501f92e1d2206e7` and is being administratively closed. The accepted §6.2G/§6.2H contract, `EfsOwnershipGraph`, closed `LocationFields` types, full qualified owner identities, `LocationExtensionRequestV1`, whole-graph fail-closed resolver and public projection exist. The remaining work is the specialized §6.2H extension semantics and full geographic independence evidence.

**Proposed scope:** Resolve only an accessible direct-ancestor, currently inherited unsuppressed `(originOwner, factId)` target; apply `set` members in authored operation order to the existing fact without changing `factId`, `kind`, original owner/index, exposure or output position; update `fieldOrigins` for set fields only and preserve untouched origins. Refuse Entity-root extension, current-node/sibling/cross-Entity targets, missing/ambiguous/suppressed facts, duplicate same-node extensions, malformed or conflicting operations, or any attempt to extend a same-node newly added fact. Preserve `inherit`/`suppress`, effective 16-fact cardinality, origin provenance and public/private isolation. Demonstrate independent Australia → Indonesia → Bali/Jakarta fact cascades and optional Page/representation variations with no routing, locale, URL or visitor-location effects.

**Deliverables:** Pure domain implementation and tests, minimal application/adapter wiring through the existing parsed-manifest candidate, positive and negative typed/provenance/ordering tests on real manifest bytes, preserved R03.A/R01/R02 regressions, exact two-host CI and canonical execution evidence. The eventual R03.B work order must separately specify the authorized file ledger and operations; this candidate authorizes no code edits.

**Exit conditions:** A valid identified extension updates exactly one inherited fact in place with full correct `fieldOrigins` and `resolutionEvents`; all invalid targets, exposure changes, duplicate operations, suppression and sibling-isolation cases refuse deterministically; location never changes EFS identity, routes, base URL, language/locale or inferred user position; no private data leaks; existing tests and both designated CI hosts remain green. Stage 03 remains OPEN for R03.C.
```

**ACTIVE neutral final content:** replace the complete file with exactly:

```markdown
# Foundation Solo — Active Work Order
```

No disposition paragraph or completed-order stub is retained in ACTIVE. Do not write any `SOLO-WO-058` order or begin R03.B as part of this administrative closure. Leave `.documents/WIP/plan-outlook.md` byte-identical.

**Architecture/About parity:** The accepted canonical §§6.2G–6.2H already specify the completed R03.A contract and the still-unimplemented R03.B extension semantics. Do not modify those documents as a cosmetic closure action; if you discover an actual conflict requiring a contract write, **STOP** for a separate Owner decision rather than extending this ledger.

## 4. Required `.temp/` audit and safe targeted cleanup

**Exact authorized temporary root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.temp` (the existing folder beneath the verified Solo checkout, not OS `%TEMP%` and not Cline's unrelated temp root). This order **expressly authorizes selective destructive deletion of positively verified obsolete disposable entries under this exact folder**, and no other Owner content. The standing SOP permission for identifiable task-owned Cline temp capture and ordinary run scratch cleanup remains unchanged.

**4A. Inventory BEFORE deletion.** Verify the actual folder is under the established checkout using literal/canonical paths; check whether it exists, whether it is a real directory, and whether any child is a symlink, junction, Windows reparse point, mount, submodule, nested Git repository or link out of the allowed root. Never traverse a linked-out target. Enumerate all direct children and inspect recursively **as metadata only** where safe; group by task/work-order identity, apparent purpose, creator/producer evidence, file count, occupied byte total and last-write timestamps. Calculate exact total bytes and MiB/GiB, and the largest attributable categories. Inspect only metadata and minimal non-secret provenance when necessary; do not dump private file contents, credentials or log secrets into the public report or agent chat. If the path is not safely confined, STOP without deletions.

**4B. Establish obsolescence positively, item by item.** Classify every material file/subtree as:

- **DELETE — established obsolete task scratch:** a disposable test output, prior completed run's intermediate, download/cache copy, diagnostic/capture log, generated temporary artifact or similarly reproducible material with demonstrable Solo/Cline task provenance **and** finished purpose; verify it is not being used by an active process, tool, current agent run, ongoing test, needed recovery/rollback, pending investigation or Owner workflow. Use the task ID, metadata, naming, producing command/history and referencing scripts/configuration to substantiate this. **Age, size or a filename extension alone never proves ownership or obsolescence.**
- **KEEP — actively needed:** any current test fixture, live cache/work directory, necessary intermediate or artifact with a concrete continued purpose. Document relative location or safe category, byte count, intended consumer and **specific cleanup event** (for example, completion of a named active test or acceptance of an identified follow-up), not an indefinite "may be needed".
- **PRESERVE / REVIEW — uncertain:** unknown provenance, shared/Owner-owned content, material with no safely ascertainable purpose, potentially valuable output, suspected secret-bearing items, or any linked-out/reparse entry. Do not remove, move, rename, overwrite, quarantine in another directory, or silently treat this as garbage. Record a safe metadata-level description, size, reason and precise Owner decision or investigation needed; do not disclose private content publicly.

For large entries, prioritize evidence gathering to free the most verified obsolete space first. **Do not delete a directory unless every contained entry has been positively classified as DELETE**, including hidden files, and no link or active-use hazard exists. Delete individual known-disposable leaf files and then empty task-owned directories where feasible. Preserve the `.temp` root itself even when empty. Use literal bounded targets only (for example PowerShell `-LiteralPath` with an item checked immediately before removal); never `Remove-Item` with a broad wildcard against `.temp`, `rd /s /q` on the root, blind recursive purge, `git clean`, a whole-folder recycle, or a deletion that follows a junction/symlink. If a file changes after classification, pause and re-evaluate it. If access/lock/permissions prevent safe removal, preserve it and record an **unresolved cleanup failure**, not a false success. Do not alter OS/IDE settings or kill active processes to release files.

**4C. Verify AFTER deletion and keep the area clean.** Reinventory the folder with the same counting method; report exact before/after byte totals, reclaimed bytes and MiB/GiB, removed file/dir counts, largest remaining categories and their classification, and whether anything remains unverified. Confirm all required retained items still exist and the folder itself is intact. Inspect task-owned Cline capture/output for this run and clean obsolete material before reporting, without touching unknown unrelated Cline files. Confirm `.temp` is still ignored/untracked/unstaged. Report actual local observations, not invented figures; do not equate a clean Git status with a cleaned `.temp` workspace.

**Cleanup completion rule:** Every **positively identified obsolete** task-owned item accessible under this authority must be removed. Verified active items may remain with a named purpose/cleanup trigger. Unknown items may remain unmodified with an itemized safe review request; this protects Owner data and is **not** permission to claim the entire folder empty. Identified obsolete items that could not be removed remain a **cleanup failure** and require `STOPPED` or a specifically justified partial disposition in the canonical report, not a fabricated PASS.

## 5. Completed-branch safety check and bounded prune

Only the exact prior implementation branch `solo-wo-056-r01` is eligible. Verify current `origin/main` includes accepted `fa724977f4e7b76493c386da3501f92e1d2206e7`; fetch/check the exact branch head; prove its entire commit history is already reachable from main and that it holds **no unique commits**, no unresolved work, no active PR or reference consumer and is not checked out in the sole worktree. If safe, the Owner authorizes normal **non-force** local and remote deletion of precisely this branch (e.g. safe `git branch -d solo-wo-056-r01` followed by a normal remote branch delete). Verify the remote ref has vanished and that `main` remains unchanged. If any condition is uncertain or false, **retain the branch without destructive attempts** and record the exact reason and next review. No other branch/tag/ref deletion, force operation or Git history rewrite is permitted. Branch cleanup is not permission to delete files under `.git/` manually.

## 6. Sequence, CI and canonical closure report

1. **Baseline and issued-order commit.** Complete §2 preflight. On the existing `main`, commit **only** the Owner-installed `workorder-active.md` containing this full issued scope, push ordinarily to `origin/main`, record full SHA/tree and verify the exact remote file. This is an administrative authorization commit, not a product change. Verify the unchanged GitHub CI on the two designated hosts for this commit; if a mandatory gate fails, STOP. No new branch or checkout.
2. **Local cleanup and safe branch pruning.** Execute §4's full before/classify/delete/after protocol on the actual Solo `.temp/` and §5's exact old-branch check/prune while the issued order remains active in Git. Do not stage any temp file. Record evidence in memory/task notes only as permitted temporary material; clean those task notes when no longer needed. Do not commit a temp inventory, extra report or helper script to the repository.
3. **Closure WIP transition.** Modify only `plan-roadmap.md`, `plan-next.md` and `workorder-active.md` to **exactly** §3's results. Before staging, inspect `git diff --check`, full diff and changed-path ledger. Commit only these three WIP files on `main` and push normally; verify full SHA, tree and remote blobs. This is the Owner-authorized closure transition: after this commit, ACTIVE is neutral, R03.A is removed, and R03.B is a *candidate only*. Run the **unchanged** `rust-verify` workflow and verify successful `ubuntu-24.04` and `windows-2025` jobs at the identical closure SHA. No test, CI or code file is modified.
4. **Canonical report last.** Overwrite `.documents/Reports/execution-report.md` only; commit and push **just this one file** from clean `main`, with no work-order archive. Use ISO date and `SOLO-WO-057 R01`; `COMPLETE` only when every in-scope mandatory administrative and verified-obsolete-cleanup requirement has been fulfilled, otherwise factual `STOPPED`/`FAILED` with a precise blocker. The report must record: starting/report prior SHAs, issued-order SHA, closure SHA/tree, exact ROADMAP/NEXT/ACTIVE contents and changed-path ledger; the prior independent R03.A ACCEPT basis; unchanged Stage 03/R03.C and unimplemented R03.B; precise temp audit method, safe categorized before/after totals, delete counts, genuinely needed retention with cleanup triggers, uncertain content preserved and review requests, cleanup failures if any; prior branch pruning evidence or why retained; Git ignore/tracked/staged check; Cline scratch cleanup; CI run IDs, both-host job/step outcomes and warnings/skips, and no product-code/version changes. **Never embed the report commit's own SHA inside its body.** Report any unrun check as `NOT RUN` and any unverifiable claim as `UNVERIFIED`.
5. **Final published HEAD verification.** After report-only push verify a **fresh** unchanged two-host CI on the report HEAD and read back the report from GitHub. Verify `main == origin/main`, clean `git status --porcelain`, one root/worktree/remote, neutral ACTIVE, exactly one R03.B NEXT candidate, R03.A absent from ROADMAP, Stage 03 and R03.C retained, OUTLOOK unchanged, no product/lock/workflow changes, `.temp` intact, and branch result as reported. Give the Owner the full final report commit SHA, tree, CI run IDs and the temp freed/retained totals. The implementation agent **does not self-ACCEPT**; the Chief Architect independently reviews this closure report.

**CI requirements:** The unchanged existing `rust-verify` matrix is the mandatory remote evidence for administrative commits. Require successful native Ubuntu 24.04 and Windows 2025 jobs, with applicable format, strict lint, tests, architecture, security/advisory and release-build checks intact. Record genuine warnings and intended opposite-host skips. As this is a **documentation-only reconciliation** with no component additions or pin changes, a new component/version upgrade or full external A1 research campaign is **NOT APPLICABLE**; cite the accepted R03.A prior A1 assessment and verify no manifests/locks/pins changed. Never report a skipped applicable check as passed.

## 7. Authorization boundaries and STOP conditions

**Authorized permanent operations:** replace the Owner-installed ACTIVE, the specified ROADMAP bullet removal/clarification, the exact one-candidate NEXT content, neutralize ACTIVE, overwrite the one canonical report, and ordinary verified commits/pushes to existing `main`; prune **only** the fully merged `solo-wo-056-r01` local/remote branch when §5 passes. The exact `.temp/` **selective destructive cleanup** in §4 and SOP-governed task-owned Cline scratch cleanup are separately and expressly authorized. Standard read-only GitHub and local validation access, existing CI execution and ordinary Git remote verification are permitted. No new implementation branch, PR or worktree is authorized.

**Not authorized:** deleting or emptying the entire `.temp/` folder indiscriminately; deleting any unknown, live, Owner-owned, linked-out or unrelated data; removing other branches or refs; changing `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, `.github/`, Rust crates, `README.md`, `AGENTS.md`, About, Architecture, SOPs or any existing fixture; dependency installation/change, destructive history commands, force-push, merge commit, tag, release, build artifact publication, provider/account access, secret operations, deployment, DNS/TLS modification, purchases, another checkout, another repository write, any work on R03.B/R03.C. The Architect has **read-only GitHub access only**.

**STOP:** unexpected repository baseline or missing matching Owner handoff; unaccounted-for permanent dirty state; contradictory canonical contract requiring an unapproved change; any linked/reparse temp path leading outside the allowed root; inability to distinguish an intended deletion from potentially valuable Owner material; materially unsafe active-use/ownership determination; a known obsolete item that cannot safely be removed; WIP transition that changes unrelated stage content; mandatory CI failure; unsafe branch state; remote divergence; or any need to expand scope. **Do not erase uncertain material to force completion.** Publish a truthful `STOPPED` canonical report only when the repository/report-only safety preflight allows and otherwise preserve the state, record exact blockers and the smallest Owner decision needed. No self-amendment or parallel work order.

**End of OWNER-ISSUED SOLO-WO-057 R01.**
