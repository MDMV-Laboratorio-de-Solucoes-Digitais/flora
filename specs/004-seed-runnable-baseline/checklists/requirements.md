# Requirements Checklist — 004 Seed Runnable Baseline

| ID | Requirement | Status | Evidence / Notes |
|---|---|---|---|
| CHK-001 | `OBJETIVO.md` criado com estado verificado vs inferido, gaps e não-objetivos | [X] | `OBJETIVO.md` |
| CHK-002 | `ROADMAP.md` criado/atualizado com prioridades 80/20 e critérios de avanço | [X] | `ROADMAP.md` |
| CHK-003 | Pesquisa consolidada documentada com fontes de alta confiabilidade | [X] | `docs/research/2026-11-15-flora-seed-80-20-research.md` |
| CHK-004 | Assessment 004 com intake/research/decision `go` e escopo delimitado | [X] | `.specify/assessments/004-seed-runnable-baseline/` |
| CHK-005 | Spec/plan/tasks do ciclo 004 criados e rastreáveis | [X] | `specs/004-seed-runnable-baseline/` |
| CHK-006 | Compose dev para Postgres/Valkey/Meilisearch sem segredos reais | [X] | `docker-compose.yml` |
| CHK-007 | Smoke runtime (`docker compose` + API + migrações + health) executado com evidência | [~] | **NV**: Docker/stack não executados nesta sessão |
| CHK-008 | Nenhuma ação fora de autonomia/autorização (deploy prod, merge/release, segredos) | [X] | Escopo restrito a docs/config dev |
