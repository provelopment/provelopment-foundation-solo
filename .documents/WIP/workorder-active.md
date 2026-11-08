# Foundation Solo — Active Work Order

## SOLO-WO-033 R01 — Stage 01 R01.B Discovery-Root Pre-Enumeration Correction

## Approved Scope — OWNER-ISSUED

### 1. Objective and reviewed starting identity

Correct **only** the residual fail-closed directory-discovery defect from the Chief Architect's independent `REVISE` of `SOLO-WO-032 R01`. This is a narrow corrective execution of the **same unfinished R01.B obligation**; it neither reopens accepted R01.A nor implements R01.C. The existing architecture checker, production CLI, graph guard, source confinement, conditional-module refusal, and exclusive scratch ownership are preserved.

**Specific defect.** `crates/solo-cli/tests/architecture_checks/source.rs` currently initializes `rust_files_under` with the package root queued and calls `DiscoveryFs::list(&directory)` immediately at the start of the loop. Its original-entry redirect inspection occurs only later, for encountered child directories. A junction/symlink at the **initial package root** (or an ancestor) may therefore have its contents listed or inventoried before this discovery routine refuses it. The overall real-tree checker has other defenses, but this discovery path **independently** must validate the directory before enumeration; later refusals do not make prior traversal safe.

**Outcome required:** verify the unresolved original entry chain and checkout/package containment of **every dequeued directory, including the first package root**, before `list`, and return a stable bounded violation without listing or descending when an entry is redirected, uninspectable, missing, wrong-kind, or outside its trusted boundary. Prove the ordering with genuine redirected-root and redirected-ancestor tests that call the **production `rust_files_under` routine** directly, not just `check_unit`; preserve legal discovery and the prior negative matrix. Never mutate the actual `crates/` or `.temp/` entries to fabricate a test.

| Reviewed identity | Exact value |
| --- | --- |
| Repository / branch | `provelopment/provelopment-foundation-solo` / existing local `main` |
| Starting HEAD | `9045b1b84ed0a2164564c4aee24d61cc830bbed3` |
| Starting tree | `7c97e423156b09e35785b5d73d20f479ca9b8890` |
| Last source commit | `0ee1e5f1a8062623b47655040d8773db0b7c9db7` |
| Current canonical report blob | `.documents/Reports/execution-report.md` — `e7f73f3744f3b739921e43c775cf8f597bc6d4dc` |
| Current ACTIVE blob | `.documents/WIP/workorder-active.md` — `2224738b40aa030452a0cce2bfd30421b31a0559` (`SOLO-WO-032 R01`) |
| Checker source blob | `crates/solo-cli/tests/architecture_checks/source.rs` — `8e42f700408d490a45471062fe6cef5154c7415f` |
| Real fixture-test blob | `crates/solo-cli/tests/check_architecture_fixtures.rs` — `714fc59bc200b4b483f585bab87c6f9c08dc09ae` |
| Shared checker module blob | `crates/solo-cli/tests/architecture_checks/mod.rs` — `89dfea91bd82a9b5623f6d437ec787f42fe4aa3b` (read only) |
| ROADMAP / NEXT / OUTLOOK blobs | `d01780d4af965b342521e3594f7a1e4a7dfc551d` / `92fa562b1af0c54391851aa5c79deb1631127069` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |

The previous agent's `COMPLETE` report is **not** Chief Architect acceptance. R01.B and R01.C remain in ROADMAP; NEXT contains only its neutral heading; ACTIVE remains `SOLO-WO-032 R01` until the Owner installs this complete order. No automatic acceptance or closure is permitted.

Read in the mandated order: `README.md`, `AGENTS.md`, About, Architecture, Architect SOP, Implementation SOP, all four WIP files, the canonical report, then the relevant Rust source/tests. Obey Architecture §§6A.2–6A.4; Architect SOP §§6, 6A, 6B, 7A, 7B, 8; Implementation SOP §§4–7A. These documents are read-only and this work order does not amend them. **Authority exists only after the Owner installs this exact `OWNER-ISSUED` ACTIVE text and gives the matching `SOLO-WO-033 R01` handoff.**

