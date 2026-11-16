# SOLO-WO-118 R14 — REPAIR WINDOWS CHECKOUT BYTE IDENTITY FOR THE PINNED FOUNDATION DONOR; COMPLETE N126 ON TWO NATIVE HOSTS

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's sole working checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Canonical ACTIVE destination:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required bounded outcome:** `R03C-FOUNDATION-DONOR-81-ICON-BYTE-IDENTITY-WINDOWS-REPAIRED-N126-TWO-HOST-V1`.

## 1. Binding architectural adjudication: R13 STOP justified; repair is one Git attribute

Chief Master Architect independently audited the canonical R21b report, precise Git refs, the root `.gitattributes`, and native CI logs. **SOLO-WO-118 R13 is not ACCEPTED or COMPLETE**: its technical implementation and Owner-review packet exist at **T21.1** but the two-host N126 proof is unfinished. The **STOP is accepted as justified**; carry the failed history and its root cause forward unaltered.

Pinned repository refs at issuance:

| Authority | Exact SHA | What it proves |
|---|---|---|
| `origin/main` R21b | `2e39e919935287c0eff28e1b4c79304a2d747ef7` | Current canonical `status=STOPPED` report, including correction for Dependabot-created heads |
| `origin/solo-wo-118-avif-v4-r01` T21.1 | `72425de9d5c91ec03f10089ce0db4810c36ee045` | R13 immutable 81-icon packet and unchanged N126 tests |
| Foundation donor commit | `fa599973a88ff77f6a8e012f3266c3c139b52c17` | Pinned original source (never track donor `main`) |
| Foundation donor tree | `89b57a92869d2e293d33527589f3f4680938f9b2` | Original donor blob manifest |
| Technical CI run `43804143667` | Ubuntu job `133090902932` **green**; Windows `133090902898` **red**; oracle `133090902700` **green** | Same T21.1 SHA, no red-job waiver |
| R21b main baseline run `43805633976` | Ubuntu `133095645294` **green**; Windows `133095645137` **green** | Baseline is distinct from technical branch |

**Root cause is Git materialization, not malformed icon source or codec.** Tracked root `.gitattributes` at T21.1 has blob **`027359ecea739c325da73236f3441579e7b5d342`** and the rule `* text=auto` but no exception for the LF-only review fixtures. On Windows, working-tree `icon-about.svg` became 455-byte CRLF data with SHA-256 `bbe534455dbcebfa1b99b9fe248c22c91c8adad01d3e269de064798b139415b0`, while the committed/donor 439-byte LF source is SHA-256 **`bf4377dff0aaaddf1676a4bb7292f90f1102f19daaca7021331ba6a191317c1c`**. The MIT notice became 1,094-byte CRLF data SHA-256 `dbfa6cff2e8426878267c6719ff68dcd8bc3dadaffb2207f5c46ce939c4becd3`, while its committed/donor 1,073-byte LF bytes are SHA-256 **`b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db`**. Windows N126 **all three failed** on byte-preservation preconditions; all prior Windows test bodies in the same step passed. Linux N126 three cases passed, **81 genuine child invocations**, and its live-log authenticity/mutation gate passed; the independent AVIF oracle passed. Neither the Windows failure nor the existing frozen donor bytes may be normalized away.

**R14 sole technical decision:** add one precisely scoped `-text` rule to the **already tracked repository-root** `.gitattributes`, following existing rules. `-text` disables Git CRLF checkout conversion for this immutable donor fixture subtree; it does not modify canonical Git blob contents, sanitizer, fixture bytes, catalogue hashes, rights records or any product implementation. The ordinary general `*.json`/`*.md` `eol=lf` safeguards outside that subtree remain intact. This is a fix to how Git checks out immutable test inputs, not a broader binary/text governance revision.

## 2. Preflight, transaction and absolutely closed path ledger

