# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | **`SOLO-WO-027 R01`** — `R01.A` corrective completion |
| Owner handoff | Received — instruction to align with `README.md`/`AGENTS.md` and execute the installed `SOLO-WO-027 R01` |
| Local root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified, single checkout) |
| Reviewed starting HEAD / tree | `1380f61a64949d3fab070f6ec492df1b8092dba0` / `2c620bcabe2a91bd2127ede9a267550ecb2acf58` |
| Corrected implementation commit | **`f75f1b0ef18fffa438c6fca59063fd5c08e0b581`** / tree **`689e8dfb7e28867f43ba3a8eba23783590889827`** |
| Compiler / target | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host **`x86_64-pc-windows-gnu`** (installed GNU variant of the pinned 1.99.1; `rust-toolchain.toml` unchanged) |
| **Status** | **`COMPLETE`** — all seven `C-01`–`C-07` gates pass on the committed identity. This is factual run status, never Architect `ACCEPT`. |

## Agent-observed corrections

**3.1 Deterministic GNU executable — implemented.** Created `.cargo/config.toml` containing only `[target.x86_64-pc-windows-gnu]` with `rustflags = ["-C", "link-arg=-Wl,--no-insert-timestamp"]` plus an explanatory comment. No global `RUSTFLAGS`, no binary patching, no normalized comparison, no dependency or pin change, and no effect on MSVC/Linux/other targets. Verified genuinely in effect: `cargo build --workspace --release --locked -v` shows the flag inside the actual `rustc` invocations, and it was **not** supplied through any environment override (`RUSTFLAGS` was empty in every build shell).

**3.2 Truthful CLI output errors — implemented.** `crates/solo-cli/src/main.rs` now routes required output through a local `OutputSink` trait with `StreamSink` for the real streams and a recording mock for tests. Any failing `write_all` **or** `flush` on required stdout/stderr returns internal `1`; a fixed ASCII line `error: PREFLIGHT_INTERNAL: output-failure` is attempted best-effort and cannot change the status. Normal outputs are byte-for-byte unchanged (`0` with the exact single success line; `2` with the exact refusal line, no stdout). No new crate, dependency or port; the single composition root is preserved.

**3.3 Safe test scratch — implemented.** `crates/solo-cli/tests/cli_smoke.rs` no longer uses the general OS temporary directory. The root is this checkout's own `.temp/`, derived from the compile-time manifest path and validated by canonicalization (must stay inside the checkout) and by `FILE_ATTRIBUTE_REPARSE_POINT`/symlink inspection. Children are uniquely named (`pid` + nanoseconds + atomic sequence), created with `create_dir` so creation is exclusive, and removed with a **non-recursive** `remove_dir` that can only delete an empty child this test created. There is no OS-temp fallback: if the authorized area cannot be safely established the test fails.

**3.4 Full dependency evidence — completed.** The prior report was wrong to describe the lock as eleven packages while reviewing five; the lock actually holds **15 packages = 4 workspace + 11 external**. All eleven external crates are reviewed individually below with checksum, registry identity, licence, attribution and advisory status.

## Eleven-package supply-chain inventory (all `registry+https://github.com/rust-lang/crates.io-index`)

| # | Package | Version | Checksum (sha256 prefix) | Licence | Attribution | Advisory review |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `itoa` | 1.0.18 | `8f42a60c…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 2 | `memchr` | 2.8.3 | `cf8baf1c…` | Unlicense OR MIT | Unlicense/MIT notice | OSV: none |
| 3 | `proc-macro2` | 1.0.107 | `985e7ec9…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 4 | `quote` | 1.0.47 | `1fbf4db1…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 5 | `serde` | 1.0.229 | `4148590a…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 6 | `serde_core` | 1.0.229 | `67dca2c9…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 7 | `serde_derive` | 1.0.229 | `e7a5d712…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 8 | `serde_json` | 1.0.154 | `e7e9cc8b…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 9 | `syn` | 3.0.9 | `d78c8dee…` | MIT OR Apache-2.0 | MIT/Apache notice | OSV: none |
| 10 | `unicode-ident` | 1.0.27 | `a2c754d6…` | (MIT OR Apache-2.0) AND **Unicode-3.0** | **Unicode-3.0 notice required** | OSV: none |
| 11 | `zmij` | 1.0.23 | `29666d0a…` | MIT | MIT notice | OSV: none |

