# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-050 R01**  
**Title:** Complete R02.B staging-security correction — executable Linux Btrfs and Windows handle-bound proof  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Execution branch:** `main`; exactly one verified Solo checkout/worktree; sole remote `origin`.  
**Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verify actual root; do not assume access to any other checkout).  
**Supersedes:** `SOLO-WO-049 R01`, whose published report at `811c509a09aac2d731bc93633cde7a0b56811e40` was `STOPPED` without any product-source changes. R04's F2/F5 defects remain unresolved. This order is the sole executable authority after its installation and matching Owner handoff. It does not accept R04/R01 or close R02.B in advance.

## 1. Objective and mandated result

**Implement and prove** the hybrid host-specific staging correction for the already implemented R02.B whole-source-set candidate pipeline. Do not stop merely because the Owner's Windows workstation has no preinstalled Rust or the local WSL filesystem is ext4. The established GitHub `rust-verify` workflow already installs pinned Rust `1.99.1` on **both** hosts and, on Ubuntu, mounts a **real loop-backed Btrfs filesystem** before executing workspace tests. Use these actual proof venues.

The current `source_set_staging.rs` implementation remains defective: `StagingRootCapability::adopt` first checks only the last path component and opens the whole path by name; `begin_with_faults` creates a named namespace through the path and checks ancestry only after creation; `remove_namespace` and its error paths remove selected **names**, which can delete a foreign replacement. Eliminate these defects, including the R04 F2/F5 adversaries, without sacrificing source-selection, stage immutability, fail-closed behavior or other accepted contracts.

- **Linux:** securely adopt the entire requested absolute root through a pinned descriptor using `rustix::fs::openat2` and `BENEATH | NO_SYMLINKS | NO_MAGICLINKS`; create unnamed, non-linkable staging objects **directly in the held root** using `O_TMPFILE | O_EXCL` and explicit owner-only permissions. No transaction-owned staging namespace, shard entry, pathname write, pathname cleanup, or deletion of any root entry.
- **Windows:** traverse the caller-selected root component by component, refusing reparse/junction/symlink redirection. Create each transaction-owned namespace and shard **relative to retained parent directory handles** through the existing pinned `NtCreateFile` binding in the `FILE_CREATE` form. Keep temporary staged data inaccessible to a second reader/writer/mapper; remove its name by **held-handle disposition**, never name lookup. Remove only an exactly held owned namespace handle, and never a replacement/foreign object. Prove crash-safe transient-name handling and the final sealed-read lifetime on real Windows 2025.
- **Whole candidate:** keep the sequential `drive_declared_source_set_v1` driver, all-of-set reservations, selected-body-only JSON-first preflight, invalid preferred JSON refusal rather than Markdown fallback, 2 MiB manifest/JSON and 1 MiB Markdown limits, 64 MiB staging shard target, bounded peak memory independently of cumulative selected bytes, precise per-member provenance and failure classification, no original-body reopen after staging, and exactly `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false`. Stage output is neither public authority nor a whole-filesystem atomic snapshot.

**Completion** requires real executable evidence on both CI hosts and the independent A1 review of the published substantive revision. No self-acceptance, premature R02.B closure, R02.C execution, release, merge or deployment.

## 2. Frozen permanent-write ledger — eight paths, no ninth file

| # | Authorized permanent path | Exact authority |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | Commit only the complete Owner-installed R01 replacement with this `SOLO-WO-050 R01` text as a single ACTIVE-only transition; then leave the text frozen. |
| 2 | `.documents/Reports/execution-report.md` | Overwrite the single canonical report after execution, including on failure; publish it in a final report-only commit. |
| 3 | `crates/solo-adapters/src/source_set_staging.rs` | Correct Linux held-root anonymous staging and Windows handle-bound namespace, shard, disposal and failure lifetimes. Remove forbidden pathname deletion. |
| 4 | `crates/solo-adapters/src/source_read_native.rs` | The only native expansion boundary. Implement safe Linux `rustix` helpers and the specifically approved Windows handle-relative `NtCreateFile(FILE_CREATE)` and `SetFileInformationByHandle(FileDispositionInfoEx)` operations, with minimal Windows-only `unsafe` and explicit safety comments. Preserve the original-source native reader. |
| 5 | `crates/solo-adapters/tests/source_set_staging.rs` | Mandatory real-host adversaries, actual Btrfs `O_TMPFILE` evidence through the existing CI fixture, Windows reparse/root substitution/foreign cleanup tests and crash/fault behavior. |
| 6 | `crates/solo-adapters/src/source_set_candidate.rs` | Optional minimal stage-lifetime/error integration only; no source-policy rewrite. |
| 7 | `crates/solo-adapters/tests/source_set_candidate.rs` | Optional additional whole-set late-failure, atomically refused candidate and no-reopen regression tests. |
| 8 | `crates/solo-adapters/src/lib.rs` | Optional minimal internal export adjustment. No public privilege or publication authority. |

