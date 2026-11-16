# 018 — Search indexing & in-app notifications wiring for Flora (80/20, VPS)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`b6440c534`) — read-only analysis; this document only.  
**Scope:** 80/20 search indexing and in-app notifications wiring for a self-hosted SMB collaboration app (Rust + PostgreSQL, optional Meilisearch) as a Macro alternative on cheap VPS (target ~2 vCPU / 4 GB RAM). Focus: Postgres FTS/`pg_trgm` as default, event/outbox or direct hooks from core mutations (messages/tasks/files) to search index and notifications, minimal notification model/delivery (in-app), debloating vs Macro's heavier pipelines, and what to defer. Cross-checked with `OBJETIVO.md`, `ROADMAP.md`, 016/017 decisions, `flora-search/`, `flora-notifications/`, `flora-api/src/routes/{search,notifications,channels,tasks,files}.rs`, `flora-core/src/services/*`, and DB schema/migrations.  
**Method:** repo inspection + synthesis of prior Flora research + multilingual web research (**EN, pt-BR, es, de, fr, zh-CN, ja**). No runtime benchmarks, no source changes, no commits. Locale-specific claims and VPS performance are planning-level unless evidenced (**NV**).

## 1. Flora cross-check (current state)

| Area | Evidence | Gap for 018 |
|---|---|---|
| North star / roadmap | `OBJETIVO.md` §2/3.4: P0 wants Postgres FTS integrated to core objects and minimal in-app notifications/events; gaps include “wiring de indexação e eventos→notificações para mutações core”. `ROADMAP.md` P0.2: wiring essencial busca/notificações com verificação determinística. 016 chose operability (C), 017 chose secure binary files (B). | 018 is the natural next P0 wiring slice after B/C. |
| Search impl | `flora-search/src/lib.rs`: Meilisearch-only (`index_message/task/file`, `search`, per-org index). `flora-core/src/config/search.rs`: enabled by default, URL required when enabled. `flora-api/src/state/app_state.rs`: optional init/degraded mode already implemented (016 outcome). `flora-api/src/routes/search.rs`: returns `503 ServiceUnavailable` when disabled/unavailable; otherwise queries Meilisearch. | No Postgres FTS backend/indexes; no mutation hooks populate any index (Meilisearch methods unused by app routes/services). Search works only if external engine configured and fed. |
| Schema | `20240818000001_init_schema.sql`: `messages.content`, `tasks.title/description`, `files.name` (+ metadata/checksum/workspace_id via 017 migration) exist; **no `tsvector`, GIN/FTS, or `pg_trgm`**. `notifications` table exists (user-scoped, `event_type`, `target_id/url`, `is_read`, org denormalized). | Adding PG FTS requires additive migration + query path. Notifications storage primitive exists but producer side is missing. |
| Mutations (core objects) | `flora-api/src/routes/channels.rs`: create/post/edit/delete message persists and emits in-process realtime hub events only. `tasks.rs`: create/update/delete persist only. `files.rs`: upload (multipart/metadata) and delete go through `FileService`/repo; no search/notification side effects. | No search indexing and no notification creation on any core mutation. |
| Services | `flora-core/src/services/{message,task,file,notification}_service.rs`: domain/repo logic only; core `NotificationService` is a stub wrapper; `MessageService` effectively unused. `flora-notifications/` crate provides `NotificationService {db, redis}` with `create/get_unread/mark_read` and Valkey `XADD` stream publish, but **not wired into `AppState`/routes** (app uses core repo directly). | Dual notification abstractions; no application-level event producer. |
| Notifications API | `flora-api/src/routes/notifications.rs`: list (user-scoped, unread/all, paginated) and mark-read with ownership check — read-side mostly usable. | No creation path from domain events; no realtime/push delivery to clients (Valkey stream in `flora-notifications` unused). |
| Realtime | In-process `broadcast` hub + WS envelopes for message events (`channels.rs`, `realtime/*`); Valkey pub/sub optional infra exists separately. | In-app notification delivery can piggyback later, but P0 only needs durable in-app records + polling/list endpoint. |

**Implication:** Flora has primitives (tables, repos, optional search/messaging infra, degraded boot) but lacks a **single, cheap, authoritative wiring path** from mutations to searchability and user-visible notifications. For Seed/VPS, default path must be Postgres-only, synchronous/transaction-adjacent in effect, and failure-tolerant (best-effort side effects must not corrupt core writes).

## 2. Multilingual evidence (EN, pt-BR, es, de, fr, zh-CN, ja)

