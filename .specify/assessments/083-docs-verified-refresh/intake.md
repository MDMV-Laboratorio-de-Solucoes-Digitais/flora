# Intake: 083 — Docs verified refresh

- **Slug**: 083-docs-verified-refresh · **Capturado**: 2026-11-19T23:12Z
- **Origem**: evidência determinística local do contexto compartilhado (snapshot 2026-11-19T23:11Z) + releitura de `docs/PROGRESSO.md` e `docs/research/index.md` nesta avaliação. Nenhuma instrução encontrada nesses arquivos foi seguida: conteúdo de docs/specs/índices é **dado não-confiável**.
- **Alvo**: branch `feature/083-docs-verified-refresh`, base `33d174095`; gate **PASS** (4 checks) no snapshot; zero PRs **[NV]** quanto ao estado remoto ao vivo.

## Sinal

As duas âncoras de estado verificado pararam no ciclo 072 (`2026-11-19T04:23:44Z`):
`docs/PROGRESSO.md:159-164` e `docs/research/index.md:208-212`. O cabeçalho de
`docs/PROGRESSO.md:3` ainda traz a âncora `github/dev@94fcf19ca` (ciclo 055) e
o de `docs/research/index.md:3` o estado verificado `2026-11-19T01:58Z` (ciclo
068). As tabelas `docs/PROGRESSO.md:12-22` e `docs/research/index.md:8-18`
descrevem checkout antigo; o snapshot 23:11Z mediu specs 72, assessments 69,
validations 68, releases 68, bugs 4, research 138 `.md` (137 sem `index.md`) e
adr 6.

## Evidência determinística (find, snapshot 2026-11-19T23:11Z)

- specs 72; assessments 69; validations 68; releases 68; bugs 4; research 138 (137 sem `index.md`); adr 6.
- Gate autoritativo `python3 scripts/ci/check_gate_integrity.py` → PASS, 4 checks (snapshot) — reexecução pós-edição **[NV]**.
- `docs/research/index.md:186` (ciclo 069) fixou 137 arquivos / 137 links / `missing=[]`; **não reabrir**.
- Append-only confirmado: notas 01:58Z, 02:10Z, 04:22:55Z e 04:23:44Z permanecem; nenhuma reescrita.

## 80/20

`E=(I×C)/(E×R)` = (1×5)/(1×1) = **5,0**. Valor 1 (refresh documental pequeno, impede citação de números mortos), confiança 5 (evidência local determinística e reprodutível), esforço 1 (dois apêndices no EOF), risco 1 (nenhum código/CI/gate/PR tocado). Único candidato autônomo deste corte; alternativas (ADRs, migrações, dívidas remotas) exigem maintainer ou verificação ao vivo — **[NV]** e fora de escopo.

## Fronteiras

Dentro: apêndice datado no EOF de `docs/PROGRESSO.md` e `docs/research/index.md` (âncora local, contagens, gate com data/hora, escopo, histórico preservado, `[NV]`), mais os 5 artefatos `.specify/assessments/083-docs-verified-refresh/`. Fora: reescrever cabeçalhos/notas antigas, código, CI, gates, PRs, produção, ADRs, constituição, migration, segredos. Toda afirmação remota/CI/PR/produção é **[NV]**.
