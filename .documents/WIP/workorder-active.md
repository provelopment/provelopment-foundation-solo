# Foundation Solo — Active Work Order

## Approved Scope — SOLO-WO-021 / R06

**Issuance rule.** This is the Chief Architect's **one proposed corrective work order**, following independent `ACCEPT` of `SOLO-WO-021 R05` and the Owner's request for a further final R00.F intent audit before closure. It is **not OWNER-ISSUED** until the Owner deliberately replaces the entire `.documents/WIP/workorder-active.md` in the **existing single Solo checkout** with this exact file **and** sends a separate matching Owner activation naming `SOLO-WO-021 R06`. The Implementation Agent must not install, amend, self-issue, or execute an order without both conditions. There is never a parallel R05/R06 order.

| Key | Frozen value |
| --- | --- |
| Order | `SOLO-WO-021 R06` |
| Package | `R00.F` — **final contact-contract disambiguation**, documentation only |
| Repository | `https://github.com/provelopment/provelopment-foundation-solo` |
| Sole working location | Owner's existing verified Foundation Solo checkout, branch `main`, sole `origin` above |
| Independently reviewed starting `main` | `358592862666ee42770a9c0e476fd227dd203393` (2026-11-07) |
| Matching Owner activation | “Execute `SOLO-WO-021 R06`: read and run `.documents/WIP/workorder-active.md`.” |
| Target outcome | R00.F **ready for independent closure review**; **no closure performed** |

**Previous authority and scope.** R05's corrections are independently accepted; they are **not reopened**. This R06 order responds solely to three additional implementation-facing ambiguities found by comparing this conversation's full R00.F decisions with the canonical published contract. The first-release catalogue is **exactly 22** `1.0.0` modules, including the registered `interaction.contact` leaf. Its three visitor data fields, localized labels, privacy notice and seven feedback texts, one private recipient per Installation, truthful seven application/four transport outcomes, field limits, dual 16-KiB ceilings, 3-per-15-minute and 60-per-hour rate limits, 24-hour attempt record, no core message persistence, static booking/directions links, analytics off, and advanced-form exclusions are all settled and remain binding. `mailto:` is a separate intentionally public client-mail action. No new module or additional product capability is authorized.

### 1. Findings and precise authorized decisions

**FINDING A — browser-enhancement contradiction.** Architecture §6A.5 defines exactly five browser enhancement kinds (`tabs`, `accordion`, `modal`, `carousel`, `motion`), while §6A.12's general sentence demands a `BrowserEnhancementV1` envelope for *every interactive module*, which could be read to include `interaction.contact`, notwithstanding its own row's express **no browser enhancement payload** rule. **Binding resolution:** `interaction.contact` is a **server-rendered, standards-based HTML form**, not a `BrowserEnhancementV1` client-controller module. The existing five browser-enhanced modules remain the only ones requiring `data-solo-envelope`, DTO parity and client-runtime decoder fixtures. Contact uses server-rendered semantic markup, an approved public POST action and progressively enhanced/no-JavaScript submission. It **must not** grow a sixth DTO `kind`, a special browser controller, arbitrary authored script, a new JSON node property, or an alternative Page-enablement mechanism. Apply generic closed-module validation, safe rendering and realistic form/keyboard/no-JS tests to contact. Its `interaction.` prefix does not confer browser-enhancement status.

