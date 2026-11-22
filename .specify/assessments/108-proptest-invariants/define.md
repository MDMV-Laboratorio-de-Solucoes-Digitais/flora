# Define: 108 — proptest

**Problema**: invariantes de domínio são verificados por exemplos fixos; entradas arbitrárias podem revelar concessões indevidas de permissão ou limites incorretos.

**Escopo**: três propriedades puras — round-trip de `Permission`, não-concessão por string arbitrária, limites de `Pagination`, e equivalência do seletor wildcard ao `trim()`.

**Fora de escopo**: DB, gate, required checks, dependências novas, mudança de contrato.
