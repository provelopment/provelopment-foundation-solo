# Foundation Solo — Active Work Order

## Approved Scope — SOLO-WO-021 / R05

**Issuance status:** This is the Chief Architect's **single proposed replacement** for `SOLO-WO-021 R04` following independent `REVISE` at the published R04 report revision. It becomes **`OWNER-ISSUED` only after the Owner deliberately installs this entire exact file as `.documents/WIP/workorder-active.md` in the single existing Solo checkout and sends the separate matching Owner activation for `SOLO-WO-021 R05`.** Preparing or downloading this file does not itself grant execution authority. There must be only one installed active order; R04 and R05 must never run concurrently.

| Item | Frozen scope |
| --- | --- |
| Work order / edition | `SOLO-WO-021 R05` — bounded correction following R04 `REVISE` |
| Package | `R00.F` — **documentation consistency and A1 procedure simplification only** |
| Repository | `https://github.com/provelopment/provelopment-foundation-solo` only |
| Execution location | Owner's **one existing** Foundation Solo checkout, verified from actual Git metadata |
| Target | `main`; sole `origin` pointing to the above repository |
| Public source baseline | `21e439053016a26f09459a53a94819a1195cc0ff` (2026-11-07 R04 report commit) |
| Matching Owner handoff | Separate Owner message explicitly naming `SOLO-WO-021 R05` and instructing the agent to read and run `.documents/WIP/workorder-active.md` |
| Success state | R00.F **ready for independent Chief Architect acceptance review**, **not** self-accepted or administratively closed |

**Owner decisions already made and NOT reopened:** The first publication profile has **exactly 22** `1.0.0` registered modules with `interaction.contact` as module 22. The first-release three-field contact contract, limits, rate thresholds, 24-hour attempt-token deduplication, privacy notice, lack of default core message retention, truthful outcomes, static booking/directions links and analytics-off policy are governed by Architecture §6B. A1 in Architect SOP §7B is the **single** security/correctness acceptance standard. The Owner has **already archived PR #1 in GitHub** and the R04 agent already removed the obsolete branch. That historical housekeeping is **complete; no PR mutation, further branch deletion, re-archiving, reopening, or recurring issue tracking is authorized or required in R05**. A GitHub REST PR response showing `closed`/`unmerged` does not disprove an Owner-performed archive because that response may not expose the archived state.

### 1. Objective and strict exclusions

Repair the **specific five-documentary discrepancies** raised by the independent R04 review and any direct contradiction introduced by their repair, without redesigning the contact capability or expanding the security process. This is a short corrective continuation, not a new product specification or separate implementation stage.

**Deliverables:** (a) exact targeted Architecture Charter corrections to historical reservations, later-stage ownership, public-form-endpoint wording, and deleted-advisory reference; (b) remove the redundant standalone Architect SOP §7C section so that **§7B A1** is the single security acceptance authority, without weakening its safeguards; (c) honest, GitHub-published canonical execution report; (d) evidence sufficient for independent `ACCEPT / REVISE / STOP` review. The Owner's PR archival is a **completed external fact**, not a new implementation task or an unprovable R05 gate.

**No application code, JSON Schema artifacts, 23rd module, change to any existing module contract, new publication/manifest field, provider choice, provider account, credential access, CI/build/tests, deployment, release, GitHub PR action, other repository write, or R00.G implementation.** Do not consume or prune R00.F from ROADMAP, fill NEXT, clear ACTIVE, open a PR, merge, or create a fifth WIP file. No broad editorial cleanup or opportunistic reformatting.

### 2. Mandatory preflight and issuance freeze (MUST occur first)

1. Read in the actual `AGENTS.md`-mandated order: root `README.md`, `AGENTS.md`, Product Definition, Architecture Charter, both SOPs, all four WIP files, current canonical report, and `.github/SECURITY.md` as applicable. Inspect the actual Git checkout rather than relying on an assumed local path.
2. Verify root (`git rev-parse --show-toplevel` and resolved location), branch `main`, upstream, exact sole `origin`, one checkout/worktree, local/remote HEAD, staged/unstaged and untracked status, four-file WIP inventory, absence of uncontrolled files and exact installed R05 edition plus separate matching Owner handoff. The **sole expected initial worktree modification** is Owner installation of `workorder-active.md`; protect any other preexisting material. Check `.documents/Security/security-issues.md` is absent from current HEAD; do not restore it.
3. Verify the baseline Git blobs for the canonical files listed below. A newer published `main` HEAD consisting **only** of independently verified canonical report-only commits may be tolerated if all content anchors are unchanged; record the ancestry. On substantive drift, missing permission, incorrect installed edition or unknown dirty state, STOP without reset, cleanup or unapproved edits.

