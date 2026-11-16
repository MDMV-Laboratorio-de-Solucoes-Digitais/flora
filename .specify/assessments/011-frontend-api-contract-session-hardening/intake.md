# Intake: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Date:** 2026-11-15  
**Base:** `dev@556b78cc8` (post cycle 010, PR #213 merged, CI green)  
**Workflow:** mdmv-dark-factory SDD (R1→R7), autonomous 80/20

## Context
- Ciclo 010 entregou: compose/docs alinhados para PostgreSQL 18 (`uuidv7` nativo), smoke integrado `select org → workspace → task` evidenciado (`rc=0`, PG18; NV porta padrão por conflito externo), checks determinísticos frontend e Rust verdes localmente, artefatos SDD/validação/release.
- `OBJETIVO.md`/`ROADMAP.md` (011 candidato) priorizam: evidência remota/CI, ergonomia operacional do caminho documentado, e alinhamento frontend↔API + consistência de auth/contexto/tenancy no slice núcleo.

## Request
Executar o próximo ciclo 80/20 para consolidar o slice auth→org→workspace→task com maior confiabilidade contratual e verificabilidade determinística, mantendo escopo mínimo e registrando NVs.

## Inputs (evidência e auditorias)
- Artefatos 010: `specs/010-*`, `.specify/assessments/validations/releases/010-*`.
- Auditorias/subagentes (011): contrato frontend↔API, autoridade de sessão/tenancy backend, gaps em `flora-tests`/CI, testes frontend obsoletos, pesquisa multilíngue (Axum sessão+DB, clients TS tipados), síntese mdmv-dark-factory.
- Código-chave: `apps/frontend/src/lib/api/client.ts`, `.../state/AuthState.svelte.ts`, rotas/páginas auth/select-org/tasks, `flora-api/src/routes/{auth,org,workspace,tasks}.rs`, `flora-api/src/extractors/auth.rs`, `flora-tests/src/contract/*`, workflows CI.

## Constraints
- Sem mudanças em migrações `sqlx` aplicadas; sem OIDC/JWKS e2e real; sem expansão ampla de produto (realtime/files/search/wiring fora do slice).
- Sem `git add -A`; PR para `dev`, CI verde, sem auto-merge cego; claims remotos/ambientais não verificados = NV.
- Porta host `5432` pode permanecer ocupada externamente (NV operacional).
