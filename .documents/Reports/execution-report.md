# Execution Report

**Work order:** `SOLO-WO-113 R03` — reconcile two out-of-ledger regression-test edits and verify the accepted four-format GIF V2 tree.
**Date:** 2026-11-14. **Bounded outcome:** `R03C-GIF-MIXED-V2-OWNER-RECONCILED-AND-REVERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

The reviewed technical tree is unchanged and re-verified: the only difference between the formerly reviewed head `b0f204a2` and the current technical head `ad7b74df` is `.documents/WIP/workorder-active.md`. Both hosts passed on the **first attempt** of run [`43605231196`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43605231196) at that exact SHA: Ubuntu job `132478354577`, Windows job `132478354755`, every step success. Nothing is merged, released, deployed or closed.

## 1. Owner handoff, authority commits and the exact prior deviation

This run began only after the Owner installed the exact `SOLO-WO-113 R03` bytes at `.documents/WIP/workorder-active.md` (working-tree blob `03fe013a7253d9bf007e9891eee90cf189a71989`) and issued a matching R03 handoff. Preflight confirmed `main == origin/main == 283932b553e5306051a389e17b1665c0e7f3f254` (tree `69aa8548`, parent `c595d27b`, R02 report blob `24817428`), the existing technical branch `solo-wo-113-gif-v2-r01` at `b0f204a2b2de470f1b0cfe8ef3631fcd51fac72d` (tree `eca5f96c`, parent `430c37ee`) locally **and** remotely, 26 remote branches, one checkout, one `origin`, empty stash, exactly four WIP files, and a clean tree apart from the Owner-installed ACTIVE.

| Step | Commit | Tree | Parent | Contents |
|---|---|---|---|---|
| R02 head (reviewed) | `b0f204a2b2de470f1b0cfe8ef3631fcd51fac72d` | `eca5f96c` | `430c37ee` | unchanged product/test tree |
| **A3** authority (`main`) | `07aef0330fe2db8fbdb27b5bf2d8222bdaeea415` | `3a7da62a` | `283932b5` | ACTIVE only, blob `03fe013a`; GitHub blob verified |
| **T1** authority (branch) | `ad7b74df8638ff56c7137f33d9242da54583f17b` | `ca675fa4` | `b0f204a2` | ACTIVE only, blob `03fe013a`; pushed, remote verified |

**The deviation being reconciled, stated without softening.** Under R02 this agent changed two regression-test files that the frozen R02 ledger did not list. R03 does **not** retroactively authorize those edits, and this report makes no compliance claim about them:

1. `crates/solo-domain/tests/property_resolution.rs` — blob **`1b69ae6d9cd0d01414174911a6f2868c60979570`**, changed in C0 `3b5a2498`.
2. `crates/solo-adapters/tests/property_resolution.rs` — blob **`18bafe2f4059845e026fd77554957fb388b9661e`**, changed in C3 `b0f204a2`.

Both were disclosed in the R02 report (§3) at the time; the Architect's REVISE was correct. R03 converts them into **prospectively Owner-adopted contents of the reviewed tree**: the Owner's installation of R03 is approval to *retain these exact bytes going forward*, not a finding that editing them under R02 had been permitted.

## 2. Independent verification of the two retained blobs and the whole change set

`git rev-parse b0f204a2:<path>` and `git rev-parse T1:<path>` were both compared against the exact 40-character SHAs quoted by the order: **all ten checked blobs match on both revisions**, including the two retained test files, so T1 retains them byte-for-byte and neither file was touched during R03.

The complete `79c4dccf..b0f204a2` change set contains **exactly nine paths**:

| Path | R02 ledger status |
|---|---|
| `.documents/WIP/workorder-active.md` | authorized (T0) |
| `.github/workflows/rust-verify.yml` | authorized (D) |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | authorized (C1/C2) |
| `crates/solo-adapters/src/lib.rs` | authorized (`crates/solo-adapters/src/lib.rs` is in the R02 ledger) |
| `crates/solo-application/src/graphic_image_inspection.rs` | authorized (C1) |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | authorized (C1/C2) |
| `crates/solo-domain/src/property_registry.rs` | authorized (C0) |
| `crates/solo-adapters/tests/property_resolution.rs` | **the disclosed out-of-ledger file** |
| `crates/solo-domain/tests/property_resolution.rs` | **the disclosed out-of-ledger file** |

No third unauthorized path exists; `graph.rs` was never changed; there is no suppressed, deleted or ignored test.

The patches were inspected directly, not inferred from commit messages:
* C0 `3b5a2498` replaces only the stale `assets/logo.gif` refusal in `r03a_06_duplicate_and_malformed_entries_refuse_without_repair` with lowercase-GIF admission plus an uppercase `.GIF` refusal; the neighbouring `../escape.png`, `.svg`, remote-URL, drive-path, missing-extension and `.PNG` refusals are untouched.
* C3 `b0f204a2` keeps the `media/logo.svg` unsupported-extension refusal, replaces the obsolete GIF refusal with a real manifest validation → real inventory → `grants_publication_authority() == false` admission case, and adds `graphic.favicon` + `.gif` → `FaviconNotPng`; the surrounding PNG-only favicon and nonpublication assertions are untouched.

## 3. Frozen product and verification tree — unchanged from `b0f204a2`

Verified identical at T1 (and therefore at `b0f204a2`): registry `crates/solo-domain/src/property_registry.rs` `889d0fa75af62574944e422811b9fe7a6ae93c26`; inward V2 `crates/solo-application/src/graphic_image_inspection.rs` `658cb36320e7307b92f7437d76f5e06911e9367a`; adapter V2 `crates/solo-adapters/src/graphic_image_inspection.rs` `14af5029d1ecd8cda85d3470eedbf685b5fc8463`; real host tests `crates/solo-cli/tests/media_worker_host_smoke.rs` `1aa18b3d6c944f0a0da7850ada7c00154c9a193d`; workflow `.github/workflows/rust-verify.yml` `db2bc14b2ef53dbb1971483327e4ccd9a7cd6394`; plus `solo-application/src/lib.rs`, `solo-adapters/src/lib.rs` and `Cargo.lock` `cf1051675576ab0e00ce7acbd5dbe139882c31b4` — 10/10 identical, zero drift. Only the ACTIVE blob differs, so the tree SHA necessarily differs (`ca675fa4` vs `eca5f96c`).

`git diff --name-only b0f204a2 T1` returns exactly `.documents/WIP/workorder-active.md`. No product, test, dependency, manifest, lockfile, toolchain, architecture, safety-guard, workflow or other documentation byte moved in this run, and the closed parser/fav-icon/security/real-worker behaviour is therefore exactly the previously verified behaviour.

## 4. Local-first evidence at T1, re-run rather than assumed

In the single verified WSL Ubuntu 26.04 checkout: `cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` clean; `cargo clippy --workspace --all-targets --locked -- -D warnings` zero findings; `cargo check --workspace --all-targets --locked --target x86_64-pc-windows-msvc` clean; `solo-domain --test property_resolution` 27 passed; `solo-adapters --test property_resolution` 20 passed; `solo-application --lib` 15 passed; `solo-adapters --lib graphic_gif` 14 passed; `media_worker_host_smoke n108_` 8 passed; `unsafe_boundary` 22 passed; `check-architecture` `architecture=legal packages=4 targets=33 files=63 edges=70`; `check_architecture_fixtures` 35 passed.

The deliberate-red switch was re-executed against the unchanged frozen test in a disposable capture: `SOLO_N113_RED_ORACLE=wrong` → exit `101` at the independent pixel oracle (`left` `34aaa746…3423`, `right` `b7d1b3a1…7c23`), and the default run passed the oracle and both forged-mode phases before stopping at the same host-fixture limitation. No test source was edited, no case was ignored and no switch was committed.

**Local limitations, disclosed not waived:** this host still has no `mkfs.btrfs`, no passwordless sudo, no `SOLO_TEST_SOURCE_FS_ROOT` and no native Windows Rust toolchain, so the five `snapshot_proofs`, the twelve real-filesystem `n097`/`n113` cases and the other fixture-bound suites cannot execute here; they refuse loudly with `SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem`. A plain directory substitute is refused `Unsupported` by the reader. Their real proof is the hosted runs in §5.

## 5. Hosted evidence at T1 — attempt 1, both hosts

Run `43605231196`, head `ad7b74df8638ff56c7137f33d9242da54583f17b`, run attempt **1**, conclusion `success`.

| Host | Job | Result |
|---|---|---|
| `ubuntu-24.04` | `132478354577` | success, 0 non-success steps |
| `windows-2025` | `132478354755` | success, 0 non-success steps |

* `n097::n113_01`–`n113_04` — exactly **4/4** on each host, inside both the workspace test step and the dedicated `n113_` gate. Ubuntu printed `n113_mixed formats=4 owners=4 order=png,jpeg,webp,gif pids=[5599, 5603, 5607, 5611]`, Windows `pids=[8224, 9868, 8904, 3860]` — four distinct declaring owners and four real child processes per host. `gif_frames=3`, `gif_loop=infinite`, `gif_cycle_ms=350`.
* Cross-host deterministic digests identical, PIDs never compared: GIF source `dc425aeadeee8daa6324c0c6093710c69f6ad6bcf6d58c12571e88df1632b84f`, transcript `8c02ba6fe2d4fea8edabba2378db8a696bd166b5fba72315a93924341019ac33`, RED `34aaa746c25a0f105c4316bbb1f009aa359f49582656ee97d73c58132d563423`, BLUE `b7d1b3a1104cc86b1cea310793cf777002db0517281d135a02de079b0ea87c23`, clear `df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119`.
* Refusal and sensitivity evidence on both hosts: `n113_hidden refused_whole=true discarded=2 partial=false`, `n113_declared all_cases_refused=true`, `n113_red … oracle=red source_distinct=true transcript_distinct=true`, `n113_forged adapter_wrong_source=refused adapter_wrong_transcript=refused partial=false`.
* `N108_01`–`N108_08` — exactly **8/8** on both hosts (`n108_08_gif_source_transcript_and_pixel_change_evidence ... ok`); retained N097 PNG/JPEG/WebP cases green; Windows `n102` **4/4** with `extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true`, and `N102 repeat 12/12 passed` / `N102 event-identity repeatability: 12/12 consecutive native Windows runs passed.`
* Real source snapshot proofs, confinement, strict Clippy/fmt, workspace tests, architecture checker with its refusal self-test, unsafe boundary, test-failure tripwire, two-host cargo-deny 0.20.2 with fresh advisory DB and the refusing `RUSTSEC-2026-0009` negative probe (`time 0.3.46`, exit 1, workspace still passing all four checks), and the byte-for-byte reproducible release builds all succeeded unchanged. `architecture=legal packages=4 targets=33 files=63 edges=70`.

## 6. Every red and green attempt, stated truthfully

| Run | SHA | Attempt | Outcome |
|---|---|---|---|
| `43602488897` (R02, D) | `430c37ee` | 1 | **FAILED, both hosts** — shared workspace test step; `r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested` still expected `media/logo.gif` to refuse |
| `43602999853` (R02, corrective) | `b0f204a2` | 1 | **success**, Ubuntu `132471572016`, Windows `132471571758` |
| `43603620598` (R02 report, `main`) | `283932b5` | 1 | **success**, Ubuntu `132473430385`, Windows `132473430194` |
| `43605231196` (R03, T1) | `ad7b74df` | 1 | **success**, Ubuntu `132478354577`, Windows `132478354755` |

The R02 failure is **not** relabelled: D `430c37ee` was not green, its first attempt failed on both hosts, and the green result belongs to corrective commit `b0f204a2` and to T1 `ad7b74df`. No main-only N102 numeric flake occurred in either main run.

## 7. Warnings, security assessment and current A1 position

The only warning is the known `warning[license-not-encountered]: license was not encountered` for the unused `Unicode-3.0` allowlist entry. It is a cargo-deny licence-list hygiene warning about an allowlist entry matching no shipped crate, not evidence of an affected shipped licence; it was not suppressed, `deny.toml` was not modified, and it is the same warning the Architect already reviewed. No new actionable security, version, deprecation or runner warning appeared on either host; `Cargo.lock` still holds exactly 43 packages at blob `cf105167`; the pinned toolchain `1.99.1`, pinned runner labels, pinned action SHAs and `cargo-deny 0.20.2` are unchanged.

A1 position: no capability assertion is weakened by this run. The four-format V2 result still reports `grants_publication_authority() == false` and `proves_whole_publication() == false`, the favicon stays PNG-only, `.avif`/`.svg`/unknown extensions stay refused, and the unlisted/visibility and platform-catalogue obligations remain open under §8.

## 8. Non-goals, repository state and remaining binding work

`main` is A3 plus this report only; it still carries no GIF source, no V2 code and no corrected N102 code. The technical branch `solo-wo-113-gif-v2-r01` is at T1 `ad7b74df` with the full preserved history (`79c4dccf` → `9b08dc6d` → `3b5a2498` → `6152f6d8` → `36466f21` → `430c37ee` → `b0f204a2` → `ad7b74df`) and is **unmerged**. The accepted WO-112 source branch `solo-wo-112-gif8-host-r01` at `40b03c73` and all earlier refs keep their SHAs; the remote branch inventory is **26 before and 26 after** — no branch was created or deleted. One checkout, one `origin`, empty stash, four WIP files, no PR, merge, tag, release, deployment or settings change, and no parent/sibling repository write.

**Task-owned temporary material:** `.temp/r03-diff.txt`, `.temp/r03-c0-test.patch`, `.temp/r03-c3-test.patch` and `.temp/wo113-r03/` (red and default captures) were created for this run and deleted; nothing was tracked. Pre-existing `.temp/` directories from earlier orders (`architecture-fixtures`, `solo-win-target`, `solo-wo-073-wip`, `solo-wo-098-wip`, `solo-wo-103-wip`, `solo-wsl-target`, `toolchain-installer`) were preserved untouched — an empty `git status` is not evidence of clean temp directories, and these are not this run's material.

**Not done, not claimed:** no GIF/V2 or N102 integration into `main`, no PR/merge, no closure of `R03.C3`, `R03.C` or Stage 03, no release or deployment. Still binding: Owner-accepted safe static true-vector **SVG**; real cross-host confined **AVIF**; lawful `platformAssetLibraryVersion: "1.0.0"` with a complete pinned `platform/assets/` catalogue (IDs, hashes, class, dimensions, licence/provenance); verified derived-output/exposure and whole-owner rules; and final controlled integration with green main verification. The Owner's conditional administrative closure applies only once those are genuinely complete and green, which they are not today.

## 9. Narrow reconciliation acceptance — self-assessment

| Requirement | State |
|---|---|
| Current exact authority A3/T1 | **PASS** — `07aef033` on `main`, `ad7b74df` on the branch, ACTIVE `03fe013a` on both, GitHub blob verified |
| Both reviewed test blobs exactly as specified, retained by this new order | **PASS** — `1b69ae6d`, `18bafe2f`, byte-identical at `b0f204a2` and T1 |
| No retroactive compliance claim | **PASS** — §1 states the deviation plainly and limits the Owner action to prospective retention |
| Zero other executable file changes from `b0f204a2` | **PASS** — `git diff b0f204a2 T1` = ACTIVE only; ten frozen blobs identical |
| Unchanged parser/fav-icon/security/real-worker behaviour | **PASS** — §3, §4, §5 |
| Exact technical two-host green T1 run and fresh main report green | **T1 PASS** — `43605231196` attempt 1; the final main run is reported in chat with its exact SHA, since a report cannot embed its own commit |
| Truthful prior failure disclosures | **PASS** — §6 |
| Clean one-checkout repository, no Owner material or Git forbidden operation | **PASS** — §8 |
| Full current A1 and warning assessment | **PASS** — §7 |

`status=COMPLETE — pending independent Chief Master Architect review.` The agent does not self-accept and awaits the Architect's independent `ACCEPT`, `REVISE` or `STOP` on the final SHA.


