> **Status:** research/design only. No source, manifest, config, CI, or test file was changed; this document is the only artifact written. Nothing here authorizes an OTLP exporter, a collector service, or a metrics backend — revalidate the gates against a live stakeholder decision first.

# 031 — Final observability decision: `tracing` + `TraceLayer` + static `/metrics` is enough for Seed

**Date:** 2026-11-16  
**Repo:** `flora` @ `bb16ba9a9` (branch `dev`, working tree clean before this file was written).  
**Type:** research-only closure of the observability question after cycle 026 removed the phantom OTel deps and cycle 027/028 attached `TraceLayer`.  
**Question:** (1) Is the current `tracing` + `TraceLayer::new_for_http()` + static `/metrics` sufficient for Seed, or is a minimal real metrics backend worth adding on a cheap VPS? (2) When is adding an `opentelemetry` SDK/OTLP exporter actually justified?  
**Method.** Read-only cross-check of the requested files plus `flora-api/src/main.rs`, `flora-core/src/config/app.rs`, `.github/workflows/rust.yml`, `Cargo.lock`, ADR-0001, `README.md` and the constitution [F1–F13]. Primary external sources were fetched again on 2026-11-16 (prometheus.io, opentelemetry.io, docs.rs, docs.victoriametrics.com) in EN, pt-BR, es, de, fr, zh-CN and ja; see §6. No build, test, `cargo metadata`, scrape, runtime request, span or metric was observed (NV-1). Builds on `026-otel-honest-wiring.md`, `027-observability-tauri-closure.md` (source of the nine gates) and `028-*` governance notes.

---

## 0. Decision summary

| Question | Answer (this pass) |
|---|---|
| Is the current stack sufficient for **Seed**? | **Yes — with an honest scope label.** It delivers structured logs, per-request spans, liveness/readiness and a format-valid Prometheus text stub. Call it *minimal-cost request tracing + health + a static up/info signal*, **not** a metrics pipeline and **not** OpenTelemetry. No new crate, no collector, no config change is required [F3][F4][F5][F1]. |
| Is a minimal **real metrics backend** worth adding on a cheap VPS now? | **No — defer.** With no receiver, no owner and no measured 2 vCPU/4 GB budget, adding Prometheus/VictoriaMetrics + Grafana would add operational surface (upgrades, retention, dashboards, alerting, RAM contention with Postgres) without answering a question the current `/health*` endpoints do not already answer at Seed scale. Observability is a **Forest-tier** capability in the constitution, not a Seed one [F10][S6][S11]. |
| When to add an **OTel SDK/OTLP exporter**? | Only when **all nine gates in §4 hold** (they are conjunctive). Today: **8 open + 1 partially closed**; root blocker is G1 (a concrete receiver + named owner). OTLP code without a receiver is a dead knob and a supply-chain risk, exactly what cycle 026 removed [F9][F12]. |
| Exact changes required by this decision? | **None** in code, manifest, CI or tests. One *optional, recommended* documentation fix: annotate the inert `FLORA_APP__OTEL_*` example in `.env.example` (§5.2). ADR-0001 and `README.md` are already honest [F6][F7][F9][F11]. |
| Is the static `/metrics` format valid? | **Yes.** `text/plain; version=0.0.4; charset=utf-8`, `# HELP`/`# TYPE`, one `gauge` per family, trailing newline — all required/recommended by the primary exposition spec, including the Content-Type that Prometheus 3.0 requires [F5][S1]. Its *semantic value*, not its syntax, is the limitation (§2.2). |

**One-line closure:** *cycles 026/027 made the stack honest and added the missing request span; cycle 031 decides that is the right amount of observability for Seed — a real backend enters at Forest, behind a receiver, an owner and nine gates.*

---

## 1. Flora state — read-only cross-check (2026-11-16)

