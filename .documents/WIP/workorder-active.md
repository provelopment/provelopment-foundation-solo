# SOLO-WO-110 R01 — FIRST REAL GIF HOST PROOFS: N108_01 + N108_02

**Status:** PROPOSED FOR OWNER ISSUANCE; NOT SELF-ACTIVATING. The Owner alone issues this order by reviewing and installing its **complete exact bytes** as the sole `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`, followed by a matching explicit `SOLO-WO-110 R01` activation. The Implementation Agent alone performs permitted local edits and Git pushes. The Chief Master Architect independently audits GitHub **READ-ONLY** and rules **ACCEPT / REVISE / STOP**. No self-acceptance, release, deployment or publication.

**Required bounded outcome:** `R03C-GIF-N108-FIRST-TWO-REAL-HOST-CANARIES-VERIFIED` — preserve the WO-109 strict GIF parser and confined worker; add **two actual executable tests**, `n108_01_gif87a_still_confined_and_parent_verified` and `n108_02_gif89a_animation_keeps_frames_delays_and_loop`, driving the real compiled `solo-cli` under the real Linux/Windows confined supervisor; a dedicated **mandatory both-host two-case CI gate** must execute and pass them at one exact technical SHA, together with unchanged N097/N102/security/architecture/reproducibility gates. Preserve a green technical branch and publish one truthful report on green main. **Do NOT integrate GIF code into main yet**. This is a deliberately small *partial proof milestone*, not full GIF acceptance. The remaining `n108_03`–`n108_08`, deliberate-red proof, V2 atomic mixed inventory, GIF integration, and secure full-vector SVG are **still required**.

**Why this order is different:** WO-107 implemented a decoder, WO-108 implemented the confined worker, and WO-109 strengthened the parser, but no real hosted GIF-worker test was written. This order starts by writing a functioning host test **before** designing another full matrix. A working test and CI proof are the immediate product of the run. Use the concrete, source-specific Rust and workflow examples below; adapt only as required by actual compiled types and observed failures. Do not stop for normal fixable in-ledger defects. If full outcome becomes blocked, preserve and publish the earliest genuine coded/tested technical checkpoint and report exactly which test executed.

## 1. Immutable preflight and preservation

