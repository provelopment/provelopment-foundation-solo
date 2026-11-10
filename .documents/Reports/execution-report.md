# Execution Report

**Order:** `SOLO-WO-048 R03` — bounded-memory, protected source-stage assembly; R02.B completion and R02.C readiness.
**Disposition: `COMPLETE` (technical work).** R02.B closure and R02.C readiness are **not** performed: they require the Chief Architect's independent `ACCEPT`, which this report does not and cannot issue.
**Date:** 2026-11-10.
**Repository:** `provelopment/provelopment-foundation-solo`, `main`, one checkout, one worktree, sole `origin`.

## 1. Owner issuance and installation

| Item | Value |
| --- | --- |
| Issued order | `SOLO-WO-048 R03`, 40,428 B, 172 lines, blob `11d9e3071f3016bf9afa90327cc786d282a50d79` |
| Installed at | `.documents/WIP/workorder-active.md` (uncommitted substitution by the Owner) |
| Transition commit | **`68e8a739af1b316890e3da426a0741d4e593a61c`** — that one file alone |
| Transition CI | run **144** `42740754349` — Ubuntu and Windows success |

The complete order was read end to end before any change: §1 objective, §2 A–G security properties, §3 fourteen-path ledger, §4 preflight, §5 phases I–V, §6 M1–M14, §7 CI/security gates, §8 commit sequence.

## 2. Repository preflight (§4)

- `HEAD == origin/main`, one worktree, sole `origin` = `github.com/provelopment/provelopment-foundation-solo`.
- Exactly four WIP files, no subfolder; the only pre-execution working-tree modification was the ACTIVE substitution.
- Anchor `47f657e1be79349da76e698ff252bd1b5a51d99c` (tree `4d32075d`) is an ancestor of the starting HEAD.
- Every intervening commit — `a49c6a18`, `e7d46412` — changed **only** `.documents/Reports/execution-report.md`, exactly as §4 now permits.
- **31 of 31** non-report ledger blobs matched exactly, including `Cargo.lock` `8bd0bb00…`, `rust-toolchain.toml`, `deny.toml`, `.github/workflows/rust-verify.yml`, all domain/application/adapter sources and tests, the CLI architecture and unsafe-boundary checkers, both SOPs, `README.md`, `AGENTS.md`, `ARCHITECTURE.md` `250bf62e…`, ROADMAP `18975332…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…`.
- Absence checks held: `crates/solo-adapters/src/source_set_staging.rs`, `crates/solo-adapters/tests/source_set_staging.rs` and `crates/solo-adapters/tests/source_read_security.rs` were all absent before this run.

## 3. Changed-path ledger — exactly the authorized paths

| # | Path | Operation |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | ACTIVE transition only (commit `68e8a739`) |
| 6 | `crates/solo-application/src/source_read.rs` | eager accumulator removed; sequential sink protocol added |
| 7 | `crates/solo-application/src/lib.rs` | exports adjusted |
| 8 | `crates/solo-application/tests/source_read.rs` | set-level tests re-driven through a recording sink; three added |
| 9 | `crates/solo-adapters/src/source_set_candidate.rs` | transaction + disk-backed candidate |
| 10 | `crates/solo-adapters/src/lib.rs` | module and export wiring only |
| 11 | `crates/solo-adapters/tests/source_set_candidate.rs` | adapted, plus the cumulative-bytes proof |
| 12 | `crates/solo-adapters/src/source_set_staging.rs` | **created**: staging capability, store, bounded reader |
| 13 | `crates/solo-adapters/tests/source_set_staging.rs` | **created**: real-host staging adversaries |
| 4 | `.documents/Reports/execution-report.md` | this report |

No other path changed. `publication_manifest.rs`, `source_body_preflight.rs`, `source_read.rs` (adapter), `source_read_native.rs`, all `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.github/**`, the checkers, the SOPs, `AGENTS.md`, `README.md`, `ARCHITECTURE.md`, ROADMAP, NEXT and OUTLOOK are byte-identical to the baseline. Ledger item 5 (ARCHITECTURE.md) was deliberately **not** used: no sentence was needed to state a parity that this report states instead.

## 4. Proof-first design: the resource and lifetime state machine

