# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | **`SOLO-WO-031 R01`** — Stage 01 `R01.B` source-loading fail-closed correction |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-031 R01` |
| Trigger | Chief Architect independent **REVISE** of `SOLO-WO-030 R01` (three defects) |
| Verified root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, branch `main`, one worktree, sole `origin` = `provelopment/provelopment-foundation-solo` |
| Starting HEAD / tree | `4abb80ea43cbd2c30c04d6200a2f24fd87af668f` / `2ad346afb2a0ed0206e233a33c9c0e8ed7ce1cfe` |
| Corrected source commit / tree | **`da96c5142b801fdf18e585240cb6f8723f811b1c`** / **`4a5e03c9a036ef9b7013b1addf3f7c4e0b97f51f`** |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host `x86_64-pc-windows-gnu` |
| **Status** | **`COMPLETE`** — the three findings are corrected and verified locally. Agent work status only; never Architect `ACCEPT`. |

## Preflight

One checkout on `main` at `4abb80ea`, no stash, one worktree, sole `origin`. **23 reviewed blobs matched with zero drift**, including the four checked files (`source.rs` `9efc69c5`, `mod.rs` `a5119190`, `check_architecture.rs` `39c77a59`, `check_architecture_fixtures.rs` `6a5e540c`), `graph.rs` `e328dd6a`, `workspace_graph.rs` `8fa4b4d0`, `Cargo.lock` `05a02666`, both manifests, toolchain/config, README, AGENTS, About, Architecture, both SOPs and all four WIP files. The only pre-existing tracked change was the Owner's complete installation of this `SOLO-WO-031 R01` (21,583 B, blob `3722a688…`), preserved byte-for-byte and carried into the implementation commit. WIP held exactly four files, no subdirectory; NEXT was the neutral heading; ROADMAP kept `R01.B`+`R01.C` without `R01.A`; OUTLOOK unchanged — none edited.

The host had no Rust installation, so the **same pinned** `1.99.1` GNU host variant was installed into git-ignored, task-owned `.temp/rt`; `rust-toolchain.toml` and `.cargo/config.toml` were not modified.

## What was corrected (F-02, F-03, F-04)

### 1. One confinement gate for every source the checker may open

`SourceReader` gained `confine(path, root) -> Result<PathBuf, ConfinementError>`, and it is now the **only** way a candidate source becomes inspectable. `FsReader::confine` proves, in order: lexical containment in the package root; then a walk of the **real directory entries** from the root to the file, refusing any entry whose `symlink_metadata` is a symlink or carries `FILE_ATTRIBUTE_REPARSE_POINT`; then `canonicalize` of both the file **and** the package root with an effective-containment comparison. Every failure is a violation — `Escape` or `Unresolvable` — never a silent skip. `MapReader::confine` stays lexical so in-memory fixtures never consult the host filesystem.

Applied to every path that can reach source: the crate root and every recursively discovered module (`scan`), literal `include!` (including non-`.rs` fragments, now through the same gate), `#[path]` modules, default `mod` resolution, target roots, the public-surface library reader, glob re-export readers, and package discovery. `rust_files_under` now fails closed too: an unreadable directory, an unreadable entry, or any link/reparse entry is reported and **never descended**, so a junction can neither hide nor substitute source. The previous `let Ok(..) = canonicalize(..) else { return; }` shape is gone.

**Consequence by construction:** a refused path is refused *before* its content is read, parsed or scanned. `scan` confines before `read`; `confined_source` confines before returning a path for traversal; discovery never crosses a redirect.

### 2. Conditional and malformed module path selection

`path_attribute` (a first-match literal lookup that silently ignored everything else) was replaced by `module_path_selection`, which returns `Default`, exactly one `Literal`, or a typed refusal:

| Attribute situation | Outcome |
| --- | --- |
| No `#[path]` | default `name.rs` / `name/mod.rs` resolution |
| Exactly one literal `#[path = "…"]` | resolved through the confinement gate |
| `#[cfg_attr(…, path = …)]` (any predicate) | **refused** `conditional-module-path` |
| nested `#[cfg_attr(cfg, cfg_attr(cfg, path = …))]` | **refused** `conditional-module-path` |
| `cfg_attr` whose arguments cannot be parsed | **refused** `conditional-module-path` |
| two or more `#[path]` attributes | **refused** `module-source-ambiguous` |
| non-literal / malformed `#[path]` value | **refused** `module-source-unresolved` |
| path-selecting attribute on an **inline** module | **refused** `conditional-module-path` |

