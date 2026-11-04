# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-005`  
**Edition:** `R01`  
**Title:** Adopt the four-file WIP execution lifecycle and codify permanent planning and A1 requirements  
**Approval marker:** `OWNER-ISSUED`  
**Issuance condition:** This work order is authorized **only when the Owner installs this exact complete text** at `.documents/WIP/workorder-active.md` in the verified Solo repository **and separately supplies the matching `SOLO-WO-005 R01` Implementation Agent handoff**. Neither the file outside ACTIVE nor the handoff without the installed file authorizes action. No changes to the text are required for the Owner's installation.  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required pre-execution `origin/main` HEAD:** `87b176a7a7cf3aa22e85d4709270e935721d0a6e`  
**Owner handoff identity:** `SOLO-WO-005 R01 — Implementation Agent Handoff`  
**Work class:** Documentation-only governance migration; no application implementation, dependency installation, product release or deployment.  
**Completion authority:** Independent Chief Architect disposition on published evidence; Owner controls acceptance-based closure and any subsequent order.

## 1. Exact objective and inputs

Perform **one** coherent documentation migration. Install the already-written, stage-only implementation roadmap as the fourth permanent WIP file; make ROADMAP → NEXT → ACTIVE the execution sequence; reserve OUTLOOK exclusively for optional ideas; move permanent planning, Foundation-reference and A1 quality rules into the existing canonical SOPs; and eliminate all contradictory current document references. The standing Solo product and architectural decisions must not change.

**Expected Owner-supplied local inputs** are **already on disk** in the verified Solo root. Their paths are repository-relative, and both are currently untracked:

| Source | Exact size | Required SHA-256 | Authorized use |
| --- | ---: | --- | --- |
| `.documents/WIP/plan-roadmap.md` | 17,982 bytes | `20378A4867E6E6816D276E6C9B8D9364230E30859C944FBD58B55F9C57FD4A2E` | Validate **in place** and add this exact, unmodified file to Git tracking. No recopying, editing, filtering or regeneration. |
| `.documents/SOP/planning-governance-amendments.md` | 27,684 bytes | `AD41144E182C65529AFF8DEEC158CA4B8E06A6A697C1ABBB64070968F4B7A9C8` | Read as the Owner-provided amendment instructions, not as a canonical SOP or authorized policy; apply relevant contents to the named existing documents. Never stage or publish this source file. Its sole permitted cleanup is specified in §4. |

Check the **actual disk file bytes**, regular-file type, sizes and hashes. No tool download or replacement source is allowed. If an expected source is missing or different, STOP. The approved roadmap's content takes precedence over any earlier draft phrasing that suggested editing it in the same migration.

## 2. Exclusive allowed permanent Git paths

These **eleven** repository paths are the complete allowed permanent change inventory (all changes on `main` only):

| Path | Authorized change |
| --- | --- |
| `README.md` | Reconcile WIP document links, roles and current status references from three slots to four; OUTLOOK is ideas, not delivery planning. |
| `AGENTS.md` | Update reading order, exact-four-file preflight, authority map, §2A handoff and §5 WIP invariant; retain the single issued-order gate, single current report and one-root restrictions. |
| `.documents/About/FOUNDATION-SOLO.md` | Fix only outdated three-file planning/governance descriptions and links. Preserve product intent and decisions. |
| `.documents/Architecture/ARCHITECTURE.md` | Fix only obsolete WIP references and, if necessary, consolidate enduring acceptance criteria within existing §6. Preserve all accepted architecture. |
| `.documents/SOP/sop-architect.md` | Make this the single durable owner of Foundation-reference discipline (§4), permanent A1 acceptance criteria (new §7B) and four-slot ROADMAP/NEXT/ACTIVE/OUTLOOK lifecycle (replace §8), with accurate cross-references elsewhere. Preserve §6B Git rules and §7A reporting. |
| `.documents/SOP/sop-implementation.md` | Replace §9 with the operational four-file rules; add cross-references to Architect SOP §4 and §7B; reconcile required reading, preflight, reporting, permissions and closure references. Do not create competing policies. |
| `.documents/WIP/plan-roadmap.md` | Add the verified existing stage-only bytes unchanged. |
| `.documents/WIP/plan-next.md` | Replace with **only** a single bounded next work definition corresponding to unfinished roadmap package `R00.A` (identity and publishing declarations): identifier, objective, prerequisites, deliverables and acceptance. No standing instructions, history or authorization discussion. |
| `.documents/WIP/plan-outlook.md` | Remove committed delivery stages and mandatory short/medium/long planning; retain only genuinely Owner-supported optional ideas, if any. Otherwise make it title-only. Do not invent ideas or move unfinished obligations here. |
| `.documents/WIP/workorder-active.md` | Publish this **exact Owner-installed** `SOLO-WO-005 R01` text unchanged in the content commit. **Do not clear, replace, annotate or alter it** before independent review. |
| `.documents/Reports/execution-report.md` | Overwrite the one current report with factual evidence of this run and publish separately as specified in §6. |

