# Research: 024 — Seed v0.1 decisive gaps (R1 synthesis)

**Sources:** `docs/research/2026-11-16-seed-*.md` (7 delivered; FASE 0 retry pending), code-vs-PHASES gap matrix,
PHASES/constitution reconciliation, 023 artifacts.

## Convergence
- **Milestone authority:** README's "First Milestone"/FASE lists are historical/aspirational. Authority order is
  code/Cargo.toml/CI/constitution (`ROADMAP.md:5-6`, `AGENTS.md`). Constitution tiers (`constitution.md:38-43`)
  define Seed = Flora + PostgreSQL; PGMQ/Valkey/RustFS/Meilisearch are Sprout/Grove.
- **Mechanism substitutions already evidenced:** local FS (streaming SHA-256 upload/download) instead of RustFS;
  PG FTS (tenant/soft-delete safe, CI-enforced) instead of Meilisearch; in-app Postgres notifications with DB-enforced
  unread dedup instead of PGMQ/Valkey. Valkey Pub/Sub is at-most-once and would be a reliability downgrade.
- **Highest-value real gap — identity:** the ID token is only base64-decoded and trusted
  (`flora-api/src/routes/auth.rs:201-215,398-414`); no signature/`iss`/`aud`/`exp`/`nonce`, no discovery/JWKS/`kid`
  rotation, no redirect allowlist, identity by email rather than `sub`. `openidconnect 4.0` is declared
  (`Cargo.toml:171`) but unused. PKCE S256, one-time transaction store, DB sessions + `jti` rotation already exist.
- **Other gaps:** Phase 0 audit scaffolding exists (`supply-chain/config.toml` with 442 exemptions) but no
  `deny.toml`, empty `audits.toml`, no CI gate; OTel crates declared but unused and `opentelemetry-otlp 0.22`
  does not exist on crates.io; `flora-notifications` XADD code is unwired; Tauri code only in non-canonical `apps/web`.
- **No live IdP needed:** deterministic verification via local RSA/JWKS fixtures + mocked token endpoint + the
  existing `dev-smoke-session.sql` bootstrap; tampered-token negative tests.

## Decision drivers
- Security: a trusted-then-decoded ID token is the only gap that can admit an unauthorized session.
- Effort: reuses an already-declared crate and existing session infrastructure; bounded to `auth.rs` + config + contracts.
- Milestone: converts "Users & Roles (OIDC)" from PARTIAL to IMPLEMENTED with executable evidence.

## Risks/NVs
- OIDC Core §3.1.3.7 allows TLS-server validation in place of ID-token signature for direct token-endpoint tokens,
  but the default issuer is `http://localhost:8080`, so that substitute is unavailable in the default profile;
  `iss`/`aud`/`exp`/`nonce` are unconditional MUSTs.
- Real-OP e2e/JWKS rotation, `oidc_subject` uniqueness/backfill, browser FE round-trip, logout/refresh semantics,
  workspace-scoped RBAC remain NVs.
