# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 05:00–05:12 |
| Work order | **`SOLO-WO-035 R01`** — Stage 01 R01.C: Verification Infrastructure and CI |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-035 R01` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / `main` / sole `origin` |
| Starting HEAD / tree | `e7f7264daf113a76b0ee151a603caba798143d03` / `55ff2916c3201f65b8f1c83ed46e8493ec9e43e9` |
| Final source revision | **`b5c6db0d7a83a27cf43ed1323e2518487a2775e0`** — the revision the green CI run evaluated |
| **Status** | **`COMPLETE`** for the R01.C CI/tooling layer. **Not** an R01.C acceptance, **not** a Stage 01 closure, **not** a release. |

## Preflight and authority (C-01)

One checkout on `main` at `e7f7264`, one worktree, no stash, sole `origin` = the intended GitHub repository, no conflict or unsafe link. The only pre-existing tracked difference was the Owner's complete installation of `SOLO-WO-035 R01` into `.documents/WIP/workorder-active.md` (32,731 bytes, blob `088892fd9173c947ef9ac9f59c147a61ff470df4`, LF-only, no BOM, `OWNER-ISSUED`, seven `###` sections, matching handoff identifier) — preserved **byte-for-byte** and committed as the issued-order record; the agent did not reword, clear or self-accept it.

Twenty-two reviewed baseline blobs matched with zero drift, including `README.md` `3f3005d6`, `AGENTS.md` `32562b67`, About `1d80f627`, Architecture `03e8923f`, both SOPs, `Cargo.toml` `14505be4`, `Cargo.lock` `05a02666` (`syn =3.0.9`), `rust-toolchain.toml` `9ec55b3b` (`1.99.1` + rustfmt + clippy), `.cargo/config.toml` `74a4fb6c` (scoped to `[target.x86_64-pc-windows-gnu]` only), `crates/solo-domain/src/lib.rs` `b3fd9f29`, `main.rs` `6faa9fae`, the checker sources `92f6e0e3`/`89dfea91`/`e328dd6a`/`0bad0e02`/`93fe5733`/`8fa4b4d0`/`921cbbe9`, ROADMAP `6f753fdb`, the prior R01.C candidate `f435de59`, and OUTLOOK `cb9c7a61`. The starting tree had `.github/CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `PULL_REQUEST_TEMPLATE.md`, `SECURITY.md` and **no** `.github/workflows/`; the Actions workflows API reported `total_count: 0`.

The only external action pin was independently verified before use: `actions/checkout` tag `v4.3.0` resolves to commit `08eba0b27e820071cde6df949e0beb9ba4906955`, exactly as the order states.

`.documents/WIP/` holds exactly four files and no subdirectory. ROADMAP still carries R01.C and the Stage 01 prerequisite/exit gate unchanged; OUTLOOK is unchanged; NEXT was neutralised to `# Foundation Solo — plan-next (Next Candidate)\n` (49 bytes, blob `92fa562b`, LF, no BOM) as the issued transition — no other candidate exists.

## What was built (permitted ledger only)

| Path | Change |
| --- | --- |
| `.github/workflows/rust-verify.yml` | **created** — 25 steps, one job, `ubuntu-24.04` + `windows-2025` matrix |
| `crates/solo-cli/tests/ci_gate_tripwire.rs` | **created** — one test-only probe, no manifest or dependency change |
| `.documents/WIP/plan-next.md` | **replaced** by the neutral heading (§2.9) |
| `.documents/WIP/workorder-active.md` | **staged unchanged** — the issue-order record, byte-identical to the Owner's installation |
| `.documents/Reports/execution-report.md` | **overwritten** — this file |

Everything else is read-only and verified unchanged: no product source, crate manifest, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, root `.gitignore`, README/AGENTS, About/Architecture/SOPs, ROADMAP, OUTLOOK, `.github/SECURITY.md`, and no `scripts/`, `browser/`, second workflow, artifact archive or secondary report.

