# Release: 103 — RBAC na criação de workspace

**Ciclo**: 103 · **Data**: 2026-11-21 · **Branch**: `feature/103-workspace-create-rbac` @ `14cb336173`
**Baseline**: `14cb336173`

## Entregue

`POST /api/v1/workspaces` agora exige `WorkspaceWrite` ou `WorkspaceAdmin` no papel da organização da sessão. Negação 403 com `{"error":"forbidden","message":"Insufficient permissions"}`, idêntica à de `update`/`delete`. Wildcard `*` e aliases legados permanecem válidos. Dois testes unitários cobrem a regra; nenhum piso do gate foi alterado.

## SemVer

**Proposta:** PATCH `0.7.4` → `0.7.5`. Correção de autorização em uma capacidade existente; nenhuma rota, permissão, campo de resposta, migration ou variável nova. Não é MAJOR: Admin/Owner/Member conservam o comportamento anterior.

## Changelog (FR / T / bug)

- `FR-001` papel carregado no tenant da sessão antes da escrita (T002).
- `FR-002` regra `WorkspaceWrite | WorkspaceAdmin` centralizada no helper `can_create_workspace` (T003).
- `FR-003` negação 403 opaca, sem vazamento (T003).
- `FR-004` wildcard/aliases e Admin preservados (T005).
- `FR-005` rastreabilidade, gates e fronteira (T006–T008).
- **Bug**: item 11 do roteiro de staging — usuário sem permissão administrativa criava workspace. Registrado em `docs/governance/2026-11-21-103-workspace-create-rbac.md`; nenhum ID de `.specify/bugs/` acionado.

## Evidência

`.specify/validations/103-workspace-create-rbac/validate.md` (**R4 APROVADO**) · `specs/103-workspace-create-rbac/analyze.md` (5/5 FR, 3/3 SC, 8/8 tarefas, 0 críticos) · `specs/103-workspace-create-rbac/converge.md` (**Converged**) · assessoria do ciclo · gate do alvo PASS reexecutado.

## Rollback

Reverter `flora-api/src/routes/workspace.rs` em **novo commit de reversão**, sem force-push. Nenhuma migration, variável ou dado persistente a desfazer; reapontar a API para o build anterior se já publicado. Não reverter gate, required checks, ADR, constituição, PRs ou merges.

## [NV]

- Staging real, CI remota, branch protection, PR/merge e deploy: **[NV]** até verificação ao vivo.
