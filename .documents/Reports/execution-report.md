# Execution Report

**Order:** `SOLO-WO-047 R02` (Owner-issued, unchanged by this run).
**Run kind:** implementation revision under Chief Architect disposition `REVISE`, plus the standing report permission.
**Implementation HEAD:** `ec4335d9f9cd2e7d7269a64d61b83abf5a6b47b1`
**Implementation tree:** `97855bd70362a77f899ccd1c7efff9ac22d81dd1`
**Workflow:** `rust-verify` run `42596833689` (run 98) — **success**
&nbsp;&nbsp;· Ubuntu job `129091422335` **success** (09:53:38 → 09:54:39)
&nbsp;&nbsp;· Windows job `129091422290` **success** (09:53:36 → 09:55:23)
**Report HEAD:** the separately committed report commit whose SHA is given in chat, never inside this file.

---

## 1. Outcome of this pass

* **A — root-capability trust boundary: implemented and verified.** The reader now accepts a
  **trusted, already-opened directory handle** and verifies it *on the handle*, not on a path.
* **B — physical ambiguity in all declared candidates: implemented and verified.** A located
  candidate with more than one directory entry fails closed even when it is the unselected member,
  and a changed link count between probe and read is refused.
* **C — the §4 requirement-to-test ledger is §4 below**, with every mandatory case mapped to a named
  test, the class of evidence it actually provides, and its observed result.
* **D — the in-place mutation guarantee is resolved for Windows and STOPPED for Linux** with the
  smallest Owner architectural decision (§5). R02 §4 has **not** been weakened; the Linux
  no-torn-read guarantee is **not** claimed.

## 2. Corrections in this pass

**A. Handle transfer, not path-plus-enum.** `ContentRootCapability::from_trusted_directory(handle,
provenance)` takes ownership of `TrustedDirectoryHandle` (`std::os::fd::OwnedFd` on Unix,
`std::os::windows::io::OwnedHandle` on Windows) and reads the object's **own** type, reparse state and
identity from that handle: Linux uses `fstat` and refuses anything that is not a directory; Windows uses
`GetFileInformationByHandleEx(FileAttributeTagInfo/FileIdInfo/FileStandardInfo/FileBasicInfo)` and
refuses a non-directory or any non-zero reparse tag. No path is resolved in that route. The previous
constructor is retained only as `adopt_bootstrap_path`, documented as the operator bootstrap that opens
the designated directory **once** — it is no longer described as proof that a handle was supplied.

**B. Ambiguity belongs to the candidate, not the selection.** `read_declared_source_v1` refuses
`IdentityCollision` when **either** located probe reports more than one directory entry, before anything
is read, so an unselected Markdown cannot hide behind a valid JSON body and neither body is opened to
find out. Probe-to-read consistency compares identity, length, change evidence **and link count**, so a
link count that changes after the probe is refused; a shared object is decided as a collision before any
change comparison.

**D (Windows half).** Handles opened to read a body request `ShareAccess = FILE_SHARE_READ` only: while
the reader holds the handle no other opener can obtain write or delete access, so the object cannot be
modified or replaced under it. That is an auditable **open policy enforced by the I/O manager**, not a
second timestamp comparison. Directory handles keep full sharing because they are only traversed.

## 3. Evidence

Local gates (pinned `1.99.1`) are green for both `x86_64-pc-windows-gnu` and
`x86_64-unknown-linux-gnu`: `fmt --check`, `check --workspace --all-targets --locked` and
`clippy --workspace --all-targets --locked -- -D warnings`. Local test execution remains impossible
here (no linker/assembler for any target), so every host-executed result is from the workflow.

Full mandatory sequence green on both hosts at `ec4335d`: locked fetch, `cargo-deny 0.20.2` gate with a
fresh RustSec refresh, advisory negative probe, package-set gates, `fmt`, strict `clippy`, the whole test
step (including every test named in §4), the real-tree architecture checker
(`architecture=legal packages=4 … edges=40`), the refusal self-test (`self-check=passed cases=29`), both
test-failure tripwires, the byte-reproducible release build, the release CLI smoke and the forbidden-import
refusal with restore.

## 4. R02 §4 requirement-to-test ledger

Every row below is a test that **executed and passed on both hosts** in run 98 at `ec4335d`, unless the
row's class says otherwise. `native` = real filesystem/kernel behaviour on that host; `fake` = the
application-level fake port, which proves the decision but not the host; `injected` = a deliberately
injected refusal or substitution, which proves the classification, never native behaviour.

