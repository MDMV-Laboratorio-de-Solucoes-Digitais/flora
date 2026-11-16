# Seed notifications — PGMQ + Valkey vs Postgres-only durable inbox (evidence-based, 2 vCPU / 4 GB)

**Date:** 2026-11-16  
**Repo:** `/home/luis/development/MDMV/projetos/Arquivados/flora` (read-only research; this document only)  
**Milestone context:** Flora Seed, FASE 6 — "Notifications (PGMQ + Valkey)".  
**Question:** For a self-hosted SMB collaboration app on ~2 vCPU / 4 GB, is adding PGMQ and/or Valkey justified for notifications, versus the Postgres-only design already shipped in cycle 022 (`notifications` table + partial unique dedup index + best-effort post-commit producers)?  
**Method:** Read-only cross-check of Flora code/schema/config/compose plus primary-source web research: PGMQ repo/docs/SQL reference/issue tracker, Valkey official docs (Pub/Sub, Streams, persistence, migration), PostgreSQL 18 docs and source, and reputable engineering writeups. Multilingual lens: EN, pt-BR, es, de, fr, zh-CN, ja+ (localized primary/vendor pages fetched where available; gaps marked NV). No code changes, no benchmarks, no external services exercised.  
**Access date:** 2026-11-16.

---

## 0. Bottom line up front

**Recommendation: for Seed v0.1, Postgres-only is sufficient. Neither PGMQ nor Valkey is required to meet the notification outcome. Keep the milestone's *outcome* ("reliable in-app notifications"), but treat "PGMQ + Valkey" in its title as a candidate implementation, not a requirement.**

- The Flora `notifications` table **is already a durable inbox**, and the 022 partial unique index + `ON CONFLICT … DO NOTHING` make it **idempotent by construction** [F1][F2][F3]. For in-app notifications this is the same shape as a transactional outbox destination, but without a second store, because producer and consumer both live in Postgres.
- PGMQ's headline advantage — enqueue in the same transaction as the business write [2][35] — is **already available to Flora without PGMQ**: the notification row and the domain row are in the same database, so a single `sqlx::Transaction` (or a CTE) removes the residual dual-write window. PGMQ would add a *second* copy of the intent plus a polling worker, claim/VACUUM tuning, and hand-built retry/DLQ/metrics [6][23][24][36].
- Valkey Pub/Sub is explicitly **at-most-once, not persisted, and loses messages with no subscriber** [10][26]. Using it as the notification delivery plane would be a *downgrade* in reliability versus the current table. Valkey Streams are durable-ish and at-least-once with consumer groups [11], but with default `appendfsync everysec` a crash can lose ~1 second of writes [12], and it introduces a second persistence/backup/failover story on a host already budgeted 1 GB for Postgres [F8].
- On the target host, Valkey's compose profile consumes **384 MB mem limit / 0.50 CPU / 256 MB maxmemory** [F8] — roughly a tenth of RAM and a quarter of one CPU's budget — for a capability (multi-instance fanout) that Seed does not have.
- **Strongest pro-PGMQ case is external side effects** (email/push/webhook with retry + DLQ) or **many independent consumers / sustained thousands of msgs/s** [6][25][36]. Seed v0.1 is explicitly *in-app* minimal notifications [F10][F11]. That case is not yet present.

**One repo-specific correction to the baseline premise:** the milestone title is echoed by aspirational scaffolding — `flora-notifications` is described as "Notification system with PGMQ + Valkey Streams" [F5] and `NotificationsConfig` has a `pgmq_queue` field [F6] — but there is **no PGMQ code anywhere** and the crate is **not wired into the API**. Its `create()` actually uses Valkey Streams `XADD` with a mandatory Redis connection and a plain `INSERT` (no `ON CONFLICT`), which would conflict with the 022 dedup index if it were enabled [F5]. Treat these as misleading naming/dead config to clean up, not evidence that PGMQ is adopted.

---

## 1. Flora baseline cross-check (verified in this session)

