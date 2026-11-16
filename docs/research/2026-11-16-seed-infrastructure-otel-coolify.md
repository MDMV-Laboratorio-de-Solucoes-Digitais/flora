# Seed Infrastructure Research — OpenTelemetry + Coolify on a Cheap VPS

**Date:** 2026-11-16
**Type:** Research only — no source files were modified.
**Scope:** Minimal, cheap-VPS-safe observability for a Rust/Axum service (traces, metrics, logs, OTLP export, Prometheus-compatible `/metrics`, zero/low-overhead defaults) and Coolify deployment basics for a Docker Compose Rust app.
**Baseline assumption (explicit):** single node, ~2 vCPU / 4 GB RAM / SSD, low initial concurrency — the same envelope used by `docs/research/2026-11-16-seed-vps-profile-audit.md`.
**Parallel-agent boundary:** a parallel agent added the static `/metrics` endpoint in `flora-api/src/routes/health.rs` (commit `15192c8fa`, `feat(observability): add GET /metrics endpoint to flora-api`, already in HEAD at research time). That file was **read only**; nothing here reverts or changes it.

---

## 0) Conclusion first (the 80/20 answer)

1. **Do not ship an observability backend in the Seed profile.** No VictoriaMetrics, no Grafana, no Loki, no standalone Jaeger, no vendor SaaS. OTel itself is *not* a backend — storage and visualization are intentionally left to other tools [S1][S11].
2. **The cheapest correct default is "no SDK wired".** The OTel API has a no-op default when no global provider is set, and libraries that depend only on the API have no impact unless the SDK is imported [S5][S6]. Keep `otel_enabled = false` (already the Flora default) and keep the OTel crates out of any crate's dependency graph until a collector actually exists.
3. **What Flora has today is enough for Seed:** `tracing` fmt logs + `/health`, `/health/ready`, `/health/live` + the new static `/metrics` (`flora_api_up`, `flora_api_info`) + Coolify Sentinel host/container CPU/memory. Add per-request spans with `tower_http::trace::TraceLayer` (feature already enabled, zero new crates) before adding any OTel dependency.
4. **When (and only when) a collector exists, add one optional trace path:** fix the broken OTel dependency set first, then gate `SdkTracerProvider` + `tracing_opentelemetry::layer()` + `opentelemetry_otlp` behind `otel_enabled`, with a batch exporter and an explicit `service.name` resource. Keep `fmt` logging always on. Add the OTel *logs* bridge and a real Prometheus exporter only if a consumer is actually deployed.
5. **Coolify deploy for Flora is mostly already aligned:** the existing GHCR → deploy-webhook workflow matches Coolify's documented GitHub Actions pattern [S14]. What is missing is a **production Compose file that contains a `flora-api` service** — the current `docker-compose.yml` only defines local dependencies (Postgres/Valkey/Meilisearch), so it cannot be pointed at Coolify as-is.

---

## 1) Research method and language coverage

Primary sources were fetched directly (docs.rs, opentelemetry.io, coolify.io/docs, docs.docker.com). Translations of the OpenTelemetry Rust section were checked in the requested languages:

| Language | Result | Evidence |
|---|---|---|
| EN | Canonical pages available — used as primary basis | [S1]–[S9], [S12]–[S18] |
| pt-BR / pt | Translated Rust overview **and exporters** page available; terminology *rastros, métricas, logs* confirmed | [S19] |
| es | `/es/docs/languages/rust/` exists but body is still English (untranslated shell) | [S22] |
| de | `/de/docs/languages/rust/` and `/de/` returned 404 in this environment — **no German primary source retrieved** | NV-7 |
| fr | `/fr/docs/languages/rust/` exists but body is English (untranslated shell) | [S22] |
| zh-CN / zh | `/zh/docs/languages/rust/` translated: *链路, 指标, 日志* | [S20] |
| ja | `/ja/docs/languages/rust/` fully translated: *トレース, メトリクス, ログ* | [S21] |

Operator one-liners (authored translations, not sourced): *pt-BR* "sem coletor, sem exportação — padrão sem custo"; *es* "sin colector, sin exportación"; *zh* "无采集器则不导出，默认零开销"; *ja* "コレクタが無ければエクスポートしない（既定は no-op）".