A `cfg_attr` that cannot select source (`#[cfg_attr(test, allow(dead_code))]`) remains legal — verified by the `legal-cfg-attr-harmless` twin. So a valid default `alternate.rs` can no longer mask an uninspected alternative: the module is refused, not silently resolved to the default.

### 3. Fixture result semantics

The process contract is now explicit: `0` legal, `1` correctly refused (or illegal real tree), `2` usage, **`3` forbidden fixture accepted / harness failure**. `--fixture` prints a stable line `… outcome=refused|accepted … category_match=true|false`, and the exit status follows the outcome through `classify_fixture_result` and `fixture_exit_code`. An accepted forbidden fixture can therefore never be quoted as evidence of a refusal.

## Adversarial evidence (F-04)

Real filesystem tests live in `check_architecture_fixtures.rs` (run by ordinary
`cargo test --workspace --all-targets --locked`) and exercise the **same** `source::check_unit` /
`FsReader` path the real tree uses. The redirect type actually created is a **Windows directory
junction** (`cmd /C mklink /J`, no elevation, no global policy change); a Unix directory-symlink
branch exists but is `NOT RUN`.

| Test | Construct | Result |
| --- | --- | --- |
| `a_redirected_include_is_refused_before_its_content_is_consumed` | in-package `include!("alias/forbidden.inc")`, `alias` = junction to a sibling directory; payload is a **non-`.rs`** fragment containing a forbidden `use` | refused `include-escape`; **no** `unparsable-source` and **no** `layer-outward-reference`, proving the payload was never read or scanned |
| `a_redirected_path_module_is_refused` | `#[path = "alias/escape_mod.rs"] mod escaped;` through the same junction | refused `module-source-escape`; redirected content not scanned |
| `a_redirected_default_module_is_refused` | `mod escaped;` whose `escaped/mod.rs` sits behind the junction | refused `module-source-escape`; redirected content not scanned |
| `a_conditional_module_path_is_refused_even_with_a_valid_default` | `#[cfg_attr(windows, path = "alt_win.rs")] mod alternate;` **with a genuine** `alternate.rs` present | refused `conditional-module-path`; the test asserts the default exists, so passing merely because it exists cannot happen |
| `a_missing_real_module_source_is_refused` | `mod missing;` on the real filesystem | refused `module-source-unresolved` |
| `legal_in_package_loading_still_succeeds` | legal `include!("fragment.inc")` + `mod inner;` | accepted (no regression) |

**Lexical versus effective proof.** The redirected `include!` test asserts, before running the
checker, that the literal path `package/src/alias/forbidden.inc` **is lexically inside** the package
root **and** that `canonicalize` of that same literal **escapes** it — the exact condition the old
code could not see. **Target preservation:** each redirect test snapshots the redirected target's
files before the run and asserts byte-identical content afterwards, that the redirect itself still
exists, and that no foreign file appeared. **Cleanup:** each test owns an exclusive scratch root
under `.temp/architecture-fixtures/`; its `Drop` removes the **links first** (so a recursive delete
can never traverse a redirect into its target), then its own directory only. Scratch residue after
the suite: **0 entries**.

Additional in-memory negatives added to the fixture registry (brought to `--self-test` and to
`cargo test`): `cfg-attr-path`, `cfg-attr-nested`, `path-duplicate`, `path-nonliteral`; plus the
legal twin `legal-cfg-attr-harmless`. The 19 earlier classes are unchanged and preserved.

## Fixture matrix — outcomes and process exits on committed `da96c514`

All 23 forbidden fixtures report `outcome=refused` with `category_match=true`. Cargo propagates the
harness=false binary's own status here — confirmed by `--disable` returning **2** (a usage status
Cargo never synthesises) rather than libtest's `101`.

| Fixture | Category | Exit |
| --- | --- | --- |
| `direct-outward`, `renamed-alias`, `reexport-chain`, `qualified-path`, `macro-argument`, `checker-bypass` | `layer-outward-reference` | 1 |
| `private-internal` | `private-crate-internal` | 1 |
| `undeclared-crate` | `undeclared-crate` | 1 |
| `path-attribute-escape` | `module-source-escape` | 1 |
| `include-escape` | `include-escape` | 1 |
| `missing-module`, `path-nonliteral` | `module-source-unresolved` | 1 |
| `path-duplicate` | `module-source-ambiguous` | 1 |
| `cfg-attr-path`, `cfg-attr-nested` | `conditional-module-path` | 1 |
| `macro-generated` | `macro-unsupported` | 1 |
| `dynamic-load` | `dynamic-loading` | 1 |
| `unsafe-block` | `unsafe-code` | 1 |
| `unparsable-source` | `unparsable-source` | 1 |
| `syn-outside-cli`, `dev-in-runtime` | `unapproved-external-crate` | 1 |
| `graph-illegal` | `graph-illegal-edge` | 1 |
| `graph-unapproved-dev` | `graph-unapproved-dependency` | 1 |
| `--disable` (any unknown flag) | usage | 2 |

