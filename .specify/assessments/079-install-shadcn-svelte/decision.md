# Decision: instalação pinada do CLI `shadcn-svelte`

- **Slug**: 079-install-shadcn-svelte
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior) | registry npm/bun e docs oficiais do shadcn-svelte (acessados 2026-11-19)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Pedido explícito do usuário; `latest = 1.8.1` / MIT no registry (npm + bun, 2026-11-19); docs oficiais com URL e data (`intake.md:8-12`; `research.md:21-26`). |
| Evidence strength | strong | `grep` sem `shadcn` em `package.json`/`bun.lock`; 51 componentes + `cn` + `components.json` corrigido (078) colados das fontes locais (`research.md:8-16`). |
| Value vs. inaction | strong | Inação mantém CLI efêmero (`bun x …@latest`), dependente de rede e sem rastreabilidade no lockfile (`define.md:9`; `decisão anterior:11`). |
| Feasibility / appetite | strong | Opção A: `bun add -D -E shadcn-svelte@1.8.1` em `apps/frontend`, sem DB/segredos — apetite trivial (`concept.md:5-8`). |
| Strategic fit | strong | devDependency é o lugar correto para ferramenta de geração; `components.json` já válido para uso futuro do CLI (`concept.md:8`; `research.md:10`). |
| Risk posture | strong | `init`/`add`/`apply` não executados ⇒ CSS e componentes intactos; diff restrito a 2 arquivos (SC-004) (`research.md:28-31`; `define.md:24-25`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. O candidato 079 foi eleito pelo 80/20 com `E=(3×5)/(2×1)=7,50` — valor 3 (toolchain local reproduzível), confiança 5 (registry + docs + estado local), esforço 2 (1 linha + lockfile), risco 1 (sem `init`, diff mínimo); alternativas: leg RBAC bloqueada e Tauri/OTLP dependem de ADR/maintainer (`intake.md:13`; `decisão anterior:5`). O pedido explícito do usuário é autorização para instalar; registry (`npm view shadcn-svelte@1.8.1`, `bun info`, 2026-11-19) confirma `dist-tags.latest = 1.8.1`, `next = 1.0.0-next.19`, licença MIT, bin `shadcn-svelte` → `./dist/index.mjs`, 4 dependências (`cn ^0.3.0`, `postcss ^8.5.10`, `commander ^14.0.0`, `node-fetch-native ^1.6.4`), publicação em 2026-11-11T21:25Z, mantenedor `huntabyte`, repo `github.com/huntabyte/shadcn-svelte` (`packages/cli`), fonte canônica `https://registry.npmjs.org/shadcn-svelte`. `https://shadcn-svelte.com/docs/installation/sveltekit`, `/docs/cli` e `/docs/components-json` (acessados 2026-11-19) descrevem `init` como inicialização que sobrescreve CSS global, recria `components.json` e reinstala componentes — por isso `init`/`add`/`apply` são deliberadamente não executados, preservando os 51 `.svelte` em `src/lib/components/ui/`, `src/lib/utils.ts` (`cn`) e o `components.json` já corrigido no ciclo 078. A opção B (caret) perde reprodutibilidade e a C mantém o estado que o usuário pediu para mudar (`concept.md:15-21`). Alvo `feature/079-install-shadcn-svelte`, HEAD-base `ea5cd2069`, gate PASS, zero PRs. Sem mutação de DB e sem segredos; único acesso de rede é o registry autorizado.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; a verificação pós-instalação é `[NV]`, não bloqueante.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: O frontend consome componentes e contratos shadcn-svelte, mas o CLI que os gera/valida não é dependência declarada em `apps/frontend/package.json` nem em `bun.lock`, forçando `bun x …@latest` (rede e versão flutuante) (`intake.md:10`; `define.md:9`).
- **Chosen approach**: Opção A — em `apps/frontend`, `bun add -D -E shadcn-svelte@1.8.1` (exato, sem `^`/`~`), commitar `bun.lock` e não executar `init`/`add`/`apply`; manter formatação conforme `prettier.config.js:5-8` (tabs, sem trailing commas, aspas duplas em JSON) (`define.md:13-19`; `concept.md:5-8`).
- **In scope / out of scope**: dentro = `apps/frontend/package.json` (1 linha adicionada) + `apps/frontend/bun.lock` (pacote + 4 transitivas `cn`, `postcss`, `commander`, `node-fetch-native`) + 5 artefatos `.specify/assessments/079-install-shadcn-svelte/`; fora = `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/CI/gate (`scripts/ci/check_gate_integrity.py`), `.github/workflows/`, supply-chain, ADRs, constituição, STATE, `specs/079-*` (exceto os 5 artefatos), DB/rede fora do registry, segredos, `.tokens/`, fechamento de issues, leg RBAC, upgrade de versão, `init`/`add`/`apply` e a tag `next`. FR-001..FR-005 e SC-001..SC-004 de `define.md:13-28`.
- **Success metrics**: SC-001 `bun pm ls shadcn-svelte` reporta `1.8.1` sem caret; SC-002 `bun run shadcn-svelte --help` lista `init`/`add`/`apply`/`registry build` sem `bun x`; SC-003 `bun install --frozen-lockfile --dry-run` sem diff de lockfile e `bun run lint`/`check` sem regressão; SC-004 `git diff --stat` só `package.json` + `bun.lock` + 5 artefatos, gate PASS, `components.json` e 51 componentes byte-idênticos ao base `ea5cd2069`, zero mutação de DB/segredos (`define.md:20-28`).
- **Carried-forward open questions**: `[NV]` execução da instalação e dos comandos de verificação (`bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run`, CI) — nenhuma bloqueia o `go`; risco de formatação do `package.json` mitigado por `prettier --check` (FR-004/SC-003) (`research.md:28-31`; `decisão anterior:20`).
