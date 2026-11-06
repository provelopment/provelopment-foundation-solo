# Foundation Solo — Active Work Order

## SOLO-WO-014 R02 — R00.D Physical Publication Contract Corrections

| Field | Exact requirement |
| --- | --- |
| Work order / edition | `SOLO-WO-014` / `R02` — replaces R01 in full; bounded documentary correction, not another architectural decision series |
| Status | `OWNER-ISSUED` |
| Classification | Stage 00 / R00.D, **documentation-only contract correction**; no application, schema artifact, test harness, dependency, media asset, release or deployment |
| Local execution root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — existing local checkout only |
| Installed work order | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Owner's direct execution instruction | `execute C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / existing `main` / verified `origin` |
| Required starting HEAD | `2d3e81011ba717722e681c15f21a9c8caa6a9d4f` — the R01 report-only publication; verify local HEAD, `origin/main`, `git ls-remote` before writes |
| Reviewed baseline physical profile | Codification commit `04d9f7ba88cf3ea99ebb36acecc71e0f21986ed3` |
| Binding procedures | Existing `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md` (particularly §§6, 6A, 6B, 7, 7A, 7B, 8) and `.documents/SOP/sop-implementation.md` |

## OWNER-ISSUED APPROVED SCOPE — SOLO-WO-014 R02

### 1. Objective

Correct **three specific physical-contract defects** found by the Chief Architect's independent review of `SOLO-WO-014 R01`. The corrections are fully described in §4 below and are the only new physical selections in this edition. Apply them within the existing canonical Architecture Charter. Preserve the twelve Owner-accepted `D01`–`D12` decisions (including the D03/D05 amendments), the rest of the first-profile `O01`–`O12` contract, the R00.A/B/C invariants, the registered twenty-one module IDs, their `1.0.0` version identities, the nine property registrations, the D/V/O/P coverage ledgers and all seven design references. No new field, module, EFS node or publishing authority is created by this correction.

The purpose is **implementation determinism**: remove contradictions or unspecified behavior that would otherwise force a future Implementation Agent to choose a schema meaning, permissibility rule, field-provenance serialization, input default, or motion behavior without an accepted contract. This is a correction of the **unimplemented first physical profile**, not a source-code task and not an R00.D administrative closure order.

### 2. Exhaustive permanent edit-path ledger

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Preserve and commit the Owner-installed **R02** byte-for-byte as the frozen scope. Do not edit, annotate, certify or clear it. |
| `.documents/Architecture/ARCHITECTURE.md` | Precisely reconcile F01 in §6.2B against §6.2J; F02 in §6.2H against §6.2G; F03 in §6.2E.2 and, only if needed for consistency, immediately adjacent §6.2E.1/§6.2E.3 and §6.2N's applicable P09 wording. No other architectural change. |
| `.documents/Reports/execution-report.md` | Overwrite the **only** canonical execution report with factual R02 evidence; publish in a separate report-only commit. |

All other permanent paths are **read-only**, including Product Definition, ROADMAP, neutral NEXT, OUTLOOK, README, AGENTS, SOPs, assets, sources, contracts and every other repository. Do not create additional decision documents, plan/work-order files, schemas, execution archives, shadow checkouts, clones or worktrees. Use only Solo `.temp/` or actual Cline-provided task temporary areas for disposable files and clean positively identified task-owned debris under the SOP.

### 3. Local execution and publication sequence

1. Read this actual installed R02 work order, README, AGENTS, both SOPs, Charter, Product Definition, four WIP files and canonical report **locally**. Verify exact Solo root, one checkout/worktree, `main`, exactly one correct `origin`, and baseline `HEAD = origin/main = ls-remote = 2d3e81011ba717722e681c15f21a9c8caa6a9d4f`. The expected pre-run modification is **only** the Owner-installed ` M .documents/WIP/workorder-active.md`; it is task input, not a reason to stop. Record installed bytes, SHA-256, Git blob and edition; stop for any other unexplained modification or divergence.
2. Commit and push only the installed R02 ACTIVE text from the local checkout; verify remote readback and unchanged installed blob. **Do not reset NEXT**, modify ROADMAP or clear ACTIVE.
3. Apply F01–F03 exactly as specified in §4 with a tightly scoped diff. Maintain the R01 first-profile contract in all unaffected paragraphs. Compare the affected Charter paragraphs with the R01 baseline and review every apparent inconsistency in context.
4. Conduct the documentary positive/refusal checks of §5, including exact default behavior, syntax of the three existing JSON examples using a local parser where available without dependencies, module registry identity/counts, D/O/V/P traceability and contradictions across the Charter and unchanged Product Definition. A section heading alone is not evidence of semantic correctness.
5. Commit and push **only** the permitted Charter edits from local `main`, observe branch protection and the one authoritative SOP Git procedure, and independently read back the published GitHub commit and changed-file list. No PR, merge, tag, release or deployment.
6. Locally overwrite the canonical execution report with actual evidence, then commit and push **that file alone**, read back the published report and supply its resulting commit SHA in the final chat handoff, **not inside the self-referential report**. A run blocked or stopped still makes a factual current report when safe. Do not describe unexecuted application tests as PASS.

