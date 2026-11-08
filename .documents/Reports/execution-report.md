# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-08 |
| Work order | **`SOLO-WO-033 R01`** — Stage 01 `R01.B` discovery-root pre-enumeration correction |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-033 R01` |
| Trigger | Chief Architect independent **`REVISE`** of `SOLO-WO-032 R01` (one residual defect) |
| Verified root | Solo checkout, branch `main`, one worktree, sole `origin` = `provelopment/provelopment-foundation-solo` |
| Starting HEAD / tree | `9045b1b84ed0a2164564c4aee24d61cc830bbed3` / `7c97e423156b09e35785b5d73d20f479ca9b8890` |
| Correction commit / tree | **`c34e323ca0b7579430cdb259e4260989c7d1edbb`** / **`addc3cb1f5f43aa5bc9d1c5abc4cf3dc6802cea2`** |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host `x86_64-pc-windows-gnu` |
| **Status** | **`COMPLETE`** — the residual defect is corrected and verified locally. Agent work status only, never Architect `ACCEPT`. |

## Preflight (H-01)

One checkout on `main` at `9045b1b`, no stash, one worktree, sole `origin`, one branch checkout. **Twenty-four reviewed blobs matched with zero drift**: README, AGENTS, About, Architecture, both SOPs, root and `solo-cli` manifests, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `graph.rs`, `mod.rs`, `check_architecture.rs`, `workspace_graph.rs`, `cli_smoke.rs`, `main.rs`, the prior report `e7f73f37`, ROADMAP/NEXT/OUTLOOK, and the two previously corrected files (`source.rs` `8e42f700`, `check_architecture_fixtures.rs` `714fc59b`). The single pre-existing tracked difference was the Owner's complete replacement of ACTIVE with `SOLO-WO-033 R01` (21,738 B, blob `272ef3fc`), preserved byte-for-byte and committed unchanged.

`.documents/WIP/` held exactly four files and no subdirectory. ROADMAP still lists `R01.B` and `R01.C` and **not** `R01.A`; NEXT is its neutral single heading; OUTLOOK is unchanged at 52 B. Neither ROADMAP, NEXT, OUTLOOK nor the ACTIVE text was edited by the agent.

**Known foreign scratch — inventoried, preserved, untouched.** `.temp/rt/tmp/system-commandline-sentinel-files/` contains a 501-byte `dotnet-suggest-registration-git-credential-manager, Version=2.9.1.0, …` file written by an external dotnet/credential-manager service. During this preflight the same service also re-created `.temp/w032/tmp/system-commandline-sentinel-files/` with an identical sentinel after the previous run had removed that directory. Neither path is task-owned; **neither was adopted, modified or deleted**. This run used the independent, freshly created subspace `.temp/w033`, and `discovery_fixtures_are_owned_and_foreign_scratch_is_untouched` asserts the foreign directory is byte-for-byte identical across a full discovery run. Cleanup trigger for the Owner: remove `.temp/rt` and `.temp/w032` once no external service holds them.

The host had no Rust installation, so the same pinned `1.99.1` GNU toolchain was installed into git-ignored, task-owned `.temp/w033`; no tracked toolchain or Cargo configuration changed and no global OS setting was touched.

## The defect and its correction (H-02, H-03)

`rust_files_under` previously queued the package root and called `DiscoveryFs::list(&directory)` at the top of the loop, inspecting original entries only for child directories encountered afterwards. A junction at the **initial package root** — or at the `crates` ancestor above it — could therefore have its contents enumerated and inventoried before any refusal. Other defences in the checker did not make that traversal safe.

The new `validate_discovery_directory` runs for **every dequeued directory, including the first package root**, and **before** `fs.list`: lexical containment of the directory in the package root and of that root in the verified checkout anchor; `inspect_original_chain` over the anchor, the package root and the directory, so every unresolved component from the volume root down must exist, be a real directory and carry no symlink or `FILE_ATTRIBUTE_REPARSE_POINT` (`0x0400`); an inspectable `EntryKind::Dir` rather than a file, redirect or unknown kind (via the production seam); and canonical containment of the directory in the root and of the root in the anchor. A failure records a deterministic `module-source-escape` (redirect / wrong kind / out of boundary) or `unreadable-source` (uninspectable / unresolvable) violation and the directory is **neither listed nor descended**.

The pre-existing per-entry fail-closed handling is retained unchanged: `DiscoveryFs::redirected`/`kind`/`list` errors, unknown entry kinds, non-`.rs` metadata errors, junction detection, and the `target`/hidden-name exclusions applied only after safe inspection. `check_unit`, `SourceReader` read confinement, public-surface traversal, module/`#[path]`/`include!` scanning, `cfg_attr` refusal and the graph guard are untouched; `MapReader` fixtures, deterministic ordering and four-package real-tree coverage are preserved. No new permanent file, dependency or policy was introduced.


