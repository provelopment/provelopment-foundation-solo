# SOLO-WO-117 R01 — INTEGRATE EXACT VERIFIED SVG V3 TECHNICAL BLOBS INTO MAIN; PROVE FULL TWO-HOST CI

**STATUS: OWNER-ISSUED — EXECUTE THIS WORK ORDER.**  
**Date:** 2026-11-15  
**Repository:** `provelopment/provelopment-foundation-solo`  
**Owner checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Active order:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo\.documents\WIP\workorder-active.md`  
**Required outcome:** `R03C-SVG-V3-EXACT-BLOB-MAIN-INTEGRATION-TWO-HOST-VERIFIED`.

## 1. Mission and fixed inputs

Integrate **exactly** the 23 verified technical files at source commit `d82ae76273b57b37014d3b4c557b8debd5299eea` from the existing `solo-wo-115-avif-svg-v3-r02` branch into `main`, without merging branch history, creating a new branch, changing the technical branch, reimplementing code, or importing any technical-branch WIP/governance files. Prove complete native Ubuntu 24.04 and Windows 2025 `rust-verify.yml` on one integrated `main` commit. Publish the single canonical report; verify baseline CI again at its report commit.

Authoritative starting remote tips:

- `main`: `da63eaa2e732c2587856cee3bfbce36bc34a27f1` (`SOLO-WO-116 R04` report, R7).
- Technical source: `solo-wo-115-avif-svg-v3-r02` at `d82ae76273b57b37014d3b4c557b8debd5299eea`, tree `b6daae33`.
- Hosted technical proof: GitHub Actions run `43674236111`, Ubuntu job `132690745816`, Windows job `132690745654`, both successful at that exact technical SHA. Its N116 4/4, six mutation checks, RED/GREEN and full retained gates are the accepted **technical input evidence**; this order is the fresh integration authorization and proof.
- Hosted current-main proof: run `43674735216`, successful on both hosts at `da63eaa2`.

The exact source includes the justified architecture checker `edges=81` assertions on both hosts and the fixed N116 gate. The verified dependency allowlist admits only the approved adapter normal roots `quick-xml`/`svgtypes` and exactly 47 adapter-closure package names. These exact source bytes are **authorized as the integration input of this work order**. Do not re-edit or weaken them.

Read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` (especially §§5, 5A, 6.2G/I/K, 6A.8), both SOPs, all four WIP files, and the current `.documents/Reports/execution-report.md` before acting. Keep all previously accepted contracts, A1 rules, security/source/architecture refusal gates, and the single Owner/Architect/Implementation Agent division of responsibility. No work outside this repository.

## 2. Hard preflight; safety before source mutation

Use the existing **one** checkout; work from local Windows 11 and/or WSL Ubuntu 26.04 on that same checkout, not a clone or second worktree. Confirm `git rev-parse --show-toplevel` is the actual Solo root, one worktree, branch `main`, exactly one remote `origin` pointing to `https://github.com/provelopment/provelopment-foundation-solo` (the equivalent `.git` remote is acceptable), empty stash, exactly four WIP files, and clean index/worktree **except** the Owner-installed new `workorder-active.md` content. Confirm `origin/main` is the exact pinned `da63eaa2` and the named technical remote branch is exactly pinned `d82ae762`. Inspect all unexpected local/remote changes; **STOP**, preserve them, and report. Never reset, rebase, force-push, `git clean`, delete unknown/Owner-owned files or temp, or add worktrees/remotes/branches.

Only the following commits on `main` are authorized:

1. **A8:** stage, commit and push **only** this exact Owner-installed `.documents/WIP/workorder-active.md`; verify working/index/remote Git blob identity. The published work order must be identical to the Owner-installed file. Do not carry older technical-branch ACTIVE text into main.
2. **I8:** integrate precisely the 23 file paths in §3 from the pinned technical SHA as identical Git blobs, commit and push only those 23 paths, then verify full hosted Linux/Windows CI at that **one I8 SHA**.
3. **R8:** overwrite, commit and push **only** `.documents/Reports/execution-report.md` with the complete current run evidence. Then verify a fresh native Linux/Windows baseline CI at the **one R8 SHA**. Do not put R8's own SHA inside its own report; give the SHA in the final handoff.

No pull request, merge commit, cherry-pick, tag, release, deployment, provider action, dependency upgrade, force-push, unauthorized code fix, or WIP/archive proliferation. Preserve the technical branch and all other remote refs unchanged. The authorization permits ordinary exact-path `git restore` for the §3 blob transfer from the pinned SHA on a verified clean `main` after A8; it does **not** permit restoring, overwriting or checking out `workorder-active.md` from the technical branch.

