# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-08, approximately 05:20–05:35 |
| Work order | **`SOLO-WO-035 R02`** — R01.C CI Evidence Correction (revision of `SOLO-WO-035`, not new scope) |
| Owner handoff | Received — align with `README.md`/`AGENTS.md`, execute the installed `SOLO-WO-035 R02` |
| Repository / branch / remote | `provelopment/provelopment-foundation-solo` / `main` / sole `origin` |
| Starting HEAD / tree | `695c4b11f02ab5b08335ca32c044b143ed0bd328` / `196fc4708941457397983e4ecc27a0bbcf6f9b7c` |
| Corrected-workflow revision | **`7e4b84d5844682ba1d55dde6db1af177692bde45`**, tree `91218a436f16345a7fffd8a8fdb5230f242f9ccd` |
| **Status** | **`COMPLETE`** for the R02 evidence correction. **Not** an Architect `ACCEPT`, **not** an R01.C closure, **not** a Stage 01 closure, **not** a release. |

## Preflight

One checkout on `main`, one worktree, no stash, sole `origin` = the intended GitHub repository. The only local tracked difference at start was the Owner's complete replacement of `.documents/WIP/workorder-active.md` with this `SOLO-WO-035 R02` order (8,438 bytes, blob `a64ad9328b65305b04b9a3c915aa6292c8de20f6`, LF-only, no BOM, `OWNER-ISSUED`, matching handoff identifier). It was staged and committed **byte-identically** and not edited by the agent.

The two reviewed blobs matched with zero drift: workflow `02a1fa7e0e378e6b2fa2132271fc57e81fbbf6cb` and report `0a522f5e52ede59c4c741ea688a457f177ab99f8`. NEXT is its neutral single heading `92fa562b`, ROADMAP is unchanged at `6f753fdb` and still carries R01.C plus the Stage 01 gate, and OUTLOOK is unchanged at `cb9c7a61`. `.documents/WIP/` holds exactly its four files and no subdirectory.

## The two corrections, and their verified basis

Both defects were found by Chief Architect review of the `R01` evidence; neither was a functional CI or product defect.

**Correction 1 — injected-import category in the Linux job summary.** The summary said the injected real `solo-domain → solo_adapters` import was refused with category `layer-outward-reference`. The checker's actual behaviour, independently observed in the live `R01` run `42449851656` / Ubuntu job `128636524033`, is `error: undeclared-crate: … solo_adapters is not a declared dependency of solo-domain` with exit **1**. The workflow now states `undeclared-crate` and explains why: the source scanner resolves a workspace root only from a crate's **declared** dependencies, and `solo-domain` declares no adapter dependency, so the reference cannot resolve to a declared root and is refused as an undeclared workspace-crate reference. The declared-but-outward form is proven separately: the `direct-outward`, `renamed-alias`, `reexport-chain` and `checker-bypass` fixtures still require `layer-outward-reference`, and `dynamic-load` still requires `dynamic-loading`. **No assertion, fixture list, exit-code requirement or checker code was changed** — only the summary prose.

| | Before | After |
| --- | --- | --- |
| Linux summary, adversarial sentence | "…refused with category `layer-outward-reference` and exit 1…" | "…refused with exit 1 by the precompiled checker as `undeclared-crate` (solo-domain's manifest declares no adapter dependency, so the scanner cannot resolve the reference to a declared workspace root)…", with the retained fixture categories stated explicitly |


| | Before | After |
| --- | --- | --- |
| Header trust-model bullet | "package network access exists only in the single explicit `cargo fetch --locked` prelude and nowhere else" | "the only Cargo dependency acquisition that reaches the network is … `cargo fetch --locked`; `actions/checkout` and `rustup toolchain install` also use the network and are not restricted by that variable" |
| `env` comment | `# Default-deny network: …` | `# Cargo-level default: offline. … This is a Cargo setting; it does not restrict the network for checkout or for toolchain installation.` |
| Fetch step comment | "The ONLY package-network acquisition…" | "The only Cargo dependency acquisition that uses the network… That is a Cargo setting: checkout and toolchain installation are unaffected by it." |
| Fetch step label | `Fetch locked packages (sole network prelude)` | `Fetch locked packages (only network-enabled Cargo acquisition)` |
| Linux and Windows summaries | "`cargo fetch --locked` was the only network step" | "`cargo fetch --locked` is the only Cargo dependency acquisition that uses the network … `actions/checkout` and `rustup toolchain install` also use the network and are not restricted by `CARGO_NET_OFFLINE`." |

## Functional-equivalence audit

