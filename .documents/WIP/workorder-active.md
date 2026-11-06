# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-011`  
**Edition:** `R02`  
**Title:** R00.C — Narrow documentary consistency correction following independent R01 review  
**Approval marker:** `OWNER-ISSUED`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Target branch:** `main`  
**Required starting `origin/main` HEAD:** `fb78dc5ccf65580e145f73cd174ba1144a4664d1`  
**Matching handoff:** `SOLO-WO-011 R02 — Implementation Agent Handoff`  
**Classification:** Corrective documentation-only scope; no application implementation  
**Quality:** A1 documentary correctness, strict scope confinement and GitHub-verifiable evidence  
**Independent review:** Chief Architect `ACCEPT / REVISE / STOP`  
**Closure / NEXT transition:** Separate subsequent Owner-issued order only

## Frozen Approved Scope — Sections 1–9

This complete Approved Scope is executable **only** when the Owner installs its exact full text in `.documents/WIP/workorder-active.md` and directly delivers the matching `SOLO-WO-011 R02 — Implementation Agent Handoff`. After verifying both and the repository preflight, the Implementation Agent **executes** all authorized phases. No supplementary proposal or attachment is necessary; do not request separate Owner Git-command approvals. Do not act outside the exact scope.

## 1. Objective and independent R01 disposition

Correct the finite stale inheritance/graphics statements left after `SOLO-WO-011 R01` without revising any C01–C10 decision. The Chief Architect independently reviewed the published R01 execution report at `fb78dc5ccf65580e145f73cd174ba1144a4664d1` (R01 content commit `ccec99462b6a1bf37c30594b51879b4cfe909123`) and returned **REVISE**: R01 G03 fails whole-document consistency, although publication path/commit evidence was verified. R01's report itself disclosed stale Architecture §6 and Product §2 wording. Additional read-only entry-point/procedural material also contains stale nearest-value or loose override wording: root README, AGENTS, and Implementation SOP. R01's reference to a bounded command-capture log also omitted its final cleanup disposition. This R02 order is the bounded corrective action; it does not retroactively modify or relabel R01 evidence.

The Owner-accepted logical contract remains:

- C01 registered typed property core with governed extensions; C02 absent/value/suppressed distinctions; C03 two-element `inheritance: inherit | suppress` plus `currentContext: none | zero-or-more typed local values`, direct ancestry, deterministic inherited-first additive resolution, effective cardinality validation. `none` belongs only to current context. Suppression blocks inherited contributions but permits valid local values. No sibling inheritance or silent nearest/last-value selection.
- C04 per-value/field provenance; C05 registry-restricted per-value public/internal projection; C06 zero-or-more structured location facts, whole-record additive inheritance by default, explicit extension of one uniquely identified inherited record, no implicit field merging, no geographic inference or routing authority.
- C07 extensible registered graphic roles with per-role cardinality; C08 typed Entity-owned and generic-platform source asset references; C09 governed role-specific generic fallback disabled for every role initially, with suppression/invalid/privacy boundaries respected; C10 incomplete drafts only outside the strictly valid published graph.
- R00.A/R00.B identity, publication, route, full-locale, visibility and unlisted contracts, one Entity/Installation and strict Hexagonal Architecture remain unchanged. Public `mailto:` requires an address exposed to the browser; internal simple-form recipients are server-resolved under later `R00.F`. No new technical or physical schema decision is authorized.

## 2. Exact permitted permanent paths and operations

