# Execution Report

**Order:** `SOLO-WO-048 R04` — protected staging integrity, confinement and transaction refusal.
**Disposition:** see §7. Implementation HEAD `1ba151495703b8d49b859d2261e5d901496216e8` is green on Ubuntu 24.04 and Windows 2025 (run 161). R02.B remains **OPEN**, R02.C is unstarted, ROADMAP/NEXT unchanged; the Chief Architect decides `ACCEPT / REVISE / STOP`.

## 1. The confinement defect and its repair

The diagnosis was exact: `begin_with_faults()` dropped the namespace handle, `open_staging_object()` then reopened the namespace **by pathname**, and `(device, inode)` cannot establish ownership once an inode can be recycled. Run 155 passed by chance; run 156 failed on identical code.

| Correction | Implementation |
| --- | --- |
| **A. Retain the descriptor** | `SourceSetStagingV1` now holds `namespace_dir: File` — the independently opened, identity-verified namespace handle — for the whole transaction and cleanup lifecycle. The adopted **root** handle is retained as before and, on Linux, additionally cloned (`root_dir`) so cleanup never re-walks a path. Root acquisition is unchanged: the parent of the created namespace must still be the adopted root's identity, so a root replaced between adoption and begin refuses before anything is created. |
| **B. Descriptor-relative creation** | `open_unnamed_staging_object` takes the retained `&File` and issues `openat(dirfd, ".", O_TMPFILE \| O_EXCL \| O_RDWR \| O_CLOEXEC)`. The namespace pathname is **never** reopened to create an object. Windows keeps its protected sharing implementation (create-new with `FILE_SHARE_DELETE` only, then unlink), which still needs a name because the standard library cannot create relative to a handle; its `FILE_ID_INFO` object id is not recycled, so the confirmations either side of the create are sound there. No new `unsafe`, FFI, dependency, feature or module. |
| **C. Fail closed** | Two independent controls, neither dependent on inode reuse: (1) the descriptor's **link count** is zero exactly when the directory has been unlinked, so `open_object` refuses with `SOURCE_UNSAFE_PATH` before creating anything; (2) any failure of descriptor-relative creation that is not a genuine permission denial or a genuine capacity error is a confinement failure, refused as `SOURCE_UNSAFE_PATH` rather than as absence or a shorter member. A substituted pathname cannot receive a staged byte, because creation never resolves the pathname at all. |
| **D. Cleanup ownership** | `Drop` no longer deletes the namespace by pathname unconditionally. Ownership is decided first (the entry must still resolve to the recorded namespace), the removal is **never recursive**, and on Linux it is `unlinkat(root_fd, name, AT_REMOVEDIR)` — no path component is re-walked. **Explicit residual:** the standard library offers no descriptor-relative rmdir, so a swap confined to the window between the ownership decision and the removal is not excluded; the consequence is bounded because only a directory inside the adopted root is considered and only an **empty** one can be removed. No staged content, no Owner material and no populated adversary directory can be deleted; anything not provably ours is left in place. |
| **E. Documentation** | The comment claiming an immutable `ctime` discriminator is gone. `ObjectIdentityV1` now states that identity is only a secondary control — Linux recycles inodes and a directory's change time moves with its contents — and that confinement rests on retained descriptors. |

## 2. Verification of the adversary

`a_substituted_namespace_is_refused_before_any_body_is_staged` was **strengthened, not relaxed**: 16 rounds, each deleting the store's own namespace and creating an impostor that holds a file of its own, then asserting that staging refuses, that no staged object appears in the impostor, that the impostor and its content survive cleanup, and that an Owner-created sentinel survives. Populating the impostor makes the survival assertion independent of inode recycling, and the link-count control makes the refusal independent of it too. `replacing_the_staging_root_between_adopt_and_begin_is_refused` (Unix), `a_real_permission_denial_refuses_as_permission_denied` (Unix, real `EACCES`), `an_unwind_during_staging_removes_only_its_own_namespace` and the Windows reparse case complete the set. The Windows case is a real decision procedure: it executes the adversary where the host permits creating a reparse point and **fails if adoption accepts the link**, or records the specific OS error where the privilege blocks it.


## 3. F1–F8 acceptance matrix

