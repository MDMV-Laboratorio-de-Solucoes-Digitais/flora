# Define: 097 — gating do link "primeira configuração" no bootstrap one-time

**Data:** 2026-11-21 · **Branch**: `feature/097-first-seed-once@696849934`

## Problema

A tela de login exibe incondicionalmente o convite "É uma instalação nova?
Fazer primeira configuração" (`+page.svelte:236-242`), mesmo quando a instalação
já foi configurada. O único lugar que sabe responder se a instalação está vazia é
o `POST /api/v1/bootstrap` (`bootstrap.rs:276-290`), e ele só responde no momento
do submit, com 409. Não existe endpoint de status consultável antes do login, e
o estado `isSetup` do frontend é simulado (`:17`). Resultado: depois de criar
`luis.granja@mdmv.com.br`, o link continua aparecendo; uma nova tentativa só falha
no envio e, ao recarregar, tudo volta ao estado inicial. A correção precisa ser
mínima, reversível e segura: expor o predicado de "instalação vazia" sem revelar
dados, tornar a UI reativa a ele e falhar fechado quando o status não puder ser
obtido — sem transformar o status em autorização e sem enfraquecer a guarda
atômica do `POST`.

## Requisitos funcionais

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

## Critérios de sucesso

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

## Fora de escopo

Corrigir a configuração de OIDC/allowlist de redirect (ticket separado do ciclo
096); configurar IdP externo; alterar o contrato do `POST /bootstrap`, o advisory
lock, o critério de organizações não-sistema ou a emissão de sessão; mudar gates,
workflows, ADRs ou a constituição; tocar em `validate/`, `release/` ou estado de
governança. IdP externo, deploy remoto e o estado real do DB de produção
permanecem **[NV]**.
