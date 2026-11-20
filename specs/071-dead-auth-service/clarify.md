# Clarify: 071 — remover stub morto AuthService

**Data:** 2026-11-19 · **R3:** máx. 3 perguntas, uma por vez.

## Esclarecimentos necessários

**Nenhum.** A fonte é determinística (`grep` + compilação) e o escopo foi fixado por `go (scoped)`.

## Decisões derivadas sem perguntar

1. **Manter `AuthProvider`/`UserInfo`**: são pontos de extensão documentados e exportados; ficar sem implementadores não causa lint e preserva a arquitetura trait-driven.
2. **Manter `Error::ServiceUnavailable`**: usado por `oidc_transactions.rs:85,119`.
3. **Não substituir o stub**: nenhum handler consome `auth_service`; criar um adaptador real seria feature nova, fora de escopo.
4. **Um commit por arquivo**: convenção da constituição §VI; a exclusão do arquivo conta como um commit.
