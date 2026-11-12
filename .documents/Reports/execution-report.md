# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-076 R01` — R03.C3: establish and prove the Linux native limit-readback seam on an isolated CI branch (no child launch).
**Matching handoff:** `SOLO-WO-076 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `BLOCKED`.** The readback seam is implemented, committed to the preserved technical branch and **executed for real on `ubuntu-24.04`** — the three `N076` kernel tests ran and passed — but the branch cannot be made green on both hosts without editing a file this order forbids touching. The unchanged `rust-verify` workflow asserts a hardcoded approved-dependency-edge count of `40`, and the one `solo-adapters → libc` Linux edge this order mandates necessarily makes it `41`. `LINUX-NATIVE-READBACK-PROVEN` is therefore **not** claimed, nothing was merged, and `R03.C3` / `R03.C` / Stage 03 stay **OPEN**.

## 1. Preflight — `S076-01` PASS

| Anchor | Required | Observed |
| --- | --- | --- |
| Baseline `main` | `d76f24b886208be415a6376455c3367bd8f848e7` | exact |
| Baseline tree | `1540ea788c911995a3abcb34840267c8ba8cfd50` | exact |
| Canonical report blob | `432c925fd21e51f0b4fd7ff1deec9b6123bb04dc` | exact |
| Prior ACTIVE blob | `6806a9002a7ce825f029da26868266c43294ec18` | exact |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| AST source checker | `df1f8758b201214e5a4066823e83449f9c6cef6f` | exact |
| Unsafe-boundary guard | `5c59a71bf4bf31808dde45be18fa123ba3fb7c44` | exact |
| Architecture fixtures | `dd8bcfe2a0c99acbe9ae5b9a6aeccb4993156ad2` | exact |
| Adapter manifest | `6109224cd3e3e89487884631f2b07c68ad8149c3` | exact |
| Adapter `lib.rs` | `4c5732983ce5a0ced0a7ebc9e25af2e24149f5ee` | exact |
| Root `Cargo.lock` | `2f9e53576df117f2020ae064d6973304f20833f1` | exact |
| Dependency graph checker | `268fcf9a33d7e41e304173586952c8341acfbda4` | exact |
| Workspace graph test | `69fa6837f52a1dff72a4a3bb0057021bf544aa4e` | exact |
| Workflow (declares `workflow_dispatch:`) | `eb78dad240bb7adb3e0ef9535870897176c99843` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| Historical branches | `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` | unchanged |
| Previous report CI | run `43195232774` both hosts success | confirmed |

One checkout, one worktree, one remote `origin=https://github.com/provelopment/provelopment-foundation-solo.git`, `main` clean apart from the Owner-installed ACTIVE replacement (working blob `9ba2a7937396a3032a393a31500909c2e8158586`), exactly four WIP files, both product paths `media_worker_host_native.rs` and `media_worker_host.rs` absent, and no local or remote `solo-wo-076-r01` before creation.

> **Disclosed discrepancy (not caused by this run):** the order asks to verify two inherited Owner archives at `.temp/` (`mingw.zip`, `rustup-init.exe`). `.temp/` contains **no** top-level files (`temp_top = 0`), so neither archive is present there. The four inherited research files in `.temp/solo-wo-073-wip/` are present and unmodified at **4 files / 46,062 bytes** (`Cargo.lock.prepared`, `Cargo.toml.prepared`, `media_worker_host_native.rs`, `NOTES.md`). I did not create, delete, overwrite or stage any of it.

## 2. Issuance — `S076-02` PASS

| Item | Value |
| --- | --- |
| Issuance commit | `5b085d4a0d9b259da80021adb4d8c133462d4616` |
| Tree | `d30d7a5afb255010578e353d624b37adfcf83d1b` |
| Parent | `d76f24b886208be415a6376455c3367bd8f848e7` |
| ACTIVE blob | `9ba2a7937396a3032a393a31500909c2e8158586` |
| Changed paths | `.documents/WIP/workorder-active.md` only |
| Push | fast-forward; `origin/main == 5b085d4a…` |
| CI | `rust-verify` run **43197813264** — `131113175631` (`ubuntu-24.04`) success, `131113176051` (`windows-2025`) success |

## 3. Technical branch and its exact ledgers

