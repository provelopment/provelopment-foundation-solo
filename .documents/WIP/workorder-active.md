# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-072 R01**  
**Title:** R03.C3 — implement the trusted-parent AVIF worker port, bounded protocol and all-or-refused envelope verification (nonpublishing first code slice)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching explicit Owner → Implementation Agent handoff:** `SOLO-WO-072 R01`  
**Execution authority:** This complete `OWNER-ISSUED` work order is the one operative instruction upon Owner installation in `.documents/WIP/workorder-active.md` and matching handoff. Perform its instructions without seeking per-command Git approval. The Chief Master Architect has **permanent read-only GitHub access**. Only the Implementation Agent performs the expressly authorized filesystem, Git and CI operations. This work order authorizes product **application/adapter contract code** but **no worker executable, native process launcher, codec, third-party dependency, publication, release or deployment**.

## 1. Approved Scope — exact objective and honest result

Implement a **real, compiling, tested, inward-owned worker request/response contract and fail-closed parent-side envelope validation** for the R03.C3 AVIF path. Connect it to the existing complete, confined Entity graphic-source set **without changing source acquisition or publication authority**. Parse a closed, strictly bounded worker metadata envelope in the outer adapter using the already-approved `serde` and `serde_json` dependencies. Independently re-check source-byte identity, reported geometry, per-plane lengths and decoded-output cryptographic hashes in the parent application use case, using a pure-Rust SHA-256 helper without adding a crate. An all-or-refused traversal discards all earlier results on one error. Prove the contract against deterministic in-memory fake worker outputs and malicious/incorrect envelopes on both CI hosts.

The successful outcome is **`WORKER-CONTRACT-PROVEN` for this nonpublishing application/protocol slice only**. It is not a production media verifier: the selected Linux `RLIMIT_AS`/`RLIMIT_CPU` and Windows Job Object launch mechanisms are already bound in Architecture §6A.8 but are **not** implemented by this work order. Fake host-enforcement fields do not attest operating-system controls. Do **not** import the experimental `oxideav` decoder, copy its proof scripts, execute untrusted media, add a subprocess to `solo-cli`, claim pixel correctness, produce public assets, or close `R03.C3`, `R03.C` or Stage 03.

**Preserve the Owner-accepted product:** one Entity; typed Entity/Facet/Site/Page/representation ownership; manifest-only authority; `GraphicAssetSourceSetV1` from the confined `assets/` reader; the PNG/JPEG/WebP/AVIF initial format set; 20 MiB source, dimensions 1–8192 and no more than 32,000,000 decoded pixels; no fallback; no visibility filtering of declared invalid assets; strict inward Ports & Adapters; no first-party `unsafe` outside the existing Windows-gated `source_read_native.rs` exception. Every result introduced here must expose a constant `GRANTS_PUBLICATION_AUTHORITY = false`, and media correctness must remain `UNVERIFIED`.

## 2. Immutable starting anchors and required reading

Verify the following **before the first non-Owner edit**:

| Anchor | Expected exact value |
| --- | --- |
| `main == origin/main == HEAD` | `ee412ba69070ad0c01c747c993d99a805bc3d948` |
| `HEAD^{tree}` | `65bd1b0ad92e114bbfa872e11d784c7fb041b51f` |
| Current canonical report blob | `279bc8cefdb4ac33a6d47f31b971377900635ce0` (`SOLO-WO-071 R01` complete) |
| Prior ACTIVE blob in `HEAD` | `ced30fa66dd08dc7f98cfb59d45750e252dec3f8` (`SOLO-WO-071 R01`) |
| Architecture blob | `88b16fd91ffe4618801b4f956340674c27c7e047` |
| NEXT blob | `92fa562b1af0c54391851aa5c79deb1631127069` |
| ROADMAP blob | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| OUTLOOK blob | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Historic branches, preserve unchanged | `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| Prior final CI | `rust-verify` run `43168436168`, success, both Ubuntu and Windows |

Read: `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2I, 6A.3, 6A.8 and especially the **new** AVIF boundary after `Asset pipeline`; both `.documents/SOP/` SOPs (Architect §§6A, 6B, 7, 7A, 7B, 8; Implementation §§4–5C); all four WIP files; `.documents/Reports/execution-report.md`; root `Cargo.toml`/`Cargo.lock`/`deny.toml`; `.github/workflows/rust-verify.yml`; and the actual current `solo-application`/`solo-adapters` public exports and `graphic_asset_read` application/tests. The retained `solo-wo-069-r01` proof's **pure-Rust SHA-256** and NIST-vector tests may be read as a **Solo-owned technical reference**; do not import the proof crate, merge its branch or copy unreviewed OS/decoder code. The baseline code/Architecture contract, not examples or external crates, controls this order.

