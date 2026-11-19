# Validation: 081 — limpeza das chaves OIDC em `.env.example`

**Data**: 2026-11-19T22:20Z · **Branch**: `feature/081-env-oidc-cleanup` · **Baseline**: `35b8d3142`
**Veredito R4**: **APROVADO**.

## Gates e verificações

| Gate | Resultado (saída colada) |
|---|---|
| `git diff -- .env.example` | `1 path`, `+4/−4` — `:62-63` substituídas por 3 linhas de comentário, `:64` reescrita, `:65` removida; `:59-61` e `:66-68` byte-idênticos (SC-004) |
| `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` | somente `FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET` (`:59-61`), `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=[...]` (`:64`, comentado) e `ZITADEL_*` (`:66-68`); zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado (SC-001) |
| `cargo check -p flora-core` | `exit 0` (T003/T004; nenhum consumidor de config muda) |
| `python3 scripts/ci/check_gate_integrity.py` | gate do alvo **PASS**; `exit 0` (SC-003; nenhum dos dois lê `.env` em runtime, conforme `.env.example:33-34`) |
| `git diff --exit-code 35b8d3142 -- flora-core/src/config/mod.rs flora-core/src/config/oidc.rs flora-api/src/routes/auth.rs` | sem saída — leitores byte-idênticos ao base; nenhum `deny_unknown_fields`, alias, variável nova ou shim introduzido (FR-003, SC-004) |
| Exemplo `FLORA_OIDC__ALLOWED_REDIRECT_URIS` | comentado, com aspas duplas internas idênticas a `.env.example:35` (array Figment `0.10.19` válido para `Vec<String>`); URI extra `127.0.0.1:4173` distinta do callback padrão; allowlist exata de `auth.rs:105-117` não duplicada (SC-002) |
| `specs/081-env-oidc-cleanup/analyze.md` | `{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}` · `exit=0` |
| Verificador isolado `EnvVerifier` | **PASS-WITH-NITS** — remoções corretas de `__REDIRECT_URI` e do alias isolado `OIDC_ISSUER_URL`, sintaxe do exemplo e leitores reais preservados, diff restrito a `.env.example` (+4/−4), `oidc.rs`/`config/mod.rs`/`auth.rs` byte-idênticos, risco funcional em runtime nulo. Ambos os nits tratados (ver § Nits) |

### Saídas coladas

Auditoria de escopo observada contra a base `35b8d3142`:

```
$ git diff -- .env.example
1 path, +4/-4

$ git diff --exit-code 35b8d3142 -- \
    flora-core/src/config/mod.rs \
    flora-core/src/config/oidc.rs \
    flora-api/src/routes/auth.rs
(sem saída)
```

Remoções e adições exatas em `.env.example` (a redação anterior de `:64` foi alinhada, virando `:65`):

```
removidas (base):
-# Redirect/callback URI (verify config key support in your flow):
-FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback
-# OIDC_ISSUER_URL=http://localhost:8080

adicionadas (HEAD):
+# Extra redirect URIs for /api/v1/auth/login: Figment array syntax (double quotes
+# inside). Exact match; the callback from FLORA_APP__BASE_URL is always allowed.
+# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]
+# Legacy fallbacks read by OidcConfig::default() via std::env::var:   (:64 → :65, reescrita)
  +4 -4
```

Bloco final verificado em `.env.example` (SC-001/SC-002):

```
59  FLORA_OIDC__ISSUER_URL=http://localhost:8080
60  FLORA_OIDC__CLIENT_ID=flora@localhost
61  FLORA_OIDC__CLIENT_SECRET=your-zitadel-client-secret
62  # Extra redirect URIs for /api/v1/auth/login: Figment array syntax (double quotes
63  # inside). Exact match; the callback from FLORA_APP__BASE_URL is always allowed.
64  # FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]
65  # Legacy fallbacks read by OidcConfig::default() via std::env::var:
66  # ZITADEL_ISSUER_URL=http://localhost:8080
67  # ZITADEL_CLIENT_ID=flora@localhost
68  # ZITADEL_CLIENT_SECRET=your-zitadel-client-secret
```

