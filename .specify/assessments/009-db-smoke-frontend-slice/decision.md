# Decision: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Date:** 2026-11-15  
**Verdict:** **go** (escopo implementado com NVs explícitos)

## Rationale
- Alta alavancagem de verificabilidade: cobrir casos DB críticos de `select-organization` e comprovar smoke integrado reduz risco do fluxo núcleo (auth/tenancy bootstrap) em ambiente VPS.
- Execução viável e segura no ambiente: smoke integrado evidenciado via serviços efêmeros em portas alternativas (evitando conflito de `5432` no host) e testes DB validados contra Postgres real.
- Frontend: fatia mínima coerente e determinística entregue para alinhar contrato crítico (state/callback + select-org/token rotation + workspace/task API), com verificação de build/check sujeita a toolchain do ambiente.

## Scope delivered
1. **DB-backed tests (`#[ignore]` + env-gated):** `flora-tests/src/contract/auth.rs` — sucesso com rotação/revogação, 403 não-membro, 401 sessão ausente, inativa e user mismatch; executáveis com `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`.
2. **CI:** `.github/workflows/rust.yml` adiciona job separado `db-tests` (Postgres service, healthcheck, env, `--ignored --test-threads=1`) mantendo job `build` intacto.
3. **Smoke integrado executado:** `select org → workspace → task` **rc=0** (alt-port stack: pg18:15432, valkey:16379, meili:17700; API local; seed + `scripts/dev-smoke-session.sql` + `test-flora.sh`). Evidência registrada no assessment/spec.
4. **Frontend mínimo:** `apps/frontend` — persistência de `state/redirect_uri` no login, callback valida/envia `state` e redireciona para seleção de org, `AuthState` com org/workspace persistidos e rotação de token, rota `/select-org`, guard em layout `(app)`, página de tasks com seleção de workspace e create/list via API.

## NV / limitations
- **Compose-as-documented:** bloqueado neste host por conflito em `5432`; tentativa com `postgres:16-alpine` falhou na migração (`uuidv7()` inexistente). Smoke comprovado com **Postgres 18** (compatibilidade nativa `uuidv7`) e mapeamento alternativo — nuance a reconciliar (imagem/compat da migração/compose).
- **Verificação frontend completa (NV parcial):** lint/check/build/unit do `apps/frontend` dependem de toolchain/deps (`bun`) no ambiente; mudanças são focadas e tipadas, mas execução determinística completa registrada conforme viável.
- **Cobertura DB em CI remoto:** depende da execução do novo job em runners GitHub (configurado, não observável localmente além da definição/execução local dos testes).
- Fora de escopo: OIDC/JWKS completo, session authority no extrator, UX/paridade frontend ampla, realtime/files/search avançados.

## Next 80/20
- Reconciliar `docker-compose.yml`/migração para compatibilidade `uuidv7` no Postgres alvo (16 vs 18/extensão/função) e validar smoke via compose documentado.
- Evidenciar `db-tests` verde no CI do PR e ampliar casos DB se necessário (expirada por tempo, refresh interactions se aplicável).
- Refinar frontend (erros/refresh/logout server-side, workspace create/list dedicado, tipos/DTOs compartilhados) e validar com `bun` em CI/ambiente preparado.
