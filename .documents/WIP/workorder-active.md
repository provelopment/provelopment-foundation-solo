# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-056 R01**  
**Title:** Implement R03.A — five-level typed property registry, direct-ancestry resolution, provenance and safe public projection  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Reviewed starting `origin/main`:** `8a0365ca5b14628448ff698b6b46c67090537f6c` (accepted Stage 02 closure/report)  
**Implementation branch:** `solo-wo-056-r01`, in the single verified Solo checkout only  
**Approved Scope:** Execute exactly the R03.A implementation and verification specified below. This order authorizes the specified source/test changes, the single ACTIVE/NEXT issuance transition, exact ordinary Git/CI operations, and the canonical execution report. It does not authorize R03.B, R03.C, routing, browser publication, release, deployment, or stage closure. Chief Architect independently reviews the published result and has **read-only GitHub access**.

## 1. Required outcome

Implement an actually callable, strict, typed R03.A property validation/resolution pipeline over **exactly one** Entity's direct `Entity → Facet → Site → Page → localized representation` lineage. It must consume the existing accepted `ParsedManifestCandidateV1` without treating that inert parser or any staged source candidate as publication authority.

Deliver all of the following:

1. A pure Rust domain registry for the **exact nine accepted** property identities under `propertyRegistryVersion: "1.0.0"`, with typed values, allowed EFS levels, effective maximums, equality/uniqueness, exposure validation and closed declarations. Unknown keys, wrong levels, malformed values, invalid enums, empty arrays and invalid per-value exposure **refuse**; no permissive generic property bag.
2. A five-level resolver that distinguishes absent declarations from explicit `inherit`/`suppress` plus `currentContext`, uses **direct containing parent only**, carries effective values in inherited-first/local-authored order, retains internal origin and resolution-event provenance, and enforces **effective** cardinality/uniqueness *before* public filtering. A singular ancestor plus a local value with `inherit` **refuses**; replacement requires explicit `suppress` plus the new value.
3. A separate validated **public projection** that retains only genuinely `public` effective values in order. Neither an `internal` value nor one removed by suppression may leak, revive, or cause an older public ancestor to be substituted. No internal provenance or private diagnostic values go to public output.
4. A real adapter bridge from the already parsed, duplicate-aware manifest's per-node inert `properties` to the typed domain input and an application-level inbound operation exposing a validated property-result candidate. Drive it through all five levels on a real manifest-shaped input and prove that a wrong qualified lineage or another Entity cannot select a sibling's properties.
5. Positive and adversarial tests for the new behavior on both CI hosts, preserving every accepted R01/R02 test and the unchanged architecture/security gates. Every applicable acceptance item in §5 must have direct evidence.

**Boundary:** `location` is a registered typed property, not a geographic owner or router. R03.A implements the ordinary validated **`add`-fact collection and cascade**, including the shape of a new independently authored fact, whole-record exposure, origin identity and ordering. `location` **`extend` is a recognized accepted syntax but its specialized identified-record mutation and field-provenance semantics are R03.B**: an attempted R03.A resolution involving `extend` must return a specific typed `LOCATION_EXTENSION_NOT_IMPLEMENTED_IN_R03A` refusal/unsupported-operation outcome rather than ignore it, treat it as `add`, publish a partial result, or claim that the accepted `extend` syntax is invalid. R03.B will implement the complete O08 operation later. Likewise R03.A validates a graphic selection's typed `source` reference and its cascade, **not** the physical source bytes, graphics catalogue, fallback, asset delivery or publication (R03.C/R07). Do not claim whole-product or full `O08`/`O09` conformance.

## 2. Exact preflight and governing contracts

Before any permanent write, read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§1.1, 1.3–1.7, 2, 2.3, 2.4, 5, 5A, 6.2A–6.2C, 6.2G–6.2I and 6.2K; both `.documents/SOP/` SOPs (Architect §§6, 6A, 6B, 7, 7A, 7B, 8; Implementation §§4–11); all four WIP files; and `.documents/Reports/execution-report.md`. Inspect the current source and tests under the exact paths in §3. Apply A1 in Architect SOP §7B at the exact implemented revision.

Verify:

- Actual `git rev-parse --show-toplevel` is the Owner's Solo checkout; `main == origin/main` at the reviewed baseline or an independently explained and safe descendant; clean starting tree **apart from the Owner-installed entire `SOLO-WO-056 R01` ACTIVE file**; one checkout, one worktree, sole `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, and no unknown/untracked Owner material.
- The one installed `.documents/WIP/workorder-active.md` contains **this exact `OWNER-ISSUED — SOLO-WO-056 R01` order** and the Owner handoff names that ID; no concurrent order. WIP contains exactly `plan-roadmap.md`, `plan-next.md`, `workorder-active.md`, `plan-outlook.md`.
- At reviewed baseline: ACTIVE was neutral (blob `ea5f20b736483c2e102456be1cea9104059fc81a`); NEXT contains sole R03.A (blob `7c28fe9b8c1f5a53a02aefb7d84f80e2dd027ade`); ROADMAP starts at Stage 03 (blob `4c7ef0211dffc5e29ffc64d475b8fc9d11af213e`); OUTLOOK is untouched (blob `cb9c7a610cf3c537557e060ca9656e44a053e69d`). The latest report is for `SOLO-WO-055 R01` at the reviewed main revision.
- Existing accepted surfaces are **not** already property publication authority: `crates/solo-adapters/src/publication_manifest.rs` has `DeclaredNode`, `InertPropertiesV1` and `parse_manifest_bytes`; the five EFS declaration types expose `.properties()`; `crates/solo-domain/src/efs_ownership.rs` has the validated one-Entity `EfsOwnershipGraph` and full qualified page/representation lineage; source staging never publishes. Do not replace these mechanisms.

If the reviewed baseline has moved, independently inspect ancestry, changed paths, WIP state and CI, and STOP for unexplained or conflicting changes. Do not overwrite or reset someone else's work. No second checkout, clone, shadow tree, destructive reset or force operation.

## 3. Exact permanent-write ledger

**These and only these tracked paths may be created/modified:**

| Path | Specific authorization |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Owner-installed complete order: commit the initial authorization transition **once**, then freeze its content. No agent-composed amendments or self-acceptance. |
| `.documents/WIP/plan-next.md` | In the same initial issue-transition commit, replace the R03.A candidate with **exactly** `# Foundation Solo — plan-next (Next Candidate)` and a final LF. Freeze afterwards. |
| `.documents/Reports/execution-report.md` | Overwrite the one canonical report at run end; report-only correction where actually necessary. No duplicate reports. |
| `crates/solo-domain/src/lib.rs` | Only module declarations and public re-exports for the new domain types. |
| `crates/solo-domain/src/property_registry.rs` | New accepted 1.0.0 registry, typed property keys/entries/validators. |
| `crates/solo-domain/src/property_resolution.rs` | New direct-ancestry property resolver, qualified owner/provenance, safe public-projection types. |
| `crates/solo-domain/tests/property_resolution.rs` | New pure-domain positive and refusal matrix. |
| `crates/solo-application/src/lib.rs` | Only add the new application use-case module and exports. |
| `crates/solo-application/src/property_resolution.rs` | New minimal inbound property-resolution operation; depend on domain only, no adapter or I/O. |
| `crates/solo-application/tests/property_resolution.rs` | New application use-case tests with typed/fake inputs as appropriate. |
| `crates/solo-adapters/src/publication_manifest.rs` | Minimal, backward-compatible, read-only access to authored `InertPropertiesV1` entries; preserve strict parser, key order, current candidate semantics and existing API. |
| `crates/solo-adapters/src/lib.rs` | Only new adapter module and exports. |
| `crates/solo-adapters/src/property_resolution.rs` | New strict translation from parsed manifest's five per-node declarations to validated typed domain property inputs; preserve qualified ancestry. |
| `crates/solo-adapters/tests/property_resolution.rs` | New end-to-end in-memory manifest → adapter → application/domain resolver tests including exact negative cases. |

