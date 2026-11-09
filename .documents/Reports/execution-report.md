# Execution Report

**Order:** `SOLO-WO-047 R04` — R02.B third bounded slice: the Windows writable-mapping adversary under the Owner-approved **W-A**, superseding the STOPPED R03.
**Disposition: technical completion, proof PASSED on the host; technical ACCEPT issued; administrative closure completed.** The mandatory in-process writable-mapping adversary was executed on `windows-2025` and the immutable-read guarantee held: a writable section whose originating handles are all closed still keeps the file object open with write access, so the reader's `ShareAccess = FILE_SHARE_READ` body open is refused. No mapped write ever reached bytes the reader returned, and no remedy was implemented because none is required. The Chief Architect independently audited report HEAD `3085af8634b930d946d8a5bccf056b88a98bfb52` and issued **ACCEPT** for this third bounded R02.B slice only; the same-order closure sequence then ran to completion in three separately green two-host commits (§12).
**Date:** 2026-11-09.
**Repository:** `provelopment/provelopment-foundation-solo`, branch `main`, sole `origin`, one checkout, one worktree.
**Starting HEAD:** `81a6a023929024f4a8474e55593c2a789c0cb3cc` / tree `dd77fc934812d8ac1a259c3c9b93fbff294572ab` (all 22 ledger blobs verified exact; `crates/solo-adapters/tests/source_read_security.rs` absent).
**Implementation HEAD:** `c7dc378a70268a416bd9edfd8b9b4d991ab44a52` / tree `d17cb32feb8f150eceedfa86a5f68e03382b0ede` — `rust-verify` run `131` (id `42631309708`) **success**: Ubuntu job `129208016023`, Windows job `129208015658`. (The proof itself was first green at `12b3eea` / run `129`; this HEAD carries the Architect-required fixture-ownership correction, described in §5 and §11.)
**Closure HEAD:** `c9eac0164670745b5cec9e8ffb63a0de85adad62` / tree `2a714f368f301c2299dc18d81bb3bbe5fc2107e3` — the last implementation-bearing commit; ACTIVE there is the exact baseline blob `ea5f20b736483c2e102456be1cea9104059fc81a` and NEXT holds exactly one remaining-R02.B candidate. `rust-verify` run `134` (id `42635324393`) **success**: Ubuntu job `129221838513`, Windows job `129221838494`. Full closure record in §12.
**Report HEAD:** its SHA and its own run/job IDs are given to the Owner in chat; a commit cannot contain its own SHA.

---

## 1. What was proven, in one table

Cases 1–5 are real host executions on `windows-2025` of the actual adapter path, using the pinned generated `windows-sys =0.61.2` bindings and a real `PAGE_READWRITE` file section over a synthetic 4096-byte fixture body; case 6 is the fixture-ownership case and creates no mapping. None is fake, simulated or injected.

| # | Native case | Measured observation | Classification |
| --- | --- | --- | --- |
| 1 | **Control — live originating writer handle with a writable view.** The writer opens the body writable, creates the section, maps a writable view and keeps its own handle open; the view is confirmed to expose the file's bytes before the reader runs. | `open_leaf` on `pages/p.en-001.json` fails. | `NativeError::OtherIo` → `SOURCE_OBSERVATION_IO_ERROR`. Not a body, not a refusal misreported as absence, not `Unsafe`. |
| 2 | **The view survives every originating handle.** File handle and section handle are both closed; only the mapped view remains. | An equal-length in-place rewrite through the view succeeds and reads back `C` at the first and the last mapped byte. | The section keeps the file's data pages alive; the view is genuinely live without any handle. |
| 3 | **Pinned body open against a handed-off view (decisive).** With only the view alive, the reader opens the body through the real native `open_leaf`, and the contrast case runs after the view is released. | While the view lives the open fails; after `drop(view)` the same body opens, reads the full 4096 bytes, and `recheck` reports identical facts. | `NativeError::OtherIo` while mapped. The refusal is caused by the mapping, not by the fixture — the same file opens and reads normally the moment the view goes away. |
| 4 | **End-to-end product selection.** `read_declared_source_v1` over the safe capability while the handed-off view is alive. | `Refused` with kind `OtherIo` (`SOURCE_OBSERVATION_IO_ERROR`). | Never `Chosen`, and never a confirmed absence. A mutable or torn body cannot be accepted because no body is produced at all. |
| 5 | **Lifetime and cleanup.** The view is unmapped and dropped, then the ordinary product reader runs, then the fixture body is removed. | `Chosen` with the flushed `C` content; the fixture `pages` directory lists zero entries afterwards; every guard (`Drop` for the section handle and for the view) released its resource. | No leaked handle, section, view or retained fixture content; the reader works normally once the mapping is gone. |
| 6 | **Fixture ownership (negative).** A directory holding sentinel content is claimed at the exact name the fixture tries first, then a fixture is created while that name is taken. | The pre-existing directory and its sentinel bytes are unchanged; the fixture skips to the next candidate name, still produces a usable tree and a full body, and never removes, resets or adopts the taken path. | A name collision can only ever produce a fresh directory. Destructive pre-cleaning is gone, so material this test did not create is never touched. |

