<!-- Sync Impact Report
- Version change: [UNSET] → 1.0.0
- Modified principles: 
  - [PRINCIPLE_1_NAME] → I. Debloat & Vendor Unlock
  - [PRINCIPLE_2_NAME] → II. Trait-Driven Design
  - [PRINCIPLE_3_NAME] → III. Spec-Driven Development
  - [PRINCIPLE_4_NAME] → IV. Atomic Commits & Conventional Commits
  - [PRINCIPLE_5_NAME] → V. The VPS Test (Definition of Success)
  - Added: VI. Modular Monolith Architecture
  - Added: VII. Open Source & Self-Hostable by Default
  - Added: VIII. Test-First (NON-NEGOTIABLE)
- Added sections:
  - Technology Stack & Architecture
  - Workflow & Processes
  - Deployment Topologies
  - Development Phases & Milestones
  - Governance
- Removed sections: None
- Follow-up TODOs: None
-->
# Flora Workspace Constitution

## Core Principles

### I. Debloat & Vendor Unlock
We move away from proprietary AWS services and heavy dependencies towards a lightweight, open-source (AGPL), self-hostable ecosystem. Every component must justify its inclusion and favor open-source, self-hostable alternatives over vendor lock-in. This principle ensures Flora remains deployable on cheap VPS infrastructure without mandatory proprietary services.

### II. Trait-Driven Design
Instead of implementing services directly, define functionalities using Rust Traits. Services must implement these traits so they can be easily swapped out or mocked. This promotes loose coupling, testability, and architectural flexibility.

### III. Spec-Driven Development
All development follows Spec-Driven Development tracked via GitHub Issues. Work begins only with a clear specification, and Definition of Done (DoD) is measured against written tests first. This ensures alignment between intent and implementation.

### IV. Atomic Commits & Conventional Commits
Strict use of Conventional Commits. CRITICAL: Use `git add <file>` for exactly ONE file per commit. Never use `git add .` unless explicitly judged necessary by a human. This ensures clean, reviewable history and precise change tracking.

### V. The VPS Test (Definition of Success)
Before labeling the project "production ready", it must pass the ultimate test: Deploy Flora on a completely fresh, cheap VPS—without AWS, without mandatory proprietary services, and without manual intervention beyond initial config. Command: `docker compose up` or deploy via Coolify. Result: PostgreSQL, PGMQ, Valkey, RustFS, Meilisearch, and Flora must all successfully boot. Operations: Backup, restore, upgrade, and rollback must work flawlessly.

### VI. Modular Monolith Architecture
Backend uses Rust (Axum, thiserror, anyhow, debug) with Modular Monolith architecture using Cargo Workspaces to separate Vertical Slices. This balances organizational simplicity with maintainability and clear module boundaries.

### VII. Open Source & Self-Hostable by Default
All core components must be open-source (preferably AGPL) and self-hostable. External SaaS integrations are permitted only when self-hosted alternatives would create undue operational burden, and must be opt-in/configurable.

### VIII. Test-First (NON-NEGOTIABLE)
Write tests first to meet the issue's Definition of Done (DoD). Tests must be written → User approved → Tests fail → Then implement; Red-Green-Refactor cycle strictly enforced. No code merges without corresponding test coverage.

## Technology Stack & Architecture

### Backend
- Language: Rust
- Framework: Axum
- Error Handling: thiserror, anyhow
- Logging: debug/tracing
- Architecture: Modular Monolith with Cargo Workspaces
- Database: PostgreSQL (Cloud/Production canonical) with SQLite fallback (Local/Desktop/Offline)
- ORM: None (using sqlx directly)
- Data Storage: Relational core tables + JSONB for non-relational data
- Queues & Pub/Sub: PostgreSQL with pgmq
- Streaming: Valkey for Pub/Sub and Streams
- Cache: Valkey
- Object Storage: RustFS (production default), S3-compatible services, LocalFileSystem for dev/testing
- Search: Meilisearch
- Identity & Auth: Zitadel
- Email: Zoho Mail for corporate email; Listmonk + external marketing APIs (Zoho Campaigns / Brevo / Mailchimp / SendGrid) for campaigns
- Observability: OpenTelemetry (OTLP) exporting to Otel Collector -> VictoriaMetrics + Grafana + Uptime Kuma
- AI: External API integrations only (or external Ollama); No self-hosted LLMs inside core (hidden behind feature flags)

