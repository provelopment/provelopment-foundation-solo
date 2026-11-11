# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-063 R01**  
**Title:** Recover, validate and conditionally integrate R03.C1's authored graphic-obligation inventory on the accepted patched `rustix 1.1.6` baseline  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner handoff ID:** `SOLO-WO-063 R01`  
**Approved Scope:** The Owner expressly authorizes the **bounded R03.C1 recovery** described in this frozen order: replace accepted maintenance order `SOLO-WO-062 R01` in the one ACTIVE slot; preserve the original, unaccepted `SOLO-WO-061 R01` source branch; reproduce its five-path R03.C1 code on a **new short-lived recovery branch** based on patched `main` by an expressly permitted, history-preserving **cherry-pick** of the one identified source commit; apply only strictly necessary, bounded R03.C1 code/test corrections in those same five paths; execute the full accepted R03.C1 matrix and fresh two-host CI; and, **only if all gates pass**, fast-forward integrate to `main` and publish the sole canonical execution report. This is **not** an authorization to complete or close R03.C, to close Stage 03, to design or deliver artwork/media/catalogue, to change dependencies/policy, to release or deploy. The Chief Architect has **GitHub read-only** authority; only the separately handed-off Implementation Agent may perform the authorized repository writes.

## 1. Accepted authority, precise initial state and intended outcome

**Binding authorities (read before work):** repository root `AGENTS.md`; `.documents/SOP/sop-architect.md` especially §§6, 6A, 6B, 7/7A/7B, 8; `.documents/SOP/sop-implementation.md` especially §§4–5C, 6–11; `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2G, 6.2I and 6.2K; accepted one-Entity EFS ancestry and R03.A/R03.B property contracts; the **frozen `SOLO-WO-061 R01`** order in Git history at issuance SHA `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` (especially its §5 named matrix); the stopped R03.C1 execution report at `9ed94120824bccc0b9bcba081544dc7cd8d11b95`; and the accepted security-maintenance report at the present starting `main` SHA. No history or NEXT entry is permission to implement except this newly Owner-issued order and matching handoff.

**Chief Architect's separate dispositions:** `STOP — SOLO-WO-061 R01` for its then-inherited RustSec failure and unrun R03.C1 tests; `ACCEPT — SOLO-WO-062 R01` for the bounded `rustix` remediation. Acceptance of the dependency repair **does not** accept the five R03.C1 files or their unrun tests. The implementation branch is preserved as source material, not as an independently accepted product revision.

**Exact remotely verified baseline, 2026-11-11:**

| Item | Required identity before the Owner-installed ACTIVE replacement is committed |
| --- | --- |
| `main == origin/main` | `084161d601577d62e85df831542e83de82628639` |
| Baseline `main` tree | `9cb58c5fad5f968294c5fcd3519cb62df647715f` |
| Accepted `rustix` substantive SHA | `65656f111b4d8ef8012fc3d394a316193a54ad59` — tree `87768ff2456ffbeb7fd9e0253d87d7f1f77f8eb9` |
| Canonical report | `.documents/Reports/execution-report.md`, blob `44d0b11c296262e658208bff1498796a349a0b7c` |
| ACTIVE | frozen accepted `SOLO-WO-062 R01`, blob `ff903266e18581ffe9c705f36792376fcae2a02a` |
| NEXT | neutral heading only, blob `92fa562b1af0c54391851aa5c79deb1631127069` |
| ROADMAP | Stage 03 OPEN, R03.C pending, blob `31a0ba2efb4f780f7248c56ff84683f20d9bc97f` |
| OUTLOOK | neutral, blob `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Adapter manifest | `crates/solo-adapters/Cargo.toml`, blob `6109224cd3e3e89487884631f2b07c68ad8149c3` — exact `rustix = "=1.1.6"` with `fs`, `std`, no default features |
| Lockfile | `Cargo.lock`, blob `2f9e53576df117f2020ae064d6973304f20833f1` — 22 packages; `rustix 1.1.6`, checksum `e5e76742e5294153d4dde1207dbec5bdd81eef6fabdc0d2118babadabfd858aa` |
| Security policy | `deny.toml`, blob `3063e864c28be1e77787f43f1b6f561d543c8f4a`, `unsound = "all"`, no exception |
| Workflow | `.github/workflows/rust-verify.yml`, blob `eb78dad240bb7adb3e0ef9535870897176c99843` |
| Ignore | root `.gitignore`, blob `f45dd49524748e9b08f6678271edfc22ac39b97f`; keep anchored `/target/` intact |
| Original retained branch | local and remote `solo-wo-061-r01` exactly `4371af21dce67f33cf1df0384738bf353ca80fc6`, tree `89c6176a5f159069eccab9eb0d4ed67e88eaba4f`, parent `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` |
| Current GitHub branch inventory | exactly `main` and `solo-wo-061-r01`, no existing `solo-wo-063-r01` |

