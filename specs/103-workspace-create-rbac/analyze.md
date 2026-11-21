# Analyze: 103 — RBAC na criação de workspace (READ-ONLY)

**Data**: 2026-11-21 · **Branch**: `feature/103-workspace-create-rbac` @ `14cb336173` · **Baseline**: `14cb336173`
**Artefatos revisados**: `.specify/assessments/103-workspace-create-rbac/{intake,research,define,shape,decision}.md` (`go`), `specs/103-workspace-create-rbac/{spec,plan,tasks}.md`, `docs/governance/2026-11-21-roteiro-staging-flora.md` item 11, `flora-api/src/routes/workspace.rs`, `flora-core/src/models/role.rs`, `flora-tests/src/contract/workspace.rs`.

## Diagnóstico determinístico

- **Reprodução**: papel `SomenteLeituraQA` (`permissions=[]`) criado no staging local e atribuído a `qa-user2@flora.test`; `POST /api/v1/workspaces` retornou **201** (`NegadoQA`). Roteiro item 11 exige bloqueio de ações administrativas.
- **Causa**: `create_workspace` (`workspace.rs:121-154`) descartava a membership em `_membership` e chamava o serviço sem consultar o papel. `update_workspace` exige `WorkspaceWrite|WorkspaceAdmin`; `delete_workspace` exige `WorkspaceAdmin` — assimetria confirmada.
- **Correção**: reaproveitar a membership já buscada, carregar o papel com `PgRoleRepository::find_by_id_and_organization(membership.role_id, org_id)` e negar com `Error::Forbidden("Insufficient permissions")` quando não houver `WorkspaceWrite` nem `WorkspaceAdmin`. `has_permission` aceita `*`, `workspace:write`/`workspace:admin` e os aliases legados `WorkspaceWrite`/`WorkspaceAdmin` (`role.rs:66-110`), preservando Admin, Owner e Member.

## Checagem determinística

```
artifacts=8  critical=0  fr=5/5  sc=3/3  tasks=8/8  go=True  divergences=0
FR match= True ['FR-001','FR-002','FR-003','FR-004','FR-005']
SC match= True ['SC-001','SC-002','SC-003']
```

Não foi adicionado teste `#[ignore]`: o gate deriva pisos de `cargo --ignored --list` e `FLOORS["workspace"]=3`. A regressão foi coberta por dois testes unitários puros do helper `can_create_workspace` (`cargo test -p flora-api workspace::tests` → 2 passed) e por medição HTTP ao vivo.

## Rastreabilidade

| # | Requisito | Tarefa(s) | Evidência |
|---|---|---|---|
| R1 | FR-001 — papel carregado no org da sessão antes da escrita | T002 | `find_by_id_and_organization(membership.role_id, org_id)` |
| R2 | FR-002 — `WorkspaceWrite` ou `WorkspaceAdmin` necessários | T003 | helper `can_create_workspace`; testes com `*`, aliases e `[]` |
| R3 | FR-003 — negação 403 opaca, sem vazamento | T003 | `Error::Forbidden("Insufficient permissions")`; medição `403 {"error":"forbidden"}` |
| R4 | FR-004 — Admin/Owner/Member preservados | T005 | medição admin `201`; testes unitários de wildcard/aliases |
| R5 | FR-005 — rastreabilidade, gates e fronteira | T006–T008 | fmt/check/clippy exit 0; gate do alvo reexecutado; nenhum required check tocado |

SC-001 (ReadOnly bloqueado) por R1/R2/R3; SC-002 (Admin/Member inalterados) por R4; SC-003 (fronteira de governança) por R5.

## [NV]

- Staging real e CI remota/PR/deploy: **[NV]** até verificação ao vivo.
- Teste DB-gated adicional no contrato `workspace.rs`: não executado (substituição documentada).

## Resultado

**PASS — 5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos.** Nenhuma edição nesta fase.
