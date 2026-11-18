# Clarify: 055 — Auth floor thirteen

**Data:** 2026-11-18T01:35Z

## Resoluções

1. **Por que 13 e não 14?** O filtro descobre exatamente 13; 14 criaria folga
   fictícia e exigiria teste inexistente.
2. **Por que editar os dois lugares?** O guardião casa a linha `run_leg auth`
   com `FLOORS["auth"]`; editar só um deixa o gate RED (prova de lockstep).
3. **Os 13 são estáveis?** Verificador isolado: `#[ignore]` + env guard,
   `--test-threads=1`, UUIDv7, janelas temporais robustas; nenhum `cfg` extra;
   13/13 passaram com Postgres real no ciclo 053. Sem flake aparente.
4. **Atualizar ADR-0003?** Não: documento de governança exige aprovação de
   maintainer; registrado como dívida (texto antigo 43).
5. **Identidade 055?** `feature.json` aponta 051; `--dry-run` → 055 por inventário.

## Pendências

Nenhum `NEEDS CLARIFICATION`. Runner ao vivo: **[NV]**.
