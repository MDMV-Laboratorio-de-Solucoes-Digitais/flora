# Release: 049 — Local logger

**Ciclo:** 049 · **Data:** 2026-11-17T15:39Z · **Baseline:** `github/dev@55363b3ca`
**Tipo:** frontend; **nenhuma tag, release ou deploy executado.**

## Entregue

- `apps/frontend/src/lib/utils/logger.ts`: coletor local-only, buffer limitado a 200, `snapshot()`.
- `apps/frontend/src/lib/utils/logger.spec.ts`: 2 testes no projeto `server`.
- `specs/002-svelte-frontend-rewrite/contracts/logger-contract.md`: contrato reconciliado.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`.

## SemVer

**Proposta:** patch `0.1.0` → `0.1.3` (mudança de comportamento interno do frontend,
sem API externa; considera as propostas 0.1.1 e 0.1.2 ainda não aplicadas).
Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — zero requisições de rede no logger.
- `FR-002` — assinaturas públicas preservadas.
- `FR-003` — buffer limitado a 200 entradas.
- `FR-004` — `snapshot()` com cópia defensiva.
- `FR-005` — contrato 002 atualizado.
- `T001`–`T006` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; o defeito veio do ranking 80/20.

## Evidência

`.specify/validations/049-local-logger/validate.md` ·
`specs/049-local-logger/converge.md` ·
`specs/049-local-logger/analyze.md`.

## Rollback

`git revert` dos commits do ciclo restaura `logger.ts`, o contrato 002 e remove o teste
e as notas. Nenhum efeito externo: sem tag, publicação, deploy ou mudança de backend.

## Não feito / NV

Backend de telemetria e OTel: fora de escopo (decisão futura). CI/PR/aprovações ao
vivo: **[NV]**. PR #262 e #263 não foram alterados. Sem merge/auto-merge/force-push.
