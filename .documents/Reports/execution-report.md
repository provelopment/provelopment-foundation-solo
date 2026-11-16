# Execution Report

**Work order:** `SOLO-WO-118 R15` — install the exact Owner-approved Foundation 81-icon Platform library `1.0.0` and prove its separate confined source reader.
**Required outcome:** `R03C-OWNER-APPROVED-81-TABLER-PLATFORM-1_0_0-INSTALLED-SEPARATE-CONFINED-SOURCE-N127-TWO-HOST-V1`.

**status=STOPPED**

**owner_81_assets_approved=true** · **platform_library_source_installed=true** · **platform_source_read_verified=true** (four N127 cases, both native hosts) · **publication_authority=false** · **derived_output_publication=false** · **release_authority=false** · **stage03_open=true**

**Stop reason, in one sentence:** two **pre-existing, order-protected** test bodies assert that no product `platform/` tree may exist, so the workspace test step — the mandatory first gate — cannot pass in the very state R15 authorizes; §3 of this order forbids editing those bodies and §2.4/§8 require a STOP rather than a green claim when a gate cannot be met inside the ledger.

## 1. What is proven, and what is not

Proven on both native hosts at one exact technical SHA (`8b340976836f86b79dea36be8c9db9d8576997dd`):

- The exact Owner-approved 81-icon product library is installed at `platform/assets/` with **84 files**, byte-exact donor-identical Git blobs and raw bytes.
- The Owner decision record exists and names the exact reviewed blobs.
- The new inward Platform contract and the new separate native Platform reader compile, are architecture-legal, and their four N127 cases **all pass** — including 81 genuine confined SVG children per host, the hostile refusal matrix, the native confinement negatives and the late-failure zero-partial case.
- The independent AVIF reference oracle is green at the same SHA.

Not proven, and therefore not claimed:

- **The dedicated N127 gate steps never executed.** They are ordered after the workspace test step, which failed on the two protected assertions, so both N127 gate steps and every later step were skipped. There is **no `n127_gate authentic=accepted` / `n127_gate mutation=rejected`** evidence at this SHA, and no `n127_evidence`/`n127_row` lines either, because the workspace step runs without `--nocapture`. The pass verdicts quoted in §5 are the real harness lines.
- No three-success technical run exists at one SHA, so R15's completion condition is unmet.

