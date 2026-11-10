# Execution Report

**Order:** `SOLO-WO-052 R01` - Complete R02.B F2/F5: Windows direct-root, handle-only staging lifecycle.
**Disposition: `COMPLETE`** (technical execution only; not Architect ACCEPT, not R02.C, not release).
**Substantive revision:** `46b3148d1cca1199f3b9bdcebfff0be6143bbe15` (merge of `1d6faaf0` + `826a1c77`), tree `639a1b59f20e577b47e0cfdd34352d6b44f5607c`.

## 1. Owner-selected correction, implemented

The Windows staging namespace is **gone**. Neither host creates a staging-owned directory now; both stage directly beneath the already adopted, retained root handle.

- **Linux (preserved):** `openat2` with `BENEATH | NO_SYMLINKS | NO_MAGICLINKS | NO_XDEV` plus `O_TMPFILE | O_EXCL` anonymous inodes through the retained descriptor; zero directory entries at any instant and no pathname delete. Mandatory Btrfs fixture still runs on real CI.
- **Windows (rewritten):** `NtCreateFile(FILE_CREATE)` relative to `OBJECT_ATTRIBUTES.RootDirectory = held_root` with `OBJ_DONT_REPARSE`, an exact one-component unique name, `FILE_NON_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT | FILE_DELETE_ON_CLOSE`, `DELETE` rights and **share mode zero**; then `FileDispositionInfoEx(DELETE | POSIX_SEMANTICS)` on that same held handle **before the first plaintext byte**; the handle is retained for bounded positional reads after `finalize`. `create_staging_namespace`, `namespace_path`, the namespace identity/disposal path and the finalize-time directory delete were deleted outright, so no cleanup-by-name exists to be misused.
- **Honest trade-off:** on the tested host a Windows delete-pending name remains *enumerable* while the retained handle is open. It cannot be reopened, read, written, truncated, mapped or renamed into a usable alias, and it disappears when the last handle closes, including at process death. `FILE_DELETE_ON_CLOSE` is retained as the crash net. This is not the Linux zero-entry promise and no test claims it is.
- `visible_staging_entries()` no longer consults a path: it counts only **openable** `shard-*` names in the adopted root, so a pending name is measured rather than assumed away.

## 2. Previous diagnostic evidence (unchanged facts)

`FileDispositionInfoEx` succeeded on a shard but its entry stayed `DELETE_PENDING` (by-name probe returned `2`), so deleting the containing namespace failed with `ERROR_DIR_NOT_EMPTY` (`145`). Those codes belonged to the removed namespace design; they are not present in the final lifecycle.

## 3. Verification (real hosts, one revision)

| Evidence | Result |
| --- | --- |
| `rust-verify` at `826a1c77` (run `42801231741`) | **success**, ubuntu-24.04 **and** windows-2025 |
| `rust-verify` at `46b3148d1cca1199f3b9bdcebfff0be6143bbe15` (run `42802621342`) | **success**, both hosts |
| `source_set_staging` on Windows | **10 passed, 0 failed** - impostor dirs (empty and populated) survive, junction/component refusal, unwind, injected write/finalize, bounded abort child, scaling, pending-name unopenability |
| `source_set_candidate` on Windows | previously-blocked whole-declaration successes now pass |
| Linux Btrfs `O_TMPFILE` | executes on real CI; zero created entries; sentinels intact |
| Mandatory checks | fmt, strict Clippy, architecture 4 packages / 29 cases, locked graph, refreshed RustSec, cargo-deny, release CLI smoke, Linux reproducibility - all green |

No test was removed, skipped or weakened: the two assertion corrections are a host split (equality after the last handle closes on Windows, unchanged on Linux) plus a real openability measurement.

## 4. Gates

H1 **PASS** (full two-host suite) - H2 **PASS** (held-root confinement, both hosts) - H3 **PASS** (Btrfs anonymous lifecycle) - H4 **PASS** (component-relative, reparse-refusing root) - H5 **PASS** (exclusive handle, disposition before first byte, retained reads, crash cleanup via delete-on-close) - H6 **PASS** (no unowned deletion possible: no directory is created or removed; impostors and sentinels byte-identical) - H7 **PASS** (injected faults, real denial, unwind, abort child) - H8 **PASS** (`unsafe` confined to the named native helpers; no dependency, feature, workflow or lock change) - H9 **PASS** (same-revision both-host green plus report-HEAD green) - H10 **PASS** (four WIP files, clean tree, canonical report).

R04: F1, F3, F4, F6, F7, F8 unchanged. **F2 and F5 close technically** on the evidence above. Residual risk, stated plainly: a Windows pending name is visible to directory enumeration until the owning handle closes; no plaintext is reachable through it, and no directory of ours can be left behind because none is created. R02.B remains **OPEN** for independent Architect acceptance.

## 5. Ledger

Implementation commits: `3100809`, `42ac3a3`, `b1766e5`, `8f30cfe`, `7885dbf`, `a8dc2ba`, `786913d`, `6448f9c`, `826a1c7` on `solo-wo-051-r01`, integrated by merge `46b3148d1cca1199f3b9bdcebfff0be6143bbe15`; ACTIVE installation `1d6faaf0`; earlier STOP report `79de89da`.
Changed paths: `crates/solo-adapters/src/source_set_staging.rs`, `src/source_read_native.rs`, `src/source_set_candidate.rs`, `tests/source_set_staging.rs`. Untouched: `Cargo.toml`, `Cargo.lock`, `deny.toml`, workflow, checkers, SOPs, `lib.rs`, candidate test expectations.

Git state: `main` = `46b3148d1cca1199f3b9bdcebfff0be6143bbe15`, remote `origin` synced, one worktree, one checkout, `git status` empty, `target/` removed, no task scratch retained.

**Disposition: `COMPLETE`** - R02.B technical execution finished; R02.C not started.