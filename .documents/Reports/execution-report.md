# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-074 R01` — R03.C3: bind the second, exact-path process-native exception and prove fail-closed architecture/unsafe checkers (documentation and test-guard slice only).
**Matching handoff:** `SOLO-WO-074 R01`. **Report date (ISO):** 2026-11-12 (run window 2026-11-12T08:40–09:40Z).

**Outcome: `NATIVE-SCOPE-GUARDS-PROVEN`.** The Charter now records exactly one reserved future process-native source path, and both independent guards — the `syn` AST checker and the first-party `unsafe`-boundary scanner — enforce two *different* path/OS rules with positive-`cfg` semantics, proved by negative and positive tests that actually executed on `ubuntu-24.04` and `windows-2025`.

No native module exists, no `unsafe` was added, no `libc` Cargo edge was created, and no dependency, lockfile, policy, toolchain, workflow, product crate, CLI or media module changed. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. Nothing here is `HOST-CONTAINMENT-PROVEN` or `MEDIA-VERIFIED`.

## 1. Preflight and issuance — verified

| Item | Verified value |
| --- | --- |
| `main == origin/main == HEAD` at start | `a6905c243e618c63039949b281710cec2ac0c8e4`, tree `a1df91166a6457c01970e426b2187323811b96e9` |
| Starting blobs | report `ee95501bf4ffb92e5dfc17d20ad2c859540e7cdd`; Architecture `88b16fd91ffe4618801b4f956340674c27c7e047`; NEXT `92fa562b…`; ROADMAP `9d7a8622…`; OUTLOOK `cb9c7a61…`; `Cargo.lock` `2f9e53576df117f2020ae064d6973304f20833f1`; AST checker `76abff5773bbe9af05293e04b3089f807d767886`; `unsafe_boundary.rs` `9088c2eea742095df3299c3fbafa3a8ce782e18c`; fixtures `7a66327693757f7c369a24933e2a1c021b61df19` |
| Previous ACTIVE blob | `ebd56fe0e5c0fe4c233f2b7f8163fa4fd7a3ec59`; installed ACTIVE `38c32b488a9f449e98bc65d0d6caa6c0271c44eb` — complete `OWNER-ISSUED — SOLO-WO-074 R01` contents, matching handoff at line 3 |
| Repository shape | one checkout/worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, clean `main`, four WIP files, branches `main` + `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` unchanged |
| Inherited scratch at start | `.temp/solo-wo-073-wip/` present and untouched (4 files, 46,062 bytes) |
| Issuance commit | `36688d5eeb7069a2f4a568f195a6f0d482bdfe58` · tree `eeda0e884eaa01d7b63ce135a81955ad3f2f4872` · parent `a6905c24…` · ACTIVE blob `38c32b48…` · **one path only** |
| Issuance CI | `rust-verify` run **43185518897** — jobs `131071587202` (ubuntu-24.04) and `131071586700` (windows-2025): **success** |

`S074-01` ✓ and `S074-02` ✓.

## 2. The Charter insertion — byte-proved

The one paragraph from §5A was inserted immediately after §6A.8 rule 6 and before `Single portable sealed artefact.`:

* Architecture commit `3fe6b1b961c0ae60abd8541a73b97dc7516d94c6` · tree `1edd953d8000b05cac97ef1f2cf46a9aa8e976c2` · parent `36688d5e…` · **one path** · blob `88b16fd9…` → `c941c7f90cc1b1955f9d60b348a3b1092cafe97f`.
* `git diff --numstat` reports **`2 0`** — exactly two inserted lines, none removed or altered.
* **Insertion-only proof executed:** removing the exact inserted span (the paragraph line plus its following blank line) from the new file and hashing the result with `git hash-object -t blob --no-filters` reproduces **`88b16fd91ffe4618801b4f956340674c27c7e047`**, the pre-change Architecture blob, exactly. No other byte, ordering, newline convention (the file is LF, no BOM) or EOF changed.
* CI at that commit: `rust-verify` run **43188095777** — jobs `131080245685` (ubuntu-24.04) and `131080245373` (windows-2025): **success**.

`S074-03` ✓.

## 3. Changed-path ledger and commit chain

