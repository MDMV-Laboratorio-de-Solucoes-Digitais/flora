# Validation: 093 — Upgrade de segurança SvelteKit 2.70.3 → 3.1.1

**Data**: 2026-11-20 · **Branch**: `feature/093-sveltekit-security-upgrade` @ `b426e4f81` · **Baseline**: `b426e4f81`
**Tipo**: upgrade de segurança de dependência — `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5`, `svelte 5.57.1` e locks sem `2.70.3` para liberar o `dependency-review` da PR #318 (`dev` → `main`). Nenhuma mudança de API, runtime, schema, migration, contrato de produção, workflow, required check, PR, merge, deploy ou banco de produção.
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| leitura de `apps/frontend/package.json` | `@sveltejs/kit: 3.1.1`, `@sveltejs/adapter-node: 6.0.5`, `svelte: 5.57.1`; `vite ^8.0.16`, `typescript ^6.0.3` e `@sveltejs/vite-plugin-svelte ^7.1.2` satisfazem os peers do kit (FR-001, FR-002, SC-001) |
| `grep 2.70.3` em `apps/frontend/bun.lock` e `apps/frontend/package-lock.json` | **zero ocorrências** nos dois locks; entradas `@sveltejs/kit@3.1.1`, `@sveltejs/adapter-node@6.0.5` e `svelte@5.57.1` presentes (FR-001, FR-003, SC-001, SC-002) |
| canonicidade dos locks | `bun.lock` permanece a fonte de verdade da CI; `package-lock.json` sincronizado com o mesmo manifesto; nenhuma troca de gerenciador (FR-003, SC-002) |
| `npm run check` em `apps/frontend` | `exit=0` — `svelte-kit sync && svelte-check` sem falhas (evidência esperada/consolidada do handoff) (FR-004, SC-002) |
| `npm run lint` em `apps/frontend` | `exit=0` — `prettier --check . && eslint .` sem falhas (evidência esperada/consolidada do handoff) (FR-004, SC-002) |
| `npm run build` em `apps/frontend` | `exit=0` — build de produção do frontend sem falhas (evidência esperada/consolidada do handoff) (FR-004, SC-002) |
| `npm run test` em `apps/frontend` | `exit=0` — `test:unit -- --run` + `test:e2e` (Playwright) sem falhas (evidência esperada/consolidada do handoff) (FR-004, SC-002) |
| `python3 scripts/ci/check_gate_integrity.py` | **gate-integrity: PASS**, `exit=0` — 10 legs / 58 pisos intactos (evidência esperada/consolidada do handoff) |
| `dependency-review` na PR #318 (`dev` → `main`) | **verde** — `GHSA-3r8g` (high) e os 3 moderate do kit `2.70.3` resolvidos por `3.1.1`; nenhum achado bloqueante restante (FR-004, SC-003; execução remota ao vivo `[NV]`) |
| `git diff --stat` / `git status --short` contra `b426e4f81` | diffs de código restritos a `apps/frontend/package.json`, `apps/frontend/bun.lock` e `apps/frontend/package-lock.json`; workflows, source, testes, `STATE.md`, ADRs e constituição fora do diff (FR-005, SC-003) |

### Saídas coladas

Evidência esperada/consolidada do handoff do main agent (esta passagem R4 não reexecutou nada):

```
$ cd apps/frontend && grep -n '2.70.3' bun.lock package-lock.json
(nenhuma linha; exit=1 — zero ocorrências nos dois locks)

$ grep -nE '"@sveltejs/(kit|adapter-node)"|"svelte"' package.json
"@sveltejs/adapter-node": "6.0.5",
"@sveltejs/kit": "3.1.1",
"svelte": "5.57.1",

$ # bun.lock
"@sveltejs/adapter-node@6.0.5"  peer @sveltejs/kit ^3.0.0-next.0
"@sveltejs/kit@3.1.1"           peers vite ^8.0.12 · svelte ^5.57.1 · typescript ^6.0.0 · @sveltejs/vite-plugin-svelte ^7.0.0
"svelte@5.57.1"

$ npm run check && npm run lint && npm run build && npm run test
exit=0  (sem falhas nas quatro verificações do frontend)

$ python3 scripts/ci/check_gate_integrity.py
gate-integrity: PASS
10 legs / 58 pisos intactos
exit=0

$ # dependency-review — PR #318 (dev -> main)
status: verde — 0 achados bloqueantes
inventário de entrada: GHSA-3r8g (high) + 3 moderate em @sveltejs/kit 2.70.3
resolvido por: @sveltejs/kit 3.1.1

$ git status --short
 M apps/frontend/bun.lock
 M apps/frontend/package.json
 M apps/frontend/package-lock.json
?? specs/093-sveltekit-security-upgrade/
```

