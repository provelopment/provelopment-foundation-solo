# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-007`  
**Edition:** `R05`  
**Title:** R00.A acceptance evidence reconciliation — correct R04 item-by-item traceability  
**Approval marker:** `OWNER-ISSUED` **only upon exact Owner installation and direct matching handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `d2f064c75f00b7c00cc144f7f650238dde5b03db`  
**Matching Owner handoff:** `SOLO-WO-007 R05 — Implementation Agent Handoff`  
**Roadmap scope:** Stage 00, `R00.A` only — an evidence-precision continuation of the existing work-order ID, not a new package or a reopen of architecture.  
**Work class:** Documentation-evidence reconciliation only; no product, architecture, SOP, application, toolchain or test implementation.  
**Chief Architect disposition prompting continuation:** `REVISE` on R04 source commit `cb7c2d22de584f508443f3d91bb5b7992b5d4dee` and report commit `d2f064c75f00b7c00cc144f7f650238dde5b03db`.  
**Completion authority:** Independent Chief Architect `ACCEPT / REVISE / STOP`; separately Owner-authorized closure after acceptance.

> **Issue gate.** This is the one complete replacement ACTIVE work order for Owner review. It becomes executable **only** when the Owner installs these exact bytes at `.documents/WIP/workorder-active.md` and directly delivers the matching R05 handoff. The prior R02/R03/R04 orders and handoffs do not authorize R05. The Implementation Agent does not self-issue, amend, rebase or close this order.

**Frozen Approved Scope:** §§1–7. The agent may publish the Owner-installed R05 text **unchanged**; it may not modify its content, append evidence, clear ACTIVE or mark acceptance.

## 1. Objective, already accepted content and exact evidence defect

The complete `R00.A` identity/publication semantic contract was recorded under R02, and the duplicate version-control procedure was eliminated under R03. R04 re-verified both and truthfully preserved the historical R03 `STOP` for an unauthorized outside-Solo temporary-file write. These durable results must **not** be altered or reopened.

Independent R04 review found two **report-traceability** defects, even though the canonical documents contain the underlying contracts:

1. The R04 report's §4 table calls itself a check of all **13 numbered R02 §2 items**, but the row numbers do not correspond to the actual numbered requirements in frozen R02: original **item 1** is separation of identity from labels/addresses (not the grammar), and original **item 6** is the starter Home Page (not generic full-locale representation identity). The first six entries are misaligned. Do not conceal this as merely a different grouping; repair the direct one-to-one crosswalk.
2. The R04 report's §5 table labels **C2** as preserving Architect SOP §6B's single Git authority, although **C2** actually requires mandatory reading of the current execution report in Implementation SOP §2. The §6B authority belongs with **C1**. Correctly label and inspect each of C1–C6, including explicit C2 documentary evidence.

These are evidence-accuracy failures of **R04 acceptance gates 4, 5 and 8**; they do not constitute a new architectural decision or evidence of another R04 outside-root write. The R03 historical `STOP` remains a fact, not a defect to erase. R05's purpose is to publish **one accurate and independently checkable current execution report** and prove that this R05 run observes the strict one-root write boundary.

No earlier Git commit or report may be rewritten; the new canonical R05 report must explicitly acknowledge the R04 errors and preserve the R03 incident and all material limitations. Do not describe any documentation example as executable test proof.

## 2. Exact evidence to verify and record in the R05 report

### A. Original R02 §2 numbering — mandatory one-to-one trace

Read the immutable R02 order **at commit `99748354b3e62659a4760b55c27c77f0feafed47`**, path `.documents/WIP/workorder-active.md`, §2. Use the **actual original item numbers and titles** below, one row per item, and provide the specific *current* canonical Architecture/Product section(s) and brief substantiated conclusion. These labels are a verification index, **not replacement architecture policy**:

