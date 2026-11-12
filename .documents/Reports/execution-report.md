# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-082 R01` — integrate the independently accepted Linux wall-deadline leader-kill/reap negative onto `main` by exact two-blob promotion.
**Matching activation ID:** `SOLO-WO-082 R01`. **Date (ISO):** 2026-11-12.

**Outcome: `LINUX-WALL-DEADLINE-LEADER-REAP-INTEGRATED`.**

Exactly the two accepted Git blobs from technical-branch commit `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` are now on `main` with **object identity**, promoted by the order's literal path-scoped `git restore`. The complete, unmodified push-triggered `rust-verify` workflow passed on `ubuntu-24.04` and `windows-2025` at the exact integration commit `bbc92a34dac62ccf0a62c515798ca34984f3703e`, and a fresh run of that workflow also passed on both hosts at the final report-only `main` HEAD. At the new mainline SHA the real Ubuntu kernel again produced a `media_host_wall=terminated … signal=9 reaped=1` line with a **newly observed** pid and wall time, never a copied value.

**What is proven:** one *trusted, descendant-free* Linux child, launched under the accepted fixed `setsid`/`RLIMIT_AS`/`RLIMIT_CPU` pre-exec boundary, verified its own four ceiling halves and its own positive `pgid == pid` **before** a deliberate 20 000 ms sleep; its owning parent held a 2 000 ms wall deadline, called safe `Child::kill` on **its own leader only**, obtained a real `SIGKILL = 9` wait status, reaped that leader and emitted the fixed bounded evidence line only after validation. Windows refuses both wall modes before any child exists. `N076`/`N079` behavior is unchanged.

**Nonclaims — still OPEN:** no descendant or process-group signalling/sweep, no tree cleanup, no hostile-child resistance, no generic process runner, no memory-allocation or CPU-burn stress, no `RLIMIT_AS`/`RLIMIT_CPU` **exhaustion** proof, no aggregate or resident-memory quota, no Windows Job Object, no filesystem/network/credential sandbox, no immutable executable identity, no AVIF parse/decode or media verification, no publication authority, release, deployment or Stage 03 completion. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**, and the media-worker public-authority and media-correctness flags remain `false`. This report proves leader kill/reap only.

## 1. Preflight — `S082-01` PASS

Single Owner checkout `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`, one worktree, exactly one remote `origin` = `https://github.com/provelopment/provelopment-foundation-solo.git`, exactly four `.documents/WIP/` files, and the only dirtiness the Owner-installed ACTIVE replacement.

