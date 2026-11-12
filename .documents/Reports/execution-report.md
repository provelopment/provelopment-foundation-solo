# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-083 R01` — prove one fixed Linux `RLIMIT_AS` virtual-address-space reservation refusal in a trusted pre-exec-limited child; isolated technical-branch proof only.
**Matching activation:** `SOLO-WO-083 R01`. **Date (ISO):** 2026-11-12.

**Outcome: `LINUX-RLIMIT-AS-RESERVATION-REFUSAL-PROVEN`** — on the isolated technical branch `solo-wo-083-r01` at `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`. **No code was merged into `main`.** `main` remains at the issuance commit with its WO-082 sources byte-identical.

**What was proven.** One *trusted, descendant-free* Linux child, launched through the **unchanged** `setsid`/`RLIMIT_AS`/`RLIMIT_CPU` pre-exec installer, first read its own kernel ceilings and verified **all four halves exactly** (`as_soft = as_hard = 536870912`, `cpu_soft = cpu_hard = 10`) and its own positive `pgid == pid`, and only then made **exactly one** fallible capacity reservation — `Vec::<u8>::new().try_reserve_exact(805306368)` (768 MiB), strictly larger than its whole 512 MiB address-space ceiling. That single request returned `Err`, so the kernel/allocator refused it. The child printed one fixed denial line, flushed it and exited zero; its owning parent re-validated every value, confirmed a normally exited and reaped leader inside its 15 000 ms deadline, and emitted one bounded `media_host_as=limit_enforced … reserve=denied …` record. Windows refused both new internal modes as `HOST_UNSUPPORTED` before any child existed and without attempting any reservation.

**The actual Linux evidence (technical-branch run, `ubuntu-24.04`):**

```
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4519 pgid=4519 wall_ms=10
n083_direct_as_child=exit=71
```

**Allocation-API semantics — precisely what this does and does not mean.** `try_reserve_exact` is a *fallible capacity reservation*. It asks the allocator for capacity and returns `Result`; it does **not** initialise, write, read, index, fill, size, resize or otherwise touch any of the requested bytes, and it does **not** make memory resident. The negative therefore proves that one over-ceiling **virtual-address-space capacity request** was refused under the child's exact limit. It proves nothing about pages, resident memory, RSS, the OOM killer, aggregate memory across processes, allocator behaviour other than Rust's default allocator on this host, or any allocation *stress*. Nothing was retried, enlarged, or replaced by a second or larger request.

**Nonclaims — remain OPEN:** no page touching, resident-memory or RSS claim; no `RLIMIT_CPU` exhaustion (this child never burns CPU); no aggregate or process-tree memory quota; no descendant cleanup, process-group signalling or tree sweep; no hostile-child resistance; no generic process runner or public capability; no Windows Job Object or Windows process control of any kind (refusal only); no filesystem, network or credential sandbox; no immutable executable identity; no AVIF parse/decode or media verification; no publication authority, release, deployment or Stage 03 completion. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and the media-worker publication/correctness flags remain `false`.

## 1. Preflight — PASS

