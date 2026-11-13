# Foundation Solo — plan-next (Next Candidate)

## R03.C3 — Bounded Linux process-group containment proof

**Objective:** Advance the accepted R03.C3 out-of-process media-worker safety boundary with a private, fixed Linux process-group/descendant lifecycle proof. Demonstrate real kernel-observed termination and truthful cleanup refusal; do not claim AVIF decoding, complete host sandboxing or production readiness.

**Prerequisites:** The accepted WO-090 reap-fail-closed correction is integrated by WO-091; the existing single-checkout, strict Hexagonal Architecture, native-seam and architecture-guard boundaries remain intact. Use the accepted Architecture §6A.8 Linux resource-limit and process-group contract. Keep Windows behavior fail-closed pending a separately bounded Windows Job Object proof.

**Proposed scope:** Add one separate, fixed internal Linux-only test mode and focused host tests. Prove a directly owned group leader plus one fixed, bounded test descendant, verified group/session identities, deadline-driven termination, bounded leader reaping, and honest refusal where descendant termination or cleanup cannot be established. Do not weaken the existing descendant-free session readiness modes. No arbitrary executable, PID, command, media input, group ID or caller-controlled resource limits. Preserve all existing Linux and Windows regressions and refusal behavior.

**Deliverables:** Exact source/host-test boundaries; independently observed Linux positive and negative lifecycle evidence; cross-host architecture and CI evidence; explicit limitations of process-group signalling, PID reuse, descendant reaping and aggregate resource containment; a single canonical execution report. No AVIF codec, generalized sandbox, publication, Stage 03 completion or release claim.

**Exit conditions:** Accepted fixed harness behavior and real Linux negative evidence at exact code revision, verified no foreign-process signalling and no unverified-cleanup success, complete two-host CI, no native/unsafe boundary escape, no Windows fallback, no source/limit/protocol drift and independent Chief Master Architect review.
