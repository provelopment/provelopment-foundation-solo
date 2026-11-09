# Execution Report

**Order:** `SOLO-WO-048 R01` — complete the remaining `R02.B` source-stage set validation. The Owner installed the complete order at `.documents/WIP/workorder-active.md` and sent the matching `SOLO-WO-048 R01` handoff; the Implementation Agent executed it and reports **only to the Owner**.
**Disposition: technical completion — `COMPLETE`.** The whole declared set is now read as one all-or-refused, owned, inert **source-stage** candidate, demonstrated by real-adapter and fake-port evidence on both hosts. No STOP condition arose, no gate failed, and no test was skipped, weakened or made conditional. R02.B closure and R02.C remain **out of scope** until an independent Architect `ACCEPT`.
**Date:** 2026-11-09.
**Repository:** `provelopment/provelopment-foundation-solo`, branch `main`, sole `origin`, one checkout, one worktree.
**Starting HEAD:** `9a9a01a125808809118e95614204b47d6b6d5693` / tree `51586be3ce30d2667273f96a39d19341790688a6` — verified against the §3 ledger below before any edit: **25 of 25 blobs matched**, and both optional paths were absent.
**Issuance HEAD:** `51d92ca4bffd930a34b8271559433d89931b16b6` / tree `8bd275092312cc89684432c2be8ba99b8eea98db` — the single atomic issuance transition (Owner-installed ACTIVE + NEXT neutralization). `rust-verify` run `136` (id `42642281217`) **success**: Ubuntu job `129246212332`, Windows job `129246213156`.
**Implementation HEAD:** `b17ca4126e968d9015838062727767f6694ade30` / tree `8ec1cf4544f400892247f307c9ce3e23fe3c176f`. `rust-verify` run `138` (id `42645845709`) **success**: Ubuntu job `129258747800`, Windows job `129258747412`.
**Documentation HEAD:** `518a17f82955ecec492a6d01d522b787fea82acf` / tree `166effb892ad1bdc50003b09c22688e31ca3a5df` — the one bounded §6A.8 parity sentence. `rust-verify` run `139` (id `42646640367`) **success**: Ubuntu job `129261540810`, Windows job `129261540433`.
**Report HEAD:** its SHA and its own run/job identifiers are given to the Owner in chat; a commit cannot contain its own SHA.

## 1. What was proven, in one table

| Claim | Evidence |
| --- | --- |
| Every representation the sole parsed manifest declares is read exactly once, in manifest order | `every_declared_representation_is_assembled_with_its_lineage`, `an_expanded_manifest_assembles_every_representation_exactly_once_in_order` (REAL ADAPTER, both hosts); `every_declared_representation_is_read_in_declaration_order` (APPLICATION FAKE) |
| One invalid representation refuses the **entire** set, with no partial candidate and no synthesis | `one_invalid_representation_refuses_the_whole_set` (REAL ADAPTER); `one_refused_declaration_refuses_the_whole_set_and_the_next_one_is_never_read` (APPLICATION FAKE) |
| The accepted JSON-first rule is unchanged: a present JSON body wins and the unselected Markdown body is never read or preflighted | `a_declared_json_body_wins_and_an_invalid_unselected_markdown_is_never_read` (REAL ADAPTER: the unselected `.md` holds bytes the Markdown preflight refuses); `a_present_json_body_is_chosen_and_its_declared_markdown_is_never_read` (APPLICATION FAKE, asserts the exact call ledger) |
| Only confirmed JSON **absence** permits the declared Markdown body, and a present-but-invalid JSON body never falls back | `markdown_is_selected_only_when_the_declared_json_is_confirmed_absent`, `a_present_but_invalid_json_body_is_refused_with_no_markdown_fallback`, `a_confirmed_json_absence_selects_markdown_in_that_declaration_only` |
| Physical ownership is settled for every declared candidate of every representation before any body is read | `two_declarations_hardlinked_to_one_object_are_refused_before_any_body_is_read` (REAL ADAPTER, real hardlink), `a_cross_representation_identity_collision_is_refused_before_any_body_is_read`, `an_unselected_hardlinked_markdown_refuses_the_whole_set_before_any_body_is_read` (APPLICATION FAKE) |
| Duplicate declaration claims are refused before any read | `a_manifest_declaring_one_locale_twice_is_refused_before_assembly` (REAL ADAPTER, strict parser) |
| A change actually observed during a set attempt refuses the whole attempt | `a_change_observed_in_the_second_declaration_refuses_the_whole_set` (APPLICATION FAKE, `SNAPSHOT_CHANGED`, declaration `1`) |
| Undeclared content is never searched, selected or consumed | `an_undeclared_body_is_never_searched_selected_or_included` (REAL ADAPTER) |
| The result is owned: later source replacement cannot change the bytes or identity it reported | `owned_bytes_and_identity_survive_a_change_to_the_source_after_assembly` (REAL ADAPTER) |
| Neither the aggregate nor any member can claim publication or snapshot authority | `no_complete_set_can_claim_publication_or_snapshot_authority`, `no_read_outcome_can_claim_publication_or_snapshot_authority`, and literal `false` constants |
| Every refusal is typed, bounded and names only a declaration | `a_set_refusal_diagnostic_names_only_a_declaration`, `a_refused_manifest_is_reported_before_anything_is_read`, `diagnostics_stay_bounded_and_carry_no_location` |

