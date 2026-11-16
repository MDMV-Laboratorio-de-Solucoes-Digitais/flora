# Release: 016 — Seed/VPS Operability Hardening

## Summary
- Slice C (016): Seed/VPS operability hardening for lean Postgres-first runtime.
- Single DB pool/init path with config-driven conservative defaults.
- Search and messaging made optional/profile-gated with degraded/unavailable handling and capability reporting.
- Compose profiles/resource guardrails/healthchecks, canonical `FLORA_*` env alignment, and dev smoke/backup guidance updated.

## Scope boundaries
- No binary file upload/download (slice B deferred).
- No new mandatory services, multi-node/durable realtime, search indexing features, OIDC/SQLite redesign, S3 rollout, or 2vCPU/4GB perf/load evidence.

## Verification
- Local: `cargo check -p flora-core -p flora-api -p flora-tests` pass, `docker compose config -q` pass.
- Full fmt/clippy/tests and DB-gated contracts: to be confirmed by CI and/or follow-up local run in uncontended environment.

## NV / Follow-ups
- Validate load/memory behavior on real 2vCPU/4GB host; verify compose resource enforcement.
- DB-gated contract/runtime smoke with disposable PG18 and optional-service matrix.
- Cycle 017 candidate: slice B (binary file upload/download hardening) per 016 decision.