| Original item | Exact requirement topic from R02 §2 | Primary canonical evidence to inspect |
| --- | --- | --- |
| **1** | **Separate identity from labels and addresses** — explicit persisted technical IDs at four levels, no ID derived from labels, routes, domain, locale, location, filesystem or provider; representation identified inside Page by locale | Architecture §1.1; Product §3 |
| **2** | **Technical ID grammar** — canonical lowercase ASCII regex, maximum 63 characters, no silent repair | Architecture §1.1 |
| **3** | **Uniqueness scope** — Entity per Installation, Facet in Entity, Site in own Facet, Page in own Site, no invented aliases/global tenant registry | Architecture §1.1 |
| **4** | **Durable reference** — explicit parent ownership, stable identity across non-identity edits; migration required for parent moves, ID changes or deletion | Architecture §1.1 |
| **5** | **Initial establishment** — author supplies Entity ID, seed Facet `main`/`Main` and Site `main`/`Main Site` | Architecture §1.3; Product §3 |
| **6** | **Starter Page** — Page ID `home`/label `Home`, real representation in an explicitly chosen supported full locale; no inferred locale or automatic undeclared publication | Architecture §1.3; Product §3 |
| **7** | **Published view cardinality** — exactly one Entity, nonempty published descendants, one direct parent, unlisted representations included in published graph | Architecture introduction and §1.3; Product §3 |
| **8** | **Incomplete drafts outside published graph** — no implicit publication, routing or discovery; complete validation required for admission | Architecture §1.3; Product §3 |
| **9** | **One authoritative publication manifest** — single explicit versioned source of EFS published graph and source references, logical contract only | Architecture §1.3; Product §3 |
| **10** | **Source integrity** — explicit one-to-one authored source resolution, no fallback/sharing/unsafe/missing or ambiguous source acceptance | Architecture §1.3 |
| **11** | **Fail-closed validation contract** — enumerated invalid cases and actionable diagnostics, no implicit repair or disclosure of secrets | Architecture §1.3 |
| **12** | **Conceptual examples and explicit negative cases** — valid default/expanded tree and refusals, labeled non-executable | Architecture §1.4 |
| **13** | **Preserved non-decisions** — R00.B–G mechanism choices remain open, while prior settled Solo invariants are intact | Architecture §7; Product §8 |

The count is **13 numbered items = 11 technical decisions (items 1–11) + 2 directive/non-decision items (12–13)**. Keep the substantive requirements intact and verify their meaning from the *original* R02 text rather than relying solely on this table. The R04 report's incorrect numbering remains in historical Git history and must be acknowledged in the new report, not retrospectively edited.

### B. Original C1–C6 definitions — mandatory separately labeled verification

Read the immutable R02 order **§3 at the same commit**. The new current report must contain a **distinct C1–C6 table** that uses these actual original labels and verifies each with direct current file/section evidence:

| Correction | Correct R02 §3 scope | Evidence to inspect |
| --- | --- | --- |
| **C1** | **One Git/GitHub procedure** — Implementation SOP §5B binds to Architect SOP §6B without a competing procedure | Both SOPs, especially §5B/§6B; R03 exact replacement text |
| **C2** | **Required reading and current evidence** — Implementation SOP §2 explicitly requires reading `.documents/Reports/execution-report.md` | Implementation SOP **§2 item 6**; AGENTS report-reading rule |
| **C3** | **A1 proof applies to actual authorized scope** — relevant requirements evidenced, future scheduled features not falsely required or claimed | Architect SOP §§7, 7B; Implementation SOP §7 |
| **C4** | **Private vulnerability reporting status** — enabled intake distinguished from open triage, support, NOTICE and disclosure policy | Product §8, Architecture §7/§8, README; `.github/SECURITY.md` read-only |
| **C5** | **README structure and authorization description** — Independence and hygiene headings separated; accurate issued-order/handoff, accepted ID grammar and unimplemented schemas | README status, installation and independence/hygiene sections |
| **C6** | **No superseded NEXT or roadmap invention** — NEXT title-only; ROADMAP/OUTLOOK untouched; no other stage accepted | All four WIP files; exact baseline blobs |

Never relabel C2 as another confirmation of C1. Include both a file/section reference and a short observed result in every row. The purpose is honest direct evidence, not a declaration of success by table formatting.

### C. Historical safety and present-run reporting

- Retain the historical facts: R03 wrote a temporary file outside Solo and received Architect **`STOP`**; R03's gate-7 `PASS` and “No STOP condition occurred” were inaccurate; removal of the specific file remains **`UNVERIFIED`** independently. Do **not** infer either residual harm or retroactive compliance.
- Distinguish R04's *reported* compliant verification methods from what is independently visible on GitHub; do not claim the Architect has seen the agent's physical checkout or process writes.
- Record R05's actual safe helper/process side-effect checks, original dirty state, working root, branch, one worktree/remote, parent ignore guard, changed paths, scratch/cache locations, cleanup, and final clean status. **No OS `%TEMP%`, home-directory, parent/sibling or outside-Solo write is authorized.** Unknown side effects trigger STOP before running the tool.
- Differentiate `PASS`, `FAIL`, `UNVERIFIED` and reasoned `NOT APPLICABLE`; list unperformed code/validator/build/browser/CI/security/release/deployment checks as **NOT RUN**, never PASSED. The agent never self-accepts.
- Preserve and accurately cite original R02/R03/R04 commit history. The report may name its **preceding content commit**; never embed its own resulting report-commit SHA in the report.

