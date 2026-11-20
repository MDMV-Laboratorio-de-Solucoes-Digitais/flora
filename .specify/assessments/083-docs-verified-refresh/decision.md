# Decision: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh · **Decidido**: 2026-11-19T23:12Z
- **Verdict**: **go (scoped)** · Artefatos revisados: intake | research | define | concept
- **80/20**: E=(1×5)/(1×1)=5,0 — valor 1 (refresh documental que impede citação de números mortos), confiança 5 (contagens por `find`, gate colado, citações caminho:linha nos dois arquivos), esforço 1 (dois blocos no EOF + 5 artefatos), risco 1 (nenhum código/CI/gate/PR tocado; append-only puro).

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | Últimas notas em `docs/PROGRESSO.md:159-164` e `docs/research/index.md:208-212` são 04:23:44Z; contagens do snapshot 23:11Z divergem. |
| Evidência | strong | `find` determinístico; citações `:3`, `:12-22`, `:151`, `:159-164`, `:3`, `:8-18`, `:186`, `:198-212`, `:208-212`; gate PASS 4 checks no snapshot. |
| Valor vs. inação | adequate | Evita decisões ancoradas em estado morto; não muda comportamento de produto. |
| Viabilidade | strong | Opção A: dois apêndices no EOF, sem dependência, DB, rede ou segredo. |
| Fit | strong | Disciplina append-only e 137/137 links (`:186`) preservadas; convenção dos ciclos 068/069/072. |
| Risco | strong | Nenhuma linha histórica tocada (FR-003/SC-003); remoto/CI/PR/produção marcados **[NV]**. |

## Veredito

`go (scoped)`: avançar com a opção A. Nenhum critério `weak`/`unknown`. Escopo = exatamente os apêndices no EOF dos dois arquivos + 5 artefatos 083. Toda afirmação remota/CI/PR/produção permanece **[NV]**.

## Handoff

Dentro: `docs/PROGRESSO.md` (após `:164`), `docs/research/index.md` (após `:212`) e os 5 artefatos `.specify/assessments/083-docs-verified-refresh/`. Fora: `:3`, `:12-22`, `:186`, `:159-164`/`:208-212` e todo histórico, código, CI, gates, scripts, workflows, ADRs, constituição, specs numeradas, PRs, produção, DB/rede, segredos e worktrees alheios. FR-001..FR-004 e SC-001..SC-004 de `define.md`.

Comandos determinísticos pós-edição (main agent):

1. `git diff --name-only` → esperado: somente `docs/PROGRESSO.md`, `docs/research/index.md` e os 5 artefatos 083.
2. `git diff --numstat` → esperado: zero deleções nos dois arquivos (`0` na coluna de remoções); apenas acréscimos.
3. `grep -n '2026-11-19T23:12Z' docs/PROGRESSO.md docs/research/index.md` → esperado: uma ocorrência nova em cada arquivo, no EOF.
4. Reexecução de `python3 scripts/ci/check_gate_integrity.py` → esperado: PASS, 4 checks; e recontagem `find` — **[NV]** nesta avaliação (sem gates/formatters/git aqui).

`[NV]` não verificados nesta avaliação: execução dos comandos acima, CI ao vivo, Forgejo/PRs, merge, proteção de branch, produção e aprovações. Gates/formatters **skipped** nesta avaliação somente leitura; base `33d174095` preservada.
