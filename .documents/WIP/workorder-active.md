# SOLO-WO-099 R01 — UNBLOCK EXACT WINDOWS NATIVE BOUNDARY AND COMPLETE R03.C PNG/JPEG/WEBP INSPECTION

**Authority:** `OWNER-ISSUED — SOLO-WO-099 R01` (effective **only** after Owner installs this entire file in the canonical ACTIVE slot and provides the matching explicit handoff).  
**Matching Owner activation:** `SOLO-WO-099 R01`  
**Required outcome:** `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED`  
**Target repository:** `provelopment/provelopment-foundation-solo`  
**Sole checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (WSL may operate on this **same** checkout under `/mnt/c`, not a second clone).  
**Target branch:** `main`, plus **only** one new technical branch `solo-wo-099-image-r01` and one deliberate-red branch `solo-wo-099-negative-r01`. No PR, merge, release, deployment or additional worktree.

## 1. Frozen Approved Scope and architectural decision

The Chief Master Architect independently reviewed the `SOLO-WO-098 R01` STOP report at `ee4e238df4f5d276eaf604a4321d777e9cd1e83c`. The forward-only green technical recovery **is accepted as fact**, not as completion of the PNG/JPEG/WebP project. `SOLO-WO-098` remains **STOPPED / NOT ACCEPTED**. This order authorizes an exact correction to the stale first-party unsafe-boundary **test contract**, followed by actual implementation, two-host proof, deliberate-negative sensitivity, byte-identified integration and a truthful report. The Owner does **not** waive any compiler, AST architecture, security, host-containment or refusal gate.

**Binding architectural decision, not an invented exception:** Architecture §6A.8, under `SOLO-WO-074 R01`, already reserves **exactly** `crates/solo-adapters/src/media_worker_host_native.rs` for positively Linux- or Windows-gated process-control `unsafe`, alongside the older strictly Windows-only `source_read_native.rs` exception. `crates/solo-cli/tests/architecture_checks/source.rs` already implements `unsafe_scope_verdict` that permits **either positive OS gate** in the reserved process-native file and refuses ungated/other-file unsafe. It is **not** the blocker. The dated `crates/solo-cli/tests/unsafe_boundary.rs` asserts *exactly one Windows module-scoped allowance* in `solo-adapters/src/lib.rs`, which prevented the newly approved Windows worker seam. Repair that guard to recognize the **one additional, exact process-module Windows allowance** while retaining the original source-read allowance and the exact Linux allowance. **No extra native file, generic `unsafe` allowance, wildcard policy, weakening of negative fixtures, or change to canonical Architecture.** The new guard must mechanically tie each allowance to its correct module declaration, and accept the pre-implementation legacy arrangement as well as the completed arrangement during this staged run.

The intended result remains **nonpublishing**: verified PNG/JPEG/WebP still-image inspection of already confined, owned Entity graphic bytes, complete typed all-or-refused evidence for all independently authored obligations, real child-only bounded decode on both Ubuntu and Windows, and trustworthy parent checks. The decoder remains exact `image =0.25.10`, `default-features=false`, features exactly `png`, `jpeg`, `webp`. Preserve all N079–N093 accepted behavior. `R03.C3`, `R03.C`, and Stage 03 remain OPEN; AVIF, video/WebVTT, platform catalogue, public projection, URL/HTTP/browser, release and deployment remain outside this order.

**Safety requirement discovered by independent source inspection:** WO-097's Windows `create_limited_job_v1()` returned `(job, limits_verified)` even when readback was false, and `SuspendedWorkerV1::resume()` did **not** prevent execution with `limits_verified=false`. Correct both edges: **no child may resume without successful readback of all four Job Object limits, verified assignment/membership and required pre-run conditions**; every early failure must terminate and confirm the owned process if one exists and close its native handles. A claimed failure after unconfined decode is NOT acceptable. Audit explicit handle inheritance (no incidental inheritable handles) and child environment (no secrets, tokens or public-output authority) before a positive security result. An OS resource limit is not a general network/filesystem sandbox; do not claim it is.

