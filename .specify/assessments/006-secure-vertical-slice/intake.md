# Intake: Flora 80/20 Cycle 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Date:** 2026-11-15  
**Status:** draft (in progress; audits/research subagents running)

## Captured request
Executar o próximo ciclo 80/20 (após ciclo 005 de reframe estratégico) para aproximar a Flora de um **Seed utilizável, seguro e verificável** como alternativa self-hosted ao Macro para PMEs em VPS barata (Rust + PostgreSQL, Svelte 5), com foco em debloat, consistência e evidência.

## Restated idea
Implementar/viabilizar uma **fatia vertical mínima segura**: unificar autenticação e contexto de tenant (JWT/sessão/org/workspace) nas rotas críticas — reduzindo confiança em headers crus — e tornar verificável um fluxo núcleo ponta a ponta em ambiente dev (`workspace → canal → mensagem` ou `tarefa`, o que for mais próximo do estado atual), alinhando contrato frontend↔backend no caminho feliz.

## Source and type
- **Source:** handoff de `specs/005-macro-alternative-mvp/plan.md`, `ROADMAP.md` (P0), `OBJETIVO.md`; análise prévia do codebase; auditorias/research em andamento (subagentes).
- **Type:** ciclo técnico (segurança/consistência + enablement de fluxo e2e verificável).

## Known context (evidence)
- P0 identificado: múltiplos handlers usam `X-User-ID`/`X-Organization-ID` em vez de extratores JWT validados; lacunas em realtime fanout, binário de arquivos, wiring busca/notificações e desalinhamento frontend↔backend (ver `OBJETIVO.md`, research 005, auditoria prévia).
- Infra dev: `docker-compose.yml` (Postgres/Valkey/Meilisearch) disponível; runtime e2e NV neste ambiente até execução.

## Unknowns / pending inputs
- Resultado detalhado das auditorias (auth/tenancy por rota com file:line; contrato frontend↔backend e slice recomendado) e pesquisa de boas práticas Axum/SvelteKit — **pendente (subagentes em background)**.
- Escolha final do slice (`mensagem` vs `tarefa`) e estratégia de migração compatível (headers dev vs JWT-only) dependem dos achados.

## Desired outcome
- Assessment 006 com decisão `go` e escopo acotado.
- Especificação/plan/tasks do slice seguro e verificável.
- Implementação incremental com evidência (testes/comandos/smoke) dentro do que for autonomamente viável; itens bloqueados por ambiente/serviços marcados NV.
