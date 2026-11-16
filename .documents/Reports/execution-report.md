# Execution Report

**Work order:** `SOLO-WO-118 R16` — complete the R15 N127 proof, adopt the Owner's Platform-first graphic cascade target, and account for all Foundation artwork.
**Required bounded outcome:** `R03C-R15-N127-POST-INSTALL-GREEN-FOUNDATION-142-ASSET-LEDGER-OWNER-GRAPHIC-CASCADE-TARGET-V1`.

```
status=COMPLETE
r15_two_host_n127_unblocked=true
owner_81_assets_approved=true
platform_library_source_installed=true
platform_source_read_verified=true
foundation_donor_asset_inventory_paths=142/142
foundation_canonical_assets_accounted=101/101
additional_assets_delivered=false
graphic_baseline_target_owner_approved=true
graphic_baseline_cascade_implemented=false
publication_authority=false
derived_output_publication=false
release_authority=false
r03c_open=true
stage03_open=true
```

**One evidence nuance, disclosed up front.** Ubuntu's dedicated N127 gate executed all four mutation probes and none was accepted — a rejection is exactly what keeps that step green, and the guard that would `exit 1` on an accepted forgery sits in the same executed script — but the Ubuntu step never prints the literal verdict line `n127_gate mutation=rejected`, because the R15-authored workflow step omits that final `echo` (`.github/workflows/rust-verify.yml:1450` carries `authentic=accepted`; only the Windows step at lines 1504/1514/1516 carries both verdicts). That file is **explicitly outside the R16 allowlist**, so it was not repaired here. Windows emits both verdicts explicitly. The substantive requirement — mutated evidence rejected on both hosts — is therefore met on both; only the Ubuntu label is missing, and the one-line addition is the first item of §9's next action.

## 1. What R16 proves

- The R15 product library's **two-host N127 proof now exists at one SHA**: four N127 cases 4/4 on Ubuntu and on Windows, **81 genuine confined SVG children per host**, the pinned aggregate, and an authentic/mutation gate on both hosts.
- All three technical CI jobs succeeded at `a8e770f7de22049b89a7f31e353090be24e69cf4`, attempt 1: Ubuntu `133149298969`, Windows `133149299183`, independent AVIF oracle `133149299224`.
- The two obsolete historical absence assertions are repaired by one shared, test-only predicate; **every other assertion in those bodies is unchanged**, and nothing else in the Rust tree moved.
- All **142** publicly available Foundation artwork paths at the pinned donor commit are accounted for: 101 canonical (81 approved + 20 pending), 41 mirrors/test fixtures, 125 distinct blobs, 13 duplicate groups, with raw SHA-256 for all 101 canonical files.
- The Owner's Platform-first cascade is recorded as an explicit **FUTURE TARGET, NOT IMPLEMENTED**, with the open schema questions listed rather than guessed.
- Nothing was published, derived, released, deployed, merged or closed; no product byte, catalogue, reader, decoder, workflow, dependency, lockfile or policy changed.

## 2. Transaction and exact changed paths

| Step | SHA | Validation |
|---|---|---|
| baseline | main R23 `40c8ee1aaf472435ec5c4a267994c7005b6fa62b`; technical T23.3 `8b340976836f86b79dea36be8c9db9d8576997dd` | one `origin`, one worktree, four WIP files, empty stash, both trees clean, 30 remote heads, no product tree on `main` |
| **A24** (main, ACTIVE only) | `31376f0b045d94ad34db90c15d86b46e962e573a`, tree `eafe94d34876b5e15825d5b482a9eb3dfdc45caa`, parent `40c8ee1a` | ACTIVE blob `82286af85c3a7b81fa0002b2bb1f58a53f657d78` identical in worktree, index, commit and `origin/main` |
| **T24.0** (branch, ACTIVE only) | `e2f13f39a32b6fe0a0b24a52e73a1e7ea257fff8`, parent `8b340976` | same blob; `git diff T23.3..T24.0` = ACTIVE only |
| **T24** (branch, the R16 changes) | `a8e770f7de22049b89a7f31e353090be24e69cf4`, tree `c7637e6be56fecbae68bc31dc86c9c5176d61c77`, parent `e2f13f39` | exactly four paths: `crates/solo-cli/tests/media_worker_host_smoke.rs` (+151/−13), `.documents/Architecture/ARCHITECTURE.md` (+15), two new report files (+3180 / +100) |
| **R24** (main, report only) | this commit (SHA in the Owner handoff) | parent `31376f0b`, exactly one path |

