# 023 — Transactional outbox-lite as a durable bridge from committed Postgres to an optional derived search index (Flora, VPS 80/20, multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` (read-only research; this document only)  
**Requested output:** `/home/luis/development/MDMV/projetos/Arquivados/flora/docs/research/2026-11-16-023-outbox-lite-indexing-pg.md`  
**Scope:** Deep-dive on **transactional outbox-lite** for Flora: outbox table design, atomic intent write with the business transaction, polling workers using `FOR UPDATE … SKIP LOCKED`, at-least-once delivery and idempotent consumers, retries/backoff/dead-letter, ordering/latest-wins, idempotency keys, and operational cost on ~2 vCPU / 4 GB, no Kafka. Compare with inline best-effort hooks and in-process async channels. Ground in primary sources (PostgreSQL 18 docs, pattern authors, AWS/Microsoft engineering guidance) with Rust/sqlx specifics. Cross-check, do not duplicate, `2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md`.  
**Method:** Read-only code/schema/migration/research cross-check in this repo + primary-source web fetches (2026-11-16). Multilingual lens: EN, pt-BR, es, de, fr, zh-CN, ja+ (localized primary pages fetched for EN/pt-BR/es/ja; de/fr/zh-CN marked **NV**). No code changes, no benchmarks, no external services exercised.

---

## 0. Relationship to 018 / 022 (explicit non-duplication)

| Doc | What it established | What this doc adds |
|---|---|---|
| `018-postgres-search-outbox-vps.md` | Proposed outbox schema + `SKIP LOCKED` poller + PG FTS as default; 80/20 VPS tuning. | Source-level mechanics, correctness arguments, and the Rust/sqlx feasibility cost that 018 did not analyze. |
| `022-meilisearch-pg-coherence-degradation-contract-vps.md` | PG-authoritative fallback, degradation contract, idempotent upsert/delete doc mapping, rebuild-from-Postgres. **R3 (outbox-lite) was explicitly deferred** until durable replay is required. | Full R3 deep-dive: claim protocol, delivery semantics, backoff/DLQ, ordering, idempotency keys, worker sketch, and a sharper "worth it vs defer" decision. |

**Bottom line up front:** outbox-lite is the right *durability* mechanism, but it does not solve *coherence* (freshness/precedence/rebuild) — 022 remains the coherence contract. Do not build R3 before R1/R2/R4 of 022 unless a concrete requirement for "no drift across crash/restart/deploy" exists (see §6).

---

## 1. Flora baseline cross-check (verified in this session)

| Area | Evidence (repo) | Current reality | Outbox-lite relevance |
|---|---|---|---|
| Outbox/job table | `grep -rniE 'outbox\|skip locked\|next_attempt\|dead_letter' --include='*.rs' --include='*.sql'` over `flora-core`, `flora-api`, `flora-notifications`, `flora-search`, `flora-messaging` → **no matches** | No outbox, no job table, no claim query, no DLQ. | Greenfield; no legacy queue to migrate. |
| Notification dedup (022) | `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql:4-6` — `CREATE UNIQUE INDEX … ON notifications (user_id, event_type, target_id) WHERE is_read = false`; `flora-core/src/repositories/notification_repository.rs:83-99` — `INSERT … ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING RETURNING …`, returns `Option<Notification>` | Duplicate unread notifications are **DB-enforced idempotent**; concurrent producers cannot create duplicates. | Search side effects are **not** similarly protected; the outbox is primarily a *search* concern. Notification intents could share it later, but gain little today. |
| Transaction boundaries | `flora-core/src/services/task_service.rs:29-70` uses `Arc<dyn TaskRepository + Send + Sync>`; all repositories hold `PgPool` (e.g. `flora-core/src/repositories/task_repository.rs:4-20`); `grep 'Transaction<\|begin()\|commit()'` over services/repos/routes → **no matches** | Writes are autocommit per repository call; services do not propagate `sqlx::Transaction`. | **Hardest feasibility cost:** atomic intent write needs transaction plumbing (§4.1). |
| Best-effort side effects | `flora-api/src/routes/tasks.rs:192-212`: `task_repo.create(task).await?` then `maybe_notify_task_assignment(...)` with `tracing::warn!` on error; same pattern in `channels.rs:362-378,419-428` | DB write and notification are separate statements; failure after commit loses the side effect permanently (until rebuild); no retry across restarts. | This is exactly the "inline best-effort" baseline outbox-lite replaces (§3). |
| Search indexing hooks | `grep` for `index_message\|index_task\|index_file\|delete_document` in app routes → no call sites (consistent with 022 [F4]) | No Meilisearch mutation wiring; PG FTS generated columns + partial GIN are transactional (`20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`). | An outbox can only deliver *index intents*; it cannot make the derived index fresh unless a consumer exists. |
| Search runtime | `flora-core/src/config/search.rs`, `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`; 022 §1 | Optional Meilisearch; init failure → `search_service: None` → PG fallback; when `Some`, Meilisearch is trusted as-is (no freshness gate). | Outbox does **not** fix "configured but stale"; that is 022 R1/R5. |

