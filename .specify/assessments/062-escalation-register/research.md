# Research: 062 — Escalation register

**Data:** 2026-11-18T04:27Z · **Evidência local + `gh api` ao vivo**; demais remotos **[NV]**.

## Primária (executada)

1. `gh api repos/.../actions/workflows/osv-scan.yml` → `{state:"active", id:420259367, created_at:"2026-11-16T11:04:49-04:00"}`.
2. `gh api repos/.../actions/workflows/osv-scan.yml/runs?per_page=10` → `total_count: 0`, `[]`.
3. `.github/workflows/osv-scan.yml` → `schedule: cron '37 14 * * 1'` (segunda-feira) + `workflow_dispatch`; pin `@v2.7.1` (ciclo 052).
4. `docs/adr/0003:19`, `docs/adr/0006:10,52` → "43"; gate → "50 total" derivado.
5. Verificador N3 (ciclo 060): órfãos **[NV]**, ADR-0006:27-31 exige SE; C5/C6 dependem de dono/SE.

## Decisão

Consolidar uma tabela append-only em `STATE.md` com: item, evidência, bloqueio,
ação necessária e status. Registrar o estado OSV verificado ao vivo — 0 runs é
esperado antes da primeira segunda-feira após a criação do workflow; o pin
permanece **[NV]** até uma execução real.

## Alternativas descartadas

- Executar ADR/N3/C5/C6: exigem aprovação; não autorizado.
- Forjar validate/release para dívidas de fase: falsificaria evidência.
- Disparar `workflow_dispatch` do OSV: efeito remoto desnecessário; cron é suficiente.

## Cobertura idiomática

PT-BR (artefatos) + EN (nomes de workflow/API). **[NV]** para demais estados remotos.
