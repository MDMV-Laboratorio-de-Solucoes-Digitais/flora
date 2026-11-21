# Release: 097 — Primeira configuração condicionada ao estado da instalação

**Ciclo**: 097 · **Data**: 2026-11-20 · **Branch**: `feature/097-first-seed-once` @ `696849934` · **Baseline**: `696849934`
**Tipo**: correção de exibição — endpoint read-only de estado da instalação, frontend condicional com fail-closed e guardião 409 do bootstrap preservado. **Nenhuma alteração de gate, required check, constituição, ADR, schema ou PR/merge/tag.** O estado real da instalação em produção e o comportamento do link após deploy seguem **[NV]** até o deploy final.

## Entregue

- `GET /api/v1/bootstrap/status`: rota read-only e pública que responde `{ "is_fresh": boolean }`, com `is_fresh = (contagem de organizações não-sistema == 0)`, sem efeito colateral, sem autenticação e sem dados de tenant/PII (FR-001, FR-003, SC-001).
- Helper de contagem de organizações não-sistema (`id <> '00000000-0000-0000-0000-000000000000'`) extraído do `POST /bootstrap` e compartilhado com a rota de status, eliminando divergência entre o sinal exibido e o critério do guardião; `BOOTSTRAP_LOCK_KEY`, `pg_advisory_xact_lock` e o `Error::DuplicateEntry("bootstrap already completed")` (409) permanecem intactos (FR-002, SC-002).
- Falha de banco na rota de status devolve `500` sem detalhes e o frontend trata erro/indefinido como "não fresh" — fail-closed (FR-003, FR-005, SC-002, SC-003).
- Tela de login (`apps/frontend/src/routes/(auth)/+page.svelte`): consulta o status ao montar e renderiza "É uma instalação nova? Fazer primeira configuração" somente quando `is_fresh === true`; em instalação configurada ou erro, o link é oculto e a tela segue com Entrar/SSO, sem regressão de login local/SSO/bootstrap (FR-004, FR-005, SC-001, SC-003).
- Testes cobrindo os três estados (vazio → link visível, configurado → oculto, erro/indefinido → oculto), o contrato JSON endpoint↔frontend e o fluxo bootstrap → sessão → `/api/v1/tasks` com o caso 409 preservado; `check_gate_integrity.py` registrado **PASS** no diagnóstico. Detalhe literal em `.specify/validations/097-first-seed-once/validate.md` (R4 **APROVADO**), com **[NV]** para deploy/Coolify/E2E autenticado até o deploy final (FR-001..FR-006, SC-001..SC-003; T001..T010).

## SemVer

**Proposta:** MINOR `0.6.0` → `0.7.0`. Motivo: adiciona uma capacidade observável ao produto — um endpoint read-only que expõe o estado de instalação e uma tela de login que condiciona o fluxo de primeira configuração a esse estado — sem remover nenhum fluxo existente. Não é patch: há novo contrato de endpoint e mudança observável na tela de login (o link deixa de aparecer em instalação configurada). Não é major: nenhuma API pública existente, contrato de dados, schema ou rota anterior é quebrado; o login local/SSO, o `POST /bootstrap` e o guardião 409 permanecem válidos. **Nenhuma tag criada**; registro a critério do maintainer.

## Changelog (FR / T / bug)

- `FR-001` `GET /api/v1/bootstrap/status`, read-only, responde `is_fresh` sem autenticação nem efeito colateral (T001, T004).
- `FR-002` contagem de organizações não-sistema compartilhada com o guardião do `POST /bootstrap`, sem remover advisory lock nem 409 (T002, T004).
- `FR-003` resposta mínima `{ "is_fresh": boolean }`, sem PII; `500` genérico em erro de DB e fail-closed no cliente (T001, T003).
- `FR-004` tela de login consulta o status ao montar e exibe o link de primeira configuração somente com `is_fresh === true` (T005, T006, T008).
- `FR-005` em configurada/erro/indefinido o link é oculto; Entrar/SSO e o 409 direto permanecem (T005, T006, T007).
- `FR-006` nenhuma alteração de gate, required check, constituição, ADR, schema ou PR/merge/tag; mudança restrita à rota, ao helper, ao frontend e aos artefatos do ciclo (T003, T007, T009, T010).
- `T001` rota de status + struct de resposta · `T002` helper de contagem compartilhado · `T003` testes do endpoint · `T004` registro da rota e preservação do guardião · `T005` estado `isFresh` + fetch no frontend · `T006` link condicional com fail-closed · `T007` testes da tela nos três estados · `T008` contrato de integração · `T009` artefatos do ciclo · `T010` fecho de gates/lockfile/supply-chain/revisão.
- **Bug**: nenhum ID de `.specify/bugs/` é acionado neste ciclo. O defeito relatado — o link "É uma instalação nova? Fazer primeira configuração" ser incondicional e levar a um 409 em instalação já configurada — é tratado por FR-001..FR-006 / T001..T010 e registrado em `docs/governance/2026-11-21-097-first-seed-once.md`; nenhum bug externo é aberto ou reclassificado por este release.

