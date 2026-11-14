# Execution Report

**status=COMPLETE — pending independent Chief Master Architect review**

**Disposition carried out: REVISE.** Two N102 validation defects were corrected on the preserved technical branch, with all previously accepted checkpoints intact: the supervised event probe now requires the parent's own recorded child pid and fully proved confinement/reap/exit instead of trusting the child's line, and the answer parser now requires **exactly one** terminated record instead of trimming newlines. Both hosts pass at the corrected SHA with the four real GIF canaries, the four N102 canaries and 12/12 native Windows repeats unchanged. No GIF or N102 code is integrated into `main`; no release, tag, deployment or self-acceptance.

## 1. Preflight (verified before editing)

`HEAD == main == origin/main == 49c849b5d2a57d59811fb5ca674c0da14d1ebb75`; `solo-wo-111-gif4-n102-r01 == origin == 371c15de81bad746f7e684e59cb2145750b4789c`; empty `git status --porcelain`; one `origin`; one worktree; empty stash; **24** remote refs; exactly four WIP files; ACTIVE `f44911cd` (exact Owner WO-111 bytes). Nothing was reset, rebased, cherry-picked, force-pushed, merged or deleted; no dependency changed.

## 2. Preserved checkpoints

| Step | Commit | Tree | Parent |
|---|---|---|---|
| A authority | `e7f8ed6c663368370e9a79f26a1b253faa976d76` | `0491495d` | `6e834854` |
| B0 exact nine-blob import | `20775dd295c1278bfb2d9bbfed06e182f88eeb14` | `a26caf55` | `e7f8ed6c` |
| C1 `n108_03` | `e2f2d744147bac6c6766ede3d51ef19a196f7e71` | `9ea7fe06` | `20775dd2` |
| C2 `n108_04` | `cee2cf41b8efc02361becc9529f1406767023ee4` | `e67f515d` | `e2f2d744` |
| C3 N102 object identity | `4bef4cccbdd67044d75d1a029a092eb0eb62f365` | `5dbfc4c5` | `cee2cf41` |
| D CI gates | `371c15de81bad746f7e684e59cb2145750b4789c` | `29e92e40` | `4bef4ccc` |
| **C3F correction** | `cdf0ceebc4a140db3d5a7dcd5b2fa8b76ea119b1` | `712d259a689c7e731eb0d9947e5ae91b0eddfcca` | `371c15de` |

**Corrected technical head: `cdf0ceebc4a140db3d5a7dcd5b2fa8b76ea119b1`** on `solo-wo-111-gif4-n102-r01`. C3F changes exactly one path — `crates/solo-adapters/src/media_worker_host.rs`, now blob `894bc6220d90161ce98bb9c0c848ff208388f666` — and `git diff --check` was clean before staging. Every earlier checkpoint and every branch ref is preserved, including the first accepted technical head `371c15de` and the first report commit `49c849b5`.

## 3. Defect 1 corrected — supervised pid identity

`run_n102_event_probe_v1` previously ended with `parse_n102_event_answer_v1(&String::from_utf8_lossy(run.stdout()))`, so a child-supplied pid was accepted without ever being compared to the pid the parent itself observed. It now ends with `validate_n102_event_run_v1(&run)`, a private function placed beside the probe that treats the parent as authoritative:

- `std::str::from_utf8(run.stdout())` must succeed — a lossy decode is no longer possible.
- `parse_n102_event_answer_v1` must produce the bounded answer.
- `run.child_pid() != 0`, `answer.pid == run.child_pid()`, `run.exit_code() == Some(0)`, `run.limits_verified()`, `run.cleanup_verified()`, `answer.attempted` and `answer.witness` must **all** hold, otherwise `N102ProbeRefusalV1::AnswerInvalid`.

Production spawn behaviour, the native two-handle allowlist, `SetEvent`/`WaitForSingleObject` signalling, process lifecycle, cleanup policy and every other N102 test are untouched; the existing four N102 cases still pass and the 12/12 repeat gate still runs.

## 4. Defect 2 corrected — exactly one event-response line

`parse_n102_event_answer_v1` no longer uses `trim_end_matches(['\r', '\n'])`. It now performs exact single-record extraction:

```rust
let line = text
    .strip_suffix("\r\n")
    .or_else(|| text.strip_suffix('\n'))
    .ok_or(N102ProbeRefusalV1::AnswerInvalid)?;
```

