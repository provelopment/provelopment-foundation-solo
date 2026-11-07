# Foundation Solo — Active Work Order

## Approved Scope — `SOLO-WO-018` / `R01`

| Field | Owner-approved value on issuance |
| --- | --- |
| Work order | `SOLO-WO-018`, edition `R01` |
| Marker | `OWNER-ISSUED` — effective **only** after the Owner installs this exact document in the single ACTIVE slot and sends the matching explicit handoff |
| Date | 2026-11-07 |
| Repository | `provelopment/provelopment-foundation-solo` (public); verified Owner-local root: `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` |
| Base ref | `main`; GitHub baseline observed during preparation: `6e98b93513dab0b511e9bd4f4df4bbd64063b576` — verify live local/remote revision before edits, do not assume it is unchanged |
| Classification | Bounded security-documentation and register-governance transition; **no software remediation, executable implementation, release or deployment** |
| Owner handoff | Separate Owner-sent message explicitly naming `SOLO-WO-018 R01` and confirming that this exact edition was installed in `.documents/WIP/workorder-active.md` |
| Independence | The Implementation Agent executes; the Chief Architect independently reviews. Neither agent self-issues, self-accepts, merges or deploys. |

### 1. Owner decision embodied by issuance

By **installing this exact approved scope and sending its matching handoff**, the Owner expressly decides and authorizes all of the following **for this task only**:

1. Introduce the dedicated canonical security-findings concern at **`.documents/Security/security-issues.md`**, outside WIP. It is a durable, evidence-linked **security issue and remediation register**; it is **not** a work-order queue, an alternative ROADMAP/NEXT/ACTIVE/OUTLOOK or an execution-report archive. The new concern is identified with narrow document-map pointers in `README.md` and `AGENTS.md`. The public register contains no confidential vulnerability details.
2. Prioritize this one bounded documentation task ahead of the currently selected `R00.F` NEXT candidate. This **does not complete, alter, accept, remove or work on** `R00.F`, `R00.G`, `R11.A`, or any ROADMAP obligation. On issuance, mechanically neutralize the pending `R00.F` entry in NEXT while preserving that obligation in ROADMAP. NEXT selection after this task is a **separate Owner-authorized closure/reconciliation action**.
3. Authorize one short-lived documentation branch and a pull request **without merge**; authorize the specific `main` commits and pushes below for frozen issuance/NEXT transition and the one canonical execution report. No other WIP workflow changes are permitted.

This Owner decision is **not effective merely because this text appears in a chat, downloadable draft or GitHub issue**. The Owner alone installs it in ACTIVE and sends the matching handoff. The agent must verify both before any non-report write.

### 2. Objective and completed deliverables

Establish a **single authoritative, reusable security-findings register** with a bounded, evidence-based assessment of the **repository as it actually exists now**. Seed it with the previously raised WordPress core update-window advisory, **CVE-2026-119977** (source facts to be independently checked by the agent), including its applicability, potential future relevance, defensive requirements and reassessment trigger.

Deliver exactly:

- `.documents/Security/security-issues.md` — a complete Markdown register with a defined upkeep convention, explicit evidence limits, finding index, detailed record for `SOLO-SEC-2026-001`, remediation/verification requirements, outstanding issues and change-history fields. It may contain additional findings only when grounded in concrete inspected evidence.
- Minimal, coherent pointers in `README.md` (project document map) and `AGENTS.md` (one authority per concern) identifying the register's limited purpose. Do not rewrite product governance or expand the four WIP files.
- An Owner-issued order frozen in Git history, an authorized neutral NEXT transition, a pushed documentation branch with an **open, unmerged PR**, and the single factual execution report at `.documents/Reports/execution-report.md` published to `origin/main`.

**Not a deliverable:** a production security certification or a code-level application penetration test. The published Solo repository currently contains governance/architecture documentation and no implemented application, dependency manifest, CI tests or deployment. The agent must reconfirm this at the reviewed revision, report limitations, and avoid marking untested product behaviour `PASS`.

### 3. Exact permanent-file write ledger

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs the exact frozen order. Agent may **stage, commit and publish those Owner-installed bytes only** as its separate issuance commit. Agent must not rewrite Approved Scope, overwrite it with a template, self-apply the `OWNER-ISSUED` marker or clear ACTIVE. |
| `.documents/WIP/plan-next.md` | **Only** after the above Owner issuance checks: replace the currently observed `R00.F — Contact and integrations` candidate with the existing one-line neutral NEXT title, `# Foundation Solo — plan-next (Next Candidate)` followed by exactly one LF. Preserve ROADMAP's `R00.F` obligation. If the observed candidate differs or the neutralization is not exact, STOP. |
| `.documents/Security/security-issues.md` | Create the one canonical register outside WIP; do not create per-finding report files, backup copies or parallel trackers. |
| `README.md` | Add only the minimum project-document-map entry/pointer for the canonical register and its purpose. Preserve existing statements on project maturity and authority. |
| `AGENTS.md` | Add only a compatible, narrowly worded reference within the existing *one authority per concern* map (Security owns public-safe security findings, not SOP/work authorization). Preserve all existing Owner, WIP and handoff rules byte-for-byte otherwise. |
| `.documents/Reports/execution-report.md` | Overwrite the **one** canonical report with this run's factual evidence and publish it to `origin/main`; do not create another report. |

