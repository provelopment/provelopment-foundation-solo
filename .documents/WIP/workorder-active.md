# Foundation Solo — Active Work Order

## SOLO-WO-039 R01 — Verified Official Scanner-Asset Fallback

**Status:** OWNER-ISSUED only upon Owner installation and matching handoff  
**Matching handoff:** `SOLO-WO-039 R01`  
**Repository:** `provelopment/provelopment-foundation-solo`, `origin/main`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Required starting HEAD:** `0b3a5d76ae3e0d3206ed2e30a4d536708d106331`  
**Required starting tree:** `cc156c3c8eebd87d6f291353df1eb6ce20e659bc`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Address the still-red scanner-acquisition gate left by `SOLO-WO-038 R01`. Its finite retries did what was designed, but the source-revision Windows job exhausted five HTTP 504s (curl 22), and the final-HEAD Windows job exhausted transfer timeouts (curl 28). Ubuntu passed the unchanged security gate. Retain the existing official `github.com` download as primary and add **one conditional, independently pinned, official GitHub release-asset API route** per host *only after the primary transfer fails*. Verify identical artifact bytes by the existing expected archive SHA-256 before any extraction or execution. This is an acquisition reliability correction, **not** approval for a security bypass, a new scanner, a dependency upgrade, or release.

**Permanent-path ledger — ONLY:**

| Path | Authorized operation |
| --- | --- |
| `.github/workflows/rust-verify.yml` | **EDIT** the two existing host-specific scanner-download blocks and their narrowly adjacent network-provenance comments / two job-summary network paragraphs only if needed for accuracy. No other executable change. |
| `.documents/WIP/workorder-active.md` | **OWNER-INSTALLED:** commit the complete installed order byte-identically as issuance history; no self-edit, neutralization or acceptance. |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the one canonical report with factual execution and two-host CI evidence. |

**Read-only:** every other permanent path, including `deny.toml`, all Cargo manifests and `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, Rust source/tests, Architecture/About, both SOPs, README, AGENTS, other workflows and GitHub configuration, root license, `.documents/WIP/plan-roadmap.md`, `plan-next.md` and `plan-outlook.md`. The four-file WIP inventory remains exact. R01.C and Stage 01 remain open.

### 2. Exact asset identities and approved network routes

Official upstream `EmbarkStudios/cargo-deny` **release `0.20.2`**, whose release-asset API identities and digests were independently read on 2026-11-08:

| Host | Exact release archive | GitHub release asset ID | Required raw SHA-256 |
| --- | --- | --- | --- |
| `ubuntu-24.04` x86_64 | `cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz` | `471598214` | `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f` |
| `windows-2025` x86_64 | `cargo-deny-0.20.2-x86_64-pc-windows-msvc.tar.gz` | `471599057` | `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc` |

1. **Primary, unchanged:** `https://github.com/EmbarkStudios/cargo-deny/releases/download/0.20.2/<exact-host-archive>`; preserve its `curl` / `curl.exe` HTTPS/TLS restrictions, `--fail`, output path and existing **bounded** `--connect-timeout 10 --max-time 30 --retry 4 --retry-max-time 120 --retry-connrefused` behavior.
2. **Only on a transient primary transfer failure** (e.g., curl exit 22 **with recorded HTTP 429 or 5xx**, connection/DNS failure, or transfer timeout; capture the HTTP status and exit code): remove its possibly partial output, then invoke **one logical fallback download through the official GitHub API asset endpoint** `https://api.github.com/repos/EmbarkStudios/cargo-deny/releases/assets/<exact-host-ID>`, requesting `Accept: application/octet-stream` as documented in GitHub's *Get a release asset* API, with appropriate GitHub API request headers. The endpoint may return the binary directly or redirect; accept only HTTPS on the initial request and every redirect; use finite per-attempt and total retry budgets, no unbounded wait, and a fresh, exact output file. The public API permits unauthenticated retrieval: **do not add or transmit a token, secret or a write credential**. This is still GitHub-hosted delivery, not an independent mirror, and may ultimately share its CDN; do not promise availability.
3. **Integrity always wins:** Do **not** fall back on a nontransient response (including HTTP 401/403/404/410), TLS/certificate verification failure, forbidden protocol/redirect, malformed response or unsupported curl option; those are STOP/FAIL. If the primary curl returns success, verify the **existing** raw SHA-256 *immediately*. A completed transfer with wrong bytes, unexpected JSON/HTML or incorrect digest must **FAIL**, not fall back to the other route. If the primary curl fails, fall back only as above, then verify the **same** raw SHA-256 and original archive-member/path safety checks before extraction. Any missing asset, invalid response, wrong checksum, unsafe archive member, wrong `cargo-deny 0.20.2` reported version or inability to verify the final binary **fails the job**. No stale cache, blind mirror, alternate version, bypass, `continue-on-error`, `|| true` that masks an ultimate failure, or skip.
4. Preserve the scanner's fresh RustSec database fetch and all four **advisories, bans, licenses, sources** checks on the unchanged locked graph; preserve all existing architecture, source-injection, version assertions, format/lint/test, byte-reproducibility, binary smoke and Ubuntu `time@0.3.46`/`RUSTSEC-2026-0009` refusal gates. Keep the checkout full-SHA pin `3d3c42e5aac5ba805825da76410c181273ba90b1`, Rust `1.99.1`, daily schedule, manual trigger, runner matrix, `contents: read`, `persist-credentials: false`, and accurate prior network-provenance caveats. Explain the newly permitted **GitHub API fallback** in comments/summaries without claiming network isolation or unconditional success.

