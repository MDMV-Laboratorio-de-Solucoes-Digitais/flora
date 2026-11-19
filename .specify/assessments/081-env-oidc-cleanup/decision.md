# Decision: limpeza das chaves OIDC em `.env.example`

- **Slug**: 081-env-oidc-cleanup · **Decidido**: 2026-11-19T22:06Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept
- **80/20**: E=(2×5)/(1×1)=10,0 — valor 2 (remove doc enganosa sobre callback e drift de config, sem mudar comportamento), confiança 5 (evidência local integral: template, leitores Figment, `std::env::var`, callback e allowlist lidos e citados), esforço 1 (2 linhas substituídas, 1 removida, 1 redação opcional, 1 arquivo), risco 1 (nenhum Rust lê `.env` em runtime; mudança é só de template). Alternativas do ciclo: flag OIDC morta 15,0 (feita em 080) · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25.

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | Drift registrado em `docs/research/2026-11-16-016-macro-files-ops-parity.md:32`; 080 removeu a flag no Rust e deixou o template sem correspondência (`.specify/assessments/080-remove-dead-oidc-flag/research.md:11`) |
| Evidência | strong | `.env.example:58-68` colado; `oidc.rs:7-22,49-71` sem `redirect_uri`; `config/mod.rs:17-31,43-49` sem `deny_unknown_fields` → chave descartada em silêncio; `auth.rs:101,105-117` mostram o mecanismo real |
| Valor vs. inação | strong | Inação mantém `__REDIRECT_URI` que parece configurar o callback (não configura) e esconde `ALLOWED_REDIRECT_URIS`, a única via para URIs extras |
| Viabilidade | strong | Opção A: editar só `.env.example`; nenhum arquivo Rust, DB, rede, segredo ou mudança de contrato de leitura |
| Fit | strong | Corrige o documento exatamente onde o erro vive; não cria segundo mecanismo nem shim; `oidc.rs`, `mod.rs` e `auth.rs` intactos |
| Risco | strong | `ZITADEL_*` e `FLORA_OIDC__{ISSUER_URL,CLIENT_ID,CLIENT_SECRET}` preservados (leitores reais); exemplo fica comentado com sintaxe Figment válida (`.env.example:35`); `[NV]` consumidores externos/CI sem quebra de leitura |

## Handoff

Opção A: em `.env.example`, substituir `:62-63` por comentário explicativo + `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (comentado), remover `:65`, opcionalmente alinhar `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`". Dentro = `.env.example` + 5 artefatos 081. Fora = `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs/specs de research, contrato `specs/001-.../contracts/OIDC.md`, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto artefatos), DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles. FR-001..FR-004 e SC-001..SC-004 de `define.md`.

Comandos determinísticos pós-edição (main agent — nenhum lê `.env` em runtime):

1. `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` — esperado: `__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET`, `__ALLOWED_REDIRECT_URIS` (comentado); zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado.
2. `git diff --name-only` — esperado: `.env.example` + 5 artefatos 081, nada mais.
3. `python3 scripts/ci/check_gate_integrity.py` — gate segue PASS.
4. `cargo check -p flora-core` — sem erro; nenhum consumidor de config muda.

`[NV]` não verificados nesta avaliação: execução dos comandos acima; consumidores externos do template (repos que copiam `.env.example` podem esperar `__REDIRECT_URI`, hoje já inerte — compat preservada pela ausência de `deny_unknown_fields` em `config/mod.rs:17-31`); CI (`.github/workflows/ci.yml:27-28` não lê `.env`, mudança é documental); comportamento de um deploy que descomenta o exemplo com sintaxe inválida (deve usar aspas duplas, como `.env.example:35`). Gates/formatters **skipped** nesta avaliação somente leitura; base `35b8d3142` preservada.
