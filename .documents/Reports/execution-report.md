# Execution Report

**status=STOPPED**

**Order:** `OWNER-ISSUED — SOLO-WO-099 R01` — required outcome `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED` — **not achieved.** The §5 governance repair is done, committed on `main` and independently green on both hosts. The image work was reconstructed on its isolated branch with §6 items 1–7 applied and type-checked for both targets, but the mandatory handle-allowlist and minimal-environment proof (§6 item 8), the real Windows host tests (§6 item 9), branch CI, the deliberate-red sensitivity run and the integration were **not** completed, so nothing was promoted and no positive containment is claimed.

## 1. Preflight — every §2 identity matched

`HEAD == main == origin/main == ls-remote == ee4e238df4f5d276eaf604a4321d777e9cd1e83c`, tree `5acc6d57…`, sole parent `ed938267…`; report `41acd865…`; tracked WO-098 ACTIVE `946b6162…` with only the unstaged WO-099 ACTIVE dirty; NEXT `92fa562b…`; ROADMAP `9d7a8622…`; OUTLOOK `cb9c7a61…`; Architecture `c941c7f9…`; both SOPs; root manifest `14505be4…`, lock `b1ee56da…`, toolchain `9ec55b3b…`, workflow `e1b5ca6e…`; `unsafe_boundary.rs` `b88e7b42…`, AST `source.rs` `df1f8758…`, `graph.rs` `7a5beada…`; adapter `lib.rs` `1550d34f…`, native seam `eacf51c5…`, façade `7eed6e15…`; graphics source-read `6499421d…` (`crates/solo-application/src/graphic_asset_read.rs`), AVIF/media-worker boundary `13811c58…` (`crates/solo-application/src/media_worker.rs`), smoke `22ee67e9…`. All **14** remote refs at baseline, including `solo-wo-093-r01=0dc45185…`, `solo-wo-094-negative-r01=0e9b3677…`, `solo-wo-089-r01=1236915c…`, `solo-wo-090-r01=0eb3fcf6…`; no `solo-wo-099-*` ref exists. Preserved inputs verified present and unchanged: local-only `solo-wo-098-image-r01 = ed938267…`, `.temp/solo-wo-098-wip/unverified-attempt.patch` **244,647 B**, `.temp/solo-wsl-target/` cache, Owner `.temp/solo-wo-073-wip/` **46,062 B**. One worktree, one remote, no stash, four WIP files.

## 2. Commits and CI actually obtained

| Phase | SHA | Tree | Sole parent | CI |
| --- | --- | --- | --- | --- |
| A — install WO-099 authority (ACTIVE only, bytes `c1ac9e85…`, 8 disclosed Owner hard-break whitespace findings, nothing normalized) | `6b1cf128506814b7be8d775b3a799fd65a0ef105` | `79acc46cd91e11c2f419e6b0873e91af10ed8df1` | `ee4e238d…` | **GREEN**, attempt 1 — run `43449633686`; Ubuntu `131968093226`, Windows `131968092703` |
| B — §5 guard repair (`unsafe_boundary.rs` only, blob `57d75796…`) | `bcd18f3f042d234ab65a89c5742173a1b2e3c07e` | `07444135333d9592fa255f5d84c6734989597e17` | `6b1cf128…` | **GREEN**, attempt 1 — run `43450140048`; Ubuntu `131969835547`, Windows `131969836007` |

Commit B replaced the brittle file-wide `matches("…allow(unsafe_code)…").count() == 1` assertion with a closed, syn-parsed **per-declaration** contract (`arrangement_v1`) that reads each `mod` item's own attributes: crate-wide `#![deny(unsafe_code)]` required and zero crate-wide allowances; `mod source_read_native` with exactly one Windows allowance and no Linux one; `mod media_worker_host_native` with exactly one Linux allowance and accepted in either the **legacy** shape (Linux gate, no Windows allowance — the shape the checkout still has) or the **completed** shape (`#[cfg(any(target_os = "linux", windows))]` availability plus exactly one Windows allowance on that same module); no allowance on any other item, no `#[path]` evasion, no duplicated or `any(..)`-gated allowance. New load-bearing negative tests inject each of those, and the positive test proves the new shape is accepted **while the real checkout stays legacy**. Local proof: `cargo fmt --all -- --check` clean, strict Clippy clean, `unsafe_boundary` **17 passed**, `check-architecture --self-test` **`self-check=passed cases=29`**, real-tree checker **`architecture=legal packages=4 targets=33 files=60 edges=41`**. `architecture_checks/source.rs` was not touched.

## 3. Isolated branch — reconstructed, partly corrected, deliberately not published

`solo-wo-099-image-r01` was created at the green Commit B in the same checkout and exists **local only**; it was never pushed and no `workflow_dispatch` was spent on it.