**Determinism gaps specific to outbox-lite:** (1) no transactional boundary to attach the intent write to; (2) no idempotency marker for search writes (notification has one); (3) no durable replay after process crash; (4) no version/sequence on outbox events to defend against out-of-order application; (5) no metrics for depth/oldest age/dead letters.

---

## 2. Primary-source mechanics

### 2.1 The pattern: intent in the same transaction [S5]

The transactional outbox pattern exists because a service must atomically change business state and record an intent to publish, while 2PC across DB and broker is unavailable or undesirable. Sending "in the middle of a transaction" is unreliable (tx may roll back); sending "after commit" can lose the message if the process crashes before sending. The solution is to store the message in the **same database transaction** that updates the business entity, then let a separate **message relay** send it afterwards [S5].

Consequences that matter here [S5]:
- **"Send if and only if commit"** for the recorded intent — the core guarantee.
- **Ordering intent** — messages must be published in the order the application sent them (across instances updating the same aggregate). This is a *design force*, not automatically a runtime property of every relay.
- **Relay may publish more than once** (crash after publishing, before recording the fact). Consumers must be idempotent, typically by tracking message IDs [S5].

Two relay implementations are named in the canonical pattern: **transaction log tailing** and **polling publisher** [S5]. Flora's 80/20 choice is the polling publisher over a Postgres table (no CDC/Debezium/Kafka) — consistent with 018/022 and with the single-node VPS constraint.

### 2.2 Claiming rows: `FOR UPDATE … SKIP LOCKED` [S1, S2]

PostgreSQL's locking clause is explicit [S1]:

> "With `SKIP LOCKED`, any selected rows that cannot be immediately locked are skipped. Skipping locked rows provides an inconsistent view of the data, so this is not suitable for general purpose work, but can be used to avoid lock contention with multiple consumers accessing a queue-like table."

Also from [S1]: `NOWAIT` errors instead of waiting; `SKIP LOCKED` applies only to row-level locks (a `ROW SHARE` table lock is still taken); `LIMIT` stops once enough rows are returned; and "it is a good idea to use an `ORDER BY` clause that constrains the result rows into a unique order" when using `LIMIT`, otherwise the subset is unpredictable.

From the explicit-locking chapter [S2]: row-level locks are released at transaction end (or savepoint rollback); row-level locks "do not affect data querying; they block only writers and lockers to the same row"; `SELECT FOR UPDATE` can cause a disk write to mark rows locked; and "it is a bad idea for applications to hold transactions open for long periods of time." Advisory locks are an alternative for application-defined keys: "faster, avoid table bloat, and are automatically cleaned up by the server at the end of the session" — transaction-level advisory locks release at tx end; session-level locks survive rollback [S2].

**Design consequences for Flora:**
- Claim in a **short transaction**: `BEGIN; SELECT … FOR UPDATE SKIP LOCKED LIMIT n; UPDATE … status='processing'; COMMIT;` — or, better, a single `WITH claimed AS (SELECT … FOR UPDATE SKIP LOCKED) UPDATE … FROM claimed … RETURNING …` statement so lock, status flip, and payload return are atomic.
- **Never hold the claim transaction open across the Meilisearch HTTP call** — a foreign call cannot join the DB transaction [S8], and a long transaction holds row locks and a pool connection [S2].
- Use `ORDER BY id` (or `(next_attempt_at, id)`) with `LIMIT` for a deterministic claim subset [S1].
- A `processing` lease/visibility timeout is required: a worker that claims and then dies must have its row requeued. (Brokers call this visibility timeout; [S2] explains that row locks die with the transaction, but a *status flag* persists, so the flag needs a lease.)
- Advisory locks are useful later for "only one reindex/replay job at a time" (transaction-level), not needed for the basic claim path [S2].

### 2.3 At-least-once and idempotent consumers [S5, S6, S7]

The canonical pattern states the relay can publish more than once [S5]. Microsoft's Idempotent Consumer pattern [S7] is more precise: at-least-once delivery is the default for most brokers, exactly-once cannot cover side effects in external systems, and "when you combine at-least-once delivery with a consumer that ignores duplicates, you achieve *effectively-once* processing." Duplicate sources include producer retries, missing acknowledgements, and consumer crashes after a DB write but before acknowledgement [S7].

Recommended consumer discipline [S6, S7]:
- Record processed IDs in a `PROCESSED_MESSAGES`-style table (or on the business entity); `(subscriber_id, message_id)` primary key makes the conflicting `INSERT` fail and the duplicate is discarded [S6].
- Write the dedup marker **and the side effect in the same transaction** (the consumer-side *Inbox* pattern) when both are local [S7]. For a side effect that cannot join the transaction (Meilisearch), use a two-phase record: `in_progress` → perform external action → `completed` + outcome; on redelivery, `completed` means skip, `in_progress` means reconcile or escalate [S7].
- **Prefer naturally idempotent operations** — upsert by business identifier, absolute-value writes, `PUT`-style semantics — "use deduplication techniques only for operations that can't be made naturally idempotent" [S7].
- Enforce dedup with a **unique constraint**; the database is the single arbiter of concurrent duplicates, not a check-then-set in application code [S7].
- Manage the lifecycle of dedup records with a TTL longer than the maximum redelivery window, including manual replays from the dead-letter store [S7].

