# Execution Report

**Work order:** `SOLO-WO-118 R11` — compose the six existing confined graphic formats into one whole-candidate V4 inert inspection.
**Required outcome:** `R03C-SIX-FORMAT-REAL-CONFINED-WHOLE-CANDIDATE-V4-N124-TWO-HOST-VERIFIED`.

**status=COMPLETE** for that bounded outcome only. One new V4 whole-candidate use case inspects all six declared formats over one real confined source read — PNG/JPEG/WebP/GIF/SVG through the **unchanged V3 port** and AVIF through the **R10 confined V1 port** plus the existing trusted-parent check — returning either one complete typed six-format set or one attributed refusal with zero partial records. **No Platform catalogue, publication, release, R03.C or Stage 03 closure is claimed.**

## 1. Transaction

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` = `52d243e74ec172ca658fd262a3fea5c418dbab7e` (R18), branch = `f4dcf9c2` (T18) | one `origin`, one worktree, 28 remote heads, four WIP files, empty stash, clean trees |
| **A19** (`main`) | `5b35cb875dd611c2a6c562f42a065d016d3f578a` — tree `7fcd94b213dcd82a9a52cddb43b2a32ff8936e71`, parent `52d243e7` | ACTIVE blob `1af4f75d1b5069fd7e419fcf9a61ee7b7e8452d4` identical in worktree, index, A19 and `origin/main` |
| **T19.0** (branch) | `cbb7fb2621f889d6ede8f69f8233ea561e4786d3`, parent `f4dcf9c2` | single-path `git restore --source=A19`; index blob identical; staged set = ACTIVE only |
| **T19** (branch, intermediate) | `ee1878b2e40034e95a73afa85767817ab0e01e6f` | first technical commit: `8 files changed, 1727 insertions(+), 16 deletions(-)`; run `43729631588` **failed** (see §5) |
| **T19** (branch, final) | `5d35c89281b27c418f0d2c1967b912e1ac5fea1c` | repair commit; push run **`43730461195` attempt 1** — see §2 |
| **R19** (`main`) | this report's commit (SHA in chat) | parent `5b35cb87`, exactly one path changed |

Final technical run `43730461195` attempt 1 at `5d35c892`: `verify (ubuntu-24.04)` `132861527493` ✅, `verify (windows-2025)` `132861527258` ✅, `avif-reference-oracle` `132861527526` ✅ — one exact SHA, no rerun, no `workflow_dispatch`.

Changed paths (exactly the ledger): `.documents/WIP/workorder-active.md` (T19.0 only), `crates/solo-application/src/graphic_image_inspection.rs`, `crates/solo-application/src/media_worker.rs`, `crates/solo-application/src/lib.rs`, `crates/solo-application/tests/media_worker.rs`, `crates/solo-adapters/src/graphic_image_inspection.rs`, `crates/solo-adapters/src/lib.rs`, `crates/solo-cli/tests/media_worker_host_smoke.rs`, `.github/workflows/rust-verify.yml`. All Cargo manifests, `Cargo.lock`, `deny.toml`, `oxideav-avif =0.0.18` (`fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee`), the supervisor, protocol, source reader, CLI main, R03/R08 fixtures and every N093–N123 test body are byte-identical to T18 — **zero lockfile or dependency drift**.

## 2. The V4 product change

**Application contract** (`graphic_image_inspection.rs`, re-exported through `lib.rs`). New V4 names only; every V3 export, discriminant, token and the historic `avif-unsupported-v3` refusal is untouched.

- `inspect_graphic_image_source_set_v4(sources, legacy: &impl GraphicImageInspectPortV3, avif: &impl MediaWorkerPortV1) -> GraphicImageInspectionOutcomeV4` iterates `evidence()` **once, in original order**, including `internal` and representation-suppressed obligations, and dispatches on `GraphicImageFormatV3::declared_by(reference)` alone — never on the byte signature. Zero/oversize lengths are refused before any port call; role grammar (`graphic.favicon` PNG-only) is applied before any child exists.
- The AVIF arm is the accepted R10 sequence: `MediaWorkerRequestV1::for_source` on the same acquired bytes, exactly one `inspect_avif` call, then the **existing** `check_observation_v1` (visibility widened to `pub(crate)`, body, order and categories unchanged). A `SourceEmpty`/`SourceOversize` request refusal maps to the legacy categories; everything else keeps the V1 parent category.
- The legacy arm builds one `GraphicImageInspectRequestV3` and reuses `validate_still_v3` / `validate_animation_v3` / `validate_vector_v3` through a new V4-only helper, so the V3 entrypoint body is not touched.
- `GraphicImagePayloadV4` is a closed union of the retained `GraphicImagePayloadV3` (still plane / GIF timeline / canonical vector text) and `AvifEvidenceV4` (`MediaWorkerCheckedEnvelopeV1` + orientation claim + transported plane digests) — no invented RGBA shape for SVG, GIF or high-depth AVIF. `GraphicImageInspectionSetV4::grants_publication_authority()` and `::proves_whole_publication()` are constant `false`; so is each record.
- `GraphicImageInspectRefusalKindV4::{Legacy(GraphicImageInspectRefusalKindV3), Media(MediaWorkerRefusalKindV1)}` keeps read-vs-inspect and legacy-vs-AVIF distinctions. The refusal retains its obligation for attribution but its `Debug` renders **only** the closed token, so a public diagnostic carries no path, reference, owner text, buffer or worker value.

**Outer adapter** (`solo-adapters::ConfinedGraphicInspectAdapterV4`). One real `read_graphic_asset_bytes_v1` on the owner capability, then `ConfinedGraphicInspectAdapterV3` for the five legacy formats and `ConfinedAvifInspectAdapterV1` for AVIF, both bound to the one already-trusted `solo-cli`. `inspect_entity_graphic_images_v4` returns `EntityImageOutcomeV4::{Complete, ReadRefused, Refused}`. No second binary, decoder, reader, `unsafe`, network or filesystem primitive; the parent still decodes nothing. V3's `inspect_entity_graphic_images_v3` still refuses AVIF with `avif-unsupported-v3` (N116's own case, green on both hosts in this run).

## 3. N124 evidence — four real cases, two native hosts

Both native logs carry `test result: ok. 4 passed; 0 failed; 0 ignored` for `n124_`, each case name exactly once and each `n124_evidence case=0N` exactly once:

```text
Ubuntu  case=01 formats=png,webp,jpeg,gif,svg,avif count=6 levels=5
        pids=[7792, 7796, 7800, 7804, 7808, 7812] distinct_pids=6 gif_frames=3
        svg_canonical_sha256=0403e0677a174ccafe18191a8a9f1c5a30993477bb074f49078b005f513caaba
        avif_planes=4 avif_geometry=256x512 avif_orientation=identity
        avif_display_sha_pinned=950be879…c84cc744 internal_inspected=true publication=false
        factory_complete=true
