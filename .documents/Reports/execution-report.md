# Execution Report

**Work order:** `SOLO-WO-118 R05` — first real confined AVIF coded-plane worker and trusted-parent path.
**Required outcome:** `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1`.

**status=STOPPED.** The runtime vertical slice ordered by §3 now exists and is green on all three hosted jobs, but the two mandated N121 real-host cases cannot be constructed through the retained source-reader boundary without changing a path this order declares immutable. No `R03.C`, Stage 03 or V4 claim is made.

## 1. Commits (verified full SHAs)

| Step | SHA | Tree | Parent |
|---|---|---|---|
| baseline (R12) | `c6775e37b7d23c397d8afef4b99a8e76b4c7488e` | — | A12 |
| **A13** (`main`, ACTIVE only) | `dbba90c9df9c1d58417fbb567d4b126cdb4b07e9` | `5037792ed074b940a52e41172b20b85863f4a932` | `c6775e37` |
| **T13.0** (branch, ACTIVE import) | `14f300a2b691bb53f8c2d9eb0a420fdc48c0adbd` | — | `cd3866d4` |
| **T13.1** (branch, runtime slice) | **`e580b25250fd5717949326a279a414edcef16007`** | `23cf029e1d109a4db4a6bf00c85ac265e11bd313` | `14f300a2` |
| **R13** | given in the chat handoff only | — | `dbba90c9` |

A13's ACTIVE blob is `95bfa60a9a6af2b0edd25c8ec010e8e5fb48bf0e` in the working tree, index, commit and `origin/main` alike. T13.0 restored that exact blob by single-path `git restore --source=<A13>` and committed exactly one path; the R04 order was never re-imported.

## 2. What T13.1 delivers (production runtime, not a test)

`crates/solo-adapters/src/graphic_avif_inspection.rs` (new, safe Rust; no `unsafe`, no native node, no new supervisor, no filesystem reader):

- `run_avif_worker_child_v1` proves `child_image_worker_confinement_v1()` **first** (refuses unconfined when absent or off-profile), then reads at most `20 MiB + 1`, refuses empty/oversize, recomputes SHA-256 over the received bytes against the caller's digest, preflights with `oxideav_avif::inspect` (brand `is_image && !is_sequence`, `!is_grid`, `!has_alpha`, `!monochrome`, depth ∈ {8,10,12}, `pixi` agreement, sides ≤ 8192, pixels ≤ 32 M, 4:2:0 or 4:4:4 only), decodes, and re-derives each plane's canonical columns/rows/stride, requiring the decoder's own stride and length to match exactly. Only then does it build the complete 16-field V1 envelope and the exact concatenated Y/U/V payload in memory and write them once.
- Tokens are exactly `1`, `decoded`, `avif`, `image/avif`, `yuv`, `unverified`; `has_alpha/is_grid/is_sequence` are `false`; every plane claim carries `ordinal/stride/rows/length/sha256` from the delivered bytes. A refusal writes nothing.
- `ConfinedAvifInspectAdapterV1` launches only the bound trusted executable through the existing `run_confined_image_worker_v1(..., "avif", source_sha_hex, source)`, maps any supervisor error to `WorkerUnavailable`, records the reported child PID, caps the header at 1 024 bytes, parses **one** envelope with the unchanged strict parser, requires the payload length to equal the checked sum of claimed lengths exactly, splits exactly the claimed planes, refuses a spare byte, and passes only honest host facts into `MediaWorkerObservationV1`.
- `graphic_image_inspection.rs`: one closed arm — exact flag + literal `avif` + canonical lower-hex digest, ≤ 4 arguments. The five-format V3 arm is untouched, so no V3 path reaches this decoder.
- `solo-application/src/media_worker.rs`: `check_observation_v1` binds every claim to the canonical 4:4:4 or 4:2:0 columns/rows/stride derived from claimed width, height and depth (three planes, ordered ordinals), **after** the existing per-claim arithmetic and 256 MiB total bound, so `WORKER_OUTPUT_OVERSIZE` keeps precedence. A retained test caught that ordering defect; its assertion was preserved and the validator order corrected.