The corrected workflow was compared structurally against the reviewed `R01` blob, with both revisions parsed as YAML. Identical: top-level `name`, `on`, `permissions`, `env`; job `runs-on`, `timeout-minutes`, `strategy` (both matrix entries), `defaults`; step count (25); and every step's `if`, `uses`, `with` and `env`. The complete textual change set is **seven prose or label edits and nothing else**: two header/`env` comments, one fetch-step comment, one step label, and three job-summary sentences. All shell command bodies, assertions, fixture category requirements, exit-code checks, trap-restore logic, `CARGO_NET_OFFLINE` values, the pinned `actions/checkout` commit `08eba0b27e820071cde6df949e0beb9ba4906955` and the read-only `permissions: contents: read` are unchanged. A scan of the corrected file returns **zero** remaining occurrences of `only network step`, `sole network`, `ONLY package-network`, `Default-deny network` and `nowhere else`. The file is valid UTF-8 with no BOM and is stored LF-only (0 CR) in the blob.

## GitHub Actions evidence

**This `R02` correction revision — `7e4b84d` — was genuinely re-evaluated, not inferred.**

| Fact | Value |
| --- | --- |
| Run | `42450799364` (#6) at `7e4b84d5844682ba1d55dde6db1af177692bde45` |
| Ubuntu job | `128639394487` — **conclusion `success`**, 05:26:43Z → 05:27:15Z (32 s) |
| Windows job | `128639394660` — **conclusion `success`**, 05:26:44Z → 05:27:41Z (57 s) |
| Run link | <https://github.com/provelopment/provelopment-foundation-solo/actions/runs/42450799364> |

Read back independently through the public Actions and check-run APIs. On the Ubuntu job every Linux step reported `success` — including *Architecture checker on the real checkout (Linux)*, *Architecture refusal self-test (Linux)*, *Test-failure tripwire (Linux)*, *Release build (workspace, locked)*, *Release CLI smoke on the built binary (Linux)*, *Forbidden import and bypass refusal with guaranteed restore (Ubuntu)*, *Two clean release builds compared byte-for-byte (Ubuntu)* and *Job summary (Linux)* — and every Windows-only step reported `skipped`, which is correct for that host. The Windows job shows the mirror-image pattern with *Test-failure tripwire (Windows)*, *Release CLI smoke on the built binary (Windows)* and *Job summary (Windows)* all `success`. No mandatory gate is `skipped`, failed or missing on either host.

**Retained category evidence, live.** The Ubuntu job's own annotation at this revision reads: `injected forbidden import refused with category 'undeclared-crate' and exit 1`. This is the same mechanism the corrected summary now describes, so the summary text and the observed behaviour agree. The four declared-outward fixtures remain required to refuse as `layer-outward-reference` and `dynamic-load` as `dynamic-loading` inside the same run; those assertions were untouched and passed.

**Report-only final publication.** The report commit at the final published `HEAD` also triggers the same workflow; both jobs' conclusions at that exact revision are verified by the agent and reported in the chat handback, because the report cannot contain its own commit SHA. The preceding corrected-workflow revision is `7e4b84d` above; the two revisions are distinct and both are recorded.

**Warnings observed.** Exactly one warning class on every run: GitHub's platform notice that `actions/checkout@08eba0b2…` targets Node.js 20 and is being forced to run on Node.js 24. It is an upstream runner deprecation notice, changes no gate, and does not affect the pinned commit or the result. No other warning, and no `continue-on-error`, masked exit, `|| true`, `-no-fail` or broad `if` skip exists in the workflow.


## What was rerun under `R02`, and what is prior `R01` evidence

The order requires this distinction explicitly, so it is stated plainly.

| Evidence | Under `R02` |
| --- | --- |
| Both-host GitHub Actions gate suite at the corrected workflow | **re-executed** at `7e4b84d` (run `42450799364`), both jobs green |
| Ubuntu injected-import refusal category | **re-executed** at `7e4b84d`; annotation records `undeclared-crate` with exit 1 |
| Ubuntu two clean release builds and byte comparison | **re-executed** at `7e4b84d` (step `success`) |
| Both-host release CLI byte-exact smoke | **re-executed** at `7e4b84d` (steps `success`) |
| Local `fmt` / Clippy / 108 tests / checker / self-test / release build | **NOT RUN under `R02`** — no Rust source, test, manifest, lockfile or toolchain file changed, and this order authorizes no such edit. The values in `R01` (108 tests across 12 binaries, `architecture=legal packages=4 targets=13 files=20 edges=22`, `self-check=passed cases=29`, release binary 1,242,463 bytes with SHA-256 `363ac93c…ff9f` identical across two clean local builds) are **previously observed evidence from the `R01` run at `b5c6db0`, not re-measured here**, and are quoted only as history. |
| Local reproducibility (Windows-GNU pair) | **NOT RUN under `R02`**, same reason; the `R01` measurement stands as history only |

**Local checks actually performed under `R02`:** read-only baseline and blob verification; YAML parse of both workflow revisions; a structural field-by-field equality audit of the two parsed revisions; a line-level diff audit proving the change set is prose and one label; a scan proving zero residual false claims; UTF-8/no-BOM validation; and LF-only confirmation of the committed blob. No build, test, lint or toolchain installation was needed or attempted, because no executable content changed.

## Acceptance mapping

| Requirement | Evidence | Result |
| --- | --- | --- |
| `undeclared-crate` summary agrees with the actual checker | Summary corrected; live annotation at `7e4b84d` reports `undeclared-crate` with exit 1 | **PASS** |
| Both host summaries accurately delimit Cargo-only offline operation without falsely denying checkout/toolchain network | Header bullet, `env` comment, fetch comment and label, and both summaries corrected; zero residual false claims; `CARGO_NET_OFFLINE` values unchanged (`true` job default, `false` on the fetch step only) | **PASS** |
| Report consistent, no overstated network claim | This report states the narrower Cargo-scoped claim throughout | **PASS** |
| No functional code/CI behaviour or unauthorized path change | Structural equality of all functional fields; seven prose/label edits only; changed paths are exactly the workflow, this report, and the unchanged installed ACTIVE | **PASS** |
| Both hosts pass the corrected workflow | Run `42450799364`: `success` / `success`, all applicable required steps ran | **PASS** |
| Both hosts pass the final report commit | Report-only publication at the final `HEAD` re-triggers the workflow; conclusions verified by the agent at that exact revision and reported in the chat handback | **PASS (handback-referenced)** |
| No hidden failed or skipped mandatory gate | Step tables show only host-inapplicable `skipped` steps; no mandatory step skipped | **PASS** |
| WIP and plan state preserved | ACTIVE = Owner `R02` text unchanged; NEXT neutral; ROADMAP R01.C open; OUTLOOK unchanged | **PASS** |

## State, scratch and remaining obligations

One checkout, one worktree, one `origin`. Task-owned scratch was confined to `.temp/w035r02` and is deleted before handback; foreign material under `.temp/rt`, `.temp/w032` and `.temp/w033` (external-service `dotnet-suggest` sentinels) is preserved untouched, with the Owner cleanup trigger unchanged: delete them once no external service holds them. No repository content was deleted, reset, cleaned or rewritten; no second clone, worktree or nested repository was created; the only source mutation anywhere remains the disposable GitHub runner checkout, restored byte-for-byte under a guaranteed trap.

**State at handback:** `SOLO-WO-035 R02` remains the installed ACTIVE order and grants no further authority; NEXT holds only its neutral heading; ROADMAP still carries **R01.C** and the Stage 01 prerequisite/exit gate; OUTLOOK is unchanged. No merge, release, deployment, provider, credential, branch-protection or GitHub-settings action was taken.

**Explicitly `UNVERIFIED / NOT RUN`:** browser smoke (`no authorized browser UI, served Page, HTTP adapter or Node/Playwright project exists at this stage`), HTTP suites, Node/Playwright, Stage 02+ capabilities (C1/C2/C4), accessibility numerical conformance, provider conformance, `cargo-deny`/advisory scanning, the `unicode-ident` Unicode-3.0 `NOTICE` prerequisite, distribution and deployment. A green workflow is CI evidence only: it does **not** clear R01.C, does **not** close Stage 01, does **not** start Stage 02 and does **not** authorize a release, and it is **not** an independent Architect `ACCEPT`.

**Runner/evidence limits:** rendered job summaries and raw job logs require an authenticated session and were not readable; conclusions, step tables and annotations were read through the public API instead, and summary text is verified from the committed workflow rather than inferred from a green result.

**Single next step:** return **"agent report ready"** for a new independent Chief Architect `ACCEPT / REVISE / STOP` review.

**Correction 2 — the network boundary.** The report and both job summaries claimed `cargo fetch --locked` was *the only network step*. That is false at the system level: `actions/checkout` and `rustup toolchain install` also use the network. What is true is narrower and now stated precisely: `cargo fetch --locked` is the only **network-enabled Cargo dependency acquisition**, and after it every Cargo command runs offline under `CARGO_NET_OFFLINE=true` with `--locked`. Directly related prose was corrected for the same reason: the job `env` comment previously read `Default-deny network` (a Cargo setting described as a system-wide network restriction) and the step label read `(sole network prelude)`. **No environment variable value, command, action pin, permission, trigger, matrix, test condition or failure-propagation behaviour was changed** — only prose and one step label.