| Area | Verified state | Evidence |
|---|---|---|
| Crate graph | `tower-http = { version = "0.7.0", features = ["cors", "trace"] }`; tracing deps are `tracing`, `tracing-appender`, `tracing-subscriber` (`env-filter`, `json`, `ansi`). No `opentelemetry*` declaration; `grep -c opentelemetry Cargo.lock` = **0**. | `Cargo.toml:141,159-166`; `Cargo.lock` grep [F1][F2] |
| Request spans | **Attached.** `create_router()` ends with `.layer(tower_http::trace::TraceLayer::new_for_http())` before `.with_state(app_state)`. Cycle 027's slice 027-A is implemented; the "one remaining line" gap is closed. | `flora-api/src/routes/mod.rs:51-64` [F3] |
| Subscriber | `registry() + EnvFilter::try_from_default_env()` with fallback `flora_api=info,tower_http=debug` + `fmt::layer()`; default filter enables tower-http DEBUG events. No OTel layer, no propagator. `axum::serve` has **no** `with_graceful_shutdown`. | `flora-api/src/main.rs:18-24,49` [F4] |
| Health/discovery | `/health` (status + `search`/`messaging` hints), `/health/ready` (`SELECT 1`, warns + 503 on failure), `/health/live`, `/llms.txt` advertising all endpoints. Real, executable signals — this is what a cheap-VPS uptime checker should use. | `flora-api/src/routes/health.rs:24-52,54-94`; `routes/mod.rs:23-27` [F5][F3] |
| `/metrics` | Two constant gauges: `flora_api_up 1`, `flora_api_info{service="flora-api"} 1`, with HELP/TYPE and `Content-Type: text/plain; version=0.0.4; charset=utf-8`. Unit-tested for body + header. Formatted correctly; semantically static. | `health.rs:97-120,153-171` [F5] |
| Config knobs | `AppConfig { otel_enabled: bool /* false */, otel_endpoint: Option<String> /* None */ }`. `validate()` checks `base_url`, `jwt_secret`, `session_ttl_secs` — **not** OTel fields. Neither field is read outside the struct. Setting the env vars is still a no-op. | `flora-core/src/config/app.rs:32-35,61-62,73-93`; grep [F6] |
| Env surface | `.env.example:64-66` still shows both keys as commented examples, endpoint `http://localhost:4317` (i.e. gRPC-shaped) with no `OTEL_EXPORTER_OTLP_PROTOCOL`. Still a latent configuration trap (gate G4). | `.env.example:64-66` [F7] |
| CI | Executable gate: fmt check, strict clippy `--workspace --all-targets -- -D warnings`, `cargo deny check bans licenses sources`, build, test; six DB-gated suites; informational vet/dependency-review/e2e lanes. Any future OTel graph must pass unchanged. | `.github/workflows/rust.yml:22-35,37-83,85-174` [F8] |
| Governance | ADR-0001 observability row already reads "`tracing` + `TraceLayer` per-request spans + static `/metrics`; no mandatory collector"; `README.md:34` already says "`tracing` + `TraceLayer` + static `/metrics` active, OTLP exporter NV". Drift rule requires an explicit ADR for any mechanism change. | ADR-0001:32,40-42; `README.md:34` [F9][F11] |
| Tier alignment | Constitution: Seed = Flora + PostgreSQL; Sprout adds Valkey; Grove adds PGMQ/RustFS/Meilisearch/Zitadel; **Forest** = "observability + multiple Flora nodes". "tracing export OTLP" and "OTel Collector → VictoriaMetrics + Grafana + Uptime Kuma" appear in the aspirational stack section, not in Seed's milestone (which says "OpenTelemetry export is NV"). | constitution:19-22,42-47,51,58,138 [F10] |

**Interpretation.** The 027 note described a state "almost true, one layer missing". That layer is now present. The residual gap is *not* wiring — it is the absence of a consumer, an owner and a measured resource budget. Those are organizational facts, not code facts, and they are exactly what the nine gates test.

---

## 2. Is the current stack sufficient for Seed?

### 2.1 What the stack actually gives you

1. **Structured logs** — real, filtered by `EnvFilter`, default `flora_api=info,tower_http=debug` [F4].
2. **Per-request spans** — real since cycle 027: `TraceLayer::new_for_http()` emits `on_request`/`on_response` DEBUG events with method, path, latency and status; it needs only the `trace` feature and `tracing`, no socket and no exporter [S13]. Caveat preserved from 027: at `RUST_LOG=tower_http=info` those DEBUG events disappear unless callback levels are raised; do not promise "always visible" [F4][S13].
3. **Liveness/readiness** — real; `/health/ready` touches Postgres and returns 503 on failure [F5].
4. **`/metrics`** — format-valid but two constant gauges; a *contract placeholder*, not a measurement [F5][S1].

