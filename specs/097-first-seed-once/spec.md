# Feature Specification: 097 — gating do link "primeira configuração" no bootstrap one-time

**Feature Branch**: `feature/097-first-seed-once`

**Created**: 2026-11-21

**Status**: Implemented

**Input**: Decisão `go` de `.specify/assessments/097-first-seed-once/decision.md` e diagnóstico `FirstSeedScout`

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Não oferecer bootstrap em instalação já configurada (Priority: P1)

Como administrador de uma instalação já configurada, não quero ver o convite
"É uma instalação nova? Fazer primeira configuração" na tela de login, para não
ser levado a uma ação que só pode terminar em erro.

**Why this priority**: é o defeito relatado. O link é renderizado
incondicionalmente em `+page.svelte:236-242`, enquanto a única definição de
"instalação vazia" está dentro do `POST /bootstrap` (`bootstrap.rs:276-290`).
Criar `luis.granja@mdmv.com.br` não altera a visibilidade do link.

**Independent Test**: com o banco já inicializado pelo bootstrap, abrir a tela de
login e confirmar que o link não aparece; em banco vazio, confirmar que o link
aparece e que o fluxo de bootstrap continua criando a conta e a sessão.

**Acceptance Scenarios**:

1. **Given** uma instalação com ao menos uma organização não-sistema, **When** a
   tela de login carrega, **Then** `GET /api/v1/bootstrap/status` responde
   `{ "available": false }` e o link de primeira configuração NÃO é renderizado.
2. **Given** um banco vazio, **When** a tela de login carrega, **Then** o status
   responde `{ "available": true }`, o link é exibido e o envio via "Fazer primeira
   configuração" cria organização, usuário, papel, associação, workspace e sessão
   (201), navegando para `/tasks`.
3. **Given** a tela já carregada, **When** o usuário recarrega a página, **Then**
   a decisão volta a derivar do status do servidor — não de estado local persistido.

---

### User Story 2 — Status público, mínimo e somente leitura (Priority: P1)

Como mantenedor, preciso que o frontend possa consultar a disponibilidade de
bootstrap antes do login sem expor dados da instalação e sem transformar essa
consulta em autorização.

**Why this priority**: o `GET /organizations` exige sessão (`org.rs:80,90`) e o
`health` não expõe estado (`health.rs:10-28`); sem um status público e mínimo, o
gating pré-login seria impossível ou vazaria informações.

**Independent Test**: chamar `GET /api/v1/bootstrap/status` sem header
`Authorization` em banco vazio e após um bootstrap, conferindo o booleano, o
header `Cache-Control: no-store` e a ausência de contagens/e-mails/nomes.

**Acceptance Scenarios**:

1. **Given** um banco vazio, **When** o status é consultado sem autenticação,
   **Then** a resposta é `200` com `{ "available": true }`, `Cache-Control: no-store`
   e nenhum outro dado.
2. **Given** um `POST /api/v1/bootstrap` já concluído (201) ou uma organização
   pré-existente, **When** o status é consultado, **Then** `available` é `false`.
3. **Given** um usuário órfão e zero organizações, **When** o status é consultado,
   **Then** `available` é `true` — o critério continua sendo somente organizações
   não-sistema, idêntico ao do `POST`.
4. **Given** uma consulta de status concorrente a um `POST`, **When** o `POST`
   vence a corrida, **Then** ele retorna 409 `bootstrap already completed` sob
   `pg_advisory_xact_lock`; o GET é apenas UX e nunca substitui essa guarda.

---

### User Story 3 — Falha fechada e 409 reativo no frontend (Priority: P2)

Como usuário, quero que o link desapareça se a API não responder ou se outra
pessoa configurar a instalação enquanto eu estou na tela, e receber uma mensagem
legível em vez de ser navegado para uma rota inválida.

**Why this priority**: sem fail-closed, erro de rede/404/405/503 exibiria o link
em instalação já configurada; sem tratar o 409 no submit, a navegação para
`/tasks` usaria uma sessão inexistente.

**Independent Test**: simular erro de rede no status e conferir link ausente;
simular 409 no `handleBootstrap()` e conferir `canBootstrap = false`, mensagem
"Este workspace já foi configurado." visível e `goto` não chamado.

**Acceptance Scenarios**:

1. **Given** uma falha de rede, 404, 405, 503 ou payload inválido no status,
   **When** a tela de login carrega, **Then** o link permanece oculto (default
   `canBootstrap = false`) e o fluxo OIDC continua disponível.
2. **Given** uma tentativa de bootstrap que recebe 409, **When** o erro é
   processado, **Then** `canBootstrap` passa a `false`, a mensagem em pt-BR é
   exibida e não há navegação para `/tasks`.
3. **Given** um envio em andamento, **When** o usuário clica novamente, **Then**
   `isSubmitting` impede o duplo envio; o botão SSO e `handleLogin()` seguem
   intactos.

---

### Edge Cases

- `GET /bootstrap/status` é somente leitura: não cria organizations, usuários,
  workspaces nem sessões, e não adquire o lock transacional do `POST`.
- Usuários órfãos (sem organization) e workspaces não entram no predicado; o
  critério é exclusivamente `COUNT(*) FROM organizations WHERE id <> nil`.
- Concorrência GET/POST: o GET pode estar momentaneamente desatualizado; o `POST`
  é a autoridade final e retorna 409 se a instalação deixou de estar vazia.
