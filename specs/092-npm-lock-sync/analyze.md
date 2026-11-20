# Analyze: 092 — npm lock sync (READ-ONLY)

**Data**: 2026-11-20 · **Modo**: READ-ONLY · **Branch**: `feature/092-npm-lock-sync` @ `bf00caf5c`
**Artefatos revisados**: `.specify/feature.json` (= `specs/092-npm-lock-sync`), `.specify/assessments/092-npm-lock-sync/decision.md` (verdict `go`), `spec.md` (FR-001..FR-005, SC-001..SC-003), `plan.md`, `tasks.md` (T001..T006), `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json` (diff contra `bf00caf5c`).
**Escopo**: regeneração de `apps/frontend/package-lock.json` com `npm install --package-lock-only --ignore-scripts` em `apps/frontend`, preservando o manifesto e o lockfile canónico do CI (`bun.lock`). Nenhuma mudança de API, runtime, schema, migration, contrato de produção, workflow, required check, PR/merge/deploy. O diff de código fica restrito ao lockfile npm; os demais arquivos do ciclo são registros documentais.

## Checagem determinística

```
artifacts=8  critical=0  requirements_covered=5/5  fr=5/5  sc=3/3  tasks=6/6  go=True  errors=[]
FR match= True ['FR-001', 'FR-002', 'FR-003', 'FR-004', 'FR-005']
SC match= True ['SC-001', 'SC-002', 'SC-003']
tasks unique= True ['T001', 'T002', 'T003', 'T004', 'T005', 'T006']
```

`requirements_covered=5/5` contabiliza os 5 requisitos funcionais (FR-001..FR-005); SC-001..SC-003 são rastreados nas mesmas linhas. `critical=0`: nenhuma divergência entre decisão, spec, plan, tasks e a fonte observada; nenhuma correção de fonte necessária. `go=True` registrado em `decision.md`. `analyze` não alterou dependências, código, workflow, `validate/` nem `release/` e não reexecutou gates, build, testes ou formatters.

## Rastreabilidade — 5/5 requisitos cobertos, 6/6 tarefas

| # | Requisito (spec) | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — `apps/frontend/package-lock.json` DEVE refletir `package.json` e o canónico `bun.lock`, incluindo `shadcn-svelte` (SC-001) | T001, T004 | diff contra `bf00caf5c`: +116 linhas no lock, raiz passa a conter `"shadcn-svelte": "1.8.1"` e entradas transitivas; `bun.lock` já continha a dependência (2 ocorrências), manifesto já a declarava |
| R2 | FR-002 — sincronização DEVE ser executada em `apps/frontend` com `npm install --package-lock-only --ignore-scripts` (SC-003) | T003 | comando contratual em `decision.md`/`tasks.md`/`plan.md`, executado no diretório do frontend; `exit=0` e nenhum lifecycle script disparado (evidência consolidada do handoff) |
| R3 | FR-003 — NÃO modificar `apps/frontend/package.json` nem `apps/frontend/bun.lock` (SC-002) | T001, T005 | `git status --short` contra `bf00caf5c` lista apenas `M apps/frontend/package-lock.json` (mais os artefatos documentais do ciclo); `package.json` e `bun.lock` fora do diff |
| R4 | FR-004 — NÃO executar lifecycle scripts NEM alterar source, CI, `validate`, `release` ou `STATE`; escopo exclusivo do lockfile npm (SC-002, SC-003) | T003, T005 | `--package-lock-only --ignore-scripts`; `node_modules`, workflows, código-fonte, `.specify/memory/STATE.md`, `validate/` e `release/` intocados pelo diff de código |
| R5 | FR-005 — sincronização idempotente e evidência registrada em decisão, spec, plano e tarefas (SC-001) | T002, T004, T006 | `decision.md`, `spec.md`, `plan.md` e `tasks.md` registram drift, canonicidade do `bun.lock`, 25 alertas e o comando; reexecução do comando produz zero diffs adicionais |

SC-001 (presença de `shadcn-svelte`, sem drift de primeira ordem e idempotência) é coberta por R1/R5; SC-002 (diff exclusivo do lock) por R3/R4; SC-003 (`exit=0`, sem lifecycle scripts nem credenciais) por R2/R4. O detalhe literal está consolidado em `.specify/validations/092-npm-lock-sync/validate.md`.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, merge ou deploy realizado/verificado neste snapshot; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo.
- **Contagem viva de alerts após a triagem**: o inventário registrado é o de entrada — 25 alertas do Dependabot no `package-lock.json` congelado. A contagem pós-regeneração/pós-triagem no painel do Dependabot não foi lida nesta passagem e permanece `[NV]`.
- **Reexecução nesta passagem**: gates, build, testes, formatters e comandos git não foram executados por este agente; `analyze` é somente leitura e a evidência de verificação vem do handoff do main agent.

## Resultado

**PASS — 5/5 requisitos cobertos, 6/6 tarefas, 0 críticos.** FR-001..FR-005, SC-001..SC-003 e T001..T006 (únicos e sequenciais) estão alinhados ao `go` de `decision.md` e ao escopo determinístico da sincronização. Nenhuma tarefa nova, nenhuma divergência e nenhuma edição de fonte nesta fase; `[NV]` e `go` preservados nos artefatos do ciclo. Analyze somente leitura — gates, build, testes e formatters não foram reexecutados aqui.
