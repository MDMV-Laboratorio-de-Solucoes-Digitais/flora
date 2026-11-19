# Tasks: 076 — hang do setup E2E do minter de sessão

- **T001** Editar `apps/frontend/playwright.config.ts`: adicionar `globalSetup: './playwright/global-setup.ts',` sem alterar `testDir`, `testMatch`, `projects` ou `webServer`. *(FR-001, FR-005, SC-003)*
- **T002** Criar `apps/frontend/playwright/global-setup.ts`: `execFileSync('cargo', ['build', '-q', '-p', 'flora-core', '--bin', 'flora-mint-session', '--features', 'dev-mint'], { cwd: REPO_ROOT, stdio: 'inherit' })` com cwd na raiz do repo. *(FR-001, FR-004, SC-001, SC-002)*
- **T003** Manter `apps/frontend/tests/e2e/seed.setup.ts` intacto: `test.setTimeout(180_000)` em `:14` e `cargo run` em `:36-53` passam a consumir o binário pré-construído, sem recompilação na janela do teste. *(FR-002, FR-003, FR-004, SC-001)*
- **T004** Gates determinísticos: `playwright test --list`, build do minter, `bun run check` / `bun run lint` / `bun run test:unit`, `check_gate_integrity.py` e `git diff --name-only`. *(FR-005, SC-003, SC-004)*
- **T005** Fechamento do ciclo: registrar a evidência local com `[NV]` para target frio, CI e produção, confirmar o diff restrito e preparar o rollback por reversão dos dois paths editados. *(FR-005, SC-002, SC-004)*

Rollback: reverter `apps/frontend/playwright.config.ts` e remover `apps/frontend/playwright/global-setup.ts`; nenhum outro path é tocado.

Convenção: um arquivo por commit; Conventional Commits em pt-BR; paths explícitos; nunca `git add -A`/`.`. Sem `/speckit.validate` nem `/speckit.release`.