**Exit 3 is not observed here by design.** It is the failure mode "a forbidden fixture was
accepted", so producing it would require a checker regression. Its mapping and text are asserted
directly: `fixture_outcomes_and_exit_codes_are_distinguishable` checks `1 ≠ 3`, `3 ≠ 0`,
`classify_fixture_result(&[]) == Accepted`, `fixture_exit_code(Accepted) == 3`, and that the outcome
text changes to `outcome=accepted … category_match=false`. The self-test remains load-bearing: any
short-circuit makes a negative fixture be accepted, which now reports `FAIL` **and** would exit 3.


## Verification on the committed identity (F-05)

| Command | Exit | Observation |
| --- | --- | --- |
| `rustc --version` / `cargo --version` | 0 | `1.99.1 (58cf8f9c3)` / `1.99.1 (5f94df478)`, host `x86_64-pc-windows-gnu` |
| `cargo fmt --all -- --check` | 0 | no diff |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | 0 | **zero** warnings, zero errors |
| `cargo test --workspace --all-targets --locked` | 0 | **11 binaries, 95 tests passed, 0 failed** |
| `cargo test --locked -p solo-cli --test check-architecture` | 0 | `architecture=legal packages=4 targets=12 files=19 edges=22` |
| `cargo test --locked -p solo-cli --test check-architecture -- --self-test` | 0 | `self-check=passed cases=29` (real tree + 23 refusals + 5 legal twins) |
| `-- --fixture cfg-attr-path` | 1 | `outcome=refused … conditional-module-path … category_match=true` |
| `-- --fixture include-escape` | 1 | `outcome=refused … include-escape … category_match=true` |
| `-- --disable` | 2 | usage refusal; no weakening flag exists |
| `cargo build --workspace --release --locked` | 0 | release binary produced |
| `cargo metadata --format-version 1 --locked --all-features` | 0 | 66,061 B, 15 packages, 4 members |
| `cargo tree --workspace --all-features --locked` | 0 | 21 lines; `syn` only under `solo-cli` (dev) |
| `cargo build --workspace --release --locked --offline` | 0 | trusted cache, no network |
| `cargo test --workspace --all-targets --locked --offline` | 0 | incl. the real-tree gate and the redirect tests |

Existing real-process gates re-run: `workspace_graph` (16 tests), `cli_smoke` (15) and
`bootstrap_preflight` (9) all pass inside the 95. The release CLI keeps its behaviour:
`bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha` → exit `0`,
stdout `preflight=ok entity_id=entity-7 base=https://example.com/alpha`; `--entity-id Main` → exit
`2`, stderr `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` (no input echo).

**Dependency and lock evidence:** `Cargo.lock` blob is still `05a02666…` and
`crates/solo-cli/tests/architecture_checks/graph.rs` is still `e328dd6a…`; `syn =3.0.9` remains a
`solo-cli` **dev**-only edge and the runtime closure is the four workspace crates only. No package,
lock, manifest or dependency changed in this run.

**Reproducibility (raw SHA-256, no header normalisation).** Two **independent** fresh-registry-cache
and fresh-target-directory release builds with the tracked `.cargo/config.toml`:

| Build | Exit | Bytes | SHA-256 |
| --- | --- | --- | --- |
| A (fresh `CARGO_HOME` + fresh target) | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |
| B (fresh `CARGO_HOME` + fresh target) | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |

`identical = true`. The executable is also bit-identical to the previously evidenced release binary,
which independently confirms that **no production source changed** in this correction.

**RUSTFLAGS finding (worth recording).** Cargo takes the *first* extra-flag source it finds, so an
**empty** `RUSTFLAGS` in the environment silently shadows `target.x86_64-pc-windows-gnu.rustflags`
from the tracked config. During this run that briefly produced a different binary (`44CD604D…`) with
an injected PE timestamp. The correct precondition is `RUSTFLAGS` **absent**, not empty; with it
absent the hash returns to `363AC93C…` and both fresh builds match. A reproducibility procedure must
therefore assert absence, not emptiness.


