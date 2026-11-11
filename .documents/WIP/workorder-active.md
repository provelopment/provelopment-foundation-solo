# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-066 R01**  
**Title:** Administratively close independently accepted R03.C2, select unissued R03.C3 Entity image-media verification, and conditionally prune fully integrated SOLO-WO-065 branches  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner→Implementation Agent handoff ID:** `SOLO-WO-066 R01`  
**Owner authorization:** On 2026-11-11 the Owner expressly authorized administrative closure of accepted R03.C2 and selection of the next bounded implementation candidate. **This is a documentation/Git-lifecycle-only order.** The Owner authorizes the exact staging/commits/pushes, conditional safe branch deletions, existing two-host CI gates and canonical report below. It does **not** issue R03.C3 for implementation, authorize any image decoder or dependency, alter production or test code, create media/catalogue/artwork, publish anything, change Architecture, or close R03.C or Stage 03. Chief Architect GitHub access is **READ ONLY**; only the separately handed-off Implementation Agent may perform authorized repository operations in the one verified Owner checkout.

## 1. Governing authority, acceptance and exact starting anchors

**Independent acceptance.** Chief Architect disposition: **`ACCEPT — SOLO-WO-065 R03 / R03.C2`**, following R01 implementation, R02 exact-byte/security-test correction and R03 direct Windows surviving writable mapped-view proof. The accepted final report HEAD on `main` is **`b7c999ac1d4d7341ea1aaee177b7be130591d8d1`**, tree **`dc4978e2a87b814d6c50aab3fae6b6380030c367`**, canonical report blob **`4cd3298ffc85dec69079ec948674efc92b7fd9fb`**. Accepted substantive R03 proof: **`bbe4a98f4da464ce189956fea34dd94323499ab8`** (tree `36a034bf003dc0c2dc389801da6449dad3aa67a8`), parent of the report commit. The accepted R02 substantive correction is `2de75c1e3f96a84b32f8cd21f491d925225d62d1`, and R01 final technical tip is `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb`. All three are reachable through the accepted main history.

**Independent CI evidence already audited:** Ubuntu 24.04 and Windows 2025 on R03 issuance run `43044051684`, correction branch `43045427407`, substantive main `43046529973`, and report HEAD **`43047979490`**: all completed successfully; final Ubuntu **407** / Windows **412** passing workspace tests, zero failed or ignored, existing host-inapplicable skips. On Windows the direct `r03c2_10c_transferred_writable_view_refuses_asset_port_until_unmapped` test passed at branch, integrated main and final report HEAD. The inherited `Unicode-3.0` `license-not-encountered` warning is nonfatal and **UNVERIFIED**, not waived or suppressed. R03.C2 acceptance is **source-byte acquisition only**, not image-validity, licence, Platform library or publication acceptance.

**Live GitHub preflight recorded at order preparation** (reverify just before agent action):

