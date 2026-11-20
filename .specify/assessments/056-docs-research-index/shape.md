# Shape: 056 — Docs research index

## Forma escolhida

Dois arquivos novos. `docs/research/index.md`: cabeçalho com contagem verificada
e metodologia, seguido de seções `### YYYY-MM-DD` com entradas
`- [título](arquivo.md)`. Gerado por script Python determinístico que lê os
nomes dos arquivos e o primeiro cabeçalho `# ` de cada um (fallback: nome do
arquivo). `docs/PROGRESSO.md`: nota datada curta, no padrão append-only já
usado em `OBJETIVO.md`/`ROADMAP.md`.

## Alternativas descartadas

- Escrita manual das 135 entradas: risco de omissão e drift.
- Índice com resumos longos: duplicaria o conteúdo dos documentos.
- Incluir ADR/agents: escopo diferente da contagem de research.
- `.generated.md`: exigiria revalidação de fonte específica.

## Risco e reversão

Risco baixo: arquivos novos, nenhum código/gate afetado; único risco é drift
futuro do índice se novos documentos forem adicionados — declarado no próprio
índice com o comando de regeneração. Reversão: `git revert` dos commits.

## Verificação

Comparar contagem de entradas com `find`, validar links, subagent verificador
isolado e gates determinísticos inalterados.
