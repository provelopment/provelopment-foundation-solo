# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-069 R01**  
**Title:** R03.C3 AVIF decoder runtime, provenance, and enforceable-resource-bound qualification on Linux and Windows  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching explicit Owner→Implementation Agent handoff ID:** `SOLO-WO-069 R01`  
**Execution authority:** This complete, issued work order directs the Implementation Agent. The Owner installs this exact document as `.documents/WIP/workorder-active.md` and delivers the matching handoff. Chief Architect GitHub access is **READ ONLY, permanently**; all authorized Git and implementation actions are performed only by the Implementation Agent from the verified Solo checkout. This order authorizes **a separate, non-product AVIF proof branch** and the required canonical report. It authorizes **no product decoder adoption or Stage 03 closure**.

## 1. Required outcome and immutable starting anchors

**Required result:** Execute real, controlled AVIF **pixel decoding** and hostile-input probes using the **exact** `oxideav-avif =0.0.11`, `oxideav-av1 =0.1.23`, `oxideav-core =0.1.43` candidate graph on **both** `ubuntu-24.04` and `windows-2025`, independently assess the package's origin and licensing/security posture, and determine whether Solo's mandatory 20 MiB input / 1–8192 width-height / 32,000,000 decoded-pixel / bounded CPU and working-memory contract can be enforced *before and throughout decoding*. The outcome must be a fully evidenced **QUALIFIED**, **BLOCKED**, or **UNVERIFIED** decision for this candidate only. A completed investigation is not the same as a qualified decoder. No application runtime is to be modified or published.

**Expected exact starting `main == origin/main`:** `74f302825f1ffc82202e54fcdfd0feb7679c03da`  
**Expected starting tree:** `babe210fe55d825ce2977c2387cdc827847cbd4d`  
**Current report blob:** `60eabdbca10c1bbb8696024b879ff99e6fdce6c5` (`SOLO-WO-068 R01 — COMPLETE`, research scope only).  
**Current ACTIVE blob before this issuance:** `cb3f9af82aaeb5116eb8254ccee9c0ddf187bf7b` (`SOLO-WO-068 R01`).  
**Unchanged WIP blobs:** NEXT `92fa562b1af0c54391851aa5c79deb1631127069`, ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`, OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`.  
**Existing branches:** `main` at the baseline and separately preserved divergent `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6`; do not alter the latter.  
**Prior final-head CI:** `rust-verify` run `43145325013`, completed successfully on Ubuntu and Windows at baseline `74f3028…`.

**Binding reading, before writes:** `README.md`; `AGENTS.md` §§1–7; both SOPs, especially Architect SOP §§6A, 6B, 7, 7A, 7B, 8 and Implementation SOP §§4, 5, 5A–5C, 7A–7B; Architecture §§5A, 6.2I, 6A.8; all four WIP files; current exact-revision execution report; `Cargo.toml`, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, the current `.github/workflows/rust-verify.yml`, existing relevant media tests and source types. Previous research findings are evidence, **not** decoder qualification or permission to weaken policy.

**Contract preserved without change:** First release must recognize PNG, JPEG, WebP and AVIF; images are ≤20 MiB, both dimensions 1–8192, total decoded pixels ≤32,000,000, with actual decode and format/MIME agreement, fail-closed source handling, controlled CPU and allocations, no unsafe first-party bypass, no substitute format or filename/header-only success. `graphic.favicon` remains PNG-only. Corrupted declared media blocks the entire candidate, including unlisted/internal/suppressed owners. This order tests **AVIF feasibility**, not the whole four-format implementation; `R03.C3`, `R03.C`, Stage 03 remain **OPEN**.

## 2. Exactly authorized permanent paths and operations

### On `main` (and `origin/main`): only two files

1. `.documents/WIP/workorder-active.md` — the **Owner installs this complete R01 text**. Agent stages and commits that exact file **alone**, without changing its contents; pushes this one-path issuance commit to `origin/main` and verifies the exact remote blob, parent and tree.
2. `.documents/Reports/execution-report.md` — overwrite **once at completion** with the factual result of this order; stage/commit **only this file** on `main`, push, remote-readback, and require completed successful existing `rust-verify` on both hosts at the final report HEAD. The report never embeds its own future commit SHA.

