# Decision: 053 — DB gate uncovered legs

**Verdict:** go (scoped)

## Decisão

Elevar a CI cobrindo os seis contratos DB hoje órfãos, com dois novos legs
dentro do job `db-tests` existente e pisos espelhados no guardião determinístico.
Nenhuma decisão de autorização, ADR ou proteção de branch é alterada.

## Delimitadores

- 2 arquivos de código existentes (`rust.yml`, `check_gate_integrity.py`).
- Artefatos do ciclo + notas datadas de estado (append-only).
- PR independente; nenhum PR/worktree/stash alheio tocado.

## NV

Execução real dos novos legs no runner do GitHub e estado de CI ao vivo: **[NV]** —
observar no PR antes de qualquer merge (ADR-0003).
