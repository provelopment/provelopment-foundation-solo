# Execution Report

**Work order:** `SOLO-WO-113 R02` — complete real four-format GIF V2 inspection and deliberate-red proof.
**Date:** 2026-11-14. **Bounded outcome:** `R03C-GIF-MIXED-V2-WHOLE-OWNER-AND-RED-SENSITIVITY-VERIFIED`.
**status=COMPLETE — pending independent Chief Master Architect review.**

Both hosts passed on the **first attempt** of run [`43602999853`](https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43602999853) at technical head `b0f204a2b2de470f1b0cfe8ef3631fcd51fac72d`: Ubuntu job `132471572016`, Windows job `132471571758`, every step success. The technical branch is **unmerged**; `main` carries no GIF source, no V2 code and no corrected N102 code.

## 1. Authority, preflight and the R01 STOP

`SOLO-WO-113 R01` stopped because the frozen domain registry admitted only `png|jpg|jpeg|webp|avif` (`crates/solo-domain/src/property_registry.rs:623`), so no real authored GIF obligation could exist; that registry file was outside R01's closed ledger and this agent did not edit it. R02 authorized exactly that one-token correction, and the preserved branch `solo-wo-113-gif-v2-r01` at `79c4dccf` was resumed rather than restarted.

Preflight, verified before writing: `main == origin/main == 400004cd5895862c8470a598cfa60737b7cde305` (tree `25a0cef9`, report blob `caf66141`), preserved branch local and remote `79c4dccf` (tree `1be86f43`, parent `ee295aea`), 26 remote branches, one checkout, one `origin`, empty stash, exactly four WIP files, clean tree apart from the Owner-installed R02 ACTIVE, and every frozen governance blob (`AGENTS.md` `6eae1da8`, Architecture `f66096ec`, both SOPs, architecture checker `1c99d348`, unsafe checker `57d75796`, root `Cargo.toml` `14505be4`, toolchain `9ec55b3b`, ROADMAP `9d7a8622`, NEXT `92fa562b`, OUTLOOK `cb9c7a61`).

## 2. Exact commits

| Step | Commit | Tree | Parent | Note |
|---|---|---|---|---|
| A2 authority on `main` | `c595d27b14a9c4a5bbd36bd2ddd4f9d4b6eecf6a` | `6d21037e` | `400004cd` | ACTIVE only, blob `6f53f86e` |
| T0 authority on branch | `9b08dc6dfe9f42af8219a5cc0a0c00eefc4f6ece` | `ad5c4ea0` | `79c4dccf` | same `6f53f86e` bytes |
| C0 domain admission | `3b5a2498d4baa0693d64028bb4a1b1a03c8e04f9` | `af632693` | `9b08dc6d` | registry blob `889d0fa7` |
| C1 real positive path | `6152f6d8217402cca8205cef93154762d1054395` | `3926efdc` | `3b5a2498` | adapters `66d6ba7b`, smoke `f7f87303` |
| C2 negative + RED matrix | `36466f2144cc6405bb8100e5af93b82872d57558` | `b9546ac6` | `6152f6d8` | smoke `1aa18b3d` |
| D cross-host gate | `430c37eec215c1d8baebb9157c869e84021ef98f` | `ae28f805` | `36466f21` | workflow blob `db2bc14b` |
| C3 corrective | `b0f204a2b2de470f1b0cfe8ef3631fcd51fac72d` | `eca5f96c` | `430c37ee` | technical head under test |

Branch `solo-wo-113-gif-v2-r01` = `b0f204a2`, pushed and verified remotely. Every B0/import path is byte-identical to `ee295aea`'s accepted blobs — `Cargo.lock` `cf105167`, `crates/solo-adapters/Cargo.toml` `3610f06f`, `graphic_gif_inspection.rs` `785d08d0`, `media_worker_host.rs` `894bc622`, `media_worker_host_native.rs` `9a26e195`, `graph.rs` `dc6d2770` — verified 6/6 with zero drift. `Cargo.lock` still holds exactly 43 locked packages; no dependency, feature, toolchain or `deny.toml` change.

## 3. C0 — the one-token admission, and two disclosed stale-test corrections

`ACCEPTED_IMAGE_EXTENSIONS` is now `[&str; 6] = ["png", "jpg", "jpeg", "webp", "avif", "gif"]`. It admits an **authored reference grammar only**: `AssetReferenceV1::parse` keeps its portable-relative, lowercase-exact, namespace and traversal rules, `svg` stays absent, uppercase `GIF` stays refused, and `graphic.favicon` remains PNG-only through `validate_value_for_key`'s `is_png()` rule.

