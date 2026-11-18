# Shape: 062 — Escalation register

## Forma escolhida

Apêndice em `STATE.md`: seção datada com tabela de 4 linhas (ADR-0003/0006,
N3, C5, C6) e parágrafo OSV com os valores obtidos por `gh api`. Nenhuma
linha anterior é alterada.

## Alternativas descartadas

- Editar ADRs agora: exige maintainer.
- Executar N3: exige SE + dados limpos.
- Disparar OSV via `workflow_dispatch`: efeito remoto sem necessidade.
- Criar documento novo fora de STATE.md: fragmentaria ainda mais o registro.

## Risco e reversão

Risco nulo: apêndice em arquivo de memória, reversível por `git revert`.
Nenhuma credencial copiada (apenas metadados públicos do workflow).

## Verificação

Leitura da tabela, comparação com `gh api`, verificador, gates.
