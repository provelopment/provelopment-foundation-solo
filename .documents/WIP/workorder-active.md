# Foundation Solo — Active Work Order

**Work Order ID:** `SOLO-WO-009`  
**Edition:** `R02`  
**Title:** R00.B corrective continuation — locale-less Page entry and documentation parity  
**Approval marker:** `OWNER-ISSUED` **only upon exact Owner installation and separately delivered matching handoff**  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Branch:** `main`  
**Required starting `origin/main` HEAD:** `adaeca72be93ea612a55a99943e90b434f56c83b`  
**Matching handoff:** `SOLO-WO-009 R02 — Implementation Agent Handoff`  
**Preceding disposition:** Chief Architect `REVISE` for `SOLO-WO-009 R01`, reviewed at `adaeca72be93ea612a55a99943e90b434f56c83b`  
**Work classification:** Narrow corrective documentation continuation; no application implementation  
**Quality:** A1 documentary contract fidelity and GitHub-verifiable evidence  
**Final authority:** Independent Chief Architect `ACCEPT / REVISE / STOP`; Owner controls closure and further issuance.

> **Issuance boundary.** This complete R02 replacement becomes executable only after exact Owner installation into `.documents/WIP/workorder-active.md` and a separately delivered, matching `SOLO-WO-009 R02 — Implementation Agent Handoff`. It supersedes the live R01 authorization for this correction; it does not create a second concurrent work order. A draft, a report, ROADMAP, NEXT or the previous R01 handoff is not execution authority.

**Frozen Approved Scope:** §§1–8. The Implementation Agent must not add scope, choose new product behavior, expand permitted paths or accept its own work.

## 1. Objective and exact correction findings

Correct **only** the two outstanding substantive documentation findings from the Chief Architect's R01 review:

**C1 — Incomplete B03 canonical contract.** The Architecture Charter records locale preference precedence and mentions locale-less entry claims for collision validation, but does not clearly state the Owner-approved capability for **optional explicitly authored locale-less deep-link entry routes**. Make the accepted behavior normative and unambiguous, with positive and refusal examples.

**C2 — Stale open-decision wording.** The Product Definition §4A and README's configurable-base-URL paragraph still imply that normalization, aliases, redirect semantics and route/collision behavior remain undecided. Correct those descriptions to distinguish accepted B01/B04/B05/B08/B09 behavior from genuinely pending schema, toolchain and implementation details.

**C3 — Historical R01 publication deviation.** R01 published two content commits instead of the one authorized content commit. GitHub confirms:

- `df84dfd4e83d7984aa0c3f06497aa6d8f8e3831c` — seven authorized documentation paths.
- `208e03d7c506be526a2492dd80d626107111b100` — additional B11 refusal-semantics text in the authorized Architecture path.

This remains a disclosed procedural deviation. **Do not amend, squash, rebase, reset, erase, reclassify or retroactively self-approve it.** R02 authorization permits correction of C1 and C2; it does not automatically waive C3. Preserve an accurate record so the Owner and Chief Architect can explicitly consider it at final disposition.

No other R00.B decision is reopened.

## 2. Binding corrective contract — C1 / B03

In `.documents/Architecture/ARCHITECTURE.md`, add precise normative text to §3.2 and/or §4.3. Integrate it with the existing route, collision, visibility and localization contracts rather than introducing a second authority.

Required meaning:

1. **Required root entry.** The configured canonical base entry supports the already accepted locale-less selection policy: supported saved preference → supported mapped `Accept-Language` → applicable authored full-locale default. This behavior is not changed.

2. **Optional explicit deep links.** The author may additionally declare locale-less entry routes for individual Pages, such as `/about`, relative to the configured base prefix. These are optional, public-facing entry addresses, **not** required entries for every Page and **not** full-locale canonical representation URLs.

3. **Page identity first.** Each declared locale-less entry route identifies exactly one stable language-neutral Page, including its actual Entity/Facet/Site parent scope, **before** language selection. The system must not infer Page ownership from URL spelling, translation, Site name, physical location or filesystem position.

4. **Real destination second.** After selecting a supported locale, the entry route redirects to the selected Page's **actual authored, published, public full-locale canonical route**. Optional localized slugs may differ; the system must never construct a destination by blindly inserting a locale prefix, translating a word or guessing a matching slug.

