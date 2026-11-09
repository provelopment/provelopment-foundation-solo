# Execution Report

**Order:** `SOLO-WO-047 R03` — R02.B third bounded slice: proof-first immutable native source snapshot (Owner-approved D2).
**Run kind:** implementation and technical report under that order; D1's residual and D3's operator promise remain rejected.
**Repository:** `provelopment/provelopment-foundation-solo`, branch `main`, sole `origin`, one checkout.
**Starting HEAD:** `d2c42e46e193886dc153c8262c5e709488e69d5e` / tree `b43850f77f503bb5d98221716ad02a5142ac9a5b` (all 20 §3 blobs matched; `source_read_security.rs` absent).
**Implementation HEAD:** `63ea2254118596074ab198fbdf249e9eb58a805e`
**Implementation tree:** `2f9c543828975a9fa2068ff8785e2271c62d0e40`
**Implementation workflow:** `rust-verify` run `42605284158` (run 107) — **success**
&nbsp;&nbsp;· Ubuntu job `129119596652` **success** (10:59:03 → 11:00:33)
&nbsp;&nbsp;· Windows job `129119596343` **success**
**Report HEAD:** its own SHA and the IDs of its own `rust-verify` run are given in chat, because a commit cannot contain its own SHA.

---

## 1. What was built

The Linux native path now takes a **real immutable snapshot** of the source before any byte is
returned, inside the one audited module (`crates/solo-adapters/src/source_read_native.rs`), in safe
`rustix` only:

1. the existing trusted root (`open` + `O_DIRECTORY|O_NOFOLLOW|O_CLOEXEC`, or an owned handle
   transferred by the caller and verified on the handle) and the existing confined source
   (`openat2` with `RESOLVE_BENEATH|NO_SYMLINKS|NO_MAGICLINKS|NO_XDEV`) are held as owned descriptors;
2. the source is re-`fstat`ed: it must be a plain regular file, its `st_dev` must equal the trusted
   root's, and its length must be inside the per-kind bound (JSON 2 MiB, Markdown 1 MiB) — an
   over-bound body is refused **before** anything is cloned;
3. `openat(root_fd, ".", O_TMPFILE|O_EXCL|O_RDWR|O_CLOEXEC, 0600)` creates an **unnamed, unlinkable**
   inode on the source's own filesystem;
4. exactly one `rustix::fs::ioctl_ficlone(&clone, &source)` — one call, no retry, no alternative;
5. the clone's own `st_dev`, regularity and length are checked against the source, and only then is
   the clone read, exactly `length` bytes, with a short clone refused as a change;
6. the clone descriptor is dropped (RAII) on success, on error and on panic, and nothing is ever
   named, linked, renamed or persisted.

Typed failure mapping is separate from the ordinary resolver mapping: `EOPNOTSUPP`, `ENOSYS`,
`EINVAL`, `EPERM`, `EROFS` and `EBUSY` are `Unsupported` (a refusal, never a copy and never absence),
`EXDEV` and a destination on another device are `Unsafe`, `EACCES` is `PermissionDenied`, everything
else is `OtherIo`. There is no `copy_file_range`, `read`/`write` copy, `std::fs::copy`, `mmap`,
advisory lock, sleep/retry or second path-open anywhere in the path.

Windows keeps its existing `NtCreateFile` traversal, by-handle reparse/type/file-ID checks and
owned-handle transfer, and its body handles request `ShareAccess = FILE_SHARE_READ`, which is what
makes a competing writer or delete fail while the read is pinned.

The application port is unchanged in shape; the only new application change is the defect R03
names in §1: `check_physical_reservations_v1` now refuses a candidate whose link count is not one
**before** recording any reservation, so a sole declaration and an unselected member are both
refused for ambiguity. No OS type, no new authority and no new dependency entered
`solo-application` or `solo-domain`.

## 2. Feasibility, and the four red runs it cost

Stage I confirmed the pinned API before any product file changed: `rustix 1.1.5` exposes
`pub fn ioctl_ficlone<Fd: AsFd, SrcFd: AsFd>(fd: Fd, src_fd: SrcFd) -> Result<()>` under feature
`fs` (it is **safe**, so no new `unsafe`), and `OFlags` carries `TMPFILE`, `EXCL`, `RDWR` and
`CLOEXEC`. The hosted Ubuntu root filesystem is ext4, which cannot reflink, so the positive proof
had to be built on a bounded disposable fixture. Every red run below is a real defect found by real
evidence, not a suppressed or retried gate:

| Run | HEAD | Result | Exact finding and correction |
| --- | --- | --- | --- |
| 100 | `331f74e` | green both | Issuance commit only (ACTIVE replaced byte-identically; NEXT neutral, ROADMAP/OUTLOOK untouched). |
| 101 | `a81b996` | Ubuntu red, Windows red | My first probe module named `rustix` in a **test** file, which the load-bearing boundary guard refuses (`unsafe_boundary::no_first_party_file_outside_the_native_module_uses_unsafe`): the guard is not relaxed for tests. The primitive proof moved **into** the audited native module as unit tests over the real `snapshot_leaf`/`read_bounded`. Ubuntu's red was the fixture step, whose first version reported only an exit status. |
| 102 | `e68ed37` | Windows **green**, Ubuntu red | Diagnostic annotations named it: `chown: cannot access '.../content': No such file or directory` — the test directory had been created *before* the loop mount and was shadowed by it. Fixed by creating it inside the mounted filesystem. |
| 103 | `071d672` | Ubuntu red | `mkdir .../content: Permission denied` — a freshly formatted btrfs root is root-owned, so the directory is created with the privilege the format requires and then chowned to the runner user. |
| 104 | `6bf6016` | Ubuntu red (one test) | `d2_product::a_snapshot_leaves_no_named_destination_and_no_open_descriptor`: my oracle counted the capability's own legitimate content-root handle as "a descriptor left open". The baseline is now measured with that handle alive, so what is asserted is that no *per-read* descriptor survives 24 reads. |
| 105 | `49d87ca` | **green both** | First complete D2 proof. |
| 106 | `1d22122` | green both | Architecture §6A.8 paragraph and the two module-level guarantees restated to match what is now proven. |
| 107 | `63ea225` | green both | Equal-size source rewrite added to the clone proof. |

The fixture is bounded exactly as §4 permits: one 192 MiB loop-backed Btrfs image and one 16 MiB
tmpfs mount, both under `$RUNNER_TEMP`, uniquely named from `github.run_id`/`run_attempt`, mounted
by `sudo` on preinstalled tools only, with the test directory passed to the tests as
`SOLO_TEST_SOURCE_FS_ROOT` and the non-reflink target as `SOLO_TEST_UNSUPPORTED_FS_ROOT`. No package
was installed, no host mount was changed, nothing outside `$RUNNER_TEMP` was written, and no image
is uploaded as an artifact. The final step is `if: always()`, unmounts only the two exact
task-owned mount points, refuses to delete behind a live mount, detaches only a loop device that
still references this job's own image, deletes only the owned image and mount points, and then
asserts that all three paths are gone (`r03_fixture_cleanup=complete`). That cleanup ran and
succeeded in every run above, including the red ones.


## 3. D2 requirement matrix — evidence class and observed result

`native` means executed on the real host against the real kernel and filesystem; `fake` is the
application-level fake port; `injected` is a deliberately injected outcome. No simulated case is
presented as a native filesystem proof, and no native case is claimed from a fake port.

### 3.1 Linux — supported filesystem (real Btrfs fixture, real `FICLONE`)

| Requirement | Test (all PASS, run 107) | Class |
| --- | --- | --- |
| A real anonymous `O_TMPFILE|O_EXCL` + `ioctl_ficlone` snapshot succeeds | `snapshot_proofs::an_anonymous_clone_keeps_its_bytes_while_the_source_is_rewritten_in_place` | native |
| The clone is its own inode and shares no identity with the source | same test (distinct `st_ino`, equal length before the rewrite) | native |
| Source overwritten **after clone, before read**: clone bytes unchanged | same test, length-changing rewrite | native |
| **Equal-size** source rewrite: clone bytes unchanged | same test, second rewrite of identical length | native |
| A second read of the same clone returns identical bytes | same test (two reads after each mutation) | native |
| A fresh snapshot *does* see the new bytes (clone is an instant, not a cache) | same test (`re-clone` → `REWRITTEN`) | native |
| No named destination is ever created | same test (`read_dir` count stays 1) + `d2_product::a_snapshot_leaves_no_named_destination_and_no_open_descriptor` | native |
| No snapshot descriptor survives the read; repeated drops are clean | `d2_product::a_snapshot_leaves_no_named_destination_and_no_open_descriptor` (24 reads, descriptor baseline unchanged) | native |
| Exact bound accepted, bound+1 refused before cloning | `snapshot_proofs::an_oversized_source_is_refused_before_anything_is_cloned`, `d2_product::a_body_at_the_exact_bound_is_snapshotted_and_one_byte_over_is_refused` | native |
| Zero-length source is an empty snapshot | `an_empty_json_leaf_is_present_with_no_bytes` (unchanged test, now through the clone) | native |
| Unselected Markdown never read while JSON is available | `an_oversized_markdown_is_never_read_when_json_exists` and the retained application cases | native/fake |
| No source write from the product read path | the read path holds `RDONLY` source descriptors only; asserted structurally by the module's tests creating no entry | native |

