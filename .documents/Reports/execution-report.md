# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 06:05–06:25 |
| Work order | **`SOLO-WO-036 R01`** — Accepted CI Reconciliation and Component-Security Gate |
| Nature | Administrative WIP reconciliation plus **read-only** security evidence. No source, test, manifest, lockfile, workflow, scanner, dependency, action pin or repository setting was changed. |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-036 R01` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / `main` / sole `origin` |
| Starting HEAD / tree | `283066652c71c848e4d68daa4d5499aeae0075a4` / `aa4364c264db5d423c18b598eba716179daf9dba` |
| Issuance-history commit (A) | `7cd22d30bdda5d697b0988f05b32c58df1528ac7`, tree `ee40991efa2808f751e5c75fafc82f36e7defaa1` |
| Reconciliation commit (B) | `39edf990c6923da8594eb9abae39fda5511b220e`, tree `64fe64b204a1c4e4c177dfea953daeb79a631990` |
| **Status** | **`COMPLETE`** for the authorized administrative reconciliation and read-only survey. **Not** an Architect `ACCEPT`, **not** a vulnerability-scan `PASS`, **not** an R01.C or Stage 01 closure, **not** a release. |

## Preflight

Verified before any write: one Solo checkout, one worktree, sole `origin` = the intended public repository, branch `main`, no stash, and a clean index except the Owner's replacement of `.documents/WIP/workorder-active.md`. All five reviewed baseline blobs matched with zero drift — Architect SOP `8760be323b6ed85695a9dcffd3e52f94628f0f07`, ROADMAP `6f753fdbfc34a08568f238769b558e93422688a6`, NEXT `92fa562b1af0c54391851aa5c79deb1631127069`, OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`, previous report `d12453a7560e302840b3e2266c96304b2562ac0d`.

The installed order was 14,888 bytes, blob `937d0cd12e4154ad7ed99a0d2af8b888fd80e97e`, LF-only, no BOM, carrying `OWNER-ISSUED` and the matching `SOLO-WO-036 R01` handoff identifier. It was committed **byte-identically** (commit A) and never edited by the agent.

## Committed revisions and scoped diff

| Commit | Content | Paths |
| --- | --- | --- |
| **A** `7cd22d3` | Owner-installed `SOLO-WO-036 R01` recorded unchanged as issuance history (committed ACTIVE blob = `937d0cd1`, identical to the installed file) | 1 |
| **B** `39edf99` | The authorized reconciliation | 3 |
| **C** | ACTIVE neutralized to exactly `# Foundation Solo — Active Work Order\n` (40 bytes, one LF, no BOM), performed only after B was published and both CI jobs had passed | 1 |
| **D** | This report, overwritten in place | 1 |