1. Repository `provelopment/provelopment-foundation-solo`; one Owner checkout at `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (WSL uses same `/mnt/c` checkout); one approved `origin`, one worktree, empty stash. Verify `HEAD == main == origin/main == git ls-remote origin main == ea63f2808b5c94e4c0eb3194cb37a1a03b95b898`, tree `3569c2c381aab26bb7785cc01a9364d7a5515e0a`, sole parent `c97765062b3e0f16f596cb5249fcfa75710cd684`. The only permitted initial tracked dirt is the Owner-installed **exact** WO-110 ACTIVE file; preserve byte-for-byte, no hard-break or newline normalization. STOP on unexplained dirt or missing pins without reset/clean/stash.
2. Prior GitHub blobs on main: `.documents/Reports/execution-report.md` `b54a29f6fca63d047b26243e84cc2a6f6f7e033f`; `.documents/WIP/workorder-active.md` `43dc6a56203ca1983a8db358b9947d970f76baaf`; NEXT `92fa562b1af0c54391851aa5c79deb1631127069`; ROADMAP `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`. Exactly **four canonical WIP** files, one overwrite-only report, no archive.
3. Frozen governance: Architecture `f66096ec375a497ac6de9a65cfbeae4c5dc85844`, architect SOP `1483c9bec1dc9cc7bb7a51590b72eb90c1506af1`, implementation SOP `51bb37922cf7fc56952e4ad6155c9a59d3c09036`, `AGENTS.md` `6eae1da8c721e13cea0c6689a1665e98fa5a9abb`, AST checker `crates/solo-cli/tests/architecture_checks/source.rs` `1c99d3482dd2a9bd7b027f5f193e17603976e361`, unsafe guard `crates/solo-cli/tests/unsafe_boundary.rs` `57d7579616fc0bab96dcb1e371584f20cfd5d8f9`. Frozen native host file `crates/solo-adapters/src/media_worker_host_native.rs` `d411eaf429b6fe1e090654d26f75833bffa597d4`. No edits, waivers or skips.
4. Frozen main technical pins: root manifest `Cargo.toml` `14505be49a281ba0495eabea5ca386548cc36347`, toolchain `rust-toolchain.toml` `9ec55b3b621dbc75fc68817cd51b96e407d3c94c`, `Cargo.lock` `5a18352becaee99b459a93ec75a9d46ce3ff6c8a`, adapter manifest `9c7d526d217bf175bbe9c3956684d28bf8e2a5b6`, workflow `995bd28e7d3d7e236da37da391a5648bfb0317ce`, adapter lib `c009e4282a56fc541d8aef4598ddb533161ba67a`, image worker `90d67201a5959f93758c2a9311e940ee3c8005e5`, safe supervisor `c6d7813943c3979be44208665ec1365830c6d81c`, graph `da7d7ee78ffe7f7c8b32b2e10c776350ab2345a2`, CLI smoke `88b4c6ab7346677196e5bac4ff576fefa7c998ef`, application inspection `bdba09c116a9b0667c4e179668b15168f53e481f`. Preserve all other paths.
5. There are **22** remote refs preflight: main + 21 frozen non-main. Source checkpoint `solo-wo-109-gif-proof-r01` must be `1be3750bed19f3145d1e39cc475603343e97dff7` (tree `7ae474603bb8269d3b18b10ff0f47690767e8d99`); `solo-wo-108-gif-worker-r01=bb3ef4ab083643b1b1d6c332c5c038f12ded68f9`; `solo-wo-107-gif-r01=efc384e17f496522484dff09e0559b4724876829`; 18 older refs unchanged. New authorized branch only `solo-wo-110-gif-host2-r01`; do not repurpose or move older branches. Preserve ignored Owner `.temp/` files, retained patches, WSL cache and old local branches.
6. GitHub independent historical evidence: WO-109 issuance run `43583223625` green on both hosts, source branch `43583511938` green, report `43583884449` green. Earlier WO-108 Windows issuance `43579699908` **red** in `n102_01`; record this historical irregularity, do not misstate as green or suppress the unchanged canary if it recurs.

## 2. Exact permissible writes and operations

Permanent closed ledger:
- `.documents/WIP/workorder-active.md`: only exact Owner bytes, commit **A**, then immutable.
- `.documents/Reports/execution-report.md`: one final overwrite-only report, committed separately on main; no additional WIP/report file.
- `.github/workflows/rust-verify.yml`: restore inherited exact checkpoint, then add two-host N108-first-two gate; do **not** edit existing N097/N102/architecture/cargo-deny/reproducibility/release gates.
- `Cargo.lock`, `crates/solo-adapters/Cargo.toml`, `crates/solo-adapters/src/graphic_gif_inspection.rs`, `crates/solo-cli/tests/architecture_checks/graph.rs`: **exact scope-imported checkpoint blobs only**, no further changes, no dependency addition.
- `crates/solo-adapters/src/graphic_image_inspection.rs`: restore checkpoint, then only fix a real discovered GIF worker/parent-validation defect exposed by N108_01/N108_02; preserve V1 still-image code.
- `crates/solo-adapters/src/media_worker_host.rs`: restore checkpoint, then only fix a proven GIF-specific bounded supervision defect; protect N097/N102 and native host.
- `crates/solo-adapters/src/lib.rs`: restore checkpoint and **only** re-export the already-existing trusted observer seam `inspect_gif_bytes_with_executable_v1` and typed `GifFrameEvidenceV2` if not already exported. Raw `inspect_gif_animation_v1` stays crate-private and non-exported.
- `crates/solo-cli/tests/media_worker_host_smoke.rs`: add exact `mod n108` with **two** real-host tests, deterministic bounded fixture helpers and bounded proof log lines; no files or external fixtures required.

No other permanent paths. The single checkout may create ordinary forward commits, fast-forward push authorized branches, run exact-head `workflow_dispatch`, and import exact named source paths with scoped Git restore. No merge, PR, rebase, cherry-pick, second checkout/worktree/clone, force-push, reset-hard, destructive clean, tag, release, deploy, new dependency or outbound fetch. Scratch only in owned Solo `.temp/` and clean only owned scratch; never touch Owner material. No release or publication authority.

## 3. Execution: get a REAL worker case running immediately

### Phase A — Owner authority

From pinned main, stage only the exact Owner-installed ACTIVE and commit A. Push to main. Verify exact `head_sha` both-host issuance CI and record conclusion. If a genuine red preexisting test occurs (including N102) investigate and preserve failure facts; do not waive or call green. Do not silently abandon the assigned technical work for a transient service delay; request the exact failing job evidence and follow existing STOP boundaries if unresolvable.

### Phase B0 — preserve WO-109 checkpoint (no reinvention)

Create `solo-wo-110-gif-host2-r01` from A after authorized issuance. Scope-import **exactly these eight** source-branch files from `1be3750bed19f3145d1e39cc475603343e97dff7`; verify every imported `git hash-object` before edits, then commit B0:

| Source path | Exact Git blob at source head |
|---|---|
| `.github/workflows/rust-verify.yml` | `866c9e51d2bbb2532bc5a8630c608a075220d500` |
| `Cargo.lock` | `cf1051675576ab0e00ce7acbd5dbe139882c31b4` |
| `crates/solo-adapters/Cargo.toml` | `3610f06ff58cdea6ce1b3993544f0c2e286087ea` |
| `crates/solo-adapters/src/graphic_gif_inspection.rs` | `785d08d0e5a9009be1cbb015eb1b88362f6ab311` |
| `crates/solo-adapters/src/graphic_image_inspection.rs` | `a3096585f3ec357ae3a65833f519a000b4eba311` |
| `crates/solo-adapters/src/lib.rs` | `3f7ab0b397af77d7116c018c1c159fc679ddf4bc` |
| `crates/solo-adapters/src/media_worker_host.rs` | `3f6eebd32e8f25458aabe3c72d162490a8592f35` |
| `crates/solo-cli/tests/architecture_checks/graph.rs` | `dc6d2770038a14199d2f366faab64db4e3d6e98b` |

The baseline GIF codec has fourteen passing unit tests, and the worker's strict parser, typed frame evidence and smaller supervisor output cap already exist. **Do not spend this run rewriting those systems**. The missing link is tests calling the actual executable.

### Phase C1 — N108_01 still GIF: write, compile, execute, commit first

The exact composition-root binary is **already defined** in `crates/solo-cli/tests/media_worker_host_smoke.rs:19`:

```rust
const BIN: &str = env!("CARGO_BIN_EXE_solo-cli");
```

This is an existing trusted Cargo binding, not PATH or authored user input. The worker seam is already implemented in source-branch `graphic_image_inspection.rs:1685`:

```rust
pub fn inspect_gif_bytes_with_executable_v1(
    executable: &std::path::Path,
    source: &[u8],
) -> Result<GifWorkerObservationV1, GifWorkerRefusalV1>;
```

**First actual edit:** Add its `pub use` in adapter `lib.rs`, alongside `GifFrameEvidenceV2` (both already defined). Do not export the raw decoder. Add a `#[cfg(any(target_os = "linux", windows))] mod n108` to `media_worker_host_smoke.rs`. Use the exact reference pattern below; this is a code-level implementation guide, not a substitute for compilation or host evidence:

```rust
#[cfg(any(target_os = "linux", windows))]
mod n108 {
    use std::path::Path;
    use solo_adapters::{
        GifLoopV1, inspect_gif_bytes_with_executable_v1,
    };
    use solo_application::sha256_media_worker_bytes_v1;

    // Valid extension-free GIF87a: 1x1 canvas, global palette red/blue,
    // one image descriptor, LZW codes clear / palette-index-0 / end.
    // Code width is three bits: codes (4, 0, 5) => 0x0144, little-endian bits.
    fn gif87a_red_pixel() -> Vec<u8> {
        let mut bytes = b"GIF87a".to_vec();
        bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
        bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
        bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
        bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0, 0x3b]);
        bytes
    }

    #[test]
    fn n108_01_gif87a_still_confined_and_parent_verified() {
        let bytes = gif87a_red_pixel();
        let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &bytes)
            .expect("real confined GIF87a worker must succeed");
        assert_eq!(obs.signature(), "GIF87a");
        assert_eq!((obs.width(), obs.height()), (1, 1));
        assert_eq!(obs.frame_count(), 1);
        assert!(!obs.is_animated());
        assert_eq!(obs.loop_semantics(), GifLoopV1::Absent);
        assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&bytes));
        assert_eq!(obs.frames()[0].composite_digest(),
                   &sha256_media_worker_bytes_v1(&[255, 0, 0, 255]));
        assert!(obs.transcript_len() > 0);
        assert_ne!(obs.worker_pid(), 0);
        assert!(obs.limits_verified() && obs.cleanup_verified());
        println!("n108_gif=still frames=1 pid={} src={} transcript={}",
            obs.worker_pid(), obs.source_digest().to_hex(),
            obs.transcript_digest().to_hex());
    }
}
```

