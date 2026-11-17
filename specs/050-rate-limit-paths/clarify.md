# Clarify: 050 — Rate limit paths

**Data:** 2026-11-17T16:10Z

## Resoluções

1. **Qual path o middleware vê?** O completo (`/api/v1/auth/login`), confirmado em
   `rate_limit.rs:155` e no `.nest("/api/v1/auth", ...)` do router.
2. **Manter compatibilidade legada?** Sim: `ends_with` cobre tanto o prefixo atual
   quanto caminhos sem prefixo, sem duplicar condições.
3. **Alterar limites?** Não. Apenas a classificação muda.
4. **Identidade 050?** `feature.json` apontava 049; `--number 050` por inventário
   (049 já mergeado), nunca pelo nome do branch.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
