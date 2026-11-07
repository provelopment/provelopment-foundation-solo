# Foundation Solo — Active Work Order

## Approved Scope — SOLO-WO-021 / R03

**Issuance marker:** `OWNER-ISSUED` **only upon the Owner's deliberate installation of this entire exact R03 file into** `.documents/WIP/workorder-active.md` **and delivery of a separate matching Owner activation message**. Downloading, reviewing, or receiving this file from the Architect alone does **not** authorize execution.

| Field | Frozen scope |
| --- | --- |
| Work order | `SOLO-WO-021` |
| Edition | `R03` — complete replacement proposal for the previously reported, locally installed but uncommitted `R02`; it is **not** a copy or reconstruction of R02 |
| Package | `R00.F` — contact and integrations, **architecture decisions/documentation only** |
| Repository | `https://github.com/provelopment/provelopment-foundation-solo` (and no other repository) |
| Working root | The Owner's **single existing** local `00.foundation-solo` checkout, established by actual Git preflight; do not assume the Windows path exists in the agent's runtime |
| Target | `main`, tracking the sole remote `origin` pointing to this repository |
| Public reference baseline | `79b8f305d07a28aae295cea39db51d82afcc1e99` (the last observed report-only `main` commit on 2026-11-07); use the immutable blob anchors in §2 for source-of-truth preflight |
| Handoff identifier | Owner message expressly saying **“Execute SOLO-WO-021 R03: read and run .documents/WIP/workorder-active.md”** after the Owner installs this exact R03 edition |
| Scope type | Finite Stage 00 codification; no application implementation, dependency installation or provider operation |
| Ownership | Owner authorizes issuance; Implementation Agent executes; independent Chief Architect reviews; Owner separately authorizes closure and later R00.G |

**Supersession safeguard.** The most recent published execution report describes an **uncommitted 68,131-byte `SOLO-WO-021 R02`** in the Owner-local checkout with SHA-256 `B88673C79D36A0090472306428BCAC776C0237D3065594A3F1F337E856A547A0`. That original R02 is **not accessible in the published Git repository or to the Architect in this session**. This R03 document therefore makes no claim to preserve R02 byte-for-byte or to carry forward undisclosed R02 decisions. **Only the Owner may explicitly replace the locally installed R02 with R03.** The Implementation Agent must not erase, migrate, or rewrite the old local edition on its own. If the installed ACTIVE is R02, if both editions are present, or if the Owner's matching R03 handoff has not been delivered, **STOP** before edits. R03 is one replacement order, not a second concurrent order.

### 1. Exact objective and boundaries

Record a coherent, first-release-ready **R00.F contact and small integrations contract** in the existing Solo architecture authority, leaving all runtime delivery and provider adoption for later authorized stages. Preserve the existing product scope: a safe authored `mailto:` action **and** a simple contact form are included; advanced forms are excluded. The contract must be precise enough that a later code-bearing order can implement it without making unapproved product decisions, but no code, executable schema, provider account or deployment is permitted here.

**Deliverables:** (a) a concise alternatives-and-decision rationale inside the existing Architecture Charter; (b) precise normative contact/integration rules, interface and trust-boundary mapping, field/response/privacy/abuse semantics and refusal cases; (c) any strictly necessary alignment of the existing Product Definition; (d) the authorized NEXT issuance transition; (e) the one canonical published execution report and independently reviewable source diff. Do not create another decision register, specification document, task queue, report or WIP file.

This is the **only** issued work order. `R00.G` remains the next **separate** architecture package after independent acceptance and Owner-authorized closure. Stage 01 or later implementation is not authorized.

### 2. Mandatory preflight and pinned evidence

Before performing any non-report edit, read `README.md`, `AGENTS.md`, the Product Definition, Architecture Charter, both SOPs, all four WIP files, the current published execution report and the security register **in the order required by AGENTS.md**. Read this entire Approved Scope. Verify the real filesystem root, `git rev-parse --show-toplevel`, branch, `HEAD`, upstream, worktree inventory, clean/known changes, sole `origin`, exact installed R03 edition, separate matching Owner handoff, and the four-file WIP inventory. Do not create a second checkout/worktree, nested repository or shadow copy, including under temporary folders.

The public `main` baseline at preparation time was `79b8f305d07a28aae295cea39db51d82afcc1e99`. This is a reference point, not authorization to reset or rewrite Git. **Immutable content anchors** observed at that baseline:

| Path | Expected Git blob SHA-1 |
| --- | --- |
| `.documents/Architecture/ARCHITECTURE.md` | `0f26366e542545201d40cb03d8be1d6e2ed8ff62` |
| `.documents/About/FOUNDATION-SOLO.md` | `51d9625826956bcd45b5977dee7e48c2323ac1d8` |
| `.documents/WIP/plan-next.md` | `e483a281718fc9b74a815212bffe35c395a1dbd2` |
| `.documents/WIP/plan-roadmap.md` | `50ff6d3037f16576b9b455f17169408acd4fda38` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/WIP/workorder-active.md` as committed before Owner installation | `ea5f20b736483c2e102456be1cea9104059fc81a` |
| `.documents/Security/security-issues.md` | `5b50e1f03f2165a4f82f5c5a5f89cc4ffe330634` |

**Stop on unexpected drift.** After the Owner has installed R03, one understandable pre-existing ` M .documents/WIP/workorder-active.md` is allowed as the intentional order installation; no other unexplained changed/untracked path may be silently discarded. If the published HEAD advanced *only* by independently verified canonical execution-report commits with all the listed content anchors unchanged, that movement alone is not a reason to reset or alter the scope; record the exact revision. Any other source, WIP, authority, branch, remote or worktree mismatch requires STOP and Owner/Architect reconciliation before work. Never infer a local Git fact solely from the report.

**Freeze only the Owner-installed edition.** Upon both verified authorities and successful preflight, commit the **unmodified** R03 ACTIVE file according to Architect SOP §6B before substantive document changes, then verify the remote commit. If the frozen installed file differs from this edition, or its identity cannot be proven, STOP; do not silently 'repair' it. Record its actual byte length and SHA-256 in the execution report. Do not place a report's own resulting commit SHA in its content.

### 3. Decisions and normative outcome to codify

The following constraints are the **Owner-reviewable R03 decision envelope**, not permission for the agent to invent other features. They become the accepted scope only with the Owner's express R03 installation and handoff. If a specification needs a major new policy choice not fixed below or already canonically decided, state the competing options and **STOP for Owner decision** rather than quietly selecting one.

**A. Two distinct contact actions.** A public `mailto:` is an authored, visibly public destination, governed by the existing safe-link rules; it opens the visitor's mail client and **must not** be described as server delivery, secret, or a guaranteed submission. A contact form is a separate server-side submission use case and must not expose an internal recipient address or provider configuration to HTML, JS, JSON, public URL parameters or diagnostics. Where an installation has no configured delivery adapter/recipient, the form is clearly disabled or unavailable, never presented as successfully operational; the safe public `mailto:` remains independently usable if configured.

**B. Minimal simple form.** Specify one bounded form with `name`, `email`, `message` and a privacy-notice acknowledgement only if its legal/operational basis and effect are explicitly defined; no additional personally identifying field is collected by default. The finalized contract must give exact field keys, types, normalization, required/optional status, scalar/byte limits, server-side validation, safe output handling and failure behavior for malformed payloads, unknown fields, oversized bodies, automated submissions and repeated submissions. Name, email and message are ordinary untrusted data. Do not interpret names or message bodies as markup or execute any authored input. Never make consent a preselected checkbox or a blanket legal-compliance claim. If the precise form key/limit/acknowledgement choices are not established by accepted canonical material or the Owner's R03 decisions, identify them as decisions requiring approval rather than as facts or implemented schemas.

**C. Inbound request and application port.** Define a typed request envelope, bounded validation and an application-level `SubmitContactInquiry` (or equivalently named) use case. The application owns intent and outcomes; a delivery port accepts a validated, server-side message and a recipient identifier resolved from private installation configuration. Provider SDKs, email transport, outbound HTTP, credential management, framework request objects, storage and environment access stay entirely in adapters/composition. Specify origin/CSRF policy, replay/idempotency policy where justified, request-size limits and safe response enumeration. No path, hostname, IP geolocation, locale or EFS location property may be used to infer a recipient or grant authority. Reuse the configured one-Entity Site/Page context only through approved typed references.

**D. Truthful outcomes.** Distinguish at least invalid request, temporarily unavailable, rate/abuse refusal, provider accepted-for-delivery, downstream rejection, and indeterminate result. An acknowledgement **may** mean accepted for delivery, not necessarily delivered to a mailbox or read. Avoid response differences that expose internal recipients or provider details. Specify accessible success/error states, keyboard/assistive-technology announcements and a safe retry policy which does not create duplicate messages after an indeterminate result. A fake adapter or no-provider mode never reports live delivery.

**E. Security and abuse prevention.** Fail closed at the request boundary. Require server-side validation regardless of client hints, constrained body size, bounded rate/abuse controls, no trust in client-supplied origin/forwarded headers without a defined trusted-proxy boundary, no execution of content, output escaping, least privilege for outbound delivery, and explicit handling of adapter outage. Prefer proportionate first-party abuse controls; do not mandate third-party CAPTCHA, tracking, a visitor account, or a persistent global identity system. Set measurable refusal criteria and later code-bearing verification cases. Where exact rate thresholds, retention periods or security response service levels need Owner policy, identify the Owner decision and keep the code-bearing gate blocked rather than fabricating a universal value.

**F. Privacy and data lifecycle.** Keep delivery credentials, internal recipient mapping and configuration outside authored/public content. Define the minimum transient processing required, limits on logs and diagnostics (no raw message body, visitor email or credentials in ordinary logs), handling of delivery failure, and whether messages are retained by Solo at all. **Recommendation for this edition:** Solo's core does not persist contact messages by default; the chosen downstream provider may have its own retention policy to be documented when adopted. No analytics or form telemetry may silently collect the submitted message. Make data-subject notices and any retention/deletion commitments explicit Owner-approved policy choices at the later provider boundary, not assumed worldwide legal compliance.

**G. Public links and analytics.** Preserve the accepted typed safe-link grammar for authored `mailto:`, internal/external booking and maps/directions URLs. First release may support **static external booking/directions links**, not a booking engine, account system, transactions, live inventory or geolocation service. **Default recommendation:** no analytics is required in the first release; any later optional privacy-respecting analytics adapter must be separately approved, independently disable-able, and never needed for public publication. If Owner approval of analytics inclusion is not evidenced, codify the no-analytics default and keep provider adoption out of this order.

**H. Contact presentation and module boundary.** Clearly specify the public simple-form entrypoint and permitted presentation arrangement without silently changing the accepted **exactly twenty-one** initial `1.0.0` authored JSON module registry in Architecture §6.2E.2. Historical R02 reporting mentioned `interaction.contact` as a possible closed module contract, but that uncommitted decision is not independently visible; do **not** claim a 22nd module has already been accepted or installed. Present the two credible placements—(1) a distinct system-owned contact use case surfaced through an expressly configured Page/action boundary, or (2) a versioned `interaction.contact` authored module with an explicit subsequent registry amendment—and recommend the simplest contract consistent with the existing public Page and module rules. A **new 22nd module or revision of O05** is a material Owner decision: STOP for explicit Owner approval if the chosen placement requires changing the original registry. Do not create a concealed module or a general form builder.

**I. Independent accepted contracts.** Preserve all R00.A–R00.E (`D01`–`D12`, `O01`–`O12`, `E01`–`E10`) requirements, conditional pins, the `C1`–`C4` **UNVERIFIED** proof status and §5/§5A architectural security boundaries. No framework, hosting provider, DNS, email vendor, CRM, analytics account or external service is selected by implication. The WordPress advisory `SOLO-SEC-2026-001` remains a documented external advisory assessed as not applicable to the code-free examined revision; do not label Solo patched or immune.

### 4. Exact permanent path and permission ledger

| Target/action | Permission in R03 |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Owner only** installs/replaces the frozen Approved Scope. **Agent** may commit/push the already-installed exact R03 edition as the authorized issuance-freeze step and write a short separate Execution Evidence area **only if its addition does not alter this Approved Scope**; no agent replacement or self-issuance |
| `.documents/Architecture/ARCHITECTURE.md` | **Agent may edit** only the R00.F contact/integration contract and necessary internal cross-references, preserving every other accepted literal |
| `.documents/About/FOUNDATION-SOLO.md` | **Agent may edit narrowly** only if needed to remove a direct inconsistency with newly accepted R00.F scope; otherwise unchanged |
| `.documents/WIP/plan-next.md` | **Agent may clear** only the now-issued `R00.F` candidate during the Owner-authorized issuance transition; retain the file/heading, and do **not** promote R00.G before the separate acceptance-and-closure order |
| `.documents/Reports/execution-report.md` | **Agent must overwrite/publish** the one canonical public-safe report, including STOP/READ-ONLY runs where Git safety permits |
| `.documents/WIP/plan-roadmap.md`, `plan-outlook.md`, `.documents/Security/security-issues.md` | **Read-only** during R03: leave R00.F and R00.G in ROADMAP until independently accepted and separately closed; do not create a fifth WIP file or modify the security register |
| `README.md`, `AGENTS.md`, both SOP files, `.github/**`, code, dependency/lock files, schemas, assets, CI, deployments, other repos | **Read-only**; no new file, provider setup, code implementation, documentation hierarchy or policy changes |
| Historical PR #1 / branch `docs/solo-wo-018-security-register` | **Read-only evidence only.** No PR merge/reopen/archive or local/remote branch deletion in this replacement R03; those housekeeping operations are intentionally deferred because they are unnecessary to R00.F codification and require independently verifiable exact authority |
| Git stage/commit/push | **Authorized only** for the paths explicitly writable above and only after authority, root, branch, clean/known-tree and remote checks. Follow *sole* version-control procedure, Architect SOP §6B; no force, history rewrite, destructive reset/clean, or unrelated branch |
| `.temp/` and active Cline task temp | Permitted **disposable** workspaces per SOP; no second checkout, and clear positively identified task-owned material before handback; disclose unknown or unremovable material |
| Secret access, external mail provider, API paid service, deployment, merge, release, live testing or third-party network writes | **Not authorized** |

**Scope prioritization:** this work order is intentionally narrower than the unpublished R02's described PR-housekeeping scope. The Owner's installation of R03 deliberately supersedes that older authority, not both editions running together. A cleaner current architecture record is more important than changing historical GitHub PR artifacts.

### 5. Required execution sequence

1. **Authority and integrity:** Confirm entire installed R03 with Owner handoff, single checkout and sole remote, immutable content anchors, clean/known tree, and four WIP files; STOP on conflict. Record actual installed R03 bytes/SHA-256.
2. **Freeze the issued order:** Commit/push/read back the unchanged Owner-installed R03 Approved Scope exactly under the authority and Git rules. Do not mutate the order to make the check pass.
3. **Compare existing architecture:** Locate already accepted contact fields, safe-link literals, property privacy, the reserved delivery port, exact module catalogue and security §5A. Write the alternatives, compatibility findings and no-go cases in the existing Charter only; do not invent a parallel decision doc.
4. **Codify settled R00.F rules:** Add one clearly bounded section with typed contact/integration contract, outcomes, privacy/abuse decisions, safe external-link policy and exact future proof matrix. If material Owner choices (especially contact presentation/module registration or exact numeric policy) remain undetermined, STOP with the smallest targeted decision request rather than claiming complete codification.
5. **Align minimal Product text:** Make only necessary changes in the permitted existing Product Definition, if any. Preserve one Entity, simple form inclusion and the advanced-form exclusion.
6. **Issue-transition cleanup:** Clear the R00.F NEXT candidate only after the issuance-freeze has succeeded; leave ROADMAP and OUTLOOK untouched. Never mark the package accepted or closed.
7. **Verify:** Perform documentation structural, reference, encoding, contradiction, permission, Git-integrity and security-lifecycle checks. Report each required gate individually, with direct evidence, not invented test commands.
8. **Report and publish:** Overwrite `.documents/Reports/execution-report.md`, safely commit and push only permitted files, verify actual GitHub results, report final source/content SHA and report-commit SHA **in chat** (not self-referentially inside the report), clean task-owned temp and hand off for independent Chief Architect review.

**No self-acceptance.** The agent may report `COMPLETE` if all applicable R03 gates pass; the Chief Architect alone returns `ACCEPT / REVISE / STOP` after GitHub-first review. The Owner separately authorizes R00.F administrative closure, ROADMAP pruning and a future R00.G order.

### 6. Acceptance and evidence gates

The agent must include a table of gates and actual evidence in the canonical report. For a **documentation-only** work order, compilation, browser execution, live form delivery, accessibility testing against a running app, dependency audit and penetration tests are **NOT APPLICABLE**, not PASS. Use `PASS`, `FAIL`, `UNVERIFIED`, `NOT RUN`, or `NOT APPLICABLE` truthfully.

| Gate | Required evidence for PASS |
| --- | --- |
| G01 — Owner authority | Installed R03 exact Approved Scope, separate matching Owner handoff, correctly identified real repository; no leftover active R02 |
| G02 — frozen issuance | Published Git commit/read-back with Owner-installed **unchanged** Approved Scope, correct `main`/`origin`, no second worktree |
| G03 — source fidelity | Canonical documents match pinned anchors before changes; zero unauthorized contract changes, no inherited Foundation assumptions |
| G04 — simple-form boundary | Precise first-release simple form vs safe `mailto:` vs private recipient; no advanced form, account, CRM, payment, attachment or booking engine |
| G05 — public/private and typed ports | Provider-neutral application use case, inbound validation, outbound port, private config, no SDK or secret in core or client output |
| G06 — request and validation | Field/request bounds, normalizations and fail-closed refusal cases explicit; unresolved policy decisions visibly escalated, never guessed |
| G07 — abuse/privacy | Anti-abuse and CSRF/origin principles, log minimization, no-provider behavior, retention/consent decision boundaries all reviewable |
| G08 — truthful outcomes | Distinct accepted/rejected/indeterminate semantics and accessible status/error handling, no fake-delivery claims |
| G09 — integrations | Safe static directions/booking links, bounded analytics/no-provider scope and no forbidden provider coupling |
| G10 — module/registry consistency | Existing exactly-21 module contract preserved unless separately approved; no hidden `interaction.contact` addition or unapproved O05 revision |
| G11 — future executable proofs | Traceable positive/negative/privacy/accessibility/adapter/refusal matrix with later-stage owners; no assertions of tests not run |
| G12 — security lifecycle | §5A/§7C/§7A cross-check, register applicability and recheck triggers preserved; no false patch or security certification |
| G13 — WIP purity | Exactly four WIP files; NEXT transition only within scope; ROADMAP and OUTLOOK remain pending/unchanged; no extra report/archive |
| G14 — publication and changed-path purity | Only allowed paths staged/committed/pushed, correct remote exact revisions and independent remote read-back; no merge/deploy |
| G15 — canonical report and cleanup | Current report overwritten with true evidence; no self-commit SHA embedded; one checkout; known task-owned temp cleaned or honestly disclosed |
| G16 — historical PR preservation | PR #1 and old branch inspected only; no archive, branch deletion or other unapproved GitHub operation |

If any gate materially applicable to the *authorized work* is `FAIL`/`UNVERIFIED`, do not claim fully accepted R00.F: report a bounded `STOPPED`/partial outcome and the smallest necessary Owner decision. A documented later-stage implementation obligation can legitimately be `NOT APPLICABLE` to this decisions-only run, but an unresolved current design decision is not a pass.

### 7. Non-negotiable contracts and stop conditions

Preserve exactly one Entity per Installation and the Entity → Facet → Site → Page → localized representation hierarchy; stable IDs independent of route, locale and hostname; full region-qualified locales; public case-insensitive canonical routes by default with allowed deliberate sensitivity; exact-case unlisted routes excluded from discovery (`unlisted` is not private); authored location independent from routing; safe typed properties and graphic cascades; authored-source provenance; the conditional Rust/Hexagonal choices with inward-only dependency boundaries; and security by design with immutable atomic publication/rollback requirements. Do not select a hosting or mail provider and do not treat the unbuilt system as implemented.

**STOP and preserve state** on missing or contradictory issuance, R02/R03 ambiguity, absent matching handoff, changed canonical anchors, unknown dirty files, wrong remote/branch, missing required field/registry Owner decision, unauthorized PR/branch action, privacy leak, technical requirement conflict, scope expansion, unprovable Git/working-tree safety or any operation that would destroy unknown/Owner-owned material. Publish a truthful STOP report under the standing report-only permission where safe. No guessed substitute approval, work order, data deletion or new authority file.

### 8. Final handback required

State the exact installed `SOLO-WO-021 R03` revision and checksum; repository root, HEAD and current remote; files changed and concise contract summary; G01–G16 outcome table with direct evidence and all limitations; whether a material Owner decision remains; links to actual content/report commits and the current published report; the GitHub-side PR/branch preservation result; one-checkout status and temporary cleanup. Do not self-approve, close R00.F, start R00.G, merge, release or deploy. The next Chief Architect action is independent GitHub-first review against the report at the exact revision.
