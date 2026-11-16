# Decision: 027 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — Honest Seed closure.** Remove the unwired PGMQ/Valkey notification crate and config; attach `TraceLayer`; reconcile ADR/README/planning/spec drift.

## Scope (in)
- Remove `flora-notifications` workspace member + workspace dep + crate directory; remove `NotificationsConfig` module and its `config/mod.rs` wiring; regenerate Cargo.lock (one stanza).
- Add `TraceLayer::new_for_http()` to the API router (feature already enabled; no exporter/collector).
- Reconcile ADR-0001 (FASE 0 CI + transport decision now resolved), README:38/70, ROADMAP/OBJETIVO constitution version, specs/001 Valkey claims, specs/026 status, AGENTS crate list, dev-smoke realtime note.
- Verification: no `pgmq|XADD|flora_notifications` in `.rs`/`.toml`; fmt, strict clippy, cargo-deny, check, unit + six DB suites, CI green; PR/merge/sync.

## NV / deferred (028)
cargo-vet store/imports + gate; browser e2e with fresh JWT; Dependabot/OSV advisory job; release SBOM; OTLP exporter; Tauri; 2vCPU/4GB validation.

## Risk controls
Keep core `NotificationService`, Postgres dedup path and Valkey realtime profile untouched; do not alter parallel work; verify Cargo.lock diff is exactly one stanza.
