# Implementation Plan: 064 — N3 org-scoped roles

**Data:** 2026-11-18T11:16Z · **Baseline:** `github/dev@448090327`

## Componentes

| Componente | Papel | Depende de |
|---|---|---|
| research | assinaturas e call sites mapeados | — |
| repositório | `find_by_id_and_organization` | research |
| serviços | get_role/check/assign/list/update escopados | repositório |
| rotas | passar org da sessão | serviços |
| migrations | UNIQUE + FK com pré-voo | research |
| verificação | testes, gates, auditoria DB | tudo |

Ordem: repositório → serviços → rotas → migrations → verificação.

## Abordagem

1. Trait + impl: `find_by_id_and_organization(id, org)` com
   `WHERE id=$1 AND organization_id=$2`.
2. `RbacService::get_role(role_id, organization_id)` — validar org no cache;
   `PermissionService`: `check_permission`/`assign_role` usam novo método;
   `list_permissions`/`update_role_permissions` ganham `organization_id`.
3. Rotas `rbac.rs`, `workspace.rs`, `org.rs` passam a org disponível da sessão
   ou da membership.
4. Migrations: `20261118000002_role_org_unique.sql` e
   `20261118000003_memberships_role_fk.sql` com `RAISE EXCEPTION` se houver
   órfãos/mismatch; `ON DELETE RESTRICT`.
5. Testes: unitário puro de rejeição cross-org + validação das migrations em
   DB temporário limpo.

## Riscos

- Assinaturas de serviço mudam, mas não há chamadores externos no repo.
- Deploy em produção com dados sujos falha fechado — desejado; **[NV]**.

## Verificação

`cargo test`, gates, psql em DB limpo, verificador isolado, CI do PR.

## Fora do plano

Deploy, limpeza de dados, ADRs, C5, C6.
