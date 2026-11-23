# Feature Specification: Central de notificações escopada por organização e navegação honesta

**Feature ID**: 117 · **Status**: specified · **Data**: 2026-11-23

## User Scenario

Como membro autenticado de uma organização no Flora, quero consultar e gerenciar minhas notificações isoladas pela organização ativa na sessão (com indicação de não lidas na barra lateral e navegação segura para o destino) e ver apenas fluxos reais na interface principal — sem atalhos enganosos para DMs inexistentes ou painel de IA sem backend.

## Acceptance Scenarios

1. **SC-001 (API de notificações escopada por organização, `target_url` e `POST /read-all`)**: **Given** usuário autenticado membro da Organização A, **when** consulta `GET /api/v1/notifications`, marca uma notificação via `POST /api/v1/notifications/{id}/read` ou marca todas via `POST /api/v1/notifications/read-all`, **then** todas as operações exigem `auth.require_org()` + associação (`ensure_member`), filtram por `(user_id, organization_id)`, incluem `target_url` opcional em `NotificationResponse`, retornam `404` fail-closed para notificações pertencentes a outra organização e `403` para notificações de outro usuário da mesma organização.
2. **SC-002 (Contrato DB e trava lockstep)**: **Given** PostgreSQL 18 com migrações aplicadas, **when** `db_notifications_org_scoped_target_url_and_read_all` executa em `flora-tests/src/contract/notifications.rs`, **then** comprova isolamento entre `Org A` e `Org B` no `GET`, exposição de `target_url`, `404` cross-org no `POST /{id}/read`, `403` cross-user na mesma org e escopo exclusivo de `POST /read-all` na org ativa; o piso `notifications` sobe em lockstep `5 -> 6` (total `80`).
3. **SC-003 (Tipos, Mapper, Utilitários e `NotificationStore` em TDD)**: **Given** suíte TDD `notifications.test.ts` executada primeiro (`RED→GREEN`), **when** `toNotification`, `notificationsQueryPath`, `notificationEventLabel`, `resolveNotificationHref` e `NotificationStore` processam notificações, **then** mapeiam `target_url` opcional, traduzem os 9 tipos de evento para `pt-BR`, normalizam rotas internas reais (`/channels`, `/tasks`, `/files`, `/settings`) bloqueando URLs externas/`javascript:`/`/dms` e mantêm `unreadCount` derivado reativamente.
4. **SC-004 (Rota `/notifications`, Sidebar com badge, remoção de placeholders e `/dms` honesto)**: **Given** usuário autenticado na interface `(app)`, **when** acessa o aplicativo, **then** a barra lateral exibe link para `/notifications` com badge reativo de não lidas, o atalho lateral `/dms` e a gaveta fake de IA (`Painel de insights de IA`) não aparecem na navegação principal, `/notifications` lista/filtra/marca lidas (individual e em lote) em `pt-BR`, e `/dms` acessível diretamente exibe estado informativo honesto com retorno para `/channels`.
5. **SC-005 (E2E Passos 6 e 7, WCAG 2.2 AA, Briefing `tenant`, R4/R7 e ACR)**: **Given** `seed-flow.e2e.ts` expandido com Passos 6 e 7, **when** executado em Chromium e acompanhado pelos artefatos R4/R7 (`0.9.0 → 0.10.0` MINOR), **then** verifica ausência de atalhos fantasma, `/notifications` (`GET 200` + estado vazio) e `/dms` honesto com zero violações WCAG 2.2 AA, além de entregar Briefing de Compreensão (`tenant`) e revisão agêntica independente.

## Requirements

- **FR-001**: Adicionar `find_unread_by_user_and_organization`, `find_by_user_and_organization` e `mark_all_as_read_in_organization` em `NotificationRepository`/`PgNotificationRepository`, e atualizar `flora-api/src/routes/notifications.rs` (`ensure_member`, escopo por org, `target_url` em `NotificationResponse`, `404` fail-closed cross-org e `POST /read-all`) com testes unitários.
- **FR-002**: Adicionar `db_notifications_org_scoped_target_url_and_read_all` em `flora-tests/src/contract/notifications.rs` e elevar em lockstep `notifications: 5 -> 6` (`check_gate_integrity.py`, `.github/workflows/rust.yml`; total `80`).
- **FR-003**: Implementar `NotificationItem` (`models.ts`), `ApiNotification`/`toNotification` (`mappers.ts`), chaves `pt-BR` (`pt-BR.ts`), `notifications.ts` e `NotificationStore.svelte.ts` em TDD (`notifications.test.ts` + `mappers.test.ts`).
- **FR-004**: Criar `(app)/notifications/+page.svelte`, atualizar `(app)/+layout.svelte` (badge reativo `/notifications`, remoção de `/dms` e drawer fake de IA), atualizar `(app)/dms/+page.svelte` (estado honesto) e adicionar Passos 6 e 7 em `seed-flow.e2e.ts`.
- **FR-005**: Preencher Briefing de Compreensão (`tenant`), governança, docs append-only, roteiro, `evidence.md`, R4, R7 (`0.10.0`) e ACR independente.

## [NV]

Execução local direta contra Postgres de staging real, tema escuro em navegador real e staging/produção externos permanecem `[NV]`.