**Baseline CI evidence:** repaired substantive `main` run `42999372565` and final report run `43000498413` are both green on `ubuntu-24.04` and `windows-2025`; they do **not** include the R03.C1 code. The original R03.C1 branch run `42989368984` failed before running tests under the prior locked `rustix 1.1.5`; the stopped source branch therefore provides authored implementation, **not** passing behavioral evidence.

**Expected post-Owner-install preflight:** the only permitted difference from the above HEAD is the Owner-installed replacement of `.documents/WIP/workorder-active.md` with the complete text of **this exact `OWNER-ISSUED — SOLO-WO-063 R01`**. Verify the matching handoff, one checked-out `main`, one genuine checkout/worktree, the sole accepted `origin` URL, no untracked/staged/unknown changes, no pending merge/rebase/cherry-pick, exact four-file WIP inventory and the unchanged source branch. If any anchor diverges or unknown material is present, **STOP**; do not infer that this order approves adapting to an unexpected repository state.

**Required result:** an independently reviewable, **nonpublishing** R03.C1 inventory candidate identical in bounded functionality to the original five-path implementation (plus any independently evidenced, in-scope corrections); fresh actual tests on both hosts at a recovered branch SHA, a fast-forward integrated `main` SHA, and the final report SHA; preserved original branch and clean one-checkout state. R03.C still requires later verified asset reading, media-byte and catalogue work. Stage 03 remains OPEN.

## 2. Frozen operation, path and branch-permission ledger

**Authorized permanent path and operation, and no others:**

| Path | Exact permitted operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner installs this complete issued order, then Agent verifies and commits this file **alone** at issuance; leave its text frozen thereafter. No self-ACCEPT or ACTIVE clearing. |
| `crates/solo-application/src/graphic_asset_inventory.rs` | Recover the exact source-branch module; only tightly necessary R03.C1 contract/test corrections if evidence requires them. |
| `crates/solo-application/src/lib.rs` | Recover exactly the R03.C1 module declaration and public exports; change only if required by an in-scope correction. |
| `crates/solo-application/tests/graphic_asset_inventory.rs` | Recover and run the original meaningful R03C1 application tests; only fix incorrect in-scope tests without loosening requirements. |
| `crates/solo-adapters/tests/property_resolution.rs` | Recover and run the original four real strict-JSON R03C1 cases; preserve all prior R03.B fixtures/tests. |
| `crates/solo-domain/tests/property_resolution.rs` | Recover and run the original three R03C1 domain cases; preserve previous assertions. |
| `.documents/Reports/execution-report.md` | Overwrite the **single canonical factual report** after integration; publish as report-only commit. If STOP, publish a truthful STOPPED report on `main` when safe. |

**No NEXT transition is necessary or authorized:** `.documents/WIP/plan-next.md` is **already neutral**. `.documents/WIP/plan-roadmap.md` retains R03.C and Stage 03; `.documents/WIP/plan-outlook.md` stays untouched. There must be exactly four files in WIP and no duplicate report or archived work order. There is **no** authorization to edit Architecture, About, SOPs, README, AGENTS, any other code/test path or any other repository.

**Forbidden changes:** `Cargo.toml` files (including the patched adapter manifest), `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/`, `.github/`, `.gitignore`, toolchain/CI/security settings, manifests/config outside this exact ledger, external dependencies, root `LICENSE`, `platform/`, assets, artwork, generated images or media catalogue, graphics contract versions, routes, publication/HTTP/renderer, alternative fallback, contact or other unrelated product functionality. No dependency or RustSec waiver, ignored test, test deletion, masked CI error, fake verified asset, edited security advisory database or invented publication result.

