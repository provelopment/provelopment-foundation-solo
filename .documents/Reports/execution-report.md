# Execution Report

**Order:** `SOLO-WO-047 R04` — R02.B third bounded slice: the Windows writable-mapping adversary under the Owner-approved **W-A**, superseding the STOPPED R03.
**Disposition: technical completion, proof PASSED on the host.** The mandatory in-process writable-mapping adversary was executed on `windows-2025` and the immutable-read guarantee held: a writable section whose originating handles are all closed still keeps the file object open with write access, so the reader's `ShareAccess = FILE_SHARE_READ` body open is refused. No mapped write ever reached bytes the reader returned, and no remedy was implemented because none is required.
**Date:** 2026-11-09.
**Repository:** `provelopment/provelopment-foundation-solo`, branch `main`, sole `origin`, one checkout, one worktree.
**Starting HEAD:** `81a6a023929024f4a8474e55593c2a789c0cb3cc` / tree `dd77fc934812d8ac1a259c3c9b93fbff294572ab` (all 22 ledger blobs verified exact; `crates/solo-adapters/tests/source_read_security.rs` absent).
**Implementation HEAD:** `12b3eea16314affb0bb9717bb4ec759e9f47fc19` / tree `fb907490aaebd9f4805a848b3e3808ff7db02f60` — `rust-verify` run `129` (id `42624789054`) **success**: Ubuntu job `129185662421`, Windows job `129185663034`.
**Report HEAD:** its SHA and its own run/job IDs are given to the Owner in chat; a commit cannot contain its own SHA.

---

## 1. What was proven, in one table

Every case below is a real host execution on `windows-2025` of the actual adapter path, using the pinned generated `windows-sys =0.61.2` bindings and a real `PAGE_READWRITE` file section over a synthetic 4096-byte fixture body. None is fake, simulated or injected.

| # | Native case | Measured observation | Classification |
| --- | --- | --- | --- |
| 1 | **Control — live originating writer handle with a writable view.** The writer opens the body writable, creates the section, maps a writable view and keeps its own handle open; the view is confirmed to expose the file's bytes before the reader runs. | `open_leaf` on `pages/p.en-001.json` fails. | `NativeError::OtherIo` → `SOURCE_OBSERVATION_IO_ERROR`. Not a body, not a refusal misreported as absence, not `Unsafe`. |
| 2 | **The view survives every originating handle.** File handle and section handle are both closed; only the mapped view remains. | An equal-length in-place rewrite through the view succeeds and reads back `C` at the first and the last mapped byte. | The section keeps the file's data pages alive; the view is genuinely live without any handle. |
| 3 | **Pinned body open against a handed-off view (decisive).** With only the view alive, the reader opens the body through the real native `open_leaf`, and the contrast case runs after the view is released. | While the view lives the open fails; after `drop(view)` the same body opens, reads the full 4096 bytes, and `recheck` reports identical facts. | `NativeError::OtherIo` while mapped. The refusal is caused by the mapping, not by the fixture — the same file opens and reads normally the moment the view goes away. |
| 4 | **End-to-end product selection.** `read_declared_source_v1` over the safe capability while the handed-off view is alive. | `Refused` with kind `OtherIo` (`SOURCE_OBSERVATION_IO_ERROR`). | Never `Chosen`, and never a confirmed absence. A mutable or torn body cannot be accepted because no body is produced at all. |
| 5 | **Lifetime and cleanup.** The view is unmapped and dropped, then the ordinary product reader runs, then the fixture body is removed. | `Chosen` with the flushed `C` content; the fixture `pages` directory lists zero entries afterwards; every guard (`Drop` for the section handle and for the view) released its resource. | No leaked handle, section, view or retained fixture content; the reader works normally once the mapping is gone. |

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
| `12b3eea16314affb0bb9717bb4ec759e9f47fc19` | measured guarantee stated in the three approved documents (**implementation HEAD**) | 129 `42624789054` | **success** — Ubuntu `129185662421`, Windows `129185663034` |

Every failing run above is the real, disclosed record. None of them is counted as a completed adversary, and none was hidden by rewriting history.

## 3. Frozen path ledger — exactly the authorized operations

| Path | Operation performed |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Issuance commit only; R04 stays issued (blob `edf31b17…`). |
| `.documents/WIP/plan-next.md` | Untouched; still the neutral blob `92fa562b…`. |
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

The fixture is a synthetic 4096-byte file in a task-owned temporary directory, written and removed by the test, never an authored Page source, and its `Drop` removes the tree on every path including panics.

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

Task-owned temporary material: `.temp/toolchain` (2.0 GB) and `.temp/target` (680 MB) remain the sanctioned local toolchain and build caches, still under the previously disclosed delete-on-accepted-closure trigger. `.temp/architecture-fixtures`, `.temp/rt`, `.temp/w032` and `.temp/w033` are empty or near-empty directories dated 2026-11-08/09, **not created by this order**; they were left in place rather than deleted as unknown material. A planned scratch file under `.temp` was never created because its write failed. The Linux proof's filesystem fixture is CI-only, created and removed by the workflow on every run.

## 10. Requested next step

Nothing is blocked and no STOP decision is requested. The acceptance path applies: the Owner relays **`agent report ready`**, and the Chief Architect independently audits this report, the exact source diff, the measured mapping outcomes and the SOP A1 gates, then issues `ACCEPT / REVISE / STOP` for **this third bounded R02.B slice only**. ACTIVE stays issued and NEXT stays neutral until that ACCEPT; the administrative closeout sequence (one remaining-R02.B NEXT candidate, then ACTIVE neutralisation to the baseline blob, then the reconciled final closure report, each in its own fully green two-host commit) is authorized only afterwards. R02.B, R02.C and Stage 02 remain open, and no publication, release or deployment action was taken or is implied.

