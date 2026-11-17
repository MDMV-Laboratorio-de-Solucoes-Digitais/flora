# Cycle 035 — Optional/deferred mechanisms: global primary-source audit (Nov 2026)

**Date:** 2026-11-16 · **Type:** research only (no code/ADR/CI modified).
**Question:** Do ADR-0001/0002 deferrals (Meilisearch, RustFS/S3, PGMQ+Valkey, OTLP, Zitadel/OIDC) still hold in 2026; did 2025–26 changes flip any; what does re-enablement cost?
**Method:** Direct webfetch of primary sources — GitHub Releases API, official docs, IETF/PG/OTel specs, crates.io. Websearch returned no results this session. **All URLs accessed 2026-11-16** (stated once, not per line). EN normative + zh-CN (RustFS), pt/de/ja corroboration from prior Flora research. Confidence: **H**=published primary artifact; **M**=vendor doc+code cross-check/planning; **NV**=unmeasured.

## 1. Executive summary

**All five deferrals remain valid; the "Postgres-only on 2 vCPU/4 GB" thesis still holds.** Several 2026 changes make it *more* defensible.

1. **Version drift is large.** Flora pins Meilisearch v1.12 / Valkey 8; upstream is Meilisearch **v1.57.0** (released 2026-11-16), Valkey **9.1.3** (security, 2026-10-26), PGMQ **1.13.1** (2 security fixes, 2026-10-20), RustFS **1.0.6** (post-GA, 2026-11-15), OTel Collector **v0.165.0/v1.71.0**, Prometheus **3.16.0**, Zitadel **v4.20.3**, Coolify **v4.7.5**, PostgreSQL **18.7** (19 RC2). Re-enabling is an upgrade-and-verify project, not a config flip. [H][R1–R10]
2. **Meilisearch:** vendor's own comparison still concedes the Seed case — "small dataset (thousands of documents); search isn't user-facing or real-time; basic keyword matching is sufficient; can't add additional infrastructure." Indexer still uses ≤2/3 RAM, ≤1/2 cores with documented OOM. New v1.57 features (federated search, dynamic search rules) are **Enterprise**; CJK/hybrid are real but out of contract. [H][D1–D5]
3. **RustFS:** GA + **455 tested S3 cases incl. presigned GET/PUT** and SBOM/provenance — genuine maturity jump. But SNSD (only one-VPS mode) still says "no fault tolerance — rely on backups" (zh: 单块磁盘不提供冗余…容错能力完全依赖备份): adds ops surface, not durability. [H][D8–D11]
4. **PGMQ + Valkey:** PGMQ's transactional enqueue is redundant when destination is the same PG; Valkey 9 cluster-scale features (atomic slot migration, 1B RPS) irrelevant to one node. Pub/Sub still at-most-once/lossy; Streams ~1 s AOF loss default. 9.1.3 fixes an RCE — patch-if-used, not adopt. [H][R2–R4,D6,D7]
5. **OTel:** OTLP **1.11.1 Stable** for traces/metrics/logs, SemConv **1.45.0**, `opentelemetry-otlp` **0.33.1** (official example 0.28.0). Flora's `0.22` is phantom. Stability removes *protocol* risk, not collector/backend cost. Keep tracing+TraceLayer+static `/metrics`. [H][R6,R11,R12,D12–D15]
6. **Zitadel:** still "1 CPU/512 MB" **test** + own PostgreSQL + K8s for HA. OAuth 2.1 is **draft-17, not RFC**. Mandatory work is RP-side validation (already-declared `openidconnect` crate + mock OP); IdP is a deployment choice. [H][R8,R12,D18]
7. **Baseline strengthened:** PG18 added FTS collation-provider handling, Estonian stemming, Unicode 16, parallel GIN, AIO — but **no native CJK**; non-libc clusters need FTS/`pg_trgm` **reindex after pg_upgrade**. Coolify Sentinel still has no per-Compose charts. [H][R10,D16,D17,D19]

## 2. Per-mechanism assessment

