# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-062 R01` — repair the inherited `rustix` security advisory with the smallest pinned manifest/lockfile update, preserving the stopped R03.C1 branch.
**Owner handoff:** `SOLO-WO-062 R01` (matching).
**Session date:** 2026-11-11.
**Root:** the single verified Owner-local checkout of `provelopment/provelopment-foundation-solo` (one worktree, sole `origin`).
**Disposition:** **COMPLETE** for this bounded order. The pinned Linux `rustix` requirement is `=1.1.6`, the lockfile resolves exactly `rustix 1.1.6`, and fresh **Ubuntu/Windows** `rust-verify` runs are fully green at **both** the repaired substantive `main` revision and (see the chat handoff for its SHA) the final report revision. **No self-ACCEPT:** `R03.C`, Stage 03 and the retained `solo-wo-061-r01` branch remain open and unaccepted.

## 1. Verified starting anchors

| Anchor | Observed |
| --- | --- |
| `main` == `origin/main` | `9ed94120824bccc0b9bcba081544dc7cd8d11b95` ✔ |
| Tree | `371d4198643a67f282bd763ab35b7342909b7196` ✔ |
| Parent of the anchor commit | `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` ✔ |
| Canonical report blob at the anchor | `5bf95baa24ccaa1b1d51838560ca2e8f3e7c86ea` ✔ |
| `.documents/WIP/workorder-active.md` at the anchor | `5b1658f4d9c625aaa3fe8828501aebbaca6b31ea` — the frozen stopped `SOLO-WO-061 R01` |
| `.documents/WIP/plan-next.md` | `92fa562b1af0c54391851aa5c79deb1631127069` — neutral |
| `.documents/WIP/plan-roadmap.md` | `31a0ba2efb4f780f7248c56ff84683f20d9bc97f` — Stage 03 OPEN with R03.C |
| `.documents/WIP/plan-outlook.md` | `cb9c7a610cf3c537557e060ca9656e44a053e69d` — neutral |
| `crates/solo-adapters/Cargo.toml` | `a677df5f4b0f0d4fbd36d50c290e32f8cacc0847` |
| `Cargo.lock` | `8bd0bb00286a97853624ebbc0fe18da42a733f12` — 22 packages, `rustix 1.1.5` |
| `deny.toml` | `3063e864c28be1e77787f43f1b6f561d543c8f4a` — `unsound = "all"`, no ignored advisories |
| `.github/workflows/rust-verify.yml` | `eb78dad240bb7adb3e0ef9535870897176c99843` |
| `.gitignore` | `f45dd49524748e9b08f6678271edfc22ac39b97f` |
| Retained branch `solo-wo-061-r01` | local and remote both `4371af21dce67f33cf1df0384738bf353ca80fc6`; tree `89c6176a5f159069eccab9eb0d4ed67e88eaba4f`; parent `dcaab733…`; exactly five R03.C1 paths |
| Working tree before the issuance write | only `M .documents/WIP/workorder-active.md` — the Owner-installed `SOLO-WO-062 R01` (blob `ff903266e18581ffe9c705f36792376fcae2a02a`, 24,263 bytes); no merge/rebase/cherry-pick state; no untracked material |

The installed file is the sole authority: `OWNER-ISSUED — SOLO-WO-062 R01`, matching handoff, exactly the approved four-path scope.

## 2. Issuance revision and the expected pre-fix red gate

| Item | Value |
| --- | --- |
| Commit | `123241651bf217fface9c3cd7fb16d69f663e3a6` |
| Tree | `68675b9377f301a7aae1267c7ca5477777031f19` |
| Paths | `.documents/WIP/workorder-active.md` only (`ff903266…`) |
| CI run | **`42998476355`** — **failure**, as pre-authorized |
| Jobs | `130448394776` `verify (windows-2025)` — failure; `130448395236` `verify (ubuntu-24.04)` — failure |

Both jobs passed setup, the pinned-toolchain assertion and the locked fetch, then failed at the **component security gate** and nowhere else:

```
error[unsound]: xattr functions mark unwritten bytes as initialized when given a zero-length buffer
  ├ ID: RUSTSEC-2026-0486   ├ Solution: Upgrade to >=1.1.6
advisories FAILED: 1 errors, 0 warnings, 0 notes
```

Exactly **one** advisory error and no other material failure; 24 (Windows) and 23 (Ubuntu) later steps were skipped, so no downstream result from that run is inferred or reported as a pass. This is recorded as the disclosed inherited pre-fix observation — the gate was not disabled, narrowed, skipped or masked.

## 3. The minimal repair on `main`

| Item | Value |
| --- | --- |
| Substantive commit | `65656f111b4d8ef8012fc3d394a316193a54ad59` |
| Tree | `87768ff2456ffbeb7fd9e0253d87d7f1f77f8eb9` |
| Paths | `crates/solo-adapters/Cargo.toml` (`6109224cd3e3e89487884631f2b07c68ad8149c3`), `Cargo.lock` (`2f9e53576df117f2020ae064d6973304f20833f1`) — nothing else |

