# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-065 R01` — implement `R03.C2`, confined and bounded Entity graphic source-byte acquisition over the complete accepted `R03.C1` obligation inventory, without publication authority.
**Matching Owner handoff:** `SOLO-WO-065 R01`.
**Session date:** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo`; one worktree, sole `origin`, no second checkout or nested repository.
**Disposition:** **COMPLETE** for this bounded, nonpublishing **source-byte acquisition** slice only. `R03.C` and **Stage 03 remain OPEN**. Nothing here accepts the work, closes `R03.C`, validates media, selects a platform catalogue, renders, publishes, releases or deploys, and **no self-ACCEPT** is claimed.

## 1. Starting baseline (verified before any agent write)

| Anchor | Verified value |
| --- | --- |
| `HEAD == main == origin/main` | `fdc695300d6affd6b21c450df51723cd6067f7b8`, tree `a84a2bb48eec5b62a5c7f17268e409b5649f96ee` |
| Last canonical report blob | `91429f56f5941a4b3a0f597efa07d3d37c83796c` |
| ACTIVE before Owner install | `ea5f20b736483c2e102456be1cea9104059fc81a` |
| NEXT before issue | `08ba72a3a58e5e4d3fbc5a4ee6fe45999562f14a` (the sole `R03.C2` candidate) |
| ROADMAP / OUTLOOK | `97df81e5cf63443d11a8f4414934d4fd05edd0d8` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `Cargo.lock` / adapter manifest / `deny.toml` / workflow | `2f9e53576df117f2020ae064d6973304f20833f1` / `6109224cd3e3e89487884631f2b07c68ad8149c3` / `3063e864c28be1e77787f43f1b6f561d543c8f4a` / `eb78dad240bb7adb3e0ef9535870897176c99843` |
| Accepted inventory / Page adapter / frozen OS boundary / frozen app port | `7de9bc7399854b2059cbec17ee23c1e1ae637159` / `c651bb3ac98e84431d47bd421b5a93359221282f` / `b0ea31dbe6ddb8a7b100da89e53769282d180fea` / `93bc39d42b276b03cb2fba1cc5b43ea7a51108c8` |
| Application / adapters `lib.rs` | `1f5115569b0c65bef3e9f44abd6a4a022f9fd081` / `b33fe9dee4cbea0a5daf557b45b44297e313ad05` |
| Retained historical branch | local and remote `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Initial tracked difference | only the Owner-installed `.documents/WIP/workorder-active.md` (blob `7c3797c61eb7eef6700228b224baddf26cf7dc8b`) |

Three new technical paths were confirmed absent, the WIP inventory was exactly four files, and the two inherited `.temp/` archives plus the ignored `target/` cache were present.

## 2. Issuance on `main` (paths 1–2)

| Item | Value |
| --- | --- |
| **Issuance commit** | **`2fb7e34bea223bbd1e8720496c97a63ef750f45e`**, tree `be154fd1bd8e8974e77f209f2446f29095ac80d5` |
| Changed paths | `.documents/WIP/workorder-active.md` (`7c3797c6…`) and `.documents/WIP/plan-next.md` (`92fa562b…`) — exactly two |
| ROADMAP / OUTLOOK | unchanged, blobs `97df81e5…` / `cb9c7a61…` |
| Issuance CI | **`43026432918`** — `success`; `130542193608` (`ubuntu-24.04`) and `130542193858` (`windows-2025`), 33 steps each, 0 failures, 9/14 counterpart-host skips, **384 / 386 tests passed**, component security gate passed |

The `NEXT` blob after issuance is byte-identical to the neutral heading that preceded the `R03.C2` candidate, which is the strongest available proof that the candidate left the file completely.

## 3. Nine-path ledger — exactly what changed

