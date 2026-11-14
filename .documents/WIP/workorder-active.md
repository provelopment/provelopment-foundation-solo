# SOLO-WO-105 R01 — COMPLETE N102 WINDOWS ISOLATION PROOF BY PER-RUN FAULT CONTEXT; PRESERVE APPROVED SVG/GIF CONTRACT

**Authority:** This is a Chief Master Architect-prepared **proposed Owner-issuable artifact**, NOT an authorization merely because it was generated. It becomes `OWNER-ISSUED — SOLO-WO-105 R01` only after the Owner reviews/installs these exact bytes at the sole canonical `.documents/WIP/workorder-active.md` and sends the matching `SOLO-WO-105 R01` activation to the Implementation Agent. **One active order only.** Installation explicitly supersedes the stopped WO-104 ACTIVE; WO-104 remains recorded in existing Git history, never copied into an extra WIP file.

**Required outcome:** `R03C-WINDOWS-N102-ISOLATION-CANARIES-VERIFIED-AND-INTEGRATED`, pending independent Chief Master Architect `ACCEPT / REVISE / STOP`.

**Repository:** `provelopment/provelopment-foundation-solo`. **Only checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (WSL uses this very checkout under `/mnt/c`, no clone or second worktree). **Branches permitted:** `main`, new `solo-wo-105-canary-r01`, new deliberate-red `solo-wo-105-negative-r01`. Existing `solo-wo-104-canary-r01` is read-only retained evidence. **Architect is GitHub READ-ONLY.** Owner alone issues. Implementation Agent alone makes authorized local/GitHub changes; no self-acceptance, PR/merge, release, deploy or publication.

## 1. Independent ruling and settled scope

`SOLO-WO-104 R01` was **STOPPED / NOT ACCEPTED overall**. Its independently verified green main-only Architecture amendment **is a successful completed subdeliverable**: commit `7205027bbb656fe07205b4c54d2575f151d0c173`, Architecture Git blob `f66096ec375a497ac6de9a65cfbeae4c5dc85844`. Binding §6.2I now explicitly requires (future implementation) **safe sanitized static SVG with true vector output**, and **still and fully animated GIF** with timing, frame order, transparency, offsets, disposal/compositing and looping preserved. Do not reverse, postpone indefinitely, rasterize SVG by default or flatten GIF. **Neither codec is presently implemented; they remain refused**; follow-on, separately Owner-issued code package is mandatory after this safety gate. `graphic.favicon` stays PNG-only until separately changed. The full Stage 03, R03.C and R03.C3 remain OPEN; no release, renderer, public asset delivery, AVIF proof, generalized OS sandbox or network/filesystem isolation is claimed.

The green `main` presently has real bounded PNG/JPEG/WebP inspection, but **WO-102's alleged COMPLETE was REVISE** because it omitted mandatory N102 Windows security tests. The red WO-104 technical branch actually contains N102 tests and a Windows-only gate. It is **NOT accepted**:

- WO-104 `C2` at `8ed1e89fd9370550b82695eda234c27d8d7abf58` demonstrated `n102::n102_01`, `02` and `03` passing on real Windows, **but** `n097_10` failed because a process-global atomic injection selector leaked across concurrently running tests.
- WO-104 `C4` at `43616ae5ca46d06084e73d48c75cff8363fb841d` showed N102 cases and `n097_10` passing, **but** the unchanged AST checker rejected `std::thread_local!` as `MacroUnsupported` on both Ubuntu and Windows. `thread_local!`, a process-global selector or any disabling/widening of the architecture checker is forbidden as the correction.
- **Additional independently found cleanup defect:** C4's native `spawn` uses `if injected_v1(9) { return None; }` **after successful `CreateProcessW` but before assigning the suspended process to a Job Object**. Returning there merely drops the owned process/thread handles; closing an *unassigned* job does not terminate or reap the child. The analogous real assignment-failure branch calls `TerminateJobObject` but does not reliably terminate a process that never joined that job; its wait/termination results are not checked. This must be corrected and proven. Do not call `cleanup_verified=true` for an unobserved wait.
- The C4 workflow's required-name pattern `test n102_01...` is inconsistent with actual cargo output `test n102::n102_01...`. Fix the exact qualified-name match and assert exact passing count; do not weaken or skip this gate.

