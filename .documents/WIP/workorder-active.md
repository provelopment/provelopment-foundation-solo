# Foundation Solo — Active Work Order

## SOLO-WO-027 R01 — R01.A Corrective Completion

## Approved Scope — OWNER-ISSUED

### 1. Objective and exact starting state

Complete the still-open **Stage 01 `R01.A`** acceptance of the Rust engineering foundation already implemented under `SOLO-WO-026 R03`. Correct and prove the four bounded findings from the Chief Architect's independent `STOP` review:

1. The default Windows GNU release executable was not byte-identical across two independent clean builds; the PE `TimeDateStamp` and derived `CheckSum` differed.
2. `solo-cli` ignores stdout/stderr `write_all` and `flush` failures and can therefore return a misleading success or refusal status.
3. A CLI smoke test creates and deletes a predictable directory under the general OS temporary directory rather than an authorized, exclusively task-owned scratch location.
4. The canonical execution report identifies only five of the eleven locked external packages in its manual supply-chain review, leaving six without individual evidence.

This is a **correction to `R01.A`**, not a new product feature. Preserve the current four-crate workspace, typed Entity ID and narrow base-candidate preflight, real CLI→adapter→application→domain call, and inward-only dependency graph. Preserve the accepted full public-address contract as a later-stage obligation. Do not implement `R01.B`, `R01.C`, Stage 02+, publication, HTTP, browser, CI, Vercel integration, release or deployment.

**Repository:** `provelopment/provelopment-foundation-solo`; existing local branch `main`.

**Reviewed starting HEAD:** `1380f61a64949d3fab070f6ec492df1b8092dba0`.

**Reviewed starting tree:** `2c620bcabe2a91bd2127ede9a267550ecb2acf58`.

**Implementation source in this tree:** `d3df20af4c76ebd734cf0b0b8118e2e401b3373b`; its source tree is `71bf0196f7a28065b422effdf0dafddaf56a6e6a`.

**Canonical prior report:** `.documents/Reports/execution-report.md`, Git blob `c4e26b26077158b5375b6e8608fcb179f876c442`, status `STOPPED`. This is the only report to overwrite; its earlier contents remain in Git history.

**WIP at this baseline:** ROADMAP still contains `R01.A`, `R01.B` and `R01.C`. NEXT is already neutral (blob `92fa562b1af0c54391851aa5c79deb1631127069`), OUTLOOK unchanged, and ACTIVE contains the previous order `SOLO-WO-026 R03` (blob `7984e34793c66a5d5bb482a3da5ae417dc8fcce4`). The Owner's installation of this exact replacement ACTIVE is the only expected pre-existing tracked change at the start of this run. Leave NEXT, ROADMAP and OUTLOOK unchanged. Never clear ACTIVE or mark `R01.A` accepted.

Read `README.md`, `AGENTS.md`, the About and Architecture documents, Architect and Implementation SOPs, the four WIP files and the current report in their mandatory order. In particular preserve Architecture §§1.1, 4.1, 5, 5A, 6.2B, 6.2J, 6A.2–6A.4, 6A.15; Architect SOP §§6, 6A, 6B, 7A, 7B, 8; and Implementation SOP §§4–7A. Architect SOP §6B remains the sole Git procedure; §7B remains the A1 standard. Do not invent a parallel policy.

### 2. Local repository and authorization preflight

