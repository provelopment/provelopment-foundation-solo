# Foundation Solo — plan-next (Next Candidate)

## R01.C — Version-Aware Component Security and CI Advisory Gate

**Objective:** verify each exact component and supply-chain boundary in the four-crate Rust workspace, then add genuinely executable, fail-closed vulnerability/supply-chain checks to the existing CI without weakening accepted gates.

**Readiness:** the locked four-crate Rust workspace and both-host CI are installed; browser/HTTP/Page is not yet installed and supplies no test target.

**Proposed bounded scope:** enumerate the complete `Cargo.lock` graph (normal/build/dev/test), first-party attack surfaces, `rustc`/Cargo/rustup, the checkout action's exact commit, hosted runners and any tool to be introduced. Cross-check current RustSec, OSV/GHSA, crates.io yank/malicious notices and official advisories; assess version applicability, provenance, toolchain/action support and mitigation. Choose a minimal verified scanner with pinned provenance, a fresh identifiable advisory database, no secret or write-token exposure and failure-propagating CI on applicable findings; include periodic rechecks so a new advisory is not missed between source commits. Investigate the checkout v4.3.0 Node.js-20/deprecation warning and independently verify any proposed secure replacement pin before changing it. Upgrades or new tools require exact Owner-approved implementation scope, not automatic latest-version selection. Preserve existing CI, tests, architecture and locked reproducibility.

**Exit:** per-component risk ledger with advisory IDs and exact affected/patched ranges; live results from a current database at the final revision; mandatory CI security checks run (not placeholders), demonstrated expected refusal on a safely controlled known-vulnerable fixture where supported, and both-host regression checks. Critical/high applicable or malicious findings are fixed and verified or remain explicit blockers; all other findings are triaged. Do not claim browser tests until an actual authorized surface exists.
