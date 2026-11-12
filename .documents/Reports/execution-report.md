# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-080 R01` — R03.C3 exact five-blob promotion of the independently proven Linux pre-exec limit smoke onto `main` (integration only).
**Matching activation ID:** `SOLO-WO-080 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `LINUX-PREEXEC-LIMIT-SMOKE-INTEGRATED`.** Exactly five accepted Git blobs were promoted byte-for-byte from technical branch commit `380393ddacf37fc1a6398805235ca2d5dbd9677e` onto `main` by path-scoped staging, in one integration commit with three modified and two added paths and no other change. The unmodified, push-triggered `rust-verify` workflow is green on `ubuntu-24.04` **and** `windows-2025` at that exact integration SHA, and the real Linux child reported the fixed ceilings from its own kernel view on `main` itself.

**Scope of the claim.** One **benign, trusted** child, launched once by the parent from the same `solo-cli` executable, received `setsid` plus hard and soft `RLIMIT_AS` = 536 870 912 bytes and `RLIMIT_CPU` = 10 consumed-CPU seconds **before `exec`**; that child read the ceilings and its group identity back from the kernel; the parent revalidated every field, required a normal zero exit inside a 15 000 ms wall deadline and reaped the single leader. Windows compiles only a fail-closed `HOST_UNSUPPORTED` path. The public CLI accepted/refused byte contracts are unchanged.

**This is not containment.** It is **not** `LINUX-HOST-CONTAINMENT-PROVEN`, **not** `HOST-CONTAINMENT-PROVEN`, **not** `MEDIA-VERIFIED`, not sandboxing, not a released feature and not an AVIF decoder. Limit **enforcement under stress** is not tested. `RLIMIT_AS` bounds virtual address space, not RSS and not aggregate child-tree memory; `RLIMIT_CPU` bounds consumed CPU time, not wall time; creating a process group proves nothing about descendant cleanup. The parent reader threads bound memory after leader exit, but that is **not** an independent proof against a descendant that inherited the pipes. Identity by `current_exe()`, parent attestation, hostile-child behaviour, filesystem/network/credential confinement and process-tree cleanup all remain unverified. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and both media-worker publication and media-correctness authority constants remain `false`.

## 1. Preflight — `S080-01` PASS

Read from the single real checkout before any write: `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, branch `main`, one worktree, one remote `origin=https://github.com/provelopment/provelopment-foundation-solo.git`, exactly four `.documents/WIP/` files.

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `7f9ed68348998bef6557c657c0be3580bb8d5fe8` | exact |
| `HEAD^{tree}` | `b4bee9da3043e9805f917f5431c06fd5fe2bbf4f` | exact |
| Canonical report blob | `b780054e45ced229cd6774d36527551583e5f66b` | exact |
| Previous ACTIVE blob | `a8da8eef897ba57d9e097b0a791fd1dfe60cb044` | exact |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `rust-verify` workflow | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1`, two `edges=41` assertions | exact |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / root | `47d732f2…` / `bd1d4c58…` | exact |
| Mainline native readback-only file | `2df6d0e98395f21fc72294606e9a97e954f97c63` | exact |
| Mainline CLI root | `6faa9fae29351123a49d45c5637ac4fbb1b615c8` | exact |
| Graph / unsafe guard / workspace graph | `7a5beada…` / `b88e7b42…` / `25a91b2c…` | exact |
| `solo-wo-079-r01` local and remote | `380393ddacf37fc1a6398805235ca2d5dbd9677e`, tree `18215c5ad9134e05baab3330bd378ec18b06e6ec` | exact, unmoved |
| `solo-wo-076-r01` | `40a40bab1d017e61340fe53399b49fb2ab335c3b` | unmoved |
| Historical branches | `4371af21…`, `e6162cdf…`, `d270f95c…` | unmoved |
| New files on `main` | `media_worker_host.rs` and `media_worker_host_smoke.rs` **absent** | absent |
| Dirty tracked paths | only the Owner-installed WO-080 ACTIVE | only that one |

The Owner-installed ACTIVE was 148 lines beginning `OWNER-ISSUED — SOLO-WO-080 R01` with the matching `SOLO-WO-080 R01` activation, working blob `c8ce3bb9c664a873bc200c6d7fdec2fdb8637a30`. All five accepted branch blobs were independently re-read and matched §3 exactly before staging. `.temp/` had no top-level files; its three inherited directories (`architecture-fixtures`, `solo-wo-073-wip`, `toolchain-installer`) and the retained `solo-wo-073-wip` four files / 46 062 bytes are **untrusted reference only** and were not imported, executed, copied or deleted. The workstation has no usable linker and no Linux target, so **all** compile/test/kernel evidence below comes from remote CI; that absence is not treated as a failure.

## 2. Issuance and integration ledger — `S080-02`, `S080-03`, `S080-06` PASS

| Step | Commit | Tree | Parent | Paths |
| --- | --- | --- | --- | --- |
| Issuance (`Issue SOLO-WO-080 R01: integrate proven Linux preexec limit smoke`) | `d40190a562b3a1ff7faf6f25bc613f86b860139f` | `d66637efc48ae099204ba8d80f43d726db74c0b3` | `7f9ed68348998bef6557c657c0be3580bb8d5fe8` | 1 — `.documents/WIP/workorder-active.md` (`M`), ACTIVE blob `c8ce3bb9…` |
| Integration (`feat: integrate proven Linux preexec limit smoke (SOLO-WO-080)`) | `0c231196e48c51fb60dc9b17bd290db04e276841` | `e56952910b492c3f25b935bda9754e2e68d4e668` | `d40190a562b3a1ff7faf6f25bc613f86b860139f` | 5 — 3 `M` + 2 `A` |
| Report (this file) | returned in the chat handoff | — | `0c231196e48c51fb60dc9b17bd290db04e276841` | 1 — `.documents/Reports/execution-report.md` |

Both pushes were ordinary fast-forwards; `origin/main` was re-read after each and matched the local SHA, and the GitHub API returned the same `sha`, `tree`, `parent` and file list. The five remote blob identities at the integration commit were read back individually from the GitHub contents API and match §3 exactly.

**Promotion mechanism.** Exactly the authorized path-scoped command, with the five paths named literally in the pathspec (single PowerShell line, no POSIX continuation copied blindly):

`git restore --source=380393ddacf37fc1a6398805235ca2d5dbd9677e --staged --worktree -- <five literal paths>`

No merge, cherry-pick, rebase, whole-tree checkout, wildcard staging, reset, stash, clean, force-push, new branch, clone or extra worktree was used, and the technical branch was neither changed nor superseded.

**Staged verification before committing** — `git diff --cached --name-status` gave exactly `M lib.rs`, `A media_worker_host.rs`, `M media_worker_host_native.rs`, `M main.rs`, `A media_worker_host_smoke.rs`; `git diff --cached --numstat` gave `20/7`, `801/0`, `83/0`, `46/2`, `319/0`; `git diff --cached --check` was empty; `git ls-files -o --exclude-standard` was empty; `git status --porcelain` listed only those five entries.

**Blob identity — branch and `main` are identical** (`G080-01`):

| Path | Accepted branch blob | Staged blob | `main` blob | Remote blob at integration | Status |
| --- | --- | --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` | same | same | same | M (from `2df6d0e9…`) |
| `crates/solo-adapters/src/media_worker_host.rs` | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | same | same | same | A |
| `crates/solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` | same | same | same | M (from `bd1d4c58…`) |
| `crates/solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` | same | same | same | M (from `6faa9fae…`) |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1f127001359e5f0ba941f92906f3280e433d8b9f` | same | same | same | A |

**Every other byte is immutable.** `git diff --name-only d40190a5 0c231196` is exactly those five paths, so no other tracked blob changed in the integration commit. Re-read at the integration commit: workflow `e1b5ca6e…`, `Cargo.lock` `b1ee56da…`, adapter manifest `47d732f2…`, unsafe guard `b88e7b42…`, architecture graph `7a5beada…`. No file was patched, rustfmt-rewritten or normalised after copying; the working tree was clean immediately after both commits.

**Behaviour and security review of the imported bytes (`S080-04`).** `media_worker_host_native.rs` remains the sole process-native boundary: two pre-existing `getrlimit`/`assume_init` `unsafe` expressions plus two added ones (`pre_exec` registration, `getpgid`), each inside a positive `#[cfg(target_os = "linux")]` gate with an adjacent `// SAFETY:` rationale; the only pre-exec operations are `setsid` and the two fixed `setrlimit` calls; no global allow, no FFI outside that file, no new native root, no Windows process control, no dynamic loading. The profile is compile-time fixed at 536 870 912 bytes and 10 seconds for both soft and hard halves, and the parent re-checks every field rather than trusting child text, requiring `pgid == pid` with both positive. The parent launches only `current_exe()` with the private two-token mode, closed stdin, capped stdout/stderr, one fixed accepted line, and closed typed refusals for malformed, partial, duplicate, overlong, non-UTF-8, wrong-exit and wrong-value evidence. No new dependency, graph edge, Charter text, workflow line or checker change exists.

