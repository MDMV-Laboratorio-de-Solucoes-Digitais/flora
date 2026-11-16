# Honest OpenTelemetry Wiring for Flora (Rust/Axum) — 2026-11-16-026

- **Type:** research only. No source, manifest, config, or test files were changed.
- **Scope:** one nested repo (`projetos/Arquivados/flora`); HEAD `0517a3a50`, working tree clean at research time.
- **Question:** what is the *correct, minimal* OpenTelemetry (OTel) wiring for Flora's Axum API when needed, with zero cost when no collector is configured — and is the prior claim that `opentelemetry-otlp = "0.22"` does not exist true?
- **Method:** primary sources only (opentelemetry.io docs/spec, docs.rs, crates.io version history). Read-only Flora cross-check against `Cargo.toml`, `flora-core/src/config/app.rs`, `flora-api/src/main.rs`, `.env.example`, `flora-api/src/routes/health.rs`, `flora-api/src/routes/mod.rs`. Complements `docs/research/2026-11-16-seed-infrastructure-otel-coolify.md` (same topic, broader Coolify scope); this note is the version/wiring-focused follow-up.

---

## 0) TL;DR — the 80/20 answer

1. **The `0.22` claim is CONFIRMED.** `opentelemetry-otlp` has no published `0.22.x`: the published series jumps **0.17.0 (2024-07-15) → 0.25.0 (2024-09-09)** and then aligns with the `opentelemetry` crate generation through **0.33.1 (2026-10-09)** [S2][S3]. (Do not confuse it with `opentelemetry_sdk`, which *did* publish 0.22.x in 2024 [S4].)
2. **Today Flora has no live OTel at all** — no OTel identifier in any `*.rs`, no crate consumes the workspace deps, and `Cargo.lock` contains **zero** `opentelemetry*` packages. The four declarations in `Cargo.toml:164-167` are *phantom*: they cost nothing at build/runtime today, but are a correctness trap for the next person who enables one [F1][F2].
3. **Minimal correct choice (recommended):** keep `tracing` fmt + `/health*` + static `/metrics` as the baseline; **remove the four dead OTel declarations now** (or, if kept for a planned slice, align them to one generation: `opentelemetry 0.28` + `opentelemetry_sdk 0.28` + `tracing-opentelemetry 0.29` + **`opentelemetry-otlp 0.28`**). Add `tower_http::trace::TraceLayer` to the router — the `trace` feature is already enabled, so **no new crate, no exporter, no collector** [F1][F6][S11].
4. **Wire full OTLP only behind `otel_enabled` and only when a collector/backend actually exists.** Use a batch exporter, an explicit `service.name`, `TraceContextPropagator`, and an explicit `shutdown()`; keep the `fmt` layer unconditional [S5][S6][S14].
5. **Zero cost when unconfigured** comes from the OTel API being a no-op facade until an SDK is imported/set, *not* from the config flag itself. Gating with `otel_enabled` avoids even constructing the SDK; keeping SDK crates out of a crate's `[dependencies]` avoids compiling them [S9][S10][S12].

**Decision in one line:** *remove the dead deps → add `TraceLayer` → stop. Only build the OTLP path when a receiver exists; never put a collector in the Seed profile.*

---

## 1) Flora state — read-only cross-check

