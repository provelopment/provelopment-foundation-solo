# Foundation Solo — Active Work Order

## SOLO-WO-038 R01 — Fail-Closed Scanner Download Resilience

**Status:** OWNER-ISSUED  
**Matching Owner handoff:** `SOLO-WO-038 R01`  
**Repository / branch / remote:** `provelopment/provelopment-foundation-solo` / `main` / `origin`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Starting GitHub HEAD:** `eee96096307d1fae02cda42bd41a01076bfbe68f`  
**Starting tree:** `641ea88d814a935d60e39843ce142bb896aa2866`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Resolve the independently reviewed `SOLO-WO-037 R02` STOP caused by repeated Ubuntu HTTP **504** responses when fetching the pinned `cargo-deny` archive. Add strictly finite, transient-error retries to the two existing scanner-archive downloads; preserve checksum verification, failure propagation, all security/architecture/test gates and the already-corrected network-provenance text. This is a **new bounded executable CI scope**, not a retroactive expansion of R02's text-only approval.

**Permanent path ledger — only these three paths:**

| Path | Authorized operation |
| --- | --- |
| `.github/workflows/rust-verify.yml` | **EDIT** only the existing Linux and Windows `curl`/`curl.exe` commands that download the exact pinned scanner archives, plus narrowly adjacent explanatory comments if needed. |
| `.documents/WIP/workorder-active.md` | **OWNER-INSTALLED:** stage and commit this entire installed order **byte-identically** as issuance history. Do not edit its approved scope, clear it, or mark acceptance. |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the one canonical report with this run's actual results, including failures and final HEAD evidence. |

**Everything else is read-only:** `deny.toml`, `Cargo.toml`, `Cargo.lock`, all crate manifests/source/tests, `rust-toolchain.toml`, `.cargo/config.toml`, all other workflows and `.github/` files, README/AGENTS, About/Architecture, both SOPs, ROADMAP, NEXT, OUTLOOK, license/notice material and repository settings. Do not introduce a new download host, mirror, artifact cache, action, dependency, scanner, workflow, script or test fixture committed to Git. Do not upgrade or downgrade packages, tools, action pins, runner labels or Rust; any future stable/LTS and upgrade-policy work is a **separately authorized** Owner requirement, not a claim of this order. R01.C and Stage 01 remain open. No browser/HTTP/Page surface is authorized.

### 2. Exact Download Correction

The final R02 workflow blob `b16a25922a06d63b41ce7f5023df5907bdc014b6` has two scanner-archive download calls, one in `Component security gate — cargo-deny 0.20.2 (Linux, verified archive)` and one in the equivalent Windows step. **Only** these two existing transfer commands may acquire a bounded retry policy:

- Preserve each existing HTTPS URL and asset name, `curl`/`curl.exe`, `--fail --silent --show-error --location --proto '=https' --tlsv1.2`, the output-to-archive file and subsequent exit-status handling. Add the **same** supported flags on both hosts: `--connect-timeout 10 --max-time 30 --retry 4 --retry-max-time 120 --retry-connrefused`. `--retry 4` means **at most five transfer attempts**, using curl's built-in exponential retry delay for transient HTTP statuses (including **504**); the per-attempt and total retry budgets must remain finite. Do **not** add `--retry-all-errors`, infinite loops, a different URL or an alternate release.
- Keep raw-archive SHA-256 verification **before extraction**, archive-member safety checks, the exact `cargo-deny 0.20.2` version assertion, current RustSec fetch, all four `advisories/bans/licenses/sources` checks, real lockfile and clean-tree assertions, and the Ubuntu `RUSTSEC-2026-0009` negative probe untouched. A successful HTTP transfer with an incorrect digest must still **fail immediately**; never retry or ignore a digest mismatch, archive violation, scanner failure or advisory finding.
- On exhausted attempts, timeout, permanent HTTP error, or curl failure, the same existing shell/PowerShell failure path must terminate the mandatory job **nonzero**. Preserve visible diagnostics; never turn failure into a warning or green result. Retries apply **only** to the pinned scanner archive download, not to application Cargo, source checkout, RustSec fetch, product tests or the probe.
- Comments, if changed, must state that the retry handles *transient acquisition availability*, does not bypass integrity/advisory gates, does not make the network offline, and does not guarantee an external CDN will always be available. Do not reintroduce R02's inaccurate exhaustive network-count wording.

**Pinned identities stay exactly:** Linux `cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz`, SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; Windows `cargo-deny-0.20.2-x86_64-pc-windows-msvc.tar.gz`, SHA-256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`. Checkout stays at `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (v7.0.1); Rust stays 1.99.1. No version-warning suppression.

### 3. Preflight, Verification and Acceptance

