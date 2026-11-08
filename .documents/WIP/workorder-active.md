# Foundation Solo — Active Work Order

## SOLO-WO-041 R01 — R01.C Accepted Security-Package Administrative Closure

**Status:** OWNER-ISSUED only when the Owner installs this entire file and gives the matching handoff.  
**Matching Owner handoff:** `SOLO-WO-041 R01`  
**Owner instruction / closure decision:** “proceed until R01.C is closed” (2026-11-08). This approves **R01.C package-level reconciliation only**; it does not authorize independent Stage 01 closure or a release.  
**Repository / branch / remote:** `provelopment/provelopment-foundation-solo` / `main` / sole `origin`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Required starting GitHub HEAD:** `a2870910c8bf7ed67d784e8a2f6251ab7486c829`  
**Required starting tree:** `b8ed0c8d00df54f30093b3cf3122641117f33776`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Close **only the remaining R01.C work package** in the live ROADMAP after the Chief Architect's independently reviewed acceptance of its implemented security and CI evidence at the required starting HEAD. This is a **documentation and WIP administrative closure**, not a new scan implementation, not an upgrade, not the Stage 01 exit-gate closure, and not authorization to implement Stage 02. The accepted R01.C deliverable is the complete fail-closed security, advisory, component provenance and version-aware two-host CI infrastructure. Historical evidence remains in existing Git commits and the one overwritten canonical report; never create an archive, register or second report.

**Exact permitted permanent path-and-operation ledger:**

| Path | Operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **OWNER-INSTALLED first:** commit this entire Owner-installed order **byte-identically** as issuance history. **Later, after the evidence gate and ROADMAP reconciliation**, replace it with exactly the neutral one-line heading `# Foundation Solo — Active Work Order` followed by one LF. Its issuance-history commit preserves the complete order. This two-phase operation is specifically authorized here; do not amend the approved order in between. |
| `.documents/WIP/plan-roadmap.md` | **EDIT Stage 01 only:** delete the sole `- **R01.C:** ...` remaining-package bullet **in its entirety** and remove the immediately orphaned `**Remaining packages**` label/spacing; preserve the Stage 01 heading, prerequisite and exact existing Stage 01 **Exit gate** text and every Stage 02+ line byte-identically. Do not replace deleted text with completion history or a progress note. |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the single canonical report with this run's actual R01.C closure evidence, scope audit, exact commits/trees, complete two-host CI, warnings, pending Stage 01 exit review and cleanup. Do not create another report or cite its own final commit in its body. |

**All other permanent paths are read-only**, including NEXT `plan-next.md` (retains its neutral heading), OUTLOOK, README, AGENTS, About/Architecture, both SOPs, `Cargo.toml`, `Cargo.lock`, every crate manifest/source/test, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/dependabot.yml`, `.github/workflows/rust-verify.yml`, `.github/SECURITY.md`, LICENSE and notices. Do not alter repository settings, action pins, runners, scanner versions/digests, dependency versions, Rust toolchain, schedule, vulnerability gate or any security rule. No additional application functionality or feature shell.

**Separation of completion:** This order retires the **R01.C package bullet only**. The **Stage 01 heading and its complete exit gate remain present** pending a separately authorized Stage 01 closure review. NEXT stays neutral; do not promote Stage 02 merely because the last named Stage 01 package has been accepted. A separate Owner decision is required to close Stage 01 or issue Stage 02. No parallel WIP files or status prose.

### 2. Chief Architect's reviewed R01.C evidence to re-check, not blindly inherit

The Chief Architect independently inspected the exact baseline and found the R01.C technical obligations supported **for package-level closure**, subject to the Agent re-verifying the evidence and new-run gates:

1. **Actual locked graph:** four first-party unpublished crates and 11 direct/transitive registry crates in immutable `Cargo.lock` blob `05a02666894f4f0bf5528acf118a049e5edb853e`; the existing single `deny.toml` blob `3063e864c28be1e77787f43f1b6f561d543c8f4a` includes all normal/build/dev graph targets and all features, `advisories`, `bans`, `licenses`, `sources`, strict source/wildcard/malware rules and no blanket ignore/skip/exception. The documented Unicode-3.0 attribution/NOTICE obligation is a **later release prerequisite**, not proof of a release or a reason to suppress a gate.
2. **Pinned supply-chain identities:** Rust toolchain `1.99.1` (`rust-toolchain.toml`), immutable `actions/checkout` commit `3d3c42e5aac5ba805825da76410c181273ba90b1` (v7.0.1, `node24`, `persist-credentials: false`), `ubuntu-24.04` / `windows-2025`, pinned `cargo-deny 0.20.2` with Ubuntu archive SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f` and Windows `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`. Its GitHub-official asset-ID acquisition fallback is conditional, HTTPS-only, bounded, digest-checked, and fail-closed; no credentials or third-party mirror. Independent acceptance of `SOLO-WO-039 R01` established that correction.
3. **Actual live CI:** source and final runs of `SOLO-WO-040 R02` passed on **both hosts**. At final `main` HEAD `a2870910...`, [run `42484385102`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42484385102) passed Ubuntu job `128742967887` and Windows job `128742967782`. Both logged real verified scanner identities and fetched RustSec database `309392b18f6b17022f82c701f300433126901068` on 2026-11-08; all four checks passed against the unchanged locked graph. Ubuntu's intentionally vulnerable `time@0.3.46` fixture **actually refused** `RUSTSEC-2026-0009`, and the malicious-import refusal, architecture graph and byte-identical clean builds all passed. Windows passed every host-applicable mandatory check. These are baseline facts to re-check at the new source and final revisions, not a waiver of another run.
4. **Vertical-slice and engineering foundation:** `crates/solo-application/src/bootstrap_preflight.rs` declares and implements real inbound `BootstrapPreflightV1` and typed accepted/refused outcomes; `crates/solo-adapters/src/bootstrap_cli.rs` calls it; `crates/solo-cli/src/main.rs` composes the actual adapter and service. Hosted CI runs real positive/refusal CLI smoke, locked fetch/build/lint/test, architecture checker/29-case self-test, failure tripwire and two byte-identical release builds. This is a **minimal engineering preflight** and is **not** a published EFS application, HTTP website, browser test or released product. Browser/HTTP smoke stays `NOT RUN` until separately authorized.
5. **Routine and currency controls:** `rust-verify` runs on push, schedule `17 4 * * *` and manual dispatch; `.github/dependabot.yml` provides weekly proposal-only Cargo and GitHub Actions checks, and the permanent A1 §7B(e1/e2) security/version policy plus Implementation SOP §7B covers installed tools, support, warnings and later safe upgrades. The first Dependabot scans were observed; their scheduled Monday occurrence has not yet been observed. RustSec is freshly fetched per CI run; the known `ubuntu-26.04` runner evaluation and optional `rust-toolchain` Dependabot ecosystem remain separate future proposals, not silently upgraded. No untriaged applicable critical/high or known malicious dependency finding was observed at the baseline; any new finding must fail this closure.

