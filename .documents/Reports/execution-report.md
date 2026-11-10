# Execution Report

**Order:** `SOLO-WO-049 R01` — R02.B hybrid staging-security correction (Linux unnamed-root objects; Windows identity-bound native lifecycle).
**Disposition: `STOPPED` at the §4 feasibility gate, before any product source edit.** The gate requires *real-host* proof of both native designs and explicitly forbids substituting documentation or a green CI run for it. This runtime has no Rust toolchain on the Windows host and no Btrfs Linux host at all, so not one experiment could be executed. No product file was touched; `crates/solo-adapters/**` remains at its anchor blobs. The precise missing capability and the smallest Owner decisions are in §7.
**R04 prior disposition:** `STOP` (report blob `473d5668…` at the anchor), replaced in ACTIVE by this order, not technically accepted. R02.B remains **OPEN**.

## 1. §3 preflight — verified, no mismatch

Actual Solo root confirmed as the sole checkout; `git rev-parse --show-toplevel` resolves to it; branch `main`; `HEAD == origin/main == 9d9b67ec…` before the transition; tree `a4bb86b2…`; one worktree; sole remote `origin` → `provelopment/provelopment-foundation-solo`; exactly four regular `.documents/WIP/` files; working tree clean except the Owner-installed ACTIVE replacement. Required reading completed: `README.md`, `AGENTS.md`, both SOPs, pertinent Architecture sections, the four WIP files, the installed R01 order and the canonical STOP report.

Every frozen anchor blob matched exactly, including `workorder-active.md` R04 `06b911a3…`, `plan-next.md` `92fa562b…`, `plan-roadmap.md` `18975332…`, `plan-outlook.md` `cb9c7a61…`, `ARCHITECTURE.md` `250bf62e…`, `sop-architect.md` `1483c9be…`, `sop-implementation.md` `51bb3792…`, `AGENTS.md` `6eae1da8…`, `README.md` `ce2d1cf9…`, `rust-verify.yml` `eb78dad2…`, `Cargo.toml` `14505be4…`, `crates/solo-adapters/Cargo.toml` `a677df5f…`, `Cargo.lock` `8bd0bb00…`, `rust-toolchain.toml` `9ec55b3b…`, `deny.toml` `3063e864…`, `source_set_staging.rs` `e20054c7…`, `source_read_native.rs` `ab6a392e…`, `source_set_candidate.rs` `311d44c8…`, `lib.rs` `31bee760…`, `tests/source_set_staging.rs` `dd2b71f3…`, `tests/source_set_candidate.rs` `4c3cbad0…`, checker `0bad0e02…` and fixtures `7a663276…`. Report blob at anchor `473d5668…`, changed only by report-only ancestry as permitted. The installed local ACTIVE is this complete R01 order, verified from its `OWNER-ISSUED` header through `**End of SOLO-WO-049 R01.**`

## 2. ACTIVE transition (sequence step 1) — published and green

One ACTIVE-only transition commit, **`f519f97571c04466682f04fb96ad1d60c928f3a3`** (tree `50c7193a1d1b3e14c75f2c7933ee4e17e8ab5569`), replacing R04 with the Owner-installed R01 text and touching no other path. ROADMAP, NEXT and OUTLOOK blobs unchanged. Four WIP files, no archive, no second ACTIVE.

**CI at transition HEAD:** run **164**, job `verify (ubuntu-24.04)` **success** `129688362517`; job `verify (windows-2025)` **success** `129688362119`.


## 3. §4 feasibility gate — outcome `STOPPED`

The order is emphatic and correct: the platform mechanisms are "**Owner-selected design directions, not presupposed proven API behavior**", and proof must come from "safe, owned disposable experiments" on the real hosts; "Do not present Microsoft protocol/API documentation, a single CI success, or a test whose attack branch did not run as real-host proof." The gate is a hard prerequisite: "If **either** host cannot prove the complete required design, STOP before product changes."

**Windows host — no build capability.** The host has no Rust toolchain: `cargo` and `rustc` are not resolvable, `%USERPROFILE%\.cargo` and `%USERPROFILE%\.rustup` do not exist, no Rust installation is present in `Program Files`, `%LOCALAPPDATA%\Programs\Rust`, or a scoop shim directory, and `rustup`/`rustup-init` are absent. The checkout has never been built on this machine (`target/` does not exist) and no vendored crate sources exist (`%USERPROFILE%\.cargo\registry` absent). Consequently I could not compile or run the single disposable experiment the order authorises for this gate, and therefore could not prove any of the three behaviours the Windows half depends on:

1. component-by-component adoption of a drive-qualified absolute path with no intermediate reparse following;
2. `NtCreateFile` in the new `FILE_CREATE` form relative to a retained directory handle, returning ownership-bearing handles;
3. exact-by-handle `FileDispositionInformationEx` mark-for-delete, including `DELETE` access requirements, share-flag interaction, delete-pending visibility, non-empty-directory refusal, close ordering and identity preservation through rename/replacement.