Diff against the starting commit touches **four** paths — `.documents/SOP/sop-architect.md`, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/workorder-active.md` — plus this report in its own final commit. Nothing else in the repository changed: the workflow, `Cargo.toml`, `Cargo.lock`, every crate manifest and source, `rust-toolchain.toml`, `.cargo/config.toml`, `README.md`, `AGENTS.md`, About/Architecture, Implementation SOP, `.github/` security files and OUTLOOK are byte-identical to the baseline.

## Documentary reconciliation (each change checked character-for-character against the order)

**`.documents/SOP/sop-architect.md` — §7B only.** The new **(e1) Current component-security and supply-chain assessment** paragraph is inserted immediately after **(e) Engineering hygiene** and immediately before **(f) Accessibility and performance**, matching the order's text exactly (1,745 characters, verified equal). Nothing else in §7B changed: the A1 preface, **(a)**–**(d)**, **(f)**–**(h)**, the scope-applicability paragraph, the disposition test and the existing A1 security standard are intact and un-renumbered. The obligation is forward-looking: it binds subsequent code-bearing reviews, including packages added later, and does not purport to certify the present repository.

**`.documents/WIP/plan-roadmap.md` — Stage 01 only.** The single `- **R01.C:**` remaining-package line is replaced, and the Stage 01 exit-gate line is replaced; both verified equal to the order's text. R01.C now names the outstanding security/advisory and supply-chain verification instead of re-describing the accepted Rust CI/tooling work, and the exit gate adds a fresh exact-version security assessment with an explicit NOT RUN browser statement. The remaining 154 lines — every other stage, package identifier, prerequisite, later-stage security obligation and exit gate — are unchanged. **R01.C and Stage 01 remain open.** No historical completion diary was added.

**`.documents/WIP/plan-next.md` — replaced.** Exactly the order's 11-line candidate, `R01.C — Version-Aware Component Security and CI Advisory Gate`, UTF-8, LF, single final newline (2,037 bytes; an extra trailing blank line introduced during editing was corrected so the file ends with one newline). It is a candidate only: non-executable until a separate Owner-issued order exists.

**`.documents/WIP/workorder-active.md`.** First the installed text as issuance history (A), then — after reconciliation, the path audit and successful publication checks — the sole neutral heading (C). No approved scope was ever rewritten.


## Component security and supply-chain survey (§4)

This is a **read-only, point-in-time survey**, not an automated audit and not a `PASS`. The Architect's preliminary 2026-11-08 notes were treated strictly as **research leads**; every row below was independently re-queried by the agent on 2026-11-08.

**Source freshness (as read this run).** RustSec advisory database at Git commit `309392b18f6b17022f82c701f300433126901068`, committed 2026-11-06T15:34:32Z — the same revision the Architect cited, and still the repository head. GitHub Advisory Database (the public OSV-mirrored feed) queried per package on 2026-11-08. crates.io registry metadata read per exact version on 2026-11-08.

**Graph shape.** Four first-party crates plus **eleven** registry packages. `solo-domain`, `solo-application` and `solo-adapters` declare **zero** external dependencies; every registry edge belongs to `solo-cli` and every one of them is a **dev/test** edge — `serde_json` and `syn` are declared `[dev-dependencies]` and the other nine are their transitive closure. Consequence: **runtime exposure of all eleven is nil**; they are compiled and executed only in the test/build graph. All eleven `source` entries are `registry+https://github.com/rust-lang/crates.io-index` — no git, path or third-party registry — and `.cargo/config.toml` contains no registry or source replacement. `build.rs` exists nowhere in the workspace, so no first-party build script runs; the proc-macro crates (`proc-macro2`, `quote`, `syn`, `serde_derive`) still execute compile-time code in the dev graph.

| Component | Version | Role | RustSec `309392b1` | GHSA/OSV 2026-11-08 | crates.io state | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| `itoa` | 1.0.18 | dev/transitive | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `memchr` | 2.8.3 | dev/transitive | no advisory directory | 0 advisories | not yanked; Unlicense OR MIT | `NO LISTED ADVISORY` |
| `proc-macro2` | 1.0.107 | dev/transitive (build-time proc-macro) | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `quote` | 1.0.47 | dev/transitive (build-time proc-macro) | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `serde` | 1.0.229 | dev/transitive | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `serde_core` | 1.0.229 | dev/transitive | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `serde_derive` | 1.0.229 | dev/transitive (build-time proc-macro) | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `serde_json` | 1.0.154 | **dev only**, declared test-only | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `syn` | 3.0.9 | **dev only**, architecture-checker parser | no advisory directory | 0 advisories | not yanked; MIT OR Apache-2.0 | `NO LISTED ADVISORY` |
| `unicode-ident` | 1.0.27 | dev/transitive | no advisory directory | 0 advisories | not yanked; (MIT OR Apache-2.0) **AND Unicode-3.0** | `NO LISTED ADVISORY`; NOTICE prerequisite open |
| `zmij` | 1.0.23 | dev/transitive | no advisory directory | 0 advisories | not yanked; MIT | `NO LISTED ADVISORY` |


**Provenance cross-check.** All eleven checksums reported by crates.io for these exact versions equal the `Cargo.lock` checksums character-for-character (`8f42a60c…` itoa, `cf8baf1c…` memchr, `985e7ec9…` proc-macro2, `1fbf4db1…` quote, `4148590a…` serde, `67dca2c9…` serde_core, `e7a5d712…` serde_derive, `e7e9cc8b…` serde_json, `d78c8dee…` syn, `a2c754d6…` unicode-ident, `29666d0a…` zmij). Ten are published by one long-established maintainer (`dtolnay`), one by `BurntSushi`; no package shows a recent publisher change or a look-alike name.

