# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-09 |
| Work order | **`SOLO-WO-045 R03`** — CI edge-count reconciliation and independent acceptance |
| Matching Owner handoff | `SOLO-WO-045 R03` |
| Required starting HEAD / tree | `d6c0bc764de7e7bc0e954963a70621be2a2a3a40` / `7ad47ed2e7e801c0b2ac9c8fcbda20404deafed7` — **matched exactly** |
| §2 baseline blob ledger | **22 of 22 blobs verified, zero drift** |
| R03 issuance commit / tree | `80d16992482e13d0add122bb9067698d95fb0be9` / `31ba6c69c51414a96a7e2b431d18ecf272ab2b53` |
| R03 workflow-fix commit / tree / workflow blob | `3a8af1bab7e511a722be8bf8045e5c20200c32aa` / `eccb693c1f4f8436869136becb94c5458e7de95b` / `c12e622bfe5a2c36be2d6c9dd99592dc86b74134` |
| Frozen R02.B code under review | `cf15cd5377663145e349f5344dedd593146f6396` / `1ba765005219a0935d8b3d9804deb645af5ac0e6` |
| Green two-host CI | run **`42557798689`** (push, `3a8af1b`) — Ubuntu job `128967312322`, Windows job `128967312395`, **both `success`** |
| **Status** | **`COMPLETE — evidence phase only`** |

## Outcome

The stale expectation is repaired and the whole mandatory workflow is now green on both hosts, through **every** gate. The only change to shipped verification machinery was the two authorized literals; the R02.B source, dependency graph, security controls and refusal fixtures are byte-unchanged from the reviewed commit.

Both previous STOPs remain in Git history and are neither waived nor retroactively accepted:

| Phase | Head | CI | Meaning |
| --- | --- | --- | --- |
| R01 | `be367d3` issuance | — | STOP: order omitted `workspace_graph.rs` from its ledger |
| R02 code | `cf15cd5` | run `42553390272` — FAILURE both hosts at `edges=22` | Implementation published; blocked by a read-only constant |
| R02 report | `d6c0bc7` | run `42554037361` — FAILURE both hosts at `edges=22` | Truthful STOP report |
| R03 issuance | `80d1699` | run `42557683069` — **expected FAILURE** both hosts at `edges=22` | Order issued under the still-stale workflow |
| **R03 workflow fix** | **`3a8af1b`** | run **`42557798689`** — **SUCCESS both hosts** | **Fully green** |

I verified the R03 issuance failure was the *same* mechanism, not a new defect, before repairing it: the Ubuntu annotation reads `failure: approved edge count changed`, with steps 1–16 already green.

## The workflow repair, exactly

`.github/workflows/rust-verify.yml`, blob `1c0e450c…` → `c12e622b…`, a two-line diff (2 insertions, 2 deletions), `git diff --check` clean, no line-ending change:

```diff
@@ -566,5 +566,5 @@
           grep -q 'architecture=legal packages=4' "$RUNNER_TEMP/arch-real.txt" \
             || { echo "::error::unexpected package set in real-tree report"; exit 1; }
-          grep -q 'edges=22' "$RUNNER_TEMP/arch-real.txt" \
+          grep -q 'edges=24' "$RUNNER_TEMP/arch-real.txt" \
             || { echo "::error::approved edge count changed"; exit 1; }
@@ -578,5 +578,5 @@
           if ($text -notmatch 'architecture=legal packages=4') { throw "unexpected package set in real-tree report" }
-          if ($text -notmatch 'edges=22') { throw "approved edge count changed" }
+          if ($text -notmatch 'edges=24') { throw "approved edge count changed" }
```

`edges=22` now appears **zero** times and `edges=24` exactly twice. Nothing else moved: step names, shell/`throw` behaviour, exit codes, the `architecture=legal packages=4` assertion, the real checker execution as its own process, the 23-refusal/5-legal matrix, the security gate, smoke tests and the Ubuntu byte-reproducibility comparison are all intact. `24` is a recorded expectation of the current approved graph, **not** a generic threshold: any future graph change still fails this gate until separately reviewed.

## Hosted evidence — every gate executed

Run `42557798689` at `3a8af1b`. **Ubuntu 24.04** (job `128967312322`, runner `GitHub Actions 1000002982`): 22 steps succeeded, 0 failed, 9 skipped as the genuine Windows-only counterparts. **Windows 2025** (job `128967312395`, runner `GitHub Actions 1000002983`): 19 succeeded, 0 failed, 12 skipped as the genuine Linux-only counterparts. No gate was skipped that should have run.

