# Converge: 081 — limpeza das chaves OIDC em `.env.example`

> Append-only. Novas entradas devem ser acrescentadas ao final; nunca reescrever,
> renumerar ou remover entradas anteriores.

## 2026-11-19T22:18Z — entrada 1

- **Decisão 80/20** (`.specify/assessments/081-env-oidc-cleanup/decision.md`,
  2026-11-19T22:06Z): `E=(2×5)/(1×1)=10,0` — valor 2 (remove doc enganosa sobre o
  callback e o drift de config, sem mudar comportamento), confiança 5 (evidência
  local integral: template, leitores Figment, `std::env::var`, callback e
  allowlist lidos e citados), esforço 1 (2 linhas substituídas, 1 removida, 1
  redação ajustada, 1 arquivo), risco 1 (nenhum Rust lê `.env` em runtime;
  mudança é só de template). Alternativas afastadas: flag OIDC morta 15,0 (feita
  em 080) · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25; leg RBAC bloqueada por
  maintainer **[NV]**. Veredito **go (scoped), opção A**: em `.env.example`,
  substituir `:62-63` por comentário explicativo +
  `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]`
  (comentado), remover `:65` e alinhar a redação de `:64` — sem tocar em
  `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, contrato
  OIDC, docs de research, worktree `state-closure-072`, gate, CI, DB ou segredos.
- **Evidência determinística consolidada (orchestrator)**, base `35b8d3142`,
  branch `feature/081-env-oidc-cleanup`, zero PRs:
  - `git diff -- .env.example` = 1 path, `+4/−4`: as linhas
    `# Redirect/callback URI (verify config key support in your flow):`,
    `FLORA_OIDC__REDIRECT_URI=http://localhost:3000/auth/callback` e
    `# OIDC_ISSUER_URL=http://localhost:8080` saem; entram 3 linhas de comentário
    (callback derivado de `FLORA_APP__BASE_URL`, allowlist via
    `FLORA_OIDC__ALLOWED_REDIRECT_URIS`, exemplo comentado) + a redação alinhada
    de `:64`. `:59-61` (`FLORA_OIDC__*`) e `:66-68` (`ZITADEL_*`) byte-idênticos.
  - `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` → somente
    `FLORA_OIDC__ISSUER_URL/CLIENT_ID/CLIENT_SECRET` (`:59-61`),
    `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=[...]` (`:64`) e `ZITADEL_*` (`:66-68`);
    zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado (SC-001).
  - `cargo check -p flora-core` → exit 0; `python3 scripts/ci/check_gate_integrity.py`
    → gate do alvo PASS (ambos sem ler `.env` em runtime, `.env.example:33-34`).
  - `git diff --exit-code 35b8d3142 -- flora-core/src/config/mod.rs
    flora-core/src/config/oidc.rs flora-api/src/routes/auth.rs` → sem saída;
    nenhum `deny_unknown_fields`, alias, variável nova ou shim introduzido.
  - Exemplo segue comentado com aspas duplas internas idênticas a
    `.env.example:35` (array Figment `0.10.19` válido para `Vec<String>`), URI
    extra `127.0.0.1:4173` distinta do callback padrão, allowlist exata de
    `auth.rs:105-117` não duplicada (SC-002).
- **Analyzer** (`analyze.md`, READ-ONLY, 2026-11-19T22:13:38Z, verificador
  `/tmp/flora-sdd-081/analyze_081.py`):

  ```
  {"artifacts": 11, "errors": [], "fr": 4, "go": true, "sc": 4, "tasks": 4}
  exit=0
  ```

  `go=True` na 1ª execução; FR-001..FR-004 e SC-001..SC-004 em `plan.md`/`tasks.md`,
  T001..T004 únicos e sequenciais; checklist com 14 itens `[x]`, `[NV]` e
  `go (scoped)` presentes. `errors=[]`; Analyze não alterou arquivos de código.
