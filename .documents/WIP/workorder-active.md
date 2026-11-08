# Foundation Solo — Active Work Order

## SOLO-WO-036 R01 — Accepted CI Reconciliation and Component-Security Gate

**Status:** OWNER-ISSUED  
**Matching handoff:** `SOLO-WO-036 R01`  
**Repository:** `provelopment/provelopment-foundation-solo` (`origin`), branch `main`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Starting HEAD:** `283066652c71c848e4d68daa4d5499aeae0075a4`  
**Starting tree:** `aa4364c264db5d423c18b598eba716179daf9dba`

### 1. Approved Scope — OWNER-ISSUED

**Objective.** Reconcile the independently accepted `SOLO-WO-035 R02` CI/tooling slice into the four-file WIP lifecycle, and establish the Owner-required **component-by-component security and vulnerability review** as a permanent engineering acceptance obligation. Select the immediately executable security-verification remainder of R01.C as NEXT. **Do not** declare the unfinished R01.C package or Stage 01 complete, and do not treat this documentation transition as a vulnerability-scan PASS.

The 2026-11-08 Chief Architect `ACCEPT` covers the R01.C Rust CI/tooling slice at final GitHub HEAD `283066652c71c848e4d68daa4d5499aeae0075a4`. The existing Rust format/lint/test, architecture refusals, adversarial checker probe, release CLI smoke and Ubuntu clean-build comparison remain accepted evidence. An executable vulnerability/advisory gate does **not** yet exist. No browser/Page/HTTP product exists; browser smoke is **NOT RUN**, never a simulated pass.

**Approved permanent operations (five paths only):**

| Path | Operation |
| --- | --- |
| `.documents/SOP/sop-architect.md` | **EDIT** §7B only, inserting the exact security requirement in §2 below |
| `.documents/WIP/plan-roadmap.md` | **EDIT** only Stage 01's R01.C remaining-package line and its exit-gate line, exactly as §3 |
| `.documents/WIP/plan-next.md` | **REPLACE** neutral heading with the single bounded R01.C security candidate in §3 |
| `.documents/WIP/workorder-active.md` | **OWNER-INSTALLED** complete R01/R02 issuance record may be staged unchanged, then **RESET** to the sole neutral heading in §3 upon successful reconciliation; never rewrite its approved scope |
| `.documents/Reports/execution-report.md` | **OVERWRITE** the sole canonical report with this reconciliation's actual evidence |

Everything else is **read-only**: `.github/workflows/rust-verify.yml`, `Cargo.toml`, `Cargo.lock`, every crate manifest/source/test, Rust toolchain and Cargo configuration, README/AGENTS, About/Architecture, Implementation SOP, OUTLOOK, security files, GitHub Settings and other repositories. No new source, test, scanner, dependency, workflow, security exception or provider is authorized here. No PR, merge, release or deployment.

### 2. Permanent security-review standard

In `.documents/SOP/sop-architect.md` §7B, insert the following paragraph **immediately after the existing `(e) Engineering hygiene` paragraph**, without changing the other requirements or renumbering them:

> **(e1) Current component-security and supply-chain assessment.** Before acceptance of every code-bearing order, independently inventory the exact reviewed revision's first-party attack surfaces and all third-party components actually used, including the complete direct/transitive **normal, build and dev/test** dependency graph, language/compiler/Cargo/rustup toolchain, CI actions and immutable pins, runner/build environments, build scripts and approved external tools or services. Research applicable exact versions against current RustSec, OSV/GitHub Security Advisories and relevant vendor/security notices, including malicious or typosquatted releases, yanked/unmaintained components, provenance, permissions and known exposed attack paths. Record the source/database freshness, affected-version range, advisory identifier/severity, actual reachability/conditions, fixed safe version where one exists, current exposure and disposition for **each** component; use appropriate repeatable, fail-closed automated advisory/license/source checks when available. Assess the first-party changed code against the applicable security boundaries and negative/refusal tests. An unrun scan, inaccessible source, untriaged material warning or unknown component is **UNVERIFIED**, never clear. Known malicious dependencies or applicable critical/high vulnerabilities prevent `ACCEPT`; other findings require risk-based mitigation or an explicit Owner-accepted documented disposition. Prefer a verified patched compatible version over a blind “latest” upgrade; any dependency, action or toolchain upgrade requires its own authorized scope, complete regression/security evidence and lock/pin review. A green build or CI run alone is not proof of security.

