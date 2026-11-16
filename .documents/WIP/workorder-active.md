# SOLO-WO-118 R23 — IMPLEMENT THE PINNED UNICODE 18 / ICU4X LOCALE AND ROUTE-KEY VALIDATION BOUNDARY (R04.A1 / R04.B1)

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Issued:** 2026-11-16 (Asia/Jakarta)  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner's sole checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Existing technical branch:** `solo-wo-118-avif-v4-r01`  
**Canonical work-order file:** `.documents/WIP/workorder-active.md`  
**Bounded outcome:** `R04AB1-UNICODE18-ICU4X-LOCALE-ROUTE-LITERAL-VALIDATION-N134-TWO-HOST-NONPUBLISHING-V1`.

## 1. Chief Master Architect disposition and starting authority

**SOLO-WO-118 R22: ACCEPT for the bounded `R04.C3` manifest-bound, nonpublishing route/visibility policy preflight.** I independently inspected the canonical R30 report, the T30/R30 commits and changed-path ledger, real Ubuntu/Windows N133 execution output, both authentic-versus-mutated gate verdicts, the independent AVIF oracle and the fresh two-host R30 main baseline. This accepts only the governed R22 policy boundary, not complete routing, locale conformance, publication, asset delivery, Stage 03 closure, merge, release or deployment.

| Item | Exact accepted baseline |
|---|---|
| `origin/main` R30 | `9f43ffe28250858e3347e9e2083d40f762a8dd31` |
| `origin/solo-wo-118-avif-v4-r01` T30 | `b3cbdeb589423b222bd8db9e683005f9b824b2a2` |
| A30 ACTIVE-only commit | `ca743ec14ad42700bd620714aa3150aa4cadd62c` |
| T30.0 ACTIVE-only commit | `56906f8b88cb1e23bed5ae3ab251eca0c346e961` |
| T30 technical workflow | `43885719190`, attempt 1; Ubuntu job `133363180568`, Windows job `133363180876`, independent AVIF oracle job `133363181138`, all `success` |
| Main R30 workflow | `43887645275`; Ubuntu job `133369999583`, Windows job `133369999914`, both `success` |
| N133 | Four real tests on each host; `n133_gate authentic=accepted` and `n133_gate mutation=rejected` on each |
| Technical architecture | `packages=4 targets=39 files=81 edges=93` |
| Platform library | Exactly 81 Owner-approved MIT Tabler SVGs and catalogue/notice/provenance (84 files), catalogue SHA-256 `ad8ea718b50641206cc4f4fe13e866ad266366a7aa11ef799c83ae8187858fc7` |

A harmless `cargo-deny` **license-not-encountered** warning for the unused `Unicode-3.0` allowance exists on the main-only locked graph; do not mask it or turn a green exit into a claim that no warning occurred. Capture actual warning dispositions. The technical branch had no R22 scope-induced security warning.