One branch only, created in the sole checkout from the issuance commit after verifying it existed neither locally nor remotely: **`solo-wo-076-r01`**, preserved at its final SHA `8e1d5645835e50850d575de1c22d463c1efe7980` (local == `origin/solo-wo-076-r01`). No merge, cherry-pick, PR, rebase, force-push or history rewrite occurred.

| Revision | Tree | Paths |
| --- | --- | --- |
| `f1bd7ccd72d144e4e9c403f1813c5e83605f7168` | `b0581cfc2146c2489172a7dacedda5e32d2ecf82` | the seven authorized paths |
| `8e1d5645835e50850d575de1c22d463c1efe7980` | `363f0f99b6666e18c1961992646959b59fcb39da` | `media_worker_host_native.rs`, `unsafe_boundary.rs` (correction) |

Branch changed-path ledger — exactly the seven authorized paths, nothing else:

1. `Cargo.lock` — one added line `"libc",` inside the `solo-adapters` dependency list.
2. `crates/solo-adapters/Cargo.toml` — one added line under the existing `[target.'cfg(target_os = "linux")'.dependencies]` group: `libc = { version = "=0.2.191", default-features = false }`.
3. `crates/solo-adapters/src/lib.rs` — the `#[cfg(target_os = "linux")]` and `#[cfg_attr(target_os = "linux", allow(unsafe_code))]` declaration of `mod media_worker_host_native;`, with its documentation.
4. `crates/solo-adapters/src/media_worker_host_native.rs` — new; the seam and its three unit tests.
5. `crates/solo-cli/tests/unsafe_boundary.rs` — the two-allowance rules and the present-file real-tree test.
6. `crates/solo-cli/tests/architecture_checks/graph.rs` — `libc` added to the approved adapter normal roots.
7. `crates/solo-cli/tests/workspace_graph.rs` — the legal fixture edge and the renamed exact-root test.

Nothing in `architecture_checks/source.rs`, `check_architecture_fixtures.rs`, the Charter, `source_read_native.rs`, `deny.toml`, the toolchain, the workflow or any plan file was touched.

**Cargo graph delta.** Exactly one new direct edge, `solo-adapters → libc`, Linux-only; `libc 0.2.191` was already a locked node, so **no** package, version, source or checksum changed and no duplicate `libc` or `windows-sys` appeared. Still four workspace crates, one `windows-sys 0.61.2`, one `libc 0.2.191`, with `rustix`, `serde`, `serde_json` and the Windows features unchanged. Local `cargo metadata --locked` exited 0.

**Native `unsafe` inventory.** Exactly **two** `unsafe` blocks, both inside the positively gated `#[cfg(target_os = "linux")] mod linux`, each with an adjacent `// SAFETY:` note: the `libc::getrlimit` call and the `MaybeUninit::assume_init` that follows the zero-status check. One FFI function in total (`getrlimit`), two compile-time-fixed selectors (`libc::RLIMIT_AS`, `libc::RLIMIT_CPU`), one raw C type (`libc::rlimit`). No `setrlimit`, `setsid`, `kill`, `prctl`, `fork`, `Command`, child process, handle, path or output channel. The crate keeps `#![deny(unsafe_code)]`; there are now exactly two module-scoped allowances — the untouched `#[cfg_attr(windows, allow(unsafe_code))]` on `source_read_native.rs` and the new Linux one on the seam — and `source_read_native.rs` is byte-identical.

## 4. Branch CI — two runs, both disclosed

**Run `43199345760` at `f1bd7ccd` (first iteration, failed).** Two genuine defects in my own new code, both inside the allowed paths:

* `ubuntu-24.04` job `131118372707`, step 16 *Strict lint (warnings are errors)*: `error: casting to the same type is unnecessary ('u64' -> 'u64')` at `crates/solo-adapters/src/media_worker_host_native.rs:135` — `raw.rlim_cur as u64`, where `rlim_t` is already `u64`.
* `windows-2025` job `131118372036`, step 17 *Test (workspace, all targets, locked)*: `the_guard_refuses_an_allowance_that_is_not_the_single_approved_one` and `no_first_party_file_outside_the_native_module_uses_unsafe` failed at `unsafe_boundary.rs:667` and `:361`. Cause: the allowance gate was matched against the comment-and-string-stripped view, which erases the `"linux"` literal, so the approved Linux form looked ungated.

