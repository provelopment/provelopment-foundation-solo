# Execution Report

**Order:** `SOLO-WO-047 R02` — audited native `SourceRead` boundary (Owner-issued, unchanged by this run).
**Run kind:** implementation revision under Chief Architect disposition `REVISE`, plus the standing report permission.
**Implementation HEAD:** `7e158a28b60a8b8e5baeb8ce445e61d4f9cbbc1e`
**Implementation tree:** `426a9835c66614cd5fd6b68b0686fe751fab44cf`
**Workflow:** `rust-verify` run `42592450903` (run 96) — **success**
&nbsp;&nbsp;· Ubuntu job `129077070913` **success** (09:18:56 → 09:20:02)
&nbsp;&nbsp;· Windows job `129077071199` **success** (09:18:57 → 09:20:34)
**Branch/remote:** `main` → `origin` `provelopment/provelopment-foundation-solo`.

---

## 1. Why this run happened

The previous run STOPped at the §5 hosted native-proof gate: `cargo test --workspace --all-targets --locked`
was red on both hosts and the job logs were unreadable from the implementation environment
(`GET .../actions/jobs/{id}/logs` → **403**; `.../check-runs` → **404**). The Chief Architect read the
raw logs and returned `REVISE` with three required corrections plus a full re-run.

## 2. What was corrected

1. **Ubuntu — the workspace checker no longer refuses the approved Windows exception.**
   `Category::UnsafeCode` was raised on any detected unsafe expression. The decision is now a single
   shared function, `unsafe_scope_verdict(location, unscoped, windows_scoped)`: it permits unsafe only
   when the file is exactly `crates/solo-adapters/src/source_read_native.rs` **and** the occurrence sits
   inside a positive `#[cfg(windows)]` scope. Every other file, and any unscoped, `not(windows)`-gated or
   other-OS-gated occurrence even in that module, is still refused, as are FFI blocks, foreign imports,
   verbatim syntax and the whole existing graph/import rule set.
   *Defect found and fixed in this run:* `scan_module` descends into modules **without** visiting the
   module as an item, so the first implementation never saw the gate and the module still failed. The
   gate is now carried in `FileCtx::windows_gated` and seeded into every item's `Facts`.
   Negative proofs live in `check_architecture_fixtures.rs` (the target that already includes the
   checker): allowed only in the approved module inside a positive Windows gate; refused outside it,
   refused for `#[cfg(not(windows))]`, refused for `#[cfg(target_os = "linux")]`, refused for an ungated
   `unsafe fn`.
2. **Windows — the junction fixture is created correctly and proved before use.**
   `cmd /C mklink /J "<link>" "<target>"` was mangled by Rust's argument quoting. The fixture now runs
   PowerShell `New-Item -ItemType Junction` with both paths passed as **environment variables**, so no
   path can restructure the command, and the helper asserts `FILE_ATTRIBUTE_REPARSE_POINT` on the result
   **before** any refusal is claimed from it. Both refusal tests now execute and pass on Windows.
3. **Trust input is explicit.** `ContentRootCapability::adopt(path, provenance)` requires a
   `RootProvenanceV1` argument that names *why* the caller asserts this directory is the content root,
   records it on the capability (`provenance()`), and opens the directory once. Path text alone can no
   longer become a root capability.
4. **A read must agree with its probe.** `CandidateProbeV1::Located` and `CandidateBodyV1::Present` now
   carry the change metadata, and `read_and_choose` refuses `ChangedDuringRead` when the body's physical
   identity, transferred length or recorded change evidence disagrees with what the probe announced —
   not merely with the handle's own before/after samples. A shared object (link count ≠ 1) is decided as
   `IdentityCollision` **before** that comparison, because sharing is a collision, not a change.


5. **Physical identity is reserved across representations.**
   `check_physical_reservations_v1(port, declarations)` probes every declared candidate of every submitted
   representation and refuses when two distinct candidates resolve to one physical object, so one authored
   body cannot answer for two published representations.

## 3. Evidence

### 3.1 Local gates (implementation environment, pinned `1.99.1`)

`cargo fmt --all -- --check`, `cargo check --workspace --all-targets --locked` and
`cargo clippy --workspace --all-targets --locked -- -D warnings` are green for **both**
`x86_64-pc-windows-gnu` and `x86_64-unknown-linux-gnu`. Local **test execution remains impossible** here:
no C toolchain and no linker for any target (`link.exe` absent for MSVC, `dlltool` has no `as.exe`, no
WSL/Docker), so every host-executed result below comes only from the mandatory workflow.

### 3.2 How the failures were identified without logs

Job logs stay `403`. The usable channels are the run/job API and **check-run annotations**, and the test
step now emits one annotation per failing test plus its panic line. That is how each cause was *named*
rather than guessed:

| Run | HEAD | What the evidence named |
| --- | --- | --- |
| 89 | `31699ed` | step 16 red on both hosts; no detail channel yet |
| 90 | `d9bdf46` | step 16 red on both hosts |
| 91 | `1213407` | step 16 red on both hosts |
| 92 | `5dea573` | five tests, listed in the run's annotations |
| 93 | `23d8bdc` | `no_first_party_file_outside_the_native_module_uses_unsafe`, `the_real_graph_carries_no_rename_…` |
| 94 | `db40d26` | step 16 **green**; `approved edge count changed` — measured `edges=40` |
| 95 | `3cb52e2` | Ubuntu `edges=40`; Windows `a_same_size_rewrite_between_probe_and_read_is_refused` |
| **96** | **`7e158a2`** | **none — success on both hosts** |

Substantive findings from that sequence, each fixed within scope:

* the unsafe-scope exception needed the module-descent fix described in §2.1;
* an `allow(unsafe_code)` request outside the approved module must be refused — but only the **ungated**
  request, because `#![deny(unsafe_code)]` and the crate root's single
  `#[cfg_attr(windows, allow(unsafe_code))]` are legal;
* `the_real_graph_carries_no_rename_…` had been failing since `rustix` was adopted: the locked graph
  contains one third-party internal rename (`rustix` depends on `errno` as `libc_errno`). The assertion now
  refuses a rename that could hide a **first-party** edge, which is the property it exists to protect;
* the recorded `edges=34` had never been measured, because step 17 was unreachable while step 16 was red.
  The real measured value is **`edges=40`**, now recorded in both host assertions, which additionally
  report the measured value on any future mismatch.

### 3.3 Full mandatory sequence at `7e158a2`

Green on both hosts at every step: locked fetch, the `cargo-deny 0.20.2` gate with a fresh RustSec refresh,
the advisory negative probe, the workspace package-set gates, `fmt`, strict `clippy`, the test step, the
real-tree architecture checker (`architecture=legal packages=4 … edges=40`), the refusal self-test
(`self-check=passed cases=29`), both test-failure tripwires, the byte-reproducible release build, the
release CLI smoke and the forbidden-import refusal with restore.

## 4. Host-executed security coverage

Exercised on both hosts at `7e158a2` (green):

* declared JSON and Markdown leaves read with their bytes and physical identity;
* oversized JSON refused rather than truncated; JSON preferred over Markdown; confirmed absence only;
* leaf that is a symlink refused; **intermediate directory that is a junction refused**; root that is a
  junction refused — with the fixture proved to be a reparse point before the refusal is claimed;
* directory in place of a leaf refused; a missing intermediate is a confirmed absence, not a failure;
* a second directory entry (hardlink) refused as a collision;
* two declared members resolving to one object refused as a collision;
* **leaf replaced by another object between probe and read refused as `SNAPSHOT_CHANGED`**, driven through
  the real reader by a port that performs the replacement between the two calls the use case makes;
* a rewrite between two separate reads is observed, never served from memory;
* adoption requires an explicit provenance assertion; an absent root is refused.

Application level (platform-independent and deterministic, both hosts): probe-to-read replacement by
change metadata, by identity and by length are each refused; a hardlinked body is a collision; two
representations resolving to one object are refused, and distinct ones reserve distinct identities.

## 5. Limitations, stated rather than claimed

* **A same-size in-place rewrite is not claimed as a host case.** It is observable only through the
  recorded change timestamps, and on Windows those advance on a system timer tick, so a rewrite
  microseconds after the probe can be genuinely indistinguishable from it — observed on run 95, where the
  Windows host returned the body. The rule itself is proved deterministically at application level, and the
  always-observable replacement case is proved on both real filesystems.
* **An unsupported kernel/filesystem refusal is not host-forced.** Nothing available here can make
  `openat2`/`NtCreateFile` return `ENOSYS` or an unsupported status without an interposer; the mapping is
  fail-closed by construction (`Unsupported`/`Unsafe` never degrade to absence) and is **unverified**.
* Job logs remain `403` from this environment; every conclusion above comes from run/job status,
  check-run annotations and step-level conclusions of the API.
* This report claims no filesystem-wide snapshot and no publication authority: the accepted constants
  `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` remain `false`.

## 6. State preserved

`ACTIVE` was not edited in this run (the order stayed issued and unchanged, blob `4cb50914`); `NEXT` stayed
neutral (`92fa562b`); `ROADMAP` untouched (`18975332`); previously accepted modules byte-identical. The
earlier R01 STOP and this order's previous R02 STOP remain in Git history. R02.B, R02.C and Stage 02 stay
open. No dependency, manifest, `deny.toml`, `Cargo.lock` or toolchain change was made; the `edges=…`
assertions, the test-step reporting and the unsafe-refusal proofs are the only workflow/test-scaffold edits.

**Cleanup retained:** `.temp/toolchain` (sanctioned, checksum-verified `1.99.1`) and `.temp/target` (cache)
are kept with the trigger *delete when this slice is accepted or abandoned*; four older `.temp/` folders and
`.temp/architecture-fixtures` are Owner/earlier-run material and were not touched. The untracked 781.6 MB
`target/` from the previous run stayed removed, and the working tree is clean.

