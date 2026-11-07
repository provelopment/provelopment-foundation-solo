# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. Not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | `SOLO-WO-021` edition **`R06`** — final contact-contract disambiguation, documentation only |
| Owner activation | **Received** — separate Owner message: *"Execute `SOLO-WO-021 R06`: read and run `.documents/WIP/workorder-active.md`."* |
| Installed order | **18,053 bytes**, SHA-256 **`7B6CF80EC9A1956BD582D8A596822467A35E5C39FCCDF03BCBEE0A1835E17190`**, 102 lines, strict UTF-8, no BOM, LF-only, `OWNER-ISSUED` |
| Reviewed starting `main` | `358592862666ee42770a9c0e476fd227dd203393` |
| Issuance-freeze commit | **`a18e4248d653b4da2970bade7c61032689090509`** (ACTIVE only) |
| Correction commit | **`90ca85f339b0a1691e877a4de4f06e29aab82a18`**, tree `eea28147e457c45ed75b1366ba1cd4edc1b4b363` |
| Status | **`COMPLETE`** — G01–G10 all PASS |

## G01 — Owner authority and preflight

Root `C:/Users/Sean/Documents/Provelopment/00.foundation-solo` equals `git rev-parse --show-toplevel`; branch `main` tracking `origin/main`; exactly one worktree; sole `origin` = `provelopment/provelopment-foundation-solo`; no stashes. The worktree carried exactly ` M .documents/WIP/workorder-active.md` — the permitted Owner installation — and nothing else.

All eleven pinned anchors matched at `3585928` before any write: pre-install ACTIVE `1e0ce769`, `ARCHITECTURE.md` `aa3bdb5f`, `execution-report.md` `e6fac4f4`, `FOUNDATION-SOLO.md` `50a58123`, `sop-architect.md` `03962512`, `sop-implementation.md` `a5771160`, `plan-roadmap.md` `613d3fcb`, `plan-next.md` `92fa562b`, `plan-outlook.md` `cb9c7a61`, `README.md` `ed492cb6`, `AGENTS.md` `32562b67` — all ✅. `.documents/Security/` remains absent from HEAD and was not recreated. Exactly four WIP files.

## G02 — Issuance freeze before any source edit

The unaltered Owner-installed R06 was committed **first and alone**: `git add` scoped to `workorder-active.md`, producing `a18e424` containing exactly that one file. It was pushed, remote head confirmed at `a18e424`, and frozen blob `99ded895` extracted and re-hashed to **18,053 bytes / `7B6CF80EC9A1956BD582D8A596822467A35E5C39FCCDF03BCBEE0A1835E17190`** — byte-identical. The extraction artifact was deleted immediately (`.temp/` back to 0). Architecture editing began only afterwards.

## FINDING A — browser-enhancement contradiction resolved

The conflict was real and located at exactly one operative sentence: §6A.12's **"Each interactive module must supply (a) server golden DOM with a valid `data-solo-envelope` …"**, which sat beside the same section's express no-`BrowserEnhancementV1` rule for `interaction.contact`.

- **§6A.12 "Boundary assertions per module":** now reads **"Each of those five — `tabs`, `accordion`, `modal`, `carousel` and `motion` — and only those five"**, and adds an explicit clause that `interaction.contact` **is not** a browser-enhancement module: it is a server-rendered, standards-based HTML form that must carry no `data-solo-envelope`, export no `BrowserEnhancementV1` payload, and require no DTO parity fixture or client-runtime decoder, with generic closed-module validation, safe rendering and real keyboard/no-JS/refusal tests applying instead. Its `interaction.` prefix confers no browser-enhancement status, and no sixth `kind`, no special browser controller, no arbitrary authored script, no extra JSON node property and no alternative Page-enablement mechanism may be added.
- **§6A.5 "Nothing else crosses":** one added sentence records that the only payload-carrying modules are the five enumerated kinds, and that a server-rendered form such as `interaction.contact` is among the non-enhanced modules.

The five-kind enumeration at §6A.5 was **not** altered and remains exactly `tabs`, `accordion`, `modal`, `carousel`, `motion`.

## FINDING B — mandatory native transport and closed five-key envelope

§6B.3's conditional *"Where the entrypoint accepts JSON or `application/x-www-form-urlencoded`"* paragraph was replaced by two precise paragraphs:

- **Mandatory first-release transport.** A standard HTML `method="post"` `application/x-www-form-urlencoded` **UTF-8** submission, fully operable with JavaScript disabled; accepted with an **absent charset parameter or `charset=UTF-8`** matched case-insensitively. `multipart/form-data`, `application/json`, every other encoding, unsupported charsets and an unbounded parser are **refused in the first release**. The earlier conditional JSON reference is recorded as decoding guidance only — never a second endpoint, general JSON inquiry API or fallback — and later JSON support needs explicit separate Owner authority.
- **One closed five-key transport envelope.** Exactly three visitor keys (`name`, `email`, `message`) plus exactly two server-owned non-personal controls (`csrfToken`, `attemptToken`); decoding **separates the two controls out before** constructing the closed three-value `ValidatedInquiry`; controls are never authored props, visitor fields, visitor data or personal data. Unknown, duplicate and repeated keys, missing or malformed control values, ambiguous percent escapes, invalid UTF-8 and both **16 KiB** ceilings refuse the complete wire submission **including the control keys**. No honeypot, tracker, JSON fallback, attachment, custom action or new API.

§6B.8 gained one clarifying sentence: the two control keys are wire transport controls, never props, so their presence in a native POST neither extends the closed prop set nor counts as an authored field — resolving the misreading that the closed three-field payload rejects the CSRF/attempt controls.

## FINDING C — token expiry versus the permanent single-dispatch rule

§6B.4's "Attempt token and deduplication" paragraph was replaced by **"Attempt token, expiry and deduplication"**: the token expires **no later than 24 hours after issue** and can never become valid again; a missing, expired, invalid or unverifiable token is refused **without any outbound dispatch**; the non-content deduplication record is effective for the token's full validity and expires at that same defined boundary; after expiry or loss of deduplication state the same token creates **no fresh dispatch authority**; that refusal maps to the existing generic form-level `invalid_request` with no new outcome and no provider detail; a genuinely fresh attempt needs a deliberate new visitor action and a newly issued token under the existing duplicate-warning policy. At-most-once dispatch is retained and downstream exactly-once inbox delivery is still explicitly never promised.

§6B.11's policy table gained a **Wire transport** row, and its **Deduplication** row now records that the token expires within 24 h and never regains dispatch authority.

## Proof cases updated (all still `NOT RUN`)

| Case | Change |
| --- | --- |
| `F01` | now requires the closed **five-key** wire envelope and refusal of unknown, additional, duplicate and repeated keys — including a repeated or unknown **control** key — plus missing/malformed controls, wrong types, invalid UTF-8, ambiguous percent escapes and oversize bodies |
| `F03` | now requires the one obligatory media type (urlencoded, absent charset or case-insensitive `charset=UTF-8`) with `multipart/form-data`, `application/json`, other encodings, unsupported charsets and unbounded parsers refused; both 16 KiB ceilings applied to the complete wire submission including control keys |
| `F13` | now covers **expiry**: missing/expired/invalid/unverifiable tokens refused without dispatch and mapped to generic `invalid_request`; ≤24 h expiry, never revalidating, no fresh authority after expiry or state loss, alongside the existing multi-instance at-most-once and no-blind-resend requirements |
| `F16` | now requires a **real native POST with JavaScript disabled** that reaches the server and renders each truthful outcome, alongside the existing accessibility semantics |

`F02`, `F04`–`F12`, `F14`, `F15` and `F17` are unchanged. All **17** cases remain `NOT RUN`.

## Acceptance gates