| Gate | Result | Evidence |
| --- | --- | --- |
| **F1 real-handle integrity** | **PASS (both hosts)** | Host experiment: with `share_mode(FILE_SHARE_DELETE)` a second handle is refused **read** (`raw=32`) and **write** (`raw=32`) while the object handle is live, unlink still succeeds, the path then does not exist and sealed bytes still read. In-repo: nameless-object, former-name-alias and exact read-back tests |
| **F2 retained confinement** | **PASS for namespace substitution, root replacement and symlinked roots**; **Windows reparse branch `UNVERIFIED`** | Substitution adversary over 16 rounds (§2); root replacement (Unix); symlinked root (Unix); Windows reparse case executes and passed in run 161, which **excludes** the security-failure outcome — whether the link was created or the host denied the privilege is recorded by the test at runtime but is not retrievable through the unauthenticated annotation channel, so the executed branch stays `UNVERIFIED` rather than being called a pass |
| **F3 protection lifecycle** | **PASS (both hosts)** | Sealing, repeated bounded reads across a shard boundary, sealed-store refusal of a late member, exact per-member length attribution |
| **F4 injected I/O** | **PASS; real ENOSPC `UNVERIFIED`** | Production-path injection for write-after-success, short write, flush and finalization; `EACCES` on Linux is a **real** host denial, not a classified code. Real disk exhaustion was **not** induced — distinct from the injected capacity faults, which are deterministic and bounded |
| **F5 all-or-refused and cleanup** | **PASS (both hosts)** | Unwind-during-staging removes only its own namespace; Owner sentinel preserved; impostor directory and its contents preserved over 16 rounds; no partial candidate exists by construction. This is panic/unwind testing, **not** abrupt process termination, which was not exercised |
| **F6 unchanged authored policy** | **PASS (both hosts)** | All R01 and R03 regressions run unmodified; both authority constants remain `false` |
| **F7 source and supply chain** | **PASS** | Edits confined to the eight-path ledger; no crate/feature/lockfile/workflow/checker/SOP/README/architecture/WIP change; no new `unsafe` or FFI; `cargo-deny` with a fresh RustSec refresh and the advisory negative probe refusing `time 0.3.46` |
| **F8 two-host end-to-end** | **PASS at implementation HEAD `1ba15149` (run 161)**; report-only HEAD stated in chat | Ubuntu `129653336762` and Windows `129653336430`, both success: locked fetch, format, strict Clippy, all-target tests, architecture `packages=4`, release build, CLI smoke, refusal self-test, byte comparison |

## 4. Run history — every intermediate failure and its cause

| Run | Commit | Cause and correction |
| --- | --- | --- |
| 149 / 150 | `445b685c`, `83a8b0f2` | Ubuntu strict lint: `STAGE_OBJECT_PREFIX` compiled but unused on Linux. Windows checker: `use std::fs::{self, OpenOptions};` unresolved. Fixed in `6ee3ec4a` exactly as directed |
| 152 | `6ee3ec4a` | Windows green; Ubuntu failed the substitution adversary: `(device, inode)` was recycled |
| 153 | `851c813f` | My `ctime` discriminator broke **ten** Linux candidate tests — writing members changes the directory's ctime — plus two fixture defects of mine and a reparse assertion. Reverted in `d7101137` |
| 154 | `d7101137` | Windows green; Ubuntu: my fixture adopted a root it had never created |
| 155 | `83b35181` | **Green by chance** — the same code failed again in 156 |
| 156 | `7cef9048` | Report-only HEAD red: the substitution adversary, the defect §1 corrects |
| 159 | `819985ca` | Windows green; the descriptor-relative refusal fired but with a different registered code than asserted |
| 160 | `6e672cc6` | Windows green; same assertion, because the code depended on which errno the file system returned |
| **161** | **`1ba15149`** | **Both hosts green**, with the refusal decided by the retained descriptor's link count instead of an errno or an inode number |

## 5. Declared `UNVERIFIED`

1. **Windows reparse branch** — executed as a test in run 161 and passed, which excludes adoption accepting a reparse root; the specific branch taken is not retrievable without the raw job log.
2. **Real ENOSPC** — not induced; injected capacity faults are deterministic but are not a substitute for it.
3. **Abrupt process termination** — not exercised; unwind testing is distinct and is what F5 proves.
4. **Cleanup residual window** — ownership decision and removal are not atomic in either host's standard library and no descriptor-relative rmdir exists; stated explicitly in §1 D rather than papered over.

## 6. State and cleanup

- ACTIVE still holds R04 (`06b911a3…`); NEXT neutral `92fa562b…`; ROADMAP `18975332…` still contains R02.B; OUTLOOK unchanged. No closure, no neutralization, no R02.C work, no self-approval.
- Clean tree, one checkout and one worktree, sole `origin`, exactly four WIP files.
- Task scratch removed: `.temp/toolchain`, `.temp/feasibility`, workspace `target/`. Pre-existing non-task `.temp` material was left untouched.

## 7. Disposition

`COMPLETE` for the R04 technical scope, conditional on the report-only HEAD also passing; that result is stated in chat against the report commit's own SHA, since it cannot be embedded here. The confinement guarantee is now independently testable and does not depend on inode behaviour or on which error a file system returns. Items in §5 are reported as `UNVERIFIED`, never as passes.
