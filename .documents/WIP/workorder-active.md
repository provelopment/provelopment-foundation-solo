# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-068 R01**  
**Title:** R03.C3 recovery — bounded decoder-selection, dependency-graph and licence/security feasibility qualification (nonimplementing)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching explicit Owner→Implementation Agent handoff ID:** `SOLO-WO-068 R01`  
**Owner authority:** Following the Chief Architect's independent **`STOP — SOLO-WO-067 R01`**, the Owner expressly directed **“proceed you are authorized.”** This is a new, separate, complete and *nonimplementing* recovery order. It authorizes safe repository issuance, bounded temporary research/decoder probes, one canonical report, and the corresponding existing two-host regression CI. It does **not** approve a codec, new runtime dependency, a licence obligation or exception, a policy change, an implementation, a release or a deployment. Chief Architect GitHub access remains **READ ONLY**. Only the Implementation Agent may perform the operations expressly issued here and only from the verified Solo root.

## 1. Verified starting state, binding sources and intended result

**Exact expected baseline `main == origin/main`:** `017272d61aa9a00057a4a486ce5e51e4a85dae0b`  
**Baseline tree:** `dfb67e808d4b4e8a067571814188dea61d80d791`  
**Canonical report:** `.documents/Reports/execution-report.md` blob `ff67067f4576e05040b4479dab30d06e77c8aab6` — reported `SOLO-WO-067 R01 — STOPPED` after a failed decoder qualification, *not* a completed C3 implementation.  
**Four WIP slots:** `.documents/WIP/workorder-active.md` blob `6acea804ec3124979d477e303c2169e43eb787a9` (frozen R01 STOP-era order); `.documents/WIP/plan-next.md` blob `92fa562b1af0c54391851aa5c79deb1631127069` (neutral); `.documents/WIP/plan-roadmap.md` blob `9d7a86226878f490fd5e26c6813a81e5f73f3f37`; `.documents/WIP/plan-outlook.md` blob `cb9c7a610cf3c537557e060ca9656e44a053e69d`.  
**Branches:** `main` at baseline and **divergent** `solo-wo-061-r01` at `4371af21dce67f33cf1df0384738bf353ca80fc6`, which **must not be deleted, moved, rebased, merged or otherwise altered**. The previous `solo-wo-065-*` branches were safely pruned in the accepted administrative closure. `solo-wo-067-r01` was never created; do not invent it.

**Read before acting:** `AGENTS.md`; `.documents/SOP/sop-architect.md` §§6A, 6B, 7A, 7B, 8; `.documents/SOP/sop-implementation.md` §§4, 5A, 5B, 5C; `.documents/Architecture/ARCHITECTURE.md` §§5A, 6.2C, 6.2G, 6.2I, 6.2K, 6A; the four WIP files; the R01 STOP report; `deny.toml`; `Cargo.toml`, `Cargo.lock`, adapter manifest; `crates/solo-cli/tests/architecture_checks/graph.rs` and `workspace_graph.rs`; the existing `rust-verify.yml` workflow and accepted R03.C1/C2 media-evidence types.

**Goal:** Measure whether there is a specific, legally and technically supportable, *four-format* decoder set for R03.C3 under current or explicitly identified future approvals. Produce an evidence-backed ranked component/feature/graph/decoder-fitness recommendation, **or** a conclusive, accurately qualified blocker matrix. Successfully executing this investigation is **not** media-validation acceptance and does not clear a future candidate for `main`. The next code-bearing order may only be prepared after independent Chief Architect `ACCEPT` of this bounded investigation and fresh Owner authorization.

**Unchanged physical contract:** PNG, JPEG, WebP and AVIF must all be covered in the eventual implementation; source input ≤20 MiB; width/height each 1–8192; decoded pixels ≤32,000,000; actual complete decoding, format/MIME agreement, bounded allocations/CPU, and whole-inventory fail-closed behavior including internal/suppressed/unlisted owners. `graphic.favicon` is PNG-only. No extension-only or magic-only success, no silent Platform/role fallback, no public derivatives and no publication. `R03.C3`, `R03.C` and Stage 03 remain **OPEN** regardless of the study's result.

## 2. Exhaustive permanent-path and operation ledger

**Exactly TWO tracked paths are writable:**

