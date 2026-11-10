# Execution Report

**Order:** `SOLO-WO-050 R01` - complete R02.B staging-security correction (Linux Btrfs zero-name lifecycle; Windows handle-bound lifecycle).
**Disposition: `STOPPED`.** The correction was designed and implemented in this run but could not be brought to a verifiable, two-host green state within it. Rather than publish a revision that knowingly fails to compile, the working tree was restored to the anchor and **no product source change was published**. `main` remains green.

## 1. Preflight - verified

Root, `main`, `HEAD == origin/main == 811c509a…`, tree `9edb0db7…`, one worktree, sole `origin` (`provelopment/provelopment-foundation-solo`), four regular WIP files, clean tree except the Owner-installed ACTIVE. Frozen blobs confirmed unchanged: staging `e20054c7…`, native `ab6a392e…`, candidate `311d44c8…`, tests staging `dd2b71f3…`, tests candidate `4c3cbad0…`, `lib.rs` `31bee760…`, NEXT `92fa562b…`, ROADMAP `18975332…`, OUTLOOK `cb9c7a61…`. Prior canonical report blob `eb3dc7c6…` read and its `STOPPED` disposition treated as history, not as this run result.

## 2. ACTIVE transition - published and green

One ACTIVE-only commit **`2e6006c095887ead0b8cdab6dfa9e2089722ae98`** (tree `81c14c1acaeabc5655b9aeaa725bbd890e0dfdaa`). CI `rust-verify` run **166** completed **success** on `ubuntu-24.04` and on `windows-2025`.

## 3. Binding audit - REAL evidence, first time proven

Read directly from the locked crate sources in the local Cargo registry (`windows-sys 0.61.2`):

| Symbol | Evidence |
| --- | --- |
| `SetFileInformationByHandle` | `kernel32` link declaration, `(HANDLE, FILE_INFO_BY_HANDLE_CLASS, *const c_void, u32) -> BOOL` |
| `FileDispositionInfoEx` | `pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = 21i32` |
| `FILE_DISPOSITION_INFO_EX` | struct present with `Flags` |
| `FILE_DISPOSITION_FLAG_DELETE` / `_POSIX_SEMANTICS` | `1u32` / `2u32` |
| `FILE_CREATE` | `NTCREATEFILE_CREATE_DISPOSITION = 2u32` (a separate parameter from the options) |
| `FILE_DIRECTORY_FILE` / `FILE_NON_DIRECTORY_FILE` | `1u32` / `64u32` (`NTCREATEFILE_CREATE_OPTIONS`) |
| `FILE_DELETE_ON_CLOSE` / `FILE_OPEN_REPARSE_POINT` | `4096u32` / `2097152u32` |
| `DELETE`, `FILE_LIST_DIRECTORY`, `FILE_WRITE_DATA`, `FILE_WRITE_ATTRIBUTES` | present under `Win32_Storage_FileSystem` |

All authorised operations are therefore expressible inside the frozen feature set, with no new dependency, feature or declaration. This resolves the open binding question from the previous report.

## 4. Work implemented, then withdrawn

Linux: adoption replaced by component validation plus `openat2` from `/` with `BENEATH | NO_SYMLINKS | NO_MAGICLINKS` and `O_DIRECTORY | O_CLOEXEC`, no `NO_XDEV`, no `canonicalize`, no path fallback; shards opened with `O_TMPFILE | O_EXCL | O_RDWR | O_CLOEXEC` through the retained root; all namespace creation and name-directed removal deleted. Windows: `open_staging_root` bootstraps the drive root and opens each further component relative to the retained parent with `OBJ_DONT_REPARSE`; `create_relative` uses `NtCreateFile(FILE_CREATE)` with `RootDirectory` set to the held parent, share mode zero for shards, and `FILE_DELETE_ON_CLOSE`; `dispose_held` applies `FileDispositionInfoEx` with `DELETE | POSIX_SEMANTICS` to the held handle. The store was restructured accordingly. All of this was reverted (`git checkout`), and the three source blobs were re-verified at their anchor values.

## 5. Why this stopped - the precise obstacle

Compilation is available only on the hosted CI. The Windows host has no MSVC linker (`link.exe` absent; installing Build Tools is a global package installation and not authorised), and the local WSL2 host is ext4 without a toolchain or a linker. The mandatory adversary suite still targets the removed namespace API, so pushing the production rewrite would have been a knowingly broken revision, which the order forbids. Reaching green therefore required several CI cycles that this run could not complete. No weakened test, fake adversary or partial host half was published.

## 6. Gates

| Gate | Status | Basis |
| --- | --- | --- |
| H1 | **PASS (unchanged)** | run 166 green on both hosts; no source changed |
| H2, H3, H4, H5, H6, H7 | **UNVERIFIED** | correction withdrawn; F2 and F5 remain open |
| H8 | **UNVERIFIED** | attempted native expansion not published; zero new `unsafe` in `main` |
| H9 | **PARTIAL** | green at the transition HEAD only; no substantive HEAD |
| H10 | **PASS** | one ACTIVE transition, four WIP files, frozen blobs, clean tree, single report, scratch removed |

R04 crosswalk preserved: F1, F3, F4, F6, F7, F8 `PASS` (unchanged); **F2 `NOT ACCEPTED`**, **F5 `NOT ACCEPTED`**. R02.B **OPEN**, R02.C unstarted, no self-ACCEPT.

## 7. Toolchain and cleanup

Installed under the order environment authority: `rustup-init 1.29.3` from the official `https://win.rustup.rs/x86_64` (SHA-256 `38027D51D69A5D05493627A0634DBEA52D6CD793C8F072D2373D3F4163EEC7DB`), toolchain `1.99.1 (58cf8f9c3)` with `rustfmt` and `clippy`, installed with `--no-modify-path`; nothing global was installed. Removed as task-owned: the installer scratch directory under `.temp/` and the `target/` build directory. The **2813 pre-existing, unattributed `.temp/` entries were not touched**. Working tree is clean and `HEAD == origin/main`.

## 8. What remains

1. migrate the staging adversaries to the new API (root snapshot equality on Linux; namespace observation on Windows); 2. add the mandatory Btrfs `O_TMPFILE` test driven by `SOLO_TEST_SOURCE_FS_ROOT`, failing rather than skipping when the fixture is absent on CI; 3. add the real junction/symlink component-redirection adversary and the empty and populated impostor-survival adversaries; 4. add the bounded crash child; 5. re-apply the withdrawn production rewrite; 6. run the two-host CI to a common green; 7. publish this report as `COMPLETE`.

**Disposition: `STOPPED`** - correction not completed or verified; no security claim is made for either platform.