## 2. Revisions, and every run in this order

| Revision | What it was | CI run | Result |
| --- | --- | --- | --- |
| `51d92ca4bffd930a34b8271559433d89931b16b6` / `8bd27509…` | **issuance transition**: the Owner-installed ACTIVE order plus neutralization of the old candidate back to the exact neutral NEXT blob `92fa562a…`, in one atomic commit; no other path | 136 `42642281217` | **success** — Ubuntu `129246212332`, Windows `129246213156` |
| `9c894fd2856ed4eec811fb36ed357cfffe7c6418` / `f888ad4c…` | application whole-set orchestration and its fake-port tests (3 files) | 137 `42644986948` | **success** — Ubuntu `129255724781`, Windows `129255725427` |
| `b17ca4126e968d9015838062727767f6694ade30` / `8ec1cf45…` | **implementation HEAD**: outer adapter composition, crate wiring and the real-adapter integration proof (3 files) | 138 `42645845709` | **success** — Ubuntu `129258747800`, Windows `129258747412` |
| `518a17f82955ecec492a6d01d522b787fea82acf` / `166effb8…` | documentation HEAD: one bounded §6A.8 parity sentence | 139 `42646640367` | **success** — Ubuntu `129261540810`, Windows `129261540433` |
| this report-only commit | the single canonical report | its own run is reported to the Owner in chat | must be green on both hosts before audit |

Every revision above was fully green on **both** hosts before the next began. The starting HEAD `9a9a01a` remains the last revision of the previous, administratively closed order.

## 3. Frozen path ledger — exactly the authorized operations

Ten paths changed in total (`git diff --name-status 9a9a01a..HEAD`), including this report: ledger items **1, 2, 4, 5, 6, 7, 8, 9, 10 and 12**. Items **3** (ROADMAP) and **11** (the existing preflight) were not touched at all, and no path outside the ledger was written.