| Area | Verified state | Evidence |
|---|---|---|
| Dependency declarations | Root `[workspace.dependencies]`: `opentelemetry = "0.28"`, `tracing-opentelemetry = "0.29"`, `opentelemetry-otlp = "0.22"`, `opentelemetry_sdk = "0.28"`, next to `tracing = "0.1"`, `tracing-appender = "0.2"`, `tracing-subscriber` (env-filter/json/ansi) | `Cargo.toml:161-168` [F1] |
| Resolved graph | `Cargo.lock` has **0** packages matching `^name = "opentelemetry"`; no member crate uses `opentelemetry.workspace = true`. The 0.22 defect is therefore latent, not a current build failure | `grep` over all `Cargo.toml` + `Cargo.lock` (local, 2026-11-16) [F2] |
| Config knobs | `AppConfig { otel_enabled: bool /* default false */, otel_endpoint: Option<String> /* default None */ }`; `validate()` does **not** validate either field | `flora-core/src/config/app.rs:32-35,61-62,73-93` [F3] |
| Runtime init | `main.rs` builds `registry() + EnvFilter + fmt::layer()` only. No provider, no OTel layer, no propagation. `otel_enabled`/`otel_endpoint` are never read | `flora-api/src/main.rs:17-24` [F4] |
| Config surface | `.env.example` documents both OTel vars as commented examples; endpoint example is `http://localhost:4317` (i.e. **gRPC** OTLP) | `.env.example:64-66` [F5] |
| Request spans | `tower-http = { version = "0.7.0", features = ["cors", "trace"] }` is declared, but the router only adds rate-limit + extension + CORS layers — **no `TraceLayer`** | `Cargo.toml:143`; `flora-api/src/routes/mod.rs:51-60` [F6] |
| Metrics | `/metrics` returns a **static** Prometheus text payload (`flora_api_up`, `flora_api_info`), advertised in `/llms.txt`. Authored by another agent — read only, not modified here | `flora-api/src/routes/health.rs:64,96-120` [F7] |
| Prior docs | ADR-0001, `OBJETIVO.md` §P1, release 023 and assessment 024 already record "honest OTel" / "0.22 does not exist" as an open item — this note supplies the primary-source verification | local docs (read only) [F8] |

**Interpretation:** adding OTel SDK crates to `flora-api` today would *increase* binary size and compile time (tonic/reqwest/prost/tokio streams) while producing no spans unless a provider is installed. The honest baseline is `tracing` + `/metrics`, which is already sufficient for a 2 vCPU/4 GB Seed box.

---

## 2) Version & compatibility matrix (primary-verified, 2026-11-16)

| Crate | Flora declares | Published reality | Pairing rule | Source |
|---|---|---|---|---|
| `opentelemetry` | `0.28` | current **0.33.1**; MSRV 1.75 | API facade + no-op default | [S1][S3] |
| `opentelemetry_sdk` (crate name has `_`; repo dir is `opentelemetry-sdk`) | `0.28` | current **0.33.1** | must match `opentelemetry` minor | [S3][S4] |
| `tracing-opentelemetry` | `0.29` | current **0.34.0** (2026-09-23) | **0.29 → otel/api/sdk 0.28**; **0.34 → otel/api 0.33** | [S6][S7][S8] |
| `opentelemetry-otlp` | **`0.22` (invalid)** | current **0.33.1**; published: 0.17 → 0.25 → 0.26…0.33 | from 0.25 onward aligns 1:1 with `opentelemetry` minor | [S2][S3] |
| `tower-http` | `0.7.0` (trace feature) | docs.rs latest 0.7.1 | `TraceLayer` requires only `tracing`; independent of OTel | [S11] |

### 2.1 The `0.22` verification, explicitly

- docs.rs version list for `opentelemetry-otlp`: `… 0.17.0 (2024-07-15), 0.25.0 (2024-09-09), 0.26.0 … 0.33.1 (2026-10-09)`. **No 0.18–0.24** [S2].
- crates.io version history (same series and dates), plus the 0.28.0 changelog stating "update `opentelemetry` dependency version to 0.28" and "Starting with this version [0.25], this crate will align with `opentelemetry` crate on major,minor versions" [S3][S9].
- Direct `crates.io/api/v1/crates/...` fetch returned no body in this sandbox; verification therefore rests on two primary registries' rendered version histories (docs.rs + crates.io). A live `cargo add opentelemetry-otlp@0.22 --dry-run` was **not** run — see NV-1.
- Root cause of the confusion: `opentelemetry_sdk` **has** a 0.22.0/0.22.1 (Feb 2024), and `opentelemetry-otlp` 0.15 (Feb 2024) depended on `opentelemetry 0.22` — so "0.22" exists *near* the family, but not as an `opentelemetry-otlp` version [S4][S9].

### 2.2 If wiring is chosen, pick ONE generation

