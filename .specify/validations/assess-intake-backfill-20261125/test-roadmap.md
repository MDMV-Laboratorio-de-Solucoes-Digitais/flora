# Roteiro de testes — backfill de intake (13 assessments)

**Data**: 2026-11-25 · **Base**: `github/dev@7ac614425` · **Superfície**: CLI/artefatos versionados

## Escopo

Criar `intake.md` ausente em 13 diretórios de `.specify/assessments/` que já possuem `decision.md`. Nenhuma decisão é alterada; nenhum código, gate, CI, ADR ou migration é tocado.

## Pré-condições

1. **Segurança de caminho** — resolver `.specify`, `.specify/assessments` e cada diretório de slug; afirmar `is_dir=True`, `is_symlink=False`, `inside=True`. Automatizável: sim. Captura: terminal.
- `.specify` e `.specify/assessments` são diretórios reais, não symlinks, dentro da raiz.
- Os 13 slugs tinham `decision.md` e nenhum `intake.md` no baseline.

## Passos

1. **Segurança de caminho** — resolver `.specify`, `.specify/assessments` e cada diretório de slug; afirmar `is_dir=True`, `is_symlink=False`, `inside=True`. Automatizável: sim. Captura: terminal.
2. **Contagem** — confirmar exatamente 13 novos `intake.md`, um por slug listado no ledger. Automatizável: sim. Captura: terminal.
3. **Estrutura SpecKit** — para cada arquivo: `# Idea Intake:`, `## Idea (as captured)`, `## Restated`, `## Origin & Context`, `## First-Glance Unknowns`, `**Slug**: <slug>` e 4–6 `[NEEDS CLARIFICATION:` (contrato do brief; entregues 5–7). Automatizável: sim. Captura: terminal.
4. **Conteúdo não genérico** — afirmar que cada intake cita `decision.md` e/ou `specs/<slug>/spec.md`, com bytes entre 2.000 e 3.500. Automatizável: sim. Captura: terminal.
5. **Gate determinístico** — `python3 scripts/ci/check_gate_integrity.py` com `TMPDIR` gravável. Esperado: `PASS`, exit 0. Automatizável: sim. Captura: terminal.
6. **Higiene** — `git diff --check`, `git status --short`, confirmação de que apenas os paths autorizados foram criados. Automatizável: sim. Captura: terminal.
7. **Preservação alheia** — checkout compartilhado sujo e worktrees alheias inalterados; PR #410 não mesclado. Automatizável: sim. Captura: terminal.

## Rollback

Remover os 13 arquivos criados e o diretório de validação; nenhum outro path é tocado. Nenhuma mudança de runtime, gate ou CI.
