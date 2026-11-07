# Foundation Solo — plan-next (Next Candidate)

## `R01.A` — Reproducible minimal source layout, typed configuration and deterministic local build

**Candidate only.** This single bounded candidate is derived from the unchanged `R01.A` bullet in [ROADMAP](plan-roadmap.md) Stage 01 and the conditionally selected stack recorded in [Architecture](../Architecture/ARCHITECTURE.md) §6A (`E01`–`E10`). Selecting it here is **not** a work order, grants no execution authority, authorizes no Stage 01 work, and decides nothing.

**Objective.** When — and only when — an Owner-issued code-bearing work order exists, establish the minimal reproducible engineering foundation: the selected source layout, strictly typed configuration, pinned tooling and a deterministic local build, without borrowing another repository's runtime.

**Prerequisite and readiness.** `R00.G` is administratively closed as a documentary package, and Stage 00 was independently reviewed and administratively closed under the issued `SOLO-WO-025 R01` process, so `R01.A` is the single NEXT candidate. A complete **unissued** first code-bearing draft was prepared by the Chief Architect and presented for Owner review; **it is not an executed or Owner-issued order**, and no Owner approval of `R01.A` exists. No application code, installed dependency, toolchain pin file, lockfile, generated artefact or executable proof exists; `C1`–`C4` remain `UNVERIFIED` and `F01`–`F17` remain `NOT RUN`. Explicit Owner approval and a subsequent, separate exact issuance will be required before any source work; nothing in this candidate is currently met.

**Proposed bounded scope (future work order only).**
- The selected **conditional** stack exactly as pinned by reference in §6A.2 — Rust 2024 edition with `rustc 1.99.1` and a tracked `rust-toolchain.toml`; the four-package workspace `solo-domain`, `solo-application`, `solo-adapters`, `solo-cli` with one tracked shared `Cargo.lock` — and no alternative, substituted or invented toolchain, package manager or framework.
- A minimal deterministic clean-room installation and `cargo build --release --locked` build from a verified compatible revision.
- Strictly typed configuration for the authored base URL of §4.1 and the Entity identifier, validated and refused on contradiction, with no implicit defaults.
- No historical Foundation compatibility runtime, no shared ingress, and no speculative feature shell for unimplemented Stage 02+ behaviour.
- No provider or hosting coupling of any kind: the core stays delivery-neutral and Vercel-independent, and no Vercel API, project, DNS, TLS or secret action is included.

**Verifiable exit conditions (future).** A clean-room install and build reproduce deterministically from the pinned inputs; configuration validation accepts valid authored values and refuses invalid ones; the dependency graph contains no core-to-adapter direction and no forbidden historical runtime; and the resulting change contains only the authorized skeleton with static checks green. Each of these is a **future gate**: none may be reported as passing before it is actually executed, and none has been executed now.

**Not authorized by this candidate:** any Stage 01 or later implementation, any code, test, CI, dependency installation or schema generation, any `R01.A` work order or agent prompt, `R01.B`/`R01.C` — which remain in ROADMAP with their separate scopes undecided here — any provider or deployment action, and any release. Stage 00's documentary closure under `SOLO-WO-025 R01` approves no source work and grants no execution authority; the unissued draft presented for Owner review must not be treated as installed, approved or activated, and a separate exact issuance remains mandatory before any source work.
