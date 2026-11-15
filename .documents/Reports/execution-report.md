# Execution Report

**Work order:** `SOLO-WO-118 R06` — prove the existing confined AVIF coded-plane worker on both native hosts.
**Required outcome:** `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1-N121-ACCEPTED`.

**status=COMPLETE** for this bounded coded-plane milestone. Both N121 real-host cases pass on native Ubuntu 24.04 and native Windows 2025 through the real confined `solo-cli` child and the real `ContentRootCapability` source reader, every value matches the unchanged independent R03 goldens, and the two-host `n121_gate` accepted the authentic log while rejecting altered copies. This is **not** V4, six-format `R03.C`, Stage 03, publication or deployment, and no alpha/orientation/display claim is made.

## 1. Commits (full SHAs; R14 appears only in chat)

| Step | SHA | Parent |
|---|---|---|
| baseline | `81eb8cc376bee48bbbf497566af16d595e36845f` (R13) | `dbba90c9` |
| **A14** (`main`, ACTIVE only) | `a11a9042b58509e6cba6b57e05c740bdb8a52491` (tree `077b6d5601d5c68358418e0a8ba99ade31e008ed`) | `81eb8cc3` |
| **T14.0** (branch, ACTIVE import) | `bc95cc69c028872b2c4ce332229d1c16b7ac0f87` | `e580b252` |
| T14.1 (first attempt) | `53ab18ce7751f5b6a83cbcd7557de36e6ac52619` — **red**, superseded | `bc95cc69` |
| **T14 final** | **`81a7bf55f238f231f3fcdd41f6adf628b4a25e97`** (tree `23cf029e` line preserved) | `53ab18ce` |

A14's ACTIVE blob `b31078872e2ec0b1453fef20c8d60db8a1f4ce35` is identical in the working file, index, A14 commit and `origin/main`. T14.0 restored that exact blob by single-path `git restore --source=<A14>` and committed one path; the R05 order was never re-imported.

## 2. Corrected analysis of the former R05 `Unsupported` diagnosis

`crates/solo-domain/src/property_registry.rs` already declares `ACCEPTED_IMAGE_EXTENSIONS: [&str; 7] = ["png","jpg","jpeg","webp","avif","gif","svg"]`, and `EntityAssetPathV1::from_reference` uses that constant; the extension grammar therefore **always admitted `.avif`** and was not modified. `PropertyRefusalKind::FaviconNotPng` is the only role-level format rule. The R05 `GraphicAssetReadRefusalKind::Unsupported` was the `NativeError::Unsupported` mapping of the **source-read mechanism** on the Owner's WSL ext4 filesystem (no `SOLO_TEST_SOURCE_FS_ROOT` Btrfs snapshot capability), not an AVIF denial — R13's attribution was wrong and is withdrawn. Both hosted runs confirm the real mechanism: Ubuntu reads from the disposable Btrfs snapshot filesystem provisioned before the tests, Windows from its native approved reader; both acquired four authentic AVIF sources successfully.

## 3. N121-01 — four authentic coded-plane results (`n121_01_four_coded_plane_goldens_cross_worker_and_parent`)

Four distinct owners (Entity `logo`, Facet `banner`, Site `background`, Page `social-preview`) author the four authentic stills into one task-owned `assets/` root. `read_graphic_asset_bytes_v1(&inventory, &root.capability())` returns `Complete` with four byte-identical obligations; `inspect_media_worker_source_set_v1` then runs the real `ConfinedAvifInspectAdapterV1` through a test-only recording port that clones each actual returned observation.

| fixture | geometry | depth | planes | result |
|---|---|---|---|---|
| `white_1x1.avif` | 1×1 | 8 | 3 (4:4:4) | source identity, per-plane and oracle-framed aggregate match the pin |
| `fox-profile0.8bpc.yuv420.avif` | 1204×800 | 8 | 3 (4:2:0) | ditto |
| `fox-profile0.10bpc.yuv420.avif` | 1204×800 | 10 | 3 (4:2:0) | ditto |
| `fox-profile2.12bpc.yuv420.avif` | 1204×800 | 12 | 3 (4:2:0) | ditto |

Each run asserts a distinct positive child PID, normal exit, verified OS limits, wall deadline met, confirmed cleanup, no leak, exact source length and digest, canonical stride/rows per plane, native 8/10/12-bit depth, per-plane SHA-256 against the unchanged goldens, and the parent's own `media_worker_output_sha256_v1` recomputation. The pinned `aggregate_sha256` is the **independent oracle's** documented `PLANE|ordinal|name|stride|rows|length\n` + bytes framing — deliberately not the product's scheme — so the test reproduces that framing over the actually received bytes and compares it to the pin. `orientation=unverified`, `has_alpha=false`, `is_grid=false`, `is_sequence=false`, and all four checked records report publication authority `false`.

Observed child PIDs — Ubuntu `[6841, 6845, 6850, 6854]`, Windows `[900, 500, 3852, 8776]` — four distinct processes per host.

## 4. N121-02 — real refusals, forged claims and the private obligation matrix