| # | Requirement | Test | Class | Result |
| --- | --- | --- | --- | --- |
| 1 | Declared JSON leaf read with its bytes and identity | `a_declared_json_leaf_is_read_with_its_bytes_and_identity` | native | pass |
| 2 | Markdown used only after confirmed JSON absence | `a_declared_markdown_leaf_is_read_when_json_is_confirmed_absent`, `confirmed_json_absence_permits_markdown` | native, fake | pass |
| 3 | Oversized JSON refused, never truncated | `an_oversized_json_is_refused_rather_than_truncated` | native | pass |
| 4 | Unselected Markdown never read while JSON is available | `an_oversized_markdown_is_never_read_when_json_exists`, `json_present_is_chosen_and_markdown_is_never_read` | native, fake | pass |
| 5 | Empty leaf is present with no bytes | `an_empty_json_leaf_is_present_with_no_bytes` | native | pass |
| 6 | Leaf symlink refused | `a_leaf_that_is_a_link_is_refused` | native | pass |
| 7 | Intermediate reparse/symlink refused | `an_intermediate_directory_that_is_a_link_is_refused` (junction on Windows, symlink on Linux) | native | pass |
| 8 | Root reparse/symlink refused | `a_root_that_is_a_link_is_refused` | native | pass |
| 9 | Directory in place of a leaf refused | `a_directory_in_place_of_a_leaf_is_refused` | native | pass |
| 10 | Missing intermediate is absence, not an error | `a_missing_intermediate_directory_is_a_confirmed_absence` | native | pass |
| 11 | Error is never converted into absence | `a_probe_refusal_is_final_and_never_falls_back`, `an_injected_unsupported_answer_is_refused_and_never_becomes_absence` | fake, injected | pass |
| 12 | Hardlink within one representation refused | `a_leaf_with_a_second_directory_entry_is_refused`, `a_hardlinked_body_is_refused_rather_than_shared` | native, fake | pass |
| 13 | Hardlink created from an unrelated directory entry refused | `a_leaf_hardlinked_from_an_unrelated_directory_is_refused` | native | pass |
| 14 | Unselected ambiguous candidate refused, winner's body not read | `an_unselected_hardlinked_markdown_is_refused_while_json_would_have_won`, `an_unselected_hardlinked_candidate_is_refused_before_any_body_is_read` | native, fake | pass |
| 15 | Link count changing between probe and read refused | `a_link_count_that_changes_between_probe_and_read_is_refused` | injected | pass |
| 16 | Two members resolving to one object refused | `two_declared_members_that_are_one_object_are_refused` | native | pass |
| 17 | Identity reserved across distinct representations | `two_representations_resolving_to_one_physical_object_are_refused`, `distinct_representations_reserve_their_own_identities` | fake | pass |
| 18 | Leaf replaced by another object between probe and read refused | `a_leaf_replaced_by_another_object_between_probe_and_read_is_refused` | native | pass |
| 19 | Probe/read disagreement on identity, length or change evidence refused | `a_probe_to_read_replacement_that_keeps_identity_and_length_is_refused`, `a_read_of_a_different_object_than_the_probe_located_is_refused`, `a_read_whose_length_disagrees_with_the_probe_is_refused` | injected | pass |
| 20 | A body vanishing after its probe is a change, not an absence | `a_body_that_vanishes_after_its_probe_is_a_change_not_an_absence` | injected | pass |
| 21 | A rewrite between separate reads is observed, never served from memory | `a_rewrite_between_reads_is_observed_and_never_served_from_memory` | native | pass |
| 22 | Trusted, already-opened directory handle accepted and verified | `a_transferred_directory_handle_is_verified_and_reads_correctly` | native | pass |
| 23 | Handle that names a regular file refused | `a_transferred_handle_to_a_regular_file_is_refused` | native | pass |
| 24 | Handle that names a reparse point refused | `a_transferred_handle_to_a_junction_is_refused` | native (Windows only) | pass |
| 25 | Absent root refused without being read as empty | `an_absent_root_is_refused` | native | pass |
| 26 | No outcome claims publication or snapshot authority | `no_read_outcome_can_claim_publication_or_snapshot_authority` | fake | pass |
| 27 | Diagnostics stay bounded and carry no location | `diagnostics_stay_bounded_and_carry_no_location` | fake | pass |
| 28 | Fingerprinting separates content and is not identity | `the_fingerprint_separates_content_and_ignores_identity` | fake | pass |
| 29 | Unsupported API/kernel answer refused, never a fallback | `an_injected_unsupported_answer_is_refused_and_never_becomes_absence` | injected — **native `ENOSYS` unverified** (§5) | pass (injected) |
| 30 | Bytes cannot be torn while they are transferred | Windows: the read handle denies write/delete sharing (§2 D); Linux: **not guaranteed** | policy, not a race test — see §5 | §5 |

