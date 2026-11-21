# Shape: 097 — gating do link "primeira configuração" no bootstrap one-time

**Data:** 2026-11-21 · **Branch**: `feature/097-first-seed-once@696849934`

## Forma escolhida

Um status endpoint mínimo, somente leitura, que reutiliza o predicado já
autoritativo do bootstrap — sem duplicar regra e sem expor dados.

1. **Predicado (T001)**: extrair de
   `flora-api/src/routes/bootstrap.rs:276-290` a lógica
   `COUNT(*) FROM organizations WHERE id <> nil` para
   `async fn installation_is_empty(db) -> Result<bool>`. O `POST` continua
   envolvendo-o no `pg_advisory_xact_lock` e respondendo 409 como antes.
2. **Endpoint (T002)**: registrar `GET /bootstrap/status` em
   `create_bootstrap_router()` (`:58-59`, já montado em `/api/v1` por
   `routes/mod.rs:32-33`), público, com resposta `{ "available": bool }` e
   `Cache-Control: no-store`. Nenhuma contagem, e-mail ou nome sai na resposta.
3. **Contratos (T003)**: estender `flora-tests/src/contract/bootstrap.rs` com os
   casos de status (vazio / pós-201 / org pré-existente / usuário órfão), Sem
   `Authorization`, header `no-store`, preservando os testes de concorrência.
4. **Estado no frontend (T004)**: em `+page.svelte`, trocar o estado simulado
   `isSetup` (`+:17`) por `canBootstrap = $state(false)` e buscar o status no
   mount. A variável de toggle permanece apenas para abrir/fechar o formulário.
5. **Render e fail-closed (T005)**: condicionar o bloco de `:236-242` a
   `canBootstrap === true`; qualquer falha de status (rede, 404, 405, 503, payload
   inválido) mantém o link oculto. O fluxo OIDC (`handleLogin`, `:37`) fica
   intacto.
6. **409 reativo (T006)**: no `handleBootstrap()` (`:123-171`), o `case 409`
   já existente passa a definir `canBootstrap = false` junto com a mensagem
   "Este workspace já foi configurado.", sem `goto('/tasks')`.
7. **Specs frontend (T007)**: estender
   `login-error.svelte.spec.ts` com mocks de status `true`/`false`, 409 no submit
   e erro de rede; ajustar os testes existentes (`:101,138`) para incluírem a
   chamada de status.
8. **Regressão e fecho (T008–T010)**: garantir que OIDC e o `POST /bootstrap`
   original não mudaram, conferir gates do alvo (PASS no handoff) e fechar a
   rastreabilidade T001..T010 em decision/spec/plan/tasks sem tocar em
   `validate/`, `release/`, `STATE.md` ou `PROGRESSO*`.

## Riscos e mitigação

- **Status usado como autorização**: mitigado ao manter o `pg_advisory_xact_lock`
  e o 409 no `POST` como guarda autoritativa; o GET é explícito sobre ser apenas
  UX (FR-002/SC-002). Concorrência entre GET e POST é inofensiva — o pior caso
  é o link aparecer por um instante e o submit retornar 409, que já é tratado.
- **Vazamento de informação**: resposta booleana, sem contagens/e-mails/nomes;
  `Cache-Control: no-store` evita caching de estado (FR-002/SC-002).
- **Link "fantasma" em erro**: fail-closed — erro de rede/404/405/503 esconde o
  link em vez de exibi-lo por default (FR-004/SC-003). Custo aceito: em instalação
  realmente vazia com API indisponível, o link não aparece até a API responder.
- **Divergência de critério**: uma única função `installation_is_empty` é a fonte
  do GET e do POST, eliminando a possibilidade de o status dizer "disponível"
  enquanto o bootstrap recusa (FR-001).
- **Estado local obsoleto**: o 409 no submit corrige `canBootstrap` imediatamente
  (FR-004); recarregar a página volta a consultar o status.
- **Regressão OIDC/frontend**: `handleLogin`, o botão SSO e `isSubmitting`
  permanecem intactos; testes existentes são ajustados, não removidos (T008).

## Entrega

Diff restrito a `flora-api` (bootstrap + testes de contrato) e `apps/frontend`
(`+page.svelte` + specs), mais artefatos do ciclo 097. Verificação local cobre
T003, T007, T008 e os gates do alvo (PASS conforme handoff consolidado). IdP
externo, configuração real de OIDC, deploy remoto e o estado do DB de produção
permanecem **[NV]**.
