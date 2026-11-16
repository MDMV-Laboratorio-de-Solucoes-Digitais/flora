# Research: 033 — R1 synthesis

- **Search UI:** GO with 3 guardrails — CI runs only Vitest `server` project (`tests/unit/*.test.ts`); Playwright matches only `**/*.e2e.ts` and is informational; backend `type`/`source` are open strings. Exact 8-file edit list with drop-in code and `$effect` teardown + `cancelled` guard.
- **ADR-0002 #2:** GO for runtime FE config, WS Origin check and DB-free tests, but NO-GO to mark the criterion met without live preflight + bearer select-organization proof, browser regression and ADR note.
- **Documents:** minimal design (one `documents` table + thin CRUD + fourth FTS branch + textarea editor, no CRDT) is P1 for 034; 19 touch points, CAS version token, known compile traps.
- **Frontend audit:** beyond search, false affordances include DMs stub, AI panel, user invite (404), fake login fields, hardcoded identity, dead telemetry logger and orphan components/stale specs.
- **Post-032 audit:** CORS/CI/zero-tags VERIFIED; residual doc drift (OBJETIVO/ROADMAP ranges, 442 annotation, 032 snapshot banners, validation prose).

## Decision drivers
Highest user-visible capability gain with a verified backend and deterministic CI test; defer security/product greenfield slices that are larger or need live proof.