## 3. Exact 23-path integration ledger — no additions or omissions

Transfer these file contents from the **single** source commit `d82ae76273b57b37014d3b4c557b8debd5299eea`:

```text
.github/workflows/rust-verify.yml
Cargo.lock
crates/solo-adapters/Cargo.toml
crates/solo-adapters/src/graphic_image_inspection.rs
crates/solo-adapters/src/graphic_svg_inspection.rs
crates/solo-adapters/src/lib.rs
crates/solo-adapters/tests/property_resolution.rs
crates/solo-application/src/graphic_image_inspection.rs
crates/solo-application/src/lib.rs
crates/solo-cli/tests/architecture_checks/graph.rs
crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md
crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_NOirot.avif
crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_irot.avif
crates/solo-cli/tests/fixtures/avif/clap_irot_imir_non_essential.avif
crates/solo-cli/tests/fixtures/avif/fox-profile0.10bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/fox-profile0.8bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/fox-profile2.12bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/white_1x1.avif
crates/solo-cli/tests/fixtures/svg/TABLER-ICONS-MIT.txt
crates/solo-cli/tests/fixtures/svg/icon-home.svg
crates/solo-cli/tests/media_worker_host_smoke.rs
crates/solo-domain/src/property_registry.rs
crates/solo-domain/tests/property_resolution.rs
```

**Use byte-safe Git objects, not PowerShell `>` redirection for binary AVIF files.** After A8, while on clean `main`, the following Bash example (Git Bash or WSL) is the prescribed transfer. It must not run on an unverified dirty tree:

```bash
set -euo pipefail
TECH=d82ae76273b57b37014d3b4c557b8debd5299eea
mapfile -t paths <<'PATHS'
.github/workflows/rust-verify.yml
Cargo.lock
crates/solo-adapters/Cargo.toml
crates/solo-adapters/src/graphic_image_inspection.rs
crates/solo-adapters/src/graphic_svg_inspection.rs
crates/solo-adapters/src/lib.rs
crates/solo-adapters/tests/property_resolution.rs
crates/solo-application/src/graphic_image_inspection.rs
crates/solo-application/src/lib.rs
crates/solo-cli/tests/architecture_checks/graph.rs
crates/solo-cli/tests/fixtures/avif/LICENSE-AND-PROVENANCE.md
crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_NOirot.avif
crates/solo-cli/tests/fixtures/avif/abc_color_irot_alpha_irot.avif
crates/solo-cli/tests/fixtures/avif/clap_irot_imir_non_essential.avif
crates/solo-cli/tests/fixtures/avif/fox-profile0.10bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/fox-profile0.8bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/fox-profile2.12bpc.yuv420.avif
crates/solo-cli/tests/fixtures/avif/white_1x1.avif
crates/solo-cli/tests/fixtures/svg/TABLER-ICONS-MIT.txt
crates/solo-cli/tests/fixtures/svg/icon-home.svg
crates/solo-cli/tests/media_worker_host_smoke.rs
crates/solo-domain/src/property_registry.rs
crates/solo-domain/tests/property_resolution.rs
PATHS
[ "${#paths[@]}" -eq 23 ]
[ "$(git branch --show-current)" = main ]
[ -z "$(git status --porcelain)" ]
for p in "${paths[@]}"; do git cat-file -e "$TECH:$p"; done

git restore --source="$TECH" --staged --worktree -- "${paths[@]}"
# Verify the exact changed-path set, then every index blob is byte-identical.
diff -u <(printf '%s\n' "${paths[@]}" | LC_ALL=C sort) \
        <(git diff --cached --name-only | LC_ALL=C sort)
for p in "${paths[@]}"; do
  expected=$(git rev-parse "$TECH:$p")
  actual=$(git rev-parse ":$p")
  [ "$expected" = "$actual" ] || { echo "BLOB MISMATCH: $p" >&2; exit 1; }
done
# Run §4 LOCAL checks before committing I8 or dispatching hosted CI.
```

This exact-path list includes ten donor fixture/provenance files (eight AVIF paths and two SVG paths). Never import `.documents/WIP/workorder-active.md` from the technical branch. Do not import the technical branch's commit ancestry or change the source branch. Do not touch any other file, including other WIP plans, architecture/SOPs, `deny.toml`, root `Cargo.toml`, source/unsafe checkers, and unchanged files.