### 2. Preflight, ownership and issue safety

Work only in the **one existing Solo checkout**, expected locally at `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, but establish the actual Git root rather than assuming an accessible path. **Before writing, installing, fetching or running source:**

1. Verify local `main` at the exact starting HEAD/tree, its tracking and sole `origin` identity, exactly one worktree/checkout, no nested or shadow repository, no unexpected staged/unstaged tracked changes, no stash, and inspect untracked/ignored state and redirect/reparse ownership. Never reset, clean, checkout, rebase, discard unknown files or create a second clone/worktree/branch.
2. The **only permitted pre-existing tracked difference** from HEAD is the Owner's replacement of the prior ACTIVE text by this **complete** `SOLO-WO-033 R01` order. Confirm `OWNER-ISSUED`, matching Owner handoff, exact scope and full file integrity. Preserve the installed ACTIVE bytes unchanged throughout execution; carry them into the source commit. Any other unexplained tracked drift is a STOP.
3. `.documents/WIP/` has **exactly** `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`, without additional file/subdirectory. ROADMAP still lists R01.B and R01.C, not R01.A; NEXT is the neutral single heading; OUTLOOK unchanged. Neither ROADMAP, NEXT, OUTLOOK nor ACTIVE content may be edited by the agent.
4. Verify the exact reviewed report/source/fixtures/WIP blobs above and the following additional read-only identities before implementation:

| Baseline read-only path | Git blob SHA |
| --- | --- |
| `README.md` | `3f3005d680d9ed182a02e574744054aaae55deb2` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `.documents/About/FOUNDATION-SOLO.md` | `1d80f62769053ba4e6ca51adf2d11c68e724c30e` |
| `.documents/Architecture/ARCHITECTURE.md` | `03e8923f834b50fc28e1b56e70f73017525cfa5b` |
| `.documents/SOP/sop-architect.md` | `8760be323b6ed85695a9dcffd3e52f94628f0f07` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `Cargo.toml` | `14505be49a281ba0495eabea5ca386548cc36347` |
| `Cargo.lock` | `05a02666894f4f0bf5528acf118a049e5edb853e` |
| `crates/solo-cli/Cargo.toml` | `1ca5e114a6e034aae386ea9d7adab3a81b4ae8bc` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| `.cargo/config.toml` | `74a4fb6c0f595ebc64841b26e7961967294f7d3e` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `e328dd6a9f54c47e3eae60053446d52edacc74b7` |
| `crates/solo-cli/tests/architecture_checks/mod.rs` | `89dfea91bd82a9b5623f6d437ec787f42fe4aa3b` |
| `crates/solo-cli/tests/check_architecture.rs` | `0bad0e027f41b6cf6029df38174fc15531c187da` |
| `crates/solo-cli/tests/workspace_graph.rs` | `8fa4b4d0bde840782abb0c9e24c84bac8f06c947` |
| `crates/solo-cli/tests/cli_smoke.rs` | `921cbbe91fb0a75919014e91455fbc080dfd111c` |
| `crates/solo-cli/src/main.rs` | `6faa9fae29351123a49d45c5637ac4fbb1b615c8` |

5. Explicitly inventory the **known pre-existing foreign scratch** in `.temp/rt/tmp/system-commandline-sentinel-files/` mentioned in the prior canonical report. It is **not task-owned**, is not this package's deliverable, and must **not** be deleted or adopted. Ensure task-owned caches/tests use an independent, verified `.temp/` subspace. If an occupied/redirected scratch ancestor cannot be proven safe, STOP. No background cleanup of the Owner's or external process's material is authorized.
6. Establish the exact pinned Rust `1.99.1` GNU toolchain and already locked registry graph; installation if necessary only into positively owned Solo task scratch, without changing tracked toolchain/config or the global OS. No new package/version is authorized.

### 3. Single targeted source correction

Edit only `crates/solo-cli/tests/architecture_checks/source.rs` for this technical change. In `rust_files_under(root, label, anchor, fs)`:

- **Before the initial `fs.list(root)`**, prove (a) the lexically normalized `root` is the intended package directory under `anchor`, (b) the checkout anchor/root and all original unresolved path entries leading through `crates` and the package root are real and nonredirected, (c) the directory is an inspectable **directory** rather than a file/redirect/unknown entry, and (d) canonical/effective containment is safe where needed. Reuse the already selected `inspect_original_chain`, `FsReader::confine`, `CheckoutAnchor` and/or the production `DiscoveryFs` seam as appropriate rather than inventing a second policy. Use the verified anchor passed by the caller; never adopt a redirected or self-canonicalized package root as its own authority.
- Apply the **same pre-enumeration check to every directory removed from the pending queue**, not just the initial root or newly discovered child. Perform it *before* `fs.list(&directory)`. If it fails, record a deterministic repository-relative `module-source-escape` for redirection/out-of-root cases, or `unreadable-source` for an uninspectable/unresolvable directory, and do not list/descend into that directory. Never downgrade a failed pre-enumeration check to a warning or skip without a violation.
- Preserve the existing per-entry fail-closed handling for `DiscoveryFs::redirected`, `kind`, `list`, unknown entry kinds, non-`.rs` metadata errors, symlink/junction detection and known hidden/`target` exclusions. Entries are not exempt from inspection merely because their names are ignored. Do not weaken `check_unit`, the source reader's existing read confinement, public-surface traversal, module/path/include scanning, `cfg_attr` refusal or the graph guard.
- Preserve deterministic ordering, bounded non-secret diagnostics, real-tree coverage of four packages/declared targets, `MapReader` in-memory fixtures, and genuine positive loading. Do not claim hostile TOCTOU race freedom. If a safe change needs another permanent file or new dependency, **STOP** rather than expanding scope.

**Acceptance hinge:** a redirected package root must be refused by **discovery itself, before even its first `list` call**; refusal elsewhere in `check-architecture` does not replace this proof.

### 4. Adversarial and positive tests — direct discovery path

Edit only `crates/solo-cli/tests/check_architecture_fixtures.rs` for tests. Reuse the established `OwnedScratch` lifecycle, safe exclusive scratch creation, genuine Windows `mklink /J` / Unix directory symlink helpers and target-preservation snapshot mechanism. Do **not** manipulate the real `.temp`, `crates` or Git root. Tests must run as ordinary `#[test]` in `cargo test --workspace --all-targets --locked`.