| # | Path | Allowed operation |
| --- | --- | --- |
| 01 | `.documents/WIP/workorder-active.md` | **Owner installs this complete R01 file**; the Agent stages and commits it **alone** on `main` as the issuance commit, without further edits. It remains frozen during the run. |
| 02 | `.documents/Reports/execution-report.md` | **Overwrite once** with factual, bounded qualification findings; stage and commit **this path alone** after research, push and remote-readback, and require full green CI at its exact final HEAD. No parallel report. |

**Read-only permanent paths:** NEXT, ROADMAP, OUTLOOK, Architecture, About, AGENTS, both SOPs, every source/test/fixture, all Cargo manifests, `Cargo.lock`, `deny.toml`, `.github/workflows/rust-verify.yml`, toolchain, `.cargo/config.toml`, `.gitignore`, all others. **NEXT is already neutral and must remain byte-identical.** WIP must remain exactly the four existing files, without folders, placeholders, suborders or an archive. No `NOTICE`, test fixture, recommendation document, ADR or implementation file may be added in the repository. The canonical report is the *only* final deliverable in Git besides the issued ACTIVE.

**Scratch exception, bounded to this task:** Within the **existing** verified Solo `.temp/` (ignored, never staged) and/or the actual task-provided Cline temporary workspace, the Agent may create a small number of *disposable, isolated Cargo probe manifests and minimal verification programs*, local copied-down published crate packages, generated Cargo locks/tree graphs, command outputs and **synthetic test-only media bytes** to evaluate the names in §3. A probe manifest under Solo `.temp/` must explicitly contain `[workspace]` where necessary to remain independent of the parent workspace; it is **not** a second checkout, linked worktree, nested repository or shadow copy of Solo. Do not clone or copy the Solo repository. Probe commands may access crates.io and official primary security sources for read-only package resolution; do not execute unknown third-party install scripts, use a Git dependency, run privileged tools, download standalone compilers, change local trust policy, or obtain network access from inside a product runtime. Only a manifest with a narrowly known package build may be compiled; a suspicious build script is STOP for that probe, not permission to execute it. Preserve the existing ignored Rust `target/` cache and pre-existing `.temp/` archives; inventory and clean **only** positively identified task-owned disposable material, with exact cleanup outcome in the report.

**Prohibited permanently:** all Rust/application/adapter code and tests, any dependency lock/pin, `deny.toml`, workflow edits, graph allowlist edits, source/native-reader changes, any Platform library or artwork, changing the accepted Architecture contract, branch creation, PR, product implementation, integration, release, distribution and provider/deployment actions. Do not push probe manifests, experimental graph results or any unreviewed decoder bytes.

## 3. Candidate matrix and precisely bounded probe permission

This list permits **investigation only**. It is not a Cargo approval, a security clearance, a legal ruling or authority to add a root to Solo. Verify all identities/versions against crates.io source packages before touching the temporary probe graph. If a stated exact version is missing, yanked, incompatible or malicious, mark it and STOP that probe; do not silently substitute `latest`.

### A. First-choice separate-format trial (four exact roots)

| Image | Trial package | Exact version | Initial investigative hypothesis — **UNVERIFIED** |
| --- | --- | --- | --- |
| PNG | `zune-png` | `=0.5.2` | Pure-Rust PNG path with `zune-core`/`zune-inflate`, potentially avoids `png 0.18.1`'s two-`miniz_oxide` conflict. Its defaults, SIMD, complete-stream behavior, metadata and working-set limits require proof. |
| JPEG | `zune-jpeg` | `=0.5.15` | Pure-Rust JPEG decoding, `zune-core` compatibility plausible; verify complete JPEG, hostile input, defaults/accelerated code, bounds. |
| WebP | `image-webp` | `=0.2.4` | Pure-Rust lossy and lossless WebP paths; verify animated/extended constructs are either completely and safely handled or explicitly excluded under an Owner-compatible profile, without accepting only one frame of an invalid container. |
| AVIF | `oxideav-avif` | `=0.0.11` | Advertises MIT and a pure-Rust default `registry` AV1 path. **Contradictory upstream descriptions** exist: some claim end-to-end decode; some `oxideav-av1` docs describe `NotImplemented` scaffolding. The Agent must prove actual decoded pixels, not `ftyp`, `av1C`, container inspection, metadata or source-level promises. If AV1 decoding is incomplete, mark **FAIL** and do not fabricate a substitute. |