## 3. Integration CI on `main` — `S080-05`, `S080-06` PASS

Run **`43231769157`**, event `push`, `head_sha` exactly `0c231196e48c51fb60dc9b17bd290db04e276841`, conclusion `success`.

| Job | ID | Result | Steps |
| --- | --- | --- | --- |
| `verify (ubuntu-24.04)` | `131228772617` | success | 33 configured, **0 failed** |
| `verify (windows-2025)` | `131228772875` | success | 33 configured, **0 failed** |

The only skipped steps are the documented host-inapplicable counterparts (Windows-only steps on the Ubuntu job and Linux/Ubuntu-only steps on the Windows job). No applicable step failed and none was skipped because of an earlier failure.

### `G080-02` — real Ubuntu child readback on `main`

This value comes from this run's own `main` log, not copied from the branch run:

```
n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4249 pgid=4249 wall_ms=11
```

```
test n079_01_the_real_child_observes_the_installed_limits ... ok
```

Both ceilings at full width with equal hard and soft halves, `pid = pgid = 4249` (both positive, so the child really is its own session and process-group leader), and `wall_ms = 11` — far inside the 15 000 ms deadline. The reader's default ceilings are unlimited, so equality with the profile can only come from the pre-exec installation, and the values are the child's own `getrlimit`/`getpgid` observations re-parsed and re-validated by the parent.

