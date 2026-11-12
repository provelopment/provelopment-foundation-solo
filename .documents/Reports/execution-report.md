# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-067 R01` — R03.C3 bounded Entity image-media verification.
**Matching Owner→Implementation Agent handoff:** `SOLO-WO-067 R01`.
**Session date (ISO):** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo`; one worktree, sole `origin`.
**Disposition: `STOPPED`.** The mandatory §4 decoder/security qualification **failed** for the AVIF path, and two further independent policy blockers were measured for the remaining scope. Per §4 and §110 this order forbids a header-only pass, a skipped format, a substitute dependency, a workflow tool addition or any policy relaxation, so **no technical branch was created, no product path was modified and no dependency was added**. The only published change is this report. `R03.C3` is **not implemented**, `R03.C` and Stage 03 remain **OPEN**, and this report is **not** a self-ACCEPT.

## 1. Baseline and issuance (verified)

| Anchor | Verified value |
| --- | --- |
| Accepted `main` / `origin/main` / HEAD | `ac9a7ad1fd3c1514546948e2e01ccba3e3e39f94` |
| Tree / parent | `81e73f73c6fe007cd5cbe1d28f5c425e607f24cc` / `c28f30a42e6529ac5742a5ef7c242eeb8f117ad6` |
| Canonical report (accepted `SOLO-WO-066 R01`) | `057751003035592888f41ad1699e3f552f3dbf90` |
| WIP baseline blobs | ROADMAP `9d7a8622…`, NEXT `aef9b6cf…` (one unissued `R03.C3`), ACTIVE `ea5f20b7…` (neutral), OUTLOOK `cb9c7a61…` |
| Branch inventory | exactly `main` and the divergent retained `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6`; no R03.C3 branch existed |
| Dirty state | only the Owner-installed complete `SOLO-WO-067 R01` ACTIVE (`6acea804ec3124979d477e303c2169e43eb787a9`, 33,741 bytes, 132 lines); handoff ID matched |
| Checkouts / worktrees / remotes | 1 / 1 / 1 — no second clone, worktree, nested repository or sequencer state |

**Issuance commit `93e91982fa79f9cefb6527df729d659581525c5a`** — tree `a84e265717777f338bf8b113fbf42e3af59179e2`, parent `ac9a7ad1…`; exactly two changed paths: `.documents/WIP/workorder-active.md` → `6acea804…` (order frozen) and `.documents/WIP/plan-next.md` → `92fa562b1af0c54391851aa5c79deb1631127069` (neutral one-line heading plus LF). ROADMAP `9d7a8622…` and OUTLOOK `cb9c7a61…` stayed byte-identical. Remote readback confirmed commit/tree/parent/blobs.

**Issuance CI `43141229695` — `success`** at `93e91982…`: Ubuntu job `130928437246` (33 steps, 9 skips, **407 passed**), Windows job `130928436877` (33 steps, 14 skips, **412 passed**), 0 failed / 0 ignored, component gate `security_gate=pass`. Inherited `warning[license-not-encountered]` (`Unicode-3.0`) still emitted on both hosts and remains **UNVERIFIED**.

## 2. §4 qualification — measured results, exact versions

Qualification used two disposable probe manifests in the permitted Solo `.temp/` scratch (a full-scope two-root graph and a PNG/JPEG/WebP-only graph), each resolved by Cargo against crates.io, measured, and then deleted. No tracked manifest, lock file or policy entry was modified; the accepted workspace still resolves exactly as at baseline.

| Root | Exact version / licence / provenance | Posture |
| --- | --- | --- |
| `image` | `=0.25.10`, `MIT OR Apache-2.0`, published 2026-03-10, not yanked, checksum `85ab80394333c02fe689eaf900ab500fbd0c2213da414687ebf995a65d5a6104`, MSRV 1.88.0 (project toolchain 1.99.1) | Pure Rust, no build script; `png`/`jpeg`/`webp` features exist and the prohibited `avif`, `avif-native`, `nasm`, `ravif`, `dav1d` and default-format features were excluded as ordered. Blocked in practice by the duplicate-version defect in §3. |
| `avif-decode` | `=3.0.0`, `BSD-3-Clause`, published 2026-09-20, not yanked, checksum `1eee11893966d77d3e18c66fdcfb1c30e2d8df050346609c775ce8eec3390344`, MSRV 1.98 | **Not acceptable:** it pulls `rav1d` with default features on x86/x86_64 (§2.1) and a required transitive is `MPL-2.0` (§2.2). |
| `rav1d` (pulled by the permitted AVIF root) | `1.1.0`, `BSD-2-Clause`, published 2025-05-07, not yanked, checksum `1932f060d5e7bd49dc9f8b272c1dc5e9ce0ffe141c28be900265d3989b36c9ed` | **Forbidden build surface:** default features `asm`, `asm_arm64_dotprod`, `asm_arm64_i8mm`, `asm_arm64_sve2`, `bitdepth_8`, `bitdepth_16`; build-dependencies `cc ^1.0.79` and `nasm-rs ^0.3` (feature `parallel`); the crate ships **48 `.asm` files / 155,378 assembly lines**, and its `build.rs` assembles them under `#[cfg(feature = "asm")]`. |
| Transitives of note | `avif-parse 2.1.0` = **`MPL-2.0`**; `imgref` `CC0-1.0 OR Apache-2.0`; `to_method` `CC0-1.0`; `byteorder` `Unlicense OR MIT`; `moxcms`, `pxfm`, `yuv` `BSD-3-Clause OR Apache-2.0`; `zlib-rs` `Zlib`; `adler2` `0BSD OR MIT OR Apache-2.0`; `r-efi` `MIT OR Apache-2.0 OR LGPL-2.1-or-later` (MIT/Apache selectable) | New licence families relative to `deny.toml`'s `allow = [Apache-2.0, MIT, Unlicense, Unicode-3.0]`. |

