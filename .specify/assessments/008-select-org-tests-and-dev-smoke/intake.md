# Intake: Flora 80/20 Cycle 008 — Select-Organization Tests & Reproducible Dev Smoke

**Date:** 2026-11-15  
**Status:** processed (research consolidated, decision `go`, scoped implementation delivered)

## Captured request
Executar o próximo ciclo 80/20 (após merge do #210/ciclo 007) para avançar um Seed utilizável, seguro e verificável como alternativa self-hosted ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5/Tauri depois), com pesquisa extensiva multi-idioma, orquestração de subagentes, e seguindo o fluxo mdmv-dark-factory para este repositório.

## Restated idea
Consolidar o fluxo de seleção de organização entregue no ciclo 007 com **testes determinísticos focados** (`select-organization`: sucesso, membership negada, sessão inválida/inativa/ausente, org nil) e um **caminho de smoke reproduzível em dev** (docker-compose + seed + bootstrap de token/sessão sem OIDC real, quando viável) para o slice `org → workspace → task`. Alinhar contrato mínimo adicional apenas se de baixo custo.

## Source and type
- **Source:** handoff de `specs/007-org-scoped-auth-e2e/{spec,plan,tasks}.md`, `OBJETIVO.md`, `ROADMAP.md`; lacunas: testes dedicados ausentes, smoke integrado NV, frontend/contrato desalinhado.
- **Type:** ciclo técnico (qualidade/verificabilidade + enablement de smoke).

## Inputs incorporated
- `ses_e1e9a5437ffeDtndGc5NhfHNUB` (flora-tests/DB patterns)
- `ses_e1e9a5435ffexYwl8z3bRWNTW3` (dev-seed/compose/token bootstrap feasibility)
- `ses_e1e9a542bffe1iScHCp3nDbNe7` (frontend minimal alignment)
- `ses_e1e9a5428ffesJ4ikPt7V732Nm` (multi-language research)
- `ses_e1e9a5426ffeFHRHkBZGVF8s6o` (mdmv-dark-factory workflow for flora)
- `ses_e1ea41e08ffeHj1Q40A2fgILJO` (org-scoped auth/session patterns)

## Outcome (scoped)
- Assessment 008: decisão `go`, escopo acotado.
- Testes CI-safe adicionados para `select-organization` (missing auth, nil org).
- Caminho de smoke dev reproduzível documentado/bootstrável sem OIDC (`scripts/dev-smoke-session.sql`, `docs/dev-smoke.md`, `test-flora.sh`); execução integrada **NV** neste ambiente.
- Casos DB-backed e alinhamento frontend completo registrados como lacunas/NV para próximos ciclos.
