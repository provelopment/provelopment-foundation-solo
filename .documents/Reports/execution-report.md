# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | **`SOLO-WO-028 R01`** — Stage 01 `R01.A` scratch-root safety correction |
| Owner handoff | Received — instruction to align with `README.md`/`AGENTS.md` and execute the installed `SOLO-WO-028 R01` |
| Local root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified, single checkout, sole `origin`) |
| Starting HEAD / tree | `49955538c6b32a9d81e7464fba28a72d2cd04927` / `b5311d10251e974d1d26b65a7eec5b507e2dd56a` |
| Corrected source commit | **`5ae69bb9f396bb8bd33a62501e620f062d78a79c`** / tree **`0535cda375d95d0f7aafb7e8f6fa7ee45ecda1a5`** |
| Compiler / target | `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, host **`x86_64-pc-windows-gnu`** |
| **Status** | **`COMPLETE`** — all six `S-01`–`S-06` gates pass on the committed identity. Factual run status only, never Architect `ACCEPT`. |

## The defect and the correction

The previous validator resolved the candidate root **first** and inspected the **resolved** entry. That cannot prove the original `.temp` entry was not redirected: a junction or symlink whose destination is itself a plausible `.temp` directory inside the checkout presents, after resolution, an ordinary directory with no reparse attribute. Resolution-only inspection therefore accepts it.

`crates/solo-cli/tests/cli_smoke.rs` now does the opposite order:

1. From the canonical checkout root it builds the direct child `checkout.join(".temp")` **without resolving that entry**.
2. It inspects that original path with `symlink_metadata` before any `canonicalize`, traversal or use: on Windows it rejects `FILE_ATTRIBUTE_REPARSE_POINT` (`0x0400`) regardless of the directory bit; elsewhere it rejects symbolic links. Non-directories and metadata/permission errors are refused (`NotADirectory`, `Unreadable`) rather than treated as an empty or safe root.
3. A missing `.temp` is created with `create_dir` — the non-following, non-recursive call that fails when the name already exists — and the **newly created unresolved** entry is inspected before use. No `create_dir_all`, no following of an existing junction, no deletion or replacement of any existing entry.
4. Only after the raw-entry check passes is the path resolved, and the result must equal `checkout/.temp` **exactly** (`NotTheCheckoutTemp` otherwise). Accepting any directory that merely lives inside the checkout or merely ends in `.temp` is refused.
5. Exclusive unique children, non-recursive removal of positively owned empty children, no deletion of the shared root or foreign content, and no OS-temp fallback are all preserved. The real CLI still writes no file into its working directory.

The validation lives in one small helper, `ensure_scratch_root(checkout_root)`, so exactly the production test-path logic can be exercised against a controlled fixture root. No dependency, production option or unrelated behaviour was added.

## Raw-versus-resolved proof (independent probe, disposable scratch)

A real Windows directory junction was made in Solo `.temp/` fixture space, pointing at another `.temp` directory inside the same checkout. On that same fixture the two inspection strategies diverge exactly as the Architect described:

| Observation | Value |
| --- | --- |
| Unresolved entry attributes | `raw_entry_reparse = True` |
| Resolved destination | `<checkout>\.temp\probe-028\target\.temp` |
| Resolved is a directory | `True` |
| Resolved is itself a reparse point | `False` |
| Resolved inside the checkout | `True` |
| Resolved basename | `.temp` |
| **Previous post-resolution-only checks** | **`accept = True`** |
| **Revised unresolved-entry check** | **`reject = True`** |
| Material inside the target after the probe | intact (`keep`) |
| Removing the link leaves the target | `target_survived = True` |

This is the direct demonstration that the old arrangement would have been accepted and that only the pre-resolution check refuses it.

## Adversarial regression in the committed suite

`a_redirected_temp_resolving_inside_the_checkout_is_refused` builds, inside positively owned Solo `.temp/` fixture space, a container holding `target/.temp` (real directory with an existing material file) and `candidate/.temp` — a genuine directory junction to `target/.temp`. Both ends are inside the checkout. The test asserts, on the same fixture, that the resolved destination looks entirely acceptable (directory, not a reparse point, inside the checkout, basename `.temp`) — precisely why resolution-only logic fails — then asserts `ensure_scratch_root(&candidate) == Err(Redirected)`; then asserts the refusal changed nothing: the link is still a reparse point, the target directory still exists, and the material file still reads `must survive validation`. Cleanup removes only the fixture link first (asserting the target survives its removal), then its own material, `target/.temp`, `target` and `candidate`, finishing with the container's own `Drop`; the test proves no fixture residue remains and never touches the real checkout `.temp` entry.

The junction is created with `mklink /J` through `cmd` — a genuine directory reparse point needing no elevation, no policy change and no global configuration. On non-Windows hosts the same test uses a real directory symlink via `std::os::unix::fs::symlink`. No synthetic boolean substitutes for the end-to-end refusal.

Retained and strengthened positive coverage: genuine non-redirected `.temp` resolves to exactly `checkout/.temp`; a missing `.temp` is created and then inspected; a non-directory `.temp` is refused and left intact; exclusive child creation still refuses a colliding child and preserves its contents; cleanup still cannot remove a non-empty directory or foreign file; a successful CLI run still writes no file into its working directory.

## Gate evidence (all on committed `5ae69bb`)

| Gate | Evidence | Result |
| --- | --- | --- |
| **S-01 Checkout/scope** | Starting HEAD/tree, canonical report (`9fd446c9`), committed ACTIVE (`d5ad2a26`), NEXT (`92fa562b`), `cli_smoke.rs` (`b9d3b80b`), `main.rs` (`6faa9fae`), `.cargo/config.toml` (`74a4fb6c`) and `Cargo.lock` (`d64704da`) all matched; one worktree, sole `origin`, no stash, no branch; only the three permitted paths changed (ACTIVE, `cli_smoke.rs`, report) | **PASS** |
| **S-02 Raw entry check** | `symlink_metadata` inspects the unresolved `checkout/.temp` before any resolution; reparse/symlink, non-directory and metadata-error cases refused; resolution strictly afterwards and must equal `checkout/.temp` | **PASS** |
| **S-03 Adversarial refusal** | Real junction whose destination is inside the checkout refused (`Redirected`); old post-resolution checks demonstrably accept the same fixture; link, target and material preserved; target survives link removal | **PASS** |
| **S-04 Scratch ownership** | Genuine root, exclusive children, collision refusal and preservation, non-recursive cleanup, zero residue (`solo-cli-smoke-*` count `0` after the whole suite), no OS-temp fallback | **PASS** |
| **S-05 No regressions** | `fmt` `0`, `clippy -D warnings` `0`, `test` `0` — **80 tests** (15 CLI process, 13 graph, 13 base, 11 identity, 11 adapter, 9 application, 8 CLI unit); release build `0`; real CLI `0`/`2`; two independent clean builds byte-identical | **PASS** |
| **S-06 Reporting/publication** | One overwritten report, authorized paths only, clean tree, one final push, remote identity verified | **PASS** |

## Commands executed (exit codes as observed locally)

`rustc --version` → `0` (`1.99.1`); `cargo --version` → `0` (`1.99.1`); host `x86_64-pc-windows-gnu`; `cargo fmt --all -- --check` → `0`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` → `0`; `cargo test --workspace --all-targets --locked` → `0` (80 tests, 10 suites); `cargo build --workspace --release --locked` → `0`; `cargo metadata --format-version 1 --locked --all-features` → `0`; `cargo tree --workspace --all-features --locked` → `0`; `cargo test --workspace --all-targets --locked --offline` → `0`; `cargo build --workspace --release --locked --offline` → `0`. Real compiled binary: `entity-7` + `https://EXAMPLE.com:443/alpha` → `0`, `preflight=ok entity_id=entity-7 base=https://example.com/alpha`; `Main` → `2`, `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start`.

