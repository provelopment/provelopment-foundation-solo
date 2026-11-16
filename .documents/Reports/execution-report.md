# Execution Report

**Work order:** `SOLO-WO-118 R19` — one unpublished, privacy-screened, immutable graphic delivery-preparation package built from the accepted mixed Entity/Platform candidate.
**Bounded outcome:** `R03C-MIXED-GRAPHIC-PRIVACY-SAFE-IMMUTABLE-UNPUBLISHED-DELIVERY-PACKAGE-N130-TWO-HOST-V1`.

```
unpublished_graphic_package_prepared=true
public_cascade_delivery=false
publication_authority=false
public_route_authority=false
derived_output_publication=false
release_authority=false
automatic_product_graphic_defaults_active=false
additional_assets_delivered=false
stage03_closed=false
r03c_open=true
stage03_open=true
```

**Status: `COMPLETE` for this bounded outcome only.** One closed preparation step now turns an accepted whole-candidate transaction into a deterministic, content-addressed, privacy-screened package whose payloads are the exact bytes that passed inspection — on native Ubuntu and native Windows. The package is **in memory**, unpublished and inert: it creates no file, route, URL, derived asset, release or deployment, and it cannot overwrite a product file. One real identity gap was found by the first native run and repaired forward; it is reported below rather than hidden. Nothing was merged, released, deployed or published, no production default was registered, and the 20 pending Foundation sources remain undelivered.

## 1. Transactions, SHAs and changed paths

| Step | SHA | Validation |
|---|---|---|
| baseline | main R26 `e37c032ae487148f530bfb3356d16438bc80c7ac`; technical T26.1 `8015a840a6c2c0c55e993f60da28f3931f3706bc` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four WIP files, empty stash, clean tracked trees, **30** remote heads observed (unchanged, including the two unrelated Dependabot app heads) |
| **A27** (main, ACTIVE only) | `e02562fc78c81ab8fe23d4cf50e2a040da719c35`, tree `65189458d3fed5ae1fa31e17126dfd508f48e4c8`, parent `e37c032a` | ACTIVE blob `e63a10e37649b8d4b18d2c91477e862d1457a563` identical in worktree, index, commit and `origin/main` |
| **T27.0** (branch, ACTIVE only) | `c38413868a01e972c97e29dc100186a64b1e2b6b`, parent `8015a840` | the same blob; `git diff T26.1..T27.0` = ACTIVE only |
| **T27** (branch, first implementation) | `8e88529501b390964f96fccafeb4d5873f6dcd06`, tree `5a77ffce4636f724d449792005f2277fc2995f76`, parent `c3841386` | **failed its technical run honestly** (§3) |
| **T27.1** (branch, forward repair — the evidence SHA) | `07b2052a48d237959b1f79ce6c14fc91b45dc769`, tree `869105c29e606bb151a8dae4bd0708e5dc314335`, parent `8e885295` | all three technical jobs green (§4) |
| **R27** (main, report only) | this commit (SHA in the Owner handoff) | parent `e02562fc`, exactly one path |

Changed paths across `T27.0..T27.1`, all inside the ledger (`git diff --numstat`):

| Path | Δ | Role |
|---|---|---|
| `crates/solo-application/src/graphic_delivery_preparation.rs` | **+1038 / −0 (new)** | the closed unpublished package: eligibility case, bounded limits, closed refusal set, selection, exact output materialisation, derived content keys, dedup, quarantine-free all-or-nothing assembly, and the separate privacy-safe outward view |
| `crates/solo-application/src/graphic_image_inspection.rs` | +43 / −15 | the narrow in-flight staging handoff: the accepted mixed driver became a thin public wrapper over a crate-private core that also returns the exact acquired bytes in plan order; every previous refusal arm and check is unchanged |
| `crates/solo-application/src/lib.rs` | +10 / −0 | module and exports |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | +367 / −35 | one composed outward entry over the two adopted roots plus the closed host-test package seam; the R18 seam was split into a reusable builder with **no behavioural change** to `test_mixed_candidate_v1` |
| `crates/solo-adapters/src/lib.rs` | +4 / −0 | exports |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | +982 / **−0** | **additive** `mod n130` only; zero deletions, so no N093–N129 body changed |
| `.github/workflows/rust-verify.yml` | +115 / **−0** | additive Linux and Windows N130 gates |
| `.documents/Architecture/ARCHITECTURE.md` | +12 / **−0** | §6.2I.3 truth note (preparation scope and authority only) |

