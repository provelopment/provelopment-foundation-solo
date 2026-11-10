# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-052 R01**  
**Title:** Complete R02.B F2/F5 — Windows direct-root, handle-only staging lifecycle and final two-host proof  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Implementation branch:** Reuse the **existing** `solo-wo-051-r01` in the single verified worktree. Do not create a second implementation branch, worktree, checkout, or remote.  
**Prior report:** `SOLO-WO-051 R01` STOPPED at `79de89da78760119d0700b0210f4d6e5e1c9113d`; prior implementation is preserved at `7885dbf0a6d4c344e58248306b7c079b2766f78b` on `solo-wo-051-r01`.

## 1. Binding architectural decision and required result

**Complete and publish the R02.B F2/F5 correction.** The real Windows 2025 CI proved that the previous Windows *named namespace* design cannot meet both post-finalize handle-backed readability and namespace deletion before candidate success. `FileDispositionInfoEx(DELETE | POSIX_SEMANTICS)` succeeded on a shard, but its name was delete-pending and remained a directory entry while the retained shard handle was open; deleting the containing namespace failed with `ERROR_DIR_NOT_EMPTY` (`145`). The subsequent by-name probe returned `2`. Windows job `129749530814` in run `42792594389` contains these exact diagnostic records. Do not attempt another name-based cleanup or defer namespace deletion as a substitute.

**Owner-selected correction: eliminate the Windows namespace altogether.** Both Linux and Windows stage *directly beneath the already adopted, retained staging-root handle*, without creating a staging-owned directory. Their implementations differ by host:

- **Linux:** keep the branch's proven direct-root `rustix::fs::openat2` confinement and `O_TMPFILE | O_EXCL` anonymous inodes, with zero staging-owned directory entries and no pathname delete. Preserve the actual Btrfs proof.
- **Windows:** use the existing component-relative, reparse-refusing adopted root. Create each **file** with `NtCreateFile` using `FILE_CREATE` exclusively and `OBJECT_ATTRIBUTES.RootDirectory = held_root`, an exact one-component unique name, `FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_DELETE_ON_CLOSE`, `DELETE` rights and **share mode zero**. Apply `FileDispositionInfoEx(DELETE | POSIX_SEMANTICS)` **to that exact new held file handle before writing the first plaintext byte**. Only after successful disposition may any body byte be written and staged. Retain the handle for bounded positional reads after `finalize`. Closing the final handle on store release or process termination must remove the task-owned pending entry. No staging namespace exists to remove or to be substituted. No name-directed file/dir deletion, pathname retry or re-open of an original source.
- **Semantics are explicit:** While a Windows store holds its files, *delete-pending names may still appear in directory enumeration* on the tested host. This is **not** the Linux zero-entry promise. The Windows mandatory security promise is that the name cannot reopen, read, write, truncate, map, rename into a writable alias, or recover **any accepted plaintext** after the pre-write by-handle disposition, and that *all task-owned pending names disappear on final handle closure/process death*. No unstaged directory may be created, and caller/foreign root entries must be byte-identical before and after store release. Assert this by host, not by falsely interpreting directory enumeration as proof of no access. If actual Windows host observations do not support these guarantees, refuse the transaction and report the exact evidence; do not weaken tests.

**Success** is an implemented, reviewed-scope correction with real same-revision Ubuntu 24.04 and Windows 2025 green CI, all applicable adversaries executed, full source-set candidate success, and an honest canonical `COMPLETE` execution report. The Implementation Agent never self-ACCEPTs or closes R02.B administratively; the Chief Architect independently reviews F2/F5 and R02.B after publication. R02.C remains untouched.

## 2. Exact permanent-write ledger — eight paths only

| # | Authorized path | Exact responsibility |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | The entire Owner-issued `SOLO-WO-052 R01`, committed **alone** as the initial ACTIVE transition on current `main`; freeze afterward. |
| 2 | `.documents/Reports/execution-report.md` | Overwrite the single canonical report after actual execution; publish in its own last report-only commit, including on STOP. |
| 3 | `crates/solo-adapters/src/source_set_staging.rs` | Remove Windows namespace fields, creation, disposition and cleanup; use one retained root and per-store/per-shard names; preserve Linux and staging API semantics, lifetime, failure refusals and shard rotation. |
| 4 | `crates/solo-adapters/src/source_read_native.rs` | **Sole** new native `unsafe` boundary for Windows exact-parent `NtCreateFile(FILE_CREATE)` file creation, by-held-handle `FileDispositionInfoEx` and error mapping; preserve the original-body reader and the proven Linux `rustix` implementation. |
| 5 | `crates/solo-adapters/tests/source_set_staging.rs` | Host-specific root snapshot/visibility, second-opener/mapping, alias, root/junction, foreign-object survival, normal/fault/unwind/abort, retained-readable-handle and cleanup tests. |
| 6 | `crates/solo-adapters/src/source_set_candidate.rs` | Only minimal necessary stage/finalize/refusal integration; preserve declared source and provenance rules. |
| 7 | `crates/solo-adapters/tests/source_set_candidate.rs` | Only necessary whole-set regression/negative coverage. No changed policy assertions. |
| 8 | `crates/solo-adapters/src/lib.rs` | Optional minimal safe test/export adjustment; no new publication authority. |