| Anchor | Exact value |
| --- | --- |
| `main` / `origin/main` expected HEAD | `b7c999ac1d4d7341ea1aaee177b7be130591d8d1` |
| Expected tree | `dc4978e2a87b814d6c50aab3fae6b6380030c367` |
| Canonical report | `.documents/Reports/execution-report.md` blob `4cd3298ffc85dec69079ec948674efc92b7fd9fb` |
| Frozen previous ACTIVE | `.documents/WIP/workorder-active.md` blob `2622aeace2329a91e15cbdced3781a30bb05d2d0` (`SOLO-WO-065 R03`; Owner will replace it with this entire R01 order) |
| ROADMAP | `.documents/WIP/plan-roadmap.md` blob `97df81e5cf63443d11a8f4414934d4fd05edd0d8` |
| NEXT | `.documents/WIP/plan-next.md` blob `92fa562b1af0c54391851aa5c79deb1631127069` (one neutral title) |
| OUTLOOK | `.documents/WIP/plan-outlook.md` blob `cb9c7a610cf3c537557e060ca9656e44a053e69d` (unchanged) |
| Accepted Entity reader | `crates/solo-adapters/src/source_read.rs` blob `fb6f005a053ea80cea452fe2d57fb9fc8ea45ad9` |
| Accepted image acquisition policy | `crates/solo-application/src/graphic_asset_read.rs` blob `6499421d6e718e9d46d64e45fb152f34e2989cc1` |
| Accepted inventory | `crates/solo-application/src/graphic_asset_inventory.rs` blob `7de9bc7399854b2059cbec17ee23c1e1ae637159` |
| Accepted native proof | `crates/solo-adapters/src/source_read_native.rs` blob `5527b2b4f0d4f47acaa86eead74c7106607c789f` |
| Cargo lock | `Cargo.lock` blob `2f9e53576df117f2020ae064d6973304f20833f1` |
| Adapter manifest | `crates/solo-adapters/Cargo.toml` blob `6109224cd3e3e89487884631f2b07c68ad8149c3` (`rustix = "=1.1.6"`) |
| Component policy / workflow / toolchain | `deny.toml` `3063e864c28be1e77787f43f1b6f561d543c8f4a`; `.github/workflows/rust-verify.yml` `eb78dad240bb7adb3e0ef9535870897176c99843`; `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c` |
| Architecture / SOP | `.documents/Architecture/ARCHITECTURE.md` `250bf62efd07037fe70022b5fc88a63610692245`; `.documents/SOP/sop-architect.md` `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1`; `.documents/SOP/sop-implementation.md` `51bb37922cf7fc56952e4ad6155c9a59d3c09036` |

**Git branch anchors (both local and remote must be freshly verified):**

| Branch | Exact expected tip | Handling |
| --- | --- | --- |
| `main` | `b7c999ac1d4d7341ea1aaee177b7be130591d8d1` before Owner-installed issuance | Remains the only integration line |
| `solo-wo-065-r01` | `7f609320afc0ee6a6bbc6637ef3f3fc68f19ecbb` | Conditional safe prune after issuance |
| `solo-wo-065-r02` | `2de75c1e3f96a84b32f8cd21f491d925225d62d1` | Conditional safe prune after issuance |
| `solo-wo-065-r03` | `bbe4a98f4da464ce189956fea34dd94323499ab8` | Conditional safe prune after issuance |
| `solo-wo-061-r01` | `4371af21dce67f33cf1df0384738bf353ca80fc6` | **PRESERVE** unchanged, divergent historical recovery branch |

GitHub compare at preparation showed each `solo-wo-065-r0*` tip as an ancestor of the accepted `main` (`behind=0` when comparing branch tip → main), whereas the original `solo-wo-061-r01` **diverges**. GitHub branch listings showed no protection on the four named task branches; the sole listed PR was an unrelated closed docs PR. These remote observations do not replace fresh agent verification of local worktrees, open PRs, ref integrity or protection.

**Required authorities to read before writes:** `AGENTS.md`; `.documents/SOP/sop-architect.md` §§2, 5–8, 6A, 6B and 7A/7B (especially WIP lifecycle and single canonical report); `.documents/SOP/sop-implementation.md` §§4–5C, 9; `.documents/Architecture/ARCHITECTURE.md` §§6.2C, 6.2G, 6.2I, 6.2K; the four WIP files and current canonical report; accepted R03.C1 inventory and R03.C2 Entity asset-read evidence APIs. The governing SOP controls Git and workspace safety; this order adds no second procedure or general permission.

## 2. Exact permanent-path and operation ledger

Across the **whole** `SOLO-WO-066 R01` run, the only permitted permanent tracked-file changes are:

