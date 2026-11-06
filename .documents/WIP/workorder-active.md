# Foundation Solo — Active Work Order

## SOLO-WO-015 R01 — R00.D Administrative Closure and R00.E NEXT Selection

| Field | Exact requirement |
| --- | --- |
| Work order / edition | `SOLO-WO-015` / `R01` |
| Status | `OWNER-ISSUED` |
| Classification | **Stage 00 administrative documentation transition only** — close the independently accepted R00.D package and select the next R00.E decision candidate; no application or schema implementation |
| Local execution root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — existing checkout only |
| Installed task file | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Execution instruction | `execute C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md` |
| Repository, branch, remote | `provelopment/provelopment-foundation-solo`; existing local `main`; verified existing `origin` |
| Required starting remote/main HEAD | `126d8fa64334d77e039061aa79a182e3ad7173d9` — R03 report-only publication; independently verify local HEAD, `origin/main`, and `git ls-remote` |
| Accepted R00.D codification | `D01`–`D12` and `O01`–`O12` documentary acceptance by the Chief Architect at the exact starting revision above; final Charter blob `0e82502540383defb9f3a619ef43c46700ca7159`, Product Definition blob `5e7a9a80f6f477e1e17d06074224a2a27fa2603e` |
| Governing local authorities | `README.md`, `AGENTS.md`, `.documents/SOP/sop-architect.md` (especially §§6A, 6B, 7, 7A, 8), `.documents/SOP/sop-implementation.md`, current ROADMAP/NEXT/ACTIVE/OUTLOOK and canonical report |

## OWNER-ISSUED APPROVED SCOPE — SOLO-WO-015 R01

### 1. Objective and completion boundary

Perform the **administrative closure of R00.D only**, following the Chief Architect's independent `ACCEPT` of `SOLO-WO-014 R03` at `126d8fa64334d77e039061aa79a182e3ad7173d9`. This order carries out the four-slot WIP lifecycle in Architect SOP §8. The accepted R00.D architectural and physical contracts are **already permanently codified** in the Architecture Charter and Product Definition; this order does not revise them, repeat their decisions, or implement them in code.

The resulting live state is exactly: `R00.D` no longer appears as a remaining ROADMAP package; Stage 00 remains with R00.E, R00.F, R00.G and its exit gate; NEXT contains **one** bounded R00.E decisions-only candidate; ACTIVE contains only its neutral identifying title; OUTLOOK is unchanged. The single canonical execution report records this run. Git history retains this frozen issued work order and every prior R00.D work order/report revision.

The R00.D closure **does not** close Stage 00, authorize R00.E decisions as already accepted, authorize application or schema source work, release software, deploy, choose a toolchain, or issue a further work order.

### 2. Exhaustive permanent edit-path ledger

| Path | Exact permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **First** preserve and commit/push the Owner-installed `SOLO-WO-015 R01` byte-for-byte as the frozen issued scope. **Then**, in the closure transition commit, replace its entire content with exactly `# Foundation Solo — Active Work Order` followed by one LF (UTF-8). No history, status essay, accepted-work summary or other content remains in LIVE ACTIVE. |
| `.documents/WIP/plan-roadmap.md` | Remove **only** the single Stage 00 remaining-package list item beginning `- **R00.D — Content and publication contract:**` including its full line. Preserve all other bytes/lines and their order, including Stage 00 outcome, R00.E, R00.F, R00.G, Stage 00 exit gate and every subsequent stage/package. Do not insert a completed-work or acceptance-history record. |
| `.documents/WIP/plan-next.md` | Replace the existing neutral header-only content with exactly the bounded R00.E candidate in §4 below. No second candidate, work order, authority paragraph or decision history. |
| `.documents/Reports/execution-report.md` | Overwrite the one canonical report with the factual `SOLO-WO-015 R01` administrative closure report specified in §6 and publish in a **separate report-only commit**. |

**Read-only permanent paths:** `.documents/Architecture/ARCHITECTURE.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/WIP/plan-outlook.md`, `README.md`, `AGENTS.md`, both SOPs, `.github/**`, source, tests, assets, generated output, contracts and every other repository. Do not create any extra WIP file, log, archive, report, ADR, artifact or separate decision authority. Temporary work follows only the existing Solo `.temp/` and actual agent-provided temporary workspace rules in the SOP; preserve Owner-owned/unknown files.

### 3. Local preflight and exact execution sequence

