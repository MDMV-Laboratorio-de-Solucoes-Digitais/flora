# Decision: Cobrir os legs DB órfãos do gate (053)

- **Slug**: 053-db-gate-uncovered-legs
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md (equivalente local; problem.md/concept.md ausentes no baseline) | shape.md | decision.md (anterior) | spec.md (ciclo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Seis dos 50 testes `#[ignore]` (`tasks` 4, `workspace` 2) não são executados por nenhum leg; `cargo test --workspace` pula ignorados (intake.md:9-11; research.md:1-4). |
| Evidence strength | strong | Contagem por filtro completo (`db::db_` → 4 e 2; filtro curto → 0) e leitura de `rust.yml:89-110` e `check_gate_integrity.py:42-49,110-114` (research.md:1-5). |
| Value vs. inaction | strong | Inação mantém 12% dos contratos DB fora da decisão de merge, reduzindo a confiança do gate abaixo do que a documentação sugere (define.md:Problema). |
| Feasibility / appetite | adequate | Escopo pequeno (2 linhas em `rust.yml`, 3 ajustes no guardião), mas risco de instabilidade dos novos legs no Postgres 18 do CI ainda **[NV]** (shape.md:Risco). |
| Strategic fit | strong | Reforça ADR-0003 sem criar required check nem alterar proteção de branch; os 4 checks permanecem válidos (define.md:FR-004; research.md:Decisão). |
| Risk posture | adequate | Legs ainda não executados no runner **[NV]**; filtro curto descartado por casar 0 testes; reversível por `git revert` (shape.md:Alternativas/Risco). |

## Verdict & Rationale

**go**, preservando o veredito histórico `go (scoped)` (decision.md anterior:3; spec.md:Status "Approved"). O scorecard justifica a decisão: a lacuna de cobertura é contada de forma determinística e a correção usa apenas dois `run_leg` no job existente, com pisos espelhados no guardião. Mantém-se o escopo do veredito anterior — 2 arquivos existentes (`rust.yml`, `check_gate_integrity.py`), artefatos do ciclo e notas datadas append-only; nenhuma decisão de autorização, ADR ou proteção de branch é alterada, e nenhum PR/worktree/stash alheio é tocado (decision.md anterior:Delimitadores). A execução real dos novos legs no runner e o estado de CI ao vivo são **[NV]**, a observar no PR antes de qualquer merge (ADR-0003). Não havia `problem.md`/`concept.md` no baseline; `define.md` e `shape.md` servem como equivalente local e a recomendação é a forma única descrita em `shape.md`.

## If needs-clarification

- **Blocking questions**: n/a — veredito é `go`; os itens remotos são acompanhamento, não bloqueio.
- **Revisit stage**: n/a

## If go — Handoff to `/speckit.specify`

- **Problem**: seis contratos DB `#[ignore]` (4 de tasks, 2 de workspace) nunca executam na CI, e o guardião registra apenas os seis pisos originais (43).
- **Chosen approach**: acrescentar dois `run_leg` dentro do step `Run DB-backed contract tests` de `rust.yml`, com os filtros completos `contract::tasks::db::db_` (piso 4) e `contract::workspace::db::db_` (piso 2), espelhando `tasks: 4` e `workspace: 2` em `FLOORS`, ajustando a regex para aceitar o segmento `db::` e atualizando o texto 6→8 pisos / 43→49 total (research.md:Decisão; shape.md:Forma escolhida).
- **In scope / out of scope**: in — `.github/workflows/rust.yml` (2 linhas), `scripts/ci/check_gate_integrity.py` (3 ajustes), artefatos do ciclo e notas datadas append-only; out — nomes de required checks, proteção de branch, ADR-0003, constituição, fail-open de env, piso de auth 12→13, C5/C6, merge automático e PRs/worktrees/stashes alheios.
- **Success metrics**: `cargo test --list` nos dois filtros → 4 e 2 testes; `check_gate_integrity.py` → PASS com 8 pisos / 49 total; gates locais (fmt/clippy/deny/build/test + frontend) verdes; execução real dos novos legs no runner **[NV]** — observar no PR antes do merge (ADR-0003).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: execução real dos dois novos legs no runner e estado de CI ao vivo?] **[NV]**; dívida documental de ADR-0003 (texto 43), fora de escopo; folga do piso de auth tratada no ciclo 055; fail-open de env tratado no ciclo 054.
