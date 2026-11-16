# Research: 011 — Frontend↔API Contract Alignment & Session Hardening

**Date:** 2026-11-15  
**Base:** `dev@556b78cc8`  
**Inputs:** código/CI/workflows, artefatos 009–010, auditorias 011 (contrato FE↔API, autoridade de sessão/tenancy, `flora-tests`/CI, testes frontend), pesquisa multilíngue (Axum JWT+sessão DB, erros tipados em fetch/TS), síntese mdmv-dark-factory.

## Principais achados

### Frontend↔API (slice auth→org→workspace→task)
- `apps/frontend/src/lib/api/client.ts`: em falhas descarta status/corpo estruturado (`401` vira `Error('Unauthorized')` após lock; outros viram string), dificultando tratamento de 400/403/422/409 e parsing de `flora_core::error::Error` (`{error,message,field?}`).
- `POST /api/v1/tasks` (`flora-api/src/routes/tasks.rs`): request aceita `description`/`assignee_id`, mas criação usa apenas `Task::new(..., title)` (ignora campos). Frontend envia `assignee_id` ("assign to me") e espera visibilidade em listagem assignee-scoped; contrato efetivo fica desalinhado.
- `/tasks` (`apps/frontend/src/routes/(app)/tasks/+page.svelte`): sem guard explícito auth/org antes de carregar (diferente de `select-org`); tipos de resposta parciais.
- `AuthState.setOrganization` (`apps/frontend/src/lib/state/AuthState.svelte.ts`) não limpa `isInvalidated` (diferente de `login`).

### Testes frontend/CI
- Testes `AuthState.test.ts` e `api-interceptor.test.ts` cobrem pouco/estão desalinhados; CI frontend executa `check`, `lint`, `test:unit -- --run --project server`, `build` (sem e2e/client project). Ajustes mínimos podem aumentar sinal determinístico.

### Backend sessão/tenancy
- `AuthExtractor` valida JWT/claims apenas (sem lookup de sessão ativa/revogada por `jti` no DB) nas rotas protegidas; `select-organization`/`refresh` aplicam checks parciais no DB. Hardening global (DB-authoritative por request) é recomendado por boas práticas, mas neste repo exige cuidado com sessão pré-seleção (`org=nil`), consistência de extractors, perf e verificação ampla — **risco > 80/20 imediato**; adequado como follow-up focado.

### `flora-tests`/CI
- Cobertura DB-backed existente concentrada em `contract::auth::db_select_organization` (`#[ignore]`, executada em job `db-tests`); lacunas no fluxo org→workspace→task. Expansão é viável mas deve ser incremental e conectada ao CI seletivamente.

### Pesquisa (multilíngue, síntese)
- Erros tipados em fetch: preservar `status/statusText/url`, `code`/`message`/`field`/corpo bruto, tratar 204/205, centralizar efeito 401 (lock) com cuidado a efeitos duplicados; alinhar a envelope Rust/Axum atual.
- Sessão Axum+sqlx: padrão 80/20 é extrator único com verificação `jti` ativa/não expirada atrelada a user/org no Postgres (DB como autoridade), sem cache inicial em APIs pequenas; requer rollout consistente.

## Decisão de escopo
Priorizar correções contratuais e determinísticas de baixo risco (tasks create, client de erros, estado/guards/testes frontend, ergonomia docs smoke) + evidência; hardening de sessão global fica como follow-up explícito.
