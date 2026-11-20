# Shape: 063 — Validate OSV run

## Forma escolhida

`gh workflow run osv-scan.yml --repo ...` seguido de consulta ao run via
`gh run list --workflow osv-scan.yml --limit 1` / `gh api`, com registro
append-only em `STATE.md`. Nenhuma alteração de arquivo de código.

## Alternativas descartadas

- Esperar cron: [NV] por dias.
- Editar workflow: sem evidência de defeito.
- Múltiplos disparos: um é suficiente; evita ruído em security-events.

## Risco e reversão

Risco controlado: a execução faz upload de SARIF (`security-events: write`),
comportamento normal do workflow; nenhuma credencial é copiada. Não há
"reversão" de um run — o registro é histórico. Reversão do apêndice: `git revert`.

## Verificação

`id`/`status`/`conclusion` via `gh`, gates locais, verificador.
