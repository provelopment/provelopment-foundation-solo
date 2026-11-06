# Foundation Solo — Active Work Order

## SOLO-WO-014 R03 — Final O05 Module-Behavior Contract Corrections

| Field | Exact requirement |
| --- | --- |
| Work order / edition | `SOLO-WO-014` / `R03` — supersedes R02 in full as the active correction edition |
| Status | `OWNER-ISSUED` |
| Classification | Stage 00, `R00.D`, **documentation-only**, no application implementation or administrative closure |
| Local checkout root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` |
| Installed task file | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / existing `main` / verified existing `origin` |
| Required starting GitHub/main revision | `586e7bc9e018ae02539c212ad3b31c628b045eaf` — independently verify local HEAD, tracking ref and actual remote before writing |
| R02 content baseline | `6609217c39cdd8d4070e956ae51194f98cad3cfa` |
| Execution instruction | `execute C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Governing procedure | Local `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md` §§6A–8 and `.documents/SOP/sop-implementation.md`, particularly existing local-Git procedure |

## OWNER-ISSUED APPROVED SCOPE — SOLO-WO-014 R03

### 1. Objective and exact boundary

Correct four remaining **first-physical-profile `O05` documentation ambiguities**, designated F04–F07 below, so the future Implementation Agent has no freedom to choose a default action style, layer geometry, parallax progress mapping or carousel playback/navigation semantics. These are **explicit new first-profile refinements** in this complete work order, not purported earlier D01–D12 Owner choices. Preserve the already-codified first-profile physical contract and the accepted architectural decisions. The Implementation Agent shall integrate the exact normative text in §4, **not invent alternate behavior**. If a required change conflicts with any previously accepted architecture, STOP with precise evidence.

The preceding R02 changes **F01 (HTTP(S) base admission), F02 (one field-provenance shape) and F03 (motion and other module defaults)** have been independently checked and must not be weakened. All `D01`–`D12`, D03/D05 amendments, `O01`–`O12`, R00.A/B/C, `V01`–`V28`, `P01`–`P14`, 21 module registrations, 9 property registrations and seven design references are preserved. This is no authority to reopen source selection, property inheritance, EFS identity, route/locale/visibility policy, graphic source safety or the approved form-builder exclusion.

**Completion:** the focused Charter edits are published in one documentation commit, the installed ACTIVE edition is preserved verbatim, every positive/refusal case below is supported by direct canonical text, the required regression checks pass, and the single current factual report is published separately. No implementation, test pass, product release or R00.D administrative closure is claimed.

### 2. Exact authorized permanent path ledger

| Path | Exact authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Commit the Owner-installed R03 edition **byte-for-byte** as the frozen scope; do not modify, clear, annotate or self-sign-off. |
| `.documents/Architecture/ARCHITECTURE.md` | Make **minimal in-place** changes only to `§6.2E.2` module rows and adjacent normative first-profile paragraphs, plus `§6.2N` **P09** for coverage. No other architecture sections may be rewritten. |
| `.documents/Reports/execution-report.md` | Overwrite with this run's factual evidence; commit/push in one **separate report-only commit**. |

Every other permanent path is **read-only**, including Product Definition, ROADMAP, NEXT, OUTLOOK, README, AGENTS, SOPs, schema artifacts, source/asset directories, tooling and other repositories. Do not create another WIP file, ADR, decision register, schema artifact, test harness, branch, worktree, checkout, clone, report archive or shadow authority. Temporary work follows the existing Solo `.temp/` / actual agent-provided temp rules and cleanup in the SOP.

### 3. Local execution sequence and safety

