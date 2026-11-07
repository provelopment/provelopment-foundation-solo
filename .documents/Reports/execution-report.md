# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | **`SOLO-WO-030 R01`** — Stage 01 `R01.B` Hexagonal source-boundary enforcement |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-030 R01` |
| Verified root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, branch `main`, one worktree, sole `origin` = `provelopment/provelopment-foundation-solo` |
| Starting HEAD / tree | `88333c4030dc906e98b9c52c784b799ff2c51928` / `4cdd1755a4c1a52310abef9d93e312b992aa6144` |
| Implementation source commit / tree | **`844f878c2e0214267d64629aa3f9eae42f975eb1`** / **`3369e1fc077cd47df4acbefa68ee864affd439fb`** |
| Toolchain | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host `x86_64-pc-windows-gnu` |
| **Status** | **`COMPLETE`** — `R01.B` implemented and verified locally. Agent work status only; never Architect `ACCEPT`. |

## Preflight

One checkout on `main` at the reviewed HEAD, no stash, no second worktree, sole `origin`. All eighteen reviewed blobs matched with **zero drift** (`README.md`, `AGENTS.md`, About, Architecture, both SOPs, NEXT, ROADMAP, OUTLOOK, report, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `crates/solo-cli/Cargo.toml`, `tests/workspace_graph.rs`, `src/main.rs`, `tests/cli_smoke.rs`); committed ACTIVE was the neutral blob `ea5f20b7…`. The only pre-existing tracked change was the Owner's complete installation of `SOLO-WO-030 R01` (26,212 B, blob `da65750c…`), which was left byte-unchanged and carried into the implementation commit. WIP held exactly four files, no subdirectories; NEXT held the single accepted `R01.B` candidate; ROADMAP held `R01.B`/`R01.C` but not `R01.A`; OUTLOOK unchanged.

**Issued-candidate transition:** `plan-next.md` replaced by the neutral heading only — 49 B, UTF-8, LF, no BOM, blob `92fa562b…`. ROADMAP, OUTLOOK and the installed ACTIVE content were not edited.

## Environment facts (disclosed, not inferred)

The host had **no Rust toolchain and no cargo registry cache**. A pinned `rustc 1.99.1` was installed into task-owned scratch (`.temp/rt`, git-ignored) via `rustup`, and the **GNU host variant** was selected (`RUSTUP_TOOLCHAIN=1.99.1-x86_64-pc-windows-gnu`) because the tracked `rust-toolchain.toml` pins the host-agnostic channel `1.99.1` and this host has no MSVC linker. `rust-toolchain.toml` was **not** modified. `.gitignore` does not list `target/` and may not be edited, so `CARGO_TARGET_DIR` was confined to `.temp/rt/target`; every build artifact, cache and log stayed inside git-ignored `.temp/`. The MSVC toolchain the installer defaulted to was not used for any recorded result.

## Dependency provenance (B-02)

