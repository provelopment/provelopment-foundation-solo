# Execution Report

**status=STOPPED — the strict complete GIF transcript grammar, the per-format GIF stdout cap enforced during read, and the narrow trusted-executable seam are implemented on a green two-host technical branch; the eight mandatory `n108_*` real-worker cases, their execution-count CI gate, the closed mode/fixture seams they require and the parser-focused unit tests are NOT delivered. `R03C-CONFINED-GIF-WORKER-N108-REAL-HOST-PROOF-AND-STRICT-TRANSCRIPT-VERIFIED` is NOT satisfied. No GIF integration, acceptance, publication, release or deployment; no SVG change or implementation.**

## 1. Preflight and authority

`HEAD == main == origin/main == git ls-remote origin main == 644570b01061924bc2f770f76c28539eca71500a`; tree `5c2dd007a59c5ebee87428bb2b9fe82cfccc03f1`; sole parent `ae65053d`; prior report blob `8df01ee3`; tracked WO-108 ACTIVE `cd7a1618`; four WIP files (`92fa562b`, `9d7a8622`, `cb9c7a61`, ACTIVE); 21 remote refs with `solo-wo-107-gif-r01 = efc384e1` and `solo-wo-108-gif-worker-r01 = bb3ef4ab` unchanged, no `solo-wo-109-*`; one origin, one worktree, empty stash; only tracked dirt was the Owner-installed WO-109 ACTIVE `43dc6a56203ca1983a8db358b9947d970f76baaf`, staged as-is.

- **A** = `c97765062b3e0f16f596cb5249fcfa75710cd684`, tree `752843787067baa651ec6806005442820294481d`, sole parent `644570b0`, changing only ACTIVE `43dc6a56`.
- **A issuance CI `43583223625` was GREEN ON BOTH HOSTS**: Ubuntu `132412204273` success, Windows `132412204509` success, including `n102::n102_01_extra_inheritable_handle_is_excluded`.
- **Historical irregularity preserved in the record:** the WO-108 issuance run `43579699908` at `ae65053d` was **red on Windows**, job `132401577479`, in exactly `n102::n102_01_extra_inheritable_handle_is_excluded`. It did **not** recur in this order's A run, nor in the branch run below. No canary was renamed, skipped, weakened or dismissed; the intermittent character is recorded, not explained away.

## 2. B0 — seven exact imported blobs

Path-scoped restore from immutable `bb3ef4ab`, every blob verified by `git hash-object`: workflow `866c9e51`, `Cargo.lock` `cf105167`, adapter manifest `3610f06f`, GIF codec `785d08d0`, adapter root `3f7ab0b3`, adapter worker `61c5e142`, architecture graph `dc6d2770`. **B0** = `31f7bde702d35f3bd6a684d7b93da246f2839c98`, tree `dfda2e78a1dd06bded937d4047f26ed51b54eac0`, parent `c9776506`. The protected fourteen GIF unit tests passed on the imported blob before any change.

## 3. C1 — strict complete transcript grammar, cap on read, trusted seam

**C1** = `1be3750bed19f3145d1e39cc475603343e97dff7`, tree `7ae474603bb8269d3b18b10ff0f47690767e8d99`, parent `31f7bde7`; adapter worker blob `a3096585f3ec357ae3a65833f519a000b4eba311`, host blob `3f6eebd32e8f25458aabe3c72d162490a8592f35`.