| # | Path | Change |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | already Owner-installed; committed alone in issuance → `7c3797c6…` (unchanged afterwards) |
| 2 | `.documents/WIP/plan-next.md` | issued candidate removed, neutral heading only → `92fa562b…` |
| 3 | `crates/solo-application/src/graphic_asset_read.rs` | **new** → `cbd51edfc236f8beaab9ed137a877a9e2d2f0954` |
| 4 | `crates/solo-application/src/lib.rs` | one module + public re-export → `b337a3f67da4b3f76d53bf112a3f8628f85184b8` |
| 5 | `crates/solo-application/tests/graphic_asset_read.rs` | **new** → `b74d7d28dddbe396808c35684149d30bdc66b241` |
| 6 | `crates/solo-adapters/src/source_read.rs` | new port impl + asset locator, `pages/` preserved → `fb6f005a053ea80cea452fe2d57fb9fc8ea45ad9` |
| 7 | `crates/solo-adapters/src/lib.rs` | crate documentation only → `107f3579f01878f7532d66c93e150f8aafc4f42f` |
| 8 | `crates/solo-adapters/tests/graphic_asset_read.rs` | **new** → `301aa18a16dacf0f0b20ffb2ca34994bf5a3d7b2` |
| 9 | `.documents/Reports/execution-report.md` | overwritten by this report-only final commit |

**Explicitly unchanged after the order**: `crates/solo-adapters/src/source_read_native.rs` (`b0ea31db…`), `crates/solo-application/src/source_read.rs` (`93bc39d4…`), `crates/solo-application/src/graphic_asset_inventory.rs` (`7de9bc73…`), all domain files, `Cargo.lock` (`2f9e5357…`), every `Cargo.toml`, `rust-toolchain.toml` (`9ec55b3b…`), `deny.toml` (`3063e864…`), `.github/` (`eb78dad2…`), `.gitignore`, `.documents/Architecture/`, About, SOP, AGENTS, ROADMAP (`97df81e5…`), OUTLOOK (`cb9c7a61…`), `platform/`, assets and every other test. No new dependency, crate, unsafe allowance, OS primitive, provider or decoder exists.

## 4. Technical interface summary

**Application (pure; `solo-domain` + `std` only, no filesystem import).**
`MAX_GRAPHIC_SOURCE_BYTES = 20 * 1024 * 1024`; `GRAPHIC_ASSET_READ_GRANTS_PUBLICATION_AUTHORITY = false`; `GRAPHIC_ASSET_READ_VALIDATES_MEDIA = false`. `EntityAssetPathV1::from_reference` is the fail-closed portable boundary that maps an already typed `AssetReferenceV1` to `assets/` plus its unchanged authored segments. `EntityAssetSourceV1` carries the adapter's observations. `GraphicAssetReadPort::read_entity_asset(reference, max_bytes)` is the minimal inward port. `read_graphic_asset_bytes_v1(inventory, port)` is the whole-inventory use case: it refuses `PlatformUnverified` before any outbound call, re-checks the bound, the exact transferred length, the link count and identity consistency, and returns either complete ordered `GraphicAssetSourceEvidenceV1` records (obligation, logical path, owned bytes, identity, length, fingerprint) or one `GraphicAssetReadRefusalV1` naming the declaring owner, role and local index.

**Adapter (safe Rust; unchanged native boundary).** `impl GraphicAssetReadPort for ContentRootCapability` re-validates the path itself, refuses a declared length above the bound **before** any allocation or clone, and then uses only `native::open_leaf`, `native::read_bounded` and `native::recheck` — the accepted Linux `openat2` `RESOLVE_BENEATH | NO_SYMLINKS | NO_MAGICLINKS | NO_XDEV` mechanism and the accepted Windows per-component `NtCreateFile` no-reparse mechanism. It never calls `std::fs::read`, `canonicalize`, a pre-open `metadata` or any fallback, and it accepts no `SourceCandidatePathV1`.

