# Decision: 033 — Wire Seed search UI and close governance drift

**Verdict:** go (scoped)

## Chosen slice
**S1 — Typed search API client + header search box + residual doc drift.**

## Scope (in)
- `SearchResult/SearchResponse` models, `ApiClient.search()`, pure helpers, `SearchBox.svelte` with debounced `$effect`, empty/loading/error states and a11y; replace the decorative header input.
- CI-enforced `tests/unit/searchApi.test.ts` (Vitest `server` project) plus an optional `search.e2e.ts` (informational).
- Governance drift: OBJETIVO/ROADMAP ranges, historical "442" annotation, supersession banners on 032 snapshots, validation prose accuracy.

## Non-goals / non-claims
Do NOT claim ADR-0002 #2 met (needs runtime FE URLs, WS Origin check, live non-browser proof). No FASE 7 Documents implementation, no cargo-vet promotion, no tag/release, no load/restore.

## NV
Client-component tests are not a merge gate (CI runs server project only); e2e informational; search i18n/CJK relevance; no real browser proof of result navigation.