No other path on `main` is writable. In particular do **not** edit `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`, Architecture, About, SOPs, product Cargo files or `Cargo.lock`, `deny.toml`, `rust-toolchain.toml`, source, existing tests or `rust-verify.yml`. WIP remains exactly four files.

### On a single temporary technical branch `solo-wo-069-r01`

Create **one** short-lived branch from the freshly issued `main` commit, within the **existing single checkout**. On that branch only, create or edit exactly:

- `verification/avif-r03-c3/Cargo.toml`
- `verification/avif-r03-c3/Cargo.lock`
- `verification/avif-r03-c3/src/main.rs`
- `verification/avif-r03-c3/src/lib.rs` and files beneath `verification/avif-r03-c3/tests/` strictly necessary for the described deterministic probe
- `.github/workflows/avif-qualification.yml` — one **branch-specific**, read-only-token, two-host qualification workflow that executes the standalone proof crate, repeatable security/graph checks and bounded runtime probes.

The standalone proof crate **must include its own `[workspace]` section** so it does not join the production four-crate workspace, cannot alter the production lockfile or affect production builds, and is clearly **non-product**. No other first-party path, workflow or file may be edited, and no third-party fixture, copied vendor source or archive may be committed. Do not merge this branch into `main`, create a PR, or publish a release. Push only this exact named technical branch to `origin` to run its authorized CI proof; preserve it intact for independent review. The work-order-specific branch is **review evidence**, never product delivery.

### Disposable material and external operations

Use only the verified Solo `.temp/` and/or the actual Cline task temporary workspace for task-owned scratch, downloaded official crate sources, generated test vectors, test outputs and temporary measurements. Reuse the ordinary ignored Rust `target/` build cache as necessary. Cleanup task-owned scratch at run end, leaving unknown, existing or Owner-owned files untouched. No second checkout, worktree, nested Git clone, alternate Solo tree, stash, reset, `git clean`, rebase, force push or history rewrite.

Permitted external reads: crates.io packages/index/checksums, canonical published source for the exact three pinned oxideav components, RustSec/OSV/GHSA and vendor security notices, and the existing GitHub CI/branch/status APIs. Permitted writes: Git commits and non-force pushes to exactly `origin/main` for the two paths above and to `origin/solo-wo-069-r01` for the enumerated experimental paths. No provider account, secrets, infrastructure, deployment, service integration, dependency installation into the product, licensing exception, GitHub administration change, or write to any other repository. A permitted `rustup` install of the **already pinned `1.99.1` toolchain on CI runners** is part of CI execution; it is not permission to modify the Owner's local toolchain. Do not install NASM, C decoders, unknown native tools, or a different compiler.

## 3. Mandatory preflight, issuance and CI gates

1. Confirm the real checkout root using `git rev-parse --show-toplevel`, exact `origin`, one worktree, `main`, exact starting HEAD/tree, remote tracking, no conflicts/sequencer, clean tracked/untracked state **except** the Owner-installed ACTIVE replacement. Verify its exact `SOLO-WO-069 R01` marker, same handoff ID and whole contents. Verify the four WIP blobs and canonical report at §1's values. Do not overwrite unrelated local material. STOP safely if any identity/permission drift is found.
2. On `main`, stage **only** the installed `.documents/WIP/workorder-active.md`, commit `Issue SOLO-WO-069 R01: AVIF runtime qualification`, and push normally to `origin/main`. Read back exact parent/tree/blob and verify other tracked paths unchanged. Require successful **existing** `rust-verify` at this issuance commit on both `ubuntu-24.04` and `windows-2025` before branch work. If CI or readback is unverified/red, STOP and publish only a safe report.
3. Create `solo-wo-069-r01` from the verified issuance SHA. No second local checkout. Preserve the unrelated `solo-wo-061-r01` branch byte-for-byte. Check remote for a same-named branch first; STOP rather than overwrite an unexpected preexisting branch.
4. No product dependency or fixture is allowed to be added during this order. Record the starting and final full SHAs/tree SHAs in the final report (the final report commit SHA itself is supplied in chat, not embedded).

