# Execution Report

**Work order:** `SOLO-WO-046 R01` — R02.B second slice: provisional JSON/Markdown source selection plus inert body-byte preflight.
**Implementation revision verified green:** `4fa9b51d11095b1a7fb919df706653f607df42dc`, tree `f8561f12d7a8e15afc597cb39b64a524b540f3da`.
**Status:** **COMPLETE — fully green on both mandatory hosts.**

## 1. Status

Both hosts pass every applicable gate at `4fa9b51`: locked dependency fetch, the pinned `1.99.1` toolchain assertion, the `cargo-deny 0.20.2` four-category gate with a fresh RustSec refresh plus a negative advisory probe, the package-set gate, format, strict Clippy with `-D warnings`, the full workspace test suite, the real-tree architecture checker with its 23 refusal fixtures, the test-failure tripwire, the release build, the release CLI smoke, the injected-forbidden-import refusal, and two byte-identical release builds.

| Gate | ubuntu-24.04 | windows-2025 |
| --- | --- | --- |
| Toolchain assertion | pass | pass |
| Locked fetch | pass | pass |
| `cargo-deny` four categories | pass | pass |
| Advisory negative probe | pass (Ubuntu only) | n/a |
| Package-set gate | pass | pass |
| Format check | pass | pass |
| Strict Clippy, warnings are errors | pass | pass |
| Workspace tests, all targets | pass | pass |
| Architecture checker, real tree | pass | pass |
| Architecture refusal fixtures | pass | pass |
| Test-failure tripwire | pass | pass |
| Release build | pass | pass |
| Release CLI smoke | pass | pass |
| Forbidden-import refusal (Ubuntu only) | pass | n/a |
| Two release builds byte-identical (Ubuntu only) | pass | n/a |

Run `42569678884`, jobs `129003918555` (ubuntu-24.04) and `129003918886` (windows-2025), both `success`.

## 2. The four corrections this slice needed

Each was found by a real gate rather than by inspection, and each is recorded because an earlier record was wrong about it.

1. `f902bec` — applied the three rustfmt hunks supplied verbatim by the Architect. This cleared the format gate for the first time, which is what finally let the compiler see the slice.
2. `d1f2171` — both `observations_both` helpers declared two distinct anonymous input lifetimes while eliding the return lifetime, which rustc refuses as E0106. The lint exit code `101` was therefore a compilation failure, not a lint nit.
3. `1b75e2e` — the no-echo case omitted `contentVersion`, so the closed-root check correctly stopped at the missing member and reported that location instead of `blocks`, which is what the test asserted. The fixture now carries the required member; no assertion was relaxed and no diagnostic changed.
4. `4fa9b51` — the exact-size helper emitted malformed JSON. As a raw string, its head literal ended at the terminator immediately after the opening bracket, swallowing the quote that should open the filler string, while its tail literal still supplied a closing quote. The generated body had an unquoted identifier where a string element belonged, so the exactly-at-limit case could never parse. Both literals are now escaped ordinary strings: measured head 52 + filler 2097097 + tail 3 = exactly 2097152 bytes, one block, valid JSON. No limit was raised, no assertion weakened, and no production validation changed.

## 3. What the slice now contains

* `crates/solo-domain/src/source_selection.rs` (420 lines) — pure selection policy over caller-supplied observations. Read order is fixed and independent of declaration order: a declared `json` member is considered first, present JSON ends the decision, and only a genuinely reported `Absent` permits considering `markdown`. A named failure is never converted into absence.
* `crates/solo-domain/tests/source_selection.rs` — 14 tests covering read order, reported absence, all four failure kinds staying distinct, refusal of observations for undeclared members, redundant Markdown refusal, empty bytes versus absence, determinism under repetition, and diagnostics that never echo a body.
* `crates/solo-adapters/src/source_body_preflight.rs` (503 lines) — inert byte preflight of the selected body only: size envelope, strict UTF-8 without a byte-order mark, strict JSON with duplicate members refused at any depth by the accepted `DeclaredNode` visitor, and the closed body root (`contentVersion` 1.0.0, non-empty `title` to 140 scalars, optional `summary` to 500, `blocks` 1 to 250).
* `crates/solo-adapters/tests/source_body_preflight.rs` — 18 tests covering the closed root, scalar and block bounds, byte-level faults, syntax faults, duplicates at root and at depth, exact size boundaries for both body kinds, non-echoing refusals, owned bytes, and five in-memory pipeline cases.