Sole Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; one worktree; exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`; exactly four `.documents/WIP/` files; the only dirtiness the Owner-installed ACTIVE replacement; `solo-wo-083-r01` absent both locally and on `origin`.

| Anchor | Required | Observed |
| --- | --- | --- |
| `main` / `origin/main` | `ae6ee66ac312a18779ca11c5833be3a38aa3e528` | exact |
| Baseline tree | `b3ef2966e41383f3f1accd3b53805203b9dc18d0` | exact |
| Canonical report blob | `510ed11ec67baa4b2493db60de0c67dd5cc5ca13` | exact |
| Previously tracked ACTIVE | `76e7723e0471e500233b7471906a9cb32b0f48e3` | exact |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `rust-verify` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact, both `edges=41` pins |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / root | `47d732f2…` / `1550d34f…` | exact |
| Linux native seam | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| **Façade before this order** | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | exact |
| CLI root | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| **Host integration test before this order** | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | exact |
| Unsafe guard / graph / workspace graph | `b88e7b42…` / `7a5beada…` / `25a91b2c…` | exact |
| `solo-wo-081-r01` · `079` · `076` | `cacc0f9b…` · `380393dd…` · `40a40bab…` | unmoved |
| Historical branches | `4371af21…`, `e6162cdf…`, `d270f95c…` | unmoved |
| Accepted CI history | integration `43250654169`, report `43252259722` | as recorded |

The installed ACTIVE was 117 lines beginning `**OWNER-ISSUED — SOLO-WO-083 R01**`, object identity `9f23156531357eebd7590e62dfad6f3ce26146ff`. Inherited `.temp/` holds `architecture-fixtures/`, `solo-wo-073-wip/` (4 files / 46 062 bytes) and `toolchain-installer/`, with **no** top-level files; `target/` exists. All were preserved untouched, were never executed, staged, copied or treated as source.

## 2. ACTIVE-only issuance on `main` — PASS

| Commit | Tree | Parent | Paths | New ACTIVE blob |
| --- | --- | --- | --- | --- |
| `5247a769a1d5d39510b7a8d9165d1451b75c4e1f` | `26cfb82040bbd96b0485b456a7f82a3ba85e55d7` | `ae6ee66ac312a18779ca11c5833be3a38aa3e528` | 1 — ACTIVE only | `9f23156531357eebd7590e62dfad6f3ce26146ff` |

One commit, one parent, one changed path, no source and no report: the canonical report blob stayed `510ed11e…` and every implementation blob stayed as listed above. Ordinary fast-forward push; GitHub readback returned the same SHA, tree, parent, single file and ACTIVE blob.

**Issuance CI — run `43255471637`, `event=push`, `head_sha=5247a769…`, conclusion `success`:** `verify (ubuntu-24.04)` job `131312381852`, `verify (windows-2025)` job `131312382747`, **33 configured steps each, 0 failures** (Ubuntu skipped exactly its 9 Windows-only steps, Windows its 14 Ubuntu-only steps). The technical branch was created **only after** that gate, from that exact issuance SHA, and its ACTIVE is still the issued blob `9f231565…`.

## 3. Authorized path ledger — exactly two files on the technical branch

| Path | Before (baseline blob) | After (branch blob) | Status | Numstat |
| --- | --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0` | **`90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f`** | M | `780 / 5` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1846af8459c21063bd2c2d7e112c05b9baf599ae` | **`30dffd08dfbd1c8f79edc1c35b7fe092653ea91b`** | M | `212 / 0` |

| Commit | Tree | Parent | Paths |
| --- | --- | --- | --- |
| `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2` (`feat: prove one over-ceiling RLIMIT_AS reservation is refused (SOLO-WO-083)`) | `b299b005c03657344213632eb0e0e4623023f694` | `5247a769a1d5d39510b7a8d9165d1451b75c4e1f` (issuance) | exactly the two `M` paths, nothing else |

The five deletions are the replaced `use super::{…}` import list of the Linux module, whose six new lines add only the new names; **no pre-existing statement, test or assertion was removed or weakened**, and no third path was touched. `cargo fmt --all` was run locally and changed **only** these two files, so `cargo fmt --all -- --check` passes before every push. Every permanent path stayed byte-identical: workflow `e1b5ca6e…`, `Cargo.lock` `b1ee56da…`, adapter manifest `47d732f2…`, adapter root `1550d34f…`, native seam `d10151e4…`, CLI root `b3c89ad0…`, unsafe guard `b88e7b42…`, graph checker `7a5beada…`, workspace graph `25a91b2c…`, Charter, SOPs, README/AGENTS, all three WIP plans, `deny.toml`, toolchain and every other first-party source. No file, module, crate, feature, dependency edge or target was added; the legitimate graph is still `packages=4 targets=33 files=60 edges=41`.

**First-party `unsafe` invariant.** The one Linux native file keeps exactly its four positively gated, locally justified `unsafe` expressions (blob `d10151e4…`, unmodified). The two authorized files add **zero** executable `unsafe`, no `libc`, no `mmap`, no allocator FFI, no raw pointer, no custom allocator, no new syscall and no new process abstraction. `#![deny(unsafe_code)]` in the adapter root is untouched.

## 4. What the two files now do

**`as-child` (Linux only).** Reads `RLIMIT_AS` and `RLIMIT_CPU` through the existing native functions; requires **all four halves** to be exactly `536870912`/`536870912`/`10`/`10`; reads its own `pgid` and requires a positive `pid == pgid`; and **only then** executes exactly one `Vec::<u8>::new().try_reserve_exact(805_306_368)`. There is no `set_len`, `resize`, indexing, filling, read, write, loop, retry, sleep, CPU burn, source read, network use or descendant. On `Err` it prints and flushes exactly one fixed line

