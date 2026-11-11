# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-064 R01**  
**Title:** Administratively close independently accepted R03.C1, select R03.C2 as the sole NEXT candidate, and prune only the safely merged recovery branch  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner handoff ID:** `SOLO-WO-064 R01`  
**Approved Scope:** The Owner expressly authorized the administrative closure of **accepted R03.C1** and preparation of the next bounded R03.C step. This is one **documentation/Git-lifecycle-only** order. It authorizes the exact issuance, closure, candidate-selection, conditional single-branch pruning, CI and reporting operations below; it does **not** authorize executing R03.C2, altering product code, selecting media artwork or catalogue entries, changing Architecture, or closing the still-incomplete R03.C / Stage 03. The Chief Architect is **GitHub READ ONLY**; only the Implementation Agent performs authorized repository writes in the single verified checkout.

## 1. Accepted authority and exact starting state

**Architect disposition:** `ACCEPT — SOLO-WO-063 R01 / R03.C1` at the independently verified canonical final report HEAD `23ce81ab2747e0556632794e1f79c4c9cbb124e3`, tree `da91fe3e2fa83a3fe66d018898bf78b40b347326`, report blob `34bc84e79c725049f588a9b1ee1f2c00f4762593`. The accepted substantive R03.C1 commit is `42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1` (tree `e3ba61cffb24e13451f9b8a732b09282cdd5bf27`), a parent of the report commit. Two-host green CI on the recovery branch, substantive `main`, and report HEAD was independently reviewed. This is acceptance of the **authored graphic asset-obligation inventory** only, not a safe media system.

**Read before acting:** `AGENTS.md`; `.documents/SOP/sop-architect.md` (especially §§6A, 6B, 7A, 8); `.documents/SOP/sop-implementation.md` (especially §§4, 5A, 5C, 9); `.documents/Architecture/ARCHITECTURE.md` (especially §§6.2C, 6.2G, 6.2I, 6.2K); the four WIP files; the canonical report at the above exact SHA; and the actual implemented R03.C1 inventory / source-read interfaces. The SOP's one-checkout, WIP purity, approved-path, report, and Git-safety rules remain binding and are not replaced by a conflicting Git procedure here.

**Expected untouched baseline at `main == origin/main`:**

