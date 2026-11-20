# Intake: 058 — Root doc index

**Data:** 2026-11-18T03:03Z · **Âncora:** `github/dev@58921a337` · branch `feature/058-root-doc-index`

## Sinal

`index.md` na raiz não existe (`test -f` → ausente), embora o Bloco 4 da
orquestração o liste junto de `OBJETIVO.md`, `ROADMAP.md` e `PROGRESSO.md`.
Os pontos de entrada agora existem (`docs/PROGRESSO.md` e
`docs/research/index.md`, ciclo 056), mas não há um índice único que os
conecte. `README.md` descreve o projeto, não o estado verificado.

## Evidência (verificada)

- `ls -1 *.md` → `AGENTS.md`, `guide-to-strict-rust.md` (gitignored),
  `OBJETIVO.md`, `README.md`, `ROADMAP.md`; **sem `index.md`**.
- `docs/PROGRESSO.md`, `docs/research/index.md` presentes (ciclo 056).
- Contagens `find`: specs 48 dirs/210 arquivos; assessments 45/180;
  validations 44/45; releases 44/44; research 136 (135 + índice); adr 6.
- Gate autoritativo PASS (8 pisos/50, total derivado de `FLOORS`).

## 80/20

`(I×C)/(E×R)` = (1×5)/(2×1) = **2.5**, maior candidato autônomo. ADR drift
(5.0) escalado **[NV]** por aprovação de maintainer; falso positivo local
1.875; N3 1.0; dívidas de fase 0.67. Registrado em `STATE.md`.

## Fronteiras

Escopo: 1 arquivo novo (`index.md`), artefatos do ciclo e notas datadas.
Fora: README, ADRs, constituição, código, workflows, C5/C6, deploy.
Factory fonte `38f68266…`, nada transplantado.