| Path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs the exact R02 order; agent preserves and publishes **unchanged** in separate Phase A; keeps it installed after execution. |
| `.documents/Architecture/ARCHITECTURE.md` | Phase B: replace **only the two obsolete verification-table rows** in §6 identified in §4. |
| `.documents/About/FOUNDATION-SOLO.md` | Phase B: replace **only §2 item 4** identified in §4. |
| `README.md` | Phase B: replace **only the EFS location-summary paragraph** identified in §4. |
| `AGENTS.md` | Phase B: replace **only the two §3 bullets** for cascading location and graphics identified in §4. |
| `.documents/SOP/sop-implementation.md` | Phase B: replace **only §6 item 6 and the §7 Property cascade table row** identified in §4. |
| `.documents/Reports/execution-report.md` | Phase C: overwrite sole canonical report with factual R02 evidence, separate commit. |
| `.git/**` | Ordinary version-control metadata for expressly authorized phases under Architect SOP §6B. |

All remaining permanent paths are **read-only**, including ROADMAP, NEXT, OUTLOOK, Architect SOP, `.github/**`, license, application code, toolchain, tests, and all other repositories. The other paragraphs and rows of the six edited documents are read-only. No additional checkout/worktree/clone, parallel report, new WIP file, other repository write, or source tree.

**Separate permissions:** Three ordinary commits and verified fast-forward pushes to Solo `origin/main` are authorized: Phase A order-only, Phase B exactly five correction-document paths, Phase C report-only. Authenticated GitHub readback is authorized. Branch/PR creation, merging, release, deployment, tooling/dependency operations, provider/account operations, secrets, destructive Git actions, force-push or rewriting history are **not** authorized. Architect SOP §6B is the sole Git procedure.

## 3. Exact preflight and required baseline

- Verify the one actual Solo checkout/toplevel, branch `main`, one expected Solo `origin`, one worktree, HEAD and remote-side `main`, safe clean status except **only** the Owner-installed exact R02 ACTIVE replacement. No assumed local path, secondary checkout or parent workspace authority.
- Verify remote starting HEAD exactly `fb78dc5ccf65580e145f73cd174ba1144a4664d1` and these **Git blob SHAs** before edits:

| Path | Required blob SHA |
| --- | --- |
| `.documents/WIP/workorder-active.md` before Owner R02 installation | `006d45ef22dbaa34e0cad9a2ca8eea5d10dd7ea9` |
| `.documents/Reports/execution-report.md` | `a47b1b73a74d08b165248bb7cf2648b1e1948588` |
| `.documents/Architecture/ARCHITECTURE.md` | `ff18a3fc8b54b4bfce53272368a73a64a5edafb7` |
| `.documents/About/FOUNDATION-SOLO.md` | `be0ff7faf23f141e7732a7101b514dabbcc8c6b5` |
| `README.md` | `572121c54b1ac91ac6b5f36515ce0b959f79d6c9` |
| `AGENTS.md` | `d4a2a028e0d35852e0077bbe97c162bdfde89d2d` |
| `.documents/SOP/sop-implementation.md` | `b868faccbe73e076d207ed1c532069559ca3590a` |
| `.documents/SOP/sop-architect.md` | `0396251245180b40ee896b50a3456b92835f9cb1` |
| `.documents/WIP/plan-roadmap.md` | `05feb2e30cada5cf013b08b1daf7011ed504eaa4` |
| `.documents/WIP/plan-next.md` | `7996a1e568f076062a0030ca7bdc8b97c5674f9f` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |

- Read README, AGENTS, both SOPs, the two canonical authorities, all four WIP files and the current report. Verify exact `SOLO-WO-011 R02` installed bytes, `OWNER-ISSUED`, and matching direct Owner handoff; inventory exactly four WIP files and one report.
- If the baseline, installed scope, other dirty files, remote, or safety cannot be established, STOP and publish a truthful report only when safe. Do not substitute a different checkout, infer approval from a file name, or ask the Owner to perform authorized routine Git commands.

## 4. Exact Phase B text substitutions (eight occurrences only)

For each target below, find exactly **one** occurrence of the specified old text inside its specified section, replace only that occurrence with the new text, and preserve every other byte of that file. Refuse a missing or nonunique match. The replacement wording codifies existing accepted decisions, not new architecture.

### 4.1 Architecture Charter §6 — Property cascade table row

**Old:**

