# Converge: 111 — auditoria WCAG 2.2 AA automatizada

**Data**: 2026-11-22 · **Modo**: append-only · **Status: Converged**

- FR-001: `@axe-core/playwright@4.14.0` em `devDependencies`; `bun.lock` e `package-lock.json` sincronizados (entradas `axe-core@4.14.1`).
- FR-002: `apps/frontend/tests/e2e/accessibility.e2e.ts` com `AxeBuilder` nas rotas `/tasks` e `/channels`, tags WCAG 2.0/2.1/2.2 A+AA, mensagem de falha em pt-BR.
- FR-003: nenhum required check, gate floor, migration ou credencial alterado.
- Verificação local: `bun run check` 4886 arquivos / 0 erros; `prettier --check` no arquivo novo OK; gate autoritativo PASS (10 legs / 77 pisos).

- 2026-11-22 (pós-CI): PR #366 `e2e-browser` falhou por redirecionamento para `/select-org` (sessão seed sem organização). Corrigido com seleção prévia de Acme, replicando o smoke existente; `bun run check` 4886 arquivos/0 erros revalidado.
- **[NV]**: e2e local não executado — 4 timeouts (eslint 2×, build, e2e) com 12/13 GiB de RAM e swap cheio; preview próprio encerrado. CI `e2e-browser` é advisory e decide.