**Method and limits.** Each lock checksum was compared against the per-version `checksum` reported by `crates.io` and matched exactly, so every artifact is the genuine published crate; none is yanked. Advisory data came from a read-only `OSV.dev` `POST /v1/query` per `(name, version)` on 2026-11-07, which aggregates RustSec advisories; no advisory was returned for any of the eleven exact versions. Attribution: `unicode-ident` additionally carries the Unicode-3.0 licence, so a `NOTICE`/licence-text inclusion is a **release-packaging obligation** still outstanding; it is not a blocker for this corrective slice, and no `NOTICE` or SBOM was created here (not authorized).

`cargo-deny` is **`NOT RUN`** — no `cargo-deny` executable exists in this environment (`cargo_deny_present=False`). No scanner pass is claimed; the table is a manual record with the stated sources and date, not a completed advisory scan.

Dependency graph unchanged: `Cargo.lock` is byte-identical to the reviewed baseline (Git blob `d64704daf4f1affcdcb1c2572a240cf833d97948`, SHA-256 `C5BB110228B6D21817C8108CBDE3422345EEC81A7EBB0D2EACFDD0F7CFE3D6C7`), and `cargo tree --workspace --all-features --locked` is unchanged. Six of the eleven — `proc-macro2`, `quote`, `serde`, `serde_derive`, `syn`, `unicode-ident` — are locked resolvable entries on optional/feature-gated paths and are **not** part of the activated build graph; the five compiled externals are `itoa`, `memchr`, `serde_core`, `serde_json`, `zmij`, reached only through the `solo-cli` dev-dependency. No new direct or transitive package was introduced.

## Gate evidence (all on committed `f75f1b0`)

