# Foundation Solo — Active Work Order

## SOLO-WO-026 R03 — Stage 01 R01.A

## Approved Scope — OWNER-ISSUED

### 1. Objective

Implement **R01.A — minimal reproducible engineering foundation**, in the existing Foundation Solo checkout at `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`. Deliver a Rust 2024 four-crate workspace, pinned Rust tooling, one tracked `Cargo.lock`, and a **real engineering-only CLI bootstrap preflight**. The CLI takes an explicitly authored Entity ID and base URL, invokes a typed application use case and pure domain validation through an actual adapter, and returns deterministic success or refusal. It neither creates an Installation nor publishes, renders, routes, serves, or reads a publication manifest.

Keep the accepted one-Entity identity model, independently authored public base URL and strictly inward-only Hexagonal Architecture. Do not implement `R01.B`, `R01.C`, Stage 02 onward, a finished publication validator, HTTP server, browser, CI, hosting provider, or product release. The complete Stage 01 exit gate is **not** an `R01.A` completion claim.

**Repository/source baseline:** `provelopment/provelopment-foundation-solo`, existing `main`, HEAD `ff658baaaa52d8eeae9a1414c976b9597bf5b459`, tree `5421891e2078eb5bc7b815adf6b88949e40491d9`. Treat changed or unexplained source state as a STOP, not as authority to reinterpret this work order.

**Governing requirements:** Read the documents in the order specified by `README.md` and `AGENTS.md`, including `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md`, both SOPs, all four WIP files and `.documents/Reports/execution-report.md`. In particular apply Architecture §§1.1, 4.1, 5, 5A, 6.2B, 6.2J, 6.2K, 6A.1–6A.4 and 6A.15; Architect SOP §§6, 6A, 6B, 7A, 7B and 8; Implementation SOP §§4–7A. The accepted Charter is the authority for semantic rules; this order only specifies the bounded implementation slice.

### 2. Local checkout and safety preflight

Perform the work **locally** in the single existing Solo checkout. Before creating or changing any local file, installing tooling or fetching dependencies:

- Verify the actual local repository root, `main` branch, `origin` configuration, HEAD and tree, tracked/untracked changes, symlinks, stashes, `git worktree list --porcelain`, ignored scratch, and any Owner-owned or unexpected files. Use local Git inspection; do not create another clone, checkout, worktree, branch, or shadow source tree.
- The source at start must match the exact baseline above. The only expected pre-existing change is the Owner's replacement of neutral `.documents/WIP/workorder-active.md` with this work order. No other unexplained dirty or untracked file may be overwritten, staged, discarded or silently incorporated. Do not change the installed work-order text.
- Verify `.documents/WIP/` contains exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`, with no other files or subdirectories; `R01.A` is the sole NEXT candidate and remains in ROADMAP; ACTIVE contains this single `SOLO-WO-026 R03` scope.
- Verify the source blobs against this reviewed baseline. The locally installed ACTIVE blob is necessarily different from the neutral ACTIVE baseline and is **not** a drift failure.

| Baseline file | Expected Git blob SHA |
| --- | --- |
| `README.md` | `3f3005d680d9ed182a02e574744054aaae55deb2` |
| `AGENTS.md` | `32562b6730e26a37b9febae63efd7d5c6a5866ac` |
| `.documents/About/FOUNDATION-SOLO.md` | `1d80f62769053ba4e6ca51adf2d11c68e724c30e` |
| `.documents/Architecture/ARCHITECTURE.md` | `03e8923f834b50fc28e1b56e70f73017525cfa5b` |
| `.documents/SOP/sop-architect.md` | `8760be323b6ed85695a9dcffd3e52f94628f0f07` |
| `.documents/SOP/sop-implementation.md` | `a577116082bf8133a6056667aa52fe4ffc2ae94d` |
| `.documents/WIP/plan-roadmap.md` | `4f46dfcc4fcb699a491f6df0847b902eb805df78` |
| `.documents/WIP/plan-next.md` | `3b753df81b67ab3b63f9031bdd91aab795325434` |
| `.documents/WIP/workorder-active.md` (**neutral prior to replacement**) | `ea5f20b736483c2e102456be1cea9104059fc81a` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `870e9064006464d9e6c8b2bfa130c2f9cd28ed81` |
| `.github/SECURITY.md` | `dd321856fe8035917be3f90c733242bc3d9a8d09` |
| `LICENSE` | `d645695673349e3947e8e5ae42332d0ac3164cd7` |
| `.gitignore` | `f51cc5306cdc8e4532422809eede68b93c4a7fd7` |

STOP on mismatched source, unexpected repository state, uncertain ownership, unauthorized path, unresolved symlink destination or conflicting governing text. Preserve unknown material; never use destructive Git cleanup to simulate a safe tree.

### 3. Local workflow and WIP transition

All implementation, tooling, dependency resolution, tests, evidence collection, source staging and Git commits occur **locally first**. **No GitHub publication, readback, intermediate push, issue, pull request, branch operation or preliminary publication ceremony occurs during implementation.** Use the existing local Git checkout and follow Architect SOP §6B for version-control mechanics where applicable.

After successful local preflight, clear the issued `R01.A` candidate from NEXT by replacing `.documents/WIP/plan-next.md` with exactly this line and a single final LF:

`# Foundation Solo — plan-next (Next Candidate)`