## 2. Exact baseline preflight — STOP on mismatch

Before permanent writes, compare actual committed Git objects/refs against these **entire** identities; the only initial dirty file may be the unstaged Owner-installed ACTIVE containing this exact order and handoff:

| Identity | Mandatory committed baseline |
| --- | --- |
| `HEAD == main == origin/main == ls-remote origin main` | `ee4e238df4f5d276eaf604a4321d777e9cd1e83c` |
| Main tree / sole parent | `5acc6d5769f653e0d23a373c13eb6d12e0f38246` / `ed938267001e4a1b20cc3001af8fafb3c82f40b5` |
| Report, `status=STOPPED`, WO-098 | `41acd865fd6e02ad01b4acea2a91a95870415048` |
| Tracked WO-098 ACTIVE | `946b6162d34ed46428b90bac7d1f8daec7731cf9` |
| Neutral NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Architecture / Architect SOP / Implementation SOP | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` / `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1` / `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |
| Root manifest / lock / toolchain / workflow | `14505be49a281ba0495eabea5ca386548cc36347` / `b1ee56da60f36536548586528db371b7cbe508e4` / `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` / `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` |
| `unsafe_boundary.rs` / AST `source.rs` / graph `graph.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` / `df1f8758b201214e5a4066823e83449f9c6cef6f` / `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` |
| Adapter `lib.rs` / native process seam / façade | `1550d34f820642bb5573e06d4fccb758109fd26c` / `eacf51c5353ab3278969961aa136a0951cb7ad67` / `7eed6e158c9374a6c2055ddd98dd3287357b814a` |
| Accepted graphics source-read / existing AVIF application boundary / smoke | `6499421d6e718e9d46d64e45fb152f34e2989cc1` / `13811c5811c2170b811ed4a4705e0d940d6d21e7` / `22ee67e94cc96a70e8e3a47db9e575464b53cd07` |
| Known green recovery commit / two-host CI | `ed938267001e4a1b20cc3001af8fafb3c82f40b5`, tree `837ebe629c742b474f06068b94cfee2d8a59da0b`, run `43442905821` |
| Green final stopped report / two-host CI | `ee4e238df4f5d276eaf604a4321d777e9cd1e83c`, run `43446437078` |
| Prior unaccepted WO-097 technical B4 | `816680fc469bc906afbc31ecaae871359247be9b` (**source reference only, not an accepted binary**) |

All **14** remote branch refs must equal baseline; important frozen refs: `solo-wo-093-r01=0dc45185110e6136bd63dc7edf774bc52b753c9e`, `solo-wo-094-negative-r01=0e9b3677cf774b9e31ac8fc52605e88112522be6`, `solo-wo-089-r01=1236915cab6e87b17ce84813430897b273fc3230`, `solo-wo-090-r01=0eb3fcf645851ae752bd59e12fe32b69604197d2`. Proposed `solo-wo-099-*` remote refs must be absent. A preexisting **local only** `solo-wo-098-image-r01` ref at the accepted recovery commit is permitted and must be preserved; no reset/delete/rebase. One checkout, one remote `origin` with fetch/push `https://github.com/provelopment/provelopment-foundation-solo.git`, no stash, exactly the four canonical WIP files and no WIP subdirectories; verify no unresolved Git operation and no unknown staged/dirty paths. Preserve ignored Owner data, particularly `.temp/solo-wo-073-wip/` (last agent report: 4 files / 46,062 B), `architecture-fixtures/`, `toolchain-installer/`, `target/`, Cline logs. Existing task-owned `.temp/solo-wo-098-wip/unverified-attempt.patch` (last reported **244,647 B**) and `.temp/solo-wsl-target/` cache are **preserved input/evidence**, never silently deleted or overwritten; verify their actual presence/size before using. If absent or changed, report and reconstruct only from Git with audited diffs; never guess lost data.

## 3. Exact permanent path ledger and permissions

