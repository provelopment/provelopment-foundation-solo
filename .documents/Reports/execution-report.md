# Execution Report

**status=COMPLETE — pending Chief Master Architect independent review**

**Order:** `OWNER-ISSUED — SOLO-WO-102 R01`; required outcome `R03C-CONFINED-PNG-JPEG-WEBP-IMAGE-INSPECTION-VERIFIED-AND-INTEGRATED`. Achieved: one inert, typed, nonpublishing inspection of every declared Entity graphic owner over PNG/JPEG/WebP, through a genuinely supervised single-use worker on **Ubuntu 24.04 and native Windows 2025**, integrated into `main` as exact verified blobs. `R03.C3` is now substantiated as this bounded slice; `R03.C`, Stage 03, release, publication, browser surface and deployment remain **OPEN**. Not self-accepted.

## 1. Preflight

`HEAD == main == origin/main == ls-remote == 4d9c8b7cbb6bc56181689ba058df41dda329dd71` with tree `6eee1e06…`; **22/22** pinned blobs exact, including the green checker `source.rs` `1c99d3482dd2a9bd7b027f5f193e17603976e361` and the accepted guard `unsafe_boundary.rs` `57d7579616fc0bab96dcb1e371584f20cfd5d8f9`. One worktree, one `origin` (expected GitHub fetch/push URLs), empty stash, four WIP files, **14** remote refs, no pre-existing `solo-wo-102-*` branch. Preserved untouched: `solo-wo-093-r01` `0dc45185`, `solo-wo-094-negative-r01` `0e9b3677`, `solo-wo-089-r01` `1236915c`, `solo-wo-090-r01` `0eb3fcf6`, local `solo-wo-098-image-r01` `ed938267`, local `solo-wo-099-image-r01` `f6e11372` (parent `4b722825`), and all three patches (244,647 / 180,065 / 187,820 B). The lost `330c08e0` object was never used.

## 2. Ordered commits and CI (all `push`/`workflow_dispatch`, attempt 1, both hosts green unless stated)

| Phase | Commit / ref | Tree | Sole parent | CI |
| --- | --- | --- | --- | --- |
| A — install WO-102 ACTIVE, blob `be350ae7` | `cae8615227b4e925a84e33893de21d5a83f7858a` | `dc849a5d…` | `4d9c8b7c…` | green `43559158941` (Ubuntu `132338740241`, Windows `132338740065`) |
| C1 — image slice on `solo-wo-102-image-r01` | `2e756330ee0317533ddaa4fdd31af765da1d7cd3` | `5973f78d…` | `cae86152…` | red iteration 1 `43560068920` |
| C2 — Windows lint, single seam import, fixture references | `50b538f394d7dccce4824c16d9babc9d25c3d83e` | — | `2e756330…` | red iteration 2 `43560531021` |
| C3 — JPEG frame-header fix, port facts, distinct owners | `a5a1b4373058efb09be6f05d1c990adbf0c92144` | — | `50b538f3…` | **green `43561284314`** (Ubuntu `132345298564`, Windows `132345298830`) |
| N1 — deliberate red `solo-wo-102-negative-r01` | `ce4fb052357705d3d3ad8d6f40f9cda41eb80dd6` | — | `a5a1b437…` | **red `43562433923`** (Ubuntu `132348855239`, Windows `132348855497`) |
| D — integration on `main` | `8876eee2c499951baa4269ced8f609b1ade6a50a` | `a9d56cd3…` | `cae86152…` | **green `43562649182`** (Ubuntu `132349530556`, Windows `132349530774`) |

Patch audit before application: `.temp/solo-wo-098-wip/wo100-branch-work.patch`, regular file, exactly 187,820 bytes, SHA-256 `73C820975AC9CEFAE1065FAAC0E620E07D9C6F147316B2B8DC87C8E4B3EAD783`, 12 paths / 4272 insertions / 15 deletions, no `.git`, `..`, binary, symlink or mode entry, `git apply --check` clean, applied once with no `--3way`. Verified baseline blobs unchanged on the new branch (`e1b5ca6e`, `b1ee56da`, `1550d34f`, `7eed6e15`, `eacf51c5`, `7a5beada`, `22ee67e9`).


## 3. Defects found and corrected inside the ledger (iteration is not a stoppage)