**Flora mapping:** Meilisearch `add_documents(..., Some("id"))` / `delete_document` are already naturally idempotent by primary key (022 §6.1–6.2). Therefore the consumer usually needs **no separate processed-ID table** for search: replay is harmless. The dedup bookkeeping that is needed is the *outbox row state* itself (`done`/`dead`), plus a version guard for ordering (§2.5). Notification side effects already have a DB-enforced dedup marker (§1), so they do not need an Inbox table either.

### 2.4 Retries, backoff, jitter, dead letters [S9, S10, S11]

AWS's Builders' Library [S9] gives the canonical rules:
- Set timeouts on every remote call (connection **and** request); choose the timeout from downstream latency percentiles at an acceptable false-timeout rate.
- Retry transient/partial failures; use **backoff** (increase wait between attempts), typically **capped exponential backoff**; cap retries and stop earlier because "retries are selfish."
- **Jitter** is required to break correlation: without it, all failed clients back off to the same instant and re-create the overload.
- "APIs with side effects aren't safe to retry unless they provide idempotency"; retry at a **single layer** (multiplied retries can increase load 243× in a five-deep stack).

AWS Well-Architected REL05-BP03 [S10] makes this a best practice: exponential backoff **with jitter** and a maximum retry count; anti-patterns include retrying all errors, retrying non-idempotent calls, retrying at multiple layers, and lacking observability/alerting on repeated failures.

Dead letters: SQS DLQs isolate messages that exceed `maxReceiveCount` so they can be inspected, diagnosed, and redriven; set the threshold high enough for transient outages, alarm on DLQ growth, and set retention longer than the source queue [S11]. AWS also warns that moving messages out of order breaks FIFO semantics [S11] — relevant to the ordering discussion below.

**Flora worker policy (concrete):** attempts cap ~5–8; `next_attempt_at = now() + full_jitter(min(cap, base * 2^(attempts-1)))` with `base ≈ 1s`, `cap ≈ 5–10 min`; retry only transient failures (Meilisearch unavailable/timeout/5xx), not permanent ones (malformed payload, unsupported entity) — permanent failures go straight to `dead`; `dead` is a status in the same table (a logical DLQ), with `last_error` preserved and an alert; replay is a rate-limited `UPDATE … SET status='pending'` runbook, and must be idempotent because a dead row may have been partially applied [S7, S11].

### 2.5 Ordering and latest-wins [S4, S5, S7]

The outbox pattern *intends* to preserve application order [S5], but a polling relay with multiple workers applying side effects outside the DB cannot guarantee apply order: workers claim different rows, Meilisearch indexing is asynchronous (022 §2.1), and retries reorder completion. Microsoft's guidance is explicit: deduplication does not guarantee order; "include sequence or version data so the consumer can reject stale messages" [S7].

Sequence facts from PostgreSQL [S4]: `nextval` is atomic and gives distinct values even with concurrent sessions, **but** values are not reclaimed on abort, `ON CONFLICT` computes `nextval` before detecting the conflict, and sequence state is not rolled back — "PostgreSQL sequence objects cannot be used to obtain 'gapless' sequences." So a `BIGINT` identity/sequence is a fine *monotonic ordering proxy* (useful for claim order and stale detection) but **not** a gapless transaction counter.

Two viable Flora strategies:
1. **Coalesced latest-wins (recommended for search):** upsert the outbox intent with `ON CONFLICT (organization_id, aggregate_type, aggregate_id) DO UPDATE SET op=EXCLUDED.op, entity_version=GREATEST(...), payload=EXCLUDED.payload, next_attempt_at=now()`. Only the latest intent per entity remains; stale intermediate states are irrelevant for a search projection. This mirrors 022 §6.3 option 1 and is naturally idempotent [S7].
2. **Append-only + version guard:** keep every event; include `entity_version`/`updated_at`; the consumer stores the last applied version per `(org, type, id)` and skips older events. Better audit, more storage, needs the version check in the consumer [S7].

Either way, carry **absolute state** (`op='upsert'` plus the full current projection fields), not relative deltas — event-carried state transfer makes the operation idempotent and avoids stale-overwrite bugs [S7]. For Flora: the payload should be the current document fields (id, org, type, title/description/name/file_type, `is_deleted`→delete), not "append to description".

If strict per-entity ordering is ever required without version guards, the options are a single worker (order = claim order, still not commit order under concurrency), partitioning by `org_id`/entity with one worker per partition, or a broker with sessions. All are deferred for the VPS 80/20 case [S5, S7, S11].

### 2.6 Idempotency keys [S5, S7, S8]

Separate two concepts often conflated:

- **Relay/event identity (internal):** a stable `event_id` (UUID) on every outbox row. The relay uses it as the dedup key; the consumer's "message ID" in [S6, S7]. Unique index makes re-delivery detectable.
- **Client-facing `Idempotency-Key` (external):** only needed if *public mutating endpoints* must be safely retryable by clients. Brandur Leach's Postgres implementation [S8] is the authoritative reference: unique `(user_id, idempotency_key)`, `request_params` stored to reject key reuse with different payloads (409), `locked_at` soft lock with timeout, `recovery_point` state machine, and **atomic phases** — local ACID mutations committed as transactions *between* foreign state mutations; a staged-jobs table is committed with the business tx and an enqueuer moves jobs to the queue only after commit [S8]. A reaper deletes keys after a horizon (~72 h in the article) and a completer pushes abandoned requests through [S8].

Brandur's most relevant caveat [S8]: if an endpoint only mutates local ACID state, mapping requests to transactions is "far easier and less complicated" than full idempotency keys. **Flora today does not need client-facing idempotency keys for outbox-lite** — mutation requests already commit locally; the internal `event_id` + upsert-by-id is sufficient. Revisit only if a public API explicitly promises safe client retries (NV).

`NOTIFY` is **not** an idempotency or durability mechanism [S3]: notifications are delivered only if the transaction commits (good), identical payloads in one transaction are folded, ordering across transactions follows commit order, and the payload is limited (~8000 bytes) — the docs recommend putting structured data in a table and sending its key. The notification queue is finite and a full queue makes `NOTIFY` fail at commit. Use `NOTIFY` only as a low-latency "wake up and poll" hint; the outbox row remains the source of truth. This is safe because a missed hint only delays a poll.

### 2.7 Operational cost on ~2 vCPU / 4 GB [S1, S2, S3, S9, S10]

- **No new daemon/port/process**: one table + one in-process poller in `flora-api` (or a thin worker binary later). Consistent with 018/022 and the "no Kafka" constraint.
- **Write amplification**: one extra small `INSERT` (or upsert) per indexed mutation; row versions accumulate until autovacuum. Keep payloads small (IDs + fields), use a partial claim index rather than indexing the whole table, and add a retention/reaper job for `done`/`dead` rows [S2, S7].
- **Claim cost**: one short indexed scan with `ORDER BY … LIMIT … FOR UPDATE SKIP LOCKED`; partial index on pending/due rows keeps it bounded [S1, S2]. Batch 10–50, worker concurrency 1 on this VPS, adaptive poll sleep with jitter (no hot polling) [S9, 018/022].
- **Connection budget**: claim tx and completion tx each briefly use one pooled connection; do not hold a connection during the Meilisearch call [S2, S8].
- **Observability minimum**: outbox depth, oldest-pending age, `dead` count, attempts distribution, claim/apply latency, Meilisearch failure count, duplicate-skips; alert on sustained lag and any `dead` > 0 [S10, S11, 022 R5].
- **Backup/DR**: the outbox is *source-side* state and must be in DB backups; the Meilisearch index remains a disposable projection with a rebuild runbook (022 §7–8). Replaying outbox rows after a DB restore can legitimately re-apply idempotent side effects [S5, S7].
- **Explicitly deferred**: PGMQ/other queue extensions, Debezium/CDC, Kafka, exactly-once delivery, cross-datacentre replication, per-tenant workers. These add infra or complexity with no 80/20 payoff at this scale [S5, 018/022].

---

## 3. Options compared: inline hooks vs in-process channel vs outbox-lite