For these precise four roots, inspect each `Cargo.toml`, features, Cargo resolver result, exact *normal/build/dev* dependency edges on Linux and Windows targets, transitive licence expressions, checksum/provenance, MSRV, `unsafe`/FFI/native assembly and build scripts. A name that declares MIT/Apache is **not** proof that every transitive is approved or that distribution notices are satisfied. Do not assume that `default-features = false` is sufficient: inspect which features the root truly requires for decoding and whether another dependency enables forbidden features additively.

**Primary upstream leads (untrusted until verified from exact crate source):**
- `https://docs.rs/crate/zune-png/0.5.2/source/Cargo.toml`
- `https://docs.rs/crate/zune-jpeg/0.5.15/source/Cargo.toml`
- `https://docs.rs/crate/image-webp/0.2.4/source/Cargo.toml`
- `https://docs.rs/crate/oxideav-avif/0.0.11/source/Cargo.toml`
- `https://docs.rs/oxideav-av1/latest/oxideav_av1/` — documentation may be inconsistent with the AVIF crate's promises; actual tagged source and pixel tests decide.

### B. Previously failed roots — evidence-preserving counterchecks only

The accepted `SOLO-WO-067 R01` STOP found (i) `avif-decode =3.0.0` x86 `rav1d` default `asm` and NASM/`cc` build surface, (ii) transitive `avif-parse 2.1.0` `MPL-2.0` licence, and (iii) `image =0.25.10` `png 0.18.1` + `flate2` resolving two `miniz_oxide` versions. The current `[bans] multiple-versions = "deny"` and all current denied licences/sources/unsafe limits remain intact.

Recheck these **specific** failures against exact upstream manifests / Cargo feature tree where useful, recording any true resolution changes rather than assuming that the former issue has disappeared. Do **not** add any of these roots to Solo, enable `image`'s AVIF-native path, substitute `zenavif` under an AGPL/commercial licence, turn on external `dav1d`/libavif, install NASM, execute a new native build, use `[patch]`, or waive an advisory/licence/multiple-version check. `MPL-2.0` may be compatible in some distribution structures but this Owner has **not** approved its compliance/notice obligations: report as a **separate decision needed**, not an automatic Apache-2.0 incompatibility or a silent permission.

### C. Optional last bounded contingency (read-only search, no new code)

Only if the §3A four-root trial cannot produce **actual valid AVIF pixels**, the Agent may inspect publicly published metadata and versioned source of **up to two** alternate *AVIF decoder* packages chosen for their evidence of a complete permissively licensed, pure-Rust and assembler-free decode path. Record exact names/versions and why each was investigated. Such alternates **may not be compiled, installed into Solo, included in any technical branch, or treated as qualified by this order**; they are input to a possible next Owner decision. No open-ended crate hunt. Licence-incompatible AGPL/GPL/commercial alternatives may be mentioned as explicitly rejected background, not executable candidates.

## 4. Required research and security procedure

### 4A — Verify the actual baseline before issuance

Verify the canonical root and sole checkout/worktree/`origin`, `main == origin/main` at §1's exact SHA/tree, branch inventory, no sequencer/conflicts and no unrelated staged/dirty/unknown material. The Owner-installed R01 `ACTIVE` must be the **only** permitted tracked pre-issuance difference. Read and verify the full installed order and matching handoff. Verify four WIP slots, report, lock, toolchain, accepted source types, frozen architecture dependency allowlists and original CI. Fail closed on drift; do not overwrite other work or use reset/clean/force.

### 4B — Issuance (one path, two-host baseline CI)

Stage only `.documents/WIP/workorder-active.md`, commit to `main` with one parent, push normally to `origin/main`, verify remote commit/tree/blob. The other three WIP files and canonical report must still be byte-identical at this issuance revision. Require **completed green** `rust-verify` on both `ubuntu-24.04` and `windows-2025` at that exact issuance SHA, with no new failures, ignored tests, weakened skipped steps or suppression. If CI is red/unverifiable, STOP and use report-only closure if safe. Do not create a task branch.

### 4C — Probe on disposable manifests, *not* the Solo workspace

