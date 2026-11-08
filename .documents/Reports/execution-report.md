# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 13:33–13:55 |
| Work order | **`SOLO-WO-041 R01`** — R01.C Accepted Security-Package Administrative Closure |
| Owner handoff / closure decision | Received — "proceed until R01.C is closed" (2026-11-08), authorizing **R01.C package-level reconciliation only** |
| Installed ACTIVE | blob `69aa517931e03fdeb0b798b958f7d5d14434e14e`, **16,044 bytes**, 85 lines, LF, no BOM, `OWNER-ISSUED`, preserved byte-identically in its issuance commit |
| Starting HEAD / tree | `a2870910c8bf7ed67d784e8a2f6251ab7486c829` / `b8ed0c8d00df54f30093b3cf3122641117f33776` |
| Issuance commit / tree | `607222d1e64061a6dab4b3c2290058e037ed2b0c` / `5fd0b40f55eb4db5c54ddf65510cc4b8174a556f` |
| **Closure source commit / tree** | `333f7ae3f6af3fac5ead2cf9d1399d590dc14ebd` / `c85dd09d86ede3cc269a5c8362b5dfd2af6473d9` |
| ACTIVE-neutralization commit | `53dd7a1a84aa679661e2399c2b1b6da1755c78d2` |
| **Status** | **`COMPLETE`** — the single remaining R01.C package bullet is closed after an independent evidence gate. **Stage 01 remains OPEN**, Stage 02 is not promoted, and this is not an Architect `ACCEPT`. |

## Preflight

Single Owner checkout, `main`, sole `origin` equal to the remote head, one worktree, no stash, and a working tree whose only tracked difference was the Owner-installed ACTIVE replacement. All **fifteen** stated baseline blobs matched with **zero drift**: ROADMAP `e6ea66cf…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…`, pre-install ACTIVE `c1654c5f…`, report `79b1eb48…`, workflow `5426c1f5…`, `dependabot.yml` `1874726f…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`, `sop-architect.md` `1483c9be…`, `sop-implementation.md` `51bb3792…`, `ARCHITECTURE.md` `4b050f63…`, `AGENTS.md` `6eae1da8…`, `README.md` `ce2d1cf9…`. Exactly four WIP files, no nested WIP directory. All documents required by §3 were read before any write.

## Execution order as performed

1. **Issuance history.** The complete installed order was staged, committed and pushed alone as `607222d…`; the published ACTIVE blob read back as `69aa5179…`, byte-identical to the installed file.
2. **Independent evidence gate** (below) — performed before any ROADMAP edit.
3. **Prune only R01.C** — pure deletion in ROADMAP Stage 01, committed and pushed alone as `333f7ae…`.
4. **Two-host closure-run verification** at `333f7ae…` — both hosts `success`, zero failed steps.
5. **Neutralize ACTIVE** to the exact heading plus one LF, committed and pushed alone as `53dd7a1…`; then this report as the final report-only HEAD.
6. **Report and handback** — this document; final commit, tree and final-HEAD CI results appear in chat only.

## Evidence gate — verdicts and reasons

