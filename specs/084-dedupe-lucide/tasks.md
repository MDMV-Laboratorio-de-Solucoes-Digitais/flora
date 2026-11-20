# Tasks: 084 — deduplicação de dependências lucide (`lucide-svelte` → `@lucide/svelte`)

- **T001** Migrar os 3 imports legados para o pacote novo, preservando os 14 símbolos e a sintaxe de named import: em `apps/frontend/src/lib/components/chat/ChatLayout.svelte:10`, trocar `'lucide-svelte'` por `'@lucide/svelte'` mantendo `Send` e `Sparkles`; em `apps/frontend/src/lib/components/search/SearchBox.svelte:2`, manter `Search`; em `apps/frontend/src/routes/(app)/+layout.svelte:8-19`, manter os 11 símbolos (com `Sparkles` repetido). Nenhum rename: o root de `@lucide/svelte@1.33.0` exporta todos os nomes (`dist/icons/index.js`, `dist/aliases/aliases.js:751` para `CheckSquare`). Não tocar os 4 componentes UI já migrados com subpath `/icons/*` (`dialog-content.svelte`, `dropdown-menu-checkbox-item.svelte`, `dropdown-menu-radio-item.svelte`, `dropdown-menu-sub-trigger.svelte`) — permanecem byte-idênticos. Manter tabs e aspas simples sem trailing comma (`prettier.config.js:5-7`). Conferir que os 14 símbolos seguem presentes após a troca. *(FR-001, SC-001)*

- **T002** Remover a dependência deprecada de `apps/frontend/package.json:53` com `bun remove lucide-svelte` (executado em `apps/frontend`), eliminando `"lucide-svelte": "^1.0.1"` do bloco `dependencies` com reflexo em `bun.lock:10` e `:517`. Preservar `bits-ui:51`, `clsx:52` e `tailwind-merge:54`. Conferir que nenhuma outra dependência de `dependencies` mudou e que `grep -rn "lucide-svelte" src package.json bun.lock` retorna zero após este passo (a auditoria final em T005 repete o grep em todo o escopo). *(FR-002, SC-001, SC-002)*

- **T003** Fixar o pacote novo em devDependencies com `bun add -D -E @lucide/svelte@1.33.0` (em `apps/frontend`), convertendo `package.json:22` de `"^1.33.0"` para `"1.33.0"` exato (sem `^`), com reflexo em `bun.lock:16` e `:95`. O pacote permanece em devDependencies; o `"dev": true` de `package-lock.json:345` é preservado. A versão contratual `1.33.0` foi aprovada por Main neste ciclo; o latest `1.76.0` informado pelo registry (2026-11-19T23:47Z) fica como `[NV]`, sem upgrade não avaliado. Conferir `grep -n '"@lucide/svelte"' package.json` e `bun pm ls`. *(FR-003, SC-002)*

- **T004** Sincronizar `apps/frontend/package-lock.json` com `npm install --package-lock-only --ignore-scripts` (em `apps/frontend`): remover a entrada `node_modules/lucide-svelte` (`:3772-3781`), manter `node_modules/@lucide/svelte` em `1.33.0` (`:341-348`) com `"dev": true` (`:345`) e preservar o restante do arquivo. Revisar o `git diff` do lockfile e descartar churn não relacionado: ausência de `node_modules/lucide-svelte`, `@lucide/svelte` em `1.33.0`, zero mudanças alheias. Conferir `npm ls @lucide/svelte lucide-svelte` (apenas `1.33.0`, sem legado). *(FR-004, SC-002)*

