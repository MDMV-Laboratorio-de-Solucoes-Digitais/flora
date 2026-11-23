# Briefing de Compreensão — 117 / #391..#395

**Data**: 2026-11-23 · **Domínio crítico**: tenant
**PR**: [#396](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/pull/396) · **Issues**: [#391](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/391)–[#395](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/395) · **Autor**: orquestrador

## Intenção

Escopar todas as operações HTTP de `/api/v1/notifications` (`GET /`, `POST /{id}/read`, `POST /read-all`) pelo `organization_id` da sessão autenticada verificada (`auth.require_org()` + `ensure_member`), expor `target_url` opcional de forma segura até a interface `/notifications` com badge reativo na barra lateral e remover os atalhos placeholder (`/dms` na sidebar e painel fake de IA no cabeçalho) sem abrir acesso ou oráculo de existência entre organizações.

## Invariantes

- **Antes**:
  - `PgNotificationRepository::find_by_user_id` / `find_unread_by_user_id` e `mark_all_as_read` consultavam e atualizavam notificações apenas por `user_id`, sem filtro por `organization_id`.
  - `list_notifications` e `mark_as_read` (`flora-api/src/routes/notifications.rs`) não chamavam `auth.require_org()` nem verificavam associação via `PgMembershipRepository`; `mark_as_read` não comparava `notif.organization_id` com o tenant da sessão (expondo `403` mesmo para outra organização) e `NotificationResponse` omitia `target_url`.
  - O layout `(app)/+layout.svelte` apontava para `/dms` (`<ChatLayout channelId="dm-channel-1" />`) e para um painel vazio de IA.

- **Depois**:
  - **Fronteira de tenant e associação explícita**: `ensure_member(&auth, &state)` (`flora-api/src/routes/notifications.rs`) exige `auth.require_org()` (derivado exclusivamente da sessão JWT/DB validada, nunca de header `X-Organization-ID`) e confirma vínculo ativo em `PgMembershipRepository::find_by_user_and_organization`.
  - **Queries escopadas por `(user_id, organization_id)`**: `PgNotificationRepository::find_by_user_and_organization`, `find_unread_by_user_and_organization` e `mark_all_as_read_in_organization` (`flora-core/src/repositories/notification_repository.rs`) incluem `WHERE user_id = $1 AND organization_id = $2` em todas as consultas e no `UPDATE` em lote (`POST /api/v1/notifications/read-all`).
  - **Fail-closed cross-org (`404`)**: em `mark_as_read`, se `notif.organization_id != org_id`, o handler retorna `Error::NotificationNotFound` (`404`) **antes** de verificar `notif.user_id != user_id` (`403`), evitando vazar existência de IDs pertencentes a outra organização.
  - **Navegação segura e estado reativo isolado no cliente**: `resolveNotificationHref` (`apps/frontend/src/lib/utils/notifications.ts`) aceita apenas prefixos internos permitidos (`/channels`, `/tasks`, `/files`, `/settings`, `/notifications`), descartando URLs externas/`//`/`javascript:`/`/dms`; `NotificationStore` (`NotificationStore.svelte.ts`) centraliza `syncContext` + `loadSequence` + deduplicação de requisições em voo (`pendingLoad`), descartando respostas obsoletas ao trocar de organização, encerrar sessão ou marcar notificações como lidas.

## Decisões implícitas

1. **Preservar métodos antigos de `NotificationRepository` para compatibilidade interna**: `find_by_user_id`, `find_unread_by_user_id` e `mark_all_as_read` permanecem no trait/repo sem remoção disruptiva, enquanto todas as rotas HTTP em `notifications.rs` usam exclusivamente os novos métodos escopados por organização.
2. **Normalização de `target_url` para rotas existentes**: links históricos gravados no seed (como `/channels/general/messages/<uuid>` ou `/tasks/<uuid>`) são normalizados para a raiz existente do módulo (`/channels` ou `/tasks`) para não produzir 404 na SPA até existir roteamento por deep-link granular.
3. **Sem alteração em RLS em runtime (`ADR-0005` / `ADR-0006`)**: isolamento segue na camada de aplicação e queries parametrizadas com `(user_id, organization_id)`.

## Comando de verificação

```bash
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets -- -D warnings && \
cargo test --workspace && \
python3 scripts/ci/check_gate_integrity.py
# Contrato DB no Postgres 18 (executado pelo job db-tests no CI):
cargo test -p flora-tests contract::notifications::db_notifications::db_notifications_org_scoped_target_url_and_read_all -- --ignored --test-threads=1
```

## Pergunta-teste ao SE

**Pergunta**: Um usuário que é membro tanto da Organização A quanto da Organização B, autenticado com sessão escopada na Organização A, consegue listar, marcar como lida ou afetar via `POST /api/v1/notifications/read-all` uma notificação pertencente à Organização B?
**Resposta esperada**: Não. `list_notifications` e `mark_all_as_read` chamam `ensure_member` e filtram por `(user_id, org_id)` da sessão da Organização A, não enxergando nem alterando notificações da Organização B; além disso, `POST /api/v1/notifications/{id_org_b}/read` com sessão da Organização A detecta `notif.organization_id != org_id` antes da checagem de usuário e retorna `404 NotFound` (fail-closed).

## [NV]

- Execução ao vivo do contrato DB `db_notifications_org_scoped_target_url_and_read_all` em Postgres 18 e do smoke Chromium `e2e-browser` (validados remotamente no CI do PR).
- Varredura WCAG em tema escuro real e staging/produção externos permanecem `[NV]`.
- Políticas `CREATE POLICY` de RLS em runtime permanecem desativadas nesta fase (`ADR-0005`/`ADR-0006`).
