# Foundation Solo — Active Work Order

## Approved Scope — SOLO-WO-024 / R01

**Issuance and status.** This is the Chief Architect's **proposed administrative closure/reconciliation order** for the *independently accepted documentary* `R00.G` package. The Owner's instruction to prepare this order is not itself execution authority. This exact document becomes **`OWNER-ISSUED` only after the Owner deliberately installs the complete file at `.documents/WIP/workorder-active.md` in the existing single Foundation Solo checkout AND separately gives the matching explicit activation handoff for `SOLO-WO-024 R01`.** An agent must STOP without both. Earlier `SOLO-WO-023 R01/R02` activations cannot authorize this order. The order does not change, reopen or implement any Owner-decided architecture.

| Identity | Authority and reviewed baseline |
| --- | --- |
| Work order | **`SOLO-WO-024 R01`** — R00.G documentary administrative closure only |
| Repository | `https://github.com/provelopment/provelopment-foundation-solo` |
| Branch / remote | `main`; one expected remote `origin` pointing to the repository above |
| Reviewed published `main` HEAD | **`5a153155a70bd8e6cd6527e805c4d0c402739021`** — canonical `SOLO-WO-023 R02` report commit |
| Independently accepted R00.G source | **`43388be86dc95669ec5683039e430f0f96061910`** — corrected Architecture and README; tree `9a162fdc868b585bf6fe423e2b9c2b3111367b71` |
| R02 issuance freeze | `ca51ccf7a0a6a864e0a43549e7c1189fa2fbcb92`; still-installed ACTIVE blob `609af99713d93372d88966826b5f9ca2205c7247` |
| Architect disposition | **`ACCEPT`** for `SOLO-WO-023 R02` at `5a153155a70bd8e6cd6527e805c4d0c402739021`, recorded in the Owner–Architect discussion; not an implementation or Stage 00 exit acceptance |
| Package before this order | `R00.G` sole ROADMAP Stage 00 remaining package and sole NEXT candidate; Stage 00 open; R02 ACTIVE still installed |
| Package after successful execution | R00.G removed from remaining ROADMAP work, canonical documentary-acceptance status reconciled, R01.A sole NEXT **candidate**, ACTIVE neutral; **Stage 00 deliberately remains unfinished** pending its separate exit gate |
| Separate Owner handoff | Owner explicitly installs this entire `SOLO-WO-024 R01` and directs the agent to read and execute this exact ACTIVE path; STOP on any mismatch |

### 1. Objective, authority and essential separation

**Do only the accepted R00.G documentary closure transition under Architect SOP §8.** Preserve the five Owner-approved decisions and refinements now recorded in Architecture §6C and the single §6.5 `RG01`–`RG11` requirement matrix. Reconcile the minimal accepted/remaining state in ROADMAP, NEXT, Architecture and ACTIVE, then publish one factual execution report. This is administrative documentation work, **not** a new architectural decision, a code-bearing work order, release, provider action or deployment.

**Stage 00 exit-gate restraint is non-negotiable.** ROADMAP Stage 00's separate exit gate requires: (1) consistent accepted contracts, (2) accepted-vs-open non-blocking decision mapping, and (3) **a minimal first code-bearing complete work order prepared by the Chief Architect for Owner review**. No such first code-bearing order has been presented in this handoff. Thus this order may close **R00.G as a package**, but it must **not** remove Stage 00 from ROADMAP, mark its exit gate passed, mark Stage 00 closed, claim the first code-bearing order already exists, or authorize Stage 01 source work. After independent review of this administrative run, the Chief Architect can prepare the separate first code-bearing order and present its precise evidence for the remaining Stage 00 exit decision. This order does not create a second work order or delegate drafting one to the Implementation Agent.

**Read mandatory authorities before action:** `README.md`; `AGENTS.md`; `.documents/About/FOUNDATION-SOLO.md`; `.documents/Architecture/ARCHITECTURE.md`; `.documents/SOP/sop-architect.md` and `.documents/SOP/sop-implementation.md`; all four current WIP files in the repository's mandated order; `.documents/Reports/execution-report.md`; and `.github/SECURITY.md`/`LICENSE` as relevant. Architect SOP §6B exclusively governs Git; §§7A–7B govern reporting, independent review and A1; §8 governs WIP transition. Read the actual files rather than copying procedures into the implementation. Agent `COMPLETE` never substitutes for independent Chief Architect `ACCEPT`.

### 2. Preflight and freeze — before any other write