```
manifest bytes
   │  strict parse (unchanged parser); enumerate every declaration (manifest order)
   ▼
private namespace created exclusively inside the caller's staging root
   │
   ├─ reservation pass: probe EVERY declared candidate of EVERY representation
   │                    (identity, link count) — no body is read yet
   │
   for each declaration, in declaration order:
   │     read ONE selected body (accepted JSON-first rule, unchanged per-file guarantee)
   │     preflight it (accepted selected-body preflight, unchanged)
   │     stage the validated bytes into a nameless object
   │     ── both transient copies dropped here ──
   │  on any refusal: typed refusal, staged objects dropped and cleaned, no candidate
   ▼
commit: candidate owns the store; members carry ranges and metadata only
```

**Every transient and retained allocation**

| Stage | What is alive | Bound |
| --- | --- | --- |
| parse | manifest bytes + parsed tree | ≤ 2 MiB manifest, unchanged |
| reservation | `Vec<PhysicalReservationV1>` / locator clones | O(declarations) metadata, no body |
| one declaration | the accepted read's `Vec<u8>` **and** the preflight's validated copy | ≤ 2 × the accepted single-body limit (2 MiB JSON, 1 MiB Markdown) |
| staging write | the bytes handed in, no extra buffer | zero additional copies |
| completed candidate | ranges + lineage + identity + fingerprint + length | **0 body bytes**; O(declarations) metadata |
| staging reader | one 8 KiB stack buffer | fixed |
| staging objects | one open handle per 64 MiB staged | `ceil(total / 64 MiB)`, independent of declaration count |

Peak live body bytes are therefore a constant multiple of the **accepted single-body maximum**, not a function of how many representations the manifest declares nor of their cumulative size. The measured value is exposed as `SourceSetCandidateV1::max_live_body_bytes` and asserted on real hosts (§6, M10).

## 5. The staging mechanism, proved per host

Each object is created with exclusive creation (`create_new`) inside a namespace this run created itself with an exclusive `create_dir` (mode `0700` on Linux), and its directory entry is **removed while the handle stays open**:

- **Linux** — ordinary unlink-while-open: the inode remains reachable only through the handle this store owns.
- **Windows 2025** — `std` opens files with `FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE`, so the delete succeeds, the name disappears immediately and the handle stays valid for metadata and positional reads.

Reads use the platform's **safe** positional-read extension — `std::os::unix::fs::FileExt::read_at` and `std::os::windows::fs::FileExt::seek_read`. No `unsafe`, no FFI, no memory mapping, no new crate, no feature flag, no build script and no `Cargo.lock` change is involved, and the single audited Windows-gated `unsafe` boundary in `source_read_native.rs` was not touched.

Consequences that the tests prove rather than assert:

- There is no path to re-open, replace, truncate or redirect a staged member: it has no name.
- A same-user adversary can create a *new* entry with the object's former name; that is a different object and cannot alter the staged member.
- Because objects are nameless, an abruptly terminated process leaves nothing to recover — at most an empty namespace directory. This is a narrower, proof-backed statement than an executed kill experiment (see §9).
- Cleanup removes only the namespace this run created and only when empty; pre-existing material in the adopted root is untouched.

## 6. Public API, before and after

| Before (R01) | After (R03) |
| --- | --- |
| `read_declared_source_set_v1(port, declarations) -> SourceReadSetOutcomeV1` collecting **every** chosen body | `drive_declared_source_set_v1(port, declarations, sink) -> SourceReadSetOutcomeV1<S::Complete, S::Refusal>` — reads one declaration, moves that one body into the sink, reads the next |
| `SourceReadSetOutcomeV1::Complete(ReadSourceSetV1) / ::Refused(..)` | `SourceReadSetOutcomeV1::Complete(C) / ::ReadRefused(SourceReadSetRefusalV1) / ::SinkRefused(R)` — the outcome names **who** refused |
| `ReadSourceSetV1` with `entries()/get()/iter()/into_entries()` over owned bodies | **removed**; no public all-body accumulator exists |
| `assemble_declared_source_set_v1(manifest, root)` retaining all bodies | `assemble_declared_source_set_v1(manifest, root, staging_root)` |
| `SourceSetMemberV1::bytes() -> &[u8]`, `body() -> &InertBodyCandidateV1` | `evidence()`, `byte_length()`, `staged_body()`; bytes are read through the candidate |
| — | `SourceSetCandidateV1::{reader, member_bytes, retained_body_bytes, max_live_body_bytes, staged_bytes, staged_object_count, visible_staging_entries}` |
| — | `StagingRootCapability::{adopt, begin}`, `SourceSetStagingV1::{stage_body, reader, …}`, `StageReaderV1::{read_chunk, read_whole, matches}`, `StagedBodyV1` |