## Adversarial tests through the direct discovery path (H-02, H-03, H-04)

All four new cases are ordinary `#[test]`s in `cargo test --workspace --all-targets --locked`, call the **production** `source::rust_files_under` directly, and instrument the existing `DiscoveryFs` seam rather than a mock policy. Identities are repository-relative (`fixture/` = the owned fixture checkout); genuine redirects are Windows directory junctions created with `cmd /C mklink /J`, no elevation and no policy change.

| Case | Construction | Observed result |
| --- | --- | --- |
| **T-01** `a_redirected_discovery_root_is_refused_before_its_first_list` | `fixture/crates/solo-application` is a junction to `fixture/real-target` (inside the same checkout), holding parsable `.rs` with a forbidden `use` | root and child **do** canonicalise consistently (former false positive), the unresolved root **is** a redirect, and the call yields `module-source-escape` with **`list` count = 0**, empty inventory, link and target bytes preserved |
| **T-02** `a_redirected_discovery_ancestor_is_refused_before_its_first_list` | `fixture/crates` is a junction to `fixture/real-crates`, so the package root looks like an ordinary directory | `module-source-escape`, the package directory is **never listed**, **`list` count = 0**, no file inventoried, link and target untouched |
| **T-03** `a_queued_directory_is_validated_before_it_is_listed` | seam fails the *second* inspection of one queued child (`src/nested`), modelling a directory that becomes uninspectable after classification | `unreadable-source`, `src/nested` **not listed**, nothing under it inventoried, while the validated root's `lib.rs` is still inventoried |
| **T-05** `discovery_fixtures_are_owned_and_foreign_scratch_is_untouched` | owned root plus a package-root junction | root is a real non-redirected directory inside `.temp/architecture-fixtures`, the redirect is refused, the link still exists as a link, the target is byte-identical, and `.temp/rt/…/system-commandline-sentinel-files` is byte-for-byte unchanged |

**Retained and re-run (T-04).** `a_redirected_package_root_is_refused_before_its_source_is_used`, `a_redirected_package_ancestor_is_refused`, the redirected `include!`, `#[path]` and default-`mod` cases, `a_conditional_module_path_is_refused_even_with_a_valid_default`, `a_missing_real_module_source_is_refused`, `legal_in_package_loading_still_succeeds`, `a_redirected_scratch_ancestor_is_refused_before_any_write`, `an_existing_fixture_directory_is_never_adopted_or_deleted`, `a_discovery_metadata_failure_on_a_non_source_entry_is_refused`, `a_redirected_discovery_directory_is_refused_and_preserved`, `the_real_discovery_accepts_a_clean_package`, `the_anchor_requires_a_workspace_manifest_a_git_root_and_crates_layout`, the 23 forbidden fixtures, the five legal twins, the bypass self-test and the real-tree contract all pass. Fixture ownership is unchanged: shared parents are inspected and never adopted, fixture roots are exclusive `create_dir`, writes use `create_new`, teardown removes links then owned files then empty directories deepest-first, and `remove_dir_all` is never called.