## 3. Exact repository permission ledger

| Exact path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Owner installs** this exact R05 replacement; agent stages and publishes the **unchanged** Owner-installed bytes in Phase A only. No agent content edits, disposition or clearing. |
| `.documents/Reports/execution-report.md` | **Phase B only:** overwrite the sole existing canonical report with accurate R05 evidence, including §2.A/§2.B one-to-one tables and §2.C safety history. |
| `.git/**` | Minimal authorized Git metadata, exact-path commits and GitHub fast-forward publication/readback under the **single** procedure in Architect SOP §6B. |

**All other permanent paths are strictly read-only**: both SOPs, Architecture, Product, README, AGENTS, ROADMAP, NEXT, OUTLOOK, `.github/**`, LICENSE, application/tests/tooling and parent/sibling repositories. No new WIP file, second report, work-order archive, code, dependency, configuration or provider operation. No new branch, remote, worktree, clone, shadow checkout or external scratch. Only agent-owned temporary files inside the verified Solo `.temp/` are permitted if genuinely necessary, with prompt safe cleanup. No destructive reset/clean/checkout, rebase, amend, force-push, history rewrite, unknown-file deletion, PR/merge, release, tag or deployment. Only authorized Solo Git transport/readback is allowed; do not restate or replace Architect SOP §6B as a separate procedure.

## 4. Mandatory preflight and STOP conditions

1. Read `README.md`, `AGENTS.md`, the Product Definition, Architecture Charter, **both** SOPs, all four WIP files and the **current published execution report**. Verify ACTIVE contains the exact Owner-installed `SOLO-WO-007 R05` text with `OWNER-ISSUED` marker **and** the Owner separately delivered the matching `SOLO-WO-007 R05 — Implementation Agent Handoff`. Neither the file alone nor any older handoff authorizes execution.
2. Verify the physical Solo root equals `git rev-parse --show-toplevel` and is not a symlink/junction; branch `main`; exactly one worktree; exactly one remote `origin` addressing `provelopment/provelopment-foundation-solo`; local `HEAD` = `origin/main` = GitHub API HEAD = **`d2f064c75f00b7c00cc144f7f650238dde5b03db`** before editing. If changed, STOP; do not silently rebase.
3. Initial `git status --porcelain` must show **only** the Owner-installed unstaged modification to `.documents/WIP/workorder-active.md`; nothing else staged, modified, ignored/untracked as unexpected task material. Capture starting blobs: ROADMAP `84f428b8d409c1665fc49d03a0e0e310c818b328`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; prior R04 ACTIVE `e2fde260d3333d1bba9ecb9ff0b05e2132925206`; Architect SOP `5f8aa8e78c75bbea73bac2a88753e1679aa930a6`; Implementation SOP `39cd894c95ff67ac858fb89b559e435a3a9d7ad9`; Architecture `28c0629549259185a715d4291ea5d065cffa3ec8`; Product `3f4e557e7b5b67c544bf2ce8ce718e5aec706fe8`.
4. Compare the original R02 order's §2 and §3 with R04's published §4 and §5 evidence tables; confirm the two stated indexing/mapping errors are real, not inferred. Establish readback and authentication helper side effects before execution; avoid any method writing outside Solo. Confirm the parent ignore guard read-only and one report/four WIP files.
5. Any missing approval/handoff, wrong HEAD or blob, unexpected dirtiness, uncontained tool side effects, prohibited write, requirement needing an unlisted edit, discovered material contract contradiction, unsafe publication or another unverified STOP condition: **STOP immediately**. Preserve data and, when safe under the SOP's standing report-only permission, publish only a truthful `STOPPED`/`FAILED` report. Do not continue to manufacture a nominal `COMPLETE` run.

## 5. Ordered execution and publication

