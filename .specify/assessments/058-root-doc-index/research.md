# Research: 058 — Root doc index

**Data:** 2026-11-18T03:03Z · **Evidência local determinística**; remoto **[NV]**.

## Primária (executada)

1. `test -f index.md` → ausente; `ls -1 *.md` confirma.
2. `README.md` — apresentação do projeto; não é índice de estado verificado.
3. `OBJETIVO.md`/`ROADMAP.md` — notas append-only por ciclo; `docs/PROGRESSO.md`
   — estado verificado datado; `docs/research/index.md` — 135 entradas.
4. `docs/adr/` — 6 ADRs (0001–0006); `AGENTS.md` — convenções do repo.
5. `.specify/memory/STATE.md` — memória de sessão/ciclos (runtime de agente).

## Decisão

Criar `index.md` curto, apontando para os documentos acima com uma linha de
descrição cada e declarando explicitamente o que é estado verificado e o que
é **[NV]**. Sem duplicar conteúdo: links relativos e contagens já verificadas.

## Alternativas descartadas

- Editar `README.md`: mudaria a apresentação do projeto, fora de escopo.
- Índice longo com resumos: duplicaria `PROGRESSO.md`/`research/index.md`.
- Incluir artefatos SDD no índice: excesso de detalhe para entrada raiz.
- `.generated.md`: exigiria revalidação de fonte específica.

## Cobertura idiomática

PT-BR (docs do alvo) + EN (nomes de arquivos). **[NV]** para estado remoto.
