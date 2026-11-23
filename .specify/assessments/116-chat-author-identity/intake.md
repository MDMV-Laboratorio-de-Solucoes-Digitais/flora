# Assess — intake: 116 — identidade real de autores no chat

**Data**: 2026-11-23 · **Origem**: lacuna #3 do núcleo Seed verificada em `github/dev@2052ad7f1`

## Problema

Os endpoints `GET/POST/PATCH/DELETE /api/v1/channels/{channel_id}/messages` (`flora-api/src/routes/channels.rs`) e eventos realtime (`flora-api/src/realtime.rs` / `websocket.rs`) serializam apenas `sender_id` (UUID). No front-end, `apps/frontend/src/lib/components/chat/ChatLayout.svelte:130,166` renderiza `Usuário {message.authorId.slice(0, 8)}` e a primeira letra hexadecimal do UUID no avatar (`:124,161`) tanto em mensagens principais quanto em respostas de thread.

## Pedido

Adicionar consulta em lote `UserRepository::find_by_ids` em `flora-core`, expor campos aditivos retrocompatíveis `sender_name` e `sender_avatar_url` em `MessageResponse` (REST) e `MessageEventPayload` (WebSocket/Hub), preencher no envio/listagem/catch-up/edição/exclusão apenas após validação de pertencimento ao canal/organização, atualizar `models.ts`, `mappers.ts`, `MessageStore.svelte.ts`, `ChatLayout.svelte` (com utilitário `chatAuthor.ts` em TDD), e validar nomes reais (`Alice Admin`, `Bob Builder`) no E2E em `# general` junto ao Briefing de Compreensão (`tenant`).

## Fora de escopo

Endpoint público de diretório de usuários, criação de DM reais (`/dms`), alteracão de migrations SQL/schema ou alterações em required checks.