`git diff --name-only 8015a840..07b2052a -- platform .gitattributes Cargo.toml Cargo.lock deny.toml crates/solo-domain/src/property_registry.rs crates/solo-domain/src/property_resolution.rs crates/solo-application/src/property_registry.rs crates/solo-application/src/graphic_asset_inventory.rs crates/solo-application/src/media_worker.rs` is **empty**. The immutable `1.0.0` catalogue (`ad8ea718…`, 81 entries, 84 product files on the technical branch), the separate Platform reader, the Entity reader's `PlatformUnverified`, the R12 review-only `PlatformSourceUnavailable`, the R17 cascade, the decoder, the sanitizer, the worker and supervisor, the AVIF oracle, the licence policy and the donor fixtures are byte-unchanged.

**Architecture, measured:** `architecture=legal packages=4 targets=39 files=77 edges=93` on both hosts — one new file (the preparation module) and no new target or package dependency edge, so the accepted crate graph is unchanged.

## 2. What the package is, and what it deliberately is not

- **One closed input.** A package is built only from an already validated `GraphicCascadeInputV1`, the closed mixed plan of *exactly* that candidate, and a successful whole-candidate acquisition and six-format inspection. The plan is recomputed from the candidate and compared, so a plan belonging to another candidate is refused **before any source is touched** (`plan-does-not-match-input`). Every authored declaration — `internal`, suppressed, superseded and unrelated — still had to pass inspection before a single record exists.
- **Eligibility, not ordering.** The package iterates the resolved roles of one explicitly scoped, fully qualified representation owner and takes only its winning `Exposure::Public` slots, then re-checks that each one is present in the accepted R17 public projection (otherwise `private-leakage`). A winning `internal` value contributes nothing and never resurrects a Platform root default; a superseded value cannot resurface.
- **Exact bytes, no second source path.** An accepted vector source is materialised as its sanitized **canonical SVG text** and the recomputed digest must equal the accepted vector output digest; every accepted raster source is a **byte-for-byte pass-through** of the very bytes that passed inspection, taken from the same in-flight transfer through a crate-private handoff. No source is re-opened by path; there is no second reader, decoder, sanitizer, encoder or rasteriser, and nothing is re-encoded, resized or recoloured.
- **Derived identity only.** Each payload gets `sha256/<64 lowercase hex>.<closed-set extension>` derived from its own rechecked bytes, validated against a closed grammar (ASCII, ≤96 bytes, no `..`, no scheme, no absolute or host path, lowercase hex equal to the recorded output digest). A key is an identity, never a URL, host origin, relative path or caller-chosen name. Truly identical final bytes and MIME may share one payload while each slot keeps its own validated provenance; nothing else coalesces.
- **All-or-nothing and bounded.** Entry count, total payload bytes and key length are bounded; a request may tighten the limits but never exceed the documented hard maxima. Wrong stage, mismatched digest, ambiguous ownership, unsafe key, quota, invalid format, private leakage, unapproved Platform source, collision or a failed post-staging recheck discards **every** record and payload: there is no partial package and no partial-success arm.
- **Inert by construction.** The payload store is in memory. This order created no output directory, no `public/`, `dist/`, `static/` or `generated/` tree, no route, no manifest for serving, no release bundle and no deployment. `publication_authority`, `routes_any_url`, `derived_output_publication` and `release_authority` are `false` on the outcome and on every view. Holding a package is a **prepublication** fact only; actual delivery requires a separate Owner-issued order and the later Stage 04 visibility/route authority.
- **Prerequisite, not proof.** The display-eligibility case (`public` / `unlisted` / `internal`) is explicitly named untrusted prepublication input with no publication capability, and it cannot drive production serving. Real representation visibility and routing remain a later gate.

## 3. One real defect found by the first native run, and repaired forward

