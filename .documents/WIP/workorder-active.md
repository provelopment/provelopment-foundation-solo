# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-051 R01**  
**Title:** Complete R02.B F2/F5 staging-security correction with real-host branch CI and verified publication on `main`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner-local root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Execution:** Single verified checkout and worktree, sole remote `origin`; transition on `main`, bounded development on `solo-wo-051-r01`, then verified fast-forward to `main`.  
**Replaces:** `SOLO-WO-050 R01`, stopped without published product edits. No R02.B or R04 F2/F5 acceptance is inherited from that run.

## 1. Required outcome — deliver the correction

**Implement, compile, test, correct and publish** the previously specified hybrid staging-security repair. **Do not end with an uncompiled local rewrite.** Use the existing GitHub-hosted Ubuntu 24.04 and Windows 2025 runners for successive real compilation and adversarial test cycles on the **single authorized development branch**. Only fast-forward a verified green candidate to `main`.

1. **Linux:** adopt the entire author-specified staging root without following any intermediate or final symlink/magic link; resolve from a held `/` descriptor with pinned `rustix::fs::openat2` (`BENEATH | NO_SYMLINKS | NO_MAGICLINKS`, **not** `NO_XDEV`). Create all staging shards directly in the retained root with `O_TMPFILE | O_EXCL`, owner-only mode, zero staging-owned filesystem names and **no name-directed cleanup**. Handle closure owns shard cleanup. Validate actual Btrfs behavior using the already-mounted CI fixture.
2. **Windows:** securely adopt the drive-qualified root by component-relative opens and reparse refusal. Create a private namespace and shards with pinned `NtCreateFile(FILE_CREATE)` **relative to held parent handles**. Keep shards inaccessible to any second reader, writer or mapper; detach each shard from its temporary name using **its own held handle** before making it usable; delete the owned empty namespace **by its own held handle**, never by name. Preserve crash, failure and foreign-impostor safety.
3. **Candidate contract:** retain the sequential `drive_declared_source_set_v1` driver, whole-declaration reservations, JSON-first selected-only preflight, present-but-invalid preferred JSON refusal, 2 MiB manifest/JSON and 1 MiB Markdown bounds, 64 MiB shard target, one selected body resident at a time, bounded peak RAM independent of total staged bytes, original physical source identity/provenance and registered diagnostics. Never reopen the original body after staging. Preserve exactly `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false`; a staged candidate does not publish anything and is not a whole-filesystem snapshot.
4. **Finish R02.B's technical correction:** close both recorded defects F2 (path-redirection) and F5 (unowned-object deletion) with real-host adversaries, including the *unowned empty-directory* replacement. No test weakening, security-contract reduction or hidden fallback.

This order implements the exact R02.B security correction. **R02.B remains in ROADMAP and R02.C is not started** until independent Chief Architect acceptance and separate Owner-authorized reconciliation. No release, deployment or other product stage is included.

## 2. Exact authorized permanent file ledger

| # | Repository path | Authorized action |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | Commit **this entire Owner-installed `SOLO-WO-051 R01` text only** in one initial ACTIVE-only transition on `main`. Freeze afterwards. |
| 2 | `.documents/Reports/execution-report.md` | Overwrite one canonical report at run end, including on STOP; final **report-only** commit on `main`. No duplicates/archives. |
| 3 | `crates/solo-adapters/src/source_set_staging.rs` | Linux no-namespace lifecycle, Windows held-namespace/shard lifecycle, fallible finalization and ownership-safe cleanup. |
| 4 | `crates/solo-adapters/src/source_read_native.rs` | Safe Linux `rustix` descriptor primitives; **sole** new Windows native/`unsafe` boundary for relative `NtCreateFile(FILE_CREATE)` and held-handle `SetFileInformationByHandle(FileDispositionInfoEx)`. Do not alter the original-source native read/clone/recheck path. |
| 5 | `crates/solo-adapters/tests/source_set_staging.rs` | Migrate every old namespace test to the new host-specific lifecycle; add exact root/adoption, Btrfs, Windows reparse, foreign replacement, failure and process-lifetime adversaries. |
| 6 | `crates/solo-adapters/src/source_set_candidate.rs` | Optional minimum stage/finalization/error integration only. |
| 7 | `crates/solo-adapters/tests/source_set_candidate.rs` | Optional full-set late-failure/atomic-refusal regression evidence. |
| 8 | `crates/solo-adapters/src/lib.rs` | Optional minimum internal test/export wiring. |

