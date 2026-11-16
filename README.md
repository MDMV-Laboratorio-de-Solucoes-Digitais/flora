# Flora 🌿

[![CI](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/workflows/rust.yml/badge.svg)](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/workflows/rust.yml)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

Flora is a modern corporate suite designed to grow from a single VPS to a distributed infrastructure without changing its technological ecosystem.

Each installation is a **seed**. Each module is a **tree**. Each deployment makes the ecosystem more **fertile**.

## 🌲 Vision

A self-hostable, modular corporate platform built in Rust, aimed at providing organizations with full control over their data and infrastructure.

## 🚀 Scalability Levels

- 🌱 **Seed**: Minimal installation (Flora + PostgreSQL).
- 🌿 **Sprout**: Core collaboration (Seed + Valkey).
- 🌳 **Grove**: Full workspace (Sprout + PGMQ + RustFS + Meilisearch + ZITADEL).
- 🌲 **Forest**: Large organization (Replicated DB, distributed workers, observability, multiple Flora nodes).

## 🗺️ Roadmap

### First Milestone: Flora Seed v0.1
Mechanisms are named after their authoritative Seed backend; optional components are profile-gated.
See [ADR-0001](docs/adr/0001-seed-v01-mechanism-substitutions.md).

- [x] Organizations & Workspaces
- [x] Users & Roles (OIDC — PKCE S256 + verified ID token/JWKS; DB sessions)
- [x] Channels & Messaging (Threads)
- [x] Notifications (in-app Postgres with DB-enforced dedup; PGMQ + Valkey deferred to Sprout/Grove)
- [x] Tasks
- [x] Files (local filesystem with streaming upload/download and SHA-256; RustFS deferred to Grove)
- [x] Search (Postgres FTS with `source: "postgres"`; Meilisearch dormant/non-authoritative)
- [~] Infrastructure: PostgreSQL + SQLx + Docker + CI verified; Coolify deploy webhook secret-gated (NV); OpenTelemetry exporter NV
- [x] License: AGPL-3.0-or-later (full text + NOTICE; dependency-audit policy in `deny.toml`)

### Development Phases
- **FASE 0**: Licensing + dependency audit (license complete; CI audit gate NV)
- **FASE 1**: Workspace/Identity
- **FASE 2**: Messaging
- **FASE 3**: Tasks
- **FASE 4**: Files (local storage; RustFS deferred to Grove)
- **FASE 5**: Search (Postgres FTS; Meilisearch deferred and gated)
- **FASE 6**: Notifications (in-app Postgres; PGMQ + Valkey deferred)
- **FASE 7**: Documents
- **FASE 8**: Email
- **FASE 9**: Integrations
- **FASE 10**: AI feature flags
- **FASE 11**: Tauri
- **FASE 12**: Svelte 5

## 🛠️ Development Methodology

1. **Spec-driven development** with GitHub issues.
2. **Value-driven prioritization** (Impact vs. Effort).
3. **Feature branching**: `username/task-description`.
4. **TDD**: Write tests first to meet the "Definition of Done".
5. **Atomic Conventional Commits**: One file per commit (strictly enforced).
6. **Trait-based services**: Use Traits to define functionality; services implement Traits for swappability.
7. **PR-driven**: Merges to `main` require 100% CI approval.
8. **Automated Deploy**: Merges to `main` trigger deployment to Coolify via GitHub Actions.

## 🖥️ Frontend apps

- **`apps/frontend`**: canonical Flora frontend (SvelteKit + Svelte 5 rewrite, spec 002 / constitution). CI runs in `apps/frontend` (`bun install`, `check`, `lint`, `test:unit`, `build`).
- **`apps/web`**: separate imported `@coparse/web` (SolidJS/Macro-style) codebase present in this checkout; **not** the canonical Svelte 5 target. Treat as legacy/reference unless explicitly reconciled.

## ⚙️ Local Development (CI)

Current repo state (prefer executable config over this list): Rust CI runs `cargo build/test/release` (`.github/workflows/rust.yml`), frontend CI targets `apps/frontend` (`.github/workflows/ci.yml`). `lefthook`, `cargo-deny`, `cargo-vet`, and `cargo-nextest` configs may not be present in this checkout despite earlier plans; use `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, and crate/app-specific test commands as documented in `AGENTS.md` and `specs/`.

## ⚖️ License

Flora is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