Operate only in the Owner's established single checkout. Verify clean `main`, clean `solo-wo-118-avif-v4-r01`, empty stash, exactly **one** `origin`, exactly one worktree, four `.documents/WIP/` files (`plan-next.md`, `plan-outlook.md`, `plan-roadmap.md`, `workorder-active.md`), actual remote refs and the exact pinned main/technical commits above. Check no pending untracked artefacts would be overwritten. Git remote-head count was **30** at R21b because two Dependabot app PR branches arrived (not 28); record the present count and distinguish external changes, **do not fail solely because the external count changes**. No Dependabot merge or update is part of this order. If an actual pinned head moved, an Owner change exists, an index is dirty, or the root is ambiguous, **STOP** with evidence. Never reset hard, force-push, `git clean`, merge, rebase, cherry-pick, rebase or rename the branch; never touch another working directory.

The **entire authorized change ledger** is:

```text
.documents/WIP/workorder-active.md      # A22 on main; byte-identical import T22.0 only
.gitattributes                          # T22.1; exactly ONE appended attribute rule; no other edit
.documents/Reports/execution-report.md # R22 on main; report-only final commit
```

Everything else is **immutable and out of scope**: all 81 donor SVGs; the two pinned `licensing/` files; the 81-entry candidate catalogue; the Owner's 81-row review sheet; `media_worker_host_smoke.rs` N126 and all earlier tests; `.github/workflows/rust-verify.yml`; every Cargo manifest/lock file, `deny.toml`, toolchain, crate, architecture checker and policy; the entire V1–V4, source reader, worker/supervisor, catalogue parser, AVIF oracle/fixture/golden and output/authorization code. Do not add `.gitattributes` files within fixture directories or test-scoped scripts to the index. Temporary local evidence commands are allowed in `.temp/` only if untracked, task-owned and removed before handoff. **No production `platform/` directory or asset library may appear.**

Transaction:

1. **A22 (main, ACTIVE only):** After Owner installs **this** downloaded order at the canonical ACTIVE destination, start from R21b `2e39e919935287c0eff28e1b4c79304a2d747ef7`, stage **only** `.documents/WIP/workorder-active.md`, inspect the staged path set, commit and push A22. Verify working-tree/index/committed/`origin/main` ACTIVE blob identities, exact A22 parent and tree SHA. Do **not** commit `.gitattributes` on main: the main branch is an intentionally separate baseline.
2. **T22.0 (technical branch, ACTIVE import only):** Start from T21.1 `72425de9d5c91ec03f10089ce0db4810c36ee045`, import **only** `.documents/WIP/workorder-active.md` from A22 by its full SHA, stage/commit this one path, and verify exact blob identity with A22. Preserve all T21.1 product and packet bytes. There is no merge from main.
3. **T22.1 (technical branch, one attribute rule):** Append **exactly** the following line after the existing binary rules in repository-root `.gitattributes` (preserve all preceding bytes and LF style):

   ```gitattributes
   crates/solo-cli/tests/fixtures/foundation-donor-icons/** -text
   ```

   There must be exactly one occurrence, with no broader `*.svg`, `*.txt`, `*.json`, repository-wide, other-fixture or root-path change. Stage **only** `.gitattributes`; verify `git diff --cached --name-only` equals that path, and `git diff --cached -- .gitattributes` is a one-line addition with no deletion or modified old line. Commit as T22.1. The two technical commits relative to T21.1 must change **exactly** ACTIVE and `.gitattributes`, not test or packet content. Push to the **same** `solo-wo-118-avif-v4-r01` branch normally (no force), then use the push-triggered CI.
4. **R22 (main, report only):** Only after the required final technical verification exists, return to main A22; publish the single canonical `.documents/Reports/execution-report.md` with a **report-only** commit R22 and push. Run fresh two-host main baseline CI and report results. R22's own SHA is supplied in the Owner handoff rather than falsely embedded into its own committed file. A failed technical verification is recorded as `status=STOPPED`, not upgraded to COMPLETE.