## 3. Exhaustive permanent path and permission ledger

**Owner installation, on `main`:** `.documents/WIP/workorder-active.md` — Owner replaces the one live ACTIVE file with this exact order. Agent only stages/commits/pushes this file unchanged for issuance.

**Implementation, on `main`, exactly six paths:**

1. `crates/solo-application/src/media_worker.rs` — **create** pure-Rust inward port, typed request/response/refusals, SHA-256 and parent-side envelope validator plus all-or-refused source-set traversal.
2. `crates/solo-application/src/lib.rs` — edit only to declare/re-export the new module's public API.
3. `crates/solo-application/tests/media_worker.rs` — **create** deterministic fake-port, positive and adversarial contract tests.
4. `crates/solo-adapters/src/media_worker_protocol.rs` — **create** strict serde/JSON wire metadata parser and conversion into the inward-owned types; do not perform I/O or start a child.
5. `crates/solo-adapters/src/lib.rs` — edit only to declare/re-export the protocol parser, with comments consistent with the existing narrow unsafe allowance.
6. `crates/solo-adapters/tests/media_worker_protocol.rs` — **create** JSON protocol and refusal tests.

**Completion, on `main`:** overwrite the **one** `.documents/Reports/execution-report.md` and commit/push this file only. These are the **only eight permanent paths** authorized across issuance, implementation and report commits.

**Absolutely excluded:** every other `.documents/` path, other crate source/tests, `Cargo.toml` files, root/probe `Cargo.lock`, `deny.toml`, `.github/`, tools, scripts, policy, checkers, OS-native modules, branch refs, fixture files, installation content, `verification/`, `.temp/` archives, `README.md`, `AGENTS.md`, stage plans and platform artwork. No new files in WIP; WIP stays exactly the four named files.

**Git permissions:** Normal non-force commits/pushes to `origin/main` **only** in the exact three commits described in §§4/7; read back SHA/tree/parent/path/blob at each. No new branch, PR, merge, tag, release, deployment, GitHub administrative change, repository settings, force push, amend, history rewrite, reset, clean, destructive checkout, stash, second worktree/clone, or unrelated repo. Existing `origin` and its credentials may be used for authorized normal operations. No secrets, provider service, package addition, OS/native installation, external codec, new dependency, licence exception, security-policy weakening or network operation beyond existing pinned CI toolchain/dependency/scanner fetches. Task-owned scratch only under verified Solo `.temp/` or the Cline-provided task temp area; preserve unknown data and inherited archives; clean only your own files. `target/` cache is not a deliverable and must not be deleted.

## 4. Mandatory execution sequence

**4A — Preflight.** Use a working terminal; prove it executes a trivial command. At the sole Solo root inspect `git rev-parse --show-toplevel`, `git rev-parse HEAD`, `git rev-parse 'HEAD^{tree}'`, `git status --porcelain=v1 --untracked-files=all`, `git worktree list`, `git remote -v`, `git branch -avv`, local/remote refs and `git rev-parse HEAD:<path>` for every anchor. There must be exactly one checkout/worktree and one remote `origin`; `main` must equal `origin/main` at the stated SHA. The **only** initial local modification must be Owner-installed `.documents/WIP/workorder-active.md`. Verify installed bytes say `OWNER-ISSUED — SOLO-WO-072 R01`, with matching explicit Owner handoff. STOP safely without edits for any mismatch, unknown dirty/staged/untracked material, no working terminal, missing approval/handoff, unexpected branch or unknown data.

**4B — Issue ACTIVE.** Stage exactly `.documents/WIP/workorder-active.md`, commit `Issue SOLO-WO-072 R01: AVIF worker contract`, push to `origin/main`, read back exact SHA/tree/parent/ACTIVE blob, and verify every other file unchanged. Require completed successful pre-existing `rust-verify` on both `ubuntu-24.04` and `windows-2025` at **that exact issuance SHA** before product-code work. Do not infer successful CI from a prior revision.

