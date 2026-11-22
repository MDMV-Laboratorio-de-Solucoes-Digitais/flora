# ADR-0007 — Opt-in wildcard de recuperação: expiração e desligamento

**Data**: 2026-11-22 · **Status**: Aceito · **Expiração**: 2026-11-29T23:59:59Z

## Contexto

`FLORA_BOOTSTRAP__ADMIN_EMAIL=*` foi introduzido como recuperação temporária quando o e-mail real do administrador era desconhecido (ciclos 100/101). A correção D11 restringiu o seletor a papéis administrativos e o tornou one-shot com marcador persistente. Ainda assim, a variável continuará setada até operação explícita.

## Decisão

O opt-in é **temporário e expira em 2026-11-29T23:59:59Z**. Após essa data, a configuração deve ser removida da aplicação; o marcador one-shot impede novo uso mesmo antes disso. Nenhum segredo ou e-mail real é versionado.

## Quatro gates de reentrada

1. **Autenticação**: apenas papel built-in `Admin`/`Owner` em organização não-sistema é elegível.
2. **Consumo atômico**: `bootstrap_wildcard_state` com `pg_advisory_xact_lock` e `UPDATE ... WHERE consumed_at IS NULL RETURNING` — uma única sessão.
3. **Expiração temporal**: ADR com data limite; variáveis removidas e API reiniciada após confirmação do acesso.
4. **Auditoria/rotação**: login com e-mail real e senha rotacionada substitui a recuperação; nenhum log registra senha, hash ou e-mail do alvo.

## Consequências

Seed e produção sem a env permanecem com OIDC PKCE + senha local por e-mail exato. Restaurar backup anterior ao consumo pode reativar o marcador, por isso a remoção das variáveis é obrigatória na data de expiração. **[NV]**: execução operacional em staging/produção.

## Rollback

Remover as variáveis e reiniciar; nenhuma migration é desfeita (tabela aditiva).