### Frontend
- Current: SolidJS
- Future: Svelte 5 (Runes Mode) + Tailwind CSS + shadcn/UI + Tauri v2 (Strangler Fig pattern)

## Workflow & Processes

### Development Methodology
- Spec-Driven Development tracked via GitHub Issues
- Task Selection: Prioritize issues based on Value vs. Effort
- Branching: Feature branches using the pattern `username/task-description`
- Test-Driven: Write tests first to meet the issue's Definition of Done (DoD)
- Atomic Commits: Strict use of Conventional Commits (one file per commit)
- Pull Requests: Opened via gh cli; PRs require 100% approval from GitHub Actions CI pipeline before merging
- Code Hosting: MDMV's Forgejo with a mirror on GitHub

### CI/CD & Tooling
**Local CI (Lefthook on commit):**
- cocogitto
- cargo fmt --fix
- cargo deny
- cargo audit
- cargo vet
- cargo check
- cargo clippy
- cargo nextest run

**GitHub Actions CI:**
- cargo fmt --check
- cargo check
- cargo clippy
- cargo deny
- cargo audit
- cargo nextest run
- cargo llvm-cov
- cargo mutants
- cargo semver-checks

**Release & Deploy:**
- Semantic Versioning
- GitHub Actions triggers release-plz on merge to main
- Automatic deployment to MDMV's Coolify

**Strict Linting:**
- [workspace.lints.rust] unsafe_code = "forbid"
- [workspace.lints.clippy] pedantic = "deny"

### Deployment Topologies
The suite must scale from a single VPS to distributed infra without changing the tech stack:

- 🌱 Seed (Minimal): Flora + PostgreSQL
- 🌿 Sprout (Core Collaboration): Flora + PostgreSQL + Valkey
- 🌳 Grove (Full Workspace): PostgreSQL + PGMQ + Valkey + RustFS + Meilisearch + ZITADEL
- 🌲 Forest (Large Org): Replicated PostgreSQL + Workers + Distributed RustFS + Valkey + Meilisearch + Observability + Multiple Flora nodes

### Development Phases & Milestones
**Excluded from v0.1:** AI, CRM, Calendar, Calls, MCP, Agents, Email Client, GitHub clone.

**First Milestone: Flora Seed v0.1 Requirements:**
Organizations, Workspaces, Users, Roles, OIDC, Channels, Messages, Threads, Notifications, Tasks, Files, PostgreSQL, SQLx, PGMQ, Valkey, RustFS, Meilisearch, Docker, Coolify, OpenTelemetry, AGPL license.

**Roadmap Phases:**
- Phase 0: Licensing + dependency audit
- Phase 1: Workspace/Identity
- Phase 2: Messaging
- Phase 3: Tasks
- Phase 4: Files + RustFS
- Phase 5: Search + Meilisearch
- Phase 6: Notifications + PGMQ + Valkey
- Phase 7: Documents
- Phase 8: Email
- Phase 9: Integrations
- Phase 10: AI feature flags
- Phase 11: Tauri
- Phase 12: Svelte 5

## Governance
This Constitution supersedes all other practices and guidelines within the Flora Workspace project. Amendments to this Constitution require:
1. Documentation of proposed changes
2. Review and approval by project maintainers
3. Migration plan for existing code (if applicable)
4. Updated version number following semantic versioning

All PRs/reviews must verify compliance with this Constitution. Complexity must be justified with reference to these principles. Project maintainers are responsible for ensuring ongoing adherence to these guidelines.

**Version**: 1.0.0 | **Ratified**: 2026-08-17 | **Last Amended**: 2026-08-17