**Nothing else may be changed permanently.** Do not touch `README.md`, `AGENTS.md`, Architecture/About/SOP, NEXT/ROADMAP/OUTLOOK, `.github/workflows/**`, manifests, lockfile, `deny.toml`, toolchain, checker/fixtures, source-body reader, application/domain crates, other repositories or hosting configuration. No dependency/feature changes, ad hoc FFI declarations or ninth file.

**Additional, exact Git/GitHub authority for this order:** after the initial ACTIVE-only transition, create and push **one** short-lived ordinary branch named `solo-wo-051-r01` from verified `main` in the **same checkout**, make and push iterative commits on that branch containing only paths 3–8, invoke the existing `rust-verify.yml` `workflow_dispatch` against that branch, and inspect the actual Actions jobs/logs. The branch may temporarily fail CI; **`main` must not receive known-broken source**. The agent may use an authenticated `gh` CLI or equivalent first-party GitHub Actions REST dispatch and read operations with already available credentials. The agent may install a user-scoped `gh` CLI only from its official verified release if absent; do not expose, print or create new tokens. If dispatch is unavailable, **one conventional non-draft PR from that same branch to `main` is authorized solely to trigger the existing `pull_request` CI**, with no PR merge; verify that its tested merge tree matches the candidate tree, and still require final `main` CI. PR descriptions must not disclose confidential vulnerability details. Close that PR after successful publication.

**Exact integration authority:** after two-host success at the selected branch candidate (or tree-equivalent PR candidate), unchanged frozen paths and fresh remote-state checks, **fast-forward only** the verified branch into `main` and push `origin/main` (no GitHub PR merge, no squash, no rebase of published commits, no force). Verify the complete existing workflow succeeds again at that **exact substantive `main` SHA**, on both hosts, before publishing a `COMPLETE` report. The Owner's installation of this entire order expressly authorizes this bounded fast-forward for this order alone under Architect SOP §6B; it grants no unrelated merge, release or deployment authority. Once both final runs and report publication are verified, remove **only** the task-owned remote/local branch and close the task-owned PR if any. Never delete the branch while unaccepted work is its only remaining copy.

**Build tools:** the previously installed user Rust `1.99.1` remains available. **Local MSVC `link.exe` and WSL compilation are not prerequisites.** Do not install Visual Studio Build Tools, a global compiler or a new Linux distribution. The GitHub runners already install pinned Rust and compile both OS targets. Locked dependency fetch/security/advisory traffic already used by the workflow is authorized. The existing Ubuntu workflow creates real loop-backed Btrfs, verifies `fstype=btrfs` and exports `SOLO_TEST_SOURCE_FS_ROOT`. Do not alter that workflow or request a new Btrfs machine.

Temporary fixtures may live only in Solo `.temp/`, the active agent's own scratch, or existing per-job CI runner temporary roots; never create a clone or secondary source tree. Do not delete or modify the **2813 previously observed, unattributed** `.temp/` entries. Remove only provably task-owned scratch; report any unavoidable owned residue.

## 3. Frozen anchor and transition preflight

**Exact reviewed anchor:** `625a3798fee1177e9ba2f26aea7bb6f87621243e`, Git tree `957406a8e3b210f02caeda93b084e9ec385a0625`, prior report `STOPPED` (`.documents/Reports/execution-report.md` blob `23f2bb434dcb5c9508a122f3d10226b8001c2365`). Its parent is the `SOLO-WO-050` ACTIVE-only transition `2e6006c095887ead0b8cdab6dfa9e2089722ae98`; report-only CI run 167 was green but **proved no new security correction**.

