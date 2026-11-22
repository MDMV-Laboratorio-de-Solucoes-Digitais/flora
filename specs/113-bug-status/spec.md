# Feature Specification: Fechamento documental de bugs

**Feature ID**: 113 · **Status**: specified · **Data**: 2026-11-22

## User Scenario

Como mantenedor, quero que o inventário de bugs reflita o que já foi corrigido e verificado, para priorizar o próximo ciclo com evidência em vez de dívida stale.

## Acceptance Scenarios

1. D15: `cargo test -p flora-api hitl_receipt` → 2 passed, anotado em `fix.md`/`test.md`.
2. OIDC: rota `/auth/password/login` presente e `cargo test -p flora-api password` → 1 passed, registrado em novos `fix.md`/`test.md`.
3. WCAG-111: PR #370 mesclado e axe "sem achados", anotado em `assessment.md`.
4. Nenhum código, gate ou required check alterado.

## Requirements

FR-001/002/003/004 conforme define.md.

## [NV]

Produção, IdP externo e staging real.