Keep `R01.A`, `R01.B` and `R01.C` in ROADMAP. Do not update ROADMAP, OUTLOOK or the Approved Scope in ACTIVE; do not fill NEXT with another candidate or declare acceptance or closure. This NEXT change is the expressly permitted issue transition, not an acceptance claim.

Develop, inspect, test and commit locally. Once all work and the canonical report are complete and the final local repository state is safe, **push the permitted commits together to the existing `origin/main` once at the end**. Confirm the final push succeeded and hand back the exact local commit/tree identities. If GitHub publication fails, report the failure; do not switch to a PR, another branch, a new remote or a destructive operation. If a STOP occurs, preserve local work, record facts in the single report where safe, and only publish a factual report at run end if safe and permitted under the SOP; do not publish unsafe or unfinished code to force completion.

### 4. Rust workspace and architectural boundaries

Create these exact Cargo workspace members and no others:

| Crate | Required real purpose | Allowed dependencies |
| --- | --- | --- |
| `solo-domain` | Validated `EntityIdV1` and bounded typed `BaseCandidateV1`; pure decisions, no side effects | `std` only; `#![forbid(unsafe_code)]` |
| `solo-application` | `BootstrapPreflightV1` inbound use-case contract and concrete application service; typed accepted/refused results | `solo-domain` only, plus `std`; `#![forbid(unsafe_code)]` |
| `solo-adapters` | External CLI flag/argument handling and safe result/diagnostic translation | Inward dependencies on `solo-domain` and/or `solo-application`; `std` only |
| `solo-cli` | Single binary and composition root wiring real adapter to real application use case and setting exit status | Inward dependencies on approved workspace crates; `std` only for runtime |

The workspace uses Rust **2024 edition**, Cargo resolver **3**, `rust-toolchain.toml` with exact `channel = "1.99.1"` and `rustfmt`/`clippy` components, and a single tracked root `Cargo.lock`. Pin every direct external dependency exactly and preserve locked transitives. Only one external crate is approved for this slice: **`serde_json = "=1.0.154"` as a `solo-cli` dev-dependency**, for parsing Cargo metadata in architecture tests, never for authored input or a production parser. No other third-party direct dependency, binary runtime, package manager, framework, transport, provider or hosting integration is approved. Do not silently change the conditional Charter pins if compilation or dependency provenance fails.