1. `crates/solo-adapters/src/media_worker_host.rs` still imported the abandoned twin module `crate::media_worker_host_native_windows`; both positive branches now import the one reserved module `crate::media_worker_host_native`. There is exactly one declaration in `lib.rs`: `#[cfg(any(target_os = "linux", windows))]` plus `#[cfg_attr(target_os = "linux", allow(unsafe_code))]` and `#[cfg_attr(windows, allow(unsafe_code))]`, with `#![deny(unsafe_code)]` and the unchanged `source_read_native` Windows allowance; no `#[path]`, no twin.
2. Windows strict Clippy: `media_worker_host_native.rs` had a `let…else` that the lint rewrites to `?`; corrected to `let length = u32::try_from(...).ok()?;`, verified with `cargo clippy --target x86_64-pc-windows-msvc -D warnings` before dispatch.
3. Authored references: `graphic_asset_read` builds `assets/{authored}` and `logical_path()` returns that prefixed form, so declared references must be bare. `n097_01` asserted the bare form and `n097_02` declared the prefixed one; both were corrected, and `Root::new` keeps writing `<root>/assets/<name>`.
4. `n097_03` declared the same graphic property twice on one inheritance chain and was refused by inventory validation; it now uses two distinct files *and* two distinct roles (`logo` at Entity, `banner` at Site), preserving the "each declaring owner keeps its own independent result" intent.
5. **Real product defect:** `shape_of_jpeg_v1` read the sampled height at the SOF body start — the precision byte — instead of one byte later, so every JPEG was refused as `INVALID_MEDIA` (child exit 6) after a correct decode. Fixed to `body_start + 1` (height) and `body_start + 3` (width), with `components` still at `body_start + 5`. The diagnosis was local and evidence-based: the pinned `image =0.25.10` decoded all three fixtures at 2x2 in a scratch probe, and the child's exit status was carried as a temporary bounded fact until the precision-byte offset was identified; the canonical fixed exit codes and unit-variant refusal were then restored exactly.

## 4. Real host evidence (verbatim bounded markers from the green branch run)

Windows 2025: `n097_image=format=png mime=image/png width=2 height=2 decoded=16 pid=9104 wall_ms=4 src=e747282dca9ff94eeb738f02e58ff070a0f9aa47c06cd4a4f5a3f57703596f30 out=c21b35e3f28e676cedf24c13575a7346682e101a2d26aad9598d0cdbcee9ee3b`; `n102_worker=jpeg exit=Some(0) pid=8772 wall_ms=4 cleanup=true limits=true stdout_len=278` with `n097_image=format=jpeg mime=image/jpeg width=2 height=2 decoded=16 pid=7600`; `n102_worker=webp exit=Some(0) pid=6700 wall_ms=4 cleanup=true limits=true stdout_len=278` with `n097_image=format=webp mime=image/webp width=2 height=2 decoded=16 pid=9956`. Ubuntu 24.04 produced the same set with its own real pids and Linux session/group facts, and N079–N093 nonregression stayed green on both. Every marker is bounded: format, MIME, geometry, decoded length, pid, wall and digests only — no environment value, absolute path, handle value or source byte.

## 5. Deliberate-red sensitivity

`solo-wo-102-negative-r01` changes only the parent-side comparison in `crates/solo-adapters/src/graphic_image_inspection.rs` (the recomputed decoded SHA-256 against the worker's claimed `out_sha256`): both digests are still computed and consumed, and the guard is removed. Format and strict Clippy passed, then the unchanged `n097_09_every_worker_refusal_and_forged_claim_is_refused` failed **in test execution** at the forged-output assertion on **both hosts** (`media_worker_host_smoke.rs:2309`). The branch is preserved and was never merged, cherry-picked or promoted.

## 6. Integration and architecture facts

Promotion used a scoped `git checkout <green branch> -- <12 paths>` with no merge, rebase or cherry-pick; all twelve `git hash-object` values equal their green branch counterparts, and `source.rs` (`1c99d348`) and `unsafe_boundary.rs` (`57d75796`) are unchanged. The real-tree checker reports `architecture=legal packages=4 targets=33 files=62 edges=68`, and the workflow asserts `edges=68` on both hosts; the 29 negative self-check fixtures are unchanged; `unsafe_boundary` is 22/22. Dependency closure is the audited `image =0.25.10` (no default features; `png`, `jpeg`, `webp`) plus the existing `windows-sys =0.61.2` Job Object/process/pipe features; cargo-deny ran with a fresh RustSec refresh and its `RUSTSEC-2026-0009` negative probe passed, with no advisory, ban, license or source finding on the workspace graph.

## 7. Limits and preservation

Local Windows runtime proof is impossible on this workstation (no MSVC linker, and installing one is out of scope), and the local Linux real-filesystem image cases need a reflink-capable fixture root, so those eight real-filesystem cases are **LOCAL-UNVERIFIED** while the identical suites pass on both hosted runners; WSL provided fmt, strict Clippy, Windows and Linux type-checking, `unsafe_boundary`, the architecture checker, and `n097_09`/`n097_10` iteration. Owner data, the three patches, `.temp/solo-wsl-target`, `.temp/solo-wo-073-wip`, both old local image branches and the frozen WO-102 branches are preserved; no branch was deleted, reset, rebased or force-pushed and no gc, prune, clone or second worktree was used. ACTIVE remains the Owner-issued WO-102 bytes; NEXT is neutral; ROADMAP, OUTLOOK, Architecture, SOPs and the accepted checker/guard are unchanged.

Submitted for independent `ACCEPT / REVISE / STOP`. This report is not self-acceptance, and no release, deployment, publication or general-sandbox claim is made.