GitHub is the **publication and subsequent independent Architect-review location**, not the Implementation Agent's editing workspace.

### 4. Exact corrective contract — no further implementer design

#### F01 — Align HTTP(S) manifest grammar with accepted R00.B

**Observed conflict:** [Architecture §6.2B `base`, formerly line 629] admits `base.canonical` as HTTPS, or HTTP loopback solely for local development; [§6.2J.1, formerly line 772] explicitly accepts an authored `http:` **or** `https:` URL with a DNS hostname or IP, with HTTPS recommended for production. The latter implements the Owner-accepted R00.B configurable **HTTP(S)** origin and optional prefix.

**Required normative resolution:** Both `base.canonical` and each `base.aliases` element admit a **validated absolute HTTP or HTTPS base URL** with a DNS hostname or IP and the safe prefix/authority restrictions of §6.2J.1. HTTP is **not** limited by the publication-schema validator to loopback or to a development-only configuration. HTTPS remains strongly recommended; a distinct deployment security policy may disallow non-HTTPS deployment without redefining the R00.D authored data schema. The physical manifest never derives its authority from the incoming Host or forwarding headers. Preserve exact alias count, IDNA/port normalization, no userinfo/query/fragment, and base-prefix-once rules. **Modify §6.2B's single base paragraph** to agree with the existing §6.2J.1; do not change §6.2J.1's accepted policy.

**Required case:** A valid `http://example.org/site` base is **schema-valid** (whether it can be deployed is a separate decision). A malformed HTTP URI, credentials, unsafe prefix, encoded separators or unknown host authority is refused. A valid `https://example.org/site` is accepted. A valid HTTP loopback base is also accepted without introducing a unique publication schema. The canonical and alias schemes are never inferred from an incoming request.

#### F02 — One exact location field-provenance representation

**Observed conflict:** [§6.2G.6, formerly line 746] gives a closed derived `fieldOrigins` map whose field values are `{ "owner": <QualifiedOwner>, "localIndex": nonnegative integer }`; [§6.2H.6, formerly line 757] instead says field provenance records `(qualified owner, operation)` at the latest explicit author. This leaves two possible per-field shapes.

**Required normative resolution:** The **sole serialized derived per-field origin shape** is precisely §6.2G.6's closed `{ "owner": <QualifiedOwner>, "localIndex": nonnegative integer }`. The `owner` and `localIndex` identify the node and precise local `add`/`extend` operation that **last explicitly authored that field**. When a fact is added, each populated field has the original `add` owner and array index. When an inherited fact is extended, only fields explicitly present in `set` receive the descendant owner and that `extend` operation's array index; untouched fields retain their prior origin. The operation kind (`added` or `extended`, along with inherited/suppressed events) is recorded **separately**, only in §6.2G.6 `resolutionEvents`, not as an extra `operation` key in `fieldOrigins`. Preserve original `originOwner`/`originLocalIndex`, fact identity and whole-record `exposure`; do not merge or disclose the internal derived provenance in public output. **Edit §6.2H.6 to defer to the exact §6.2G.6 schema** rather than giving the incompatible abbreviated pair. Do not add a second serialized provenance representation.

**Required case:** Entity `location.add` at local index 0 populates `countryCode`; Facet `location.extend` at local index 1 supplies `namedPlace`. The resolved fact retains the original identity; `fieldOrigins.countryCode` points to Entity/index 0, `fieldOrigins.namedPlace` points to Facet/index 1; `resolutionEvents` records the add and extend separately. No field-origin `operation` member is permitted. An extension never changes exposure, order or original fact identity.

