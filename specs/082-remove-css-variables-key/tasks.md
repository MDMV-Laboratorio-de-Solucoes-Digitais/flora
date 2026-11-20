# Tasks: 082 — remoção da chave `cssVariables` de `components.json`

- **T001** Em `apps/frontend/components.json`, remover a linha `:7` (`"cssVariables": true`) do objeto `tailwind`. A chave não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `tailwind.additionalProperties = false`, `required = ["css","baseColor"]`) e já é descartada em silêncio pelo parser Zod `$strip` do CLI instalado `shadcn-svelte@1.8.1` (`schema-Bq2uExa4.d.mts:407-414`, `index-Ch0UJ9Cm.d.mts:13-16`); zero consumidores locais. A remoção isolada deixa trailing comma em `:6` e JSON inválido — concluir T002 antes de verificar SC-001; a remoção da chave já zera o `grep` de SC-002. *(FR-001, SC-001, SC-002)*
- **T002** Ajustar a linha `apps/frontend/components.json:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula que separava `baseColor` da chave agora excluída), preservando o valor `zinc`. Com T001 e T002 concluídos, `python3 -c` com `json.load` retorna `set(obj["tailwind"]) == {"css", "baseColor"}` → `True`, `cssVariables` está ausente em todos os níveis e o arquivo segue sem trailing comma (`prettier.config.js:7`). *(FR-002, SC-001)*
- **T003** Preservar a fronteira: manter byte-idênticos `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor de `tailwind.baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`), com a formatação do Prettier local (tabs em `prettier.config.js:5`, aspas duplas obrigatórias em JSON, nenhuma trailing comma em `:7`). **Não remover `style`**: ele está declarado no schema (`schema-Bq2uExa4.d.mts:409`; `properties.style` online) e apenas deprecado em v4. Ao fim, `grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .` não retorna `apps/frontend/components.json` nem qualquer arquivo de código/script/CI — apenas os registros históricos em `.specify/memory/STATE.md:626,644` e `specs/078-*`/`.specify/assessments/078-*`. *(FR-003, SC-002, SC-004)*
- **T004** Auditar o diff e preparar o rollback: `git diff --name-only` deve listar somente `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada), os 5 artefatos de assessment (`.specify/assessments/082-remove-css-variables-key/`: `intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os 6 docs do ciclo (`specs/082-remove-css-variables-key/`: `spec.md`, `clarify.md`, `plan.md`, `tasks.md`, `checklist.md`, `checklists/requirements.md`); `cd apps/frontend && bunx prettier --check components.json` termina com exit 0; `bun run check`, `bun run lint`, `bun run test:unit -- --run` e `bun run build` (`package.json:8,11,13,15`; `test:unit` é `vitest`, `package.json:15`, que exige `--run` para execução única) seguem sem regressão; `python3 scripts/ci/check_gate_integrity.py` mantém gate PASS. Zero mutação de `style`, `baseColor` (valor), `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, `specs/078-*`/`.specify/assessments/078-*`, `specs/082-*` (exceto os 6 docs do ciclo), DB/rede, segredos, `.tokens/`, fechamento de issues, leg RBAC e lockfiles; base `8f24519fe` preservada; zero PRs abertos. Registrar `[NV]` para validação online, gates pós-edição e consumidores externos. *(FR-004, SC-003, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-001 |
| T003 | FR-003 | SC-002, SC-004 |
| T004 | FR-004 | SC-003, SC-004 |

SC-001 aparece em T001 e T002 porque o `json.load` só passa após remover a chave (T001, `:7`) **e** a vírgula (T002, `:6`); removendo apenas a linha, o JSON fica inválido por trailing comma. SC-002 aparece em T001 (a remoção da chave zera o `grep`) e em T003 (preservação byte-idêntica de `$schema`/`style`/`baseColor`/`css`/`aliases`); SC-004 aparece em T003 e T004 (fronteira de paths e auditoria de escopo). IDs sempre individuais, sem intervalos (R4/R7).

## Gates determinísticos (locais/offline; `[NV]` até a implementação)

```
# 1. objeto tailwind — esperado: set == {"css", "baseColor"} -> True;
#    cssVariables ausente em todos os níveis
python3 -c 'import json; o=json.load(open("apps/frontend/components.json")); print(set(o["tailwind"]) == {"css","baseColor"})'

# 2. varredura — esperado: nenhuma correspondência em components.json,
#    código, scripts ou CI; só registros históricos em STATE.md e 078-*
grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .

# 3. formatação (tabs, aspas duplas, sem trailing comma)
cd apps/frontend && bunx prettier --check components.json

# 4. sem regressão (nenhum comando lê cssVariables em runtime)
cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run && bun run build

# 5. integridade de gate, escopo (base 8f24519fe) e zero PRs
python3 scripts/ci/check_gate_integrity.py
git diff --name-only
```

A auditoria espera somente `apps/frontend/components.json` e os 5 artefatos 082 (`.specify/assessments/082-remove-css-variables-key/`); qualquer outro path invalida SC-004. Os gates são executados pelo main agent; `[NV]` nesta avaliação.

Rollback: reverter `apps/frontend/components.json` ao estado do base `8f24519fe` (restaurando a linha `:7` `"cssVariables": true` e a vírgula em `:6`); nenhum outro path é tocado — `style`, `$schema`, `aliases`, `tailwind.css`, `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/scripts, workflows, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos 082 são documentação do próprio ciclo e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos nos comandos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
