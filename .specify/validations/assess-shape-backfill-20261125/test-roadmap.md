# Roteiro de testes — backfill/regularização de concept (54 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@266437bbfa` · **Superfície**: CLI/artefatos versionados

## Escopo

Criar `concept.md` em 30 assessments sem `shape.md`/`concept.md`, reescrever 2 `concept.md` finos e criar 22 `concept.md` ao lado de `shape.md` fino. Como `116-chat-author-identity` não tinha `problem.md` (pré-requisito do comando), o artefato equivalente local `define.md` foi usado para gerar o `problem.md` canônico; os `shape.md`/`define.md` antigos não foram alterados.

## Pré-condições

- Worktree isolada `assessment/assess-shape-backfill-20261125` a partir do SHA remoto `266437bbfa`.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- `intake.md`, `research.md`, `decision.md` e `problem.md`/`define.md` existem nos 54 diretórios.

## Passos

1. **Segurança de caminho** — resolver os diretórios e afirmar `is_dir`, não symlink, dentro da raiz. Automatizável: sim. Captura: terminal.
2. **Contagem** — 54 `concept.md` (52 novos + 2 reescritos) e 1 `problem.md` novo de pré-requisito (`116`). Automatizável: sim. Captura: terminal.
3. **Formato canônico** — H1 `# Concept:`, `Slug` exato, `Recommended option`, 2–3 `### Option`, cada uma com Sketch/Appetite/Trade-offs/Rabbit holes, `Recommendation`, `Out of Scope` e `Assumptions to Validate`. Automatizável: sim. Captura: terminal.
4. **Conteúdo** — opções distintas, appetite honesto, recomendação ligada a goals/métricas do problema, nenhuma spec/API/tarefa detalhada; fatos antigos preservados nas 2 reescritas. Automatizável: sim (texto) + inspeção. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável e `cargo clean` prévio no target da worktree. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check` e `git status --short` com apenas os paths autorizados. Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 (`dev→main`) já mesclado por `lfgranja`, não tocado neste ciclo. Automatizável: sim. Captura: terminal.

## Rollback

Remover os 53 arquivos criados e reverter os 2 reescritos via commit reverso explícito; nada mais é tocado. Nenhuma mudança de runtime, gate ou CI.
