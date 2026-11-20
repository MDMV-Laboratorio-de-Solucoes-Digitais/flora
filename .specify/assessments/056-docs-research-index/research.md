# Research: 056 — Docs research index

**Data:** 2026-11-18T02:12Z · **Evidência local determinística**; remoto **[NV]**.

## Primária (executada)

1. `find docs/research -type f` → 135 arquivos, todos `*.md`, sem subdiretórios.
2. Convenção de nomes: `YYYY-MM-DD[-NNN]-slug.md`. O sufixo numérico, quando
   presente, corresponde ao ciclo (ex.: `…-046-auth-hardening.md`).
3. `docs/PROGRESSO.md`, `docs/research/index.md` e `index.md` ausentes;
   `OBJETIVO.md`/`ROADMAP.md` já registram a dívida (notas de 17/11).
4. `docs/adr/` tem 6 ADRs; `docs/agents/` tem 2 documentos — não fazem parte
   deste índice (escopo apenas `docs/research/`).

## Decisão

Gerar `docs/research/index.md` de forma determinística a partir dos nomes de
arquivo e do primeiro cabeçalho `# ` de cada documento, agrupando por data.
Gerar `docs/PROGRESSO.md` como nota datada de estado verificado (contagens por
`find`, âncora, gates, dívidas, **[NV]**), preservando o formato append-only
das notas em `OBJETIVO.md`/`ROADMAP.md`.

## Alternativas descartadas

- Índice manual: 135 entradas, risco de omissão e drift imediato.
- Incluir ADR/agents no índice: escopo diferente; não casaria a contagem de `find`.
- `.generated.md`: exigiria revalidação de fonte específica; não é o caso.
- Editar ADR-0003 no mesmo PR: governança, aprovação de maintainer pendente.

## Cobertura idiomática

PT-BR (docs do alvo) + EN (nomes de arquivos e títulos extraídos). **[NV]** para
qualquer afirmação remota.