| # | Path | Operation performed |
| --- | --- | --- |
| 1 | `.documents/WIP/workorder-active.md` | Owner installed the complete R01 text; the first Agent commit recorded that substitution verbatim (blob `475112622a40ee948058d2f38b755b30549d355c`, 32,655 B). **Still issued.** |
| 2 | `.documents/WIP/plan-next.md` | Neutralized at issuance only, to the exact neutral blob `92fa562b1af0c54391851aa5c79deb1631127069` (49 B). No candidate during implementation. |
| 3 | `.documents/WIP/plan-roadmap.md` | **UNTOUCHED** — blob `18975332c2820170a32558c4ee224176667b91ab`, identical to the starting HEAD. R02.B is still listed; nothing was preemptively removed. |
| 4 | `.documents/Reports/execution-report.md` | This single canonical report. |
| 5 | `crates/solo-application/src/source_read.rs` | `+230`: the whole-set orchestration `read_declared_source_set_v1`, its typed aggregate `ReadSourceSetV1` / `SetSourceEntryV1` / `SourceReadSetOutcomeV1` / `SourceReadSetRefusalV1`, and two consuming accessors. The accepted per-representation selection and refusal semantics are unchanged. |
| 6 | `crates/solo-application/src/lib.rs` | `+8/−1`: the new safe exports only; every previous export preserved; dependency direction unchanged. |
| 7 | `crates/solo-application/tests/source_read.rs` | `+375`: nine set-level cases with a locator-keyed fake port; all pre-existing tests preserved byte-for-byte and still running. |
| 8 | `crates/solo-adapters/src/source_set_candidate.rs` | **CREATED**, 504 lines: the composition over the accepted parser, capability, whole-set read and selected-body preflight. No file-opening implementation, no publication, no unsafe. |
| 9 | `crates/solo-adapters/src/lib.rs` | `+5`: module declaration and re-exports only; no lint or allowance change. |
| 10 | `crates/solo-adapters/tests/source_set_candidate.rs` | **CREATED**, 542 lines: eleven real-adapter cases over exclusively owned task fixtures. |
| 11 | `crates/solo-adapters/src/source_body_preflight.rs` | **UNTOUCHED** — blob `e9a336a7585d9ed5c15df1b6811be3cff71bc17f`. The existing preflight already consumes chosen verified bytes, so no adaptation was needed and no policy was duplicated. |
| 12 | `.documents/Architecture/ARCHITECTURE.md` | One sentence appended to the existing §6A.8 trust/probe/reservation paragraph, after the proof was green. |

**Byte-identical to the starting HEAD:** `Cargo.lock` (`8bd0bb00…`), `crates/solo-adapters/Cargo.toml` (`a677df5f…`, still the single `Win32_System_Memory` entry), `crates/solo-adapters/src/source_read.rs` (`c651bb3a…`), `crates/solo-adapters/src/source_read_native.rs` (`787a6842…`), `crates/solo-adapters/src/publication_manifest.rs` (`f883dd93…`), `crates/solo-adapters/tests/source_read.rs` (`6e5ab1ea…`), every `solo-domain` file, the CLI checkers and their fixtures, `deny.toml`, `rust-toolchain.toml`, the workflow, both SOPs, README, AGENTS and `.documents/WIP/plan-outlook.md` (`cb9c7a61…`).

## 4. Contract map — five different things the word "member" could mean

Written before any source edit, as §4.I requires.

| Term | What it is | Status in this order |
| --- | --- | --- |
| (a) a declared candidate **path** | one validated portable locator in the manifest's `sourceCandidates`, e.g. `pages/home.en-001.json` | already accepted; **probed and reserved** for every representation, including a Markdown locator that is never read |
| (b) a JSON-first selected **body** | the one member actually chosen and read for one representation | already accepted; **read exactly once** per representation, then preflighted |
| (c) one published EFS **representation** | one Entity/Facet/Site/Page/locale tuple | now carries the qualified lineage and the selected source evidence |
| (d) the full declared **set** | every representation the sole parsed manifest declares | **completed by this order**: one all-or-refused aggregate |
| (e) the §6.2K **publishable** candidate | the sealed artefact, its claim ledger, rendered output and digests | **not** attempted, not claimed, and explicitly out of scope |

**Checks this order completes:** strict manifest parsing with duplicate/unknown/version refusal; complete enumeration of declared representations with unique qualified lineage; cross-representation physical reservation of every declared candidate before any body read; selected-only body read under the accepted per-file guarantees; selected-body content validation (strict UTF-8, duplicate-key rejection, accepted root/content version for JSON; bounded UTF-8, BOM rejection and non-empty rules for Markdown); one owned, immutable, ordered aggregate or one typed refusal.

**Checks that remain in later stages** (and are *not* claimed here): full Markdown/GFM and the twenty-two-module body, link and asset validation, route and locale resolution, property and visibility projection, rendering, and every part of sealing a publishable artefact. The source-stage candidate performs none of them, and §6 of this report states so again.

**Refusal vocabulary.** No new code string was invented for an existing condition: the set refusal delegates to the accepted `SOURCE_*`, `PREFLIGHT_*` and `SNAPSHOT_CHANGED` tokens. `NoDeclaredRepresentation` reuses `SOURCE_NONE_EFFECTIVE`; `DuplicateLineage` reuses the registered duplicate-claim token `PREFLIGHT_SOURCE_CLAIM_DUPLICATE`; the internal `SelectionDisagreement` reuses `SOURCE_UNSUPPORTED`. No public key, route or locale was created anywhere in this order.