**Run immediately**, in the same checkout under WSL (these tests use owned memory bytes and do **not** require a reflink root):

```bash
cargo fmt --all -- --check
cargo test -p solo-cli --test media_worker_host_smoke --locked \
  n108_01_ -- --nocapture --test-threads=1
```

If `n108_01` fails, read the exact refusal/stack and fix only the authorized seam/worker; do not create a second decoding path, change the guard, drop the test or claim the failure is caused by the unrelated reflink-only tests. **Commit the working first real-worker test as C1** on the preserved technical branch even before writing the second test. This checkpoint is mandatory for progress and remains retained if later gates fail.

### Phase C2 — N108_02 animated GIF: same production worker

Use a deterministic **three-frame 1x1 GIF89a** generated inline using the same GIF image descriptor and LZW pixel payload. It requires a NETSCAPE2.0 application extension and three Graphic Control Extensions, not three separately decoded images. An example fixture builder to adapt is:

```rust
fn gif89a_three_frames() -> Vec<u8> {
    let mut bytes = b"GIF89a".to_vec();
    bytes.extend_from_slice(&[1, 0, 1, 0, 0x80, 0, 0]);
    bytes.extend_from_slice(&[255, 0, 0, 0, 0, 255]);
    bytes.extend_from_slice(&[0x21, 0xff, 11]);
    bytes.extend_from_slice(b"NETSCAPE2.0");
    bytes.extend_from_slice(&[3, 1, 0, 0, 0]); // infinite loop
    for (delay, disposal, transparent) in [(10u8, 1u8, false),
                                           (20, 2, true), (5, 1, true)] {
        let gce = (disposal << 2) | u8::from(transparent);
        bytes.extend_from_slice(&[0x21, 0xf9, 4, gce, delay, 0, 0, 0]);
        bytes.extend_from_slice(&[0x2c, 0, 0, 0, 0, 1, 0, 1, 0, 0]);
        bytes.extend_from_slice(&[2, 2, 0x44, 0x01, 0]);
    }
    bytes.push(0x3b);
    bytes
}
```

Write **exactly** `n108_02_gif89a_animation_keeps_frames_delays_and_loop`. Invoke `inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &bytes)` as for C1. Assert: `signature == "GIF89a"`, `frame_count == 3`, `is_animated`, `loop_semantics == GifLoopV1::Infinite`, delays `[10,20,5]` centiseconds, one-cycle `350` milliseconds, ordered indexes `0,1,2`, and disposal (`Keep`, `Background`, `Keep`). The current accepted codec's defined transparent-background rule should yield composited RGBA digests red, red, transparent ([0,0,0,0]) for this fixture. Also assert source SHA, valid child PID, limits/readback, verified reap, nonzero transcript length. **A second actual test body** (place inside the same `mod n108` and add `GifDisposalV1` to the adapter imports) is:

```rust
#[test]
fn n108_02_gif89a_animation_keeps_frames_delays_and_loop() {
    let bytes = gif89a_three_frames();
    let obs = inspect_gif_bytes_with_executable_v1(Path::new(super::BIN), &bytes)
        .expect("real confined animated GIF89a worker must succeed");
    assert_eq!(obs.signature(), "GIF89a");
    assert_eq!((obs.width(), obs.height()), (1, 1));
    assert_eq!(obs.frame_count(), 3);
    assert!(obs.is_animated());
    assert_eq!(obs.loop_semantics(), GifLoopV1::Infinite);
    assert_eq!(obs.one_cycle_delay_ms(), 350);
    let frames = obs.frames();
    assert_eq!(frames.iter().map(|f| f.index()).collect::<Vec<_>>(),
               [0, 1, 2]);
    assert_eq!(frames.iter().map(|f| f.delay_centiseconds()).collect::<Vec<_>>(),
               [10, 20, 5]);
    assert_eq!(frames.iter().map(|f| f.disposal()).collect::<Vec<_>>(),
               [GifDisposalV1::Keep, GifDisposalV1::Background, GifDisposalV1::Keep]);
    let red = sha256_media_worker_bytes_v1(&[255, 0, 0, 255]);
    let clear = sha256_media_worker_bytes_v1(&[0, 0, 0, 0]);
    assert_eq!(frames[0].composite_digest(), &red);
    assert_eq!(frames[1].composite_digest(), &red);
    assert_eq!(frames[2].composite_digest(), &clear);
    assert_eq!(obs.source_digest(), &sha256_media_worker_bytes_v1(&bytes));
    assert!(obs.transcript_len() > 0 && obs.worker_pid() != 0);
    assert!(obs.limits_verified() && obs.cleanup_verified());
    println!("n108_gif=animation frames=3 duration_ms=350 pid={} src={} transcript={}",
        obs.worker_pid(), obs.source_digest().to_hex(),
        obs.transcript_digest().to_hex());
}
```