**Authorized Git operations:** within the **one** genuine local Solo checkout, after clean-root preflight, ordinary `fetch`/remote verification; commit and push the Owner-installed issuance file; create **one** new short-lived branch `solo-wo-063-r01` from the exact new issuance `main` SHA; **explicitly cherry-pick only original commit** `4371af21dce67f33cf1df0384738bf353ca80fc6` onto that new branch, preserving the old branch/ref and published history; commit/push bounded five-path corrections if needed; dispatch/read GitHub CI against the **exact recovery branch SHA**; and **only after fully passing gates**, use **fast-forward-only** integration of that new recovery branch into `main` and push it, followed by the one report-only commit/push. No PR, squash, rebase, reset, force-push, amend, merge commit, deletion of either branch, tag, release, deployment, extra remote, nested checkout, shadow clone or Git worktree is authorized. A safe clean-tree `git switch` is allowed; destructive checkout is not. Do not ask the Owner to perform routine Git commands the Agent is authorized to do.

**Retained original branch is immutable in this order:** `refs/heads/solo-wo-061-r01` **and** `origin/solo-wo-061-r01` must remain exactly `4371af21dce67f33cf1df0384738bf353ca80fc6` at end. Cherry-picking creates a **new** commit on the **new** branch and does not rewrite or fast-forward the original. The new `solo-wo-063-r01` may also remain at its accepted substantive tip pending later Owner-authorized administrative closure/pruning. No branch is pruned here.

**Other permissions:** no provider credentials, account operations, secrets, external hosted tools, purchasing, release or deployment. Read-only authoritative crates.io/RustSec/OSV/vendor/version research and normal network-enabled locked `cargo fetch`/CI operations are authorized where already supported by the accepted workflow. Per SOP §6A, disposable task scratch is permitted in the verified Solo `.temp/` or the actual Cline-provided task temp workspace, with cleanup and evidence; no new repository checkout there.

## 3. Recovery sequence — each gate precedes the next

### 3A. Safety preflight before any permanent change

1. Confirm actual root using repository tooling, branch `main`, remote ownership/URL, `HEAD`/`origin/main` and tree exactly as §1. Enumerate tracked/untracked state and worktrees. The Owner-installed ACTIVE file is the **only** permissible initial modification. Check existing WIP and report blobs, patched manifests/lock, deny/workflow/ignore hashes and both branch tips against §1.
2. Independently inspect the retained `SOLO-WO-061 R01` code and its original §5 R03C1-01 through R03C1-12 acceptance matrix. Confirm its one substantive commit has **only** the exact five paths in §2, with no added dependency, asset, licence, catalogue or publication path. The source branch is not automatically trusted because the agent previously authored it.
3. STOP if either branch or `main` has diverged, a second checkout/worktree exists, unknown dirty/Owner-owned files are at risk, the exact `rustix 1.1.6` baseline is absent or the new installed `OWNER-ISSUED` identity/handoff mismatches. No reset, clean, stash or unapproved reconciliation.

### 3B. Publish issuance; require repaired-baseline CI to stay green

1. Stage/commit **only** the Owner-installed `.documents/WIP/workorder-active.md` from `main` and push via the accepted sole `origin`. Verify new remote HEAD, tree and ACTIVE blob, with NEXT, ROADMAP, OUTLOOK and Cargo files unchanged. Record exact issuance commit and tree.
2. Require the normal `main` push `rust-verify` run at this exact issuance SHA to finish **success on both** `ubuntu-24.04` and `windows-2025`, including a fresh RustSec check and all mandatory steps. Unlike the previous security-remediation order, a newly failed security gate is **not expected or permitted** to be waived. Record the CI run/jobs, warnings and counterpart-host skips before continuing.
3. Do not cherry-pick while the issuance gate is red or the main/remote state is uncertain. STOP and report accurately instead.

### 3C. History-preserving R03.C1 recovery branch

