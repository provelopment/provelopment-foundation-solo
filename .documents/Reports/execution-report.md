# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 06:35–07:05 |
| Work order | **`SOLO-WO-037 R01`** — R01.C Version-Aware Component Security and CI Advisory Gate |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-037 R01` |
| Installed ACTIVE blob | `8b0e81db7f743a2fc8d3c10b771f82284499d946` (23,412 bytes, 101 lines, LF, no BOM), retained **unchanged** |
| Starting HEAD / tree | `f7127800cc6811763fd63710fc6dca2a6fc5c244` / `4b72cd50ce7a47b874405f7efcbc857ff548cca2` |
| Final source HEAD / tree | `5ebdcc942ea33599ed39cc62736a677826a2ecd7` / `4e971dc17e6163dbe0767ca6abc96eee60a17e3a` |
| **Status** | **`COMPLETE`** — the security gate is implemented, executed on both hosts and demonstrated refusing a real advisory. **Not** an Architect `ACCEPT`, **not** R01.C or Stage 01 closure, **not** a release, and **not** a certification that the product has no vulnerabilities. |

## Preflight (all verified before any write)

Solo root verified as the intended Owner-local checkout, branch `main`, one worktree, no stash, sole `origin` = the intended public repository, remote `main` equal to local HEAD, and a clean index apart from the single Owner installation of ACTIVE. All fifteen baseline blobs matched with **zero drift**: `README.md`, `AGENTS.md`, Architecture, both SOPs, all four WIP files, the previous report, the workflow, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` and `.cargo/config.toml`. `deny.toml` did **not** exist, and no parallel security workflow or scanner policy was present — `.github/workflows/` contained exactly `rust-verify.yml`.

Recorded before any change: `Cargo.lock` file SHA-256 `0f78cd0606141ca2497da75e359d8082364f0d9bdc422ba7f470c4cef2031257` (blob `05a02666894f4f0bf5528acf118a049e5edb853e`), toolchain pin `1.99.1` (`rust-toolchain.toml` blob `9ec55b3b`), prior checkout pin `actions/checkout@08eba0b27e820071cde6df949e0beb9ba4906955` (v4.3.0), matrix `ubuntu-24.04` + `windows-2025`.

## Committed revisions and scoped diff

| Commit | Content |
| --- | --- |
| **A** `930968dd730937efbdd2e03097b887ab92d7cd4b` | Owner-installed `SOLO-WO-037 R01` recorded byte-identically as issuance history |
| **B** `5236a218fc725d059a3edb8203260b745ed51b65` (tree `6d5ec3e4…`) | `deny.toml` created; the single workflow extended; NEXT neutralised |
| **C** `ba0b87e2f58fc216732c50de03ab997b9e4ba3c3` (tree `56af7046…`) | policy-scope correction after the first real run refused the gate (see below) |
| **D** `5ebdcc942ea33599ed39cc62736a677826a2ecd7` (tree `4e971dc1…`) | notice-annotation observability for the gate and the probe |
| **E** | this report — final published revision |

Diff against the starting commit touches exactly the four permitted source paths — `deny.toml` (new), `.github/workflows/rust-verify.yml`, `.documents/WIP/plan-next.md`, `.documents/WIP/workorder-active.md` — plus this report. **Preserved byte-identically:** `Cargo.toml` `14505be4…`, `Cargo.lock` `05a02666…` (and its file digest above, unchanged before and after the gate on both hosts), `rust-toolchain.toml` `9ec55b3b…`, `.cargo/config.toml`, every crate manifest, source and test, `README.md`, `AGENTS.md`, About/Architecture, both SOPs, ROADMAP `e6ea66cf…`, OUTLOOK `cb9c7a61…`, `.github/SECURITY.md` and the root licence. No second workflow, fixture project, scanner cache, archive or report was committed.


## What was implemented

