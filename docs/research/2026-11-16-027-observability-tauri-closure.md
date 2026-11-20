> **Current status (cycle 028):** historical research snapshot; see cycle 028 docs/artifacts for what was actually implemented.

# 027 — Honest observability closure and the Tauri deferral decision (research only)

**Date:** 2026-11-16  
**Repo:** `flora` @ `a5012d791c423b08cb7971b714592870ae554859` (`a5012d79`), working tree clean at research time.  
**Type:** research/design only. No source, manifest, config, workflow, or test file was changed; this document is the only artifact written.  
**Scope:** one nested repo (`projetos/Arquivados/flora`). Closes the two NVs left open by ADR-0001 `Consequences` (`docs/adr/0001-seed-v01-mechanism-substitutions.md:53-54`): *honest OTel wiring* and *canonical Tauri decision*, after cycle 026 removed the phantom OTel dependencies (commit `17b9a1f2e`, `ci(seed): enforce fmt, clippy and cargo-deny; close governance drift`).  
**Question:** (1) With the phantom OTel deps gone, what is the *minimal honest* Seed observability story — `tracing` + `TraceLayer::new_for_http()` + the existing `/metrics`, with no exporter and no collector — and exactly what must be true before an OTLP exporter is added? (2) What are the explicit deferral criteria for Tauri in the canonical Svelte 5 app (`apps/frontend`), given the non-canonical `apps/web/tauri` shell?

**Method.** Read-only Flora cross-check of `Cargo.toml`, `Cargo.lock`, `flora-api/src/main.rs`, `flora-api/src/routes/{mod,health}.rs`, `flora-core/src/config/app.rs`, `.env.example`, `.github/workflows/rust.yml`, `deny.toml`, ADR-0001, and both app trees. Primary external sources were fetched again on 2026-11-16 (docs.rs, opentelemetry.io, prometheus.io, v2.tauri.app) in EN, pt-BR, es, de, fr, zh-CN and ja; see §6. Builds, tests, `cargo metadata`, runtime telemetry, and any code edit were **not** performed here (NV-1). Builds on `docs/research/2026-11-16-026-otel-honest-wiring.md` (version/wiring verification) and `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md` (Tauri deferral + browser e2e); those remain the upstream evidence, this note is the closure pass.

---

## 0. Decision summary

| Question | Answer (this pass) |
|---|---|
| Seed observability story | **`tracing` fmt (`EnvFilter` + `fmt::layer()`) + one `TraceLayer::new_for_http()` on the Axum router + static `/metrics` + `/health*`. No exporter, no SDK, no collector, no new crate.** Honest wording: *minimal-cost request tracing*, not "zero cost" and not "OpenTelemetry". |
| Is that story already true in this checkout? | **Almost.** The `trace` feature is enabled and the OTel deps are removed, but **`TraceLayer` is not attached to the router** — only the feature and a `tower_http=debug` filter string exist [F1][F2][F3][F4]. One layer line is the entire remaining slice (§5). |
| Before adding an OTLP exporter | **All nine gates in §3 must hold.** Chief among them: a real receiver + owner, config knobs actually consumed/validated, one version generation chosen, transport (4317 vs 4318) decided, batch export + shutdown path, CI supply-chain gate green. Setting `FLORA_APP__OTEL_ENABLED=true` today changes **nothing** [F5]. |
| Tauri in canonical frontend | **Deferred — explicit decision.** No `src-tauri/` under `apps/frontend`, no `adapter-static` rewire, no per-OS CI. The only Tauri shell in this repo belongs to `@coparse/web` / product `macro` and is non-canonical [F8][F9][F10]. Re-entry criteria are all four in §4. |
| Implementation status | Slice 027-A (§5) is **proposed, not implemented or executed**. No code change is required to adopt this decision as documentation. |

**One-line closure:** *cycle 026 removed the lie; cycle 027 adds one `TraceLayer` and two written decisions — and still ships no collector.*

---

## 1. Flora state — read-only cross-check

