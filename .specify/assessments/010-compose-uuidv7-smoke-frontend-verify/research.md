# Research: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Date:** 2026-11-15  
**Inputs:** cycle 009 handoff/evidence, codebase audit, subagents `ses_e1e70c42bffeEeB6ykdz0lHRlg`, `ses_e1e70c429ffelej5ra63x4mjkr`, `ses_e1e70c421ffe8wdiTy72gocnXw`, `ses_e1e70c41effe44U20bkhSQ83U5`, `ses_e1e70c41cffe7ciYAWvFvne7bG`, `ses_e1e6f8d94ffeYNn6mu2GT06pKa`, `ses_e1e6f8d92ffeqp08mld5uEDEZy`, and loaded skills (`postgres-safe-migration`, `sqlx-code-review`, `svelte-testing`).

## Dark-factory workflow/gates
- Ciclo deve seguir SDD R1→R3→R4→R7: assessment com `decision.md` `go`, `specs/<NNN>-<slug>/{spec,plan,tasks,checklists}`, validação aprovada e release com rastreabilidade/rollback; `converge` append-only quando aplicável. Gates determinísticos autoritativos, sem auto-merge cego, PR para `dev` com CI verde e sync mirror; claims remotos/runtime sem evidência = **NV**.

## `uuidv7` / compose / migrações (auditoria + pesquisa)
- **Causa-raiz:** `docker-compose.yml` usava `postgres:16-alpine`, mas `flora-core/src/migrations/sql/20240818000001_init_schema.sql` define `DEFAULT uuidv7()` em múltiplas tabelas (PKs). `uuidv7()` é nativo no **PostgreSQL 18+**; em PG16 a migração falha no fresh install (evidenciado no ciclo 009). CI `db-tests` já usa `postgres:18` e smoke alternativo em PG18 passou (rc=0).
- **Imutabilidade `sqlx`:** `sqlx::migrate!` embute/valida migrations; alterar migração aplicada quebra checksum/histórico. Como a init falha antes de qualquer migration posterior em PG16 fresh, uma nova migration compat **não** resolve fresh install atual sem bootstrap pré-migrate ou mudança de baseline (arriscado). Polyfills/extensões (`pg_uuidv7`, SQL/PLpgSQL) existem, mas exigem cuidado de semântica/monotonicidade, disponibilidade em imagem e bootstrap antes da init.
- **Decisão 80/20:** alinhar o caminho documentado para **PostgreSQL 18** (`postgres:18-alpine` no compose, docs atualizados), sem editar init migration. Documentar reset de volume PG16→18 (`docker compose down -v` em ambiente descartável) e mitigação de conflito de portas. Isso torna fresh install + smoke documentado consistente com schema/CI.

## Smoke/dev runtime (probe + pesquisa)
- Probe: host `5432` ocupado (externo), `.env` ausente, 6379/7700/3000 livres, sem containers/volumes flora. Smoke documentado no-go as-is por porta+imagem; viável com compose compatível, projeto/volumes isolados e env/JWT consistente. Boas práticas: `--wait`/healthchecks, `docker compose port`/override para portas efêmeras, não tocar DB não-descartável, capturar logs/rc e teardown seletivo.

## Frontend determinístico (auditoria)
- CI (`.github/workflows/ci.yml`, `apps/frontend`): `bun install`, `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build` (sem e2e, sem projeto client Vitest).
- Riscos/gaps: `vite.config.ts` server project inclui `src/**/*.{test,spec}` e pode não cobrir `tests/unit|integration/**`; alguns testes podem estar defasados pós-009; alinhamento fino de tipos/erros/contratos do slice auth→org→workspace→task pode ser incremental. Verificação determinística viável sem backend: check/lint/build e testes unitários node/server conforme configuração.

## Riscos/NV
- Porta 5432 ocupada no ambiente atual (externo) impede execução literal na porta padrão; smoke deve ser evidenciado por caminho equivalente documentado/parametrizado com NV explícito. Volumes PG16 existentes exigem reset consciente. OIDC/JWKS e2e real, CI remoto além do PR, e paridade frontend ampla permanecem NV/fora de escopo.
