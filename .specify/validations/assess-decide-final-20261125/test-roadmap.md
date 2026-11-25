# Roteiro de testes — canonicalização final de decision.md (42 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@2b2658e9b21f` · **Superfície**: CLI/artefatos versionados

## Escopo

Reescrever os 42 `decision.md` restantes que não tinham scorecard canônico, completando a normalização de 111/111 assessments. Vereditos históricos (`go`) preservados. Nenhuma spec, gate, CI, código ou outro artefato é alterado.

## Pré-condições

- Worktree isolada `assessment/assess-decide-final-20261125` a partir do SHA remoto `2b2658e9b21f`.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- Cada diretório tem `decision.md` e os artefatos de entrada equivalentes (`problem.md`/`define.md`, `concept.md`/`shape.md`).

## Passos

1. **Segurança de caminho** — resolver diretórios; afirmar `is_dir`, não symlink, dentro da raiz. Automatizável: sim. Captura: terminal.
2. **Contagem** — 42 `decision.md` reescritos; 111/111 com scorecard canônico ao final. Automatizável: sim. Captura: terminal.
3. **Formato canônico** — H1 `# Decision:`, Slug, Decided, Verdict válido, scorecard com exatamente 6 critérios entre `## Scorecard` e `## Verdict & Rationale`, ratings permitidos e ambas as seções condicionais. Automatizável: sim. Captura: terminal.
4. **Conteúdo** — veredito histórico preservado, `Evidence strength` nunca `weak`/`unknown` em `go`, matriz P01–P34 e contagens do assessment Sanderson preservadas, 188 IDs do assessment de arquitetura preservados. Automatizável: sim (texto) + inspeção. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável e `cargo clean` prévio. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check` e `git status --short` com apenas os 42 paths autorizados (além de validação/notas). Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 já mesclado por humano, não tocado. Automatizável: sim. Captura: terminal.

## Rollback

Reverter os 42 `decision.md` via commit reverso explícito; nada mais é tocado. Nenhuma mudança de runtime, gate ou CI.
