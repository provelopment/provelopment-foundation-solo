# SOLO-WO-098 R01 — RECOVER GREEN MAIN, REPAIR AND PROVE CONFINED IMAGE INSPECTION

**Authority:** `OWNER-ISSUED — SOLO-WO-098 R01`  
**Matching Owner activation:** `SOLO-WO-098 R01`  
**Required outcome:** `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED`  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Target:** `main`; one isolated correction branch and one deliberately red negative branch expressly permitted below.  
**Status:** FINAL Owner-installable execution scope, not a historical completion or Architect self-issuance.

## 1. Approved Scope — full recovery followed by bounded completion

The Owner's matching handoff authorizes these phases **in order**: preserve failed WO-097 history, restore the technical tree of the last green WO-097 issuance commit using one forward-only recovery commit on `main`, use the **same checkout** and WSL Linux for fast local compilation/tests, correct the existing WO-097 implementation on one isolated branch, obtain independently green Ubuntu and Windows workflow-dispatch evidence plus an intentional failing refusal-sensitivity test, promote precisely the green branch's inspected technical files to `main`, and publish one canonical report. Do not skip the recovery gate or call the prior STOPPED WO-097 accepted.

**No alternative outcome is a successful completion.** If a required condition fails or an unapproved permanent path becomes necessary, STOP with an honest report. In particular, do not suppress CI, remove the image feature, declare an in-process decoder safe, fake Linux/Windows kernel facts, bypass a refusal to get green, force-push, or claim that Windows compilation alone proves Job Object containment.

The scope is **one inert, nonpublishing Entity-source image inspection** over the already accepted `R03.C1` declaration inventory and `R03.C2` confined `GraphicAssetSourceSetV1`. Preserve one complete set across every declared owner including internal, suppressed and unlisted obligations. Real full PNG, JPEG, WebP decode from already-owned confined bytes, exact extension/magic/observed MIME/decoder agreement, ≤20 MiB source, 1–8192 pixels per side, ≤32,000,000 pixels and bounded decoded bytes are the only positive results. Parent verification and single-use worker containment precede all acceptance. A bad second obligation refuses the entire set. Neither the worker nor its result may publish. Preserve the existing `image =0.25.10` three-format pin and its audited lock graph; do not change it to evade defects.

Still outside scope: AVIF correctness, video/WebVTT, platform asset catalogue/artwork, generic role fallback, whole-publication candidacy, derived public assets, route/HTTP/browser behaviour, release, deployment, closing R03.C3, R03.C or Stage 03. The previously accepted N079–N093 behavior must not regress.

## 2. Exact starting state and safety preflight

Inspect from the single checkout **before any permanent write**, and STOP on any unexplained mismatch:

- `HEAD == main == origin/main == ls-remote origin main == 93c2658c92cc5d3377ba267a2122d7b122551c23`; tree `c16aeb10c61e430771ce34e8595496feb95b0b5d`; sole parent `816680fc469bc906afbc31ecaae871359247be9b`. This is a **red** final WO-097 report revision, not a green candidate.
- `.documents/Reports/execution-report.md` Git blob `7eb21b56b80c4f61193e10353da0f06aa32a2ccd`, `status=STOPPED` and order `SOLO-WO-097 R01`.
- Tracked ACTIVE at current HEAD: `66f5d61af25efc7c700a070be7c4d4cdd82ee1e2`. After Owner installs this complete WO-098, the **only** initial dirty path may be `.documents/WIP/workorder-active.md`, containing precisely this order and the matching Owner handoff; do not normalize or overwrite its bytes.
- Neutral NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; unchanged Architecture `c941c7f90cc1b1955f9d60b348a3b1092cafe97f`, Architect SOP `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1`, Implementation SOP `51bb37922cf7fc56952e4ad6155c9a59d3c09036`.
- Previously green WO-097 issuance **A**: commit `e7de2e11509488b813f29a03e25a75f7c474a05d`, tree `9ee75648da0c0c5e98ae06de1e8b5004a0680e0f`, parent `b6b4df924745a2e6c304d50b85bbf07af1be0f24`, green two-host push run `43430492911`. Recovered **technical** blobs must equal this commit exactly; ACTIVE and canonical report need not equal it.
- Broken technical **B4** source commit `816680fc469bc906afbc31ecaae871359247be9b` is retained by history. DO NOT interpret it as a validated implementation. Prior red runs `43435300870`, `43435752257`, `43436065749`, `43436576617` and report red run `43437079537` remain auditable.
- Four WIP files only, no WIP subdirectory; one `origin` fetch/push URL `https://github.com/provelopment/provelopment-foundation-solo.git`, one worktree, empty stash, no unknown staged files. Exactly 14 existing branch refs (main plus thirteen non-main); confirm every non-main SHA before and after. In particular `solo-wo-093-r01=0dc45185110e6136bd63dc7edf774bc52b753c9e`, `solo-wo-094-negative-r01=0e9b3677cf774b9e31ac8fc52605e88112522be6`, and the proposed `solo-wo-098-image-r01` and `solo-wo-098-negative-r01` do not yet exist.
- Preserve ignored/Owner material including `.temp/solo-wo-073-wip/` (previously agent-observed 4 files/46,062 bytes), `architecture-fixtures/`, `toolchain-installer/`, `target/`, unknown Cline logs, and every local pre-existing file. Never run `git reset --hard`, `git clean`, destructive checkout, force-push, rebase, amendment, shadow clone/worktree or broad filesystem cleanup.

## 3. Exact permanent write ledger — no implicit extension

**Allowed documentation:**

1. `.documents/WIP/workorder-active.md`: Commit A, Owner-installed WO-098 bytes **unchanged**; remain ACTIVE after report, pending a later Owner-issued closure.
2. `.documents/Reports/execution-report.md`: overwrite once for the final factual complete-or-STOP report on main; no copies or archives.

**Exactly twelve allowed technical paths, and ONLY these twelve:**

1. `.github/workflows/rust-verify.yml` — count assertions and mandatory N097 gate for actual graph/results only; never relax original gates, triggers, host matrix, security, action pins or permissions.
2. `Cargo.lock` — existing reviewed exact additive PNG/JPEG/WebP dependency graph only.
3. `crates/solo-adapters/Cargo.toml` — pinned `image =0.25.10`, `default-features = false`, exactly `png`, `jpeg`, `webp`; existing minimally needed `windows-sys 0.61.2` Job Object features; no additional direct package or update.
4. `crates/solo-adapters/src/graphic_image_inspection.rs` — contained decoder child, bounded transport, parent verification; this path is a new file relative to green A.
5. `crates/solo-adapters/src/lib.rs` — module registration/exports with the exact pre-existing unsafe boundary.
6. `crates/solo-adapters/src/media_worker_host.rs` — safe process supervision and fixed closed internal mode; restore N079–N093 behavior.
7. `crates/solo-adapters/src/media_worker_host_native.rs` — only approved positive OS-specific native/unsafe Job Object/Linux launch, limit and cleanup seam; every unsafe block locally justified.
8. `crates/solo-application/src/graphic_image_inspection.rs` — pure inward policy and whole-set refusal; new relative to green A.
9. `crates/solo-application/src/lib.rs` — exact outward re-exports of new inward policy.
10. `crates/solo-cli/src/main.rs` — one fixed internal child dispatch with unchanged public CLI.
11. **`crates/solo-cli/tests/architecture_checks/graph.rs`** — explicitly authorized in this order: closed adapter root/closure change required for image; ensure no weakened graph tests. This resolves the out-of-ledger defect in stopped WO-097; previous change is NOT retroactively accepted.
12. `crates/solo-cli/tests/media_worker_host_smoke.rs` — real host tests and deliberate negative-sensitivity assertion.

`crates/solo-cli/tests/check_architecture.rs` is **not** the location of the approved dependency allowlists and is not writable. All earlier files/source-read, manifest, AVIF protocol, security policy, toolchain, Architecture/About/SOP/AGENTS, ROADMAP/NEXT/OUTLOOK, prior branches, test fixtures and `.documents/WIP/` extras are read-only. No new permanent path without a replacement Owner authorization.