Three focused tests were added inside the authorized registry file: the lowercase GIF reference parses (and `avif` remains a valid reference); a GIF reaches `graphic.background` but a GIF favicon is refused with `FaviconNotPng` at `DeclaredOwnerPropertiesV1::new`; uppercase/SVG/unknown/unsafe references still refuse.

**Disclosure — two stale accepted assertions, both intrinsic to the authorized admission and both minimally corrected:**
1. `crates/solo-domain/tests/property_resolution.rs` asserted `"assets/logo.gif"` refuses with `UnsupportedAssetExtension`. That single assertion now asserts admission and adds the still-refused uppercase `assets/logo.GIF`; every other refusal in the test is untouched.
2. `crates/solo-adapters/tests/property_resolution.rs` (`r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested`) listed `media/logo.gif` among unsupported extensions. It now asserts only `media/logo.svg` there, plus a positive GIF case that resolves through the real manifest validation, real inventory collection and `grants_publication_authority() == false`, plus a `graphic.favicon` + `.gif` → `FaviconNotPng` case.

Both files lie outside the order's file enumeration; both changes are single-assertion corrections of the very rule the order authorized changing, are disclosed here rather than hidden, and leave every security refusal and every other case intact. This is the only place this run wrote outside the enumerated paths, and no production behaviour beyond the authorized one-token change was altered.

## 4. C1/C2 — the real four-format whole-owner proof and the four exact cases

`test_inventory_v1` was extended (inside the authorized adapter file) with a `Facet` owner, a `Representation` owner and the registered `background` / `social-preview` roles, and a sibling `test_inventory_suppressed_v1` emits `Inherit::Suppress` declarations. Both run the real `AssetReferenceV1::parse`, real `DeclaredOwnerPropertiesV1` validation, real whole-graph resolution and real inventory collection — never a fabricated inventory or a fake host result.

Four integration cases live in `mod n097` (the module that owns the real filesystem harness), named exactly as required:

| Case | What it proves |
|---|---|
| `n113_01_mixed_four_formats_real_host` | One Entity with **four distinct real EFS declaring owners** — Entity `graphic.favicon` PNG, Facet `graphic.logo` JPEG, Site `graphic.banner` WebP (`internal` exposure), Page `graphic.background` GIF — read through the real confined `assets/` reader and inspected by the real confined worker. Four typed results in inventory order, each with its own owner, `local_index 0`, `assets/…` logical path, exact parent source digest, canonical MIME, dimensions, real PID and no publication authority; the GIF row carries `GIF89a`, 1×1, 3 frames, delays 10/20/5 cs, cycle 350 ms, `Infinite`, disposal Keep/Background/Keep, composites RED/RED/clear, a distinct transcript digest and verified limits/cleanup. |
| `n113_02_hidden_obligation_refuses_whole_candidate` | Representation-owned and `internal` declarations are full obligations (four owners, representation owner distinct from Page); an invalid `internal` GIF **in the middle** returns exactly one attributed refusal (`port-refused`, owner, role, index 0) after discarding two earlier successes, with no partial array; a suppressing Page declaration is still inspected as its own obligation. |
| `n113_03_declared_format_and_favicon_refusals` | Real PNG-claimed-as-GIF, GIF-claimed-as-PNG and corrupt GIF each refuse `port-refused`; uppercase extension, SVG, unknown extension, GIF-favicon and WebP-favicon refuse at the real property-validation stage; `.avif` refuses `unsupported-format`; empty, 20 MiB+1 and absent sources refuse at the typed stage that enforces them. |
| `n113_04_gif_forgery_and_red_oracle_sensitivity` | Real 1×1 RED and BLUE GIFs through the real worker and through the whole-candidate V2 gate give exactly `34aaa746…` vs `b7d1b3a1…`, with distinct source and transcript digests and distinct PIDs; the two closed forged modes (`WrongSourceDigest`, `WrongTranscriptDigest`) refuse `source-digest-mismatch` / `transcript-digest-mismatch` through the same parent validator, and through the V2 adapter over a real acquired source set they refuse `port-refused` with no candidate. |

The RED oracle is load-bearing, proven by a controlled local red run:

```text
SOLO_N113_RED_ORACLE=wrong cargo test … n113_04  ->  exit 101
  panicked at …/media_worker_host_smoke.rs: the RED asset's independent pixel oracle
  left: MediaWorkerDigestV1([52,170,167,70,…])   = 34aaa746…
 right: MediaWorkerDigestV1([183,209,179,161,…]) = b7d1b3a1…
default cargo test … n113_04  ->  n113_red red=34aaa746… blue=b7d1b3a1… oracle=red
                                  n113_forged wrong_source=refused wrong_transcript=refused
```

