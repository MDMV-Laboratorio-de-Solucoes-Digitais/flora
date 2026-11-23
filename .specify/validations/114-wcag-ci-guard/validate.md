# Validação R4 — ciclo 114

**Data**: 2026-11-22 · **Veredito: APROVADO PARA CI** *(pré-CI; aguarda execução remota e ACR antes do merge em `dev`)*

| Comando | Resultado observado |
|---|---|
| `bun run test:unit -- --run --project server tests/unit/a11yAudit.test.ts` (RED) | exit 1 (`Cannot find module '../../src/lib/utils/a11yAudit'`) |
| `bun run test:unit -- --run --project server tests/unit/a11yAudit.test.ts` (GREEN) | 1 arquivo / 3 testes passed, exit 0 |
| `bun run check` (`svelte-check`) | 0 errors and 0 warnings, exit 0 |
| `bun run lint` (`prettier --check . && eslint .`) | All matched files use Prettier code style / 0 problemas ESLint, exit 0 |
| `bun run test:unit -- --run --project server` | Test Files 15 passed (15), Tests 53 passed (53), exit 0 |
| `bun run build` (`vite build`) | built in 35.87s (`adapter-node` done), exit 0 |
| `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS` (4 required checks, all 10 db contract floors intact / 77 total), exit 0 |
| `git diff --check` | limpo, exit 0 |

Roteiro de testes pré-PR: `.specify/validations/114-wcag-ci-guard/test-roadmap.md`.
Issues R6 vinculadas: `#373`..`#377`.

**Pendente até o merge em `dev`**: execução do PR no GitHub Actions (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`, `supply-chain-vet`, `e2e-browser` com `"sem achados"` em `/tasks`, `/channels`, `/files`), revisão agêntica independente (0.4/7.3c) e registro final em `evidence.md`. Modo escuro e staging/produção permanecem **[NV]**.