**Version control permissions:** ordinary linear commits and fast-forward pushes of only named paths on `main`, creation/push of precisely `solo-wo-098-image-r01` and `solo-wo-098-negative-r01`, branch switching in the **same checkout**, scoped restore of exactly the above twelve known tracked technical paths from named historical commits, existing workflow `workflow_dispatch` on the two new branches. No PR, merge, force-push, reset, branch deletion, tag, release, deployment or external provider. Temporary task artifacts may live only under Solo `.temp/` or the Cline-supplied task temp locations (§6A); clean only positively task-owned residue and report any retained data. Do not install Windows MSVC Build Tools, change global Windows toolchains or touch a parent/sibling repository.

**WSL setup permission, newly granted by this Owner-issued order:** check the existing installed WSL Linux distro first. If needed, install *only* user-scoped official Rust `1.99.1` toolchain with rustfmt/clippy using the distro's existing rustup, or official rustup bootstrap if absent. If the Linux C linker is absent on an Ubuntu/Debian WSL distro, install *only* the distro's `build-essential` package and its required package-manager dependencies using the distro's existing authenticated apt/sudo; no unrelated apt upgrades, no extra decoder binary, no elevated Windows installer, no new Git checkout or system-wide Rust version switch. Record precise installed packages/commands/versions. If privilege, distro or download access prevents local WSL setup, do not fake local checks: use existing branch CI for both-host diagnosis, report the limitation, and still require every final GitHub safety gate; never perform an unapproved installation. Approved network use is limited to authenticated GitHub operations, existing documented Cargo/security fetches and the exact official WSL/toolchain/package prerequisites stated here.

## 4. Commit A — issue corrective authority, without concealing red CI

On `main`, stage **only** the Owner-installed `.documents/WIP/workorder-active.md`. Verify one `M` path, byte equality with Owner-provided file and no unexplained unstaged changes. Commit and fast-forward push with sole parent `93c2658c92cc5d3377ba267a2122d7b122551c23`; read back GitHub commit/tree/blob. **The existing CI is expected to remain red here because code is still broken**. Record run and job statuses truthfully. Do not waste time retrying it or call this issuance green. NEXT is already neutral; do not modify it.

## 5. Commit B — restore a stable technical baseline using forward history

While clean on `main` after Commit A, use a **path-scoped restore from green A** for exactly the twelve paths in §3. For paths absent at green A, restoring from that tree removes only those two known unaccepted new files; confirm they contain no Owner/untracked modifications first. Example (from the verified Solo root, in the **current** main worktree):

```powershell
$green = 'e7de2e11509488b813f29a03e25a75f7c474a05d'
$technical = @(
  '.github/workflows/rust-verify.yml', 'Cargo.lock',
  'crates/solo-adapters/Cargo.toml',
  'crates/solo-adapters/src/graphic_image_inspection.rs',
  'crates/solo-adapters/src/lib.rs',
  'crates/solo-adapters/src/media_worker_host.rs',
  'crates/solo-adapters/src/media_worker_host_native.rs',
  'crates/solo-application/src/graphic_image_inspection.rs',
  'crates/solo-application/src/lib.rs',
  'crates/solo-cli/src/main.rs',
  'crates/solo-cli/tests/architecture_checks/graph.rs',
  'crates/solo-cli/tests/media_worker_host_smoke.rs'
)
git restore --source=$green --staged --worktree -- $technical
# Before committing, verify *each* technical blob equals $green, including absence at $green.
# Stage NOTHING beyond these twelve known technical paths.
```

This is a deliberate, authorized **forward-only content restoration**, not a branch reset, history rewrite or claim that red commits disappeared. Verify exact path ledger, all twelve technical paths/absences match green A and no unrelated path changes; `git diff --cached --check` must pass. Commit once on `main`, push and require a **fresh successful push run at exactly recovery Commit B**, Ubuntu and Windows, all original architecture/security/reproducibility/N079–N093 gates. If recovery run is red, STOP with a factual canonical report; do not start the image branch. The canonical STOP report remains in history (and working tree) until the eventual new report.