## 4. Build the exact standalone AVIF proof harness

Create `verification/avif-r03-c3/Cargo.toml` with this **exact dependency identity**; additional third-party roots are prohibited:

```toml
[package]
name = "solo-avif-r03-c3-proof"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]
resolver = "3"

[dependencies]
oxideav-avif = { version = "=0.0.11", default-features = true }
oxideav-av1 = { version = "=0.1.23" }
oxideav-core = { version = "=0.1.43" }
```

Lock and track **only this probe crate's** `Cargo.lock`. `cargo metadata --manifest-path verification/avif-r03-c3/Cargo.toml --locked --format-version 1` must show exactly those direct roots and every resolved transitive, with package name, version, source, checksum and features. Never substitute newer `oxideav` packages or a floating release. If registry resolution rejects the exact set, STOP the technical probe and record the actual conflict rather than changing the selected versions.

For the decoding API, use the real exact-package source as the authority. The documented 0.0.11 interface includes the explicit-limits `decode_with(bytes, &DecodeOptions)` interface and the `AvifDecoder::decode_file` registry decoder. Start the proof with **explicit** caller limits, not unlimited/default limits, for example:

```rust
// Illustrative API invocation: align imports and result accessors to
// the inspected 0.0.11 published source before committing.
let options = oxideav_avif::api::DecodeOptions::new()
    .with_max_width(Some(8_192))
    .with_max_height(Some(8_192))
    .with_max_pixels(Some(32_000_000))
    .with_max_bytes(Some(20 * 1024 * 1024))
    .with_strict(true)
    .with_threads(Some(1));
// Call the exact 0.0.11 `decode_with` and assert real output planes/pixels.
```

**Important semantic distinction:** The candidate's `DecodeOptions.max_bytes` is documented as an **input-file-byte ceiling**, **not** a total allocation or decoded-working-set ceiling. Do not claim it solves the missing `DecoderLimits` enforcement. Explicitly inspect source call sites for pre-allocation dimension checks, all internal allocations, tile/grid/alpha/sequence work, and the decoder's actual complete-file/error behavior. An `inspect`, successful `ftyp`, `av1C` or parsed `AvifInfo` without **decoded pixel-plane evidence** is a failed proof.

Use a checked arithmetic precheck (`width.checked_mul(height)` after conversion to `u64`, not `u32` multiplication), actual 1–8192 bounds, the input-size cap before calling the decoder, and a complete pixel-plane fingerprint of the returned decoded frame. Never accept a file based only on header geometry. The harness must emit machine-readable **one-line JSON per case** (fixture ID, source checksum, result PASS/FAIL, decoded width/height/depth, plane lengths, decoded-pixel SHA-256, time, and measured memory when available), without leaking absolute private paths or raw attack bytes. Write the complete, deterministic decoded plane sequence to a task-owned, non-committed raw-output file with explicit plane order, stride, byte depth and lengths; calculate SHA-256 over **those actual output bytes** using `sha256sum` on Ubuntu and `Get-FileHash -Algorithm SHA256` on Windows. Hash the original input file separately and include both hashes in the per-case record. Do not add a hashing crate, commit decoded output or hash a printed metadata description in place of pixels. Explicitly compare the decoded-byte hash for each same fixture across the two hosts. If the decoder cannot produce actual bytes, mark the pixel gate `FAIL`/`UNVERIFIED`; never invent an output digest.

## 5. Exact positive/negative proof matrix

