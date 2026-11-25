# Roteiro de testes — backfill/regularização de research (18 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@e88dc8e4d` · **Superfície**: CLI/artefatos versionados

## Escopo

Criar `research.md` em 13 assessments que não tinham o artefato e regularizar outros 5 que estavam abaixo de 900 bytes/fora do formato canônico. Nenhum veredito, intake, spec, gate, CI ou código é alterado.

## Pré-condições

- Worktree isolada `assessment/assess-research-backfill-20261125` a partir de `github/dev`.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- Hosts web não allowlisted não foram buscados; quando encontrados, registrados como `auto-refused`.

## Passos

1. **Segurança de caminho** — resolver `.specify`, `.specify/assessments` e os 18 diretórios; afirmar `is_dir`, não symlink, dentro da raiz. Automatizável: sim. Captura: terminal.
2. **Contagem** — 13 arquivos novos + 5 modificados, exatamente um por slug atribuído. Automatizável: sim. Captura: terminal.
3. **Formato canônico** — H1 `# Idea Research:` e 8 seções: Users & Demand, Prior Art, Market & Context, Data & Constraints, Evidence Against the Idea, Gaps & Open Questions, Sources (mais `Slug` exato). Automatizável: sim. Captura: terminal.
4. **Qualidade de evidência** — ≥4 fontes em Sources, `[NEEDS CLARIFICATION:]` em Gaps, `Evidence Against` não vazio, fatos antigos preservados nos 5 regularizados. Automatizável: sim (texto) + inspeção. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check` e `git status --short` com apenas os 18 paths autorizados (além de validação/notas). Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 não mesclado. Automatizável: sim. Captura: terminal.

## Rollback

Restaurar os 13 arquivos novos (remoção) e os 5 antigos via commit reverso explícito; nada mais é tocado. Nenhuma mudança de runtime, gate ou CI.