## 3. Proof that the attribute applies but does not rewrite any source

**Before** the T22.1 commit, read the root `.gitattributes` and confirm the exact existing rules. Inspect the 83 tracked fixture paths under `crates/solo-cli/tests/fixtures/foundation-donor-icons/`: **81** `icons/icon-*.svg` plus **exactly two** licensing records (`licensing/TABLER-ICONS-MIT.txt`, `licensing/icon-provenance.json`). There may be no extra files beneath that subtree. Confirm the candidate packet files under `.documents/Reports/` exist and are unchanged.

After applying the rule, prove from actual Git attribute precedence (not a comment or an inference):

```bash
git check-attr text eol -- \
  crates/solo-cli/tests/fixtures/foundation-donor-icons/icons/icon-about.svg \
  crates/solo-cli/tests/fixtures/foundation-donor-icons/icons/icon-home.svg \
  crates/solo-cli/tests/fixtures/foundation-donor-icons/licensing/TABLER-ICONS-MIT.txt \
  crates/solo-cli/tests/fixtures/foundation-donor-icons/licensing/icon-provenance.json
```

Each must resolve **`text: unset`**, including the JSON file notwithstanding the earlier generic `*.json text eol=lf`. Check all 83 paths under the scoped subtree as well; unexpected attribute overrides or any file outside this exact scope changing attribute behavior **STOP**. A comparison for an unrelated ordinary SVG outside the subtree must still resolve under inherited policy, establishing that the exception is scoped.

**Blob identity is an independent obligation:** for **each of all 83** fixtures, compare (a) the immutable donor blob at Foundation `fa599973a88ff77f6a8e012f3266c3c139b52c17`, (b) the technical Git index and T21.1 committed blob, and (c) the T22.1 committed blob. Git blob SHA must be **byte-identical** at every stage; the exact input SHA-256 from the R13 review sheet/catalogue is untouched. Representative anchors:

```text
icons/icon-about.svg: donor Git blob c407077e4f849eaf536908a9ee444c4aa4752af4
  exact LF SHA256 bf4377dff0aaaddf1676a4bb7292f90f1102f19daaca7021331ba6a191317c1c
licensing/TABLER-ICONS-MIT.txt: donor Git blob 3e82379dab3fe93d9ee22251949604ed63ddea39
  exact LF SHA256 b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db
licensing/icon-provenance.json: donor Git blob a74d234417d5bb63ff05e2b64f93d757864f3d46
  exact LF SHA256 e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b
```

The R13 review-only catalogue `.documents/Reports/platform-library-1.0.0-foundation-candidate.json` retains **exactly 81 entries**, SHA-256 `33a8872be4ba131db58ca37d27b7bfb57166315248399e66701e59777c76119a`. The Owner review sheet `.documents/Reports/platform-library-1.0.0-foundation-review.md` retains SHA-256 `4d7b432e594498582ebc8e8c4995e7cd8dbf00547a022c0eb3cc347ebe77dabd`. Both are reviews, **not** production catalogue approval. Review dates, legal evidence and Owner approval fields may not be edited to manufacture clearance.

**Working-tree bytes are also material.** In a **fresh, disposable** native Windows clone/checkout of the final technical commit—or equivalently the clean `actions/checkout` already used by the hosted Windows job—read all 83 fixture files as raw bytes. They must match their T22.1 Git blobs and the donor; no file may show CRLF expansion, especially `icon-about.svg` and the MIT notice. Do not compute an adjusted/normalized digest, translate newlines in tests, disable the original assertion or rematerialize tracked fixture bytes in CI. The dedicated Git attribute is the only permitted mechanism. Existing Owner working trees must not be destroyed or force-rewritten to simulate a clean checkout. If the current Owner checkout already has old CRLF projections, use a separate disposable checkout to prove the new rule; **do not stage, commit or silently overwrite altered donor bytes**. `git status --porcelain` and `git diff --check` must be clean after the final commits.

