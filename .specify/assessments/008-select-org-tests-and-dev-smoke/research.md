# Research: 008 — Select-Organization Tests & Reproducible Dev Smoke

**Date:** 2026-11-15  
**Inputs:** codebase audit + subagent audits `ses_e1e9a5437ffeDtndGc5NhfHNUB`, `ses_e1e9a5435ffexYwl8z3bRWNTW3`, `ses_e1e9a542bffe1iScHCp3nDbNe7`, `ses_e1e9a5428ffesJ4ikPt7V732Nm`, `ses_e1e9a5426ffeFHRHkBZGVF8s6o`, `ses_e1ea41e08ffeHj1Q40A2fgILJO`.

## Codebase/test reality
- `POST /api/v1/auth/select-organization` (`flora-api/src/routes/auth.rs:325-389`): rejeita `organization_id` nil (`422` via `Error::Validation`), exige `AuthExtractor`, valida sessão por `jti` (ativa, mesmo usuário), valida membership, revoga sessão atual, cria nova sessão org-scoped com `jti` rotacionado e retorna JWT.
- `flora-tests/src/contract/auth.rs`: testes in-process com `tower::ServiceExt::oneshot`, `PgPool::connect_lazy(...)`, `Config::default()`, sem migrações/harness DB compartilhado. Casos que não tocam DB (decode/extractor curto-circuito, validação antes de acesso) são viáveis em CI.
- CI (`.github/workflows/rust.yml`): `cargo build` + `cargo test` sem Postgres/Valkey/Meilisearch → testes que exigem DB real devem ser gated/ignorados/condicionados.

## Bootstrap/smoke feasibility (sem OIDC real)
- `dev-seed.sql` provê org `11111111-1111-1111-1111-111111111111`, usuários (Alice `22222222-...`), memberships, workspaces (`88888888-...`, `99999999-...`), tasks; **sem sessões**.
- Fluxo viável: subir `docker compose` (Postgres/Valkey/Meilisearch), rodar `flora-api` (aplica migrations), carregar seed, inserir sessão ativa pré-seleção (nil org, `jti` fixo) para Alice e usar JWT HS256 assinado com o mesmo `FLORA_JWT_SECRET` contendo `sub/jti/organization_id/email/exp/iat/ver` compatíveis; chamar `select-organization` para obter token org-scoped e seguir `GET /workspaces`, `POST /tasks`.
- `GET /api/v1/tasks` é filtrado por assignee do usuário autenticado no org; task criada via API não define assignee por padrão → criação pode suceder sem aparecer no list-after-create assignee-scoped (nuance documentada).

## Product/research synthesis (80/20, multi-idioma)
- MVP leve para VPS ~2 vCPU/4 GB: Postgres-first, Valkey opcional e limitado, busca dedicada diferida (FTS primeiro), escopo chat/tasks/files básico com tenancy clara.
- Auth multi-tenant recomendada: um org ativo por token/sessão, `select-organization` membership-checked, rotação de sessão/`jti`, autoridade de sessão no DB para revogação/consistência; browser ideal futuro cookie HttpOnly/BFF. OIDC ID token verification completa segue lacuna conhecida (não escopo 008).

## Frontend alignment (deferido)
- `apps/frontend` não persiste `state` OIDC, não chama `select-organization`, não substitui token rotacionado, payloads/shapes e UI de tasks/workspaces desalinhados; menor fatia coerente exige gate org/workspace e cliente/state real — **deferido** para ciclo futuro.

## Decision input
- **CI-safe agora:** testes de contrato sem DB (missing auth, nil org) adicionados e validados.
- **DB-backed cases (success/non-member/invalid-inactive session):** viáveis localmente com Postgres+migrations+fixtures, porém **NV em CI** sem infra; implementar como automatizados exigiria serviço DB em CI ou estratégia `#[ignore]`/env-gated — documentado como lacuna controlada neste incremento.
- **Dev smoke:** caminho reproduzível documentado e bootstrável deterministicamente (`scripts/dev-smoke-session.sql`, `docs/dev-smoke.md`, `test-flora.sh`); execução integrada **NV** neste ambiente.