**Strict filename guard (defence in depth, re-run immediately before the native open).** Refused exactly as authored, never lowercased, trimmed, shortened or repaired: uppercase anywhere, a first character that is not `a`–`z`, a component longer than 80 bytes, a complete logical path longer than 240 bytes, more than 8 segments including `assets/`, an empty, `.` or `..` component, a trailing dot or space, adjacent dots, a reserved Windows stem (`con`, `prn`, `aux`, `nul`, `com1`–`com9`, `lpt1`–`lpt9`) at any extension, and any byte outside `[a-z0-9._-]`. The final segment must carry exactly one admitted lowercase extension (`png`, `jpg`, `jpeg`, `webp`, `avif`). The domain parser and the accepted Page locator grammar are untouched.

## 5. R03C2-01…12 acceptance and refusal matrix

All twelve rows are evidenced on **both** hosts. The 16 focused suites are named `r03c2_XX_…` and were observed passing on `ubuntu-24.04` and `windows-2025` in the final branch run.

| Row | Evidence |
| --- | --- |
| **R03C2-01** | `r03c2_01_complete_inventory_yields_ordered_evidence_for_every_authored_value` — the real five-level graph yields six ordered records: entity logo (`internal`), facet banner, two site backgrounds (local indexes 0 and 1), page social preview and the descendant-suppressed representation favicon. Owner descriptions, roles (`graphic.logo`, `graphic.banner`, `graphic.background`, `graphic.social-preview`, `graphic.favicon`), indexes, exposures, namespaces, references, logical paths, bytes, lengths and identities are asserted per record, and a second inventory built for a different requested target yields identical evidence. |
| **R03C2-02** | `r03c2_02_empty_inventory_makes_no_calls_and_private_inventory_still_reads` — an empty authoring set returns one empty complete set with **zero** port calls; an all-`internal` inventory still reads both declared sources and grants no publication right. |
| **R03C2-03** | `r03c2_03_platform_only_and_mixed_inventories_refuse_before_any_outbound_call` — Platform-only and mixed inventories (Platform declared after an Entity value, and `internal`) both refuse `ASSET_PLATFORM_UNVERIFIED` with **zero** outbound calls, naming the site/background obligation. |
| **R03C2-04** | `r03c2_04_same_filename_at_two_owners_is_two_obligations_and_reread_must_agree` — one literal filename authored by the Entity and by the Site stays two evidenced obligations, each read once; a changed identity on the second observation refuses `IdentityCollision`, and same-identity different content refuses `ChangedDuringRead`. |
| **R03C2-05** | `r03c2_05_injected_failures_refuse_the_whole_candidate_without_partial_evidence` — six injected categories each produce their exact code (`ASSET_SOURCE_MISSING`, `…_PERMISSION_DENIED`, `…_CHANGED`, `…_UNSAFE_LOCATION`, `…_UNSUPPORTED`, `…_IO_ERROR`) after the earlier successes are discarded, and the rendered diagnostic contains neither `assets/` nor the authored filename. |
| **R03C2-06** | `r03c2_06_bound_is_exactly_twenty_mib_and_oversize_is_never_truncated` (application, injected) and `r03c2_06_real_bound_accepts_twenty_mib_and_refuses_one_byte_more` (real filesystem, both hosts) — exactly 20 MiB succeeds, 20 MiB + 1 is refused, a reported length beyond the bound is refused even with short bytes, a zero-byte source is acquired with `media_verified() == false`, and a short transfer is a change. |
| **R03C2-07** | `r03c2_07_link_count_and_physical_identity_collisions_are_refused` plus the real hardlink case in `r03c2_08_leaf_intermediate_and_hard_links_are_refused` — link count 2 refuses `IdentityCollision`, and two distinct logical paths observed at one physical identity refuse even when the port reports a benign link count. |
| **R03C2-08** | Six real cases per host: nested `assets/` read with exact bytes/length/identity; missing leaf and missing `assets/` namespace; `pages/` isolation in both directions; directory leaf; leaf symlink, intermediate junction/symlink and hardlink; prohibited spellings refused before any open while the lowercase file stays readable. |
| **R03C2-09** | `r03c2_09_portable_boundary_refuses_prohibited_spellings_without_repair` — admitted lowercase labels (`a.png`, `brand/logo.png`, `bg/deep-nested_1.jpeg`, `i/f.webp`, `x/y.avif`, `j/photo.jpg`) map to `assets/…`; uppercase, reserved stems, leading digit, leading underscore, adjacent dots, an over-long component and an over-deep path are refused with the authored text unchanged; spellings the domain already refuses are shown to be unreachable; a non-PNG body behind a `.png` name is acquired and still reports no media validity. |
| **R03C2-10** | `r03c2_10_same_size_rewrite_is_observed_and_never_served_from_memory` on both hosts — a same-length in-place rewrite is returned by the next read rather than cached, with the same identity. Deterministic fake-port before/after change cases cover the identity/content transitions; the real native confinement, writer-denial and reflink refusal mechanisms are exercised unchanged by the accepted R02 suites and the Ubuntu-only native negative probe. |
| **R03C2-11** | Every earlier suite still passes: workspace totals rose from 384/386 to **400/402** with no regression, the architecture checker reports `architecture=legal packages=4 edges=40` on both hosts, strict Clippy and `cargo fmt --check` pass, and the new code adds no dependency, primitive or unsafe (application forbids `unsafe_code`; the adapter keeps its single Windows-gated allowance). |
| **R03C2-12** | Four separate two-host `rust-verify` runs are green: issuance `43026432918`, branch tip `43029085066`, substantive `main` `43030195684`, and the report-HEAD run recorded in the chat handoff. Fresh A1 review is in §6. |

