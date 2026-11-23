# Assess — define: 116

**Data**: 2026-11-23

## Escopo mínimo de sucesso (80/20)

- `UserRepository::find_by_ids(&[Uuid])` executando uma única query SQL (`WHERE id = ANY($1)`) com retorno imediato se vazio (zero N+1).
- Resolução pura `resolve_sender_name(display_name, email)` (`display_name` trimmed → fallback `email` trimmed → `None`), garantindo zero exposição de `password_hash`, `oidc_subject`, `profile` ou `email` quando `display_name` está preenchido.
- Campos opcionais aditivos `sender_name` e `sender_avatar_url` em `MessageResponse` e `MessageEventPayload` (`#[serde(default)]`).
- Contrato DB `db_messages_include_author_display_name_across_post_list_and_catchup` + lockstep `messaging: 4 -> 5` (79 total).
- Utilitário puro TDD `chatAuthor.ts` (`formatAuthorName`, `authorInitials`), `MessageStore.upsertMessage` preservando autoria em updates parciais, `ChatLayout.svelte` com nomes/iniciais reais e `aria-label` no composer, e E2E `# general` validando `'Alice Admin'` / `'Bob Builder'`.