**Read-only:** `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, both SOPs, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-outlook.md`, `.github/SECURITY.md`, `.github/**`, repository hygiene files and all other project paths. If authoritative contract changes beyond the narrow, expressly approved Security mapping are needed, STOP for a new Owner decision. Do not alter other repositories or the parent workspace.

### 4. Register content specification

The security register must have clear sections for:

1. Purpose and boundaries; assessed repository, commit, date, assessor; distinction between **current findings**, **external advisories/watch items**, and **future hardening requirements**. A security issue is not an implementation authorization.
2. Public-reporting safety and reference to the existing GitHub private vulnerability reporting channel in `.github/SECURITY.md`; no sensitive proofs, exploit recipes, exposed tokens, PII, internal addresses or confidential attack paths in this public repository.
3. Finding index with stable issue ID, short title, source/reference, source severity versus Solo-specific risk, affected component/scope, impact, status, owner/TBD, priority, evidence date and revisit trigger.
4. Detailed finding records: technical summary, affected versions/configuration where source-supported, applicability to the **actually assessed Solo revision**, evidence and gaps, threat/impact, concrete remediation or preventive safeguards, objective validation/closure criteria, and links to authorized implementation work when it later exists. Never report a vulnerability as remediated without tested evidence.
5. Assessment methodology, **checks actually run**, precise scope exclusions, unknowns and limitations; a concise future-review checklist as code and deployment capabilities arrive, without inventing a current vulnerability backlog.
6. Register maintenance rule and change history. Future discoveries are incorporated into **this** file with stable IDs and appropriate independent review and Owner-authorized edits; do not make this file a replacement for the four WIP slots.

#### Required initial record: `SOLO-SEC-2026-001`

- Title: **WordPress core full-update window — potential unauthenticated remote code execution**.
- CVE: **CVE-2026-119977**. Verify the official CVE/WordPress/CISA record and quote exact affected/fixed versions, severity, conditions and dates only where supported; link the primary sources and retrieval date. Do **not** copy the previous agent's claims as proof without source checking.
- Investigate repository contents, manifests and deployment evidence *actually present*, including WordPress-specific files, dependencies or integration declarations. The prior read-only report observed no WordPress and no implemented application; the agent must verify again at exact HEAD. If still absent, classify **`Not applicable — assessed Solo repository at <SHA>`**, **not** `Fixed`, `Mitigated` or "Solo cannot ever be affected." An external installation/service cannot be cleared without separate evidence.
- Record the vendor-level risk separately from **Solo's observed applicability**. Include a reassessment trigger if Solo adopts WordPress, integrates a WordPress-managed service, or acquires an update mechanism with comparable time-of-check/time-of-use exposure.
- Record implementation-agnostic future safety requirements (e.g. signed/integrity-checked artifacts, immutable or atomic publication, least-privilege permissions, controlled update staging, monitoring and rollback) as **proposed preventive controls to assess during later authorized lifecycle/security work**, not as implemented protections or defects in nonexistent code.

If no other repository-backed finding exists, explicitly say so within the bounded assessment; do not manufacture vulnerabilities or pretend that a static documentation-only review assures future software security. Preserve existing `.github/SECURITY.md` private reporting policy.

### 5. Preflight, execution and version-control procedure

Follow `AGENTS.md`, Architect SOP §6A–§6B/§7A–§8 and Implementation SOP §4–§5C/§9. This order grants these **outcomes and file boundaries**, not competing Git rules or a command-by-command replacement for §6B.

**Preflight:** Re-read governing documents in their mandatory order. Confirm the physical Solo root, `main`, current HEAD, `origin/main`, remote URL, one worktree, Git status, existing WIP file inventory, installed ACTIVE exact bytes and separate matching Owner handoff. The previously reported local dirty `workorder-active.md` represented an invalid order; the Owner must have **explicitly replaced** that material with this edition. Treat any unexplained differences/dirty state as STOP; do not reset, clean, stash, overwrite or discard Owner-owned material.

**Authorized Git flow:**