## 5. What was implemented

**Application layer — orchestration only, no I/O, no new dependency** (`crates/solo-application/src/source_read.rs`). `read_declared_source_set_v1(port, declarations)`:

1. refuses an empty declaration list as `SOURCE_NONE_EFFECTIVE` rather than returning a successful empty set;
2. runs the accepted `check_physical_reservations_v1` over **all** declarations first, so a hardlink, alias or cross-representation identity collision refuses the entire attempt before any body is opened;
3. then calls the accepted `read_declared_source_v1` once per declaration, in declaration order, so the JSON-first rule and every per-file guarantee apply unchanged;
4. returns `SourceReadSetOutcomeV1::Complete(ReadSourceSetV1)` only after the last declaration succeeds — the collected entries are dropped on the first refusal, so no partial aggregate type can exist.

`ReadSourceSetV1` exposes only `len`, `is_empty`, `entries`, `get`, `iter`, consuming `into_entries`, and the two authority predicates that are literally `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` and `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` — both `false`. `SourceReadSetRefusalV1` carries the accepted refusal plus the declaration index, and its `Display` names that index only.

**Adapter layer — composition, no new mechanism** (`crates/solo-adapters/src/source_set_candidate.rs`). `assemble_declared_source_set_v1(manifest_bytes, root)`:

1. `parse_manifest_bytes` (the accepted strict parser) — a refusal is reported before anything is read;
2. `declare_every_representation` walks Entity → Facets → Sites → Pages → representations only, building the qualified lineage and refusing a duplicate lineage; nothing is listed on disk and no manifest is inferred from filenames;
3. `read_declared_source_set_v1(root, &sets)` over the **real** `ContentRootCapability` — reservations, then one selected read per declaration, with the Linux D2 clone and the Windows share/mapping exclusion exactly as accepted;
4. per declaration, the observations for the body in hand are rebuilt and handed back to the accepted `select_provisional_source`, the chosen kind/path must agree with the read, and only then does `preflight_selected_body` produce the owned `InertBodyCandidateV1`; a disagreement or a fingerprint mismatch refuses the set;
5. the result `SourceSetCandidateV1` holds the entity id and, per declaration, `SourceSetMemberV1` with qualified lineage, member kind, logical locator, physical identity, linkage fingerprint and preflighted owned bytes — with no mutable accessor and `false` authority predicates.

No new file-opening code, crate, feature, dependency, build script, unsafe region, global maximum, process or network call was added. Counts and allocations follow the accepted manifest and source bounds; nothing is truncated, and no arbitrary new product maximum was introduced.

## 6. Required evidence matrix — B1 to B11

Classification is explicit: **REAL ADAPTER** cases drive the accepted parser, capability, port, selection and preflight over real files; **APPLICATION FAKE** cases drive the pure orchestration through a recording fake port; **PURE** cases observe constants. Every case below was executed on both hosts by run `138` and is also reproducible from the stated command.

Command on both CI hosts: `cargo test --workspace --all-targets --locked`.