**Before writes:** read `README.md`, `AGENTS.md`, Architect SOP §§6–8 (including §7B(e1)), Implementation SOP §§4–5C, 7–9, the current canonical report, existing workflow and all four WIP files. Verify one Solo checkout/worktree, `main`, sole correct `origin`, the stated starting HEAD/tree, safe Git status/index, and no unexpected changes, stash or untracked Owner material. The **only** anticipated local tracked change is the Owner's replacement of R02 ACTIVE with this complete R01 order. STOP on any drift or mismatch; never discard Owner data or change another repository.

**Baseline GitHub blobs:**

| Path | Expected blob SHA |
| --- | --- |
| `.github/workflows/rust-verify.yml` | `b16a25922a06d63b41ce7f5023df5907bdc014b6` |
| `.documents/WIP/workorder-active.md` (pre-install R02) | `7e3744d31cc1788d2303f97bb5f0e512bdb5730c` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` |
| `.documents/WIP/plan-roadmap.md` | `e6ea66cffa14d75598ce051b284f4ce12a16f67b` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `.documents/Reports/execution-report.md` | `131796182dfbf514c6b34c23250f87441624b5bb` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` |
| `Cargo.lock` | `05a02666894f4f0bf5528acf118a049e5edb853e` |
| `rust-toolchain.toml` | `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |

**Verification requirements (all must be observed before reporting `COMPLETE`):**

1. Compare the original/new parsed workflow and line diff: unchanged triggers (including daily schedule), permissions, runner matrix, checkout SHA, env, 28 existing steps, all commands/assertions **except the two authorized curl argument lists**, and only narrowly local comments. Check the exact flags on **both** hosts; no masked failure, skip, `continue-on-error`, extra network target or policy exception.
2. Demonstrate the actual curl retry behavior using a **disposable local-only, non-executable test payload** where host tooling permits: transient **504 then 200** must finish successfully within the attempt budget, persistent 504 must exhaust and return nonzero, and a downloaded wrong-digest payload must be rejected by the existing hash verification. Use only task-owned Solo `.temp/` or the current agent-provided temporary area (or CI runner temporary space), never a second checkout, tracked fixture, extra dependency or real vulnerable executable. If a particular local test is technically unavailable, say **`NOT RUN`** and rely on direct CI evidence; never claim a test passed without execution.
3. Commit/push only authorized paths to `origin/main`. Verify both hosts at the **new source revision** and **final report-only HEAD**. On each host require a real successful verified scanner acquisition, current RustSec database retrieval, `cargo-deny` advisories/bans/licenses/sources, locked workspace regression gates and no untriaged version/deprecation warning. On Ubuntu additionally require the real `time@0.3.46` / `RUSTSEC-2026-0009` advisory refusal, architecture import refusal and byte-identical clean release builds. Inspect **actual** GitHub check-run steps and raw logs where accessible, not merely a top-level green badge. If an upstream HTTP 504 persists beyond the bounded retries, **FAIL/STOP** honestly; do not bypass scanner, install a fallback, declare a missing test passed, or repeatedly publish reports to manufacture a green result.
4. Keep `Cargo.lock` and `deny.toml` byte-identical; retain four WIP files, neutral NEXT, unchanged ROADMAP R01.C/Stage 01 and OUTLOOK, and the complete installed ACTIVE order. Confirm exact final remote HEAD/tree, clean local status and task-owned scratch cleanup. No release, merge, deployment or Stage 01 closure.

**Canonical report:** overwrite only `.documents/Reports/execution-report.md` with `COMPLETE`, `FAILED` or `STOPPED` as actually supported; name exact source/final revisions, permitted-path audit, original HTTP 504 evidence, chosen finite curl flags, local retry/refusal tests run or `NOT RUN`, both-host CI job IDs/step outcomes, scanner asset digests, current advisory DB revision/time, negative probe and regression outputs, warnings and skipped steps, clean-tree/scratch evidence and outstanding A1 latest-stable/LTS/upgrade-process policy. The report cannot contain its own final commit SHA; provide that SHA/tree and final-host CI links in the Agent's chat handback. Report actual unavailable evidence as `UNVERIFIED`, never PASS.

**Permissions:** Owner authorizes the exact workflow edit, ordinary temporary testing, ordinary download/network checks already allowed by the workflow, and authorized stage/commit/push mechanics under Architect SOP §6B. No new external service, mirror, registry, installation, toolchain/dependency change, GitHub setting, token/secret, branch, PR, merge, release, provider call, deployment, force update or destructive Git operation. The Implementation Agent does **not** award Architect `ACCEPT`, clear ACTIVE, issue the next order or consume ROADMAP.

**Handback:** `SOLO-WO-038 R01`, final GitHub HEAD/tree, the canonical report link, exact changed paths, both-host source/final runs, network/retry and security results, and **“agent report ready”** for independent Chief Architect review.
