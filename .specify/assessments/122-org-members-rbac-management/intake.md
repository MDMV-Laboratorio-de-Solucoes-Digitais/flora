# Assess — intake: 122

**Data**: 2026-11-24 · **Ciclo**: 122 · **Slug**: `122-org-members-rbac-management`

## Demanda

- Fechar o escopo remanescente de **`BF-02`** (`backlog_de_fechamento` em `.specify/memory/STATE.md` / `.specify/assessments/119-rbac-roles-admin-ui/define.md` / `120-hitl-receipt-http-endpoint/decision.md`): após a entrega do CRUD de papéis RBAC (`PR #409`) e do seletor de tema (`PR #419`), falta expor a gestão de membros da organização ativa (`GET /api/v1/roles/members`, `POST /api/v1/roles/members`, atribuição de papéis e revogação com proteção contra auto-rebaixamento/auto-revogação e UI `MemberManager.svelte` em `/settings`).
- Com este ciclo, todos os itens acionáveis do `backlog_de_fechamento` (`BF-01`, `BF-02`, `BF-03`, `BF-04`) ficam integralmente fechados, restando apenas itens bloqueados por decisão externa (`BF-05` RLS runtime e `BF-06` expiração operacional `2026-11-29T23:59:59Z`).
