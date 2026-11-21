# Intake: 097 — gating do link "primeira configuração" no bootstrap one-time

**Data:** 2026-11-21T00:00Z · **Âncora:** `feature/097-first-seed-once@696849934` · `.specify/feature.json` → `specs/097-first-seed-once`

## Sinal

Bug reportado: o link "É uma instalação nova? Fazer primeira configuração" (original
_"Is this a fresh install? Start First Seed Journey"_) continua aparecendo na tela de
login mesmo depois de a conta `luis.granja@mdmv.com.br` e seu workspace terem sido
criados via `POST /api/v1/bootstrap`. O diagnóstico do `FirstSeedScout` é
determinístico: o bloco é renderizado **incondicionalmente** em
`apps/frontend/src/routes/(auth)/+page.svelte:236-242`; `toggleSetup()`
(`:174-175`) apenas alterna estado local. O backend só verifica "fresh install"
**dentro** de `POST /bootstrap` (`flora-api/src/routes/bootstrap.rs:276-290`) e não
expõe nenhum endpoint de status consultável antes do login. Criar a conta não faz
o link desaparecer — o estado local volta a `false` a cada recarregamento.

## Evidência

```
+page.svelte:17                 isSetup = $state(false)  // estado simulado, sem fonte no backend
+page.svelte:174-175            toggleSetup() só alterna estado local
+page.svelte:236-242            bloco "É uma instalação nova? Fazer primeira configuração" sempre renderizado
+page.svelte:123-171            handleBootstrap() -> POST /api/v1/bootstrap, 409 -> "Este workspace já foi configurado."
+page.svelte:37                 handleLogin() fluxo OIDC inalterado
bootstrap.rs:58-59              create_bootstrap_router() registra SOMENTE POST /bootstrap
bootstrap.rs:276-290            pg_advisory_xact_lock + COUNT(*) organizations (id <> nil) -> 409 "bootstrap already completed"
flora-api/src/routes/mod.rs:32-33  mount público em /api/v1
flora-api/src/routes/org.rs:80,90 / workspace.rs:84,97  GET exigem AuthExtractor (inúteis para gating pré-login)
flora-api/src/routes/health.rs:10-28  health/metrics não expõem contagem nem estado de bootstrap
GET https://flora.mdmv.qzz.io/  contém o texto do link (produção, leitura)
GET /api/v1/bootstrap           -> 405 (somente POST)
GET /api/v1/organizations       -> 401
```

Registro completo da investigação: transcrição `FirstSeedScout` (somente leitura,
nada editado, nenhum gate/build/teste reexecutado).

## Autorização

Modo autônomo. Decisão `go` registrada em
`.specify/assessments/097-first-seed-once/decision.md` antes da implementação.
Escopo: extrair o predicado `installation_is_empty(db)` e expor
`GET /api/v1/bootstrap/status` (`{ "available": bool }`, `Cache-Control: no-store`,
sem e-mails ou contagens), gating reativo no frontend com fail-closed e tratamento
de 409. O `POST` e o advisory lock permanecem a guarda autoritativa; o status é
apenas UX, nunca autorização. IdP externo, deploy remoto e estado real do DB de
produção permanecem **[NV]**.

## Fronteira

Backend Rust (`flora-api/src/routes/bootstrap.rs`, `routes/mod.rs`), frontend
SvelteKit (`+page.svelte` e specs), contratos/tests
(`flora-tests/src/contract/bootstrap.rs`) e artefatos do ciclo. Sem toques em
`validate/`, `release/`, `.specify/memory/STATE.md` ou `PROGRESSO*`. Nenhuma
mudança de gate, workflow, ADR ou constituição.
