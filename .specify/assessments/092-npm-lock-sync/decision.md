# Decision: Sincronização do lockfile npm (ciclo 092)

- **Slug**: 092-npm-lock-sync
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (anterior, decidido em 2026-11-20) | specs/092-{spec,plan,tasks,analyze,converge}.md | docs/governance/2026-11-20-092-package-lock-sync.md | STATE.md/PROGRESSO.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `bun.lock` é canônico para CI, mas `package-lock.json` estava congelado sem `shadcn-svelte` embora `package.json:40` o declarasse; drift medido e 25 alertas no inventário de entrada (`decision.md:12-17`, `research.md:9-11`). |
| Evidence strength | strong | Leitura direta dos locks neste worktree (`package-lock.json:36,4065`; `bun.lock:33,531`), diff `+116/−0` registrado contra `bf00caf5c`, SHA-256 do lock e `analyze` com `critical=0`, `5/5`, `3/3`, `6/6`, `errors=[]` (`research.md:29-34`). |
| Value vs. inaction | adequate | Elimina divergência e ruído do artefato derivado; consumidores concretos do `package-lock.json` não foram identificados (`research.md:11,42`), então parte do valor é documental. |
| Feasibility / appetite | strong | Um único comando contratual (`npm install --package-lock-only --ignore-scripts`), idempotente no registro local, sem instalação, lifecycle scripts ou mudança de gerenciador (`concept.md:9-13`). |
| Strategic fit | strong | Preserva `bun.lock` como fonte canônica da CI, `package.json` intacto e a fronteira de governança (`validate/`, `release/`, `STATE`), sem promover advisories a required check. |
| Risk posture | adequate | Reversível por restauro via VCS e diff confinado a um arquivo; `[NV]` idempotência independente (sem acesso ao registry), contagem viva de alertas, consumidores do lock, CI/PR/merge/produção e instalação real com `--ignore-scripts`. |

## Verdict & Rationale

Veredito histórico **`go` preservado sem alteração**; decisão original em 2026-11-20, branch `feature/092-npm-lock-sync` @ `bf00caf5c`, âncora `.specify/feature.json` → `specs/092-npm-lock-sync`. O scorecard converge com a decisão anterior: problema medido, correção de um único comando já contratado e resultado reversível; a CI continua ancorada em `bun.lock`. Compromissos rastreáveis originais preservados: **FR-001** conformidade de `package-lock.json` com `package.json`/`bun.lock` incluindo `shadcn-svelte`; **FR-002** comando contratual `npm install --package-lock-only --ignore-scripts` em `apps/frontend`, nenhum outro mecanismo autorizado; **FR-003** não modificar `package.json` nem `bun.lock`; **FR-004** sem lifecycle scripts e sem toque em source, CI, `validate/`, `release/` ou `STATE`; **FR-005** sincronização idempotente com registro em decisão, spec, plano e tarefas. Evidência de entrada: manifesto declara `"shadcn-svelte": "1.8.1"`, `bun.lock` contém a entrada, `package-lock.json` não continha; inventário de 25 alertas do Dependabot; diff contra `bf00caf5c` = **+116 linhas em `apps/frontend/package-lock.json`** com entrada raiz e transitivas, nenhum outro arquivo de código. SHA-256 registrado do lock: `68bd731c7e84b6e81e91a2d00b3a05c13500014fa7388a08d8b7e28a17790a28` (`docs/governance/2026-11-20-092-package-lock-sync.md:22`); recontagem `npm audit --package-lock-only --json` = 21 advisories (5 high, 15 moderate, 1 low, 0 critical) com `auditReportVersion 2` — fonte distinta do painel do Dependabot, sem comparação estrita, registrada como contexto, não remediação. Opções B (ajuste manual pontual) e C (eliminar artefato/migrar CI para npm) rejeitadas em `concept.md:15-25`: a primeira não garante coerência do grafo nem idempotência, a segunda troca reparo reversível por mudança de infraestrutura contrária à política registrada.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; as perguntas abaixo são carried-forward, não bloqueantes para a sincronização registrada).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `package-lock.json` derivado estava congelado e divergente de `package.json`/`bun.lock`, sem `shadcn-svelte 1.8.1`, com falsas expectativas para consumidores não identificados.
- **Chosen approach**: Opção A — regeneração determinística e restrita ao lockfile npm via `npm install --package-lock-only --ignore-scripts`; requisitos FR-001..FR-005 / SC-001..SC-003 e tarefas T001..T006 (`specs/092-npm-lock-sync/`).
- **In scope / out of scope**:
  - **In**: `apps/frontend/package-lock.json` como única alteração de código, com entrada raiz `"shadcn-svelte": "1.8.1"` e transitivas (`@emnapi/*`, `cn`, `commander`, `node-fetch-native`, `node_modules/shadcn-svelte`), mais decisão/spec/plano/tarefas e nota de governança.
  - **Out**: `package.json`, `bun.lock`, source, workflows, troca de gerenciador na CI, lifecycle scripts, `node_modules/`, remediação individual dos 25 alertas, promoção de advisories a required check, `validate/`, `release/`, `.specify/memory/STATE.md`, rotação de credenciais, PR/merge/tag/deploy/produção.
- **Success metrics**: lock npm reflete manifesto e lock canônico (`package-lock.json:36,4065`); diff de código confinado ao lock (`+116/−0` em 1 arquivo, baseline `bf00caf5c`); regeneração idempotente (registro local); `analyze` com `critical=0`, `requirements_covered=5/5`, `sc=3/3`, `tasks=6/6`, `errors=[]`; CI segue ancorada em `bun.lock`.
- **Carried-forward open questions**: `[NEEDS CLARIFICATION: qual política impede que o lockfile npm volte a divergir do manifesto e do bun.lock após esta sincronização?]`; `[NEEDS CLARIFICATION: qual é a contagem viva de alertas do Dependabot após a triagem, dado que só o inventário de entrada foi registrado ([NV])?]`; `[NEEDS CLARIFICATION: existem consumidores do package-lock.json que dependam da regeneração e quais são eles?]`; `[NEEDS CLARIFICATION: como confirmar a idempotência do comando contratual em um ambiente sem acesso ao registry?]`; `[NEEDS CLARIFICATION: qual é a expectativa de tratamento posterior para os 25 alertas registrados — contexto apenas ou triagem futura?]`; `[NEEDS CLARIFICATION: há requisito de evidência da sincronização fora dos artefatos documentais do ciclo?]` (`intake.md:23-28`, `research.md:46-51`). `[NV]` contagem viva de alertas e CI do PR/merge/produção — nenhum PR, merge ou deploy realizado/verificado na decisão original; host do registry não allowlisted nesta avaliação.