| Anchor | Required | Observed |
| --- | --- | --- |
| `HEAD == main == origin/main` | `13bf1c56dc1e974fb1acda7c86d9dca7fd74cae9` | exact, all three |
| `HEAD^{tree}` | `ec826f902736f0f2f3c4fc720fa6b36fe7ec3cb1` | exact |
| Prior canonical report blob | `e5bc3309c1e53610784620d0584f0fb3acd49765` | exact |
| Tracked pre-installation ACTIVE | `a9d2a84db8ef7cb2eb6d85d823aaefc0ecbf89b6` | exact |
| Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| `rust-verify` | `e1b5ca6e1738a5d5129a8788e24d51aee02421d1` | exact, both `edges=41` pins |
| `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Adapter manifest / root | `47d732f2…` / `1550d34f…` | exact |
| Linux native pre-exec seam | `d10151e40e807c9831491ba0da0eb44d08a81589` | exact |
| **Main's façade before promotion** | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | exact |
| CLI root | `b3c89ad0ce256f69e5763855559d20757554c461` | exact |
| **Main's host integration test before promotion** | `1f127001359e5f0ba941f92906f3280e433d8b9f` | exact |
| Unsafe guard / graph / workspace graph | `b88e7b42…` / `7a5beada…` / `25a91b2c…` | exact |
| Proof branch local and remote | `solo-wo-081-r01` = `cacc0f9b…`, tree `2dabf3e5…` | exact on both |
| Earlier proof branches | `solo-wo-079-r01` `380393dd…`, `solo-wo-076-r01` `40a40bab…` | unmoved |
| Historical branches | `4371af21…`, `e6162cdf…`, `d270f95c…` | unmoved |

The installed ACTIVE was 155 lines beginning `**OWNER-ISSUED — SOLO-WO-082 R01**` with three occurrences of the matching activation token, and its Git object identity was `76e7723e0471e500233b7471906a9cb32b0f48e3` before it was staged. Accepted-source checks confirmed the proof branch's issuance ancestor `e725c29a…` is contained in `cacc0f9b…`, that `e725c29a… → cacc0f9b…` yields **only** the two modifications (`775/4`, `255/0`), that the branch's ACTIVE is the prior issuance blob `a9d2a84d…` and its report is the WO-080 blob `346b8fe5…` — so **neither the branch's WIP nor its report was imported**. `main`'s history is report → WO-081 issuance → WO-080 report → WO-080 integration, and `main` code was still exactly the WO-080 integration. Inherited `.temp/solo-wo-073-wip/` is 4 files / 46 062 bytes and an existing `target/` is present; both were preserved and neither was executed, staged or treated as accepted source. No unknown tracked, staged or untracked item existed at any point.

## 2. Mainline issuance — `S082-02` PASS

| Step | Commit | Tree | Parent | Paths |
| --- | --- | --- | --- | --- |
| Issuance (`Issue SOLO-WO-082 R01: integrate the accepted Linux wall-deadline leader kill and reap proof`) | `5b7ecb3c919302f3e3d6040c1467e21f2a3b3e89` | `c0ddb92425b866dba80475e7e810bbb7af9a2a13` | `13bf1c56dc1e974fb1acda7c86d9dca7fd74cae9` | 1 — ACTIVE only, new blob `76e7723e0471e500233b7471906a9cb32b0f48e3` |

One commit, one parent, one changed path, no implementation and no report. Ordinary fast-forward push; the GitHub API read back the same `sha`, `tree`, `parent`, the single file, and the new ACTIVE blob `76e7723e…`.

**Issuance CI — run `43249672836`, `event=push`, `head_sha=5b7ecb3c…`, conclusion `success`:** `verify (ubuntu-24.04)` job `131291722791` and `verify (windows-2025)` job `131291722303`, **33 configured steps each, 0 failures**. Ubuntu skipped exactly the 9 Windows-only steps; Windows skipped exactly the 14 Ubuntu-only steps. Nothing else was skipped and no previous or branch run was reused. After issuance `HEAD == main == origin/main == 5b7ecb3c…` with clean index and working tree, and no branch was created.

## 3. Two-blob promotion — `S082-03` PASS

From clean `main` at the issuance commit, the order's authorized mechanism was used verbatim with literal paths:

```text
git restore --source=cacc0f9b10f6b43c38c4b72e8cda6e92310eef19 --staged --worktree -- crates/solo-adapters/src/media_worker_host.rs crates/solo-cli/tests/media_worker_host_smoke.rs
```

No merge, cherry-pick, rebase, whole-tree checkout, reset, stash, force push, additional worktree, `.temp` copy or byte edit was used, and **no rustfmt was applied** to the promoted files.

| Path | Main blob before | Accepted blob now | Numstat |
| --- | --- | --- | --- |
| `crates/solo-adapters/src/media_worker_host.rs` | `97efbbf0c7de425cd00f79f126ed05c9f8b13e8d` | **`aba0cb1acf646f8d6a3e9d459cf2a02f90b993a0`** | `775 / 4` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | `1f127001359e5f0ba941f92906f3280e433d8b9f` | **`1846af8459c21063bd2c2d7e112c05b9baf599ae`** | `255 / 0` |

All five pre-commit proofs were recorded before committing:

1. `git diff --cached --name-status` was exactly those two `M` paths — nothing added, deleted or third.
2. `git diff --cached --numstat` was exactly `775/4` and `255/0`, and `git diff --cached --check` was clean.
3. `git rev-parse :<path>` returned `aba0cb1a…` and `1846af84…`.
4. Those two index blobs equal `git rev-parse cacc0f9b…:<path>` individually, and the `--worktree` objects hashed identically too, so no line-ending or formatting drift exists.
5. `git ls-files -o --exclude-standard` was empty and no unrelated path was modified or staged.

| Step | Commit | Tree | Parent | Paths |
| --- | --- | --- | --- | --- |
| Integration (`feat: integrate proven Linux wall deadline leader reap (SOLO-WO-082)`) | `bbc92a34dac62ccf0a62c515798ca34984f3703e` | `e7724efc2a2fbb3afbf6131453b88dcb38075fb7` | `5b7ecb3c919302f3e3d6040c1467e21f2a3b3e89` | exactly 2 `M`, `775/255` additions, `4/0` deletions |

Ordinary fast-forward push; GitHub readback returned the same SHA, tree, parent equal to the issuance commit, exactly two files with additions `775,255` and deletions `4,0`, and remote blob identities `aba0cb1a…` / `1846af84…`. Local `HEAD`, `main`, `origin/main` and the GitHub commit all agree, and both index and working tree were clean afterwards. No accepted byte was patched to make a test pass, because none needed to be: the accepted objects are on `main` unaltered.

## 4. Complete permanent path ledger

Exactly four paths changed across the entire order, in three commits:

| Path | Commit(s) | Note |
| --- | --- | --- |
| `.documents/WIP/workorder-active.md` | issuance only | installed Owner bytes, blob `76e7723e…` |
| `crates/solo-adapters/src/media_worker_host.rs` | integration only | promoted blob `aba0cb1a…` |
| `crates/solo-cli/tests/media_worker_host_smoke.rs` | integration only | promoted blob `1846af84…` |
| `.documents/Reports/execution-report.md` | report-only | this report |

Everything else stayed byte-identical at the integration commit: workflow `e1b5ca6e…`, `Cargo.lock` `b1ee56da…`, native seam `d10151e4…`, adapter root `1550d34f…`, CLI root `b3c89ad0…`, adapter manifest `47d732f2…`, unsafe guard `b88e7b42…`, graph checker `7a5beada…`, workspace graph `25a91b2c…`, ARCHITECTURE, all three WIP plans and every other source path. The accepted invariants hold: the one Linux native file still has exactly its four positively gated, locally justified `unsafe` expressions, and the two integrated files hold **zero** executable `unsafe` (the façade's only two occurrences of the word are prose in its module documentation). No new native FFI, dependency, feature, process abstraction, OS signal action, executable input, generic runner or public CLI contract was added; `killpg`, process-group signalling, descendant fork/spawn and cleanup claims remain absent. The pinned workflow still enforces `edges=41` with four workspace crates.

## 5. Mainline integration CI and fresh kernel evidence — `S082-05` … `S082-08` PASS

**Run `43250654169`, `event=push`, branch `main`, `head_sha=bbc92a34dac62ccf0a62c515798ca34984f3703e` (exact), conclusion `success`.** `verify (ubuntu-24.04)` job `131295215219` and `verify (windows-2025)` job `131295216142`, 33 configured steps each, **0 failures**; Ubuntu skipped only its 9 Windows-only steps and Windows only its 14 Ubuntu-only steps. Logs, not badges, were read for every observation below.

### `G082-01` — identity and integrity

GitHub's two-path ledger and both integrated blob SHAs equal §3; the workflow, `Cargo.lock`, native seam, adapter/CLI roots, guard, graph and workspace-graph blobs are still exactly the §1 values; the proof branch and all earlier/historical branches are unmoved; architecture reports `packages=4 targets=33 files=60 edges=41` on both hosts — so **zero** additional dependency edges were introduced.

### `G082-02` — real Ubuntu wall deadline at the *new mainline* SHA

```
n081_wall=media_host_wall=terminated as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4402 pgid=4402 signal=9 reaped=1 wall_ms=2003
n081_wall_timing=wall_ms=2003 case_elapsed_ms=2013 signal=9 reaped=1
```

`n081_01_the_wall_parent_kills_and_reaps_its_own_overdue_child` and `n081_02_the_termination_is_the_short_deadline_not_the_full_sleep` both ran and passed on `ubuntu-24.04`. The child's own pre-sleep verification is visible in the line itself: all four halves at full width (`as_soft = as_hard = 536870912`, `cpu_soft = cpu_hard = 10`) and a positive `pid = pgid = 4402`. The termination is `signal=9` with `reaped=1`, and the parent's own measurement is `wall_ms = 2003` — 3 ms past the configured 2 000 ms deadline, `case_elapsed_ms = 2013`, and about 18 seconds short of the child's 20 000 ms sleep, so a normal completion cannot explain it. **These are newly observed values for this run: the pid, wall time and case elapsed time all differ from the WO-081 branch run (`4489`, `2008`, `2018`) and were not copied.** Leader kill and reap only; no group or tree sweep is claimed.

### `G082-03` — Unix direct negative and parser refusals

`n081_03_a_direct_wall_child_and_every_wrong_wall_token_are_refused` passed on Ubuntu with `n081_direct_wall_child=exit=71`: a direct, unsupervised `wall-child` refused at its own limit/identity check with zero stdout, zero stderr and no 20-second sleep, and five wrong token shapes still refused at status `3` with `INTERNAL_MODE_INVALID` while the legacy `child` mode kept its exact prior status. The pure cross-platform cases ran on **both** hosts and passed unchanged: `n081_01_the_wall_negative_uses_the_fixed_deadline_and_profile`, `n081_03_the_wall_tokens_are_closed_and_additive`, and the four `n081_04` cases covering malformed and partial readiness, out-of-profile and impossible identity, non-`SIGKILL` and un-reaped status, and the fixed terminated-only evidence wording. No test or assertion was redefined.

### `G082-04` — Windows fail-closed

`n081_05_both_wall_modes_are_unsupported_before_any_child_exists` ran and passed on `windows-2025`: both wall modes returned status **3** with exactly `error: MEDIA_HOST_PROBE_REFUSED: HOST_UNSUPPORTED` and empty stdout, promptly and without creating a child. The Windows log contains **zero** occurrences of `n081_wall=`, `n081_01_the_wall_parent…`, `n081_02_…`, `n081_03_a_direct…`, `n079_01_the_real…`, `n079_04_…` or `n076_`: no Linux native module, no real Linux wall test and no kernel readback executed there. The pure cross-platform refusals and `n079_05`/`n079_06` did run and pass.

### `G082-05` — all accepted regressions

`N076-01`, `N076-02` and `N076-03` passed on Ubuntu with real `getrlimit` readback. `N079-01` passed with the benign pre-exec child observation `n079_observation=media_host_probe=accepted as_soft=536870912 as_hard=536870912 cpu_soft=10 cpu_hard=10 pid=4387 pgid=4387 wall_ms=11` (512 MiB, 10 CPU-seconds, `pgid == pid`), and `N079-04` passed with `n079_direct_child=exit=71`. `N079-03` (including the real-binary token-shape cases), `N079-05`, `N079-06` and the unchanged public CLI bytes/exits (`preflight=ok entity_id=entity-7 base=https://example.com/alpha` at `0`; `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start` at `2`) passed on the appropriate hosts. No process group, descendant, stress allocation, CPU burn or decoder was introduced.