**FINDING B — no-JavaScript submission framing left conditional.** Architecture §6B.3 says "where" JSON or `application/x-www-form-urlencoded` is accepted but does not declare a mandatory first-release wire format, and its three-field closed payload can be misread as rejecting the already required server-issued CSRF/attempt controls on a native form POST. **Binding minimal first-release resolution:** The required browser-facing contact submission is a standard HTML `method="post"` form using **`application/x-www-form-urlencoded` UTF-8** so it works with JavaScript disabled. Accept that media type with an absent charset parameter or `charset=UTF-8` (case-insensitive charset label); reject unsupported encodings, `multipart/form-data`, `application/json`, and other media types **in the first release**. The earlier conditional JSON reference was decoding guidance, **not** approval of a second endpoint or a general JSON inquiry API; any later JSON support requires explicit separate Owner authority. A request has exactly **three visitor data keys** (`name`, `email`, `message`) and exactly **two server-owned, non-personal form-control keys** (`csrfToken`, `attemptToken`). These five wire keys comprise one **closed transport envelope** even though native form encoding puts them in the same POST body; decoding separates the two controls before constructing the closed three-value `ValidatedInquiry`. The control names and values are never authored module props or additional visitor fields. Reject unknown/duplicate keys, missing/malformed control values, ambiguous percent escapes, invalid UTF-8 and any body over the ratified **16 KiB raw and 16 KiB decoded** limits; the bounds apply to the complete wire submission including control keys. Preserve server-authoritative validation, origin/CSRF verification, and the existing outcome enumeration. No honeypot field, tracker, JSON fallback, attachment, alternative custom action or new API is silently introduced.

**FINDING C — expiry versus permanent single-dispatch rule.** Architecture §6B.4 requires a given server-issued attempt token to authorize **at most one outbound dispatch** but allows its non-content deduplication record to expire after 24 hours without explicitly governing an old token presented after that expiry. **Binding resolution:** An attempt token expires **no later than 24 hours after issue** and can **never** become valid again; server-side state/verification must reject a missing, expired, invalid or unverifiable token without outbound dispatch. The atomic non-content deduplication record remains effective for the token's full validity and may expire at the defined 24-hour boundary. After expiry or lost deduplication state the same token must **not** create fresh dispatch authority. Such refusal maps to the existing generic `invalid_request` form-level outcome (no new visitor outcome or provider detail). A *fresh* attempt requires a deliberate new visitor action, a newly issued token, and the existing duplicate-warning policy where appropriate. Same-token concurrent/multi-instance claims still dispatch at most once; an `indeterminate` result never triggers blind or automatic retry. This codifies the previously ratified policy, not downstream exactly-once inbox delivery.

**No additional Owner decisions are opened.** These precise narrow resolutions are presented for Owner approval **through installation of this R06 edition**; they are not instructions for the agent to choose among alternatives. If any resolution conflicts with another accepted contract beyond the named wording, the agent must STOP and report the exact conflict, not invent a fourth design change.

### 2. Required canonical documentation changes

Modify **only** `.documents/Architecture/ARCHITECTURE.md` within the following tightly bounded locations:

1. **§6A.5 / §6A.12 browser boundary**: disambiguate the general `Each interactive module` statement so its enhanced-DOM/DTO requirements apply to **only the five enumerated browser-enhancement modules**, not `interaction.contact`. Retain the contact row's no-`BrowserEnhancementV1` rule and the existing five kinds, browser safety/parity requirements, 22-ID catalogue, and unchanged contract rows. If a nearby generic assertion would otherwise require a false `data-solo-envelope` on contact, correct only that assertion. No schema or runtime files.
2. **§6B.3, §6B.8 and applicable form matrix evidence**: fix the first-release POST media type, exact separation of three visitor keys from two server-owned control keys, and the closed native form transport, without changing the visitor field types, limits, normalization, public endpoint/secret boundary, module `props`, or user-visible outcomes. Add or refine only the **appropriate `F01`/`F03`/`F16` future tests** to require exact media-type/charset, hidden control handling, no-JS real POST, and unknown/repeated key refusal. All remain `NOT RUN`.
3. **§6B.4–§6B.5, §6B.11 and `F13`**: make token expiry and loss-of-state refusal explicit, and reflect its safe `invalid_request` mapping and 24-hour expiry edge in the future test. Do **not** introduce a new storage provider, token system, visitor data field, eighth outcome, longer retention, automatic resend or a weaker at-most-once guarantee.
4. **Cross-consistency only**: make the smallest direct references needed inside the same Architecture file; do not reword accepted decisions for style. Recheck §6.2E.2, §6A.5, §6A.12, §6B and §7 for contradictory statements and preservation of the 22-module first profile. If another file genuinely needs substantive change, STOP and request separate Owner scope rather than extending this order.