| Gate | Test(s) | Class | Result |
| --- | --- | --- | --- |
| `B1` one Entity/Facet `main`/Site `main`/Page `home`, several published full-locale representations | `every_declared_representation_is_assembled_with_its_lineage` — three locales, exact lineage/kind/path/bytes/fingerprint per member | REAL ADAPTER | pass, both hosts |
| `B2` expanded single Entity, two Facets, three Sites, repeated Page id in different parents, locale repeated across Pages | `an_expanded_manifest_assembles_every_representation_exactly_once_in_order` — five representations, exact lineage tuple list, exact path order, identical result on a second attempt | REAL ADAPTER | pass, both hosts |
| `B3` JSON and Markdown both declared and both present: JSON wins, Markdown body never read | `a_declared_json_body_wins_and_an_invalid_unselected_markdown_is_never_read` — the unselected `.md` holds bytes the Markdown preflight refuses, so a read would fail the test; plus the call ledger `a_present_json_body_is_chosen_and_its_declared_markdown_is_never_read` | REAL ADAPTER + APPLICATION FAKE | pass, both hosts |
| `B4` JSON positively absent selects Markdown; malformed or unreadable present JSON fails with no fallback; single-member and both-missing cases | `markdown_is_selected_only_when_the_declared_json_is_confirmed_absent`, `a_present_but_invalid_json_body_is_refused_with_no_markdown_fallback`, `a_confirmed_json_absence_selects_markdown_in_that_declaration_only`, plus the retained `markdown_only_is_chosen_when_it_is_the_only_declared_member` and `confirmed_absence_of_every_declared_member_is_none_effective` | REAL ADAPTER + APPLICATION FAKE | pass, both hosts |
| `B5` one bad representation among valid ones refuses the whole set; no partial candidate | `one_invalid_representation_refuses_the_whole_set` (invalid body root at declaration 1 of 3), `one_refused_declaration_refuses_the_whole_set_and_the_next_one_is_never_read` (asserts the third declaration is never read) | REAL ADAPTER + APPLICATION FAKE | pass, both hosts |
| `B6` duplicate/alias/case-folded claims, cross-representation identity collision, unselected or singleton hardlink, link and reparse escape | `a_manifest_declaring_one_locale_twice_is_refused_before_assembly`, `two_declarations_hardlinked_to_one_object_are_refused_before_any_body_is_read` (real hardlink, `SOURCE_COLLISION`, set-level), `a_cross_representation_identity_collision_is_refused_before_any_body_is_read`, `an_unselected_hardlinked_markdown_refuses_the_whole_set_before_any_body_is_read`, plus the retained link/reparse and second-directory-entry cases in the unchanged `source_read` suite | REAL ADAPTER + APPLICATION FAKE | pass, both hosts |
| `B7` mutation or replacement between probe and read, identity/link/timestamp disagreement, late observed change | `a_change_observed_in_the_second_declaration_refuses_the_whole_set` (`SNAPSHOT_CHANGED`, declaration 1), plus the retained `a_probe_to_read_replacement_that_keeps_identity_and_length_is_refused`, `a_read_of_a_different_object_than_the_probe_located_is_refused`, `a_read_whose_length_disagrees_with_the_probe_is_refused`, `a_link_count_that_changes_between_probe_and_read_is_refused`, `a_body_that_vanishes_after_its_probe_is_a_change_not_an_absence`, and the real-host rewrite case in the unchanged `source_read` suite | APPLICATION FAKE + REAL ADAPTER (retained) | pass, both hosts |
| `B8` an undeclared body or draft is never searched, selected or included; no filename-derived manifest; no shadow authority | `an_undeclared_body_is_never_searched_selected_or_included` — an extra valid JSON file sits beside the declared one and is neither included nor consumed | REAL ADAPTER | pass, both hosts |
| `B9` real Linux D2 clone + tmpfs refusal and real Windows mapped-view exclusion retained, with unchanged native boundary and checkers | Retained unchanged and still executing: the Linux anonymous `FICLONE` clone with its Btrfs positive and tmpfs negative proof, the unlink/replacement matrix, the five Windows `PAGE_READWRITE` mapped-view cases, the exclusive fixture-collision case, `unsafe_boundary` (8 cases) and the architecture checker + 26-case fixture matrix | REAL ADAPTER (retained) + PURE | pass, both hosts |
| `B10` owned result survives caller changes; no unsafe aliasing; no partial output | `owned_bytes_and_identity_survive_a_change_to_the_source_after_assembly` — the source is replaced after assembly and the bytes, identity and fingerprint stay exactly as reported; `no_complete_set_can_claim_publication_or_snapshot_authority` | REAL ADAPTER + PURE | pass, both hosts |
| `B11` source-stage versus deferred Stage 03–05 validation stated accurately, never mislabelled | This report §4 and §9, the new module's own documentation, and the one §6A.8 parity sentence. No later-stage capability is claimed anywhere | PURE (documentary) | satisfied |

No mocked positive is the sole proof of the adapter: B1, B2, B3, B4, B5, B6, B8 and B10 each include a real-filesystem case, and the fake cases add the call-ledger assertions a real filesystem cannot show.

## 7. Gates at the implementation HEAD