## 2. Transaction and exact changed-path ledger

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` R22 `248661f3f2bdd62e786d37e5e82a961afcc687cd`; technical T22.1 `5e26672e4f68f26967b9b98ab6d70383db34ccdb` | one `origin`, one worktree, four WIP files, empty stash, both trees clean, 30 remote heads |
| **A23** (`main`, ACTIVE only) | `3e2497e920fa547e7dea3d706cbbaa9734ef737a`, tree `19506f4b47dbc5dfa4ad8ac6ee6c21523e972532`, parent `248661f3` | ACTIVE blob `86ae97341b9ee39b5953bb548965c1dfef43e946` identical in worktree, index, commit and `origin/main` |
| **T23.0** (branch, ACTIVE only) | `502892fa0dc6f37fe462e78ade425f2b7a509a47`, parent `5e26672e` | same ACTIVE blob; `git diff --name-only T22.1..T23.0` = ACTIVE only |
| **T23.1** (branch, the R15 implementation) | `bc277bf38d515c486d8233eb35a7707213e91d16` | 94 paths: 84 product files + 10 ledger files (`.gitattributes`, owner-decision record, two new application files, two new adapter files, two new test targets, `mod n127`, two CI gates) |
| **T23.2** (branch, one repair) | `0da55e28202ad5c7d36604a3d6306805a36c05bd` | one file: a missing catalogue reports `CatalogueUnavailable`, not a missing asset |
| **T23.3** (branch, two test repairs) | `8b340976836f86b79dea36be8c9db9d8576997dd` | one file, `mod n127` only: an empty root reports `CatalogueUnavailable`; the withheld filename is proved a non-member rather than an unsafe spelling |
| **R23** (`main`, report only) | this commit (SHA in the Owner handoff) | parent `3e2497e9`, exactly one path |

`.gitattributes` gained exactly one line after the accepted R14 rule, with no other change: `platform/assets/** -text` (blob `b22f49ab…` → `b3462a6…`, 442 → 467 bytes). No existing fixture, R14 candidate or review sheet, sanitizer, decoder, reader, worker, domain source, manifest, `Cargo.lock`, `deny.toml`, architecture policy or earlier gate body was modified. Every red run below is retained; nothing was reset, cleaned or force-pushed.

## 3. The installed library: 84 files, byte-exact

Installed from the pinned Git blobs only — never by `cp -r`, a glob or donor traversal — after parsing the pinned candidate catalogue and matching it one-to-one to the 81 review-sheet rows and the 81 review fixtures (order and membership verified):

| Content | Count | Identity |
|---|---|---|
| `platform/assets/icons/<name>.svg` | **81** | each raw SHA-256 equals its catalogue digest, each byte sequence equals the reviewed donor fixture, all LF, `currentColor` preserved in the source |
| `platform/assets/catalogue-1.0.0.json` | 1 | byte copy of the R14 candidate, blob `a02db160…`, 65 397 bytes, SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` |
| `platform/assets/licensing/TABLER-ICONS-MIT.txt` | 1 | blob `3e82379d…`, 1 073 bytes, SHA-256 `b740a1d4…` |
| `platform/assets/licensing/icon-provenance.json` | 1 | blob `a74d2344…`, 25 274 bytes, SHA-256 `e0f0ea62…` |

`icon-services.svg`, `icon-status-info.svg` and `icon-volunteer.svg` are absent from the product tree and from the catalogue. `git check-attr text eol --` resolves **`text: unset` for all 84 product paths**; the accepted fixture rule is intact and unrelated paths keep their earlier attributes. The unchanged R12 strict parser accepts the installed catalogue with the version supplied independently, yielding exactly 81 entries.

`.documents/Reports/platform-library-1.0.0-owner-decision.md` records the Owner's "approved, proceed" authorization for exactly the 81 reviewed assets identified by the candidate blob `a02db160…` and the review-sheet blob `edb8468d…`, with the MIT and provenance conditions, the explicit exclusions and the disabled authority gates. It is a factual record, not a licence instrument; the historical review-sheet checkboxes are untouched.


## 4. The new separate Platform source seam

**Inward** (`crates/solo-application/src/platform_asset_read.rs`, additive): `PlatformAssetLibraryReadPortV1` with exactly two readings (the catalogue document, and one exact identifier's own bytes); a typed `PlatformAssetRefV1` that accepts only `icons/<portable>.svg` and refuses any `Entity`-namespace reference; an explicit `PlatformRootSelectionV1` token; and `read_pinned_platform_sources_v1`, which

1. refuses an unselected version **before reading anything**;
2. hashes the transferred catalogue bytes **in the application** and refuses unless they are `ad8ea718…`, additionally cross-checking the typed model against those exact bytes per identifier and digest so an inconsistent model cannot smuggle an unapproved digest;
3. requires exactly 81 declared entries and the approved version;
4. hashes every source in the application and compares it with the catalogue's digest for that exact identifier **before** any worker could run;
5. refuses duplicates, unknown identifiers, oversize sources, ambiguous identities and any late failure, discarding every earlier record.

Every authority accessor is a constant `false`; the set reports `source_namespace() == Platform` only.

**Outward** (`crates/solo-adapters/src/platform_asset_read.rs`, additive): `PlatformRootCapabilityV1` owns its **own** adopted `source_read_native::RootHandle` for the operator-designated `platform/` root and resolves exactly four leaf shapes (`assets/catalogue-1.0.0.json`, `assets/icons/<name>.svg`, `assets/licensing/TABLER-ICONS-MIT.txt`, `assets/licensing/icon-provenance.json`) by single validated segments through the accepted `open_leaf`/`read_bounded`/`recheck` mechanism — no `Path::join` on authored text, no `canonicalize`, no `stat`-as-proof, no second reader, no `std::fs::read` in the port. Identity, length, link count and timestamps must agree across the transfer, links must be exactly one, and a host without the immutable-transfer mechanism refuses rather than copying. `inspect_pinned_platform_icon_sources_v1` composes that gate with the **already accepted** `solo-cli` SVG child, one real process per approved asset, validating 24×24 vector geometry and the distinct source and canonical digests.


## 5. N127 on both native hosts — all four cases pass; the gates did not run

Run `43816931677` at `8b340976836f86b79dea36be8c9db9d8576997dd`, attempt 1:

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133131005793` | failure (two protected assertions only) |
| `verify (windows-2025)` | `133131005880` | failure (the same two) |
| `avif-reference-oracle` | `133131005598` | **success** |

Inside the workspace suite, on **both** hosts:

```text
test n127::n127_01_owner_approved_platform_81_real_confined_svg_sources ... ok
test n127::n127_02_version_catalogue_licence_and_scope_refusal_matrix ... ok
test n127::n127_03_separate_native_root_symlink_hardlink_and_changed_source_refused ... ok
test n127::n127_04_late_hidden_platform_failure_zero_partial_and_no_namespace_alias ... ok
```

Case 01 copied the tracked library into a scratch Platform root on the verified filesystem (Linux: the disposable Btrfs mount; Windows: the runner temporary directory), read all 81 sources through the new capability, hashed each in the application, launched **81 genuine confined SVG children** and compared the resulting aggregate with `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`. Cases 02–04 executed the eighteen-scenario refusal matrix, the native confinement negatives (hardlink alias, symlink leaf where host privilege permits, directory leaf, absent leaf, unsafe identifiers, file-as-root, oversize source, replaced root and, on Linux, the unverified-filesystem refusal) and the late-corruption zero-partial case. Earlier red runs show those cases failing only on my own wrong expectation for a missing catalogue, which T23.2 and T23.3 repaired.

**Architecture** on the technical branch is legal and was re-measured, not assumed: `packages=4 targets=38 files=74 edges=93` (the previous 36/70/93 plus the two authorized modules and two authorized test targets; no edge changed). The refusal self-test passes all 29 cases. `cargo fmt`, strict Clippy, the safe-code boundary, `cargo-deny` with a fresh advisory refresh, the release/reproducibility stages and every N093–N126 case except the two protected assertions are unaffected; no dependency, lockfile, policy or `continue-on-error` was touched.

## 6. The blocker, precisely

Two accepted, order-protected test bodies assert that the product tree must **not** exist, which is exactly the state R15 is ordered to create:

| Test | File:line | Assertion |
|---|---|---|
| `n125_04_real_entity_reader_still_refuses_platform_and_no_unapproved_library` | `crates/solo-cli/tests/media_worker_host_smoke.rs:8241` | `!manifest.join("../../platform").exists()` and the same for `../../platform/assets` |
| `n126_02_pinned_mit_provenance_and_r12_catalogue_review_only` | `crates/solo-cli/tests/media_worker_host_smoke.rs:8677` | the identical two-path absence loop |

Both are statements about the **R12/R14 state** ("no product Platform tree is introduced by this order"), not about the readers they also prove: N125_04's real assertions — the Entity reader refusing a Platform obligation and the R12 `PlatformSource` binding refusing — still hold and are separately re-proved by N127_04 and N127_02. Their load-bearing content is unaffected; only the absence loop is now false by design.

§3 of the order says **never change** any existing N093–N126 test body, while §1 and §4 require installing `platform/assets/`. Those two requirements cannot both be satisfied, so the order cannot complete inside its own ledger. Nothing was weakened to paper over this: the two bodies are untouched in every commit, and the failure is left visible in three retained red runs.

**Smallest necessary new decision (Owner/Architect, not self-issued).** Authorize one bounded amendment in a new order, choosing either:

- **(a) preferred** — replace the two-path absence loop in each of those two bodies with an assertion that *either* no product `platform/` tree exists *or* it is exactly the approved library (84 files, pinned catalogue SHA-256, no withheld name), which keeps both tests load-bearing under R15; or
- **(b)** delete the two absence loops (two small blocks each) and keep everything else in those bodies byte-identical; or
- **(c)** re-scope the installation to a path those bodies do not name — not recommended, because it moves the approved product root away from `platform/assets/` and would need the whole R12/R15 naming chain revisited.

With (a) or (b) the technical branch needs one commit and one fresh run; the N127 gate steps would then execute for the first time, and the four `n127_evidence` lines plus `n127_gate authentic=accepted` / `n127_gate mutation=rejected` would become available for the completion claim.

**Separation preserved.** `ContentRootCapability`, `read_graphic_asset_bytes_v1` and the Entity V4 path are unchanged and still refuse any Platform obligation (`PlatformUnverified`); the R12 `PlatformSource` binding still returns `PlatformSourceUnavailable`; no `platform/assets/` read is reachable from an Entity root, a route, a locale, a role, a filename or a fallback.

## 7. Red runs retained as history

| Run | SHA | Outcome |
|---|---|---|
| `43815292648` attempt 1 | `bc277bf3` | Ubuntu `133125852713` red, Windows `133125853095` red, oracle `133125853026` green. Failures: the two protected assertions plus my own N127 catalogue-kind expectations (repaired by T23.2), and one N127 alias expectation (repaired by T23.3). |
| `43816188208` attempt 1 | `0da55e28` | Ubuntu `133128659300` red, Windows `133128659308` red, oracle `133128658974` green. N127_02 green; N127_03/N127_04 red on the two stale expectations repaired by T23.3; the two protected assertions still red. |
| `43816931677` attempt 1 | `8b340976` | Ubuntu `133131005793` red, Windows `133131005880` red, oracle `133131005598` green. **Only** the two protected assertions red; all four N127 cases green on both hosts. |

The accepted R14 history is preserved unchanged: technical run `43807461257` (Ubuntu `133101393481`, Windows `133101393733`, oracle `133101394241`), main baseline `43808522755`, and the original R13 Windows failure `133090902898`. No earlier report, gate body or fixture byte was edited to make this run look better.

## 8. Rights, product and publication boundaries — unchanged and closed

- `owner_81_assets_approved=true` records the Owner's set-wide approval of exactly the 81 reviewed assets under the Tabler MIT terms; the MIT notice and the provenance record are installed **beside** the sources and must travel with any separately authorized distribution. The Foundation root Apache-2.0 licence neither replaces that notice nor grants rights in names, logos or trademarks.
- Not authorized by this order and not done: artwork beyond those 81 bytes, an approval token, optimised or derived artwork, alias reinstatement, licensing of `icon-services.svg`, third-party platform or social marks, any publication, any release or deployment, any merge, and any closure of `R03.C` or Stage 03.
- Still refused by construction: `AssetNamespace::Platform` in the Entity reader (`PlatformUnverified`), the V4 `PlatformSource` matcher (`PlatformSourceUnavailable`), and every authority constant (`publication_authority=false`, `derived_output_publication=false`, `release_authority=false`, `stage03_open=true`). Mixed Entity+Platform five-level whole-candidate publication remains unproven and unclaimed.

## 9. Repository state and cleanup

- `main` = A23 `3e2497e9` then R23 (report only); technical branch = T23.3 `8b340976`; both pushed, both clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, `git status --porcelain` and `git diff --check` clean.
- **Observed remote heads: 30** — the 28 long-standing heads plus the two Dependabot app heads (`dependabot/cargo/svgtypes-0.16.2`, `dependabot/cargo/syn-3.0.10`), which this agent did not create, merge or modify. No new head, tag or worktree was created.
- Task scratch (installation script, attribute patch, transaction scripts) lived only in untracked `.temp/` and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched. The Linux fixture filesystem and the Windows linker remain unavailable locally, so N127 was never executed on this machine and is reported only from the hosted runs.

## 10. Disposition and next Owner step

`status=STOPPED` for `R03C-OWNER-APPROVED-81-TABLER-PLATFORM-1_0_0-INSTALLED-SEPARATE-CONFINED-SOURCE-N127-TWO-HOST-V1`, because the order's own "never change an N093–N126 test body" clause collides with its mandated installation: two protected assertions require the absence of the very tree R15 installs.

Everything else the order asked for exists and is verified on both hosts: the 84-file byte-exact library, the pinned catalogue/licence/provenance, the one scoped attribute line, the Owner decision record, the new inward contract, the new native Platform capability, and four N127 cases green on Ubuntu and Windows with 81 real confined children each. What is missing is a green three-job run at one SHA — blocked solely by those two assertions, whose gate steps therefore never ran.

**Next Owner decision:** authorize option (a), (b) or (c) in §6 in a new Owner-issued order. Option (a) or (b) is a two-line change in test-only code, after which one commit and one fresh run produce the N127 gate evidence and the three-success proof this outcome needs. `R03.C` and Stage 03 remain **OPEN**.