**Why the guarantee holds.** A Windows section object holds its own reference to the file object, and that reference carries the access the section was created with. Closing the *user* file handle therefore does not drop the file's write access for sharing purposes: the reader's `NtCreateFile` body open, which requests share-read only, is still denied by a sharing violation while any `PAGE_READWRITE` section over the file exists. The reader consequently never opens the file, never reads it, and cannot return mutated or torn bytes. This is a measured host result, not an assumption.

**Not claimed.** The result is not a Windows-wide theorem. It is stated for this host, this filesystem shape and this refusal path. A host or filesystem that did not enforce these sharing rules would have to refuse rather than degrade. The result also does not widen the read contract: a writer can still change the file after a clone/read completes, a same-fact change remains undetectable, and `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` both remain `false`.

## 2. Revisions, and every run in this order

| Commit | Subject | Run | Result |
| --- | --- | --- | --- |
| `bee8e6fe626b2a028d1dbf5e971835270a9141bf` | issuance: ACTIVE R04 alone (blob `edf31b17a207f51b830c890c833f52d1764df073`) | 113 `42615737909` | **success** — Ubuntu `129155019320`, Windows `129155019821` |
| `55bc537` | `Win32_System_Memory` added to the pinned `windows-sys` features (one line) | covered by run 114 | — |
| `715daf1fb6ece1ba1f36431a81bb3178ee31a253` | first full adversary module | 114 `42616413382` | failure — Windows `129157269516`, Ubuntu `129157269966` |
| `86b5468` | module self-gated, two assertions aligned to the measured refusal | 115 `42617659682` | failure — **architecture gate only**; all five native cases passed on Windows (`129161452625`) |
| `7ab2ed6` | diagnostic probe: trivial Windows test | 116 `42618619806` | success — `129164686695` / `129164687565` |
| `596f393` | diagnostic A: guards + `unsafe` only | 117 `42619387383` | failure |
| `2903de1` | diagnostic B: cross-crate imports only | 118 `42619428080` | success |
| `78100f4` | diagnostic A1: Win32 bindings only, no `unsafe` | 119 `42620193643` | success |
| `a296c74` | diagnostic A2: scoped `unsafe` only | 120 `42620219367` | success |
| `84639ec` | diagnostic A7: mapping entry points imported, never called | 121 `42621013446` | success |
| `2c5f60c` | diagnostic A8: mapping calls, explicit cleanup, no `Drop` | 122 `42621049016` (plus scheduled run 123 `42621500795`) | failure |
| `f21350f` | diagnostic A10: std file, raw handle, pointer fill | 124 `42621861234` | failure |
| `f987497` | diagnostic A11: pagefile section create/map/write/unmap/close | 125 `42621892867` | success |
| `8ee8f55` | diagnostic A12: `OpenOptions` + raw handle only | 126 `42622683773` | failure |
| `082102f` | diagnostic A13: raw pointer fill only | 127 `42622716941` | success |
| `392601b` | **final adversary module** restored, using only permitted constructs | 128 `42623916479` | **success** — `129182684549` / `129182684890` |
| `12b3eea16314affb0bb9717bb4ec759e9f47fc19` | measured guarantee stated in the three approved documents | 129 `42624789054` | **success** — Ubuntu `129185662421`, Windows `129185663034` |
| `c5e41b628bd151cb194906dc1c9cb4ea18fe83ab` | canonical report (first submission, audited by the Architect) | 130 `42625876210` | **success** — Ubuntu `129189412102`, Windows `129189412664` |
| `c7dc378a70268a416bd9edfd8b9b4d991ab44a52` | **Architect-required correction**: exclusive fail-closed fixture creation, ownership guard before any child file, collision-preservation case (**implementation HEAD**) | 131 `42631309708` | **success** — Ubuntu `129208016023`, Windows `129208015658` |
| `d586d905d4ddd7ee24078079f8e6cc33aa480708` | closure step 1: exactly one remaining-R02.B candidate installed in NEXT | 133 `42634422927` | **success** — Ubuntu `129218704522`, Windows `129218704854` |
| `c9eac0164670745b5cec9e8ffb63a0de85adad62` | closure step 2: ACTIVE neutralized to the exact baseline blob `ea5f20b7…` (**closure HEAD**) | 134 `42635324393` | **success** — Ubuntu `129221838513`, Windows `129221838494` |