## Verification on the correction commit `c34e323c` (H-05, H-06)

| Command | Exit | Observation |
| --- | --- | --- |
| `rustc --version` / `cargo --version` | 0 | `1.99.1 (58cf8f9c3)` / `1.99.1 (5f94df478)`, host `x86_64-pc-windows-gnu` |
| `cargo fmt --all -- --check` | 0 | no diff |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | 0 | **zero** warnings |
| `cargo test --workspace --all-targets --locked` | 0 | **11 binaries, 107 tests passed, 0 failed** (103 + 4 new) |
| `cargo test --locked -p solo-cli --test check-architecture` | 0 | `architecture=legal packages=4 targets=12 files=19 edges=22` |
| `cargo test --locked -p solo-cli --test check-architecture -- --self-test` | 0 | `self-check=passed cases=29` |
| `-- --fixture <id>` × 23 registered forbidden ids | 1 each | every one reports `outcome=refused` with `category_match=true`; nothing was weakened to reach this |
| `-- --disable` (unknown switch) | 2 | usage refusal; no disabling flag exists |
| `cargo build --workspace --release --locked` | 0 | release binary |
| `cargo metadata --format-version 1 --locked --all-features` | 0 | 4 members, 15 packages |
| `cargo tree --workspace --all-features --locked` | 0 | `syn` only under `solo-cli` (dev) |
| `cargo build --workspace --release --locked --offline` | 0 | trusted cache, no network |
| `cargo test --workspace --all-targets --locked --offline` | 0 | all targets, including the new pre-list cases |

Real-process CLI gates from the release binary: `bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha` → exit `0`, stdout `preflight=ok entity_id=entity-7 base=https://example.com/alpha`; `--entity-id Main` → exit `2`, stderr `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start`, no input echoed.

**Reproducibility** — two independent fresh-`CARGO_HOME` **and** fresh-`CARGO_TARGET_DIR` `x86_64-pc-windows-gnu` release builds of this commit under the unchanged tracked `.cargo/config.toml`:

| Build | Exit | Bytes | Raw SHA-256 |
| --- | --- | --- | --- |
| A | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |
| B | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |

`identical = true` between the two **new** runs, compared as raw bytes with no header normalisation. `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` were verified **absent** (not set to empty), so the tracked `-C link-arg=-Wl,--no-insert-timestamp` remains the effective flag source; no ephemeral linker flag was added. The hash coincides with the prior run's binary, which is expected and independently confirms that only test-only code changed.

**Read-only drift audit:** `Cargo.lock` `05a02666`, both manifests, `rust-toolchain.toml`, `.cargo/config.toml`, `mod.rs` `89dfea91`, `graph.rs` `e328dd6a`, `check_architecture.rs` `0bad0e02`, `workspace_graph.rs` `8fa4b4d0`, `cli_smoke.rs` `921cbbe9`, `main.rs` `6faa9fae`, README/AGENTS/About/Architecture/SOPs and ROADMAP/NEXT/OUTLOOK are all identical to the reviewed baseline.


## Gates (H-07)

| Gate | Evidence | Result |
| --- | --- | --- |
| **H-01 Authority and diff** | Exact HEAD/tree, 24 reviewed blobs with zero drift, one checkout, one Owner-installed ACTIVE difference committed unchanged, exactly the four permitted permanent paths touched, four WIP slots intact and ROADMAP/NEXT/OUTLOOK untouched | **PASS** |
| **H-02 Root pre-enumeration** | Direct real `rust_files_under`: redirected initial package root refused with `module-source-escape`, **`list` count 0**, empty inventory, link and target preserved; the same case fails on the pre-correction code with `list` count 1 | **PASS** |
| **H-03 Ancestors and queued dirs** | Redirected `crates` ancestor refused with **`list` count 0**; every dequeued directory validated before `list` (queued-directory case refused `unreadable-source` and never enumerated); non-`.rs` metadata-error coverage retained | **PASS** |
| **H-04 Ownership and existing positives** | `OwnedScratch` collision and redirected-ancestor tests pass; legal discovery and in-package source loading pass; foreign `.temp/rt` sentinel byte-identical across the run; zero task residue | **PASS** |
| **H-05 Checker and CLI regression** | Legal four-crate real check, 23 forbidden fixtures with `outcome=refused`/`category_match=true`, five legal twins, bypass self-test, graph negatives, CLI success and refusal all green; no reduced scanning surface | **PASS** |
| **H-06 Toolchain and reproducibility** | Pinned `1.99.1` GNU, fmt clean, strict Clippy zero warnings, 107 locked tests, offline test and release build, metadata/tree, two raw-byte-identical independent releases, absent flag overrides, unchanged lock and dependencies | **PASS** |
| **H-07 Report and publication** | One accurate report, diff confined to the allow-list, clean checkout, safe scratch disposition, one final push and exact remote readback | **PASS** |

