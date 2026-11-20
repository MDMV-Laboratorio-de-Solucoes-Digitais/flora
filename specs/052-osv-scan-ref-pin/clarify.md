# Clarify: 052 — OSV scan ref pin

**Data:** 2026-11-17T23:39Z

## Resoluções

1. **`@v2` poderia ser um branch?** Não. `git ls-remote` sem truncamento não
   encontrou `refs/heads/v2` nem `refs/tags/v2`; há apenas tags `v2.x`.
2. **Qual tag escolher?** `v2.7.1`, recomendado pela doc oficial e verificado
   por `git ls-remote` (`32d3cd48…`).
3. **O pin muda inputs?** Não. O workflow no tag expõe `scan-args` e
   `fail-on-vuln`, exatamente os parâmetros já passados.
4. **Vira required check?** Não. Isso exigiria decisão de governança do alvo e
   alteração de ADR-0003 — fora de escopo.
5. **Identidade 052?** `feature.json` apontava 051 (fechado);
   `create-new-feature.sh --dry-run` → `052`. Nunca pelo nome do branch.

## Pendências

Nenhum `NEEDS CLARIFICATION`. Execução do job remoto: **[NV]**.