```text
| Property cascade | Nearest declaration wins; absence inherits; explicit suppression works; provenance is correct |
```

**New:**

```text
| Property cascade | Direct-ancestry additive `inheritance`/`currentContext` resolution; absent declaration inherits; `suppress` blocks inherited contributions but retains valid local values; per-value provenance, effective cardinality and invalid-declaration refusal verified |
```

### 4.2 Architecture Charter §6 — Graphic cascade and shared library row

**Old:**

```text
| Graphic cascade and shared library | One platform public graphic asset is reachable throughout Solo; local/inherited graphic properties override appropriately; explicit suppression prevents fallback; content owner assets remain correctly sourced |
```

**New:**

```text
| Graphic cascade and shared library | Shared generic assets are available throughout one Installation; graphic roles use additive direct-ancestry resolution with effective role cardinality; `suppress` may retain valid local selections but blocks inherited values and automatic fallback; typed Entity/generic sources remain distinct |
```

### 4.3 Product Definition §2 — Outcome item 4

**Old:**

```text
4. **Reuse without duplication:** permit shared Entity-level facts and narrower overrides via a predictable property cascade.
```

**New:**

```text
4. **Reuse without duplication:** permit additive direct-ancestry inheritance of shared Entity-level property values, explicit suppression of inherited contributions, and registered local additions; identified location-record extension is deliberate, not implicit geographic merging.
```

### 4.4 Root README — EFS location paragraph

**Old:**

```text
**Location is an explicitly authored cascading EFS property, not a routing input.** Entity location `Australia` can be overridden by Facet `Indonesia` and then by Sites `Bali` or `Jakarta`; their `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` localized Page representations do not infer or alter location.
```

**New:**

```text
**Location is explicitly authored EFS property metadata, never a routing input.** Direct-ancestry inheritance is additive: an Entity fact `Australia` and a Facet fact `Indonesia` can both remain effective when the Facet selects `inherit`; a Site may select `suppress` and contribute `Bali` or `Jakarta` alone. Structured location records may be explicitly extended by identified reference, never implicitly completed from a route, locale, name or geography; `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` do not alter authored location.
```

### 4.5 Root AGENTS §3 — Cascading location bullet

**Old:**

```text
- **Cascading location:** an explicitly authored EFS property at Entity, Facet, Site, Page or representation level, with nearest direct-ancestor declaration taking precedence. A location may be Australia at Entity, Indonesia at Facet, Bali or Jakarta at different Sites. No location is inferred from a URL, ID, locale, hostname or route. Location never selects a route, canonical, language or Page.
```

**New:**

```text
- **Cascading location:** explicitly authored at Entity, Facet, Site, Page or representation level, with additive values along direct ancestry under `inheritance: inherit | suppress` and `currentContext: none | zero-or-more local facts`. A Fact at Entity (Australia) can combine with a Facet fact (Indonesia), or a Site can suppress inherited facts and contribute Bali/Jakarta; explicit identified-record extension is the only permitted field-level location-merging operation. No location is inferred from URL, ID, locale, hostname or route; location never selects a route, canonical, language or Page.
```

### 4.6 Root AGENTS §3 — Graphics bullet

**Old:**

```text
- **Graphics:** a generic public graphic library is available across the single Installation; authored graphic roles cascade through Entity, Facet, Site, Page and representation; explicit suppression prevents inherited and generic fallbacks.
```

**New:**

```text
- **Graphics:** one shared generic public graphic library serves the Installation; registered role selections use additive direct-ancestry EFS inheritance with validated effective cardinality. `suppress` excludes inherited selections but permits valid local selections; it never independently authorizes an automatic generic fallback. All automatic role defaults are disabled unless expressly approved.
```

### 4.7 Implementation SOP §6 — Item 6

**Old:**

