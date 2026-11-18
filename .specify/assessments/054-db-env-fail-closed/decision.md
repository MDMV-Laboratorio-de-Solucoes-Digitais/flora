# Decision: 054 — DB env fail-closed

**Verdict:** go (scoped)

## Decisão

Tornar o job `db-tests` fail-closed: o próprio `run_leg` rejeita a ausência das
duas variáveis de DB, e o guardião determinístico passa a exigir que elas estejam
declaradas no job. Nenhum required check, teste ou proteção de branch é alterado.

## Delimitadores

- 2 arquivos de código existentes (`.github/workflows/rust.yml`,
  `scripts/ci/check_gate_integrity.py`).
- Artefatos do ciclo + notas datadas (append-only).
- PR independente; nenhum PR/worktree/stash alheio tocado.

## NV

Execução do step no runner do GitHub e estado de CI ao vivo: **[NV]** — observar
no PR antes de qualquer merge (ADR-0003).
