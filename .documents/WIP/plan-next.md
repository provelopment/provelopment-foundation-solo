# Foundation Solo — plan-next (Next Candidate)

**Status: candidate only.** This file is planning reference. It is not an instruction, it authorizes nothing, and no agent may execute it. Only a complete `OWNER-ISSUED` `workorder-active.md` with a matching Owner handoff can authorize work, and only the Owner installs one.

**Drawn from:** ROADMAP **Stage 02 → R02.B**, the work still open after the three accepted bounded slices. **R02.C is not selected and stays unopened** until R02.B is fully closed.

## Candidate — R02.B whole-set validation and the complete immutable candidate

**Why this is the remaining work.** The accepted slices delivered the validated manifest declaration with provenance-safe locators, the JSON-first declared selection with per-file body preflight, and the native immutable single-body read now proven by host execution on `windows-2025` (a surviving writable mapping forces a sharing refusal, never altered bytes) and on Linux (the D2 `FICLONE` snapshot). What R02.B still lacks is the *whole* candidate: every declared member read and validated, content validated rather than merely carried, and the set resolved as one immutable candidate.

**Bounded scope**

1. Read and validate **every** member of a declared candidate set through the existing `SourceRead` port, not only the single selected body, keeping each member's pinned per-file guarantee intact.
2. Validate each declared member's **content** — declared kind honoured, malformed, empty, truncated, duplicated or mismatched members refused with their typed error — with no partial acceptance and no member silently skipped.
3. Assemble the complete immutable candidate: one result that is either the whole validated set with per-member provenance retained, or a typed refusal. No partial-set fallback, no generated-over-authored substitution, no sibling fallback.
4. Keep the honest authority boundary: a per-file result is not a global snapshot, so `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` stay `false` unless a later order proves and an Architect accepts otherwise; declaration never becomes publication.
5. Preserve the accepted behaviour byte-for-byte: Windows `ShareAccess = FILE_SHARE_READ` refusal, the Linux D2 anonymous clone and its refusals, every typed error, the architecture and `unsafe` boundaries, and the four-crate locked graph. No new crate, version, feature, provider, framework or permanent fixture.

**Prerequisites**

- Independent Architect `ACCEPT` of the third bounded R02.B slice at report HEAD `3085af8634b930d946d8a5bccf056b88a98bfb52` — **issued**.
- Completion of the same-order R04 administrative closure that installs this file, neutralizes ACTIVE and publishes the reconciled final report.
- A fresh, separately Owner-issued work order naming this one candidate, with its exact starting HEAD/tree, frozen path ledger and matching handoff.

**Evidence and gates this candidate would owe**

- Positive whole-set reads plus negative cases for missing, undeclared, duplicated, ambiguous, malformed and truncated members, proving refusal rather than partial acceptance.
- Fully green `rust-verify` on both `ubuntu-24.04` and `windows-2025`: locked fetch, verified `cargo-deny`, format, strict Clippy on both targets, all-target tests, the real-tree architecture checker with its refusal fixtures, release build and CLI smoke.
- A canonical report at report HEAD, independently audited by the Architect before any closure. No test deleted, weakened, ignored or conditionally skipped.

**Explicitly outside this candidate:** R02.C, Stage 02 exit, routing/localization/visibility, publication of any kind, release, deployment, provider or secret access, any WIP roadmap edit, and any relaxation of the immutable-read, ownership or architecture contracts.