| File | Git blob SHA-1 at reviewed R04 HEAD |
| --- | --- |
| `.documents/WIP/workorder-active.md` **as committed before Owner installs R05** | `c69ec236c0f2d5c221fb90f0cd1928eaff5e8b5b` |
| `.documents/Architecture/ARCHITECTURE.md` | `120b8703e2e84b2b3b6d016b4a81270f57e77a7b` |
| `.documents/SOP/sop-architect.md` | `30bf8addb604b71b8c106665a202d7f8c6c8aba6` |
| `.documents/About/FOUNDATION-SOLO.md` | `50a58123b2ec5797635eec87cdd788ee28e2abf7` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/plan-roadmap.md` | `613d3fcb979c4e3687438136f9c286a54fd6a07c` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/Reports/execution-report.md` | `2bca0075ec5070c7ef973231c960d94b5163d503` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `README.md` | `ed492cb689276a2a8b2112438a7f7cae941af233` |

4. **Before editing any other file**, calculate installed R05 bytes and SHA-256, then commit and push the **unaltered Owner-installed R05 ACTIVE file alone** as the issuance-freeze commit. Read it back from GitHub and independently confirm exact blob/bytes/checksum and correct remote HEAD. The R04 report disclosed the reverse sequence: **do not repeat that deviation**. No source edit before the freeze and read-back succeed. Follow Architect SOP §6B; no reset, clean, force, history rewrite, clone or secondary worktree.

### 3. Architecture corrections — exact bounded scope

Edit **only** `.documents/Architecture/ARCHITECTURE.md` within its identified conflicting paragraphs/cross-references. The following are required resolutions, not alternative designs.

**C01 — Public form endpoint versus private delivery details (R04 review G05).**

- At §6B.2 (currently around line 1597), distinguish the **necessarily visible, publicly routable, server-owned HTML form submission action URL** and permissible non-secret CSRF/attempt tokens from the **private** internal recipient, credentials, provider identity/configuration, provider API/SMTP destination, diagnostic body and server-only transport mapping. The form's publicly observable `action` is not a secret and is never an author-supplied custom action URL. Do not imply internal mail-provider URLs are public.
- In §6A.12's `interaction.contact` module-to-port row (currently around line 1435), replace the absolute claim **“no recipient, address, provider, credential or endpoint appears in output”** with the precise distinction: the **public form action and safe control tokens may appear**, while internal recipient/address, delivery provider configuration, credentials, internal provider endpoint and transport diagnostics may **not**. Preserve all refusal/no-JS/port requirements.
- In §6B.6, where private endpoint detail is mentioned, qualify it as **internal provider/delivery endpoint detail**, not the browser-visible public action.
- Preserve the closed 22-module grammar, qualified representation context, single private recipient per Installation, incoming CSRF/origin validation and truthful delivery semantics. Do not invent or fix an actual runtime URL/path or add authoring keys.

**C02 — Correct R00.F versus later-stage ownership (R04 review G12).**

- Replace §6.5's **“R00.F — delivery and contact implementation”** bullet with an accurate **R00.F decisions-only** description, referring to §6B and its later implementation allocation. Contact form, mailto and provider adapter implementation live in **Stage 08 (`R08.A`–`R08.C`)**; static booking/directions and analytics-off implementation belong to **Stage 09 (`R09.A`–`R09.C`)**; establishment, deployment/update/recovery belong to **R00.G's lifecycle decision contract and later Stage 10 implementation**, with security/nonfunctional evidence later in Stage 11. R00.F **does not** retain a hidden implementation, deployment or DNS/TLS obligation and is **not yet closed** until reviewed and authorized.
- Repair §6B.12's **“remaining R00.F implementation and deployment scope”** wording with the same attribution; keep schema/runtime artifacts assigned to their actual code-bearing stages rather than claiming these were delivered.
- In §6A.16's **“Disclosed pre-existing wording mismatches”** list, delete or replace its item 3 **“Package-label split”** as a current unresolved contradiction, because the allocation is being conclusively corrected under this Owner-issued order. Retain items 1 and 2 about unrelated graphic/source ROADMAP wording without falsely declaring them to be R00.F ambiguities. Preserve the R00.E-at-the-time distinction, but stop implying that R00.F itself owns host deployment work.
- Ensure §7's remaining-work narrative, and any directly affected §6.5 cross-references, do not restore the mislabeling. Do **not** edit unrelated stages or ROADMAP content already consistent with this allocation.

**C03 — Update historical reservation now that R00.F decisions are fixed (R04 review G14).**

- §6A.16's paragraph **“Explicitly reserved, not decided here”** must truthfully state that **at the earlier R00.E edition**, contact field set, bounds, simple-form placement, abuse/rate policy, consent/privacy-notice policy, delivery outcomes, retry/deduplication and analytics scope were reserved **for R00.F and are now decided in §6B**. Provider adoption and applicable provider-specific privacy/retention terms remain later-scoped, as do R00.G's nonfunctional/lifecycle decisions and the applicable implementation stages.
- Preserve the R00.E toolchain/security assumptions, immutable artifacts, C1–C4 `UNVERIFIED` status, and all exact accepted publication literals. Do not rewrite old commit history or imply R00.E initially resolved contact design.

