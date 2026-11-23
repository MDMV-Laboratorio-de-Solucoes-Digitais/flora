# Assess — intake: 117 — central de notificações escopada por organização e navegação honesta

**Data**: 2026-11-23 · **Origem**: lacunas #1, #2 e #4 verificadas em `github/dev@0cc41336f` e issues `#391`..`#395`

## Problema

1. **Fronteira multi-tenant e contrato de `/api/v1/notifications`**: `NotificationRepository::find_by_user_id` / `find_unread_by_user_id` e `list_notifications`/`mark_as_read` em `flora-api/src/routes/notifications.rs:51-108` não filtram por `organization_id` da sessão autenticada (`auth.require_org()`) nem verificam associação (`PgMembershipRepository`), não expõem `target_url` no DTO `NotificationResponse` e não oferecem marcação em lote por organização (`POST /api/v1/notifications/read-all`). Além disso, `mark_as_read` retorna `403` mesmo se uma notificação pertencer a outra organização.
2. **Ausência de UI para notificações**: embora o backend já crie e deduplica notificações (`Reply`, `Assignment`, `dev-seed.sql`), o frontend `apps/frontend` não possui `NotificationItem`, `NotificationStore`, rota `/notifications` nem indicador de não lidas na barra lateral.
3. **Navegação enganosa (`+layout.svelte` e `/dms`)**:
   - `apps/frontend/src/routes/(app)/+layout.svelte:136-143` exibe link para `/dms` e `apps/frontend/src/routes/(app)/dms/+page.svelte:9` renderiza `<ChatLayout channelId="dm-channel-1" />` com ID hardcoded inexistente no backend (`channels` só aceita `Public` e `Private`), causando erro ao abrir `/dms`.
   - `+layout.svelte:241-299` renderiza botão e gaveta lateral `Painel de insights de IA` (`Sparkles`) com `<Input>` sem ação nem rota no backend.

## Pedido

Escopar `/api/v1/notifications` por `organization_id` da sessão (com `ensure_member`, cross-org fail-closed `404`, `target_url` em `NotificationResponse` e `POST /api/v1/notifications/read-all`), construir os utilitários/store em Svelte 5 em TDD (`RED→GREEN`), entregar a rota `/notifications` com badge reativo na barra lateral, remover os atalhos placeholder (`/dms` e drawer vazio de IA) do layout principal, tornar `/dms` honesto e validar tudo em E2E (Passos 6 e 7) + WCAG 2.2 AA e Briefing de Compreensão (`tenant`).

## Fora de escopo

Implementar backend real de DMs (`/dms`), criar assistente de IA/LLM, introduzir WebSocket push de notificações ou alterações de schema SQL/required checks.
