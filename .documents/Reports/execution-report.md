# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-08 |
| Work order | **`SOLO-WO-032 R01`** — Stage 01 `R01.B` root-anchoring, scratch-ownership and discovery correction |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-032 R01` |
| Trigger | Chief Architect independent **REVISE** of `SOLO-WO-031 R01` (three defects) |
| Verified root | Solo checkout, branch `main`, one worktree, sole `origin` = `provelopment/provelopment-foundation-solo` |
| Starting HEAD / tree | `c2ab315142f1b1f9cfe45018197d2cfaf934a60e` / `13000e5bdf8fec4c2cc8a4265045f2ba12e0835b` |
| Corrected source commit / tree | **`0ee1e5f1a8062623b47655040d8773db0b7c9db7`** / **`96be5b0e3a2fbcd295f18ed4e92ea2c3b224fc18`** |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host `x86_64-pc-windows-gnu` |
| **Status** | **`COMPLETE`** — the three findings are corrected and verified locally. Agent work status only, never Architect `ACCEPT`. |

## Preflight

One checkout on `main` at `c2ab315`, no stash, one worktree, sole `origin`. **All reviewed blobs matched with zero drift**: the report `5bbd0396`, prior ACTIVE `3722a688`, the three previously corrected files (`source.rs` `141261fd`, `mod.rs` `adf2f0be`, `check_architecture_fixtures.rs` `7d20147e`), `graph.rs` `e328dd6a`, `check_architecture.rs` `0bad0e02`, `workspace_graph.rs` `8fa4b4d0`, `cli_smoke.rs` `921cbbe9`, `main.rs` `6faa9fae`, both `Cargo.toml`s, `Cargo.lock` `05a02666`, `rust-toolchain.toml` `9ec55b3b`, `.cargo/config.toml` `74a4fb6c`, README, AGENTS, About, Architecture, both SOPs, and the ROADMAP/NEXT/OUTLOOK blobs. The only pre-existing tracked difference was the Owner's complete installation of this order (24,855 B, blob `2224738b`), preserved byte-for-byte and committed unchanged. WIP held exactly four files with no subdirectory; the agent edited neither ROADMAP, NEXT, OUTLOOK nor cleared ACTIVE.

The host had no Rust installation, so the **same pinned** `1.99.1` GNU variant was installed into git-ignored, task-owned `.temp/w032`; `rust-toolchain.toml` and `.cargo/config.toml` were not modified.


## Corrections (G-02, G-03, G-04)

### A. Package-root anchoring before any source use

A new `CheckoutAnchor` is derived from **locked Cargo metadata** (`workspace_root`) plus the real tree and validated before any package is inspected: the anchor's own original chain must be redirect-free, it must contain an inspectable `[workspace]` manifest and a `.git` entry, and every package root must be lexically exactly `crates/<package>` under it.

`FsReader::confine` now takes `(path, root, anchor)` and proves, in this order: lexical containment of the file in the package root and of that root in the anchor; then `inspect_original_chain` over the **unresolved** entries from the volume root through the anchor, the package root and the file — refusing a missing entry, a non-directory intermediate, or any symlink / `FILE_ATTRIBUTE_REPARSE_POINT` (`0x0400`) component; only then `canonicalize` of the file and the package root with an effective-containment comparison. Any failure is a violation (`module-source-escape` or `unreadable-source`), never a silent return. This replaces the defect where only components *below* the package root were inspected, so a self-consistent canonicalisation of a redirected root and its redirected child could look confined. A redirect into another directory **inside the same checkout** is refused exactly like one outside it.

The same gate covers target roots, library public surfaces, glob re-export readers, `include!` (including `.inc`), `#[path]`, default `mod`, and package discovery. The limitation is unchanged and explicit: this is per-use validation, not a proof against hostile TOCTOU races.

### B. Discovery fails closed

`rust_files_under` was rewritten against a `DiscoveryFs` seam (`list` / `kind` / `redirected`, real implementation `RealDiscoveryFs` using `symlink_metadata` only). For **every** entry, including a non-`.rs` name, the redirect status and the unresolved entry kind are established with explicit `Result` handling: an inspection error is `unreadable-source`; a redirect is `module-source-escape` and is never descended; an entry that is neither file nor directory is refused. `entry.metadata().is_ok_and(...)`, `path.is_dir()`, and the previous "interm inspection failure ⇒ treat as an ordinary non-directory entry" path are gone. A directory selected for traversal is checked with `inspect_original_chain` and against the anchor before descending, and the `target`/hidden-name exclusions are applied only **after** the entry's identity has been inspected safely.

