# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-086 R01` — integrate the independently accepted Linux `RLIMIT_CPU` kernel-termination proof into `main` by exact two-blob promotion, with no modification of accepted implementation bytes.
**Matching activation:** `SOLO-WO-086 R01`. **Date (ISO):** 2026-11-13.

**Implementation-agent claimed outcome: `LINUX-RLIMIT-CPU-KERNEL-TERMINATION-INTEGRATED`.** This is a claim for independent review, not self-acceptance. Two accepted Git blobs from preserved branch `solo-wo-085-r01` were transplanted to `main` byte-for-byte by one path-scoped `git restore`; no merge, no cherry-pick, no whole-branch operation, no reformat and no source edit occurred.

**What "integrated" means.** One Linux kernel-enforced CPU-time limit for one trusted, descendant-free child is now present on `main`. It is **not** whole host containment. It does not prove aggregate CPU budgets, hostile-child resistance, descendant or group termination, exhaustion across a tree, a Windows Job Object, RSS/aggregate memory confinement, executable identity, AVIF correctness, media publication or release. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**; public media-correctness and publication authority flags remain `false`.

**Three mainline commits, four distinct tracked paths.** `main` advanced from `09aab85a…` by three ordered commits: (1) the Owner-installed ACTIVE file only, (2) exactly the two accepted implementation files, (3) this canonical report only. No other tracked byte changed.

## 1. Preflight — PASS (`S086-01`)