1. From the single verified checkout, publish a **separate frozen-issuance/NEXT-transition commit to `origin/main`**, consisting only of the exact Owner-installed `.documents/WIP/workorder-active.md` and the mechanically neutralized `.documents/WIP/plan-next.md`. Verify remote readback and preserve the earlier NEXT candidate in Git history. No amendment or force-push.
2. From that verified `main` revision, create **one short-lived branch** (suggested `docs/solo-wo-018-security-register`) in the same checkout. Create/update only the register, `README.md` and `AGENTS.md`. Validate the content and stage only those paths; commit, push and verify the exact remote commit/tree/blobs. Open one PR to `main` with scope and evidence; **leave it unmerged**.
3. Return safely to the existing checkout's `main` without discarding work. Overwrite **only** `.documents/Reports/execution-report.md` with the factual run report; commit and push that report to `origin/main`, then read back the remote report and commit. Keep ACTIVE installed and NEXT neutral until separate Owner-authorized reconciliation following independent review.
4. Report branch, PR URL, commit SHAs, base/head, tree IDs, exact changed-path lists, actual Git commands/outcomes, remote URL and clean final worktree. If a safe clean state is impossible, STOP and disclose the preserved state without deleting anything.

**Permissions:** commit/push on `main` **only** for the frozen order + NEXT transition and the canonical report; create/push one documentation branch and PR for the three documentation deliverables. No PR merge, auto-merge, release/tag/deploy, provider/account/DNS/TLS operations, dependency installs, security scanning against external networks, credential access or disclosure, destructive operations, force-push, history rewrite, secondary checkout, or other-repository write. No WIP evidence edits beyond the two named transition paths. The Owner retains all separate merger/release decisions.

**Temporary files:** Normal scratch may use only Solo's ignored `.temp/` and the agent's actually supplied Cline temporary workspace, never an extra checkout. Verify scratch is not tracked/staged; clean identifiable obsolete task-owned files/logs. Disclose justified retention with trigger and unresolved cleanup failures; never delete unknown/Owner-owned material.

### 6. Required report and acceptance gates

Overwrite and push `.documents/Reports/execution-report.md` on **every** run, including STOP/FAIL, when safe. It must state session date, `SOLO-WO-018 R01`, real starting and final revisions, exact paths and changes, Owner-issuance match, detailed actions, evidence links, test/check outcomes, PR state, privacy review, cleanup/retention, limitations, and stop/resume point. Do not embed the report's own eventual commit SHA inside the report; include it in the agent's chat handback after publication. The report is on `main` even while the review branch PR remains unmerged.

Independent Architect acceptance will use the following gates, marked **PASS / FAIL / UNVERIFIED** (or **NOT APPLICABLE** with reason when genuinely inapplicable):

- **G01 — Authorization and baseline:** exact Owner-installed ACTIVE ID/edition/marker and matching handoff; verified Solo root, one checkout, single correct remote, current refs, known status.
- **G02 — WIP purity:** exactly four WIP files; ACTIVE frozen in a published commit; NEXT neutralized exactly; ROADMAP including `R00.F` and `R11.A` and OUTLOOK untouched; no additional WIP/report file.
- **G03 — Register and document mapping:** canonical `.documents/Security/security-issues.md` created; stable schema; minimal matching README/AGENTS pointers; no competing authorities.
- **G04 — Source accuracy:** WordPress advisory facts supported by independently checked primary sources; vendor severity distinguished from Solo applicability; exact assessment SHA and limitations recorded.
- **G05 — Actionability and truthfulness:** remediation, future preventive controls, verification/reassessment criteria and finding status are specific; no invented code/hosting/test capability or unjustified finding closure.
- **G06 — Public repository safety:** no sensitive exploit/credential/private data; confidential reports routed to existing GitHub private channel; no unsupported public-security guarantees.
- **G07 — Git provenance:** scoped frozen order/main transition and report commits, scoped branch commit, pushed PR unmerged, remote readback of exact trees/blobs, no force-push or unauthorized writes, clean tree or fully reported blocker.
- **G08 — Cleanup and reporting:** one current report on main, truthful checks and `NOT RUN`/`NOT APPLICABLE` explanations, no task-owned scratch residue or unreported retention, no claim of self-acceptance.

For this **documentation-only** scope, build, runtime, application security testing, CI and browser gates are **NOT APPLICABLE or NOT RUN with the reason recorded**, never `PASS`. No stage or work package is accepted, consumed or removed by this run; no merge is implicit. The Architect independently verifies the exact GitHub report and PR content and issues `ACCEPT / REVISE / STOP` to the Owner. Closure/reconciliation of ACTIVE and NEXT requires a **separate** Owner-approved order.

### 7. STOP / escalation

STOP without further implementation changes if: issuance ID/edition/marker or matching handoff is missing; unexpected local dirty/Owner-owned material or revision divergence exists; the required path is disputed; WIP would contain a fifth file; actual facts contradict source assertions; the branch/remote or authorization is unsafe; confidential evidence would be exposed; a new architecture/provider/security-response decision is needed; or the bounded path ledger cannot be honored. Preserve state, do not silently substitute a different path or task, and publish the STOPPED canonical report when safe.

**This Approved Scope remains frozen after Owner issuance. The agent never marks it accepted or clears ACTIVE.**
