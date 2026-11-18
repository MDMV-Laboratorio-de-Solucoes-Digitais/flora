# Clarify: 063 — Validate OSV run

**Data:** 2026-11-18T04:47Z

## Resoluções

1. **Disparar é autorizado?** Sim: `workflow_dispatch` é gatilho declarado no
   próprio workflow; não é merge/force-push/deleção de branch.
2. **Quantos disparos?** Um — evita ruído em `security-events`.
3. **Se falhar?** Registrar a falha e abrir candidato separado; não alterar
   o workflow no mesmo ciclo.
4. **Identidade 063?** `feature.json` aponta 051; `--dry-run` → 063.

## Pendências

Nenhum `NEEDS CLARIFICATION`.
