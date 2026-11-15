# Tasks: 004 — Seed Runnable Baseline (Dev)

**Status legend:** `[X]` done, `[~]` partial/NV, `[ ]` pending

## Phase 0 — Research and consolidation
- [X] T001 Consolidar achados internos (código/specs/assessments/CI/verificação local 2026-11-15) e pesquisa externa em `docs/research/2026-11-15-flora-seed-80-20-research.md`.
- [X] T002 Criar `OBJETIVO.md` com estado real, verificado vs inferido, gaps, não-objetivos e próximo 80/20.
- [X] T003 Criar/atualizar `ROADMAP.md` com marcos, backlog priorizado (P0-P3) e critérios de avanço.

## Phase 1 — Assessment and decision
- [X] T010 Criar `.specify/assessments/004-seed-runnable-baseline/intake.md`.
- [X] T011 Criar `.specify/assessments/004-seed-runnable-baseline/research.md`.
- [X] T012 Criar `.specify/assessments/004-seed-runnable-baseline/decision.md` com veredito `go` e escopo delimitado.

## Phase 2 — Spec/plan/tasks
- [X] T020 Criar `specs/004-seed-runnable-baseline/spec.md`.
- [X] T021 Criar `specs/004-seed-runnable-baseline/plan.md`.
- [X] T022 Criar `specs/004-seed-runnable-baseline/tasks.md` e `checklists/requirements.md`.

## Phase 3 — Dev enablement (reproducible baseline)
- [X] T030 Adicionar `docker-compose.yml` (dev) com Postgres, Valkey e Meilisearch (healthchecks, volumes, portas localhost, sem segredos reais).
- [~] T031 Validar execução `docker compose up -d` e smoke (`cargo run -p flora-api`, health/migrações, seed opcional) — **NV** neste ambiente (requer Docker/recursos/serviços).
- [ ] T032 Ajustar compose/docs conforme feedback de execução real (se necessário) em ciclo seguinte.

## Phase 4 — Follow-ups (recommended next cycles)
- [ ] T040 Implementar/verificar fatia vertical auth→org/workspace→mensagem/tarefa com evidência.
- [ ] T041 Fechar gaps PKCE (mock provider/JWKS success path, testes de estado) e consistência auth nas rotas.
- [ ] T042 Integrar indexação busca e eventos→notificações; alinhar contratos de arquivos/binário e quota.
- [ ] T043 Tornar health/readiness reflexo de dependências críticas e estabilizar verificações frontend/CI.
