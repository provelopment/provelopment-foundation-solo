# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 11:25–12:05 |
| Work order | **`SOLO-WO-039 R01`** — Verified Official Scanner-Asset Fallback |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-039 R01` |
| Installed ACTIVE blob | `fded5f6ed21d463e3dfde02d9dcfa6ad01943ce7` (11,985 bytes, 62 lines, LF, no BOM), retained **unchanged** |
| Starting HEAD / tree | `0b3a5d76ae3e0d3206ed2e30a4d536708d106331` / `cc156c3c8eebd87d6f291353df1eb6ce20e659bc` |
| Issuance record commit | `35e0424ae803e8b1b329795a7d5866e56a63e5b1` |
| **Source revision (workflow change)** | `4d998d6e03b20bde906b9af8511cd7c4a53357bc`, tree `1d6b836289229ff2979fba1ab9adc8548dcfe684` |
| Workflow blob | `119cd6f98ebb1545a673cafebd7ce2da320c9c5a` → `5426c1f5da6a084a9fa41e70b2d85d8741578d08` |
| **Status** | **`COMPLETE`** for this order's four verification requirements. Both hosts passed the unchanged security gate at the source revision. Not an Architect `ACCEPT`, not R01.C or Stage 01 closure, not a release. |

## Preflight

Solo root verified as the intended Owner-local checkout, branch `main`, one worktree, no stash, sole `origin` = the intended public repository, remote `main` equal to local HEAD, index clean apart from the single Owner installation of ACTIVE. All **nine** stated starting blobs matched with **zero drift**: workflow `119cd6f9…`, pre-install ACTIVE `727bcd1c…`, NEXT `92fa562b…`, ROADMAP `e6ea66cf…`, OUTLOOK `cb9c7a61…`, prior report `a8a61dd1…`, `deny.toml` `3063e864…`, `Cargo.lock` `05a02666…`, `rust-toolchain.toml` `9ec55b3b…`. Exactly four WIP files, no WIP subdirectory. `README.md`, `AGENTS.md`, Architect SOP §6B and §7B including (e1), Implementation SOP §5C, §7 and §9, the prior report, the four WIP files and the workflow were read before editing.


## The authorized change

Each host block keeps its **unchanged** primary transfer and gains exactly one conditional second route.

- **Primary, byte-identical flags:** `--fail --silent --show-error --location --proto '=https' --tlsv1.2 --connect-timeout 10 --max-time 30 --retry 4 --retry-max-time 120 --retry-connrefused`, same HTTPS URL, same asset, same `--output` target. The only addition is `--write-out '%{http_code}'`, captured into `PRIMARY_HTTP_CODE` / `$PrimaryHttpCode`, with the transfer status captured separately; the former `|| fail` and `$LASTEXITCODE` throw became an explicit equivalent failure path.
- **Fallback, only after a transient primary failure:** `https://api.github.com/repos/EmbarkStudios/cargo-deny/releases/assets/<pinned-id>` with `--header 'Accept: application/octet-stream'`, `--header 'X-GitHub-Api-Version: 2022-11-28'`, `--proto '=https' --proto-redir '=https'` so HTTPS is pinned for the initial request **and every redirect**, the same finite budget, a fresh output file after removing any partial one, and **no token, secret or credential of any kind**. Both hosts fail nonzero if it also fails.
- **Transient classification, identical on both hosts:** curl exit `6, 7, 18, 28, 52, 56`, or exit `22` with a recorded HTTP `429`/`5xx`. Anything else is a hard failure with **no** fallback, including TLS/certificate rejection, forbidden protocol, unsupported option, malformed reply and HTTP 401/403/404/410.
- **Integrity is unchanged and route-independent:** the existing raw SHA-256 gate, archive-member safety checks, extraction, `cargo-deny 0.20.2` version assertion, RustSec fetch, all four policy classes, lock/clean-tree assertions and the `RUSTSEC-2026-0009` probe run after whichever route completed. A completed transfer with wrong bytes fails there and is never retried through the other route.
- Both job-summary network paragraphs and the step comment now name the GitHub API fallback, and still state that neither route is guaranteed available and that no Cargo variable denies the network system-wide.