**Documentation:** `.documents/WIP/workorder-active.md` (Commit A, frozen Owner bytes), `.documents/Reports/execution-report.md` (final report only). No NEXT, ROADMAP, OUTLOOK, SOP, About or Architecture edits.

**One checker correction, separate Commit B on `main`:** `crates/solo-cli/tests/unsafe_boundary.rs` only. This is the sole amendment to a previously accepted guard. `crates/solo-cli/tests/architecture_checks/source.rs` is **read-only**: it already admits a positive Windows scope in exactly the approved reserved path. Do not change it just to suppress a diagnostic.

**Exactly twelve technical paths in the isolated correction branch, later exact-blob promotion to `main`:**

1. `.github/workflows/rust-verify.yml` — verified exact graph counts and mandatory N097 host proof only.
2. `Cargo.lock` — reviewed, pinned additive image graph only.
3. `crates/solo-adapters/Cargo.toml` — `image =0.25.10` exactly, 3 features only; exact minimum necessary preexisting `windows-sys 0.61.2` features. No other new direct dependency or version change.
4. `crates/solo-adapters/src/graphic_image_inspection.rs` (new) — real in-worker decode, strict framed protocol, parent verification, confined source composition.
5. `crates/solo-adapters/src/lib.rs` — one dual-target process-native module declaration, preserve the original Windows source-read exception, exports and `#![deny(unsafe_code)]`.
6. `crates/solo-adapters/src/media_worker_host.rs` — safe parent process supervisor, fixed worker mode and checked cleanup; all older probes unchanged.
7. `crates/solo-adapters/src/media_worker_host_native.rs` — **the sole** additional process-native OS boundary, gated internally as separate positive Linux/Windows modules, and no other new `unsafe` file.
8. `crates/solo-application/src/graphic_image_inspection.rs` (new) — pure inward port, typed whole-set refusal/provenance; no decoder/native imports.
9. `crates/solo-application/src/lib.rs` — exact type exports only.
10. `crates/solo-cli/src/main.rs` — fixed internal worker dispatch; existing public CLI behavior unchanged.
11. `crates/solo-cli/tests/architecture_checks/graph.rs` — closed exact new dependency-root/closure, never a blanket exemption.
12. `crates/solo-cli/tests/media_worker_host_smoke.rs` — genuine Linux/Windows N097 integration tests and adversarial refusal suite.

Everything else is **read-only**: no extra fixture files, checker entrypoint, AST source checker, unsafe policies, manifest, original graphic/AVIF/source-read modules, `deny.toml`, compiler pin, action pins, dependencies outside approved graph or prior branches. Do not add a second native file or move `unsafe` to another path. Any unavoidable other permanent path is an immediate STOP/escalation, not an implicit ledger extension.

**Permitted Git operations:** ordinary scoped fast-forward commits/pushes on `main` for A, B, exact technical promotion and report; creation/push of only `solo-wo-099-image-r01` and `solo-wo-099-negative-r01`; workflow_dispatch for the new branches; read-only comparisons of old commits and refs; safe path-scoped restore of exactly authorized files from Git. No `git reset`, `git clean`, destructive checkout, force-push, rebase, amend, cherry-pick, merge, new clone/worktree/remote, branch delete, PR, release, deployment or secret/cloud/provider action. The WSL Linux environment and user-scoped Rust 1.99.1 already exist; reuse them (no new system installs). If the WSL environment has an actual missing required prerequisite, STOP and report rather than installing an unapproved package. Existing network-approved Git, Cargo fetch and security/advisory checks only; maintain current CI fetch boundaries. Temp work restricted to known task-owned paths, preserving Owner material.

## 4. Commit A: install authority; require green unchanged-code CI

On exact baseline, commit **only** the Owner-installed ACTIVE as one ordinary fast-forward child of `ee4e238d…`; preserve its bytes exactly, including Markdown hard-break whitespace; disclose whitespace findings, do not normalize Owner text. Push. Confirm GitHub readback commit, tree, sole parent and ACTIVE blob. Require a fresh successful **`push`** `rust-verify.yml` run, attempt 1, at that SHA with Ubuntu 24.04 and Windows 2025 GREEN. No technical edit or branch before this gate. NEXT stays neutral.

