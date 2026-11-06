# Foundation Solo — Active Work Order

## SOLO-WO-017 R01 — R00.E Administrative Closure and R00.F NEXT Selection

| Field | Exact requirement |
| --- | --- |
| Work order / edition | **`SOLO-WO-017` / `R01`** |
| Issuance marker | **`OWNER-ISSUED`**, effective only when the Owner installs this complete edition in the existing ACTIVE slot and delivers the matching Owner execution handoff |
| Classification | **Stage 00 administrative documentation transition only**; no application, toolchain, schema, test, asset, release or deployment activity |
| Target repository | `provelopment/provelopment-foundation-solo`, existing local `main`, verified one `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo.git` |
| Authorized local execution root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — the **existing** Solo checkout only |
| Installed order file | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Normal local instruction | `execute C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Required published starting HEAD | `01ba66770701b56922920cf53ffb57589a867e6a` on `origin/main`; verify live remote and local match before edits |
| Accepted content revision | `4785ae0d4ea397e672cf8ad09b2df0631218644d` — `SOLO-WO-016 R02` Architecture Charter §6A codification |
| Accepted report revision | `01ba66770701b56922920cf53ffb57589a867e6a` — Chief Architect independently reviewed this exact GitHub revision and issued **ACCEPT of documentary codification** |
| Runtime evidence limit | C1 parser/schema/Unicode/Markdown conformance, C2 delivery behavior, C3 import and Rust/TS boundary tests and C4 executable module/refusal tests remain **UNVERIFIED, NOT RUN**, and blocking future code-bearing acceptance; documentary closure does not waive these gates |
| Governing authority | Root `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md` especially §§6–8, `.documents/SOP/sop-implementation.md` especially §§4–11, canonical Charter/Product Definition, all four WIP slots and canonical GitHub execution report |

## OWNER-ISSUED APPROVED SCOPE — SOLO-WO-017 R01

### 1. Objective, accepted basis and completion boundary

Close **only the independently accepted R00.E software structure and toolchain Stage 00 decision package** administratively, then select **R00.F — Contact and integrations** as the one immediate NEXT candidate under Architect SOP §8. The R00.E selection is already permanently recorded in `.documents/Architecture/ARCHITECTURE.md` §6A as `E01`–`E10`, preserving all accepted `D01`–`D12`, `O01`–`O12`, `P01`–`P14`, and `V01`–`V28`. The Owner has explicitly approved the Chief Architect's recommended architectural preferences; the independent review of published `SOLO-WO-016 R02` at `01ba66770701b56922920cf53ffb57589a867e6a` returned **ACCEPT for documentary codification**, not for runtime behavior.

The target live state after this transition is **exactly**: ROADMAP's Stage 00 retains R00.F, R00.G and its existing exit gate but no R00.E pending-package line; NEXT contains exactly the §4 bounded R00.F candidate; ACTIVE contains only its one-line neutral identifying header; OUTLOOK remains unchanged; the canonical single execution report is overwritten with this closure run. The accepted Charter and Product Definition remain byte-for-byte unchanged, and the current issued scope is retained in Git history.

**Closure limits:** This does not close Stage 00, accept R00.F or R00.G, issue their work orders, validate a parser, prove Unicode or module behavior, build an application, adopt a provider, approve a release or deployment, or waive any `C1`–`C4` or future runtime acceptance gate. The earlier `SOLO-WO-016 R02` report's documentary acceptance is the only accepted R00.E scope consumed here.

### 2. Exhaustive permitted permanent edits and preservation ledger

| Exact path | Authorized change |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **First** preserve and publish the **entire Owner-installed `SOLO-WO-017 R01` edition byte-for-byte** in a separate frozen-order commit. **Then**, in the WIP closure transition commit, replace it with exactly `# Foundation Solo — Active Work Order` and a single LF, no blank line, acceptance log, other order or text. |
| `.documents/WIP/plan-roadmap.md` | **Remove one line only:** the exact complete Stage 00 R00.E remaining-package line reproduced in §3. Preserve every other byte, line and order, including Stage 00 heading and outcome, R00.F/R00.G, Stage 00 exit gate and all later Stage 01–13 packages. Do not insert a completion paragraph or history. |
| `.documents/WIP/plan-next.md` | Replace the baseline neutral title-only file with the **exact** complete R00.F candidate of §4, UTF-8 with LF line endings and one final LF. No second candidate, work order or authorization text. |
| `.documents/Reports/execution-report.md` | Overwrite the single canonical report with a factual `SOLO-WO-017 R01` report, publish separately **after** the WIP transition, and verify the remote readback. No parallel report. |