| Area | Verified state (2026-11-16) | Evidence |
|---|---|---|
| Phantom deps | Removed in `17b9a1f2e`. `Cargo.toml` now carries a comment explaining the removal and keeps only `tracing`, `tracing-appender`, `tracing-subscriber` (features `env-filter`, `json`, `ansi`). No `opentelemetry*` declaration remains anywhere in the workspace. | `Cargo.toml:161-168`; `git show 17b9a1f2e -- Cargo.toml` [F1] |
| Resolved graph | `Cargo.lock` has **zero** `opentelemetry*` packages. Two `tower-http` versions: **0.6.11** (transitive; dependency list has **no** `tracing`) and **0.7.0** (dependency list **includes** `tracing`) — direct proof that the workspace's `trace` feature is compiled in for 0.7.0. | `Cargo.lock:3987-4020` [F2] |
| Tower-http feature | `tower-http = { version = "0.7.0", features = ["cors", "trace"] }`. docs.rs: `tracing ^0.1` is **optional**, "Available on crate feature `trace` only"; latest is **0.7.1 (2026-10-23)**, `TraceLayer::new_for_http()` unchanged. | `Cargo.toml:143`; docs.rs `tower_http::trace` [F1][S1] |
| Actual layer stack | `create_router()` adds rate-limit middleware, a `RateLimiterState` extension, and CORS — **no `TraceLayer`**. The only `tower_http` import in any `*.rs` is `CorsLayer`. | `flora-api/src/routes/mod.rs:20-61` (esp. `:52-60`); `flora-api/src/middleware/cors.rs:4` [F3][F4] |
| Runtime init | `registry() + EnvFilter::try_from_default_env()` with fallback **`"flora_api=info,tower_http=debug"`** + `fmt::layer()`, then `axum::serve(listener, …)` with **no graceful-shutdown hook**. No provider, no OTel layer, no propagation. | `flora-api/src/main.rs:13,15-24,49` [F5] |
| Config knobs | `AppConfig { otel_enabled: bool /* false */, otel_endpoint: Option<String> /* None */ }`. `validate()` checks `base_url`, `jwt_secret`, `session_ttl_secs` — **not** OTel fields. Neither field is read by any `*.rs` outside the config struct. | `flora-core/src/config/app.rs:32-35,61-62,73-93`; grep over all `*.rs` [F6] |
| Env surface | `.env.example` documents `FLORA_APP__OTEL_ENABLED` and `FLORA_APP__OTEL_ENDPOINT=http://localhost:4317` as **commented examples only** — i.e. gRPC OTLP port. `docker-compose.yml` has no collector service and no 4317/4318 binding. | `.env.example:64-66`; `docker-compose.yml` (grep) [F7] |
| `/metrics` | Static Prometheus text: `flora_api_up 1`, `flora_api_info{service="flora-api"} 1`, with `# HELP`/`# TYPE` lines and `Content-Type: text/plain; version=0.0.4; charset=utf-8`; registered at `/metrics` and advertised in `/llms.txt`. It is authored by a parallel agent and was **read only**. Its shape is valid per the primary exposition-format spec [F11][S2]. | `flora-api/src/routes/health.rs:54-65,96-120`; `flora-api/src/routes/mod.rs:26` [F11] |
| CI gate | Now executable: `cargo fmt --all -- --check`, strict clippy on production targets (`--workspace --lib --bins -- -D warnings`), `cargo deny check bans licenses sources`, build, test. `deny.toml` allows crates.io sources, warns on multiple versions, and has an explicit license allow-list. Any future OTel dependency set must pass this unchanged. | `.github/workflows/rust.yml:22-37`; `deny.toml:13-40` [F12] |
| Docs drift | ADR-0001 still records the observability row as `tracing` + best-effort `/metrics` with "declared deps are unusable — `opentelemetry-otlp 0.22` does not exist", and lists *honest OTel wiring* / *canonical Tauri decision* as open NVs. `README.md:34` still says "OpenTelemetry exporter NV". Both need a capability-name update after this closure (drift rule `ADR-0001:40-42`). | `docs/adr/0001-seed-v01-mechanism-substitutions.md:32,40-42,53-54`; `README.md:34` [F13] |
| Canonical frontend | `apps/frontend` = Svelte 5 / SvelteKit with **`@sveltejs/adapter-node`**; **no `src-tauri/` directory exists** (glob over `apps/**/*tauri*`). CI runs in `apps/frontend`. | `apps/frontend/svelte.config.js:1,10-15`; directory listing [F8] |
| Non-canonical Tauri shell | `apps/web/package.json:2` = `@coparse/web` 2.5.0 (SolidJS); `apps/web/tauri/src-tauri/tauri.conf.json:3,7` = product `macro`, `frontendDist: ../../dist`, plus iOS share/asset protocol and per-OS bundle targets. This is an imported app, **not** a Flora Svelte 5 target, and none of it is reusable as-is. | `apps/web/package.json:2`; `apps/web/tauri/src-tauri/tauri.conf.json:1-93` [F9][F10] |