- **T005** Auditar a fronteira e preparar o rollback: reexecutar os greps de SC-001/SC-002 e exigir `git diff --name-only` com somente os 3 imports + `apps/frontend/package.json` + `bun.lock` + `package-lock.json` + os 5 artefatos `.specify/assessments/084-dedupe-lucide/` (`intake.md`, `research.md`, `define.md`, `concept.md`, `decision.md`) + os documentos do próprio ciclo em `specs/084-dedupe-lucide/`. Zero mutação de gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, `specs/083-*`, DB/rede, segredos, `node_modules/`; zero PRs abertos; base `9da1aaea6` preservada. Rodar os gates determinísticos da seção abaixo: `bun run check`, `bun run lint`, `bun run test:unit -- --run`, `bun run build`, `prettier --check` e `python3 scripts/ci/check_gate_integrity.py` (PASS), sem regressão. Registrar como `[NV]` a regressão visual em browser, o grafo final do bundler e o estado ao vivo de CI/PR — os 13 `iconNode`s comparados são idênticos, mas gates não detectam aparência. Não invocar `/speckit.validate` nem `/speckit.release`. Preparar o rollback descrito ao fim. *(FR-005, SC-001, SC-002, SC-003, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001 |
| T002 | FR-002 | SC-001, SC-002 |
| T003 | FR-003 | SC-002 |
| T004 | FR-004 | SC-002 |
| T005 | FR-005 | SC-001, SC-002, SC-003, SC-004 |

SC-001 aparece em T001, T002 e T005: os 14 símbolos são conferidos na troca (T001), o grep do pacote legado só zera após a remoção da dependência (T002) e é reexecutado na auditoria final (T005). SC-002 aparece em T002, T003, T004 e T005: ausência do legado e pin exato são verificados em cada mutação de manifesto/lock e reconferidos no fim. SC-003 e SC-004 aparecem em T005: gates, fronteira de paths, base preservada e zero PRs são exigidos na auditoria. IDs sempre individuais, sem intervalos (R4/R7).

## Gates determinísticos (locais/offline; `[NV]` até a implementação)

```
# 0. rodar em apps/frontend salvo o gate (item 4)
cd apps/frontend

# 1. legado removido + 3 imports no pacote novo (esperado: zero 'lucide-svelte')
grep -rn "lucide-svelte" src package.json bun.lock package-lock.json
grep -rn "@lucide/svelte" src/lib/components/chat/ChatLayout.svelte src/lib/components/search/SearchBox.svelte "src/routes/(app)/+layout.svelte"

# 2. pin exato + árvore de dependências (esperado: "1.33.0" sem ^; só @lucide/svelte@1.33.0)
grep -n '"@lucide/svelte"' package.json
bun pm ls
npm ls @lucide/svelte lucide-svelte

# 3. check/lint/test/build + formatação (esperado: sem regressão; prettier limpo)
bun run check && bun run lint && bun run test:unit -- --run && bun run build
bunx prettier --check package.json "src/**/*.svelte"

# 4. integridade de gate e escopo (na raiz; esperado: PASS e só os paths permitidos)
python3 scripts/ci/check_gate_integrity.py
git diff --name-only

# 5. revisão dos locks (esperado: sem node_modules/lucide-svelte; @lucide/svelte em 1.33.0; zero churn)
git diff -- package-lock.json bun.lock
```

Os comandos acima são determinísticos e executados pelo main agent após a edição; sua execução real, a regressão visual em browser, o grafo final do bundler, o churn real dos lockfiles e o estado ao vivo de CI/PR são **[NV]** nesta spec (sem gates/formatters/git aqui), conforme `decision.md`. A auditoria espera somente os 3 imports, os 3 manifestos/locks e os artefatos/documentos 084; qualquer outro path invalida SC-004. Como os diretórios 084 podem estar ainda não rastreados no início do ciclo, auditorias com `$BASE..HEAD` pressupõem que os artefatos foram adicionados com paths explícitos (nunca `git add -A`/`.`).

## Rollback

Reverter T001..T005 restaurando os 3 imports ao estado do base `9da1aaea6` (`'lucide-svelte'`), `"@lucide/svelte": "^1.33.0"` em `package.json:22`, `"lucide-svelte": "^1.0.1"` em `:53` e os dois lockfiles ao conteúdo do base (`bun.lock:10,16,95,517`; `package-lock.json:19,341-348,3772-3781`). Os 4 componentes UI com `/icons/*` não são tocados. Os artefatos de assessment e de ciclo 084 podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

## Convenção de commit

Um arquivo por commit, com paths explícitos nos comandos e Conventional Commits em pt-BR; nunca `git add -A`/`.`. Mutações atômicas de `bun remove`, `bun add` e `npm install --package-lock-only` agrupam `package.json` e o lockfile correspondente no mesmo commit, sempre com `git add <path> <path>` explícito. Exemplos de mensagens: `refactor(frontend): troca imports lucide para @lucide/svelte`, `chore(frontend): remove dependência deprecada lucide-svelte`, `chore(frontend): fixa @lucide/svelte em 1.33.0`, `chore(frontend): sincroniza package-lock.json`, `docs(084): adiciona artefatos de especificação`. Sem `/speckit.validate` nem `/speckit.release`.