**One red attempt, disclosed.** Branch run **`43028102915`** at `1048bf26` failed on both hosts in exactly one application test: `r03c2_01` compared role labels as camelCase while `PropertyKey::name()` renders dotted names (`graphic.logo`, …). Every other compared field already matched. The correction was confined to that test expectation in path 5 (commit `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`, blob `b74d7d28…`); no production code, bound, refusal or policy changed, and no published commit was amended or rewritten.

## 6. Fresh A1 supply-chain and source-security assessment

| Item | Finding |
| --- | --- |
| Locked graph | 22 packages: 4 first-party and **18** third-party (`bitflags 2.13.2`, `errno 0.3.14`, `itoa 1.0.18`, `libc 0.2.191`, `linux-raw-sys 0.12.1`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `rustix 1.1.6`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `windows-link 0.2.1`, `windows-sys 0.61.2`, `zmij 1.0.23`). Unchanged by this order; `rustix` stays pinned at `=1.1.6`. |
| RustSec | the CI gate `cargo-deny 0.20.2` passed on both hosts with a fresh database (`security_gate=pass db_commit=fd6f39814c4d3c359fdbe223d0280dec4933f78f fetched_at=2026-11-11T13:26:19Z`), with the workflow's archive integrity verification. No waiver, ignore or policy relaxation exists in `deny.toml`. |
| OSV / GHSA sweep | 18/18 locked third-party versions returned **no** advisories from `api.osv.dev` (crates.io ecosystem) in this session. |
| Yank and provenance | 18/18 versions report `yanked = false` from the crates.io API; CI re-verified the locked fetch and lockfile checksums. |
| Toolchain and runners | pinned `1.99.1` with `rustfmt` and `clippy`; workflow actions and hosted runners unchanged (`ubuntu-24.04`, `windows-2025`). |
| New-code reachability | the new code opens files only through the unchanged native boundary, uses no network, no `unsafe`, no FFI and no dynamic loading; the 20 MiB bound is enforced before allocation; every failure is a typed refusal that leaks no path or byte. |
| Inherited warning | the non-fatal `warning[license-not-encountered]` for `Unicode-3.0` at `deny.toml:51` is still emitted on both hosts (4 log occurrences). It remains **UNVERIFIED**: not suppressed, and its disposition left unchanged without evidence. This run is **not** warning-free. The inherited `itoa 1.0.18` and `syn 3.0.9` patch lags are outside this order and untouched. |
| Skips | host-inapplicable only: 9 on `ubuntu-24.04`, 14 on `windows-2025` (counterpart-host steps, the Ubuntu-only advisory negative probe and byte-reproducible double build, R03 snapshot filesystems). No test was ignored or silently skipped in place of a passing assertion. |