**First technical run `43853803633` at T27 `8e885295` failed.** Ubuntu job `133252828912` (started 11:00:05, failed 11:01:52) and Windows job `133252829459` (failed 11:03:04) both failed inside `Test (workspace, all targets, locked)` (step 17) — *not* inside the N130 gate — while the independent oracle `133252829450` passed. Three of the four cases were green on both hosts; `n130_03_identity_substitution_and_immutable_collision` failed at its unapproved-Platform assertion.

**The real gap, not a test typo.** Case N130_03 registered the approved identifier `icons/icon-home.svg` with a digest that is *not* the approved asset digest. The accepted composition acquires the real catalogue-pinned bytes, and `approved_platform_graphic_binding_v1` rebuilds its binding from the catalogue itself — so nothing compared the **registration's own approved identity** with the bytes that were acquired, and the attempt reached the payload stage. That is exactly the identity boundary this order exists to close.

**Repair (T27.1).** Before any payload is derived, every winning root-layer Platform default must now be exactly the approved asset it claims: same identifier as the winning reference, its declared asset digest equal to the acquired source digest, the approved catalogue pin `ad8ea718…` and `platformAssetLibraryVersion` `1.0.0`. Any mismatch refuses the whole attempt with a new closed arm, `unapproved-platform-source`, and nothing is staged. The case now asserts that exact arm and also that the read itself succeeded (`source_refusal` is `None`), which distinguishes "the source was fine, the registration was not approved" from a read failure.

No check was weakened, no bound relaxed, no case skipped and no gate downgraded. The failed run stays in history.

## 4. Final technical proof — run `43856345609`, attempt 1, at `07b2052a`

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133261472669` | **success** (11:16:54 → 11:22:35) |
| `verify (windows-2025)` | `133261472522` | **success** (11:16:54 → 11:21:51) |
| `avif-reference-oracle` | `133261472204` | **success** (11:16:53 → 11:17:35) |

Every retained gate in that job was green on both hosts: format check, strict Clippy, the locked dependency fetch, the pinned `cargo-deny 0.20.2` gate with a fresh RustSec refresh, the `N124`/`N125`/`N126` six-format and AVIF mutation verdicts, the owner-approved library `N127` gate, the governed cascade `N128` gate, the mixed-candidate `N129` gate, the filesystem canaries, the worker isolation proofs, the release build, the byte-for-byte release reproducibility comparison and the tripwires. Concretely, on Ubuntu `N127` (step 37), `N128` (39), `N129` (41), **`N130` (43)** and the real-tree architecture check (48) all concluded `success`; on Windows the same five steps (38, 40, 42, **44**, 49) concluded `success`. The N127 aggregate pin is unchanged at `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`, and the architecture line is `packages=4 targets=39 files=77 edges=93` on both hosts.

**The N130 gate is fail-closed by construction.** Each host runs all four cases with `--nocapture` and `--test-threads=1`, then requires exactly `4 passed, 0 failed, 0 ignored`, exactly one `n130_evidence case=0X` marker per case, the exact case names, `svg_vector=true`, `canonical_sha_matched=true`, `content_addressed=true`, `internal_hidden=true`, `unlisted_discovery=false`, `late_refusal=whole`, `zero_partial=true`, `immutable=true`, `product_tree_unchanged=true`, `legacy_parity=true`, the catalogue pin, `publication=false`, `route=false`, at least four distinct 64-hex `output_sha=` values with none zeroed, and the absence of `FAILED`/`ignored` result lines. Only then does it print `n130_gate authentic=accepted`. It then re-checks **four altered copies of that same genuine log** — a deleted case, a forged privacy claim (`internal_hidden=false`, `unlisted_discovery=true`), a forged output digest (every `output_sha=` zeroed) and a forged authority/partial claim (`publication=true`, `zero_partial=false`) — and prints `n130_gate mutation=rejected` only if all four are rejected. Because the step exits non-zero on any failed requirement, the step's `success` on each host is the gate's certificate; the verdict lines themselves are inside the step log, which the Architect can read against this exact revision.

**N130 — four cases, genuinely executed on both native hosts.**

| Case | Required facts and refusals, as exercised |
|---|---|
| **N130_01** | One approved root Platform registration for `graphic.logo` plus an Entity PNG favicon, an Entity WebP social preview and a Facet-then-Site `background` chain. Four eligible winning slots, three deduplicated payloads. The Platform source stays a **vector**: MIME `image/svg+xml`, format `svg`, an actual `<svg … viewBox …>` text payload with no `<script` and no external `xlink:href="http`, the source digest equal to the approved catalogue digest, the **output digest different** from it, and `sha256(payload)` equal to the recorded output digest. Favicon and social preview are byte-for-byte pass-through (`source_sha256 == output_sha256`), and the nearer Site supersession — not the superseded Facet value — is the staged `background`. Every content key satisfied the closed grammar and derived from its own output digest; `publication=false`, `route=false`. |
| **N130_02** | A closer `internal` Entity logo supersedes the public Platform default: the role contributes **nothing**, the Platform default never resurfaces, and the superseded source digest appears in no staged provenance. The same fully valid candidate under `unlisted` and under `internal` display eligibility produced **zero** entries, zero payloads and zero bytes with `discoverable=false`, while still passing every real acquisition and inspection. A late invalid obligation (a WebP declared for the PNG-only favicon role at the closest owner) refused the **whole** candidate with zero records and zero payloads, and the refusal token it produced is printed in the evidence line. |
| **N130_03** | A plan taken from a **different** valid candidate is refused `plan-does-not-match-input` before any source is read (`source_refusal` is `None`, nothing staged). A `.png` name over WebP bytes is refused by the accepted inspector. An unapproved Platform registration is refused `unapproved-platform-source` (the repair of §3) with the read itself successful. Three slots whose final bytes are genuinely identical share **one** payload while keeping separate validated provenance, and different bytes or formats never coalesce. The tracked product tree (catalogue, approved icon, MIT notice) is byte-identical before and after, a payload written to a task-owned disposable directory round-trips exactly, and the payload digest re-verifies. |
| **N130_04** | The same fully validated input twice produced byte-identical entries, payloads and ordering (`first == second`). A tightened 64-byte total bound and a tightened two-entry bound each refused `quota-exceeded` with zero records; zero, over-maximum and above-ceiling limit requests were refused `invalid-limits` at construction. The accepted legacy envelope stayed exactly as it was — one public value, no slot identity, therefore **no** slot-aware record and no invented default (`legacy_parity=true`, and that parity candidate reports `discoverable=false`). `publication=false`, `route=false`, and the eligible package still asserts no authority. |