1. **The parent validator now consumes the entire received document once, in order.** A single-use cursor walks: initial marker; exactly one `signature` (must equal the parent-held `GIF87a`/`GIF89a` prefix); `mime=image/gif`; `width`/`height` equal to the parent-held logical-screen bytes; positive `frames` equal to the header claim; `loop` with its exact `loop_count` relation (a finite zero count is refused, and `loop_count` is refused for absent/infinite); `one_cycle_delay_ms`; `total_composited_pixels`; `source_sha256` equal to the parent-computed digest; then exactly `frames` sequential `frame=<zero-based>` lines each requiring the exact ordered keys `delay_cs`, `rect`, `disposal`, `transparent`, `interlaced`, `local_palette`, `composite_sha256` and no extra token; then exactly one terminal `end=SOLO-GIF-ANIMATION-V1`; then **no remaining line or byte**. Empty lines, CR, other control bytes, a missing final LF, duplicates, reordering, unknown keys and trailing data all refuse.
2. **Arithmetic and geometry are re-derived, not trusted.** Frame rectangles must be inside the canvas with positive sizes; `frames × width × height == total_composited_pixels` within the 128M cap; `sum(delay_cs × 10) == one_cycle_delay_ms` within 3,600,000; decimals must be canonical (no leading zeros) and digests lowercase 64-hex. The parent still decodes **no** pixels: it reads only the ten fixed header bytes it owns.
3. **Ordered per-frame typed evidence is now exposed** on the supervised observation (`GifFrameEvidenceV2`: index, delay, rect, disposal, transparent index, interlaced, local palette, composite digest), so a host test can assert authored timing, geometry, disposal, palette and interlace rather than a solitary hash.
4. **The smaller GIF stdout cap is applied during read on both hosts.** A fixed `worker_stdout_cap_v1` returns `MAX_GIF_TRACE_BYTES_V1 + 4096` for exactly the closed `gif` token and the unchanged still-image cap otherwise, and it is used at both the Linux reader and the Windows job-object reader. Over-cap output is therefore capped and refused during the read, not after collecting the larger allowance.
5. **One narrow trusted composition-root seam** `inspect_gif_bytes_with_executable_v1` shares the identical validation with `inspect_gif_bytes_v1` (which supplies `current_exe`). No authored content, environment variable, CLI command or argument list can supply the path. The raw decoder stays crate-private.

**Local gates:** `cargo fmt --all` clean; strict Clippy `-D warnings` clean; `cargo check --workspace --all-targets --locked` 0 errors; `graphic_gif` 14 passed / 0 failed; `unsafe_boundary` 22 passed; architecture `legal packages=4 targets=33 files=63 edges=70`. `media_worker_host_native.rs`, the source checker and the unsafe guard are untouched.

**Branch CI `43583511938` at `1be3750b` was GREEN ON BOTH HOSTS**: Ubuntu `132413059690`, Windows `132413059920` — architecture, cargo-deny, release smoke, reproducibility and the four Windows N102 canaries all green.

## 4. What is missing — the reason this is STOPPED

| Required | State |
|---|---|
| `n108_01` … `n108_08` real `solo-cli` worker cases in one `cfg(any(target_os="linux", windows)) mod n108` | **Not written** |
| Both-host workflow step `cargo test … n108_ -- --nocapture --test-threads=1` with exact qualified names, `8 passed; 0 failed; 0 ignored`, no extra/skipped case | **Not added** |
| Closed seams needed by those cases (`gif-wrong-source` / `gif-wrong-transcript` through the real child, malformed-response fixture through the production parser) | **Not added** |
| Parser-focused unit tests (trailing line, duplicate/reordered metadata, skipped frame, unknown key, bad MIME/signature, forged-but-valid digest, rect outside canvas, delay-sum mismatch, noncanonical numbers) | **Not added** |
| `lib.rs` re-export of `GifFrameEvidenceV2` and `inspect_gif_bytes_with_executable_v1` (the export edit did not apply; the library blob `3f7ab0b3` is unchanged) | **Not applied** |

Consequently no real hosted worker process has ever executed the GIF path, the strict document grammar is proven only by compilation and by the codec's own unit tests, and the bounded outcome is unmet. The blocker is remaining execution capacity for the eight-case matrix and its gate, not a governance contradiction; stopping is preferable to fabricating host evidence or presenting compiled code as verified behaviour.

## 5. Nonclaims and preservation

No GIF file was integrated into `main`; green `main` remains PNG/JPEG/WebP only. No publication, delivery, release, deployment, tag or self-acceptance. The binding true-vector sanitized SVG requirement (Architecture §6.2I) is unchanged and still pending. `solo-wo-107-gif-r01` and `solo-wo-108-gif-worker-r01` remain untouched; the only new remote ref is `solo-wo-109-gif-proof-r01` (22 refs total: 21 preserved + 1). ACTIVE remains the exact Owner WO-109 bytes; NEXT/ROADMAP/OUTLOOK, Architecture, SOPs, AGENTS, root manifest, toolchain, `deny.toml`, application V1, native seam, CLI main and the source checker are unchanged. No dependency was added. Task scratch was not created outside owned `.temp/`; local WSL limitation unchanged (five reflink-dependent `snapshot_proofs` fail on this filesystem and pass on hosted CI).