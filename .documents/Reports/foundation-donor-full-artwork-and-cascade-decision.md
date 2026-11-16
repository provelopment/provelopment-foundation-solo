# Foundation donor — complete artwork accounting and Platform-first cascade decision (R16)

**Status: source accounting and normative-target record only.** This document is not an approval, a product catalogue, a licence or a permission to publish. It accompanies `.documents/Reports/foundation-donor-full-artwork-inventory.json` (142 paths, 184 479 bytes, SHA-256 `6d6199cf63f486a95c38395b449f2f68363d6da45cd1f18c49212d391b6a903e`).

## 1. Count reconciliation against the pinned donor

| Measure | Value | How it was measured |
|---|---|---|
| Donor commit | `fa599973a88ff77f6a8e012f3266c3c139b52c17` (2026-10-29T06:43:45Z) | commit API; tree `89b57a92869d2e293d33527589f3f4680938f9b2` |
| Media files matching `.svg/.png/.jpg/.jpeg/.webp/.gif/.avif/.ico` | **142** | full recursive tree, `truncated=false` |
| Canonical Foundation sources | **101** | 84 icon-library + 10 role placeholders + 7 platform marks |
| Mirrors and test fixtures | **41** | 4 EFS capsule + 10 German spoke + 27 test fixtures |
| Distinct Git blobs among the 142 | 125 | 13 duplicate-byte groups |
| Already installed and approved | **81** | byte-exact members of catalogue `1.0.0` |
| Canonical sources still outstanding | **20** | 3 icons + 10 placeholders + 7 platform marks |
| Raw SHA-256 computed for canonical sources | **101 / 101** | over the exact bytes fetched at the pinned commit |

No path is omitted and no count is inferred: the bucket totals are asserted, not assumed. The 81 approved catalogue IDs are preserved exactly — every one of their donor raw SHA-256 values equals the catalogue declaration, and exactly three donor icons are absent from the approved catalogue (the two byte-identical aliases and the one Provelopment original).

## 2. The 20 outstanding canonical sources

**Icons (3).** `icon-services.svg` (496 B, SHA-256 `ce90da20…`) is Provelopment original artwork with **no third-party licence recorded**, so no licence may be inferred from its presence in a public repository. `icon-status-info.svg` (439 B, `bf4377df…`) and `icon-volunteer.svg` (610 B, `944697c4…`) are **byte-identical aliases** of the approved `icon-about.svg` and `icon-clients.svg`; the approved single representative of each pair is already installed. Admitting them needs a governed alias model, not new artwork.

**Role placeholders (10).** `favicon.svg`, `header-graphic.svg`, `footer-graphic.svg`, `logo-header.svg`, `logo-footer.svg`, `logo-title.jpg`, `sidebar-open.svg`, `sidebar-close.svg`, `sidebar-default-icon-open.svg`, `sidebar-default-icon-closed.svg`. They are neutral template defaults, **not** brand art: the donor README calls them "the neutral, blank defaults a fresh Foundation site ships… replace these in place". No per-file licence record exists for them, so their state is `review-required`, and they must not be classified as approved brand identity or wired to arbitrary logo/favicon roles.

**Platform marks (7).** `facebook.png` (54 771 B), `github.svg`, `instagram.svg`, `linkedin.png` (8 375 B), `messenger.svg`, `telegram.svg`, `whatsapp.svg`. Their donor record is explicit: **"PARKED — NOT INTEGRATED"**, "Nothing in this package is referenced by, copied into, or depended upon by the Foundation runtime", and "Approval status: **Not approved.** This file records the published basis relied upon; it does **not** assert legal certainty, trademark clearance, or brand-owner approval". Each mark carries its owner's published use basis and four cross-cutting preconditions:

| Precondition | Requirement before any mark ships |
|---|---|
| **P-1** | Monochrome marks (`github.svg`, `instagram.svg`) are the owners' **black** variants: either render on a fixed light surface, or swap the official white variant for a dark surface. Never recolour or filter the owner's file. |
| **P-2** | Facebook's clear space: raise the icon→label gap to ≥ 0.5 em if the Facebook mark ships (a shared-CSS decision, deliberately not made in the parking pass). |
| **P-3** | Messenger's page publishes digital rendering rules but not an explicit "link to your presence" sentence: Meta confirmation remains open. |
| **P-4** | No accounts, handles, phone numbers or profiles may be invented; a mark may only be wired to a real, existing presence (and LinkedIn's condition requires one). |

## 3. Duplicate identities and mirrors

Thirteen donor blobs appear at more than one path. They are not independent assets:

| Blob | Paths | Meaning |
|---|---|---|
| `c407077e4f84` | `icon-library/icons/icon-about.svg` = `icon-status-info.svg` | approved + withheld alias |
| `b0d6ecc811b7` | `icon-library/icons/icon-clients.svg` = `icon-volunteer.svg` | approved + withheld alias |
| `4a363d207e8e`, `c26655731e11`, `5ccfb019fa23`, `84ef006ff90c`, `892d0e3679e5`, `729936e0f022`, `acef1df1aac4`, `46dd2a1a499d` | each German spoke placeholder = the Foundation placeholder of the same name | deployment mirror |
| `4f8cd00cc563` | EFS capsule 01 `foundation` favicon = capsule 02 `germany` favicon = German spoke `favicon.svg` | three copies of one 338-byte file |
| `972ca9b6595d` | EFS capsule 01 `foundation` logo = capsule 02 `germany` logo = German spoke `logo-header.svg` | three copies of one 364-byte file |
| `49eed7b0e00a` | `tests/fixtures/efs/.../acme/assets/brand/acme-main.svg` = the neutral fixture logo | test fixture reuse |

The four `deployment/efs/**` files are **Entity-owned demonstration identity** (a sample capsule's own `assets/brand/`). They are recorded, not promoted: Solo must not ship them as generic Platform branding, and the mirror copies are never a second delivery.

## 4. Rights and delivery classification

| Class | Count | Rights state | Delivery intent |
|---|---:|---|---|
| Approved neutral icons | 81 | `owner-approved-exact-81` | already installed at `platform/assets/icons/<name>`; catalogue `1.0.0` unchanged |
| Unapproved icon sources | 3 | `review-required` | separate exact-version-pinned extension plus an alias model; **not installed** |
| Role placeholders | 10 | `review-required` | candidate `platform/assets/placeholders/` after role and licence review; **not installed** |
| Platform marks | 7 | `third-party-conditional` | candidate `platform/assets/platform-marks/` only after an Owner rights decision and P-1…P-4 resolution; **not installed**, never a generic role default |
| EFS demonstration mirrors | 4 | `mirror-non-authoritative` | no product delivery |
| German spoke mirrors | 10 | `mirror-non-authoritative` | no product delivery; duplicate of the canonical placeholders |
| Test fixtures | 27 | `test-only` | no product delivery; never a public runtime library |

**Attribution and runtime obligations.** The 81 installed icons travel with the Tabler MIT notice and the provenance record (`platform/assets/licensing/`), which must accompany any separately authorized distribution; the Foundation root Apache-2.0 licence does not replace that notice and grants no rights in names, logos or trademarks. Any future placeholder or mark delivery inherits the obligation to keep its own notice/provenance beside the bytes and to record the mark owner's published use basis. Passing Rust tests supplies no trademark right.

**Source documents actually inspected at the pinned commit:** `BRAND_ASSETS.md` (root), `deployment/spokes/foundation/content/assets/README.md`, `.../icon-library/icons/README.md`, `.../icon-library/licensing/icon-provenance.json`, `.../icon-library/licensing/TABLER-ICONS-MIT.txt`, `.../platform-marks/platform-marks-provenance.md`. The platform-marks record is quoted only as evidence of its **parked, not-approved** status and its P-1…P-4 preconditions; its historical wording is not legal clearance.

## 5. Owner target: the Platform-first graphic cascade (recorded, not implemented)

`.documents/Architecture/ARCHITECTURE.md` §6.2I.1 now records the Owner's future target: the version-pinned Platform library is the **root baseline** for graphic role/slot defaults; a closer declaration that explicitly **supersedes** it wins; suppression and bad input are final and never resurrect a default; collections may append, supersede one uniquely identified slot, or suppress and repopulate; and no automatic default is activated before a separately gated migration proves the precedence cases on both native hosts. That amendment is explicitly labelled **FUTURE TARGET, NOT IMPLEMENTED**, and the existing additive `property_resolution` behaviour is unchanged by this order.

**Three questions this record deliberately leaves open for the migration order** (they change meaning, so they may not be guessed here):

1. **Registration schema.** A role-keyed default table beside the catalogue, or per-role default fields inside a new catalogue version? The former keeps `1.0.0` immutable; the latter forces a version bump.
2. **Slot grammar for collection supersession.** Whether a slot is identified by a stable authored key, by an explicit ordinal supplied in the document, or by a typed role-plus-variant pair; "first/last/background" inference is forbidden either way.
3. **Unregistered roles.** Whether an entirely unregistered role inherits the Platform baseline or remains absent. The Owner's directive points at root-baseline-by-default, but making that automatic is exactly the high-consequence change the migration gate exists to bound.


## 6. Risk-proportionate verification policy (normative for the next orders)

| Change category | Risk (likelihood × impact) | Minimum proof | Prohibited |
|---|---|---|---|
| Byte-for-byte donor-art import inside an already accepted class, no resolver or decoder change | Lower behavioural impact | complete file/count/raw SHA-256/case/size/provenance validation; MIME and geometric bounds; one cheap representative type-specific inspection; reuse existing per-asset sanitizer proofs where raw digests match | a fresh 81× adversarial matrix merely because bytes were copied; skipping licence/role evidence |
| New external mark or role placeholder | Moderate, legal and UI | recorded source, provenance and published use conditions; authorized purpose; relevant size/clear-space/contrast conditions; versioned catalogue impact; refusal of disallowed use | treating a green Rust suite as a trademark right; recolouring an owner's file; inventing an account |
| Alias/index/metadata schema, catalogue version or immutable package assembly | Moderate to high integrity | exact membership; no digest aliases unless separately governed; negative tamper/missing/extra-version checks; deterministic packaging; cross-host where filesystem semantics differ | mutating `1.0.0` in place; silent alias collapse |
| Platform root acquisition, isolation, cross-namespace resolution, and the future five-level graphic override/suppression/visibility cascade | **High** confidentiality and integrity | focused adversarial native Ubuntu **and** Windows tests: duplicate, conflict, unsafe, suppressed, internal, unlisted, wrong-slot and late failure; fail-closed whole candidate; retained AVIF oracle and mutation gates | skipping native proofs for speed; weakening a confinement canary |
| Publication, public URL, derived output, release | High external effect | its own future Owner authorization, privacy/security proof and immutable-output contract | any claim in R16 |

Verification effort scales with **new risk and consequence**, not with the number of old filenames. Existing N093–N127 bodies and their authentic/mutation CI gates stay intact except the two obsolete absence assertions repaired in this order; no gate was waived, skipped or made advisory, and no `#[ignore]`, `continue-on-error` or output spoofing was introduced.

## 7. Single sequenced next implementation proposal

One sequence, not four parallel work streams:

**Step 1 — rights and role support for the 20 outstanding sources.** (a) An Owner rights decision naming, per source, whether Solo may redistribute the bytes: `icon-services.svg` needs an explicit Owner licence statement (or stays excluded); the two aliases need an approved **alias model** (one approved byte-identity per catalogue ID, with a declared duplicate rule that the R12 parser currently refuses); the ten placeholders need a role/licence decision and a declared role mapping; the seven marks need a third-party decision that resolves P-1…P-4. (b) Only then a **new** catalogue version (`1.1.0`, never an in-place mutation of `1.0.0`) with exact pins, plus — if the alias model is approved — the smallest parser change that admits a governed alias without weakening the duplicate-digest refusal. (c) Category safety: placeholders and any approved original artwork may be installed once their licence state is recorded; platform marks should stay out until their named rights decision exists, because importing unapproved trademark artwork into a public repository may itself be redistribution.

**Step 2 — the governed Platform default and closer supersession.** (a) Settle the three open questions in §5 and record the answers as the normative amendment replacing §6.2I.1's open items. (b) Implement the typed default registration (exact role/slot + approved catalogue ID), the closed `supersede/replace` operation, and final suppression, without touching location, SEO or contact inheritance. (c) Prove it with a dedicated two-host suite covering root default, closer override, suppress, invalid closer refusal, wrong role/slot, same-owner conflict, mixed owners, internal/unlisted obligations and no unauthorized fallback — and only then enable the automatic baseline.

**Step 3 — delivery surface.** Any public projection, derived canonical asset, route or release remains a separate Owner-authorized order with its own privacy and immutable-output proof.

**Owner action needed now:** the per-source rights decision for the 20 outstanding files (or an explicit "placeholders and aliases only, marks deferred"), so Step 1 can be scoped exactly.

