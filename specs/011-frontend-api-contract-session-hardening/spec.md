# Feature Specification: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Feature Branch**: `feat/011-frontend-api-contract-session-hardening`  
**Created**: 2026-11-15  
**Status**: Implemented (pending PR/CI/merge)  
**Assessment**: `.specify/assessments/011-frontend-api-contract-session-hardening/{intake,research,problem,concept,decision}.md`

## Goal
Consolidar o slice núcleo auth→org→workspace→task com contratos mais corretos e verificáveis: (1) API de criação de task persiste campos enviados; (2) frontend trata erros/status de forma tipada e consistente; (3) estado/guards/testes do slice alinhados; (4) documentação de smoke mais resiliente a conflitos de porta.

## User Scenarios & Testing
- **US1 (P1):** usuário cria task com descrição e atribuição ("assign to me") e a task criada persiste `description`/`assignee_id` conforme request.
- **US2 (P1):** falhas de API no slice expõem status/código/mensagem/campo (quando aplicável) via `ApiError`; 401 continua acionando lock de sessão.
- **US3 (P2):** página `/tasks` redireciona para `/` sem auth/token e para `/select-org` sem organização selecionada.
- **US4 (P2):** checks determinísticos (Rust + frontend server/unit + build) passam.

## Requirements
- **FR-001:** `POST /api/v1/tasks` aplica `description` e `assignee_id` do request na task criada. **Done**
- **FR-002:** `ApiClient` preserva `status` e corpo de erro (JSON/texto) em erro tipado; em 401 chama `authState.lockSession()`. **Done**
- **FR-003:** `AuthState.setOrganization` limpa `isInvalidated`; testes unitários de `AuthState`/interceptor alinhados ao comportamento atual. **Done**
- **FR-004:** `/tasks` possui guard mínimo de auth/org antes de carregar. **Done**
- **FR-005:** `docs/dev-smoke.md` inclui exemplo concreto de override de portas para conflito de host (com NV). **Done**
- **FR-006:** Evidência de verificação determinística e artefatos SDD/validação/release completos. **Done (local)**

## Non-goals
- Revalidação DB global de sessão em todas as rotas protegidas (follow-up), OIDC/JWKS e2e real, mudanças de migração aplicada, realtime/files/search/notificações, expansão ampla de UI/produto.