**Manifest diff (one token, one line):**

```
 [target.'cfg(target_os = "linux")'.dependencies]
-rustix = { version = "=1.1.5", default-features = false, features = ["fs", "std"] }
+rustix = { version = "=1.1.6", default-features = false, features = ["fs", "std"] }
```

The target guard, `default-features = false`, `features = ["fs", "std"]`, the Windows `windows-sys` block and every comment are byte-unchanged.

**Lockfile diff (`cargo update -p rustix --precise 1.1.6`, Cargo-native):**

```
 name = "rustix"
-version = "1.1.5"
+version = "1.1.6"
 source = "registry+https://github.com/rust-lang/crates.io-index"
-checksum = "891efababe418670775f199f0d233d84843c227a0949a883ce15b37c78d6629d"
+checksum = "e5e76742e5294153d4dde1207dbec5bdd81eef6fabdc0d2118babadabfd858aa"
```

Only the `rustix` version and its authentic registry checksum changed; the dependency edge list is identical and no other package record moved. The graph is still **22 packages** (4 first-party, 18 external); no new component, source or registry appeared.

**Local resolution evidence (pinned toolchain):**

- `cargo metadata --locked --offline`: 22 packages, 4 workspace members, exactly **one** `rustix` entry at `1.1.6`, features `fs` and `std` present, `license = Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`, `rust_version = 1.65`.
- Linux target graph (`cargo tree --locked --target x86_64-unknown-linux-gnu -e normal -p solo-adapters`): `rustix v1.1.6 → bitflags v2.13.2, linux-raw-sys v0.12.1`.
- Windows target graph (`--target x86_64-pc-windows-msvc`): `windows-sys v0.61.2 → windows-link v0.2.1`, with **no** `rustix`; the Windows path is version-unchanged.
- API compatibility checked against the actual downloaded 1.1.6 source (`rustix::fs::openat2`, `open`, `ResolveFlags` with `RESOLVE_BENEATH`, `RESOLVE_NO_SYMLINKS`, `RESOLVE_NO_MAGICLINKS`, `RESOLVE_NO_XDEV`, `OFlags`, `Errno` all present), so the confinement call sites needed no change.

## 4. Fresh A1 security and compatibility review (2026-11-11)

| Item | Finding |
| --- | --- |
| `RUSTSEC-2026-0486` (re-read `16:55:37 +07:00`) | `package = rustix`, `date = 2026-10-27`, `informational = "unsound"`, `aliases = ["GHSA-jh2m-fhpj-3vr7"]`, `patched = [">= 1.1.6"]`; the ordered fix is the published patch |
| `rustix 1.1.6` metadata | not yanked; published 2026-10-27 by `sunfishcode`; checksum `e5e76742…`; licence family unchanged; MSRV **1.65** (pinned Rust 1.99.1 is far above) |
| Dependency requirements of 1.1.6 | `bitflags ^2.9.2`, `errno ^0.3.10`, `libc ^0.2.182`, `linux-raw-sys ^0.12`, `windows-sys >=0.52, <0.62` — every edge already satisfied by the previously locked versions, which is why the resolution moved only `rustix` |
| OSV sweep after the change (`16:55:38 +07:00`) | all **18** external locked packages, including `rustix 1.1.6`, returned **no** advisory |
| Behavioural scope | `rustix` remains a normal Linux-target dependency of `solo-adapters` powering the `openat2` confinement boundary; no `xattr` call site exists in Solo, and the advisory's functions remain unreached, but the unchanged policy denies the locked crate itself |
| Unchanged policy | `deny.toml` untouched: `unsound = "all"`, no waived advisory, no ignore, no `[patch]`, no alternate registry, no `#[ignore]`, no CI change |
| Remaining non-actionable items | `itoa 1.0.18 → 1.0.19` and `syn 3.0.9 → 3.0.10` patch lags remain, explicitly outside this order; `windows-link 0.2.1` is a separate major series (`0.100.0`), not a patch lag, and carries no advisory |
| Inherited licence warning | `warning[license-not-encountered]: Unicode-3.0` at `deny.toml:51` is still emitted, non-fatally, on **both** hosts; it remains `UNVERIFIED` and was neither suppressed nor closed here |

## 5. Repaired `main` — full two-host gate

| Item | Value |
| --- | --- |
| CI run | **`42999372565`** at `65656f111b4d8ef8012fc3d394a316193a54ad59` — **success** |
| Jobs | `130451384598` `verify (ubuntu-24.04)` — success; `130451384047` `verify (windows-2025)` — success |

Per-host step evidence (each host runs its own 33 steps; only counterpart-host steps are skipped, exactly as before this change):

