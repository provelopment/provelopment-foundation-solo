# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-062 R01**  
**Title:** Repair inherited `rustix` security advisory with the smallest pinned manifest/lockfile update; preserve stopped R03.C1 for separately authorized verification  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Owner handoff ID:** `SOLO-WO-062 R01`  
**Approved Scope:** The Owner expressly authorizes **only** the narrow security-remediation, release-independent Git integration, verification, and factual reporting steps enumerated in this frozen order. This order replaces the STOPPED `SOLO-WO-061 R01` in the one ACTIVE slot. It does **not** accept or resume R03.C1 implementation, run the unverified branch through a new dependency baseline, merge or prune its branch, close R03.C/Stage 03, upgrade other packages, change a security policy, or authorize any release, deployment, product asset or provider work. Only the Implementation Agent, after the Owner installs this file and gives the exact matching handoff, performs GitHub or local permanent writes. The Chief Architect's GitHub access is **READ ONLY**.

## 1. Accepted authority, verified starting anchor and reason for this order

**Governing rules:** `AGENTS.md`; `.documents/SOP/sop-architect.md` §§6, 6A, 6B, 7A, 7B, 8; `.documents/SOP/sop-implementation.md` §§4, 5, 5A, 5B, 5C, 7, 7A, 7B, 8–10; accepted `.documents/Architecture/ARCHITECTURE.md`; the four-slot WIP lifecycle. Read these in the required order, not merely their headings. There is exactly one permitted implementation checkout and one canonical execution report.

**Architect disposition:** `STOP — SOLO-WO-061 R01`, independently established after reading the published canonical report at `9ed94120824bccc0b9bcba081544dc7cd8d11b95`. The STOP was because freshly refreshed RustSec data causes the existing mandatory `cargo-deny` gate to reject `rustix 1.1.5` under `RUSTSEC-2026-0486`. Both R03.C1-branch and report-HEAD CI runs failed **before** behavioral tests; their skipped downstream steps are **NOT VERIFIED**. The R03.C1 source and tests on the retained branch have **not** been accepted or integrated.

**Verified GitHub `origin/main` at order preparation:** commit `9ed94120824bccc0b9bcba081544dc7cd8d11b95`; tree `371d4198643a67f282bd763ab35b7342909b7196`. The report there is `.documents/Reports/execution-report.md`, blob `5bf95baa24ccaa1b1d51838560ca2e8f3e7c86ea`. The immediate main parent is issuance commit `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae`. Re-fetch live before every permanent step and STOP if any identity or scope anchor has unexpectedly moved.

**Four WIP slots at starting HEAD:**

| Existing file | Expected Git blob / state |
| --- | --- |
| `.documents/WIP/workorder-active.md` | `5b1658f4d9c625aaa3fe8828501aebbaca6b31ea` — frozen stopped `SOLO-WO-061 R01`, to be replaced **only** by the Owner-installed `SOLO-WO-062 R01` at issuance |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` — neutral heading, **unchanged** throughout this order |
| `.documents/WIP/plan-roadmap.md` | `31a0ba2efb4f780f7248c56ff84683f20d9bc97f` — Stage 03 OPEN, R03.C remains, **unchanged** |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` — neutral heading, **unchanged** |

**Other pinned evidence:**

- `crates/solo-adapters/Cargo.toml`: blob `a677df5f4b0f0d4fbd36d50c290e32f8cacc0847`; its Linux-target dependency is exactly `rustix = { version = "=1.1.5", default-features = false, features = ["fs", "std"] }`.
- `Cargo.lock`: blob `8bd0bb00286a97853624ebbc0fe18da42a733f12`; locked `rustix 1.1.5`, registry source, recorded checksum `891efababe418670775f199f0d233d84843c227a0949a883ce15b37c78d6629d`. The accepted pre-update inventory contains 22 packages (four first-party and 18 external).
- `deny.toml`: blob `3063e864c28be1e77787f43f1b6f561d543c8f4a`; `unsound = "all"`, no ignored advisories; **must not change**.
- `.github/workflows/rust-verify.yml`: blob `eb78dad240bb7adb3e0ef9535870897176c99843`; its two-host CI and fresh RustSec check **must not change**.
- `.gitignore`: blob `f45dd49524748e9b08f6678271edfc22ac39b97f`; the inherited root `/target/` exclusion **must not change**.
- `rust-toolchain.toml` remains exact Rust `1.99.1`; no compiler/toolchain upgrade.
- Retained R03.C1 implementation branch **locally and remotely**: `solo-wo-061-r01`, tip `4371af21dce67f33cf1df0384738bf353ca80fc6`, tree `89c6176a5f159069eccab9eb0d4ed67e88eaba4f`, parent `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae`. It contains exactly five authorized R03.C1 paths. It must be preserved **byte- and commit-identical**, not merged, reset, cherry-picked, rebased, renamed, deleted, fast-forwarded or pruned under this order.