Ubuntu  case=02 refused_token=WORKER_UNAVAILABLE refused_arm=media whole_set_discarded=true
        partial_records=0 private_path_absent=true valid_count=6 valid_avif_planes=4 publication=false
Ubuntu  case=03 vectors=10 labels=["png-path-with-jpeg-bytes", "truncated-avif", "active-svg",
        "truncated-gif", "missing-authored-file", "favicon-role-refused-before-worker" ×3,
        "platform-namespace-refused-at-acquisition", "late-suppressed-internal-hostile"]
        whole_set_discarded=true partial_records=0 private_path_absent=true fallback=false
        publication=false
Ubuntu  case=04 obligations=7 pids=[7927, 7931, 7935, 7939, 7947, 7943, 7951] distinct_pids=7
        records=7 coded_avif=planes3,yuv,unverified alpha_avif=planes4,yuva,identity
        alpha_geometry=256x512 alpha_display_sha_pinned=950be879…c84cc744
        forged_ports_refused=true publication=false
Windows case=01 pids=[8296, 6620, 9548, 6236, 8024, 1264] … case=04 pids=[5320, 7140, 1592, 1952, 952, 2028, 8836]
        — identical tokens and flags
```

- **case 01** — a six-obligation inventory across **five EFS levels** (Entity carries two distinct registered roles, `favicon` PNG and `banner` WebP; two sources are `internal`) declares PNG, JPEG (`.jpg`), WebP, GIF (three real frames), the rich static SVG (**canonical digest `0403e067…` reproduced from the canonical bytes**) and the R10 associated-alpha AVIF. The composed adapter really launches **six distinct children** across both port families and the inward V4 result preserves the acquired order record-for-record; the outer `inspect_entity_graphic_images_v4` entrypoint returns the same six positions and digests. No publication authority.
- **case 02** — five valid non-AVIF sources plus a **representation-owned `internal` suppressed** AVIF carrying the defective `NOirot` association: the real confined AVIF worker refuses it although `0.0.18` renders it, and the V4 outcome is **one attributed media-armed refusal with zero partial records**; the public diagnostic contains no asset path, filename, staging tag or role text. Replacing that slot with the accepted associated-alpha still yields six records and one AVIF child. The refusal is late by construction — obligations are visited in order, so a media-armed refusal at position six means the five legacy inspections ran and were discarded. `refused_token=WORKER_UNAVAILABLE` is the accepted R10 mapping for "the confined child produced no trustworthy result".
- **case 03** — ten hostile vectors: a `.png` authored path carrying JPEG bytes (**no content-based substitution**), a truncated AVIF, an active SVG (`<script>`), a truncated GIF, a declared-but-absent source (read refusal, never an empty success), the `graphic.favicon` PNG-only grammar for `.avif`/`.gif`/`.svg` (**refused before any child exists**), a Platform-namespaced reference through the Entity route, and a **late suppressed `internal` hostile SVG** that still discards the whole set.
- **case 04** — seven obligations in one set: the five legacy formats, the R06 `white_1x1.avif` coded-plane AVIF and the R10 associated-alpha AVIF as two distinct authored obligations. Seven real child PIDs, seven ordered records; the coded record is `planes3, yuv, unverified` with its three plane digests equal to the independent golden, the alpha record is `planes4, yuva, identity` at 256×512 with the four pinned R10 displayed planes, and the five legacy records carry **no** orientation claim. Application-level fake-port assertions (`crates/solo-application/tests/media_worker.rs`, running locally and in CI) refuse the entire V4 set for a forged V3 source digest, V3 host fact, V3 format, V1 source digest, V1 host fact, V1 plane claim and V1 aggregate — each with its own typed token and a path-free diagnostic.

Each host also printed `n124_gate authentic=accepted` and `n124_gate mutation=rejected`: the same verifier rejected all **four** deliberately corrupted copies of the real log — a missing case, a false `3 passed; 1 failed`, changed `alpha_display_sha_pinned`/`svg_canonical_sha256` claims, and false `private_path_absent`/`whole_set_discarded` flags.


## 4. Retained contracts and earlier evidence

- **V3 compatibility:** `inspect_graphic_image_source_set_v3`, `GraphicImageInspectRefusalKindV3::AvifUnsupported` (`avif-unsupported-v3`), its discriminants and its adapter are unchanged; N116's own four cases and its `avif-unsupported-v3` assertion are green on both hosts in this same run, and the V2/V3 gate steps still report `4 passed`.
- **N121** remains two cases with `refused=10` and twelve counterfeits; **N122** three cases with the pinned display digest, 123952 non-opaque alpha and fifteen counterfeits; **N123** repaired with `alpha_differences=0`; **N120** four independent coded-plane goldens — all green on both hosts at `5d35c892`, and their three/four mutation probes still report `mutation=rejected`.
- **Independent oracle:** `avif-reference-oracle` unchanged and green (`pristine_manifest_matches=true`, `mutation_rejections=3/3`).
- **Architecture and security:** `architecture=legal packages=4 targets=34 files=66 edges=93` measured (not altered), architecture refusal self-test, forbidden-import/unsafe gates, cargo-deny with a fresh RustSec refresh, the real native source-reader fixtures, timeout/cleanup canaries, the release build/CLI smoke and the byte-for-byte reproducibility pair all pass on both hosts.
- **Honest note on the first attempt:** run `43729631588` at `ee1878b2` failed `Test (workspace, all targets, locked)` on both hosts because three N124 cases declared a descendant obligation for a key an ancestor already declared while inheriting it — the real inventory validator refused, exactly as designed (`media_worker_host_smoke.rs:4473`). The repair moved those cases onto the accepted **suppressing** multi-level seam (`suppressed_inventory_of`, as N121/N122 already use) and made the assertions locate records by format/class instead of by guessed index, plus removed a PID assertion that the accepted AVIF supervisor does not guarantee for a refused child. Nothing was skipped, ignored or weakened; the failed attempt and its cause are recorded here rather than hidden.

## 5. Local verification and honest limits

Recorded in the single Owner checkout before the pushes: `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` 0; `workspace_graph` `32 passed`; `check_architecture_fixtures` `35 passed`; `check-architecture` 0 at `packages=4 targets=34 files=66 edges=93`; `graphic_avif_inspection` `5 passed`; `media_worker` `16 passed` (14 retained + 2 new V4 fake-port tests); host-smoke `--no-run` builds; `cargo test --workspace --all-targets --locked` → **87 passed, 5 failed**, the five failures being the documented `source_read_native::snapshot_proofs` precondition panics at `source_read_native.rs:2017` because this WSL host has no verified disposable Btrfs `SOLO_TEST_SOURCE_FS_ROOT`. `n116_`/`n121_`/`n122_`/`n124_` therefore **did not run locally** (`NOT RUN LOCALLY`), and Windows linkage (`link.exe`) is unavailable locally: all real-host evidence above is native CI only. The N124 verifier logic was additionally exercised locally against a synthetic log in the printed shape — `authentic=accepted`, `mutation=rejected` — before the push, and the hosted logs then reproduced those verdicts natively. No `#[ignore]`, `--skip`, `|| true`, `continue-on-error`, substituted reader or fabricated token was introduced.