**Workflow design (least privilege, fail-closed):** `permissions: contents: read` with no other permission and no write-capable step; one `actions/checkout` per job pinned to the full commit SHA with `persist-credentials: false`; job-level `CARGO_NET_OFFLINE: "true"` so **`cargo fetch --locked` is the only step that reaches the network**; the toolchain installed as exactly `1.99.1` with rustfmt and Clippy and then asserted to report `rustc 1.99.1`; every mandatory gate is a visible failing step with no `continue-on-error`, no `|| true`, no `-no-fail`, no `-ignore`, no broad `if` skip and no `pull_request_target`. Triggers are `push` to `main` and `pull_request` targeting `main`.


## Local verification (pinned toolchain, Owner's checkout)

Pinned `rustc 1.99.1 (58cf8f9c3 2026-10-13)` / `cargo 1.99.1 (5f94df478 2026-08-27)` installed only into the run's Solo `.temp` scratch; build output confined to `.temp`; `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` confirmed **absent** so the tracked `.cargo/config.toml` link flag applies. `Cargo.lock` SHA-256 `0f78cd0606141ca2497da75e359d8082364f0d9bdc422ba7f470c4cef2031257` before and after — the lock was never modified.

| Gate | Observed | Result |
| --- | --- | --- |
| `cargo metadata --locked` (full and `--no-deps`) | exit 0; packages exactly `solo-adapters`, `solo-application`, `solo-cli`, `solo-domain`; unique `solo-*` names = 4 | **PASS** |
| `cargo fmt --all -- --check` | exit 0 (after one rustfmt correction to the new tripwire file) | **PASS** |
| `cargo clippy … -D warnings` | exit 0, zero warnings | **PASS** |
| `cargo test --workspace --all-targets --locked` | exit 0; **108 tests** across **12** binaries, 0 failed (was 107/11; +1 tripwire test) | **PASS** |
| Real-tree checker | exit 0, `architecture=legal packages=4 targets=13 files=20 edges=22` | **PASS** |
| `--self-test` | exit 0, `self-check=passed cases=29`; 23 `refused as <category>` lines, 5 `legal form accepted`, 0 `FAIL` | **PASS** |
| Named fixture categories | `--fixture` for `direct-outward`, `renamed-alias`, `reexport-chain`, `checker-bypass` → exit 1, `outcome=refused … category_match=true`, `expected=layer-outward-reference`; `dynamic-load` → exit 1, `expected=dynamic-loading` | **PASS** |
| Usage errors | `--disable` → exit 2; `--fixture not-a-real-fixture` → exit 2 | **PASS** |
| Tripwire normal | exit 0, `test result: ok. 1 passed; 0 failed` | **PASS** |
| Tripwire injected (`SOLO_CI_INJECT_TEST_FAILURE=1`) | exit **101**, diagnostic `SOLO_CI_TRIPWIRE_INJECTED_FAILURE` present, `1 failed` | **PASS** |
| Release build | exit 0; `solo-cli.exe` 1,242,463 bytes, SHA-256 `363ac93cdd286a1c424a22acb8d973f6a27a9f473f368e70d34e24399290ff9f` | **PASS** |
| Release CLI accepted path | exit 0; stdout byte-identical to `preflight=ok entity_id=entity-7 base=https://example.com/alpha\n`; stderr empty | **PASS** |
| Release CLI refused path | exit 2; stdout empty; stderr byte-identical to `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start\n` | **PASS** |
| Windows-GNU two clean builds | two independent empty target directories, both exit 0, `1,242,463` bytes each, identical SHA-256 `363ac93c…ff9f`, raw bytes sequence-equal — and identical to the accepted R01.B digest | **PASS** |

The Windows-GNU reproducibility result is recorded separately from the MSVC host: this host is GNU, Windows 2025 MSVC remains a build/test/CLI host only, and no MSVC byte-identity is claimed.


## GitHub Actions evidence — actual runs at actual revisions