**Independent security evidence:** Official `RustSec/advisory-db` entry `crates/rustix/RUSTSEC-2026-0486.md` (also alias `GHSA-jh2m-fhpj-3vr7`) identifies unsound zero-length-buffer handling in six `rustix::fs` xattr functions for `>=1.0.0, <1.1.6`, with patched range `>=1.1.6`. `rustix 1.1.6` is a published stable compatible patch release. Local Solo's `openat2` use is not a known xattr call site, but the accepted mandatory policy fails the *locked crate*, not only exercised functions; code-path mitigation is never a substitute for passing the unchanged gate.

**Published gate evidence:** issuance `42987963563` at `dcaab733` passed before the advisory refresh; branch `42989368984` at `4371af21` and report HEAD `42990147933` at `9ed94120` both failed the component security step on Ubuntu and Windows. Later format, Clippy, tests, architecture and release steps were skipped, so no downstream pass may be inferred. The inherited `Unicode-3.0` `license-not-encountered` warning and separate `itoa`/`syn` patch lags are not remediation authority here.

## 2. Precise deliverable and non-negotiable boundaries

**Objective:** Replace the affected `rustix` exact pin and its resolved lock entry with **exactly `1.1.6`**, preserve the original feature set, target guard, dependencies and policy, and produce fresh successful two-host `rust-verify` evidence for the repaired **main** revision. Publish a complete canonical report. Do **not** declare R03.C1 tested or accepted from those main-branch runs.

**Permanent write ledger — four and only four paths:**

| Path | Exact allowed operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | At issuance only: commit the complete **Owner-installed** `SOLO-WO-062 R01` file. Thereafter freeze it; do not edit, mark ACCEPT or neutralize it. |
| `crates/solo-adapters/Cargo.toml` | Change exactly one existing dependency-version token, `version = "=1.1.5"` to `version = "=1.1.6"` for the Linux-gated `rustix` dependency. Preserve `default-features = false`, `features = ["fs", "std"]`, all comments, formatting and all other manifest bytes. |
| `Cargo.lock` | Generate a valid Cargo-native locked resolution for `rustix 1.1.6`. Its `rustix` `[[package]]` record may change **only** `version` and the authentic registry `checksum`. Every other package identity, version, source, checksum and dependency edge must stay identical. A further change is a STOP/escalation item, not implied authority to upgrade other components. |
| `.documents/Reports/execution-report.md` | Overwrite the sole canonical factual report, in a **report-only final commit**. No second report or archive. Record the early inherited-red gate accurately. |

All other paths are **read-only**. In particular: `.documents/WIP/plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`; every Rust source/test file; all other Cargo manifests; `.cargo/config.toml`; `rust-toolchain.toml`; `deny.toml`; `.github/`; `.gitignore`; `AGENTS.md`; About/Architecture/SOP; `platform/`; all fixtures/assets; any other repository. Do not change `itoa`, `syn`, `windows-link`, `windows-sys`, their graph edges or any release pin. No new file or branch is authorized except disposable scratch in the two established temporary areas. No generated source code, patch carrier, local override, `[patch]`, alternate registry, ignored advisory or CI workaround is permitted.

**Version-control permissions:** On the single genuine checked-out `main`, permit staging **only** the above phase-specific paths, creating ordinary non-amended commits, pushing the same existing `origin/main` by normal fast-forward push, and fetching/verifying commits, branches and Actions. There is **no** authority for a PR, branch creation, merge commit, rebase, squash, cherry-pick, reset, `git clean`, force, branch deletion, parallel worktree, extra checkout, tag, release, deployment or any write to the retained `solo-wo-061-r01` branch. If protected branch policy or repository state prevents safe ordinary commits/pushes, STOP and report instead of circumventing it.

**Publication boundary:** A green security-remediation revision on `main` proves the repaired baseline; it is not evidence that the R03.C1 code on a separate, older branch was built, tested, accepted, published or merged. R03.C1 will need a separately Owner-issued, history-preserving branch reconciliation and full fresh Ubuntu/Windows validation after this order is independently accepted. This order does not authorize that reconciliation.

## 3. Execution sequence, including the expected pre-fix red gate

