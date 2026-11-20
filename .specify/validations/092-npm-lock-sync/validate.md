# Validation: 092 — npm lock sync

**Data**: 2026-11-20 · **Branch**: `feature/092-npm-lock-sync` · **Baseline**: `bf00caf5c`
**Tipo**: sincronização de lockfile — `apps/frontend/package-lock.json` regenerado a partir do manifesto com `npm install --package-lock-only --ignore-scripts`, preservando `package.json` e o lockfile canónico `bun.lock`. Nenhuma mudança de API, runtime, schema, migration, contrato de produção, workflow, required check, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| leitura de `apps/frontend/package-lock.json` | diff de +116 linhas contra `bf00caf5c`; raiz com `"shadcn-svelte": "1.8.1"` e entradas transitivas presentes (FR-001, SC-001) |
| leitura de `apps/frontend/package.json` | `devDependencies.shadcn-svelte = "1.8.1"` inalterado — manifesto fora do diff (FR-003, SC-002) |
| leitura de `apps/frontend/bun.lock` | intacto, canónico para CI; já continha `shadcn-svelte` (FR-003, SC-002) |
| `npm install --package-lock-only --ignore-scripts` em `apps/frontend` | `exit=0` — nenhum lifecycle script executado, nenhuma credencial exposta (FR-002, SC-003) |
| reexecução do comando contratual | zero alterações adicionais — idempotência confirmada (FR-005, SC-001) |
| `git diff --stat` / `git status --short` | `apps/frontend/package-lock.json \| 116 +` — único arquivo de código no diff; `package.json`, `bun.lock`, source e CI intactos (FR-004, SC-002) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — 10 legs / 58 pisos intactos (evidência esperada/consolidada do handoff) |
| frontend `npm run check`, `npm run lint`, `npm run build` | sem falhas — `exit=0` nas três verificações (evidência esperada/consolidada do handoff) |
| inventário de alerts (entrada) | 25 alertas do Dependabot no lockfile congelado; contagem viva pós-triagem `[NV]` (fora de escopo de remediação) |

### Saídas coladas

Evidência esperada/consolidada do handoff do ciclo:

```
$ cd apps/frontend && npm install --package-lock-only --ignore-scripts
exit=0  (nenhum lifecycle script, nenhuma credencial exibida)

$ git diff --stat -- apps/frontend/package-lock.json
 apps/frontend/package-lock.json | 116 ++++++++++++++++++++++++++++++++++++++++
 1 file changed, 116 insertions(+)

$ git diff -- apps/frontend/package-lock.json | head -12
@@ -32,6 +32,7 @@
 				"prettier": "^3.8.3",
 				"prettier-plugin-svelte": "^4.1.0",
 				"prettier-plugin-tailwindcss": "^0.8.0",
+				"shadcn-svelte": "1.8.1",
 				"svelte": "^5.56.1",
 				"svelte-check": "^4.6.0",
 				"tailwind-variants": "^3.3.1",

$ git status --short
 M apps/frontend/package-lock.json
?? specs/092-npm-lock-sync/

$ npm run check && npm run lint && npm run build
exit=0  (sem falhas nas três verificações do frontend)

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS
10 legs / 58 pisos intactos
exit=0

inventário de entrada: 25 alertas (7 high) em apps/frontend/package-lock.json
contagem viva pós-triagem: [NV]
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal **esperada e consolidada do handoff do main agent**: lock regenerado com `shadcn-svelte`, `package.json`/`bun.lock` intactos, comando contratual `exit=0` e idempotente, gate PASS e `check`/`lint`/`build` sem falhas. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `bf00caf5c`, a mudança de código fica restrita a um arquivo:

- `apps/frontend/package-lock.json` — entrada `shadcn-svelte@1.8.1` e dependências transitivas adicionadas; nenhuma outra entrada removida ou reescrita (FR-001, FR-002).
- `apps/frontend/package.json` e `apps/frontend/bun.lock` — inalterados; o lockfile bun permanece canónico da CI (FR-003).

Nada mais foi alterado: sem workflow, código-fonte, testes, config de produção, `STATE.md`, ADRs, constituição, PR/merge/deploy, `validate/` ou `release/` fora deste R4. Os artefatos do ciclo 092 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`) são registros documentais e não afetam o sistema.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; execução ao vivo do GitHub Actions, aprovações de maintainer, proteção de branch e ambiente de produção não verificados neste ciclo. O gate local PASS não substitui a CI remota.
- **Contagem viva de alerts após a triagem**: o inventário registrado é o de entrada (25 alertas, 7 high, no `package-lock.json` congelado); a contagem pós-regeneração/pós-triagem no painel do Dependabot não foi lida e permanece `[NV]`. A operação não resolve alertas individualmente nem os promove a gate.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido.
- **Escala da mudança**: 1 arquivo (lockfile npm); sem teste de carga, verificação visual de UI ou alteração de comportamento de runtime a validar além das verificações do frontend e do gate.

## Portarias

R1 `go` para a sincronização restrita ao lockfile npm, com decisão materializada em `.specify/assessments/092-npm-lock-sync/decision.md` (2026-11-20); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3` e `tasks=6/6` (`specs/092-npm-lock-sync/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.6 → 0.4.7`, changelog FR-001..FR-005 / T001..T006 e rollback restaurando o `apps/frontend/package-lock.json` anterior (sem `shadcn-svelte`), sem tocar `package.json`, `bun.lock`, CI ou DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais deste ciclo.
