# SOLO-WO-118 R12 — CLOSED PLATFORM ASSET CATALOGUE CONTRACT AND INERT V4 FACT-BINDING, OWNER-REVIEW READY

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's single checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required bounded outcome:** `R03C-PLATFORM-ASSET-CATALOGUE-STRICT-INERT-N125-TWO-HOST-OWNER-REVIEW-READY-V1`.

## 1. Binding architectural decision and accepted baseline

The Chief Master Architect **ACCEPTS `SOLO-WO-118 R11`** for the inert six-format V4 whole-candidate inspector, **not** for a Platform source library, catalogue, publication, `R03.C` or Stage 03 closure. Independent evidence:

- `origin/main` at canonical R19 **`a69486c7530e00f7006ee87ba6eeea4ad2785dba`**; technical `solo-wo-118-avif-v4-r01` at **`5d35c89281b27c418f0d2c1967b912e1ac5fea1c`**.
- Technical push run **`43730461195`**: Ubuntu 24.04 `132861527493`, Windows 2025 `132861527258`, and independent `avif-reference-oracle` `132861527526` all **success**, with authentic `N124` **4/4** and both N124 mutation gates rejecting corruption. Architecture remains `packages=4 targets=34 files=66 edges=93`. First technical run `43729631588` at `ee1878b2e40034e95a73afa85767817ab0e01e6f` failed; later commit corrected test declarations and assertions forward. `main` R19 run **`43731508886`**, Ubuntu `132864743659`, Windows `132864743935`, both success.
- R19 only introduced the distinct V4 orchestration and its tests; V3's `avif-unsupported-v3` refusal, the native read boundary, six accepted workers, R03 independent goldens, N116/N120–N124 and publication-denial constants are retained.

**Authoritative Charter** `.documents/Architecture/ARCHITECTURE.md` §6.2A.3, §6.2B root, §6.2G and especially §6.2I.1–6:

1. Entity sources resolve **only** under `content/assets/`; the separate installed, product-distributed, read-only Platform source library is **`platform/assets/`**, never an EFS owner, alternate Installation, fallback or second publisher. An authored asset uses the closed `{ "namespace": "entity"|"platform", "reference": "…" }` shape; `graphic.favicon` stays PNG-only.
2. The manifest's `platformAssetLibraryVersion` is **exactly** `"1.0.0"` for the first accepted profile, independent of other versions. A usable Platform library needs an immutable, **Owner-reviewed lawful initial catalogue** enumerating **every** available ID (the exact portable relative filename), SHA-256, media class, dimensions, licence and provenance. Unknown/missing versions or IDs never fall back or auto-upgrade.
3. Every asset's bytes and actual media evidence must agree with its declared source, format/magic, limits and catalogue record. Publication authority and immutable derived outputs require later distinct gates. Section 6.2I does **not** mandate any artwork, file name, catalogue JSON field spelling or permission to invent licensing.

**Independent repository finding:** the current R19 tree contains no installed `platform/assets/` artwork and no accepted Platform catalogue. The accepted `read_graphic_asset_bytes_v1` still rejects Platform obligations as `ASSET_PLATFORM_UNVERIFIED`; V4 is a six-format **Entity**-source inspector. This is correct and **must remain** until later explicit rights and native Platform-acquisition orders.

**R12 decision:** build a **closed, bounded, inert catalogue parser/validator and an independent V4 evidence-matching gate** suitable for later Owner review. Use test-owned synthetic bytes and catalogue records only. **Do not** generate or claim an approved production library, legal clearance, real Platform source acquisition, derived assets, publication or Stage 03 closure. No `platform/` product tree or package-artwork file is authorized in R12. A synthetic catalogue passing tests does **not** establish that any real artwork is lawfully distributable.

## 2. Transaction, branch discipline and immutable ledger