**`deny.toml` (new, root).** One reviewed `cargo-deny 0.20.2` policy, authored from that exact release's own `init` template rather than from memory, and validated against the 0.20.2 binary before use. All four classes are enabled and none is suppressed: **advisories** fetch the official RustSec database on every run, deny yanked locked versions, treat unmaintained and unsound notices as failures for any crate in the graph, and set `unused-ignored-advisory = "deny"` with an empty `ignore` list; **bans** deny several versions of one crate, deny every wildcard requirement, and ban the six known 2026 malicious package names outright plus only the three compromised releases of otherwise legitimate packages (`arrayref@0.3.10`, `internment@0.8.7`, `append-only-vec@0.1.9`), with empty `allow`, `skip` and `skip-tree`; **licenses** allow exactly `Apache-2.0`, `MIT`, `Unlicense` and `Unicode-3.0` with no exceptions and no private-crate opt-out; **sources** deny unknown registries and any git source, allowing only `https://github.com/rust-lang/crates.io-index`. `[graph]` covers the Linux GNU/musl and Windows GNU/MSVC triples and enables all features; dev and build dependencies are never excluded.

**The single workflow.** A component security gate step runs on **both** matrix hosts immediately after the pinned toolchain and the locked product fetch. It downloads only the host's official `cargo-deny 0.20.2` GitHub release archive, verifies the **raw archive SHA-256 before extraction**, rejects absolute, drive-qualified or parent-traversing member names, extracts into a freshly created runner temporary directory, and asserts the reported version is exactly `cargo-deny 0.20.2`. It then fetches the current RustSec database, resolves the fetched commit, and runs `advisories`, `bans`, `licenses` and `sources` **separately** against the real workspace with `--config deny.toml --manifest-path Cargo.toml --locked --all-features`, so a failure names its class and cannot be masked. Afterwards it re-asserts that `Cargo.lock` is byte-identical and that `git status --porcelain` is empty. A failed download, a digest mismatch, an unsafe archive member, a wrong version, a failed advisory fetch, missing data or any finding fails the job.

An **Ubuntu-only negative probe** proves detection rather than description: it resolves a disposable `time =0.3.46` manifest in `$RUNNER_TEMP` (no Git repository, no production crate, the crate is never compiled) with `cargo generate-lockfile`, then runs the same scanner with the same policy file and requires a **nonzero exit together with the `RUSTSEC-2026-0009` diagnostic and an advisory-class error**. An unexpected pass, a missing advisory or an unrelated failure fails the step; the fixture is deleted afterwards.

The workflow also gains a daily off-the-top-of-the-hour `schedule` (`17 4 * * *`) and `workflow_dispatch`; upgrades **only** the existing `actions/checkout` full-SHA pin to the verified `v7.0.1` commit `3d3c42e5aac5ba805825da76410c181273ba90b1` (which declares `runs.using: node24`), keeping `persist-credentials: false` and `permissions: contents: read`; and corrects the network narrative in the header, the `env` comment and both job summaries so that checkout, toolchain installation and the security gate's own data refresh are named as networked while all application Cargo work stays offline under `CARGO_NET_OFFLINE=true`. The accepted `Browser smoke: UNVERIFIED / NOT RUN` statement is retained, and `cargo-deny` was removed from the "not covered" list only because it is now genuinely covered.

## Failures during this run, disclosed

**First real run of the new gate failed on both hosts** (run `42455972978`, jobs `128654950888` Ubuntu and `128654950766` Windows, commit B). The annotation named the cause exactly: `cargo-deny check bans failed for the locked workspace graph`. This was a defect in the new policy file, not a dependency finding — with `wildcards = "deny"` and `allow-wildcard-paths` at its default, cargo-deny also reports **path** dependencies carrying no version requirement, and the four private first-party crates depend on each other by path. Commit C adds `allow-wildcard-paths = true`, the documented option scoped to path/git requirements in private crates and path/git dev-dependencies; registry requirements remain governed by `wildcards = "deny"`, `bans.allow` stays empty, no finding is suppressed, and the advisories class had already passed before the failure, so no advisory was bypassed. Commit D then added notice annotations to make the gate and probe results publicly verifiable instead of log-only.

One intermediate local edit of this run briefly misplaced a notice inside a multi-line `grep … || fail` construct; `bash -n` caught it before any push, it was repaired, and no broken revision was ever published.


## Tool, action and environment provenance

