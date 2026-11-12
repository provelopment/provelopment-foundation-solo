# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-074 R01**  
**Title:** R03.C3 — bind the second, exact-path process-native exception and prove fail-closed architecture/unsafe checkers (documentation and test-guard slice only)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Exact matching Owner → Implementation Agent handoff:** `SOLO-WO-074 R01`  
**Authority:** This is the complete Owner-issued operational instruction for the single `.documents/WIP/workorder-active.md` execution slot after Owner installation and matching handoff. Perform only the authorized operations. The Chief Master Architect has **permanent GitHub read-only access**. The Implementation Agent alone conducts all expressly authorized local and remote Git operations. This order closes a bounded **charter and checker** task, not the aborted `SOLO-WO-073 R01` native implementation.

## 1. Approved Scope — what must be delivered

Record one narrowly defined, second platform-native source-file exception in the canonical Architecture Charter §6A.8 and enforce the exact new rule in the **existing** source AST checker and independent first-party `unsafe` boundary guard, with positive/negative self-tests on both `ubuntu-24.04` and `windows-2025`. Keep the prior Windows-only exception for `crates/solo-adapters/src/source_read_native.rs` unchanged in purpose and scope. Reserve only the exact prospective file `crates/solo-adapters/src/media_worker_host_native.rs` for **later** separately Owner-issued Linux/Windows process-control implementation. The prospective file **must not be created in this order**.

The new path is eligible only for native **process launch, per-child resource limits, process-group/Job Object lifecycle, and native accounting** under positively proven `target_os = "linux"` or `windows` scope. It is not eligible for AVIF decode, media parsing, filesystem source reading, arbitrary dynamic loading, unrestricted syscalls, credential access, network publication or any other purpose. No blanket `unsafe` allowance, implicit test exemption, alias loophole, source-checker bypass, dynamic-loading waiver or dependency-graph wildcard may be introduced.

**This order authorizes the Architecture Charter decision and the test-guard implementation only.** It does **not** authorize adding the prospective native file, the safe host façade, a process supervisor, CLI internal probes, OS limits, a decoder, a new Cargo root, a new feature flag, an additional dependency, a lockfile rewrite, a branch, a public asset or any publication behavior. The next code-bearing Owner order must separately authorize and prove real native behavior. Preserve exactly one Entity, strict EFS ancestry, manifest-only publication authority, the four-format graphic declaration set, source ceiling 20 MiB, dimension ceiling 8192, pixel ceiling 32,000,000, whole-candidate refusal and existing architecture/unsafe tripwires. All media-worker results remain nonpublishing and do **not** prove media correctness. `R03.C3`, `R03.C` and Stage 03 remain **OPEN**.

**Result vocabulary:** `NATIVE-SCOPE-GUARDS-PROVEN` only after the complete Charter/checker/tests/CI sequence passes; `BLOCKED` for a confirmed required failure; `UNVERIFIED` when essential evidence was not obtained. No result from this order may be called `HOST-CONTAINMENT-PROVEN` or `MEDIA-VERIFIED`.

## 2. Exact preflight anchors and obligatory reading

Before any non-Owner file edit, verify the **real** sole checkout and these exact current-GitHub anchors. The only permitted initial worktree difference is the Owner-installed ACTIVE replacement bearing the matching ID.