## Limitations — stated, not inferred

- **Windows GNU only.** MSVC-hosted and Unix builds are `UNVERIFIED`/`NOT RUN`; the Unix symlink branch of the fixture helpers and the Unix redirect tests were not executed on this host.
- **Redirect types.** Evidenced with Windows directory junctions; file symlinks and mount-point reparse types were not exercised.
- **T-03 determinism.** A queued directory that becomes uninspectable is modelled by a deterministic injected seam failure rather than an unsafe live race, because creating a real queued-directory redirect mid-walk is not reproducible; the failure travels the production pre-list code path.
- **Time-of-check/time-of-use.** Pre-enumeration validation proves ownership at the moment of checking; no OS-supported locking evidence is offered and race freedom is **not** claimed.
- **`cargo-deny` / advisory scanning: `NOT RUN`.** The `unicode-ident` Unicode-3.0 `NOTICE` obligation remains outstanding for release packaging.
- **R01.C, CI, browser, HTTP, provider, Stage 02+, merge, release and deployment: `NOT RUN`.** Full Stage 01 acceptance is not claimed.

## Changed-path ledger (commit `c34e323c`, tree `addc3cb1`)

| Path | Change | Blob |
| --- | --- | --- |
| `crates/solo-cli/tests/architecture_checks/source.rs` | `validate_discovery_directory` pre-enumeration gate applied to every dequeued directory before `list`; no other policy change | `92f6e0e3` |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Direct-discovery regressions: recording seam with a `list` counter, redirected root and ancestor pre-list cases, queued-directory pre-list case, ownership/foreign-scratch preservation | `93fe5733` |
| `.documents/WIP/workorder-active.md` | Owner-installed `SOLO-WO-033 R01`, bytes unchanged, committed | `272ef3fc` |

## WIP state and scratch disposition

`.documents/WIP/` still holds exactly four files with no subdirectory; ROADMAP keeps `R01.B` and `R01.C` (no pruning), NEXT is its neutral heading, ACTIVE still holds the complete Owner-issued `SOLO-WO-033 R01` and was not cleared, OUTLOOK unchanged.

Task-owned scratch `.temp/w033` (pinned toolchain, two reproducibility caches and targets, logs, fixture subtrees) was created by this run and removed at the end; `.temp/architecture-fixtures` is the shared fixture parent, left in place and verified empty because it is not task-owned. Pre-existing foreign `.temp/rt` and `.temp/w032` sentinels were preserved untouched and are disclosed above with their cleanup trigger. No unknown or Owner material was deleted, no redirect target was used as a cleanup root, and no destructive Git command was issued.

**Empirical proof that the correction is load-bearing.** With only the new `validate_discovery_directory` call temporarily replaced by `Ok(())` and nothing else changed, the three pre-list regressions failed, and the root case reported `no directory may be enumerated once the root is refused: [".temp/architecture-fixtures/discovery-root-pre-list-…/checkout/crates/solo-application"]` with `left: 1, right: 0` — i.e. the old code enumerated the redirected root exactly once. The file was then restored and verified byte-identical (`92f6e0e3`) before committing.
