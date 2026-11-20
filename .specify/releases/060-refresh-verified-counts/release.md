# Release: 060 — Refresh verified counts

**Ciclo:** 060 · **Data:** 2026-11-18T03:54Z · **Baseline:** `github/dev@262ac042f`
**Tipo:** documentação; **nenhuma tag, release ou deploy executado.**

## Entregue

- `docs/PROGRESSO.md`: duas seções datadas e append-only (03:51Z + correção
  03:52Z) com contagens verificadas por `find` e **[NV]** remoto.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.14` (documentação, sem API externa; considera
propostas 0.1.1–0.1.13 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — contagens atuais obtidas por `find` (12/12 conferidas).
- `FR-002` — 0 linhas removidas; nota 02:23Z intacta.
- `FR-003` — âncora e NV explícitos.
- `FR-004` — nenhum código, DDL, ADR ou gate alterado.
- `SC-001`–`SC-004` — verificados (números, diff, verificador, gates).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/060-refresh-verified-counts/validate.md` ·
`specs/060-refresh-verified-counts/converge.md` ·
`specs/060-refresh-verified-counts/analyze.md`.

## Rollback

`git revert` dos commits `docs: atualiza contagens verificadas do progresso` e
`docs: corrige contagens apos artefatos do ciclo`. Nenhum efeito externo.

## Não feito / NV

ADR-0003/0006 (aprovação de maintainer), N3 (FK com órfãos **[NV]** +
ADR-0006/SE), C5/C6: escalados, não executados. Estado remoto/CI: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