**4C — Implement the six-path code slice** using the fixed contract and tests in §§5–6, no dependency or lockfile changes. Commit the six paths only with subject `feat: add nonpublishing AVIF worker envelope boundary (SOLO-WO-072)`; push, verify exact parent/tree/path ledger and source blobs. Require green existing `rust-verify` on both hosts at **that implementation SHA**; verify the new tests executed. Do not mask failures or skip the architecture checker.

**4D — Independently inspect the actual code diff**, `git diff --check`, `git diff --name-status` relative to the issuance SHA, Cargo graph and guard results, unsafe/import scan, test output and CI job steps. Ensure all existing source-read, graphic and bootstrap suites still pass. Check that no test/helper output is a published asset.

**4E — Publish the canonical report** on clean `main`, overwrite `.documents/Reports/execution-report.md`, stage and commit only that file with `Report: SOLO-WO-072 R01 worker contract evidence`, push and read back exact remote SHA/tree/parent/report blob. Require successful both-host `rust-verify` at **final report HEAD**. Report final SHA in chat, not inside the report itself. Do not self-ACCEPT or proceed to another order.

## 5. Binding code contract — no interface design left to guess

### 5A. Application-owned contract (`solo-application/src/media_worker.rs`)

Provide a public, versioned, nonpublishing API with these **named** elements (derive `Debug`, `Clone`, `PartialEq`, `Eq` where valid, with no OS/native types):

- `MEDIA_WORKER_PROTOCOL_VERSION_V1: u32 = 1`; `MEDIA_WORKER_MAX_SOURCE_BYTES_V1: u64 = 20 * 1024 * 1024`; `MEDIA_WORKER_MAX_ENVELOPE_BYTES_V1: usize = 64 * 1024`; `MEDIA_WORKER_MAX_DECODED_BYTES_V1: u64 = 256 * 1024 * 1024` (an **internal defensive envelope bound**, not a new permitted image format or released resource promise); `MEDIA_WORKER_REFERENCE_MEMORY_BYTES_V1: u64 = 536_870_912`; `MEDIA_WORKER_REFERENCE_CPU_SECONDS_V1: u64 = 10`; `MEDIA_WORKER_REFERENCE_WALL_MS_V1: u64 = 15_000`; `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1: bool = false`; `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1: bool = false`.
- `MediaWorkerRequestV1<'a>`: only the borrowed already-acquired source bytes, byte length, SHA-256 content ID and the fixed reference limits. **No path, authored URL, Entity identifier, route, visibility decision, filesystem capability or OS handle is sent to a worker**. Create it in the parent from `GraphicAssetSourceEvidenceV1::bytes()`; reject zero bytes and more than 20 MiB **before** calling the port.
- `MediaWorkerPortV1`: one `fn inspect_avif(&self, request: &MediaWorkerRequestV1<'_>) -> MediaWorkerRunV1`. This is an **outbound port** whose eventual adapter must own process execution; the first implementation uses only deterministic fakes. It must not offer a raw `run arbitrary executable` function.
- `MediaWorkerRunV1`: a typed, parent-visible **host observation** (launch limits verified, normal exit status, wall deadline met, child tree reaped, no leak, fixed effective limits, independently owned plane bytes) **separate** from an untrusted `MediaWorkerEnvelopeV1`. A positive result must require all positive host observations. Define typed variants for transport refusal rather than mapping aborts/timeouts to normal exit. The existence of a Rust constructor or a fake `true` bit is **not** an OS attestation and must not be documented as one.
- `MediaWorkerEnvelopeV1`: a decoded *metadata claim* with protocol version, exact status `decoded`, exact format token `avif`, exact MIME `image/avif`, source byte count and lowercase 64-hex SHA-256, width, height, bit depth, color model, alpha/grid/sequence/orientation facts, one to four numbered planes and a lowercase 64-hex overall decoded-output SHA-256. Each `MediaWorkerPlaneClaimV1` has ordinal, stride, rows, length and lowercase SHA-256. Keep every constructor/test path subject to the same application validator; no unvalidated success marker.
- `MediaWorkerRefusalKindV1` with **stable codes** for at least `SOURCE_EMPTY`, `SOURCE_OVERSIZE`, `WORKER_UNAVAILABLE`, `WORKER_LIMIT_UNVERIFIED`, `WORKER_EXIT_ABNORMAL`, `WORKER_WALL_TIMEOUT`, `WORKER_DESCENDANTS_REMAIN`, `WORKER_PROTOCOL_INVALID`, `WORKER_SOURCE_MISMATCH`, `WORKER_DIMENSIONS_INVALID`, `WORKER_PLANE_INVALID`, `WORKER_DIGEST_MISMATCH`, `WORKER_OUTPUT_OVERSIZE`, `WORKER_FORMAT_UNSUPPORTED`, `WORKER_MEDIA_UNVERIFIED`. A refusal may carry the authored obligation's typed owner/role/index for internal diagnostics, but **never** raw private bytes, absolute filesystem paths or untrusted text as a public error.
- `MediaWorkerBoundaryOutcomeV1` with `EnvelopeChecked(Vec<MediaWorkerCheckedEnvelopeV1>)` or `Refused(...)`. The `CheckedEnvelope` type is an **internal structural/provenance check only**. Its methods `grants_publication_authority()` and `proves_media_correctness()` must both return `false` and cannot be overridden. It must never be confused with `GraphicAssetSourceSetV1` or a future sealable publication candidate.
- `inspect_media_worker_source_set_v1(&GraphicAssetSourceSetV1, &impl MediaWorkerPortV1) -> MediaWorkerBoundaryOutcomeV1`: iterate **every** acquired authored obligation in existing inventory order, including unlisted/internal/suppressed. For this AVIF-specific slice, call the worker only for a `.avif` source path; any other image format or unsupported token returns `WORKER_FORMAT_UNSUPPORTED` for the **entire** attempt, not a partial claim that four formats are supported. If any entry fails, drop all prior checked envelopes and return the first stable typed refusal. Empty source set may return an empty `EnvelopeChecked` but cannot assert media verification or publication.

