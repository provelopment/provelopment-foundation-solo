# Foundation Solo — Active Work Order

## SOLO-WO-045 R03 — R02.B First-Slice CI Edge-Count Reconciliation and Independent Acceptance

**Status:** `OWNER-ISSUED` — complete, final replacement scope.  
**Matching Owner handoff:** `SOLO-WO-045 R03`  
**Replacement authority:** R03 supersedes the stopped `SOLO-WO-045 R02` work order in the one ACTIVE slot. The two STOP reports and the previously published R02 code stay in Git history. R03 does not amend R01/R02 in place, waive their failed CI, or retrospectively claim the R02 implementation was accepted.  
**Owner direction:** Advance the earliest open Stage 02 obligation through a rigorously bounded CI reconciliation, independent acceptance of the already committed R02.B first slice, and orderly handoff of the still-open R02.B remainder.  
**Repository:** `provelopment/provelopment-foundation-solo` · existing `main` · sole `origin` `https://github.com/provelopment/provelopment-foundation-solo.git`. Verify the actual Solo root and the one-checkout discipline; do not assume an available Windows path.  
**Required starting remote HEAD:** `d6c0bc764de7e7bc0e954963a70621be2a2a3a40`  
**Required starting tree:** `7ad47ed2e7e801c0b2ac9c8fcbda20404deafed7`  
**Stage and package:** Stage 02 — One-Entity EFS identity and publication authority; R02.B **first bounded slice only**. **R02.B and R02.C remain open** after this order; Stage 02 is not closed.

### 1. Approved Scope — exact permitted permanent operations

**Objective.** Correct a stale **workflow verification expectation**, not an architectural failure: the R02-authorized normal dependency edges `solo-adapters → serde` and `solo-adapters → serde_json` made the real-tree graph report `architecture=legal packages=4 targets=16 files=26 edges=24`; the existing two-host workflow still requires `edges=22`. Both hosted jobs failed on those read-only hardcoded assertions, after their source checker, full tests and security checks passed. Restore strict *load-bearing* CI by changing only those two expected literals, preserve all real graph and source enforcement, and obtain full hosted CI/security/refusal/reproducibility evidence at the unchanged implemented R02.B source. Do not present a syntactically parsed candidate as published content.

**Only these four paths may differ from the starting HEAD as a consequence of R03:**

| Permanent path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Commit this complete Owner-installed R03 order byte-identically as one standalone issuance commit**, replacing the previously installed R02 order. Do not modify the issued text. **Only after independent Chief Architect ACCEPT and successful §4 closure gates**, neutralize ACTIVE to the exact UTF-8 `# Foundation Solo — Active Work Order` followed by one LF — 40 bytes, Git blob `ea5f20b736483c2e102456be1cea9104059fc81a` — as its own commit. |
| `.github/workflows/rust-verify.yml` | **Exactly two literal substitutions, no other change:** Linux real-tree architecture check `grep -q 'edges=22'` becomes `grep -q 'edges=24'`; Windows real-tree check `$text -notmatch 'edges=22'` becomes `$text -notmatch 'edges=24'`. Retain the `architecture=legal packages=4` checks, real checker execution, strict failure propagation, output/regex logic, the original error/throw behavior, all step names, security/smoke/reproducibility tests and the mandatory **23 refusal / 5 legal** matrix unchanged. No generalization, deletion, skip, warning suppression or masking is authorized. |
| `.documents/Reports/execution-report.md` | Overwrite the **one** canonical report after the repair evidence run, and again at final reconciliation; truthfully record the STOP histories, original red CI and this order's verified runs. No second report, archive, completion fiction or self-referential report-commit SHA. |
| `.documents/WIP/plan-next.md` | **Only after independent Chief Architect ACCEPT** of the now fully green R02.B first slice: populate one bounded **remaining-R02.B** candidate for actual safe source reads/confinement, effective candidate selection and the still-unproven semantic validations. Mark readiness on direct evidence, not on a future plan. Never make NEXT executable or mark R02.B closed. |

**Everything else is permanently read-only.** In particular: `crates/solo-cli/tests/workspace_graph.rs`, all Rust source and test files, `Cargo.toml`, `Cargo.lock`, every crate manifest, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, every other `.github/` file, `README.md`, `AGENTS.md`, About, Architecture, both SOPs, `.documents/WIP/plan-roadmap.md` and `plan-outlook.md`, licence and notices. No R02.B code rewrite, new dependency, version or toolchain change, CI action pin change, provider, API, HTTP/browser, real publication, release or deployment. No extra WIP files or checkout.

