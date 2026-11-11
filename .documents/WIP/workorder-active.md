# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-058 R01**  
**Title:** Resolve the accepted R03.A closure's remaining clean-tree gate by narrowly ignoring the live root Rust build cache  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Owner handoff ID:** `SOLO-WO-058 R01`  
**Approved Scope:** Perform the exact root `.gitignore` hygiene change, final-state verification, ACTIVE reconciliation and canonical-report publication specified here. The Owner has explicitly authorized this work following the Chief Architect's `REVISE` review of `SOLO-WO-057 R01`. This is an **operational Owner-issued work order**, not an R03.B implementation order. The Chief Architect reviews GitHub **read-only** and never makes repository writes.

## 1. Authorized outcome and acceptance basis

`SOLO-WO-057 R01` administratively closed the independently accepted R03.A package and selected R03.B as the sole NEXT candidate; its WIP/documentation changes and safe `.temp/` cleanup were independently corroborated. The remaining mandatory verification gate was explicitly `UNMET`: the local IDE's actively generated, untracked **root** `target/` Rust build cache appeared as `?? target/` in `git status --porcelain`. Deleting the live cache was intentionally refused to avoid disrupting `rust-analyzer` and needless rebuilding. This order authorizes the narrow, durable repository-appropriate correction: **ignore only the repository-root `/target/` directory** while preserving the cache itself.

**Reviewed starting `main` and `origin/main`:** `cb9efab41103c8e8aabdb882c453bf43f96c481a` (tree `96b1cf233eac83e1d1c57365f7adb8baeccf5f43`). The report blob at that revision is `a41f72d828f46127883ed2953408da68424ffbce`. The previously accepted R03.A implementation is the ancestor `fa724977f4e7b76493c386da3501f92e1d2206e7`; the R03.A administrative closure commit is `4626ef8de0fa3168e6aeb98a53dd8cf95fbc2f8f`. Existing two-host report CI at `cb9efab` is green, run `42952425852`. These refs are acceptance/preflight anchors, not permission to rewrite history.

The complete requested outcome is:

1. Leave the real, live root `target/` directory and its producing IDE process undisturbed; add a **root-anchored `/target/`** ignore entry, with one explanatory comment, to the existing root `.gitignore`.
2. Prove an **actually empty `git status --porcelain`** (and full untracked-file view) after the tracked changes are committed, without deleting/moving the cache, altering local `.git/info/exclude`, changing global Git ignore configuration or hiding any other untracked item.
3. Preserve all R03.A closure outcomes: neutral ACTIVE at the end; R03.B remains the **sole NEXT planning candidate**, not authorized code work; Stage 03 stays open for R03.B and R03.C; ROADMAP, OUTLOOK and accepted R03.A product code remain unchanged.
4. Record the final reconciliation and the previously disclosed `Unicode-3.0` cargo-deny warning honestly in the **single canonical execution report**; do not silently fix, suppress or ignore the pre-existing warning.
5. Publish ordinary commits to the existing `main`, obtain and inspect unchanged two-host CI for each required published checkpoint, and supply the exact final evidence for independent Architect disposition.

**This is a bounded hygiene and verification order.** It does not retroactively modify the contents or claimed execution of `SOLO-WO-057`; its outcome provides a new verified clean-tree state against which the Architect may close the outstanding disposition. It grants no R03.B/R03.C, product-feature, release, deployment, dependency or policy-expansion authority.

## 2. Required preflight and preserved state