**Frozen starting blobs:** previous ACTIVE `c1388487f7dad8cff8eee56d091ea303d59a766b`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `18975332c2820170a32558c4ee224176667b91ab`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; staging `e20054c76a4296c187d0ccbbf87601737c9c5c88`; native `ab6a392ec45d6a6b946bdf8a6e4ce9a567b3ae26`; staging tests `dd2b71f31b09c4ee1067aa98d27aef21d19fccfe`; candidate `311d44c807f7fbc55df175161054041c78012869`; candidate tests `4c3cbad0aa274871689f031c105dbafd2d7cb779`; `lib.rs` `31bee760d918e014fc6378ef40ec1e9eb058449f`. Read-only canonical Architecture/SOP and CI files remain unchanged.

1. Verify root, branch `main`, clean Git status except the **Owner-installed** exact replacement ACTIVE, sole `origin` pointing to `provelopment/provelopment-foundation-solo`, one worktree, exact four WIP files, `HEAD == origin/main`, anchor ancestry, no unauthorized drift. Read `README.md`, `AGENTS.md`, relevant Architecture, **both SOPs including Architect SOP §6B/§7B**, all four WIP files, the published `SOLO-WO-050` report and the current source/tests. A mismatch that is not exclusively allowed report-only ancestry is a STOP requiring review. Do not erase unknown files or adopt another repository.
2. Confirm the installed ACTIVE text bears this order ID and title and the Owner handoff names **`SOLO-WO-051 R01`**. Commit/push **only** ACTIVE as the first commit to `main`. Verify GitHub shows it and Ubuntu/Windows CI is green.
3. From that verified transition SHA, create `solo-wo-051-r01` in the **same** checkout. Do **not** make an additional worktree, clone, source copy, branch for each test, or a competing WIP artifact. A short-lived branch is explicitly authorized here because the previous two executions were unable to compile locally and must never test a knowingly broken `main`.

## 4. Direct implementation instructions

### 4A. Migrate the test harness before pushing source

The existing `tests/source_set_staging.rs` assumes a visible namespace on **both** hosts. Its functions `a_new_entry_with_a_former_object_name_cannot_change_a_staged_member`, `dropping_the_store_removes_only_its_own_namespace`, `a_staged_object_never_exists_as_a_path_an_adversary_could_open`, `a_substituted_namespace_is_refused_before_any_body_is_staged`, `an_unwind_during_staging_removes_only_its_own_namespace`, and `a_real_permission_denial_refuses_as_permission_denied` must be **rewritten or host-split**, not left asserting an intentionally removed Linux namespace. Retain every underlying security assertion.

- **Linux tests:** use an exclusive, task-owned staging directory. Take a sorted directory-entry snapshot *before*, *during* and *after* staged writes, finalization, errors, drop and unwind. Assert **zero new staging-owned entries**, byte-exact sealed reads, unchanged foreign sentinel and root survival. Replace namespace replacement with stronger root-ancestor symlink, final symlink, root substitution and retained-descriptor attacks; they must exercise real traversal refusal or prove that accepted writes cannot redirect. Do not reintroduce a directory merely to satisfy old tests.
- **Btrfs test (mandatory):** on `target_os=linux`, consume `SOLO_TEST_SOURCE_FS_ROOT` from the existing CI Btrfs mount; create one exclusively owned child under it, adopt it as the staging root and execute the actual `O_TMPFILE` stage/read/drop path. In GitHub CI, **fail if the variable is missing or if this mandatory adversary does not run**. Outside CI, a missing fixture is reported as a local environment limitation and not called PASS; ordinary ext4 tests still run. The workflow already creates and cleans the Btrfs mount. Keep the owned child cleanup scoped to its own directory. The earlier GitHub run 167 Ubuntu log recorded `fstype=btrfs` and the exported variable.
- **Windows tests:** keep the real created namespace observable while testing foreign replacement. Assert no accepted shard name after detach and no second read/write/mapping alias. Replace a namespace with an **empty impostor** and with a **populated impostor** at the exact same name; both must remain untouched after the transaction, while only the held owned object is disposed. Exercise intermediate junction/reparse points: if creating one is denied, classify **only** the exact applicable privilege error as `UNVERIFIED`; all unrelated errors fail, and a created attack must execute and refuse. Do not pretend an unexercised attack passed.
- **Cross-host:** retain positive roundtrip, repeated bounded sealed reads, no further writes after finalize, write/short-write/flush/finalize injected failures, full-set late-member refusal, ownership and sentinel isolation. Keep real permission-denial tests where the host allows the condition, and distinguish unavailable privilege from a passed check. Add a bounded child-process-abort test using synthetic content: record pre-crash ownership, terminate the child, assert no recoverable stage plaintext/foreign deletions, and report a genuine host limitation explicitly.

