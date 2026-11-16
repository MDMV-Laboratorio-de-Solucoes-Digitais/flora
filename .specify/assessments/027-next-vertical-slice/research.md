# Research: 027 — R1 synthesis

- **Scaffolding:** `flora-notifications` has zero Rust consumers; `NotificationsConfig` fields are validated but never read; the XADD path would collide with the 022 dedup index. Removal is mechanical: workspace member + workspace dep + crate dir + config module + Cargo.lock stanza. **Keep** `flora-core::NotificationService` (9 construction sites), `PgNotificationRepository`/dedup migration, and the `realtime` Valkey profile (messaging/OIDC).
- **Observability:** `tower-http` `trace` feature is enabled but `TraceLayer::new_for_http()` is never attached; one line in `routes/mod.rs`. OTLP exporter gated on 9 conditions (receiver, aligned versions, shutdown, budget) — NV.
- **Doc drift:** ADR-0001 still calls resolved items NV; README:38/70 stale; constitution version in ROADMAP/OBJETIVO still v1.2.0; specs/001 tasks mark unwired Valkey-stream work [X].
- **cargo-vet:** feasible as new-dependency gate (one `multer` exemption fixes `check --locked`; `suggest` panic isolated), but store is gitignored and imports need network — defer to 028.
- **E2E:** JWT mint must include a seeded `sessions` row (`jti` authority); the dev-smoke token expired today. Feasible but a larger slice — defer to 028.
- **Advisories:** no independent oracle; add scheduled OSV/Dependabot as advisory signal, not a blocking gate — defer.

## Decision drivers
Highest value per effort: make the codebase advertise only what runs, close governance contradictions, and add free per-request tracing without infra.
