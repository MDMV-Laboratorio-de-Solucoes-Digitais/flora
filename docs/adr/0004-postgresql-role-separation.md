# ADR-0004 — PostgreSQL role separation: owner NOLOGIN + app role least-privilege

**Date:** 2026-11-17
**Status:** Accepted
**Scope:** Flora `flora` repository — Seed v0.1 database infrastructure

## Context

The Seed connects to PostgreSQL as the `postgres` superuser (`.env.example:24`,
`docker-compose.yml:17-19`, `.github/workflows/rust.yml:65`). The app role is also the
table owner (migrations run through the same pool, `flora-core/src/migrations/mod.rs:15-22`),
so PostgreSQL row-level security is inert: owners and superusers bypass policies. A prova
em DB descartável (`/tmp/opencode/dark-factory-80-20/rls-10-executed-proof.md`) confirmou
o estado: 11 tabelas com RLS `ENABLE`, **zero policies**, **zero FORCE**, dono = `postgres`.

The user explicitly authorized creating a dedicated Postgres role and writing ADRs.

## Decision

1. Introduce two roles, provisioned by a **one-shot bootstrap executed by a superuser**,
   kept **outside** `migrations/sql/` (sqlx migrations run as the pool role and cannot
   `CREATE ROLE`):
   - `flora_owner` — `NOLOGIN NOSUPERUSER NOCREATEROLE NOBYPASSRLS`; owns the schema and
     all tables; runs DDL/migrations; never used by a connection pool.
   - `flora_app` — `LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOBYPASSRLS`; receives only
     `SELECT/INSERT/UPDATE/DELETE` on API tables and `USAGE` on sequences via GRANT.
2. `ALTER TABLE ... OWNER TO flora_owner` is executed by the **superuser**, not by
   `SET ROLE flora_owner` (prova: `SET ROLE` falha com "must be owner of table
   _sqlx_migrations"). Bootstrap is idempotent (`DO $$ IF NOT EXISTS $$` + `ALTER ROLE`),
   verified by two consecutive runs (rc=0).
3. The app password is injected via `psql -v app_password="$FLORA_APP_PASSWORD"` — never
   written to the repository.
4. In development, `postgres` remains the **bootstrap/migration role only**; it is not the
   application runtime role. Changing the default runtime connection string is **not**
   part of this ADR (see ADR-0006 rollout).

## Considered options

| Option | Why rejected |
|---|---|
| Keep `postgres` as app role, rely on app-level `WHERE organization_id` | Leaves RLS inert; a single missed filter becomes a cross-tenant leak. |
| `flora_app` with `BYPASSRLS` | Reintroduces the bypass this ADR removes. |
| App as table owner + `FORCE RLS` | Owner can still `DROP POLICY`/disable RLS; no separation of duties. |

## Consequences

- Positive: smaller blast radius; RLS becomes applicable; permission errors surface as
  query failures. Proven in disposable PG18: roles created with the right attributes,
  13/13 tables owned by `flora_owner`, grants present.
- Negative: CI/test pipelines need the roles provisioned in their disposable PG instance;
  changing the runtime connection string is deferred to ADR-0006.
- Drift: capability "tenancy" now has a Postgres role backend; recorded per ADR-0001 §4.

## Verification

Executed (2026-11-17, disposable PG18): `\du`-equivalent role query (app: `rolcanlogin=t`,
`rolsuper=f`, `rolbypassrls=f`, `rolcreaterole=f`; owner: `rolcanlogin=f`); 13 tables owned
by `flora_owner`; bootstrap idempotency (2 runs). Nota: o gate `cargo test --workspace`
já está correto desde o ciclo 036 (`.github/workflows/rust.yml:41`) — não reabrir. **NV**
até adoção por CI/runtime: seis suítes DB-gated sob `flora_app` e o smoke integrado.

## Relationship

Pré-requisito do ADR-0005. ADR-0006 controla o rollout sem condicionar required checks.