5. **Missing representation.** If that Page has no representation for the selected locale, apply only its already accepted B07 same-Page, public-only fallback policy. A Page opting for strict/not-found remains strict. Never substitute a different Page or Site.

6. **Visibility.** A locale-less entry may not expose or automatically advertise an effectively unlisted representation. It cannot select an unlisted fallback or invent a public representation. If no authorized public destination exists, return not-found.

7. **Case and Unicode.** The entry route is an explicitly authored route claim subject to the accepted B04/B05 Unicode normalization, declaration-level case policy where applicable, collision checks and safe encoding rules. The Implementation Agent must not invent a new universal case rule for locale-less entries.

8. **Collisions.** Every locale-less entry route is included in the complete prepublication route-claim ledger. Refuse any ambiguity between entry routes, canonical localized routes, other aliases, and public/unlisted claims. Neither declaration order nor a convenient first match can choose a destination.

9. **HTTP and caching.** Locale-less entry selection uses the accepted B08 temporary redirect behavior, ordinarily `302` for appropriate safe navigation requests, and B12 protection against shared caching of preference-dependent responses. Redirect targets derive only from validated routing authority.

10. **Independence.** Adding a locale-less entry does not create an Entity, Facet, Site, Page, translation or new content-source authority; EFS identities and the versioned publication manifest remain decisive.

**Positive conceptual example:**

A Page with stable ID `about` within one declared Site has public representations:

- `en-001` at `/en-001/about`
- `de-DE` at `/de-de/ueber-uns`

The author explicitly declares the locale-less entry `/about` for that same Page.

A visitor requesting `/about` with a supported saved preference `de-DE` receives a temporary redirect to `/de-de/ueber-uns`. The destination is looked up from the actual Page declaration; no translated slug is invented. A visitor with `en-001` preference receives `/en-001/about`.

The same visitor requesting `/en-001/about` directly stays on English regardless of saved preference, browser preference or physical location.

**Refusal conceptual examples:**

- Two different Pages claim the same effective locale-less `/about` entry: refuse publication.
- A locale-less entry claim overlaps another route or alias under its effective matching rules: refuse publication.
- A declaration attempts to identify a Page by guessing its localized slug rather than explicit identity: refuse publication.
- The selected Page has only an unlisted representation in the chosen locale and no authorized public fallback: not-found; do not reveal the unlisted route.
- A locale-less entry resolves a Page in a different Facet or Site because another Page has the same ID: refuse/diagnose the invalid declaration; no sibling selection.

Label examples as **conceptual specification cases, not executed tests**.

Do not select concrete manifest field placement or physical encoding; those remain R00.D.

## 3. Binding wording correction — C2

### A. Product Definition

In `.documents/About/FOUNDATION-SOLO.md` §4A, replace the stale statement that precise normalization, aliases, redirects and public route grammar remain separately designed.

The corrected wording must accurately state:

- **Accepted behavior:** B01 standards-based safe base URL normalization; B04 validated Unicode public routes; B05 default-insensitive and explicitly sensitive localized claims with strict collisions; B08 condition-appropriate redirects; B09 one canonical base with optional explicitly authorized aliases.
- **Accepted address independence:** Configured origin/prefix changes do not alter EFS identity or require router redesign.
- **Still open:** Physical declaration and manifest schema/field names (R00.D), pinned Unicode data version and toolchain choices (R00.E), concrete runtime matcher, HTTP adapters, DNS/TLS/provider wiring and deployment operations.
- **No broad claim:** Do not say all normalization or routing mechanics are fully implemented, tested or deployed.

Keep the paragraph concise. Do not repeat the entire Architecture Charter's technical algorithm.

### B. README

Replace the last stale sentence in README's configurable-base-URL paragraph, which currently characterizes exact normalization, aliases and route-collision rules as open decisions.

A concise suitable replacement is:

> **R00.B now accepts** safe standards-based base-address normalization, explicitly authorized optional aliases, validated Unicode localized route claims, author-selected public case sensitivity and fail-closed collision rules. The physical schema, pinned Unicode/toolchain implementation and deployment/provider wiring remain later work; no router or deployment is yet implemented.

Match the repository's established terminology and preserve the existing surrounding explanation about configuration-only base-URL changes.

Do not update unrelated README content.

## 4. Exact authorized path and operation ledger

