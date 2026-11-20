# Shape: 055 — Auth floor thirteen

## Forma escolhida

Três edições literais: `12` → `13` em `rust.yml:107` e em
`check_gate_integrity.py:43`; `49 total` → `50 total` em `:120`. Nada mais.

## Alternativas descartadas

- Piso 14: folga fictícia; nenhum 14º teste existe.
- Ajustar filtro: já casca os 13; sem ganho.
- Atualizar ADR junto: exige aprovação de maintainer; separar do gate.
- Novo check: mudaria proteção de branch (4 contextos).

## Risco e reversão

Risco baixo: fortalecimento de gate; os 13 testes passaram com Postgres real
no ciclo 053 e foram confirmados por `--list` agora. Reversão: `git revert` dos
dois commits. Nenhuma credencial envolvida.

## Verificação

Guardião PASS, `--list` = 13, leg auth com DB real isolado (13 passed),
mutação temporária confirmando que o piso 13 é exigido, gates locais.
