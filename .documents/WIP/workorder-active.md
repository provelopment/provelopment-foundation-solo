# SOLO-WO-118 R09 — PIN THE PUBLISHED UPSTREAM ALPHA-TRANSFORM REPAIR; PROVE N123 RED → GREEN ON BOTH HOSTS

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Canonical ACTIVE:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-AVIF-UPSTREAM-ALPHA-COMPOSITION-REPAIR-PINNED-RED-GREEN-TWO-HOST-V1`.

## 1. Binding Chief Master Architect decision

`SOLO-WO-118 R08` is **ACCEPTED**, **only** for independent adjudication of the broken associated-alpha transform. Its tests and independent native logs prove that the *correctly associated* `abc_color_irot_alpha_irot.avif` yields 14,460 differing alpha samples (zero RGB differences) in `oxideav-avif 0.0.16`, while the immutable independently pinned R03 RGBA display digest is `950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`, alpha range `128..255` and 123,952 nonopaque. The negative `abc_color_irot_alpha_NOirot.avif` lacks a required alpha `irot` association and **remains invalid media by policy**, regardless of what either decoder renders. R08's independent `avifdec`/dav1d/Pillow referee and every R03 golden stay authoritative and immutable.

**Authorize precisely one upstream registry upgrade: `oxideav-avif = "=0.0.16"` → `oxideav-avif = "=0.0.18"`, with `default-features = false, features = ["registry"]` unchanged. Do not vendor, patch, fork, use Git dependencies, add a codec, re-pin any reference, lower a gate, or add a fallback.** The Architect independently verified these versioned public facts, not a moving master branch:

- Official crates.io registry index `rust-lang/crates.io-index` at `ox/id/oxideav-avif` has **0.0.18 published, not yanked**, checksum **`fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee`**; 0.0.16 was `f36a98615006596baed023d26c16d58bf9327126c919b716ebfdf8122fee8c78`.
- Upstream Git tag **`OxideAV/oxideav-avif@v0.0.18`**, MIT, Rust MSRV **1.95** (Solo's accepted CI Rust is 1.99.1, so no lower toolchain); `oxideav-heif >=0.0.13,<0.1` is satisfied by Solo's *existing locked* 0.0.15. The crate's registry dependency names are still exactly `oxideav-av1`, `oxideav-core`, `oxideav-heif`; those are already locked at 0.1.24, 0.1.43, 0.0.15 respectively.
- Upstream **`CHANGELOG.md` 0.0.18, 2026-11-10, “Fixed”** explicitly states: an alpha auxiliary carrying its own transformative properties is attached to the *colour item's output image*, rather than being resized onto the reconstructed image and transformed a second time; a no-transform auxiliary continues through the master's transforms. The **tagged** `src/decoder.rs` now applies the master's transforms before attachment when the auxiliary is transformed, and returns without applying the master's transforms twice.
- This is an upstream repair hypothesis with strong source evidence, **not yet an accepted Solo test result**. The fixed version must reproduce the immutable RGBA hash, zero alpha differences, the four R03 native-depth plane goldens, real N121 on two hosts and every retained security gate. A newer tagged source, Git master, a different checksum or a version-range dependency is **not** authority.

**Deliberately bounded scope:** R09 changes the third-party dependency used by the real product and proves its corrected behavior; it does **not yet authorize the child/parent to accept alpha images**. The existing confined child must continue to refuse all `has_alpha=true` images (including the valid reference fixture) at its R06 coded-plane policy boundary. That refusal is a *temporary limitation*, not a declaration that the valid fixture is bad. The N121 refusal canary must remain green **unaltered**. A subsequent separate order will authorize the additional four-plane, associated-alpha, correctly oriented class and the real-host N122 matrix. **No N122, V4, R03.C, Stage 03, catalogue, publication, release or deployment claim here.**

## 2. Exact baseline and authorised transaction: A17 → T17.0 → T17 → R17

Require before touching anything:

- `origin/main` exactly **`b74e5e3ec760a8d8ab60e8e8af64c7ded7b04a43`** (R16 canonical adjudication report). Parent A16: `021bdc3900ec9f11b7b3014ecb15bb3708bfc5da`.
- The existing, sole technical branch **`solo-wo-118-avif-v4-r01`** exactly **`448a7a04d6b587d9756b70ef1b8efe5a5d7a159d`** (T16.4), with accepted N121, R03 reference oracle, and R08 N123 referee. Its parent history includes `d800cb77b823ca397084cc97ce992954845a6563`.
- One existing checkout and worktree, one approved `origin`, **28** existing remote heads (do not create or delete one), empty stash, clean prior trees, exactly four `.documents/WIP/` files, no PR/merge/tag/rebase/release/deploy, and no unknown changes. Record exact Git refs and `git status --porcelain=v1`; never `reset --hard`, `git clean`, force push or silently discard Owner material.

Read the repository's `AGENTS.md`, execution SOPs, three WIP plans, foundation briefing, Architecture §§5A/6.2G/6.2I/6.2K/6A.8, accepted work order and R16 report; read **actual** `Cargo.toml`, `Cargo.lock`, `deny.toml`, the N120, N121 and N123 tests, source/host protocols and CI workflow before editing. The accepted architecture must remain four first-party crates, strict hexagonal inward boundaries, and no publication authority.

1. **A17 on `main`:** commit/push only the Owner-installed **new** `.documents/WIP/workorder-active.md` over R16. Verify the exact blob agrees across working tree/index/A17 commit/`origin/main`; record full SHA, tree and parent. Neither report nor code changes here.
2. **T17.0 on the existing technical branch:** require the exact T16.4 tip above and a clean tree. Import only the A17 ACTIVE blob via `git restore --source="$A17" --staged --worktree -- .documents/WIP/workorder-active.md`; check `git rev-parse ':.documents/WIP/workorder-active.md'` equals `git rev-parse "$A17:.documents/WIP/workorder-active.md"` and the staged path set contains only ACTIVE. Commit/push ACTIVE-only; do not re-import old R08 order or main technical content.
3. **T17 technical:** perform §§3–5 in ordinary, forward commits on that *same* existing branch. **First** make the N123 repair assertions intentionally RED against locked 0.0.16 (do not publish a red branch as a misleading pass). **Then** upgrade solely to 0.0.18, prove the N123 assertion GREEN, retain N120 and N121, update N123 CI gate to accept **repair** rather than the historical defect, and obtain native verification. Before each technical push run the local host-independent checks and capture exact results. On the **one exact final T17 SHA**, the *push-triggered* `verify (ubuntu-24.04)`, `verify (windows-2025)` and `avif-reference-oracle` jobs must all be successful, with new load-bearing repair evidence on both native hosts. Do not rely on `workflow_dispatch` to prove the branch-scoped oracle.
4. **R17 on `main`:** after the technical outcome is independently recorded and all worktrees clean, commit/push **only** the overwritten `.documents/Reports/execution-report.md`; verify fresh `main` Ubuntu and Windows baseline jobs. State the R17 full SHA **in chat only**, rather than a report self-reference. If a proved API, compatibility, security, resource or native counterexample prevents exact completion, report `status=STOPPED`, preserve the actual code and logs, and do not call a partial/test-only result complete. The inability to run Btrfs real-source tests locally is not a blocker when the hosted native tests exist.

The Architect performs **read-only** connected GitHub verification. The Implementation Agent alone performs Owner-authorized Git writes. Never alter a protected branch or produce product changes on `main` before a separately issued integration order.

## 3. Closed technical ledger and actual red-to-green repair

On the technical branch permanently edit only:

```text
.documents/WIP/workorder-active.md                    # exact A17 blob, no edit
crates/solo-adapters/Cargo.toml                        # one pinned oxideav-avif version + truthful explanatory comment
Cargo.lock                                             # targeted resolved package version/checksum only
crates/solo-adapters/tests/graphic_avif_inspection.rs  # N123 real pixel RED→GREEN assertions; retain two older N120 tests
.github/workflows/rust-verify.yml                       # N123 authentic+mutated log verifier; preserve all older gates
```

**Exception, not a default:** `crates/solo-adapters/src/graphic_avif_inspection.rs` may receive the minimum API-compatibility change **only if** the release genuinely changed an invoked function/type and the exact compiler error proves it. Such a change may not widen the currently accepted media classes, alter OS limits, weaken validation or accept alpha. Cite the exact compiler error and diff in the report. Otherwise the production runtime source remains byte-identical to T16.4. No new paths are authorized.

No root `Cargo.toml`, `[patch.crates-io]`, `deny.toml`, vendored source, Git dependency, new licence, `crates/solo-domain`, `crates/solo-application`, `media_worker_host*`, `media_worker_protocol.rs`, `source_read*`, `solo-cli/src/main.rs`, AVIF fixture binary, R03 `reference-goldens.json`, `reference_oracle.py`, R08 `alpha_transform_referee.py`, provenance, architecture-checker source, CI security knobs, N093/N102/N108/N113/N116/N121 tests, WIP plans, SOP or Architecture changes. Preserve byte-for-byte V3 `avif-unsupported-v3`, existing Png-only favicon and every publication-is-false property.

### 3.1 Audit the exact one-package upgrade

Only `crates/solo-adapters/Cargo.toml`'s direct requirement may change to:

```toml
oxideav-avif = { version = "=0.0.18", default-features = false, features = ["registry"] }
```

Use a **targeted** resolver action such as `cargo update -p oxideav-avif --precise 0.0.18`, then compare **all** old/new lock entries before any push. Require `Cargo.lock` to record `oxideav-avif 0.0.18` from the **official crates.io registry**, checksum `fcaf60ff8283c30fc9e84e462fc9a9e016f5f4e4f1ab1817a15cebf2885b1cee`, still exactly the three dependency names `oxideav-av1`, `oxideav-core`, `oxideav-heif`. Require the locked `oxideav-av1 0.1.24`, `oxideav-core 0.1.43`, `oxideav-heif 0.0.15`, every other crate version/checksum/source/features and the complete graph to remain unchanged. If Cargo cannot resolve **exactly** this isolated change, **STOP with its actual resolver output**; do not authorize broad lockfile drift, a dependency override or a second duplicate node by inference.

The MIT licence is already permitted in strict `deny.toml`; the Rust MSRV 1.95 is within the project's native 1.99.1. That permits **evaluation**, not exemption from RustSec, `cargo-deny`, no-duplicate/no-wildcard, architecture, unsafe, component or native-host gates. The version is not allowed from any Git URL or floating tag. Document the registry checksum, upstream tag, licence and dated 0.0.18 fix in the report for the outstanding later release NOTICE/provenance obligation; do not claim the external NOTICE obligation closed in R09.

### 3.2 N123 test: require a genuinely failing pre-upgrade state, then exact success

The existing `crates/solo-adapters/tests/graphic_avif_inspection.rs` has **three tests**: N120 8/10/12-bit goldens, N120 negative fixture disjointness, and `n123_associated_alpha_irot_decoder_differential_is_measured`. Keep all three and their canonical fixed fixture paths. Strengthen the *existing* N123 test, rather than adding a second unrelated self-test, to assert the **independent positive reference**:

- Fixed source hashes for the valid and invalid fixtures; independently checked **equal coded colour and alpha item payloads**, exact primary/alpha IDs; negative remains invalid by metadata *policy*, even if the codec returns a picture.
- Positive and negative decoder display geometry 256×512, 8-bit YUVA444P, four canonical stride-256 planes of 131,072 bytes each; the alpha plane in `try_to_rgba8()` matches plane 4 byte for byte; the RGB samples remain identical across the two decoded sources. The test-only decoder comparison **does not mean** the real child accepts either source.
- **Positive** packed RGBA length 524,288 bytes, SHA-256 **exactly** `950be879c536c208d4e2dc0cdb8aa698e73922b4a57e8070c4fea389c84cc744`, alpha minimum **128**, maximum **255**, non-opaque count **123952**. This equals the immutable `reference-goldens.json`'s valid-positive `rgba_display_sha256` and measured statistics; no re-pinning or hard-coded product pixels.
- **Positive and negative decoded** RGBA buffers must now be exactly equal on every sample, with **`alpha_differences=0`, `plane_alpha_differences=0`, `rgb_differences=0`**, consistent with R08's **independent** referee returning `same_coded_rgba=true same_display_rgba=true` and zero differences. `positive_matches=true` must be a measured fact, not a printed assertion token. A negative *raw decode* match never promotes an invalid metadata association to a valid MediaWorker acceptance.
- The existing R08 historical counterexample remains preserved in R16, its immutable Git run logs and the previous commit. In the repaired test do **not** keep an assertion that 14,460 wrong alpha samples are desirable. Update the old diagnostic output to `n123_decoder repair_verified=true` **only after all exact assertions pass**, with actual hashes, geometry/counts, and `n123_evidence alpha_differences=0 rgb_differences=0`/`positive_matches=true` output derived from measured buffers.

**True RED before GREEN:** With the test tightened and the lock still at 0.0.16, run `cargo test -p solo-adapters --test graphic_avif_inspection --locked n123_associated_alpha_irot_decoder_differential_is_measured -- --nocapture` on the local working checkout (this test does *not* require Btrfs). Capture the real nonzero result and the actual failing equality, 14,460 disagreement and positive SHA if printed. **Do not** bypass the failing check or push a new mandatory RED suite as if it passed. Then perform the version upgrade above; repeat exactly the command with `--locked` and require actual exit 0 with the real positive pin and zero alpha differences. The old 0.0.16 evidence may also be shown from R16 only if the in-progress local RED attempt is technically unavailable; then report that limitation instead of inventing a new negative run.

N120's four direct non-alpha goldens must stay byte-exact and green after upgrading, as must the already accepted **real-child** N121 2/2 tests on both native hosts, including the refusal of *all* alpha (temporarily still unsupported by the R06 worker). If the new decoder changes even one pinned 8/10/12-bit plane, produces the right RGBA under a different format, or breaks an N121 or component gate, R09 **fails** until a separate Owner decision; do not weaken goldens or assert a self-generated replacement value.

### 3.3 CI N123 transition to repair (not measurement of an old defect)

Modify only the existing N123 Linux and Windows proof steps in `.github/workflows/rust-verify.yml` (or the smallest adjacent verifier statements): run the **authentic single N123 test** and require `test result: ok. 1 passed; 0 failed; 0 ignored`, the exact one named test, the actual `n123_decoder repair_verified=true`, `positive_matches=true`, `alpha_differences=0`, `rgb_differences=0`, the positive expected 64-character hash and nonopaque count 123952. **Reject** a stale historical `n123_decoder measured=true` pass and reject old `alpha_differences=14460` evidence. The test's repaired facts must come from real decoded bytes, not from `echo` in the workflow. Require the same authentic-log verifier to reject at least three altered scratch copies: deleted case/token, falsely changed alpha difference/hash and false pass count, before printing `n123_gate authentic=accepted` and `n123_gate mutation=rejected` on each host. Do not modify old N121 gates or reference job; do not omit genuine runner exits.

Retain all existing Ubuntu Btrfs fixture setup/guaranteed cleanup, native Windows reader, N093/N102/N108/N113/N116/N121, locked Cargo, official-source and component-security checks, Btrfs/source/unsafe/architecture/forbidden-import fixture, 23-fixture refusal self-test, release/reproduction and CI tripwire gates. No `continue-on-error`, `#[ignore]`, `--skip`, masked exit, `|| true` around an acceptance command or arbitrary filesystem fallback. R08's separate independent referee step still measures the same two fixtures and must remain green without modification. The `avif-reference-oracle` remains a **separate** branch-`push` job with a read-only token; success of its goldens never waives a red native verification job.