- **Conservative (matches official docs and Flora's other three declarations):** `opentelemetry 0.28`, `opentelemetry_sdk 0.28` (add feature `rt-tokio` for a Tokio batch span processor), `tracing-opentelemetry 0.29`, `opentelemetry-otlp 0.28` with `features = ["grpc-tonic"]` if using gRPC/4317 [S5][S6][S9][S13].
- **Modern:** the whole 0.33.1 family with `tracing-opentelemetry 0.34.0`. Note the official "Getting Started"/"Exporters" pages still show the 0.28 generation, so 0.33 examples must be checked against docs.rs of the exact versions — see NV-2 [S1][S5][S8].
- **Never mix** e.g. `tracing-opentelemetry 0.29` with `opentelemetry 0.33`: 0.29 pins `^0.28.0` for both API and SDK [S6].

---

## 3) Why `TraceLayer` works with no exporter — and what "zero cost" really means

- `tower_http::trace::TraceLayer::new_for_http()` wraps each request in a `tracing` span with method/path/latency/status and emits `on_request`/`on_response`/`on_failure` events. With `tracing_subscriber::fmt::init()` and `RUST_LOG=tower_http=trace` these appear in logs — **no OTel crate, no provider, no network socket involved** [S11].
- Honest wording: this is **"no-exporter / no-new-crate"**, not literally zero CPU. One span per request is created; the existing `EnvFilter` (`tower_http=debug` is Flora's default) controls whether it is enabled. Call it *minimal-cost request tracing* [F4][S11].
- The OTel zero-cost story is separate and comes from the API/SDK split:

  > "there is a no-op default if this is not set by your application" — docs.rs `opentelemetry::trace` [S10]

  > "Libraries can leverage the OpenTelemetry API as a dependency, which will have no impact on applications using that library, unless the OpenTelemetry SDK is imported." — OTel instrumentation concepts [S12]

  Consequently: instrumenting with `tracing` is free today; installing SDK crates but never initializing providers still costs compile time and binary size. **Do not pull the SDK into any crate's `[dependencies]` until `otel_enabled` can actually be true** [S1][S12].

---

## 4) Minimal correct OTLP wiring (illustrative only — NOT applied)

Precondition: a collector/backend exists and `config.app.otel_enabled && config.app.otel_endpoint.is_some()`. Use the 0.28 generation (matches Flora's other declarations and the official guide) [S5][S6]:

```toml
# [workspace.dependencies] — only if the opt-in slice is actually implemented
opentelemetry        = "0.28"
opentelemetry_sdk    = { version = "0.28", features = ["rt-tokio"] }   # batch processor on Tokio
tracing-opentelemetry = "0.29"
opentelemetry-otlp   = { version = "0.28", features = ["grpc-tonic"] } # for 4317
```

```rust
// In main.rs, only when otel_enabled + endpoint present.
let exporter = opentelemetry_otlp::SpanExporter::builder()
    .with_tonic()                      // gRPC; endpoint defaults to http://localhost:4317
    .with_endpoint(endpoint)           // from config.app.otel_endpoint
    .build()?;
let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
    .with_resource(
        opentelemetry_sdk::Resource::builder()
            .with_service_name("flora-api")
            .build(),
    )
    .with_batch_exporter(exporter)     // prefer batch over simple on an async server
    .build();
opentelemetry::global::set_text_map_propagator(
    opentelemetry_sdk::propagation::TraceContextPropagator::new(),
);
opentelemetry::global::set_tracer_provider(provider.clone());
let telemetry = tracing_opentelemetry::layer().with_tracer(provider.tracer("flora-api"));
tracing_subscriber::registry()
    .with(EnvFilter::from_default_env())   // existing filter
    .with(tracing_subscriber::fmt::layer()) // keep fmt ALWAYS on
    .with(telemetry)
    .init();
// ... on graceful shutdown:
provider.shutdown()?;
```

Correctness notes grounded in sources:

- `grpc-tonic` is **not** in `opentelemetry-otlp` 0.28 defaults (defaults are `http-proto` + `reqwest-blocking-client` + signals) — gRPC needs the explicit feature; the official guide uses `features = ["grpc-tonic"]` + `.with_tonic()` [S5][S9].
- The endpoint value in `.env.example` is `:4317`, i.e. gRPC. The OTLP spec defaults are **gRPC `http://localhost:4317`** and **HTTP `http://localhost:4318`**, with `OTEL_EXPORTER_OTLP_PROTOCOL` defaulting to `http/protobuf` and env vars `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, `OTEL_EXPORTER_OTLP_TIMEOUT` (10 s). If HTTP is chosen instead, change the documented port and drop `grpc-tonic` [F5][S9][S14].
- Prefer a **batch** exporter: the OTel API docs recommend it for async runtimes ("more efficient" than sending each span as it ends) [S10]. `rt-tokio` is a non-default SDK feature in 0.28 [S13].
- The provider must be kept alive and shut down explicitly (`SdkTracerProvider::shutdown()`); dropping the last reference also triggers shutdown [S5][S15]. With a batch exporter, a down collector must not crash the API — verify this acceptance criterion under test (NV-3).
- Do **not** add an OTel *logs* bridge or a Prometheus exporter by default. OTel Rust has no end-user logging API (it bridges `tracing` via `opentelemetry-appender-tracing`), and `tracing-opentelemetry`'s `MetricsLayer` is a different mechanism (events → metrics) [S1][S5].
- Add propagation (`OpenTelemetrySpanExt`) only where inbound headers exist (Axum extractor/middleware); avoid instrumenting every repository call [S6].

---

## 5) When to stay with `tracing` + `/metrics` only

Stay on the baseline when **all** of these hold (Flora Seed does):

- no receiver/backend is deployed or contracted; "in a development or small-scale environment you can get decent results without a collector" [S16];
- host/container resource data is available from Coolify Sentinel, and process health from `/health`, `/health/ready`, `/health/live` + static `/metrics` [F7];
- the team cannot own collector upgrades, sampling, retention, and PII filtering — the Collector's value is retries/batching/encryption/sensitive-data filtering, which is real work, not free magic [S16];
- traces (the signal in question) remain **Beta** in OTel Rust while metrics/logs are Stable [S1].

**Trigger to revisit:** a concrete consumer (e.g. a deployed OTLP backend plus a grafana/Jaeger viewer) exists, *and* someone owns the collector. Then follow §4. No consumer ⇒ no collector, no SDK, no new crates.

---

## 6) Keeping the Seed profile free of heavy collector stacks

- Keep `otel_enabled = false` as the default [F3]; keep the OTel crates as **workspace declarations only** (or remove them) — do not add them to any member's `[dependencies]` [F2][S12].
- Keep `TraceLayer` as the only request-tracing addition (feature already present) [F6][S11].
- If a collector is eventually needed, add it as a **separate optional compose profile** (`observability`), bind OTLP 4317/4318 to the internal network only, and start with a constrained `debug`/OTLP pipeline. Never publish OTLP ports to the host [S16].
- Out of Seed scope unless a consumer exists: VictoriaMetrics, Grafana, Loki, Jaeger UI, OTel log export, real Prometheus exporter behind `/metrics` [S1][S16].
- Rust 1.98 toolchain satisfies the OTel 0.33 MSRV of 1.75 — version choice is a compatibility question, not a compiler question [S3].

---

## 7) Multilingual verification (requested EN / pt-BR / es / de / fr / zh-CN / ja+)

Primary OTel Rust landing pages were fetched per locale; status table (Traces Beta, Metrics/Logs Stable) is identical in genuinely translated pages [S17]–[S22].

| Locale | Page | Translation status of the Rust overview |
|---|---|---|
| EN | `/docs/languages/rust/` | Source text [S1] |
| pt-BR | `/pt/docs/languages/rust/` | **Genuinely translated** — "Rastros / Métricas / Logs", "Estado e Lançamentos" [S17] |
| es | `/es/docs/languages/rust/` | Shell only (body in English) [S18] |
| de | `/de/docs/languages/rust/` | **404** in this environment; `/de/docs/languages/` also 404 — no primary source retrievable (NV-4) |
| fr | `/fr/docs/languages/rust/` | Shell only (body in English) [S19] |
| zh-CN | `/zh/docs/languages/rust/` | **Genuinely translated** — "链路 / 指标 / 日志", "状态以及版本发布" [S20] |
| ja | `/ja/docs/languages/rust/` | **Genuinely translated** — "トレース / メトリクス / ログ", "ステータスとリリース" [S21] |

Key-term agreement across translated pages: traces are Beta; metrics/logs Stable; the crate list is untranslated in every locale. Technical citations in this note remain English-first because that is where the authoritative code-level text lives; no non-English source was used to infer a version or API fact.

---

## 8) The three options, ranked (80/20)

| Option | Effort | Benefit | Risk / cost | Verdict |
|---|---|---|---|---|
| **A. Remove the four dead declarations (keep `tracing` + `/metrics`)** | minutes | Removes a broken version trap; no behaviour change (deps are not in `Cargo.lock`) | none | **Do now** |
| **B. Add `TraceLayer::new_for_http()` to the router** | ~1 line + log-level check | Per-request method/path/latency/status spans in existing logs; no new crate, no exporter | one span/request; guard with existing `EnvFilter` | **Do now (highest value)** |
| **C. Full OTLP behind `otel_enabled`** | ~1 file + aligned deps + shutdown path + a receiver | Cross-service traces, correlation | tonic/reqwest/prost compile+binary cost; Beta traces; needs a collector owner | **Defer until a collector exists** |

Recommended sequence: **A + B now; C only on a documented trigger.** If A is rejected because a future slice is already committed, then the minimum repair is `opentelemetry-otlp = "0.28"` (or a full aligned 0.33.1 move) — but keep those deps out of member crates until C is implemented [S2][S3][S6].

---

## 9) NV / limitations

- **NV-1:** no live registry resolution (`cargo metadata`, `cargo add --dry-run`, `cargo update`) was executed. The `0.22` conclusion rests on docs.rs + crates.io primary version histories; direct crates.io API fetch produced no body in this sandbox. Verify before editing `Cargo.toml` [S2][S3].
- **NV-2:** official OTel Rust "Getting Started"/"Exporters" pages still document the 0.28 generation while crates.io latest is 0.33.1. If the 0.33 path is chosen, re-verify all four version pairings and API signatures against exact docs.rs pages (e.g. `tracing-opentelemetry 0.34 ↔ opentelemetry 0.33`) [S1][S5][S8].
- **NV-3:** the §4 snippet is **illustrative, not compile-verified**. In particular the exact `with_batch_exporter` signature/feature for the chosen generation (`rt-tokio` in 0.28) and the failure behaviour with a down collector must be validated in a build/test. No telemetry was emitted or measured here [S5][S13][S15].
- **NV-4:** no German primary page was retrievable (`/de/docs/languages/rust/` and `/de/docs/languages/` both 404); es/fr pages are English shells. These are reported as coverage gaps, not inferred translations [S18][S19].
- **NV-5:** `AppConfig::validate()` does not validate `otel_enabled`/`otel_endpoint`; a malformed endpoint would fail during exporter construction, not config load. No config code was changed [F3].
- **NV-6:** the `.env.example` endpoint (`http://localhost:4317`) encodes gRPC, while the OTLP spec's default protocol is `http/protobuf` on 4318. A future wiring must choose one transport explicitly and update the example [F5][S14].
- **NV-7:** no collector footprint, CPU, or memory numbers were measured for this 2 vCPU/4 GB VPS. Collector sizing remains an unverified planning estimate; "small-scale without a collector" is a spec statement, not a benchmark [S16].
- **NV-8:** `/metrics` is static and was authored by another agent; its exact metric names may change. Re-read `flora-api/src/routes/health.rs` before relying on them. The file was read only [F7].

---

## 10) Sources (primary)

| ID | Source | Claim used |
|---|---|---|
| S1 | [OpenTelemetry Rust overview](https://opentelemetry.io/docs/languages/rust/) | Traces Beta / metrics+logs Stable; crate list; no logging API |
| S2 | [docs.rs `opentelemetry-otlp` versions](https://docs.rs/crate/opentelemetry-otlp) | no 0.22; 0.17 → 0.25 → … → 0.33.1 |
| S3 | [crates.io `opentelemetry-otlp` version history](https://crates.io/crates/opentelemetry-otlp/versions) + [latest 0.33.1 release notes](https://docs.rs/crate/opentelemetry-otlp/latest/source/CHANGELOG.md) | published series, dates, MSRV, alignment from 0.25 |
| S4 | [crates.io `opentelemetry_sdk`](https://crates.io/crates/opentelemetry_sdk) / [docs.rs 0.28 features](https://docs.rs/crate/opentelemetry_sdk/0.28.0/features) | 0.22.x exists for SDK (not exporter); SDK 0.33.1 current |
| S5 | [OTel Rust Exporters](https://opentelemetry.io/docs/languages/rust/exporters/) | `grpc-tonic`, `.with_tonic()`, `localhost:4317`, `Resource::builder().with_service_name`, batch exporter, propagator, shutdown |
| S6 | [docs.rs `tracing-opentelemetry` 0.29.0](https://docs.rs/tracing-opentelemetry/0.29.0/tracing_opentelemetry/) | 0.29 pins otel/sdk `^0.28.0`; layer/bridge API; `otel.*` fields |
| S7 | [docs.rs `tracing-opentelemetry` 0.34.0](https://docs.rs/tracing-opentelemetry/0.34.0/tracing_opentelemetry/) | latest pins `opentelemetry ^0.33.0` |
| S8 | [docs.rs crate page `tracing-opentelemetry`](https://crates.io/crates/tracing-opentelemetry) | 0.34.0 published 2026-09-23 |
| S9 | [docs.rs `opentelemetry-otlp` 0.28.0 CHANGELOG](https://docs.rs/crate/opentelemetry-otlp/0.28.0/source/CHANGELOG.md) | removal of `grpc-tonic` from defaults; 0.25 alignment; 10 s timeout |
| S10 | [docs.rs `opentelemetry` 0.28 `trace` module](https://docs.rs/opentelemetry/0.28.0/opentelemetry/trace/index.html) | "no-op default if this is not set"; batch processor for async runtimes |
| S11 | [docs.rs `tower_http::trace`](https://docs.rs/tower-http/latest/tower_http/trace/index.html) | `TraceLayer::new_for_http()`, spans/fields, `RUST_LOG=tower_http=trace`, no OTel dependency |
| S12 | [OTel instrumentation concepts](https://opentelemetry.io/docs/concepts/instrumentation/) | "no impact … unless the OpenTelemetry SDK is imported" |
| S13 | [docs.rs `opentelemetry_sdk` 0.28.0 features](https://docs.rs/crate/opentelemetry_sdk/0.28.0/features) | `rt-tokio` non-default; default signal features |
| S14 | [OTLP exporter spec](https://opentelemetry.io/docs/specs/otel/protocol/exporter/) | 4317/4318 defaults, protocol/endpoint env vars, 10 s timeout, retry |
| S15 | [docs.rs `SdkTracerProvider` 0.28.0](https://docs.rs/opentelemetry_sdk/0.28.0/opentelemetry_sdk/trace/struct.SdkTracerProvider.html) | `shutdown()`, `force_flush()`, drop triggers shutdown |
| S16 | [OTel Collector overview](https://opentelemetry.io/docs/collector/) | when to use a collector; small-scale can work without; retries/batching/filtering role |
| S17 | [OTel Rust — pt-BR](https://opentelemetry.io/pt/docs/languages/rust/) | translated status table |
| S18 | [OTel Rust — es](https://opentelemetry.io/es/docs/languages/rust/) | English shell |
| S19 | [OTel Rust — fr](https://opentelemetry.io/fr/docs/languages/rust/) | English shell |
| S20 | [OTel Rust — zh-CN](https://opentelemetry.io/zh/docs/languages/rust/) | translated status table |
| S21 | [OTel Rust — ja](https://opentelemetry.io/ja/docs/languages/rust/) | translated status table |
| S22 | [OTel Rust — de (404)](https://opentelemetry.io/de/docs/languages/rust/) | coverage gap (NV-4) |

**Local (read-only) evidence:** F1 `Cargo.toml:143,161-168`; F2 `Cargo.lock` grep + all-manifest grep; F3 `flora-core/src/config/app.rs:32-35,61-62,73-93`; F4 `flora-api/src/main.rs:17-24`; F5 `.env.example:64-66`; F6 `flora-api/src/routes/mod.rs:51-60`; F7 `flora-api/src/routes/health.rs:64,96-120`; F8 local ADR/OBJETIVO/release/assessment docs.