The switch only ever selects the wrong *expected* digest; the committed workflow never sets it, the default expectation is the true RED oracle, and no tracked source was altered to manufacture the failure. The BLUE asset is a valid asset, not an invalid one.

## 5. D — the exact-count cross-host gate and its local sensitivity proof

Two new steps (`V2 four-format image inspection gate (Linux)`, `(Windows)`) were added after the existing snapshot-fixture setup. Each runs `cargo test -p solo-cli --test media_worker_host_smoke --locked n113_ -- --nocapture --test-threads=1`, asserts the four fully qualified `n097::n113_*` names, an execution count of exactly 4, `test result: ok. 4 passed; 0 failed; 0 ignored`, the absence of `FAILED`/`ignored`, and eight real evidence tokens. The eight N108 cases, the four N102 canaries, the 12-run Windows repeat, the N097 evidence step, source snapshot, security, architecture, unsafe, tripwire, advisory and reproducible-release gates are unchanged.

Gate sensitivity was proven against the **extracted** committed script with a disposable cargo shim (no committed byte changed): `good → exit 0`, `missing case → 1`, `FAILED → 1`, `ignored → 1`, duplicate/3-of-4 count → `1`.

## 6. CI evidence — both hosts, first attempt, exact SHA

Run `43602999853`, head `b0f204a2`, run attempt 1, conclusion `success`:

| Host | Job | Result |
|---|---|---|
| `ubuntu-24.04` | `132471572016` | success, every step |
| `windows-2025` | `132471571758` | success, every step |

* `n097::n113_01..04` — `4 passed; 0 failed; 0 ignored` on both hosts, inside both the workspace test step and the dedicated gate.
* N108 — `8 passed; 0 failed; 0 ignored` on both hosts.
* N102 — `4 passed` with `extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true`, and `N102 repeat 12/12 passed` / `N102 event-identity repeatability: 12/12 consecutive native Windows runs passed.`
* Cross-host deterministic digests identical, PIDs never compared: GIF source `dc425aea…b84f`, transcript `8c02ba6f…ac33`, RED `34aaa746…3423`, BLUE `b7d1b3a1…7c23`, clear `df3f6198…1119`. Observed child PIDs: Ubuntu `[5814, 5818, 5822, 5826]`, Windows `[7644, 4780, 3308, 5024]`.
* Security and quality: cargo-deny 0.20.2 (Linux and Windows, verified archive) with advisories/bans/licenses/sources ok and the unchanged 43-package locked graph; the advisory negative probe refused `time 0.3.46` under `RUSTSEC-2026-0009` with exit 1 while the workspace passed all four checks; strict Clippy, fmt, architecture checker (Linux and Windows incl. refusal self-test), unsafe boundary, test-failure tripwire, reproducible release build byte-for-byte and release CLI smoke all success.

**Warning review:** `warning[license-not-encountered]: license was not encountered` appears for the unused `Unicode-3.0` allowlist entry. It is a cargo-deny licence-list hygiene warning about an allowlist entry matching no shipped crate; it is not evidence of an affected shipped licence, it was not suppressed, `deny.toml` was not modified, and no new advisory, deprecation or vulnerability appeared.

## 7. First-attempt failure, kept for the record

Run `43602488897` at **D** `430c37ee` failed on attempt 1 on **both** hosts at the shared `Test (workspace, all targets, locked)` step: `r03c1_09_reference_grammar_is_unchanged_and_no_media_is_attested` still asserted `media/logo.gif` refuses. That is the second stale assertion disclosed in §3; it was not visible locally because the local workspace run aborts at the pre-existing btrfs `snapshot_proofs`. Corrective commit `b0f204a2` fixed exactly that assertion (adding the positive GIF case), passed the targeted local checks, and both hosts then passed on the first attempt of the re-run. The gate bytes themselves never needed a change.

## 8. Local-first evidence and honest limitations

Run in the single verified WSL Ubuntu 26.04 checkout: `cargo fmt --all -- --check` clean; `cargo check --workspace --all-targets --locked` clean; `cargo clippy --workspace --all-targets --locked -- -D warnings` zero findings; `cargo check --target x86_64-pc-windows-msvc` clean; `solo-domain --lib` 3 passed (all 8 domain test targets green, `property_resolution` 27 passed); `solo-application --lib` 15 passed including the five V2 validator tests; `solo-adapters --lib graphic_gif` 14 passed; `solo-adapters --lib n102_event` 3 passed; `solo-adapters --test property_resolution` 20 passed; `media_worker_host_smoke n108_` 8 passed; `unsafe_boundary` 22 passed; `check-architecture` `architecture=legal packages=4 targets=33 files=63 edges=70` (no graph adjustment needed); `check_architecture_fixtures` 35 passed.