## 6. Repository state and forward boundary

`main` = A19 `5b35cb87` + this report; technical branch = `5d35c892` (clean); **28** remote heads, one worktree, one `origin`, exactly four `.documents/WIP/` files, empty stash, no branch/tag/PR/merge/release/deploy, no force push, no `reset --hard`, no `git clean`. Task scratch (`.temp/a19.sh`, `.temp/gate_n124.py`) removed; the earlier R10 probe targets stayed deleted, so the graph remains `targets=34 files=66`.

The **verified valid six-obligation layout** is: Entity `graphic.favicon`→`icon.png` (public) and Entity `graphic.banner`→`banner.webp` (internal); Facet `graphic.logo`→`logo.jpg`; Site `graphic.background`→`scene.gif`; Page `graphic.social-preview`→`mark.svg`; Representation `graphic.social-preview`→`alpha-irot.avif` (internal) — declared through the accepted suppressing seam because a descendant must narrow, not inherit, an ancestor's same key.

`status=COMPLETE` **only** for `R03C-SIX-FORMAT-REAL-CONFINED-WHOLE-CANDIDATE-V4-N124-TWO-HOST-VERIFIED`. Still open and explicitly **not** claimed: the Owner-reviewed `platform/assets/` library (`platformAssetLibraryVersion: "1.0.0"`), asset-ID/hash/licence/provenance catalogue, guarded Platform namespace acquisition, immutable derived-output privacy, publication authority, release/deploy and **Stage 03 / `R03.C` closure**. Platform-namespaced references remain unreadable through the Entity route, and V4 output is inert evidence only.