## 4. Local-first verification and native acceptance

Before each technical push record actual command, exit and output:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check-architecture --locked
cargo test -p solo-adapters --test graphic_avif_inspection --locked -- --nocapture
cargo test -p solo-application --test media_worker --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked --no-run
cargo test --workspace --all-targets --locked
```

Expect a four-package repository and **`architecture=legal packages=4 targets=34 files=66 edges=93`** on the technical branch, provided the graph is truly unchanged; verify actual output. `workspace_graph` must stay 32/32. The R03 oracle's pinned manifest, inputs and hashes must be untouched. Real-source tests that require `SOLO_TEST_SOURCE_FS_ROOT` may be marked **NOT RUN LOCALLY** on the Owner's non-Btrfs WSL; local Windows linkage with absent `link.exe` is also NOT RUN LOCALLY. Do not disguise either as passed, suppress old tests or substitute a weaker read mechanism. Hosted Ubuntu 24.04 must run with the already approved disposable Btrfs fixture; hosted Windows 2025 must use its actual native reader.

The final **one identical technical commit** must be proven green by **all three** fresh **push-triggered** jobs: `verify (ubuntu-24.04)`, `verify (windows-2025)`, `avif-reference-oracle`. Each verification log must show the **repaired** N123 positive digest/alpha 0 difference evidence, N120 goldens, N121 real-child 2/2 and valid hostile refusal semantics, N123 deliberate log mutation rejection, and all retained gates. The oracle log must show `pristine_manifest_matches=true`, `mutation_rejections=3/3`, the R08 `n123_reference same_coded_rgba=true same_display_rgba=true alpha_differences=0 rgb_differences=0`, `alias_refusal=verified`, and `cleanup=complete`. Preserve and report failed attempts even if a later fresh attempt passes. A failed job at the final commit is **not** acceptance; N093/N102 intermittent failures are never waived.

## 5. Canonical report and explicit completion boundary

Overwrite only `.documents/Reports/execution-report.md` on `main` with `SOLO-WO-118 R09` outcome. Include: full A17/T17.0/T17 technical SHA(s) (R17 self-SHA in chat only), parent/tree/blob validation, exact 0.0.18 registry checksum and edition/MSRV/licence, pre/post lock graph (show only targeted package changed), exact local pre-upgrade **RED** and post-upgrade **GREEN**, the positive and negative RGBA actual hashes, `alpha_differences=0`, RGB0, geometry, statistics, unchanged R03/N120/N121, official external referee and source provenance, N123 native proof and deliberate forged-log rejection, required CI run and job IDs/head/attempt/conclusions, fresh R17 baseline Ubuntu and Windows run, architecture/security/unsafe/snapshot gates and all local NOT RUN constraints. Confirm 28 existing remote heads, one worktree/`origin`, exactly four WIP files, empty stash, both branches clean, no extra branch/worktree/PR/release, and removal of only task-created temporary scratch.

`status=COMPLETE` **only** for `R03C-AVIF-UPSTREAM-ALPHA-COMPOSITION-REPAIR-PINNED-RED-GREEN-TWO-HOST-V1` when the exact 0.0.18 registry upgrade and no other dependency/source drift are proven, the **valid** fixture's corrected RGBA digest and zero delta are proven on both hosts, all four 8/10/12-bit independent plane goldens remain exact, real N121 2/2 and genuine alpha refusals remain intact, and all three branch CI jobs are green at the same final SHA. A test-only report, unpinned git source, local-only pass, skipped N121, alpha policy expansion, mismatched reference or changed safety gate **cannot** qualify.

**After R09:** the next Owner-issued order will explicitly add the *associated-alpha* V1 YUVA444P/essential-`irot` worker and parent policy, perform genuine N122 real-host positive and negative tests (including the necessary transparent migration of N121's former valid-alpha refusal), then separately integrate the whole six-format source/owner candidate and Platform catalogue. **Do not claim V4, R03.C or Stage 03 closure from a corrected codec alone.**

**Execute the actual official upstream decoder repair and prove it now.**