1. From clean issued `main`, create **new** `solo-wo-063-r01` at that issuance SHA; confirm it is not a second worktree and the original branch still has the exact untouched source tip.
2. Cherry-pick **only** `4371af21dce67f33cf1df0384738bf353ca80fc6` onto the new branch. Because the original code commit changes only five paths and patched `main` changed only authorization/report/manifest/lock, the expected operation is clean. Record the original SHA, new cherry-pick SHA, the source and destination parents and the exact patch/path comparison. If any conflict, unexpected path, unexplained textual drift or unresolved Git state occurs, **STOP** without inventing conflict choices or discarding data.
3. The first recovered code commit must have exactly the original five technical paths, with the original content of those paths reproduced. The expected source-branch blob IDs are:

   | Recovered path | Original source branch blob |
   | --- | --- |
   | `crates/solo-application/src/graphic_asset_inventory.rs` | `7de9bc7399854b2059cbec17ee23c1e1ae637159` |
   | `crates/solo-application/tests/graphic_asset_inventory.rs` | `49b0038d49277862329b00d9c8da9fdd172467a3` |
   | `crates/solo-application/src/lib.rs` | `1f5115569b0c65bef3e9f44abd6a4a022f9fd081` |
   | `crates/solo-adapters/tests/property_resolution.rs` | `332372ef2f5d9ae290477ea6836f4157d5c84cd6` |
   | `crates/solo-domain/tests/property_resolution.rs` | `5fea4e4748f493bb43f7f80189081c11bb75e162` |

   Verify complete graph/output semantics against the accepted R03.C1 order rather than assuming byte identity proves correctness. New recovery-branch source must inherit **patched `rustix 1.1.6`** and every immutable §1 dependency/security/WIP baseline blob; the source commit must not restore the former `1.1.5` dependency.
4. Use the original R03.C1 tests as substantive test input, not as already-passed evidence. If CI reveals a genuinely bounded bug or a faulty expectation, repair **only** the exact five listed source/test paths, commit that correction separately on the **recovery** branch and rerun **all** required gates. Each correction must be linked to a named R03C1 matrix row with precise before/after assertions. No deleting, skipping, XFAILing, trivializing or relaxing tests, no invented media verification, and no adding behavior outside this inventory slice. STOP and seek separate Owner authority if the repair requires any forbidden path, a new dependency, another architecture decision, new production adapter code or a changed security policy.

### 3D. Fresh, complete branch validation — mandatory integration gate

1. Push `solo-wo-063-r01` ordinarily to `origin` (new branch only); verify remote tip/tree agrees. Dispatch the unchanged `rust-verify` workflow **at that exact branch ref/SHA** and confirm jobs actually checked out the intended SHA. Do not treat old green `main` runs as branch-code evidence.
2. Both Ubuntu and Windows must pass every applicable existing gate: pinned toolchain, locked fetch and exact graph, `cargo-deny 0.20.2` with **fresh RustSec**, licence/source/bans, package-set gate, `cargo fmt --all -- --check`, strict Clippy (`-D warnings`), `cargo test --workspace --all-targets --locked`, architecture checker and refusal/tripwire, locked release build, real CLI accepted/refused smoke, Linux-only RustSec negative probe and **Ubuntu byte-for-byte reproducibility**. Host-inapplicable steps may skip; any other skip, new warning, failure, ignored test or red result must be triaged, not waved through.
3. Capture exact test results on each host, including execution of named `r03c1_*` tests (not merely their names appearing in the source), and verify **all** R03C1-01…12 requirements in §4. The inherited `warning[license-not-encountered]` for `Unicode-3.0` remains **UNVERIFIED** unless separately repaired; disclose its actual presence and context, and do not suppress or relabel it as a pass. Record fresh component/security review of the exact 22-package locked graph, RustSec/OSV/vendor freshness, source access boundaries, OS-specific variants and any remaining patch lags. Material new/advisory findings stop integration.
4. No completed green two-host branch run means **no integration**. If STOP, preserve both branches, return to safe clean `main` when possible, publish one truthful STOPPED report via the standing report-only permission if safe, and request the smallest exact additional Owner decision.

### 3E. Conditionally fast-forward integrate and re-verify `main`

**This section is operative only if §3D is fully green and the exact five-path R03.C1 acceptance conditions are evidenced.**

1. Re-fetch and recheck that `origin/main` is still the exact green issuance SHA, the recovered branch is its descendant, no unexpected remote commits or branch drift exist, the original 061 branch is still untouched, and the worktree is clean. STOP on divergence; never force or rewrite history.
2. Switch safely to `main` and integrate **only** `solo-wo-063-r01` with **fast-forward-only** semantics (no merge commit and no squash). The resulting `main` tree must consist of the original patched dependency baseline plus the five authorized R03.C1 paths, and the frozen issued ACTIVE order. Push `main` ordinarily and independently read back commit, tree and changed-path diff. Neither branch is deleted.
3. Wait for the normal `main` push CI at that substantive SHA and verify **success on both hosts**, including all actually executed R03C1 tests. A failure, untriaged warning or material difference from the branch verification is a STOP, not permission to publish a COMPLETE claim or silently broaden scope.

