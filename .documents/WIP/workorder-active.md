# SOLO-WO-118 R13 — PIN FOUNDATION DONOR ICON ARTWORK, IMPORT REVIEW-ONLY FIXTURES, AND PROVE AN 81-ASSET OWNER-REVIEW PACKET

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's single checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required bounded outcome:** `R03C-FOUNDATION-DONOR-81-TABLER-ICONS-IMMUTABLE-OWNER-REVIEW-PACKET-N126-TWO-HOST-V1`.

## 1. Binding decision, authority, and current pinned state

The Chief Master Architect independently **ACCEPTED SOLO-WO-118 R12** for a strict but **inert** catalogue parser and V4 fact-matching gate. `origin/main` at handoff is R20 `b1c557244d276d3d04db2010fac681c92d61170c`; `origin/solo-wo-118-avif-v4-r01` is T20 `d219f6b8943664f5e32eb4288edefb7ba3d3268c`. The accepted technical run `43736439423` attempt 1 has successful Ubuntu `132879959703`, Windows `132879959898`, and independent `avif-reference-oracle` `132879959950`. R20 main baseline run `43737572434` passed on both hosts. Current architecture = `packages=4 targets=36 files=70 edges=93`; this is a baseline measurement, not a frozen count after edits.

The Owner has now explicitly selected **`https://github.com/provelopment/provelopment-foundation`** as the **donor repository for actual artwork**, rather than commissioning new illustrations. This is authority to identify, copy **MIT-eligible generic artwork into review-only test fixtures**, establish immutable provenance and prepare exact asset-by-asset catalogue data. It is **not** authority to deem every donor graphic generic, claim third-party trademarks, deem original artwork appropriately licensed, install a usable product `platform/assets/` library, enable Platform reads, or publish/release/deploy. R03.C / Stage 03 stay OPEN.

**Independently verified immutable donor:** `provelopment/provelopment-foundation` default-branch source commit / recursive tree `fa599973a88ff77f6a8e012f3266c3c139b52c17`, recorded October 29, 2026. Never use mutable `main` as import authority. The donor root `LICENSE` is Apache-2.0 **for covered source code** and expressly does **not** grant rights to Provelopment names/logos or relicense third-party material. The specific neutral-icon collection separately carries Tabler Icons **v3.46.0**, `icons/outline`, **MIT**, with copyright `(c) 2020-2026 Paweł Kuna` and a complete notice. Preserve this MIT text when copying the icons.

**Pinned donor paths, exact Git blobs:**

- 84 SVG inventory: `deployment/spokes/foundation/content/assets/icon-library/icons/icon-*.svg`.
- Machine provenance: `deployment/spokes/foundation/content/assets/icon-library/licensing/icon-provenance.json`, Git blob `a74d234417d5bb63ff05e2b64f93d757864f3d46`.
- Tabler MIT notice: `deployment/spokes/foundation/content/assets/icon-library/licensing/TABLER-ICONS-MIT.txt`, Git blob `3e82379dab3fe93d9ee22251949604ed63ddea39`.
- Role dictionary: `deployment/spokes/foundation/content/assets/icon-library/icons/README.md`, blob `f002f2ab72aa7f02144f4acc2e5f8d41d8f8d8d4`; generic policy README: `deployment/spokes/foundation/content/assets/icon-library/README.md`, blob `e26e929197d9430e18be6c13714d0110ac532164`.

The donor records **83 Tabler/MIT files and one `icon-services.svg` original Provelopment work**, with two **byte-identical** artwork pairs:

1. `icon-about.svg` = `icon-status-info.svg`, both Git blob `c407077e4f849eaf536908a9ee444c4aa4752af4`;
2. `icon-clients.svg` = `icon-volunteer.svg`, both Git blob `b0d6ecc811b7e1fd4c6d98fc3a1a59e7224c54bc`.