**Static scope proof.** Parsed before/after: identical triggers including the daily `17 4 * * *` schedule, `permissions: contents: read`, job fields, runner matrix, `timeout-minutes`, all 28 step names in order, checkout pin `3d3c42e5…`, `persist-credentials: false`, Rust `1.99.1`, both digest constants and every assertion. Exactly **four** steps differ and in **`run` only**: the two security gates and the two job summaries, the latter by exactly one executable line each (the network-provenance paragraph). Modified executable lines, enumerated: per gate, the added asset ID and API URL, the primary capture (`if`/`else`/`fi` on Linux; assignment plus `$LASTEXITCODE` on Windows), the transient classification, the explicit nontransient failure, the partial-file removal, the fallback invocation with its own failure path, and one diagnostic echo; the removed lines are only the former primary invocation and its former inline failure check. Asserted: one primary and at most one API fallback per host, `releases/assets/` appearing exactly twice, no `Authorization` header, no token reference, no `continue-on-error` on any step, no `|| true` added anywhere, finite per-attempt and total budgets on both routes, no alternate version and no `http://` endpoint. `bash -n` returned 0 for the Linux gate and the Linux summary; the PowerShell parser reported **0** errors for the Windows gate. The Windows summary reports the same **2** pre-existing parser notices as the unmodified baseline (identical messages, verified against `HEAD`), non-blocking and previously seen in green runs.


## Local fault-injection evidence — 18 cases, both host shells

**Simulated, and labelled as such:** the two gate slices were extracted verbatim from the workflow, their URLs and expected digest substituted for a task-owned loopback server (plain HTTP on 127.0.0.1, plus a self-signed TLS listener), and `--proto`/`--proto-redir` dropped **only** in the loopback variants so plain HTTP could be reached. Production flags were never weakened to make a fixture pass — cases L10/L11 keep them precisely to prove refusal. This proves the **logic**, not that the real GitHub endpoint responded. No scanner, real archive or executable ran; the payloads are disposable non-executable fixtures. All 18 cases matched their expectation.

| Case | primary | API | expect | rc | API hits | observed |
| --- | --- | --- | --- | --- | --- | --- |
| L1 transient then fallback | 504 | good | succeed | **0** | 1 | primary 5 attempts, fallback used |
| L2 primary healthy | good | good | succeed | **0** | 0 | no fallback |
| L3 primary wrong digest | corrupt | good | fail, no fallback | 1 | **0** | `archive digest mismatch` |
| L4 primary 404 | 404 | good | fail, no fallback | 1 | **0** | `nontransiently (curl exit 22, http 404)` |
| L5 primary 403 | 403 | good | fail, no fallback | 1 | **0** | `nontransiently (curl exit 22, http 403)` |
| L6 both routes fail | 504 | 500 | fail | 1 | 5 | `release-asset API fallback download failed (http 500)` |
| L7 API returns JSON | 504 | json | fail | 1 | 1 | `archive digest mismatch` (never deserialized) |
| L8 API returns HTML | 504 | html | fail | 1 | 1 | `archive digest mismatch` |
| L9 API archive escapes root | 504 | unsafe | fail | 1 | 1 | `member that escapes the extraction root` |
| L10 plain-HTTP transport | http URL | — | fail, no fallback | 1 | **0** | `nontransiently (curl exit 1, http 000)` |
| L11 TLS verification | self-signed TLS | — | fail, no fallback | 1 | **0** | `nontransiently (curl exit 60, http 000)` |
| W1 transient then fallback | 504 | good | succeed | **0** | 1 | primary 5 attempts, fallback used |
| W3 primary wrong digest | corrupt | good | fail, no fallback | 1 | **0** | `archive digest mismatch` |
| W4 primary 404 | 404 | good | fail, no fallback | 1 | **0** | `nontransiently (curl exit 22, http 404)` |
| W6 both routes fail | 504 | 500 | fail | 1 | 5 | `release-asset API fallback download failed` |
| W7 API returns JSON | 504 | json | fail | 1 | 1 | `archive digest mismatch` |
| W9 API archive escapes root | 504 | unsafe | fail | 1 | 1 | `member escapes the extraction root` |
| W11 TLS verification | self-signed TLS | — | fail, no fallback | 1 | **0** | `nontransiently (curl exit 60, http 000)` |