The ONE approved remediation: carry a **closed, immutable selected injection step as per-run input** plus a **per-run reached-step/cleanup record** through the safe supervisor and the three platform paths, and through the native Windows `spawn`, `resume`, `wait_ms` and owned cleanup functions as necessary. No shared mutable injection state, no `thread_local!`, no environment-sourced injection, no generic public worker switches. Production worker calls carry `None` and are non-injected. A test-only, closed fixed entrypoint may select integer steps 1..=11, but untrusted authored bytes, normal CLI, public flags or runtime configuration cannot select them.

**This order authorizes correction, full real host proof, deliberate-red sensitivity and exact-blob integration of the existing N102 test seam ONLY. It does NOT authorize SVG/GIF codecs or Architecture changes.** Ordinary correctable compiler/lint/CI errors within this ledger are iteration tasks, not an automatic STOP.

## 2. Mandatory exact preflight — before any write

From the single Owner checkout, check the remote and local Git object identities; STOP on genuine material discrepancy and preserve Owner data.

| Item | Pinned value |
| --- | --- |
| `HEAD == main == origin/main == git ls-remote origin main` | `0d66aae7e0db34095f71de1d7d055d9b1712b7ed` |
| Main tree and sole parent | `c90476432db0f7d578b79b7635cf87f92c3ccbd1` / `7205027bbb656fe07205b4c54d2575f151d0c173` |
| Canonical WO-104 STOP report | `.documents/Reports/execution-report.md` blob `807cd3fd1920ef575af1a64c4bb4f83676122803` |
| Tracked WO-104 ACTIVE | `.documents/WIP/workorder-active.md` blob `ff4308e61af453f95eec0edd94ab8b8d06a693c1` |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Approved SVG/GIF Architecture | `.documents/Architecture/ARCHITECTURE.md` blob `f66096ec375a497ac6de9a65cfbeae4c5dc85844` |
| SOPs / `AGENTS.md` | Architect `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1`; Implementation `51bb37922cf7fc56952e4ad6155c9a59d3c09036`; AGENTS `6eae1da8c721e13cea0c6689a1665e98fa5a9abb` |
| Root manifest / lockfile / toolchain | `14505be49a281ba0495eabea5ca386548cc36347` / `5a18352becaee99b459a93ec75a9d46ce3ff6c8a` / `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| Current workflow (accepted main) | `.github/workflows/rust-verify.yml` blob `d7d8990ca36207d5da8b919543f4fba2520cb405` |
| Existing native seam / safe host / image adapter | `47fd94aeeb73f303658dbf98e364134faeeafeb2` / `43398847a94faf0cd06b37d2847353aaa4c76d10` / `65b0cb29c1ca80b24b16d6ee20290b040936c1c2` |
| Existing adapter root / CLI / smoke test | `e99f3fa974c22496cd48e938db5ba39050a6702f` / `17221fac3145c9e67e4f0419e4df8a18c5210956` / `c716d8b231cc5642230d64c8a6c79aa096f43313` |
| Frozen AST checker / unsafe guard / graph | `1c99d3482dd2a9bd7b027f5f193e17603976e361` / `57d7579616fc0bab96dcb1e371584f20cfd5d8f9` / `da7d7ee78ffe7f7c8b32b2e10c776350ab2345a2` |
| Frozen app image policy | `bdba09c116a9b0667c4e179668b15168f53e481f` |
| Preserved, red, remote WO-104 technical branch | `solo-wo-104-canary-r01` = `43616ae5ca46d06084e73d48c75cff8363fb841d`, tree `3cc54080e5a5ad2c43a39cb786b99f5c9f5c3e72`, sole parent `23da19fb1b7422990fc5a6b2f1e64bc229201c9e` |

**Refs and local hygiene:** exactly **17 remote branches** before WO-105: `main`, the 15 frozen non-main refs from before WO-104 (including the WO-102 image+negative branches), and `solo-wo-104-canary-r01`. No `solo-wo-105-*` ref yet. One verified worktree, one approved `origin`, empty stash; verify actual status, ignored scratch, local branches, ownership and no unknown dirty path. Owner installation should make only ACTIVE dirty with the precise new Owner bytes. Preserve all historical branches, owner `.temp/solo-wo-073-wip/`, old 098/099/100 patches, WO-103 `n102-seam-wip.patch`, WSL build cache and unrelated content. No use of missing legacy Git objects, no broad deletion.

**Frozen WO-104 source import:** compare the seven approved technical paths from B `7205027b...` to `43616ae5...` and require exactly this set: `.github/workflows/rust-verify.yml`; `crates/solo-adapters/src/graphic_image_inspection.rs`; `crates/solo-adapters/src/lib.rs`; `crates/solo-adapters/src/media_worker_host.rs`; `crates/solo-adapters/src/media_worker_host_native.rs`; `crates/solo-cli/src/main.rs`; `crates/solo-cli/tests/media_worker_host_smoke.rs`. Verify source branch's exact seven Git blob IDs, no unexpected symlink/mode/filename, and clean checkout before scoped import. It is an **unaccepted draft**, not an approved implementation merely because its Git objects are readable.

## 3. Closed permanent-write ledger; authority and exclusions

Only the following nine permanent paths may change:

1. `.documents/WIP/workorder-active.md` — Owner-installed exact issuance, commit A only; do not edit after issuance or neutralize before independent disposition.
2. `.documents/Reports/execution-report.md` — one canonical report-only final commit (or truthful STOP), no archive.
3. `crates/solo-adapters/src/media_worker_host_native.rs` — positively Windows-gated native N102 seam, per-run fault context, checked process/job cleanup, bounded real handle/env probes; existing Linux area unchanged except mechanically required signatures under its positive gate.
4. `crates/solo-adapters/src/media_worker_host.rs` — safe per-run routing/refusal/evidence, exact Windows/Linux/unsupported platform variants, no product code path using an injection selector.
5. `crates/solo-adapters/src/graphic_image_inspection.rs` — only existing closed N102 internal probe protocol if a demonstrated necessary fix, never modify decoder format admission or parent digest guard.
6. `crates/solo-adapters/src/lib.rs` — only minimal safe N102 exports and no additional native/unsafe allowance.
7. `crates/solo-cli/src/main.rs` — only fixed closed internal N102 dispatch if needed; no public CLI functionality.
8. `crates/solo-cli/tests/media_worker_host_smoke.rs` — real N102_01/N102_02/N102_03, parallel noninterference and failure/cleanup tests; retain all existing N079–N097 tests unchanged in meaning.
9. `.github/workflows/rust-verify.yml` — mandatory exact Windows N102 gate, qualified test-name matching and existing verification unchanged.

**Hard read-only:** `.documents/Architecture/ARCHITECTURE.md` (SVG/GIF decision is already approved); `source.rs` AST checker; `unsafe_boundary.rs`; all application/domain policies, `Cargo.toml`, `Cargo.lock`, adapter manifest, root `AGENTS.md`, SOPs, all other files, four-WIP structure, NEXT/ROADMAP/OUTLOOK, initial source readers, dependency graph `graph.rs`, `deny.toml`, tools, pins, existing branches and remote settings. If truly essential permanent code needs an unlisted path, STOP with exact diagnosis and ask Owner; do not improvise. No new dependencies, system installer, compiler/toolchain switch or platform source files. Scratch only in Solo ignored `.temp/` or actual Cline task temp and clean responsibly.

**Git authorization:** one normal forward-only `main` authority commit; two *new named* branches and their ordinary forward commits/push/`workflow_dispatch`; one scoped main technical integration commit from proven exact green source blobs; one final canonical report-only commit; full normal host CI on all relevant SHAs. No force, rebase, reset, history rewrite, broad checkout/discard, cherry-pick/merge, PR, second worktree/clone, tag, release, deploy, git clean or deletion of foreign material. Respect protected branch rules. This order supersedes stopped WO-104 authority only when installed and handed off.

## 4. Sequence A–G; branch iterations until actual acceptance gates

### A — Authority and two-host green baseline

On exact pinned `main`, commit **only** Owner-installed `workorder-active.md` as commit A, sole parent `0d66aae7...`, without normalizing Owner whitespace or altering the frozen SVG/GIF Architecture. Ordinary push and confirm a *new exact-head* `push` GitHub Actions run green on Ubuntu 24.04 and Windows 2025. Inspect paths and readback remote identity. Do not produce a checker-only STOP report.

### B — Rebuild the *correction branch*, not the product

From verified A, create `solo-wo-105-canary-r01` in the same checkout. Import **only** seven listed technical source blobs from retained `43616ae5...` with an exact scoped file operation, record source hashes and diff. **Before committing**, remove the unsupported `std::thread_local!` implementation and every process-global atomic selector; finish C and D, run local gates and commit an ordinary forward-only technical change. No additional bootstrap patch or parallel workspace. Preserve `solo-wo-104-canary-r01` exactly unchanged and inspect the retained draft patch only as optional reference.

### C — Per-attempt parameter, typed evidence and checked native cleanup: mandatory correction

**Pass the selected injection step as a value, NEVER via global state.** The implementation may use an internal `N102FaultContextV1` constructed per invocation, with `requested: Option<u8>` plus `reached: Option<u8>` and explicitly recorded `cleanup_verified`. This is an implementation pattern, not permission for a new public general fault API:

```rust
// Illustrative, not a copy/paste replacement for existing signatures.
struct N102FaultContextV1 {
    requested: Option<u8>, // None on EVERY normal production image run
    reached: Option<u8>,   // set only at the exact native boundary reached
}
impl N102FaultContextV1 {
    fn inject_here(&mut self, boundary: u8) -> bool {
        if self.requested == Some(boundary) {
            self.reached = Some(boundary);
            true
        } else { false }
    }
}
```

Thread this per-run context (or an equally explicit immutable selector + local reached-result) through `run_confined_image_worker_*`, the Linux, Windows and unsupported variants, the safe N102 helper, and `SuspendedWorkerV1::spawn`, `resume`, `wait_ms` or dedicated checked cleanup methods. All normal production callers construct a non-injected context; the test-only closed wrapper explicitly selects 1–11 and rejects `0`/out-of-range as a test request, NOT a silent normal run. If a safe call returns early, the owned per-run record must still report which specific boundary was reached and the actual cleanup outcome; do not infer it from the requested number or rely on a static. Native test-injected fault evidence MUST be distinguishable from a genuine kernel error. Never print or expose raw handles, environment/canary values, private paths or source bytes.

**Mandatory safety fix for `CreateProcessW` and assignment failures:** The old C4 `injected_v1(9) -> return None` is after the suspended child exists and before Job assignment. **Do not return here without terminating that owned process and checking a bounded `WaitForSingleObject`/exit.** A Job Object that has not yet acquired the child cannot kill it merely by closing/terminating the job. On real `AssignProcessToJobObject` failure or failed job membership, terminate the *owned process handle* through a properly gated native operation and confirm wait/exit; if the process is definitely a job member, controlled Job termination is acceptable with a confirmed wait. On injected wait failure (`11`), use a distinct, non-injected checked termination/wait path; the injected refusal must not suppress cleanup itself. All post-create failure paths must transfer/close native ownership exactly once and cannot report successful cleanup without observed completion. If termination or bounded wait cannot be confirmed, refuse explicitly as cleanup-UNVERIFIED and STOP acceptance; never conceal it behind `Option::None` or RAII destructor requests. Windows `CREATE_SUSPENDED` -> limited job verification -> assignment/membership -> resume remains invariant. No fallback uncontained child, no environment/handle broadening. Keep 512 MiB process/job memory, 10 CPU seconds and 15,000 ms wall deadlines.

**Handle probe accuracy:** extra event must be explicitly inheritable yet excluded from exact `[stdin_read, stdout_write]` child allowlist. Validate handle argument conversion without truncation; a reused/ambiguous numeric child handle never counts as exclusion. Use existing vendored `windows-sys 0.61.2` features only and kernel `GetHandleInformation`, plus `std::env` in the actual child. The isolated parent-only environment variable is injected only via an independently launched helper `Command::env`, never through shared runner state. Actual worker must receive the already approved minimal UTF-16 environment and have `SystemRoot`; no accidental parent-only variable. Preserve source/digest/whole-inventory refusal and PNG/JPEG/WebP decode rules unchanged.

**N102 refusal proof:** Keep three actual Windows integration tests with names `n102_01_extra_inheritable_handle_is_excluded`, `n102_02_parent_environment_is_excluded`, `n102_03_native_failures_refuse_and_cleanup`. For N102_03 require every requested step 1–11 to reach its named real native boundary, result in a typed refusal, make zero successful inspection, and prove no orphaned/unreaped child; evidence of an injected result must remain explicitly labeled injected. Do not accept a test that just sets a requested step and assumes it was encountered. Add a focused **noninterference regression** proving an N102 fault does not affect an unmodified N097 image worker concurrently on Windows (or prove with normal parallel `cargo test` plus an explicit repeat); specifically `n097_10` MUST remain passing with ordinary default test concurrency. Do not serialize the entire workspace as a workaround.

### D — Honest local and two-host verification; correct the CI gate

Use WSL in the same checkout for `cargo fmt --all -- --check`, `cargo check --workspace --all-targets --all-features --locked`, strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test -p solo-cli --test unsafe_boundary --locked`, `cargo test -p solo-cli --test check-architecture --locked`, `--self-test`, available N097 cases, plus `cargo check --target x86_64-pc-windows-msvc -p solo-adapters --all-targets --locked` (type-check only; never native Windows runtime proof). Local reflink fixture inaccessibility remains LOCAL-UNVERIFIED, NOT success; hosted CI must run those suites. Do not install MSVC or modify shared system settings.

