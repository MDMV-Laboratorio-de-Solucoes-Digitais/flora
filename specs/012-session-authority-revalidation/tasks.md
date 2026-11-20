# Tasks: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Status legend:** `[X]` done, `[~]` in progress, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Assessment & scope
- [X] T001 Intake/problema/conceito/decision `go` com base em auditoria/pesquisa.
- [X] T002 Research internacional + audit de contrato frontend↔API + skills relevantes.

## Phase 1 — Implementação
- [X] T010 Helper `authenticate_bearer` com validação de sessão no PostgreSQL.
- [X] T011 Extractors `AuthExtractor`/`UserContext`/`OrgContext` usando o helper.
- [X] T012 Frontend: corpo do 401 preservado no `ApiError` + teste determinístico.
- [X] T013 Testes DB-backed (7 casos) e remoção/ajuste de testes não-DB impactados.
- [X] T014 CI `db-tests` ampliado para `contract::auth::db_`.

## Phase 2 — Verificação & SDD
- [X] T020 fmt/check/clippy + testes Rust + DB tests em PG18 (12 passed).
- [X] T021 Frontend check/lint/test:unit(server)/build.
- [X] T022 Spec/plan/tasks/checklist, validação e release.
- [~] T023 PR para `dev`, CI verde, merge e sync `origin/dev` (manter `dev`/`main`).