**All other tracked paths are immutable**, especially NEXT/ROADMAP/OUTLOOK, Architecture/About/SOP, README, AGENTS, Cargo manifests/lock, toolchain, deny policy, `.github/workflows/rust-verify.yml`, checker/fixtures, `source_read.rs`, other crates, release/deploy files. No new crate/feature/raw FFI declaration/unsafe module; no ninth path, auxiliary persistent test file, PR merge, release, deployment, or other repository write. Use the exact already-pinned Rust `1.99.1`, `rustix 1.1.5`, `windows-sys 0.61.2` dependency graph and existing two-host GitHub Actions and Btrfs fixture. No local MSVC/WSL compiler installation is required or authorized. The user-scope Rust installation already performed may remain.

Never delete or modify the **2813 pre-existing unattributed entries in Solo `.temp/`**. Use only uniquely task-owned scratch; remove what this task owns when finished. No additional worktree, clone, remote, branch per CI run, shadow source tree, stash or history rewrite.

## 3. Exact starting state and repository sequence

The independently verified remote **`main`** is `79de89da78760119d0700b0210f4d6e5e1c9113d` (report blob `ee838959e414a2d629f79186dcb7753dc4051433`). It contains the `SOLO-WO-051 R01` ACTIVE text (blob `da22c897420ba4315bda271bb3b17af8daf105c3`) and no published F2/F5 product correction. The existing remote implementation branch `solo-wo-051-r01` is `7885dbf0a6d4c344e58248306b7c079b2766f78b`, tree `5e41545ed442a3b190b665b9c4b4da200c2d86fb`. GitHub comparison finds that branch **diverged** from `main` (latest report-only commit not yet in branch); do not repeat the prior report's unsupported claim that it is fully merged. The implementation branch already modifies only four source/test paths: `source_set_staging.rs`, `source_read_native.rs`, `source_set_candidate.rs`, `tests/source_set_staging.rs`. Its Linux correction and Btrfs tests passed in real CI. Last diagnostic branch run: `42792594389` (Ubuntu success; Windows failing tests). The unrelated current `main` CI run `42793710760` passed but is **not implementation evidence**.

1. Verify the actual single Solo checkout, `main`, HEAD/remote ancestry, `git status --porcelain`, exactly four regular `.documents/WIP/` files, sole `origin` URL for this repository, one worktree and the existence/identity of the existing implementation branch. Read README, AGENTS, both SOPs (§6B, §7B in Architect SOP), relevant Architecture, all four WIP files, the canonical STOP report, branch code/tests and raw CI logs. No destructive `checkout`, `reset`, `clean`, `rebase`, force-push or changes to unknown local material. Treat any unexpected local modifications or remote divergence outside the documented report commits as a safety issue, not as permission to overwrite.
2. With the complete Owner-issued `SOLO-WO-052 R01` installed at the prescribed path, commit/push **only** `.documents/WIP/workorder-active.md` on the clean `main` as the initial ACTIVE-only transition. Confirm the two-host verification runs pass for that transition and that frozen NEXT/ROADMAP/OUTLOOK remain unchanged.
3. Reuse the **existing** `solo-wo-051-r01` branch; do not create another branch. Fetch verified `origin/main`, then incorporate the latest `main` (including this order and the canonical reports) by a normal non-destructive **merge commit** into the existing branch, preserving all prior implementation commits and without rebasing published history. The new branch head must descend from the new ACTIVE transition. Resolve only ordinary authorized-path conflicts; STOP on other conflicts or unknown files. Verify branch CI code is not misrepresented as `main` source.
4. Perform the corrections in §4 and the tests in §5 **on that branch**. Push coherent bounded commits and dispatch the existing `rust-verify.yml` against that branch until **both CI hosts pass on the same branch SHA**; ordinary red intermediate branch runs are for diagnosis and are not accepted source. Keep `main` green and unchanged by implementation attempts until the success gate.
5. With two-host green branch evidence, independently verify branch/main ancestry, no unauthorized path changes, no remote advances, and make a fast-forward-only integration to `main` (`git merge --ff-only`), then push and verify an entirely new two-host green `rust-verify` at the exact substantive `main` HEAD. Never force-push, squash, rebase, perform a GitHub PR merge or merge a known-red revision. If `main` advances, integrate only the expected permitted reporting/administrative ancestry, retest, and fast-forward with fresh verification.
6. Overwrite **only** `.documents/Reports/execution-report.md`; report complete result accurately and commit/push that file **alone**. Verify both-host CI is green again at the final report HEAD; remote HEAD/tree and clean local tree agree. Give the final report SHA and links in the chat handoff. The canonical report must **not** embed its own SHA. When the successful code is reachable from `main` and both final runs are verified, remove only the task-owned implementation branch if safe; never delete the sole remaining implementation copy.

