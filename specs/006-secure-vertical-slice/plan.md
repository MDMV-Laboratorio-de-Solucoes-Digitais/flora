# Plan: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Branch:** `[006-secure-vertical-slice]`  
**Status:** executed (partial) — 2026-11-15

## Phase A — Synthesize inputs
- [x] Consolidar auditorias (auth/tenancy, contrato frontend↔backend) e pesquisa de boas práticas.
- [x] Finalizar escopo executável: priorizar endurecimento JWT/tenant nas rotas críticas; deferir E2E UI por desalinhamento e dependências (token org-scoped).

## Phase B — Implement (completed scope)
1. [x] `AuthExtractor::require_org()` e adoção de `AuthExtractor` (JWT) nas rotas: workspaces, channels/messages, tasks, org, files, search, notifications.
2. [x] Membership + tenancy checks (incl. validação de `workspace_id` na criação de canal e tenancy em mutações/leitura sensíveis).
3. [x] WebSocket: contexto por Bearer JWT verificado (sem headers crus), rejeição de org nil.
4. [x] Ajustes de lint/compilação (`unused_results`/imports) e formatação.

## Phase C — Validate & handoff
- [x] Evidência local: `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54).
- [NV] Smoke e2e integrado (compose/DB/OIDC) e CI remota.
- [~] Artefatos 006 atualizados; próximo passo: empacotar em branch/PR no GitHub (canônico) e sync mirror.

## Follow-ups
- Implementar escopo de token/sessão org após seleção (ou fluxo equivalente) para habilitar rotas tenant-scoped pós-login.
- Alinhar contrato frontend↔backend para slice e2e (org/workspace context, endpoints/shapes/WS) ou fornecer smoke API/script atualizado.
- Atualizar `test-flora.sh`/contratos e testes de integração afetados pelo novo modelo de auth.
- Hardening adicional WS (membership em subscribe, fanout) conforme necessidade.