| Anchor | Required exact value |
| --- | --- |
| `main == origin/main == HEAD` | `a6905c243e618c63039949b281710cec2ac0c8e4` |
| `HEAD^{tree}` | `a1df91166a6457c01970e426b2187323811b96e9` |
| Canonical report blob | `ee95501bf4ffb92e5dfc17d20ad2c859540e7cdd` (`SOLO-WO-073 R01`, `UNVERIFIED`) |
| Previous ACTIVE blob at HEAD | `ebd56fe0e5c0fe4c233f2b7f8163fa4fd7a3ec59` |
| Architecture blob at HEAD | `88b16fd91ffe4618801b4f956340674c27c7e047` |
| NEXT / ROADMAP / OUTLOOK blobs | `92fa562b1af0c54391851aa5c79deb1631127069` / `9d7a86226878f490fd5e26c6813a81e5f73f3f37` / `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Root `Cargo.lock` blob | `2f9e53576df117f2020ae064d6973304f20833f1` |
| AST checker blob | `76abff5773bbe9af05293e04b3089f807d767886` (`crates/solo-cli/tests/architecture_checks/source.rs`) |
| Unsafe boundary test blob | `9088c2eea742095df3299c3fbafa3a8ce782e18c` (`crates/solo-cli/tests/unsafe_boundary.rs`) |
| Architecture-fixture test blob | `7a66327693757f7c369a24933e2a1c021b61df19` (`crates/solo-cli/tests/check_architecture_fixtures.rs`) |
| Existing verified CI | `rust-verify` run `43178472596`, both hosts successful at starting HEAD |
| Preserved branches | `solo-wo-061-r01` = `4371af21dce67f33cf1df0384738bf353ca80fc6`; `solo-wo-069-r01` = `e6162cdf733ea3136e96cbc139e46a6daf4054ee`; `solo-wo-070-r01` = `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |

Read before edits: `README.md`, `AGENTS.md`, `.documents/About/FOUNDATION-SOLO.md`, `.documents/Architecture/ARCHITECTURE.md` §§5A, 6A.3 and 6A.8, both current SOPs (Architect §§5–8 and Implementation §§4–5C), all four WIP files, the exact canonical report, the current `crates/solo-cli/tests/architecture_checks/{source.rs,graph.rs,mod.rs}`, `crates/solo-cli/tests/{unsafe_boundary.rs,check_architecture_fixtures.rs,workspace_graph.rs}`, `crates/solo-adapters/src/{lib.rs,source_read_native.rs}`, Cargo manifests/lock/policy, toolchain and CI workflow. Read the original `SOLO-WO-073 R01` scope as historical authority **only**; this order supersedes its incomplete executable scope. Existing ignored `.temp/solo-wo-073-wip/` is uncompiled prior-run scratch, not source authority, permission or a verified implementation; preserve it unchanged for later native code work.

## 3. Permanent path ledger, permissions and exclusions

**Only these repository paths may be modified and committed:**

1. `.documents/WIP/workorder-active.md` — Owner installs the complete text; agent stages and commits it **unchanged**, by itself, at issuance.
2. `.documents/Architecture/ARCHITECTURE.md` — one exact new normative paragraph in §6A.8 as §5A below. Keep every other byte of the Charter unchanged.
3. `crates/solo-cli/tests/architecture_checks/source.rs` — exact-path and positive-OS-gate extension of the existing AST/source checker; retain the actual file-loading, dependency, alias, macro, re-export, FFI, dynamic-load and negative-test mechanisms.
4. `crates/solo-cli/tests/unsafe_boundary.rs` — exact two-path independent refusal scanner and focused negative/positive tests, with **no** invented second module in the real tree.
5. `crates/solo-cli/tests/check_architecture_fixtures.rs` — tests of the exact AST verdict and real scanner, including negative Linux/Windows qualification and forbidden native-root fixtures; no broad fixture-array refactor.
6. `.documents/Reports/execution-report.md` — exactly one replacement at final handoff, in a report-only commit.

Every other tracked file is read-only, especially `crates/solo-adapters/src/lib.rs`, the prospective `media_worker_host_native.rs`, `source_read_native.rs`, `Cargo.toml` at root and all crates, `Cargo.lock`, `crates/solo-cli/tests/architecture_checks/graph.rs`, `workspace_graph.rs`, `deny.toml`, `rust-toolchain.toml`, every workflow, the production CLI, the application/domain/media-worker modules, the other three WIP files, and all past proof branches. **Do not create a new crate/module or modify `mod.rs`**. The permitted AST change does not itself change the locked Cargo graph: preserve the existing four crates and the current closed approved roots. `libc =0.2.191` must **not** be added as a new root yet. That later declaration and matching graph changes belong to the subsequent Linux-native code order. In particular, do not remove `libc` from the existing global dynamic-loading deny list or approve `libloading`/`dlopen` anywhere.