## 6. Architecture, security and downstream gates — `G082-06`, `G082-07` PASS

| Gate | Ubuntu `131295215219` | Windows `131295216142` |
| --- | --- | --- |
| Pinned toolchain `1.99.1`, `cargo fmt --all -- --check`, strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, workspace locked all-target tests | pass | pass |
| Real-tree architecture | `architecture=legal packages=4 targets=33 files=60 edges=41` | identical |
| Refusal self-test | `self-check=passed cases=29`, negative dynamic-load/bypass refusals still refusing | `self-check=passed cases=29` |
| Component security gate `cargo-deny 0.20.2` | archive sha256 `9f12ed4c…`, RustSec `af7b3ec33460b8e2de8d422e16d628518891a519` fetched `2026-11-12T15:42:32Z`, advisories/bans/licenses/sources all ok | archive sha256 `975a2214…`, same RustSec commit fetched `2026-11-12T15:42:57Z`, all ok |
| Advisory negative probe (RUSTSEC-2026-0009) | refused as designed | not applicable to this host |
| Test-failure tripwire | pass, injected failure detected | pass |
| Release build + release CLI smoke | `release_smoke=verified accepted_exit=0 refused_exit=2` | `release_smoke=verified accepted_exit=0 refused_exit=2` |
| Forbidden import / bypass refusal with guaranteed restore | `forbidden_import_probe=verified injected_exit=1 restored_sha256=8b627ffeadd231f36515ada4ee691e401c2aa0de7cdec3619f736280c837b4ba` | Ubuntu-only step |
| Two clean release builds compared on raw bytes | `reproducibility=verified`, `clean_release_bytes=641112` | Ubuntu-only step |
| Raw checkout `Cargo.lock` SHA-256 | `bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` | `afffb921f61318c10308fd087266f764960ab914a8b2f5a34996809755dfb70e` |