### 3.2 Linux — real unsupported filesystem (real tmpfs, real `EOPNOTSUPP`)

| Requirement | Test (PASS, run 107) | Class |
| --- | --- | --- |
| A real filesystem without reflink refuses at the primitive | `snapshot_proofs::a_filesystem_without_reflink_is_refused_instead_of_copied` (`Unsupported`) | native |
| The **product** path refuses rather than copying | `d2_product::a_filesystem_without_reflink_is_refused_instead_of_read_by_copying` (`SOURCE_UNSUPPORTED`) | native |
| No fallback, no false absence | both above: tmpfs reads perfectly, so only a real clone requirement can refuse it | native |
| Mount-crossing, root/reparse/hardlink and permission refusals stay classified | the retained negative suite (`a_root_that_is_a_link_is_refused`, `a_leaf_that_is_a_link_is_refused`, `an_intermediate_directory_that_is_a_link_is_refused`, `a_leaf_with_a_second_directory_entry_is_refused`, `a_leaf_hardlinked_from_an_unrelated_directory_is_refused`, `a_directory_in_place_of_a_leaf_is_refused`, `a_missing_intermediate_directory_is_a_confirmed_absence`) | native |

### 3.3 Windows — real writer exclusion (real Windows 2025)

| Requirement | Test (PASS, run 107) | Class |
| --- | --- | --- |
| A preexisting incompatible write handle blocks the reader | `writer_exclusion::a_preexisting_write_handle_prevents_the_body_read` | native |
| While a body handle is pinned, a competing write open fails | `writer_exclusion::the_reader_share_mode_denies_a_competing_writer_and_a_delete` | native |
| While a body handle is pinned, a delete fails | same test | native |
| The denial comes from the pin, not from a locked file | same test: both succeed once the pin is dropped | native |
| Reparse/junction, file-ID, transfer and hardlink coverage retained | `a_transferred_handle_to_a_junction_is_refused`, `a_leaf_that_is_a_link_is_refused`, `two_declared_members_that_are_one_object_are_refused`, `a_leaf_hardlinked_from_an_unrelated_directory_is_refused` | native |

### 3.4 Application — the global reservation defect

| Requirement | Test (PASS) | Class |
| --- | --- | --- |
| A **sole** declaration with two directory entries is refused | `a_sole_declaration_with_two_directory_entries_is_refused_before_any_reservation` | fake |
| An **unselected** multi-link member is refused globally | `an_unselected_multi_link_member_is_refused_by_the_global_reservation` | fake |
| Refused before any body read | both assert the exact call list (`probe:*` only) | fake |
| Collision diagnostics unchanged | both assert `SOURCE_COLLISION` and the offending member | fake |
| Earlier reservation, probe-to-read and publication-false tests still pass | the whole retained suite, including `two_representations_resolving_to_one_physical_object_are_refused`, `distinct_representations_reserve_their_own_identities`, `a_link_count_that_changes_between_probe_and_read_is_refused` | fake/injected |

The full R02 §4 30-row matrix is retained and still passes; every row that was `native` before is
still `native`, now additionally inside the snapshot path on Linux.


## 4. Version, safety and the complete gate evidence

Rust stays pinned at `1.99.1`; the locked graph is unchanged (four first-party packages, 22 total
locked packages, `rustix =1.1.5` Linux-only, `windows-sys =0.61.2` Windows-only), `Cargo.lock`,
every `Cargo.toml`, `deny.toml` and `rust-toolchain.toml` are byte-identical to the starting tree,
and no crate, feature, version or allowance was added. The Linux implementation is **safe Rust**:
`ioctl_ficlone`, `openat`, `fstat` and `read` are all safe `rustix` calls, so the project's only
`unsafe` remains the Windows-gated region in `source_read_native.rs` with its unchanged
`// SAFETY:` notes; `#![deny(unsafe_code)]` at the adapter root and `#![forbid(unsafe_code)]` in the
inner crates are untouched, and the boundary guard continues to refuse `unsafe`, native imports or
extra allowances in every other first-party file — it is stricter than before, not looser, because
it now also covers the new test module.

Green at `63ea225` on **both** hosts: locked fetch; the verified `cargo-deny 0.20.2` gate
(advisories, bans, licenses, sources, fresh RustSec DB, `RUSTSEC-2026-0009` advisory negative probe
refused with exit 1 on Ubuntu); the four-package guard; `cargo fmt --check`; all-target
`clippy -D warnings`; the whole test step including every D2 test above; the real-tree architecture
checker (`architecture=legal packages=4 … edges=40`); the refusal self-test
(`self-check=passed cases=29`); both test-failure tripwires; the release build; the Ubuntu forbidden
import rejection with guaranteed restore; the release CLI smoke on both hosts; and the two
byte-identical clean release builds. No gate was weakened, skipped or made advisory, and the
fixture cleanup is itself a fail-closed step that ran green.