1. Read the installed R03 and all governing local documents, including the Charter, Product Definition, all four WIP files and current canonical report. Confirm the existing local checkout is exactly the stated Solo repository; one worktree, branch `main`, exactly one authorized `origin`; verify `git rev-parse HEAD`, `origin/main` and `git ls-remote` all agree with the required starting revision. Confirm exactly the four approved WIP files. Verify `git status --porcelain` has **only** the expected Owner-installed ` M .documents/WIP/workorder-active.md`, with no unrelated changes. Record actual ACTIVE SHA-256, byte count and Git blob. Owner-installed ACTIVE is task input and not an unexpected-dirty-state failure.
2. Stage **only** ACTIVE; commit and push the exact installed R03 bytes using the established local Git procedure; verify remote readback. Do **not** reset NEXT (it is neutral), remove R00.D from ROADMAP or clear ACTIVE.
3. Apply F04–F07 below **only** in the permitted Charter paragraphs; do not change any module ID, field name, enum, unrelated default or version. Examine current `§6.2E.1`, all 21 `§6.2E.2` module rows, the first-profile optional-value paragraph, P09 and the R02 correction diff for consistency.
4. Check every positive/refusal case in §4 and all eight §5 acceptance gates. Verify exact JSON syntax of the three existing worked examples using available local read-only parsing without adding dependencies; check their structural compatibility. Recheck that all 21 module IDs, 9 property keys, 12 D index rows, 12 O gates, 28 V cases, 14 P cases and seven design references remain intact. Check for stale or contradictory wording in both canonical documents. Record real line spans and a complete changed-line audit.
5. Commit/push only the permitted Charter changes from the local checkout, read back the actual published GitHub commit and Charter blob, then separately overwrite, commit and push the one canonical report and verify its published content. Report all commit IDs and URLs in the chat handback; do **not** self-reference the report's future commit ID inside the report.
6. End with an honest local worktree and temporary-artifact inventory, preserving all unknown/Owner-owned files. If unable to complete safely, STOP and publish a factual current report when safe. Never self-award an Architect ACCEPT.

GitHub is the **publication and subsequent Chief Architect review location**, not the agent's editing filesystem.

### 4. Complete corrective first-profile contract (normative)

#### F04 — Deterministic omitted `content.actions.items[].variant`

**Defect:** `§6.2E.2` registers each action item with an optional enum `variant: primary|secondary|text`, yet the R02 optional-value rule says optional behavioral enum values have **only** an explicitly documented default. The `content.actions` row has no such default. The missing value would force a renderer to choose the visible style.

**Required exact first-profile selection:** In `content.actions.props.items[]`, the optional `variant` has exactly the allowed enum `primary|secondary|text` and **when omitted is `text`**. `text` is the neutral inline/link-like treatment; it must remain visibly interactive, have a clear accessible name and visible focus, and must **not** silently turn into a primary CTA. `primary`/`secondary` are explicit author-selected relative visual-emphasis variants, not separate link authority or HTTP methods. All three use the same previously accepted typed-link validation; their visual treatment cannot override navigation, exposure, accessibility, or routing constraints. Add the exact default to the existing `content.actions` row and cross-reference it from the general optional-value rule. **Do not** add an unspecified fourth variant or a global styling override.

**Positive/refusal cases:** Item with no `variant` → `text`; `primary` or `secondary` explicitly authored → respective validated intent; an unknown enum → `MODULE_INVALID_PROPS`; no variant ever gives a link extra destination, routing or fetch authority.

#### F05 — Physical `layout.layers.minHeight` scale and deterministic layer alignment

**Defect:** `layout.layers.props.minHeight` permits `sm|md|lg` with default `md`, but its **visible minimum-height mapping** is not defined. The generic padding spacing tokens (`sm=8`, `md=16`, `lg=24` CSS px) do **not** state whether they govern layer minimum height; interpreting the layout height as padding tokens or inventing different values would silently alter authored layouts. The axis/meaning of layer `align` is also not declared.

**Required exact first-profile selection:** Treat `layout.layers.props.minHeight` as its **own closed layer-height scale**, not a spacing/padding token: `sm` means **240 CSS pixels**, `md` means **400 CSS pixels**, `lg` means **640 CSS pixels**, at every breakpoint unless a larger natural content requirement expands the region. The omitted default remains `md` → **400 CSS pixels**. These are **minimum**, never maximum or forced fixed heights. Content can grow the layer region; the module must not clip essential content, create hidden focus targets or conceal another interactive layer. Children share one relative layer region, retain authored DOM/reading order, and paint in authored order with later children visually above earlier ones; paint order never creates semantic or keyboard precedence. `align` (`start|center|end`; omitted `center` as already fixed by R02) governs **vertical block-axis positioning** of each layer's natural-height contents in that region: start/top, center/middle, end/bottom. Horizontal layout fills the available content width; no independent implicit cross-axis enum, raw z-index or authored CSS field exists. If layers cannot be rendered accessibly without obscuring essential material or controls, **fail validation** rather than allowing silent overlap or clipping. Add these literal meanings adjacent to the `layout.layers` module row and preserve the prior `presentation`, `span` and node-depth rules.

