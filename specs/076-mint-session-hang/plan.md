# Implementation Plan: 076 — hang do setup E2E do minter de sessão

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/076-mint-session-hang/{decision,define,concept,research}.md` (go scoped, opção A).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`.

## Passos

1. **T001** Inserir exatamente uma linha em `apps/frontend/playwright.config.ts`: `globalSetup: './playwright/global-setup.ts',`, preservando `testDir`, `testMatch`, `projects`, `webServer` e o `testMatch` do projeto `setup`; o arquivo apontado não casa com nenhum `testMatch`, ficando fora da coleta de testes. *(FR-001, FR-004, SC-001, SC-003)*
2. **T002** Criar `apps/frontend/playwright/global-setup.ts`: executar, via `execFileSync`, `cargo build -q -p flora-core --bin flora-mint-session --features dev-mint` com `cwd` na raiz do repositório (resolvida a partir do arquivo, como em `seed.setup.ts:6`) e `stdio: 'inherit'`; falhar rápido se o build falhar; não adicionar nenhum comando de DB. *(FR-001, FR-002, FR-004, FR-005, SC-001, SC-002, SC-004)*
3. **T003** Confirmar que `apps/frontend/tests/e2e/seed.setup.ts` permanece intocado: `test.setTimeout(180_000)` na linha 14 sem aumento e o bloco `cargo run` de 36-53 preservado, agora apenas consumindo o target quente produzido por T002, sem recompilação. *(FR-002, FR-003, FR-004, SC-001, SC-002)*
4. **T004** Executar os gates determinísticos abaixo: descoberta com `playwright test --list` (globalSetup carregado, `global-setup.ts` não listado como teste, `seed.setup.ts` único no projeto `setup`), build do minter isoladamente (mesmo comando de T002) e os checks do alvo, incluindo `check_gate_integrity.py`. *(FR-005, SC-001, SC-003, SC-004)*
5. **T005** Auditoria de escopo com `git diff --name-only dev..HEAD`: apenas `apps/frontend/playwright.config.ts`, `apps/frontend/playwright/global-setup.ts` e os artefatos `specs/076-mint-session-hang/{spec,clarify,plan}.md`; registrar **[NV]** para o tempo com target frio e para CI/produção. *(FR-005, SC-003, SC-004)*

## Gates determinísticos

```
# a partir de apps/frontend — descoberta (sem executar testes, sem tocar DB)
bunx playwright test --list

# a partir da raiz do repositório — mesmo comando do globalSetup
cargo build -q -p flora-core --bin flora-mint-session --features dev-mint

# frontend (alvo)
bun run check
bun run lint
bun run test:unit

# integridade de gates e escopo do diff
python3 scripts/ci/check_gate_integrity.py
git diff --name-only dev..HEAD
```

## Rollback

Reverter o commit que introduziu T001/T002 (remover a linha `globalSetup` e excluir `apps/frontend/playwright/global-setup.ts`), restaurando a config original; `seed.setup.ts` nunca foi alterado, então não há reversão de setup. Nenhum código de produção, workflow, schema, required check, ADR, constituição, STATE ou dado de DB é modificado; sem force-push ou reescrita de histórico.
