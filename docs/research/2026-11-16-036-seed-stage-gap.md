# Seed stage gap — literal milestone list vs ADR-0001 authoritative mechanisms (2026-11-16)

**Type:** research only — no source, config or CI files were changed; this document is the only new file.
**Repo:** `flora` (Archived), branch `dev` @ `7e1f3dee9`. Working tree at research time: `OBJETIVO.md` and
`ROADMAP.md` modified (uncommitted), untracked `docs/research/2026-11-16-035-*.md`. No CodeGraph/reindex used.
**Question answered:** reconcile the user's *literal* Seed milestone list (Organizations; Users/Roles OIDC;
Channels/Messaging threads; Notifications **PGMQ+Valkey**; Tasks; Files **RustFS**; Search **Meilisearch**;
Infra **PostgreSQL/SQLx/Docker/Coolify/OpenTelemetry**; **AGPL**; **FASE 0–12**) with ADR-0001's
authoritative substitutions, and state precisely what "reaching Seed" means and whether it is reached today.
**Authority order:** ADR-0001 (Accepted) > executable evidence (`Cargo.toml`, code, `.github/workflows/`) >
`ROADMAP.md`/`OBJETIVO.md` > `README.md`/constitution milestone prose. Per `ROADMAP.md:5` and `AGENTS.md`,
README/old docs are partly aspirational; code and workflows win.

---

## 1. The two lists and the substitution rule

The literal list is the *historical* milestone wording that ADR-0001 §Context identifies at
`README.md:23-47` / `constitution.md:134-149`: it names heavy backends (PGMQ+Valkey, RustFS, Meilisearch,
OpenTelemetry) inside the Seed tier, contradicting the constitution's own tier table
(`constitution.md:42-47`: Seed = Flora + PostgreSQL; Valkey = Sprout; PGMQ/RustFS/Meilisearch/ZITADEL = Grove).
ADR-0001 §Decision 1 rules those mechanism names **superseded where they name a Seed mechanism**; §Decision 2
substitutes the mechanisms below; §Decision 3 makes "verified" an *executed-evidence* claim, not prose.

The current `README.md:27-35` and `constitution.md:138-147` have **already adopted** ADR-0001 wording
(mechanism-neutral; heavy backends "deferred to Sprout/Grove"). The literal list in the request therefore
describes a pre-ADR snapshot, not the checked-in docs. This table reconciles the two.

**Status legend.** **DONE** = authoritative mechanism implemented *and* backed by executed evidence in this
session or a merged, CI-green cycle. **PARTIAL** = authoritative mechanism present but a named sub-capability
or its verification is incomplete. **NV** = present/configured but not exercised by executed evidence.
**ABSENT** = literal mechanism intentionally not built (deferred/gated by ADR-0001; not a Seed blocker).

## 2. Gap table

