# Intake: 070 — contrato RBAC real no lugar do stub

**Data:** 2026-11-19T02:58Z · **Âncora:** `dev@0c7890ef4` (= `github/dev`) · branch `feature/070-rbac-contract`

## Sinal

`flora-tests/src/contract/rbac.rs:3-7` contém apenas um teste que retorna `Ok(())`. É o último contrato vácuo do módulo; os handlers reais existem desde o ciclo 066 (`flora-api/src/routes/rbac.rs:168-320`), incluindo autoridade `RoleWrite || OrgAdmin`, imutabilidade de built-ins e escopo por sessão. Um contrato vazio não detecta regressão de autorização multi-tenant.

## Evidência

```
flora-tests/src/contract/rbac.rs           -> 1 teste, corpo `Ok(())`
flora-api/src/routes/rbac.rs:168-320       -> list/create/get/update/delete/assign/revoke reais
flora-tests/src/contract/workspace.rs:5-7  -> padrão de contrato DB-gated #[ignore] + env
pg_isready -h 127.0.0.1 -p 5432            -> aceitando conexões (PostgreSQL 18.6)
```

## Autorização

Modo autônomo; escolha 80/20 `(3×5)/(3×2)=2.50` registrada em `STATE.md` antes do BLOCO 6. Sem necessidade de autorização adicional: mudança restrita a testes. Emenda da constituição segue escalada **[NV]** por maintainer.

## Fronteira

`flora-tests/src/contract/rbac.rs`, artefatos do ciclo e `STATE.md`. Nenhum handler, migrations, config, workflows ou gates alterados.