Begin by verifying exact refs, worktree and index. One checkout, one worktree, one `origin`, 28 remote heads, exactly four files in `.documents/WIP/` (`plan-next.md`, `plan-outlook.md`, `plan-roadmap.md`, `workorder-active.md`), empty stash, clean `main` and technical branch. If any precondition differs, **STOP** with evidence; never reset, clean, force-push, cherry-pick unrelated code or silently alter authority.

**A20 / T20 / R20 transaction:**

1. Owner installs this downloaded file at the canonical ACTIVE destination. On `main` commit and push **A20**, **ACTIVE only**, parent R19 `a69486c7530e00f7006ee87ba6eeea4ad2785dba`. Prove working-tree/index/commit/`origin/main` ACTIVE blobs identical; record SHA, tree and blob.
2. Return to existing technical branch **`solo-wo-118-avif-v4-r01`** at T19 `5d35c89281b27c418f0d2c1967b912e1ac5fea1c`; import **A20 ACTIVE only** as **T20.0**, proving exactly one staged path and identical blob. Do not merge main into the technical branch.
3. Implement the bounded R12 catalogue logic and tests on the technical branch. Run local-first checks before each technical push, push normal forward commits, obtain three green jobs at **one exact final T20 SHA**. Preserve the first failed attempts and actual diagnostics if any.
4. Back on `main` after A20, publish the **canonical** `.documents/Reports/execution-report.md` as **R20**, report-only commit. Run fresh baseline two-host CI; report actual results. R20's own full SHA goes in Owner chat handoff, not in its own committed report.

**Only these code paths may change (or be added as explicitly marked):**

```
.documents/WIP/workorder-active.md                          # A20 + T20.0 only
crates/solo-application/src/platform_asset_catalogue.rs    # NEW — pure typed invariant gate
crates/solo-application/src/lib.rs                          # export exact new API
crates/solo-application/tests/platform_asset_catalogue.rs  # NEW — pure positive/negative tests
crates/solo-adapters/src/platform_asset_catalogue.rs       # NEW — strict JSON parser; no native reader
crates/solo-adapters/src/lib.rs                             # export exact new parser
crates/solo-adapters/tests/platform_asset_catalogue.rs     # NEW — bounded parsing/negative tests
crates/solo-cli/tests/media_worker_host_smoke.rs            # additive N125 cases only
.github/workflows/rust-verify.yml                           # additive two-host N125 gates only
.documents/Reports/execution-report.md                     # R20 on main only
```

The catalogue parser may use **already-locked** `serde`, `serde_json` and the existing hash routine. **No Cargo.toml, Cargo.lock, deny.toml, cargo-deny policy, new crate, new unsafe or feature change**. Do not change `source_read.rs`, `source_read_native.rs`, the `GraphicAssetReadPort` or `read_graphic_asset_bytes_v1`, any V1/V2/V3/V4 inspector, decoder, child, supervisor, protocol, CLI production source, domain registry, product fixtures, R03/R08 Python, immutable goldens, architecture scripts, SOP, preexisting N093–N124 bodies, WIP plans or report outside its canonical handoff. Keep `platform/assets/` absent as product content. New module and integration-test files change target/file/edge measurements: **record actual architecture output**; do not hard-code `34/66/93` as if no files were added, weaken the graph checker or change its policy.

## 3. Proposed closed `1.0.0` catalogue contract — parse syntax, not rights approval

Choose and document one **provisional**, review-eligible catalogue JSON shape. The exact field names below are an **R12 implementation proposal**, not claimed to be separately Owner-approved canonical library artwork. Use them consistently inside the typed model and strict parser; if technical constraints force a compatible spelling correction, report the difference, retain equivalent meaning, and never infer legal approval.

```
{
  "platformAssetLibraryVersion": "1.0.0",
  "assets": [
    {
      "id": "icons/example.png",
      "sha256": "<64 lowercase hex SHA-256>",
      "mediaClass": "image",
      "mime": "image/png",
      "width": 2,
      "height": 2,
      "licence": {
        "expression": "<nonempty precise licence identifier or expression>",
        "rightsHolder": "<nonempty asserted holder>",
        "evidence": "<nonempty reviewable rights evidence reference>"
      },
      "provenance": {
        "source": "<nonempty source/creation description>",
        "evidence": "<nonempty reviewable origin record reference>"
      }
    }
  ]
}
```