| Component | Identity | Verification |
| --- | --- | --- |
| Scanner | `cargo-deny 0.20.2` (Architecture §6A.2 selection), release published 2026-07-09T17:58:46Z | `--version` asserted in CI on both hosts |
| Ubuntu archive | `cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz`, 4,936,832 bytes | SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f` confirmed from the release API **and** from the archive itself in CI before extraction |
| Windows archive | `cargo-deny-0.20.2-x86_64-pc-windows-msvc.tar.gz`, 4,759,363 bytes | SHA-256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc` confirmed from the release API, the official `.sha256` sidecar and a local download used for the dry run |
| Archive contents | one top-level directory containing `cargo-deny(.exe)`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md` | member names validated for traversal and unsafe characters before extraction |
| Advisory data | RustSec `rustsec/advisory-db` at commit `309392b18f6b17022f82c701f300433126901068`, committed 2026-11-06T15:34:32Z | fetched fresh in CI: **06:53:55Z** (Ubuntu) and **06:54:13Z** (Windows) on 2026-11-08; commit resolved from the fetched clone, not cached |
| Checkout action | `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (= tag `v7.0.1`, 2026-07-20) | tag→commit resolved from the Git ref API; `action.yml` at that commit read directly: `runs.using: node24`, `persist-credentials` still overridden to `false`; both hosted images ran it successfully, which is the compatibility evidence for the runner requirement |
| Toolchain | Rust/Cargo `1.99.1`, pinned by `rust-toolchain.toml` | the pre-existing version assertion step (`rustc 1.99.1 …`) passed on both hosts |
| Runners | GitHub-hosted `ubuntu-24.04` and `windows-2025` | platform-managed, **not** digest-pinned by this repository |

No other action, tool, service or dependency was introduced.

## Gate execution evidence (real, at the published revisions)

Both hosts, run [`42456575109`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42456575109) at source HEAD `5ebdcc9`:

- **Ubuntu** job [`128656767994`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42456575109/job/128656767994) — `success`, 25 s.
- **Windows** job [`128656767838`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42456575109/job/128656767838) — `success`, 59 s.

Public notice annotations recorded by that run (retrievable from the check-run API without an authenticated log fetch) state, verbatim:

- Ubuntu: `component security gate passed: cargo-deny 0.20.2 archive sha256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T06:53:55Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`
- Windows: `component security gate passed: cargo-deny 0.20.2 archive sha256 975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T06:54:13Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`
- Ubuntu: `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks`
- Ubuntu: `injected forbidden import refused with category 'undeclared-crate' and exit 1` (the previously accepted architecture gate, still intact)

**Known-vulnerable fixture.** Disposable manifest `advisory-negative-probe` in `$RUNNER_TEMP`, dependency `time = "=0.3.46"`, lock resolved with `cargo generate-lockfile` (the crate is never compiled or executed). `RUSTSEC-2026-0009` — "Denial of Service via Stack Exhaustion", CVSS 4.0 (`VC:N/VI:N/VA:H/SC:N/SI:N/SA:H`), aliases `CVE-2026-25727` and `GHSA-r6v5-fh4h-64xc`, affected `>=0.3.6, <0.3.47`, patched `>=0.3.47` — genuinely covers `0.3.46`, verified from the advisory file itself before the run. The gate refused it with exit **1** and the advisory-specific diagnostic, while the real locked graph passed all four checks: detection proven, not simulated.


## Component-by-component assessment (final source revision)

RustSec advisory data at `309392b1` (2026-11-06) as fetched by the gate; GitHub Advisory Database (the public OSV-mirrored feed) re-queried per package on 2026-11-08 at approximately 06:55Z; crates.io registry metadata re-read 2026-11-08. Path to inclusion: `solo-domain`, `solo-application` and `solo-adapters` declare **no** external dependencies, so every registry package below is reached only through `solo-cli` **dev/test** edges — `serde_json` and `syn` are declared `[dev-dependencies]`, the other nine are their transitive closure. **Runtime exposure of all eleven is nil.** All eleven have `source = registry+https://github.com/rust-lang/crates.io-index` and their crates.io checksums equal the `Cargo.lock` checksums.