### 3A — Mandatory safety preflight, no writes

1. Read the required documents and verify the actual checkout root `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, its Git root, exactly one worktree, exactly one `origin`, owner URL and `main` branch. Run status checks including untracked files, pending merge/rebase/cherry-pick state, and tracked/untracked inventory. Confirm no unauthorized working-tree changes. The **only** permitted initial tracked difference is the Owner-installed complete `SOLO-WO-062 R01` replacing the stopped ACTIVE file. No unexpected modifications are safe to discard.
2. Fetch remote refs normally; verify local main and `origin/main` are exactly `9ed94120824bccc0b9bcba081544dc7cd8d11b95`, tree `371d4198643a67f282bd763ab35b7342909b7196`, and expected four WIP/source/lock/policy/workflow blob IDs. Verify the retained local and remote branch agree at `4371af21dce67f33cf1df0384738bf353ca80fc6`; do not switch to, alter or prune that branch.
3. Verify the Owner-installed ACTIVE contains `OWNER-ISSUED — SOLO-WO-062 R01`, the matching handoff and exactly the approved scope. This file is the sole authority; the preceding STOPPED work order is preserved in Git history, not kept as a second active order.
4. Independently inspect official current RustSec advisory metadata and the published `rustix 1.1.6` crate/version/feature support. Verify the locked graph's relevant dependency role and the unchanged `deny.toml` security semantics. If the prescribed `1.1.6` fix is withdrawn, yanked, inapplicable or requires another dependency version, **STOP before changing code** and request fresh Owner authority.

### 3B — Issue this order, commit ACTIVE only

1. Stage and commit **only** the Owner-installed `.documents/WIP/workorder-active.md` on `main`. Keep NEXT neutral and ROADMAP/OUTLOOK unchanged. Verify the commit's changed-path list and exact GitHub readback; do not edit the order content after issuance.
2. Push normally to `origin/main` and trigger/inspect the unchanged two-host `rust-verify` run at this exact issuance revision. Because the baseline is demonstrably still pinned to vulnerable `rustix 1.1.5`, its component-security step is **expected to fail** with `RUSTSEC-2026-0486` on both hosts. **Do not report this as PASS.** This order explicitly permits advancing to the pre-authorized dependency repair **only if** the gate's sole material failure is that exact inherited advisory, with no additional failures or unexpected findings. If new relevant advisories, protection errors, changed locks, anomalous skips or unrelated failures appear, STOP, preserve all evidence and do not continue automatically.
3. Record issuance SHA/tree, changed path, both job IDs and logs. The early security failure is an **inherited pre-fix observation**, not permission to disable, skip, narrow or mask `cargo-deny`.

### 3C — Minimal manifest and lockfile repair on main

1. On clean `main` at the verified issuance tip, edit exactly the existing Linux `rustix` requirement `=1.1.5` → `=1.1.6`. Keep all other lines untouched, especially the target-gating, `default-features = false`, `features = ["fs", "std"]`, and Windows implementation.
2. Use the pinned toolchain's Cargo and the official registry to resolve precisely that package: `cargo update -p rustix --precise 1.1.6` (an equivalent package-disambiguated Cargo-native precise operation is allowed if necessary). Do **not** hand-invent a checksum, rewrite the full lockfile arbitrarily, use an unlocked product build or update unrelated crates. Verify the graph is still the same 22 packages; inspect the exact before/after `Cargo.lock` diff. Only the rustix version/checksum may differ. If Cargo needs any other package change, or resolution cannot produce a real checksum, STOP and report it. Do not silently select another patch release.
3. Check `cargo metadata --locked` and the Linux target graph for one `rustix 1.1.6`, correct `fs`/`std` features, the same inward-only dependency graph and no extra version. Verify the target-gated Windows graph remains valid and version-unchanged for Windows dependencies. Check no architecture or source policy was inadvertently altered.
4. Fresh A1 triage: official RustSec and OSV/GHSA/vendor notices for the patched `rustix` and every relevant locked component; patch release origin, licence, feature/API compatibility, MSRV against pinned Rust 1.99.1, Linux `openat2` confinement path, new or remaining advisories, status of the inherited `Unicode-3.0` licence warning (keep `UNVERIFIED` unless specifically proven), and remaining non-actionable `itoa`/`syn` patch lag. Record current sources and actual queries, not assumed clean results. Do not suppress warnings or modify the security policy.
5. Stage **only** `crates/solo-adapters/Cargo.toml` and `Cargo.lock`; commit on `main` and push by ordinary fast-forward. The substantive commit must change exactly these two files; no source/test or STOPPED-branch commit may enter main.

### 3D — Repaired main CI, mandatory all-green threshold

1. Fetch exact substantive SHA and tree from GitHub. Trigger or inspect its automatically initiated `rust-verify` on **both `ubuntu-24.04` and `windows-2025`**. All mandatory applicable steps must **PASS** (fresh advisories, bans, licences, sources; locked fetch; formatting; strict Clippy; full workspace/all-target tests; fail-closed architecture/refusal self-tests; tripwire; release compilation; real binary CLI positive/negative smoke; Linux byte-identical two-build check). Host-specific counterpart skips are acceptable only when documented and unchanged.
2. Confirm the advisory no longer appears for the locked `rustix 1.1.6`; inspect the actual security gate logs and fresh DB commit/query time on both hosts, not merely overall green badges. Check the source-reader confinement and R01/R02/R03.A/R03.B regression suites, and any test failures introduced by the dependency change. If any required gate fails or is inappropriately skipped, **STOP**; no policy suppression, unapproved code changes or force continuation.
3. The old R03.C1 branch is **not** part of these repaired-main CI runs. Do not copy its source into main or claim its R03C1-01–12 tests have passed.

### 3E — Single canonical report-only final commit

1. After all relevant repaired-main CI jobs are green, overwrite **only** `.documents/Reports/execution-report.md` with this order's objective, exact SHA/tree/branch ledger, lockfile difference, A1 evidence, mandatory host/job/step/log outcomes, the pre-fix red issuance run, the patched main outcome, warnings and limitations, preserved retained branch, temporary-cleanup evidence and next independent Architect review gate.
2. Do **not** put the report's own as-yet-unknown final commit SHA into its content; provide it only in the chat handoff after publishing. Do not create a report backup, WIP report folder, task-named log, status archive, or append a historical diary to ROADMAP.
3. Commit the report file **alone**, push main normally and require a new full successful **two-host CI** at the exact final report SHA. Read back the report blob and verify the exact remote tree, branch list and four WIP file blobs; ACTIVE must still hold the frozen `SOLO-WO-062 R01`, NEXT must still be neutral, ROADMAP/OUTLOOK unchanged, and retained R03.C1 branch tip unchanged. Report the new SHA in the chat, not self-ACCEPT.
4. Finish with an empty `git status --porcelain` and `--untracked-files=all`, one real worktree, no unneeded task-owned scratch. If the final report-head CI or a repository-safety check fails, report STOPPED honestly at the safe point rather than claiming completion.

## 4. Exact acceptance matrix — SOLO-WO-062 R01

| ID | Required verifiable outcome |
| --- | --- |
| S062-01 | Exact starting main/tree/branch/remote/WIP/lock/policy anchors and only Owner-installed ACTIVE initial difference; one checkout/worktree. |
| S062-02 | Issued order committed alone; expected pre-fix two-host RustSec failure accurately recorded, with no other new material failure; no gate bypass. |
| S062-03 | Only the exact Linux `rustix` version token changes in `crates/solo-adapters/Cargo.toml`; feature set, target guard and comments unchanged. |
| S062-04 | Cargo-native lock resolution chooses exact `rustix 1.1.6`, authentic checksum, same other 21 package records, no new graph components. |
| S062-05 | Official `RUSTSEC-2026-0486` patch requirement checked, fresh patched dependency A1/risk/compatibility review; no applicable material advisory remains. |
| S062-06 | `deny.toml`, CI, toolchain, all other manifests/dependencies, source/test/product/assets unchanged; no waiver, skip, `#[ignore]`, override, unpinned release or new project files. |
| S062-07 | Both hosts run and pass the complete unchanged mandatory suite at repaired substantive main SHA, including Linux confinement regressions and both hosts' security gates. |
| S062-08 | R03.C1 retained local/remote branch equals original tip `4371af21…`, five source/test path diff preserved, not merged, rebased, updated, pruned or claimed tested. |
| S062-09 | Exactly four WIP files remain: frozen ACTIVE 062; neutral NEXT; Stage 03/R03.C unchanged ROADMAP; OUTLOOK unchanged. |
| S062-10 | Final report-only commit from green substantive main and fresh two-host green CI at exact final HEAD; correct one canonical report, truthful warnings, honest nonlocal limits. |
| S062-11 | Clean single worktree, safe task-temp cleanup/retention, root ignored live `target/` preserved, inherited `.temp/` archives preserved, no unrelated deletion. |
| S062-12 | No second order or Stage 03 closure, no R03.C1 acceptance, no release/deployment, no policy weakening, no unauthorized branch/PR or other-repo writes. |