The **R12 catalogue rejects duplicate SHA-256 aliases**. Do not evade this by altering donor graphics or weakening the catalogue invariant. Choose `icon-about.svg` and `icon-clients.svg` as the sole representative entries for these duplicates. Record `icon-status-info.svg` and `icon-volunteer.svg` as WITHHELD ALIASES with their actual donor IDs, same-byte evidence and no automatic substitution. Withhold `icon-services.svg` as OWNER-ORIGINAL/SEPARATE-RIGHTS-PROFILE until an explicit per-asset licence/distribution decision. Thus the **R13 review candidate is exactly 81 unique MIT Tabler icons**, not 84 and not an inferred catalogue approval.

**Exclude** all donor branded logos, brand/favicons, blank/identity placeholders, demonstration/synthetic assets, all `platform-marks/` social-service logos (third-party trademark policies, not blanket MIT), branded banners and unrelated code. In particular, do not import the Foundation `deployment/efs/.../assets/brand/` trees or any `deployment/spokes/*/content/assets/placeholders/` as Platform generic artwork. Do not use/derive copyrighted visual artwork from screenshots or UI code.

## 2. One-checkout transaction and immutable write ledger

Before doing anything, establish `git status --porcelain`, index, `origin/main`, technical head, `git worktree list`, `git remote -v`, `git stash list`, all remote heads, the four `.documents/WIP/` files, and absence of pre-existing untracked scratch changes. Expected 28 remote heads, one worktree, one `origin`, clean branches and empty stash. On unexpected state, **STOP**, report actual evidence; no cleanup, reset or forcing.

**A21:** Owner installs this exact ACTIVE into the existing local checkout on `main`. Commit and push **ACTIVE-only** A21 from R20 `b1c557...`. Prove byte-identical ACTIVE worktree/index/commit/`origin/main` blob, commit parent and only-path diff.

**T21.0:** On existing `solo-wo-118-avif-v4-r01` from `d219f6b...`, import ACTIVE byte-for-byte using the accepted single-path `git restore --source=A21` method, commit and push ACTIVE-only. No new branch, worktree, clone of Solo, alternate remote, tag or merge.

**T21 technical:** Commit **only** the following review-only paths on that same technical branch, with normal forward commits and an exact final technical SHA:

```
.documents/WIP/workorder-active.md                                       # T21.0 ONLY
.documents/Reports/platform-library-1.0.0-foundation-review.md         # NEW immutable owner review sheet
.documents/Reports/platform-library-1.0.0-foundation-candidate.json     # NEW R12-shaped review-only catalogue
crates/solo-cli/tests/fixtures/foundation-donor-icons/icons/            # NEW exactly 81 donor SVG blobs (one per eligible ID)
crates/solo-cli/tests/fixtures/foundation-donor-icons/licensing/        # NEW exact pinned MIT text + exact pinned icon-provenance.json
crates/solo-cli/tests/media_worker_host_smoke.rs                        # ADDITIVE mod n126 only
.github/workflows/rust-verify.yml                                       # ADDITIVE N126 Ubuntu/Windows evidence gates only
```

No product `platform/`, `platform/assets/`, product catalogue, extra fixture outside this ledger, second reader, modified codec, `Cargo.toml`, `Cargo.lock`, `deny.toml`, architecture checker, domain grammar, R12 parser/contract, V1–V4 code, prior N093–N125 test bodies, golden, Python oracle, package script, source data, WIP plan, or other file. If a real incompatibility requires a path outside this list, **STOP** and propose a separately Owner-authorized revision; do not quietly broaden the ledger.

**R21 main report:** After technical CI completes, return to `main` with A21, replace only `.documents/Reports/execution-report.md` as one report-only R21 commit, then obtain fresh native Ubuntu + Windows baseline green CI. Report the full R21 commit SHA in the Owner handoff (not inside its own file). This is not a technical merge; technical output remains on its branch for review.

## 3. Audit and exact-copy procedure — NO assumed rights or hidden transformations

Use a GitHub source download pinned to the **full donor commit `fa599973...`**, not a checkout of donor `main` and not the Owner's additional working tree. Use task-owned `.temp/` scratch **outside tracked paths** and clean only files this operation demonstrably created; never run `git clean`, `reset --hard`, branch deletion or force push. The donor is read-only. If source access is unavailable, **STOP**, do not silently replace the source with the pre-existing N116 generic fixture, a fetched other-version Tabler icon, generated art or a fake SHA.