## 5. Commit B: exact-path unsafe-checker contract repair, independently green

Edit **only** `crates/solo-cli/tests/unsafe_boundary.rs`; the pure AST checker `architecture_checks/source.rs` remains untouched and load-bearing. The existing guard currently counts exactly **one** Windows allowance in the adapter root while §6A.8 already allows both native paths. Replace the brittle count-only assertion with a closed **per-declaration** verifier (e.g., parse the real `solo-adapters/src/lib.rs` using the already pinned `syn` test dependency):

- The crate root retains exactly `#![deny(unsafe_code)]` and **zero** crate-wide `allow(unsafe_code)`; no additional allowance in any `mod`, function, other file or nested scope.
- `mod source_read_native;` retains exactly one Windows-only `#[cfg_attr(windows, allow(unsafe_code))]` on **that module**, and no Linux allowance.
- `mod media_worker_host_native;` retains exactly one Linux-only `#[cfg_attr(target_os = "linux", allow(unsafe_code))]` on **that module**. During this staged order accept exactly TWO **finite** shapes: **legacy** `#[cfg(target_os = "linux")]` with no Windows allowance; **new** `#[cfg(any(target_os = "linux", windows))]` plus exactly one Windows-only `#[cfg_attr(windows, allow(unsafe_code))]` on this same process-native module. The `any(..)` is for **module availability only**, never a proof of the scope of an `unsafe` expression. Every actual process-native `unsafe` occurrence inside the file must still be inside its own explicit `#[cfg(target_os = "linux")]` or `#[cfg(windows)]` scope as judged by the unchanged AST checker.
- No duplicated allowance, extra module, `#[path]` evasion, misplaced attribute, alternate predicate, wildcard `allow`, negated/ambiguous gating, dynamic-load/native root expansion or permission for a copied path. Both approved exact native-file identities and the list of permitted native roots remain unchanged. Do not turn a real violation into a skip or a warning.
- Add **load-bearing negative tests** which inject an extra/misplaced Windows allowance, an allowance on an ordinary module, missing Linux allowance, a combined `any`-gated unsafe expression, ungated unsafe, a copied path, forbidden native root, and a missing `// SAFETY:` explanation. Each must fail the **same production checker**. Retain all existing negative and source-reader tests. A positive synthetic test of the *new exact module arrangement* must succeed while the **real checkout remains in the legacy shape**.

An acceptable **target source declaration after technical integration** (formatting may vary) is:

```rust
#![deny(unsafe_code)]
// Existing modules unchanged.
#[cfg(any(target_os = "linux", windows))]
#[cfg_attr(target_os = "linux", allow(unsafe_code))]
#[cfg_attr(windows, allow(unsafe_code))]
mod media_worker_host_native;
// Keep the original source-read Windows allowance separately and exactly:
#[cfg_attr(windows, allow(unsafe_code))]
mod source_read_native;
```

Never rewrite this as `#[cfg_attr(any(...), allow(unsafe_code))]`, a file-level `#![allow]`, or a `cfg(any(...))` permission for native code. Before publishing this checker correction, run locally in WSL `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --all-features --locked`, strict Clippy, `cargo test -p solo-cli --test unsafe_boundary --locked`, and `cargo test -p solo-cli --test check-architecture --locked -- --self-test` and the real checker. For WSL missing fixture volumes, do not claim broad suite pass. Commit **only** `unsafe_boundary.rs` on main, push, and verify **fresh successful two-host `push` CI** on exact B SHA, including all guard tests. This is a governance-test correction with no new production feature. If it fails, **STOP before code**.

## 6. Isolated correction branch: reuse saved work, fix proven errors before pushing