### 2.1 AVIF requires a native assembler (measured)

`cargo tree -e features -i rav1d` in the full-scope probe resolved:

```text
rav1d v1.1.0
└── rav1d feature "asm"
    └── rav1d feature "asm_arm64_dotprod"
        └── rav1d feature "default"
            └── avif-decode v3.0.0
```

`avif-decode 3.0.0` declares `rav1d` with `default_features = true` for `cfg(any(target_arch = "x86", target_arch = "x86_64"))` — the exact architecture of **both** accepted CI hosts — and sets `default-features = false` plus `bitdepth_8`/`bitdepth_16` only for non-x86 targets. Because Cargo features are additive, `default-features = false` on `avif-decode` **cannot** disable `rav1d`'s `asm`; the only levers would be a direct `rav1d` root (unledgered, §3) or a `[patch]` (unledgered and a policy change). Either way the graph needs `nasm` plus `cc` at build time on both hosts, and the workflow may be changed only for exact `edges=…` counts (§3 #13), never to install a tool.

### 2.2 AVIF introduces a non-permissive licence (measured)

`avif-parse 2.1.0`, a non-optional normal dependency of the permitted AVIF root, is **`MPL-2.0`**. §3 #14 permits adding only "specifically audited **permissive** SPDX licences actually introduced in the lock graph", and §4 item 2 makes any licence needing a rights decision or a new distribution obligation a STOP. MPL-2.0 file-level source-availability obligations are exactly such a decision, so no `allow` entry could lawfully be authored under this order.

### 2.3 Whole-scope graph size and licence growth (measured)

The full two-root probe resolved **66 packages** against the accepted **18-crate** closure, introducing at least `MPL-2.0`, `BSD-2-Clause`, `BSD-3-Clause`, `CC0-1.0`, `0BSD`, `Zlib` and `Unlicense` expressions, `cc`/`nasm-rs`/`jobserver`/`find-msvc-tools` build tooling, and a second `miniz_oxide` version.

## 3. Third independent blocker — duplicate locked versions (measured)

The PNG path alone breaks `[bans] multiple-versions = "deny"`. The PNG/JPEG/WebP-only probe (`image =0.25.10`, defaults off, `png`/`jpeg`/`webp`) resolved **23 packages** and still locked **two `miniz_oxide` versions**:

```text
miniz_oxide v0.8.9 └── png v0.18.1 └── image v0.25.10
miniz_oxide v0.9.1 └── flate2 v1.1.11 └── png v0.18.1 └── image v0.25.10
```

`png 0.18.1` itself requires both, so the duplication cannot be resolved by unification, and `cargo tree -i miniz_oxide` refuses the query as ambiguous. The only ways to pass would be a `[bans].skip`/`skip-tree` entry or an `allow`, each explicitly forbidden ("no `skip`, no `skip-tree`", §4 item 2; no weakening, §7). §4 item 3 requires "no multiple versions", so this holds for the full scope **and** for any narrowed PNG subset.

## 4. A1 supply-chain posture recorded (partial, honestly labelled)

The current RustSec advisory database (`rustsec/advisory-db`, last push **2026-11-11T12:22:49Z**) was probed per package: `rav1d`, `avif-decode`, `avif-parse`, `nasm-rs`, `moxcms`, `miniz_oxide` and `pxfm` have **no advisory directory**; `image`, `zerocopy` and `zlib-rs` **do**, and their exact version-range applicability to the candidate graph was **not** evaluated. Both direct roots and `rav1d` are not yanked, with the published checksums above and maintainer identities matching their GitHub accounts (`image`: Aurelia Molzer; `avif-decode`: Kornel; `rav1d`: Per Larsen / memorysafety).

Because no candidate graph was ever committed, **no two-host `cargo-deny` run was performed for it** and no A1 clearance is claimed: the licence and duplicate-version defects already fail the gate regardless of advisory status.

## 5. What was and was not changed

**Changed by this order (published):** `.documents/WIP/workorder-active.md` (Owner-installed order, frozen) and `.documents/WIP/plan-next.md` (neutralized) in the issuance commit; then this report alone.

**Unchanged and verified at the current HEAD:** ROADMAP `9d7a8622…`, OUTLOOK `cb9c7a61…`; `AGENTS.md`; About; Architecture; all SOPs; every Rust source, test, fixture and example; `Cargo.toml`, all crate manifests and `Cargo.lock` `2f9e5357…`; `deny.toml` `3063e864…`; `.github/workflows/rust-verify.yml` `eb78dad2…`; `rust-toolchain.toml` `9ec55b3b…`; `.gitignore`; `platform/`; the accepted `graphic_asset_read.rs` `6499421d…`, `graphic_asset_inventory.rs` `7de9bc73…`, `source_read.rs` `fb6f005a…` and `source_read_native.rs` `5527b2b4…`. No file was added, deleted or renamed; no fixture was created; four-WIP purity holds and no report archive exists.

**Product state:** `R03.C3` has **no** port, **no** adapter, **no** dependency, **no** tests and **no** verified-media evidence. The accepted acquisition layer still reports `media_verified = false` and remains exactly as accepted. Nothing here establishes image validity, dimensions, MIME agreement, Platform capability, publication authority, release or deployment.

## 6. Acceptance matrix — truthful statuses