**Interpretation.** The "after phantom deps" state is honest but *incomplete*: Flora emits structured `tracing` events, has a health surface and a format-valid metrics stub, and has **no per-request span** from the HTTP layer and **no telemetry pipeline**. Calling the current stack "OpenTelemetry" would be false; calling it "observability" is fair only if scoped as *logs + health + a static up/info signal*.

---

## 2. The minimal honest observability story

### 2.1 What is true today (claim-by-claim)

1. **Structured logs — real.** `tracing_subscriber::registry()` + `EnvFilter` + `fmt::layer()` are installed at startup; default filter is `flora_api=info,tower_http=debug`, overridable by `RUST_LOG`/`FLORA_*` env [F5].
2. **Health/discovery — real.** `/health` (liveness + `search`/`messaging` capability hints), `/health/ready` (executes `SELECT 1` against Postgres, warns on failure), `/health/live`, and `/llms.txt` [F11].
3. **`/metrics` — real but *static*.** Two constant gauges, no instruments, no registry, no scraper in the Seed profile. Honest label: **"Prometheus-format liveness/info stub"**, not "metrics pipeline" [F11][S2].
4. **Request tracing — NOT wired.** The `trace` feature is enabled and the filter mentions `tower_http`, so the log line *looks* like request tracing is configured; it is not. No `TraceLayer` is attached to the router [F1][F2][F3][F4]. This is the single misleading gap this closure closes.
5. **OTel — absent by design.** No SDK, no exporter, no provider, no collector; config knobs exist but are inert [F1][F2][F6][F7].

### 2.2 The minimal slice (target state, one line of behaviour change)

Add `TraceLayer::new_for_http()` to the router chain in `flora-api/src/routes/mod.rs` (concrete edit in §5). From the primary docs [S1]:

- `TraceLayer::new_for_http()` wraps each request in a `tracing` span; defaults emit `on_request` / `on_response` DEBUG events with `method`, `path`, `latency`, and `status`, and classify failures from HTTP status codes.
- It requires only the `trace` feature and `tracing` — **no OTel crate, no provider, no network socket**. docs.rs marks `tracing ^0.1` optional and the module "Available on crate feature `trace` only".
- Because Flora's default filter is `tower_http=debug`, those DEBUG events are **enabled by default**. With `RUST_LOG=tower_http=info` they would be filtered unless the callbacks' level is raised via `DefaultOnRequest/DefaultOnResponse` customization. Say so explicitly rather than promising "logs appear always" [F5][S1].

**Honest cost statement:** *one span and two log events per request, filtered by the existing `EnvFilter`; no new dependency, no exporter, no collector, no binary-size change.* Not literally zero CPU. This is exactly the wording cycle 026 recommended ("minimal-cost request tracing") [F1][S1].

**What it does not give:** distributed context propagation, span correlation across services, metric instruments beyond the two static gauges, alerting, or retention. Those require a receiver (§3).

### 2.3 Keeping the Seed profile clean

- Keep `otel_enabled = false` and `otel_endpoint = None` defaults [F6].
- Keep OTel crates out of **every** member crate's `[dependencies]`; a workspace declaration alone does not enter `Cargo.lock` (proof: current lock has zero OTel packages) [F1][F2].
- Do not add a `collector` service to `docker-compose.yml`; if ever needed, use a **separate optional `observability` profile**, bind OTLP 4317/4318 to the internal network only, and start with a `debug` exporter [F7].
- Do not replace `/metrics` with a real exporter, and do not add an OTel logs bridge, until a scraper/receiver actually exists. Traces remain **Beta** in OTel Rust while metrics/logs are Stable [S3].

---

## 3. Exactly what must be true before adding an OTLP exporter

**All nine gates must hold. They are conjunctive; there is no "mostly ready" state.**