| Commit | Message | Paths | Tree |
| --- | --- | --- | --- |
| `36688d5e` | `Issue SOLO-WO-074 R01: native checker reservation` | `.documents/WIP/workorder-active.md` | `eeda0e88` |
| `3fe6b1b9` | `architecture: reserve exact AVIF process-native boundary (SOLO-WO-074)` | `.documents/Architecture/ARCHITECTURE.md` | `1edd953d` |
| `fff1523d` | `test: enforce exact two-file native scope with OS gates (SOLO-WO-074)` | `crates/solo-cli/tests/architecture_checks/source.rs`, `crates/solo-cli/tests/unsafe_boundary.rs`, `crates/solo-cli/tests/check_architecture_fixtures.rs` | `ecc180ab` |
| `3761d351` | `test: correct the ambiguous-all gate expectations (SOLO-WO-074)` | `crates/solo-cli/tests/check_architecture_fixtures.rs` | `501784fc` |
| `577cc832` | `test: refuse an ungated allowance at every path and track dlsym (SOLO-WO-074)` | `crates/solo-cli/tests/unsafe_boundary.rs` | `1a91bd4b` |
| report-only commit | `Report: SOLO-WO-074 R01 native scope guards` | `.documents/Reports/execution-report.md` | — |

Checker blobs: `source.rs` `76abff57…` → `df1f8758b201214e5a4066823e83449f9c6cef6f`; `check_architecture_fixtures.rs` `7a663276…` → `da76dd67…` → `dd8bcfe2a0c99acbe9ae5b9a6aeccb4993156ad2`; `unsafe_boundary.rs` `9088c2ee…` → `5cb0f0ee…` → `5c59a71bf4bf31808dde45be18fa123ba3fb7c44`.

Unchanged, byte-identical to the starting HEAD: `crates/solo-adapters/src/lib.rs`, `crates/solo-adapters/src/source_read_native.rs`, `crates/solo-cli/tests/architecture_checks/{mod.rs,graph.rs}`, `crates/solo-cli/tests/workspace_graph.rs`, every `Cargo.toml` and `Cargo.lock` (`2f9e5357…`), `deny.toml`, `rust-toolchain.toml`, every workflow, the production CLI, and every application/domain/media-worker module. **No file was created**, and `crates/solo-adapters/src/media_worker_host_native.rs` does not exist.

## 4. What the checker now enforces, and the evidence that it does

### 4A. The single verdict, extended to two paths (`architecture_checks/source.rs`)

One decision function, `unsafe_scope_verdict(location, unscoped, windows_scoped, linux_scoped)`, is applied to the real tree and to every synthetic fixture:

* **`crates/solo-adapters/src/source_read_native.rs`** keeps its original rule — positive Windows-scoped `unsafe` only. Linux-scoped `unsafe` there is refused with its own stable reason (`… is the Windows-only source-read boundary …`), which is what stops a new gate from silently widening an approved exception.
* **`crates/solo-adapters/src/media_worker_host_native.rs`** (reserved, absent) admits positively Windows- or Linux-scoped `unsafe` and nothing else.
* **Every other path**, including tests, the safe façade, the CLI and inner crates, still refuses any scoped or unscoped `unsafe`.

Path identity is the repository-relative path with `\` normalised to `/`, so a Windows spelling cannot dodge the comparison in either direction (proved by two fixtures in `G074-05`).

### 4B. Positive-gate semantics, parsed rather than substring-matched

`cfg_os_gate` / `meta_os_gate` parse each `#[cfg(..)]` predicate through `syn` into a platform set, and gates compose by intersection:

| Predicate | Set | Result |
| --- | --- | --- |
| `windows` | `{windows}` | positive Windows gate |
| `target_os = "linux"` | `{linux}` | positive Linux gate |
| `all(test, windows)` | `{windows}` | positive Windows gate — the real source-read module depends on this |
| `all(test, target_os = "linux")` | `{linux}` | positive Linux gate |
| `all(windows, all(test, windows))` | `{windows}` | positive Windows gate |
| `all(any(unix, windows), target_os = "linux")` | ∅ | not a proof |
| `all(windows, target_os = "linux")` | ∅ | not a proof |
| `any(windows, feature = "x")`, `any(target_os = "linux", feature = "x")`, `any(windows, unix)` | ∅ | not a proof |
| `not(windows)`, `not(test)` | ∅ | not a proof |
| `target_os = "macos"`, `target_family = "windows"` | ∅ | not a proof |
| `cfg_attr(windows, allow(dead_code))` | identity | never establishes a gate |
| `all()` (malformed/empty) | ∅ | not a proof |
| no `cfg`, `test`, `unix`, `feature = "x"`, `target_arch = ".."` | `{windows, linux}` | no restriction stated, so `unsafe` is unscoped and refused |

