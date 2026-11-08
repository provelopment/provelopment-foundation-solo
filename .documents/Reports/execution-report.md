# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 10:42–11:05 |
| Work order | **`SOLO-WO-038 R01`** — Fail-Closed Scanner Download Resilience |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-038 R01` |
| Installed ACTIVE blob | `727bcd1c59d6f7734170e2aa1049076a320e2e8d` (10,461 bytes, 66 lines, LF, no BOM), retained **unchanged** |
| Starting HEAD / tree | `eee96096307d1fae02cda42bd41a01076bfbe68f` / `641ea88d814a935d60e39843ce142bb896aa2866` |
| Issuance record commit | `ca952b255b2b9a76d1d119620d2b7a601c252474` |
| **Source revision (workflow change)** | `451fdcc8611a8ece10799cf0e077faccae7a9d2b`, tree `6f6f7a8718aa40abd5b17c1abf414c02d011614c` |
| Workflow blob | `b16a25922a06d63b41ce7f5023df5907bdc014b6` → `119cd6f98ebb1545a673cafebd7ce2da320c9c5a` |
| **Status** | **`FAILED`** against verification requirement 3: Ubuntu passed every applicable step, the Windows gate failed once at the retry-bounded download during a GitHub degradation window, and the cause could not be log-verified (log access denied). Not an Architect `ACCEPT`, not R01.C or Stage 01 closure, not a release. |

## Preflight

Solo root verified as the intended Owner-local checkout, branch `main`, one worktree, no stash, sole `origin` = the intended public repository, remote `main` equal to local HEAD, index clean apart from the single Owner installation of ACTIVE. All **nine** stated baseline blobs matched with **zero drift**: workflow `b16a2592…`, pre-install ACTIVE `7e3744d3…`, NEXT `92fa562b…`, ROADMAP `e6ea66cf…`, OUTLOOK `cb9c7a61…`, previous report `13179618…`, `deny.toml` `3063e864…`, `Cargo.lock` `05a02666…`, `rust-toolchain.toml` `9ec55b3b…`. Exactly four WIP files, no WIP subdirectory. `README.md`, `AGENTS.md`, Architect SOP §6B and §7B (including (e1)), Implementation SOP §5C, §7 and §9, the four WIP files, the previous report and the current workflow were read before editing.

## The authorized change, exactly

The R02 `curl` argument lines gained **one continuation line per host** carrying the identical Owner-specified finite retry policy, plus a narrow explanatory comment per host:

```
--connect-timeout 10 --max-time 30 --retry 4 --retry-max-time 120 --retry-connrefused
```

Preserved byte-for-byte on both hosts: the HTTPS URL, the asset name, `curl`/`curl.exe`, `--fail --silent --show-error --location --proto '=https' --tlsv1.2`, the `--output` archive target, and the existing failure path (`|| fail "cargo-deny archive download failed"` on Linux; the `$LASTEXITCODE` throw on Windows). `--retry 4` is at most five attempts; `--max-time 30` bounds each attempt; `--retry-max-time 120` bounds the whole retry window; `--retry-all-errors` is deliberately **absent** and no loop, alternate URL, host, mirror, action, dependency or workflow was added.

**Diff audit:** 11 changed lines — 9 comment lines and the 2 flag continuation lines, **0 other**. **Parsed-workflow audit:** identical triggers (including the daily `17 4 * * *` schedule), `permissions`, runner matrix, checkout pin `3d3c42e5…`, `env`, all 28 step names, and every step body except the two gate steps; within those two the only changed lines are the comment and curl/argument lines. Verified flags present in order on both hosts, `--retry-all-errors` absent, exactly two `curl … --fail` commands, pinned digests, the `RUSTSEC-2026-0009` probe, the checkout pin, the schedule and both failure paths intact. `bash -n` on the Linux gate script exited 0 and the PowerShell parse of the Windows gate script reported **0** errors. Comments state that the retry covers transient acquisition availability only, bypasses no integrity or advisory gate, does not make the network offline, cannot guarantee the external CDN, and that an exhausted budget still fails the job; no exhaustive network-count wording was reintroduced.

## Local execution evidence — disposable loopback payload

No scanner, crate or fixture was executed; the payload was a 45-byte non-executable text file served from a task-owned loopback server under `.temp/w038` (removed afterwards). Local `curl` is 8.22.0 and reports support for all five flags.

| # | Case | Flags used | Result |
| --- | --- | --- | --- |
| A | transient **504, 504, then 200** | the five retry flags | **exit 0**, 3.1 s, **3 attempts**, 45 bytes — succeeded inside the budget |
| B | **persistent 504** | the five retry flags | **exit 22**, 15.1 s, **5 attempts** (1 + 4 retries, delays 1/2/4/8 s) — finite budget exhausted, nonzero |
| C | preserved `--proto '=https'` against a plain-HTTP URL | full workflow flag set | **exit 1** — the transport restriction still rejects non-HTTPS |
| D | good transfer, then wrong expected digest | five retry flags, then the workflow's own bash check | transfer exit 0; digest check **exit 1** with `cargo-deny archive digest mismatch: …` |
| E | control: same check with the correct digest | workflow bash check | **exit 0** (proves D failed on the digest, not on syntax) |
| F | same wrong-digest case through the Windows check style | `Get-FileHash` + `-ne` … `throw` | **threw** `cargo-deny archive digest mismatch: …` |


## CI evidence at the source revision

**Run [`42471790660`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42471790660) at `451fdcc8…`.**

**`ubuntu-24.04` — [job `128704028629`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42471790660/job/128704028629) — success (29 s), every applicable step green.** Inspected step-by-step rather than by badge: checkout, identity, toolchain install, exact version assertion, locked fetch, Linux security gate, advisory negative probe, package-set gate, format, strict lint, workspace tests, architecture checker, architecture refusal self-test, test-failure tripwire, release build, CLI smoke, forbidden-import refusal and the byte-identical double release build all `success`; Windows-only steps `skipped` as designed. Three notices, quoted exactly:

- `injected forbidden import refused with category 'undeclared-crate' and exit 1`
- `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks`
- `component security gate passed: cargo-deny 0.20.2 archive sha256 9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T10:31:50Z; advisories, bans, licenses and sources all ok on the unchanged locked graph`

This is the first run in which the Ubuntu host completed the gate at all: the bounded retry **survived real 504 responses** and reached a live digest-verified scanner acquisition, a fresh advisory database and a refused `time@0.3.46`. The `RUSTSEC-2026-0009` probe, `NOT RUN` in every R02 run, executed and refused here.

**`windows-2025` — [job `128704028414`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42471790660/job/128704028414) — failure (57 s).** The failing step is number 10, `Component security gate — cargo-deny 0.20.2 (Windows, verified archive)`; steps 1–8 succeeded, including the locked Cargo fetch, and steps 11–29 were consequently `skipped`. Its single annotation is `Process completed with exit code 1.`

The precise failing line is **`UNVERIFIED`**: `GET /actions/jobs/128704028414/logs` returned **403 “Must have admin rights to Repository.”**, so raw logs and the retry-by-retry curl diagnostics were not retrievable with the access this agent holds. What can be stated from direct evidence is bounded: the step failed inside the gate; the `throw` paths there produce only the generic exit-code annotation, so the message itself is not observable; the step consumed roughly 20–30 s before failing, which is inconsistent with an unsupported-flag failure (that aborts in about a second) and consistent with transfer attempts that waited or retried; and the same flags in the same order succeeded on the sibling Ubuntu runner in the same run. **Whether the Windows transfer exhausted its five attempts on persistent 504, or individually exceeded `--max-time 30` on a degraded link, is `UNVERIFIED`.**

**Network context, kept separate from the workflow verdict.** From this agent's environment both pinned release URLs returned **HTTP 504** on every attempt for roughly 40 minutes straddling the run, then both returned **200** at approximately 10:52Z; the release API throughout reported the unchanged asset digests and sizes, so provenance was never in doubt. GitHub's public status feed at 10:36Z listed an active `investigating` incident, *“Increase in timeouts for some unauthenticated users”*, which accounts for this agent's own 504/500 responses from `api.github.com`; it is **not** claimed to explain the runner-side 504s, which are a distinct observed phenomenon. No Solo policy, pin, digest or gate was altered to work around any of this.

## Final report-only HEAD

This revision changes only this report and therefore re-executes the identical workflow blob `119cd6f9…`. Its result is stated in the chat handback, because a report cannot contain evidence about its own commit. It was published immediately after both asset URLs had returned 200 again, to give the re-run the best available chance of an unaffected verdict; had the 504 persisted beyond the bounded retries it would be reported as such rather than retried again.


## Warnings, skips and limits

- **Warnings.** No version, deprecation or platform warning appeared as an annotation on either host. The Node-20 notice stays retired by the `actions/checkout` v7.0.1 pin (`runs.using: node24`). Nothing was suppressed and no warning was converted into a success.
- **Skips.** Every `skipped` step is the host-inapplicable half of a Linux/Windows pair, exactly as designed; no mandatory step was skipped on a host where it applies, and the host where the gate applies and failed is reported as a failure, not as a skip.
- **Versions and provenance.** Rust 1.99.1 per `rust-toolchain.toml`; `cargo-deny 0.20.2` verified by archive digest `9f12ed4c…` (Ubuntu) and expected `975a2214…` (Windows); checkout `3d3c42e5…` (v7.0.1); runners `ubuntu-24.04` and `windows-2025`. Advisory database commit observed in this run: `309392b1…`, fetched 2026-11-08T10:31:50Z (Ubuntu).
- **Local `NOT RUN`.** No Rust toolchain exists on this machine, so local build/lint/test/`cargo-deny` are `NOT RUN`; local evidence is the loopback retry and digest demonstrations, the parsed-workflow comparison, `bash -n` and the PowerShell parse. Local `curl` is 8.22.0, not the runners' builds; the runners' curl versions are `UNVERIFIED` beyond the fact that the flags did not abort instantly on Ubuntu.
- **Other `UNVERIFIED`.** Raw step logs (403, above); hosted-runner image internals; direct OSV queries (POST-only API); the daily `schedule` trigger has still not been observed firing; browser/HTTP evidence remains `UNVERIFIED / NOT RUN`, and the workflow keeps that statement.
- **Outstanding Owner requirement, not claimed.** The A1 requirement for newest-stable or preferred-LTS versions, no unresolved version or deprecation warnings, and a standing safe-upgrade process is **not** implemented or satisfied by this order. It remains a separately authorized policy and maintenance obligation, and nothing here should be read as version-policy compliance.

## Verification requirements and acceptance

| # | Requirement | Result | Evidence |
| --- | --- | --- | --- |
| 1 | Parsed workflow and line diff show only the two authorized curl argument lists and narrow local comments; identical triggers, permissions, matrix, pins, env and 28 steps; same flags on both hosts; no masked failure, skip or extra network target | **PASS** | 11 changed lines = 9 comments + 2 flag lines; parsed-YAML step comparison; `--retry-all-errors` absent; exactly two curl commands; both failure paths intact; `bash -n` 0 and PowerShell parse 0 errors |
| 2 | Local demonstration: 504-then-200 succeeds, persistent 504 exhausts and returns nonzero, wrong-digest payload refused | **PASS** | cases A–F executed with real exit codes, attempt counts and timings |
| 3 | Both hosts green at the source revision and the final report-only HEAD, with live scanner acquisition, current advisory data, all four policy classes, regression gates, the Ubuntu advisory refusal and byte-identical builds; no untriaged warning | **FAIL** | Ubuntu fully green including the `RUSTSEC-2026-0009` refusal; the Windows gate failed once at the retry-bounded download with an unverifiable cause, so that host's policy classes and regression gates did not execute. The final report-only HEAD is reported in the handback |
| 4 | `Cargo.lock` and `deny.toml` byte-identical; four WIP files; neutral NEXT; unchanged ROADMAP/OUTLOOK; complete installed ACTIVE; clean tree and scratch cleanup | **PASS** | `deny.toml` `3063e864…`; `Cargo.lock` `05a02666…` (file digest `0f78cd06…`); ACTIVE `727bcd1c…`; NEXT `92fa562b…`; ROADMAP `e6ea66cf…`; OUTLOOK `cb9c7a61…` |
| — | Canonical report accurate and complete | **PASS** | this document |

The one unmet requirement is 3, on one host for one step. It is reported as `FAIL` rather than excused: the Windows gate did not reach a verified scanner, so that host's advisories/bans/licenses/sources results, lock and tree assertions and regression gates did not run at this revision.

## Repository state, cleanup and handback

One checkout, one worktree, sole `origin`, no stash, clean working tree after publication, exactly four WIP files with no WIP subdirectory. Task-owned scratch was confined to `.temp/w038` (loopback server, its log, verification scripts, extracted gate scripts, message files) and was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was preserved untouched. No branch, PR, clone, worktree, dependency install, mirror, alternate host, secret, provider or settings action, release, merge or deployment occurred, and no destructive Git operation was used.

`R01.C` and Stage 01 remain open; ACTIVE remains `SOLO-WO-038 R01`; the Implementation Agent does not declare Architect `ACCEPT`. **“agent report ready”** — see the chat handback for the final commit/tree, both run and job link sets, and the final report-only HEAD's disposition.

Case C is why A/B/D/F omit `--proto '=https'`: a loopback plain-HTTP server cannot satisfy it. C demonstrates the flag is intact rather than weakened. **`NOT RUN`:** no local `cargo`/`rustc`/`rustup`, so no local build, Clippy, test or real `cargo-deny` run; no local TLS or proxy fault injection; the retry of a *stalled* (rather than failing) transfer was not separately exercised.
