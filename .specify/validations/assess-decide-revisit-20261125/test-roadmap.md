# Roteiro de testes — revisita canônica de decision.md (62 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@f13d321c7a2b` · **Superfície**: CLI/artefatos versionados

## Escopo

Reescrever 62 `decision.md` com scorecard canônico de 6 critérios, veredito, racional e handoff, preservando o veredito histórico (`go` em todos os 62). Como `114-wcag-ci-guard` e `115-files-list-persistence` tinham apenas `define.md`/`shape.md`, foram criados seus `problem.md` e `concept.md` canônicos como pré-requisitos do comando.

## Pré-condições

- Worktree isolada `assessment/assess-decide-revisit-20261125` a partir do SHA remoto `f13d321c7a2b`.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- Todos os 62 diretórios têm `intake.md`, `research.md`, `problem.md` e `concept.md` (os dois últimos criados quando necessário).

## Passos

1. **Segurança de caminho** — resolver diretórios; afirmar `is_dir`, não symlink, dentro da raiz. Automatizável: sim. Captura: terminal.
2. **Contagem** — 62 `decision.md` reescritos + 4 artefatos de pré-requisito (`problem.md`/`concept.md` de 114 e 115). Automatizável: sim. Captura: terminal.
3. **Formato canônico** — H1 `# Decision:`, Slug, Decided, Verdict válido, scorecard com 6 critérios e ratings `strong|adequate|weak|unknown`, `Verdict & Rationale` e ambas as seções condicionais. Automatizável: sim. Captura: terminal.
4. **Conteúdo** — veredito histórico preservado (ou mudança justificada), `Evidence strength` nunca `weak`/`unknown` em `go`, handoff ligado a problem/concept, nenhum spec/plano/tarefa. Automatizável: sim (texto) + inspeção. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável e `cargo clean` prévio. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check` e `git status --short` com apenas os paths autorizados. Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 já mesclado por humano, não tocado. Automatizável: sim. Captura: terminal.

## Rollback

Reverter os 62 `decision.md` e remover os 4 artefatos de pré-requisito via commit reverso explícito; nada mais é tocado. Nenhuma mudança de runtime, gate ou CI.
