# Shape: 057 — Dynamic floor total

## Forma escolhida

Substituir, em `scripts/ci/check_gate_integrity.py:120`, o literal
`"all eight db contract floors intact (50 total)"` por
`f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"`.

## Alternativas descartadas

- Constante global `TOTAL`: duplicaria a fonte de verdade.
- Remover a mensagem: perderia o sinal de integridade dos pisos.
- Tabela de numerais (`eight`, `nine`): complexidade sem benefício; o número
  8 no texto é apenas descritivo e pode virar `%d`.
- Editar ADRs: governança, aprovação pendente.

## Risco e reversão

Risco baixo: mudança apenas da string de sucesso; a lógica de falha e a regex
permanecem idênticas. Reversão: `git revert` do commit único. Nenhuma
credencial envolvida.

## Verificação

Guardião PASS (saída idêntica hoje), mutação em `/tmp` com 9º piso provando
que o texto se adapta, gates locais.
