# Validation: 091 — Auditoria viva de governança do CI

**Data**: 2026-11-20 · **Branch**: `feature/091-governance-ci-audit` @ `673c687ea` · **Baseline**: `673c687ea`
**Tipo**: auditoria viva, documental e não mutante da governança do CI após os ciclos 086–090 — emenda constitucional v1.4.0 (ADR-0003, CI green basta) e workflow de auditoria advisory; nenhum required check novo, nenhuma credencial exposta, nenhuma mudança de runtime, gate, PR, merge, deploy ou produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `python3 scripts/ci/check_gate_integrity.py --remote` | **PASS** — `dev: live required contexts match`; `main: live required contexts match` (FR-001, SC-001) |
| leitura da proteção de branch ao vivo (`gh` + PAT do SE) | 4 required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) com *strict* e `enforce_admins`, sem required reviews; secret `FLORA_BRANCH_PROTECTION_TOKEN` registrado, nenhum valor exibido (FR-001) |
| `.github/workflows/branch-protection-audit.yml` | YAML válido — `schedule` (`cron: '23 4 * * *'`) + `workflow_dispatch`; job `branch-protection-audit` **não required**; `GH_TOKEN` = secret registrado, nunca em URL/argumento/log (FR-001, FR-005) |
| `.specify/memory/constitution.md` | **v1.4.0** — `Last Amended: 2026-11-20`, aprovada SE; emenda 2026-11-20 à ADR-0003 vigente (FR-002, SC-002) |
| deploy em produção | **`[NV]`** — somente app Staging existe no Coolify; sem app de produção/FQDN/revisão (FR-003, SC-003) |
| `apps/frontend/package-lock.json` | 25 alerts (7 high) — recomendação: **não promover a gate**; triar/limpar (FR-005) |
| scripts SDD da factory (4) | fora do gate / omitidos em relatórios — decisão 089 (FR-005, SC-005) |

### Saídas coladas

Evidência literal consolidada do handoff do main agent (R4 não reexecutou nada):

```
$ python3 scripts/ci/check_gate_integrity.py --remote
dev: live required contexts match
main: live required contexts match
gate-integrity: PASS
exit=0

$ # .github/workflows/branch-protection-audit.yml
workflow YAML válido: on.schedule(cron='23 4 * * *') + on.workflow_dispatch
job 'branch-protection-audit': advisory (não required)
secret FLORA_BRANCH_PROTECTION_TOKEN: registrado no GitHub (somente GH_TOKEN no passo de auditoria)

$ # .specify/memory/constitution.md
**Version**: 1.4.0 | **Ratified**: 2026-08-17 | **Last Amended**: 2026-11-20
Emenda 2026-11-20 à ADR-0003 aprovada SE — "CI/CD GREEN basta para merge autônomo em dev/"

$ # deploy produção — [NV]
Coolify: somente app Staging presente; sem app de produção / FQDN / revisão

$ # apps/frontend/package-lock.json
25 alerts (7 high) — recomendação: não promover a gate; triar/limpar

$ # scripts SDD da factory — fora do gate / omitidos
check_sdd_completeness.py · check_converge_append_only.py
check_task_id_uniqueness.py · check_work_order_dag.py
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal **consolidada do handoff do main agent**: `--remote` PASS com os dois contextos vivos alinhados, workflow válido e advisory, constituição v1.4.0 aprovada SE, deploy produção `[NV]` (só Staging), 25 alerts (7 high) não promovidos e scripts SDD omitidos. Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `673c687ea`, o delta entregue é governamental/documental:

- `.specify/memory/constitution.md` — emenda v1.3.0 → **v1.4.0** (ADR-0003, CI green basta; Svelte 5 canônico; PGMQ/OTLP/Tauri/mobile conforme ADR-0001/0002), aprovada SE.
- `.github/workflows/branch-protection-audit.yml` — novo, **advisory**, não required; consulta diária dos contextos required via `check_gate_integrity.py --remote` com o secret registrado.
- Artefatos do ciclo 091 — `decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`, este R4 e o release R7: registros documentais, sem efeito de runtime.

Nada mais alterado: sem runtime, gate autoritativo, required check novo, aprovador humano, ADRs além da emenda registrada, PR, merge, tag, deploy, produção, DBs ou valor de credencial. O secret **não é revertido** (permanece registrado).

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; execução ao vivo do GitHub Actions, aprovações de maintainer e ambiente de produção não verificados além da leitura `--remote` do handoff. Gate local PASS não substitui a CI remota para promoção.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **Deploy em produção**: `[NV]` — somente app Staging no Coolify; sem app de produção/FQDN/revisão para inspecionar; reconsultar antes de qualquer promoção, rollback ou novo deploy.
- **Alertas de dependência**: 25 alerts (7 high) em `package-lock.json` registrados como recomendação de triagem/limpeza, **não** como gate; nenhum promovido a required check.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido.

## Portarias

R1 `go` para a auditoria viva e a emenda governamental, com decisão materializada em `.specify/assessments/091-governance-ci-audit/decision.md` (2026-11-20); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `tasks=8/8` (`specs/091-governance-ci-audit/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.5 → 0.4.6`, changelog FR-001..FR-005 / T001..T008, rollback revertendo a constituição para v1.3.0 e removendo o workflow de auditoria — **sem reverter o secret** — e nenhum DB de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais deste ciclo.
