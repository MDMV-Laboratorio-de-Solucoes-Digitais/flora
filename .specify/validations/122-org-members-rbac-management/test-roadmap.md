# Test roadmap — ciclo 122 (`122-org-members-rbac-management`)

**Data**: 2026-11-24 · **Baseline**: `github/dev@8f984adb7` · **Superfície**: Repositório (`MembershipRepository::update_role`), HTTP RBAC (`/api/v1/roles/members`, `/{id}/assign`, `/{id}/revoke`), PG 18 (`contract::rbac::db::db_`), Frontend Svelte 5 (`/settings` — `MemberManager.svelte`, `MemberStore.svelte.ts`) e E2E Playwright Chromium + Axe WCAG 2.2 AA

## Escopo

FR-001/002/003/004 · Fechamento integral de **`BF-02 -> fechado`** · Domínio crítico `auth`/`rbac` · Lockstep pisos DB (`rbac: 3 -> 5`, total `85 -> 87`).

## Passos

| # | Ação | Resultado esperado | Superfície | Automatizável? |
|---|---|---|---|---|
| 1 | `cargo test -p flora-tests contract::rbac::db::db_rbac_members_ -- --ignored --test-threads=1` (pré-implementação TDD RED) | Falha esperada (`400` vs `401` em `/members`, `204` vs `403` em auto-rebaixamento) | Rust + PG 18 | sim |
| 2 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources && cargo test --workspace` | Formatação, Clippy estrito, supply-chain e testes de unidade verdes (incl. novos unit tests em `routes::rbac::tests`) | Rust / CLI | sim |
| 3 | `cargo test -p flora-tests contract::rbac::db::db_ -- --ignored --test-threads=1` + `check_gate_integrity.py` (`87` pisos) + `check_deploy_workflows.py` | **`5 passed; 0 failed`** em `rbac` e `gate-integrity: PASS` (`87` pisos DB) | PG 18 / Gate | sim |
| 4 | `bun run test:unit -- --run --project server tests/unit/members.test.ts` pré-implementação (`TDD RED`) → implementação `MemberStore`/`utils/members` → `bun run check && bun run lint && bun run test:unit -- --run --project server && VITE_API_URL=http://127.0.0.1:3000 bun run build` | Falha inicial em módulo ausente; após implementação: `svelte-check` 0/0, `prettier`/`eslint` exit 0, **`21/21` arquivos / `89/89` testes**, `vite build` exit 0 | Frontend / CLI | sim |
| 5 | `bun run test:e2e` contra `flora-api` recompilado + `vite preview` + PG 18.7 (`Passo 9/10` `/settings`) | Listagem (`Alice Admin` `Você`, `Bob Builder`, `Carol Viewer`), reatribuição `Bob` -> `Viewer` + `page.reload()` + restauração `Member`, remoção `Carol` (`204`) + readição `carol@acme.test` (`201`), **`15/15` auditorias WCAG 2.2 AA: `sem achados`**, capturas `18`/`19`/`20` em Gist público | Browser + API + DB | sim |
| 6 | `gh pr checks <PR> --watch` + inspeção ao vivo (`frontend-build`, `rust-build`, `db-tests` `87/87`, `e2e-browser`) + ACR independente (`research`/`pro`) | Todos required checks GREEN, `e2e-browser` PASS e ACR `APROVADO` | CI / GitHub | sim |
| 7 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Produção | não (`[NV]`) |
