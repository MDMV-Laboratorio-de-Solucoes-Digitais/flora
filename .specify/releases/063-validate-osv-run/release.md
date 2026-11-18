# Release: 063 — Validate OSV run

**Ciclo:** 063 · **Data:** 2026-11-18T04:50Z · **Baseline:** `github/dev@e1ff815bf`
**Tipo:** validação de CI advisory; **nenhuma tag, release ou deploy executado.**

## Entregue

- Execução real do workflow OSV (`workflow_dispatch --ref dev`), run
  `44239662140`, `conclusion=success`, registrada em `STATE.md`.
- Fechamento documental do `[NV]` de 052 sobre o pin `@v2.7.1`.
- Artefatos do ciclo (assess/spec/clarify/plan/checklist/tasks/analyze/
  converge/validate/release).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.17` (evidência de CI, sem API externa;
considera propostas 0.1.1–0.1.16 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — 1 execução disparada via gatilho declarado.
- `FR-002` — id/status/conclusion/timestamp registrados com ressalva.
- `FR-003` — nenhum código/workflow/ADR/gate alterado.
- `FR-004` — resultado registrado sem maquiagem.
- `SC-001`–`SC-004` — verificados (run, evidência, verificador, gates).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/063-validate-osv-run/validate.md` ·
`specs/063-validate-osv-run/converge.md` ·
`specs/063-validate-osv-run/analyze.md`.

## Rollback

`git revert` do commit `docs: registra validacao real do pin OSV`. O run remoto
não é reversível, mas é histórico e não alterou código; nenhuma tag/deploy.

## Não feito / NV

ADR-0003/0006, N3, C5/C6: escalados. Migração `ubuntu-latest` → Ubuntu 26
(anotação do runner): impacto futuro **[NV]**. Sem merge/auto-merge/force-push.
