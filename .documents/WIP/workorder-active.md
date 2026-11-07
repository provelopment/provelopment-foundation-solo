# Foundation Solo — Active Work Order

## Approved Scope — SOLO-WO-022 / R01

**Issuance:** This is the Chief Architect's **single proposed administrative closure order** following independent `ACCEPT` of `SOLO-WO-021 R06` at published `main` revision `7863342d6b60b5af1b28bd8b41e29fe9efbe6765`. It becomes **`OWNER-ISSUED` only when the Owner deliberately installs this entire exact file as `.documents/WIP/workorder-active.md` in the one existing Foundation Solo checkout and separately directs the Implementation Agent to execute `SOLO-WO-022 R01` by reading and running that path**. Mere download or drafting is not issuance. No parallel orders are authorized.

| Item | Frozen authority |
| --- | --- |
| Work order | `SOLO-WO-022 R01` |
| Package | **Administrative closure of accepted `R00.F` and selection of `R00.G` as NEXT candidate only** |
| Repository | `https://github.com/provelopment/provelopment-foundation-solo` |
| Target | The Owner's one existing verified Solo checkout, `main`, sole `origin` above |
| Reviewed `main` | `7863342d6b60b5af1b28bd8b41e29fe9efbe6765` — R06 canonical report commit |
| Accepted evidence | `SOLO-WO-021 R06` correction commit `90ca85f339b0a1691e877a4de4f06e29aab82a18`; canonical R06 report; independent Chief Architect `ACCEPT` of R06 and its three contact-contract corrections in this conversation |
| Required separate Owner handoff | Owner explicitly installs this R01 and instructs agent to **read and run `.documents/WIP/workorder-active.md`** for `SOLO-WO-022 R01` |
| Intended end state | `R00.F` removed from ROADMAP; `R00.G` remains pending and is the **only** NEXT candidate; ACTIVE contains **only its neutral title**; accepted R00.F contract remains durable in Architecture/Product; one GitHub-published report |

### 1. Exact objective and hard boundary

Close **only** the already independently accepted `R00.F` decisions-only package, under Architect SOP §8. This is **WIP reconciliation and narrow acceptance-status parity**, **not** another R00.F contract revision, not `R00.G` implementation, and not a Stage 00 completion claim. The Owner's issuance and matching handoff provide the *separate express closure authorization* required by the SOP.

The binding first-release contract remains as published: exactly **22** registered `1.0.0` JSON modules, including `interaction.contact`; a separate safe public `mailto:` action; exactly three contact data fields; the native five-key UTF-8 URL-encoded POST with server-owned controls and no-JavaScript functionality; seven truthful outcomes; private recipient and replaceable `ContactDelivery` port; fixed bounds, rate controls, token expiry and single-dispatch policy; privacy notice and no default core message retention; static booking/directions links; analytics **off**; advanced forms excluded. `F01`–`F17` remain `NOT RUN` and `C1`–`C4` `UNVERIFIED`. **No implementation, test, delivery, release or deployment acceptance is inferred.**

**Out of scope:** All security procedure/remediation/advisory work, historic PR #1 and its branch, provider selection or operation, secret/configuration changes, application code, schemas, tests, CI, module changes, other Stage 00 packages, Stage 01–13 execution, architectural redesign, additional documents, extra WIP files, PR creation/merge, release, deployment and all other repositories. Do not alter A1, its references or the established private reporting channel. No decision from the accepted R00.F contact contract may be reinterpreted, weakened or silently reopened.

### 2. Preflight and first commit — freeze Owner-installed closure authority

1. Read in the repo-mandated order: `README.md`, `AGENTS.md`, Product, Architecture, Architect and Implementation SOPs, all four WIP files and the current canonical report. Inspect the real Git checkout and status; do not assume a path or permission based on this text.
2. Verify exactly one Solo root/worktree, real Git root, branch `main`, sole expected `origin`, tracking ref, `HEAD`, clean or explained working tree, no unknown modifications, no stashes or second checkout. The **sole expected initial local modification** is the Owner's installation of this R01 in `WIP/workorder-active.md`. Check the R01 edition and the separate matching Owner activation; neither the previous R06 order nor this draft alone authorizes execution. STOP on any unexpected dirty file, authority mismatch, or substantive upstream drift.
3. Independently match these pinned source blobs from reviewed `main` and the actual pre-installation checkout. A later **report-only** commit may be tolerated after verified ancestry and unchanged source anchors; any substantive change requires STOP, not an assumption:

| File | Expected Git blob at reviewed R06 `main` |
| --- | --- |
| `.documents/WIP/workorder-active.md` **before Owner installation** | `99ded895032e4a79331db65a52aa3dcf0e2ca97c` |
| `.documents/Architecture/ARCHITECTURE.md` | `f702617e274dd9ffe22b963e679ca935cd3a30eb` |
| `.documents/About/FOUNDATION-SOLO.md` | `50a58123b2ec5797635eec87cdd788ee28e2abf7` |
| `.documents/WIP/plan-roadmap.md` | `613d3fcb979c4e3687438136f9c286a54fd6a07c` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/SOP/sop-architect.md` | `0396251245180b40ee896b50a3456b92835f9cb1` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/Reports/execution-report.md` | `c640d2730074b875af3ca666f7ed0da805001b01` |
| `README.md` | `ed492cb689276a2a8b2112438a7f7cae941af233` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |

4. Confirm the **four and only four** WIP files exist; ROADMAP still has both `R00.F` and `R00.G`, NEXT is neutral with no candidate, and ACTIVE still contains the frozen R06 **before installation**. Confirm the current published report documents R06 and the owner's R01 installation is the only uncommitted change.
5. **Before editing any other file**, calculate the installed R01 byte length and SHA-256; commit/push **only the exact Owner-installed `workorder-active.md`** as the issuance-freeze commit; read back the blob and remote HEAD and verify its bytes/checksum. No source/ROADMAP/NEXT edits may precede this first remote-verified freeze. Follow Architect SOP §6B, without extra PR, force push, reset, destructive clean or additional worktree.

### 3. Administrative reconciliation — perform in this order

**A. ROADMAP — remove only accepted R00.F.** In `.documents/WIP/plan-roadmap.md`, delete **exactly** the one current bullet beginning `- **R00.F — Contact and integrations:**` under Stage 00's `**Remaining packages**`. Preserve `R00.G` word-for-word, the Stage 00 heading/outcome/exit gate, all later stage obligations and their order. Stage 00 remains **open** while R00.G is pending. Add no completion history, acceptance matrix, archive note or policy to ROADMAP.

**B. NEXT — select R00.G as the one candidate, do not issue.** Populate `.documents/WIP/plan-next.md` beneath its existing title with a **single bounded** `R00.G — Lifecycle and nonfunctional standards` candidate, derived from the unchanged ROADMAP R00.G bullet and Architecture §6.5/§6A reservations. It should contain only: identifier/objective, prerequisite/readiness, proposed decisions-only scope/deliverables, and verifiable documentary exit conditions. Include first-release setup/preview/content-update/upgrade/recovery responsibilities; immutable release identity and independent Installation/deployment portability; supported versions and response responsibilities; accessibility/performance targets; precise acceptance matrix. Mark dependency/readiness honestly, without inventing Owner-approved numerical values, service/provider choices or evidence that tests passed. The candidate **does not** authorize execution. Do not populate a second candidate, work order, or Stage 01 action.

**C. Durable Architecture status parity — minimal, no contract edits.** In `.documents/Architecture/ARCHITECTURE.md`:
- At §6.5's `R00.F` bullet, replace **only** the outdated ending that says R00.F is “not yet closed” and pending review with a brief truthful statement that the documentary `R00.F` contract is **independently accepted and administratively closed by this Owner-issued reconciliation**. Keep its full decided-scope and Stage 08/09/10/11 allocations unchanged.
- At §6B's **opening status paragraph only**, update the historical `SOLO-WO-021 R04` edition-specific status to state that the accepted first-release documentary contract incorporates the later R05/R06 clarifications, and that closure is administrative, **not** implemented contact functionality. Do **not** restate, modify, add or remove any module, field, limit, method, output, port, provider, privacy, abuse or token rule.
- If another direct **administrative status** sentence in the same file affirmatively claims R00.F is still unaccepted/open despite the above, repair that sentence alone with a cited exact line in the report. Do not edit historical R00.E-at-the-time reservations, substantive §6B.1–§6B.12 rules, or unrelated accepted contracts. If a meaningful design contradiction would require more, STOP and report; do not expand scope under “parity.”

**D. ACTIVE — clear only after A–C have been safely staged/committed according to §6B.** Replace the frozen installed `workorder-active.md` with exactly its neutral identifying heading and final newline:

`# Foundation Solo — Active Work Order`

No Approved Scope, `NONE` essay, historical work order, policy, execution evidence or preauthorization remains. The **Owner's R01 authorization** allowed this final clearing; it does **not** authorize running R00.G. Do not clear ACTIVE prematurely if prior steps cannot complete. Git history preserves the R01 issuance-freeze and earlier orders.

**E. Canonical report.** Overwrite **only** `.documents/Reports/execution-report.md` with the factual administrative closure evidence, publish it as required by the SOP and read it back from GitHub. Preserve the report as the one current report, not a history/archive. Report exact starting/freeze/reconciliation/final commit and tree identities and the path diff. Do not prewrite the report's own commit SHA inside its content. Report any incomplete step honestly rather than claiming closure.

### 4. Allowed paths and operations — exact ledger

| Path | Authority |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs R01; agent freezes **unaltered** R01 first and finally clears ACTIVE to its sole neutral heading after successful reconciliation |
| `.documents/WIP/plan-roadmap.md` | **Delete only** the existing R00.F bullet; preserve all other ROADMAP content and pending Stage 00 |
| `.documents/WIP/plan-next.md` | **Insert one** bounded R00.G candidate derived from ROADMAP; never an issued work order |
| `.documents/Architecture/ARCHITECTURE.md` | **Narrow R00.F administrative-status parity only** in §6.5 and §6B opening status, plus strictly necessary identical-status correction if discovered |
| `.documents/Reports/execution-report.md` | Overwrite/publish the **one** canonical evidence report |
| Product, SOPs, `plan-outlook.md`, `README.md`, `AGENTS.md`, `.github/**`, all other files | **Read only; no edits**. If a necessary change lies here, STOP rather than widen the order |
| Git | Ordinary §6B staging/commits/push/read-back confined to authorized paths on verified `main`; no branch creation, PR, merge, reset, clean, history rewrite or deployment |
| Temporary material | Only Solo `.temp/` or active Cline-supplied temp area; clean task-owned disposable artifacts; never remove unknown items |

### 5. Acceptance gates — independently verifiable

| Gate | Evidence needed for PASS |
| --- | --- |
| G01 Authority | Correct Owner-installed R01 and distinct Owner activation; pinned source anchors, checkout, branch, remote and WIP baseline independently checked |
| G02 Freeze | R01 exact byte identity frozen, pushed and read back as **ACTIVE-only first commit before any other edits** |
| G03 ROADMAP | Precisely the R00.F bullet removed; Stage 00 and R00.G retained, no other backlog history/pruning |
| G04 NEXT | Exactly one R00.G documentary candidate with readiness, scope and exit conditions; no numerical/provider approvals invented; no execution authority |
| G05 ACTIVE | Exactly one identifying heading and final LF after reconciliation; no live order remains |
| G06 Durable parity | Architecture says R00.F documentation accepted and administratively closed; full §6B contract preserved verbatim apart from opening status; no implemented capability claimed |
| G07 Contract preservation | All 22 module IDs/contracts, five enhancement kinds, five-key native form transport, field bounds, outcomes, token expiry, rate limits, privacy/no-retention, analytics-off and future proof cases unchanged |
| G08 Scope purity | No security-related edits, code, provider, release, deployment, PR or branch mutations; only five authorized source/report paths changed |
| G09 Honesty | `F01`–`F17` still `NOT RUN`; `C1`–`C4` still `UNVERIFIED`; no R00.G decision or test presented as approved/executed |
| G10 Git/report | Correct published commit/tree and remote correspondence, clean working tree, one worktree, exactly four WIP files, single overwritten report and task-temp cleanup evidence |

**STOP / partial execution:** If any gate fails, source drift or unapproved work exists, or an authorized edit cannot be safely completed, preserve the evidence and Owner data, do not claim R00.F closed, and publish a factual `STOPPED`/`FAILED` report when safe. No new work-order file, backlog, PR, provider action or alternative branch. All future work remains behind its own separate Owner-issued order.

### 6. Final handback

The agent reports the exact remote commit/tree, changed paths, all ten gates and their evidence, and returns for independent Chief Architect `ACCEPT / REVISE / STOP`. This closure order **must not** itself issue an R00.G implementation order or prompt, run any R00.G task, claim Stage 00 closed, alter security governance, or make product release/deployment claims. The Owner controls any subsequent distinct issuance.
