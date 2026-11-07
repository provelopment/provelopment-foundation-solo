# Foundation Solo — Security Issues Register

> **Canonical register.** This file is the single authoritative, public-safe record of security findings, external advisories watched, and proposed preventive controls for Foundation Solo. It is **evidence-linked documentation**. It is **not** a work order, not an authorization to change code, and **not** a fifth WIP slot: work remains authorized only by an `OWNER-ISSUED` order in [ACTIVE](../WIP/workorder-active.md).

## 1. Purpose, boundaries and assessed scope

This register exists so that a security finding raised against Solo has one durable, reviewable home with a stable identifier, an honest applicability verdict, a named owner and a testable closure condition. It records what was actually observed, distinguishes vendor-level risk from Solo-specific applicability, and states plainly what was not examined.

| This register **is** | This register **is not** |
| --- | --- |
| A register of findings, advisories under observation, and proposed preventive controls | An implementation authorization, backlog or work-order queue |
| A public-safe summary of evidence and applicability | A confidential disclosure channel — never a place for exploit recipes or sensitive data |
| A durable record with stable IDs and change history | A replacement for ROADMAP, NEXT, ACTIVE or OUTLOOK |
| A place for proposed, unimplemented controls | A per-task report archive or a second execution report |

**Assessed snapshot**

| Field | Value |
| --- | --- |
| Repository | `provelopment/provelopment-foundation-solo` (public) |
| Revision assessed | `0a948be028dffbbe2e737900d1df185e8ccfea17` on `main` |
| Assessment date | 2026-11-07 |
| Assessor | Implementation Agent, under `SOLO-WO-018 R01` |
| Independent review | Pending Chief Architect `ACCEPT / REVISE / STOP` on the opened pull request |
| Repository state observed | 19 tracked files, 503,920 bytes; documentation and governance only |
| Environments **in** assessed scope | The published Git repository at the revision above, including all tracked files and repository-native configuration |
| Environments **out** of assessed scope | Any hosting account, DNS/TLS configuration, CMS installation, CI/service configuration, email or form provider, secret store, and any web property not in this repository — none of which exists for Solo yet, and none of which is verifiable from this checkout |

**Three distinct categories are kept separate throughout.** (1) **Current findings** — grounded in evidence from the assessed revision. (2) **External advisories and watch items** — external product vulnerabilities tracked because they may become relevant, with an explicit applicability verdict. (3) **Future hardening requirements** — proposed preventive controls to evaluate during later authorized lifecycle or security work; these are not defects in existing code and are not implemented protections.

A finding's presence here never authorises remediation work. Remediation requires its own separately issued order.

**Protocol.** This register is the recording surface of the six-step security lifecycle — **identify → verify → assess → prevent/respond → validate → reassess** — whose architecture constraints are in [Architecture](../Architecture/ARCHITECTURE.md) §5A, whose review procedure is [Architect SOP](../SOP/sop-architect.md) §7C, and whose agent duties are [Implementation SOP](../SOP/sop-implementation.md) §7A. Entries record the evidence that pipeline produced; they never substitute for it, and no entry becomes work outside an Owner-issued order.

## 2. Public-repository safety and reporting channel

This repository is public, so this register is public. It therefore carries **no** live secrets or tokens, no personal data, no internal hostnames or addresses, no confidential attack paths, and no exploit recipes or reproduction payloads. Findings are described at the level already published by vendors and national advisories.

**Do not report a new vulnerability here or in any public issue, pull request or discussion.** Solo's reporting channel is GitHub **private vulnerability reporting**, enabled for this repository and configured in [`.github/SECURITY.md`](../../.github/SECURITY.md): **Security → Advisories → Report a vulnerability** (<https://github.com/provelopment/provelopment-foundation-solo/security/advisories/new>). Confidential evidence stays in that private channel; only a sanitised, non-operational summary is later added here after triage and, where appropriate, coordinated disclosure.

| Severity | Meaning in this register |
| --- | --- |
| Critical / High / Medium / Low / Informational | Impact if the finding actually applies to Solo, prioritising Solo-specific reachability over the vendor's headline score |
| Status: Open / Investigating / Mitigated / Resolved / Risk accepted / Not applicable | `Mitigated` means exposure is reduced but the condition remains; it is **never** treated as equal to `Resolved`, which requires tested evidence |

## 3. Finding index

