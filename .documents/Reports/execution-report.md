# Execution Report

**Work order:** `SOLO-WO-118 R21` — bridge the existing strict publication manifest to the typed EFS visibility ledger, without granting publication authority.
**Bounded outcome:** `R04C2-STRICT-MANIFEST-TO-EFS-VISIBILITY-BRIDGE-N132-TWO-HOST-NONPUBLISHING-V1`.

```
status=COMPLETE
manifest_visibility_bridge_implemented=true
manifest_efs_visibility_scope_verified=true
n132_two_host_gates_verified=true
locale_conformance_full=false
route_ledger_implemented=false
whole_manifest_semantic_validation=false
public_cascade_delivery=false
publication_authority=false
public_route_authority=false
derived_output_publication=false
release_authority=false
automatic_product_graphic_defaults_active=false
additional_assets_delivered=false
r03c_open=true
stage03_open=true
```

**Status: `COMPLETE` for the bounded `R04.C2` bridge only.** One additive, no-I/O adapter now converts the output of the **existing** strict, duplicate-aware publication-manifest parser into one accepted `EfsOwnershipGraph`, one complete R20 `RepresentationVisibilityLedgerV1`, and the checked SHA-256 of the supplied bytes, and offers a pure EFS-membership scope check for an already validated graphic candidate. Four focused cases prove it on Ubuntu and Windows. This is **not** completion of `R04.C`: no route ledger, no full Unicode-18/BCP47 locale conformance, no whole-manifest semantic validation, no HTTP surface, no public asset, no activated snapshot and no release exists. Nothing was served, routed, published, released, deployed or merged, and the 20 pending Foundation sources remain undelivered.

## 1. Transactions, SHAs and exact equivalence

| Step | SHA | Validation |
|---|---|---|
| baseline | main R28 `40107f18c177d4769b4eec18f89e0492163015c9` (report blob `1498057b34cbb65c617b735bf78e6317d47f3e34`); technical T28.2 `c9722f20b5f05b0044d39c6dbc066375b3ce3ede` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four WIP files, empty stash, clean trees, **30** remote heads observed (unrelated Dependabot heads untouched) |
| **A29** (main, ACTIVE only) | `f207765c57ef96eae21319fce2a3908d465a3c18`, tree `377aa0dab515b470a39caaae3ef742632600c80f`, parent `40107f18` | ACTIVE blob `15421d9cc4e0bdaa0ff9fef127c5d4d72ded0e36`, file sha256 `3b2e3567e3c33d39890c0e53636c026e31952dd96af1790ba8e0dd3176566171`, 28 328 bytes / 150 lines, identical in worktree, index, commit and `origin/main`; exactly one path |
| **T29.0** (branch, ACTIVE only) | `419bccbbc914e003882e7f0d277fc762f3031b80`, tree `0c42eaa632b3962e12500a064c1d3b7e1b1c2529`, parent `c9722f20` | the **identical** blob; `git diff T28.2..T29.0` = ACTIVE only; pushed |
| **T29** (branch, implementation) | `f02aa20d24c5d7d87fed4d391cb6bc74f6d3b69e`, tree `f8d2ef5885b60d1b3268d9151a601e20257d48ef`, parent `419bccbb` | **all three native jobs green on the first attempt** (§3); pushed |
| **R29** (main, report only) | this commit (SHA in the Owner handoff) | parent `f207765c`, exactly one path |

Changed paths across `T29.0..T29` — exactly the authorised ledger, nothing else:

| Path | Δ | Role |
|---|---|---|
| `crates/solo-adapters/src/manifest_visibility_bridge.rs` | **+577 / −0 (new)** | the no-I/O bridge: existing parser → accepted graph → complete R20 ledger, restricted-profile locale admission, closed refusals, pure EFS scope check |
| `crates/solo-adapters/src/lib.rs` | +7 / −0 | one module line and the public re-exports |
| `crates/solo-adapters/tests/publication_manifest.rs` | **+664 / −0** | additive `mod n132_manifest_visibility` only; all 16 existing tests byte-identical |
| `.github/workflows/rust-verify.yml` | +127 / −0 | additive Linux and Windows N132 gates after the retained N131 gate |
| `.documents/Architecture/ARCHITECTURE.md` | +3 / **−1** | the compact R21 truth note, plus the **one replaced clause** of the R20 note that R21 makes stale (see below) |

`publication_manifest.rs`, every `crates/solo-domain/**` and `crates/solo-application/**` source, `Cargo.toml`, `Cargo.lock`, `deny.toml`, `.gitattributes`, `platform/assets/**`, all donor and licensing fixtures, every existing security test body, `.documents/WIP/plan-*` and the CLI runtime are **unchanged**: `git diff --name-only T29.0..T29` lists only the five paths above. The approved tree is still exactly 84 product files and the catalogue digest is still `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7`.