Use a versioned `BootstrapPreflightV1` inbound interface and concrete service with `BootstrapInputV1`, `BootstrapAcceptedV1` and typed diagnostic/refusal outcome (equivalent clearly named typed structures are acceptable). Values must be validated before construction; public constructors must not bypass domain rules. `solo-adapters` translates raw CLI input to the application boundary; domain and application never import the adapter, OS/HTTP/filesystem/provider models, or concrete CLI concerns. No ornamental outbound port is needed because this use case reads or writes nothing external. Tests exercise the application through its interface and the **actual compiled process**. A test-only alternative invocation is fine but may not become a second production composition root.

`R01.B` retains the full alias/re-export/macro/`include!`/qualified path/dynamic-loading source checker (`syn =3.0.9`) and substantial C3 enforcement. `R01.C` retains CI, production-mode HTTP/browser smoke and real request-to-application integration. The Cargo graph guard in this work order cannot be described as completion of either package.

### 5. Exact bootstrap-preflight behavior

Command:

```text
cargo run --locked -p solo-cli -- bootstrap-preflight --entity-id <authored-id> --base-url <authored-base>
```

Either flag order is accepted. Each option must appear exactly once, with one separately supplied nonempty UTF-8 value. Reject missing/duplicated/unknown flags, unexpected positional arguments, extra values, non-UTF-8 OS arguments and any interactive, environment, current-directory or URL-derived fallback. No manifest path, implicit Entity ID, persistent write, network call or publication action is part of this command.

**Entity ID:** validate **exactly** `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$`, length **1–63 ASCII bytes**. Accept `solo`, `entity-7` and valid boundary-length examples. Refuse empty, uppercase, Unicode, `1x`, `_a`, `a.b`, slashes, percent escapes, leading/trailing hyphens, repeated hyphens, whitespace and >63 bytes. Never lowercase, replace, truncate, generate or infer an ID. It remains the **same** validated ID when the base address changes.

**Base candidate, deliberately narrow engineering subset:** Accept lowercase `http://` or `https://`; ASCII DNS host (≤253 octets) consisting of labels of 1–63 ASCII octets, each starting/ending alphanumeric with only alphanumerics and internal hyphens; host ASCII case normalizes to lowercase; optional decimal port 1–65535, removing default `http:80` or `https:443`; and 0–5 ASCII-safe prefix segments each exactly `[a-z][a-z0-9-]*`. Canonicalize a single terminal `/` away except for an origin-only URL. Preserve a legitimate nondefault port. Refuse credentials, query, fragment, controls/whitespace, backslash, percent-encoded prefix, repeated/empty segments, traversal, invalid port, invalid DNS labels, ambiguous separators and unsafe encoding.

IP literals, IDNA/Unicode or `xn--` host labels, and other forms requiring the future full URL profile must return an explicit **unsupported-by-this-preflight** refusal. This is **not** a finding that the full product Charter prohibits those addresses: the accepted §6.2J publication contract admits broader validated HTTP(S) host types, and later stages must implement it. Do not impose HTTPS-only authorship, infer a host from requests, or make URL configuration an Entity identity source.

**Observable outcomes:** exit `0` for valid input and write exactly one deterministic machine-checkable success line on stdout with the validated ID and canonical base; exit `2` on input refusal, with a stable code and short safe stderr diagnostic; exit `1` only for an internal error. No untrusted control sequences, secrets, local paths or implicit repaired values in diagnostics. Distinguish at least `PREFLIGHT_ARGS_MISSING`, `PREFLIGHT_ARGS_DUPLICATE`, `PREFLIGHT_ARGS_UNKNOWN`, `PREFLIGHT_ENTITY_ID_INVALID`, `PREFLIGHT_BASE_INVALID`, `PREFLIGHT_BASE_UNSUPPORTED`; these are **engineering preflight codes**, not additions to the future §6.2K production diagnostic registry.

