# Test roadmap — ciclo 114

**Data**: 2026-11-22 · **Baseline**: `github/dev@b32f7cafe` · **Superfície**: UI real (SvelteKit) + CLI/CI

## Escopo

FR-001/002/003/004/005 · Tarefas `T-001`..`T-005` (issues `#373`..`#377`). Nenhum required check alterado. Nenhuma regra axe desabilitada.

## Pré-condições

- Branch `feature/114-wcag-ci-guard` publicada em `github`.
- Dependências frontend instaladas via `bun install --frozen-lockfile`.
- Job `e2e-browser` (advisory) provisionando Postgres 18, `flora-api` e preview SvelteKit em `http://127.0.0.1:4173`.

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `python3 scripts/ci/check_gate_integrity.py` | PASS, 4 required checks, 10 legs / 77 pisos, exit 0 | CLI | sim |
| 2 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI | sim |
| 3 | `bun run test:unit -- --run --project server` em `apps/frontend` | Todos os testes unitários passam, incluindo `a11yAudit.test.ts` (3/3) | CLI | sim |
| 4 | `bun run build` em `apps/frontend` | Build Vite/SvelteKit exit 0 | CLI | sim |
| 5 | `gh pr checks <PR> --watch` no head exato do PR | Required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) e informativos (`supply-chain-vet`, `e2e-browser`) SUCCESS | CI | sim |
| 6 | `gh run view <run> --job <e2e_job> --log \| grep -E 'WCAG 2.2 AA em\|passed'` | Três linhas `"sem achados"` (`/tasks`, `/channels`, `/files`) e `4 passed` | CI/UI (Chromium) | sim |
| 7 | Code Review agêntico independente (contexto limpo, modelo distinto) | Nenhum blocker determinístico crítico/high | PR | sim |
| 8 | Verificação visual em tema escuro e staging/produção reais | `[NV]` — sem credenciais de staging e sem toggle `dark` no smoke E2E | UI | parcial (`[NV]`; oportunidade de automação: fixture Playwright com `prefers-color-scheme: dark` / classe `.dark`) |

## Plano de capturas

Logs de execução local, saída completa de `gh pr checks`, linhas de auditoria WCAG do log de `e2e-browser` e resultado da revisão agêntica registrados em `.specify/validations/114-wcag-ci-guard/evidence.md`. Nenhuma captura com token/credencial é versionada.

## Rollback

Novo commit de reversão dos arquivos do ciclo 114; sem force-push e sem alteração de migrations ou required checks.
