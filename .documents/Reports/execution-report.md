# Execution Report

**Order:** `SOLO-WO-051 R01` - complete R02.B F2/F5 staging correction and publish only verified green source on `main`.
**Disposition: `STOPPED`.** The Linux half of the correction is implemented and **proven green on the real Btrfs fixture**; the Windows half compiles, lints clean and passes its own adversaries, but the pre-existing whole-candidate suite still fails on Windows 2025 because namespace creation returns `SOURCE_UNSAFE_PATH`. `main` therefore carries **no** product change and remains green. The complete implementation is preserved on the authorized branch `solo-wo-051-r01`.

## 1. Preflight and transition

Anchor `625a3798fee1177e9ba2f26aea7bb6f87621243e` (tree `957406a8e3b210f02caeda93b084e9ec385a0625`), `HEAD == origin/main`, one worktree, sole `origin`, four WIP files, clean tree except the Owner-installed ACTIVE. Frozen blobs verified unchanged (staging `e20054c7…`, native `ab6a392e…`, staging tests `dd2b71f3…`, candidate `311d44c8…`, candidate tests `4c3cbad0…`, `lib.rs` `31bee760…`, NEXT/ROADMAP/OUTLOOK unchanged).

ACTIVE-only transition on `main`: **`b5bf7e7f1f780328b0115adb16c88ee22920aada`**, tree `e8b3230c7d8f82589141f17615d456ef8e79d051`. Task branch `solo-wo-051-r01` created from it in the same checkout.

## 2. Implementation on the branch (preserved, not published)

Branch tip **`4a389350710f6461bb4ea4f9dcad60ffb45e2055`**, commits `c3b36952`, `355e5702`, `7677757d`, `6bf8d09e`, `7ad1a64a`, `c4c2adf5`, `4a389350`.

- **Linux (F2):** `StagingRootCapability::adopt` no longer inspects the path with `symlink_metadata`; the adapter parses the absolute path by component (refusing NUL, `.`, `..`, prefixes and relative forms) and resolves it from a descriptor on `/` with `rustix::fs::openat2` under `BENEATH | NO_SYMLINKS | NO_MAGICLINKS` and `RDONLY | DIRECTORY | CLOEXEC`. No `NO_XDEV`, no `canonicalize`, no path fallback. Shards are `O_TMPFILE | O_EXCL | RDWR | CLOEXEC` inodes opened through the retained root; every namespace directory, name-directed create and name-directed unlink path was deleted. A root unlinked while held is refused from the descriptor's own link count.
- **Windows (F2/F5):** drive-root bootstrap plus component-relative opens (`NtCreateFile` with `RootDirectory`, `OBJ_DONT_REPARSE`), with create/delete rights requested **only** on the final adopted root and traversal rights on intermediates. One namespace and each shard are created with `NtCreateFile(FILE_CREATE)` relative to held parents, shard share mode **0**, `FILE_DELETE_ON_CLOSE`, and the transient name detached by `SetFileInformationByHandle(FileDispositionInfoEx, DELETE | POSIX_SEMANTICS)` on the shard's own handle. The owned namespace is disposed **by its held handle** in `finalize` before success is exposed, and best-effort in `Drop`; no `remove_dir`, `remove_file` or name lookup can delete anything.
- New Windows `unsafe` is confined to `source_read_native.rs` for those named calls, with per-call `// SAFETY:` notes; the original-source reader is unchanged.

## 3. Real CI evidence

Dispatched `rust-verify.yml` on the branch (runs `42782385103`, `42783397886`, `42784061919`, `42784787493`, `42785539803`, `42786258870`, `42786901491`).

- **Ubuntu 24.04: green from run `42784787493` onward**, including the mandatory Btrfs case: `SOLO_TEST_SOURCE_FS_ROOT` was present, the fixture reported `fstype=btrfs`, `O_TMPFILE` shards were created, read and dropped, and the caller directory gained **zero staging-owned entries** across write, finalize, drop, unwind, injected faults and the bounded abort child. Root/intermediate symlink refusal, traversal refusal, unlinked-root refusal, real permission denial and handle scaling also passed.
- **Windows 2025: red.** Format, strict lint and compilation now pass, and the staging adversaries themselves are not reached because the **pre-existing** `tests/source_set_candidate.rs` fails first: `refusal: SourceReadRefusalV1 { kind: UnsafeLocation }` from namespace creation. Earlier Windows defects found and repaired during this run: missing `FILE_ADD_FILE`/`FILE_ADD_SUBDIRECTORY` rights, create/delete rights wrongly requested on intermediates such as `C:\Users`, and `FILE_OPEN_REPARSE_POINT` supplied with `FILE_CREATE` (an open-only option). One further cause remains unidentified within this session.

## 4. Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| H1 | **UNVERIFIED** | Linux M1–M14 regression green (REAL); Windows suite red |
| H2 | **PASS (Linux)** | REAL: intermediate/final symlink, traversal, unlinked-root refusals |
| H3 | **PASS** | REAL: Btrfs `O_TMPFILE`, zero staging-owned entries, foreign sentinel intact |
| H4 | **UNVERIFIED** | Windows junction/root adversaries not reached |
| H5 | **UNVERIFIED** | Windows detach semantics implemented, not yet exercised |
| H6 | **UNVERIFIED** | impostor adversaries not reached |
| H7 | **PASS (Linux) / UNVERIFIED (Windows)** | REAL injected write/short/flush/finalize + real EACCES; abort child |
| H8 | **PASS** | `unsafe` confined; no dependency, feature, workflow or lock change |
| H9 | **FAIL** | no two-host green at one candidate tree |
| H10 | **PASS** | one ACTIVE transition, four WIP files, clean `main`, branch preserved |

R04 crosswalk: F1, F3, F4, F6, F7, F8 unchanged; **F2 partly proven (Linux), F5 not yet proven**. R02.B remains OPEN; R02.C not started; no self-ACCEPT.

## 5. Preserved state and next step

The branch `solo-wo-051-r01` is deliberately **retained** at `4a389350…` as the only copy of the implementation. `main` is unchanged apart from this report. Local Rust `1.99.1` (user scope) is used for `cargo fmt`; no MSVC toolchain was installed and no global package was added. The 2813 pre-existing unattributed `.temp/` entries were not touched.

Next: reproduce the Windows `UnsafeLocation` from namespace creation with the staging adversaries ordered before the candidate suite, then correct it inside the same native boundary and re-run both hosts.

**Disposition: `STOPPED`** - no security claim is made for the Windows lifecycle.