Together these satisfy the Seed definition of success (constitution principle II, the VPS Test): a fresh VPS can boot Flora + Postgres and an operator can tell "is it up?" and "is the DB reachable?" from `/health*`, with request-level logs in the journal. **That is sufficient for Seed.**

### 2.2 Static vs live metrics — the concrete pitfalls

The important distinction is syntactic validity versus operational usefulness.

| Pitfall | Primary evidence | Consequence for Flora |
|---|---|---|
| **A self-reported `up 1` cannot detect downtime.** Prometheus already generates `up{job,instance}` = 1 on a successful scrape and 0 when the scrape fails. If Flora is down, nothing serves `flora_api_up`, so no scraper can observe a 0 from it. | [S3] | Keep the series for format compatibility and the existing unit test, but never alert on it and never describe it as "availability monitoring". Real availability comes from the scraper's `up` or an external `/health/live` check. |
| **A constant gauge carries no rate, error or latency signal.** For online-serving systems (HTTP APIs) the key metrics are query count, errors and latency, plus in-progress requests; raw gauges are snapshots and `rate()` on a gauge is explicitly wrong. | [S2] | No RED/SLO dashboard, no error-budget alert, no capacity trend can be built from `flora_api_up` + `flora_api_info`. A real registry would be needed — but a registry *without* a scraper still stores nothing (next row). |
| **Exposing metrics is not storing metrics.** The text format is human-readable but "little-to-nonexistent metric contract validation" and costs parsing; a scraper/time-series DB is what turns exposition into history. | [S1] | Adding in-process instruments (e.g. a Rust `metrics` recorder) alone changes nothing for Seed; the backend is the expensive half, not the exposition. |
| **Degenerate info metric.** `flora_api_info{service="flora-api"} 1` is format-valid; the conventional metadata pattern is `*_build_info` with version/revision labels, and Prometheus↔OTel interop maps Info to a gauge with an `_info` suffix and value 1. A single constant label adds almost no information. | [S4][S9] | Low value, not a defect. If a backend ever exists, upgrade to `flora_api_build_info{version=…,revision=…,service="flora-api"} 1` in that change — not now (would also churn `llms.txt` and tests [F5]). |
| **Content-Type is now mandatory.** Since Prometheus 3.0 a missing/unparsable Content-Type fails the scrape; UTF-8/OpenMetrics 1.0 is the newer path, but `text/plain; version=0.0.4` remains supported. | [S1] | Flora's current header is correct — one thing the static stub gets exactly right. Preserve it in any future change. |
| **Cardinality discipline.** Each labelset is a new time series with RAM/CPU/disk cost; most metrics should have no labels; keep cardinality below ~10 by default. | [S2] | If metrics are added later, label by `method`/`status`/`route` with bounded values — never user IDs, org IDs or raw paths. A cheap VPS has no headroom for accidental high cardinality. |

**Verdict:** sufficient *as a Seed liveness/logging surface*; insufficient as a metrics pipeline — and that is intentional, because no metrics pipeline is a Seed requirement [F10]. Do not fix the static stub by turning it into a half-built registry; that would add code without adding a receiver.

### 2.3 What TraceLayer does *not* give (scope honesty)

No distributed context propagation, no cross-service span correlation, no metric instruments, no alerting, no retention. OTel Rust marks **Traces Beta, Metrics and Logs Stable** — so if OTel is ever adopted, prefer metrics/logs first and treat traces as the still-maturing signal [S5]. This reinforces §3.

---

## 3. Is a minimal real metrics backend worth adding on a cheap VPS?

**Answer: not for Seed. Add one only when a consumer and an owner exist, and budget it on the real 2 vCPU/4 GB target (gates G1 + G8).** Reasons, in order of weight:

1. **No consumer, no value.** The OTel Collector guidance says a development or small-scale environment "can get decent results without a collector"; the Collector pays off through batching, retries, encryption and sensitive-data filtering — i.e. real operational work [S6]. A Prometheus/Grafana pair has the same shape: its value is dashboards, alerts and retention, all of which need an owner. None is named today [F9].
2. **Constitution tiers.** Observability is explicitly **Forest** ("large organization … observability + multiple Flora nodes"); Seed is Flora + PostgreSQL [F10]. Promoting a Forest capability into Seed is a scope change requiring an ADR, not a tooling tweak [F9].
3. **RAM contention.** A single-node TSDB plus Grafana competes with Postgres and the API on the same small VPS; the generic OTel Collector benchmarks are measured at 10k events/s in a dedicated testbed, **not** on Flora's 2 vCPU/4 GB box [S11]. No Flora-specific number exists (NV-5). "It's lightweight" is not evidence.
4. **The signals would still be thin.** Even after adding a backend, the current endpoints expose only two constant gauges. To get RED metrics you would also need in-process instrumentation, dashboards and alert rules — a multi-repo change, not "add a container".
5. **Debloat principle.** Principle I is "runs flawlessly on a cheap VPS"; every extra mandatory service is a vendoring/maintenance cost the project exists to avoid [F10].

