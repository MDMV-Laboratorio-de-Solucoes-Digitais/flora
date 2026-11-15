# Tasks: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Status legend:** `[X]` done, `[~]` partial, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Research & scoping
- [X] T001 Criar estrutura de assessment/spec 006 (intake, research, decision, spec/plan/tasks/checklist).
- [X] T002 Auditoria auth/tenancy por rota (subagente `ses_e1eb2b2a3ffejE2sCEf1o3Vbv9`) — sintetizada.
- [X] T003 Auditoria contrato frontend↔backend e slice recomendado (subagente `ses_e1eb2b2a2ffeM7nlCUTEZ1NOUp`) — sintetizada.
- [X] T004 Pesquisa Axum/SvelteKit auth, realtime mínimo e migração header→JWT (subagente `ses_e1eb2b29bffe6XnW1Y8015ZFgq`) — sintetizada.
- [X] T005 Consolidar achados e definir escopo executado (backend auth/tenant hardening; E2E UI deferido/NV por desalinhamento).

## Phase 1 — Auth/tenancy enforcement (slice)
- [X] T010 Enforce contexto autenticado via `AuthExtractor` (JWT) + `require_org()` nas rotas tenant-scoped.
- [X] T011 Aplicar membership/tenant checks e validar tenancy de recursos no escopo (workspaces, channels/messages, tasks, files, search, notifications); remover dependência de headers `X-User-ID`/`X-Organization-ID` nessas rotas.
- [X] T012 Ajustes mínimos de compatibilidade de compilação/lints (`unused_results`, imports) — testes de contrato header-based existentes podem requerer atualização futura (ver lacunas).
- [X] T013 WebSocket: extração de contexto por Bearer JWT verificado (sem trust em headers), rejeição de org nil.

## Phase 2 — Core flow e2e
- [~] T020 Contrato backend endurecido para slice crítico (workspace→channel→message / tasks) — **parcial** (auth/tenancy), mas fluxo e2e completo depende de token org-scoped pós-seleção.
- [ ] T021 Alinhar frontend (caminho feliz) — **deferido** (grande desalinhamento de contrato/estado; fora do incremento mínimo seguro).
- [NV] T022 Smoke e2e integrado com Postgres/services/OIDC — não executado neste ambiente.

## Phase 3 — Evidence & delivery
- [X] T030 Verificações determinísticas executadas: `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54).
- [NV] T031 Smoke e2e com compose/serviços e CI remota — NV.
- [~] T032 Artefatos 006 atualizados com resultados/lacunas; preparação de branch/PR (GitHub canônico) — em andamento.
