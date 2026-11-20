# Implementation Plan: 081 — limpeza das chaves OIDC em `.env.example`

**Entrada**: `spec.md`, `clarify.md` e `.specify/assessments/081-env-oidc-cleanup/{intake,research,define,concept,decision}.md` (go (scoped), opção A, E=10,0).
**Ordem**: paths explícitos, um arquivo por commit, nunca `git add -A`/`.`. Convenção: Conventional Commits em pt-BR; o único arquivo de código/config alterado (`.env.example`) vai em um commit próprio e os documentos do ciclo podem ser commitados em bloco à parte.
**Base do diff**: `35b8d3142` (HEAD da branch `feature/081-env-oidc-cleanup` no início do ciclo).

## Rastreabilidade

| Passo | Requisito funcional | Critério de sucesso |
|---|---|---|
| T001 | FR-001 | SC-001, SC-002 |
| T002 | FR-002 | SC-001 |
| T003 | FR-003 | SC-002, SC-004 |
| T004 | FR-004 | SC-003, SC-004 |

Cada célula referencia identificadores individuais — não intervalos. Cobertura: FR-001, FR-002, FR-003, FR-004 e SC-001, SC-002, SC-003, SC-004 aparecem ao menos uma vez. SC-001 aparece em T001 e T002 porque o `grep` só satisfaz a métrica após a nova chave entrar (T001) **e** o alias isolado sair (T002); SC-002 aparece em T001 (sintaxe do exemplo) e T003 (leitores preservados); SC-004 aparece em T003 e T004 (byte-identidade e auditoria).

## Passos

1. **T001** Em `.env.example`, substituir as linhas `:62` (`# Redirect/callback URI (verify config key support in your flow):`) e `:63` (`FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback`) por 3 linhas de comentário: duas explicando que o callback padrão deriva de `FLORA_APP__BASE_URL` (`flora-api/src/routes/auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`flora-core/src/config/oidc.rs:20-21`, `auth.rs:105-117`), seguidas do exemplo **comentado** `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (aspas duplas internas, como em `.env.example:35`). Conferir que `:59-61` e `:66-68` não são tocadas. O `grep` de SC-001 ainda não zera o `OIDC_ISSUER_URL` isolado neste ponto (`:65` permanece); não commitar isoladamente — T001 e T002 incidem sobre o mesmo arquivo e formam um único commit. *(FR-001, SC-001, SC-002)*
2. **T002** Remover a linha `:65` (`# OIDC_ISSUER_URL=http://localhost:8080`). Opcionalmente realinhar `:64` para `# Legacy fallbacks read by \`OidcConfig::default()\` via \`std::env::var:\``. Rodar o `grep` de SC-001 e conferir: `__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET` e `__ALLOWED_REDIRECT_URIS` (comentado) presentes; zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado fora de `ZITADEL_ISSUER_URL`. Commitar somente `.env.example` (`git add .env.example`; nunca `git add -A`/`.`). *(FR-002, SC-001)*
3. **T003** Verificar, sem editar, que nenhum leitor mudou: `flora-core/src/config/oidc.rs`, `flora-core/src/config/mod.rs` e `flora-api/src/routes/auth.rs` permanecem byte-idênticos ao base; nenhum `deny_unknown_fields`, alias, variável nova ou shim foi introduzido. Confirmar que o exemplo segue comentado, com aspas duplas válidas para `Vec<String>` (`figment 0.10.19`), e que a allowlist exata de `auth.rs:105-117` não foi duplicada com o callback padrão. Rodar `cargo check -p flora-core` e `python3 scripts/ci/check_gate_integrity.py`, exigindo gate do alvo PASS. Qualquer diff em Rust, auth, gate ou workflows invalida o passo. *(FR-003, SC-002, SC-004)*
4. **T004** Auditar o escopo com `git diff --name-only 35b8d3142..HEAD` e `git diff --stat 35b8d3142..HEAD`: somente `.env.example` (2 linhas substituídas, 1 removida, no máximo 1 redação ajustada) e os artefatos do ciclo 081 (6 documentos em `specs/081-env-oidc-cleanup/` e 5 de assessment em `.specify/assessments/081-env-oidc-cleanup/`). Confirmar byte-idênticos os demais paths (Rust, auth, docs de research, contrato OIDC, worktree `state-closure-072`, gate/scripts, `.github/workflows/`, ADRs, constituição, STATE, DB, segredos, lockfiles). Registrar os `[NV]` em `spec.md`/`clarify.md`, confirmar zero mutação de DB/segredos e validar a reversão do único path conforme a seção Rollback. *(FR-004, SC-003, SC-004)*

## Gates determinísticos

```
# 0. base de comparação (raiz do repo)
BASE=35b8d3142

# 1. chaves do bloco OIDC (raiz) — esperado: __ISSUER_URL, __CLIENT_ID,
#    __CLIENT_SECRET, __ALLOWED_REDIRECT_URIS (comentado); zero __REDIRECT_URI
#    e zero OIDC_ISSUER_URL isolado fora de ZITADEL_ISSUER_URL
grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example

# 2. compilação do crate alvo (raiz) — nenhum consumidor de config muda
cargo check -p flora-core

# 3. integridade de gate (raiz, sem DB/rede)
python3 scripts/ci/check_gate_integrity.py

# 4. artefatos sensíveis byte-idênticos ao base (raiz) — falha se imprimir algo
git diff --exit-code "$BASE" -- \
  flora-core/src/config/mod.rs \
  flora-core/src/config/oidc.rs \
  flora-api/src/routes/auth.rs \
  scripts/ci/check_gate_integrity.py \
  .github/workflows

# 5. auditoria de escopo do diff (raiz)
git diff --stat "$BASE"..HEAD
git diff --name-only "$BASE"..HEAD
```

O gate 1 verifica diretamente SC-001/SC-002; o gate 2 e o gate 3 não leem `.env` em runtime (`.env.example:33-34`). A auditoria de escopo espera somente `.env.example` e os artefatos do ciclo 081; qualquer outro path invalida SC-003/SC-004. Como os diretórios 081 estão ainda não rastreados no início do ciclo, auditorias com `$BASE..HEAD` pressupõem que os artefatos foram adicionados com paths explícitos (nunca `git add -A`/`.`). Todos os comandos acima são determinísticos e não dependem de DB, segredos ou rede; sua execução real fica com o main agent após a edição e é `[NV]` nesta spec (marcadores em SC-001 e SC-003 de `spec.md`).

## Rollback

Reverter o commit que introduziu T001/T002 ou restaurar diretamente o único path ao base:

```
git checkout 35b8d3142 -- .env.example
```

Isso devolve `# Redirect/callback URI (verify config key support in your flow):`, `FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback` e `# OIDC_ISSUER_URL=http://localhost:8080` (além da redação original de `:64`, se tiver sido ajustada), restaurando `.env.example` byte-idêntico ao base. Nenhum `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, doc, worktree, gate/script, workflow, ADR, constituição, STATE, DB, rede ou segredo é modificado. Os artefatos do ciclo 081 são documentação e podem ser revertidos em bloco. Sem force-push ou reescrita de histórico; após a reversão, repetir os gates 1, 2, 3, 4 e 5 para confirmar o retorno ao estado base.
