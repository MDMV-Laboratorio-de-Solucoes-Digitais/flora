# Research: 019 — Postgres FTS fallback + in-app reply notification

**Date:** 2026-11-16  
**Method:** synthesis of 018/019 multilingual research (EN, pt-BR, es, de, fr, zh-CN, ja), primary PostgreSQL/Macro evidence, and codebase audits.

## Key evidence
- **Current gaps:** `flora-api/src/routes/search.rs` returns 503 when `state.search_service` (Meilisearch-only) absent; `flora-search` has no PG path; `channels.rs::post_message` persists replies (`thread_id`) and emits realtime but creates no notifications; notification model/repo/table/routes exist (`Reply` supported) but core mutation wiring is absent (`flora-notifications` crate unused by API).
- **PG FTS 80/20 (019):** stored/generated `messages.search_tsv` with fixed `'simple'` config + partial GIN (`WHERE is_deleted = false`) gives DB-maintained indexing on writes; safe parsing via `websearch_to_tsquery` with `plainto_tsquery` fallback; rank/snippet capped and tenant-scoped (`organization_id`) preserves contract and cheap VPS posture. Messages-only is highest value/minimal risk; tasks/files/trgm are additive follow-ups.
- **Fallback/degradation:** make Postgres the default/auto backend so Meilisearch absence/disability does not break search (503 only if no backend/DB path usable). Preserve `GET /api/v1/search?q=&type=&limit` and response `{query, results[{type,id,snippet}]}`.
- **Notification 80/20:** reply-in-thread to parent message author is simplest verifiable event (explicit recipient via parent lookup, existing `Reply` type/schema/repo/read APIs, single hook after message persist). Rules: only when `thread_id` resolves to existing parent in same org/channel, parent not deleted, and `parent.sender_id != sender` (no self-notify). Best-effort/non-fatal side effect preferred (do not fail successful post/realtime path).
- **Frontend:** `apps/frontend` has no search/notifications integration; backend-compatible contract/fallback and REST-visible notification require no FE change for 019 correctness.
- **Macro lessons (debloted):** emulate deterministic, idempotent, tenant-scoped derivation from source-of-truth DB state and simple repairability; avoid Kafka/SQS/OpenSearch/workers, ingress/egress queues, push/digest, and heavy extraction/fanout on Seed/VPS.

## Convergent recommendation
Implement **messages-only PG FTS fallback + reply-in-thread notification** with additive migration and DB-gated deterministic tests (disposable PG18). Defer tasks/files search, trgm tuning, outbox/backfill workers, mention/assignment/completion fanout, realtime/push delivery, channel-ACL search filtering, and relevance/i18n/perf benchmarks as NVs.
