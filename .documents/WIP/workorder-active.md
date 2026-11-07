# Foundation Solo — Active Work Order

## Approved Scope — `SOLO-WO-019` / `R02`

| Field | Owner-issued instruction |
| --- | --- |
| Marker | **`OWNER-ISSUED`**, effective only when the Owner installs this exact document in `.documents/WIP/workorder-active.md` **and** sends the matching `SOLO-WO-019 R02` handoff |
| Objective | Codify **proactive security by design** as a permanent, cross-cutting Foundation Solo requirement while Solo has **no application code** |
| Classification | One bounded documentation/architecture-governance implementation; no application code, deployment, release, or security remediation |
| Repository | `provelopment/provelopment-foundation-solo`; expected local root `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verify actual root) |
| Reference baseline | `main` was `4082ab7be2ae4648f360548926a7e25672fa0d03` when prepared; verify actual local and remote HEAD before edits |
| Existing review artifact | PR #1: `https://github.com/provelopment/provelopment-foundation-solo/pull/1`, branch `docs/solo-wo-018-security-register`, known head `daa2cb98625c67fc33f29eb0cdc78e6026f336d9` |
| Previous order | This R02 **replaces** the Owner-installed `SOLO-WO-019 R01` in ACTIVE. It **removes the prerequisite** that `SOLO-WO-018` be closed or PR #1 merged before security-by-design codification. Existing `REVISE`/`STOP` reports remain truthful history, not execution blockers under R02. |

### 1. Explicit Owner decision and simplified execution

By installing this order and sending its matching handoff, the Owner:

1. Adopts **security by design, from planning through maintenance**, as a core Solo product principle. An external vulnerability is a source of preventive requirements, **not automatically an identified vulnerability in Solo**.
2. Authorizes using the **existing PR #1 and its security-register branch** as the starting review artifact, extending that branch with the documentation below. **Do not require PR #1 to be merged first; do not create a second PR merely to continue this task.** Update PR #1's title/description to reflect the consolidated scope. The combined PR awaits one independent Architect review.
3. Expressly prioritizes this documentation work ahead of the existing NEXT candidate. Preserve all remaining ROADMAP work including `R00.F`, `R00.G`, and `R11.A`. NEXT is already neutral and **must remain untouched**. No stage or package is deemed complete or accepted by this task.
4. Authorizes the implementation agent's routine Git operations for the explicitly named paths and branches under Architect SOP §6B. This approval is **not** permission to merge PR #1, release, deploy, self-accept, or clear ACTIVE.

**Do not STOP solely because WO018 has an unresolved `REVISE`, PR #1 remains unmerged, or the security register is absent on `main`: those are precisely the prior dependency errors resolved by this Owner-issued replacement.** The register is present on the approved PR #1 branch. Do STOP if actual repository/authorization/safety evidence contradicts this order.

### 2. Deliverable — one lasting, enforceable security protocol

Codify this concise lifecycle in the appropriate **existing** canonical documents, with links rather than competing copies:

**Identify → Verify → Assess → Prevent/Respond → Validate → Reassess.**

- **Identify:** security advisories, dependency/technology changes, design threats, code findings, vulnerability reports, incidents, and release/update risks; protect confidential disclosure through existing `.github/SECURITY.md`.
- **Verify:** record authoritative sources, dates, affected versions/components and evidence, not headlines or unproven claims.
- **Assess:** classify applicability to Solo's exact current revision, trust boundaries, exposure, consequences, confidence, severity, owner and unknown/out-of-scope assets. Distinguish vendor risk from Solo-specific risk.
- **Prevent/Respond:** consider secure-by-default designs *before* implementation; specify threat/abuse and refusal cases; propose controls or remediation with an Owner-approved bounded work order when changes are required. Do not turn the security register into a parallel task queue.
- **Validate:** make security acceptance conditions explicit in relevant future work orders. Require proportionate source/configuration/dependency review, negative and abuse tests, least privilege, secret isolation, safe public projection, and secure release/update/rollback evidence **when those capabilities exist**. Never mark tests not run as PASS; never mark a finding `Resolved` without evidence.
- **Reassess:** revisit the register when dependencies, frameworks, adapters/providers, deployments, release/update designs or vulnerabilities change, and before release. Leave exact cadence, response SLA, supported-version and provider decisions for Owner approval in the proper existing roadmap packages.

