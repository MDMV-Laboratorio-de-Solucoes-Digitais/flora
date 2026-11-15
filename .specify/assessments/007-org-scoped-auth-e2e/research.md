# Research: 007 — Org-Scoped Auth + Verifiable Core Slice

**Date:** 2026-11-15  
**Status:** synthesized (2 subagent audits completed; 2 subagents pending at synthesis time — workflow/research inputs incorporated where available)

## Inputs (local evidence)
- `flora-api/src/routes/auth.rs`: callback criava sessão/JWT com `organization_id=nil` e sinalizava `select_or_create_organization`; refresh reemitia token com mesmo `jti`/`session.organization_id`; logout revogava sessões do usuário.
- `flora-core/src/{models/session.rs,utils/jwt.rs,traits/repository.rs,repositories/session_repository.rs,services/session_service.rs}`: sem update de `organization_id/jti`; rotação possível via revoke+create (novo `jti`).
- Ciclo 006: rotas tenant-scoped exigem `AuthExtractor::require_org()` (org não-nil) + membership; WS exige Bearer JWT com org concreta.
- `test-flora.sh`: usava headers `X-User-ID`/`X-Organization-ID` e path `/auth/login` (incompatível).

## Subagent findings (completed)
- **Backend auth/session audit (`ses_e1ea41e0dffeWk8vTPsHFXBV5k`)**: confirmou ausência de endpoint de seleção/escopo de org e de update de sessão; recomendou endpoint aditivo autenticado que valide membership, rotacione sessão (novo `jti`+org, revogar anterior) e emita JWT org-scoped; refresh já reflete `session.organization_id`.
- **Frontend/contrato audit (`ses_e1ea41e07ffebb36eOtH32TuIi`)**: identificou bloqueios (callback nil-org ignorado, sem seleção/token org-scoped, payloads/estado divergentes); recomendou slice verificável **API-level** mínimo: `nil-org token → select org → org-scoped session → create workspace → create/list task`, adiando wiring completo de UI/WS.

## Subagents (pending at synthesis time)
- Workflow mdmv-dark-factory (`ses_e1ea41e0fffeDgn1ojT3GM1BXp`) — **NV** no momento da síntese.
- Pesquisa multi-idioma de padrões org-scoped auth (`ses_e1ea41e08ffeHj1Q40A2fgILJO`) — **NV** no momento da síntese.

## Implemented direction (80/20)
- Adicionado `POST /api/v1/auth/select-organization` (Bearer JWT autenticado; aceita token com org nil): valida `organization_id`, verifica sessão ativa por `jti`, exige membership, revoga sessão atual e cria nova sessão+JWT com org selecionada e novo `jti` (rotação).
- `test-flora.sh` atualizado para modelo Bearer/JWT, documentando fluxo OIDC→callback→select-organization e fornecendo smoke de org-selection + workspace/task quando `FLORA_TOKEN`/token escopado está disponível.

## Remaining gaps / NV
- Execução e2e integrada com OIDC real/DB/seed e validação runtime do smoke: **NV** neste ambiente.
- UI/frontend e alinhamentos maiores de contrato (org slug no form, seleção/persistência de contexto, WS/sync, listagem por workspace, etc.): **pendentes/deferidos**.
- Possível necessidade de endurecimento adicional (rate limit específico, auditoria, refresh/rotation policies) conforme pesquisa/workflow pendentes.
