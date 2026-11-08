# Foundation Solo — Active Work Order

## SOLO-WO-032 R01 — Stage 01 R01.B Root-Anchoring, Scratch-Ownership and Discovery Correction

## Approved Scope — OWNER-ISSUED

### 1. Objective and reviewed authority

Correct **only** the outstanding defects identified by the Chief Architect's independent `REVISE` review of `SOLO-WO-031 R01` at the exact GitHub revision below. This remains the same unfinished **Stage 01 `R01.B`** obligation; it is not a new product feature, a replacement architecture, or permission to close `R01.B`:

1. `FsReader::confine` checks path components **below** the package root, but not the unresolved package-root entry itself (or relevant ancestors between the verified Solo checkout and that entry). Canonicalizing both a redirected root and its redirected contents can make an unowned source tree appear confined. A package-root junction/symlink must fail **before** its contents are opened, even if its destination is another directory inside the same Solo checkout.
2. The new `Scratch` test helper creates its purportedly unique fixture directory with `create_dir_all` through unverified `.temp/architecture-fixtures` ancestors, and its `Drop` calls recursive `remove_dir_all` without establishing exclusive ownership. It must never traverse a redirected scratch ancestor, reuse an existing fixture or recursively remove foreign material.
3. The `rust_files_under` discovery loop uses `entry.metadata().is_ok_and(...)` and `path.is_dir()` such that metadata/entry inspection failure can be treated as an ordinary nonredirected, non-directory entry and silently omitted (notably when the filename is not `.rs`). Every uninspectable relevant entry must produce an actual refusal.

Preserve the already corrected `include!`/`#[path]`/default-module confinement, `cfg_attr` refusal, `--fixture` outcome semantics, the full architecture-checker matrix, the exact `syn =3.0.9` dev-only dependency, the four-crate workspace and all production behavior. Do not start `R01.C`, Stage 02+, CI/browser/HTTP, provider work, release or deployment. The previous agent's `COMPLETE` report does **not** mean `R01.B` is Architect-accepted; the reviewed disposition was `REVISE`.

| Reviewed identity | Exact value |
| --- | --- |
| Repository and branch | `provelopment/provelopment-foundation-solo` / existing `main` |
| Starting HEAD | `c2ab315142f1b1f9cfe45018197d2cfaf934a60e` |
| Starting tree | `13000e5bdf8fec4c2cc8a4265045f2ba12e0835b` |
| Last corrected source commit | `da96c5142b801fdf18e585240cb6f8723f811b1c` |
| Canonical report blob | `.documents/Reports/execution-report.md` — `5bbd0396d92dfc4e26f8ef081d5f35581d0347b5` |
| Previous committed ACTIVE blob | `.documents/WIP/workorder-active.md` — `3722a6885107b69124b0a64fb1ee64a046cf6e39` (`SOLO-WO-031 R01`) |
| Source checker blob | `crates/solo-cli/tests/architecture_checks/source.rs` — `141261fd7005931522330c1ed0a195583fd3763a` |
| Real-fixture tests blob | `crates/solo-cli/tests/check_architecture_fixtures.rs` — `7d20147ed22101aeda97258caeafe6f590957ba4` |
| Shared fixture/checker module blob | `crates/solo-cli/tests/architecture_checks/mod.rs` — `adf2f0be7b9bf19102b8d1926d5dacb46ce428ec` |
| Graph module / checker process blobs | `graph.rs` — `e328dd6a9f54c47e3eae60053446d52edacc74b7`; `check_architecture.rs` — `0bad0e027f41b6cf6029df38174fc15531c187da` |
| WIP reference blobs | ROADMAP `d01780d4af965b342521e3594f7a1e4a7dfc551d`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d` |

Read and obey `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, both SOPs, all **four** WIP files and the one canonical execution report in their mandated order, then relevant source/tests. Architect SOP §§6, 6A, 6B, 7A, 7B, 8 and Implementation SOP §§4–5C govern authority, one checkout, source evidence, scratch, Git and final reporting. This order does not amend them. The Owner must install this exact file in `.documents/WIP/workorder-active.md` and issue the matching `SOLO-WO-032 R01` handoff before implementation.

### 2. Local preflight — stop on drift

