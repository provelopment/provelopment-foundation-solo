# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-048 R04**  
**Title:** Correct R02.B protected staging integrity, confinement and transaction refusal  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Execution branch:** `main`; one verified checkout and worktree; sole remote `origin`.

## Approved Scope — SOLO-WO-048 R04

### 1. Objective and success condition

Correct the three independently identified defects and evidence gaps in the R03 disk-backed source-stage implementation **without abandoning its one-selected-body-at-a-time architecture**:

1. **M7 — integrity:** A staged body must never yield silently accepted changed bytes. Prove the relevant Linux and Windows writable-handle, file-sharing, mapped-view and same-principal/path-adversary boundaries. A removed filename, a read-only path permission or a returned noncryptographic FNV value is not sufficient proof of immutability.
2. **M8 — confinement:** Root adoption and every staging operation must use a trustworthy, retained directory/object identity and reject path-component redirection, root replacement and symlink/reparse-point attacks. A `symlink_metadata` check on the final path followed by later pathname operations is insufficient.
3. **M9 — failure atomicity:** Exercise real staging transaction code under deterministic injected write/short-write, flush/close-or-finalization, capacity, permission, and late-member failures. Return one accurate registered refusal; expose no partial candidate; retain no failed transaction's accessible staged bytes; never delete an unowned object.

**Implementation strategy:** First establish whether the preceding guarantees can be demonstrated with existing approved Rust standard APIs and the already pinned dependencies/features, entirely inside the path ledger below and without adding project-owned `unsafe`, FFI, new crates/features or a diagnostic contract. If demonstrably feasible, implement the smallest correct Option A repair and prove it on both real hosts. If not feasible, **STOP** and publish the single canonical report identifying the precise missing host capability or permission. Do **not** invent Option B native privileges, weaken the threat model or replace the staging design. A new scoped Owner decision will govern any subsequent native-security replacement.

**Preserve:** the R03 sequential `drive_declared_source_set_v1` protocol, full-declaration physical reservations before any body read, selected-only JSON-first read and preflight, complete-or-refused transaction, per-file host snapshot protections, bounded RAM independent of total selected bytes, no added aggregate source-size ceiling, existing 2 MiB manifest / 2 MiB JSON / 1 MiB Markdown and EFS limits, and all accepted security, identity and provenance behavior. Preserve the existing staged candidate's non-publishing and non-global-snapshot semantics: `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false`, `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false`.

**Excluded:** R02.C implementation or planning transition; ROADMAP/NEXT/OUTLOOK updates; ACTIVE neutralization; publication, route/asset/module/render/HTTP work; new dependencies, native features, project-owned unsafe/FFI, schema/diagnostic registry changes, release, merge or deployment. This order covers the **technical correction and report only**. R02.B stays OPEN until independently technically accepted and separately administratively closed.

### 2. Frozen permanent-write ledger — eight paths

Only the following permanent file operations are authorized. A path listed as optional may remain untouched. Do not create another permanent work order, WIP file, report or source module.

| # | Repository path | Permitted operation |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | Commit the complete installed **R04** text in a single ACTIVE-only transition commit. Retain R04 unchanged through the technical report. Do not update execution/status prose inside it. |
| 2 | `.documents/Reports/execution-report.md` | Overwrite the **one** canonical report with factual evidence, including STOP. Commit/push it alone after the last substantive commit; verify the published report and two-host run. |
| 3 | `crates/solo-adapters/src/source_set_staging.rs` | Correct trusted staging-root/object lifetime, write→sealed-read integrity, fallible commit/finalization, precise refusal mapping and bounded retrieval. No extra public authority or new unsafe. |
| 4 | `crates/solo-adapters/tests/source_set_staging.rs` | Add real Linux/Windows hostile-handle, path/reparse, confinement and cleanup adversaries; deterministic *production-path* stage I/O and finalization failure injection; preserve existing positive tests. |
| 5 | `crates/solo-adapters/src/source_set_candidate.rs` | Narrow changes needed to consume the corrected staging transaction, protect member ranges, propagate staged-read/refusal failures and require fallible all-member finalization. Keep original source selection, metadata and body limits unchanged. |
| 6 | `crates/solo-adapters/tests/source_set_candidate.rs` | End-to-end actual adapter whole-set refusal and integrity tests with failure injected **after** one or more successful members; verify cleanup, no partial return and original-source independence. Retain existing R03/R01 tests. |
| 7 | `crates/solo-adapters/src/lib.rs` | **Optional**, only minimal module/public-export adjustment forced by items 3–6. No new privilege, dependency or unsafe allowance. |
| 8 | `crates/solo-adapters/src/source_read_native.rs` | **Optional**, a narrowly isolated **safe-Rust-only staging helper using existing pinned/approved APIs** if indispensable; do not change native source-body open/clone/read/recheck semantics, platform features, unsafe regions, FFI imports or the accepted D2/Windows security proofs. STOP if genuine native/unsafe expansion is needed. |