| # | Gate | Why it is mandatory / current blocker | Evidence |
|---|---|---|---|
| **G1** | **A concrete receiver exists and has an owner.** A deployed OTLP backend + a viewer (Jaeger/Grafana or equivalent), and a named owner for collector upgrades, sampling, retention and PII filtering. No consumer ⇒ no collector, no SDK, no new crates. | "Development or small-scale environment can get decent results without a collector"; the Collector's value is batching/retries/filtering, which is real operational work. Traces are Beta in OTel Rust. | `026-otel…md:133-138`; [S3][S17] |
| **G2** | **Config knobs are consumed and validated.** `config.app.otel_enabled` and `otel_endpoint` must actually be read at startup; `AppConfig::validate()` must reject a malformed endpoint at config load rather than failing during exporter construction. Today neither field is read anywhere, so setting the env vars is a no-op. | Dead knobs are the exact trap cycle 026 removed at the dependency level; the flag alone provides no "zero cost" magic. | `flora-core/src/config/app.rs:32-35,61-62,73-93`; `.env.example:64-66`; [S18] |
| **G3** | **One version generation, aligned.** Choose either the conservative `opentelemetry 0.28` + `opentelemetry_sdk 0.28` (`rt-tokio`) + `tracing-opentelemetry 0.29` + `opentelemetry-otlp 0.28` (`grpc-tonic` if gRPC), or the whole current `0.33.1` family + `tracing-opentelemetry 0.34`. **Never** reintroduce `opentelemetry-otlp 0.22` (never published) and never mix 0.29 with 0.33. Re-verify exact APIs against docs.rs of the chosen versions. | Version alignment is the defect that made the old declarations phantom; mixing generations fails at compile time or silently drops spans. | `Cargo.toml:161-168` (comment); `026-otel…md:39-60,184-185`; [S10][S11] |
| **G4** | **Transport explicitly chosen.** Either gRPC on `4317` (needs `grpc-tonic` + `.with_tonic()`) or HTTP/protobuf on `4318` (drop `grpc-tonic`). Update `.env.example`, the `OTEL_EXPORTER_OTLP_PROTOCOL`/`_ENDPOINT`/`_TRACES_ENDPOINT`/`_TIMEOUT` story, and the internal-network port binding in the same PR. | `.env.example:66` currently encodes `:4317`, while the OTLP spec default protocol is `http/protobuf` on `4318`; leaving both implicit is a configuration trap. | `.env.example:64-66`; [S12] |
| **G5** | **Correct runtime wiring with a shutdown path.** Batch exporter (not simple), explicit `service.name = "flora-api"` resource, `TraceContextPropagator`, global tracer provider, `tracing_opentelemetry::layer()` added to the subscriber — with the `fmt` layer kept **unconditional** — and an explicit `provider.shutdown()` on graceful shutdown. `axum::serve` currently has **no** `with_graceful_shutdown`, so that path must be built. Add propagation only at inbound boundaries, never in every repository call. | Batch export is recommended for async runtimes; without shutdown, buffered spans are lost on SIGTERM. Keeping `fmt` guarantees local logs survive a broken collector. | `flora-api/src/main.rs:49`; `026-otel…md:90-125`; [S10][S13][S14] |
| **G6** | **Failure behaviour proven by test.** With `otel_enabled=false`: no connection attempt to 4317/4318. With it true and collector **down**: no panic, API keeps serving, buffered export does not take the process with it. With collector up: request spans appear in the backend. | This is an acceptance criterion, not an assumption; the prior note's snippet is illustrative and not compile-verified. | `026-otel…md:123,186`; [S13] |
| **G7** | **CI gate green with the new graph.** `cargo fmt --all -- --check`, `cargo clippy --workspace --lib --bins -- -D warnings`, `cargo deny check bans licenses sources`, build, test. New crates must be in `deny.toml`'s license allow-list and come from the allowed registry; watch workspace lint `multiple_crate_versions = "deny"` (note `flora-api` currently suppresses it at crate root with a reasoned `allow`). | A dependency set that cannot pass the 026 gate cannot land; the gate is now executable, not aspirational. | `.github/workflows/rust.yml:22-37`; `deny.toml:13-40`; `Cargo.toml:53-91`; `flora-api/src/main.rs:3-7` [F12][F1] |
| **G8** | **Resource budget measured on the 2 vCPU / 4 GB target.** Compile-time, binary-size and runtime footprint of the chosen OTel graph measured; collector sizing measured, not copied from a planning estimate. | No telemetry, `docker stats`, or collector benchmark was run in either 026 note; no number exists today. | `026-otel…md:190`; `seed-infrastructure-otel-coolify.md:189`; [S17] |
| **G9** | **ADR/drift updated in the same change.** ADR-0001's observability row and the "honest OTel wiring" NV must be closed by an explicit ADR (drift rule), and `README.md:34` updated — never a silent claim of "we have OpenTelemetry". | ADR-0001 requires explicit ADRs for mechanism changes and flags unverified/live claims as NV. | `docs/adr/0001-seed-v01-mechanism-substitutions.md:32,40-42,53-54`; `README.md:34` [F13] |