Rows 15, 19 and 20 have no native counterpart that can be produced deterministically on both hosts, and
row 29 cannot be produced natively at all without a syscall interposer; they are labelled injected for
exactly that reason. No simulated case is presented as a native-filesystem proof.


## 5. D — the in-place mutation guarantee, and the one decision needed

**Windows has the guarantee, by open policy.** A body handle requests `ShareAccess = FILE_SHARE_READ`.
While it is open no other opener can obtain write or delete access, so the object cannot be modified or
replaced underneath the transfer; a writer that already holds a write handle makes our open fail, and
that failure is a refusal rather than a silent read. This is enforcement by the I/O manager and is
auditable in one call (`NtCreateFile` share argument), not an inference from two timestamp samples.

**Linux does not have it, and cannot have it under the approved design.** Every available mechanism was
assessed and rejected for cause:

* `fcntl`/`flock` locks are **advisory** — a writer that does not take the lock ignores it, so they
  cannot make a read atomic;
* mandatory locking (`MS_MANDLOCK`) is deprecated, requires mount-level configuration, and is not
  present on the hosted runner or on ordinary ext4 — it cannot be a product guarantee;
* `openat2` with `RESOLVE_*`, `O_NOFOLLOW` and `O_CLOEXEC` govern **resolution**, not concurrent
  modification, and the existing `RESOLVE_NO_XDEV|NO_SYMLINKS|NO_MAGICLINKS|BENEATH` policy is about
  where the bytes come from, not whether they change during the transfer;
* a real immutable snapshot needs a snapshot or reflink primitive (`FICLONE`, a btrfs subvolume
  snapshot, an LVM/ZFS snapshot) — filesystem-specific, unavailable on the ext4 runner, and it would
  introduce a new dependency and a new host-capability requirement that this order does not authorize;
* `mmap(MAP_PRIVATE)` gives copy-on-write pages but the kernel still faults in pages that a writer may
  have modified, so it is not a snapshot either.

Consequently the residual is unchanged and is **not** presented as solved: on Linux an in-place,
same-size rewrite that lands inside one filesystem timestamp tick can leave identity, length, link
count and change evidence identical, so detection cannot be guaranteed. Two comparisons and a
fingerprint are not atomic-read evidence, and the report does not use them as such.

**STOP for the smallest Owner architectural decision.** Exactly one of these is required before §4 can
be claimed on Linux:

* **D1 (smallest):** accept detect-and-refuse as the contracted Linux behaviour of `SourceRead`, with
  the residual above stated as a permanent, documented limitation of the accepted design; or
* **D2:** require a snapshot-capable filesystem and adopt an explicit snapshot primitive, adding the
  host-capability probe, the refusal path for hosts without it, and the dependency that this implies; or
* **D3:** require an operator-guaranteed exclusive-access discipline on the content root, making the
  existing detection authoritative and the residual a configuration violation.

No code change in this run presumes any of the three, and none weakens the accepted §4 wording.

## 6. Preserved state, ledger and cleanup

`ACTIVE` was not edited (order still issued and unchanged, blob `4cb50914`); `NEXT` neutral
(`92fa562b`); `ROADMAP` untouched (`18975332`); previously accepted modules byte-identical; the earlier
R01 STOP and this order's earlier R02 STOP remain in history. R02.B, R02.C and Stage 02 stay open, and no
administrative closure was begun. No dependency, manifest, `Cargo.lock`, `deny.toml` or toolchain change
was made in this pass.

Paths touched in this run, all inside the R02 21-path authorization ledger:
`crates/solo-adapters/src/source_read.rs`, `crates/solo-adapters/src/source_read_native.rs`,
`crates/solo-adapters/src/lib.rs`, `crates/solo-adapters/tests/source_read.rs`,
`crates/solo-application/src/source_read.rs`, `crates/solo-application/tests/source_read.rs`, plus the
earlier corrections in `crates/solo-cli/tests/architecture_checks/source.rs`,
`crates/solo-cli/tests/workspace_graph.rs`, `crates/solo-cli/tests/unsafe_boundary.rs`,
`crates/solo-cli/tests/check_architecture_fixtures.rs`, `.github/workflows/rust-verify.yml` and this
report. The authority flags remain `false`: `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT = false` and
`SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY = false`.

**Cleanup.** `.temp/toolchain` (sanctioned, checksum-verified `1.99.1`) and `.temp/target` (cache) are
retained with the trigger *delete on acceptance or abandonment*; the four older `.temp/` folders and
`.temp/architecture-fixtures` are Owner/earlier-run material and were not touched; the untracked 781.6 MB
`target/` from an earlier run stayed removed; the working tree is clean.

**Standing limitations.** Job logs remain `403` from this environment, so all results above come from
run/job status, check-run annotations and step conclusions. Local test execution is impossible here.

