# Implementation Plan: 079 — instalação pinada do CLI `shadcn-svelte`

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/079-install-shadcn-svelte/{intake,research,define,concept,decision}.md` (go scoped, opção A, E=7,50).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`. Convenção: Conventional Commits em pt-BR, um arquivo por commit; os artefatos do ciclo são documentação e podem ser commitados em bloco à parte.
**Base do diff**: `ea5cd2069` (HEAD no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-003 |
| T003 | FR-003 | SC-004 |
| T004 | FR-004 | SC-003 |
| T005 | FR-005 | SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004, FR-005 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez.

## Passos

1. **T001** Em `apps/frontend`, executar `bun add -D -E shadcn-svelte@1.8.1` (`latest = 1.8.1` no registry em 2026-11-19; MIT). Conferir que `devDependencies` ganhou exatamente `"shadcn-svelte": "1.8.1"`, sem `^` nem `~`, e que `dependencies` (`bits-ui`, `clsx`, `lucide-svelte`, `tailwind-merge`) não mudou. Validar com `bun pm ls shadcn-svelte` e `bun run shadcn-svelte --help` (bin local, sem `bun x`); não executar `init`, `add` nem `apply`. *(FR-001, SC-001, SC-002)*
2. **T002** Confirmar que `apps/frontend/bun.lock` registrou a entrada do pacote e as 4 transitivas (`cn`, `postcss`, `commander`, `node-fetch-native`) com `lockfileVersion: 2` e o workspace `""` consistente. Rodar `bun install --frozen-lockfile --dry-run` em `apps/frontend` e exigir exit 0 sem diff de lockfile; commitar `package.json` e `bun.lock` em commits separados (um arquivo por commit). *(FR-002, SC-003)*
3. **T003** Preservar byte-idênticos `$schema`, `style`, `tailwind` e `aliases` de `apps/frontend/components.json:1-15`, `src/routes/layout.css`, os 51 `.svelte` de `src/lib/components/ui/` e `src/lib/utils.ts`. Verificar com `git diff --exit-code ea5cd2069 -- <paths>`; nenhum comando `init`/`add`/`apply` é executado em nenhum ponto do passo. *(FR-003, SC-004)*
4. **T004** Conferir a formatação de `apps/frontend/package.json` (tabs, sem trailing commas, aspas duplas em JSON; `prettier.config.js:5-8`) e a ausência de segredos, `.tokens/`, mudanças de gate/scripts/workflows e mutação de DB/rede fora do registry. Executar `bun run check`, `bun run lint` (inclui `prettier --check .`), `bun run test:unit` e `bun run build` em `apps/frontend`, mais `python3 scripts/ci/check_gate_integrity.py` na raiz (sem DB/rede). *(FR-004, SC-003)*
5. **T005** Auditar o escopo com `git diff --name-only ea5cd2069..HEAD` e `git diff --stat ea5cd2069..HEAD`: somente `apps/frontend/package.json` (1 linha adicionada), `apps/frontend/bun.lock` e os artefatos do ciclo 079. Registrar os `[NV]` em `spec.md`/`clarify.md`, confirmar zero mutação de DB/segredos e validar a reversão dos 2 paths conforme a seção Rollback. *(FR-005, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=ea5cd2069

# 1. declaração exata sem caret/tilde (raiz)
grep -nF '"shadcn-svelte": "1.8.1"' apps/frontend/package.json
! grep -nE '"shadcn-svelte": "[~^]' apps/frontend/package.json

# 2. workspace e bin resolvido localmente (em apps/frontend)
cd apps/frontend
bun pm ls shadcn-svelte
bun run shadcn-svelte --help | grep -E 'init|add|apply|registry build'

# 3. lockfile consistente (em apps/frontend — sem mutar o arquivo)
bun install --frozen-lockfile --dry-run

# 4. formatação, tipo, lint, testes e build (em apps/frontend)
bun run check
bun run lint
bun run test:unit
bun run build
cd -

# 5. integridade de gate (raiz, sem DB/rede)
python3 scripts/ci/check_gate_integrity.py

# 6. artefatos do ciclo 078 byte-idênticos ao base (raiz)
git diff --exit-code "$BASE" -- \
  apps/frontend/components.json \
  apps/frontend/src/routes/layout.css \
  apps/frontend/src/lib/utils.ts \
  apps/frontend/src/lib/components/ui

# 7. auditoria de escopo do diff (raiz)
git diff --name-only "$BASE"..HEAD
git diff --stat "$BASE"..HEAD -- apps/frontend
```

A auditoria de escopo espera somente `apps/frontend/package.json`, `apps/frontend/bun.lock` e `specs/079-install-shadcn-svelte/` (mais os artefatos de assessment já registrados em `.specify/assessments/079-install-shadcn-svelte/`); qualquer outro path invalida SC-004. O diff de `package.json` deve conter exatamente 1 linha adicionada em `devDependencies`, e o de `bun.lock`, a entrada do pacote mais as 4 transitivas. Todos os comandos acima são determinísticos e não dependem de DB, segredos ou rede — exceto o registry já autorizado para a instalação de T001; sua execução real é `[NV]` nesta spec.

## Rollback

Reverter os commits que introduziram T001/T002 ou restaurar diretamente os 2 paths ao base:

```
git checkout ea5cd2069 -- apps/frontend/package.json apps/frontend/bun.lock
```

Isso devolve `package.json` sem a entrada `shadcn-svelte` e `bun.lock` sem a resolução, mantendo `node_modules/` local e ignorado. Nenhum `components.json`, `layout.css`, componente de `src/lib/components/ui/`, `utils.ts`, `prettier.config.js`, `vite.config.ts`, gate/script, workflow, ADR, constituição, STATE, DB, rede ou segredo é modificado. Os artefatos do ciclo 079 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; após a reversão, repetir os gates 1, 3, 5, 6 e 7 para confirmar o retorno ao estado base.
