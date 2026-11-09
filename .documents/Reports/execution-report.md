# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place each run; Git history retains earlier revisions. It is neither a WIP file nor a source of authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date (UTC) | 2026-11-09 |
| Work order | **`SOLO-WO-046 R01`** — R02.B second bounded slice: provisional source selection and inert body-byte preflight |
| Matching Owner handoff | delivered verbatim in chat |
| Required starting HEAD / tree | `8f81078ac42d26a882277a5e4bb954bba0303a07` / `ae83cf9c8d737c97cdc6f820bfe76291a361782f` — **matched exactly** |
| §2 baseline blob ledger | **24 of 24 blobs verified, zero drift**; all four required-absent paths confirmed absent |
| Current HEAD / tree | `57c905a86c525fef521d86b00be1d7c23ac4f609` / `1ba06d1a199e885473436d1a3860dfefce9d8ea6` |
| **Status** | **`STOP`** — Architect returned `REVISE` at `6a2b794`; the three rustfmt hunks were not received and cannot be retrieved here, so **no code change was made this turn** |

## Outcome

The order's preflight, issuance, scope and refusal design were completed exactly as specified, and the bounded code was written to the §1 ledger and published in seven reviewable commits. **It has never been compiled.** The mandatory two-host workflow has failed four times on **step 14, "Format check"** — the first gate after the security and package-set gates, and before every compile, lint, test, architecture, release and reproducibility gate. Those gates therefore **did not run at all** on any revision this run.

I could not observe *why* formatting is rejected, so I am stopping rather than continuing to publish unverifiable corrections. This report records that honestly and asks for the smallest Owner decision.

## Chief Architect disposition — `REVISE`, and what this turn could not obtain

The Architect reviewed at `6a2b794be0486de6e48815c3115734fbd27887b5` and reported retrieving the raw Ubuntu and Windows logs for run **`42565562908`** (jobs `128991196631` Ubuntu, `128991196456` Windows, both `failure`), which establish **exactly three outstanding rustfmt differences**, stated to be reproduced in the review. **Those three differences did not reach this agent**, and I did not invent them.

This turn changed no file. Every route to the formatter output was tried again and refused:

| Route | Result |
| --- | --- |
| `GET /repos/…/actions/runs/42565562908` | 200 — confirms `head_sha = 6a2b794…`, `conclusion = failure` |
| `GET /repos/…/actions/runs/42565562908/jobs` | 200 — both job ids and names only |
| `GET /repos/…/actions/jobs/{id}/logs` | **403**, no `Location` |
| `GET /repos/…/actions/runs/{id}/logs` | **403** |
| Same, without the JSON `Accept` header | **403** |
| `GET /repos/…/check-runs/{id}/annotations` | only "Process completed with exit code 1" |
| `GET /commits/{sha}/check-runs` → `output.summary` / `output.text` | **empty** |

The format step itself is confirmed to be exactly `cargo fmt --all -- --check` with no configuration file and no extra flags, so nothing about the workflow narrows the answer.

### I must also retract an overstatement in the previous revision of this report

That revision said "every mechanical check I can perform locally is clean", offered as if it validated the formatting. **It does not, and I withdraw it.** Calibrating the same checks against the 26 Rust files in this repository that are already rustfmt-clean and CI-green:

* they report **19 lines longer than 100 columns** in clean files, and
* they report **50 "joinable" multi-line calls** in clean files that rustfmt deliberately left broken.

A concrete demonstration of the gap, in `solo-domain/src/source_locator.rs`:

```rust
            return Err(SourceLocatorError::at(SourceLocatorErrorKind::Empty, None));   // line 260: 82 columns, ONE line
            return Err(SourceLocatorError::at(                                           // line 265: 86 columns, BROKEN
                SourceLocatorErrorKind::TooLong,
                None,
            ));
```

Across the corpus, 35 such nested `return Err(…)` statements are single-line and 69 are broken, and I cannot derive the rule that separates them. My local heuristics therefore cannot certify rustfmt output, and — because the real diff is unobservable here — **I will not apply three guessed edits and label them as the Architect's.** The order requires preserving actual evidence and forbids guessing or bypassing the gate; producing plausible-looking but unverified changes would be worse than the present STOP, since it would manufacture the appearance of compliance.

**What is needed to finish in one pass:** the three rustfmt hunks verbatim (file, range, before/after), or log read access for `42565562908`, or the pinned `rustfmt 1.99.1` in this environment. With any one of those I can apply exactly the intended edits and then drive every mandatory gate to a real result.


| Commit | Content |
| --- | --- |
| `0eac119` | ACTIVE recorded byte-identically (issuance history), blob `f7eb973e` |
| `e401ff6` | NEXT cleared to the exact neutral 49-byte heading, blob `92fa562b` |
| `824a437` | Domain: pure provisional JSON-first selection policy + tests + `lib.rs` wiring |
| `8b56f47` | Adapters: inert body preflight, closed body root, in-memory pipeline + tests + `lib.rs` wiring |
| `c37c51b` | Style: first rustfmt-clean restructure (short refusal/observation helpers, one statement per line) |
| `dfd3f6d` | Style: recomputed import packing and file endings |
| `57c905a` | Style: call arguments brought inside the rustfmt width limits |

