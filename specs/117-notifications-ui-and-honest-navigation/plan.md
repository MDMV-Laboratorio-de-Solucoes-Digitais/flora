# Plan: 117 — central de notificações escopada por organização e navegação honesta

**Data**: 2026-11-23 · **Baseline**: `github/dev@0cc41336f`

## Mapeamento FR/SC → Arquivos e Verificação

| FR / SC | Arquivos alvo | Verificação |
|---|---|---|
| FR-001 / SC-001 | `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/notification_repository.rs`, `flora-api/src/routes/notifications.rs` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (incl. `notifications::tests`) |
| FR-002 / SC-002 | `flora-tests/src/contract/notifications.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml` | `python3 scripts/ci/check_gate_integrity.py` (80 total) + job `db-tests` (`notifications: passed=6 floor=6`) |
| FR-003 / SC-003 | `models.ts`, `mappers.ts`, `pt-BR.ts`, `utils/notifications.ts`, `state/NotificationStore.svelte.ts`, `tests/unit/notifications.test.ts`, `tests/unit/mappers.test.ts` | TDD RED→GREEN (`notifications.test.ts`), `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server` |
| FR-004 / SC-004 / SC-005 | `routes/(app)/notifications/+page.svelte`, `routes/(app)/+layout.svelte`, `routes/(app)/dms/+page.svelte`, `tests/e2e/seed-flow.e2e.ts` | `bun run build` + job `e2e-browser` (`4 passed`, Passos 6 e 7, WCAG `"sem achados"`) |
| FR-005 / SC-005 | `2026-11-23-117-briefing-compreensao.md`, `2026-11-23-117-notifications-ui-and-honest-navigation.md`, `converge.md`, `.specify/validations/117-notifications-ui-and-honest-navigation/*`, `.specify/releases/117-notifications-ui-and-honest-navigation/release.md`, `STATE.md`, `PROGRESSO.md`, `research/index.md`, `OBJETIVO.md`, `ROADMAP.md` | `git diff --check`, recontagem `find`, ACR independente e CI remoto verde |

## Passos ordenados

1. Fechar `tasks.md` e `analyze.md` (somente leitura) alinhados às issues R6 existentes `#391`..`#395`.
2. Implementar `T-001` (`#391`): métodos por organização em `NotificationRepository`/`PgNotificationRepository`, isolamento + `target_url` + `POST /read-all` em `notifications.rs`, contrato DB e trava lockstep (`notifications: 5 -> 6`, total `80`).
3. Implementar `T-002` (`#392`) em TDD (`RED→GREEN`): `notifications.test.ts` → `models.ts`, `mappers.ts`, `pt-BR.ts`, `utils/notifications.ts`, `NotificationStore.svelte.ts`, `mappers.test.ts`.
4. Implementar `T-003` (`#393`) e `T-004` (`#394`): `(app)/notifications/+page.svelte`, `(app)/+layout.svelte` (badge + remoção de `/dms` e gaveta fake de IA), `(app)/dms/+page.svelte` honesto e Passos 6 e 7 em `seed-flow.e2e.ts`.
5. Executar verificação local completa, preencher `T-005` (`#395`) — Briefing (`tenant`), R4/R7 (`0.10.0`), docs append-only — abrir PR para `dev`, concluir CI + ACR + `evidence.md`, mesclar em `dev` e atualizar PR `#371` (`release/112-promotion` → `main`).