| Milestone (literal / FASE) | Authoritative mechanism per ADR-0001 | Verified status | Evidence |
|---|---|---|---|
| **Organizations & Workspaces** (FASE 1) | Postgres tables via `sqlx`, membership/RBAC ownership; no substitution (literal name retained) | **DONE** | `README.md:27` `[x]`; `flora-core` init migration + org/workspace routes; `/api/v1/organizations` → 401 without token and smoke `select org → workspace → task` rc=0 (`ROADMAP.md:24`, `OBJETIVO.md:49`); DB contract `contract::auth::db_` in `rust.yml:73` |
| **Users & Roles (OIDC)** (FASE 1) | OIDC PKCE S256 + **verified** ID token (JWKS/`iss`/`aud`/`exp`/`sub`/`nonce`, redirect allowlist), DB sessions with `jti`; roles authoritative in Flora's own DB; external IdP optional | **PARTIAL** | OIDC half **DONE**: cycle 024 real verification, 5 deterministic contracts (`ROADMAP.md:48`, `OBJETIVO.md:65`, `README.md:28`, ADR-0001:31). Roles half **not done**: all 5 RBAC HTTP endpoints still placeholders (`flora-api/src/routes/rbac.rs:24-48`, `// Placeholder`); `AuthService` stub + workspace permission delegating to org scope (`seed-oidc-identity-gap.md` G6/G10/G11); real-OP e2e, JWKS rotation, `oidc_subject` UNIQUE/backfill = **NV** (ADR-0001:31, gaps doc NV-2/NV-4) |
| **Channels & Messaging (Threads)** (FASE 2) | Channels/messages with `thread_id` in Postgres; realtime = in-process hub; Valkey `PUBLISH` optional decoration, **not** a delivery plane | **DONE** (REST/thread contract); realtime resilience **NV** | `README.md:29`; cycles 013–015 REST slice + WS handshake + `after` catch-up (`OBJETIVO.md:52-54`, `ROADMAP.md:37-39`); `contract::messaging::db_` + `contract::websocket::db_` (`rust.yml:75,77`); Valkey has `PUBLISH` but no `SUBSCRIBE`/consumer (`seed-notifications-pgmq-valkey.md` F9; `ROADMAP.md:26`). Post-handshake revalidation / `/messages/sync` replay = NV |
| **Notifications (literal PGMQ + Valkey)** (FASE 6) | **In-app Postgres** `notifications` table with DB-enforced unread dedup (`20261116000003_*`); PGMQ + Valkey streams deferred to Sprout/Grove | **DONE** (PG in-app); literal PGMQ+Valkey **ABSENT by design** | ADR-0001:30; `README.md:30`, `ROADMAP.md:46` (cycle 022); repository `ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING` (`notification_repository.rs:87`); `contract::notifications::db_` (`rust.yml:81`). Scaffolding retired in cycle 027 (`ROADMAP.md:51`); `flora-notifications` crate no longer exists; `flora-core/src/config` has no `pgmq_queue`. Residual NV: cross-statement crash window, realtime push |
| **Tasks** (FASE 3) | Postgres tasks with description/assignee, tri-state unassign, best-effort `Assignment` in-app notification | **DONE** | `README.md:31`; cycles 011/020/021 (`OBJETIVO.md:50,59,60`, `ROADMAP.md:35,44,45`); assignment/dedup exercised under notification DB contracts; CPU unit suite (part of 104 tests) |
| **Files (literal RustFS)** (FASE 4) | **Local filesystem**, contained paths, streaming upload/download, SHA-256; RustFS/S3 deferred to Grove and feature-gated | **DONE** (local FS); literal RustFS **ABSENT by design** | ADR-0001:29; `README.md:32`, `ROADMAP.md:41-42` (cycles 017/018); `contract::files::db_` (`rust.yml:83`); no `rustfs`/`aws-sdk-*`/`object_store` in compose or crate deps, `rust_fs_url` is an app download path (`seed-files-rustfs.md` §2/F10/F14/F8). Residual NV: symlink/TOCTOU open-time containment, transactional quota, purge↔storage cleanup, load/restore |
| **Search (literal Meilisearch)** (FASE 5) | **Postgres FTS** (`search_tsv` + partial GIN), global multi-entity `message\|task\|file`, `source: "postgres"`; Meilisearch dormant, non-authoritative, gated G1–G4 | **DONE** (PG FTS authoritative); literal Meilisearch **ABSENT by design** | ADR-0001:28; `README.md:33`, `ROADMAP.md:47` (cycle 023); `search.rs:52,82,103` returns `source: "postgres"`; `contract::search::db_` (8 tests, local PG18 — `ROADMAP.md:22,47`); re-enablement gates G1–G4 in `seed-search-meilisearch-reconciliation.md` §4. NV: CJK/typo/facets/cross-engine parity, 2vCPU/4GB benchmark |
| **Infra — PostgreSQL + SQLx + Docker + CI** (FASE 0) | Same literal mechanisms; authoritative, no substitution | **DONE** (implementation); **PARTIAL** (CI test gate) | PostgreSQL 18 native `uuidv7()` migrations, `sqlx`, lean compose (`ROADMAP.md:16,34,40`); `rust.yml:24-33` runs fmt, clippy `--workspace --all-targets`, `cargo deny check bans licenses sources`, build; `ci.yml` frontend check/lint/unit/build. **Defect:** `rust.yml:35` runs `cargo test` → **0 tests** (root placeholder package); the **104 tests (43 ignored) only run with `--workspace`**, verified locally this session (`ROADMAP.md:20-21`, `OBJETIVO.md:82-84`, `035-global-progress-and-80-20.md:9,25,29-33`). This breaks ADR-0001 §3 "executed evidence" for every PR |
| **Infra — Coolify deploy** (FASE 0) | GHCR image + authenticated deploy webhook; production Compose with a `flora-api` service is ***not*** part of the verified Seed | **NV** (not exercised / blocked) | `docker-staging.yml` matches Coolify's documented pattern but fails in 100/100 runs (workflow error) and the fix is gated by explicit deploy authorization (`ROADMAP.md:27,66`); current `docker-compose.yml` defines only dependency services, no deployable `flora-api` service (`seed-infrastructure-otel-coolify.md` §0/§5, [S13]/[S14]). ADR-0001:38 marks frontend CI/dev smoke as the gate, not live deploy |
| **Infra — OpenTelemetry / observability** (FASE 0) | `tracing` + `TraceLayer` per-request spans + static `/metrics`; **no mandatory collector**. OTLP exporter/collector deferred | **PARTIAL** (authoritative in place); literal OTLP exporter **ABSENT** | ADR-0001:32; `routes/mod.rs:65` `TraceLayer::new_for_http()`; static `/metrics` + `/health*` → 200 this session (`ROADMAP.md:24`); phantom OTel deps removed from `Cargo.toml:161-162` (cycle 026). Real OTLP requires a receiver, aligned crate versions and a shutdown path (`seed-infrastructure-otel-coolify.md` §0/§4.5) — NV |
| **AGPL / License** (FASE 0) | AGPL-3.0-or-later; full LICENSE + NOTICE; `deny.toml` policy; `cargo deny check bans licenses sources` enforced in CI; cargo-vet/advisory informational | **PARTIAL** (core DONE, full FASE 0 closure NV) | ADR-0001:43-47; `README.md:35,38`; `LICENSE` full text + `NOTICE`, `deny.toml` with `AGPL-3.0-or-later`, gate at `rust.yml:31` (cycles 025/026, green runs cited in ADR). Open NVs: blocking cargo-vet + CODEOWNERS, RustSec advisories + SBOM on tag (informational/tag-only now — `rust.yml:85-110`, `ROADMAP.md:55-56`), §13 "Source" UI link and SPDX headers (`seed-license-dependency-audit.md` §5/§NV) |
| **FASE 7 Documents** | Not a Seed capability (P1/P2); design exists, implementation not merged | **ABSENT from Seed / NV** | `README.md:45`; `ROADMAP.md:60` cycle 035 is a *candidate* ("design exato pronto, ~30 arquivos"); explicitly "não priorizar agora" (`ROADMAP.md:68`) |
| **FASE 8 Email · 9 Integrations · 10 AI flags** | Out of Seed scope by constitution/ROADMAP (P2–P3) | **ABSENT (out of scope)** | `constitution.md:138` "Excluded for now: AI, CRM, calendar, calls, MCP, agents, email client, GitHub clone"; `ROADMAP.md:94,103-105`; `OBJETIVO.md:33-34` |
| **FASE 11 Tauri** | Deferred by **ADR-0002** (canonical Seed frontend is browser) | **ABSENT (deferred by ADR)** | `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md`; `README.md:49`; `ROADMAP.md:51,58` (criteria #1/#3/#4 NV) |
| **FASE 12 Svelte 5 (canonical frontend)** | Canonical rewrite target; CI = `bun install/check/lint/test:unit --project server/build` | **PARTIAL** | `README.md:50,65`; `ci.yml:15-24` green; 30 server tests, build ok (`ROADMAP.md:25`, `OBJETIVO.md:82`). Residual gaps: DM fix, `KanbanBoard` mock, `TaskFilter` stub (`ROADMAP.md:25`) |

## 3. What "reaching Seed" means under ADR-0001 — and the verdict

**Definition.** "Seed v0.1" is a *capability + verification* statement, not an infrastructure inventory:
1. **Capability layer** — the authoritative mechanisms are present: org/workspace tenancy; verified OIDC
   identity with DB sessions; channels/messages/threads; tasks; local-FS files with checksum; Postgres-FTS
   search with `source: "postgres"`; in-app Postgres notifications with DB-enforced dedup; tracing +
   TraceLayer + static `/metrics`; AGPL-3.0 with a `cargo deny` gate. PGMQ, Valkey delivery, RustFS/S3,
   Meilisearch-authoritative reads and OTLP export are **explicitly not Seed requirements** (ADR-0001
   §Decision 2 — Grove/Sprout profile-gated, re-enablement requires a new ADR); their absence is *not* a gap.
2. **Verification layer** — ADR-0001 §Decision 3 (echoed in §Decision 4's "no self-certification", NV marking
   and `ROADMAP.md:6,98`): fmt + strict clippy + unit tests, six `#[ignore]` DB-gated contract suites on
   disposable PostgreSQL 18, integrated dev smoke, canonical frontend CI, and per-cycle R4 artifacts — all as
   *executed* evidence. "CI green" without the suite actually running does not satisfy it.

**Is it reached today? Verdict: capability YES — verified Seed NO (not yet, fully).**
- The capability layer is substantially reached: all authoritative substitutions are implemented; 34 cycles
   merged; six DB suites in `rust.yml:37-83`, smoke rc=0, frontend CI ok, runtime `/health*`/`/metrics` 200,
   and this session executed `cargo test --workspace` = **104 tests (43 ignored)** green together with fmt,
   strict clippy, `cargo build -p flora-api` (`ROADMAP.md:20-25`, `OBJETIVO.md:47-86`). Under ADR-0001 the
   literal PGMQ/Valkey/RustFS/Meilisearch/OTLP items are *non-goals for Seed*, so their absence cannot block it.
- But ADR-0001's verification gate is **not satisfied as an enforced, self-certification-free claim** today:
  `rust.yml:35` runs `cargo test` (0 tests), so CI is green without executing the 104; the fix
  (`cargo test --workspace`, one line) is the top-ranked 80/20 action and is still pending
  (`ROADMAP.md:63-65`, `035-global-progress-and-80-20.md:43,61`). This single defect undermines every
  "CI green" claim used to close cycles/specs/ADRs (035 synthesis:33).
- Remaining open NVs keep "verified Seed" incomplete: RBAC endpoints still stubs; real-OP OIDC/JWKS rotation;
  cargo-vet blocking + CODEOWNERS, RustSec/SBOM on tag; Coolify deploy (workflow fails, authorization-gated);
  2 vCPU/4 GB load/OOM + restore drill; search CJK/typo/parity and file symlink/quota hardening; realtime
  post-handshake/replay; FASE 7 Documents (ADR-0001:59-60, `OBJETIVO.md:69`, `ROADMAP.md:60-68`).
- Therefore the accurate one-liner (matching `ROADMAP.md:72-74`) is: **"núcleo Seed entregue (mecanismos
  autoritativos no lugar, heavy backends adiados por ADR), verificação parcial"** — Seed is *substantively*
  reached in code, but **not ADR-0001-verified/reached** until the CI test gate executes the 104 tests and the
  listed NVs are either verified or explicitly accepted. Do not declare it green on prose.