**Architectural authorities:** `AGENTS.md`; `.documents/About/FOUNDATION-SOLO.md`; `.documents/Architecture/ARCHITECTURE.md` §§3.2B, 6.2B, 6.2J.2–6, 6A.2 and 6A.7; Architect SOP §6B/§7B; Implementation SOP §5B/§5C/§7A. **Pinned implementation references:** [ICU4X 2.4 release](https://github.com/unicode-org/icu4x/releases), [NFC API](https://docs.rs/icu_normalizer/2.4.0/icu_normalizer/struct.ComposingNormalizerBorrowed.html), [case mapper](https://docs.rs/icu_casemap/2.4.0/icu_casemap/struct.CaseMapperBorrowed.html), [locale canonicalizer](https://docs.rs/icu_locale/2.4.0/icu_locale/struct.LocaleCanonicalizer.html), [general-category API](https://docs.rs/icu_properties/2.4.0/icu_properties/props/enum.GeneralCategory.html), and [Unicode 18](https://www.unicode.org/versions/Unicode18.0.0/). Current code inputs: `crates/solo-adapters/src/publication_manifest.rs` (sole accepted strict parser), `manifest_visibility_bridge.rs` (R21 validated membership and still-restricted authored locale profile), `manifest_route_visibility_policy.rs` (R22 effective visibility and claim intent). Preserve them unchanged in this order.

## 2. Implement exactly this bounded capability

**Objective:** Install and exercise the already Owner-selected ICU4X **2.4.0** data and APIs for **Unicode 18 / CLDR 49**, proving a reusable **pure, nonpublishing** validator of (a) authored, region-qualified full-locale identity and (b) every declared canonical representation path, public alias and locale-less Page entry-path in an R21 manifest snapshot. Bind it to the **same manifest's** R22 policy. The output is a validated **route-literal/key preflight**, **not** the complete R04.A route-claim collision ledger, IDNA/base URL validator, source/content validator, HTTP router, publication authority or release.

This task removes the existing use of a merely shape-checked locale **at this new boundary** and replaces guessed ASCII/standard-library Unicode behavior with the selected pinned data. The accepted R21 bridge remains intentionally conservative, and its current permitted authored locale subset is not widened by this order. Unsupported valid BCP47 forms are explicitly refused by the first physical profile; never claim that BCP47 itself forbids them. The full later request-language preference pipeline, IDNA host validation, whole-route collision ledger, complete Unicode 18 official-corpus proof and serving remain separate follow-on work.

### 2.1 Install exactly the selected Unicode dependencies

In `crates/solo-adapters/Cargo.toml` add only the following direct normal dependencies and regenerate **one** tracked `Cargo.lock` using the pinned Rust toolchain:

```toml
icu_normalizer = { version = "=2.4.0", default-features = false, features = ["compiled_data"] }
icu_casemap = { version = "=2.4.0", default-features = false, features = ["compiled_data"] }
icu_locale = { version = "=2.4.0", default-features = false, features = ["compiled_data"] }
icu_locale_core = { version = "=2.4.0", default-features = false }
icu_properties = { version = "=2.4.0", default-features = false, features = ["compiled_data"] }
```

Check the exact published feature names and compatibility against the pinned source before saving the lock. `icu_locale_core`'s own feature set may require a minimal explicitly justified feature for the used API; do not add a sixth package family, wildcard version, `icu` umbrella crate, a platform ICU library, a second Unicode version or any unapproved fallback. Confirm `cargo tree -p solo-adapters --locked` contains one coherent ICU4X 2.4 family and all Unicode data comes from the tracked dependency graph. Run the A1 freshness, advisories, provenance and license assessment for all new direct/transitive normal/build/dev dependencies; capture actual resolved versions and relevant warnings. Do not change `deny.toml` to make a failure disappear. A blocked advisory/license/source result is `STOPPED`, never a passed implementation.

### 2.2 New adapter module and API: exact source binding, no I/O

Create `crates/solo-adapters/src/manifest_unicode_route_literals.rs` as **one** new safe module. Add only minimal public exports in `crates/solo-adapters/src/lib.rs`. The module must have no filesystem, network, HTTP, process, clock, environment, source-reader, graphics-worker or provider dependency. No new crate, framework, transport, global registry, unsafe block or separate manifest parser.

Provide exactly this closed public entrypoint and result/refusal type names; use private helpers for implementation details:

```rust
pub fn validate_manifest_unicode_route_literals_v1(
    snapshot: &ManifestVisibilitySnapshotV1,
) -> Result<ValidatedManifestRouteLiteralsV1, RouteLiteralRefusalV1>;
```

**Only the accepted R21 snapshot is input.** Internally call `build_manifest_route_visibility_policy_v1(snapshot)` once to obtain R22 policy (map a failure to a closed refusal preserving safe location). Walk `snapshot.parsed().entity()` in authored containment order, require every representation and Page policy to correspond to the same fully qualified EFS owner, and require complete representation/Page cardinality. The result is owned, immutable, carries the exact source `manifest_sha256`, and cannot be assembled publicly from arbitrary candidate records. A failure discards the whole result, never keeps earlier acceptable rows. No user-controlled path, label, source candidate, location or property value is placed in a diagnostic.

Represent each validated canonical representation claim, alias claim and Page entry claim as an internal typed record with: `QualifiedOwnerV1`, closed `Canonical | Alias | PageEntry` kind, closed effective `PublicInsensitive | PublicSensitive | ExactUnlisted` comparison intent as applicable, authored path's **validated NFC** spelling, canonical spelling for a public-insensitive claim, and an explicit compare key. A Page entry is **preference-dependent and not itself a representation**. Keep each record's declaration ordinal or another deterministic authored-order key and a safe structural field location. Do not export these records to a public/browser projection, generate an URL, pretend an alias is published, or use raw authored strings in public diagnostics. This typed in-memory structure is a future input to the complete collision ledger, not an activated route set.

### 2.3 Unicode 18 route-token implementation — do not substitute Rust std or ASCII

Use compiled ICU4X 2.4 data to implement the **three different** normalizations demanded by Architecture §6.2J.4:

1. **Authored/exact spelling:** NFC. Sensitive public claims and unlisted claims compare on this exact NFC key, retaining authored capitalization.
2. **Insensitive compare key:** NFC → full **default** Unicode case folding → NFC. Not simple lowercase, not Turkish/Turkic folding and not a language-specific lower operation.
3. **Public-insensitive canonical Page spelling:** NFC → full **root/default** Unicode lowercase mapping → NFC. This differs from the fold key (`Straße` and `STRASSE` have equal full-fold keys but are not the same lowercased display spelling).

Executable API guidance, verified against the ICU4X 2.4 published interfaces (adapt lifetime/borrowing details to the compiler):

```rust
use icu_casemap::CaseMapperBorrowed;
use icu_locale_core::LanguageIdentifier;
use icu_normalizer::ComposingNormalizerBorrowed;

let nfc = ComposingNormalizerBorrowed::new_nfc();
let casemap = CaseMapperBorrowed::new();
let root = LanguageIdentifier::default(); // invariant/root language, never a visitor locale
let exact_nfc = nfc.normalize(authored).to_string();
let caseless_key = nfc.normalize(&casemap.fold_string(&exact_nfc)).to_string();
let public_insensitive_spelling = nfc
    .normalize(&casemap.lowercase_to_string(&exact_nfc, &root))
    .to_string();
```

Validate the **whole** authored path before deriving keys. Every representation or public alias path begins with exactly one `/`, then the **lowercase complete authored locale tag**, followed by **1–8 nonempty Page-route segments**; no trailing `/`, doubled separator or alternate/missing locale. Page entries have no locale segment, 1–8 Page-route segments, and never claim a reserved full-locale first segment. Page-route segments allow Unicode 18 letters, marks and decimal digits plus **internal** `-` / `_`; reject invalid general categories, controls, bidi format/control characters, null, `%` escapes in authored logical paths, encoded delimiters, backslash, dot/dot-dot, empty segments, leading/trailing hyphen or underscore and all ambiguous interpretations. Each Page segment is at most **60 Unicode scalars**, the complete Page token portion at most **160 scalars**; count scalar values with `.chars().count()`, not bytes. Reject malformed or double-encoded URI material rather than decoding it here. Keep URL request percent-decoding outside this source-declaration validator for a later authorized request boundary.

Use Unicode 18 **property data** for validation, not `char::is_alphabetic()` or `char::is_alphanumeric()` as the authority:

```rust
use icu_properties::{CodePointMapData, props::GeneralCategory};
let category = CodePointMapData::<GeneralCategory>::new().get(ch);
let allowed = matches!(category,
    GeneralCategory::UppercaseLetter
    | GeneralCategory::LowercaseLetter
    | GeneralCategory::TitlecaseLetter
    | GeneralCategory::ModifierLetter
    | GeneralCategory::OtherLetter
    | GeneralCategory::NonspacingMark
    | GeneralCategory::SpacingMark
    | GeneralCategory::EnclosingMark
    | GeneralCategory::DecimalNumber
);
```

Allow `-` and `_` separately only at internal token positions. Other category values, including `Format`, fail closed. Do not alter the accepted EFS **technical identifier** grammar or the existing source-locator grammar. Do not guess equivalence for confusable-looking strings: this stage may flag a bounded fixed diagnostic for known dangerous forms, but **full UTS #39 v18 confusable tables and official-corpus verification remain an explicit future gate**. Do not claim them complete here.

### 2.4 Full authored locale keys at the new validation boundary

For each Site `defaultLocale` and each representation locale, use ICU4X 2.4's real parser and alias canonicalizer; require an explicit **region** (`US`, `DE`, `ID`, `CN`, `001`, etc.), optional script, lowercased language, title-cased script and uppercase region. The accepted first-release **authored/public identity profile** contains only language + optional script + region; no inferred region, variant, private-use or extension key. Preserve and test that valid BCP47 tags outside this authored profile are *unsupported by the product profile*, not invalid BCP47.

Guidance for the selected API:

```rust
use icu_locale::{Locale, LocaleCanonicalizer};
let mut parsed: Locale = candidate.parse().map_err(|_| LocaleRefusal::Syntax)?;
LocaleCanonicalizer::new_extended().canonicalize(&mut parsed);
// Inspect parsed.id.language, parsed.id.script, parsed.id.region,
// parsed.id.variants and parsed.extensions, then require exact
// canonical authored serialization and the mandatory explicit region.
// Never call likely-subtag maximize() to invent a missing region.
```

Do **not** normalize a noncanonical authored locale into an apparently authored canonical member: `de-de`, `EN-001`, deprecated language aliases and bare `en` must be closed refusals for authored locale identity. The path locale portion must be exactly the **lowercase full path spelling of the already canonical authored key**; this does not lowercase the sensitive Page portion. For comparison/request handling later, a separate function may return a **non-authoritative canonicalized request-tag candidate**, but it may not select a representation, infer a region or grant a redirect in this order. Authoring identity and request-language negotiation are separate operations.

Reject all duplicate normalized locale identities, unexpected membership, unsupported-extension/variant forms, or an absence of Site/representation coverage. Keep site default membership and mapping semantics for a later `R04.B` task; do not invent a representation because `defaultLocale` is declared. R21's current conservative locale bridge is unchanged and remains the source snapshot gate.

### 2.5 Alias and Page-entry binding

Every canonical path uses the matching R22 representation's effective visibility/intent. Every **alias** belongs to exactly its containing **public** representation and is validated with the same locale and Unicode grammar; its own authored `caseMatching`, when present, determines its intended comparison mode, otherwise the public default is `insensitive` according to the accepted per-claim rule. Unlisted aliasing remains refused by R22. Every `entryPaths` item belongs to exactly its Page, requires that Page's own `public_representation_count > 0` (already R22) and stays a locale-less preference-dependent entry claim, not a guessed target. Use the exact owning Facet/Site/Page; two sibling `home` Page IDs are not interchangeable.

**This order validates individual literals and computes candidate keys only.** It does not accept or refuse *cross-claim collisions* as a complete route ledger; preserve all distinct claims in authored order even when their keys overlap. The next route-ledger work will perform complete whole-graph canonical/alias/entry/base collision refusal using these exact keys. No first-match-wins, route priority or implicit deduplication is permitted now.

### 2.6 Closed refusals and nonauthority

Refusal families: invalid/noncanonical Site/representation locale, locale/path mismatch, invalid route segment/category/length, unsafe URI encoding, unlisted-policy inconsistency, wrong/missing EFS policy membership, invalid alias, invalid Page entry, overflow, and a wrapped R22 policy failure. Report a **fixed ASCII** code and structural location like `entity.facets[0].sites[1].pages[0].representations[2].route.path`; no authored route string or private value in error text. Do not panic, truncate, partially build, silently normalize into acceptance or select an alternate manifest. No second parser, no `serde_json::Value` bypass.

Every result must expose constant `false` authority claims: `publication=false`, `public_route_authority=false`, `http=false`, `derived_output=false`, `release=false`, `deployment=false`. The output is never called `Published`, `ReadyToServe`, `ValidatedManifest` or a complete `RouteLedger`.

## 3. Exact allowed-path ledger

Only these paths may change in the described transaction:

| Path | Authorization |
|---|---|
| `.documents/WIP/workorder-active.md` | **A31** on main and byte-identical **T31.0** on existing technical branch only; no other edits |
| `crates/solo-adapters/Cargo.toml` | add the exact five ICU4X 2.4 dependencies only |
| `Cargo.lock` | single `cargo`-generated update resolving those dependencies; no manual edits |
| `crates/solo-adapters/src/manifest_unicode_route_literals.rs` | **NEW** pure Unicode-18/locale/route-literal validator, typed keys, closed failures |
| `crates/solo-adapters/src/lib.rs` | minimal additive module and public re-exports |
| `crates/solo-adapters/tests/publication_manifest.rs` | **additive** `mod n134_manifest_unicode_route_literals` with precisely five focused N134 tests; preserve all prior tests |
| `.github/workflows/rust-verify.yml` | additive native Ubuntu/Windows N134 gate steps after N133; do not alter prior checks |
| `.documents/Architecture/ARCHITECTURE.md` | one concise factual implementation status note near §6A.7/§3.2B; record remaining full corpus, full collision ledger, IDNA/HTTP, no false proof claims |
| `.documents/Reports/execution-report.md` | R31 on main only, after technical proof; overwrite the one report |

**Strictly outside scope:** `crates/solo-domain/**`, `crates/solo-application/**`, `crates/solo-cli/**`, `publication_manifest.rs`, `manifest_visibility_bridge.rs`, `manifest_route_visibility_policy.rs`, all media/source/file-system/decoder modules, platform assets and licence files, the 81 approved icons, the 20 unapproved sources, all `.documents/WIP/plan-*`, `deny.toml`, `.gitattributes`, other CI workflows, new branches/worktrees/PRs, package major-version upgrades, IDNA crate installation, HTTP handler, output publication, release or deployment. If an exact required behavior cannot be implemented safely inside this ledger, STOP and report the precise blocker rather than trespassing into an unapproved path.

## 4. Mandatory transaction A31 → T31.0 → T31 → R31

1. **Preflight, READ ONLY.** Verify physical checkout root, `git rev-parse --show-toplevel`, `main`/technical remote heads at the exact starting SHAs in §1, clean index and working tree, empty stash, exactly one worktree and one `origin` pointing at `provelopment/provelopment-foundation-solo`, exactly four WIP files, expected ACTIVE bytes, current remote head inventory and preservation of Owner-owned `.temp/`. Do not create another clone/worktree. If any drift is unexplained, STOP and report.
2. **A31 — main authority installation.** On `main`, commit **only** the newly Owner-installed `.documents/WIP/workorder-active.md`, push, verify the same byte-identical blob in worktree/index/commit/`origin/main`. No implementation source on main.
3. **T31.0 — technical authority mirror.** Resume the existing `solo-wo-118-avif-v4-r01` branch from T30 `b3cbdeb5`, install the **identical** ACTIVE blob and commit **only** that file. Verify identical A31/T31.0 ACTIVE SHA and no unrelated diff.
4. **T31 — technical implementation.** Change only §3 source/dependency/test/workflow/architecture files; use bounded, explainable commits. First prove behavior **locally** in Windows 11 PowerShell and/or WSL Ubuntu 26.04, using the pinned toolchain. Run focused new tests, correct defects, then entire target and all-workspace lint/test/architecture/security checks feasible locally **before pushing**. Do not present a WSL local test as a hosted Ubuntu native job. Push the final technical SHA and await hosted verification. Preserve any genuine RED CI run in history and repair forward as T31.1/T31.2, never amend/rebase/force-push or weaken a gate.
5. **Hosted proof.** Obtain a single final fresh technical workflow at the same exact commit SHA with **Ubuntu `verify` SUCCESS, Windows `verify` SUCCESS and independent AVIF-reference-oracle SUCCESS**. Capture exact run/attempt/job IDs, N134 outputs and all previous retained N124–N133 verdicts. Re-check platform catalogue digest, lockfile/security gate and architecture checker. No skipped, ignored, mocked, red or absent mandatory gate is acceptance.
6. **R31 — canonical report.** Return to `main` parented to A31. Overwrite **only** `.documents/Reports/execution-report.md` recording exact technical and native evidence, all SHA/changed-path/cleanliness truth, the new Unicode source-data pins, all failures and remaining limits. Commit and push only the report; capture fresh two-host **main** verification after R31, with exact run/job IDs. Report R31's final commit SHA in the chat handoff, not inside the report itself. Keep technical branch **unmerged**. Clean task-owned `.temp` without deleting Owner files.

**Local-first suggested commands (run from the verified Solo checkout; record actual exit statuses):**

```powershell
rustc --version
cargo tree -p solo-adapters --locked
cargo test -p solo-adapters --test publication_manifest --locked n134_ -- --nocapture --test-threads=1
cargo test -p solo-adapters --test publication_manifest --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
cargo build --workspace --release --locked
```

Use the existing repository-specific architecture-checker and `cargo-deny` invocations **as defined in its workflow/SOP**, not invented commands or a global unpinned tool. Run WSL checks in the authorized **same checkout** through its mounted path; do not create a shadow checkout. Tool installation or registry access is allowed only as needed for the exact Owner-issued dependency additions. Test selection must not trigger a new decoder or unapproved source fetch.

## 5. N134: five efficient acceptance cases, run against real strict manifest bytes

Add **exactly five** `n134_01...n134_05` tests inside `crates/solo-adapters/tests/publication_manifest.rs`, preserving its existing cases unmodified. Use the **real** `parse_manifest_visibility_snapshot_v1` then new `validate_manifest_unicode_route_literals_v1`; no model-only test that bypasses the parser, R21 graph or R22 policy. Emit exactly one `n134_evidence case=0X` line **after assertions** from each `--nocapture` test.

**N134_01 — Unicode 18 normalization, distinct canonical and fold keys.** Validate actual authored locale-qualified paths with `Straße` vs `STRASSE`, Greek sigma/final sigma, dotted I, NFC composed/decomposed forms and non-Latin letters. Prove insensitive full-fold equality where required, public-insensitive Unicode lowercase canonical spelling, and sensitive/unlisted exact-NFC case preservation. No locale-specific Turkish tailoring, no Rust `std`-only case/normalization. Assert `unicode_data=18`, `fold_not_lower=true`, `sensitive_exact=true`, no URL/serving.

**N134_02 — full-locale authored contract.** Accept `en-001`, `de-DE`, `id-ID`, `zh-Hans-CN` at the *actual Site and representation declarations*, with path locale segment lowercase; refuse bare `en`, authored `de-de`, regionless locale, deprecated alias, duplicate after canonicalization, and valid-but-unsupported profile extension or variant without claiming invalid BCP47. Confirm no inferred `defaultLocale`, country or mapping and no change to R21's accepted source profile. Record `locales_checked=true`, `unsupported_profile=refused`.

**N134_03 — route segment grammar and unsafe inputs.** Refuse malformed leading/trailing/multiple slash, wrong/uppercased locale path segment, empty Page segment, greater than eight segments, greater than 60 scalars per segment, greater than 160 Page-token scalars, leading/trailing `-` or `_`, dot segments, percent/encoded separator, backslash, control/bidi formatting code point, emoji/symbol and invalid category. Accept internal `_`/`-`, Unicode letters/marks/digits under the pinned property data and legitimate NFC decomposition. Each refusal yields no partially usable result and a safe structural location. Record `invalid_route=refused`, `whole_input_refusal=true`.

**N134_04 — entire manifest ownership, aliases and Page entries.** One Entity with two sibling Sites each containing `home`, one public-sensitive representation, one public-insensitive representation, one unlisted representation, at least one public alias (with its own caseMatching), plus a mixed Page's locale-less `entryPaths`. Confirm exact fully qualified owner on **every** literal and authored order, all public alias and entry claims represented, no unlisted aliases (R22 refusal), no conversion of Page entry to a representation, and stable source digest. Use a specifically invalid alias and reserved full-locale entry to demonstrate refusal. Record `sibling_isolated=true`, `aliases_bound=true`, `entry_locale_less=true`, `before_read=true`.

**N134_05 — deterministic, nonpublishing boundary and regression.** Identical input bytes yield identical validated literal records/source SHA; label-only changes move the SHA while governed keys remain equivalent. Check R22 policy-only behavior still holds, no artificial route collision resolution, no unreviewed dependency/class imports, no source reads and all six public-authority flags false. A syntactically parsed but unsafe route **cannot** be promoted to a validated literal. Record `deterministic=true`, `route_ledger=false`, `publication=false`, `http=false`, `release=false`, `zero_partial=true`.

**N134 is bounded evidence, not the whole official Unicode corpus.** Full `NormalizationTest.txt`, `CaseFolding.txt`, `SpecialCasing.txt` and UTS #39 18.0.0 corpus/test-data SHA and independent route-oracle verification remain mandatory in the later complete E05/R04.A release gate. Do not report them passed just because N134 passes. Use normative official Unicode examples with fixed expected output; do not generate expected strings by calling the implementation under test.

### 5.1 Native authenticity / mutation gates

Add one N134 gate step to each **existing** native verify job **after N133**, without modifying existing N124–N133 steps. Capture the genuine `cargo test -p solo-adapters --test publication_manifest --locked n134_ -- --nocapture --test-threads=1` output to a task-owned temporary log; fail on command error, fewer/more than five exact named tests, any `ignored`/`FAILED`, missing evidence marker or unsafe/no-authority token. Require `test result: ok. 5 passed; 0 failed; 0 ignored`, five distinct named test/evidence lines `case=01..05`, rooted verified tokens, and the expected SHA provenance. The verifier must consume **captured test output**, never its own echoed code.

Then create and reject at least **four direct mutations of the same authentic log**: remove one case; forge fold-vs-lower or sensitive/unlisted equality; forge acceptance of a malformed/foreign route or locale; forge `publication=true`, `route_ledger=true`, a partial result or zeroed digest. Fail the whole step if any forged log is accepted. Print the two tokens **only after actual verification**:

```text
n134_gate authentic=accepted
n134_gate mutation=rejected
```

Keep the prior full workflow, lock integrity, RustSec freshness, format, strict Clippy, architecture checker, reproducible release build, CLI smoke, source/codec gates and independent AVIF oracle. Do not repeat the whole 81-icon test campaign as a **new** N134 case; existing N127 continues to run as already required. Record observed architecture counts rather than hard-code a guessed count after dependency additions.

## 6. Report, preservation and exact completion meaning

The R31 report must state `status=COMPLETE` only if all code, dependency, negative-test, two-host N134, three-job technical, security and fresh main requirements are genuinely satisfied; otherwise `status=STOPPED` with the exact evidence and blocker. It must record the actual toolchain, `Cargo.lock`/ICU4X resolved graph, Unicode-data version, warnings, advisory-db freshness, positive and refusal results, changed paths and files, exact SHAs/run IDs, test counts, preserved Owner asset checksums and cleanup truth. Required explicit outcome facts (or equivalent):

```text
manifest_route_visibility_policy_preflight_implemented=true
icu4x_2_4_0_installed_and_locked=true
unicode18_route_literal_keys_verified=true
full_locale_authored_profile_verified=true
n134_two_host_gates_verified=true
unicode18_complete_official_corpus_conformance=false
uts39_complete_confusable_gate=false
idna_base_validator_implemented=false
complete_route_collision_ledger_implemented=false
request_locale_negotiation_implemented=false
whole_manifest_semantic_validation=false
http_delivery_implemented=false
publication_authority=false
public_route_authority=false
derived_output_publication=false
release_authority=false
deployment_authority=false
r04a_open=true
r04b_open=true
r04c_open=true
r04d_open=true
r03c_open=true
stage03_open=true
```

The 81 approved Tabler assets and immutable `1.0.0` catalogue remain unchanged; precisely 20 canonical Foundation sources remain unapproved; no automatic product role default is registered. `main` remains governance/report-only, technical source stays unmerged, one checkout and one origin remain, no release, serving, DNS/provider action, public preview or deployment is authorized.

**On fully proven success, the Agent's final handoff line is exactly:**

`SOLO-WO-118 R23 — Unicode-18/ICU4X validated manifest route-literal and full-locale foundation COMPLETE (nonpublishing)`.