**C04 — Finish A1-only security cleanup (R04 review G10).**

- Delete **only** the obsolete final `SOLO-SEC-2026-001` and WordPress-advisory sentence in §6B.12 (currently around line 1743); the removed security register is historical Git evidence, not active Solo governance. Do not add a new CVE, advisory watchlist, security register or substitute workflow.
- Preserve §5A's actual architectural security boundaries: fail-closed validation, secrets, public projection, least-privilege adapters, immutable publication integrity/rollback and the truthful `NOT RUN`/`NOT APPLICABLE` rule. The **sole acceptance standard is Architect SOP §7B A1**; private GitHub vulnerability reporting remains the confidential channel.

**C05 — Directly adjacent textual consistency (no new product choice).**

- If §6B.5's visitor-state table still demands field-level errors for **every** `invalid_request`, qualify it so malformed fields get field-specific accessible errors **where safe**, while CSRF/origin refusal gets the already-accepted **generic form-level** error. This must not weaken validation or reveal sensitive security detail.
- If §6B.5's retry sentence is inconsistent with §6B.4's **same-token no second dispatch / new-token only on deliberate visitor reattempt** rule, align the wording to §6B.4. This is clarification only: **never** implement automated retries after `indeterminate` or falsely promise downstream exactly-once inbox delivery.
- Refuse any discovered contradiction that would require an architectural choice beyond the previously ratified R04 contract; do not silently widen the order.

### 4. Architect SOP cleanup — A1 is the single procedure

Edit **only** `.documents/SOP/sop-architect.md` to remove the redundant **entire §7C standalone security decision/review-protocol section** (currently three substantive paragraphs under that heading), because its requirements are already governed by **§7B A1** and Architecture §5A. No independent §7C checklist, advisory register, triage programme or shadow security standard remains. Preserve exactly the existing binding **§7B A1** quality/security, role separation, §6B Git procedure, §7/§7A review/report duties, WIP lifecycle and private disclosure safeguards. If an actual live direct reference to §7C is found outside the frozen work order, repair it only when that reference lies within a specifically authorized writable file; otherwise STOP and request a narrow Owner-approved amendment. Do not modify `.github/SECURITY.md` or Implementation SOP §7A, which already link to A1.

### 5. PR #1 — completed Owner action; no new work

The Owner explicitly confirms that **PR #1 is archived on GitHub**. The exact obsolete `docs/solo-wo-018-security-register` branch was already deleted on origin and locally by the R04 agent and the GitHub branch list currently shows only `main`. This R05 order **does not** authorize any PR or branch mutation and **does not** require an archive API capability or re-archive operation. An optional read-only metadata check may be recorded, clearly distinguishing archive visibility from REST `closed` state; it is **not** a required condition for document editing or acceptance. Do not keep PR #1 as an active R00.F obligation, re-create a historical issue, or use a stale report's G11 failure as if it were new work. A one-line acknowledgment of Owner-completed housekeeping in the final report suffices, or omit it if irrelevant.

### 6. Exact path-and-operation ledger

| Path/action | R05 permission |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Owner installs** R05. Agent may **commit/push the exact installed R05 unmodified as the first freeze step**; no alteration of Approved Scope, self-issuance, clearing or pruned work |
| `.documents/Architecture/ARCHITECTURE.md` | **Edit only** the C01–C05 specified contradictory paragraphs and their strictly necessary immediate cross-references; no 22-module contract change |
| `.documents/SOP/sop-architect.md` | **Edit only** removal of redundant §7C and any necessary within-file dangling cross-reference; preserve A1 §7B and all other procedures |
| `.documents/Reports/execution-report.md` | **Overwrite/publish one canonical report** on every run, including STOP/FAILED, when safe; no duplicate report or archive |
| `.documents/About/FOUNDATION-SOLO.md`, `.documents/SOP/sop-implementation.md`, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, `README.md`, `AGENTS.md`, `.github/**` | **Read-only**: preserve the already-correct Product Definition, Implementation SOP, ROADMAP, neutral NEXT, OUTLOOK, security policy and root files. If a necessary correction requires writing here, STOP for Owner scope rather than silently extending the ledger |
| `.documents/Security/security-issues.md` | **Must remain absent**; no recreation, no history rewrite |
| GitHub PRs, branches, release, deployment, accounts, providers, other repositories | **No mutations**. Do not touch historical PR #1; no PR creation, merge, branch deletion or DNS/provider action |
| Git staging, commits, pushes | Normal safe mechanics on verified `main` restricted to the permitted paths, under Architect SOP §6B; no force/reset/clean/history rewrite/extra checkout |
| Solo `.temp/` and active Cline-supplied task temp | Permitted *disposable* task work under Implementation SOP §5A; clean own temporary artifacts and report any unavoidable remainder |