## 4. NV / not validated in this research

- **NV-1** No build/test command was re-executed in this session; the 104-test result is taken from the
  session's verified results recorded in `ROADMAP.md:20-21`/`OBJETIVO.md:82-84` and the 035 synthesis.
- **NV-2** `OBJETIVO.md`/`ROADMAP.md` are modified and the 035 research docs untracked in the working tree;
  line numbers cited are from the files as read. No git operations were performed.
- **NV-3** The pre-implementation `seed-*.md` docs describe an earlier baseline (e.g. OIDC ID token unverified,
  PGMQ crate present, TraceLayer absent). Status above uses **current** code/workflows, which have since
  closed those gaps (cycles 023/024/026/027); the research docs are used only for the authoritative
  mechanism definitions and deferred-backend rationale.
- **NV-4** No live Coolify, real OIDC provider, Meilisearch, RustFS, Valkey, OTLP collector or 2 vCPU/4 GB
  load/restore drill was exercised; all such claims are marked NV, not estimated.

## 5. Evidence index

- `docs/adr/0001-seed-v01-mechanism-substitutions.md` — substitutions (§Decision 2), verification gate
  (§Decision 3), drift rule (§4), FASE 0 (§5), residual NVs (Consequences).
- `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md` — FASE 11 deferral.
- `ROADMAP.md:5,20-27,34-60,63-68,72-74,98` · `OBJETIVO.md:19-34,47-86` — operational truth, cycle log,
  verified results, CI defect, 80/20, Seed criterion.
- `README.md:14-50,68-80` · `.specify/memory/constitution.md:42-47,49-59,135-153` — literal vs adopted
  milestone wording and deployment tiers.
- `.github/workflows/rust.yml:24-35,37-83,85-110,112-174` · `ci.yml:15-24` · `docker-staging.yml` — gates,
  the `cargo test` defect, six DB suites, informational gates, browser smoke, frontend CI, deploy webhook.
- Code cross-check: `flora-api/src/routes/{search.rs:52,82,103,rbac.rs:24-48,mod.rs:65}`,
  `flora-core/src/repositories/notification_repository.rs:87`, `Cargo.toml:161-162`, migration
  `20261116000003_notifications_unread_dedup.sql`, `flora-core/src/config` (no `pgmq_queue`).
- Cycle research: `2026-11-16-seed-{search-meilisearch-reconciliation,oidc-identity-gap,notifications-pgmq-valkey,files-rustfs,infrastructure-otel-coolify,license-dependency-audit}.md`;
  synthesis `2026-11-16-035-global-progress-and-80-20.md:9,25,29-33,43,61,68`.

---

*End of research deliverable. No source, config or CI files were modified; no commits were made.*
