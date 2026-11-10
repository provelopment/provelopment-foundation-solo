# Execution Report

**Order:** `SOLO-WO-054 R01` - implement R02.C: checked-in default one-Entity starter seed and expanded EFS fixture family.
**Disposition: `COMPLETE`** - for the technically completed R02.C scope only. Not a self-acceptance: `R02.C` remains in ROADMAP, ACTIVE remains issued, and the Chief Architect must independently review this revision.

## 1. Revision ledger

| Item | Value |
| --- | --- |
| Starting `HEAD` / `origin/main` | `b5139bca02acc49a5200e591145e6fec4ea244ae` (tree `74363d0f6e6e868bc125e42319c3ac5db4810d68`) |
| Issuance commit (ACTIVE + cleared NEXT) | `7cca97f5c41fe5da9de54e2c5fb192b7ed31feb2` (tree `9a45572a64ebeff45cdde8036f885c0e73879e5b`) |
| Task branch `solo-wo-054-r01` final | `5596a2b1d7b0c70b288265cbcaed73a6f6c96c85` |
| Substantive `main` (fast-forward, verified) | `5596a2b1d7b0c70b288265cbcaed73a6f6c96c85` (tree `bf58b7d80310688bfbf1478e949205ee600fd5ae`) |
| Accepted R02.B substantive ancestor | `1f4de7f439e81ea9af36b655688627c598f281f2` (unchanged, untouched) |
| Remote | `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, sole remote |

Integration was `git merge --ff-only` from the verified task branch; no PR, squash, rebase, force-push, reset or history rewrite was used.

## 2. WIP ledger

| File | Before | After |
| --- | --- | --- |
| `workorder-active.md` | `ea5f20b736483c2e102456be1cea9104059fc81a` (neutral) | `875faf68ef27b4b221769df8db370c1af9d4f053` (the issued order, frozen) |
| `plan-next.md` | `ba110885d1d511240b4c31cf3a9566ef7afeec05` (R02.C candidate) | `92fa562b1af0c54391851aa5c79deb1631127069` (original title-only slot) |
| `plan-roadmap.md` | `e7be6b4a78f0da0f6f4f09e92b2eb31c2409f437` | `e7be6b4a...` **unchanged** - R02.C is still pending |
| `plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` | `cb9c7a61...` **unchanged** |
| `execution-report.md` | `a44ab968dcce2d83184c8a66f128de0ba55d29d3` | overwritten by this single report-only commit |

Exactly four WIP files exist. ACTIVE still holds the complete `OWNER-ISSUED SOLO-WO-054 R01` order: it was frozen after issuance and never cleared.

## 3. Delivered content (real committed bytes)

**Starter seed** - `examples/starter/content/publication.json` (1156 B) and `examples/starter/content/pages/main-home.en-001.md` (725 B): one Entity `example`, Facet `main`, Site `main` (`defaultLocale` `en-001`, `languagePreferences` `{"en": "en-001"}`), Page `home`, one `en-001` representation at `/en-001/home`, with `base.canonical` `https://example.org/site` and a fully qualified `base.landing`. Both a `json` and a `markdown` candidate are declared; the JSON file is intentionally absent, so the accepted JSON-first rule selects the real Markdown body. No installer, command, asset or graphic reference exists.

**Expanded synthetic fixture** - `crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json` (3567 B) plus six selected bodies:

| Facet / Site (default) | Page | Representation | Selected body (bytes) |
| --- | --- | --- | --- |
| `visitors` / `bali` (`en-001`) | `home` | `en-001` `/en-001/home` | `bali-home.en-001.md` (314) |
| `visitors` / `bali` (`en-001`) | `home` | `id-ID` `/id-id/Hidden7A`, `visibility: unlisted` | `bali-home.id-id.md` (325) |
| `visitors` / `bali` (`en-001`) | `about` | `en-001` `/en-001/about` | `bali-about.en-001.md` (347) |
| `visitors` / `bali` (`en-001`) | `about` | `id-ID` `/id-id/TentangKami`, `caseMatching: sensitive` | `bali-about.id-id.json` (77) |
| `visitors` / `jakarta` (`en-001`) | `home` | `en-001` `/en-001/jakarta` | `jakarta-home.en-001.md` (323) |
| `company` / `global` (`de-DE`) | `home` | `de-DE` `/de-de/unternehmen` | `company-home.de-de.md` (317) |

Two Facets, three Sites, four Page ownership nodes, six representations; Page ID `home` is correctly scoped and reused under three Sites. The single JSON body uses the accepted envelope (`contentVersion` `1.0.0`, non-empty `title`, `blocks: [{}]`). The Entity carries one inert `properties.location` declaration. `visibility: unlisted` and `caseMatching: sensitive` are retained as inert declarations only - no route matching, discovery, visibility or fallback behaviour is implemented or claimed.

**Test** - `crates/solo-adapters/tests/r02c_seed.rs` (1199 lines, 16 named cases) drives the committed bytes through `parse_manifest_bytes`, `EfsOwnershipDraft`/`EfsOwnershipGraph` and `assemble_declared_source_set_v1` with exclusively claimed scratch roots (`content` + `staging`). No existing source, manifest, domain, candidate, staging or workflow file was modified.

## 4. Acceptance matrix - measured outcomes

All 16 cases executed on both hosts (`running 16 tests`, listed cases `ok`, enclosing step `success`).