```
SOLO-MEDIA-HOST-AS-V1 as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=<u64> pgid=<u64> request_bytes=805306368 reserve=denied
```

and returns status `0`. If the reservation unexpectedly returns `Ok`, the vector is **dropped without a byte being touched** and the child returns its own distinct status `74` with no output at all. Any readback or group mismatch refuses with status `71`/`72`/`73`, again with no output and **no reservation attempt** — which is exactly what the direct-child negative observes.

**`as-parent` (Linux only).** Re-executes this repository's own trusted `solo-cli` in `as-child` using the **unchanged** `install_linux_child_limits_v1` pre-exec boundary; drains both pipes to the existing 1 024/512-byte caps; tracks time from before `spawn`; refuses on timeout after killing and reaping through its own `Child` handle; requires a normally exited and reaped leader with status zero; strictly parses exactly one `AS` line with the marker, the eight fields in fixed order, plain unsigned decimal values, exactly one trailing newline and exactly `request_bytes=805306368` plus the literal `reserve=denied`; re-validates all four ceilings, a positive pid and `pgid == pid`; requires `wall_ms` strictly below 15 000; and only then emits one bounded line

```
media_host_as=limit_enforced as_soft=… as_hard=… cpu_soft=… cpu_hard=… request_bytes=805306368 reserve=denied pid=… pgid=… wall_ms=…
```

A timeout, abnormal or signal exit, failed reap, absent/over-cap/non-UTF-8/malformed/duplicate/extra text, `reserve=granted`, wrong request size, limit or group mismatch, or an out-of-deadline interval is one typed refusal with **no** success line and no child output, OS error text or path. Existing `N079`/`N081` semantics are reused unchanged; no old helper was relaxed.

**Windows and every other non-Linux target.** The closed argument classifier and the whole pure parser/validator/evidence layer compile and are exercised, but `as-parent` and `as-child` deterministically return status **3** with `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` and empty stdout, **no child is created and no reservation is attempted** (no Linux module, no `try_reserve_exact`, no native call is compiled there). Unknown, extra, repeated or misplaced tokens return `INTERNAL_MODE_INVALID`, the reserved flag in any other position still routes to the unchanged public CLI, and there is no new public subcommand or help output.

**Critical safety reasoning.** The one request is deliberately larger than the *whole* verified ceiling (by 256 MiB), is made only after the ceiling and group proof, is fallible, occurs exactly once, touches no byte, and is discarded in both branches; an unexpected success can therefore never become resident memory or accepted evidence.

## 5. Proof cases and complete CI — all PASS

**Final branch CI — run `43258192188`, `event=workflow_dispatch`, `head_sha=ccc3ca8fb37d52b405a2854e4962fb79aff06ab2` (exact), conclusion `success`.** `verify (ubuntu-24.04)` job `131322039061` and `verify (windows-2025)` job `131322039733`, **33 configured steps each, 0 failures**; Ubuntu skipped exactly its 9 Windows-only steps and Windows exactly its 14 Ubuntu-only steps, so no applicable step was skipped and none failed. `push` triggers only `main`, so `workflow_dispatch` against this exact branch was used, and the run's `head_sha` was verified before any result was trusted. **This order had no red CI iteration**: the first branch push compiled, linted and tested green on both hosts, so no corrective commit was needed.

