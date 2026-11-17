# Research: 051 — Dead invite UI

**Data:** 2026-11-17T17:29Z · **Evidência local**; remoto **[NV]**.

## Primária (executada)

- `UserManager.svelte:18-21` — `ApiClient.request('/api/v1/users/invite', POST)`.
- `:24-25` — `catch` → `logger.error(...)`; nenhuma mensagem de erro na UI.
- `apps/frontend/src/routes/(app)/settings/+page.svelte:4,16` — import e renderização.
- `flora-api/src/routes/mod.rs:20-68` — 13 módulos montados; nenhum `/users`.
- `api/client.ts:130-131` — lança em não-2xx, portanto `successMessage` nunca é
  exibido na prática (404). Não há falso sucesso; há falha silenciosa.
- `specs/002-svelte-frontend-rewrite/spec.md:34` — cenário de convite ainda listado
  como se fosse entregue; drift documental a marcar, não apagar.

## Decisão

Remover a affordance morta (formulário que nunca pode funcionar). Alternativa de
implementar rota exigiria persistência de convite, emissão de token/e-mail e
decisão de autoridade (`OrgAdmin` vs `RoleWrite`) — fora desta sessão.

## NV

Backlog externo do dono sobre convites e intenção de implementar e-mail: **[NV]**.