1. Work **inside the existing local Solo checkout**, not in a GitHub editing workspace. Read the installed ACTIVE R01 in full and all governing local files, including current GitHub-published report as available from local tracking and remote verification. Verify the exact repository root, `main`, a single worktree, one correct `origin`, the four allowed WIP files, `HEAD = origin/main = ls-remote = 126d8fa64334d77e039061aa79a182e3ad7173d9`, and `git status --porcelain`.
2. Expect **only** the Owner-installed change ` M .documents/WIP/workorder-active.md` as the initial local dirty path. This is the assigned input, **not** a blocker. Verify `SOLO-WO-015 R01`, its full contents, actual SHA-256 and Git blob before editing. STOP on any unrelated dirty file, unknown revision difference, wrong checkout/remote, or missing/mismatched edition. Verify the baseline Charter and Product blob IDs above. Confirm ROADMAP still contains exactly one R00.D remaining-package item, NEXT is exactly `# Foundation Solo — plan-next (Next Candidate)` plus one LF, and Stage 00 still contains R00.E/F/G.
3. Stage **only** `.documents/WIP/workorder-active.md`; commit and push the installed R01 byte-for-byte using the existing local Git procedure (Architect SOP §6B). Verify the published GitHub commit and the ACTIVE blob are the same content before proceeding.
4. Apply **exactly** the three §2 WIP transition edits: remove the sole R00.D ROADMAP bullet (not the Stage), install §4's R00.E NEXT candidate, and clear ACTIVE to the exact neutral header. No edits in Architecture, Product Definition, OUTLOOK or any other path. Inspect a focused diff and compare all non-target ROADMAP lines byte-for-byte.
5. Verify all §5 conditions, then commit and push **only** those three WIP transitions from local `main` to the already verified `origin/main` using the SOP Git procedure. Read back actual remote tree/file contents; confirm the frozen work order remains accessible in the immediately preceding commit. No branch, PR, merge, force push, rewrite, tag, release or deployment.
6. Overwrite `.documents/Reports/execution-report.md` **locally** with the current factual closure report, commit and push **that file alone**, and read back the published report. Do not put the report commit's own future SHA inside the report; provide that SHA in the final chat handback. End with an honest worktree and task-owned temporary cleanup account.

### 4. Exact NEXT content — one R00.E candidate

Write `.documents/WIP/plan-next.md` as **exactly** the following UTF-8/LF text, with one terminal LF and no text before or after:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R00.E — Software structure and toolchain

**Objective:** Establish an Owner-reviewable, exact and verifiably enforceable Solo software-structure and toolchain contract that will implement the accepted R00.A–R00.D architecture through strict Hexagonal Architecture (Ports and Adapters), without silently importing another Foundation project's framework, build system or runtime assumptions.

**Prerequisites and readiness:** R00.A–R00.D are architecturally accepted and canonically codified; R00.D is administratively closed by the issued closure transition. R00.E is the earliest unfinished Stage 00 package. The current repository has no Solo application implementation; language, framework, runtime, dependency selection and development toolchain remain decisions to obtain, not conventions to assume.

**Proposed scope:** Compare credible language, framework, build, package management, test runner and CI options on explicit portability, maintainability, safety, accessibility and evidence criteria. Define inward-only domain/application dependencies, required ports, replaceable adapters, a single composition root, source/contract generation boundaries, dependency-version pinning and supply-chain policy. Choose approaches that can actually enforce the accepted JSON Schema Draft 2020-12, CommonMark/GFM, Unicode 18.0.0, route and source safety, atomic publication and registered module contracts. Specify real unit, contract, integration, import-boundary and production-mode/browser proof architecture, including checks against aliases/re-exports and negative cases. Keep R00.F contact/integration policy, R00.G lifecycle/nonfunctional acceptance, provider deployment and any code-bearing implementation outside this package.

**Proposed deliverables:** A Chief Architect decision dossier with credible options, tradeoffs and clear recommendations; explicit Owner decisions for every blocking toolchain and structural choice; precise accepted amendments to the existing Architecture Charter and, only where genuinely needed, Product Definition; an implementable dependency/port/adapter/validator/testing matrix and refusal-evidence plan. Do not create application code, dependencies, executable schemas, CI workflows or alternate documentation authorities during Stage 00 decisions.

