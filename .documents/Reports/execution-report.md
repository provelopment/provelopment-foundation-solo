# Execution Report

**Work order:** `SOLO-WO-118 R13` — pinned Foundation donor artwork, 81 review-only fixtures and an Owner-review packet.
**Required outcome:** `R03C-FOUNDATION-DONOR-81-TABLER-ICONS-IMMUTABLE-OWNER-REVIEW-PACKET-N126-TWO-HOST-V1`.

**status=STOPPED** — the packet is complete and authentic on Linux, but the **native Windows** host cannot satisfy the byte-identity requirement: the tracked `.gitattributes` rule `* text=auto` projects CRLF into the working tree for the `.svg`/`.txt` fixtures. That file is **outside the R13 write ledger**, and the ledger forbids adding an attribute file inside the two enumerated fixture directories ("exactly 81 donor SVG blobs", "exact pinned MIT text + exact pinned icon-provenance.json"). Per §2 the order stops rather than broadening the ledger or normalising donor bytes.

**owner_per_asset_approval_pending=true** · **rights_review_pending=true** · **platform_library_delivered=false** · **platform_read_enabled=false** · **publication_authority=false** · **stage03_open=true**

## 1. Transaction

| Step | SHA | Validation |
|---|---|---|
| baseline | `main` = R20 `b1c557244d276d3d04db2010fac681c92d61170c`; branch = T20 `d219f6b8943664f5e32eb4288edefb7ba3d3268c` | one `origin` = `provelopment/provelopment-foundation-solo`, one worktree, 28 remote heads, four WIP files, empty stash, clean trees |
| **A21** (`main`) | `041a1449705f09928a9d8f7041d72207a033d9b9` — tree `4eac8409e974d2d68f7b4f964931994b1a367dbb`, parent `b1c5572` | ACTIVE blob `3857ff2cf36097d00456a5852165286afbd1cea1` identical in worktree, index, commit and `origin/main`; only path `.documents/WIP/workorder-active.md` |
| **T21.0** (branch) | `1b97b60925ae27bb0e67e951313e69fbf0c66e23`, parent `d219f6b` | single-path `git restore --source=A21`; identical blob; staged set = ACTIVE only |
| **T21.1** (branch) | `72425de9d5c91ec03f10089ce0db4810c36ee045` | the R13 packet: `87 files changed, 4540 insertions(+)`, no deletions, no edits to prior N093–N125 bodies |
| **R21** (`main`) | this report's commit (SHA in chat) | parent `041a1449`, exactly one path changed |

Technical run **`43804143667` attempt 1** at `72425de9`: `verify (ubuntu-24.04)` `133090902932` ✅, `avif-reference-oracle` `133090902700` ✅, `verify (windows-2025)` `133090902898` ❌. No rerun, no dispatch, no `continue-on-error`, no `--skip`.

## 2. Donor provenance — exact and immutable

- Donor repository `provelopment/provelopment-foundation`; **immutable commit** `fa599973a88ff77f6a8e012f3266c3c139b52c17`, recursive tree `89b57a92869d2e293d33527589f3f4680938f9b2`, commit date `2026-10-29T06:43:45Z`. Mutable `main` was never used as import authority.
- 84 SVGs plus two metadata files were fetched **from that commit only**; every file matched the donor's own Git blob SHA before acceptance (88/88, 82 302 bytes).
- `TABLER-ICONS-MIT.txt` blob `3e82379dab3fe93d9ee22251949604ed63ddea39`, 1 073 bytes, SHA-256 `b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db`, with the MIT grant and `Copyright (c) 2020-2026 Paweł Kuna`.
- `icon-provenance.json` blob `a74d234417d5bb63ff05e2b64f93d757864f3d46`, 25 274 bytes, SHA-256 `e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b`.
- Programmatic reconciliation, both directions, **zero failures**: 84 records ↔ 84 files, **83 MIT + 1 original**, every MIT record `Tabler Icons 3.46.0`, every record naming its own `source_icon_name`, exactly the two declared byte-identical pairs (`icon-about` = `icon-status-info`, blob `c407077e4f849eaf536908a9ee444c4aa4752af4`; `icon-clients` = `icon-volunteer`, blob `b0d6ecc811b7e1fd4c6d98fc3a1a59e7224c54bc`) and no unexpected same-blob pair.
- The donor root Apache-2.0 licence is recorded as **source-code-only** and is never applied to the icons.

