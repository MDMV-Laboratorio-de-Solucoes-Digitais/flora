# Clarify: 054 — DB env fail-closed

**Data:** 2026-11-18T00:30Z

## Resoluções

1. **Por que não editar os 50 testes?** O gate de merge é o que precisa ser
   confiável; 50 edições mecânicas aumentariam risco sem ganho de gate.
2. **`${VAR:?}` é seguro com `set -u`?** Sim: a expansão fatal ocorre antes de
   qualquer comando e encerra o step com exit != 0.
3. **Isso quebra a execução local sem DB?** O step é de CI; localmente,
   `cargo test --ignored` sem env ainda reporta passes — dívida documentada,
   fora deste escopo.
4. **Novo check?** Não: mudaria os 4 contextos de proteção (risco de deadlock).
5. **Identidade 054?** `feature.json` aponta 051; `--dry-run` → `054` por inventário.

## Pendências

Nenhum `NEEDS CLARIFICATION`. Runner ao vivo: **[NV]**.