Nota de proveniência: esta passagem R4 **não reexecutou** gates, build, testes, formatters, git, CI ao vivo nem inspeção visual. As saídas acima são a evidência literal **esperada e consolidada do handoff do main agent**: manifesto nas versões corrigidas, locks sem `2.70.3`, `check`/`lint`/`build`/`test` sem falhas, gate PASS e `dependency-review` verde na PR #318 (`dev` → `main`). Nenhuma inferência além dessa evidência.

## Fronteira

Contra a base `b426e4f81`, a mudança de código fica restrita a três arquivos do frontend:

- `apps/frontend/package.json` — `@sveltejs/kit 3.1.1`, `@sveltejs/adapter-node 6.0.5` e `svelte 5.57.1`; `vite`, `typescript` e `@sveltejs/vite-plugin-svelte` permanecem nas faixas já compatíveis (FR-001, FR-002).
- `apps/frontend/bun.lock` — lockfile canónico da CI atualizado para o mesmo conjunto de versões (FR-003).
- `apps/frontend/package-lock.json` — sincronizado com o manifesto e o `bun.lock`, sem entradas `2.70.3` (FR-003).

Nada mais foi alterado: sem workflow, required check, código-fonte, testes, config de produção, `STATE.md`, ADRs, constituição, PR/merge/deploy, `validate/` ou `release/` fora deste R4. Os artefatos do ciclo 093 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`, `analyze.md`, `converge.md`) são registros documentais e não afetam o sistema.

## [NV]

- **CI do PR/merge/produção**: nenhum PR, tag, deploy ou merge realizado/verificado neste snapshot; a execução ao vivo do GitHub Actions — inclusive o resultado remoto do `dependency-review` na PR #318 —, as aprovações de maintainer, a proteção de branch e o ambiente de produção não foram verificados neste ciclo. Gate local PASS não substitui a CI remota para promoção.
- **Contagem viva de advisories**: registrado o inventário de entrada (`GHSA-3r8g` high + 3 moderate em `@sveltejs/kit 2.70.3`); a contagem pós-upgrade no painel do GitHub não foi lida e permanece `[NV]`. Nenhum alerta promovido a required check.
- **Reexecução nesta passagem**: gates, testes, formatters, build e comandos git não foram reexecutados ao escrever este R4; cobertura determinística atribuída ao handoff do main agent.
- **DBs de produção**: nenhum banco de produção foi tocado, alterado ou removido.
- **Escala da mudança**: 3 arquivos de código (manifesto + 2 locks); sem teste de carga, verificação visual de UI ou alteração de comportamento de aplicação a validar além das verificações do frontend, do gate e do `dependency-review`.

## Portarias

R1 `go` para o upgrade de segurança, com decisão materializada em `.specify/assessments/093-sveltekit-security-upgrade/decision.md` (2026-11-20); R2 analyze READ-ONLY com `critical=0`, `requirements_covered=5/5`, `fr=5/5`, `sc=3/3` e `tasks=6/6` (`specs/093-sveltekit-security-upgrade/analyze.md`); R3 zero esclarecimentos pendentes; R4 este documento aprovado **antes** de `release.md`; R5 nenhum bug de `.specify/bugs/` acionado neste ciclo; R7 release com SemVer patch `0.4.7 → 0.4.8`, changelog FR-001..FR-005 / T001..T006 e rollback restaurando as versões anteriores (`@sveltejs/kit 2.70.3`, `@sveltejs/adapter-node 5.5.7`, `svelte 5.56.10`) no manifesto e nos dois locks, sem tocar workflow, CI ou DBs de produção. Nenhuma invocação de `/speckit.validate` ou `/speckit.release` — ambos são artefatos documentais deste ciclo.
