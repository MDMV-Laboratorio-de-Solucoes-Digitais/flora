# Converge: 070 — contrato RBAC real

**Data:** 2026-11-19T03:02Z · **Tipo:** append-only · **Baseline:** `dev@0c7890ef4`

## Estado convergido

- Decisão 80/20 `(3×5)/(3×2)=2.50` registrada em `STATE.md` antes do BLOCO 6.
- Assess (5), spec (8), analyze READ-ONLY (1 conflito corrigido na fonte → `go=True`), converge.
- Implementação: `flora-tests/src/contract/rbac.rs` — 256 inserções / 5 remoções; stub `Ok(())` substituído por 3 contratos DB-gated com fail-closed.
- Evidência de execução: `3 passed` com Postgres descartável `flora_070_rbac_verify`; `FAILED ... required for DB contract tests` sem env; mutação da asserção cross-org produziu `CROSS-ORG ROLE LEAK`, confirmando sensibilidade do teste; fonte restaurada byte-a-byte.

## Intenção preservada

- Nenhum handler, config, migration, workflow, gate, ADR ou constituição alterado.
- Nenhum piso de `db-tests` modificado; nenhum job renomeado.
- Convenção do repo preservada (`#[cfg(test)] mod db`, `#[tokio::test] #[ignore]`, `anyhow::bail!/ensure!`, `make_test_state`).

## Dívida registrada (não corrigida neste ciclo)

- O CI não possui leg `contract::rbac::db::db_`; os 3 testes são `#[ignore]` e não entram em `cargo test --workspace`. Para executá-los no CI é necessário novo leg em `rust.yml` + `FLOORS` no guardião + alinhamento de ADRs (governança, aprov maintainer) — ciclo separado, **[NV]**.
- Contratos de assign/revoke ainda não escritos; duplicatas T-ID legadas e emenda da constituição permanecem como dívidas já registradas.

## Próxima portaria

`validate.md` (R4) → `release.md` (R7) → PR → CI verde ao vivo → merge deliberado em `dev/`.

## Correção de redação (append-only) — 2026-11-19T03:08Z

O verificador isolado apontou um nit textual: o item acima cita `anyhow::bail!/ensure!`,
mas `db_state()` falha fechado com `ok_or_else(|| anyhow::anyhow!(...))?` (comportamento
equivalente: `Err` sem env, nunca `Ok` vazio). A linha histórica permanece intacta;
esta nota a corrige. Nenhuma lógica ou teste foi alterado.