## 4. Local-first tests and exact acceptance gates

**First, before GitHub CI,** run all applicable checks locally on Windows 11 and/or WSL Ubuntu 26.04 against the staged 23-file integration, retaining the exact command, environment, exit status and relevant output. In the approved checkout:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test -p solo-cli --test workspace_graph --locked
cargo test -p solo-cli --test check_architecture_fixtures --locked
cargo test -p solo-cli --test check_architecture --locked
cargo test -p solo-cli --test media_worker_host_smoke --locked n116_02 -- --nocapture
cargo test -p solo-cli --test media_worker_host_smoke --locked n116_04 -- --nocapture
cargo test -p solo-adapters --locked
```

Run the full `cargo test --workspace --all-targets --locked` on any local host where its real-source/snapshot test prerequisites are present; run the complete N113/N116 native-host subset there as well. WSL without an actual prepared `SOLO_TEST_SOURCE_FS_ROOT` Btrfs fixture may legitimately be unable to execute `n113_`, `n116_01`, `n116_03` and five native snapshot tests. State those as **NOT RUN LOCALLY** and rely on genuinely executed hosted Ubuntu verification; do not bypass, ignore or fake them. Use the Windows local machine for additional native checks where runnable, sequentially on the same checkout rather than making a second worktree. Fixing product code is **not authorized in this exact-blob integration order**: a surprising failure is STOP with diagnostics, not an improvised patch.

Verify the imported workflow still has N116 four exact tests, two host gate steps plus RED, six mutated-log rejections, the independently pinned canonical SVG digests, and retained exact real-tree `edges=81` checks. Verify `Cargo.lock` is identical to the pinned technical source, the approved `quick-xml =0.42.0` and `svgtypes =0.16.1` normal roots occur only in `solo-adapters`, the 47-name closure still fails closed, and both negative architecture fixtures remain effective. No relaxed CI check, missing/ignored canary, hidden error, changed fixture byte, uncontrolled dependency, or unreviewed warning is allowed.

**After local checks pass:** commit I8, confirm all 23 `HEAD:path` Git blob SHAs exactly equal `TECH:path`, verify `origin/main` still matches the expected predecessor, push by ordinary fast-forward, and examine the fresh `rust-verify.yml` workflow triggered by that `main` push. Obtain **one complete native Ubuntu 24.04 + Windows 2025 run at identical I8 SHA**, both jobs `success`, including complete locked workspace tests, architecture/source/unsafe/security gates, RustSec negative probe, N102/N108/N113, N116 exact 4/4 plus six mutations and deliberate RED/GREEN, all host snapshots and reproduction gates, V2 and V3 evidence, and the `edges=81` checker. Inspect actual logs, not only the green badge. Confirm both pinned SVG digests match across hosts and every essential step was actually reached. Do not count legitimate platform-inapplicable steps as failures or as performed work.

After I8 proof, overwrite the **single** canonical `.documents/Reports/execution-report.md` with the run's full evidence and an explicit `status=COMPLETE — pending independent Chief Master Architect review`, commit/push only that report as R8, and verify fresh Linux/Windows hosted CI at the exact R8 SHA. If an error or unresolved scope/authority issue occurs at any phase, STOP safely, leave no unauthorized changes and publish an honest `STOPPED` report whenever the report-only safety gate permits.

## 5. Required report and explicit exclusions

The report must identify A8, I8 and R8 commits, parents, trees, exact paths and blob equality; source and destination tips; local versions/commands/results and any Btrfs limits; two-host run IDs, jobs, attempts, SHAs, step conclusions, complete N116 evidence including process IDs and per-host digests; retained negative/refusal/security/reproducibility gates; all warnings and skips, and local cleanliness/temporary cleanup. Preserve exactly four WIP files and 27 existing branch refs with no unapproved changes. No reporting archive and no reference to a second checkout. Give the final report commit SHA in the Implementation Agent chat handoff only.

**Do not** implement real AVIF decoding, modify graphics policy, add platform assets, complete `R03.C`, close Stage 03, build a browser/HTTP route, change the contact system, publish a release, create a PR, deploy, or use a provider. AVIF still refuses before any worker; the SVG integration is one bounded step of the open Stage 03. Following a successful report I will independently inspect the published report, exact I8/R8 code, tests and two-host logs and return `ACCEPT / REVISE / STOP`; work on AVIF and platform catalogue remains separately sequenced.
