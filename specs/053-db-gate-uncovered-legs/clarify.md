# Clarify: 053 — DB gate uncovered legs

**Data:** 2026-11-18T00:04Z

## Resoluções

1. **Por que o filtro curto falha?** Os testes vivem em `mod db`; `--list`
   comprovou 0 casamentos para `contract::tasks::db_` e 4 para `db::db_`.
2. **Novo check ou mesmo job?** Mesmo job `db-tests`; novos checks exigiriam
   reconfigurar proteção de branch (risco de deadlock em PRs).
3. **Qual piso?** Exatamente o número de testes descobertos: 4 e 2. Sem folga
   artificial; os pisos existentes não são reduzidos.
4. **Corrigir o piso 12 de auth?** Não: reforço independente, fora deste escopo.
5. **Corrigir fail-open da env?** Não: candidato V1, separado (esforço/risco maiores).
6. **Identidade 053?** `feature.json` aponta 051; `--dry-run` → `053` por inventário.

## Pendências

Nenhum `NEEDS CLARIFICATION`. Resultado remoto dos legs: **[NV]**.