**Positive real decode:** Inspect the **published** `oxideav-avif 0.0.11` crate's shipped test-fixture directory via its resolved `cargo metadata` package `manifest_path`. Locate and enumerate its **12 named AVIF fixtures** identified in the WO-068 report: `alpha_video`, `bbb_alpha`, `black420`, `gray32`, `kimono_rotate90`, `midgray`, `monochrome`, `red`, `white16`, `y_plane_divergence_match`, `y_plane_roundtrip_avif1`, `y_plane_roundtrip_avif2`. Derive actual names/extensions from the installed published package rather than guessing. For each fixture actually present, decode the full file; assert width and height within the contract, actual nonempty decoded planes, correct plane sizes/strides, no partial or metadata-only success, and a full decoded-pixel fingerprint. Compare identical fixture fingerprints across Linux and Windows. Record bit depths, alpha, grid/transform and sequence coverage truthfully: a filename is not proof that a feature was exercised. Use an independent expected pixel/golden value for at least one ordinary image and one high-bit-depth/alpha/transform case, grounded in a reproducible external reference or independently established expected samples. A self-produced fingerprint compared only to itself is **not** proof of pixel correctness. If the published crate lacks any named fixture, explicitly report it; do not silently replace missing input with a fixture from an unapproved source.

**Additional controlled variants:** Exercise actual 8-, 10- and 12-bit decode and alpha, grid and transformed geometry where available in the published, clearly licensed test assets. If an exact variant is absent, build a small synthetic test vector only with already authorized, locally executed pure-Rust machinery and explain its origin, or mark that variant `UNVERIFIED`; a self-encoder round trip alone is not an independent oracle. No native encoder, external package install, unknown corpus, fixture licence uncertainty or network source substitution is permitted.

**Mandatory refusal cases:** Test each under the exact bound configuration: zero bytes, random short bytes, invalid/truncated `ftyp`, header-only or `av1C`-only payload, truncated valid AVIF at several positions, corrupted AV1 tile/extent, invalid brand, trailing garbage under strict mode, declared oversized 8193 width or height, over-32M total pixels, and >20 MiB input. Create controlled mutations of a copied, authorized test vector; when a dimension mutation is used, show that it reaches the actual dimension gate rather than failing earlier for an unrelated malformed container. Assert no acceptance of incomplete data, no panic/abort or hang reported as success, no quiet partial-image return, and no leaking of unlisted/nonpublic inputs. If a negative case cannot reach the intended layer, classify that exact gate `UNVERIFIED`, not `PASS`.

**Bounding evidence:** For each fixture, record actual runtime and peak process memory on both hosts in separate OS processes; apply a hard finite wall-clock timeout to every hostile decode so a hang cannot stall the workflow. Record the exact timeout and peak-memory measurement mechanism, its reliability and raw CI evidence. Independently inspect the crate for a **working, audited allocation/cancellation budget** across AV1 decode, planes, alpha, grids and metadata. OS peak RSS alone is observational and **never** proves a library-enforced allocation cap; the `DecoderLimits` type being declared, but not wired into the invoked path, is likewise not proof. If no enforceable memory/CPU limit exists, classify the bounded-decoder requirement `BLOCKED` regardless of ordinary fixtures passing. Never bypass the resource contract by raising the approved image limits or suppressing a failure.

**Provenance and supply chain:** Inspect the actual published tarballs and Cargo checksums for all three exact roots; explain with tagged published-source evidence the `oxideav-av1 0.1.23` “orphan-rebuild scaffold” description and its disagreement with pixel-decoder claims. Establish provenance, licence/attribution/NOTICE duties, published date, package ownership and maintenance signals; enumerate exact normal/build/dev dependencies, `unsafe`, FFI, assembly/build scripts, duplicated package versions, yanks, and advisory exposure against current RustSec/OSV/GHSA/vendor notices. Run the **same pinned `cargo-deny 0.20.2` verification policy** on this separate lockfile on CI, using the existing `rust-verify.yml` verified scanner acquisition pattern and an explicit `--manifest-path` for the probe; never relax the repository `deny.toml` rules. A green product scan is not a scan of the probe graph. Record current-version/LTS evaluation without substituting versions. No legal, security or provenance issue may be waived by the Implementation Agent.

