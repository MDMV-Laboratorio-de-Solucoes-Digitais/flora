# Intake: 060 — Refresh verified counts

**Data:** 2026-11-18T03:49Z · **Âncora:** `github/dev@262ac042f` · branch `feature/060-refresh-verified-counts`

## Sinal

`docs/PROGRESSO.md:11-22` registra contagens verificadas em 02:23Z (após o
ciclo 056). Os ciclos 057–059 acrescentaram artefatos; `find` agora mostra
specs 50 dirs/224 arquivos (antes 47/202), assessments 47/192 (44/175),
validations 46/47 (42/43), releases 46/46 (42/42). `index.md` defere a
contagem a `PROGRESSO.md`, que ficou defasado como "estado verificado".

## Evidência (verificada por find)

- specs 50 dirs / 224 arquivos; assessments 47/192; validations 46/47;
  releases 46/46; bugs 4/12; research 135 (sem `index.md`); adr 6.
- Nota anterior datada 2026-11-18T02:23Z; não será reescrita (append-only).
- Gate autoritativo PASS (8 pisos/50 derivados de `FLOORS`).

## 80/20

`(I×C)/(E×R)` = (2×5)/(2×1) = **5.0**, maior candidato autônomo. ADR drift
(5.0) escalado **[NV]**; N3 `(4×3)/(3×4)=1.0` escalado por risco de migração
e ADR-0006 (SE); C5/C6 dependem de dono/SE.

## Fronteiras

Escopo: nova nota append-only em `docs/PROGRESSO.md`, artefatos do ciclo e
notas datadas. Fora: reescrever nota anterior, código, CI, ADRs, migrações.
Factory fonte `0cb34c4d…`, nada transplantado.