## 6. Fast local build/test loop — WSL first, never wait for repeated GitHub push failures

In Windows PowerShell inspect `wsl.exe --list --verbose`; determine the **actual installed Linux distro**, version, access and compiler availability. Verify that it can see the *same* Solo checkout (typically `/mnt/c/Users/Sean/Documents/Provelopment/00.foundation-solo`) and that WSL's `git rev-parse --show-toplevel` resolves that single mounted root, not a Linux clone. No other worktree. Windows-side Git performs all commits and branch switches; WSL is used for build/test on the same checkout, never concurrently with a Windows Git operation. Keep compiled `target/` or `CARGO_TARGET_DIR` beneath allowed Solo `.temp/` or the actual Cline task-temp; no duplicate source tree.

Use the exact pinned Linux toolchain; a Windows `cargo.exe` found on PATH is **not** a Linux toolchain. From **inside the WSL Linux shell** and the verified mounted Solo root:

```bash
uname -s
command -v rustup; command -v cargo; command -v cc || command -v gcc
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy --no-self-update
rustc +1.99.1 --version
cargo +1.99.1 --version
cargo +1.99.1 fetch --locked
cargo +1.99.1 fmt --all -- --check
cargo +1.99.1 check --workspace --all-targets --all-features --locked
cargo +1.99.1 clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo +1.99.1 test --workspace --all-targets --locked --no-fail-fast
cargo +1.99.1 test -p solo-cli --test media_worker_host_smoke --locked n097 -- --nocapture --test-threads=1
```

Run commands in this order; fix each **code/compile/lint/test failure** before advancing. A demonstrably WSL-specific prerequisite or filesystem privilege failure (for example, an unavailable local mount capability) may be recorded as LOCAL-UNVERIFIED and deferred to the mandatory exact-head CI proof; distinguish it by direct logs, never mislabel a real code failure as environmental. Inspect and preserve meaningful stderr/stdout; never mask an exit with PowerShell piping, `|| true`, skipping a test, Clippy allowance, ignored errors or a blanket lint suppression. Run local Windows `cargo check --workspace --all-targets --locked` and Windows `cargo clippy` as an **additional diagnostic** if the native environment can execute them, even without `link.exe`; record if a build-script/proc-macro link prevents that. A successful WSL Linux test **does not** prove Windows FFI, Windows Job Object runtime or Windows CI; those remain mandatory real Windows-host gates. A local Windows linker installation is **not** approved here. Avoid repeatedly pushing code that WSL `cargo check` already reports as broken.

WSL alone does not provide native Windows MSVC execution. For platform-specific Windows failures use the **isolated branch** `workflow_dispatch` after local Linux checks are green, or a pre-existing local native Windows linker if genuinely present. Cross-target diagnostics (if independently available) are supplemental, never Windows runtime proof. No `cargo-xwin`, MinGW, Visual Studio install, floating `stable` or alternate dependency graph is permitted merely to get green.

## 7. Isolated correction branch — explicit repairs, then real two-host proof

After recovery B succeeds, `git switch -c solo-wo-098-image-r01` from **recovered main**, still in the same checkout. Recover the saved WO-097 technical implementation into the branch by the same **twelve-path-scoped** `git restore --source=816680fc469bc906afbc31ecaae871359247be9b --staged --worktree -- <technical paths>`; check the exact twelve-file diff against recovery B. **Do not** restore old WO-097 ACTIVE, NEXT, STOP report or unrelated WIP paths. The restored initial code is known broken: fix it locally BEFORE pushing any implementation branch commit. Correct every post-repair compiler error in the authorized files only.

**Mandatory first repairs, grounded in the actual exact-head compiler errors:**