### C. Exclusive, non-destructive fixture ownership

`Scratch::new`/`Drop` (which used `create_dir_all` through unverified ancestors and recursive `remove_dir_all`) is replaced by `OwnedScratch`:

| Concern | Behaviour |
| --- | --- |
| Shared `.temp` and `.temp/architecture-fixtures` | inspected as unresolved entries; created exclusively when absent, inspected and **never adopted** when present; a redirected or non-directory shared path is refused |
| Final fixture root | exclusive `create_dir` at an exact requested name (`create_exact` ⇒ `AlreadyExists` refused even if empty), or at a fresh unique name that retries instead of adopting |
| Nested dirs and files | `create_dir` one level at a time, each recorded; files written with `create_new` so nothing is overwritten; a link is never created over an existing entry |
| Teardown | links first with the link-only operation, then owned files, then owned directories **deepest-first** with non-recursive `remove_dir`; `remove_dir_all` is never called, no shared parent is removed, and any failure is reported rather than claimed as clean |


## Adversarial evidence (G-02, G-03, G-04)

Genuine **Windows directory junctions** (`cmd /C mklink /J`, no elevation, no global policy change) on exclusively owned fixture trees. Identities are repository-relative (`fixture/` = the owned fixture checkout).

| Test | Construct | Refusal | Preservation |
| --- | --- | --- | --- |
| `a_redirected_package_root_is_refused_before_its_source_is_used` | `fixture/crates/solo-application` is a junction to `fixture/real-target` (inside the same checkout) | `module-source-escape`; **no** `layer-outward-reference`, so the source was never consumed | target bytes and the junction preserved |
| `a_redirected_package_ancestor_is_refused` | `fixture/crates` is a junction to `fixture/real-crates`, so the package root looks like an ordinary directory after resolution | `module-source-escape`; source never consumed | target preserved |
| `a_redirected_include_is_refused_before_its_content_is_consumed` | `include!("alias/forbidden.inc")` where `alias` is a junction; the payload is a non-`.rs` fragment containing a forbidden `use` | `include-escape`; no `unparsable-source` and no `layer-outward-reference` | link and target preserved |
| `a_redirected_path_module_is_refused` / `a_redirected_default_module_is_refused` | `#[path = "alias/escape_mod.rs"]`, and default `mod escaped;`, behind a junction | `module-source-escape`; content never scanned | target preserved |
| `a_conditional_module_path_is_refused_even_with_a_valid_default` | `#[cfg_attr(windows, path = "alt_win.rs")] mod alternate;` with a genuine `alternate.rs` on disk | `conditional-module-path` | n/a |
| `a_missing_real_module_source_is_refused` | `mod missing;` | `module-source-unresolved` | n/a |
| `a_redirected_scratch_ancestor_is_refused_before_any_write` | synthetic `case/checkout/.temp` junction to `case/checkout/target` | `ScratchError::Redirected` **before** any create or write | target still empty; junction preserved |
| `an_existing_fixture_directory_is_never_adopted_or_deleted` | pre-existing fixture directory holding a sentinel file | `ScratchError::AlreadyExists` | sentinel byte-identical |
| `a_discovery_metadata_failure_on_a_non_source_entry_is_refused` | injected `redirected()` error for the non-`.rs` name `notes.txt`, through the production walker | `unreadable-source` | n/a |
| `a_redirected_discovery_directory_is_refused_and_preserved` | `vendor` junction inside a package | `module-source-escape`; the redirected file is never inventoried | target preserved |
| `the_anchor_requires_a_workspace_manifest_a_git_root_and_crates_layout` | misplaced package root, and an anchor without `.git` | refused; a well-formed checkout anchors | n/a |
| `legal_in_package_loading_still_succeeds`, `the_real_discovery_accepts_a_clean_package` | nonredirected root with `include!`, `mod`, harmless `cfg_attr` | accepted (no regression) | n/a |

Each of these fails against the previous implementation: the redirected root and ancestor cases would have produced `layer-outward-reference` (content consumed) instead of refusing; the metadata-failure case produced an empty inventory; the collision and redirected-ancestor cases were adopted or created through the link.

