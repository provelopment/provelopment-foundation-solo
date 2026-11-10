# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-054 R01**  
**Title:** Implement R02.C — checked-in default one-Entity starter seed and expanded EFS fixture family  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Initial remote `main`:** `b5139bca02acc49a5200e591145e6fec4ea244ae` (accepted administrative R02.B closure / R02.C NEXT opening)  
**Implementation branch:** `solo-wo-054-r01`, created only in the existing verified checkout, after the initial Owner-issued ACTIVE/NEXT transition is committed to `main`.  
**Owner decision and scope:** The Owner directed that R02.C proceed. This order authorizes only the concrete R02.C seed/fixture implementation and its tests, exact specified Git integration and reporting. It does **not** authorize R10.A installation machinery, full publication, a browser/HTTP server, Stage 03/04 functionality, a release, deployment or subsequent administrative closure. The Chief Architect independently reviews the final published revision.

## 1. Binding objective, accepted boundary and non-goals

Deliver **real, checked-in source bytes**, not only hard-coded test strings or a proposal:

1. An intentionally simple, inspectable, deterministic **starter content seed** with exactly one Entity, one default Facet, one default Site, one Home Page and one explicit full-locale (`en-001`) representation; valid authored Markdown is selected because the declared preferred JSON body is *genuinely absent*. This is a **source seed for a future Installation**, not an installer, activated publication, deployable site, shipped product or browser preview.
2. An **expanded, synthetic, checked-in one-Entity fixture** with exactly two Facets, three Sites, four distinct Page ownership nodes and six declared localized representations, including correctly scoped reuse of `home` beneath different Sites, full locale keys, one intentionally unlisted representation declaration and one sensitive route declaration retained only as inert metadata. Every selected body genuinely exists and passes the already implemented body preflight.
3. Meaningful positive and adversarial tests driving the actual source bytes through existing `solo-adapters::parse_manifest_bytes`, `solo-domain::EfsOwnershipDraft`/`EfsOwnershipGraph`, and `solo-adapters::assemble_declared_source_set_v1` with explicit, real, confined filesystem capabilities. Prove qualified parentage, deliberate source selection, whole-set behavior, no undeclared-source publication, and Page identity independent of display labels, authored addresses, locale count and route strings.

**Preserve exact trust classes.** The parser returns an inert syntactic `ParsedManifestCandidateV1`, the ownership graph validates structural EFS membership and qualified lineage, and the source-set assembler returns a staged **candidate**, not publication authority, a global filesystem snapshot or a renderer. The R02.C fixture tests may transform parsed declarations into domain drafts *inside the tests* using the known already-canonical fixture locale keys; this is not permission to invent a new runtime publication bridge, locale canonicalizer, route engine, installer, auto-discovery mode, generic source-fallback path or parallel EFS hierarchy.

Exactly **one Entity → Facet → Site → Page → localized representation**. No Spokes, hidden secondary Entity, route-derived parentage, locale-derived geography, filesystem filename-derived publication or inferred fallbacks. Preserve strict Hexagonal Architecture: pure domain, explicit application boundaries, outbound native/file access confined to adapters, no delivery/provider framework leakage inward.

## 2. Authoritative anchors and mandatory preflight

At this order's drafting, independently verified public `origin/main` is `b5139bca02acc49a5200e591145e6fec4ea244ae`. Its accepted R02.B substantive ancestor is `1f4de7f439e81ea9af36b655688627c598f281f2`. The canonical administrative report is `.documents/Reports/execution-report.md` at blob `a44ab968dcce2d83184c8a66f128de0ba55d29d3`; its final CI `42819616976` passed on Ubuntu 24.04 and Windows Server 2025. Current WIP blobs are:

| File | Before Owner installation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | `ea5f20b736483c2e102456be1cea9104059fc81a` (neutral, title only) |
| `.documents/WIP/plan-next.md` | `ba110885d1d511240b4c31cf3a9566ef7afeec05` (the accepted R02.C NEXT candidate) |
| `.documents/WIP/plan-roadmap.md` | `e7be6b4a78f0da0f6f4f09e92b2eb31c2409f437` (R02.C remains) |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` (unchanged) |

Before *any* permanent change, read `README.md`, `AGENTS.md`, both `.documents/SOP/` SOPs (Architect §§6, 6B, 7A, 7B, 8; Implementation §§5–9), all four WIP files, the canonical report, `.documents/About/FOUNDATION-SOLO.md`, relevant `.documents/Architecture/ARCHITECTURE.md` §§1.1, 1.3–1.7, 3.2, 6.2B, 6.2M.1, 6.2M.2, 6.2N/P01–P04/P13, 6.3/V01–V06 and current code/test APIs. Verify the **real root** and `git rev-parse --show-toplevel`; `main`, `HEAD == origin/main`; one worktree/checkout; sole remote `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo.git`; exactly four WIP files; no unexpected dirty/untracked data; and the Owner-installed entire order with matching Owner handoff ID `SOLO-WO-054 R01` as the sole permitted initial local tracked delta. Verify branch inventory, published report, source ancestry and actual file blobs. Preserve unknown Owner-owned material. If `main` advances, classify and verify the diff/ancestry before proceeding; stop on any unexplained change.

**Existing proof, not a blank slate:** `efs_ownership.rs` already validates cardinality/lineage, `publication_manifest.rs` parses strict manifests, `source_set_candidate.rs` stages real declared sources, and `source_set_candidate.rs`/`publication_manifest.rs` tests already exercise synthetic in-memory expanded JSON. Do not replace, fork or duplicate these mechanisms. R02.C's new value must be **checked-in source fixtures and exercised cross-boundary proof**, not a renamed repetition of those old tests.

## 3. Exact permanent-write ledger — and everything else frozen

Only these paths may be created/changed; intermediate directories implied by **new authorized files** are permitted, but no other file, source or fixture is:

| # | Exact path | Permission |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | Owner installs this full order; commit it once as an ACTIVE authorization transition, freeze thereafter |
| 2 | `.documents/WIP/plan-next.md` | At that **same** issuance transition, reset to its original one-line identifying title `# Foundation Solo — plan-next (Next Candidate)` followed by newline; freeze thereafter |
| 3 | `.documents/Reports/execution-report.md` | Overwrite once at end as one canonical report, with permitted report-only correction if unavoidable |
| 4 | `examples/starter/content/publication.json` | New, exact starter manifest |
| 5 | `examples/starter/content/pages/main-home.en-001.md` | New, real and safe starter body |
| 6 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json` | New, complete six-representation expanded synthetic manifest |
| 7 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/bali-home.en-001.md` | New, selected body |
| 8 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/bali-home.id-id.md` | New, selected body |
| 9 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/bali-about.en-001.md` | New, selected body |
| 10 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/bali-about.id-id.json` | New, selected JSON body |
| 11 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/jakarta-home.en-001.md` | New, selected body |
| 12 | `crates/solo-adapters/tests/fixtures/r02c/expanded/content/pages/company-home.de-de.md` | New, selected body |
| 13 | `crates/solo-adapters/tests/r02c_seed.rs` | New focused real-fixture integration test |
| 14 | `crates/solo-adapters/src/publication_manifest.rs` | **Optional** minimum correction of a newly demonstrated R02.C parser/contract defect, with regression |
| 15 | `crates/solo-adapters/src/source_set_candidate.rs` | **Optional** minimum correction of a newly demonstrated R02.C candidate defect, with regression |
| 16 | `crates/solo-domain/src/efs_ownership.rs` | **Optional** minimum correction of a newly demonstrated R02.C structural defect, with regression |

Paths 14–16 are NOT permission for opportunistic refactoring, public feature extension or new parsing/route/locale design. If existing contracts cannot support the required fixture without a broader runtime interface, a new dependency or a seventeenth path, **STOP with evidence**; do not add architecture autonomously. Do not modify existing candidate/manifest/domain tests merely to silence a regression. No new crate, module, executable, binary placeholder/media file, README, separate manual, secondary report, generated output, or source-copy directory.

**Every other tracked path is frozen**, especially `.documents/WIP/plan-roadmap.md` (R02.C must remain pending), `plan-outlook.md`, About, Architecture, SOPs, AGENTS, README, `.github/workflows`, `Cargo.toml`, `Cargo.lock`, toolchain, `deny.toml`, `lib.rs` of any crate, CLI, `source_read.rs`, `source_read_native.rs`, `source_set_staging.rs`, `source_body_preflight.rs`, security/architecture checkers, release and deployment files. Preserve the existing pinned Rust `1.99.1` and dependency/tool graph. No package install, version update, unsafe/FFI expansion or CI shortcut.

## 4. Precisely required authored seed and fixture content

### 4A. Real starter seed (source-controlled, never auto-installed)