| Case | Host | Directly observed result |
| --- | --- | --- |
| `N083-01` | Ubuntu | `n083_01_the_child_reserves_over_the_ceiling_and_is_refused` **ok**; `n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4519 pgid=4519 wall_ms=10` — one real `Err` from the single 768 MiB request under the 512 MiB ceiling, normal exit 0, reaped leader, `wall_ms` far inside 15 000 |
| `N083-02` | Ubuntu | `n083_02_a_direct_as_child_refuses_before_the_reservation` **ok**; `n083_direct_as_child=exit=71` — empty stdout, empty stderr, prompt refusal from the **prior** limit check, never reaching the request |
| `N083-03` | both | `n083_03_every_wrong_reservation_token_is_refused_by_the_real_binary` **ok** on both hosts; six pure cases **ok** on both hosts: closed/additive tokens, exact-line acceptance, seventeen malformed/duplicate/reordered/`reserve=granted`/wrong-size/ceiling-size/extra/missing refusals, every abnormal/un-reaped/non-UTF-8/over-deadline classification, fixed bounded evidence wording, and the over-ceiling magnitude |
| `N083-04` | Windows | `n083_04_both_reservation_modes_are_unsupported_before_any_child_exists` **ok** — both modes status 3, `HOST_UNSUPPORTED`, empty stdout, prompt, no child; **zero** occurrences of `n083_01_`, `n083_02_`, `n083_as=`, `n083_direct_as_child=`, `n076_`, `n079_01_the_real`, `n079_04_a_direct`, `n081_01_the_wall_parent`, `n081_02_the_termination` or `n081_03_a_direct` in the Windows log |
| `N083-05` | both | `N076-01..03` green with real `getrlimit` readback; `N079-01` `n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4475 pgid=4475 wall_ms=11`; `N079-03/04/05/06` green; `N081-01` `n081_wall=media_host_wall=terminated … pid=4490 pgid=4490 signal=9 reaped=1 wall_ms=2007`, `N081-02/03/04/05` green; public CLI bytes unchanged (`preflight=ok …` at `0`, `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` at `2`); architecture, self-check and negative guards intact |
| `N083-06` | both | complete `rust-verify`: fmt, strict Clippy `-D warnings`, all locked all-target tests, `cargo-deny 0.20.2` with a fresh RustSec refresh, real-tree architecture, refusal self-test, tripwires, release build and CLI smoke, Ubuntu forbidden-import byte restore, two clean release builds — all green on both hosts |
| `N083-07` | — | exactly the two implementation paths on the preserved branch, no code on `main`, issuance and report each one path, all old branches unmoved, one worktree, four WIP files, inherited scratch preserved (§7) |

## 6. Architecture, security and downstream gates

| Gate | Ubuntu `131322039061` | Windows `131322039733` |
| --- | --- | --- |
| Pinned `1.99.1`, `cargo fmt --all -- --check`, strict clippy, locked all-target tests | pass | pass |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Refusal self-test | `self-check=passed cases=29` | `self-check=passed cases=29` |
| `cargo-deny 0.20.2` component security gate | archive sha256 `9f12ed4c…`, RustSec `af7b3ec33460b8e2de8d422e16d628518891a519` fetched `2026-11-12T16:24:05Z`, advisories/bans/licenses/sources all ok | archive sha256 `975a2214…`, same RustSec commit fetched `2026-11-12T16:24:30Z`, all ok |
| Advisory negative probe (RUSTSEC-2026-0009) | refused as designed | host-inapplicable |
| Test-failure tripwire | pass, injected failure detected | pass |
| Release build + CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | `release_smoke=verified accepted_exit=0 refused_exit=2` |
| Forbidden import/bypass refusal with guaranteed restore | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | Ubuntu-only step |
| Two clean release builds, raw bytes | `reproducibility=verified`, `clean_release_bytes=654096` | Ubuntu-only step |
| Raw checkout `Cargo.lock` SHA-256 | `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` | `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e` |

The two raw lock hashes differ **only** because of checkout line endings; the single Git blob identity `b1ee56da60f36536548586528db371b7cbe508e4` is unchanged on both hosts, and the security gate re-verified that hash before and after itself. This is CI test-building, never release or deployment authority.

## 6A. Gate table

| Gate | Status | Basis |
| --- | --- | --- |
| Preflight and identities | **PASS** | §1 — baseline, tree, report and ACTIVE blobs, all §2 anchors exact; branch absent; only the installed ACTIVE dirty |
| ACTIVE-only issuance on `main` | **PASS** | §2 — one commit, one path, `5247a769…`, remote readback, fresh two-host push CI `43255471637` |
| Branch created only after that gate | **PASS** | §2 — `solo-wo-083-r01` branched from the exact issuance SHA; branch ACTIVE unchanged |
| Two-path implementation ledger | **PASS** | §3 — exactly the two authorized files, `780/5` and `212/0`, no other path, no dependency/feature/target change |
| `N083-01` real reservation refusal | **PASS** | §5 — `n083_as=media_host_as=limit_enforced … request_bytes=805306368 reserve=denied pid=4519 pgid=4519 wall_ms=10` |
| `N083-02` direct-child negative | **PASS** | §5 — `n083_direct_as_child=exit=71`, empty output, prompt, before the request |
| `N083-03` pure cross-platform cases | **PASS** | §5 — six pure cases plus the real-binary token case green on both hosts |
| `N083-04` Windows negative | **PASS** | §5 — status 3 `HOST_UNSUPPORTED`, empty stdout, zero Linux-only execution |
| `N083-05` prior regressions | **PASS** | §5 — `N076`, `N079`, `N081` and public CLI unchanged and green |
| `N083-06` complete final branch CI | **PASS** | §5/§6 — run `43258192188` at `ccc3ca8f…`, both hosts, 33 steps, 0 failures |
| `N083-07` authority and separation | **PASS** | §3/§7 — branch-only proof, `main` code untouched, branches preserved, cleanup recorded |
| Canonical report publication | **PASS** | §7 — one report-only commit on `main`, remote readback, fresh two-host run at the report HEAD |