**Git and remote authority:** One real checkout/worktree, exactly one existing `origin`, `main` only. Authorized: narrowly staged commits, non-force push to `origin/main`, read-only GitHub CI queries and SHA/blob readback. Unauthorized: new branch, PR, merge, tag, release, deployment, provider/secret access, remote/settings edit, another checkout/worktree, force, reset, rebase, amend, stash, cherry-pick or `git clean`. Do not alter sibling or parent repositories. Internet operations only those the existing pinned `rust-verify` workflow already performs; no new Cargo acquisition, dependency/tool installation, dynamic payload, scripts or networked services. Use Solo ignored `.temp/` and the actual Cline task temporary workspace for bounded scratch only. Preserve Owner-owned archives and prior `solo-wo-073-wip` files; remove only positively identified **this-order-owned** disposable material, log both temp-area inventories safely, and report any cleanup issue.

## 4. Required operation and CI sequence — no skipped stage

**4A. Safe preflight.** Run in the actual sole repository:

```powershell
$root = 'C:\Users\Sean\Documents\Provelopment\00.foundation-solo'
git -C $root rev-parse --show-toplevel
git -C $root rev-parse HEAD
git -C $root rev-parse 'HEAD^{tree}'
git -C $root status --porcelain=v1 --untracked-files=all
git -C $root worktree list
git -C $root remote -v
git -C $root branch -avv
```

Validate the exact anchor blobs via `git rev-parse HEAD:<path>`, the four-file WIP inventory, clean `main == origin/main`, precisely one Owner-installed ACTIVE change, exact content ID `OWNER-ISSUED — SOLO-WO-074 R01`, handoff match and intact inherited scratch. STOP without other edits on any mismatch; no history or data repair is authorized.

**4B. Issue ACTIVE.** Stage **only** `.documents/WIP/workorder-active.md`; commit as `Issue SOLO-WO-074 R01: native checker reservation`; push non-force; verify exact remote parent, tree, ACTIVE blob and one-path changed-file list. Require completed successful original `rust-verify` on **both** `ubuntu-24.04` and `windows-2025` at that issuance SHA before editing the Charter.

**4C. Bind Charter only.** Insert the exact paragraph from §5A as one contiguous addition in Architecture §6A.8, **immediately after numbered rule 6, before the `Single portable sealed artefact` paragraph**. Do not edit any other byte, preceding rule, newline convention or EOF. Verify insertion-only by removing the exact inserted byte span and reproducing the pre-change Architecture Git blob. Stage **only** `.documents/Architecture/ARCHITECTURE.md`; commit `architecture: reserve exact AVIF process-native boundary (SOLO-WO-074)`; push; inspect GitHub diff and enforce completed original `rust-verify` success on both hosts at this commit SHA.

**4D. Checker and hostile-test implementation.** Change only the three authorized `crates/solo-cli/tests/` paths in §3. Enforce every detailed test in §§5B–6 and the continuing baseline protections. No real native module, no manifest/lock/graph edit. Before commit run all possible local syntax/format/read-only diff checks. Stage just those three checker/test paths; commit `test: enforce exact two-file native scope with OS gates (SOLO-WO-074)`; push; require completed **both-host** original `rust-verify`, with new positive and negative tests actually executed. Fix any defect only within the same three allowed paths; disclose all intermediate red runs and fixes; never weaken a safety condition to obtain green.

**4E. Report and closeout.** On clean `main`, overwrite **only** `.documents/Reports/execution-report.md` with complete §7 evidence. Stage/commit it alone with `Report: SOLO-WO-074 R01 native scope guards`; push non-force; remotely verify final SHA, parent, tree, report blob and one-path commit. Require completed successful original `rust-verify` on both hosts at the **exact final report HEAD**. Preserve the three historical branches, existing WIP four-file shape, `target/` and the inherited `.temp/solo-wo-073-wip/` scratch. Return all commit/tree/blob IDs and all CI run/job IDs. Do not self-ACCEPT or start the next order.

