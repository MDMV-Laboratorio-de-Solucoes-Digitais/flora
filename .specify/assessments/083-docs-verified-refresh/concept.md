# Shape: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh · **Recomendado**: Opção A (apêndice datado no EOF dos dois arquivos) · **Data**: 2026-11-19T23:12Z

## Opção A — apêndice datado, append-only (recomendada)

- **Sketch**: ao final de `docs/PROGRESSO.md` (após `:164`) e de `docs/research/index.md` (após `:212`), bloco curto `## Estado verificado — 2026-11-19T23:12Z (ciclo 083; append-only)` com âncora local, contagens do snapshot 23:11Z (72/69/68/68/4, research 137 sem `index.md` / 138 com ele, adr 6), gate PASS 4 checks com data/hora, escopo restrito aos dois arquivos, registro de que `:3`, `:12-22`, `:186`, `:159-164`/`:208-212` permanecem e `[NV]` para remoto/CI/PR/produção. A redação final do apêndice é do dono do texto (DocContract/main); este shape fixa só os elementos obrigatórios.
- **Apetite**: trivial — dois blocos no EOF, nenhum código.
- **Trade-offs**: corrige a âncora para leitores futuros sem risco de sobrescrever histórico; mantém intactas as correções 02:10Z (`:141`) e 02:35Z/137-137 (`:186`). Não "conserta" cifras antigas — intencional.
- **Rabbit holes**: editar `:3` (trocar `94fcf19ca`), reescrever tabelas `:12-22`/`:8-18`, regenerar `index.md`, reabrir dívida 047+, converter `[NV]` em fato, tocar código/CI/gate/PR.

## Opção B — não fazer nada

- **Sketch**: manter as notas de 04:23:44Z como últimas âncoras.
- **Trade-offs**: custo zero, mas preserva a desinformação; falha SC-001. Rejeitada.

## Opção C — reescrever/regenerar os documentos

- **Sketch**: consolidar uma única nota "estado atual" substituindo cabeçalhos e tabelas antigos, possivelmente com geração automática.
- **Trade-offs**: leitura mais limpa, porém destrói append-only, exige fonte regenerável inexistente e viola FR-003/SC-003. Rejeitada.

## Fora de escopo

Código, config, CI, gates, pisos, scripts, workflows, ADRs, constituição, specs numeradas (inclusive `specs/083-docs-verified-refresh/spec.md:1`), PRs, produção, DB/rede, segredos, worktrees alheios e toda afirmação remota **[NV]**.

## Premissas

- Contagens do snapshot 23:11Z são a fonte da verdade para o apêndice; o diretório `specs/083-docs-verified-refresh/` foi materializado às 23:11:17Z e explica o +1 em `find` ao vivo — não é divergência a corrigir.
- `docs/PROGRESSO.md:164` e `docs/research/index.md:212` são os EFIs atuais, ambos com newline final (`0a`): o apêndice é acréscimo puro.
- FR-001..FR-004 e SC-001..SC-004 de `define.md` regem a implementação; nenhum `[NV]` é inferido como verificado.
