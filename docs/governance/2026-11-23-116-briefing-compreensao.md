# Briefing de Compreensão — 116 / PR #390

**Data**: 2026-11-23 · **Domínio crítico**: tenant
**PR**: [#390](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/390) · **Autor**: orquestrador

## Intenção

Expor a identidade pública dos autores das mensagens de chat (`sender_name` e `sender_avatar_url`) nas respostas REST (`MessageResponse`) e eventos realtime (`MessageEventPayload`) usando busca em lote (`UserRepository::find_by_ids`) e renderizar nome/iniciais reais em `ChatLayout.svelte`, substituindo o fallback opaco `Usuário <8 chars do UUID>` sem abrir enumeração cross-tenant nem expor credenciais/segredos de `users`.

## Invariantes

- **Antes**:
  - `UserRepository` (`flora-core/src/traits/repository.rs`) só oferecia consultas unitárias (`find_by_id`, `find_by_email`, `find_by_oidc_subject`).
  - `MessageResponse` (`flora-api/src/routes/channels.rs`) e `MessageEventPayload` (`flora-api/src/realtime.rs`) emitiam apenas `sender_id` (`Uuid`).
  - `apps/frontend/src/lib/components/chat/ChatLayout.svelte:130,166` formatava toda autoria como `Usuário <primeiros 8 chars do UUID>`.

- **Depois**:
  - **Autorização antes de qualquer lookup**: todos os handlers em `flora-api/src/routes/channels.rs` (`list_messages` `:330-366`, `post_message`, `edit_message`, `delete_message`) e `handle_send_message` em `flora-api/src/routes/websocket.rs` primeiro exigem `org_id` derivado da sessão (`AuthExtractor`) e validam `channel.organization_id == org_id` (`validate_channel_ownership`).
  - **Lote fechado pelo conteúdo do canal**: `enrich_messages` (`flora-api/src/routes/channels.rs:148-169`) coleta exclusivamente os `sender_id`s deduplicados das mensagens pertencentes ao canal autorizado (`HashSet<Uuid>` -> `Vec<Uuid>`) e chama `PgUserRepository::find_by_ids` (`flora-core/src/repositories/user_repository.rs:70-85`, `WHERE id = ANY($1)` com retorno imediato se vazio). Nenhum parâmetro de requisição do cliente escolhe IDs de usuário.
  - **Projeção mínima de identidade**: `MessageResponse::with_author` (`channels.rs:138-145`) e `resolve_sender_name` (`flora-api/src/realtime.rs:73-83`) expõem apenas `sender_name` (`display_name.trim()` ou fallback para `email.trim()`, ou `None`) e `sender_avatar_url`, omitindo `password_hash`, `password_updated_at`, `oidc_subject` e `profile`.
  - **Frontend / Store**: `MessageStore.upsertMessage` preserva `authorName` / `authorAvatarUrl` quando atualizações parciais os omitem (`null`); `ChatLayout.svelte:34-65, 129-185, 219-225` usa `authorInitials`/`formatAuthorName`, guarda `loadSeq` e `aria-label` acessível no composer.

## Decisões implícitas

1. **Zero endpoint genérico `/api/v1/users`**: optou-se por enriquecer diretamente `MessageResponse` e `MessageEventPayload` no backend a partir dos `sender_id`s do canal autorizado em vez de criar um diretório aberto `/api/v1/users?ids=...`, eliminando superfície de enumeração de usuários entre organizações.
2. **Fallback controlado de e-mail dentro do mesmo canal**: quando `display_name` está vazio/apenas espaços, `resolve_sender_name` usa `email.trim()` exclusivamente para autores que já postaram no canal acessível ao membro.
3. **WS best-effort**: em `websocket.rs`, falha transitória na leitura do autor em `PgUserRepository::find_by_id` gera `tracing::warn!` e emite o evento com `sender_name: None` (cliente faz fallback para `Usuário <8 chars>`), sem interromper entrega em tempo real.
4. **Sem alteração em RLS em runtime (`ADR-0005` / `ADR-0006`)**: isolamento segue na camada de aplicação + `organization_id` na tabela `channels`/`messages`.

## Comando de verificação

```bash
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets -- -D warnings && \
cargo test --workspace && \
python3 scripts/ci/check_gate_integrity.py
# Contrato DB no Postgres 18 (executado pelo job db-tests no CI):
cargo test -p flora-tests contract::messaging::db_messaging::db_messages_include_author_display_name_across_post_list_and_catchup -- --ignored --test-threads=1
```

## Pergunta-teste ao SE

**Pergunta**: Um membro autenticado da Organização A consegue usar os endpoints de mensagens de chat ou a query `find_by_ids` para descobrir `display_name`/`email` de um usuário da Organização B que nunca participou dos canais da Organização A?
**Resposta esperada**: Não. Nenhum endpoint aceita lista arbitrária de `user_id`s do cliente: `list_messages` (`channels.rs:330-366`) valida `channel.organization_id == org_id` antes de buscar mensagens do canal, e `enrich_messages` (`:148-169`) passa a `find_by_ids` exclusivamente os `sender_id`s já persistidos nessas mensagens autorizadas. Além disso, o DTO expõe somente `sender_name` e `sender_avatar_url`, sem `password_hash`, `oidc_subject` ou `profile`.

## [NV]

- Execução ao vivo do contrato DB `db_messages_include_author_display_name_across_post_list_and_catchup` em Postgres 18 e do smoke Chromium `e2e-browser` (verificados remotamente no CI do PR).
- Entrega ao vivo multi-usuário via WebSocket no `e2e-browser` e varredura em modo escuro real permanecem `[NV]`.
- Políticas `CREATE POLICY` de RLS em runtime permanecem desativadas nesta fase (`ADR-0005`/`ADR-0006`).