| ID | Title | Source / reference | Source severity | Solo-specific risk | Affected component in Solo | Impact | Status | Owner | Priority | Evidence date | Revisit trigger |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `SOLO-SEC-2026-001` | WordPress core full-update window — potential unauthenticated remote code execution | [CVE-2026-119977](https://cveawg.mitre.org/api/cve/CVE-2026-119977); Cyber Centre [AV26-1117](https://www.cyber.gc.ca/en/alerts-advisories/wordpress-security-advisory-av26-1117); [WordPress 7.1.4](https://wordpress.org/download/releases/) | **High** — CVSS 3.1 base 8.1 (`AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:H`), listed in CISA KEV | **None observed** — no WordPress present in the assessed revision | None | No impact on the assessed Solo revision | **Not applicable — assessed Solo repository at `0a948be0`** | Owner / TBD | High for tracking only; no Solo remediation outstanding | 2026-11-07 | If Solo adopts WordPress, integrates a WordPress-managed service, or acquires an update mechanism with comparable time-of-check/time-of-use exposure |

**No other repository-backed finding exists at the assessed revision.** That is a statement about a documentation-only repository with no application, dependency manifest, CI workflow or deployment configuration — **not** an assurance that future software will be secure, and not a substitute for the application security testing that belongs to later code-bearing packages.

## 4. Detailed finding records

### `SOLO-SEC-2026-001` — WordPress core full-update window — potential unauthenticated remote code execution

#### 4.1 Verified source facts

| Field | Verified value |
| --- | --- |
| CVE | `CVE-2026-119977` — state `PUBLISHED` |
| Assigning CNA | Wordfence (CVE Program record) |
| Published / updated | 2026-10-31 / 2026-11-04 |
| Class | **CWE-367** Time-of-check Time-of-use (TOCTOU) race condition |
| Severity (vendor/national) | **CVSS 3.1 base 8.1 — HIGH**; vector `AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:H` |
| Affected product | WordPress Core, versions from 4.7.0 through the 7.1 branch **prior to 7.1.4** |
| Fixed version | **7.1.4**, released **2026-10-28** (WordPress release archive) |
| Exploitation status | CISA **KEV**, entry dated **2026-11-03**; SSVC recorded as *Exploitation: active*, *Automatable: no*, *Technical Impact: total* |
| National advisory | Canadian Centre for Cyber Security **AV26-1117**, dated 2026-11-03: “WordPress Core Prior to 7.1.4”, recording the CISA KEV addition of this CVE on the same date |

#### 4.2 Sources consulted and retrieval

Retrieved 2026-11-07 for this assessment:

1. **CVE Program record** — <https://cveawg.mitre.org/api/cve/CVE-2026-119977>. Primary source for state, CNA, CVSS vector and score, CWE class, affected/fixed versions, KEV metadata and vendor references.
2. **Canadian Centre for Cyber Security advisory AV26-1117** — <https://www.cyber.gc.ca/en/alerts-advisories/wordpress-security-advisory-av26-1117>. Corroborates the affected range and the KEV date.
3. **WordPress release archive** — <https://wordpress.org/download/releases/>. Confirms 7.1.4 as released 2026-10-28 and as the maintained 7.1 release.

**Source limitation, recorded honestly:** the NVD detail page (<https://nvd.nist.gov/vuln/detail/CVE-2026-119977>) returned only a JavaScript application shell with no record content when retrieved, so **NVD was not independently confirmed**. The CVE Program record and the national advisory above are sufficient and are the sources cited. The CISA KEV **catalogue page itself was not separately retrieved**; the KEV entry date is taken from the CISA ADP enrichment inside the CVE record and corroborated by AV26-1117.

Facts asserted by the originating request were independently re-checked against these sources rather than accepted, as the order required; the affected range, fixed version, release date, severity and KEV date all matched.

#### 4.3 Applicability to the assessed Solo revision

**Verdict: `Not applicable — assessed Solo repository at 0a948be028dffbbe2e737900d1df185e8ccfea17`.**

This verdict is deliberately narrow. It states that the assessed repository contains no affected component. It does **not** state that Solo is “fixed”, “mitigated”, immune, or that it can never be affected — no code, CMS or deployment exists to be affected at this revision, and an external WordPress installation is not cleared by anything below.

| Check actually run at `0a948be0` | Result |
| --- | --- |
| `git ls-files` inventory | 19 tracked files, 503,920 bytes |
| Executable or script or config files (`.js .ts .php .py .rb .go .rs .sh .ps1 .java .cs .c .cpp .html .css .json .yaml/.yml .toml .lock`) | **0** |
| Non-Markdown tracked files | `LICENSE`, `.editorconfig`, `.gitattributes`, `.gitignore` only — licence and tool-neutral hygiene, no code |
| Dependency manifests and lockfiles (`package.json`, `composer.json`, `Gemfile`, `go.mod`, `requirements.txt`, `pyproject.toml`, `Cargo.toml` and equivalents) | **0** |
| WordPress-specific paths (`wp-config.php`, `wp-content/`, `wp-includes/`) | **0** |
| `git grep -i -E 'wordpress\|wp-config\|wp-content\|wp-includes'` | 12 lines, **all** in `SOLO-WO-018 R01`'s approved-scope text and the execution report — governance prose that *documents this advisory*. No code, configuration, manifest or integration declaration references WordPress |
| CI/CD workflows (`.github/workflows/**`) | **0** — `.github/` contains four Markdown community files only |
| Deployment, release, environment or infrastructure artifacts (`.env`, container, IaC, Helm, Ansible) | **0** |
| Released or deployed application version | **None** — `README.md` and `AGENTS.md` both record no implemented application, test suite, release or deployment |

There is no web root, no runtime, no package ecosystem and no update mechanism in this repository, so the time-of-check/time-of-use condition described by the advisory has nothing to act on here.

#### 4.4 Evidence gaps and what remains uncleared

- Any WordPress installation, plugin-managed site or hosted service operated outside this repository is **outside** the assessed scope and **not** cleared. Clearing it would require separate, explicitly authorized evidence from that system.
- No hosting, DNS, TLS, provider, mailbox or form-delivery configuration was inspected, because none is recorded in this repository and no provider access was authorized.
- Absence of a vulnerability cannot be inferred from code search alone once code exists; this verdict rests on the total absence of an application, not on absence of a matching string.

#### 4.5 Threat and impact at vendor level

The advisory describes a race condition in WordPress core's full-package update process that can be exploited during the vulnerable window to achieve remote code execution on an affected installation, without authentication and without user interaction in the recorded vector. Because exploitation is recorded as active in KEV, an affected installation should be treated as urgent, not theoretical. Impact on such a system would be total for confidentiality, integrity and availability. No attack steps, payloads or reproduction details are recorded in this register, by policy (§2).

**Impact on Solo at the assessed revision: none**, because no affected component is present (§4.3).

#### 4.6 Conditional remediation and preventive requirements if applicability changes

Recorded as requirements to execute **only** if Solo later adopts WordPress or an equivalent update mechanism, under a separate authorized order. Nothing here is implemented.

| Requirement | Owner | Verification |
| --- | --- | --- |
| Before upgrading any vulnerable installation, restrict HTTP access and PHP execution in update staging paths | TBD — whoever operates the installation | Request to a staging path returns a denial; no code executes from it |
| Apply the fixed release (7.1.4 or the current maintained branch) and record the exact version | TBD | Version string confirmed post-upgrade; the affected version is retired |
| Review update-window logs for suspicious activity and assess compromise indicators | TBD | Log review recorded with date, scope and conclusion |
| Confirm access controls and version after every future update | TBD | Repeatable check recorded per update |

#### 4.7 Validation and closure criteria

The finding may move from `Not applicable` only with evidence, and only in the direction the evidence supports:

- **To `Open`/`Investigating`** — a WordPress installation, dependency or WordPress-managed service is identified in an authorized scope, with its version recorded.
- **To `Resolved`** — the operating owner supplies post-remediation evidence: fixed version confirmed, staging paths unreachable from the public network, no execution possible there, and log review completed. Tested evidence is mandatory; a proposed fix alone never closes a finding.
- **This finding never becomes `Resolved` by Solo work alone** while no system exists; the correct status at any documentation-only revision is `Not applicable` with the assessed revision named.

#### 4.8 Reassessment trigger

Reassess this entry when any of the following occurs: Solo adopts WordPress or a WordPress-managed service; a first application, dependency manifest, CI workflow or deployment configuration is added; an update mechanism with comparable time-of-check/time-of-use exposure is designed; or the CVE/KEV record changes materially. The trigger is recorded so this entry is re-read rather than silently inherited by later revisions.

#### 4.9 Proposed preventive controls — future assessment only

Implementation-agnostic properties worth designing for when Solo's release and update lifecycle is authorized. These are **proposals to assess**, not implemented protections and not defects in nonexistent code:

- integrity-checked or signed release artifacts, verified before use;
- immutable or atomic publication, so a partially written state is never servable;
- least-privilege filesystem and runtime permissions for any update process;
- no public reachability for update staging, temporary or unpacking directories;
- monitoring and alerting on update activity, with a tested rollback path.

They are recorded here so later lifecycle and security work has a starting checklist; accepting or rejecting any of them is a separate Owner decision.

#### 4.10 Worked trace through that lifecycle

This case is the register's first complete pass through the pipeline, at the planning stage where Solo has no code to test:

| Step | What this case actually did | Evidence |
| --- | --- | --- |
| Identify | An external WordPress advisory was raised against a Solo repository that does not use WordPress | §4.1 |
| Verify | Source facts re-derived from the CVE record, a national advisory and the vendor release archive; the NVD page was unreadable and that limitation is disclosed | §4.1–§4.2 |
| Assess | Applicability tested against the exact revision `0a948be0`: no WordPress, no dependency manifest, no CI, no deployment | §4.3–§4.4 |
| Prevent/Respond | No remediation justified; preventive requirements proposed for any future update mechanism with comparable exposure | §4.6, §4.9 |
| Validate | Vendor-level impact separated from Solo risk; closure criteria stated as conditional evidence tests, not as a claim that tests passed | §4.5, §4.7 |
| Reassess | Explicit trigger recorded: WordPress adoption, a comparable update mechanism, or any unexamined external system | §4.8 |

The point of the example is procedural, not sensational: a genuinely severe vendor vulnerability correctly produced **no Solo defect**, because the assessment stayed evidence-bound. Every entry in this register must be able to show the same trace, or state honestly which step could not be completed.

## 5. Methodology, checks actually run, exclusions and limitations

**Method.** Read-only inspection of the exact assessed revision: tracked-file inventory, structural classification of every tracked file, targeted content searches, manifest/CI/deployment presence checks, and comparison against the governing documents. External advisory facts were re-derived from primary sources (§4.2) rather than reused from the request that raised the finding.

| Check | Outcome | Status |
| --- | --- | --- |
| Repository and working-tree preflight (root, branch, `main`, remote URL, one worktree, status) | Verified in the single checkout before any write | **PASS** |
| Tracked-file inventory and classification at `0a948be0` | 19 files; 0 executable/script/config; 4 non-Markdown hygiene/licence files | **PASS** |
| Dependency manifest and lockfile search | None present | **PASS** (nothing to audit) |
| WordPress path and reference search | Only governance/report prose that documents the advisory | **PASS** |
| CI/CD, deployment, environment and infrastructure artifact search | None present | **PASS** (nothing to audit) |
| External advisory verification (CVE record, national advisory, vendor release archive) | Confirmed as recorded in §4.1–§4.2 | **PASS** |
| NVD record confirmation | Page returned a JavaScript shell only | **NOT RUN** — limitation recorded |
| Dependency vulnerability scanning (`npm audit`, `composer audit`, equivalent) | No dependency ecosystem exists | **NOT APPLICABLE** |
| Static analysis / linting of application code | No application code exists | **NOT APPLICABLE** |
| Unit/integration/CI test execution | No test suite or CI configuration exists | **NOT APPLICABLE** |
| Application penetration testing or live exploitation | Explicitly out of scope and prohibited | **NOT RUN** by design |
| Hosting, provider, DNS/TLS, mailbox and form-delivery review | No such configuration is recorded; no access authorized | **NOT APPLICABLE** (nothing to review) / **out of scope** where a system could exist externally |

**Exclusions and unknowns.** Everything outside this repository is excluded: any hosting account, external web property, CMS installation, provider console, secret store and account/DNS/TLS setting. Where such a system exists outside this repository, this register neither clears nor implicates it. No credential, secret or personal data was accessed.

**Limitation to state plainly.** A clean result on a documentation-only repository says nothing about the security of software that does not yet exist. It is not a certification, not a statement of product security maturity, and not a substitute for application security testing once code-bearing work begins.

**Future review checklist — apply as capability arrives.** When a first application, dependency or deployment exists, re-review this register for: dependency audit results and pinned versions; secret handling and configuration isolation; authentication and authorization boundaries; input validation, uploads and path handling; output encoding, CSRF/XSS and security headers/CORS; the Ports-and-Adapters boundary, ensuring adapters do not leak provider details inward and no location/locale inference crosses it; unlisted Page noindex and exact-URL behaviour; publication atomicity and update integrity (§4.9); and logging, backup and recovery assumptions. Add findings with new stable IDs as evidence emerges — never as a generic checklist dressed up as a confirmed vulnerability.

## 6. Maintenance rule and change history

**Upkeep.** This single file is the whole register; no per-finding files, copies or parallel trackers are created. Identifiers are stable and never reused: `SOLO-SEC-<year>-<sequence>`. A new finding is added with its sources, evidence date, honest applicability verdict and owner; existing findings are updated in place with their history preserved. Findings discovered privately are triaged through the private channel (§2) and summarised here only when publication is safe and coordinated. Edits to this register are documentation changes and still require authorized scope — this file confers no authority to remediate. A finding is never marked `Resolved` without tested evidence, and `Mitigated` is never treated as `Resolved`.

This register is not a fifth WIP slot: the four WIP files remain exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md` and `plan-outlook.md`.

| Date | Author | Revision | Change |
| --- | --- | --- | --- |
| 2026-11-07 | Implementation Agent (`SOLO-WO-018 R01`) | 1 | Register created; assessed revision `0a948be0` recorded; `SOLO-SEC-2026-001` added as `Not applicable` to the assessed scope with verified source facts, reassessment trigger and proposed preventive controls |
