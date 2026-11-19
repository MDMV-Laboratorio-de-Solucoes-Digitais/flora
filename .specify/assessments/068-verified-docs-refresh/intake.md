# Intake: 068 — refresh verificado de documentação

**Data:** 2026-11-19T01:56Z · **Âncora:** `dev@cbad031b5` (= `github/dev@cbad031b5`, verificado ao vivo) · branch `feature/068-verified-docs-refresh`

## Sinal

A documentação de estado do alvo registra contagens e intervalos defasados em relação ao filesystem. O agente de auditoria SDD (somente leitura) apontou:

- `docs/research/index.md:3,14-19` afirma `135` arquivos de research e uma tabela `202 / 175 / 43 / 42 / 12` — números copiados de `docs/PROGRESSO.md:17` no ciclo 056.
- `docs/PROGRESSO.md:88-90` afirma `53` validations e `53` releases; o `find` atual conta `54` de cada.
- `OBJETIVO.md:3,39` consolida até o ciclo 033; `ROADMAP.md:3` até 046; o filesystem chega ao ciclo 067.
- `flora-api/src/routes/rbac.rs:4-5` diz que os handlers "stay placeholders", embora o ciclo 066 tenha implementado CRUD real.

## Evidência determinística (coletada nesta sessão)

```
find specs -mindepth 1 -maxdepth 1 -type d | wc -l            -> 58
find specs -type f | wc -l                                    -> 280
find .specify/assessments -mindepth 1 -maxdepth 1 -type d | wc -l -> 55
find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l -> 54
find .specify/releases -mindepth 1 -maxdepth 1 -type d | wc -l    -> 54
find .specify/bugs -mindepth 1 -maxdepth 1 -type d | wc -l        -> 4
find docs/research -maxdepth 1 -type f -name '*.md' ! -name index.md | wc -l -> 137
find docs/adr -type f -name '*.md' | wc -l                        -> 6
python3 scripts/ci/check_gate_integrity.py --remote -> PASS (8 pisos / 51; dev/main required contexts OK)
```

## Autorização

Modo autônomo. Escolha registrada em `.specify/memory/STATE.md` antes do BLOCO 6 pelo critério `E = (impacto × confiança) ÷ (esforço × risco) = (2×5)/(2×1) = 5.00`. Nenhuma autorização adicional é necessária: o escopo é documental e a correção de um comentário, sem comportamento, gates, migrations ou produção. A emenda da constituição (que pontuaria 7.50) exige aprov maintainer e foi **escalada [NV]**, não executada.

## Fronteira

Quatro arquivos de documentação, um comentário em `flora-api/src/routes/rbac.rs`, artefatos do ciclo e `STATE.md`. Nenhum workflow, ADR, constituição, required check, migração, deploy, tag ou segredo.