| Gate | Ubuntu | Windows |
| --- | --- | --- |
| Checkout (pinned full commit SHA, no persisted credentials) | success | success |
| Install and assert the pinned toolchain | success | success |
| Assert exact `rustc`/`cargo` versions | success | success |
| Fetch locked packages (only network Cargo acquisition) | success | success |
| Component security gate — `cargo-deny 0.20.2` | success | success |
| Advisory negative probe — RUSTSEC-2026-0009 | success | n/a (Linux-only) |
| Workspace package-set gate | success | success |
| Format check | success | success |
| Strict lint (warnings are errors) | success | success |
| Test (workspace, all targets, locked) | success | success |
| **Architecture checker on the real checkout** | **success** | **success** |
| Architecture refusal self-test | success | success |
| Test-failure tripwire | success | success |
| Release build (workspace, locked) | success | success |
| Release CLI smoke on the built binary | success | success |
| Forbidden import and bypass refusal with guaranteed restore | success | n/a (Linux-only) |
| **Two clean release builds compared byte-for-byte** | **success** | n/a (Linux-only) |
| Job summary | success | success |

Annotations on the green run:

- Ubuntu: `injected forbidden import refused with category 'undeclared-crate' and exit 1` — the load-bearing restore gate genuinely refused an injected violation and restored the tree.
- Ubuntu: `advisory negative probe refused time 0.3.46 with RUSTSEC-2026-0009 and exit 1, while the unchanged locked workspace graph passed all four checks`.
- Both: `component security gate passed: cargo-deny 0.20.2 … advisories, bans, licenses and sources all ok` — archive sha256 `9f12ed4c…` (Linux) / `975a2214…` (Windows), RustSec `7a40350e1b8ba5b656d909f88222a8059a83d88d` fetched `2026-11-09T03:42:01Z` (Linux) and `03:42:15Z` (Windows).

### Why these step results are themselves the proof of the counts

The workflow **encodes** the exact expectations as failing assertions, so a green step is not a claim I am making — it is the gate refusing to pass otherwise:

- the architecture step fails unless the report matches `architecture=legal packages=4` **and** `edges=24`, which is why the two-host result proves the checker printed `edges=24` at `3a8af1b`;
- the refusal self-test step fails unless `self-check=passed cases=29`, `^ok ` count is 29, `^FAIL` count is 0, `refused as ` count is **23** and `legal form accepted` count is **5**, plus the checker-bypass and dynamic-load probes — all of which therefore held;
- the tripwire step fails unless the normal run passes *and* the deliberately injected failure is refused;
- the reproducibility step fails unless two clean release builds are byte-identical.

Raw job logs returned **HTTP 403** for this token, so I could not read log bodies first-hand. I am reporting that plainly rather than implying I read them: the evidence above is step-level conclusions plus the job annotations, and the per-count proof rests on the workflow's own failing assertions rather than on log text.


## Security and dependency evidence

| Property | Value |
| --- | --- |
| Locked packages | **15** (4 workspace + 11 external) — unchanged names, versions, sources and checksums |
| Lock version | 4 |
| `Cargo.lock` blob at start and unchanged since | `36125ff1be3eec79b30c5370a7ac4e2642e5d10d` |
| Adapter external **normal** roots | `serde =1.0.229`, `serde_json =1.0.154` |
| Adapter enumerated reachable closure | `itoa`, `memchr`, `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_derive`, `serde_json`, `syn`, `unicode-ident`, `zmij` |
| `solo-cli` externals | `serde_json`, `syn` — **dev only**, unchanged |
| `solo-domain`, `solo-application` externals | **none of any kind** |

**Runtime reachability changed and is not waved through.** `serde` and `serde_json` were previously reached only as development material of `solo-cli`; they are now **shipped** normal dependencies of `solo-adapters`, so the earlier "zero third-party normal dependencies" finding no longer holds and is not repeated here. What was re-measured first-hand at this commit is the consequence: a fresh two-host `cargo-deny 0.20.2` over the expanded shipped graph returned **advisories, bans, licenses and sources all ok**, and the disposable negative probe still refused the known-vulnerable fixture — i.e. the scanner is genuinely load-bearing, not merely quiet. No suppression, severity override, scanner substitution or vendor input was used, and no applicable critical/high or malicious component was left unresolved.

## The frozen R02.B first slice

Independently re-confirmed as published, byte-identical to the reviewed blobs, and **not edited** by this order:

| Property | State |
| --- | --- |
| `crates/solo-domain/src/source_locator.rs` | `2b507a8a…` — `std`-only logical `pages/` locator, precise refusals, non-empty set, logical reservation incl. absent candidates |
| `crates/solo-adapters/src/publication_manifest.rs` | `f883dd93…` — duplicate-member refusal during map access at every depth; closed five-key root; exact `1.0.0`; size/UTF-8/BOM refusals |
| Public parser outcome | `ParsedManifestCandidateV1`, untrusted and non-publishable; `is_publication_authorized()` is constant `false` |
| `graph.rs` / `workspace_graph.rs` | `774d49fb…` / `9d9d2157…` — one reviewed owner/kind rule, enumerated closure, unconditional real-metadata test, adversarial negatives |

