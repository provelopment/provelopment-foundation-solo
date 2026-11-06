# Foundation Solo — Active Work Order

## SOLO-WO-013 R03 — R00.D Codification Corrections F01–F04

| Field | Exact requirement |
| --- | --- |
| Work order / edition | `SOLO-WO-013` / `R03` — replacement correction edition for the R02 codification, not a new architectural decision series |
| Status | `OWNER-ISSUED` |
| Classification | **Documentation-only correction and independent-review evidence. No application implementation.** |
| Local execution root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — existing local checkout only |
| Installed work order | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / `main` / verified existing `origin` |
| Expected starting `main` HEAD | `34a7df1b669925145755339d004ecd22fb7da9e8` (the R02 report-only commit); confirm local `HEAD`, `origin/main` and remote before editing |
| Baseline codification commit | `38f65e35e63fb25d1fc5e4e02d2f5201c0280065` (R02 documentary result) |
| Exact execution instruction | `execute C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Governing procedures | `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md` (especially §§6A, 6B, 7, 7A, 7B, 8), `.documents/SOP/sop-implementation.md` |

## OWNER-ISSUED APPROVED SCOPE — SOLO-WO-013 R03

### 1. Objective and completion boundary

Correct **exactly four independently identified documentary defects** in the R02 publication of the Owner-accepted R00.D D01–D12 architecture: **F01** contradictory D03 absence wording; **F02** omitted accepted root `schemaVersion` key and stale JSON-format wording; **F03** omitted accepted inline `properties` key; **F04** Product Definition incorrectly treating D06 visibility field placement as undecided. The precise defects and required normative repairs are in §4 below. No new architectural choices are made by this order.

The R02 codification of **D01–D12**, including the Owner's D03 JSON-first amendment, D05 CommonMark/GFM and named versioned trusted Solo modules, the D06 visibility matrix, the remaining R00.A/R00.B/R00.C semantics, all seven design references, all 28 conceptual cases `V01–V28` and all twelve OPEN physical-specification gates `O01–O12` **must be preserved**. Correct the contradictory or incomplete canonical wording in place; do not create a competing addendum, independently authoritative document or new decision. This is a **bounded revision**, not permission to rewrite, augment or finalize the physical schema.

Completion requires: the exact fixes applied to the two existing canonical authorities; all specified targeted positive/refusal cases expressed without contradiction; the unchanged portions and WIP boundaries preserved; authorized local commits pushed to `origin/main`; one current factual GitHub-published canonical execution report; and an evidence map enabling the Chief Architect to independently review F01–F04 and the affected D01/D03/D06/D08 decisions. **No implementation or runtime verification is claimed.**

### 2. Exact edit-path ledger

| Permitted permanent path | Permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Commit the Owner-installed **R03** byte-for-byte as the frozen ACTIVE scope; never edit, clear or annotate it during execution. |
| `.documents/Architecture/ARCHITECTURE.md` | **Only** minimally correct F01, F02 and F03 and, if necessary for parity, clarify D06 direct field placement within the existing §3.2A paragraph. Target §§1.3, 1.5, 2.3 and optionally 3.2A. No other architecture changes. |
| `.documents/About/FOUNDATION-SOLO.md` | **Only** minimally correct F01 in §3 publication declaration and F04 in §4B visibility paragraph. No other product changes. |
| `.documents/Reports/execution-report.md` | Overwrite with the one factual current R03 run report; publish as a report-only commit. |

All other paths are **read-only**, including `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md` (already neutral), `.documents/WIP/plan-outlook.md`, `README.md`, `AGENTS.md`, both SOPs, license, `.github`, all source and assets, and every other repository. Do not create any new permanent file, ADR, decision register, schema, code, test file, plan or report archive. Authorized disposable workspaces are exclusively Solo `.temp/` and the agent's actual Cline-provided task temporary area, governed by the SOP; identify and clean task-owned artifacts.

### 3. Local preflight and execution sequence

1. Work **locally** inside the existing Solo checkout at the path above. Read the installed R03 ACTIVE and the governing local README, AGENTS, both SOPs, Architecture, Product Definition, all four WIP files and current execution report. Verify `git rev-parse --show-toplevel`, branch `main`, exactly one checkout/worktree and one correct `origin`, local `HEAD`, `origin/main`, remote `ls-remote` and `git status --porcelain`. Verify the four-file WIP inventory.
2. The expected initial dirty status is **only** ` M .documents/WIP/workorder-active.md` containing the Owner-installed R03. Treat it as the assigned input, not an unrelated dirty-state failure. Check its ID/edition and record its actual bytes, SHA-256 and Git blob; do not require a checksum from an external handoff or the replaced R02. Stop on unexplained additional dirty paths, wrong root/branch/remote, wrong edition or revision divergence.
3. In the existing local `main`, stage **only** the R03 ACTIVE file and commit/push that preservation step per the existing SOP §6B. Confirm GitHub remote readback. **Do not reset NEXT**: it is already a neutral header, and no new candidate is to be selected.
4. Edit **only** the permitted canonical paragraphs. Apply F01–F04 as defined below with focused wording changes; preserve all adjacent accepted R00.A/B/C and D01–D12 requirements. Run text-level verification of all positive/refusal pairs and check for contradicting statements in both edited files. No additional architectural field names, schema versions, module identities, media profiles or thresholds may be invented.
5. Record exact line-based evidence and diff. Confirm no changes to forbidden paths and that the existing D01–D12 index (12), V01–V28 ledger (28), O01–O12 register (12), seven design URLs, D05 module and Markdown scope, and all accepted route/property/exposure rules remain intact. Commit/push the two canonical-document corrections on `main` and independently read the published versions back from GitHub.
6. Locally overwrite **only** `.documents/Reports/execution-report.md` with the current factual R03 report described in §7; commit/push it separately and independently verify the published report. Do not embed that report's own future commit SHA in its content; supply it in the chat handback. End with a clean working tree and report actual temp cleanup; any blocked publication must be reported truthfully.

GitHub is the publication and subsequent Architect-review destination, **not** the Implementation Agent's editing workspace.

### 4. Four mandatory repairs — complete normative specification

#### F01 — D03 absent candidate is permitted, not automatically refused

**Affected source:** Architecture §1.3, especially the **Source integrity** and **Fail-closed refusal specification** paragraphs (R02 baseline approximately lines 61 and 63); Product Definition §3 **Publication declarations, identity stability and drafts**, approximately line 74.

**Accepted rule to codify verbatim in meaning:** A published representation has one explicitly declared source-selection contract with **at most two** designated candidates, preferred JSON and Markdown. **At least one candidate must be declared.** Exactly one valid effective source is selected. A **genuinely absent preferred JSON candidate is permitted** if a valid declared Markdown candidate can be selected. When valid JSON exists it wins. If JSON exists but is malformed, unsafe, unreadable, inaccessible or otherwise invalid, publication **fails**, regardless of valid Markdown. If neither candidate yields one valid selectable effective source, publication **fails**. Duplicate/ambiguous candidate claims, including claims to absent paths and aliases, **fail**; unselected Markdown content is not merged or used as fallback for invalid existing JSON. A path or declaration being unsafe cannot be treated as source absence. Files outside declared publication never publish.

**Required edit in Architecture §1.3:** Remove/qualify *both* absolute phrases (a) `declared missing, unsafe or ambiguous sources fail validation` and (b) `an absent, duplicate-claimed, unsafe or ambiguous declared source candidate` so the text no longer prohibits absence of an unselected preferred JSON candidate. Replace the reason for refusal with an unsafe/ambiguous/duplicate claim, **invalid or inaccessible selected/preferred resource**, or **failure to select exactly one valid effective source**. Keep cross-representation candidate exclusivity and whole-publication fail-closed validation. Recheck §1.6's seven-case table for consistency, but do **not** change it without a proven conflict.

**Required edit in Product Definition §3:** Replace the generic `explicit source references` / `exactly one permitted authored source` / `missing ... sources ... refused` formulation with a succinct truthful summary: **one explicit source-selection declaration, up to two named candidates, JSON-first, one valid effective source; genuine JSON absence permits valid Markdown; invalid present JSON and no selectable source refuse**. Keep one-manifest authority, drafts, published cardinality, source ownership and identity unchanged. The Product Definition may refer to Architecture §1.6 for details; it must not compete with that authority.

**Targeted conceptual assertions:** (a) JSON candidate declared but absent + valid declared Markdown = **ACCEPT Markdown**; (b) both valid = **ACCEPT JSON only**; (c) JSON present invalid/unreadable + valid Markdown = **REFUSE**; (d) both absent = **REFUSE**; (e) absent candidate also claimed by another representation = **REFUSE duplicate claim**. The word `missing` may appear only where it unambiguously means **no selectable effective source** or an invalid required resource, not an absent nonselected JSON candidate.

#### F02 — D01 root `schemaVersion` is already accepted; do not leave it unnamed

**Affected source:** Architecture §1.5 **The publication manifest — sole authority, encoding and version**, particularly the versioning bullet (R02 approximately line 109), and §1.3's **One authoritative publication declaration** paragraph (approximately line 59).

**Required normative wording:** The sole authoritative publication manifest is directly authored **strict UTF-8 JSON** and has a mandatory root field **`schemaVersion`**. Unsupported version values, unknown fields, duplicate keys and malformed JSON fail closed. `schemaVersion` is the **accepted field name**, not an illustrative suggestion. Its actual initial version **value**, concrete schema artifact and remaining subordinate field layout **remain OPEN**. The manifest schema version is distinct from the Solo product release, JSON content schema, JSON Schema dialect and module contract versions.

**Required edit:** State the literal `schemaVersion` field name in §1.5; amend §1.3 to avoid saying that the manifest's physical **file format** remains undecided. The manifest's **filename/location**, exact subfield layout, initial `schemaVersion` value, source locator serialization and JSON Schema implementation/dialect remain open as listed in §6.2. Keep the one-Entity, sole-publication-authority and strict-refusal contract unchanged.

**Targeted conceptual assertions:** (a) root `schemaVersion` is **mandatory**; (b) missing or unsupported `schemaVersion` **REFUSES publication**; (c) no particular version value, filename or schema dialect is silently selected; (d) the single-manifest strict JSON format is settled.

#### F03 — D08 accepted inline object key is `properties`

**Affected source:** Architecture §2.3 **Declared property encoding direction** (R02 approximately line 221).

**Required normative wording:** On **each** of Entity, Facet, Site, Page and localized representation, the optional **directly inline `properties` object** contains declarations keyed by registered property identity. The object is attached to its EFS owner, not to a central authoritative property ledger or a separate publishing structure. A wholly absent property declaration inherits from immediate ancestry. When a property is declared it has the accepted **independent elements** `inheritance` (`inherit` or `suppress`) and `currentContext` (`none` or typed ordered local values). Inherited-first/local-second ordering, effective cardinality, registered definitions, per-value provenance and public-only projection are preserved. Unknown properties and incompatible definitions refuse publication.

**Required edit:** Name the **literal** `properties` object and its **direct placement on each EFS node** in the opening paragraph of §2.3. Do **not** infer the unresolved registry catalogue, value envelopes, exact local-value serialization or exposure enumeration. Keep §2's R00.C additive semantics intact.

**Targeted conceptual assertions:** (a) `properties` is an approved exact key, not merely a description of some unnamed container; (b) an omitted property inherits; (c) explicit suppress with valid local values is allowed; (d) an inherited singular value plus another local value without suppression is **REFUSED**, not silently overwritten.

#### F04 — D06 direct visibility-field placement is decided, not OPEN

**Affected source:** Product Definition §4B **Public and unlisted Pages**, first Owner-accepted granularity paragraph (R02 approximately line 118), and Architecture §3.2A only if necessary to eliminate ambiguity.

**Required normative wording:** Optional **`visibilityDefault` is a direct inline field of the Page declaration**, defaulting to `public`; optional **`visibility` is a direct inline field of each localized representation declaration** and may explicitly override the Page's default, including `unlisted` → `public`. Both fields are in the single authoritative publication manifest, **not** content JSON, Markdown, a central override table or a `publication` wrapper. Only `public` and `unlisted` are valid states. The four-case effective-visibility matrix, exact-case unlisted direct serving, `noindex, nofollow`, complete automatic-discovery exclusion, unlisted not being private and no unlisted fallback remain binding.

**Required edit:** Remove Product §4B's statement that **field placement** remains open. Say only the **full schema encoding and validation mechanics** remain open. If Architecture §3.2A is not already explicit enough about the direct inline placement, add one short sentence there; do not change its visibility policy or matrix.

**Targeted conceptual assertions:** (a) absent Page/representation fields → `public`; (b) Page unlisted and representation omitted → `unlisted`; (c) Page unlisted and representation explicit public → `public`; (d) no Page-level visibility may be independently declared in a body source; (e) unlisted is published but undiscoverable, not private.

### 5. Preservation and document-quality checks

**Mandatory exact preservation:** Keep all other D01–D12 accepted behavior and the R00.A/B/C logical contracts unchanged. In particular, do not change the accepted ID grammar or scoping; JSON-first selection; CommonMark/GFM and no Solo Markdown; advanced named/versioned modules and strict Hexagonal Architecture; seven design reference URLs; routing, full locales, case/Unicode and collisions; all four visibility outcomes; additive properties and public projection; location `add`/`extend`; graphic roles and typed namespaces with zero automatic fallback; safe relative resource resolution; atomic whole-publication validation; and excluded advanced contact-form subsystem. Neither produce nor finalize implementation schemas, literal file/path/version values, module catalogues or registry limits through this correction.

**Mandatory structural evidence:** The Architecture Charter retains the `D01–D12` 12-row decision index, `V01–V28` 28-row conceptual ledger, `O01–O12` 12 OPEN gates and all seven design reference URLs. All existing external references and internal links remain intact. Ensure no redundant, contradictory or residual unqualified language survives the edit: search both documents for `missing`, `absent`, `source reference`, `source candidate`, `schemaVersion`, `properties`, `field placement`, `visibilityDefault`, `visibility` and inspect every relevant hit in context rather than trusting a string count. Provide a targeted diff audit showing no unrelated rewrites.

**Do not misstate readiness:** These are corrections to published architecture documentation. The 12 physical specification gates **remain OPEN**; the corrected Charter is not an implementation-ready schema. `R00.D` stays in ROADMAP. NEXT stays neutral. ACTIVE remains R03. No Architect sign-off, package closure, Stage 00 completion, code, tests, release or deployment is included in this order.

### 6. Permissions and prohibited operations

| Operation | Authorized? |
| --- | --- |
| Read local Solo project files, inspect Git/remote and read back GitHub commits | **YES** |
| Modify local permanent files | **YES: only §2 paths and stated operations** |
| Stage/commit/push authorized docs on existing `main` to verified `origin/main` | **YES**, following Architect SOP §6B and branch policy |
| Reset/modify NEXT or ROADMAP; select a new candidate; clear ACTIVE | **NO** |
| Additional branch, checkout, clone, worktree, submodule or shadow repository | **NO** |
| PR creation, merge, force-push, rewrite history, tag, release, deployment | **NO** |
| Application code, schema artifact creation, dependency install, framework/runtime/build implementation | **NO** |
| Secret/provider/account/DNS/TLS/payment changes, external writes or other repositories | **NO** |
| Automatic cleanup of unknown Owner-owned files or unsafe destructive commands | **NO** |
| One local report overwrite and report-only GitHub commit | **YES**, as §3 requires |

No unlisted operation is authorized. Follow the SOP's normal Git mechanics without asking the Owner to execute routine commands. Respect actual branch protections and STOP rather than bypassing them.

### 7. Mandatory report and independent handoff evidence

Overwriting `.documents/Reports/execution-report.md` produces the **only current report** for this run. It must include:

- `SOLO-WO-013 R03`, run date, status `COMPLETE | STOPPED | FAILED`, local root, verified branch/remote, initial and ending HEAD; preflight one-worktree/one-origin and expected Owner-installed ACTIVE dirty state; installed workorder SHA-256/blob.
- Separate local commit/push evidence for **(A)** preserved ACTIVE, **(B)** precise Charter/Product fixes, and **(C)** report-only publication. Include commit IDs/tree identifiers, changed-file ledger and remote readback.
- **F01–F04**, each with the exact canonical file/section and final line spans, quoted positive-and-refusal semantics, documentary `PASS | FAIL | UNVERIFIED` and direct source evidence.
- Individual documentary regression rechecks for affected **D01, D03, D06, D08** plus confirmation that D02, D04, D05, D07, D09, D10, D11 and D12 material was not modified and their canonical anchors remain present. Do not impersonate the independent Chief Architect sign-off.
- The count and preservation of D01–D12, V01–V28, O01–O12 and seven design references; targeted contradiction search findings; preservation blobs for ROADMAP, NEXT, OUTLOOK, README, AGENTS and SOPs.
- Actual local checks performed and outputs. Tests/builds/HTTP/browser proof **NOT APPLICABLE** (documentation-only repository); never claim unrun tests PASS.
- Clean-tree check, temporary-space inventory and task-owned cleanup details, exact stop/blocker if any, unauthorized-operation avoidance and the precise resumption point for Architect review.

The report must not self-reference its own publication commit; report that commit in the final **chat handback** along with the GitHub URLs for the complete current report and the two content commits. The Chief Architect's review begins by fetching the current GitHub report and independently verifying the exact published revision and four corrections.

### 8. Stop conditions

STOP and report the factual state if: R03 is not installed correctly; an unexpected dirty file or remote divergence appears; repository root, branch or origin mismatches; completing a repair requires modifying forbidden paths or deciding an OPEN physical schema; an accepted R00.A/B/C/D contract conflicts in a way not covered by F01–F04; a destructive change or secret exposure is needed; GitHub publication is blocked; or proof of an applicable requirement cannot be produced. Do not conceal a failure by claiming the documentation passes. Preserve unknown or Owner-owned material. Even on failure, publish a truthful current canonical report when safe under the SOP.

**End of frozen approved scope — SOLO-WO-013 R03.**