- R02C-01: starter parses with 5 nodes / 1 page / 1 representation, single full locale, landing and base exact; staging yields exactly one `Markdown` member at `pages/main-home.en-001.md`, byte-identical, correct fingerprint and lineage, and no publication or snapshot authority.
- R02C-02: a valid JSON added at the declared preferred path wins with identical lineage; a present-but-invalid JSON refuses the whole candidate as a selected-body refusal with no Markdown fallback.
- R02C-03: expanded manifest parses to 16 nodes / 4 pages / 6 representations in authored order; all six bodies stage byte-identically; exactly one is `Json`.
- R02C-04: the pure graph resolves all four Pages and all six representations; wrong Entity, Facet, Site, Page and locale each raise the registered lineage refusal (`PREFLIGHT_EFS_LINEAGE_UNRESOLVED`), with no sibling substitution.
- R02C-05: refused as registered - root `entities` collection (`UnknownKey`), two-Entity `entity` array (`NotAnObject`), unauthorized `parent` hint (`UnknownKey`), duplicate Facet/Site/Page (`DuplicateScopedId`), duplicate locale (`DuplicateLocale`), empty facets/sites/pages (`EmptyCollection`). A bare `en`/`id`/`de` key is refused as `NotFullLocale`.
- R02C-06: an undeclared body is neither selected nor staged (still one staging object); a duplicate logical claim across representations is refused before staging as `DuplicateSourceClaim` / `PREFLIGHT_MANIFEST_INVALID`.
- R02C-07: one missing selected body refuses the whole set as `SOURCE_NONE_EFFECTIVE` naming the last declaration, with no partial candidate.
- R02C-08: changing labels, base origin/prefix and authored routes leaves every Page target resolvable and membership unchanged; membership differs only where authored differently (2 vs 1 keys on the two `home` Pages).
- R02C-09: declared order preserved; `retained_body_bytes() == 0`, one staging object, staged total equals the sum of the committed bodies, peak live bytes inside twice the single-body bound.
- R02C-10: the suite re-reads each committed fixture from disk and asserts it equals the bytes it used; scratch trees are removed on drop and never overlap a fixture.

## 5. CI evidence

| Run | Revision | Result |
| --- | --- | --- |
| `42823587570` | `7cca97f5` (issuance on `main`) | Ubuntu 24.04 success, Windows 2025 success |
| `42824870948` | `cc339342` (first branch attempt) | **FAILED on both hosts** - see disclosure |
| `42825967792` | `5596a2b1` (task-branch HEAD) | Ubuntu 24.04 success, Windows 2025 success, 0 failed steps |
| `42826688112` | `5596a2b1` (substantive `main`) | Ubuntu 24.04 success, Windows 2025 success, 0 failed steps |

Both jobs report 33 steps; every `skipped` step is the other host's counterpart (e.g. the Windows job skips the Linux deny, architecture, tripwire, CLI-smoke and summary steps, and vice versa), so no mandatory case is skipped on both hosts. On Ubuntu the disposable Btrfs fixture was created (`btrfs-progs v6.6.3`, `fstype=btrfs`), exported as `SOLO_TEST_SOURCE_FS_ROOT`, used by the snapshot tests and removed. The deliberate test-failure tripwire step succeeded as designed. Locked graph, `cargo-deny` all classes with a refreshed RustSec set, the pinned `1.99.1` toolchain assertion, format, strict Clippy, the architecture checker with its refusal self-test, the release CLI smoke and the byte-reproducible Ubuntu build all passed.

**Disclosure - first branch run failed.** `42824870948` failed the test step on both hosts for two reasons, both corrected and re-verified: (a) the parser records an unauthored route case policy as `None` rather than defaulting it, so three expectations in the ordering assertion were wrong; (b) the test originally read fixtures with `include_bytes!`/`include_str!`, which the accepted architecture gate refuses in the real tree as source-loading macros. The fix uses the order's third permitted form - fixed known paths under `CARGO_MANIFEST_DIR` - and the checker, workflow and every other frozen file were left unchanged.

## 6. Changed paths (complete authorized list)

`examples/starter/content/publication.json`, `examples/starter/content/pages/main-home.en-001.md`, `crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json`, its six `content/pages/*` bodies, `crates/solo-adapters/tests/r02c_seed.rs`, plus the already-committed issuance paths (`.documents/WIP/workorder-active.md`, `.documents/WIP/plan-next.md`) and this report. Paths 14-16 of the ledger were **not** used: no parser, candidate or domain defect was demonstrated. No dependency, manifest, lockfile, workflow, checker, SOP, About, Architecture, README, AGENTS or tooling file changed. `R02.C` remains in ROADMAP; Stage 02 and its exit gate are untouched.

## 7. Git state

`git status --porcelain` is empty; one checkout, one worktree, sole `origin`; local `main` and `origin/main` are the substantive SHA above. Branch `solo-wo-054-r01` is deliberately **retained** (its post-acceptance pruning needs a later separate authorization). Task-owned scratch directories were removed, and the one local build directory created by a link-stage attempt was deleted; no Owner material was touched.

## 8. True limits

- Local compilation and local test execution were **not** possible on this workstation: the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain is installed but no MSVC linker is present, so `cargo` cannot link build scripts. Every execution result above is CI evidence at the exact revisions listed; nothing is inferred from a local run.
- R02.C is **not** accepted, not removed from ROADMAP, and ACTIVE is not cleared. No R10.A installer, publication, HTTP/browser surface, route engine, locale validator, asset pipeline, Stage 03/04 work, release or deployment is present or claimed.
- The Entity location declaration and the two route/visibility declarations are inert authored data only; no inheritance, discovery, exact-case enforcement or fallback behaviour follows from them.

**Disposition: `COMPLETE`** (R02.C implementation scope) - awaiting independent Architect review.