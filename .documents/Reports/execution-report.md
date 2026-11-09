# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-09 |
| Work order | **`SOLO-WO-045 R02`** — R02.B first bounded slice |
| Owner handoff | `SOLO-WO-045 R02` (matching; the order had been replaced by the Owner locally) |
| Required starting HEAD / tree | `732203b7436603a705d169b368a7734ed72febbd` / `44b2abdef8d2b183be5eec7b8a5beee6b0d70bf5` — **matched exactly** |
| R02 issuance commit / tree | `be367d34335d8d229611dae0a934fb2f81503998` / `b53d457f0198985bc1576af652a4214070d4df09` |
| Implementation commit / tree | `cf15cd5377663145e349f5344dedd593146f6396` / `1ba765005219a0935d8b3d9804deb645af5ac0e6` |
| CI run | `42553390272` — Ubuntu job `128953690724`, Windows job `128953691010` |
| **Status** | **`STOP` at §5.2 verification** — the slice is implemented and every local and hosted gate but one passes; a **read-only CI constant** blocks green CI |

## Outcome

I executed `SOLO-WO-045 R02` for real this time: §5.1 issuance, then the whole §5.2 bounded source change. **The R01 blocker is genuinely fixed** — `workspace_graph.rs` now carries the reviewed two-owner rule and passes against real Cargo metadata on both hosts.

The run then stopped on a **second, previously unlisted instance of the same defect class**: the mandatory workflow `.github/workflows/rust-verify.yml` hardcodes the real-tree architecture report to `edges=22`, and the authorized dependency promotion necessarily makes it `edges=24`. The workflow is read-only under §1 and §3E forbids editing it, so the run stops here rather than weakening a gate or editing a forbidden path.

This is the same kind of defect R02 was issued to repair — R01 forgot `workspace_graph.rs`; R02 forgot that its sibling assertion lives in the workflow.

Everything else is verified, and the verification is unusually strong: on **both** hosts steps 1–16 passed, including the four-category `cargo-deny` gate, the advisory negative probe, format, strict Clippy and the entire all-target test suite. Only the architecture-checker step failed.

## What was delivered and published

All work is confined to the §1 ledger. Exactly ten paths changed, with surgical diffs (no line-ending rewrites):

| Path | Operation | Lines |
| --- | --- | --- |
| `crates/solo-domain/src/source_locator.rs` | create | +711 |
| `crates/solo-domain/src/lib.rs` | narrow module, docs, re-exports | +11 |
| `crates/solo-domain/tests/source_locator.rs` | create | +395 |
| `crates/solo-adapters/Cargo.toml` | two normal dependencies | +3 |
| `Cargo.lock` | two direct references in the adapter stanza | +2 |
| `crates/solo-adapters/src/publication_manifest.rs` | create | +1766 |
| `crates/solo-adapters/src/lib.rs` | narrow module, exports, docs | +22/−7 |
| `crates/solo-adapters/tests/publication_manifest.rs` | create | +597 |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | shared rule + closure | +67/−4 |
| `crates/solo-cli/tests/workspace_graph.rs` | invariant replacement + adversarial cases | +259/−24 |

**`source_locator.rs`** is `std`-only. It validates one logical `sourceCandidates` locator against the exact §6.2C.1–§6.2C.2 grammar (namespace, byte/segment/depth ceilings, per-segment charset, trailing-dot and doubled-dot rules, reserved Windows stems, and precise refusals for root, drive, UNC, `.`/`..`, backslash, colon, tilde, query, fragment, percent and control characters), builds a non-empty one-or-two member declaration set, and reserves claims across representations so two declarations cannot hold one logical path **including an absent one**. It compares logical paths only and states, in code and in a testable constant, that filesystem alias/hardlink/case identity is **not proven**.

**`publication_manifest.rs`** deserializes into an ordered-pair tree through `serde`'s `visit_map`, refusing a duplicate member **during map access** at every depth, before any map could let a second member win. It then enforces the closed five-key root, the nested one-Entity grammar, ids validated through the accepted `EntityIdV1`/`FacetIdV1`/`SiteIdV1`/`PageIdV1`, per-parent uniqueness, the 10,000/500/100 budgets, authored ordering, and closed syntactic sub-shapes for `base`, route, `sourceCandidates`, `entryPaths`, `missingTranslation`, `visibilityDefault` and inert bounded `properties`. Locales stay raw authored strings and are never passed to `LocaleKeyV1::from_already_canonical`. The output type is `ParsedManifestCandidateV1`, documents each deferred semantic as *not proven*, and exposes `is_publication_authorized() == false`.

