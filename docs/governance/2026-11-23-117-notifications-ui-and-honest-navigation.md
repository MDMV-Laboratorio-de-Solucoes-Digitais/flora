# 117 — central de notificações escopada por organização e navegação honesta

**Data**: 2026-11-23 · **Critério 80/20**: `240/625` · **Issues**: `#391`..`#395` · **Domínio crítico**: `tenant`

## O que mudou

- **Isolamento multi-tenant e `POST /read-all` (`T-001` / `#391`)**:
  - `flora-core/src/traits/repository.rs` e `flora-core/src/repositories/notification_repository.rs`: adicionados `find_unread_by_user_and_organization`, `find_by_user_and_organization` e `mark_all_as_read_in_organization` com filtro explícito `WHERE user_id = $1 AND organization_id = $2`.
  - `flora-api/src/routes/notifications.rs`: adicionado `ensure_member` (via `auth.require_org()` + `PgMembershipRepository`), exposição de `target_url` em `NotificationResponse`, resposta fail-closed `404 NotFound` quando uma notificação pertence a outra organização, `POST /api/v1/notifications/read-all`, e testes unitários em `notifications::tests`.
  - `flora-tests/src/contract/notifications.rs`: adicionado contrato `db_notifications_org_scoped_target_url_and_read_all` com elevação em lockstep `notifications: 5 → 6` (total `80`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- **Tipos, mapper, utilitários e `NotificationStore` em TDD (`T-002` / `#392`)**:
  - `models.ts`, `mappers.ts`, `pt-BR.ts`, `utils/notifications.ts` (`notificationsQueryPath`, `notificationEventLabel`, `resolveNotificationHref` com allowlist de rotas reais da SPA), `NotificationStore.svelte.ts` (`unreadCount` reativo), `tests/unit/notifications.test.ts` e `tests/unit/mappers.test.ts` (`RED → GREEN`).
- **Central `/notifications` e navegação honesta (`T-003` / `#393`)**:
  - Nova rota `apps/frontend/src/routes/(app)/notifications/+page.svelte` com filtro Todas/Não lidas, atualização, marcação individual/em lote e links seguros.
  - `apps/frontend/src/routes/(app)/+layout.svelte`: adicionado link `/notifications` com badge reativo sincronizado por organização e removidos os atalhos placeholder (`/dms` na barra lateral e gaveta vazia de IA no cabeçalho).
  - `apps/frontend/src/routes/(app)/dms/+page.svelte`: substituído `<ChatLayout channelId="dm-channel-1" />` por card informativo honesto em `pt-BR` com botão para `/channels`.
- **E2E e governança (`T-004` / `#394`, `T-005` / `#395`)**:
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts`: Passos 6 e 7 validando ausência de atalhos fantasma, `/notifications` (`GET 200` + empty state), `/dms` honesto e zero violações WCAG 2.2 AA.
  - Briefing de Compreensão (`tenant`) em `docs/governance/2026-11-23-117-briefing-compreensao.md`, R4/R7 (`0.9.0 → 0.10.0` MINOR) e atualização append-only com recontagem determinística.

## [NV]

Execução remota de `db-tests` em Postgres 18, `e2e-browser` em Chromium, ACR independente, tema escuro em navegador real e staging/produção externos.

## Rollback

Novo commit de reversão dos arquivos do ciclo 117 (reduzindo em lockstep `notifications` `6 → 5` caso `db_notifications_org_scoped_target_url_and_read_all` seja revertido); nenhuma migration SQL alterada e nenhum force-push.
