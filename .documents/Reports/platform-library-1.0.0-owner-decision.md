# Platform asset library `1.0.0` — Owner decision record

**Owner decision:** `APPROVED`

**Recorded by:** the Implementation Agent under `SOLO-WO-118 R15`, from the Owner's own reply. This file records an instruction the Owner gave; it is **not** a signed licence instrument, a notarisation, a copyright assignment or a certification of upstream legal correctness.

## What was approved

The Owner approved **all 81 exact assets** listed in the R14 review packet for inclusion and redistribution in Solo's pinned `platformAssetLibraryVersion: "1.0.0"` generic Platform library, **subject to** the recorded Tabler MIT licence terms and the recorded per-asset provenance.

The approved set is identified by immutable Git blobs, not by a list copied into this file:

| Authority | Git blob | Canonical LF SHA-256 | Bytes |
|---|---|---|---|
| Selected catalogue `.documents/Reports/platform-library-1.0.0-foundation-candidate.json` | `a02db1605bac6cf1b45214a07b452d58895d7742` | `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` | 65,397 |
| Owner review sheet `.documents/Reports/platform-library-1.0.0-foundation-review.md` | `edb8468d7ef6ae290574672e1829bf5b583419ca` | `095942fb96521ddac088b799f4321e914c0db3e915de0bd416ae2a0c97ed49aa` | 75,313 |

Those two blobs are unchanged by this order: the review sheet keeps its historical blank checkboxes exactly as they were written before the decision, and the catalogue keeps its schema, order and rights text without an added approval field. **This record is the approval linkage**, not an edit of either artefact.

- **Membership:** exactly the 81 rows of the review sheet, in its order, which is also the catalogue's order; 81 distinct source SHA-256s; no 82nd asset.
- **Immutable donor source:** `provelopment/provelopment-foundation` commit `fa599973a88ff77f6a8e012f3266c3c139b52c17`, tree `89b57a92869d2e293d33527589f3f4680938f9b2`; every approved product byte is the corresponding donor original, unmodified (lowercase filenames, LF line endings, `currentColor` left in the source).
- **Licence subject:** Tabler Icons v3.46.0, MIT, `Copyright (c) 2020-2026 Paweł Kuna`. The full notice travels in the product tree at `platform/assets/licensing/TABLER-ICONS-MIT.txt` (blob `3e82379dab3fe93d9ee22251949604ed63ddea39`, SHA-256 `b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db`) and must accompany any distribution of these bytes. The Foundation root Apache-2.0 licence does **not** replace it and grants no rights in names, logos or trademarks.
- **Provenance subject:** `platform/assets/licensing/icon-provenance.json` (blob `a74d234417d5bb63ff05e2b64f93d757864f3d46`, SHA-256 `e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b`), the machine record of source library, version, source icon name, canonical filename and licence for each approved asset.

## Decision context

On 2026-11-16 (Asia/Jakarta) the Chief Master Architect asked the Owner whether the Owner approved **all 81 exact assets in the R14 review sheet** for inclusion and redistribution in Solo's pinned `platformAssetLibraryVersion: "1.0.0"` library, subject to the recorded Tabler MIT licence and provenance. The Owner answered **"approved, proceed"**. That reply is set-wide per-asset approval **by exact pinned membership**; it is not permission to expand the set, invent evidence, waive a media gate or self-issue a licence.

## Explicitly outside this approval

- `icon-services.svg` — Provelopment original artwork, not Tabler/MIT, no licence inferred.
- `icon-status-info.svg` and `icon-volunteer.svg` — byte-identical duplicates of the approved `icon-about.svg` and `icon-clients.svg`; the single representative of each alias is the approved file. No alias table, extra entry or duplicate digest is authorized.
- Social platform marks, brand or identity logos, synthetic or blank placeholders, screenshots and every other source file.

## Authority that remains disabled

`publication_authority=false` · `derived_output_publication=false` · `platform_read_enabled` covers **source-byte reads only** · `release_authority=false` · `deployment_authority=false` · `stage03_open=true`.

Approving these source bytes does not publish any Page, authorize any EFS property or route, produce an immutable public derivative, complete the mixed Entity/Platform whole-candidate gate, permit a release or deployment, or close `R03.C` / Stage 03.
