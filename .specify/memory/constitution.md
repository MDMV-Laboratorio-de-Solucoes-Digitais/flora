<!-- Sync Impact Report
- Version change: 1.1.0 → 1.2.0
- Modified principles:
  - IV. Code Rigor & Linting Standards (Added Svelte 5 UI/UX rigor requirements, error states, and a11y)
- Added sections:
  - Added section: Visual Identity & UI/UX Design System
- Removed sections: None
- Follow-up TODOs: None
-->
# Flora Workspace Constitution

## Core Principles

### I. Project Vision & Philosophy (Debloat & Vendor Unlock)
Flora Workspace is a complete ecosystem of lightweight, free, and open-source (AGPL) enterprise applications. It is a fork of the "Macro-inc/macro" project, completely debloated from heavy, proprietary dependencies and AWS vendor lock-in. Core Philosophy: "This is a collaborative forest. The more seeds and trees planted here, the more fertile the soil becomes." It must run flawlessly on a cheap VPS.

### II. The VPS Test (Definition of Success)
The ultimate test of "production readiness" is deploying Flora on a completely fresh, cheap VPS without AWS, without mandatory proprietary services, and without manual intervention beyond initial configuration (via `docker compose up` or Coolify). PostgreSQL, PGMQ, Valkey, RustFS, Meilisearch, and Flora must boot up automatically. Backup, restore, upgrade, and rollback must also work flawlessly.

### III. Trait-Driven Architecture
The architecture is a Modular Monolith with Cargo Workspaces separated into Vertical Slices. Instead of direct implementations, the system must use Traits to define required functionalities, allowing services to be easily swapped out. This promotes loose coupling, testability, and architectural flexibility.

### IV. Code Rigor & Linting Standards (NON-NEGOTIABLE)
The backend enforces extreme compiler rigor via strict `rust` and `clippy` lints. The Svelte 5 frontend must establish an absolutely equivalent level of strictness.
- **Backend:** Enforced `deny` on `unsafe_code`, `missing_docs`, `unwrap_used`, `expect_used`, `panic`, and all `clippy::pedantic` lints.
- **Frontend (Svelte 5):** Strictest possible TypeScript/Svelte configurations: `strict: true`, no `any` types, no unsafe assignments, mandatory `svelte-check`, strict ESLint rules matching Clippy's pedantic level, forbidden `console.log` (except via proper loggers), and explicit handling of all Promises/fallible operations.
- **UI/UX Rigor:** 
  - **Error States:** Every component must have a fallback UI (e.g., reconnecting WebSocket alerts).
  - **Accessibility (a11y):** Mandatory ARIA attributes, keyboard navigation, and screen reader support (leveraging shadcn/ui and Bits UI).
  - **Zero Console Errors:** Graceful handling of all Promise rejections via Svelte 5 Error Boundaries or global toast notifications. Raw `console.log` and unhandled exceptions are strictly forbidden.

### V. Spec-Driven & Test-First Development
All development follows Spec-Driven Development tracked via GitHub Issues. TDD is required: Write tests satisfying the "Definition of Done" before implementation. Tests must be written → User approved → Tests fail → Then implement.

### VI. Atomic Commits & PR Discipline
Strict use of Conventional Commits. Use `git add <specific_file>` for exactly ONE file per commit. Strictly no `git add .` unless human-approved. Pull Requests opened via `gh cli`. Merge to main requires 100% CI pipeline approval. GitHub Actions handles release-plz and Coolify deployment.

## Deployment Tiers
The suite grows from a single VPS to a distributed infrastructure without changing the tech ecosystem:
- **🌱 Seed (Minimal installation):** Flora + PostgreSQL
- **🌿 Sprout (Core collaboration):** Flora + PostgreSQL + Valkey
- **🌳 Grove (Full workspace):** PostgreSQL + PGMQ + Valkey + RustFS + Meilisearch + ZITADEL
- **🌲 Forest (Large organization):** Replicated PostgreSQL + workers + distributed RustFS + Valkey + Meilisearch + observability + multiple Flora nodes