### `G080-03`, `G080-04` — refusals and negatives on `main`

```
n079_direct_child=exit=71
test n079_04_a_direct_unsupervised_child_is_refused ... ok
test n079_03_wrong_internal_arguments_are_refused_by_the_real_binary ... ok
test media_worker_host::tests::n079_03_the_reserved_flag_is_honoured_only_in_second_position ... ok
test media_worker_host::tests::n079_03_every_wrong_internal_token_shape_is_a_typed_refusal ... ok
test media_worker_host::tests::n079_03_only_the_exact_observation_line_is_accepted ... ok
test media_worker_host::tests::n079_03_malformed_observations_are_refused_without_partial_values ... ok
test media_worker_host::tests::n079_03_child_exit_statuses_are_classified ... ok
test media_worker_host::tests::n079_03_values_outside_the_fixed_profile_are_refused ... ok
test n079_06_the_public_cli_is_unchanged ... ok
```

The unsupervised direct child exits `71` (one of the three fixed child verification statuses) with empty stdout and empty stderr — no partial or fabricated evidence — and the code claims no parent authentication. `N079-02` remains asserted **inside** `n079_01` (one line, eight `key=value` tokens, fixed field order, full-width values, one `pid`, one `pgid`, `pgid == pid`); it is **not** reported as a separately named test.

### `G080-05` — Windows refusal and unchanged public CLI

```
test n079_05_both_internal_modes_are_unsupported_before_any_child_exists ... ok
test n079_06_the_public_cli_is_unchanged ... ok
```

plus the six platform-independent `n079_03` parser cases. The Windows log contains **zero** occurrences of `n079_01`, `n079_04`, `n079_direct_child`, `n079_observation` or `n076_`: no Linux native module, test or readback was compiled or executed there. Both internal modes return exit `3` with `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` before any child exists, and the public CLI still yields `preflight=ok entity_id=entity-7 base=https://example.com/alpha` at exit `0` and `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` at exit `2`.

### `G080-06` — previously accepted kernel readback and regression suites intact

```
test media_worker_host_native::linux::tests::n076_01_getrlimit_rlimit_as_and_rlimit_cpu_observe_the_real_kernel ... ok
test media_worker_host_native::linux::tests::n076_02_ceilings_keep_their_full_range_and_repeat_consistently ... ok
test media_worker_host_native::linux::tests::n076_03_a_rejected_status_is_a_typed_refusal_not_a_partial_observation ... ok
```

