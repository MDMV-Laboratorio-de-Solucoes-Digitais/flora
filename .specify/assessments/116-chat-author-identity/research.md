# Assess — research: 116

**Data**: 2026-11-23 · **Baseline**: `github/dev@2052ad7f1`

## Evidências em código

1. `flora-core/src/traits/repository.rs:15-45` (`UserRepository`) e `flora-core/src/repositories/user_repository.rs:27` (`PgUserRepository` — único implementador): oferecem `find_by_id` único, sem método em lote `find_by_ids(&[Uuid])`.
2. `flora-core/src/models/user.rs:14-39`: `User` contém `display_name: String`, `email: String`, `avatar_url: Option<String>`, além de campos sensíveis/internos (`password_hash`, `password_updated_at`, `oidc_subject`, `profile`).
3. `flora-api/src/routes/channels.rs:94-141, 280-540`: `list_messages` (catch-up `after` e listagem paginada), `post_message`, `edit_message` e `delete_message` validam `channel.organization_id == org_id` antes de emitir `MessageResponse` e `MessageEventPayload`.
4. `flora-api/src/routes/websocket.rs:274-365`: `handle_send_message` valida `validate_channel_ownership`, persiste via `PgMessageRepository` e publica `MessageEventPayload`.
5. `apps/frontend/src/lib/types/models.ts:27-36`, `mappers.ts:11-44`, `websocket.ts:8-17`, `MessageStore.svelte.ts:21-30`, `ChatLayout.svelte:120-180`: `ChatLayout.svelte` usa `format('chat.author', { id: message.authorId.slice(0, 8) })`; `dev-seed.sql` já possui usuários `'Alice Admin'` (`22222222-...`) e `'Bob Builder'` (`33333333-...`) com mensagens no canal `# general`.