**Checker matrix unchanged and green:** 23 forbidden fixtures each report `outcome=refused` with the expected category and checker status `1`; the 5 legal twins are accepted; `--self-test` reports `self-check=passed cases=29`; `--disable` (and any unknown flag) is usage status `2`. Exit `3` remains the "forbidden fixture accepted" failure mode and is asserted structurally (`1 ≠ 3 ≠ 0`, `classify_fixture_result(&[]) == Accepted`), because producing it live would require a checker regression.


## Verification on the committed identity (G-05, G-06)

| Command | Exit | Observation |
| --- | --- | --- |
| `rustc --version` / `cargo --version` | 0 | `1.99.1 (58cf8f9c3)` / `1.99.1 (5f94df478)`, host `x86_64-pc-windows-gnu` |
| `cargo fmt --all -- --check` | 0 | no diff |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | 0 | **zero** warnings |
| `cargo test --workspace --all-targets --locked` | 0 | **11 binaries, 103 tests passed, 0 failed** |
| `cargo test --locked -p solo-cli --test check-architecture` | 0 | `architecture=legal packages=4 targets=12 files=19 edges=22` |
| `cargo test --locked -p solo-cli --test check-architecture -- --self-test` | 0 | `self-check=passed cases=29` |
| `-- --fixture cfg-attr-path` / `--fixture direct-outward` | 1 / 1 | `outcome=refused`, `category_match=true` |
| `-- --disable` | 2 | usage refusal; no weakening flag exists |
| `cargo build --workspace --release --locked` | 0 | release binary |
| `cargo metadata --format-version 1 --locked --all-features` | 0 | 66,227 B, 15 packages, 4 members |
| `cargo tree --workspace --all-features --locked` | 0 | 24 lines; `syn` only under `solo-cli` (dev) |
| `cargo build --workspace --release --locked --offline` | 0 | trusted cache, no network |
| `cargo test --workspace --all-targets --locked --offline` | 0 | incl. real-tree gate and redirect tests |

Real-process CLI gates re-run from the release binary: `bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha` → exit `0`, stdout `preflight=ok entity_id=entity-7 base=https://example.com/alpha`; `--entity-id Main` → exit `2`, stderr `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` (no input echo).

**Reproducibility** — two independent fresh-registry-cache **and** fresh-target-directory `x86_64-pc-windows-gnu` release builds of this committed identity, under the unchanged tracked `.cargo/config.toml`:

| Build | Exit | Bytes | Raw SHA-256 |
| --- | --- | --- | --- |
| A | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |
| B | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |

`identical = true`, compared as raw bytes with no header normalisation. **Flag preconditions verified, not assumed:** `RUSTFLAGS` is **absent** (not empty) and `CARGO_ENCODED_RUSTFLAGS` is absent, so the tracked `target.x86_64-pc-windows-gnu.rustflags = ["-C", "link-arg=-Wl,--no-insert-timestamp"]` is the effective flag source. No ephemeral linker flag was added.

**Unchanged-blob audit:** `Cargo.lock` `05a02666`, root `Cargo.toml` `14505be4`, `crates/solo-cli/Cargo.toml` `1ca5e114`, `rust-toolchain.toml` `9ec55b3b`, `.cargo/config.toml` `74a4fb6c`, `graph.rs` `e328dd6a`, `check_architecture.rs` `0bad0e02`, `workspace_graph.rs` `8fa4b4d0`, `cli_smoke.rs` `921cbbe9`, `main.rs` `6faa9fae`, and the ROADMAP/NEXT/OUTLOOK blobs are identical to the reviewed baseline. The release executable is also bit-identical to the previously evidenced binary, which independently confirms that no production source changed.


## Gates (G-07)