Every earlier source-read, protocol, media-worker, staging and CLI suite still passes on both hosts.

### `G080-07`, `G080-08` — architecture, guards, toolchain and security gate on both hosts

| Gate | Ubuntu | Windows |
| --- | --- | --- |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Workflow `edges=41` assertions | both unchanged literal checks passed | both unchanged literal checks passed |
| Refusal self-test | `self-check=passed cases=29` | identical, under its own `-notmatch` guard |
| Negative controls | `ok dynamic-load refused as dynamic-loading (1 finding(s))`, `ok checker-bypass refused as layer-outward-reference (1 finding(s))`, plus the outward/alias/re-export/qualified/private/undeclared/`#[path]`/`include!`/missing-module refusals | same matrix |
| Format / strict Clippy / all-target tests | `cargo fmt --all -- --check`, `cargo clippy … -D warnings`, `cargo test --workspace --all-targets --locked` | same commands |
| Pinned toolchain | `rustc 1.99.1`, `cargo 1.99.1` asserted | same, asserted |
| Component security gate | `cargo-deny 0.20.2`, archive sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`, RustSec `5fe96cf51fde2fd4e749d51ac8bdc59edbdce77b` fetched `2026-11-12T13:54:58Z`, advisories/bans/licenses/sources ok | same version, archive sha256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`, same RustSec commit fetched `2026-11-12T13:55:29Z`, all ok |

No dependency, lock, feature, toolchain, policy, workflow, Charter or checker file changed: the graph stays at **41 edges** and **four** crates, and `targets`/`files` moved 32→33 and 58→60 only because one integration-test target and two source files were added, which is precisely the authorized ledger.

### `G080-09` — tripwires, release gates and reproducibility

| Proof | Ubuntu | Windows |
| --- | --- | --- |
| Test-failure tripwire | normal pass plus required injected failure | same |
| Architecture/advisory refusal tripwires | injected forbidden import refused as `undeclared-crate` with exit 1 | same |
| Forbidden-import restore | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | n/a — Ubuntu-only step |
| Locked release build | pass | pass |
| Release CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | `release_smoke=verified accepted_exit=0 refused_exit=2` |
| Two clean rebuilds, raw bytes | `clean_release_sha256_a = clean_release_sha256_b = 49b066056a333e3d67e3b1f384c456b07a8ea65c485ad71d064313cb8b053fbe`, `clean_release_bytes=623472`, `reproducibility=verified` | n/a — Ubuntu-only step |

`Cargo.lock` identity, recorded honestly as two different kinds of value: the **shared Git blob** is `b1ee56da60f36536548586528db371b7cbe508e4` on both hosts, while the **host checkout bytes** differ by line-ending handling — Ubuntu raw SHA-256 `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723`, Windows raw SHA-256 `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e`. Each job re-hashed the file before and after its security gate to prove the gate itself changed nothing. The two raw values are **not** equated with the blob, and neither is presented as corruption.

### `G080-10` — CI revisions

The original push-triggered workflow is successful at the exact integration SHA `0c231196…` (run `43231769157`), and its success is required again at the report-only SHA; that run and its job IDs are returned in the chat handoff because this report cannot embed its own future commit. The narrow nonpublishing claim is preserved: nothing was published, released or deployed.

## 4. Gate table — stated honestly

| Gate | Status | Basis |
| --- | --- | --- |
| `S080-01` | **PASS** | §1 — every anchor, blob and reference exact; one checkout/remote/worktree; four WIP files; clean unknown-state inventory; branch unmoved |
| `S080-02` | **PASS** | §2 — ACTIVE-only issuance `d40190a5…`, remote readback exact, run `43230869669` green both hosts |
| `S080-03` | **PASS** | §2 — five path-scoped promotions, staged and remote blobs all five accepted values, branch `380393dd…` preserved, no whole-branch merge |
| `S080-04` | **PASS** | §2 — four Linux-gated `unsafe` expressions with local rationales, fixed compile-time profile, imported files unchanged from the branch, no dependency/graph/Charter/workflow change |
| `S080-05` | **PASS** | §3 — real Ubuntu parent/child and negatives, Windows unsupported refusal, ten `G080` proofs observed in this run's `main` logs |
| `S080-06` | **PASS** | §2/§3 — one integration commit with five paths, push-triggered full CI green both hosts at the exact SHA |
| `S080-07` | **PASS** | §5 — this is its own one-path commit; its blob/tree/parent and the report-head CI are returned in the chat handoff |
| `S080-08` | **PASS** | §5 — no branch, worktree, clone or remote change; clean `main`; nothing unknown or Owner-owned erased; task scratch removed; inherited scratch inventoried |
| `S080-09` | **PASS** | `LINUX-PREEXEC-LIMIT-SMOKE-INTEGRATED` only; one harmless trusted child; `R03.C3`, `R03.C`, Stage 03 and all unimplemented safety/media gates OPEN |