## 3. 84 → 81 exclusion proof

| Withheld | Reason | Representative kept |
|---|---|---|
| `icon-status-info.svg` | byte-identical duplicate of `icon-about.svg`; the R12 catalogue refuses duplicate SHA-256 aliases | `icon-about.svg` |
| `icon-volunteer.svg` | byte-identical duplicate, declared `intentional_source_reuse` of `heart-handshake` | `icon-clients.svg` |
| `icon-services.svg` | donor provenance says **Provelopment original artwork**, not MIT; the root Apache-2.0 grant does not cover artwork | none inferred — needs an explicit ownership/licensing decision |

`platform-marks/` social logos, Foundation brand trees, placeholders, demonstration/synthetic assets and any screenshot- or UI-code-derived artwork were excluded **by category**, not treated as missing.

## 4. Imported review-only fixtures — byte identity proved

- 81 SVGs under `crates/solo-cli/tests/fixtures/foundation-donor-icons/icons/` (identical filenames) plus the two pinned metadata files under `.../licensing/`; **83 files**, exactly the ledger's set.
- For every one of the 83: working-tree `git hash-object` **and** staged index blob equal the donor Git blob; zero byte mutation; 81 unique source SHA-256 values. `text=auto` is a no-op **at commit level** because the donor bytes are already LF.
- No colour replacement, recompression, comment removal, reflow, conversion or newline edit was applied anywhere.

## 5. Candidate catalogue and Owner review sheet

- `.documents/Reports/platform-library-1.0.0-foundation-candidate.json` — 81 entries, deterministic ascending `icons/<canonical filename>` order, unique digests, `image/svg+xml`, 24×24, `MIT` / `Paweł Kuna`, licence evidence naming the pinned commit and MIT blob, provenance naming `Tabler Icons v3.46.0` plus each authoritative `source_icon_name` and exact donor Git blob. 65 397 bytes, SHA-256 `33a8872be4ba131db58ca37d27b7bfb57166315248399e66701e59777c76119a`. It parses under the unchanged R12 `parse_platform_asset_catalogue_v1("1.0.0")` and carries **no** `approved`/`rightsReviewed`/`published`/`installed` key.
- `.documents/Reports/platform-library-1.0.0-foundation-review.md` — **81 individually reviewable rows**, each with approval blank, catalogue ID, donor path, commit, Git blob, source SHA-256, byte count, 24×24 geometry, MIME, MIT/version/holder, authoritative source icon, licensing and metadata paths, the **real confined sanitizer classification and canonical output SHA-256**, and a direct link to the pinned donor blob; plus the withheld table, category exclusions, the NOTICE reproduction plan and the authority boundary. Marked **PENDING OWNER PER-ASSET APPROVAL — NOT DISTRIBUTABLE**. 75 313 bytes, SHA-256 `4d7b432e594498582ebc8e8c4995e7cd8dbf00547a022c0eb3cc347ebe77dabd`.
- `rights_review_pending=true`: a well-formed MIT row is evidence for an Owner decision, not clearance.


## 6. N126 evidence — real confined workers, and the honest Windows failure

**Local (WSL, real `solo-cli` children; these tests need no Btrfs root):** `cargo test -p solo-cli --test media_worker_host_smoke --locked n126_ -- --nocapture --test-threads=1` → `3 passed; 0 failed; 0 ignored` in ~2.7 s, **81 genuine confined child invocations**, aggregate `3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`.

**Ubuntu `133090902932` — accepted**, one authentic evidence line per case:

```text
case=01 originals=81 inspected=81 real_children=81 geometry=24x24 donor_pinned=true
        source_scope=review-fixture publication=false aggregate=3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca
case=02 entries=81 mit_covered=81 originals_withheld=1 duplicate_aliases_withheld=2
        review_only=true platform_read=false publication=false
case=03 forgeries_refused=true alias_digest_refused=true licence_missing_refused=true
        donor_drift_refused=true platform_unavailable=true zero_authority=true
n126_gate authentic=accepted
n126_gate mutation=rejected
```

The aggregate matched the locally measured value exactly, and the gate rejected all four corruptions (missing case, 80-asset count, forged `publication`/`platform_read`, wrong `mit_sha`).