## 4. Security and dependency evidence from this run

* Ubuntu: `cargo-deny 0.20.2` archive sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; RustSec `7a40350e1b8ba5b656d909f88222a8059a83d88d` fetched `2026-11-09T05:51:48Z`; advisories, bans, licenses and sources all ok on the unchanged locked graph.
* Windows: `cargo-deny 0.20.2` archive sha256 `975a22143262fd27476d19ee00c7af67978426e40e1dee94eed6bbade1cf87dc`; the same RustSec revision fetched `2026-11-09T05:52:30Z`; the same four categories ok.
* Negative probe: a disposable manifest resolving `time 0.3.46` was refused with `RUSTSEC-2026-0009` and exit 1, while the unchanged locked workspace graph passed all four checks.
* Injected forbidden import was refused with category `undeclared-crate` and exit 1, after which the tree was restored.

## 5. Corrections to earlier statements in this report

* Withdrawn: that no compiler had ever accepted this code, and that the slice was therefore unproven. Both hosts now compile it under strict Clippy with `-D warnings` and pass its tests.
* Withdrawn: my claim that `json_of_exact_size` emitted valid JSON, and the conclusion drawn from it that the 2 MiB boundary case had to pass. The helper emitted malformed JSON; the assertion was right and my reading of the raw string was wrong.
* Still withdrawn from the first revision: the local rustfmt heuristic, which fired 50 times on already-clean files and cannot certify formatter output.

## 6. Limitations that remain by design

Nothing here is publication authority. `OBSERVATION_IS_FILESYSTEM_PROOF`, `PROVISIONAL_CHOICE_IS_FINAL_SELECTION` and `CONTENT_VALIDITY_PROVEN` are all `false`, and both authority accessors on the selection and on the preflight result return `false`. No module opens, stats or lists anything; no absence is verified, only reported. Selection is provisional, not final effective-source selection. Markdown grammar (CommonMark, GFM), raw-HTML refusal, heading and anchor semantics, link and URL safety, `props`, `presentation` and module-registry validity are all outside this slice.

## 7. Repository invariants held

* `ROADMAP` byte-identical at `18975332c2820170a32558c4ee224176667b91ab`.
* `NEXT` neutral at `92fa562b1af0c54391851aa5c79deb1631127069`.
* `ACTIVE` still installed and issued at `f7eb973e5cb9744036594ad77c9c58842261a21c`; not reconciled and not neutralised.
* `OUTLOOK` unchanged at `cb9c7a610cf3c537557e060ca9656e44a053e69d`.
* Four WIP files, no subdirectory, clean tree, local `main` equal to `origin/main`.
* The `R01` and `R02` STOPs and the `R03` ACCEPT stand unwaived in history.

## 8. Transient acquisition failure on the first run at this revision

The first workflow run at this report revision, run `42570194179`, showed Windows green across all 31 steps and the Ubuntu leg failing at step 9 with these two annotations, preserved verbatim:

```
Process completed with exit code 1.
cargo-deny release-asset API fallback download failed (http 403)
```

This is a pinned-tool **acquisition** failure in the runner, not a repository, dependency or advisory finding. The identical gate, with the identical locked graph, had passed on both hosts six minutes earlier at `4fa9b51` (RustSec fetched `2026-11-09T05:51:48Z`, advisories, bans, licenses and sources all ok), and the failing step failed before `cargo-deny` ever ran, so no gate downstream of step 9 was evaluated on the Ubuntu leg of this revision. The Windows leg evaluated all of them and passed.

Re-running the failed job requires repository write access, which this agent does not hold: `POST /actions/runs/42570194179/rerun-failed-jobs` returns `401 Requires authentication`. A re-run of the failed job is the single remaining action, and it is safe to do because nothing in the repository changed between the fully green `4fa9b51` and this docs-only revision.

## 9. Request

Independent technical review of `4fa9b51` for `ACCEPT`. Reconciliation of `ACTIVE`, and any `ROADMAP` movement, is left to the Owner and Architect after that judgement. `R02.C` and Stage 02 remain open.