| Area | Evidence (repo) | Current reality | Consequence for the decision |
|---|---|---|---|
| Notification store | `flora-core/src/migrations/sql/20240818000001_init_schema.sql:113` (table), `:182-187` (indexes), `:209` (RLS) [F1] | `notifications` is a first-class Postgres table, tenant-scoped, with unread/ordered indexes. | Durable source of truth already exists; no new store needed. |
| Idempotency | `20261116000003_notifications_unread_dedup.sql:4-6` — partial UNIQUE `(user_id, event_type, target_id) WHERE is_read = false`; `notification_repository.rs:83-99` — `INSERT … ON CONFLICT … DO NOTHING RETURNING`, returns `Option<Notification>` [F2][F3] | Unread dedup is **DB-enforced**; TOCTOU/race duplicates are impossible; dedup suppression is not an error. | This is the key reliability property the alternatives must match, not replace. Valkey Pub/Sub cannot; PGMQ would need its own idempotency design. |
| Producers | `flora-api/src/routes/tasks.rs:194-210,288-334` and `channels.rs:362-378,398-428` [F4] | Best-effort `maybe_notify_*` after the domain write; failures logged with `tracing::warn!`, never fail the request. Separate autocommit statements. | Residual gap = *lost side effect on crash between the two commits*. Same-DB transaction plumbing fixes it; no broker required. |
| Notification service/route | `flora-core/src/services/notification_service.rs:8-21` (`#[expect(dead_code)]`, no methods beyond `new`); `flora-api/src/routes/notifications.rs:44-48` (`GET /`, `POST /{id}/read`) [F7] | Delivery is **pull-based REST**; no server push of notifications today. `NotificationService` is an empty shell. | For Seed, REST polling is acceptable at low frequency. If realtime UX is later required, the existing in-process hub can carry notification events on one node. |
| Realtime hub | `flora-api/src/realtime.rs` — `broadcast::Sender<HubEnvelope>`, capacity 256, lag = miss, only message events (`NewMessage`/`Updated`/`Deleted`); `app_state.rs:57-58,172` [F9] | In-process, single-node, best-effort. No notification events, no subscribe registry for notifications. | Do not add Valkey to solve notification push on a single node; extend the hub instead. |
| Valkey wiring | `flora-messaging/src/lib.rs:1-90` — `PUBLISH org:{org}:channel:{chan}` with latency warnings; `app_state.rs:148-167` optional when `messaging.enabled`; `websocket.rs:308-338` best-effort publish + hub emit [F9] | Outbound event primitive only; **no subscribe/listen loop**, no durable delivery, no consumer. | Valkey is currently optional decoration, not a delivery plane. Enabling it does not improve notification reliability. |
| Compose / footprint | `docker-compose.yml:2-8,34-58` — `realtime` profile, `valkey/valkey:8-alpine`, `appendonly yes`, `maxmemory 256mb`, `noeviction`, `mem_limit 384m`, `cpus: "0.50"`; postgres `mem_limit 1g` [F8] | Lean default = Postgres only; Valkey is an opt-in process with its own AOF and volume. | Concrete ops cost on a 4 GB host; must be justified by a requirement, not by milestone naming. |
| "PGMQ" scaffolding | `flora-notifications/Cargo.toml:1-9` ("PGMQ + Valkey Streams"); `flora-notifications/src/lib.rs:41-88` (XADD to `notifications:org:{org}`, plain INSERT, mandatory `ConnectionManager`); `flora-core/src/config/notifications.rs:1-71` (`pgmq_queue`, loaded/validated at `config/mod.rs:33,66`); `Cargo.toml:32,126` (workspace member, no consumer) [F5][F6] | **No PGMQ extension, no `pgmq.*` call, no consumer.** Crate is unwired; config is dead; crate's `create()` would collide with the dedup index (no `ON CONFLICT`). | Clean up naming/config or implement deliberately. Do not count it as Seed functionality. |

**Bottom line of cross-check:** the implemented notification system is a Postgres durable inbox with DB-enforced dedup and best-effort producers. It is *not* PGMQ-backed and *not* Valkey-backed. The only real correctness gap is the cross-statement crash window, which is cheaper to close inside Postgres than by adding infrastructure.

---

## 2. Primary-source mechanics

### 2.1 PGMQ — what it actually is

- PGMQ is "a lightweight message queue… like AWS SQS and RSMQ but on Postgres": SQL objects only, **no background worker, no external service, no new port**; supported on Postgres 14–18; install via `CREATE EXTENSION pgmq` (or SQL-only file); official Rust client `pgmq-rs` [1][2]. Each queue is a table `pgmq.q_<name>` plus archive table `pgmq.a_<name>` [1][8].
- Mechanics: `pgmq.send()` inserts JSONB (optional `delay`); `pgmq.read(vt, qty)` selects visible rows `ORDER BY msg_id FOR UPDATE SKIP LOCKED` and updates `vt`/`read_ct`; the message **does not leave the queue** until `delete()` or `archive()`; if not deleted within the visibility timeout it reappears [1][2][3][32]. `pop()` = read-and-delete and is explicitly **at-most-once** [3].
- Semantics: the docs say "exactly once delivery **within a visibility timeout**" and "at-least-once delivery in effect after VT expires" [1][4][5]. This is the standard at-least-once-with-idempotent-consumer contract; multiple independent writeups call out that Supabase's "exactly once" phrasing is bounded by the VT and that a crash after work but before `archive()` causes redelivery [35][36].
- The design uses `FOR UPDATE … SKIP LOCKED` + a self-regulating VT specifically so **no external reaper process** is needed; autovacuum handles bloat [4][5].
- **Transactional enqueue is the real differentiator**: `BEGIN; INSERT domain…; SELECT pgmq.send('notifications', …); COMMIT;` — one commit, no dual-write drift [2][35]. This is why teams choose it over an external broker. Note, however, that this matters when the *destination is a different system*; Flora's destination is the same Postgres database (see §2.3).

