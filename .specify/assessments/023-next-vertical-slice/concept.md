# Concept: 023

1. **PG-authoritative search:** `GET /api/v1/search` always executes Postgres FTS; Meilisearch is removed from the request path and remains a dormant, explicitly non-authoritative capability (`flora-search` crate + compose `search` profile preserved for later gated re-enablement).
2. **Explicit capability contract:** response gains `source: "postgres"`; contract tests assert this even when `search_service` is configured with an unreachable endpoint (deterministic, no Meili service needed).
3. **Cheap PG FTS hardening:** normalize `-`, `.`, `_` separators in generated `tsvector` expressions (additive migration recreating derived columns/indexes) so hyphenated names match sub-token queries.
4. **Config/docs:** align `SearchConfig::default().enabled` with the Seed-oriented `.env.example`; document remaining `FLORA_SEARCH__*` keys and the dormant-capability/degradation contract.
5. **Evidence:** DB-gated contracts on disposable PG18 + local fmt/clippy/tests + CI green. No frontend changes, no new services.