### 3F. One canonical final report, verified at its own SHA

1. Only after green branch and substantive `main` runs, overwrite **exactly** `.documents/Reports/execution-report.md` with a factual `COMPLETE` run record: order and Owner approval, exact baseline/issuance/recovery/cherry-pick/substantive main commit+tree identifiers; the five path/blob mapping and any correction commits; test names/total counts and all R03C1-01…12 outcomes; CI run IDs, per-host job conclusions and security timestamps; A1 component ledger and warnings; original and recovery branch tips; root/worktree/remotes; source/asset nonpublication boundaries; `.temp`/Cline scratch cleanup and retained archive triggers; and explicit unimplemented R03.C work. Do not embed the final report's own future SHA in its content.
2. Commit/push **only the canonical report** from clean `main`. Independently read it back from GitHub at the resulting report SHA, verify its blob/tree and confirm **full two-host `rust-verify` success at the report commit itself**. If report CI fails, correct only within authorized scope and report truthfully; do not claim completed acceptance from an earlier green revision.
3. Final handoff names exact commit/tree/blobs, GitHub URLs, complete authorized whole-order diff, green job/step evidence, warning/skips, clean local status and retained two branches. The Agent reports `COMPLETE` or `STOPPED` factually but **never self-ACCEPTs**. Chief Architect independently issues `ACCEPT / REVISE / STOP` against the final published report.

## 4. Frozen R03.C1 acceptance matrix — no reinterpretation

The complete **`SOLO-WO-061 R01` §5 `R03C1-01` through `R03C1-12` matrix is incorporated by reference exactly**, and **every row requires fresh evidence now**. The original branch had test files, but its two-host gates never passed; do not promote any of those rows from `NOT VERIFIED` merely because the tests compile. For convenient execution, the following are the observable contracts, subordinate to the exact original matrix:

| ID | Required observation after recovery |
| --- | --- |
| R03C1-01 | Five-level original authored graphic obligations: Entity, Facet, Site, Page, representation, ordered graph-parent-first, registry-role-first, local-index-first; full qualified owners. |
| R03C1-02 | All five graphic role keys (logo, favicon, banner, background, social-preview); original source namespace/reference, exposure, local index, and role retained exactly. |
| R03C1-03 | At least two sibling Sites and two region-qualified representations; whole-graph inventory independent of requested target; no sibling leakage or inferred locale/geography. |
| R03C1-04 | Internal, unlisted and later-suppressed authored sources all inventoried; safe public projection still hides internal and suppressed values. |
| R03C1-05 | Suppress with `currentContext: "none"` produces no local obligation; suppress+local produces exactly one; no invented values from absence/`none`. |
| R03C1-06 | Same filenames at different owners remain distinct; same filename in `entity` vs `platform` namespaces remains distinct; no fallback or cross-Entity ownership. |
| R03C1-07 | Existing full-effective (pre-public-filter) singular 0–1, background 0–4 and exact duplicate-background refusals, plus legal suppress+replacement, remain intact. |
| R03C1-08 | Strict real JSON, complete owner inventory, independent sibling/representation cases; malformed declaration on unlisted/unrequested branch refuses whole candidate with typed safe cause. |
| R03C1-09 | Closed asset-reference grammar: approved image extensions, favicon PNG only; refuse external/absolute/relative escapes, malformed/encoded/active references; no media-byte attestation. |
| R03C1-10 | No publication/asset/source/network authority; repeatable nonpublishing inventory; empty and all-internal cases; no default or generated asset. |
| R03C1-11 | Existing R01/R02/R03.A/R03.B and architecture/security/negative regression suites pass unchanged; domain pure std and application only inward dependencies. |
| R03C1-12 | Full patched-baseline green issuance, recovered-branch, fast-forward-integrated substantive main, and report-head CI, both hosts, actual non-skipped test/gate evidence and full code-bearing A1 review. |