**All other tracked files are read-only.** In particular, do not modify `README.md`, `AGENTS.md`, About/Architecture/SOP, ROADMAP/NEXT/OUTLOOK, Cargo manifests/lock, pinned toolchain, `deny.toml`, `.github/workflows/rust-verify.yml`, checker/fixtures, application/domain crates or `source_read.rs`. Do not install a new crate, add a Cargo feature, create a new native module, add a workflow file or alter the original-source/native read mechanisms. No sibling/parent repository write, extra checkout, worktree, PR, branch, remote or archival report.

**Scoped development-environment authority:** On the verified Owner-local Windows workstation and its WSL2 instance only, install the official **pinned Rust `1.99.1` toolchain**, Cargo, rustfmt and Clippy with first-party `rustup` as needed, in the standard current-user toolchain directories. The official HTTPS installer and locked Cargo dependency acquisition are authorized network uses for this task. Before running the installer, verify its official source and provenance; record what was installed. Do not install global packages, upgrade the OS, alter system security settings, install new compiler versions, add persistent PATH changes beyond the standard user-level Rust setup, or touch unknown files. The existing hosted CI already provides both required build hosts and Btrfs; no separate Btrfs host or filesystem package installation is authorized or needed. Installing local Rust is a convenience for fast correction, **not an acceptance gate and not a reason to STOP if unavailable**: exact-head CI on the existing hosted hosts is the authoritative real-host proof.

Use only verified Solo `.temp/` or the active Cline task scratch for task-owned disposable experiments. **There were 2813 pre-existing, unowned/unattributed `.temp/` entries at the prior run**: do not delete or modify them merely to claim cleanliness. Create uniquely identified task-owned scratch paths and remove only those after use. A separate source checkout or worktree is prohibited, even inside scratch.

## 3. Frozen starting point and mandatory preflight