**Architecture, measured:** `architecture=legal packages=4 targets=39 files=80 edges=93` on both native hosts — exactly one new file, zero new packages, targets or dependency edges.

**Disclosed corrections made before commit.** (a) The R20 truth note in §3.2A still claimed that "no production code yet parses `visibilityDefault` or `visibility` from a manifest", which R21 makes false; that single clause was replaced with the truthful statement that R21 now bridges those fields while semantic route, content and publication authority remain unbuilt — hence the one deletion above. (b) The architecture guard refused a local binding named `region` (a member of its `DYNAMIC_LOADING_ROOTS` list) in the locale profile helper; the identifier was renamed before commit. No guard, gate or policy was weakened.

## 2. N132 — four cases, genuinely executed on both native hosts

| Case | What the native run proves |
|---|---|
| **N132_01** | Real strict JSON bytes (one Entity, sibling Sites both declaring `home`, an extra `about` Page, four representations) convert to one graph and one ledger with `rows=4` in stable authored order, the four accepted combinations (`public/page-default`, `unlisted/representation-override`, `public/representation-override`, `unlisted/page-default`), distinct sibling `home` owners, correct discovery/robot intent, and `manifest_sha` pinned to the SHA-256 of the supplied bytes. The R20 typed constructor over the **same** graph produces an **identical** ledger, proving no second inheritance algorithm. All authority booleans are false and `parsed().base().canonical()` is still merely an inert declaration. |
| **N132_02** | Fourteen refusals through the real parser entry point: a duplicate decoded `visibility`/`\u0076isibility`, an unknown nested key, `visibility: "internal"`, invalid UTF-8, a BOM, truncated JSON, trailing tokens, a wrong `platformAssetLibraryVersion`, a missing root key, a missing child key, and 2 MiB+1 keep the parser's exact causes; `de-de`, bare `en` and `en-001-u-ca-gregory` are refused by the **bridge** as `locale-not-yet-validated` at `….locale` while `parse_manifest_bytes` alone still accepts them. No refusal produces a graph, a ledger or a snapshot. |
| **N132_03** | The matching candidate returns `scope=exact-EFS-membership`; seven one-dimension perturbations — Entity, Facet, sibling Site, missing Page, extra Page, changed representation locale, missing representation — each refuse with `scope-mismatch`. The sibling-rename variant demonstrably **shares** the Entity and still contains the target owner, yet refuses: matching only the Entity and a target would be insufficient, and is not what is implemented. |
| **N132_04** | Identical bytes produce an identical snapshot, digest, graph and ledger; a label-only change moves the digest while the graph and every ledger row stay identical; an `unlisted` representation keeps its exact typed identity with `noindex,nofollow` intent and is **not** private; the parsed candidate's `route().path()`, `sourceCandidates` and `base` remain inert; and `internal` is refused as a representation token while the closed R20 type admits exactly `public` and `unlisted`. |

