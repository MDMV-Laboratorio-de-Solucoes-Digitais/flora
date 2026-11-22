# Bug Report: D11 — recuperação wildcard sem papel e sem one-shot

- **Slug**: `d11-wildcard-unbounded`
- **Data**: 2026-11-22
- **Origem**: INBOX do SE (revisão interagentes), severidade MAJOR
- **Status**: confirmado; correção no ciclo 107

## Sintoma

Com `FLORA_BOOTSTRAP__ADMIN_EMAIL=*`, o startup e o login local selecionam o primeiro usuário ativo sem filtro de papel (`flora-core/src/config/bootstrap.rs:133-147`, `flora-api/src/routes/auth.rs:456-470`) e o opt-in permanece ativo indefinidamente enquanto a variável existir.

## Impacto

Qualquer usuário ativo — inclusive sem privilégio administrativo — pode receber o hash de recuperação e autenticar; o seletor continua utilizável após o primeiro login, ampliando a janela de abuso. As notas 100/101 registraram mitigação apenas procedural.

## Evidência local

`grep -rn 'is_wildcard_selector' flora-api/src flora-core/src` → 6 ocorrências; nenhuma consulta filtra `memberships`/`roles`; nenhum marcador persistente de consumo.

## Critério de aceite

Seleção restrita a papel built-in `Admin`/`Owner`, consumo one-shot atômico em Postgres, ADR com data de expiração, testes sem alterar pisos do gate e nenhuma credencial em log/PR.