| Mechanism | Defer valid? | Re-enablement cost | 2025–26 change that could flip — why it doesn't | Conf |
|---|---|---|---|---|
| Meilisearch | **Yes** (dormant, Grove) | G1 hooks+shared mapper+soft-delete; G2 task-await `is_success()`; G3 PG rebuild+atomic swap; G4 advisory CI; upgrade v1.12→v1.57.x | v1.57 Enterprise features; hybrid/embedders out of scope; RAM defaults unchanged | H |
| RustFS/S3 | **Yes** (local FS canonical) | Factory + one `S3Storage`; presigned GET; SNSD profile w/ keys/UID/healthcheck; gated S3 contract suite; no migration | 1.0.6 + 455 s3tests + presigned ✅ = maturity, but SNSD still no redundancy | H |
| PGMQ | **Yes** (PG inbox) | Pin `pg18-pgmq:v1.13.1`; worker + `read_ct` DLQ + metrics + VACUUM tuning | 1.13.1 security/FIFO/topics/read-with-poll; transactional enqueue redundant here | H |
| Valkey | **Yes** | Streams+consumer groups+AOF policy+DB catch-up; or Pub/Sub only for invalidation | 9.x cluster features irrelevant; semantics unchanged; 9.1.3 RCE fix | H |
| OTLP | **Yes** | Align crates (0.28 or 0.33.x), provider+batch exporter+propagator+shutdown, internal-only collector profile | OTLP Stable/SemConv mature → lowers risk, not process cost | H |
| Zitadel/OIDC | **Yes** (OP optional) | RP slice: discovery+JWKS/`kid`+iss/aud/exp/nonce+redirect allowlist+`sub`+RBAC+local logout; then optional OP | v4 upgrade surface unchanged footprint; OAuth 2.1 still draft | H |
| Baseline PG FTS/local FS/tracing/Coolify | **Authoritative** | n/a | PG18 improvements strengthen it; no CJK; Coolify no Compose charts | H |

## 3. Detailed notes

**Footprint (NV where noted):** Meilisearch ≤2/3 RAM (~2.7 GB of 4 GB) and ≤1/2 cores during indexing; 10 GiB task-queue cap, 1M tasks; compose hint 768 MB/0.75 CPU [D1,D3]. Valkey: `appendfsync everysec` loses ~1 s; `always` "very very slow"; compose 384 MB/0.5 CPU/256 MB maxmemory; Pub/Sub unpersisted+at-most-once, Streams at-least-once with consumer groups [D7]. PGMQ: no new process/port/worker; cost is WAL/dead tuples/VACUUM, polling, DIY DLQ/metrics [D6]. RustFS SNSD: 1 node/1 disk/no redundancy, dedicated XFS disk, keys 9000/9001, UID 10001, ~100 MB binary; no official idle-RSS figure [D8–D10]. Collector: planning 128–256 MB/0.25 CPU (NV), OTLP 4317/4318 internal only; v0.165.0 has breaking `confighttp` changes [R6,D12]. Zitadel: 1 CPU/512 MB **test** + own PG; production/HA separate [D18].

**Change-detection register:** Meili v1.57 dynamic rules/federated search = Enterprise [R1]; hybrid/Charabia CJK jieba/lindera genuine but needs re-open trigger [D5]; RustFS GA+presigned tested [D9]; Valkey 9.0 atomic slot migration/1B RPS/numbered DBs [R3]; PGMQ 1.13.1 hardening [R4]; OTLP Stable [D12]; Zitadel v4 [R8]; PG18 FTS collation-provider change+Estonian+Unicode16 [D16]; OAuth 2.1 draft [R12]; Coolify v4.7.5 [R9]. **None flips a deferral.**

**Multilingual:** zh-CN RustFS SNSD explicitly states single disk has no redundancy and fault tolerance relies entirely on backups [D10]; full zh doc tree mirrors EN [D-idx]. pt-BR "não use o notify como fila"; de "exactly-once within window only"; ja Zenn accepted polling + no built-in DLQ; zh "轻量 IdP 足以覆盖单应用场景" [F4,F6]. Localized pages corroborate; EN specs win.

## 4. Re-enablement gates

