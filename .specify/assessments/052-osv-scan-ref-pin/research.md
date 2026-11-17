# Research: 052 — OSV scan ref pin

**Data:** 2026-11-17T23:38Z · **Evidência local + primária remota**; estado de CI ao vivo **[NV]**.

## Primária (executada)

1. Refs do action (fonte 1 — `git ls-remote`, 2026-11-17T23:32Z):
   - `refs/heads/v2` → vazio; `refs/tags/v2` → vazio.
   - `v2.7.1` → `32d3cd482bdadfb38b186c0ad1b031902d55d932` (tag anotada, peeled).
   - Falso positivo descartado: `tail` havia truncado a lista; reconsulta sem
     truncamento confirmou ausência de `v2`.
2. Documentação oficial (fonte 2 — `https://google.github.io/osv-scanner/github-action/`,
   consultada 2026-11-17T23:33Z): snippet de scan agendado usa
   `osv-scanner-reusable.yml@v2.7.1`.
3. Workflow no tag `v2.7.1` (fonte 3 — `raw.githubusercontent.com/google/osv-scanner-action/v2.7.1/.github/workflows/osv-scanner-reusable.yml`):
   `on.workflow_call.inputs` contém `scan-args` e `fail-on-vuln`; os dois são
   exatamente os parâmetros passados por `.github/workflows/osv-scan.yml:16-19`.
4. Impacto: `.github/workflows/osv-scan.yml` é semanal + `workflow_dispatch`,
   `fail-on-vuln: false`, fora dos 4 required checks (ADR-0003:23).

## Decisão

Trocar apenas `@v2` por `@v2.7.1`. Alternativas descartadas: SHA completo
(mais rígido, mas fugiria da convenção documentada pelo action e exigiria
processo de atualização); `@v2.6.0` (existe, porém a documentação oficial
recomenda `v2.7.1`); remover o job (perderia sinal advisory de supply-chain).

## Cobertura idiomática

EN (docs Google + YAML do action) + PT-BR (artefatos do alvo). Nenhuma afirmação
sobre execução real do job remoto é feita — **[NV]**.