## The blocker, exactly

The mandatory workflow asserts the real-tree architecture report's edge count on **both** hosts:

| Host | Line | Assertion |
| --- | --- | --- |
| Ubuntu 24.04 | `.github/workflows/rust-verify.yml:568` | `grep -q 'edges=22'` → else `echo "::error::approved edge count changed"; exit 1` |
| Windows 2025 | `.github/workflows/rust-verify.yml:580` | `if ($text -notmatch 'edges=22') { throw "approved edge count changed" }` |

The checker now reports, and is correct to report:

```
architecture=legal packages=4 targets=16 files=26 edges=24
```

The two extra edges are exactly the two edges the order commands: `solo-adapters → serde` and `solo-adapters → serde_json`. Baseline was 22 because every `resolve` edge is counted, including third-party-internal ones. No order-compliant change can avoid this: declaring the dependencies is the point of §1, and the count is structural, not a policy knob.

§1 lists `.github/` as read-only *"including `rust-verify.yml`"*, and §3E states *"Changes to the CI workflow, architecture contract or whole-test policy are forbidden."* I therefore did not touch it, and I did not weaken, skip or re-point anything to manufacture a green badge.

**Minimal Owner decision needed:** authorize a one-line update in the workflow, on both hosts, changing the expected real-tree edge count from `22` to `24`, justified by the two newly approved adapter edges. No other change is required: with that single constant updated the remaining gates can run, and three of them were already rehearsed locally with the pinned toolchain.

## Verified evidence

**Hosted CI (`run 42553390272`) — identical outcome on both hosts.** Steps 1–16 succeeded; step 17/18 failed:

| Step | Ubuntu | Windows |
| --- | --- | --- |
| Install / assert pinned toolchain | success | success |
| Fetch locked packages (only network Cargo acquisition) | success | success |
| Component security gate — `cargo-deny 0.20.2` | success | success |
| Advisory negative probe — RUSTSEC-2026-0009 | success | n/a (Linux-only) |
| Workspace package-set gate | success | success |
| Format check | success | success |
| Strict lint (warnings are errors) | success | success |
| **Test (workspace, all targets, locked)** | **success** | **success** |
| **Architecture checker on the real checkout** | **failure — `approved edge count changed`** | **failure — exit 1** |
| Architecture refusal self-test onward | skipped | skipped |

Annotation on the Ubuntu job: `failure: approved edge count changed`, plus the notice that the negative probe refused `time 0.3.46` with `RUSTSEC-2026-0009` while the locked workspace graph passed all four checks.


**Local verification with the pinned `1.99.1` toolchain** (GNU host variant acquired into task-owned `.temp/` for verification only — no tracked path, no second checkout):

| Command | Result |
| --- | --- |
| `cargo fetch --locked` | pass — proves the hand-written lock stanza is consistent and adds no component |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | pass, zero warnings |
| `cargo test --workspace --all-targets --locked` | **pass — every target, 0 failures** |
| Refusal self-test matrix | `self-check=passed cases=29`, `ok` 29, `FAIL` 0, `refused as` **23**, `legal form accepted` **5** — the mandatory fixtures are intact and unchanged |
| Release build `--workspace --locked` | pass |
| Release CLI smoke (replica of the CI step) | accepted exit 0 with byte-exact `preflight=ok entity_id=entity-7 base=https://example.com/alpha` and empty stderr; refused exit 2 with `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` and empty stdout |
| Test-failure tripwire | normal run passes; `SOLO_CI_INJECT_TEST_FAILURE=1` fails as designed |

The architecture checker passing on the real tree is the load-bearing proof that the boundary work is genuine: `solo-domain` and `solo-application` still reach **no** external crate, the adapter may reach only `serde`, `serde_json` and their enumerated locked closure, and the adapter still cannot depend outward on `solo-cli`. The source checker needed **no** edit — it already derives a package's allowed normal roots from real metadata — so `source.rs`, `mod.rs` and `check_architecture_fixtures.rs` are byte-identical, and the 23 fixtures are untouched.

## Dependency and security transition

| Property | Before | After |
| --- | --- | --- |
| Locked packages | 15 | **15 — unchanged** |
| Lock version | 4 | 4 |
| Adapter external normal roots | none | `serde =1.0.229`, `serde_json =1.0.154` |
| `solo-cli` externals | `serde_json`, `syn` (dev) | unchanged, still dev-only |
| Adapter reachable closure | none | `itoa`, `memchr`, `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_derive`, `serde_json`, `syn`, `unicode-ident`, `zmij` — enumerated in one reviewed constant |