The empty set and the unrestricted set are deliberately different: the former means "no platform is guaranteed", the latter "no restriction stated", and **both** refuse `unsafe`. A doubly gated scope (a Windows gate nested inside a Linux gate, which can never both hold) also refuses.

### 4C. The reserved `libc` rule, and what it is not

`libc` remains a `DYNAMIC_LOADING_ROOTS` member for ordinary source. The single reserved exception is a *path-and-gate* condition inside `classify_segments`: `libc` is permitted only when the path is exactly the reserved process file **and** the occurrence sits in a positive Linux gate. No crate declares `libc`, so this branch is reachable only from an in-memory synthetic source — it is a tested checker rule, not a Cargo permission. `libloading`, `dlopen`, `dlsym`, `libdl`, `LoadLibrary`, aliases, macro indirection, `#[path]` escapes and `include!` are untouched and still refused.

### 4D. The independent boundary guard (`unsafe_boundary.rs`)

The scanner keeps its code-only tokenisation and now:

* recognises exactly two roles (`SourceRead`, `ProcessNative`) and refuses the keyword everywhere else;
* delegates the gate question to `architecture_checks::source::classify_unsafe_scope` — the **same** AST rule, so the two guards cannot drift into two policies, while the keyword/native-root scan stays independent;
* refuses an ungated `allow(unsafe_code)` at **every** path (a genuine hole the negative tests exposed, see §6);
* fixes each exception's native roots: `windows_sys`/`rustix` for the source reader, `windows_sys`/`libc` for the reserved process path, none anywhere else, with all five dynamic-loading roots refused everywhere;
* still requires one `// SAFETY:` note per `unsafe` block in an eligible file, and still proves the real tree contains the pre-existing module only — asserting that the reserved file is **absent** and that the adapter root mentions it **nowhere**, so no allowance was pre-installed for a file that does not exist.

## 5. The proof matrix — every case executed on both hosts

The two targets now hold **30** tests (`check_architecture_fixtures`, 25 pre-existing + 5 new) and **14** tests (`unsafe_boundary`, 8 pre-existing + 6 new); all of them ran and passed in the green run.

| ID | Executed proof | Result |
| --- | --- | --- |
| `G074-01` | Real-tree source/dependency checkers over all four packages; the Windows source-read module still valid and byte-unchanged | **PASS** (real-tree test + workspace graph tests) |
| `G074-02` | Source-read path accepts `#[cfg(windows)]`; refuses Linux, negated-Windows, negated-Linux, `target_os="macos"` and unscoped; reason names the Windows-only boundary | **PASS** (`g074_02_…`, `the_checker_allows_…`, `the_checker_refuses_unsafe_outside_…`) |
| `G074-03` | Reserved path accepts positive Linux, positive Windows, `all(test, windows)`, `all(test, target_os="linux")`, `all(windows, all(test, windows))`; refuses unscoped and a file with one ungated `unsafe fn` | **PASS** (`g074_03_…`, `the_guard_accepts_a_positively_gated_reserved_process_module`) |
| `G074-04` | `any(windows, feature)`, `any(target_os="linux", feature)`, `any(windows, unix)`, `not(test)`, `not(windows)`, `target_os="macos"`, `target_family="windows"`, `cfg_attr`-only, `all()`, `all(any(...), target_os="linux")`, `all(windows, target_os="linux")` are **not** OS proofs, at both paths | **PASS** (`g074_04_…`, `the_guard_refuses_an_unproven_gate_…`) |
| `G074-05` | Unscoped `unsafe`, ten ineligible path spellings (domain, application, façade, `lib.rs`, CLI, test file, copy, `.bak`, sibling `_extra`, rootless), an unapproved allowance in four forms, and a copied reserved module all refuse; the real adapter root keeps exactly one unchanged allowance and mentions the reserved file nowhere | **PASS** (`g074_05_…`, `the_guard_refuses_a_copied_or_aliased_native_module`, `the_guard_refuses_an_allowance_…`, `the_adapter_lint_arrangement_…`, `the_reserved_process_module_does_not_exist_…`) |
| `G074-06` | `libc` refused at the domain, application, adapter root, CLI and source-read paths; `rustix` refused at the reserved path; `libloading`/`dlopen`/`dlsym`/`libdl`/`LoadLibrary` refused at every path including both eligible ones | **PASS** (`the_guard_refuses_native_roots_outside_their_own_exception`) |
| `G074-07` | Unknown macro, `#[path]`/`include!` escape, foreign block, unresolved alias and source spoof still refused by the real checker; original negative fixtures still execute | **PASS** (every pre-existing fixture and the self-test matrix) |
| `G074-08` | Existing checker/boundary tests, workspace-graph/lock checks, CLI/application/media-worker tests, and format/lint/test tripwires | **PASS** (both-host run `43190767437`) |
| `G074-09` | No native module, no `unsafe`, no manifest/lock/policy/toolchain/workflow/product change, no stress or codec code | **PASS** (byte-identical inventory in §3) |
| `G074-10` | `fmt --check`, strict Clippy, `test --workspace --all-targets --locked`, architecture checker + refusal self-test, pinned `cargo-deny 0.20.2` | **PASS** on both hosts |

