# Converge: 071 — remover stub morto AuthService

**Data:** 2026-11-19T03:56Z · **Tipo:** append-only · **Baseline:** `dev@cdf50eaff`

## Estado convergido

- Decisão 80/20 `(2×5)/(2×2)=2.50` registrada em `STATE.md` antes do BLOCO 6.
- Assess (5), spec (8), analyze READ-ONLY (1 conflito FR-004 corrigido na fonte → `go=True`), converge.
- Implementação: 1 arquivo excluído + 10 arquivos com remoção de referências; 11 commits de um arquivo por commit.
- Evidência: `grep` zero ocorrências de `AuthService`/`auth_service`; `cargo check --workspace --all-targets` exit 0.

## Intenção preservada

- `ServiceUnavailable` continua usado por `oidc_transactions.rs` e definido em `error.rs`.
- Trait `AuthProvider` e `UserInfo` mantidos como pontos de extensão documentados (sem implementadores, sem custo de lint).
- Nenhum handler de auth, fluxo OIDC, workflow, gate, piso, migration, ADR ou constituição alterado.

## Não convergido (registrado, fora de escopo)

- Remoção da trait `AuthProvider`/`UserInfo` — decisão de arquitetura trait-driven; não feita sem autorização.
- Leg `rbac` no CI — escalado **[NV]** por aprov maintainer `rust.yml` + `FLOORS` + ADRs.
- Renomeação de 124 títulos sem prefixo de ciclo e duplicatas T-ID legadas — dívidas anteriores.

## Próxima portaria

`validate.md` (R4) → `release.md` (R7) → PR → CI verde ao vivo → merge deliberado em `dev/`.

## Correção append-only — 2026-11-19T04:02Z (contagem verificada)

O verificador isolado apontou off-by-one na redação acima: o diff contém **10 arquivos Rust**
(1 excluído + 9 com referências removidas) e **10 commits `refactor`**, não 11. Comando:
`git diff --name-only dev..HEAD | grep -c '\.rs$'` → `10`;
`git log dev..HEAD --oneline | grep -c 'refactor'` → `10`. Nenhum arquivo faltou ou sobrou;
apenas a cifra documentada estava errada. As linhas originais permanecem como histórico.
Adicionalmente, `cargo fmt --all -- --check` esteve RED por formatação de imports e foi
corrigido com `cargo fmt --all` + commit `style(rust)`, sem relaxar gate.