**Published GitHub API documentation:** `https://docs.github.com/en/rest/releases/assets#get-a-release-asset`. The upstream API release entry and pinned IDs/digests must be re-read before work; a mismatch is STOP, not a silent new identity.

### 3. Preflight and implementation boundary

Before writing, read `README.md`, `AGENTS.md`, Architect SOP §§6–8 including §7B(e1), Implementation SOP §§4–5C and 7–9, the current canonical report, all four WIP files, and the workflow. Verify the correct Solo root, single checkout/worktree, clean `main`, sole `origin`, no stash or unknown untracked content, matching Owner-installed full R01 order, and exact required HEAD/tree. Only the Owner's installation of ACTIVE may be an anticipated local tracked change. STOP on mismatch, changed remote HEAD, unsafe local state, or authority conflict; do not reset, discard or alter Owner data.

**Critical starting Git blobs:** `rust-verify.yml` `119cd6f98ebb1545a673cafebd7ce2da320c9c5a`; `deny.toml` `3063e864c28be1e77787f43f1b6f561d543c8f4a`; `Cargo.lock` `05a02666894f4f0bf5528acf118a049e5edb853e`; `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`; previous ACTIVE `727bcd1c59d6f7734170e2aa1049076a320e2e8d` (Owner installation is the sole permitted replacement); NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `e6ea66cffa14d75598ce051b284f4ce12a16f67b`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; prior canonical report `a8a61dd18a6c494e21729c755676939ccacf8293`. Recheck additional source/SOP baseline from actual HEAD and disclose any drift.

Implement only the minimal acquisition-fallback logic in the existing two host steps. Do not change application Cargo network settings, introduce helper actions/scripts or new workflow files, install a new scanner, embed a binary or change accepted security policy. Do not log tokens or sensitive response headers. A GitHub API response that serves metadata rather than an archive must fail the checksum gate; do not try to deserialize it as an executable.

### 4. Verification and handback

1. **Static scope proof:** compare parsed YAML before/after: triggers, permissions, runner matrix, all existing step names, checkout pin, all non-download executable bodies, digest constants and security assertions unchanged. Enumerate every modified executable line. Assert one primary and at most one API fallback per host, with finite failure propagation and HTTPS-only redirects.
2. **Failure/negative proof** in task-owned disposable scratch, without executing malicious code: demonstrate, on both host shell implementations where feasible, a failed primary followed by a valid substitute succeeds *only after matching expected digest*; primary success with wrong digest fails **without** fallback; primary permanent 404/403 and TLS rejection fail **without** fallback; both routes failing yields nonzero; an API JSON/HTML response, corrupted archive, unsafe archive member or wrong SHA fails; no fallback to a different version or HTTP transport. A simulated/loopback test is labeled as such, **not** proof the real GitHub endpoint worked. Record all actual exit codes. Do not weaken production flags to make the fixture pass.
3. **Live CI is indispensable:** commit and push the authorized workflow revision, then inspect GitHub **source-commit** CI on **both** `ubuntu-24.04` and `windows-2025`: scanner archive verified at the pinned digest via either official route, exact version asserted, current RustSec revision/fetch time logged, all four policy classes passing, existing Rust regression gates running, Ubuntu real advisory refusal and byte-identical build. Repeat verification at the **final report-only HEAD**. A failed or missing mandatory job is `FAIL`, not a waived condition; no repeated report-only commits just to fabricate a green badge. Warn/deprecation inventory must be actual and without untriaged version or runtime support warnings.
4. **Scope and WIP:** `Cargo.lock`, `deny.toml` and all unapproved paths byte-identical. Four WIP files only; NEXT neutral; ROADMAP retains R01.C/Stage 01; OUTLOOK unchanged; full installed ACTIVE unchanged. Verify final remote HEAD/tree, clean local status and task-owned scratch cleanup.

Overwrite and publish only `.documents/Reports/execution-report.md` with the exact work-order identity and Owner handoff; baseline, issuance/source/final commits and trees (report cannot embed its own SHA); permitted-path diff, official asset IDs and SHA-256, every actual source attempted per host, HTTP/exit/digest outcomes; local fault-injection evidence; GitHub both-host step/log links; advisory DB commit/time, negative-case results; `PASS`/`FAIL`/`NOT RUN`/`UNVERIFIED` distinctions; warnings and residual risk. A green scanner at one host does not certify the other. If GitHub delivery continues to fail on both routes, report `FAILED`, retain all gates and seek a distinct Owner decision rather than adding third-party mirrors, introducing privileged credentials or weakening checks.

**Permissions:** inside the single Owner checkout, edit/commit/push **only** the three paths in §1 to existing `origin/main`; fetch official public GitHub release metadata and either of the two described archive routes; perform safe host-side disposable tests and the existing CI workflow. No PR, merge, release, deployment, new provider/account, token use, paid service, package upgrade, parent/sibling-repository write, second checkout/worktree, destructive Git operation or GitHub settings change. The latest-stable/prefer-LTS, zero-warning and safe-upgrade **permanent A1 policy request remains a separate unfinished Owner requirement** and is not authorized here.

**Handoff:** After publication and exact remote verification, return the final HEAD/tree, two-host CI run/job links and **“agent report ready”** for independent Chief Architect review. Do not self-ACCEPT, clear ACTIVE, close R01.C or Stage 01, or issue another order.