Operate only in the **one existing Solo checkout**, expected locally at `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; establish its real root from local Git evidence, not from the expected path's spelling. **Before editing, installing, fetching or executing the checker**:

- Verify the sole `origin` is `provelopment/provelopment-foundation-solo`, branch `main`, HEAD/tree exactly as above, tracking and remote state, exactly one worktree/checkout, no extra branch checkout, nested repository or stash, and no unsafe reparse-point/symlink transition. Inspect tracked, staged, unstaged, untracked and ignored state. Do not reset, clean or discard unrelated or Owner-owned material.
- The **only** permitted pre-existing tracked difference is the Owner's entire replacement of the prior ACTIVE with this **complete** `SOLO-WO-032 R01`, marked `OWNER-ISSUED`. Verify exact Owner handoff and preserve the installed file byte-for-byte in the implementation commit. Unexpected content or any second change means **STOP**.
- `.documents/WIP/` must hold exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md` and no other entries/subdirectories. ROADMAP retains `R01.B` and `R01.C`, not `R01.A`; NEXT is a neutral single heading; OUTLOOK is unchanged. **Do not edit ROADMAP, NEXT or OUTLOOK. Do not clear ACTIVE.**
- Verify the reviewed source, report and WIP blobs above. Also verify `README.md` `3f3005d680d9ed182a02e574744054aaae55deb2`, `AGENTS.md` `32562b6730e26a37b9febae63efd7d5c6a5866ac`, About `1d80f62769053ba4e6ca51adf2d11c68e724c30e`, Architecture `03e8923f834b50fc28e1b56e70f73017525cfa5b`, Architect SOP `8760be323b6ed85695a9dcffd3e52f94628f0f07`, Implementation SOP `a577116082bf8133a6056667aa52fe4ffc2ae94d`, root `Cargo.toml` `14505be49a281ba0495eabea5ca386548cc36347`, `Cargo.lock` `05a02666894f4f0bf5528acf118a049e5edb853e`, `crates/solo-cli/Cargo.toml` `1ca5e114a6e034aae386ea9d7adab3a81b4ae8bc`, `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`, `.cargo/config.toml` `74a4fb6c0f595ebc64841b26e7961967294f7d3e`, `workspace_graph.rs` `8fa4b4d0bde840782abb0c9e24c84bac8f06c947`, `solo-cli/src/main.rs` `6faa9fae29351123a49d45c5637ac4fbb1b615c8` and `cli_smoke.rs` `921cbbe91fb0a75919014e91455fbc080dfd111c`.
- Identify the installed exact Rust `1.99.1` host/target and existing locked registry evidence. Conditional installation into verified **owned disposable scratch only** is allowed for the unchanged pinned toolchain/lock graph. Do not install unapproved packages or change host-wide settings.

### 3. Required technical correction — anchored effective-source ownership

Correct the **real** source checker in `crates/solo-cli/tests/architecture_checks/source.rs`; do not add a second checker or a fixture-only substitute.

**A. Verify the package root as an original filesystem entry.** Derive a trusted checkout/workspace anchor from the verified existing Solo tree and real, locked Cargo metadata/manifest identities; do not treat a self-declared canonical package root as independent proof of its own ownership. Establish that each of the four workspace packages occupies its exact expected direct path `crates/<package>` under that anchor and that the declared source target belongs there. Before using any package root, inspect its **unresolved directory entry**, plus every relevant path component from the verified checkout to that entry (including `crates`). On Windows reject `FILE_ATTRIBUTE_REPARSE_POINT` (`0x0400`) and on Unix reject symlinks; refuse non-directory, uninspectable, missing, ambiguous and unexpected redirected components. The checkout anchor itself must be validated against the already verified Git root; do not follow a redirected checkout entry and silently declare its target to be the permitted workspace. A redirect into a different folder **inside** the checkout is just as forbidden as one outside it. Do not rely on `canonicalize(root)` and `canonicalize(child)` both resolving consistently; that is the defect.

**B. One fail-closed source gate.** Keep `SourceReader` and in-memory fixture support, extending `UnitSpec`, `check_packages` or the narrowly related helper interfaces **only as needed** to carry the authoritative checkout anchor. Apply original-entry checks **before** canonicalization, read, metadata that follows links, module traversal or content parsing for target roots, library public surfaces, `include!` (including `.inc`), `#[path]`, default `mod`, glob-reexport source reading and discovery. An inaccessible entry or failed canonicalization is a violation, never a silent return/skip. Require lexical **and** effective ownership against the correct checkout/package identity and refuse any link/reparse component. Do not introduce a general OS-temp fallback or a second root. Retain the explicit limitation that this pre-use validation is not a proof against hostile TOCTOU races; do not claim race freedom without OS-supported evidence.