1. `.documents/WIP/workorder-active.md` — **issuance**: Owner-installed complete R01 order; **closure**: reset to the exact neutral single-title file in §3D.
2. `.documents/WIP/plan-roadmap.md` — **closure only**: one exact existing R03.C bullet replacement in §3D; every other byte stays unchanged.
3. `.documents/WIP/plan-next.md` — **closure only**: replace neutral title with exactly the one unissued R03.C3 candidate in §4.
4. `.documents/Reports/execution-report.md` — **last only**: overwrite the **single** canonical report with factual closure evidence; no report archive.

`.documents/WIP/plan-outlook.md` must remain byte-identical. WIP retains **exactly four files** and no extra folder or archive. The full order comprises an **ACTIVE-only issuance commit**, one **ROADMAP + NEXT + ACTIVE closure commit**, and one **report-only final commit**, each pushed normally on existing `main` and each gated by successful full Ubuntu/Windows `rust-verify` CI at its **exact** revision. Neither a new branch nor PR is authorized for this administrative work.

The sole non-file Git ref operation authorized is **conditional, independently checked, normal/non-force deletion** of the three fully integrated `solo-wo-065-r01`, `solo-wo-065-r02`, `solo-wo-065-r03` refs as detailed in §3C. This authority is **not** permission to delete or move `solo-wo-061-r01`, force-push, reset, rebase, amend published history, create a second checkout/worktree, or modify other branches. If any safety condition is unproven, retain the affected ref and report the blocker rather than force deletion.

**Expressly prohibited tracked changes:** all Rust sources/tests and crates, native boundary and Win32 proof, Cargo manifests/lock, `deny.toml`, rust toolchain, workflow, `.gitignore`, AGENTS, SOP, About, Architecture, other documentation, assets, `platform/`, media fixtures, catalogues and the OUTLOOK slot. No dependency selection, decoder install, security exception, renderer, public route, publisher, release, deployment, external provider, or other-repository write. No R03.C3 implementation or issuance. No change to R03.C/Stage 03 exit gates or to Stages 04–13.

## 3. Ordered execution protocol — no implicit authorization

### 3A. Read-only preflight, one verified root

Confirm a **single** Owner-local Solo checkout/worktree, correct real Git root and sole `origin`, current local `HEAD == main == origin/main` at §1's exact baseline, no unknown staged/unstaged/untracked/Owner-owned work, no sequencer, no second worktree or nested checkout. The **only** expected local tracked difference is the Owner's complete installed `SOLO-WO-066 R01` ACTIVE file; verify it is the same document with exact Owner handoff ID. Verify the exact four WIP files and their baseline blobs; exact report/blob/tree; frozen Cargo/security/workflow/Architecture; all four task branch tips local and remote. Inspect any open/closed PRs and repo policy. If the baseline changed or unrelated material is present, STOP without using `reset`, `clean`, forced checkout or a guessed rebase.

Read the current ROADMAP and verify its old R03.C bullet matches §3D **byte-for-byte** before planning any replacement. Confirm the accepted Entity reader is nonpublishing and does not already perform verified media decoding; the next candidate must not misrepresent acquired bytes as validated images. Confirm that R03.C and Stage 03 still contain outstanding media, Platform catalogue, whole-candidate and derived/public work.

### 3B. Publish issuance — ACTIVE only

Stage and commit **only** the Owner-installed `.documents/WIP/workorder-active.md` on `main` (one commit, no branch switch). Push normally to `origin/main`; verify the actual remote commit, parent/tree and committed ACTIVE blob, and that the diff contains precisely that path. Run and inspect the existing **full** `rust-verify` workflow on `ubuntu-24.04` and `windows-2025` at the exact issuance SHA: both completed green; test/security/build/architecture/negative refusal and host-applicable CI steps must pass. Report warnings and any host-specific skips accurately. Stop on a mandatory failure; do not neutralize ACTIVE yet or use a different SHA's green run as evidence.

### 3C. Conditional ref cleanup, while this issued ACTIVE still exists

