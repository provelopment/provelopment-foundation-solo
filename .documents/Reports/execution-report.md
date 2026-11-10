# Execution Report

**Order:** `SOLO-WO-048 R04` — corrective continuation of `0d7b929cf54d06b1d1e9cfda2c586e1461b23af7`.
**Disposition: `STOPPED`.** Defects A and B cannot be closed inside this order's own eight-path ledger: each requires either genuine native/unsafe expansion, which ledger row 8 forbids in terms ("STOP if genuine native/unsafe expansion is needed"), or a narrowing of an already accepted security contract. No production code was changed in this run; the implementation of record stays green at `1ba15149` (runs 161/162). **F2 and F5 are not accepted.** The smallest Owner decisions that would unblock each are in §4.

## 1. The three findings, confirmed in the code

**Defect A — adoption and creation are path-authoritative.** `StagingRootCapability::adopt()` calls `symlink_metadata(root)` (final component only) and then `open_directory(root)`, a pathname open. Intermediate components — symlinks, junctions, reparse points, mount substitutes — are never examined. `begin_with_faults()` then calls `create_private_directory(&namespace)`, a pathname `create_dir`, and only *afterwards* proves the parent by identity through `open_directory(&namespace.join(".."))`. That is precisely the check-after-path-based-creation pattern the correction forbids: the identity proof does not prevent the creation from having been redirected.

**Defect B — every cleanup path selects its victim by name.** `remove_namespace()` compares the resolved entry's identity and then removes the entry with `std::fs::remove_dir` (Windows) or `unlinkat(root, name, AT_REMOVEDIR)` (Linux). `unlinkat` pins the *parent* but still selects the **final entry by name**, so a replacement landing in the interval between check and removal is deleted although ownership cannot be proven at the deletion. The same shape appears in the early-error paths of `begin_with_faults()` (`remove_dir(&namespace)` at three sites) and in the Windows shard fallback (`remove_file` twice). The defence I offered last run — "only an empty directory can be removed" — does not hold: the correction requires that an *unowned empty* directory also survive.

**Defect C — the Windows reparse adversary accepts any error.** As written it passes on any `raw_os_error()` being present, and the passing branch is invisible in ordinary test output.

## 2. Feasibility of each correction against ledger row 8

| Correction | Linux | Windows |
| --- | --- | --- |
| **A. no-redirection adoption** | **Feasible and safe.** `rustix::fs::openat2(from "/", relative, OFlags::DIRECTORY\|CLOEXEC, Mode::empty(), ResolveFlags::BENEATH\|NO_SYMLINKS\|NO_MAGICLINKS)` gives kernel-enforced resolution over every component; `NO_XDEV` is deliberately omitted so no unrelated cross-device restriction is introduced. No new `unsafe`, FFI or crate | **Feasible for adoption only.** The audited component-relative machinery exists (`sys::adopt_root`, `NtCreateFile` per component with `OBJ_DONT_REPARSE`), and its `OwnedHandle` converts to `std::fs::File` through safe std |
| **A. no-redirection creation** | **Feasible.** `mkdirat(root_fd, name, 0700)` plus `openat(root_fd, name, O_DIRECTORY\|O_NOFOLLOW)` creates and acquires the namespace relative to the retained descriptor, so the root pathname is never resolved again | **Blocked.** The standard library cannot create relative to a handle. Doing so needs `NtCreateFile` with a `RootDirectory` — a **new native call shape** |
| **B. ownership-bound cleanup** | **Blocked as designed.** POSIX has no unlink bound to a directory's identity; `unlinkat` selects by name, and `renameat2` cannot prove ownership. The only safe alternative is to remove the named namespace entirely and place `O_TMPFILE` objects in the retained root, which is feasible | **Blocked.** Deleting the namespace needs `FileDispositionInformationEx` by handle — **native expansion**. The shard itself *can* be made handle-bound with `FILE_FLAG_DELETE_ON_CLOSE` (safe flag), but the namespace directory cannot |

**Conclusion.** The cleanup guarantee — "a transaction may never delete a directory or object whose ownership it cannot prove at the deletion operation" — has no mechanism in safe Rust on either host while a disposable named directory exists. On Windows, even the creation guarantee is unattainable. Since F2 and F5 must hold on **both** hosts, no partial implementation can make this order acceptable, so I did not ship one: a Linux-only patch would leave the Windows half of F2 and all of F5 failing while creating the appearance of closure.

