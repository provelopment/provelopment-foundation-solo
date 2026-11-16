# Execution Report

**Work order:** `SOLO-WO-118 R14` — repair Windows checkout byte identity for the pinned Foundation donor and complete N126 on two native hosts.
**Required outcome:** `R03C-FOUNDATION-DONOR-81-ICON-BYTE-IDENTITY-WINDOWS-REPAIRED-N126-TWO-HOST-V1`.

**status=COMPLETE** (for that bounded outcome only)

**owner_per_asset_approval_pending=true** · **rights_review_pending=true** · **platform_library_delivered=false** · **platform_read_enabled=false** · **publication_authority=false** · **stage03_open=true**

The R13 STOP was justified and stands as history: its Windows host failed on byte-preservation preconditions while Linux passed. R14 applies the single adjudicated Git attribute and re-proves the untouched packet on both native hosts. No artwork is approved, no Platform library installed, no read enabled, nothing published, released or deployed.

## 1. Transaction and exact changed-path ledger

| Step | Full SHA | Validation |
|---|---|---|
| baseline | `main` R21b `2e39e919935287c0eff28e1b4c79304a2d747ef7`; technical T21.1 `72425de9d5c91ec03f10089ce0db4810c36ee045` | one `origin` (`provelopment/provelopment-foundation-solo`), one worktree, four `.documents/WIP/` files, empty stash, both trees clean |
| **A22** (`main`, ACTIVE only) | `b2db73f39038e14e7e972ad7845a0372d2d47867`, tree `fe3e66c1efe207dc5c9ef7b5906553b4f608f687`, parent `2e39e919` | ACTIVE blob `0fa3890ba0b5c567d377a35f396953258e4f599e` identical in worktree, index, commit and `origin/main`; staged set exactly `.documents/WIP/workorder-active.md` |
| **T22.0** (branch, ACTIVE only) | `a03fc30e0d3586b6606468d312e15925e5595306`, parent `72425de9` | `git restore --source=A22` of one path; identical ACTIVE blob; `git diff --name-only T21.1..T22.0` = ACTIVE only |
| **T22.1** (branch, one attribute line) | `5e26672e4f68f26967b9b98ab6d70383db34ccdb`, parent `a03fc30e` | `.gitattributes` blob `027359ecea739c325da73236f3441579e7b5d342` → `b22f49ab43c3b226fad80b6cf20551af48937be3`, 379 → 442 bytes, `--numstat` = `1 0` (pure addition, no modified line) |
| **R22** (`main`, report only) | this commit (SHA in the Owner handoff) | parent `b2db73f3`, exactly one path |

`git diff --name-only T21.1..T22.1` = `.documents/WIP/workorder-active.md` + `.gitattributes`, `--numstat` = `82 85 ACTIVE` and `1 0 .gitattributes`. No test, fixture, catalogue, sheet, workflow, manifest, lockfile, `deny.toml` or product source changed. Main and the technical branch stay separate: **no merge, no rebase, no cherry-pick, no force push, no `git clean`, no `reset --hard`.**

## 2. Root cause, and proof the fix is scoped

**R13 history preserved unaltered.** Technical run `43804143667` attempt 1 at T21.1: Ubuntu `133090902932` ✅, oracle `133090902700` ✅, **Windows `133090902898` ❌**. There, `icon-about.svg` hashed `bbe534455dbcebfa1b99b9fe248c22c91c8adad01d3e269de064798b139415b0` (455 bytes) and the MIT notice `dbfa6cff2e8426878267c6719ff68dcd8bc3dadaffb2207f5c46ce939c4becd3` (1094 bytes) — exactly the CRLF projections of the 439-byte and 1073-byte LF sources, because root `.gitattributes` carried `* text=auto` and no exception for the LF-only review fixtures.

**The only change** is one appended rule in the already tracked root `.gitattributes`:

```diff
@@ -18,3 +18,4 @@
 *.woff binary
 *.woff2 binary
 *.zip binary
+crates/solo-cli/tests/fixtures/foundation-donor-icons/** -text
```

**Attribute precedence, measured with `git check-attr` (not inferred):**

| Path | `text` | `eol` |
|---|---|---|
| `…/foundation-donor-icons/icons/icon-about.svg` | **unset** | unspecified |
| `…/foundation-donor-icons/icons/icon-home.svg` | **unset** | unspecified |
| `…/foundation-donor-icons/licensing/TABLER-ICONS-MIT.txt` | **unset** | unspecified |
| `…/foundation-donor-icons/licensing/icon-provenance.json` | **unset** (beats the generic `*.json text eol=lf`) | `lf` |
| all **83** scoped paths | **83 / 83 unset** | — |
| `crates/solo-cli/tests/fixtures/other-sample.svg` (outside subtree) | `auto` | unspecified |
| `crates/solo-cli/tests/fixtures/avif/alpha_transform_referee.py` | `auto` | unspecified |
| `.documents/Reports/platform-library-1.0.0-foundation-candidate.json` | `set` | `lf` |

