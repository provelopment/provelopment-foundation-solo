# Execution Report

**Order:** `SOLO-WO-048 R04` — correct R02.B protected staging integrity, confinement and transaction refusal.
**Disposition: `STOPPED`.** The §4 feasibility gate **passed** and a correction was implemented, but both mandatory two-host gates are red at the pushed HEAD and the failing evidence cannot be obtained with the available tooling. R02.B therefore remains **OPEN**: nothing was closed, neutralized or approved.

## 1. Order identity, repository preflight and baseline

| Item | Value |
| --- | --- |
| Installed order | `SOLO-WO-048 R04`, 20,421 B, 111 lines, blob `06b911a3aedef1558404577c0b40bf95ad643761` |
| Reviewed anchor | `9f587abb89e63067d96b1bc27ac53e497e5f50f5` — an ancestor of the pre-transition HEAD (it **was** that HEAD) |
| Intervening commits | none |
| Frozen substantive blobs | **19 of 19 exact**, including `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, the workflow, both SOPs, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, ROADMAP, NEXT, OUTLOOK and all six R03 adapter blobs |
| Working tree before transition | clean except the installed ACTIVE replacement |
| Repository state | one checkout, one worktree, sole `origin`, exactly four WIP files |
| ACTIVE transition | `3a08e1eb1cddae1e9283233235a87a25e78fda23` (that file alone), tree `babfd3fa01b5f66c089a9a983e0e25c61325a7a9` |

## 2. §4-I / §4-II feasibility: **PASSED**, with executed host evidence

The decisive question was whether an unnamed, un-aliasable staged object and a retained directory identity are reachable **without** new `unsafe`, new FFI, new crates/features or a new permanent path. They are, on both hosts, and this was established before product code changed — using throwaway experiments under Solo `.temp/` on the real Windows host.

| Platform | Primitive | Proof |
| --- | --- | --- |
| **Windows** | `OpenOptions::share_mode(FILE_SHARE_DELETE)` — safe std | Executed: a second handle for **read** is denied (`raw=32` ERROR_SHARING_VIOLATION) and for **write** likewise; `remove_file` still succeeds while the handle is open; the path then does not exist; positional reads still return the sealed bytes; `share_mode(0)` cannot be unlinked (error 32), so delete-sharing is the minimal correct choice |
| **Windows** | `custom_flags(FILE_FLAG_BACKUP_SEMANTICS)` — safe std | Executed: `File::open(directory)` alone fails (`raw=5` ACCESS_DENIED), but the backup-semantics handle opens (`is_dir=true`), so a directory can be retained as a handle |
| **Linux** | `O_TMPFILE \| O_EXCL` via the already-pinned `rustix` | Already accepted **in this repository**: `snapshot_leaf` creates exactly this inode and the Linux D2 gate proves it on Btrfs |
| **Linux** | `std::os::unix::fs::MetadataExt` (`dev`,`ino`) | Stable std; the accepted `PhysicalIdentityV1` type is reused |

Rejected mechanisms, recorded so they are not retried: `canonicalize` or a final-component `symlink_metadata` alone (path, not identity); `File::set_permissions(readonly)` (does not revoke an existing handle); FNV or cryptographic digests (a substitute for protected storage, which the order forbids); `share_mode(0)` (blocks the unlink that removes the path); `MetadataExt::file_index`/`volume_serial_number` (still unstable — the `windows_by_handle` feature).

## 3. What was implemented (five ledger paths, two commits)

| Commit | Content |
| --- | --- |
| `445b685c` | unnamed objects, retained-identity confinement, fallible finalization, production-path fault seam |
| `83a8b0f2` | reuse the accepted `PhysicalIdentityV1` for staging identity (removes a Linux-only dead field) |

- **`source_read_native.rs`** (ledger item 8; safe helpers only — **no** new `unsafe`, FFI or feature): `open_unnamed_staging_object` (Linux `O_TMPFILE|O_EXCL`), `staging_handle_identity` (safe wrapper over the already-audited `FILE_ID_INFO` query; `handle_facts` widened to `pub(super)`), and one error-mapping helper.
- **`source_set_staging.rs`**: `adopt` retains a directory **handle plus its identity**; `begin` creates the namespace and then confirms, by identity, that its parent is the adopted root; every object creation is confirmed before *and* after; objects are created unnamed (Linux) or with delete-only sharing (Windows); `finalize()` re-confirms the namespace identity and every staged length, then seals; a `#[doc(hidden)]` `begin_with_faults` seam injects `Write`, `ShortWrite`, `Flush` and `Finalize` failures **through the production path**.
- **`source_set_candidate.rs`**: `finish()` now requires a fallible `finalize()` before any candidate can exist.
- **`source_set_staging.rs` tests**: added path-absence / no-writable-alias, namespace substitution, injected write, short-write, flush and finalization failures, and repeated sealed reads.
- Preserved unchanged: the R03 sequential driver, reservations-before-read, JSON-first selection, every body/manifest/EFS limit, both `false` authority constants, the original-source read guarantees and the D2/Windows native proofs.