**C. Discovery metadata must fail closed.** Refactor `rust_files_under` and its traversal to use **unresolved** entry metadata with explicit `Result` handling. Do not use `entry.metadata().is_ok_and(...)`, `path.is_dir()` or `filter_map`/`flatten` in a way that converts inspection failure into `false` or drops a possibly relevant entry. For each visited directory entry, either establish its type and allowed, nonredirected location or record a bounded `UnreadableSource`/`ModuleSourceEscape` violation; an entry with a non-`.rs` filename is **not** exempt from metadata/error handling. For directories selected for traversal, validate confinement/entry identity before descending. The existing deliberate exclusions for known build output and hidden directories may remain only when the entry's identity can first be inspected safely; do not hide an unreadable or redirected directory behind an ignored-name shortcut.

**D. Retain prior rules.** Preserve conditional `cfg_attr(path = …)` rejection (including nested, duplicate, malformed and inline cases), real graph enforcement, aliases/re-exports, `syn` AST checks, the full 23-forbidden-fixture / five-legal-twin matrix, the custom test-only process status contract (`0` legal, `1` refused, `2` usage, `3` unexpected acceptance) and audited macros. No disabling, looser fallback, runtime dependency or product change. If the existing design cannot safely support an anchored checker inside the authorized files, **STOP**, describe the exact additional path required, and do not improvise broader scope.

### 4. Required correction — exclusive scratch-fixture ownership

In `crates/solo-cli/tests/check_architecture_fixtures.rs`, replace the unsafe `Scratch::new`/`Drop` ownership model **without modifying the checkout's actual `.temp` entry**:

- Establish from the verified Solo root that `.temp` is the direct authorized root and is a **real, nonredirected directory**. Inspect the unresolved `.temp` entry (Windows reparse attribute / Unix symlink) **before** any `create_dir_all`, `canonicalize` or write. When absent, use exclusive `create_dir` and re-inspect; when already present, inspect and preserve it. Never blindly create through a pre-existing redirect.
- Do the same for the fixed `.temp/architecture-fixtures` parent: if missing, create safely and inspect; if it exists, do not overwrite or follow a redirect. Do not remove shared parents containing foreign material or treat an existing parent as task-owned simply because its name matches.
- Create each task-specific final fixture directory with exclusive `create_dir` (never `create_dir_all` of that final entry), refuse `AlreadyExists` even if it looks empty, and positively record only the paths this test created. Every nested fixture write/dir/link must remain inside that exclusively owned, previously validated root; reject any pre-existing collision or injected link at a prospective write destination. Keep real Windows junction and Unix symlink tests; if a genuine redirect cannot be created, the mandatory redirect gate is **UNVERIFIED / STOP**, never fabricated PASS.
- Teardown must remove the test's **own links first**, using the operation that removes only the link; then remove positively owned files and **empty** directories from deepest to shallowest using non-recursive operations. **Do not call `remove_dir_all` on a shared root, a redirect, a colliding directory, an unverified fixture or a directory whose contents may be foreign.** Preserve unexpected content; record/raise cleanup failure rather than silently claiming full cleanup. No destructive Git commands, privilege changes or deletion of unknown Owner material.
- Existing genuine redirected `include!`, `#[path]`, default-module, conditional-path, missing-source and legal-loading regressions must still execute and leave their target bytes intact. They should use the revised owner-safe fixture lifecycle, not a parallel unsafe helper.

### 5. Mandatory adversarial and refusal tests

Add ordinary `#[test]` cases in the authorized test file, exercised by `cargo test --workspace --all-targets --locked`. Use the **same production checker** `FsReader`/`check_unit` (or real-tree entry path where relevant), not mock logic that returns a preselected error. All fixtures must be owned in the authorized scratch area and cleaned safely.