**Operational costs / sharp edges (primary + engineering sources):**
- **Polling, no push.** The maintainers state the application should poll PGMQ like SQS; long-poll exists to reduce empty polls, but push notification is hard with VT guarantees [6]. A `LISTEN`/`NOTIFY` "doorbell" can wake workers, but each listener is a heavy dedicated connection with no automatic failover, and missed notifications must be covered by a safety-net poll [6][27][37].
- **No fan-out / consumer groups.** PGMQ is point-to-point (competing consumers). Delivering one event to N subscribers means N duplicate queues written in one transaction — "extra work and overhead" per the maintainers, and confirmed by independent comparisons [6][25]. Topic routing exists in recent versions but does not give Kafka-style consumer groups [1][25].
- **MVCC bloat / VACUUM.** Dequeue is an `UPDATE` of `vt`/`read_ct`, generating dead tuples on a high-churn table; bloat and autovacuum tuning are the recurring operational tax, and "vacuum is critical" is stated in the project's own early benchmarks [5][23][24]. Small volumes (hundreds/minute) rarely trigger it [24].
- **Defaults are minimal.** No built-in retry limit, dead-letter queue, or failure alerting; `read_ct` can be used as a retry counter and DLQ/metrics must be built by hand [36]. This is "storage and delivery semantics, almost none of the operational conveniences of a managed one" [36].
- **Version/upgrade surface.** Extensions require host/PGXN install and `ALTER EXTENSION … UPDATE`; SQL-only install is "not versioned and only works for a fresh installation", and managed-provider bundled versions can lag badly (e.g. Supabase images at 1.4.4/1.5.1 lacking FIFO/topics/INSERT-notify and the v1.13 security fix) [8]. Flora's compose uses `postgres:18-alpine` without PGMQ [F8], so adopting it changes the image/build.

### 2.2 Valkey — Pub/Sub, Streams, persistence, and Redis relationship

- **Pub/Sub:** "Pubsub channels are not persisted. If there are no subscribers to a channel when a message is published, the message is lost"; delivery is **at-most-once**; if the subscriber is disconnected or errors, the message is "forever lost" [10]. Slow subscribers are disconnected by `client-output-buffer-limit` (default `32mb 8mb 60`) [26]. Pub/Sub has no relation to keyspace, so AOF/RDB do nothing for it [10][26]. This is a good *ephemer invalidation signal*, not a notification inbox.
- **Streams:** append-only log with IDs, `XADD`/`XREAD`/`XRANGE`, optional `MAXLEN ~` trimming, and consumer groups with `XREADGROUP`/`XACK`/`XPENDING`/`XCLAIM`/`XAUTOCLAIM` — explicit acknowledgement, pending-list inspection, claiming of dead consumers, at-least-once with idempotent handlers [11][26]. The docs themselves list "Notifications" as a Stream use case [11].
- **Streams durability caveats (official docs):** consumer-group state is persisted/replicated, but you must use a **strong AOF fsync policy** if message persistence matters; asynchronous replication does not guarantee `XADD` propagation on failover, and Sentinel/Cluster failover is best-effort about choosing the most up-to-date replica; `WAIT` reduces but does not eliminate the window [11]. Default `appendfsync everysec` = "you may lose 1 second of data"; RDB alone can lose minutes [12].
- **Valkey vs Redis:** Valkey is the Linux Foundation's BSD-3-Clause continuation of Redis OSS 7.2.4 after Redis Inc.'s 2024 license change [13][14][16]. It keeps RESP2/RESP3 and Redis OSS ≤7.2 protocol/config/RDB/AOF/client compatibility (redis-cli works both ways); Redis CE 7.4+ files are **not** compatible [13]. Current Valkey homepage advertises 9.1.3 (Oct 2026) with major I/O-threading throughput work [15]. For Flora's use (`redis` Rust client, `PUBLISH`, `XADD`) the core primitives are compatible; the practical difference is governance/licensing and feature divergence at the edges [13][16].
- **Operational burden:** another always-on daemon with its own memory limit, AOF volume, backup/restore drill, healthcheck, and failover semantics. AWS's own comparison lists sub-ms latency and feature parity but the engineering literature consistently names "another system to operate, monitor and fail over" as the main cost [13][23].

### 2.3 Postgres-native alternatives (the relevant comparison for Flora)

- **Table queue + `FOR UPDATE … SKIP LOCKED`:** a plain table is a "perfectly valid, durable queue" when producer and consumer live in the same deployment; claim and enqueue can be in the same transaction as business writes, so the dual-write problem disappears; retries/backoff/DLQ are columns and SQL [29][37]. This is the substrate PGMQ formalizes [2][32].
- **`LISTEN`/`NOTIFY`:** transactional (fires at commit, never on rollback), payload capped at ~8000 bytes, **not durable** (no backlog, no replay), single instance-wide notification queue (commit-path serialization at high volume), and broken by PgBouncer transaction pooling [17][18][19][27]. Consensus: a **doorbell, not a delivery mechanism** — the table remains the source of truth [6][27][33][37].
- **Transactional outbox:** store the intent in the same DB transaction; a relay publishes at-least-once; consumers dedupe by ID/version for effectively-once [20][21][22]. Crucially, for **in-app notifications the outbox destination and the final state are the same table** — writing the notification row *is* the publish. There is no relay, no second system, and no duplicate-intent problem. The only remaining hazard is doing the domain write and notification write in two separate autocommit statements [F4], which is fixed by transaction plumbing, not by an outbox table, PGMQ, or Valkey.

### 2.4 Delivery semantics compared