## 4. Why this is a STOP

| Host | Failing gate | Detail |
| --- | --- | --- |
| Ubuntu 24.04 | **Strict lint (warnings are errors)** | exit 101; red in run 149 and still red in run 150 after the identity refactor |
| Windows 2025 | **Test (workspace, all targets, locked)** | `the_real_tree_satisfies_the_source_boundary_contract` and `the_self_test_matrix_is_complete_and_passing` both fail in the read-only architecture checker |

| Run | Commit | Ubuntu | Windows |
| --- | --- | --- | --- |
| 149 `42750648831` | `445b685c` | failure — Strict lint (`129613194596`) | failure — Test (`129613194831`) |
| 150 `42751551661` | `83a8b0f2` | failure — Strict lint (`129616096551`) | failure — Test (`129616096193`) |

- The architecture checker is **read-only under this order** (§7: no checker edits), and it now refuses the real tree. Its violation text is not retrievable: the job-log endpoint answers **403** to unauthenticated access and the run publishes **no artifacts**, so only panic *locations* are visible.
- Both failing checker tests share one root cause, because `check_real_tree()` supplies the graph the self-test also consumes. The most probable cause is the platform-specific `std::os` usage now present in `source_set_staging.rs` rather than confined to the approved native module — but that is a **hypothesis, not evidence**, and the order forbids blind iteration.
- The Ubuntu lint also survives the refactor and its message is equally unavailable.

Per §5 ("record FAIL or UNVERIFIED and STOP; do not claim COMPLETE from a green workflow alone") and §6.4, the order is **not** satisfied: F1 and F3–F5 have executed host evidence, F2 is implemented but **UNVERIFIED**, and F6–F8 are **FAIL** because two-host CI is red.

## 5. The smallest decision needed

Either of these unblocks a compliant continuation; both are Owner decisions, not mine:

1. **Confirm that item 8 covers the staging host primitives** — that directory-handle open, unnamed-object creation and handle identity may be *declared inside* `source_read_native.rs`, where platform-specific `std::os` and `rustix` already live, with `source_set_staging.rs` calling only safe crate-internal functions. This is the smallest change and preserves every security property.
2. **Or** decide that the architecture contract should admit a second, equally scoped platform boundary for staging — which is a checker change and therefore outside any current authority.

## 6. State, cleanup and what was deliberately not done

- ACTIVE still holds R04 (blob `06b911a3…`); NEXT remains the neutral `92fa562b…`; ROADMAP `18975332…` still contains R02.B; OUTLOOK unchanged. No closure, no neutralization, no R02.C work.
- `HEAD == origin/main`, `git status` clean, one checkout/worktree/remote, exactly four WIP files, and both authority constants remain `false`.
- Task scratch removed: the `.temp/feasibility` experiment crate and the pinned `.temp/toolchain` (1.7 GB) were deleted, as was the workspace `target/`. Pre-existing, not-task-owned `.temp` items (`architecture-fixtures`, `rt`, `target`, `toolchain-installer`, `w032`, `w033`) were left untouched.
- Not done, because the discipline forbids it: no checker, workflow, SOP, README or lockfile edit; no new dependency; no new `unsafe`; no native privilege; no history rewrite; no self-approval.
- Open risks: staged-object integrity is now structurally protected but **unproven on CI**; the confinement identity path is **unverified** on both hosts; a Linux-only lint of unknown content remains; and F2's Windows reparse adversary was never executed, so it is **UNVERIFIED** rather than PASS.

**Disposition: `STOPPED` — evidence preserved, blocker named, no substitute invented.**