#### F03 — Close optional behavior in the first-version module contracts

**Observed gap:** §6.2E.2 allows several **optional** presentation, motion and interaction parameters without defining their behavior when absent. In particular `media.carousel` allows `autoplay:true` with `intervalMs` unspecified and `design.motion` allows omitted duration/direction despite an authored effect. This contradicts the first profile's requirement that an input's meaning not be chosen by an implementation adapter.

**Required exact changes, retaining all current field names, value enums and module IDs:**

| Existing module/field | Exact first-profile omission/conditional behavior |
| --- | --- |
| `layout.stack.props.align` | Optional; omitted defaults to `stretch`. Preserve its current accepted enum. |
| `layout.layers.props.align` | Optional; omitted defaults to `center`. Preserve its current enum. |
| `design.watermark.props.position` | Optional; omitted defaults to `center`. Preserve all five accepted positions. |
| `media.carousel.props.autoplay` | Optional, omitted means `false` as already approved. Controls are always present. |
| `media.carousel.props.intervalMs` | **Only** valid when effective `autoplay` is `true`; if autoplay is true and the field is omitted, its exact default is **10000 ms**. An authored interval must remain an integer within **6000–20000 ms**. If autoplay is false or absent, `intervalMs` is prohibited; silently ignoring it is not allowed. |
| `design.motion.props.durationMs` | Optional, authored range remains integer **100–3000 ms**. Omission defaults to **800 ms** for `fade`, `slide`, `reveal`, `parallax` and **3000 ms** for `marquee`. For `parallax`, duration defines smoothing toward the current scroll-linked target, not an invented time-based autoplay. |
| `design.motion.props.direction` | Optional **only for** `slide` or `marquee`: omission defaults to `up` for `slide`, `left` for `marquee`. Allowed authored values remain `up|down|left|right`. It is prohibited for `fade`, `reveal`, `parallax`; do not ignore an invalid extra field. |
| `interaction.accordion.props.initialOpen` | Optional; omission means **all panels initially collapsed**. When present, it must name one actual item ID. `singleOpen` retains its existing default `true`. |

**Observable motion meaning, to prevent a second layer of guesswork:** `fade` reveals from opacity 0 to 1 on first viewport entry; `slide` enters from a **16 CSS-pixel offset** opposite its declared/default direction and ends in its document-flow position; `reveal` uncovers content **top-to-bottom** on first entry; these complete once, do not repeat and keep the entire underlying content accessible even before entry. `marquee` shows its content statically **until the visitor starts motion with an always-visible Start/Pause control**; once started, it scrolls the child track continuously in the declared/default direction, using `durationMs` for each **24 CSS pixels of travel**, and pauses on pointer hover, keyboard focus or user Pause. `parallax` follows normalized viewport scroll progress with a bounded **−24 to +24 CSS-pixel translation** and the specified/default smoothing duration. **All five effects render statically** with reduced-motion preference, no essential content may depend on movement, no keyboard focus may be hidden/trapped, and no authored JS/CSS is accepted. Concrete framework or animation implementation remains a replaceable adapter choice, but the visible meaning and defaults are normative.

**General optional-value rule:** Within the 21 registered module contracts, a non-behavioral optional item such as `caption`, `tagline`, `action`, `eyebrow`, `when`, or a presentation override is **absent** when omitted: do not synthesize authored content, links, SEO, assets, route claims or extra controls. Other optional numeric/enum behavior retains only the **already documented** explicit default (such as `gap:md`, `fit:cover`, `singleOpen:true`, `loop:false`), and an unrecognized or incompatible prop always refuses publication. These corrections define the first `1.0.0` contract **before any shipped implementation**; do not auto-upgrade or reinterpret an existing installed product (none exists).

**Required conceptual cases:** `autoplay:true` without an interval yields **10000 ms**, `autoplay:false` with any interval refuses; omitted motion direction and duration yield the exact effect-specific values; `direction` on `fade` refuses; omitted watermark position resolves `center`; omitted accordion initial-open yields all collapsed. Reduced-motion results in static accessible content, and content remains readable without executing an effect. Keep P09 and the existing total number of modules and physical cases unchanged; extend P09 only if needed to make the defaults/refusals enforceable without creating a parallel case ledger.

