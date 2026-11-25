# Implementation Plan: `122-org-members-rbac-management`

**Baseline**: `github/dev@8f984adb7` · **SemVer**: MINOR (`0.13.0 -> 0.14.0`)

## Etapas

1. **T-001 (`#420`)**: TDD RED contratos PG 18 (`flora-tests/src/contract/rbac.rs`) → implementar `MembershipRepository::update_role`, `GET/POST /api/v1/roles/members`, hardening de `assign_role`/`revoke_role`, testes unitários `rbac::tests`, lockstep `rbac: 5` (`87` total) e validar `fmt`/`clippy`/`deny`/`test`/`db_`/`gate_integrity`/`flora-api` build.
2. **T-002 (`#421`)**: TDD RED unitário (`tests/unit/members.test.ts`) → implementar `OrganizationMemberItem` (`models.ts`), `utils/members.ts`, `api/roles.ts` (`encodeURIComponent` + API de membros) e `MemberStore.svelte.ts` → validar GREEN.
3. **T-003 (`#422`)**: Chaves `pt-BR.ts`, componente `MemberManager.svelte`, montagem em `routes/(app)/settings/+page.svelte` e extensão do Passo 9 E2E (`seed-flow.e2e.ts`) + WCAG 2.2 AA + Gist.
4. **T-004 (`#423`)**: Briefing `auth`/`rbac`, governança, R4/R7 (`0.14.0`), docs append-only fechando `BF-02 -> fechado`, ACR independente (`research`/`pro`), CI remoto, merge em `dev` e escalonamento em `#410` (`dev -> main`).