| # | R01.C obligation | Verdict | Evidence and freshness |
| --- | --- | --- | --- |
| 1 | Complete locked graph | **PASS** | `Cargo.lock` `05a02666…` (lockfile v4) parsed this run: **15 packages** = 4 first-party (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`, each `0.1.0`, unpublished) + **11 registry** (`itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `zmij 1.0.23`) |
| 2 | All four policies over every graph target and feature | **PASS** | `deny.toml` `3063e864…` re-read in full: `[graph]` four targets (linux-gnu, linux-musl, windows-gnu, windows-msvc) with `all-features = true`; `[advisories]` RustSec `db-urls`, `yanked = "deny"`, `unmaintained = "all"`, `unsound = "all"`, `unused-ignored-advisory = "deny"`, **`ignore = []`**; `[licenses]` allow-list of exactly the four present licences with `exceptions = []` and `private.ignore = false`; `[bans]` `multiple-versions = "deny"`, `wildcards = "deny"`, `allow-wildcard-paths = true` (documented, confined to path/git edges of private crates), `allow = []`, `skip = []`, `skip-tree = []`, plus three 2026 supply-chain crate bans and three exact compromised releases; `[sources]` `unknown-registry`/`unknown-git` = `"deny"`, crates.io only, `allow-git = []`, empty org lists. No blanket ignore, skip, exception or reduced severity exists |
| 3 | Pinned toolchain, action, runners, scanner | **PASS** | `rust-toolchain.toml` `9ec55b3b…` = `1.99.1` with `rustfmt`/`clippy`; `rust-verify.yml` `5426c1f5…` pins `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` without persisted credentials, matrix `ubuntu-24.04`/`windows-2025`, `DENY_VERSION="0.20.2"`, Linux digest `9f12ed4c…`, Windows digest `975a2214…`, HTTPS-only bounded asset-ID fallback, no third-party mirror |
| 4 | Gate runs and fails closed | **PASS** | Step-level job inspection: the `cargo-deny` step is mandatory and failure-propagating on both hosts; the workflow contains no `continue-on-error`, no masked exit and no forced-true fallback |
| 5 | Fresh advisory data each run | **PASS** | RustSec commit `309392b18f6b17022f82c701f300433126901068` fetched `2026-11-08T13:44:54Z` (Ubuntu) and `13:45:12Z` (Windows) in the closure run, and again at `13:19:27Z`/`13:19:45Z` at the base revision |
| 6 | Four checks pass on the unchanged graph | **PASS** | Closure-run annotations, both hosts: advisories, bans, licenses and sources all ok on the unchanged locked graph |
| 7 | Negative probe proves the gate can fail | **PASS** | Ubuntu annotation: `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1` while the real graph passed |
| 8 | Independent advisory cross-check | **PASS** | Fresh **OSV** `querybatch` for the exact 11 registry crates and versions at `2026-11-08T13:40:27Z`: **no known vulnerability** for any of them — a second database, not a restatement of the RustSec result |
| 9 | Malicious/malformed component refusal | **PASS** | `[bans].deny` covers the 2026 malicious crates and three compromised releases; the architecture probe independently refused an injected forbidden import with category `undeclared-crate`, exit 1 |
| 10 | Application-port / CLI refusal evidence | **PASS** | `crates/solo-application/src/bootstrap_preflight.rs` declares inbound `BootstrapPreflightV1` with `BootstrapOutcomeV1::Accepted`/`Refused` and typed `BootstrapRefusalCode`; `crates/solo-cli/src/main.rs` composes `BootstrapPreflightService`, exits `0` accepted / `2` refused / `1` internal and asserts a byte-for-byte refusal line. Minimal engineering preflight — **not** a published EFS application |
| 11 | Routine version monitoring | **PASS** | `rust-verify` triggers on push, `cron "17 4 * * *"` and `workflow_dispatch`; `dependabot.yml` `1874726f…` is weekly Monday 04:30/04:45 UTC for `cargo` and `github-actions`, proposal-only, limit 3, no ignores |
| 12 | Reproducible build / provenance | **PASS** | The Ubuntu step comparing two clean release builds byte-for-byte succeeded in every run observed this session |
| 13 | GitHub security-alert API inventory | **UNVERIFIED** | `GET /dependabot/alerts` and `/code-scanning/alerts` both returned **401 Requires authentication**; no access changed, so no claim is made about alert counts |
| 14 | Scheduled Dependabot Monday run | **NOT OBSERVED** | only configuration-triggered update jobs exist so far; the first scheduled occurrence has not happened |
| 15 | Browser/HTTP security evidence | **NOT RUN** | no authorized website/browser/HTTP surface exists; the R01.C text itself makes browser smoke conditional on one |
| 16 | `unicode-ident` Unicode-3.0 attribution | **DEFERRED — release prerequisite** | recorded inside `deny.toml` and preserved here; a later release NOTICE obligation, not a gate failure and not a reason to suppress a check |

**Gate conclusion.** Every mandatory security, advisory, provenance and version-monitoring obligation of R01.C is satisfied at the closure baseline: no material untriaged vulnerability, no known malicious component, no failed, missing or hidden mandatory gate. A green badge was not treated as a comprehensive security guarantee — three items remain genuinely `UNVERIFIED`, `NOT OBSERVED` and `NOT RUN` above, plus one `DEFERRED` release prerequisite, and none of them is converted into `PASS`.


## The single closure — ROADMAP Stage 01

`plan-roadmap.md` went from blob `e6ea66cf…` (16,139 bytes, 157 lines) to `145f9d7a…` (15,656 bytes, 153 lines). The raw diff is a **pure deletion — 0 insertions, 4 deletions, exactly 483 bytes**:

```
@@ -6,6 +6,2 @@
 
-**Remaining packages**
-
-- **R01.C:** establish repeatable, fail-closed security/advisory and supply-chain verification …
-
 **Exit gate:** clean-room dependency installation and build, …
```

Independently verified byte for byte against the starting blob: the result is **strictly equal** to the original with those four lines removed and nothing else changed. `R01.C` no longer appears anywhere in the file; Stage 01's heading, its `**Prerequisite:**` line and its complete `**Exit gate:**` text are identical; all Stage 02–13 lines are identical; 13 stage headings and 13 exit gates remain. No completion history, progress note or substitute wording was inserted — Git history and the previous reports already carry that evidence.

**Scope discipline.** Only the **R01.C package bullet** is retired. **Stage 01 remains OPEN** pending a separately authorized exit-gate review. Stage 02 is **not** promoted, NEXT keeps its neutral heading, OUTLOOK is untouched, and no parallel WIP file or status prose was created.

## ACTIVE neutralization

The two-phase operation was performed in the authorized order: the complete installed order was first preserved in `607222d…` (blob `69aa5179…`, unmodified), and only after the evidence gate and the ROADMAP reconciliation was the working slot replaced with the exact neutral content — `# Foundation Solo — Active Work Order` followed by one real LF, **40 bytes**, tail bytes `65 72 0a`, no CR, no BOM, no literal backslash-n, blob `ea5f20b7…`, diff `0 insertions / 83 deletions`. The approved scope was never amended in between.