| Lang | Evidence focus (representative) | Takeaway for 018 |
|---|---|---|
| **EN** | PostgreSQL docs: FTS (`tsvector`, `GIN`/`GiST`, `to_tsvector`/`plainto_tsquery`/`websearch_to_tsquery`, ranking, generated columns/triggers) and `pg_trgm` for similarity/fuzzy on names; transactional consistency vs external engines. Outbox/transactional-outbox patterns (reliable side effects without distributed tx) and “sync in-request + async rebuild” pragmatism for small scale. Meilisearch/docs + prior Flora 016-search: strong typo-tolerance/instant UX but extra process/memory/sync. | For <~10⁵–low 10⁶ docs and “search-as-feature”, **Postgres FTS in-DB is the 80/20 default**; keep Meilisearch optional profile. Prefer **direct/transactional hook or simple outbox table** over brokers; best-effort secondary effects. |
| **pt-BR** | Consultas/sinais: “indexação PostgreSQL full-text tsvector pg_trgm notificações in-app outbox tabela Postgres Rust VPS 4GB sem Kafka”. Prática de self-host em VPS pequena favorece menos serviços, fila no próprio Postgres e gatilhos/updates simples; notificações como registros consultáveis (sino) antes de push complexo. | **Postgres-first, outbox simples opcional, polling de notificações**; evitar Kafka/Redis Streams obrigatórios. |
| **es** | Consultas: “búsqueda en PostgreSQL tsvector vs Meilisearch VPS barata”, “notificaciones in-app tabla Postgres patrón outbox sin colas externas”. Consenso: indexar en transacción o job ligero en DB, notificación = fila + lectura paginada. | Hooks directos/transaccionales mínimos y modelo in-app por tabla. |
| **de** | Consultas: “PostgreSQL Volltextsuche Trigger tsvector GIN Outbox Pattern Benachrichtigungen Tabelle VPS wenig RAM”. Betrieb: konsistente, wartbare Kernpfade; optionale Dienste degradierbar. | Deterministisch, wenig Komponenten, klare Degradation bei Such-Backend-Fehlern. |
| **fr** | Consultas: “indexation recherche PostgreSQL tsvector pg_trgm notifications in-app outbox sans Kafka auto-hébergé VPS 4 Go”. Privilégier cohérence transactionnelle et simplicité opérationnelle. | PG FTS + table `notifications`/outbox comme base fiable. |
| **zh-CN** | 资料/查询: “PostgreSQL 全文检索 tsvector GIN pg_trgm 站内通知 outbox 事务消息表 低配VPS Rust”. 小规模优先数据库内方案、事务一致或最终一致的简单补偿；站内通知先落库再拉取。 | **DB内索引+通知落库**，避免引入重消息队列；Meilisearch 按需。 |
| **ja** | 検索: “PostgreSQL 全文検索 tsvector GIN pg_trgm アプリ内通知 outbox パターン VPS 4GB”. 小規模自前運用ではDB集約・同期的/準同期的な簡易配線が運用容易。 | 小さく検証可能な配線（直結 or 単純outbox）を優先。 |

**Synthesis:** Across locales, the dominant 80/20 for cheap VPS is **Postgres as source of truth + in-DB search + in-app notification rows**; external search and stream/push infra are optional enhancements. Macro-like breadth (cross-surface ranking, ML, complex fanout pipelines) is explicitly out of scope.

## 3. 80/20 design options (and fit)

| Option | Pros | Cons/Risks | VPS/Macro-debloat fit |
|---|---|---|---|
| **A. Direct hook in mutation path (same tx/service boundary), best-effort side effects** | Simple, no new tables/workers, immediate consistency for PG FTS (`tsvector` in-row/generated). | Can couple latency if external calls; must not fail core write. | **Best for 018 MVP** if PG-native updates (or best-effort enqueue) used. |
| **B. Postgres outbox table + lightweight poller/inline drain** | Reliable delivery/retry, decouples external indexer/notification fanout. | Extra migration + worker semantics. | Good follow-up when Meilisearch/async delivery needed; can be minimal single-table. |
| **C. LISTEN/NOTIFY or PG triggers** | DB-native signaling/maintenance. | Payload limits, missed events, trigger complexity/debuggability. | Defer except narrow generated-`tsvector` maintenance. |
| **D. Valkey Streams/PGMQ/Kafka-style pipeline** | Scalable async, replay. | Extra infra/ops, heavy for Seed. | **Defer/drop for Seed** (contradicts debloat). |
| **E. Meilisearch as primary/default** | Better instant/typo UX. | Mandatory-ish ops/memory/sync; already de-risked but optional. | Keep **optional**; never default on 4 GB Seed. |