**Reviewed published anchor:** `811c509a09aac2d731bc93633cde7a0b56811e40`, Git tree `9edb0db7e1a913c03f41ca833c2e3cfcd6245c3b`, whose parent is the R01 ACTIVE-only transition `f519f97571c04466682f04fb96ad1d60c928f3a3`. Anchor changes relative to that parent are report-only. CI `rust-verify` run **165** completed successfully on `ubuntu-24.04` (job `129691614357`) and `windows-2025` (job `129691614743`). The published R01 canonical report blob at the anchor is `eb3dc7c666022570f674f01a7a2ca563cc33b9c9`. The previously frozen source blobs remain unchanged: staging `e20054c76a4296c187d0ccbbf87601737c9c5c88`, native `ab6a392ec45d6a6b946bdf8a6e4ce9a567b3ae26`, candidate `311d44c807f7fbc55df175161054041c78012869`, tests staging `dd2b71f31b09c4ee1067aa98d27aef21d19fccfe`, tests candidate `4c3cbad0aa274871689f031c105dbafd2d7cb779`, `lib.rs` `31bee760d918e014fc6378ef40ec1e9eb058449f`. NEXT `92fa562b1af0c54391851aa5c79deb1631127069`, ROADMAP `18975332c2820170a32558c4ee224176667b91ab`, OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d` stay frozen.

1. Verify actual root, `git rev-parse --show-toplevel`, branch `main`, exact current HEAD/remote tracking, `git status --porcelain`, one worktree and sole `origin` with URL `provelopment/provelopment-foundation-solo`. Verify exactly the four prescribed regular WIP files; verify that the Owner-installed local ACTIVE contains **this entire** `SOLO-WO-050 R01` order, and that the Owner handoff names the same ID and edition. An unrecognized remote change or other dirty path is STOP, not permission to overwrite.
2. Read README, AGENTS, both SOPs, relevant Architecture sections, all four WIP files, the last report, the existing staging/native code and tests. Verify HEAD ancestry and unchanged frozen files. Do not confuse the last report's `STOPPED` disposition with a new current result.
3. Commit/push **only** the Owner-installed ACTIVE replacement as the first commit; verify existing Ubuntu/Windows CI is green. No implementation source edits in this transition.
4. Inspect the current CI before coding. Existing `.github/workflows/rust-verify.yml` already mounts a loop-backed **Btrfs** image on Ubuntu, confirms `supported_fstype: fstype=btrfs`, exports `SOLO_TEST_SOURCE_FS_ROOT` and `SOLO_TEST_UNSUPPORTED_FS_ROOT` to the test step, and cleans owned mount/image paths afterward. Run 165's Ubuntu raw log proves this. The Windows runner already installs `1.99.1` before testing. **Do not request new test hosts, workflow edits, or Btrfs packages.**

## 4. Implementation sequence — implement a complete, testable correction

The preceding order's scratch-experiment-before-any-source rule is **replaced** by this executable sequence. Build and run isolated local probes when useful, but do not block on the unavailable local test venues. Integrate the actual host probes into the **authorized existing staging tests**, implement the corresponding production mechanisms, and use the existing real-host GitHub CI at the same substantive revision for proof. Do not commit a deliberately failing adversary or a known incomplete host half. A genuine failed CI run is an engineering failure to diagnose and repair under this same scope; it is not an authorization to weaken a test. Commits correcting code/tests inside this ledger are authorized until a complete substantive revision passes on both hosts.

### 4A. Linux — retained root and zero named staging objects

1. Remove staging-root pathname-open validation from the protected Linux path. Parse the caller's absolute `Path` by components without lossy re-encoding; refuse NUL, relative, parent traversal and unsafe components. Start with an opened `/` descriptor and resolve all following components using `rustix::fs::openat2` with **`ResolveFlags::BENEATH | ResolveFlags::NO_SYMLINKS | ResolveFlags::NO_MAGICLINKS`** and `OFlags::DIRECTORY | OFlags::CLOEXEC`. Do not add `NO_XDEV`, call `canonicalize` as security preflight, or fall back to a path-based `open`. Treat the filesystem `/` root explicitly. Prove actual descriptor identity and refusal classes; inspect the held descriptor when testing known root unlink/replacement.
2. Open every staging shard through the **retained root descriptor** using `rustix::fs::openat(&root_fd, ".", OFlags::TMPFILE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC, owner_only_mode)` (exact type/imports confirmed against locked `rustix 1.1.5`). Hold the returned owned descriptor for the shard lifetime; write, seek, flush, seal and read only through it. Use `O_EXCL` to prohibit later linking of an `O_TMPFILE` inode. No created namespace and no staging-root pathname used for a later create/remove.
3. Remove Linux namespace creation and `remove_namespace` code paths entirely from the Linux compilation path. Dropping the last shard handle performs cleanup. The caller-owned root and every foreign entry remain untouched. Preserve stage size, member limits, disk-backed bounded memory, all read/finalize refusals, and no exposure on a late member failure. The source-read snapshot code remains unchanged.
4. Migrate Linux tests that formerly counted named staging directories into tests that snapshot the directory's entries before/during/after staging and assert that **no staging-owned entry ever appears**; check success, late failure, injected capacity/flush, unwind/drop, retained-handle replacement and a bounded crash child. Use synthetic content only.

**Implementation pattern (adapt to existing types; use the locked API, not ad hoc syscalls):**

```rust
use rustix::fs::{openat, openat2, Mode, OFlags, ResolveFlags};