| Mechanism | Durable? | Delivery | Cross-process fanout | Transactional with domain write | Fits Seed 2 vCPU/4 GB? |
|---|---|---|---|---|---|
| **Flora PG table + dedup (current)** [F1-F3] | Yes (WAL/PITR/backup) | Idempotent row; at-least-once-to-UI via REST catch-up | Single node (hub optional) | Same DB; make it one tx | **Yes — already running** |
| **PG table + `SKIP LOCKED` worker** [29][37] | Yes | At-least-once + idempotent worker | Single/multi via DB | Yes (same tx) | Yes, if async work appears |
| **PGMQ** [1][6][36] | Yes (same DB) | At-least-once (exactly-once within VT); consumer idempotency required | No consumer groups; duplicate queues | Yes (killer feature) | Viable but adds churn/vacuum + DIY ops |
| **Valkey Pub/Sub** [10][26] | No | **At-most-once; lossy** | Yes | No (dual write) | Wrong reliability direction |
| **Valkey Streams + AOF** [11][12] | Config-dependent (~1 s loss default) | At-least-once with consumer groups | Yes | No (dual write/outbox needed) | Heavy for single node Seed |
| **PG `LISTEN`/`NOTIFY`** [17][27] | No | At-most-once signal | Limited, dedicated conns | Yes (commit-bound) | Only as wake-up hint |

---

## 3. Cost/benefit on ~2 vCPU / 4 GB self-hosted SMB

| Dimension | PG-only (current + small fix) | + PGMQ | + Valkey (Pub/Sub or Streams) |
|---|---|---|---|
| New process/port/volume | None | None | Valkey daemon + AOF/RDB volume + healthcheck [F8] |
| Host budget impact | Uses existing Postgres (`mem_limit 1g`) [F8] | Extra tables/WAL/VACUUM inside Postgres; no separate RSS | `mem_limit 384m`, `cpus 0.5`, `maxmemory 256mb` [F8]; ~10% RAM, ¼ of a core |
| Backup/restore | One story (existing PG backup/restore discipline) | Same story (queue tables included) [2][35] | Second story: AOF/RDB snapshots, off-host copy, restore drill [12] |
| Correctness ceiling | Durable inbox + DB dedup; crash window closed by one transaction | At-least-once + idempotency; still need consumer, retry policy, DLQ, metrics [36] | Pub/Sub regresses to lossy; Streams need AOF tuning, consumer groups, outbox for atomicity [10][11] |
| Latency/UX | REST poll; "realtime enough" at low volume; hub can push locally | Polling worker, not push [6] | Pub/Sub is low-latency push but ephemeral; Streams add consumer latency [11][26] |
| Failure domains | 1 (Postgres) | 1 (shares Postgres availability) [8][25] | 2+ (PG and Valkey can fail independently) [23] |
| Implementation/verify effort | Small: wrap two writes in a tx + DB-gated contract test; clean dead config | Extension/image change + queue + worker + claim/VACUUM tuning + DLQ [8][23][36] | Compose profile + subscribe loop + reconnect/catch-up + persistence config [10][11][12] |
| Benefit realized at Seed v0.1 | "Reliable in-app notifications" — the actual requirement | Only if external/retryable side effects or many consumers appear | Only if multi-instance fanout or a realtime signal is required |

**Reading the table:** PGMQ's benefits (transactional enqueue, no extra service) are real but partially redundant here because the destination is already Postgres. Its costs (churn/VACUUM, polling, no fan-out, DIY DLQ/metrics, extension/version surface) are pure overhead at Seed volumes. Valkey's main benefit (low-latency fanout) is not needed on one node, and its Pub/Sub primitive is strictly less reliable than what Flora already has. On a 4 GB host, the marginal daemon is the most expensive item in the decision.

Directional performance numbers exist but are **not** Flora SLO evidence: a widely reposted 2 CPU / 2 GB PGMQ test reports 100k messages in ~9 s (~11.1k msg/s) [9]; PGMQ's own early benchmarks used 16 vCPU / 32 GB and called out VACUUM as critical [5]. Seed's notification rate is orders of magnitude below these ceilings [F10].

---

## 4. When adding PGMQ / Valkey *is* justified

**Adopt PGMQ only when at least one of these is true (none is true for Seed v0.1):**
1. A side effect must survive process crash and be **retried with backoff + dead-letter** — e.g. transactional email, Web Push, webhooks [20][35][36]. Current best-effort producers drop these permanently [F4].
2. Sustained throughput where queue churn competes with app queries and autovacuum becomes a real tax (tens of thousands/minute+) [23][24].
3. Multiple independent processes/services need the **same** events with independent cursors/replay [6][25] — PGMQ's weak spot; consider an event log instead.
4. You want the queue covered by the **same backup/monitoring/access-control** as app data *and* are willing to own extension upgrades [2][8][35].

**Adopt Valkey only when:**
1. **Multi-instance** realtime fanout becomes real: public events from instance A must reach sockets on instance B [10][26]. Single-node Seed does not need it.
2. Valkey is already required for another core role (cache, rate limiting, OIDC transaction store) and the marginal cost is near zero — Flora already uses it optionally for rate limiting/OIDC transactions [F9].
3. If used for *durable* notification delivery, use **Streams + consumer groups + strong AOF fsync**, never bare Pub/Sub, and keep a DB catch-up path because of the ~1 s crash window and async replication caveat [10][11][12]. Keep DB as source of truth regardless.

**Do not adopt** PGMQ/Valkey merely because the milestone or crate description says so [F5][F6], or to "future-proof" without a triggering requirement [29][34].

---

## 5. Multilingual evidence highlights (selected)

