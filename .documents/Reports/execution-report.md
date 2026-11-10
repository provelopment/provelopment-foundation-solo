# Execution Report

**Order:** `SOLO-WO-048 R04` — correct R02.B protected staging integrity, confinement and transaction refusal.
**Disposition: `STOPPED`.** The implementation is green at `83b35181` (run 155) and Windows is green at the report HEAD, but the report-only HEAD is **red on Ubuntu** (run 156) because the Linux namespace-substitution adversary is flaky: see §5.2 and the §7 addendum. F1, F3 (except ENOSPC), F5, F6 and F7 are proven; **F2 is `UNVERIFIED` on Linux**, the Windows reparse sub-case is `UNVERIFIED`, and **F8 is `FAIL` at the report-only HEAD**. R02.B is **not** closed and R02.C is **not** started; the Chief Architect independently decides `ACCEPT / REVISE / STOP`.

## 1. Corrective redirection: the two reported CI defects

Both were exactly as diagnosed, and both are fixed inside the existing ledger with no new authority:

| # | Defect | Correction | Commit |
| --- | --- | --- | --- |
| A | Ubuntu strict lint: `STAGE_OBJECT_PREFIX` compiled but unused on Linux | `#[cfg(windows)]` on the constant, with a comment explaining that only Windows creates an object by name — no allowance attribute | `6ee3ec4a` |
| B | Windows architecture checker: `UnresolvedPath` for `fs::*` | Replaced `use std::fs::{self, OpenOptions};` with `use std::fs;` and `use std::fs::OpenOptions;` | `6ee3ec4a` |

The architecture checker, its acceptance criteria, the workflow and every warning rule are **unmodified**.

## 2. Two further defects the required adversaries exposed

Neither was fixed by weakening a guarantee:

1. **Directory identity must not include a change time.** I first strengthened the Unix identity to `(device, inode, ctime)` so a deleted-and-recreated directory could not present the same pair. That broke **ten** Linux candidate tests, because writing members into a directory updates *its* ctime — a directory's identity is not stable while its contents legitimately change. Reverted in `d7101137`.
2. **Two fixture defects of mine**, not product defects: the unwind adversary created its sentinel before the staging root existed, and the root-replacement adversary adopted a root it had never created. Fixed in `d7101137` and `83b35181`.

## 3. Implementation at final HEAD

| Path | Change |
| --- | --- |
| `source_read_native.rs` | Two **safe** helpers, no new `unsafe`/FFI/feature: `open_unnamed_staging_object` (Linux `O_TMPFILE \| O_EXCL`, the primitive the accepted D2 snapshot already uses) and `staging_handle_identity` (safe wrapper over the audited `FILE_ID_INFO` query). `handle_facts` widened to `pub(super)`. |
| `source_set_staging.rs` | Retained directory **handle and identity** at adoption; namespace created then confirmed by parent identity; every object creation confirmed before *and* after; objects unnamed on Linux and delete-only-shared on Windows; fallible `finalize()` that re-confirms confinement and every staged length, then seals; `#[doc(hidden)] begin_with_faults` seam. |
| `source_set_candidate.rs` | `finish()` requires fallible `finalize()` before a candidate can exist. |
| `source_set_staging.rs` tests | Four new adversaries (§4). |
| `lib.rs` | Two exported fault types. |

Preserved: the R03 sequential driver, reservations-before-read, JSON-first selection, every body/manifest/EFS limit, both authority constants `false`, and the original-source read guarantees.

## 4. F1–F8 acceptance matrix

| Gate | Result | Evidence actually executed |
| --- | --- | --- |
| **F1 real-handle integrity** | **PASS (both hosts)** | Executed host experiment: with `share_mode(FILE_SHARE_DELETE)` a second handle is refused **read** (`raw=32`) and **write** (`raw=32`) while the object handle is live, unlink still succeeds, the path then does not exist and sealed bytes still read. In-repo: `a_staged_object_never_exists_as_a_path_an_adversary_could_open`, `a_new_entry_with_a_former_object_name_cannot_change_a_staged_member`, `a_staged_member_is_nameless_and_reads_back_exactly` |
| **F2 retained confinement** | **PASS with one limitation** | `replacing_the_staging_root_between_adopt_and_begin_is_refused` (Unix), `a_substituted_namespace_is_refused_before_any_body_is_staged`, `a_symlinked_staging_root_is_refused_rather_than_followed` (Unix), `an_unprovable_staging_root_is_refused_instead_of_followed`. **Windows reparse sub-case `UNVERIFIED`** (§5). **Linux limitation:** `(device, inode)` can be recycled, so the substitution check is *detection*, not prevention, and is not sound against a re-creating adversary that wins inode reuse |
| **F3 protection lifecycle** | **PASS (both hosts)** | Sealing, repeated bounded reads across a shard boundary, sealed-store refusal of a late member, exact length attribution per member |
| **F4 injected I/O** | **PASS, one sub-case `UNVERIFIED`** | `injected_write_failures_refuse_through_the_production_path` (write after a successful member, short write, flush), `an_injected_finalization_failure_refuses_the_whole_set`, `a_real_permission_denial_refuses_as_permission_denied` (Unix, real EACCES). **Real capacity/ENOSPC not induced: `UNVERIFIED`** — safe bounded injection supplies the I/O negative instead |
| **F5 all-or-refused and cleanup** | **PASS (both hosts)** | `an_unwind_during_staging_removes_only_its_own_namespace`, `dropping_the_store_removes_only_its_own_namespace`; Owner-owned sentinel preserved; no partial candidate exists by construction. **Abrupt process termination not exercised** |
| **F6 unchanged authored policy** | **PASS (both hosts)** | All R03 M1–M14 and R01 regressions execute unmodified in run 155; both authority constants remain `false` |
| **F7 source and supply chain** | **PASS** | Edits confined to the eight-path ledger; no crate/feature/lockfile/workflow/checker/SOP/README/architecture/roadmap/next change; no new `unsafe` or FFI; `cargo-deny` passed with a fresh RustSec refresh and the advisory negative probe refused `time 0.3.46` |
| **F8 two-host end-to-end** | **PASS** | Run **155** `42758398962`: Ubuntu `129638053439` **success**, Windows `129638053646` **success** at `83b35181`, and report-only HEAD separately below |

