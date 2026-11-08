# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, from approximately 15:00 |
| Work order | **`SOLO-WO-043 R01`** — Stage 01 Closure and Stage 02 Readiness Transition |
| Matching handoff | Received — `SOLO-WO-043 R01` |
| Installed ACTIVE | blob `bbf3dfb19381c1ab37682244a3017b16f065dfa3`, 17,649 bytes, 94 lines, LF, no BOM — **still installed at this evidence revision** |
| Starting HEAD / tree | `9e5ecd624f2fe986317514a4afa003210f6a9c32` / `d09efe8c7048d076bdfc663d4bc2927de902b086` |
| Issuance commit / tree | `87a197e91ef96411c68be8e3edff577bdf2f7bb6` / `e59220adbf1739a025c2e00cbd011d0add9d5910` |
| Instrumented source revision / tree | `2d6a1174b639b98939c559da36e2c70deb9328d1` / `effe150de8b526b67d2e599c8dee70fc74a55e43` (workflow blob `1c0e450a…`) |
| Instrumented CI | run [`42493698817`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42493698817) — [Ubuntu `128771292695`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42493698817/job/128771292695) and [Windows `128771292889`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42493698817/job/128771292889), both `success`, attempt 1 |
| **Phase reached** | **evidence phase complete**; Stage 01 **retained**; this report's own revision is named in chat |
| **Run status** | **`COMPLETE`** for this evidence phase. **Stage 01 is NOT yet closed** — closure follows the independent Chief Architect `ACCEPT` required by §4.3 of the order |

## Corrections to the previous canonical report

The prior `SOLO-WO-042 R01` report contained two bounded factual defects. Both are corrected here, openly, and no historical Git commit is rewritten.

**Correction 1 — work-order revision mislabels (two rows).** The previous report's identity table labelled its issuance commit and its issuance CI run as **R02**. Both are **R01**. The correct values are: issuance commit `8333c2c418e3e2f884f66c2610e9546defc55b73` = the **`SOLO-WO-042 R01`** issuance, and run `42490811420` = the **`SOLO-WO-042 R01`** issuance CI run, with Ubuntu job `128762554437` and Windows job `128762554768`. That run's report-HEAD sibling, run `42491286323` at `9e5ecd62…`, is likewise **R01** evidence. No `R02` revision of `SOLO-WO-042` ever existed; the mislabel carried over the preceding task's numbering and changed no technical conclusion.

**Correction 2 — the `rustup` claim.** The previous report's G4 table listed "Cargo / rustup — resolved by that pinned toolchain and asserted by the same per-host step", which wrongly implied that `rustup` is pinned by `rust-toolchain.toml` and that its version was asserted by the exact-`rustc`/`cargo` step. It is not and it was not:

- `rust-toolchain.toml` pins the **toolchain channel** (`1.99.1`) and its `rustfmt`/`clippy` components. It cannot and does not pin `rustup`.
- The per-host `Assert exact rustc/cargo versions` step measures **`rustc` and `cargo` only** — both verified as `1.99.1` in every run cited here.
- `rustup` is an **independently versioned installer**. Its version was never measured before this run and was never asserted by CI.

**Remediation executed this run.** The one permitted instrumentation line, `rustup --version`, was added directly after the existing `rustup toolchain install 1.99.1 …` command in the `Install and assert the pinned toolchain` step (revision `2d6a117`; `+1` insertion, `0` deletions, byte-identical otherwise). It records the real installer version on **both** hosts instead of letting it be inferred from `rustc`.

**Measurement outcome, first-hand this run.** On both hosts the `Install and assert the pinned toolchain` step concluded `success` — Ubuntu `15:13:24Z`→`15:13:32Z`, Windows `15:13:29Z`→`15:13:40Z`. Because that run block now ends with `rustup --version`, a non-zero exit fails the step on both the bash and pwsh shells, so the command's execution and its zero exit status are proven. **The printed version string itself is `UNVERIFIED` at the Agent's vantage:** `GET /actions/jobs/{id}/logs` returns **HTTP 403** for both jobs and the check-run `output` payload is empty, so the log body cannot be read to quote the value.