No other permanent path may be altered. In particular **freeze** `.documents/WIP/plan-roadmap.md`, `plan-outlook.md`, About, Architecture, SOPs, README, AGENTS, all existing R01/R02 tests/fixtures and source modules other than the single accessor above, `.github/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `deny.toml`, existing architecture checker, CI workflow, CLI and starter content. Do not add a crate, dependency, schema, asset/library, server, frontend, route engine, mock publisher, generated output, extra report, WIP file, test fixture file or licence material. No copied code from another repository.

Scratch is allowed only in ignored Solo `.temp/` and the actual task's Cline-provided temporary workspace under the existing SOP. Never track `.temp/`; remove identifiable obsolete task-owned scratch at run end, preserve unknown material and disclose any cleanup limitation.

## 4. Implementation instructions — follow in this order

### A. Registry and strict authored-input translation

- Declare an exact typed key enum/registry for: `location`, `graphic.logo`, `graphic.favicon`, `graphic.banner`, `graphic.background`, `graphic.social-preview`, `seo.title`, `seo.description`, `contact.email`. All levels permit `location`, graphics and `contact.email`; only **Site, Page and representation** permit `seo.*`. Effective maxima: `location` **16**, `graphic.background` **4**, all other keys **1**. No arbitrary custom keys under initial version `1.0.0`.
- Preserve `propertyRegistryVersion == "1.0.0"`; do not change the accepted five root manifest keys. Represent an entirely **absent** inline `properties` entry separately from `Some` explicitly declared `inherit`/`none`, `suppress`/`none`, or `inherit`/a nonempty array. The declaration object has **exactly** `inheritance` and `currentContext`. `inheritance` must be `inherit|suppress`, and `currentContext` must be either the literal string `"none"` or a nonempty array. Reject `null`, `{}`, `[]`, missing fields, extra fields, wrong types, duplicate members, unknown keys and wrong-level declarations.
- Preserve `DeclaredNode`'s duplicate-aware authored member order. Add an `InertPropertiesV1::entries()` *read-only* iterator returning `(key, &DeclaredNode)` without exposing mutators or changing `parse_manifest_bytes()`'s inert return type. Make the adapter responsible for converting JSON nodes and validating closed field sets; never add a JSON parser dependency to domain or application.
- Each typed entry must be closed and explicitly `exposure: "public"|"internal"`. `seo.title` uses `{ "text": <plain-text 1..140>, "exposure": ... }`, `seo.description` similarly 1..300, and `contact.email` uses `{ "address": <single valid mailbox>, "exposure": ... }`. Refuse controls/active markup in plain text, invalid/unsafe email, `null`, extra fields, unknown exposure and coercions. A mailbox is one safe addr-spec (no CR/LF, display name, list, comments or header injection), never a mailto endpoint.
- Graphics entries use `{ "source": { "namespace": "entity"|"platform", "reference": "<portable relative image file>" }, "exposure": ... }`. Validate the closed reference, portable relative path and accepted image extensions (`png`, `jpg`, `jpeg`, `webp`, `avif`), with `graphic.favicon` **PNG only**. No remote URL, path escape, active SVG/GIF, guessed asset or fallback. This is *declaration validation only*; file existence, media decoding, immutable catalogue and delivery remain deferred.
- `location` `add` entries must conform to §6.2H: closed `{ "operation":"add", "factId": <technical-id>, "kind":"association"|"premises", "fields": <nonempty closed typed LocationFields>, "exposure": ... }`. Validate coordinates' finiteness/ranges, allowed field names/types/lengths, `countryCode`'s explicit uppercase ISO alpha-2 form, valid fact ID and complete whole-record exposure. Preserve the original qualified owner and `factId` identity and initial field origins. Do not infer missing geography or choose a primary. Recognize `"operation":"extend"` and return the dedicated R03.A unsupported-operation result **at resolution**, leaving R03.B's extension semantics untouched; do not silently accept an incomplete extension.

An accepted simple authored declaration at a representation, illustrating the required **replacement** of a singular inherited value, is:

```json
"properties": {
  "seo.title": {
    "inheritance": "suppress",
    "currentContext": [
      { "text": "About our work", "exposure": "public" }
    ]
  }
}
```

The same declaration with `"inheritance":"inherit"` **must refuse** when an ancestor already has an effective `seo.title`, even when one value is `internal`.

### B. Domain resolution and provenance

- Build a typed, validated property input attached to each actually declared EFS owner. Check parent containment against the existing one-Entity ownership graph; accept an Entity root and only a **contiguous qualified lineage** to the targeted node. Do not select by bare Page ID, route, filename, Site label, base URL, locale-as-geography, or find the first matching sibling. Reused IDs in distinct parents remain lawful.
- For each registered key resolve along the target's exact direct chain: begin at the Entity with an empty inherited set; on a missing key carry the immediate parent's effective result; on explicit `inherit` carry that result then append local typed contributions in original array order; on `suppress` clear inherited contributions and then add local values; `"none"` contributes no local values. Never use nearest-wins, a parent/sibling search, silent deduplication, a generic merge, or post-projection cardinality checks.
- Enforce maximum and duplicate/uniqueness at **each effective level before** projecting public values. Distinct location facts are uniquely identified by `(originOwner, factId)`; the same original fact inherited downstream is **not** a duplicate, whereas two authored occurrences with the same original identity or conflicting entries refuse. `graphic.background` refuses repeated exact source references in one effective ordered collection. `graphic.logo`, `seo.title` etc. reject a second effective value, even if one is internal. There is no automatic platform graphic fallback under any role.
- Retain exactly the internal §6.2G-derived provenance shape: `{ property, values, resolutionEvents }`, where each value contains `data`, `exposure`, `originOwner`, `originLocalIndex` (and `fieldOrigins` only for location), and each event has `owner`, `operation` (`inherited|added|suppressed|extended`), `localIndex` (nullable only as specified), and optional `targetFactId` for an extension. Preserve original owner and index through inheritance; trace ancestor inclusion and suppression in direct-ancestry order. Do not invent an additional wire field or expose raw diagnostic records publicly. Distinguish an absent declaration from explicitly authored `inherit` + `none` inside the typed internal model and its verification evidence; never conflate their authored meanings.
- Expose a **separate** public projection entry point/type. It accepts only a validated resolved result and outputs only `public` entries in their effective order. No `internal` values, internal `resolutionEvents`, origin filesystem path, source bytes or secret-bearing diagnostic content. A filtered result of zero values stays zero, **not** a resurrected ancestor or generic fallback. Suppression means a value removed upstream cannot reappear downstream unless an independently authored later contribution explicitly adds it.

Illustrative algorithmic shape (adjust identifiers to the actual Rust API, not the rule):

```rust
let mut effective = match declaration {
    None => parent_effective.to_vec(),
    Some(d) if d.inheritance == Inheritance::Inherit => parent_effective.to_vec(),
    Some(_) => Vec::new(), // explicit suppress clears the inherited set
};
if let Some(d) = declaration {
    effective.extend(d.local_additions_in_authored_order()?);
}
registry.validate_effective(key, &effective)?; // BEFORE projecting public
let public_values = effective.iter()
    .filter(|value| value.exposure() == Exposure::Public)
    .map(ValidatedPublicValue::from)
    .collect::<Vec<_>>();
