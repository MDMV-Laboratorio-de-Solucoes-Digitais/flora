# Bug Fix — OIDC redirect_uri sem fallback local

**Bug**: `oidc-redirect-uri-not-allowed` · **Data do fechamento**: 2026-11-22

## Correção verificada

O fallback local de senha foi implementado nos ciclos 096–101:

- `flora-api/src/routes/auth.rs:83` registra `POST /auth/password/login`.
- `flora-api/src/routes/auth.rs:443-447` implementa `password_login` para ingresso sem IdP.
- `flora-api/src/routes/mod.rs:31` documenta a rota no mesmo router.
- Dicionário e UI pt-BR atualizados nos ciclos 096/099.

Nenhuma credencial, webhook ou configuração de IdP foi alterada neste fechamento documental.

## [NV]

Disponibilidade de IdP externo e FQDN em produção; não observados ao vivo.