`DeclaredSourceSetSinkV1 { type Refusal; type Complete; stage(SetSourceEntryV1); finish() }` is the new application-side port fragment; `finish()` is the only route to a completed value, so no partial set is observable.

## 7. M1–M14 evidence

**Command for every case:** the workflow's own `Test (workspace, all targets, locked)` step — `cargo test --locked --workspace --all-targets` — executed in run **146** on **Ubuntu 24.04 job `129590199277`** and **Windows 2025 job `129590198959`**, both `success`. Class per case: `PURE` (no filesystem), `APP FAKE` (fake port, deterministic), `REAL` (actual host filesystem, both hosts unless stated).

| Gate | Evidence | Class | Result |
| --- | --- | --- | --- |
| **M1** | `every_declared_representation_is_assembled_with_its_lineage` — ordered five-part lineage, member kind, logical path, identity, original FNV fingerprint, and identical bytes retrieved from the live stage through `member_bytes` | REAL | pass, both hosts |
| **M2** | `an_expanded_manifest_assembles_every_representation_exactly_once_in_order` — one Entity, two Facets, several Sites, scoped duplicate Page ids, deterministic repeat | REAL | pass, both hosts |
| **M3** | `a_declared_json_body_wins_and_an_invalid_unselected_markdown_is_never_read`, `markdown_is_selected_only_when_the_declared_json_is_confirmed_absent`, `a_present_but_invalid_json_body_is_refused_with_no_markdown_fallback`; fake-port equivalents `a_present_json_body_is_chosen…`, `a_confirmed_json_absence…` | REAL + APP FAKE | pass, both hosts |
| **M4** | `two_declarations_hardlinked_to_one_object_are_refused_before_any_body_is_read`; fake-port `a_cross_representation_identity_collision…`, `an_unselected_hardlinked_markdown…`; the unchanged accepted native/source suites still refuse case/alias/reserved-name/link/junction/mount escapes | REAL + APP FAKE | pass, both hosts |
| **M5** | `one_invalid_representation_refuses_the_whole_set`; `a_refusing_sink_ends_the_attempt_and_no_later_declaration_is_read`; `one_refused_declaration_refuses_the_whole_set_and_the_next_one_is_never_read` | REAL + APP FAKE | pass, both hosts |
| **M6** | `owned_bytes_and_identity_survive_a_change_to_the_source_after_assembly` — the authored source is rewritten after assembly; staged bytes, identity and fingerprint are unchanged and the original path is never reopened | REAL | pass, both hosts |
| **M7** | `a_new_entry_with_a_former_object_name_cannot_change_a_staged_member` (the former name is re-created with hostile content and the member still matches), `a_staged_member_is_nameless_and_reads_back_exactly` (no visible entry; out-of-range read refused) | REAL | pass, both hosts |
| **M8** | `an_unprovable_staging_root_is_refused_instead_of_followed` (relative, missing, non-directory), `a_symlinked_staging_root_is_refused_rather_than_followed` (Linux), `dropping_the_store_removes_only_its_own_namespace` (pre-existing sentinel preserved) | REAL | pass; Linux symlink case exercised there, Windows reparse equivalent see §9 |
| **M9** | `capacity_failures_are_io_errors_and_never_absence`, `every_failure_kind_maps_onto_a_registered_token` — the classification is a pure function and is asserted exactly for storage-full, quota, size, permission, missing, existing, unsupported and generic failures, including that none is reported as absence | PURE | pass, both hosts |
| **M10** | `cumulative_staged_bytes_exceed_the_live_body_bound` (2,400,000 B staged across 8 members > the 2 MiB single-body bound, `retained_body_bytes == 0`, `max_live_body_bytes ≤ 4 MiB`, one nameless object), `open_object_count_follows_staged_size_not_member_count` (500 members, one handle), `the_driver_never_holds_more_than_one_body_while_many_pass_through` (observed peak 1 body while 6 bodies pass) | REAL + APP FAKE | pass, both hosts |
| **M11** | No limit was added or altered: `MAX_JSON_SOURCE_BYTES`, `MAX_MARKDOWN_SOURCE_BYTES`, `MAX_JSON_BODY_BYTES`, `MAX_MARKDOWN_BODY_BYTES`, the 2 MiB manifest and the EFS ceilings are byte-identical, and the unchanged at-bound/just-over suites still pass | REAL | pass, both hosts |
| **M12** | `dropping_the_store_removes_only_its_own_namespace`, plus `visible_staging_entries() == 0` on a live candidate in M10; abrupt-exit handling see §9 | REAL | pass, both hosts |
| **M13** | The unchanged accepted suites ran in the same jobs: Linux Btrfs positive and tmpfs negative, unlink/replacement, same-sized mutation, Windows share-mode and writable-view adversaries, exclusive fixture collision; no native helper was added and the `unsafe` region is untouched | REAL | pass, both hosts |
| **M14** | Architecture checker `architecture=legal packages=4 … edges=40` (the workflow's own `edges=40` gate passed on both hosts), refusal self-check `self-check=passed cases=29`, unsafe-boundary matrix and tripwire both green; `retained_body_bytes() == 0`; both authority constants still `false`; no new dependency is reachable | PURE + REAL | pass, both hosts |

**Test and warning inventory.** Application tests 34 (31 before, 3 added), adapter candidate tests 12 (11 before, 1 added), staging integration tests 6 (new file), staging unit tests 2 (new). No test was deleted, ignored, skipped or shrunk; no warning was suppressed; `cargo clippy --workspace --all-targets -D warnings` and `cargo fmt --check` were also run locally and are clean. One platform-conditional case exists (`a_symlinked_staging_root_is_refused_rather_than_followed`, `#[cfg(unix)]`), and it is a host-capability difference, not a suppressed failure.

## 8. Refusal matrix and storage-exhaustion behaviour

No new diagnostic vocabulary was introduced. Every staging failure maps onto an already registered token, and the mapping is asserted exactly:

| Host condition | Registered code | Why this code |
| --- | --- | --- |
| Storage full, quota exceeded, file too large, out of memory, zero-length write | `SOURCE_OBSERVATION_IO_ERROR` | an I/O failure — never absence, never a Markdown fallback |
| Permission denied on the object or the namespace | `SOURCE_OBSERVATION_PERMISSION_DENIED` | the accepted permission refusal |
| An object this store already owns answers outside its validated range, or stops producing data | `SNAPSHOT_CHANGED` | a detected change to bytes we already own |
| A namespace or object name already exists where exclusive creation was required | `SOURCE_UNSAFE_PATH` | an unprovable location, never adopted or reused |
| The host provides no positional read, so the guarantee cannot be proved | `SOURCE_UNSUPPORTED` | an honest capability refusal |
| Relative, missing, symlinked or non-directory staging root | `SOURCE_UNSAFE_PATH` | the location cannot be proven private |

A staging refusal is carried as `SourceSetRefusalV1::Staging { declaration, refusal }` and ends the attempt with **no** candidate: earlier staged members are dropped with the store. Storage exhaustion therefore refuses rather than switching to an all-RAM path, and it is never reported as a positively confirmed absent authored JSON.

## 9. Security, A1 and version findings

- **Component gate.** The workflow's pinned `cargo-deny 0.20.2` step — advisories with a fresh RustSec refresh, bans, licences, sources, plus its negative probe — passed on both hosts in run 146. No dependency, `Cargo.lock`, toolchain, workflow or CI action was added or changed, so the reviewed closure is unchanged from the accepted baseline.
- **New attack surface.** One private namespace directory (mode `0700` on Linux) and one nameless object per 64 MiB. No new `unsafe`, FFI, mapping, symlink traversal, shell, network or provider path; the single audited Windows `unsafe` region was not modified and the unsafe-boundary checker stayed green.
- **TOCTOU.** Removed by construction for staged bytes: an object without a directory entry cannot be raced by pathname. The residual same-user risk is a new entry inside the namespace, which the M7 adversary shows cannot affect a sealed member.
- **Confidentiality.** Diagnostics carry a declaration index and a registered token only — no absolute path, no body byte, no handle, no native error text; the refusal-diagnostic test asserts the rendered message contains no `pages/` fragment.
- **Not claimed.** Green CI is not itself security evidence, and this report makes no claim beyond the executed adversaries above.

## 10. Continuous integration at each HEAD

| Commit | Purpose | Run | Ubuntu | Windows |
| --- | --- | --- | --- | --- |
| `68e8a739` | ACTIVE transition alone | **144** `42740754349` | success | success |
| `91db2910` | first implementation | **145** `42742540818` | **failure** (test step) | **failure** (test step) |
| `9b9b9854` | corrective: staged-change token expectations | **146** `42743558046` | **success** `129590199277` | **success** `129590198959` |

Run 145 is reported rather than hidden: the implementation compiled and passed format, Clippy, architecture, unsafe and dependency gates on both hosts, and failed only in the test step because two new unit assertions expected `SOURCE_OBSERVATION_CHANGED_DURING_READ` where the accepted contract maps a detected change to `SNAPSHOT_CHANGED`. The correction changed those expectations and the matching doc comments — no product behaviour changed — and run 146 is fully green.

## 11. Residual limitations, cleanup and state

**Limitations, stated rather than glossed over**

1. Real device exhaustion was **not** induced: ENOSPC/quota is covered by the deterministic classification test, not by filling a host disk. The OS→code mapping is proven; the OS actually producing that error is standard behaviour, not an executed fixture.
2. The Windows reparse-point staging-root adversary is not exercised (creating one needs privileges this runner does not grant); the equivalent Linux symlink case is, and the Windows-side relative/missing/non-directory refusals are exercised.
3. No abrupt-kill experiment was executed. The narrower, proof-backed statement stands: objects are nameless while alive, so a terminated process leaves at most an empty namespace directory and no recoverable plaintext entry.
4. Open handles grow as `ceil(staged bytes / 64 MiB)`. A maximum-size set (~9.8 GiB) therefore needs ~153 handles and ~9.8 GiB of scratch — an inherent consequence of Option B, disclosed rather than capped by a new product limit.
5. Peak live body bytes are measured by implementation counters in the staging transaction, not by an external allocator profiler.
6. Markdown bodies are validated at the accepted byte level (size, UTF-8, BOM, non-empty); Markdown grammar remains a later stage.
7. This stage proves stable **staged** bytes. It does not prove a simultaneous manifest-plus-files filesystem snapshot, and it grants no publication authority — both constants remain `false`.

**Temporary material and cleanup.** The pinned `rust-toolchain 1.99.1` was installed into Solo `.temp/` (git-ignored) purely to run `fmt` and `clippy` locally, and the whole `.temp/toolchain` tree (2.7 GB) plus `.temp/logs` were deleted before this report commit. The workspace `target/` directory produced by those checks was deleted and is not tracked. Pre-existing, not-task-owned items in `.temp/` (`architecture-fixtures`, `rt`, `target`, `toolchain-installer`, `w032`, `w033`) were **left untouched**; their ownership is unverified and they were not created by this run.

**State.** One checkout, one worktree, sole `origin`, `HEAD == origin/main`, `git status --porcelain` empty apart from this report. Exactly four WIP files and no subfolder. ACTIVE still holds the issued R03 order; NEXT remains the neutral `92fa562b…`; ROADMAP still contains the accepted R02.B bullet plus R02.C and Stage 02; OUTLOOK unchanged. `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false`.

**Disposition: `COMPLETE` for the technical scope, reported to the Owner only.** R02.B is **not** closed, R02.C is **not** started and ACTIVE is **not** neutralized: all three require the Chief Architect's independent `ACCEPT` and a further express Owner direction. No publication, release, deployment or provider action was performed.
