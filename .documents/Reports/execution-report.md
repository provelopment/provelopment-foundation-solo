# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-072 R01` — R03.C3: implement the trusted-parent AVIF worker port, bounded protocol and all-or-refused envelope verification (nonpublishing first code slice).
**Matching handoff:** `SOLO-WO-072 R01`. **Report date (ISO):** 2026-11-12 (run window 2026-11-12T06:38–07:25Z).

**Outcome: `WORKER-CONTRACT-PROVEN` for this nonpublishing application/protocol slice only.** The inward worker contract, the parent-side fail-closed envelope validator, the closed V1 JSON parser and 23 new deterministic tests are implemented, executing and green on both hosts, and every existing gate still passes. This is **not** `MEDIA-VERIFIED`: no decoder was selected or imported, no worker executable or OS supervisor exists, no untrusted media was ever executed, no pixel, colour, orientation, MIME/magic or AVIF conformance claim is made, and `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

## 1. Anchors, commits and CI

| Item | Value |
| --- | --- |
| Baseline `main == origin/main == HEAD` | `ee412ba69070ad0c01c747c993d99a805bc3d948`, tree `65bd1b0ad92e114bbfa872e11d784c7fb041b51f` |
| Baseline blobs | report `279bc8cefdb4ac33a6d47f31b971377900635ce0`; Architecture `88b16fd91ffe4618801b4f956340674c27c7e047`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; prior ACTIVE `ced30fa66dd08dc7f98cfb59d45750e252dec3f8` |
| Installed ACTIVE | `c307e6e35100263e6716ae1150576e0ba42bfb9f` — `OWNER-ISSUED — SOLO-WO-072 R01`, handoff matches |
| 1. Issuance | `f973acb319af4cdd602c3704bc6fcd14016de59b` · tree `a87d95cefc0e178e85057ddd7da82cc6b41ea73c` · parent `ee412ba6…` · ACTIVE blob `c307e6e3…` · one path |
| 2. Implementation | `832cb7b4dde7b3964bea52a453a4bee864bc61c3` · tree `d18b39dbf386fe87d574c0a558e57a6a57469cef` · parent `f973acb3…` · the six authorized paths |
| 3. Correction A | `ff711b8fea35ce6ee1465456957bb79a25431e52` · tree `ea5f41decae3c70719cfee9464b40f482072fa35` · parent `832cb7b4…` · `crates/solo-adapters/src/media_worker_protocol.rs` only |
| 4. Correction B | `17e4d74758a177c14cdc42ea5df5b40b92c6a132` · tree `b4622b16a3a06c92fa12a710777a0e3d5f0517b2` · parent `ff711b8f…` · protocol parser + application test file |
| 5. Report | this file only, parent `17e4d747…` (final SHA given in chat, never embedded here) |

CI (workflow `rust-verify`, both hosts required):

| Revision | Run | Result |
| --- | --- | --- |
| Issuance `f973acb3` | `43170884027` — jobs `131023629386` (ubuntu-24.04), `131023629638` (windows-2025) | **success**, 33 steps, 0 failures |
| Implementation `832cb7b4` | `43172336765` | **failure** — step 16 *Strict lint* on both hosts: `serde_json::Error::message()` is not a public accessor (E0599) |
| Correction A `ff711b8f` | `43173093172` | **failure** — step 17 *Test*: 3 contract tests and 2 architecture-checker tests failed (fixtures and a lowercase path root; see §5) |
| Correction B `17e4d747` | `43173985800` — jobs `131033681754` (ubuntu-24.04), `131033681451` (windows-2025) | **success**, 33 steps, 0 failures; **431 passed / 0 failed** (ubuntu), **435 passed / 0 failed** (windows); `security_gate=pass` RustSec db `bebcc0944133af1658304d1edc0a17b3d4395518` |

## 2. What was built (eight-path operation ledger)

Exactly eight permanent paths were touched across the whole run — the six implementation paths, the ACTIVE order and this report. Nothing else in the repository changed.

| Path | Change |
| --- | --- |
| `.documents/WIP/workorder-active.md` | staged unchanged and committed for issuance only |
| `crates/solo-application/src/media_worker.rs` | **new**, 1,555 lines — the inward worker contract, refusals, pure-Rust SHA-256 and the parent validator |
| `crates/solo-application/src/lib.rs` | modified only to declare and re-export the module |
| `crates/solo-application/tests/media_worker.rs` | **new**, 968 lines — 11 deterministic contract tests |
| `crates/solo-adapters/src/media_worker_protocol.rs` | **new**, 573 lines — the strict closed V1 envelope parser |
| `crates/solo-adapters/src/lib.rs` | modified only to declare, re-export and document the parser's narrow boundary |
| `crates/solo-adapters/tests/media_worker_protocol.rs` | **new**, 368 lines — 12 protocol and refusal tests |
| `.documents/Reports/execution-report.md` | overwritten once, this run |

**Application contract.** `MEDIA_WORKER_PROTOCOL_VERSION_V1 = 1`, the 20 MiB source bound, the 64 KiB envelope bound, the 256 MiB defensive decoded-output bound, the 512 MiB / 10 CPU-second / 15,000 ms reference profile, and `GRANTS_PUBLICATION_AUTHORITY = false` / `PROVES_MEDIA_CORRECTNESS = false`. `MediaWorkerRequestV1` borrows only already-acquired bytes, measures their exact length, hashes them in the parent — and refuses zero bytes and more than 20 MiB **before** the port can be called. `MediaWorkerPortV1` has one method, `inspect_avif`, with no way to run an arbitrary executable. `MediaWorkerRunV1` separates a completed host observation from a typed transport refusal, so an abort, timeout or unreachable worker can never read as a normal exit. `MediaWorkerEnvelopeV1` is an explicitly untrusted claim with no validity flag; only `MediaWorkerCheckedEnvelopeV1` results, and its `grants_publication_authority()` and `proves_media_correctness()` are constant `false`. Fifteen stable refusal codes cover the whole closed set, and a refusal carries only the declaring owner, role and local index — never a path, byte, digest or worker text.

**Validation order (fixed, deterministic).** Transport refusal → limits verified and exactly the reference profile → normal exit → wall deadline → descendants reaped → no leak → protocol version → format → source length and digest against the parent's own → geometry (1–8,192 per side, ≤ 32,000,000 pixels, overflow-checked) → bit depth 8/10/12 → colour/plane-count and alpha agreement → ordinal sequence → `stride × rows` equality → **claimed total ≤ 256 MiB before any byte is read** → received plane count and lengths → per-plane SHA-256 → aggregate SHA-256. Any single failure discards every earlier checked envelope.

**Aggregate digest.** The documented V1 encoding `ordinal:u32 LE || stride:u64 LE || rows:u64 LE || length:u64 LE || raw plane bytes`, hashed as one stream by an incremental pure-Rust SHA-256 (no crate, no `unsafe`), so no second copy of up to 256 MiB is materialised.

**Protocol parser.** One public entry point, `parse_media_worker_envelope_v1`. It refuses empty and oversize input before parsing, then one complete UTF-8 JSON object with the closed key set at both levels, duplicate rejection at both levels, no floats, negatives, overflow, aliases, comments or trailing document, and a bounded 1–4 plane list. It has no I/O, no child process, no `serde_json::Value` shortcut and no unsafe allowance.

## 3. Test matrix (23 new tests, all executing on both hosts)

| Suite | Tests | Coverage |
| --- | --- | --- |
| `solo-application` / `media_worker` | 11 | positive structure with both booleans false and exactly one port call; source gate (0 bytes, 20 MiB, 20 MiB+1, zero calls vs one call); whole inventory with a failing internal obligation (one refusal, no partials, two calls); mixed AVIF/PNG (format refusal, one call / zero calls); six host-fact refusals; thirteen single-defect adversarial envelopes by category; claimed-output oversize before bytes; transposed orientation kept as a claim yet still length-checked; short received buffer; mutated plane byte; NIST vectors (empty, `abc`, one million `a`); strict lowercase-hex parsing; exact aggregate encoding |
| `solo-adapters` / `media_worker_protocol` | 12 | full accepted object field by field; size boundary (empty, exactly 65,536 accepted, 65,537 refused); duplicate keys at both levels; unknown keys at both levels; trailing documents; invalid UTF-8 and malformed JSON; non-object roots; wrong version/status/format/MIME/colour/orientation tokens; malformed digests; negative, floating, overflowing and coerced numbers; empty, surplus and four-plane lists; missing root and plane keys |

Every hostile case asserts an exact typed category, never `is_err()` alone. Existing suites continued to pass unchanged: `source-read`, graphic inventory and asset-read, bootstrap, manifest and property suites, the architecture checker with its refusal fixtures, the unsafe-boundary guard and the CI tripwire.

## 4. Version, graph and policy checks

Four crates (`solo-domain`, `solo-application`, `solo-adapters`, `solo-cli`) with unchanged names; `Cargo.toml` at the root and in both touched crates, `Cargo.lock` (`2f9e5357…`), `deny.toml`, `rust-toolchain.toml` and `.github/workflows/rust-verify.yml` are byte-identical to the baseline commit — no dependency, lockfile, toolchain, licence, policy or workflow change. `cargo fmt --all -- --check` is clean on the whole workspace. Clippy with `-D warnings` over the whole workspace (all targets, all features) passed on both hosts. No `unsafe`, `std::process`, `Command`, file, network, codec or decoder symbol appears in any of the four new files; the sole unsafe allowance remains the Windows-gated `source_read_native.rs`.

## 5. Every failure, refusal and workaround, recorded honestly

1. **Dead terminal in the previous attempt (not this run).** The earlier attempt of this order stopped before any operation when the shell process had exited; it produced no file, commit, push or CI change. This run started from a fresh, verified terminal and re-proved the anchors from scratch.
2. **No local compiler for the adapter crate.** This host has the pinned `1.99.1-x86_64-pc-windows-msvc` toolchain but no MSVC linker (`link.exe`) and no Visual Studio Build Tools, so any target that must *link* — including `serde`'s build scripts — cannot be built locally; installing a toolchain or linker is outside this order's permissions. `cargo check`, `cargo clippy` and `cargo fmt` were therefore run locally for `solo-application`, and full compile, lint, test and architecture execution evidence for both crates comes from the **mandatory two-host CI**, which runs exactly `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` and `cargo test --workspace --all-targets --locked`.
3. **Implementation `832cb7b4` failed strict lint on both hosts.** `serde_json::Error::message()` does not exist in `serde_json 1.0.154`. Correction A formats the pinned error through its `Display` implementation and still matches only this parser's own fixed prefixes. One authorized file changed; no bound, category or test was altered.
4. **Correction A `ff711b8f` failed the test step on both hosts**, for three genuine defects, all fixed inside the authorized paths by Correction B:
   - the contract tests declared a two-value array under `graphic.logo`, whose effective cardinality is one, so the property candidate was correctly refused before any worker ran — the tests now use `graphic.background`, the one graphic role that admits four layers;
   - one host-fact case intended to report a *wrong* memory profile reported `512 MiB`, which is exactly the reference value, so the observation was legitimately accepted and the test's own expectation failed — it now reports `256 MiB`;
   - the parser aliased `serde::de` as the lowercase `de` and called `de::Error::…`, which the source-boundary checker refuses as an unresolved path root; it now imports `serde::de::Error as DeError`. The security condition and every emitted message are unchanged.
5. **No other workaround was used.** No bound was relaxed, no test skipped or weakened, no gate bypassed, no refusal mapped to success, and the architecture checker was never disabled.

**Required disclosure — one mechanism substitution, flagged for Architect review.** The order's §5B prescribes `#[derive(Deserialize)]` structs with `#[serde(deny_unknown_fields)]`. `solo-adapters` depends on `serde` **without** its `derive` feature (the accepted manifest parser in the same crate hand-implements `Deserialize` with visitors), and enabling it would add a `serde_derive` dependency edge, edit `Cargo.toml` and change `Cargo.lock` — three things this same order explicitly excludes and lists as STOP conditions. The parser is therefore hand-written with explicit visitors, which reproduces and in places strengthens every property §5B requires: closed keys at every level, duplicate rejection at every level, no floats/negatives/overflow/aliases/comments, no `Value` shortcut, no silent normalisation, one complete document, and a 65,536-byte ceiling. The order's own direction that "the baseline code … controls this order" supports this choice; a literal derive implementation would need a separate Owner-authorized dependency change.

