# Decision: instalação pinada do CLI `shadcn-svelte`

- **Slug**: 079-install-shadcn-svelte · **Decidido**: 2026-11-19T20:57Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(3×5)/(2×1)=7,50 — valor 3 (toolchain local reproduzível), confiança 5 (registry + docs + estado local), esforço 2 (1 linha + lockfile), risco 1 (sem `init`, diff mínimo). Alternativas: leg RBAC bloqueada; Tauri/OTLP dependem de ADR/maintainer.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | Pedido explícito do usuário; `latest = 1.8.1` / MIT no registry (npm + bun, 2026-11-19); docs oficiais com URL e data |
| Evidência | strong | `grep` sem `shadcn` em `package.json`/`bun.lock`; 51 componentes + `cn` + `components.json` corrigido (078) colados das fontes locais |
| Valor vs. inação | strong | Inação mantém CLI efêmero (`bun x …@latest`), dependente de rede e sem rastreabilidade |
| Viabilidade | strong | Opção A: `bun add -D -E shadcn-svelte@1.8.1` em `apps/frontend`, sem DB/segredos |
| Fit | strong | devDependency é o lugar correto para ferramenta de geração; `components.json` já válido para uso futuro do CLI |
| Risco | strong | `init`/`add`/`apply` não executados ⇒ CSS e componentes intactos; diff restrito a 2 arquivos (SC-004) |

## Handoff

Opção A: em `apps/frontend`, `bun add -D -E shadcn-svelte@1.8.1`; dentro = `apps/frontend/package.json` + `apps/frontend/bun.lock` + 5 artefatos 079; fora = `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, scripts/CI/gate, ADRs, constituição, STATE, specs/079-* (exceto artefatos), DB/rede fora do registry, segredos, `.tokens/`, fechamento de issues, leg RBAC, `init`/`add`/`apply` e a tag `next`. FR-001..FR-005 e SC-001..SC-004 de `define.md`. Perguntas carry-forward: `[NV]` execução da instalação e dos comandos de verificação (`bun pm ls shadcn-svelte`, `bun run shadcn-svelte --help`, `bun install --frozen-lockfile --dry-run`, CI) — nenhuma bloqueia o go; risco de formatação do `package.json` mitigado por `prettier --check` (FR-004/SC-003).