| Dimension | A. Inline best-effort hook (today's notification pattern) | B. In-process async channel (`tokio::mpsc` / spawned task) | C. Outbox-lite (Postgres table + poller) |
|---|---|---|---|
| Commit↔intent atomicity | None; side effect can be lost after commit or fire before rollback is known | None; enqueue happens post-commit, in memory | **Yes** — intent commits with the business tx [S5] |
| Crash / restart durability | Lost permanently until rebuild | Lost permanently until rebuild | **Durable**; replay after restart [S5] |
| Duplicate side effects on retry | Possible; no dedup for search | Possible; channel has no dedup | Possible (at-least-once) but **safe** with upsert-by-id + event_id [S5, S7] |
| Backoff across restarts | None | In-memory only | Persisted in `next_attempt_at`/`attempts` [S9] |
| Ordering | Request order, not guaranteed after failure | FIFO within process only | Claim order; latest-wins/version guard needed for correctness [S4, S5, S7] |
| Observability / replay | Logs only | Logs only | Depth, oldest age, dead letters, deterministic replay [S10, S11] |
| Request latency | High if awaited; fail-open if swallowed | Low | Low |
| Code delta | Smallest | Small (no schema) | Migration + tx plumbing + worker (largest) |
| Deterministic testability | Medium | Medium–low | **High** — drain queue and assert state in DB-gated tests |
| VPS 2 vCPU/4 GB fit | OK but silently degrades | OK but silently degrades | **Good** if bounded: 1 worker, small batches, partial index |

**Verdict:** A and B are acceptable only as *best-effort, fail-open* conveniences with PG as the deterministic fallback (022 §4 Options A/B). C is the only option that makes "no silent drift across crash/restart/deploy" true. It does **not** replace 022 R1 (precedence/fallback), R2 (idempotent hooks + delete-on-soft-delete), or R4 (rebuild). In fact C *depends* on R2's deterministic document builder and R1's degradation policy to be useful.

---

## 4. Feasibility in Flora: Rust/sqlx sketch

### 4.1 The transaction-plumbing problem (most important NV)

`sqlx::Transaction<'c, DB>` starts at `Pool::begin()`/`Connection::begin()`, ends at `commit()`/`rollback()` (and rolls back on drop if still open), and can be used as an `Executor` via `&mut *tx` [S12]. That is exactly what atomic intent needs:

```rust
// Sketch only — not committed code.
let mut tx = state.db_pool.begin().await?;
domain_repository.create_in(&mut tx, entity).await?;   // future helper
outbox_repository.enqueue_in(&mut tx, intent).await?;  // future helper
tx.commit().await?;                                    // intent exists iff entity exists
```

Today this is **not** a drop-in change: repositories are pool-bound and services hold `Arc<dyn Repository>` trait objects (§1), with no transaction parameter in any trait method. Three options, increasing invasiveness:

1. **Mutation helpers with `&mut PgConnection`** (recommended minimal): keep existing pool-bound read repos; add a small `mutations` module with free functions taking `&mut PgConnection` (or `&mut Transaction<'_, Postgres>`) for the handful of write paths that need an outbox row. No trait churn.
2. **Executor-generic repo methods**: change trait signatures to accept `impl Executor<'_, Database = Postgres> + Copy`. More uniform, but trait objects (`dyn`) cannot express generic methods — would require replacing the trait objects in services with generics or enums. Larger refactor.
3. **Unit-of-work struct** owning the `Transaction` and exposing both domain and outbox writes. Cleanest long-term, biggest delta.

Other sqlx specifics:
- Keep the claim tx and completion tx separate from the Meilisearch call; the foreign call cannot join the transaction [S8] and long transactions are harmful [S2].
- Reuse the already-proven idempotency SQL shape from `notification_repository.rs:83-99` (`ON CONFLICT … DO NOTHING RETURNING`, returns `Option`) as the template for both coalesced enqueue and "already done" checks.
- New queries can be runtime `sqlx::query_as` strings (matching current repo style); compile-time checked queries would need an offline cache (NV).
- `Transaction` must stay inside one async task (single connection); spawn the worker as a single `tokio` task with a `PgPool` clone and an `Arc<dyn SearchBackend>`-like handle; use `tokio::select!` with a shutdown signal for graceful stop, and adaptive `sleep` with jitter when a poll returns no rows [S9].

### 4.2 Minimal schema sketch (additive migration, not applied)

```sql
-- 2026-11-16-023_add_search_outbox.sql (sketch)
CREATE TABLE search_outbox (
    id               BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, -- ordering proxy; gappy [S4]
    event_id         UUID        NOT NULL UNIQUE,                     -- stable relay/dedup key [S5,S7]
    organization_id  UUID        NOT NULL,
    aggregate_type   TEXT        NOT NULL,   -- 'message' | 'task' | 'file'
    aggregate_id     UUID        NOT NULL,
    op               TEXT        NOT NULL,   -- 'upsert' | 'delete'
    entity_version   BIGINT      NOT NULL DEFAULT 0,                 -- stale guard [S7]
    payload          JSONB       NOT NULL DEFAULT '{}'::jsonb,       -- absolute projection, small
    status           TEXT        NOT NULL DEFAULT 'pending',         -- pending|processing|done|dead
    attempts         INT         NOT NULL DEFAULT 0,
    next_attempt_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    locked_at        TIMESTAMPTZ,                                    -- lease for 'processing'
    last_error       TEXT,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at     TIMESTAMPTZ
);

-- Coalescing variant (recommended): one latest intent per entity.
INSERT ... ON CONFLICT (organization_id, aggregate_type, aggregate_id) DO UPDATE ...
-- requires:
CREATE UNIQUE INDEX IF NOT EXISTS idx_search_outbox_latest_per_entity
    ON search_outbox (organization_id, aggregate_type, aggregate_id);

-- Bounded claim index (partial), not a full-table index:
CREATE INDEX IF NOT EXISTS idx_search_outbox_due
    ON search_outbox (next_attempt_at, id)
    WHERE status IN ('pending', 'processing');
```

If the append-only variant is chosen instead, drop the unique-per-entity index and add a small `search_index_state (organization_id, aggregate_type, aggregate_id, applied_version)` table with a unique key for the consumer's stale check [S6, S7].

### 4.3 Producer + worker loop (sketch)

**Producer** — in the same transaction as the domain write (§4.1): insert/upsert `search_outbox` with `event_id = Uuid::now_v7()`, `op`, `entity_version` (entity `updated_at` epoch or a per-entity monotonic counter), and the **absolute** projection. Soft-delete maps to `op='delete'`; update/unassign maps to `op='upsert'` (022 §6.2). Commit; the rest is asynchronous.

**Worker** (one task):
1. **Requeue expired leases**: `UPDATE search_outbox SET status='pending', next_attempt_at=now() WHERE status='processing' AND locked_at < now() - lease` — bounded by the partial index.
2. **Claim** (short tx, one statement):

   ```sql
   WITH claimed AS (
       SELECT id FROM search_outbox
       WHERE status='pending' AND next_attempt_at <= now()
       ORDER BY id
       LIMIT $1            -- 10–50
       FOR UPDATE SKIP LOCKED
   )
   UPDATE search_outbox o
      SET status='processing', attempts=o.attempts+1, locked_at=now()
     FROM claimed c
    WHERE o.id=c.id
   RETURNING o.*;
   ```
   [S1, S2]

3. **Commit claim tx**. Apply each event **outside** any DB transaction: `upsert` = `add_documents(id, current projection)`; `delete` = `delete_document(id)`, treating not-found as success (022 §6.1–6.2). Skip stale events if `entity_version` is behind the applied version (§2.5). Measure call latency; enforce a timeout on the Meilisearch call [S9].
4. **Complete** in another short tx: on success `status='done', processed_at=now()`; on transient failure `status='pending', next_attempt_at = now() + jittered_backoff(attempts)`; on permanent failure or `attempts >= max` → `status='dead', last_error=…`, emit alert [S9, S10, S11].
5. If no rows claimed, sleep with jitter (e.g. 200–800 ms) before repolling; optional `NOTIFY` from a trigger on insert wakes the poller early but is never required for correctness [S3].

**Verification (DB-gated / contract tests; exact harness wiring NV):**
- domain rollback also removes the outbox row (atomicity);
- two concurrent claims never return the same `id` (`SKIP LOCKED`);
- replaying a `done` event twice leaves the index unchanged (idempotent apply);
- an older `entity_version` is skipped (latest-wins);
- `attempts` cap moves the row to `dead` and a rate-limited replay reprocesses it;
- simulated crash after apply but before `done` → redelivery is harmless.

---

## 5. Multilingual considerations (EN, pt-BR, es, de, fr, zh-CN, ja+)

- **The outbox itself is locale-neutral.** IDs, org scoping, `op`, `entity_version`, and JSONB payload shape do not depend on language. No per-locale queues or tables.
- **What is locale-sensitive is the consumer/derived index**, not the queue: tokenization, diacritics, and CJK/JP segmentation (022 §3). Therefore carry **absolute, fully rendered projection fields** in the payload (including `name`, `file_type`, title/description) rather than language-specific deltas or "recompute from engine state later" — event-carried state transfer is both the idempotency and the locale-correctness strategy [S7]. Ranking/snippet parity stays NG/parity scope for 022; do not promise equal recall across engines.
- **Localized primary sources checked (high reliability):** AWS Builders' Library pt-BR uses *"tempos limite, novas tentativas e recuo com jitter"* and explains idempotent APIs as a prerequisite for safe retries [S13]; Microsoft Learn pt-BR *"Padrão Consumidor Idempotente"* uses *"entrega pelo menos uma vez"* and *"caixa de entrada"* (Inbox) [S14]; es-es *"Patrón de consumidor idempotente"* [S15]; ja-jp *"べき等コンシューマー パターン"* / *"少なくとも 1 回の配信"* [S16]. EN originals remain normative: the localized Microsoft pages carry `ms.translationtype: MT` metadata (machine translation), so cite EN text for exact contract wording and localized pages for terminology/ops-runbook translation.
- **de / fr / zh-CN:** localized pattern pages are expected to exist but their exact current text was **not fetched** this session (**NV**). Do not quote them as verified.
- **Ops docs/runbooks**: if Flora's operator audience uses pt-BR/es/de/fr/zh-CN/ja, translate the outbox runbook (claim/backoff/DLQ/replay terms) but keep SQL identifiers and metric names in English to avoid drift.

---

## 6. 80/20 decision guidance — when outbox is worth it vs defer

**Build outbox-lite when at least one of these is true:**
1. Meilisearch is enabled for a deployment **and** the product promises that committed mutations are not silently lost from the index across restarts/deploys/crashes.
2. Search (or another derived read model) becomes **authoritative** for some reads, making drift a correctness bug rather than a UX nuisance.
3. There are multiple mutating paths (routes + background tasks + imports) and you need one durable, replayable intent stream instead of auditing every hook.
4. Rebuild-from-Postgres is expensive enough that you need incremental catch-up after an outage.
5. Acceptance/contract tests require "kill worker after index update, restart, index converges without manual rebuild."

**Defer (keep 022 R1/R2/R4 only) when:**
1. PG FTS is the authoritative default (Seed/lean VPS) and Meilisearch is optional/disabled — the current baseline (022 §8).
2. Mutation hooks and delete-on-soft-delete do not exist yet; an outbox would queue intents that nothing reliably consumes.
3. Write volume is low and a full rebuild from Postgres is cheap.
4. No requirement for cross-restart durability; fail-open hooks with a documented "PG authoritative" contract are acceptable.
5. Team capacity is better spent on 022 R1 (deterministic precedence/fallback) and R2 (idempotent hooks) — outbox amplifies any bug in those.

**Staged 80/20 order (unchanged from 022 §9, with this doc as the R3 detail):**
`R1 contract+tests → R2 idempotent hooks + delete-on-soft-delete + fail-open → R4 rebuild tool → R3 (this doc) outbox-lite → R5 metrics/health`.

**Never (in this phase):** Kafka/Debezium/PGMQ, exactly-once distributed delivery, `LISTEN/NOTIFY` as correctness, dual-write without idempotency, blocking domain commits on Meilisearch, unbounded polling/retries, or cross-engine merged results without a normalized contract.

---

## 7. Risks and open verification items (NV)

| Risk | Why it matters | Mitigation / NV |
|---|---|---|
| Transaction plumbing refactor (§4.1) | Pool-bound repos + trait objects cannot atomically write domain + intent today | Start with `&mut PgConnection` mutation helpers; NV: exact trait/service churn per affected route |
| Long transaction / pool starvation | Holding tx during Meilisearch call holds row locks and a connection [S2, S8] | Two short txns + foreign call between; bound pool; NV: measured latency under load |
| "Ordering" illusion | Pattern promises intent order, not apply order; multiple workers + async index reorder [S5, S7] | Coalesced latest-wins or version guard; absolute state payload; never rely on claim order |
| Sequence gaps | `nextval` non-transactional, `ON CONFLICT` consumes values, aborts create gaps [S4] | Treat IDs as monotonic proxies only; use `entity_version` for correctness |
| Duplicate/partial apply | Relay crashes after apply before `done` [S5] | Upsert-by-id + delete-not-found-as-success + event_id; NV: verify Meilisearch task outcome handling (022 §10) |
| Outbox bloat / vacuum | Hot queue table accumulates dead tuples; partial index helps but reaping needed [S2, S7] | TTL/reaper for `done`/`dead`, small payloads, partial index; NV: measured autovacuum behavior on target VPS |
| Silent DLQ growth | Dead letters convert loud failures into quiet ones [S11] | Alarm on `dead > 0`, preserve `last_error`, tested rate-limited replay |
| Claim starvation / hot polling | Bad `ORDER BY`/index or fixed 0 sleep wastes 2 vCPU | `ORDER BY id`, partial index, jittered adaptive sleep [S1, S9] |
| Dedup/retention too short | Late replay reintroduces duplicates [S7] | Keep markers longer than max retry/replay window (brandur ~72 h horizon [S8]) |
| Multilingual parity false confidence | Outbox is neutral; index quality is not (esp. zh-CN/ja) | Mark NV; carry full absolute fields; keep 022 parity contract and corpus validation |
| No benchmarks/measurements | All VPS sizing here is NV | Measure depth/latency/RAM with real corpus before enabling Meilisearch profile |
| Meilisearch async task outcomes ignored by adapter | Upsert/delete may be accepted then fail in engine (022 §2.1/§10) | Capture task status in worker; dead-letter on failure; NV |

---

## 8. Sources and citations

### Primary / authoritative (fetched 2026-11-16 unless noted)

- **[S1]** PostgreSQL 18 Documentation — `SELECT` (The Locking Clause: `NOWAIT`/`SKIP LOCKED`, `LIMIT` + unique `ORDER BY`, subquery locking): https://www.postgresql.org/docs/current/sql-select.html (also anchored: https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE)
- **[S2]** PostgreSQL 18 Documentation — Explicit Locking (row locks released at tx end, row locks can cause disk writes, long transactions discouraged, advisory locks session vs transaction level): https://www.postgresql.org/docs/current/explicit-locking.html
- **[S3]** PostgreSQL 18 Documentation — `NOTIFY` (delivery on commit, identical-payload folding, commit-order delivery, ~8000-byte payload, use a table for structured data, finite queue): https://www.postgresql.org/docs/current/sql-notify.html
- **[S4]** PostgreSQL 18 Documentation — Sequence Manipulation Functions (`nextval` atomic/distinct, gaps on abort and `ON CONFLICT`, not gapless): https://www.postgresql.org/docs/current/functions-sequence.html
- **[S5]** Chris Richardson — Pattern: Transactional outbox (2PC rejected, store message in same tx, relay, ordering force, relay may publish more than once, polling publisher vs log tailing): https://microservices.io/patterns/data/transactional-outbox.html
- **[S6]** Chris Richardson — Pattern: Idempotent Consumer (`PROCESSED_MESSAGES` table, `(subscriberId, messageID)` PK, discard duplicate): https://microservices.io/patterns/communication-style/idempotent-consumer.html
- **[S7]** Microsoft Azure Architecture Center — Idempotent Consumer pattern (at-least-once → effectively-once, stable dedup key, atomic marker+side effect/Inbox, unique constraint as arbiter, naturally idempotent operations, in-progress→completed for external calls, dedup TTL, ordering via sequence/version, observability): https://learn.microsoft.com/en-us/azure/architecture/patterns/idempotent-consumer
- **[S8]** Brandur Leach — Implementing Stripe-like Idempotency Keys in Postgres (unique `(user_id, idempotency_key)`, `locked_at`, atomic phases committed before foreign mutation, transactionally staged jobs + enqueuer, recovery points, reaper/completer, "local ACID only" simpler path): https://brandur.org/idempotency-keys (2017-10-27)
- **[S9]** Amazon Builders' Library (Marc Brooker) — Timeouts, retries, and backoff with jitter (timeouts, capped exponential backoff, retries are selfish, idempotency prerequisite, jitter to break correlation, single retry layer): https://builder.aws.com/content/3EumjoZascWd1oZiEgL8ORlv3qE/timeouts-retries-and-backoff-with-jitter (canonical: https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/)
- **[S10]** AWS Well-Architected Framework — REL05-BP03 Control and limit retry calls (exponential backoff + jitter + max retries; anti-patterns): https://docs.aws.amazon.com/wellarchitected/latest/framework/rel_mitigate_interaction_failure_limit_retries.html
- **[S11]** AWS SQS Developer Guide — Using dead-letter queues (isolate failures, `maxReceiveCount`, inspection/redrive/alarms, retention, FIFO ordering warning): https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html
- **[S12]** docs.rs — `sqlx::Transaction` 0.9.0 (`Pool::begin`/`Connection::begin`, `commit`/`rollback`, rollback on drop, usable as `Executor` via `&mut *tx`): https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html
- **[S13]** AWS Builders' Library (pt-BR) — "Tempos limite, novas tentativas e recuo com jitter": https://aws.amazon.com/pt/builders-library/timeouts-retries-and-backoff-with-jitter
- **[S14]** Microsoft Learn (pt-BR) — "Padrão Consumidor Idempotente": https://learn.microsoft.com/pt-br/azure/architecture/patterns/idempotent-consumer
- **[S15]** Microsoft Learn (es-es) — "Patrón de consumidor idempotente": https://learn.microsoft.com/es-es/azure/architecture/patterns/idempotent-consumer
- **[S16]** Microsoft Learn (ja-jp) — "べき等コンシューマー パターン": https://learn.microsoft.com/ja-jp/azure/architecture/patterns/idempotent-consumer

### Cross-referenced (not re-fetched; no duplication intended)

- 022 — Meilisearch async tasks/eventual visibility and rebuild: [S1,S2] of `docs/research/2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md` (https://www.meilisearch.com/docs/learn/async/asynchronous_operations, https://www.meilisearch.com/docs/learn/getting_started/documents).
- 018 — `docs/research/2026-11-16-018-postgres-search-outbox-vps.md` (original outbox sketch + VPS tuning).
- 022 — `docs/research/2026-11-16-022-meilisearch-indexing-hooks-reindex-parity-vps.md` (hook/reindex parity).

### Repo evidence (Flora cross-check)

- **[F1]** `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql`; `flora-core/src/repositories/notification_repository.rs:83-99`
- **[F2]** `flora-core/src/services/task_service.rs:29-70`; `flora-core/src/repositories/task_repository.rs:4-20`; repo-wide grep (no `Transaction<`/`begin()` in services)
- **[F3]** `flora-api/src/routes/tasks.rs:192-212`; `flora-api/src/routes/channels.rs:362-378,419-428`
- **[F4]** `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`; `20261118000001_add_tasks_files_search_fts.sql`; `flora-search/src/lib.rs`; `flora-core/src/config/search.rs`; `flora-api/src/state/app_state.rs`; `flora-api/src/routes/search.rs`
- **[F5]** repo-wide grep: no outbox/job table, no `index_*` call sites in app routes

### Research limitations

- Web sources were fetched with `webfetch`; some pages are long and were read via saved tool output. Localized pages for de/fr/zh-CN were not fetched (**NV**); Microsoft localized pages are machine-translated (`ms.translationtype: MT`) and are cited for terminology, not normative wording.
- No runtime benchmarks, no code changes, no Postgres/Meilisearch instance exercised. All VPS sizing, bloat, and latency numbers are NV until measured on target hardware with a representative corpus.
- 022's coherence/decision work is intentionally not repeated; this doc assumes its R1/R2/R4 baseline and only deepens R3.

---

**Bottom line (023, 80/20):** Transactional outbox-lite is the correct *durability* bridge — atomic intent (`BEGIN` domain write + outbox row + `COMMIT`), `FOR UPDATE SKIP LOCKED` claims, at-least-once delivery made safe by upsert/delete-by-id, persisted backoff with jitter, `dead` status + replay, and latest-wins via coalescing or version guards [S1–S11]. In Flora it is **feasible but not free**: the real cost is introducing a transaction boundary into pool-bound, trait-object repositories (§4.1), not the poller itself. Because PG FTS is transactional and authoritative today and Meilisearch remains optional, **defer R3 until "no drift across crash/restart" is a stated requirement**; land 022 R1/R2/R4 first, then add this table + single bounded worker as R3. Never use `LISTEN/NOTIFY` as correctness, never block commits on Meilisearch, and keep the outbox in backups while treating the derived index as rebuildable. [S1–S16, F1–F5]