| Mandatory step | Ubuntu | Windows |
| --- | --- | --- |
| Pinned toolchain install/assert, locked fetch | success | success |
| Component security gate (`cargo-deny 0.20.2`, archived + verified) | success | success |
| Advisory negative probe (`RUSTSEC-2026-0009`) | success | n/a (Ubuntu-only) |
| Workspace package-set gate | success | success |
| Format check | success | success |
| Strict lint (warnings are errors) | success | success |
| Test (workspace, all targets, locked) | success | success |
| Architecture checker on the real checkout | success | success |
| Architecture refusal self-test | success | success |
| Test-failure tripwire | success | success |
| Release build (workspace, locked) | success | success |
| Release CLI smoke on the built binary | success | success |
| Two clean release builds compared byte-for-byte | success | n/a (Ubuntu-only) |

- **Security gate outcome:** `security_gate=pass` on both hosts, RustSec DB commit `31cf5a8673e10d3a4d7d829ad5095f47ed419049` fetched `09:56:43Z` (Ubuntu) and `09:57:07Z` (Windows); `advisories, bans, licenses and sources all ok`; `RUSTSEC-2026-0486` no longer appears for the locked `rustix 1.1.6`.
- **Test totals:** **370 passed / 0 failed / 0 ignored** on Ubuntu and **372 passed / 0 failed / 0 ignored** on Windows — the complete locked workspace suite across all targets, including the source-reader confinement, R01/R02/R03.A/R03.B regression and architecture suites. No test was added, removed, renamed, skipped or ignored by this order.
- **Warnings:** the inherited `license-not-encountered` warning for `Unicode-3.0` (`deny.toml:51`) is present on both hosts and remains non-fatal and `UNVERIFIED`. No other warning was observed in the mandatory gates.
- **Scope of this evidence:** a green repaired `main` proves the dependency repair only. The `solo-wo-061-r01` branch is **not** part of these runs; its R03C1-01–12 tests remain **NOT RUN / NOT VERIFIED** and are not accepted here.

## 6. Preserved retained branch (untouched)

| Item | Value |
| --- | --- |
| Branch | `solo-wo-061-r01` — local `4371af21dce67f33cf1df0384738bf353ca80fc6`, remote `origin/solo-wo-061-r01` `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Tree | `89c6176a5f159069eccab9eb0d4ed67e88eaba4f` |
| Parent | `dcaab733a56b0a95040e5a63fa3b2a0f2aa6d8ae` |
| Paths | exactly the five R03.C1 paths: `crates/solo-application/src/graphic_asset_inventory.rs`, `crates/solo-application/src/lib.rs`, `crates/solo-application/tests/graphic_asset_inventory.rs`, `crates/solo-adapters/tests/property_resolution.rs`, `crates/solo-domain/tests/property_resolution.rs` |

No merge, rebase, reset, cherry-pick, fast-forward, prune, rename or deletion was performed on that branch, and none of its commits entered `main`. Its source is still unaccepted and still unverified against the patched baseline; a separately Owner-issued, history-preserving reconciliation and full two-host retest is required before any independent ACCEPT of R03.C1.

## 7. Workspace, scratch and cache

| Item | Evidence |
| --- | --- |
| One checkout / one worktree | verified at every phase; no clone, shadow repo or nested repository |
| Solo `.temp/` | unchanged inherited baseline: 2 files, **289,253,806 bytes** (`architecture-fixtures/`, `toolchain-installer/`); this run added nothing there |
| Task scratch | one disposable `cargo metadata` JSON in the OS temporary directory, deleted immediately after reading; report composition parts deleted after assembly |
| Cline task temp area | empty |
| Root `target/` | live cache preserved and still ignored by `.gitignore:24` `/target/`; no cleanup, no ignore/config change |
| Owner material | nothing unknown deleted, reset or cleaned |

## 8. Final state at this report revision

| Item | Value |
| --- | --- |
| `main` / `origin/main` | identical; contains the issued ACTIVE (`SOLO-WO-062 R01`, frozen), the two-file `rustix` repair and this single canonical report |
| WIP slots | exactly four: ACTIVE `SOLO-WO-062 R01` (`ff903266…`), NEXT neutral (`92fa562b…`), ROADMAP unchanged (`31a0ba2e…`, Stage 03 OPEN, R03.C still listed), OUTLOOK unchanged (`cb9c7a61…`) |
| Product/source/tests/assets | unchanged by this order; no `platform/`, artwork, catalogue or media material created |
| Working tree | clean (`git status --porcelain` and `--untracked-files=all` both empty) |
| Stage 03 / R03.C | **OPEN** and not closed by this maintenance order |
| Release / deployment | none requested, none performed |

## 9. Handoff

Reported `COMPLETE` only for the bounded `SOLO-WO-062 R01` scope. The Chief Architect reads this report at the published final SHA, reviews the exact diffs and both-host logs above, and issues `ACCEPT`, `REVISE` or `STOP`. Any subsequent R03.C1 reconciliation is a new Owner-issued order; no automatic platform or graphic-role fallback, artwork, catalogue, release or Stage 03 closure is authorized by this order.