After green issuance CI and **before** clearing ACTIVE, independently assess each of `solo-wo-065-r01`, `solo-wo-065-r02`, `solo-wo-065-r03` against the exact §1 tip, with these conditions for **each** ref:

- Local `refs/heads/<name>`, remote-tracking ref and authoritative GitHub `refs/heads/<name>` exist and all have precisely the stated SHA; no unseen commit, divergent head, unreviewed PR dependency, branch protection/ruleset blocker, other worktree attachment or active external dependency exists. Do not infer safety from a GitHub branch listing alone.
- Current `main` includes that exact tip by ancestry (`git merge-base --is-ancestor <tip> main` succeeds, `git rev-list --left-right --count main...<tip>` shows zero branch-only commits). All branch work is retained in `main` history, including the accepted corrections; `main` is clean and current; no active branch is being deleted from under a worktree.
- Only after proving all conditions, delete the **local** branch with normal `git branch -d <name>` (never `-D`) and delete the **remote** ref with normal `git push origin --delete <name>` (never force). Read back remote branch inventory and compare the post-action main SHA. Take the branches individually; do not use bulk destructive commands.

**Preserve `solo-wo-061-r01` exactly** at `4371af21dce67f33cf1df0384738bf353ca80fc6`, both locally and on GitHub. Its divergent work is not contained in main; no deletion, move or reconciliation is authorized. If any of the three `solo-wo-065` refs is unsafe or cannot be proven safe, **leave it untouched**, record which and why, and safely defer that individual prune; do not force success or block a sound documentation closure solely because an optional prune is protected. If unexpected Owner work, a destructive side effect, branch mismatches or other real repository integrity risk appears, STOP and report. Preserve all commit history.

### 3D. Exact administrative closure and one NEXT candidate

Only after §3C is complete or each unsafe deletion is safely deferred, edit **exactly** the three WIP paths in one closure commit.

**ROADMAP: exact OLD single line (must match currently committed line verbatim, with no `>` prefix):**

```text
- **R03.C:** complete the remaining governed graphic-asset delivery beyond the accepted authored-obligation inventory: confined Entity `content/assets/` and explicitly authored, exactly pinned Platform `platform/assets/` source acquisition; authoritative bounded media-byte, format/magic/MIME, image dimension and pixel-limit verification; an Owner-reviewed lawful `platformAssetLibraryVersion: "1.0.0"` catalogue with complete asset IDs, hashes, media class, dimensions and licence/provenance; whole-candidate validation across all declared owners including internal, suppressed and unlisted declarations; and privacy-safe public use and immutable derived outputs. Preserve the accepted five-level graphic inheritance, provenance, exposure, cardinality, suppression and **no automatic platform or role fallback** under binding Architecture §§6.2G/6.2I.
```

**ROADMAP: exact NEW single line (replacement; no other ROADMAP edit):**

```text
- **R03.C:** complete the remaining governed graphic-asset delivery: authoritative bounded media-byte, format/magic/observed MIME, safe decoding, image dimension and pixel-limit verification of confined source bytes; explicitly authored, exactly pinned Platform `platform/assets/` source acquisition and an Owner-reviewed lawful `platformAssetLibraryVersion: "1.0.0"` catalogue with complete asset IDs, hashes, media class, dimensions and licence/provenance; whole-candidate validation across all declared owners including internal, suppressed and unlisted declarations; and privacy-safe public use with immutable derived outputs. Preserve the accepted five-level graphic inheritance, provenance, exposure, cardinality, suppression and **no automatic platform or role fallback** under binding Architecture §§6.2G/6.2I.
```

Replace **only** the literal matched old R03.C line. This removes already accepted Entity `content/assets/` *byte acquisition* from unfinished ROADMAP obligations while preserving every still-outstanding media-verification, Platform catalogue/source, complete validation, privacy/public and derived-output requirement. It does not modify or reinterpret the binding Architecture, create a no-fallback exception, or close the R03.C package or Stage 03. Preserve the Stage 03 exit gate, every heading and all Stages 04–13 with their existing gates, including any unrelated wording not covered by this order. Verify `git diff --numstat` shows one deletion and one insertion in ROADMAP and `git diff --check` is clean. An absent/mismatched old line is a STOP, not a fuzzy replace or broader rewrite.

