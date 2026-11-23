# Assess — shape: 117

**Data**: 2026-11-23

## Desenho selecionado

- **Tenant boundary fail-closed no backend**: manter os métodos antigos em `NotificationRepository` intactos para não quebrar callers/testes legados e adicionar métodos explícitos `*_and_organization` / `mark_all_as_read_in_organization`. Nos handlers HTTP, exigir `ensure_member(&auth, &state)`, filtrar todas as queries por `(user_id, org_id)`, e em `mark_as_read` responder `NotificationNotFound` (`404`) antes de checar `user_id` quando `notif.organization_id != org_id` (evitando oráculo de existência cross-tenant).
- **Normalização segura de `target_url` no frontend**: rotas legadas gravadas em seed (como `/channels/general/messages/<uuid>` ou `/tasks/<uuid>`) ou valores malformados/externos (`https://`, `//`, `javascript:`) passam por `resolveNotificationHref`, que mapeia apenas para as rotas reais da SPA (`/channels`, `/tasks`, `/files`, `/settings`), evitando navegação aberta ou 404 interno.
- **Sincronia segura de badge na sidebar**: `+layout.svelte` carrega `notificationStore` via `$effect` com `loadSeq` monotônico somente quando autenticado com `organizationId`, e limpa o store ao trocar de organização ou encerrar sessão.
- **Navegação honesta**: remover o atalho `/dms` da sidebar e o drawer vazio de IA do cabeçalho; se alguém acessar `/dms` diretamente por URL, exibir cartão honesto explicando que mensagens diretas ainda não estão disponíveis nesta versão e oferecer botão para `/channels`.