## Gates

| Gate | Evidence | Result |
| --- | --- | --- |
| **F-01 Authority and diff** | Exact baseline HEAD/tree, 23/23 blobs, one checkout, only the Owner-installed ACTIVE difference before work, exactly the six listed permanent paths (four source/test + ACTIVE), ROADMAP/NEXT/OUTLOOK and all read-only blobs verified unchanged | **PASS** |
| **F-02 Effective filesystem confinement** | One `confine` gate for target roots, `include!` (incl. non-`.rs`), `#[path]`, default modules, public-surface/glob readers and discovery; genuine junction escapes refused for include, `#[path]` and default `mod`; canonicalisation failure refuses rather than skips | **PASS** |
| **F-03 Conditional path selection** | `cfg_attr(path)` refused with a valid default present; nested, duplicate, non-literal and inline cases refused; harmless `cfg_attr` and `#[cfg(test)]` remain legal | **PASS** |
| **F-04 Real negatives and fixture truth** | Real junction tests, target-preservation assertions, missing-source refusal, all 23 fixtures with expected category and `outcome=refused`, 5 legal twins accepted, refusal/acceptance statuses distinct, self-test load-bearing | **PASS** |
| **F-05 Regression/reproducibility** | fmt, strict Clippy, 95 locked tests, `--offline` tests and build, graph guard, CLI exits/output, byte-identical independent GNU executables | **PASS** |
| **F-06 Security/cleanup/report** | No `unsafe`, no hidden fallback, no new runtime dependency, repository-relative diagnostics only, scratch cleaned, one canonical report, clean tree, one final push | **PASS** |

## Limitations — stated, not inferred

- **Windows GNU only.** MSVC-hosted and Unix builds remain `UNVERIFIED`/`NOT RUN`; the Unix symlink branch of the redirect tests was not executed.
- **Redirect type.** Evidenced with Windows directory junctions; file symlinks and mount-point reparse types were not exercised.
- **Time-of-check/time-of-use.** Confinement proves the destination at the moment it is checked; it does **not** claim to eliminate hostile TOCTOU races, and no platform-supported locking evidence is offered.
- **`cargo-deny` / advisories: `NOT RUN`** (tool unavailable); no vulnerability claim is made. The `unicode-ident` Unicode-3.0 `NOTICE` obligation remains outstanding for release packaging.
- **Exit 3 not observed** as a live process status (it is by definition the failure mode); its mapping and text are asserted directly instead.
- **`R01.C`, CI, browser, HTTP, Stage 02+, release and deployment: `NOT RUN`.** The full Stage 01 exit gate is **not** claimed; `R01.B` stays in ROADMAP until independent Architect acceptance and separate Owner closure.

## Changed-path ledger (commit `da96c514`, tree `4a5e03c9`)

| Path | Change |
| --- | --- |
| `crates/solo-cli/tests/architecture_checks/source.rs` | `SourceReader::confine` and real effective-path enforcement wherever source is opened or discovered; `module_path_selection` replacing `path_attribute`; redirect-aware discovery; `conditional-module-path` refusals |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | `ConditionalModulePath` category; outcome/exit-code contract; 4 new fixtures; 1 new legal twin |
| `crates/solo-cli/tests/check_architecture.rs` | Distinct statuses and a stable outcome line for `--fixture` |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Genuine junction escape tests, conditional-path test, target-preservation and outcome-semantics assertions |
| `.documents/WIP/workorder-active.md` | Owner-installed order, bytes unchanged, committed |

No production Rust file, manifest, `Cargo.lock`, `graph.rs`, workspace configuration, toolchain file, governing document or other repository was modified.

## WIP state and scratch cleanup

`.documents/WIP/` still holds exactly four files with no subdirectory. ROADMAP still contains `R01.B`
and `R01.C` (no pruning — closure is not the agent's decision); NEXT is the neutral heading; ACTIVE
still holds the complete Owner-issued `SOLO-WO-031 R01` (not cleared by the agent); OUTLOOK unchanged.
All task-owned scratch — the pinned toolchain, two reproducibility caches/targets, logs and the
`.temp/architecture-fixtures` test roots — lived under git-ignored `.temp/` and was removed at run
end. No unknown, Owner-owned or foreign material was deleted, and no redirected target directory was
ever used as a cleanup root.