This requirement applies on subsequent code-bearing reviews, including packages introduced later; it does not falsely represent a complete live security audit of the present repository. Keep the rest of §7B, including the existing A1 security standard, intact.

### 3. Exact WIP reconciliation

1. **ROADMAP:** Under Stage 01, replace the sole existing `- **R01.C:**` remaining-package line with exactly:

   `- **R01.C:** establish repeatable, fail-closed security/advisory and supply-chain verification across the exact locked Cargo graph (including dev/transitive dependencies), CI actions, Rust/Cargo toolchain and relevant build inputs; triage applicable findings and make the version-aware checks part of routine CI. Browser smoke becomes applicable when a separately authorized real Page/HTTP/browser surface exists; do not install a fictitious browser target.`

   Replace only the Stage 01 exit-gate line with:

   `**Exit gate:** clean-room dependency installation and build, one genuine request-to-application-port or equivalent minimal vertical-slice proof, green static checks and deliberately failing forbidden-import architecture probe, plus a fresh, exact-version security/advisory assessment with no unresolved applicable critical/high or malicious-component finding and no hidden mandatory security-scan skip. No premature feature shell or provider dependency. Browser evidence remains NOT RUN until a real authorized browser surface exists.`

   Preserve every other ROADMAP line, package/stage identifier, later-stage security obligation and exit gate unchanged. **R01.C and Stage 01 remain open.** This removes accepted CI/tooling obligations from the *remaining* package description; it does not insert a historical completion diary.

2. **NEXT:** Replace the existing neutral file with exactly this bounded proposal (UTF-8, LF, final newline):

   ```markdown
   # Foundation Solo — plan-next (Next Candidate)

   ## R01.C — Version-Aware Component Security and CI Advisory Gate

   **Objective:** verify each exact component and supply-chain boundary in the four-crate Rust workspace, then add genuinely executable, fail-closed vulnerability/supply-chain checks to the existing CI without weakening accepted gates.

   **Readiness:** the locked four-crate Rust workspace and both-host CI are installed; browser/HTTP/Page is not yet installed and supplies no test target.

   **Proposed bounded scope:** enumerate the complete `Cargo.lock` graph (normal/build/dev/test), first-party attack surfaces, `rustc`/Cargo/rustup, the checkout action's exact commit, hosted runners and any tool to be introduced. Cross-check current RustSec, OSV/GHSA, crates.io yank/malicious notices and official advisories; assess version applicability, provenance, toolchain/action support and mitigation. Choose a minimal verified scanner with pinned provenance, a fresh identifiable advisory database, no secret or write-token exposure and failure-propagating CI on applicable findings; include periodic rechecks so a new advisory is not missed between source commits. Investigate the checkout v4.3.0 Node.js-20/deprecation warning and independently verify any proposed secure replacement pin before changing it. Upgrades or new tools require exact Owner-approved implementation scope, not automatic latest-version selection. Preserve existing CI, tests, architecture and locked reproducibility.

   **Exit:** per-component risk ledger with advisory IDs and exact affected/patched ranges; live results from a current database at the final revision; mandatory CI security checks run (not placeholders), demonstrated expected refusal on a safely controlled known-vulnerable fixture where supported, and both-host regression checks. Critical/high applicable or malicious findings are fixed and verified or remain explicit blockers; all other findings are triaged. Do not claim browser tests until an actual authorized surface exists.
   ```

   NEXT is **not** executable until a future separate Owner-issued work order. Do not create that order here.

3. **ACTIVE:** Verify and stage the Owner-installed complete `SOLO-WO-036 R01` file *unchanged* as an issuance-history commit before clearing it. After the exact §2–§3 reconciliation, permitted-path audit and successful publication checks, replace ACTIVE with exactly `# Foundation Solo — Active Work Order\n` (one LF; no BOM or extra text), then commit/push it. If reconciliation is stopped or fails, retain the installed ACTIVE text rather than silently neutralizing it.

4. **OUTLOOK:** unchanged. WIP contains exactly its four existing files and no subdirectory or extra report. Git history is the record of prior work; do not add an archive, historical summary or completed-work status entry into ROADMAP/NEXT/ACTIVE.

### 4. Read-only security evidence for this administrative run

Independently re-check the current lockfile inventory and supply-chain exposure against dated upstream sources, **without installing or altering dependencies**. The Architect's preliminary 2026-11-08 read-only observations are **research leads, not PASS certificates**:

- The tracked lockfile contains 11 registry dependencies: `itoa 1.0.18`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.9`, `unicode-ident 1.0.27`, `zmij 1.0.23`. All production crate dependency edges remain local; `syn` and `serde_json` are test-only.
- The RustSec advisory database at Git commit `309392b18f6b17022f82c701f300433126901068` (2026-11-06) had no per-package advisory directory entry for any of those 11. That **does not establish complete absence of vulnerabilities** in other sources or after that database revision.
- The August 2026 malicious crates `arrayref 0.3.10`, `internment 0.8.7`, `append-only-vec 0.1.9` and spoofed `proc-macro1` do not appear in the locked inventory; `proc-macro2` is a different legitimate package. Verify against the Rust Security Response Team notice, `https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/`.
- Current CI uses `actions/checkout@08eba0b27e820071cde6df949e0beb9ba4906955` (v4.3.0) and receives a Node.js-20 deprecation warning; the official action has newer releases with improved dependency/security handling. This is a **follow-up upgrade review**, not proof of an exploitable vulnerability in this configuration. No action pin may be changed under this order.
- Rust 1.99.1 is pinned. The published Cargo symlink advisory CVE-2026-5223 affects Cargo versions before the Rust 1.96.0 fix for third-party registries; verify the actual tool version and registry usage independently. Do not infer security of third-party tooling merely from Rust version.

Check source provenance and freshness, version matches, severity, relevance of dev-only versus runtime exposure, licenses/notices, CI token/cache/PR trust boundary and any unsupported scan. Record each component's `NO LISTED ADVISORY / FINDING / UNVERIFIED` truthfully, with exact source/time; never use `PASS` to imply an automated whole-lock scan was run. **No dependency remediation, CI edit, license-policy exception, secret access or new security tool installation is authorized** in this administrative run. If an actual applicable critical/high or malicious dependency is found, preserve the evidence safely and STOP before declaring the Stage 01 gate met; follow the private disclosure route when necessary.

### 5. Execution, publication and STOP

Read `README.md`, `AGENTS.md`, Architect/Implementation SOPs and all WIP files. Before any write verify actual one Solo Git root/worktree, `main`, sole `origin`, clean index except the exact Owner-installed `SOLO-WO-036 R01` ACTIVE replacement, no unexpected untracked files, and starting HEAD/tree. Match the starting blobs: Architect SOP `8760be323b6ed85695a9dcffd3e52f94628f0f07`; ROADMAP `6f753fdbfc34a08568f238769b558e93422688a6`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; previous report `d12453a7560e302840b3e2266c96304b2562ac0d`. STOP on drift, unexplained state or any required change outside the five-path ledger.

Perform only §§2–4. Keep the order's installation record unchanged until it has been committed, then perform the Owner-authorized neutralization in §3. Commit/push allowed files to `origin/main` under Architect SOP §6B; do not ask Owner to run routine Git. Verify the exact remote final HEAD/tree, both-host CI triggered by the published documentation changes (or report a real CI blocker), changed-path audit, four-file WIP inventory, no unauthorized bytes, clean local status, and scratch cleanup. Use only verified Solo `.temp/` or the current Cline task temp; remove owned obsolete scratch and preserve foreign material. No clones, worktrees, forced updates, broad Git clean/reset, branch/PR/merge, dependency install, provider or release.

**Canonical report:** overwrite `.documents/Reports/execution-report.md` once for this run, recording identity/baseline, actual committed revisions and scoped diff, documentary reconciliation, source-linked per-component security survey and explicit advisory-data freshness, which checks were **not run** (including `cargo audit`, if not run), current security gaps, CI final-HEAD checks, warnings, WIP state, cleanup, and no release/deploy. Distinguish Architect preliminary research from the Agent's first-hand checks; do not place the report's own final commit SHA inside its body. If an unresolved material security risk, false report, invalid WIP transition or failing mandatory CI check is discovered, report `STOPPED`/`FAILED` without claiming accepted closure.

**Handback:** final commit SHA/tree, the sole canonical report link, exact five-path-or-less change list, both-host CI evidence, security findings/limitations, retained R01.C and selected security NEXT, and the words **“agent report ready”** for independent Chief Architect review. Neither this administrative run nor its report awards Architect `ACCEPT`, completes R01.C/Stage 01 or authorizes the next candidate.