## Technology Stack & Architecture
- **Architecture:** Modular Monolith with Cargo Workspaces separated into Vertical Slices.
- **Backend:** Rust (Axum, thiserror, anyhow, tracing export OTLP).
- **Frontend:** Currently SolidJS, but planning a strangler fig pattern rewrite to **Svelte 5 (Runes Mode) + Tailwind CSS + shadcn/UI**. Desktop app via Tauri v2.
- **Database:** PostgreSQL (Canonical production DB, relational core tables + JSONB for non-relational) using `sqlx` (No ORM). SQLite as a fallback for local/desktop/offline.
- **Message Broker / PubSub:** PostgreSQL (pgmq) replacing AWS SQS/SNS. Valkey replacing Redis/Kafka.
- **Storage:** RustFS (production default ObjectStorage, S3-compatible) or LocalFileSystem.
- **Search & Identity:** Meilisearch (replacing ElasticSearch) and Zitadel (replacing FusionAuth).
- **Email:** Zoho Mail for corporate SMTP, Listmonk + external API (SendGrid/Brevo) for marketing. No self-hosted SMTP/Stalwart overhead.
- **Observability & CI/CD:** OpenTelemetry Collector -> VictoriaMetrics + Grafana + Uptime Kuma. CI via Lefthook (local) and GitHub Actions. Deployment via Coolify.
- **AI Features:** Activated via feature flags, utilizing external APIs/LLMs (or Ollama), but never strictly self-hosted within the core.

## Visual Identity & UI/UX Design System

### Brand Philosophy & Visual Metaphor
Flora Workspace is built on the concept of organic, self-sustained growth. The visual language must communicate lightweight agility, robust security, and natural scalability.
- **The Metaphor:** The UI should feel like fertile soil—unobtrusive and foundational. Data and interactions are the seeds and plants.
- **The Vibe:** Corporate but approachable. Clean, snappy, and distraction-free. It should look perfectly at home on a minimal Tauri desktop app or a standard web browser.
- **The Promise:** Zero lag, zero bloat, zero vendor lock-in.

### Color Palette (The "Forest" Spectrum)
Utilizing Tailwind CSS and shadcn/ui, the core palette revolves around CSS variables that switch seamlessly between Light and Dark modes:
- **Flora Seed (Primary Accent):** Vibrant green (`#10b981` / Emerald 500) for primary actions (e.g., "Create Workspace").
- **Flora Forest (Deep Backgrounds):** Dark pine green/gray (`#064e3b` to `#022c22` / Emerald 900 to 950) for sidebars and dark mode backgrounds to reduce eye strain.
- **Fertile Soil (Base Neutrals):** Grays with a slight warm (brownish/earthy) or cool (slate) undertone. Light Mode Base: `#f8fafc` (Slate 50); Dark Mode Base: `#0f172a` (Slate 900).

### Typography
Typography prioritizes readability for long-form reading and dense data visualization:
- **Primary Font (UI & Content):** Inter or Geist (highly legible, modern sans-serif).
- **Monospace Font (Code & Logs):** JetBrains Mono or Geist Mono.
- **Hierarchy:** `h1` to `h3` use slightly tight tracking (`tracking-tight`) and semi-bold weight. `body` uses regular weight and relaxed line-height (`leading-relaxed`) for chat readability.

### UI/UX Core Principles
- **Progressive Disclosure (Scaling with Deployment):** UI adapts based on the deployment tier. Seed/Sprout UI hides complex administrative panels, behaving like a simple chat app. Grove/Forest UI unlocks advanced settings (RustFS config, Meilisearch index tweaking) using shadcn/ui Tabs and Accordions.
- **Digital Sovereignty Indicators:** When external AI is invoked, show a distinct visual indicator (e.g., subtle purple sparkle) to denote data temporarily leaving the local environment. For local data, use instant, optimistic UI updates (powered by Svelte 5 Runes) to make the app feel native.
- **The "VPS Test" Performance Constraints:** No frontend backend polling. Rely exclusively on Valkey Pub/Sub pushed via WebSockets for real-time notifications. Use Svelte 5 Granular Reactivity (`$state`, `$derived`) to update specific DOM nodes without full-page repaints.

