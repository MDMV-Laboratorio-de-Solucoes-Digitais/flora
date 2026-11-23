# Analyze: 117 (READ-ONLY)

**Data**: 2026-11-23 · **Baseline**: `github/dev@0cc41336f`

## Checagem cruzada (`spec.md` ↔ `plan.md` ↔ `tasks.md` ↔ GitHub `#391`..`#395`)

- **Rastreabilidade FR → Tasks → Issues R6**:
  - `FR-001` / `FR-002` / `SC-001` / `SC-002` → `T-001` → [#391](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/391) (`NotificationRepository`, `PgNotificationRepository`, `notifications.rs`, `flora-tests/src/contract/notifications.rs`, `check_gate_integrity.py`, `.github/workflows/rust.yml`)
  - `FR-003` / `SC-003` → `T-002` → [#392](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/392) (`models.ts`, `mappers.ts`, `pt-BR.ts`, `utils/notifications.ts`, `NotificationStore.svelte.ts`, `tests/unit/notifications.test.ts`, `tests/unit/mappers.test.ts`)
  - `FR-004` / `SC-004` → `T-003` → [#393](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/393) (`(app)/notifications/+page.svelte`, `(app)/+layout.svelte`, `(app)/dms/+page.svelte`)
  - `FR-004` / `SC-005` → `T-004` → [#394](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/394) (`apps/frontend/tests/e2e/seed-flow.e2e.ts`)
  - `FR-005` / `SC-005` → `T-005` → [#395](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/395) (Briefing `tenant`, governança, docs append-only, R4/R7 `0.10.0`, `test-roadmap.md`/`evidence.md`, ACR)
- **Nota de aderência (`#395`)**: o repositório não possui `CHANGELOG.md` na raiz; o changelog versionado do ciclo reside em `.specify/releases/117-notifications-ui-and-honest-navigation/release.md` (SemVer `0.9.0 → 0.10.0`), mantendo consistência com os ciclos anteriores (`114`–`116`).
- **Órfãos / Conflitos**: 0 requisitos órfãos; 0 tarefas órfãs; 0 conflitos com Constituição v1.4.0 ou ADR-0003.
- **Lockstep gate check**: `FLOORS["notifications"]` e `run_leg notifications ...` atualizados em par (`5 -> 6`, total `79 -> 80`), preservando os 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) e `ALLOWED_JOB_IF`.
- **Ordem normativa**: `tasks` concluído antes de `analyze`; `analyze` somente leitura com mapping R6 existente `#391`..`#395` antes de `12 implement`.
- **Resumo**: `fr=5/5 sc=5/5 tasks=5/5 critical=0 go=True`.

## [NV]

Execução local direta contra Postgres de staging real, tema escuro em navegador real e staging/produção externos.
