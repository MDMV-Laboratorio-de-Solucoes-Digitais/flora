# Assess — decide: 115

**Data**: 2026-11-23

## Decisão

**Veredito: go**

Score 80/20: **400 / 625** (`impacto=5 × confiança=5 × (6−esforço=2)=4 × (6−risco=2)=4`), maior lacuna de usabilidade do núcleo Seed. Aproveita `FileService::list_files` e `PgFileRepository::find_by_workspace_id` já existentes, sem alterar schema nem required checks.

## Restrições

- Obrigatório preencher Briefing de Compreensão (`tenant`, `.specify/templates/briefing-compreensao.md`, ciclo 109).
- Atualização lockstep obrigatória de `FLOORS["files"] = 7` em `check_gate_integrity.py` e `.github/workflows/rust.yml` junto ao novo contrato DB.
- Passo `11b taskstoissues` obrigatório após `analyze` e antes de `implement`.
- Sem `.unwrap()`/`.expect()` em produção; SemVer `MINOR` (`0.7.16` → `0.8.0`) no `release.md` pela adição de novo endpoint público `GET /api/v1/files`.