**Binding parser invariants:**

- Input is already-owned bytes, UTF-8 without BOM, **≤2 MiB**, exactly one JSON object. Reject malformed syntax, duplicate keys **at every object depth**, unknown keys, nulls, missing fields, wrong types, trailing data and more than **1024** entries. `serde_json::Value` parsed via ordinary duplicate-overwriting semantics is **not** a proof; use a strongly typed `Deserialize` shape with `deny_unknown_fields` and duplicate-field detection, or an equivalent explicit duplicate-rejecting visitor. Never accept YAML, JSON-with-comments, case-folded IDs or alias schema fields.
- `platformAssetLibraryVersion` must be the exact requested `"1.0.0"`; any other value, missing requested version or absence of an explicitly supplied trusted requested version refuses. No “latest”, major/minor matching, discovery, version migration, default version or fallback. The validator takes the selected version as an **independent trusted argument**, not guessed from the catalogue itself.
- `assets` is **nonempty** for a candidate product catalogue; test-only in-memory catalogues may contain single entries but cannot be installed or published. Every exact `id` must be a closed §6.2C / domain portable-relative path, **≤240 UTF-8 bytes, ≤8 segments**, with an image file extension; no leading slash, empty/dot/dotdot segment, percent-/backslash-/control-based ambiguity, reserved name, trailing dot/space, absolute drive, platform/Entity prefix, manifest/catalogue filename collision, case-insensitive alias or duplicate ID. Reuse existing domain parsing without using an Entity namespace to authorize Platform IO; the plain grammar is shared, the namespace authority is not.
- `sha256` exactly **64 lowercase hexadecimal digits** and not all-zero; duplicate SHA may be legal only if distinct exact IDs are explicitly accepted as distinct entries and are not silently aliased (preferred for this first profile: reject duplicate hash aliases to keep provenance unambiguous; document the policy). No unpinned hash algorithm.
- `mime` and extension must agree **exactly** with the six accepted V4 image classes (`image/png`, `image/jpeg` for `.jpg`/`.jpeg`, `image/webp`, `image/gif`, `image/svg+xml`, `image/avif`); `mediaClass` is exactly `"image"` for this image-only R12 slice. Video and VTT belong to future separately approved media inspectors and **refuse** here rather than pretending coverage. The role's PNG-only favicon rule is not relaxed. Never infer format from a catalogue string when source-byte inspection contradicts it.
- `width` and `height` are positive integers ≤8192; checked `width × height` ≤32,000,000. Reject fractions, negative numbers, overflow, zero, impossible values and format/size mismatch; do not supply a default.
- `licence.expression`, `licence.rightsHolder`, `licence.evidence`, `provenance.source` and `provenance.evidence` must be nonempty bounded text (e.g. 1–512 Unicode scalar values; reject control/NUL/URL-fetch instruction). **Syntactic nonemptiness is not proof of licence or provenance truth**; the Owner must inspect primary evidence and approve the actual asset plus its proposed legal attribution separately. Do not accept an `approved=true` literal or create a self-certified approval token from these strings. For test-only examples, mark the test evidence as synthetic in code comments and logs and never present it as a real rights assertion.
- Canonical typed catalogues preserve each entry exactly once, in declared order, with no implicit extra sources. No directory scan to supply missing entries, no file-name-based ownership, case-insensitive lookup, implicit library-root adoption or use of the catalogue as a second publisher.
- Every refusal is a **closed, path-free stable category** (`InvalidSyntax`, `VersionMismatch`, `InvalidId`, `DuplicateId`, `InvalidDigest`, `InvalidMediaClass`, `InvalidDimensions`, `MissingRightsEvidence`, etc. as appropriate). The `Debug`/public-facing explanation contains no asset path, licence holder, evidence text, source location or untrusted JSON fragment. Retain attribution in an internal typed context only if needed. No partial parsed set may be returned after one bad entry.

