# Release: 116 — identidade real de autores no chat

**Ciclo**: 116 · **Data**: 2026-11-23 · **Baseline**: `github/dev@2052ad7f1`

## Entregue

- `FR-001` / `T-001` (`#385`): método em lote `UserRepository::find_by_ids` (`WHERE id = ANY($1)`), função pura `resolve_sender_name` (+ 2 testes unitários em `realtime.rs`) e enriquecimento de `MessageResponse` (`channels.rs`) e `MessageEventPayload` (`realtime.rs`, `websocket.rs`) com `sender_name` e `sender_avatar_url`.
- `FR-002` / `T-002` (`#386`): contrato DB `db_messages_include_author_display_name_across_post_list_and_catchup` em `flora-tests/src/contract/messaging.rs` e elevação em lockstep `messaging: 4 → 5` (total `79`) em `check_gate_integrity.py` e `.github/workflows/rust.yml`.
- `FR-003` / `T-003` (`#387`): utilitário TDD `chatAuthor.ts` (`chatAuthor.test.ts` 4/4), extensão retrocompatível de `Message` (`models.ts`), `ApiMessage`/`toMessage` (`mappers.ts`), `WsMessagePayload` (`websocket.ts`), preservação de autoria em `MessageStore.upsertMessage` (`mappers.test.ts` 3/3, `MessageStore.test.ts` 2/2), guarda `loadSeq`, `aria-hidden` nos avatares e `aria-label` no composer em `ChatLayout.svelte`.
- `FR-004` / `T-004` (`#388`): Passo 3 em `apps/frontend/tests/e2e/seed-flow.e2e.ts` validando `'Alice Admin'` e `'Bob Builder'` em `# general` sem `Usuário 22222222`.
- `FR-005` / `T-005` (`#389`): Briefing de Compreensão (`tenant`), governança, docs append-only com recontagem determinística, roteiro, R4/R7 e ACR independente.

## SemVer

MINOR `0.8.0` → `0.9.0`: extensão retrocompatível de payloads REST/WebSocket (`sender_name`, `sender_avatar_url`) e resolução de identidade real de autores na UI de chat, sem quebra de contrato existente ou alteração de schema/required checks.

## Evidência

Portas locais verdes (`cargo fmt`, `clippy -D warnings`, `cargo test --workspace`, `check_gate_integrity.py` PASS 10 legs / 79 pisos, `svelte-check` 0/0, `lint` exit 0, Vitest `server` `17/64`, `vite build` `26.46s`). Evidência remota de CI (`db-tests`, `e2e-browser`) e ACR será anexada em `.specify/validations/116-chat-author-identity/evidence.md`.

## Rollback

Novo commit de reversão dos arquivos do ciclo 116 (reduzindo em lockstep `messaging` `5 → 4` caso `db_messages_include_author_display_name_across_post_list_and_catchup` seja revertido); os novos campos JSON são aditivos e opcionais, sem migração SQL e sem force-push.

## [NV]

Execução local direta com Postgres de staging real, entrega WS multi-usuário ao vivo em E2E, modo escuro em navegador real, espelho Forgejo e deploy em produção.
