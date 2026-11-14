# Execution Report

**Work order:** `SOLO-WO-114 R01` — controlled exact-blob integration of the accepted four-format GIF V2 and N102 work into `main`.
**Date:** 2026-11-14. **Approved bounded outcome:** `R03C-FOUR-FORMAT-GIF-V2-AND-N102-MAIN-INTEGRATED-VERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

The Owner-issued order was installed as exact bytes and executed as three named checkpoints: **A** authority, **I** integration, **R** report. All 15 ledger paths now sit on `main` byte-for-byte as accepted on `solo-wo-113-gif-v2-r01`; the integration run at exact **I** `062176427faa9f38091149434e5135ea3ab593ee` was green on Ubuntu 24.04 **and** Windows 2025 on **attempt 1**, including N113 4/4, N108 8/8 and Windows N102 4/4 + 12/12. No accepted source byte was altered, no history was merged, and `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

## 1. Owner authority, preflight and checkpoint commits

The Owner installed the exact R01 text at `.documents/WIP/workorder-active.md` (raw `git hash-object` and Git-filtered blob both `5041c71c181c6767a8af3ea382d339fb18afeee0`, 67 lines) and issued the matching activation handoff.

| Step | Commit | Tree | Parent | Contents |
|---|---|---|---|---|
| initial `main` | `b42003f62f2efd2c828be0b7310f0c8bae4caa39` | `a5315673` | `07aef033` | R03 report, no integrated source |
| **A** authority | `14cc2ad83bbf87bced79fe95472ba3c8c2f557aa` | `a1d59f659ddd9838fc3626dbf5af2285ab8bfeee` | `b42003f6` | ACTIVE only |
| **I** integration | `062176427faa9f38091149434e5135ea3ab593ee` | `7b9bc45b06194317b579ad0d24ddd906ea593597` | `14cc2ad8` | exactly the 15 ledger paths |
| **R** report | reported in chat after publication | — | `06217642` | report only |

Preflight confirmed: one Windows/WSL Solo checkout, one `origin` to `provelopment/provelopment-foundation-solo`, local `main` **and** remote `main` at `b42003f6`, accepted branch local **and** remote at `ad7b74df` (tree `ca675fa4`, parent `b0f204a2`), 26 remote branch refs, one worktree, empty stash, exactly four WIP files, and a tree clean apart from the Owner-installed ACTIVE. `main` had advanced from `e698ad80` only through `.documents/WIP/workorder-active.md` and `.documents/Reports/execution-report.md` — **no GIF/V2 or N102 source existed on `main` beforehand**, and all 15 ledger paths differed from `main` (the GIF adapter file was absent entirely). Governance blobs were verified at their pinned SHAs: `AGENTS.md` `6eae1da8`, `ARCHITECTURE.md` `f66096ec`, Architect SOP `1483c9be`, Implementation SOP `51bb3792`, architecture source checker `crates/solo-cli/tests/architecture_checks/source.rs` `1c99d348`, unsafe checker `crates/solo-cli/tests/unsafe_boundary.rs` `57d75796`, root `Cargo.toml` `14505be4`, `rust-toolchain.toml` `9ec55b3b`, and `plan-roadmap`/`plan-next`/`plan-outlook` `9d7a8622`/`92fa562b`/`cb9c7a61` — 11/11 identical on `main` and on the accepted branch. A was committed with only the ACTIVE path staged and the GitHub blob was read back as `5041c71c` matching the installed bytes.

## 2. The 15-path integration ledger — source, staged, committed

Integration used path-limited `git restore --source=ad7b74df… -- <15 explicit paths>`, never an overwrite of unrelated data, never a cherry-pick, rebase or branch merge. Each path was checked three times before commit (source `ad7b74df:<path>`, filtered worktree `git hash-object`, and index) and a fourth time after commit (`I:<path>`), with **15/15 matches on all four checks**.

