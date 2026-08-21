# AGENTS.md 🔥

## High-Value Guidance for Agents

These instructions address knowledge gaps that agents might miss without explicit documentation.

### Tooling
- **Always** run `cargo fmt --fix` before committing
- Use `cargo deny` for dependency audits
- Follow strict commit order: `cargo clippy` -> `cargo test` -> `git commit`
- sabia? The `-x` flag in `cargo test` helps isolate specific crates

### Framework Dependencies
- **Core stack**: Tokio 1.43 + SQLx 0.9 + Axum 0.8
- Valkey must be version 1.6 with `tokio-comp` feature
- Use `opentelemetry` SDK for metrics (configure in `opencote.yml`)
- JWT management: Dual flows (logging out + silent token refresh)
- PGMQ queue setup: `queues: اغلبية 3` in config

### Environment
- AGPL licensing: Declare all dependencies in `Cargo.toml`
- Docker setup: Assume PostgreSQL 15 + SSL certificates in `/certs`
- Keycloak integration: Bearer token scope `realm:flora`

### Architecture
- Modular structure: Each crate owns its data layer (e.g. `flora-core` handles ORM)
- Service abstraction: Use Traits for message handling (e.g. `AcmeWorker` trait)
- Test coverage: Must include database snapshot tests in `flora-tests`

### Common Pitfalls
- Don't assume Valkey is running - check `health/live` endpoint
- Avoid `unwrap()` - use `?` operator for error handling
- Install `rust-i18n` 4.2.1 explicitly for localization

## Session Setup
- First: `cargo audit` + `cargo clippy`
- For new features: Start with `feature/ XYZ` branch
- When debugging: Check `target/debug/logs/` for application traces