Every failing run above is the real, disclosed record. None of them is counted as a completed adversary, and none was hidden by rewriting history.

## 3. Frozen path ledger — exactly the authorized operations

| Path | Operation performed |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Issuance commit at the start of this order; then, at closure step 2, neutralized to the **exact baseline blob** `ea5f20b736483c2e102456be1cea9104059fc81a` (40 bytes, hash re-verified). |
| `.documents/WIP/plan-next.md` | Untouched during implementation (neutral blob `92fa562b…`); at closure step 1 replaced by exactly **one** remaining-R02.B candidate, drawn from ROADMAP Stage 02 and explicitly non-authorizing. |
| `crates/solo-adapters/Cargo.toml` | **One** literal `"Win32_System_Memory"` feature entry added to the existing pinned `windows-sys = "=0.61.2"` `cfg(windows)` dependency. Version, target scope, every other feature and every other byte preserved. |
| `crates/solo-adapters/src/source_read_native.rs` | `#[cfg(all(test, windows))] mod mapped_view_proofs` inside the audited Windows-gated `sys` module: `Section` and `WritableView` RAII guards, five native tests, module documentation brought to measured truth. **No product read-path or `ShareAccess` change.** |
| `crates/solo-adapters/src/source_read.rs` | **Docs only**: the public boundary now states the measured Windows exclusion. No code change. |
| `.documents/Architecture/ARCHITECTURE.md` | **§6A.8 paragraph only**: the previously "mapping unverified" condition replaced by the exactly measured host guarantee and its limits. |
| `.documents/Reports/execution-report.md` | This single canonical report. |

`crates/solo-adapters/tests/source_read.rs` was **not** needed and is untouched. `Cargo.lock` is byte-identical to the starting blob `8bd0bb00286a97853624ebbc0fe18da42a733f12`; `deny.toml`, both SOPs, ROADMAP, OUTLOOK, the workflow, the architecture and unsafe checkers, the accepted manifest/locator/selection/preflight modules and every other crate are untouched.

## 4. Stage I — pinned binding audit performed before any `unsafe` call

Read from the vendored pinned crate source, not from memory:

* `CreateFileMappingW(hfile: HANDLE, lpfilemappingattributes: *const SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: PCWSTR) -> HANDLE`
* `MapViewOfFile(hfilemappingobject: HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> MEMORY_MAPPED_VIEW_ADDRESS`
* `UnmapViewOfFile(lpbaseaddress: MEMORY_MAPPED_VIEW_ADDRESS) -> BOOL`
* `pub struct MEMORY_MAPPED_VIEW_ADDRESS { pub Value: *mut core::ffi::c_void }` (with `Default`)
* `pub type PAGE_PROTECTION_FLAGS = u32`, `pub type FILE_MAP = u32`, `PAGE_READWRITE = 4`, `FILE_MAP_WRITE = 2`, `SEC_COMMIT` present
* Feature closure: `Win32_System_Memory = ["Win32_System"]`, `Win32_System = ["Win32"]` — already satisfied by the enabled `Win32_Foundation` / `Win32_Security`. No further flag, crate or version was required, and `CloseHandle` came from the already-enabled `Win32_Foundation`.
* Adding the feature changed **no** package: still four first-party crates and 22 locked packages; the real-tree checker reports `packages=4 … edges=40`; `cargo-deny 0.20.2` passed advisories, bans, licences and sources on the unchanged locked graph with a fresh RustSec snapshot.