| Component | Version | Role | Reachability | RustSec `309392b1` | GHSA/OSV 2026-11-08 | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| `itoa` | 1.0.18 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `memchr` | 2.8.3 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `proc-macro2` | 1.0.107 | dev/transitive, build-time proc-macro | compile-time in test graph | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `quote` | 1.0.47 | dev/transitive, build-time proc-macro | compile-time in test graph | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `serde` | 1.0.229 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `serde_core` | 1.0.229 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `serde_derive` | 1.0.229 | dev/transitive, build-time proc-macro | compile-time in test graph | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `serde_json` | 1.0.154 | declared `[dev-dependencies]` | test only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `syn` | 3.0.9 | declared `[dev-dependencies]`, checker parser | test only | no entry | 0 advisories | `NO LISTED ADVISORY` |
| `unicode-ident` | 1.0.27 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY`; licence carries **Unicode-3.0**, whose attribution/`NOTICE` obligation stays open |
| `zmij` | 1.0.23 | dev/transitive | test/build only | no entry | 0 advisories | `NO LISTED ADVISORY` |

| Non-registry component | Identity | Assessment |
| --- | --- | --- |
| First-party crates (4) | `solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`, all `publish = false`, licence `Apache-2.0` | All carry `#![forbid(unsafe_code)]`; `src/` contains no process spawn, no network type and no filesystem write. Surfaces: CLI argument parsing and refusal, and a test-only source scanner that reads repository files and runs fixed toolchain commands. No credential, secret or network input is read. |
| Rust std / compiler / Cargo / rustup | `1.99.1` pinned | No `crates/std` directory in RustSec at this revision and no GHSA advisory for package `std`; `CVE-2026-5223`/`GHSA-jq42-7mfv-hm57` (Cargo symlink cache override, **medium**) affects Cargo before Rust 1.96.0 and explicitly **not** crates.io users — **not applicable** to this pin and registry. rustup itself is exercised only by the pre-existing toolchain step. |
| Scanner binary and artifact | `cargo-deny 0.20.2`, official release archives | No RustSec entry and no GHSA advisory for `cargo-deny`. Provenance by immutable release asset + SHA-256 + version assertion. Residual risk: a statically linked third-party binary is executed in CI; it runs with the job's read-only token and no secrets, and its managed advisory/index cache lives only in runner temporary space. |
| `actions/checkout` | `v7.0.1`, commit `3d3c42e5…` | The action's repository publishes **zero** security advisories. Pinned by commit, credentials not persisted, `contents: read`, no `pull_request_target` job, no secrets, no other action. |
| Hosted runners | `ubuntu-24.04`, `windows-2025` | Platform-managed and not digest-pinned here: **`UNVERIFIED`** — no image inventory or attestation is available to this repository. |
| Build scripts / proc macros | none first-party; `proc-macro2`, `quote`, `syn`, `serde_derive` are compiled in the dev graph | Compile-time code execution is confined to the test graph; the workspace contains no `build.rs`. |
| Configuration | `deny.toml`, `.cargo/config.toml`, `rust-toolchain.toml` | Reviewed above; no registry or source replacement, no scanner cache or exception committed. |
| External services | GitHub Actions, the cargo-deny release host, the RustSec Git remote | No other service, account, secret or upload; the workflow uploads nothing. |

The known 2026-08-20 supply-chain incident (`proc-macro1`, `proc-macro-en`, `aovine`, `arone`, `aronenao`, `tinymember`, plus `arrayref@0.3.10`, `internment@0.8.7`, `append-only-vec@0.1.9`) is covered twice: RustSec holds live entries for those names (`RUSTSEC-2026-0265`, `0260`, `0262`, `0266`, `2021-0036`, `2020-0017`), none of them appears in the locked graph, and `deny.toml` now bans them permanently so a future reintroduction fails the build.


## Regression evidence for the previously accepted gates

The security gate was inserted **after** the pinned toolchain and locked fetch and **before** the existing gates, and every previously accepted mandatory step is retained verbatim: locked offline fetch, four-crate workspace/package-set gate, `cargo fmt --check`, strict Clippy with `-D warnings`, `cargo test --workspace --all-targets --locked`, the real-tree architecture checker, the 23-fixture refusal self-test, the test-failure tripwire (normal pass plus required injected failure), the locked release build, the byte-exact accepted/refused CLI smoke, the Ubuntu forbidden-import injection with guaranteed restore, and the two clean release builds compared byte-for-byte. YAML parsing confirms **28 steps** (the original 25 plus the three new ones), unchanged triggers `push`/`pull_request` on `main`, unchanged `permissions: contents: read`, and both matrix hosts. No `continue-on-error`, no masked exit, no `|| true`, no broad `if` skip was added; the only skipped steps are host-inapplicable ones.