`S074-04` ✓, `S074-05` ✓, `S074-06` ✓, `S074-07` ✓. A passing in-memory future-source test is a parser/guard proof only: it does **not** show that the reserved file exists or compiles, and it gives no OS-kernel, Job Object or resource-exhaustion result.

## 6. Red iterations, disclosed in full

Three CI cycles were needed. Neither red run was caused by a weakened check; both were real defects found by the new negative tests, and both were fixed only inside the three permitted paths.

| Run | Commit | Result | Diagnosis | Fix |
| --- | --- | --- | --- | --- |
| `43188949943` | `fff1523d` | **red** — `Test` step on both hosts | My *expectation* was wrong: `all(any(unix, windows), target_os = "linux")` does **not** prove Linux, because the `any` branch does not necessarily include it, so the rule correctly refused it while the test demanded acceptance. | `3761d351`: that predicate moved to the refused list; the accepted nested-`all` case became `all(windows, all(test, windows))`. The rule was not changed. |
| `43189884326` | `3761d351` | **red** — `Test` step on both hosts | Two real guard defects: (a) `dlsym` was in the forbidden list but missing from the inventoried native-root list, so it was never examined; (b) the ungated-allowance refusal was folded into the unsafe-keyword branch, so an *eligible* file could ask for a blanket `allow(unsafe_code)` and be missed — a hole the new negative test exposed. | `577cc832`: `dlsym` added to the inventory (now 8 roots), and the allowance refusal made unconditional at every path. Both changes *strengthen* the guard. |
| `43190767437` | `577cc832` | **green** — 33/33 steps on both hosts | — | — |

Both red runs failed only the `Test` step: `Format` and `Clippy` (with `-D warnings`) passed in every run, which is independent evidence that the new code is lint-clean.

## 7. Final CI ledger

| Revision | Run | `ubuntu-24.04` | `windows-2025` | Result |
| --- | --- | --- | --- | --- |
| issuance `36688d5e` | **43185518897** | job `131071587202` success | job `131071586700` success | green |
| Charter `3fe6b1b9` | **43188095777** | job `131080245685` success | job `131080245373` success | green |
| checker `fff1523d` | **43188949943** | job `131083095957` failure (`Test`) | job `131083096428` failure (`Test`) | red |
| correction `3761d351` | **43189884326** | job `131086199624` failure (`Test`) | job `131086199138` failure (`Test`) | red |
| **code HEAD** `577cc832` | **43190767437** | job `131089193184` success, 33 steps | job `131089193692` success, 33 steps | **green** |
| report HEAD | see the chat handoff | — | — | required green |

The unchanged workflow ran the locked dependency fetch, the pinned `cargo-deny 0.20.2` gate with a fresh RustSec refresh and its advisory negative probe, format, strict Clippy, all-target tests, the real-tree architecture checker with its refusal self-test, the byte-reproducible release build and the release CLI smoke evidence. The `RustSec` probe line `advisories FAILED: 1 errors` visible in the logs is the **expected** negative control (RUSTSEC-2026-0009 fixture), not a workspace finding; the workspace advisory check itself passed.

## 8. Gates