**Source SHA and plane rehash:** Implement a **pure Rust SHA-256** routine in the same application module with zero third-party imports. Inspect the accepted Solo-owned SHA routine in `verification/avif-r03-c3/src/lib.rs` on `solo-wo-069-r01` for the hashing rounds and NIST/FIPS test vectors, then implement it inside the authorized application module; do not create an extra module, dependency or new first-party `unsafe`. Check independently against known SHA-256 vectors (empty bytes, `abc`, one million `a`), and on both CI hosts compare at least `abc` with the host SHA-256 tool in the test or verified job evidence. SHA-256 is over **the actual raw source/plane bytes**, not a printed metadata string or a non-cryptographic source fingerprint. Never trust a hash supplied by the worker without rehashing the corresponding owned bytes in the parent.

For the aggregate plane digest, fix this unambiguous V1 encoding: concatenate each plane in ordinal order as **`ordinal:u32 LE || stride:u64 LE || rows:u64 LE || length:u64 LE || raw plane bytes`**, then SHA-256 that concatenation. Reject duplicate, missing, out-of-order ordinals and unsigned-integer overflow. A claim's `length` must equal the actual received plane bytes length **and** `stride.checked_mul(rows)`; reject zero stride/rows/length, length overflow, total output over 256 MiB, zero/unsupported bit depth (accept the *claimed* 8, 10 or 12 but do not infer decode conformance), one dimension above 8192, zero dimensions, or width × height greater than 32,000,000. Do **not** assume all plane strides match the displayed width or that `kimono_rotate90` is already correct; plane orientation is explicitly unverified. The input source SHA and byte count must match those calculated before the worker call. Reject inconsistent alpha/color/plane counts (gray 1; yuv 3; yuva 4; `has_alpha` iff `yuva`). Accepting an internally consistent fake is **not** proof of valid image pixels, color semantics, MIME/magic, orientation or AVIF specification compliance.

Code pattern for the parent validator (adjust only to the named types, not the security condition):

```rust
let byte_count = u64::try_from(source.len()).map_err(|_| MediaWorkerRefusalKindV1::SourceOversize)?;
if byte_count == 0 { return Err(MediaWorkerRefusalKindV1::SourceEmpty); }
if byte_count > MEDIA_WORKER_MAX_SOURCE_BYTES_V1 { return Err(MediaWorkerRefusalKindV1::SourceOversize); }
let source_sha256 = sha256(source); // the parent hashes these exact trusted source bytes
// Invoke the typed port only now. Re-check the envelope, raw plane bytes, limits,
// normal exit, deadline, source ID, geometry and each SHA-256 before retaining it.
let pixel_count = u64::from(width)
    .checked_mul(u64::from(height))
    .ok_or(MediaWorkerRefusalKindV1::WorkerDimensionsInvalid)?;
if !(1..=8192).contains(&width) || !(1..=8192).contains(&height)
    || pixel_count > 32_000_000 { return Err(MediaWorkerRefusalKindV1::WorkerDimensionsInvalid); }
```