`text: unset` is Git's "no end-of-line conversion" state, so the scoped subtree is excluded from the inherited `text=auto` policy while every unrelated path keeps it. No `.gitattributes` exists inside either fixture directory and none was added.

## 3. Byte identity: all 83 fixtures, every stage

For **each** of the 83 tracked fixtures the donor blob at Foundation `fa599973a88ff77f6a8e012f3266c3c139b52c17` (tree `89b57a92869d2e293d33527589f3f4680938f9b2`) equals the T21.1 committed blob, the T22.1 committed blob, the index blob and the working-tree `git hash-object` — **83/83 identical, zero mismatches**, and the three withheld names (`icon-status-info.svg`, `icon-volunteer.svg`, `icon-services.svg`) remain absent from the subtree.

| Anchor | Donor / T21.1 / T22.1 / index / worktree blob | Source SHA-256 |
|---|---|---|
| `icons/icon-about.svg` | `c407077e4f849eaf536908a9ee444c4aa4752af4` | `bf4377dff0aaaddf1676a4bb7292f90f1102f19daaca7021331ba6a191317c1c` |
| `licensing/TABLER-ICONS-MIT.txt` | `3e82379dab3fe93d9ee22251949604ed63ddea39` | `b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db` |
| `licensing/icon-provenance.json` | `a74d234417d5bb63ff05e2b64f93d757864f3d46` | `e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b` |

**Raw bytes, not just blobs.** The hosted Windows job's own clean `actions/checkout` is the disposable fresh Windows checkout required by §3 (no second clone or worktree was created, per the repository's one-checkout rule). There, unmodified N126_01 reads each fixture as raw bytes and compares their SHA-256 with the catalogue; all 81 matched, including `icon-about.svg` at `bf4377df…` and the MIT notice at `b740a1d4…`, with **no CRLF expansion**. In the Owner checkout the same raw bytes verify for the 83 fixtures plus both packet files. No normalised digest was computed, no newline translated, no assertion disabled, no fixture rematerialised in CI, and no `core.autocrlf` override, subdirectory attribute or build-time transformation is involved. `git status --porcelain` and `git diff --check` are clean after every commit.

**Packet unchanged.** Both review artefacts are the *same Git blobs* as T21.1:

| Artefact | Blob (T21.1 = T22.1) | Bytes | Canonical LF SHA-256 |
|---|---|---|---|
| `platform-library-1.0.0-foundation-candidate.json` | `a02db1605bac6cf1b45214a07b452d58895d7742` | 65 397 | `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` |
| `platform-library-1.0.0-foundation-review.md` | `edb8468d7ef6ae290574672e1829bf5b583419ca` | 75 313 | `095942fb96521ddac088b799f4321e914c0db3e915de0bd416ae2a0c97ed49aa` |

**Disclosed correction:** the digests quoted in R13 and in this order for those two files (`33a8872b…` and `4d7b432e…`) were computed over the **Windows CRLF working-copy projection** of the same content (66 779 / 71 492 bytes), while the committed blobs are 65 397 / 75 313 bytes of LF. Blob identity proves the content never changed; the canonical LF digests above are the authoritative ones, and the working tree now matches them exactly. The catalogue still holds exactly 81 entries with `platformAssetLibraryVersion: "1.0.0"`, and no approval, rights or publication field was added.

## 4. N126 re-proved unchanged on both native hosts

No test, verifier, workflow or golden was touched. Command, unchanged: `cargo test -p solo-cli --test media_worker_host_smoke --locked n126_ -- --nocapture --test-threads=1`.

**Final technical run `43807461257`, attempt 1, at one SHA `5e26672e4f68f26967b9b98ab6d70383db34ccdb`:**

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133101393481` | **success** |
| `verify (windows-2025)` | `133101393733` | **success** (previously the failing host) |
| `avif-reference-oracle` | `133101394241` | **success** |

Both hosts, independently:

- `n126_01_exact_eighty_one_tabler_donor_vectors_real_confined_workers` → `originals=81 inspected=81 real_children=81 geometry=24x24 donor_pinned=true source_scope=review-fixture publication=false`, **81 genuine confined SVG child invocations**, canonical cross-host aggregate **`3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`** — identical on Ubuntu, on Windows and in the local WSL run.
- `n126_02_pinned_mit_provenance_and_r12_catalogue_review_only` → `entries=81 mit_covered=81 originals_withheld=1 duplicate_aliases_withheld=2 review_only=true platform_read=false publication=false`, `mit_sha=b740a1d4…`, `provenance_sha=e0f0ea62…`, `donor_commit=fa599973…`, `donor_tree=89b57a92…`.
- `n126_03_forged_source_alias_missing_notice_and_platform_authority_refused` → `forgeries_refused=true alias_digest_refused=true licence_missing_refused=true donor_drift_refused=true platform_unavailable=true zero_authority=true review_only=true platform_read=false publication=false`.
- Real log verified per host: `n126_gate authentic=accepted`, then all four corrupted copies rejected → `n126_gate mutation=rejected`. Ubuntu N126 gate: `3 passed; 0 failed; 0 ignored; 64 filtered out`; Windows N126 gate: `3 passed; 0 failed; 0 ignored; 58 filtered out` (the six Linux-only host tests are compiled out on Windows), and the Windows workspace step also shows all three N126 cases `... ok`.
- Every retained gate is green at the same SHA: full `cargo test --workspace --all-targets --locked`, fmt, check, strict Clippy, safe-code/forbidden-import, architecture `packages=4 targets=36 files=70 edges=93` (unchanged), cargo-deny with a fresh RustSec refresh, native Btrfs source-reader canaries, N093/N102/N108/N113/N116/N120–N125 with their adversarial log gates, release reproducibility and CLI smoke, and the independent AVIF reference oracle. No dependency, lockfile, `deny.toml` or CI semantics changed; no `continue-on-error`, `--skip`, `#[ignore]`, `|| true` or fake token was introduced.