**Verifiable exit conditions:** The Owner has accepted one reproducible, pinned, accessible and portable software/toolchain architecture; every required foundation contract maps to an explicit port/adapter/validator and test boundary; Hexagonal Architecture enforcement and real failure-path proof are specified; no accepted D01–D12 or O01–O12 semantics have been revised by convenience; undecided R00.F/R00.G and later-stage matters are explicitly reserved. Stage 00 remains open for R00.F, R00.G and its exit gate.
```

### 5. Mandatory acceptance and refusal gates

| Gate | PASS requires direct evidence |
| --- | --- |
| G01 — Authority/baseline | Actual local preflight verifies `SOLO-WO-015 R01`, correct Solo root, one worktree/remote, exact beginning revision and only expected Owner-installed ACTIVE change; record actual SHA-256/blob. |
| G02 — Frozen scope | Initial commit preserves the Owner-installed complete R01 byte-for-byte; remote GitHub readback proves it. |
| G03 — ROADMAP purity | Exactly one R00.D remaining-package line is removed; Stage 00 heading/outcome, R00.E/F/G, exit gate and **all** later stages and package lines are unchanged, in order. No completed-work log added. |
| G04 — NEXT readiness | NEXT exactly matches the §4 content, is one bounded R00.E decision candidate, reflects the dependency-ready earliest remaining package, and does not preselect a toolchain or issue code implementation. |
| G05 — ACTIVE closure | Final ACTIVE is **exactly** `# Foundation Solo — Active Work Order\n` (one LF, no hidden second active order), while Git history preserves the issued `SOLO-WO-015 R01` edition. |
| G06 — Canonical preservation | Architecture Charter and Product Definition Git blobs unchanged; all D01–D12 and O01–O12 accepted contracts remain canonically recorded. OUTLOOK, README, AGENTS, SOPs, other paths and WIP four-file inventory are unchanged. |
| G07 — Git/report evidence | Permitted local A/B/C commits/pushes only, correct changed-path ledger and GitHub readback; canonical R01 report published as report-only commit; truthful final worktree/temp inventory. |
| G08 — No premature work | R00.D alone is consumed; Stage 00 remains open, R00.E/F/G undecided, no application implementation, schema artifacts, dependency installation, test-run claims, release, deployment or self-awarded Architect acceptance. |

Refusal examples: a second unrecognized dirty path, a changed remote revision, an absent or duplicated R00.D ROADMAP line, a non-neutral initial NEXT, different canonical blob, unintended alteration of R00.E/F/G, an incomplete NEXT replacement or any attempt to edit Architecture/Product **must STOP** and produce a factual current report when safe. The agent shall not silently repair unrelated deviations or choose the missing architecture itself.

### 6. Permissions, report and resumption evidence

| Operation | Permission |
| --- | --- |
| Read local Solo files, Git state, verified existing remote | **YES** |
| Modify the four §2 permanent paths, only with the specified operations | **YES** |
| Locally stage/commit and push those bounded changes on existing `main` to verified `origin` using SOP §6B | **YES** |
| Modify Architecture, Product, OUTLOOK, README, AGENTS, SOPs, sources, contracts, assets or other repos | **NO** |
| Create branch, clone, extra worktree, PR, merge, rewrite/force push, tag, release, deployment, paid service or provider change | **NO** |
| Install dependency or implement application/schema/tests or run unapproved external operations | **NO** |

The single `.documents/Reports/execution-report.md` must record: `SOLO-WO-015 R01`, date and `COMPLETE | STOPPED | FAILED`; verified local path/branch/upstream/one-worktree/one-origin and exact initial HEAD; installed ACTIVE bytes/SHA-256/blob; frozen-order, WIP transition and report-only commit identities and changed paths; exact ROADMAP deletion and proof of all other lines preserved; exact NEXT content comparison; final ACTIVE neutral content; unchanged canonical Architect/Product and read-only blobs; Stage 00 remaining R00.E/F/G; G01–G08 with direct source/commit evidence; tests/build/browser marked **NOT APPLICABLE** rather than PASS; true worktree and temp cleanup; any actual blockers. The report may not include its own future Git commit SHA; supply that SHA to the Owner in the chat handback.

**Resumption:** After the Owner receives the agent's execution handback, the Chief Architect performs the SOP-mandated GitHub-first report review and independently returns `ACCEPT`, `REVISE` or `STOP` on the exact published closure revision. No acceptance is delegated to the Implementation Agent.

**End of frozen issued scope — SOLO-WO-015 R01.**
