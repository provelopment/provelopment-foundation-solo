# Execution Report

**Work order:** `SOLO-WO-046 R01` — R02.B second bounded slice: provisional JSON/Markdown source selection plus inert body-byte preflight.
**Disposition:** Chief Architect **ACCEPT**, received for the technical-evidence phase and for the second bounded R02.B slice, reviewed at report HEAD `1cd2751` / tree `bb718fb8`.
**Lifecycle:** §§5.5–5.6 executed under the existing order. The one remaining-R02.B candidate is published in NEXT, ACTIVE is neutral, ROADMAP is unchanged, and `R02.B`, `R02.C` and Stage 02 remain open.

## 1. Verified two-host green evidence

| Revision | Tree | What it is | Run | ubuntu-24.04 | windows-2025 |
| --- | --- | --- | --- | --- | --- |
| `4fa9b51` | `f8561f12` | implementation of this slice | `42569678884` | job `129003918555` success | job `129003918886` success |
| `1cd2751` | `bb718fb8` | report revision the Architect accepted | `42570426821` | job `129006305275` success | job `129006305283` success |
| `3eb0a33` | `c23443c6` | NEXT reconciled to one remaining-R02.B candidate | `42571585235` | job `129009945031` success | job `129009944716` success |
| `3cdc556` | `4874c204` | ACTIVE restored to the approved neutral file | `42571864925` | job `129010827487` success | job `129010827724` success |

Every applicable step passed on both hosts in each of those runs: toolchain assertion, locked fetch, the `cargo-deny 0.20.2` four-category gate with a fresh RustSec refresh, the package-set gate, format, strict Clippy with `-D warnings`, the full workspace test suite, the real-tree architecture checker with its 23 refusal fixtures, the test-failure tripwire, the release build, the release CLI smoke, the injected-forbidden-import refusal, and two byte-identical release builds.

**This revision is verified by the run it triggers.** That outcome is not asserted in advance here: each lifecycle step above is verified at its own revision, and the final verification of this exact commit is identified by the run given with the handoff.

## 2. The source correction this slice required

The exact-size test helper emitted malformed JSON. As a raw string, its `head` literal ended at the string terminator immediately after the opening bracket, swallowing the quote that should open the filler string, while its `tail` literal still supplied a closing quote. The generated body had an unquoted identifier where a string element belonged, so the exactly-at-limit case could never parse, and the test failed honestly instead of the boundary being tolerated with a malformed body.

Both literals are now escaped ordinary strings, so the filler is the single string element inside `blocks` and the body is valid in every other respect. Measured: head 52 + filler 2097097 + tail 3 = exactly 2097152 bytes, one element, parsing as JSON. No limit was raised, no assertion weakened, and no production validation changed. Committed as `4fa9b51`.

Three earlier corrections are part of the same accepted record: `f902bec` applied the formatting changes without which the format gate never passed, `d1f2171` resolved E0106 in both `observations_both` helpers, and `1b75e2e` gave the no-echo fixture the member whose absence was masking the assertion it targets.

## 3. Accepted scope

* `crates/solo-domain/src/source_selection.rs` (420 lines) — provisional selection over caller-supplied observations. Read order is fixed and independent of declaration order: a declared `json` member is considered first, present JSON ends the decision, and only a genuinely reported `Absent` permits considering `markdown`. A named failure is never converted into absence, and no fallback follows invalid selected JSON.
* `crates/solo-domain/tests/source_selection.rs` — 14 tests covering read order, reported absence, four distinct failure kinds, refusal of observations for undeclared members, redundant Markdown refusal, empty bytes versus absence, determinism, and diagnostics that never echo a body.
* `crates/solo-adapters/src/source_body_preflight.rs` (503 lines) — inert byte preflight of the selected body only: size envelope, strict UTF-8 without a byte-order mark, strict JSON with duplicate members refused at any depth, and the closed body root (`contentVersion` 1.0.0, `title` to 140 scalars, optional `summary` to 500, `blocks` 1 to 250).
* `crates/solo-adapters/tests/source_body_preflight.rs` — 18 tests covering the closed root, scalar and block bounds, byte-level faults, syntax faults, duplicates at root and depth, exact size boundaries for both kinds, non-echoing refusals, owned bytes, and five in-memory pipeline cases.

## 4. Security and dependency evidence

* `cargo-deny 0.20.2` verified archives: Ubuntu sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`, Windows sha256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`; RustSec `7a40350e1b8ba5b656d909f88222a8059a83d88d` fetched at `2026-11-09T05:51:48Z` and `05:52:30Z`; advisories, bans, licenses and sources all ok on the unchanged locked graph.
* The negative probe resolves a disposable `time 0.3.46` manifest, refuses it with `RUSTSEC-2026-0009` and exit 1, and leaves the real locked graph passing all four checks, so the gate is proven capable of failing.
* An injected forbidden import is refused with category `undeclared-crate` and exit 1, after which the tree is restored.
* The architecture checker reports the approved inward-only graph on both hosts (`packages=4 targets=18 files=30 edges=24`, `self-check=passed cases=29`, as independently read by the Architect from the raw logs); the checkout carries 23 refusal fixtures.