**Positive/refusal cases:** absent `minHeight` → minimum 400 CSS px; explicit `sm`/`lg` → minimum 240/640 CSS px; content taller than the minimum expands without clipping; `align:start` top / `center` middle / `end` bottom; unknown minHeight/align values or inaccessible essential overlay → `MODULE_INVALID_PROPS` or the appropriate registered content refusal, not a renderer-chosen fallback. All values remain valid irrespective of viewport breakpoint; a future breakpoint-dependent height token would require an explicit new versioned contract.

#### F06 — Deterministic `design.motion` parallax progress and smoothing meaning

**Defect:** R02 specifies a bounded −24..+24 CSS-pixel parallax transform driven by "normalized viewport scroll progress", but without defining the range endpoints or mapping. Different adapters could interpret the same serialized `parallax` node differently.

**Required exact first-profile selection:** For an element whose rendered bounding rectangle has viewport-relative top `T` and positive height `E`, and a viewport of positive height `H`, define clamped normalized progress as:

`p = clamp((H - T) / (H + E), 0, 1)`.

Thus `p=0` when the element's **top** reaches the viewport **bottom**, and `p=1` when its **bottom** reaches the viewport **top**. The desired vertical translation is exactly `y = (2p - 1) × 24 CSS px` (−24 px through +24 px). Parallax has **no** direction prop. Its optional `durationMs` retains R02's default **800 ms** and authored integer range **100–3000 ms**, now interpreted as the time for a **linear interpolation from current displayed translation to the latest desired translation**, restarting from the current interpolated position on retargeting, with no overshoot. If the element or viewport has nonpositive or nonfinite geometry, render it statically instead of dividing by zero or exposing an unsafe transform. When reduced motion is requested or no scroll-capable viewport exists, translation is **0** and all content remains visible and keyboard-accessible. The adapter can choose its animation library but cannot change this observable meaning or default timing. Preserve already specified `fade`, `slide`, `reveal`, `marquee`, motion pause and safety contracts unchanged.

**Positive/refusal cases:** `p=0` ⇒ −24 px, `p=.5` ⇒ 0, `p=1` ⇒ +24 px; reduced motion ⇒ 0 static; `direction` on parallax refuses as already specified; out-of-range `durationMs` refuses; no finite geometry ⇒ safe static fallback, never invalid arithmetic or hidden content.

#### F07 — Deterministic `media.carousel` initial, manual and autoplay behavior

**Defect:** The R02 contract fixes carousel controls, interval and reduced-motion safety, but does not state initial slide, how previous/next operate at endpoints or how user navigation interacts with autoplay. These are visible behaviors a future adapter would otherwise invent.

**Required exact first-profile selection:** On initial render, **slide index 0** (the first authored `slides[]` item) is active. Previous/Next move exactly **one** authored slide per activation and **wrap cyclically** (`previous` from index 0 activates last; `next` from last activates index 0). The non-active slides must not expose hidden focusable controls or be announced as current content; preserve the real author-supplied alt/caption semantics. Controls remain present and keyboard-operable, with accessible names `Previous`, `Next`, and a stateful `Start`/`Pause` control. When `autoplay` is omitted or false, advancement never starts automatically; the Start control may start a visitor-requested slideshow using the **same** 10000 ms default interval if no `intervalMs` is authored, but no `intervalMs` field is permitted in the authored object unless effective `autoplay:true`. When `autoplay:true` is authored, advancement begins automatically at the R02 validated/derived interval **only when reduced motion is not requested**. Each automatic advancement resets the complete interval; it wraps cyclically. Hover/focus temporarily pauses a running carousel and it resumes when that hover/focus ends **only if** the visitor has not manually paused. Pressing Pause or manually selecting Previous/Next stops automatic advancement until the visitor explicitly presses Start again. The Start/Pause control remains **visible but cannot enable automatic cycling under a reduced-motion preference**; manual Previous/Next remain operable. Do not preload/fetch undeclared assets or let off-screen focus leak through inactive slides. Preserve slide validation, 2–12 bound, `controls:true`, and all R02 `intervalMs` positive/refusal rules.