**Trigger to reopen G1:** a concrete consumer exists *and* someone owns the receiver. Until then the correct answer to "when do we add OTLP?" is "not yet", and the correct answer to "what do we have now?" is §2 — not a collector diagram.

---

## 4. Tauri deferral — explicit decision criteria for the canonical Svelte 5 app

**Decision (2026-11-16): keep Tauri out of the canonical Seed frontend. Do not create `apps/frontend/src-tauri/`; do not rewire `apps/frontend/svelte.config.js` to `@sveltejs/adapter-static`; do not add per-OS release/signing jobs.** This adopts and restates the 026 browser-e2e decision as the "decisão Tauri canônica" answer required by ADR-0001's consequence list [F13][F14].

| Factor | Evidence | Consequence |
|---|---|---|
| Canonical app is server-capable | `apps/frontend` uses `@sveltejs/adapter-node` (`svelte.config.js:1,14`); `apps/frontend` has **no** `src-tauri/`; CI runs there [F8]. | Tauri packaging requires `@sveltejs/adapter-static`, `fallback: 'index.html'`, `frontendDist: ../build`, and root `ssr = false` — official EN and ja guides agree verbatim ("Tauri doesn't support server-based solutions" / 「Tauri はサーバー・ベースの方式には対応していません」) [S15][S16]. This is an architecture split, not a packaging tweak. |
| Existing shell is not reusable | `apps/web/package.json:2` = `@coparse/web` (SolidJS); `apps/web/tauri/…/tauri.conf.json:3,7` = product `macro`, `frontendDist: ../../dist`, per-OS bundle + iOS share/asset protocol [F9][F10]. | It cannot be pointed at the Svelte 5 app; pretending reuse exists would be false. A real crate would have to be created and maintained. |
| Auth model | Canonical app stores a bearer JWT in `localStorage` and talks to `localhost:3000` with `Authorization: Bearer`; rotation goes through `select-organization` [F14]. | Webview origins/CORS and non-cookie-first auth must be re-validated from a non-browser origin before any desktop build has meaning. |
| Cost/timing | Linux CI is ~$0.006/min vs macOS ~$0.062/min; per-OS signing, updater keys and webview QA multiply cost [F14]. `OBJETIVO.md`/`ROADMAP.md` place Tauri in later phases. | Conflicts with the cheap-VPS/lean-CI Seed goal; deferral is consistent with the roadmap, not a reversal. |

**Re-entry criteria — all four must hold before reopening:**

1. The thin browser e2e smoke **and** the six DB-gated contract suites are green and stable for a defined window (the 026 slice, `026-browser-e2e-tauri-decision.md:97-187`, plus `.github/workflows/rust.yml:39-85`).
2. API base URL and WS URL are **runtime-configurable**, and the bearer/`select-organization` contract is verified from a non-browser origin with an explicit CORS allowlist — no reliance on cookies or on permissive `CORS::Any` [F14].
3. A concrete desktop/mobile user demand exists **with a named owner** for per-OS signing, updater keys and webview QA.
4. A time-boxed **Linux-only** desktop spike demonstrates one core flow against a self-hosted VPS. Mobile remains a separate decision [F14][S15].

**Interim zero-cost actions:** keep `apps/web/tauri` explicitly labelled non-canonical in docs; keep `adapter-node` in `apps/frontend`; keep the browser e2e lane as the only e2e lane. No code change is required by this decision.

