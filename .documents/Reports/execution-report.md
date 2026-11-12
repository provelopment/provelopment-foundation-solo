# Execution Report

**Order:** `OWNER-ISSUED — SOLO-WO-077 R01` — reconcile the exact native-readback CI edge-count pin and complete independent two-host proof of the preserved Linux `getrlimit` seam (workflow-only correction on the existing technical branch).
**Matching handoff:** `SOLO-WO-077 R01`. **Report date (ISO):** 2026-11-12.

**Outcome: `LINUX-NATIVE-READBACK-PROVEN`.** The preserved native source and the exact locked graph are byte-identical to `SOLO-WO-076 R01`; the only functional change anywhere in this order is a two-literal correction inside `.github/workflows/rust-verify.yml` on the technical branch. With that correction, the **complete, unchanged** `rust-verify` workflow ran green on **both** `ubuntu-24.04` and `windows-2025` at the exact branch commit, and the job logs directly re-prove the three real Ubuntu kernel readback tests and the absence of any Linux readback on Windows.

**This is a narrow statement.** It says the calling process's *own existing* `RLIMIT_AS` and `RLIMIT_CPU` values are read back through `getrlimit`. It is **not** `LINUX-HOST-CONTAINMENT-PROVEN`, not `HOST-CONTAINMENT-PROVEN`, not `MEDIA-VERIFIED`, and no sandbox, escape, child-enforcement or Windows Job Object claim is made. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**. Nothing was merged into `main`, and `main` deliberately still carries the `edges=40` pin because it has no Linux `libc` edge.

## 1. Preflight — `S077-01` PASS

Every anchor was read from the real repository before any edit.

| Evidence | Required | Observed |
| --- | --- | --- |
| `main == origin/main == HEAD` | `37832b7a6d79a0c5cbb6530d45ea83d8212f4641` | exact |
| `main` tree | `8967116c3201d6d94a64a3ff69bd6b78c30b28a5` | exact |
| Canonical report blob (WO-076, `BLOCKED`) | `cb56f54b2a1a647831496123764f6f48ffd8417b` | exact |
| Prior ACTIVE blob at HEAD | `9ba2a7937396a3032a393a31500909c2e8158586` | exact |
| Architecture Charter | `c941c7f90cc1b1955f9d60b348a3b1092cafe97f` | exact |
| NEXT / ROADMAP / OUTLOOK | `92fa562b…` / `9d7a8622…` / `cb9c7a61…` | exact |
| Technical branch local and remote | `solo-wo-076-r01` = `8e1d5645835e50850d575de1c22d463c1efe7980`, tree `363f0f99…` | exact |
| Branch parent / base | `f1bd7ccd72d144e4e9c403f1813c5e83605f7168` / `5b085d4a…` | exact |
| Workflow on `main` **and** branch | `eb78dad240bb7adb3e0ef9535870897176c99843` | exact |
| Branch native readback file | `2df6d0e98395f21fc72294606e9a97e954f97c63` | exact |
| Branch `Cargo.lock` | `b1ee56da60f36536548586528db371b7cbe508e4` | exact |
| Branch adapter manifest / crate root | `47d732f26b18addc67a2fc6d0a4a94939b8ca134` / `bd1d4c58f5a91f38af63e84e143c850d6372ca64` | exact |
| Branch graph / unsafe guard / graph tests | `7a5beada…` / `b88e7b42…` / `25a91b2c…` | exact |
| Historical branches | `solo-wo-061-r01` `4371af21…`, `solo-wo-069-r01` `e6162cdf…`, `solo-wo-070-r01` `d270f95c…` | unmoved |
| Previous main CI at report HEAD | `43201596763` (`131126131562`, `131126130631`) both success | confirmed |
| Previous branch blocker | `43200373681` (`131121910857`, `131121911134`) both fail only at the edge-count pin | confirmed |

