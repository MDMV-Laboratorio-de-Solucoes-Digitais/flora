# Implementation Plan: 082 — remover a chave `cssVariables` de `components.json`

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/082-remove-css-variables-key/{intake,research,define,concept,decision}.md` (go (scoped), opção A, E=5,0).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`. Convenção: Conventional Commits em pt-BR; o único arquivo de código/config alterado (`apps/frontend/components.json`) vai em um commit próprio e os documentos do ciclo podem ser commitados em bloco à parte.
**Base do diff**: `8f24519fe` (HEAD da branch `feature/082-remove-css-variables-key` no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-001 |
| T003 | FR-003 | SC-002, SC-004 |
| T004 | FR-004 | SC-003, SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez. SC-001 aparece em T001 e T002 porque o `json.load` só satisfaz a métrica após a chave sair (T001) **e** a vírgula ser ajustada (T002); SC-002 aparece em T001 (chave ausente) e T003 (byte-identidade dos demais campos); SC-004 aparece em T003 e T004 (preservação e auditoria).

## Passos

1. **T001** Em `apps/frontend/components.json`, remover a linha `:7` (`"cssVariables": true`) do objeto `tailwind`. A chave não existe em nenhum nível do schema oficial (`https://shadcn-svelte.com/schema.json`, acessado 2026-11-19: `tailwind.properties = {css, baseColor, config}`, `additionalProperties = false`) e é descartada por `$strip` no parser Zod do CLI instalado (`schema-Bq2uExa4.d.mts:410-414`). Conferir que nenhuma outra linha é tocada; neste ponto o JSON está temporariamente inválido por causa da vírgula em `:6`, que T002 ajusta. *(FR-001, SC-001, SC-002)*
2. **T002** Ajustar a linha `:6` de `"baseColor": "zinc",` para `"baseColor": "zinc"` (remover a vírgula que separava `baseColor` da chave agora excluída), preservando o valor `zinc`. Rodar o gate 1 e conferir: `set(obj["tailwind"]) == {"css","baseColor"}` → `True` e `cssVariables` ausente em todos os níveis; o gate 2 não retorna `components.json`. Commitar somente `apps/frontend/components.json` (`git add apps/frontend/components.json`; nunca `git add -A`/`.`). *(FR-002, SC-001)*
3. **T003** Verificar, sem editar, que nada além de `:6-7` mudou: `$schema` (`:2`), `style` (`:3`), `tailwind.css` (`:5`), o valor `baseColor` (`:6`) e todo o bloco `aliases` (`:9-14`) permanecem byte-idênticos ao base; formatação com tabs, aspas duplas e sem trailing comma (`prettier.config.js:5-7`). Não remover `style` (declarado no schema, apenas deprecado em v4); não adicionar `config`, alias, chave nova ou dependência. Rodar os gates 2 e 3, exigindo `grep` limpo para `components.json` e `prettier --check` com exit 0. Qualquer diff fora de `:6-7` invalida o passo. *(FR-003, SC-002, SC-004)*
4. **T004** Auditar o escopo com `git diff --name-only 8f24519fe..HEAD` e `git diff --stat 8f24519fe..HEAD`: somente `apps/frontend/components.json` (1 linha removida + 1 vírgula ajustada) e os artefatos do ciclo 082 (6 documentos em `specs/082-remove-css-variables-key/` e 5 de assessment em `.specify/assessments/082-remove-css-variables-key/`). Confirmar byte-idênticos os demais paths (`prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, `node_modules/`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, specs/078-*, DB, segredos, lockfiles). Rodar os gates 4 e 5, exigindo `bun run check`, `bun run lint`, `bun run test:unit -- --run` (o script `test:unit` é `vitest`, `package.json:15`) e `bun run build` sem regressão, com gate PASS; registrar os `[NV]` em `spec.md`/`clarify.md`, confirmar zero mutação de DB/segredos, zero PRs e validar a reversão do único path conforme a seção Rollback. *(FR-004, SC-003, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=8f24519fe

# 1. forma exata do objeto tailwind (raiz) — esperado: True; cssVariables ausente
python3 -c 'import json; o=json.load(open("apps/frontend/components.json")); print(set(o["tailwind"])=={"css","baseColor"})'
python3 -c 'import json; o=json.load(open("apps/frontend/components.json")); print("cssVariables" not in str(o))'

# 2. varredura de cssVariables (raiz) — esperado: nenhuma linha em components.json
#    nem em código/script/CI; só registros históricos em STATE/078
grep -rn "cssVariables" --exclude-dir=node_modules --exclude-dir=.git .

# 3. formatação (apps/frontend) — esperado: exit 0
(cd apps/frontend && bunx prettier --check components.json)

# 4. check/lint/test:unit -- --run/build (apps/frontend) — esperado: sem regressão
(cd apps/frontend && bun run check && bun run lint && bun run test:unit -- --run && bun run build)

# 5. integridade de gate e auditoria de escopo (raiz)
python3 scripts/ci/check_gate_integrity.py
git diff --name-only "$BASE"..HEAD
git diff --stat "$BASE"..HEAD
```

O gate 1 verifica diretamente SC-001; o gate 2 verifica SC-002; o gate 3 verifica SC-003 (formatação); os gates 4 e 5 verificam SC-003 e SC-004. Nenhum comando lê `cssVariables` em runtime. A auditoria de escopo espera somente `apps/frontend/components.json` e os artefatos do ciclo 082; qualquer outro path invalida SC-004. Como os diretórios 082 estão ainda não rastreados no início do ciclo, auditorias com `$BASE..HEAD` pressupõem que os artefatos foram adicionados com paths explícitos (nunca `git add -A`/`.`). Todos os comandos acima são determinísticos e não dependem de DB, segredos ou rede; sua execução real fica com o main agent após a edição e é `[NV]` nesta spec (marcadores em SC-001 e SC-003 de `spec.md`).

## Rollback

Reverter o commit que introduziu T001/T002 ou restaurar diretamente o único path ao base:

```
git checkout 8f24519fe -- apps/frontend/components.json
```

Isso devolve `"baseColor": "zinc",` em `:6` e `"cssVariables": true` em `:7`, restaurando `apps/frontend/components.json` byte-idêntico ao base. Nenhum `prettier.config.js`, `package.json`, `vite.config.ts`, `layout.css`, `+layout.svelte`, doc, gate/script, workflow, ADR, constituição, STATE, spec numerada, DB, rede ou segredo é modificado. Os artefatos do ciclo 082 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; após a reversão, repetir os gates 1, 2, 3, 4 e 5 para confirmar o retorno ao estado base.