Both were corrected in `8e1d5645`: the cast is gone (the fields pass through unchanged, and the SAFETY note now says `rlim_t` is `u64` on Linux), and `has_ungated_allowance` removes the two exact approved attribute texts from the **raw** source first and then checks only the residual code, so a comment or string mention still never counts and every unapproved, combined, negated or crate-wide form is still refused.

**Run `43200373681` at `8e1d5645` (final, failed at the real-tree architecture gate).**

| Host | Job | Reached | Result |
| --- | --- | --- | --- |
| `ubuntu-24.04` | `131121910857` | step 18 *Architecture checker on the real checkout (Linux)* | **fail** — `##[error]approved edge count changed: edges=41` |
| `windows-2025` | `131121911134` | step 19 *Architecture checker on the real checkout (Windows)* | **fail** — the same pinned count |

Everything before those steps passed on both hosts: exact-version assertions, the workspace package-set gate, `cargo fmt --check`, strict Clippy with `-D warnings`, and `cargo test --workspace --all-targets --locked` including every pre-existing suite and guard test. Steps after them (release build, CLI smoke, both tripwires, the forbidden-import refusal and the byte-for-byte reproduction) were therefore **skipped**, so `S076-07` is not met.

## 5. The blocker, stated exactly

The checker's own real-tree output is `architecture=legal packages=4 targets=32 files=58 edges=41` — the architecture check **accepted** the new file, the new edge and the two-allowance arrangement. The failure is the workflow's separate assertion that the approved edge count is exactly forty:

* `.github/workflows/rust-verify.yml:660` — `grep -q 'edges=40' "$RUNNER_TEMP/arch-real.txt" || { echo "::error::approved edge count changed: …"; exit 1; }`
* `.github/workflows/rust-verify.yml:672` — the PowerShell twin, `if ($text -notmatch 'edges=40') { … }`

The mandated Linux-only `libc` edge raises that count to 41 by construction. Clearing it needs either the workflow's pinned number (forbidden: "no changes to … workflow") or a change to the checker's fixture semantics (forbidden: "do not weaken or edit it in this order"), and both lie outside the seven authorized paths. That is the order's own STOP condition — an error in scope that would require another path — so I stopped instead of editing an unauthorized file to manufacture a green run. **No workflow, checker, fixture, manifest or policy was modified to make the count pass.**

## 6. `N076` evidence — the readback is real

The three native unit tests are ordinary in-file `#[cfg(test)]` tests, so they ran as part of the unchanged `cargo test --workspace --all-targets --locked` on `ubuntu-24.04`. Verbatim from the `131121910857` log:

```
test media_worker_host_native::linux::tests::n076_01_getrlimit_rlimit_as_and_rlimit_cpu_observe_the_real_kernel ... ok
test media_worker_host_native::linux::tests::n076_02_ceilings_keep_their_full_range_and_repeat_consistently ... ok
test media_worker_host_native::linux::tests::n076_03_a_rejected_status_is_a_typed_refusal_not_a_partial_observation ... ok
```

* **`N076-01`** — both `getrlimit(RLIMIT_AS, …)` and `getrlimit(RLIMIT_CPU, …)` returned successful typed observations on the real kernel, the selectors are named in the test, and no struct or mock was substituted. The test also asserts `soft <= hard` for both, which the kernel guarantees.
* **`N076-02`** — soft and hard are reported as separate fields, `RLIM_INFINITY` survives as `u64::MAX` with no truncation or false finite inference, a distinct pair stays distinct, and two consecutive observations compare equal. No fixed CI-host limit value is asserted, and no process limit is changed.
* **`N076-03`** — a **source-path proof**, not an injected syscall failure: `ceiling_status_ok` is proved to accept only `0`, and the two selectors keep distinct closed refusals. A rejected call returns before `assume_init`, so a partly initialised observation cannot become a success. No privilege was used and no fault injected.
* **`N076-04`** — on `windows-2025` the module is not compiled at all (the crate root declares it only for `target_os = "linux"`), no `n076_*` test appears in that job, all unchanged suites and guard tests passed, and no Windows kernel readback or containment is claimed.