---

## 2) Primary sources

| ID | Source |
|---|---|
| S1 | [OpenTelemetry Rust — overview](https://opentelemetry.io/docs/languages/rust/) (traces Beta, metrics/logs Stable) |
| S2 | [OpenTelemetry Rust — Getting Started](https://opentelemetry.io/docs/languages/rust/getting-started/) (three-signal example, providers, shutdown) |
| S3 | [OpenTelemetry Rust — Exporters](https://opentelemetry.io/docs/languages/rust/exporters/) (OTLP endpoint, `grpc-tonic`, `localhost:4317`) |
| S4 | [docs.rs `tracing-opentelemetry` 0.29.0](https://docs.rs/tracing-opentelemetry/0.29.0/tracing_opentelemetry/) (`OpenTelemetryLayer`, `otel.*` fields, deps on `opentelemetry`/`opentelemetry_sdk` 0.28) |
| S5 | [docs.rs `opentelemetry` 0.28 — `trace` module](https://docs.rs/opentelemetry/0.28.0/opentelemetry/trace/index.html) ("no-op default if this is not set"), `noop` module, batch processor guidance |
| S6 | [OpenTelemetry — Instrumentation concepts](https://opentelemetry.io/docs/concepts/instrumentation/) ("no impact on applications using that library, unless the OpenTelemetry SDK is imported") |
| S7 | [OpenTelemetry Collector overview](https://opentelemetry.io/docs/collector/) (when to use a collector; default OTLP exporters assume local collector) |
| S8 | [Collector quick start](https://opentelemetry.io/docs/collector/quick-start/) (`otel/opentelemetry-collector:0.165.0`, 4317/4318/55679) |
| S9 | [Collector benchmarks](https://opentelemetry.io/docs/collector/benchmarks/) (load tests exist; no single headline number — see NV-5) |
| S10 | [docs.rs `opentelemetry-prometheus` latest](https://docs.rs/opentelemetry-prometheus/latest/opentelemetry_prometheus/) (registry + `TextEncoder`, OTel → Prometheus text) |
| S11 | [docs.rs `opentelemetry-otlp` versions](https://docs.rs/crate/opentelemetry-otlp) + crates.io API `.../0.22.0/dependencies` → 404 (version series 0.17 → 0.25 → … → 0.33.1) |
| S12 | [docs.rs `tower_http::trace`](https://docs.rs/tower-http/latest/tower_http/trace/index.html) (`TraceLayer::new_for_http()`, spans with method/path/latency/status) |
| S13 | [Coolify — Docker Compose](https://coolify.io/docs/applications/builds/docker-compose) (Git Compose vs Compose Empty, env interpolation, domains, healthchecks, networking) |
| S14 | [Coolify — GitHub Actions](https://coolify.io/docs/applications/sources/github/actions) (GHCR build + authenticated deploy webhook, trigger only after checks pass) |
| S15 | [Coolify — Metrics](https://coolify.io/docs/core/observability/monitoring/metrics) (Sentinel; **no dedicated charts for Compose apps**; defaults 10/7/60) |
| S16 | [Coolify — Resource Limits](https://coolify.io/docs/applications/configuration/resource-limits) (CPU/mem fields; low limits kill apps or time out health checks) |
| S17 | [Coolify — Health Checks](https://coolify.io/docs/applications/configuration/health-checks) (Compose healthcheck wins; HTTP checks need `curl` inside container; Traefik unhealthy behavior) |
| S18 | [Docker Compose overview](https://docs.docker.com/compose/) (single YAML for multi-container services, networks, volumes; works in dev/staging/prod/CI) |
| S19 | [OTel Rust — pt](https://opentelemetry.io/pt/docs/languages/rust/) and [pt exporters](https://opentelemetry.io/pt/docs/languages/rust/exporters/) |
| S20 | [OTel Rust — zh](https://opentelemetry.io/zh/docs/languages/rust/) |
| S21 | [OTel Rust — ja](https://opentelemetry.io/ja/docs/languages/rust/) |
| S22 | [OTel Rust — es](https://opentelemetry.io/es/docs/languages/rust/) and [fr](https://opentelemetry.io/fr/docs/languages/rust/) (English body) |
| S23 | [Coolify docs index / llms.txt](https://coolify.io/docs/llms.txt) |

---

## 3) Flora cross-check (read-only)

| Area | Verified finding | Evidence |
|---|---|---|
| OTel config module | **No `flora-core/src/config/otel*` file exists.** OTel knobs live in `AppConfig`: `otel_enabled: bool` (default `false`) and `otel_endpoint: Option<String>` (default `None`). Loader is Figment `Env::prefixed("FLORA_").split("__")`. | `flora-core/src/config/app.rs:32-35,61-62`; `flora-core/src/config/mod.rs:6-12,46-51` |
| Env surface | `.env.example` documents the two knobs as commented examples only: `FLORA_APP__OTEL_ENABLED`, `FLORA_APP__OTEL_ENDPOINT=http://localhost:4317`. | `.env.example:64-66` |
| Dependency declarations | Root `Cargo.toml` declares `opentelemetry = "0.28"`, `tracing-opentelemetry = "0.29"`, `opentelemetry-otlp = "0.22"`, `opentelemetry_sdk = "0.28"` in `[workspace.dependencies]`. | `Cargo.toml:161-168` |
| **Unused / phantom deps** | No `opentelemetry*` identifier appears in any `*.rs` file; no crate uses `opentelemetry.workspace = true`. `Cargo.lock` contains **no** `opentelemetry` package entries (unused workspace deps are not resolved). | `grep` across `*.rs`; `Cargo.lock` |
| **Version skew (breaking if wired)** | `opentelemetry-otlp = "0.22"` is **not a published version** — docs.rs jumps 0.17 → 0.25 (crates.io API 404 for 0.22.0). Even a hypothetical 0.22 would pair with OTel 0.22, while `tracing-opentelemetry 0.29` requires `opentelemetry`/`opentelemetry_sdk` 0.28. Current generation as of 2026-10 is 0.33.1. | [S11], [S4] |
| Subscriber wiring | `main.rs` initializes only `EnvFilter` + `fmt::layer()`. No OTel layer, no `TraceContextPropagator`, no `Resource`/`service.name`, no provider shutdown. | `flora-api/src/main.rs:13-24` |
| Request spans | `tower-http` is declared with the `trace` feature, but `TraceLayer` is **not** in the router; only rate-limit and CORS layers are present. | `Cargo.toml:143`; `flora-api/src/routes/mod.rs:51-60` |
| Existing signal usage | Many structured `tracing::{info,warn,debug,error}!` calls across `flora-api`, `flora-core`, `flora-messaging`, `flora-search` — a ready-made source for both fmt logs and (later) an OTel logs bridge. | `grep` over `*.rs` (e.g. `flora-api/src/routes/tasks.rs:214`, `flora-api/src/routes/health.rs:43`) |
| `/metrics` (parallel agent) | Static Prometheus text: `flora_api_up 1`, `flora_api_info{service="flora-api"} 1`; content type `text/plain; version=0.0.4; charset=utf-8`; registered at `/metrics`; advertised in `/llms.txt`. **Read-only; not modified.** | `flora-api/src/routes/health.rs:54-120`; `flora-api/src/routes/mod.rs:26` |
| Health endpoints | `/health` (capability hints), `/health/ready` (DB `SELECT 1`, 503 on failure), `/health/live`. | `flora-api/src/routes/health.rs:23-52` |
| Compose | Lean core = Postgres only; `realtime`/`search` are profiles. No collector service. Dependencies have `mem_limit`/`cpus` hints; API is not containerized in this file. | `docker-compose.yml:1-9,11-87` |
| Image | `rust:1.98-slim` builder → `debian:trixie-slim` runtime with `ca-certificates` **and `curl`**, `EXPOSE 3000`. `curl` present ⇒ Coolify HTTP healthchecks will work. | `Dockerfile:1-11`; [S17] |
| CI/CD | `rust.yml` (build/test + PG18 service), `ci.yml`, `docker-staging.yml` builds `linux/arm64` GHCR image on `dev` and POSTs authenticated Coolify deploy webhook. This matches Coolify's documented pattern. | `.github/workflows/*.yml`; [S14] |

**Net:** Flora has configuration *shape* and dependency *declarations* for OTel, but **no functioning exporter**. The static `/metrics` is an intentional minimal uptime signal, not OTel-derived instrumentation.

---

## 4) Technical findings — OpenTelemetry

### 4.1 Signals, status, and the "free" default

- Rust signal maturity: **traces Beta, metrics Stable, logs Stable** [S1]. Treat trace APIs as potentially breaking across minor versions.
- The API is inert without the SDK: *"there is a no-op default if this is not set by your application"* [S5] and *"no impact on applications using that library, unless the OpenTelemetry SDK is imported"* [S6]. This is the primary mechanism for zero-cost defaults.
- Practical consequence: instrumenting code with `tracing` is free today; installing SDK crates but never initializing providers still costs compile time/binary size, even though calls remain no-ops. **Do not add SDK crates to any crate's `[dependencies]` until `otel_enabled` can actually be true.**

### 4.2 Traces — the only signal worth wiring first

- `tracing-opentelemetry` bridges `tracing` spans into OTel: `tracing_opentelemetry::layer().with_tracer(tracer)` combined with `Registry`, and exports via `SdkTracerProvider` [S4].
- Reserved span fields: `otel.name`, `otel.kind`, `otel.status_code`, `otel.status_message`; semantic-convention names such as `url.full` pass through [S4].
- `TraceLayer::new_for_http()` already creates per-request spans with method, path, latency and status [S12]; because Flora enables the `trace` feature, this needs **no new dependency**. It is the highest-value, lowest-risk trace step.
- Provider should use a **batch exporter** (`with_batch_exporter`) — recommended over per-span export for async runtimes [S3][S5].
- Context propagation for inbound HTTP: `global::set_text_map_propagator(TraceContextPropagator::new())` appears in the official exporter example [S3].

### 4.3 Metrics — two different paths, don't conflate them

| Path | What it gives | Cost / caveat |
|---|---|---|
| Current static `/metrics` | `flora_api_up`, `flora_api_info` | ~0 cost, no dependencies; **not** real instrumented metrics |
| `opentelemetry-prometheus` | OTel instruments → `prometheus::Registry` → `TextEncoder` text exposition [S10] | Adds `prometheus` + exporter crates; must be version-aligned with the rest of the OTel family |
| OTLP metrics → collector | Full OTel metric model, no lossy conversion [S3] | Requires collector; Prometheus can also receive OTLP directly [S3] |

The static endpoint already satisfies the *format* contract (Prometheus text 0.0.4). Upgrading it to a real OTel-backed registry is an **optional later step**, gated on an actual scraper existing.

### 4.4 Logs

- OTel Rust has **no end-user logging API**; it bridges existing frameworks. For `tracing`, the documented bridge is `opentelemetry-appender-tracing::OpenTelemetryTracingBridge`, wired into `SdkLoggerProvider` [S2].
- Do **not** confuse this with `tracing-opentelemetry`'s `MetricsLayer` (events → metrics) [S4].
- Skip log export in Seed unless a collector is forwarding logs somewhere readable; otherwise it adds CPU and a second exporter for no operator benefit.

### 4.5 OTLP and the collector

- Default OTLP exporter endpoint is `http://localhost:4317` (gRPC, `grpc-tonic` feature); collector quick start also exposes 4318 (HTTP) and 55679 (ZPages debug UI) [S3][S8].
- "Using the Collector in production environments is a best practice", but for "development or small-scale environment you can get decent results without a collector" [S7]. The collector's value here is **batching, retries, filtering and backend flexibility**, not visualization.
- Core distribution image: `otel/opentelemetry-collector:0.165.0` [S8]. Bind OTLP ports to the internal Docker network, never publish publicly.
- Avoid heavy stacks: Grafana/VictoriaMetrics/Loki are *backends*, not OTel components [S1][S11]. A collector with a `debug` exporter plus Coolify's built-in logs/metrics covers Seed diagnosis.

---

## 5) Coolify deployment findings

- **Compose paths** [S13]: Git-based Compose build pack → *Application* (branch deploys, webhooks, previews); "Docker Compose Empty" → *user-defined Service* (no Git automation). Flora should use the **Git-based Application** path with a dedicated production Compose file.
- **Current Compose is not deployable as-is**: it has no `flora-api` service (only dependency services bound to `127.0.0.1`). Coolify warns against publishing ports unless directly reachable [S13]; use its generated domain with the internal port suffix, e.g. `https://flora.example.com:3000` [S13].
- **Env interpolation** [S13]: `${VAR}` creates an editable Coolify variable; `${VAR:-default}` gives a safe default; `${VAR:?}` makes it required and blocks deploy until set. Use `${FLORA_DATABASE__POSTGRES_URL:?}` and `${FLORA_APP__JWT_SECRET:?}`; expose OTel as `${FLORA_APP__OTEL_ENABLED:-false}` / `${FLORA_APP__OTEL_ENDPOINT:-http://otel-collector:4317}`. Coolify "magic" `SERVICE_PASSWORD_64_*` values can generate/reuse DB credentials without committing them [S13].
- **CI deploy** [S14]: build/push image → authenticated `POST` to deploy webhook with `Bearer` API token; keep the webhook **after** all checks. Flora's `docker-staging.yml` already does exactly this. Verify the VPS CPU architecture: the workflow pins `linux/arm64` and `ubuntu-24.04-arm` — an x86-64 VPS cannot pull that image (NV-6).
- **Health checks** [S17]: Compose services use their Compose `healthcheck` (the dashboard Healthcheck page does not apply); HTTP checks run from **inside** the container and need `curl`/`wget` — Flora's runtime image has `curl`. Use `/health/ready` for readiness and `/health/live` for liveness, with `start_period` covering startup migrations. With Traefik, unhealthy containers are removed from routing and all-unhealthy can surface as `404`/`No available server`.
- **Resource limits** [S16]: Coolify exposes CPU limit/set/weight and memory reservation/limit fields; low hard limits cause OOM kills or health-check timeouts. Start from measured usage. Existing compose hints (PG 1g/1 CPU, Valkey 384m/0.5, Meili 768m/0.75) are a starting point, not API sizing.
- **Monitoring reality for Compose** [S15]: Sentinel provides server CPU/memory/disk/network and a fleet "Hottest containers" table, but **dedicated per-resource charts are not available for Docker Compose applications**. Sampling default 10 s, history retention 7 days, push interval 60 s; data lives under `/data/coolify/sentinel`. This is another reason not to add a separate Prometheus/Grafana stack in Seed.

---

## 6) Minimal 80/20 Seed slice (recommendations, not implemented)

### Step 0 — Keep the free baseline (default profile)
- Keep `otel_enabled = false`, no OTel SDK in any crate's dependency list, no collector container.
- Keep `fmt` logs, `/health*`, static `/metrics` (parallel agent), and Coolify Sentinel for host/container metrics.
- Optional, no new crates: add `TraceLayer::new_for_http()` to the Axum router for request spans and set `RUST_LOG` via the existing `EnvFilter`.

### Step 1 — Repair the latent OTel dependency set (precondition)
- Remove or replace the nonexistent `opentelemetry-otlp = "0.22"`.
- Choose **one** generation and align all four crates. Two viable choices:
  - Conservatively keep the declared 0.28 family: `opentelemetry = 0.28`, `opentelemetry_sdk = 0.28`, `tracing-opentelemetry = 0.29`, `opentelemetry-otlp = 0.28` [S4][S11].
  - Or move the whole family to current 0.33.x (published 2026-10-09) with a matching `tracing-opentelemetry` release; verify pairings in docs.rs before editing [S11].
- Keep them in `[workspace.dependencies]` only; do not pull them into `flora-api` until Step 2 is ready.

### Step 2 — One opt-in trace path (only with a collector)
- In `main.rs`, when `config.app.otel_enabled` and `otel_endpoint` is `Some`: build `SdkTracerProvider` with `Resource` `service.name = "flora-api"`, a batch OTLP span exporter, `TraceContextPropagator`, and `tracing_opentelemetry::layer()`. Keep the `fmt` layer unconditionally.
- Add `tracing_opentelemetry::OpenTelemetrySpanExt` propagation only where inbound headers are available; avoid instrumenting every repository call.
- Call `provider.shutdown()` on graceful exit [S2].
- **Acceptance:** with `otel_enabled=false`, no connection attempts to 4317; with it true and collector up, request spans appear; no panic if collector is down (batch exporter + shutdown path).

### Step 3 — Collector as an optional Compose profile, not a default service
- Add a separate `observability` profile: `otel/opentelemetry-collector` (core) with OTLP gRPC/HTTP receivers on the internal network and a `debug` exporter (or Prometheus exporter + scrape only if a scraper exists).
- Constrain it (planning values, NV-5): start around `mem_limit: 128–256m`, `cpus: "0.25"`, no published OTLP ports; verify with `docker stats`.
- Explicitly out of Seed scope: VictoriaMetrics, Grafana, Loki, Jaeger UI, vendor SaaS, OTel log export.

### Step 4 — Coolify production slice
- New production Compose (e.g. `deploy/coolify/docker-compose.yml`) with a `flora-api` service using `image: ghcr.io/<org>/flora:staging` (not `build:`), `${...:?}` secrets, and the optional collector profile.
- Set the Coolify domain with internal port `:3000`; add a Compose healthcheck `curl -fsS http://localhost:3000/health/ready` with `start_period` for migrations [S13][S17].
- Set measured CPU/memory limits after first deploy [S16]; keep the deploy webhook gated behind CI checks [S14]; verify image architecture (NV-6).

### What to skip — and why

| Skip | Reason |
|---|---|
| VictoriaMetrics / Grafana / Loki | Backends, not OTel; Coolify Sentinel + runtime logs already cover Seed-scale need [S1][S15] |
| Prometheus scrape of a real OTel exporter | Static `/metrics` already exposes the format; add real instruments only with a scraper present |
| OTel logs pipeline | Second exporter + collector load with no Seed consumer; existing `tracing` fmt logs are sufficient |
| Collector in the default profile | Adds a moving part; "small-scale environment can get decent results without a collector" [S7] |
| Publishing OTLP 4317/4318 to the host | Unauthenticated telemetry ingestion; use the internal network only |

---

## 7) NV / limitations

- **NV-1:** OTel dependency resolution was not executed (`cargo metadata`/`cargo update` not run here). The `0.22` conclusion is based on docs.rs version history and a crates.io 404 for that version; verify with a live registry before editing `Cargo.toml` [S11].
- **NV-2:** No runtime telemetry, `docker stats`, or collector benchmark was measured in this checkout. Memory/CPU suggestions are planning estimates, not measurements; Collector benchmarking data exists but is a dashboard of commit-level results, not a single canonical footprint number [S9].
- **NV-3:** The parallel agent's `/metrics` implementation was in HEAD at research time (`15192c8fa`) but may be extended by other agents. Re-read `flora-api/src/routes/health.rs` before relying on the exact metric names.
- **NV-4:** Coolify instance version/configuration (API access enabled, Sentinel version, proxy type, destination network) is unknown; Compose domain/healthcheck behavior can differ from the docs' current version [S13][S15].
- **NV-5:** Collector right-sizing (128–256 MB / 0.25 CPU) is an unverified starting point; validate under representative load and adjust using Coolify's "Hottest containers" table.
- **NV-6:** `docker-staging.yml` targets `linux/arm64`; the target VPS architecture was not verified. An x86-64 host would need `linux/amd64` or a multi-arch build.
- **NV-7:** No German primary source was retrievable (`/de/` 404 in this environment); es/fr Rust pages were English shells. Those languages are reported as coverage gaps rather than inferred from translated primary text.
- **NV-8:** `otel_enabled`/`otel_endpoint` validation is not implemented in `AppConfig::validate()`; an invalid endpoint would fail at exporter construction, not at config load.