// `slash` is an already opened, trusted descriptor for `/`.
// `safe_relative` has been built from the original absolute Path components.
let held_root = openat2(
    &slash,
    safe_relative,
    OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
    Mode::empty(),
    ResolveFlags::BENEATH | ResolveFlags::NO_SYMLINKS | ResolveFlags::NO_MAGICLINKS,
)?;
let shard = openat(
    &held_root,
    ".",
    OFlags::TMPFILE | OFlags::EXCL | OFlags::RDWR | OFlags::CLOEXEC,
    Mode::from_raw_mode(0o600),
)?;
```

Use the appropriate existing refusal mapping, not `?` into an unrelated error type; verify the `Mode` constructor against the locked version before finalizing.

### 4B. Windows — handle-relative create and handle-directed deletion

1. **Separate staging adoption from the existing original-body reader.** For staging, bootstrap the drive-qualified absolute root safely and open every subsequent component **relative to the retained parent handle**, using `NtCreateFile` with no-follow/reparse controls and by-handle type, tag and physical-identity checks. Reject unexpected UNC/device syntax, ambiguous components and all intermediate/final reparse traversal; do not resolve the whole untrusted pathname with `CreateFileW` and only inspect its last component. Keep UTF-16 exact; do not normalize by `to_string_lossy`.
2. Create the transaction namespace with `NtCreateFile(FILE_CREATE | FILE_DIRECTORY_FILE)` expressed as the correctly separated creation-disposition and create-options arguments, and with `OBJECT_ATTRIBUTES.RootDirectory` set to the already held root handle. Create each shard the same way with `RootDirectory` set to the held namespace handle, `FILE_CREATE` disposition and `FILE_NON_DIRECTORY_FILE` option. Give each call a validated **single component**. Request exact access including `DELETE` for by-handle disposition; set sharing to prevent a second reader/writer/mapper. Never retry by concatenating a pathname after an error. The returned `OwnedHandle`, not a subsequent lookup, is the ownership capability.
3. Before any stage member is admitted as usable, detach its temporary name through `SetFileInformationByHandle` using `FileDispositionInfoEx` and `FILE_DISPOSITION_INFO_EX` with the appropriate tested `FILE_DISPOSITION_FLAG_DELETE | FILE_DISPOSITION_FLAG_POSIX_SEMANTICS` flags and handle lifecycle. Preserve a valid sealed read handle while ensuring no name remains reachable after successful detach. Prove whether `FILE_DELETE_ON_CLOSE` is necessary and compatible for process-death cleanup between creation and detach. Do not assume POSIX-name disappearance occurs at a particular instant merely because an API returns success: test the exact close sequence and confirm it on Windows 2025.
4. Dispose of the **held owned namespace** by handle only, after all owned shards are closed and only when the kernel confirms deletion can succeed. An adversary may rename/remove the namespace's directory entry and replace it with an empty or nonempty impostor at the same name; **the impostor must survive untouched**. A foreign object must not be removed even when empty. Never call `remove_dir`, `remove_file`, `DeleteFileW` or name-directed `unlinkat` on that protected lifecycle, including early-return paths. Cleanup failures must preserve refusal or be explicitly reported; never transform cleanup failure into a successful candidate.
5. Confine all new or modified Windows `unsafe` to the existing audited `crates/solo-adapters/src/source_read_native.rs` file. Use the exact already-feature-enabled bindings from `windows-sys =0.61.2`; no manually defined C structs, guessed offsets or new FFI declarations. Include narrow per-call `// SAFETY:` notes for pointer sizes, owned handle lifetime, UTF-16 buffer lifetime, access/share flags, the exact disposition structure and error conversion. Keep the original-source `NtCreateFile(FILE_OPEN)` reader unchanged.

**Verified symbol names to import from the existing `Win32_Storage_FileSystem` feature:**

```rust
use windows_sys::Win32::Storage::FileSystem::{
    SetFileInformationByHandle, FileDispositionInfoEx, FILE_DISPOSITION_INFO_EX,
    FILE_DISPOSITION_FLAG_DELETE, FILE_DISPOSITION_FLAG_POSIX_SEMANTICS,
};

// A held, DELETE-access-capable, owned handle; never a pathname lookup.
let disposition = FILE_DISPOSITION_INFO_EX {
    Flags: FILE_DISPOSITION_FLAG_DELETE | FILE_DISPOSITION_FLAG_POSIX_SEMANTICS,
};
let ok = unsafe {
    SetFileInformationByHandle(
        held_handle,
        FileDispositionInfoEx,
        (&disposition as *const FILE_DISPOSITION_INFO_EX).cast(),
        std::mem::size_of::<FILE_DISPOSITION_INFO_EX>() as u32,
    )
};
if ok == 0 {
    // Translate GetLastError immediately to the existing registered refusal.
    // Ensure every owned handle is closed without deleting any foreign name.
}
```

The example establishes the binding and call shape, **not** a proven safe lifetime. Finish the exact handle rights, sharing, name-detach, mapped-view and delete-pending behavior by running Windows-native adversaries. `NtCreateFile` continues to take separate `FILE_CREATE` creation-disposition and `FILE_DIRECTORY_FILE`/`FILE_NON_DIRECTORY_FILE` create-options fields; do not bitwise-OR them into one argument.

### 4C. Cross-host candidate integration and negative cases