Sole Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, verified as the real worktree root (`git rev-parse --show-toplevel` = `C:/Users/Sean/Documents/Provelopment/00.foundation-solo`, not a symlink target or parent workspace). Exactly one worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git` (fetch and push identical, no second remote), no stash, exactly four WIP filenames, and `git status --porcelain` showing **only** ` M .documents/WIP/workorder-active.md` — the Owner-installed order and nothing else.

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `09aab85a49a8d44ff72e39c90ae9484a9818c0f8` | exact, local and via GitHub `git/ref/heads/main` |
| `HEAD^{tree}` | `d092f729f80a2833e5f6054423b6f02a9ecf4a2d` | exact |
| Canonical report blob | `6a510196334cc3a74f5f8532ea77f6fc8925c42c` | exact |
| Previously tracked ACTIVE blob | `d3d1096fdfb21d1a25dca00d0200d9bfb1594ad5` | exact |
| Charter `ARCHITECTURE.md` | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact: `92fa562b1af0c54391851aa5c79deb1631127069`, `9d7a86226878f490fd5e26c6813a81e5f73f3f37`, `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| `rust-verify.yml` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact; both literal `edges=41` checks present (`grep -q 'edges=41'` and `-notmatch 'edges=41'`) |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| `solo-adapters/Cargo.toml` | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` | exact |
| `solo-adapters/src/lib.rs` | `1550d34f820642bb5573e06d4fccb758109fd26c` | exact |
| `media_worker_host_native.rs` | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| `media_worker_host.rs` (main before) | `90ec7301ebcdcf4e7bbacf7f3f13ee14557bd71f` | exact |
| `media_worker_host_smoke.rs` (main before) | `30dffd08dfbd1c8f79edc1c35b7fe092653ea91b` | exact |
| `solo-cli/src/main.rs` | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| Unsafe guard `tests/unsafe_boundary.rs` | `b88e7b42ab03bd78557e58d4c54b957b59369d6c` | exact |
| Arch graph `tests/architecture_checks/graph.rs` | `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b` | exact |
| Workspace graph `tests/workspace_graph.rs` | `25a91b2c212805633fcf0fd80e4aaea11c595ba3` | exact |
| Accepted branch `solo-wo-085-r01` | `5bf9082bd44e6ecaedd783e2a93e6260a87073d3`, tree `d796ba2d4fc5aa35b618f029a96afe6e30075a2a` | exact locally **and** on `origin` |
| Branch ancestors | `bb1da94cc7d84d0c9d27f105839baac361f76bcf` / `a763655f3c137c3dd557552a3d8e34e84e4afacf` | exact (branch tip's single parent is `a763655f…`; issuance tree `7fc2354c923322f0a8874cf798dc00dbbe804cba`) |
| Branch façade / test blobs | `88d94ecc81ccdd912792bad8721d5950d3fb217c` / `038c3193e96633f34e0d34baffab7d04b348573d` | exact |
| Branch's own ACTIVE / own report | `d3d1096f…` / branch-only `30a880227acf0e0536c358688b256cc2eba514c8` | inspected, **neither imported** |


**Source ancestry verified.** `git diff --numstat bb1da94c…5bf9082b` = exactly `928 7 crates/solo-adapters/src/media_worker_host.rs` and `239 2 crates/solo-cli/tests/media_worker_host_smoke.rs`; `--name-status` = exactly two `M` rows. Branch `main.rs`, native module, lockfile, workflow, guards, Charter and the three WIP plans are all byte-identical to `main` at those same blobs. The branch's own ACTIVE and its own report differ from `main`'s and were excluded from every stage.

**Preserved proof and historical branches** — verified locally and on `origin`, unmoved before and after this run: `solo-wo-083-r01` `ccc3ca8fb37d52b405a2854e4962fb79aff06ab2`; `solo-wo-081-r01` `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19`; `solo-wo-079-r01` `380393ddacf37fc1a6398805235ca2d5dbd9677e`; `solo-wo-076-r01` `40a40bab1d017e61340fe53399b49fb2ab335c3b`; `solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8`.

**Prior CI provenance (history only, not reused as WO-086 evidence).** Branch proof run `43363659723` = `workflow_dispatch` on `5bf9082b…`, `success`. Prior `main` report run `43364662038` = `push` on `09aab85a…`, `success`.

**Inherited ignored material preserved by direct inspection.** `.temp/` contains `architecture-fixtures/`, `solo-wo-073-wip/` (**4 files / 46 062 bytes**: `Cargo.lock.prepared` 4 518, `Cargo.toml.prepared` 2 915, `media_worker_host_native.rs` 32 319, `NOTES.md` 6 310 — never imported, executed or staged), `toolchain-installer/`, and this run's own disposable scratch directory. `/target/` exists and was untouched. Only `.temp/` and `/target/` are ignored; no unknown tracked, staged or untracked path existed.

## 2. ACTIVE-only issuance on `main` — PASS (`S086-02`)

Exactly one path was staged. Nothing else was in the index.

| Commit | Tree | Parent(s) | Ledger | New ACTIVE blob |
| --- | --- | --- | --- | --- |
| `626c7000727b985cdc1ee4cb5f76c2fdb444e75c` | `7594f7a6429606ac3209c30e3cd9236a990426a7` | `09aab85a49a8d44ff72e39c90ae9484a9818c0f8` (only) | `M .documents/WIP/workorder-active.md`, `+83/-65` | `381c4e6061488cd7af1586aa2c425d589cc41d74` |

The complete Owner-installed order (131 lines, ending at the §9 STOP line) was committed unchanged; no source or report file entered this commit. Push was `09aab85..626c700  main -> main` — an ordinary fast-forward, never forced. GitHub readback (`git/ref/heads/main`, commit endpoint, contents endpoint) returned the identical commit, tree, single parent, **one-file** change ledger `+83/-65` and ACTIVE blob `381c4e60…`. The checkout was clean at `626c7000…` before any implementation staging.

**Issuance CI — run `43377567862`, `event=push`, `head_branch=main`, `head_sha=626c7000727b985cdc1ee4cb5f76c2fdb444e75c`, path `.github/workflows/rust-verify.yml`, conclusion `success`.**

| Job | ID | Steps | Outcome |
| --- | --- | --- | --- |
| `verify (ubuntu-24.04)` | `131727516237` | 33 configured | 24 `success`, 9 host-inapplicable `skipped`, **0 failed** |
| `verify (windows-2025)` | `131727516539` | 33 configured | 19 `success`, 14 host-inapplicable `skipped`, **0 failed** |

The nine Ubuntu skips are exactly the Windows-only steps (revision record, version assert, cargo-deny, package-set gate, architecture checker, refusal self-test, tripwire, release smoke, job summary); the fourteen Windows skips are the Linux-only and Ubuntu-only steps. No implementation staging began before this gate was green.


## 3. Exactly two authorized paths — accepted Git blobs only (`S086-03`)

On clean `main` at the successful issuance SHA, one **literal** path-scoped restore was executed and nothing else:

```text
git restore --source=5bf9082bd44e6ecaedd783e2a93e6260a87073d3 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