| Gate | Evidence | Result |
| --- | --- | --- |
| **C-01 Source safety** | Reviewed HEAD/tree and all twelve baseline blobs matched (`README` `3f3005d6`, `AGENTS` `32562b67`, Architecture `03e8923f`, both SOPs, `Cargo.toml` `14505be4`, `Cargo.lock` `d64704da`, `rust-toolchain.toml` `9ec55b3b`, `main.rs` `610efd3a`, `cli_smoke.rs` `cfbaf689`, report `c4e26b26`, NEXT `92fa562b`); one worktree, sole `origin`, no stashes, no reparse points; only the four authorized paths changed | **PASS** |
| **C-02 GNU reproducibility** | Four independent release builds across fresh task-owned caches and targets (`rcA/tA`, `rcB/tB`, `rcE/tE`, `rcF/tF`): every `solo-cli.exe` is `1,242,463` bytes with SHA-256 `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` — all identical, on both the pre-commit and the committed identity; no environment override; lock blob unchanged | **PASS** |
| **C-03 Output truthfulness** | 8 injected-failure unit tests: failing stdout `write_all` and `flush`, and failing stderr `write_all` and `flush`, each return `1` with no false `0`/`2`; normal paths byte-identical to before; diagnostic is fixed ASCII and never echoes OS error text | **PASS** |
| **C-04 Scratch confinement** | 4 scratch tests: root resolves inside the checkout and is not a reparse point; exclusive creation refuses a pre-existing child (error kind `AlreadyExists`) while preserving its contents; cleanup cannot remove a non-empty directory or foreign file; the successful run leaves no residue | **PASS** |
| **C-05 Dependency completeness** | Eleven-row inventory above; every checksum verified against crates.io; none yanked; no advisory returned by OSV for any exact version; `unicode-ident` Unicode-3.0 notice recorded as an outstanding release obligation; `cargo-deny` honestly `NOT RUN` | **PASS** (manual record; no scanner claim) |
| **C-06 No `R01.A` regression** | `cargo fmt --all -- --check` `0`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` `0`; `cargo test --workspace --all-targets --locked` `0` — **77 tests** (11 identity, 13 base, 9 application, 11 adapter, 8 CLI unit, 12 CLI process, 13 graph); `cargo build --workspace --release --locked` `0`; real-process cases unchanged; graph negatives and guard-disabled proof unchanged | **PASS** |
| **C-07 Final evidence** | This single overwritten report; four WIP files; ROADMAP/NEXT/OUTLOOK untouched; ACTIVE still holds this order; no unauthorized path; clean tree; one final push; scratch cleaned | **PASS** |

## Commands executed (exit codes as observed locally)

`rustc --version` → `0`; `cargo --version` → `0`; `cargo fmt --all -- --check` → `0`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` → `0`; `cargo test --workspace --all-targets --locked` → `0`; `cargo build --workspace --release --locked` → `0`; `cargo metadata --format-version 1 --locked --all-features` → `0`; `cargo tree --workspace --all-features --locked` → `0`; `cargo build --workspace --release --locked --offline` → `0`; `cargo test --workspace --all-targets --locked --offline` → `0` (10 suites, after trusted caches were populated); `cargo build --workspace --release --locked -v` → `0` (flag observed in the `rustc` invocations); fresh-cache builds A/B/E/F → `0`, identical hashes; deliberate `--locked` refusal in disposable OS-temp scratch with no lock file → exit `101` with `error: cannot create the lock file … because --locked was passed`, probe deleted. Real-process behaviour: `entity-7` + `https://EXAMPLE.com:443/alpha` → exit `0`, `preflight=ok entity_id=entity-7 base=https://example.com/alpha`; `Main` → exit `2`, `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start`; `https://192.168.0.1` → exit `2`, `error: PREFLIGHT_BASE_UNSUPPORTED: ip-literal`; missing `--base-url` → exit `2`, `error: PREFLIGHT_ARGS_MISSING: --base-url`.

## Changed files in this correction

`.cargo/config.toml` (new, 7 lines), `crates/solo-cli/src/main.rs` (+309/−0 net rewrite of output handling plus unit tests), `crates/solo-cli/tests/cli_smoke.rs` (scratch confinement and regressions), `.documents/WIP/workorder-active.md` (Owner-installed text, committed with content unchanged), `.documents/Reports/execution-report.md` (this overwrite). No other path changed; `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, crate manifests, README/AGENTS/About/Architecture/SOP/security/licensing files, ROADMAP, NEXT and OUTLOOK were not edited, and every new index entry is stored as LF.

## Limits, deferred scope and cleanup

**UNVERIFIED / NOT RUN:** MSVC-hosted build (no MSVC linker or Windows SDK exists on this machine — never inferred as passing); `cargo-deny`; Node/npm/TypeScript/Vite/Playwright; any browser or performance measurement. **Deferred and not passed:** `C1`–`C4`, `F01`–`F17`, `RG01`–`RG11`, the `R01.B` source-level import checker, the `R01.C` CI and production-browser suite, Stage 02+ publication/routing/site/locale/visibility, and all provider/release/deployment work. `COMPLETE` here is factual run status only: `R01.A` is not self-accepted, ACTIVE was not cleared, ROADMAP still holds `R01.A`/`R01.B`/`R01.C`, and NEXT remains the neutral heading.

Cleanup: all task-owned material was confined to Solo `.temp/` (toolchain, four Cargo caches, five target directories, metadata capture) and is removed at run end; the OS-temp lock-refusal probe was deleted and verified gone. No foreign or Owner-owned content was touched, and no destructive Git operation was performed.