### 5. Mandatory verification and refusal gates

| Gate | Required documentary proof |
| --- | --- |
| G01 | Installed R02 edition and starting revision verified locally; one Solo checkout/worktree/origin; only expected ACTIVE modification. |
| G02 | F01's two canonical base-URL paragraphs agree and preserve `http|https`, no Host-derived authority and base prefix exactly once. |
| G03 | F02's internal location `fieldOrigins` has exactly `owner`/`localIndex` per field; `resolutionEvents` alone records operation; ancestor extension and exposure semantics unchanged. |
| G04 | F03's optional module fields have exact written defaults or valid omissions; negative cases explicitly refuse, and motion behavior is bounded and reduced-motion-safe. All 21 module IDs remain. |
| G05 | `D01`–`D12` and their D03/D05 amendments, `O01`–`O12`, `V01`–`V28`, `P01`–`P14`, nine property keys, seven design references and all unaffected first-profile literals preserved. |
| G06 | The three existing JSON examples remain syntactically valid; minimal/expanded graph and JSON body references remain coherent; no new schema artifact or code/test was generated. |
| G07 | No unknown permanent file changes; unchanged Product, ROADMAP/NEXT/OUTLOOK/README/AGENTS/SOPs and one report; accurate local Git commits, remote readback and cleanup evidence. |
| G08 | No premature R00.D closure, no next package promotion, no acceptance self-certification, no claims of unrun tests. `R00.E`–`R00.G` obligations remain reserved for their stages. |

All applicable gates require `PASS` with concrete source lines and checked positive/refusal cases. `FAIL` or `UNVERIFIED` requires a bounded revision or STOP and factual reporting. Tests/build/HTTP/browser execution are **NOT APPLICABLE** to this documentation-only change, not PASSED.

### 6. Permissions and STOP conditions

| Operation | Permission |
| --- | --- |
| Read local Solo files; inspect verified GitHub remote for publication checks | YES |
| Edit permanent files | YES, only the three paths and operations in §2 |
| Stage, commit and push authorized changes on existing `main` to verified `origin/main` | YES, per standing Architect SOP §6B |
| Create branches, clones, linked or shadow worktrees, PRs or extra permanent documents | NO |
| Rewrite history, force-push, merge, tag, release or deploy | NO |
| Install dependencies, implement schemas, source code, assets, modules, tests or toolchain | NO |
| Change ROADMAP/NEXT/OUTLOOK/Owner-owned unrelated files, provider settings or other repositories | NO |
| Read secrets, create new accounts or make payments | NO |

STOP and report if the R02 installation is wrong; baseline/root/branch/remote diverge; other dirty files appear; a correction cannot be made without touching forbidden paths or selecting another Owner decision; the approved first profile conflicts with an earlier accepted contract outside F01–F03; security exposure would result; remote publication is blocked; or necessary evidence cannot be established. Preserve unknown work, do not reset/clean/force and publish a factual current report when safe.

### 7. Required canonical report and final handoff

In **one overwritten** `.documents/Reports/execution-report.md` state the session date; `SOLO-WO-014 R02`; status `COMPLETE | STOPPED | FAILED`; actual local root, branch, origin, one worktree and preflight HEAD; installed ACTIVE bytes/SHA-256/blob; exact three-step commits/trees and changed-file ledger; F01–F03 each with original contradiction, corrected canonical section/lines, positive/refusal cases and agent's documentary `PASS | FAIL | UNVERIFIED`; D01–D12 and O01–O12 regression status; exact D/V/O/P/module/design-reference counts; JSON example parse/structural checks; unchanged-read-only blob inventory; searches for `HTTP`, `loopback`, `fieldOrigins`, `operation`, `durationMs`, `intervalMs`, `initialOpen`, `align`, `position`, `OPEN`; clean-tree and temporary cleanup evidence; any actual failures or unverified items. Runtime tests are NOT APPLICABLE and no Architect `ACCEPT` or R00.D closure may be claimed.

Do not place the report's own final commit SHA in the report. Give the final report-commit SHA and GitHub URL in the chat handback. The Chief Architect will independently fetch the current report from GitHub first and inspect the exact published correction revision.

**End of frozen approved scope — SOLO-WO-014 R02.**