Every other tracked path is **read-only**, especially `crates/solo-application/**`, `crates/solo-domain/**`, `crates/solo-cli/**`, `crates/solo-adapters/src/source_read.rs`, `crates/solo-adapters/src/publication_manifest.rs`, `crates/solo-adapters/src/source_body_preflight.rs`, all `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.github/**`, `.documents/Architecture/**`, SOPs, README, AGENTS, ROADMAP/NEXT/OUTLOOK and all sibling repositories. If a reliable correction needs a ninth permanent path or an incompatible API change outside this ledger, STOP and state exactly why. Test-only throwaway programs and fixtures may use only the SOP-permitted Solo `.temp/` and current Cline-provided task scratch; they are not permanent deliverables and must be cleaned by known ownership.

### 3. Repository preflight and exact anchored baseline

Verify the actual Solo repository root (the Owner-local Windows path is `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`), remote URL, `main`, `HEAD == origin/main`, one worktree/checkout, clean state **except the installed ACTIVE replacement**, and exactly four regular WIP files. Read `README.md`, `AGENTS.md`, both SOPs, the relevant Architecture sections and the complete current ACTIVE work order before implementation. No second checkout/worktree, shadow source tree, stash, reset, `git clean`, force push or history rewrite.

**Reviewed anchor:** GitHub commit `9f587abb89e63067d96b1bc27ac53e497e5f50f5`, tree `073388ff8a3acf229005469a074725a10a43dfb4`. Verify this commit is an ancestor of the pre-transition `main` HEAD. Every intervening committed change prior to the ACTIVE transition must affect **only** `.documents/Reports/execution-report.md`; otherwise STOP. The canonical report is overwritten every run, so its blob and HEAD/tree may change through report-only commits. Never demand exact equality to the anchor HEAD/tree or to the previous report blob.

At pre-transition committed HEAD, verify the following frozen substantive blobs; compare other tracked paths against the anchored tree. The installed local ACTIVE holds this complete R04 order; its **committed** pre-transition predecessor is R03's `11d9e3071f3016bf9afa90327cc786d282a50d79`. Do not ask for an expected hash of R04 from inside R04 itself.

