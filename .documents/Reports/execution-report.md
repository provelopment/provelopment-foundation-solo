# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-065 R03` — resolve the final `R03.C2` Windows writable mapped-view asset-route proof, test-only, with no native production change.
**Matching Owner handoff:** `SOLO-WO-065 R03`.
**Session date:** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo`; one worktree, sole `origin`, no second checkout.
**Disposition:** **COMPLETE** for this one test-only evidence recovery. The proof was obtained rather than waived: with only a surviving writable mapped view, the actual typed Entity asset port **refused** (`OtherIo` / `ASSET_SOURCE_IO_ERROR`), and after the view was unmapped the same capability and reference returned exactly the rewritten 4,096 bytes. **`R03.C2` is still not independently ACCEPTED**, **`R03.C` and Stage 03 remain OPEN**, and this report is **not** a self-ACCEPT.

## 1. Starting baseline (verified before any write)

| Anchor | Verified value |
| --- | --- |
| `HEAD == main == origin/main` | `f78c54306a033815d9dab3cabfd2a7e8b535069c`, tree `e90f12a870c95944531291f20e61ebd3996b3764` |
| Report blob at baseline | `c91c2fa3dbac038be3ace1768976bb41654c5110` (the R02 `STOP`-era report) |
| ACTIVE before Owner install | `a40536aec3cc509b9a15613025470257a14195dc` (frozen R02 order) |
| NEXT / ROADMAP / OUTLOOK | `92fa562b1af0c54391851aa5c79deb1631127069` / `97df81e5cf63443d11a8f4414934d4fd05edd0d8` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Branches at preflight | exactly `main`, `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`, `solo-wo-065-r01` = `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`, `solo-wo-065-r02` = `2de75c1e3f96a84b32f8cd21f491d925225d62d1`; no R03 branch existed |
| Technical target before change | `crates/solo-adapters/src/source_read_native.rs` = `b0ea31dbe6ddb8a7b100da89e53769282d180fea`, 2,026 lines |
| Dirty state | only the Owner-installed complete R03 `ACTIVE` (working blob `2622aeace2329a91e15cbdced3781a30bb05d2d0`, 23,958 bytes, 119 lines); nothing untracked, no sequencer state |

The existing Windows test-only `mapped_view_proofs` module and the R02 canonical report were read before editing, and the frozen `Section`, `WritableView`, `Fixture`, `handed_off_view` and `BODY_LENGTH` helpers were reused rather than duplicated.


## 2. Issuance on `main` (ACTIVE alone)

| Item | Value |
| --- | --- |
| **Issuance commit** | **`87110df640271fdfad60e4b8e2e3742ad6ed0230`**, tree `8d06419713a85937a3b041e041a7d1a91b486859`, parent `f78c5430…` |
| Changed paths | `.documents/WIP/workorder-active.md` only → `2622aeace2329a91e15cbdced3781a30bb05d2d0` |
| NEXT / ROADMAP / OUTLOOK | unchanged (`92fa562b…`, `97df81e5…`, `cb9c7a61…`) |
| Issuance CI | **`43044051684`** — `success`; Ubuntu job `130603176244` (33 steps, 9 skips), Windows job `130603176264` (33 steps, 14 skips); **407 / 411 passed**, 0 failed, 0 ignored |

## 3. The single technical path — exact diff

| Path | Before | After | Change |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/source_read_native.rs` | `b0ea31dbe6ddb8a7b100da89e53769282d180fea` (2,026 lines) | **`5527b2b4f0d4f47acaa86eead74c7106607c789f`** (2,176 lines) | +154 / −3, entirely inside `#[cfg(all(test, windows))] mod mapped_view_proofs` |

**Technical commit:** `bbe4a98f4da464ce189956fea34dd94323499ab8`, tree `36a034bf003dc0c2dc389801da6449dad3aa67a8`, parent `87110df6…`, exactly the one path above.

The three deleted lines are the two pre-existing `use` items in that test module, extended to import `EntityAssetSourceV1`, `GraphicAssetReadPort`, `GraphicAssetReadRefusalKind`, `MAX_GRAPHIC_SOURCE_BYTES`, `PhysicalIdentityV1`, `AssetNamespace` and `AssetReferenceV1`. Everything else is addition:

1. `Fixture::asset_leaf()` and `Fixture::layout_asset_leaf()` — create `content/assets/brand/logo.png` as an ordinary regular file of exactly `BODY_LENGTH` (`4096`) bytes of `b'A'` **under the content root that guard already claimed exclusively**. Only the new test calls them, so the `pages/` tree every legacy case asserts is shaped exactly as before.
2. The new test itself.

**Production code is untouched:** no instruction, binding, sharing mode, `unsafe` block, FFI item, import outside the test module, dependency, manifest, lock, toolchain, policy or workflow changed. `crates/solo-adapters/src/source_read.rs` `fb6f005a053ea80cea452fe2d57fb9fc8ea45ad9`, `crates/solo-application/src/graphic_asset_read.rs` `6499421d6e718e9d46d64e45fb152f34e2989cc1`, application `source_read.rs` `93bc39d42b276b03cb2fba1cc5b43ea7a51108c8`, `graphic_asset_inventory.rs` `7de9bc7399854b2059cbec17ee23c1e1ae637159`, both `lib.rs` (`b337a3f67da4b3f76d53bf112a3f8628f85184b8`, `107f3579f01878f7532d66c93e150f8aafc4f42f`), the application asset tests `e9ea65c53c3ecd554d7fc44846b428b6ff96d801`, the adapter asset tests `8d33a6d82c29b1618af282f75d3937d32a0f3953`, `Cargo.lock` `2f9e53576df117f2020ae064d6973304f20833f1`, `crates/solo-adapters/Cargo.toml` `6109224cd3e3e89487884631f2b07c68ad8149c3`, `deny.toml` `3063e864c28be1e77787f43f1b6f561d543c8f4a`, `rust-verify.yml` `eb78dad240bb7adb3e0ef9535870897176c99843` and `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` are all unchanged at the integrated revision.


## 4. The proof, assertion by assertion

`r03c2_10c_transferred_writable_view_refuses_asset_port_until_unmapped`, inside the existing Windows-gated module, deterministic — no sleep, race, retry or skip:

| Step | Assertion |
| --- | --- |
| Adversary construction | the asset leaf is opened read+write, a `PAGE_READWRITE` section is created over it, `FILE_MAP_WRITE` maps the whole file, and **both** the section handle and the file handle are dropped, so only the view survives |
| The view is real | `view.byte(0) == b'A'` and `view.byte(4095) == b'A'` still read the object after both handles closed |
| The view can still write | `view.fill(b'C')` then the first and last byte are `b'C'`, and `fs::metadata(..).len()` is still exactly `4096`, so the rewrite is in place at unchanged length |
| Asset route, view alive | `ContentRootCapability::adopt_bootstrap_path(content/, OperatorDesignatedContentRoot)` then `read_entity_asset(&AssetReferenceV1::parse(AssetNamespace::Entity, "brand/logo.png"), MAX_GRAPHIC_SOURCE_BYTES)` must be `Refused` with `GraphicAssetReadRefusalKind::OtherIo` and code `ASSET_SOURCE_IO_ERROR`; a `Present` of any length panics the test |
| Asset route, view released | the same capability, the same typed reference and the same bound now yield `Present` with `length == 4096`, `bytes.len() == 4096`, every byte `b'C'`, `links == 1`, an identity different from the content root and different from `PhysicalIdentityV1::new(0, 0, 0)` |
| Stability | one repeat read returns identical bytes and an identical identity |

No Page locator, projection or fallback appears anywhere in the new test: the subject is the real `assets/brand/logo.png` through the real asset trait, and the refusal is the frozen native Windows mapping for a blocked open, asserted as observed rather than widened.

**Actual Windows log evidence** (run `43045427407`, job `130607992837`, tested shas `bbe4a98f…`):

```
test source_read_native::sys::mapped_view_proofs::a_writable_view_keeps_writing_after_its_originating_handles_close ... ok
test source_read_native::sys::mapped_view_proofs::a_handed_off_writable_view_still_blocks_the_pinned_body_open ... ok
test source_read_native::sys::mapped_view_proofs::a_live_writer_handle_over_a_writable_view_makes_the_body_open_refuse ... ok
test source_read_native::sys::mapped_view_proofs::a_pre_existing_fixture_name_is_preserved_and_never_adopted ... ok
test source_read_native::sys::mapped_view_proofs::the_product_reader_refuses_a_body_while_a_handed_off_writable_view_is_alive ... ok
test source_read_native::sys::mapped_view_proofs::the_reader_succeeds_normally_once_the_mapping_is_released ... ok
test source_read_native::sys::mapped_view_proofs::r03c2_10c_transferred_writable_view_refuses_asset_port_until_unmapped ... ok
```