**Worked example:** Existing `SOLO-SEC-2026-001`, WordPress Core update-window CVE-2026-119977, stays an independently sourced **external advisory not applicable to the assessed code-free Solo repository**. Demonstrate the pipeline from source verification to applicability assessment, proposed future safeguards (e.g. verified release artifacts, staging isolation, atomic activation, least privilege and rollback), objective future tests, and a trigger for reassessment if Solo ever adds relevant technology. **No WordPress patch, code, CMS adoption, scan, or deployment is required or claimed.**

### 3. Exact permitted permanent-file changes

| Path | Change permitted |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Owner replaces** R01 with this exact R02. Agent may only commit/push the Owner-installed bytes unchanged as a frozen-issuance commit on `main`. Do not self-edit, clear, or add agent acceptance. |
| `.documents/About/FOUNDATION-SOLO.md` | One compact product-level requirement: security is proactive and continuous, planned before code. |
| `.documents/Architecture/ARCHITECTURE.md` | One integrated cross-cutting security subsection: risk/trust-boundary assessment, safe defaults, validated supply chain, release/update integrity, defensive tests and reassessment points; preserve all accepted EFS, routing, D/O/E, C1–C4 contracts and statuses. |
| `.documents/SOP/sop-architect.md` | Minimal Architect procedure for the six-step security lifecycle, threat/risk evaluation and independently verifiable gates; **do not replace or duplicate §6B Git procedure** or weaken A1. |
| `.documents/SOP/sop-implementation.md` | Minimal implementation-agent obligations for security evidence, negative/refusal checks, truthful `NOT RUN` / `NOT APPLICABLE` statuses and private disclosure handling; refer to Architect SOP rather than restating it. |
| `.documents/Security/security-issues.md` | Preserve the PR #1 register, its stable issue ID and narrow WordPress applicability; add only concise process links/trace to the new protocol, if missing. |
| `.documents/WIP/plan-roadmap.md` | Strengthen **only** the pending `R00.G` security-response/lifecycle contract and `R11.A` security review expectations, and `R10` update/recovery acceptance if genuinely necessary. Preserve all pending work, stage identities and unrelated text. |
| `README.md` and `AGENTS.md` | Minimal coherent document-map and security-by-design references; **preserve** all existing authorization, WIP and product rules. Preserve the PR #1 register links. |
| `.documents/Reports/execution-report.md` | Overwrite **the one** existing canonical report with actual run evidence; push a report-only commit on `main`. |

**Do not modify** `plan-next.md`, `plan-outlook.md`, `.github/SECURITY.md`, application/dependency files, other repositories or any other permanent path. Do not create another WIP file, SOP, register, report archive or status tracker. There are exactly four WIP files.

### 4. Execution and publication

