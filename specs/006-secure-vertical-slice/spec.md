# Feature Specification: 006 — Secure Vertical Slice (Auth/Tenancy + Core Flow)

**Feature Branch**: `[006-secure-vertical-slice]`  
**Created**: 2026-11-15  
**Status**: Implemented (partial) — backend auth/tenant hardening delivered; e2e UI/contract alignment deferred (NV)  
**Assessment**: `.specify/assessments/006-secure-vertical-slice/{intake,research,decision}.md`  
**Depends on**: `specs/005-macro-alternative-mvp/` (strategic direction, P0 handoff)

## Context

Flora tinha P0s de segurança/consistência: contexto de auth/tenant derivado de headers crus em rotas críticas e ausência de fluxo núcleo ponta a ponta verificável. Este ciclo entregou o endurecimento backend mínimo (JWT-derived context) como base para a fatia e2e.

## Delivered

1. **Consistência de auth/tenancy (backend):** identidade e escopo organizacional derivados de JWT Bearer validado (`AuthExtractor`) nas rotas de workspaces, channels/messages, tasks, org, files, search, notifications e websocket.
2. **Autorização/membership:** verificações de membership e de tenancy de recursos no escopo (ex.: `workspace_id ∈ org` na criação de canal; checagens de org em leitura/mutação/delete).
3. **WebSocket:** contexto extraído apenas de Bearer JWT verificado (sem trust em headers `x-user-id/x-organization-id`), com rejeição de org nil.

## Deferred / NV

- **Token org-scoped pós-seleção:** callback ainda emite `organization_id=nil`; rotas tenant-scoped exigem org válida — fluxo de seleção/escopo de sessão não implementado.
- **Contrato frontend↔backend e UI e2e:** desalinhamentos significativos (auth/estado, seleção org/workspace, endpoints/shapes/WS, `/messages/sync` ausente, tarefas estáticas) — não alinhados neste incremento.
- **Verificação integrada:** smoke e2e com Postgres/Valkey/Meilisearch/OIDC e CI remota **NV** neste ambiente.

## Evidence (local, 2026-11-15)
- `cargo fmt --all -- --check` ✓
- `cargo check -p flora-api -p flora-core` ✓
- `cargo test -p flora-core --lib` ✓ (8 passed)
- `cargo test -p flora-tests --lib` ✓ (54 passed)

## Notes / compatibility impact
- Clientes que enviavam `X-User-ID`/`X-Organization-ID` (ex.: `test-flora.sh`, frontend atual) **não são mais aceitos** nas rotas refatoradas; migração/compatibilidade controlada fica para follow-up.
