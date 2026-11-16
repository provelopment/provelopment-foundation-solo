# SOLO-WO-118 R16 — COMPLETE R15 N127 PROOF, ADOPT OWNER PLATFORM-FIRST GRAPHIC CASCADE TARGET, AND ACCOUNT FOR ALL FOUNDATION ARTWORK

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's sole checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `.documents/WIP/workorder-active.md` in that checkout  
**Required bounded outcome:** `R03C-R15-N127-POST-INSTALL-GREEN-FOUNDATION-142-ASSET-LEDGER-OWNER-GRAPHIC-CASCADE-TARGET-V1`.

## 1. Chief Master Architect adjudication and binding Owner direction

**R14 ACCEPTED; R15 JUSTIFIED STOP (NOT COMPLETE).** Independent audit of canonical R23 report, commits, native logs and source confirms:

- `main` **R23** `40c8ee1aaf472435ec5c4a267994c7005b6fa62b`, report-only after A23 `3e2497e920fa547e7dea3d706cbbaa9734ef737a`.
- Technical branch `solo-wo-118-avif-v4-r01` **T23.3** `8b340976836f86b79dea36be8c9db9d8576997dd`. Technical run `43816931677`: Ubuntu `133131005793` **red**, Windows `133131005880` **red**, independent AVIF oracle `133131005598` **green**. BOTH workspace jobs have exactly two historical absence failures; **all four actual N127 tests are green on both native hosts** but the downstream dedicated N127 authenticity/mutation steps did **not** execute. No full three-success technical run exists.
- Main R23 baseline `43817901578`: Ubuntu `133134040690`, Windows `133134041094`, both success. Architecture on technical branch `packages=4 targets=38 files=74 edges=93`.
- R15 installed the **exact 81 Owner-approved Tabler/MIT SVGs** and pinned catalogue + MIT notice + provenance (`84` product files) under `platform/assets/`. Its separate `PlatformRootCapabilityV1`, `read_pinned_platform_sources_v1`, exact catalogue/source digests, refusal matrix, and 81 genuine confined SVG inspections were demonstrated by N127 test bodies. The approved product files, source bytes, source read capability, registry and decoded results must not be edited by this order.

**New Owner directives, issued directly after the R15 report:**

1. Solo's global Platform library shall sit at the **top/root of the graphic property cascade**; when a closer qualified owner explicitly **supersedes** a selected graphic source, the closer selection shall win. A suppressed graphic shall stay suppressed; a bad or absent closer source must **not** quietly trigger a different Platform image.
2. Account for **ALL** publicly available Foundation artwork, **not merely the 81 neutral icons**. Preserve the distinction between neutral reusable icons, role placeholders, social/platform trademarks, duplicate semantic aliases, original Provelopment artwork, deployment mirrors and test/synthetic fixtures. Make every source file visible in the ledger and plan lawful delivery of eligible categories without claiming the public GitHub repository grants every redistribution or trademark right.
3. Be **risk-proportionate**: mechanical source additions should receive cheap complete byte/hash/licence/inventory validation plus representative media checks; source confinement, cross-namespace boundaries, cascade precedence, visibility and publishing controls deserve stronger negative and cross-host proof. Avoid another gratuitous per-icon adversarial test suite. Preserve already accepted oracles and CI while this repair is executed; do not waive a genuine high-consequence gate to reduce test time.

These directives supersede the **future target** of the old no-Platform-default and additive-only graphics design, but do NOT magically change the code already delivered. This R16 order **records the normative amendment and completes the R15 gate**; R16 does **not yet claim that automatic graphic baseline/supersession is implemented** or that all Foundation assets are installed. Implementing a new graphic precedence algorithm is a separate, high-consequence code change with targeted security tests. No revision to current runtime behavior is hidden inside a test-only repair.

## 2. Immutable source-of-truth pins and repository discipline