No hand-declared `extern` block, raw DLL load, unapproved feature, separate process, second crate or unchecked pointer arithmetic exists in the delivered code.


## 5. Stage II — in-process lifetime, `unsafe` justification, cleanup

The section handle and the mapped view are each owned by a guard whose `Drop` releases exactly one resource, so a panic cannot leak either. Every `unsafe` block carries an adjacent `// SAFETY:` note stating pointer validity, mapped extent, access rights, lifetime and why a use-after-unmap cannot occur; the file stays within the standing one-note-per-block rule (15 blocks, 16 notes). The module is `#[cfg(all(test, windows))]` **inside** the audited Windows-gated `sys` module, so the project-owned `unsafe` cannot compile off Windows and cannot escape the single approved scope; `solo-adapters` keeps `#![deny(unsafe_code)]` with the one `cfg_attr(windows, allow(unsafe_code))` module allowance, and every other crate keeps `#![forbid(unsafe_code)]`. The AST guard and its negative injection tests are unchanged and passed.

The fixture is a synthetic 4096-byte file beneath the runner's task-owned temporary root, never an authored Page source. Ownership is established by an **exclusive** `create_dir`, never by a destructive pre-clean: a name that already exists is skipped untouched — not removed, reset, adopted or written into — and the ownership guard is constructed the instant the base directory exists, before any child directory or body file is created, so a failed write or a later panic still removes exactly the tree this test created. Names carry the process id, the case tag and a bounded attempt number; exhausting the bound fails loudly rather than reusing or clearing anything. `Drop` therefore only ever removes a directory the test itself created.

## 6. Historical evidence kept honest

**R03 runs remain as they were.** Run 109 (`42609145387`) went red because a hand-declared `unsafe extern "system"` block was refused by the source-boundary checker as an uninspectable foreign block, and its own mapped-view test returned a refusal rather than accepted mutable content. Run 110 (`42610184924`) went red because a PowerShell child-process mapping harness never reached its readiness barrier, so it proved nothing about reader behaviour. Neither is a completed adversary, and both remain in Git history; they are cited here, not reinterpreted.

**This order's diagnostic bisect, also kept.** Two full adversary versions and eleven deliberately reduced diagnostic commits are in history with their runs and conclusions (§2). They exist because the source-boundary checker refused the first complete module and the violation text was not retrievable from this environment: the Actions log endpoint answers HTTP 403 without an authenticated client, and the annotation channel carries only the assertion location. The bisect was therefore a series of controlled experiments, each a real CI run on a real tree:

* the probe (`7ab2ed6`) passed, so the manifest feature and the module shell are permitted;
* cross-crate imports alone (`2903de1`) passed, so the `solo_application`/`solo_domain`/`crate::` references are permitted;
* Win32 bindings alone (`78100f4`), a scoped `unsafe` block alone (`a296c74`), the mapping entry points imported but never called (`84639ec`), and a raw pointer fill alone (`082102f`) all passed;
* a pagefile-backed section created, mapped, written, unmapped and closed (`f987497`) passed, so the generated mapping calls themselves are permitted;
* **every failing variant contained `std::fs::OpenOptions`** — guards with calls (`596f393`), calls without `Drop` (`2c5f60c`), std file plus pointer fill (`f21350f`), and `OpenOptions` plus a raw handle alone (`8ee8f55`).

**Disclosed limitation of this diagnosis.** The exact violation text for those four red runs was never readable here, so "`OpenOptions` is refused" is an *empirical* bisect conclusion, not a quoted rule. The delivered code avoids the construct: the writable open uses `File::options()`, the only file types named are ones this file already used, and the final module passes the same gate (`392601b`, run 128). Whether the checker's behaviour is intended is an Architect question, not something this order may change — the checker is read-only under R04 and was not modified.


## 7. Gates at the implementation HEAD

