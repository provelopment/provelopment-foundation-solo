# Execution Report

**status=STOPPED**

**Order:** `OWNER-ISSUED — SOLO-WO-098 R01` — required outcome `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED` — **not achieved.** Phases 1–3 and the whole local WSL loop succeeded; the Windows half of §7 cannot be built inside the authorized file because two frozen, out-of-ledger gates forbid it. No gate was weakened, no unverified code was promoted, and `main` is left **green**.

## 1. Preflight (all matched)

`HEAD == main == origin/main == ls-remote == 93c2658c92cc5d3377ba267a2122d7b122551c23`, tree `c16aeb10…`, sole parent `816680fc…`; report `7eb21b56…` = `status=STOPPED` / `SOLO-WO-097 R01`; tracked ACTIVE `66f5d61a…` with only ` M .documents/WIP/workorder-active.md` dirty (WO-098 bytes, `946b6162…`, unmodified); NEXT `92fa562b…`; ROADMAP `9d7a8622…`; OUTLOOK `cb9c7a61…`; Architecture `c941c7f9…`; both SOPs; green WO-097 A `e7de2e11…` tree `9ee75648…` parent `b6b4df92…`; 14 refs; `solo-wo-093-r01=0dc45185…`, `solo-wo-094-negative-r01=0e9b3677…`; no `solo-wo-098-*` ref existed; one worktree, one remote, empty stash.

## 2. Commits

| Phase | SHA | Tree | Parent | CI |
| --- | --- | --- | --- | --- |
| A (issue WO-098) | `c85d6a772d8856a04cced9a90b6a2ed1c5c0b999` | `1fd30c7c…` | `93c2658c…` | **red as predicted** — run `43442837445`; Ubuntu `131944579847`, Windows `131944580108`, both failing at *Strict lint* (code still the red WO-097 tree) |
| B (recover green tree) | `ed938267001e4a1b20cc3001af8fafb3c82f40b5` | `837ebe629c742b474f06068b94cfee2d8a59da0b` | `c85d6a77…` | **GREEN** — run `43442905821`; Ubuntu job `131944821076`, Windows job `131944820685`, all original gates |

Commit B restored exactly the twelve §3 technical paths from green A: 10 restored byte-identically and 2 known-unaccepted new files removed, verified path-by-path (`path_mismatches=0/12`, including absence at A); nothing else staged; `git diff --cached --check` clean.

## 3. WSL local loop (established, used, and recorded)

`wsl.exe --list --verbose` → **Ubuntu-26.04**, WSL 2, running. Inside it: `Linux x86_64`, `Ubuntu 26.04.1 LTS`, pre-existing `rustup` and `gcc`; `cc (Ubuntu 15.2.0-16ubuntu1) 15.2.0`. `rustup` auto-installed the repository-pinned **1.99.1-x86_64-unknown-linux-gnu** with `rustfmt` and `clippy` (triggered by `rust-toolchain.toml`; no system-wide switch, no extra package). Verified `cargo 1.99.1`, `rustc 1.99.1`, `git rev-parse --show-toplevel` = `/mnt/c/Users/Sean/Documents/Provelopment/00.foundation-solo`, one worktree, clean tree — the same single checkout, no Linux clone.

At recovery B, inside WSL: `cargo fetch --locked` → 0; `cargo fmt --all -- --check` → 0; `cargo check --workspace --all-targets --all-features --locked` → 0; `cargo clippy … -D warnings` → 0.

Full `cargo test --workspace --all-targets --locked --no-fail-fast` was **not** clean locally, and the reason is environmental, proven by direct log text rather than asserted: the five real-filesystem suites refuse to run because the workflow's fixture roots are absent, e.g. `panicked at crates/solo-adapters/tests/source_read.rs:43: SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem`. Creating those fixtures locally is impossible here: `sudo -n` reports `interactive authentication is required` and `mkfs.btrfs` is absent, so no loopback btrfs can be created without an unapproved installation. Those suites are **LOCAL-UNVERIFIED** and remain covered by the mandatory exact-head CI, which supplies the fixtures (proved green at B).

## 4. Branch work actually completed before the blocker

`solo-wo-098-image-r01` was created from recovery B in the same checkout and the twelve WO-097 technical paths restored from `816680fc…`. The §7 repairs were applied and **verified locally**:

1. **Brace/scope defect (item 1)** — `ImageWorkerConfinementV1::frame_fields_v1` now closes immediately after its `match self.host`, the `impl` closes next, and `child_image_worker_confinement_v1` ×3, `ImageWorkerRunRefusalV1`, `ImageWorkerRunV1` and their impls are module-level. The orphaned `/// The Linux half …` doc comment was moved directly onto `#[cfg(target_os = "linux")] mod linux`, with no blank line after the doc text.
2. **Linux native helper (item 2)** — `terminate_own_group_v1` moved out of `#[cfg(test)] mod tests` into ordinary `pub(crate)` scope inside `mod linux`, behaviour unchanged: own-child PID/group/session checks, `try_wait` brackets, `SIGKILL` error handling and `// SAFETY:` rationale preserved; the bottom `pub(crate) use linux::{…}` re-export stays valid.
3. **Windows Job Object names (item 3)** — all eight `JOB_OBJECT_*_INFORMATION` type uses corrected to `JOBOBJECT_BASIC_ACCOUNTING_INFORMATION` / `JOBOBJECT_EXTENDED_LIMIT_INFORMATION`; the `JOB_OBJECT_LIMIT_*` flag names and `JobObject…Information` class constants are unchanged.
4. **Option/Result (item 4)** — `SuspendedWorkerV1::spawn` now converts with `.ok_or(ImageWorkerRunRefusalV1::SpawnFailed)?`.