## CI ledger — five red runs, all at the same step

| Revision | Ubuntu job | Windows job | Failing step |
| --- | --- | --- | --- |
| `8b56f47` | `128981825840` | `128981826036` | 14 Format check |
| `c37c51b` | `128984463594` | `128984463409` | 14 Format check |
| `dfd3f6d` | `128986426061` | `128986425951` | 14 Format check |
| `57c905a` | `128988195189` | `128988194976` | 14 Format check |
| `57dc490` (this report) | `128989735856` | `128989736036` | 14 Format check |

On every attempt the gates **before** formatting passed on both hosts: locked fetch, pinned-toolchain assertion with exact `rustc`/`cargo` versions, `cargo-deny 0.20.2` advisories/bans/licenses/sources on the unchanged 15-package locked graph with a fresh RustSec database, the Ubuntu `RUSTSEC-2026-0009` known-vulnerable negative probe correctly **refused**, and the workspace package-set gate. Steps 15 onward — strict lint, tests, real-tree architecture checker, refusal self-test, tripwire, release build, CLI smoke, forbidden-import refusal and byte-reproducibility — are **skipped**, not passed.

The last row is this report itself: a documentation-only revision fails at the **same** step, which confirms the rejection follows the four new Rust files and not the WIP or report edits. Consequently the order's requirement of two-host green CI **at the report HEAD** is **not met**, and a final documentation-only amendment after this one could not be verified either — that limitation is disclosed rather than papered over.

## Why this stopped

* **No Rust toolchain exists in this environment.** `cargo`, `rustc` and `rustup` are all absent from `PATH`, and `~/.cargo`, `~/.rustup`, `C:\Program Files\Rust`, `C:\Rust` and `%LOCALAPPDATA%\Programs\Rust` do not exist. I therefore cannot run `cargo fmt`, or compile, or test, locally.
* **The workflow log body is not readable.** `GET /actions/jobs/{id}/logs` returns **HTTP 403 with no redirect**, so the token cannot reach the archive; the check-run annotations expose only "Process completed with exit code 1" for that step. The `cargo fmt` diff — the one artifact that would name the offending lines — is unobservable.
* **Installing a toolchain is not permitted here** (`no global installer`, `unapproved network/component`), and the workflow is read-only for this order, so there is no sanctioned way to surface the diff.

I attempted four corrections, each modelling a different rustfmt rule, and validated my model by reproducing untouched, rustfmt-clean blocks in this same repository byte-for-byte:

1. `c37c51b` — replaced discretionary layouts with short helpers and one statement per line.
2. `dfd3f6d` — recomputed wrapped import blocks with the fill rule rustfmt actually uses (validated against `publication_manifest.rs` and `source_locator.rs`, which it reproduces exactly), and removed stray trailing blank lines.
3. `57c905a` — shortened fixtures and hoisted long `format!` bodies so no single-line argument list exceeds the 60-character call budget.

After each attempt every mechanical check I can perform locally is clean: no line exceeds 100 columns, no wrapped call or macro could fit on one line, no single-line call exceeds the 60-character argument budget except one `fn` signature where that rule does not apply, no macro block carries a trailing comma, no trailing whitespace, exactly one final newline, LF endings, and balanced delimiters. The gate still fails, so at least one rustfmt behaviour I cannot observe differs from my model. Further blind guessing would only add unverifiable churn to `main`.

**No workaround was attempted outside §1:** I did not edit the workflow, the checker, a manifest, the lockfile, or any unlisted file, and I installed nothing.

## What was written, and its exact boundaries

Four new files and two narrowly edited `lib.rs` files, all within the §1 ledger:

* **`solo-domain/src/source_selection.rs`** — the pure policy. `select_provisional_source` consumes the accepted `SourceCandidateSetV1` and typed observations (`Absent`, `Present(bytes)`, `Failure(kind)` with permission / other-I/O / unsafe-location / changed-during-read kept distinct). Declared JSON is considered first; present JSON is chosen without the Markdown body being requested; only a reported JSON `Absence` permits declared Markdown; a JSON `Failure` is an immediate refusal; an observation for an undeclared member, a missing observation, or Markdown bytes supplied alongside present JSON are refused. No I/O, no dependency, no `unsafe`, no clock, locale or directory state.
* **`solo-adapters/src/source_body_preflight.rs`** — the inert preflight. Bounds the envelope *before* copying, refuses BOM, invalid UTF-8, trailing input, comments, non-finite literals and duplicate members at any depth by reusing the accepted duplicate-aware `DeclaredNode` visitor, then enforces the closed body root (§6.2E.1). Returns an owned immutable copy; `CONTENT_VALIDITY_PROVEN` and both authority accessors are `false`. `run_in_memory_source_pipeline` is the single documented trust boundary and performs no filesystem access anywhere, including in tests.
* **`solo-domain/tests/source_selection.rs`**, **`solo-adapters/tests/source_body_preflight.rs`** — public-API matrix covering the precedence rules, absence-versus-error, no-fallback, the closed root, scalar and size boundaries at exactly 2 MiB and 1 MiB, caller-buffer mutation after preflight, and non-echoing diagnostics.