**Representative Linux root-entry assertion (adapt to existing `Fixture`/store APIs):**

```rust
#[cfg(target_os = "linux")]
{
    let before = entries(&fixture.staging());
    let root = StagingRootCapability::adopt(&fixture.staging()).unwrap();
    let mut store = root.begin().unwrap();
    let body = store.stage_body(b"synthetic staged bytes").unwrap();
    assert_eq!(entries(&fixture.staging()), before, "O_TMPFILE created a name");
    assert!(store.reader(body).unwrap().matches(b"synthetic staged bytes").unwrap());
    store.finalize().unwrap();
    drop(store);
    assert_eq!(entries(&fixture.staging()), before, "cleanup touched a name");
}
```

This is a test *pattern*, not permission to use `unwrap()` in production, omit negative cases, or silently skip Btrfs CI.

### 4B. Linux held-root and unnamed-object production path

1. Replace `StagingRootCapability::adopt`'s current `symlink_metadata(root)` + path-authoritative `open_directory(root)` on Linux. Validate absolute `Path` components without lossy conversion or traversal, open `/` as the trusted descriptor, then call pinned `rustix::fs::openat2` relative to it with `ResolveFlags::BENEATH | ResolveFlags::NO_SYMLINKS | ResolveFlags::NO_MAGICLINKS`; use `OFlags::RDONLY | DIRECTORY | CLOEXEC`. No `NO_XDEV`, `canonicalize`, intermediate pathname dereference, or weaker fallback. Define and test root `/`, parent traversal and known replaced/unlinked directory states.
2. Hold the opened directory `File`/descriptor and its identity throughout the stage lifetime. On Linux there is **no private staging namespace**: `begin_with_faults` creates no `solo-source-stage-*` directory, and `open_object` opens shards directly from the held root. Use safe `rustix::fs::openat(&held_root, ".", OFlags::TMPFILE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC, owner_only_mode)`; check exact API constants against pinned `rustix 1.1.5`. Never use an object path or remove a named entry. Preserve open-handle and 64 MiB shard accounting and existing staged-member offsets.
3. Make all success/error/drop/unwind code paths close only **owned** anonymous descriptors. Failure to create/read/finalize refuses the whole candidate; no JSON-to-Markdown fallback or re-read of original source. Unsupported filesystem/kernel behavior refuses explicitly, not via a path fallback.

**Descriptor-use pattern:**

```rust
let fd = rustix::fs::openat(
    &held_root,
    ".",
    rustix::fs::OFlags::TMPFILE
        | rustix::fs::OFlags::EXCL
        | rustix::fs::OFlags::RDWR
        | rustix::fs::OFlags::CLOEXEC,
    owner_only_mode,
).map_err(map_native_staging_error)?;
let shard: std::fs::File = fd.into();
```

`held_root`, `owner_only_mode` and `map_native_staging_error` denote values/functions from the implemented native boundary; do not introduce a new public API just to reproduce this illustration. Confirm types and modes with the pinned crate.

### 4C. Windows handle-relative creation and held-handle disposition