`rust-verify` runs `136`, `137`, `138` and `139` are each fully green on both `ubuntu-24.04` and `windows-2025`, with the per-host job identifiers listed in §2. Each run exercises the workflow's own mandatory gates: locked dependency fetch; the pinned `cargo-deny 0.20.2` gate with a fresh RustSec snapshot, bans, licences, sources and the known-advisory negative probe; the four-workspace-package guard; `cargo fmt --all -- --check`; all-target strict Clippy with `-D warnings` on both targets; `cargo test --workspace --all-targets --locked` including every new case; the real-tree architecture checker with its refusal-fixture matrix; both failure-injection tripwires; the forbidden-import refusal with guaranteed restore; the release build and CLI smoke; two byte-identical clean release builds; and the guaranteed Linux Btrfs/tmpfs fixture setup and cleanup.

**Measured source-graph counts at the documentation HEAD** (printed by the real-tree checker, which asserts legality rather than a frozen number, and was not modified): `architecture=legal packages=4 targets=22 files=38 edges=40`. That is `+1` target and `+2` files over the previous baseline of `21/36/40` — exactly the new `source_set_candidate` module and its test target — with **no new legal edge** and no newly illegal edge. The fixture self-check remains `self-check=passed cases=29`, `unsafe_boundary` remains 8 cases, and the CLI crate's own suites (`check-architecture`, 26 fixture cases, `workspace_graph` 27, `unsafe_boundary` 8, `ci_gate_tripwire`, `cli_smoke` 15) all pass.

**Test inventory executed in this order:** application `31` (22 retained + 9 new), domain `9/13/27` across its suites, adapters `6/11/16/18/21/11` across its six targets (the `11` is the new proof target; the `21` is the retained `source_read` suite), CLI as listed above. No test was deleted, weakened, ignored, made conditional, or made green by removing a failure.

**Local pre-verification disclosure.** Before each push the same trees were built, formatted, linted and tested on the Owner's workstation. Two honest limitations of that local host: (a) it has no MSVC linker, so local iteration used the *same pinned `1.99.1` toolchain's* `x86_64-pc-windows-gnu` host variant plus a task-owned portable mingw-w64 in `.temp/`, while the authoritative CI builds used the pinned default host on both runners; (b) the workstation lacks `SeCreateSymbolicLinkPrivilege`, so the pre-existing, unmodified case `a_leaf_that_is_a_link_is_refused` cannot create its mandatory symlink locally and was filtered **locally only** — it is untouched and passes on `windows-2025` in runs `136`–`139`. Neither limitation touches the delivered code, the ledger, or any CI gate.

## 8. Security, dependency and version review (2026-11-09)

* **Dependency surface unchanged.** No crate, feature, version, registry, build script or provider was added. `Cargo.lock` remains `8bd0bb00…`; the four-package closure, `rustix =1.1.5` (Linux only), `windows-sys =0.61.2` (Windows only, still the single `Win32_System_Memory` addition approved under `SOLO-WO-047 R04`), `serde`/`serde_json` and the pinned `1.99.1` toolchain are all as accepted. The `cargo-deny 0.20.2` gate with a fresh RustSec snapshot passed on both hosts in every run of this order, with no applicable advisory, licence or source finding.
* **`unsafe` unchanged.** `solo-adapters` still `#![deny(unsafe_code)]` with exactly one Windows-gated allowance in `source_read_native.rs` (byte-identical blob `787a6842…`). The new module contains no `unsafe`, no native import, no FFI declaration and no pointer arithmetic; the unsafe boundary checker reports 8/8.
* **Architecture unchanged.** Dependency direction is still strictly inward: the application layer gained only std-only orchestration, and the adapter composes its own accepted sub-modules. No application→adapter or domain→application edge exists and the workspace-graph suite is green.
* **Attack surface added:** one pure function and its types in the inner crate, and one composition function in the outer adapter, both reachable only through explicit caller-supplied manifest bytes and an explicitly adopted content-root capability. No new parser, no new file access path, no new external input channel, no network, no secrets, and no executable authored content.
* **Applicability note.** Browser, HTTP, accessibility, performance and public projection are not applicable to this internal source-stage order because no user-facing surface or delivery exists here; that is not a waiver for any code-borne defect, and none was found.

## 9. Limitations and non-claims