## 5. Binding Charter and checker design

### 5A. Exact Architecture text to insert

Insert the following **one paragraph**, preserving the paragraph's exact wording, directly between §6A.8's existing numbered rule 6 and the following `Single portable sealed artefact` paragraph:

> **Native process-control source boundary — `SOLO-WO-074 R01`.** This Owner-issued architectural decision reserves exactly one additional first-party native source boundary, `crates/solo-adapters/src/media_worker_host_native.rs`, exclusively for future Linux/Windows process launch, per-child resource-limit installation, bounded lifecycle/termination and OS accounting. Any `unsafe` operation in that file must be inside a positively established `#[cfg(target_os = "linux")]` or `#[cfg(windows)]` scope and carry a local `// SAFETY:` rationale; unsafe/native access remains forbidden everywhere else except the previously approved, strictly Windows-only `crates/solo-adapters/src/source_read_native.rs` source-read exception. The reserved path is not a crate-wide allowance, an FFI or dynamic-loading waiver, a sandbox guarantee, or approval to run untrusted media. No executable native worker, extra crate/dependency, AVIF codec, media/publication claim, release or Stage 03 closure is authorized by this paragraph or this checker-only order. Adding the file, adjusting the audited Cargo graph and proving real kernel/Job Object enforcement require a separate Owner-issued code work order with two-host evidence.

A purely formatting insertion must not alter §6A.8's existing clauses, including the warning that resource limits are **not** filesystem/network/syscall sandboxing, the periodic Windows CPU overshoot, reference 512 MiB/10 CPU s/15 s settings, and pending conformance/provenance/notice obligations.

### 5B. The source AST checker — exact responsibility

Extend the current `APPROVED_NATIVE_MODULE` policy in `crates/solo-cli/tests/architecture_checks/source.rs` without weakening its existing real-tree scan or test fixture interface. Preserve the exact old path as an approved **Windows-only** case and introduce a distinct constant for the **future process path**. Do not change existing source-read behavior merely to make the new file fit. The check must examine the **true repository-relative path** (including correctly normalized Windows separators), distinguish *positively proven* `windows` and *positively proven* `target_os="linux"` item/ancestor gates, and refuse every unsafe occurrence without the appropriate positive OS fact:

- Old source-native path: only positive Windows-scoped `unsafe` is admitted; Linux-scoped and unscoped unsafe always fail as before.
- New process-native path: positive Windows-scoped **or** positive Linux-scoped `unsafe` is admitted; unscoped, ambiguously scoped, conditional-without-guaranteed-OS, negated and cross-target unsafe fail.
- All other paths, including all tests, safe façade, CLI, inner crates and copied/symlink-aliased modules: any `unsafe` fails.
- A literal positive platform predicate and an `all(...)` whose every valid branch necessarily includes that positive platform predicate can establish a gate. `cfg(any(windows, feature = "x"))`, `cfg(not(windows))`, `cfg(any(target_os = "linux", feature = "x"))`, wrong target values, malformed predicates and unknown `cfg_attr` forms **do not** establish a positive gate. Do not infer a gate because a token `windows` or `linux` appears in an attribute string. Preserve currently legal `#[cfg(windows)]` source-read definitions.
- Every analysis path remains fail-closed: `extern` blocks, native/dynamic loading, unknown macros, source escape, aliases and renamed dependency imports may not be masked by the new exact-file exception. `libc` is still a forbidden dynamic-loading root for ordinary source. The only reserved exception is direct, declared, pinned `libc` API access from the future process-native file within a positively Linux-gated scope; **no** `libloading`, `dlopen`, `dlsym`, `LoadLibrary`, wildcard native root, unapproved imported crate or arbitrary symbol-resolution API is permitted. As no future file or new `libc` dependency exists in this order, this is a **checker rule tested on an in-memory synthetic source**, not permission to add a Cargo edge. Actual Cargo graph approval and dependency installation remain deferred.
- Keep both structural and path-based checks. An injected `#[path]` escape, `include!` escape, alias/re-export bypass or out-of-root module cannot receive a native exception by spelling its intended filename.