1. Do not return a completed source-set candidate until **every declared representation** has been accepted and staged with all required handle cleanup/detach guarantees. Keep each member's original selected locator, identity, snapshot provenance and failure attribution.
2. Preserve actual production-path injection for write/short-write, capacity, flush/finalization, permission and cleanup failures. An invalid preferred JSON never falls through to Markdown. A failed later member returns no earlier staged member to a caller. No new public publication authority.
3. Build deterministic real adversaries for (a) ancestor symlink/reparse; (b) adopted root rename/replacement; (c) namespace substitution before/at deletion; (d) **empty foreign directory** replacement (must survive); (e) populated foreign directory; (f) second writer/reader/mapped view; (g) injected errors after earlier successful members; (h) bounded child-process abrupt exit. A test must assert that its hostile operation truly occurred before it claims the production refusal or protected survival.
4. Windows reparse/junction tests must attempt a real junction or symlink under owned scratch. Log created-and-tested refusal; classify only the precise host privilege denial as `UNVERIFIED` for that **specific** unavailable adversary. Any other setup error fails. At least one actually executed component-redirection adversary is mandatory for H4; a Windows CI job that runs none cannot pass H4.
5. The Btrfs staging test is mandatory **inside the existing** `crates/solo-adapters/tests/source_set_staging.rs`. On Ubuntu CI, read the already exported `SOLO_TEST_SOURCE_FS_ROOT` (known `btrfs` mount) and create a uniquely named, task-owned child staging directory there; open `O_TMPFILE` shards inside that child and assert zero staging-owned entries. The existing workflow cleans its Btrfs mount. **Make a missing CI Btrfs fixture a test failure**, not a silent skip; local runs without the CI environment can report the host-specific test as not run, but cannot be cited as its proof. Do not edit the CI workflow to create a second mount or introduce a ninth file.

## 5. Real execution, CI and failure discipline

