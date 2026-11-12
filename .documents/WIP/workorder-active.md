# Foundation Solo — Active Work Order

**OWNER-ISSUED — SOLO-WO-071 R01**  
**Title:** R03.C3 — adopt the verified out-of-process AVIF resource-containment architecture and fix the remaining product-adoption gates (architecture-only execution)  
**Repository:** `provelopment/provelopment-foundation-solo` — `https://github.com/provelopment/provelopment-foundation-solo`  
**Sole Owner-local checkout:** `C:\Users\Sean\Documents\Provelopment\00.foundation-solo`  
**Matching Owner → Implementation Agent handoff ID:** `SOLO-WO-071 R01`  
**Authority:** This is the complete Owner-issued operational instruction installed in the single `workorder-active.md` slot. The Implementation Agent carries out the precisely bounded documentation operations below. The Chief Architect has **permanent GitHub READ-ONLY access**; all authorized repository writes belong to the Implementation Agent alone. There is no product-code, dependency, provider, merge, release or deployment authorization in this order.

## 1. Fixed result and acceptance boundary

Record **one selected AVIF resource-containment architecture** in the existing canonical Architecture Charter §6A.8, using the independently accepted `SOLO-WO-070 R01` two-host proof. The selected approach is **a fresh, nonpersistent, OS-contained decoder process per input**, with validated limits installed and verified before its first decoder instruction, a finite parent wall deadline, descendant cleanup, and no promotion of unverified or partial outputs. The result of this order is an authoritative **design/engineering contract**, **not** an approved runtime decoder, an executable product feature, or a released security guarantee.

The resource-control experiment is accepted as evidence **only at its measured scope**: `ubuntu-24.04` used hard/soft `RLIMIT_AS` and `RLIMIT_CPU` plus a process-group watchdog; `windows-2025` used a Job Object, suspended process creation, verified job assignment and limits before resume. The measured proof settings were 536,870,912 bytes address-space/commit, 10 CPU seconds, 15,000 ms wall time per decode. These become the **engineering reference profile to be validated in product implementation**; they are not already installed in Solo, nor a guarantee that every conforming AVIF can finish within them.

Keep all binding product requirements intact: exactly one Entity; strict inward-only Hexagonal Architecture; manifest-only publication authority; 20 MiB maximum image input; image dimensions 1–8,192 on each side; at most 32,000,000 decoded pixels; PNG, JPEG, WebP and AVIF all retained in the first-release set; fail-closed validation for all declared owners including internal, suppressed and unlisted references; no automatic graphic fallback; no publication from the mere presence of a file. `R03.C3`, `R03.C` and Stage 03 **stay OPEN**. **Do not select or add a production AVIF codec in this order.**

## 2. Immutable starting anchors and required reading

Before any Git operation verify the actual one-checkout Solo root, one worktree and one `origin` remote pointing to this exact repository; the Owner-installed `ACTIVE` replacement is the only initial tracked difference.

| Anchor | Exact required value |
| --- | --- |
| Starting `main == origin/main` | `e3a223439e0127f720a6bf2885f9ac65aca3c217` |
| Starting tree | `b27d54d95b31cb42c3c826838d24272604107be0` |
| Current report blob | `05b220d7c4966d73d3237d87a71f4bfee6a32835` (`SOLO-WO-070 R01`, `OS-ISOLATION-PROVEN`) |
| Prior ACTIVE blob | `0d3edb136c54c3a115cd3ec265fae73f2e597fd3` (`SOLO-WO-070 R01`) |
| Architecture blob | `250bf62efd07037fe70022b5fc88a63610692245` |
| NEXT blob | `92fa562b1af0c54391851aa5c79deb1631127069` (neutral) |
| ROADMAP blob | `9d7a86226878f490fd5e26c6813a81e5f73f3f37` |
| OUTLOOK blob | `cb9c7a610cf3c537557e060ca9656e44a053e69d` |
| Retained `solo-wo-061-r01` branch | `4371af21dce67f33cf1df0384738bf353ca80fc6` |
| Retained `solo-wo-069-r01` branch | `e6162cdf733ea3136e96cbc139e46a6daf4054ee` |
| Retained `solo-wo-070-r01` branch | `d270f95cad10db11b67d2cbc68a5c593ecbbc7c8` |
| WO-070 proof CI | `43162966403`, both platform jobs successful |
| WO-070 final report CI | `43163844387`, both platform jobs successful |

Read, in order: `README.md`; `AGENTS.md`; `.documents/About/FOUNDATION-SOLO.md` relevant product boundaries; `.documents/Architecture/ARCHITECTURE.md` §§5, 5A, 6.2C, 6.2G, 6.2I, 6A.8 and 6A.9; Architect SOP §§6A, 6B, 7–8; Implementation SOP §§4–5C and 7A; the four WIP files; the exact WO-070 execution report; and the two branch proof launchers at `solo-wo-070-r01` (`verification/avif-r03-c3/limit-linux.sh`, `limit-windows.ps1`) plus its workflow. **Read those branch files; do not copy, merge, cherry-pick or import them.**

