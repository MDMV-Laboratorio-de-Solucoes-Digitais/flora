# Release: 056 — Docs research index

**Ciclo:** 056 · **Data:** 2026-11-18T02:29Z · **Baseline:** `github/dev@94fcf19ca`
**Tipo:** documentação; **nenhuma tag, release ou deploy executado.**

## Entregue

- `docs/research/index.md`: índice determinístico de 135 documentos, com
  metodologia, contagens verificadas e comando de regeneração.
- `docs/PROGRESSO.md`: nota datada de estado verificado (âncora, gates,
  contagens, funcionamento, dívidas, **[NV]**).
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.10` (documentação, sem API externa; considera
propostas 0.1.1–0.1.9 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001`/`FR-002` — 135 entradas, links válidos, geração determinística e idempotente.
- `FR-003` — `PROGRESSO.md` com nota verificada e **[NV]** inline.
- `FR-004` — nenhum código, workflow, ADR ou constituição alterado.
- `SC-001`–`SC-004` — verificados (contagem, links, verificador, gates).
- `T001`–`T005` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/056-docs-research-index/validate.md` ·
`specs/056-docs-research-index/converge.md` ·
`specs/056-docs-research-index/analyze.md`.

## Rollback

`git revert` dos commits `docs: adiciona indice verificados de research`,
`docs: adiciona nota de progresso verificada` e das duas correções posteriores.
Nenhum efeito externo; nenhuma tag/deploy envolvido.

## Não feito / NV

ADR-0003/0006 (_texto 43_, aprovação de maintainer), `index.md` na raiz, N3,
C5/C6: fora de escopo. Estado remoto/CI: **[NV]**. Sem merge/auto-merge/force-push.