**Windows CI N102 qualified-name correction:** the existing draft Windows workflow expects `test n102_01...`, but actual Rust names are `test n102::n102_01...`. Update regex to require exact fully qualified `n102::...` names followed by `... ok`, and require the exact passing summary `test result: ok. 3 passed; 0 failed; 0 ignored`. Require each named case and bounded success markers/booleans, no skipped cases, no hidden PowerShell `$LASTEXITCODE` masking and no test-name false positives. Preserve existing pinned checkout, matrix, RustSec/cargo-deny with advisory negative probe, all existing tests and security, real graph `packages=4 targets=33 files=62 edges=68`, 29 self-check fixtures, unsafe-boundary 22/22, N079–N097, release smoke and reproducibility. If one of the three tests must be subdivided, preserve the three required names and add precise extra cases/count assertions, not weaker matching.

Commit/push the technical branch forward and `workflow_dispatch` exact head. Iterate on correctable **in-ledger** compiler/Clippy/runtime/test faults with new ordinary commits and fresh dispatch until the SAME technical SHA is fully green on Ubuntu 24.04 and native Windows 2025. Verify actual host job logs, not just checkmarks: three N102 cases ran and passed; N097_10 passed with concurrency; native resource and cleanup facts observed, no invented host proof. Preserve red intermediate commits; do not force-push or delete branches. The Linux job must remain green; the Windows host alone certifies kernel canaries.