**Positive/refusal cases:** initial first slide; Previous from first wraps to last, Next from last wraps to first; autoplay omitted/false never autoadvances; autoplay true with absent interval uses 10000 ms; manual navigation pauses autoplay; user Pause persists after hover/focus ends; reduced motion disables autoadvancement and Start action while preserving manual navigation; `intervalMs` authored with `autoplay` false/absent still refuses; invalid slide count/asset/alt refuses. No passive transition can change EFS publication or discovery.

**Important reconciliation:** F04–F07 become exact `1.0.0` behavior only through the accepted R03 edition. They are not an instruction for the agent to make product/UI design decisions. In case any statement cannot be reconciled with existing §6.2E.1–§6.2E.3, STOP rather than silently revise other accepted constraints.

### 5. Mandatory acceptance and refusal gates

| Gate | Evidence required |
| --- | --- |
| G01 — Local authority and hygiene | Correct R03 edition, baseline SHA, local single worktree, branch/remote, only expected Owner-installed ACTIVE change, exact frozen ACTIVE blob and safe publishing. |
| G02 — F04 action variants | `content.actions` omitted variant exact `text`, explicit values preserved, invalid enum refused, typed-link authority unaffected. |
| G03 — F05 layer layout | minHeight exact 240/400/640 CSS px and expanding minimum, vertical alignment semantics, content/focus safety refusal; no confusion with padding tokens. |
| G04 — F06 parallax | Exact scroll progress formula/endpoints, ±24 px transform, 800 ms default and interpolation semantics, reduced-motion and invalid-geometry behavior, prohibited direction. |
| G05 — F07 carousel | Index 0 initial; cyclic one-slide manual controls; timer/pause/hover/focus and reduced-motion decisions; R02 interval/refusal cases unchanged. |
| G06 — Complete preservation | Existing `D01`–`D12` + D03/D05 amendments, `O01`–`O12`, 21 modules, 9 properties, `V01`–`V28`, `P01`–`P14`, seven URLs and R00.A/B/C contracts retained, with no unrelated edits. |
| G07 — Examples and contradiction audit | Three existing JSON examples still parse; no worked-example schema/routing/visibility/source inconsistency; all 21 module rows and optional-value paragraph inspected for remaining omission ambiguity. Explicitly report any unresolved first-profile choice, never assume a renderer default. |
| G08 — Exact publication and status | Only authorized permanent paths, locally created commits/pushes and remote readback, separate report-only commit, clean-tree/temp evidence, ROADMAP/NEXT/OUTLOOK untouched, ACTIVE not cleared, R00.D not closed, no fabricated tests or Architect sign-off. |

All applicable documentary gates require direct published evidence. A failed or unresolved gate means **STOP or bounded correction and factual report**, not generic PASS. Application code/tests/build/browser/deployment are **NOT APPLICABLE** to this order and must never be claimed as run. No unapproved new module, provider service, schema artifact, route, field, styling escape hatch or form subsystem is allowed.

### 6. Permissions, reports, resumption

Read all necessary local Solo governance and contract files; edit only the ledger paths; use existing local `main`, `origin`, and standing SOP Git mechanics. No branch, clone, worktree, history rewriting, force push, PR, merge, release, deployment, dependency installation, code, asset delivery or other repository write. Never change ROADMAP, NEXT or OUTLOOK or clear ACTIVE. If root/HEAD/remote conflict, an unrelated dirty path, an accepted-contract conflict, a new implementer choice, a security boundary or publication block is found, STOP and report exactly what was found without destructive cleanup.

Overwrite the **single** `.documents/Reports/execution-report.md`, identifying `SOLO-WO-014 R03`, outcome, preflight, ACTIVE SHA-256/blob, separate A/B/C commits, exact diff and file ledger, F04–F07 anchors and positive/refusal checks, G01–G08 evidence, D/O/V/P/module/property/reference counts, parser evidence and limitations, unchanged-path blob checks, cleanup, current WIP state and Chief Architect review resumption point. Publish the report in a separate commit; supply its final GitHub SHA in the handback, not within the report itself. No agent self-certification of R00.D acceptance or closure.

**End of frozen approved scope — SOLO-WO-014 R03.**
