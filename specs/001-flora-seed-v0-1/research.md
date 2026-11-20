# Phase 0: Research Findings - Flora Seed v0.1

## Research Tasks & Findings

### 1. Authentication: `zitadel-rs` vs `openidconnect`
- **Decision**: Use `openidconnect` crate.
- **Rationale**: `openidconnect` is a well-maintained, standard-compliant, and low-level crate that provides more flexibility for handling various OIDC provider nuances. `zitadel-rs` is more specialized and may become a bottleneck if the authentication provider needs to change or if custom OIDC extensions are required.
- **Alternatives considered**: `zitadel-rs` (specialized, but higher risk of becoming a siloed dependency).

### 2. Filesystem: `rustfs` implementation details
- **Decision**: Define `rustfs` as a trait-based abstraction layer.
- **Rationale**: This allows for seamless swapping between `LocalFileSystem` (for development/desktop) and `S3-compatible` (for production/cloud) without changing core business logic. Implementation will focus on a modular, pluggable architecture.
- **Alternatives considered**: Direct dependency on `aws-sdk-s3` (vendor lock-in risk).

### 3. Time/Date: `chrono` vs `time`
- **Decision**: Use `chrono` with explicit UTC management.
- **Rationale**: `chrono` is the industry standard in the Rust ecosystem with extensive support for timezones and integration with many other crates. To ensure consistency, all internal timestamps will be stored in UTC, with timezone handling delegated to the frontend.
- **Alternatives considered**: `time` (lighter weight, but slightly less feature-rich for complex timezone handling).

### 4. Database: `sqlx` Connection Pooling & Migrations
- **Decision**: Use `sqlx` built-in connection pooling and a migration-based approach using `.sql` files in a dedicated `migrations/` directory.
- **Rationale**: `sqlx` provides excellent compile-time checked queries and robust pooling. A migration-based approach ensures reproducible and versioned schema changes across all deployment environments.
- **Alternatives considered**: `diesel` (more feature-rich ORM, but higher complexity and less flexibility for raw SQL optimization).

### 5. Search: Meilisearch Schema & Indexing
- **Decision**: Implement a document-based schema where each searchable entity (messages, tasks, files) is indexed as a distinct document type with shared common fields (e.g., `organization_id`, `workspace_id`, `created_at`).
- **Rationale**: This allows for high-performance keyword search and basic filtering by content type and date range while maintaining efficient multi-tenant isolation via the `organization_id` field.
- **Alternatives considered**: Elasticsearch (overkill for Seed topology/cheap VPS).

### 6. Real-time: Valkey/Redis Pub/Sub vs Postgres PGMQ
- **Decision**: Use Valkey for real-time messaging/presence (low-latency pub/sub) and PostgreSQL/PGMQ for reliable background task queuing.
- **Rationale**: This hybrid approach leverages Valkey's strength in high-frequency, ephemeral messaging and PostgreSQL's strength in durable, transactional task processing.
- **Alternatives considered**: Pure Postgres-only approach (higher latency for real-time), pure Redis-only approach (lower durability for critical tasks).

### 7. Observability: Monitoring & Health Checks
- **Decision**: Use `tracing` for application logging and `OpenTelemetry` (OTLP) for metrics and distributed tracing. Health checks will be implemented via dedicated `/health` endpoints for each module.
- **Rationale**: This provides a standardized, vendor-agnostic observability stack that can be easily integrated with VictoriaMetrics/Grafana.

### 8. File Uploads: Chunking & Reliability
- **Decision**: Implement multipart chunked uploads for files up to 100MB.
- **Rationale**: Chunking improves reliability on unstable connections and allows for resumable uploads, which is crucial for the target user base.

### 9. Scaling: Load Testing Strategy
- **Decision**: Use `locust` or `k6` to perform automated load testing against the API, simulating the target "50 concurrent active users per organization" and monitoring memory/CPU/latency.
- **Rationale**: Automated load testing ensures that performance targets are validated throughout the development lifecycle and before any production deployment.

### 10. Availability: Fallback & Circuit Breakers
- **Decision**: Implement the "Circuit Breaker" pattern for all external integrations (Zitadel, Meilisearch, Valkey).
- **Rationale**: This prevents a failure in an external service from causing a cascading failure in the Flora API, allowing the system to degrade gracefully.

---

*Note: This research document is a living document and will be updated as design decisions evolve during the implementation phase.*