Log only bounded metadata, without logging source bytes, paths or secrets. If a host reveals an actually incorrect fixture or correct product semantics differing from the stated expectation, diagnose with the parser/compositor code and preserve the correct semantics; never force the decoder to agree with an invalid fixture just to make the test green.

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked \
  n108_ -- --nocapture --test-threads=1
cargo test -p solo-adapters --lib --locked graphic_gif
cargo test -p solo-cli --test unsafe_boundary --locked
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all -- --check
```

**Commit C2** when both tests actually run and pass on WSL. Do not wait to write unrelated N108_03–08 cases; those are explicitly a subsequent order.

### Phase D — strict 2-test CI gate, not an empty-workflow success

Add **dedicated** workflow steps on both `ubuntu-24.04` and `windows-2025`, immediately after existing N097, without deleting the existing N102, architecture, cargo-deny, release or reproducibility gates. Exact invocation on both:

```bash
cargo test -p solo-cli --test media_worker_host_smoke --locked \
  n108_ -- --nocapture --test-threads=1
```

Ubuntu `shell: bash` reference pattern (run separately as a normal job step, not a permissive conditional):

```bash
set -euo pipefail
log="$RUNNER_TEMP/n108-first-two.txt"
if ! cargo test -p solo-cli --test media_worker_host_smoke --locked n108_ -- --nocapture --test-threads=1 >"$log" 2>&1; then
  cat "$log"; exit 1
fi
cat "$log"
for name in \
  n108_01_gif87a_still_confined_and_parent_verified \
  n108_02_gif89a_animation_keeps_frames_delays_and_loop; do
  grep -Eq "^test n108::${name} \\.\\.\\." "$log" || exit 1
done
[ "$(grep -Ec '^test n108::n108_0[12]_[A-Za-z0-9_]+ \.\.\.' "$log")" -eq 2 ]
grep -Eq '^test result: ok\. 2 passed; 0 failed; 0 ignored;' "$log"
! grep -Eq '^test n108::.* \.\.\. FAILED|^test result: FAILED' "$log"
```

Windows `shell: pwsh` reference pattern:

```powershell
$required = @('n108_01_gif87a_still_confined_and_parent_verified',
              'n108_02_gif89a_animation_keeps_frames_delays_and_loop')