Create precisely `solo-wo-099-image-r01` at the GREEN Commit B, in the SAME checkout. Do **not** delete or reset the existing local `solo-wo-098-image-r01`, and preserve the 244,647-byte unverified patch as forensic evidence. Reconstruct the twelve technical paths from the known WO-097 B4 source and/or an inspected copy of that patch **only after** comparing its complete path list and checking it has no unapproved writes, symlinks, submodules, external command execution, unsafe source locations or new manifest. The patch is untrusted, not self-proving or accepted. Use path-scoped Git restores, manually reviewed hunks and WSL builds; do not `git apply` blindly or create a shadow clone. Preserve the checker B revision and use the actual current base for diffs.

**Direct corrections already verified in WSL during WO-098; reapply/check without undoing working parts:**

1. `media_worker_host.rs`: `ImageWorkerConfinementV1::frame_fields_v1` closes immediately after its `match`; its `impl` closes before three `child_image_worker_confinement_v1` cfg variants and module-level `ImageWorkerRunRefusalV1` / `ImageWorkerRunV1`. Attach the orphaned `/// The Linux half` comment to the actual Linux `mod` declaration. No nested `pub struct/enum` inside a function.
2. `media_worker_host_native.rs`: place `terminate_own_group_v1` in the positive Linux module's normal `pub(crate)` scope, **outside** `#[cfg(test)] mod tests`. Preserve all prior N090/N093 signal/reap and PID/PGID refusal semantics. Export only private crate-scoped fixed helpers.
3. Windows structure types in `windows-sys 0.61.2`: use `JOBOBJECT_BASIC_ACCOUNTING_INFORMATION` and `JOBOBJECT_EXTENDED_LIMIT_INFORMATION` **for types and sizes**; keep the distinct `JOB_OBJECT_LIMIT_*` flag names and `JobObject...Information` info-class constants. No guessed FFI type alias.
4. `SuspendedWorkerV1::spawn(..) -> Option<Self>` must be converted with `.ok_or(ImageWorkerRunRefusalV1::SpawnFailed)?`, not `.map_err` on Option.
5. New module-level Linux supervisor helpers must reach the `media_worker_host_native` functions via **private** audited re-exports, not public native root leaks. The `solo-cli` smoke target cannot directly import `solo_domain` because it has no declared edge: retain the confined adapter-level `test_inventory_v1` builder, and correct reference fixture paths under the actual `assets/` convention.

**Additional independently discovered Windows safety fixes (mandatory before any positive decode):**

6. `create_limited_job_v1` currently returns `Some((job, verified))` even when `verified == false`. Change its contract/control flow so a false **readback** terminates/closes that new job and returns an explicit failure; never proceed to process launch. After suspended `CreateProcessW`, a failed job assignment, failed `IsProcessInJob`/the exact-job membership verification, false limit readback or failed handle setup must terminate the **owned** process/job, perform checked bounded wait if created, close all handles **exactly once**, and report a refusal; none may produce a resumable object. Add a guard **inside** `SuspendedWorkerV1::resume()` that rejects any `!limits_verified` or unverified assignment/membership, even if a caller regresses. Parent caller must also assert all facts before resume. No child is permitted to execute a single decoder instruction first and then report containment failure.
7. Audit `CreateProcessW` failure paths: existing `?` after acquiring `job`/pipes must not leak handles. Use owned RAII handles or explicit matched cleanup for all fallible paths; eliminate close-on-error double-free. Do not treat `TerminateJobObject` return alone as proof that a created process is gone: check `WaitForSingleObject` and exit state, then readers. Checked normal-success wait/exit is also mandatory. Prove no live process/owned handle survives refusal/timeout.
8. `bInheritHandles=TRUE` with only the two parent pipe ends marked non-inheritable is **not sufficient proof of an explicit handle allowlist**: prove `PROC_THREAD_ATTRIBUTE_HANDLE_LIST`/equivalent fixed allowlist containing only child stdin/stdout handles and no inherited owner/secret handles, or STOP if the exact Windows host cannot enforce it. Give worker a deliberately minimal, explicitly constructed child environment (no token, credential, provider secret or arbitrary Owner environment propagation) and a safe working directory; preserve the fixed trusted executable identity and robust Windows command-line quoting. Do not assert rights/network sandboxing beyond what was actually tested.
9. Verify `JobObjectExtendedLimitInformation` flags and all four readback values (10 s process CPU, 536,870,912 B process commit, 536,870,912 B job commit, KILL_ON_JOB_CLOSE), suspended-**before**-assignment ordering, exact-job membership, resume, 15,000 ms parent wall guard and checked wait. Add real Windows tests that prove unverified readback refuses **before resume**, wrong job/membership refuses, unexpected handle inheritance is absent, and timeout kills the owned job; ensure the positive case proves an actual contained worker decoded real bytes. Do not manufacture host facts from a worker claim or assert a Linux-only proof is Windows proof.

