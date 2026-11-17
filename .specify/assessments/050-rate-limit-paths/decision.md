# Decision: 050 — Rate limit paths

**Verdict:** go (scoped)

## Decisão

Extrair classificador puro de endpoint e corrigir os paths casados pelo rate limiter,
preservando limites, janelas e comportamento de resposta.

## Delimitadores

- Apenas `flora-api/src/middleware/rate_limit.rs` (código + testes).
- Nenhuma rota, autoria, CORS, constituição, ADR, workflow ou limite alterado.
- Nenhum bug report em aberto; nenhum fluxo bug paralelo necessário.

## NV

Runner/CI ao vivo e comportamento em produção: **[NV]**.