**Not automatically passed:** GitHub security-alert API access and independent hosted runner internals may remain `UNVERIFIED`; do not convert these into `PASS` or override a new actionable warning. Re-check currently available authoritative evidence, security advisories and the actual locked graph. If a material unknown, security finding, broken mandatory gate or architectural discrepancy prevents a defensible package closure, **STOP** and publish the factual blocker; never remove R01.C to make a roadmap look complete.

### 3. Frozen GitHub baseline and preflight

Before writes: read `README.md`, `AGENTS.md`, Architect SOP §§6–8 (especially §6B, §7A, §7B(e1/e2), §8), Implementation SOP §§4–5C, 7–9, the exact ROADMAP Stage 01, all four WIP files, the published report, `Cargo.lock`, `deny.toml`, current workflow and Dependabot file. Confirm single intended Solo checkout/worktree, branch `main`, sole correct `origin`, starting remote HEAD/tree, no stash, and a clean index except the **one expected Owner-installed replacement of ACTIVE**. STOP on unexplained drift, dirty Owner material or a different order/handoff. Record all exceptions rather than discarding Owner data.

**Required unchanged baseline blobs at starting HEAD:**

| Path | Git blob |
| --- | --- |
| `.documents/WIP/plan-roadmap.md` | `e6ea66cffa14d75598ce051b284f4ce12a16f67b` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/WIP/workorder-active.md` **before Owner installation** | `c1654c5f58150f104c38acf1052f43745dd68778` |
| `.documents/Reports/execution-report.md` | `79b1eb48ea0faa724f1180bf56fa8d057824128d` |
| `.github/workflows/rust-verify.yml` | `5426c1f5da6a084a9fa41e70b2d85d8741578d08` |
| `.github/dependabot.yml` | `1874726fc2406575605b3c7ac467f45402fb7f13` |
| `Cargo.lock` | `05a02666894f4f0bf5528acf118a049e5edb853e` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.documents/SOP/sop-architect.md` | `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` |
| `.documents/SOP/sop-implementation.md` | `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |
| `.documents/Architecture/ARCHITECTURE.md` | `4b050f6345db8730c5ca818cddd7129544ab8865` |
| `AGENTS.md` | `6eae1da8c721e13cea0c6689a1665e98fa5a9abb` |
| `README.md` | `ce2d1cf9aee4a1805772dc797f437d081bd00540` |

`.documents/WIP/` must contain exactly the four required files, with no nested WIP directory. The entire Owner-installed `SOLO-WO-041 R01` ACTIVE file must be staged and committed byte-identically and stay unedited through that issuance commit. All other baseline blobs must remain identical through the final report-only HEAD except the specifically authorized ROADMAP and ACTIVE mutations.

### 4. Exact execution order and acceptance gates

1. **Preflight and issuance history.** Verify §3. Stage/commit/push the **complete installed** `SOLO-WO-041 R01` ACTIVE blob alone to `origin/main`; read back its exact blob and commit. Do not edit approved scope before this historical issuance is safely recorded.
2. **Independent evidence gate before pruning.** At the current published checkout and exact applicable CI revision, inspect the actual locked components and all four policies, pinned tools/actions/runners, source and security-scan results, release/supply-chain provenance, applicable security advisories/warnings and the real application-port/CLI refusal evidence. Confirm all R01.C security and version-monitoring requirements are satisfied and there is **no material untriaged vulnerability or failed/missing mandatory gate**. Record `PASS/FAIL/UNVERIFIED/NOT APPLICABLE` with reasons, including browser `NOT RUN`. Do **not** equate a green badge with a comprehensive security guarantee.
3. **Prune only R01.C.** Only after #2 passes, delete exactly the R01.C bullet and its now-orphaned `**Remaining packages**` label from ROADMAP Stage 01. Do not alter the Stage 01 prerequisite/exit gate, any Stage 02+ item, NEXT or OUTLOOK. Commit/push just the ROADMAP change, and independently verify the raw diff. This is the explicitly Owner-authorized **R01.C package-level closure** action. It must not be described as a Stage 01 closure.
4. **Two-host closure-run verification.** Inspect real `rust-verify` at the new ROADMAP-closure source commit: `ubuntu-24.04` and `windows-2025` both **success**, all host-applicable steps passing with live digest-verified `cargo-deny 0.20.2`, freshly fetched RustSec, advisories/bans/licenses/sources, locked dependency fetch, package graph, strict lint/test/build, architecture and refusal/tripwire/smoke, plus the Ubuntu `RUSTSEC-2026-0009` refusal and byte-identical clean builds. Inspect raw logs and all available warning/annotation evidence. No untriaged actionable deprecation/version or security warning. On failure leave ACTIVE installed and report `FAILED` rather than clearing it or treating the ROADMAP mutation as accepted; do not hide a failing check or repeatedly publish report commits to manufacture green.
5. **Neutralize ACTIVE, then report.** Only after #4 passes and ROADMAP has exactly the intended diff, replace ACTIVE with exactly the neutral heading `# Foundation Solo — Active Work Order\n` (real LF newline, **not** literal backslash-n), stage/commit/push this path only. Then overwrite/publish only `.documents/Reports/execution-report.md` as the final report-only HEAD. Verify **both** `rust-verify` hosts at that exact final HEAD; inspect logs/annotations and report actual outcome. An unrun or failed final gate is not a closure `PASS`. Confirm the entire WIP directory inventory, unchanged NEXT/OUTLOOK, absence of `R01.C` in ROADMAP, preservation of Stage 01 exit gate and all Stage 02+ material, and clean local status. If the final run fails, report that fact and do not self-ACCEPT; remedy requires a separate Owner decision.
6. **Reporting and handback.** The report states work order and matching Owner handoff, baseline/source/final commit and tree identities (the report may not embed its own commit), the single package-level closure and explicit Stage 01 **OPEN**, complete allowed-path and Git blob diff, evidence for each R01.C gate with freshness and limits, CI source/final run/job IDs, warnings/skips, remaining release-only NOTICE and future runner/upgrade proposals, verified neutral ACTIVE and WIP purity, and safe cleanup of task-owned files in Solo `.temp/` or Cline-supplied temp. State the exact final commit/tree and give **“agent report ready”** in chat for independent Chief Architect review. An Agent `COMPLETE` is not Architect `ACCEPT`.