### 7. Execution sequence and acceptance gates

**Order is mandatory:** (1) read/verify installed R05 and immutable starting anchors; (2) **freeze, push and independently read back ACTIVE before editing**; (3) perform C01–C05 document corrections and SOP §7C removal; (4) inspect exact final affected paragraphs against Product/ROADMAP/§7B/§5A/§6B, without editing read-only files; (5) run documentary structure, anchors, cross-reference, contradiction, text encoding and diff-permission checks; (6) publish the changed authorized docs and **one** current report to `origin/main`, verify the remote commit/tree and the clean local checkout, and hand off for independent review.

Every criterion below must be supported by **actual source or Git evidence**, not by a blanket agent claim:

| Gate | Required evidence for PASS |
| --- | --- |
| G01 Owner authority | Exact installed R05 approved scope, separate matching handoff, verified one Solo root, `main`/`origin`, four WIP files and known worktree |
| G02 Freeze sequence | **Issuance-freeze committed/pushed/read back before** any Architecture/SOP editing; frozen order blob unchanged; no repeat of R04 sequencing deviation |
| G03 Correct public endpoint | §6A.12 and §6B.2/§6B.6 distinguish public form action + non-secret tokens from private provider/recipient/credentials; no invented authoring field or provider |
| G04 Stage allocation | §6.5, §6A.16 and §6B.12 consistently assign R00.F to documentary decisions; implementation to R08/R09, lifecycle policy to R00.G and implementation/release to their later packages; stale package-label split removed |
| G05 Historical reservation | R00.E's original unresolved-at-the-time language now explicitly points to ratified R00.F §6B decisions; future provider/lifecycle matters still deferred correctly |
| G06 A1 only | Removed standalone Architect SOP §7C; A1 §7B, Architecture §5A and private reporting preserved; no live reference to §7C or removed register outside frozen historical ACTIVE |
| G07 Advisory clutter eliminated | No active Architecture text naming `SOLO-SEC-2026-001`/WordPress or presenting a security register as authority; historical Git evidence unmodified |
| G08 Contract preservation | Exactly **22 distinct registered `1.0.0` module IDs** with the original 21 and `interaction.contact` row **unchanged**; no changes to EFS, routing, location, publication, contact numeric/privacy/abuse/replay semantics or A1 security thresholds |
| G09 Form semantics consistency | `invalid_request` states and safe retry wording do not conflict with already ratified §6B.4/§6B.5; future F01–F17 all remain `NOT RUN`, C1–C4 `UNVERIFIED` |
| G10 Allowed-path Git purity | Only ACTIVE, Architecture, Architect SOP and canonical report changed by R05; no extra WIP, no security register, no PR/branch mutations, no deployment |
| G11 Report and checkout | Single overwritten canonical report with gate evidence, real revisions, remote read-back, one worktree, clean local tree and owned task-temp cleanup; no self-acceptance |

**No false runtime success:** This repository still has no implemented application, schema, tests, CI, mail provider, contact endpoint or deployment. All executable `F01`–`F17` cases are `NOT RUN`; `C1`–`C4` remain `UNVERIFIED`. A documentation check may PASS; an unrun security, browser or delivery test may not.

**Stop conditions:** Unverifiable Owner issuance, an unexpected changed source anchor, unknown/unrelated dirty files, wrong root or remote, unsuccessful freeze, request for a new product choice, need to edit outside this ledger, changed original module contracts, privacy/credential exposure, or unsafe Git operation. Preserve all Owner-owned data and report the exact blocker. Never fake PR archive verification or restore the removed historical register.

### 8. Final handback and review boundary

Overwrite `.documents/Reports/execution-report.md` with status `COMPLETE`, `STOPPED` or `FAILED` as warranted; record exact R05 identity/bytes/SHA-256, starting and final Git commits/tree, source diff, G01–G11 results with direct evidence, cross-reference/count checks, temp cleanup, GitHub read-back, and all limitations. The report must **not** embed its own report-commit SHA; provide it in the agent's chat handback. State that PR #1's archival was **Owner-confirmed**, not agent-executed, without resurfacing it as a live blocker.

The Implementation Agent may **not** declare `ACCEPT`, close R00.F, prune ROADMAP, clear ACTIVE, select or start R00.G, merge, release or deploy. The Chief Architect independently reviews the exact GitHub report and diff and issues `ACCEPT / REVISE / STOP`; only after `ACCEPT` and **separate express Owner-authorized closure/reconciliation** may the WIP slots be transitioned under Architect SOP §8.