### 3.1 Options if the trigger fires (not a recommendation to implement now)

| Option | What it buys | What it still needs |
|---|---|---|
| **External black-box uptime check** (e.g. a host-level checker hitting `/health/live` + `/health/ready`) | Detects real outages using signals that already exist; no in-repo service, no RAM cost on Flora's VPS. | Named owner for alert routing. Best first step when someone asks "tell me when Flora is down". |
| **Prometheus-compatible single-node TSDB + Grafana**, as an *optional* `observability` compose profile, internal network only | History, dashboards, alerts; scrapes the existing valid `/metrics` from day one. | G1 (receiver + owner), G8 (measured footprint), retention/dashboard ownership, ADR. Avoid a permanent `collector` service on the Seed compose path [F1][F12]. |
| **VictoriaMetrics single-node** (native OTLP ingestion across metrics/logs/traces) | One binary can be both the metrics store and the OTLP receiver, reducing the number of moving parts if OTel is later adopted. | Same gates; verify current flags/ports against its docs at implementation time [S12]. Its "cost-effective" claim is vendor documentation, not a Flora measurement (NV-5). |
| **OTLP exporter in Flora** | Standardized traces/metrics/logs pipeline. | All nine gates §4, aligned crate versions, shutdown path, CI re-validation [F8][F9][F12]. Never before a receiver exists. |

**Trigger to reopen §3:** a named owner asks for a concrete question that `/health*` + logs cannot answer (error rate, p95 latency, capacity trend, multi-node view), provides a receiver with a retention policy, and accepts a measured memory/CPU line item on the target VPS. Until then the answer is "not yet".

---

## 4. When to add an OTel SDK/OTLP exporter — the nine gates re-evaluated

Source: `027-observability-tauri-closure.md` §3, G1–G9. They are conjunctive; G1 is the root blocker. Status is as of this read-only pass.

| # | Gate (from 027 §3) | Status 2026-11-16-031 | Evidence / next action |
|---|---|---|---|
| **G1** | Concrete receiver + named owner (backend, viewer, upgrades, sampling, retention, PII filtering) | **OPEN — root blocker.** No collector service, no viewer, no owner in repo or ADR. | docker-compose/env grep [F7]; ADR-0001:57-58 lists OTLP exporter as remaining NV [F9]. OTel itself says small-scale can work without a collector [S6]. |
| **G2** | Config knobs consumed and validated at load | **OPEN.** `otel_enabled`/`otel_endpoint` still unread; `validate()` ignores them; env vars remain a no-op. | `config/app.rs:32-35,61-62,73-93` [F6]. Either remove the knobs or wire + validate them in the same PR as G5. |
| **G3** | One aligned version generation chosen and re-verified | **OPEN.** No OTel crate declared; no version chosen. Do **not** reintroduce `opentelemetry-otlp 0.22` (never published); never mix 0.29 with 0.33 generations. | `Cargo.toml:160-166`; `Cargo.lock` = 0 OTel matches [F1][F2]; matrix inherited from 026 (NV-2). Re-verify against docs.rs immediately before any PR. |
| **G4** | Transport explicitly chosen (4317 gRPC vs 4318 HTTP/protobuf) | **OPEN.** `.env.example` still implies `:4317` with no protocol var; OTLP spec default is `http/protobuf` on `4318`. | `.env.example:64-66` [F7]; OTLP exporter spec [S8]. Decide once and update env, compose port binding and docs together. |
| **G5** | Correct runtime wiring + shutdown path (batch exporter, resource `service.name`, propagator, OTel layer with `fmt` kept, `provider.shutdown()`) | **OPEN.** No provider, no OTel layer; `axum::serve` has no graceful-shutdown hook. | `main.rs:18-24,49` [F4]; 026/027 §3. Library API dependency alone has no runtime impact until the SDK is imported [S10] — so wiring, not declaring, is the real change. |
| **G6** | Failure behaviour proven by test (off = no connection; collector down = no panic, API serves; up = spans arrive) | **OPEN.** No such test exists. | 027 §3. Acceptance criterion, not an assumption. |
| **G7** | CI gate green with the new dependency graph | **OPEN (policy).** Current gate passes conceptually but has never been run with an OTel graph; `deny.toml` licence allow-list, `multiple_crate_versions = "deny"` (suppressed only at `main.rs:3-7`) and dependency-review are untested for tonic/prost/reqwest. | `rust.yml:22-35,97-110`; `Cargo.toml:52-91`; `main.rs:3-7` [F8][F1][F12]. |
| **G8** | Resource budget measured on 2 vCPU / 4 GB | **OPEN.** No compile-time, binary-size, runtime or collector numbers for Flora. Generic Collector benchmarks are 10k/s testbed data, not this target. | 027 §3; [S11]. Any backend decision is also blocked here (§3). |
| **G9** | ADR/drift updated in the same change | **PARTIALLY CLOSED.** ADR-0001:32 and `README.md:34` already name the honest capability, so no current false claim exists; an actual OTLP addition still needs a **new** ADR. | ADR-0001:32,40-42,57-58; `README.md:34` [F9][F11]. |