## Path and blob ledger

| Path | Before | After | Change |
| --- | --- | --- | --- |
| `.documents/WIP/workorder-active.md` | `c1654c5f…` (neutral heading) | `ea5f20b7…` (neutral heading) | order installed then neutralized; no net content difference |
| `.documents/WIP/plan-roadmap.md` | `e6ea66cf…` | `145f9d7a…` | R01.C bullet and orphaned label deleted only |
| `.documents/Reports/execution-report.md` | `79b1eb48…` | this document | overwritten |

Every other path is byte-identical to the starting HEAD: `README.md` `ce2d1cf9…`, `AGENTS.md` `6eae1da8…`, `ARCHITECTURE.md` `4b050f63…`, both SOPs `1483c9be…`/`51bb3792…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`, `.cargo/config.toml`, `dependabot.yml` `1874726f…`, `rust-verify.yml` `5426c1f5…`, `.github/SECURITY.md`, LICENSE and notices, every crate manifest/source/test, NEXT `92fa562b…` and OUTLOOK `cb9c7a61…`. No repository setting, action pin, runner, scanner version or digest, dependency version, toolchain, schedule, vulnerability rule or threshold was altered anywhere in this run.


## CI evidence

| Purpose | Revision | Run / jobs | Result |
| --- | --- | --- | --- |
| Baseline (accepted R02 final HEAD) | `a287091…` | [`42484385102`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42484385102) · Ubuntu `128742967887` · Windows `128742967782` | both `success` |
| Current published checkout before pruning | `607222d…` | [`42486042241`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42486042241) · Ubuntu `128748013566` (22 ok / 9 skipped / **0 failed**) · Windows `128748013296` (19 ok / 12 skipped / **0 failed**) | both `success` |
| **Closure source** | `333f7ae…` | [`42486401315`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42486401315) · [Ubuntu `128749121648`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42486401315/job/128749121648) (22 ok / 9 skipped / **0 failed**) · [Windows `128749121475`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42486401315/job/128749121475) (19 ok / 12 skipped / **0 failed**) | both `success` |
| Final report-only HEAD | this commit | verified after publication; identifiers and outcome in the chat handback | see handback |

The closure run exercised the live digest-verified `cargo-deny 0.20.2` from the verified archive on both hosts, freshly fetched RustSec, all four checks, locked dependency fetch, workspace package-set gate, format, strict lint, all-target tests, the architecture checker and its refusal self-test, the test-failure tripwire, release build, CLI smoke, the Ubuntu `RUSTSEC-2026-0009` refusal and two byte-identical clean release builds. The only skipped steps are the Linux-only or Windows-only counterparts on the other host; no mandatory check was skipped on either host and no step failed.

## Warnings, skips and limits

The only annotations on the observed `verify` jobs are the expected gate, refusal and advisory-probe notices: there is no new actionable version, deprecation, unsupported-runtime or end-of-life warning and no material untriaged warning of any kind. Raw job logs remain unreadable to this agent (HTTP 403), so log-internal text is not claimed. Dependabot's own updater diagnostics — the certificate-store `rehash` warning and the empty `GITHUB_REGISTRIES_PROXY` message — were classified in the previous report as benign updater-container messages with no effect on Solo's source, CI or toolchain, and nothing here changes that classification. The `ubuntu-latest` → Ubuntu 26 migration stays an upstream runner-image announcement and an open proposal: Solo's matrix pins `ubuntu-24.04` and `windows-2025`, and no migration happened.

## Remaining obligations and future proposals

- **`unicode-ident` Unicode-3.0 attribution/NOTICE** — an open **release** prerequisite; not a code, gate or closure defect.
- **`ubuntu-26.04` runner evaluation** — a separate future proposal; not started and not implied by this closure.
- **`rust-toolchain` Dependabot ecosystem** — deliberately not enabled; remains an Owner-visible manual weekly review item.
- **Dependabot security-alert and code-scanning inventories** — `UNVERIFIED` at the API level (401), so they are not asserted.
- **Browser/HTTP, accessibility and real-Page evidence** — `NOT RUN`; nothing in Stage 01 may claim them.

## Repository state

One checkout, one worktree, no stash, sole `origin`, clean working tree after publication, exactly the four WIP files with no nested directory, no second report or work-order file, ACTIVE holding the exact neutral heading, NEXT and OUTLOOK byte-identical, and no branch, pull request, merge, release or deployment created. Task scratch (`.temp/w041`) was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was preserved. No dependency was installed, no setting changed, no secret accessed and no destructive Git operation performed.

**`agent report ready`** — R01.C is closed at package level, **Stage 01 stays OPEN**, and the final commit, tree and final-HEAD CI results are given in chat. An Agent `COMPLETE` is not an Architect `ACCEPT`.

