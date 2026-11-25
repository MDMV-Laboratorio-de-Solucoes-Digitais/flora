# Decision: caminhos de configuração do `components.json` (shadcn-svelte)

- **Slug**: 078-shadcn-config-paths
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior) | docs/schema oficiais do shadcn-svelte (acessados 2026-11-19)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `config` deprecado em v4 e `css` inexistente no schema oficial (acessado 2026-11-19); CSS real em `layout.css:1` e `prettier.config.js:11` (`intake.md:9-11`; `research.md:8`). |
| Evidence strength | strong | Linhas coladas de `components.json:5-6`; schema/docs oficiais com URL e data; varredura determinística sem consumidores locais (`research.md:6-19`). |
| Value vs. inaction | strong | Inação mantém contrato enganoso para futura geração de componentes via CLI (`define.md:9`; `decisão anterior:11`). |
| Feasibility / appetite | strong | Opção A: 2 linhas em 1 arquivo, sem DB/rede, sem segredos e sem toque no gate — apetite trivial (`concept.md:5-8`). |
| Strategic fit | strong | Alinhamento direto ao schema oficial (`tailwind.css`, `baseColor`) e ao toolchain Tailwind v4 local (`@tailwindcss/vite` em `vite.config.ts:3,9`) (`research.md:11-17`). |
| Risk posture | strong | Zero consumidores locais; diff mínimo e reversível (SC-003/SC-004), nenhuma credencial envolvida (`define.md:22-25`; `concept.md:10`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. O candidato 078 foi eleito pelo 80/20 com `E=(2×5)/(1×1)=10,00`, acima de GIN-JSONB (2,00) e OTLP (1,67); a leg RBAC permanece bloqueada e Tauri/ADR dependem de maintainer (`intake.md:12`; `decisão anterior:5`). Opção A (remover a chave deprecada `config` em `components.json:5` e corrigir `css` para `src/routes/layout.css` na linha 6) é o recorte mínimo: `https://shadcn-svelte.com/schema.json` marca `tailwind.config` como "DEPRECATED IN TAILWIND v4!" e exige apenas `css`/`baseColor`, enquanto `https://shadcn-svelte.com/docs/components-json` define `tailwind.css` como o caminho do CSS que importa Tailwind e o arquivo como opcional ("only required if you're using the CLI") — ambos acessados em 2026-11-19. As duas fontes locais (`+layout.svelte:2`, `prettier.config.js:11`) convergentes e a varredura com zero consumidores eliminam risco de runtime; alvo `feature/078-shadcn-config-paths`, HEAD-base `de141fc53`, gate PASS, zero PRs. A opção B mantém metadado enganoso e a C cria arquivo morto contrariando o CSS-first do v4 (`concept.md:12-19`). `style` (deprecado em v4) e `cssVariables` (não declarado no schema) permanecem intactos por escolha de escopo mínimo; `[NV]` validação pelo CLI externo não instalado (`bunx shadcn-svelte@latest` exigiria rede). Sem mutação de DB/rede/segredos.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; os pontos abaixo são `[NV]` de escopo, não bloqueantes.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: `apps/frontend/components.json` descreve um setup inexistente — `config` aponta para `tailwind.config.ts` (deprecado em v4) e `css` para `src/app.css`, enquanto o CSS real é `src/routes/layout.css:1` (`intake.md:9-11`).
- **Chosen approach**: Opção A — deletar `"config": "tailwind.config.ts"` e trocar `"css": "src/app.css"` por `"css": "src/routes/layout.css"`, preservando byte-idênticos `$schema`, `style`, `baseColor: "zinc"`, `cssVariables: true` e todo o bloco `aliases`, com formatação Prettier local (tabs, sem trailing commas) (`define.md:13-17`; `concept.md:5-8`).
- **In scope / out of scope**: dentro = `apps/frontend/components.json` (1 linha removida + 1 valor trocado) + 5 artefatos `.specify/assessments/078-shadcn-config-paths/`; fora = `style`/`baseColor`/`cssVariables`/`aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows (`scripts/ci/check_gate_integrity.py`), supply-chain, ADRs, constituição, STATE, `specs/078-*` (exceto os 5 artefatos), DB/rede, segredos, fechamento de issues, leg RBAC e instalação do CLI. FR-001..FR-004 e SC-001..SC-004 de `define.md:13-25`.
- **Success metrics**: SC-001 sem chave `tailwind.config`; SC-002 `tailwind.css` aponta para arquivo existente com `@import 'tailwindcss'` e casa com `prettier.config.js:11`; SC-003 `json.load` válido, `bunx prettier --check components.json` limpo e `bun run check`/`lint`/`test:unit`/`check_gate_integrity.py` sem regressão; SC-004 diff restrito a `components.json` + 5 artefatos, gate PASS, zero mutação de DB/rede/segredos (`define.md:20-25`).
- **Carried-forward open questions**: `[NV]` validação pelo CLI externo `shadcn-svelte` (não instalado; exigiria rede); `[NV]` `style` deprecado em v4 e `cssVariables` não declarado no schema, ambos preservados por escopo mínimo — nenhuma bloqueia o `go` (`research.md:16-18`; `concept.md:23-25`).
