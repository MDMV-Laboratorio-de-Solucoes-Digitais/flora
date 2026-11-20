# Tasks: 007 — Org-Scoped Auth + Verifiable Core Slice

**Status legend:** `[X]` done, `[~]` partial, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Research & scoping
- [X] T001 Criar scaffolding de assessment/spec 007.
- [NV] T002 Workflow mdmv-dark-factory para flora (`ses_e1ea41e0fffeDgn1ojT3GM1BXp`) — resultado não incorporado antes da síntese.
- [X] T003 Auditoria auth/session/org-scoping (`ses_e1ea41e0dffeWk8vTPsHFXBV5k`) — incorporada.
- [NV] T004 Pesquisa multi-idioma org-scoped auth/sessions (`ses_e1ea41e08ffeHj1Q40A2fgILJO`) — resultado não incorporado antes da síntese.
- [X] T005 Auditoria frontend/contrato e slice mínimo (`ses_e1ea41e07ffebb36eOtH32TuIi`) — incorporada.
- [X] T006 Consolidar achados disponíveis e finalizar decisão/escopo mínimo executado.

## Phase 1 — Backend implementation
- [X] T010 Implementar `POST /api/v1/auth/select-organization` com validação de membership/sessão, revogação da sessão atual e emissão de sessão+JWT org-scoped (novo `jti`).
- [X] T011 Garantir consistência com refresh/extrator/rotas tenant-scoped (sem mudanças adicionais necessárias no escopo).
- [~] T012 Testes focados dedicados ao novo endpoint — **não adicionados** neste incremento; suites existentes (`flora-core` lib, `flora-tests` lib) passaram.

## Phase 2 — Verification & contract
- [X] T020 Atualizar `test-flora.sh` para auth Bearer/JWT e fluxo de seleção de org + smoke tenant-scoped condicionado a token/ambiente.
- [ ] T021 Alinhamento mínimo frontend/contrato — **deferido**.
- [X] T022 Executar verificações determinísticas: `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-tests --lib` (54).
- [NV] T023 Smoke integrado com OIDC/DB/seed e runtime real — NV neste ambiente.

## Phase 3 — Delivery
- [X] T030 Atualizar artefatos 007 com resultados/lacunas/evidência.
- [~] T031 Branch/PR no GitHub (CI verde), merge e sync origin; higiene de branches — em andamento.
