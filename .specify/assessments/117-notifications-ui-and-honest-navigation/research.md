# Assess — research: 117

**Data**: 2026-11-23 · **Baseline**: `github/dev@0cc41336f`

## Evidências em código

1. **Schema (`flora-core/src/migrations/sql/20240818000001_init_schema.sql:113-126`)**: `notifications` possui `id`, `organization_id`, `user_id`, `event_type`, `target_id`, `target_url`, `is_read`, `created_at`, `updated_at` e índice único parcial de dedup em `20261116000003_notifications_unread_dedup.sql`.
2. **Trait e Repositório (`flora-core/src/traits/repository.rs:290-333`, `flora-core/src/repositories/notification_repository.rs:26-157`)**: hoje existem apenas `find_unread_by_user_id`, `find_by_user_id` e `mark_all_as_read(user_id)` sem filtro por `organization_id`.
3. **Rotas (`flora-api/src/routes/notifications.rs:20-108`)**:
   - `NotificationResponse` omite `target_url`.
   - `list_notifications` e `mark_as_read` leem apenas `auth.user_id` sem chamar `auth.require_org()` nem `PgMembershipRepository`.
   - Os 5 testes existentes em `flora-tests/src/contract/notifications.rs` já passam `org_id` válido via `FloraClaims` e criam membership para os usuários seedados.
   - Em `files.rs:250` e `tasks.rs`, recursos fora da organização retornam `404 NotFound` (fail-closed contra enumeração cross-tenant).
4. **Frontend (`apps/frontend/src/routes/(app)/+layout.svelte`, `dms/+page.svelte`, `pt-BR.ts`)**:
   - `+layout.svelte:136-143` aponta para `/dms`; `dms/+page.svelte:8-9` monta `<ChatLayout channelId="dm-channel-1" />`.
   - `+layout.svelte:241-299` monta botão e painel de IA sem estado nem handler.
   - `dev-seed.sql:299-315` insere notificações de exemplo com `target_url` `/channels/general/messages/eeeeeeee-...` (para Bob) e `/tasks/33333333-...` (para Carol); Alice começa com 0 notificações em `dev-seed.sql`.
5. **Issues R6 pré-existentes no GitHub**: `#391` (`T1`), `#392` (`T2`), `#393` (`T3`), `#394` (`T4`), `#395` (`T5`).
