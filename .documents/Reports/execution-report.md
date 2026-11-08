# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 09:35–10:10 |
| Work order | **`SOLO-WO-037 R02`** — R01.C Security Gate Evidence Correction |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-037 R02` |
| Installed ACTIVE blob | `7e3744d31cc1788d2303f97bb5f0e512bdb5730c` (10,685 bytes, 56 lines, LF, no BOM), retained **unchanged** |
| Starting HEAD / tree | `d41ac2067a0dab6c2f7add0890cac14320b3a7f4` / `0f9538666035e1d4dd6241e1e11662f13b313455` |
| Starting workflow blob | `270891755eb1979c1db48db560b0126d1dbbd023`, corrected to `b16a25922a06d63b41ce7f5023df5907bdc014b6` |
| Starting report blob | `b19dc0d1518847d1f7d62c4a25772e1adee88000` |
| Correction commit (source revision) | `6591945307011f139ac30344f9b182e295931490`, tree `a15c8cbdc79a84fabd2343450af3a2b8689659f4` |
| **Status** | **`FAILED` against acceptance condition (e).** The authorized text correction is complete and verified (§ audit below), but CI is blocked by a recurrent failure of the security gate's scanner-archive download on the `ubuntu-24.04` host, twice consecutively, during an independently observed upstream HTTP 504 incident. Not an Architect `ACCEPT`, not R01.C or Stage 01 closure, not a release. |

## Preflight

Solo root verified as the intended Owner-local checkout, branch `main`, one worktree, no stash, sole `origin` = the intended public repository, remote `main` equal to local HEAD, index clean apart from the single Owner installation of ACTIVE. All eight stated baseline blobs matched with **zero drift**: the workflow `27089175…`, the previous report `b19dc0d1…`, `deny.toml` `3063e864…`, `Cargo.lock` `05a02666…`, NEXT `92fa562b…`, ROADMAP `e6ea66cf…`, OUTLOOK `cb9c7a61…` and the pre-install ACTIVE `8b0e81db…`; `Cargo.lock` file SHA-256 `0f78cd06…` unchanged. Exactly four WIP files, no WIP subdirectory. `README.md`, `AGENTS.md`, Architecture §6A.2 (which fixes `cargo-deny =0.20.2` as the pinned licensing/audit tool), both SOPs, the four WIP files, the previous report and the current workflow were read before editing.

## The discrepancy that blocked acceptance, and its correction

**Before (workflow blob `27089175…`).** The job summaries opened with “only three steps use the network” while the same paragraph separately named the network-enabled `cargo fetch --locked` as a fourth; the header called the security gate “the only other networked step”; the comment above the locked fetch said “everything after this step runs with the job-level `CARGO_NET_OFFLINE=true`”; and the security-gate comment said “every application Cargo command stays offline”. Two statements were exhaustive or incomplete rather than merely loose: the Ubuntu-only `RUSTSEC-2026-0009` negative probe sets `CARGO_NET_OFFLINE: "false"` in its step `env` and resolves its throwaway manifest with `cargo generate-lockfile`, which is a network-capable Cargo operation that no “only”/“every” phrasing accounted for; and a Cargo environment variable is not a system-wide network control.

**After (workflow blob `b16a2592…`).** The header, the `env` comment, the comments above the locked fetch, both security-gate comments and the negative-probe comment, and both job-summary paragraphs now name every network-capable operation — checkout, `rustup toolchain install`, the single network-enabled locked product-dependency fetch, the security gate's verified scanner-archive download and RustSec advisory-data refresh, and the Ubuntu-only disposable fixture's `cargo generate-lockfile` — and keep the three concerns separate: production dependency acquisition, the security/advisory data path, and the test-only fixture. They state that ordinary product build, lint, test and check commands after the locked fetch run with `CARGO_NET_OFFLINE=true` and `--locked` where applicable, that neither the advisory refresh nor the fixture is offline, and — explicitly — that no claim is made that a Cargo environment variable denies the network system-wide. **The corrected summaries make no exhaustive network-count claim of any kind.**

**Audit.** The raw diff against the starting revision is 68 changed lines: **64 comment lines and 4 summary-output lines** (two old, two new), with **zero** other changed lines in 8 hunks. Parsing both YAML revisions proves `on`, `permissions`, `env`, job non-step fields (matrix, `run-on`, timeout, defaults), all 28 step names, every `uses` pin and every non-summary `run` body identical; the only differing steps are `Job summary (Linux)` and `Job summary (Windows)`, and inside each the only changed line is the network-provenance paragraph. The locked-fetch step still carries `CARGO_NET_OFFLINE: "false"` and `run: cargo fetch --locked`; the probe still carries `CARGO_NET_OFFLINE: "false"` and `cargo generate-lockfile`; the job-level default is still `"true"`. `deny.toml` (`3063e864…`), `Cargo.lock` (`05a02666…`), the digest constants, the advisory-freshness assertions, the action pin and all Rust/architecture/reproducibility/smoke gates are untouched. A sweep of the entire corrected file for `only three`, `only other`, `every other Cargo`, `all other Cargo`, `stays offline`, `reaches the network`, `uses the network`, `only network step` and `Everything after` returns **zero** hits; the three remaining `system-wide` occurrences are the explicit disclaimers.

One label was deliberately **not** changed: the pre-existing step name `Fetch locked packages (only network-enabled Cargo acquisition)`. §1 authorizes comments and summary prose only and §2 forbids altering a step label; the corrected prose scopes that claim to the acquisition of this workspace's locked product dependencies, which remains true, and names the other network-capable operations explicitly.


## R01 evidence versus gates actually rerun on R02

Prior `R01` evidence — the scanner archives at `9f12ed4c…` / `975a2214…`, the RustSec database at `309392b1…`, the four passing policy classes, and the `RUSTSEC-2026-0009` refusal — was produced at revisions `ba0b87e`, `5ebdcc9` and `d41ac20`. **None of it is presented here as this run's evidence.** On `R02` only the following were actually rerun and observed.

**Run `42468379278` at the corrected source revision `65919453…`** (workflow blob `b16a2592…`):

| Host | Job | Result | Detail |
| --- | --- | --- | --- |
| `windows-2025` | [`128693105094`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42468379278/job/128693105094) | **success**, 68 s, 1 notice | `component security gate passed: cargo-deny 0.20.2 archive sha256 975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc; RustSec 309392b18f6b17022f82c701f300433126901068 fetched 2026-11-08T09:44:04Z; advisories, bans, licenses and sources all ok on the unchanged locked graph` |
| `ubuntu-24.04` | [`128693105275`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42468379278/job/128693105275) | **failure**, 14 s, 2 annotations | `cargo-deny archive download failed` → `Process completed with exit code 1` |

The Ubuntu failure is in the security gate's *download* step, which is byte-identical to the revision that had passed four consecutive times earlier the same day; the Windows host downloaded its own archive successfully in the same run minutes later. The gate therefore **failed closed exactly as designed** — no scanner, no green gate. Because the correction is comment/summary text only, no edited line can cause it.

Independent corroboration of an upstream availability incident, checked outside GitHub and recorded as observed: the release API still lists both assets with the **unchanged** digests and sizes, so provenance is intact, but from this environment the asset download URLs returned **HTTP 504 (“Gateway Time-out”)** for four consecutive attempts spanning roughly eight minutes, including both the Linux and the Windows asset. This is a third-party (GitHub release CDN) availability problem, not a Solo defect: there is **no** dependency, action, toolchain, scanner or policy change, and no checksum, digest or provenance ambiguity.

Rerunning a failed job is a GitHub-side action that requires a write-capable token, which this agent does not hold and is not authorized to use, so the source revision could not be re-executed directly. The next published revision changes **only** this report, so it re-runs the identical workflow blob `b16a2592…`; **its result at the final report HEAD is recorded in the chat handback, not in this document**, because the report cannot contain evidence about its own commit.

Because the Ubuntu host could not proceed past the download, the following were **not** re-observed at the R02 source revision on that host: the scanner version assertion, the RustSec fetch and its commit/freshness, the four policy classes, and the `RUSTSEC-2026-0009` advisory refusal. They are `NOT RUN` for Ubuntu on this run — not failed, not passed, and not inherited from `R01` as if they were this run's results.

**Repeat at the next report-only revision.** At revision `ac27e08…`, which changes only this document and therefore re-executes the identical workflow blob `b16a2592…`, the same pair repeated: `windows-2025` **passed** (job `128696064067`, 55 s, `975a2214…`, RustSec `309392b1…` fetched 2026-11-08T09:57:11Z, all four classes ok), and `ubuntu-24.04` **failed again** (job `128696064290`, 26 s) with the identical annotation `cargo-deny archive download failed`. Two consecutive Ubuntu failures at the same byte-identical download step, against two consecutive Windows successes on the same workflow blob, establish that this is a host-egress and upstream-artifact availability problem rather than a defect in the corrected workflow, and it is not a permissible skip: the mandatory step is red.

**Escalation.** Condition (e) therefore fails at both the corrected source revision and the following report-only revision. This requires an Owner decision, and no Solo policy, pin, digest or gate was altered to work around it. Two options are visible and **neither is implemented here** because each is an executable or policy change outside `R02`: tolerate and retry (bounded retries with backoff around the single download), or remove the runtime dependency on a third-party CDN for the scanner by pinning an Owner-approved, digest-verified artifact source. Until one is authorized, the Ubuntu host cannot reach a green security gate whenever that CDN path fails, which is the correct fail-closed behaviour but leaves acceptance (e) unmet.


## Warnings, dependencies and limits

- **Warnings.** No version, deprecation or platform warning was raised as an annotation on either host; the only annotations were the security gate's own notices and the Ubuntu failure. The Node-20 deprecation notice that existed before `SOLO-WO-035 R01` is retired by the `actions/checkout` v7.0.1 pin, which declares `runs.using: node24`. Raw step logs are not retrievable without an authenticated token, so any log-level warning that does not surface as an annotation is **`UNVERIFIED`** rather than asserted absent.
- **Version and provenance.** rustc/cargo `1.99.1` per `rust-toolchain.toml`, asserted by the existing version gate; `cargo-deny 0.20.2` with archive digests `9f12ed4c…` (Linux musl) and `975a2214…` (Windows msvc); `actions/checkout@3d3c42e5…` (v7.0.1); runners `ubuntu-24.04` and `windows-2025`. The advisory database commit observed in this run is `309392b1…`, fetched 2026-11-08T09:44:04Z on Windows.
- **Dependency and policy identity.** `Cargo.lock` blob and file digest unchanged; `deny.toml` unchanged; no dependency, action, toolchain or policy upgrade or downgrade occurred in this run.
- **`NOT RUN` / `UNVERIFIED`.** No local Rust execution is possible here (no `cargo`, `rustc` or `rustup` on this machine), so local build/test/Clippy/`cargo-deny` are `NOT RUN`; static work only was done — YAML parsing of both revisions, the per-step semantic comparison above, `bash -n` on the corrected Linux summary script, a PowerShell parse of the corrected Windows summary script (two identical pre-existing `${{ }}` messages before and after, resolved by GitHub expression substitution before execution), and a one-line diff between the baseline and corrected Windows summary script. Direct OSV.dev queries remain impossible (POST-only API). Hosted-runner image internals are `UNVERIFIED`. The daily `schedule` trigger has still not been observed firing. Browser/HTTP evidence remains `UNVERIFIED / NOT RUN`, and the workflow keeps that statement.
- **Not claimed.** This report does **not** claim that the Owner's separate requirement — newest stable or preferred LTS versions, no unresolved version or deprecation warnings, and safe future upgrades — has been integrated into the permanent A1 policy. That is a separately scoped policy and maintenance change, and `R02` neither implements it nor treats the current pins as satisfying it.

## Acceptance conditions

| # | Condition | Result | Evidence |
| --- | --- | --- | --- |
| a | Header, comments and both summaries name the network-capable operations and exceptions, with no false “only N steps” / “all other Cargo commands” claim | **PASS** | zero hits for every false pattern; the three `system-wide` occurrences are the explicit disclaimers; both summaries rewritten |
| b | Workflow functionality, `deny.toml`, lock and pins unchanged | **PASS** | 68 changed lines = 64 comments + 4 summary lines; parsed-YAML step-by-step identity; `deny.toml` `3063e864…` and `Cargo.lock` `05a02666…` unchanged |
| c | Canonical report accurate | **PASS** | this document, including the disclosed Ubuntu failure and the `R01`-versus-`R02` separation |
| d | Only the three authorized permanent paths changed | **PASS** | the source revision changes only `.github/workflows/rust-verify.yml`; the issuance commit stages the unchanged installed order; this revision changes only the report |
| e | Both source-revision and final-report-HEAD runs pass on both hosts, including the live advisory negative probe, with no untriaged warning or missing mandatory step | **FAIL** | Windows passed at both observed revisions with the live gate and full four-class results; Ubuntu failed at both with `cargo-deny archive download failed`, so the scanner assertion, advisory refresh, policy classes and `RUSTSEC-2026-0009` refusal did not run on that host |
| f | WIP remains four-file and unclosed | **PASS** | ACTIVE is the unchanged `R02` order (`7e3744d3…`), NEXT is the neutral `92fa562b…` heading, ROADMAP still carries R01.C and the Stage 01 gate, OUTLOOK unchanged |

Condition (e) is the one unmet item. It is unmet for an environmental reason rather than a defect in the corrected workflow — the same workflow blob downloaded, verified and passed on Windows in both runs — but it is still unmet, and it is reported as `FAIL` rather than excused. Conditions (a) to (d) and (f) pass on static and repository evidence that does not depend on the affected host. The disposition of the newest report-only revision is stated in the chat handback, since this document cannot contain evidence about its own commit.

## Repository state, cleanup and handback

One checkout, one worktree, sole `origin`, no stash, clean working tree after publication, exactly four WIP files with no WIP subdirectory. Task-owned scratch lived only in `.temp/w037r02` (verification scripts, message files, one probe download attempt) and was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was preserved untouched. No branch, PR, clone, worktree, dependency install, secret, provider or settings action, release or deployment occurred, and no destructive Git operation was used.

**A hardening observation for a separately authorized order**, not implemented here: the security gate performs a single download attempt with no retry, so an upstream CDN incident fails the job closed (correctly) but without resilience.

`R01.C` and Stage 01 remain open; ACTIVE remains `SOLO-WO-037 R02`; the Implementation Agent does not declare Architect `ACCEPT`. **“agent report ready”** — see the chat handback for the final commit/tree, both run and job link sets, and the newest report-only revision's disposition of condition (e).