One worktree, one remote `origin=https://github.com/provelopment/provelopment-foundation-solo.git`, and the **only** initial dirty path was the Owner-installed ACTIVE replacement (working blob `39fafdd139b1a37d5fde7fd8d58e1202134fc531`). Four WIP files; both product paths absent from `main`; `.temp/` had no top-level files and the four inherited WO-073 research files were untouched at **4 files / 46,062 bytes**.

## 2. Issuance — `S077-02` PASS

| Item | Value |
| --- | --- |
| Issuance commit | `3d6268441b6105cb324552da8ce83014b8f3e693` |
| Tree | `94d9102561a981205f57031fedb79a46a60fec28` |
| Parent | `37832b7a6d79a0c5cbb6530d45ea83d8212f4641` |
| ACTIVE blob | `39fafdd139b1a37d5fde7fd8d58e1202134fc531` |
| Changed paths | `.documents/WIP/workorder-active.md` only (byte-for-byte as installed) |
| GitHub readback | sha/tree/parent/blob all match; `files=.documents/WIP/workorder-active.md` |
| CI | `rust-verify` run **43203540346**, event `push`, at exactly this SHA — `131132815962` (`ubuntu-24.04`) success, `131132819167` (`windows-2025`) success, **33 configured steps each, 0 failures** |

## 3. The technical branch — two commits, both authorized

The preserved branch `solo-wo-076-r01` was advanced by exactly two commits. No new branch, worktree, clone, tag, PR, merge, rebase, force-push or history rewrite occurred, and the branch remains local == `origin`.

| Order | Revision | Tree | Parent | Changed paths |
| --- | --- | --- | --- | --- |
| 4C cherry-pick | `b63284afda35d4b874758e2719117bfa88a8dbb4` | `e44fd26a7b311c9fc19310867a675993a6cd31cb` | `8e1d5645835e50850d575de1c22d463c1efe7980` | ACTIVE only |
| 4D workflow | `40a40bab1d017e61340fe53399b49fb2ab335c3b` | `84de1cbfc192336529a6b073641c8ef6a9e74247` | `b63284afda35d4b874758e2719117bfa88a8dbb4` | `.github/workflows/rust-verify.yml` only |

**`S077-03` — the single authorized cherry-pick.** `git cherry-pick -x 3d626844…` applied with no conflict onto the verified branch tip `8e1d5645…`. The result `b63284a` has exactly **one parent**, the original branch tip; its **only** changed path is `.documents/WIP/workorder-active.md`; and its ACTIVE blob `39fafdd139b1a37d5fde7fd8d58e1202134fc531` is **identical to the newly issued main ACTIVE**. Nothing else moved, and the report, WO-076 code and every other main commit were left alone.

**`S077-04` — the workflow-only correction.** Two and only two literal spans changed, from `40` to `41`:

* line 660, Linux real-tree step: `grep -q 'edges=40'` → `grep -q 'edges=41'`
* line 672, Windows real-tree step: `if ($text -notmatch 'edges=40')` → `if ($text -notmatch 'edges=41')`

Direct proof that nothing else changed: `git diff --numstat` is `2 2 .github/workflows/rust-verify.yml`; `git diff --check` is clean; the file now contains exactly **two** `'edges=41'` literals and **zero** `'edges=40'`; and replacing both `'edges=41'` literals back to `'edges=40'` reconstructs the untouched original blob **exactly** — the inverse text hashes to `eb78dad240bb7adb3e0ef9535870897176c99843`, the required original. The count remains an exact fixed literal — no wildcard, no `edges=4` substring, no greater-than test, no derived leniency and no suppressed failure. `architecture=legal packages=4`, the checker process exit gate, every `|| { … exit 1; }` and `throw` path, the 29-case self-test assertions, the security gates, pinned versions and actions, jobs, triggers, permissions and all downstream steps are untouched.