No other permanent path may be written, staged, deleted, renamed or committed. The WIP directory must end with **exactly four tracked files and no subdirectories**: `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`. The canonical report must remain **outside WIP**, and no additional SOP, log, archive, report, tracker or task file is permitted.

## 3. Required documentation changes

1. **Architect SOP §4 — reference-source restriction.** State that historical Foundation and other repositories are reference material only. Their code, dependencies, routing, forms, tests and defects cannot be copied into Solo by assumption. Any future reuse requires Owner-approved scope and evidence of contract fit, negative behavior, security, provenance, licensing/attribution and hidden dependencies. The implementation SOP refers to this rule rather than duplicating it.
2. **Architect SOP §7B — permanent A1 standard.** Codify evidence-based correctness, strict inward-only Ports-and-Adapters dependencies, meaningful import-boundary and port tests, validated and safe public content, positive and refusal tests, real browser/HTTP proof where applicable, accessibility/performance against Owner-approved targets, reproducible builds, safe repository operations, truthful reports, and independent `ACCEPT / REVISE / STOP`. Do not silently make an unaccepted WCAG level or numerical budget an Owner-approved mandate. Implementation SOP §7 must reference §7B without copying the entire charter.
3. **Architect SOP §8 — exclusive lifecycle and purity.** Define ROADMAP as unfinished committed stages only; NEXT as one bounded unissued next candidate; ACTIVE as the sole exact Owner-issued work order; OUTLOOK as optional, nonbinding ideas only. ROADMAP → NEXT → ACTIVE is the only progression. The Owner controls selection and issuance. The selected candidate is removed from NEXT when actually issued; ROADMAP retains the obligation until independently accepted and Owner-authorized closure. Failure/REVISE/STOP leaves the obligation in ROADMAP. After acceptance and separate Owner-authorized closure, remove accepted packages/stages, clear ACTIVE and select a new NEXT candidate. At full completion, retained ROADMAP/NEXT/ACTIVE each have zero entries; OUTLOOK may contain ideas. Historical outcomes remain in Git history, not WIP. No generic policy, explanatory commentary, completed-work diary or permanent quality standards belong in any of the three execution documents.
4. **Implementation SOP §9.** Operationalize four-slot inventory and purity, the exact handoff gate, no agent-initiated NEXT promotion or ACTIVE closure, authorized transitions only, single report, and safe STOP behavior. Keep the existing single-root, no additional worktree and independent-review boundaries intact.
5. **All affected current documents.** Apply the content and consistency requirements in the verified `planning-governance-amendments.md` sections A–G and I, insofar as they match this issued order. The amendment file's own draft/issuance language and its section H are **not** authority; this work order overrides them. Examine every current WIP reference, section reference, inventory count and report explanation in the ten governing/persistent paths. Resolve stale three-file counts, mandatory short/medium/long execution planning, competing policy ownership and broken links. Preserve the one canonical report and the standing report-only exception.
6. **Never revise settled technical decisions.** Preserve in meaning: exactly one Entity per Installation; EFS identity hierarchy; strict Hexagonal Architecture; full region-qualified locale tags; authored location/graphics five-level cascade and suppression; configurable HTTP(S) base origin plus optional path; public/unlisted route casing and collision refusal; `visibilityDefault` and localized `visibility`; safe JSON/Markdown publication; simple contact form and safe `mailto:` actions with advanced forms excluded; Apache-2.0 and independent release/deployment authority. If reconciliation would require changing any of these, STOP for Owner decision.