- `4b72282519f9193eed4a2409b64b58eec807a824` — the twelve §3 technical paths restored path-scoped from the known WO-097 B4 source `816680fc…`, then the inspected WO-098 patch applied **after** auditing it: its `diff --git` list is exactly five of the twelve authorised paths (the other seven were staged when it was captured), and `git apply --check` passed against the B4 content before it was applied. No `git apply` of anything unaudited, no shadow clone, no extra path.
- `f6e1137250d1204d588742b49ebce029714763a6` — §6 items 6 and 7: `create_limited_job_v1() -> Option<OwnedHandleV1>` now **closes the job and returns `None`** whenever `SetInformationJobObject` or the four-value readback fails, so no caller can hold an unverified job; `SuspendedWorkerV1::spawn` acquires every handle through an `OwnedHandleV1` guard (`Drop` closes exactly once) so no fallible step can leak, wraps the created process and thread handles, and on failed assignment/membership terminates the parent's own job and **waits 5 000 ms on the owned process** before any handle is dropped; the constructed worker carries `limits_verified: true` only because construction now requires a verified readback; and `resume()` refuses unconditionally when `!self.limits_verified`.

Items 1–5 (brace/scope boundary, `terminate_own_group_v1` in `pub(crate)` Linux scope outside `#[cfg(test)] mod tests`, `JOBOBJECT_*_INFORMATION` types, `.ok_or(...)?` on the suspended spawn, private `super::native_worker` re-exports plus the confined `test_inventory_v1` builder and the `assets/`-namespace fixture references) are the verified WO-098 corrections, re-applied and re-checked here.

Local evidence for the branch, this run: `cargo fmt` clean; `cargo check -p solo-adapters --all-targets --locked` **zero diagnostics** on Linux; and — new this order — `cargo check --target x86_64-pc-windows-msvc -p solo-adapters --all-targets --locked` **zero diagnostics** after installing only the user-scoped `rust-std` component for that target with the existing WSL rustup (no system install, no cross linker, no `cargo-xwin`/MinGW; a type-check only, never a Windows runtime claim). The N097 suites were **not** re-run on this branch in this run, so no test result is claimed for it here.

## 4. What stopped it (§6 item 8 / §10)

The order requires proof of an explicit inheritance allowlist — `PROC_THREAD_ATTRIBUTE_HANDLE_LIST` containing only the child's stdin/stdout handles — together with a deliberately minimal child environment, and it states that `bInheritHandles = TRUE` with only the parent ends cleared is **not** sufficient. The branch code still has exactly that insufficient arrangement: it inherits the default environment and relies on cleared parent ends. Implementing `STARTUPINFOEXW` + `InitializeProcThreadAttributeList`/`UpdateProcThreadAttribute`, a constructed environment block and the associated Windows negative tests, then proving them on both hosts, is the remaining mandatory work; it was not reached in this run. Because §10 stops the run on any worker resumed without proven handle boundaries, and because claiming a positive containment result without it would be false, I stopped instead of pushing a branch that could only misrepresent its own guarantees.

Not performed, and therefore not claimed: branch `workflow_dispatch`; the `solo-wo-099-negative-r01` deliberate-red mutation; exact-blob promotion to `main`; any Windows or Linux N097 host proof for this order; the workflow graph-count pins; the final green two-host report CI. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

## 5. State, preservation and cleanup

`main` is at `bcd18f3f…` (green Commit B), working tree clean, no stash, one worktree, one remote, four WIP files, WO-099 ACTIVE installed and unmodified, NEXT neutral, ROADMAP/OUTLOOK/Architecture/SOPs/`deny.toml`/toolchain untouched; all 14 remote refs unchanged from preflight. The local branch `solo-wo-099-image-r01` is preserved at `f6e11372…` with the earlier local `solo-wo-098-image-r01` at `ed938267…`; neither is deleted, reset or rebased.

Retained task-owned material, with triggers: `.temp/solo-wo-098-wip/unverified-attempt.patch` (244,647 B, untouched), `.temp/solo-wo-098-wip/wo099-branch-work.patch` (**180,065 B**, the exact `bcd18f3f..f6e11372` branch diff, kept as audit evidence — delete when the follow-up order lands or the Owner declines), and `.temp/solo-wsl-target/` (build cache, disposable). Owner `.temp/solo-wo-073-wip/` (46,062 B) and every other ignored path were left alone.

## 6. Requested decision

The remaining scope is implementable inside the existing twelve-path ledger and needs no new file — it needs the handle-allowlist/environment work plus its Windows host proof. Options:

1. re-issue this same order (or a successor) so the branch continues from `f6e11372…` with §6 item 8 and item 9 as the first task; or
2. accept a narrower outcome that records the Windows seam as unproven and integrates only Linux-verified behaviour — which would **not** satisfy the required outcome and is offered only for completeness.

No gate was weakened, no uncontained decoder was used, no host evidence was invented, no red branch was merged, and nothing was released, deployed or self-accepted. Submitted for independent Chief Master Architect `ACCEPT / REVISE / STOP`.