```text
6. Resolve location along the explicit **Entity → Facet → Site → Page → representation** property ancestry only. For example: `Australia` at Entity, `Indonesia` at a Facet, `Bali` at one Site and `Jakarta` at another. Full locale tags `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` do not alter those values absent an explicit representation override. Do not mint language-only published keys. Suppression and privacy must follow the accepted property contract.
```

**New:**

```text
6. Resolve location solely through the direct **Entity → Facet → Site → Page → representation** ancestry using accepted additive `inheritance`/`currentContext` semantics: a Facet can inherit the Entity's Australia fact and add Indonesia; a Site can explicitly suppress inherited facts and contribute Bali or Jakarta; an identified inherited record may be extended only under its accepted validated extension rule. Full locale tags `en-001`, `de-DE`, `zh-Hans-CN` and `id-ID` never infer or modify location. Do not mint language-only published keys or use location to determine routes. Public/internal projection and suppression must follow the accepted property contract.
```

### 4.8 Implementation SOP §7 — Property cascade row

**Old:**

```text
| Property cascade | Nearest explicit value wins; direct ancestry only; suppression works; private fields not leaked |
```

**New:**

```text
| Property cascade | Additive direct-ancestry inheritance and valid local values; `suppress` excludes inherited contributions but permits local ones; effective collection cardinality and duplicate/refusal cases verified; per-value provenance and restricted public projection do not leak internal facts |
```

## 5. Independent evidence and consistency checks

The Agent must directly verify the **eight exact replacements** and no other textual changes in the five Phase B documents, including exact `git diff --check` or equivalent checks and hunk/content review. Confirm:

- Neither canonical file contains the stale nearest-value-wins acceptance requirement; Architecture §2 and Product §4 remain the existing R00.C Owner-accepted source of truth.
- README, AGENTS and Implementation SOP now summarize additive inheritance without contradicting the canonical contract; the Owner-decided `inheritance`/`currentContext` and location-extension semantics remain unchanged.
- No change to R00.A/B ID, publication, locale, route, case, Unicode, unlisted, base URL, one Entity or Hexagonal boundaries; no new approved role names, generic fallbacks, physical schema or provider choice.
- No extra WIP files, report archives, source artifacts or unauthorized edits. ROADMAP/NEXT/OUTLOOK remain exactly the baseline blobs.
- Document positive/refusal matrix for: inherit+none; inherit+local; suppress+none; suppress+local; effective singular graphic cardinality failure; a location extension with existing target and rejection of missing/suppressed target; internal-to-public leakage refusal; invalid draft outside published graph; location/route and locale independence. These are **documentary assertions only**, not executed software tests.
- Evidence should distinguish GitHub-observed commits/paths from locally observed worktree/cleanup. No software build, browser, unit, integration, provider-delivery or deployment results are to be invented.

**Cleanup follow-up:** R01's report mentioned a bounded command-capture log but did not establish its cleanup/disposition. At R02 preflight, inventory safe, accessible task-owned Cline/command-capture files and Solo `.temp/`. Remove only positively identified obsolete task-owned files where permitted and safe; do not delete unknown, inaccessible, unrelated or Owner-owned items. Report the R02 preflight observation, scope of accessible checks, any actual cleanup, unverifiable R01 historical state, and justified retention/cleanup trigger. A new empty temp area does not retrospectively prove R01 cleanup. At R02 run end leave no unnecessary R02 temp garbage and explicitly report both temporary-area categories.

## 6. Authorized publication phases

**Phase A — Issued R02 order:** Preserve the Owner-installed R02 ACTIVE bytes exactly; stage only ACTIVE, make one ordinary order-only commit, fast-forward push to verified `origin/main`, and read back order/commit paths/HEAD before Phase B.

**Phase B — Five-document correction:** Perform §4's eight substitutions only. Stage only Architecture, Product, README, AGENTS, Implementation SOP. Validate exact old/new matches, complete diff, no unintended changes and §5 matrix. Make one ordinary five-file content commit, fast-forward push, verify remote readback and exact changed paths. Do not touch ACTIVE, NEXT, ROADMAP or OUTLOOK.