A missing terminator, two terminators, an embedded newline or any second record is refused rather than trimmed. The existing ASCII, length, fixed-order field and canonical token checks are retained unchanged. The three specified negatives were added and all return `Err(N102ProbeRefusalV1::AnswerInvalid)`:

```text
SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\n\n
SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\r\n\r\n
SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242\nEXTRA\n
```

## 5. Failure-sensitivity evidence

New test `n102_event_run_validation_refuses_every_inconsistent_observation` drives the **real** `validate_n102_event_run_v1` with constructed `ImageWorkerRunV1` fixtures (parent facts supplied independently of the child's bytes) — no second implementation exists. Accepted: stdout `SOLO-N102-EVENT-V1 attempted=1 witness=1 pid=4242`, `child_pid=4242`, `exit_code=Some(0)`, `limits_verified=true`, `cleanup_verified=true`. Refused independently, each returning `AnswerInvalid`:

| Fixture defect | Result |
|---|---|
| child claims pid `4243`, parent observed `4242` | refused |
| parent child pid `0` | refused |
| `limits_verified = false` | refused |
| `cleanup_verified = false` | refused |
| `exit_code = Some(1)` | refused |
| `exit_code = None` | refused |
| `attempted=0`, `witness=0`, or both `0` | refused |
| repeated record, appended `EXTRA`, blank second line, missing terminator | refused |
| stdout bytes that are not UTF-8 | refused |

Together with the parser tests, `cargo test -p solo-adapters --lib --locked n102_event` runs 3 tests, all passing: `n102_event_answer_refuses_wrong_protocol_and_missing_fields`, `n102_event_answer_refuses_noncanonical_values`, `n102_event_run_validation_refuses_every_inconsistent_observation`.

## 6. Local verification (before any CI dispatch)

WSL **Ubuntu 26.04.1 LTS**, exactly the requested sequence, all green with no suppression:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | clean |
| `cargo check --workspace --all-targets --locked` | 0 errors |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | clean |
| `cargo test -p solo-adapters --lib --locked n102_event` | 3 passed |
| `cargo test -p solo-cli --test media_worker_host_smoke --locked n108_` | 4 passed |
| `cargo test -p solo-cli --test unsafe_boundary --locked` | 22 passed |
| `cargo test -p solo-cli --test check-architecture --locked` | `architecture=legal packages=4 targets=33 files=63 edges=70` |
| `cargo test -p solo-cli --test check_architecture_fixtures --locked` | 35 passed |
| `cargo check --workspace --all-targets --locked --target x86_64-pc-windows-msvc` | clean type-check |

The repository's actual real-tree architecture target is `check-architecture` (hyphen); `check_architecture` is not a target name, so the exact existing command was used rather than inventing one. No test was removed, renamed or skipped.

**Limits, stated honestly:** this Windows 11 host has no MSVC linker (`link.exe` is absent), so a native local `cargo test` cannot link; the Windows build is covered by the clean cross-target type-check and by real `windows-2025` execution. The known WSL reflink limitation still fails five `source_read_native::snapshot_proofs` cases locally; they were not skipped and they pass hosted.

## 7. Technical CI at the corrected SHA

Run `43591877505` (attempt **1**, no retry) at `cdf0ceebc4a140db3d5a7dcd5b2fa8b76ea119b1` — https://github.com/provelopment/provelopment-foundation-solo/actions/runs/43591877505

| Job | Result | Gate steps |
|---|---|---|
| `verify (ubuntu-24.04)` `132438147626` | **success** | 18 N097 success, 19 N108 (Linux) success |
| `verify (windows-2025)` `132438147827` | **success** | 18 N097 success, 20 N108 (Windows) success, 21 N102 four canaries success, 22 N102 12-run repeatability success |

All other gates in both jobs were green: format, strict Clippy, workspace tests, cargo-deny 0.20.2 with a fresh RustSec refresh plus the advisory negative probe, package-set checks, real-tree architecture checker with its refusal fixtures, test-failure tripwires, release build, release CLI smoke, forbidden-import refusal, and the byte-for-byte reproducible double release build.

**N108 four-case evidence (identical on both hosts):**

| Fixture | Source SHA-256 | Transcript SHA-256 |
|---|---|---|
| `n108_03` restore-previous | `91635990bdb2edaabdbf0eb97399def3009a0a415236422cbc5440711d3d3300` | `8e0ef6881dc3429d5f4c4e082053b769463f430aee478bd95105266bc913af6d` |
| `n108_03` background (blue index) | `7bc2081e115c27cc6bf554956036472eb939b7fea82245ba59632679fe1565f5` | `c1c6a33d21411ee7d51fe87f20372a8fa8810fe69bd55b1ddc59df16c5f452fe` |
| `n108_03` local-palette | `25d3d821aedaf21ee7bce1d99d66dff71b19473bfab8e98537e3e6cc402fda51` | `9aa9d186e2cdcaaf2590e334739e09a44a5eaebba3e3764cf9cb656fbe1b6a90` |
| `n108_03` interlace-offset | `e1f5dc87408e10d19176d8c2f268a3d82ac590f5e34c0dbaac31bc4065ecfb0c` | `9acbdc806453cbc4715464ee17cd19c1c9481a1d2835244c2ea565e6219fa0ae` |

`n108_01` (`a0679760…` / `7e0b25b3…`) and `n108_02` (`dc425aea…` / `8c02ba6f…`) are unchanged from the pinned values, `n108_04` refused all nine malformed inputs on both hosts, and each step reported exactly `test result: ok. 4 passed; 0 failed; 0 ignored`. Real child pids on the corrected SHA: Ubuntu `5417, 5421, 5425, 5429`; Windows `7080, 7116, 1232, 1096`.

**N102 evidence:** the four-canary step passed (`4 passed; 0 failed; 0 ignored`) and printed `n102_01 extra_event_signaled=false control_event_signaled=false local_witness=true parent_positive_control=true child_pid=1184`. The repeat step ran twelve times with twelve distinct real child pids (`5768, 2108, 5828, 7480, 4732, 7116, 5024, 7692, 2100, 4612, 3624, 2464`), each printing the same object-identity line and an exact single-case pass count, ending with `N102 repeat 12/12 passed` and `N102 event-identity repeatability: 12/12 consecutive native Windows runs passed.` The tightened validator therefore accepts real Windows evidence, not a synthetic fixture only.

## 8. Warnings, security and component status

No new dependency, feature, toolchain or provider was introduced: `Cargo.toml`, `Cargo.lock`, the adapter manifest, `deny.toml` and the pinned `1.99.1` toolchain are unchanged by this run. `cargo-deny 0.20.2` ran against a freshly refreshed RustSec advisory database together with its negative advisory probe, and the locked dependency graph was verified offline — no vulnerability, licence, source or ban finding and no advisory suppressed. The reproducible release build matched byte-for-byte across two independent builds.

The only new `unsafe`-free code is safe Rust: one private validator, one parser change and test-only fixture construction. `unsafe_boundary` (22 tests, including the `// SAFETY:` adjacency policy) and the real-tree architecture checker both pass, so the previously accepted unsafe surface is unchanged.

## 9. Not claimed, preserved, remaining

No GIF/N102 integration into `main` (still PNG/JPEG/WebP), no product acceptance, no whole-owner V2, no deliberate-red sensitivity proof, no release, tag, deployment or self-acceptance. R03.C3, R03.C and Stage 03 remain OPEN, and the binding safe true-vector SVG requirement is unchanged and unimplemented.

Preserved exactly: all 22 prior non-main refs at their frozen SHAs plus `solo-wo-111-gif4-n102-r01` (24 refs = 23 preflight + 1), ignored Owner data and WSL cache, the four WIP files and the exact WO-111 ACTIVE blob `f44911cd`. Task scratch existed only as owned temporary logs inside the ignored workspace and was removed; the working tree was clean before the report commit.

Still mandatory in later separately authorised orders: `n108_05`–`n108_08`, atomic mixed-format PNG/JPEG/WebP/GIF V2 whole-owner inspection, deliberate-red digest-sensitivity proof, exact-blob GIF integration, and the Owner-approved safe true-vector SVG package.

**Final note for review:** the two corrected defects are independent of GIF and of production spawn semantics; they tighten only what the parent accepts as evidence from its own supervised child, and both hosts plus twelve native Windows repetitions confirm the tighter rules accept real evidence while every inconsistent fixture is refused.