## 6. Mandatory two-host branch-specific CI

Create `.github/workflows/avif-qualification.yml` **only on `solo-wo-069-r01`**. Requirements:

- Trigger `push` **only** to `solo-wo-069-r01` plus an optional manual `workflow_dispatch`; `permissions: contents: read`; no secrets or write tokens, no unsafe third-party action or floating action version. Reuse the exact **full commit SHA** of `actions/checkout` already approved in `rust-verify.yml` and `persist-credentials: false`. Never change `rust-verify.yml`.
- Run separate actual `ubuntu-24.04` and `windows-2025` jobs, fixed `rustc/cargo 1.99.1`, a `cargo fetch --locked` of the **standalone probe** during a clearly network-enabled acquisition step, then offline `cargo test --locked --all-targets` and `cargo run --locked` against the same probe manifest. Record exact runner/tool identities and fixture/probe package checksums.
- Explicitly run the positive fixtures, adversarial failures and per-process timeout/memory observations on **both** hosts; fail the proof workflow when an asserted mandatory test fails or a fixture silently disappears. Distinguish a **qualifying negative finding** (e.g. library lacks allocation controls) from CI infrastructure failure: the harness should still produce unambiguous evidence even when its verdict is `BLOCKED`, and no CI step may swallow a genuine test failure.
- Run `cargo-deny 0.20.2` against the standalone graph with the unaltered Solo policy and refresh/check advisories using the same verified, pinned process as existing CI. Any unrun scanner or new material warning is `UNVERIFIED`, never green security proof.
- Install no additional OS-native libraries, assembler, C codecs, local Git repositories, external test executables or unreviewed Actions. Never run code received from a random URL. Fetch exact registries through Cargo and perform self-tests using the compiled fixed-version Rust packages.
- Set an explicit job timeout and emit concise reproducible case-by-case evidence, including errors and any skipped checks. Preserve CI run and job URLs in the final report. **No claim that an ordinary successful Cargo build proves decoder correctness or security.**

After publishing the branch, wait for both platform jobs to complete and inspect logs/check conclusions. Correct **only the explicitly allowed proof files** when a harness bug makes a test unreliable. The agent may commit/push bounded fixes to `solo-wo-069-r01` with clear commit messages and repeat full two-host evidence at the **final branch HEAD**. Do not modify the `oxideav` dependency versions, waive limits, change policy or introduce alternate decoders as a troubleshooting shortcut. A genuine decoder defect, resource-budget failure, provenance concern or no hosted CI execution is a result to report, not a reason to invent success.

## 7. Required result, canonical report and preservation

Before leaving the branch, record exact branch HEAD/tree/remote SHA, workflow run IDs, each host result and no unspecified changed paths. Switch safely back to `main`, without deleting or resetting the proof branch. If the switch is unsafe due to unknown local data, STOP and preserve it; do not attempt destructive cleanup. On clean `main`, overwrite **only** `.documents/Reports/execution-report.md` with:

1. Work-order ID, handoff, session date, exact starting and issuance commits/trees/blobs, permitted branch HEAD/tree, remote names, root/worktree/branch inventory and path ledger.
2. The complete candidate lock/feature/normal-build-dev graph with checksums, component-security/licence/provenance inspection dates, exact tooling version, scanner outcome and explanation of the “orphan-rebuild” mismatch.
3. Positive cases with exact provenance, dimensions, bit depth, alpha/grid/transform facts, decoded-pixel SHA-256 evidence and independent-oracle status; Windows/Linux outcomes and discrepancies. Separate *decode produced pixels* from *pixels externally verified correct*.
4. Negative/refusal matrix with specific failure codes, exact guard reached, hard timeouts, runtime/peak memory evidence, and an honest finding on internal allocation/CPU enforcement. Record why any applicable test did not run.
5. A single decision: **`QUALIFIED FOR A FUTURE OWNER DECISION`** only when all mandatory runtime, security/provenance and hard resource gates actually pass; **`BLOCKED`** on a demonstrated mandatory failure; or **`UNVERIFIED`** on essential missing evidence. State exactly what next smallest Owner decision/code order would be required. **Never** call the AVIF codec product-approved or `R03.C3` complete.
6. Existing `rust-verify` issuance and final report CI (both hosts); branch qualification CI (both hosts); real command outcomes; all warnings, unexpected source drift and available remediation. Preserve inherited `Unicode-3.0` notice warning as an outstanding non-suppressed release obligation. Disclose every `NOT RUN`/`UNVERIFIED` claim.
7. Only positively identified task-owned `.temp/`/Cline temporary artifacts cleaned; retained `target/` and unrelated archived user material untouched. Report clean `main`, branch preservation and exact remote readback.