Measured totals: Ubuntu **407** on every run (unchanged — the new case is Windows-only by construction), Windows **412** on the branch, substantive `main` and report-head runs (`411` at issuance), 0 failed, 0 ignored, 9 Ubuntu / 14 Windows host skips. The new case was not `#[ignore]`d, not Linux-gated, not skipped and not weakened.


## 5. S065R03-01…11 acceptance matrix

| Gate | Result | Evidence |
| --- | --- | --- |
| **S065R03-01** | PASS | §1: baseline `f78c5430…`/`e90f12a8…`, report and ACTIVE/WIP blobs, three retained branches at exact tips, sole checkout/worktree/origin, only the Owner-installed R03 ACTIVE dirty, module and R02 report inspected, no drift. |
| **S065R03-02** | PASS | §2: ACTIVE alone at `87110df6…`, blob `2622aeac…` readback, run `43044051684` green on both hosts, NEXT/ROADMAP/OUTLOOK unchanged. |
| **S065R03-03** | PASS | `solo-wo-065-r03` created only at issued-green `87110df6…`; the technical commit changes exactly `crates/solo-adapters/src/source_read_native.rs` inside the Windows test module; `4371af21…`, `7f609320…` and `2de75c1e…` unchanged; no rewrite, force, rebase or amend. |
| **S065R03-04** | PASS | §4: real `content/assets/brand/logo.png`, 4,096 `b'A'` bytes, mapped writable, **both** originating handles closed, view still readable and rewriting in place at unchanged length; legacy `pages/` fixture and all six earlier mapped-view tests unchanged and green. |
| **S065R03-05** | PASS | With the live view, the actual typed `read_entity_asset` over a real `ContentRootCapability` refused `OtherIo` / `ASSET_SOURCE_IO_ERROR`; a `Present` would have panicked. The passing Windows log line is quoted verbatim in §4 at the tested SHA. |
| **S065R03-06** | PASS | After unmapping, the same capability and reference produced the exact rewritten 4,096 `b'C'` bytes, `links == 1`, a non-root non-zero identity, and a repeat read with matching bytes and identity. |
| **S065R03-07** | PASS | All prior `r03c2_*`, R01/R02/R03.A/B/C1, old and new mapped-view, Page-native, Linux snapshot and unsupported-filesystem, 20 MiB bound, graph/architecture, refusal self-test and tripwire gates green on both hosts; `architecture=legal packages=4 targets=30 files=53 edges=40`; zero ignored tests; measured totals reported, not guessed. |
| **S065R03-08** | PASS | §3: no production native instruction or binding changed, no new `unsafe`, FFI, import outside the test module, dependency or OS primitive; frozen blobs verified byte-identical; `cargo fmt --all -- --check` clean; strict Clippy `-D warnings` green on both hosts. |
| **S065R03-09** | PASS with inherited warning | §6: full locked graph, fresh RustSec (`db_commit=fd6f39814c4d3c359fdbe223d0280dec4933f78f`), fresh OSV 18/18 clean, 18/18 not yanked, unchanged pins and policy; `warning[license-not-encountered]` on `Unicode-3.0` still emitted and remains **UNVERIFIED**; CI is **not** claimed warning-free. |
| **S065R03-10** | PASS | Four green two-host gates at exact SHAs: issuance `43044051684`, branch `43045427407`, substantive `main` `43046529973`, final report HEAD (run id given in the chat handoff, never inside this file). No red attempt occurred in this order. |
| **S065R03-11** | PASS | This single canonical report, report-only commit on `main`, remote tree/blob readback, one clean worktree, one origin, retained older and R03 branches, `target/` and the inherited `.temp/` archives preserved; no self-ACCEPT, no R03.C/Stage 03 closure, no catalogue, media, renderer, release or deployment. |


## 6. CI history, A1 and workspace preservation