Analisador (`specs/081-env-oidc-cleanup/analyze.md`):

```
{"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
exit=0
```

### Nits (fechados)

- **Nit 1 — FR-001, corrigido em paralelo**: a 2ª linha do comentário novo dizia apenas "the configured callback is always allowed", sem nomear `FLORA_APP__BASE_URL` como origem do callback padrão. Corrigido pelo agente `EnvNitFix` (escopo exclusivo de `.env.example`) para `# inside). Exact match; the callback from FLORA_APP__BASE_URL is always allowed.` (`:63`) — confirmado no bloco final acima; FR-001 passa a estar integralmente satisfeito. Nenhuma outra linha foi alterada por essa correção.
- **Nit 2 — SC-003, informativo, fechado sem edição de spec**: SC-003 nomeia 11 artefatos (6 docs do ciclo + 5 de assessment), mas há 12 não rastreados nos dois diretórios 081, com o extra `specs/081-env-oidc-cleanup/analyze.md` — todos dentro do escopo permitido (analyze é documentação READ-ONLY do ciclo; `converge.md` também é doc do ciclo). Registrado para rastreabilidade na entrada 1 do `converge.md` (2026-11-19T22:18Z); nenhuma edição de `spec.md`/`plan.md`/`tasks.md` foi feita.

Nota de proveniência: esta passagem de documentação não reexecutou gates, build, testes, formatters ou git — as saídas acima são a evidência consolidada do ciclo (orchestrator, verificador isolado `EnvVerifier` **PASS-WITH-NITS**, correção `EnvNitFix` e `converge.md`). Nenhuma inferência além dela.

## Fronteira

Somente `.env.example` mudou: `:62-63` substituídas por 3 linhas de comentário que explicam que o callback padrão deriva de `FLORA_APP__BASE_URL` (`auth.rs:101`, `{base_url}/api/v1/auth/callback`) e que URIs extras exigem `FLORA_OIDC__ALLOWED_REDIRECT_URIS` (`oidc.rs:20-21`, allowlist exata em `auth.rs:105-117`), com o exemplo mantido **comentado**; `:64` reescrita para `# Legacy fallbacks read by OidcConfig::default() via std::env::var:` (`:65`); `:65` original (`# OIDC_ISSUER_URL=http://localhost:8080`) removida. `:59-61` (`FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET`) e `:66-68` (`ZITADEL_*`, lidos por `std::env::var` em `oidc.rs:27,29,31`) permanecem byte-idênticos, assim como o restante do template. Nenhum `deny_unknown_fields`, alias, variável nova ou shim foi introduzido: `Config` (`config/mod.rs:17-31`) e `Config::load` (`:43-49`) continuam ignorando a chave legada `FLORA_OIDC__REDIRECT_URI` sem erro de startup. Todo o restante são artefatos do ciclo 081. `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, contrato OIDC, docs de research, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, `.specify/memory/STATE.md`, DB/rede, segredos, `.tokens/` e lockfiles permanecem byte-idênticos e intocados. Nenhuma tarefa foi anexada a `tasks.md` (byte-for-byte inalterado) e `analyze.md` continua READ-ONLY.

## [NV]

Consumidores externos do template (repos que copiam `.env.example` podem esperar `__REDIRECT_URI`, hoje já inerte; compatibilidade preservada pela ausência de `deny_unknown_fields`, não por shim) e comportamento de deploy fora do repo que ainda exporta `FLORA_OIDC__REDIRECT_URI` ou descomenta o exemplo com sintaxe inválida (deve usar aspas duplas, como `.env.example:35`); CI ao vivo, Forgejo, PRs, aprovações, merge, produção e fechamento de issues (zero PRs, nenhum merge/deploy/tag; `.github/workflows/ci.yml:27-28` não lê `.env`); leg RBAC/maintainer (ADRs, constituição, `FLOORS` e workflows dependem de aprovação, ciclo separado); cópia em worktree `state-closure-072` e contrato OIDC histórico (`specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17`, fora do escopo por SC-004). Nenhuma inferência além da evidência local consolidada no ciclo.
