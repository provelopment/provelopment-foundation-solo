# Execution Report

**status=STOPPED — N102 Windows canaries implemented but not green; SVG/GIF contract amendment delivered; no self-acceptance**

**Order:** `OWNER-ISSUED — SOLO-WO-104 R01`. **Required outcome:** `R03C-WINDOWS-N102-CANARIES-VERIFIED-AND-SVG-GIF-CONTRACT-APPROVED`. Deliverable **B is delivered and green**; deliverable **C–F is not verified**: the technical branch implements the three canaries and the mandatory CI gate, but two mutually exclusive frozen-checker constraints could not both be satisfied inside this run. No integration to `main`, no deliberate-red branch, no acceptance claim. `R03.C3`, `R03.C`, Stage 03, publication, release and deployment remain **OPEN**. SVG and GIF have **no implemented codec**.

## 1. Preflight (all pins exact)

`HEAD == main == origin/main == git ls-remote origin main == 6c6abe8553607d18fa8b973e2ca67a0fff5a662c`; tree `bca82525c049eb03dbc55f2d2a52c804ac95b9cd`; sole parent `f14ac8958c5e64ce8e30deff55aa3fe9b53c6743`. Report `30440b3b`, tracked ACTIVE `27971711`, Owner-installed ACTIVE `ff4308e6`, NEXT `92fa562b`, ROADMAP `9d7a8622`, OUTLOOK `cb9c7a61`, Architecture `c941c7f9`, Architect SOP `1483c9be`, Implementation SOP `51bb3792`, `AGENTS.md` `6eae1da8`, root `Cargo.toml` `14505be4` / `Cargo.lock` `5a18352b` / toolchain `9ec55b3b`, workflow `d7d8990c`, restricted decoder `65b0cb29`, native seam `47fd94ae`, safe supervisor `43398847`, adapter root `e99f3fa9`, adapter manifest `9c7d526d`, CLI dispatch `17221fac`, smoke test `c716d8b2`, checker `1c99d348`, unsafe guard at `crates/solo-cli/tests/unsafe_boundary.rs` `57d75796`, graph `da7d7ee7`, app policy at `crates/solo-application/src/graphic_image_inspection.rs` `bdba09c1`. Exactly 16 remote refs with no `solo-wo-104-*`; one worktree, one `origin`, empty stash, four WIP files, and the only preflight dirty path was the Owner-installed ACTIVE.

## 2. Commit A — authority installed (green)

`179a35bbf59059b9dd7445cffc91a762f5216b43`, tree `37895dd4b3771f540bc6d05ba363a0c8de76f1ae`, sole parent `6c6abe85…`, ACTIVE blob `ff4308e6…` byte-identical to the Owner bytes. `push` run **`43567570961` GREEN** on Ubuntu 24.04 and Windows 2025.

## 3. Commit B — Architecture O09 SVG/GIF contract (green, on `main`)

`7205027bbb656fe07205b4c54d2575f151d0c173`, tree `c623e7dfc516f594bf90070168c327a2ccbf40ba`, sole parent `179a35bb…`, Architecture `c941c7f9…` → `f66096ec375a497ac6de9a65cfbeae4c5dc85844`. `push` run **`43567811029` GREEN** both hosts; `git diff --check` clean; exactly one changed file, 6 insertions / 6 deletions.

| Section | Change |
| --- | --- |
| §3.3 O09 summary | the O09 literal now names "active SVG and remote fetch" refusals **plus** the Owner-accepted future formats, instead of a blanket SVG/GIF refusal. |
| O09 decision table | records static sanitized SVG and still/animated GIF as **Owner-accepted future formats whose codecs are not implemented**, with "no active SVG". |
| §6.2I.3 Formats | separates the **implemented** PNG/JPEG/WebP slice (AVIF admitted, codec proof pending) from two normative **future** requirements: static SVG that must remain true scalable vector output with bounded parsing, deterministic sanitization of an approved construct set, whole-asset refusal on any unsupported construct, and never silent flattening or rasterization; and GIF covering GIF87a stills plus GIF89a **animated** images with preserved frame order, palette/transparency, offsets, compositing, disposal, delays and looping, never a first-frame substitute. States that the current code **refuses** `.svg`/`.gif` until a separate Owner-issued codec order passes its own cross-host suite, and that no claim may describe SVG or GIF as implemented. Active SVG stays refused; `graphic.favicon` stays PNG-only. |
| §6.2I.4 Bounds | extends 20 MiB / 1–8,192 per side / 32,000,000 pixels with checked arithmetic to both formats, and requires later-pinned aggregate frame, composition, memory and duration budgets for animated GIF and bounded element/depth/attribute/reference/cycle/filter complexity for SVG, so animation is never silently reduced and 512 MiB / 10 CPU-s / 15 s is not assumed sufficient. |
| §6.5 asset pipeline | replaces the blanket "No SVG, GIF" sentence with the accepted-future-format statement, the transitional refusal and the unchanged prohibition on external URL, unapproved embed/codec and inferred generic artwork. |
| Coverage table (D10/O09) | `V20`–`V21` now reads "…plus the transitional `.svg`/`.gif` format refusal". |
| `P07` (unchanged) | "active SVG … refused" remains correct and was not weakened. |

No codec, dependency, renderer, media asset, schema or delivery promise was added in this commit.

