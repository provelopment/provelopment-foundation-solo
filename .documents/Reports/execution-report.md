# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-061 R01` — implement R03.C1, the exhaustive authored-graphic asset-obligation inventory and its fail-closed nonpublishing boundary.
**Owner handoff:** `SOLO-WO-061 R01` (matching).
**Session date:** 2026-11-11.
**Root:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` — verified single checkout, one worktree, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`.
**Disposition:** **STOPPED.** A material, newly published RustSec advisory (`RUSTSEC-2026-0486`, `unsound`, `rustix 1.1.5`) now fails the mandatory component-security gate on **both** hosts for the unchanged locked graph. Remediation requires `rustix >= 1.1.6`, which is a `Cargo.lock` change this order expressly forbids. The branch was therefore **not** integrated, **R03.C and Stage 03 remain OPEN**, and the Implementation Agent does **not** self-ACCEPT.

## 1. Preflight — verified baseline

| Check | Observed |
| --- | --- |
| `HEAD` == `origin/main` at start | `2dc75f7e8a91891e8a121ef87ccc3eb028526766` ✔ |
| Tree at start | `62dac0ad0e5e2334bd10ae13708df4a05b0493a9` ✔ |
| Branch / worktree / remote | `main`; 1 worktree; 1 remote (`origin`, the accepted URL) |
| Only tracked pre-authorization difference | ` M .documents/WIP/workorder-active.md` — the Owner-installed `SOLO-WO-061 R01` (blob `5b1658f4…`, 31,882 bytes), nothing else staged/modified/untracked ✔ |
| WIP blobs at `HEAD` | ACTIVE `ea5f20b7…` (neutral), NEXT `0e5f4342…` (R03.C candidate), ROADMAP `31a0ba2e…`, OUTLOOK `cb9c7a61…` ✔ |
| `.gitignore` | `f45dd49524748e9b08f6678271edfc22ac39b97f`, `/target/` anchored ✔ |
| Installed order identity | clearly `OWNER-ISSUED — SOLO-WO-061 R01`, matching scope and handoff, **not** an R03.C-wide order ✔ |

Interface confirmation from source (all present, none renamed): `ValidatedPropertyInputV1::owners()`/`declaration_for()`, `PropertyResolutionCandidateV1::input()`, `AuthoredPropertyValueV1::value()`/`exposure()`, `PropertyKey::ALL`/`is_graphic()`, `PropertyValueV1::asset_reference()`, `QualifiedOwnerV1`, `AssetReferenceV1::namespace()`/`reference()`, `Exposure`. No reconciliation or forbidden production edit was needed.

## 2. Authorization revision — published and green on both hosts

| Item | Value |
| --- | --- |
| Commit | `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` |
| Tree | `0b194bef13b4d78cbda99fa5ddb2bcc74b5e3021` |
| Paths | `.documents/WIP/workorder-active.md` (`5b1658f4…`), `.documents/WIP/plan-next.md` (`92fa562b1af0c54391851aa5c79deb1631127069`, exactly the authorized neutral heading) |
| CI | run **`42987963563`** — success on `ubuntu-24.04` and `windows-2025`; 33 steps, 0 failed, 9 / 14 counterpart-host skips; security gate passed with advisory-DB commit `132f5db7633f57369ba099098285b2cb680b1886` fetched `2026-11-11T08:34:33Z` |

ROADMAP and OUTLOOK were untouched by this run.

## 3. Implementation branch — published, gate failed

