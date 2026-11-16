# Problem: 019 — Core search and in-app feedback are not usable without heavy optional infra

## Problem statement
Flora lacks Seed/VPS-usable wiring for two core collaboration primitives: global search depends on Meilisearch and degrades to 503 when unavailable, and core mutations do not emit in-app notifications despite existing notification storage/read APIs. This leaves the self-hosted MVP incomplete on cheap deployments and blocks low-friction user feedback loops.

## Evidence
- **Search:** `flora-api/src/routes/search.rs` requires `state.search_service` (Meilisearch, `flora-search`) and returns `ServiceUnavailable` when absent; `flora-core/src/config/search.rs` is Meilisearch-centric; no `messages.search_tsv`/GIN or PG query path exists in migrations/models/routes.
- **Notifications:** `flora-api/src/routes/notifications.rs` can list/mark-read, and `notifications` table/model/repo/types (including `Reply`) exist, but `flora-api/src/routes/channels.rs::post_message` and task/file mutations emit no notification records; `flora-core` notification service is a stub and `flora-notifications` (Valkey-coupled) is not wired into the API mutation path.
- **Planning/NVs:** 018 decision and `OBJETIVO.md`/`ROADMAP.md` set 019 as essential search/notifications wiring (Postgres-first, verifiable, no new mandatory services).
- **Audits/research:** 019 readiness/hook-point audits and multilingual research converge on messages-only PG FTS fallback and reply-in-thread as the smallest high-value, testable slice.

## Impact if unresolved
- Search is non-functional on default/lean Seed deployments without Meilisearch, undermining discoverability of core content.
- Users receive no in-app signal for relevant collaboration events (e.g. replies), reducing engagement and parity with baseline SMB collaboration expectations.
- Continued reliance on heavier/optional components conflicts with debloat and cheap-VPS operability goals.

## Desired outcome (019)
A verified, Postgres-first slice: `/api/v1/search` serves message results via PG FTS when Meilisearch is absent (stable contract, tenant-safe), and threaded replies create a deterministic in-app `Reply` notification for the parent author (visible via existing notifications API), both covered by DB-gated tests on disposable PostgreSQL 18.