1. Read `README.md`, `AGENTS.md`, Product Definition, Architecture Charter, both SOPs, the four WIP files and current execution report **in the required order**. Confirm correct actual Solo checkout, `main`, one worktree, exactly one `origin` with the correct URL, base revision/remote agreement and Git status. The sole permitted existing dirty file is the **Owner-replaced R02 ACTIVE**, unless another known and separately authorised change is verified. Verify the full order ID, edition and `OWNER-ISSUED` marker plus a separate matching Owner handoff. STOP on unexplained dirty or Owner-owned data; no reset/clean/stash/force push.
2. Verify PR #1 still corresponds to the Solo repository and contains the existing canonical register on its known branch/head. Confirm NEXT is already the neutral one-line title and ROADMAP still contains the pending `R00.F`, `R00.G` and `R11.A` obligations. **No prerequisite merge or former-order closure check applies to this R02.** If PR #1 has changed since preparation, inspect the change and use it only if the provenance/scope remains clear; otherwise STOP with the concrete discrepancy.
3. Commit/push the **exact Owner-installed R02 ACTIVE bytes only** to `origin/main` as a frozen issuance. From the **same checkout**, work on the existing PR #1 branch, without rebasing or rewriting its published commit. Incorporate `main` by a non-destructive branch-side merge only if required, then update only the authorized documentation files. Commit and push to that same branch, update PR #1 description/title, **leave it open and unmerged**, and verify remote branch commit/tree and changed paths.
4. Return safely to `main`, overwrite and publish **only** the current `.documents/Reports/execution-report.md` on `origin/main`, and verify GitHub readback. Report the report-commit SHA in chat, not inside the report. **After publication**, record actual `git status --porcelain`, local `HEAD`, upstream and remote SHA in the chat handoff; document both Solo `.temp/` and Cline-provided temporary workspace cleanup or limitations. Retain ACTIVE and neutral NEXT until a later Owner-authorized reconciliation.
5. Apply Architect SOP §6B as **the sole Git/GitHub procedure**. Ordinary commit, push, branch, PR-edit and remote-verification mechanics are delegated; no separate approvals for individual commands. No PR merge, release, deployment, provider/DNS/TLS/account action, dependency installation, secret access, live exploitation, destructive action, new worktree/clone or other-repository write.

### 5. Acceptance criteria (independent Chief Architect review)

Mark each **PASS / FAIL / UNVERIFIED**, or reasoned **NOT APPLICABLE**:

- **G01 — Authority/provenance:** matching installed R02 and Owner handoff, correct root/remote, safe starting state, prior PR #1 branch verified, unchanged frozen scope published.
- **G02 — Core requirement:** Product Definition and Architecture explicitly make security-by-design a requirement from planning to maintenance, with no code-existence or security-certification fiction.
- **G03 — Process:** the six steps have clear inputs, role ownership, confidentiality, response decision, evidence, objective closure and reassessment triggers; no second governance authority is created.
- **G04 — Future lifecycle gates:** SOPs and ROADMAP require proportional security proof in relevant future implementation, release and update work; A1, existing contracts and no-early-release boundaries preserved.
- **G05 — WordPress exemplar:** the original ID and source/applicability remain intact; controls are proposed, not implemented; this case visibly exercises the security process.
- **G06 — Repository purity:** exactly four WIP files, no new parallel trackers/SOPs, authorized files only, private intake kept private, no sensitive public disclosure.
- **G07 — Git/report:** frozen issuance on `main`, updated existing PR #1 open/unmerged, remote verified, canonical execution report published on `main`; actual final Git status/remote equality evidenced.
- **G08 — Cleanup/truthfulness:** actual checks and limitations reported, Solo and Cline temp disposition specified, `NOT RUN` and `NOT APPLICABLE` used honestly; no self-approval, merge or deployment claim.

There is **no application code, CI, dependency or deployment to test**. Such checks are `NOT APPLICABLE` for this documentation-only order, not PASS. The Architect will review the **exact published report and PR** and return `ACCEPT / REVISE / STOP`; acceptance alone never authorizes merge or WIP closure.

### 6. STOP and handback

STOP only for **real** authorization, repository, confidentiality or scope conflicts: mismatch between installed R02 and handoff, unexpected changed PR provenance, unknown local edits, inability to preserve the four-file WIP rule, secret leakage, or an unapproved contract change. Preserve existing work and publish a factual STOPPED report when safe. **Do not resurrect the removed WO018-merge prerequisite.**

Hand back: PR #1 link, frozen-order commit, updated PR head/tree, report commit, path ledger, validation results, remaining limitations, final local Git status and temporary-workspace disposition. Neither agent may self-accept or merge this work.