**What this slice explicitly does not do:** no file is opened, no absence is confirmed, no confinement, no-follow identity, alias/hardlink/case equivalence or filesystem snapshot exists, no module/`props`/`presentation` schema is validated, no Markdown grammar is parsed, and nothing is publishable. §6.2C.3–§6.2C.4 remain unproven by design.

## Evidence quality — what is not proven

Recorded plainly, because a green badge was never obtained:

* **`NOT RUN` — compilation, lint, tests, architecture checker, refusal self-test, tripwire, release build, CLI smoke, byte-reproducibility.** All skipped behind the format gate. **No claim is made that any test passes**, and no test result can be inferred from the code being present.
* **`PASSED` on every attempt** — locked fetch, pinned-toolchain assertion, `cargo-deny` advisories/bans/licenses/sources with a fresh RustSec refresh and recorded digest, the Ubuntu known-vulnerable advisory negative probe refusing `RUSTSEC-2026-0009`, and the workspace package-set gate.
* **`UNVERIFIED`** — raw job logs (HTTP 403, no redirect available), GitHub alert APIs (401), so alert counts are unknown.
* **`NOT OBSERVED`** — a scheduled Dependabot scan.
* **`NOT APPLICABLE`** — browser, HTTP, rendering, release and deployment evidence; none exists for this slice.
* `serde`/`serde_json` remain shipped adapter normal dependencies; the locked graph is unchanged at 15 packages with no new component. The `unicode-ident` Unicode-3.0 attribution/NOTICE obligation remains an open future-release prerequisite. Ubuntu `rustup 1.29.2` versus Windows `1.29.3` remains a tracked low-urgency parity proposal, not an authorised upgrade.

## State ledger and cleanup

`.documents/WIP/` holds exactly four files with no subdirectory:

| File | Blob | State |
| --- | --- | --- |
| `workorder-active.md` | `f7eb973e5cb9744036594ad77c9c58842261a21c` | **order installed and preserved** — not neutralised, because no `ACCEPT` occurred |
| `plan-next.md` | `92fa562b1af0c54391851aa5c79bdeb1631127069` | neutral, exactly as this order required after issuance |
| `plan-roadmap.md` | `18975332c2820170a32558c4ee224176667b91ab` | **byte-identical to the baseline**; `R02.B`, `R02.C` and Stages 02–13 intact |
| `plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` | byte-identical |

Exactly eight paths differ from the baseline, all within §1: the two WIP files above, the four new Rust files, and the two named `lib.rs` files. Every other baseline blob is unchanged, including `ARCHITECTURE.md`, both SOPs, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, the workflow, the graph checker and every previously accepted domain/adapter source file.

**Cleanup.** No toolchain was installed, so no build artifacts and no `target/` exist. I created no scratch files in the repository and no `.temp/` content; the only temporaries were commit-message texts in the OS temp directory, each deleted immediately after use. The tree is clean, on one worktree with zero stashes, and `main` equals `origin/main`. No PR, branch, amend, rebase, reset, force-push, tag, release, workflow dispatch/rerun/cancel, settings change or other-repository write occurred.

## Preserved history — not erased, not reinterpreted

* **`R01` STOP** (issuance omitted `workspace_graph.rs` from its ledger) and **`R02` STOP** (read-only workflow carried a stale edge count) remain in Git history, unwaived.
* **`R03` was independently `ACCEPT`ed and closed**: the stale `edges=22` constant was repaired to `edges=24` in exactly two literals, both hosts went fully green, ACTIVE was neutralised to its 40-byte blob, and the report was published. That closure stands and is unaffected by this run.
* **`R02.B`, `R02.C` and Stage 02 remain open.** Nothing here closes them, and the provisional-selection policy is not final effective-source selection.

## The smallest decision I need

`main` remains **red** at the implementation revision: the format gate rejects changes to the four new Rust files whose exact deviation I cannot observe, and my local model of rustfmt is now proven unsound. I will not self-authorise a fix, a revert, or a workaround. One of these is needed:

1. **Send the three rustfmt hunks verbatim** — file, line range, and before/after for each. This is the smallest possible input and is sufficient: I can apply exactly those edits and then run every gate to a real result.
2. **Enable read access to the workflow log body** for run `42565562908`, so the formatter output is directly observable rather than inferred.
3. **Provide the pinned toolchain** (`rustc`/`cargo`/`rustfmt` `1.99.1`) in this environment, so the exact formatter can be run before pushing.
4. **Authorise a revert** of the seven commits to restore the last green baseline (`8f81078`), and re-issue the slice when (1), (2) or (3) is available.

Until one of these arrives, the slice is **unproven and unjudged on its merits** — the failure is confined to formatting, no functional judgement of the policy or the preflight has been made by anyone, and `R02.B`, `R02.C` and Stage 02 remain open.

**`agent report ready`**
