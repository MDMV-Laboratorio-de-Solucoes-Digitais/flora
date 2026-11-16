# Feature Specification: 033 — Wire Seed search UI

**Branch**: `feat/033-search-ui` · **Status**: Completed (PR #246)  
**Assessment**: `.specify/assessments/033-next-vertical-slice/decision.md`

## Goal
Turn the decorative header search box into a working, accessible search over the verified Postgres FTS endpoint.

## Scope
- `SearchResult`/`SearchResponse` models, `ApiClient.search()`, pure `search.ts` helpers, `SearchBox.svelte` with debounced `$effect`, loading/empty/error states and a11y; replace the decorative input in `(app)/+layout.svelte`.
- CI-enforced `tests/unit/searchApi.test.ts` (Vitest `server` project) + optional informational `tests/e2e/search.e2e.ts`.
- Governance drift closed (OBJETIVO/ROADMAP ranges, historical 442 annotation, 032 snapshot banners, validation prose).

## Non-goals / non-claims
Do NOT claim ADR-0002 #2 met; no FASE 7 Documents, cargo-vet promotion, tag/release or load/restore.

## NV
Client-component tests are not a merge gate (CI runs server project only); e2e informational; search i18n/CJK relevance; result navigation not implemented (results show type/snippet only).