1. Verify actual accessible existing Solo Git root, branch `main`, current/local/tracking/remote HEAD and ancestry, sole correct `origin`, exactly one worktree and checkout, clean or precisely explained status, stashes, worktree inventory, four and only four WIP files, and existing report. Never create a clone, extra branch, alternate worktree, shadow tree or nested repository. The **only expected initial dirty file** is the Owner's entire replacement of `.documents/WIP/workorder-active.md` from the committed R02 to the installed R01 for **this new work order**. STOP on any other unexpected dirty, untracked or Owner-owned state rather than cleaning it destructively.
2. Confirm the separate Owner handoff explicitly matches `SOLO-WO-024 R01` and this entire installed `Approved Scope`. Read back its exact contents; calculate byte length and SHA-256, UTF-8 without BOM, LF-only and final LF. Do not assert an unmeasured digest or accept partial installation.
3. Confirm reviewed remote baseline HEAD **`5a153155a70bd8e6cd6527e805c4d0c402739021`**; verify published ancestry includes `ca51ccf7…` then `43388be8…` then the report commit. Verify all file blob anchors below from the actual Git tree before first write. Only a newer **report-only** descendant with independently checked ancestry and unchanged substantive blobs may be tolerated; any other drift triggers STOP/Owner review.
4. **The first mutation/commit MUST be the exact installed ACTIVE alone.** Commit and push the full Owner-installed `.documents/WIP/workorder-active.md` without changes, before touching any other authorized file. Independently read back its published blob from GitHub and verify identical byte length and SHA-256. STOP if freeze, publication or remote readback fails. Do not annotate or clear ACTIVE until the specifically authorized later step.

| File at reviewed `main` | Exact Git blob SHA |
| --- | --- |
| `README.md` | `3f3005d680d9ed182a02e574744054aaae55deb2` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `.documents/About/FOUNDATION-SOLO.md` | `1d80f62769053ba4e6ca51adf2d11c68e724c30e` |
| `.documents/Architecture/ARCHITECTURE.md` | `a5dcdbfe9875a54d4b340b8b8ec06ff78a4bd4ee` |
| `.documents/SOP/sop-architect.md` | `8760be323b6ed85695a9dcffd3e52f94628f0f07` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/workorder-active.md` **before Owner replacement** | `609af99713d93372d88966826b5f9ca2205c7247` |
| `.documents/WIP/plan-next.md` | `a3e49290c06d525221a3c73a17abf13c7f4c9381` |
| `.documents/WIP/plan-roadmap.md` | `9b0439fe1ad7f831e595a171c0d3eef3d09a4fc3` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `7b8b61312377546be87b638b66361e1822f7273e` |
| `.github/SECURITY.md` | `dd321856fe8035917be3f90c733242bc3d9a8d09` |
| `LICENSE` | `d645695673349e3947e8e5ae42332d0ac3164cd7` |

### 3. Exact reconciliation steps — only after remote-verified ACTIVE freeze

**A. ROADMAP: prune accepted R00.G, keep Stage 00 exit outstanding.** In `.documents/WIP/plan-roadmap.md`, remove **only** the sole Stage 00 bullet beginning `- **R00.G — Lifecycle and nonfunctional standards:**` under `**Remaining packages**`. Since no Stage 00 package remains, remove its now-empty `**Remaining packages**` label as well. Preserve Stage 00's heading, outcome and exit gate **verbatim**, including the requirement for a complete first code-bearing work order prepared for Owner review. Preserve every Stage 01–13 stage, prerequisite, item, description, exit gate and their order byte-for-byte except mechanically unavoidable surrounding newline adjustment. Do not add a completed-history note, new policy, or a substitute Stage 00 work item. Do **not** delete Stage 00 merely because R00.G is accepted; its exit gate is not yet fully evidenced.

**B. NEXT: replace R00.G with the single first Stage 01 candidate.** Overwrite only `.documents/WIP/plan-next.md` under its existing identifying title with exactly one **`R01.A` — Reproducible minimal source layout, typed configuration and deterministic local build** candidate derived from the remaining ROADMAP Stage 01 `R01.A` bullet and Architecture §6A `E01`–`E10`. State only: identifier/objective, prerequisite/readiness, proposed bounded scope/deliverables and verifiable exit conditions. Include: the selected **conditional** toolchain and precise pinned structure/versions by reference to §6A (not invented alternatives); a minimal deterministic clean-room installation/build; strictly typed configuration; no historical Foundation runtime; no speculative feature shells; no provider/Vercel coupling. State honestly that no application code, installed dependencies or executable proof currently exists, `C1`–`C4` remain `UNVERIFIED`, and the **Stage 00 exit gate awaits a complete first code-bearing work order prepared by the Chief Architect for Owner review** before Stage 00 can be finally closed. This is a **candidate for future Owner review only**, not an issued work order, permission to code, a Stage 01 activation or proof of Stage 00 exit. Do not put R01.B/C into this candidate, or silently decide their separate scopes; ROADMAP retains them. If Stage 01 `R01.A` is not dependency-ready on factual inspection, do not misstate readiness: record only a truthful, readiness-limited candidate if consistent with SOP §8, otherwise keep NEXT to its neutral identifying title and explain the blocking prerequisite in the report. Never create an extra NEXT candidate.

**C. ARCHITECTURE: narrow R00.G documentary acceptance parity.** In `.documents/Architecture/ARCHITECTURE.md` change **only** two direct contradictory status claims, leaving the actual §6C contracts and RG matrix unchanged:

1. In §6.5's existing `R00.G` summary bullet, replace the ending statement that the package is **“not yet independently accepted”** with a concise, truthful statement that its **documentary** source was independently `ACCEPT`ed by the Chief Architect at published R02 report revision `5a153155a70bd8e6cd6527e805c4d0c402739021` and its administrative closure is being reconciled under the distinct Owner-issued `SOLO-WO-024 R01`; future functionality/tests still unimplemented/unverified.
2. At §6C's opening **Status and authority** paragraph, replace only its obsolete “R00.G is not independently accepted by this recording” status with a truthful, narrowly worded independent acceptance/administrative closure status, preserving the statement that **Stage 00 remains open** until its distinct exit gate is satisfied and all runtime evidence remains future. No new normative requirement, approved version or implementation evidence is introduced.

Keep §6.5's single `RG01`–`RG11` matrix, full §6C.1–§6C.9 policies, all twenty-two modules, entire §6B contact contract, §6A conditionally selected stack and every historical/cross-reference rule untouched. Historical “this section itself does not authorize Stage 00 closure or R00.G acceptance” is a statement of **authority limits**, not a false status and must remain unchanged. If any other direct conflicting administrative-status sentence is genuinely unavoidable, STOP and report exact lines rather than quietly widening scope.

**D. ACTIVE: clear only after A–C are committed, pushed and verified.** Only once the ROADMAP/NEXT/Architecture reconciliation is safely published and the exact authorized diff is confirmed, replace `.documents/WIP/workorder-active.md` with **exactly** the following content, including one final newline and nothing else:

`# Foundation Solo — Active Work Order`