## 4. Local state, permitted cleanup and explicit exclusions

The following local preflight differences are **anticipated and permitted only under this exact issued order**: (a) Owner installation of this document as the sole tracked modification, and (b) the **two** hash-locked untracked inputs in §1. Before migration, `origin/main` tracks **exactly three** WIP files; the fourth exists locally untracked. This order expressly permits that transition after issuance and requires **exactly four** tracked WIP files in the accepted end state. No other initial modification, staging or untracked item is permitted.

After verifying the amendment source's **exact** original path, bytes and hash, applying all relevant amendments, and successfully pushing and reading back the **content commit**, the Owner explicitly authorizes deletion of **only** `.documents/SOP/planning-governance-amendments.md` so that it does not remain an untracked apparent SOP. Recheck the hash immediately before removal. Deletion is not authorized if the file has changed, is tracked, is a symlink, or ownership/safety is uncertain. Record evidence of removal in the current report. Do not remove the roadmap or any other Owner-owned file. **If content publication fails, preserve the amendment source.** A failure after verified content publication must be documented truthfully; never claim a clean tree without checking it.

Permitted Git operations: inspect, stage **only** the exact allowed paths, make **one ordinary content commit and one ordinary report commit**, and fast-forward push each to verified Solo `origin/main`, then read back the remote tree and files. The Owner's approval of this work order explicitly permits these two scoped commits and pushes. The ordinary Git procedure of Architect SOP §6B governs details. Do not perform merge, PR creation, force push, rebase, amend, destructive reset/clean, unsafe overwrite, tag, release, deployment, provider setup or secrets access.

No application code, tests, CI, tools, package manifests, dependencies, asset changes, new SOP or new branch are authorized. No additional Git worktree, clone, nested repository, shadow checkout or outside-Solo scratch space may be created. Any necessary temporary/cache files must remain inside the verified Solo root under `.temp/`, and only verified agent-created ephemeral files may be cleaned. The enclosing or other Provelopment repositories are read-only and must never be modified.

## 5. Mandatory preflight and STOP conditions

Before **any** documentation or other implementation write:

1. Read the exact current `README.md`, `AGENTS.md`, both SOPs, product definition, Architecture, all currently tracked WIP files, and the current published execution report. Confirm this **complete** `SOLO-WO-005 R01` is installed as `OWNER-ISSUED` in `.documents/WIP/workorder-active.md`, and the Owner supplied the **separate matching handoff** named above. If either condition fails, STOP; only the existing report-only exception remains.
2. Confirm the physical Solo root and Git top level coincide, not a symlink/junction. Verify `main`, local `HEAD`, `origin/main` and GitHub's remote `main` all equal `87b176a7a7cf3aa22e85d4709270e935721d0a6e` before the first content commit. Confirm exactly one worktree, exactly one correct `origin`, the parent ignore guard, no parent/sibling writes and GitHub authentication. A remote advancement or unverified mismatch is STOP, not permission to rebase or reset.
3. Verify exactly the expected changed/unstaged/unstaged-untracked inventory: Owner-installed ACTIVE plus the two §1 files, no other staged change or unknown file. Verify current committed WIP has three files before the authorized migration and the canonical current report exists outside WIP. Inspect the two source hashes and real-file types.
4. Confirm the work order's expected paths and the source amendment specifications are coherent with current governing rules. Identify all affected cross-references before editing. If any instruction requires a new Owner decision, an unlisted path, unsafe cleanup, an unverified source or source alteration, STOP and publish factual evidence under the report-only rule when safe.

STOP immediately on missing issuance/handoff, failed identity/branch/HEAD/worktree/remote/preflight, unexpected dirty or staged data, hash mismatch, unsafe file cleanup, conflicting Owner-approved architecture, a need for out-of-scope modification, inability to publish safely, or any attempt to conceal a failed check. Preserve Owner-owned material. Do not self-issue a replacement order or silently expand scope.

## 6. Ordered execution, verification and publication

**Phase A — governance/content commit.** Complete the ten permanent-document operations (everything in §2 except the report). Preserve the ACTIVE work-order bytes. Verify the complete edited diff against the allowed inventory; check structural WIP purity; verify the roadmap matches the §1 SHA-256; audit all current documents for obsolete three-file/OUTLOOK assertions and conflicting authority; validate Markdown links and referenced section anchors; verify no mixed instructions remain, no unapproved architecture changes occurred, and no other paths changed. Report each check honestly. Stage exactly the ten named content paths; create a single ordinary commit and push it safely to `origin/main`. Read back the GitHub commit/diff and four-file WIP inventory, the ACTIVE blob and the roadmap blob.

