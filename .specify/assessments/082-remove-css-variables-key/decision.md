# Decision: remover a chave `cssVariables` de `components.json`

- **Slug**: 082-remove-css-variables-key · **Decidido**: 2026-11-19T22:31Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(2×5)/(1×1)=5,0 — valor 2 (elimina chave inválida sob validação estrita do schema, sem mudar comportamento), confiança 5 (evidência local integral: linha colada, schema oficial com URL e data de acesso, parser Zod `$strip` do CLI instalado, varredura de consumidores zero), esforço 1 (1 linha removida, 1 vírgula ajustada, 1 arquivo), risco 1 (nenhum runtime/gate/CI lê o arquivo; o CLI já descarta a chave). Alternativas do ciclo: flag OIDC morta 15,0 (feita em 080) · `.env.example` OIDC 10,0 (feita em 081) · `cssVariables` 5,0 (esta) · lucide 2,50 · OTLP 1,25 (`.specify/memory/STATE.md:644`).

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | `cssVariables` ausente em todo o schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19; `tailwind.additionalProperties = false`); 078 a preservou como pendência de escopo mínimo |
| Evidência | strong | `apps/frontend/components.json:4-8` colado; `$strip` no CLI 1.8.1 (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`); varredura determinística sem consumidores; `prettier.config.js:5-7` |
| Valor vs. inação | strong | Inação mantém metadado que reprova validação estrita do schema em um arquivo que já foi parcialmente corrigido em 078 |
| Viabilidade | strong | Opção A: 1 linha removida + 1 vírgula em 1 arquivo; nenhuma dependência, gate, DB, rede ou segredo |
| Fit | strong | Alinhamento direto ao schema oficial sem criar campo novo; `style` permanece porque é declarado (deprecado em v4) |
| Risco | strong | CLI já descarta a chave (`$strip`); zero consumidores locais; diff mínimo e reversível (SC-003/SC-004) |

## Handoff

Opção A: em `apps/frontend/components.json`, remover a linha `:7` (`"cssVariables": true`) e trocar `"baseColor": "zinc",` por `"baseColor": "zinc"` na linha `:6`. Dentro = `apps/frontend/components.json` + 5 artefatos 082. Fora = `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/078-*, specs/082-* (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles. FR-001..FR-004 e SC-001..SC-004 de `define.md`.

Comandos determinísticos pós-edição (main agent — nenhum lê `cssVariables` em runtime):

1. `python3 -c` com `json.load` em `apps/frontend/components.json` — esperado: `set(obj["tailwind"]) == {"css","baseColor"}` → `True`; `cssVariables` ausente em todos os níveis.
2. `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` — esperado: nenhuma correspondência em `apps/frontend/components.json` nem em código/script/CI; só os registros históricos em `.specify/memory/STATE.md` e `specs/078-*`/`.specify/assessments/078-*`.
3. `cd apps/frontend && bunx prettier --check components.json` — esperado: exit 0 (tabs, aspas duplas, sem trailing comma — `prettier.config.js:5-7`).
4. `cd apps/frontend && bun run check && bun run lint && bun run test:unit && bun run build` — esperado: sem regressão (`package.json:8,11,13,15`).
5. `python3 scripts/ci/check_gate_integrity.py` — gate segue PASS; `git diff --name-only` — esperado: `apps/frontend/components.json` + 5 artefatos 082, nada mais; zero PRs.

`[NV]` não verificados nesta avaliação: execução dos comandos acima; validação online do arquivo editado contra `https://shadcn-svelte.com/schema.json` (exigiria rede + edição; evidência local `$strip` + schema colado já sustentam a decisão); consumidores externos do `components.json` (repos fora do checkout podem esperar `cssVariables`, mas nenhum leitor oficial depende dela — `schema-Bq2uExa4.d.mts:410-414`); CI em produção. Gates/formatters **skipped** nesta avaliação somente leitura; base `8f24519fe` preservada.
