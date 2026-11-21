# Research: 097 — gating do link "primeira configuração" no bootstrap one-time

**Data:** 2026-11-21 · **Evidência local determinística + leitura de produção** · IdP externo **[NV]** · branch `feature/097-first-seed-once@696849934`

## Primária (local)

1. `apps/frontend/src/routes/(auth)/+page.svelte:17` —
   `let isSetup = $state(false); // Simulated state`. O estado que controla a
   visibilidade do formulário de primeira configuração é puramente local e não
   deriva de nenhuma consulta ao backend.
2. `+page.svelte:174-175` — `toggleSetup()` apenas inverte `isSetup`. Não há
   reconsulta, reação a mudança de estado no servidor nem persistência.
3. `+page.svelte:236-242` — o bloco "É uma instalação nova? / Fazer primeira
   configuração" está no branch de login do `{#if}` e é renderizado
   **incondicionalmente**, sem nenhum `{#if isBootstrapAvailable}` ou equivalente.
   É a causa direta do link continuar aparecendo.
4. `+page.svelte:123-171` — `handleBootstrap()` chama
   `POST ${API_BASE_URL}/api/v1/bootstrap`, grava a sessão via `authState.login()`
   e navega para `/tasks`; o `case 409` (`:140-142`) já exibe "Este workspace já
   foi configurado.", mas apenas **após o submit** — não há gating antes disso.
   O `isSubmitting` (`:75,103`) já protege contra duplo envio.
5. `flora-api/src/routes/bootstrap.rs:58-59` —
   `create_bootstrap_router()` registra somente
   `Router::new().route("/bootstrap", post(bootstrap))`. Não existe `GET`,
   `/status` ou qualquer endpoint de consulta.
6. `flora-api/src/routes/bootstrap.rs:276-290` — a autoridade de "fresh install"
   vive dentro do `POST`: `pg_advisory_xact_lock(BOOTSTRAP_LOCK_KEY)` seguido de
   `SELECT COUNT(*) FROM organizations WHERE id <> nil`; se `> 0`, retorna
   `Error::DuplicateEntry("bootstrap already completed")` → 409. Usuários e
   workspaces **não** entram no critério (conforme `specs/085-local-first-seed/spec.md`,
   usuários órfãos não impedem bootstrap).
7. `flora-api/src/routes/mod.rs:32-33` — o sub-router é montado publicamente em
   `/api/v1`, sem `AuthExtractor`; um `GET /bootstrap/status` pode ser público sem
   alterar o contrato de autenticação.
8. `flora-api/src/routes/org.rs:80,90` e `workspace.rs:84,97` — `GET /organizations`
   e `GET /workspaces` exigem `AuthExtractor`/sessão; não podem servir de gating
   pré-login sem expor dados ou criar superfície autenticada.
9. `flora-api/src/routes/health.rs:10-28` — health/metrics respondem apenas
   `status: "ok"` e flags de search/messaging; não expõem contagem nem estado de
   bootstrap.
10. `flora-core/src/migrations/sql/20240818000001_init_schema.sql:5-8` e
    `flora-core/src/error.rs:242,324-325` — `users.email` é UNIQUE; violação mapeia
    para `DuplicateEntry`/409. A criação da conta é atômica: erro faz rollback de
    toda a transação (`bootstrap.rs:274-291`).
11. Contratos existentes em `flora-tests/src/contract/bootstrap.rs:138,188,211,229`
    cobrem sucesso, segunda chamada, organização pré-existente e concorrência —
    todos seguem o mesmo predicado de contagem de organizações.
12. Specs frontend em
    `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts:15,101,138` cobrem
    login local, criação de workspace com senha e conflito 409; nenhuma cobre a
    visibilidade do link de primeira configuração.

## Leitura de produção (FirstSeedScout, somente leitura)

- `GET https://flora.mdmv.qzz.io/` contém o texto do link — confirma o
  comportamento do código em produção.
- `GET https://flora-api.mdmv.qzz.io/api/v1/bootstrap` → **405** (somente POST).
- `GET https://flora-api.mdmv.qzz.io/api/v1/organizations` → **401**.
- `/health` → `{"status":"ok","search":false,"messaging":false}`;
  `/health/ready` sem corpo de erro.
- O erro OIDC reportado é independente: a URL de autorização retornada usa
  `issuer=http://localhost:8080` e `redirect_uri=http://localhost:3000/api/v1/auth/callback`
  (`flora-core/src/config/app.rs:45`, `config/oidc.rs:27-38`,
  `flora-api/src/routes/auth.rs:99-117`). Corrigir a allowlist é ticket separado.

## Diagnóstico

Cadeia fechada e determinística: o frontend renderiza o link sem consultar o
backend (1–3), o backend só sabe dizer se a instalação é nova dentro do `POST`
(5–6), e nenhum endpoint público de status existe (7–9). Portanto criar
`luis.granja@mdmv.com.br` não altera a visibilidade do link: uma nova tentativa
falha apenas no submit com 409 e, após recarregar, o estado local volta a
`false`. A correção mínima é (a) extrair o predicado para uma função reutilizável
e expô-lo como `GET /bootstrap/status` público, somente UX; (b) tornar o link
reativo a esse status no frontend, com fail-closed em erro de rede/404/405/503 e
atualização no 409. O `POST` mantém o advisory lock como guarda autoritativa.

## [NV]

- Existência real de `luis.granja@mdmv.com.br` e de organizations no DB de
  produção: não há endpoint público de contagem; hosts SSH configurados falharam
  com `Connection reset by /run/ssh-unix-local/socket`; nenhum `POST` foi feito
  (evitando mutação). **[NV]**
- Se a conta foi criada por bootstrap ou por outro caminho, e quais variáveis
  `FLORA_*` estão efetivamente definidas no serviço: a URL de autorização sugere
  defaults, mas não é prova direta do env. **[NV]**
- Correção proposta não implementada nem testada neste artefato; gates, build e
  testes são evidência consolidada do handoff. **[NV]**
- Cobertura idiomática: pt-BR nos artefatos e na UI; inglês em nomes de código,
  seguindo a convenção do repositório.
