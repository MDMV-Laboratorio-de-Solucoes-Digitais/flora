# Intake: 065 — ADR floor alignment

**Data:** 2026-11-18T13:05Z · **Âncora:** `github/dev@aef13277a` · branch `feature/065-adr-floor-alignment`

## Sinal

`docs/adr/0003:19` e `docs/adr/0006:10,52` descrevem os pisos do `db-tests`
como `12/4/8/8/5/6 — total 43`. O gate real (ciclo 064) imprime
`all 8 db contract floors intact (51 total)` — `FLOORS` com
auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6,
tasks 4, workspace 3.

## Autorização SE

O SE autorizou o patch documental citando "total 50" (valor vigente antes do
ciclo 064). A cifra verificada agora é **51**; registrei a divergência em
`STATE.md` e alinho ao valor real do gate.

## Evidência

- `python3 scripts/ci/check_gate_integrity.py` → `8 pisos / 51 total`.
- `FLOORS` em `check_gate_integrity.py:42-51`; `rust.yml` legs 105-114.
- Nenhuma afirmação remota: **[NV]**.

## 80/20

`(I×C)/(E×R)` = (2×5)/(1×1) = **10.0** — maior candidato autorizado.
C6 canário 3.33, C5 2.5, enfileirados.

## Fronteiras

Escopo: 2 ADRs + artefatos do ciclo. Fora: constituição, código, CI, C5/C6.
Factory `c81f0f81…`, nada transplantado.
