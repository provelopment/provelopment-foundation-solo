# Execution Report

**status=STOPPED**

**Order:** `OWNER-ISSUED — SOLO-WO-100 R01` — required outcome `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED` — **not achieved.** The Windows handle allowlist and minimal environment required by §5 are implemented and compile clean against both targets, but the two frozen checkers cannot both accept any module declaration of the single reserved native file, so the worker cannot reach a green mandatory gate. Nothing was pushed to a technical branch, nothing was integrated, and no host proof or containment claim is made.

## 1. Preflight — every §2 identity and local pin matched

`HEAD == main == origin/main == git ls-remote origin main == 3af6e00af6c60601b199e5d35611ec773e13f558`, tree `aa86132e848b763ebd6ed2a759565a56f42920c6`, sole parent `bcd18f3f042d234ab65a89c5742173a1b2e3c07e`; only dirt was the unstaged WO-100 ACTIVE (`51dfef89…`), preserved byte-for-byte, never normalized. All **22** baseline blob pins matched, including report `7bb1f9e4…`, tracked WO-099 ACTIVE `c1ac9e85…`, NEXT/ROADMAP/OUTLOOK, Architecture, both SOPs, `AGENTS.md` `6eae1da8…`, root manifest/lock/toolchain/workflow, the accepted `unsafe_boundary.rs` `57d75796…`, AST `source.rs` `df1f8758…`, `graph.rs` `7a5beada…`, adapter `lib.rs`/`media_worker_host.rs`/`media_worker_host_native.rs`, application `graphic_asset_read.rs`/`media_worker.rs`, smoke `22ee67e9…`. Exactly **14** remote refs, all at baseline (093 `0dc45185`, 094-negative `0e9b3677`, 089 `1236915c`, 090 `0eb3fcf6`); no `solo-wo-099-image-r01` or `solo-wo-100-negative-r01` remote ref. Local `solo-wo-099-image-r01 = f6e1137250d1204d588742b49ebce029714763a6`, parent `4b72282519f9193eed4a2409b64b58eec807a824`, base `bcd18f3f…`; `solo-wo-098-image-r01 = ed938267…`; both retained patches exact (`unverified-attempt.patch` 244,647 B, `wo099-branch-work.patch` 180,065 B); `git diff bcd18f3f..f6e11372 --name-only` is exactly the twelve authorised paths. One worktree, one remote, empty stash, four WIP files.

## 2. Commit A — authority installed and green on both hosts

| Phase | SHA | Tree | Sole parent | CI |
| --- | --- | --- | --- | --- |
| A — install WO-100 authority (ACTIVE only) | `e085a4e90857c857aad86c64a0dddc670c559946` | `de1cc3fad3c0726311587179383e83f2ab076173` | `3af6e00a…` | **GREEN**, attempt 1 — run `43453293051` (Ubuntu + Windows) |

## 3. §5 implementation completed and type-checked — then blocked by a checker contradiction

Delivered on the local branch (never pushed) in `330c08e0b105e1e4e8c6e4f80888456ee0d31044`, parent `f6e11372…`, touching only authorised paths:

- **§5.1 allowlist.** `AttributeListV1` RAII owner in the Windows module: documented null-list size probe that must fail with a sane bounded size (`> 0`, `≤ 4096` B), checked `bytes_needed → Vec<usize>` sizing with `usize` alignment and no reallocation, real `InitializeProcThreadAttributeList`, `UpdateProcThreadAttribute(PROC_THREAD_ATTRIBUTE_HANDLE_LIST, …)` over a fixed `[HANDLE; 2]` (stdin read + stdout write only, validated non-null and distinct), and `DeleteProcThreadAttributeList` exactly once in `Drop` — never after a failed initialisation. `STARTUPINFOEXW` with `cb = size_of::<STARTUPINFOEXW>()`, `STARTF_USESTDHANDLES`, `hStdError = hStdOutput`, a live `lpAttributeList`, and `CreateProcessW(…, TRUE, CREATE_SUSPENDED | EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT, env, cwd, &startup.StartupInfo, &mut info)`. Signatures were read from the vendored `windows-sys-0.61.2` source, not guessed.
- **§5.2 environment.** `minimal_environment_v1()` builds a fresh sorted, double-NUL-terminated UTF-16 block admitting only `SystemRoot` (and `WINDIR` when different), each validated as non-empty, ≤ 260, NUL-free, ASCII and absolute, with `PATH`, profile, temp, token, cloud, repository and every ambient value refused; the working directory is bound to the verified executable's own parent.
- **Local evidence** (WSL Ubuntu-26.04, Rust 1.99.1, cache `.temp/solo-wsl-target`): `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean; `cargo check -p solo-adapters --all-targets --locked` zero diagnostics; `cargo check --target x86_64-pc-windows-msvc -p solo-adapters --all-targets --locked` zero diagnostics (type-check only, not Windows runtime evidence); `cargo test -p solo-cli --test unsafe_boundary --locked` **17 passed**. One pre-existing `unused_mut` in `media_worker_host.rs` was corrected.

**The blocker.** The file must be declared in `crates/solo-adapters/src/lib.rs`, and no declaration shape satisfies both frozen checkers:

1. **Twin items (the branch's inherited shape)** — `#[cfg(target_os = "linux")] mod media_worker_host_native;` plus `#[cfg(windows)] #[cfg_attr(target_os = "windows", allow(unsafe_code))] #[path = "media_worker_host_native.rs"] mod media_worker_host_native_windows;` — is refused by the accepted governance guard: the `#[path]` declaration is an evasion and `target_os = "windows"` is not the approved `windows` predicate, so `the_adapter_lint_arrangement_is_exactly_the_approved_one` and `no_first_party_file_outside_the_native_module_uses_unsafe` both fail.
2. **One completed-shape item** — `#[cfg(any(target_os = "linux", windows))]` with both `cfg_attr` allowances — is refused by the unchanged AST checker with twelve errors, the first being `dynamic-loading: crates/solo-adapters/src/media_worker_host_native.rs: dynamic loading is not inspectable and is forbidden (libc::RLIMIT_AS)` and the rest naming `RLIMIT_CPU`, `SIGKILL`, `getpgid`, `getrlimit`, `getsid` and further `libc::*` uses, plus `unsafe code outside a #[cfg(windows)] scope … and outside a positive #[cfg(target_os = "linux")] scope`: with an `any(..)` module item the checker can no longer prove the positive platform scope of the file's inner items.

Every alternative that would resolve this lies outside my authority: `architecture_checks/source.rs` is read-only, a second reserved native file needs an Architecture change, and I may not widen the guard's shape list.

Not performed, therefore not claimed: §5.3 Windows canary tests (extra inheritable handle, environment canary), branch `workflow_dispatch`, N097/N100 host evidence, `solo-wo-100-negative-r01`, integration and final green report CI. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

## 4. State, preservation and cleanup

`main` is at `e085a4e9…` (green Commit A), working tree clean after this report, no stash, one worktree, one remote, four WIP files, WO-100 ACTIVE installed and unmodified, NEXT neutral, ROADMAP/OUTLOOK/SOP/Architecture/protected files identical, 14 remote refs unchanged. Local branches preserved without any history operation: `solo-wo-099-image-r01` now at `330c08e0…` (forward from `f6e11372…`), `solo-wo-098-image-r01` at `ed938267…`; neither deleted, reset nor rebased.

Retained task-owned evidence with cleanup triggers: `.temp/solo-wo-098-wip/unverified-attempt.patch` (244,647 B), `wo099-branch-work.patch` (180,065 B), **`wo100-branch-work.patch` (187,820 B — exact `bcd18f3f..330c08e0`)** and `.temp/solo-wsl-target/`. Owner `.temp/solo-wo-073-wip/` (46,062 B) and all other ignored paths untouched.

## 5. Requested decision — one narrow ruling is needed

1. amend the **AST checker** so the single reserved module may be declared `#[cfg(any(target_os = "linux", windows))]` while every `unsafe`/`libc` use stays in a positive inner item (preferred: it matches §5's own layout and removes the `#[path]` evasion); or
2. amend the **governance guard** to accept exactly the twin declaration with the `windows` predicate and one documented `#[path]` onto the same reserved file; or
3. authorise a **second reserved native file** for the Windows half (an Architecture change).

With any one of these the branch continues from `330c08e0…`: the §5 worker code is written and type-checked on both targets, and needs only §5.3 canaries plus host CI. No gate was weakened, no uncontained decoder was used, no evidence was invented, no red branch was merged, and nothing was released, deployed or self-accepted. Submitted for independent Chief Master Architect `ACCEPT / REVISE / STOP`.