Two harness artefacts were found and fixed during setup rather than reported as defects: GNU `sha256sum` prefixes its output with `\` when the archive path contains a backslash, and GNU `tar` reads `C:/…` as a remote `host:path`. Both come from running a Windows-shaped temporary path through Git-bash; the Linux variant is given a POSIX temp path, and neither condition exists on `ubuntu-24.04`.

## CI evidence at the source revision — both hosts green

**Run [`42476801856`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42476801856) at `4d998d6e…`.** Inspected by step and annotation, not by badge.

- **`windows-2025` — [job `128719860598`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42476801856/job/128719860598) — success, 71 s.** The security gate step reports `success` in 8 s. Notice, quoted exactly: `component security gate passed: cargo-deny 0.20.2 archive sha256 975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T11:41:29Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`. **This is the first Windows pass of the security gate in this series**, after the five-504 and timeout-exhaustion failures that motivated this order.
- **`ubuntu-24.04` — [job `128719860263`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42476801856/job/128719860263) — success, 25 s.** Gate step `success` in 2 s; every applicable step green (locked fetch, gate, advisory probe, package-set, format, strict lint, tests, architecture checker, refusal self-test, tripwire, release build, CLI smoke, forbidden-import refusal, byte-identical double build). Three notices: `injected forbidden import refused with category 'undeclared-crate' and exit 1`; `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks`; and `component security gate passed: cargo-deny 0.20.2 archive sha256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T11:41:00Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`.

Both hosts therefore verified the archive at the pinned digest, asserted the exact `cargo-deny 0.20.2` version, fetched the same current advisory database commit and passed advisories, bans, licenses and sources on the unchanged locked graph.

**Which route served each archive is `UNVERIFIED`.** The gate steps completed in 2 s (Ubuntu) and 8 s (Windows), which is consistent with a first-attempt primary transfer, and the diagnostic lines that would name the route are inside the job log, which returns **403 “Must have admin rights to Repository.”** to this agent's access. The fallback is therefore proven by the 18 local cases rather than observed in CI, and artifact identity does not depend on the route: both converge on the same SHA-256 gate.


## Final report-only HEAD

This revision changes only this report and therefore re-executes the identical workflow blob `5426c1f5…`. Its result is stated in the chat handback, because a report cannot contain evidence about its own commit.

## Warnings, skips and residual risk

- **Warnings.** Neither host produced a version, deprecation or platform warning for this change: the only annotations were the three Ubuntu notices and the one Windows gate notice. The Node-20 notice stays retired by the `actions/checkout` v7.0.1 pin (`runs.using: node24`). Nothing was suppressed and no warning was converted into a success.
- **Skips.** Every `skipped` step is the host-inapplicable half of a Linux/Windows pair. No mandatory step was skipped on a host where it applies.
- **Residual risk, stated plainly.** Both routes are GitHub-hosted delivery of the same artifact and may share a CDN, so a GitHub-wide degradation can still fail both; the fallback is not a mirror and no availability is promised. The unauthenticated release-asset API is rate limited per IP, and runner egress addresses are shared, so under load the fallback can legitimately answer `403`, which this design classifies as nontransient and fails the job — deliberately, rather than adding a token, an alternate host or a weaker check. The Owner-specified `--max-time 30` per attempt is unchanged and now also bounds the fallback, so a sufficiently slow GitHub path can exhaust both routes and fail closed.
- **`UNVERIFIED`.** Route attribution per host (job logs are 403 to this agent's access); runner curl builds beyond the fact that no flag aborted; hosted-runner image internals; direct OSV queries (POST-only API); the daily `schedule` trigger, still never observed firing; browser/HTTP evidence, still `UNVERIFIED / NOT RUN` and still stated as such by the workflow.
- **`NOT RUN` locally.** No Rust toolchain exists on this machine, so no local build, Clippy, test or real `cargo-deny`. Local `curl` is 8.22.0, not the runners' builds.
- **Outstanding Owner requirement, not claimed.** The A1 requirement for newest-stable or preferred-LTS versions, zero unresolved version or deprecation warnings, and a standing safe-upgrade process is **not** implemented or satisfied here. It remains separately authorized work.

## Verification requirements and acceptance

| # | Requirement | Result | Evidence |
| --- | --- | --- | --- |
| 1 | Parsed before/after scope proof; identical triggers, permissions, matrix, step names, pins and non-download bodies; every modified executable line enumerated; one primary plus at most one API fallback per host; finite failure propagation; HTTPS-only redirects | **PASS** | four differing steps, `run`-only; enumerated lines; route assertions; `bash -n` 0 and PowerShell 0 for the gates |
| 2 | Failure/negative proof in disposable scratch on both host shells, without executing malicious code, with all actual exit codes | **PASS** | 18/18 cases; exit codes 0, 1; curl 22/1/60 paths; API-hit counts prove the absence of fallback where it is forbidden |
| 3 | Both hosts green at the source revision and the final report-only HEAD, with either official route, pinned digest, exact version, current advisory data, four policy classes, Rust regression gates, Ubuntu advisory refusal and byte-identical build | **PASS at the source revision** | Windows job `128719860598` success; Ubuntu job `128719860263` success; final HEAD in the handback |
| 4 | `Cargo.lock`, `deny.toml` and all unapproved paths byte-identical; four WIP files; NEXT neutral; ROADMAP retains R01.C/Stage 01; OUTLOOK unchanged; installed ACTIVE unchanged; final remote HEAD/tree; clean status; scratch cleanup | **PASS** | `deny.toml` `3063e864…`; `Cargo.lock` `05a02666…`; NEXT `92fa562b…`; ROADMAP `e6ea66cf…`; OUTLOOK `cb9c7a61…`; ACTIVE `fded5f6e…` |
| — | Canonical report accurate and complete | **PASS** | this document |

## Repository state, cleanup and handback

One checkout, one worktree, sole `origin`, no stash, clean working tree after publication, exactly four WIP files with no WIP subdirectory. Task-owned scratch was confined to `.temp/w039` (loopback servers, fixtures, slices, generated case scripts, self-signed certificate, verification scripts, request logs, message files) and was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was preserved untouched. Production ports were loopback-only; the self-signed certificate was generated locally for the TLS-refusal case and is not tracked. No branch, PR, merge, clone, worktree, dependency install, alternate host or mirror, token or secret use, provider call, settings change, release or deployment occurred, and no destructive Git operation was used.

`R01.C` and Stage 01 remain open; ACTIVE remains `SOLO-WO-039 R01`; the Implementation Agent does not declare Architect `ACCEPT` and does not issue another order. **“agent report ready”** — final commit/tree, both run and job links, and the final report-only HEAD's disposition are in the chat handback.

**Upstream identities re-read before work (2026-11-08) and matching the order exactly — no mismatch, so no STOP:** release `0.20.2`; `cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz` asset ID **471598214**, 4,936,832 bytes, API digest `sha256:9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; `cargo-deny-0.20.2-x86_64-pc-windows-msvc.tar.gz` asset ID **471599057**, 4,759,363 bytes, API digest `sha256:975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`.