**Phase C — One report:** Overwrite only `.documents/Reports/execution-report.md` with the R02 factual report; include phases A/B SHAs, GitHub verification, positive/refusal and gates, scopes and limitations, current cleanup evidence, old gap disposition and next Architect review. Make a separate report-only ordinary commit, fast-forward push and read back GitHub. Do not embed the Phase C commit's own SHA in its content; return SHA/URL in chat.

**Required end state:** ACTIVE is still the exact issued R02 order. WIP remains four files. One current canonical report. Clean tree and safe temporary cleanup as observed. R00.C remains administratively open; this task is corrective codification only.

## 7. Acceptance gates

| Gate | Requirement |
| --- | --- |
| G01 — Authority/baseline | Exact Owner-issued `SOLO-WO-011 R02`, matching direct handoff, correct baseline, single Solo checkout/worktree and expected only Owner-installed ACTIVE modification |
| G02 — Phase A | Exact R02 order preserved alone in one pushed commit, unchanged bytes and remote readback |
| G03 — Canonical correction | Architecture §6 two rows and Product §2 item 4 exactly corrected, while §2/§4 detailed contracts remain intact |
| G04 — Root entry points | README EFS location paragraph and AGENTS two §3 bullets exactly corrected |
| G05 — Implementation SOP | Only §6 item 6 and §7 Property cascade row corrected; Architect SOP unaffected |
| G06 — Owner contract fidelity | C01–C10 unchanged; additive property, explicit suppression, located-record extension, privacy and graphics meaning consistent across the documents |
| G07 — R00.A/B preservation | ID/publication/routing/locale/unlisted/Hexagonal and other accepted contracts unchanged; no schema/code/provider selected |
| G08 — Exact Phase B scope | Only the five authorized correction documents, exactly eight replacements, one content commit; GitHub readback |
| G09 — Phase C report | One current report, separate report-only commit, remote readback and proper no-self-SHA convention |
| G10 — WIP hygiene | Four WIP files, issued ACTIVE unchanged after Phase A, ROADMAP/NEXT/OUTLOOK preserved |
| G11 — Worktree and temporary cleanup | Actual clean tree, one worktree, no unreported obsolete task-owned files; honest Cline/Solo-temp inventory and R01 gap handling, no blanket deletion |
| G12 — Honest evidence and exclusions | Direct document/diff/gate evidence; documentary positive/refusal scenarios not mislabeled runtime tests; no premature closure or unauthorized implementation |

For every gate give `PASS`, `FAIL`, `UNVERIFIED`, or `NOT APPLICABLE`, with direct evidence/reason. Software builds, app tests, HTTP/browser, contact-delivery and deployment checks are `NOT APPLICABLE` to this documentation-only correction, not fabricated PASS results.

## 8. STOP and escalation

STOP if installed authority/handoff is unverifiable; baseline/remote/branch diverges; unknown dirty content or second worktree is present; one of §4's exact old texts is absent/nonunique; correction requires an unlisted path/paragraph; the accepted contract cannot be preserved within eight substitutions; a safety/temporary-cleanup problem needs destructive handling; or authorized publication is unsafe. Do not expand scope or change approved R00.C decisions. Preserve evidence and publish a truthful STOP report only when safe under the standing report-only exception.

## 9. Completion handback and independence

Return exact Phase A, B and C GitHub SHAs/links, starting/final HEAD, verified path inventory, eight replacement checks, full G01–G12 matrix, canonical report URL, actual local status, cleanup outcome and all limitations. The Agent must not self-accept, close `R00.C`, edit ROADMAP/NEXT/OUTLOOK, clear ACTIVE, start R00.D, merge, release or deploy. Independent Chief Architect review determines `ACCEPT / REVISE / STOP` against the exact published Phase C report revision. Separate subsequent Owner authorization is required for R00.C administrative closure and NEXT transition.
