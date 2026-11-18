# Shape: 054 — DB env fail-closed

## Forma escolhida

Duas linhas de expansão fatal `${VAR:?…}` dentro de `run_leg` em `rust.yml`,
mais um bloco condicional em `check_gate_integrity.py` que exige as duas chaves
de env no job `db-tests`. Nenhum teste é tocado.

## Alternativas descartadas

- `anyhow::bail!` nos 9 helpers: muda semântica local mas exige 8 arquivos de
  teste e deixa código inalcançável; diff maior sem ganho no gate de merge.
- Checar env apenas no guardião: não protege a execução real do step.
- Checar env apenas no runtime: não protege contra remoção futura da linha de env.
- Novo required check: reconfiguraria a proteção de branch.

## Risco e reversão

Risco baixo: a expansão `:?` é semantics POSIX/bash padrão e `set -euo pipefail`
já está ativo. Efeito colateral esperado: se um leg futuro não usar DB, ainda
assim exigirá env — aceitável porque o job `db-tests` é inteiramente DB-backed.
Reversão: `git revert` dos dois commits de implementação. Nenhuma credencial
envolvida (as URLs são do serviço Postgres efêmero do CI).

## Verificação

Executar o `run:` do step com env removida (deve falhar antes do cargo) e com
DB real isolada (8 legs verdes); guardião PASS + teste de mutação temporário.