**Phase A — Owner-issued order publication only.** Complete preflight and read-only cross-checks. Stage **only** `.documents/WIP/workorder-active.md`, byte-identical to the Owner-installed R05 text; create **one ordinary content commit**, fast-forward push to the verified Solo `origin/main`, and read back GitHub commit, file and unchanged baseline blobs. Do not make any other content edit. If a canonical semantic defect is actually found, STOP for separate Owner decision rather than modifying its source.

**Phase B — One truthful R05 report.** Overwrite **only** `.documents/Reports/execution-report.md` with the exact original R02 1–13 item index and separately correct C1–C6 audit, supporting current file/section evidence, explicit acknowledgment of the R04 mapping errors, preserved R03 safety `STOP`, R05 preflight/verification and scope proof, local-vs-GitHub provenance and limitations, and unperformed software checks. Stage only the report, create **one separate ordinary report commit**, fast-forward push, verify current `origin/main` and published report, and provide the final report commit SHA **only in chat**. Leave ACTIVE `OWNER-ISSUED` and ROADMAP/NEXT/OUTLOOK unchanged; STOP for independent Architect review.

**Failure path:** Preserve all Owner material and the frozen ACTIVE text; publish truthful STOP evidence only when safe. No historical rewrite or false `PASS`, even if a prohibited action is followed by successful cleanup.

## 6. Scope and closure boundary

This order does not reapprove or change the accepted R00.A semantics, the R03 SOP correction, any existing Solo invariant, or the prior R03 `STOP` disposition. A clean R05 can support acceptance of the **complete R00.A documentary deliverable**, provided all applicable gates pass; it cannot make a failed historical run compliant. Application code, validator tests, website behavior, CI, security scans, release and deployment are not implemented by this order.

**Only after independent `ACCEPT`** may the Owner separately authorize the WIP closure of the single completed **R00.A** package; the Implementation Agent may not prune ROADMAP, clear ACTIVE or select NEXT in this order. **Stage 00 remains open** for R00.B–R00.G, and no additional roadmap work is authorized here.

## 7. Independent Chief Architect acceptance gates

Each applicable gate requires direct evidence and `PASS / FAIL / UNVERIFIED`; inapplicable general software standards require `NOT APPLICABLE` and a reason, never a fabricated PASS.

1. **Authority and baseline:** exact R05 installation and direct matching handoff, expected `d2f064c7…` starting HEAD, verified Solo identity, single worktree/remote, initial dirty state and matching recorded blobs.
2. **13-item trace precision:** report's 1–13 rows match the **actual original R02 §2 item numbers and topics**, each with current canonical evidence; 11 decisions and 2 directives are counted correctly; conceptual tests are not claimed as executed.
3. **C1–C6 mapping precision:** report uses **the actual original R02 §3 C1–C6 labels**, explicitly checks C2 mandatory current report reading in Implementation SOP §2, and locates Git authority under C1 only, with real current document evidence for all six corrections.
4. **Historical integrity:** report acknowledges both R04 table errors, preserves R03 `STOP` and unresolved independent cleanup fact, avoids history rewrite, and separates earlier run failure from current execution truth.
5. **Architectural preservation:** R00.A content and all other read-only Solo contracts/SOPs unchanged; no reopened design, new code, test tooling, runtime proof or invented implementation result.
6. **WIP and publication discipline:** exactly four WIP files and one current report; unchanged ROADMAP/NEXT/OUTLOOK blobs, exact Owner-installed R05 ACTIVE and no parallel work order/report; one permitted Phase A content commit and one Phase B report commit.
7. **Current execution safety:** no outside-Solo scratch, tool-cache or helper write, no uncontained side effects or unsafe deletion; local evidence attributed accurately; actual worktree/temp state and remote readback checked; STOP if any new violation occurs.
8. **Evidence integrity and acceptance separation:** accurate, inspectable 1–13 and C1–C6 crosswalks and report statements, checks and limitations, clear GitHub-vs-local provenance, reasoned NOT RUN/NOT APPLICABLE software gates, no falsely marked PASS, self-acceptance, ACTIVE clearance or early roadmap pruning.

**Completion boundary:** The Implementation Agent stops after the two authorized, verified publications and returns exact commit links and candid limitations. Only the independent Chief Architect may return `ACCEPT / REVISE / STOP`; only a later explicit Owner authorization may close `R00.A`. `R00.B–R00.G` and Stage 00 remain outstanding.
