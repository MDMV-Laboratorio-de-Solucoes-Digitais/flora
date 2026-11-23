# Assess — define: 117

**Data**: 2026-11-23

## Escopo mínimo de sucesso (80/20)

- Escopar `GET /api/v1/notifications`, `POST /api/v1/notifications/{id}/read` e novo `POST /api/v1/notifications/read-all` pelo `organization_id` da sessão verificada (`ensure_member`), retornando `404` fail-closed para notificações de outra organização e expondo `target_url` opcional no DTO `NotificationResponse`.
- Adicionar contrato DB comprovando isolamento cross-org, `target_url` e `POST /read-all` (`notifications: 5 → 6`, total `80`).
- Criar tipos, mapper `toNotification`, helpers puros em `notifications.ts` e `NotificationStore.svelte.ts` em TDD (`RED→GREEN`).
- Implementar rota `/notifications` em `pt-BR`, adicionar link + badge reativo de não lidas na barra lateral, remover link lateral `/dms` e gaveta fake de IA do `+layout.svelte`, e tornar `/dms` uma página informativa honesta.
- Cobrir `/notifications` e `/dms` no `seed-flow.e2e.ts` (Passos 6 e 7) com zero violações WCAG 2.2 AA e documentar Briefing de Compreensão (`tenant`).