### Component Architecture (shadcn/ui + Svelte 5)
- **Navigation & Layout:** 
  - **App Shell:** Collapsible left sidebar containing Workspace switching, Channels, Direct Messages, and Modules (Tasks, Files).
  - **Header:** Minimalist. Contains global search (Meilisearch), context title, and user profile.
  - **Right Panel:** Used contextually for Thread replies or Task details, sliding in without obstructing the main view.
- **Kanban Task Board:** Standard shadcn/ui Cards for tasks. Drag-and-drop implemented with lightweight Svelte actions. Drop zones highlight with Flora Seed color.
- **Real-Time Chat & Messaging:** Clean, full-width message rows optimizing horizontal space. Integrated markdown support for rich text. File uploads via drag-and-drop with shadcn/ui progress bars, streaming directly to RustFS.

### Key User Journeys
1. **The "First Seed" (Initial Setup):** An Admin runs `docker compose up` and accesses the web UI. They see a minimalist setup screen (shadcn/ui Card). They create an Admin account and name the workspace. This grants immediate access with zero AWS keys or complex DB configurations.
2. **Feature-Flagged AI Interaction:** User clicks "Summarize Thread". A dialog warns that an external API is used (Feature Flag: ON). The user accepts, and a shadcn/ui Skeleton loader appears. The summary is injected into the chat with a specific visual tag indicating it was AI-generated.

## Development Methodology & Code Standards

### Linting Configuration
**Backend (Rust) Rules:**
```toml
[workspace.lints.rust]
unsafe_code = "deny"
missing_docs = "deny"
missing_debug_implementations = "deny"
unreachable_pub = "deny"
unused_results = "deny"
unused_qualifications = "deny"
trivial_casts = "deny"
trivial_numeric_casts = "deny"
unused_extern_crates = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
nursery = { level = "warn", priority = -1 }
allow_attributes = "deny"
allow_attributes_without_reason = "deny"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
fallible_impl_from = "deny"
clone_on_ref_ptr = "deny"
dbg_macro = "deny"
print_stdout = "warn"
use_self = "deny"
wildcard_dependencies = "deny"
multiple_crate_versions = "deny"
```

### Processes & Milestones
- **Prioritization:** Task selection based on Value vs. Effort.
- **Branching:** `username/task-description`.
- **First Milestone (Flora Seed v0.1):** Organizations, Workspaces, Users, Roles, OIDC, Channels, Messages, Threads, Notifications, Tasks, Files, PostgreSQL, SQLx, PGMQ, Valkey, RustFS, Meilisearch, Docker, Coolify, OpenTelemetry, AGPL. (Excluded for now: AI, CRM, calendar, calls, MCP, agents, email client, GitHub clone).

### Development Phases Roadmap
- **Phase 0:** Licensing + dependency audit
- **Phase 1:** Workspace/Identity
- **Phase 2:** Messaging
- **Phase 3:** Tasks
- **Phase 4:** Files + RustFS
- **Phase 5:** Search + Meilisearch
- **Phase 6:** Notifications + PGMQ + Valkey
- **Phase 7:** Documents
- **Phase 8:** Email
- **Phase 9:** Integrations
- **Phase 10:** AI feature flags
- **Phase 11:** Tauri
- **Phase 12:** Svelte 5

## Governance
This Constitution supersedes all other practices and guidelines within the Flora Workspace project. Amendments to this Constitution require documentation of proposed changes, maintainer review and approval, a migration plan for existing code (if applicable), and an updated version number following semantic versioning. All PRs/reviews must verify compliance with this Constitution. Complexity must be justified with reference to these principles. Project maintainers are responsible for ensuring ongoing adherence to these guidelines.

**Version**: 1.2.0 | **Ratified**: 2026-08-17 | **Last Amended**: 2026-08-22