**`S077-05` — nothing else moved.** At the final branch head `40a40bab…` every WO-076 blob is still its exact prior value: native readback `2df6d0e98395f21fc72294606e9a97e954f97c63`, `Cargo.lock` `b1ee56da60f36536548586528db371b7cbe508e4`, adapter manifest `47d732f26b18addc67a2fc6d0a4a94939b8ca134`, crate root `bd1d4c58f5a91f38af63e84e143c850d6372ca64`, dependency graph checker `7a5beadaa0fc6cae4b5f83bf6a3ff3f97484d45b`, unsafe guard `b88e7b42ab03bd78557e58d4c54b957b59369d6c`, workspace graph test `25a91b2c212805633fcf0fd80e4aaea11c595ba3`. The new workflow blob is `e1b5ca6e1738a5d5129a8788e24d51aee02421d1`. Still exactly one approved direct Linux external root (`libc 0.2.191`), still four workspace packages, and the native source still holds only the **two** justified, positively Linux-gated `getrlimit` `unsafe` blocks, with no child launch, no limit mutation and no new source file.

## 4. Branch CI — the full workflow on both hosts

The existing authenticated manual dispatch was used against the completed branch head, so the workflow itself was read from that branch.

| Fact | Value |
| --- | --- |
| Run | **`43204471496`** — conclusion **success** |
| Head SHA | `40a40bab1d017e61340fe53399b49fb2ab335c3b` — exactly the workflow-correction commit |
| Event | `workflow_dispatch` |
| `ubuntu-24.04` job | `131136025121` — success, **33 configured steps: 24 ok, 0 failed, 9 skipped** (all nine are the Windows-only counterparts) |
| `windows-2025` job | `131136025260` — success, **33 configured steps: 19 ok, 0 failed, 14 skipped** (all fourteen are the Linux-only counterparts) |

No applicable step was skipped, and no step was skipped because an earlier one failed. Verified from the logs, not from green status:

**Real-tree architecture evidence — the corrected count, on both hosts:**

```
architecture=legal packages=4 targets=32 files=58 edges=41
```

The same line appears in the Ubuntu step, whose executed command is `grep -q 'edges=41' "$RUNNER_TEMP/arch-real.txt"`, and in the Windows step, whose executed condition is `if ($text -notmatch 'edges=41') {`. The checker's own process exited zero, the `architecture=legal packages=4` requirement is intact, and the refusal self-test still passed on both hosts with `self-check=passed cases=29`, including `ok dynamic-load refused as dynamic-loading (1 finding(s))` and `ok checker-bypass refused as layer-outward-reference (1 finding(s))`.

**The three real Ubuntu kernel readback tests — verbatim:**

```
test media_worker_host_native::linux::tests::n076_01_getrlimit_rlimit_as_and_rlimit_cpu_observe_the_real_kernel ... ok
test media_worker_host_native::linux::tests::n076_02_ceilings_keep_their_full_range_and_repeat_consistently ... ok
test media_worker_host_native::linux::tests::n076_03_a_rejected_status_is_a_typed_refusal_not_a_partial_observation ... ok
```

**`S077-07` — Windows stays free of it.** The `windows-2025` log contains **zero** `n076_` occurrences: the Linux-native module is neither compiled nor linked there, no Linux readback is claimed, and every existing suite and guard test passed. The Windows toolchain was asserted as the pinned `rustc 1.99.1 (58cf8f9c3 2026-10-13)` and `cargo 1.99.1 (5f94df478 2026-08-27)`.

**Downstream gates that ran and passed on the applicable host.** `cargo fmt --all -- --check`; strict `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`; `cargo test --workspace --all-targets --locked`; the workspace package-set gate; the pinned `cargo-deny 0.20.2` component security gate on **both** hosts with a fresh RustSec refresh — `component security gate passed: cargo-deny 0.20.2 archive sha256 9f12ed4c…; RustSec 5fe96cf5…; advisories, bans, licenses and sources all ok on the unchanged locked graph`, with `lock_sha256=bf421c6e17034eb642cccfceeba22e01b5bea5ed138f03a3e5daf1c833086723` and the Ubuntu-only RUSTSEC-2026-0009 negative probe; the real-tree architecture checker and its 29-case self-test on both hosts; both test-failure tripwires; the release build; `release_smoke=verified accepted_exit=0 refused_exit=2`; the forbidden-import and bypass refusal with guaranteed byte-for-byte restore; and the Ubuntu-only two-clean-build byte-reproducibility comparison. `deny.toml`, the toolchain, every other workflow and every source file are untouched by this order.