$log = Join-Path $env:RUNNER_TEMP 'n108-first-two.txt'
cargo test -p solo-cli --test media_worker_host_smoke --locked n108_ -- --nocapture --test-threads=1 *>&1 | Tee-Object -FilePath $log
if ($LASTEXITCODE -ne 0) { throw "N108 real worker test failed: $LASTEXITCODE" }
$text = Get-Content -LiteralPath $log -Raw
foreach ($name in $required) {
  $pattern = '(?m)^test n108::' + [regex]::Escape($name) + ' \.\.\.'
  if ($text -notmatch $pattern) { throw "N108 case absent: $name" }
}
$actual = [regex]::Matches($text,'(?m)^test n108::n108_0[12]_[A-Za-z0-9_]+ \.\.\.').Count
if ($actual -ne 2) { throw "N108 exactly-two requirement failed: $actual" }
if ($text -notmatch 'test result: ok\. 2 passed; 0 failed; 0 ignored;') { throw 'N108 passing count wrong' }
if ($text -match '(?m)^test n108::.* \.\.\. FAILED|test result: FAILED') { throw 'N108 failed' }
```

**Important:** because the command uses a Rust test-name filter, the harness may legitimately report other cases as `filtered out`; do **not** require `0 filtered out`. The gate must nonetheless require the exact two named tests, `2 passed; 0 failed; 0 ignored`, no extra `n108_` case and no skipped/failed case. With `--nocapture`, a test can print after the `...` prefix, so do not require a same-line `... ok` suffix; require the execution prefix plus the harness total, as above.

Commit D, push technical branch fast-forward and dispatch the existing workflow. The actual GitHub run's `head_sha` must equal the commit and **both** `verify (ubuntu-24.04)` and `verify (windows-2025)` must succeed. Inspect logs showing both test names and actual real child PID, limits, cleanup, expected 1/3 frame counts, 350ms animation and source/transcript digests. Compare source/transcript digest hex across host logs; PIDs and wall times differ. Rerun after a forward-only correction if a fixable test failure appears. Keep the green technical branch, do **not** integrate to main.

### Phase E — truthful final report and main CI

After the actual branch gate: return to green main A, overwrite only `.documents/Reports/execution-report.md`, commit **report-only R**, push and obtain fresh both-host green CI for R. Do not touch ACTIVE, NEXT/ROADMAP/OUTLOOK or any other main technical file. Report full SHAs/trees/parents, exact imported blobs, A and D/R CI run/job/host SHA identity, N108_01/02 observed results and digests, any errors/corrections, preservation/refusals, and the **six still-pending tests N108_03–08**.

Status may say `COMPLETE — bounded first-two host-proof milestone pending Architect review` **only** if both named real tests are genuinely green on both hosts, strict two-case CI is in the exact green branch tree, and the source remains safely confined/nonpublishing. Otherwise STOP truthfully **but retain every real test/technical checkpoint on the authorized branch**. Do not call GIF a product capability, an accepted full N108 proof or a published asset. `R03.C3`, `R03.C`, Stage 03 remain OPEN. The binding safe scalable SVG and full animated GIF obligations remain OPEN.

## 4. Explicit acceptance, next concrete milestones and references

**Architect ACCEPT of this bounded order requires:** verified authority; exact source import; real `n108_01` and `n108_02` using **the production executable and supervisor**, not a direct raw decoder; nonzero native child PID and true OS bounds/reap; correct still and animation timeline; exact parent-trusted source/transcript facts; mandatory actual two-host CI gate with named execution + 2/0/0; N097/N102 and strict security/architecture/dependency/repro gates unchanged and green; preserved technical branch; report-only main final green; no release/integration. Anything less receives REVISE or STOP with preserved progress.

**Next order:** implement N108_03–N108_08, including negatives, malformed transcript, budget adversaries and cross-host pinned digests, and increase the gate to **eight**; then an Owner-issued V2 atomic mixed GIF/raster policy + deliberate-red digest proof + exact-blob integration order; then mandatory actual safe-vector SVG code. No format is silently dropped or flattened. This smaller outcome intentionally supersedes WO-109's unrealized all-eight test milestone; it does **not** lower the eventual all-eight obligation.

**Source-level implementation references** (these are pointers for the agent to read, not authority to broaden the ledger):
- Existing `CARGO_BIN_EXE_solo-cli` test binary binding and N097 test style: `https://github.com/provelopment/provelopment-foundation-solo/blob/1be3750bed19f3145d1e39cc475603343e97dff7/crates/solo-cli/tests/media_worker_host_smoke.rs#L13-L22` and `#L1803-L1930`.
- Trusted GIF worker seam and typed frame accessors: `https://github.com/provelopment/provelopment-foundation-solo/blob/1be3750bed19f3145d1e39cc475603343e97dff7/crates/solo-adapters/src/graphic_image_inspection.rs#L1450-L1600` and `#L1644-L1720`.
- Existing strict GIF transcript validation: `https://github.com/provelopment/provelopment-foundation-solo/blob/1be3750bed19f3145d1e39cc475603343e97dff7/crates/solo-adapters/src/graphic_image_inspection.rs#L1885-L2084`.
- Existing Windows N102 strict named-test CI step pattern: `https://github.com/provelopment/provelopment-foundation-solo/blob/1be3750bed19f3145d1e39cc475603343e97dff7/.github/workflows/rust-verify.yml#L650-L684`.
- Cargo integration-test `CARGO_BIN_EXE_<name>` contract: `https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests`.
- Pinned GIF decoder API documentation: `https://docs.rs/gif/0.14.2/gif/struct.DecodeOptions.html` and `https://docs.rs/gif/0.14.2/gif/enum.MemoryLimit.html`.

**Prohibitions remain:** no source checker/unsafe-guard changes, no second clone, no extra WIP/report archive, no force/reset/rebase/merge, no weakened tests, no arbitrary file or network access in the worker, no unapproved native operations, no release/deploy/publication, and no self-acceptance.
