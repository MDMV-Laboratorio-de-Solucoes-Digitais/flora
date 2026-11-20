# Release: 075 — E2E de criação de tarefa

**Ciclo**: 075 · **Data**: 2026-11-19T12:10:18Z · **Baseline**: `dev@d8f647798`
**Tipo**: teste E2E. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/tests/e2e/seed-flow.e2e.ts:41-55` reforçado: `Create task` começa desabilitado, habilita após o título, descrição é preenchida e conferida, e título/descrição aparecem na região `To Do column`.
- Uma única seleção de org por suíte — o verificador isolado provou que um arquivo de teste adicional revogaria o jti da sessão e quebraria o smoke existente; escopo corrigido na fonte e registrado no `converge.md`.
- Artefatos completos: assess (5), spec, clarify, plan, checklist revisado, tasks, analyze (3 execuções), converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.2.5` → `0.2.6` (cobertura de teste, sem mudança de API ou comportamento). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` sessão one-shot preservada; `FR-002` seleção de org escopada; `FR-003` guard; `FR-004` descrição; `FR-005` coluna `To Do`; `FR-006` fronteira.
- `SC-001` descoberta por `--list`; `SC-002` check/lint/Prettier; `SC-003` diff restrito; `SC-004` gates e DB intacto.
- `T001`–`T005` executados; issue #193 linkada como origem do dado, não fechada sem autorização.
- Bug: nenhum em `.specify/bugs/` acionado.

## Evidência

`.specify/validations/075-task-create-e2e/validate.md` · `specs/075-task-create-e2e/converge.md` (correção de escopo) · `analyze.md`.

## Rollback

Reverter o commit do arquivo de teste em `dev/` com novo commit de reversão; sem force-push ou reescrita. Nada de código de produção, schema, required check ou produção é alterado.

## [NV]

Execução runtime do smoke, CI do PR, Forgejo, aprovações e merge: **[NV]**.