## 3. Final technical proof — run `43878336393`, attempt 1, at `f02aa20d`

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133337210986` | **success** (67 steps) |
| `verify (windows-2025)` | `133337211553` | **success** (67 steps) |
| `avif-reference-oracle` | `133337211574` | **success** |

**No failed attempt exists for this run:** T29 passed all three jobs on attempt 1. Every retained gate is green on both hosts at that one SHA — Ubuntu: N125 (33), N126 (35), N127 (37), N128 (39), N129 (41), N130 (43), N131 (46), **N132 (47)**, N124 (49), architecture checker on the real checkout (52), format; Windows: N124 (32), N125 (34), N126 (36), N127 (38), N128 (40), N129 (42), N130 (44), N131 (45), **N132 (48)**, architecture (53). The independent AVIF oracle passed at the same SHA, and the measured architecture line on both hosts is `packages=4 targets=39 files=80 edges=93`.

**The N132 gate is fail-closed, on both shells.** Each host runs `cargo test -p solo-adapters --test publication_manifest --locked n132_ -- --nocapture --test-threads=1`, then requires exactly the four accepted test names, `test result: ok. 4 passed; 0 failed; 0 ignored`, exactly one `n132_evidence case=01..04` marker per case, no `FAILED` or `ignored` line, the grounded tokens `rows=4`, `sibling_isolated=true`, `manifest_sha_pinned=true`, `duplicate_decoded_key=refused`, `internal_representation=refused`, `locale_profile=restricted`, `parser_accepts_locale=true`, `scope_exact=true`, `foreign_refused=true`, `sibling_refused=true`, `entity_only_insufficient=true`, `before_read=true`, `unlisted_not_private=true`, `ledger_stable=true`, `zero_partial=true`, `publication=false`, `route=false`, and exactly one 64-hex `manifest_sha=` that is not all zeros. Only then does it print `n132_gate authentic=accepted`. It then re-checks **four directly altered copies of that same genuine log** — a deleted case, a forged `unlisted_not_private=false`, a forged `scope_exact=false`/`foreign_refused=false`, and a forged `publication=true`/`route=true`/`zero_partial=false` — and prints `n132_gate mutation=rejected` only if all four are refused; any failure exits non-zero, so the green step is the certificate and the verdict lines live in that step's log at this exact revision.

**Local pre-commit proof, honestly scoped.** N132 needs no native filesystem, so it was executed locally in full: 20/20 tests in the target (the 16 pre-existing cases plus N132 4/4) pass, fmt is clean, strict Clippy is clean and the architecture checker reports `files=80 edges=93`. The **Linux gate body was additionally run end-to-end locally**, invoking the real `cargo test` and then rejecting all four mutations; the **Windows gate body** was executed against the genuine captured log under `pwsh`, likewise accepting the authentic log and rejecting all four mutations. Nothing local is presented as native evidence, and no local run substitutes for the hosted two-host verdict above.

## 4. What the bridge is, and what it is not

- **One parser, reused exactly once.** `parse_manifest_visibility_snapshot_v1` calls the accepted `parse_manifest_bytes` a single time on the supplied bytes. There is no second JSON vocabulary, no `serde_json::Value` trust bypass, no re-parse-as-validity, no relaxed unknown-field policy and no handwritten replacement for its decoded-key duplicate checks. A parser refusal is **wrapped, not erased**: `ManifestVisibilityBridgeRefusalKindV1::Manifest(<exact ManifestErrorKind>)` at the parser's safe structural location.
- **The complete parsed tree becomes one graph.** Every declared Facet, Site, Page and representation is visited in authored containment order, each ID reused exactly as parsed, and the accepted `EfsOwnershipDraft::build()` performs the identity, cardinality and duplicate refusals — nothing is defaulted, synthesised or repaired (no invented `main`, Page, locale or route).
- **One complete ledger, no new inheritance algorithm.** Exactly one `PageVisibilityDeclarationV1` per declared Page carries the **effective** declared Page default (`PageDeclarationV1::visibility_default()` already resolves an omitted default to `public`; whether it was authored is unknowable here and is not claimed), and each representation override is added **only** when `visibility()` is `Some`, so absence stays distinct from an explicit override. The accepted `build_representation_visibility_ledger_v1` then produces the ledger, and the conversion asserts ledger coverage equals `representation_count` and the built Page count equals `page_count` — a cardinality or membership inconsistency refuses the whole pair, never a partial one.
- **A provenance pin, not an authorisation.** `manifest_sha256()` is the checked SHA-256 of the **supplied** bytes via the accepted `sha256_media_worker_bytes_v1`. It is an internal pin: not a signed grant, not source approval and not publication authority.
- **The snapshot holds the original parsed candidate.** `parsed()` returns the very `ParsedManifestCandidateV1` the parser produced — never a rebuilt look-alike — so later stages cannot be misled into thinking the `base`, route claims, aliases, missing-translation rules, `sourceCandidates`, inert `properties` or labels were semantically validated. They keep exactly their existing syntactic checks and remain expressly pending.
- **Deliberately restricted locale profile.** `LocaleKeyV1::from_already_canonical` validates membership *shape* only, so the bridge first applies the **canonical-shaped base profile**: lowercase two/three-letter language, optional Title-cased four-letter script, mandatory uppercase two-letter or three-digit region, with no further subtag. `en-001`, `de-DE`, `id-ID` and `zh-Hans-CN` are admitted; `de-de`, bare `en`, `en-001-u-ca-gregory` and any mixed-case or extended form are refused as `locale-not-yet-validated` with no case-folding and no inference. The restriction is in the **bridge**, not the parser — N132_02 proves the parser still accepts those spellings. This profile is intentionally **narrower** than valid BCP47; full ICU4X/Unicode-18 canonicalisation, aliases, extensions and equivalence are open `R04.B`/`E05` work, and a currently unsupported-but-valid variant is a safe refusal, not a claim that the standard forbids it. The Site `defaultLocale` gets the same check but never invents a representation.
- **A narrow, pure scope check.** `check_manifest_efs_scope_v1(snapshot, candidate)` compares the **complete, fully qualified, ordered** Entity → Facet → Site → Page → representation membership of the manifest graph against `ValidatedPropertyInputV1::owners()`. It returns exactly one token, `exact-EFS-membership`, or a closed `scope-mismatch`. It takes no port at all, so it can only refuse **before** any reader or worker exists. It binds EFS ownership scope **only**: it never claims the candidate's inert properties, source bytes or routes came from these bytes.
- **A syntactically accepted manifest is not a validated publication.** The bridge exposes six constant-`false` authority facts (`grants_publication_authority`, `routes_any_url`, `serves_http`, `publishes_derived_output`, `grants_release_authority`, `grants_deployment_authority`), no URL, no route, no HTTP handler, no output file and no release artifact.

## 5. What R21 does **not** establish

- **`R04.C` is not complete.** There is no production semantic route ledger (`route_ledger_implemented=false`), no full Unicode-18/BCP47 locale conformance (`locale_conformance_full=false`) and no whole-manifest semantic validation (`whole_manifest_semantic_validation=false`). A manifest that the accepted parser accepts and this bridge converts is *syntactically* accepted and *EFS-converted*; it is **not** a semantically validated publication candidate.
- **No route, no delivery, no authority.** `public_cascade_delivery=false`, `publication_authority=false`, `public_route_authority=false`, `derived_output_publication=false`, `release_authority=false`. This order produced no URL, handler, server, route manifest, canonical tag, sitemap, `robots` file, response header, public asset, activated snapshot or deployment, and `main` still carries no product delivery.
- **No new product behaviour.** `automatic_product_graphic_defaults_active=false`: the production registration table stays empty, no role is bound to an approved asset, and nothing in the manifest can imply a graphic default.
- **The scope check is not an authentication.** It binds EFS ownership membership only; it does not prove the candidate's inert properties, source bytes or routes were derived from these manifest bytes, and it is deliberately not named `Published`, `ValidatedManifest` or `ReadyToServe`.
- **`R04.A`, `R04.B`, `R04.D`, `R03.C` and Stage 03 remain open** (`r03c_open=true`, `stage03_open=true`).

## 6. Rights, assets and outstanding decisions

- The installed library is unchanged: exactly the 81 Owner-approved MIT Tabler icons plus `catalogue-1.0.0.json`, the complete MIT notice and the provenance record — **84 product files**, catalogue SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` at `platformAssetLibraryVersion: "1.0.0"`. No catalogue field, entry, product byte or rights flag changed and no artwork was installed (`additional_assets_delivered=false`).
- **Precisely 20 outstanding Foundation canonical sources remain NOT approved for installation:** the original `icon-services.svg`, the two duplicate-icon aliases, ten neutral role placeholders and seven third-party platform marks (`PARKED — NOT INTEGRATED`, P-1…P-4). The 142/142 frozen R16 ledger already accounts for all publicly available Foundation artwork; it is not a redistribution or trademark grant. The 81 approved icons stay installed.
- **The production default registry is still empty** and no `role/slot → approved asset` pair is registered. Activating one, importing the 20 sources, or promoting the pinned library to a new version each require a separate Owner decision.