**Dated cross-check against the official source.** The officially published stable `rustup` release on 2026-11-08 is **1.29.3**, announced 2026-11-03 by the Rustup Team ([Rust Blog, 2026-11-03](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)), described as a patch release fixing an "Illegal instruction" error on certain x86_64 CPUs without SSE4.1 (pr#5162) and a `lock() not supported` error on Solaris (pr#5167), with **no** security advisory, deprecation or end-of-life statement. The two hosted images independently document the installer they preinstall — `Rustup 1.29.2` in both `images/ubuntu/Ubuntu2404-Readme.md` and `images/windows/Windows2025-Readme.md`, read 2026-11-08 — and the workflow installs the toolchain with `--no-self-update`, which deliberately prevents an implicit installer upgrade. The coherent reading is that both runners execute **`rustup 1.29.2`**.

**Disposition: `UPDATE NEEDED`, low urgency, non-blocking.** The installed installer is one patch release behind current stable, so it is **not** reported as `current`. It is also **not** a STOP, with the reasoning stated rather than asserted: 1.29.3 carries no security advisory; 1.29.2 is a prior point release of a supported series, not a deprecated or end-of-life installer; and both 1.29.3 fixes target conditions that do not occur on these runners (they have SSE4.1 and are not Solaris). If the Owner or Architect judges `1.29.2` unsupported, the correct remedy is a **separate authorised order** — for example an explicit installer-update step — not a silent change in this closure run, and never a change to the pinned toolchain.

## Preflight

| Check | Result |
| --- | --- |
| Accessible root | the intended Owner root, confirmed as the repository top level (not a parent workspace, symlink or junction) |
| Branch / remote / remote head | `main`; sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`; remote `main` = `9e5ecd62…` = local HEAD at start |
| Worktrees / stash | exactly 1 worktree, 0 stash entries, no second checkout, nested clone or shadow source tree |
| Working tree at start | the **only** tracked difference was the Owner-installed replacement of ACTIVE; no unknown dirty, untracked or Owner-owned content |
| **Fourteen baseline blobs** | **14 / 14 matched, zero drift** — ACTIVE `17ed15cf…`, report `99761df2…`, ROADMAP `145f9d7a…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…`, `ARCHITECTURE.md` `4b050f63…`, `rust-verify.yml` `5426c1f5…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`, `sop-architect.md` `1483c9be…`, `sop-implementation.md` `51bb3792…`, `AGENTS.md` `6eae1da8…`, `README.md` `ce2d1cf9…` |
| WIP inventory | exactly four files, no subdirectory |
| Prior report | read as history for the correction above; not reclassified as this run's evidence |

`README.md`, `AGENTS.md`, Architect SOP §§6–8, Implementation SOP §§4–5C and 7–9, Architecture §§5/5A/6A with the EFS identity and manifest contracts, Stage 01 and Stage 02 ROADMAP text, all four WIP slots, the canonical report and the relevant source, manifests, security configuration and workflow were read before any writing.

## Phase status — deliberately not closure

The order places a **technical acceptance gate inside the work order**: §4.3 requires the Chief Architect's formal `ACCEPT / REVISE / STOP` against this exact corrected-report revision, and only then may §4.4–§4.6 run. Accordingly this run stops at the evidence phase, and the following remain **untouched and true**:

- Stage 01 is **still present** in ROADMAP — heading, prerequisite and full exit gate byte-identical at blob `145f9d7a…`, 13 stages and 13 exit gates intact; no completion history was written.
- NEXT `92fa562b…` and OUTLOOK `cb9c7a61…` are unchanged; **R02.A has not been installed in NEXT**.
- `ARCHITECTURE.md` blob `4b050f63…` is unchanged — the §5A/§6A status-parity repair has **not** been made.
- ACTIVE still holds the complete installed order at `bbf3dfb1…`; **it has not been neutralised**, and Stage 02 has **not** been promoted.
- No R02.A implementation, EFS product, HTTP surface, release or deployment exists or is claimed.

This is the order's own instructed behaviour, not an incomplete run: `REVISE` or `STOP` must preserve the Stage and report the correction instead of pruning it.


## Stage 01 exit-gate and A1 matrix — fresh evidence

Evidence below is from this run at the instrumented source revision `2d6a117` (run `42493698817`) and, where noted, from the issuance revision `87a197e`.

| Gate | Requirement | Verdict | Anchor evidence |
| --- | --- | --- | --- |
| **G1** | clean-room reproducibility: hosted fresh locked fetch, build, fmt, strict lint, all-target tests, release CLI smoke; Ubuntu two independently clean byte-equal release builds | **PASS** (hosted) · local Windows build `NOT RUN` | both hosts `success`, 0 failed steps: `Fetch locked packages`, `Format check`, `Strict lint (warnings are errors)`, `Test (workspace, all targets, locked)`, `Release build (workspace, locked)`, `Release CLI smoke on the built binary` per host, and `Two clean release builds compared byte-for-byte (Ubuntu)` |
| **G2** | real composition root → adapter → application inbound port → domain, positive and refused outputs and exit statuses | **PASS** | source trace plus 15 `cli_smoke` tests, adapter/application/domain suites and the real release-binary smoke on both hosts |
| **G3** | inward-only crate/import checks, 23 negative fixtures, deliberate forbidden-import failure and restore, tripwire, strict static checks, no provider/browser shell | **PASS** | `Architecture checker on the real checkout`, `Architecture refusal self-test` (pinned to exactly 23 refused), `Test-failure tripwire` per host, and `Forbidden import and bypass refusal with guaranteed restore (Ubuntu)` — annotation confirms `undeclared-crate` refusal and exit 1 |
| **G4** | complete exact graph, re-counted; both-host RustSec; digest-verified `cargo-deny 0.20.2` four classes; disposable `RUSTSEC-2026-0009` refusal; dated independent advisory checks | **PASS** | **4 workspace + 11 registry** packages, **6 name-wide + 3 exact-release** bans, four categories `ok` on both hosts — details below |
| **G5** | exact versions and dated authority for compiler, Cargo, **rustup**, every locked dependency, scanner, action and runners; raw-log and annotation warning inspection | **PASS with one `UPDATE NEEDED`** | version table below; the `rustup` mismatch is disclosed as `UPDATE NEEDED`, low urgency, and the raw-log limitation as `UNVERIFIED` |
| **G6** | per-host PASS versus host-conditional `NOT APPLICABLE`, `NOT RUN`/`UNVERIFIED` never counted as success; alert inventory never asserted clean | **PASS** with enumerated unknowns | step inventory and access register below |
| **Browser / HTTP / rendered / contact / release / deployment** | surfaces do not exist | **`NOT RUN` / `NOT APPLICABLE`** | no such code path exists in `crates/` |

### G1 — clean room and reproducibility

The instrumented run `42493698817` at `2d6a117` has **31 step slots per job** and completed with **zero failures**: Ubuntu 22 `success` / 9 `skipped` / 0 failed; Windows 19 `success` / 12 `skipped` / 0 failed. Every skip is the mirrored host-conditional counterpart of a step that ran on the other host, plus the three Ubuntu-only steps (advisory probe, forbidden-import refusal, two clean builds) — enumerated in G6.

The Ubuntu job builds the release binary twice, each into its own initially empty target directory, and compares the two binaries on raw bytes; that is the reproducibility proof, and it is Ubuntu-only. Windows byte-equality is **not** claimed: CI's Windows host is the default `x86_64-pc-windows-msvc`, so the tracked `[target.x86_64-pc-windows-gnu]` timestamp link-arg is inert there.

**Local reproduction remains `NOT RUN`, unchanged and not invented:** this Owner host has no MSVC Build Tools, no Visual Studio and no mingw/gcc/clang/ld, so `cargo build`, `cargo clippy … -D warnings` and `cargo test` cannot link (`error: linker 'link.exe' not found`). Acquiring a linker is a new external executable this order forbids. The previous run's first-hand local results stand as recorded: `cargo fetch --locked`, `cargo tree --locked --offline`, `cargo metadata --locked --offline` and `cargo fmt --all --check` each exited 0 in an isolated disposable root; the hosted two-host clean room satisfies this gate.

**Provenance:** the workflow's own statement, re-read this run, limits network-capable operations to checkout, `rustup toolchain install`, the single `cargo fetch --locked`, the scanner archive and advisory refresh, and the Ubuntu advisory fixture; ordinary build/lint/test/check commands then run with `CARGO_NET_OFFLINE=true` and `--locked`. That variable is a Cargo-scoped setting and is **not** claimed as a system-wide network denial.


### G2 — genuine minimal vertical slice

| Layer | Exact evidence |
| --- | --- |
| Composition root | `crates/solo-cli/src/main.rs` — `fn main()` calls `run(std::env::args_os(), &BootstrapPreflightService)`; `emit()` maps the typed outcome to `ExitCode` 0 accepted / 2 refused / 1 internal, with no validation rule in `main.rs` |
| Adapter boundary | `crates/solo-adapters/src/bootstrap_cli.rs` — `parse_request` builds `BootstrapInputV1`; `pub fn run<I>(args: I, preflight: &dyn BootstrapPreflightV1)` consumes the **port trait**, not a concrete service |
| Inbound port | `crates/solo-application/src/bootstrap_preflight.rs` — `pub trait BootstrapPreflightV1 { fn execute(&self, input: BootstrapInputV1) -> BootstrapOutcomeV1; }`, implemented by `BootstrapPreflightService`, with typed accepted/refused outcomes |
| Domain validation | `crates/solo-domain` — `EntityIdV1`, `EntityIdError`, `ENTITY_ID_MAX_BYTES` and the base-address candidate rules, `std` only |

Real tests: `crates/solo-cli/tests/cli_smoke.rs` (15 tests, including `required_success_case_canonicalizes_the_authored_base` and `required_refusal_case_reports_an_identity_code`), `crates/solo-adapters/tests/bootstrap_cli.rs`, `crates/solo-application/tests/bootstrap_preflight.rs` and the domain suites — all exercised by the all-target test step on both hosts, with the release binary's accepted/refused behaviour additionally proven by `Release CLI smoke on the built binary` on both hosts. The port is a required argument of `run`, so no application-free stub path exists. This proves the minimal preflight slice **only** — not JSON/Markdown publication, multi-Entity support, routing, a website or a released Installation.

### G3 — architecture, refusal and static gates

| Check | Evidence and result |
| --- | --- |
| Format | `Format check`, both hosts `success`; independently re-run locally earlier with exit 0 |
| Strict Clippy | `Strict lint (warnings are errors)` runs `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, both hosts `success` |
| All-target tests | `Test (workspace, all targets, locked)`, both hosts `success` |
| Real-tree boundary | `Architecture checker on the real checkout` per host, `success` — checks the actual checkout, not a mock |
| 23 negative fixtures | `pub const FIXTURES: [Fixture; 23]` in `crates/solo-cli/tests/architecture_checks/mod.rs`; `Architecture refusal self-test` per host `success`, and the workflow hard-asserts exactly 23 refused outcomes (`grep -c 'refused as ' … -eq 23`, `if ($refused -ne 23) { throw … }`) |
| Deliberate forbidden import | `Forbidden import and bypass refusal with guaranteed restore (Ubuntu)` `success`, annotation *"injected forbidden import refused with category `undeclared-crate` and exit 1"*; the step records the pre-injection digest, injects into `crates/solo-domain/src/lib.rs`, requires refusal, restores from backup, re-checks the digest and fails on any `git diff` or residue |
| Failure tripwire | `Test-failure tripwire` per host `success` — the unmodified target passes and a deliberate injection must fail the gate |
| Boundary shape | Declared edges from `cargo metadata`: `solo-domain` **0 dependencies**; `solo-application` → `solo-domain`; `solo-adapters` → `solo-application`, `solo-domain`; `solo-cli` → `solo-adapters`, `solo-application`, with `serde_json =1.0.154` and `syn =3.0.9` as **dev-dependencies only** |
| No premature shell | No provider, browser, HTTP server, routing or advanced contact-form subsystem exists in `crates/` (8,157 lines across 20 `.rs` files); the release binary has **zero third-party normal-dependency crates** |

Exit semantics make refusal observable rather than assumed: `EXIT_LEGAL = 0`, `EXIT_REFUSED = 1`, `EXIT_USAGE = 2`, and `EXIT_HARNESS_FAILURE = 3`, the last existing so a checker that *accepts* a forbidden fixture can never be mistaken for a pass.


### G4 — exact-version dependency and supply-chain security

**Independently recounted, not inherited:** the lockfile holds **15 packages = 4 first-party path crates + 11 registry crates**, every registry entry from `registry+https://github.com/rust-lang/crates.io-index`, and **every registry crate is dev-path only** — the shipped release binary has no third-party normal dependency. The ban inventory was re-parsed from `deny.toml`: **6 name-wide** (`proc-macro1`, `proc-macro-en`, `aovine`, `arone`, `aronenao`, `tinymember`) plus **3 exact-release** (`arrayref@0.3.10`, `internment@0.8.7`, `append-only-vec@0.1.9`) entries.

| Item | Exact evidence, this run |
| --- | --- |
| Advisory data | RustSec commit `309392b18f6b17022f82c701f300433126901068`, freshly fetched **`2026-11-08T15:13:34Z`** (Ubuntu) and **`15:13:49Z`** (Windows) |
| Scanner integrity | `cargo-deny 0.20.2`; Linux archive SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; Windows archive SHA-256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc` |
| Four policy classes | `advisories`, `bans`, `licenses`, `sources` — **all ok** on both hosts, on the unchanged locked graph, with `all-features = true` and every `targets` entry |
| Fail-closed posture | `ignore = []`, `yanked = "deny"`, `unmaintained = "all"`, `unsound = "all"`, `unused-ignored-advisory = "deny"`, `unknown-registry = "deny"`, `unknown-git = "deny"`; the one narrow option is `allow-wildcard-paths = true`, needed for path dependencies |
| Negative probe | `Advisory negative probe — RUSTSEC-2026-0009 (Ubuntu only)` `success`, annotation *"refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks"*; the fixture is a throwaway `$RUNNER_TEMP` manifest resolved with `cargo generate-lockfile`, never compiled into the product |
| Independent advisory query | OSV `querybatch` over all 11 registry crates at their exact locked versions, **`2026-11-08T15:17:14Z`**: **0** known vulnerabilities |
| Toolchain assertion | per-host step verifies `rustc 1.99.1` and `cargo 1.99.1`; `rustup` is measured separately by the new line — see G5, `UPDATE NEEDED` |

**Finding: none blocking.** No applicable critical or high severity advisory, no known-malicious, yanked or unsound use in the locked graph, no scanner failure, no suppression, `ignore` entry, reduced severity or missing target, and no hidden mandatory security-scan skip. Risk was not accepted on the Owner's behalf and `deny.toml` was not touched.

### G5 — version currency, warnings and provenance

| Component | In use | Latest supported | Dated authority | Disposition |
| --- | --- | --- | --- | --- |
| Rust toolchain | `1.99.1` | `1.99.1` | `rust-lang/rust` latest release, published 2026-10-16 | **current**; Rust has **no separate LTS track**, so there is none to prefer |
| Cargo | `1.99.1` (asserted per host) | ships with the toolchain | same release | **current** |
| **`rustup`** | **`1.29.2`** per both image readmes; actual printed value `UNVERIFIED` (logs 403) | **`1.29.3`** | [Rust Blog 2026-11-03](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/); image readmes read 2026-11-08 | **`UPDATE NEEDED`** — one patch behind, **low urgency**, no advisory or deprecation; not `current`, not a STOP |
| `itoa` | 1.0.18 | 1.0.18 | crates.io, 2026-03-20 | **current** |
| `memchr` | 2.8.3 | 2.8.3 | crates.io, 2026-07-08 | **current** |
| `proc-macro2` | 1.0.107 | 1.0.107 | crates.io, 2026-07-19 | **current** |
| `quote` | 1.0.47 | 1.0.47 | crates.io, 2026-07-19 | **current** |
| `serde`, `serde_core`, `serde_derive` | 1.0.229 | 1.0.229 | crates.io, 2026-07-18 | **current** |
| `serde_json` | 1.0.154 | 1.0.154 | crates.io, 2026-10-11 | **current** |
| `syn` | 3.0.9 | 3.0.9 | crates.io, 2026-10-12 | **current** |
| `unicode-ident` | 1.0.27 | 1.0.27 | crates.io, 2026-10-12 | **current** (notice obligation still open, see G6) |
| `zmij` | 1.0.23 | 1.0.23 | crates.io, 2026-07-13 | **current** |
| `cargo-deny` | 0.20.2 | 0.20.2 | `EmbarkStudios/cargo-deny` latest, 2026-07-09 | **current** |
| `actions/checkout` | v7.0.1 (full SHA `3d3c42e5aac5ba805825da76410c181273ba90b1`) | v7.0.1 | action latest release, 2026-07-20 | **current**; retires the older Node-20 platform notice |
| Node runtime | node24 via the pinned action | node24 | action release | **current** |
| `ubuntu-24.04` runner | in use | `ubuntu-26.04` image exists upstream | `actions/runner-images` listing, read 2026-11-08 | **`update-proposed`** — supported image, not EOL; a migration is a separate authorised order and was **not** made here |
| `windows-2025` runner | in use | `windows-2025` current GA (a `Windows2025-VS2026` variant exists) | same listing | **current** |

Fresh re-verification this run: all **11** crates re-queried against crates.io — **0** not at latest stable; OSV re-queried — **0** vulnerabilities. Every crate, the toolchain, the scanner and the action are current; the only currency debt is the installer above.

**Warnings — actual, not summarised.** Annotations at run `42493698817`: **three** on Ubuntu (forbidden-import refusal; advisory-probe refusal; component security gate passed) and **one** on Windows (component security gate passed). No dependency-update, deprecation, unsupported-runtime, end-of-life, unmaintained-component or incompatible-version warning appears, and none was hidden, downgraded or triaged away. The scanner, RustSec freshness, digests and policy outcomes are all corroborated above.

**Reported, not changed:** Architecture §6A's older "no code" phrasing remains out of step with the verified baseline. Repairing it is authorised by this order but is **sequenced after** the technical acceptance gate, so `ARCHITECTURE.md` is byte-identical at blob `4b050f63…` in this revision.


### G6 — limits, host-conditional skips and scope integrity

**Per-host step inventory, run `42493698817` (31 slots each):**

| Step group | Ubuntu | Windows |
| --- | --- | --- |
| `Set up job`, `Checkout (pinned full commit SHA, credentials not persisted)`, `Install and assert the pinned toolchain` (now also logging `rustup --version`), `Fetch locked packages`, `Format check`, `Strict lint`, `Test (workspace, all targets, locked)`, `Release build`, `Post Checkout`, `Complete job` | `success` | `success` |
| `Record revision and runner identity`, `Assert exact rustc/cargo versions`, component security gate for own platform, `Workspace package-set gate`, `Architecture checker on the real checkout`, `Architecture refusal self-test`, `Test-failure tripwire`, `Release CLI smoke`, `Job summary` | own variant `success`; other host's variant `skipped` | own variant `success`; other host's variant `skipped` |
| `Advisory negative probe — RUSTSEC-2026-0009 (Ubuntu only)` | `success` | `skipped` — `NOT APPLICABLE` |
| `Forbidden import and bypass refusal with guaranteed restore (Ubuntu)` | `success` | `skipped` — `NOT APPLICABLE` |
| `Two clean release builds compared byte-for-byte (Ubuntu)` | `success` | `skipped` — `NOT APPLICABLE` |

Every mandatory gate therefore ran on at least one host and the common gates on both; no step uses `continue-on-error`, masks an exit, or is written `|| true`.

**Access register — exact endpoints, never reported as "no alerts":**

| Endpoint | Status | Classification |
| --- | --- | --- |
| `…/dependabot/alerts?state=open` | **HTTP 401** `Requires authentication` | `UNVERIFIED` |
| `…/dependabot/alerts?state=dismissed` | **HTTP 401** `Requires authentication` | `UNVERIFIED` |
| `…/code-scanning/alerts?state=open` | **HTTP 401** `Requires authentication` | `UNVERIFIED` |
| `…/actions/jobs/{ubuntu,windows}/logs` | **HTTP 403** (both hosts, run `42493698817`) | log bodies unreadable — this is why the printed `rustup` value is `UNVERIFIED`; step records and annotations used as the substitute |
| `…/check-runs/{job}` `output` | empty `title`/`summary` | no substitute value available |

The scheduled Dependabot run stays **`NOT OBSERVED`** — an unobserved schedule is not a current scan. No repository setting, security feature or permission was read, changed or bypassed.

**`NOT RUN` / `NOT APPLICABLE`:** browser and HTTP behaviour, rendered-page and visual checks, numerical accessibility conformance, contact-form delivery, provider conformance, sitemap/routing, release, distribution and deployment — all because those surfaces do not exist at this stage.

**Stage-scope integrity:** ROADMAP blob `145f9d7a…` unchanged, 13 stage headings and 13 exit gates present, Stage 01 block intact, no `R01.D` and no completion history; NEXT `92fa562b…` and OUTLOOK `cb9c7a61…` unchanged; `ARCHITECTURE.md` `4b050f63…` unchanged; four WIP files and no subdirectory; Stage 02 not promoted and no Stage 02 order installed.

**Open prerequisites, correctly scoped:** `unicode-ident` 1.0.27's Unicode-3.0 third-party attribution/NOTICE remains a **release** prerequisite — neither waived nor counted as Stage 01 product delivery — plus the `rustup 1.29.2 → 1.29.3` installer update as a separately authorised, low-urgency item.

## Changed-path ledger for this evidence phase

| Path | Operation | Blob |
| --- | --- | --- |
| `.documents/WIP/workorder-active.md` | issuance-history commit only, **byte-for-byte unchanged**, still installed | `bbf3dfb19381c1ab37682244a3017b16f065dfa3` (17,649 B) |
| `.github/workflows/rust-verify.yml` | one added line, `rustup --version` | `5426c1f5…` → `1c0e450c0a4a50b6f22f547d8a7c646dce8b79b1` |
| `.documents/Reports/execution-report.md` | overwritten in place with this corrected evidence package | `99761df2…` → this revision |

Verified byte-identical, not merely absent from a short `git status`: `README.md`, `AGENTS.md`, both SOPs, `ARCHITECTURE.md`, `plan-roadmap.md`, `plan-next.md`, `plan-outlook.md`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/dependabot.yml`, `.github/SECURITY.md`, every crate manifest and all 20 `.rs` files, `LICENSE` and all notices. No dependency, pin, runner, scanner, policy, setting, branch, PR, merge, release or deployment was created or changed.


## Remaining authorised sequence — not executed by this phase

In the order's own terms, pending the Architect's `ACCEPT` against this revision:

1. **§4.4 Close Stage 01** — delete the complete Stage 01 block from ROADMAP, leaving the title and Stage 02–13 byte-identical; populate NEXT with the single `R02.A` candidate; repair only the narrowly identified §5A/§6A status contradictions in ARCHITECTURE; commit those three paths; verify the byte diff and two-host CI on the closure revision.
2. **§4.5 Restore neutral ACTIVE** — replace the installed order with the exact 40-byte neutral heading (blob `ea5f20b7…`); commit that one file.
3. **§4.6 Publish the final report** — overwrite this report with the actually executed sequence, the closure revisions, the final ledger and remaining warnings; it must not embed its own commit SHA, and final CI is reported in chat rather than pre-claimed.

On `REVISE` or `STOP` this report is corrected instead, and Stage 01 stays exactly where it is.

## Temporary material and cleanup

Nothing was written outside the ledgered paths and ordinary disposable locations. Scratch from the prior run (`.temp/w042`) was already removed; this run created no build caches, no downloaded toolchain, no on-disk logs, and no OS-temp files beyond the two commit-message texts, both deleted after use. Pre-existing non-task material — `.temp/rt`, `.temp/w032`, `.temp/w033`, `.temp/architecture-fixtures` — was preserved untouched. No secret, credential or personal data was read or exposed, and no destructive Git operation was performed.

## Honest summary for the independent reviewer

The Stage 01 engineering gate carries a first-hand `PASS` on hosted two-host evidence, and the two prior report defects are corrected rather than hidden. Two items are deliberately **not** clean: the `rustup` installer is `UPDATE NEEDED` (1.29.2 versus 1.29.3, low urgency, no advisory, exact printed value `UNVERIFIED` behind an HTTP 403 log restriction), and the GitHub alert inventories remain `UNVERIFIED` behind 401. Nothing in this phase closed Stage 01 — the ROADMAP Stage, NEXT, ARCHITECTURE and the installed ACTIVE are all untouched, pending the acceptance gate the order itself places here.

**`agent report ready`** — status `COMPLETE` for the evidence phase. No Agent `ACCEPT` is claimed, Stage 01 is retained, Stage 02 is not promoted, and the closure sequence awaits the Chief Architect's decision against this exact revision.