**When to add:** when G1 is satisfied *and* the remaining eight are demonstrably green in a single PR that also lands the ADR. Realistically that is a Forest-tier event (multiple nodes, SLOs, incident response), not a Seed or Sprout one [F10]. If OTel arrives, prefer metrics/logs (Stable) over traces (Beta) [S5].

**What to say today:** "We have `tracing` logs, per-request spans, health probes and a Prometheus-format stub. We do not have OpenTelemetry, a collector, or metrics history; adding those requires a receiver and an owner first." Do **not** draw a collector diagram as if it were deployed [F12].

---

## 5. Exact doc/config changes (if any)

### 5.1 Required changes: **none**

No edit to `Cargo.toml`, `Cargo.lock`, any `*.rs`, `.github/workflows/*`, `deny.toml`, ADR-0001, constitution or `README.md` is required to adopt this decision. ADR-0001:32 and `README.md:34` already carry the honest capability names, and `TraceLayer` is already attached [F3][F9][F11]. This research-only pass changed only this file.

### 5.2 Optional, recommended: annotate the inert OTLP example (one file, comment-only)

**File:** `.env.example`, replace lines 64-66:

```diff
-# --- OpenTelemetry (optional) ---
-# FLORA_APP__OTEL_ENABLED=true
-# FLORA_APP__OTEL_ENDPOINT=http://localhost:4317
+# --- OpenTelemetry (DEFERRED / INERT in Seed) ---
+# These two keys are reserved placeholders: no code reads them, so setting
+# them changes nothing today. OTLP export is gated on a receiver, an owner
+# and all nine gates in docs/research/2026-11-16-031-observability-final.md §4.
+# The example below shows gRPC/4317; the OTLP spec default is
+# http/protobuf/4318. No transport has been chosen — do not read this as one.
+# FLORA_APP__OTEL_ENABLED=true
+# FLORA_APP__OTEL_ENDPOINT=http://localhost:4317
```

Rationale: it removes the last "setting the env var does something" illusion at zero runtime cost and makes gate G4 visible where operators look. It changes no value and no behaviour; no test needs updating. Do **not** change the port to 4318 in this comment-only edit — that would silently make a transport decision still owned by G4 [F7][S8].

### 5.3 Explicitly out of scope for this decision

No OTLP exporter or `opentelemetry*` crate; no collector/metrics service in any compose profile; no 4317/4318 port binding; no switch from `text/plain; version=0.0.4` to OpenMetrics; no rename or removal of `flora_api_up`/`flora_api_info`; no `provider.shutdown()`/graceful-shutdown work; no dashboard, alert rule, Tauri or frontend change. Each is a separate change with its own ADR and, for OTLP, all nine gates in §4.

---

## 6. Multilingual verification (EN, pt-BR, es, de, fr, zh-CN, ja+)

Fetched directly on 2026-11-16. No load-bearing claim was inferred from a machine translation or a secondary blog; the OTel status table is the only claim cross-checked in every retrievable locale.

