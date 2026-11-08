# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 11:55–12:25 |
| Work order | **`SOLO-WO-040 R01`** — Permanent A1 Version Currency and Safe Upgrade Process |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-040 R01` |
| Installed ACTIVE blob | `dbbb2b4d88b3d8856c7f963c78f980db092a5ada` (18,311 bytes, 90 lines, LF, no BOM), retained **unchanged** |
| Starting HEAD / tree | `828aafb1b9c2e741b60980b747b1048d9f892edc` / `4b10fedcb552f41c82f86acde8c376ba1ca77690` |
| Issuance record commit | `5f51832df320c7b2aa8ad1e91d079265f6157381` |
| **Source revision** | `5e7c513fbea46524769431ae81916e8c8fb42d76`, tree `16b53bbd0f6a21c76572035f454664a830f84a2e` |
| `.github/dependabot.yml` blob | new — `1874726fc2406575605b3c7ac467f45402fb7f13` (2,163 bytes) |
| **Status** | **`COMPLETE`.** Governance, monitoring configuration and truthful documentation only. Not a package upgrade, not a release, not an Architect `ACCEPT`. |

## Preflight

Intended Solo root, branch `main`, one worktree, no stash, sole `origin` equal to remote `main`, index clean apart from the single Owner installation of ACTIVE. All **fourteen** stated baseline blobs matched with **zero drift**: `AGENTS.md` `32562b67…`, `README.md` `3f3005d6…`, `sop-architect.md` `54c9676e…`, `sop-implementation.md` `a5771160…`, `ARCHITECTURE.md` `03e8923f…`, workflow `5426c1f5…`, pre-install ACTIVE `fded5f6e…`, ROADMAP `e6ea66cf…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…`, prior report `b4a2c8d3…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`. `.github/dependabot.yml` was **absent**, as stated. Exactly four WIP files, no WIP subdirectory. `README.md`, `AGENTS.md`, both SOPs, Architecture §6A.2 and §6C, all four WIP files, the prior report and the workflow were read before writing.

## The authorized change — six paths

| Path | Change |
| --- | --- |
| `.documents/SOP/sop-architect.md` | §7B gains one permanent rule, **(e2)**, beside the existing (e1); edition → revision 7 |
| `.documents/SOP/sop-implementation.md` | A1 cross-reference extended and a new **§7B** operational procedure; edition → revision 7 |
| `.documents/Architecture/ARCHITECTURE.md` | one narrow paragraph inside §6A.2 |
| `AGENTS.md` | A1 pointer extended to §7B(e2); stale current-state claim replaced |
| `README.md` | two current-status passages aligned to the verified baseline |
| `.github/dependabot.yml` | **new**, minimal proposal-only weekly monitoring |

Nothing else changed: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, the workflow, `.github/SECURITY.md`, `LICENSE`, all four crate manifests, ROADMAP, NEXT and OUTLOOK are byte-identical to the starting revision, and the workflow contains no reference to Dependabot.

## A1 policy adoption — one authority, no duplication

- **Architect SOP is the single authority.** Its §7B contains exactly one A1 heading and exactly one **(e2) Version currency, LTS and safe upgrade — permanent** rule, placed inside §7B between (e1) and (f) — not as a competing section. It binds: freshness as a tested attribute; recording source, query date, installed/selected and candidate versions with a disposition; never presenting an architecture-selected but uninstalled pin as installed; latest verified stable or latest **suitable** LTS (Node) with no invented Rust LTS; zero unresolved actionable version/deprecation/unsupported-runtime/EOL notices with each genuine warning recorded; exact-version/SHA/digest locking with no floating `latest`; product contract versions kept distinct from dependency versions; the discover→propose→authorized-update→exact-head-verification→independent-acceptance→separate-merge lifecycle with a recovery path; and that a Dependabot proposal is an input, never an upgrade that already passed.
- **Implementation SOP adds only operational steps** and states it is not a parallel policy: it defers to A1 §7B(e2), keeps §5B/§6B as the sole version-control procedure, and covers inventory, research, warnings, proposal, authorized update, exact-head verification, rollback and cadence. It adds no register — the ledger lives in this report.
- **Architecture stays explanatory.** The single added paragraph in §6A.2 ties its conditional exact pins to that periodic review, states that a row binds only when its gate proves it on a real revision, that nothing in the paragraph reclassifies any listed `E01` component, and that product contract versions are not dependency versions. All fifteen pinned `E01` rows are unchanged, and §6C.3/§6C.7 product support and release policy are untouched.
- **`AGENTS.md` points, it does not legislate:** "(Architect SOP §7B) with its permanent version-currency, LTS, warning and safe-upgrade rule (§7B(e2)), and the agent obligations that mirror and operationalize it (Implementation SOP §7A and §7B)".

## Installed-versus-current ledger (queried 2026-11-08)

Sources: crates.io API per crate; `github.com/rust-lang/rust` tags; EmbarkStudios `cargo-deny` releases; `actions/checkout` release metadata and its `action.yml`; `nodejs.org/dist/index.json` and `endoflife.date/api/nodejs.json`; `actions/runner-images` README and issue #14748; OSV `/v1/querybatch`. A green security scan is **not** treated as freshness evidence anywhere below.

| Component | Installed / selected | Latest stable or supported (source, 2026-11-08) | Disposition |
| --- | --- | --- | --- |
| First-party crates | `solo-domain`, `solo-application`, `solo-adapters`, `solo-cli` all `0.1.0`, `publish = false` | not published — internal | **current** (internal, no upstream) |
| `itoa` | 1.0.18 | 1.0.18 (crates.io, updated 2026-03-20) | **current** |
| `memchr` | 2.8.3 | 2.8.3 (2026-07-08) | **current** |
| `proc-macro2` | 1.0.107 | 1.0.107 (2026-07-19) | **current** |
| `quote` | 1.0.47 | 1.0.47 (2026-07-19) | **current** |
| `serde` | 1.0.229 | 1.0.229 (2026-07-18) | **current** |
| `serde_core` | 1.0.229 | 1.0.229 (2026-07-18) | **current** |
| `serde_derive` | 1.0.229 | 1.0.229 (2026-07-18) | **current** |
| `serde_json` | 1.0.154 (direct, `=1.0.154`) | 1.0.154 (2026-10-11) | **current** |
| `syn` | 3.0.9 (direct, dev/architecture tool) | 3.0.9 (2026-10-12) | **current** |
| `unicode-ident` | 1.0.27 | 1.0.27 (2026-10-12) | **current** |
| `zmij` | 1.0.23 | 1.0.23 (2026-07-13) | **current** |
| Rust / Cargo channel | `1.99.1` (`rust-toolchain.toml`, `rustfmt` + `clippy`), MSRV `rust-version = "1.99"`, edition 2024 | `1.99.1` is the newest stable tag | **current** |
| `cargo-deny` | `0.20.2`, verified archive digests `9f12ed4c…` / `975a2214…` | `0.20.2` released 2026-07-09 | **current** |
| `actions/checkout` | `v7.0.1` pinned by full SHA `3d3c42e5…`, `runs.using: node24` | `v7.0.1` released 2026-07-20 | **current** |
| Action Node runtime | `node24` (bundled in the action) | Node 24 LTS, EOL 2028-04-30 | **current**, supported |
| `ubuntu-24.04` runner | Ubuntu 24.04.5 LTS image | `ubuntu-26.04` is GA; `ubuntu-latest` migrates to 26.04 across 2026-10-19 → 2026-11-19 (runner-images #14748) | **outdated but supported → proposal**, not changed here |
| `windows-2025` runner | Windows Server 2025 image | `windows-2025` is the newest Windows Server x64 label; no `windows-2026` exists | **current** |
| Node.js application toolchain | **not installed** — §6A.2 selects `26.13.0` LTS | `v26.13.0` is the newest LTS (Lithium, LTS since 2026-10-28, EOL 2029-04-30) | **selected-but-not-installed**; equalling the newest LTS is not an installation |
| TypeScript `7.0.2`, Vite `8.3.6`, Playwright `1.64.1`, `@axe-core/playwright` `14.4.0`, `ts-rs` `12.0.1` | **not installed** (future browser stage) | not assessed — nothing installed to query | **selected-but-not-installed**, `UNVERIFIED` |
| RustSec / OSV | gate DB commit `309392b1…`; OSV batch over the exact 11 crates | **0** advisories returned for the locked set | **clear** for this revision |
| Dependabot security alerts | repository feature | API returned **401** to this agent | **`UNVERIFIED`** — no access, not claimed clear |

**No upgrade is required or proposed on version grounds**, because every installed component is already the latest stable or supported release. One **proposal** follows from runner currency, not from a defect: evaluate moving the Linux matrix entry from `ubuntu-24.04` to `ubuntu-26.04` in a separate Owner-authorized order, weighing the documented breaking differences (removed Java 8, systemd 259, kernel 7.0, newer system headers that break some cross-compiles) against this workflow's actual needs. It is **not** performed here, and the explicit `ubuntu-24.04` pin is deliberately preferred over `ubuntu-latest`, which is itself mid-migration.


## Dependabot configuration — validated, and already live

The file is schema `version: 2` with **exactly two** ecosystems rooted at `/`: `cargo` and `github-actions`. Each is `weekly` on Monday at 04:30 and 04:45 `Etc/UTC`, on the default branch (`target-branch` omitted), with `open-pull-requests-limit: 3`. There is no `ignore`, no `groups`, no `registries`, no `allow`, no `versioning-strategy`, no credential and no auto-merge key. Because `open-pull-requests-limit: 0` is what disables an ecosystem, a bound of 3 keeps both active while capping proposals. Dependabot **security** updates are a separate repository feature with their own fixed limit of ten open pull requests and are neither configured nor disabled here.

Checked against the official `dependabot.yml` options reference, the supported-ecosystems reference and the configure-version-updates how-to: **44 of 44** assertions passed, covering required keys, documented `schedule.interval` values, `hh:mm` time, IANA timezone, default branch, the per-ecosystem bound, and the absence of every forbidden key.

**Live evidence, not just schema validity.** Adding the file triggered GitHub's Dependabot update jobs, which both completed `success` on the source revision ([job `128727737592`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42479369649/job/128727737592), [job `128727737337`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42479369510/job/128727737337)) — the configuration was accepted and executed. It produced **no pull requests**: the repository has **0** pull requests in any state, consistent with the ledger above showing nothing outdated. No PR was manufactured, approved, merged or claimed.

**What this file cannot see**, and therefore stays in the manual weekly review the new SOP requires: the `rust-toolchain.toml` channel (GitHub exposes a *separate* `rust-toolchain` ecosystem for it, deliberately **not** enabled by this order), the external `cargo-deny` release archive and digest, the hosted runner image labels, and any future npm ecosystem, which cannot be monitored before a browser manifest and lockfile actually exist.

## Warning and risk inventory

- **One genuine platform warning, recorded as required.** Both Dependabot jobs carried the notice `The ubuntu-latest label will migrate to Ubuntu 26 beginning October 19, 2026` (source `actions/runner-images` issue #14748; rollout 2026-10-19 → 2026-11-19). It concerns GitHub's own `ubuntu-latest` runner used by the Dependabot job. **Impact on Solo: none of our jobs use `ubuntu-latest`** — the matrix pins `ubuntu-24.04` and `windows-2025` explicitly, which is why neither `verify` job emitted it. Bounded remediation: none required now; it becomes part of the open `ubuntu-26.04` proposal above.
- **No version, deprecation, unsupported-runtime or EOL warning in our own jobs.** The `verify` annotations on the source revision are exactly the four expected notices — the two gate notices, the architecture refusal and the advisory refusal — and nothing else.
- **Two pre-existing parser notices** in the Windows job-summary script (``Use `{` instead of { in variable names``) remain, unchanged by this order, byte-identical in count to the baseline; they are documentation-string diagnostics, not runtime warnings, and the step succeeds.
- **Residual risk, stated plainly.** Dependabot may in future open proposals this order forbids anyone from auto-merging, and an open proposal is not evidence that an upgrade passed. The manual review surfaces (toolchain channel, external scanner binary, runner images, licences) are human processes that CI cannot enforce; only the SOP makes them mandatory. `ubuntu-24.04` will eventually be retired and today's currency is a point-in-time fact that decays.
- **`NOT RUN` / `UNVERIFIED`.** Dependabot's *scheduled* weekly run (only the config-triggered update jobs were observed, not the Monday 04:30/04:45 schedule); Dependabot security alerts (401); TypeScript/Vite/Playwright currency (not installed); browser/HTTP smoke (**NOT RUN** — no authorized browser surface). No local Rust toolchain exists on this machine, so no local build, lint or test was run.
- **Not claimed anywhere:** that this task upgraded a package, that the product is current in a product sense, or that a green scan proves freshness.

## CI evidence at the source revision — both hosts green

Run [`42479361260`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42479361260) at `5e7c513f…`, inspected by step and annotation rather than by badge:

- **`ubuntu-24.04` — [job `128727718624`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42479361260/job/128727718624) — success, 29 s.** Gate notice: archive `sha256 9f12ed4c…`, RustSec `309392b1…` fetched `2026-11-08T12:15:32Z`, advisories/bans/licenses/sources all ok on the unchanged locked graph; plus the architecture refusal (`undeclared-crate`, exit 1) and the real `RUSTSEC-2026-0009` refusal of `time 0.3.46`.
- **`windows-2025` — [job `128727718908`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42479361260/job/128727718908) — success, 62 s.** Gate notice: archive `sha256 975a2214…`, RustSec `309392b1…` fetched `2026-11-08T12:15:52Z`, all four classes ok.

Both hosts therefore re-ran the complete accepted gate — including the fresh advisory refresh and the Ubuntu negative probe — on a revision that changes only documentation and monitoring configuration, so no regression was introduced.


## Verification requirements and acceptance

| # | Requirement | Result | Evidence |
| --- | --- | --- | --- |
| 1 | Policy parity: one authoritative A1 standard in Architect §7B, operational guidance in Implementation SOP, matching `AGENTS.md` pointer, one narrow §6A.2 clarification; no duplicated policy, false version claims or unapproved pin changes | **PASS** | 6/6 parity groups, 0 failures, including "§7B(e2) is the single authority", "no parallel policy", and all fifteen `E01` rows unchanged |
| 2 | Concrete version assessment with sources, dates, major versions and blockers, distinguishing selected-but-not-installed | **PASS** | 23-row ledger; 11/11 crates current; Rust, scanner and action current; Node toolchain explicitly selected-but-not-installed; one evidenced runner proposal; one `UNVERIFIED` (alert API) |
| 3 | Config test: full YAML/schema validation against authoritative docs; only Cargo and GitHub Actions; cadence, branch, PR limit and non-auto-merge shown; settings access recorded without changing settings; first scan behaviour honest | **PASS** | 44/44 assertions; live Dependabot update jobs succeeded; 0 pull requests; `rust-toolchain` ecosystem deliberately not enabled; settings untouched |
| 4 | Regression: read-only blobs unchanged, source **and** final-HEAD CI green on both hosts including fresh `cargo-deny` and the Ubuntu advisory probe; warnings reviewed | **PASS at the source revision** | 15/15 read-only paths byte-identical; both `verify` jobs success with fresh advisory data; final HEAD in the handback |
| 5 | Repository/WIP: only listed paths, ACTIVE exactly Owner-installed, NEXT/ROADMAP/OUTLOOK unchanged, R01.C and Stage 01 open, four WIP files, clean status, scratch removed | **PASS** | ACTIVE `dbbb2b4d…`; ROADMAP `e6ea66cf…`; NEXT `92fa562b…`; OUTLOOK `cb9c7a61…`; six changed paths; clean tree |
| — | Canonical report accurate and complete | **PASS** | this document |

`R01.C` and Stage 01 remain **open**; this order did not consume them. `SOLO-WO-039 R01`'s verified scanner fallback is unchanged and passed again on both hosts, so it did not regress.

## Repository state, cleanup and handback

One checkout, one worktree, sole `origin`, no stash, clean working tree after publication, exactly four WIP files with no subdirectory, no second report or work-order file. Task scratch stayed in `.temp/w040` (inventory, research, schema validator, parity verifier, downloaded reference documents, commit messages) and was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was preserved untouched. No package, toolchain, manifest, lock, action or pin was installed or changed; no setting was altered; no branch, PR, merge, release or deployment was created by this agent; no credential, token or paid service was used; no destructive Git operation was performed.

**A separate Owner-authorized update order is the only route** for the `ubuntu-26.04` runner proposal, for enabling the `rust-toolchain` ecosystem, and for any future Dependabot proposal. This run does not approve any of them, does not declare the implementation Agent's work accepted, and does not close any roadmap obligation. **"agent report ready"** — the final commit, tree and both-host final-HEAD verification are in the chat handback.

- Both SOP editions incremented from revision 6 to revision 7.
