# 102 — Escopo por workspace na listagem de tarefas

**Data**: 2026-11-21 · **Branch**: `feature/102-task-workspace-scope` @ `ae8632e10`
** Âncora**: `.specify/feature.json` → `specs/102-task-workspace-scope`
**Gate do alvo**: `python3 scripts/ci/check_gate_integrity.py` — **PASS** (`exit=0`, 10 legs / 77 pisos)
**Fonte factory (somente leitura)**: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`

## Sinal e evidência

Roteiro `docs/governance/2026-11-21-roteiro-staging-flora.md` itens 4.6 e 7.6 exigem que tarefas e canais fiquem escopados ao workspace selecionado. Em staging local zerado (Postgres 18 isolado, API e frontend executados a partir do HEAD do alvo), `QA-001 — validar staging` permaneceu visível após trocar o seletor de `Main Workspace` para `QA Workspace 2`.

Causa determinística: `flora-api/src/routes/tasks.rs` (`list_tasks`) listava por responsável em toda a organização e `apps/frontend/src/routes/(app)/tasks/+page.svelte` não enviava o workspace selecionado nem recarregava a lista na troca.

## Decisão

Tornar `workspace_id` um query param **opcional** de `GET /api/v1/tasks`. Sem o parâmetro, o contrato atual permanece exatamente o mesmo. Com o parâmetro, o servidor valida o workspace contra a organização da sessão (`ensure_workspace_in_org`) e usa `TaskService::list_tasks(..., assignee_id = Some(user_id))`, reaplicando os filtros `!is_deleted` e `organization_id == org_id`. O frontend envia o parâmetro e recarrega ao trocar o seletor, sem logout.

Fora de escopo: gate, required checks, constituição, ADR, schema, migration, RBAC, flags, contrato de escrita de tarefas, paginação, formato de `TaskResponse`, staging real/FQDN e deploy.

## Verificação observada

- `cargo fmt --all -- --check` exit 0; `cargo check -p flora-api` 0 erros; `cargo clippy -p flora-api --all-targets -- -D warnings` exit 0.
- `bun run test:unit -- --run --project server` → 45/45 (13 arquivos); `bun run check` 0 erros; `bun run lint` exit 0; `bun run build` exit 0.
- Gate do alvo: `gate-integrity: PASS`, `all 10 db contract floors intact (77 total)`.
- API ao vivo: sem parâmetro → `QA-001`; `workspace_id=<QA Workspace 2>` → `[]`; `workspace_id=<Main Workspace>` → `QA-001`; workspace inexistente → `422` com `field=workspace_id`.
- UI ao vivo: troca para `QA Workspace 2` mostra estado vazio pt-BR; volta a `Main Workspace` restaura a tarefa.

Rollback: reverter os quatro arquivos de código/teste do ciclo (novo commit de reversão, sem force-push) e reapontar API/frontend para o build anterior. Nenhuma migration ou variável de ambiente nova.

**[NV]**: staging real da MDMV-Autonoma (URLs/credenciais não entregues), CI remota, branch protection, PR/merge e deploy. Nenhum segredo foi registrado.
