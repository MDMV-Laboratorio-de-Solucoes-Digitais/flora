# Feature Specification: Seed Runnable Baseline (Dev)

**Feature Branch**: `[004-seed-runnable-baseline]`  
**Created**: 2026-11-15  
**Status**: Draft (assessment-backed, go — cycle 004)  
**Assessment**: `.specify/assessments/004-seed-runnable-baseline/{intake,research,decision}.md`

## Context

Flora possui implementação e specs avançadas em partes, mas não dispõe de baseline local reproduzível para dependências do Seed (Postgres/Valkey/Meilisearch) nem de consolidação documental atualizada de objetivo/roadmap. Isso dificulta verificação ponta a ponta, onboarding e priorização baseada em evidência.

## Goal

Entregar um recorte 80/20 de enablement: **documentação consolidada** (OBJETIVO/ROADMAP/pesquisa) e **baseline dev reproduzível** para subir dependências locais e executar a API com passos claros, sem prometer “verde” integral onde serviços/CI externos não foram exercitados.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Subir dependências locais (Priority: P1)
Como desenvolvedor, quero iniciar Postgres, Valkey e Meilisearch localmente com um comando reproduzível para rodar a API e migrações.

**Independent Test:** `docker compose up -d` (com Docker disponível) sobe serviços com healthchecks; portas locais acessíveis conforme compose.

**Acceptance Scenarios:**
1. **Given** Docker/Compose instalado, **When** executo `docker compose up -d` na raiz, **Then** postgres/valkey/meilisearch iniciam e reportam healthy (quando imagem provê healthcheck).
2. **Given** `.env` baseado em `.env.example`, **When** configuro URLs/keys compatíveis com o compose, **Then** `flora-api` consegue conectar e aplicar migrações no startup.

### User Story 2 — Direção e estado consolidados (Priority: P1)
Como mantenedor, quero OBJETIVO e ROADMAP atualizados com evidências e próximos passos para orientar ciclos seguintes.

**Independent Test:** Documentos existem, referenciam fontes/artefatos do repo e distinguem verificado de não verificado.

**Acceptance Scenarios:**
1. **Given** o repositório, **When** leio `OBJETIVO.md` e `ROADMAP.md`, **Then** encontro estado atual, gaps, não-objetivos e backlog priorizado (80/20).
2. **Given** o ciclo 004, **When** consulto assessment/spec, **Then** há decisão `go` rastreável e escopo delimitado.

### User Story 3 — Smoke path documentado (Priority: P2)
Como contribuidor, quero um checklist mínimo de smoke para validar que a API está viva e conectada às dependências.

**Independent Test:** Passos manuais documentados (health, logs de startup/migração) executáveis quando o ambiente está provisionado.

**Acceptance Scenarios:**
1. **Given** stack dev ativa e API rodando, **When** executo `GET /health*` e verifico logs, **Then** confirmo processo ativo e migrações aplicadas (conforme evidência observável).

## Requirements *(mandatory)*

- **FR-001:** Fornecer `docker-compose.yml` (dev) para Postgres, Valkey e Meilisearch com volumes, restart policy e healthchecks, sem expor segredos reais.
- **FR-002:** Manter/consolidar `OBJETIVO.md`, `ROADMAP.md` e `docs/research/2026-11-15-flora-seed-80-20-research.md` alinhados ao estado verificado em 2026-11-15.
- **FR-003:** Registrar assessment 004 (`intake`, `research`, `decision` go) e spec/plan/tasks do ciclo.
- **FR-004:** Documentar passos mínimos de execução local (compose, `.env`, `cargo run -p flora-api`, seed opcional) e limitações/NV.

## Non-goals
- Deploy/infra de produção, HA, mudanças em CI remota/proteções, merges/releases.
- Implementação de features de produto além do enablement de baseline (ex.: fanout WS completo, reescrita de auth) — ficam para ciclos seguintes.
- Inclusão de segredos/credenciais reais ou serviços pagos/proprietários.

## Risks & Notes
- Verificação de `docker compose` e smoke end-to-end depende de Docker/recursos locais (NV nesta sessão se não executado).
- Config de Meilisearch inclui chave de desenvolvimento no compose; não usar em ambientes não-locais.
- OIDC real requer provedor configurado; baseline dev foca infra de dados/cache/busca.
