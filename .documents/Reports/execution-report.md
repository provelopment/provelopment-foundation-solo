# Execution Report

**Order:** `SOLO-WO-054 R01` - implement R02.C: checked-in default one-Entity starter seed and expanded EFS fixture family.
**Revision:** second publication, incorporating the Architect disposition **REVISE - one bounded acceptance-evidence gap** (R02C-08 positive proof). Supersedes the report published at `72e67c007e9434b6ab5313bb28b345932eaa50b9`.
**Disposition: `COMPLETE`** - for the technically completed R02.C scope only. Not a self-acceptance: `R02.C` remains in ROADMAP, ACTIVE remains issued, and the Chief Architect must independently review this revision.

## 1. Revision ledger

| Item | Value |
| --- | --- |
| Starting `HEAD` / `origin/main` for the first publication | `b5139bca02acc49a5200e591145e6fec4ea244ae` (tree `74363d0f6e6e868bc125e42319c3ac5db4810d68`) |
| Issuance commit (ACTIVE + cleared NEXT) | `7cca97f5c41fe5da9de54e2c5fb192b7ed31feb2` (tree `9a45572a64ebeff45cdde8036f885c0e73879e5b`) |
| First substantive revision | `5596a2b1d7b0c70b288265cbcaed73a6f6c96c85` (tree `bf58b7d80310688bfbf1478e949205ee600fd5ae`) |
| First report publication | `72e67c007e9434b6ab5313bb28b345932eaa50b9` (report blob `0e981a92f9de36f2052ec828e996671207d33175`) |
| Task branch before the correction | advanced onto `72e67c00` by `git merge --ff-only main`, so the published report commit is an ancestor and was never rewritten |
| Corrected substantive `main` / branch `HEAD` | `c57a45e6d010a75668c29ff4158909975ed04ef4` (tree `98ff14fa38d67949a7e6b8977afaff4c885dc286`) |
| Accepted R02.B substantive ancestor | `1f4de7f439e81ea9af36b655688627c598f281f2` (unchanged, untouched) |
| Remote | `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, sole remote |

Integration was fast-forward only in both directions: the task branch was advanced onto the published `main`, the correction was added there, and `main` was then fast-forwarded to the corrected branch. No PR, squash, rebase, reset, force-push or history rewrite was used.

## 2. WIP ledger (unchanged by this revision)

| File | Blob |
| --- | --- |
| `workorder-active.md` | `875faf68ef27b4b221769df8db370c1af9d4f053` - the complete `OWNER-ISSUED SOLO-WO-054 R01`, still installed and frozen |
| `plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` - title-only slot |
| `plan-roadmap.md` | `e7be6b4a78f0da0f6f4f09e92b2eb31c2409f437` - **R02.C still present** |
| `plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `execution-report.md` | previously `0e981a92f9de36f2052ec828e996671207d33175`; overwritten by this single report-only commit |

Exactly four WIP files exist; ACTIVE was not cleared and ROADMAP was not advanced.

## 3. The correction made under REVISE

Only `crates/solo-adapters/tests/r02c_seed.rs` changed relative to the first substantive revision (`git diff --name-only 72e67c00 c57a45e6` reports exactly that one path). The committed starter seed, the expanded fixture, the accepted R02.B implementation, the workflow and every other frozen file are byte-identical. No runtime architecture, public API or dependency changed; the addition is a test-only manifest transformation plus one new named test, taking the file from 16 to 17 cases.

**New case - `removing_one_authored_representation_changes_membership_without_re_identifying_the_page`.** It uses a test-owned copy of the committed expanded manifest, in which the helper `without_bali_home_unlisted` locates the Bali Home `id-ID` representation object, bracket-matches it and removes that object together with the single separator that joined it to its array. Nothing is invented, no other declaration is touched, and the fixture on disk is never modified.

Measured outcomes of the six required steps, on both hosts:

1. The committed Page `example / visitors / bali / home` resolves with membership exactly `["en-001", "id-ID"]`, and the manifest declares six representations.
2. After removing only that representation, the copy still authors `/id-id/TentangKami` (Bali About, case-sensitive) and no longer contains `/id-id/Hidden7A`; it reparses to five representations and four Pages.
3. The ownership graph rebuilt from that copy still resolves the same complete Page target `example / visitors / bali / home`, with the same authored Entity, Facet, Site and Page identifiers - the Page was not re-identified.
4. Its membership is now exactly `["en-001"]`, while the other three Page targets (`visitors/bali/about`, `visitors/jakarta/home`, `company/global/home`) each keep exactly the membership they had before.
5. `assemble_declared_source_set_v1` runs over the test-owned scratch tree holding all six committed bodies and selects exactly five, in authored order, each byte-identical to its committed file with the expected member kind and locale; `pages/bali-home.id-id.md` is never selected even though the test first asserts that file still exists on disk, so absence cannot explain the result.
6. The baseline parse, the reduced parse, the assembled candidate and every member report no publication authority and no verified filesystem snapshot.

