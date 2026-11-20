# Implementation Plan: 084 — Dedupe lucide

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/084-dedupe-lucide/{intake,research,define,concept,decision}.md` (go (scoped), opção A, E=2,50; pin `1.33.0` aprovado por Main, latest `1.76.0` como `[NV]`).
**Ordem**: T001 → T002 → T003 → T004 → T005. Paths explícitos, um commit por passo, nunca `git add -A`/`.`; mensagens em Conventional Commits pt-BR. Onde o gerenciador altera manifesto e lock na mesma operação atômica (`bun remove`, `bun add`), o commit inclui ambos os paths explicitamente.
**Base do diff**: `9da1aaea6` (HEAD da branch `feature/084-dedupe-lucide` no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001 |
| T002 | FR-002 | SC-001, SC-002 |
| T003 | FR-003 | SC-002 |
| T004 | FR-004 | SC-002 |
| T005 | FR-005 | SC-001, SC-002, SC-003, SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004, FR-005 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez. SC-001 reaparece em T002 (o `bun remove` também limpa `bun.lock`) e em T005 (auditoria final do grep); SC-002 reaparece em T003 (pin exato), T004 (árvore final) e T005 (auditoria do pin); SC-003 e SC-004 são verificados na auditoria T005.

## Passos

1. **T001** Em `apps/frontend`, trocar apenas a string da origem nos 3 imports: `src/lib/components/chat/ChatLayout.svelte:10` (`Send`, `Sparkles`), `src/lib/components/search/SearchBox.svelte:2` (`Search`) e `src/routes/(app)/+layout.svelte:8-19` (11 símbolos, fechando em `:19` com `} from '@lucide/svelte';`). Preservar os 14 símbolos, a sintaxe de named import, tabs, aspas simples e ausência de trailing comma (`prettier.config.js:5-7`); nenhum rename. Não tocar os 4 componentes UI com subpath `/icons/*`. Conferir `grep -rn "lucide-svelte" src` → zero e `grep -rn "from '@lucide/svelte'" src` → 3 linhas. Commitar somente os 3 arquivos com paths explícitos (`git add apps/frontend/src/lib/components/chat/ChatLayout.svelte apps/frontend/src/lib/components/search/SearchBox.svelte 'apps/frontend/src/routes/(app)/+layout.svelte'`). *(FR-001, SC-001)*

2. **T002** Em `apps/frontend`, executar `bun remove lucide-svelte`: remove `"lucide-svelte": "^1.0.1"` de `package.json:53` (bloco `dependencies`) e as entradas de `bun.lock:10` e `:517`. Verificar que `bits-ui:51`, `clsx:52` e `tailwind-merge:54` permanecem e que nenhuma outra dependência foi tocada. Commitar `apps/frontend/package.json` e `apps/frontend/bun.lock` (mutados juntos pelo comando) com paths explícitos. *(FR-002, SC-001, SC-002)*

3. **T003** Em `apps/frontend`, executar `bun add -D -E @lucide/svelte@1.33.0`: fixa `package.json:22` como `"@lucide/svelte": "1.33.0"` (sem `^`), mantém o pacote em `devDependencies` e `bun.lock:95` em `1.33.0` (`bun.lock:16` atualizado). Não usar `bun add` sem `-E` (reintroduziria `^` e permitiria `1.76.0`). Conferir `grep -n '"@lucide/svelte"' package.json` → `"@lucide/svelte": "1.33.0"`. Commitar `apps/frontend/package.json` e `apps/frontend/bun.lock` com paths explícitos. *(FR-003, SC-002)*

4. **T004** Em `apps/frontend`, executar `npm install --package-lock-only --ignore-scripts`: remove a entrada `node_modules/lucide-svelte` (`package-lock.json:3772-3781`), mantém `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) e preserva `"dev": true` (`:345`). Revisar o diff e descartar churn não relacionado (flags `dev`, integrities, ordem). **Não** rodar `npm install` sem as flags — mutaria `node_modules/` (fora de escopo). Commitar somente `apps/frontend/package-lock.json`. *(FR-004, SC-002)*

5. **T005** Auditar o resultado sem novas edições de código: reexecutar os gates determinísticos abaixo; exigir zero ocorrências de `lucide-svelte`, 3 imports no root novo com 14 símbolos, pin exato sem `^`, só `@lucide/svelte@1.33.0` resolvido, quality gates sem regressão, gate autoritativo PASS e `git diff --name-only` restrito aos paths do ciclo. Confirmar que os 4 componentes UI, `node_modules/`, gate/CI/scripts/workflows, ADRs, constituição, STATE, `specs/083-*`, DB/rede, segredos e PRs não foram tocados; nenhum `[NV]` promovido a fato. Preparar o rollback abaixo. Os artefatos do ciclo (5 de assessment + 6 documentos) são documentação e acompanham o diff em bloco à parte. *(FR-005, SC-001, SC-002, SC-003, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=9da1aaea6

# 1. pacote legado ausente em todos os paths — esperado: zero ocorrências
cd apps/frontend
grep -rn "lucide-svelte" src package.json bun.lock package-lock.json

# 2. 3 imports no root novo, 14 símbolos preservados — esperado: 3 linhas
grep -rn "from '@lucide/svelte'" src/lib/components/chat/ChatLayout.svelte \
  src/lib/components/search/SearchBox.svelte 'src/routes/(app)/+layout.svelte'

# 3. pin exato e árvore de dependências — esperado: "1.33.0" sem ^;
#    somente @lucide/svelte@1.33.0, nenhuma entrada lucide-svelte
grep -n '"@lucide/svelte"' package.json
bun pm ls
npm ls @lucide/svelte lucide-svelte

# 4. quality gates — esperado: sem regressão
bun run check && bun run lint && bun run test:unit -- --run && bun run build

# 5. integridade de gate (raiz) + escopo — esperado: PASS e só os paths do ciclo
cd ../..
python3 scripts/ci/check_gate_integrity.py
git diff --name-only "$BASE"..HEAD
```

O gate 1 verifica SC-001 (ausência do legado); o gate 2 verifica SC-001 (3 imports e símbolos); o gate 3 verifica SC-002; o gate 4 verifica SC-003; o gate 5 verifica SC-004 (e reforça SC-001/SC-002 na auditoria final de T005). O escopo esperado em `git diff --name-only` é: os 3 imports, `apps/frontend/package.json`, `apps/frontend/bun.lock`, `apps/frontend/package-lock.json`, os 5 artefatos `.specify/assessments/084-dedupe-lucide/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) e os 6 documentos `specs/084-dedupe-lucide/` (`spec.md`, `clarify.md`, `plan.md`, `checklists/requirements.md`, `checklist.md`, `tasks.md`); qualquer outro path invalida SC-004. Como os diretórios 084 podem estar ainda não rastreados no início do ciclo, a auditoria com `$BASE..HEAD` pressupõe que os artefatos foram adicionados com paths explícitos (nunca `git add -A`/`.`). Todos os comandos são determinísticos e não dependem de DB, segredos ou rede; sua execução real fica com o main agent e é `[NV]` nesta spec (marcadores em SC-002, SC-003 e SC-004 de `spec.md`).

## Rollback

Reverter os commits de T001..T004 ou restaurar diretamente os 6 paths rastreados ao base:

```
git checkout 9da1aaea6 -- \
  apps/frontend/src/lib/components/chat/ChatLayout.svelte \
  apps/frontend/src/lib/components/search/SearchBox.svelte \
  'apps/frontend/src/routes/(app)/+layout.svelte' \
  apps/frontend/package.json apps/frontend/bun.lock apps/frontend/package-lock.json
```

Isso devolve os 3 imports e os 3 manifestos/locks byte-idênticos ao base `9da1aaea6`, com `lucide-svelte@^1.0.1` em `dependencies`, `@lucide/svelte@^1.33.0` em `devDependencies` e as entradas originais dos lockfiles. `node_modules/` não é rastreado e está fora de escopo; a reconstrução da árvore instalada é assunto de ambiente (`[NV]`). Os artefatos do ciclo 084 (5 de assessment e 6 documentos) são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; nenhum outro path é tocado. Após a reversão, repetir os gates 1, 2, 3, 4 e 5 para confirmar o retorno ao estado base.
