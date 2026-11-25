# Problem Definition: Escrita de papéis RBAC ainda placeholder

- **Slug**: 066-rbac-role-write
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md | define.md (equivalente local; `problem.md` ausente no baseline) | shape.md

## Problem Statement

`flora-api/src/routes/rbac.rs:131-164` contém quatro handlers placeholder — `create_role`, `get_role`, `assign_role` e `revoke_role` — que apenas retornam strings depois de checar `require_org_admin`; `list_roles` já é real desde o ciclo 042 (`rbac.rs:102-128`) e `require_org_admin` usa `can_manage_roles(&role)` (`:77-95`) (intake.md:9-12; research.md:5-9). A autoridade para criar/atribuir papéis não estava definida até a decisão do SE: permissão `RoleWrite` dedicada, `OrgAdmin` habilitado por padrão e papéis `is_builtin` imutáveis (intake.md:14-17; decision.md:7-9). A presença de `RoleWrite` no modelo `Permission` e as assinaturas de `RoleRepository`/`MembershipRepository` estavam a confirmar no mapeamento do verificador (research.md:10-13). Ranqueamento 80/20 = 2.5, último item de código autorizado (intake.md:25-27).

## Affected Users & Stakeholders

- **Users**: administradores de uma organização que precisam criar papéis e alterar associações de fato, em vez de receber respostas de no-op (intake.md:9-12; define.md:8-11).
- **Stakeholders**: SE, que decidiu a autoridade (`RoleWrite` + `OrgAdmin`, builtins imutáveis), e mantenedores da API/RBAC, que precisam de testes de autorização sem habilitar RLS em produção (`decision.md:7-13`; `define.md:19`).

## Goals

- Fazer `can_manage_roles` aceitar `RoleWrite` e manter `OrgAdmin` como autoridade padrão (`define.md:7`; `shape.md:9-10`).
- Implementar `create_role` com permissões validadas e `get_role` a partir da organização da sessão (`define.md:8-9`).
- Fazer `assign_role`/`revoke_role` alterarem membership real (`define.md:10`; `shape.md:11-12`).
- Proteger papéis `is_builtin` contra alteração/exclusão e não criar rota nova sem autenticação e tenant da sessão (`define.md:11-12`).

## Non-Goals

- Não habilitar RLS em produção (C6) (`define.md:16`; `decision.md:12`).
- Não adicionar convites, e-mail ou token (`define.md:17`).
- Não alterar ADRs, constituição ou deploy (`decision.md:12`; `intake.md:31-33`).
- Não permitir edição de builtin nem adotar `RoleWrite` sozinho sem `OrgAdmin` (`research.md:22-23`; `shape.md:10-11`).

## Success Metrics

- Testes unitários cobrem `RoleWrite` e imutabilidade de builtin (baseline: 4 handlers placeholder, sem testes de autoridade de escrita — define.md:15; research.md:5-7).
- fmt/clippy/deny/build/test e frontend verdes (baseline: verdes; handlers ainda no-op — define.md:16).
- CI verde e diff revisável (baseline: nenhuma mudança de código — define.md:17; decision.md:11).
- Nenhuma mudança em RLS/ADRs/constituição (baseline: RLS fora do escopo — define.md:18).
- Execução real em DB/PR: **[NV]** — CI observada antes do merge (decision.md:17).

## Cost of Inaction

Os quatro handlers continuam respondendo como se tivessem mutado papéis sem gravar nada, enquanto a autoridade decidida pelo SE não se reflete no código. Administradores não conseguem gerenciar papéis e a superfície permanece enganosa; o risco de uma implementação posterior sem as guardas de tenant e builtin aumenta (intake.md:9-17; research.md:19-23).

## Open Questions

- [NEEDS CLARIFICATION: `RoleWrite` já existe no modelo `Permission` ou a variante precisa ser adicionada?] (`research.md:10-11`; `intake.md:19`)
- [NEEDS CLARIFICATION: as assinaturas de `RoleRepository::{create,update,delete}` e `MembershipRepository` permitem assign/revoke como registrados?] (`research.md:12-13`)
- [NEEDS CLARIFICATION: `revoke_role` = remover a membership do usuário na organização é a semântica aceita?] (`shape.md:11`; assunção registrada)
- [NEEDS CLARIFICATION: em qual PR a execução real em DB/CI será observada?] (`decision.md:17`; **[NV]**)