## 4. Mandatory code corrections — direct, non-negotiable

### 4A. Windows: remove namespace and stage directly under held root

**Files:** `source_set_staging.rs` and `source_read_native.rs`.

- Remove Windows `namespace_dir`, `namespace_identity`, `namespace_path` and all namespace create/dispose logic from protected staging. Remove any `namespace_name()` and `STAGE_NAMESPACE_PREFIX` mechanism that serves only that directory. Store retains **only** the adopted, verified `root_dir` capability/identity and its own shard handles; no child directory is created. `begin_with_faults()` performs no filesystem creation on either host. `finalize()` checks the retained root/recorded shard lengths and fault seams, then seals; it does **not** try to delete a directory or close readable shard handles. `Drop` closes every owned shard handle; it never deletes a pathname. `root()` remains the exact author-provided path for diagnostics, **not** a subsequent filesystem authority.
- Keep a per-store collision-resistant, **nonsecret** staging token (e.g. existing PID+monotonic counter+nanosecond seed, with a store ordinal) and append a shard index, e.g. `solo-stage-<store-token>-shard-<index>.bin`. Generated single-component names must have bounded UTF-16 length, never include a separator/NUL/`.`/`..`, never derive from authored source filenames or body content, and never replace an existing entry. Any `FILE_CREATE` collision **refuses** and leaves existing material untouched; do not use `OPEN_IF`, `OVERWRITE_IF`, `SUPERSEDE`, random deletion of collisions or a path retry.
- In Windows staging-only `open_staging_component`, final-root access should request the **least privilege needed to create files relative to it** (directory `FILE_ADD_FILE`/equivalent plus metadata/traversal and synchronization), **not** `DELETE` over the caller's root or unnecessary subdirectory creation rights. Intermediate ancestor components retain traversal/attribute-only rights. Preserve native component-relative opens, drive-root bootstrap and reparse refusal; reject unsafe path shapes and lossy Unicode conversions.
- Adapt the Windows native helper to create **one regular shard file** relative to the held root, never a directory, and request exactly `FILE_READ_DATA | FILE_WRITE_DATA | FILE_READ_ATTRIBUTES | DELETE | SYNCHRONIZE` (plus narrowly required rights if CI proves them necessary). `OBJ_DONT_REPARSE`, `FILE_CREATE` disposition, `FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_DELETE_ON_CLOSE` options, zero sharing, and the held parent's `RootDirectory` are mandatory. Keep the native handle owned by `OwnedHandle`/`File` exactly once. Preserve accurate `StagingNativeError::NativeFailure` versus `Unsafe`, `PermissionDenied`, `Unsupported`; never discard real Win32/NT error information into a fabricated `UnsafeLocation` or expose private file paths in CI diagnostics.
- **Before the first staged byte:** call `SetFileInformationByHandle(FileDispositionInfoEx, DELETE | POSIX_SEMANTICS)` on the just-created file handle; if it fails, close the still-empty delete-on-close file and refuse, never write or return the file. No unproven name-reopen check as the security control. After it succeeds, allow bounded writes and positional reads through the held handle. On Windows 2025, the deletion may be *pending until close*: this is intentionally acceptable **only** if the pathname cannot yield a second reader/writer/mapper or accepted plaintext, and the pending entry disappears on final handle closure/crash. A directory-listing-visible pending name is not proof of reopening and is not by itself a failure.
- Never add name-based cleanup (`std::fs::remove_file`, `remove_dir`, `DeleteFileW`, `RemoveDirectoryW`, `unlinkat(name)`), a `FileDispositionInfoEx` call on an unowned handle, a parent-root disposal, a newly-created namespace, an open-by-name fallback or a new FFI declaration. All new `unsafe` stays inside `source_read_native.rs` with `// SAFETY` invariants. Never modify the accepted original-body reader's native semantics.

