# Release: 117 — central de notificações escopada por organização e navegação honesta

**Ciclo**: 117 · **Data**: 2026-11-23 · **Baseline**: `github/dev@0cc41336f`

## Entregue

- `FR-001` / `FR-002` / `T-001` (`#391`): escopo explícito por `organization_id` em `NotificationRepository`/`PgNotificationRepository` (`find_unread_by_user_and_organization`, `find_by_user_and_organization`, `mark_all_as_read_in_organization`), `ensure_member` e fail-closed `404` cross-org em `flora-api/src/routes/notifications.rs`, `target_url` opcional em `NotificationResponse`, endpoint `POST /api/v1/notifications/read-all`, testes unitários (`notifications::tests`), contrato DB `db_notifications_org_scoped_target_url_and_read_all` e trava lockstep `notifications: 5 → 6` (total `80`).
- `FR-003` / `T-002` (`#392`): `NotificationItem` (`models.ts`), `ApiNotification`/`toNotification` (`mappers.ts`), traduções `pt-BR` (`pt-BR.ts`), helpers puros em `utils/notifications.ts` e `NotificationStore.svelte.ts` em TDD `RED → GREEN`.
- `FR-004` / `T-003` (`#393`): rota `(app)/notifications/+page.svelte` (listagem, filtro Todas/Não lidas, marcação individual e em lote, links normalizados), badge reativo na barra lateral em `(app)/+layout.svelte`, remoção do atalho lateral `/dms` e da gaveta fake de IA, e página honesta em `(app)/dms/+page.svelte`.
- `FR-004` / `T-004` (`#394`): Passos 6 e 7 em `apps/frontend/tests/e2e/seed-flow.e2e.ts` validando ausência de placeholders, `/notifications` (`GET 200` + empty state), `/dms` honesto e conformidade WCAG 2.2 AA.
- `FR-005` / `T-005` (`#395`): Briefing de Compreensão (`tenant`), governança, docs append-only com recontagem determinística, roteiro, R4/R7 e ACR independente.

## SemVer

MINOR `0.9.0` → `0.10.0`: extensão retrocompatível de `/api/v1/notifications` (`target_url` opcional e `POST /read-all`), isolamento multi-tenant na camada de serviço/rota e entrega da UI `/notifications` com remoção de placeholders sem alteração de schema SQL ou required checks.

## Evidência

Portas locais verdes (`cargo fmt`, `clippy -D warnings`, `cargo test --workspace`, `check_gate_integrity.py` PASS 10 legs / 80 pisos, `svelte-check` 0/0, `lint` exit 0, Vitest `server` `18/69`, `vite build` `3m 10s`). Evidência remota de CI (`db-tests`, `e2e-browser`) e ACR será anexada em `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`.

## Rollback

Novo commit de reversão dos arquivos do ciclo 117 (reduzindo em lockstep `notifications` `6 → 5` caso `db_notifications_org_scoped_target_url_and_read_all` seja revertido); nenhuma migration SQL alterada e nenhum force-push.

## [NV]

Execução local direta com Postgres de staging real, tema escuro em navegador real, espelho Forgejo e deploy em produção.