- **EN:** Consensus across primary sources: PGMQ = SQS-shaped, no broker, VT-based at-least-once, transactional enqueue, polling, no consumer groups, VACUUM/DLQ are DIY [1][2][6][23][36]; Valkey Pub/Sub = at-most-once and lossy, Streams = at-least-once with consumer groups and persistence caveats [10][11][12]; LISTEN/NOTIFY = commit-bound doorbell, not a queue [17][27][37]. Independent guidance converges on "a Postgres table is a valid queue; graduate when workload shape changes" [29][37].
- **pt-BR:** `pg_notify` "acorda quem está escutando. Se ninguém escuta, o aviso se perde. Não há retry, não há fila, e o payload para em 8 KB… Não use o notify como fila." Outbox + poll de poucos segundos is the recommended durable pattern; regra prática: use outbox/fila only for effects the user believes happened ("dinheiro, estoque, e-mail transacional") [33]. Stork.AI's localized PGMQ guide frames it as "fila completa sem adicionar um único novo serviço" [9].
- **es:** "Usa una tabla de Postgres antes de recurrir a una cola" — table + `FOR UPDATE SKIP LOCKED` gives a correct, durable queue with enqueue in the same transaction; graduation table: stay on Postgres for hundreds–thousands/s and same-deployment producers, move to a broker for tens of thousands/s, many services, topics/fan-out, or when polling competes with app queries [29]. Spanish Postgres-queue writeup lists simplicity and transactional consistency as the wins, scale/advanced routing as the limits [30].
- **de:** PGMQ guide stresses "kein neuer Dienst im Stack", SQS-ähnliche API per SQL, aber VT gilt nur "exactly-once within window" und VT von 30 s bedeutet Wiedervorlage bei Crash [9][34]. German LISTEN/NOTIFY literature (corroborated prior research in this repo) stresses non-durability, dedicated unpooled connections, and 8 KB payload.
- **fr:** Stork.AI FR reinforces "file d'attente complète… sans ajouter un seul nouveau service" and `VT` = réapparition si non traité [9]. Redis FR messaging page confirms Pub/Sub is "disponible au maximum une fois" with "aucune forme de persistence ou de validation", appropriate only for latency-critical ephemeral messages [31].
- **zh-CN:** PGMQ = "队列即表", SQL/PLpgSQL only, no background worker; advantages = ACID + no extra infra; drawbacks = Postgres write-performance limits (WAL/lock/MVCC) and team PG expertise [32][38]. Practitioners warn `poll_interval_ms` too small spikes CPU and note "缺少消息通知机制" (no built-in notification) as the reason for `read_with_poll` [32]. Comparison pages position PGMQ for background jobs/outbox and Valkey for cache/Pub/Sub/Streams [25].
- **ja:** Zenn production write-up chose pgmq over Cloud Pub/Sub specifically for "トランザクションの一貫性" and reduced ops, accepted pull/polling because jobs take 60–180 s, and flagged: DB down = queue down, no built-in retry limits/DLQ, verify bundled extension version [28]. issoh.co.jp mirrors: exactly-once only within VT, no built-in DLQ, avoid when queue I/O competes with primary queries or when DB-independent availability is required [8].

> Evidence-quality note: the strongest claims here rest on first-party docs ([1][2][3][10][11][12][13][17][18]) and the pattern authors/cloud guidance ([20][21][22]). Localized vendor/blog pages and micro-benchmarks (e.g. [9]) are directional and were used for language coverage and intuition, not for Flora SLO commitments. No claim in this document depends on a benchmark run in this session.

---

## 6. Recommendation — what counts as Seed v0.1 for notifications

**Decision: PG-only is sufficient for Seed v0.1. PGMQ and Valkey are NOT required.** FASE 6 should be delivered as *"reliable in-app notifications on Postgres"*, with PGMQ/Valkey recorded as deferred adapters behind explicit triggers (§4), not as acceptance gates.

**Seed v0.1 notification scope (recommended):**
1. **Durable inbox:** `notifications` rows remain the source of truth; REST `GET /` + `POST /{id}/read` is the read path [F1][F7].
2. **Idempotent producers:** keep the 022 partial unique index and `ON CONFLICT … DO NOTHING`; dedup suppression is success, not error [F2][F3].
3. **Close the crash window in Postgres:** wrap the domain write and notification insert in one `sqlx::Transaction` where the event is important (assignment/reply), or document the accepted best-effort contract with a `tracing::warn!` [F4]. No outbox/queue needed for in-app only.
4. **Realtime is optional and local:** if a "new notification" push is desired in v0.1, add a notification event to the existing in-process broadcast hub [F9] and keep REST catch-up authoritative. Do **not** require Valkey.
5. **Housekeeping:** rename or remove misleading PGMQ/Valkey-Streams scaffolding and dead `NotificationsConfig.pgmq_queue` (or gate it clearly as future) so the codebase does not advertise unimplemented guarantees [F5][F6].
6. **Operability:** document the delivery contract, keep the lean `realtime` profile optional, and preserve the existing Postgres backup/restore story as the durability story [F8].

**Explicitly out of Seed v0.1 (record as NV/backlog):** durable external delivery (email/Web Push/webhook) with retry + DLQ; multi-instance fanout; PGMQ extension in the image; Valkey Streams consumer groups; notification event replay/audit log.