1. After the single ACTIVE transition, implement the **entire** cross-platform correction and adversaries within the authorized ledger; use local pinned tools for fast `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, and `cargo test --workspace --all-targets --locked` when available. Use the existing official first-party rustup installer for pinned `1.99.1` on the Owner-local host as needed, and fetch only already locked crates. No other install or manifest alteration.
2. Commit/push a coherent substantive implementation with the adversaries **together**, not an intentionally broken test-only intermediate, to existing `origin/main` using the permitted Git preflight. The existing CI's Ubuntu Btrfs fixture and Windows 2025 runner are the actual host proof. Read **raw job logs** and every warning/skip. Repair substantive CI failures in bounded follow-up commits touching only the same ledger paths; rerun the full two-host CI until a common exact substantive HEAD is green. Do not mask failures, skip tests, replace native adversaries with fakes, weaken refusal codes, add manual assumptions or change the runner.
3. Test case requirements: Windows namespace and shard creation/ownership, `FileDispositionInfoEx`, transient-name and crash process behavior; Linux actual mounted Btrfs `O_TMPFILE` + ext4 root behavior where run; adversary replacement of a foreign **empty** directory; root/ancestor substitution; unauthorized alias access; original source-set M1–M14 regression and every applicable R04 F1–F8 guard. Preserve full `cargo-deny` security, architecture checker, format/lint, all-target tests, release CLI smoke and release-byte reproducibility gates.
4. **No fabricated success.** The only acceptable substantive `COMPLETE` report follows passing actual production-path tests at the verified source commit on both CI hosts. Distinguish `REAL`, `INJECTED` and `PURE` evidence. A host-only permission restriction may leave a narrowly identified nonessential case `UNVERIFIED`, but never call H2/H3/H4/H5/H6 or the F2/F5 security obligations `PASS` without their real attack/execution proof. If the necessary kernel operation itself cannot provide the required guarantee, stop with the exact binding/OS failure; do not loosen the contract or invent another native privilege.
5. Overwrite only `.documents/Reports/execution-report.md` with exact results, publish it in one **report-only** commit and verify fresh two-host CI at that final report HEAD; independently confirm `HEAD == origin/main`, sole worktree/remote, clean tracked tree and exactly four WIP files. Do not embed the final report commit SHA in its own content; deliver that SHA in the agent's final chat message.

**Authorized Git actions for the Implementation Agent:** exact ACTIVE transition, necessary substantive commits correcting the same authorized source/test paths, a single report-only completion or STOP report, safe push to existing `origin/main`, post-push revision and CI verification. **Not authorized:** self-approval, checkout duplication, branch/PR creation, force-push, unrelated write, Git-history rewrite, merge, release, deployment, new credentials or services. The Chief Architect has no GitHub write role; none is conferred by this order.

## 6. Acceptance matrix — every applicable gate must be independently reviewable

| Gate | Required executable evidence |
| --- | --- |
| **H1 — source-selection integrity** | R01/R03 M1–M14 and existing F1/F3/F4/F6 preserve reservations, JSON-first selection, limits, provenance, bounded staging memory, no original reopen and both `false` authority constants. |
| **H2 — Linux root confinement** | `openat2` rejects real intermediate/final symlink/magic-link escapes and unsafe traversal; root substitution cannot redirect a held-descriptor write; no name-based fallback. |
| **H3 — actual Btrfs zero-name lifecycle** | Ubuntu CI's existing `SOLO_TEST_SOURCE_FS_ROOT` really reports Btrfs; `O_TMPFILE | O_EXCL` shards are opened/read successfully there, and normal/fault/drop/unwind leaves **zero transaction-created entries** and zero pathname deletions. Owned caller root and foreign entries remain. |
| **H4 — Windows root and create confinement** | Actual Windows 2025 relative-handle create and reparse/junction/ancestor/root adversaries execute, reject redirection and cannot write outside the held parent. Test setup branches are classified precisely. |
| **H5 — Windows sealed stage and name removal** | Actual Windows second reader/writer/mapping attempts fail; held shard becomes unreachable by former name before member admission; read handle remains valid, `DELETE` access and disposition are proven; crash/restart does not expose orphan plaintext. |
| **H6 — owned-only deletion and whole-candidate refusal** | Deterministic Windows **empty and populated** foreign replacement objects survive cleanup; all actual deletion targets verified owned handles; Linux performs no pathname deletion; late failures expose no partial candidate or staged data. |
| **H7 — failure and lifetime truth** | Production-path injected write/short-write/capacity/flush/finalization/handle errors and host permission failures preserve registered refusals; real bounded process termination is distinguished from unwind. |
| **H8 — limited native authority** | Only existing approved native source file contains new Windows-specific `unsafe` for the exact two call families; zero new dependency, feature, toolchain pin, workflow change, external native declaration or original-source read change. |
| **H9 — two-host A1 and security** | Exact substantive and report-HEAD Ubuntu 24.04 + Windows 2025 CI fully green; real raw logs, fresh advisory/licence/source checks, strict lint/tests, architecture checker/refusal fixture, byte reproducibility, CLI smoke, warning/skip inventory and exact versions documented. |
| **H10 — repository and governance** | Exact eight-path ledger, one ACTIVE, frozen other three WIP blobs, report-only final commit, clean one-worktree/sole-origin state, only task-owned scratch removed, no self-ACCEPT, R02.B and R02.C untouched administratively. |

Use `PASS`, `FAIL`, `UNVERIFIED` or reasoned `NOT APPLICABLE` for every H gate and each preserved R04 F1–F8. H2–H6, F2 and F5 must be proven for technical `COMPLETE`. A1 is binding at the exact candidate revision, including current component advisories, vulnerabilities, versions, warning/skip currency and relevant security refusals. An independently unverified requirement is not a pass.

## 7. Canonical reporting and final handoff

Overwrite `.documents/Reports/execution-report.md` with: order ID/edition, prior R01 STOP and anchor, Git preflight/transition/substantive/report revision evidence, exact path diff, Rust installation source and actual versions, Windows 0.61.2 binding audit and exact native flags/rights/`unsafe` comments, Linux Btrfs mount identification and O_TMPFILE results, ownership and empty-impostor negatives, source-stage limits and no-reopen regression, H1–H10 and F1–F8 crosswalk, `REAL/INJECTED/PURE` evidence, exact GitHub run/job IDs and relevant raw-log excerpts, warning/skips and A1/supply-chain disposition, known genuinely unverified aspects, and safe scratch cleanup including disclosure of the **pre-existing 2813 unowned entries**. Use `COMPLETE` only for fully evidenced technical implementation; otherwise `STOPPED` with the precise code/host/contract obstacle. No independent `ACCEPT` claim.

On `COMPLETE`, **leave ACTIVE installed and R02.B in ROADMAP**, because the Chief Architect must independently accept the exact substantive report and the Owner separately authorizes reconciliation and stage advancement. The immediate deliverable is a secure, green, evidence-backed R02.B staging correction ready for independent acceptance—not a release or deployment.

**End of SOLO-WO-050 R01.**