1. Implement staging-specific secure root adoption in `source_read_native.rs`. Bootstrap a supported drive-root path, open each subsequent component relative to the retained parent with `NtCreateFile` and no-follow `OBJ_DONT_REPARSE`/`FILE_OPEN_REPARSE_POINT`, then check type, reparse tag and physical identity **on the returned handle**. Preserve exact UTF-16. Reject unprovable UNC/device paths, reparse/junction intermediate components and changed target. Do not weaken or rewrite original authored-body read/clone/recheck behavior.
2. Create exactly one task-owned namespace relative to the held root and each shard relative to that held namespace. `NtCreateFile` accepts **separate** `CreateDisposition = FILE_CREATE` and `CreateOptions = FILE_DIRECTORY_FILE` or `FILE_NON_DIRECTORY_FILE`; never combine disposition with options. Use a validated, single-component name, exclusive creation, correct `DELETE`/data/access rights and share flags, and a returned `OwnedHandle`. **Never create by `std::fs::create_dir(path)` then inspect after creation.** Do not re-open a concatenated path to recover from native failure.
3. For a shard, ensure no second writer, reader or mapped view can access a transient name; use the proven share-denial and `FILE_DELETE_ON_CLOSE`/equivalent owned-handle lifetime. Detach the shard name using `SetFileInformationByHandle(held_shard, FileDispositionInfoEx, ...)` with the existing `windows-sys 0.61.2` exact bindings; verify `DELETE | POSIX_SEMANTICS` host semantics, close ordering and retained sealed-read handle before admitting it. If the host rejects one flag combination, determine the exact safe supported disposition sequence within these same approved native calls and prove it; **never** fall back to a visible accepted shard or name-based deletion.
4. At finalization and all relevant failures, dispose of the **held owned namespace handle** only. The old `remove_namespace` check-then-`remove_dir(path)` and early-return `remove_dir`/`remove_file` paths must disappear. Handle-directed disposition must never remove an empty or populated substitute located at the former namespace name. A failed disposal must refuse success; preserve the object for safe handle closure and report any actual residue. Protect adversary entries and Owner sentinels even when they are empty.
5. New/modified Windows `unsafe` is allowed **only** within `source_read_native.rs` for the named functions, with per-call pointer, lifetime, buffer, access-right and ownership `// SAFETY:` explanations. Import exactly the audited pinned symbols; no generated binding edits, manual structs, new FFI, dependencies or feature flags.

**Correct binding shape (confirmed against locked sources by the prior report; test actual host semantics):**

```rust
use windows_sys::Win32::Storage::FileSystem::{
    SetFileInformationByHandle, FileDispositionInfoEx, FILE_DISPOSITION_INFO_EX,
    FILE_DISPOSITION_FLAG_DELETE, FILE_DISPOSITION_FLAG_POSIX_SEMANTICS,
};
let info = FILE_DISPOSITION_INFO_EX {
    Flags: FILE_DISPOSITION_FLAG_DELETE | FILE_DISPOSITION_FLAG_POSIX_SEMANTICS,
};
let ok = unsafe {
    SetFileInformationByHandle(
        held_handle,
        FileDispositionInfoEx,
        (&raw const info).cast(),
        std::mem::size_of::<FILE_DISPOSITION_INFO_EX>() as u32,
    )
};
if ok == 0 {
    // Immediately capture the native error, close owned handles, and refuse.
}
```

This snippet illustrates the API boundary, **not** an assertion that these flags suffice on every host or a replacement for an ownership-proving test. Keep F1's actual no-alias/nameless-after-create guarantee, F2 confinement and F5 unowned-empty survival.

### 4D. Candidate integration and security regressions

Preserve original-source read identity/snapshot guarantees, manifest/source locator logic, member ownership, selected-only preflight, error vocabulary and all R01/R03 M1–M14 and R04 F1/F3/F4/F6/F7 tests. Propagate every fallible `finalize`/cleanup error through the source-set candidate; never return a partially staged candidate on a late fault. Keep source reads and staged reads separate. No global RAM accumulation or new source-set size ceiling. Preserve the checked 64 MiB shard target and limit changes to the authorized files.

## 5. Build–verify–fix–publish execution sequence

**Phase A — install/transition (on `main` only)**

1. Complete §3 preflight and commit/push only the Owner-installed ACTIVE file; verify both hosts green at that transition.
2. Create the one task branch and commit the **coherent** migrated test suite and production changes there. It is acceptable for early **task-branch** revisions to be red. Do not push intentionally incomplete code to `main`.

