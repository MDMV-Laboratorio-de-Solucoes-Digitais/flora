# Intake: 050 — Rate limit paths

**Data:** 2026-11-17T16:09Z · **Âncora:** `github/dev@b3bfda505` · branch `feature/050-rate-limit-paths`

## Sinal

C25 (auditoria 012, `docs/research/2026-11-15-012-frontend-api-contract-audit.md:161`):
`flora-api/src/middleware/rate_limit.rs:159-170` compara `/auth/login`, `/auth/refresh`
e `/invites`, mas o router monta `/api/v1/auth/login`, `/api/v1/auth/refresh`
(`routes/mod.rs:29` + `routes/auth.rs:72,75`) e `/api/v1/users/invite` (`UserManager.svelte:18`).

## Teste vermelho (executado)

```
/api/v1/auth/login   -> GENERAL (BUG)
/api/v1/auth/refresh -> GENERAL (BUG)
/api/v1/users/invite -> GENERAL (BUG)
/health              -> GENERAL (correto)
```

Os limites específicos (5/60 login, 20/60 refresh, 10/3600 invite) nunca disparam;
tudo cai em `gen:{ip}` (100/min default). Tests em `rate_limit.rs:193-207` cobrem
só o contador, não o casamento de path.

## 80/20

`(I×C)/(E×R)` = (4×5)/(2×2) = **5.0** — à frente de N3 (4.0), C4 (3.75), C5 (2.08), C6 (0.80).

## Fronteiras

Escopo: `middleware/rate_limit.rs` + testes. Fora: rotas, C4/C5/C6, constituição,
workflows, ADRs, merge. Nenhum bug report em aberto em `.specify/bugs/`.
