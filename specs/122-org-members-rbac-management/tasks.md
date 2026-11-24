# Tasks: `122-org-members-rbac-management`

- T-001 (Issue #420): `MembershipRepository::update_role`, `GET/POST /api/v1/roles/members`, hardening `assign_role`/`revoke_role`, unit tests e 2 contratos PG 18 (`rbac: 3 -> 5`, total `85 -> 87`) [FR-001, FR-002, FR-004]
- T-002 (Issue #421): `OrganizationMemberItem`, `utils/members.ts`, `api/roles.ts` (`encodeURIComponent` + membros), `MemberStore.svelte.ts` e `tests/unit/members.test.ts` (TDD RED -> GREEN) [FR-003, FR-004]
- T-003 (Issue #422): `pt-BR.ts`, `MemberManager.svelte` em `/settings` e Passo 9 E2E em `seed-flow.e2e.ts` + WCAG 2.2 AA + screenshots Gist [FR-003, FR-004]
- T-004 (Issue #423): Briefing `auth`/`rbac`, governança, R4/R7 (`0.14.0`), docs append-only fechando `BF-02 -> fechado`, ACR independente, merge em `dev` e escalonamento `#410` [FR-004]