**Phase B — real CI iteration (on the one task branch)**

3. Use the exact existing workflow, no workflow edit. Normal command pattern after branch push:

```bash
git push -u origin solo-wo-051-r01
gh workflow run rust-verify.yml --ref solo-wo-051-r01
gh run list --workflow rust-verify.yml --branch solo-wo-051-r01 --limit 10
gh run view <actual-run-id> --log-failed
```

`<actual-run-id>` is a placeholder for a real returned ID, never a hard-coded invented result. Use native PowerShell syntax or an authenticated REST equivalent as appropriate. If the dispatch permission is unavailable, use the specifically authorized single PR fallback to trigger existing CI, verify tested tree identity, and retain the full final-`main` exact-SHA requirement. Record actual workflow event and SHA.

4. Iterate on the **same** branch. For each failing Ubuntu/Windows job, read actual compiler/lint/test output; correct the specific source or test; re-run. Typical blockers to check first: Linux cfg-gating and `rustix` types; Windows generated import types, `NTSTATUS`, raw-handle conversions and share-rights; obsolete namespace expectations in existing tests; owned handle close order and destination identity; Btrfs test actually consuming `SOLO_TEST_SOURCE_FS_ROOT`. **A normal compile error, unavailable local MSVC, or a failed branch CI run is not itself grounds to abandon the task**; diagnose, fix and re-test within the same ledger. Never claim branch CI succeeded until both real jobs succeeded for one exact candidate tree/revision.
5. Review actual warnings, skips and test names, not just the workflow's green summary. A CI `success` that never executed an essential adversary is **UNVERIFIED**, not PASS. No `#[ignore]`, `continue-on-error`, deleted adversary, false-positive acceptance, swallowed error or changed checker to manufacture green.

**Phase C — verified main and report**

6. Re-fetch `origin/main` and confirm it is exactly the authorized transition (or has only nonconflicting, independently reviewed report-only ancestry); confirm only authorized source/test paths differ, no unowned edits, no uncommitted work, and CI green for the exact branch candidate. Fast-forward `main` to that exact verified branch revision—**no rebase/force/squash or PR merge**. Push `origin/main` and verify its new SHA/tree. Run/check `rust-verify` at that exact substantive main SHA; both OS jobs must pass. If the publish or main CI fails, do not mark `COMPLETE`: preserve evidence and correct only through the authorized branch procedure, without rewriting published history.
7. Overwrite `.documents/Reports/execution-report.md` **only on `main`**, commit/push this single file, confirm the final report SHA and that `rust-verify` is green on both hosts again. Do not embed the report's own final SHA inside it; give it in chat. Verify `git status --porcelain` empty, `HEAD == origin/main`, exactly four WIP files, one worktree, sole remote; clean task-owned scratch and preserve unknown items.
8. After verifying the report and retained code on `main`, remove only the task-owned branch (and close its task-owned PR if used), without destroying any unrelated work. No release, deployment, R02.B administrative closure, R02.C or WIP pruning.

**Hard STOP conditions:** actual violation of Owner scope or repository identity; missing required credential privilege with neither approved CI trigger available; Windows by-handle/name-detach semantics disproven by real-host evidence with no safe solution inside this native boundary; an essential attack that cannot be exercised; new vulnerability or security regression; unavoidable unauthorized dependency/feature/unsafe expansion; unsafe cleanup; inability to preserve Owner/unattributed data. Publish a factual `STOPPED` report. **Do not stop merely for lack of local MSVC, a normal first CI failure, or limited per-session convenience.** Where an exact safe change is in progress but cannot be finished, preserve it on the single expressly authorized task branch with exact SHA and report its state instead of deleting the only implementation.

## 6. Mandatory acceptance evidence — H1–H10