| Item | Exact anchor |
| --- | --- |
| `main` / `origin/main` | `23ce81ab2747e0556632794e1f79c4c9cbb124e3` |
| Final accepted tree | `da91fe3e2fa83a3fe66d018898bf78b40b347326` |
| `.documents/WIP/workorder-active.md` | `4ec5c24511a9420d7ed782f7316e9d14b0b091da` — frozen `SOLO-WO-063 R01`, to be replaced by Owner-installed `SOLO-WO-064 R01` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` — one neutral heading |
| `.documents/WIP/plan-roadmap.md` | `31a0ba2efb4f780f7248c56ff84683f20d9bc97f` — Stage 03 and remaining R03.C |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` — unchanged neutral heading |
| `Cargo.lock` | `2f9e53576df117f2020ae064d6973304f20833f1` — 22 packages, patched `rustix 1.1.6` |
| `crates/solo-adapters/Cargo.toml` | `6109224cd3e3e89487884631f2b07c68ad8149c3` — exact Linux `rustix = "=1.1.6"` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` — no advisory waiver |
| `.github/workflows/rust-verify.yml` | `eb78dad240bb7adb3e0ef9535870897176c99843` — two-host required gate |
| Original historical branch `solo-wo-061-r01` | `4371af21dce67f33cf1df0384738bf353ca80fc6`, tree `89c6176a5f159069eccab9eb0d4ed67e88eaba4f` |
| Accepted recovery branch `solo-wo-063-r01` | `42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1` — safely contained in `main` before this order |

**Branch decision based on independently observed Git history:** `solo-wo-063-r01` is an ancestor of accepted `main` and had no GitHub PR as checked for this order; it is eligible for **ordinary conditional pruning** under §3C. `solo-wo-061-r01` is **divergent** from `main` (its original implementation commit was later cherry-picked, not merged as a parent). Its unique historical commit must remain addressable. **Retain the original branch locally and remotely, unchanged**; do not attempt to prune it, force-delete it, rewrite it, or substitute a tag/archival mechanism under this order.

**Architecture precedence:** the retained ROADMAP line mentions an older optional generic fallback, but binding Architecture §6.2I.1 and §6.2I.6 explicitly prohibit **automatic platform or role fallback**. The exact ROADMAP one-line replacement below reconciles *planning wording* with the already binding rule; it is **not** an Architecture amendment or authorization to invent an alternate asset provider. The initial lawful `platform/assets/` catalogue, its bytes, IDs, hashes and licence/provenance require a later Owner-reviewed delivery package. No catalogue or artwork exists on the currently reviewed tracked tree.

## 2. Exact permanent-path ledger — no other tracked edits

Only these four tracked paths may change across this order:

1. **`.documents/WIP/workorder-active.md`** — Owner installs this complete `SOLO-WO-064 R01`; Implementation Agent verifies it is the only tracked pre-issuance difference, commits it **alone** as the issuance revision, then resets this *same path* to the exact neutral content in §3D as part of the accepted-closure revision. No historical summary/status prose remains after clearing.
2. **`.documents/WIP/plan-roadmap.md`** — in §3D replace **exactly the one R03.C bullet** identified in §3D, removing the accepted R03.C1 obligation from unfinished work and describing only still-unimplemented R03.C delivery. Preserve the Stage 03 heading, prerequisite, exit gate and every other byte/line of Stage 04 onward. **Do not** remove R03.C or Stage 03, and do not add a completed-work ledger.
3. **`.documents/WIP/plan-next.md`** — replace its one neutral heading with the **exact single R03.C2 candidate** in §4. NEXT remains a proposal, not an executable order. Do not create a parallel plan or issue a code-bearing order.
4. **`.documents/Reports/execution-report.md`** — overwrite the **sole canonical report** and publish a report-only final commit after the closure revision and required CI. Never create another report or evidence folder.

**Explicitly forbidden:** changes to `.documents/WIP/plan-outlook.md`, `.documents/Architecture/ARCHITECTURE.md`, About, SOP, AGENTS, README, source/tests, fixtures, `platform/`, assets, `Cargo.toml` or `Cargo.lock`, dependency graph, `deny.toml`, workflows, toolchain, `.cargo/`, `.gitignore`, release files, any other repository, remote or worktree. No new tracked file or directory. No changes to third-party licensing, NOTICE, provider, renderer, URL/routing, location, manifest or publication behavior. No code implementation of R03.C2, no generated media and **no deployment or release**.

**Git authorizations:** normal staging/commit/push of only the named paths on existing `main` for issuance, closure and report; read-only compare/PR/branch/CI inspection; **only if §3C conditions pass**, ordinary local and remote deletion of **`solo-wo-063-r01`** after its tip is proven included in current `main`. No new branch or PR, merge, fast-forward operation, rebase, cherry-pick, hard reset, force-push/force-delete, history rewrite, tag, extra worktree/clone, nested Git repository, or deletion of other refs. Follow Architect SOP §6B for the version-control procedure.

## 3. Required execution sequence

### 3A — Mandatory read-only preflight

Confirm the actual Owner-local Solo root, one existing checkout and worktree, sole correct `origin` remote, `main == origin/main` at the exact accepted baseline, clean staging area, exactly the four named WIP files, and **only** the Owner-installed ACTIVE replacement as a permitted initial tracked difference. Confirm no unknown untracked/Owner-owned material, no unsafe Git operation in progress, no unexpected branch/PR drift, and both original/recovery branch tips as anchored. Read back the newly installed complete order; the matching Owner handoff must name `SOLO-WO-064 R01`. If any anchor, branch, order identity, working-tree safety or authorisation fails, STOP without overriding it.

### 3B — Issue this administrative order (ACTIVE only)

Stage and commit **only** the Owner-installed `.documents/WIP/workorder-active.md`, verify exact committed blob and paths, and push normally to `origin/main`. Record issuance commit/tree. Trigger/observe the existing `rust-verify` workflow (ordinary `main` push) on Ubuntu 24.04 and Windows 2025 at that *exact* issuance commit. All applicable 33-step mandatory gates must be green, with only host-inapplicable skips. A newly failing security gate, regression, architecture check, test or build is a STOP, not a reason to alter CI/policy/code. Do not claim the closure is done on an issuance-only revision.

### 3C — Conditional prune of the safely integrated recovery branch ONLY

After the issuance commit has green CI, **while the issued `SOLO-WO-064 R01` is still installed in ACTIVE**, freshly verify that (a) the local and remote `solo-wo-063-r01` tips are **both** precisely `42e9588f9e3fb84f4ec15c2f7d02f9ad5ae440d1`; (b) this tip is a **true ancestor** of the then-current `main`, with its history already reachable through the accepted substantive, report and issuance commits; (c) there is no open or otherwise active PR, branch protection, another worktree, unknown divergent commit or other dependency on that branch; and (d) the worktree is clean on `main`.

Only after those checks, prune **that named branch only** using ordinary non-force deletion locally (`git branch -d`, not `-D`) and remote (`git push origin --delete solo-wo-063-r01`, not a force-push), observing repo permissions and reporting the exact result. The history remains reachable on `main`. Never alter the tip of `main` to prune a branch. **Do not delete `solo-wo-061-r01`**: it is divergent and keeps the historical `4371af21…` commit reachable. If recovery-branch pruning is blocked or cannot be proven safe, do **not** substitute a force operation: preserve the ref, explain why in the canonical report, and mark pruning `UNVERIFIED/BLOCKED` rather than claiming it completed; STOP if this reveals unknown or Owner-owned work at risk. The administrative closure must not be misrepresented as a code acceptance failure solely because a protected branch remains.

### 3D — Accepted R03.C1 closure and selection of sole NEXT candidate

After §3C is completed or its optional pruning safely deferred with the reason recorded, perform **only** these three WIP edits in one closure commit:

**ROADMAP, exact old bullet:**

> - **R03.C:** implement graphic role declarations, public shared generic library, authored asset ownership, inheritance/override/suppression, and optional role-specific generic fallback strictly beneath authored hierarchy.

**ROADMAP, exact replacement bullet (one Markdown paragraph; keep surrounding lines unchanged):**

> - **R03.C:** complete the remaining governed graphic-asset delivery beyond the accepted authored-obligation inventory: confined Entity `content/assets/` and explicitly authored, exactly pinned Platform `platform/assets/` source acquisition; authoritative bounded media-byte, format/magic/MIME, image dimension and pixel-limit verification; an Owner-reviewed lawful `platformAssetLibraryVersion: "1.0.0"` catalogue with complete asset IDs, hashes, media class, dimensions and licence/provenance; whole-candidate validation across all declared owners including internal, suppressed and unlisted declarations; and privacy-safe public use and immutable derived outputs. Preserve the accepted five-level graphic inheritance, provenance, exposure, cardinality, suppression and **no automatic platform or role fallback** under binding Architecture §§6.2G/6.2I.

This is a scoped removal of the **accepted** R03.C1 portion and clarification of *remaining* Stage 03 implementation only. It corrects stale planning wording by reference to **already binding** Architecture §6.2I; it does not create or change the Architecture contract. No other ROADMAP line may change. If the exact old bullet is absent, do not guess a fuzzy replacement; STOP and report.

**NEXT:** replace the neutral file with exactly the Markdown candidate in §4 (from its `# Foundation Solo` title to the end, one final newline). No second candidate or older candidate may be appended.