## 5. Exact digest and identity facts, and the disclosed limitations

- **Source → canonical output → immutable output.** For the vector case the chain is: approved catalogue source digest → the sanitizer's canonical vector bytes, whose own recomputed digest must equal the accepted `output_digest` recorded by the confined worker → the package content key `sha256/<output digest>.svg`. Source digest and output digest are **distinct and both recorded**, and the case asserts the inequality rather than assuming it. For the raster cases the chain is: the exact bytes of the immutable confined transfer → the same bytes (`source_sha256 == output_sha256`) → `sha256/<digest>.png|webp`.
- **No source re-read, no public path.** The bytes come from the same in-flight transfer that passed inspection, through a crate-private handoff; no filesystem path, owner ancestry, namespace, rights-holder string or test provenance can reach the outward view, which carries only the key, role, slot, MIME, geometry, length and output digest.
- **Disclosed in-memory packaging.** The package is held in memory only. That is acceptable for this step and asserts neither a persistent nor a public output; no staged artifact outside a disposable task-owned test directory was ever written, and nothing was committed as a product output.
- **Known bounded limitation carried forward from R18.** A root-layer Platform registration whose approved asset is AVIF is still refused (`root-registration-avif-unsupported`), because the AVIF envelope records an authored EFS obligation as its own provenance and a root registration has none. No approved `1.0.0` asset is AVIF, so nothing deliverable is lost; admitting one later requires a separate order to extend that envelope. This order added no second limitation.

## 6. Rights, assets and what remains open