1. Establish a reproducible per-probe manifest, exact crate versions, target triples (`x86_64-unknown-linux-gnu` and `x86_64-pc-windows-msvc` at minimum), explicit features and offline/network switches. Keep every generated lock and source in task scratch and exclude old Solo `Cargo.lock` from mutation. Verify checksums and code origin from crates.io's published package, not only docs.rs or a README.
2. Collect `cargo metadata` / `cargo tree` including normal/build/dev and feature-unification paths for every relevant root and for the **combined** four-root graph. Record all newly required package names/versions, direct and transitive edges, any duplicate package names at different versions, yanked/supply-chain concerns, and exact build-script/native/FFI/assembly requirements. Run a **candidate-only** strict `cargo-deny 0.20.2` equivalent with the *unchanged* Solo policy whenever the scratch manifest and scanner support it; a failure is evidence, not something to ignore. Never claim the existing green Solo security workflow cleared an uncommitted scratch graph.
3. For each root, check actual licence SPDX expression, chosen available permissive branch where OR permits, source/attribution/NOTICE duties, crate checksum, maintenance signals, MSRV, and current RustSec/OSV/GHSA/vendor notices with dates. Distinguish **clear**, **affected**, **not checked** and **requires Owner legal decision** per transitive. An empty advisory search hit is not proof of vulnerability absence.
4. For each root, test actual **full decoded pixels** on **small, task-created or clearly licenced** sample bytes *only if* the tested crate can safely be compiled under the current toolchain and without a forbidden build tool. Check output dimensions/pixel content, supported variants, full-file consumption and invalid/truncated/header-only refusals. For AVIF, test an independently valid AVIF and show an exact decoded pixel checksum or explicit `NotImplemented`/error; container inspection does not qualify. Do not import unknown corpus or execute arbitrary installed native utilities. If Windows local linking is unavailable, mark native Windows runtime **UNVERIFIED** rather than claiming it; a later code order will require actual two-host CI. No first-party `unsafe` and no system/toolchain installation.
5. Determine whether real decoder APIs can impose the eventual 20 MiB source / 1–8192 dimensions / 32 M pixels **before** potentially unbounded allocation, with a defensible working-memory and CPU cost bound for high-bit-depth, alpha, tiled/animated inputs and decompression bombs. If the crate has no auditable budget/cancellation path, mark **UNVERIFIED/FAIL** and do not treat normal small-image success as sufficient. Document whether stricter decoding exclusions would conflict with binding Architecture §6.2I and require Owner approval.
6. Calculate exactly which architecture graph-allowlist declarations, transitive-closure entries, Cargo lock versions, explicit permitted SPDX licence entries and CI `edges=…` assertions would be needed **in a future code order**. Do **not** edit those files now or invent approval. Separate unchanged accepted pins from new components and any newly discovered incompatibility.

### 4D — Decide and record a ranked, evidence-constrained recommendation

Use a succinct comparison matrix, not an open-ended brainstorming note. For the four-root candidate, record one of:

- **`STATICALLY PROMISING — NOT IMPLEMENTATION-APPROVED`** only if every named root's real source, full resolved graph, compatibility, licensing and existing no-duplicate/no-unsafe/no-native-tool policy check support it. Even then actual two-host decoder runtime, workload bounds, architecture checker and A1 remain **UNVERIFIED** until a separately issued code order. Never label this `SAFE`, `COMPLETE MEDIA`, `C3 ACCEPTED` or `green CI for candidate`.
- **`BLOCKED`** where any mandatory property fails, with the smallest independently supported root cause and alternatives requiring Owner decisions.
- **`UNVERIFIED`** where an essential probe could not run, including missing linker, missing sanctioned test vector, source disagreement, unavailable registry or negative advisory review.

If there is no qualified full four-format route, recommend **one** next smallest decision-ready option for the Owner (e.g., authorize a specific new AVIF root; separately assess actual `MPL-2.0` obligations; or ask whether a PNG/JPEG/WebP-only nonpublishing increment is acceptable). A subset is **not** a complete four-format verification and may never be presented as such. Do not change the policy, accept AGPL/commercial conditions, approve a licence or choose publication scope on the Owner's behalf.

### 4E — Canonical report and final exact-head CI