The donor Git tree at that commit is the authoritative list of paths and blob SHAs. Fetch pinned source bytes, calculate `git hash-object -- <download>` (must exactly equal the donor's Git blob SHA) and SHA-256 (64 lowerhex) for **each** of the 84 original icons and both metadata files. Extract the exact 81 selected originals without newline normalisation, SVG cleaning, colour replacement, font embedding, recompression, comments, reflow, conversions or any byte mutation. Preserve source files as test-only fixtures under `foundation-donor-icons/icons/` with identical filenames. Pin the two metadata files byte-for-byte inside `foundation-donor-icons/licensing/`. Verify metadata blob SHAs above. Collect exact byte counts.

Programmatically reconcile the `icon-provenance.json` 84 canonical names against the 84 Git paths **in both directions**, every per-item source, version, `modified` value and declared licence; require `83 MIT + 1 original`, `84 present`, two exact duplicate pairs, and no unexpected same-SHA pair. Fail on one absent/extra record, changed source or undocumented licence. Confirm the MIT file actually contains its grant and copyright. Compare each selected SVG against its own provenance (`source_icon_name`, vendor version). The Git blob SHA is **not** a substitute for the 64-character SHA-256 catalogue field; record both separately.

No R13 process may infer permission for `icon-services.svg` from a repository licence; its provenance says `Provelopment original artwork` and its proposed rights require explicit Owner signoff. Social marks have their own brand-policy register and are excluded. Never treat the source repository's public visibility as proof of licence.

## 4. Review-only candidate catalogue and per-asset Owner decision sheet

Produce `platform-library-1.0.0-foundation-candidate.json` with the **exact** R12 wire contract and field spelling, parseable by `parse_platform_asset_catalogue_v1(selected_version="1.0.0")`:

```
{
  "platformAssetLibraryVersion":"1.0.0",
  "assets":[{
    "id":"icons/icon-home.svg",
    "sha256":"<SHA-256 of exactly pinned SVG source bytes, NOT its Git blob>",
    "mediaClass":"image",
    "mime":"image/svg+xml",
    "width":24,
    "height":24,
    "licence":{
      "expression":"MIT",
      "rightsHolder":"Paweł Kuna",
      "evidence":"Foundation pinned Tabler MIT notice at fa599973...; source licensing/TABLER-ICONS-MIT.txt"
    },
    "provenance":{
      "source":"Tabler Icons v3.46.0, icons/outline; canonical name icon-home.svg; source home",
      "evidence":"Foundation fa599973... icon-provenance.json; exact donor Git blob <sha>"
    }
  }]
}
```

The object above is a **shape example only**; its placeholder SHA **MUST NOT** enter the produced file. For every selected asset use the actual authoritative Tabler `source_icon_name` (not guessed from canonical filename), exact donor blob and SHA256, with bounded 1–512-scalar text and exact ID `icons/<canonical filename>`. The entries must be in deterministic ascending ID order and have 81 unique SHA256 digests. Keep exact selected source bytes in test fixtures separate from the prospective production destination. Do not set `approved`, `published`, `installed`, `rightsReviewed` or other invented trust keys. The catalogue must pass the R12 strict parser/typed invariants; its existence is **not** an installed Platform catalogue.

The Markdown Owner review sheet must contain **81 individually reviewable rows**, with at least: checkbox/approval blank, catalogue ID, donor exact source path, immutable donor commit, original Git blob SHA, source SHA-256, exact source size, width/height, canonical MIME, MIT/version/rights holder, provenance source-icon name, independent licensing/metadata paths, actual confined SVG sanitizer classification and canonical-output hash (when available), plus a stable link to the exact pinned donor source. Explicitly mark the entire sheet **PENDING OWNER PER-ASSET APPROVAL — NOT DISTRIBUTABLE**. Include a separate withheld table containing both duplicate aliases, the original `icon-services.svg`, each with cause and review alternative. Exclude all brand/placeholder/social assets by category, not by pretending they are missing. Include a rights and NOTICE reproduction plan (preserve Paweł Kuna 2020–2026 MIT notice in an eventual installation/distribution; no silent re-licensing under the root Apache grant).

Record 81 exact source hashes and evidence **without a disclaimer that substitutes for missing facts**. Any ambiguous SHA, byte modification, absent provenance or licence mismatch makes this order `STOPPED`, not an approval.

## 5. N126 native evidence — review-only real SVG worker, no platform-source impersonation

In existing `crates/solo-cli/tests/media_worker_host_smoke.rs`, append **one new `mod n126`**. Use the already-pinned real `solo-cli` binary and `inspect_svg_bytes_with_executable_v1` / accepted SVG confined supervisor. Do not construct a fake observation to satisfy this order. Read the 81 actual **tracked review-only fixtures** as test data, not via a Platform source reader; plainly state `source_scope=review-fixture` in the evidence. Each accepted SVG must use the existing strict sanitizer and one genuine confined child invocation (81 actual child invocations per host); prove its original SHA-256 equals the independent candidate catalogue entry, its real SVG width/height equal 24×24, its canonical output is nonempty, bounded and stable, and its source-file / licence provenance is exactly represented. Never treat worker's own claims alone as rights verification. If any source is refused, retain its exact filename and refusal **in private CI diagnostics only**, fail and `STOP`; never modify SVG or weaken sanitizer to force acceptance.

Required three focused tests, with exactly the names and intent below:

1. `n126_01_exact_eighty_one_tabler_donor_vectors_real_confined_workers` — all 81 original SVGs, 81 genuine supervised invocations, 81 **observed nonzero child PIDs; do not assert global PID uniqueness because OS PID reuse is possible**, source hashes agree with 81 catalogue entries, width/height 24×24, canonical output verified and deterministic; report an **aggregate hash** of sorted `id + source_sha + canonical_sha` records, identical across hosts. Do not print rights-holder paths or private source data. Marker: `n126_evidence case=01 originals=81 inspected=81 real_children=81 geometry=24x24 donor_pinned=true source_scope=review-fixture publication=false` plus aggregate digest.
2. `n126_02_pinned_mit_provenance_and_r12_catalogue_review_only` — strict R12 parse of 81-entry JSON, exact `1.0.0`, exact metadata hashes, 83/1 donor reconciliation, two aliases withheld, original withheld, one normalized static MIT notice, no product Platform tree, no approved rights flag, no Platform source. Marker: `n126_evidence case=02 entries=81 mit_covered=81 originals_withheld=1 duplicate_aliases_withheld=2 review_only=true platform_read=false publication=false`.
3. `n126_03_forged_source_alias_missing_notice_and_platform_authority_refused` — independently tamper a selected fixture SHA, reinsert duplicate aliases as catalogue entries (must refuse `DuplicateDigest`), reintroduce original/unreviewed or social asset (test selection verifier refuses), omit the MIT notice, corrupt provenance source/version, change selected version, and attempt a `PlatformSource` R12 binding (must refuse). Never write to tracked assets as a mutation mechanism: mutate test-owned in-memory data only. Marker: `n126_evidence case=03 forgeries_refused=true alias_digest_refused=true licence_missing_refused=true donor_drift_refused=true platform_unavailable=true zero_authority=true`.

If 81 simultaneously installed entries cannot be matched because worker-canonical colors differ from donor `currentColor`, do **not** misrepresent: the worker's canonical SVG may deliberately resolve `currentColor` to fixed black. Report the source SHA over **unmodified bytes** and the independent canonical output SHA separately. Never change source SHA to the canonical digest. No automatic colour change or asset alias is authorized.

The **existing R12 `PlatformSourceUnavailable` refusal and `read_graphic_asset_bytes_v1` Platform refusal stay binding**. The test-only fixtures and JSON must not be mistakenly included in a shipped product library, release-artifact contents, runtime asset glob or live read path. The legacy V3 `avif-unsupported-v3` refusal and all N093–N125 tests are immutable.

## 6. Two-host CI / mutation gate and local honesty

Extend existing `rust-verify.yml` with only **two** N126 host stages (Ubuntu and Windows). Both must run `cargo test -p solo-cli --test media_worker_host_smoke --locked n126_ -- --nocapture --test-threads=1` against genuine compiled binary, require `3 passed; 0 failed; 0 ignored`, one case name and one `n126_evidence` line for each, exact 81/81 and review-only tokens, and the matching aggregate digest. Emit `n126_gate authentic=accepted` only after checking a **real captured** log. Use the **same verifier** on at least four deliberate corrupted copies (missing case, bad count 80, publication/platform_read forged true, wrong donor hash or licence token); emit `n126_gate mutation=rejected` only after *all* are rejected. Do not fabricate an expected log; local synthetic checker preflight is not host evidence.

Keep all existing native Btrfs/Windows snapshot-source tests, N093–N125, reference oracle job, security/architecture, cargo-deny/RustSec refresh, release/reproducibility steps and log-mutation gates unchanged. At **one final technical SHA**, require three successful jobs: native Ubuntu, native Windows, independent `avif-reference-oracle` (same exact R03 pin). Also obtain fresh R21-main two-host baseline after report publication. Compare N126 aggregate across hosts from actual logs, and record exact job IDs, attempts and source/target SHAs.

Before pushing, run format/check/clippy, graph/architecture fixtures, relevant catalogue tests, new N126 tests **where host supports them**, unchanged smoke builds, and the full workspace suite. An unverified WSL Btrfs filesystem must continue to fail honestly (`NOT RUN LOCALLY` for real-source tests), not be reclassified as green. Windows linker absence locally is not a waiver of native CI. Never use `--skip`, `#[ignore]`, `continue-on-error`, `|| true`, fake tokens or an unconfined fallback. If any genuine host gate fails, correct forward under the fixed ledger with actual evidence or publish STOPPED; no repeat-until-green without diagnosis.

## 7. R21 canonical report; acceptance boundary and Owner next step

R21 `.documents/Reports/execution-report.md` must include exact two-repository provenance (Foundation pinned commit/tree, 81 selected donor IDs, 81 raw hashes, MIT notice and provenance blob SHA), 84→81 exclusion proof, all changed paths, test-only import decisions, the strict JSON and review sheet blob SHAs, genuine 81-worker evidence on *both* hosts, four mutations, prior gate outcomes, architecture measured change, baseline workflow and clean worktree/stash/remote checks. Record honest failures and corrections, no right-clearance overstatement.

Set `status=COMPLETE` **only** for `R03C-FOUNDATION-DONOR-81-TABLER-ICONS-IMMUTABLE-OWNER-REVIEW-PACKET-N126-TWO-HOST-V1` when all these gates pass. In the same report state: `owner_per_asset_approval_pending=true`, `rights_review_pending=true`, `platform_library_delivered=false`, `platform_read_enabled=false`, `publication_authority=false`, `stage03_open=true`. R13 **does not** close R03.C or Stage 03. If source/rights/compatibility/gate evidence fails, `status=STOPPED` and precise counterexample, not a fabricated success.

**Required subsequent Owner action:** Review the per-asset 81-row packet (or approve an explicitly reduced subset) and confirm the exact selected bytes, per-asset licensing/provenance and redistribution scope for the first pinned `platformAssetLibraryVersion: "1.0.0"` release. Only the *next* Owner-issued order after that review may install `platform/assets/`, its complete hash-pinned immutable catalogue, and a **separate native read-only Platform root**, prove N127 real approved Platform reads and derive/publish nothing prematurely. Withheld brand marks, original `icon-services.svg` and duplicate aliases require their own explicit decisions.

**Architectural stop rule:** no new trust source is created by a Git URL, a right-hand licence string, a passing SVG decoder or the Owner's selection of a donor repository. Those inputs remain evidence for an informed, specific per-asset approval. GitHub connector use by the Chief Master Architect is READ ONLY; implementation through the Owner's existing checkout is exclusively the Implementation Agent's job.