Required success includes `entity-7` plus `https://EXAMPLE.com:443/alpha` producing `entity-7` and `https://example.com/alpha`, a safe `http://example.org` origin, five prefix segments, an explicit nondefault port and valid boundary IDs. Replacing only the base with `https://other.example/beta` preserves the Entity ID. Required refusals include every denied ID/base/argument class, contradictory input, malformed encoding and malicious control characters. No successful outcome may create content or an Installation.

### 6. Allowed permanent file operations

Only the following paths may change under the **single verified local Solo root**. `crates/` and their parent directories are structural, not blanket permissions to add more files.

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Keep installed bytes unchanged; include in final local commits and final push |
| `.documents/WIP/plan-next.md` | Replace candidate with the single heading specified in §3 |
| `Cargo.toml` | Create four-member workspace and resolver/pin policy |
| `Cargo.lock` | Create and track one shared lockfile |
| `rust-toolchain.toml` | Create exact 1.99.1 pin |
| `crates/solo-domain/Cargo.toml` | Create package manifest |
| `crates/solo-domain/src/lib.rs` | Create validated pure domain types, optionally split into below files |
| `crates/solo-domain/src/entity_id.rs` | Optional type module |
| `crates/solo-domain/src/base_candidate.rs` | Optional type module |
| `crates/solo-application/Cargo.toml` | Create package manifest |
| `crates/solo-application/src/lib.rs` | Create inbound contract/service, optionally split into below file |
| `crates/solo-application/src/bootstrap_preflight.rs` | Optional use-case module |
| `crates/solo-adapters/Cargo.toml` | Create package manifest |
| `crates/solo-adapters/src/lib.rs` | Create adapter public boundary, optionally split into below file |
| `crates/solo-adapters/src/bootstrap_cli.rs` | Optional CLI parsing/output module |
| `crates/solo-cli/Cargo.toml` | Create binary manifest and dev-only pinned `serde_json` |
| `crates/solo-cli/src/main.rs` | Create sole composition root/executable |
| `crates/solo-domain/tests/identity.rs` | Create ID positive/refusal tests |
| `crates/solo-domain/tests/base_candidate.rs` | Create base positive/refusal tests |
| `crates/solo-application/tests/bootstrap_preflight.rs` | Create use-case and ID/base-independence tests |
| `crates/solo-adapters/tests/bootstrap_cli.rs` | Create CLI boundary/diagnostic safety tests |
| `crates/solo-cli/tests/cli_smoke.rs` | Create real-process success/refusal tests |
| `crates/solo-cli/tests/workspace_graph.rs` | Create actual Cargo metadata graph guard and negative-injection tests |
| `.documents/Reports/execution-report.md` | Overwrite the single report with factual run evidence |

No existing README, AGENTS, About, Architecture, SOP, ROADMAP, OUTLOOK, security, license, `.gitignore` or `.github` file may be edited. Do not create new WIP files, reports, schema/contract artifacts, browser files, CI workflows, content, artwork, build products in root `target/`, or documentation archives. Generated build/test artifacts and task logs go only to Solo's ignored `.temp/` or the Cline-provided task temporary area. Use `CARGO_TARGET_DIR` and controlled task-local Cargo/rustup caches as needed; remove owned scratch when no longer needed, preserve unknown material and record any required retention/cleanup limitation.

### 7. Operation permissions

| Operation | Permission |
| --- | --- |
| Local source changes, tests, local Git staging/commits | **Allow** only for paths in §6, after preflight |
| Final push to existing `origin/main` | **Allow** **once after local work, tests and report are complete**, with safe push mechanics from Architect SOP §6B; confirm successful push outcome |
| Exact Rust 1.99.1 and Rustfmt/Clippy installation if absent | **Allow conditionally** from official trusted sources, into task-owned approved temporary locations only; verify version/provenance; no uncontrolled global installation |
| Cargo dependency fetch | **Allow conditionally** only for exact approved pins and locked transitive dependencies from trusted official registry sources; review provenance, licenses/advisories; no floating/git dependencies |
| Approved disposable local/Cline task scratch and cleanup | **Allow**, with ownership-aware cleanup and no second checkout |
| Intermediate GitHub push, GitHub file edits, PR, issue, branch creation or merge | **Deny** |
| Rebase, force push, history rewrite, destructive Git clean/reset/checkout, extra clone/worktree | **Deny** |
| Other Provelopment repository writes, arbitrary permanent OS/profile changes, unspecified files or dependencies | **Deny** |
| Vercel or other hosting, external provider/API setup, DNS, TLS, accounts, secrets, billing, release, deployment | **Deny** |

