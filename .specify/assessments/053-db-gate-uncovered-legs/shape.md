# Shape: 053 — DB gate uncovered legs

## Forma escolhida

Acrescentar duas linhas `run_leg` dentro do único step `Run DB-backed contract
tests` de `rust.yml`, usando `contract::tasks::db::db_` (piso 4) e
`contract::workspace::db::db_` (piso 2). No guardião, adicionar `tasks: 4` e
`workspace: 2` a `FLOORS`, ajustar a regex para aceitar o segmento `db::` e
atualizar o texto de 6→8 pisos / 43→49 total.

## Alternativas descartadas

- Criar checks novos: mudaria `REQUIRED` e a proteção de branch — risco de travar PRs.
- Usar filtro curto: casa zero testes; o leg falharia por piso.
- Elevar todos os pisos: mudaria a semântica dos legs existentes sem evidência.
- Tocar `db_state()` para fail-closed: escopo V1, maior e independente.

## Risco e reversão

Risco baixo/médio: os dois legs ainda não foram executados no runner ( **[NV]** );
se algum teste for instável no Postgres 18 do CI, o leg ficará RED e será corrigido
na branch antes do merge. Reversão: `git revert` dos dois commits de implementação.
Nenhuma credencial envolvida.

## Verificação

`--list` (4/2), `check_gate_integrity.py` (8 pisos, 49 total), gates locais e CI ao vivo.