This R01 order expressly authorizes its own final clearing; it does **not** authorize a new `R01.A` work order or any implementation. Preserve the original frozen R01 in Git history. Verify ACTIVE is neutral on remote after committing and pushing. If A–C cannot be completed or independently verified, leave the frozen ACTIVE installed and publish factual STOP evidence when safe.

**E. REPORT: publish one factual run report, last.** Overwrite only `.documents/Reports/execution-report.md` with this closure run's factual details, exact Owner handoff, initial/frozen/reconciliation/ACTIVE-clear commits and tree identities, expected/actual file blobs, diff audit, NEXT readiness, and the truthful **Stage 00 exit requirement still pending**. Document G01–G10 with actual readback evidence, relevant commands and skip/limitation inventories. Do **not** claim that a first code-bearing work order has been prepared, that Stage 00 has completed, or that any executable proof passed. Publish and read back the one canonical report. Its own commit SHA goes in the agent chat handback, not inside the report. No archive or second report.

### 4. Allowed paths, operations and prohibitions

| Path | Exact authorized change |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs this complete R01; agent freezes it **unaltered first** and, **only after verified reconciliation**, clears it to exactly the neutral heading plus LF |
| `.documents/WIP/plan-roadmap.md` | Remove only Stage 00's `R00.G` bullet and now-empty `Remaining packages` label; retain unfinished Stage 00 exit gate and all later stages unchanged |
| `.documents/WIP/plan-next.md` | Replace sole R00.G candidate with one honest bounded R01.A candidate; no executable order or later-stage work |
| `.documents/Architecture/ARCHITECTURE.md` | Two existing administrative-status statements only, §6.5 `R00.G` bullet and §6C opening status; existing contracts and matrix must remain byte-identical |
| `.documents/Reports/execution-report.md` | Overwrite/publish the one current evidence report |
| `.documents/About/FOUNDATION-SOLO.md`, `README.md`, `AGENTS.md`, both SOPs, `.github/**`, `LICENSE`, `plan-outlook.md`, and all other files | **Read only; never edit**. STOP if a required action demands another path |
| Git | Ordinary authorized commit/push/readback operations for exact listed paths on verified `main` under Architect SOP §6B; no PR, new branch, merge, reset, force push, history rewrite, second worktree, clone or destructive cleanup |
| Temporary | Only safe, task-owned disposable materials within the authorized Solo `.temp/` or active agent temp; clean without touching unknown or Owner-owned content |

