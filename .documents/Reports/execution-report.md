# Execution Report

**Order:** `SOLO-WO-047 R03` — R02.B third bounded slice, plus the Owner-authorized **corrective evidence pass** that followed the Chief Architect's STOP at report HEAD `962ca5aeab104b9272b1a5e090a6dd050b84fe0a`.
**Disposition of this report: STOP** — the mandatory Windows writable-mapping adversary could not be executed inside the authorized surface, so nothing is claimed about it either way. The Linux half of the corrective pass is delivered and host-executed.
**Repository:** `provelopment/provelopment-foundation-solo`, branch `main`, sole `origin`, one checkout.
**Corrected implementation HEAD:** `7bdf1dcc607b0826c4e9c85eda1976397465afde` / tree `6fe626bb3857b1075f844ce7be141b341ac87413`
**Implementation workflow:** run `42610969651` (run 111) — **success**, Ubuntu job `129138983698`, Windows job `129138982914`.
**Report HEAD:** its SHA and its own run/job IDs are given to the Owner in chat; a commit cannot contain its own SHA.

---

## 1. Why this is a STOP

The corrective order required, first: *execute the mandatory pre-existing writable-memory-mapping
adversary on Windows 2025*. It cannot be executed with the current authorization, and it was not
executed. Three independent, verified blockers, in the order they were discovered:

1. **No approved binding exposes a file section.** The pinned `windows-sys =0.61.2` gates
   `CreateFileMappingW` behind feature `Win32_System_Memory`
   (`src/Windows/Win32/System/Memory/mod.rs`) and `NtMapViewOfSection` behind feature
   `Wdk_System_Memory` (`src/Windows/Wdk/System/Memory/mod.rs`). The adapter enables neither.
   `NtCreateSection` *is* reachable through the enabled `Wdk_Storage_FileSystem`, but a section
   that cannot be mapped and written proves nothing.
2. **Enabling those features is outside the ledger.** R03 §2 forbids any Cargo dependency, feature
   flag or manifest change, so the agent may not add them on its own initiative.
3. **Hand-declaring the two entry points is refused by an existing gate.** A first attempt declared
   `CreateFileMappingW`/`MapViewOfFile`/`UnmapViewOfFile` as `unsafe extern "system"` inside the
   audited native module. Run 109 (`0703bb01c13596d842d2a30193de320341d6f0ab`) went red on **both**
   hosts at `crates/solo-cli/tests/check_architecture_fixtures.rs:26` and `:97`: the real-tree
   boundary check refused the tree under `Category::ForeignBlock`, whose rule reads
   *"extern/FFI blocks are not inspectable and are refused"*
   (`crates/solo-cli/tests/architecture_checks/source.rs:883`). The architecture contract that keeps
   this project auditable forbids the workaround, so the attempt was reverted rather than the gate.

A second attempt used an approved *mechanism* instead of a declaration: a separate `pwsh` process
created a real writable section with `System.IO.MemoryMappedFiles.MemoryMappedFile.CreateFromFile`,
mapped a writable view, and (in the variant of record) disposed its own file handle so only the
section survived, handshaking with the test through files. Run 110
(`2ca2010737a0de9d8895786a1f827fcdd8864f4d`) had Ubuntu green and Windows red: **both** adversary
tests failed at `crates/solo-adapters/tests/source_read.rs:912`, the 60-second handshake guard, so
the child never signalled that the section existed. That is a harness failure, not a reader result;
the panic-detail channel available to this environment shows only the assertion location, so the
child's own error could not be read. The harness was therefore removed in run 111 rather than being
kept as a green test that asserts nothing, and **no behavioural claim is made from it**.

Consequence: whether a writable section that outlives its file handle can alter the bytes
`SourceRead` returns on Windows is **neither proven nor excluded**. It is recorded as an open
adversary, and the repository no longer asserts the stronger claim.

## 2. What the corrective pass did deliver — Linux unlink and replacement, host-executed

Both cases run against the **real anonymous `FICLONE` snapshot** on the disposable Btrfs fixture, and
both are new in this pass. Evidence class: **real native**.

| Requirement | Test (all PASS, run 111) |
| --- | --- |
| Unlinking the source cannot move an existing clone | `snapshot_proofs::an_unlinked_source_does_not_move_an_existing_clone` — clone taken, `remove_file`, then two reads of the same clone return the original bytes |
| A fresh lookup after the unlink is a confirmed absence, not an error and not a body | same test: `open_leaf` → `NativeError::Absent`; the fixture directory is empty |
| The still-open descriptor keeps the inode alive and still clones its bytes | same test: a second `snapshot_leaf` from the held handle returns the original bytes |
| Replacing the source leaves the clone untouched | `snapshot_proofs::a_replaced_source_is_a_new_object_while_the_clone_keeps_the_old_bytes` — unlink then recreate at the same name; held clone still returns the original |
| A replacement is a different physical object, and a fresh clone reads it as itself | same test: `identity` differs, `links == 1`, new clone returns the replacement bytes |
| Product classification: unlink is absence, replacement is a new object | `d2_product::an_unlinked_source_is_a_confirmed_absence_and_a_replacement_is_a_new_object` — `SOURCE_NONE_EFFECTIVE` after the unlink, then the replacement's bytes are chosen; neither is reported as a refusal or a stale body |