| Item | Value |
| --- | --- |
| Branch | `solo-wo-061-r01`, created from the authorization SHA in the same checkout |
| Commit | `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Tree | `89c6176a5f159069eccab9eb0d4ed67e88eaba4f` |
| Remote | pushed ordinarily to `origin/solo-wo-061-r01` (same SHA); **no PR created**, no merge, no rebase, no force |
| Paths | exactly the five authorized: `crates/solo-application/src/graphic_asset_inventory.rs` (new, 237 lines), `crates/solo-application/src/lib.rs` (+5 lines: module decl and re-exports), `crates/solo-application/tests/graphic_asset_inventory.rs` (new, 906 lines), `crates/solo-adapters/tests/property_resolution.rs` (+366/−4: import plus four appended tests), `crates/solo-domain/tests/property_resolution.rs` (+355/−0: three appended tests) |
| Unchanged | `Cargo.toml`, `Cargo.lock` (`8bd0bb00286a97853624ebbc0fe18da42a733f12`, identical to `main`), `rust-toolchain.toml`, `.cargo/`, `deny.toml`, `.github/`, `.gitignore`, every domain/application/adapter production file except the two authorized, `platform/`, ROADMAP, OUTLOOK |

Implemented behaviour (`crates/solo-application/src/graphic_asset_inventory.rs`): a pure, nonpublishing `collect_graphic_asset_obligations_v1(&PropertyResolutionCandidateV1) -> Result<GraphicAssetObligationInventoryV1, PropertyResolutionRefusalV1>` over the already whole-graph validated candidate, iterating `input().owners()` (graph/parent-first), then `PropertyKey::ALL` filtered by `is_graphic()`, then each authored `currentContext` index; each immutable obligation exposes declaring owner, role, zero-based local index, authored exposure and the original `AssetReferenceV1` (namespace plus unchanged reference text); `GRAPHIC_ASSET_INVENTORY_GRANTS_PUBLICATION_AUTHORITY = false` with matching accessors; a defensive fail-closed refusal (`InconsistentRecord`, naming only owner and key) for an impossible non-graphic value on a graphic key. No I/O, transport, decoder, catalogue, provider, route, locale or publication path exists in the module.

## 4. Local verification actually performed — and what could not run

| Gate | Result |
| --- | --- |
| `cargo fmt --all` / `--check` | clean; the formatter parsed every changed file, including the adapter test file |
| `cargo check --locked -p solo-domain -p solo-application --all-targets` | **passed** — the new production module and the new application test file compile with all targets |
| Focused tests, workspace all-target tests | **NOT RUN locally** — this Windows host has no MSVC linker (`link.exe` absent, no Visual Studio/Build Tools installation), so no test binary can be linked. `cargo check --locked -p solo-adapters --all-targets` fails at `serde_json`'s build script with `linker link.exe not found`, so the adapter test file could not be compiled locally either. |
| Two-host CI | available only through the workflow, which is where the blocker in §5 was found |

No compiler check is presented as a passing test, and no gate is claimed from a badge alone: the branch run below was read step by step.

## 5. The blocker — `RUSTSEC-2026-0486` against the unchanged locked `rustix 1.1.5`

Branch CI run **`42989368984`** (dispatched at the branch ref, unchanged workflow) **failed on both hosts** at the mandatory component-security gate — step 9 on `ubuntu-24.04`, step 10 on `windows-2025`:

```
error[unsound]: xattr functions mark unwritten bytes as initialized when given a zero-length buffer
  ┌─ Cargo.lock:9:1
  │ rustix 1.1.5 registry+https://github.com/rust-lang/crates.io-index
  ├ ID: RUSTSEC-2026-0486
  ├ Solution: Upgrade to >=1.1.6 (try `cargo update -p rustix`)
  ├ rustix v1.1.5
  │   └── solo-adapters v0.1.0
  │       └── solo-cli v0.1.0
advisories FAILED: 1 errors, 0 warnings, 0 notes
##[error]cargo-deny check advisories failed for the locked workspace graph
```

Independently confirmed advisory metadata (fetched from the RustSec advisory database, 2026-11-11): `id = RUSTSEC-2026-0486`, `package = rustix`, `date = 2026-10-27`, `informational = "unsound"`, `keywords = [xattr, uninitialized-memory]`, `aliases = ["GHSA-jh2m-fhpj-3vr7"]`, affected functions `rustix::fs::{getxattr,lgetxattr,fgetxattr,listxattr,llistxattr,flistxattr}` for `>= 1.0.0, < 1.1.6`, patched `>= 1.1.6`. An independent OSV query for `rustix 1.1.5` at `2026-11-11T15:46:56+07:00` returned the same advisory id; the GHSA alias itself is not yet resolvable in OSV's mirror, so the advisory is very fresh.

**This is inherited, not introduced by R03.C1.**

- `Cargo.lock` is byte-identical on the branch and on `main`: both `8bd0bb00286a97853624ebbc0fe18da42a733f12`. No manifest, lock, toolchain, workflow or policy file was touched by this run.
- The same scanner, the same pinned `cargo-deny 0.20.2` archive (SHA-256 verified) and the same `deny.toml` passed this gate on the authorization revision at `08:34:33Z` (DB commit `132f5db7…`). The branch run fetched the advisory database at `08:44:20Z` and failed. Only the advisory database advanced; the code and the locked graph did not.
- `deny.toml` line 37 sets `unsound = "all"` — a hard failure for any crate in the graph, with no exception list. Line 33 denies yanked versions and line 36 denies unmaintained crates, so the policy has no bypass path that would be legitimate to use.
- Because the security gate runs before the build gates, every later branch step was **skipped** on both hosts: package-set gate, format, strict Clippy, the full locked all-target test suite, architecture real-tree/refusal/tripwire, release build, CLI smoke and the Ubuntu byte-comparison. **No two-host evidence exists for the R03.C1 code itself.**

**Reachability triage (reported, not waived).** `rustix` is a normal dependency of `solo-adapters`, target-gated to Linux, used in `source_read_native.rs` only for `openat2`/`open`/`OFlags`/`Mode`/`ResolveFlags` confinement. The advisory concerns only the `rustix::fs` extended-attribute functions; a repository search finds **zero** `xattr` call sites in Solo source or tests, so no Solo code path exercises the affected functions. That mitigates exploitability; it does not change the policy verdict, because `unsound = "all"` denies the version rather than a call site. This is recorded as an honest triage of a real advisory, not as a reason to treat the gate as passed.

## 6. Why this cannot be repaired inside `SOLO-WO-061 R01`

The only remedies are (a) `cargo update -p rustix` to `1.1.6`+ — a `Cargo.lock` change; (b) a `deny.toml` exception or a narrowed policy — a forbidden policy edit; or (c) an unapproved workaround such as disabling the gate. §3 and §7 of this order explicitly reserve dependency, lockfile and policy changes to a separate Owner order, and require a STOP on "a material security concern or two-host gate fails". No gate was weakened, no warning was silenced, and no test was skipped or `#[ignore]`d to manufacture a green result. The branch and its evidence are preserved for a lawful retry.