CI evidence for this case: the `r02c_seed` binary reported `running 17 tests` with `test removing_one_authored_representation_changes_membership_without_re_identifying_the_page ... ok` on Ubuntu 24.04 and on Windows 2025.

## 4. Delivered content (unchanged by this revision)

**Starter seed** - `examples/starter/content/publication.json` (1156 B) and `examples/starter/content/pages/main-home.en-001.md` (725 B): one Entity `example`, Facet `main`, Site `main` with `defaultLocale` `en-001` and explicit `languagePreferences`, Page `home`, one `en-001` representation at `/en-001/home`, `base.canonical` `https://example.org/site` and a fully qualified `base.landing`. Both a `json` and a `markdown` candidate are declared and the JSON file is intentionally absent, so the accepted JSON-first rule selects the real Markdown body. No installer, command, asset or graphic reference exists.

**Expanded synthetic fixture** - `crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json` (3567 B) plus six selected bodies:

| Facet / Site (default) | Page | Representation | Selected body (bytes) |
| --- | --- | --- | --- |
| `visitors` / `bali` (`en-001`) | `home` | `en-001` `/en-001/home` | `bali-home.en-001.md` (314) |
| `visitors` / `bali` (`en-001`) | `home` | `id-ID` `/id-id/Hidden7A`, `visibility: unlisted` | `bali-home.id-id.md` (325) |
| `visitors` / `bali` (`en-001`) | `about` | `en-001` `/en-001/about` | `bali-about.en-001.md` (347) |
| `visitors` / `bali` (`en-001`) | `about` | `id-ID` `/id-id/TentangKami`, `caseMatching: sensitive` | `bali-about.id-id.json` (77) |
| `visitors` / `jakarta` (`en-001`) | `home` | `en-001` `/en-001/jakarta` | `jakarta-home.en-001.md` (323) |
| `company` / `global` (`de-DE`) | `home` | `de-DE` `/de-de/unternehmen` | `company-home.de-de.md` (317) |

Two Facets, three Sites, four Page ownership nodes, six representations; Page ID `home` is correctly scoped and reused under three Sites. The single JSON body uses the accepted envelope (`contentVersion` `1.0.0`, non-empty `title`, `blocks: [{}]`). The Entity carries one inert `properties.location` declaration. The `unlisted` and `sensitive` declarations remain inert: no route matching, discovery, visibility or fallback behaviour is implemented or claimed.

**Test** - `crates/solo-adapters/tests/r02c_seed.rs` (1401 lines, **17** named cases) drives the committed bytes through `parse_manifest_bytes`, `EfsOwnershipDraft`/`EfsOwnershipGraph` and `assemble_declared_source_set_v1` over exclusively claimed scratch roots (`content` + `staging`), reading fixtures from fixed paths under `CARGO_MANIFEST_DIR`. No source-loading include macro is used anywhere.

## 5. Acceptance matrix - measured outcomes

- R02C-01: starter parses to 5 nodes / 1 page / 1 representation with the single full locale, exact landing and base; staging yields exactly one `Markdown` member, byte-identical, correct fingerprint and lineage, with no publication or snapshot authority.
- R02C-02: a valid JSON added at the declared preferred path wins with identical lineage; a present-but-invalid JSON refuses the whole candidate as a selected-body refusal with no Markdown fallback.
- R02C-03: expanded manifest parses to 16 nodes / 4 pages / 6 representations in authored order; all six bodies stage byte-identically; exactly one is `Json`.
- R02C-04: the pure graph resolves all four Pages and all six representations; wrong Entity, Facet, Site, Page and locale each raise `PREFLIGHT_EFS_LINEAGE_UNRESOLVED`, with no sibling substitution.
- R02C-05: refused as registered - root `entities` collection (`UnknownKey`), two-Entity `entity` array (`NotAnObject`), unauthorized `parent` hint (`UnknownKey`), duplicate Facet/Site/Page (`DuplicateScopedId`), duplicate locale (`DuplicateLocale`), empty facets/sites/pages (`EmptyCollection`); a bare `en`/`id`/`de` key is `NotFullLocale`.
- R02C-06: an undeclared body is neither selected nor staged (still one staging object); a duplicate logical claim across representations is refused before staging as `DuplicateSourceClaim` / `PREFLIGHT_MANIFEST_INVALID`.
- R02C-07: one missing selected body refuses the whole set as `SOURCE_NONE_EFFECTIVE` naming the last declaration, with no partial candidate.
- R02C-08: **positive proof added under REVISE** - removing one authored representation keeps the Page identity and changes only its membership, while the other Page identities and memberships are unchanged and the real source stage selects exactly the five remaining declarations (section 3).
- R02C-09: declared order preserved; `retained_body_bytes() == 0`, one staging object, staged total equals the sum of the committed bodies, peak live bytes inside twice the single-body bound.
- R02C-10: the suite re-reads each committed fixture from disk and asserts it equals the bytes it used; scratch trees are removed on drop and never overlap a fixture.

