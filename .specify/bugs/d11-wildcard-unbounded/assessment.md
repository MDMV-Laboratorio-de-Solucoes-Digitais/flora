# Assessment: D11

- **Causa raiz**: o seletor `*` é apenas uma variável de config; não há vínculo com papel nem estado de uso.
- **Risco**: escalada de privilégio (qualquer usuário ativo) + janela indefinida de recuperação.
- **Correção**: helper SQL único com `JOIN memberships/roles` filtrando `is_builtin AND name IN ('Admin','Owner')`; tabela singleton `bootstrap_wildcard_state` com `UPDATE ... WHERE consumed_at IS NULL RETURNING` sob `pg_advisory_xact_lock`; leitura no startup e consumo no login na mesma transação da sessão.
- **Alternativas rejeitadas**: flag em memória (não sobrevive restart), flag em `organizations.settings` (sem CAS atômico), flag em `users` (backfill caro).
- **Gate**: nenhum `#[ignore]` novo; adicionar testes unitários puros e estender testes DB existentes sem mudar contagens (bootstrap 13, auth 23).
