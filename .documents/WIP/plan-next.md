# Foundation Solo — plan-next (Next Candidate)

## R00.A — Identity and publishing declarations

**Objective:** settle the identity and publication-declaration contract that blocks the first coherent Solo implementation, as recorded decisions only and without executable application code.

**Prerequisites:** no preceding implementation stage gates this package. It depends only on the accepted product and architecture decisions already recorded in [`About/FOUNDATION-SOLO.md`](../About/FOUNDATION-SOLO.md) and [`ARCHITECTURE.md`](../Architecture/ARCHITECTURE.md).

**Deliverables:**

- Stable Entity, Facet, Site and Page identifiers, their uniqueness scope and the grammar that validates them.
- Default Facet and Site naming and creation rules for a new Installation.
- Whether an incomplete draft may exist outside the publishable model, and under what declaration.
- The authoritative manifest and content boundary, plus the integrity rule between authored source and published declarations.
- Fail-closed detection for duplicate and orphan declarations.

**Acceptance:** every item above recorded as an explicit Owner decision in its canonical About/Architecture location; a duplicate or orphan declaration refused with a diagnostic instead of a guess; no application code, dependency installation, release or deployment produced by the candidate.

**Roadmap source:** Stage 00, package `R00.A`.
