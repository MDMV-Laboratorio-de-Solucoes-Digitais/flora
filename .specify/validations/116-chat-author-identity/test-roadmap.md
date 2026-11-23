# Test roadmap — ciclo 116

**Data**: 2026-11-23 · **Baseline**: `github/dev@2052ad7f1` · **Superfície**: API HTTP (`channels.rs`), WebSocket (`websocket.rs`), Contrato DB (`flora-tests`) e UI real (`/channels` SvelteKit)

## Escopo

FR-001/002/003/004/005 · Tarefas `T-001`..`T-005` (issues `#385`..`#389`) · Domínio crítico `tenant`. Trava lockstep elevada para `messaging: 5` (total `79`), sem alteração de required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`).

## Pré-condições

- Branch `feature/116-chat-author-identity` publicada em `github`.
- Dependências frontend instaladas via `bun install --frozen-lockfile`.
- Job `db-tests` (Postgres 18) e job `e2e-browser` (preview SvelteKit em `http://127.0.0.1:4173` + Chromium + `flora-api` com `dev-seed.sql`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` | Zero erros de formatação ou clippy pedantic | CLI / Rust | sim |
| 2 | `cargo test --workspace` | Todas as suítes Rust unitárias/integração sem banco passam, incl. `realtime::tests` | CLI / Rust | sim |
| 3 | `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, 4 required checks, 10 legs / 79 pisos (`messaging: 5`) | CLI / Gate | sim |
| 4 | `bun run check && bun run lint` em `apps/frontend` | `svelte-check` 0 erros/0 avisos; `prettier` + `eslint` exit 0 | CLI / Frontend | sim |
| 5 | `bun run test:unit -- --run --project server` em `apps/frontend` | Todas as suítes passam, incl. TDD `chatAuthor.test.ts` + novos casos em `mappers.test.ts` e `MessageStore.test.ts` | CLI / Frontend | sim |
| 6 | `bun run build` em `apps/frontend` | Build Vite/SvelteKit exit 0 | CLI / Frontend | sim |
| 7 | `gh pr checks <PR> --watch` no head exato do PR | Required checks (`frontend-build`, `rust-build`, `db-tests`, `dependency-review`) e informativos (`supply-chain-vet`, `e2e-browser`) SUCCESS | CI | sim |
| 8 | Inspeção de logs `db-tests` no GitHub Actions (`gh run view`) | Perna `messaging` reporta `passed=5`, `floor=5`, `0 failed` incluindo `db_messages_include_author_display_name_across_post_list_and_catchup` | CI / Postgres 18 | sim |
| 9 | Inspeção de logs `e2e-browser` (`gh run view`) | `4 passed`, `'Alice Admin'` / `'Bob Builder'` visíveis em `# general`, `Usuário 22222222` ausente e 3 linhas WCAG `"sem achados"` (`/tasks`, `/channels`, `/files`) | CI / UI (Chromium) | sim |
| 10 | Code Review agêntico independente (contexto limpo, modelo `pro`) | 0 blockers críticos/high em tenant isolation, batch SQL, DTO safety, WS e Svelte store/race guard | PR | sim |
| 11 | Entrega WS multi-usuário ao vivo em E2E, tema escuro real e staging/produção | `[NV]` — `e2e-browser` usa seed REST + Chromium em tema claro | UI / Staging | não (`[NV]`) |

## Plano de capturas

Saídas locais registradas em `validate.md`; evidência pós-PR (`gh pr checks`, logs `db-tests` 79/79 e `e2e-browser`, resultado ACR) registrada em `.specify/validations/116-chat-author-identity/evidence.md`. Nenhuma captura com tokens ou segredos é versionada.

## Rollback

Novo commit de reversão dos arquivos do ciclo 116 (voltando `messaging` `5 → 4` em lockstep apenas se o teste de contrato for removido); sem alteração de schema e sem force-push.