* **Source-stage, not publishable.** The candidate proves that every declared representation's selected body was read under its own accepted guarantee and validated by the accepted selected-body preflight. It performs **none** of the deferred Stage 03–05 work: no full Markdown/GFM, no twenty-two-module body validation, no link or asset validation, no route, locale, property or visibility resolution, no rendering, and nothing of §6.2K sealing. R02.B's source stage is complete; the publishable candidate is not built and is not claimed.
* **Not a global snapshot.** Each body is read under its own per-file guarantee. A concurrent writer can still change one source after its own read, and a change that leaves identity, length, link count and both timestamps identical remains undetectable. `SOURCE_READ_PROVES_GLOBAL_SNAPSHOT` and `SOURCE_READ_GRANTS_PUBLICATION_AUTHORITY` are literal `false` on the aggregate, on each member and on the adapter result.
* **The FNV fingerprint is a linkage aid** over exactly the validated bytes. It is not a cryptographic digest and is never used to decide safety.
* **Duplicate-lineage refusal is defence in depth.** The accepted strict parser already refuses a duplicated locale, a duplicated scoped id and a duplicated claim, so `DuplicateLineage` cannot be reached through the public path today; it exists so a future parser change cannot silently produce two members for one representation. `SelectionDisagreement` is likewise an invariant guard, not a reachable policy path.
* **Host coverage is host-executed, not universal.** Linux clone-capable filesystems and this Windows 2025 share/mapping behaviour are the proven shapes; other hosts must refuse rather than degrade, and that refusal behaviour is unchanged by this order.
* **No authority elsewhere.** Nothing here resolves a route, decides visibility, selects a locale, generates discovery data, writes an output, acts as a privacy control, or publishes anything. R02.C, Stage 02 exit, publication, release and deployment remain untouched and unissued.

## 10. Repository state, retained material and cleanup

At report time: clean `git status`, one checkout, one worktree, one remote `origin` with `HEAD` equal to `origin/main`, exactly four regular files in `.documents/WIP` with no subdirectory, and the optional security-test path `crates/solo-adapters/tests/source_read_security.rs` still **absent**. Both authority flags read `false`.

**Task-owned temporary material.** Local iteration in this order required a toolchain that the previous closure had removed, so `.temp/toolchain` (toolchain `1.99.1` for both the default and the GNU host, ~2.68 GB), `.temp/toolchain-installer` (rustup and mingw archives, ~276 MB) and `.temp/target` (~639 MB) now exist as **task-owned** caches. All three are disposable and are retained only for the immediately following closure or audit work, with the standing delete-on-accepted-closure trigger: they must be removed, without touching anything else, when the Owner-authorized closure runs. `.temp/architecture-fixtures`, `.temp/rt`, `.temp/w032` and `.temp/w033` remain the pre-existing near-empty directories **not created by this order** and were preserved untouched. Every test fixture is created with an exclusive `create_dir` under the runner's task temp (or the verified Linux snapshot filesystem), guarded by an owner-first RAII value, and removed on drop: no `solo-source-set-*` directory survives a run, and no destructive pre-clean exists in the new file. Ten `proceed-while-running-*.log` files remain in Cline's own shared IDE temp folder, outside the repository, not provably this order's alone and partly open by running sessions; they are retained with the trigger "Cline rotation or Owner clearing". `git status` proves only that tracked files are clean; it is not claimed as proof of local temp cleanup.

## 11. Requested next step

`COMPLETE`. The Owner relays **`agent report ready`**, and the Chief Architect independently audits this report at its exact GitHub revision, the source diffs at `b17ca41`, the measured B1–B11 evidence and the raw two-host logs before issuing `ACCEPT / REVISE / STOP`. R02.B is **not** administratively closed, ROADMAP still lists it, and R02.C remains in ROADMAP but is unissued and unstarted. Only after an independent technical `ACCEPT` may the four-commit closure sequence of R01 §8.4 run — ROADMAP bullet removal, one R02.C NEXT candidate, ACTIVE neutralization to `ea5f20b7…`, and the reconciled closeout report — each in its own fully green two-host commit, followed by a separate final administrative audit. No closure write, no R02.C work, no successor order, no publication, release or deployment was performed or is implied by this report.
