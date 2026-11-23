# Validação R4 — ciclo 117

**Data**: 2026-11-23 UTC · **Veredito: APROVADO PARA CI** *(evidência local completa; validação remota de `db-tests`, `e2e-browser` e ACR independente a anexar em `evidence.md`)*

| Comando | Resultado observado |
|---|---|
| `bun run test:unit -- --run --project server tests/unit/notifications.test.ts` (`task-199` → `task-222`, RED → GREEN) | exit `1` pré-implementação (`Cannot find module '../../src/lib/state/NotificationStore.svelte'`) → `8/8 passed` (`mappers.test.ts` `4`, `notifications.test.ts` `4`) |
| `cargo fmt --all -- --check` (`task-331`) | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` (`task-331`) | `0` erros em todo o workspace (`Finished dev profile`) |
| `cargo test --workspace` (`task-331`) | `flora-api` `46 passed`, `flora-core` `35+21 passed`, `flora-tests` `36 passed; 0 failed; 80 ignored`, exit `0` |
| `python3 scripts/ci/check_gate_integrity.py` (`task-331`) | `gate-integrity: PASS`, `all 10 db contract floors intact (80 total)` (`notifications: 6`), exit `0` |
| `bun run check` (`svelte-check`, `task-295`) | `svelte-check found 0 errors and 0 warnings`, exit `0` |
| `bun run lint` (`prettier --check . && eslint .`, `task-295`) | `All matched files use Prettier code style!` / `0` erros ESLint, exit `0` |
| `bun run test:unit -- --run --project server` (`task-295`) | `Test Files  18 passed (18)`, `Tests  69 passed (69)`, exit `0` |
| `bun run build` (`vite build`, `task-295`) | `✓ built in 3m 10s` (`@sveltejs/adapter-node` `✔ done`), exit `0` |

Roteiro de testes: `.specify/validations/117-notifications-ui-and-honest-navigation/test-roadmap.md` · Evidência completa pós-CI/ACR: `.specify/validations/117-notifications-ui-and-honest-navigation/evidence.md`.
Briefing de Compreensão (`tenant`): `docs/governance/2026-11-23-117-briefing-compreensao.md`.
Issues R6 vinculadas: `#391`..`#395`.

**Conclusão**: todos os passos locais automatizáveis do roteiro (1–6) obtiveram `PASS`; os passos remotos (`7–10`: `db-tests` 80/80, `e2e-browser` Passos 6 e 7 + WCAG 2.2 AA, ACR independente) e o Passo 11 (tema escuro real e staging/produção externos) permanecem **`[NV]`** até execução no PR.
