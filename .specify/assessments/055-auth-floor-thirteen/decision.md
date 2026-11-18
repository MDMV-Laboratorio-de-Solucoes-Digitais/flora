# Decision: 055 — Auth floor thirteen

**Verdict:** go (scoped)

## Decisão

Fechar a folga de 1 no leg `auth` do `db-tests`, exigindo os 13 testes que o
filtro descobre. Nenhuma decisão de autorização, ADR ou proteção de branch é alterada.

## Delimitadores

- 2 arquivos existentes, 3 linhas (`.github/workflows/rust.yml`,
  `scripts/ci/check_gate_integrity.py`).
- Artefatos do ciclo + notas datadas (append-only).
- PR independente; nenhum trabalho alheio tocado.

## NV

Execução do leg no runner do GitHub e CI ao vivo: **[NV]** — observar no PR
antes do merge (ADR-0003).