Overwrite only `.documents/Reports/execution-report.md`. Include the starting/issuance anchors, exact probe manifests and derived feature graphs (condensed), each candidate root's SPDX/provenance/advisories, full new transitive list and duplicate detection, native/toolchain concerns, actual decoded-pixel tests with test provenance or why not run, resource ceiling assessment, contradictions/unknowns, ranked recommendation and required Owner decisions. Record `PASS/FAIL/UNVERIFIED/NOT APPLICABLE` at each gate; separate source-derived statements from measured probes and hypotheses. Report the `Unicode-3.0` inherited warning as **UNVERIFIED**, no suppression. Include exact scratch inventory and cleanup, preserved user data, no outside permanent writes, no new branch/code/lock/policy change. Status may be **`COMPLETE` for the bounded *research task*** if those investigations were performed and the outcomes accurately recorded, even if the code candidates are `BLOCKED`; it cannot assert media completion. Use **`STOPPED`** for authority/safety/identity failures or blocked publication of a safe report.

Commit this **single report path only** to `main`, push normally, and require completed green Ubuntu/Windows `rust-verify` on its exact final report HEAD. Confirm remote report blob/tree/parent readback; the report **must not embed its own future final SHA**. Give final SHA/tree/run IDs in chat. No unrelated files, archives, stashes or branches. Git history preserves this superseded STOP and the new research results.

## 5. Acceptance matrix — qualification-only

| Gate | Required evidence |
| --- | --- |
| `S068-01` | Exact current baseline SHA/tree/report/WIP/branches, single checkout/worktree/`origin`, clean safety and matching installed Owner handoff. |
| `S068-02` | One issuance commit changing ACTIVE **alone**, frozen order readback, completed green two-host CI at issuance SHA. |
| `S068-03` | All other WIP slots unchanged; exactly four files; old R03.C3 STOP not reclassified as success; no branch/PR/product edits. |
| `S068-04` | Four exact root names/versions in §3A are assessed from official published source and full manifest/feature/target graph; actual contrary evidence overrides README marketing. |
| `S068-05` | `png 0.18.1`/`miniz_oxide` and `avif-decode`/`rav1d`/`avif-parse` earlier STOP causes are correctly confirmed, superseded by proof or recorded unresolved; no silent waiver. |
| `S068-06` | Combined proposed graph: exact roots/transitives, duplicate detection, targets, build scripts/assembler/FFI, original accepted pins and needed future architecture-whitelist edits are documented without editing them. |
| `S068-07` | Every licence expression, actual notice obligation, source/checksum, advisory/yank posture, unsafe surface and component freshness is classified truthfully; no unverified security or legal decision recorded as PASS. |
| `S068-08` | Actual pixel decode/complete-stream/resource probe outcomes documented when safely runnable; otherwise explicit `UNVERIFIED` with the host or safety reason; no header-only or fake AVIF proof. |
| `S068-09` | One bounded Owner-decision-ready four-format recommendation or accurately BLOCKED matrix; no conditional research result promoted to a code order or approval. |
| `S068-10` | Issuance and report-only final CI green on both hosts, existing workspace 407/412 regression counts or explained legitimate change, no ignored tests or security/architecture bypass. |
| `S068-11` | Final report-only commit, exact remote blob/tree/parent readback, preserved divergent branch, four WIP files and unchanged product/lock/policy/workflow, clean worktree and only task-owned scratch cleanup. |

## 6. Strict STOP conditions and handoff

**STOP without technical implementation** on any baseline/Owner-handoff mismatch, unknown dirty/staged content, unexpected branch or root, unapproved package substitution, dependency/advisory/rights risk that would require a production exception, unreviewed build-script/native executable, unreliable source/download, inadequate decode proof, unsafe scratch cleanup, architecture/policy edit, red mandatory CI, accidental source or lock change, an attempt to waive duplicate-version policy, or a request to start R03.C3 code without a new exact order. A *blocked* experimental decoder is an ordinary **FAIL conclusion of this research**, not a reason to edit source, policy or NEXT. If reporting the failure is safe, publish the canonical report as in §4E.

**No separate permission is implied:** This order permits only investigation. It does not select an approved runtime decoder, grant permission to add nonpermissive licences or notices, soften `[bans] multiple-versions`, alter CI/toolchain, implement R03.C3, close ROADMAP, touch retained historical branches, publish media or start Stage 04. Preserve `R03.C3` / `R03.C` / Stage 03 as **OPEN**. The Implementation Agent does not self-ACCEPT. Return factual final SHA/tree/report blob and both-host CI to the Chief Architect for independent `ACCEPT / REVISE / STOP` of this **qualification-only** order.