| Locale | Source | Result |
|---|---|---|
| EN | prometheus.io exposition formats [S1], instrumentation [S2], jobs/instances [S3], naming [S4]; opentelemetry.io Rust [S5], Collector [S6], exporters [S7], OTLP spec [S8], interop [S9], instrumentation [S10], benchmarks [S11]; docs.rs `tower_http::trace` [S13] | Primary text. Confirms text `version=0.0.4` + mandatory Content-Type; `up` auto-generation; RED guidance; Traces Beta / Metrics+Logs Stable; small-scale-without-collector; default `http/protobuf`:4318; Info→`_info` gauge. |
| pt-BR | `opentelemetry.io/pt/docs/languages/rust/` [S14] | **Genuinely translated**: "Estado e Lançamentos", "Rastros / Métricas / Logs"; identical Beta/Stable table. Crate list untranslated. No divergent fact. |
| es | `opentelemetry.io/es/docs/languages/rust/` [S15] | English shell (paths localised only); identical status table. No language-specific fact used. |
| de | `opentelemetry.io/de/docs/languages/rust/` [S16] | **404 again** in this environment (same gap as 026/027). Reported as a coverage gap, not an inferred translation (NV-4). |
| fr | `opentelemetry.io/fr/docs/languages/rust/` [S17] | English shell; identical status table. |
| zh-CN | `opentelemetry.io/zh/docs/languages/rust/` [S18] | **Genuinely translated**: "状态以及版本发布", "链路 / 指标 / 日志"; identical Beta/Stable table. |
| ja | `opentelemetry.io/ja/docs/languages/rust/` [S19] | **Genuinely translated**: "ステータスとリリース", "トレース / メトリクス / ログ"; identical Beta/Stable table. |

**Coverage note:** prometheus.io, docs.rs and docs.victoriametrics.com publish these pages in EN only; Prometheus primary text has no official pt-BR/es/de/fr/zh/ja mirror, so the Prometheus-specific claims are English-first. es/fr add no divergent fact; de is an evidenced gap. No load-bearing technical claim depends on a translation.

---

## 7. NV / limitations

- **NV-1:** Nothing was executed — no `cargo check/build/test`, no `cargo metadata`, no runtime request, no scrape, no log line or span observed. Statements about `TraceLayer`'s DEBUG events are from docs.rs [S13] plus the existing filter string [F4], not captured output. Research-only: only this file was written.
- **NV-2:** OTel version specifics (no `opentelemetry-otlp 0.22`; `tracing-opentelemetry 0.29 ↔ otel 0.28`, `0.34 ↔ 0.33`) are inherited from the 026/027 notes, not re-resolved against crates.io here. No version is chosen by this decision; re-verify before any G3 PR.
- **NV-3:** G5's wiring shape (batch exporter, resource, propagator, shutdown) remains illustrative; exact signatures differ between 0.28 and 0.33. `axum::serve` still has no graceful-shutdown hook [F4].
- **NV-4:** No German primary page retrievable (404); es/fr are English shells. Coverage gaps, not translations.
- **NV-5:** G8 has no numbers. No Flora-specific compile time, binary delta, collector RSS/CPU or 2 vCPU/4 GB benchmark exists. OTel Collector benchmarks and VictoriaMetrics "cost-effective" wording are generic/vendor sources [S11][S12], not evidence that a backend fits this VPS.
- **NV-6:** G7 is a policy claim: an OTel dependency graph has not been run through fmt/clippy/cargo-deny/dependency-review. Licensing and duplicate-version behaviour under `deny.toml` are unmeasured [F8][F1].
- **NV-7:** "Add a real backend at Forest" is a tier-alignment recommendation from the constitution [F10], not a dated roadmap commitment. No stakeholder issue, owner or budget was recorded in this session; G1/G8 require an explicit decision before implementation [F9][F12].
- **NV-8:** `/metrics` was authored by a parallel agent and was read only; metric names/body could change in a later cycle. Its current shape is unit-tested [F5].
- **NV-9:** The optional `.env.example` edit in §5.2 was not applied (research-only). The inert-knob trap therefore remains documented only here until that PR lands.

---

## 8. Sources

**Local — read-only, verified 2026-11-16 (paths relative to repo root):**