Everything accepted earlier is retained and still runs on both hosts: the immutable clone and its
equal-size and length-changing rewrite immunity, the real `EOPNOTSUPP` refusal on tmpfs at both
primitive and product level, the exact-bound and bound+1 cases, the no-named-destination and
descriptor-lifetime checks, the Windows share-mode tests (a live writer handle blocks the read; a
pinned body handle denies a new write and a delete), the whole 30-row R02 §4 matrix, and the
application-side link-count refusal for sole and unselected members.

## 3. What the repository now claims about Windows — narrowed in this pass

Because the adversary is untested, three statements were reduced to exactly what is proven:

* `crates/solo-adapters/src/source_read_native.rs` module docs: Windows denies **handle-holding**
  writers while the read handle is open; a writable section mapped earlier that has outlived its own
  file handle is stated as an adversary the order stopped before executing, with no claim of
  exclusion.
* `crates/solo-adapters/src/source_read.rs` module docs: same narrowing at the public boundary.
* `ARCHITECTURE.md` §6A.8: the Windows sentence now reads that `ShareAccess = FILE_SHARE_READ`
  excludes a **handle-holding** writer or delete, that a section outliving its handle is **not**
  covered, that it was not executable under R03's authorized surface, and that it is recorded as an
  untested adversary rather than claimed as excluded. No other architecture text changed.

Linux is unaffected by this narrowing: the anonymous clone is immutable against every writer,
including a mapped one, because the clone's data is already separate copy-on-write extents.


## 4. Smallest necessary Owner decision

The order forbids inventing a substitute, so the decision is put back to the Owner, in the narrowest
form that unblocks the mandatory proof. It is **one** choice:

* **W-A — authorize one feature flag on the already-pinned crate (preferred).** Add
  `Win32_System_Memory` to the existing `windows-sys =0.61.2` dependency in
  `crates/solo-adapters/Cargo.toml`: no version change, no new crate, no lockfile package change, no
  new `unsafe` outside the already-authorized Windows-gated module. This allows
  `CreateFileMappingW`/`MapViewOfFile`/`UnmapViewOfFile` to be called from the audited native module
  as approved bindings, which is exactly the in-process adversary the order describes.
* **W-B — authorize a bounded test-only scripted adversary** (separate process holding the section),
  with permission to make the harness diagnosable. Weaker provenance: the mapping is created by a
  child process, not by the reader's host module, and the run-110 handshake failure would have to be
  debugged first.
* **W-C — accept the limitation permanently.** Windows exclusion covers handle-holders only; a
  writable section that outlived its file handle may alter the bytes returned, and that is recorded
  as a permanent, documented limitation of the Windows adapter (Linux keeps the strong guarantee).
  Under W-C the current narrowed architecture text is already the final text.

If W-A is chosen and the adversary then **does** alter the bytes `SourceRead` returns, the Owner's
item-4 STOP applies and a further architectural decision is required; no mechanism is proposed here,
no dependency is added and no contract is relaxed in anticipation.

## 5. Gate evidence at the corrected implementation HEAD

`7bdf1dc` is green on **both** hosts (run 111): locked fetch; verified `cargo-deny 0.20.2` with a
fresh RustSec DB and the `RUSTSEC-2026-0009` negative probe refused on Ubuntu; the four-package
guard; `cargo fmt --check`; all-target `clippy -D warnings`; the full test step including the new
Linux matrix; real-tree architecture checker `architecture=legal packages=4 … edges=40`; refusal
self-test `self-check=passed cases=29`; both failure tripwires; release build; Ubuntu forbidden-import
refusal with guaranteed restore; release CLI smoke on both hosts; two byte-identical clean release
builds; and the fail-closed fixture cleanup.

Dependency graph and security boundaries are untouched: `Cargo.lock`, every `Cargo.toml`, `deny.toml`
and `rust-toolchain.toml` are byte-identical to `962ca5a`, the Linux path is still safe `rustix` only,
the single Windows-gated `unsafe` region is unchanged, and the boundary guard is unmodified.

## 6. Ledger, authority and cleanup

Paths touched by this corrective pass, all already authorized: `crates/solo-adapters/src/source_read_native.rs`
(Linux unlink/replace proofs and the narrowed module guarantee), `crates/solo-adapters/src/source_read.rs`
(narrowed guarantee), `crates/solo-adapters/tests/source_read.rs` (product unlink/replace case),
`.documents/Architecture/ARCHITECTURE.md` (§6A.8 sentence only), and this report. The optional
`crates/solo-adapters/tests/source_read_security.rs` remained absent, and no other file changed.

`SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` remain `false`;
R02.B, R02.C and Stage 02 remain open; ACTIVE R03 (`27951900`) and neutral NEXT (`92fa562b`) are
preserved, and ROADMAP (`18975332`) and OUTLOOK (`cb9c7a61`) are byte-identical. The working tree is
empty, there is one checkout and one remote, and no new temporary material was left behind; the
sanctioned `.temp/toolchain` and `.temp/target` remain under the disclosed delete-on-acceptance
trigger. WSL 2 is available on the Owner's machine but offered no Linux linker or reflink filesystem
without installing packages, so it did not add reproduction and was not used to override the order.

**This report claims no ACCEPT.** It reports a STOP on the Windows adversary, delivered Linux
evidence, and the single decision above. The Owner alone relays `agent report ready`.