| Gate | Result | Direct evidence |
| --- | --- | --- |
| G01 Owner authority | **PASS** | Exact R06 installed (18,053 B / `7B6CF80E…`), distinct matching activation, verified root, `main`, sole `origin`, one worktree, known initial state (ACTIVE only) |
| G02 Freeze | **PASS** | `a18e424` contains ACTIVE alone; pushed and remote-read-back **before** any Architecture edit; frozen blob `99ded895` re-hashed byte-identically |
| G03 Browser contract | **PASS** | Kinds still exactly five (`tabs`, `accordion`, `modal`, `carousel`, `motion`); §6A.12 now binds `data-solo-envelope`, DTO parity and decoder fixtures to **only those five** and states `interaction.contact` carries none while still requiring server-rendered, keyboard and no-JS form behaviour; §6A.5 clarifies payload ownership; no sixth `kind` |
| G04 Transport contract | **PASS** | One obligatory `application/x-www-form-urlencoded` UTF-8 native POST (absent charset or case-insensitive `charset=UTF-8`); five closed keys with exactly three visitor fields and two server-owned controls; duplicate/unknown/malformed/ambiguous-escape/invalid-UTF-8/oversize refusal; multipart and JSON refused; no form builder or default JSON |
| G05 Replay expiry | **PASS** | §6B.4 sets ≤24 h expiry, never revalidating, refusal without dispatch when missing/expired/invalid/unverifiable, no fresh authority after expiry or state loss, generic `invalid_request` mapping; §6B.11 dedup row and `F13` make the boundary verifiable |
| G06 Preservation | **PASS** | Registry still **exactly 22** IDs; lowest changed line is **1300** in a 3,000+ line file, so the §6.2E.2 registry at line 725 and all 21 preceding module rows are untouched; the `interaction.contact` registry row is unchanged; **seven** outcomes intact; field, rate, retention, analytics, port and site decisions intact; §5A at line 556 untouched |
| G07 Future proof cases | **PASS** | `F01`/`F03`/`F13`/`F16` updated for five-key decoding, no-JS POST, token expiry and multi-instance replay; **17 of 17** cases still `NOT RUN`; `C1`–`C4` remain `UNVERIFIED` |
| G08 No unrelated changes | **PASS** | `git diff --name-only` reports only `.documents/Architecture/ARCHITECTURE.md`; no security work, no PR/branch mutation, no provider, no `R00.G`, no `.github/**` change, no security register recreated |
| G09 WIP governance | **PASS** | Exactly four WIP files retained; ROADMAP still lists `R00.F`/`R00.G` (blob `613d3fcb` unchanged); NEXT neutral (`92fa562b` unchanged); ACTIVE still R06 (`99ded895`); no archive or extra document created |
| G10 Git/report hygiene | **PASS** | Exact commits recorded, diff **13 insertions / 10 deletions in 1 file**, single overwritten report, remote read-back performed, one worktree, clean tree, task temp cleaned to 0 |

**Overall: `COMPLETE`.** No gate is self-accepted; independent disposition belongs to the Chief Architect.

## No false runtime success

This repository still contains no application, schema artefact, test suite, CI job, contact endpoint, mail provider or deployment. **`F01`–`F17` are all `NOT RUN`; `C1`–`C4` remain `UNVERIFIED`.** No contact service, browser behaviour, security certification, delivery or deployment is claimed.

## Changed paths

| Path | Change | Commit |
| --- | --- | --- |
| `.documents/WIP/workorder-active.md` | Owner-installed R06 frozen verbatim | `a18e424` |
| `.documents/Architecture/ARCHITECTURE.md` | FINDING A/B/C resolutions and four proof-case updates | `90ca85f` |
| `.documents/Reports/execution-report.md` | This report | report commit |
| Product, both SOPs, ROADMAP, NEXT, OUTLOOK, `README.md`, `AGENTS.md`, `.github/**` | **Read-only — unchanged** (all anchor blobs identical) | — |
| `.documents/Security/security-issues.md` | **Remains absent**; not recreated | — |

## Consistency checks run

- Diff hunks confined to lines **1300, 1437, 1617–1619, 1635, 1683, 1703, 1705, 1715, 1718, 1731–1732** — every one inside the authorized §6A.5, §6A.12, §6B.3, §6B.4, §6B.8, §6B.10 and §6B.11 regions. Nothing outside them.
- Registry **22** rows; `interaction.contact` present exactly once in §6.2E.2 (line 725) and once in the §6A.12 matrix (line 1435); seven outcome rows; **17** proof cases all `NOT RUN`; §6B still **12** subsections.
- Stale wording eliminated: zero occurrences of *"Where the entrypoint accepts JSON"*, *"separate closed transport envelope"* or the ambiguous *"Each interactive module"*.
- Zero occurrences of `SOLO-SEC`, `WordPress` or `CVE-`; §5A and `.github/SECURITY.md` untouched — R06's security exclusion honoured.
- File is **strict valid UTF-8, no BOM, LF-only (0 CRLF), 0 mojibake**.

## Cleanup and remote read-back

Clean working tree; one worktree on `main`; `.documents/WIP/` exactly four files; Solo `.temp/` verified **0 items** (the single frozen-blob re-hash artifact was created and deleted); no task-owned material retained and no cleanup failure. `origin/main` verified at the correction commit and again at the report commit — see the chat handback for the exact SHA and tree.

## Handback

R00.F is **ready for independent closure review**, with all ten gates supported by source or Git evidence above. It is **not** closed: `R00.F` remains in ROADMAP, NEXT is untouched and neutral, ACTIVE still holds the frozen R06 order, and no stage was pruned, promoted, reassigned, started, merged, released or deployed. Independent `ACCEPT / REVISE / STOP` is required against this report and the exact diff. Only after that `ACCEPT` may the Owner separately authorize **one administrative closure** — removing `R00.F` from ROADMAP, clearing ACTIVE to its neutral title, and selecting `R00.G` in NEXT after a readiness assessment — which is a distinct order and **not** authorized by R06.


