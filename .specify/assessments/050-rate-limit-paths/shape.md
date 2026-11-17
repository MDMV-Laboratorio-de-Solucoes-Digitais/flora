# Shape: 050 — Rate limit paths

## Forma escolhida

Extrair um `enum RateLimitClass { Login, Refresh, Invite, General }` e uma função
pura `classify_endpoint(&str) -> RateLimitClass` em `middleware/rate_limit.rs`.
O middleware continua calculando `(key, limit, window)` a partir da classe, com
os mesmos números de antes. Testes unitários cobrem paths reais e legados.

## Alternativas descartadas

- Só trocar os literais inline: sem teste isolado, fácil regressão futura.
- Casar por regex: dependência ou complexidade desnecessária para 3 sufixos.
- Mover o middleware para dentro das rotas: mudança arquitetural fora de escopo.

## Risco e reversão

Risco baixo: nenhuma mudança de limites/janelas/resposta, apenas de classificação.
Reversão: `git revert` do commit do ciclo.

## Verificação

`cargo test -p flora-api`, gates completos e `check_gate_integrity.py`.
