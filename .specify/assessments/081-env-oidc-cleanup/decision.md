# Decision: limpeza das chaves OIDC em `.env.example`

- **Slug**: 081-env-oidc-cleanup
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md | decision.md (anterior, decidido em 2026-11-19T22:06Z) | decisão 080 e docs/research de drift via intake/research

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `.env.example:62-63` documenta `FLORA_OIDC__REDIRECT_URI` sem campo em `OidcConfig`; drift registrado em `docs/research/2026-11-16-016-macro-files-ops-parity.md:32` e sucessor direto de 080 (`intake.md:4`). |
| Evidence strength | strong | Template colado, leitores Figment/`std::env::var` lidos, callback derivado de `FLORA_APP__BASE_URL` em `auth.rs:101` e allowlist exata em `:105-117` (`research.md:7-14`). |
| Value vs. inaction | strong | Inação mantém chave que parece configurar o callback (não configura) e esconde `FLORA_OIDC__ALLOWED_REDIRECT_URIS`, a única via para URIs extras (`define.md:7`). |
| Feasibility / appetite | strong | Opção A: 2 linhas substituídas, 1 removida, 1 redação opcional, em `.env.example`; nenhum arquivo Rust, DB, rede ou segredo (`concept.md:5-9`). |
| Strategic fit | strong | Corrige o documento exatamente onde o erro vive, preservando `ZITADEL_*` e `FLORA_OIDC__{ISSUER_URL,CLIENT_ID,CLIENT_SECRET}` e sem criar segundo mecanismo. |
| Risk posture | strong | Template não é lido em runtime (`.env.example:33-34`); sem `deny_unknown_fields`, chave legada continua ignorada sem erro; exemplo permanece comentado com sintaxe Figment válida de `:35`. `[NV]` consumidores externos e CI. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado como `go`**; decisão original em 2026-11-19T22:06Z, branch `feature/081-env-oidc-cleanup`, base `35b8d3142`, gate PASS, zero PRs. 080 removeu a flag no Rust e deixou o template sem correspondência; 081 limpa o template sem tocar comportamento. 80/20 original integral: **E=(2×5)/(1×1)=10,0** — valor 2 (remove doc enganosa sobre callback e drift de config), confiança 5 (evidência local integral: template, leitores Figment, `std::env::var`, callback e allowlist lidos e citados), esforço 1 (2 linhas substituídas, 1 removida, 1 redação opcional, 1 arquivo), risco 1 (nenhum Rust lê `.env` em runtime). Alternativas do ciclo: flag OIDC morta 15,0 (feita em 080) · `cssVariables` 5,0 · lucide 2,50 · OTLP 1,25. Opções B (dar suporte real à chave no Rust) e C (não editar) rejeitadas em `concept.md:11-19`: a primeira muda contrato/auth e cria segunda fonte de callback; a segunda deixa 080 pela metade. O callback real vem de `FLORA_APP__BASE_URL` (`auth.rs:101`) e URIs extras exigem allowlist exata (`oidc.rs:20-21`, `auth.rs:105-117`); `localhost` e `127.0.0.1` são origens distintas.

## If needs-clarification

- **Blocking questions**: N/A (veredito `go`; incógnitas seguem como carried-forward).
- **Revisit stage**: N/A.

## If go — Handoff to `/speckit.specify`

- **Problem**: `.env.example` documenta `__REDIRECT_URI` inerte e `# OIDC_ISSUER_URL` fantasma, omitindo o mecanismo real de callback e allowlist; drift de config que induz operadores a erro.
- **Chosen approach**: Opção A — substituir `:62-63` por comentário explicativo + `# FLORA_OIDC__ALLOWED_REDIRECT_URIS=["http://127.0.0.1:4173/callback"]` (comentado), remover `:65`, opcionalmente alinhar `:64` para "Legacy fallbacks read by `OidcConfig::default()` via `std::env::var:`"; FR-001..FR-004 e SC-001..SC-004 de `define.md`.
- **In scope / out of scope**:
  - **In**: `.env.example` e os 5 artefatos 081.
  - **Out**: `flora-core/src/config/{mod,oidc}.rs`, `flora-api/src/routes/auth.rs`, docs/specs de research, contrato `specs/001-.../contracts/OIDC.md`, worktree `state-closure-072`, gate/`scripts/ci/check_gate_integrity.py`, `.github/workflows/`, ADRs, constituição, STATE, specs/081-* (exceto artefatos), DB/rede, segredos, `.tokens/`, leg RBAC e lockfiles.
- **Success metrics**: `grep -n "FLORA_OIDC__\|OIDC_ISSUER_URL\|ZITADEL_" .env.example` → `__ISSUER_URL`, `__CLIENT_ID`, `__CLIENT_SECRET`, `__ALLOWED_REDIRECT_URIS` (comentado); zero `FLORA_OIDC__REDIRECT_URI` e zero `OIDC_ISSUER_URL` isolado; `git diff --name-only` só `.env.example` + 5 artefatos; gate PASS; `cargo check -p flora-core` sem erro.
- **Carried-forward open questions**: `[NV]` execução dos comandos acima; `[NV]` consumidores externos do template (compat de leitura preservada pela ausência de `deny_unknown_fields`); `[NV]` CI (`.github/workflows/ci.yml:27-28` não lê `.env`); `[NV]` deploy que descomenta o exemplo com sintaxe inválida — exigir aspas duplas como em `.env.example:35`. Gates/formatters **skipped** nesta avaliação somente leitura; base `35b8d3142` preservada.