**Explicitly excluded:** any application source, tests, CI, generated schema, dependency/toolchain installation, content/asset change, change to accepted architectural literals, provider/Vercel API calls or setup, DNS/TLS/secrets, external service or account activity, payment, monitoring or backup provider work, SBOM/NOTICE creation, product release, deployment, Stage 01 work-order issuance/execution, closure of Stage 00, modification of other repositories or an additional WIP/report file. `C1`–`C4` remain `UNVERIFIED`, `F01`–`F17` `NOT RUN`; `RG01`–`RG11` remain planned future obligations.

### 5. Document-only acceptance gates — independent A1 review

| Gate | Direct evidence required for `PASS` and negative check |
| --- | --- |
| **G01 — authority and identity** | Whole exact installed `SOLO-WO-024 R01` plus matching separate Owner handoff; verified Solo root/branch/remote, sole checkout/worktree, inspected initial Git status, correct reviewed ancestor and all thirteen pinned blobs; no unexplained drift |
| **G02 — issuance-freeze integrity** | First published commit changes ACTIVE alone; exact Owner-installed bytes/hash rechecked against GitHub; no source/report edit before freeze |
| **G03 — ROADMAP precision** | Only R00.G bullet and vacated label removed; Stage 00 outcome/exit gate unchanged and remains in ROADMAP; all Stage 01–13 packages/exit gates unchanged; no completed history inserted |
| **G04 — NEXT correctness** | R00.G no longer NEXT; exactly one R01.A bounded candidate if ready, otherwise neutral NEXT with factual blocker; correct Stage 01 scope, no hidden work order, explicit unimplemented/unverified and Stage 00 exit-gate caveats |
| **G05 — canonical acceptance parity** | Architecture §6.5 and §6C opener describe independently accepted R00.G documentary status; no false claim of implementation or Stage 00 closure; all accepted contracts, RG01–RG11 matrix, prior ledgers/versions and statuses preserved |
| **G06 — safe ACTIVE clearance** | Reconciliation commit precedes ACTIVE clearing; frozen order still visible in Git ancestry; ACTIVE remotely contains only neutral identifying heading and LF, with no new issued order |
| **G07 — scope confinement** | Whole published diff contains only the five authorized paths; About, README, AGENTS, SOPs, SECURITY, LICENSE, OUTLOOK and all app/other files unchanged; no provider/config/API operation |
| **G08 — evidence honesty and Stage 00 hold** | `R00.G` documentary acceptance and administrative closure only; Stage 00 still open; first complete code-bearing work order NOT claimed prepared; C/F/RG executable gates still future; no invented test, provider, release or deployment claim |
| **G09 — published evidence and Git** | Published reconciliation and clearing commits/trees, remote tracking/readback, clean or accurately explained worktree, one worktree and origin, no stray task-owned temp; last current report overwritten and remote-fetched |
| **G10 — preservation/negative audit** | Verify no changed D/O/V/P/E/C/F/RG ledger, twenty-two registered modules, one-Entity/EFS identities, exact version/refusal/route contracts, security/A1, approved R00.G D1–D5, pragmatic AAA, Vercel-independent core or contact guarantees; no Stage 01 authorization |

Each gate must be reported as `PASS`, `FAIL`, `UNVERIFIED` or reasoned `NOT APPLICABLE` with actual evidence and source paths. Read-only documentary inspections, diff comparisons, reference checks and rejection/absence audits are appropriate; **code/browser/test gates are not executable now**. Do not label unrun test commands PASS, and do not conceal a material contradiction with a broad `NOT APPLICABLE`.

### 6. STOP, report and handback

STOP on incorrect Owner activation, unexpected checkout/worktree/remote/HEAD changes, missing source anchors, changed contracts, nonmatching frozen bytes, unverifiable Git readback, attempt to delete Stage 00 before its exit gate is met, attempt to issue `R01.A` source work, or need to edit unlisted files. Do not destructively discard user data or quietly expand scope to complete the Stage 00 exit requirement.

After actual completion, the Agent must publish the sole canonical report on GitHub, return exact remote final commit/tree and each gate's evidence and scope limits, and explicitly state that **independent Chief Architect review is still required**. `COMPLETE` is the agent's run status, not the Architect's `ACCEPT`. The Owner controls any subsequent Stage 00 exit decision and the separate first code-bearing work order; this order grants no automatic next action.