**Binding-surface verification — attempted, inconclusive, reported as such.** §4 also demands that "Before implementing, verify the relevant `windows-sys 0.61.2` symbols, constants and structures exist within the exact pinned features." Without vendored sources or a build I could not do this locally, so I attempted a read-only symbol search of the pinned crate's generated sources and **calibrated it before trusting it**: a search for `SetFileInformationByHandle`, which the pinned feature `Win32_Storage_FileSystem` certainly carries, also returned zero hits, proving the code-search index does not cover those very large generated modules. This is reported as *unverified*, not as absent — no negative claim is made, and no undocumented struct layout, offset or hand-rolled declaration was invented to work around it. The frozen feature set (`Wdk_Foundation`, `Wdk_Storage_FileSystem`, `Win32_Foundation`, `Win32_Security`, `Win32_Storage_FileSystem`, `Win32_System_IO`, `Win32_System_Memory`) is unchanged; `NtCreateFile` is already imported and exercised at `source_read_native.rs:775`, so the residual risk concentrates on the disposition class and its `FILE_DISPOSITION_INFO_EX`-shaped structure.

**Linux host — the required evidence cannot be produced.** The only Linux reachable from this runtime is a WSL2 distribution, kernel `6.18.58.1-microsoft-standard-WSL2`, whose root filesystem is **ext4**; Btrfs is not present, and it carries no `cargo` or `rustc`. Two independent obstacles follow: (a) **H3** requires "**Actual Btrfs root** creates and reads `O_TMPFILE` shards"; (b) the GitHub `ubuntu-24.04` verification runner is likewise ext4 and, more fundamentally, executes only *committed* sources. Solo `.temp/` is excluded from Git, and the ledger forbids a ninth permanent path, a committed throwaway module and a deliberately red intermediate `main`, so CI cannot legally host the disposable experiments this gate requires.

**Container path — deliberately not taken.** Docker Desktop and WSL are present, but pulling a Rust image or installing a toolchain is a new toolchain/package installation plus additional network use, which the order's permissions clause expressly withholds ("No new dependency/toolchain/feature/lockfile/package installation"; "Approved network only for required existing GitHub/Cargo/advisory and verification behavior"). A container image also would not be the pinned toolchain and still could not perform the Windows proofs.

**Gate result: zero experiments executed, and none claimed.** Per the gate's own outcome rule and §7.4 ("If an important host proof fails ... STOP immediately"), the run stops before product changes. I make no host-semantics claim for either platform.

## 4. What was deliberately not done

- **No product source change.** `source_set_staging.rs`, `source_read_native.rs`, both test files, `source_set_candidate.rs` and `lib.rs` are byte-identical to the anchor blobs in §1. The Linux no-namespace rewrite and the Windows `FILE_CREATE`/by-handle disposition work would both have been written on unproven host semantics, which the order forbids ("Do not ship a half-proved cross-platform implementation").
- **No speculative native code, offsets or hand-rolled declarations** for the disposition information class.
- **No test weakening, deletion, skipping or renamed adversary.** No adversary was added either, because adding one that cannot execute on the available hosts would be theatre.
- **No checker, workflow, `Cargo.toml`, `Cargo.lock`, `deny.toml`, toolchain or feature change**; no ninth permanent path; no new native module; no branch, PR or second remote.
- **No toolchain installation and no container pull**, per §3.
- **No partial publication of one host's half** as a security improvement.

## 5. Cleanup and material honesty

- Repository: `git status --porcelain` was empty immediately after the transition commit, exactly four WIP files, one worktree, one remote, `HEAD == origin/main`.
- Solo `.temp/` was inspected read-only and already contained **2813 pre-existing entries** before this run. Its ownership is unverified and it is neither task-owned by this run nor safe to delete; I did not touch it. **No file was created by this run in `.temp/` or anywhere else** except the two ledger files named in §2 and §6.
- No build artifacts: `target/` does not exist and was not created. No permanent log, no archived report, no duplicate WIP file.
- The Cline task scratch referenced in §3 is host-temporary and outside the repository; nothing durable is claimed from it.

## 6. Acceptance gates H1–H10

No implementation exists in this run, so every gate that depends on the new design is `UNVERIFIED` by construction rather than by failure. `REAL` = executed on a real host by the verification pipeline; `INJECTED` = production-path fault seam; `PURE` = static inspection only.