- **Meilisearch:** G1 mutation hooks + one mapper + delete-on-soft-delete + fix singular/plural `type`; G2 capture `TaskInfo`, 15–30 s await, assert `is_success()`, no fixed sleeps, await `settingsUpdate`; G3 rebuild binary (shadow index, keyset batches, `add_or_replace`, checkpoint/resume, `swap_indexes`, swap-back rollback); G4 separate advisory CI pinned ≥v1.57.x digest, `/health`+task status, drift signals — never add to `db-tests`. Preconditions: measured 2vCPU/4GB rebuild; prod env + ≥16-byte key + HTTPS + search (not admin) key + tenant tokens + firewall [D4]; backup runbook [D2]. Re-open triggers: typo/as-you-type requirement, validated CJK corpus, measured latency breach, faceting in contract [F2].
- **RustFS:** factory branch on `backend`; `S3Storage` via `aws-sdk-s3`/`object_store`, SHA-256 verify; presigned GET only with app authz first (matrix tests presigned GET/PUT at commit `1e6f5f1e`, verify at pinned tag); SNSD profile labeled non-production, UID `10001` volume fix; gated contract put/get/delete/head + strong read-after-write + last-writer-wins; no per-object ACL assumption [D9,D11]. Trigger: S3 clients, multi-node shared namespace, real erasure coding [F3].
- **PGMQ/Valkey:** choose semantics explicitly (PGMQ `read` at-least-once within VT, `pop` at-most-once; Pub/Sub lossy; Streams + `XACK`/`XAUTOCLAIM`); AOF `always` or accept ~1 s; DB catch-up regardless; pin image, prove redelivery via `read_ct`/VT, `read_ct` DLQ + `pgmq.metrics()` alerts, measure VACUUM. Triggers: retryable external side effects, tens of thousands/min, multi-instance fanout [F4].
- **OTLP:** replace phantom `0.22`; align generation (0.28 family per official example with `grpc-tonic`, or 0.33.x) [R11,D14]; `TraceLayer` first (zero new crates); then `SdkTracerProvider`+`service.name`+batch exporter+`TraceContextPropagator`+`shutdown()`, keep `fmt` unconditional; optional collector profile, OTLP ports internal only; no backend stack. OTLP 1.11.1/SemConv 1.45.0 make names/wire safe when opening [D12,D13].
- **Zitadel:** RP-first with `openidconnect 4.0`: discovery, exact `iss`, `aud`, `exp`, `nonce`, `alg` allowlist, `kid` only inside pinned `jwks_uri`, reject `jwk`/`jku`/`x5u`/`alg=none`; exact redirect allowlist; identity by `iss+sub`; 5 RBAC endpoints w/ negative cross-tenant tests; local logout per `jti`; RFC 7636 App.B vector. Then optional v4.20.3 OP + own PG + HTTPS + rotation + `end_session_endpoint` if needed; read v4 upgrade notes; cite OIDC specs as normative, OAuth2.1 as draft [F6,R8,R12].

## 5. Contrarian views

1. "RustFS is mature now — deferral stale." Fair (GA, 455 cases, presigned). Counter: maturity answers "can it speak S3," not "does one VPS benefit"; SNSD still no redundancy [D10].
2. "Valkey 9.1.3 RCE means adopt/patch." Non sequitur — Seed runs no Valkey; patch is optional-profile debt, not new scope [R2].
3. "OTLP Stable ⇒ exporter is conservative." Protocol Stable ≠ zero cost (collector+backend+shutdown); Coolify has no Compose charts, so no operator gain [D12,D19].
4. "Vendor comparison is marketing." True — used only as an admission; normative weight rests on hard RAM/thread docs and zero-hook code [D1,D2].
5. "Real IdP is mandatory security." RP verification is mandatory and mock-testable; external IdP changes deployment, not verification. Leaving the unverified base64-decode path *is* the defect [F6].
6. "OAuth 2.1 is current standard." No — draft-17, expires 2027-05-04 [R12].
7. "PG18 fixed FTS gaps." No native CJK/typo/facets; CJK re-open trigger stands [D16,D17].

## 6. Open questions (NV)

