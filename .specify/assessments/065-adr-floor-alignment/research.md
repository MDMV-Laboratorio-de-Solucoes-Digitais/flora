# Research: 065 — ADR floor alignment

**Data:** 2026-11-18T13:05Z · **Evidência local determinística**; remoto **[NV]**.

## Primária

1. `docs/adr/0003:19` — `piso (12/4/8/8/5/6 — total 43)`.
2. `docs/adr/0006:10` — `db-tests com piso 43`; `:52` — `(piso 43)`.
3. `scripts/ci/check_gate_integrity.py:42-51` — FLOORS: auth 13, messaging 4,
   websocket 8, search 8, notifications 5, files 6, tasks 4, workspace 3 → 51.
4. Guardião imprime o total dinamicamente (`len(FLOORS)`/`sum(FLOORS.values())`).

## Decisão

Substituir as três citações por `13/4/8/8/5/6/4/2 — total 51`
(0003:19) e `piso 51` (0006:10,52), preservando o texto restante e a
numeração/versão dos ADRs. A Constituição §Governance exige registro da
autorização — feita pelo SE e anotada em `STATE.md`.

## Alternativas descartadas

- Manter 43: documentação contradiz gate determinístico.
- Editar a constituição: fora da autorização.
- Atualizar `STATE.md` histórico: append-only, não reescrito.

## Cobertura idiomática

PT-BR (ADRs/artefatos) + EN (nomes de checks). **[NV]** para remoto.