- The installed library is still exactly the 81 Owner-approved MIT Tabler icons plus the catalogue, the complete MIT notice and the provenance record at `platform/assets/` (84 files, on the technical branch only — `main` carries governance and report commits and no product bytes). The catalogue digest is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` at `platformAssetLibraryVersion: "1.0.0"`. **No catalogue field, entry, product byte or rights flag changed**, and the production registration table is still empty (`automatic_product_graphic_defaults_active=false`). The N130 cases reuse only representative approved assets — one Platform icon plus the existing PNG and WebP fixtures — and ran no fresh 81-asset matrix; the retained N127 gate remains the per-asset regression evidence.
- **The one grounded rights/role decision queue — 20 canonical Foundation sources**, drawn from the existing R16 142/142 ledger (101 canonical, 41 mirrors/test) and never re-counted here. The owner direction "every publicly available Foundation artwork path must be accounted for" stands, and none of these 20 is authorized for public product installation by this order:
  1. **`icon-services.svg`** — the original proprietary work; no recorded redistribution grant.
  2. **Two byte-duplicate alias names** — blocked twice over: no alias model exists, and adding names would require a new versioned catalogue rather than mutating the immutable `1.0.0` one.
  3. **Ten neutral placeholders** — no per-file role decision and no redistribution decision; naming them as logos, banners, favicons, backgrounds or social previews would invent a production default.
  4. **Seven third-party platform marks** — donor record **PARKED — NOT INTEGRATED / Not approved**; conditions P-1…P-4 outstanding, and no trademark licence can be inferred from public availability.
- `public_cascade_delivery=false`, `publication_authority=false`, `public_route_authority=false`, `derived_output_publication=false`, `release_authority=false`, `additional_assets_delivered=false`, `stage03_closed=false`, `r03c_open=true`, `stage03_open=true`. No preview, route, served asset, release, deployment or merge was produced.

## 7. Fresh `main` baseline, environment and cleanup

- **Fresh `main` two-host baseline after R27:** the `rust-verify` run triggered by this report commit is recorded in the Owner handoff with its run and job IDs and its exact SHA — a report cannot embed its own commit SHA, so it is delivered in chat instead. As an already-verified `main` baseline for this report's parent, the A27 `main` push ran `rust-verify` `43847924589` at `e02562fc`, attempt 1: `verify (ubuntu-24.04)` `133232712091` **success** (10:20:19 → 10:23:33) and `verify (windows-2025)` `133232711665` **success** (10:20:19 → 10:23:14). `main` contains only A27 (ACTIVE-only) and R27 (report-only); the technical branch was never merged, so `main` still carries no product bytes, no `platform/assets/` and no new crate code.
- Local constraints, disclosed honestly: this machine has neither a reflink-capable filesystem nor a Windows linker, so the four N130 cases cannot execute locally — they refuse loudly on the missing `SOLO_TEST_SOURCE_FS_ROOT`, and when pointed at a plain directory the readers refuse with their own `Unsupported` arm (the retained N129 case behaves identically, which is how the cause was isolated). The package assembly, the identity binding, the key grammar, the quota and limit paths, the privacy selection and the refusal mapping were exercised locally; the native transfer, the confined workers and every filesystem proof ran only on the hosted native hosts, exactly as the order requires, and **no local success is reported as native evidence**. Both new gates were dry-run locally against a synthetic log (authentic accepted, all four mutations rejected) before commit, in both the bash and the PowerShell form.
- Repository state: `main` = A27 `e02562fc` then R27 (report only); technical = T27.0 `c3841386` → T27 `8e885295` → **T27.1 `07b2052a`**; both pushed, trees clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, **30** remote heads observed (no quota claimed). No new branch, tag, PR or worktree; no force-push, rebase, reset, `git clean`, merge or history rewrite. The failed run `43853803633` is preserved in history and reported in §3.
- Task scratch lived only in untracked, ignored `.temp/` (transaction scripts, the refactor script, the module draft, the two gate dry-run harnesses and their extracted scripts) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched, and no unknown or Owner-owned material was removed.

**Next true dependencies after acceptance.**

1. **An Owner order for actual public delivery** — Stage 04 representation visibility and route eligibility plus a separately governed public output/delivery decision. Nothing this gate prepares may be served, routed, released or deployed before that order exists.
2. **The Owner's recorded rights/role grant for the 20 pending canonical sources** — the original icon, the two alias names (with a catalogue-version decision), the ten placeholders and the seven marked third-party assets, the last group only after P-1…P-4. Without it no further artwork can be installed, and the full-coverage direction cannot be satisfied.
3. **The still-open production graphic default decision from R17** — the registration table remains empty by design; naming exact role/slot → approved-asset pairs (with explicit adoption) would be a product-behaviour change and this order deliberately did not make it.