Work in the **single existing local Solo checkout**, whose expected Owner location is `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; establish the actual available root rather than assuming a path. Before any writes, dependency/toolchain installation or build:

- Verify Git root, branch `main`, HEAD and tree, `origin`, remote tracking, status (including untracked and ignored), stashes, `git worktree list --porcelain`, symlink/reparse points and any Owner-owned or unrelated material. Exactly one Solo checkout/worktree is allowed; no clone, secondary worktree, shadow tree or destructive cleanup.
- Verify the reviewed HEAD/tree above and these immutable file blobs: `README.md` `3f3005d680d9ed182a02e574744054aaae55deb2`; `AGENTS.md` `32562b6730e26a37b9febae63efd7d5c6a5866ac`; Architecture `03e8923f834b50fc28e1b56e70f73017525cfa5b`; Architect SOP `8760be323b6ed85695a9dcffd3e52f94628f0f07`; Implementation SOP `a577116082bf8133a6056667aa52fe4ffc2ae94d`; `Cargo.toml` `14505be49a281ba0495eabea5ca386548cc36347`; `Cargo.lock` `d64704daf4f1affcdcb1c2572a240cf833d97948`; `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`; `crates/solo-cli/src/main.rs` `610efd3a0973b9e15b56f4c87a9f0b66f0366ad6`; `crates/solo-cli/tests/cli_smoke.rs` `cfbaf6897acdd4c3d4fb8209b78fc6e1d16697d2`.
- Verify `.documents/WIP/` has **exactly** `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md` and no extra files/directories. The Owner-installed ACTIVE must identify **`SOLO-WO-027 R01`** and the complete scope here. Its difference from the previous committed ACTIVE is expected; any other unexplained local change is a STOP.
- Inspect available compiler/linker, OS/host triple, toolchain provenance and Cargo configuration. Do not silently substitute MSVC for GNU, install a different Rust version, or reinterpret a failed executable gate. Any existing unreviewed Cargo config/rustflags affecting these tests must be disclosed and resolved before proceeding.

STOP on drift, unexpected dirty/untracked files, conflicting orders, unsafe paths, uncertain ownership or inability to establish these facts. Never delete or overwrite unfamiliar content to make a checkout look clean.

### 3. Exact code corrections

**3.1 Deterministic Windows GNU executable.** Create `.cargo/config.toml` with only the target-specific mechanism below (plus, at most, a brief explanatory comment):

```toml
[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-Wl,--no-insert-timestamp"]
```

The baseline agent isolated the GNU linker PE wall-clock timestamp as the source of the two-byte difference and observed byte-identical results when this argument was used experimentally. Commit the setting so ordinary locked release builds use it by default on **`x86_64-pc-windows-gnu`**. Do not set global `RUSTFLAGS`, patch binaries after linking, normalize away differences in the comparison, change Cargo dependency versions, use a custom unchecked linker, or change behavior for MSVC/Linux/other targets. A toolchain/target conflict or an unsupported linker argument is a STOP for an Owner decision, not a reason to weaken byte identity.

Use the existing exact **Rust 1.99.1**, Rust 2024 workspace and single tracked `Cargo.lock`. The report's observed working host was `x86_64-pc-windows-gnu` with `rustc 1.99.1`; re-establish and verify the actual environment. It is permitted to select the installed **1.99.1 GNU host variant** to test this target without changing the pinned `rust-toolchain.toml`. The MSVC-hosted build remains **UNVERIFIED** unless independently run in a genuinely available supported toolchain; never label it as passed by inference. No new Windows SDK or MSVC setup is required by this order.

**3.2 Truthful CLI output errors.** Modify the existing single CLI composition root so unsuccessful `write_all` **or** `flush` for required stdout/stderr output can never be silently ignored. Preserve normal outcomes byte-for-byte: valid preflight produces exactly one success line and exit `0`; authored-input refusal produces safe stderr and exit `2`; unrecoverable output I/O failure produces nonzero **internal failure exit `1`** (not `0` or a false input-refusal success). Do not echo raw OS errors, paths, tokens or authored input to stderr. Any best-effort secondary diagnostic must be fixed and safe; it cannot mask the nonzero exit. Keep all validation policy in domain/application and CLI formatting in the adapter; do not introduce another production composition root or dependency.

Add deterministic **failure-injection** tests for failing `write_all` and failing `flush`, for both accepted-output and refusal-output branches, asserting exit `1` and absence of false success. A small testable writer abstraction local to `crates/solo-cli/src/main.rs` is acceptable; no new crate, external dependency or ornamental domain/application port. Retain real-process smoke tests of ordinary success and refusal, including their exact streams and codes. Where an OS permits a reliable closed-pipe process test, it may supplement but never replace writer failure injection.

**3.3 Safe test scratch.** Replace the unsafe system-temp directory selection in `crates/solo-cli/tests/cli_smoke.rs`. The smoke test may create its own uniquely named child **only under the verified Solo `.temp/` directory or the current Cline-provided task temporary directory**. For this test, prefer Solo `.temp/` derived from the compile-time manifest root, and validate that the effective location is still inside the actual Solo checkout and not redirected by a symlink/reparse point. Require exclusive creation (refuse an already existing child), avoid fixed-name reuse, and clean up **only** the child positively established as created by the current test. Never recursively remove a pre-existing directory or delete the shared scratch root. If an authorized scratch area cannot safely be established, the test fails without falling back to an arbitrary OS temp folder. Do not change the actual CLI's rule that it creates no files in the working directory.

Add a negative regression proving pre-existing/colliding scratch material is not overwritten or removed, and positive proof the successful test leaves no task-owned residue. Keep all build caches, logs and inspection files in the authorized task scratch areas and clean them up when no longer needed; retain neither another checkout nor uncontrolled scratch.

**3.4 Full dependency evidence.** The same `Cargo.lock` contains **11 external packages**, not five: `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `zmij 1.0.23`. Review **all eleven**, including development-only transitives, against the exact locked checksums, source/registry identity, license/attribution, yanked status where verifiable, and applicable published security advisories. Distinguish checked official provenance and advisory evidence from opinion or an unrun scanner. Record the source/date of any advisory data and the exact limitations. If the already available approved `cargo-deny 0.20.2` can run without an unapproved installation, run it and report the exact command/exit code; otherwise mark `cargo-deny` **NOT RUN**, do not claim a scanner pass, and supply a comprehensive manual record instead. Any plausible exploitable critical/high finding or untriaged license/security conflict requires STOP, not a silent exception.

The permitted product dependency graph remains **unchanged**. Do not change `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, crate manifests or any dependency version in this correction. If a dependency actually requires replacement or an additional verifier must be installed beyond permitted tools, STOP and request distinct scope rather than choosing it autonomously.

### 4. Allowed permanent paths and operations

| Path | Permission |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner replaces previous ACTIVE with this exact order; Implementation Agent leaves its installed content unchanged and includes it in final local commit |
| `.cargo/config.toml` | **Create** only the GNU target-specific deterministic linker setting in §3.1 |
| `crates/solo-cli/src/main.rs` | **Modify** output-error handling and localized unit/failure-injection tests only |
| `crates/solo-cli/tests/cli_smoke.rs` | **Modify** scratch location/ownership safety and related positive/refusal tests only |
| `.documents/Reports/execution-report.md` | **Overwrite** the sole canonical report with factual evidence for this run |

No other permanent file edit is authorized. In particular, leave `.documents/WIP/plan-next.md` neutral, ROADMAP/OUTLOOK unchanged, the four Cargo manifests and `Cargo.lock` unchanged, and all README/AGENTS/About/Architecture/SOP/security/licensing files untouched. No new test file, toolchain file, CI workflow, alternate report, schema, browser file, artifact archive, public manifest or provider configuration may be created. `.cargo/` is allowed only as the structural parent for the single listed config file.

**Local operations authorized:** constrained local source edits; trusted and exact Rust 1.99.1/rustfmt/Clippy setup if necessary inside approved task-owned scratch; existing locked dependency retrieval from the trusted official registry; read-only official provenance/advisory lookups; local Cargo commands/tests; task-owned scratch management and cleanup; local Git staging/commits of listed paths; and **one final push** of the completed safe commits to existing `origin/main` after the tests and report are finished. Respect Architect SOP §6B.

**Not authorized:** intermediate GitHub publication/readback, PR, new branch or worktree, second clone, other-repository write, destructive reset/clean/checkout, rebase/history rewrite, force-push, package or toolchain version substitution, new third-party direct dependency, global software mutation beyond approved disposable setup, provider account, secret, DNS/TLS/hosting, Vercel, merge, release, deployment, WIP progression, acceptance or closure. Unlisted permanent writes or external actions are denied.

### 5. Required local proof and falsifiable A1 gates

Run against the **same newly committed implementation source tree**, using the exact compiler version/target, unchanged lockfile and controlled scratch locations; capture commands, exit codes, warnings/skips and source identities. At minimum:

```text
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --workspace --release --locked
cargo metadata --format-version 1 --locked --all-features
cargo tree --workspace --all-features --locked
cargo test --workspace --all-targets --locked --offline    [after trusted caches populated]
cargo build --workspace --release --locked --offline       [after trusted caches populated]
```

Re-run the actual `bootstrap-preflight` success/refusal cases from the prior order, including `entity-7` plus `https://EXAMPLE.com:443/alpha`, invalid `Main`, malformed or unsupported base, missing/duplicate/unknown flags, no side-effect, and entity-ID/base independence. Re-run the real Cargo dependency graph tests with deliberately forbidden edges and the guard-disabled negative test; ensure no regression and no expansion into `R01.B`.

**Independent byte-identity proof:** perform two fresh `cargo build --workspace --release --locked` runs from the **same single checkout**, each with a separate clean task-owned Cargo cache and separate target directory, with the same verified GNU Rust 1.99.1 host triple and same build environment/features. Do not inject the timestamp-suppression flag through an ephemeral environment override: the tracked `.cargo/config.toml` must make the normal command deterministic. Capture the SHA-256 and size of both shipped `solo-cli.exe` binaries; hashes must be **exactly equal** (no PE-header normalization and no ignored bytes). Capture `Cargo.lock` SHA-256 and unchanged Git blob. Perform a deliberate locked-file refusal in disposable scratch without altering the real workspace or tracked lock; confirm a missing/changed lock cannot pass `--locked`. A mismatch, warning that implies an unreviewed non-deterministic linker path, or inability to reproduce is a failed gate.

**CLI-output negative proof:** using deterministic injected writers, force `write_all` failure and `flush` failure separately for stdout success and stderr refusal. Assert **exit `1`** in each case; accepted/refused normal paths still exactly match their old outputs and codes, and no unfiltered I/O error reaches public diagnostics. Tests must fail if output-error handling is disabled.

**Scratch negative proof:** verify only task-owned, exclusive child directories under an allowed scratch root are created/removed; prove an existing/colliding child is preserved and the test refuses it, including no unsafe recursive cleanup. Check repository and task scratch state after tests. No inferred ownership based solely on a familiar filename.

**Supply chain:** produce an exact eleven-row inventory with version, checksum/source, license, attribution requirements, security/advisory review source/date, triage and uncertainty, including transitive and dev-only packages. Confirm the locked graph contains no new direct or transitive packages. A `NOT RUN` scanner must remain honestly labeled; it cannot be described as a clean advisory scan. A known unaddressed material finding fails the gate.

| Gate | Passing evidence | Mandatory refusal or failure sensitivity |
| --- | --- | --- |
| **C-01 Source safety** | Exact baseline, Owner-installed ACTIVE, one checkout, only authorized paths | Mismatched source, unowned/untracked material, extra WIP file or second checkout stops |
| **C-02 GNU reproducibility** | Default tracked-config builds twice; same SHA-256 binary, pinned compiler/target, no lock drift | Raw binary mismatch, linker incompatibility, missing/changed lock or unreviewed flag override fails |
| **C-03 Output truthfulness** | Normal process success/refusal preserved; each injected write/flush error returns internal exit `1` | Swallowing I/O failure, false exit `0`/`2`, or leaking unsafe error text fails |
| **C-04 Scratch confinement** | Exclusive authorized child, safe cleanup, no leftover task garbage | OS-temp fallback, symlink escape, collision deletion or unsafe cleanup fails |
| **C-05 Dependency completeness** | All 11 exact locked crates individually reviewed; material risk resolved | Missing crate, unverifiable required fact, untriaged high/critical finding or license conflict fails |
| **C-06 No R01.A regression** | All applicable fmt/lint/test/build, CLI positive/refusal, graph-negative and offline checks green | Disabled tests, suppressed warnings, failing refusal or unauthorized dependency edge fails |
| **C-07 Final evidence** | One overwritten canonical report, correct WIP, local clean tree, one final successful push | Premature push, undocumented skip, extra report, scope drift or self-closure fails |

The complete Stage 01 exit, comprehensive `R01.B` source-level import checker and `R01.C` CI/production-browser suite remain deferred and **not passed**; neither may excuse a defect in the code changed here. MSVC verification, Node/browser, full publication/routing, C1–C4, F01–F17 and RG01–RG11 remain **UNVERIFIED/NOT RUN or NOT APPLICABLE** as appropriate. `COMPLETE` by the agent is factual run status, never independent `ACCEPT`.

### 6. Local-only execution, final report and handback

Perform all implementation and proof **locally**. Stage only the allowed files, commit corrected code/configuration locally, and run final gates against that committed source identity. Overwrite the existing `.documents/Reports/execution-report.md` with one concise factual account (including the table of eleven external packages), then commit the report **locally**. Do not put the report's own final commit SHA into its contents. Confirm locally that WIP has exactly four files, ROADMAP still contains R01.A/R01.B/R01.C, NEXT is neutral, ACTIVE still contains this work order, no unauthorized path changed, the working tree is clean, and task-owned scratch is cleaned or explicitly justified.

After completing that local work, make **one final push** of authorized commits together to the existing `origin/main`. Confirm the result from the push; do not create a PR/branch, publish intermediate work or perform a second publication ceremony. Provide the exact source commit/tree and final report commit/tree and final push result in the chat handback. If the push or any mandatory gate fails, preserve evidence and report `STOPPED`/`FAILED` accurately; do not publish unsafe incomplete code merely to force a green status. Publish a factual stopped report at end only when safe under the SOP; if it cannot safely be published, explain the blocker rather than rewriting history or destroying work.

The report must distinguish **agent-observed local command outputs** from independent Architect review, including the exact Rust target; list every changed file and command/exit, raw binary hashes, negative cases, scanner status, advisory/provenance limitations, remaining unsupported platforms, temp cleanup and any `STOP`. No duplicate report or archive. The Chief Architect independently reviews the exact published revision and returns `ACCEPT`, `REVISE` or `STOP`. Only later separate Owner authorization can clear ACTIVE, consume `R01.A` or advance NEXT.

### 7. STOP conditions

STOP upon wrong repository/HEAD/worktree, conflicting order, unknown or unowned data, any unlisted permanent edit, unsafe path/link/collision, untrusted Rust/dependency supply, missing required compiler or unsupported GNU linker flag, failed byte identity, an output-refusal regression, unsafe test cleanup, a known material unresolved advisory, inability to give complete required evidence, or need to change any accepted contract, toolchain pin or package graph. Do not weaken exact binary comparisons, skip failure injections, choose another host and call the GNU target passing, suppress failing checks, or reinterpret an open ROADMAP package as completed. Report the smallest precise decision needed and safe local resumption point.