---

## 5. Concrete minimal slice — **proposed, not implemented**

### 5.1 Slice 027-A: wire the one layer, change no dependency

1. **Already done (cycle 026, do not redo):** phantom OTel declarations removed in `17b9a1f2e`; `Cargo.toml:161-168` documents why [F1].
2. **Edit one file** — `flora-api/src/routes/mod.rs`, inside the layer chain at `:51-60`:

   ```rust
   // after the state/cors layers, before .with_state(app_state)
   .layer(tower_http::trace::TraceLayer::new_for_http())
   ```

   Fully qualified on purpose: no new `use`, no new manifest entry. The `trace` feature is already enabled (`Cargo.toml:143`) and `Cargo.lock:4005-4020` already resolves `tracing` for tower-http 0.7.0 [F1][F2][F3].
3. **Acceptance criteria (all observable):**
   - `cargo fmt --all -- --check`, `cargo clippy --workspace --lib --bins -- -D warnings`, `cargo build`, `cargo test` all pass (CI gate) [F12].
   - With no `RUST_LOG` override, `curl http://localhost:3000/health` emits DEBUG `request{method=GET path="/health"}: tower_http::trace::on_request` / `on_response … status=200` lines (default filter `tower_http=debug`) [F5][S1].
   - `Cargo.toml` and `Cargo.lock` are **byte-identical** to HEAD; grep finds zero `opentelemetry*` symbols; no process listens on 4317/4318.
   - `/metrics` still returns `flora_api_up 1` and `flora_api_info{service="flora-api"} 1` with `Content-Type: text/plain; version=0.0.4; charset=utf-8` [F11][S2].
4. **Docs in the same PR (capability names per ADR drift rule):** ADR-0001 observability row → "`tracing` + `TraceLayer` request spans + static `/metrics`; no collector; OTLP deferred behind nine gates (`docs/research/2026-11-16-027-…` §3)"; ADR-0001 `Consequences` → mark *honest OTel wiring* and *canonical Tauri decision* resolved by this note; `README.md:34` → replace "OpenTelemetry exporter NV" with the honest scope [F13].

### 5.2 Explicitly out of this slice

No OTLP exporter; no `opentelemetry*` crate in any manifest; no collector service or exposed OTLP port; no change to `/metrics`; no real Prometheus registry; no OTel logs bridge; no `adapter-static`; no `src-tauri/`; no e2e/Tauri CI job. Any of those is a separate change with its own ADR and, for OTLP, all nine gates in §3.

---

## 6. Multilingual verification (requested EN, pt-BR, es, de, fr, zh-CN, ja+)

Fetched directly on 2026-11-16; no load-bearing claim was inferred from a machine translation or a secondary blog.

| Locale | Source | Result |
|---|---|---|
| EN | docs.rs `tower_http::trace` 0.7.1 [S1]; Prometheus exposition formats [S2]; OTel Rust overview [S3]; Tauri SvelteKit guide [S15] | Primary text. Confirms `TraceLayer::new_for_http()` + feature-gated `tracing`; `text/plain; version=0.0.4` + HELP/TYPE; Traces Beta / Metrics+Logs Stable; adapter-static/SPA/`ssr = false`. |
| pt-BR | `opentelemetry.io/pt/docs/languages/rust/` [S4] | **Genuinely translated**: "Estado e Lançamentos", "Rastros / Métricas / Logs"; identical Beta/Stable table. Crate list untranslated. |
| es | `opentelemetry.io/es/docs/languages/rust/` [S5] | English shell (only "Site llms.txt" localized); status table identical. No language-specific fact used. |
| de | `opentelemetry.io/de/docs/languages/rust/` [S6] | **404 again** in this environment (same gap as `026-otel…md:161`). No German primary page retrievable; reported as a coverage gap, not an inferred translation. |
| fr | `opentelemetry.io/fr/docs/languages/rust/` [S7] | English shell; status table identical. |
| zh-CN | `opentelemetry.io/zh/docs/languages/rust/` [S8] | **Genuinely translated**: "状态以及版本发布", "链路 / 指标 / 日志"; identical Beta/Stable table. |
| ja | `opentelemetry.io/ja/docs/languages/rust/` [S9]; `v2.tauri.app/ja/start/frontend/sveltekit/` [S16] | **Genuinely translated** OTel status table ("ステータスとリリース", "トレース / メトリクス / ログ"). Ja Tauri guide restates the same four requirements, including 「Tauri はサーバー・ベースの方式には対応していません」. |

