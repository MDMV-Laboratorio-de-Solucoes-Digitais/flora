# Analyze: 097 — Primeira configuração condicionada ao estado da instalação (READ-ONLY)

**Data**: 2026-11-20 · **Modo**: READ-ONLY · **Branch**: `feature/097-first-seed-once` @ `696849934` · **Baseline**: `696849934`
**Artefatos revisados**: `.specify/feature.json` (= `specs/097-first-seed-once`), `specs/097-first-seed-once/spec.md` (FR-001..FR-006, SC-001..SC-003), `plan.md`, `tasks.md` (T001..T010), `docs/governance/2026-11-21-097-first-seed-once.md` (diagnóstico e `go`) e a cadeia de bootstrap no repositório (`apps/frontend/src/routes/(auth)/+page.svelte`, `flora-api/src/routes/bootstrap.rs`).
**Escopo**: eliminar a oferta incondicional do fluxo "Primeira configuração" na tela de login — hoje "É uma instalação nova? Fazer primeira configuração" é renderizado sempre no branch `{:else}` (`+page.svelte:238-243`), embora a garantia de bootstrap único exista somente no backend, dentro de `POST /api/v1/bootstrap` (advisory lock + contagem de organizações não-sistema, `:28-29`, `:238-244`, `:287-291`, 409). A decisão expõe o estado real da instalação por uma rota de leitura e condiciona o link a esse estado, preservando o guardião 409 como defesa final. Fora de escopo: gate, required checks, constituição/ADRs, schema, PR/merge/tag, deploy e observação ao vivo de produção.

## Diagnóstico determinístico (confirmado por leitura direta no baseline)

- `apps/frontend/src/routes/(auth)/+page.svelte:238-243` — no branch `{:else}` da tela de login, o rodapé exibe "É uma instalação nova?" e o botão "Fazer primeira configuração" (`toggleSetup()` em `:174-177`) **sem consultar nenhum estado da instalação**; em workspace já configurado, `handleBootstrap()` (`:123-172`) recebe 409 e exibe "Este workspace já foi configurado." (`:140-143`). O frontend oferece uma ação que o backend necessariamente rejeita.
- `flora-api/src/routes/bootstrap.rs:28-29` — `BOOTSTRAP_LOCK_KEY` com `pg_advisory_xact_lock` na mesma transação; `:238-244` documenta 409 quando já inicializado; `:287-291` conta organizações não-sistema (`id <> '00000000-0000-0000-0000-000000000000'`) e devolve `Error::DuplicateEntry("bootstrap already completed")` quando `org_count > 0`. O guardião é correto e atômico — é a última linha de defesa.
- `flora-api/src/routes/bootstrap.rs:57-60` — `create_bootstrap_router()` registra somente `POST /bootstrap`; não existe endpoint/flag que informe ao cliente se a instalação está vazia. Sem esse sinal, o cliente não pode decidir se exibe ou oculta o link.
- `docs/governance/2026-11-21-097-first-seed-once.md` — registra o sintoma, a causa raiz, a decisão de condicionar a exibição ao estado real (zero organizações não-sistema), a manutenção do guardião 409 e o estado **[NV]** até deploy; `go` materializado. A tradução do relato "Is this a fresh install? Start First Seed Journey" é o par de strings pt-BR acima — o defeito é de exibição, não de idioma.

## Checagem determinística

```
artifacts=9  critical=0  requirements_covered=6/6  fr=6/6  sc=3/3  tasks=10/10  go=True  errors=[]
FR match= True ['FR-001', 'FR-002', 'FR-003', 'FR-004', 'FR-005', 'FR-006']
SC match= True ['SC-001', 'SC-002', 'SC-003']
tasks unique= True ['T001', 'T002', 'T003', 'T004', 'T005', 'T006', 'T007', 'T008', 'T009', 'T010']
```

`requirements_covered=6/6` contabiliza os 6 requisitos funcionais (FR-001..FR-006); SC-001..SC-003 são rastreados nas mesmas linhas. `critical=0`: nenhuma divergência de contrato entre a nota de governança, `spec.md`, `plan.md`, `tasks.md` e a cadeia de bootstrap revisada — o diagnóstico (link incondicional, único sinal de "fresh" confinado ao `POST /bootstrap`) e as quatro frentes de correção (rota de leitura, helper de contagem compartilhado, frontend condicional com fail-closed e testes dos três estados) são coerentes. `go=True` registrado na nota de governança. `analyze` não alterou código, migration, config, workflow, gate, ADR, constituição, `STATE.md`, `validate/` nem `release/`, não reexecutou gates, build, testes, formatters nem comandos git de mutação.

## Rastreabilidade — 6/6 requisitos cobertos, 10/10 tarefas

