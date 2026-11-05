# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-010`  
**Edition:** `R01`  
**Title:** R00.B — Accepted address/locale contract administrative closure  
**Approval marker:** `OWNER-ISSUED` only upon exact Owner installation and matching direct handoff  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Target branch:** `main`  
**Required starting `origin/main` HEAD:** `d70f01d356af327d329fe6459de4001f15829a54`  
**Matching handoff:** `SOLO-WO-010 R01 — Implementation Agent Handoff`  
**Classification:** Administrative documentation and WIP reconciliation only  
**Quality standard:** A1 documentary accuracy, scope confinement and GitHub-verifiable evidence  
**Independent review:** Chief Architect `ACCEPT / REVISE / STOP`  
**Closure authority:** Owner

> **Issuance boundary.** This prepared order is not executable until the Owner installs this exact complete replacement in `.documents/WIP/workorder-active.md` and separately delivers the matching `SOLO-WO-010 R01 — Implementation Agent Handoff`. No Agent may self-issue, amend, accept or extend this order.

**Frozen Approved Scope:** Sections 1–8.

## 1. Objective and closure basis

Administratively close the accepted R00.B address/locale contract and advance the sole NEXT candidate to R00.C, without changing any accepted architecture or starting implementation.

The completed architectural work comprises:

- `SOLO-WO-009 R01`: Initial canonical codification of Owner-approved B01–B12, including Unicode routes and author-selected case-sensitive public routing.
- `SOLO-WO-009 R02`: Corrective documentation incorporation of optional locale-less Page entry routes (B03) and accepted-versus-open language for B01/B04/B05/B08/B09.

**Independent disposition:** The Chief Architect reviewed `SOLO-WO-009 R02` at GitHub commit `d70f01d356af327d329fe6459de4001f15829a54` and returned `ACCEPT` for its corrective documentation scope. The substantive B01–B12 decisions are present in the existing canonical Product and Architecture authorities.

**Owner adjudication incorporated by issuance:** In installing and issuing this order, the Owner:

1. Confirms that the matching R01 and R02 Implementation Agent handoffs were genuinely Owner-issued.
2. Accepts the previously disclosed R01 procedural deviation: the extra authorized-path B11 content commit `208e03d7c506be526a2492dd80d626107111b100`, following content commit `df84dfd4e83d7984aa0c3f06497aa6d8f8e3831c`.
3. Treats that deviation as a **one-time accepted procedural exception**, not retroactive alteration of the original work-order terms and not permission for future agents to exceed issued scope.
4. Authorizes the precise R00.B administrative closure operations defined below, subject to safety preflight and truthful evidence.

If the Owner cannot affirm either prior handoff or does not accept the C3 exception, **STOP**; the agent may not manufacture or infer that approval.

This closure is limited to the accepted **documentation contract**. No runtime capability, source code, tests, schema implementation, language selector, released product or deployment is represented as complete.

## 2. Closure invariants

Preserve all accepted R00.A and R00.B decisions unchanged.

In particular:

- Exactly one Entity per Installation, expandable through the EFS ownership hierarchy.
- Explicit persistent EFS IDs and the single versioned authoritative publication manifest.
- Configurable HTTP(S) canonical base URL with optional path prefix and authorized aliases.
- Explicit Page route ownership, optional localized slugs and validated Unicode paths.
- Case-insensitive public matching by default, with author-selected case-sensitive matching per localized route claim.
- Exact-case unlisted behavior and strict public/unlisted collision refusal.
- Full region-qualified locale tags, independent locale-segment normalization and explicit locale preference mappings.
- Optional explicit locale-less Page entry routes identifying the Page before language selection.
- Truthful same-Page, public-only fallback.
- No forced language based on geography; direct explicit-locale navigation and account-free language selection remain first-release requirements.
- No routing inference from location, EFS names or URL text.
- Strict Hexagonal Architecture and all existing authorization boundaries.

This order may **not** edit any permanent architectural or product authority. The goal is to record accepted completion in WIP, not to amend the accepted contract.

## 3. Exact permitted permanent paths

| Path | Authorized operation |
|---|---|
| `.documents/WIP/workorder-active.md` | Owner installs this exact order. Agent preserves it unchanged in Phase A, then resets it to the neutral header in Phase B. |
| `.documents/WIP/plan-roadmap.md` | Delete exactly one R00.B package bullet from Stage 00. No other additions, deletions or rearrangements. |
| `.documents/WIP/plan-next.md` | Replace the existing R00.B candidate with the exact R00.C candidate in §4. |
| `.documents/Reports/execution-report.md` | Overwrite the sole canonical report with the factual closure evidence in Phase C. |
| `.git/**` | Only ordinary Git metadata required for the explicitly authorized commits and pushes, governed by Architect SOP §6B. |

**All other permanent paths are read-only.** This includes Architecture, Product, README, AGENTS, both SOPs, OUTLOOK, source files, dependencies, LICENSE, `.github/**` and all other repositories.