## 5. Verified limitations — explicitly not claimed

* **Kernel and tool versions are not reported here.** The fixture step records `host_kernel`,
  `mkfs_version`, `supported_mount` and `unsupported_mount` in the Ubuntu job log, which this
  environment cannot read (raw log download returns 403). The *filesystem classes* are nonetheless
  proven by behaviour: the positive target cloned and the negative target refused with a real
  `EOPNOTSUPP`. No kernel or `mkfs.btrfs` version claim is made.
* **Kernel `ENOSYS` is UNVERIFIED and was not simulated.** It was unnecessary: a genuine unsupported
  filesystem produced a genuine unsupported error, so no simulated `ENOSYS` classification is
  claimed anywhere.
* **Identical synthetic timestamps on the equal-size rewrite are UNVERIFIED.** The equal-size
  rewrite itself is proven natively; setting both timestamps identically would need a new
  timestamp API or dependency, which R03 does not authorize. This does not weaken the proof: the
  clone's bytes are separate before any timestamp is consulted.
* **No injected clone/read barrier seam was built.** Instead the unit proof holds one real clone
  across a real source mutation and reads it twice — a stronger oracle than an injected seam, and it
  is labelled as the real mechanism proof it is.
* **The Windows preexisting *memory-mapping* adversary is UNVERIFIED.** A writable mapped section
  held across the read was not constructed; the proven Windows exclusion is the share-mode
  behaviour of the read handle against write and delete opens.
* **Per-file only.** Nothing here proves a global candidate, manifest, asset or multi-file
  publication snapshot. `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false` and
  `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false` are unchanged, no publication, routing,
  visibility or validation authority was added, and R02.B, R02.C and Stage 02 remain **open**.
* **A product host without the clone capability must refuse.** That is now enforced
  (`SOURCE_UNSUPPORTED`), not merely documented, and there is no best-effort degradation.


## 6. Path and blob ledger

Nine of the eleven authorized paths were touched; the optional one stayed absent.

| Path | Operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner-installed R03 committed byte-identically, alone, in the issuance commit |
| `.github/workflows/rust-verify.yml` | Bounded Ubuntu fixture, host env routing, guaranteed cleanup; no existing gate altered (+134) |
| `crates/solo-adapters/src/source_read_native.rs` | Linux anonymous clone snapshot, typed mapping, restated guarantee, proof unit tests (+250) |
| `crates/solo-adapters/src/source_read.rs` | Snapshot call site and accurate guarantee docs (14 changed) |
| `crates/solo-adapters/tests/source_read.rs` | Fixture-root routing, product snapshot/refusal/cleanup/bound cases, Windows writer exclusion (+209) |
| `crates/solo-application/src/source_read.rs` | Global reservation link-count refusal only (14 changed) |
| `crates/solo-application/tests/source_read.rs` | Sole and unselected multi-link refusal cases (+60) |
| `.documents/Architecture/ARCHITECTURE.md` | §6A.8 `SourceRead` paragraph and its adjacent trust paragraph only (4 changed) |
| `.documents/Reports/execution-report.md` | This single canonical report |
| `crates/solo-adapters/tests/source_read_security.rs` | **OPTIONAL CREATE — remained absent** |

`README.md`, `AGENTS.md`, both SOPs, `ROADMAP`, `OUTLOOK`, `NEXT` (`92fa562b`), every manifest,
`Cargo.lock`, `deny.toml`, the toolchain file, the checker and unsafe-boundary sources, the accepted
manifest/locator/selection/preflight modules and the other architecture sections are untouched;
`plan-roadmap.md` (`18975332`) and `plan-outlook.md` (`cb9c7a61`) are byte-identical.

## 7. Cleanup and status

The working tree is empty, there is one checkout on `main` with one remote, and the WIP holds exactly
four regular files with no subdirectory. Every CI path the fixture created was removed by the
guaranteed step in every run, including the red ones; nothing was left mounted or attached. Locally,
`.temp/toolchain` (sanctioned, checksum-verified `1.99.1`) and `.temp/target` are retained with the
disclosed trigger *delete on acceptance or abandonment*; no Owner-owned or other-order temporary
material was touched, and no other checkout, worktree, clone or stash exists.

**This report claims technical evidence only.** It does not claim ACCEPT, does not close R02.B, does
not open R02.C and does not move `ROADMAP`. Its own run and SHA are reported to the Owner in chat,
and the Owner alone relays `agent report ready`.