**Important:** the catalogue is an assertion made by its author. Even a valid parsed entry must be cross-checked against a separately inspected authoritative source. The validator **never** grants media correctness, publication, rights clearance, immutable distribution or platform-source authority.

## 4. Inert linkage to the already accepted six-format V4 evidence

Implement a pure application operation, e.g. `match_catalogue_entry_to_v4_evidence_v1`, which consumes an **explicitly selected** typed catalogue entry and one **already checked** `GraphicImageInspectionV4` record. It neither opens bytes nor starts a worker. Its result may be `MatchedForReview` or a closed refusal; it must never be named or treated as `Approved`, `Licensed`, `Publishable` or `PlatformRead`.

The matcher independently checks, at minimum:

1. The exact requested asset ID is present once; the requested namespace is explicitly Platform at the intended later call site (a test-only adapter may bind to an Entity fixture record **only for content-fact comparison**, never represent that as a real Platform acquisition). No name similarity, basename match, guessed equivalent, case folding, alternate extension, fallback, preferred/default asset or directory enumeration.
2. The catalogue SHA-256 equals the parent-computed V4 **source** digest; never confuse an AVIF plane/output digest, a GIF timeline digest, a canonical SVG output digest or a decoded PNG/JPEG/WebP digest with the source SHA. A matching source hash is necessary, not sufficient.
3. `mime`, `mediaClass`, `width` and `height` agree with the V4 record's actual format/MIME/intrinsic or displayed bounds. For accepted associated-`irot` AVIF the V4 displayed shape is **256×512**, not coded 512×256. For GIF retain the timeline, for SVG retain the original V3 canonical-vector facts; do not rasterize/reinterpret them. The V4 worker/supervisor evidence must remain genuine; no newly trusted data is taken from the catalogue to override observations.
4. A negative, missing, ambiguous, invalid or unverifiable entry returns **one refusal, zero retained matches** for the whole requested set; a later mismatch discards all earlier matches and does not leak private identifiers. Keep any effective-public projection disabled in R12.
5. The operation's authority flags (if defined) remain constant `false`: `grants_publication_authority`, `proves_media_correctness`, `rights_reviewed`, `platform_source_readable`, `derived_output_verified` and `release_ready`. Avoid boolean methods whose names falsely imply approval. Tests prove that a catalogue cannot manufacture authority not present in the inspected record.

**Boundary firewall:** **do not connect this provisional catalogue to `read_graphic_asset_bytes_v1` or `inspect_entity_graphic_images_v4`** and do not make Platform references readable in R12. The future authorized Platform reader must have its **own explicitly designated read-only product root** and must reuse the accepted native atomic no-follow / immutable-source mechanism without tunnelling through `content/assets/`; that is a subsequent work order after Owner rights/artwork review. On R12 the real Entity source reader remains an explicitly proved refusal for `AssetNamespace::Platform`.

## 5. Exactly four N125 real-host cases; pure tests and deliberate mutation gate

In existing `crates/solo-cli/tests/media_worker_host_smoke.rs` add **only** `mod n125` with exactly these names. Reuse the accepted `n116::Root` disposable fixture, `test_inventory_v1`, real source reader, real `solo-cli` confined workers, and V4 factory for the cases that bind source facts. No new codec or filesystem mechanism. Test catalogue bytes are **in-memory, test-owned, synthetic and explicitly unapproved**; they are not added to `platform/assets/`. `n125_` requires the verified Btrfs `SOLO_TEST_SOURCE_FS_ROOT` on Linux and native Windows reader, exactly as N124 did.

