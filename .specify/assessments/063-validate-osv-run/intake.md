# Intake: 063 — Validate OSV run

**Data:** 2026-11-18T04:46Z · **Âncora:** `github/dev@e1ff815bf` · branch `feature/063-validate-osv-run`

## Sinal

O ciclo 052 pinou o workflow reutilizável de OSV em `@v2.7.1`, mas a execução
real ficou **[NV]**. `gh api` (04:43Z) confirma `state=active`, 0 runs, e o
workflow declara `workflow_dispatch` — há, portanto, um caminho autorizado para
validar o pin sem esperar o cron de segunda-feira.

## Evidência (verificada)

- `gh api repos/.../actions/workflows/osv-scan.yml/runs?per_page=3` → `total_count: 0`.
- `gh api repos/.../actions/workflows/osv-scan.yml` → `state=active`, `id=420259367`.
- `.github/workflows/osv-scan.yml` → `on: schedule (cron '37 14 * * 1')` +
  `workflow_dispatch`; `uses: ...osv-scanner-reusable.yml@v2.7.1`;
  `fail-on-vuln: false`; `permissions: actions/contents read, security-events write`.
- Tabela canônica em `STATE.md:...` marca o [NV] do OSV.

## 80/20

`(I×C)/(E×R)` = (2×5)/(2×2) = **2.5**, único candidato autônomo restante.
ADR/N3/C5/C6 escalados; dívidas de fase não forjadas.

## Fronteiras

Escopo: disparar o workflow já declarado, observar o resultado e registrar
nota datada em `STATE.md`. Fora: código, ADRs, constituição, gate, C5/C6.
Factory fonte `b5bc7265…`, nada transplantado.
