# Research: 057 — Dynamic floor total

**Data:** 2026-11-18T02:44Z · **Evidência local determinística**; remoto **[NV]**.

## Primária (executada)

1. `scripts/ci/check_gate_integrity.py:42-51` — `FLOORS` com 8 chaves:
   auth 13, messaging 4, websocket 8, search 8, notifications 5, files 6,
   tasks 4, workspace 2 → soma 50.
2. `:117-120` — cada piso é validado por regex contra `rust.yml`; se todos
   passam, `:120` acrescenta o texto literal `"all eight ... (50 total)"`.
3. `:114` — mensagem de sucesso genérica; nenhuma outra saída usa o total.
4. Histórico: o literal foi editado em 052 (43→49), 054 (49→50) e 055 (→50),
   sempre que `FLOORS` mudava — prova do drift recorrente.

## Decisão

Substituir o literal por uma f-string derivada de `FLOORS`:
`f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"`.
Nenhuma outra linha muda; a regex e as falhas permanecem idênticas.

## Alternativas descartadas

- Atualizar ADRs no mesmo PR: exige aprovação de maintainer; separar.
- Constante `TOTAL = 50`: continua sendo fonte duplicada.
- Remover a mensagem: perderia sinal útil do gate.
- Novo check: mudaria a proteção de branch (4 contextos).

## Cobertura idiomática

PT-BR (artefatos do alvo) + EN (código do guardião). **[NV]** para CI remota.
