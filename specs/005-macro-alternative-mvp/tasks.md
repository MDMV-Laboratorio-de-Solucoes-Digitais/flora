# Tasks: 005 — Macro Alternative MVP Reframe & 80/20 Direction

**Status legend:** `[X]` done, `[~]` partial/NV, `[ ]` pending

## Phase 0 — Research & synthesis
- [X] T001 Coletar evidências públicas iniciais do Macro (README, LICENSE, Cargo.toml, package.json, escopo de produto).
- [X] T002 Delegar pesquisas em background: mapeamento codebase Flora, análise Macro/debloat, padrões cheap-VPS/self-hosted e stack Svelte 5/Tauri.
- [X] T003 Consolidar pesquisa em `docs/research/2026-11-15-macro-alternative-80-20-research.md` com fontes, recomendações 80/20 e limitações/NV.

## Phase 1 — Assessment (SpecKit)
- [X] T010 Criar `.specify/assessments/005-macro-alternative-mvp/intake.md`.
- [X] T011 Criar `.specify/assessments/005-macro-alternative-mvp/research.md`.
- [X] T012 Criar `.specify/assessments/005-macro-alternative-mvp/decision.md` com veredito `go`.

## Phase 2 — Direction artifacts
- [X] T020 Atualizar `OBJETIVO.md` com norte “alternativa ao Macro para PMEs em VPS barata”, estado verificado/NV, MVP 80/20 e não-objetivos.
- [X] T021 Atualizar `ROADMAP.md` com prioridades P0–P3, debloat/Postgres-first e critérios de avanço.

## Phase 3 — Spec/plan/tasks
- [X] T030 Criar `specs/005-macro-alternative-mvp/spec.md`.
- [X] T031 Criar `specs/005-macro-alternative-mvp/plan.md` (inclui handoff para próximo slice técnico).
- [X] T032 Criar `specs/005-macro-alternative-mvp/tasks.md` e `checklists/requirements.md`.

## Phase 4 — Follow-ups (next cycles, not done here)
- [ ] T040 Ciclo 006: implementar fatia vertical mínima segura/verificável (auth/contexto consistente + fluxo núcleo workspace/canal/mensagem ou tarefa) com evidência.
- [ ] T041 Alinhar contrato frontend↔backend (Svelte 5) para o fluxo núcleo e/ou smoke API reproduzível.
- [ ] T042 Reduzir dependências obrigatórias no Seed (Postgres-first: FTS/jobs/realtime simples onde viável) e tornar Valkey/Meilisearch/OIDC opcionais por perfil.
- [ ] T043 Endurecer segurança/operabilidade (JWKS/validação completa conforme aplicável, RLS/policies ou controles equivalentes documentados, health/readiness real, backup/restore).
- [~] T044 Refinar pesquisa com benchmarks/feedback de PMEs e atualização incremental do consolidado — **NV** (requer ambiente/usuários/medições).