Run the WSL fast loop on `/mnt/c/...` (same checkout), using existing Rust 1.99.1, rustfmt/Clippy and linker:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test -p solo-cli --test unsafe_boundary --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-cli --test check-architecture --locked -- --self-test
cargo test -p solo-cli --test media_worker_host_smoke --locked n097_09 -- --nocapture --test-threads=1
cargo test -p solo-cli --test media_worker_host_smoke --locked n097_10 -- --nocapture --test-threads=1
cargo test -p solo-cli --test media_worker_host_smoke --locked n097_11 -- --nocapture --test-threads=1
```

Investigate every new compilation diagnostic first, before any GitHub branch push; no repeated known-red pushes. Existing WSL filesystem suites which demand unavailable `SOLO_TEST_SOURCE_FS_ROOT`/special fixture filesystem may remain explicitly `LOCAL-UNVERIFIED` only: they **must** pass on configured CI hosts. Never disable/skip them, fake a source root, install unsupported WSL packages, or claim WSL proves Windows. Keep the image dependency graph exact and audit current RustSec/OSV/vendor support/versions/licences and `cargo-deny` output; preserve `Cargo.lock` and preexisting security rules. `Architecture` and all earlier accepted source/AVIF files remain untouched.

## 7. Branch CI; real two-host proof before integration

After WSL fmt/check/strict-Clippy and available tests pass, commit the bounded twelve-path technical diff to `solo-wo-099-image-r01`, ordinary push and use existing `rust-verify.yml` **workflow_dispatch** at that exact SHA. Ubuntu **and** Windows jobs must both pass, including the actual N097 tests, all original N079–N093 regressions, the real-tree checker with accurately measured/pinned graph counts, AST/unsafe boundary negative fixtures, tripwire, locked all-target tests, release smoke, reproducibility, and fresh cargo-deny advisories/bans/licences/sources on both hosts. One correct attempt per new revision; if new substantive diagnostics appear, fix only authorized paths locally first and push a **new forward** branch commit, retaining earlier red evidence. Never hide a failing skipped mandatory check or loosen the workflow count/feature/security assertions.

Real evidence required for **all three formats**: PNG, JPEG, WebP full decode inside a single-use contained worker from an actual confined Entity `assets/` root, correct observed MIME/magic and known pixels/digest, correct source provenance, 20 MiB/8192/32M ceilings, whole-inventory refusal, internal/unlisted/suppressed obligations, no fallback, zero publication authority. Exact Linux `RLIMIT_AS`, `RLIMIT_CPU`, new session/process group, measured deadline and direct child reap; exact Windows `CREATE_SUSPENDED` before attachment, four Job limits and readback, membership, controlled handles/environment, resume only after checks, normal exit and checked wait. Negative tests cover invalid and mismatched format/extension/MIME, corrupt/truncated/bomb/animated media, bad dimensions and source/decoded hashes, malformed/extra frames, crashed/panicked child, timeout, unverified confinement and cleanup, plus first-valid/second-invalid whole-set refusal. No fake-port-only positive. Include safe measured N097 log markers on the actual CI heads. WSL fixture-limited tests must have CI passes, not synthetic substitutions.

If actual Windows API features or security constraints cannot be satisfied inside the named paths/approved dependency pin, STOP **on the branch** with honest report and retain green main. Never change branch protection, migrate to an in-process decoder, claim a green Linux-only implementation, or promote partial work.

## 8. Deliberate-red sensitivity proof

Only after **one exact green two-host technical SHA** on `solo-wo-099-image-r01`, create **exactly one** branch `solo-wo-099-negative-r01` at that SHA. Make **one** isolated production guard mutation in `crates/solo-adapters/src/graphic_image_inspection.rs`: remove only the comparison rejecting a false `frame.output_digest` while keeping test code and all other gates/behavior unchanged. Format, compile and strict Clippy must remain green on both hosts; via **workflow_dispatch** the `n097_09` forged-output case must fail at **test execution** (Ubuntu at minimum, Windows preferably as well). If it fails only at compilation/format, or the forged claim is accepted without a failing assertion, sensitivity is NOT proved: STOP and correct test/load-bearing guard before integration. Record exact negative SHA, single-file diff, run/job IDs and failure. Keep that branch intentionally red, never merge it. Return to the original green technical SHA with the refusal check present.

## 9. Integrate exact green blobs, publish report, never self-accept

Require `main` still at the green Commit B. Promote **exactly** the twelve technical paths from the verified green technical SHA using path-scoped Git restore, not a merge or cherry-pick. Check **index** object IDs, main committed Git blobs and the technical branch match at each of the twelve paths (including exactly the two newly added files); verify `unsafe_boundary.rs` stays the independently green Commit B object. Check the whole staged path list has no extras; commit as one ordinary fast-forward integration commit on `main`, push and obtain a **new, independent two-host `push` CI PASS** at that main SHA. Reconfirm N097 Linux and Windows real host evidence and all old gates at the integration SHA. Both technical and negative branches remain unchanged; no other prior branch moves.

Overwrite **only** `.documents/Reports/execution-report.md` in a final, separate linear report commit. If all gates passed, record `status=COMPLETE` with required outcome **pending Chief Master Architect review**; if any gate fails, record `status=STOPPED` with all positive and negative evidence, without claiming acceptance. Do not embed the report's own future SHA, tree, blob, job or run ID in its content. Push and require fresh successful two-host `push` CI on the exact report SHA. Return report URL, SHA/tree/parents and complete per-commit path/blob ledgers; on-branch + on-main exact CI run/job IDs/events/attempts; Linux/Windows host proofs; dependency/security scanner freshness and warning classification; deliberate-red mutation identity and failing assertion; local WSL pass/fixture limitations; all fourteen original remote refs and any two new refs; preserved Owner ignored-file inventory and scoped task-temp cleanup. Final expected state: one clean checkout, `HEAD=main=origin/main=ls-remote`, empty stash, one remote, four WIP files, **WO-099 still ACTIVE**, NEXT neutral and ROADMAP/OUTLOOK unchanged. Keep historical WO-097/098 STOP status and Stage 03 OPEN; do not release or deploy.

## 10. STOP triggers and truthful nonclaims

STOP on a wrong baseline/Owner handoff, unknown local-data changes, extra permanent file, altered protected file or prior branch, failure of checker B on either host, any unbounded in-process decoder, any Windows worker resumed without confirmed limits/job/handle boundaries, unverifiable process termination/reap, unsafe broadening, arbitrary helper/secret/handle inheritance, actual critical/high applicable security finding, missing/failed mandatory real-host N097 proof, graph/architecture failure, unproven deliberate negative sensitivity, lack of exact branch integration equivalence or failed final main/report CI. Do not turn a STOP into ACCEPT by using an alternate backend or skipping a host. Existing 15-second watchdog is a **deadline**, not an exact scheduling SLA. Linux `RLIMIT_AS` is per-process virtual memory, not aggregate memory; neither OS limit is a complete hostile-code sandbox. No new AVIF correctness, platform library, publication, release, deployment or Stage 03 completion may be claimed.

Only the Chief Master Architect independently reviews the final exact GitHub history and returns `ACCEPT`, `REVISE` or `STOP`; the Implementation Agent cannot self-accept. Architect and assistant remain **GitHub read-only** throughout.