## 4. Prioritized keep / replace / defer

### Keep (build on, low-risk)
1. **Postgres as system of record** for messages/tasks/files/notifications; `organization_id` tenant scoping and existing auth/membership checks in routes.
2. **Read APIs already present:** `GET /api/v1/search` contract shape, `GET/POST /api/v1/notifications` (list/mark-read) — extend, don’t redesign.
3. **Optional infra capability flags:** `search.enabled`/degraded boot, messaging optionality; in-process realtime hub for message events.
4. **Deterministic, env-gated DB contract tests pattern** (PG18 disposable) used in 012–015/017; CI-safe checks.

### Replace / reshape (80/20, near-term)
1. **Search backend default:** add a **Postgres FTS path** (additive schema + repository/query service) as default when Meilisearch unavailable/disabled; treat Meilisearch as **secondary/optional accelerator**, not the only index.
2. **Indexing model:** from “external-engine-only, no producers” to **in-DB searchable representation maintained on mutation** (generated `tsvector` columns or small projection) for P0 entities (messages, tasks, files metadata/name).
3. **Notification production:** from “no producers / unused crate” to **explicit, minimal producer calls on core mutations** writing `notifications` rows (core repo/service) for a tiny set of high-value events; avoid `flora-notifications` Valkey stream coupling in 018.
4. **Wiring location:** prefer a thin **application/service-layer hook** invoked by route/service after successful persist (or inside same logical unit where feasible) with **best-effort, logged failures** for non-core side effects — smallest change vs broad refactor.

### Defer (explicit, post-018 or need-driven)
1. Cross-entity/global ranking tuning, fuzzy/semantic/vector search, highlighting/snippets sophistication, CJK-optimized tokenization, per-locale dictionaries tuning, file-content extraction/indexing.
2. Async workers, outbox drainers, LISTEN/NOTIFY fanout, multi-node/durable delivery, Valkey Streams/PGMQ/Redis pubsub delivery, email/push/digest, notification preferences/routing rules.
3. Mentions parsing, subscription/follow graphs, per-channel mute, read receipts as notifications, bulk backfill tooling beyond a simple SQL/scriptable path.
4. Unifying/removing duplicate crates (`flora-notifications`, `flora-search`, `flora-files`) beyond minimal touch needed for wiring — treat as later debloat once behavior is verified.
5. Performance/load/OOM benchmarks on 2vCPU/4GB, browser e2e/multi-client notification UX, OIDC/JWKS-adjacent identity concerns.

## 5. Proposed 018 slice — smallest verifiable wiring

**Goal:** deliver a **deterministic, Postgres-first, single-node** wiring slice that makes core mutations produce (a) searchable state and/or (b) in-app notification records, verifiable in disposable PG18, without adding mandatory services.

**Recommended slice (scoped): `018-S1 — PG FTS MVP + minimal in-app notifications for 1–2 core events`**

### In scope (implementation intent for cycle 018)
1. **Search (Postgres FTS MVP, additive):**
   - Additive migration: searchable columns/indexes for **messages** (required) and optionally **tasks/files name** if trivial (e.g. generated/stored `tsvector` + `GIN`, scoped by `org_id`; include needed btree/composite indexes for tenant+recency).
   - `search` read path: implement a **Postgres-backed query** used when Meilisearch is unavailable/disabled (or as explicit default backend) supporting `q`, `type` filter (message/task/file), `limit` cap, and existing response shape; preserve current 503 only when *no* backend can serve (or adjust contract minimally and test it).
   - **Indexing on mutation:** ensure new/edited/deleted messages become searchable/non-searchable via DB-native mechanism (generated column updates on row change) and/or explicit update in mutation path — no external worker.
2. **Notifications (in-app minimal):**
   - Use existing `notifications` table + core `PgNotificationRepository`/API.
   - Produce a notification on **one high-value, easy-to-target event**: e.g. `Reply` (message with `thread_id` to parent author, excluding self) and/or `TaskCompleted` or `Assignment` on task status/assignee change — pick **one** for minimal verifiability, document the contract.
   - Creation path: call core notification repo/service from the relevant route/service after successful mutation; **best-effort/logged** (must not break the primary mutation response except where atomicity is explicitly chosen and tested).
3. **Verification (deterministic):**
   - DB-gated contract/integration tests on disposable PG18 for: (a) search returns inserted/updated message and respects tenant/type/empty-query behavior per chosen contract; (b) mutation creates expected notification row and list/mark-read remain correct.
   - Rust fmt/check/tests for touched crates + existing CI matrix green.