## 7. Fresh `main` baseline, environment and cleanup

- **Fresh post-R29 two-job `main` baseline:** the `rust-verify` run triggered by this report commit is supplied in the Owner handoff with its run and job IDs and its exact SHA, because a report cannot know its own not-yet-created SHA and none is invented here. Its verified parent baseline is A29 `f207765c`, whose `main` push produced green `rust-verify` runs.
- **`main` stays governance-only for this work:** A29 (ACTIVE only) and R29 (report only). No technical code, no `crates/` change and no `platform/assets/` change was merged or deployed; the technical branch remains unmerged.
- **Repository state:** `main` = A29 `f207765c` then R29; technical = T29.0 `419bccbb` → **T29 `f02aa20d`**; all pushed, trees clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, **30** remote heads observed. No new branch, tag, PR or worktree; no force-push, rebase, reset, `git clean`, merge or history rewrite.
- **Scratch disposition:** task-owned scratch lived only in ignored, untracked `.temp/` (transaction scripts, the N132 gate dry-run harnesses and their captured logs) and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched. One process correction is disclosed: the first source edit was initially drafted while `main` was checked out, detected before any commit, reverted in full (`git checkout` of the one modified tracked file plus removal of the one untracked draft) and re-applied on the technical branch; no `main` commit ever contained technical code.

**Next true dependencies after acceptance.**

1. **The full `R04.C` semantic layer**, in bounded stages: the production route ledger with exact-case unlisted claims, full ICU4X/Unicode-18 BCP47 locale binding and equivalence (widening the deliberately restricted profile), real Content JSON/Markdown selection, schema-generated publication authority and the immutable activated snapshot.
2. **`R04.A`/`R04.B`**, including base-URL configuration and the route-case/Unicode collision contract, before any HTTP surface exists.
3. **Owner authorisation for public serving, release and deployment**, plus the recorded rights/role decision for the 20 pending Foundation sources and the explicit `role/slot → approved asset` default registrations. None of these is conferred by this order.