| File | Expected committed blob |
| --- | --- |
| `.documents/WIP/workorder-active.md` | `11d9e3071f3016bf9afa90327cc786d282a50d79` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `18975332c2820170a32558c4ee224176667b91ab` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Architecture/ARCHITECTURE.md` | `250bf62efd07037fe70022b5fc88a63610692245` |
| `.documents/SOP/sop-architect.md` | `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` |
| `.documents/SOP/sop-implementation.md` | `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |
| `AGENTS.md` | `6eae1da8c721e13cea0c6689a1665e98fa5a9abb` |
| `README.md` | `ce2d1cf9aee4a1805772dc797f437d081bd00540` |
| `Cargo.lock` | `8bd0bb00286a97853624ebbc0fe18da42a733f12` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` |
| `.github/workflows/rust-verify.yml` | `eb78dad240bb7adb3e0ef9535870897176c99843` |
| `crates/solo-adapters/src/source_set_staging.rs` | `e88b2b5dfb3073168e4abb15792a86bc62675087` |
| `crates/solo-adapters/tests/source_set_staging.rs` | `ad25e90c33c596a67daecc606cc360412f845d61` |
| `crates/solo-adapters/src/source_set_candidate.rs` | `0b10fec2409baa27cb9c18326ebd7f5077f6a1f5` |
| `crates/solo-adapters/tests/source_set_candidate.rs` | `4c3cbad0aa274871689f031c105dbafd2d7cb779` |
| `crates/solo-adapters/src/lib.rs` | `d1c26d6be992c96cb938e10650f179a4769fbf2c` |
| `crates/solo-adapters/src/source_read_native.rs` | `787a6842ca617999ddeffcda14b4a8851e8b0189` |

### 4. Implementation steps — feasibility first

**I. Record the proof target before code.** Map the existing `StagingRootCapability::adopt/begin`, `SourceSetStagingV1::open_object/stage_body/reader/Drop`, `StageReaderV1::read_chunk/read_whole/matches`, and the candidate transaction `stage/finish`. State the precise supported threat boundary separately for Linux and Windows. Include (a) untrusted ancestor components and path replacement between verification and use, (b) existing writable handles, pathname aliases, inherited handles, and mapped views, (c) the distinction between no directory entry and no writable alias, (d) the semantics of flushed/finalized bytes and subsequent reads, and (e) cleanup after error, unwind, and process termination. A directory mode, `canonicalize`, final-component `symlink_metadata`, name deletion, `File::set_permissions(readonly)` or noncryptographic FNV on its own is not a proof. Validate actual host behaviors with isolated, owned scratch tests where needed. Do not claim universal same-user process isolation where the OS does not guarantee it.

**II. Feasibility decision — existing capabilities only.** Identify concrete standard-library or already-authorized safe API operations sufficient to pin the staging root and to prevent an unobserved post-validation mutation of every staged member on both hosts. Check the exact writer-handle, name-unlink, read-only handle transition and Windows file-sharing/mapped-view semantics; document the lifetime and race proof. Include how errors during the last write, length verification and whole-set finalization are refused rather than accidentally committed. If there is no complete, testable proof using permitted APIs, **STOP at this gate** without product source changes. Publish a factual `STOPPED` report identifying the smallest missing operation/permission, the relevant host and the test/contract requirement. Do not add crypto as a substitute for protected storage and do not trial unapproved FFI/unsafe through a new module.

**III. Implement only a proven solution.** Make the smallest modifications in the ledger to retain an owned, trusted location/object identity throughout the staging transaction, protect staged bytes against the tested threat model, and ensure successful reads are unchanged or return a typed failure. Keep the existing body-memory bound and original-source read guarantees. Do not reopen original authored files for completed candidate consumption. Use checked offset arithmetic and fallible bounded reads; when an unexpected read returns short/changed bytes, refuse rather than return a successful truncated body. Preserve existing accurate diagnostic codes, without manufacturing a new unregistered token.

**IV. Prove real transaction failures.** Add a deterministic fault-injection seam that exercises the production stage/commit code (not a separate fake algorithm) at an arbitrary later-member write, short write, flush/sync-or-equivalent, stage finalization, and cleanup step. Verify each result: correct typed code, no completed candidate, no accepted subset, earlier staged bytes inaccessible, only exclusively owned material cleaned, external sentinel unchanged. Exercise root/path race and writable-view/handle adversaries on the real supported hosts. Where a host test cannot obtain a necessary privilege, report that portion as **UNVERIFIED**; do not relabel a Linux symlink or a synthetic error-code mapper as a Windows reparse or transaction failure test.

**V. Preserve and regress.** Keep the 14 existing R03 M1–M14 logical, source, provenance, D2/Linux, Windows mapped-view and bounded-RAM tests executing. Do not weaken, ignore, delete or narrow an applicable failure to achieve green. Verify all source limitations remain: source-stage only; not global atomic snapshot; not publishable. No full-size disk-fill or unsafe OOM experiment is required: safe bounded fault injection supplies capacity/IO negatives.

### 5. Acceptance matrix — all mandatory

| Gate | Required independent evidence |
| --- | --- |
| **F1: real-handle integrity** | On actual Ubuntu and Windows, attempts using a pre-existing or acquired writable alias, a live writer and supported mapped-view cases cannot produce a **successful changed** staged read. Prove prevention or correct fail-closed detection from the actual consumer API; a renamed file with the old pathname is not enough. |
| **F2: retained confinement** | A stage-root ancestor symlink/reparse, root replacement between adopt and `begin`, and namespace-path substitution cannot redirect accepted writes or cause deletion of unowned data. Demonstrate with the real host where supported; report a privilege-bound unexecuted test as unverified, not pass. |
| **F3: protection lifecycle** | Read-after-write, sealing/transition, repeated bounded reads and candidate drop behave correctly without exposing write access or silently accepting post-validation mutation, including at a shard boundary. Exact original byte length and content remain attributable to the member. |
| **F4: injected I/O** | Inject write error and short write after one or more valid staged members; inject insufficient capacity/quota, permission and flush/finalization errors through the production staging path. Verify exact registered codes and zero public candidate. Mere pure `ErrorKind` classification is insufficient. |
| **F5: all-or-refused and cleanup** | A later failure or panic/unwind makes prior staged bytes inaccessible and deletes only owned names/handles; a pre-existing sentinel, adversary-created file and Owner-owned path remain untouched. Document abrupt process termination and remanence accurately; exercise it if safe/available. |
| **F6: unchanged authored policy** | All R03 M1–M14 (including Linux D2 Btrfs/tmpfs proof, Windows original-source sharing/mapping, JSON-first and unselected Markdown, full reservation, provenance, body bounds and peak-RAM scaling) still pass on both relevant hosts. No global snapshot or publication authority. |
| **F7: source and supply chain** | All implementation edits in eight-path ledger; no crate/feature/lockfile/workflow/checker/SOP/README/architecture/roadmap/next modifications; no new `unsafe`/FFI; architecture and unsafe boundaries pass. |
| **F8: two-host end-to-end gates** | Full `rust-verify` green on Ubuntu 24.04 and Windows 2025 at final implementation HEAD and report-only HEAD; verify actual raw logs, test names, warnings, full locked security checks and negative probe, architecture `packages=4` / selfcheck 29, format, Clippy, tests, release CLI smoke and reproducibility. |

For any applicable F1–F8 gate not actually executed or proven, record **FAIL** or **UNVERIFIED** and STOP. Do not claim COMPLETE from green workflow alone. On actual success, report `COMPLETE` for the R04 technical scope only; the Chief Architect independently decides `ACCEPT / REVISE / STOP`.

### 6. Execution sequence, permissions, reporting and STOP

1. Verify §3; commit and push only the installed R04 ACTIVE replacement, then obtain green two-host `rust-verify`. Preserve ROADMAP, NEXT and OUTLOOK.
2. Perform §4 proof-first feasibility using owned disposable scratch. If feasibility fails, publish a STOP report only; do not fabricate a native fix. If proof succeeds, implement only the §2 ledger changes and run §5 acceptance tests.
3. Commit and push the completed substantive changes; obtain full Ubuntu and Windows green `rust-verify` at their exact implementation HEAD. Overwrite the canonical report, commit/push **that file alone**, obtain two-host green CI at report HEAD, and independently verify the report blob and final remote HEAD. Send the final HEAD and evidence to the Owner for Chief Architect review; do not self-approve technical completion.
4. On STOP or red mandatory gate, preserve completed evidence, keep R04 ACTIVE, ROADMAP/NEXT neutral and R02.B OPEN. Publish the factual single report when safely possible. State the smallest precise missing native permission/API or blocker, without creating a new work order or implementing Option B autonomously.

**Repository writes:** allowed only for the eight ledger paths. **Commits and pushes to `origin/main`:** authorized solely for the sequence above, under Architect SOP §6B and repository safeguards; no force push or history rewrite. **Temporary files:** permitted under Architect SOP §6A / Implementation SOP §5A for positively owned task scratch; clean promptly and report host-local material truthfully. **Dependencies:** locked fetch/build/test and the already pinned toolchain and security tools only; no package/feature/lockfile updates. **Network:** approved GitHub/Cargo/security advisory access strictly required by the existing verification pipeline; no new service/provider connections. **PR creation/merge, release, deployment, secrets access, destructive modifications to unknown files, other repositories:** not authorized.

**Report to `.documents/Reports/execution-report.md`:** exact order identity; anchor/actual initial HEAD/tree, intervening-report-only ancestry, R04 ACTIVE transition; changed paths and exact commits/trees; Option A host feasibility proof and rejected mechanisms; F1–F8 with REAL/INJECTED/PURE classification and test names; relevant before/after code, controlled attacks, OS versions, error tokens and failure cleanup; bounded memory/source-policy regression; supply-chain/advisory findings; exact two-host run/job IDs and warning/skip inventory, including any red intermediate run; one checkout/remote and four WIP files; temp cleanup and original-source privacy; `COMPLETE` or `STOPPED` and open risks. Do not place private absolute paths, secrets, raw authored bytes or unredacted sensitive findings in the public report. Never include the report commit's own SHA in its contents; give the actual final HEAD in Owner-only chat after publication.

**Immediate STOP:** identity/committed baseline divergence other than report-only advancement; unexpected local writes or changed protected path; lack of a host-proven integrity/confinement primitive; inability to run mandatory real-host adversaries; change requiring new native/unsafe authority, registered error/contract, dependency/feature or ninth permanent path; any mutable staged bytes accepted as unchanged; inability to guarantee complete-or-refused cleanup; any failed critical test or security gate; or an unexpected cleanup/destructive-write risk. Do not close R02.B, prepare/start R02.C, neutralize ACTIVE, merge, release or deploy.

**End of SOLO-WO-048 R04.**
