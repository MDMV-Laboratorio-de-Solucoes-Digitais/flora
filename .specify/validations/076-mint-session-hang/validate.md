# Validation: 076 — hang do setup E2E do minter

**Data**: 2026-11-19T19:14:42Z · **Baseline**: `dev@f22a2a32c`
**Veredito R4**: **APROVADO**.

| Gate | Resultado |
|---|---|
| `check_gate_integrity.py` | PASS — 4 required checks, 8 pisos/51, exit 0 |
| `cargo fmt --all -- --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | `No issues found`, exit 0 |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`, exit 0 |
| `cargo build --workspace` / `cargo test --workspace` | exit 0; 105 passed / 0 failed |
| `bun run check` | 0 erros, 0 warnings |
| `bun run lint` | Prettier + ESLint ok, exit 0 (após correção de `_c`) |
| `bun run test:unit -- --run --project server` | 42/42 |
| `npx playwright test --list` | `Total: 4 tests in 3 files` — nenhum teste novo coletado |
| Prova do hook | `bun /tmp/run-gs076.ts` → `GLOBAL_SETUP_OK`; cargo build do minter executado fora do teste |
| `analyze.md` | `artifacts=11, errors=[], fr=5, sc=4, tasks=5, go=true` na 1ª execução |
| Verificador isolado | `GlobalSetupVerifier`: **PASS**, 0 findings |
| Fronteira | `seed.setup.ts` byte-a-byte idêntico; só `playwright.config.ts` + novo `playwright/global-setup.ts` + artefatos 076 |

## [NV]

Tempo com target totalmente frio não medido (30.18s medidos com cache); CI ao vivo, Forgejo, aprovações, merge e produção: **[NV]**. A execução completa do E2E com DB continua dependente do ambiente — não foi reexecutada neste ciclo.