### E — Deliberate-red sensitivity

From the final confirmed green `solo-wo-105-canary-r01` SHA, create **only** `solo-wo-105-negative-r01`. Make ONE bounded test-only negative mutation inside the approved Windows native process creation path: in the closed *isolated N102 helper path only*, replace the minimal-environment handoff with the inherited helper-parent environment while retaining valid handle/job/lifecycle controls. Do **not** mutate any test or normal PNG/JPEG/WebP path, change compiled toolchain/deps, disable a CI stage, or leak canary value. Format and strict lint must pass; on real Windows the **unchanged `n102_02_parent_environment_is_excluded` must fail AT TEST EXECUTION because the canary was observed**, not because of spawn, timeout, no output, missing SystemRoot, parser, lint or checker failure. If this negative is not valid, an equally focused handle-allowlist mutation may be used with exact evidence and Architect-consistent confinement; no mutation without a load-bearing unchanged test. The negative branch stays red, never promoted or used as main source. Record exact mutated file and Git blob, run/job/head, diagnostic and scope.

### F — Exact technical main integration, independent two-host recheck

Return to pinned green `main` at the A descendant, confirm no foreign changes. Scope-copy ONLY the final green technical changes among the seven listed paths from `solo-wo-105-canary-r01`, compare every staged `git hash-object` with that branch, forbid any other tracked differences, keep accepted SVG/GIF Architecture `f66096ec375a497ac6de9a65cfbeae4c5dc85844`, checker `1c99d348...`, guard `57d75796...`, Cargo.lock and dependency pins exact. Make **one** normal forward-only technical commit, push and require new *push* CI at exact main SHA green both hosts, including N102 Windows named cases and all old N097/architecture/security proofs. Do not copy the negative mutation, do not merge/rebase/cherry-pick.