## Evidência

`.specify/validations/097-first-seed-once/validate.md` (R4 **APROVADO**; gate do alvo `check_gate_integrity.py` **PASS** registrado no diagnóstico, demais verificações esperadas do handoff e [NV] para deploy/E2E autenticado até o deploy final) · `specs/097-first-seed-once/analyze.md` (READ-ONLY, `critical=0`, `requirements_covered=6/6`, `tasks=10/10`) · `specs/097-first-seed-once/converge.md` (**Converged**, sem novas tarefas) · baseline `696849934`. Nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release.

## Rollback

Restaurar o estado anterior da tela de login — **reapontar o app/web** e **reverter a rota de status, o helper e o frontend**, sem force-push e sem reescrita de histórico:

1. no Coolify, reapontar o app `Flora Web` (FQDN `https://flora.mdmv.qzz.io`) para a imagem/build anterior ao ciclo 097 e executar o redeploy; AGUARDAR o deploy terminar antes de declarar a reversão concluída. O link "É uma instalação nova? Fazer primeira configuração" volta a aparecer incondicionalmente — comportamento anterior esperado, não regressão do rollback;
2. reverter a config/rotas: remover o mount de `GET /api/v1/bootstrap/status` e restaurar `create_bootstrap_router()` para registrar somente `POST /bootstrap`; não há variáveis de ambiente específicas deste ciclo para limpar;
3. restaurar a contagem de organizações não-sistema inline em `POST /bootstrap`, preservando `BOOTSTRAP_LOCK_KEY`, `pg_advisory_xact_lock` e o `Error::DuplicateEntry("bootstrap already completed")` (409); nenhum dado de tenant é alterado pela reversão;
4. restaurar `apps/frontend/src/routes/(auth)/+page.svelte` ao estado pré-097 (sem estado `isFresh` e com o link incondicional no branch `{:else}`); se as alterações ainda estiverem não commitadas, descartar apenas os arquivos do ciclo 097; se commitadas, usar novo commit de reversão ou `git revert` — nunca `--force`. Os documentos do ciclo (decision/governança/spec/plan/tasks/analyze/converge, R4 e este release) são registros e podem permanecer como histórico;
5. **não reverter** gate, `check_gate_integrity.py`, required checks, constituição, ADR-0003, login local/SSO, o guardião 409 do bootstrap, PRs, merge ou tags — nenhum deles foi alterado por este ciclo;
6. após o redeploy e a reversão, reexecutar `python3 scripts/ci/check_gate_integrity.py` (esperado permanecer **PASS**, `exit=0`), conferir `/`, `/config.js` e `/health` → 200 e confirmar que o link voltou ao comportamento anterior (incondicional) e que o bootstrap direto em instalação configurada torna a falhar com 409. Com a UI anterior, o link voltará a ser oferecido em instalação configurada — esse é o estado conhecido que este ciclo veio corrigir.

## [NV]

- **Estado real da instalação**: se a instalação de destino está vazia ou configurada não foi medido; o comportamento do link após deploy permanece **[NV]** até o deploy final (SC-001).
- **E2E autenticado**: bootstrap → sessão → `GET /api/v1/tasks`, incluindo o 409 preservado em chamada direta/concorrente, é evidência esperada do handoff, não observação ao vivo nesta passagem (SC-002).
- **CI/PR/produção**: nenhum PR, merge, tag, aprovação, proteção de branch ou ambiente de produção verificado; gate PASS registrado no diagnóstico não substitui CI remota para promoção.
- **Coolify/Forgejo/DNS**: publicação de imagem, resultado do deploy e medição em navegador real dos dois estados da tela permanecem **[NV]** até o deploy final, conforme a nota de governança do ciclo.
- **Reexecução nesta passagem**: nenhum gate, teste, formatter, build ou comando git foi reexecutado ao escrever este release; evidência literal consolidada no R4 e na nota de governança do ciclo.
