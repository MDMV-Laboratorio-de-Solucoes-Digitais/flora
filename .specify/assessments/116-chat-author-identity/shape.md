# Assess — shape: 116

**Data**: 2026-11-23

## Desenho selecionado

- **Separação de fronteiras**: `list_messages`/`post_message`/`edit_message`/`delete_message` continuam verificando `channel.organization_id == org_id` antes de qualquer lookup em `users`; `find_by_ids` recebe exclusivamente os `sender_id`s deduplicados das mensagens pertencentes àquele canal autorizado, impedindo enumeração cross-tenant.
- **WebSocket best-effort**: `handle_send_message` busca `PgUserRepository::find_by_id(user_id).await.ok().flatten()`, nunca abortando a entrega em tempo real se o lookup falhar.
- **Retrocompatibilidade no cliente**: `Message.authorName` / `authorAvatarUrl` permanecem opcionais; `formatAuthorName` cai graciosamente para `t['chat.author']` caso um payload legado omita `sender_name`. `loadMessages` ganha `loadSeq` monotônico evitando corrida ao trocar rápido de canal.
