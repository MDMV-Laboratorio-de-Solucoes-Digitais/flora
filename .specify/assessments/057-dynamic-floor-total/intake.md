# Intake: 057 — Dynamic floor total

**Data:** 2026-11-18T02:44Z · **Âncora:** `github/dev@97838cd9c` · branch `feature/057-dynamic-floor-total`

## Sinal

`scripts/ci/check_gate_integrity.py:120` imprime um texto literal com a quantidade
de pisos e o total: `"all eight db contract floors intact (50 total)"`. Esse
número foi editado manualmente em ciclos recentes (052: 43→49, 053/054: 49→50,
055: →50), enquanto `FLOORS` já contém os valores canônicos. É uma fonte
duplicada de verdade dentro do próprio guardião determinístico.

## Evidência (verificada)

- `sed -n '40,52p;115,123p' scripts/ci/check_gate_integrity.py` → `FLOORS` com
  8 entradas (soma 50); `:120` com literal "eight"/"50 total".
- Guardião PASS (8 pisos/50) — o literal está correto **hoje**, o defeito é
  latente: qualquer novo piso exigirá outra edição manual.
- ADR-0003:19 e ADR-0006:10,52 mantêm "43" antigo — dívida de governança,
  escalada **[NV]**, fora de escopo.

## 80/20

`(I×C)/(E×R)` = (3×5)/(2×1) = **7.5**, maior candidato autônomo. ADR drift
(5.0) exige aprovação de maintainer; root `index.md` 2.5; falso positivo local
1.875; N3 1.0; dívidas de fase 0.67. Registrado em `STATE.md` antes do BLOCO 6.

## Fronteiras

Escopo: 1 linha em `scripts/ci/check_gate_integrity.py`, artefatos do ciclo e
notas datadas. Fora: ADRs, constituição, workflow, testes, C5/C6, deploy.
Factory fonte `38f68266…`, nada transplantado.