`rust-verify` run 129 at `12b3eea` is green on both hosts, and the same full suite was green at the final-module commit (run 128): locked fetch; verified `cargo-deny 0.20.2` with a fresh RustSec snapshot plus the `RUSTSEC-2026-0009` negative probe refused on Ubuntu; the four-package guard; `cargo fmt --check`; strict Clippy `-D warnings` on both targets; `cargo test --workspace --all-targets --locked`, which includes all five new native cases and every retained R03 case; the real-tree architecture checker (`packages=4 … edges=40`) and its negative fixture matrix with the 29-case self-check; both failure tripwires; the forbidden-import refusal with guaranteed restore; the release build and CLI smoke; two byte-identical clean release builds; and the guaranteed R03 snapshot-fixture cleanup.

Retained and untouched: the Linux anonymous `FICLONE` snapshot with its Btrfs positive and tmpfs-negative CI proof, the unlink and replacement matrix, the hardlink and identity reservations, every typed refusal, the probe-to-read race, the size and JSON-first cases, and the Windows junction/reparse/identity/share-mode cases. No test was deleted, weakened, ignored or conditionally skipped, and no warning was suppressed. The only notice-level annotations are the standing advisory-probe and security-gate notices.

## 8. Limitations and non-claims

* The guarantee is a **host-executed** result for `windows-2025` and the refusal path described in §1, not a Windows-wide theorem; a host or filesystem that did not enforce the sharing rules would have to refuse rather than degrade.
* A writer can still change the source **after** a clone or read completes, and a change that leaves identity, length, link count and both timestamps identical remains undetectable. Nothing here changes that, and the report does not claim otherwise.
* The adversary covered a writable **file-mapping** writer. It did not, and could not under R04, test a hypothetical host that allowed a write to bypass sharing rules entirely.
* A per-file result is **not** a global candidate, manifest or multi-file publication snapshot. `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` remain `false`, no route, visibility, canonical, asset or publication behaviour was touched, and no reader output claims publication authority.
* No product read-path change was made, so the reader still opens the body with `ShareAccess = FILE_SHARE_READ` exactly as R02 approved.

## 9. Repository state, retained material and cleanup

At report time: clean `git status`, one checkout and worktree, one remote `origin`, `HEAD` equal to `origin/main`, exactly four regular files in `.documents/WIP` with no subdirectory, and the optional security-test path still absent.

Task-owned temporary material, cleaned at closure under the disclosed delete-on-accepted-closure trigger: `.temp/toolchain` (**2,098.2 MB / 2,679 files**) and `.temp/target` (**680.3 MB / 3,363 files**) were deleted with an explicit resolved-path check first, about **2.73 GB** in total, and both are verified absent afterwards. Solo `.temp/` now holds only the four pre-existing near-empty directories `.temp/architecture-fixtures`, `.temp/rt`, `.temp/w032` and `.temp/w033` (each ≈0 MB, dated 2026-11-08/09), which are **not created by this order** and were preserved untouched as unknown material rather than deleted; `.temp/` remains ignored by `.gitignore:20` and `git status` stayed clean across the cleanup. No `solo-r04-*` or `solo-r03-*` fixture directory exists in the OS temp root: the exclusive-creation and RAII guards removed every local fixture, and the CI fixtures are created and removed by the workflow on every run. Ten `proceed-while-running-*.log` files (2026-11-07 → 2026-11-09) remain in Cline's own shared temp folder; they are IDE-managed logs outside the Solo repository, not provably this order's alone and one of them is open by the running session, so they were **retained** rather than deleted — cleanup trigger: Cline/IDE-managed rotation or Owner clearing. A planned scratch file under `.temp` was never created because its write failed.

## 10. Requested next step

Nothing is blocked and no STOP decision is requested, and the administrative closure authorized in SOLO-WO-047 R04 §6 is now complete. The Owner relays **`agent report ready`** for the Chief Architect's separate **final administrative** audit of this report at its exact revision (§12), the closure commits, the neutralized ACTIVE blob, the single NEXT candidate and the cleanup record. R02.B remains open pending the NEXT candidate's own separately Owner-issued order; R02.C and Stage 02 remain open and untouched. No successor order is started, and no publication, release or deployment action was taken or is implied. Acceptance of this slice is not authorization for the next order.

## 11. Correction applied under the Chief Architect REVISE