`git diff --name-only T24.0..T24 -- platform/assets .gitattributes .github` is **empty**: the 84 product files, the catalogue, the MIT notice, the provenance record, the owner decision, the donor fixtures and the attribute rule are byte-unchanged. No `.github/`, crate source, domain, dependency, lockfile or policy file was touched, and no `#[ignore]`, `continue-on-error`, `--skip` or output spoofing was introduced.

## 3. The two repaired assertions

Both obsolete loops (`n125_04_real_entity_reader_still_refuses_platform_and_no_unapproved_library` and `n126_02_pinned_mit_provenance_and_r12_catalogue_review_only`) previously required that `../../platform` and `../../platform/assets` **not exist** — a statement about the pre-install state. They now call one shared, test-only predicate, `assert_absent_or_exact_approved_platform_library`, which accepts only two states:

- **not installed** — neither `platform/` nor `platform/assets/` exists (a *half-created* tree is refused); or
- **installed** — the enumerated relative path set under `platform/assets/` equals **exactly** the approved catalogue's 81 identifiers plus `catalogue-1.0.0.json`, `licensing/TABLER-ICONS-MIT.txt` and `licensing/icon-provenance.json` (84 files), with no extra file, no unexpected directory and no symlink or reparse point; the installed catalogue is **byte-identical** to the approved candidate document and hashes to `ad8ea718…`; it re-parses at `1.0.0` with the same 81 identifiers in the same order; and both licensing documents keep their pinned SHA-256.

Every other assertion in both bodies is untouched: the Entity reader still proves `PlatformUnverified`, the R12 binding still proves `PlatformSourceUnavailable`, zero-alias-fallback and no-publication-authority stand, and `../../content/platform` must still be absent. Neither test was deleted, renamed, filtered or ignored.

**Local fast proof, executed before pushing** (temporary harness, removed before the commit; the committed file contains only the helper and the two call sites):

```text
t24_predicate_proof absent=accepted half=refused exact=accepted extra=refused \
  missing_catalogue=refused wrong_catalogue=refused alias=refused symlink=refused
```

An unauthorised extra product file, a missing pinned catalogue, a changed catalogue byte, a reinstated withheld alias filename and a symlinked leaf are all refused; the real approved tree and the historical absent state are both accepted.


## 4. N127 on both native hosts — the R15 gate, now executed

Technical run **`43822728379`**, attempt 1, at `a8e770f7de22049b89a7f31e353090be24e69cf4`.

| Job | ID | Conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `133149298969` | **success** |
| `verify (windows-2025)` | `133149299183` | **success** |
| `avif-reference-oracle` | `133149299224` | **success** |

Inside the workspace suite on both hosts: `test n127::n127_01 … ok`, `n127_02 … ok`, `n127_03 … ok`, `n127_04 … ok` (4/4). The dedicated gates then re-ran the four cases with `--nocapture` and emitted:

```text
n127_evidence case=01 approved=81 read=81 real_children=81 version=1.0.0 exact_sha=true \
  aggregate=3c34fe9919341862a5a70a87d205fbfc942741dec5f3996514d3edd0aeb1aaca \
  publication=false source_root=platform entity_fallback=false derived_output=false release_authority=false
n127_evidence case=02 refusals=21 pin_refused=true licence_refused=true provenance_refused=true \
  entity_fallback=false publication=false entity_source_reader_refuses_platform=true \
  platform_source_binding_refused=true
n127_evidence case=03 confinement=true hardlink_refused=true unsafe_refused=true \
  native_mechanism=linux-openat2-beneath-otmpfile-excl-ficlone-immutable-clone natives=11 publication=false
n127_evidence case=03 confinement=true hardlink_refused=true unsafe_refused=true \
  native_mechanism=windows-ntcreatefile-obj-dont-reparse-file-id-info natives=10 publication=false
n127_evidence case=04 late_refused=true zero_partial=true private_path_absent=true fallback=false \
  publication=false standalone_set_complete=true wrong_root_refused=true namespace_alias_refused=true
```