**`n125_01_six_format_catalogue_matches_real_v4_evidence_for_review`** — build one strict, bounded **synthetic** candidate with six uniquely named image entries matching a genuine six-format V4 candidate across five EFS levels. Obtain the real V4 records through six genuine confined children (one each PNG/JPEG/WebP/GIF/SVG/associated-alpha AVIF) and the real Entity test root, then run the new **inert** catalogue entry-to-V4 matcher against the parent-observed source SHA, exact MIME and dimensions. Make the provisional Entity-vs-Platform distinction **explicit in the test**: the match is content-fact comparison only, **not Platform acquisition**. Assert exact source identities, SVG/GIF typed facts, AVIF displayed geometry and no publication/rights authority. Marker `n125_evidence case=01 ... records=6 distinct_pids=6 review_only=true platform_read=false publication=false` after all assertions.

**`n125_02_closed_catalogue_parser_and_version_refusal_matrix`** — at least 18 independently instantiated hostile catalogue inputs: bad/missing version, wrong selected version, duplicate IDs, case-colliding IDs, traversal, reserved segment, uppercase extension, missing ID, malformed hash, uppercase/non-hex/all-zero digest, unsupported MIME/class, wrong extension/MIME pair, zero/oversize/overflow geometry, missing licence/provenance evidence, duplicate JSON field nested in rights metadata, unknown field, BOM, trailing extra JSON and oversize body. Each must produce a stable typed refusal with no partial output and a path-free public/debug string. Preserve no-fallback and exact-version behavior. Marker `n125_evidence case=02 ... refusals>=18 duplicate_keys=refused version_mismatch=refused approval=false`.

**`n125_03_late_inert_catalogue_mismatch_discards_entire_candidate`** — six authentic V4 results, first five fact-matched, last catalogued AVIF entry carries a **forged** source hash / displayed dimensions / MIME in three separate runs. The matcher must fail the whole catalogue binding, return **zero partial matches**, and not expose an `internal` suppressed declaration's asset ID/path/rights details. Also test source digest vs plane/output digest confusion and never-upgrade/fallback when one requested ID is absent. Marker `n125_evidence case=03 ... zero_partial=true private_path_absent=true source_hash_not_plane_hash=true fallback=false`.

**`n125_04_real_entity_reader_still_refuses_platform_and_no_unapproved_library`** — construct the actual `AssetReferenceV1` with `namespace=platform`, exercise the unchanged `read_graphic_asset_bytes_v1` or real reader through its ordinary accepted call path, assert typed `PlatformUnverified` refusal **before any source read or child launch**, even when an Entity asset of the same basename exists. Demonstrate no product `platform/assets/` or `1.0.0` production catalogue is introduced by this order (a repo-tree test or deterministic checked-file assertion is fine, not runtime directory discovery as availability). Under absent/unapproved catalog state the runtime must remain unusable, not partially ready. Marker `n125_evidence case=04 ... platform=unverified entity_alias=false worker_launches=0 publication=false`.

Add **focused unit/integration tests** in the two new crate test files: typed constructor invariants, strict parser duplicate/unknown rejection, all refusal tokens, order preservation, zero partiality, pure matching on real V4 records or purpose-built fake records labelled as fakes. No test may turn an asserted licence string into legal approval. No `#[ignore]`, test-only product bypass, invented screenshot/oracle SHA or suppressed failing result.

**CI:** add load-bearing `N125` Linux and Windows steps after existing N124 steps and Linux disposable Btrfs fixture provisioning. Run the authentic command:

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked n125_ -- --nocapture --test-threads=1
```

Each host must prove **exactly `4 passed; 0 failed; 0 ignored`**, all four test names exactly once, one evidence line for each case, `review_only=true platform_read=false publication=false`, `zero_partial=true`, refusal and absent-approval markers. Use an interleaving-safe checker modeled on N124, **not** a fragile `... ok$` assumption. The very same checker must reject **at least four deliberately mutated copies of the live log**: deleted case, changed success count, forged `platform_read=true`/`publication=true`, and corrupted duplicate-key/version or `zero_partial` refusal evidence. Print exactly `n125_gate authentic=accepted` and `n125_gate mutation=rejected` only after these real checks. Existing N093/N102/N108/N113/N116/N120–N124, other mutation gates, `avif-reference-oracle`, Btrfs, permission/snapshot, cargo-deny fresh RustSec, architecture and release/reproducibility stages remain load-bearing and unedited.

**Mandatory nonnegative control:** R12 should **not** claim a production Platform reader, catalogue inclusion completeness against real product files, legal approval or publication. The test-only synthetic catalogue must be incapable of being discovered as shipped `platform/assets/` data. An authentic `n125_gate` passing means only that the proposed schema and inert V4-fact binding are implemented and falsifiable.

## 6. Local-first gates and hosted proof

Run before every technical push in the one original worktree:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-application --test platform_asset_catalogue --locked
cargo test -p solo-adapters --test platform_asset_catalogue --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked n125_ -- --nocapture --test-threads=1
cargo test --workspace --all-targets --locked
```