No gate is `FAIL`, `UNVERIFIED` or `BLOCKED`, and **no failing CI iteration occurred in this order**: the single bounded branch push was green on both hosts on its first run, so no corrective commit and no red run needs disclosure. The earlier WO-081 red branch lint (run `43238905991`, corrected before the accepted `cacc0f9b…`) and the WO-082 work are prior, separate history, not WO-083 results.

## 7. Final state, limitations, cleanup and handoff

**Repository.** One checkout, one worktree, exactly one remote `origin`, exactly four `.documents/WIP/` files, clean index and working tree on `main`. `main` is at the issuance commit `5247a769a1d5d39510b7a8d9165d1451b75c4e1f` plus this report-only commit, and its source blobs are still the accepted WO-082 pair (`aba0cb1a…` and `1846af84…`), so **no technical-branch code reached `main`** — no merge, cherry-pick, path promotion, rebase, reset, stash, force push, additional worktree, clone or shadow repository was used anywhere.

**Branch preservation.** `solo-wo-083-r01` is preserved locally and on `origin` at exactly `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`, tree `b299b005c03657344213632eb0e0e4623023f694`, with its ACTIVE still the issued blob `9f231565…`. `solo-wo-081-r01` (`cacc0f9b…`), `solo-wo-079-r01` (`380393dd…`), `solo-wo-076-r01` (`40a40bab…`), `solo-wo-061-r01` (`4371af21…`), `solo-wo-069-r01` (`e6162cdf…`) and `solo-wo-070-r01` (`d270f95c…`) are byte-for-byte unchanged. No tag, release, deployment or pull request was created.

**Local limitation.** This Owner workstation still cannot link (`cargo check`/`build`/`test` require `link.exe`, which is absent), so **every** compile, lint, test, security and kernel statement in this report comes from the two-host GitHub CI at the exact SHAs named. Locally only Git-level and formatting evidence was possible: `cargo fmt --all -- --check` (rustfmt 1.10.0-stable, pinned toolchain `1.99.1`), `git diff --check`, path/numstat/blob readback and the brace/paren balance sanity check — and those are reported as such, never as compiler evidence.

**Cleanup.** This run created no task-owned temporary artifacts outside ignored scratch: report assembly used a few short-lived files in Solo `.temp/` that were deleted after assembly, and `.temp/` again has **zero** top-level files. The inherited `architecture-fixtures/`, `solo-wo-073-wip/` (4 files / 46 062 bytes) and `toolchain-installer/` directories and the existing `target/` are preserved exactly as found and remain unverified reference only — not executed, not staged, not treated as source. No Owner or unknown file was deleted, moved or reinterpreted, and no `git clean`, destructive reset/checkout, stash, force push or branch rewrite was performed. No agent temp area outside Solo was needed.

**Residual risks and what must not be inferred.** This proves one refused capacity request, not memory pressure, not RSS, not an OOM outcome, not `RLIMIT_CPU` exhaustion, not a process tree and not any Windows containment. The child's ceiling came from the same installer that earlier orders accepted, so the negative is only as strong as that boundary; a host that silently ignored `RLIMIT_AS` would make the child refuse with status `74` and the parent refuse with `HOST_LIMIT_UNAVAILABLE` rather than produce false evidence — which is the intended fail-closed behaviour, but it also means this negative cannot *by itself* prove the ceiling was installed, only that the request was refused while the ceilings read back exactly. Descendant handling, tree cleanup, stress, Job Objects, sandboxing, decoding, media correctness, publication and Stage 03 all remain future Owner-issued work.

The Implementation Agent has **not** self-ACCEPTed and has not begun another order. Independent Chief Master Architect disposition — **ACCEPT / REVISE / STOP** — is requested against the report HEAD, the exact two-file branch contents, the real Linux denial evidence and both green two-host runs. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** regardless of this narrow branch-only proof.