| Gate | Status | Basis |
| --- | --- | --- |
| **H1** source policy / no regression | **PASS (unchanged code)** — `REAL` (run 164, both hosts) + `INJECTED` (existing seams) | The full suite ran unchanged on Ubuntu and Windows at the transition HEAD; no M1–M14/F1/F3/F4/F6 surface was edited, so their previous evidence stands verbatim |
| **H2** Linux root confinement | **UNVERIFIED** — not executed | No Rust toolchain and no Btrfs Linux host; no implementation, therefore no new adversary. No claim made |
| **H3** Linux zero-name lifecycle | **UNVERIFIED** — not executed | H3 names an actual Btrfs root; the reachable Linux host is ext4/WSL2 with no toolchain, and CI runs only committed code |
| **H4** Windows component/root confinement | **UNVERIFIED** — not executed | No compiler on the Windows host; component adoption, relative `FILE_CREATE` and reparse refusal could not be exercised |
| **H5** Windows immutable staged bytes | **PARTIAL** — existing accepted model `PASS` (unchanged, run 164); new by-handle detach `UNVERIFIED` | Share-mode/no-alias behaviour at the anchor is still green; the new detach/disposition mechanism was not built or run |
| **H6** owned-only cleanup / atomicity | **UNVERIFIED** — not executed, and **defect open** | The R04 unowned-empty-directory deletion race is neither closed nor newly proven; no empty/populated impostor adversary added |
| **H7** error classification / lifetime | **PASS (unchanged)** for injected write/short-write/permission/flush/finalize; abrupt termination remains **UNVERIFIED** as before | Existing production-path seams unchanged and green; no new claim about process death |
| **H8** limited native authority | **PASS** — `PURE` | Zero new/modified `unsafe` or FFI; only report and the pre-existing ACTIVE text changed; features, lockfile, checker and workflow untouched |
| **H9** two-host A1/security evidence | **PARTIAL** — green at transition HEAD (run 164) and at the report HEAD; **no substantive implementation HEAD exists** | There is no implementation to verify, so the gate's primary requirement cannot be met in this run |
| **H10** repository / governance / report | **PASS** for what this run controls | One ACTIVE-only transition (`f519f97`), four WIP files, frozen NEXT/ROADMAP/OUTLOOK blobs, one checkout/one remote, clean tree, single canonical report, task scratch not polluted, no self-ACCEPT, R02.B OPEN, R02.C unstarted |

**R04 F1–F8 crosswalk (preserved, not re-scored):** F1 `PASS` (unchanged, green); **F2 `NOT ACCEPTED`** — root/adoption/creation redirection defects stand exactly as the R04 STOP recorded them; F3, F4, F6 `PASS` (unchanged); **F5 `NOT ACCEPTED`** — unowned-object deletion race stands; F7 `PASS`; F8 `PASS` for the code of record. No gate was weakened, renamed to pass, or converted into a skip.

## 7. The precise missing capability, and the smallest Owner decisions

This is **not** a reopening of the R04 design disagreement. The design question is settled: this order narrows the native authority to component-relative `FILE_CREATE` and by-handle `FileDispositionInformationEx`, and the Linux half needs no native authority at all. The blocker is purely that the mandated proof venue does not exist on the runtime I was given. Three decisions, any subset of which unblocks work:

1. **A build-capable Windows host with the pinned `1.99.1` toolchain**, or explicit authorization to install exactly that pinned toolchain on this Windows host (a machine change plus network use, which the permissions clause withholds). Without it, the three Windows host proofs in §3 cannot exist, and the order forbids inference from documentation or from CI.
2. **A Linux host whose staging root is Btrfs**, or an explicit amendment replacing H3's "actual Btrfs root" with ext4 evidence plus a disclosed Btrfs limitation. No available host satisfies the literal gate: WSL2 here is ext4 and GitHub's `ubuntu-24.04` runner is ext4.
3. **Read-only access to the pinned `windows-sys 0.61.2` sources for the symbol audit** — enough to confirm `SetFileInformationByHandle`, the `FILE_INFO_BY_HANDLE_CLASS::FileDispositionInformationEx` value and the `FILE_DISPOSITION_INFO_EX` structure exist under the frozen feature set, with no lockfile or feature change. If any is absent, the Windows half needs a further amendment to the feature list, which this order does not grant and which I will not take unilaterally.

With (1) and (2) granted, the hybrid design in §1/§5 is implementable inside the eight-path ledger without further authority, and the adversary set demanded by H2/H4/H6 is executable on real hosts. I did not spend this run converting an unavailable attack into a passing test, and I did not leave a half-fix in `main`.

## 8. State and disposition

- ACTIVE holds `SOLO-WO-049 R01` (transition `f519f97…`, published and green). NEXT neutral `92fa562b…`, ROADMAP `18975332…` and OUTLOOK `cb9c7a61…` unchanged and still carrying R02.B.
- **R02.B remains OPEN; R02.C remains unstarted.** No closure, merge, release, deployment, PR, second remote or self-ACCEPT.
- Working tree clean after the report-only commit; one checkout, one worktree, four WIP files, `HEAD == origin/main` verified after publication.

**Disposition: `STOPPED`** — at the mandatory feasibility gate, before product code, for the exact missing capabilities in §7. Nothing in this report is offered as evidence for F2 or F5, and no host-semantics claim is made.