1. **Genuine redirected package-root entry.** In a safely owned fixture subtree under Solo `.temp/`, create a candidate package-root junction (Windows) or symlink (Unix) to another directory, ideally still inside the same verified checkout. Place parsable Rust with a sentinel in the target. Show that the redirected root and source both canonicalize to internally consistent paths (the former defect's false-positive condition), yet the **unresolved** root entry is detected and the checker refuses **before consuming its source**. Assert link and target material survive unchanged. Do not replace or redirect the real `crates/` or real `.temp/` entries.
2. **Redirected package ancestor.** Exercise a redirected directory component between a verified fixture checkout anchor and package root (e.g. fixture `crates` entry). It must be refused even when the package-root entry itself looks like an ordinary directory after resolution. Verify preservation and confined cleanup.
3. **Scratch ancestor/collision preservation.** Exercise a pre-existing final fixture directory with a sentinel and show that exclusive creation refuses and never deletes/rewrites it. In a separate positively owned test subspace, create a redirected synthetic scratch ancestor pointing at another in-checkout directory and prove the **same** scratch-root validation rejects it **before** writing or deleting at its target. Do not manipulate the real shared `.temp` root. A test that merely checks a boolean on already canonicalized metadata is inadequate.
4. **Discovery inspection failure.** Test the exact production discovery routine (or a narrowly injected entry-enumeration/metadata seam used by it) with a metadata/entry-inspection error for a non-`.rs` name, and assert a real error-category refusal rather than an empty inventory or success. Cover a redirected discovery directory and preservation. Do not rely on flaky timing or OS permission tricks; where deterministic fault injection is necessary, use the **same** error-handling path as the real filesystem walker.
5. **Positive and regression cases.** A genuine nonredirected package root and nested source work; in-package `include!`, `#[path]`, default modules and harmless `cfg_attr` continue to pass. Run all original negative categories, genuine junction/source escapes, legal twins, checker-bypass self-test and normal process CLI outcomes. Retain verification that unexpected fixture acceptance is exit `3`, not refusal `1`.

The checker and fixture safety gates must not claim PASS merely from a Rust `cargo test` process exit when the required *adversarial observation* did not occur. Report genuine link type, path relationship, exact refusal class, target preservation, cleanup behavior and any unavailable OS-specific branch.

### 6. Exact permanent-path and operation ledger

**Only** these permanent Solo paths may be changed:

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Preserve **Owner-installed** `SOLO-WO-032 R01` text unchanged; include it in the implementation commit |
| `crates/solo-cli/tests/architecture_checks/source.rs` | Correct checkout/package-root anchoring, component validation and fail-closed discovery; narrow associated helpers |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Implement exclusive non-destructive fixture ownership; genuine redirected-root/ancestor/collision and metadata-error regressions |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | Only if necessary to propagate an anchored `UnitSpec`/fixture configuration, or add tightly related negative/positive fixture wiring; preserve existing matrix |
| `.documents/Reports/execution-report.md` | Overwrite the **single** canonical current-run report with factual results |

All other permanent paths are **read-only**: all product Rust modules, `graph.rs`, `check_architecture.rs`, `workspace_graph.rs`, `cli_smoke.rs`, all manifests, `Cargo.lock`, `.cargo/config.toml`, `rust-toolchain.toml`, root README/AGENTS, About, Architecture, SOPs, ROADMAP/NEXT/OUTLOOK, `.gitignore`, community/security/licensing files, CI/browser, and every other repository. No new tracked test/fixture files, new WIP file, secondary report or alternate checker. If a sound correction requires a different permanent file or dependency, **STOP** and present the smallest proposed permission change; do not silently extend the ledger.

**Allowed:** read-only local preflight, edits only above, exact pinned toolchain and **already locked** dependencies in validated task scratch if missing, locally executable focused/full checks, independently fresh Cargo build caches/targets in `.temp/` or the actual Cline-supplied task scratch, normal local staging/commits of exactly authorized paths, **one final push** of fully completed safe commits to existing `origin/main`, and readback verification. Follow Architect SOP §6B (do not invent a parallel Git procedure). **Denied:** intermediate push, GitHub file mutations, cloning/worktrees/branches, PR/merge, destructive reset/clean/rebase/force-push, other-repo writes, new dependency or version, global OS modification, secrets or provider access, release/deployment, any R01.C implementation, and WIP administrative closure. Unlisted operations are not authorized.

### 7. Verification gates and publication procedure

Complete the focused correction and tests locally. Commit the corrected **source/tests and Owner-installed ACTIVE** locally; run all verification on that exact committed source identity. At minimum record direct invocations/exit codes for:

```text
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo test --locked -p solo-cli --test check-architecture
cargo test --locked -p solo-cli --test check-architecture -- --self-test
cargo build --workspace --release --locked
cargo metadata --format-version 1 --locked --all-features
cargo tree --workspace --all-features --locked
```

Run the genuinely redirected-root/ancestor, scratch-collision and metadata-error cases as observed tests, plus every original/new `--fixture <id>` with `outcome=refused`, the **correct expected category** and checker status `1`; an unexpected acceptance must show status `3` and fail the self-test. Re-run the real `workspace_graph` and `cli_smoke` suite and CLI process success (`entity-7`, `https://EXAMPLE.com:443/alpha` → `0`, exact canonical stdout) and refusal (`Main` → `2`, safe exact stderr). Run locked **offline** build/tests once trusted caches are populated. Verify no unauthorized code, `Cargo.lock`/manifest/toolchain/graph drift.

**Reproducibility:** perform two independent fresh-registry-cache **and** fresh-target-directory `x86_64-pc-windows-gnu` Rust `1.99.1` release builds from this same committed identity under the unchanged tracked `.cargo/config.toml`; compare exact **raw** SHA-256 and byte length with no header normalization. The prior report established that even an **empty** environment `RUSTFLAGS` can shadow the tracked target-specific linker flags; ensure `RUSTFLAGS` is **unset/absent**, not merely set to an empty value, and verify any other flag overrides cannot mask the tracked deterministic-link configuration. Record environment precondition and effective command evidence. Do not silently add an ephemeral linker flag or compare only normalized PE headers. If GNU/toolchain or required tests are unavailable, mark **UNVERIFIED/FAIL** and STOP rather than reusing the last report's hashes.

| Gate | Required PASS evidence |
| --- | --- |
| **G-01 — Authority and scope** | Exact starting Git revision/tree, verified one checkout and Owner-issued ACTIVE, five-path permanent ledger, four unchanged WIP slots, no unrelated changes |
| **G-02 — Package-root anchoring** | Original entry and all relevant checkout-to-package components inspected before source use; genuine redirected root and ancestor **inside the same checkout** rejected, target bytes preserved |
| **G-03 — Discovery fail-closed** | Every metadata error is a violation (including non-`.rs` entries), every redirect refused before descent, genuine/legal positive source continues to pass |
| **G-04 — Scratch ownership and cleanup** | Unresolved `.temp` and fixture parent validated; collision preservation, exclusive child create, actual redirected synthetic parent refused, only owned link/file/empty-directory cleanup, zero orphan task residue or truthful failure |
| **G-05 — Checker regression** | All real targets checked; existing 23 negative fixtures, five legal twins, conditional-module/source redirects, exit status mapping, CLI and graph tests pass without weakened policy |
| **G-06 — Pinned toolchain and reproducibility** | fmt/strict Clippy/test/build/metadata/tree/offline PASS, genuine GNU fresh builds raw byte-identical, flag override absence evidenced; `Cargo.lock` and product artifacts remain unchanged |
| **G-07 — Factual report and publication** | One report; exact changed paths and results; task scratch cleared; no secrets; safe clean local tree; **one** final push and exact remote HEAD/tree verified |

**Report:** Overwrite only `.documents/Reports/execution-report.md` with the session date, work-order ID/edition and Owner handoff, verified root/branch/starting HEAD/tree, corrected implementation source commit/tree, original versus effective root/ancestor test identities (repository-relative), refusal/positive results and categories, metadata failure and collision observations, link-target preservation, owned scratch inventory/cleanup, actual check commands/exits, reproducibility caches/flags/raw hashes, unchanged blob audit, WIP state, host and security limitations, gate-by-gate `PASS`/`FAIL`/`UNVERIFIED`, final status `COMPLETE`/`FAILED`/`STOPPED`, and final one-push verification. Do **not** embed the report's own final commit SHA in the report; the SHA and report link belong in the agent's chat handback. Never include secrets, private absolute Owner paths or unredacted hostile payloads in the public report.

The final sequence is **preflight → allowed edits → focused negatives → local source/ACTIVE commit → full verification on that commit → canonical report overwrite/commit → diff and clean-tree/scratch audit → one final push to `origin/main` → remote identity verification**. No intermediate online publication, releases, self-acceptance or retrospective ROADMAP pruning. The Chief Architect separately reviews the published result and returns `ACCEPT` / `REVISE` / `STOP`.

### 8. Mandatory STOP conditions

STOP and preserve unrelated data if HEAD/tree/Owner handoff is wrong, a redirection or metadata failure can still be ignored, an ancestor's identity is unprovable, a fixture might write/delete anything not positively owned, genuine adversarial link creation fails, a mandatory test or raw reproducibility comparison fails, checkout/WIP changes exceed the ledger, a dependency/toolchain or permanent path must expand, or Git publication/cleanup is unsafe. Report truthfully under the one canonical report procedure when safe; do not publish known unsafe incomplete code to manufacture a green status. `R01.B` remains in ROADMAP and ACTIVE is not cleared by this run.