| Path | Permitted operation |
|---|---|
| `.documents/WIP/workorder-active.md` | Owner installs the full R02 order. Agent may commit/publish those installed bytes **unchanged** in Phase A only. No further edit. |
| `.documents/Architecture/ARCHITECTURE.md` | C1 only: integrate explicit B03 locale-less Page entry semantics and conceptual positive/refusal cases; only necessary surrounding text for consistency. |
| `.documents/About/FOUNDATION-SOLO.md` | C2 only: correct §4A's stale accepted-versus-open classification. |
| `README.md` | C2 only: correct the stale base-address sentence. |
| `.documents/Reports/execution-report.md` | Overwrite the single current report with complete factual R02 evidence in Phase C only. |
| `.git/**` | Minimal normal metadata for explicitly authorized ordinary commits and pushes, governed exclusively by Architect SOP §6B. |

**All other permanent paths are read-only**, including `AGENTS.md`, both SOPs, `.documents/WIP/plan-roadmap.md`, `.documents/WIP/plan-next.md`, `.documents/WIP/plan-outlook.md`, LICENSE, `.github/**`, `.gitignore`, `.gitattributes`, all other repository files and all other repositories.

The existing NEXT candidate still contains historical universal public-case wording. That is a **known deferred Owner-controlled WIP reconciliation item**, not permission to edit NEXT during R02. Its replacement belongs to the later separately authorized R00.B closure transition.

Do not create additional WIP slots, report archives, architecture decision files, code, tests, dependencies or provider configuration.

## 5. Baseline and preflight

**Exact required starting GitHub `main` HEAD:**

`adaeca72be93ea612a55a99943e90b434f56c83b`

Verify the installed R02 order, direct matching Owner handoff and repository state under the current Architect and Implementation SOPs before any write.

Relevant required baseline Git blobs:

| Path | Git blob SHA-1 |
|---|---|
| `.documents/Architecture/ARCHITECTURE.md` | `168e858db31246927705fce038ebb4ad0b77b545` |
| `.documents/About/FOUNDATION-SOLO.md` | `1ac9652d484473e9e15b8110458b5ba3b88d8e75` |
| `README.md` | `9f1c9872288b4ec21340cd8b20241fd8caa7dddd` |
| `.documents/WIP/workorder-active.md` before Owner R02 installation | `e1751601832916898afdd15085ef76c652de7621` |
| `.documents/Reports/execution-report.md` | `4724b807185a3fc635164f6d2a75bc6c5a4d85cf` |
| `.documents/WIP/plan-roadmap.md` | `b3aa0e52ad3383256386a938408905536bc090bb` |
| `.documents/WIP/plan-next.md` | `375aee17fc9f8ae5355c119394fb8dca030cab23` |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `AGENTS.md` | `d4a2a028e0d35852e0077bbe97c162bdfde89d2d` |
| `.documents/SOP/sop-architect.md` | `0396251245180b40ee896b50a3456b92835f9cb1` |
| `.documents/SOP/sop-implementation.md` | `b868faccbe73e076d207ed1c532069559ca3590a` |

The expected initial authorized difference from the R01 baseline is **only the exact Owner-installed R02 replacement of ACTIVE**. Unexpected permanent changes, baseline mismatch or missing authority require STOP rather than guessed reconciliation.

Preserve all preexisting Owner material. Follow the established single-checkout, scoped-write, safe temporary-workspace, cleanup, remote verification and version-control procedures. This order does not create another Git procedure.

## 6. Authorized publication sequence

**Phase A — Publish the exact issued R02 order.**

Verify the Owner-installed order and matching handoff. Stage and commit only `.documents/WIP/workorder-active.md`, using the exact installed bytes. Make one ordinary commit and an authorized fast-forward push to verified `origin/main`. Verify GitHub readback before continuing.

**Phase B — Publish one complete corrective content commit.**

Edit **only**:

- `.documents/Architecture/ARCHITECTURE.md`
- `.documents/About/FOUNDATION-SOLO.md`
- `README.md`

Before committing, perform a complete document review against §§2–3, including all required positive/refusal examples, unchanged accepted B01–B12 policies, accurate accepted-versus-open classification and absence of unrelated edits.

Stage exactly the three corrected files, make **one ordinary content commit**, fast-forward push and verify GitHub readback.

