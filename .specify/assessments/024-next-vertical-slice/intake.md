# Intake: 024 — Next vertical slice (Seed v0.1 milestone)

**Date:** 2026-11-16  
**Baseline (expected):** `dev@` 023 merge commit (cycle 023 in flight at time of writing).  
**Directive:** advance Flora to a *verifiable* "First Milestone: Flora Seed v0.1" via 80/20 cycles, autonomous decisions.

## Inputs (R1 Seed research, 8 background agents)
- `docs/research/2026-11-16-seed-license-dependency-audit.md` (FASE 0)
- `docs/research/2026-11-16-seed-oidc-identity-gap.md`
- `docs/research/2026-11-16-seed-notifications-pgmq-valkey.md`
- `docs/research/2026-11-16-seed-files-rustfs.md`
- `docs/research/2026-11-16-seed-infrastructure-otel-coolify.md`
- `docs/research/2026-11-16-seed-search-meilisearch-reconciliation.md`
- Code-vs-PHASES gap matrix and PHASES/constitution reconciliation (session reports)

## Milestone reconciliation (evidence-based)
`README.md:23-47` and `constitution.md:134-149` name Meilisearch/PGMQ/Valkey/RustFS as "Seed", but the repo's
own authority order (`ROADMAP.md:5-6`, `AGENTS.md`, `constitution.md:38-43`) makes code + Cargo.toml + CI +
constitution tiers normative: **Seed = Flora + PostgreSQL**, with local FS, PG FTS, and in-app notifications.
Verified Seed v0.1 = consistent auth/tenancy + one core e2e flow + local binary files + PG FTS + in-app notifs
+ deterministic gates (fmt/clippy/unit + six DB-gated PG18 suites + frontend CI).

## Candidate slices (80/20)
1. **S1 — OIDC/ID-token verification (recommended):** the single highest-value decisive gap. Today the ID token is
   only base64-decoded and trusted (`flora-api/src/routes/auth.rs:201-215,398-414`); no `iss`/`aud`/`exp`/`nonce`
   or signature/JWKS verification. `openidconnect 4.0` is declared but unused. Use the declared crate + mocked
   discovery/JWKS fixtures; add negative tampered-token contracts. Converts the biggest PARTIAL into IMPLEMENTED.
2. **S2 — Phase 0 enforcement:** `cargo audit`/`cargo vet` in CI + `deny.toml` + decide the 442 exemptions.
3. **S3 — Seed decision record + docs reconciliation ADR:** mechanism substitutions, verification gate, drift rule.
4. **S4 — OTel honest accounting:** fix the nonexistent `opentelemetry-otlp 0.22` dep, zero-cost `TraceLayer`, no heavy collector.
5. **S5 — Browser-backed vertical e2e** (`workspace→channel→message→task→file`) + Tauri deferral decision.

## Constraints
- No new mandatory services; cheap VPS; deterministic tests (no live IdP needed); strict lints; no blind merge;
  CI green; explicit-path staging; preserve other agents' uncommitted work.
