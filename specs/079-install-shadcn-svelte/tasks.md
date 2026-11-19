# Tasks: 079 — instalação pinada do CLI `shadcn-svelte`

- **T001** Instalar o CLI em `apps/frontend`: executar `bun add -D -E shadcn-svelte@1.8.1` nesse diretório, declarando `"shadcn-svelte": "1.8.1"` (exato, sem `^` e sem `~`) em `devDependencies` de `apps/frontend/package.json` (`latest = 1.8.1` no registry em 2026-11-19). Verificar com `bun pm ls shadcn-svelte` e `grep` por `shadcn-svelte@1.8.1` sem caret, e com `bun run shadcn-svelte --help` (lista `init`, `add`, `apply`, `registry build`; bin local, sem `bun x`). *(FR-001, SC-001, SC-002)*
- **T002** Registrar a resolução em `apps/frontend/bun.lock`: entrada do pacote + 4 deps transitivas `cn`, `postcss`, `commander`, `node-fetch-native`, com lockfile rastreado e coerente com `package.json`. Confirmar com `bun install --frozen-lockfile --dry-run` em `apps/frontend`, sem diff de lockfile. *(FR-002, SC-003)*
- **T003** Preservar os artefatos já corretos do ciclo 078: `$schema`, `style`, `tailwind` e `aliases` de `apps/frontend/components.json:1-15`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/` e `src/lib/utils.ts` byte-idênticos ao base `ea5cd2069`. **Não** executar `init`, `add` nem `apply` (docs oficiais: `init` sobrescreve CSS global, recria `components.json` e reinstala componentes). *(FR-003, SC-004)*
- **T004** Manter a formatação e a pureza do diff: `package.json` com tabs, sem trailing commas e aspas duplas em JSON (`prettier.config.js:5-8`); nenhum segredo, `.tokens/`, mudança em gate/scripts/workflows ou mutação de DB/rede fora do registry. Verificar com `bun run lint` (inclui `prettier --check .`) e `bun run check` em `apps/frontend` e com `python3 scripts/ci/check_gate_integrity.py` na raiz, sem regressão. *(FR-004, SC-003)*
- **T005** Auditar a fronteira e preparar o rollback: `git diff --name-only` e `git diff --stat` contra o base `ea5cd2069` devem mostrar somente `apps/frontend/package.json` (1 linha adicionada) e `apps/frontend/bun.lock`, além dos 5 artefatos 079; `node_modules/` permanece ignorado; gate do alvo PASS; zero mutação de DB/segredos. Registrar os `[NV]` da execução/gates/CI em `spec.md`/`clarify.md`. *(FR-005, SC-004)*

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-003 |
| T003 | FR-003 | SC-004 |
| T004 | FR-004 | SC-003 |
| T005 | FR-005 | SC-004 |

## Gates determinísticos (portarias SDD — locais/offline, exceto o registry para resolver o pacote)

```
# 1. declaração exata e versão instalada (em apps/frontend)
cd apps/frontend
bun add -D -E shadcn-svelte@1.8.1
bun pm ls shadcn-svelte
grep -n '"shadcn-svelte": "1.8.1"' package.json
bun run shadcn-svelte --help

# 2. lockfile congelado sem diff (em apps/frontend)
bun install --frozen-lockfile --dry-run

# 3. formatação e tipos sem regressão (em apps/frontend)
bun run lint
bun run check

# 4. integridade de gate (raiz, sem DB/rede)
cd -
python3 scripts/ci/check_gate_integrity.py

# 5. auditoria de escopo (base ea5cd2069)
git diff --name-only ea5cd2069
git diff --stat ea5cd2069
```

A auditoria espera somente `apps/frontend/package.json` e `apps/frontend/bun.lock` (mais os 5 artefatos de `specs/079-install-shadcn-svelte/` e os artefatos de assessment já registrados em `.specify/assessments/079-install-shadcn-svelte/`); qualquer outro path invalida SC-004. `bun run test:unit` e `bun run build` pertencem ao plano de verificação do orchestrator, não a esta portaria SDD. A execução da instalação e dos gates é `[NV]` até a implementação.

Rollback: reverter `apps/frontend/package.json` e `apps/frontend/bun.lock` ao estado do base `ea5cd2069` (removendo a linha de `devDependencies` e restaurando o lockfile original); nenhum outro path é tocado — `components.json`, `layout.css`, os 51 componentes, `utils.ts`, `prettier.config.js`, `vite.config.ts`, gate/scripts/workflows, ADRs, constituição, STATE, specs alheias, DB, rede e segredos permanecem intactos. Os artefatos 079 são documentação do próprio ciclo e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos nos comandos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