1. **`media_worker_host.rs` bad brace boundary around former lines 2533–2707.** `ImageWorkerConfinementV1::frame_fields_v1` must close **immediately after its `match self.host`**, and the `impl ImageWorkerConfinementV1` must close next. The three target-gated `child_image_worker_confinement_v1` functions, `ImageWorkerRunRefusalV1`, `ImageWorkerRunV1` and their impls are *module-level*, not nested inside `frame_fields_v1`. Correct skeleton (adapt surrounding preserved code, don't paste a second definition):

   ```rust
   impl ImageWorkerConfinementV1 {
       pub const fn matches_profile_v1(&self) -> bool { /* preserve verified rule */ }
       pub fn frame_fields_v1(&self) -> String {
           match self.host {
               ImageWorkerHostV1::Linux => format!(
                   "host=linux as_soft={} as_hard={} cpu_soft={} cpu_hard={} pid={} pgid={} sid={}",
                   self.address_space_soft, self.address_space_hard,
                   self.cpu_soft, self.cpu_hard, self.pid, self.pgid, self.sid
               ),
               ImageWorkerHostV1::Windows => format!("host=windows job=member pid={}", self.pid),
           }
       }
   }
   // The three cfg-gated child_image_worker_confinement_v1 functions go HERE,
   // at module scope, followed by both ImageWorkerRun* declarations/impls.
   // Preserve the original #[cfg(target_os = "linux")] mod linux below.
   ```

   The existing `/// The Linux half ...` doc comment was separated from `mod linux` by the new block; move that doc comment directly in front of the actual `#[cfg(target_os = "linux")] mod linux` declaration, with no orphan blank line after doc text. Do not move the preexisting Linux probe module into the new struct's impl.

2. **`media_worker_host_native.rs` Linux native helper is inside `#[cfg(test)] mod tests`.** Move the whole existing `terminate_own_group_v1` implementation to ordinary `pub(crate)` scope **inside** its existing positively gated `mod linux`, **outside** its `#[cfg(test)]` tests module. Preserve its own-child PID/group/session/parent-group checks, checked `try_wait` brackets, `SIGKILL` error handling and `// SAFETY:` rationale. Keep the bottom `#[cfg(target_os = "linux")] pub(crate) use linux::{...,terminate_own_group_v1,...};` valid. No `pub` widening beyond `pub(crate)` and no general group-signalling API.

3. **Windows Job Object FFI names in `media_worker_host_native.rs`.** The actual `windows-sys =0.61.2` **struct** names are `JOBOBJECT_BASIC_ACCOUNTING_INFORMATION` and `JOBOBJECT_EXTENDED_LIMIT_INFORMATION`, while **limit flag** names remain `JOB_OBJECT_LIMIT_*`. Replace the two invalid struct imports and every corresponding type use (including `size_of`, locals, zeroed structures) with the exact `JOBOBJECT_*` identifiers. Keep information-class enum/constant spellings `JobObjectBasicAccountingInformation` and `JobObjectExtendedLimitInformation`. Inspect the exact pinned crate definitions on Windows CI; no fabricated alias or new crate. Reference: https://docs.rs/windows-sys/0.61.2/windows_sys/Win32/System/JobObjects/index.html .

4. **Windows Option/Result mismatch in `media_worker_host.rs` near former line 6510.** `SuspendedWorkerV1::spawn(...) -> Option<SuspendedWorkerV1>`. Convert with `.ok_or(ImageWorkerRunRefusalV1::SpawnFailed)?`, **not** `.map_err(...)`. Preserve the suspended-before-assign/readback/resume invariant and exact cleanup. Do not convert a failed spawn to success.

5. **Graph ledger now explicitly authorized:** keep `crates/solo-cli/tests/architecture_checks/graph.rs` exact closed additions for the six approved external adapter roots and all audited 37 dependency nodes, with no unverified wildcard, weakened negative fixture, changed version or unapproved transitive. Do not hardcode `edges=68` merely from `cargo metadata`: run the real-tree architecture check and independently match counts on both CI hosts, retaining every refusal fixture. Recalculate and tightly pin counts in the existing workflow only when supported by the passing checker.

6. **Further errors are expected.** Examine *all* next diagnostic messages, especially Windows native handle signatures, WSL compiler errors, stderr/pipe closure and worker lifetime. Fix only within §3 and prove behavior; don't assume all 27/29 errors are caused by one brace. `matches_profile_v1` on Windows is not itself a Job Object attestation: actual parent-side specific-job membership, limit readback before `ResumeThread`, and confirmed process wait are required. `IsProcessInJob(current_process, null_job)` proves membership in **some** job only and is not sufficient alone to prove membership in the intended owned job. Do not join a pipe-writer or reader after an unconfirmed reap, including on pre-resume error paths, or falsely claim a timeout cleanup succeeded because a kill request returned true. If a vulnerability or unprovable containment is discovered, STOP rather than produce media success.

**No new decoder/security policy:** retain exact `image =0.25.10`, only PNG/JPEG/WebP; trusted parent reads no untrusted media headers before child confinement unless a separately proven bounded safe-byte shape predicate is explicitly reviewed; never decode in the parent or permit out-of-limit unsigned arithmetic, animation, malformed/trailing payload acceptance, arbitrary image paths or a fallback. Every complete result must match real source digest, observed format/MIME, decoded dimensions, output length/digest and verified host facts. Child panic, wrong hash, invalid frame, overrun, timeout, bad process membership/limits, failed reap, unverified cleanup, bad second image and every unsupported source reject the **whole** candidate.

When WSL is operational, require **local WSL format/check/strict Clippy/required tests green** before each technical branch push. If WSL or a genuinely host-unsupported local test remains unavailable after the bounded setup above, record direct environmental evidence, treat that local gate as UNVERIFIED, and use the isolated-branch exact-head CI for the missing proof; do not present a remote check as a local run. In either case, commit only §3 technical paths on `solo-wo-098-image-r01` and fast-forward push the branch. Dispatch the existing `rust-verify.yml` with `workflow_dispatch` at that exact branch ref, **never** infer branch CI from a push (push trigger is `main` only). Verify the run's `head_sha`, `event=workflow_dispatch`, attempt, Ubuntu and Windows jobs, Clippy, test, N097 real safe-process/format records, N079–N093 non-regression, security refresh, architecture test/negative self-test, reproducibility and skip/warning ledger. If a host exposes a new in-ledger defect, correct forward on the same branch, local-WSl check first, push and dispatch **a new exact SHA**; preserve failed runs. Do not promote code until **both** hosts green at the **same** technical SHA with real image inspection and refusal evidence.

Useful exact commands (use the installed authenticated `gh` CLI or equivalent existing GitHub API client, no new auth secrets):

```powershell
gh workflow run rust-verify.yml --repo provelopment/provelopment-foundation-solo --ref solo-wo-098-image-r01
gh run list --repo provelopment/provelopment-foundation-solo --workflow rust-verify.yml --branch solo-wo-098-image-r01 --event workflow_dispatch --limit 10
gh run view <actual-run-id> --repo provelopment/provelopment-foundation-solo --json headSha,headBranch,event,status,conclusion,jobs
```

No run is green if a required host safety case is absent, skipped or only a fake-port assertion. A failed `cargo test --workspace` or a warning suppressed to green is a STOP/revise trigger, not a passed check.

## 8. Required mutation-negative proof — separate preserved red branch

Only from the final **green exact technical branch** SHA, create `solo-wo-098-negative-r01` in the **same** worktree. Make **one** explicit mutation in `crates/solo-adapters/src/graphic_image_inspection.rs`: disable just the comparison that rejects a worker's forged `frame.output_digest` when it does not equal the parent-recomputed `received_digest` in `validate_image_worker_run_v1` (formerly `if received_digest != frame.output_digest { return ... OutputDigestMismatch; }`). Do **not** modify any tests, worker output construction, format handling, protocol, limits, cleanup, workflow, graph checker or other gate; make the minimal syntactically valid change, preserving compilability and format. Existing `n097_09_every_worker_refusal_and_forged_claim_is_refused` must then fail at its **`wrong-output`** assertion, after build/strict lint, on Ubuntu and Windows (both preferred and expected). If the guard proves redundant and the test does not fail for that specific reason, STOP — do not manufacture a different red by changing a test. Push only the negative branch and `workflow_dispatch` the same workflow at that exact negative SHA; report run/job logs and intentional test failure, not a green job. Preserve that red branch only as audit evidence and restore the single checkout to the clean unmutated green technical branch with zero mutation in its Git tree.

## 9. Promote exact verified technical code to main and publish canonical result

Only after recovery B green + exact green branch on **both** hosts + deliberate test-stage RED negative run:

1. Switch (same checkout; no second worktree) to the clean, recovered `main` at recovery B. Restore **only** the twelve §3 technical paths from the **verified green branch technical SHA**, stage and prove each resulting Git blob/absence equals that SHA exactly. In particular do **not** restore any branch's ACTIVE, NEXT, report, red mutation, other document or Owner file. Commit one ordinary forward-only **integration commit C**, push `main`, require a new exact-head `push` run with successful Ubuntu/Windows jobs. Re-prove N097 host evidence and N079–N093, architecture and all security/repro gates on the integrated `main`, without assuming branch CI stands in for integration CI.
2. Overwrite **only** `.documents/Reports/execution-report.md` on `main` in final **report Commit D**, sole parent integration C. Report `status=COMPLETE`, outcome pending **independent** Chief Master Architect `ACCEPT/REVISE/STOP` **only if** every required gate is proven. Provide commit and tree SHA ledger, per-file blob identity, branch refs, all four phases, WSL local command outputs (or justified no-local environment), exact image/security/host facts, intentional red mutation logs, candid warning and unverified limits, and preservation/cleanup. Never embed the report's own unknown SHA, tree or CI ID into report contents. Require a new exact-head `push` run with successful Ubuntu/Windows jobs; give actual report SHA/run/job IDs in chat.
3. Final `HEAD == main == origin/main == ls-remote main == report D SHA`; empty stash, clean checkout, single remote and worktree, four WIP files, WO-098 ACTIVE unchanged, NEXT neutral, ROADMAP/OUTLOOK/Architecture/security/toolchain unchanged. All prior thirteen non-main refs frozen; newly authorized technical and negative refs preserved. No release, PR, merge, tag, deployment or Stage 03 completion.

If **any** required green or negative gate is unreachable, publish one truthful `status=STOPPED` canonical report on the last safely established `main` revision, identify precise code/CI limitation, do not promote unverified code, and keep the green recovery state intact. Do not turn incomplete tests, unverified source correctness or safety into `COMPLETE`.

## 10. Mandatory evidence and role boundary

Every CI claim needs exact run URL/ID, `head_sha`, event, job status, relevant log text and revision. Include real Linux RLIMIT_AS/RLIMIT_CPU/session/group and parent wait/cleanup evidence, real Windows `CREATE_SUSPENDED`→Job Object four limits and readback→assign/membership→resume→wait evidence, one each real PNG/JPEG/WebP complete decode with known pixel/output digest and MIME, expected malformed/oversize/mismatch/animated/timeout/refusal matrix, all-declared-inventory failure and unchanged EFS owner lineage, actual security advisory freshness, strict Clippy and architecture counts. No general security sandbox, race-free Linux PID/PGID ownership, aggregate process resources, descendant reap, AVIF correctness, platform catalogue, public output or release claim is authorized.

The Implementation Agent may write **only** within this Owner-issued scope and the exact single Solo checkout; it must execute required Git operations itself and cannot self-approve. The Chief Master Architect retains **read-only** GitHub access and will independently verify the final evidence. No future work, scheduled monitoring, direct Cline control or unobserved local execution is implied by this artifact. A later Owner-issued closure will separately clear ACTIVE and reconcile NEXT/ROADMAP; this order leaves unfinished R03.C in ROADMAP regardless of a successful inert image-inspection gate.
