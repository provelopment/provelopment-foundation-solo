# Execution Report

**Work order:** `SOLO-WO-047 R01` — R02.B third bounded slice: capability-confined `SourceRead` and physical-identity evidence.
**Result:** **STOP** under §1 and §7.3 of the order. No source file, dependency, lockfile, checker, workflow or licence file was written. The mandatory proof-before-library audit found two independent, individually fatal gaps that the exact authorized pin cannot close.

## 1. Verified starting state and issuance

* Required start confirmed exactly: remote HEAD `4b50ef3ab8e34e44f4e9a884eda600476eedb259`, tree `95d4a167c7daf556d89412fb7a9027f81af2f9db`, `main` tracking the sole approved `origin`, one worktree, zero stashes, four WIP regular files with no subdirectory, one canonical report.
* All seventeen §3 baseline blobs compared: **zero drift**. The five §2 paths that had to be absent were absent in both the tree and the checkout.
* Issuance `224ff56` (tree `7f04535a`): ACTIVE committed byte-identically, blob `c9870fc06351bf4175f1728c5455f1c16999c34b`. Run `42576108632` green — Ubuntu job `129024075025` (22 applicable steps), Windows job `129024075397` (19 applicable steps), zero non-success steps.
* NEXT neutralized in its own commit `1477c2b` (tree `24de656b`): 49-byte heading, blob `92fa562b1af0c54391851aa5c79deb1631127069`. Run `42576284482` green — Ubuntu job `129024625852`, Windows job `129024626177`.
* `plan-roadmap.md` was never touched; ROADMAP remains `18975332` and `R02.B`, `R02.C` and Stage 02 remain open.

## 2. The blocker, part A — Windows physical identity is unavailable on stable Rust

The order requires Windows evidence of volume plus file-index identity and reparse/alias control on the actual handle. On the pinned stable toolchain the standard library marks exactly those accessors as unstable. From the `std 1.99.1` reference for `std::os::windows::fs::MetadataExt`:

* `volume_serial_number() -> Option<u32>` — nightly-only experimental API, feature `windows_by_handle` (#63010).
* `number_of_links() -> Option<u32>` — nightly-only experimental API, feature `windows_by_handle` (#63010).
* `file_index() -> Option<u64>` — nightly-only experimental API, feature `windows_by_handle` (#63010).

Only `file_attributes`, `creation_time`, `last_access_time`, `last_write_time` and `file_size` are stable. By-handle volume serial and file index are therefore reachable only through a nightly feature build, or through direct `unsafe` FFI via `windows-sys` or `winx`. Both routes are closed here: the toolchain is pinned stable, no direct OS crate is approved, and no `unsafe` block may be added while the crate-level `#![forbid(unsafe_code)]` stays in force. `cap-std` and `cap-primitives` publish no by-handle `BY_HANDLE_FILE_INFORMATION` accessor, so the `windows-sys` already inside their closure cannot be reached safely either. Hardlink, alias and case-collision refusal on the actual handle cannot be implemented, and the §5.5 cases cannot be satisfied or honestly reported as verified.

## 3. The blocker, part B — no no-follow open exists in the authorized API

The order requires refusal of any symlink, junction or reparse point at root, intermediate directory or leaf, proven by a handle-based mechanism. The audited 4.0.3 surface cannot express that request at all:

* `cap_std::fs::OpenOptions` exposes exactly `new`, `read`, `write`, `append`, `truncate`, `create`, `create_new`, plus a Unix-only `OpenOptionsExt` carrying `mode` and `custom_flags`. There is no `follow`, `follow_symlinks` or `FollowSymlinks` control.
* `cap_primitives::fs::OpenOptions` 4.0.3 documents the identical seven-method surface, and likewise none that selects follow behaviour.
* `cap_std::fs::Dir::open_with` and `open_dir` accept no follow parameter; `Dir` imports the `FollowSymlinks` type internally and chooses the policy itself.
* `Dir::symlink_metadata` exists, but using it as a pre-check and then opening is precisely the initial-check-then-unconstrained-open pattern the order classes as categorically insufficient.

cap-std does confine a caller to a root directory, and that confinement is real. Following a symlink that stays inside the root is not the same property as refusing to follow it, and the order states explicitly that `cap-std::Dir::open` is not proof of symlink refusal, physical identity, race freedom or hardlink safety. The audit confirms the pin offers no public means to close that gap in safe Rust on the permitted paths.

## 4. The closure that would have been added, and why it does not help

Had the pin been installed, `cap-std 4.0.3` would have pulled `cap-primitives ^4.0.3`, `io-extras ^0.19.0`, `io-lifetimes ^3.0.1`, and `rustix ^1.0.0` with the `fs` feature on non-Windows targets. `cap-primitives 4.0.3` in turn requires `ambient-authority ^0.0.2`, `fs-set-times ^0.20.0`, `io-extras ^0.19.0`, `io-lifetimes ^3.0.1`, `ipnet ^2.5.0`, `maybe-owned ^0.3.4`, `rustix` with `fs`, `process`, `termios` and `time` on non-Windows, `rustix-linux-procfs ^0.1.1` on Linux and Android, and on Windows `windows-sys >=0.60, <0.62` with eight Win32/Wdk feature groups plus `winx ^0.36.0`. That is a materially larger platform-split closure than the accepted fifteen packages, and its licence is `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`, which would have required its own evidence and an allow-list decision in `deny.toml`.

None of that expansion buys the two missing capabilities. The FFI crates are present but not exposed by any safe public API, and the no-follow selector does not exist at either layer. Installing the pin would have produced a bigger, more privileged dependency graph that still could not produce the required proofs, which is exactly what §7.3 asks this agent to establish before touching the lockfile.

## 5. Why none of this is waivable under this order

* §4 forbids `canonicalize`, string-prefix comparison, path `metadata` alone, a follow-symlinks API and any pre-open check as security evidence.
* §5.4 and §5.5 make the leaf, intermediate, loop, junction, swap, volume-boundary, hardlink and alias cases mandatory, and state that partial support on only one host is not a pass.
* §1 and §7.3 direct a STOP with a concrete architectural or security decision record when the mechanism cannot be implemented and executably proven with this exact pin, stable toolchain, listed paths and safe Rust, and forbid publishing tentative source code as if it were secure.
* §8 forbids weakening an invariant, adding an unapproved crate or `unsafe`, and substituting permissive errors or untested platform assumptions.

Writing a reader that opened files through `cap-std` and reported success would have been a fabricated guarantee: it would follow leaf symlinks, could not distinguish a hardlink alias on Windows, and could not resample a stable by-handle identity. That is the single most damaging outcome available to this run, so nothing was published.

## 6. The smallest Owner decision this STOP requires

Exactly one of the following, then this slice can be re-issued:

* **(a) Approve an explicit, separately reviewed platform layer.** Direct `rustix` for Unix (`O_NOFOLLOW`, `fstat` device, inode and link count) and direct `windows-sys` for Windows (`CreateFileW` with `FILE_FLAG_OPEN_REPARSE_POINT` and `FILE_FLAG_BACKUP_SEMANTICS`, then `GetFileInformationByHandle`), plus a bounded `unsafe` policy that names the crate, the single file, the review duty, and the replacement of `#![forbid(unsafe_code)]` for that crate only. `cap-std` may remain the confinement primitive above such a layer, but it cannot supply either property itself.
* **(b) Replace the conditional `cap-std` selection** in Architecture §6A.8 with a different audited mechanism that already exposes no-follow opens and by-handle identity on both hosts on stable Rust.
* **(c) Reduce the acceptance requirement** so this slice proves only logical reservation plus root confinement, deferring physical identity and leaf no-follow to a later Stage. Only the Owner can make that requirements change, and it would leave the §5.4 and §5.5 cases unsatisfied rather than verified.

Whichever is chosen also settles two smaller items: the allow-list entry for `Apache-2.0 WITH LLVM-exception` if the cap-std family stays in the graph, and approval of the exact expanded closure before a lockfile is written.

## 7. What this run explicitly does not claim

Nothing was implemented, so nothing is claimed. No `SourceRead` port or adapter exists, no `cap-std` dependency was added, `Cargo.lock` still holds the accepted fifteen packages, and no checker, workflow, licence or security file changed. The accepted manifest parser, lexical locator, provisional selection policy and inert body preflight remain exactly as accepted: they still open no file and still prove no filesystem state. `R02.B`, `R02.C` and Stage 02 all remain open.

## 8. Preserved state

ACTIVE still holds this order; a STOP does not consume or neutralize authorization. NEXT is neutral at the approved 49-byte heading. ROADMAP is byte-identical, so no `R02.B`, `R02.C` or Stage 02 item was consumed. The earlier STOPs and the R03 ACCEPT stand unwaived, and the previous report text remains in Git history.

## 9. Ledger actually used, CI and cleanup

Only three of the sixteen permitted paths changed: `.documents/WIP/workorder-active.md` (issued, byte-identical), `.documents/WIP/plan-next.md` (neutralized to the approved heading), and `.documents/Reports/execution-report.md` (this report). No other file in the repository was created, modified or deleted.

This report is verified by the run that its own commit triggers; that outcome is not asserted here in advance. The two issuance revisions are verified above with their run and job identifiers.

No task-owned scratch remains: no `.temp/` material and no Cline temporary files were created this run, and no fixture directory was made because no test could be executed locally. The four pre-existing `.temp/` folders from earlier orders (`architecture-fixtures`, `rt`, `w032`, `w033`) belong to those orders and were left untouched rather than deleted.

Disclosure: this environment has no Rust toolchain, so no `cargo` command was run locally and no local compiler, test, format or lockfile result is claimed anywhere in this report.