- Resposta de status inválida (payload ausente, tipo inesperado) é tratada como
  falha — link oculto.
- Um 409 após o submit corrige `canBootstrap` na hora; recarregar a página
  volta a consultar o status.
- Nenhum e-mail, contagem, nome ou detalhe de usuário pode aparecer na resposta
  de status, em log ou em artefato do ciclo.
- O erro OIDC (`redirect_uri is not allowed`) e a configuração de IdP externo são
  independentes e permanecem **[NV]** — fora deste escopo.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: O sistema DEVE extrair o predicado de instalação vazia de
  `flora-api/src/routes/bootstrap.rs:276-290` para uma função reutilizável
  `installation_is_empty(db)`, baseada em
  `COUNT(*) FROM organizations WHERE id <> nil`, preservando exatamente o mesmo
  critério do `POST` (usuários órfãos e workspaces não alteram o resultado).
- **FR-002**: O sistema DEVE expor `GET /api/v1/bootstrap/status`, público e sem
  `Authorization`, respondendo `{ "available": bool }` com
  `Cache-Control: no-store`. A resposta NÃO DEVE conter contagens, e-mails, nomes
  ou qualquer detalhe de usuários/organizações. O status é exclusivamente UX;
  o `POST /api/v1/bootstrap` com `pg_advisory_xact_lock` permanece a guarda
  autoritativa e inalterada em seu comportamento (201/409).
- **FR-003**: O frontend DEVE manter `canBootstrap = $state(false)` por default,
  consultar o status no mount da tela de login e renderizar o bloco "É uma
  instalação nova? / Fazer primeira configuração" somente quando
  `available === true`.
- **FR-004**: O frontend DEVE falhar fechado: erro de rede, 404, 405, 503 ou
  qualquer resposta inválida de `/bootstrap/status` DEVE manter o link oculto.
  No `handleBootstrap()`, um 409 DEVE atualizar `canBootstrap = false`, exibir a
  mensagem legível já existente ("Este workspace já foi configurado.") e NÃO
  navegar para `/tasks`; `isSubmitting` e o fluxo OIDC permanecem intactos.
- **FR-005**: Os contratos backend DEVEM cobrir: `available: true` em banco
  vazio, `false` após POST 201, `false` com organização pré-existente, `true` com
  usuário órfão e zero organizações, ausência de header `Authorization`, presença
  de `Cache-Control: no-store`, e os testes de concorrência existentes devem
  continuar passando sem alteração de semântica.
- **FR-006**: As specs frontend DEVEM cobrir: status `false` → link ausente;
  status `true` → link presente; 409 no submit → link removido, erro visível e
  `goto` não chamado; erro de rede → link oculto; e DEVEM ajustar os testes
  existentes para incluir a chamada de status. Os gates do alvo DEVEM permanecer
  PASS e o diff NÃO DEVE tocar em `validate/`, `release/`, `STATE.md` ou
  `PROGRESSO*`.

### Key Entities

- **installation_is_empty**: predicado único, extraído do `POST`, que responde se
  existem organizações não-sistema; fonte de verdade para o status e para o POST.
- **Bootstrap status**: `GET /api/v1/bootstrap/status` → `{ "available": bool }`,
  público, `no-store`, sem dados de usuários; somente UX, nunca autorização.
- **canBootstrap**: estado do frontend, falso por default, derivado do status no
  mount e corrigido no 409; substitui o estado simulado `isSetup`.
- **Bootstrap POST**: guarda autoritativa inalterada — advisory lock + contagem +
  409 `bootstrap already completed`, emissão de sessão em transação única.
- **Fluxo OIDC**: `handleLogin()`, SSO e `isSubmitting`, preservados.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Com a instalação já configurada (conta `luis.granja@mdmv.com.br` e
  workspace criados), a tela de login NÃO exibe mais o link de primeira
  configuração; em banco vazio o link continua disponível e o fluxo de bootstrap
  segue funcional (201 → sessão → `/tasks`).
- **SC-002**: Nenhum dado sensível é exposto pelo status: sem e-mails, contagens,
  nomes ou detalhes de usuários; `GET /bootstrap/status` é público,
  `Cache-Control: no-store`, e o `POST` continua 409 em instalação não vazia sob
  o mesmo lock — o status nunca substitui a guarda autoritativa.
- **SC-003**: Falha de rede/404/405/503 no status esconde o link (fail-closed) e
  um 409 no submit remove o link sem navegação; contratos e specs frontend
  passam, o gate do alvo é PASS (registrado no handoff) e o diff não toca em
  `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Assumptions

- O diagnóstico é o da transcrição `FirstSeedScout`: o link é incondicional no
  frontend e o backend só verifica "fresh" dentro do `POST`, sem endpoint de status.
- A existência real da conta e de organizations no DB de produção, a via de
  criação e as variáveis `FLORA_*` efetivas são **[NV]** — não foi feito `POST`
  algum na investigação e hosts SSH falharam.
- A configuração de OIDC/allowlist de redirect e o deploy remoto são **[NV]** e
  pertencem a ciclos separados; esta especificação cobre apenas o contrato local
  observável do gating.
- O critério de "instalação vazia" continua sendo o de
  `specs/085-local-first-seed/spec.md`: somente organizações não-sistema contam;
  usuários órfãos não impedem bootstrap.
- As verificações e o estado do gate são os registrados no handoff consolidado;
  este artefato não reexecutou gates, build, testes ou formatters.