Use a narrow, auditable rule/decision function evaluated by both the actual checker and dedicated negative tests. Ensure the current checker reports its old categories and that any new category is stable, bounded and carries no private input. Keep the original `APPROVED_NATIVE_MODULE` constant usable by existing tests where possible; do not replace the tool with a standalone loose text scanner.

### 5C. Independent `unsafe_boundary` guard

Update `crates/solo-cli/tests/unsafe_boundary.rs` to recognize exactly two **different** eligible paths, but assert that **only the pre-existing source-native module is present in the real tree today**. All real `.rs` files must still be examined, including tests, and all pre-existing refusal-injection fixtures must remain effective. The guard must:

- Prove the current `#![deny(unsafe_code)]` adapter lint and existing **single active** `#[cfg_attr(windows, allow(unsafe_code))]` on `source_read_native.rs` remain exactly as-is. Do **not** add a second `allow` attribute to `solo-adapters/src/lib.rs` until the future code order introduces the actual new module.
- Verify using in-memory snippets that a correctly target-gated prospective native module with a `// SAFETY:` note is eligible, and a copied unsafe source under another path, an ungated allow, an allowance in `lib.rs`, `cfg(any(...))`, `cfg(not(...))`, missing safety note, a Windows-only unsafe block under Linux, and a Linux-only block under Windows are all refused.
- Treat `windows_sys`/`rustix` as existing source-native roots per the currently accepted rule; the prospective process-native path may use only target-gated `windows_sys` and `libc` for the fixed process-control purpose. Refuse these native roots in any other first-party file. Do not overmatch harmless comments/strings/lifetimes as executable code, and do not hide a real import behind a rename.
- Preserve the existing guard's code-only scanner and test tripwire, with an AST-backed positive-gate decision for any ambiguity rather than string substring matching. An unprovable gate refuses the source.

## 6. Required test/proof matrix on both hosts

Create deterministic in-memory source snippets **inside the permitted test files**. They must run through the actual checker/guard, not a disconnected duplicate rule or comments asserting the expected behavior. At minimum, each of these case IDs has an explicit pass/fail expectation and actual host evidence:

| ID | Required executed proof |
| --- | --- |
| `G074-01` | Actual current repository source tree passes the unchanged architecture dependency and source checkers; the old Windows source-native file remains valid and unchanged. |
| `G074-02` | Existing Windows-only approved path accepts properly Windows-gated unsafe and refuses Linux-only, negated-Windows and unscoped unsafe. |
| `G074-03` | Future exact process-native path accepts positively Linux-gated and positively Windows-gated unsafe snippets with a local safety rationale, never outside the gate. |
| `G074-04` | `cfg(any(windows, feature = "x"))`, `cfg(any(target_os = "linux", feature = "x"))`, `cfg(not(...))`, wrong-platform value and malformed/ambiguous `cfg_attr` are **not** treated as an OS proof. |
| `G074-05` | Unscoped unsafe, native access from domain/application/CLI/tests/safe façade, third/copycat filename, crate-wide `allow` and wildcard allowance all refuse. The real adapter root keeps exactly one unchanged active module-level allowance. |
| `G074-06` | Direct Linux `libc` usage in the exact future file may be recognized only with positive Linux gate; renamed `libc`, `libloading`/`dlopen`/runtime symbol loading and `windows_sys` or `rustix` in forbidden files cannot evade the guard. No global dynamic-loading bypass. |
| `G074-07` | Unknown macro, `#[path]`/`include!` escape, foreign block, unresolved alias and source spoof/escape remain refused by the real checker; original negative fixtures still execute. |
| `G074-08` | All existing architecture-checker/unsafe-boundary tests, actual workspace graph/lock checks, normal CLI/source/media-worker tests, and format/lint/test tripwires pass. |
| `G074-09` | No native module or `unsafe` code is introduced; no manifest/lock/policy/toolchain/workflow/product module changes; no stress, OS containment or codec code runs. |
| `G074-10` | `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-targets --locked`, the existing architecture checker plus refusal self-test and pinned `cargo-deny 0.20.2` pass on both hosts at checker commit and final report HEAD. |