**T-01 — Redirected initial package root.** Construct a genuine junction/symlink at `fixture-checkout/crates/solo-application` pointing to another **in-fixture-checkout** directory with ordinary parsable `.rs` source. Show that root and child canonicalize consistently (the former false-positive scenario), and that the unresolved original root is a redirect. Call the production `source::rust_files_under(&package_root, label, &fixture_checkout, &inspector)` **directly** with `RealDiscoveryFs` or a transparent recording wrapper that delegates to it. Require an actual `module-source-escape`, **no inventoried redirected `.rs` file**, and unchanged link/target bytes. Make the proof stronger by instrumenting the existing `DiscoveryFs` `list` call and asserting **zero calls for that refused root**; a list performed before a later error is a test failure.

**T-02 — Redirected package ancestor.** Repeat the direct discovery test with `fixture-checkout/crates` as the real redirected component (package-root entry appears ordinary after resolution). Assert the same category, **no enumeration of the redirected package directory**, and untouched target/link. Target may be within the owned fixture checkout, but no file through that link may be consumed.

**T-03 — Fail-closed queued directory.** Show that the walker validates each dequeued non-root directory before calling `list`, not solely when it is appended to the queue. Use a deterministic injected `DiscoveryFs` seam if needed to represent a queued directory becoming uninspectable/redirected after initial classification; the pre-enumeration validation must observe the real condition and return a violation without enumerating it. Do not rely on unreliable timing races or change the production routine to accommodate a mock-only result. If a real queued-directory redirect cannot be created safely, provide a bounded deterministic equivalent exercised through production pre-list code and state the limitation.