## 6. Preserved state, cleanup and next action

Preserved: one Entity and the EFS hierarchy; typed Entity/Facet/Site/Page/representation ownership; manifest-only authority; `GraphicAssetSourceSetV1` from the confined `assets/` reader as the sole input to this boundary; the PNG/JPEG/WebP/AVIF set unchanged; 20 MiB / 1–8,192 / 32,000,000-pixel ceilings unchanged; no fallback; no visibility filtering of declared invalid assets; strict inward Ports & Adapters with the new port owned by `solo-application` and implemented by no product adapter yet. Every result exposes constant `false` for publication authority and media correctness, and the module documents that a fake `true` host flag is never an operating-system attestation.

Not claimed: any decoded result, pixel or colour correctness, orientation correctness (the `kimono_rotate90` transposition remains unverified), MIME/magic agreement, AVIF or AV1 conformance, licensing, real host isolation, release, deployment, provider compatibility or closure of `R03.C3`/`R03.C`/Stage 03. The two-host CI is a regression and document-integrity gate for this slice, not media evidence.

Cleanup: only task-owned report fragments were created under Solo `.temp/` and were removed; `.temp/` again holds exactly the two inherited Owner archives (`mingw.zip`, `rustup-init.exe`, 2 files / 289,253,806 bytes) which were not touched, and `target/` was left alone. Final state: `main == origin/main`, empty `git status --porcelain`, one worktree, one remote, four WIP files, four branches (`main`, `solo-wo-061-r01`, `solo-wo-069-r01`, `solo-wo-070-r01`) unchanged.

**Next smallest action** (Owner/Architect — not taken here): `ACCEPT`, `REVISE` or `STOP` this exact revision. On `ACCEPT`, the next work is a separately Owner-issued order for the real Linux/Windows process supervisor in a separately authorized OS boundary — including an explicit Owner decision on any new Windows first-party `unsafe` allowance — followed by real codec, conformance and host-security testing and whole-candidate integration. No such work is authorized here.