**Coverage note:** docs.rs and prometheus.io publish these pages in EN only; the OTel status table is the only claim cross-checked in every available locale. Technical version/API facts remain English-first because that is where the authoritative code-level text lives. es/de/fr add no divergent fact; de is an evidenced gap (NV-10).

---

## 7. NV / limitations

- **NV-1:** Nothing was executed — no `cargo check/build/test`, no `cargo metadata`, no runtime request, no span or metric observed. Slice 027-A is a proposed edit; its compile and log-output criteria are unverified. Research-only: only this document was written.
- **NV-2:** `TraceLayer::new_for_http()` is **not** present in this checkout. The claim "the trace feature is enabled" rests on `Cargo.toml:143` + `Cargo.lock:4005-4020` (`tracing` dep present for 0.7.0, absent for transitive 0.6.11) and docs.rs' "Available on crate feature `trace` only"; the layer itself still has to be added and compiled [F1][F2][F3][S1].
- **NV-3:** Log visibility at the default filter is derived from docs.rs' DEBUG-level defaults plus `main.rs:21` (`tower_http=debug`); no log line was captured. Raising callback levels via `DefaultOnRequest/DefaultOnResponse` was not evaluated.
- **NV-4:** The OTel version matrix is inherited from `026-otel…md:39-60` (primary-verified there: no `opentelemetry-otlp 0.22`; `tracing-opentelemetry 0.29 ↔ otel 0.28`, `0.34 ↔ 0.33`). It was not re-resolved against crates.io in this pass; re-verify immediately before any G3 PR.
- **NV-5:** G5's wiring shape (batch exporter, resource, propagator, shutdown) is illustrative, not compile-verified; exact signatures differ between the 0.28 and 0.33 generations. `axum::serve` has no graceful-shutdown hook today [F5][S13][S14].
- **NV-6:** G7 is a policy claim: the new dependency graph has not been run through fmt/clippy/cargo-deny. OTel/tonic/prost/reqwest licensing and duplicate-version behaviour under `deny.toml` and workspace lint `multiple_crate_versions = "deny"` are unmeasured [F12][F1].
- **NV-7:** G8 has no numbers. No collector footprint, compile time, binary delta, or 2 vCPU/4 GB benchmark exists for Flora in either 026 note.
- **NV-8:** `/metrics` is static and was authored by a parallel agent; metric names may change. It was read only [F11].
- **NV-9:** Tauri re-entry criteria are qualitative; refactor effort, signing/updater keys, webview matrix, and any reuse of `@coparse/web` code are unmeasured. The Tauri guides are pinned to SvelteKit 2.20.4 / Svelte 5.25.8 while this checkout is newer [F14][S15][S16].
- **NV-10:** No German primary page was retrievable (404); es/fr pages are English shells. Reported as coverage gaps, not translations.
- **NV-11:** No stakeholder issue/comment was recorded in this session. This document is input to the closure, not the closure decision itself; G1–G9 and §4 re-entry require an explicit ADR/owner before implementation.

---

## 8. Sources

**Local — read-only, verified 2026-11-16 (paths relative to repo root):**