## 3. Exact writable-path and operation ledger

**Only these three permanent paths may change, on `main` only:**

1. `.documents/WIP/workorder-active.md` — Owner installs this complete R01 file; the agent stages and commits this one path **without editing it**, then pushes the one-path issuance commit to `origin/main`.
2. `.documents/Architecture/ARCHITECTURE.md` — edit **only the `## 6A` / `### 6A.8` section**, immediately after its **Asset pipeline** paragraph and before **Single portable sealed artefact**, inserting the exact accepted boundary text supplied in §5 below. Make no other alteration to §6A.8 or any other architecture section. Commit and push this one file separately after the issuance CI succeeds.
3. `.documents/Reports/execution-report.md` — overwrite **once** at run completion with the execution facts, actual checks and final evidence. Commit and push this **one file only**, separately from the architecture commit.

**Every other tracked path is READ-ONLY.** In particular: no `plan-next.md`, `plan-roadmap.md`, `plan-outlook.md`, About, README, AGENTS, SOP, `.github/`, test, `verification/`, product source, `Cargo.toml`, `Cargo.lock`, `deny.toml`, Rust toolchain, license/NOTICE or fixture edits. WIP remains exactly four existing files, without new subfolders or report archives.

No branch creation, PR, merge, tag, release, dependency install, unreviewed external executable, deployment, provider action, secret change, GitHub administration, extra checkout/worktree/nested clone/shadow tree or history rewriting. No Git `reset`, `clean`, stash, force-push, amend or rebase. No change to any preserved historical branch. Permitted networking is limited to the existing authenticated `origin` push, official GitHub CI/status readback, and the existing CI's pinned/verified toolchain and dependency acquisition; **no new package/network research** is required.

The agent may use existing Solo `.temp/` or its provided task temporary workspace for disposable diff/validation output; remove only positively task-owned material and do not touch inherited archives or `target/`.

## 4. Required execution sequence

**4A — Preflight.** In the verified single checkout execute and record the actual outcomes of:

```text
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse HEAD^{tree}
git branch -avv
git worktree list
git remote -v
git status --porcelain --untracked-files=all
git diff --name-only
```

Verify the exact §2 SHA/tree, four WIP blobs, report, Architecture blob, branch inventory and correct sole origin. The installed ACTIVE must say `OWNER-ISSUED — SOLO-WO-071 R01` and the received handoff must name **that exact ID/edition**. Only ACTIVE may differ locally from the remote baseline. If anything else differs or is unaccounted for, **STOP** and preserve it. Do not use a second checkout or destructive cleanup.

**4B — Issue the unchanged order.** On `main` stage **only** `.documents/WIP/workorder-active.md`, commit `Issue SOLO-WO-071 R01: AVIF isolation architecture`, and push normally to `origin/main`. Verify the remote parent is precisely the §2 starting commit, the new remote tree and ACTIVE blob, and byte-identity of every other file. Require completed **successful** `rust-verify` on **both `ubuntu-24.04` and `windows-2025`** at this exact issuance SHA. Red or unverifiable mandatory CI is a STOP: use the report-only closeout where safe, without Architecture changes.

**4C — Apply the exact architecture contract.** Insert the Markdown in §5 as **one** contiguous block after the existing **Asset pipeline** paragraph in §6A.8 and before the existing **Single portable sealed artefact** paragraph. Preserve all existing charter statements, headings, examples and normative contract literals. No other documentation edits. Verify `git diff --name-only` shows only `.documents/Architecture/ARCHITECTURE.md` after issuance. Commit it separately as `architecture: bind isolated AVIF resource-control boundary (SOLO-WO-071)` and push normally. Verify exact remote diff/parent/tree and two-host successful `rust-verify` at this architecture SHA.

**4D — Validate fidelity.** Inspect the rendered Markdown, surrounding §6A.8 text and all references. The new block must occur **once**, contain every fixed rule below, and preserve its own explicit `UNVERIFIED` and future-work obligations. Verify before/after full-text differences are limited to the one insertion. No product build/test on this work order is claimed as evidence of implemented media: existing CI is a *no-regression/document integrity* gate only. Mark new decoder, worker runtime, codec conformance, release and browser checks `NOT APPLICABLE` to this documentation-only scope, not `PASS`.