| # | Path | Blob (source = worktree = index = committed) |
|---|---|---|
| 1 | `.github/workflows/rust-verify.yml` | `db2bc14b2ef53dbb1971483327e4ccd9a7cd6394` |
| 2 | `Cargo.lock` | `cf1051675576ab0e00ce7acbd5dbe139882c31b4` |
| 3 | `crates/solo-adapters/Cargo.toml` | `3610f06ff58cdea6ce1b3993544f0c2e286087ea` |
| 4 | `crates/solo-adapters/src/graphic_gif_inspection.rs` | `785d08d0e5a9009be1cbb015eb1b88362f6ab311` |
| 5 | `crates/solo-adapters/src/graphic_image_inspection.rs` | `14af5029d1ecd8cda85d3470eedbf685b5fc8463` |
| 6 | `crates/solo-adapters/src/lib.rs` | `10817e4b73db5773c563958ab1def2cdc03610a1` |
| 7 | `crates/solo-adapters/src/media_worker_host.rs` | `894bc6220d90161ce98bb9c0c848ff208388f666` |
| 8 | `crates/solo-adapters/src/media_worker_host_native.rs` | `9a26e195b295974b54f7e2da7ce7ae8fbc510c4a` |
| 9 | `crates/solo-adapters/tests/property_resolution.rs` | `18bafe2f4059845e026fd77554957fb388b9661e` |
| 10 | `crates/solo-application/src/graphic_image_inspection.rs` | `658cb36320e7307b92f7437d76f5e06911e9367a` |
| 11 | `crates/solo-application/src/lib.rs` | `2d1f71b4dcb21396aeb9543a6ad63d04e5e99313` |
| 12 | `crates/solo-cli/tests/architecture_checks/graph.rs` | `dc6d2770038a14199d2f366faab64db4e3d6e98b` |
| 13 | `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1aa18b3d6c944f0a0da7850ada7c00154c9a193d` |
| 14 | `crates/solo-domain/src/property_registry.rs` | `889d0fa75af62574944e422811b9fe7a6ae93c26` |
| 15 | `crates/solo-domain/tests/property_resolution.rs` | `1b69ae6d9cd0d01414174911a6f2868c60979570` |

`git diff --cached --name-status` showed exactly 15 entries (14 `M`, 1 `A` for the new GIF adapter) and no 16th path; the worktree had no untracked file and nothing dirty outside the stage. After commit, `git diff-tree --name-only I` lists exactly those 15, and `git diff --name-only e698ad80 I` contains exactly 17 paths — the 15 plus the Owner ACTIVE and the previous canonical report — so **no change exists outside the three permitted ledger categories**. The two previously reconciled regression-test files were imported at their exact accepted blobs without repeating the R02 omission.

## 3. Local-first confidence on the exact imported bytes (before commit and push)

Executed on the checked-in staged bytes in the single verified WSL Ubuntu checkout:

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | clean |
| `cargo check --workspace --all-targets --locked` | clean |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | **0** findings |
| `cargo check --workspace --all-targets --locked --target x86_64-pc-windows-msvc` | clean (type-check only; no MSVC linker here) |
| `solo-domain --test property_resolution` | 27 passed |
| `solo-adapters --test property_resolution` | 20 passed |
| `solo-application --lib` | 15 passed |
| `solo-adapters --lib graphic_gif` | 14 passed |
| `solo-adapters --lib n102_event` | 3 passed |
| `media_worker_host_smoke n108_` | 8 passed |
| `unsafe_boundary` | 22 passed |
| `check-architecture` | exit 0, `architecture=legal packages=4 targets=33 files=63 edges=70` |
| `check_architecture_fixtures` | 35 passed |

Full workspace suite `cargo test --workspace --all-targets --locked --no-fail-fast`: **26 targets passed, 517 tests passed**. Six targets cannot run here and fail only for environment reasons — `source_read_native::snapshot_proofs` (5 cases), `d2_product` (4), and the real-filesystem `n097` suite (`n097_01`–`n097_08`, `n113_01`–`n113_04`). All 73 failure messages are the same refusal, e.g. `SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem; refusing to run the real-filesystem suite on a filesystem that was never verified to support an immutable clone`, plus two `SOLO_TEST_UNSUPPORTED_FS_ROOT` refusals. **Zero semantic failures.** This host has no `mkfs.btrfs`, no passwordless sudo, no `SOLO_TEST_SOURCE_FS_ROOT` and no native Windows Rust toolchain, so those cases are honestly unproven locally and are proven instead by the hosted run in §4. No test, workflow or configuration byte was altered to make any gate pass, and no `#[ignore]` was added.

## 4. Hosted integration evidence at exact I — attempt 1, both hosts

Automatic exact-SHA run [`43607380648`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43607380648), head `062176427faa9f38091149434e5135ea3ab593ee`, **run attempt 1**, conclusion `success`, **40 of 40 steps success on each host, zero non-success steps**.

