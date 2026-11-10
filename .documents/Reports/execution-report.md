# Execution Report

**Order:** `SOLO-WO-052 R01` - R02.B F2/F5, Windows direct-root handle-only staging; **REVISE round 2**.
**Disposition: `COMPLETE`** for this technical execution only. Not Architect ACCEPT, not R02.B closure, not R02.C, not release.
**Verified revision:** `1f4de7f439e81ea9af36b655688627c598f281f2`, tree `2a9ee1a8d4d0d4233c85884811a5ea78ca2b7c41`; `main` fast-forwarded to it. Windows root keeps the
least-privilege rights; Linux/Btrfs behaviour untouched.

## 1. The two weak adversaries are now decisive

**Junction.** `an_intermediate_junction_component_is_refused` no longer tolerates a failed fixture. It
must create the junction with `mklink /J`, then **prove the redirect is real** by writing a probe
through the junction and requiring it to appear in the real tree, and only then require
`StagingRootCapability::adopt` to refuse with exactly `SOURCE_UNSAFE_PATH`. A creation failure aborts
the case with the sanitized host reason and exit code - no fixture path is printed - instead of
returning successfully. Because the test is green on windows-2025, the redirect existed, redirected,
and was refused.

**Root substitution.** `a_root_moved_and_replaced_at_its_path_never_receives_a_staged_byte` now treats
a host refusal to move the held directory as a **failure with an explicit marker**, not as a quiet
pass. A bare `ok` therefore means, by construction, that the substitution really executed: the
adopted directory was moved aside, an impostor was installed at its former path, no staged byte
reached the impostor, and the moved directory was never deleted. The run is green, so substitution is
**EXECUTED**, not inferred.

Adversaries that executed on the real Windows runner: pending-name reach (read open, write open,
writable-handle acquisition for a mapped view, delete, rename - all refused, no alias), native
disposition refusal, native `FILE_CREATE` name collision, root relocation with impostor, impostor
survival (empty and populated), intermediate junction. **None was refused before construction, and
none is reported from an unexecuted fixture.**

## 2. Exact refusal codes

Assertions are pinned to the registered classification, not a family:
- name collision -> `SOURCE_UNSAFE_PATH` (`STATUS_OBJECT_NAME_COLLISION` is the unsafe-path refusal);
- kernel-refused disposition -> `SOURCE_OBSERVATION_IO_ERROR` (a genuine native failure, never a
  claimed path refusal).

## 3. Accurate wording, and the correction the report itself needed

The disposition seam does **not** omit `DELETE` or `FILE_DELETE_ON_CLOSE`. The object is created
exactly as production creates it and the **kernel refuses the disposition call itself**; that is the
real-host refusal, and `FILE_DELETE_ON_CLOSE` is what removes the still-empty file at closure. All
comments and doc comments now say this, in `source_read_native.rs`, `source_set_staging.rs` and the
test.

Obsolete namespace-creation, namespace-disposal and immediately-absent-name claims are gone from the
authorized files. The source states the truth instead: **direct-root staging with an enumerable,
unopenable `DELETE_PENDING` name** that disappears when its last handle closes. `source_set_staging`
now mentions "namespace" only to deny it.

Linux staging still **omits** `NO_XDEV` on purpose (a staging root on another mount is legitimate);
`NO_XDEV` belongs to original-source leaf resolution, which is separate. No Linux security semantics
changed in this round.

## 4. Ledger - corrected

Correction commits on `solo-wo-051-r01`: `898127c4` (decisive junction and substitution, exact codes,
wording) and `1f4de7f4` (green substitution must mean executed), after a non-destructive merge of
published `main`. `main` fast-forwarded `e4cdf1cc -> 1f4de7f4`.

**Changed paths in this revision:** `crates/solo-adapters/src/source_read_native.rs`,
`src/source_set_staging.rs`, `src/source_set_candidate.rs`, `tests/source_set_staging.rs`.
**`src/lib.rs` did not change** - the earlier ledger entry for it was wrong and is retracted here.
Unchanged: `Cargo.toml`, `Cargo.lock`, `deny.toml`, workflow, checkers, candidate expectations.

## 5. Gates

- **H1 PASS** - full two-host suite at one revision (`42810918941`), and again at the fast-forwarded
  `main` revision (`42811616822`).
- **H2 PASS** - retained-root confinement; relocation adversary executed.
- **H3 PASS** - real Btrfs anonymous lifecycle on Ubuntu CI.
- **H4 PASS** - the previously unverified junction case is now executed and proved: created, verified
  to redirect, refused as `SOURCE_UNSAFE_PATH`. This is a real-host result, not source reasoning.
- **H5 PASS** - exclusive handle, disposition before the first plaintext byte, exact retained reads,
  pending name gone at handle closure, `FILE_DELETE_ON_CLOSE` for process death.
- **H6 PASS** - no directory created or removed by staging; impostor directories (empty and
  populated), foreign sentinel and colliding file all left byte-identical.
- **H7 PASS** - injected write/short-write/flush/finalize, real kernel disposition refusal, real
  permission denial, unwind and the bounded abort child all refuse without a partial candidate.
- **H8 PASS** - `unsafe` confined to the named Windows helpers in the one gated module; no new
  dependency, feature, workflow or lockfile change; root rights reduced, never widened.
- **H9 PASS** - same-revision two-host green, fast-forwarded `main` green, and this report HEAD green.
- **H10 PASS** - four WIP files, one checkout, empty `git status`, one canonical report.

**F1-F8:** all green. **F2 and F5 close technically** on the evidence above. Residual risk unchanged
and disclosed: a Windows pending name stays enumerable while its handle is open; it is unopenable,
carries no plaintext, and cannot outlive the handle.

Cleanliness: one checkout, one remote, `git status` empty, `target/` removed, no task scratch kept.

**Disposition: `COMPLETE`** - R02.B technical correction finished; R02.C not started.