| Gate | Evidence | Result |
| --- | --- | --- |
| **G-01 Authority and scope** | Exact starting HEAD/tree, 23-blob preflight with zero drift, one checkout, Owner-issued ACTIVE committed unchanged, permanent ledger stayed at five paths (three source/test + ACTIVE + report), ROADMAP/NEXT/OUTLOOK untouched | **PASS** |
| **G-02 Package-root anchoring** | `CheckoutAnchor` from locked metadata + Git root + `crates/<package>` layout; unresolved entry chain from the volume root through anchor and package root inspected before use; genuine redirected root and ancestor **inside the same checkout** refused with target bytes preserved | **PASS** |
| **G-03 Discovery fail-closed** | Every entry, including non-`.rs`, has unresolved kind and redirect status established with explicit errors; injected metadata failure produces `unreadable-source`; a redirected directory is refused before descent and never inventoried; a clean package is still accepted | **PASS** |
| **G-04 Scratch ownership and cleanup** | Unresolved `.temp` and fixture parent validated (created exclusively or inspected, never adopted); exact-name collision refused with the sentinel intact; redirected synthetic `.temp` refused before any write; teardown links → files → empty dirs, deepest-first, no `remove_dir_all`; zero residue | **PASS** |
| **G-05 Checker regression** | All 4 packages / 12 targets / 19 files checked; 23 negatives + 5 legal twins green; conditional, include, `#[path]`, default-module and missing-source refusals intact; exit mapping and CLI/graph suites green | **PASS** |
| **G-06 Pinned toolchain and reproducibility** | fmt, strict Clippy (zero warnings), 103 locked tests, release/metadata/tree/offline PASS, raw byte-identical independent builds, flag-override absence evidenced, lock and manifests unchanged | **PASS** |
| **G-07 Factual report and publication** | One canonical report, exact changed paths and results, task scratch cleared (pre-existing foreign item disclosed), no secrets, clean local tree, one final push and remote verification | **PASS** |

## Limitations — stated, not inferred

- **Windows GNU only.** MSVC-hosted and Unix builds remain `UNVERIFIED`/`NOT RUN`; the Unix symlink branch of the fixture helpers was not executed.
- **Redirect type.** Evidenced with Windows directory junctions; file symlinks and mount-point reparse types were not exercised.
- **Time-of-check/time-of-use.** Anchoring and confinement prove ownership of the destination at the moment it is checked; no OS-supported locking evidence is offered and race freedom is **not** claimed.
- **`cargo-deny` / advisory scanning: `NOT RUN`.** The `unicode-ident` Unicode-3.0 `NOTICE` obligation remains outstanding for release packaging.
- **Exit `3`** is asserted structurally rather than observed live, by definition of it being the failure mode.
- **`R01.C`, CI, browser, HTTP, provider, Stage 02+, release and deployment: `NOT RUN`.** Full Stage 01 acceptance is **not** claimed; `R01.B` remains in ROADMAP and ACTIVE is not cleared.

## Changed-path ledger (commit `0ee1e5f1`, tree `96be5b0e`)

| Path | Change | Blob |
| --- | --- | --- |
| `crates/solo-cli/tests/architecture_checks/source.rs` | `CheckoutAnchor` and original-chain anchoring in `SourceReader::confine`; anchored propagation through target roots, public surfaces, glob readers, `include!`, `#[path]`, default modules and discovery; fail-closed `DiscoveryFs` walker | `8e42f700` |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | Passes the metadata-derived `workspace_root` anchor into `check_packages`; fails closed when metadata reports none | `89dfea91` |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | `OwnedScratch` exclusive ownership and non-recursive teardown; redirected package-root, ancestor, discovery and scratch-ancestor regressions; collision preservation; injected metadata-failure case | `714fc59b` |
| `.documents/WIP/workorder-active.md` | Owner-installed order, bytes unchanged, committed | `2224738b` |

No production Rust file, manifest, `Cargo.lock`, `graph.rs`, `check_architecture.rs`, workspace configuration, toolchain file, governing document or other repository was modified.

## WIP state and scratch

`.documents/WIP/` still holds exactly four files with no subdirectory. ROADMAP retains `R01.B` and `R01.C` (no pruning); NEXT is the neutral heading; ACTIVE still holds the complete Owner-issued `SOLO-WO-032 R01` and was not cleared; OUTLOOK is unchanged.

Owned scratch inventory and outcome: `.temp/w032` (pinned toolchain, two reproducibility caches/targets, logs, fixture subtrees) was created by this run and removed at the end. `.temp/architecture-fixtures` is the shared fixture parent, left in place and verified **empty** (0 entries) because it is shared and not task-owned. `.temp/rt` is the pre-existing foreign item described above, deliberately preserved and disclosed with its cleanup trigger. No unknown or Owner material was deleted, no redirect target was ever used as a cleanup root, and no destructive Git command was issued.

**Pre-existing foreign scratch, disclosed and preserved.** `.temp/rt/tmp/system-commandline-sentinel-files/` contains a 501-byte `dotnet-suggest-registration-git-credential-manager, Version=2.9.1.0, …` file written by a dotnet/credential-manager service during the previous run, which pointed `TEMP` there. It is not repository content, it is not task-owned this run, and it was **left untouched**; this run used an isolated `TEMP` under `.temp/w032/tmp`. Cleanup trigger: the Owner may delete `.temp/rt` once no service holds that file.