| Item | Evidence |
| --- | --- |
| Direct dev-dependency added | `syn = { version = "=3.0.9", features = ["full", "visit"] }` in `crates/solo-cli` **dev-dependencies only** |
| Lock change | `git diff Cargo.lock` = **exactly one line** (`"syn"` appended to `solo-cli`'s dependency list); blob `d64704da…` → `05a02666…`; no version of any other package changed |
| Locked package count | 15 (4 workspace + 11 external); resolves cleanly under `--locked` |
| `syn` 3.0.9 provenance | registry `crates.io`, licence `MIT OR Apache-2.0`, declared `rust-version 1.71` — compatible with the pinned 1.99.1 |
| New transitive crates | none: `proc-macro2 1.0.107`, `quote 1.0.47`, `unicode-ident 1.0.27` were already locked |
| `serde_json` | unchanged dev-only `=1.0.154` |
| Runtime closure | `cargo tree -p solo-cli --edges normal` = the four workspace crates **only**; no external crate at all |
| Shipped binary | the release executable contains no `syn`/`serde_json` marker text and is **byte-identical** to the R01.A release binary (`363AC93C…`), confirming nothing new was linked |

**Licence/attribution obligation carried forward:** `unicode-ident` is `(MIT OR Apache-2.0) AND Unicode-3.0`; the Unicode-3.0 `NOTICE` obligation remains outstanding for the release package, as recorded in the earlier supply-chain review. `cargo-deny` is not installed on this host, so **advisory scanning is `NOT RUN`** and no vulnerability claim is made.

## Checker: entry point and inspected surface (B-03)

`[[test]] name = "check-architecture", path = "tests/check_architecture.rs", harness = false` — a test-only process, not a second binary and not a workspace package. Modes: **no arguments** inspects the real checkout and exits `0` only when graph *and* source are legal; `--self-test` runs the whole matrix; `--fixture <id>` runs one forbidden fixture. Any other argument (`--disable`, `--relax`, …) is a usage error with exit `2`; there is **no** disable switch, environment override or partial-scan success path.

Default real-tree result on the committed identity:

```text
architecture=legal packages=4 targets=12 files=19 edges=22
```

Enumeration comes from `cargo metadata --locked --all-features` (never filenames): 4 packages → **12 declared targets** → **19 Rust files** parsed, including inline `#[cfg(test)]` modules, out-of-line modules, integration-test roots and the checker's own three files. A file no declared target reaches is refused as `hidden-source`, so an unchecked file cannot smuggle an edge. Package roots are hard boundaries: `#[path]`, `include!` and `mod` resolution are normalised lexically *and* canonicalised, so a symlink/junction escape is refused.

**What it inspects:** `use`/`pub use` trees including globs and `as` renames, `extern crate … as …`, every `syn::Path` in types, expressions, patterns, attributes and `#[derive(...)]` lists, fully qualified `::crate::…` paths, `crate`/`self`/`super` routes, cross-crate privacy against each crate's computed public surface, out-of-line and inline modules, `#[path]` literals, literal `include!` content (parsed and scanned, not skipped), audited macro arguments, `unsafe` blocks/functions/impls/traits, `extern` blocks, `libloading`/`dlopen`-class roots and `syn::Item::Verbatim`.

**How aliases are defeated:** `use`/`extern crate` aliases are recorded as *full* paths and substituted before classification, so `use forbidden::Thing as helper; helper::new()` resolves to `forbidden::Thing::new`, and a type imported by name keeps its owning crate so `Type::method()` is attributed correctly. Bare single identifiers are classified only when they could name a dependency root — which is what catches `use libloading;`.

**Audited harmless macro allowlist (documented):** `assert`, `assert_eq`, `assert_ne`, `concat`, `debug_assert`, `debug_assert_eq`, `debug_assert_ne`, `eprint`, `eprintln`, `env`, `format`, `format_args`, `matches`, `option_env`, `panic`, `print`, `println`, `stringify`, `vec`, `write`, `writeln`. Their arguments are re-parsed into real AST nodes and every path inside is classified like ordinary code.

**Conservatively refused rather than exempted:** locally defined `macro_rules!` and any unknown or qualified macro invocation (token streams cannot be expanded soundly here); `include_str!`/`include_bytes!`; non-literal `include!`; unresolvable or ambiguous module locations; unparsable source; verbatim items; `extern` blocks; any path root that resolves to no declared dependency or local module. **Limit, stated plainly:** this is AST/token analysis, not compiler macro expansion or name resolution; anything not inspectable is forbidden.


## Negative, bypass and regression evidence (B-04)

Every fixture runs through the **same** routines as the real tree, using an in-memory source map and a deterministic fixture world — no second checkout and no mutation of product source. All nineteen exit **nonzero**:

| Fixture | Required class | Exit | Bounded, repository-relative diagnostic |
| --- | --- | --- | --- |
| `direct-outward` | direct outward dependency | 1 | `layer-outward-reference: fixture/src/lib.rs: solo-application may not depend on solo-adapters` |
| `renamed-alias` | renamed crate / alias | 1 | `layer-outward-reference` via `extern crate solo_adapters as upstream` |
| `reexport-chain` | re-export chain | 1 | `layer-outward-reference` from a `pub use` inside a nested module |
| `qualified-path` | qualified / reference path | 1 | `layer-outward-reference` on `::solo_adapters::CliOutcome` |
| `private-internal` | `crate`/`self`/`super`, private sibling | 1 | `private-crate-internal: solo_adapters::bootstrap_cli::parse_request` |
| `undeclared-crate` | undeclared workspace crate | 1 | `undeclared-crate: solo_adapters is not a declared dependency` |
| `path-attribute-escape` | module loading via `#[path]` | 1 | `module-source-escape: module stolen resolves outside its package root` |
| `include-escape` | module loading via `include!` | 1 | `include-escape: include! resolves outside the package root` |
| `missing-module` | unresolved module source | 1 | `module-source-unresolved: module absent has no resolvable source file` |
| `macro-generated` | macro-generated edge | 1 | `macro-unsupported: macro hide cannot be soundly inspected` (2 findings) |
| `macro-argument` | path hidden in macro arguments | 1 | `layer-outward-reference` on `solo_adapters::CliOutcome::run` |
| `dynamic-load` | dynamic / foreign load | 1 | `dynamic-loading: libloading::Library` |
| `unsafe-block` | unsafe escape hatch | 1 | `unsafe-code: unsafe code is forbidden in this workspace` |
| `unparsable-source` | unsupported / invalid input | 1 | `unparsable-source: source file is not parsable Rust` |
| `syn-outside-cli` | approved tool outside `solo-cli` | 1 | `unapproved-external-crate: syn is a development-only dependency` |
| `dev-in-runtime` | dev dependency in a shipped target | 1 | `unapproved-external-crate: serde_json is a development-only dependency` |
| `graph-illegal` | illegal / renamed graph edge | 1 | `graph-illegal-edge: graph:solo-domain->solo-adapters (aliased as inward)` |
| `graph-unapproved-dev` | illegal dev / transitive graph | 1 | `graph-unapproved-dependency: graph:solo-application->serde_json` (3 findings) |
| `checker-bypass` | checker bypass / short-circuit | 1 | `layer-outward-reference` on `solo_adapters::CliOutcome::run` |

**Disabled-guard proof.** `checker-bypass` hides the forbidden reference behind every short-circuit a lazy checker might take — a `#[cfg(test)]` inline module, a function body, a closure and a macro argument. If the guard is deleted, short-circuited, made permissive, or skips any one of those branches, that fixture is accepted and `--self-test` reports `FAIL` instead of passing. The matrix also pairs the refusals with four **legal twins** (`legal-application`, `legal-cli-test-dev-deps`, `legal-macro-usage`, `legal-inline-module`): dropping the rules makes the negatives pass, and over-refusing makes the twins fail. `--self-test` → `self-check=passed cases=24` (real tree + 19 refusals + 4 legal twins).

**Positive regressions preserved.** The earlier graph suite still refuses a domain→adapter edge, an application→CLI edge, an application→adapter edge, a domain→external edge, a renamed illegal edge, illegal dev and build edges, cycles, and now also the approved check tool declared as a *normal* (shipped) dependency or taken by an inner crate. `the_guard_is_load_bearing_not_decorative` shows the same graph is accepted, refused only by the added edge, and accepted again once it is removed. The single rule set (`graph::edge_allowed`) is shared by the Cargo graph guard, the source checker and the real-graph tests, so the layers cannot disagree.


## Verification results (B-05) — all on committed `844f878c`

| Command | Exit | Note |
| --- | --- | --- |
| `rustc --version` / `cargo --version` | 0 | `1.99.1 (58cf8f9c3)` / `1.99.1 (5f94df478)`, host `x86_64-pc-windows-gnu` |
| `cargo fmt --all -- --check` | 0 | no diff |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | 0 | **zero** warnings, zero errors |
| `cargo test --workspace --all-targets --locked` | 0 | **11 test binaries, 87 tests passed, 0 failed** |
| `cargo test --locked -p solo-cli --test check-architecture` | 0 | `architecture=legal packages=4 targets=12 files=19 edges=22` |
| `cargo test --locked -p solo-cli --test check-architecture -- --self-test` | 0 | `self-check=passed cases=24` |
| each `-- --fixture <id>` (19 runs) | **1** each | intentionally nonzero; see table above |
| `cargo build --workspace --release --locked` | 0 | release binary produced |
| `cargo metadata --format-version 1 --locked --all-features` | 0 | 66,061 B JSON, 15 packages, 4 members, 15 resolve nodes |
| `cargo tree --workspace --all-features --locked` | 0 | 21 lines; `syn` only under `solo-cli` (dev) |
| `cargo build --workspace --release --locked --offline` | 0 | trusted caches populated, no network |
| `cargo test --workspace --all-targets --locked --offline` | 0 | incl. the real-tree architecture gate |

**Lock-drift refusal (honest limits):** `--locked` is used for every command, so a manifest/lock divergence fails rather than silently re-resolving; that property was exercised implicitly (the first `cargo metadata` without `--locked` was the only resolving call, and every subsequent gate ran `--locked`). **Not exercised:** a *deliberate* lock-drift injection — doing so would require editing the lockfile outside the authorized change, so it is `NOT RUN` rather than claimed.

**Ordinary CLI behaviour unchanged** (release binary, exact output):

```text
solo-cli bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha
  exit 0, stdout: preflight=ok entity_id=entity-7 base=https://example.com/alpha

solo-cli bootstrap-preflight --entity-id Main --base-url https://example.com
  exit 2, stderr: error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start   (no echo of input)
```

**Reproducibility (raw SHA-256, no PE normalisation, no ephemeral linker flag):** two independent fresh-registry-cache **and** fresh-target-directory release builds of the same pinned `x86_64-pc-windows-gnu` toolchain with the unchanged `.cargo/config.toml` produced byte-identical executables:

| Build | Exit | Bytes | SHA-256 |
| --- | --- | --- | --- |
| A (fresh `CARGO_HOME` + fresh target) | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |
| B (fresh `CARGO_HOME` + fresh target) | 0 | 1,242,463 | `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` |

`identical = true`; lock blob `05a02666894f4f0bf5528acf118a049e5edb853e`. The acceptance target is equality **between the two newly built artifacts**, which holds; no comparison is claimed against any former release artifact (sources and check metadata changed).


## Gates

| Gate | Evidence | Result |
| --- | --- | --- |
| **B-01 — Safety and issue** | Exact baseline and tree, one checkout, sole `origin`, sole permitted Owner-installed ACTIVE change, WIP exactly four files, neutral NEXT, only allow-listed paths changed | **PASS** |
| **B-02 — Selected tool and graph** | `syn =3.0.9` dev-only in `solo-cli`; lock +1 line, no new package; real metadata graph legal for normal/dev/build; runtime closure has no external crate | **PASS** |
| **B-03 — Real source boundary** | AST checker invoked on all 12 real targets / 19 real files; legal tree exits 0; alias, re-export, qualified-path, privacy, module, include and macro analysis all exercised | **PASS** |
| **B-04 — Negative and bypass** | All 19 fixtures exit nonzero with their expected category; bypass probe and legal twins make disable/over-refusal fail the self-test; malformed and unsupported forms fail closed | **PASS** |
| **B-05 — No regressions** | fmt, strict Clippy (zero warnings), 87 tests, real CLI success + refusal, locked and `--offline` builds, and two byte-identical fresh-cache release builds | **PASS** |
| **B-06 — Security and cleanup** | No new `unsafe`, no new runtime dependency, bounded relative diagnostics with no secrets or absolute owner paths, licence/advisory review recorded, task scratch removed | **PASS** (advisory scan itself `NOT RUN`) |
| **B-07 — Report and final publication** | One canonical factual report, clean local tree, only allow-listed paths, one final push, remote identity confirmed | **PASS** |

## Verification limits — stated, not inferred

- **MSVC-hosted build: `UNVERIFIED`.** This host has no MSVC linker or SDK; all recorded results are the pinned GNU host variant.
- **Unix/macOS: `NOT RUN`.** No such host was available; nothing is inferred from the Windows result.
- **`cargo-deny` / advisory scanning: `NOT RUN`** (tool not installed). Direct licences were read from `cargo metadata`; a high/critical advisory remains unassessed and therefore cannot be claimed clean.
- **Macro semantics: deliberately conservative.** `macro_rules!`, proc-macro invocations and any unknown macro are *refused*, not expanded; the checker never claims compiler macro expansion or full name resolution.
- **`R01.C` scope untouched: `NOT RUN`** — CI wiring, HTTP/browser/production smoke, Node/TypeScript/Vite/Playwright and performance work were not started. The **full Stage 01 exit gate is not claimed**: one genuine request-to-application-port vertical-slice proof and the CI/browser evidence remain outstanding, and `R01.B` stays in ROADMAP until an independent Architect acceptance and separate Owner closure.
- **Deliberate lock-drift injection:** `NOT RUN` (see B-05).

## Changed-path ledger

Implementation commit `844f878c` (tree `3369e1fc`), exactly the authorized paths:

| Path | Change |
| --- | --- |
| `crates/solo-cli/Cargo.toml` | `syn =3.0.9` dev-dependency (`full`, `visit`) + `[[test]] check-architecture` (`harness = false`) |
| `Cargo.lock` | Cargo-generated, one added line |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | Shared checker: categories, violations, real-tree entry point, fixture registry, self-test |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | Cargo graph model + single inward-only rule set + metadata/packages |
| `crates/solo-cli/tests/architecture_checks/source.rs` | `syn` AST checker: aliases, re-exports, privacy, modules, includes, macros, dynamic loading |
| `crates/solo-cli/tests/check_architecture.rs` | Test-only process entry point and exit-code interface |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Ordinary `#[test]` gate executed by `cargo test` |
| `crates/solo-cli/tests/workspace_graph.rs` | Constrained refactor onto the shared rule set; all earlier negatives preserved and extended |
| `.documents/WIP/plan-next.md` | Issued candidate replaced by the exact neutral heading |
| `.documents/WIP/workorder-active.md` | Owner-installed order, bytes unchanged, carried into the commit |

No production Rust file, crate root, other manifest, workspace `Cargo.toml`, `.cargo/config.toml`, `rust-toolchain.toml`, README, AGENTS, About, Architecture, SOP, ROADMAP, OUTLOOK, `.gitignore`, licence/security file, workflow or other repository was modified.

## WIP state and scratch cleanup

`.documents/WIP/` still holds exactly four files with no subdirectories: ROADMAP keeps Stage 01 with `R01.B` and `R01.C` (nothing pruned — closure is not the agent's decision), NEXT is the neutral heading (blob `92fa562b…`), ACTIVE still contains the complete Owner-issued `SOLO-WO-030 R01` (the agent does not clear ACTIVE), OUTLOOK unchanged. All task-owned scratch — toolchain, caches, two reproducibility caches/targets, logs and the fixture/self-test outputs — lived under the git-ignored `.temp/rt` and was **deleted at run end**, leaving `.temp/` empty. No unknown or Owner-owned material was deleted, and no destructive Git operation, reset, rebase or force push was performed at any point.