No wildcard, no whole-tree checkout, no cherry-pick, no merge, no rebase, no third path. Exit status `0`.

### 3.1 Five mandatory precommit checks — all PASS

**Check 1 — ledger and whitespace.** `git diff --cached --name-status` returned exactly two `M` rows; `git diff --cached --numstat` returned exactly `928 7 crates/solo-adapters/src/media_worker_host.rs` and `239 2 crates/solo-cli/tests/media_worker_host_smoke.rs`; `git diff --cached --check` produced no output and exit `0`. No other status, no renamed, added or deleted file.

**Check 2 — index object identity vs the accepted branch.** Both separately confirmed, all three values identical per path:

| Path | `git rev-parse :<path>` (index) | `5bf9082b…:<path>` | Required |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `88d94ecc…` ✔ |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `038c3193e96633f34e0d34baffab7d04b348573d` | `038c3193e96633f34e0d34baffab7d04b348573d` | `038c3193…` ✔ |

**Check 3 — filter-aware hash round-trip, CRLF disclosed without modification.**

| Path | `git hash-object <path>` (filter-aware) | `:path` / `5bf9082b:path` | `git hash-object --no-filters` (raw worktree bytes) |
| --- | --- | --- | --- |
| `…/media_worker_host.rs` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `88d94ecc…` ✔ | `96c719deb92ebf3955f1f7eef882dd48ac3d3d04` |
| `…/media_worker_host_smoke.rs` | `038c3193e96633f34e0d34baffab7d04b348573d` | `038c3193…` ✔ | `4ce688b527600a8215c11e9e41efd96ef056c8c1` |

Line-ending facts, measured directly from the physical files with **no write of any kind performed to obtain them**: repository `core.autocrlf=true`, `core.eol` unset, `.gitattributes` `* text=auto`, `git check-attr` reports `text: auto` / `eol: unspecified` for both paths. Consequently the Owner Windows checkout holds CRLF bytes: façade `145 604` bytes with `LF=3268`, `CRLF=3268`, bare `LF=0`; test `38 807` bytes with `LF=1023`, `CRLF=1023`, bare `LF=0`. The raw/no-filter hashes therefore differ from the canonical LF blobs, which is the expected pre-existing line-ending mapping and **not** a source difference. Under the filters the worktree round-trips exactly to `88d94ecc…` / `038c3193…`, so promoted source identity is judged by index, canonical Git blob and filter-aware hash — all three identical. Nothing was normalised, rewritten or reformatted.

**Check 4 — untracked and unstaged.** `git ls-files -o --exclude-standard` was empty. `git status --porcelain` showed only the two authorized staged `M` entries and no unstaged or third-path change. Ignored inherited scratch in `.temp/` and `/target/` was preserved.

**Check 5 — protected-path hold.** `git diff --name-only 626c7000727b985cdc1ee4cb5f76c2fdb444e75c --cached` contained only the two authorized paths. `git diff --name-only` (unstaged) was empty. Every protected path was re-verified at identical index and `HEAD` blobs: workflow `e1b5ca6e…`, `Cargo.lock` `b1ee56da…`, adapter manifest `47d732f2…`, `lib.rs` `1550d34f…`, native module `d10151e4…`, CLI root `b3c89ad0…`, unsafe guard `b88e7b42…`, architecture graph `7a5beada…`, workspace graph `25a91b2c…`, three WIP plans `92fa562b…`/`9d7a8622…`/`cb9c7a61…`, ACTIVE `381c4e60…`, Charter `c941c7f9…`, report `6a510196…` (untouched at this point). The branch's own WIP and report were never staged.


## 4. Two-path integration commit on `main` — PASS (`S086-04`)

| Commit | Tree | Parent(s) | Ledger | Façade blob | Test blob |
| --- | --- | --- | --- | --- | --- |
| `b4c06630ab79b3a38c57ae58eb0bc45f9f699665` | `8ef2e7749512197c27224e156dc1e5e6a956a261` | `626c7000727b985cdc1ee4cb5f76c2fdb444e75c` (only) | two `M` rows: `+928/-7`, `+239/-2` | `88d94ecc81ccdd912792bad8721d5950d3fb217c` | `038c3193e96633f34e0d34baffab7d04b348573d` |