Never parse AVIF magic, `ftyp`, `av1C`, pixel content or arbitrary user input **in the parent**. That later implementation belongs strictly inside the already-contained worker under Architecture §6A.8. No raw worker output may become a public URL, file, sealable object or entitlement.

### 5B. Strict outer JSON envelope parser (`solo-adapters/src/media_worker_protocol.rs`)

Expose `parse_media_worker_envelope_v1(bytes: &[u8]) -> Result<MediaWorkerEnvelopeV1, MediaWorkerProtocolRefusalV1>` and no other new external entry point. The input is a **single complete JSON object**, UTF-8, **≤65,536 bytes including whitespace**, with no trailing JSON, duplicate keys at any nested level, unknown fields, floats, negative/overflowing numbers, aliases, comments, extra arrays or coerced strings. Use `serde::Deserialize` structs marked `#[serde(deny_unknown_fields)]` **at every nesting level**, rejecting duplicate fields via derive's duplicate-field detection, then explicitly map into the inner application's types. Do not use free-form `serde_json::Value` as an acceptance shortcut or silently normalize invalid fields.

The closed snake_case JSON V1 shape is:

```json
{
  "version": 1,
  "status": "decoded",
  "format": "avif",
  "mime": "image/avif",
  "source_length": 299,
  "source_sha256": "<64 lowercase hex characters>",
  "width": 32,
  "height": 32,
  "bit_depth": 8,
  "color_model": "yuv",
  "has_alpha": false,
  "is_grid": false,
  "is_sequence": false,
  "orientation": "identity",
  "planes": [
    {"ordinal": 0, "stride": 32, "rows": 32, "length": 1024, "sha256": "<64 lowercase hex characters>"},
    {"ordinal": 1, "stride": 16, "rows": 16, "length": 256, "sha256": "<64 lowercase hex characters>"},
    {"ordinal": 2, "stride": 16, "rows": 16, "length": 256, "sha256": "<64 lowercase hex characters>"}
  ],
  "output_sha256": "<64 lowercase hex characters>"
}
```

Allowed `color_model`: `gray`, `yuv`, `yuva`. Allowed `orientation`: `identity`, `transposed`, `unverified`. `version`, `status`, `format`, `mime` must match exactly as shown. No worker-produced human-readable diagnostic is considered evidence. Use a separately bounded owned plane-byte collection in the typed port's `MediaWorkerRunV1`; the JSON never embeds raw planes, filesystem paths, base64, links, identities, private references, or executable names. The adapter parses protocol structure only and **does not** claim successful decoding, security isolation, licensing or publication.

### 5C. Whole-candidate behavior

Use the actual `GraphicAssetSourceSetV1::evidence()` produced by the accepted source-read use case. A source may be named `.avif` for dispatch only; extension is not a media validation result. The parent must check the known source byte length and owned bytes before asking the fake worker; the fake port returns its own independently owned plane bytes and typed host facts. Validate every returned item in strict order, discarding earlier results on any single source, envelope, hostile transport or digest failure. The source set's unlisted/internal/suppressed declarations remain mandatory. Tests must explicitly show a passing first AVIF followed by a failing second AVIF refuses the whole result and exposes **zero** checked successes.

## 6. Required regression and malicious-input proofs

Add deterministic, portable tests in the two new test files (plus local unit tests inside the new application module if helpful):

