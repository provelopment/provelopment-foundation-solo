# Foundation Solo — Active Work Order

## Approved Scope — `SOLO-WO-019` / `R03`

**Marker:** `OWNER-ISSUED` — executable only after the Owner installs this exact R03 in `.documents/WIP/workorder-active.md` and sends the matching Owner handoff.

**Owner decision:** This R03 **replaces R02** and supersedes its PR-based delivery procedure. PR #1 was **closed without merge** by the Owner's direction. The former WO-018 review/merge/closure prerequisites are withdrawn for this bounded documentation task. **No new PR, extra approval loop, or application-code activity is required.** This does not claim that a prior work order was accepted or closed.

**Purpose:** Make proactive **security by design** a permanent, documented Foundation Solo governance requirement **now, while the project has no application code**. Publish the already prepared security documentation directly to `main` in the single Solo checkout. The WordPress advisory is the first process example, **not** a vulnerability found in Solo.

### Repository and fixed source

- Repository: `provelopment/provelopment-foundation-solo`; expected Owner-local root: `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (the agent must verify its actual accessible root).
- Target: existing `main` and existing single `origin`, `https://github.com/provelopment/provelopment-foundation-solo.git`.
- Observed `main` at preparation: `e3053ece0b95a3bb081055f9022732573feee246`; agent must check its actual HEAD and remote before writing.
- **Approved content source:** the existing, now closed PR #1 head `b62d856adeff708ed34daaf14a62aed8d10d0c00` on `docs/solo-wo-018-security-register`. This commit already contains the completed security register and governance changes. **Closing the PR did not integrate these changes into `main`.**

### Authorized file changes — exactly this scope

The Owner installs this R03 in `.documents/WIP/workorder-active.md`. The agent may commit those Owner-installed bytes **unchanged**, but may not rewrite or clear ACTIVE.

Integrate the existing content at `b62d856` for these **eight** documentation files onto `main` (use existing Git objects in the one checkout; do not open another worktree or clone):

1. `.documents/About/FOUNDATION-SOLO.md` — proactive security as a core product principle.
2. `.documents/Architecture/ARCHITECTURE.md` — §5A security-by-design requirements and lifecycle.
3. `.documents/SOP/sop-architect.md` — §7C security review protocol.
4. `.documents/SOP/sop-implementation.md` — §7A implementation-agent security duties.
5. `.documents/Security/security-issues.md` — canonical register, including first WordPress worked example.
6. `.documents/WIP/plan-roadmap.md` — security gates in pending `R00.G`, `R10.C`, `R11.A`, preserving all existing packages.
7. `README.md` — links and document map.
8. `AGENTS.md` — authority mapping for the security process and register.

Also overwrite the **one** canonical `.documents/Reports/execution-report.md` with the factual run report. **No other permanent file may be modified.** In particular, leave `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, `.github/SECURITY.md`, other repositories, code and toolchain files untouched. WIP still contains exactly four files.

### What must be codified

The documents must clearly say that security is part of Solo's planning, architecture, implementation, release, update and maintenance processes, not a late hardening exercise. The single process is **Identify → Verify → Assess → Prevent/Respond → Validate → Reassess**. The Architect SOP owns the review process; the Implementation SOP owns agent obligations; Architecture owns design constraints; the Security register owns public-safe findings, evidence, accountability and revisit triggers. The register **never** authorizes work, and private vulnerability detail remains in GitHub private reporting under the existing `.github/SECURITY.md`.

Retain `SOLO-SEC-2026-001` (WordPress Core `CVE-2026-119977`) as the initial worked example. Its conclusion stays **Not applicable to the assessed code-free Solo revision**, while any relevant future update-integrity controls are clearly described as **proposed**, not implemented or tested. Do not claim an application, deployment, scanner, penetration test, remediation or security certification exists.

### Simple execution and evidence

1. Read Solo's mandatory entry documents and SOPs; verify the local root, `main`, its remote/HEAD, one worktree, four WIP files, installed R03 marker and matching Owner handoff. Treat only the Owner-installed ACTIVE modification as expected; STOP on unknown changes. Verify the source commit and eight authorized content paths are accessible. Never discard Owner data or rewrite Git history.
2. From `main` in the single existing checkout, apply **only** those eight content files from `b62d856` (or their exact equivalent changes) plus the Owner-installed R03 ACTIVE. Do **not** merge or reopen PR #1, create a new PR, use a second branch/worktree, or accidentally overwrite files outside the ledger. If `main` has relevant intervening edits, preserve them and STOP for a material conflict rather than overwriting.
3. Check the documentation links, consistency of the six-step protocol, the four WIP file names, preservation of all ROADMAP packages, and `git diff --check`. **Application builds, dependency audits and runtime tests are NOT APPLICABLE because there is no code.**
4. Overwrite `.documents/Reports/execution-report.md` with factual evidence: changed paths, what was codified, source and resulting Git revisions, checks actually performed, no-code limitations, and task-owned temp-file cleanup or limitations. Stage **only** the ten named paths (eight documents, Owner-installed ACTIVE, canonical report), commit and push them **directly to `origin/main`** under Architect SOP §6B. Verify remote readback. Do not embed the report's eventual commit SHA inside itself; provide it in the agent's chat handback. Leave ACTIVE installed for later Owner reconciliation.

**Permissions:** Normal stage/commit/push on `main` for these exact paths are authorized. No pull request, merge operation, release, deployment, dependency install, external security scan, credential access, provider/DNS operation, destructive Git action, or other repository write is authorized. Use Solo `.temp/` and the active Cline-provided temporary area only for disposable work, and remove task-owned residue when practical.

**Completion condition:** The eight documentation files are present on `main`, the proactive security process is linked from the canonical documents, the WordPress example is retained with its honest applicability, the four-file WIP rule holds, and the one factual execution report is published on `main`. Agent reports the final `main` commit and any actual blocker. No PR review or app-code testing is required.
