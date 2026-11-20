# Research: 068 — refresh verificado de documentação

**Data:** 2026-11-19T01:56Z · **Evidência local verificada por `find`/`sed`/`gh`** · remoto futuro **[NV]**.

## Primária (local)

1. `docs/research/index.md:3` — `> **Contagem:** 135 arquivos.` A própria linha 4 define a fonte: `find docs/research -maxdepth 1 -type f -name '*.md' ! -name index.md`. Executada hoje: **137**.
2. `docs/research/index.md:11-19` — tabela `docs/research/ 135`, `docs/adr/ 6`, `specs/ 202`, `assessments 175`, `validations 43`, `releases 42`, `bugs 12`. Contagens reais com a mesma semântica: `137 / 6 / 58 dirs (280 arquivos) / 55 dirs / 54 / 54 / 4`.
3. `docs/PROGRESSO.md:88-90` — `validations 53`, `releases 53`; `find` atual: **54 / 54**. `docs/PROGRESSO.md:16-22` registra `47 dirs / 202 arquivos` e `research 135`, também superado.
4. `OBJETIVO.md:3,39` — `specs 001-033`, `assessments 003-033`; `ROADMAP.md:3` — `ciclos 011–046`. `find specs ... | sort | tail -1` → `067-c6-canary-rls-design`.
5. `flora-api/src/routes/rbac.rs:4-5` — `the remaining handlers stay placeholders until their semantics are decided by the owner`; contradito por `rbac.rs:180-198,230-264,337-367` (CRUD real, ciclo 066) e por `.specify/memory/STATE.md:363-369`.
6. `scripts/ci/check_gate_integrity.py:42-51` — FLOORS `13/4/8/8/5/6/4/3` = 51; gate executado localmente e com `--remote`: PASS, `dev`/`main` com os 4 contexts required, strict, enforce_admins, sem required reviews.

## Secundária (remoto, observado ao vivo nesta sessão)

- `gh repo view` → `MDMV-Laboratorio-de-Solucoes-Digitais/flora`, default branch `dev`.
- `git fetch github dev` → `HEAD` local e `github/dev` ambos `cbad031b5`, `0 0` divergência.
- `gh pr list --state open` → nenhum PR aberto.
- `gh run list --branch dev --limit 5` → 4 execuções recentes `completed/success` (PRs #284–#286).
- **[NV]** CI futura, runner semantics, Forgejo, produção e aprovações.

## Triangulação

Não há afirmação carregada dependente de fonte externa. Todas as cifras vêm de comando determinístico executado no checkout; a única afirmação remota foi confirmada por `gh` e está datada. Cobertura idiomática: pt-BR (docs e STATE); nenhuma fonte multilíngue necessária para contagens locais.
