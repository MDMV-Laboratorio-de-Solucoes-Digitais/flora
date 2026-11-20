# Research: 063 — Validate OSV run

**Data:** 2026-11-18T04:46Z · **Evidência local + `gh api` ao vivo**; resultado da execução pendente **[NV]**.

## Primária (executada)

1. `gh api .../actions/workflows/osv-scan.yml/runs?per_page=3` → `total_count: 0` (04:43Z).
2. `gh api .../actions/workflows/osv-scan.yml` → `state=active`, `id=420259367`.
3. Workflow: `workflow_dispatch` + `schedule`; pin `@v2.7.1`; `fail-on-vuln: false`; sem segredos lidos.
4. `gh workflow run osv-scan.yml --repo ...` é o comando de disparo (gatilho já declarado).

## Decisão

Disparar uma execução manual, acompanhar até conclusão e registrar em
`STATE.md` o `id`, `status`, `conclusion`, timestamp e o que isso prova sobre
o pin. Se falhar, o resultado vira candidato de correção em ciclo separado —
não será escondido.

## Alternativas descartadas

- Esperar o cron de segunda: deixaria o [NV] aberto por dias.
- Alterar o workflow: não há evidência de defeito; o pin foi validado estaticamente em 052.
- `gh api` direto para criar run: `gh workflow run` é a interface declarada.

## Cobertura idiomática

PT-BR (artefatos) + EN (nomes de workflow/comandos).