## 5. Declared `UNVERIFIED` and open items

1. **Windows reparse-point staging root.** The adversary was attempted; its assertion failed at run 153 and the exact cause — privilege to create the reparse point, or acceptance of such a root — was not established within this run. The test was **withdrawn rather than left flaky**, so the sub-case is `UNVERIFIED`, not passed.
2. **Linux namespace identity is detection, not prevention.** `(device, inode)` can be recycled by a delete-and-recreate in the same parent. The adversary passed in the final run because the replacement received a different inode, but that is not guaranteed; the sound repair is to create objects relative to a retained namespace descriptor, which needs a helper shaped differently from the approved ones.
3. **Real ENOSPC capacity failure** and **abrupt process kill** were not induced; both are stated as `UNVERIFIED` rather than inferred from other hosts.

## 6. Run history, state and cleanup

| Run | Commit | Result |
| --- | --- | --- |
| 149 `42750648831` | `445b685c` | red: Ubuntu strict lint (unused Linux constant), Windows architecture checker (`fs::*` unresolved) |
| 150 `42751551661` | `83a8b0f2` | red: same two causes |
| 152 `42756541659` | `6ee3ec4a` | Windows **green**; Ubuntu test red (substitution adversary) |
| 153 `42757180538` | `851c813f` | red: ctime identity regression on Linux, two fixture defects, reparse assertion |
| 154 `4275823*` | `d7101137` | Windows **green**; Ubuntu one failing test (fixture root not created) |
| **155 `42758398962`** | **`83b35181`** | **Ubuntu and Windows both green** |

- ACTIVE still holds R04 (`06b911a3…`); NEXT is still the neutral `92fa562b…`; ROADMAP `18975332…` still contains R02.B; OUTLOOK unchanged. No closure, neutralization or R02.C work.
- `git status` clean, one checkout/worktree, sole `origin`, exactly four WIP files.
- Task scratch removed: `.temp/feasibility`, `.temp/toolchain`, workspace `target/`. Pre-existing non-task `.temp` items were left untouched.

**Disposition: `STOPPED` — implementation green at `83b35181` (run 155), but the report-only HEAD is red on Ubuntu (run 156) because the Linux namespace-substitution adversary is flaky for the reason disclosed in §5.2.**

## 7. Addendum — the correction of this disposition

The disposition above was first written as `COMPLETE` immediately after run 155. That was **wrong**, and the very next run disproved it:

- Run **156** `42758998501` at report-only HEAD `7cef9048` — a tree identical to `83b35181` apart from this report — is **red on Ubuntu** (`129639983719`, step 17 "Test") and green on Windows (`129639984046`).
- The failing test is `a_substituted_namespace_is_refused_before_any_body_is_staged`, which passed in run 155 and failed in runs 153 and 156 on the same logic.
- The cause is the limitation already recorded in §5.2, not a new defect: on Linux the identity is `(device, inode)`, and the file system may or may not recycle a freed inode number. When it does, the substituted namespace is indistinguishable and the store accepts it, so the accepted write **would be redirected**.

This is therefore a **real, intermittent failure of F2 on Linux**, not a bad test, and it must not be presented as green. The correct repairs, in ascending cost, are:

1. Create staging objects *relative to the retained namespace descriptor* on Linux (`openat` on the held directory handle) so redirection is prevented rather than detected. This needs the approved Linux helper reshaped to take a directory handle instead of a path; the Windows path stays as it is, since `FILE_ID_INFO` is not recycled.
2. Failing that, an authoritative birth-time discriminator, which is not exposed by the stable standard library on either host.

F1, F3, F4 (except real ENOSPC), F5, F6 and F7 remain proven by run 155. **F2 is `UNVERIFIED` on Linux and the Windows reparse sub-case is `UNVERIFIED`; F8 is `FAIL` at the report-only HEAD.** R02.B stays OPEN and no administrative closure is performed.