## 6. CI evidence

| Run | Revision | Result |
| --- | --- | --- |
| `42823587570` | `7cca97f5` (issuance on `main`) | Ubuntu success, Windows success |
| `42824870948` | `cc339342` (first branch attempt) | **FAILED on both hosts** - disclosed below |
| `42825967792` | `5596a2b1` (first substantive branch HEAD) | Ubuntu success, Windows success |
| `42826688112` | `5596a2b1` (first substantive `main`) | Ubuntu success, Windows success |
| `42827519936` | `72e67c00` (first report HEAD) | Ubuntu success, Windows success |
| `42831101491` | `c57a45e6` (corrected branch HEAD) | Ubuntu success, Windows success, 0 failed steps |
| `42831826400` | `c57a45e6` (corrected substantive `main`) | Ubuntu success, Windows success, 0 failed steps |

Both jobs run 33 steps; every `skipped` step is the other host's counterpart, so no mandatory case is skipped on both hosts. On Ubuntu the disposable Btrfs fixture was created (`btrfs-progs v6.6.3`, `fstype=btrfs`), exported as `SOLO_TEST_SOURCE_FS_ROOT`, used by the snapshot tests, unmounted and removed. The deliberate test-failure tripwire step succeeded as designed. Locked graph, `cargo-deny` all classes with a refreshed RustSec set, the pinned `1.99.1` assertion, format, strict Clippy, the architecture checker with its refusal self-test, the release CLI smoke and the byte-reproducible Ubuntu build all passed. A fresh report-HEAD run was dispatched for this report-only commit and is verified separately.

**Disclosure - first branch run failed.** `42824870948` failed the test step on both hosts for two reasons, both corrected before the first publication: the parser records an unauthored route case policy as `None` rather than defaulting it, and `include_bytes!`/`include_str!` are refused by the accepted architecture gate in the real tree. The fix reads fixtures from fixed known paths under `CARGO_MANIFEST_DIR`; the checker, workflow and every other frozen file were left unchanged.

## 7. Changed paths

Committed under this order: `examples/starter/content/publication.json`, `examples/starter/content/pages/main-home.en-001.md`, `crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json` with its six `content/pages/*` bodies, `crates/solo-adapters/tests/r02c_seed.rs`, the issuance paths `.documents/WIP/workorder-active.md` and `.documents/WIP/plan-next.md`, and this report. Ledger paths 14-16 were not used: no parser, candidate or domain defect was demonstrated. No dependency, manifest, lockfile, workflow, checker, SOP, About, Architecture, README, AGENTS or tooling file changed.

## 8. Git state

`git status --porcelain` is empty; one checkout, one worktree, sole `origin`; local `main` and `origin/main` are the corrected substantive SHA above. Branch `solo-wo-054-r01` is deliberately **retained** (its post-acceptance pruning needs a later separate authorization). Task-owned scratch directories were removed and no Owner material was touched.

## 9. True limits

- Local compilation and local test execution were **not** possible on this workstation: the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain is installed but no MSVC linker is present, so `cargo` cannot link build scripts. Every execution result above is CI evidence at the exact revisions listed; nothing is inferred from a local run.
- R02.C is **not** accepted, not removed from ROADMAP, and ACTIVE is not cleared. No R10.A installer, publication, HTTP/browser surface, route engine, locale validator, asset pipeline, Stage 03/04 work, release or deployment is present or claimed.
- The Entity location declaration and the two route/visibility declarations are inert authored data only; no inheritance, discovery, exact-case enforcement or fallback behaviour follows from them.

**Disposition: `COMPLETE`** (R02.C implementation scope, including the REVISE correction) - awaiting independent Architect review.