## 5. Local verification and honest constraints

- Local (WSL, real `solo-cli` children): N126 `3 passed; 0 failed; 0 ignored`, 81 confined children, the same aggregate; `cargo fmt --all -- --check` 0; `cargo check --workspace --all-targets --locked` 0; `cargo clippy … -D warnings` 0; `workspace_graph` and `check_architecture_fixtures` green; `check-architecture` `legal packages=4 targets=36 files=70 edges=93`.
- **NOT RUN LOCALLY, truthfully:** native Windows linking (no `link.exe` in the Owner checkout) and the Btrfs-backed `source_read_native::snapshot_proofs` cases (no verified `SOLO_TEST_SOURCE_FS_ROOT`; the local workspace run shows 87 passed / 5 precondition failures). Those are precondition gaps, not product defects, and the hosted Ubuntu/Windows CI above is the authoritative evidence for both hosts.
- The dedicated attribute is the only mechanism used; the Owner's existing checkout was neither destroyed nor force-rewritten, and no disposable second checkout was created because the repository permits exactly one.

## 6. Rights, product and publication boundaries — unchanged and closed

- The 81 MIT candidates stay **review-only** test fixtures; the per-asset review sheet remains **PENDING OWNER PER-ASSET APPROVAL — NOT DISTRIBUTABLE**.
- No artwork approval, no approval token, no derivative, no optimised variant, no extra catalogue entry, no alias reinstatement, no `icon-services.svg` licensing decision and no third-party platform/social mark was added.
- The Tabler MIT notice is retained unchanged as evidence and must travel with any separately authorized future distribution; the Foundation root Apache-2.0 licence neither replaces it nor grants rights in names, logos or trademarks.
- `platform/assets/` does not exist; `AssetNamespace::Platform` stays unreadable by the Entity reader and the V4 matcher's `PlatformSource` remains refused (`PlatformSourceUnavailable`). All rights/publication/derived-output/release flags remain constant `false`.
- This success is **only** cross-host proof of a review packet. It is **not** `R03.C`, not a `platformAssetLibraryVersion: "1.0.0"` installation, not a release, publication or Stage 03 closure.

## 7. Repository state and cleanup

- `main` = A22 `b2db73f3` then R22 (report only); technical branch = T22.1 `5e26672e`, both pushed, both clean; one worktree; one `origin`; exactly four `.documents/WIP/` files; empty stash; `git status --porcelain` and `git diff --check` clean.
- **Observed remote heads: 30** — the 28 long-standing heads plus `dependabot/cargo/svgtypes-0.16.2` (PR #3) and `dependabot/cargo/syn-3.0.10` (PR #2), which were created by the Dependabot app, not by this agent. They were not merged, reviewed, rebased or modified; they would change the locked graph if merged, which is a separate Owner decision.
- Task scratch (attribute patch script, donor tree listing, proof script, transaction scripts) lived only in untracked `.temp/` and was deleted before handoff. Pre-existing Owner scratch in `.temp/` was left untouched.

## 8. Disposition and next Owner step

`status=COMPLETE` for `R03C-FOUNDATION-DONOR-81-ICON-BYTE-IDENTITY-WINDOWS-REPAIRED-N126-TWO-HOST-V1`: one scoped `-text` rule fixed Git's Windows materialisation, all 83 donor blobs and raw bytes are identity-proved, and the unchanged N126 suite passes 3/3 with 81 real confined workers on both native hosts at one exact SHA, with authentic/mutated-log gates and all three technical jobs green.

**Remaining Owner decision (not delegated):** review the 81-row per-asset packet and approve (or reduce) the exact bytes, per-asset licensing/provenance and redistribution scope for the first pinned `platformAssetLibraryVersion: "1.0.0"` release. Only a later Owner-issued order may install `platform/assets/`, its hash-pinned catalogue and a separate read-only Platform root, and prove real approved Platform reads. Withheld items — `icon-services.svg` (Provelopment original, no licence inferred) and the duplicate aliases `icon-status-info.svg` / `icon-volunteer.svg` — each need their own explicit decision. `R03.C` and Stage 03 remain **OPEN**.

