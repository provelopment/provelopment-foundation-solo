# Foundation Solo — plan-next (Immediate Candidate)

**Purpose:** Exactly one next candidate (or `NONE`) while current work is governed by `workorder-active.md`. This file is planning/reference only and is **not executable**.  
**Status:** `CANDIDATE — NOT AUTHORIZED` (current work governed by `SOLO-WO-002`)  
**Last reviewed:** 2026-11-04 (after the initial public publication under `SOLO-WO-002`)

## Next candidate

**Architectural acceptance of the first implementation contracts.** The Architect should bring one compact decision package to the Owner covering, at minimum:

1. **Identity.** Entity/Facet/Site/Page identity and ID grammar, including how a localized representation is keyed by a full locale tag.
2. **Minimal authoring data structure.** The smallest structure that expresses the Entity → Facet → Site → Page → representation hierarchy in authored source, without yet choosing a storage technology or file format.
3. **Base URL and route grammar.** Origin plus optional base path as configured data, lowercase public canonical paths, exact-case unlisted Page-route tokens, and full-locale path segments that normalize consistently.
4. **Cascading property types.** Starting with location and graphic roles: declarable at any EFS level, nearest direct ancestor wins, explicit suppression honoured, no inference from route, host or locale.

**Explicitly excluded from this candidate:** choosing a framework, runtime, language, storage engine or test tool; authoring application code, dependencies or CI; and settling any schema by guesswork rather than accepted decision.

**Not executable.** This candidate is a proposal for the Architect to develop and for the Owner to accept; it authorizes nothing. A first implementation checkpoint — and any framework, runtime, storage or test-tool decision — requires its own exact Owner-issued `SOLO-WO-*` order installed in `workorder-active.md`.

WIP may contain only this file, `plan-outlook.md`, and `workorder-active.md`. Do not create more `plan-*.md`, `workorder-*.md` or report files.
