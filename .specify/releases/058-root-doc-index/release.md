# Release: 058 — Root doc index

**Ciclo:** 058 · **Data:** 2026-11-18T03:08Z · **Baseline:** `github/dev@58921a337`
**Tipo:** documentação; **nenhuma tag, release ou deploy executado.**

## Entregue

- `index.md` na raiz: 26 linhas, 8 links relativos, cabeçalho datado com âncora
  e **[NV]**, agrupamento Direção / Estado verificado / Referência.
- Notas datadas em `OBJETIVO.md`/`ROADMAP.md`/`STATE.md` (append-only).

## SemVer

**Proposta:** patch `0.1.0` → `0.1.12` (documentação, sem API externa; considera
propostas 0.1.1–0.1.11 ainda não aplicadas). Nenhuma tag criada.

## Changelog (IDs)

- `FR-001` — 8 links relativos válidos (7 exigidos + constituição).
- `FR-002` — data, âncora e NV no cabeçalho/rodapé.
- `FR-003` — sem duplicação de contagens/dívidas (correção pós-verificação).
- `FR-004` — nenhum código/workflow/ADR alterado.
- `SC-001`–`SC-004` — verificados (links, 26 linhas, verificador, gates).
- `T001`–`T004` — concluídas; evidência em `validate.md`.
- Bug: nenhum reportado em `.specify/bugs/`; candidato veio do ranking 80/20.

## Evidência

`.specify/validations/058-root-doc-index/validate.md` ·
`specs/058-root-doc-index/converge.md` ·
`specs/058-root-doc-index/analyze.md`.

## Rollback

`git revert` dos commits `docs: adiciona indice de documentacao na raiz` e
`docs: remove duplicacao de conteudo do indice`. Nenhum efeito externo.

## Não feito / NV

ADR-0003/0006 (aprovação de maintainer), README, falso positivo local, N3,
C5/C6: fora de escopo ou escalados. Estado remoto/CI: **[NV]**.
Sem merge/auto-merge/force-push nesta fase.