**Nonpublication and architecture contract:** The graphic inventory enumerates authored obligations, **including** those later suppressed; it never asserts that a referenced file exists, is valid media, licensed, safe, read from a confined source or present in a pinned catalogue. Binding `.documents/Architecture/ARCHITECTURE.md` §6.2I.1 prohibits **automatic platform fallback for any role**; explicit `platform` references do not convey a fallback. The pending ROADMAP wording about optional generic fallback remains unchanged and is not authority to override Architecture. `platformAssetLibraryVersion: "1.0.0"` is a contract pin, **not** proof that an Owner-reviewed `platform/assets/` catalogue/artwork has been delivered. No R03.C or Stage 03 completion is permitted within this slice.

## 5. A1, quality, version currency and test integrity

1. Before acceptance, record the complete current first-party + third-party **normal/build/dev**, host-target and transitive dependency/attack-surface inventory for the exact recovered code SHA and current patched baseline (22 locked packages unless a STOP-worthy mismatch is discovered). Use fresh official RustSec and relevant OSV/GHSA/vendor data with exact versions, dates, advisories, reachability and disposition. In particular confirm the patched `rustix 1.1.6` remains in the exact locked graph with no `RUSTSEC-2026-0486` exposure or hidden waiver; verify the CI actions/toolchains/runners and environment stability. Do not refresh dependencies as a side effect.
2. Respect `deny.toml`'s `unsound = "all"`, source/licence/bans checks and the existing inherited `Unicode-3.0` nonfatal warning; this order may document but **not** remediate or suppress its unknown cause. Independently record any remaining `itoa`/`syn` version lag as applicable; no maintenance update is authorized.
3. Validate the Ports-and-Adapters dependency direction with actual architecture checks; the recovered module is an application-level use case over existing validated property inputs, and must not reach filesystem, providers, external APIs or rendering. Test actual behavioral and refusal semantics, not only JSON lexical parsing, compilation, a green workflow badge, a `has_*` predicate or a test name. Host-specific skipped steps are `NOT APPLICABLE` for that host, not an unqualified PASS.
4. Report actual command results. When local Windows lacks `link.exe`, local test execution is `NOT RUN`; two-host GitHub CI with actual test-step logs is the requirement, not a substitute assumed from local `cargo check`. Never use stale advisory databases, `--offline` for fresh advisory acquisition, disabled security gates, ignored tests or a mock publication pipeline to manufacture success.

## 6. Repository, temporary workspace, branch safety and stop rules

**Single checkout only:** use `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; no secondary clone, shadow source tree, nested repo, additional worktree or extra remote anywhere. Preserve the live root `target/` cache and anchored `/target/` `.gitignore` rule, including all unrelated Owner material. Only clean identified task-owned scratch in Solo `.temp/` or the actual Cline-provided task temp area, retaining the inherited `architecture-fixtures/` and `toolchain-installer/` material (previous observed **2 files / 289,253,806 bytes**) unless a separately Owner-authorized cleanup trigger has been met. Do not broadly delete `.temp/`, OS temp roots or unfamiliar artifacts; record actual before/after state and any cleanup limitation.

**STOP immediately** on a mismatched Owner order/handoff, unexpected root/HEAD/branch/tree/blobs, untracked or Owner-owned data conflict, nontrivial cherry-pick conflict, extra technical changed path, security advisory or failed mandatory CI, cross-Entity/sibling leakage, internal/unlisted publication, wrong fallback, source/asset verification claim that is not proven, unsafe deployment/publication side effect, hidden use of secrets or another repository, or an R03.C1 defect needing broader design, dependency or code scope. Do not rewrite either published branch, force push, reset or clean unknown data. Preserve the original R03.C1 source and any safely published recovery progress; report exactly which named acceptance rows are still `UNVERIFIED`, the observed blocker and smallest permissible Owner decision.

**Final WIP state even after green runs:** exactly four retained files; ACTIVE still this frozen `OWNER-ISSUED — SOLO-WO-063 R01`; NEXT neutral; ROADMAP retaining the R03.C pending package and Stage 03 exit gate; OUTLOOK unchanged. Green CI is **not** administrative closure, new artwork approval, media safety proof, a release, a deployment or authorization to start R03.C2. Chief Architect may independently `ACCEPT` only this R03.C1 slice; separate Owner closure/reconciliation is required to neutralize ACTIVE, advance NEXT or prune the two retained branches.

**End of OWNER-ISSUED SOLO-WO-063 R01.**
