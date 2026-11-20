# 045 — Fecho cross-tenant: `workspace_id` do body não é autorização

**Data:** 2026-11-17 · **Metodologia:** hub `mdmv-dark-factory` aplicado ao Flora;
6 agentes de decisão (ranking, caçada cross-tenant, resíduos auth, wrapper RLS, OWASP, custo de gate), gates determinísticos, auditoria author-blind.

## 1. Decisão 80/20

Com o Seed fechado no 044, os 6 agentes ranquearam os gaps restantes. O escolhido foi o **único** que é simultaneamente falha de segurança confirmada em código, autônomo, de baixo esforço e verificável:

**`POST /api/v1/tasks` aceitava `workspace_id` de outra org** (`tasks.rs:188` antes do fix): só checava membership (`:185`) e gravava o `workspace_id` cru do body em `Task::new`, com `organization_id` da sessão. Atacante da Org A podia criar task apontando para workspace da Org B — cross-tenant **write** (CWE-639 / A01:2021 / BOLA). `list_tasks` filtra `org_id`, contendo o read.

| Hipótese | Impacto | Confiança | Esforço | Veredito |
|---|---|---|---|---|
| H1 — Validar `workspace_id ∈ org` em `create_task` | Alto (segurança) | Alta | Baixo | **FEITO** |
| H2 — Promover 5 contratos a legs `db-tests` | Alto | Alta | Baixo | Bloqueado (required check — SE) |
| H3 — Wrapper de transação p/ RLS | Médio | Média | Médio | Não (2 ciclos; ADR-0006) |
| H4 — RBAC assign/revoke | Médio | Baixa | Médio | Não (decisão do dono) |
| H5 — Logout/401/invite frontend | Médio | Alta | Médio | Follow-up (não bloqueia) |

## 2. Evidência primária (OWASP, EN/PT-BR/ES)

- **A01:2021 / CWE-639 / API1:2023 BOLA**: "client-supplied tenant identifiers are *selectors only*"; "verify the authenticated principal is authorized to act in the selected tenant"; "fail closed"; "every API endpoint that receives an ID ... should implement object-level authorization" — inclusive no payload.
- **ASVS 4.0.3 V4.2.1**, **Multi-Tenant/IDOR/Authorization Cheat Sheets**, **CISA AA23-208A** (body manipulation): checar titularidade em todo request; lookup escopo `WHERE id=? AND org_id=?`.
- **RLS é defesa em profundidade**, não substituto da checagem na app (OWASP + PostgreSQL 18 §5.9: owner/BYPASSRLS bypassam, FK/unique bypassam, filtro silencioso) — alinhado ao ADR-0005.
- Incidente real: First American Financial (2019), ~885M documentos por ID sequencial na URL.

## 3. Implementação

1. Novo módulo compartilhado `flora-api/src/tenant.rs` com `ensure_workspace_in_org(state, org_id, workspace_id)` — centraliza a checagem (MUST OWASP #4) em vez de manter helper privado em `files.rs`.
2. `files.rs`: helper duplicado removido; passa a importar o compartilhado (6 contratos DB de files continuam verdes).
3. `tasks.rs`: `create_task` chama o helper **antes** de `Task::new` (1 novo call site).
4. Novo contrato DB `db_task_cross_org_workspace_rejected`: workspace próprio → 201; workspace de outra org → 4xx (swap test OWASP).

## 4. Verificação

- 4 contratos tasks passando em PG18 descartável (`:55433`, DB `flora_045`), incluindo o novo; 6 contratos files passando (non-regression do helper movido).
- Gates: fmt, clippy `--all-targets -D warnings`, deny, `cargo test --workspace` (13 suítes), `check_gate_integrity.py` — todos verdes.
- **Não tocado**: required checks, deploy, RLS enforcement, ADRs, trabalho de outros agentes.