The two host-specific raw lock hashes differ only because of checkout line endings; the single Git blob identity `b1ee56da60f36536548586528db371b7cbe508e4` is unchanged on both, and the security gate itself re-verified that hash before and after running. This is CI test-building, never release or deployment authority.

## 7. Gate table

| Gate | Status | Basis |
| --- | --- | --- |
| `S082-01` | **PASS** | §1 — every commit, tree and blob exact; only the installed ACTIVE dirty; accepted branch and both target blobs confirmed; scratch preserved |
| `S082-02` | **PASS** | §2 — ACTIVE-only issuance `5b7ecb3c…`, exact remote readback, fresh push CI green on both hosts, no branch created |
| `S082-03` | **PASS** | §3 — literal path-scoped restore, five pre-commit proofs, two-path integration `bbc92a34…`, parent = issuance, remote blobs equal accepted |
| `S082-04` | **PASS** | §4 — four-path ledger only; workflow, lock, native seam, roots, guards and graph immutable; zero executable `unsafe` added |
| `S082-05` | **PASS** | §5 — integration CI green on both hosts at the exact SHA with fresh Linux kill/reap evidence |
| `S082-06` | **PASS** | §5 — Windows fail-closed negative with zero Linux execution, legacy public/private CLI unchanged |
| `S082-07` | **PASS** | §5/§6 — `N076`/`N079` regressions, architecture, self-test, security, tripwire, release smoke, restore and reproducibility all green |
| `S082-08` | **PASS** | §5 — CI exactness: push-triggered run at the integration SHA *and* the final report HEAD, no earlier run reused |
| `S082-09` | **PASS** | §8 — one report-only commit, GitHub readback, clean end state, and a fresh green two-host run at the report HEAD |