### G — Canonical report and final exact-head CI

Overwrite **only** `.documents/Reports/execution-report.md` with one truthful final report, commit it alone, push and obtain a second new exact-report-head green both-host CI run. Report may use `status=COMPLETE — pending independent Chief Master Architect review` ONLY if A + branch green + actual N102 canaries and cleanup + deliberate-red sensitivity + main integration and report CI all exist and pass. Otherwise `status=STOPPED`, preserve main and identify exact missing proof. Report full SHA/tree/sole parent/changed-path/blob ledger; Actions runs, both job IDs, events/attempts/heads; positive/negative Windows kernel booleans/PIDs and checked cleanup (without private values), source/decoded digests, parallel noninterference, actual labels for injected-vs-kernel failures, job limits and group/descendant cleanup, security and warning notes, N097/N090/N093 nonregression, SVG/GIF frozen architecture, Owner local/scratch preservation, exact approved ignored-path cleanup and remaining LOCAL-UNVERIFIED items. Never write self-future commit/run IDs into the report body; provide later in handoff. Leave ACTIVE frozen for Owner-authorized closure.

## 5. STOP, acceptance boundaries and next format implementation

Hard STOP only for material authority/pin mismatch, unauthorized path, unfixable safety boundary in the fixed ledger, unreviewed dependency/security change, lost/foreign Owner data, a process that cannot be proven terminated and reaped, a Windows host inability to prove actual allowlist/environment/limits/cleanup under the current exact rules, invalid negative sensitivity, or irreparable CI red after authorized forward-only iterations. Ordinary in-ledger errors **require attempts to correct**, not premature STOP. Do not claim Windows kernel evidence from WSL type-checking, or that this is a general-purpose sandbox. Never self-accept, release, publish or deploy.

Upon independent ACCEPT of this bounded N102 correction, the **next separately Owner-issued code-bearing task** must deliver the already accepted SVG and GIF capabilities: static sanitized SVG preserved as immutable vector output and still/animated GIF preserving real frame timing, looping and disposal. This order intentionally does **not** implement those codecs and does not close R03.C3, R03.C or Stage 03. The Owner's accepted SVG/GIF decision is binding and cannot be quietly dropped from the roadmap or actual future implementation.
