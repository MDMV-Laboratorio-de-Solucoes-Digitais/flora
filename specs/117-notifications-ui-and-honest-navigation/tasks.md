# Tasks: 117 — central de notificações escopada por organização e navegação honesta

**Data**: 2026-11-23 · **Spec**: `specs/117-notifications-ui-and-honest-navigation/spec.md`

- T-001 (Issue #391): Escopo explícito por `organization_id` (`ensure_member` + cross-org fail-closed 404), campo opcional `target_url` em `NotificationResponse`, endpoint `POST /api/v1/notifications/read-all` e contrato DB com lockstep `notifications: 5 -> 6` (total `80`) [FR-001 / FR-002 / SC-001 / SC-002]
- T-002 (Issue #392): Criar `NotificationItem`/`ApiNotification`/`toNotification`, chaves `pt-BR`, utilitários puros em `apps/frontend/src/lib/utils/notifications.ts` e `NotificationStore.svelte.ts` em TDD `RED->GREEN` [FR-003 / SC-003]
- T-003 (Issue #393): Implementar rota `/notifications`, badge reativo na barra lateral, remover links/painéis placeholder (`/dms` e drawer vazio de IA) do layout principal e tornar `/dms` honesto [FR-004 / SC-004]
- T-004 (Issue #394): Expandir `apps/frontend/tests/e2e/seed-flow.e2e.ts` com Passos 6 e 7 para validar `/notifications`, `/dms` honesto, ausência de atalhos fantasma e auditoria WCAG 2.2 AA [FR-004 / SC-005]
- T-005 (Issue #395): Registrar Briefing de Compreensão `tenant`, validação R4, release `0.10.0`, docs append-only do ciclo 117, `test-roadmap.md`/`evidence.md` e ACR independente [FR-005 / SC-005]
