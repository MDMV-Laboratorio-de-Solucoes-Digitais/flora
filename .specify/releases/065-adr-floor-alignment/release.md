# Release: 065 — ADR floor alignment

**Ciclo:** 065 · **Data:** 2026-11-18T13:09Z · **Baseline:** `github/dev@aef13277a`
**Tipo:** documentação de governança; **nenhuma tag, release ou deploy.**

## Entregue

- ADR-0003 e ADR-0006 alinhados aos pisos reais (`13/4/8/8/5/6/4/2`, total 51).
- Registro da divergência 50→51 em `STATE.md` (apêndice, histórico preservado).
- Artefatos do ciclo.

## SemVer

**Proposta:** patch `0.1.0` → `0.1.18` (documentação, sem API externa;
propostas 0.1.1–0.1.17 permanecem não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001/002` — 3 citações de piso atualizadas para 51.
- `FR-003` — nenhum código/gate/constituição/C5/C6 alterado.
- `FR-004` — autorização SE e divergência 50→51 registradas.
- `SC-001`–`SC-004` — verificados (grep, guardião, gates, diff de 2 ADRs).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum em `.specify/bugs/`; item veio da tabela de escalonamento.

## Evidência

`.specify/validations/065-adr-floor-alignment/validate.md` ·
`specs/065-adr-floor-alignment/converge.md` ·
`specs/065-adr-floor-alignment/analyze.md`.

## Rollback

`git revert` dos commits `docs(adr): alinha pisos do db-tests ao gate atual` e
`docs(adr): alinha piso do db-tests a 51`. Nenhum efeito externo.

## Não feito / NV

C5 (`RoleWrite`) e C6 (canário RLS) seguem autorizados e enfileirados;
produção **[NV]**. Sem merge/auto-merge/force-push nesta fase.
