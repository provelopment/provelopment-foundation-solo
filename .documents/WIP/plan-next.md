# Foundation Solo — plan-next (Immediate Candidate)

**Purpose:** Exactly one next candidate (or `NONE`) while current work is governed by `workorder-active.md`. This file is planning/reference only and is **not executable**.
**Status:** `ONE CANDIDATE — non-executable`
**Last reviewed:** 2026-11-04 (canonical reconciliation: strict hexagonal contract, contact scope, neutral status)

## Next candidate

**Prepare the first implementation contract dossier — decisions only, no code.** A future Owner-issued work order would need, at minimum:

1. Entity/Facet/Site/Page identity and ID grammar, the minimal authoring manifest, and whether incomplete drafts may exist.
2. Authored base URL and route grammar: origin/prefix validation, default Site paths, canonical and alias rules, and fail-closed collision handling.
3. Typed property encoding for the five-level cascade (location, graphics, suppression) plus encoding of the approved `visibilityDefault`/`visibility` declaration.
4. A minimal hexagonal module skeleton and port catalogue: the first inbound/outbound ports, composition-root wiring, and the import/boundary check that will enforce inward-only dependencies.
5. Contact ports for the included `mailto:` action and the **simple** contact form — field set, delivery adapter, anti-abuse controls and retention. Advanced contact forms remain excluded.

This is a dossier of decisions, not an implementation task. It becomes executable only when the Owner issues an exact `SOLO-WO-*` order installed in `workorder-active.md` with a matching handoff.

## Standing report-only exception

Only one permission survives an empty ACTIVE slot: an agent may inspect Solo and overwrite, commit and publish **only** `.documents/Reports/execution-report.md` after repository-safety checks, as defined in [`README.md`](../../README.md), [`AGENTS.md`](../../AGENTS.md) and both SOPs. Nothing else is authorized.

**Not executable.** This file proposes no task and authorizes nothing; no item here is an implementation order. Any future work requires its own exact Owner-issued `SOLO-WO-*` order installed in `workorder-active.md` with a matching handoff.

WIP may contain only this file, `plan-outlook.md`, and `workorder-active.md`. Do not create more `plan-*.md`, `workorder-*.md` or report files.