The initial pre-fix CI failure in S062-02 is **expected and disclosed**, not marked PASS as a gate. `ACCEPT` of this order requires full substantive-main and final-report green results. `NOT RUN`, `UNVERIFIED`, and `NOT APPLICABLE` must be used honestly. A safe STOP is preferable to concealing a new problem.

## 5. Dependency reachability, safety and regression expectations

`rustix` is a **normal Linux-target** dependency of `solo-adapters`, not a dev-only crate; it powers the accepted native `openat2` confinement boundary in `crates/solo-adapters/src/source_read_native.rs`. `windows-sys 0.61.2` remains the separate Windows native root. The advisory's named xattr functions are not known Solo call sites, yet unchanged `cargo-deny` policy applies the advisory to the locked package. A patch must preserve no-follow, beneath-root, no-magic-link and no-cross-device file-open semantics. Windows should show a fixed/reviewed lockfile while compiling its original Windows path unchanged. No new `unsafe` permission or alternate source reader may be added. The approved product architecture remains four crates with inward-only dependencies and complete test-suite gates.

The R03.C1 branch's five implementation/test paths, which are outside `main`, must remain preserved for a later order to reconcile by **separately authorized** history-preserving Git mechanics and retest all twelve R03C1 groups against the patched main. A dependency patch on main does not automatically transitively retest an earlier branch. No independent `ACCEPT` of R03.C1 is possible until such new execution evidence is reviewed.