| Proof | Status |
| --- | --- |
| `G080-01` blob equality, 3 M / 2 A, refs preserved | **PASS** — §2 |
| `G080-02` real Ubuntu readback at this `main` SHA | **PASS** — `pid=4249 pgid=4249 wall_ms=11` from run `43231769157` |
| `G080-03` direct unsupervised refusal | **PASS** — exit `71`, empty stdout, no accepted evidence |
| `G080-04` internal-argument, parser, profile, exit-status and binary refusals | **PASS** — nine real cases on Ubuntu, six on Windows, `N079-02` inside `n079_01` |
| `G080-05` Windows unsupported, no Linux execution, public CLI unchanged | **PASS** — zero Linux test names in the Windows log |
| `G080-06` `N076-01..03` and earlier regressions | **PASS** |
| `G080-07` architecture/graph on both hosts | **PASS** — `packages=4 targets=33 files=60 edges=41`, `cases=29`, negative controls refusing |
| `G080-08` fmt/Clippy/test/toolchain/deny on both hosts | **PASS** — fresh RustSec on each host |
| `G080-09` tripwires, release, smoke, reproducibility, lock identity | **PASS** — with the Git-blob versus host-bytes distinction recorded |
| `G080-10` both CI revisions | **PASS** at the integration SHA; report-head run returned in chat |

Nothing in this order reached `FAIL` or `NOT RUN`. There was no late CI failure, so no corrective source change was needed or made, and no sixth path was touched.

## 5. Final state, limitations and cleanup

**State.** `main` now carries exactly the five accepted blobs on top of the Owner-issued ACTIVE; `main == origin/main == 0c231196e48c51fb60dc9b17bd290db04e276841` (tree `e56952910b492c3f25b935bda9754e2e68d4e668`) before this report, with a clean working tree, one worktree and one remote. The technical branch `solo-wo-079-r01` remains `380393ddacf37fc1a6398805235ca2d5dbd9677e` locally and on `origin`, and `solo-wo-076-r01` (`40a40bab…`), `solo-wo-061-r01` (`4371af21…`), `solo-wo-069-r01` (`e6162cdf…`) and `solo-wo-070-r01` (`d270f95c…`) are byte-for-byte unchanged. No new branch, pull request, tag, release or deployment exists, and no history was rewritten or force-pushed.

**Toolchain limitation.** This Owner-local workstation still has no usable linker and no Linux target, so `cargo check`, `cargo build` and `cargo test` cannot run locally — even `cargo check` fails because dependency build scripts must be linked. Every compile, lint, guard, test and kernel-readback statement above comes from remote two-host CI. `target/` and all unrelated local files were left untouched.

**Cleanup.** The only task-owned material created by this run was disposable report scratch in the ignored `.temp/`, which was deleted after assembly, so `.temp/` again has no top-level files. Its three inherited directories and the `solo-wo-073-wip` four files / 46 062 bytes are preserved exactly as found and remain unaccepted reference only, with their future cleanup still belonging to a later separately authorized native-containment order. No Owner data was deleted, moved or reinterpreted, and no `git reset`, `git clean`, stash or destructive checkout was used.

**Remaining risks and explicitly open work.** Limit enforcement under stress, smaller resource-cap trip tests, descendant spawn and kill/sweep coverage, an independent wall-watchdog negative, concurrent aggregate quotas, Windows Job Objects, decoder quality and media correctness, source-confinement least privilege, `current_exe()` identity and anti-replacement, hostile-child behaviour, and whole-publication validation are all **future Owner-issued work**, not hidden acceptance criteria of this integration. A green mainline integration must never be described as full host containment.

The Implementation Agent has not self-ACCEPTed, has not merged or released anything, and has not begun another order. It stops here for independent Chief Master Architect disposition against this report and the exact revisions named above: **ACCEPT / REVISE / STOP**.
