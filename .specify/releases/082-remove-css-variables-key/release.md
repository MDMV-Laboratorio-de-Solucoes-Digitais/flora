# Release: 082 — remoção da chave `cssVariables` de `components.json`

**Ciclo**: 082 · **Data**: 2026-11-19T22:54Z · **Branch**: `feature/082-remove-css-variables-key` · **Baseline**: `8f24519fe`
**Tipo**: correção de metadado de configuração (remoção da chave `cssVariables`, inexistente no schema oficial do shadcn-svelte, e ajuste da vírgula em `baseColor`), sem mudança de comportamento, API ou contrato de leitura. **Nenhuma tag, release, deploy, merge ou produção.**

## Entregue

- `apps/frontend/components.json` — `+1/−2` em 1 path: linha `:7` (`"cssVariables": true`) removida do objeto `tailwind` e vírgula em `:6` ajustada de `"baseColor": "zinc",` para `"baseColor": "zinc"`. Resultado: `tailwind = {css, baseColor}`, JSON válido, sem trailing comma; `json.load` confirma `set(obj["tailwind"]) == {"css","baseColor"}` e ausência de `cssVariables` em todos os níveis.
- `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor `zinc` de `baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`) permanecem byte-idênticos ao base. **`style` mantido**: declarado no schema oficial (apenas deprecado em v4), não removido.
- Compatibilidade por ausência, sem shim: o CLI `shadcn-svelte@1.8.1` já descartava a chave via parser Zod `$strip` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`); nenhum leitor local ou de runtime depende de `cssVariables`.
- Artefatos completos: assessment (5), spec, clarify, plan, checklists/tasks, analyze, converge append-only, validate R4, release R7.

## SemVer

**Proposta:** patch `0.3.2` → `0.3.3` (correção de metadado em arquivo de configuração, sem mudança de API, comportamento de runtime ou dependência ativa; a branch anterior 081 entregou `0.3.1` → `0.3.2`). Registro opcional a critério do maintainer. Nenhuma tag criada.

## Changelog (FR / SC / T)

- `FR-001` linha `:7` (`"cssVariables": true`) removida — chave ausente em todo o schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `required = ["css","baseColor"]`, `additionalProperties = false`) e já descartada por `$strip` no CLI 1.8.1; `FR-002` vírgula de `:6` ajustada (`"baseColor": "zinc",` → `"baseColor": "zinc"`), valor `zinc` preservado, sem trailing comma (`prettier.config.js:7`); `FR-003` `$schema`, `style`, `tailwind.css`, valor de `baseColor` e `aliases` byte-idênticos, formatação Prettier preservada (tabs, aspas duplas em JSON, sem trailing comma) e `style` mantido por ser declarado no schema (só deprecado em v4); `FR-004` diff rastreado limitado a `apps/frontend/components.json` (`+1/−2`) + artefatos 082, zero segredos, dependências novas ou mudanças em gate/scripts/workflows/lockfiles.
- `SC-001` `json.load` → `set(obj["tailwind"]) == {"css","baseColor"}` é `True`, `cssVariables` ausente em todos os níveis; `SC-002` varredura por `cssVariables` sem correspondência em `components.json`, código, scripts ou CI — só os registros históricos em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*`, com `$schema`/`style`/`baseColor`/`css`/`aliases` byte-idênticos; `SC-003` `bunx prettier --check components.json`, `bun run check`, `bun run lint`, `bun run test:unit -- --run` (`42/42`) e `bun run build` verdes, `python3 scripts/ci/check_gate_integrity.py` **PASS**, nenhum comando lendo `cssVariables` em runtime; `SC-004` `git diff --name-only` mostra só `components.json` + artefatos 082, zero mutação de gate/CI/scripts/workflows/DB/rede/segredos e zero PRs.
- `T001`..`T004` executados; verificador isolado `CssVerifier` **PASS** sem findings; `analyze` `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}`; decisão `E=(2×5)/(1×1)=5,0` (`go (scoped)`, opção A). Nenhum bug de `.specify/bugs/` acionado; leg RBAC permanece bloqueada por maintainer (fora do ciclo).

## Evidência

`.specify/validations/082-remove-css-variables-key/validate.md` · `specs/082-remove-css-variables-key/converge.md` · `specs/082-remove-css-variables-key/analyze.md` · `.specify/assessments/082-remove-css-variables-key/decision.md`.

## Rollback

Reverter, em `dev/`, exclusivamente `apps/frontend/components.json` ao estado do base `8f24519fe` — restaurando `"baseColor": "zinc",` (`:6`, com vírgula) e a linha `"cssVariables": true` (`:7`) no objeto `tailwind` — com:

```
git checkout 8f24519fe -- apps/frontend/components.json
```

Novo commit de reversão, sem force-push ou reescrita de histórico. É um único arquivo de configuração; nenhum outro path é tocado: `style`, `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos do ciclo 082 são documentação e podem ser revertidos em bloco.

## [NV]

Validação online do arquivo editado contra o schema oficial (exigiria rede + edição); consumidores externos do `components.json` e deploy fora do repo que ainda espera `cssVariables`; CI ao vivo, Forgejo, aprovações, merge, tag e produção; leg RBAC/maintainer. Nenhuma inferência além da evidência local consolidada no ciclo.
