# Intake: 071 — remover stub morto AuthService

**Data:** 2026-11-19T03:49Z · **Âncora:** `dev@cdf50eaff` (= `github/dev`) · branch `feature/071-dead-auth-service`

## Sinal

`flora-core/src/services/auth_service.rs` é um stub com 5 métodos que retornam `ServiceUnavailable("... not implemented")` e `is_available() == false`. Está wiring em `AppState` e em 7 sites de teste, mas nenhum handler o utiliza — a autenticação real está em `routes/auth.rs` + extractor de sessão. É o último resíduo do tipo removido no ciclo 061 (circuit-breaker morto), alinhado ao Princípio I (debloat).

## Evidência

```
grep -rn 'AuthService\|auth_service' --include='*.rs' -> 25 ocorrências em 11 arquivos
  flora-core/src/services/auth_service.rs   (definição + impl AuthProvider)
  flora-core/src/services/mod.rs            (mod + pub use)
  flora-api/src/state/app_state.rs          (campo, init, struct literal)
  flora-api/src/extractors/auth.rs:278      (teste)
  flora-tests/src/contract/{auth,files,messaging,notifications,search,websocket}.rs
grep -rn 'auth_service' flora-api/src/routes/ -> 0 usos
Error::ServiceUnavailable ainda é usado por oidc_transactions.rs -> manter variante
```

## Autorização

Modo autônomo; decisão 80/20 `(2×5)/(2×2)=2.50` em `STATE.md` antes do BLOCO 6. Sem autorização adicional: remoção de código morto, verificável por compilação/gates.

## Fronteira

11 arquivos Rust + artefatos do ciclo + `STATE.md`. Nenhum workflow, gate, migration, ADR ou constituição.
