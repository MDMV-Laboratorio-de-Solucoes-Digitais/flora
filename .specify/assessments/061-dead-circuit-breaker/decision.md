# Decision: 061 — Dead circuit breaker

**Verdict:** go (scoped)

## Decisão

Remover o placeholder de circuit-breaker nunca compilado e seus artefatos
órfãos de configuração e erro, sem alterar comportamento de runtime.
Nenhuma decisão de autorização, migração ou CI é alterada.

## Delimitadores

- 3 arquivos em `flora-core/src/` (1 exclusão, 2 edições).
- Artefatos do ciclo + notas datadas.
- PR independente; nenhum trabalho alheio tocado.

## NV

CI/runner ao vivo: **[NV]** — observar no PR antes do merge (ADR-0003).
