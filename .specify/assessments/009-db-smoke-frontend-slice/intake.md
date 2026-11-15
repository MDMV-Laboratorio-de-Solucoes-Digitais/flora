# Intake: Flora 80/20 Cycle 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Date:** 2026-11-15  
**Status:** go (implemented; pending PR/CI/merge)

## Captured request
Executar o próximo ciclo 80/20 (após ciclo 008, `dev@fb45e9700`) seguindo o fluxo mdmv-dark-factory para este repositório, com pesquisa extensiva multi-idioma e orquestração de subagentes, avançando verificabilidade e caminho feliz mais próximos do integrado.

## Scope (delivered)
1. **Cobertura DB de `select-organization`:** testes automatizados env-gated/`#[ignore]` (e job CI com Postgres) para sucesso/rotação, membership negada e sessão ausente/inativa/mismatch.
2. **Smoke integrado reproduzível:** execução de `docs/dev-smoke.md` (`select org → workspace → task`) com evidência (rc=0) em stack efêmera com portas alternativas; NV/nuances de compose/compatibilidade registradas.
3. **Fatia mínima frontend↔backend:** callback preserva/envia `state`, seleção de organização com rotação/troca de token e slice `workspace → tarefa` no `apps/frontend`, com guards mínimos.

## Source and type
- **Source:** `OBJETIVO.md` / `ROADMAP.md` (ciclo 009 recomendado), `specs/008-select-org-tests-and-dev-smoke/`, `.specify/assessments/008-*`.
- **Type:** ciclo técnico (verificabilidade + integração dev/runtime + alinhamento contrato mínimo).

## Constraints / NV
- Smoke via `docker compose` documentado bloqueado no ambiente por conflito de porta `5432`; migração falhou em `postgres:16-alpine` por ausência de `uuidv7()` (evidenciado em pg18). NV: compatibilidade compose/pg16-alvo e execução do job `db-tests` no CI remoto do PR.
- Frontend: verificação determinística completa (`check/lint/build/tests`) dependente de toolchain/ambiente; alterações implementadas e tipadas, `cargo fmt/check` e testes Rust ok localmente.

## Subagent inputs (completed)
- `ses_e1e8f0341ffeS9O37ZKM946HT1` dark-factory workflow/gates (SDD/R1-R7, NV discipline, PR/CI/merge hygiene)
- `ses_e1e8f0341ffdZw46QTHDDb5BhM` DB-backed tests + CI Postgres audit (env-gated/ignore, fixtures, `--test-threads=1`, job separado)
- `ses_e1e8f0341ffcqTrfH71HTjgehq` frontend minimal happy-path audit (state/callback/select-org/token rotation/workspace-task, guards)
- `ses_e1e8f0340ffe4m9OxyO9VZASaF` pesquisa multi-idioma (axum/sqlx/Postgres, GitHub Actions services, SvelteKit bearer/org selection)
- `ses_e1e8f0340ffdtK4QZLcUFWjhpp` viabilidade smoke: no-go as-is por `5432` ocupado; sequência/risco documentados

## Outcome
- Artefatos 009 em `.specify/assessments/009-db-smoke-frontend-slice/` e `specs/009-db-smoke-frontend-slice/`.
- Implementação: DB-backed tests + `db-tests` CI job, smoke integrado executado (rc=0, alternativa), fatia frontend mínima.
- Entrega pendente: PR GitHub para `dev`, CI verde (incl. `db-tests`), merge e sync `origin/dev`.
