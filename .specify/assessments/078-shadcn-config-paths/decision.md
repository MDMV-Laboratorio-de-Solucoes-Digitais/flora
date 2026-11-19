# Decision: caminhos de configuração do `components.json` (shadcn-svelte)

- **Slug**: 078-shadcn-config-paths · **Decidido**: 2026-11-19T20:21Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(2×5)/(1×1)=10,00 — candidato 078 eleito (GIN-JSONB 2,00; OTLP 1,67); leg RBAC bloqueada e Tauri/ADR dependem de maintainer.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | `config` deprecado em v4 e `css` inexistente no schema oficial (acessado 2026-11-19); CSS real em `layout.css:1` e `prettier.config.js:11` |
| Evidência | strong | Linhas coladas de `components.json:5-6`; schema/docs oficiais com URL e data; varredura determinística sem consumidores |
| Valor vs. inação | strong | Inação mantém contrato enganoso para futura geração de componentes via CLI |
| Viabilidade | strong | Opção A: 2 linhas em 1 arquivo, sem DB/rede, sem segredos, sem gate |
| Fit | strong | Alinhamento direto ao schema oficial (`tailwind.css`, `baseColor`) e ao toolchain Tailwind v4 local |
| Risco | strong | Zero consumidores locais; diff mínimo e reversível (SC-003/SC-004) |

## Handoff

Opção A (remover `tailwind.config` de `components.json:5`; `css` → `src/routes/layout.css` na linha 6); dentro = `apps/frontend/components.json` + 5 artefatos 078; fora = `style`/`baseColor`/`cssVariables`/`aliases`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, gate/scripts/workflows, ADRs, constituição, STATE, specs/078-* (exceto artefatos), DB/rede, segredos, fechamento de issues, leg RBAC, instalação do CLI. FR-001..FR-004 e SC-001..SC-004 de `define.md`. Perguntas carry-forward: `[NV]` validação pelo CLI `shadcn-svelte` (não instalado; exigiria rede) e observação de `style` deprecado / `cssVariables` não declarado no schema — ambos preservados por escopo mínimo, sem bloqueio.