The promotion moves `serde`/`serde_json` from development reachability to a **shipped** adapter dependency, so the prior "zero third-party normal dependencies" finding no longer holds and is not repeated. The 15-package set, versions, sources and checksums are identical to baseline, so no new component, version movement or checksum change entered the graph.

First-hand security evidence from the two hosted jobs: `cargo-deny 0.20.2` with a fresh RustSec database (`7a40350e1b8ba5b656d909f88222a8059a83d88d`, fetched 2026-11-09T02:53:57Z on Ubuntu and 2026-11-09T02:54:22Z on Windows) reported **advisories, bans, licenses and sources all ok** on the changed graph, and the Ubuntu disposable negative probe still refused `time 0.3.46` with `RUSTSEC-2026-0009`. No suppression, severity change, scanner change or unapproved vendor input was used.

## What remains open in R02.B

Unchanged by this run and still required: official parser/Unicode/schema conformance; resolved-source no-follow confinement and snapshot atomicity; existing-invalid-JSON refusal without fallback; absence-versus-I/O-error semantics; effective JSON/Markdown selection and body validity; canonical full-region locale validation; route, base and property/registry semantics; and independent publication-authority acceptance. `R02.B` and `R02.C` remain open, Stage 02 remains open, and this slice's parse is never publication-authorized.


## State ledger and honest limitations

**WIP is exactly four files and unchanged by this run:** `workorder-active.md` holds the complete R02 order at blob `1953f0fb39f37b4911e2855d6bcc566a37f5a775` (31,310 bytes, installed), `plan-next.md` stays neutral at `92fa562b1af0c54391851aa5c79deb1631127069` (49 bytes), `plan-roadmap.md` is byte-identical at `18975332c2820170a32558c4ee224176667b91ab` with `R02.B` and `R02.C` still open, and `plan-outlook.md` is byte-identical at `cb9c7a610cf3c537557e060ca9656e44a053e69d`. I did not clear ACTIVE, did not populate NEXT and did not touch ROADMAP, because §5.5–§5.6 are gated on independent ACCEPT and no ACCEPT exists.

**Not proven, and not claimed.** The byte-reproducibility gate (two clean release builds compared byte-for-byte) was skipped by CI behind the failing step and was **not** run locally; it must be treated as `NOT RUN`. The same applies to the hosted refusal self-test, tripwire, release build and CLI smoke steps, which CI skipped — those four were rehearsed locally instead and are reported as local results, not as hosted passes. Dependabot and code-scanning alert endpoints returning 401 remain `UNVERIFIED`, never "clear". Browser, HTTP, accessibility, provider, UI, release and deployment remain `NOT APPLICABLE` to this parser slice. The `unicode-ident` Unicode-3.0 attribution/NOTICE obligation is still an open release-time requirement and is not waived. `rustup 1.29.3` was current in the verification environment; the tracked `rustup` patch note is unchanged.

**No red run was manufactured.** The two-host failure at `42553390272` is entailed by authorized work meeting a stale read-only constant, not by a speculative push. Publishing the implementation was necessary to obtain per-step hosted evidence and to avoid discarding approved work; the alternative — reverting a complete, locally green slice — would have destroyed the delivery and left the blocker unproven.

**Cleanup.** The task-owned toolchain, crate cache, build directory and scratch reports lived only in git-ignored `.temp/` and were removed at run end; nothing was added to version control there. No `target/` exists in the repository root, no WIP file or report copy was added, no fixture directory or archive was created, and no OS-temp scratch remains. No PR, merge, branch, stash, force-push, rebase, amend, tag, release, workflow dispatch/rerun/cancel, settings change, secret access or other-repository write occurred.

## Conclusion

**`SOLO-WO-045 R02` stops at §5.2 verification on a read-only CI constant, with the authorized slice fully implemented and every other gate green on both hosts.**

The R01 defect is genuinely closed: the real-metadata two-owner rule is in place, `solo-domain` and `solo-application` still reach no external crate, and 23 refusal fixtures remain untouched. The new blocker is a single stale integer in `.github/workflows/rust-verify.yml` that the order forbids me to change.

Resuming requires one Owner decision — authorize updating `edges=22` to `edges=24` in both the Linux and Windows steps, justified by the two approved adapter edges — after which the remaining gates can run unchanged. The Chief Architect's disposition on this revision is `ACCEPT` of the stop and its evidence, `REVISE` with the workflow constant authorized, or `STOP`.

**`agent report ready`**