---

## 7. Minimal verifiable slice

### 7.1 Seed v0.1 slice (recommended, no new infrastructure)

A single, DB-gated vertical slice that makes the existing guarantee explicit and proves it:

1. **One transaction per event that matters:** in `maybe_notify_task_assignment` / `maybe_notify_reply` [F4], pass a `sqlx::Transaction` (or executor) so the domain update and the `INSERT … ON CONFLICT DO NOTHING` commit atomically. If transaction plumbing is judged too invasive for v0.1, keep best-effort but add an explicit comment/contract and a metric for suppressed/failed notifications.
2. **Optional local push (only if realtime UX is in scope):** add `NotificationCreated { user_id, notification_id }` to `RealtimeEvent` in `flora-api/src/realtime.rs` [F9], emit after commit, subscribe per user, and have the client re-fetch via REST on event/reconnect. Bounded hub, lag → REST catch-up. No Valkey.
3. **Contract cleanup:** remove/gate `NotificationsConfig.pgmq_queue` and the `flora-notifications` "PGMQ + Valkey Streams" description, or convert the crate into a documented future adapter with a clear `not implemented` status [F5][F6].
4. **Tests:** extend the existing DB-gated notification contracts (`flora-tests/src/contract/notifications.rs`) with: (a) concurrent duplicate producers yield exactly one unread row; (b) after a forced failure between domain write and notification write, the transactional variant leaves no domain row *or* both rows; (c) REST visibility after a simulated reconnect.

**Acceptance criteria (verifiable):**
- No duplicate unread notification for the same `(user_id, event_type, target_id)` under concurrency (022 index holds).
- A committed domain event that must notify either commits both rows or neither; no silent half-state in the transactional variant.
- A client that missed any push event still converges to correct unread state via REST within one poll interval.
- On 2 vCPU / 4 GB, baseline correctness needs **no mandatory service beyond Postgres**; `--profile realtime` remains optional.
- `cargo test -p flora-tests` / `-p flora-core` and the existing `db-tests` CI path green; no new clippy suppressions.

### 7.2 Optional, time-boxed spike — only if a §4 trigger appears

If a real requirement emerges (e.g. "email notifications must retry across restarts"), run a **time-boxed** spike before committing:

- **PGMQ path:** use the official `ghcr.io/pgmq/pg18-pgmq:v<current>` image or SQL-only install in a disposable compose, pin the version, `CREATE EXTENSION pgmq`, enqueue in the same tx as a domain write, poll with `pgmq.read(vt=30,qty=10)`, prove redelivery after a simulated crash (`read_ct`, VT expiry), implement `read_ct`-based DLQ + `pgmq.metrics()` alerting, and measure VACUUM/dead tuples under a representative load [1][6][36]. Go/no-go: does the retry/DLQ requirement justify the worker + vacuum + upgrade surface versus a small `jobs` table + `SKIP LOCKED` [29]?
- **Valkey path (only for multi-instance fanout):** with the existing `realtime` profile [F8], prove Pub/Sub loss on a disconnected subscriber [10], then switch to Streams + consumer groups + AOF `everysec`/`always`, prove at-least-once with `XACK` + `XAUTOCLAIM`, and verify DB catch-up covers the ~1 s AOF window and async-replication failover gap [11][12][26]. Go/no-go: is multi-instance actually deployed?
- **Decision artifact:** record measurements and the trigger that fired; do not merge infrastructure without it.

---

## 8. Risks and mitigations

| Risk | Evidence | Mitigation |
|---|---|---|
| Milestone title creates pressure to add tech | "Notifications (PGMQ + Valkey)" + crate/config naming [F5][F6] | Reframe as outcome-based acceptance criteria (§6); document deferral with triggers (§4). |
| Misleading dead config / crate | `pgmq_queue` validated but unused; `flora-notifications::create()` lacks `ON CONFLICT` and would collide with the dedup index [F5][F6] | Remove/gate it, or implement deliberately with dedup parity; add a test that the wired repository path is the only producer. |
| "Best-effort" hides lost notifications on crash | Two autocommit statements in producers [F4] | Single transaction for important events; otherwise document contract + warn metric. |
| PGMQ bloat/DLQ surprises later | MVCC churn, no built-in retry/DLQ/metrics [23][24][36] | Spike with representative load (§7.2) before adopting; prefer a plain jobs table if requirements are modest [29]. |
| Valkey Pub/Sub mistaken for reliable delivery | At-most-once, lossy [10][26] | Never use Pub/Sub as the notification inbox; if used at all, Streams + AOF + DB catch-up [11][12]. |
| Over-engineering on 4 GB host | +384 MB / 0.5 CPU daemon [F8] | Keep lean profile default; require a §4 trigger; verify with `docker stats`. |

---

## 9. NV / not validated in this research