**Read-only / preserved:** `.documents/Architecture/ARCHITECTURE.md` (accepted Charter blob `7b6414659e293e6f196f9750544f1434b0cab34a`), `.documents/About/FOUNDATION-SOLO.md` (blob `5e7a9a80f6f477e1e17d06074224a2a27fa2603e`), `.documents/WIP/plan-outlook.md` (blob `cb9c7a610cf3c537557e060ca9656e44a053e69d`), `README.md`, `AGENTS.md`, both SOP files, root license/hygiene files, `.github/**` and **all other paths and repositories**. Do not modify R00.D keys, E01–E10, diagrams, normative tables, accepted examples, modules or any source file. WIP contains exactly the four retained files; do not create a subdirectory, extra WIP file, ADR, archive, report copy or evidence ledger.

The three pre-existing prose mismatches disclosed in Charter §6A.16 (ROADMAP's graphic fallback/source-discovery language and the Charter §6.5 package-label split) are **not** authorized edits in this closure order. R00.F NEXT follows the ROADMAP's explicit `R00.F — Contact and integrations` label; any separate Charter/ROADMAP reconciliation requires its own exact Owner-authorized scope. Do not silently rewrite previous accepted contracts or the Stage 00 plan to resolve a mismatch.

### 3. Mandatory baseline preflight and exact administrative transition

1. Work **only inside the existing Owner-local Solo checkout**, not by editing GitHub-hosted files. Read the installed entire `SOLO-WO-017 R01`, `README.md`, `AGENTS.md`, the Product Definition, Architecture Charter (including §6A), both SOPs (Architect SOP §§6A, 6B, 7, 7A, 7B, 8), all four WIP files and the **current** execution report. Verify the actual physical root, branch `main`, single worktree, one correct `origin`, upstream, `HEAD`, `origin/main` and `git ls-remote` all equal `01ba66770701b56922920cf53ffb57589a867e6a`. Verify the four-file WIP inventory and actual worktree status.
2. The **only expected local pre-run modification** is the Owner-installed new ACTIVE work order. Read its whole content, ID/edition and `OWNER-ISSUED` marker, verify it matches the Owner's handoff and record its byte count, SHA-256 and Git blob. Confirm the baseline committed ACTIVE blob is `f4c9cdeb45e5fa5e75f6bc22f161cdd5c11e52b8` (`SOLO-WO-016 R02`); the Owner-installed `SOLO-WO-017 R01` replaces that locally. An unrelated dirty file, unknown revision, extra checkout, unexpected remote or altered edition **STOPs**; never reset or discard unknown data.
3. Verify **before any write** that ROADMAP blob is `ab382665a82ce9a280b47f5561dc1b0c36542667`, NEXT blob is `92fa562b1af0c54391851aa5c79deb1631127069` and contains exactly `# Foundation Solo — plan-next (Next Candidate)` plus one LF, OUTLOOK and both canonical blobs are those listed above, and the canonical report blob at baseline is `32d6e37ff025461c1aa3bbd2302e09940bb7271b`. Check that ROADMAP has **exactly one** target R00.E bullet, retains R00.F/R00.G and Stage 00's exit gate, and all 14 stage headings (Stage 00–13) are present. A changed or ambiguous baseline is a STOP, not permission to guess.

   The **only ROADMAP text authorized for removal** is exactly this one existing complete line and its terminating LF (copy it as written; do not match a broader paragraph or neighboring line):

   ```text
   - **R00.E — Software structure and toolchain:** compare credible framework/language/build/testing choices, choose simplest verifiably strict Ports-and-Adapters design, dependency/port catalogue, composition root, boundary test method, CI and browser proof architecture. Do not inherit Foundation's Next.js/pnpm choice by default.
   ```
4. **Frozen-order publication:** stage **only** `.documents/WIP/workorder-active.md`, commit the complete Owner-installed `SOLO-WO-017 R01` without changing its text, and push from the verified local `main` to existing `origin/main` per Architect SOP §6B. Independently read back the GitHub commit and confirm the installed work order is byte-identical to the published frozen blob. Record commit and tree identities. Do not embed a mutable approval or evidence section in the frozen order.
5. **Closure transition:** remove the one ROADMAP R00.E line reproduced verbatim in §3, write §4 NEXT in full, and replace ACTIVE with its exact one-line neutral header. Verify the remainder of ROADMAP byte-for-byte against the original minus that one line; verify NEXT exactness, ACTIVE exactness, unchanged OUTLOOK, unchanged Charter and Product Definition, and four-file WIP purity. No other work. Stage **only** the three authorized WIP files, commit and push this transition, read back the remote blobs, and verify the frozen issued WO017 is retained in the immediately prior Git commit.
6. **Canonical report:** overwrite `.documents/Reports/execution-report.md` with the factual report specified in §6, stage **that file alone**, commit and push, then read the actual canonical report from GitHub to prove publication. State the preceding known WIP transition commit in the report, **not** the report commit's own future SHA, which is self-referential. Supply report-commit SHA and GitHub URL in the final chat handoff.
7. Finish with `git status --porcelain` empty, exactly one Solo worktree, one correct remote and upstream. Verify no unauthorized edits or tracked scratch. Identify and clean task-owned material in Solo `.temp/` and actual Cline-provided task workspace, and report any residual or impossible-to-verify cleanup truthfully. Preserve all unfamiliar/Owner-owned files.

No command needs separate Owner approval when already authorized by this complete order. Follow the single Git procedure in Architect SOP §6B; do not duplicate it with a competing process.

### 4. Exact NEXT content — R00.F decisions-only candidate

Write `.documents/WIP/plan-next.md` with the following exact Markdown, preserving the blank lines shown and ending in **one** LF, with no preceding or trailing extraneous text:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R00.F — Contact and integrations

**Objective:** Settle the complete Owner-reviewable first-release contract for safe `mailto:` actions, the included simple contact form, and tightly bounded external booking/directions links and optional privacy-respecting analytics, while preserving Foundation Solo's one-Entity and strict Ports-and-Adapters architecture.

**Prerequisites and readiness:** R00.A–R00.D's accepted physical publication contracts and R00.E's selected software structure/toolchain are canonically recorded; R00.E has been independently accepted for documentary codification and administratively closed by the authorized transition. The ROADMAP's R00.F contact-and-integrations package is the earliest unfinished Stage 00 package. The accepted `contact.email` property, typed safe-link contracts and reserved delivery port are implementation boundaries, not completed form/provider policy. R00.G lifecycle and nonfunctional standards remain pending.

**Proposed scope:** Compare credible minimal contact-delivery, consent, abuse-prevention and privacy approaches. Define an exact simple form field set, input limits, validation and accessible states, submissions and truthful receipt/failure semantics, a provider-neutral outbound delivery port, credential isolation, disabled/no-provider behavior, rate and anti-abuse safeguards, consent, data minimization, retention/deletion and safe diagnostics. Preserve the already accepted `mailto:` address and link safety contract. Decide the bounded scope for authored static booking/directions actions and whether privacy-respecting analytics is optional or omitted in the first release, without building booking inventory, location inference, transactions or visitor accounts. Distinguish any later R00.G lifecycle/performance/security targets and any deployment-provider configuration from this decisions-only package.

**Proposed deliverables:** Complete alternative comparison and Chief Architect recommendations; exact Owner decisions; precise and non-contradictory amendments to the existing canonical Architecture Charter and, only if genuinely needed, the Product Definition; typed inbound/outbound contracts and transport/provider boundaries; positive, negative, privacy, accessibility and genuine delivery-evidence verification matrix. Prepare the complete separate work order and matching local execution prompt for Owner assessment. Do not create form code, providers, dependencies, schemas, test suites, CI workflows or deployment in Stage 00.

**Verifiable exit conditions:** The Owner accepts a deterministic and testable simple contact and integration contract with truthful outcomes, safe `mailto:`, concrete data and abuse/privacy rules, a replaceable delivery adapter, explicit provider-absence behavior and bounded external actions/analytics policy. No advanced forms, attachments, CRM/ticketing, form builder, account system, booking engine or hidden provider dependency is introduced. The accepted R00.A–R00.E contracts remain intact; executable proof belongs to later code-bearing packages. R00.G and the Stage 00 exit gate remain outstanding.
```

This NEXT candidate is a **non-authorizing proposal**. It does not itself select providers or contact-policy outcomes, modify the Charter, or create implementation permission. The label `R00.F` follows the ROADMAP; the pre-existing §6.5 package-label mismatch is disclosed, not repaired here.

### 5. Mandatory closure acceptance and refusal gates

| Gate | Acceptance condition and proof |
| --- | --- |
| **G01 — Authority / preflight** | Exact Owner-installed `SOLO-WO-017 R01`, proper matching handoff, correct root/branch/one worktree/one origin, baseline HEAD/remote match, four-file WIP, only expected initial ACTIVE modification, recorded actual SHA-256/blob, unchanged accepted Charter/Product blobs. If not, STOP. |
| **G02 — Frozen order** | Initial commit publishes the whole installed approved scope byte-for-byte, separately from closure, with remote readback; no prior R02 or newly installed order silently overwritten without historical retention. |
| **G03 — ROADMAP purity** | Delete **exactly** the sole §3 R00.E bullet and LF; preserve Stage 00 outcome, remaining R00.F and R00.G lines and exit gate; preserve every Stage 01–13 heading, package and text byte-for-byte; add no completed summary. Negative: a second deletion, a modified unrelated byte or R00.E remaining is FAIL. |
| **G04 — NEXT readiness** | Exact §4 R00.F candidate and one terminal LF, precisely one candidate with no executable authority, first dependency-ready ROADMAP package; R00.G not promoted. Negative: a second candidate, unauthorized policy choice, or change to candidate text is FAIL. |
| **G05 — ACTIVE neutral** | Final live ACTIVE is exactly `# Foundation Solo — Active Work Order\n` (one LF and no other characters), and frozen WO017 edition is confirmed in Git history. Negative: leftover issuance, completed-order history or alternative active order is FAIL. |
| **G06 — Canonical preservation** | Charter blob `7b6414659e293e6f196f9750544f1434b0cab34a` and Product blob `5e7a9a80f6f477e1e17d06074224a2a27fa2603e` unchanged; E01–E10, D/O and C1–C4 future runtime holds not altered; OUTLOOK and all unlisted paths unchanged. Negative: source code, dependency, generated contract, test, policy or module change is FAIL/STOP. |
| **G07 — Git and report** | Only permitted local commits and remote pushes, exact path and commit/tree ledger, published frozen order and WIP readback, canonical overwritten report as a separate report-only commit, no other repo or operation; final clean-tree, one-worktree and scratch-cleanup evidence. |
| **G08 — Correct classification** | R00.E documentary package alone administratively consumed; C1–C4 executable proof remains `UNVERIFIED`/future blocking, no faux runtime acceptance; R00.F/R00.G and Stage 00 stay open; no implementation, provider, release, merge or deployment claim. Negative: Stage 00 closure, waiver of C1 or agent self-acceptance is STOP. |

Record each gate as **PASS / FAIL / NOT RUN** with concrete Git/readback or file-diff evidence; a claim alone is not proof. Fail closed on ambiguity or unexpected state. Executable parser/schema/Unicode, browser, build or HTTP tests are **NOT APPLICABLE to this administrative transition**, never PASS.

### 6. Permissions, STOP rules, report and handoff

**Authorized:** read required Solo files; perform the exact four permanent-path operations, ordinary Git stage/commit/push/readback on existing `main`/`origin` under Architect SOP §6B, and ordinary task-owned scratch and safe cleanup under Architect SOP §6A and Implementation SOP §5A. No dependency installation, network/provider action other than the required GitHub Git publication and verification, source creation, schema generation, CI editing, branch/PR/merge, release, deployment, DNS, TLS, secret access, destructive reset, `git clean`, force push, history rewrite, second clone/worktree, or other repository writes. No parent `.gitignore` modification.

**STOP without destructive repair if**: local root, branch, remote, baseline or active edition mismatches; WIP inventory is not exactly four; an unrelated or unknown dirty path exists; ROADMAP has none or more than one matching target line; NEXT is not neutral; a canonical blob differs from the baseline; the expected preflight or matching Owner issuance cannot be established; a push would overwrite advanced remote history; the approved scope cannot be preserved; an unauthorized extra write or materially conflicting instruction would be required. Preserve state; publish a factual `STOPPED` report if safe under SOP and tell the Owner exactly what prevents execution. Never turn this into an R00.F implementation run.

**Canonical execution report** must contain: ISO session date, `SOLO-WO-017 R01`, `COMPLETE | STOPPED | FAILED`; starting and actual final roots, remote, branch, worktree count and commits; prior `SOLO-WO-016 R02` ACCEPT reference and the unexecuted C1–C4 status; frozen ACTIVE edition SHA-256/blob and commit; exact pre/post WIP files and blobs; single ROADMAP deleted line and byte/line remainder proof; character-exact NEXT and ACTIVE verification; unchanged Charter/Product/OUTLOOK and all other path preservation; G01–G08 proof dispositions; every command or action and actual result; changed-path ledger, push/readback links, warning/skip/limitation inventory, temp ownership and cleanup, final worktree state and separate report publication check. Do **not** self-assign Chief Architect `ACCEPT`, claim test execution, expose secrets or record the report's own commit ID in itself. Report-commit SHA belongs in the **chat** handoff only.

**Independent review:** When the Owner later says the closure report is ready, the Chief Architect's **first action** is to fetch `.documents/Reports/execution-report.md` from GitHub, then independently verify the exact final report revision, the commits and changed-file boundaries, the WIP bytes, final authoritative Charter and Product blobs and every G01–G08 gate. Only then issue **ACCEPT / REVISE / STOP** against that reviewed revision. Acceptance of this closure does not accept R00.F, R00.G, Stage 00 or executable code.

**End of approved scope for `SOLO-WO-017 R01`.**