Both files were committed **together in one commit** whose only parent is the issuance commit. Push was `626c700..b4c0663  main -> main`, an ordinary fast-forward only. GitHub readback confirmed commit `b4c06630…`, tree `8ef2e774…`, parent list `626c7000…`, the exact two-file change ledger with `+928/-7` and `+239/-2`, and contents blobs `88d94ecc…` / `038c3193…` — identical to the accepted branch. `solo-wo-085-r01` remained at `5bf9082bd44e6ecaedd783e2a93e6260a87073d3` both locally and on `origin` throughout: never moved, merged, rebased or cherry-picked as a branch. No new branch, no tag, no release, no pull request, no branch-protection or repository-setting change.

**Complete permanent WO-086 tracked edit ledger — four distinct paths, three ordered mainline commits.**

1. `.documents/WIP/workorder-active.md` — issuance commit `626c7000…` only.
2. `crates/solo-adapters/src/media_worker_host.rs` **and** `crates/solo-cli/tests/media_worker_host_smoke.rs` — integration commit `b4c06630…` only.
3. `.documents/Reports/execution-report.md` — this report-only commit only.

No workflow, manifest, lockfile, native `unsafe` module, CLI root, library, guard, other test, WIP plan, Charter, SOP, README, media input, release file or fixture was edited.

## 5. Fresh full Ubuntu + Windows integration CI at the exact integration SHA — PASS (`S086-05`, `S086-06`, `S086-07`)

**Run `43378355694`, `event=push`, `head_branch=main`, `head_sha=b4c06630ab79b3a38c57ae58eb0bc45f9f699665`, path `.github/workflows/rust-verify.yml`, `run_attempt=1`, conclusion `success`.** This is a new push-triggered run at the integration SHA; the branch proof run, the issuance run and earlier history were not reused.

| Job | ID | Steps | Outcome |
| --- | --- | --- | --- |
| `verify (ubuntu-24.04)` | `131730062269` | 33 configured | 24 `success`, 9 host-inapplicable `skipped`, **0 failed** |
| `verify (windows-2025)` | `131730061854` | 33 configured | 19 `success`, 14 host-inapplicable `skipped`, **0 failed** |

### 5.1 `N085-01` — real Linux kernel CPU termination (Ubuntu)

Test `n085_01_the_kernel_cpu_limit_terminates_the_child ... ok`, and the CI log carries the fresh line at this integrated SHA:

```text
n085_cpu=media_host_cpu=kernel_terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4381 pgid=4381 signal=9 reaped=1 parent_killed=0 wall_ms=10004
```