Eleven real refusals, each attributed to the boundary that produced it: `source-empty-before-launch` and `source-oversize-before-launch` (parent source gate, before any spawn), `graphic-favicon-not-png` (authored property grammar), `bad-magic.avif`, `truncated.avif`, `alpha-irot.avif`, `alpha-noirot.avif`, `clap-irot-imir.avif` (real child preflight through the real confined worker), `whole-set-discarded-by-representation` (a suppressed, `internal` representation-level obligation whose bad bytes discard the earlier complete success — no partial record), `platform-namespace-refused-at-acquisition` (the inventory validator accepted the declaration but acquisition refused the Platform namespace, so no fallback owner or format was consulted), and `unconfined-direct-invocation` (the real binary invoked with the reserved flag and token but without the approved OS confinement exits non-zero and writes nothing).

Twelve counterfeit V1 claims built from a genuinely acquired 1×1 source each produced the documented typed refusal: forged source digest → `WorkerSourceMismatch`; forged aggregate and forged plane digest → `WorkerDigestMismatch`; swapped U/V ordinals, invalid stride/rows, short payload and trailing payload → `WorkerPlaneInvalid`; wrong coded depth → `WorkerMediaUnverified`; abnormal exit → `WorkerExitAbnormal`; false limits → `WorkerLimitUnverified`; false cleanup → `WorkerDescendantsRemain`; wall timeout → `WorkerWallTimeout`. Three mutated copies of a canonical header were refused by the unchanged strict parser with `MissingField`, `DuplicateField` and `UnknownField`. A counterfeit proves inward rules only; it is never presented as an independent semantic oracle, and the plain coded-plane contract proves no transformed-display correctness.

Both cases print one `n121_evidence` line after their assertions on both hosts, byte-identical apart from PIDs.

## 5. Load-bearing two-host CI gate

`.github/workflows/rust-verify.yml` gained `Confined AVIF coded-plane worker gate (Linux)` and `(Windows)` after the R03 snapshot-filesystem setup. Each runs exactly `cargo test -p solo-cli --test media_worker_host_smoke --locked n121_ -- --nocapture --test-threads=1`, captures real stdout+stderr with a propagated exit code, and requires exactly `test result: ok. 2 passed; 0 failed; 0 ignored`, both fully-qualified names exactly once, no `FAILED`/`ignored`, and the real evidence tokens. The same verifier must then reject three altered scratch copies (a deleted case, a `1 passed; 1 filtered out` result line, and corrupted `counterfeits_refused=`) before the job may print `n121_gate mutation=rejected`; the authentic log is never modified in place. Both hosts printed `n121_gate authentic=accepted` and `n121_gate mutation=rejected`.

## 6. Hosted result at one exact SHA

Branch push run **`43709995330`**, attempt 1, head `81a7bf55f238f231f3fcdd41f6adf628b4a25e97`, overall **success**:

| job | id | conclusion |
|---|---|---|
| `verify (ubuntu-24.04)` | `132799270408` | success — N121 2/2, gate authentic+mutated rejected |
| `verify (windows-2025)` | `132799270213` | success — N121 2/2, gate authentic+mutated rejected |
| `avif-reference-oracle` | `132799270515` | success — pristine pinned manifest |

The earlier attempt at `53ab18ce` (run `43709094133`) was **red** on both verify hosts in the workspace-test step and is reported, not hidden: two genuine defects (underscore-bearing hostile fixture filenames are not portable asset references, and the pinned aggregate uses the oracle's own framing rather than the product's digest) were diagnosed from the real logs and fixed forward in `81a7bf55`; no assertion, gate or case was weakened, skipped or retried as a substitute for a green run.

## 7. Local-first record and repository state

Local: `fmt --check` 0 · `check --workspace --all-targets --locked` 0 · `clippy -D warnings` 0 · `check-architecture` 0 → `architecture=legal packages=4 targets=34 files=66 edges=93` · `check_architecture_fixtures` 0 · `workspace_graph` 0 · `graphic_avif_inspection` 2/2 · `media_worker` 11/11 · `media_worker_host_smoke --no-run` 0 · workflow YAML parses (42 verify steps). WSL **NOT RUN LOCALLY**: without a verified Btrfs `SOLO_TEST_SOURCE_FS_ROOT` the real reader returns `NativeError::Unsupported`, so no N121 positive can pass there and no fallback reader or arbitrary directory was used. Local Windows **NOT RUN LOCALLY**: `link.exe` is absent in this shell. Both facts are environmental, and the native proof comes from the hosted runs above.

`main` = `a11a9042` + this report; branch tip = `81a7bf55`; **28** remote heads; one worktree and one `origin`; exactly the four WIP files; empty stash; both trees clean; task scratch removed. Only the ledger paths changed: the two N121 cases plus local helpers in `crates/solo-cli/tests/media_worker_host_smoke.rs`, the two gate steps in `.github/workflows/rust-verify.yml`, and the A14 ACTIVE. Product code is byte-identical to T13.1; no immutable path, dependency, manifest, oracle artefact, fixture, guard or accepted test was touched.

**Status:** `status=COMPLETE` — `R03C-AVIF-CONFINED-CODED-PLANE-PARENT-VERIFIED-TWO-HOST-V1-N121-ACCEPTED` for this bounded milestone only. V4 alpha/orientation, the six-format whole-candidate matrix, integration of this technology into `main`, and Stage 03 remain open.