**ACTIVE:** reset its complete contents to exactly:

```text
# Foundation Solo — Active Work Order
```

The three paths `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` are the **only** staged files in the closure commit. Compare each byte/diff before committing; push normally to `origin/main`. Obtain complete green Ubuntu/Windows CI at this exact closure SHA before report publication.

### 3E — Canonical report, final CI, cleanliness

Overwrite **only** `.documents/Reports/execution-report.md` with factual execution evidence: issued-order identity, exact baseline, issuance/closure/report commit+tree anchors (do **not** embed the report's own commit SHA in its contents), all changed paths and before/after WIP blobs, one-line ROADMAP delta, exact NEXT candidate match, immutable OUTLOOK, Stage 03 still OPEN, baseline regression/security proof, both-host CI runs/jobs/steps and warning/skips, original-branch retention and conditional recovery-branch pruning, and any local cleanup limitations. Report `COMPLETE` only for the verified administrative scope; no self-ACCEPT of the closure or of R03.C2.

Commit this **one report path only** and push to `origin/main`; obtain full green Ubuntu/Windows CI at its exact final report HEAD. Read back the canonical report blob at the exact remote HEAD and verify the three-commit linear ancestry (issuance → closure → report), clean local `main == origin/main`, correct remote branch inventory, unchanged product/lock/policy/workflow, and exact WIP file count. Report final SHA/tree in the *chat handoff*, not self-referentially in the report.

**Scratch discipline:** the already ignored live root `target/` cache must remain present and unaltered; retain the inherited Solo `.temp/` archives (reported baseline 2 files / 289,253,806 bytes) until their defined later use/cleanup trigger. Task-owned `.temp/` and Cline-provided temporary logs may be used, but clean identified obsolete task files promptly; never delete unknown material or infer scratch cleanliness from an empty `git status`. Do not alter `.gitignore` or invent an alternate temp root.

## 4. Exact NEXT content — one **unissued** R03.C2 candidate

Use **exactly** the following content as `.documents/WIP/plan-next.md`; it contains one roadmap-derived candidate and no implementation authorization:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R03.C2 — Confined Entity graphic asset-byte acquisition

**Objective:** Establish the next narrow, nonpublishing gate after accepted R03.C1: for each explicitly authored `namespace: "entity"` graphic obligation, acquire bounded authoritative source bytes from that Entity's confined `content/assets/` namespace and return typed evidence or an explicit fail-closed refusal. This gate proves **only** source acquisition and confinement, not that the bytes are a safe or correctly decoded image. No inferred source, role fallback, external fetch or publication is permitted.

**Prerequisites/readiness:** R03.A's registry, R03.B's complete five-level resolution/provenance, R03.C1's whole-graph authored graphic inventory and accepted R02 capability-confined `content/pages/` reader are present; locked `rustix 1.1.6` is green on both hosts. Binding Architecture §§6.2C and 6.2I locate Entity asset references relative to `content/assets/`, independent of Page source candidates. Before any code-bearing order, inspect the real native source-reader, bootstrap root trust, application ports, asset reference grammar, physical alias/collision policy, negative host tests and all current interfaces; propose the smallest precise path/operation ledger and any narrowly required safety reviews. Existing Page source reading is `pages/`-specific and **cannot** be assumed to cover assets without an explicitly tested extension. No tracked artwork or lawful platform catalogue is approved by this candidate.

**Proposed scope:** Drive the accepted R03.C1 *entire authored obligation inventory*, not the selected public representation alone, through a typed, explicit Entity-asset byte-acquisition boundary. Resolve only `namespace: "entity"` beneath its operator/trusted-handle-designated `content/assets/` root, with validated single-component traversal and the existing Linux/Windows non-follow, no-escape, identity/recheck and immutable-transfer guarantees; reject absent, unsafe, inaccessible, cross-device, reparse/symlink, hardlink/physical-alias and changed files rather than treating failures as absence or falling back. Bound an image source to **20 MiB maximum** before/during the read, keep exact declaring owner/role/index/exposure/namespace/ref and safe diagnostic categories, and refuse the whole candidate if *any* declared Entity image fails, including an internal, descendant-suppressed or unlisted declaration. Do not scan directories, guess extensions or infer asset ownership. A declared `namespace: "platform"` remains **unverified and ineligible** until a separate Owner-reviewed, exactly pinned `platformAssetLibraryVersion: "1.0.0"` catalogue and reader exist; mixed inventories must not be represented as fully validated by simply skipping platform references.

**Proposed deliverables:** A future separately Owner-issued, exact file/operation ledger; pure inward port/use-case types and narrowly scoped outward confined reader integration if warranted; a typed nonpublishing read-evidence/refusal collection bound to each original authored obligation; realistic Linux/Windows filesystem positive and adversarial cases (including no-follow, traversal, reparse/junction, hardlinks/identity collision, oversize, missing and changed source), whole-graph internal/unlisted/suppressed negative cases, zero-publication/zero-default assertions, unchanged Page source-reader and all earlier R01/R02/R03 regressions, fresh A1 review and full two-host CI. Use disposable test content, not invented product artwork or a platform catalogue. If native boundary changes or safe asset-byte validation are too broad, subdivide in the later exact order without pretending R03.C is complete.

**Exit conditions:** Every declared Entity graphic obligation has exactly its own confined, bounded read evidence or causes deterministic whole-candidate refusal; no unrelated or absent file is discovered; namespace and original owner/provenance remain exact, no sibling Entity/Site/representation leak occurs, no public/internal/suppression fallback appears, and Linux/Windows confinement and regression gates are green. Successful byte acquisition **never** asserts MIME/magic match, safe media decode, image dimensions/pixel bounds, licence rights, catalogue validity, immutable public derivative or publication permission. These remain pending R03.C delivery gates, including the Owner-reviewed pinned Platform library. Stage 03 and R03.C remain OPEN.
```

## 5. Mandatory acceptance and refusal matrix — administrative only

| ID | Required independently verifiable result |
| --- | --- |
| `S064-01` | Exact starting `main`, tree, report, WIP, dependency/policy/workflow, branch tips, one-checkout/remote and installed Owner order match §1. |
| `S064-02` | Issuance commit changes **only** ACTIVE; clean two-host CI at exact issuance SHA; order's Approved Scope frozen. |
| `S064-03` | ROADMAP's only diff is the one exact R03.C bullet replacement in §3D; accepted inventory removed from pending work, remaining R03.C and Stage 03 retained, every later stage/exit gate preserved. |
| `S064-04` | NEXT has **exactly** §4's single R03.C2 candidate and is **not** executed; all five NEXT purity fields present, no second candidate. |
| `S064-05` | ACTIVE becomes exactly its neutral heading, OUTLOOK remains byte-identical, exactly four WIP files remain. |
| `S064-06` | ROADMAP wording honors already binding §6.2I no-fallback rule; Architecture itself unchanged, no invented artwork/catalogue/media or new contract decision. |
| `S064-07` | Closure commit changes exactly ROADMAP/NEXT/ACTIVE; green two-host CI at the closure SHA; no product code/test/manifest/lock/policy/toolchain/workflow change. |
| `S064-08` | Original divergent `solo-wo-061-r01` remains local and remote at `4371af21…` unchanged; recovery `solo-wo-063-r01` is pruned **only** when containment/branch/PR/local safety checks prove it lossless, otherwise honestly reported retained with blocker; no force/history rewrite. |
| `S064-09` | Canonical report is the only report; report-only final commit and full green Ubuntu/Windows CI at exact final HEAD; readback SHA/blob/tree/parent and accurate warning/skipped-step ledger. |
| `S064-10` | No ignored test, security exception, bypass or undeclared dependency; known `Unicode-3.0` warning remains noted `UNVERIFIED`, `itoa`/`syn` patch lags untouched. |
| `S064-11` | Single checkout and clean worktree; truthful temp archive/cache preservation and identified task scratch cleanup, no unknown-data removal, exact WIP count. |
| `S064-12` | No implementation of NEXT, new branch/code/asset, Stage 03/R03.C closure, release, deployment, PR, provider or other-repository write; Implementation Agent does not self-ACCEPT. |

## 6. STOP and final handoff

**STOP**, preserving state and issuing a truthful canonical `STOPPED` report where safe, for an unverified Owner handoff, baseline or branch identity mismatch; unexpected tracked/untracked material; any required edit outside the ledger; inability to preserve unique original-branch history; attempts to remove R03.C/Stage 03; a non-fast-forward or forced Git operation; changes to the frozen issued scope; a mandatory two-host gate failure; accidental publication/asset/fallback claim; unsafe branch deletion; or any unknown Owner-data risk. A merely blocked **optional** recovery-branch prune is reported as such and not bypassed. Never continue by mutating `deny.toml`, altering CI, inventing assets or executing NEXT.

**Handoff:** report the exact issuance, closure and final report SHAs/trees, WIP blobs and diff proof, branch tip and safe prune outcome, local/remote cleanliness, full CI run/job/log anchors, and remaining R03.C delivery. After the Chief Architect independently returns `ACCEPT / REVISE / STOP` for **this administrative order**, the Owner may separately issue an R03.C2 code order, subject to its own technical ledger and readiness gate. The presence of R03.C2 in NEXT itself confers **no execution authority**.

**End of OWNER-ISSUED SOLO-WO-064 R01.**