| Host | Job | Result |
|---|---|---|
| `ubuntu-24.04` | `132484847682` | success, 40/40 steps, 0 non-success |
| `windows-2025` | `132484847470` | success, 40/40 steps, 0 non-success |

**Four-format V2 (`n097::n113_01`–`n113_04`) — exactly 4/4 on each host**, in both the shared workspace test step and the dedicated per-host `V2 four-format image inspection gate`:
* Ubuntu `n113_mixed formats=4 owners=4 order=png,jpeg,webp,gif pids=[5614, 5618, 5622, 5626]`; Windows `pids=[9404, 10160, 9872, 5888]` — four distinct declaring owners and four real child processes per host, PIDs different by construction and never compared.
* `gif_frames=3 gif_loop=infinite gif_cycle_ms=350` — full three-frame GIF89a animation preserved.
* Deterministic digests identical across both hosts: GIF source `dc425aeadeee8daa6324c0c6093710c69f6ad6bcf6d58c12571e88df1632b84f`, transcript `8c02ba6fe2d4fea8edabba2378db8a696bd166b5fba72315a93924341019ac33`, RED `34aaa746c25a0f105c4316bbb1f009aa359f49582656ee97d73c58132d563423`, BLUE `b7d1b3a1104cc86b1cea310793cf777002db0517281d135a02de079b0ea87c23`, clear `df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119` — matching the accepted WO-113 R03 technical run.
* Negative and sensitivity gates on both hosts: `n113_hidden refused_whole=true discarded=2 partial=false` (whole-candidate fail-closed, no partial result), `n113_declared all_cases_refused=true` (declared-format and PNG-only favicon refusals), `n113_red … oracle=red source_distinct=true transcript_distinct=true` (deliberate-red oracle load-bearing), `n113_forged adapter_wrong_source=refused adapter_wrong_transcript=refused partial=false`.

**N108 real GIF worker canaries — exactly 8/8 on each host**, with parent-authenticated evidence, e.g. `n108_08_gif_source_transcript_and_pixel_change_evidence … case=A1 src=aef0685e1c2625c61fac6647d9d7dd908b5ecc7eb3ebcf46e857f55788484915 transcript=370a685d7526ccf614ab4e456958f8845a607076010af7e18bc824ae25db61a1 pixel=34aaa746…3423`; retained N097 PNG/JPEG/WebP, snapshot/confinement, strict identity and cleanup assertions all green.

**Windows N102 correction — 4/4 plus 12/12 native repeats**: `n102_01 extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true` observed with distinct real child PIDs (`9684`, `7804`, …), then `N102 repeat 12/12 passed` and `N102 event-identity repeatability: 12/12 consecutive native Windows runs passed.`

Also green on both hosts, unchanged: snapshot-test filesystems provisioned and cleaned, format check, strict lint, workspace all-target tests, architecture checker on the real checkout (`architecture=legal packages=4 targets=33 files=63 edges=70`), architecture refusal self-test, unsafe guard, test-failure tripwire, workspace package-set gate, release build, release CLI smoke, forbidden-import/bypass refusal with guaranteed restore, and the two byte-for-byte compared clean release builds.

## 5. Security, dependency and warning assessment

The accepted locked graph was imported exactly: `Cargo.lock` is blob `cf105167` (no new package, feature, version, toolchain, runner label or action pin). `cargo-deny 0.20.2` ran on **both** hosts from the verified archive (`DENY_SCANNER_SHA256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`, `DENY_LOCK_SHA256 350c6acf987f7497fdae69f6e1126534f522edd1d9f1ed7f49db91e5a72ee1c3`), with a refreshed advisory database and the four checks (advisories, licences, sources, bans) passing. The negative advisory probe still **refuses** the pinned specimen: `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks`.

A1 inventory of the integrated surface: canonical MIME/magic and complete decoded structure are enforced (PNG/JPEG/WebP retained, lowercase `gif` admitted through one allowlist token, uppercase `.GIF`, `.svg`, `.avif`-by-implementation-status and unknown extensions refused); one child process per untrusted input with bounded OS confinement, verified cleanup and no uncontrolled handle inheritance (the N102 correction proves the extra inheritable handle is excluded while the parent positive control still signals); no secrets, no unexpected network, no arbitrary executable content, no publication authority (`grants_publication_authority() == false`, `proves_whole_publication() == false`), no first-frame GIF downgrade, and caller-supplied path semantics unchanged. The GIF limits are retained — 20 MiB source, 8192-side, 32M intrinsic pixels plus the stricter total frame/composited-pixel/duration limits — and the V2 atomic whole-owner design is intact: any bad obligation discards the entire candidate with no partial result. Favicon remains PNG-only. Nothing grants a route, Page publication or a global multi-file snapshot. No new `unsafe` code or guard bypass was introduced; the unsafe checker and architecture checker, with their refusal self-tests, pass on both hosts.