Do not create another WIP document, archival report, decision register, branch, checkout or worktree.

## 4. Exact intended WIP results

### 4.1 ROADMAP

Current Stage 00 contains R00.B–R00.G.

Delete **exactly the one current R00.B package line** beginning:

`- **R00.B — Address/locale contract:**`

Do not alter any other line.

After the edit:

- R00.B must occur **zero times** in ROADMAP.
- The remaining Stage 00 packages are exactly R00.C, R00.D, R00.E, R00.F and R00.G, in that order.
- Stage 00's outcome and exit gate remain unchanged.
- Stages 01–13, all their packages and exit gates remain unchanged.
- No completed-work summary or closure narrative is inserted into ROADMAP.

### 4.2 NEXT

Replace `.documents/WIP/plan-next.md` with exactly the following candidate content:

# Foundation Solo — plan-next (Next Candidate)

## R00.C — Property/graphic contract

**Objective:** Prepare an Owner-reviewable logical contract for authored EFS properties, inheritance and suppression, safe public/private projection, authored location facts and cascading graphics, without revising the accepted R00.A identity/publication or R00.B address/locale contracts.

**Prerequisites and readiness:** R00.A and R00.B architectural contract packages are accepted and Owner-authorized for closure. Applicable Product and Architecture invariants are available. R00.C remains pending in Stage 00. This is a decisions-only candidate, not an implementation task.

**Proposed scope:** Settle the typed property-value model, authoring declarations, absence versus override versus explicit suppression, direct-ancestry inheritance and provenance, public/private projection and validation; structured location facts across Entity, Facet, Site, Page and representation, without any routing or locale inference; graphic-role catalogue, authored asset references, the generic public graphic library, role-specific fallback and suppression/refusal behavior. Preserve one-Entity ownership and the distinction between logical property contracts and later physical schema/implementation choices.

**Proposed deliverables:** A bounded R00.C Owner decision proposal with explicit credible options and recommendations; precise proposed amendments to existing canonical Product/Architecture authorities where needed; a conceptual positive/refusal matrix for inheritance, suppression, location independence, privacy and graphic fallback. Distinguish accepted principles from open technical decisions. Do not select a framework, write executable code, or claim an unapproved physical schema.

**Verifiable exit conditions:** Each blocking R00.C issue has an explicit Owner decision or clearly identified blocker; decisions preserve R00.A/R00.B and Solo's five-level EFS property cascade; public/private projection, explicit suppression, direct-ancestor inheritance, independent authored location and graphic fallback have testable positive/refusal semantics; Stage 00 remains open until its remaining packages and exit gate are accepted.

This NEXT candidate is **non-executable**. No R00.C implementation or document amendment is authorized by its presence.

### 4.3 ACTIVE

After Phase B, `.documents/WIP/workorder-active.md` must contain **exactly**:

```text
# Foundation Solo — Active Work Order
```

with one trailing newline and no other text.

This is the normal neutral state: no current executable work order. It does not pause or freeze the project.

### 4.4 OUTLOOK

`.documents/WIP/plan-outlook.md` remains byte-identical.

### 4.5 REPORT

The one `.documents/Reports/execution-report.md` must be overwritten in Phase C with accurate closure evidence. Never retain the previous report as a second file; Git history preserves it.

## 5. Baseline and preflight

**Required GitHub `origin/main` starting HEAD:**

`d70f01d356af327d329fe6459de4001f15829a54`

Verify exact baseline blobs before any authorized write:

| Path | Required Git blob |
|---|---|
| `.documents/WIP/plan-roadmap.md` | `b3aa0e52ad3383256386a938408905536bc090bb` |
| `.documents/WIP/plan-next.md` | `375aee17fc9f8ae5355c119394fb8dca030cab23` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/WIP/workorder-active.md` before Owner installation | `3aad9411e2f1f15fc3e1554ce3bbefc5c79be59b` |
| `.documents/Reports/execution-report.md` | `3b7c896a77a86311fb5bb0efa5dc1ce089898ed5` |
| `.documents/Architecture/ARCHITECTURE.md` | `5ebabc7e056c582035f357e28015f9dc8333d39d` |
| `.documents/About/FOUNDATION-SOLO.md` | `95b8fd753dc90834811d5050c62a1c5c7a30ef2a` |

The Architecture and Product blobs are **read-only preservation controls**.

Follow all applicable repository safety, single-checkout, Git verification and reporting requirements in the existing SOPs.

The only expected initial change is the Owner-installed replacement of ACTIVE with this exact issued order. Any other unexpected change, baseline mismatch or authority conflict is a STOP condition.

## 6. Authorized execution and publication sequence

### Phase A — Preserve closure authority

After verifying Owner issuance and the matching handoff:

1. Stage the exact Owner-installed ACTIVE bytes, unchanged.
2. Create **one ordinary commit** modifying only `.documents/WIP/workorder-active.md`.
3. Fast-forward push to the verified `origin/main`.
4. Independently verify the published commit, exact file bytes and remote HEAD before continuing.

Do not amend or reformat the frozen approved scope.

### Phase B — Perform exact WIP reconciliation

Before committing, validate the complete intended result.

Make only these three edits:

1. ROADMAP: remove exactly one R00.B bullet, with no added lines.
2. NEXT: replace its existing content with the exact R00.C candidate in §4.2.
3. ACTIVE: replace the issued order with exactly the neutral header in §4.3.

Verify OUTLOOK, Architecture, Product and all other read-only files are unchanged.

Stage **only** these three WIP files. Publish **one ordinary content commit** to `origin/main` and verify GitHub readback, final WIP inventory and exact resulting contents.

Do not include the execution report in this commit.

If the desired result requires altering any other file, STOP rather than expanding the scope.

### Phase C — Publish the closure execution report

After Phase B has been verified, use the standing report-only permission in the SOPs to overwrite and publish only `.documents/Reports/execution-report.md`.

The report must record:

- Work order and matching handoff identity.
- Baseline and actual GitHub revisions.
- Explicit Owner approval of the R00.B closure and the one-time R01 C3 procedural exception.
- Exact removal of R00.B from ROADMAP.
- Replacement of NEXT with the single R00.C candidate.
- Neutral ACTIVE and unchanged OUTLOOK.
- Preservation of canonical Architecture/Product content.
- Exact commit sequence and changed paths.
- Four-file WIP inventory and single current report.
- Actual verification actions and limitations.
- Explicit distinction between documentation acceptance and unimplemented application capabilities.
- Truthful STOP/failure or cleanup limitations where applicable.

Make **one separate ordinary report commit**, push and independently verify its GitHub presence.

Do not place the report commit's own SHA inside the report. Supply that SHA and GitHub URL in the handback.

**No other permanent edits or publication phases are authorized.**

## 7. Acceptance gates

| Gate | Requirement |
|---|---|
| G01 — Authority | Exact `SOLO-WO-010 R01` Owner-issued order and matching handoff; correct baseline |
| G02 — Closure basis | Prior accepted R00.B documentary scope and explicit Owner adjudication of R01 C3 identified accurately |
| G03 — Phase A | Exact issued order published alone, unchanged |
| G04 — ROADMAP precision | Exactly one R00.B bullet removed; R00.C–R00.G and all remaining stages preserved |
| G05 — NEXT precision | Exactly one R00.C candidate using the approved §4.2 text; no R00.B candidate |
| G06 — ACTIVE neutrality | Header-only ACTIVE after Phase B, with correct trailing newline |
| G07 — OUTLOOK preservation | Exact original blob retained |
| G08 — Canonical preservation | Architecture, Product, R00.A/R00.B decisions and all other read-only paths unchanged |
| G09 — Publication integrity | Exactly one Phase A order commit, one Phase B three-file WIP commit and one Phase C report-only commit |
| G10 — Inventory | Exactly four WIP files and the one canonical report |
| G11 — Honest evidence | GitHub-verified commits and scope, no invented software verification or deployment |
| G12 — Governance | No self-acceptance, no R00.C execution, no unauthorized changes |

For every applicable gate, provide direct evidence and classify the outcome as `PASS`, `FAIL` or `UNVERIFIED`; use `NOT APPLICABLE` only with an explicit reason.

Software build, unit tests, HTTP/browser tests, CI, release and deployment checks are **NOT RUN / NOT APPLICABLE** to this administrative documentation task. No product behavior is being implemented.

## 8. Explicit exclusions, STOP conditions and completion

**Not authorized:**

- Architectural decisions or revisions to B01–B12 or R00.A.
- R00.C decision-making or implementation.
- Application code, dependencies, schema files, tests or build tooling.
- Other WIP files, reports or archives.
- Other repositories, additional checkouts/worktrees, branches, PRs, merges, tags, releases, deployments or external provider actions.
- Destructive Git commands, force-push, history rewrite or retrospective editing of prior commits.
- Reclassifying the R01 C3 exception as a general procedure.
- Implementing software or claiming Stage 00 completion.

**STOP immediately** if the Owner-issued approval, handoff, baseline, exact path ledger or historical deviation adjudication cannot be verified; if an unrelated modification is present; if the R00.B ROADMAP entry does not match uniquely; or if any required output cannot be produced within the frozen scope.

**Required successful end state:**

- R00.B has been removed from remaining ROADMAP obligations.
- R00.C is the only NEXT candidate and remains non-executable.
- ACTIVE is neutral.
- OUTLOOK remains unchanged.
- The accepted B01–B12 contract remains canonical and unmodified.
- The current closure execution report is published to GitHub.
- Stage 00 remains open, with R00.C–R00.G pending.
- No application implementation has been started.
- The agent stops for independent Chief Architect review.

**The Agent must not issue its own `ACCEPT` or begin the next package.** The Owner retains authority to issue the next work order.