**Windows `133090902898` — rejected.** All three N126 tests fail while every other test in that step passes:

```text
n126_01 → icons/icon-about.svg: the original source bytes match the catalogue entry
  left:  bbe534455dbcebfa1b99b9fe248c22c91c8adad01d3e269de064798b139415b0  (455 bytes)
  right: bf4377dff0aaaddf1676a4bb7292f90f1102f19daaca7021331ba6a191317c1c  (439 bytes)
n126_02 → the pinned Tabler MIT notice is unchanged
  left:  dbfa6cff2e8426878267c6719ff68dcd8bc3dadaffb2207f5c46ce939c4becd3  (1094 bytes)
  right: b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db  (1073 bytes)
```

**Cause proved, not guessed:** both left-hand digests are exactly the SHA-256 of the **CRLF projection** of those files (439→455 and 1073→1094 bytes). The committed blobs are byte-exact — `git cat-file blob` of `icons/icon-about.svg` hashes to `bf4377df…` and its blob is the donor's `c407077e…`; the MIT notice blob is `3e82379d…` hashing to `b740a1d4…`. The divergence appears only when Git materialises the working tree on Windows under the tracked rule `.gitattributes:2: * text=auto`.

**Why this is a STOP and not a workaround:** `.gitattributes` is outside the R13 ledger; adding an attribute file inside `.../icons/` or `.../licensing/` would contradict "exactly 81 donor SVG blobs" and "exact pinned MIT text + exact pinned icon-provenance.json"; normalising CRLF at read time is precisely the byte modification and hidden transformation the order forbids; and editing the sanitizer or the assertions would weaken the proof.

## 7. Retained gates, local verification and repository state

- Every prior gate is intact and green at `72425de9` on Ubuntu: N093/N102/N108/N113/N116/N120–N125 with their own mutation rejections, V2/V3/V4 format gates, architecture `packages=4 targets=36 files=70 edges=93` (unchanged — the packet adds fixtures and test code only, no target or edge), strict Clippy, safe-code, `cargo-deny` with a fresh RustSec refresh, release reproducibility, and the independent `avif-reference-oracle` (`133090902700`) at the same SHA.
- Local: `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy … -D warnings` 0; `workspace_graph` 32 passed; `check_architecture_fixtures` 35 passed; `check-architecture` legal; `platform_asset_catalogue` 7 + 3 passed; `media_worker` 16 passed; `graphic_avif_inspection` 5 passed; `cargo test --workspace --all-targets --locked` → **87 passed, 5 failed**, the five being the documented `source_read_native::snapshot_proofs` Btrfs preconditions (**NOT RUN LOCALLY**, not reclassified as green). Native Windows linkage is unavailable locally, so Windows host evidence could only come from CI.
- Repository: one worktree, one `origin`, **28** remote heads, exactly four `.documents/WIP/` files, empty stash, clean `main` and technical trees, no branch/tag/PR/merge/release/deploy, no force push, no `reset --hard`, no `git clean`. Task scratch under `.temp/` (donor download, generator scripts, captured rows) was deleted at run end.

## 8. STOP disposition and the least permissible next change

`status=STOPPED` for `R03C-FOUNDATION-DONOR-81-TABLER-ICONS-IMMUTABLE-OWNER-REVIEW-PACKET-N126-TWO-HOST-V1`, because the two-host requirement cannot be met while Windows materialises the fixtures as CRLF.

**Least permissible next change (one authorized line, one existing file):** append to the tracked `.gitattributes`

```text
crates/solo-cli/tests/fixtures/foundation-donor-icons/** -text
```

This changes **no** fixture byte, no test, no assertion and no sanitizer behaviour; it only stops Git from projecting CRLF into that directory, after which `72425de9`'s existing code should pass on Windows unchanged and the packet can be re-proved at one final SHA. An equivalent alternative is an Owner-authorized `.gitattributes` inside each of the two fixture directories (two new files, changing the "exactly 81 blobs" / "exact two metadata files" wording). Anything else — normalising bytes, relaxing an assertion, re-materialising tracked files in CI — is a weaker proof and is not proposed.

No artwork is approved, nothing is installed, no Platform read is enabled, nothing is published, deployed or released, and **`R03.C` / Stage 03 remain OPEN**.

