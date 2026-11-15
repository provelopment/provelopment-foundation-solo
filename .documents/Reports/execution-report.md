# Execution Report

**Work order:** `SOLO-WO-118 R12` — closed Platform asset catalogue contract and inert V4 fact-binding, Owner-review ready.
**Required outcome:** `R03C-PLATFORM-ASSET-CATALOGUE-STRICT-INERT-N125-TWO-HOST-OWNER-REVIEW-READY-V1`.

**status=COMPLETE** for that bounded outcome only. A closed, bounded, **inert** `1.0.0` catalogue parser/validator and an independent V4 evidence-matching gate now exist, proved against synthetic test records on both native hosts. **No product Platform library, no approved artwork, no rights clearance, no Platform source read, no publication authority, no `R03.C` and no Stage 03 closure exists or is claimed.**

**rights_review_pending=true** · **stage03_open=true** · **platform_library_delivered=false** · **publication_authority=false**

## 1. Transaction

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` = R19 `a69486c7530e00f7006ee87ba6eeea4ad2785dba`, branch = T19 `5d35c892` | one `origin`, one worktree, 28 remote heads, four WIP files, empty stash, clean trees |
| **A20** (`main`) | `964c169b1ef6add12a7a66be383db5124507820c` — tree `95f796e6fa105feb4731646203274087771e2f41`, parent `a69486c7` | ACTIVE blob `bcbf6f00a6c93a3e415a2ab9580f1aa68980b01a` identical in working tree, index, A20 and `origin/main` |
| **T20.0** (branch) | `3cd18423428ba6e2eddac1636496f2163adab1aa`, parent `5d35c892` | single-path `git restore --source=A20`; identical blob; staged set = ACTIVE only |
| **T20.1** (branch) | `23404593a97d647343fc48246bbddb4927489b37` | the R12 implementation: `8 files changed, 3171 insertions(+)`; run `43735794385` **failed** (see §5) |
| **T20** (branch, final) | `d219f6b8943664f5e32eb4288edefb7ba3d3268c` | repair commit; push run **`43736439423` attempt 1** — all three jobs green |
| **R20** (`main`) | this report's commit (SHA in chat) | parent `964c169b`, exactly one path changed |

Final technical run `43736439423` attempt 1 at `d219f6b8`: `verify (ubuntu-24.04)` `132879959703` ✅, `verify (windows-2025)` `132879959898` ✅, `avif-reference-oracle` `132879959950` ✅ — one exact SHA, no rerun, no dispatch.

Changed paths (exactly the ledger): `.documents/WIP/workorder-active.md` (T20.0 only), `crates/solo-application/src/platform_asset_catalogue.rs` (**new**), `crates/solo-application/src/lib.rs`, `crates/solo-application/tests/platform_asset_catalogue.rs` (**new**), `crates/solo-adapters/src/platform_asset_catalogue.rs` (**new**), `crates/solo-adapters/src/lib.rs`, `crates/solo-adapters/tests/platform_asset_catalogue.rs` (**new**), `crates/solo-cli/tests/media_worker_host_smoke.rs` (additive `mod n125` only), `.github/workflows/rust-verify.yml` (two additive N125 gates). **No Cargo.toml, `Cargo.lock`, `deny.toml`, policy, crate, `unsafe`, feature, decoder, child, supervisor, protocol, CLI production source, reader, V1–V4 inspector, fixture, golden, oracle or SOP change.** `oxideav-avif =0.0.18` stays pinned at `fcaf60ff…b1cee`; zero lock drift.

## 2. The R12 product change

**Application contract** (`solo-application/src/platform_asset_catalogue.rs`, re-exported). Pure typed invariants, no serde, no I/O:

- `PlatformAssetDeclarationV1` carries one declared identity; `PlatformAssetCatalogueEntryV1::new(declaration, licence, provenance)` validates it and derives the format from the identifier's extension. `PlatformAssetCatalogueV1::new(selected_version, entries)` takes the **selected version as an independent trusted argument** (never discovered, guessed or upgraded), refuses anything but `"1.0.0"`, an empty set, more than 1024 entries, a duplicate id, a case-colliding id and a duplicate digest alias.
- Identifier rules reuse the shared closed portable-relative-path grammar under the **Platform** namespace, plus ≤8 segments, reserved Windows stems, reserved namespace/route segments (`platform`, `entity`, `content`, `pages`) and the catalogue's own filename; no leading slash, traversal, control/percent/backslash ambiguity, uppercase extension or trailing dot/space. The V4 `declared_by` Entity gate is left untouched and is **not** repurposed for Platform.
- MIME must be one of the six accepted V4 image types and must agree exactly with the extension; `mediaClass` is exactly `image`; geometry is 1..=8192 with a checked ≤32,000,000 pixel ceiling; licence/provenance text is 1–512 control-free scalars. **Syntactic non-emptiness is not licence truth**, and no constructor accepts an approval flag.
- `PlatformCatalogueRefusalV1` is a closed, path-free taxonomy (17 categories) whose `Debug`/`Display` render only the token.
- `match_catalogue_entry_to_v4_evidence_v1` and `bind_catalogue_to_v4_evidence_v1` compare an entry with an **already checked** `GraphicImageInspectionV4`: the parent-computed **source** digest (never an AVIF plane/output, GIF timeline, canonical SVG or decoded-plane digest), format, canonical MIME, media class and geometry. `PlatformCatalogueBindingScopeV1::PlatformSource` is refused with `PlatformSourceUnavailable` because R12 has no Platform reader; only the explicit test-only `ContentFactComparisonOnly` scope can reach `MatchedForReview`/`CompleteForReview{matched}`. The first disagreement refuses the whole binding with a position and a category and **retains no earlier match**. Every authority flag — `grants_publication_authority`, `proves_media_correctness`, `rights_reviewed`, `platform_source_readable`, `derived_output_verified`, `release_ready` — is a constant `false`.

**Strict parser** (`solo-adapters/src/platform_asset_catalogue.rs`). Input is owned bytes, UTF-8, no BOM, ≤2 MiB, exactly one JSON object. `serde`'s `derive` feature is not in the locked feature set, so every nested shape is a **hand-written duplicate-rejecting visitor** with a closed key set and an explicit duplicate mask — the pattern the accepted worker-protocol parser already uses. A `serde_json::Value` round trip is never used as a proof. Duplicates are refused at every depth (root, asset, licence, provenance); unknown fields, nulls, wrong types, missing fields, trailing data and aliased spellings are refused.

**Boundary firewall.** The catalogue is **not** connected to `read_graphic_asset_bytes_v1` or `inspect_entity_graphic_images_v4`; no Platform reference became readable; `platform/assets/` and any production catalogue remain absent from the product tree.


## 3. N125 evidence — four real cases on two native hosts

Both hosts ran `cargo test -p solo-cli --test media_worker_host_smoke --locked n125_ -- --nocapture --test-threads=1` and reported `test result: ok. 4 passed; 0 failed; 0 ignored`, each case name exactly once and each `n125_evidence case=0N` exactly once:

```text
Ubuntu  case=01 formats=png,webp,jpeg,gif,svg,avif records=6 levels=5
        pids=[7925, 7929, 7933, 7937, 7941, 7945] distinct_pids=6 matched=6
        review_only=true platform_read=false publication=false unapproved_synthetic=true