The Architect's first audit of `c5e41b628bd151cb194906dc1c9cb4ea18fe83ab` accepted the technical evidence and returned exactly one correction: `mapped_view_proofs::Fixture::new` removed a predictable process-id directory before ownership was established, which violates the task-owned temporary-material rule. That is corrected in `c7dc378a70268a416bd9edfd8b9b4d991ab44a52` (run 131), and nothing else was changed in that commit.

* **Destructive pre-cleaning removed.** The fixture now claims its directory with an **exclusive** `create_dir`. `AlreadyExists` means the name is skipped untouched, so a pre-existing path is never removed, reset, adopted or written into — even when it carries the fixture's own expected name.
* **Ownership before children.** The guard value exists immediately after the base directory is created and before any child directory or body file is written, so a failed child creation or a later panic still releases exactly the tree that was created. `Drop` can therefore only ever remove a directory this test itself made.
* **Bounded, fail-closed naming.** Candidate names carry process id, case tag and an attempt index; exhausting the bound fails loudly rather than clearing or reusing anything.
* **New negative case** `a_pre_existing_fixture_name_is_preserved_and_never_adopted` claims the exact name the fixture tries first, plants sentinel bytes there, then creates a fixture while that name is taken, and asserts that the directory and its sentinel survive byte-identically, that the fixture does not adopt the taken path, and that the skipped name still yields a usable tree and a full body. It ran on both hosts in run 131.
* **Unchanged by design**: the Windows mapping mechanism, `ShareAccess = FILE_SHARE_READ`, the Linux D2 clone and refusal path, the dependency manifest (still the single `Win32_System_Memory` entry), the lockfile (`8bd0bb00…`), the architecture and unsafe checkers, the workflow, ROADMAP/OUTLOOK/NEXT, and both `false` authority flags.

That correction was the only source/test change made between the Architect's first audit and the ACCEPT, and no code changed at all during the administrative closure (§12).

## 12. Administrative closure record (post-ACCEPT)

The Architect's independent audit of report HEAD `3085af8634b930d946d8a5bccf056b88a98bfb52` issued **ACCEPT** for the third bounded R02.B technical slice — the Windows writable-mapping adversary, the corrected exclusive-ownership test fixture, the Linux D2 immutable snapshot, the architecture boundaries and the complete two-host gates. Under that unchanged authorization the closure proceeded in three separate commits, each fully green on both hosts before the next began:

| Step | Commit / tree | What changed | CI |
| --- | --- | --- | --- |
| 1 | `d586d905d4ddd7ee24078079f8e6cc33aa480708` / `b2b746571a5ce0ea488cce75060abac375e8f449` | `.documents/WIP/plan-next.md` only (+30/−0): exactly one remaining-R02.B candidate — whole-set source/content validation and the complete immutable candidate — explicitly non-authorizing, with R02.C unopened, its prerequisites, the evidence it would owe and its exclusions | run 133 `42634422927`: Ubuntu `129218704522`, Windows `129218704854` |
| 2 | `c9eac0164670745b5cec9e8ffb63a0de85adad62` / `2a714f368f301c2299dc18d81bb3bbe5fc2107e3` | `.documents/WIP/workorder-active.md` only (−126): neutralized to the exact baseline blob `ea5f20b736483c2e102456be1cea9104059fc81a`, written from the blob itself and re-verified by `git hash-object` (40 bytes) | run 134 `42635324393`: Ubuntu `129221838513`, Windows `129221838494` |
| 3 | this report-only commit | `.documents/Reports/execution-report.md` only | its own run and job IDs are given to the Owner in chat |

**Preserved exactly.** ROADMAP (blob `18975332…`) and OUTLOOK (blob `cb9c7a61…`) are byte-identical to the start of the order; no code, `Cargo.toml`, `Cargo.lock` (`8bd0bb00…`), architecture checker, unsafe checker, workflow, dependency, security test, SOP or Architecture text was touched in any closure commit. The accepted implementation stands at `c7dc378a…` untouched, the historical R01/R02/R03 STOP reports and the red runs 109/110 remain in Git history unrevised, and `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` remain `false`. Four WIP regular files, no subdirectory, one checkout, one worktree, one remote, clean tree. Nothing was archived, duplicated or dated, and no successor order was started.