**Approved source cleanup.** After Phase A remote verification, recheck the §1 amendment source and remove only that exact untracked file as authorized in §4. Verify the final working-tree inventory; if cleanup cannot be performed safely, STOP and report the remaining source rather than deleting anything else.

**Phase B — report commit.** Overwrite **only** `.documents/Reports/execution-report.md` with the run's factual evidence: original HEAD; exact Owner-issued ID and handoff; source hashes; authorized before/after inventory; source cleanup outcome; links and reference audits; frozen ACTIVE preservation; content commit; exact changed files; tests/checks PASS/FAIL/UNVERIFIED; no-runtime-test limitation; Git/worktree/remote/parent/temp evidence; remote readback; and the independent-review next action. Stage this one report only, commit and fast-forward push it to `origin/main`. Verify exact remote `main` and report content. Give the final report-commit SHA in chat, **not** inside the report text. Leave ACTIVE issued, and start no new work order.

**Failure/STOP:** No premature roadmap reduction, NEXT execution or ACTIVE neutralization. If safe, overwrite/publish only the factual current report using the standing report-only exception; otherwise report the precise blocker without claiming publication or cleanup. Do not delete unverified Owner material.

## 7. Required acceptance evidence (each `PASS`, `FAIL` or `UNVERIFIED`)

1. Installed exact `OWNER-ISSUED` `SOLO-WO-005 R01` and separate Owner handoff verified **before** source changes; expected initial `main`/remote SHA and approved dirty-tree exception recorded.
2. Original local roadmap and amendment source each matched the specified path, byte length, SHA-256 and regular-file type; roadmap content committed unchanged; any amendment-source deletion matched §4 exactly.
3. Exactly four permanent tracked WIP files and one report outside WIP; no duplicate logs/archives/SOP files or published amendment source.
4. ROADMAP contains only still-pending Stage 00–13 and associated packages/gates, and the exact source bytes. NEXT names only `R00.A` with bounded deliverables and acceptance. ACTIVE retains exact frozen issued scope. OUTLOOK contains only supported optional ideas or title only.
5. Architect SOP owns permanent reference-only, A1 and four-slot lifecycle requirements; Implementation SOP operationalizes them without competing authority; all references and numbering are correct.
6. All existing active docs consistently express four-file WIP, ROADMAP → NEXT → ACTIVE, speculative OUTLOOK, the sole issued-order-plus-handoff execution gate, one overwritten report, independent acceptance and Owner-authorized closure. No stale three-file counts or mandatory delivery-horizon assertions remain.
7. Product/Architecture's accepted one-Entity/EFS, locales, route/visibility/case, cascades/graphics, strict Hexagonal Architecture, simple contact/exclusion and licensing rules remain semantically unchanged.
8. Exact patch, all changed files, Markdown links/anchors, source hashes, single-root/worktree/remote and parent isolation are verified; no unknown modification/deletion, unsafe Git mechanics or unintended code/tool/provider work.
9. Content commit and report commit are ordinary, linear and pushed to the verified Solo `origin/main`; GitHub's exact-head tree and current report were read back; local working tree is clean or a real limitation is prominently reported.
10. The report truthfully distinguishes documentation verification from any **unperformed** build, CI, unit, browser, release or deployment tests; no absent test is claimed green.

## 8. Review and completion condition

The Implementation Agent delivers only the published Git revision(s), the canonical execution report, and a factual account of the checks and any deviations. The Agent must not independently accept the work, reduce ROADMAP, promote another NEXT item, clear ACTIVE or infer release/deployment permission.

The Chief Architect independently reviews the exact GitHub revision(s) and source-level evidence, deciding `ACCEPT`, `REVISE` or `STOP`. This order **does not** authorize post-review closure writes. After independent `ACCEPT`, the Owner may separately authorize the exact WIP closure/reconciliation operation; until then the issued order remains in ACTIVE. The next execution task must have its own exact Owner-issued order and matching handoff.