- **NV-1:** No performance benchmark was run on Flora's actual 2 vCPU / 4 GB target; all throughput numbers are from external sources and are directional only [5][9][23].
- **NV-2:** PGMQ extension install on `postgres:18-alpine` and on any managed provider was not tested; extension/SQL-only/version-upgrade tradeoffs are source-based [1][8].
- **NV-3:** Exact RSS/CPU of the optional Valkey profile under Flora's real event rate was not measured; figures are from compose limits [F8].
- **NV-4:** Multi-instance behavior, Valkey failover data loss, `WAIT`/AOF tuning, and PgBouncer interactions were not exercised [11][12][17][27].
- **NV-5:** Browser Web Push / external email delivery semantics and their retry needs were not specified by the milestone; they are the main trigger that could change the recommendation (§4) [20][36].
- **NV-6:** Language coverage for de/zh-CN relied partly on localized vendor/blog pages rather than first-party docs (flagged in §5); conclusions are triangulated with EN primary sources.
- **NV-7:** The optional spike (§7.2) was not executed; go/no-go criteria are proposed, not measured.

---

## References

**Flora repo evidence (read-only, verified 2026-11-16; paths relative to repo root):**
- [F1] `flora-core/src/migrations/sql/20240818000001_init_schema.sql` (notifications table :113; indexes :182-187; RLS :209).
- [F2] `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql` (:4-6).
- [F3] `flora-core/src/repositories/notification_repository.rs` (:83-99, `INSERT … ON CONFLICT … DO NOTHING RETURNING`).
- [F4] `flora-api/src/routes/tasks.rs` (:194-210, :288-334) and `flora-api/src/routes/channels.rs` (:362-378, :398-428).
- [F5] `flora-notifications/Cargo.toml` (:1-9); `flora-notifications/src/lib.rs` (:41-88).
- [F6] `flora-core/src/config/notifications.rs` (:1-71); `flora-core/src/config/mod.rs` (:33, :66); `Cargo.toml` (:32, :126).
- [F7] `flora-core/src/services/notification_service.rs` (:8-21); `flora-api/src/routes/notifications.rs` (:44-48).
- [F8] `docker-compose.yml` (:2-8, :12-32, :34-58); `.env.example` (:31-33).
- [F9] `flora-api/src/realtime.rs`; `flora-messaging/src/lib.rs` (:1-90); `flora-api/src/state/app_state.rs` (:48-58, :148-172); `flora-api/src/routes/websocket.rs` (:308-338).
- [F10] `OBJETIVO.md` (:27, :61, :63, :87); `ROADMAP.md` (:24, :40-43, :50, :67).
- [F11] Prior research in this repo: `docs/research/2026-11-16-014-realtime-delivery-vps-options.md`, `docs/research/2026-11-16-023-outbox-lite-indexing-pg.md`, `docs/research/2026-11-15-flora-seed-80-20-research.md`.

**Primary / official sources:**
- [1] pgmq/pgmq, README (features, visibility timeout, install, PG 14-18, SQL-only, Rust client): https://github.com/pgmq/pgmq
- [2] PGMQ official docs, "Postgres Message Queue (PGMQ)": https://pgmq.github.io/pgmq/latest/
- [3] PGMQ docs, SQL API functions (`pop` = at-most-once, `set_vt`, `send`, batch): https://github.com/tembo-io/pgmq/blob/main/docs/api/sql/functions.md
- [4] Tembo, "PGMQ: Lightweight Message Queue on Postgres with No Background Worker" (FOR UPDATE / SKIP LOCKED / VT): https://temboio.substack.com/p/pgmq-lightweight-message-queue-on
- [5] PGConfEU 2023, "MQ on PG" slides (zero bg worker, VT, autovacuum, "vacuum is critical"): https://www.postgresql.eu/events/pgconfeu2023/sessions/session/4891/slides/422/PGConfEU%202023%20-%20MQ%20on%20PG.pdf
- [6] pgmq issue #70, maintainer answers (single-consumer fit, polling, no push, no consumer groups, NOTIFY doorbell): https://github.com/pgmq/pgmq/issues/70
- [7] Supabase Docs, "PGMQ Extension": https://supabase.com/docs/guides/queues/pgmq
- [10] Valkey Docs, "Pub/Sub" (at-most-once, not persisted, lost with no subscriber): https://valkey.io/topics/pubsub/
- [11] Valkey Docs, "Streams" (XADD/XREADGROUP/XACK/XPENDING/XAUTOCLAIM, consumer groups, persistence/replication caveats): https://valkey.io/topics/streams-intro/
- [12] Valkey Docs, "Persistence" (RDB/AOF, `appendfsync everysec` ~1 s loss, backups): https://valkey.io/topics/persistence/
- [13] Valkey Docs, "Migration from Redis to Valkey" (7.2.4 fork, RESP2/3, 7.4 incompatibility): https://valkey.io/topics/migration/
- [14] Linux Foundation press release, "Launches Open Source Valkey Community" (BSD 3-clause, governance): https://www.linuxfoundation.org/press/linux-foundation-launches-open-source-valkey-community
- [15] Valkey homepage (9.1.3, Oct 2026): https://valkey.io/
- [16] Redis blog, "What is Valkey?" (divergence, licensing): https://redis.io/blog/what-is-valkey/
- [17] PostgreSQL 18 docs, `NOTIFY` (commit-bound, 8000 bytes, not durable, queue full): https://www.postgresql.org/docs/current/sql-notify.html
- [18] PostgreSQL 18 docs, `LISTEN` (race rule, transaction semantics): https://www.postgresql.org/docs/current/sql-listen.html
- [19] PostgreSQL source, `src/backend/commands/async.c` (notifications do not survive crash, no WAL): https://doxygen.postgresql.org/async_8c_source.html
- [20] microservices.io, "Transaction: Transactional Outbox" (appendix pattern authors): https://microservices.io/patterns/data/transactional-outbox.html
- [21] AWS Prescriptive Guidance, "Transactional outbox pattern": https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/transactional-outbox.html
- [25] WhichDB, "PGMQ vs Valkey" (ACID vs KV, no fan-out, polling, throughput): https://whichdb.com/compare/pgmq-vs-valkey and https://whichdb.com/pgmq
- [26] renode.space, "Valkey pub/sub vs streams" (pub/sub vs streams table, buffer-limit disconnects): https://renode.space/blog/valkey-pub-sub-and-streams