| Gate | Status | Basis |
| --- | --- | --- |
| `S067-01` | **PASS** | §1: accepted `ac9a7ad1…`/`81e73f73…`/report `05775100…`, four baseline WIP blobs, sole checkout/worktree/origin, only the installed ACTIVE dirty, matching handoff, `solo-wo-061-r01` preserved and no other branch. |
| `S067-02` | **PASS** | §1: issuance changed only ACTIVE + neutralized NEXT, no code or dependency, and full two-host CI was green at `93e91982…` before any trial. |
| `S067-03` | **NOT EXERCISED (compliant)** | `solo-wo-067-r01` was deliberately **not** created: §4 requires the static qualification to pass first, and it did not. The §3 ledger shows zero product-path operations, no extra fixture and no collateral change. |
| `S067-04` | **FAIL** | No complete real decoder proof is possible within scope: AVIF requires an assembler-built `rav1d`, and no format could be added under the policy as it stands. No header-only or first-frame substitute was claimed. |
| `S067-05` | **UNVERIFIED** | Fail-closed boundaries were not implemented because no inspection path could be legitimately constructed. |
| `S067-06` | **UNVERIFIED** | Bound enforcement (20 MiB, 1–8192, ≤32,000,000 pixels, bounded working set) is unimplemented; no allocation budget could be certified for `rav1d`. |
| `S067-07` | **UNVERIFIED** | Whole-inventory semantics, refusal attribution and privacy-safe diagnostics are unimplemented. |
| `S067-08` | **FAIL** | The candidate graphs violate the exact-closure and `multiple-versions = "deny"` rules (§2.3, §3) and would need an unledgered `rav1d` root or a forbidden skip; the checkers were correctly left untouched rather than weakened. |
| `S067-09` | **FAIL / UNVERIFIED** | No two-host `cargo-deny` on a candidate graph; `MPL-2.0` needs a rights decision; `image`/`zerocopy`/`zlib-rs` advisories unevaluated. Inherited `Unicode-3.0` warning remains `UNVERIFIED`. |
| `S067-10` | **PASS for existing regressions only** | All accepted R01–R03.C2 gates and the mapped-view/immutable-snapshot proofs are green at the issuance SHA (407 Ubuntu / 412 Windows). C3-specific media proof does not exist; user-visible rendering/HTTP/browser checks are genuinely **NOT APPLICABLE** because no public use is authorized. |
| `S067-11` | **NOT REACHED** | No branch and no integration; a fast-forward was never considered because mandatory gates were unproven. |
| `S067-12` | **PARTIAL — report gate only** | This single report-only commit is published with two-host CI at its exact HEAD; the product gates it would attest remain unmet, so the order's completion condition is not satisfied. |

## 7. Owner decisions required to continue

1. **AVIF component choice.** The order's only permitted AVIF root needs NASM plus a C toolchain at build time on both hosts and drags in `MPL-2.0`. Continuing needs a new Owner-issued order naming a genuinely pure-Rust AVIF decoder that is permissively licensed, needs no assembler/FFI, and can bound its whole decode path — with licence, unsafe-surface and resource evidence. `zenavif` (AGPL-3.0/commercial) remains unacceptable for this Apache-2.0 product, and `image`'s `avif`/`avif-native` paths still require `ravif`/`dav1d`.
2. **Duplicate-version policy.** `png 0.18.1` forces two `miniz_oxide` versions, failing `[bans] multiple-versions = "deny"`. Continuing PNG needs an explicit Owner decision on a narrowly scoped, documented skip (a policy change this order forbids) **or** a decoder set that avoids the duplication.
3. **Scope decision.** A PNG/JPEG/WebP-only increment is still blocked by (2) and would be a deliberately partial media claim, which §110 bars under this order; it would need its own bounded order with the residual risk stated.
4. **Licence approvals.** Any path forward must add, with audit, the permissive expressions it actually introduces (`BSD-2-Clause`, `BSD-3-Clause`, `CC0-1.0`, `0BSD`, `Zlib`, `Unlicense`) and separately decide any non-permissive expression such as `MPL-2.0`.

## 8. Workspace, preservation and handoff

One checkout, one worktree, one `origin`; `main == origin/main`, empty `git status --porcelain --untracked-files=all`; branches exactly `main` and preserved divergent `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6` on both local and GitHub; WIP exactly four files; ignored `target/` untouched; inherited Solo `.temp/` archives preserved at **2 files / 289,253,806 bytes**. Both disposable probe manifests and their locks were deleted after measurement; no unknown or Owner-owned material was touched, and no `git clean`, reset, force, amend, rebase, cherry-pick, PR, release, deployment or provider action occurred.

**Handoff:** `SOLO-WO-067 R01 — STOPPED`, for the reasons in §§2–3 and 7. Implementation was **not** started and **no** media verification is claimed; `R03.C`, Stage 03 and all remaining packages stay **OPEN**. The Chief Architect should independently audit this report's exact remote HEAD and return `ACCEPT`, `REVISE` or `STOP` on the administrative disposition; any resumption requires a new Owner-issued order covering the decisions in §7.