**T-04 — Existing cases remain load bearing.** Preserve and run the prior genuine root/ancestor source-checker tests, `include!`, `#[path]`, default `mod` redirect tests, conditional `cfg_attr` tests with a valid default, missing source refusal, legal nested module and in-package include, scratch ancestor/collision preservation, injected non-`.rs` metadata-failure test, redirected discovery-child refusal and legal discovery. The 23 forbidden checker fixtures must still report **the expected category** with `outcome=refused`, exit `1`; five legal twins and default real-tree check must remain legal; `--self-test` must still fail on disabling/short-circuiting the guard. The unexpected-acceptance exit `3` must remain distinct from refusal `1` and usage `2`.

**T-05 — Safety evidence.** Tests must positively prove fixture roots were exclusive, no redirected path became a cleanup root, no foreign file was changed/deleted, the real junction/redirect actually existed, and only their own links/files/empty directories were removed. If genuine redirection is unavailable on the test host, the mandatory test is **UNVERIFIED / STOP** rather than claimed PASS. Preserve the disclosed foreign `.temp/rt` content untouched.

### 5. Exact permanent path and operation permissions

Only the following **four** permanent paths may change:

| Path | Authorized action |
| --- | --- |
| `.documents/WIP/workorder-active.md` | **Owner-installed** `SOLO-WO-033 R01` replacement; preserve its text byte-for-byte and commit it |
| `crates/solo-cli/tests/architecture_checks/source.rs` | Narrow pre-enumeration source-discovery correction, no broader checker rewrite |
| `crates/solo-cli/tests/check_architecture_fixtures.rs` | Direct discovery-root/ancestor/pre-list regressions and their narrowly necessary test helper |
| `.documents/Reports/execution-report.md` | Overwrite **one** canonical factual run report |

Everything else is **read-only**: ROADMAP, NEXT, OUTLOOK, all other WIP entries, `mod.rs`, `graph.rs`, `check_architecture.rs`, `workspace_graph.rs`, production Rust code, every manifest, `Cargo.lock`, toolchain/config, README/AGENTS, About, Architecture, SOPs, security/license/CI/browser files, other repositories and Owner data. No new tracked fixture file/subdirectory, alternate checker, WIP log/archive, second report or dependency update.

Allowed operations: read-only safe Git/file preflight; bounded code/tests in the two approved Rust files; pinned Rust/locked-dependency operations in verified task-owned scratch; genuine temporary regression fixtures under the established safe Solo `.temp/` hierarchy; local fmt/Clippy/test/Cargo metadata/release reproducibility checks; local Git commits of allow-listed paths only; and **one final push** of completed safe commits to existing `origin/main`, with remote readback, under Architect SOP §6B.

Denied: intermediate pushes or direct GitHub file mutations, extra branches/clones/worktrees, new remote, PR/merge, force-push/reset/rebase/destructive clean, global OS or registry modifications, unapproved dependencies/versions, foreign scratch cleanup, other-repo writes, CI/browser/HTTP/provider operations, R01.C, publication/release/deployment, and R01.B WIP administrative closure. Unknown operations require an explicit new Owner decision.

### 6. Verification gates and required report

After editing and focused local adversarial checks, commit the two source/test edits **together with the unchanged Owner-installed ACTIVE** as the corrected source identity. Run verification on that **committed** source. Record exact commands and process exits, not just summary labels:

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

Run focused new root/ancestor/pre-list cases and previous architecture, graph and CLI smoke regressions. Execute each registered forbidden `--fixture <id>` with observed `outcome=refused`, correct category and checker exit `1`, not merely `status != 0`; verify unknown switch returns usage `2`. Verify CLI release success (`bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha` ⇒ `0`, `preflight=ok entity_id=entity-7 base=https://example.com/alpha`) and input refusal (`--entity-id Main` ⇒ `2`, safe exact stderr). Once trusted caches exist, also run locked **offline** test and release build. No test may be ignored or conditionally skipped to manufacture PASS.