**4E — Publish the canonical report.** Return to a clean `main`; overwrite only `.documents/Reports/execution-report.md`. Record starting SHA/tree/blobs; installed order ID and handoff; issuance/architecture commits and CI run/job IDs; exact one-block Architecture diff and checks; retained branches and four-WIP inventory; `PASS/FAIL/UNVERIFIED/NOT APPLICABLE` for every §6 acceptance gate; preserved code/lock/policy; real cleanup; explicit next action. State that accepted architecture is **documentary**, with no released decoder, media validation, provider compatibility or security proof beyond WO-070's harness. The report must **not embed its own future final commit SHA**. Stage/commit **only this report**, push and verify remote blob/tree/parent. Require completed successful `rust-verify` **both hosts** at the exact final report HEAD. Include the final report SHA/tree/blob and CI run links in the agent chat handoff.

## 5. Exact Architecture Charter text to insert

Insert the following Markdown verbatim, including its heading. Its normative wording constitutes the adopted resource-containment architectural decision. Keep it in the existing §6A.8; do not open a new architecture file, ADR, register, policy or WIP item.

---

**AVIF out-of-process resource boundary — accepted engineering design (`SOLO-WO-071 R01`; not an implemented adapter).** The R03.C3 AVIF media verifier shall run each untrusted image decoding attempt in a **new, bounded, single-use child process**, with the parent retaining exclusive publication authority. The verified `SOLO-WO-070 R01` experimental Linux and Windows harness establishes that OS-level resource controls can be installed before child execution and can deny deliberate memory, CPU, wall-time and descendant escapes on the tested hosts. This result selects **out-of-process resource containment** as the AVIF engineering direction; it neither selects `oxideav-*` as a production dependency nor certifies complete AVIF conformance, host portability, network/filesystem sandboxing or released functionality. In-library allocation controls remain welcome but are not assumed to exist.

1. **Trusted parent, untrusted bytes.** The parent receives only bytes already obtained through the authorized confined-source and manifest-declaration pipeline. It checks the 20 MiB source byte maximum **before launching a worker**. Any parsing of untrusted AVIF media headers, including `inspect`/`ftyp`/`av1C`, occurs **inside the already-contained worker**, never in an unbounded parent process. Inside the worker, apply checked integer conversion and multiplication, refuse either dimension outside 1–8,192 or product above 32,000,000, and only then attempt full decoding. File name, extension, magic, metadata or a partial plane never alone suffice. MIME/magic and complete media consistency remain whole-candidate publication gates.
2. **Limits before execution.** Each worker uses the reference engineering profile of **536,870,912 bytes** for child OS address-space/commit, **10 CPU seconds**, and a **15,000 ms parent-observed wall deadline**. On Linux, install and read back hard/soft `RLIMIT_AS` and `RLIMIT_CPU` before `exec` in a fresh process session, then terminate/reap the entire process group on every terminal path. On Windows, create a Job Object with `JOB_OBJECT_LIMIT_PROCESS_TIME`, `JOB_OBJECT_LIMIT_PROCESS_MEMORY`, `JOB_OBJECT_LIMIT_JOB_MEMORY` and `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`; create the worker **suspended**, assign it to the job, query the effective limits, verify membership, and only then resume. Any inability to install, verify, enforce or clean up limits **refuses the image**: no unsupervised fallback. The reference values require product-host validation; they do not relax the accepted image source/dimension/pixel ceilings.
3. **What the limits actually mean.** Linux `RLIMIT_AS` bounds a process's **virtual address space**, not its RSS or aggregate descendants' memory. Windows Job Object memory controls bound **committed** process/job memory, not physical RSS. Windows per-process CPU accounting is periodically enforced: the WO-070 2-second test accrued **5.203 CPU seconds**, so neither 2 seconds nor the 10-second normal CPU setting is an exact hard real-time deadline. The independent wall watchdog is the outer elapsed-time guard, with actual kill/reap completion checked; scheduling overhead must be measured and disclosed. A general Linux multi-process aggregate memory/pid cap, host concurrency cap, denial of service and global resource accounting **are not proven** by `RLIMIT_AS` plus a process group and must be addressed before claiming a production bound for process trees or concurrent images.
4. **Isolation is not a general security sandbox.** `RLIMIT_*`, process groups and Windows Job Objects limit resources and lifecycle; **they do not prohibit arbitrary filesystem, network, credential or system-call access**. A later code-bearing order must prove the worker runs with minimal rights, no secrets, no direct public-output path, no untrusted command/script resolution, no unexpected network capability and no uncontrolled child creation. It must verify executable and helper identity, immutable inputs, platform availability, least privilege, secure temp-directory ownership, symlink/reparse and TOCTOU resistance, handle inheritance/closure, command-line quoting, process-tree accounting and worker protocol validation on real Linux/Windows hosts. An unavailable enforcement primitive produces refusal, never a degraded decoder.
5. **Typed worker protocol, no publication by decoding.** The application defines an inward-owned media-verification port and a typed output/refusal contract. The outer adapter owns OS process mechanics and never puts native OS handles, transport or provider code in `solo-domain` or `solo-application`. The parent passes one content-identified input and bounded worker configuration; the worker emits a length-bounded, versioned result with verified dimensions, bit depth, color/alpha/transform facts, complete plane/output hashes and a typed failure. Treat the worker's result as untrusted: validate every count, byte length, claimed digest and output provenance in the parent. Stage all outputs outside any public address; promote only complete, re-read and independently digested outputs after **normal exit, verified limits, finite deadlines, successful validation and whole-candidate authority**. Crash, timeout, limit trip, malformed input, missing output, mismatched digest, worker/protocol error or cleanup failure yields one refusal and no discoverable or partial output. Never allow an image worker to publish its own result or imply Page/Entity/route authority.
6. **Quality and implementation gates remain open.** WO-069 and WO-070 proved the exact **twelve 8-bit still-image fixture plane digests** and thirteen refusals on both hosts, not overall pixel correctness. **10/12-bit AVIF, grids, AVIS sequences, independent alpha and rotation goldens, upstream provenance inconsistencies and complete decode/format behavior are UNVERIFIED**. The rotated fixture's raw planes were transposed relative to reported geometry; reproduction of its hash does not establish correct orientation. The currently tested `oxideav-avif =0.0.11`, `oxideav-av1 =0.1.23`, `oxideav-core =0.1.43` are **research pins only**. Product adoption must receive a separate Owner-issued code order and A1 evidence on exact dependency/security/licence/notice versions, decoder quality, two-host process-boundary escape/refusal tests, actual confined-source composition, deployment-mode capabilities and whole-publication fail-closed behavior. No additional first-party `unsafe` is approved: preserve the sole existing Windows native exception in `source_read_native.rs` until a separately accepted architecture decision changes it. This architectural direction does not authorize source changes, a codec, Stage 03 closure, release or deployment.