| Revision | Run | Ubuntu | Windows | Outcome |
| --- | --- | --- | --- | --- |
| `5236a21` (B) | `42455972978` | job `128654950888` | job `128654950766` | **failed** — bans policy scope (see disclosure) |
| `ba0b87e` (C) | `42456225687` | job `128655708810` success 31 s | job `128655708949` success 71 s | green, both hosts |
| `5ebdcc9` (D) | `42456575109` | job `128656767994` success 25 s | job `128656767838` success 59 s | green, both hosts, with the annotations quoted above |

## Checks NOT run / UNVERIFIED

- **No local Rust execution.** This machine has no `cargo`, `rustc` or `rustup`, so no local build, test, Clippy, `cargo metadata` or `cargo-deny check` could run; that is `NOT RUN` locally, and CI supplied the executed evidence instead. Locally I did verify both archive digests, extracted and ran the scanner (`cargo-deny 0.20.2`), confirmed the policy deserialises under the real binary (a deliberately invalid key produced a distinct deserialization error), and dry-ran both host scripts — the gate correctly **failed closed** locally precisely because `cargo` was absent.
- **Direct OSV.dev queries** were not possible: its package query API is POST-only, which this environment cannot issue. GitHub's OSV-mirrored advisory database was queried per package instead; that substitution is a source limitation, not a clearance.
- **Hosted-runner internals** — image inventory, build number, patch level — are `UNVERIFIED`; only the image labels and the successful executions are observed.
- **Scheduled execution** has not been observed. The `schedule` trigger exists and is syntactically valid, but a cron run cannot be claimed until a future scheduled run actually occurs; the evidence in this report is from `push` runs.
- **Raw job log text** was not retrievable without an authenticated token, so step-level results are evidenced by the check-run conclusions plus the notice annotations the workflow itself emits; the annotations quote the verified digests, versions, advisory-database commit and refusal results.
- **Browser/HTTP/Page** evidence remains `UNVERIFIED / NOT RUN`: no authorized surface exists, and none was invented.

## Residual gaps and open obligations

1. `cargo-deny` covers advisories, bans, licences and sources for the Cargo graph and known-bad packages. It is **not** a whole-system audit: hosted runner images, GitHub Actions infrastructure, `rustup` distribution and the scanner's own internals remain outside it.
2. A green gate is not a proof of absence of vulnerabilities; it is a dated check against a specific database revision. Freshness is enforced per run, and the daily schedule exists precisely so that a new advisory can fail this repository without a code change.
3. The release attribution prerequisites are still unmet: actual copyright holders, `NOTICE` content including the `unicode-ident` **Unicode-3.0** obligation, a named Security Maintainer, and the published release/support table.
4. No SBOM and no independent (non-RustSec) advisory feed is wired into CI; corroboration is done per review, as in this report.
5. **R01.C and Stage 01 remain open.** This run adds the verification capability; independent Architect acceptance and separate Owner closure are still required, and no release, deployment, provider or settings action occurred.

## Cleanup and safety

Task-owned scratch lived only in `.temp/w037` (downloaded archives, extracted scanner, extracted script copies, dry-run temporaries) and was removed before handback. Foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` is preserved untouched. The scanner also populated a RustSec advisory cache in the standard user Cargo cache during local analysis — a tool-managed cache outside the repository, not tracked and not part of Solo. No clone, second worktree, branch, PR, merge, force-update, dependency install or upgrade, secret access, settings change, release or deployment occurred; changed permanent paths are exactly the four permitted source paths plus this report.

## Handback

There was no unresolved applicable critical/high or malicious finding, no checksum or provenance ambiguity, no failing mandatory host job at the final revision, and nothing was suppressed. `COMPLETE` describes this run's scope only: **"agent report ready"** for independent Chief Architect review at the final published revision.

**WIP state.** ACTIVE is the **unchanged** Owner-installed order (blob `8b0e81db…`, not rewritten, not cleared, not self-accepted). NEXT is the neutral identifying heading, reproducing its historical blob `92fa562b1af0c54391851aa5c79deb1631127069` exactly. ROADMAP still carries R01.C and the Stage 01 prerequisite/exit gate. OUTLOOK unchanged. Exactly four WIP files, no subdirectory.