### Acceptance (evidence-oriented)
- `cargo` checks/tests for affected crates pass locally; DB-gated tests pass with `TEST_DATABASE_URL` (PG18).
- API-level or service-level proof that: message mutation → searchable via PG path (no Meilisearch); selected mutation → notification visible via `GET /api/v1/notifications`.
- No new mandatory runtime dependency; Meilisearch/Valkey remain optional.

### Out of scope for 018-S1
- Meilisearch indexing hooks/backfill, outbox workers, push/realtime delivery of notifications, mention parsing, preferences, multi-entity ranking polish, file-content indexing, frontend notification UI beyond existing contracts, large refactors of service layering.

## 6. Risks and mitigations

| Risk | Mitigation |
|---|---|
| FTS migration/query performance on 4 GB (GIN build, `work_mem`) | Additive, scoped indexes; cap limits; prefer generated columns; defer heavy backfill/bench (**NV**). |
| Search contract change when Meilisearch disabled | Define explicit precedence (PG fallback vs 503) and lock with tests; keep response schema stable. |
| Side-effect failures break core mutations | Best-effort + structured logging for non-critical hooks; transactional only where necessary and tested. |
| Notification noise/duplication semantics unclear | Limit to 1 event type with clear recipient/trigger rules and idempotency guard if obvious (e.g. same target/type/user). |
| Tenant leakage in search results | Mandatory `organization_id` predicates/membership checks consistent with routes; contract tests include cross-org negative case. |
| Dual abstractions (`flora-notifications` vs core) cause confusion | 018 uses core repo/API path only; document crate status as follow-up cleanup. |

## 7. NVs (needs verification)

- **NV-1 (runtime):** p95 latency/memory/IO impact of PG FTS indexes and queries on 2 vCPU/4 GB with realistic data volumes.
- **NV-2 (relevance/i18n):** ranking/snippet quality and tokenizer behavior for **pt-BR, es, de, fr, zh-CN, ja** content (CJK/trigram needs especially uncertain).
- **NV-3 (contract precedence):** best product/API contract when both backends absent vs PG-only fallback (client expectations in `apps/frontend`/legacy clients).
- **NV-4 (scope of entities):** whether tasks/files should be included in 018 FTS/notification wiring or staged after messages-only proof.
- **NV-5 (atomicity):** whether notification creation must be in-DB-transactional with the source mutation (requires refactor/tx plumbing) or acceptable as best-effort post-commit.
- **NV-6 (backfill):** strategy for making pre-018 rows searchable (migration-time generated columns vs batch) and for historical notifications (none expected).
- **NV-7 (unused crates):** safe convergence path for `flora-notifications`/`flora-search` APIs vs core services without breaking feature boundaries.

## 8. Sources

**Primary / docs**
- PostgreSQL: Full Text Search — https://www.postgresql.org/docs/current/textsearch.html
- PostgreSQL: Text search indexes (GIN/GiST) — https://www.postgresql.org/docs/current/textsearch-indexes.html
- PostgreSQL: `pg_trgm` — https://www.postgresql.org/docs/current/pgtrgm.html
- Microservices.io: Transactional outbox — https://microservices.io/patterns/data/transactional-outbox.html
- Meilisearch comparison (Postgres context) — https://www.meilisearch.com/docs/resources/comparisons/postgresql

**Flora internal (cross-check)**
- `OBJETIVO.md`, `ROADMAP.md`
- `.specify/assessments/016-next-vertical-slice/decision.md`, `.specify/assessments/017-next-vertical-slice/decision.md`
- `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`, `docs/research/2026-11-16-016-operability-codebase-audit.md`, `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-15-macro-alternative-80-20-research.md`
- Code/schema: `flora-api/src/routes/{search.rs,notifications.rs,channels.rs,tasks.rs,files.rs}`, `flora-api/src/state/app_state.rs`, `flora-search/src/lib.rs`, `flora-notifications/src/lib.rs`, `flora-core/src/{config/search.rs,config/messaging.rs,migrations/sql/*.sql,models/notification.rs,repositories/notification_repository.rs,services/{message,task,file,notification}_service.rs,traits/repository.rs}`

**Multilingual note:** EN/pt-BR/es/de/fr/zh-CN/ja evidence synthesized from primary docs plus language-targeted queries/practitioner signals; no locale-exclusive authoritative benchmark found for Flora’s exact workload (**NV** where extrapolated).