**Reproducibility:** two independent builds with separate clean task-owned caches and targets (`rcA/tA`, `rcB/tB`), same pinned GNU target, `RUSTFLAGS` empty in both shells so the tracked `.cargo/config.toml` supplies the flag: both `solo-cli.exe` are `1,242,463` bytes with SHA-256 `363AC93CDD286A1C424A22ACB8D973F6A27A9F473F368E70D34E24399290FF9F` — **raw bytes exactly equal**, no timestamp normalization. It matches the prior run's value, as expected, since only a test source file changed. `Cargo.lock` blob `d64704da…` and `.cargo/config.toml` blob `74a4fb6c…` are unchanged from baseline.

## Changed files

`crates/solo-cli/tests/cli_smoke.rs` (scratch-root validator plus positive and adversarial tests), `.documents/WIP/workorder-active.md` (Owner-installed text, committed with content unchanged), `.documents/Reports/execution-report.md` (this overwrite). No other permanent path changed: `Cargo.toml`, `Cargo.lock`, crate manifests, `rust-toolchain.toml`, `.cargo/config.toml`, `crates/solo-cli/src/main.rs`, all other Rust files, README/AGENTS/About/Architecture/SOP, ROADMAP, NEXT, OUTLOOK, `.gitignore` and security/license files were untouched. Both edited files are stored LF.

## Limits, deferred scope and cleanup

**UNVERIFIED / NOT RUN:** MSVC-hosted build (no MSVC linker or SDK on this host — never inferred as passing); `cargo-deny`; Node/browser/Playwright and performance measurement; the Unix symlink branch of the adversarial test was not executed here, as no Unix host was used. **Deferred and not passed:** `R01.B` source-level import checker, `R01.C` CI and production-browser suite, `C1`–`C4`, `F01`–`F17`, `RG01`–`RG11`, Stage 02+ publication/routing/site/locale/visibility, and all release/deployment work. `COMPLETE` is factual run status only: `R01.A` is not self-accepted, ACTIVE was not cleared, ROADMAP still carries `R01.A`/`R01.B`/`R01.C`, and NEXT remains the neutral heading.

Cleanup: all task-owned material lived in Solo `.temp/` (pinned toolchain, Cargo caches, target directories, metadata capture, the junction probe) and is removed at run end; the probe fixture and its target were deleted and verified gone, and the real `.temp` entry was never redirected (`reparse=False`) or deleted. No foreign or Owner-owned content was touched and no destructive Git operation was performed.