Commit only the canonical report on `main` and push normally to `origin/main`; verify remote content blob/tree/parent. Require completed successful **existing** Linux/Windows `rust-verify` at this **exact final report commit**, with no material new warning, no mandatory bypass and no unexplained skip. Never include the report's own future SHA in its text. Give the final report commit SHA, report blob, tree, branch SHA and both-host run links in the chat handoff. The Implementation Agent does **not** issue the next order, accept its own work, modify ROADMAP/NEXT, merge anything, or make a release/deployment claim.

## 8. Explicit stop rules and acceptance gates

**Immediate STOP and report safely:** unexpected root/remote/worktree/branch state; unmatched Owner handoff or wrong ACTIVE contents; preflight SHA/blob mismatch; unidentified dirty content; non-fast-forward push; inaccessible GitHub; inability to verify issued CI; unexpected branch-name collision; prohibited first-party or root lockfile edits; attempted dependency/version substitution or policy relaxation; suspicious source/build script/native requirement; secret exposure; unsafe/unknown cleanup; unexpected irreversible action. A **decoder that genuinely fails a runtime/resource/security test** is recorded as `BLOCKED` without altering the product or adding a workaround; the investigation can still be complete if its evidence and publication gates are met.

| Gate | Mandatory reviewed evidence |
| --- | --- |
| `S069-01` | Exact preflight; correct handoff and Owner-installed ACTIVE; one checkout/worktree/`origin`; four WIP files; historical divergent branch preserved. |
| `S069-02` | ACTIVE-only issuance commit and exact remote blob/parent/tree; original `rust-verify` green on both hosts at issuance. |
| `S069-03` | One isolated `verification/avif-r03-c3` Cargo probe and one branch-only read-only CI workflow on `solo-wo-069-r01`; no production code/lock/policy change. |
| `S069-04` | Exact `oxideav` pins and graph/checksums/features; source, copyright/licence/provenance, RustSec/OSV/GHSA and version-currency evaluation; candidate-specific unchanged-policy `cargo-deny` result or correctly reported unverified. |
| `S069-05` | Real pixel-plane decoding on both hosts, reproducible output fingerprints and explicit independent expected-pixel verification; all positive/negative variant gaps correctly marked. |
| `S069-06` | Actual cap/refusal/timeout/memory evidence; explicit assessment of enforceable internal CPU/allocation bounds, not inferred from input-file or dimension caps. |
| `S069-07` | Every required negative case recorded with the guard it actually reaches and accurate `PASS`/`FAIL`/`UNVERIFIED` status. |
| `S069-08` | Completed branch-specific CI at final proof-branch HEAD on Linux and Windows, with truthful failure/non-execution evidence. |
| `S069-09` | Single accurate qualified/blocked/unverified verdict; `R03.C3`, `R03.C`, Stage 03 kept OPEN; no codec/runtime approval or product claims. |
| `S069-10` | Report-only final `main` commit, exact remote readback, green unchanged existing `rust-verify` both hosts, clean `main`, preserved unrelated branch and proper scratch cleanup. |

**Handoff:** Publish the factual report and return exact revision/evidence to the Chief Architect for an independent `ACCEPT / REVISE / STOP` on **this qualification order only**. No next-stage action occurs without the next full Owner-issued work order and matching handoff.
