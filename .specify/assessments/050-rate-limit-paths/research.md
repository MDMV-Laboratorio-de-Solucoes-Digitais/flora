# Research: 050 — Rate limit paths

**Data:** 2026-11-17T16:09Z · **Evidência local**; remoto **[NV]**.

## Primária (executada)

- `flora-api/src/middleware/rate_limit.rs:155` — `let path = req.uri().path();`
- `:159` — `path.starts_with("/auth/login")`; `:161` — `"/auth/refresh"`;
  `:167` — `path.contains("/invites") || path.contains("/invitations")`.
- `flora-api/src/routes/mod.rs:29` — `.nest("/api/v1/auth", auth::create_auth_router())`;
  `:25-27` — router montado sob `/api/v1`.
- `flora-api/src/routes/auth.rs:72` — `/login` (GET); `:75` — `/refresh` (POST).
- `apps/frontend/src/lib/components/users/UserManager.svelte:18` — `POST /api/v1/users/invite`.
- Consequência: limites específicos nunca casam; `gen:{ip}` (100/min) domina.
- `rate_limit.rs:193-207` — 2 testes cobrem apenas `check_rate_limit`, sem path matching.

## Correção planejada

Extrair classificador puro `classify_endpoint(path: &str) -> RateLimitClass` com
`ends_with` nos sufixos reais (`/auth/login`, `/auth/refresh`, `/users/invite`,
`/invites`, `/invitations`), cobrindo tanto o prefixo `/api/v1` quanto o legado.
Middleware passa a usar o classificador. Testes unitários cobrem os 4 casos.

## NV

Comportamento do middleware em produção e state distribuído do limitador: **[NV]**.