All unlisted operations are unauthorized. Do not expose confidential information or vulnerability details in public output; follow `.github/SECURITY.md` for private vulnerability reporting.

### 8. A1 tests and evidence

Run on the **actual locally committed implementation revision** with identical pinned compiler/target/features, controlled scratch, and actual exit-code capture:

```text
rustc --version
cargo --version
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --workspace --release --locked
cargo metadata --format-version 1 --locked --all-features
cargo tree --workspace --all-features --locked
cargo run --locked -p solo-cli -- bootstrap-preflight --entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha
cargo run --locked -p solo-cli -- bootstrap-preflight --entity-id Main --base-url https://example.com
```

Also run the full representative positive/refusal/edge cases required by §5. Preserve exact input class, observed outcome, stable diagnostics and exit codes. Test through the actual compiled CLI as well as the domain/application interface; a fake-only suite or successful compilation is insufficient.

**Architecture tests:** Inspect real Cargo metadata resolve nodes for all dependency kinds and features, including dev/build, renamed aliases and transitive workspace graph. Refuse prohibited `solo-domain` external dependencies and outward/sibling cycles. Deliberately inject **in-memory graph fixtures** for `solo-domain → solo-adapters`, `solo-application → solo-cli`, `solo-application → solo-adapters`, `solo-domain → external`, a renamed illegal edge and a dev/build illegal edge. Assert graph check returns nonzero/errors for each, and that disabling the guard makes the suite fail. Review all created Rust imports manually; do **not** claim the source-level checker scheduled for `R01.B` is implemented.

**Reproducibility:** Perform two separate, fresh, task-owned Cargo cache and target-directory release builds against the **same local source checkout** and recorded compiler/target/features, without a second checkout or source copy. Compare the hashes of shipped binary output and the unchanged tracked `Cargo.lock`; a mismatch is a failed gate, not an acceptable approximation. Include a `--locked` lock-change refusal check and, after caches are populated, an `--offline --locked` build/test where applicable. Never substitute a different compiler, pin or unchecked lockfile for a failing command. Record `rustc`/Cargo version, OS/target triple, source/tree ID, exact commands, exit codes, crate inventory, checksums, warnings, skips, failed/untested results and cleanup. If an approved `cargo-deny =0.20.2` executable is verifiably available in the allowed environment, run its relevant checks; otherwise mark the scan **NOT RUN**, explicitly assess dependency risks, and do not claim a scan passed.

**A1 acceptance gates** (Architect SOP §7B supplies the quality standard; this is a task-specific proof checklist):