| Phase | Run | Revision | Result |
| --- | --- | --- | --- |
| Issuance | `43044051684` | `87110df6…` | success — Ubuntu `130603176244` 407 / Windows `130603176264` 411 |
| Correction branch | `43045427407` | `bbe4a98f…` | **success, first attempt** — Ubuntu `130607993306` 407 / Windows `130607992837` **412** |
| Substantive `main` | `43046529973` | `bbe4a98f…` | success — Ubuntu `130611818760` 407 / Windows `130611818213` 412 |
| Final report HEAD | given in chat handoff | report commit | required green on both hosts |

0 failed and 0 ignored everywhere; skipped steps are the same host-inapplicable set as before (counterpart-host steps, the Ubuntu-only advisory negative probe and byte-reproducible double build, R03 fixture provisioning). **No red attempt occurred in this order.**

**Fresh A1.** 4 first-party + 18 third-party packages; `Cargo.lock` and the adapter manifest are byte-identical to the baseline and `rustix` is still `"=1.1.6"`; `cargo-deny 0.20.2` passed on both hosts against a fresh database `db_commit=fd6f39814c4d3c359fdbe223d0280dec4933f78f` (`fetched_at=2026-11-11T15:16:02Z` / `15:16:06Z`); a fresh OSV sweep returned **18/18 clean** and crates.io **18/18 not yanked**; toolchain, workflow and runner images are unchanged and no waiver or suppression was added. The added code is test-only, moves only in-memory bytes through the existing RAII wrappers, and introduces no `unsafe`, no FFI, no Win32 import and no network access; its fixture is task-owned, exclusively created and removed by its guard.

**Inherited warning, unchanged:** `warning[license-not-encountered]` for `Unicode-3.0` at `deny.toml:51` is still emitted on both hosts and remains **UNVERIFIED**. The inherited `itoa` / `syn` patch lags are untouched and outside this order.

**Workspace.** `main` = `bbe4a98f4da464ce189956fea34dd94323499ab8` (local == `origin/main`), tree `36a034bf003dc0c2dc389801da6449dad3aa67a8`, single parent — a fast-forward-only move from `87110df6…` with no merge commit. Retained and unchanged: `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`, `solo-wo-065-r01` = `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`, `solo-wo-065-r02` = `2de75c1e3f96a84b32f8cd21f491d925225d62d1`; the new `solo-wo-065-r03` stays at `bbe4a98f4da464ce189956fea34dd94323499ab8` locally and on GitHub. Exactly four WIP files; ACTIVE frozen at the R03 document `2622aeace2329a91e15cbdced3781a30bb05d2d0`; NEXT `92fa562b…` neutral; ROADMAP `97df81e5…` and OUTLOOK `cb9c7a61…` unchanged. One checkout, one worktree, one `origin`, and a clean `git status --porcelain --untracked-files=all` at every published revision; no `git clean`, reset, force, amend, rebase or cherry-pick; the ignored `target/` cache is preserved; Solo `.temp/` is still exactly **2 files / 289,253,806 bytes**; the task-owned Cline CI log created by this run's waits was deleted and none remain; nothing unknown or Owner-owned was deleted, moved or overwritten.

**Local limitation, stated honestly.** This host has no MSVC `link.exe`, so local executable tests are **NOT RUN**; the authoritative functional evidence is the CI runs above. Local `cargo fmt --all -- --check` passes; local `clippy` could not complete because dependency build scripts require a linker, so strict Clippy is evidenced by both CI hosts instead.

## 7. Remaining scope

Still unimplemented and outside this order: authoritative media-byte / format / MIME, dimension and pixel-limit verification; an Owner-reviewed lawful Platform asset catalogue with identifiers, hashes, media class, dimensions and licence provenance; whole-candidate validation that includes that pinned library; derived outputs; routes, HTTP surface, renderer, release and deployment. `namespace: "platform"` remains deliberately unverifiable and refuses the whole attempt. Stage 03 and `R03.C` remain OPEN.

**Handoff:** `SOLO-WO-065 R03 — COMPLETE` for this one test-only evidence recovery, with every applicable gate directly evidenced. The Implementation Agent does **not** self-ACCEPT: `R03.C2` awaits independent Chief Architect `ACCEPT / REVISE / STOP` against this report's exact remote HEAD, and no closure, waiver, catalogue, media, release or deployment follows from it.