If WSL lacks the verified Btrfs fixture and native Windows link tools, `n125_` and snapshot-dependent positives are **NOT RUN LOCALLY**, **not** skipped, waived or reported as passed; the already checked non-host tests are still mandatory. Local host-specific expected failures may be recorded honestly with their precise cause. The two **native** hosted verify jobs and the independent AVIF oracle must all pass at one exact technical SHA. No `continue-on-error`, `|| true`, `#[ignore]`, cargo-deny exemption, insecurity fallback, silent test filtering or malformed-probe acceptance.

Architecture expected to retain **four crates** and inward-only edges. Because R12 explicitly adds two Rust modules and two test targets, the observed target/file/edge counts may differ from T19; record the actual command output, keep its enforcement intact and prove every new edge legal. Do not edit the architecture checker to accept an otherwise forbidden dependency. Do not widen any V4 worker or native reader contract. No technical code on `main`.

## 7. R20 canonical report and Owner handoff

Publish `.documents/Reports/execution-report.md` as a report-only main commit R20, with **`status=COMPLETE` only for the bounded R12 strict-inert catalogue validation outcome** if and only if:

1. A20/T20.0/R20 transaction and exact clean two-branch state proven; change ledger contains only allowed paths.
2. Parser rejects the full closed hostile matrix without duplicate-key overwrites, source/rights authority inflation, partial records or version fallback; pure V4 matching checks actual digest, MIME, dimensions and class.
3. **All four** N125 real-host cases and the deliberate mutated-log rejection succeed on **both** native hosts with copied authentic lines and real PIDs, at the same final T20 SHA. All prior N116/N120–N124 tests and mutations remain green.
4. Three technical jobs succeed at one full T20 SHA; newly added modules preserve architecture, strict deny, safe code, full locked graph and release reproducibility; fresh R20 `main` baseline Ubuntu + Windows jobs pass.
5. The product still has **no installed approved Platform catalogue or product artwork**, accepts **no real Platform namespace source**, and grants **no publication rights**. An R12 `COMPLETE` status is never portrayed as the charter's Owner-reviewed asset delivery. Explicit `rights_review_pending=true` and `stage03_open=true` must appear in the report.

Otherwise publish **`status=STOPPED`** with the exact unsatisfied evidence and least permissible proposed next change. The canonical report must include commit/blob/tree SHAs, complete file changes, command outcomes, N125 actual host log excerpts and mutation results, three job IDs/attempts, the known WSL constraints, and clean working-tree/stash and remote-head checks. The Owner handoff message supplies R20's actual SHA separately.

**Required subsequent Owner decision (not authorized by R12):** choose and supply the **actual** lawful initial Platform artwork, or authorize a separately reviewed original-art creation package; approve **each** intended asset ID, exact source bytes/SHA, format and intrinsic/display dimensions, licence, rights holder and supporting provenance. Only after that approval can a later work order install `platform/assets/`, pin its complete immutable `1.0.0` catalogue, provide a **separate native Platform root** and bind public/derived-output gates. Do not fill this gap with generated stand-ins, copied Foundation artwork, a guessed licence or a self-certified approval.

**Stage 03 / `R03.C`: OPEN.** No release, deployment, publication, source fallback, automatic generic substitution or closure is authorized by R12.