| # | Requisito (spec) | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — `GET /api/v1/bootstrap/status`, rota read-only e pública que responde `is_fresh = (contagem de organizações não-sistema == 0)`, sem efeito colateral e sem exigir autenticação (SC-001) | T001, T004 | `create_bootstrap_router()` hoje registra só `POST /bootstrap` (`:57-60`); T001 acrescenta o handler e a struct mínima de resposta, T004 registra a rota no mount `/api/v1` e confirma que CORS/allowlist e o guardião 409 permanecem inalterados |
| R2 | FR-002 — o cálculo de `is_fresh` reusa a **mesma** definição do guardião de `POST /bootstrap` (`COUNT(*) FROM organizations WHERE id <> sentinel`, na mesma semântica da transação de `:287-291`), eliminando duplicação e divergência (SC-002) | T002, T004 | o helper compartilhado é extraído de `:287-291` e chamado tanto pela rota de status quanto pelo `POST`; T004 verifica que o `BOOTSTRAP_LOCK_KEY` e o `Error::DuplicateEntry` seguem intactos — nenhuma chamada direta ou concorrente deixa de falhar com 409 |
| R3 | FR-003 — resposta mínima `{ "is_fresh": boolean }`, sem dados de tenant/PII; falha de banco devolve `500` sem detalhes e o cliente trata erro como "não fresh" (fail-closed) (SC-002, SC-003) | T001, T003 | a struct de resposta expõe só o booleano; T003 cobre o erro de DB com `500` sem vazar organização/usuário e verifica que nenhum campo extra é serializado |
| R4 | FR-004 — a tela de login consulta o status ao montar e renderiza "É uma instalação nova? Fazer primeira configuração" **somente** quando `is_fresh === true` (SC-001) | T005, T006, T008 | o bloco de `:238-243` passa a depender do estado carregado por `onMount` (T005) e da condição `isFresh` (T006); T008 confirma o contrato JSON entre endpoint e frontend e que `toggleSetup()`/`handleBootstrap()` seguem inalterados |
| R5 | FR-005 — em instalação configurada (`is_fresh === false`) ou se a consulta falhar/indefinida, o link permanece oculto e a tela segue apenas com Entrar/SSO; nenhuma regressão nos fluxos existentes (SC-001, SC-003) | T005, T006, T007 | o padrão fail-closed em T006 oculta o link em `false`, `undefined` e erro; T007 cobre os três estados da tela e preserva login local/SSO e a mensagem de 409 em chamada direta |
| R6 | FR-006 — nenhuma alteração em gate, required check, constituição, ADR, schema, PR/merge/tag; mudança restrita a rota de leitura, helper compartilhado, frontend de login e artefatos do ciclo, com testes cobrindo vazio/configurado/erro (SC-003) | T003, T007, T009, T010 | T003 e T007 cobrem os três estados; T009 materializa spec/plan/tasks/analyze/converge e T010 consolida gates, lockfile/supply-chain, revisão isolada e **[NV]** sem tocar em gate, required check, constituição, ADR ou schema |

SC-001 (link visível só em instalação vazia e fluxo de primeira configuração até `/tasks`) é coberta por R1/R4/R5; SC-002 (status idêntico ao critério do guardião 409, sem divergência e sem vazamento) por R2/R3/R5; SC-003 (nenhuma regressão, nenhum vazamento de tenant e nenhuma alteração de gate/required check/constituição/ADR/schema) por R3/R5/R6. T001/T004/T002 entregam a rota e o helper compartilhado; T003 os testes do endpoint; T005/T006/T007 o frontend condicional e fail-closed com seus testes; T008 o contrato de integração; T009 os artefatos do ciclo; T010 o fecho de gates/lockfile/supply-chain/revisão. O detalhe literal está consolidado em `.specify/validations/097-first-seed-once/validate.md`.

## [NV]

- **Estado real em produção**: se a instalação de destino está vazia ou configurada não foi medido nesta passagem; o comportamento do link após deploy permanece **[NV]** até o deploy final (SC-001).
- **E2E autenticado**: o fluxo bootstrap → sessão → `GET /api/v1/tasks`, incluindo o caso 409 preservado em instalação configurada, é evidência esperada do handoff — não executado por este agente, conforme o modo READ-ONLY.
- **CI/PR/produção**: nenhum PR, merge, tag, required check, aprovação de maintainer, proteção de branch ou ambiente de produção foi executado ou inspecionado ao vivo; gate local não substitui CI remota.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git de mutação não foram executados por este agente; a análise é somente leitura e a evidência vem da nota de governança do ciclo e da leitura direta do baseline.

## Resultado

**PASS — 6/6 requisitos cobertos, 3/3 critérios de sucesso, 10/10 tarefas, 0 críticos.** FR-001..FR-006, SC-001..SC-003 e T001..T010 (únicos e sequenciais) estão alinhados ao `go` da nota de governança e ao escopo determinístico de expor o estado de instalação por rota de leitura e condicionar o link "Fazer primeira configuração" a esse estado, mantendo o guardião 409 como defesa final. Nenhuma tarefa nova, nenhuma divergência de contrato e nenhuma edição de fonte nesta fase; `[NV]` preservado nos artefatos do ciclo. Analyze somente leitura — gates, build, testes e formatters não foram reexecutados aqui.