| Gate | Status |
| --- | --- |
| `S074-01` one checkout/worktree/remote, every named blob, matching handoff, unchanged branches and scratch | **PASS** (§1) |
| `S074-02` ACTIVE-only issuance, remote readback, both-host CI at issuance SHA | **PASS** (§1, §7) |
| `S074-03` one exact paragraph; removal reconstructs `88b16fd9…`; other bytes unchanged; both-host CI | **PASS** (§2) |
| `S074-04` AST checker enforces two different path/OS rules with positive predicate semantics and no source/alias/macro/dynamic escape | **PASS** (§4, §5) |
| `S074-05` boundary guard preserves the sole active allowance, recognises only the exact reserved path, refuses mis-scoped/third-path code, keeps every old tripwire | **PASS** (§4D, §5) |
| `S074-06` no dependency/source/product/CLI/manifest/lock/policy/workflow/toolchain change, no native file, no `unsafe`, no media/release claim | **PASS** (§3) |
| `S074-07` checker-only commits touch only the three permitted paths; full `rust-verify` green at the code HEAD with all `G074` cases observed | **PASS** (§3, §5, §7) |
| `S074-08` report-only final commit, exact remote readback, both-host CI at the report HEAD, clean single worktree, preserved branches and scratch | **PASS** (this commit; readback in §9) |
| `S074-09` one accurate outcome, Stage 03 media gates OPEN, no self-acceptance | **PASS** — outcome `NATIVE-SCOPE-GUARDS-PROVEN`; no `HOST-CONTAINMENT-PROVEN`, no `MEDIA-VERIFIED`, no self-ACCEPT |

## 9. Limits, inventories and cleanup

**Terminal and toolchain.** As in the previous run, this Owner-local host has the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain but no MSVC linker or Visual Studio Build Tools and no Linux target, so any crate that must link — including `serde`'s build scripts, and therefore `solo-cli` and its test targets — cannot be built here. Local evidence was therefore limited to `rustfmt` (parse and formatting of the three permitted files, which were formatted in place until `--check` was clean with zero diffs) plus read-only Git/GitHub inspection. **Every compile, lint, test, architecture-check and security-gate result in this report comes from the two CI hosts**, not from this workstation. Three CI cycles were needed and all three are disclosed in §6.

**`unsafe` and exception inventory.** No `unsafe` token was added anywhere: zero unsafe blocks, zero `unsafe fn`, zero `SAFETY` notes were introduced by this order. The only active allowance in the repository remains the single, unchanged `#[cfg_attr(windows, allow(unsafe_code))]` at `crates/solo-adapters/src/lib.rs` for `source_read_native.rs`; the guard asserts its count is exactly one, that no ungated form exists, and that the adapter root does not mention the reserved file at all. The reserved path exists only as a constant and as in-memory test sources.

**Dependency and security posture.** `Cargo.lock` is byte-identical (`2f9e5357…`), the four-crate set and every approved root are unchanged, `libc 0.2.191` remains a transitive node only, `deny.toml` is untouched, and the pinned `cargo-deny 0.20.2` advisories/bans/licenses/sources gate passed on both hosts with a fresh RustSec refresh.

**Scratch and cleanup.** The inherited `SOLO-WO-073 R01` scratch was left untouched and re-measured independently this run: `.temp/solo-wo-073-wip/` still holds exactly 4 files / 46,062 bytes (`media_worker_host_native.rs`, `Cargo.toml.prepared`, `Cargo.lock.prepared`, `NOTES.md`). Task-owned material created by this order was removed before the report: `.temp/probe.txt`, `.temp/errors.txt`, `.temp/errors2.txt`, `.temp/ci_ok.log`, `.temp/ci2.log`, `.temp/ci_ok_run.log`, `.temp/tests.txt`, `.temp/arch_rebuilt.md` and the report assembly fragments. The two Owner-owned archives (`mingw.zip`, `rustup-init.exe`) and `target/` were not touched. No process was spawned, no stress or codec code executed, and no network operation beyond the existing workflow's own pinned fetches occurred.

**Final state.** `main == origin/main`, empty `git status --porcelain`, one worktree, one remote, four WIP files, and the four branches unchanged. The report-only commit's parent, tree and report blob were re-read from the GitHub API after the push and matched the local values; the exact SHAs are given in the chat handoff rather than embedded here, because this report cannot contain its own commit identity.

**Not claimed, and not to be inferred:** the existence, compilation or behaviour of `crates/solo-adapters/src/media_worker_host_native.rs`; any process launch, limit installation, process-group or Job Object enforcement; any AVIF decode, media correctness or publication; any sandbox, privilege or host-security clearance; any release or deployment; and any closure of `R03.C3`, `R03.C` or Stage 03. A green in-memory guard test proves that the *policy* is enforced for the text it is given; it proves nothing about a native implementation that does not yet exist.

**Next action.** The Architecture Charter and both guards are now ready for the separate, Owner-issued Linux and Windows native containment code orders. This order authorizes none of that work, and the Implementation Agent has begun no other order and does not self-ACCEPT.
