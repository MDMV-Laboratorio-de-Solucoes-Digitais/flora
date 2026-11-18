# Clarify: 067 — C6 canário RLS

**Data:** 2026-11-18T15:52Z · **Modo:** READ-ONLY (nenhuma fonte alterada)

## Perguntas respondidas com evidência local

1. **"RLS já não está ativa?"** — Não. `ENABLE ROW LEVEL SECURITY` está presente em
   11 tabelas, mas não há `CREATE POLICY`, `FORCE`, GUC, papéis de DB ou transação
   por request. A app conecta como owner; RLS é inerte. Fonte: migração init
   `:196-215` + auditoria por grep + ADR-0005.
2. **"O canário pode incluir `users`/`organizations`?"** — Não nesta fase. O
   carregamento da sessão por `jti` e o login precisam de lookup anterior ao GUC;
   incluí-los faria o fail-closed bloquear a própria autenticação (ressalva
   `seed-13` registrada no ADR-0005).
3. **"Uma flag `RLS_ENABLED` pode condicionar os required checks?"** — Não.
   GitHub conta `skipped` como sucesso; ADR-0003/0006 proíbem `if:` e
   `continue-on-error` em required check.
4. **"Rollback apaga dados?"** — Não. Consiste em remover/desativar policies e
   `FORCE`, restaurar grants e voltar ao filtro da app; nenhuma DML destrutiva.
5. **"Este ciclo substitui um ADR?"** — Não. ADR-0005 (desenho) e ADR-0006
   (rollout) permanecem como autoridade; o documento é um desdobramento operacional.

## Premissas registradas

- O canário roda **somente** em DB descartável; produção é **[NV]** e exige SE.
- A prova em PG18 citada no ADR-0005 foi executada fora deste checkout — **[NV]**
  localmente; o documento a trata como evidência de projeto, não como execução nossa.
- A autorização SE para "documento, sem produção" já existe; não foi solicitada
  autorização nova neste ciclo.

## Resultado

**PASS — sem ambiguidades que bloqueiem a especificação.**
