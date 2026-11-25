# Governança 120 — Endpoint HTTP RBAC `POST /api/v1/hitl/receipts` (`BF-03`)

**Data**: 2026-11-24 · **Baseline**: `github/dev@e47983874` · **Issues**: [#411](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/411)–[#413](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/413)

## Objetivo e Escopo

Fechar o item `BF-03` do `backlog_de_fechamento` expondo o endpoint HTTP consumidor de `HitlReceiptAdapter` em `flora-api` com guarda RBAC de administrador de organização (`Permission::OrgAdmin`), derivação exclusiva de `operator_id` no servidor, mapeamento `503 Service Unavailable` sanitizado em `flora-core` e contratos PostgreSQL 18 (`hitl: 3`, elevando o piso total de `82` para `85`).

## Alterações previstas e rastreabilidade

- **HTTP + Error (`T-001` / `#411`)**:
  - `flora-core/src/error.rs`: `Error::ServiceUnavailable(_)` mapeado para `503 SERVICE_UNAVAILABLE` sanitizado + teste unitário.
  - `flora-api/src/routes/hitl.rs` e `flora-api/src/routes/mod.rs`: rota `POST /api/v1/hitl/receipts` com `#[serde(deny_unknown_fields)]`, `can_emit_hitl_receipt`, `build_decision`, `require_hitl_operator` e `3` testes unitários.
- **Contratos DB e Lockstep (`T-002` / `#412`)**:
  - `flora-tests/src/contract/{mod.rs,hitl.rs}`, `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml` (`hitl: 3`, total `85`).
- **Briefing, R4/R7 (`0.12.0` MINOR) e Memória (`T-003` / `#413`)**:
  - Artefatos SDD em `.specify/{assessments,validations,releases}/120-hitl-receipt-http-endpoint/`, `specs/120-hitl-receipt-http-endpoint/`, `docs/governance/2026-11-24-120-*` e atualizações append-only (`BF-03 -> fechado`).