At `examples/starter/content/publication.json`, use the existing schema `1.0.0` (and both registry version fields `1.0.0`) with `base.canonical = "https://example.org/site"` and a fully qualified `base.landing` of `{entity: "example", facet: "main", site: "main", page: "home"}`. It declares exactly Entity `example`, Facet `main`, Site `main` (`defaultLocale: "en-001"`, optional explicit `languagePreferences: {"en": "en-001"}`), Page `home`, and one representation (`locale: "en-001"`, authored `route.path: "/en-001/home"`). Its `sourceCandidates` declares **both** `json: "pages/main-home.en-001.json"` and `markdown: "pages/main-home.en-001.md"`. Do **not** create the JSON file: its observed absence is intentional and allows the Markdown selection under the accepted JSON-first policy. The markdown path is real, nonempty, preflight-valid and human-readable; use one H1 and safe copy. Avoid reference to a missing graphic or asset: Architecture §6.2M.1's illustrative Markdown image requires an actual asset, which this scoped seed does not provide. No fake installed defaults, hard-coded real operator address, secret, external fetch or undeclared source.

The source seed is portable **data**; there is no command that publishes it automatically. Its authored base and route are inert declarations at this stage, not verified URLs, and `label` values do not assign ID.

### 4B. Expanded single-Entity fixture with six exact representations

`crates/solo-adapters/tests/fixtures/r02c/expanded/content/publication.json` follows Architecture §6.2M.2's structural example, constrained to this stage's actually supported parser/source preflight. One Entity `example`, two Facets `visitors` and `company`:

| Facet | Site (defaultLocale) | Page | Representation(s) and real selected file |
| --- | --- | --- | --- |
| `visitors` | `bali` (`en-001`) | `home` | `en-001` → `pages/bali-home.en-001.md`; `id-ID` → `pages/bali-home.id-id.md` |
| `visitors` | `bali` (`en-001`) | `about` | `en-001` → `pages/bali-about.en-001.md`; `id-ID` → `pages/bali-about.id-id.json` |
| `visitors` | `jakarta` (`en-001`) | `home` | `en-001` → `pages/jakarta-home.en-001.md` |
| `company` | `global` (`de-DE`) | `home` | `de-DE` → `pages/company-home.de-de.md` |

This is **two Facets, three Sites, four Pages, six representations**, including Page-ID reuse under three distinct Sites. Declare no two source candidates with the same logical name, and no source named by an undeclared filename. Include the authored `base.landing` for `visitors/bali/home`; use representative explicit full-locale `route.path` values from Architecture §6.2M.2 (`/en-001/home`, `/id-id/Hidden7A`, `/en-001/about`, `/id-id/TentangKami`, `/en-001/jakarta`, `/de-de/unternehmen`). Retain `visibility: "unlisted"` as an **inert declaration** for Bali Home's `id-ID`, and `caseMatching: "sensitive"` for Bali About `id-ID`; don't claim that route enforcement, discovery or visibility has been implemented here. No base/route/locale/location inference. Properties, location and graphics may only be treated as inert authored data under the current parser; do not implement inheritance or asset delivery in this order. Prefer omitting unresolvable graphic references rather than shipping incomplete assets.

At least one expanded representation must use a real JSON body (`bali-about.id-id.json`) with the **current accepted preflight envelope**: `contentVersion: "1.0.0"`, a nonempty valid `title`, and a nonempty `blocks` array (e.g. `[{}]` is inert at this stage). All other selected bodies are real valid Markdown with a single primary heading and nonempty content. If adding declared JSON+Markdown alternatives to test precedence, an **absent** alternative may remain intentionally absent, and any extra candidate pathname must be unique; never silently create an unlisted extra tracked file.

### 4C. Fixture harness and safety

`tests/r02c_seed.rs` must exercise the **actual files** above. It may read fixed checked-in fixture bytes with `include_bytes!`, `include_str!` or a fixed known path relative to `CARGO_MANIFEST_DIR`; the tests must not modify source-controlled fixtures, run a recursive copy of unknown Owner material, scan source directories to infer publication or grant publication authority. To exercise the actual adapter's confined reads and staging, construct **exclusively claimed, task-owned scratch directories** with explicit known-file materialization, separate adopted `content` and `staging` roots, and resource-safe cleanup. On Linux use the already configured real `SOLO_TEST_SOURCE_FS_ROOT` Btrfs fixture (fail CI if missing); on Windows use the existing safe runner temporary root. Follow existing `tests/source_set_candidate.rs` fixture patterns; no fixed global location, broad pre-clean, symlink traversal, destructive cleanup of unknown names or network access.

