# Release: 107 — wildcard one-shot

**Ciclo**: 107 · **Data**: 2026-11-22 · **Baseline**: `df8c1e185`

## Entregue

`FLORA_BOOTSTRAP__ADMIN_EMAIL=*` seleciona apenas administradores `Admin`/`Owner` e é one-shot via tabela singleton com lock e CAS. Startup não reaplica hash após consumo; login falha 401. ADR-0007 com expiração 2026-11-29 e quatro gates.

## SemVer

**Proposta:** PATCH `0.7.8` → `0.7.9`. Correção de segurança em mecanismo temporário; nenhuma rota, contrato de sessão ou config obrigatória nova.

## Changelog (FR / T / bug)

- `FR-001..FR-004` seleção por papel, CAS persistente, startup e login fail-closed (T002–T005).
- `FR-005` ADR 0007 e contratos sem novos `#[ignore]` (T006, T007).
- **Bug**: D11 do INBOX autorizado (`d11-wildcard-unbounded`).

## Evidência

R4 APROVADO · 13 contratos DB de bootstrap + 4 de login wildcard com Postgres local · 70 testes unitários · gate PASS (10 legs / 77 pisos).

## Rollback

Remover as env e reiniciar; migration é aditiva. Novo commit de reversão sem force-push.

## [NV]

Staging/produção, credenciais e restart multiréplica.