Ubuntu  case=02 refusals=24 duplicate_keys=refused version_mismatch=refused approval=false
        review_only=true platform_read=false publication=false
Ubuntu  case=03 records=6 zero_partial=true private_path_absent=true
        source_hash_not_plane_hash=true fallback=false version_fallback=false
        review_only=true platform_read=false publication=false
Ubuntu  case=04 platform=unverified entity_alias=false worker_launches=0
        catalogue_shipped=false review_only=true platform_read=false publication=false
Windows identical tokens; case=01 pids=[6816, 896, 5468, 7272, 7388, 9008] distinct_pids=6
```

- **case 01** — one **synthetic** six-entry candidate whose every digest, MIME and dimension is derived from six genuine confined children over the accepted five-level Entity fixture; the real composed V4 path really launches six distinct children (five legacy PIDs from their own records plus the one AVIF child), and the inert binder returns `CompleteForReview{matched:6}`. The test states the Entity/Platform distinction explicitly: the records are Entity-sourced, the catalogue is a Platform assertion, `platform_source_readable()==false`, and the `PlatformSource` scope is refused. GIF timeline (3 frames), canonical SVG digest `0403e067…` and the AVIF four-plane 256×512 class are asserted unchanged, with no publication or rights authority.
- **case 02** — 24 independently instantiated hostile documents in the host test (plus 34 in the parser unit target): bad/missing version, wrong selected version, duplicate root/asset/nested-licence keys, unknown fields, empty set, traversal, absolute, reserved-name and uppercase-extension ids, Platform-prefixed ids, malformed/uppercase/all-zero digests, unsupported MIME, wrong extension/MIME pair, bad media class, zero/overflow/fraction geometry, missing licence and provenance evidence, NUL rights text, BOM, trailing JSON and an oversize body. Every one returns its closed typed refusal, with no partial output and a path-free rendering.
- **case 03** — six authentic V4 records; the first five fact-match, and the final AVIF entry is forged in five separate runs (source hash, displayed dimensions, declared format/MIME, a **plane digest used as a source digest**, and a missing identifier). Each refuses the **whole** binding at that position with zero partial matches, never falls back and never upgrades; the refusals expose no asset id, path or rights text.
- **case 04** — with a real Entity `icon.png` readable as a control, an Entity-declared `platform/icon.png` never resolves inside the Entity tree, a catalogued Platform entry with the same basename confers nothing, the `PlatformSource` binding is refused, and the repository carries no `platform/`, `platform/assets/` or `content/platform` tree. Zero worker launches. The typed `PlatformUnverified` refusal for a **real** Platform obligation is proved separately in `solo-application`'s catalogue test (`the_real_read_use_case_refuses_a_platform_obligation_before_any_source_read`) with a fake read port whose call counter stays **0** — the refusal precedes every source read. This split is deliberate: the CLI crate cannot name the domain namespace types its neighbours do.

Both hosts printed `n125_gate authentic=accepted` and `n125_gate mutation=rejected`: the same verifier rejected all **four** corrupted copies of the live log — a deleted case, a changed success count (`3 passed; 1 failed`), forged `platform_read=true`/`publication=true` flags, and corrupted `duplicate_keys`/`zero_partial` refusal evidence.


## 4. Retained contracts and earlier evidence

- **V4 / N124 preserved:** `inspect_graphic_image_source_set_v4`, `ConfinedGraphicInspectAdapterV4` and the six-format whole-candidate inspection are untouched; N124's four cases and both its gates are green on both hosts in this same run (`distinct_pids=6`, `distinct_pids=7`).
- **V3, N116, N121–N123, N120 preserved:** the `avif-unsupported-v3` refusal, the native read boundary, the six accepted confined workers, the R03 independent goldens and the AVIF planes all remain; N121/N122/N123 gates report `authentic=accepted` and `mutation=rejected` on both hosts.
- **Independent oracle:** `avif-reference-oracle` green at the same SHA (`pristine_manifest_matches=true`, `mutation_rejections=3/3`).
- **Architecture measured, not assumed:** the two new modules and two new test targets change the graph as expected — `architecture=legal packages=4 targets=36 files=70 edges=93` (was `34/66/93`). Four crates, inward-only edges, no forbidden import, no `unsafe`, no new dependency edge. The checker was **not** modified or weakened.
- **Honest failure record:** run `43735794385` at `23404593` failed `Test (workspace, all targets, locked)` on both hosts because one N125 assertion claimed the declaration grammar refuses `platform/icon.png` — it does not, because the shared path grammar admits a directory literally named `platform`. The repair replaced that claim with the true behaviour: an Entity-declared `platform/icon.png` never resolves inside the Entity tree, and the typed `PlatformUnverified` refusal is proved where the namespace can be constructed. A second local-only finding was real too: `assets` had been listed as a reserved first segment, so an `assets/…` id was refused as `InvalidId` before its digest was examined; the reserved set now names only the namespace/route prefixes and the catalogue filename. Nothing was skipped or weakened, and neither failure was hidden.

## 5. Local verification and honest limits

Recorded in the single Owner checkout: `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` 0; `workspace_graph` `32 passed`; `check_architecture_fixtures` `35 passed`; `check-architecture` 0 at `packages=4 targets=36 files=70 edges=93`; `graphic_avif_inspection` `5 passed`; `media_worker` `16 passed`; `platform_asset_catalogue` (application) `7 passed`; `platform_asset_catalogue` (adapters) `3 passed`; host-smoke `--no-run` builds; `n125_02` also passes **locally** (24 refusals, real evidence line) because it needs no source filesystem; `cargo test --workspace --all-targets --locked` → **87 passed, 5 failed**, the five being the documented `source_read_native::snapshot_proofs` precondition panics (no verified disposable Btrfs `SOLO_TEST_SOURCE_FS_ROOT` on this WSL host). `n116_`/`n121_`/`n122_`/`n124_` and N125 cases 01/03/04 are therefore **NOT RUN LOCALLY**, and native Windows linkage (`link.exe`) is unavailable: all host evidence above is native CI only. The N125 verifier logic was exercised locally against a synthetic log in the printed shape — `authentic=accepted`, `mutation=rejected` — before the push, and the hosted logs then reproduced those verdicts natively. No `#[ignore]`, `--skip`, `|| true`, `continue-on-error`, exemption or fabricated token was introduced.