Every required property holds: `as_soft == as_hard == 536870912`, `cpu_soft == cpu_hard == 10`, `pid=4381` positive, `pgid=4381 == pid`, `signal=9` (kernel `SIGKILL`, from the leader's real waited status before any kill by this parent), `reaped=1`, `parent_killed=0` — the parent neither caused nor needed to cause this termination — and `wall_ms=10004` inside `0 < wall_ms < 60000`. The observed wall time is a **measurement** and is not consumed CPU seconds; no exact wall duration is asserted. `signal=24` (`SIGXCPU`) would have been equally acceptable. The enclosing suite line is `test result: ok. 13 passed; 0 failed; … finished in 10.10s`.

Note on freshness: `pid=4381`/`pgid=4381` differ from the branch evidence's `4346`, and `wall_ms=10004` differs from `10008`, but neither difference is required for freshness — the new run ID, integration SHA, job ID and log occurrence at `2026-11-13T06:24:13Z` establish it.

### 5.2 `N085-02` — direct child refusal (Ubuntu)

```text
n085_direct_cpu_child=exit=71
```

Test `n085_02_a_direct_cpu_child_refuses_before_the_loop ... ok`: an unsupervised `cpu-child` refused promptly with fixed status `71`, no stdout, no stderr, before any loop. No unsupervised CPU burn occurred.

### 5.3 `N085-03` — typed refusals (both hosts)

Seven pure neutral cases passed on both hosts plus the real-binary token test on Ubuntu: `n085_03_malformed_cpu_readiness_is_refused`, `n085_03_only_the_exact_cpu_readiness_line_is_accepted`, `n085_03_refused_cpu_attempts_never_become_evidence`, `n085_03_the_cpu_evidence_line_is_fixed_and_bounded`, `n085_03_every_unexpected_cpu_termination_is_refused`, `n085_03_the_cpu_tokens_are_closed_and_additive`, `n085_03_the_fixed_cpu_profile_is_the_ten_second_kernel_limit`, plus `n085_03_every_wrong_cpu_token_is_refused_by_the_real_binary` — all `... ok`.


### 5.4 `N085-04` — Windows fail-closed (Windows)

`test n085_04_both_cpu_modes_are_unsupported_before_any_child_exists ... ok` at the integration SHA. The test drives both closed modes `cpu-parent` and `cpu-child` through the real binary and asserts, for each: no timeout, `elapsed_ms < 10 000` (prompt, never waiting for a deadline), `exit_code == Some(3)`, `stdout.is_empty()`, no `media_host_cpu=kernel_terminated` record, no `SOLO-MEDIA-HOST-CPU-V1` readiness line, and `stderr == "error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED\n"`. So both new modes returned **status `3`** with the fixed `HOST_UNSUPPORTED` refusal, empty stdout and no CPU loop or Linux child.

**Zero Linux-only execution on Windows**, confirmed by log inspection rather than badges: `n085_cpu=` count `0`, `n085_direct_cpu_child` count `0`, Linux-only `n076_01` count `0`, `n083_01` count `0`. The only `n081` line present is the platform-neutral unit test `n081_01_the_wall_negative_uses_the_fixed_deadline_and_profile`, which is a pure profile assertion, not a Linux child case. Neutral parser, validator and unit tests still executed and passed (34 `test result:` groups, all `ok`, `0 failed`), and existing public CLI preflight success/refusal output and codes were unchanged.

### 5.5 `N076` / `N079` / `N081` / `N083` regressions (Ubuntu, independent kernels)

```text
n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4303 pgid=4303 wall_ms=10
n079_direct_child=exit=71
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4319 pgid=4319 signal=9 reaped=1 wall_ms=2004
n081_direct_wall_child=exit=71
n083_as=media_host_as=limit_enforced as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 request_bytes=805306368 reserve=denied pid=4351 pgid=4351 wall_ms=10
n083_direct_as_child=exit=71
```

`N076-01..03` kernel readback (`getrlimit` of `RLIMIT_AS` and `RLIMIT_CPU`, full-range ceilings, typed refusal of a rejected status) all `... ok`. `N079` benign limit readback, `N081` 2 000 ms deadline `SIGKILL` with leader reap, `N083` one `805306368`-byte reservation denied under the `536870912`-byte ceiling, and every direct-child refusal behaved exactly as accepted and unmodified. All closed-token and public-CLI contract cases stayed green with byte-exact exits and outputs. No media or publication implication is asserted by any of these.

### 5.6 Both-host original downstream gates

| Gate | Ubuntu `131730062269` | Windows `131730061854` |
| --- | --- | --- |
| Pinned toolchain `1.99.1` (locked fetch) | pass | pass |
| `cargo fmt --all -- --check` | pass | pass |
| Strict Clippy `--workspace --all-targets --all-features --locked -- -D warnings` | pass | pass |
| Locked all-target unit/integration tests | pass, `0 failed` throughout | pass, `0 failed` throughout |
| Real-tree architecture checker | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Both literal workflow edge pins | `grep -q 'edges=41'` and `-notmatch 'edges=41'` both satisfied | both satisfied |
| Refusal self-test | `self-check=passed cases=29` | `self-check=passed cases=29` |
| Dynamic-load / checker-bypass negatives | `ok checker-bypass refused as layer-outward-reference (1 finding(s))`; every forbidden fixture refused by its expected category | refused as designed |
| `cargo-deny 0.20.2` (verified archive) | archive sha256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`; RustSec `af7b3ec33460b8e2de8d422e16d628518891a519` fetched `2026-11-13T06:23:38Z`; advisories `0` errors, bans `0` errors, licenses `0` errors / `1` warning, sources `0` errors | same gate, `all ok` |
| Advisory negative probe `RUSTSEC-2026-0009` | detected as designed | host-inapplicable |
| Injected test-failure tripwire | detected; workflow stayed green | detected; workflow stayed green |
| Release build + CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | identical |
| Forbidden-import injection + exact restoration | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | host-inapplicable |
| Two clean byte-identical release builds | `reproducibility=verified`, `clean_release_bytes=667584` | host-inapplicable |
| Raw checkout `Cargo.lock` SHA-256 | `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` (unchanged through the security gate) | `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e` (unchanged through the security gate) |

The single cross-host **Git** identity for the lockfile remains blob `b1ee56da60f36536548586528db371b7cbe508e4`; the two raw SHA-256 values differ only through the pre-existing host line-ending mapping and not through any change by this order. These are CI test-build gates, not product release permissions.


## 6. Gate ledger (`S086-01` .. `S086-09`)

| Gate | Status | Evidence |
| --- | --- | --- |
| `S086-01` exact preflight | **PASS** | Baseline `09aab85a…`/tree `d092f729…`; all 21 pinned anchors exact locally and via GitHub; one checkout, one worktree, one `origin`, four WIP files, clean except Owner ACTIVE; seven preserved branches at exact SHAs; inherited `.temp/solo-wo-073-wip/` 4 files / 46 062 bytes intact |
| `S086-02` ACTIVE-only issuance | **PASS** | `626c7000…`, tree `7594f7a6…`, single parent `09aab85a…`, one path `+83/-65`, ACTIVE blob `381c4e60…`, GitHub-verified; issuance run `43377567862` green on both hosts, 0 failed |
| `S086-03` two literal restores + five proofs | **PASS** | One literal path-scoped `git restore`; exactly two `M`, `928/7` + `239/2`, `--check` clean; index = branch = filter-aware hash = `88d94ecc…` / `038c3193…`; CRLF worktree bytes disclosed and round-tripping; no untracked, no third path, all 15 protected paths held |
| `S086-04` two-path integration commit | **PASS** | `b4c06630…`, tree `8ef2e774…`, single parent `626c7000…`, two paths only; GitHub ledger and both blobs exact; `solo-wo-085-r01` unmoved locally and remotely |
| `S086-05` integration CI + real kernel evidence | **PASS** | Run `43378355694` green both hosts; `n085_cpu=…pid=4381 pgid=4381 signal=9 reaped=1 parent_killed=0 wall_ms=10004`; `n085_direct_cpu_child=exit=71`; `N085-03` complete |
| `S086-06` Windows fail-closed + regressions | **PASS** | `n085_04… ok` with status 3, empty stdout, exact `HOST_UNSUPPORTED`; zero `n085_cpu=`/Linux-only kernel tests on Windows; `N076`/`N079`/`N081`/`N083` green; CLI bytes unchanged |
| `S086-07` original downstream gates | **PASS** | `edges=41` (both pins), `self-check=passed cases=29`, cargo-deny `0.20.2` fresh RustSec, tripwires, `release_smoke=verified accepted_exit=0 refused_exit=2`, forbidden-import refusal + restoration, `reproducibility=verified` |
| `S086-08` report-only commit + second CI | **PASS** | This commit changes exactly one path with its only parent equal to `b4c06630…`; pushed fast-forward; the fresh report-head two-host run and its kernel evidence are returned in the final agent handoff, since this file cannot embed its own identities |
| `S086-09` preservation + truthful cleanup | **PASS** | All branches preserved; three mainline commits and four tracked paths only; disposable scratch inventoried and removed; no self-ACCEPT; scope and remaining OPEN gates stated |


## 7. Canonical report-only `main` commit and second full CI (`S086-08`)

`.documents/Reports/execution-report.md` — the one canonical report — was overwritten with this complete factual WO-086 report. No dated report, parallel evidence folder, report archive or fifth WIP file was created. The report commit changes **exactly one path**, the canonical report, and its **only parent** is the accepted two-file integration commit `b4c06630ab79b3a38c57ae58eb0bc45f9f699665`. It was pushed to `origin/main` by ordinary fast-forward, after which `HEAD == main == origin/main` and the worktree was clean.

A **fresh** push-triggered `rust-verify` run at this exact report HEAD was required, on both `ubuntu-24.04` and `windows-2025`, with all original applicable steps green and with the real Linux `N085` kernel signal and refusal re-executing alongside the `N083`/`N081`/`N079`/`N076` regressions and the Windows refusal and cross-platform gates inspected in the logs. That run is distinct from integration run `43378355694` and from the original `WO-085` branch run `43363659723`. Because this file cannot contain the identity of its own not-yet-created commit, the report commit SHA, tree, report Git blob and both report-head run/job IDs are given in the Implementation Agent's final handoff message rather than fabricated here.

If the report-head CI had been red or absent, completion would not have been claimed and the truthful status would have been published without any unauthorised correction.

## 8. Preservation, cleanup, limitations (`S086-09`)

**Preservation.** One checkout, one worktree and the single `origin` were maintained for the whole run: no clone, no second worktree, no shadow checkout and no alternate remote. Proof branches `085`, `083`, `081`, `079`, `076` and older `061`, `069`, `070` remain at their exact §2 SHAs locally and on `origin`, with their history intact. No pull request, whole-branch merge, cherry-pick, tag, release, deployment, production media processing, AVIF decoder or public publication occurred. `solo-wo-085-r01` was read from, never written to.

**Cleanup — direct evidence, not inferred from `git status`.** This run's only task-owned artifacts were disposable evidence files under the single disposable directory `.temp/solo-wo-086-run/` (preflight captures, downloaded CI logs, the report draft) plus the Cline OS-temp task log area. They were inventoried explicitly and deleted by safe targeted deletion at run end, and `.temp/solo-wo-086-run/` was removed. The inherited directories `architecture-fixtures/`, `solo-wo-073-wip/` (still 4 files / 46 062 bytes, never imported, compiled, executed or staged — deliberately **not** treated as approved source) and `toolchain-installer/`, plus the existing `/target/`, were preserved untouched. No Owner, unknown or shared material was deleted or moved, and no `reset`, `clean`, stash or destructive Git operation was used against unknown content. `git status --porcelain` alone was not used as proof of ignored-scratch cleanup.


**Limitations, stated truthfully.**

- A passing CPU signal proves kernel enforcement for one trusted, single-threaded child that verified its own fixed ceilings, **not** protection from arbitrary workloads. There is no process-tree or descendant sweep, no group signal, no hostile-child control, no aggregate CPU budget, no RSS or aggregate memory budget, no Windows Job Object, no immutable executable identity, no sandbox, no network or credential confinement, and no media-correctness, publication, release or deployment authority.
- The observed `wall_ms=10004` is a measurement, not an assertion that the kernel fires at exactly 10 000 ms of wall time, and **wall time is not consumed CPU time**.
- The parent's 60 000 ms watchdog is a refusal path and was not exercised in a passing run; its pure validator cases do prove that a parent-produced status is refused.
- The Owner-local Windows workstation still has **no local linker** (`link.exe`), so no local compile, lint or test result exists. **All** compile, lint, unit-test, integration-test and kernel evidence in this report comes from the actual GitHub `ubuntu-24.04` and `windows-2025` runners at the exact SHAs named; the only local actions were Git operations, file inspection, filter-aware hashing and line-ending measurement.
- No genuine red CI iteration occurred during WO-086 itself: the issuance run, the integration run and the report-head run at these three mainline commits were each green on both hosts on their first push. The earlier `WO-085` branch red iteration (`43362920499`, Windows Clippy `dead_code`) is disclosed here for history and belongs to that order, not this one.

**Independent review.** The Implementation Agent has **not** self-accepted, has not merged or cherry-picked the accepted branch as a whole, has published no AVIF output, and has not released, deployed, tagged or begun another work order. The Chief Master Architect is asked to audit this canonical report, the accepted branch `solo-wo-085-r01` at `5bf9082b…`, the three mainline commits and their exact blob identities, and the issuance, integration and report-head runs, then return **ACCEPT / REVISE / STOP**.