| Run | Revision | Result |
| --- | --- | --- |
| 1 — `42449012942` (#1) | `92c87a7c4e59ba2e75763f2afe1ec47e204937ba` | **both failed** — two real defects, see below |
| 2 — `42449232294` (#2) | `1fab18263d9c100897e19530c8ab8590563392d9` | windows **success**, ubuntu failed (category assertion) |
| 3 — `42449400312` (#3) | `93d200b2c5dcd36d000a59d8fd9cc1f57f95ae7a` | windows **success**, ubuntu failed (diagnostic notice emitted) |
| 4 — `42449568184` (#4) | `b5c6db0d7a83a27cf43ed1323e2518487a2775e0` | **both success** |

Run links: <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42449012942>, <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42449232294>, <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42449400312>, <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42449568184>.

**Two real defects were found by the first run and fixed, not waived.** (1) GitHub invokes bash with `-e`, so the Ubuntu adversarial step's deliberately failing `--fixture` and injected-checker invocations aborted it before their statuses were asserted; that step now disables `-e` explicitly and checks every status by hand. (2) GitHub's `pwsh` wrapper exits with the residual `$LASTEXITCODE`, so the Windows tripwire step failed *after* correctly verifying the injected failure; it now clears that value and exits 0 only once its assertions pass. A third correction pinned the injected-import category: the scanner classifies a `solo-domain → solo_adapters` reference as **`undeclared-crate`** (a workspace root is resolvable only from *declared* dependencies, and `solo-domain` declares none), while declared-but-outward forms are refused as `layer-outward-reference`; both are now required explicitly.

Run 4 (`b5c6db0`), read back independently through the public Actions and check-run APIs: `verify (ubuntu-24.04)` — run label `ubuntu-24.04`, **conclusion `success`**, 23 s, every Linux step `success` and every Windows-only step `skipped`; `verify (windows-2025)` — **conclusion `success`**, 57 s, mirror-image step pattern. That adversarial step's own annotation records `injected forbidden import refused with category 'undeclared-crate' and exit 1`. Step-level evidence that the gates are load-bearing rather than skipped: the release-smoke step can only pass with a real `target/release/solo-cli[.exe]`, the two-clean-build step can only pass after two fresh release builds, and the tripwire step can only pass after a genuine failing test carrying its diagnostic.

**Honest limitations.** The `Job summary (Linux)`/`(Windows)` steps reported `success`, which proves they ran and wrote the summary; the rendered summary panel is not readable without an authenticated session, so its exact text is verified from the committed workflow rather than read back. That text states in the summary: `Browser smoke: UNVERIFIED / NOT RUN — no authorized browser UI, served Page, HTTP adapter or Node/Playwright project exists at this stage`. Raw job logs are likewise unavailable unauthenticated; conclusions, step tables and annotations were read through the public API instead. The only warning class on every run is one GitHub platform notice: `actions/checkout@08eba0b2…` targets Node.js 20 and is being run on Node.js 24 — an upstream runner deprecation notice, not a gate warning, and it affects neither the pinned commit nor the result.


## Gate ledger

| Gate | Evidence | Result |
| --- | --- | --- |
| **C-01 Identity, issuance and scope** | Starting HEAD/tree and 22 blobs verified; Owner-installed complete order plus matching handoff; one root/worktree/origin; exactly the five approved permanent paths differ; NEXT cleared only as authorized; ROADMAP and OUTLOOK preserved | **PASS** |
| **C-02 Pinned, least-privilege CI** | Workflow accepted and executed by GitHub Actions; sole action pinned to full SHA `08eba0b2…` (verified against tag `v4.3.0`); `permissions: contents: read`; `persist-credentials: false`; both `ubuntu-24.04` and `windows-2025` jobs genuinely ran at published revisions with `push`/`pull_request` scoping | **PASS** |
| **C-03 Toolchain/locked/offline** | Both hosts installed and asserted `rustc 1.99.1`; lock digest recorded; `cargo fetch --locked` the only network step, every later Cargo command offline and `--locked`; lock unchanged, no dependency added | **PASS** |
| **C-04 Standard Rust gates** | Format, `-D warnings` Clippy, all-target locked tests, real-tree checker and the 23-fixture `--self-test` ran and passed on both hosts with named-category assertions | **PASS** |
| **C-05 Actual release CLI** | Release binary on each host returned exit 0 / exit 2 with byte-identical stdout and stderr including the trailing LF; no debug-binary substitution, no masked status | **PASS** |
| **C-06 Deliberate test fault** | Tripwire passes unmodified on both hosts; with `SOLO_CI_INJECT_TEST_FAILURE=1` the real Cargo test process fails with the specific diagnostic and exactly one failed test; missing, unrelated or swallowed outcomes fail the probe | **PASS** |
| **C-07 Forbidden import and bypass** | Ubuntu: precompiled checker exited 0 on the legal tree, then exit **1** with category `undeclared-crate` after a genuine domain→adapter source edit (scanner evidence, not a compiler error); `checker-bypass`, `direct-outward`, `renamed-alias`, `reexport-chain` refused as `layer-outward-reference` and `dynamic-load` as `dynamic-loading`; usage errors exit 2; source restored byte-for-byte with a clean `git diff --exit-code` and the tree legal again | **PASS** |
| **C-08 Determinism and cleanup** | Ubuntu: two fresh offline release target directories produced identical raw binary digests; local Windows-GNU pair likewise identical and equal to the accepted R01.B digest; no lock, source or workflow residue; task scratch removed; foreign material preserved; single checkout | **PASS** |
| **C-09 Failure propagation/security** | No `continue-on-error`, masked exit, implicit skip, unpinned action or write token; the injected test fault, the illegal import and the version assertions demonstrably fail at their own gate (proved by runs 1–3); no secrets printed; read-only tokens for fork PRs | **PASS** |
| **C-10 Truthful non-runs** | Browser/HTTP/Node/Playwright, Stage 02+ C1/C2/C4 and unimplemented future commands are `UNVERIFIED`/`NOT RUN`; no claim that green Rust CI closes Stage 01 or authorizes a release | **PASS** |
| **C-11 Publication and independent reviewability** | Published paths are exactly the approved ledger plus the unchanged installed order; remote `origin/main` and the evaluated revision verified; workflow conclusions read back for that revision; canonical report published; clean checkout; four WIP files | **PASS** |

## Scratch, state and remaining obligations

One checkout, one worktree, one `origin`. The run's owned scratch was `.temp/w035` (downloaded `rustup-init.exe`, pinned toolchain homes, two independent clean target directories, captured streams and logs); it is deleted before handback. Foreign material was preserved untouched and is **not** this run's: `.temp/rt/tmp/…`, `.temp/w032/…` and `.temp/w033/…` each hold a 501-byte `dotnet-suggest-registration-git-credential-manager` sentinel written by an external service. Owner cleanup trigger: delete those three directories when no external service holds them. `.temp/architecture-fixtures` remains an empty shared parent. No repository content was deleted, reset, cleaned or rewritten, and no second clone, worktree or nested repository was created. The only source mutation anywhere was inside the disposable GitHub runner checkout, restored byte-for-byte under a guaranteed trap.

**State at handback:** NEXT holds only the neutral heading; ACTIVE holds the Owner-installed `SOLO-WO-035 R01` order unchanged as the issued-order record and grants no further authority; ROADMAP still carries **R01.C** and the Stage 01 prerequisite/exit gate; OUTLOOK is unchanged. No merge, release, deployment, provider, credential or GitHub-settings action was taken, and no distributable was published.

**Still open and explicitly not satisfied by this run:** R01.C is *not* accepted and the **Stage 01 exit gate remains open** — both await independent Chief Architect review and a separate Owner-authorized reconciliation. Browser smoke, HTTP suites, Node/Playwright, Stage 02+ capabilities (C1/C2/C4), accessibility numerical conformance, provider conformance, `cargo-deny`/advisory scanning, the `unicode-ident` Unicode-3.0 `NOTICE` prerequisite, distribution and deployment remain `UNVERIFIED`/`NOT RUN`; this workflow certifies none of them and does not constitute a security audit.

**Single next step:** return **"agent report ready"** for independent Chief Architect `ACCEPT / REVISE / STOP` against the published revision.

Gates executed on **both** hosts: workspace package-set (exactly the four approved crates via `cargo metadata --locked --no-deps`), `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-targets --locked`, the real-tree checker, the 23-fixture `--self-test` matrix with named-category assertions, the failure tripwire, `cargo build --workspace --release --locked` and byte-exact release-binary smoke. Ubuntu additionally runs the injected-forbidden-source probe with guaranteed restoration and two independent clean release builds.
