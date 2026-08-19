# Implementation Plan: Flora Seed v0.1 Core Features

**Branch**: `[001-flora-seed-v0-1]` | **Date**: 2026-08-17 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `./spec.md`

**Note**: This template is filled in by the `/speckit.plan` command; its definition describes the execution workflow.

## Summary

Flora Seed v0.1 is a lightweight, open-source (AGPL) self-hostable collaboration platform core featuring organization and workspace management, user authentication via Zitadel OIDC, role-based access control, real-time messaging, task management, file sharing, global search, and notifications. Built as a Rust modular monolith using Cargo Workspaces and Trait-Driven Design, it targets deployment on a cheap VPS with PostgreSQL, Valkey, RustFS, and Meilisearch, providing horizontal scalability to larger topologies.

## Technical Context

**Language/Version**: Rust 1.75+ (stable channel)

**Primary Dependencies**: Axum (web framework), thiserror (error handling), anyhow (flexible error handling), tracing & tracing-subscriber (logging), sqlx (async PostgreSQL/SQLite driver), valkey-rs (Valkey/Redis client), meilisearch-sdk (Meilisearch client), openidconnect (OIDC client), rustfs (trait-based abstract filesystem), serde & serde_json (serialization), tokio (async runtime), tower & tower-http (middleware, services), uuid (entity IDs), chrono (date/time with UTC management), mockall (trait mocking for tests)

**Storage**: PostgreSQL (Cloud/Production canonical) with SQLite fallback (Local/Desktop/Offline) using sqlx with connection pooling and `.sql` migration files; Core relational tables with JSONB for flexible metadata; Valkey for caching, pub/sub, and streams with a failover strategy for high availability; RustFS as production object storage with S3-compatible and LocalFileSystem backends; Meilisearch for full-text search with a document-based schema and multi-tenant isolation via `organization_id` indexing

**Testing**: cargo test (unit/integration), mockall for trait mocking, cargo nextest, sqlx::testing or testcontainers for DB integration tests, valkey mock/meilisearch test doubles

**Target Platform**: Linux server (x86_64/arm64), deployable via Docker or bare metal; compatible with cheap VPS (low memory footprint)

**Project Type**: Web service (backend API) with modular monolith architecture; frontend SolidJS consumes JSON/REST endpoints

**Performance Goals**: Sub-200ms p95 API latency for core operations (monitored via OpenTelemetry); support 50 concurrent active users per organization (validated via automated load testing with `k6`); handle 10MB file uploads under 10s (using multipart chunked uploads); search latency under 5s for past week data (optimized via Meilisearch indexing); 99.9% uptime for core services (enforced via health checks and circuit breakers)

**Constraints**: Must run on minimal VPS (e.g., 1GB RAM, 1 vCPU); open-source AGPL licensed dependencies only; no vendor lock-in; self-hostable all components; trait-driven design for swapability; atomic commits (one file per git add); Conventional Commits

**Scale/Scope**: Designed for 2-100 members per organization (Seed topology); horizontally scalable to Grove/Forest topologies via replicated PostgreSQL, distributed RustFS, Valkey clustering, Meilisearch clustering, multiple Flora API nodes

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

All gates pass: Debloat & Vendor Unlock (open-source, self-hosted deps), Trait-Driven Design (core traits defined), Spec-Driven Development (plan follows spec), Atomic Commits (git add <file>), VPS Test (designed for cheap VPS), Modular Monolith (cargo workspace), Open Source & Self-Hostable (AGPL deps), Test-First (TDD approach).

## Project Structure

### Documentation (this feature)

```text
specs/001-flora-seed-v0-1/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
flora/
├── Cargo.toml
├── flora-core/                # Shared traits, types, utilities
│   ├── src/
│   │   ├── lib.rs
│   │   ├── traits/
│   │   │   ├── user_repository.rs
│   │   │   ├── message_service.rs
│   │   │   ├── storage_provider.rs
│   │   │   ├── notification_dispatcher.rs
│   │   │   └── file_store.rs
│   │   ├── models/
│   │   │   ├── user.rs
│   │   │   ├── organization.rs
│   │   │   ├── workspace.rs
│   │   │   ├── channel.rs
│   │   │   ├── message.rs
│   │   │   ├── task.rs
│   │   │   ├── file.rs
│   │   │   └── notification.rs
│   │   └── error.rs
│   └── Cargo.toml
├── flora-organization/        # Organization vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-messaging/           # Messaging vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-tasks/               # Tasks vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-files/               # Files vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-search/              # Search vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-notifications/       # Notifications vertical slice
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── flora-api/                 # Axum API layer integrating vertical slices
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/
│   │   │   ├── org.rs
│   │   │   ├── workspace.rs
│   │   │   ├── messaging.rs
│   │   │   ├── tasks.rs
│   │   │   ├── files.rs
│   │   │   ├── search.rs
│   │   │   ├── notifications.rs
│   │   ├── extractors/
│   │   │   └── auth.rs
│   │   └── state.rs
│   └── Cargo.toml
├── flora-tests/               # Integration tests (optional)
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
└── docker-compose.yml         # Seed topology deployment
```

**Structure Decision**: Modular monolith using Cargo workspace with separate crates for each vertical slice (organization, messaging, tasks, files, search, notifications) plus a core crate containing shared traits and models, and an API crate that wires everything together. This matches the "Vertical Slices" approach and enables independent development and testing.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