**NEXT:** replace the entire neutral `.documents/WIP/plan-next.md` with **exactly the §4 Markdown candidate** (one `#` title, one `## R03.C3` candidate, five labeled fields, one final newline). It is **unissued**; nothing in NEXT grants implementation permission. No secondary candidate, previous candidate, retrospective or generic policy is permitted.

**ACTIVE:** reset its entire content, including the issued order, to the single neutral title plus exactly one trailing newline:

```text
# Foundation Solo — Active Work Order
```

**OUTLOOK:** byte-identical and untouched. Verify four WIP files and no others. Stage and commit **only** ROADMAP, NEXT and ACTIVE together. Push normally to `origin/main` (one single-parent closure commit), verify exact remote SHA/tree/blobs and one-line ROADMAP difference. Require completed full two-host green `rust-verify` CI at the closure commit before report publication. The accepted R03.C2 code and prior canonical reports stay preserved in Git history.

### 3E. One canonical report, final CI and handoff

Overwrite **only** `.documents/Reports/execution-report.md` with a factual `SOLO-WO-066 R01` administrative execution report, replacing (not archiving) the previous report. Include ISO date; full baseline and issuance/closure known commit/tree/blobs; exact path ledger; literal one-line ROADMAP delta; byte comparison of §4 and committed NEXT; neutral ACTIVE; OUTLOOK unchanged; four-file WIP purity; retained Stage 03 and R03.C; every attempted conditional branch prune with exact local/remote SHA, ancestry, protection, worktree and PR evidence; divergent historical branch preservation; three CI runs with Ubuntu/Windows job IDs, steps, passed/ignored/failed totals, host skips, nonfatal and unverified warnings; product/security/Architecture/lock/workflow invariance; cleanup and local limitations. Report `COMPLETE` only for the demonstrated administrative scope. Do **not** assert R03.C3 is issued, implemented, media-valid, or public; the Agent does **not** self-ACCEPT this administrative run.

Make **one report-only** commit and push normally to `origin/main`; run and inspect the full existing two-host CI at that **exact final report HEAD**. Read the GitHub remote report content back and verify its blob, final commit parent and tree, all three changed-path sets, clean local `main == origin/main`, root/worktree count, final WIP inventory, branch inventory and all forbidden-path invariants. Do **not** put the report's own commit SHA in its body (self-reference); provide report SHA/tree and URLs in the handoff message after publication. The Architect will independently review the final canonical report and return `ACCEPT / REVISE / STOP` for this administrative order.

**Temporary-workspace discipline:** protect the ignored live Rust `target/` cache. The inherited Solo `.temp/` archives were previously reported as **2 files / 289,253,806 bytes**; preserve them unless an already documented cleanup trigger is positively satisfied and specifically authorized, and report their actual current inventory (do not guess). Clean only task-owned disposable Solo `.temp/` or supplied Cline task logs and fixtures after use, with truthful counts. Do not `git clean`, reset, force-add ignored content, delete unknown material or work outside the permitted single checkout and supplied temporary areas. A clean Git status is not a substitute for owned-scratch cleanup.

## 4. Exact NEXT file — one unissued, bounded R03.C3 candidate

Place **exactly** the following Markdown, starting at its `#` title and ending with one newline, in `.documents/WIP/plan-next.md` during §3D. This section is content to be installed, not permission to implement it:

```markdown
# Foundation Solo — plan-next (Next Candidate)

## R03.C3 — Bounded Entity image-media verification

**Objective:** Establish a nonpublishing, fail-closed validation gate for the **actual acquired Entity image bytes** delivered by accepted R03.C2. For every authored Entity graphic obligation, verify that the complete source is a supported, structurally and semantically valid image within the binding media limits; return typed, provenance-linked verification evidence or reject the whole attempt. A filename extension, magic-byte prefix or noncryptographic source fingerprint alone is never proof of a valid image, and no validated result grants publication authority.

**Prerequisites/readiness:** Accepted R03.C1 complete five-level authored graphic-obligation inventory and accepted R03.C2 whole-inventory, confined `content/assets/` byte acquisition and typed `GraphicAssetSourceSetV1` evidence are available on both CI hosts. Binding Architecture §§6.2C, 6.2G and 6.2I supplies the media contract; the current acquisition layer deliberately has `media_verified = false` and no decoder. Prior to a distinct implementation order, inspect the existing inward/application ports, accepted byte ownership and asset-role constraints, actual media inspection/decoder support for **all four** image families, worst-case memory/pixel ceilings, licence/advisory posture and the specific adapter/dependency ledger. No Platform catalogue, artwork or authenticated Platform read capability exists yet; do not infer either from the accepted Entity source bytes.

**Proposed scope:** Consume the accepted, complete **Entity-only** source evidence without reopening paths, scanning the filesystem, fetching external URLs or changing any source identities. Validate `.png` as PNG, `.jpg`/`.jpeg` as JPEG, `.webp` as WebP and `.avif` as AVIF using authoritative bounded inspection of complete bytes, coherent format signatures/observed media type, safe structure and actual decode/verification rather than header-only acceptance. Enforce the already accepted source bound of **20 MiB**, image width and height each **1–8192**, and **at most 32,000,000 decoded pixels**, plus fail-closed resource/decompression bounds. Reject truncated, corrupted, mismatched-format, active or ambiguous image payloads and unsupported encodings; `graphic.favicon` remains PNG-only. The decision is over **every** declared graphic obligation, including internal, unlisted and descendant-suppressed owners, independent of a selected public target or duplicate literal filename. On any invalid image, refuse the complete evidence set with typed path/byte-safe diagnostics; retain original owner, role, local index, exposure, namespace, reference and source identity in successful inert evidence. Empty Entity-only inventories may return an inert empty validation result. Any `namespace: "platform"` obligation remains `PlatformUnverified`, including mixed inventories, until its distinct, exactly pinned and Owner-reviewed catalogue/source gate exists; never skip it and claim full success.

**Proposed deliverables:** A separately scoped technical order with exact files and vetted inspection/decoder/dependency decisions; pure inward validation policy with an explicit replaceable media-inspection port and a bounded outward implementation; typed, owned, **nonpublishing** per-obligation media evidence/refusal results; genuine in-memory fixtures and adversarial bytes for PNG/JPEG/WebP/AVIF, favicon/wrong-class, false magic/MIME, truncated input, zero-byte input, oversized metadata, malicious/decompression-bomb cases, resource limits, repeated authored references, private/suppressed/unlisted failures, and mixed Platform refusal; tested Linux/Windows real decoder behavior, fresh A1 component/security review and existing full `rust-verify` regression gates. No new dependency is implied or pre-approved by this candidate: if all required formats cannot be verified safely within an Owner-issued ledger, the later work order must narrow the proof honestly or STOP rather than assert a header-only media-valid result.

**Exit conditions:** Every eligible acquired Entity image is either **actually** media-verified against its authored type/role, exact source bytes and width/height/pixel limits with bounded resource behavior, or yields a deterministic whole-candidate refusal; corrupted declared images fail even when their declarations are internal, unlisted or suppressed. No incomplete/unsupported body, ambiguous identity, automatic Platform/role fallback or unrelated unreferenced file is accepted; source provenance and five-level privacy/inheritance are preserved; all applicable positive and negative gates pass on Ubuntu and Windows with no new unsafe dependency or publication path. This slice does **not** verify video/captions, establish the Platform `1.0.0` catalogue or licences, produce public URLs, generate derivatives, validate a complete mixed-namespace publishable snapshot, or authorize rendering, release or deployment. R03.C and Stage 03 remain OPEN.
```