A passing test for an in-memory *future* source snippet is only a parser/guard proof, **not** proof that the new native file exists or compiles. Do not give `PASS` to any OS-kernel/Job Object/resource-exhaustion behavior: those are explicitly outside this work order. No dependency installation or cargo policy change is allowed; if the exact future-path exception cannot be made sound under the current checker without an unauthorized change, STOP and record the precise blocker rather than adding a workaround.

## 7. Final acceptance gates, report and stop conditions

| Gate | Mandatory evidence |
| --- | --- |
| `S074-01` | Verified one checkout/worktree/remote, exact baseline SHA/tree/all named blobs, clean main except Owner-installed ACTIVE, matching `OWNER-ISSUED SOLO-WO-074 R01` handoff, unchanged branch and scratch inventory. |
| `S074-02` | ACTIVE-only issuance commit; exact remote readback; successful original `rust-verify` on both hosts at issuance SHA. |
| `S074-03` | One exact paragraph inserted in §6A.8; removed insertion reconstructs original Architecture blob `88b16fd9…`; all other Charter bytes unchanged; both-host CI at Architecture commit. |
| `S074-04` | Actual AST checker enforces the two **different** path/OS scope rules, positive predicate semantics, no source/alias/macro/dynamic escape, all actual tests and negatives execute. |
| `S074-05` | Independent boundary guard preserves actual sole active native allowance, recognizes only the exact reserved future path in snippets, refuses ungated/native/mis-scoped/third-path code and preserves every old tripwire. |
| `S074-06` | No change to dependency graph, source readers, product crates, CLI, manifest/lock/policy/workflow/toolchain, no new native file/unsafe, no AVIF/media/release/publish claim. |
| `S074-07` | Checker/test-only commit changes exactly the three permitted checker/test paths; full existing `rust-verify` successful on both hosts at that SHA and all `G074` cases observed. |
| `S074-08` | Report-only final commit, exact remote parent/tree/blob readback, final both-host original CI green at report HEAD, clean single worktree and preserved historical branches/temp artifacts. |
| `S074-09` | One accurate outcome (`NATIVE-SCOPE-GUARDS-PROVEN` / `BLOCKED` / `UNVERIFIED`), all Stage 03 media gates OPEN, no self-acceptance or unauthorized next work. |

Overwrite exactly `.documents/Reports/execution-report.md` with the order ID and handoff, starting/issuance/Architecture/checker/report-parent SHAs/trees, changed-path ledger, Charter insertion byte proof, actual checker source and test cases, G074 and S074 verdict tables, terminal/toolchain limitations, all CI run/job/step IDs and red iterations, unchanged graph and RustSec/security checks, actual/no-op `unsafe` and exception inventories, unchanged `target/`, preserved inherited `.temp/solo-wo-073-wip/` (four files/46,062 bytes as previously recorded, with current independent inventory), task-owned scratch cleanup, and exact report-only remote readback. Report the final commit SHA/tree/blob to the Owner in chat (do not embed the uncreated final commit SHA inside the report). The next anticipated code orders are **separately authorized Linux and Windows native containment**, not part of this order.

**STOP immediately** on dead terminal, wrong root/ref/commit/tree/blob, unexplained dirty/untracked material, branch drift, wrong handoff, Charter mismatch, impermissible path or `unsafe` occurrence, any broadening of native or dynamic-loading privileges, unapproved `libc` Cargo edge, weakened/disabled test, unproven positive cfg gate, unsafe data cleanup, new package/version, unverified CI, unexpected runner failure, secret leakage or non-fast-forward push. Publish a factual blocked/unverified report safely if possible; otherwise leave the tree and Owner data intact and return the blocker in chat. Do not self-ACCEPT, merge, deploy, release or start another work order.