- **Ubuntu**: the clone path plus the tmpfs no-reflink refusal (`natives=11`), `n127_gate authentic=accepted`, and all four mutation probes executed and rejected.
- **Windows**: `natives=10` (the reparse-point refusal is exercised where the runner's privilege allows it and a privilege decline is recorded honestly), `n127_gate authentic=accepted` **and** `n127_gate mutation=rejected`.
- Each host launched **81 real confined SVG children** with the R14 aggregate and re-proved the 84-file product tree, the catalogue pin, the 21-scenario refusal matrix and the late-corruption zero-partial case on its own native filesystem.
- `N093`–`N126` gates all ran green in the same run, including `n125_gate mutation=rejected`, `n126_gate authentic=accepted` and `n126_gate mutation=rejected` on both hosts.
- Architecture measured `packages=4 targets=38 files=74 edges=93` on both hosts — unchanged from R15, as the ledger-only change implies.

## 5. The complete Foundation artwork ledger

`.documents/Reports/foundation-donor-full-artwork-inventory.json` — 142 source records, 184 479 bytes, SHA-256 `6d6199cf63f486a95c38395b449f2f68363d6da45cd1f18c49212d391b6a903e`; paired with `.documents/Reports/foundation-donor-full-artwork-and-cascade-decision.md`.

| Category | Paths | Rights state | Delivery |
|---|---:|---|---|
| Foundation icon library | 84 | 81 `owner-approved-exact-81`, 3 `review-required` | 81 installed; `icon-services.svg` and the two byte-identical aliases pending |
| Foundation role placeholders | 10 | `review-required` | not installed; no per-file licence record |
| Foundation platform marks | 7 | `third-party-conditional` | not installed; donor record is PARKED / not approved with P-1…P-4 open |
| EFS demonstration mirrors | 4 | `mirror-non-authoritative` | no delivery; Entity-owned demo identity |
| German spoke mirrors | 10 | `mirror-non-authoritative` | no delivery; duplicates of the canonical placeholders |
| Test fixtures | 27 | `test-only` | no delivery; never a runtime library |

Verified, not assumed: 142 media paths at the pinned commit; 101 canonical and 41 mirror/test; 125 distinct blobs with 13 duplicate groups; every one of the 81 approved donor raw SHA-256 values equals its catalogue declaration; exactly three donor icons are absent from the approved catalogue; every canonical file was fetched at the pinned commit, size-matched against the tree and **sniffed by magic bytes** (98 `image/svg+xml`, 2 `image/png`, 1 `image/jpeg`, zero unrecognised) rather than classified by filename. The 20 outstanding sources are enumerated individually in the decision report with their sizes, digests, role meaning and the exact condition that must be satisfied before any of them may be delivered. No pending byte was installed, and the approved `1.0.0` catalogue is unchanged.


## 6. Owner amendment recorded in the architecture

`.documents/Architecture/ARCHITECTURE.md` gains one clearly labelled subsection, **§6.2I.1 "Owner amendment — Platform-first graphic cascade (FUTURE TARGET, NOT IMPLEMENTED)"**, appended after the original §6.2I text and cross-referencing §3.3A and §6.2G. It states that the original "no automatic platform fallback" wording, §3.3A's zero-fallback rule and today's additive `property_resolution` **remain as coded**, and records the Owner's target semantics for the next order:

1. the version-pinned Platform library is a **root layer**, not an EFS node, a Facet, an Entity or a publisher; each default must name an exact role/slot and one approved catalogue ID, registration is explicit and deterministic, and an unregistered role has **no** default;
2. a descendant's explicit, validated **supersede/replace** for the same role/slot wins; the nearest valid superseding declaration prevails; conflicting same-owner selections still refuse; location, SEO and contact inheritance are untouched;
3. suppression and bad input are final — a missing, invalid, private/ineligible or wrong-class closer selection refuses the **whole candidate** and never resurrects a default; internal/unlisted omissions never cause implicit reappearance;
4. collections may append, supersede one **uniquely identified slot** or suppress-and-repopulate, never silently overwrite, and internal provenance records namespace, owner, slot, source SHA and supersession chain;
5. no automatic baseline is activated before the migration contract and a dedicated two-host suite prove the nine required cases.

Three questions are recorded as **open** rather than guessed: the registration schema (role-keyed table versus per-role catalogue fields), the slot-identifier grammar, and whether an entirely unregistered role inherits the baseline. `graphic_baseline_cascade_implemented=false` stands.

## 7. Verification proportionate to risk

The decision report carries the risk table the Owner asked for: byte-exact artwork import inside an accepted class gets cheap complete file/count/raw-hash/case/provenance validation plus one representative media check (no new 81× adversarial matrix merely because bytes were copied); a new external mark or placeholder additionally needs its provenance, published use conditions and clearance space; catalogue/alias/version changes need exact membership and tamper proof; and Platform root acquisition, isolation, cross-namespace resolution and the future cascade get focused adversarial two-host proofs with no waivers. Existing N093–N127 bodies and their authentic/mutation gates stayed intact apart from the two designated obsolete assertions; no gate was skipped, ignored or made advisory, and no test was deleted to make the suite cheaper.


## 8. Rights, authority and what remains closed

- `owner_81_assets_approved=true` still refers to exactly the 81 reviewed assets under the Tabler MIT terms; their notice and provenance remain installed beside the bytes and must accompany any separately authorized distribution. The Foundation root Apache-2.0 licence does not replace that notice and grants no rights in names, logos or trademarks.
- The 20 outstanding sources are **classified, not delivered**: `icon-services.svg` has no recorded third-party licence, the two aliases need a governed alias model, the ten placeholders need a role/licence decision, and the seven platform marks carry a donor record that says **PARKED — NOT INTEGRATED / Not approved** with preconditions P-1…P-4 and brand-owner use terms. No rights were invented, no mark was recoloured, no account or handle was assumed.
- `additional_assets_delivered=false`, `publication_authority=false`, `derived_output_publication=false`, `release_authority=false`, `graphic_baseline_cascade_implemented=false`, `r03c_open=true`, `stage03_open=true`. Nothing was published, derived into a public asset, released, deployed or merged; `R03.C` and Stage 03 remain open.

## 9. Repository state, cleanup and the next Owner action

- `main` = A24 `31376f0b` then R24 (report only); technical branch = T24 `a8e770f7`; both pushed, both clean, one worktree, one `origin`, four `.documents/WIP/` files, empty stash, `git status --porcelain` and `git diff --check` clean.
- **Observed remote heads: 30** — the 28 long-standing heads plus the independent Dependabot app heads (`dependabot/cargo/svgtypes-0.16.2`, `dependabot/cargo/syn-3.0.10`), which this agent did not create, merge or modify. No new branch, tag, PR or worktree was created.
- R15's red history is retained in §10. Task scratch (inventory and fetch scripts, transaction scripts) lived only in untracked `.temp/` and was deleted before handoff; the Owner's pre-existing `.temp/` entries were left untouched. Local N127 and other real-filesystem suites cannot run on this machine (no reflink-capable filesystem, no Windows linker): they are reported only from the hosted runs, never as local native success.

**Shortest next Owner action.** Three things, in this order:

1. **One-line workflow repair** (needs authorization; not in the R16 ledger): add `echo 'n127_gate mutation=rejected'` as the final line of the *Linux* N127 gate step so both hosts emit the explicit mutation verdict. Until then the Ubuntu rejection is proven by the step's success and the executed guards, not by a printed label.
2. **The rights decision for the 20 outstanding sources** — which may be redistributed, which stay excluded, and whether the two byte-identical aliases get a governed alias model. Recommended split: placeholders and any approved original artwork first (once their licence state is recorded), platform marks deferred until the named third-party decision resolves P-1…P-4.
3. **Authorization for the platform-first cascade order**, which settles the three open schema questions in §6 and implements the precedence algorithm with its own two-host suite; the architecture amendment is target direction and must not be read as behaviour that exists.

## 10. History retained

| Run | SHA | Outcome |
|---|---|---|
| `43815292648` | `bc277bf3` | R15 attempt 1: both verify jobs red (two protected assertions plus my own wrong catalogue-kind expectations), oracle green |
| `43816188208` | `0da55e28` | R15 attempt 2: two N127 expectation defects repaired, the two protected assertions still red |
| `43816931677` | `8b340976` | R15 attempt 3: only the two protected assertions red; N127 4/4 on both hosts but the dedicated gates never executed |
| `43817901578` | `40c8ee1a` | R23 main baseline: Ubuntu `133134040690`, Windows `133134041094`, both success |
| `43822728379` | `a8e770f7` | **R16 T24: Ubuntu `133149298969` success, Windows `133149299183` success, oracle `133149299224` success** |

The R14 acceptance (`43807461257`, main `43808522755`) and the original R13 Windows failure remain unchanged history. No earlier report, gate body or fixture byte was edited to improve any of these outcomes.