## 3. What I deliberately did not do

- **No partial patch.** Changing Linux adoption alone would alter the security boundary on one host only while the acceptance gates remain unmet on the other.
- **No committed failing adversary.** §5 of the direction asks for an empty-directory replacement adversary that targets the check/deletion interval. Written against the current code it *demonstrates the defect*, i.e. it fails by construction. Committing it would leave `main` permanently red, which no authorization in this order covers, so it is specified below for the corrective order instead of being smuggled in or, worse, weakened until it passes.
- **No contract narrowing.** Eliminating the named namespace on Windows is only possible by keeping a visible shard name for the life of the transaction, which contradicts the accepted F1 evidence that the former name no longer reaches the object. Narrowing that test is not authorized.
- **No checker, workflow, dependency, WIP or architecture change.** Nothing outside `.documents/Reports/execution-report.md` was written.


## 4. The smallest Owner decisions required

1. **Authorize one audited native operation on Windows** — `NtCreateFile` with a `RootDirectory` relative object name, and `FileDispositionInformationEx` for delete-by-handle — so namespace creation, shard creation and namespace removal are all handle-bound. This is a native/unsafe expansion inside `source_read_native.rs` and must be explicitly scoped; ledger row 8 currently forbids it.
2. **Or authorize the design change on both hosts**: remove the disposable named namespace entirely. On Linux, `O_TMPFILE` objects are created in the retained root and cleanup deletes nothing. On Windows the same shape needs `FILE_FLAG_DELETE_ON_CLOSE`, and an explicit decision is required on the resulting trade-off — a visible shard name for the transaction's life, refused read/write/delete by share mode — which changes the accepted F1 evidence "the former name no longer reaches it". Choose option 1 for the stronger guarantee, option 2 for zero deletions.
3. **Authorize the empty-directory replacement adversary** to be committed as a *failing* test against the current implementation until the repair lands, or authorize the repair order first and the adversary with it. Either is fine; what is not acceptable is an adversary that cannot fail.

With decision 1 or 2, the remaining Linux work is one safe `openat2` helper plus `mkdirat`/`openat` namespace creation, and the remaining verification is the four adversaries already specified in §5 of the direction: intermediate-parent symlink, root replacement, ancestor substitution, and the real Windows reparse/junction case with strict classification (only `ERROR_PRIVILEGE_NOT_HELD` on reparse creation may be reported as privilege-blocked; any other error fails the test rather than passing it).

## 5. F1–F8 status

| Gate | Status |
| --- | --- |
| **F1** | Unchanged and previously evidenced (run 161, both hosts); the Windows share-mode and sealed-read facts stand |
| **F2** | **NOT ACCEPTED.** Defect A: intermediate components unverified on both hosts; Windows creation path-authoritative. Defect C: the reparse branch is not strictly classified |
| **F3, F4, F6, F7** | Unchanged and previously evidenced. Real ENOSPC and abrupt process termination remain **UNVERIFIED** and are distinct from the injected capacity and unwind tests |
| **F5** | **NOT ACCEPTED.** Defect B: every deletion selects by name |
| **F8** | The prior green two-host evidence stands for the code of record (`1ba15149`, run 161; report `0d7b929c`, run 162). This run's report commit is verified green separately; no failing test is introduced |

## 6. State and cleanup

- ACTIVE still holds R04 (`06b911a3…`); NEXT neutral (`92fa562b…`); ROADMAP (`18975332…`) still contains R02.B; OUTLOOK unchanged. **R02.B remains OPEN**, R02.C unstarted. No closure, merge, release, deployment or self-approval.
- Working tree clean apart from this report; one checkout, one worktree, sole `origin`, exactly four WIP files. No task scratch retained: no `.temp` material, no `target/`, no toolchain left behind.

## 7. Disposition

**`STOPPED`** — not because the work is hard, but because the two remaining defects each need a capability this order's own ledger forbids me to add, and I will not substitute a weaker design and call it fixed. The precise blockers are: (1) handle-relative creation and handle-bound directory deletion on Windows, requiring a scoped native authorization; (2) ownership-bound directory deletion on Linux, which is unattainable while a disposable named directory exists. Answer §4 and the repair is small, bounded and testable on both hosts.