No gate is `FAIL`, `UNVERIFIED` or `BLOCKED`. **No WO-082 failure occurred.** The earlier WO-081 red lint iteration on the *technical branch* (`43238905991`, corrected before the accepted revision `cacc0f9b…`) is disclosed history, not a WO-082 failure, and it was not re-run or re-litigated here.

## 8. Final state, limitations, cleanup and handoff

**Repository.** Single checkout, one worktree, exactly one remote `origin`, exactly four `.documents/WIP/` files, clean index and working tree. `main` carries the issuance `5b7ecb3c…`, the integration `bbc92a34…` and this report commit, in that order, with no branch merge anywhere in the history. The accepted technical branch `solo-wo-081-r01` is preserved and unaltered at `cacc0f9b10f6b43c38c4b72e8cda6e92310eef19` both locally and on `origin`; `solo-wo-079-r01` (`380393dd…`), `solo-wo-076-r01` (`40a40bab…`), `solo-wo-061-r01` (`4371af21…`), `solo-wo-069-r01` (`e6162cdf…`) and `solo-wo-070-r01` (`d270f95c…`) are byte-for-byte unchanged. No branch was created, moved, rewritten or force-pushed, and no pull request, tag, release or deployment was made.

**Local limitation.** This Owner workstation still cannot link (`cargo check` fails inside dependency build scripts because `link.exe` is absent), so **every** compile, lint, test, security and kernel statement in this report comes from the two-host GitHub CI at the exact SHAs named. Locally only Git-level proofs were possible — object-identity comparison, `git diff --cached --check`, and the readback of every blob — and they are reported as such rather than as compilation evidence.

**Cleanup.** The only task-owned temporary material this run created was disposable report scratch under the ignored Solo `.temp/`, removed after assembly; `.temp/` again has no top-level files. The inherited `architecture-fixtures`, `solo-wo-073-wip` (4 files / 46 062 bytes) and `toolchain-installer` directories are preserved exactly as found and remain unverified reference only — not executed, not staged, not treated as accepted source. The existing `target/` was left untouched. No Owner or unknown file was deleted, moved or reinterpreted, and no `git reset`, `git clean`, stash, secondary worktree, clone, shadow repository or force push was used. No agent temp area outside Solo was needed.

**Remaining risks.** Descendant and process-group handling, tree sweep/cleanup, hostile-child resistance, aggregate or resident memory quotas, `RLIMIT_AS`/`RLIMIT_CPU` exhaustion, Windows Job Objects, filesystem/network/credential confinement, immutable executable identity, decoder quality and media correctness, publication and Stage 03 all remain future Owner-issued work. This order integrated two already-accepted, byte-identical blobs and proved no new capability; the accepted `unsafe` inventory, dependency graph, workflow and public contracts are unchanged.

The Implementation Agent has **not** self-ACCEPTed and has not begun another order. Independent Chief Master Architect disposition — **ACCEPT / REVISE / STOP** — is requested against the final report HEAD, the exact integrated blobs and both green two-host runs. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** regardless of this narrow integration outcome.