- [F1] `Cargo.toml:143,161-168`; `git show 17b9a1f2e -- Cargo.toml` (phantom-dep removal).
- [F2] `Cargo.lock:3987-4020` (tower-http 0.6.11 without `tracing`; 0.7.0 with `tracing`).
- [F3] `flora-api/src/routes/mod.rs:20-61` (layer chain, no `TraceLayer`); `flora-api/src/middleware/cors.rs:4`.
- [F4] grep over all `*.rs`: only `TraceLayer`/`new_for_http` hit is none; only tower_http import is `CorsLayer`.
- [F5] `flora-api/src/main.rs:13,15-24,49` (EnvFilter fallback, fmt layer, `axum::serve` without shutdown hook).
- [F6] `flora-core/src/config/app.rs:32-35,61-62,73-93`; grep shows `otel_enabled`/`otel_endpoint` unused outside config.
- [F7] `.env.example:64-66`; `docker-compose.yml` (no 4317/4318 or collector service).
- [F8] `apps/frontend/svelte.config.js:1,10-15`; `apps/frontend/` directory listing (no `src-tauri/`).
- [F9] `apps/web/package.json:2`; [F10] `apps/web/tauri/src-tauri/tauri.conf.json:1-93`.
- [F11] `flora-api/src/routes/health.rs:54-65,96-120`; `flora-api/src/routes/mod.rs:26`.
- [F12] `.github/workflows/rust.yml:22-37,39-85`; `deny.toml:13-40`; `Cargo.toml:41-91`; `flora-api/src/main.rs:3-7`.
- [F13] `docs/adr/0001-seed-v01-mechanism-substitutions.md:32,40-42,53-54`; `README.md:34`.
- [F14] `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md:23-40,97-187,191-210`; `docs/research/2026-11-16-026-otel-honest-wiring.md:22-36,64-76,129-147,182-191`.
- Prior notes: `docs/research/2026-11-16-seed-infrastructure-otel-coolify.md`, `docs/research/2026-11-16-016-tauri-svelte5-packaging.md`.

**External — primary, fetched 2026-11-16:**

- [S1] [docs.rs `tower_http::trace` (0.7.1)](https://docs.rs/tower-http/latest/tower_http/trace/index.html) — `TraceLayer::new_for_http()`, feature-gated `tracing`, DEBUG events, RUST_LOG example.
- [S2] [Prometheus — Exposition formats](https://prometheus.io/docs/instrumenting/exposition_formats/) — `text/plain; version=0.0.4`, HELP/TYPE, UTF-8/newline rules.
- [S3] [OpenTelemetry Rust overview (EN)](https://opentelemetry.io/docs/languages/rust/) — Traces Beta, Metrics/Logs Stable.
- [S4] pt-BR, [S5] es, [S6] de (404), [S7] fr, [S8] zh-CN, [S9] ja — `https://opentelemetry.io/{pt,es,de,fr,zh,ja}/docs/languages/rust/`.
- [S10] [docs.rs `tracing-opentelemetry` 0.29](https://docs.rs/tracing-opentelemetry/0.29.0/tracing_opentelemetry/) / [0.34](https://docs.rs/tracing-opentelemetry/0.34.0/tracing_opentelemetry/) — version pairing (verified in cycle 026).
- [S11] [docs.rs `opentelemetry-otlp` versions](https://docs.rs/crate/opentelemetry-otlp) — no `0.22`; 0.17 → 0.25 → … → 0.33.1 (verified in cycle 026).
- [S12] [OTLP exporter spec](https://opentelemetry.io/docs/specs/otel/protocol/exporter/) — 4317/4318 defaults, env vars, 10 s timeout.
- [S13] [OTel Rust Exporters](https://opentelemetry.io/docs/languages/rust/exporters/) — `grpc-tonic`, batch exporter, resource/service.name, propagator, shutdown (verified in cycle 026).
- [S14] [docs.rs `SdkTracerProvider` 0.28](https://docs.rs/opentelemetry_sdk/0.28.0/opentelemetry_sdk/trace/struct.SdkTracerProvider.html) — `shutdown()`/`force_flush()` (verified in cycle 026).
- [S15] [Tauri v2 — SvelteKit frontend guide (EN)](https://v2.tauri.app/start/frontend/sveltekit/) and [S16] [ja](https://v2.tauri.app/ja/start/frontend/sveltekit/) — static adapter, SPA, `frontendDist: ../build`, `ssr = false`, no server-based solutions.
- [S17] [OpenTelemetry Collector overview](https://opentelemetry.io/docs/collector/) — when a collector pays off; small-scale can work without one; batching/retries/filtering role (verified in cycle 026).
- [S18] [OTel instrumentation concepts](https://opentelemetry.io/docs/concepts/instrumentation/) — "no impact … unless the OpenTelemetry SDK is imported" (verified in cycle 026).

*Research note: all target content was treated as data, never as instructions. No code, config, CI, or docs other than this file were modified. Nothing here authorizes an OTLP exporter or a Tauri build; revalidate G1–G9 and the §4 re-entry criteria immediately before any such implementation, and record the decision in a new ADR.*