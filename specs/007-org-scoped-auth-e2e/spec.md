# Feature Specification: 007 — Org-Scoped Auth + Verifiable Core Slice

**Feature Branch**: `[007-org-scoped-auth-e2e]`  
**Created**: 2026-11-15  
**Status**: Implemented (partial) — org-selection/session-scoping delivered; integrated e2e/UI alignment NV/deferred  
**Assessment**: `.specify/assessments/007-org-scoped-auth-e2e/{intake,research,decision}.md`  
**Depends on**: `specs/006-secure-vertical-slice/`

## Delivered
1. **Seleção de organização (backend):** `POST /api/v1/auth/select-organization` aceita Bearer JWT autenticado (incl. token inicial com org nil), valida organização/membership e sessão ativa, revoga a sessão atual e emite nova **sessão+JWT org-scoped** com `jti` rotacionado.
2. **Consistência de acesso:** credencial retornada inclui `organization_id` e é compatível com `AuthExtractor::require_org()` e rotas tenant-scoped endurecidas no ciclo 006.
3. **Smoke/contrato atualizado:** `test-flora.sh` migrado para Bearer/JWT, documenta fluxo OIDC→callback→select-organization e executa chamadas tenant-scoped (workspaces/tasks) quando token/ambiente estão disponíveis.

## Deferred / NV
- **E2E integrado:** execução com OIDC real, Postgres/seed e serviços auxiliares **NV** neste ambiente.
- **Frontend/UI e contrato completo:** seleção/persistência de contexto org/workspace, correções de payload/rotas/WS e slice UI verificável permanecem pendentes.
- **Testes dedicados do endpoint:** não adicionados neste incremento (suites existentes passaram).

## Evidence (local, 2026-11-15)
- `cargo fmt --all -- --check` ✓
- `cargo check -p flora-api -p flora-core` ✓
- `cargo test -p flora-core --lib` ✓ (8 passed)
- `cargo test -p flora-tests --lib` ✓ (54 passed)

## Compatibility impact
- Fluxo de smoke agora assume Bearer/JWT e seleção explícita de organização; clientes/scripts baseados em headers `X-User-ID`/`X-Organization-ID` permanecem incompatíveis (mudança iniciada no ciclo 006).
