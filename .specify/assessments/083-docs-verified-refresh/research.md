# Research: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh · **Coletado**: 2026-11-19T23:12Z
- **Entradas**: contexto compartilhado (snapshot determinístico 23:11Z) + releitura local dos dois arquivos de destino. Nenhuma fonte remota consultada; credenciais nunca copiadas. Estado remoto/CI/PR/produção: **[NV]**.

## Evidência local (caminho:linha)

| Ponto | Leitura |
|---|---|
| `docs/PROGRESSO.md:1,3` | Título "(estado verificado)"; cabeçalho `Atualizado: 2026-11-18T02:16Z · Âncora: github/dev@94fcf19ca` (ciclo 055) — histórico, preservar. |
| `docs/PROGRESSO.md:9` | Gate PASS, 4 required checks, 8 pisos / 50 total (data do snapshot) — reexecução pós-edição **[NV]**. |
| `docs/PROGRESSO.md:12-22` | Tabela "Contagens verificadas" 2026-11-18T02:23Z (47/202, 44/175, 42, 42, 4, 135, 6) — histórico. |
| `docs/PROGRESSO.md:141,151,159-164` | Notas append-only 02:10Z (068), 04:22:55Z (072) e a mais recente 04:23:44Z (072) em `:159-164`; fim do arquivo em `:164`. |
| `docs/research/index.md:3-6` | Cabeçalho "Estado verificado: 2026-11-19T01:58Z" (ciclo 068), 137/138 arquivos, regeneração por `find`, `[NV]` para remoto. |
| `docs/research/index.md:8-18` | Tabela de contagens 01:58Z (137, 6, 58/280, 55, 54, 54, 4) — histórico. |
| `docs/research/index.md:186-196` | Correção do ciclo 069: 137 arquivos / 137 links / `missing=[]` / `extra=[]` — dívida 047+ fechada por evidência, **não reabrir**. |
| `docs/research/index.md:198-206,208-212` | Notas 04:22:55Z e 04:23:44Z (ciclo 072); última linha do arquivo em `:212`. |

## Comandos determinísticos executados nesta avaliação

- `nl -ba docs/PROGRESSO.md | head -25` e `| tail -25` → `:3`, `:12-22`, `:151`, `:159-164` confirmados; EOF em `:164`, com newline final (`0a`).
- `nl -ba docs/research/index.md | head -22` e `| tail -25` → `:3-6`, `:8-18`, `:186`, `:198-212` confirmados; EOF em `:212`, com newline final (`0a`).
- `find … | wc -l` → releitura ao vivo: specs 73, assessments 69, validations 68, releases 68, bugs 4, research 138/137, adr 6. O +1 em specs é `specs/083-docs-verified-refresh/spec.md:1`, materializado às 23:11:17Z (informe do contexto: specs 72 no snapshot imediatamente anterior). É o ciclo em curso, não uma divergência a corrigir.

## Decisão

Acrescentar, somente no EOF dos dois arquivos, um apêndice datado (2026-11-19T23:12Z, ciclo 083) contendo: âncora local, contagens do snapshot, gate PASS com data/hora, escopo estrito, registro de que o histórico foi preservado e `[NV]` para remoto/CI/PR/produção. Nenhum cabeçalho, nota antiga, link ou número histórico é editado.

## Alternativas descartadas

- Reescrever `:3`/`:12-22`/`:8-18`: quebra append-only e apaga histórico (violaria FR-003/SC-003).
- Regenerar `index.md` por `find`: não há fonte `.generated.md` verificável; a checagem 137/137 em `:186` já fechou a dívida — reabri-la seria retrocesso.
- Tocar código/CI/gate/PR/produção: fora de escopo e exigiria verificação ao vivo **[NV]**; nenhum benefício para o problema documental.

## Cobertura idiomática

PT-BR (docs do alvo) + EN apenas em caminhos, nomes de comando e tokens `[NV]`/`PASS`. Toda afirmação não verificada localmente é marcada **[NV]**, não tratada como falsa nem como falha.