## 5. Mandatory administrative acceptance matrix

| ID | Independently verifiable result |
| --- | --- |
| `S066-01` | Agent proves exact accepted `main` SHA/tree/report blob, all four WIP baseline blobs, single checkout and origin, Owner-installed complete R01 ACTIVE/matching handoff and absence of unrelated dirty/unknown material. |
| `S066-02` | One issuance commit changes **only** ACTIVE; remote readback of its blob/parent/tree; full green Ubuntu/Windows CI at issuance SHA before further action. |
| `S066-03` | Conditional local and remote pruning of **only** the three named 065 branches occurs after exact-tip/ancestor/no-other-worktree/no-active-PR/protection safety checks, otherwise is deferred honestly; never force; original divergent 061 tip retained byte-for-byte on both refs. |
| `S066-04` | ROADMAP is one literal-line R03.C replacement matching §3D, exactly one insertion/deletion; already accepted Entity acquisition not presented as unfinished; all remaining media, Platform/library, whole-candidate, privacy and derivative gates remain; Stage 03/04–13 untouched. |
| `S066-05` | NEXT is byte-identical to §4, exactly one `R03.C3` heading, exactly five purity fields and no second candidate; NEXT grants no code authority, Platform/media publication claims or hidden fallback. |
| `S066-06` | ACTIVE becomes exact one-heading neutral file; OUTLOOK blob unchanged; WIP contains exactly four files without archives or extra subfolders. |
| `S066-07` | One closure commit changes **only** ROADMAP/NEXT/ACTIVE, pushes normally, and passes complete Ubuntu/Windows CI at its exact SHA. |
| `S066-08` | Architecture, About, AGENTS, SOP, all product source/test, Cargo lock/manifest, toolchain, workflow, security policy, `.gitignore`, assets/catalogues and unissued code remain unchanged. |
| `S066-09` | The canonical report alone changes in the last commit; its remote blob/tree/parent read back, both-host final-head CI green, no parallel report, and all ancestor/diff/WIP/ref checks accurate. |
| `S066-10` | Nonfatal inherited `Unicode-3.0` warning remains **UNVERIFIED** absent an independently justified disposition; no suppressions, skipped mandatory gates, test softening or invented supply-chain clearance. |
| `S066-11` | Single checkout/worktree/remote, clean final Git status, preserved build cache and inherited archives, truthful task-owned temporary cleanup, no release/deploy/provider/extra repository action; Agent states `COMPLETE` only for administrative scope and does not self-ACCEPT or begin R03.C3. |

## 6. STOP conditions, boundaries and final handoff

STOP without unauthorized repair if current `main`/tree/report/WIP/branch anchors do not match, Owner-installed ACTIVE or matching handoff is absent, ROADMAP's exact old line differs, unauthorized files or Owner-owned material are dirty, safe Git/ref handling is not possible, mandatory CI fails, or the documentation change would weaken Architecture or falsely mark unfinished work completed. Do not conceal or force past any defect. For a **prunable branch only**, safely defer its optional deletion and report the reason when doing so does not threaten repository integrity; this is not permission to rewrite or destroy it.

The Agent's final handoff must state `SOLO-WO-066 R01 — COMPLETE` (administrative scope only) or the truthful `STOPPED/FAILED` disposition; exact issuance/closure/report commit SHAs, trees, paths and WIP blobs; conditional branch outcomes; all CI run/job evidence; inherited warnings, skips and cleanup inventory; and a link to the one canonical remote report at the final HEAD. The Chief Architect then independently audits the published report and issues `ACCEPT / REVISE / STOP`. **No autonomous R03.C3 code order, implementation, release, deployment, media artwork or Platform catalogue follows from an unissued NEXT candidate.**