**Not claimed:** this observes a process's own ceilings. It proves **no enforcement** — nothing about a future child starting under 512 MiB or 10 CPU seconds, no watchdog, no process group, no descendants, no Windows Job Object, no filesystem/network/syscall isolation, no decoder, no media correctness and no publication. `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1` and `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1` remain `false`.

## 7. Gates

| Gate | Status |
| --- | --- |
| `S076-01` preflight | **PASS** — §1, with the archive discrepancy disclosed |
| `S076-02` issuance, readback, both-host CI | **PASS** — §2 |
| `S076-03` one branch, seven-path bounded delta, FFI limited to gated `getrlimit` | **PASS** — §3 (no process launch, stress or Windows FFI) |
| `S076-04` one new Linux direct `libc` edge, no new package/version, source-read protections intact | **PASS** — §3 |
| `S076-05` two exact allowances, AST plus independent guard accept only those two | **PASS** — the real-tree checker reported `architecture=legal`, and both guard suites passed on both hosts after the `8e1d5645` correction |
| `S076-06` `N076-01`–`N076-03` executed on Ubuntu, `N076-04` Windows compilation/regression | **PASS** — §6 |
| `S076-07` both hosts pass **all** `rust-verify` steps at the exact branch SHA | **NOT MET** — the pinned `edges=40` assertion aborts both jobs at steps 18/19; later steps skipped |
| `S076-08` branch preserved without merge, code-free clean `main`, historical refs preserved | **PASS** — §8 |
| `S076-09` one report-only main commit, readback, both-host CI at report HEAD | **PASS for the report commit** — see §8 and the chat handoff |
| `S076-10` accurate vocabulary, no containment/decoder claim, Stage 03 open, no self-acceptance | **PASS** — `BLOCKED` |

## 8. State, inventories and cleanup

**`main` is code-free and unchanged since issuance.** `HEAD == main == origin/main == 5b085d4a0d9b259da80021adb4d8c133462d4616` with tree `d30d7a5afb255010578e353d624b37adfcf83d1b` — the issuance commit itself — an empty `git status --porcelain`, and `crates/solo-adapters/src/media_worker_host_native.rs` absent. The only difference between `main` and the branch is the seven authorized paths. This report is the one report-only commit on top of it.

**Branches.** `main`, `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` unchanged, plus the preserved proof branch `solo-wo-076-r01` `8e1d5645…` (local == remote, **not** deleted, **not** merged). Two mainline commits in this order, one path each: the ACTIVE issuance and this report.

**Local gates actually executed on this workstation.** `cargo fmt --all -- --check` exit 0 (after `cargo fmt --all`), `cargo metadata --locked` exit 0, `git diff --check` exit 0. There is still no MSVC linker or Linux target here, so **no** local compile, test, lint or Clippy result is claimed; every build/test statement above comes from the two-host CI logs.

**Security posture.** `deny.toml`, `rust-toolchain.toml` and the workflow are unmodified; `cargo-deny 0.20.2` passed on both hosts in the issuance run, while the final branch run's security step came **before** the failing architecture step and therefore did pass on both hosts in run `43200373681`. No dependency was installed, no target added, no provider, secret or setting touched, and no unauthorized file was written. The four WIP files are intact and `.temp/solo-wo-073-wip/` is unchanged at 4 files / 46,062 bytes, with its cleanup trigger unchanged (delete only when a superseding order completes the native scope). All task scratch (`.temp/r1.md`, `r2.md`, `r3.md`) was removed after the report was assembled.

**What the Architect must decide.** The seam is correct, minimal, positively gated, proven to compile on both hosts and proven to execute two real kernel readbacks on `ubuntu-24.04`. The one thing it cannot do is satisfy a hardcoded approved-edge count that only the Owner can authorize changing. The smallest sufficient decision is one of: (a) authorize a workflow edit raising the pinned `edges=40` to `41` (and its PowerShell twin) as a separate order, or (b) authorize the checker's own `workspace_graph`/`graph` fixtures to derive the count rather than pin it. Nothing else about this order is blocked, and the branch is intact for that follow-up.

The Implementation Agent does not self-ACCEPT, has merged nothing, and has begun no other work order.