**Implement this operation ordering, adapting existing types and error classifications:**

```rust
// Windows staging-only safe wrapper in source_read_native.rs.
// `root` is the *retained adopted root File*, not a resolved pathname.
let owned = sys::create_relative_file(
    root.as_raw_handle(),
    &single_component_name,
)?; // NtCreateFile: FILE_CREATE; no sharing; FILE_DELETE_ON_CLOSE.
let file = File::from(owned); // or transfer the OwnedHandle exactly once.
if let Err(error) = dispose_staging_object(&file) {
    // The named file was EMPTY; FILE_DELETE_ON_CLOSE cleans it on close.
    drop(file);
    return Err(error);
}
// Only now may stage_body write plaintext through `file`.
Ok(file)
```

The above illustrates **ordering**, not a substitute for checked Rust types or host execution. Do **not** pass `FILE_CREATE` as a bit of `CreateOptions`: it is the separate `CreateDisposition` parameter. Keep `FILE_DELETE_ON_CLOSE` on the file create; the previous 145 conflict arose from deleting an **enclosing namespace**, which no longer exists in this design. Consult the pinned generated binding types rather than hand-writing structs.

### 4B. Linux and whole-source-set integration

- Preserve the already-proven Linux `openat2` held-root adoption, `BENEATH | NO_SYMLINKS | NO_MAGICLINKS` (**no** `NO_XDEV`), direct-root `O_TMPFILE | O_EXCL | RDWR | CLOEXEC` shards, mode `0600`, no paths or staged names, no namespace deletion, Btrfs fixture/negative cases, source identity and refusal mapping. No speculative Linux rewrite to accommodate Windows.
- Preserve the candidate driver, JSON-first selected-only semantics, immutable selected body, 2 MiB manifest/JSON limit, 1 MiB Markdown limit, 64 MiB shard target, bounded peak RAM and per-member range/provenance, full reservations, real late-member failure refusal, no original source reopen, exact two `false` publication/snapshot authority constants. Do not weaken F1/F3/F4/F6 or M1–M14.
- Keep `StagingFaultsV1` production-path write/short-write/flush/finalize fault injection. A failure before acceptance returns no partial candidate. If a native operation cannot meet the promised exclusive handle/lifetime constraints, refuse with the **correct registered classification**, never as a missing body or Markdown fallback.

## 5. Required real-host acceptance tests and CI evidence

**Migrate the existing staging test harness from `namespace_path` to the correct host contract.** In `tests/source_set_staging.rs`, obtain a sorted snapshot of the caller-owned staging-root entries before the transaction. Linux must have exactly the same snapshot throughout and after the store. On Windows, while files remain open, newly observed entries (if any) may be **only the store's own delete-pending shard files**, not a namespace directory; those entries must not open with read/write/delete access, permit mapping, or reveal accepted content. After `Drop`/unwind/process termination, assert the **exact original root-entry snapshot**, with foreign files and empty and populated directories unchanged. For Windows, **do not assert snapshot equality at `finalize()` before the last shard handle closes**. Instead prove successful `finalize`, repeated exact positional reads through live handles and post-`Drop` snapshot equality. Keep the tests honest about kernel-level directory enumeration.

**Native Windows attack coverage must actually execute:**

1. Success: `StagingRootCapability::adopt -> begin -> stage_body (several) -> finalize -> repeated reader.matches/read_whole -> drop`, with no `ERROR_DIR_NOT_EMPTY` dependency and zero directory creation. Test one shard and shard rotation/large cumulative bytes. Exercise the previously red `source_set_candidate` whole-declaration successes (all seven previously blocked). Do not change their accepted JSON-first expectations.
2. Open/recover: after pre-write disposition and after real data staging, attempt second read/write/open, rename, deletion and writable section/mapped-view acquisition using the actual generated API through the audited native module as needed. If an attack cannot be constructed on CI, explicitly mark it `UNVERIFIED`, not PASS. A merely missing guessed filename is **not** a sufficient adversary: derive/observe the *actual* task-owned pending name for the test without giving it public filesystem authority. A report of no sharing must name the actual tested host result. Do not add `unsafe` to the integration-test source.
3. Confinement: intermediate/final reparse/junction/symlink refusal, absolute path-shape/Unicode handling, root path rename/replacement, relative creates through the pinned root and a hostile same-name replacement attempt. No created file may land in an impostor root, no caller root may be deleted. Distinguish actual attack execution from genuine privilege denial.
4. Ownership: include **empty** and **populated** pre-existing adversary directories at names a prior namespace mechanism would have used, plus foreign files/sentinels and task-file-name collisions. No code may remove/modify any of them, even when empty. An attack must attempt the action it is supposed to detect; do not assert on an inert fixture. There is no namespace cleanup in this design, so the F5 proof is that every shard is exact-handle owned and **no directory cleanup exists**.
5. Failures: injected write/short-write, capacity/quota, flush/finalize, real permission denial where executable, disposition failure via a test-reachable real native-path seam, early return, unwind and **bounded process abort**. Successful disposition occurs before first plaintext write; a killed process cannot leave a recoverable plaintext name or persistent shard object. Assert task-entry cleanup after termination; no hidden skip or slow unbounded disk stress. `Drop` is not a crash test.
6. Linux: mandatory `SOLO_TEST_SOURCE_FS_ROOT` on actual CI Btrfs, `O_TMPFILE` read/write and zero created entries; root/intermediate symlink refusal; unlinked/replaced root; successful staged reads; preserved Owner entries; handle counts; injected failures and abort child. CI without the required Btrfs fixture **fails** rather than skipping.