// Never retry the ancestor when public_values is empty.
```

The real implementation must additionally preserve provenance and special-operation refusals; this excerpt is **not** permission to skip them.

### C. Application and adapter integration

- Add a minimal application inbound operation that accepts validated typed input, the selected fully qualified owner, and the existing validated EFS graph, and returns the domain-resolved **property candidate** plus a safe public projection. No disk/network/file/HTTP port is required for pure property calculation; do not create ceremonial outbound ports. Domain and application depend inward only, adapter translates outward JSON. Do not import `solo-adapters`, `serde`, JSON nodes, filesystem, platform services or providers into the domain/application crates.
- In `crates/solo-adapters/src/property_resolution.rs`, traverse `ParsedManifestCandidateV1::entity()` and its actual contained Facet/Site/Page/representation arrays; obtain each optional `properties()` via the new read-only accessor and attach its typed declarations to that **same owner**. Validate every authored declaration including unlisted representations, not only the requested public target. No directory scanning or source-file selection determines property ownership.
- Implement a deliberately named nonpublishing public adapter operation (e.g. `validate_manifest_property_candidate_v1`) that returns a typed whole-graph property candidate or a typed refusal. Do **not** rename `parse_manifest_bytes()` to imply it now grants full publication authority; do **not** report property validation as manifest routing, locale/canonical or source-snapshot validation.
- Keep refusal diagnostics stable, typed and owner/key-specific, but **never echo internal property contents, confidential data, filesystem paths or tokens**. An unknown property should be reported by a safe category and qualified authored location, not by serializing a sensitive payload.

## 5. Mandatory acceptance tests (all are gates)

Add named Rust tests at the three **new** test paths in §3. Each test must assert actual values, origins, event order or a specific refusal. No empty/assert-true tests and no mocked execution that merely asserts the intended source code exists.

| Case | Required decisive evidence |
| --- | --- |
| `R03A-01` Registry and closed encoding | All nine registered keys, version `1.0.0`, correct allowed levels/maxima; wrong key, extra envelope field, `null`, `[]`, invalid `inheritance`, unknown exposure and wrong-level `seo.title` refuse. |
| `R03A-02` Five-level ancestry | Entity → Facet → Site → Page → representation real positive path; absent declarations inherit only direct ancestors; change one parent's value, observe only descendants on that branch change. |
| `R03A-03` Explicit empty and suppression | Absent versus explicit `inherit`/`none` are distinctly represented; `suppress`/`none` yields no effective values; `suppress`/local adds new; inherited values never leak back downstream. |
| `R03A-04` Ordering and provenance | Inherited-first, then ordered local values; exact qualified `originOwner` and zero-based `originLocalIndex`, `inherited`/`added`/`suppressed` events, correct location initial `fieldOrigins`; repeated resolution is byte-/value-deterministic. |
| `R03A-05` Effective cardinality | Singular inherited + local `inherit` refuses; `graphic.background` fifth effective layer refuses; location seventeenth fact refuses; limits checked **before** private filtering. |
| `R03A-06` Duplicate and typed refusal | Duplicate background `namespace/reference`, duplicate location original `(owner,factId)`, malformed asset reference, favicon JPEG, malformed email/text, wrong location field/coordinates and invalid fact ID refuse without repair. |
| `R03A-07` Exposure | Public/private ancestor and descendant combinations; public projection omits every `internal` contribution and all internal provenance, never reveals suppressed/private fallback; a hidden internal value still counts for singular conflict. |
| `R03A-08` Qualified isolation | Identical `home` IDs under different Sites, different Facets and locales; wrong Entity/Facet/Site/Page/representation refuses and never borrows a sibling; no second Entity or implicit routing/geography path. |
| `R03A-09` Actual adapter boundary | Start from real strict JSON bytes with nested inline `properties`, run `parse_manifest_bytes` → new adapter translation → app/domain use case → validated public projection; prove all five owner levels and both positive/refused cases. The original parser remains inert and existing R02.C fixture tests stay unchanged/green. |
| `R03A-10` Deferred specialists remain honest | `location` add inheritance works; a valid `extend` is recognized but produces the **named R03.A unsupported-operation outcome**, not wrong in-place merging. Graphic typed references resolve as declarations, but no file/asset library, HTTP output, fallback or release is claimed. |
| `R03A-11` Invariants and security | Changing properties alone does not mutate Entity/Page IDs, locale keys, route strings or configured base; `internal` fields never appear in any public serialization, diagnostic text or output props. Fail closed on one invalid declaration anywhere in the one-Entity candidate. |
| `R03A-12` Existing baseline and CI | Existing R01/R02 tests, inward-dependency/unsafe refusal checks, locked full all-target tests, strict Clippy, format, cargo-deny/security refresh, deliberate CI tripwire, Windows native guards, Linux Btrfs cases, reproducible release build and CLI smoke remain green on both designated hosts at the exact substantive SHA and final report SHA. |

Add a test with this specific distinction: an Entity-level `seo.title` **cannot** be used as a valid case (SEO is Site/Page/representation only). Use `contact.email` or `graphic.background` for an all-five-level inheritance trace; use `seo.title` for an allowed Site→Page→representation singular/suppression case. This level rule is a mandatory refusal, not a test convenience.

Do not alter existing tests/CI to reduce or conceal a failure. If the actual Windows workstation lacks the MSVC linker, explicitly record local `NOT RUN` and use the project's **real** Ubuntu 24.04 and Windows 2025 GitHub CI for the exact revision. A green workflow alone does not prove the new tests cover the cases above; identify the test names and assertions in the report.

## 6. Exact execution, Git, CI and reporting sequence

1. **Authorization transition.** After the §2 preflight, commit **only** the Owner-installed `workorder-active.md` and the cleared `plan-next.md` on `main`. Push normally and verify `origin/main` commit and tree. The ROADMAP remains untouched and Stage 03/R03.A remains outstanding throughout technical execution. Confirm the two-host existing workflow at this authorization revision.
2. **Implementation branch.** From that verified updated `origin/main`, create `solo-wo-056-r01` in the existing checkout. Implement precisely §3–§5. Commit only allowed code/test paths on this branch; push it normally. Do not open a pull request or a second checkout.
3. **Focused checks.** Run `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo test --locked -p solo-domain --test property_resolution`; `cargo test --locked -p solo-application --test property_resolution`; `cargo test --locked -p solo-adapters --test property_resolution`; then `cargo test --locked --workspace --all-targets`. Execute the repository's **unchanged** architecture and security verification through the existing `rust-verify` workflow. Honor the pinned Rust `1.99.1`, tracked `Cargo.lock` and `cargo-deny 0.20.2` gate. Do not alter locks, version pins, runner labels or tests.
4. **Identical-SHA CI.** Verify both `ubuntu-24.04` and `windows-2025` jobs pass at the **same exact branch commit** with required host-specific steps and real new tests. Review job logs, material warning/skip inventory, dependency/advisory source freshness and A1 version-currency findings. No skipped applicable check may be marked PASS; no material warning may be silenced.
5. **Substantive integration.** After independently verifying the source diff, changed-path ledger, no conflict and clean worktree, **fast-forward only** the verified branch into `main` in the same checkout; ordinary push to `origin/main`, no squash/rebase/force. Verify exact substantive SHA/tree and obtain a **fresh two-host green** CI on substantive `main`. STOP if fast-forward or remote state is unsafe. Retain branch for separate Owner-authorized later pruning.
6. **Canonical report last.** Overwrite `.documents/Reports/execution-report.md` with `SOLO-WO-056 R01`, ISO date, status `COMPLETE` only when all in-scope behavior is implemented and verified (otherwise `STOPPED`/`FAILED`), objective, actual root/branch, starting/authorization/branch/substantive commits and trees, every changed path, test matrix `R03A-01`–`12` with direct code/test/CI evidence, CI run IDs/jobs, dependency/security/version-currency findings, warnings/skips, limitations, scratch cleanup, no extra checkout and truth about unimplemented R03.B/R03.C. Commit/push **only** this report from `main`, then verify fresh two-host CI **at report HEAD**. Never embed the report's own commit SHA in the report; return final SHA and link in agent chat only. Verify clean `git status --porcelain`, one root/worktree/remote, correct WIP inventory and identical local/remote final HEAD.

**Explicit permissions:** On this Solo repository only, the Implementation Agent may perform the one bounded ACTIVE/NEXT transition, modify allowed paths, create one named task branch, ordinary commits and pushes, fast-forward-only integration to `main`, and read/dispatch the existing GitHub Actions verification. Network use is allowed for the unchanged locked dependency fetch, pinned verification tools/security database refresh, official read-only version/advisory research and normal Git operations. Installing only the already-pinned Rust toolchain/components for local verification is permitted; **adding/upgrading dependencies or changing manifest/lock/toolchain pins is not**. No PR, merge commit, branch deletion, force push, history rewrite, destructive Git cleanup, new remote, other-repository change, secret/provider operation, package publication, release/tag, deployment, DNS/TLS action, purchase or privileged external service is authorized. The Chief Architect has **no write access or write permission on GitHub** and performs independent read-only review.

**Reporting discipline:** Only one canonical `.documents/Reports/execution-report.md`; no archive or parallel log. The agent must never write `ACCEPT` as its own disposition, clear ACTIVE, remove R03.A/Stage 03 from ROADMAP, or advance NEXT. A completed implementation awaits the Chief Architect's independent `ACCEPT / REVISE / STOP`; stage/active closure takes a **separate Owner-issued administrative order**.

## 7. Mandatory STOP and escalation triggers

STOP without destructive changes if the Owner-installed order or matching handoff is missing/mismatched; actual root, branch, remote, WIP inventory or reviewed history differs without an explanation; Owner-owned material is dirty; a second checkout/worktree is present; a required new behavior cannot fit §3's exact allowed paths; a new dependency/major architecture choice is necessary; the accepted §6.2G/§6.2H/§6.2I contract conflicts with the planned typed model; direct ancestry or provenance cannot be proven; a private value leaks; a fallback bypasses `suppress`; a valid `extend` is silently miscomputed; an applicable mandatory gate fails or is unverified; the branch cannot fast-forward safely; or a public source/security issue would be disclosed unsafely.

On STOP, preserve safe existing work, publish the single report with precise completed subset, failing requirement, exact command/refusal evidence, smallest necessary decision and safe resumption point whenever the SOP's report-only safety preflight permits. Do not self-amend this order or create a second work order. The Chief Architect will review the exact report and give a bounded correction directive or a replacement fully issued order through the Owner.

**End of SOLO-WO-056 R01.**