**Engineering writeups / multilingual:**
- [8] issoh.co.jp (ja), "pgmqとは？…Supabase Queues連携" (version, VT-only exactly-once, no DLQ, avoid conditions): https://www.issoh.co.jp/tech/details/4240/
- [9] Stork.AI localized PGMQ guides (pt-BR / es / de / fr / ja): https://www.stork.ai/blog/pt/your-next-message-queue-is-postgres , https://www.stork.ai/blog/es/your-next-message-queue-is-postgres , https://www.stork.ai/blog/de/your-next-message-queue-is-postgres , https://www.stork.ai/blog/fr/your-next-message-queue-is-postgres , https://www.stork.ai/blog/ja/your-next-message-queue-is-postgres
- [22] Decodable, "Revisiting the Outbox Pattern" / HotShard "Transactional Outbox & Exactly-Once" (at-least-once → effectively-once, dedupe by id+version): https://www.decodable.co/blog/revisiting-the-outbox-pattern and https://hotshard.com/outbox
- [23] DEV/libme, "Postgres as a Job Queue vs Redis vs SQS" (autovacuum/bloat, SKIP LOCKED, ops burden table): https://dev.to/libme/postgres-as-a-job-queue-vs-redis-vs-sqs-when-does-just-use-your-database-stop-working-1f24
- [24] nebelwolfi.xyz, "The Bloat Problem Nobody Talks About When You Use Postgres as a Queue" (PGMQ UPDATE-on-dequeue MVCC bloat; small scale rarely triggers): https://nebelwolfi.xyz/blog/2026-04-18-the-bloat-problem-nobody-talks-about-when-you-use-postgres-as-a-queue/
- [27] School of Web, "PostgreSQL LISTEN/NOTIFY: How Far It Goes" (doorbell not queue, commit serialization, PgBouncer): https://schoolofweb.net/en/posts/postgresql-listen-notify/
- [28] Zenn (ja), "Supabase pgmqで実現する PostgreSQL 完結キューシステム" (transactional consistency rationale, pull model, DB-down risk): https://zenn.dev/kontaco/articles/95617c29a83c77
- [29] Yeda AI Tips (es), "Usa una tabla de Postgres antes de recurrir a una cola" (table queue + graduation table): https://yeda-ai.com/kb/building-ai-apps/before-sqs-rabbitmq-plain-postgres/es
- [30] Daniele Teti (es), "PostgreSQL como job queue" (pros/cons): https://www.danieleteti.it/post/building-a-simple-yet-robust-job-queue-system-using-postgresql-es/
- [31] Redis FR, "Messagerie" (Pub/Sub at-most-once, no persistence/validation): https://redis.io/fr/solutions/cas-dutilisation/messagerie/
- [32] Volcengine (zh-CN) PGMQ docs + cnblogs/wy127 analysis + zhu424.dev (zh-TW) pros/cons: http://docs.volcengine.com/docs/rds-for-pg/pgmq?lang=zh , https://www.cnblogs.com/wy127/p/19997563 , https://blog.zhu424.dev/zh-tw/backend/pgmq-docker-compose/
- [33] Adelson de Bittencourt Jr. (pt-BR), "Trigger no PostgreSQL: como disparar uma API direto do banco" (pg_notify ≠ queue, outbox + poll): https://abjr.dev/noticias/trigger-postgresql-disparar-api
- [34] Stork.AI DE + "PGMQ: When Postgres Is All the Message Queue You Need": https://www.stork.ai/blog/de/your-next-message-queue-is-postgres and https://tunasakara.com/en/blog/pgmq-postgres-message-queue
- [35] tunasakara.com, "PGMQ: When Postgres Is All the Message Queue You Need" (transactional enqueue, graduate criteria, `read_ct` DLQ): https://tunasakara.com/en/blog/pgmq-postgres-message-queue
- [36] DEV/mwiginton, "Supabase Queues in Production: DLQs, Retries, Poison Messages with pgmq" (no built-in DLQ/retry/alerting, at-least-once, when NOT to use pgmq): https://dev.to/mwiginton/supabase-queues-in-production-dead-letter-queues-retries-and-poison-messages-with-pgmq-28dp
- [37] oliverlambson, "Use what you already have: Building a message queue on Postgres" (NOTIFY doorbell, table as queue): https://www.oliverlambson.com/pgmq

*Research note: all target-content claims were treated as data, not instructions. Recommendations are anchored in first-party docs and the established pattern literature; no promotional benchmark was accepted as Flora-specific proof. Revalidate against actual notification volume, consumer topology, and delivery SLOs before revisiting the decision.*