Q1 measured co-located RSS/CPU of PG+API+Meili rebuild on actual 2vCPU/4GB. Q2 VPS arch x86-64 vs arm64 (staging pins arm64). Q3 CJK corpus + PG vs Charabia recall. Q4 `pg_trgm` on short fields after PG18 collation change. Q5 FTS/GIN+`pg_trgm` reindexed after pg_upgrade on non-libc. Q6 `unaccent()` immutability vs generated `search_tsv`. Q7 crate pairings tracing-opentelemetry 0.29 vs 0.33.1 on Rust 1.98. Q8 Zitadel own-PG production footprint on same box. Q9 RustFS 1.0.6 still tests presigned GET/PUT at pinned tag. Q10 real external/retryable-notification requirement. Q11 multi-instance deployment planned. Q12 operators on `search`/`realtime` profiles [F2–F7].

## 7. Full sources

**All accessed 2026-11-16.** Releases/registries:
[R1] Meilisearch v1.57.0: https://api.github.com/repos/meilisearch/meilisearch/releases/latest
[R2] Valkey 9.1.3 (SECURITY): https://api.github.com/repos/valkey-io/valkey/releases/latest
[R3] Valkey 9.0.0 + blog: https://github.com/valkey-io/valkey/releases/tag/9.0.0 ; https://valkey.io/blog/introducing-valkey-9/
[R4] PGMQ v1.13.1: https://api.github.com/repos/pgmq/pgmq/releases/latest
[R5] RustFS 1.0.6: https://api.github.com/repos/rustfs/rustfs/releases/latest
[R6] OTel Collector v0.165.0: https://api.github.com/repos/open-telemetry/opentelemetry-collector/releases/latest
[R7] Prometheus 3.16.0: https://api.github.com/repos/prometheus/prometheus/releases/latest
[R8] Zitadel v4.20.3: https://api.github.com/repos/zitadel/zitadel/releases/latest
[R9] Coolify v4.7.5: https://api.github.com/repos/coollabsio/coolify/releases/latest
[R10] PG 18.7/19RC2: https://www.postgresql.org/about/news/postgresql-187-1712-1616-1520-1425-and-19-rc-2-released-3442/
[R11] opentelemetry-otlp 0.33.1: https://crates.io/api/v1/crates/opentelemetry-otlp
[R12] OAuth 2.1 draft-17: https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1

Official docs:
[D1] https://www.meilisearch.com/docs/resources/self_hosting/performance/ram_multithreading.md
[D2] https://www.meilisearch.com/docs/resources/comparisons/postgresql (vendor, interest not neutral)
[D3] https://www.meilisearch.com/docs/learn/async/asynchronous_operations
[D4] https://www.meilisearch.com/docs/resources/self_hosting/security/overview.md
[D5] https://www.meilisearch.com/docs/resources/help/language.md
[D6] https://pgmq.github.io/pgmq/latest/
[D7] https://valkey.io/topics/persistence/
[D8] https://docs.rustfs.com/en/installation
[D9] https://docs.rustfs.com/en/reference/s3-compatibility
[D10] https://docs.rustfs.com/zh/installation/linux/single-node-single-disk
[D-idx] https://docs.rustfs.com/llms.txt
[D11] https://docs.min.io/aistor/operations/software-lifecycle
[D12] https://opentelemetry.io/docs/specs/otlp/
[D13] https://opentelemetry.io/docs/specs/semconv/
[D14] https://opentelemetry.io/docs/languages/rust/exporters/
[D15] https://opentelemetry.io/docs/specs/otel/document-status/
[D16] https://www.postgresql.org/docs/18/release-18.html
[D17] https://www.postgresql.org/docs/current/pgtrgm.html
[D18] https://zitadel.com/docs/self-hosting/deploy/overview
[D19] https://coolify.io/docs/core/observability/monitoring/metrics

Flora-local (read-only): [F1] `docs/adr/0001-*.md`, `docs/adr/0002-*.md`; [F2] `…-seed-search-meilisearch-reconciliation.md`; [F3] `…-seed-files-rustfs.md`; [F4] `…-seed-notifications-pgmq-valkey.md`; [F5] `…-seed-infrastructure-otel-coolify.md`; [F6] `…-seed-oidc-identity-gap.md`; [F7] `…-seed-vps-profile-audit.md`; [F8] `docker-compose.yml`, `.env.example`, `Cargo.toml`, `.github/workflows/*.yml`.

26 primary sources; all H claims backed by primary artifacts; footprint figures explicitly NV. No other file modified.
