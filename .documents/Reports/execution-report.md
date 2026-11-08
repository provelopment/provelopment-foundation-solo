# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions, including the `SOLO-WO-040 R01` report this one supersedes without rewriting. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 13:05–13:30 |
| Work order | **`SOLO-WO-040 R02`** — A1 Version-Policy Evidence and Documentation Correction |
| Matching Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-040 R02` |
| Installed ACTIVE | blob `c1654c5f58150f104c38acf1052f43745dd68778`, **15,360 bytes**, 94 lines, LF, no BOM, `OWNER-ISSUED`, retained **byte-for-byte unchanged** |
| Starting HEAD / tree | `19f1e9191d7e75fe775dff12917b053843571aa1` / `a3de01b30a11d5f80b664b8fff9998f29bbc3025` |
| Issuance record commit | `c839aaf01cad19bee124b6cbf6417e69f19265b7` |
| **Source revision** | `ee844ad0bed49f945cce1836b84257e07e67bd0e`, tree `b8813a0e41700cc02cca6ec90128582b9a18b532`, `README.md` blob `ce2d1cf9aee4a1805772dc797f437d081bd00540` |
| **Status** | **`COMPLETE`** — a bounded factual correction of the four defects the Chief Architect's `REVISE` identified. Not a product, dependency or CI change; not an Architect `ACCEPT`; `R01.C` and Stage 01 remain open. |

## Preflight

Verified the single Owner checkout, `main`, sole `origin` equal to the remote head, one worktree, no stash, and a working tree whose **only** tracked difference was this Owner-installed ACTIVE replacement. All **fifteen** stated baseline blobs matched with **zero drift**: `README.md` `d17f19b8…`, report `d9e2a2fd…`, pre-install ACTIVE `dbbb2b4d…`, `sop-architect.md` `1483c9be…`, `sop-implementation.md` `51bb3792…`, `ARCHITECTURE.md` `4b050f63…`, `AGENTS.md` `6eae1da8…`, `dependabot.yml` `1874726f…`, workflow `5426c1f5…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`, ROADMAP `e6ea66cf…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…`. Exactly four WIP files, no WIP subdirectory. `README.md`, `AGENTS.md`, both SOPs, Architecture §6A.2 (including the canonical `@axe-core/playwright =4.14.0` row), all four WIP files, the previous report, `dependabot.yml` and the `rust-verify` workflow were read before any write. No rebase, reset or guess was needed.

## The authorized change — exactly three paths

| Path | Operation |
| --- | --- |
| `README.md` | one substitution inside the single stale passage of "Installation, authoring and lifecycle" (diff: 1 line removed, 1 line added) |
| `.documents/WIP/workorder-active.md` | committed unchanged as issuance history |
| `.documents/Reports/execution-report.md` | this document, overwritten |

Nothing else changed. `AGENTS.md`, both SOPs, Architecture, `dependabot.yml`, the workflow, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/SECURITY.md`, every crate manifest, source and test, ROADMAP, NEXT, OUTLOOK and the licence/notice files remain byte-identical to the starting HEAD, and no dependency, toolchain, scanner, action, runner or product-contract version was altered.

## Correction A — the README contradiction

The passage claimed: *"There is no application code, installed toolchain, dependency manifest, test suite, CI job, release or deployment."* That sentence is now replaced by the actual distinction, and no stale global denial of the existing manifests, tests or CI remains in the modified passage:

- **The engineering baseline exists and is verified** — the four-crate inward-only workspace (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`), recorded exact toolchain pin, tracked manifests and `Cargo.lock`, executable architecture and unit tests, and the mandatory Linux/Windows CI proving the security gate, architecture and refusal checks, strict build, lint and tests, byte-reproducible release build and CLI smoke evidence.
- **The Solo EFS product does not exist** — its JSON/Markdown publication validators, real website/browser/HTTP surface, installation, release, deployment and the later §6A/§6C promises remain unimplemented and unverified.
- **No local installation is implied** — pinning a toolchain in the repository and executing it in hosted CI does not install it on any user's machine.

Everything around the substitution is preserved verbatim: the conditional §6A selection and its still-owed conformance proof, the §6.2 schemas that are specified without a schema file, validator or command, the documentary §6C decisions, the accepted `§1.1–§1.4` identifier grammar and publication-declaration semantics, Apache-2.0, and the rule against borrowing historical Foundation commands. The corrected passage now agrees with the unchanged `AGENTS.md` §7 entry (`6eae1da8…`), which independently records the same four crates, the same lock, the same two hosts and the same denial of the product, browser surface, release and deployment.

## Correction B — the exact-version error (`@axe-core/playwright`)

The `R01` report recorded the future browser test stack as `@axe-core/playwright` **`14.4.0`**. That was **wrong**, and this report corrects it instead of hiding it: the binding Architecture §6A.2 row selects **`@axe-core/playwright =4.14.0`** (with `@playwright/test =1.64.1`), read from the unchanged `ARCHITECTURE.md` blob `4b050f63…` at line 1254 and echoed by the §6A.13 future test stack.

- **Value recorded:** `@axe-core/playwright =4.14.0`, `@playwright/test =1.64.1`, both **Architecture-selected and not installed**.
- **Architecture pin:** not edited, not reclassified, not "corrected" — the report was the defect, not the contract.
- **npm freshness: `UNVERIFIED`.** No npm registry release metadata was queried in this run and no package was installed, so nothing here claims that `4.14.0`, `1.64.1`, TypeScript `=7.0.2`, Vite `=8.3.6` or `ts-rs =12.0.1` is the current publishable version. Their standing is *selected-but-not-installed*, exactly as the canonical text says.
- **Node.js `26.13.0`** remains a **selected future pin, not an installed Node application**; the November 8, 2026 observation in the `R01` ledger that it matched the then-current LTS is retained as a dated historical fact, not as a present-tense freshness claim.
- No version number anywhere was adjusted to look newer.

## Correction C — the pull-request inventory

`R01` stated that the repository had "**0** pull requests in any state". That statement was **false**, and the mechanism matters: an *unauthenticated* REST call to `/repos/.../pulls?state=all` returned an empty array, and that artefact was reported as the repository's inventory. Freshly queried on 2026-11-08 for this report:

| Query | Result |
| --- | --- |
| GitHub pull-request inventory for this repository (authenticated tooling) | **exactly one PR in total** |
| `GET /repos/.../pulls?state=all&per_page=100` (unauthenticated) | `[]` — an empty body, which is the source of the earlier error |
| `GET /repos/.../pulls/1` (unauthenticated) | `404 Not Found` |
| `GET /repos/.../issues?state=all` (unauthenticated) | 1 entry, carrying `pull_request`, i.e. the same PR |

**The one historical pull request:**

| Field | Value |
| --- | --- |
| Number | **#1** |
| Title | `docs(security): establish security register and codify security by design (SOLO-WO-019 R02)` |
| Author | `provelopment` (Owner's own account — **not** Dependabot) |
| State | **closed**, `merged: false` |
| Created / closed | `2026-11-07T03:06:07Z` / `2026-11-07T04:11:07Z` |
| Head → base | `docs/solo-wo-018-security-register` → `main` |
| Relation to this work | predates it; it belongs to `SOLO-WO-019 R02`, not to the Dependabot configuration added on 2026-11-08 |

**Corrected statement.** The repository has had exactly **one** pull request ever — a historical, closed, unmerged Owner-authored PR (#1). It has **zero open pull requests**, and **zero Dependabot-authored pull requests**: the initial configuration-triggered Dependabot update jobs produced none, consistent with the version ledger showing nothing outdated. "Zero open update PRs" is therefore not the same claim as "no PR ever existed", and this order created, closed, approved and merged no pull request.

## Correction D — Dependabot warning attribution and classification

`R01` attributed the `ubuntu-latest` migration notice to both Dependabot update jobs. Rechecked directly for this report, with the retrieval time recorded:

- **Check-run annotations — observed by this agent.** `GET /check-runs/{id}/annotations` returned **HTTP 200** for both jobs (`128727737592`, the Cargo updater; `128727737337`, the GitHub Actions updater) at `2026-11-08T13:12:15Z` and `13:12:16Z`. Each carries **exactly one** annotation, level `notice`, `path=.github`: *"The ubuntu-latest label will migrate to Ubuntu 26 beginning October 19, 2026. For more information, see https://github.com/actions/runner-images/issues/14748"*. The `R01` attribution was therefore annotation-supported rather than invented — but an annotation is not the raw updater log, and the two are not interchangeable.
- **Raw updater logs — not retrievable by this agent.** `GET /actions/jobs/{id}/logs` returned **HTTP 403** for both jobs. Everything below that comes from raw logs is attributed to the **independent Chief Architect's retrieval**, not to this agent; my own reproduction of those lines is `UNVERIFIED`.
- **Raw-log messages the Architect's review identified, classified without suppression:**

| Message | Source | Severity / classification | Affects Solo source, CI or toolchain? | Disposition |
| --- | --- | --- | --- | --- |
| `rehash: warning: skipping ca-certificates.crt,it does not contain exactly one certificate or CRL` | certificate-store rehash inside the Dependabot **updater container** | benign warning about a bundle file in the updater's own trust store; not a dependency, version or deprecation notice | **No** — Solo's runners, crates, toolchain and actions are unaffected | recorded here, no action; not suppressed, muted or ignored |
| `Failed to parse GITHUB_REGISTRIES_PROXY environment variable` | Dependabot updater diagnostic, empty proxy environment value | benign configuration diagnostic; no registry proxy is configured for Solo | **No** — Solo uses crates.io and public actions only | recorded, no action; no setting changed |
| *ubuntu-latest → Ubuntu 26 migration* | upstream runner-image announcement, `actions/runner-images` issue #14748, rollout 2026-10-19 → 2026-11-19 | upstream platform notice attached to the Dependabot job (annotation `path=.github`), concerning **GitHub's own** updater runner label | **No** — the `rust-verify` matrix explicitly pins `ubuntu-24.04` and `windows-2025`, which is why neither `verify` job shows it | discussed as a **separate upstream proposal** only; no runner migration is approved by R02 and no matrix label changed |

**Net assessment.** Neither updater message proves an outdated installed Solo dependency, an EOL component or an actionable `rust-verify` warning. At this revision **no actionable version, deprecation, unsupported-runtime or end-of-life warning remains** for Solo: the only annotations on the two `verify` jobs are the four expected gate, architecture and advisory-probe notices. Two pre-existing parser notices in the Windows job-summary string remain unchanged from the baseline; they are documentation-string diagnostics, not runtime warnings, and that step succeeds. The `ubuntu-latest` item stays an upstream runner-image announcement and an open proposal — not a Solo defect and not a passed item.



## Change-scope evidence

| Path | At source revision | Note |
| --- | --- | --- |
| `README.md` | changed, blob `ce2d1cf9…` | one line replaced; 1 insertion, 1 deletion |
| `.documents/WIP/workorder-active.md` | committed unchanged | blob `c1654c5f…`, equals the installed order |
| `.documents/Reports/execution-report.md` | overwritten | this document |

Every other path — `AGENTS.md` `6eae1da8…`, `sop-architect.md` `1483c9be…`, `sop-implementation.md` `51bb3792…`, `ARCHITECTURE.md` `4b050f63…`, `dependabot.yml` `1874726f…`, `rust-verify.yml` `5426c1f5…`, `Cargo.lock` `05a02666…`, `deny.toml` `3063e864…`, `rust-toolchain.toml` `9ec55b3b…`, ROADMAP `e6ea66cf…`, NEXT `92fa562b…`, OUTLOOK `cb9c7a61…` and all first-party source — is byte-identical to the starting HEAD. No gate, threshold, scanner rule, suppression, setting or policy was touched, so nothing was bypassed to make this documentation order pass.

## CI evidence at the source revision — both hosts green

Run [`42483885541`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42483885541) at `ee844ad0…`, inspected step by step with annotations read directly:

- **`ubuntu-24.04` — [job `128741448810`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42483885541/job/128741448810) — success, 30 s, 0 failed steps.** All Linux steps succeeded: pinned-SHA checkout without persisted credentials, pinned toolchain install and exact `rustc`/`cargo` assertion, locked fetch, `cargo-deny 0.20.2` gate from the verified archive (`sha256 9f12ed4c…`, RustSec `309392b1…` fetched `2026-11-08T13:13:13Z`, advisories/bans/licenses/sources all ok), package-set gate, format, strict lint, all-target tests, architecture checker on the real checkout, architecture refusal self-test, test-failure tripwire, release build, release CLI smoke, forbidden-import refusal with guaranteed restore, two byte-identical release builds, and the job summary. The advisory negative probe refused `time 0.3.46` with **`RUSTSEC-2026-0009`** and exit 1 while the unchanged locked graph passed.
- **`windows-2025` — [job `128741448832`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42483885541/job/128741448832) — success, 69 s, 0 failed steps** (19 succeeded, 12 Linux-only steps skipped by design): Windows identity, exact toolchain assertion, locked fetch, the Windows `cargo-deny` gate (`sha256 975a2214…`, RustSec `309392b1…` fetched `2026-11-08T13:13:31Z`), package-set gate, format, strict lint, all-target tests, Windows architecture checker and refusal self-test, tripwire, release build, Windows CLI smoke and summary.

Advisory database state is therefore current at the source revision: RustSec commit `309392b1…` fetched twice on 2026-11-08, with both hosts confirming all four check classes on the unchanged graph.

**Final report-only HEAD.** This document is part of that commit, so its own run cannot be described inside it. After publication the same `rust-verify` jobs were inspected on both hosts at the final HEAD and are reported in the chat handback together with the final commit SHA and tree. No report-only commit was repeated to manufacture a success.


## Acceptance conditions

| # | Condition | Result | Evidence |
| --- | --- | --- | --- |
| 1 | Factual correction: a minimal diff to the single README passage distinguishes the verified engineering foundation from the unimplemented EFS product and does not claim a locally installed toolchain; no stale global denial remains in it; `AGENTS.md` and the tree agree | **PASS** | one line replaced (1/1); all four stale denials absent; baseline, product and toolchain-caveat statements present; `AGENTS.md` `6eae1da8…` unchanged and consistent |
| 2 | Evidence/report precision: `@axe-core/playwright =4.14.0` recorded as Architecture-selected and not installed; true PR inventory with a fresh query; the actual certificate-rehash and proxy diagnostics classified; `ubuntu-latest` treated as upstream unless personally observed; no false `PASS`, fabricated test or npm-freshness claim | **PASS** | corrections B, C and D above, each with query path, timestamp and evidence owner; npm freshness explicitly `UNVERIFIED` |
| 3 | No change to gates or policy: A1 §7B(e2), both SOPs, Architecture pins, Dependabot config, `rust-verify`, action SHA, toolchain, locked graph and first-party source byte-identical; only three paths changed from the starting HEAD | **PASS** | change-scope table; 15/15 starting blobs matched before writing and all non-authorized paths still match at the source revision |
| 4 | Exact CI evidence at the source and final HEAD on both hosts, with live verified `cargo-deny`, fresh RustSec, all four checks, strict build/lint/tests, architecture/refusal, smoke, the Ubuntu `RUSTSEC-2026-0009` probe and byte-identical release builds; logs and annotations read where available | **PASS at the source revision** | run `42483885541`: 0 failed steps on either host; annotations read directly; final HEAD reported in the handback |
| 5 | Repository hygiene and WIP: only permitted paths pushed, clean local status, ROADMAP `R01.C` and Stage 01 still open, NEXT neutral, OUTLOOK unchanged, four WIP files, installed ACTIVE unmodified, scratch removed, no closure or reconciliation | **PASS** | see below; no closure performed |

## Unrun and unverified evidence

- **Browser/HTTP smoke: `NOT RUN`** — no authorized application or browser surface exists.
- **Dependabot raw updater logs: `UNVERIFIED` for this agent (HTTP 403).** The two updater diagnostics are recorded from the Architect's independent retrieval and are not claimed as personally reproduced.
- **npm registry freshness for the future browser stack: `UNVERIFIED`** — nothing was installed and no npm metadata was queried.
- **Dependabot security alerts: `UNVERIFIED`** — the alerts API returned 401 to this agent in the previous run and no access changed.
- **Dependabot's scheduled Monday run: not observed** — only the configuration-triggered update jobs exist so far.
- **No local Rust build, lint or test was run** — this machine has no local Rust toolchain; all build evidence is hosted CI, and it is described as such.

## Repository state and cleanup

One checkout, one worktree, no stash, sole `origin`, clean working tree after publication, exactly four WIP files with no subdirectory, no second report or work-order file, and the installed ACTIVE unmodified. Task scratch stayed under `.temp/w040r2` and was removed; foreign material under `.temp/rt`, `.temp/w032`, `.temp/w033` and `.temp/architecture-fixtures` was left untouched. No dependency was installed, no setting changed, no branch, pull request, merge, release or deployment was created, no secret was accessed and no destructive Git operation was performed. Git history retains the superseded `R01` report unchanged; nothing was rewritten or backdated.

`R01.C` and Stage 01 remain **open**; the `ubuntu-26.04` runner question, enabling the separate `rust-toolchain` ecosystem and any future Dependabot proposal each still require their own Owner-authorized order. This run does not declare `ACCEPT`, does not close a stage and does not approve the Agent's own work. **"agent report ready"** — the final commit, tree and both-host final-HEAD verification are in the chat handback.