## 4. Technical branch — implemented, not green

Branch `solo-wo-104-canary-r01` (pushed, **not** integrated): `946c94de` → `8ed1e89f` → `23da19fb` → **`43616ae5ca46d06084e73d48c75cff8363fb841d`** (tree `3cc54080e5a5ad2c43a39cb786b99f5c9f5c3e72`). Seven ledger paths changed: `media_worker_host_native.rs` (eleven injection boundaries — `create_limited_job_v1`, both `CreatePipe` calls, `SetHandleInformation`, the attribute list, the minimal environment, the working directory, `CreateProcessW`, `AssignProcessToJobObject`/membership, `resume`, `wait_ms`; an extra real inheritable `CreateEventW` handle excluded from the two-handle allowlist; `GetHandleInformation` and `std::env` child probes), `media_worker_host.rs` (bounded answers, typed `N102ProbeRefusalV1`, the probe runners and the isolated helper parent), `graphic_image_inspection.rs` (closed `probe-handle:`/`probe-environment:` modes and bounded answer lines; parent digest guard, formats and decoding untouched), `lib.rs` (seam re-exported **through** the safe supervisor, because the frozen guard forbids `pub use media_worker_host_native`), `main.rs` (closed `--internal-n102-env-parent` dispatch), `media_worker_host_smoke.rs` (`n102_01_extra_inheritable_handle_is_excluded`, `n102_02_parent_environment_is_excluded`, `n102_03_native_failures_refuse_and_cleanup` over steps 1–11), and `rust-verify.yml` (Windows-only N102 gate asserting all three exact names and `test result: ok. 3 passed; 0 failed; 0 ignored`).

Locally verified before dispatch: format clean, strict Clippy clean, Windows-target type-check clean, `unsafe_boundary` **22 passed**, real-tree architecture `architecture=legal packages=4 targets=33 files=62 edges=68`.

### Exact blocker: two frozen accepted protections conflict

| Run | Event / head | Result |
| --- | --- | --- |
| `43568360641` | dispatch `946c94de` | **red** both hosts: `unsafe_boundary.rs:626` — the frozen guard forbids `pub use media_worker_host_native`; fixed by laundering the seam through the safe supervisor. |
| `43568709222` | dispatch `8ed1e89f` | Ubuntu green; **Windows red**: `n097_10_the_wall_deadline_terminates_and_reaps_the_worker` FAILED at `media_worker_host_smoke.rs:2342`, because the injection selector was a process-global atomic and `n102_03` injected refusals into concurrently running host tests. Real interference, observed, not flaky. |
| `43569056579` | dispatch `23da19fb` | red both hosts at **Format check** (missing file terminator after the rewrite). |
| `43569184308` | dispatch `43616ae5` | **red** both hosts at `check_architecture_fixtures.rs:26` and `:97`: `MacroUnsupported — macro \`thread_local\` cannot be soundly inspected and is refused` in `media_worker_host_native.rs`. |

The accepted AST checker refuses the only macro-based way to scope the selector per thread, while the macro-free process-global form demonstrably corrupts an unrelated accepted test. The correct remedy is to **carry the injection step as a parameter** through the closed probe run (no macro, no shared state, no interference), which means widening the native platform-run signature across its three host variants and the supervisor wrappers. That is real engineering, not a gate weakening, but it exceeded this run's capacity. I did **not** edit the frozen checker or guard, did **not** filter or skip `n097_10`, and did **not** integrate anything.

## 5. Draft-patch audit and preservation

`.temp/solo-wo-103-wip/n102-seam-wip.patch`: 13,570 bytes, SHA-256 `7FBB2A16D6D21BBCA69513271833022AAD79623329A9523AE702FC932600100F`, diff paths `crates/solo-adapters/src/lib.rs` and `crates/solo-adapters/src/media_worker_host_native.rs` only, no symlink, binary or mode change, `git apply --check` exit 0 against the real base. It was applied as **untrusted draft**, then reworked for visibility, imports, thread-safety, terminator and guard compliance; it is preserved untouched, as are `.temp/solo-wo-073-wip/`, the three older patches, the WSL cache and the earlier local branches.

Final state: `main` = `7205027b` green; branch `solo-wo-104-canary-r01` = `43616ae5` red, pushed, unmerged; 17 remote refs (baseline 16 plus that branch, which the order permits); ACTIVE still the Owner-issued WO-104 bytes; four WIP files; one worktree, one origin, empty stash; no negative branch; no deletion, reset, rebase or force-push; no release, deploy or self-acceptance.

## 6. What the next run must do

1. Carry the injection step as a parameter through `run_confined_image_worker_*` and the three platform variants, re-dispatch, and require both hosts green with all three canaries actually executed.
2. Only then create `solo-wo-104-negative-r01`, mutate the minimal-environment boundary in the closed helper path, and require the unchanged `n102_02` to fail at Windows runtime.
3. Integrate the exact verified technical blobs, run fresh two-host CI, and republish this report as `COMPLETE`.

Local limits, stated plainly: no MSVC linker here, so no local Windows runtime proof; the five reflink-only `snapshot_proofs` cases and the `n097` real-filesystem cases fail locally for the documented missing reflink fixture root while passing on both hosted runners. Every Windows fact above is a hosted `windows-2025` observation, and the three canaries remain **unproven**.