## 5. Correction of an earlier claim in this report

This report previously recorded the Ubuntu `cargo-deny` acquisition failure as outstanding and needing an Owner re-run. **That claim is withdrawn.** Run `42570194179` was transiently red at step 9, before `cargo-deny` executed, because the runner could not download the pinned release asset (`http 403`); its Windows leg was already green. Run `42570426821` then passed the entire workflow, every applicable step on both hosts, at the same report HEAD `1cd2751`. Nothing from that transient remains outstanding, and it was never a security finding, a masked gate or a repository defect.

## 6. WIP transitions

* **NEXT** moved from the neutral file (`92fa562b`) to the single remaining-R02.B candidate (`17b12f17`) at `3eb0a33`. The candidate carries both accepted slices forward as delivered work it must not rebuild and marks `READY` only on that recorded evidence, disclosing one authorization dependency rather than an unmet prerequisite: the native confinement mechanism, and any new vetted component, need their own Owner-issued order and security review. NEXT is never authorization.
* **ACTIVE** at `3cdc556` was restored to the approved neutral file: 40 bytes, blob `ea5f20b736483c2e102456be1cea9104059fc81a`, verified with `git hash-object` before the commit. An empty slot means only that no work is authorized; it is not a project pause, a freeze or a reopening gate.
* **ROADMAP** is byte-identical at `18975332`, so `R02.B`, `R02.C` and Stage 02 stay open. **OUTLOOK** is unchanged at `cb9c7a61`. The WIP holds exactly four files, with no subdirectory and no archive.

## 7. Remaining limitations

* No file is opened anywhere in the accepted code or in its tests. A declared `Absent` is a report, never proof of absence.
* Selection is provisional: `OBSERVATION_IS_FILESYSTEM_PROOF`, `PROVISIONAL_CHOICE_IS_FINAL_SELECTION` and `CONTENT_VALIDITY_PROVEN` are all `false`, and both authority accessors return `false`.
* No immutable snapshot exists, so nothing yet prevents a selected body from changing beneath a reader.
* Confinement, no-follow containment, and resolved alias, hardlink, separator and case collision refusal remain unproven. They are the NEXT candidate, not this slice.
* Markdown grammar, raw-HTML refusal, heading and anchor semantics, link and URL safety, `props`, `presentation` and module-registry validity are out of scope, including for the selected body.
* Nothing here is publishable, routable, visible or deployable: no route, representation, canonical, base URL, visibility or provider behaviour changed.

## 8. Preserved history

* The R01 STOP, the R02 STOP and the R03 ACCEPT stand unwaived. Each REVISE disposition on this slice, and the correction that closed it, remain in git history and in the commit messages that record them.
* This is the one canonical report: overwritten each run, never dated, duplicated or archived, and never a source of authority.

## 9. Changed-path ledger and invariants

Across the whole order, `8f81078..HEAD` changes exactly eight paths, all inside the nine permitted:

| Path | Change |
| --- | --- |
| `.documents/Reports/execution-report.md` | modified |
| `.documents/WIP/plan-next.md` | modified |
| `crates/solo-domain/src/lib.rs` | modified |
| `crates/solo-domain/src/source_selection.rs` | added |
| `crates/solo-domain/tests/source_selection.rs` | added |
| `crates/solo-adapters/src/lib.rs` | modified |
| `crates/solo-adapters/src/source_body_preflight.rs` | added |
| `crates/solo-adapters/tests/source_body_preflight.rs` | added |

`.documents/WIP/workorder-active.md` is the ninth permitted path and nets to no change: it was issued and then restored to its own baseline blob, so the final tree carries no installed order.

Unchanged against the baseline and verified by blob equality: `ARCHITECTURE.md`, both SOPs, `plan-roadmap.md`, `plan-outlook.md`, `Cargo.toml`, `Cargo.lock` (15 packages, identical blob), `rust-toolchain.toml`, `.cargo/config.toml`, `deny.toml`, `.github/workflows/rust-verify.yml`, the architecture checker sources, and the accepted R02.A and R02.B-first-slice domain and adapter sources and tests. No dependency, feature, target, workspace member, workflow, schema artifact or security rule was touched.

The repository holds one worktree and one remote, `origin` at the public Solo repository, with local `main` equal to `origin/main` and an empty working tree. No task-owned scratch remains: the order used no `.temp/` material this phase, and the disposable commit-message files it did create were removed. Four pre-existing `.temp/` folders from earlier runs (`architecture-fixtures`, `rt`, `w032`, `w033`) are not this order and were left untouched rather than deleted.