Do not split Phase B into multiple commits. Do not publish before completing the scope check. If a new omission or blocker is discovered **after** Phase B publication, STOP and report it rather than creating an unauthorized additional content commit.

**Phase C — Publish the canonical report.**

Overwrite only `.documents/Reports/execution-report.md`. Record accurate authority, scope, baseline, commit hashes, GitHub-visible content, conceptual contract checks, any deviation, actual checks, unaffected WIP, and the retained historical R01 C3 deviation.

Make **one separate ordinary report commit**, fast-forward push and verify its GitHub readback. Do not place the report commit's own SHA inside the report; provide it in the handback.

No additional content commit, branch, PR, merge, tag, release, deployment, dependency installation, external account/provider action, destructive Git operation, history rewrite, second checkout or other-repository change is authorized.

If a phase cannot safely complete, STOP, preserve the last valid state and report the actual result under existing SOP permissions. Never invent completion evidence.

## 7. R02 A1 acceptance gates

The Agent records factual results; the Chief Architect independently determines disposition.

| Gate | Required proof |
|---|---|
| R02-1 — Authority | Exact installed R02 and matching Owner handoff; required starting revision and document blobs verified |
| R02-2 — Path scope | Exactly three corrective content files changed; ACTIVE only installed/published unchanged; report only overwritten separately |
| R02-3 — Complete B03 | Optional locale-less Page entry routes explicitly authorized and defined independently of root entry |
| R02-4 — Page identity | Each locale-less route identifies one stable Page before language negotiation; no inferred translated slug or sibling fallback |
| R02-5 — Positive and refusal cases | Preference-selected actual localized destination, direct explicit-locale precedence, ambiguous-route refusal and unlisted non-disclosure |
| R02-6 — Product parity | §4A correctly distinguishes accepted B01/B04/B05/B08/B09 behavior from outstanding encoding/implementation |
| R02-7 — README parity | No remaining claim that accepted normalization, aliases or collisions are undecided |
| R02-8 — Prior contracts preserved | B01–B12, R00.A, case-sensitive Unicode routes, strict unlisted policy and Hexagonal boundaries unchanged |
| R02-9 — WIP preservation | ROADMAP, NEXT and OUTLOOK unchanged; ACTIVE remains issued R02; exactly four WIP files and one report |
| R02-10 — Publication discipline | Exact Phase A order commit, **one** Phase B three-file correction commit, **one** Phase C report commit; GitHub verified |
| R02-11 — Honest reporting | C3 disclosed without rewriting or self-waiver; conceptual checks not described as executed software tests |
| R02-12 — Governance | No unauthorized changes, self-acceptance, R00.B closure or new package issuance |

Document checks may be marked `PASS`, `FAIL` or `UNVERIFIED`, with source evidence. Truly inapplicable gates require `NOT APPLICABLE` and a reason.

Application build, unit, HTTP, browser, architecture-boundary, security, release and deployment testing are **NOT RUN**; no implementation exists and none is authorized here.

## 8. STOP and completion boundary

STOP immediately if any of these occur:

- Installed work order or handoff does not match `SOLO-WO-009 R02`.
- `main` has moved from the approved starting revision.
- An unanticipated permanent modification or unrelated material threatens the allowed scope.
- C1 requires inventing a new route policy or contradicting accepted B02/B03/B05/B07.
- C2 requires a substantive architectural change rather than correcting classification wording.
- An authorized fix requires modifying an unlisted path, including NEXT or ROADMAP.
- A second Phase B content commit appears necessary.
- The Agent is asked to retrospectively waive C3, self-accept R01 or R02, remove R00.B from ROADMAP or begin R00.C.
- Any permission, security or repository-safety condition fails.

On successful execution:

1. The C1 and C2 documentary defects are corrected.
2. The historical R01 C3 deviation remains accurately reported and unaltered.
3. ACTIVE contains exact issued `SOLO-WO-009 R02`.
4. ROADMAP still lists R00.B; NEXT remains its existing candidate; OUTLOOK remains unchanged.
5. The single current report is published to GitHub.
6. No application implementation or release operation is claimed.
7. The Agent stops for independent Chief Architect review.

**Independent disposition remains outstanding until the Chief Architect reviews the exact published revision.** Closure, R01 deviation adjudication and progression to R00.C require separate Owner-controlled actions.
