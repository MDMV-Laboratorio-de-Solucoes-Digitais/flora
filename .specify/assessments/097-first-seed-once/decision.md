# Decision: 097 — gating do link "primeira configuração" no bootstrap one-time

- **Slug**: 097-first-seed-once
- **Decidido**: 2026-11-21
- **Verdict**: **go**
- **Branch**: `feature/097-first-seed-once` @ `696849934`
- **Âncora**: `.specify/feature.json` → `specs/097-first-seed-once`
- **Diagnóstico**: transcrição `FirstSeedScout` (somente leitura)
- **Artefatos revisados**: intake | research | define | shape | spec | plan | tasks

## Contexto / problema — causa raiz determinística

O convite "É uma instalação nova? Fazer primeira configuração" (original
_"Is this a fresh install? Start First Seed Journey"_) continua na tela de login
depois de a conta `luis.granja@mdmv.com.br` ter sido criada. A cadeia está toda no
código comitado e foi confirmada em produção por leitura:

1. `apps/frontend/src/routes/(auth)/+page.svelte:17` — `isSetup` é estado simulado,
   sem fonte no backend; `toggleSetup()` (`:174-175`) só o inverte.
2. `+page.svelte:236-242` — o bloco do link é renderizado **incondicionalmente**
   no branch de login; não existe consulta nem condição de disponibilidade.
3. `flora-api/src/routes/bootstrap.rs:58-59` — `create_bootstrap_router()` registra
   apenas `POST /bootstrap`; `GET /api/v1/bootstrap` em produção responde **405**.
4. `flora-api/src/routes/bootstrap.rs:276-290` — a única definição de
   "fresh install" está dentro do `POST`: `pg_advisory_xact_lock` +
   `COUNT(*) FROM organizations WHERE id <> nil` → 409 `bootstrap already completed`.
5. `flora-api/src/routes/org.rs:80,90` e `workspace.rs:84,97` exigem sessão;
   `health.rs:10-28` não expõe estado; logo, não há gating pré-login possível hoje.
6. `GET https://flora.mdmv.qzz.io/` contém o texto do link em produção;
   `GET /api/v1/organizations` → 401. O caso 409 já é tratado em
   `+page.svelte:140-142`, mas apenas no submit — não há reação anterior.

Não há indício de falha intermitente: o comportamento observado é exatamente o
que o código produz. O erro OIDC reportado (`redirect_uri is not allowed`) é
independente e foi registrado no ciclo 096 — fora deste âmbito.

## Decisão

Registrar o ciclo 097 como **gating reativo e idempotente do link de primeira
configuração**, com seis compromissos rastreáveis (FR-001..FR-006), três
critérios (SC-001..SC-003) e dez tarefas (T001..T010, todas concluídas):

1. **Predicado (FR-001 / T001)**: extrair `installation_is_empty(db)` de
   `bootstrap.rs:276-290`, com o mesmo critério de organizações não-sistema.
2. **Status (FR-002 / SC-002 / T002)**: `GET /api/v1/bootstrap/status` público →
   `{ "available": bool }`, `Cache-Control: no-store`, sem dados de usuários;
   POST + advisory lock seguem como guarda autoritativa.
3. **Contratos (FR-005 / T003)**: vazio / pós-201 / org pré-existente / usuário
   órfão / sem `Authorization` / `no-store` / concorrência preservada.
4. **Frontend (FR-003 / T004–T005)**: `canBootstrap` falso por default, fetch no
   mount, render condicional e fail-closed em erro de rede/404/405/503.
5. **Idempotência reativa (FR-004 / T006)**: 409 no submit → `canBootstrap=false`,
   mensagem legível e sem navegação; OIDC e `isSubmitting` intactos.
6. **Specs e fecho (FR-006 / T007–T010)**: mocks de status/409/rede, regressão
   OIDC/bootstrap, gates do alvo PASS e rastreabilidade documental.

## Evidência de entrada

- Baseline `696849934`, branch `feature/097-first-seed-once`.
- `FirstSeedScout`: investigação somente leitura de frontend, bootstrap, rotas,
  contratos e leitura de produção; nenhum arquivo editado, nenhum gate reexecutado.
- `flora-api/src/routes/mod.rs:32-33` — sub-router público em `/api/v1`, sem
  alteração de autenticação necessária para o GET de status.
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql:5-8` —
  `users.email` UNIQUE; criação atômica com rollback em 409.
- `flora-tests/src/contract/bootstrap.rs:138,188,211,229` e
  `apps/frontend/src/routes/(auth)/login-error.svelte.spec.ts:15,101,138` —
  suítes existentes a estender, não a substituir.

## Racional

**go.** O defeito é consequência direta de três pontos verificáveis: link
incondicional no frontend, predicado de "fresh" confinado ao `POST` e ausência de
endpoint de status. A correção é isolada: uma função extraída, um GET booleano
somente leitura, uma condição de render e o tratamento reativo de um 409 já
existente. Nenhum gate, contrato do `POST`, lock, critério de organização ou
infraestrutura é alterado. Todos os FR/SC são verificáveis por contratos, specs e
inspeção locais; o diff é reversível e a fronteira é preservada. Falhar fechado
custa, no pior caso, um link ausente enquanto a API não responde — aceitável
frente ao risco de oferecer bootstrap em instalação já configurada.

## [NV]

- Existência real de `luis.granja@mdmv.com.br` e de organizations no DB de
  produção, bem como a via de criação da conta e as variáveis `FLORA_*`
  efetivamente definidas no serviço: **[NV]**. Não há endpoint público de
  contagem; hosts SSH falharam com `Connection reset by /run/ssh-unix-local/socket`;
  nenhum `POST /bootstrap` foi executado na investigação.
- Configuração de OIDC, allowlist de redirect e deploy remoto: **[NV]** —
  tratados em ciclos separados; este veredito autoriza apenas o gating local.
- O gate do alvo está registrado como PASS no handoff consolidado; este artefato
  não reexecutou gates, build, testes ou formatters.

## Fora de âmbito

Mudanças em `validate/`, `release/`, `.specify/memory/STATE.md` ou `PROGRESSO*`;
alteração de required checks, workflows, ADRs ou constituição; mudança no
contrato do `POST /bootstrap`, no advisory lock ou no critério de organizações
não-sistema; configuração de IdP externo ou correção da allowlist OIDC.

## Handoff

`/speckit.specify` — especificar o gating como requisitos verificáveis
(FR-001..FR-006 / SC-001..SC-003) e tarefas T001..T010, com status booleano sem
vazamento de dados, fail-closed no frontend, 409 reativo, contratos/specs
estendidos e fronteira preservada.