| Gate | Required direct proof; all apply to the corrected source |
| --- | --- |
| **H1 Source policy/regression** | M1–M14 and F1/F3/F4/F6/F7: reservation, JSON-first invalid-preferred refusal, source identity, selected-only bounded preflight, original-source no-reopen and both `false` constants; exact source and limits intact. |
| **H2 Linux confinement** | Actual symlinked intermediate and final components, `..`, root substitution/retained-descriptor and supported mount cases; no redirected accepted write and no unchecked pathname fallback. |
| **H3 Linux unnamed lifecycle** | **Real Btrfs** via `SOLO_TEST_SOURCE_FS_ROOT` plus ext4 general cases: staged readable `O_TMPFILE` objects, zero new staging-owned entries, owner root/sentinels unchanged on write, finalize, error, unwind, drop and bounded abort. Mandatory Btrfs test executes in CI. |
| **H4 Windows confinement** | Real Windows 2025 drive-root/component-relative creation and intermediate reparse/junction, replacement and refusal; creation confirmed against held handles. Real attack exercised or privilege-specific non-pass recorded. |
| **H5 Windows staged-byte safety** | No second read/write/mapping of transient named shard, no pathname reaches detached shard, immutable repeated sealed reads, verified `FILE_DELETE_ON_CLOSE` and by-handle disposition semantics, process-crash limits. |
| **H6 Ownership-bound cleanup/atomicity** | Linux no name deletion; Windows held-handle disposal; **empty and populated impostors both survive**, even at former owned name; Owner sentinels untouched; late source/member/disposition failures refuse and never expose partial candidate. |
| **H7 Error/lifetime** | Real and injected permission/write/short-write/capacity/flush/finalize/disposition failures classified accurately; unwind and abort differentiated, no missing-JSON fallbacks or hidden partial success. |
| **H8 Confined native scope** | All new `unsafe` limited to named Windows native helpers in `source_read_native.rs`, exact bindings reviewed, no crate feature/lock/workflow/checker/other-native-reader changes. |
| **H9 Host CI + A1** | Two-host CI green for one substantive branch candidate (or proven tree-identical PR CI), **and again at exact substantive `main` SHA and final report-only SHA**; review job logs, emitted test counts/names, warning and skip inventory, architecture checker/refusal self-tests, cargo-deny refreshed RustSec, locks, lint, release smoke and reproducibility. Full A1 dependency/security/currency review of actual versions, not green alone. |
| **H10 Governance and report** | Initial ACTIVE-only transition; only eight permanent ledger files; exact four WIP files, no extra worktree/checkout/remote, final clean tracked tree, safely owned scratch cleanup, verified report, R02.B still OPEN, R02.C unstarted, no self-ACCEPT. |

For each H1–H10 and R04 F1–F8 record `PASS`, `FAIL`, `UNVERIFIED` or genuinely `NOT APPLICABLE` with reason and exact runnable evidence (`REAL`, `INJECTED` or `PURE`). **No partial acceptance.** Green CI plus unexercised host attacks is not an acceptable technical completion. Independently verify that the two original defects are fixed by the actual source and not only by tests asserting the intended result.

## 7. Canonical execution report and handoff

Overwrite the one `.documents/Reports/execution-report.md` with: order/edition and complete/stop status; exact initial anchor, ACTIVE transition, task branch commits, CI run/job links, substantive main SHA/tree, report-preceding source SHA; per-file diff and authorized ledger reconciliation; platform-specific ownership, root/adoption, lifetime, by-handle `FILE_CREATE`/`FileDispositionInfoEx` flags and safety rationale; actual Btrfs and Windows attacks with positive and negative outcomes; bounded-member and late-fault results; H1–H10/F1–F8 crosswalk with evidence type; exact security/advisory freshness, warnings/skips and remaining unverified cases; task temp cleanup and any previous unknown `.temp/` material untouched; final HEAD/remote verification; `COMPLETE` or `STOPPED`. No report duplication or secret/vulnerability exploit disclosure. Do not embed the report's own commit SHA in its contents; return that SHA only in the chat handoff.

**`COMPLETE` means this technical order is implemented and proven; it never means the Implementation Agent has independently accepted itself, administratively closed R02.B, authorized release, or started the next stage. The Chief Architect independently returns ACCEPT / REVISE / STOP on the exact published report and source revision.**

**End of SOLO-WO-051 R01.**