### 5. Permissions and stop conditions

**Permitted only:** read the intended public Solo repository and authoritative public security/version sources, use task-owned disposable scratch in the SOP-permitted locations, perform the single Owner checkout's safe Git staging/committing/pushing **only the three §1 paths**, and inspect GitHub jobs. No branch/PR creation, merge, force-push, reset, clean, history rewrite, second checkout/worktree, new dependency/toolchain installation or upgrade, provider/credential/secret access, settings change, release, deployment or changes to any sibling repository. All temporary files must be safely cleaned without deleting unknown data. The normal Git mechanics are delegated to the Implementation Agent under Architect SOP §6B.

**STOP** on any baseline, Owner authorization, source SHA, branch, worktree or file-ownership mismatch; inability to substantiate the R01.C acceptance conditions; an unresolved material warning or security finding; unexpected ROADMAP work; missing/red mandatory CI; any change outside the exact ledger; or attempts to close **Stage 01** or promote **Stage 02** under this package-only authorization. Never make the roadmap look complete by suppressing a gate, inventing an acceptance or moving a failed obligation into OUTLOOK.

### 6. Exact Owner handoff (copy separately)

> Align with README.md and AGENTS.md, then execute work order SOLO-WO-041 R01 at .documents\WIP\workorder-active.md.