**Limitations, disclosed rather than waived:** this host has no `mkfs.btrfs`, no passwordless sudo, no `SOLO_TEST_SOURCE_FS_ROOT` and no native Windows Rust toolchain, so the five `snapshot_proofs`, the twelve real-filesystem `n097`/`n113` cases and the remaining fixture-bound suites cannot execute locally; they refuse loudly with `SOLO_TEST_SOURCE_FS_ROOT must name the R03 snapshot fixture filesystem` (and a plain-directory substitute is refused `Unsupported` by the reader), which is a host-fixture limitation, not a product regression. Their real proof is the hosted Ubuntu btrfs and Windows runs above. The deliberate-RED capture is local and real; the full default-green run of `n113_04` is the hosted one.

## 9. Preservation, cleanup and nonclaims

`main` is A2 plus this report only, and still carries no GIF source, no V2 code and no corrected N102 code. The technical branch `solo-wo-113-gif-v2-r01` at `b0f204a2` keeps every checkpoint (T0 → C0 → C1 → C2 → D → C3) and is **unmerged**. All prior refs keep their SHAs (26 branches before this run, 26 now: the existing technical branch was reused, no branch was created or deleted); `main` advanced by A2 and this report. The four WIP files, both SOPs, `AGENTS.md`, Architecture, both checkers, root manifests, toolchain, `deny.toml`, `.gitignore`, provider/release/deploy definitions, `platform/assets` and every Owner-owned or ignored file are untouched. Task scratch lived only in Solo `.temp/wo113-r02/` (extracted gate, synthetic gate logs, red and default run captures) and was removed; nothing was tracked, no `git clean` ran, and no unknown material was deleted.

**Nonclaims:** this order does not integrate GIF/V2 or the corrected N102 code into `main`; it creates no route, renderer, public URL or published asset; it settles no AVIF codec, no safe true-vector SVG, no `platform/assets/` catalogue, no derived responsive outputs and no visibility/publication model; and it does **not** close `R03.C`, `R03.C3` or Stage 03. `GraphicImageInspectionSetV2::grants_publication_authority()` remains `false` and `proves_whole_publication()` remains `false`. The agent does not self-accept, merge, release, deploy or tag.

## 10. Independent ACCEPT criteria — self-assessment

| # | Criterion | State |
|---|---|---|
| 1 | R02 exact authority and branch-visible T0 | **PASS** — A2 `c595d27b`, T0 `9b08dc6d`, ACTIVE `6f53f86e` on both refs |
| 2 | One-line `.gif` admission, favicon PNG-only, syntax security intact | **PASS** — registry blob `889d0fa7`; 3 focused registry tests; disclosed stale-assertion corrections in §3 |
| 3 | Real validated four-format, four-distinct-EFS-owner inventory/source set | **PASS** — `n113_01` on both hosts, four owners, four real child PIDs |
| 4 | Typed inward V2, no V1 still regression, full animation, bounds, no authority | **PASS** — V1 suites unchanged and green; app `n113_01..05` validator tests |
| 5 | Hidden/internal/representation/suppressed atomic rejection, no fabricated source set | **PASS** — `n113_02` (three scenarios) on both hosts |
| 6 | Missing/mismatched/oversized/corrupt refusals, favicon and unsupported policy, digest guards | **PASS** — `n113_03` plus the inward validator tests |
| 7 | Forged worker modes on the same validator, plus real-host RED vs BLUE red→green | **PASS** — forged modes refuse through adapter and validator; local red capture §4; hosted green |
| 8 | Load-bearing exact-count four-case gate plus eight N108 and Windows N102 12/12 | **PASS** — gate sensitivity §5; N108 8/8; N102 4/4 and 12/12 |
| 9 | Real Ubuntu and Windows green at the exact SHA, security/architecture/unsafe/reproducibility, truthful warnings | **PASS** — run `43602999853` attempt 1; §6 |
| 10 | No unauthorized permanent writes, prior refs and Owner data preserved, one report, fresh green final main CI | **PASS with the §3 disclosure** — the only out-of-enumeration writes are the two single-assertion corrections of the rule R02 authorized changing; the final main run is reported in chat with its exact SHA because a report cannot embed its own commit |

`status=COMPLETE — pending independent Chief Master Architect review.`