You may build a test-only pure `EfsOwnershipDraft` from the *already parsed* declaration graph using `FacetDraft` / `SiteDraft` / `PageDraft` and fixed already-canonical `LocaleKeyV1` values. Validate complete qualified Page and representation lineage in authored order; do **not** claim that this construction is the missing production parse→graph publication bridge or general BCP47 validator. Testing both structural and source-set APIs is required, but never equate their *provisional* success to a fully validated publishable Installation.

## 5. Acceptance matrix — execute, do not just describe

All applicable cases below require named tests and direct Ubuntu/Windows logs. Reuse accepted coverage where appropriate but test the **new committed files** in each central case.

- **R02C-01 Starter files:** starter manifest parses with exact one/one/one/one/one counts and IDs, sole explicit full locale, real Markdown bytes present, declared JSON genuinely absent. Exactly one selected staged member, `Markdown`, correct path, bytes/fingerprint/lineage, no publication/snapshot authority.
- **R02C-02 JSON-first upgrade/refusal:** in separate owned test scratch, add a valid JSON file *at the already-declared preferred path* and show it wins without changing Entity/Page identity; add a present-but-invalid JSON instead and show the entire candidate refuses without Markdown fallback. Never mutate checked-in starter files.
- **R02C-03 Expanded files:** parse the committed expanded manifest, observe two Facets, three Sites, four Pages, six representations and exactly the six authored lineages in manifest order; selected bodies read and stage byte-identically; the declared selected JSON is `Json`, the rest are `Markdown`.
- **R02C-04 Parentage/isolation:** build the corresponding pure domain graph; resolve complete valid targets; Page `home` reused legitimately under multiple Sites, but wrong Facet/Site/Page/Entity or locale gets the correct registered refusal; no cross-parent selection, borrowed source or guessed default.
- **R02C-05 Exactly-one-Entity/structural negatives:** malformed second-Entity shape, root-level invented entity collection/extra unknown fields, duplicate scoped Facet/Site/Page/locale, missing required child arrays/empty Page representations, and invalid bare-language membership keys are refused with existing typed diagnostics. Nested syntax cannot express an orphan independently: attempt an unauthorized parent hint/unknown field and a wrong qualified ancestor instead of inventing a new orphan schema.
- **R02C-06 Declared-source isolation:** undeclared body placed in owned scratch is neither selected nor included; duplicate declaration of a logical source path refuses before stage; where an actual hardlink/case alias attack is applicable, reuse real adapter proof without weakening the existing reservation gate. Same name under another parent is only legal when it is an ID, never when it aliases a claimed source body.
- **R02C-07 No partial candidate:** missing, malformed or inaccessible selected source in one expanded representation refuses **the whole** source-set candidate rather than publishing a sibling, a partial graph or another format; error is typed and not misreported as absent when the host denied access.
- **R02C-08 Identity independence:** with same author-approved Entity/Facet/Site/Page IDs and parentage, modify test-owned **labels**, base origin/prefix, authored route strings and the set of explicit full-locale representations, and prove Page target identity is unchanged while representation membership/selection changes only when explicitly authored. Do not imply full route/locale/canonical validation or infer identity from filenames/geography.
- **R02C-09 Budgets/order/provenance:** preserve manifest/body bounds, EFS node/page budgets, declaration order, qualified staging evidence, bounded transient bytes and exact selected-only body read rules; unchanged accepted baseline tests remain green.
- **R02C-10 Safety and preservation:** no writes to checked-in example/test fixtures during tests, no unknown cleanup, no retained scratch, no private path/secrets in public diagnostics. Linux real Btrfs and Windows actual filesystem tests execute; security/architecture/unsafe gate remains unchanged and green.
- **R02C-11 Scope truthfulness:** test/readme comments and final report consistently call the outputs *seed / inert manifest / ownership graph / source-stage candidate*, never installed/published/served/rendered. No installer, HTTP, browser route, asset pipeline, module renderer, contact form, provider or release action.
- **R02C-12 Git and CI:** authorized paths only, one checkout and remote, both-host green at the **same** substantive branch SHA, a fresh two-host green at substantive `main` SHA, and final report-HEAD green; no agent self-acceptance or R02.C ROADMAP removal.

If a target is not testable on one host, mark it `UNVERIFIED` and stop rather than silently skipping it. A test requiring a genuine attack must construct the attack to pass. Never mask failed tests, add empty assertions or loosen existing negative cases. The `rust-verify` workflow's existing deliberate negative tripwire is expected to show a failed **subprocess** while its enclosing step succeeds; don't report that as a product regression.

## 6. One-checkout Git, CI, reporting sequence