**Test assertion pattern (actual host split):**

```rust
let before = entries(&fixture.staging());
let mut store = root.begin().expect("begin");
let staged = store.stage_body(b"synthetic-body").expect("stage");
store.finalize().expect("finalize");
assert!(store.reader(staged).expect("reader").matches(b"synthetic-body").expect("match"));
#[cfg(target_os = "linux")]
assert_eq!(entries(&fixture.staging()), before); // anonymous inode
#[cfg(windows)]
{
    // Do NOT require an empty directory listing while the file handle remains live.
    // Assert the actual entry, if visible, is task-owned and provably UNOPENABLE,
    // including via a real Windows second-opener/mapping adversary.
}
drop(store);
assert_eq!(entries(&fixture.staging()), before); // both hosts, after handle close
```

**Verification:** Run focused `cargo test --locked -p solo-adapters --test source_set_staging` and `cargo test --locked -p solo-adapters --test source_set_candidate` on the authorized real CI hosts (local Windows cannot link; do not block on it). Then run the full existing `rust-verify` workflow at **one exact branch SHA**. Inspect raw Ubuntu and Windows job logs and account for compiler/format/strict Clippy, all tests and adversaries, Linux Btrfs fixture and cleanup, `architecture=legal packages=4` / 29 self-check cases, safety/no-unsafe-outside-native, locked graph, refreshed RustSec and adversary advisory probe, cargo-deny all classes, release CLI smoke, Linux reproducibility, and real warnings/skips. Keep failures genuine and visible: no test removal, broad `cfg` skip, skipped security test, ignored error, invented mapping result, or workflow edit.

**H1–H10, all mandatory:** H1 full source/manifest/candidate regressions; H2 held-root confinement both hosts; H3 Linux Btrfs anonymous lifecycle; H4 Windows component/reparse/root proof; H5 Windows exclusive, disposition-before-write, retained-byte-read, crash cleanup; H6 no unowned deletion and exact pre/post snapshot with empty/populated impostors; H7 true faults/refusal classification and no partial candidate; H8 least-privilege/`unsafe`/feature boundary; H9 full two-host same-HEAD A1/security and final report-HEAD green; H10 exact Git/file ledger/cleanliness/canonical report. Re-score R04 F1–F8 and explicitly close F2 and F5 *technically* only with verified evidence. Mark every gate `PASS`, `FAIL`, `UNVERIFIED`, or genuinely `NOT APPLICABLE` with real test/CI citations and residual risks; do not call green CI alone a proof of an unrun adversary.

## 6. Reporting, role separation and final result

Publish exactly one overwritten `.documents/Reports/execution-report.md`. Include the previous Windows `145`/`2` diagnostic evidence, adopted **direct-root Windows** resolution and what trade-off it makes (delete-pending names may be directory-enumerable until the retained handle closes), observed native failure codes on the final hosts, true before-write disposition and process-death proof, branch reconciliation, all changed paths, exact test names/host outcomes, H1–H10 and F1–F8, current RustSec/lock/CI evidence, report-preceding substantive SHA/tree, remote HEAD/tree and scratch-cleanup facts. Do not embed the report's own commit SHA, private absolute paths, secret material or source bodies. Report the final report commit SHA only in the handoff.

The agent's `COMPLETE` status means only that this **technical execution** is implemented and proven; it is **not** Architect ACCEPT, R02.B administrative closure, R02.C authorization or deployment authority. The Chief Architect independently reviews the published exact substantive and report revisions and returns `ACCEPT / REVISE / STOP`. **Do not start R02.C.**

**End of SOLO-WO-052 R01.**