## 6. Repository state and the required Owner decision

`main` = A20 `964c169b` + this report; technical branch = `d219f6b8` (clean); **28** remote heads, one worktree, one `origin`, exactly four `.documents/WIP/` files, empty stash, no branch/tag/PR/merge/release/deploy, no force push, no `reset --hard`, no `git clean`. Task scratch files were removed from Solo `.temp/`.

**The product still has no installed approved Platform catalogue and no product artwork**, accepts **no** real Platform namespace source, and grants **no** publication rights. An R12 `COMPLETE` is a parser-and-binding milestone, never the charter's Owner-reviewed asset delivery. **rights_review_pending=true**, **stage03_open=true**.

**Required next Owner decision (not authorized here):** supply the actual lawful initial Platform artwork, or authorize a separately reviewed original-art creation package, and approve **each** intended asset id, exact bytes/SHA, format, intrinsic/display dimensions, licence, rights holder and supporting provenance. Only then can a later order install `platform/assets/`, pin the complete immutable `1.0.0` catalogue and provide a **separate native Platform root** — never tunnelled through `content/assets/`. No generated stand-in, copied Foundation artwork, guessed licence or self-certified approval may fill that gap.

**Stage 03 / `R03.C`: OPEN.** No release, deployment, publication, source fallback or closure is authorized or claimed by R12.

