# Execution Report

**Order:** `SOLO-WO-051 R01` (Architect disposition: REVISE, then complete).
**Disposition: `STOPPED`** - one required guarantee is provably unsatisfiable on Windows Server 2025 with the authorized native boundary. `main` carries **no** product change; the complete implementation is preserved on `solo-wo-051-r01` at `7885dbf0a6d4c344e58248306b7c079b2766f78b`.

## 1. Correction of the previous report

The previous report wrongly stated that the Windows adversaries were never reached. Real job `129730310234` (run `42786901491`) executed `--test source_set_staging` with **7 passed, 3 failed**. Passing on Windows today: `an_impostor_at_the_namespace_name_survives_untouched`, `an_intermediate_junction_component_is_refused`, `an_unwind_during_staging_touches_only_its_own_objects`, `injected_write_failures_refuse_through_the_production_path`, `an_injected_finalization_failure_refuses_the_whole_set`, `a_bounded_abort_child_leaves_no_recoverable_stage_plaintext`, `staging_root_adoption_refuses_unsafe_shapes`. Failing: the three tests that require a successful `finalize()` or an empty namespace. `source_set_candidate`: 5 passed, 7 failed.

## 2. Root cause, proven on the real host

Temporary diagnosis (now removed) recorded the raw code of every failing native call:

- `dispose_held raw_error=145` (`ERROR_DIR_NOT_EMPTY`) - every namespace disposal.
- `probe_after_unlink raw_error=2` - immediately after a shard detach, reopening its name fails with a status mapping to `STATUS_DELETE_PENDING`.

So `SetFileInformationByHandle(FileDispositionInfoEx, DELETE | POSIX_SEMANTICS)` **succeeds** on a shard, but the host only marks the entry delete-pending: the name is unopenable (no reader, writer, mapper, no recoverable plaintext) yet still occupies a directory entry until the handle closes. The namespace is therefore never empty while a member stays readable, and its removal fails with 145. Granting `FILE_SHARE_DELETE` on the shard changed nothing (still probe=2), and `FILE_DELETE_ON_CLOSE` produced the same 145.

The conflict is minimal and exact: *members readable through retained handles after successful finalization* and *the owned namespace removed by its held handle before candidate success* cannot both hold on this host, because the removal requires an empty directory and the entries only vanish when the very handles that keep members readable are closed. The only ways forward are an Architect decision: (a) defer namespace disposal to store release, after shard handles close, accepting that a successful candidate leaves an empty owned namespace until then; or (b) drop the namespace on Windows and stage shards directly in the adopted root as delete-pending objects. Both preserve every F1/F2/F5 security property; neither is adopted here without approval.

## 3. Corrections delivered and preserved on the branch

- Real native-failure classification: `StagingNativeError::NativeFailure` now maps to an I/O refusal instead of being collapsed into `SOURCE_UNSAFE_PATH`, so a host failure is never concealed as a path refusal.
- The shard name is detached **before any byte is written**, so an abrupt termination can leave only an empty object; `FILE_DELETE_ON_CLOSE` was removed because it guarantees the very pending-entry state that blocks namespace removal.
- The invalid Windows test expectation was corrected: on Windows the root is asserted to hold exactly the store's own namespace plus the original snapshot, and the namespace must hold no openable shard name; the post-`finalize` snapshot equality remains required on both hosts.
- Windows adversaries that previously never ran now execute and pass (listed above), including both impostor cases, the junction refusal and the bounded abort child.

## 4. Gates

| Gate | Status | Evidence |
| --- | --- | --- |
| H1 | **UNVERIFIED** | Linux M1-M14 green (REAL); Windows candidate suite blocked by the disposition conflict |
| H2 | **PASS** | REAL: symlink, traversal, junction and unlinked-root refusals on both hosts |
| H3 | **PASS** | REAL: Btrfs `O_TMPFILE`, zero staging-owned entries, sentinels intact (Ubuntu) |
| H4 | **PASS (Windows)** | REAL: junction refusal executed; component-relative adoption confirmed |
| H5 | **PASS (Windows)** | REAL: no second reader/writer/mapper; detach proven delete-pending, not openable |
| H6 | **UNVERIFIED** | impostors survive (REAL), but success-path namespace disposal is the blocked requirement |
| H7 | **PASS** | REAL injected write/short/flush/finalize + real EACCES + abort child |
| H8 | **PASS** | `unsafe` confined to the named Windows helpers; no dependency, feature, workflow or lock change |
| H9 | **FAIL** | no two-host green at one candidate tree |
| H10 | **PASS** | ACTIVE transition `b5bf7e7f`, four WIP files, clean `main`, branch preserved |

R04: F1, F3, F4, F6, F7, F8 unchanged; **F2 proven on both hosts**, **F5 unaccepted** pending the decision in section 2. R02.B remains OPEN; R02.C not started; no self-ACCEPT.

## 5. State

`main` = `b5bf7e7f` (ACTIVE only) + this report. Branch `solo-wo-051-r01` merged with the published report commit and preserved at the SHA above; it holds the only copy of the implementation. Local Rust `1.99.1` used for `cargo fmt` only; no MSVC toolchain or global package added; the 2813 pre-existing unattributed `.temp/` entries were untouched; task scratch removed.

**Disposition: `STOPPED`** - exact OS errors and the minimal architecture conflict are reported for Owner/Architect decision, not worked around.