Before any permanent write, read `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, relevant `.documents/Architecture/ARCHITECTURE.md` §§1, 5A, 6.2G–6.2I, both `.documents/SOP/sop-architect.md` (§§6, 6A, 6B, 7A, 7B, 8) and `.documents/SOP/sop-implementation.md` (§§4–5C, 8–11), the four WIP files, the current canonical execution report **from GitHub at the exact reviewed SHA**, `.gitignore`, `.cargo/config.toml` and the unchanged CI workflow. Follow the matching Owner handoff naming this ID. The SOP and exact authorized path ledger govern all execution; neither NEXT nor the earlier REVISE gives independent implementation permission.

Verify and record all of the following:

- The real local Git root is **exactly** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`; one checkout, one worktree, sole `origin` pointing to the stated repository, no shadow checkout or nested Git repository. On the existing `main`, both local HEAD and the fetched `origin/main` are `cb9efab41103c8e8aabdb882c453bf43f96c481a`. If the remote/baseline has moved, **STOP** and report, rather than rebase, reset or reinterpret the approved scope.
- The Owner-installed local `workorder-active.md` is exactly this **`OWNER-ISSUED — SOLO-WO-058 R01`** document. The only expected tracked modification before the initial authorization commit is that Owner installation. Do not overwrite any other dirty, staged or untracked Owner material.
- Current original GitHub blobs: root `.gitignore` `f51cc5306cdc8e4532422809eede68b93c4a7fd7`; neutral ACTIVE `ea5f20b736483c2e102456be1cea9104059fc81a`; ROADMAP `754ce1b75917d3168457965c3efed1137ff3aa9c`; NEXT `251c5dcc40f52c7c0a214aaee9daa2518b5cd887`; OUTLOOK `cb9c7a610cf3c537557e060ca9656e44a053e69d`; report `a41f72d828f46127883ed2953408da68424ffbce`. WIP has exactly four files and no parallel register/archive.
- If root `target/` still exists, establish that it is the reproducible Rust/Cargo/**rust-analyzer flycheck** build cache identified in the `SOLO-WO-057` report; check safely for unexpected Owner content, nested `.git`, reparse points or path redirection, and for any tracked/staged files under it. Its last reported size was **29,640,743 bytes** (about 28.27 MiB); remeasure without assuming size, file count, PID or process state are still unchanged. `rust-analyzer` is allowed to keep running; its PID may have changed. If the IDE has already removed the cache, document that fact and still validate the anchored ignore rule without recreating material solely for this test.
- Check `git status --porcelain` and `git status --porcelain --untracked-files=all` before the change. The only allowed unexpected *working-state* entry at preflight is the **verified** root `target/` cache, plus the single Owner-installed ACTIVE edit. If a different entry is present, its provenance/ownership is uncertain, the cache is not a known disposable build directory, or a path is linked out of Solo, **STOP**; never introduce a broad ignore rule to conceal it.
- `.temp/` remains ignored, untracked and unstaged. The previously retained `.temp/toolchain-installer/` archives (last reported **289,253,806 bytes**) are a possible future Windows-GNU/offline setup resource, **not used by the currently passing MSVC-hosted Windows CI**. The earlier `.temp` cleanup reclaimed **669,778,642 bytes**; do not redo it or present those earlier numbers as newly reclaimed. Preserve all remaining `.temp/` material; clean only new, identified task-owned disposable scratch under the standing SOP.

## 3. Exact permanent path and content ledger

**Only the following three permanent tracked files may change, and only by these operations.** The initial and final ACTIVE transitions count as the same authorized path; never create a duplicate order, dated report, WIP file or archive.

| Path | Authorized operation |
| --- | --- |
| `.documents/WIP/workorder-active.md` | Initially publish the Owner-installed `SOLO-WO-058 R01` as the **sole** file in an issued-order commit; after every hygiene acceptance gate and CI at the substantive commit succeeds, reset to the exact neutral one-line heading below and publish its own closure commit. |
| `.gitignore` | Add **exactly** the two lines specified below, after the existing `coverage/` entry and before the existing `# Common JavaScript/build outputs` comment; leave all previous lines, order and content untouched. |
| `.documents/Reports/execution-report.md` | Replace the single public-safe canonical report with factual results for `SOLO-WO-058 R01`; **only** this path in the last report commit. No independent report archive or helper file. |

**Required `.gitignore` insertion** (one blank line already separates `coverage/` from the next existing section; insert these two lines before that blank line):

```gitignore
# Rust/Cargo and rust-analyzer generated build cache at the repository root.
/target/
```

The resulting relevant excerpt is exactly:

```gitignore
# General logs and temporary files
# .temp/ is the ONLY permitted agent-created scratch root inside Solo; clean promptly.
*.log
*.tmp
*.bak
.temp/
.cache/
coverage/
# Rust/Cargo and rust-analyzer generated build cache at the repository root.
/target/

# Common JavaScript/build outputs (adapt after toolchain is chosen)
```

The anchored pattern must not become generic `target/` or `**/target/`, may not exclude any tracked source or other nested `target` directory, and may not hide unknown outputs by adding broader patterns. Do **not** modify root `.gitattributes`, root or local Git config, `.git/info/exclude`, workspace manifests, `CARGO_TARGET_DIR`, `RUSTFLAGS`, toolchain, executable scripts or CI workflow to achieve the same result. `.gitignore` is a repository-standard, nonarchitectural file; no About/Architecture/SOP contract edit is required or authorized.

**Final neutral ACTIVE text, exact:**

```markdown
# Foundation Solo — Active Work Order
```

Leave ROADMAP, NEXT and OUTLOOK **byte-identical** to their above baseline blobs. Leave `deny.toml`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `.github/`, all crates/tests/fixtures, `README.md`, `AGENTS.md`, `.documents/About/`, `.documents/Architecture/`, both SOPs, `LICENSE`, the root `target/` files, and the retained `.temp` contents untouched.

## 4. Mandatory behavioral proof, local safety and regression checks

Before staging the `.gitignore` change, check the full proposed diff; `git diff --check` must be clean, and the added content must be exactly the two specified lines. No other file may be staged with it.

The hygiene change is a **real clean-worktree fix**, not a cosmetic declaration. Verify with recorded commands/results after the `.gitignore` file has been committed and again at the final report HEAD:

1. `git check-ignore -v --no-index -- target/` identifies the new, **root-anchored** `.gitignore` `/target/` rule whether or not the directory currently exists; if it exists, verify it remains physically present, unchanged by deliberate deletion/move and still contains its live build cache. An IDE may write new bytes during this task; record real measurements rather than claiming byte-for-byte identity.
2. `git ls-files -- target/` shows **zero tracked** paths. Confirm zero staged paths under the directory, and inspect any unexpected nested `.git` or owner-owned material identified in preflight; do not conceal it through ignore behavior.
3. `git status --porcelain` and `git status --porcelain --untracked-files=all` return **exactly empty output** once the authorized tracked changes are committed. A `git status` command that is still showing other dirty/untracked entries is **not** a pass. Do not use `git clean`, deletion, resetting, hiding additional paths, global excludes or force-add to create false cleanliness.
4. Check the pattern's scope: only the root `target/` is excluded by the new rule. Confirm the existing `.temp/` ignore remains unchanged and check that expected tracked authorities (e.g. `README.md`, `.documents/WIP/plan-next.md`) remain tracked. An incidental external/global ignore outside this order must be disclosed, not altered.
5. Verify `git diff --check`, final changed-path ledger, one worktree and remote, exact `origin/main` SHA/tree, unmodified ROADMAP/NEXT/OUTLOOK, neutral ACTIVE and no new branch/ref. Compare actual root cache and retained `.temp/` state to the earlier report without attributing changing IDE cache sizes to task deletion.
6. Report `Unicode-3.0` **`warning[license-not-encountered]`** in `cargo-deny 0.20.2` as a **pre-existing, inherited, non-vulnerability licence-policy warning**, one per CI host where observed. The checked graph/allowance mismatch remains `UNVERIFIED` until a separately Owner-issued dependency-policy investigation. Do not treat a green gate as warning-free, and do not edit `deny.toml` to silence it. Treat any **new or material** warnings, missing mandatory checks or changed security outcome as a blocker.

**No code-bearing tests are added or changed.** The unchanged GitHub `rust-verify` workflow is the mandatory full regression: Ubuntu 24.04 and Windows 2025 must each pass the required checks, including format, strict lint, locked workspace tests, architecture and refusal checks, fresh RustSec/cargo-deny gate, tripwire, release build, CLI smoke and the applicable reproducibility checks, with genuinely host-specific skipped steps classified as such. A new full external A1 version-currency campaign is **NOT APPLICABLE** for this ignore-only hygiene order, but actual CI advisories and warnings must be reviewed and recorded without suppression.

## 5. Exact Git, CI and reporting sequence

1. **Issue the Owner order.** After §2 preflight, on the one existing checkout at `main`, commit and ordinarily push **only** the Owner-installed `.documents/WIP/workorder-active.md`. Record its SHA and tree; verify the remote ACTIVE blob matches this issued order. Await/verify a successful real unchanged two-host `rust-verify` run at this issued-order SHA before modifying `.gitignore`. No new branch, PR, clone or worktree.
2. **Implement only the ignore rule.** Insert exactly §3's two lines in `.gitignore`, verify `git diff --check`, `git diff`, only the intended path staged, and that the live cache was neither deleted nor moved. Commit and ordinarily push **only** `.gitignore` on existing `main`; verify full substantive SHA, tree and remote `.gitignore` blob. At this point ACTIVE remains the issued order and NEXT stays R03.B. Independently inspect both-host `rust-verify` at the **exact substantive main SHA**. If any required gate fails, STOP and preserve ACTIVE; never neutralize it just to report completion.
3. **Prove reconciliation and neutralize ACTIVE.** Once the substantive main CI and local behavioral checks are green, replace **only** ACTIVE with its single neutral title line. Commit and push **only** ACTIVE on `main`; verify the resulting neutral blob exactly `ea5f20b736483c2e102456be1cea9104059fc81a`. Run/verify successful two-host CI at this neutralization SHA. Record full local `git status --porcelain` (it must be empty after the commit), and verify ROADMAP/NEXT/OUTLOOK unchanged. This is the sole authorized ACTIVE-closure reconciliation; there is no further R03.A or R03.B scope.
4. **Publish the canonical report last.** Overwrite **only** `.documents/Reports/execution-report.md` with factual evidence for `SOLO-WO-058 R01` and its reconciliation of the `SOLO-WO-057` clean-tree gate. Commit and ordinarily push **only** that report on `main`, then verify a **fresh** successful real two-host CI at final report HEAD. Read back the exact public report blob from GitHub; never embed its own SHA in its own text. Final `git status --porcelain` and the full-untracked view must be empty after the report commit. Return exact issued/hygiene/neutralization/report commit SHAs and trees, CI run IDs/jobs, changed-path ledger and local cache/ignore evidence in agent chat.

**Required report contents:** ISO session date; authorized ID and local root; starting and each intermediate commit/tree; original `SOLO-WO-057` outstanding gate and the mechanism by which `/target/` resolves it without deleting the active cache; exact `.gitignore` diff and final ignore/track/status proofs; physical-cache presence; retained `.temp` material and any task-owned scratch cleanup; original `SOLO-WO-057` cleanup totals attributed accurately to that prior run; WIP unchanged/neutral evidence; prior branch already pruned (do not prune again); both-host CI/skip/warning inventory including the inherited unmatched `Unicode-3.0` allowance; applicability of A1; explicit outstanding GNU archive retention/cleanup trigger; unimplemented R03.B/R03.C; exact scope and absence of unauthorized actions. Report a check as `PASS`, `FAIL`, `NOT RUN`, or `UNVERIFIED` truthfully. Use report status `COMPLETE` **only if the new mandatory final clean-tree and CI gates actually pass**; otherwise report `STOPPED` or `FAILED` and the factual blocker.

**Non-destructive Git mechanics:** Follow Architect SOP §6B as the one authority. Only ordinary commits and pushes to the existing `main` are authorized; no branch creation, PR, merge commit, deletion of refs, tag, history rewrite, force-push, `git reset`, `git clean`, destructive checkout, stash, secondary checkout, new remote or release. Before each commit verify that only the intended file is staged and only authorized files are changed. If `origin/main` diverges or any unexpected Owner content appears, STOP; do not ask the Owner to run routine Git operations the agent is authorized to perform.

## 6. Explicit permissions, exclusions and STOP

**Permitted:** Read the named Solo repository and its official CI evidence; use the current local checkout and the existing installed verification tools; install the Owner-issued ACTIVE as the authorization commit; add exactly the anchored ignore entry to `.gitignore`; neutralize ACTIVE after verified success; publish the canonical report; run or dispatch the unchanged CI; use already-permitted Solo `.temp/` and current Cline-provided temporary workspace for disposable task scratch and remove only identified newly obsolete task-owned scratch. Direct GitHub write operations are delegated **only** to the Implementation Agent inside this order; the Architect's GitHub access remains **read-only**.

**Not permitted:** Delete, alter, move, kill or interrupt the live root `target/` cache or its owning process; remove/alter the retained `.temp/toolchain-installer/` contents or other existing `.temp` files; edit `.git/info/exclude` or global Git ignores; ignore broad root/recursive patterns or arbitrary generated/Owner files; change ROADMAP, NEXT, OUTLOOK, About, Architecture or SOPs; implement R03.B/R03.C; modify product source, tests, fixtures, scripts, dependency manifests, lockfile, toolchain pins, `deny.toml`, `.cargo/config.toml`, CI or workflow; execute a dependency upgrade or new external service; change another repository; create another checkout or worktree; merge, publish, release, deploy, access secrets, perform DNS/TLS/provider action or purchase. No destructive data cleanup is authorized by this order beyond ordinary identified new task-owned scratch cleanup under the existing SOP.

**STOP and report safely** if the Owner-installed ID or starting ref is mismatched, an unexpected dirty/Owner-owned item exists, an existing root `target/` is not positively identifiable as a generated build cache, any nested repository/reparse escape is found, the ignore rule inadvertently hides unrelated Owner material, a different permanent path must change, a CI gate fails, a consequential warning or security advisory is introduced, the remote cannot be updated by ordinary Git mechanics, the clean-tree gate is still unmet after the exact change, or completion requires product/policy changes beyond this order. Preserve all uncertain material, do not self-amend the order and do not self-ACCEPT. Publish a factual `STOPPED` canonical report only when the standing report-only safety checks permit; otherwise report the exact blocked state in chat. The Chief Architect independently issues `ACCEPT`, `REVISE` or `STOP` after a fresh GitHub-first review.

**End of OWNER-ISSUED SOLO-WO-058 R01.**
