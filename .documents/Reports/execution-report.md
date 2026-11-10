# Execution Report

**Order:** `SOLO-WO-052 R01` - R02.B F2/F5, Windows direct-root handle-only staging; **REVISE round 1**.
**Disposition: `COMPLETE`** for this technical execution only. Not Architect ACCEPT, not R02.B closure, not R02.C, not release.
**Verified revision:** `eb19ba5b2e06a221578a83a874d01b5698228d3e`, tree `f0b133a6d77576c803bc465a0ab86037ff057de6` (`main` fast-forwarded to it; no history rewrite).

## 1. Correction delivered in this round

**Least privilege on the caller-owned root.** `open_staging_component` now requests exactly
`FILE_READ_ATTRIBUTES | FILE_WRITE_DATA | SYNCHRONIZE` on the staging root and
`FILE_READ_ATTRIBUTES | SYNCHRONIZE` on an intermediate. `FILE_WRITE_DATA` is the documented
`FILE_ADD_FILE` right and is what handle-relative file creation needs. Removed from the root:
`DELETE`, `FILE_APPEND_DATA` (`FILE_ADD_SUBDIRECTORY`), `FILE_WRITE_ATTRIBUTES` and
`FILE_LIST_DIRECTORY`. No further right was added: the full two-host CI below proves creation still
succeeds with this set, which is the empirical proof the order asked for. The component-relative,
`OBJ_DONT_REPARSE` walk is unchanged.

**Linux report correction.** The earlier report claimed Linux staging uses `NO_XDEV`. That was wrong,
and the behaviour is deliberately unchanged: staging-root adoption uses
`BENEATH | NO_SYMLINKS | NO_MAGICLINKS` **without** `NO_XDEV`, because a staging root on another mount
is legitimate and crossing a mount does not redirect a write. `NO_XDEV` appears only in the source
reader's leaf resolution, where a content leaf must not leave the adopted device. No Linux security
semantics were altered in this round.

**Documentation.** Module-level claims about an owned namespace, namespace disposal and an
immediately empty namespace are gone. The source now states the truth: direct-root staging with an
unenforceable-by-enumeration, unopenable delete-pending name that disappears when its last handle
closes.

## 2. Real-host adversary evidence (all executed, not reasoned)

Windows job of run `42807390454`; the same suite ran green at `eb19ba5b` in run `42806664621` and at
the pre-report revision. Named results:

- `a_pending_shard_name_cannot_be_reopened_changed_renamed_or_mapped` **ok** - the test derives the
  actual task-owned pending name by listing the caller's own directory, then attempts a read open, a
  write open, a writable-handle acquisition (the prerequisite for a writable mapped view), a delete
  and a rename **for real**. All are refused, no alias is created, the staged bytes still read back
  exactly through the legitimate retained handle, finalization succeeds, and the name is gone with
  the root snapshot equal after the last handle closes.
- `a_host_denied_disposition_refuses_before_any_plaintext_is_written` **ok** - one real native call:
  the object is created exactly as production does, with `DELETE` and `FILE_DELETE_ON_CLOSE`, and the
  disposition call is issued with a flag value this host does not define, so the kernel refuses it.
  The transaction refuses before any body byte, `object_count`/`staged_bytes` stay zero, and the
  empty file is removed by handle closure so the root is byte-identical afterwards.
- `a_preexisting_file_at_a_shard_name_is_refused_and_left_byte_identical` **ok** - the production
  `FILE_CREATE` path is pointed at a pre-existing foreign file; it refuses and the foreign bytes are
  unchanged.
- `a_root_moved_and_replaced_at_its_path_never_receives_a_staged_byte` **ok** - the retained root is
  relocated and an impostor directory installed at the old path; the impostor never receives a
  staged member or any cleanup, and the moved directory is never deleted.
- `an_impostor_at_the_namespace_name_survives_untouched` (empty and populated) **ok**,
  `an_intermediate_junction_component_is_refused` **ok**.

Staging suite: 14 passed, 0 failed on Windows; candidate suite fully green, including the seven
whole-declaration successes that were previously blocked. Linux: the mandatory Btrfs `O_TMPFILE`
fixture runs, reads back, creates no entry, and preserves Owner sentinels.

## 3. Gates, distinguished by evidence class

- **H1 PASS** - full two-host suite at one revision.
- **H2 PASS** - retained-root confinement; root relocation adversary executed.
- **H3 PASS** - real Btrfs anonymous lifecycle on Ubuntu CI.
- **H4 PASS for the executed cases** - component-relative `OBJ_DONT_REPARSE` walk, root relocation,
  name collision, impostor survival. **UNVERIFIED sub-case:** whether the junction fixture could be
  *created* on this host. The test does not convert an unavailable fixture into PASS; the log proves
  the case ran, not that creation succeeded, and that is reported as unverified rather than claimed.
- **H5 PASS** - exclusive handle (share mode zero), disposition before the first plaintext byte,
  retained byte reads after finalize, pending name gone at handle closure, `FILE_DELETE_ON_CLOSE` as
  the process-death net.
- **H6 PASS** - no directory is created, so no directory cleanup exists; impostor directories (empty
  and populated), a foreign sentinel and a colliding file are all left byte-identical.
- **H7 PASS** - injected write/short-write/flush/finalize, a real kernel disposition refusal, real
  permission denial, unwind and the bounded abort child all refuse without a partial candidate.
- **H8 PASS** - `unsafe` confined to the named Windows helpers inside the one gated module; no new
  dependency, feature, workflow or lock change; root rights reduced, not widened.
- **H9 PASS** - same-revision two-host green, plus green at the fast-forwarded `main` revision and at
  this report HEAD.
- **H10 PASS** - four WIP files, one checkout, empty `git status`, canonical single report.

**F1-F8:** F1, F3, F4, F6, F7, F8 unchanged and green. **F2 and F5 close technically** on the executed
evidence above. Residual risk, stated plainly: a Windows pending name remains enumerable while its
handle is open; it is unopenable and carries no plaintext, and it cannot outlive the handle.

## 4. Ledger

Correction commits on `solo-wo-051-r01`: `f42e1ed` (least privilege plus the new adversaries) and
`eb19ba5b` (disposition refusal through one real native call with delete-on-close), after
non-destructive merge `5d78b812` of published `main`. `main` fast-forwarded `0e2b2683 -> eb19ba5b`.
Changed paths: `crates/solo-adapters/src/source_read_native.rs`, `src/source_set_staging.rs`,
`src/lib.rs`, `tests/source_set_staging.rs`. Unchanged: `Cargo.toml`, `Cargo.lock`, `deny.toml`,
workflow, checkers, candidate expectations.

Cleanliness: one checkout, one remote, `git status` empty, `target/` removed, no task scratch kept.

**Disposition: `COMPLETE`** - R02.B technical correction finished; R02.C not started.