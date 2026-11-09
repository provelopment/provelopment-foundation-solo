# Execution Report

**Work order:** `SOLO-WO-047 R02` — R02.B third bounded slice: audited native `SourceRead`, no-follow confinement and physical identity.
**Result:** **STOP** at the §5 hosted native-proof gate. The implementation is complete, committed and clean under every static gate on both hosts, but the mandatory test step is red on Ubuntu and Windows and this environment cannot read the job logs, so the individual failing case is **UNVERIFIED**. No security guarantee is claimed and every authority flag remains `false`.

## 1. Revisions and what each one proves

| Revision | Content | Hosted outcome |
| --- | --- | --- |
| `dc17adb` | Owner-issued R02 installed byte-identically in the ACTIVE slot | Green — run `42580264429`, Ubuntu job `129037045542`, Windows job `129037045169` |
| `58793cac` | Native boundary, public wrapper, inward port and use case, dependencies, lockfile, graph/workflow guards, real-OS tests | **Red at step 16** `Test (workspace, all targets, locked)` — run `42583699542`, Ubuntu job `129048280534` (13 steps green before it), Windows job `129048280760` (12 steps green before it) |
| `e0c81d2` | Fake-port tests, load-bearing unsafe guard, architecture paragraph | Typechecked and lint-clean; runtime behaviour **unverified** |

Green on **both** hosts at `58793cac`, before the failing step: locked dependency fetch, the pinned `cargo-deny 0.20.2` four-category gate with a fresh RustSec database, the negative advisory refusal, the package-set assertion, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`. That last one matters: it compiles every new library, binary and test target on Ubuntu and Windows with warnings denied, so this is not a build, lint, licence or policy failure on either host.

## 2. What the change actually contains

* **`crates/solo-adapters/src/source_read_native.rs`** — the single audited native file. Linux makes one `openat2` call with `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_MAGICLINKS | RESOLVE_NO_XDEV`, so confinement, no-follow and no-cross-device are decided by the kernel inside the same operation that opens the file, and `ENOSYS`/`EINVAL` refuse rather than fall back to a weaker open. Windows opens one single relative name at a time beneath the previous directory handle with `NtCreateFile` and `OBJ_DONT_REPARSE`, then reads reparse tag, type, volume serial, 128-bit file ID, length, link count and both change timestamps from the handle it actually opened. Nine `unsafe` blocks, each with an adjacent `// SAFETY:` rationale, and one justified `zeroed` helper so no struct initialisation is unsafe at its call site.
* **`crates/solo-adapters/src/source_read.rs`** — safe wrapper holding the root capability; refuses a non-`pages/` namespace, a separator, an empty segment, a link, a non-regular leaf, a hardlink (`links != 1`) and any before/after difference in identity, length, link count or timestamps (`SNAPSHOT_CHANGED`).
* **`crates/solo-application/src/source_read.rs`** — the outbound port and the read use case. Two operations, so a candidate can be identified without loading its body; JSON is probed and read first, a located Markdown body is never read while JSON exists, a probe refusal is final, two members resolving to one physical object are refused from the probes alone, and the accepted `select_provisional_source` decides. `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` and `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` are both `false`.
* **`crates/solo-adapters/tests/source_read.rs`** — the real-filesystem matrix: JSON leaf, Markdown leaf, oversized Markdown never read, oversized JSON refused, empty JSON, two members as one object, hardlinked leaf, leaf link, intermediate link, root link, absent root, directory as leaf, missing intermediate as absence, and a rewrite between reads.
* **`crates/solo-application/tests/source_read.rs`** — fake-port tests that assert the recorded call sequence, so "Markdown was never read" is proven rather than asserted.
* **`crates/solo-cli/tests/unsafe_boundary.rs`** — the load-bearing guard: strips comments, strings and raw strings, refuses `unsafe`, a native import or an extra allowance in any other first-party file, requires one `// SAFETY:` per block, and proves itself against injected violations.
* Dependencies: `rustix =1.1.5` (`fs`, `std`, Linux only) and `windows-sys =0.61.2` (six features, Windows only). Measured real graph: **22 packages, 34 distinct edges**; `windows-sys` appears once; `itoa` was held at `1.0.18`; `deny.toml` needed no change because both licences already satisfy the existing `Apache-2.0`/`MIT` alternatives.

## 3. Why this is a STOP and not a correction

The mandatory gate is red on both hosts, and the order makes that a STOP unless there is an **evidenced** in-scope correction. I do not have the evidence, and I will not invent it:

* `GET /repos/provelopment/provelopment-foundation-solo/actions/jobs/129048280534/logs` returns **403**.
* `GET /repos/.../actions/runs/42583699542/check-runs` returns **404**, so there are no annotations to read either.
* Both hosts fail at the **same step** with **different** platform code, and every step that could have caught a build, lint, format, licence, source-policy or package-set problem already passed on both.

So the failing assertion is a **runtime** result I cannot observe. Guessing which case it is and pushing speculative edits would be exactly the behaviour the order forbids — publishing unproven security code and calling it progress — and the STOP conditions name "a mandatory red CI gate without an evidenced in-scope correction" and "skipped mandatory host test" directly. Every authority flag therefore stays `false`, and no guarantee from §4 of the order is reported as met.

## 4. Environment limitation, disclosed

This run installed the sanctioned task-local pinned toolchain (`rustup`, official distribution, `rustup-init.exe` SHA-256 `f6289656ce42e1099f3cbc6f10ef6321dfde54773faf599dd36fd3b1f07d9103` verified against the published checksum, isolated `RUSTUP_HOME`/`CARGO_HOME` under `.temp/`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)` — the same build CI and the standard library documentation use). With it:

* `cargo check --locked --target x86_64-unknown-linux-gnu --workspace --all-targets` — clean.
* `cargo check --locked --target x86_64-pc-windows-gnu --workspace --all-targets` — clean.
* `cargo clippy --locked --target … --workspace --all-targets --all-features -- -D warnings` — clean on both targets.
* `cargo fmt --all --check` — clean.
* `cargo metadata` measured the graph at 22 packages / 34 edges.

What it could **not** do is **execute** a test, because this machine has no working linker: the bundled GNU `dlltool` needs an `as` that the toolchain does not ship, there are no MSVC build tools, the LLVM-flavoured target lacks a UCRT, and there is no WSL or container runtime. Tests were therefore typechecked and linted, never run — which is precisely why the hosted red result cannot be narrowed from here.

## 5. The smallest Owner decision this STOP requires

**Provide the failing test name(s) from the two job logs**, or grant this environment read access to Actions logs. Either is enough: with the name, the correction is small and in-scope, no requirement has to be relaxed anywhere, and it can be made under this same order. Nothing else is needed, and no dependency, guarantee or scope change is being requested.

If log access cannot be granted, the alternative is to reissue with a one-line instruction that the test step may be run with `--no-fail-fast` and its output uploaded as an artifact, which makes the diagnosis readable without exposing anything.

## 6. Preserved state

* ACTIVE still holds the Owner-issued R02 (`4cb50914`); a STOP does not consume or neutralize authorization.
* NEXT is neutral at the approved 49-byte heading (`92fa562b`) and ROADMAP is byte-identical (`18975332`), so `R02.B`, `R02.C` and Stage 02 all remain open and no item was consumed.
* `deny.toml`, `crates/solo-cli/tests/architecture_checks/source.rs`, `crates/solo-cli/tests/check_architecture_fixtures.rs`, the root `Cargo.toml`, `rust-toolchain.toml` and `.cargo/config.toml` are byte-identical to the required starting state.
* The accepted `source_locator`, `source_selection`, `publication_manifest` and `source_body_preflight` modules and their tests are byte-identical, so both accepted R02.B slices are untouched and the earlier R01 STOP history is preserved.
* Path ledger actually used: ten paths in `58793cac`, three in `e0c81d2`, one in `dc17adb`, and this report. Every one is inside the order's twenty-one-path ledger; no path outside it was created, modified or deleted.

## 7. Cleanup disclosure

Removed: the untracked `target/` build tree this run created at the repository root (781.6 MB — the repository does not ignore `target/`, so it was removed rather than left as dirt), plus the throwaway `.temp/target2` and `.temp/linklib` experiment directories.

Retained with an explicit trigger: `.temp/toolchain` (the checksum-verified pinned 1.99.1 toolchain) and `.temp/target` (its build cache), because the follow-up correction needs them and re-installing costs a large download. **Cleanup trigger: delete both when this slice is accepted or abandoned.** The four pre-existing `.temp/` folders from earlier orders (`architecture-fixtures`, `rt`, `w032`, `w033`) belong to those orders and were left untouched.

Nothing else task-owned remains: no fixture directory in the repository, no temporary source file and no untracked path outside `.temp/`.
