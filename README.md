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
- [ ] Organizations & Workspaces
- [ ] Users & Roles (OIDC)
- [ ] Channels & Messaging (Threads)
- [ ] Notifications (PGMQ + Valkey)
- [ ] Tasks
- [ ] Files (RustFS)
- [ ] Search (Meilisearch)
- [ ] Infrastructure: PostgreSQL, SQLx, Docker, Coolify, OpenTelemetry
- [ ] License: AGPL

### Development Phases
- **FASE 0**: Licensing + dependency audit
- **FASE 1**: Workspace/Identity
- **FASE 2**: Messaging
- **FASE 3**: Tasks
- **FASE 4**: Files + RustFS
- **FASE 5**: Search + Meilisearch
- **FASE 6**: Notifications + PGMQ + Valkey
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

## ⚙️ Local Development (CI)

We use `lefthook` for local CI on commit:
- `cocogitto` (Conventional commits)
- `cargo fmt --fix`
- `cargo deny`
- `cargo audit`
- `cargo vet`
- `cargo check`
- `cargo clippy`
- `cargo nextest run`

## ⚖️ License

Flora is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.