The implementation agent must explain if any `core.autocrlf` override, subdirectory attribute or build-system transformation prevents raw identity; it must **STOP** rather than writing an extra workaround. Raw SHA-256 is the source rule, not `git hash-object` after a clean filter.

## 4. Re-prove unchanged N126 with real workers on BOTH native hosts

**No new test cases, branches or mutable goldens.** The previously committed N126 tests and existing two-host N126 verifier in `.github/workflows/rust-verify.yml` are load-bearing and must remain byte-identical. The same test command runs in native hosted CI:

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked n126_ -- --nocapture --test-threads=1
```

Both Ubuntu 24.04 and Windows 2025 must independently show `test result: ok. 3 passed; 0 failed; 0 ignored` and exactly the original N126 case names and evidence:

1. **`n126_01_exact_eighty_one_tabler_donor_vectors_real_confined_workers`** — `originals=81 inspected=81 real_children=81 geometry=24x24 donor_pinned=true source_scope=review-fixture publication=false`; **81 actual** confined SVG worker invocations, one per distinct immutable source, no fake port, no single-worker substitution; canonical cross-host aggregate **`3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca`**. Do not alter a row, source digest, canonical output digest, `currentColor` policy or worker-count assertion.
2. **`n126_02_pinned_mit_provenance_and_r12_catalogue_review_only`** — `entries=81 mit_covered=81 originals_withheld=1 duplicate_aliases_withheld=2 review_only=true platform_read=false publication=false`; exact pinned MIT digest **`b740a1d46122672da62833e97f7e7c8a13fa85cbc7445b584b297cc00dde93db`**, provenance digest **`e0f0ea621c11140cf581d86aaab0b73421d0202114092a4e58dc3321ff1a325b`**, committed donor SHA and the unchanged strict R12 parser.
3. **`n126_03_forged_source_alias_missing_notice_and_platform_authority_refused`** — `forgeries_refused=true alias_digest_refused=true licence_missing_refused=true donor_drift_refused=true platform_unavailable=true zero_authority=true review_only=true platform_read=false publication=false`, including actual refusal paths; nothing reads `platform/assets/` or treats a review catalogue as a publication token.

On **both** hosts, the existing N126 verifier must print **`n126_gate authentic=accepted`** and **`n126_gate mutation=rejected`**, having validated the real hosted log and rejected all four deliberately falsified copies (case deleted, 80 instead of 81, forged publication/Platform-read flags, corrupt MIT digest). Keep the original `n126_row` source/canonical digests, canonical 81-row aggregate and test scripts. Do not change `.github/workflows/rust-verify.yml` just to make a green result easier.

**Existing green tests are mandatory too.** At the **same exact final technical SHA**, the normal push CI must show all three successful jobs: `verify (ubuntu-24.04)`, `verify (windows-2025)`, and `avif-reference-oracle`. Preserve the full `cargo test --workspace --all-targets --locked`, fmt, check, Clippy, architecture and forbidden-import/unsafe checks, cargo-deny / current RustSec refresh, genuine R03 native source-reader/Btrfs canaries, N093/N102/N108/N113/N116/N120–N125, every earlier adversarial log gate, release/CLI smoke and byte-for-byte reproducibility. Do not bypass Windows CI, dispatch a nonmatching branch, exclude N126 from the workspace suite, weaken CI `continue-on-error` semantics, update dependencies or merge Dependabot work. Architecture on the technical branch is expected to remain `packages=4 targets=36 files=70 edges=93`; independently record the measured actual result and stop if the graph changes unexpectedly. The **main** baseline remains separately at its own package/target/file/edge counts.

Local Windows linkage and native Btrfs snapshot verification may remain **NOT RUN LOCALLY** where the Owner checkout lacks those native prerequisites; never describe these cases as locally passed or classify precondition panics as product defects. The final hosted Ubuntu/Windows CI is authoritative. If the CI tests produce a new source/hash/codec failure, the single-line attribute repair is **not proven** and R14 must stop for a new, independently adjudicated bounded order.

## 5. Rights, product and publication boundaries stay CLOSED

R14 does **not** approve individual artwork, add an approval token, ship artwork, install an actual Platform root, copy Foundation brand artwork, license `icon-services.svg`, include duplicate aliases as extra catalogue entries, introduce third-party platform/social marks, create optimised derivatives or publish anything. The 81 MIT candidates are **review-only** and require explicit Owner approval at the per-asset review sheet before any product-distribution package. The MIT notice must be retained unchanged as evidence and must travel with any separately authorized eventual distribution. The Foundation root Apache-2.0 licence does **not** replace this third-party MIT notice, nor grant rights in logos/trademarks. `platform/assets/` remains absent; `AssetNamespace::Platform` remains unreadable by the Entity reader and V4 matcher `PlatformSource` remains refused.

The product still has constant-`false` rights/publication/derived-output/release authority flags. R14 success is **only** cross-host proof of a review packet; it is **not** `R03.C`, a completed `platformAssetLibraryVersion: "1.0.0"` installation, a product release, publication or Stage 03 closure. The next separate Owner review will examine the 81 actual rows, individual rights evidence, intended use and attribution; it is not delegated to the Implementation Agent by this order.

## 6. Required R22 report and exact STOP/COMPLETE semantics

On success publish `status=COMPLETE` for **only** `R03C-FOUNDATION-DONOR-81-ICON-BYTE-IDENTITY-WINDOWS-REPAIRED-N126-TWO-HOST-V1`. The canonical `.documents/Reports/execution-report.md` must include:

- A22 full SHA/tree/ACTIVE blob equality, T22.0 full SHA/blob equality, T22.1 full SHA and exact one-line attribute diff, R22 report-only parent/full SHA in the Owner handoff. Confirm main-vs-technical separation and exact changed-path ledger.
- Original T21.1 Windows **FAILURE**, its specific CRLF hashes, the exact Git attribute precedence before/after, and all 83 committed donor blob identities **and** fresh Windows worktree-byte identities. Keep R13's justified STOP as history; do not rewrite it into a previously successful run.
- Three real final technical CI job IDs/conclusions/attempts at **one identical SHA**; N126 full case names, pass counts, source/canonical digest and all evidence tokens on both hosts; verifier authentic/mutation verdicts and retained N093–N125 and R03 reference oracle; architecture facts, no dependency drift, no rule weakening; any local Btrfs/Windows constraints truthfully separated.
- Exact SHA-256 for the **unchanged** 81-entry candidate catalogue, 81-row review sheet, MIT notice and provenance; all three withheld names, 81 original LF icon files still test-only; no added production `platform/assets/` or host Platform reader.
- Fresh `main` baseline Ubuntu/Windows run/job IDs after R22; actual one-worktree/one-remote/clean branches/empty stash/WIP count and **observed** remote heads (externally created Dependabot heads distinguished from this agent's edits); task scratch cleanup.
- Explicit **`owner_per_asset_approval_pending=true`**, **`rights_review_pending=true`**, **`platform_library_delivered=false`**, **`platform_read_enabled=false`**, **`publication_authority=false`**, **`stage03_open=true`**.

If any bounded requirement is unsatisfied, publish **`status=STOPPED`** with exact cause, host logs and smallest necessary new proposal; never forge green, mutate the donor, silently expand the ledger or request broad approval. No PR, merge, release, deploy or publishing claim may be made.

**Final Owner handoff:** exact A22/T22.0/T22.1/R22 SHAs, final CI run and three job IDs, first and final Windows N126 outcomes, R22 baseline CI, complete/review-only status, remaining Owner rights approval, and the next dependency. The Chief Master Architect alone separately audits and ACCEPTS this bounded outcome. Stage 03 remains OPEN.
