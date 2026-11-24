# Briefing de Compreensão — 119 / #405..#408

**Data**: 2026-11-23 · **Domínio crítico**: auth / rbac
**Issues**: [#405](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/405)–[#408](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/408) · **Autor**: orquestrador

## Intenção

Expor no frontend canônico (`/settings`) uma interface segura e acessível em `pt-BR` para administração CRUD de papéis RBAC (`RoleManager.svelte`, `RoleStore.svelte.ts`, `api/roles.ts`, `utils/roles.ts`) sobre os endpoints autenticados de `flora-api/src/routes/rbac.rs`, sem afrouxar a imutabilidade de papéis nativos nem o isolamento multi-tenant no backend ou no estado reativo do navegador.

## Invariantes

- **Antes**:
  - Os endpoints `GET/POST /api/v1/roles` e `GET/PATCH/DELETE /api/v1/roles/{id}` já impunham `auth.require_org()`, associação ativa, permissão `OrgRead|OrgAdmin` / `RoleWrite|OrgAdmin` e bloqueio `403` para mutações em `is_builtin = true`, mas não havia cliente API, store nem UI em `apps/frontend` para administrá-los.
- **Depois**:
  - **Autoridade exclusiva no backend preservada**: `api/roles.ts` envia apenas `Bearer <token>` sem header nem campo `organization_id`; o servidor continua derivando o tenant via `AuthExtractor::require_org()` e verificando `can_list_roles` / `can_manage_roles` / `can_modify_role`.
  - **Imutabilidade de nativos e permissões canônicas**: `RoleManager.svelte` oculta os controles `Editar`/`Excluir` quando `isBuiltin === true` (`canModifyRole`) e restringe o formulário às `27` permissões canônicas de `ROLE_PERMISSION_OPTIONS` (sem expor curinga `*`).
  - **Isolamento reativo multi-contexto (`RoleStore.svelte.ts`)**: `syncContext(contextKey)` + `loadSequence` + deduplicação `pendingLoad` invalidam e descartam respostas em voo quando a organização ou sessão (`organizationId|token`) muda, enquanto `$effect` em `RoleManager.svelte` dispara carga exatamente uma vez por mudança de `contextKey` (sem loop em lista vazia ou `403`).
  - **Tratamento seguro de erros (`pt-BR`)**: erros `403` exibem `t['roles.forbidden']`, erros `409`/`conflict` exibem `t['roles.duplicateName']` e demais erros passam por `apiErrorMessage(...)` sem nunca renderizar mensagens cruas do backend ou o fallback enganoso de conflito de bootstrap.

## Decisões implícitas

1. **Não incluir curinga `*` no seletor visual de papéis personalizados**: reduz risco de escalação acidental ao criar papéis customizados mantendo paridade com as 27 strings canônicas de `Permission::from_str`.
2. **Normalizar descrição em branco para `null` na leitura (`toRoleItem`) e enviar `""` apenas na limpeza durante edição (`updateRole`)**: garante compatibilidade com o `if let Some(description) = req.description` de `PATCH /api/v1/roles/{id}` em `flora-api/src/routes/rbac.rs:252-254`.
3. **Sem alteração no backend nem nos contratos DB (`82` pisos)**: `db_rbac_*` (`3/3`) já valida listagem escopada, criação com `role:write` e imutabilidade de nativos; o Passo 9 do Playwright valida o fluxo ponta a ponta no navegador real.

## Comando de verificação

```bash
cd apps/frontend && \
bun run check && \
bun run lint && \
bun run test:unit -- --run --project server && \
bun run build && \
bun run test:e2e
```

## Pergunta-teste ao SE

**Pergunta**: Se um membro com papel `Member` (sem `role:write` ou `org:admin`) abrir `/settings` ou enviar `POST/PATCH/DELETE /api/v1/roles`, ou se um usuário trocar de organização enquanto `GET /api/v1/roles` da organização anterior ainda estiver em voo, a UI vaza papéis de outro tenant, entra em loop ou permite alterar um papel nativo?
**Resposta esperada**: Não. `RoleStore.syncContext` incrementa `loadSequence` e descarta respostas da `contextKey` anterior; `$effect` acompanha `observedContextKey` sem re-disparar após falha `403` (`roles.forbidden`); papéis `isBuiltin = true` não recebem ações `Editar`/`Excluir` e qualquer tentativa direta é rejeitada com `403 Forbidden` em `require_org_admin` / `can_modify_role` (`flora-api/src/routes/rbac.rs`).

## [NV]

- Execução em staging/produção externos com Coolify (`workflow_dispatch`, fora do gate de PR).