1. **Owner issuance:** the Owner installs this entire `SOLO-WO-054 R01` at the exact ACTIVE path and hands the matching activation prompt to the agent. Verify the local delta is only this installed order. In one commit on `main`, commit **only** `.documents/WIP/workorder-active.md` and the title-only cleared `.documents/WIP/plan-next.md` as the specifically authorized issue transition. Push, verify the exact remote SHA/tree, and verify CI green on both hosts. Do not touch ROADMAP: R02.C is not accepted yet.
2. In that same verified checkout, create only `solo-wo-054-r01` from updated `origin/main`; implement the fixture/seed/test path ledger, and (only if evidenced) bounded permitted fixes. Do not create a second worktree, clone, nested repository, remote, alternate source tree or PR. Do not copy unreviewed material from the historical Foundation project. Never use `reset`, force-push, destructive checkout/clean or history rewrite. Preserve the original accepted R02.B code.
3. Run focused `cargo test --locked -p solo-adapters --test r02c_seed`, the relevant `publication_manifest`, `source_set_candidate`, `efs_ownership` regressions, format, strict Clippy, and the **unchanged full `rust-verify`**. The two real workflow jobs (`ubuntu-24.04`, `windows-2025`) must pass at **one identical branch HEAD**. Inspect raw logs including mandatory Linux Btrfs, Windows source-stage attacks, Rust 1.99.1, architecture packages=4/checker 29 refusal cases, locked graph, cargo-deny all classes, refreshed RustSec, security advisory negative probe, release CLI smoke, Linux reproducible build and warnings/host-specific skips. Fix only authorized defects; do not modify workflow/checkers to get green.
4. Independently verify branch/main ancestry, exact changed-path scope, no remote advance, and clean status. Integrate the verified branch to `main` **fast-forward only** (no PR merge, squash, rebase or force). Push, verify substantive `origin/main` exact SHA/tree and obtain a fresh full two-host green run **on that substantive `main` SHA**. If a legitimate new main commit appeared, use only non-destructive branch reconciliation and reverify; STOP on unexplained drift/conflicts.
5. **Last permanent write:** overwrite only `.documents/Reports/execution-report.md`, commit/push that single path from `main`, and obtain fresh two-host green `rust-verify` at final report HEAD. Record actual fixture counts, names, measured positive/negative outcomes, exact substantive source SHA/tree, final report HEAD returned *in chat only*, source and WIP blob ledger, CI run IDs/job conclusions, warnings/skips, Git cleanliness, no source fixture mutation and true limits. Report `COMPLETE` only for technically completed R02.C scope; leave R02.C in ROADMAP and ACTIVE issued for independent Architect review. If stopped, overwrite the one report with `STOPPED`/`FAILED` truth when safe.
6. At end, ordinary `git status --porcelain` must be empty, local/remote `main` and sole origin verified, task-owned scratch removed safely, no unneeded task files retained; a local-only discrepancy must be clearly disclosed rather than asserted as verified by GitHub.

**Git authority for this specific order:** commits, ordinary pushes, one task branch, standard non-destructive merges needed for legitimate main drift, **fast-forward-only** integration to `main`, existing workflow dispatch/read and final report-only commit are authorized for precisely the paths above. Deleting this task branch is **not** necessary here; its post-acceptance pruning requires a later separate authorized closure. No release, tag, deployment, DNS, provider, secrets, external account or other-repository write is authorized.

## 7. Stop conditions and role separation

**STOP** for: missing entire Owner-installed order or mismatched handoff; unrelated dirty work; wrong root/remote/second worktree; unexpected source, SOP, WIP or CI workflow change; an unavailable required host/Btrfs mount; a fixture that cannot satisfy the accepted schema without a wider change; a new public API/module/dependency/architecture decision beyond the ledger; a proposed implicit locale, route, location, second Entity or source discovery; security/ownership ambiguity; unproven adversary; red mandatory CI or unexplained warnings; overwritten Owner content; a branch merge that is not safe/fast-forward; or inability to publish the exact single canonical report. Preserve evidence and request a new Owner decision rather than expand scope by implication.

The Chief Architect remains **read-only on GitHub**, independently inspects the published report/source/tests and issues `ACCEPT / REVISE / STOP`. The Implementation Agent cannot self-ACCEPT, remove R02.C from ROADMAP, close Stage 02, clear ACTIVE, create the next order, release or deploy. After acceptance, a **separate Owner-authorized administrative closure** reconciles ROADMAP/NEXT/ACTIVE according to Architect SOP §8.

**End of SOLO-WO-054 R01.**