---

## 6. Independent acceptance gates for this exact order

| ID | Required evidence |
| --- | --- |
| `S071-01` | Exact baseline commit/tree/report/Architecture/WIP blobs and branch inventory; one verified checkout/worktree/origin; matching `OWNER-ISSUED — SOLO-WO-071 R01` handoff and only installed ACTIVE differing. |
| `S071-02` | One-path ACTIVE issuance; exact remote parent/tree/blob; pre-existing Linux and Windows `rust-verify` successful on issuance SHA. |
| `S071-03` | Single contiguous insertion into §6A.8 between the existing **Asset pipeline** and **Single portable sealed artefact** paragraphs, with verbatim §5 text and no other Architecture diff. |
| `S071-04` | Explicit distinctions preserved: input check outside child but **all AVIF header parsing inside contained child**; 20 MiB / 1–8192 / 32 M pixels and original PNG/JPEG/WebP/AVIF set untouched. |
| `S071-05` | Linux and Windows pre-execution mechanisms; reference resource values; Windows periodic CPU overrun, Linux per-process virtual address space and unproven global/descendant bounds disclosed. |
| `S071-06` | Document states OS resource containment **is not syscall/network/filesystem isolation**; secure worker identity, privileges, protocol, staging, symlink/race and whole-publication proof are future non-negotiable product gates. |
| `S071-07` | `oxideav` research pins not adopted; high-bit-depth/grid/sequence/alpha/rotation/provenance gaps stay `UNVERIFIED`; first-party `unsafe`, source/lock/policy, ROADMAP/NEXT, branches and product remain unchanged. |
| `S071-08` | Architecture-only commit one file, exact remote readback, `rust-verify` successfully completed on both hosts at architecture SHA. |
| `S071-09` | One report-only final commit and exact remote readback; `rust-verify` green on both hosts at report HEAD; clean main and safe scratch cleanup; no hidden stage closure or release claims. |

## 7. Strict STOP and reporting rules

STOP without Architecture edits on any mismatch of baseline SHA/tree/blob, root, worktree count, remote, branch, installed Owner order/handoff, unexplained dirty/unknown material, historical-branch drift, non-fast-forward push, failed issuance CI or unverified GitHub readback. Once issued, STOP and publish a factual report safely if the exact one-block insertion would conflict with an accepted architectural invariant, if a prohibited path changes, if mandatory two-host CI is red or unverifiable, or if reporting/cleanup cannot be performed safely. Preserve unknown and Owner-owned data; do not use destructive Git mechanics or weaken a check to reach green.

The Implementation Agent **does not** self-ACCEPT the architecture update or issue the next order. The Chief Architect independently returns `ACCEPT / REVISE / STOP` on the **published exact revision**. This order closes **only its documentary task** on successful evidence; R03.C3/R03.C/Stage 03 stay open for a separately issued code-bearing implementation and decoder-conformance qualification.
