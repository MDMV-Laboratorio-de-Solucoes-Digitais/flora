# Assess — decide: 117

**Data**: 2026-11-23

## Decisão

**Veredito: go**

Score 80/20: **240 / 625** (`impacto=4 × confiança=5 × (6−esforço=3)=3 × (6−risco=2)=4`), alinhado às issues `#391`..`#395` e ao backlog priorizado em `STATE.md`.

## Restrições

- Preencher Briefing de Compreensão (`tenant`, `.specify/templates/briefing-compreensao.md`, ciclo 109) comprovando isolamento cross-organization de `/api/v1/notifications` e proteção contra enumeração (`404` fail-closed).
- Atualização lockstep obrigatória de `FLOORS["notifications"] = 6` em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` (total `80`).
- Reutilizar as issues R6 existentes `#391`..`#395` (`11b taskstoissues`), sem duplicar issues nem adicionar `dark-factory:eligible`.
- Sem `.unwrap()`/`.expect()` em produção; SemVer `MINOR` (`0.9.0 → 0.10.0`).