## 5. Gates

| Gate | Status |
| --- | --- |
| `S077-01` preflight | **PASS** — §1, every anchor exact |
| `S077-02` issuance, readback, both-host CI | **PASS** — §2 |
| `S077-03` authorized issuance-only cherry-pick | **PASS** — §3 |
| `S077-04` two-literal workflow correction, inverse reconstructs the original blob | **PASS** — §3 |
| `S077-05` prior blobs preserved, one `libc` root, two justified gated `unsafe` blocks | **PASS** — §3 |
| `S077-06` corrected branch CI green on both hosts with real `edges=41` | **PASS** — §4, run `43204471496` |
| `S077-07` three Ubuntu `N076` tests executed; Windows free of the module | **PASS** — §4 |
| `S077-08` one report-only main commit, main CI green, branches and scratch intact | **PASS** — §6 |
| `S077-09` correct scoped outcome, red runs disclosed, Stage 03 open | **PASS** — `LINUX-NATIVE-READBACK-PROVEN`, narrowly |

## 6. State, limitations and cleanup

**`main` is code-free.** Its HEAD before this report was the issuance commit `3d6268441b6105cb324552da8ce83014b8f3e693` (tree `94d9102561a981205f57031fedb79a46a60fec28`), `main == origin/main`, working tree clean, `crates/solo-adapters/src/media_worker_host_native.rs` absent, and `.github/workflows/rust-verify.yml` still the original `eb78dad240bb7adb3e0ef9535870897176c99843` with `edges=40`. Two mainline commits in this order, one path each: the ACTIVE issuance and this report. **No branch code was merged, cherry-picked or otherwise promoted into `main`.**

**The proof branch is preserved and diverged deliberately.** `solo-wo-076-r01` sits at `40a40bab1d017e61340fe53399b49fb2ab335c3b` locally and on `origin`; it is not deleted and not merged. It carries the WO-076 code plus the matching `edges=41` CI pin, while `main` carries neither. The two workflow pin values will legitimately differ until a later, separately approved integration order unifies the code with its count. Historical branches `solo-wo-061-r01`, `solo-wo-069-r01` and `solo-wo-070-r01` are unmoved, and the WIP still contains exactly four files.

**Honest limitations.** No failure occurred in this order, so there is no new red run to disclose beyond the previously recorded WO-076 history: run `43199345760` (two defects in the new code, corrected) and run `43200373681` (the stale pin, corrected here). This workstation still has no MSVC linker and no Linux target, so **every** build, lint, test and kernel-readback statement above comes from the two-host CI logs; locally only `git diff --check` was meaningful. The readback observes the calling process's own ceilings and proves **no** enforcement — not a child's limits, not a watchdog, not a process group or descendants, not a Windows Job Object, not sandboxing, not decoding, not media correctness and not publication. `MEDIA_WORKER_GRANTS_PUBLICATION_AUTHORITY_V1` and `MEDIA_WORKER_PROVES_MEDIA_CORRECTNESS_V1` remain `false`.

**Cleanup.** The only scratch created in this run was `.temp/s1.md`, `s2.md`, `s3.md` (report fragments) and a temporary `.temp/inverse-rust-verify.yml` used for the blob-inverse proof; all were deleted before publication, and `.temp/` has no top-level files left. The four inherited WO-073 research files are untouched at **4 files / 46,062 bytes** and remain unverified reference only — not copied and not deleted. No Owner archive was present to preserve or disturb, and no unknown or Owner-owned material was removed.

The Implementation Agent does not self-ACCEPT, has merged nothing, released nothing and begun no other order; the branch and its evidence are returned for independent Chief Architect disposition: **ACCEPT / REVISE / STOP**.