## 7. A1 component ledger — fresh review dated 2026-11-11

Registry and advisory queries were made on 2026-11-11 between `15:44` and `15:47` local time (`08:44`–`08:47Z`); the RustSec database revision used by CI is the one the workflow fetched at `08:44:20Z`.

**Locked graph: 22 packages — 4 first-party (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli` 0.1.0) and 18 third-party.**

| Package | Locked | crates.io max stable | Notes |
| --- | --- | --- | --- |
| bitflags | 2.13.2 | 2.13.2 | current |
| errno | 0.3.14 | 0.3.14 | current |
| itoa | 1.0.18 | 1.0.19 | inherited patch lag |
| libc | 0.2.191 | 0.2.191 | current |
| linux-raw-sys | 0.12.1 | 0.12.1 | current |
| memchr | 2.8.3 | 2.8.3 | current |
| proc-macro2 | 1.0.107 | 1.0.107 | current |
| quote | 1.0.47 | 1.0.47 | current |
| rustix | 1.1.5 | **1.1.6** | **RUSTSEC-2026-0486 (`unsound`); the blocker** |
| serde / serde_core / serde_derive | 1.0.229 | 1.0.229 | current |
| serde_json | 1.0.154 | 1.0.154 | current |
| syn | 3.0.9 | 3.0.10 | inherited patch lag |
| unicode-ident | 1.0.27 | 1.0.27 | current |
| windows-link | 0.2.1 | 0.100.0 (new major series) | not a patch lag; required by `windows-sys 0.61`; no advisory |
| windows-sys | 0.61.2 | 0.61.2 | current |
| zmij | 1.0.23 | 1.0.23 | current |

**Toolchain, CI and environment (all unchanged):** Rust `1.99.1` (`1.99.1-x86_64-pc-windows-msvc` locally), `rustfmt`/`clippy` from that pinned toolchain, `cargo-deny 0.20.2` from the pinned verified archive (Linux archive SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; the Windows pin is the workflow's own documented digest), `actions/checkout` pinned to one full commit SHA, GitHub-hosted `ubuntu-24.04` and `windows-2025`, no browser environment in scope.

**Findings.** `rustix 1.1.5` carries `RUSTSEC-2026-0486` (`unsound`, patched `>=1.1.6`), confirmed independently by the RustSec database and by an OSV query for that exact version. No other locked third-party package returned any advisory (OSV per-package queries for all 18 returned none). `itoa 1.0.18`, `syn 3.0.9` and `rustix 1.1.5` are one patch behind; `windows-link 0.2.1` is a distinct major line rather than an out-of-date patch. The inherited `cargo-deny` `license-not-encountered` warning for `Unicode-3.0` at `deny.toml:51` remains **`UNVERIFIED`** and is not repaired, silenced or claimed resolved here; this run's security gate stopped at the advisories error, so licence output is not presented as evidence either way.

## 8. Acceptance matrix status — written, but not verified

Every behavioural row below has corresponding implementation and test code on the retained branch. **None of them is verified**: the security gate failed before the test steps, and no local test binary can be linked on this host. They are reported as `NOT VERIFIED`, never as passed.

| ID | Code written | Status |
| --- | --- | --- |
| R03C1-01 | `r03c1_01_inventory_covers_all_five_levels_in_graph_and_registry_order` (owner-order, registry-order and local-index assertions; nine obligations) | NOT VERIFIED |
| R03C1-02 | `r03c1_02_all_five_roles_…` (application) and `r03c1_07_five_registered_graphic_roles_keep_exact_origin` (domain) | NOT VERIFIED |
| R03C1-03 | `r03c1_03_inventory_is_identical_for_any_requested_target_…` (application) and `r03c1_03_sibling_sites_and_two_locales_inventory_is_target_independent` (real JSON) | NOT VERIFIED |
| R03C1-04 | `r03c1_04_internal_and_descendant_suppressed_sources_stay_inventoried_and_unpublished` plus the unlisted coverage in the adapter tests | NOT VERIFIED |
| R03C1-05 | `r03c1_05_suppress_without_a_local_value_adds_no_obligation` | NOT VERIFIED |
| R03C1-06 | `r03c1_06_same_filename_at_two_owners_and_two_namespaces_stays_distinct` | NOT VERIFIED |
| R03C1-07 | `r03c1_07_graphic_cardinality_duplicates_and_suppression_are_unchanged`, `r03c1_07_graphic_suppression_and_internal_filtering_are_unchanged` (exact refusal kinds) | NOT VERIFIED |
| R03C1-08 | `r03c1_08_real_json_graphic_inventory_covers_all_five_levels`, `r03c1_08_invalid_graphic_on_an_unlisted_representation_refuses_before_inventory` | NOT VERIFIED |
| R03C1-09 | `r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested` | NOT VERIFIED |
| R03C1-10 | `r03c1_10_inventory_is_nonpublishing_repeatable_and_empty_safe` (constant false, repeatability, empty and all-internal sets) | NOT VERIFIED |
| R03C1-11 | No existing test was deleted, loosened, skipped or XFAILed; production changes are limited to the one new module and its re-exports; domain stays std-only and application imports domain/std only | PARTLY VERIFIED (by diff review and `cargo check`), suites NOT RUN |
| R03C1-12 | Two-host gates | **FAILED** — authorization revision green (`42987963563`); branch revision failed on both hosts (`42989368984`) |

## 9. Deferred work and unchanged surfaces

Not implemented by this order and still unavailable: any confined asset-byte reader or media decoder, MIME/magic/dimension/pixel verification, checksum/licence/provenance catalogue, the pinned `platformAssetLibraryVersion: "1.0.0"` `platform/assets/` catalogue and its artwork, responsive derivatives, public activation, and every publication authority. No `platform/` file, artwork, font, catalogue or media byte was created; the accepted R03.A/R03.B behaviour and the nine-key registry are unchanged.

`R03.C` remains the pending Stage 03 package (ROADMAP unchanged, blob `31a0ba2e…`), `plan-outlook.md` is unchanged (`cb9c7a61…`), **R03.C is not complete and Stage 03 remains OPEN**. No automatic platform or role fallback was added, and the binding §6.2I.1 rule was neither reinterpreted nor weakened.

## 10. Workspace, temporary material and cache

| Item | Evidence |
| --- | --- |
| Solo `.temp/` before | 2 files, 289,253,806 bytes — the retained `architecture-fixtures/` and `toolchain-installer/` archives |
| This run's scratch | `.temp/draft-graphic_asset_inventory.rs`, `.temp/draft-app-tests.rs`, `.temp/s1.md`–`.temp/s3.md` (composition parts), all task-owned |
| Solo `.temp/` after | 2 files, 289,253,806 bytes — identical to the inherited baseline, archives preserved under their existing trigger |
| Live root `target/` | present, still ignored by `.gitignore:24` `/target/`, untouched; no cleanup, no `.gitignore` or local-exclude edit |
| Cline task temp area | no file left behind |
| Secondary checkouts | none; one checkout, one worktree, no nested repository |

An empty `git status` is not offered as proof of scratch cleanup; the byte counts above are the evidence.

## 11. Final Git state

| Item | Value |
| --- | --- |
| `main` | `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` (authorization commit only) plus this report commit |
| `origin/main` | matches `main` after the report push |
| Branch retained | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6` locally and on `origin`; **not** integrated, **not** pruned (retention is authorized until a separate closure) |
| Working tree | clean (`git status --porcelain` and `--untracked-files=all` both empty) |
| WIP slots | exactly four: ACTIVE = frozen issued order `5b1658f4…`, NEXT `92fa562b…` (neutral), ROADMAP `31a0ba2e…`, OUTLOOK `cb9c7a61…` |
| Product, dependency or asset changes on `main` | none |

## 12. Disposition

**STOPPED for `SOLO-WO-061 R01`.** The bounded R03.C1 implementation and its acceptance tests exist and are published on the retained branch, but the mandatory two-host verification cannot pass while the locked `rustix 1.1.5` matches `RUSTSEC-2026-0486`, and every lawful remedy is a dependency, lockfile or policy change that this order forbids.

**Smallest Owner decision needed (one):** issue a bounded dependency-policy order authorizing `cargo update -p rustix` to `1.1.6` (and, if wanted in the same bounded order, `itoa` → `1.0.19` and `syn` → `3.0.10`), with fresh A1 re-verification and no other change; then re-run this branch's two-host gate. No R03.C1 code change is expected for that retry.

The Implementation Agent does not self-ACCEPT, does not clear the NEXT candidate, does not integrate the branch and does not begin further R03.C work. Stage 03 remains OPEN.
