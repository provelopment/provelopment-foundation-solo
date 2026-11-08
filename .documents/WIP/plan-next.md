# Foundation Solo — plan-next (Next Candidate)

**Candidate:** `R02.B` — Stage 02, One-Entity EFS identity and publication authority. A single non-authorizing candidate; readiness only, never an instruction to execute.

**Objective.** Fix the single validated authoring/publication manifest and its provenance-safe source declarations — one `content/publication.json` as the sole publication authority, strict UTF-8 JSON with the mandatory root `schemaVersion`, and per-representation JSON/Markdown candidate locators from which exactly one effective source is selected — grounded in `ARCHITECTURE.md` §1.5–§1.6 and §6.2B–§6.2C.

**Satisfied prerequisites.** `R02.A` is closed and accepted at source revision `1d4d962294999a749d21b6d305ed45f9066f45a9`, leaving the verified four-crate inward-only skeleton, its two-host gates, the accepted identity grammar and the closed manifest keys, EFS container shape and qualified-reference shape in place.

**Readiness.** `READY` — dependency-ready with no unresolved architectural gap identified in advance. Execution still requires its own Owner-issued work order.

**Bounded future deliverables.** Strict parsing and refusal of the declared manifest shape, schema-version validation, qualified EFS reference resolution over the accepted lineage, declared candidate locators, and effective-source selection with absence separated from error, plus the applicable architecture and security gates. No route, visibility, rendering, UI, provider, release, new package or application feature is implied.

**Future acceptance and refusal criteria.** Malformed JSON, a duplicate member, an unsupported `schemaVersion`, an unknown field, a second manifest or any discovery path, an orphan or cross-parent declaration, an undeclared or empty candidate set and every access, traversal or ambiguity error must each be refused. Declaration and filename presence must never publish, an existent invalid JSON source must never fall back to Markdown, and the two bodies must never merge.
