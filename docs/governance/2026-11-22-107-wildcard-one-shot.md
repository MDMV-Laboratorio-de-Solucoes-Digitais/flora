# 107 — Wildcard de recuperação com papel e one-shot

**Data**: 2026-11-22 · **Âncora**: `specs/107-wildcard-one-shot`
**Bug**: `.specify/bugs/d11-wildcard-unbounded/` (MAJOR, INBOX autorizado)
**ADR**: `docs/adr/0007-wildcard-recovery-expiration.md` (expira 2026-11-29)

## Decisão

O seletor `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` passa a escolher apenas usuários ativos com papel `Admin`/`Owner` em organização não-sistema e só pode ser usado uma vez (marcador persistente com `pg_advisory_xact_lock`). O startup não reaplica o hash após o consumo; o login falha com 401 genérico.

## Evidência

Migration aditiva; helper SQL único; testes de contrato existentes ajustados com papel administrativo sem alterar as contagens 13/23; ADR com quatro gates e data de expiração.

**[NV]**: staging/produção, credenciais reais e restart multiréplica. Rollback = remover as env e reiniciar; a tabela é aditiva.
