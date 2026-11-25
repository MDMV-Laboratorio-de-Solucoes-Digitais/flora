# Roteiro de testes — backfill/regularização de problem definition (42 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@df75b1bcd` · **Superfície**: CLI/artefatos versionados

## Escopo

Criar `problem.md` em 30 assessments sem `define.md`/`problem.md`, reescrever 5 `problem.md` finos e criar 7 `problem.md` canônicos ao lado de `define.md` fino. Nenhum veredito, intake, research, spec, gate, CI ou código é alterado.

## Pré-condições

- Worktree isolada `assessment/assess-define-backfill-20261125` a partir de `github/dev`.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- `intake.md` e `research.md` já existem nos 42 diretórios (backfills anteriores).

## Passos

1. **Segurança de caminho** — resolver `.specify`, `.specify/assessments` e os 42 diretórios; afirmar `is_dir`, não symlink, dentro da raiz. Automatizável: sim. Captura: terminal.
2. **Contagem** — 30 criações + 5 reescritas + 7 criações = 42 arquivos, um por slug. Automatizável: sim. Captura: terminal.
3. **Formato canônico** — H1 `# Problem Definition:`, `Slug` exato, seções Problem Statement, Affected Users & Stakeholders, Goals, Non-Goals, Success Metrics, Cost of Inaction, Open Questions. Automatizável: sim. Captura: terminal.
4. **Conteúdo** — `- **Users**:`, `- **Stakeholders**:`, pelo menos 1 métrica com `baseline:`, Cost of Inaction não vazio e `[NEEDS CLARIFICATION:]` em Open Questions; fatos antigos preservados nos 5 reescritos e nos 7 com `define.md` intacto. Automatizável: sim (texto) + inspeção. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check` e `git status --short` com apenas os 42 paths autorizados (além de validação/notas). Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 não mesclado. Automatizável: sim. Captura: terminal.

## Rollback

Remover os 37 arquivos novos e reverter os 5 reescritos via commit reverso explícito; nada mais é tocado. Nenhuma mudança de runtime, gate ou CI.