Two further real defects surfaced only because the WSL loop existed: the façade's `linux` module imported the native helpers **privately**, so the new module-level supervisors could not see them (now `pub(crate) use`); and the smoke test cannot name `solo_domain` at all, because that crate is not a dependency of `solo-cli` and its manifest is outside the ledger — so a confined `test_inventory_v1` builder was added inside the authorized adapter file, and the fixture references were corrected to the reader's real convention (`hero.png` beneath the `assets/` namespace, matching the adapter's own tests).

Local results on the branch: `cargo check` and strict `cargo clippy` **clean**; `n097_09` (wrong-source, wrong-output, short-output, trailing, child-refused, mislabeled-polyglot — all through the real supervisor, real Linux confinement and framed protocol) **pass**; `n097_10` **pass** with real evidence `n097_deadline=elapsed_ms=15006`; `n097_11` **pass**. `n097_01…08` stop locally at `SourceRead(… Unsupported …)`, the reader's filesystem-capability guard on a reflink-less host — the same refusal the repository's own suites produce without the CI fixture — so those eight are **LOCAL-UNVERIFIED** pending CI.

## 5. The blocker (exact, reproducible, out of ledger)

Adding the Windows Job Object seam to `crates/solo-adapters/src/media_worker_host_native.rs` is refused by gates the order does not authorize me to change:

- `cargo test -p solo-cli --test check-architecture` → `error: unsafe-code: crates/solo-adapters/src/media_worker_host_native.rs: unsafe code outside a #[cfg(windows)] scope is forbidden in this workspace, and outside a positive #[cfg(target_os = "linux")] scope too`.
- `cargo test -p solo-cli --test unsafe_boundary` → `no_first_party_file_outside_the_native_module_uses_unsafe`: `crates/solo-adapters/src/lib.rs: asks for allow(unsafe_code) outside the single approved Windows gate`; and `the_adapter_lint_arrangement_is_exactly_the_approved_one` requires exactly one `#[cfg_attr(windows, allow(unsafe_code))]` in the adapter root (the source reader).

Every ledger-legal arrangement was tried: one `#[cfg(any(linux, windows))]` gate (erases the provable platform gate, making the whole file unscoped and refusing every `libc::` use), a Windows twin module with a second crate-root allowance (fails the "exactly one Windows allowance" assertion), and a module-scoped inner `#![allow(unsafe_code)]` (judged an ungated allowance). None satisfies all three frozen tests. §7 item 3 is therefore **unimplementable** in the authorized file, and Linux-only containment would not meet the required outcome — it would fake the Windows host fact that the order forbids faking.


## 6. State, evidence and cleanup

`main` is at `ed938267…` (green recovery B): tree clean, stash empty, one worktree, one remote, four WIP files, ACTIVE still the Owner-installed WO-098 `946b6162…`, NEXT neutral, ROADMAP/OUTLOOK/Architecture/SOPs/`deny.toml`/toolchain untouched; all 13 non-main remote refs frozen at their preflight SHAs.

Retained, task-owned, each with a cleanup trigger:

- `.temp/solo-wo-098-wip/unverified-attempt.patch` (**244,647 B**) — the complete, never-pushed branch attempt, kept only as audit evidence. Trigger: delete when the follow-up order lands or the Owner declines it.
- `.temp/solo-wsl-target/` — the WSL build cache; holds no source and is disposable at will.
- Local branch `solo-wo-098-image-r01` at the same commit as `main`, **never pushed**; not deleted, because branch deletion is outside this order's permissions.

No AVIF, platform catalogue, public projection, route, release, deployment or Stage 03 closure was attempted; `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. Nothing was measured on Windows, so no Windows runtime fact is claimed anywhere in this report.

## 7. Requested Owner decision

One of these is required before the Windows half can exist:

1. authorise a **second** approved native file (or an explicit exception) for the Windows Job Object seam, leaving `unsafe_boundary.rs` and the source checker unchanged; **or**
2. authorise a specific, reviewed amendment to `crates/solo-cli/tests/unsafe_boundary.rs` and `architecture_checks/source.rs` that recognises a Windows-scoped process seam inside the reserved file — a governance change, not a code fix; **or**
3. explicitly narrow this slice to Linux containment and record the Windows Job Object proof as deferred.

I did not push a failing implementation commit, did not create the negative branch, did not merge, force-push, reset, tag, release or deploy, and did not promote unverified code to `main`. This report claims no completion and is submitted for independent Chief Master Architect `ACCEPT / REVISE / STOP`.