**Malicious / typosquatted components.** The Rust Security Response Team notice of 2026-08-20 (`https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/`, read this run) names `proc-macro1`, `proc-macro-en`, `aovine`, `arone`, `aronenao`, `tinymember` (all versions), plus `append-only-vec@0.1.9`, `arrayref@0.3.10` and `internment@0.8.7`. First-hand check: RustSec does carry entries for these — `RUSTSEC-2026-0265` (proc-macro1), `RUSTSEC-2026-0262` (append-only-vec), `RUSTSEC-2026-0260` (arrayref), `RUSTSEC-2026-0266`/`2021-0036`/`2020-0017` (internment). **None of those package names appears in the locked graph**, verified against the `Cargo.lock` inventory rather than inferred; `proc-macro2` is the legitimate distinct crate. `FINDING: none`.

**Toolchain.** `rust-toolchain.toml` pins `channel = "1.99.1"` with `rustfmt` and `clippy`. RustSec has no `crates/std` directory at `309392b1`, and GitHub advisories for package `std` returned zero, so the pinned compiler/std pair is `NO LISTED ADVISORY` at these two sources. The Architect's `CVE-2026-5223` lead was checked against the actual advisory — `GHSA-jq42-7mfv-hm57`, **medium**, "Cargo crates in third party registries can override the cached source of other crates", vulnerable range `< 0.97.0` (all Cargo shipped before Rust 1.96.0), and the advisory states explicitly that **crates.io users are not affected** because crates.io forbids symlinks. The pinned channel postdates the fix and the project uses only crates.io, so the finding is **`NOT APPLICABLE`** on both counts. Limitation: the toolchain binaries were **not executed** — no toolchain install was authorized — so version evidence is the pinned channel file, not `cargo --version` output.

**CI action, runners and build inputs.** The workflow uses immutably pinned `actions/checkout@08eba0b27e820071cde6df949e0beb9ba4906955` (v4.3.0) with `persist-credentials: false`, job permissions `contents: read`, no secrets and no other external action. First-hand: the action's repository publishes **zero** security advisories; its current release is `v7.0.1` (2026-07-20). Every run emits GitHub's Node-20 deprecation warning for the pinned v4.3.0 — a **follow-up upgrade review**, not evidence of an exploitable vulnerability, and no pin change is authorized here. Hosted `ubuntu-24.04` and `windows-2025` images are platform-managed and not digest-pinned by this repository: **`UNVERIFIED`** as an independently audited component.

**Licences.** All eleven are permissive (MIT, Apache-2.0 or Unlicense); `unicode-ident` additionally carries **Unicode-3.0**, so its attribution/NOTICE obligation remains an open release prerequisite recorded in Architecture §6C. No copyleft component exists, so no licence-policy exception is required.

**First-party attack surfaces.** Four crates, all carrying `#![forbid(unsafe_code)]` — no `unsafe` block exists. `src/` contains no process spawn, no network type, no filesystem write and no filesystem read; the CLI's only inputs are local paths supplied by the caller, and the test-only checker reads repository files and invokes fixed toolchain commands. Negative and refusal behaviour continues to be enforced by the 29-case fixture self-test and the injected forbidden-import probe.


## Checks deliberately not run

- **No automated advisory scanner ran.** `cargo audit`, `cargo deny`, `cargo vet` and OSV-Scanner are all **`NOT RUN`**: no scanner installation or execution is authorized by this order, and no automated security gate exists in CI — that absence is precisely the R01.C gap now recorded in ROADMAP and NEXT. The per-component findings above are a hand-checked survey against two public databases, not a lock-file scan.
- **No build, test, lint or metadata command ran.** No dependency, toolchain or action install, upgrade or pin change; `cargo --version`, `cargo metadata`, `cargo build`, `cargo test`, Clippy and rustfmt were **not** executed, because no executable content changed and no toolchain work is in scope.
- **OSV.dev could not be queried directly.** Its package query API is POST-only, which this read-only environment cannot issue; GitHub's advisory database, which feeds OSV for the Rust ecosystem, was used instead and its per-package results are recorded above. That substitution is a **source limitation, not a clearance**.
- **Not attempted:** SBOM generation, licence-policy evaluation, runner-image inventory or attestation, `rustc` release-note/CVE triage beyond RustSec and GHSA, and any scheduled or continuous re-check.