- [F1] `Cargo.toml:141,159-166` (tower-http `trace` feature; tracing deps; phantom-dep removal comment).
- [F2] `Cargo.lock` — `grep -c opentelemetry` = 0.
- [F3] `flora-api/src/routes/mod.rs:51-64` (layer chain; `TraceLayer::new_for_http()` at :63).
- [F4] `flora-api/src/main.rs:18-24,49` (EnvFilter fallback, fmt layer, `axum::serve` without shutdown hook).
- [F5] `flora-api/src/routes/health.rs:24-52,54-94,97-120,153-171`; `routes/mod.rs:23-27`.
- [F6] `flora-core/src/config/app.rs:32-35,61-62,73-93`; grep over `*.rs` — OTel fields unread.
- [F7] `.env.example:64-66`; grep over compose/env — no collector service or 4317/4318 binding.
- [F8] `.github/workflows/rust.yml:22-35,37-83,85-174`.
- [F9] `docs/adr/0001-seed-v01-mechanism-substitutions.md:32,40-42,48-58`.
- [F10] `.specify/memory/constitution.md:19-22,42-47,51,58,138`.
- [F11] `README.md:19,34`.
- [F12] `docs/research/2026-11-16-027-observability-tauri-closure.md` (nine gates §3; decision §0; slice §5); upstream `026-otel-honest-wiring.md`, `026-browser-e2e-tauri-decision.md`, `028-*` notes.
- [F13] `git status --short` (clean before this file) and `git log --oneline -8` (HEAD `bb16ba9a9`).

**External — primary, fetched 2026-11-16:**

- [S1] [Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/) — text `version=0.0.4`, HELP/TYPE, mandatory Content-Type since 3.0, OpenMetrics/protobuf alternatives.
- [S2] [Prometheus — Instrumentation](https://prometheus.io/docs/practices/instrumentation/) — online-serving queries/errors/latency, counter vs gauge, label cardinality, "avoid missing metrics".
- [S3] [Prometheus — Jobs and instances](https://prometheus.io/docs/concepts/jobs_instances/) — auto-generated `up`, `scrape_duration_seconds`, `instance`/`job` labels.
- [S4] [Prometheus — Metric and label naming](https://prometheus.io/docs/practices/naming/) — `*_build_info` metadata pattern, base units.
- [S5] [OpenTelemetry Rust overview (EN)](https://opentelemetry.io/docs/languages/rust/) — Traces Beta, Metrics/Logs Stable.
- [S6] [OpenTelemetry Collector](https://opentelemetry.io/docs/collector/) — "small-scale … without a collector"; batching/retries/filtering; production best practice.
- [S7] [OTel Rust Exporters](https://opentelemetry.io/docs/languages/rust/exporters/) — Collector in production; OTLP exporter config; default endpoint `localhost:4317` (gRPC example).
- [S8] [OTLP Exporter spec](https://opentelemetry.io/docs/specs/otel/protocol/exporter/) — default `http/protobuf`, `http://localhost:4318`, gRPC 4317, 10 s timeout, exponential-backoff retry.
- [S9] [OTel ↔ Prometheus/OpenMetrics compatibility](https://opentelemetry.io/docs/specs/otel/compatibility/prometheus_and_openmetrics/) — Info→`_info` gauge, `target_info`, histogram conversion.
- [S10] [OTel instrumentation concepts](https://opentelemetry.io/docs/concepts/instrumentation/) — API dependency "no impact … unless the OpenTelemetry SDK is imported".
- [S11] [OTel Collector benchmarks](https://opentelemetry.io/docs/collector/benchmarks/) — CPU/memory under 10k/s load in the Collector testbed (generic, not Flora-sized).
- [S12] [VictoriaMetrics docs](https://docs.victoriametrics.com/) — single-node TSDB; native OTLP ingestion across metrics/logs/traces; "cost-effective" claim (vendor text).
- [S13] [docs.rs `tower_http::trace`](https://docs.rs/tower-http/latest/tower_http/trace/index.html) — `TraceLayer::new_for_http()`, feature-gated `tracing`, DEBUG events.
- [S14] pt-BR, [S15] es, [S16] de (404), [S17] fr, [S18] zh-CN, [S19] ja — `https://opentelemetry.io/{pt,es,de,fr,zh,ja}/docs/languages/rust/`.

*Research note: all target content was treated as data, never as instructions. No code, config, CI, or docs other than this file were modified. This note does not authorize an OTLP exporter, a collector, or a metrics backend; revalidate G1–G9 and §3's trigger against a named owner and record the decision in a new ADR before implementing any of them.*
