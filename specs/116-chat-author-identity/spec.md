# Feature Specification: Identidade real de autores no chat

**Feature ID**: 116 · **Status**: specified · **Data**: 2026-11-23

## User Scenario

Como membro autenticado de uma organização no Flora, quero ver o nome de exibição real (ou e-mail como fallback) e iniciais dos autores nas mensagens e respostas de thread de um canal, em vez de identificadores técnicos truncados (`Usuário <uuid>`), para identificar quem falou nas conversas de forma legível.

## Acceptance Scenarios

1. **SC-001 (Lookup em lote + DTO retrocompatível `REST`/`WS`)**: **Given** usuário autenticado em canal da sua organização, **when** posta, lista, sincroniza (`after`), edita ou exclui mensagens em `flora-api`, **then** `MessageResponse` e `MessageEventPayload` incluem `sender_name` (`display_name` trimmed, ou `email` trimmed quando `display_name` vazio, ou `None`) e `sender_avatar_url`, obtidos em lote sem N+1 via `UserRepository::find_by_ids` exclusivamente para `sender_id`s das mensagens do canal autorizado e sem expor `password_hash`, `oidc_subject` ou `profile`.
2. **SC-002 (Contrato DB e trava lockstep)**: **Given** PostgreSQL 18 com migrações aplicadas, **when** `db_messages_include_author_display_name_across_post_list_and_catchup` executa em `flora-tests/src/contract/messaging.rs`, **then** comprova `sender_name` via `POST`, fallback para e-mail quando `display_name` está em branco, resolução em lote no `GET`, resolução no catch-up `?after=`, preservação no `PATCH` e ausência de campos secretos (`password_hash`, `oidc_subject`); o piso `messaging` sobe em lockstep `4 -> 5` (total `79`).
3. **SC-003 (Utilitário TDD, Mapper, Store e `ChatLayout`)**: **Given** suíte TDD `chatAuthor.test.ts` executada primeiro (RED→GREEN), **when** `toMessage`, `MessageStore.upsertMessage` e `ChatLayout.svelte` processam mensagens e respostas de thread, **then** exibem `authorInitials` (com `aria-hidden="true"`) e `formatAuthorName` (caindo para `Usuário <8 chars>` apenas se ausente), preservam autoria em atualizações parciais, protegem troca rápida de canal via `loadSeq` e expõem `aria-label` acessível no campo de composição.
4. **SC-004 (E2E Chromium em `# general`)**: **Given** Passo 3 em `seed-flow.e2e.ts` com dados `dev-seed.sql`, **when** `# general` abre e uma nova mensagem de fumaça é enviada, **then** `'Alice Admin'` e `'Bob Builder'` estão visíveis e `Usuário 22222222` tem contagem `0`, mantendo zero violações WCAG 2.2 AA.
5. **SC-005 (Briefing `tenant` e governança)**: **Given** ciclo que resolve identidades na fronteira `users`/`memberships`/`channels`, **when** encerrado, **then** entrega Briefing de Compreensão (`docs/governance/2026-11-23-116-briefing-compreensao.md`), nota de governança, contagens `find` append-only, roteiro/evidência, R4/R7 (`0.8.0 -> 0.9.0` MINOR) e Code Review agêntico independente.

## Requirements

- **FR-001**: Adicionar `UserRepository::find_by_ids` em `flora-core`, `resolve_sender_name` (+ testes unitários) e campos `sender_name`/`sender_avatar_url` em `MessageEventPayload` (`flora-api/src/realtime.rs`), `MessageResponse` (`flora-api/src/routes/channels.rs`) e `websocket.rs`.
- **FR-002**: Adicionar `db_messages_include_author_display_name_across_post_list_and_catchup` em `flora-tests/src/contract/messaging.rs` e elevar em lockstep `messaging: 4 -> 5` (`check_gate_integrity.py`, `.github/workflows/rust.yml`; total `79`).
- **FR-003**: Implementar `chatAuthor.ts` em TDD (`chatAuthor.test.ts`), atualizar `models.ts`, `mappers.ts`, `websocket.ts`, `MessageStore.svelte.ts`, `ChatLayout.svelte`, `mappers.test.ts` e `MessageStore.test.ts`.
- **FR-004**: Reforçar Passo 3 de `apps/frontend/tests/e2e/seed-flow.e2e.ts` verificando nomes reais e ausência de `Usuário 22222222`.
- **FR-005**: Preencher Briefing de Compreensão (`tenant`), governança, docs append-only, roteiro, `evidence.md`, R4, R7 (`0.9.0`) e ACR independente.

## [NV]

Execução local contra Postgres de staging real, entrega ao vivo via WebSocket multi-usuário em `e2e-browser` e varredura em modo escuro real permanecem `[NV]`.