The Architecture Charter remains the **single permanent normative home** for these contacts. Do not create a contact supplement, decision log, issue register, archive or fifth WIP file. No live permanent document should describe this documentary correction as executable feature delivery.

### 3. Security-work exclusion and other immutable boundaries

**Security remediation, the earlier security-governance cleanup, PR #1 and its archive, and the deleted historical advisory register are explicitly OUT OF SCOPE.** No work in Architecture §5A, either SOP's security clauses, `.github/SECURITY.md`, GitHub PR/branch history or any security register is authorized. This order does not ask for a security audit, vulnerability search, security fix or new security procedure. Merely preserve the existing accepted publication, private-data and safe-contact invariants where they are directly relevant to the three contact decisions above.

**No code, schemas, registry executables, dependencies, tests run, provider activation, mail accounts, analytics, CI, deployment, DNS/TLS, release, R00.G changes, stage reassignment, or roadmap pruning.** Preserve the single-Entity EFS hierarchy, source authority, exact routing/locale/visibility, location independence, properties/graphics, inward-only Ports and Adapters, 21 preceding module rows, and published `interaction.contact` module props verbatim. No additional form fields, advanced form builder, CMS/CRM, booking engine, account identity system, mandatory analytics, or new browser DTO kind.

### 4. Preflight, frozen order and Git safety

1. Read root `README.md`, `AGENTS.md`, Product Definition, Architecture, both SOPs, all four WIP files, current canonical execution report, and applicable root-mandated materials in their prescribed order. Verify the real Solo root, `main` and tracking upstream, correct sole `origin`, **one** worktree/checkout, no stashes or unknown user-owned changes, HEAD, exact WIP inventory, and the Owner's **separate matching R06 activation**. Expected initial modification is **only** the Owner-installed replacement `.documents/WIP/workorder-active.md`; never discard unknown content.
2. Verify the published baseline `main` is `358592862666ee42770a9c0e476fd227dd203393` or has advanced **only** by independently verified report-only commits leaving all following anchors unchanged. The previous frozen ACTIVE and canonical source blob IDs at that reference are:

| Exact path | Expected blob SHA-1 |
| --- | --- |
| `.documents/WIP/workorder-active.md` **before Owner installs R06** | `1e0ce769b900458a2833eff273f20253101ed7c4` |
| `.documents/Architecture/ARCHITECTURE.md` | `aa3bdb5fe6121e66a3268fb3c9425a8f37a54b02` |
| `.documents/Reports/execution-report.md` | `e6fac4f47470f9e17801a1d083a997603d181206` |
| `.documents/About/FOUNDATION-SOLO.md` | `50a58123b2ec5797635eec87cdd788ee28e2abf7` |
| `.documents/SOP/sop-architect.md` | `0396251245180b40ee896b50a3456b92835f9cb1` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/plan-roadmap.md` | `613d3fcb979c4e3687438136f9c286a54fd6a07c` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `README.md` | `ed492cb689276a2a8b2112438a7f7cae941af233` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |

3. STOP if there is substantive drift, a missing matching handoff, an unexpected dirty file, multiple checkout/worktrees, conflicting authority, or any unsupported write. No history rewrite, reset, clean, force push, new branch, new clone or PR.
4. **Before editing any other file**, hash the exact Owner-installed R06 (bytes, LF/UTF-8 and SHA-256), then make the **first commit and push exclusively `.documents/WIP/workorder-active.md`**, unchanged. Verify the frozen blob and remote read-back. Only then may the agent edit Architecture. Follow the sole Git version-control procedure in Architect SOP §6B.

### 5. Exact path-and-operation ledger

| Path | Authority |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs exact R06; agent may only **freeze/commit/push it verbatim first**, never edit scope or clear ACTIVE |
| `.documents/Architecture/ARCHITECTURE.md` | **Edit** the three enumerated contact ambiguities and necessary immediate references only |
| `.documents/Reports/execution-report.md` | **Overwrite, commit/push, read back** the one canonical factual report for this run |
| All other files including Product, both SOPs, ROADMAP, NEXT, OUTLOOK, root docs, `.github/**` and removed security register | **Read-only / no recreation** |
| PRs, branches, releases, providers, deployments and other repositories | **No mutations** |
| Solo `.temp/` and the active task's Cline-provided temporary workspace | Disposable scratch only, with careful cleanup of task-owned material |

### 6. Execution and independent acceptance gates

**Sequence:** verify authority and anchors → freeze and remotely read back exact R06 **before any other source edit** → edit Architecture only → check every cross-contract consistency and final diff → publish corrected documentation and overwrite the single report → read back `origin/main` and verify final status/cleanup → hand off to independent Architect review. Never create a PR.

| Gate | Evidence required for PASS |
| --- | --- |
| G01 — Owner authority | Exact R06 installed, distinct matching Owner handoff, verified root/`main`/sole `origin`, one worktree and known initial state |
| G02 — Freeze | Byte-identical R06 alone committed and pushed **before** editing Architecture, independently read back |
| G03 — Browser contract | Exactly five `BrowserEnhancementV1` kinds remain; `interaction.contact` has **no** DTO/envelope, but does have server-rendered, keyboard and no-JS form requirements; all other existing enhanced-module obligations retained |
| G04 — Transport contract | One obligatory UTF-8 URL-encoded native POST media type, five **closed** transport keys with exactly three visitor data fields + two server-owned controls, strict duplicate/unknown/malformed/oversize refusal; no JSON/multipart default or added form builder |
| G05 — Replay expiry | Token expires no later than 24h; expired/missing state fails without dispatch; no same-token resend even after record expiry; `F13` and relevant documentation make the boundary verifiable |
| G06 — Preservation | The original 21 module rows and the `interaction.contact` registry row unchanged, 22 IDs and seven outcomes remain, all prior field/rate/retention/analytics/port/site decisions intact |
| G07 — Future proof cases | Relevant `F01`/`F03`/`F13`/`F16` updated for decoding, no-JS POST, token expiry and multi-instance replay; all `F01`–`F17` still `NOT RUN`; `C1`–`C4` remain `UNVERIFIED` |
| G08 — No unrelated changes | Only frozen ACTIVE, Architecture, and canonical report changed; security work, PR #1, provider and R00.G untouched |
| G09 — WIP governance | Exactly four retained WIP files; ROADMAP still lists `R00.F` and `R00.G`; NEXT neutral; ACTIVE remains R06; no unapproved archive or extra document |
| G10 — Git/report hygiene | Exact commits, changed-path diff, current report and remote read-back, one worktree and clean checkout, owned temporary files cleaned or an explicit failure |

**Every gate applies to the documentary scope only.** No implemented contact service, schema, browser behavior, test result, security certification, delivery or deployment may be claimed. A reported `COMPLETE` is not independent `ACCEPT`. If a material inconsistency survives or any gate fails, publish a truthful partial or STOP report when safe; do not claim R00.F closure.

### 7. Handoff and next transition

In `.documents/Reports/execution-report.md`, report exact installed R06 identity and checksum, preflight and issuance-freeze evidence, source commits/tree, exact Architecture paragraphs amended, full G01–G10 results, preservation and contradictory-text checks, any skipped/unverifiable proof, working-tree/temp cleanup and remote read-back. Do not embed the report commit's own SHA in its content; include the real URL/SHA in the agent's separate handback.

The agent **must not** declare independent `ACCEPT`, close `R00.F`, remove it from ROADMAP, clear ACTIVE, populate NEXT, issue `R00.G`, or begin later implementation. After independent Architect `ACCEPT` of R06, the Owner may separately authorize **one administrative R00.F closure/reconciliation**: remove exactly R00.F from ROADMAP, clear ACTIVE to the neutral title, and select `R00.G` in NEXT only after readiness assessment. That is a distinct order and is **not** authorized by R06.