The only warning is the known `warning[license-not-encountered]: license was not encountered` for the unused `Unicode-3.0` allowlist entry on both hosts. It is allowlist hygiene — an allowlisted licence matching no shipped crate — not evidence of an affected shipped licence. It was **not** suppressed and `deny.toml` was not edited. No new actionable security, version, deprecation or runner warning appeared.

## 6. Preservation, cleanliness and remaining Stage obligations

`main` is A + I + this report; the accepted technical branch `solo-wo-113-gif-v2-r01` remains at `ad7b74df` untouched and `solo-wo-112-gif8-host-r01` at `40b03c73`, with the remote branch inventory at **26** before and after — no branch created, deleted or force-pushed. `plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, `README`, `AGENTS.md`, Architecture, SOPs, root `Cargo.toml`, `rust-toolchain.toml`, the architecture/unsafe checkers, `deny.toml` and `.gitignore` were not imported or modified. One checkout, one `origin`, empty stash, four WIP files, no second worktree, clone, cherry-pick, rebase, reset, `git clean`, tag, release, deployment, PR, merge, provider/account operation or other-repository write. The final `git status --porcelain` is empty after this report commit.

**Task-owned temporary material:** this run created no files in `.temp/` — the local full-suite logs were written to the disposable WSL `/tmp` area only, and nothing was tracked. Pre-existing `.temp/` directories from earlier orders (`architecture-fixtures`, `solo-win-target`, `solo-wo-073-wip`, `solo-wo-098-wip`, `solo-wo-103-wip`, `solo-wsl-target`, `toolchain-installer`) were left untouched; an empty `git status` is not evidence of clean temp directories, and none of them is this run's material.

**Stage status unchanged and binding:** `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. This integration delivers a proven four-format image inspection slice into the product branch; it does **not** finish Stage 03 graphics. **AVIF** is admitted but still unimplemented, **SVG** must be safe true-vector rather than flattened, the exact pinned licensed `platform/assets/` catalogue with IDs, hashes, class, dimensions and licence/provenance still does not exist, and whole-candidate derived-output/exposure/publication obligations are unfinished. Stage 04 routing/visibility/publication is not treated as finished. No release, deployment or Stage closure is claimed.

## 7. Acceptance self-assessment

| Requirement | State |
|---|---|
| Exact Owner installation/handoff and A | **PASS** — raw == filtered == GitHub `5041c71c`; A `14cc2ad8` ACTIVE-only |
| 15/15 source/worktree/index/commit/remote SHA match | **PASS** — four-way equality on all 15; remote `main` = `06217642` |
| No change outside the three ledger categories | **PASS** — 17 paths since baseline = 15 + ACTIVE + previous report |
| No reimplementation or unapproved version changes | **PASS** — path-limited restore; `Cargo.lock` `cf105167` |
| Unchanged accepted source/security boundary | **PASS** — §5, both-host guards and negative probes |
| Exact technical branch and all 26 prior refs preserved | **PASS** — `ad7b74df`, `40b03c73`, 26 refs |
| All applicable local gates | **PASS** — §3; fixture-bound suites honestly listed as host-limited |
| Real Ubuntu/Windows full-I gates: N113 4/4, N108 8/8, Windows N102 4/4+12/12 | **PASS** — run `43607380648` attempt 1 |
| Current A1 security/advisory negative probe | **PASS** — §5 |
| Precise report-only R and fresh green main R CI | **R published**; its exact SHA and the fresh both-host run at that SHA are reported in chat, since a report cannot embed its own commit |
| No unauthorized merge/release/deploy/Stage closure | **PASS** — §6 |
| Clean final state and honest temporary cleanup | **PASS** — §6 |

`status=COMPLETE — pending independent Chief Master Architect review.` The agent does not self-accept and awaits the Architect's independent `ACCEPT`, `REVISE` or `STOP` on the final SHA.



