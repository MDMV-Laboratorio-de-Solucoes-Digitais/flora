# 116 — identidade real de autores no chat

**Data**: 2026-11-23 · **Critério 80/20**: `240/625` · **Issues**: `#385`..`#389` · **Domínio crítico**: `tenant`

## O que mudou

- **Backend em lote e DTOs aditivos (`T-001` / `#385`)**:
  - `flora-core/src/traits/repository.rs` & `flora-core/src/repositories/user_repository.rs`: método `UserRepository::find_by_ids` com query única `WHERE id = ANY($1)` e retorno imediato para slice vazio.
  - `flora-api/src/realtime.rs`: `MessageEventPayload` ampliado com `sender_name` e `sender_avatar_url` (`#[serde(default)]`) e função pura `resolve_sender_name` + 2 testes unitários em `realtime::tests`.
  - `flora-api/src/routes/channels.rs`: `MessageResponse` enriquecido via `with_author`, `enrich_messages` (lote deduplicado em `list_messages` normal e catch-up `after`) e `enrich_single_message` (`post_message`, `edit_message`, `delete_message`).
  - `flora-api/src/routes/websocket.rs`: enriquecimento best-effort em `handle_send_message` sem derrubar broadcast em caso de erro de leitura.
- **Contrato DB e trava lockstep (`T-002` / `#386`)**:
  - `flora-tests/src/contract/messaging.rs`: adicionado `db_messages_include_author_display_name_across_post_list_and_catchup` cobrindo `POST` com `display_name`, fallback para `email` em `display_name` em branco, resolução em lote no `GET`, catch-up `?after=`, preservação no `PATCH` e ausência de `password_hash`/`oidc_subject`.
  - Atualizado em lockstep `FLOORS["messaging"] = 5` em `scripts/ci/check_gate_integrity.py` e `run_leg messaging contract::messaging::db_ 5` em `.github/workflows/rust.yml` (total `79` testes `#[ignore]`). Nenhum required check alterado.
- **Frontend TDD e acessibilidade (`T-003` / `#387`)**:
  - `apps/frontend/src/lib/utils/chatAuthor.ts` + `tests/unit/chatAuthor.test.ts` (TDD RED→GREEN): `formatAuthorName` e `authorInitials`.
  - `apps/frontend/src/lib/types/models.ts`, `mappers.ts`, `websocket.ts`, `MessageStore.svelte.ts`, `mappers.test.ts`, `MessageStore.test.ts`: mapeamento normalizado de `sender_name`/`sender_avatar_url` e preservação em updates parciais.
  - `apps/frontend/src/lib/components/chat/ChatLayout.svelte`: renderização de nome/iniciais reais em mensagens principais e threads (`aria-hidden="true"` nos avatares), guarda `loadSeq` no `loadMessages` e `aria-label` acessível no `<textarea>`.
- **Smoke E2E (`T-004` / `#388`)**:
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts`: Passo 3 em `# general` verifica `'Alice Admin'` e `'Bob Builder'` visíveis e contagem `0` de `'Usuário 22222222'` antes e após envio da mensagem de fumaça.
- **Briefing `tenant`, SDD e governança (`T-005` / `#389`)**:
  - Briefing de Compreensão em `docs/governance/2026-11-23-116-briefing-compreensao.md`, R4/R7 (`0.8.0 -> 0.9.0` MINOR) e docs append-only com recontagem determinística.

## [NV]

Execução remota de `db-tests` em Postgres 18, `e2e-browser` em Chromium, ACR independente, WS multi-usuário ao vivo no E2E, modo escuro em navegador real e staging/produção externos.

## Rollback

Novo commit de reversão dos arquivos do ciclo 116 (reduzindo em lockstep `messaging` `5 → 4` caso `db_messages_include_author_display_name_across_post_list_and_catchup` seja revertido); os campos novos em JSON são opcionais/aditivos sem migração SQL e sem force-push.