## 6. Mandatory scratch, security and reporting discipline

- All permanent edits remain inside the verified Solo root and exact four-path ledger. Temporary disposable work may occur only within the accepted Solo `.temp/` or the actual Cline-provided task workspace. Inventory task-created items, delete them when no longer needed and report any unremovable task-owned residue. Never delete unknown Owner content.
- Preserve the inherited **289,253,806-byte** `.temp/` archive baseline (`architecture-fixtures/` and `toolchain-installer/`), subject to its previous explicitly documented later cleanup trigger. Do not delete/modify the live ignored root `target/` rust-analyzer/build cache. Do not modify `.gitignore`, local excludes or Git config to hide scope violations.
- Do not create another checkout/worktree/clone, new WIP file, archive or parallel report. The only permanent report is `.documents/Reports/execution-report.md`. The previous execution report remains recoverable from Git history, not a new filename.
- Do not expose secret material or full private terminal paths in the public report; safe categories and observed counts suffice. No credentials, billing, DNS, hosting or external account management permitted. Official public crate/advisory network queries necessary for the pinned repair are allowed; no provider action or purchase is authorized.
- Git ownership, rights, remote branches, status and content must be independently checked at each phase. If user-owned material or unexpected divergence is found, STOP, preserve and report; never `reset --hard`, `clean -fdx`, rebase, force push or make a second worktree as a workaround.
- A1 for this **code-bearing dependency update** is mandatory. Preserve supply-chain integrity, registry checksums, known license obligations and provenance; interpret a new current advisory as a new blocker until independently triaged, not an excuse to ignore security. The inherited `Unicode-3.0` unused-allowance warning remains visible and `UNVERIFIED` unless evidence proves otherwise; do not suppress it.

## 7. Completion state and handoff

At a successful completion, main contains only (i) the issued replacement ACTIVE, (ii) the exact two-file `rustix` security fix and (iii) the canonical report revision; both substantive and report-only final revisions have green Ubuntu/Windows `rust-verify` evidence. The `solo-wo-061-r01` branch is retained at exactly `4371af21dce67f33cf1df0384738bf353ca80fc6` and still **unaccepted**. ACTIVE stays `SOLO-WO-062 R01` frozen; NEXT stays neutral; ROADMAP continues to list R03.C under OPEN Stage 03; OUTLOOK is unchanged. The Implementation Agent sends the exact published report SHA/tree, scoped diff, locked Rustix version/checksum, A1 findings, CI run/job IDs, branch preservation proof and honest temporary-workspace ledger to the Chief Architect.

**The Implementation Agent shall not self-ACCEPT.** The Chief Architect independently fetches the canonical report first at the agent's published final SHA, reviews precise changes and both-host logs, then issues `ACCEPT`, `REVISE` or `STOP`. Only after **Owner-authorized subsequent work** may the previously stopped R03.C1 branch be safely reconciled and its complete unrun tests executed. No automatic platform/graphic-role fallback, artwork, catalogue or Stage 03 closure is authorized by this maintenance order.