- **Verificador isolado `EnvVerifier`**: **PASS-WITH-NITS** — remoções corretas
  de `__REDIRECT_URI` e do alias isolado `OIDC_ISSUER_URL`, sintaxe do exemplo e
  leitores reais preservados, diff restrito a `.env.example` (+4/−4),
  `oidc.rs`/`config/mod.rs`/`auth.rs` byte-idênticos, risco funcional em runtime
  nulo. Dois nits registrados:
  - **Nit 1 (FR-001, corrigido em paralelo)**: a 2ª linha do comentário novo
    dizia apenas "the configured callback is always allowed", sem nomear
    `FLORA_APP__BASE_URL` como origem do callback padrão. Corrigido pelo agente
    `EnvNitFix` (escopo exclusivo de `.env.example`) para
    `# inside). Exact match; the callback from FLORA_APP__BASE_URL is always allowed.`
    — confirmado no `git diff` acima; FR-001 passa a estar integralmente
    satisfeito. Nenhuma outra linha foi alterada por essa correção.
  - **Nit 2 (informativo, SC-003, sem edição de spec)**: SC-003 nomeia 11
    artefatos (6 docs do ciclo + 5 de assessment), mas há 12 não rastreados nos
    dois diretórios 081, com o extra `specs/081-env-oidc-cleanup/analyze.md` —
    todos dentro do escopo permitido (analyze é documentação READ-ONLY do ciclo;
    este `converge.md` também é doc do ciclo). Registrado para rastreabilidade;
    nenhuma edição de `spec.md`/`plan.md`/`tasks.md` feita.
- **Dívidas e [NV]**:
  - **Consumidores externos do template**: repos que copiam `.env.example` podem
    esperar `__REDIRECT_URI`, hoje já inerte; a remoção não quebra leitura de
    config (compatibilidade preservada pela ausência de `deny_unknown_fields` em
    `config/mod.rs:17-31`, não por shim). Consumo externo **[NV]**.
  - **CI e produção**: status de CI ao vivo, PRs, Forgejo, produção, merge e
    fechamento de issues não verificados neste ciclo (zero PRs, sem
    merge/deploy/tag); `.github/workflows/ci.yml:27-28` não lê `.env` **[NV]**.
  - **Leg RBAC / aprovação maintainer**: leg RBAC bloqueada; qualquer mudança em
    ADRs, constituição, `FLOORS` ou workflows depende de aprovação maintainer —
    ciclo separado, fora deste escopo **[NV]**.
  - **Comportamento fora do repo**: deploy que ainda exporta
    `FLORA_OIDC__REDIRECT_URI` ou que descomenta o exemplo com sintaxe inválida
    (deve usar aspas duplas, como `.env.example:35`) — não inferido além da
    evidência local e não induzido pelo template **[NV]**.
  - **Cópia em worktree `state-closure-072` e contrato OIDC histórico**
    (`specs/001-flora-seed-v0-1/contracts/OIDC.md:10,17`): texto antigo fora do
    escopo por SC-004 **[NV]**.
- **Próxima portaria**: emitir `validate.md` (R4) e `release.md` (R7) em
  `.specify/validations/081-env-oidc-cleanup/` e
  `.specify/releases/081-env-oidc-cleanup/`, respectivamente. Mudança é
  puramente documental em template: nenhum bump de código obrigatório; se o
  maintainer optar por registrar versão, patch `0.3.1 → 0.3.2` (a branch 080
  entregou `0.3.0 → 0.3.1`). Rollback de 1 path:
  `git checkout 35b8d3142 -- .env.example`. Nenhuma tag, deploy ou merge
  executados neste ciclo — PR a critério do maintainer.

## Resultado

**Converged — a implementação satisfaz spec, plan, tasks e analyzer**, com os
dois nits do verificador tratados (Nit 1 corrigido em paralelo em
`.env.example`; Nit 2 informativo registrado acima, sem edição de spec).
Nenhuma tarefa foi anexada a `tasks.md` (permanece byte-for-byte inalterado);
`analyze.md` continua READ-ONLY. Registro criado apenas para rastreabilidade;
gates, build, testes, formatters e git não foram reexecutados por este agente.

converge written: 1 file; gates/formatters skipped