1. **Positive structure:** one `GraphicAssetSourceSetV1` generated via the existing graphic-inventory/read fake pattern, with one AVIF, matching canonical source SHA, valid fake host observations, exactly one gray/yuv/yuva plane structure and independent per-plane/composite hashes. Outcome `EnvelopeChecked`, with both publication and media-correctness booleans **false**; fake port called exactly once.
2. **Source-before-worker:** zero bytes and 20 MiB+1 refuse with **zero port calls**. Exactly 20 MiB is allowed past the byte gate (it may then be refused for an unrelated later reason); demonstrate this distinction.
3. **Whole inventory:** two AVIF obligations, first valid and second bad SHA, geometry or resource facts; get a single refusal, no partial record, two port calls. Check an effectively unlisted/internal obligation still drives a call and its defect refuses the entire set. Mixed AVIF and PNG is `WORKER_FORMAT_UNSUPPORTED`, never four-format success.
4. **Host facts:** missing limits readback, wrong 512 MiB/10 s/15,000 ms profile, abnormal/crash exit, timeout, a surviving child, incomplete staging or cleanup: all refuse before retaining output. A fake `true` flag does not establish production host conformance; this remains contract-only evidence.
5. **Protocol:** empty input, 65,536 and 65,537-byte boundaries, duplicate root/nested keys, unknown root/nested keys, trailing second document, invalid UTF-8, wrong/missing version/status/format/MIME, uppercase or malformed SHA, bad enum tokens, negative/floating/overflowing integers, surplus planes and wrong plane order; every incorrect case is rejected by the parser or parent and has a stable typed refusal.
6. **Parent-side adversarial:** source digest mismatch; reported count versus actual buffer length; `stride * rows` overflow/mismatch; missing, empty or duplicate planes; total-output cap; width/height 0 or 8,193; 8,192×8,192 over 32 million; invalid alpha/color mapping; mutated plane byte with unchanged digest; changed overall hash; `kimono_rotate90`-style transposed planes not guessed correct from width alone. Assert explicit refusal categories rather than only `is_err()`.
7. **Hash correctness:** SHA-256 NIST vectors and host-tool cross-check on at least one deterministic vector per CI host; hash raw owned plane bytes, not metadata.
8. **Guard proof:** Cargo graph unchanged (four crates; unchanged lockfile), `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --all-targets --locked`, architecture checker/refusal self-tests and existing source-read/graphic tests. Both-host `rust-verify` verifies pinned `cargo-deny` and current no-bypass checks. No new first-party unsafe, native import, decoder or process call.

**Outcome of this order is not permitted to say `MEDIA-VERIFIED`**, even when all new tests pass. All executed positive cases are fabricated protocol/fake-port fixtures and prove only fail-closed parent data handling and strict parsing. Existing experimental AVIF pixel hashes are prior research evidence, not product acceptance evidence.

## 7. Three exact Git publications, result and STOP rules

Make these commits, in this order, on `main` only, with no interleaved unrelated write:

| Commit | Stage | Exact changed paths | Mandatory remote CI |
| --- | --- | --- | --- |
| 1 | ACTIVE-only issuance, §4B | `.documents/WIP/workorder-active.md` | both hosts green |
| 2 | code + tests, §4C | exactly the six paths in §3; do not add another path | both hosts green; new tests executed |
| 3 | report-only closeout, §4E | `.documents/Reports/execution-report.md` | both hosts green |

The canonical report must contain exact main/issuance/code/report-parent SHAs and trees, blob anchors, verified branch inventory, all eight-path operation ledger, version/graph checks, new test matrix and exact counts, every refusal and workaround, all three two-host CI run IDs/job IDs/step results, unchanged toolchain/security policy, source/pixel claims deliberately **not** made, clean worktree, both task temp inventories and ownership-safe cleanup. Report the outcome as **`WORKER-CONTRACT-PROVEN`** only if the complete contract, all adversarial tests, all existing CI/security/architecture gates and remote readbacks actually pass. Otherwise report **`BLOCKED`** (demonstrated necessary failure) or **`UNVERIFIED`** (essential evidence unavailable) without inventing a pass. At every outcome preserve Stage 03/R03.C/R03.C3 **OPEN**; no codec, operating-system worker, URL/publication or release permission is implied.

**STOP immediately** without unauthorized writes on dead terminal, preflight or handoff mismatch, unexpected dirty material, branch/worktree/remote drift, inability to get both mandatory CI hosts, failed or indeterminate authenticated remote push/readback, any file outside the path ledger, new external dependency/lock change, unapproved `unsafe`, native/process/decoder source creeping into the product, unsafe cleanup, password/secret exposure, failing cargo-deny/architecture gate, unbounded allocation or parser operation, no trustworthy typed whole-candidate refusal, or pressure to call fake-port success an actual decoder. Report the facts safely through the canonical report if the required Git/report conditions can be met; otherwise preserve the untouched tree and return the blocker in chat. Never relax a bound, delete historical branches, or move to Stage 04 to escape a media gate.

**Next engineering sequence after this exact order:** a later distinct Owner-issued order must implement real Linux/Windows process supervisors in a separately authorized OS boundary (including a specific Owner decision for any new Windows first-party `unsafe` allowance), then real codec/conformance/host security tests and whole-candidate integration. No such work is authorized here.
