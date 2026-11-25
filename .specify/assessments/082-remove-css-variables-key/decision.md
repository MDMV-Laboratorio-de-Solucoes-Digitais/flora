# Decision: remover a chave `cssVariables` de `components.json`

- **Slug**: 082-remove-css-variables-key
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior, decidido em 2026-11-19T22:31Z) | schema oficial shadcn-svelte e CLI 1.8.1 citados em research.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `cssVariables` ausente em todo o schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19; `tailwind.additionalProperties = false`); 078 preservou a chave como pendência de escopo mínimo (`intake.md:4`). |
| Evidence strength | strong | `components.json:4-8` colado, parser Zod `$strip` no CLI 1.8.1 (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`) e varredura determinística sem consumidores (`research.md:7-13`). |
| Value vs. inaction | strong | Inação mantém metadado inválido sob validação estrita em arquivo já parcialmente corrigido em 078 (`define.md:7`). |
| Feasibility / appetite | strong | Opção A: 1 linha removida + 1 vírgula ajustada em `apps/frontend/components.json`; sem dependência, gate, DB, rede ou segredo (`concept.md:5-9`). |
| Strategic fit | strong | Alinhamento direto ao schema oficial sem criar campo novo; `style` permanece porque é declarado (apenas deprecado em v4), conforme 078. |
| Risk posture | strong | CLI já descarta a chave (`$strip`), zero consumidores locais, diff mínimo e reversível (SC-003/SC-004). `[NV]` validação online do arquivo editado, consumidores externos e CI. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; decisão original em 2026-11-19T22:31Z, branch `feature/082-remove-css-variables-key`, base `8f24519fe`, gate PASS, zero PRs. Sucessor direto de 078/079 com `shadcn-svelte@1.8.1` instalado, o que torna o `$strip` evidência local verificável. 80/20 original integral: **E=(2×5)/(1×1)=5,0** — valor 2 (elimina chave inválida sob validação estrita do schema, sem mudar comportamento), confiança 5 (evidência local integral: linha colada, schema oficial com URL e data de acesso, parser Zod do CLI instalado, varredura de consumidores zero), esforço 1 (1 linha removida, 1 vírgula ajustada, 1 arquivo), risco 1 (nenhum runtime/gate/CI lê o arquivo; o CLI já descarta a chave). Alternativas do ciclo: flag OIDC morta 15,0 (080) · `.env.example` OIDC 10,0 (081) · `cssVariables` 5,0 (esta) · lucide 2,50 · OTLP 1,25 (`.specify/memory/STATE.md:644`). Opções B (confiar no `$strip`) e C (migrar para campo equivalente) rejeitadas em `concept.md:11-19`: não existe campo equivalente no schema e o custo da correção é uma vírgula. Atenção de implementação: remover a vírgula de `:6` junto com `:7`, sob pena de JSON inválido e `trailingComma: 'none'` (`prettier.config.js:7`).

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; nenhuma pergunta bloqueante nos artefatos).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `apps/frontend/components.json:7` declara `"cssVariables": true`, chave inexistente no schema oficial (`additionalProperties = false`) e silenciosamente descartada pelo CLI.
- **Chosen approach**: Opção A — remover `:7` e trocar `"baseColor": "zinc",` por `"baseColor": "zinc"` em `:6`; manter `$schema`, `style`, `css`, `baseColor` e `aliases` byte-idênticos. FR-001..FR-004 e SC-001..SC-004 de `define.md`.
- **In scope / out of scope**:
  - **In**: `apps/frontend/components.json` e os 5 artefatos 082.
  - **Out**: `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/078-*, specs/082-* (exceto artefatos), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles.
- **Success metrics**: `json.load` → `set(obj["tailwind"]) == {"css","baseColor"}` e zero `cssVariables`; `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` sem correspondência em código/CI (restam só registros históricos em `STATE.md`/`specs/078-*`/`.specify/assessments/078-*`); `bunx prettier --check components.json` exit 0; `bun run check && bun run lint && bun run test:unit && bun run build` sem regressão; gate PASS; diff só o arquivo + 5 artefatos.
- **Carried-forward open questions**: `[NV]` execução dos comandos acima; `[NV]` validação online do arquivo editado contra o schema (evidência local já sustenta a decisão); `[NV]` consumidores externos do `components.json`; `[NV]` CI em produção. Gates/formatters **skipped** nesta avaliação somente leitura; base `8f24519fe` preservada.