## 3. Local gates before the push (`e580b252`)

`fmt --check` 0 · `check --workspace --all-targets --locked` 0 · `clippy -D warnings` 0 · `check-architecture` 0 → `architecture=legal packages=4 targets=34 files=66 edges=93` (targets/files rose by exactly the one new approved module; **edges unchanged at 93**) · `check_architecture_fixtures` 0 · `workspace_graph` 32/32 · R04 independent pixel goldens **2 passed** · `solo-application --test media_worker` **11 passed**.

Local Windows-native `cargo` cannot link (`link.exe` not found — no MSVC build tools in this shell), so native Windows verification is delegated to the hosted job. On Linux the real-source suites need `SOLO_TEST_SOURCE_FS_ROOT`; without the Btrfs fixture they are **NOT RUN LOCALLY**.

## 4. Hosted result at `e580b252`

Push-triggered run **`43706976479`**, attempt 1, overall **success**: `verify (ubuntu-24.04)` success (`132790054004`), `verify (windows-2025)` success (`132790053715`), `avif-reference-oracle` success (`132790053944`). This proves every retained gate still passes with the new module compiled in; it is **not** R05 acceptance, because no N121 case runs yet.

## 5. The blocker: AVIF cannot reach the worker through the retained reader

The ordered N121 cases must use "the real `ContentRootCapability` source reader and the existing `inspect_media_worker_source_set_v1`". `GraphicAssetSourceSetV1` has a private field and exactly one producer, `read_graphic_asset_bytes_v1`, which resolves each obligation through `EntityAssetPathV1::from_reference` → `accepted_image_extension` → `ACCEPTED_IMAGE_EXTENSIONS` in `crates/solo-application/src/graphic_asset_read.rs`. That file contains **no** `avif` mention, and the real run fails exactly there:

```text
thread 'n121::n121_02_real_worker_and_hostile_claim_refusals' panicked at ...:
real source acquisition refused: GraphicAssetReadRefusalV1 {
    kind: Unsupported,
    declaring_owner: Some(QualifiedOwnerV1 { entity: EntityIdV1("example"), .. }),
    role: Some(GraphicLogo),
    local_index: Some(0) }
```

That path is outside this order's ledger (only `media_worker.rs` may change in `solo-application`). Adding `avif` to `ACCEPTED_IMAGE_EXTENSIONS` is a one-line product decision with real scope consequences — it widens what the Entity V1 reader hands to every consumer — so it is **not** mine to take unilaterally, and no guard was bypassed to work around it.

Recorded honestly: both prepared cases were written, compiled and executed against the real binary and reader; they cannot pass until the reader admits AVIF, so the test module was **reverted** and never pushed — a red or `#[ignore]`d gate is not evidence. No N121 CI gate was added either, since §5 forbids gating on a case that cannot pass; `avif-reference-oracle` and both `verify` jobs remain the load-bearing jobs.

## 6. State and the smallest next Owner decision

`main` = `dbba90c9` + this report; branch tip = `e580b252`; **28** remote heads; one worktree; empty stash; exactly four WIP files; both trees clean; task scratch removed. Permanent writes stayed inside the ledger: the new module, one dispatch arm, `lib.rs` registration, the strengthened validator, and the A13 ACTIVE. No `Cargo.toml`/`Cargo.lock`, architecture checker, source/unsafe scanner, `media_worker_host*.rs`, `media_worker_protocol.rs`, `source_read_native.rs`, `solo-cli/src/main.rs`, oracle artefact, fixture or other WIP path changed.

**Smallest decision needed:** a new Owner-issued order authorizing `avif` in `ACCEPTED_IMAGE_EXTENSIONS` (or an equivalent approved acquisition route for authored AVIF sources). The two N121 cases can then be restored exactly as described here, run on both native hosts, and R05 closed. Until then the child, token, port and parent verification exist, compile, and pass every retained gate, but are **unproven on a real host** and publish nothing.

**Status:** `status=STOPPED` — runtime slice delivered at `e580b252` with all three hosted jobs green; N121 2/2, and therefore `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1`, are **not** achieved.