## Current security gaps (unchanged by this run)

1. CI contains **no** advisory or supply-chain gate, so a future vulnerable or malicious dependency would not fail the build. R01.C remains open for exactly this reason.
2. The pinned `actions/checkout` v4.3.0 is a superseded major with a live Node-20 deprecation warning; replacement needs its own authorized scope and verification.
3. Release attribution prerequisites are still unmet: copyright holders, `NOTICE` content including the Unicode-3.0 obligation for `unicode-ident`, a named Security Maintainer, and the release/support table.
4. Hosted runner images are not digest-pinned, so runner drift is outside this repository's control and unverified.
5. No SBOM and no dependency-source or licence policy check exists in CI.
6. This survey is **point-in-time** (2026-11-08). A new advisory published afterwards will not be noticed automatically, and no scheduled re-check exists.

## Publication verification

**Reconciliation revision `39edf99` — genuinely re-evaluated.** Run `42453986582`: `verify (ubuntu-24.04)` job `128648975130` **success** (06:15:25Z → 06:15:52Z, 27 s) and `verify (windows-2025)` job `128648975007` **success** (06:15:25Z → 06:16:15Z, 50 s). Run link: <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42453986582>. Every applicable mandatory step ran and passed; only host-inapplicable steps are `skipped`. A documentation-only change did not weaken or bypass any gate.

**Final published HEAD.** The ACTIVE-neutralization commit and this report commit are published after that verification; both jobs' conclusions at the exact final `HEAD` were re-checked and are reported in the chat handback, because the report cannot contain its own commit SHA. Remote `origin/main` equals the local HEAD, tree and both-host conclusions are recorded there.

**Warnings.** One warning class only: GitHub's platform notice that the pinned `actions/checkout` targets Node.js 20 and is forced onto Node.js 24. No `continue-on-error`, masked exit, `|| true` or broad `if` skip exists in the workflow.

## WIP state at handback

`SOLO-WO-036 R01` is executed and its slot is **neutralized** — ACTIVE holds only `# Foundation Solo — Active Work Order`, so no implementation authority is in force and this report grants none. NEXT holds exactly one non-executable candidate (the R01.C version-aware component-security and CI advisory gate) and is not executable without a separate Owner-issued order. ROADMAP still carries **R01.C** and the Stage 01 prerequisite/exit gate, both **open**. OUTLOOK is unchanged. WIP contains exactly its four files and no subdirectory, extra report or archive.

## Scratch, safety and scope

Task-owned scratch was confined to `.temp/w036` and removed before handback. Foreign material under `.temp/rt`, `.temp/w032` and `.temp/w033` (external-service `dotnet-suggest` sentinels) is preserved untouched; the Owner cleanup trigger is unchanged — delete them once no external service holds them. No repository content was deleted, reset, cleaned or rewritten; no clone, second worktree, branch, PR, force-update, dependency install, provider call, secret access, setting change, merge, release or deployment occurred. Changed paths are exactly the four ledger paths plus this report.

## Handback

No unresolved material security risk, false report or invalid WIP transition was found, and no mandatory CI check failed; nothing was suppressed or simulated. `cargo audit` and every other scanner remain **`NOT RUN`**, the browser/HTTP/Node/Playwright evidence remains **`NOT RUN`** for want of an authorized surface, and no release or deployment exists.

**Next step:** return **"agent report ready"** for independent Chief Architect review of the exact final published revision.

**`.documents/WIP/plan-outlook.md`.** Untouched. WIP holds exactly its four files and no subdirectory or extra report.