- Technical parent: T23.3 `8b340976836f86b79dea36be8c9db9d8576997dd`.
- Main parent: R23 `40c8ee1aaf472435ec5c4a267994c7005b6fa62b`.
- Foundation donor: `provelopment/provelopment-foundation` **commit** `fa599973a88ff77f6a8e012f3266c3c139b52c17`, recursive tree `89b57a92869d2e293d33527589f3f4680938f9b2` (not movable donor `main`).
- Approved Solo library candidate: catalogue blob `a02db1605bac6cf1b45214a07b452d58895d7742`, **65,397 raw LF bytes**, SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`, 81 exact named source rows; review-sheet blob `edb8468d7ef6ae290574672e1829bf5b583419ca`.
- MIT notice donor blob `3e82379dab3fe93d9ee22251949604ed63ddea39`, SHA-256 `b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db`; original donor provenance blob `a74d234417d5bb63ff05e2b64f93d757864f3d46`, SHA-256 `e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b`.
- Use existing technical branch; one checkout and origin, four established WIP paths, no new branch/tag/worktree/remote, no force/reset/clean, no merge, no release/deploy. The two Dependabot-created remote heads are unrelated; observe their actual state rather than treating `30` as immutable. GitHub inspection may be read-only; only the Implementation Agent with Owner authority writes repo commits.

## 3. Exact staged transaction and allowlist

1. **A24** on `main` parent R23: install this entire ACTIVE order in `.documents/WIP/workorder-active.md`; stage/commit **ONLY ACTIVE**, verify index/worktree/commit/`origin/main` blob identical; push main.
2. **T24.0** on existing technical branch parent T23.3: import **ONLY ACTIVE** blob from A24, verify one-path change, commit/push.
3. **T24** code-bearing commit on technical branch: only the precise changed-path ledger below. Perform fast local focused tests first, then full native CI; repair forward if a genuine defect appears. Do not bypass first failing gate.
4. **R24** main report-only commit, parent A24, changing only `.documents/Reports/execution-report.md`; push and confirm fresh main Ubuntu/Windows baseline. Report exact full SHAs, blob IDs, paths and job IDs, including red history and Owner directives. No `COMPLETE` unless all mandated technical CI jobs succeed at one T24 SHA.

**Allowlisted paths:**

```
.documents/WIP/workorder-active.md                                    # A24 + T24.0 only
crates/solo-cli/tests/media_worker_host_smoke.rs                        # change ONLY two obsolete absence loops; small shared test helper permitted
.documents/Architecture/ARCHITECTURE.md                                 # short explicitly labelled Owner target amendment, not a rewrite
.documents/Reports/foundation-donor-full-artwork-inventory.json          # NEW: all pinned donor image paths with status
.documents/Reports/foundation-donor-full-artwork-and-cascade-decision.md # NEW: exhaustive disposition, risk matrix, next implementation scope
.documents/Reports/execution-report.md                                  # R24 on main only
```

**Not allowlisted:** `.github/workflows/rust-verify.yml`, `platform/assets/**`, `crates/solo-application/**`, `crates/solo-adapters/**`, `crates/solo-domain/**`, existing R12/R13/R14 catalogue/review/approval records, donor fixtures, `.gitattributes`, toolchain/dependencies/lock/policy, sanitizer, reader, decoder/worker, oracle, architecture checker, any pre-existing N093–N127 body **except the two precisely designated loops**. No `#[ignore]`, `continue-on-error`, `--skip`, output spoofing, or fixture byte normalization. If new status demands broader code or legal authority, STOP and report rather than expanding scope.

## 4. Repair exactly two obsolete historical absence assertions

The only R15 red tests are in `crates/solo-cli/tests/media_worker_host_smoke.rs`:

- `n125_04_real_entity_reader_still_refuses_platform_and_no_unapproved_library`, around line **8240** at T23.3;
- `n126_02_pinned_mit_provenance_and_r12_catalogue_review_only`, around line **8676** at T23.3.

Both have the same loop rejecting the existence of `../../platform` and `../../platform/assets`, inherited from the earlier state in which no product library had been approved. **Do not delete the tests, rename them, or remove Entity/Platform isolation assertions.** Replace *only* those obsolete loops with a small shared, test-only assertion `assert_absent_or_exact_approved_platform_library(...)` or equivalent, preserving all other test-body behavior.

**Required predicate:**

- If `platform/` and `platform/assets/` are wholly absent, accept historical pre-install state (for a branch without the approved library); a half-created `platform/` tree is NOT considered absent.
- If installed, require **exactly** the expected `84` regular files under the product `platform/assets/`: 81 `icons/<name>.svg` whose exact relative IDs are the fixed 81 approved candidate rows; one `catalogue-1.0.0.json`; the exact original MIT notice and `icon-provenance.json`. Enumerate actual paths deterministically and compare full set (not just the count), reject extras, unexpected directories, symlinks, withheld alias filenames, an `assets` subtree without `platform` or vice versa. Do not follow symlink/reparse paths during the test-only enumeration.
- Verify pinned **catalogue SHA-256** and its parseable `1.0.0` 81-entry membership; confirm installed product catalogue is byte-identical to the already approved candidate document. Preserve the MIT/provenance-presence assertions as appropriate. For individual content byte security, rely on **unchanged** N127 source digest/reader checks rather than introducing a second redundant 81-asset sanitizer matrix; optionally spot-check actual fixture blob identity if cheap.
- On either branch, still assert `../../content/platform` is absent and preserve the actual Entity reader's `PlatformUnverified`, the R12 review binding's `PlatformSourceUnavailable`, zero alias fallback, and no publication authority. Historical `n125_evidence`/`n126_evidence` lines concern their **original review-only seams**, not absence of a product library after R15; do not forge new live-Platform claims by modifying log tokens.

Do not use this predicate to grant publication, omit malformed asset refusal, or turn any unsafe Platform source into an Entity source. Do not weaken the N127 full-system case. Existing N125/N126 N127 CI gates must run unmodified after workspace passes.

**Local fast proof before pushing:** run the two changed tests, verify accepted tree branch and reject a *test-only* unauthorized extra file / missing catalogue / malicious symlink / wrong-catalogue digest. Clean test-only scratch safely; do not modify production `platform/assets/`. Then run the original workspace suite `--locked` without skip or waiver.

## 5. Owner normative direction: global Platform-first graphic cascade, target only

Add a concise, **clearly labelled future-target Owner amendment** at the end of §6.2I of canonical `.documents/Architecture/ARCHITECTURE.md`, cross-referencing §§3.3A and 6.2G. Do **not** erase archival original rules or pretend the current implementation obeys the new target. State explicitly that, for graphic properties, this later Owner direction controls once the separately gated migration is implemented; until then, current additive `property_resolution` and existing outputs remain as coded.

The amendment MUST settle these minimum target semantics for the NEXT work order:

1. **Global root layer, not an EFS node.** The read-only version-pinned `platform/assets/` library is an available root-level source of graphic role/slot defaults. It is not a hidden Facet, alternate Entity, or public publisher. Each default must identify an **exact role/slot and an approved catalogue ID**, with deterministic explicit registration/authoring; do not guess that `icon-home.svg` is a banner, assign generic logos, or invent a favicon. Absence of a registered default means the role is absent, not any available icon chosen by filename.
2. **Closer wins on explicit supersession.** Given an eligible registered default or inherited graphic choice, a descendant's explicit validated **supersede/replace** selection for the same graphic role/slot takes precedence over that selected ancestor. The nearest *valid explicitly superseding* declaration prevails; no silent coalescing of two singular values into a cardinality failure for the new supersede operation, but conflicting same-owner selections still refuse. Keep the operation typed/closed and do not change unrelated location, SEO or contact D08 inheritance semantics.
3. **Suppression and bad inputs are final.** Explicit suppression blocks the inherited/default graphic for its scope. If closer selected bytes are missing, invalid, private/ineligible, or wrong-class, refuse the WHOLE candidate; never resurrect a Platform default as an automatic error fallback. No implicit reappearance after an internal/unlisted property is omitted by public projection. Preserve complete validation of every authored obligation including suppressed, internal and unlisted sources.
4. **Collection roles and provenance.** A graphic collection may append under explicit existing additive semantics, or supersede a **uniquely identified slot** (no guessed first/last/background index), or suppress and explicitly repopulate; never silently overwrite unrelated entries. Record the winning graphic's real namespace, origin owner/slot, source SHA and supersession chain in internal provenance; public projection only after exposure/route validation. Library source reference remains explicitly `namespace:"platform"` at the boundary, even when its default comes from the root layer.
5. **Migration proof and risk.** No global auto-default is activated until the new precedence algorithm and registration/migration contract are in place, backwards compatibility is intentional, and a focused R17 native two-host suite covers root default, closer override, suppress, invalid closer, wrong role/slot, same-owner conflict, mixed Entity/Platform owners, internal/unlisted and no unauthorized fallback. The architecture amendment is **normative target direction**, not evidence these cases pass today.

If a concrete unresolved schema choice would change the Owner's requested meaning (e.g., collection slot identifiers vs full replacement), document the exact question and a recommended safe implementation in the decision report; do not create an unauthorized live behavior under this R16 ledger. Do not silently use the old §6.2I no-fallback language to defeat the new Owner directive: its original default-deny intent remains for **undeclared/invalid** substitutions; the Owner has now expressly authorized a governed root default and explicit supersession policy to be implemented in a later bounded step.

## 6. Full immutable Foundation asset inventory — every publicly visible source accounted for

Produce `.documents/Reports/foundation-donor-full-artwork-inventory.json` as a deterministic machine-readable, **unapproved** source ledger over the immutable Foundation donor commit `fa599973a88ff77f6a8e012f3266c3c139b52c17`. Read the real donor tree, never current `main` and never infer image contents from filename. Treat the following known baseline as a reproducible assertion to verify, not as a license grant:

| Pin-scoped source category | Tracked source media files | Proposed destination / status |
|---|---:|---|
| Canonical Foundation neutral icon library `deployment/spokes/foundation/content/assets/icon-library/icons/` | **84** | 81 **already installed/approved**, `icon-services.svg` original and two byte-alias names individually **pending**, not secretly relabelled MIT |
| Canonical Foundation role placeholders `deployment/spokes/foundation/content/assets/placeholders/` | **10** | **pending** compatibility/role and ownership review; do not classify as approved brand or assign to arbitrary logo/favicon roles |
| Canonical Foundation social/platform marks `deployment/spokes/foundation/content/assets/platform-marks/` | **7** | **pending** third-party trademark/use-condition review; separate licence classes, not Tabler MIT and not default for generic graphic roles |
| Mirrored EFS sample brand assets under `deployment/efs/` | **4** | record all paths and duplicate blobs, but do not independently ship Entity-owned demonstration identity as generic Platform branding |
| Mirrored Germany placeholder role assets | **10** | record all source paths, exact blob aliases, no duplicate independent delivery |
| All `tests/fixtures/**` media | **27** | record all paths, mark TEST ONLY / NOT PRODUCT; do not expose test/synthetic artwork as a public runtime library |
| **Total donor media matching `.svg/.png/.jpg/.jpeg/.webp/.gif/.avif/.ico`** | **142** | **No path omitted.** Canonical source pool 101; mirrors/tests 41. Re-measure and explain any discrepancy before continuing. |

Required per-source ledger fields: exact relative donor path; donor commit and Git blob; size; normalized extension/media category; original category and semantically correct role (if known); whether bytes are a duplicate of an earlier donor blob; relationship to a canonical source; known provenance/licensing record URI/path; **rights decision state** (`owner-approved-exact-81` / `review-required` / `third-party-conditional` / `test-only` / `mirror-non-authoritative`), and intended Solo placement or explicit non-delivery reason. For all **101 canonical** media files, independently calculate raw source SHA-256 from pinned donor bytes if reliably available and record that; for all 142 paths record exact Git blob and size at minimum. A missing or unfetchable source is an honest STOP, not a filled-in digest. Normalize no source bytes. Maintain source authority: a test fixture or a deployment mirror does not become independently approved art merely because it has a public Git URL.

In the paired `.documents/Reports/foundation-donor-full-artwork-and-cascade-decision.md`, report:

- complete count reconciliation 142/101, all 81 approved catalogue IDs preserved, 20 canonical outstanding paths (3 icons incl two aliases, 10 placeholders, 7 platform marks), duplicate identities, rights/role classification, notice/attribution and runtime-use obligations;
- source documents actually inspected: donor `BRAND_ASSETS.md`, `deployment/spokes/foundation/content/assets/README.md`, `icon-library/icons/README.md`, `licensing/icon-provenance.json`, `licensing/TABLER-ICONS-MIT.txt`, and `platform-marks/platform-marks-provenance.md` at the pinned commit; the latter explicitly calls the seven marks **PARKED — NOT INTEGRATED / Not approved** and lists conditionals P-1..P-4; do not cite its historical wording as legal clearance;
- a **single sequenced next implementation proposal**, not four overlapping work streams: first address licensing/role support for the 20 sources and explicit alias model, then add the governed Platform baseline / closer supersession and corresponding bounded tests; recommend which categories can be safely installed without inventing rights, and which need a named Owner rights decision. The user wants all assets brought into Solo; do not treat **excluding pending classes from immediate release** as forgetting or refusing them. Preserve a concrete disposition for every one of the 142 source paths.

Do **not** copy the 20 pending canonical images into distributable product `platform/assets/` under this R16 work order; the approved `1.0.0` 81-entry catalogue remains unchanged. Importing unapproved trademark artwork into another public repository may itself be redistribution. The new inventory is a source manifest and implementation queue, **not** a product catalogue or an authorization to publish. A separately approved asset extension must be exact-version pinned; do not mutate `1.0.0` in place without an explicit compatibility/version decision.

## 7. Risk-proportionate test policy, not risk blindness

In the decision report add a compact table with risk (likelihood × impact), minimum proof and prohibitions:

| Change category | Risk / consequence | Minimum future proof |
|---|---|---|
| Byte-for-byte donor-art import, within already accepted class, no resolver/decoder change | Lower behavioral impact | complete file/count/raw SHA-256/case/size/provenance check; MIME/geometric bounds; cheap representative type-specific inspection. Reuse known per-asset sanitizer proofs where raw digests match. **No new 81× adversarial matrix solely because artwork was copied.** |
| New external mark or role placeholder | Moderate/legal/UI risk | source/provenance/use-conditions and authorised purpose, relevant contrast/size/clear-space, versioned catalogue impact, refusal of disallowed use. No claim that passing Rust tests supplies trademark rights. |
| Alias/index/metadata schema, catalogue version or immutable package assembly | Moderate/high integrity risk | exact membership, no digest aliases unless separately governed, negative tamper/missing/extra-version checks, deterministic packaging; cross-host where filesystem semantics may differ. |
| Platform root acquisition, isolation, cross-namespace resolution, five-level graphic override/suppression/visibility | **High** confidentiality/integrity risk | focused adversarial native Ubuntu/Windows tests, duplicate/conflict/unsafe/suppressed/internal/unlisted/late-failure cases, fail-closed whole candidate, retained AVIF oracle and relevant mutation gates. No skipping these for speed. |
| Publication, public URL/derived output, releases | High external effect | requires its **own** future Owner authorization, privacy/security proof and immutable-output contract. Not in R16. |

Verification effort scales with **new risk and consequence**, not with the raw number of old SVG filenames. Existing N093–N127 test bodies and authentic/mutation CI gates stay intact except the two designated obsolete assertions; test redundancy reduction itself, if ever sought, needs a separate proved equivalence and targeted Owner/Architect approval. Do not present a green per-icon hash loop as sufficient evidence for untrusted media parsing or a confined root: those are independent trust boundaries already proven or still required.

## 8. Mandatory R16 acceptance gates

**For T24 at one complete SHA:**

1. Changed-path ledger exact; previous 84 product files, candidate/catalogue/notice/provenance, owner decision, original donor fixtures and `.gitattributes` blobs **unchanged**; current graph unchanged except docs/test contents (`packages=4 targets=38 files=74 edges=93` unless a legitimate checker re-measurement explains otherwise).
2. The two repaired N125/N126 tests pass without disabling any of their other assertions; deliberate unauthorized extra product file/missing pinned catalogue/wrong hash must fail the new shared predicate. Existing N127 cases remain **4/4 green**.
3. All workspace Rust tests, formatting, strict clippy, native filesystem security/Btrfs canaries, denial/architecture and release reproducibility gates pass natively at T24. No WSL local snapshot tests are described as native-host success.
4. **Dedicated N127 gate executes and accepts authentic and rejects mutated evidence on BOTH Ubuntu and Windows**. Its **81 real confined workers per host** and four cases must be supported by actual emitted CI logs, at this same SHA. The independent AVIF R03/R08 oracle must pass at this SHA; N093–N126 gates also retained. Never claim N127 success based only on the previous R15 workspace `... ok` lines.
5. New 142-path inventory must reconcile the pinned donor and distinguish approved 81, pending 20 and mirror/test 41 with honest class/rights states; the architecture amendment must be visibly FUTURE TARGET not CURRENT IMPLEMENTED. Neither docs file is a permission token.
6. **All three** T24 technical CI jobs: Ubuntu verify + Windows verify + independent AVIF oracle **SUCCESS**, attempt and full job IDs recorded. A red run is a STOP, even if caused by a newly discovered old test assumption; repair forward under a new authorization if outside ledger.
7. R24 report-only main handoff plus a **fresh** two-host main run; main's architecture may measure differently because it is a report-only branch. Report clean local checkout/branch/origin/WIP/remote-head state honestly, explain discrepancies, retain R15 failed history and all statuses.

**R16 `status=COMPLETE` can mean only:** the previous R15 product source installation now has its two-host N127 three-job proof; ALL Foundation source artwork is inventoried with truthful delivery/legal classification; the Owner's global-Platform-baseline/supersession target and risk-based verification policy are formally recorded. It DOES NOT mean all Foundation artwork has been installed, the new graphic cascade is implemented, publication is approved, `R03.C` is closed, or Stage 03 is complete.

## 9. STOP and reporting boundaries

Use `status=STOPPED` whenever source evidence is missing, inventory cannot be reconciled, two-host CI fails, the revised test helper weakens an essential isolation condition, or implementation would require new source/runtime changes. No invented artwork, licence, permission, identity, external account, fallback, release readiness or stage closure. Source rights and project directives are distinct from a legally validated right to use every third-party mark. Preserve error diagnostics path-free and private source metadata non-public. The technical branch still does **not** authorize real publication/derived output.

The R24 canonical report must state one of these bounded outcomes, not infer global completion:

```
status=COMPLETE | STOPPED
r15_two_host_n127_unblocked=<true|false>
owner_81_assets_approved=true
platform_library_source_installed=<verified-bool>
platform_source_read_verified=<verified-bool>
foundation_donor_asset_inventory_paths=<actual>/142
foundation_canonical_assets_accounted=<actual>/101
additional_assets_delivered=false
graphic_baseline_target_owner_approved=true
graphic_baseline_cascade_implemented=false
publication_authority=false
derived_output_publication=false
release_authority=false
r03c_open=true
stage03_open=true
```

Follow the established one-checkout, staged-ledger, original test gates, authentic/mutated-log, fresh-main-CI and report-only handoff discipline. End with the shortest Owner action needed for the next iteration—do not invent a third-party rights approval because a file was visible in a public donor repository.