**Reproducibility:** two independent, fresh-`CARGO_HOME` and fresh-`CARGO_TARGET_DIR` `x86_64-pc-windows-gnu` release builds using unchanged pinned Rust `1.99.1` and tracked `.cargo/config.toml`, comparing raw executable length and SHA-256. Confirm `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` are **absent**, not set to empty values (empty `RUSTFLAGS` can shadow the tracked deterministic linker flags). No ephemeral linker flag or PE-header normalization. Verify identical hashes between **the two new runs**; previous hash is context, never a substitute for new execution. Keep MSVC/Unix, advisory scan, CI/browser, release and provider limits factually distinguished from tests actually run.

| Gate | Passing evidence |
| --- | --- |
| **H-01 — Authority and diff** | Exact HEAD/tree and reviewed blobs; one checkout; one Owner-installed ACTIVE difference; exactly four changed permanent paths (if all used); WIP inventory and untouched ROADMAP/NEXT/OUTLOOK |
| **H-02 — Root pre-enumeration** | Direct real `rust_files_under` genuinely redirected initial package root refuses before **any** `DiscoveryFs::list`; no inventory; preserved target and link |
| **H-03 — Ancestors and queued dirs** | Direct discovery refuses redirected `crates` ancestor; every queued directory validated before `list`; injected/pre-list negative cannot be bypassed; original metadata-error coverage preserved |
| **H-04 — Ownership and existing positives** | Existing exclusive `OwnedScratch` collision/redirect tests pass; legal discovery and in-package source loading still pass; no foreign scratch touched, no task residue |
| **H-05 — Checker and CLI regression** | Legal four-crate real check, all 23 forbidden fixtures and five legal twins, bypass, graph negatives and actual CLI success/refusal green; no reduced scanning surface |
| **H-06 — Toolchain/reproducibility** | Pinned compiler, fmt, strict warning-free Clippy, locked/offline tests and build, metadata/tree, two raw-byte-identical independent Windows GNU releases and unchanged lock/dependencies |
| **H-07 — Report/publication** | One accurate report, diff allowlist, clean checkout, safe scratch disposition, one final push and exact remote HEAD/tree readback |

Overwrite **only** `.documents/Reports/execution-report.md`, stating work-order ID, Owner handoff, exact starting HEAD/tree, correction commit/tree, file blob ledger, actual root/ancestor test construction and **pre-list count**, evidence that the original redirect existed and material remained untouched, categories and positive cases, exact test/process commands/exits, binary hashes/flags, read-only file drift audit, known foreign `.temp/rt` preservation, task-owned cleanup outcome, OS/toolchain/advisory limits and H-01–H-07 `PASS`/`FAIL`/`UNVERIFIED` statuses. Agent status `COMPLETE` is not Architect `ACCEPT`. Do not embed the report commit SHA in its own content; hand the final GitHub SHA back in chat.

Sequence: safe preflight → bounded edits → focused negatives → local source/ACTIVE commit → full gates **on that commit** → report overwrite and commit → safe owned-scratch cleanup plus diff/working-tree audit → **one final push** to `origin/main` → remote identity verification. Follow SOP §6B, no intermediate publication. A mandatory failure is `STOP`/`FAILED` with truthful safe reporting, not permission to publish unsafe incomplete code.

### 7. STOP conditions and future boundaries

STOP on wrong baseline, missing full Owner-issued ACTIVE and matching handoff, multiple checkout/worktrees, unexplained drift or unsafe scratch, a root/ancestor/listing escape not refused before enumeration, inability to prove a genuine redirect or proper ownership, missing or weakened regression tests, unauthorized path/dependency, failed pinned toolchain/locked/offline/reproducibility gate, unclean task-owned scratch or unsafe Git publication. Do not weaken the checker to conceal the defect; report the smallest specific blocking Owner decision if permission expansion is actually necessary.

**R01.B stays in ROADMAP and ACTIVE remains this complete order until a later independent Architect `ACCEPT` and distinct Owner-authorized closure.** R01.C and the full Stage 01 exit remain pending. No next implementation work, release, merge or deployment is authorized by this corrective order.