**Frozen code under review, not a new code authorization.** The R02 implementation commit is `cf15cd5377663145e349f5344dedd593146f6396` / tree `1ba765005219a0935d8b3d9804deb645af5ac0e6`. It supplied the inert `ParsedManifestCandidateV1` adapter parser, pure lexical `SourceLocator` domain model, new public-API tests, and a narrowly reviewed adapter-normal `serde 1.0.229` / `serde_json 1.0.154` graph exception. R03 may test, inspect and accept or refuse those committed bits but **must not edit any of them**. If the complete hosted run or independent review reveals a code defect, return STOP/REVISE for a separately scoped correction, not an out-of-ledger fix.

### 2. Exact baseline, safety and preflight

Read root `README.md` and `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `ARCHITECTURE.md` §§1–1.6, 5/5A, 6.2A–6.2C, 6.2J–6.2K, 6A.2–6A.3, both SOPs (Architect §§6–8, Implementation §§4–5C and 7–9), all four WIP files, the entire current canonical report, relevant code/tests/dependency manifests, the current workflow, and the complete installed R03 order. Establish real root, `main`, sole approved `origin`, exactly one worktree, zero stash, no nested/shadow checkout, no unrelated dirty, untracked or Owner-owned material. The **only expected local tracked change at preflight** is Owner's installation of the exact R03 order over ACTIVE; the remote `main` must still have the stated starting HEAD/tree. Never reset, force-clean, amend, rebase or rewrite published history.

**Required starting blob ledger (at remote `d6c0bc7`):**

| Path | Exact Git blob SHA-1 |
| --- | --- |
| `.documents/WIP/workorder-active.md` (old R02) | `1953f0fb39f37b4911e2855d6bcc566a37f5a775` |
| `.documents/WIP/plan-next.md` (neutral) | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` (R02.B/C open) | `18975332c2820170a32558c4ee224176667b91ab` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `7ffb81dcb34672ad8b1fb77cbedff28cb8198af3` |
| `.github/workflows/rust-verify.yml` | `1c0e450c0a4a50b6f22f547d8a7c646dce8b79b1` |
| `Cargo.toml` | `14505be49a281ba0495eabea5ca386548cc36347` |
| `Cargo.lock` | `36125ff1be3eec79b30c5370a7ac4e2642e5d10d` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `crates/solo-domain/src/source_locator.rs` | `2b507a8abfe36da05625b562ae22c916754a4687` |
| `crates/solo-domain/src/lib.rs` | `c50eeea611328da7409ba351c88671b2be21241a` |
| `crates/solo-domain/tests/source_locator.rs` | `412ba016588eda7f446e70c4ea473d85fbf8e485` |
| `crates/solo-adapters/Cargo.toml` | `0e37bc5e3ac40f163bd396e3db8ee94af3e53745` |
| `crates/solo-adapters/src/lib.rs` | `ea788668182de006598ef9d917e98097a2065fca` |
| `crates/solo-adapters/src/publication_manifest.rs` | `f883dd93e8da6a04c3d8b7df7072c2ec0dde6a26` |
| `crates/solo-adapters/tests/publication_manifest.rs` | `2fe055a2d9c237afe63698958b28a5ea7fbdc75f` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `774d49fbc901807e3f0f7201a300528fc8772d6c` |
| `crates/solo-cli/tests/workspace_graph.rs` | `9d9d21579ca8abdf7d8eac345a681b878ab9115b` |
| `.documents/Architecture/ARCHITECTURE.md` | `43edcc02ff4c794f91a42098a797ea1060e2635b` |
| `.documents/SOP/sop-architect.md` | `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` |
| `.documents/SOP/sop-implementation.md` | `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |

Verify the whole Git tree before and after, not only `git status`. WIP has exactly four files, no subdirectory. `plan-next.md` is intentionally neutral while R03 is active. ROADMAP must retain R02.B/R02.C and all Stage 02–13 material **byte-identically throughout**. Note: the R02 source is already published at its exact blob IDs above; the current failure is not evidence that those paths are yet independently accepted.

### 3. CI repair and required conformance evidence

**One tiny, reviewable workflow patch.** At the first R03 workflow-change commit, the entire changed diff of `.github/workflows/rust-verify.yml` must consist of exactly these two line replacements, each preserving surrounding shell and existing refusal behavior:

```diff
-          grep -q 'edges=22' "$RUNNER_TEMP/arch-real.txt" \
+          grep -q 'edges=24' "$RUNNER_TEMP/arch-real.txt" \
...
-          if ($text -notmatch 'edges=22') { throw "approved edge count changed" }
+          if ($text -notmatch 'edges=24') { throw "approved edge count changed" }
```

This is **not** a waiver or exception to structural policy: the real-checker **must run**, independently assert the legal four-crate ownership graph, and print `architecture=legal packages=4 targets=16 files=26 edges=24` at the tested commit; the workflow refuses any different count. The `workspace_graph.rs` two-owner invariant, enumerated approved adapter closure, negative graph fixtures, AST checker, 23 forbidden fixture tests, no-default-allow and no outward/dependency escape remain enforced **without editing them**. If the graph ever changes again, a fresh code/security review is necessary; `24` is not a generic acceptance threshold.

**Fresh, complete two-host push CI after the patch is mandatory**, including the steps the prior red runs never reached. Verify at the exact revised workflow/source commit and at the published evidence-report HEAD:

- locked acquisition, no unexpected lock or workspace change, pin/toolchain assertion (including separate measured `rustup` version), format, strict Clippy with `-D warnings`, full all-target workspace tests and newly introduced parser/locator/regression tests, actual real-tree checker;
- all **29 architecture self-test cases** including exactly **23 prohibited forms refused** and **5 legal twins accepted**, no `FAIL`, the source/layer checker-bypass and dynamic-load probes, and injected forbidden import with verified restore;
- deliberate CI test-failure tripwire must refuse the injected failure; release build and positive/refused built-CLI smoke must run and pass on both hosts; **Ubuntu** two clean release builds must compare byte-for-byte;
- fresh two-host `cargo-deny 0.20.2` (advisories, bans, licenses, sources) and disposable `RUSTSEC-2026-0009` negative advisory probe, correctly classified source/advisory freshness and zero applicable unresolved critical/high or malicious component. Evidence must inventory all **15 locked packages**, including the change in **runtime reachability** of `serde` and `serde_json`; unchanged names/versions are not proof that an expanded attack surface can be ignored;
- exact action SHA, Node runtime, Rust/Cargo/rustup/runner versions and current support/advisory evidence, actual annotations, errors, warnings and skipped-vs-host-conditional steps, no warning suppression or invented scan success. A green status without first-hand gate inspection is insufficient.

**Prior red run evidence is retained.** R02 code run `42553390272` and STOP-report run `42554037361` are **FAILURE** on both hosts at the stale `edges=22` assertion. They prove only that earlier steps ran; all downstream hosted gates were `NOT RUN`. The **R03 issuance commit** runs under the **still-stale** workflow and may legitimately show precisely the same known failure; it is not a new implementation error and must be reported as **expected red**, never green. The **first workflow-fix commit** must be green on both hosts; further pushes must remain green. If it fails for any new reason, STOP truthfully and do not edit unapproved paths or relabel it accepted.

**Pure first-slice contract, not product publication.** Independently verify and report the code already published under R02: strict duplicate-member rejection before value materialization (including nested arbitrary duplicate keys), closed five-key single manifest with exact `schemaVersion: "1.0.0"`, size/UTF-8/BOM refusals, one nested Entity with accepted ID grammar/cardinality/scoped uniqueness, inert bounded `properties` and declared `sourceCandidates`, portable logical `pages/` locators and duplicate reservation even for absent candidates. Verify valid minimal/expanded declarations and negative fixtures. The only public parser outcome remains an **untrusted, non-publishable `ParsedManifestCandidateV1`**, no filesystem reads, effective JSON/Markdown selection, canonical BCP47/Unicode, runtime routing or publication. Actual source no-follow confinement, resolved-file/case/hardlink collision, snapshot/atomicity, selected source body correctness and further semantic/schema/URL/property/visibility conformance are still future R02.B work. Do not claim C1 or C2–C4 complete.

**Known limits remain limits.** Local test execution may be supplemented but never fabricated; raw log access errors, GitHub alert endpoints returning 401, scheduled Dependabot `NOT OBSERVED`, and missing browser/HTTP/deployment checks must remain `UNVERIFIED`, `NOT OBSERVED` or `NOT APPLICABLE` as appropriate. Release-only `unicode-ident` Unicode-3.0 attribution/NOTICE remains open. Ubuntu `rustup 1.29.2` versus Windows `1.29.3` has previously been a low-urgency non-blocking parity proposal, not an authorized upgrade here. Classify true measured values and version currency, do not assume patches match.

### 4. Required execution sequence and independent disposition

1. **Preflight and R03 issuance:** Confirm matching Owner handoff and §2 baseline exactly, then commit/push the complete, byte-unchanged Owner-installed R03 ACTIVE order **alone**. Verify canonical GitHub blob and issuance tree, and record the push-run's likely expected failure due to unchanged `edges=22` (inspect that it is the same mechanism; if not, investigate and STOP). Do not treat a pre-repair red issuance as permission to weaken or skip tests.
2. **Repair only the workflow:** Commit/push **only** the precise two substitutions in §3. `git diff --check`, exact before/after inspection, unchanged workflow lines/steps and every other baseline path are required. Verify the resulting GitHub HEAD, the two changed lines and mandatory green Ubuntu/Windows CI through **the entire workflow**, including all gates previously skipped after the failed architecture step. No source/Cargo/security/config edit is authorized.
3. **Publish technical evidence:** Overwrite/push **only** `.documents/Reports/execution-report.md` with the R01/R02 STOP history, the existing R02 code SHA/tree and locked dependency graph, R03 issuance SHA, workflow-fix SHA/tree/blob, all test/negative/security/version and two-host run/job IDs, exact outcomes, warnings, limitations and cleanup. No self-referential report HEAD in its own body; the handback provides it. Verify both-host green CI at the report HEAD. Report status `COMPLETE — evidence phase only`, if true; otherwise `STOPPED`/`FAILED`. End that first handback with `agent report ready`.
4. **Independent Chief Architect review:** The Architect checks the published canonical report at exact GitHub HEAD, workflow diff, unchanged implemented code, real binary/test/refusal evidence, security exposure, and job logs, then returns **ACCEPT / REVISE / STOP**. **Do not self-accept the source, close R02.B, neutralize ACTIVE or populate NEXT before ACCEPT.** An ACCEPT here is for the bounded **first slice** only, not the complete R02.B or Stage 02. REVISE/STOP leaves ACTIVE issued, NEXT neutral, ROADMAP unmodified, and a truthful canonical report.
5. **After ACCEPT, reconcile only the accepted slice:** ROADMAP stays byte-for-byte unchanged (R02.B/R02.C open). Populate `plan-next.md` with **one** dependency-ready **remaining-R02.B** candidate: capability-confined no-follow source access, valid candidate source selection with absence-vs-error and JSON-first, resolved collision refusal, body validation/snapshot, and the outstanding parser/schema/Unicode/URL/property/visibility contract proofs. Never claim already executed R02.B or make NEXT executable. Commit/push **only NEXT**, verify exact diff and both-host CI.
6. **Neutralize ACTIVE and publish final report:** After accepted NEXT reconciliation and green CI, restore ACTIVE to the **exact 40-byte** neutral blob `ea5f20b736483c2e102456be1cea9104059fc81a`, in its own commit. Overwrite/push only the final canonical report with complete accepted evidence, exact path ledger, unchanged R02.B ROADMAP, R02.C, still-open Stage 02, R01/R02 defects retained in history, verified security/warnings/limits, and temporary cleanup. Verify final remote HEAD/tree, both-host full CI/logs/annotations at HEAD, precisely four WIP files/no subdirs, a clean local `main`, one worktree, zero stash and no task-owned scratch garbage. Give final commit/tree and run/job URLs **in chat**; conclude `agent report ready`. **Do not commence the next remaining-R02.B order.**

### 5. Explicit permissions, exclusions and STOP conditions

**Authorized Git operations:** stage/commit/push only the four paths in §1, in the §4 sequence, using the existing `main` and sole approved origin, without any PR, merge, branch, extra checkout/worktree, stash, amend, rebase, force-push, destructive reset/clean, tag, release or manual workflow rerun/dispatch/cancel. Existing push-triggered CI is authorized. Temporary task data only in the permitted Solo `.temp/` or the actual Cline-provided task temp area; safe cleanup of task-owned material is mandatory, unknown/Owner-owned material is preserved.

**No additional approval gate** is required for the already enumerated post-ACCEPT continuation inside this one final issued order. The Chief Architect's independent ACCEPT is the order's technical evidence gate, not a substitution for Owner issuance. A separate new order and matching handoff will be necessary before executing the next remaining-R02.B slice.

**Mandatory STOP:** wrong root/remote/HEAD/blob or unmatched handoff; unexpected Owner-owned/untracked material; a needed edit outside the four paths; any change to R02 source/deps/manifests/security controls/ROADMAP; altering `edges=24` into a non-failing condition or weakening the source/graph checker; newly red mandatory CI; non-deterministic/ambiguous source or parser safety; untriaged applicable critical/high or malicious component; unknown registry/security/runner impact; hidden warning or skipped gate; executable publication inferred from parse; inability to verify clean diff, provenance, temporary cleanup or final two-host CI. Record unresolved defects in the canonical report, preserve the issued order and the remaining ROADMAP obligations; never assert ACCEPT merely to move work forward.

### 6. Exact Implementation Agent handoff

> Align with README.md and AGENTS.md, then execute work order SOLO-WO-045 R03 at .documents\WIP\workorder-active.md.