**Proven by this slice:** strict duplicate rejection before value materialisation (including nested arbitrary and `properties`-nested duplicates); one Entity with accepted ID grammar, cardinality, per-parent uniqueness and authored ordering; budgets; inert bounded `properties`; declared and validated `sourceCandidates` with extension matching, reserved-stem and traversal refusal, and duplicate logical reservation even for a path that does not exist; a syntactically accepted declaration is never publication-authorized.

**Explicitly *not* proven and not claimed:** filesystem no-follow confinement, alias/hardlink/case collision beyond logical comparison, snapshot atomicity, absence-versus-error semantics, effective JSON-first selection, selected body validity, canonical BCP47/Unicode/NFC conformance, route/URL/property-registry/visibility semantics, and any routing, HTTP, browser, provider, publication, release or deployment behaviour. `R02.B` and `R02.C` remain **open**; `C1` and `C2–C4` are **not** claimed complete; Stage 02 is not closed.

## Versions and environment

`rust-toolchain.toml` remains pinned at `1.99.1` with `rustfmt` and `clippy` and is byte-unchanged (`9ec55b3b…`). The two "Assert exact rustc/cargo versions" steps succeeded on both hosts, so the measured compiler and Cargo matched the pin exactly. Runner labels were `ubuntu-24.04` and `windows-2025`. Action pins, Node runtime and `deny.toml` (`3063e864…`) are unchanged, and the security gate re-verified its own archive digest (`9f12ed4c…` Linux, `975a2214…` Windows) rather than trusting a download.


## Warnings, unknowns and limitations

- Raw job logs: **HTTP 403 — `UNVERIFIED`** for bodies; step conclusions and annotations only, as stated above.
- GitHub Dependabot and code-scanning alert endpoints remain **`UNVERIFIED`** (401) — never "clear". A scheduled Dependabot run is **`NOT OBSERVED`**, which is not a scan.
- Browser, HTTP, accessibility, provider, UI, release and deployment checks: **`NOT APPLICABLE`** to this parser/CI slice, never PASS.
- Release-only `unicode-ident` Unicode-3.0 attribution/NOTICE remains an **open** release obligation and is not waived.
- Ubuntu `rustup 1.29.2` versus Windows `1.29.3` is a tracked low-urgency parity proposal, **not** an authorized upgrade; no toolchain change was made.
- No warnings were suppressed, no step relabelled, and no green status is asserted without the gate inspection above.

## State ledger and cleanup

`.documents/WIP/` holds exactly four files with no subdirectory:

| File | Blob | State |
| --- | --- | --- |
| `workorder-active.md` | `543cc7ae0d1466f40d4fb8649b2d1dc0ef76c088` | **R03 order installed** (not neutralized — §4.4 ACCEPT has not occurred) |
| `plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` | **neutral, untouched** (population is gated on ACCEPT) |
| `plan-roadmap.md` | `18975332c2820170a32558c4ee224176667b91ab` | **byte-identical**; `R02.B`/`R02.C` and Stages 02–13 intact |
| `plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` | byte-identical |

I did **not** self-accept the slice, close `R02.B`, neutralize ACTIVE or populate NEXT: §4.4 requires the Chief Architect's independent disposition first. Three paths differ from the starting HEAD, exactly as §1 permits: ACTIVE (issuance), the workflow (repair) and this report. `plan-next.md` is deliberately left neutral.

**Cleanup.** R03 produced no build or toolchain artifacts: no local toolchain was installed this run, no `target/` exists in the repository, and I created no scratch files. Task-owned material from the previous run had already been removed, and pre-existing `.temp/` entries belonging to other tasks were left untouched. No PR, merge, branch, extra worktree, stash, amend, rebase, force-push, reset, tag, release, manual workflow dispatch/rerun/cancel, settings change or other-repository write occurred.

## Disposition requested

The **evidence phase** of `SOLO-WO-045 R03` is complete: the full mandatory workflow is green on both hosts at the workflow-fix commit, with every gate — including refusal self-test, tripwire, release smoke and byte-reproducibility — executing rather than skipped.

The Chief Architect should verify at exact GitHub HEAD — the two-line workflow diff, the unchanged R02.B code blobs, the real binary/test/refusal evidence, the shipped-dependency exposure and the run and job identifiers above — and return **`ACCEPT`**, **`REVISE`** or **`STOP`**. On `ACCEPT` of this bounded first slice only, §4.5–§4.6 remain: populate one remaining-R02.B NEXT candidate, verify its both-host CI, then neutralize ACTIVE to blob `ea5f20b736483c2e102456be1cea9104059fc81a` in its own commit and publish the final report. None of that has been started, and a separate order will be required before the next remaining-R02.B slice.

**`agent report ready`**