| Gate | Required accepted behavior | Required failure/refusal proof |
| --- | --- | --- |
| A1-01 Local safety and scope | Exact local baseline, one checkout, WIP inventory, permitted paths only | Unexpected HEAD/tree, dirty Owner file, wrong remote/worktree or unapproved file triggers STOP |
| A1-02 Workspace/toolchain | Four real Rust 2024 crates, exact compiler and one tracked locked graph | Different pin, extra crate/dependency, missing lock, unresolved dependency fails |
| A1-03 Identity | Typed correct 1–63-byte Entity ID and stable independent identity | Invalid IDs, length boundaries, coercion and inferred IDs refuse |
| A1-04 Base candidate | Canonical safe HTTP(S) ASCII subset, prefix 0–5, port handling | Unsafe/ambiguous/unsupported addresses and hidden base fallback refuse |
| A1-05 End-to-end CLI/use case | Actual binary→adapter→application→domain success; no persistent side effects | Missing/duplicate/unknown/conflicting/non-UTF-8 args and malicious diagnostics refuse |
| A1-06 Hexagonal dependency | Actual inward-only Cargo graph, pure domain, inner `forbid(unsafe_code)` | Forbidden direct/aliased/dev/build/transitive edges and disabled graph guard fail |
| A1-07 Reproducibility/quality | fmt, Clippy, tests and locked release builds green; independent output hashes match | Lock/pin drift, failed/skipped mandatory checks or mismatched output fails |
| A1-08 Security/supply chain | No secrets, unsafe logging, unreviewed package or consequential warning | High/critical exploitable risk, unsafe dependency, secret leakage or unchecked diagnostics triggers STOP |
| A1-09 Local completion/publication | One factual report, clean local tree, final one-time push of authorized commits, successful final push result | Early publication, extra WIP/report, ROADMAP closure, unauthorized provider/release or missing evidence fails |

Report each applicable gate as **PASS**, **FAIL** or **UNVERIFIED** from actual evidence; use **NOT APPLICABLE** with a reason for scope-inapplicable standards. Deferred—not passed—in this R01.A slice: full C1 parser/schema/Unicode/Markdown validation; C2 sealed artifact/delivery; complete C3 imports/browser wire checker; C4 modules ledger; F01–F17 contact; RG01–RG11 release; R01.B source-level boundary tooling; R01.C CI/HTTP/browser tests; Node/npm/TypeScript/Vite/Playwright; Stage 02+ publication/route/site/locale/visibility; WCAG/browser/CWV measurement and provider deployment. A security, correctness or architecture violation **introduced now** cannot be excused as deferred.

### 9. Single execution report and completion

Overwrite **only** `.documents/Reports/execution-report.md` with this run's factual record. Include date, `SOLO-WO-026 R03`, actual local root, initial and implementation source HEAD/tree, complete changed-path list, Rust/toolchain/dependency/lock identities, successful and refused CLI observations, real graph-negative fixtures, build/reproducibility hashes, security/provenance conclusions, exact test commands and exit codes, failed/skipped/not-run gates, temporary-workspace cleanup, branch/remote state, limitations, and status `COMPLETE`, `STOPPED` or `FAILED`. Do **not** put the report's own final commit SHA in the report; give it in the final chat handback.

Locally commit the completed allowed changes, including the installed ACTIVE text, NEXT transition, source/test/lock files and overwritten canonical report. Run a final local diff/path/WIP/working-tree audit. Then make **one final push** to `origin/main`; confirm successful push outcome and identify the pushed HEAD and source/tree revision. Do not publish another interim report, create an archive or perform another GitHub action to stage the work. If publication cannot safely complete, state the precise blocker and preserve local evidence without destructive correction.

The Implementation Agent reports implementation facts; the Chief Architect separately performs independent `ACCEPT`, `REVISE` or `STOP` review. Keep ROADMAP and ACTIVE unchanged for that review; do not self-accept, close `R01.A`, advance NEXT, merge, release or deploy.

### 10. STOP conditions

STOP on an authorization or source/checkout mismatch, unknown Owner data, an unauthorized path, WIP contamination, unsafe symlink, untrusted compiler/dependency or licensing/advisory problem, Rust 1.99.1 incompatibility, an unapproved technical choice, accepted-contract conflict, illegal import/dependency, unsafe diagnostics or secret exposure, mandatory failed build/test/negative gate, non-reproducible binary hash, irreversible/destructive operation, need for another branch/PR/provider/deployment, or inability to publish safely at the end. Record the observed failure, completed subset and smallest necessary decision; do not silently narrow tests, change pins, invent a fallback or treat failure as completion.