## 7. Test totals, integration and branch integrity

| Item | Value |
| --- | --- |
| Issuance (`2fb7e34b…`) | 384 / 386 passed, 0 failed, 0 ignored (behaviour unchanged) |
| Branch tip (**`7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`**, tree `2e6932c1614aae62540f59e1b53b81f93566fde4`) | **400 / 402** passed, 0 failed, 0 ignored; 16 distinct `r03c2_*` suites per host |
| Integration | fast-forward-only from `main` `2fb7e34b…`; `main` now equals the tested branch tip `7f609320…`, single-parent linear history, no merge commit |
| Substantive `main` CI | **`43030195684`** — `success`; `130554959616` (`ubuntu-24.04`) and `130554959083` (`windows-2025`), 33 steps each, 0 failures, 9/14 skips, **400 / 402** passed |
| Retained branches | `solo-wo-061-r01` unchanged locally and on `origin` at `4371af21dce67f33cf1df0384738bf353ca80fc6`; **`solo-wo-065-r01` retained** at its verified tip `7f609320…` on GitHub pending independent acceptance. No branch was pruned, force-deleted or rewritten. |
| Local execution | **NOT RUN** — this Windows host has no MSVC `link.exe`, so no local binary can be linked. Local `cargo fmt --all -- --check` and strict `cargo clippy -p solo-application --all-targets -- -D warnings` pass; all behavioural evidence is the two-host CI runs above. |

## 8. Workspace, cache and temporary hygiene

| Item | State |
| --- | --- |
| Checkout / worktree / remote | one verified checkout, one worktree, sole `origin`; no second clone, shadow tree or nested repository |
| Working tree | clean at every published revision (`git status --porcelain --untracked-files=all` empty) |
| Live build cache | ignored `target/` preserved and never cleaned |
| Inherited temp archives | Solo `.temp/` preserved at **2 files / 289,253,806 bytes**, untouched |
| Task-owned scratch | the two Cline task log files created by this run's long CI waits were deleted; none remain. No Solo scratch file was created for this run. |
| Unknown material | nothing unknown, shared or Owner-owned was deleted, moved or overwritten; no `git clean`, destructive reset, force-push or unapproved branch deletion occurred |

## 9. What remains open

`R03.C` is **not** complete and Stage 03 is **not** closed. Still unimplemented and explicitly outside this order: authoritative media-byte, format/magic/MIME, image-dimension and pixel-limit verification; an Owner-reviewed lawful `platformAssetLibraryVersion: "1.0.0"` catalogue with asset IDs, hashes, media class, dimensions and licence/provenance; whole-candidate validation that includes that pinned Platform library; and privacy-safe public use with immutable derived outputs. A `namespace: "platform"` reference is deliberately **unverifiable** here and refuses the whole attempt. No catalogue, artwork or derived file exists on the tracked tree, and no automatic platform or role fallback is permitted.

**Unverified limitations.** The exact-bound and over-bound behaviour is proven through the public adapter on both hosts, but a Linux filesystem without reflink support is exercised only by the workflow's existing native negative probe rather than by a dedicated asset fixture, because the test harness requires the verified snapshot filesystem; the asset path inherits that mechanism unchanged. Media correctness is never asserted, by design. The `Unicode-3.0` licence warning remains unverified as recorded above.

## 10. Handoff

`COMPLETE` is reported only for this bounded, nonpublishing source-byte acquisition slice. The Implementation Agent does **not** self-ACCEPT and does not close `R03.C`, Stage 03 or the order: the frozen ACTIVE order is preserved, NEXT is the neutral heading, ROADMAP and OUTLOOK are unchanged, and `solo-wo-065-r01` remains at its verified tip for independent review. The Chief Architect independently returns `ACCEPT / REVISE / STOP` against this report's exact remote HEAD.
